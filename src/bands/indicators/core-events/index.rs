use axum::response::sse::{Event, KeepAlive, Sse};
use futures_util::{stream, StreamExt};
use std::{collections::HashMap, convert::Infallible, time::Instant};

const CORE_LEASE_SECONDS: u64 = 30;
static CORE_LEASES: OnceLock<Mutex<HashMap<String, Instant>>> = OnceLock::new();
fn core_leases() -> &'static Mutex<HashMap<String, Instant>> { CORE_LEASES.get_or_init(|| Mutex::new(HashMap::new())) }

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CoreRenewQuery { stream_id: String }

#[derive(Clone)]
struct CoreStreamState { stream_id: String, session: Session, index: usize, opened: bool }
impl Drop for CoreStreamState { fn drop(&mut self) { core_leases().lock().unwrap().remove(&self.stream_id); } }

pub(crate) async fn core_pulse_route(headers: axum::http::HeaderMap) -> Response {
    let session = session_from_headers(&headers);
    let (_id, frames) = subscribe_core_stream(session, Duration::from_secs(CORE_LEASE_SECONDS));
    Sse::new(frames.map(|frame| Ok::<Event, Infallible>(Event::default().event(frame.0).id(frame.1).data(frame.2))))
        .keep_alive(KeepAlive::new().interval(Duration::from_secs(10)).text("core.keepalive"))
        .into_response()
}

pub(crate) async fn core_pulse_renew_route(Query(query): Query<CoreRenewQuery>) -> Response {
    if renew_core_stream(&query.stream_id, Duration::from_secs(CORE_LEASE_SECONDS)) {
        Json(serde_json::json!({"schema":"coronatio.core.events.renewal.v1","streamId":query.stream_id,"status":"renewed","leaseSeconds":CORE_LEASE_SECONDS})).into_response()
    } else {
        (StatusCode::NOT_FOUND, Json(serde_json::json!({"schema":"coronatio.core.events.renewal.v1","streamId":query.stream_id,"status":"unknown-stream"}))).into_response()
    }
}

pub(crate) fn subscribe_core_stream(session: Session, lease: Duration) -> (String, stream::BoxStream<'static, (String, String, String)>) {
    let stream_id = format!("core-{}", uuid::Uuid::new_v4());
    core_leases().lock().unwrap().insert(stream_id.clone(), Instant::now() + lease);
    let state = CoreStreamState { stream_id: stream_id.clone(), session, index: 0, opened: false };
    let frames = stream::unfold(Some(state), move |state| async move {
        let mut state = state?;
        if Instant::now() >= core_leases().lock().unwrap().get(&state.stream_id).copied().unwrap_or_else(Instant::now) {
            let id = state.stream_id.clone(); core_leases().lock().unwrap().remove(&id);
            return Some((("core.expired".into(), id.clone(), serde_json::json!({"streamId":id,"status":"expired"}).to_string()), None));
        }
        if !state.opened {
            state.opened = true;
            let id = state.stream_id.clone();
            let data = serde_json::json!({"schema":"coronatio.core.events.v1","streamId":id,"leaseSeconds":CORE_LEASE_SECONDS,"renewRoute":format!("/api/core/pulse/renew?streamId={}", state.stream_id),"topics":catalog().iter().map(|e| e.topic_id).collect::<Vec<_>>()}).to_string();
            return Some((("core.open".into(), id, data), Some(state)));
        }
        if state.index >= catalog().len() { tokio::time::sleep(Duration::from_secs(1)).await; state.index = 0; }
        let entry = catalog()[state.index]; state.index += 1;
        let payload = match entry.collector.map(|collector| collector(state.session)) {
            Some(Ok(value)) => serde_json::json!({"schema":"coronatio.core.topic.v1","topicId":entry.topic_id,"status":"snapshot","snapshot":value}),
            Some(Err(error)) => serde_json::json!({"schema":"coronatio.core.topic.v1","topicId":entry.topic_id,"status":"unavailable","fault":error}),
            None => serde_json::json!({"schema":"coronatio.core.topic.v1","topicId":entry.topic_id,"status":"unavailable","fault":"collector absent"}),
        };
        Some(((entry.topic_id.into(), format!("{}:{}", state.stream_id, state.index), payload.to_string()), Some(state)))
    }).boxed();
    (stream_id, frames)
}

pub(crate) fn renew_core_stream(stream_id: &str, lease: Duration) -> bool {
    if let Some(deadline) = core_leases().lock().unwrap().get_mut(stream_id) { *deadline = Instant::now() + lease; true } else { false }
}

pub(crate) fn collect_indicator_topic(topic_id: &str, session: Session) -> Result<serde_json::Value, String> {
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs_f64()).unwrap_or_default();
    match topic_id {
        "internet.status" => { let raw = internet_status_snapshot(); serde_json::to_value(if session == Session::Admin { serde_json::to_value(project_internet_status_admin(&raw)).unwrap() } else { serde_json::to_value(project_internet_status_guest(&raw)).unwrap() }).map_err(|e| e.to_string()) },
        "power.status" => read_power_usage_sample().map(|sample| serde_json::json!({"ok":true,"status":"available","current":sample.current_watts,"historical":sample.history_watts,"unit":"W","timestamp":sample.timestamp_secs})).map_err(str::to_string),
        "tailscale.status" => Ok(serde_json::json!({"ok":true,"status":"rust-route","interface":"tailscale0","timestamp":timestamp,"scope":if session == Session::Admin {"admin"} else {"guest"}})),
        "vpn.status" => Ok(serde_json::json!({"ok":true,"vpnStatus":"rust-route","transmissionStatus":"rust-route","timestamp":timestamp,"scope":if session == Session::Admin {"admin"} else {"guest"}})),
        "services.status" => Ok(serde_json::json!({"ok":true,"status":"rust-route","services":[],"timestamp":timestamp,"scope":if session == Session::Admin {"admin"} else {"guest"}})),
        _ => Err(format!("unknown topic: {topic_id}")),
    }
}
