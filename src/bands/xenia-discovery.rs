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
const PROBE_TIMEOUT: Duration = Duration::from_millis(750);
const PROXY_TIMEOUT: Duration = Duration::from_secs(8);
const SSE_STREAM_LIMIT: Duration = Duration::from_secs(35);
const MAX_BODY: usize = 2 * 1024 * 1024;
const MAX_HEALTH_BODY: usize = 64 * 1024;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Discovery {
    pub endpoint: Option<String>,
    pub content_kind: String,
    pub rung: String,
    pub static_dir: Option<String>,
    pub health: Option<String>,
}

#[derive(Clone)]
struct HeldDiscovery {
    at: Instant,
    result: Discovery,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct DiscoveryStamp {
    endpoint: Option<String>,
    content_kind: String,
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
        .or_else(|| {
            transport.and_then(|value| value.get("endpoint").and_then(serde_json::Value::as_str))
        })
}

fn declared_endpoint(tab: &CoronatioTabContract) -> Option<String> {
    declared_endpoint_text(tab).and_then(loopback_endpoint)
}

fn one_runtime_listener(tab: &CoronatioTabContract) -> Option<String> {
    let listeners = tab.listeners.as_ref()?.as_array()?;
    let mut loopback = listeners.iter().filter_map(|listener| {
        (listener
            .get("loopback")
            .and_then(serde_json::Value::as_bool)
            == Some(true))
        .then(|| {
            listener
                .get("endpoint")
                .and_then(serde_json::Value::as_str)
                .and_then(loopback_endpoint)
        })
        .flatten()
    });
    let only = loopback.next()?;
    loopback.next().is_none().then_some(only)
}

fn content_kind(headers: &hyper::HeaderMap) -> &'static str {
    let content_type = headers
        .get(hyper::header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.split(';').next())
        .map(str::trim)
        .unwrap_or("")
        .to_ascii_lowercase();
    if content_type == "text/event-stream" {
        "stream"
    } else if content_type == "text/html" || content_type == "application/xhtml+xml" {
        "html"
    } else if content_type.starts_with("application/") {
        "api"
    } else {
        "none"
    }
}

fn client() -> Client<HttpConnector, Full<Bytes>> {
    let mut connector = HttpConnector::new();
    connector.enforce_http(true);
    Client::builder(TokioExecutor::new()).build(connector)
}

fn upstream_uri(endpoint: &str, path_and_query: &str) -> Result<HyperUri, &'static str> {
    let base = url::Url::parse(endpoint).map_err(|_| "endpoint-invalid")?;
    let joined = base
        .join(path_and_query.trim_start_matches('/'))
        .map_err(|_| "path-invalid")?;
    if loopback_endpoint(endpoint).is_none() {
        return Err("endpoint-not-loopback");
    }
    joined.as_str().parse().map_err(|_| "uri-invalid")
}

async fn probe(endpoint: &str, path: &str) -> Result<(String, u16), &'static str> {
    let uri = upstream_uri(endpoint, path)?;
    let request = Request::builder()
        .method("GET")
        .uri(uri)
        .header(hyper::header::ACCEPT, "*/*")
        .body(Full::new(Bytes::new()))
        .map_err(|_| "probe-request-invalid")?;
    let response = tokio::time::timeout(PROBE_TIMEOUT, client().request(request))
        .await
        .map_err(|_| "probe-timeout")?
        .map_err(|_| "probe-unreachable")?;
    let status = response.status().as_u16();
    let kind = content_kind(response.headers()).to_string();
    if !(200..400).contains(&status) {
        return Err("probe-not-ok");
    }
    Ok((kind, status))
}

