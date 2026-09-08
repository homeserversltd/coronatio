// Crown-side plumbing only: admission, process custody and opaque HTML proxy.
// No roster, metric, collector or renderer is called in the crown process.
use super::*;
use std::time::Instant;
use tokio::io::{AsyncBufReadExt, AsyncReadExt};
use tokio::process::{Child, Command as AsyncCommand};
use tokio::sync::Mutex as AsyncMutex;

pub(super) const PATH: &str = "/cartridge/my-devices";
const RESTART_DELAY: Duration = Duration::from_secs(30);
const PROXY_TIMEOUT: Duration = Duration::from_secs(9);
struct Running {
    child: Child,
    addr: SocketAddr,
    token: String,
}
#[derive(Default)]
struct Manager {
    running: Option<Running>,
    retry_at: Option<Instant>,
    closing: bool,
}
fn manager() -> &'static AsyncMutex<Manager> {
    static MANAGER: OnceLock<AsyncMutex<Manager>> = OnceLock::new();
    MANAGER.get_or_init(|| AsyncMutex::new(Manager::default()))
}

pub(super) fn is_target(raw: &str) -> bool {
    url::Url::parse(raw)
        .ok()
        .map(|u| u.path() == PATH && u.query().is_none() && u.fragment().is_none())
        .unwrap_or(false)
}
pub(super) fn admitted() -> bool {
    appliance_cartridge("my-devices")
        .map(|c| c.guest_class == "iframe" && !c.admin_only && is_target(&c.url))
        .unwrap_or(false)
}

async fn endpoint() -> Result<(SocketAddr, String), &'static str> {
    let mut state = manager().lock().await;
    if state.closing {
        return Err("worker-stopping");
    }
    if let Some(running) = state.running.as_mut() {
        if matches!(running.child.try_wait(), Ok(None)) {
            return Ok((running.addr, running.token.clone()));
        }
        state.running.take();
        state.retry_at = Some(Instant::now() + RESTART_DELAY);
        return Err("worker-exited");
    }
    if state.retry_at.map(|v| Instant::now() < v).unwrap_or(false) {
        return Err("worker-recovery-wait");
    }
    state.retry_at = Some(Instant::now() + RESTART_DELAY);
    let exe = env::current_exe().map_err(|_| "worker-executable-unavailable")?;
    let token = uuid::Uuid::new_v4().simple().to_string();
    let mut child = AsyncCommand::new(exe)
        .arg("--my-devices-worker")
        .env("CORONATIO_MY_DEVICES_TOKEN", &token)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::inherit())
        .kill_on_drop(true)
        .spawn()
        .map_err(|_| "worker-start-failed")?;
    let stdout = child.stdout.take().ok_or("worker-ready-unavailable")?;
    let mut line = String::new();
    let mut reader = tokio::io::BufReader::new(stdout.take(80));
    let read = tokio::time::timeout(Duration::from_millis(750), reader.read_line(&mut line)).await;
    let addr = match read {
        Ok(Ok(_)) => line.trim().parse::<SocketAddr>().ok().filter(|v| {
            v.ip() == std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST) && v.port() != 0
        }),
        _ => None,
    };
    let Some(addr) = addr else {
        let _ = child.start_kill();
        let _ = tokio::time::timeout(Duration::from_secs(1), child.wait()).await;
        return Err("worker-ready-unavailable");
    };
    state.running = Some(Running {
        child,
        addr,
        token: token.clone(),
    });
    Ok((addr, token))
}
async fn failed(token: &str) {
    let mut state = manager().lock().await;
    // A delayed response from an older generation cannot kill its replacement.
    if state.running.as_ref().map(|v| v.token.as_str()) != Some(token) {
        return;
    }
    if let Some(mut running) = state.running.take() {
        let _ = running.child.start_kill();
        let _ = tokio::time::timeout(Duration::from_secs(1), running.child.wait()).await;
    }
    state.retry_at = Some(Instant::now() + RESTART_DELAY);
}
pub(super) async fn stop() {
    let mut state = manager().lock().await;
    state.closing = true;
    if let Some(mut running) = state.running.take() {
        // Close the dedicated parent-lifetime pipe, then bound and reap shutdown.
        running.child.stdin.take();
        if tokio::time::timeout(Duration::from_secs(1), running.child.wait())
            .await
            .is_err()
        {
            let _ = running.child.start_kill();
            let _ = tokio::time::timeout(Duration::from_secs(1), running.child.wait()).await;
        }
    }
}
fn fault(signal: &str) -> Response {
    record_cartridge_fault("my-devices", CartridgeFaultKind::UpstreamError);
    let body = format!("<!doctype html><html><head><title>My Devices unavailable</title></head><body><main data-cartridge-fault=\"true\"><h1>My Devices is unavailable</h1><p>{}</p><p>The other tabs remain available. Wait 30 seconds, then retry.</p><a href=\"{}\">Retry My Devices</a></main></body></html>", html_escape(signal), PATH);
    (
        StatusCode::SERVICE_UNAVAILABLE,
        [("x-coronatio-fault", "cartridge-fragment")],
        Html(body),
    )
        .into_response()
}
pub(super) async fn activation_ready() -> bool {
    let Ok((addr, token)) = endpoint().await else {
        return false;
    };
    match cartridge_http::tcp_get(
        addr,
        &format!("/{token}/health"),
        Duration::from_millis(250),
    )
    .await
    {
        Ok(reply) if reply.status == 200 && reply.body == b"ready" => true,
        _ => {
            failed(&token).await;
            false
        }
    }
}
pub(super) async fn proxy() -> Response {
    // The registry, written only by attended Caduceus admission, is the gate.
    if !admitted() {
        return StatusCode::NOT_FOUND.into_response();
    }
    let mut response = match endpoint().await {
        Err(signal) => fault(signal),
        Ok((addr, token)) => {
            let path = format!("/{token}");
            match cartridge_http::tcp_get(addr, &path, PROXY_TIMEOUT).await {
                Ok(reply) if reply.status == 200 => match String::from_utf8(reply.body) {
                    Ok(body) => Html(body).into_response(),
                    Err(_) => {
                        failed(&token).await;
                        fault("worker-invalid-response")
                    }
                },
                _ => {
                    failed(&token).await;
                    fault("worker-unavailable")
                }
            }
        }
    };
    response
        .headers_mut()
        .insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    // The guest has no script/network dialect and cannot reach sibling documents.
    response.headers_mut().insert(header::CONTENT_SECURITY_POLICY, HeaderValue::from_static("default-src 'none'; style-src 'self' 'unsafe-inline'; img-src 'self' data:; font-src 'self'; form-action 'self'; frame-ancestors 'self'; base-uri 'none'; sandbox allow-forms allow-same-origin"));
    response
}
