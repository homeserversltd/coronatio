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
    last_collected: HashMap<&'static str, Instant>,
    last_payload: HashMap<&'static str, String>,
}
impl Drop for CoreStreamState { fn drop(&mut self) { core_memberships().lock().unwrap().remove(&self.stream_id); } }

pub(crate) async fn core_pulse_route(headers: axum::http::HeaderMap) -> Response {
    let session = session_projection_from_headers(&headers).await;
    let (_id, frames) = subscribe_core_stream(session, Duration::from_secs(CORE_LEASE_SECONDS));
    Sse::new(frames.map(|frame| Ok::<Event, Infallible>(Event::default().event(frame.0).id(frame.1).data(frame.2))))
        .keep_alive(KeepAlive::new().interval(Duration::from_secs(10)).text("core.keepalive"))
        .into_response()
}

pub(crate) async fn core_pulse_renew_route(headers: axum::http::HeaderMap, Query(query): Query<CoreRenewQuery>) -> Response {
    if !validate_core_host_membership(&query.stream_id, &headers).await {
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
    if session_projection_from_headers(&headers).await != Session::Admin {
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
    if let Some(attendance) = crate::caduceus_access::attendance_from_headers(&headers) { crate::caduceus_access::bust_attendance_projection(&attendance, &document, "stream-downgraded"); }
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
    let state = CoreStreamState { stream_id: stream_id.clone(), session, control_rx, index: 0, opened: false, last_collected: HashMap::new(), last_payload: HashMap::new() };
    let frames = stream::unfold(Some(state), move |state| async move {
        let mut state = state?;
        while let Ok(CoreControl::Set(session)) = state.control_rx.try_recv() {
            if state.session != session { state.session = session; state.last_collected.clear(); state.last_payload.clear(); }
        }
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
        while let Ok(CoreControl::Set(session)) = state.control_rx.try_recv() {
            if state.session != session { state.session = session; state.last_collected.clear(); state.last_payload.clear(); }
        }
        let entry = catalog()[state.index]; state.index += 1;
        let now = Instant::now();
        let refresh_due = state.last_collected.get(entry.topic_id).map(|last| now.duration_since(*last) >= minimum_refresh_interval(entry.topic_id)).unwrap_or(true);
        let payload = if refresh_due {
            tracing::debug!(topic_id = entry.topic_id, "collecting core indicator topic");
            let collected = if entry.topic_id == "vpn.status" {
                Some(collect_transmission_indicator_sample(state.session))
            } else {
                entry.collector.map(|collector| collector(state.session).map(|snapshot| IndicatorTopicSample {
                    snapshot,
                    indicator_state: None,
                }))
            };
            let payload = match collected {
                Some(Ok(sample)) => {
                    let mut envelope = serde_json::json!({"schema":"coronatio.core.topic.v1","topicId":entry.topic_id,"status":"snapshot","snapshot":sample.snapshot});
                    if let Some(indicator_state) = sample.indicator_state {
                        envelope["indicatorState"] = serde_json::Value::String(indicator_state.to_string());
                    }
                    envelope
                }
                Some(Err(error)) => {
                    let mut envelope = serde_json::json!({"schema":"coronatio.core.topic.v1","topicId":entry.topic_id,"status":"unavailable","fault":error});
                    if entry.topic_id == "vpn.status" {
                        envelope["indicatorState"] = serde_json::Value::String("unavailable".to_string());
                    }
                    envelope
                }
                None => {
                    let mut envelope = serde_json::json!({"schema":"coronatio.core.topic.v1","topicId":entry.topic_id,"status":"unavailable","fault":"collector absent"});
                    if entry.topic_id == "vpn.status" {
                        envelope["indicatorState"] = serde_json::Value::String("unavailable".to_string());
                    }
                    envelope
                }
            }.to_string();

            state.last_collected.insert(entry.topic_id, now);
            state.last_payload.insert(entry.topic_id, payload.clone());
            payload
        } else { state.last_payload.get(entry.topic_id).expect("cached core topic payload").clone() };
        Some(((entry.topic_id.into(), format!("{}:{}", state.stream_id, state.index), payload), Some(state)))
    }).boxed();
    (stream_id, frames)
}

fn minimum_refresh_interval(topic_id: &str) -> Duration {
    match topic_id {
        "source.currency" => Duration::from_secs(60),
        "tailscale.status" | "vpn.status" | "services.status" => Duration::from_secs(15),
        _ => Duration::from_secs(1),
    }
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

async fn validate_core_host_membership(stream_id: &str, headers: &axum::http::HeaderMap) -> bool {
    let is_host = core_memberships().lock().unwrap().get(stream_id).is_some_and(|membership| membership.session == Session::Admin);
    if is_host && session_projection_from_headers(headers).await != Session::Admin {
        downgrade_core_stream(stream_id, None);
        false
    } else {
        true
    }
}

pub(crate) struct IndicatorTopicSample {
    pub(crate) snapshot: serde_json::Value,
    pub(crate) indicator_state: Option<&'static str>,
}

pub(crate) fn collect_indicator_topic(topic_id: &str, session: Session) -> Result<serde_json::Value, String> {
    match topic_id {
        "internet.status" => {
            let raw = internet_status_snapshot();
            serde_json::to_value(if session == Session::Admin {
                serde_json::to_value(project_internet_status_admin(&raw)).unwrap()
            } else {
                serde_json::to_value(project_internet_status_guest(&raw)).unwrap()
            })
            .map_err(|error| error.to_string())
        }
        "power.status" => read_power_usage_sample()
            .map(|sample| serde_json::json!({"ok":true,"status":"available","current":sample.current_watts,"historical":sample.history_watts,"unit":"W","timestamp":sample.timestamp_secs}))
            .map_err(str::to_string),
        "tailscale.status" => collect_caduceus_indicator("/api/v1/tailscale/status", session, &[
            "ok", "success", "status", "interface", "timestamp", "firstMissingSignal",
        ]),
        "vpn.status" => collect_transmission_indicator_sample(session).map(|sample| sample.snapshot),
        "services.status" => collect_services_indicator(),
        "source.currency" => collect_source_currency_indicator(session),
        _ => Err(format!("unknown topic: {topic_id}")),
    }
}

fn collect_transmission_indicator_sample(session: Session) -> Result<IndicatorTopicSample, String> {
    const PATH: &str = "/api/v1/transmission/status";
    let readback = caduceus_http("GET", PATH);
    if !readback.ok {
        return Err(format!("{}: {PATH}", readback.first_missing_signal));
    }
    let Some(object) = readback.body.as_object() else {
        return Err(format!("caduceus-invalid-json: {PATH}"));
    };
    Ok(project_transmission_status(object, session))
}

pub(crate) fn project_transmission_status(
    object: &serde_json::Map<String, serde_json::Value>,
    session: Session,
) -> IndicatorTopicSample {
    let conditions = object.get("conditions").and_then(serde_json::Value::as_object);
    let condition = |name: &str| {
        conditions
            .and_then(|values| values.get(name))
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let namespace = condition("namespace");
    let tunnel = condition("tunnel");
    let forward = condition("forward");
    let daemon = condition("daemon");
    let peer_port = condition("peerPort");
    let vpn_status = if namespace && tunnel && forward { "running" } else { "stopped" };
    let transmission_status = if daemon { "running" } else { "stopped" };
    let satisfied = [namespace, tunnel, forward, daemon, peer_port]
        .into_iter()
        .filter(|condition| *condition)
        .count();
    let indicator_state = match satisfied {
        5 => "ok",
        1..=4 => "warn",
        _ => "error",
    };
    let timestamp = object.get("timestamp").cloned().unwrap_or(serde_json::Value::Null);

    if session == Session::Guest {
        let mut projection = serde_json::Map::new();
        projection.insert("ok".to_string(), object.get("ok").cloned().unwrap_or(serde_json::Value::Null));
        projection.insert("vpnStatus".to_string(), serde_json::Value::String(vpn_status.to_string()));
        projection.insert("transmissionStatus".to_string(), serde_json::Value::String(transmission_status.to_string()));
        projection.insert("timestamp".to_string(), timestamp);
        projection.insert("firstMissingSignal".to_string(), object.get("firstMissingSignal").cloned().unwrap_or(serde_json::Value::Null));
        return IndicatorTopicSample { snapshot: serde_json::Value::Object(projection), indicator_state: Some(indicator_state) };
    }

    let mut projection = object.clone();
    projection.insert("vpnStatus".to_string(), serde_json::Value::String(vpn_status.to_string()));
    projection.insert("transmissionStatus".to_string(), serde_json::Value::String(transmission_status.to_string()));
    projection.entry("timestamp".to_string()).or_insert(timestamp);
    IndicatorTopicSample { snapshot: serde_json::Value::Object(projection), indicator_state: Some(indicator_state) }
}

fn collect_caduceus_indicator(path: &str, session: Session, guest_fields: &[&str]) -> Result<serde_json::Value, String> {
    let readback = caduceus_http("GET", path);
    if !readback.ok {
        return Err(format!("{}: {}", readback.first_missing_signal, path));
    }
    let Some(object) = readback.body.as_object() else {
        return Err(format!("caduceus-invalid-json: {path}"));
    };
    if session == Session::Admin {
        return Ok(serde_json::Value::Object(object.clone()));
    }
    let mut projection = serde_json::Map::new();
    for field in guest_fields {
        if let Some(value) = object.get(*field) {
            projection.insert((*field).to_string(), value.clone());
        }
    }
    projection.entry("ok".to_string()).or_insert(serde_json::Value::Bool(true));
    Ok(serde_json::Value::Object(projection))
}

fn collect_source_currency_indicator(session: Session) -> Result<serde_json::Value, String> {
    if crate::CORONATIO_BUILD_SHA.is_empty() {
        return Ok(serde_json::json!({
            "ok": false,
            "schema": "caduceus.coronatio.source_currency.v1",
            "status": "unavailable",
            "originMainSha": null,
            "buildSha": "",
            "relation": "unknown",
            "firstMissingSignal": "CORONATIO_BUILD_SHA",
        }));
    }
    collect_caduceus_indicator(
        &format!("/api/v1/coronatio/source-currency?buildSha={}", crate::CORONATIO_BUILD_SHA),
        session,
        &["ok", "schema", "originMainSha", "buildSha", "relation", "firstMissingSignal"],
    )
}

fn collect_services_indicator() -> Result<serde_json::Value, String> {
    let portals = read_portals_config()?.portals;
    let mut services = Vec::new();
    for portal in portals {
        let mut systemd_names = Vec::new();
        let mut states = Vec::new();
        let mut checked = 0_usize;
        let mut active = 0_usize;
        for service in portal.services {
            let systemd_name = normalize_systemd_unit(&service);
            let state = systemctl_is_active(&systemd_name);
            systemd_names.push(systemd_name);
            states.push(state.clone().unwrap_or_else(|| "unavailable".to_string()));
            match state.as_deref() {
                Some("active") => {
                    checked += 1;
                    active += 1;
                }
                Some(_) => checked += 1,
                None => {}
            }
        }
        let status = match (checked, active) {
            (0, _) => "unknown",
            (checked, active) if checked == active => "up",
            (_, 0) => "down",
            _ => "partial",
        };
        services.push(serde_json::json!({
            "name": portal.name,
            "description": portal.description,
            "systemdName": systemd_names.join(", "),
            "isActive": status == "up",
            "status": status,
            "statusDetails": states.join(", "),
            "isScriptManaged": portal.r#type == "script",
            "port": portal.port,
            "needsReboot": portal.r#type == "script",
        }));
    }
    let (checked, active) = services.iter().fold((0_usize, 0_usize), |(checked, active), service| {
        match service.get("status").and_then(serde_json::Value::as_str) {
            Some("up") => (checked + 1, active + 1),
            Some("down") => (checked + 1, active),
            _ => (checked, active),
        }
    });
    let status = match (checked, active) {
        (0, _) => "unknown",
        (checked, active) if checked == active => "up",
        (_, 0) => "down",
        _ => "partial",
    };
    Ok(serde_json::json!({
        "ok": true,
        "status": status,
        "services": services,
        "timestamp": SystemTime::now().duration_since(UNIX_EPOCH).map(|duration| duration.as_secs_f64()).unwrap_or_default(),
    }))
}
