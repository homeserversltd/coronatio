// This module is entered only by --my-devices-worker, before crown startup.
// All roster, collectors, caches and rendering belong to that child process.
use super::*;
use futures_util::future::join_all;
use std::net::Ipv4Addr;
use std::time::Instant;
use tokio::sync::Mutex as AsyncMutex;

const TTL: Duration = Duration::from_secs(30);
const READ_TIMEOUT: Duration = Duration::from_secs(3);
const STATS_PATH: &str = "/api/v1/appliance/stats";
#[derive(Clone)]
enum SnapshotFailure {
    Transport(&'static str),
    Unavailable(String),
}
type Snapshot = Result<serde_json::Value, SnapshotFailure>;
struct Held<T> {
    at: Instant,
    value: T,
}
type Cell<T> = Arc<AsyncMutex<Option<Held<T>>>>;
#[derive(Clone)]
struct Device {
    mac: String,
    hostname: String,
    ipv4: Option<Ipv4Addr>,
    gateway: bool,
}
#[derive(Clone)]
struct Roster {
    devices: Vec<Device>,
    missing: Option<String>,
}
#[derive(Default)]
struct Worker {
    roster: Cell<Roster>,
    stats: AsyncMutex<BTreeMap<String, Cell<Snapshot>>>,
}

pub(super) async fn run() -> Result<(), String> {
    // Pipe custody is ephemeral. EOF on crown death/shutdown terminates even a
    // stopped collection; no persistent unit, pid file or endpoint is created.
    let token = env::var("CORONATIO_MY_DEVICES_TOKEN").map_err(|_| "worker-token-missing")?;
    if token.len() != 32 || !token.bytes().all(|v| v.is_ascii_hexdigit()) {
        return Err("worker-token-invalid".into());
    }
    let listener = tokio::net::TcpListener::bind((Ipv4Addr::LOCALHOST, 0))
        .await
        .map_err(|_| "worker-bind-failed")?;
    let addr = listener.local_addr().map_err(|_| "worker-address-failed")?;
    let route = format!("/{token}");
    let router = Router::new()
        .route(&route, get(page))
        .route(&format!("{route}/health"), get(|| async { "ready" }))
        .with_state(Arc::new(Worker::default()));
    println!("{addr}");
    std::io::stdout()
        .flush()
        .map_err(|_| "worker-ready-failed")?;
    std::thread::spawn(|| {
        let mut byte = [0_u8];
        while std::io::stdin().read(&mut byte).unwrap_or(0) != 0 {}
        std::process::exit(0);
    });
    axum::serve(listener, router)
        .await
        .map_err(|_| "worker-serve-failed".into())
}

fn text(value: Option<&serde_json::Value>) -> String {
    value
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .chars()
        .take(160)
        .collect()
}
fn mac(value: &str) -> Option<String> {
    let compact = value.replace([':', '-'], "");
    if compact.len() != 12 || !compact.bytes().all(|v| v.is_ascii_hexdigit()) {
        return None;
    }
    Some(
        (0..6)
            .map(|i| compact[i * 2..i * 2 + 2].to_ascii_lowercase())
            .collect::<Vec<_>>()
            .join(":"),
    )
}
fn device(row: &serde_json::Value, gateway: bool) -> Device {
    Device {
        mac: text(row.get("mac")),
        hostname: text(row.get("hostname")),
        ipv4: row
            .get("ipv4")
            .and_then(|v| v.as_str())
            .and_then(|v| v.parse().ok()),
        gateway,
    }
}
fn decode(
    reply: Result<cartridge_http::Reply, cartridge_http::Failure>,
) -> Result<serde_json::Value, cartridge_http::Failure> {
    let reply = reply?;
    let failure = |outcome| cartridge_http::Failure {
        outcome,
        status: Some(reply.status),
    };
    if !(200..300).contains(&reply.status) {
        return Err(failure("http-error"));
    }
    let value: serde_json::Value =
        serde_json::from_slice(&reply.body).map_err(|_| failure("decode-error"))?;
    if !value.is_object() {
        return Err(failure("decode-error"));
    }
    Ok(value)
}
fn upstream_missing(value: &serde_json::Value) -> Option<String> {
    let signal = text(
        value
            .get("firstMissingSignal")
            .or_else(|| value.get("first_missing_signal")),
    );
    if !signal.is_empty() && signal != "none" {
        return Some(signal);
    }
    (value.get("ok").and_then(|v| v.as_bool()) == Some(false))
        .then(|| "upstream-unavailable".into())
}

impl Worker {
    async fn roster(&self) -> Roster {
        let mut held = self.roster.lock().await;
        if let Some(v) = held.as_ref().filter(|v| v.at.elapsed() < TTL) {
            return v.value.clone();
        }
        let value = match decode(caduceus_get_bounded("/api/v1/ruyi", READ_TIMEOUT).await) {
            Err(e) => Roster {
                devices: Vec::new(),
                missing: Some(format!("ruyi-{}", e.outcome)),
            },
            Ok(value) => {
                let mut missing = upstream_missing(&value);
                let mut devices = Vec::new();
                let mut seen = BTreeSet::new();
                // Seat is independent of staves: an empty roster still has self.
                if let Some(seat) = value.get("seat").filter(|v| v.is_object()) {
                    let mut own = device(seat, true);
                    let own_key = mac(&own.mac);
                    if let Some(rows) = value.get("staves").and_then(|v| v.as_array()) {
                        if let Some(row) = rows
                            .iter()
                            .find(|r| own_key.is_some() && mac(&text(r.get("mac"))) == own_key)
                        {
                            let full = device(row, true);
                            if own.hostname.is_empty() {
                                own.hostname = full.hostname;
                            }
                            own.ipv4 = full.ipv4;
                        }
                    }
                    if let Some(key) = own_key {
                        seen.insert(key);
                    }
                    if !own.mac.is_empty() || !own.hostname.is_empty() {
                        devices.push(own);
                    }
                }
                match value.get("staves").and_then(|v| v.as_array()) {
                    Some(rows) => {
                        for row in rows {
                            let peer = device(row, false);
                            if let Some(key) = mac(&peer.mac) {
                                if seen.insert(key) {
                                    devices.push(peer);
                                }
                            } else {
                                missing.get_or_insert("ruyi-row-mac-unavailable".into());
                                devices.push(peer);
                            }
                        }
                    }
                    None => {
                        missing.get_or_insert("ruyi-staves-unavailable".into());
                    }
                }
                if !devices.iter().any(|v| v.gateway) {
                    missing.get_or_insert("ruyi-seat-unavailable".into());
                }
                Roster { devices, missing }
            }
        };
        // Prune departed keys only when not held by a concurrent request.
        let keys: BTreeSet<_> = value.devices.iter().map(stats_key).collect();
        self.stats
            .lock()
            .await
            .retain(|k, v| keys.contains(k) || Arc::strong_count(v) > 1);
        *held = Some(Held {
            at: Instant::now(),
            value: value.clone(),
        });
        value
    }
    async fn stats(&self, target: &Device) -> Snapshot {
        let key = stats_key(target);
        let cell = self.stats.lock().await.entry(key).or_default().clone();
        let mut held = cell.lock().await;
        if let Some(v) = held.as_ref().filter(|v| v.at.elapsed() < TTL) {
            return v.value.clone();
        }
        let value = if !target.gateway && mac(&target.mac).is_none() {
            Err(SnapshotFailure::Unavailable("device-mac-unavailable".into()))
        } else if !target.gateway && target.ipv4.is_none() {
            Err(SnapshotFailure::Unavailable("device-address-unavailable".into()))
        } else {
            stats_attempt(target).await
        };
        *held = Some(Held {
            at: Instant::now(),
            value: value.clone(),
        });
        value
    }
}
fn stats_key(target: &Device) -> String {
    format!(
        "{}:{}:{}",
        target.gateway,
        mac(&target.mac).unwrap_or_default(),
        target.ipv4.map(|v| v.to_string()).unwrap_or_default()
    )
}

async fn stats_attempt(target: &Device) -> Snapshot {
    let started = Instant::now();
    let request_id = uuid::Uuid::new_v4().simple().to_string();
    // Log START before the transport is polled, including attempts that fail.
    // No cache hit or singleflight waiter reaches this boundary.
    let unix = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|v| v.as_secs().to_string())
        .unwrap_or_else(|_| "unavailable".into());
    eprintln!(
        "my-devices stats START id={} unix={} mac={} target={} path={}",
        request_id, unix,
        mac(&target.mac).unwrap_or_else(|| "unavailable".into()),
        if target.gateway { "gateway" } else { "peer" }, STATS_PATH
    );
    let raw = if target.gateway {
        caduceus_get_bounded(STATS_PATH, READ_TIMEOUT).await
    } else {
        cartridge_http::tcp_get(
            SocketAddr::from((target.ipv4.expect("validated peer address"), 8787)),
            STATS_PATH,
            READ_TIMEOUT,
        )
        .await
    };
    let status = raw
        .as_ref()
        .ok()
        .map(|r| r.status)
        .or_else(|| raw.as_ref().err().and_then(|e| e.status));
    let upstream_detail = raw
        .as_ref()
        .ok()
        .and_then(|reply| serde_json::from_slice::<serde_json::Value>(&reply.body).ok())
        .and_then(|value| upstream_missing(&value));
    let decoded = decode(raw);
    let outcome = decoded
        .as_ref()
        .err()
        .map(|v| v.outcome)
        .unwrap_or_else(|| {
            if decoded.as_ref().ok().and_then(upstream_missing).is_some() {
                "upstream-unavailable"
            } else {
                "ok"
            }
        });
    eprintln!(
        "my-devices stats COMPLETE id={} outcome={} status={} elapsed_ms={}",
        request_id, outcome,
        status.map(|v| v.to_string()).unwrap_or_else(|| "none".into()),
        started.elapsed().as_millis().min(u32::MAX as u128) as u32
    );
    match decoded {
        Err(e) if matches!(e.outcome, "timeout" | "connect-failed" | "read-failed" | "write-failed") => {
            Err(SnapshotFailure::Transport(e.outcome))
        }
        Err(e) => Err(SnapshotFailure::Unavailable(upstream_detail.unwrap_or_else(|| e.outcome.into()))),
        Ok(v) => match upstream_missing(&v) {
            Some(e) => Err(SnapshotFailure::Unavailable(e)),
            None => Ok(v),
        },
    }
}

