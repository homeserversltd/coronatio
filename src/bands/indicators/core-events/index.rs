use axum::response::sse::{Event, KeepAlive, Sse};
use futures_util::{stream, StreamExt};
use std::{collections::HashMap, convert::Infallible, time::Instant};

const CORE_LEASE_SECONDS: u64 = 30;
static CORE_MEMBERSHIPS: OnceLock<Mutex<HashMap<String, CoreMembership>>> = OnceLock::new();
fn core_memberships() -> &'static Mutex<HashMap<String, CoreMembership>> { CORE_MEMBERSHIPS.get_or_init(|| Mutex::new(HashMap::new())) }

struct CoreMembership {
    deadline: Instant,
    session: Session,
    document: Option<String>,
    control_tx: tokio::sync::mpsc::UnboundedSender<CoreControl>,
}

#[derive(Clone, Copy)]
enum CoreControl { Set(Session) }

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CoreRenewQuery { stream_id: String }

struct CoreStreamState {
    stream_id: String,
    session: Session,
    control_rx: tokio::sync::mpsc::UnboundedReceiver<CoreControl>,
    index: usize,
    opened: bool,
}
impl Drop for CoreStreamState { fn drop(&mut self) { core_memberships().lock().unwrap().remove(&self.stream_id); } }

pub(crate) async fn core_pulse_route(headers: axum::http::HeaderMap) -> Response {
    let session = session_from_headers(&headers);
    let (_id, frames) = subscribe_core_stream(session, Duration::from_secs(CORE_LEASE_SECONDS));
    Sse::new(frames.map(|frame| Ok::<Event, Infallible>(Event::default().event(frame.0).id(frame.1).data(frame.2))))
        .keep_alive(KeepAlive::new().interval(Duration::from_secs(10)).text("core.keepalive"))
        .into_response()
}

pub(crate) async fn core_pulse_renew_route(headers: axum::http::HeaderMap, Query(query): Query<CoreRenewQuery>) -> Response {
    if !validate_core_host_membership(&query.stream_id, &headers) {
        return core_membership_response(StatusCode::UNAUTHORIZED, &query.stream_id, "attendance-refused");
    }
    if renew_core_stream(&query.stream_id, Duration::from_secs(CORE_LEASE_SECONDS)) {
        Json(serde_json::json!({"schema":"coronatio.core.events.renewal.v1","streamId":query.stream_id,"status":"renewed","leaseSeconds":CORE_LEASE_SECONDS})).into_response()
    } else {
        (StatusCode::NOT_FOUND, Json(serde_json::json!({"schema":"coronatio.core.events.renewal.v1","streamId":query.stream_id,"status":"unknown-stream"}))).into_response()
    }
}

pub(crate) async fn core_pulse_upgrade_route(headers: axum::http::HeaderMap, Query(query): Query<CoreRenewQuery>) -> Response {
    let Some(document) = crate::caduceus_access::document_incarnation_from_headers(&headers) else {
        return core_membership_response(StatusCode::BAD_REQUEST, &query.stream_id, "document-required");
    };
    if session_from_headers(&headers) != Session::Admin {
        downgrade_core_stream(&query.stream_id, None);
        return core_membership_response(StatusCode::UNAUTHORIZED, &query.stream_id, "attendance-refused");
    }
    if upgrade_core_stream(&query.stream_id, document) {
        core_membership_response(StatusCode::OK, &query.stream_id, "upgraded")
    } else {
        core_membership_response(StatusCode::NOT_FOUND, &query.stream_id, "unknown-stream")
    }
}

pub(crate) async fn core_pulse_downgrade_route(headers: axum::http::HeaderMap, Query(query): Query<CoreRenewQuery>) -> Response {
    let Some(document) = crate::caduceus_access::document_incarnation_from_headers(&headers) else {
        return core_membership_response(StatusCode::BAD_REQUEST, &query.stream_id, "document-required");
    };
    if downgrade_core_stream(&query.stream_id, Some(&document)) {
        core_membership_response(StatusCode::OK, &query.stream_id, "downgraded")
    } else {
        core_membership_response(StatusCode::NOT_FOUND, &query.stream_id, "unknown-stream")
    }
}

