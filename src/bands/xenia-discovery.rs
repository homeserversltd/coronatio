use super::*;
use axum::body::Body;
use http_body_util::{BodyExt, Full};
use hyper::{Request, Uri as HyperUri};
use hyper_util::{
    client::legacy::{connect::HttpConnector, Client},
    rt::TokioExecutor,
};
use std::{collections::HashMap, convert::Infallible, time::Instant};
use tokio::sync::mpsc;

const DISCOVERY_TTL: Duration = Duration::from_secs(30);
const DEFAULT_PROBE_TIMEOUT: Duration = Duration::from_millis(5_000);
const MIN_PROBE_TIMEOUT: Duration = Duration::from_millis(100);
const PROXY_TIMEOUT: Duration = Duration::from_secs(8);
const SSE_STREAM_LIMIT: Duration = Duration::from_secs(35);
const MAX_BODY: usize = 2 * 1024 * 1024;
const MAX_HEALTH_BODY: usize = 64 * 1024;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Discovery {
    pub endpoint: Option<String>,
    pub content_kind: Option<String>,
    pub rung: String,
    pub static_dir: Option<String>,
    pub health: Option<String>,
    pub fault_signal: Option<String>,
}

#[derive(Clone)]
struct HeldDiscovery {
    at: Instant,
    result: Discovery,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct DiscoveryStamp {
    endpoint: Option<String>,
    content_kind: Option<String>,
    rung: String,
}

fn cache() -> &'static Mutex<HashMap<String, HeldDiscovery>> {
    static CACHE: OnceLock<Mutex<HashMap<String, HeldDiscovery>>> = OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

fn stamped() -> &'static Mutex<HashMap<String, DiscoveryStamp>> {
    static STAMPED: OnceLock<Mutex<HashMap<String, DiscoveryStamp>>> = OnceLock::new();
    STAMPED.get_or_init(|| Mutex::new(HashMap::new()))
}

pub(super) fn invalidate() {
    cache()
        .lock()
        .unwrap_or_else(|error| error.into_inner())
        .clear();
}

pub(super) fn health_degraded(tab_id: &str) -> bool {
    cache()
        .lock()
        .unwrap_or_else(|error| error.into_inner())
        .get(tab_id)
        .is_some_and(|held| held.result.health.is_some())
}

fn string_field<'a>(value: &'a serde_json::Value, names: &[&str]) -> Option<&'a str> {
    names
        .iter()
        .find_map(|name| value.get(*name).and_then(serde_json::Value::as_str))
}

fn static_dir(entry: &serde_json::Value) -> Option<String> {
    string_field(entry, &["static_dir", "staticDir"])
        .or_else(|| {
            entry
                .get("install")
                .and_then(|install| string_field(install, &["static_dir", "staticDir"]))
        })
        .map(str::trim)
        .filter(|value| {
            !value.is_empty()
                && !value.starts_with('/')
                && !value.split('/').any(|part| part == "..")
        })
        .map(str::to_string)
}

fn health_path(entry: &serde_json::Value) -> String {
    let raw = entry
        .get("health")
        .and_then(|health| {
            health
                .as_str()
                .or_else(|| string_field(health, &["route", "path"]))
        })
        .unwrap_or("/health")
        .trim();
    if raw.starts_with('/') && !raw.contains(['\r', '\n']) {
        raw.to_string()
    } else {
        "/health".to_string()
    }
}

fn fragment_path(entry: &serde_json::Value) -> &str {
    string_field(entry, &["fragment_path", "fragmentPath"]).unwrap_or("/")
}

fn loopback_endpoint(raw: &str) -> Option<String> {
    let parsed = url::Url::parse(raw.trim()).ok()?;
    if parsed.scheme() != "http" || parsed.query().is_some() || parsed.fragment().is_some() {
        return None;
    }
    let loopback = match parsed.host()? {
        url::Host::Domain(host) => host.eq_ignore_ascii_case("localhost"),
        url::Host::Ipv4(ip) => ip.is_loopback(),
        url::Host::Ipv6(ip) => ip.is_loopback(),
    };
    (loopback && parsed.port_or_known_default().is_some())
        .then(|| parsed.to_string().trim_end_matches('/').to_string())
}