async fn page(State(worker): State<Arc<Worker>>) -> Html<String> {
    // A detached bounded collector survives an HTTP viewer disconnect. Cache
    // mutexes singleflight both keys even for simultaneous independent viewers.
    let collector = tokio::spawn(async move { render_page(worker).await });
    Html(collector.await.unwrap_or_else(|_| {
        "<h1>My Devices unavailable</h1><p>Readings could not be loaded. Retry this tab.</p>".into()
    }))
}
async fn render_page(worker: Arc<Worker>) -> String {
    let roster = worker.roster().await;
    let rows = join_all(roster.devices.into_iter().map(|target| {
        let worker = worker.clone();
        async move {
            let value = worker.stats(&target).await;
            render_row(&target, value)
        }
    }))
    .await;
    let warning = roster
        .missing
        .map(|_| "<p role=\"status\">The device list is unavailable or incomplete. Try refreshing shortly.</p>".to_string())
        .unwrap_or_default();
    let style = [
        include_str!("shell/ux/shell/base-and-chrome.css"),
        include_str!("shell/ux/library/_button.css"),
        include_str!("shell/ux/library/_table.css"),
        include_str!("shell/ux/library/_badge.css"),
        // The independent guest receives no selected theme; keep library structure with neutral paint.
        "body { color: CanvasText; background: Canvas; overflow: auto; } main { padding: var(--theme-content-padding, 1rem); } \
        .ui-badge { color: CanvasText; background: Canvas; border: 1px solid currentColor; } \
        .ui-button.ui-button--secondary, .ui-button.ui-button--secondary:hover:not(:disabled) { color: ButtonText; background: ButtonFace; border: 1px solid currentColor; } \
        .ui-table { overflow: auto; margin-top: 1rem; } \
        .ui-table th, .ui-table td { color: var(--text, CanvasText); border: 1px solid var(--border, currentColor); } \
        .ui-table th { background: var(--hiddenTabBackground, Canvas); white-space: nowrap; } \
        .ui-table .device-metric { font-variant-numeric: tabular-nums; white-space: nowrap; font-weight: 600; }",
    ].concat();
    format!(
        r#"<!doctype html><html><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1"><title>My Devices</title><style>{style}</style></head><body><main class="pane-content"><h1>My Devices</h1><p>Other devices are checked on port 8787; offline can also mean a different port. Readings refresh no more than once every 30 seconds.</p>{warning}<form method="get" action="/cartridge/my-devices"><button class="ui-button ui-button--secondary" type="submit">Refresh devices</button></form><div class="ui-table"><table><thead><tr><th>Device</th><th>Status</th><th>Load (1 min)</th><th>Memory used</th><th>Disk used (root)</th><th>Temperature</th><th>MAC</th><th>Address</th><th>Details</th></tr></thead><tbody>{}</tbody></table></div></main></body></html>"#,
        rows.join("")
    )
}
fn number(v: Option<&serde_json::Value>) -> Option<f64> {
    v.and_then(|v| v.as_f64()).filter(|v| v.is_finite())
}
fn percent(v: f64) -> Option<f64> {
    (v.is_finite() && (0.0..=100.0).contains(&v)).then_some(v)
}
fn memory(v: &serde_json::Value) -> Option<f64> {
    let total = number(v.pointer("/memory/MemTotal")).filter(|v| *v > 0.0)?;
    let used = if let Some(used) = v.pointer("/memory/usedBytes") {
        number(Some(used))?
    } else {
        total - number(v.pointer("/memory/MemAvailable"))?
    };
    percent(used / total * 100.0)
}
fn disk(v: &serde_json::Value) -> Option<f64> {
    let rows = v.pointer("/disk/usage")?.as_array()?;
    let root = rows
        .iter()
        .find(|r| r.get("path").and_then(|v| v.as_str()) == Some("/"))?;
    // Disk means the root filesystem, not an invented aggregate of mounts.
    number(root.get("usePercent"))
        .or_else(|| {
            root.get("usePercent")?
                .as_str()?
                .trim_end_matches('%')
                .parse()
                .ok()
        })
        .and_then(percent)
}
fn metric(value: Option<f64>, unit: &str) -> String {
    value
        .map(|v| format!("{v:.1}{unit}"))
        .unwrap_or_else(|| "unavailable".into())
}
fn render_row(target: &Device, snapshot: Snapshot) -> String {
    let (reachability, load, mem, disk, temp, missing) = match snapshot {
        Err(SnapshotFailure::Transport(reason)) => ("offline", None, None, None, None, reason.to_string()),
        Err(SnapshotFailure::Unavailable(reason)) => ("unavailable", None, None, None, None, reason),
        Ok(v) => {
            let load = number(v.pointer("/load/one")).filter(|v| *v >= 0.0);
            let mem = memory(&v);
            let disk = disk(&v);
            let temp = number(v.pointer("/temperature/celsius"));
            let missing = [
                (load, "load-unavailable"),
                (mem, "memory-unavailable"),
                (disk, "root-disk-unavailable"),
                (temp, "temperature-unavailable"),
            ]
            .into_iter()
            .find(|(v, _)| v.is_none())
            .map(|(_, name)| name)
            .unwrap_or("none")
            .to_string();
            ("reachable", load, mem, disk, temp, missing)
        }
    };
    let hostname = if target.hostname.is_empty() {
        "unavailable"
    } else {
        &target.hostname
    };
    let identity = if target.mac.is_empty() {
        "unavailable"
    } else {
        &target.mac
    };
    let badge = match reachability {
        "reachable" => "success",
        "offline" => "danger",
        _ => "warning",
    };
    let detail = match missing.as_str() {
        "none" => "Readings available",
        "timeout" if reachability == "offline" => "Device did not respond in time",
        "connect-failed" if reachability == "offline" => "Could not connect to device",
        "read-failed" | "write-failed" if reachability == "offline" => "Connection interrupted",
        "http-error" => "Device returned an unsuccessful response",
        "decode-error" => "Device response could not be read",
        "device-mac-unavailable" => "Device identity is missing",
        "device-address-unavailable" => "Device address is missing",
        "load-unavailable" => "Load reading is missing",
        "memory-unavailable" => "Memory reading is missing",
        "root-disk-unavailable" => "Root disk reading is missing",
        "temperature-unavailable" => "Temperature reading is missing",
        _ => "Device readings are unavailable",
    };
    let address = if target.gateway {
        "This device".to_string()
    } else {
        target.ipv4.map(|v| v.to_string()).unwrap_or_else(|| "unavailable".into())
    };
    format!(r#"<tr><td>{}</td><td><span class="ui-badge ui-badge--{badge}">{reachability}</span></td><td class="device-metric">{}</td><td class="device-metric">{}</td><td class="device-metric">{}</td><td class="device-metric">{}</td><td>{}</td><td>{}</td><td>{detail}</td></tr>"#,
        html_escape(hostname), metric(load, ""), metric(mem, "%"), metric(disk, "%"), metric(temp, " °C"),
        html_escape(identity), address)
}