async fn probe_health(endpoint: &str, path: &str) -> Result<(), &'static str> {
    let uri = upstream_uri(endpoint, path)?;
    let request = Request::builder()
        .method("GET")
        .uri(uri)
        .header(hyper::header::ACCEPT, "application/json")
        .body(Full::new(Bytes::new()))
        .map_err(|_| "health-request-invalid")?;
    let response = tokio::time::timeout(PROBE_TIMEOUT, client().request(request))
        .await
        .map_err(|_| "health-timeout")?
        .map_err(|_| "health-unreachable")?;
    if !response.status().is_success() {
        return Err("health-http-not-ok");
    }
    let mut upstream = response.into_body();
    let body = tokio::time::timeout(PROBE_TIMEOUT, async {
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
    let static_dir = static_dir(&entry);
    let health_path = health_path(&entry);
    let declared = declared_endpoint_text(tab).is_some();
    let source_rung = if declared { "declared" } else { "listening" };
    let endpoint = if declared {
        declared_endpoint(tab)
    } else {
        one_runtime_listener(tab)
    };
    let result = if let Some(endpoint) = endpoint {
        match probe(&endpoint, "/").await {
            Ok((kind, _)) => Discovery {
                health: probe_health(&endpoint, &health_path)
                    .await
                    .err()
                    .map(str::to_string),
                endpoint: Some(endpoint),
                content_kind: kind,
                rung: source_rung.to_string(),
                static_dir: static_dir.clone(),
            },
            Err(_) if static_dir.is_some() => Discovery {
                endpoint: None,
                content_kind: "static".to_string(),
                rung: "static".to_string(),
                static_dir: static_dir.clone(),
                health: None,
            },
            Err(signal) => Discovery {
                endpoint: None,
                content_kind: "none".to_string(),
                rung: "probe".to_string(),
                static_dir: None,
                health: Some(signal.to_string()),
            },
        }
    } else if let Some(static_dir) = static_dir.clone() {
        Discovery {
            endpoint: None,
            content_kind: "static".to_string(),
            rung: "static".to_string(),
            static_dir: Some(static_dir),
            health: None,
        }
    } else {
        Discovery {
            endpoint: None,
            content_kind: "none".to_string(),
            rung: if declared { "declared" } else { "listening" }.to_string(),
            static_dir: None,
            health: Some(
                if declared {
                    "declared-endpoint-invalid"
                } else {
                    "loopback-listener-not-singular"
                }
                .to_string(),
            ),
        }
    };
    cache()
        .lock()
        .unwrap_or_else(|error| error.into_inner())
        .insert(
            tab.id.clone(),
            HeldDiscovery {
                at: Instant::now(),
                result: result.clone(),
            },
        );
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
                    parser.parse_nested_block(|nested| walk(nested, offender))?
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
    match walk(&mut Parser::new(&mut input), &mut offender) {
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
    body: Bytes,
) -> Result<Response, &'static str> {
    if body.len() > MAX_BODY {
        return Err("request-body-too-large");
    }
    let uri = upstream_uri(endpoint, path_and_query)?;
    let mut builder = Request::builder().method(method.as_str()).uri(uri);
    for (name, value) in headers {
        if safe_request_header(name) {
            builder = builder.header(name, value);
        }
    }
    builder = builder.header("x-forwarded-prefix", format!("/api/tabs/{tab_id}"));
    let request = builder
        .body(Full::new(body))
        .map_err(|_| "proxy-request-invalid")?;
    let response = tokio::time::timeout(PROXY_TIMEOUT, client().request(request))
        .await
        .map_err(|_| "proxy-timeout")?
        .map_err(|_| "proxy-unreachable")?;
    let status = response.status();
    let response_kind = content_kind(response.headers());
    let mut response_builder = Response::builder().status(status);
    for (name, value) in response.headers() {
        if !hop_header(name) && !sensitive_response_header(name) {
            response_builder = response_builder.header(name, value);
        }
    }
    if response_kind == "stream" {
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
    let mut bytes = tokio::time::timeout(PROXY_TIMEOUT, response.into_body().collect())
        .await
        .map_err(|_| "proxy-timeout")?
        .map_err(|_| "proxy-read-failed")?
        .to_bytes();
    if bytes.len() > MAX_BODY {
        return Err("response-body-too-large");
    }
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
                ],
                format!("PackCssRefused: {offender}\n"),
            )
                .into_response());
        }
    }
    if path_and_query.split('?').next() == Some("/fragment") && response_kind == "html" {
        let source = String::from_utf8(bytes.to_vec()).map_err(|_| "fragment-invalid-utf8")?;
        let prefix = format!("/api/tabs/{tab_id}");
        let rewritten = source
            .replace("\"/events/renew\"", &format!("\"{prefix}/events/renew\""))
            .replace("\"/events\"", &format!("\"{prefix}/events\""))
            .replace(
                "\"/static/pack.css\"",
                &format!("\"{prefix}/static/pack.css\""),
            );
        bytes = Bytes::from(rewritten);
    }
    response_builder
        .body(Body::from(bytes))
        .map_err(|_| "proxy-response-invalid")
}