fn declared_endpoint_text(tab: &CoronatioTabContract) -> Option<&str> {
    let transport = tab.transport.as_ref();
    transport
        .and_then(|value| {
            value
                .as_str()
                .or_else(|| value.get("declared").and_then(serde_json::Value::as_str))
        })
        .or_else(|| {
            tab.xenia_entry.as_ref().and_then(|entry| {
                string_field(
                    entry,
                    &["local_transport_endpoint", "localTransportEndpoint"],
                )
            })
        })
}

fn declared_endpoint(tab: &CoronatioTabContract) -> Option<String> {
    declared_endpoint_text(tab).and_then(loopback_endpoint)
}

fn one_runtime_listener(tab: &CoronatioTabContract) -> Option<String> {
    let listeners = tab.listeners.as_ref()?.as_array()?;
    let loopback = listeners
        .iter()
        .filter(|listener| {
            listener
                .get("loopback")
                .and_then(serde_json::Value::as_bool)
                == Some(true)
        })
        .collect::<Vec<_>>();
    if loopback.len() != 1 {
        return None;
    }
    loopback[0]
        .get("endpoint")
        .and_then(serde_json::Value::as_str)
        .and_then(loopback_endpoint)
}

fn content_kind(headers: &hyper::HeaderMap) -> Option<&'static str> {
    let content_type = headers
        .get(hyper::header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.split(';').next())
        .map(str::trim)
        .unwrap_or("")
        .to_ascii_lowercase();
    if content_type == "text/event-stream" {
        Some("stream")
    } else if content_type == "text/html" || content_type == "application/xhtml+xml" {
        Some("html")
    } else if content_type.starts_with("application/") {
        Some("api")
    } else {
        None
    }
}

fn client() -> &'static Client<HttpConnector, Full<Bytes>> {
    static CLIENT: OnceLock<Client<HttpConnector, Full<Bytes>>> = OnceLock::new();
    CLIENT.get_or_init(|| {
        let mut connector = HttpConnector::new();
        connector.enforce_http(true);
        Client::builder(TokioExecutor::new()).build(connector)
    })
}

pub(super) fn validate_proxy_path(path: &str) -> Result<(), &'static str> {
    if !path.starts_with('/')
        || path.starts_with("//")
        || path.contains("//")
        || path.contains("://")
        || path.contains(['\r', '\n', '?', '#'])
        || path.split('/').any(|segment| segment == "..")
    {
        return Err("path-invalid");
    }
    let relative: HyperUri = path.parse().map_err(|_| "path-invalid")?;
    if relative.scheme().is_some() || relative.authority().is_some() {
        return Err("path-invalid");
    }
    Ok(())
}

fn upstream_uri(endpoint: &str, path_and_query: &str) -> Result<HyperUri, &'static str> {
    if loopback_endpoint(endpoint).is_none() {
        return Err("endpoint-not-loopback");
    }
    let path = path_and_query
        .split_once('?')
        .map(|(path, _)| path)
        .unwrap_or(path_and_query);
    validate_proxy_path(path)?;
    let relative: HyperUri = path_and_query.parse().map_err(|_| "path-invalid")?;
    if relative.scheme().is_some() || relative.authority().is_some() {
        return Err("path-invalid");
    }
    let base: HyperUri = endpoint.parse().map_err(|_| "endpoint-invalid")?;
    let scheme = base.scheme_str().ok_or("endpoint-invalid")?;
    let authority = base.authority().ok_or("endpoint-invalid")?.clone();
    let combined: HyperUri = format!("{scheme}://{authority}{path_and_query}")
        .parse()
        .map_err(|_| "uri-invalid")?;
    if combined.authority() != Some(&authority) {
        return Err("path-invalid");
    }
    Ok(combined)
}