fn core_membership_response(status: StatusCode, stream_id: &str, membership_status: &str) -> Response {
    (status, Json(serde_json::json!({"schema":"coronatio.core.events.membership.v1","streamId":stream_id,"status":membership_status}))).into_response()
}

pub(crate) fn subscribe_core_stream(session: Session, lease: Duration) -> (String, stream::BoxStream<'static, (String, String, String)>) {
    let stream_id = format!("core-{}", uuid::Uuid::new_v4());
    let (control_tx, control_rx) = tokio::sync::mpsc::unbounded_channel();
    core_memberships().lock().unwrap().insert(stream_id.clone(), CoreMembership {
        deadline: Instant::now() + lease,
        session,
        document: None,
        control_tx,
    });
    let state = CoreStreamState { stream_id: stream_id.clone(), session, control_rx, index: 0, opened: false };
    let frames = stream::unfold(Some(state), move |state| async move {
        let mut state = state?;
        while let Ok(CoreControl::Set(session)) = state.control_rx.try_recv() { state.session = session; }
        if Instant::now() >= core_memberships().lock().unwrap().get(&state.stream_id).map(|membership| membership.deadline).unwrap_or_else(Instant::now) {
            let id = state.stream_id.clone(); core_memberships().lock().unwrap().remove(&id);
            return Some((("core.expired".into(), id.clone(), serde_json::json!({"streamId":id,"status":"expired"}).to_string()), None));
        }
        if !state.opened {
            state.opened = true;
            let id = state.stream_id.clone();
            let data = serde_json::json!({"schema":"coronatio.core.events.v1","streamId":id,"leaseSeconds":CORE_LEASE_SECONDS,"renewRoute":format!("/api/core/pulse/renew?streamId={}", state.stream_id),"topics":catalog().iter().map(|e| e.topic_id).collect::<Vec<_>>()}).to_string();
            return Some((("core.open".into(), id, data), Some(state)));
        }
        if state.index >= catalog().len() { tokio::time::sleep(Duration::from_secs(1)).await; state.index = 0; }
        while let Ok(CoreControl::Set(session)) = state.control_rx.try_recv() { state.session = session; }
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
    if let Some(membership) = core_memberships().lock().unwrap().get_mut(stream_id) { membership.deadline = Instant::now() + lease; true } else { false }
}

pub(crate) fn upgrade_core_stream(stream_id: &str, document: String) -> bool {
    let mut memberships = core_memberships().lock().unwrap();
    let Some(membership) = memberships.get_mut(stream_id) else { return false; };
    membership.session = Session::Admin;
    membership.document = Some(document);
    membership.control_tx.send(CoreControl::Set(Session::Admin)).is_ok()
}

pub(crate) fn downgrade_core_stream(stream_id: &str, document: Option<&str>) -> bool {
    let mut memberships = core_memberships().lock().unwrap();
    let Some(membership) = memberships.get_mut(stream_id) else { return false; };
    if document.is_some() && membership.document.as_deref() != document { return false; }
    membership.session = Session::Guest;
    membership.document = None;
    membership.control_tx.send(CoreControl::Set(Session::Guest)).is_ok()
}

pub(crate) fn downgrade_core_document(document: &str) {
    let mut memberships = core_memberships().lock().unwrap();
    for membership in memberships.values_mut().filter(|membership| membership.document.as_deref() == Some(document)) {
        membership.session = Session::Guest;
        membership.document = None;
        let _ = membership.control_tx.send(CoreControl::Set(Session::Guest));
    }
}

fn validate_core_host_membership(stream_id: &str, headers: &axum::http::HeaderMap) -> bool {
    let is_host = core_memberships().lock().unwrap().get(stream_id).is_some_and(|membership| membership.session == Session::Admin);
    if is_host && session_from_headers(headers) != Session::Admin {
        downgrade_core_stream(stream_id, None);
        false
    } else {
        true
    }
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