async fn probe(endpoint: &str, path: &str, timeout: Duration) -> Result<(Option<String>, u16), &'static str> {
    let uri = upstream_uri(endpoint, path)?;
    let request = Request::builder()
        .method("GET")
        .uri(uri)
        .header(hyper::header::ACCEPT, "*/*")
        .body(Full::new(Bytes::new()))
        .map_err(|_| "probe-request-invalid")?;
    let response = tokio::time::timeout(timeout, client().request(request))
        .await
        .map_err(|_| "probe-timeout")?
        .map_err(|_| "probe-unreachable")?;
    let status = response.status().as_u16();
    let kind = content_kind(response.headers()).map(str::to_string);
    if !(200..400).contains(&status) {
        return Err("probe-not-ok");
    }
    Ok((kind, status))
}

async fn probe_health(endpoint: &str, path: &str, timeout: Duration) -> Result<(), &'static str> {
    let uri = upstream_uri(endpoint, path)?;
    let request = Request::builder()
        .method("GET")
        .uri(uri)
        .header(hyper::header::ACCEPT, "application/json")
        .body(Full::new(Bytes::new()))
        .map_err(|_| "health-request-invalid")?;
    let response = tokio::time::timeout(timeout, client().request(request))
        .await
        .map_err(|_| "health-timeout")?
        .map_err(|_| "health-unreachable")?;
    if !response.status().is_success() {
        return Err("health-http-not-ok");
    }
    let mut upstream = response.into_body();
    let body = tokio::time::timeout(timeout, async {
        let mut body = Vec::new();
        while let Some(frame) = upstream.frame().await {
            let frame = frame.map_err(|_| "health-read-failed")?;
            if let Ok(data) = frame.into_data() {
                if body.len().saturating_add(data.len()) > MAX_HEALTH_BODY {
                    return Err("health-body-too-large");
                }
                body.extend_from_slice(&data);
            }
        }
        Ok::<Vec<u8>, &'static str>(body)
    })
    .await
    .map_err(|_| "health-timeout")??;
    let value: serde_json::Value =
        serde_json::from_slice(&body).map_err(|_| "health-json-invalid")?;
    if value.get("schema").and_then(serde_json::Value::as_str) != Some("xenia.health.v1") {
        return Err("health-schema-invalid");
    }
    if value.get("ok").and_then(serde_json::Value::as_bool) != Some(true) {
        return Err("health-not-ok");
    }
    Ok(())
}

fn snapshot(result: &Discovery) -> serde_json::Value {
    serde_json::json!({
        "endpoint": result.endpoint,
        "content_kind": result.content_kind,
        "rung": result.rung,
        "at": SystemTime::now().duration_since(UNIX_EPOCH).map(|duration| duration.as_secs()).unwrap_or(0),
    })
}

fn stamp_if_changed(tab_id: &str, result: &Discovery) {
    let stamp = DiscoveryStamp {
        endpoint: result.endpoint.clone(),
        content_kind: result.content_kind.clone(),
        rung: result.rung.clone(),
    };
    let changed = {
        let mut held = stamped().lock().unwrap_or_else(|error| error.into_inner());
        if held.get(tab_id) == Some(&stamp) {
            false
        } else {
            held.insert(tab_id.to_string(), stamp);
            true
        }
    };
    if !changed {
        return;
    }
    let tab_id = tab_id.to_string();
    let discovered = snapshot(result);
    tokio::task::spawn_blocking(move || {
        let _ = caduceus_http_json_with_attendance_and_document_timeout(
            "POST",
            &format!("/api/v1/xenia/{tab_id}/observe"),
            serde_json::json!({"discovered": discovered}),
            None,
            None,
            Duration::from_secs(2),
        );
    });
}

pub(super) async fn discover(tab: &CoronatioTabContract) -> Discovery {
    if let Some(held) = cache()
        .lock()
        .unwrap_or_else(|error| error.into_inner())
        .get(&tab.id)
        .cloned()
    {
        if held.at.elapsed() < DISCOVERY_TTL {
            return held.result;
        }
    }
    let entry = tab
        .xenia_entry
        .as_ref()
        .cloned()
        .unwrap_or_else(|| serde_json::json!({}));
    let probe_timeout = entry.get("fault_recovery").and_then(|recovery| recovery.get("timeout_ms")).and_then(serde_json::Value::as_u64).map(|milliseconds| Duration::from_millis(milliseconds).max(MIN_PROBE_TIMEOUT)).unwrap_or(DEFAULT_PROBE_TIMEOUT);
    let static_dir = static_dir(&entry);
    let health_path = health_path(&entry);
    let fragment_path = fragment_path(&entry);
    let declared = declared_endpoint_text(tab).is_some();
    let source_rung = if declared { "declared" } else { "listening" };
    let endpoint = if declared {
        declared_endpoint(tab)
    } else {
        one_runtime_listener(tab)
    };
    let result = if let Some(endpoint) = endpoint {
        match probe(&endpoint, fragment_path, probe_timeout).await {
            Ok((kind, _)) => {
                let health = probe_health(&endpoint, &health_path, probe_timeout)
                    .await
                    .err()
                    .map(str::to_string);
                if let Some(signal) = health.as_deref() {
                    record_cartridge_fault_signal(
                        &tab.id,
                        CartridgeFaultKind::UpstreamError,
                        &format!("health:{signal}"),
                    );
                }
                Discovery {
                    health,
                    endpoint: Some(endpoint),
                    content_kind: kind,
                    rung: source_rung.to_string(),
                    static_dir: static_dir.clone(),
                    fault_signal: None,
                }
            }
            Err(signal) => {
                let first_missing = format!("{source_rung}:probe:{signal};endpoint={endpoint}");
                if let Some(static_dir) = static_dir.clone() {
                    Discovery { endpoint: None, content_kind: Some("static".to_string()), rung: "static".to_string(), static_dir: Some(static_dir), health: None, fault_signal: Some(first_missing) }
                } else {
                    Discovery { endpoint: None, content_kind: None, rung: "none".to_string(), static_dir: None, health: None, fault_signal: Some(first_missing) }
                }
            },
        }
    } else if let Some(static_dir) = static_dir.clone() {
        Discovery {
            endpoint: None,
            content_kind: Some("static".to_string()),
            rung: "static".to_string(),
            static_dir: Some(static_dir),
            health: None,
            fault_signal: None,
        }
    } else {
        Discovery {
            endpoint: None,
            content_kind: None,
            rung: "none".to_string(),
            static_dir: None,
            health: None,
            fault_signal: Some(
                if declared {
                    "declared:declared-endpoint-invalid"
                } else {
                    "listening:loopback-listener-not-singular"
                }
                .to_string(),
            ),
        }
    };
    {
        let mut cache = cache().lock().unwrap_or_else(|error| error.into_inner());
        if result.rung == "none" {
            cache.remove(&tab.id);
        } else {
            cache.insert(tab.id.clone(), HeldDiscovery { at: Instant::now(), result: result.clone() });
        }
    }
    stamp_if_changed(&tab.id, &result);
    result
}

fn hop_header(name: &hyper::header::HeaderName) -> bool {
    matches!(
        name.as_str().to_ascii_lowercase().as_str(),
        "connection"
            | "keep-alive"
            | "proxy-authenticate"
            | "proxy-authorization"
            | "te"
            | "trailer"
            | "transfer-encoding"
            | "upgrade"
            | "host"
            | "content-length"
    )
}

fn safe_request_header(name: &hyper::header::HeaderName) -> bool {
    matches!(
        name.as_str().to_ascii_lowercase().as_str(),
        "accept"
            | "accept-language"
            | "cache-control"
            | "content-encoding"
            | "content-language"
            | "content-type"
            | "if-match"
            | "if-modified-since"
            | "if-none-match"
            | "if-unmodified-since"
            | "last-event-id"
            | "pragma"
    )
}

fn sensitive_response_header(name: &hyper::header::HeaderName) -> bool {
    matches!(
        name.as_str().to_ascii_lowercase().as_str(),
        "set-cookie"
            | "set-cookie2"
            | "www-authenticate"
            | "authentication-info"
            | "proxy-authenticate"
            | "proxy-authentication-info"
    )
}

fn author_face() -> &'static BTreeSet<String> {
    static NAMES: OnceLock<BTreeSet<String>> = OnceLock::new();
    NAMES.get_or_init(|| {
        serde_json::from_str::<serde_json::Value>(include_str!("shell/ux/author-face.json"))
            .ok()
            .and_then(|value| {
                value
                    .get("variables")
                    .and_then(serde_json::Value::as_array)
                    .cloned()
            })
            .unwrap_or_default()
            .into_iter()
            .filter_map(|value| value.as_str().map(str::to_string))
            .collect()
    })
}

pub(super) fn pack_css_offender(css: &[u8]) -> Option<String> {
    let Ok(css) = std::str::from_utf8(css) else {
        return Some("invalid-css-utf8".to_string());
    };
    use cssparser::{ParseError, Parser, ParserInput, Token};
    fn walk<'i, 't>(
        parser: &mut Parser<'i, 't>,
        offender: &mut Option<String>,
        depth: usize,
    ) -> Result<(), ParseError<'i, String>> {
        while !parser.is_exhausted() {
            let token = parser.next_including_whitespace_and_comments()?.clone();
            match token {
                Token::Ident(name) if name.starts_with("--") => {
                    if !author_face().contains(&name[2..]) {
                        *offender = Some(name.to_string());
                        return Err(parser
                            .new_custom_error("custom property outside author face".to_string()));
                    }
                }
                Token::Function(_)
                | Token::ParenthesisBlock
                | Token::SquareBracketBlock
                | Token::CurlyBracketBlock => {
                    if depth >= 32 {
                        *offender = Some("nesting-too-deep".to_string());
                        return Err(parser.new_custom_error("CSS nesting exceeds 32".to_string()));
                    }
                    parser.parse_nested_block(|nested| walk(nested, offender, depth + 1))?
                }
                Token::BadString(_)
                | Token::BadUrl(_)
                | Token::CloseParenthesis
                | Token::CloseSquareBracket
                | Token::CloseCurlyBracket => {
                    return Err(parser.new_custom_error("malformed CSS token".to_string()));
                }
                _ => {}
            }
        }
        Ok(())
    }
    let mut input = ParserInput::new(css);
    let mut offender = None;
    match walk(&mut Parser::new(&mut input), &mut offender, 0) {
        Ok(()) => None,
        Err(_) => offender.or_else(|| Some("malformed-css".to_string())),
    }
}

pub(super) async fn proxy(
    tab_id: &str,
    endpoint: &str,
    method: Method,
    path_and_query: &str,
    headers: &axum::http::HeaderMap,
    mut body: Body,
) -> Result<Response, &'static str> {
    let uri = upstream_uri(endpoint, path_and_query)?;
    if headers
        .get(hyper::header::CONTENT_LENGTH)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse::<usize>().ok())
        .is_some_and(|length| length > MAX_BODY)
    {
        return Err("request-body-too-large");
    }
    let request_body = tokio::time::timeout(PROXY_TIMEOUT, async {
        let mut request_body = Vec::new();
        while let Some(frame) = body.frame().await {
            let frame = frame.map_err(|_| "request-body-read-failed")?;
            if let Ok(data) = frame.into_data() {
                if request_body.len().saturating_add(data.len()) > MAX_BODY {
                    return Err("request-body-too-large");
                }
                request_body.extend_from_slice(&data);
            }
        }
        Ok::<Vec<u8>, &'static str>(request_body)
    })
    .await
    .map_err(|_| "request-body-timeout")??;
    let mut builder = Request::builder().method(method.as_str()).uri(uri);
    for (name, value) in headers {
        if safe_request_header(name) {
            builder = builder.header(name, value);
        }
    }
    builder = builder.header("x-forwarded-prefix", format!("/api/tabs/{tab_id}"));
    let request = builder
        .body(Full::new(Bytes::from(request_body)))
        .map_err(|_| "proxy-request-invalid")?;
    let response = tokio::time::timeout(PROXY_TIMEOUT, client().request(request))
        .await
        .map_err(|_| "proxy-timeout")?
        .map_err(|_| "proxy-unreachable")?;
    let status = response.status();
    if status.is_redirection() {
        return Err("redirect-refused");
    }
    let response_kind = content_kind(response.headers());
    if response
        .headers()
        .get(hyper::header::CONTENT_LENGTH)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse::<usize>().ok())
        .is_some_and(|length| length > MAX_BODY)
        && response_kind != Some("stream")
    {
        return Err("response-body-too-large");
    }
    let mut response_builder = Response::builder().status(status);
    for (name, value) in response.headers() {
        if !hop_header(name)
            && !sensitive_response_header(name)
            && name != hyper::header::CACHE_CONTROL
            && name != hyper::header::CONTENT_SECURITY_POLICY
            && name.as_str() != "x-content-type-options"
        {
            response_builder = response_builder.header(name, value);
        }
    }
    response_builder = response_builder.header(hyper::header::CACHE_CONTROL, "no-store");
    response_builder = response_builder.header("x-content-type-options", "nosniff");
    if response_kind == Some("html") {
        response_builder = response_builder.header(
            hyper::header::CONTENT_SECURITY_POLICY,
            CROWN_CONTENT_SECURITY_POLICY,
        );
    }
    if response_kind == Some("stream") {
        let mut upstream = response.into_body();
        let (sender, receiver) = mpsc::channel::<Bytes>(8);
        tokio::spawn(async move {
            let deadline = tokio::time::sleep(SSE_STREAM_LIMIT);
            tokio::pin!(deadline);
            loop {
                tokio::select! {
                    _ = &mut deadline => break,
                    frame = upstream.frame() => match frame {
                        Some(Ok(frame)) => if let Ok(data) = frame.into_data() {
                            if sender.send(data).await.is_err() { break; }
                        },
                        Some(Err(_)) | None => break,
                    }
                }
            }
        });
        let stream = futures_util::stream::unfold(receiver, |mut receiver| async move {
            receiver
                .recv()
                .await
                .map(|bytes| (Ok::<Bytes, Infallible>(bytes), receiver))
        });
        return response_builder
            .body(Body::from_stream(stream))
            .map_err(|_| "proxy-response-invalid");
    }
    let mut upstream = response.into_body();
    let bytes = tokio::time::timeout(PROXY_TIMEOUT, async {
        let mut body = Vec::new();
        while let Some(frame) = upstream.frame().await {
            let frame = frame.map_err(|_| "proxy-read-failed")?;
            if let Ok(data) = frame.into_data() {
                if body.len().saturating_add(data.len()) > MAX_BODY {
                    return Err("response-body-too-large");
                }
                body.extend_from_slice(&data);
            }
        }
        Ok::<Bytes, &'static str>(Bytes::from(body))
    })
    .await
    .map_err(|_| "proxy-timeout")??;
    if path_and_query.split('?').next() == Some("/static/pack.css") {
        if let Some(offender) = pack_css_offender(&bytes) {
            record_cartridge_fault_signal(
                tab_id,
                CartridgeFaultKind::PackCssRefused,
                &format!("pack-css-refused:{offender}"),
            );
            caduceus_hyalos_reflect_best_effort(
                "xenia",
                "error".to_string(),
                format!("PackCssRefused {offender}"),
                Some(tab_id.to_string()),
                Some(serde_json::json!({"first_offender": offender, "rung": "pack-css"})),
            );
            return Ok((
                StatusCode::UNPROCESSABLE_ENTITY,
                [
                    ("content-type", "text/plain; charset=utf-8"),
                    ("x-coronatio-fault", "pack-css-refused"),
                    ("cache-control", "no-store"),
                    ("x-content-type-options", "nosniff"),
                ],
                format!("PackCssRefused: {offender}\n"),
            )
                .into_response());
        }
    }

    response_builder
        .body(Body::from(bytes))
        .map_err(|_| "proxy-response-invalid")
}
