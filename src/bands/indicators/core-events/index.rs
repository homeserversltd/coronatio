use axum::response::sse::{Event, KeepAlive, Sse};
use futures_util::{stream, StreamExt};
use std::{collections::HashMap, convert::Infallible, time::Instant};

const CORE_LEASE_SECONDS: u64 = 30;
static CORE_MEMBERSHIPS: OnceLock<Mutex<HashMap<String, CoreMembership>>> = OnceLock::new();
fn core_memberships() -> &'static Mutex<HashMap<String, CoreMembership>> {
    CORE_MEMBERSHIPS.get_or_init(|| Mutex::new(HashMap::new()))
}

struct CoreMembership {
    deadline: Instant,
    session: Session,
    document: Option<String>,
    parent_attendance: Option<crate::caduceus_access::AttendanceProof>,
    admin_stats_cache: Option<std::sync::Arc<ScopedAdminStatsCache>>,
    fallback_signal: Option<String>,
    generation: u64,
    upgrading: bool,
    control_tx: tokio::sync::mpsc::UnboundedSender<CoreControl>,
}

#[derive(Clone, Copy)]
enum CoreControl {
    Set(Session),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CoreRenewQuery {
    stream_id: String,
}

#[derive(Clone)]
struct CoreAdminContext {
    attendance: crate::caduceus_access::AttendanceProof,
    document: String,
    stats_cache: std::sync::Arc<ScopedAdminStatsCache>,
    generation: u64,
}

#[derive(Clone)]
struct CoreLiveProjection {
    session: Session,
    generation: u64,
    admin: Option<CoreAdminContext>,
    fallback_signal: Option<String>,
}

#[derive(Clone)]
struct CorePulseBatch {
    generation: u64,
    current: CaduceusStatsReadback,
    admin: Option<CaduceusStatsReadback>,
    admin_signal: Option<String>,
    fallback_signal: Option<String>,
}

struct CoreStreamState {
    stream_id: String,
    session: Session,
    generation: u64,
    control_rx: tokio::sync::mpsc::UnboundedReceiver<CoreControl>,
    index: usize,
    opened: bool,
    last_collected: HashMap<&'static str, Instant>,
    last_payload: HashMap<&'static str, String>,
    pulse_batch: Option<CorePulseBatch>,
}

impl Drop for CoreStreamState {
    fn drop(&mut self) {
        remove_core_membership(&self.stream_id, "stream-dropped");
    }
}

pub(crate) async fn core_pulse_route(headers: axum::http::HeaderMap) -> Response {
    let session = session_projection_from_headers(&headers).await;
    let (stream_id, frames) = subscribe_core_stream(
        Session::Guest,
        Duration::from_secs(CORE_LEASE_SECONDS),
    );
    if session == Session::Admin {
        if let (Some(document), Some(parent)) = (
            crate::caduceus_access::document_incarnation_from_headers(&headers),
            crate::caduceus_access::attendance_from_headers(&headers),
        ) {
            let _ = upgrade_core_stream_attended(&stream_id, document, parent).await;
        }
    }
    Sse::new(frames.map(|frame| {
        Ok::<Event, Infallible>(Event::default().event(frame.0).id(frame.1).data(frame.2))
    }))
    .keep_alive(KeepAlive::new().interval(Duration::from_secs(10)).text("core.keepalive"))
    .into_response()
}

pub(crate) async fn core_pulse_renew_route(
    headers: axum::http::HeaderMap,
    Query(query): Query<CoreRenewQuery>,
) -> Response {
    if !validate_core_host_membership(&query.stream_id, &headers).await {
        return core_membership_response(
            StatusCode::UNAUTHORIZED,
            &query.stream_id,
            "attendance-refused",
        );
    }
    if renew_core_stream(&query.stream_id, Duration::from_secs(CORE_LEASE_SECONDS)) {
        Json(serde_json::json!({
            "schema":"coronatio.core.events.renewal.v1",
            "streamId":query.stream_id,
            "status":"renewed",
            "leaseSeconds":CORE_LEASE_SECONDS
        }))
        .into_response()
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "schema":"coronatio.core.events.renewal.v1",
                "streamId":query.stream_id,
                "status":"unknown-stream"
            })),
        )
            .into_response()
    }
}

pub(crate) async fn core_pulse_upgrade_route(
    headers: axum::http::HeaderMap,
    Query(query): Query<CoreRenewQuery>,
) -> Response {
    let Some(document) = crate::caduceus_access::document_incarnation_from_headers(&headers)
    else {
        return core_membership_response(StatusCode::BAD_REQUEST, &query.stream_id, "document-required");
    };
    if session_projection_from_headers(&headers).await != Session::Admin {
        downgrade_core_stream(&query.stream_id, None);
        return core_membership_response(
            StatusCode::UNAUTHORIZED,
            &query.stream_id,
            "attendance-refused",
        );
    }
    let Some(parent) = crate::caduceus_access::attendance_from_headers(&headers) else {
        downgrade_core_stream(&query.stream_id, Some(&document));
        return core_membership_response(
            StatusCode::UNAUTHORIZED,
            &query.stream_id,
            "attendance-refused",
        );
    };
    match upgrade_core_stream_attended(&query.stream_id, document, parent).await {
        Ok(()) => core_membership_response(StatusCode::OK, &query.stream_id, "upgraded"),
        Err(signal) if signal == "unknown-stream" => {
            core_membership_response(StatusCode::NOT_FOUND, &query.stream_id, "unknown-stream")
        }
        Err(signal) => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({
                "schema":"coronatio.core.events.membership.v1",
                "streamId":query.stream_id,
                "status":"attendance-refused",
                "firstMissingSignal":signal
            })),
        )
            .into_response(),
    }
}

pub(crate) async fn core_pulse_downgrade_route(
    headers: axum::http::HeaderMap,
    Query(query): Query<CoreRenewQuery>,
) -> Response {
    let Some(document) = crate::caduceus_access::document_incarnation_from_headers(&headers)
    else {
        return core_membership_response(StatusCode::BAD_REQUEST, &query.stream_id, "document-required");
    };
    if let Some(attendance) = crate::caduceus_access::attendance_from_headers(&headers) {
        crate::caduceus_access::bust_attendance_projection(
            &attendance,
            &document,
            "stream-downgraded",
        );
    }
    if downgrade_core_stream(&query.stream_id, Some(&document)) {
        core_membership_response(StatusCode::OK, &query.stream_id, "downgraded")
    } else {
        core_membership_response(StatusCode::NOT_FOUND, &query.stream_id, "unknown-stream")
    }
}

fn core_membership_response(
    status: StatusCode,
    stream_id: &str,
    membership_status: &str,
) -> Response {
    (
        status,
        Json(serde_json::json!({
            "schema":"coronatio.core.events.membership.v1",
            "streamId":stream_id,
            "status":membership_status
        })),
    )
        .into_response()
}

pub(crate) fn subscribe_core_stream(
    session: Session,
    lease: Duration,
) -> (String, stream::BoxStream<'static, (String, String, String)>) {
    let stream_id = format!("core-{}", uuid::Uuid::new_v4());
    let (control_tx, control_rx) = tokio::sync::mpsc::unbounded_channel();
    core_memberships().lock().unwrap().insert(
        stream_id.clone(),
        CoreMembership {
            deadline: Instant::now() + lease,
            session,
            document: None,
            parent_attendance: None,
            admin_stats_cache: None,
            fallback_signal: None,
            generation: 0,
            upgrading: false,
            control_tx,
        },
    );
    let state = CoreStreamState {
        stream_id: stream_id.clone(),
        session,
        generation: 0,
        control_rx,
        index: 0,
        opened: false,
        last_collected: HashMap::new(),
        last_payload: HashMap::new(),
        pulse_batch: None,
    };
    let frames = stream::unfold(Some(state), move |state| async move {
        let mut state = state?;
        while let Ok(CoreControl::Set(session)) = state.control_rx.try_recv() {
            if state.session != session {
                state.session = session;
                state.last_collected.clear();
                state.last_payload.clear();
            }
        }
        let Some(live) = core_live_projection(&state.stream_id) else {
            let id = state.stream_id.clone();
            remove_core_membership(&id, "lease-expired");
            return Some((
                (
                    "core.expired".into(),
                    id.clone(),
                    serde_json::json!({"streamId":id,"status":"expired"}).to_string(),
                ),
                None,
            ));
        };
        reconcile_core_stream_state(&mut state, &live);
        if !state.opened {
            state.opened = true;
            let id = state.stream_id.clone();
            let data = serde_json::json!({
                "schema":"coronatio.core.events.v1",
                "streamId":id,
                "leaseSeconds":CORE_LEASE_SECONDS,
                "renewRoute":format!("/api/core/pulse/renew?streamId={}", state.stream_id),
                "topics":catalog().iter().map(|entry| entry.topic_id).collect::<Vec<_>>()
            })
            .to_string();
            return Some((("core.open".into(), id, data), Some(state)));
        }
        if state.index >= catalog().len() {
            tokio::time::sleep(Duration::from_secs(1)).await;
            state.index = 0;
            state.pulse_batch = None;
            while let Ok(CoreControl::Set(session)) = state.control_rx.try_recv() {
                if state.session != session {
                    state.session = session;
                    state.last_collected.clear();
                    state.last_payload.clear();
                }
            }
        }
        let Some(live) = core_live_projection(&state.stream_id) else {
            let id = state.stream_id.clone();
            remove_core_membership(&id, "lease-expired");
            return Some((
                (
                    "core.expired".into(),
                    id.clone(),
                    serde_json::json!({"streamId":id,"status":"expired"}).to_string(),
                ),
                None,
            ));
        };
        reconcile_core_stream_state(&mut state, &live);
        let entry = catalog()[state.index];
        state.index += 1;
        let now = Instant::now();
        let refresh_due = state
            .last_collected
            .get(entry.topic_id)
            .map(|last| now.duration_since(*last) >= minimum_refresh_interval(entry.topic_id))
            .unwrap_or(true);
        let mut payload = if refresh_due {
            tracing::debug!(topic_id = entry.topic_id, "collecting core indicator topic");
            let topic_id = entry.topic_id;
            let stats_batch = if topic_id == "source.currency" {
                None
            } else {
                if state
                    .pulse_batch
                    .as_ref()
                    .is_none_or(|batch| batch.generation != live.generation)
                {
                    let batch_live = live.clone();
                    let batch = tokio::task::spawn_blocking(move || read_core_pulse_batch(&batch_live))
                        .await
                        .unwrap_or_else(|_| failed_core_pulse_batch(
                            live.generation,
                            "caduceus-stats-collector-task-failed",
                        ));
                    state.pulse_batch = Some(batch);
                }
                state.pulse_batch.clone()
            };
            let topic_session = live.session;
            let collected = if let Some(batch) = stats_batch.clone() {
                let session = topic_session;
                tokio::task::spawn_blocking(move || {
                    Ok(project_core_stats_topic(topic_id, session, batch))
                })
                .await
                .unwrap_or_else(|_| Err("caduceus-stats-collector-task-failed".to_owned()))
            } else {
                tokio::task::spawn_blocking(move || collect_source_currency_core_topic(topic_session))
                    .await
                    .unwrap_or_else(|_| Err("caduceus-source-currency-task-failed".to_owned()))
            };
            let still_current = core_live_projection(&state.stream_id)
                .is_some_and(|current| current.generation == live.generation);
            let mut collected = collected;
            if !still_current {
                state.last_collected.clear();
                state.last_payload.clear();
                state.pulse_batch = None;
                let guest = if topic_id == "source.currency" {
                    tokio::task::spawn_blocking(move || collect_source_currency_core_topic(Session::Guest))
                        .await
                        .unwrap_or_else(|_| Err("caduceus-source-currency-task-failed".to_owned()))
                } else if let Some(batch) = stats_batch {
                    tokio::task::spawn_blocking(move || {
                        let signal = Some("caduceus-attendance-session-transitioned".to_owned());
                        Ok(project_core_stats_topic(
                            topic_id,
                            Session::Guest,
                            CorePulseBatch {
                                current: batch.current,
                                admin: None,
                                admin_signal: None,
                                fallback_signal: signal,
                                ..batch
                            },
                        ))
                    })
                    .await
                    .unwrap_or_else(|_| Err("caduceus-stats-collector-task-failed".to_owned()))
                } else {
                    Err("caduceus-attendance-session-transitioned".to_owned())
                };
                collected = guest;
                if let Some(current) = core_live_projection(&state.stream_id) {
                    reconcile_core_stream_state(&mut state, &current);
                } else {
                    remove_core_membership(&state.stream_id, "lease-expired");
                    return None;
                }
            }
            let payload = match collected {
                Ok(sample) => {
                    let unavailable = sample.unavailable_signal;
                    let mut envelope = serde_json::json!({
                        "schema":"coronatio.core.topic.v1",
                        "topicId":entry.topic_id,
                        "status":if unavailable.is_some() { "unavailable" } else { "snapshot" },
                        "snapshot":sample.snapshot
                    });
                    if let Some(signal) = unavailable {
                        envelope["firstMissingSignal"] = serde_json::Value::String(signal);
                    }
                    if let Some(indicator_state) = sample.indicator_state {
                        envelope["indicatorState"] = serde_json::Value::String(
                            if envelope["status"] == "unavailable" {
                                "unavailable".to_string()
                            } else {
                                indicator_state.to_string()
                            },
                        );
                    }
                    envelope.to_string()
                }
                Err(signal) => {
                    let mut envelope = serde_json::json!({
                        "schema":"coronatio.core.topic.v1",
                        "topicId":entry.topic_id,
                        "status":"unavailable",
                        "firstMissingSignal":signal
                    });
                    if entry.topic_id == "vpn.status" {
                        envelope["indicatorState"] = serde_json::Value::String("unavailable".to_owned());
                    }
                    envelope.to_string()
                }
            };
            state.last_collected.insert(entry.topic_id, now);
            state.last_payload.insert(entry.topic_id, payload.clone());
            payload
        } else {
            state
                .last_payload
                .get(entry.topic_id)
                .expect("cached core topic payload")
                .clone()
        };
        let Some(current) = core_live_projection(&state.stream_id) else {
            state.last_collected.clear();
            state.last_payload.clear();
            state.pulse_batch = None;
            remove_core_membership_if_generation(&state.stream_id, state.generation, "lease-expired");
            return None;
        };
        if current.generation != state.generation {
            state.last_collected.clear();
            state.last_payload.clear();
            state.pulse_batch = None;
            reconcile_core_stream_state(&mut state, &current);
            let sample = unavailable_topic_sample(
                entry.topic_id,
                "caduceus-attendance-session-transitioned".to_owned(),
            );
            let mut envelope = serde_json::json!({
                "schema":"coronatio.core.topic.v1",
                "topicId":entry.topic_id,
                "status":"unavailable",
                "firstMissingSignal":"caduceus-attendance-session-transitioned",
                "snapshot":sample.snapshot
            });
            if let Some(indicator_state) = sample.indicator_state {
                envelope["indicatorState"] = serde_json::Value::String(indicator_state.to_owned());
            }
            payload = envelope.to_string();
        }
        Some((
            (
                entry.topic_id.into(),
                format!("{}:{}", state.stream_id, state.index),
                payload,
            ),
            Some(state),
        ))
    })
    .boxed();
    (stream_id, frames)
}

fn reconcile_core_stream_state(state: &mut CoreStreamState, live: &CoreLiveProjection) {
    if state.session != live.session || state.generation != live.generation {
        state.session = live.session;
        state.generation = live.generation;
        state.last_collected.clear();
        state.last_payload.clear();
        state.pulse_batch = None;
    }
}

fn minimum_refresh_interval(topic_id: &str) -> Duration {
    match topic_id {
        "source.currency" => Duration::from_secs(60),
        "tailscale.status" | "vpn.status" | "services.status" => Duration::from_secs(5),
        "internet.status" => Duration::from_secs(7),
        _ => Duration::from_secs(1),
    }
}

pub(crate) fn renew_core_stream(stream_id: &str, lease: Duration) -> bool {
    let now = Instant::now();
    {
        let mut memberships = core_memberships().lock().unwrap();
        if memberships
            .get(stream_id)
            .is_some_and(|membership| membership.deadline > now)
        {
            if let Some(membership) = memberships.get_mut(stream_id) {
                membership.deadline = now + lease;
                return true;
            }
        }
        let _ = memberships.remove(stream_id);
    }
    false
}

async fn upgrade_core_stream_attended(
    stream_id: &str,
    document: String,
    parent: crate::caduceus_access::AttendanceProof,
) -> Result<(), String> {
    let generation = match begin_core_upgrade(stream_id, &document, &parent)? {
        Some(generation) => generation,
        None => return Ok(()),
    };
    let call = crate::caduceus_access::CaduceusAccessClient::default()
        .attendance_validate_async(parent.clone(), document.clone())
        .await;
    if !call.receipt.ok {
        let signal = call.receipt.code;
        fail_core_upgrade(stream_id, generation, signal.clone());
        return Err(signal);
    }
    if finish_core_upgrade(stream_id, generation, document, parent) {
        Ok(())
    } else {
        Err("caduceus-attendance-session-transitioned".to_owned())
    }
}

fn begin_core_upgrade(
    stream_id: &str,
    document: &str,
    parent: &crate::caduceus_access::AttendanceProof,
) -> Result<Option<u64>, String> {
    let mut memberships = core_memberships().lock().unwrap();
    let Some(membership) = memberships.get_mut(stream_id) else {
        return Err("unknown-stream".to_owned());
    };
    if membership.deadline <= Instant::now() {
        return Err("unknown-stream".to_owned());
    }
    if membership.upgrading {
        return Err("caduceus-attendance-upgrade-in-flight".to_owned());
    }
    if membership.session == Session::Admin
        && membership.document.as_deref() == Some(document)
        && membership.parent_attendance.as_ref() == Some(parent)
        && membership.admin_stats_cache.is_some()
    {
        return Ok(None);
    }
    membership.admin_stats_cache = None;
    membership.document = Some(document.to_owned());
    membership.parent_attendance = Some(parent.clone());
    membership.fallback_signal = None;
    if membership.session != Session::Guest {
        membership.session = Session::Guest;
        let _ = membership.control_tx.send(CoreControl::Set(Session::Guest));
    }
    membership.upgrading = true;
    membership.generation = membership.generation.wrapping_add(1);
    Ok(Some(membership.generation))
}

fn finish_core_upgrade(
    stream_id: &str,
    generation: u64,
    document: String,
    parent: crate::caduceus_access::AttendanceProof,
) -> bool {
    let mut memberships = core_memberships().lock().unwrap();
    let Some(membership) = memberships.get_mut(stream_id) else {
        return false;
    };
    if membership.generation != generation
        || !membership.upgrading
        || membership.deadline <= Instant::now()
    {
        return false;
    }
    membership.session = Session::Admin;
    membership.document = Some(document);
    membership.parent_attendance = Some(parent);
    membership.admin_stats_cache = Some(std::sync::Arc::new(ScopedAdminStatsCache::default()));
    membership.fallback_signal = None;
    membership.upgrading = false;
    let _ = membership.control_tx.send(CoreControl::Set(Session::Admin));
    true
}

fn fail_core_upgrade(stream_id: &str, generation: u64, signal: String) {
    if let Some(membership) = core_memberships().lock().unwrap().get_mut(stream_id) {
        if membership.generation == generation {
            membership.session = Session::Guest;
            membership.document = None;
            membership.parent_attendance = None;
            membership.admin_stats_cache = None;
            membership.fallback_signal = Some(signal);
            membership.upgrading = false;
            let _ = membership.control_tx.send(CoreControl::Set(Session::Guest));
        }
    }
}

pub(crate) fn downgrade_core_stream(stream_id: &str, document: Option<&str>) -> bool {
    let mut memberships = core_memberships().lock().unwrap();
    let Some(membership) = memberships.get_mut(stream_id) else {
        return false;
    };
    if document.is_some() && membership.document.as_deref() != document {
        return false;
    }
    membership.session = Session::Guest;
    membership.document = None;
    membership.parent_attendance = None;
    membership.admin_stats_cache = None;
    membership.fallback_signal = None;
    membership.upgrading = false;
    membership.generation = membership.generation.wrapping_add(1);
    let _ = membership.control_tx.send(CoreControl::Set(Session::Guest));
    true
}

pub(crate) fn downgrade_core_document(document: &str) {
    let mut memberships = core_memberships().lock().unwrap();
    for membership in memberships
        .values_mut()
        .filter(|membership| membership.document.as_deref() == Some(document))
    {
        membership.session = Session::Guest;
        membership.document = None;
        membership.parent_attendance = None;
        membership.admin_stats_cache = None;
        membership.fallback_signal = None;
        membership.upgrading = false;
        membership.generation = membership.generation.wrapping_add(1);
        let _ = membership.control_tx.send(CoreControl::Set(Session::Guest));
    }
}

fn core_live_projection(stream_id: &str) -> Option<CoreLiveProjection> {
    let memberships = core_memberships().lock().unwrap();
    let membership = memberships.get(stream_id)?;
    if membership.deadline <= Instant::now() {
        return None;
    }
    let admin = match (
        membership.session,
        membership.document.as_ref(),
        membership.parent_attendance.as_ref(),
        membership.admin_stats_cache.as_ref(),
    ) {
        (Session::Admin, Some(document), Some(attendance), Some(stats_cache)) => Some(CoreAdminContext {
            attendance: attendance.clone(),
            document: document.clone(),
            stats_cache: stats_cache.clone(),
            generation: membership.generation,
        }),
        _ => None,
    };
    Some(CoreLiveProjection {
        session: if admin.is_some() { Session::Admin } else { Session::Guest },
        generation: membership.generation,
        admin,
        fallback_signal: membership.fallback_signal.clone(),
    })
}

fn remove_core_membership(stream_id: &str, _reason: &str) {
    let _ = core_memberships().lock().unwrap().remove(stream_id);
}

fn remove_core_membership_if_generation(stream_id: &str, generation: u64, _reason: &str) {
    let mut memberships = core_memberships().lock().unwrap();
    if memberships
        .get(stream_id)
        .is_some_and(|membership| membership.generation == generation)
    {
        let _ = memberships.remove(stream_id);
    }
}

async fn validate_core_host_membership(
    stream_id: &str,
    headers: &axum::http::HeaderMap,
) -> bool {
    let is_host = core_memberships()
        .lock()
        .unwrap()
        .get(stream_id)
        .is_some_and(|membership| membership.session == Session::Admin);
    if is_host && session_projection_from_headers(headers).await != Session::Admin {
        downgrade_core_stream(stream_id, None);
        false
    } else {
        true
    }
}

#[derive(Clone)]
pub(crate) struct IndicatorTopicSample {
    pub(crate) snapshot: serde_json::Value,
    pub(crate) indicator_state: Option<&'static str>,
    pub(crate) unavailable_signal: Option<String>,
}

pub(crate) fn collect_indicator_topic(
    topic_id: &str,
    session: Session,
) -> Result<serde_json::Value, String> {
    if topic_id == "source.currency" {
        return collect_source_currency_indicator(session);
    }
    let admin_signal = (session == Session::Admin)
        .then(|| "caduceus-attendance-admin-scope-required".to_owned());
    let sample = collect_stats_indicator_topic(topic_id, session, None, admin_signal);
    match sample.unavailable_signal {
        Some(signal) => Err(signal),
        None => Ok(sample.snapshot),
    }
}

fn collect_source_currency_core_topic(session: Session) -> Result<IndicatorTopicSample, String> {
    collect_source_currency_indicator(session).map(|snapshot| IndicatorTopicSample {
        snapshot,
        indicator_state: None,
        unavailable_signal: None,
    })
}

fn read_core_pulse_batch(live: &CoreLiveProjection) -> CorePulseBatch {
    let current = caduceus_stats_sample();
    let (admin, admin_signal) = if live.session == Session::Admin {
        match live.admin.as_ref() {
            Some(context) if context.generation == live.generation => {
                let readback = scoped_caduceus_admin_stats_sample(&context.stats_cache, || {
                    caduceus_http_with_attendance_and_document(
                        "GET",
                        CADUCEUS_STATS_ADMIN_PATH,
                        Some(&context.attendance),
                        Some(&context.document),
                    )
                });
                let signal = (!readback.ok).then(|| readback.first_missing_signal.clone());
                (Some(readback), signal)
            }
            _ => (
                None,
                Some("caduceus-attendance-admin-scope-required".to_owned()),
            ),
        }
    } else {
        (None, None)
    };
    CorePulseBatch {
        generation: live.generation,
        current,
        admin,
        admin_signal,
        fallback_signal: live.fallback_signal.clone(),
    }
}

fn failed_core_pulse_batch(generation: u64, signal: &str) -> CorePulseBatch {
    CorePulseBatch {
        generation,
        current: failed_stats_readback(signal).into(),
        admin: None,
        admin_signal: Some(signal.to_owned()),
        fallback_signal: Some(signal.to_owned()),
    }
}

fn project_core_stats_topic(
    topic_id: &str,
    session: Session,
    batch: CorePulseBatch,
) -> IndicatorTopicSample {
    collect_stats_indicator_topic_from_readbacks(
        topic_id,
        session,
        batch.current,
        batch.admin,
        batch.admin_signal,
        batch.fallback_signal,
    )
}

pub(crate) async fn collect_indicator_topic_for_request(
    topic_id: &str,
    headers: &axum::http::HeaderMap,
) -> (Session, IndicatorTopicSample) {
    let session = session_projection_from_headers(headers).await;
    if topic_id == "source.currency" {
        let topic = topic_id.to_owned();
        let error_topic = topic.clone();
        let sample = tokio::task::spawn_blocking(move || {
            collect_source_currency_core_topic(session)
        })
        .await
        .unwrap_or_else(|_| Err("caduceus-source-currency-task-failed".to_owned()))
        .unwrap_or_else(|signal| unavailable_topic_sample(&error_topic, signal));
        return (session, sample);
    }
    if session != Session::Admin {
        let topic = topic_id.to_owned();
        let error_topic = topic.clone();
        let sample = tokio::task::spawn_blocking(move || {
            let current = caduceus_stats_sample();
            Ok(collect_stats_indicator_topic_from_readbacks(
                &topic,
                Session::Guest,
                current,
                None,
                None,
                None,
            ))
        })
        .await
        .unwrap_or_else(|_| Err("caduceus-stats-collector-task-failed".to_owned()))
        .unwrap_or_else(|signal| unavailable_topic_sample(&error_topic, signal));
        return (Session::Guest, sample);
    }

    let Some(parent_document) = crate::caduceus_access::document_incarnation_from_headers(headers)
    else {
        return (
            Session::Guest,
            unavailable_topic_sample(topic_id, "caduceus-attendance-document-required".to_owned()),
        );
    };
    let Some(parent) = crate::caduceus_access::attendance_from_headers(headers) else {
        return (
            Session::Guest,
            unavailable_topic_sample(topic_id, "caduceus-attendance-required".to_owned()),
        );
    };
    let readback = tokio::task::spawn_blocking(move || {
        read_scoped_admin_stats_once(&parent_document, parent)
    })
    .await
    .unwrap_or_else(|_| failed_stats_readback("caduceus-attendance-task-failed"));
    let topic = topic_id.to_owned();
    let error_topic = topic.clone();
    let sample = tokio::task::spawn_blocking(move || {
        let current = caduceus_stats_sample();
        let admin_signal = (!readback.ok).then(|| readback.first_missing_signal.clone());
        Ok(collect_stats_indicator_topic_from_readbacks(
            &topic,
            Session::Admin,
            current,
            Some(readback.into()),
            admin_signal,
            None,
        ))
    })
    .await
    .unwrap_or_else(|_| Err("caduceus-stats-collector-task-failed".to_owned()))
    .unwrap_or_else(|signal| unavailable_topic_sample(&error_topic, signal));
    (Session::Admin, sample)
}

fn read_scoped_admin_stats_once(
    parent_document: &str,
    parent: crate::caduceus_access::AttendanceProof,
) -> CaduceusHttpReadback {
    let client = crate::caduceus_access::CaduceusAccessClient::default();
    let call = client.attendance_validate(&parent, parent_document);
    if !call.receipt.ok {
        return failed_stats_readback(&call.receipt.code);
    }
    caduceus_http_with_attendance_and_document(
        "GET",
        CADUCEUS_STATS_ADMIN_PATH,
        Some(&parent),
        Some(parent_document),
    )
}

fn failed_stats_readback(signal: &str) -> CaduceusHttpReadback {
    CaduceusHttpReadback {
        ok: false,
        status: StatusCode::SERVICE_UNAVAILABLE.as_u16(),
        path: CADUCEUS_STATS_ADMIN_PATH.to_owned(),
        body: serde_json::json!({"ok":false,"firstMissingSignal":signal}),
        first_missing_signal: signal.to_owned(),
    }
}

fn collect_stats_indicator_topic(
    topic_id: &str,
    session: Session,
    admin_readback: Option<CaduceusStatsReadback>,
    admin_signal: Option<String>,
) -> IndicatorTopicSample {
    collect_stats_indicator_topic_from_readbacks(
        topic_id,
        session,
        caduceus_stats_sample(),
        admin_readback,
        admin_signal,
        None,
    )
}

fn collect_stats_indicator_topic_from_readbacks(
    topic_id: &str,
    session: Session,
    current: CaduceusStatsReadback,
    admin_readback: Option<CaduceusStatsReadback>,
    admin_signal: Option<String>,
    fallback_signal: Option<String>,
) -> IndicatorTopicSample {
    let Some(block_name) = topic_block_name(topic_id) else {
        return unavailable_topic_sample(topic_id, format!("unknown topic: {topic_id}"));
    };
    if !current.ok {
        let mut sample = unavailable_topic_sample(topic_id, current.first_missing_signal);
        if let Some(signal) = admin_signal.or(fallback_signal) {
            sample.unavailable_signal = Some(signal);
        }
        return sample;
    }
    let Some(block) = current.body.get(block_name).and_then(serde_json::Value::as_object)
    else {
        let signal = stats_signal(current.body.get(block_name), "caduceus-stats-block-missing");
        return unavailable_topic_sample(topic_id, signal);
    };
    if let Some(signal) = stale_block_signal(block, stats_block_stale_after(block_name), block_name) {
        return unavailable_topic_sample(topic_id, signal);
    }
    let mut sample = match topic_id {
        "power.status" => project_power_guest(block),
        "tailscale.status" => project_tailscale_guest(block),
        "internet.status" => project_internet_guest(block),
        "services.status" => project_services(block, true, None),
        "vpn.status" => project_vpn_guest(block),
        _ => unavailable_topic_sample(topic_id, format!("unknown topic: {topic_id}")),
    };
    if session != Session::Admin {
        if let Some(signal) = fallback_signal {
            sample = mark_admin_degraded(sample, signal);
        }
        return sample;
    }
    if topic_id == "power.status" {
        // Power carries no admin fields; it never waits on the admin read.
        return sample;
    }

    let Some(admin_readback) = admin_readback.filter(|readback| readback.ok) else {
        let signal = admin_signal
            .or(fallback_signal)
            .unwrap_or_else(|| "caduceus-attendance-admin-read-unavailable".to_owned());
        return mark_admin_degraded(sample, signal);
    };
    let Some(admin_block) = admin_readback
        .body
        .get(block_name)
        .and_then(serde_json::Value::as_object)
    else {
        let signal = admin_signal.unwrap_or_else(|| {
            stats_signal(
                admin_readback.body.get(block_name),
                "caduceus-stats-admin-block-missing",
            )
        });
        return mark_admin_degraded(sample, signal);
    };
    if let Some(signal) = stale_block_signal(
        admin_block,
        stats_admin_block_stale_after(block_name),
        block_name,
    ) {
        return mark_admin_degraded(sample, signal);
    }
    if topic_id == "services.status" {
        sample = project_services(block, false, admin_block.get("units"));
    }
    merge_admin_indicator_fields(topic_id, &mut sample.snapshot, admin_block);
    sample
}

/// A refused or missing admin read keeps the guest snapshot live and names why admin fields are absent.
fn mark_admin_degraded(mut sample: IndicatorTopicSample, signal: String) -> IndicatorTopicSample {
    if let Some(snapshot) = sample.snapshot.as_object_mut() {
        snapshot.insert("adminFirstMissingSignal".to_owned(), serde_json::Value::String(signal));
    }
    sample
}

fn mark_sample_unavailable(
    mut sample: IndicatorTopicSample,
    signal: String,
) -> IndicatorTopicSample {
    if let Some(snapshot) = sample.snapshot.as_object_mut() {
        let existing = snapshot
            .get("firstMissingSignal")
            .and_then(serde_json::Value::as_str);
        if existing.is_none_or(|value| value.is_empty() || value == "none") {
            snapshot.insert(
                "firstMissingSignal".to_owned(),
                serde_json::Value::String(signal.clone()),
            );
        }
    }
    sample.unavailable_signal = Some(signal);
    if sample.indicator_state.is_some() {
        sample.indicator_state = Some("unavailable");
    }
    sample
}

fn topic_block_name(topic_id: &str) -> Option<&'static str> {
    match topic_id {
        "power.status" => Some("power"),
        "tailscale.status" => Some("tailscale"),
        "internet.status" => Some("internet"),
        "services.status" => Some("services"),
        "vpn.status" => Some("vpn"),
        _ => None,
    }
}

fn stats_block_stale_after(block: &str) -> Duration {
    match block {
        "tailscale" | "services" | "vpn" => Duration::from_secs(15),
        "internet" => Duration::from_secs(21),
        "power" => Duration::from_secs(10),
        _ => Duration::from_secs(15),
    }
}

fn stats_admin_block_stale_after(block: &str) -> Duration {
    match block {
        "internet" => Duration::from_secs(24 * 60 * 60 + 300),
        "power" => Duration::from_secs(10),
        _ => Duration::from_secs(15),
    }
}

fn stats_signal(block: Option<&serde_json::Value>, fallback: &str) -> String {
    block
        .and_then(|value| value.get("firstMissingSignal"))
        .and_then(serde_json::Value::as_str)
        .filter(|signal| !signal.is_empty() && *signal != "none")
        .unwrap_or(fallback)
        .to_owned()
}

fn stale_block_signal(
    block: &serde_json::Map<String, serde_json::Value>,
    stale_after: Duration,
    name: &str,
) -> Option<String> {
    let ts = block
        .get("ts")
        .and_then(|value| value.as_i64().or_else(|| value.as_u64().and_then(|v| i64::try_from(v).ok())));
    let Some(ts) = ts else {
        return Some(stats_signal(
            Some(&serde_json::Value::Object(block.clone())),
            "caduceus-stats-block-timestamp-missing",
        ));
    };
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;
    let block_value = serde_json::Value::Object(block.clone());
    if ts <= 0 {
        Some(stats_signal(
            Some(&block_value),
            "caduceus-stats-block-timestamp-missing",
        ))
    } else if ts > now.saturating_add(1) {
        Some(stats_signal(
            Some(&block_value),
            "caduceus-stats-block-timestamp-invalid",
        ))
    } else if now.saturating_sub(ts) > stale_after.as_secs() as i64 {
        Some(stats_signal(
            Some(&block_value),
            &format!("caduceus-stats-{name}-stale"),
        ))
    } else {
        None
    }
}

fn unavailable_topic_sample(topic_id: &str, signal: String) -> IndicatorTopicSample {
    let snapshot = match topic_id {
        "power.status" => {
            let (calibration, _) = caduceus_power_calibration();
            serde_json::json!({
                "ok":false,"success":false,"status":"unavailable",
                "current":null,"historical":[],"averages":{"5s":null,"30s":null,"60s":null},
                "calibration":calibration,"calibrationMinimum":0.1,"calibrationMaximum":10.0,
                "unit":"W","timestamp":null,"watts":null,"sources":[],"ts":null,
                "firstMissingSignal":signal
            })
        }
        "tailscale.status" => serde_json::json!({
            "ok":false,"success":false,"status":"unavailable","interface":null,
            "timestamp":null,"ts":null,"firstMissingSignal":signal
        }),
        "internet.status" => serde_json::json!({
            "ok":false,"success":false,"status":"unavailable",
            "timestamp":null,"ts":null,"firstMissingSignal":signal
        }),
        "services.status" => serde_json::json!({
            "ok":false,"success":false,"status":"unavailable","services":[],"rows":[],
            "counts":{"up":0,"total":0},"timestamp":null,"ts":null,
            "firstMissingSignal":signal
        }),
        "vpn.status" => serde_json::json!({
            "ok":false,"success":false,"status":"unavailable",
            "vpnStatus":null,"transmissionStatus":null,"timestamp":null,"ts":null,
            "firstMissingSignal":signal
        }),
        _ => serde_json::json!({"firstMissingSignal":signal}),
    };
    IndicatorTopicSample {
        snapshot,
        indicator_state: (topic_id == "vpn.status").then_some("unavailable"),
        unavailable_signal: Some(signal),
    }
}

fn finite_nonnegative(value: &serde_json::Value) -> Option<f64> {
    value
        .as_f64()
        .filter(|watts| watts.is_finite() && *watts >= 0.0)
}

fn null_power_averages() -> serde_json::Value {
    serde_json::json!({"5s":null,"30s":null,"60s":null})
}

fn stats_timestamp_seconds(value: &serde_json::Value) -> Option<i64> {
    value.as_i64().or_else(|| value.as_u64().and_then(|seconds| i64::try_from(seconds).ok()))
}

fn average_power_window(
    samples: &std::collections::BTreeMap<i64, f64>,
    current_ts: i64,
    window_seconds: i64,
) -> serde_json::Value {
    let start = current_ts.saturating_sub(window_seconds);
    let recent = samples.range(start..=current_ts).map(|(_, watts)| *watts).collect::<Vec<_>>();
    if recent.is_empty() {
        serde_json::Value::Null
    } else {
        serde_json::json!(recent.iter().sum::<f64>() / recent.len() as f64)
    }
}

fn power_history_projection(
    current: Option<f64>,
    current_ts: Option<i64>,
) -> (Vec<f64>, serde_json::Value, Option<String>) {
    let (Some(current), Some(current_ts)) = (current, current_ts) else {
        return (Vec::new(), null_power_averages(), Some("caduceus-stats-power-timestamp-unavailable".to_owned()));
    };
    let history = caduceus_stats_history();
    let mut samples = std::collections::BTreeMap::new();
    if !history.ok {
        samples.insert(current_ts, current);
        return (
            samples.values().copied().collect(),
            null_power_averages(),
            Some(history.first_missing_signal),
        );
    }
    let Some(raw) = history.body.pointer("/tiers/raw").and_then(serde_json::Value::as_array) else {
        samples.insert(current_ts, current);
        return (
            samples.values().copied().collect(),
            null_power_averages(),
            Some("caduceus-stats-history-raw-unavailable".to_owned()),
        );
    };
    let mut history_signal = None;
    for sample in raw {
        let Some(ts) = sample.get("ts").and_then(stats_timestamp_seconds) else {
            history_signal.get_or_insert_with(|| "caduceus-stats-history-timestamp-missing".to_owned());
            continue;
        };
        if ts > current_ts {
            continue;
        }
        let Some(watts) = sample.pointer("/power/watts").and_then(finite_nonnegative) else {
            history_signal.get_or_insert_with(|| "caduceus-stats-history-power-watts-unavailable".to_owned());
            continue;
        };
        samples.insert(ts, watts);
    }
    samples.insert(current_ts, current);
    let historical = samples.values().copied().collect::<Vec<_>>();
    // Rows without a timestamp or watts are skipped above; the averages use every valid row.
    let averages = serde_json::json!({
        "5s":average_power_window(&samples, current_ts, 5),
        "30s":average_power_window(&samples, current_ts, 30),
        "60s":average_power_window(&samples, current_ts, 60)
    });
    (historical, averages, history_signal)
}

fn project_power_guest(block: &serde_json::Map<String, serde_json::Value>) -> IndicatorTopicSample {
    let current = block.get("watts").and_then(finite_nonnegative);
    let signal = stats_signal_from_object(block, "caduceus-stats-power-watts-unavailable");
    let timestamp = block.get("ts").cloned().unwrap_or(serde_json::Value::Null);
    let current_ts = block.get("ts").and_then(stats_timestamp_seconds);
    let sources = block
        .get("sources")
        .cloned()
        .unwrap_or_else(|| serde_json::json!([]));
    let (historical, averages, history_signal) = if current.is_some() {
        power_history_projection(current, current_ts)
    } else {
        (Vec::new(), null_power_averages(), None)
    };
    let mut first_missing = block
        .get("firstMissingSignal")
        .cloned()
        .unwrap_or(serde_json::Value::Null);
    if first_missing
        .as_str()
        .is_none_or(|value| value.is_empty() || value == "none")
    {
        if let Some(signal) = history_signal.as_ref() {
            first_missing = serde_json::Value::String(signal.clone());
        }
    }
    let (calibration, calibration_signal) = caduceus_power_calibration();
    if first_missing
        .as_str()
        .is_none_or(|value| value.is_empty() || value == "none")
    {
        if let Some(signal) = calibration_signal {
            first_missing = serde_json::Value::String(signal);
        }
    }
    IndicatorTopicSample {
        snapshot: serde_json::json!({
            "ok":current.is_some(),"success":current.is_some(),
            "status":current.map(|watts| format!("{watts:.2}")).unwrap_or_else(|| "unavailable".to_owned()),
            "current":current,"historical":historical,"averages":averages,
            "calibration":calibration,"calibrationMinimum":0.1,"calibrationMaximum":10.0,
            "unit":"W","timestamp":timestamp,"watts":current,"sources":sources,"ts":timestamp,
            "firstMissingSignal":first_missing,
            "historyFirstMissingSignal":history_signal.map(serde_json::Value::String).unwrap_or(serde_json::Value::Null)
        }),
        indicator_state: None,
        unavailable_signal: current.is_none().then_some(signal),
    }
}

fn project_tailscale_guest(block: &serde_json::Map<String, serde_json::Value>) -> IndicatorTopicSample {
    let raw_status = block.get("status").and_then(serde_json::Value::as_str);
    let interface = block.get("interface").and_then(serde_json::Value::as_bool);
    let valid = raw_status.is_some_and(|value| matches!(value, "connected" | "disconnected" | "needs-login" | "absent"))
        && interface.is_some();
    let signal = stats_signal_from_object(block, "caduceus-stats-tailscale-state-unavailable");
    let timestamp = block.get("ts").cloned().unwrap_or(serde_json::Value::Null);
    IndicatorTopicSample {
        snapshot: serde_json::json!({
            "ok":valid,"success":valid,
            "status":if valid { raw_status.unwrap_or("unavailable") } else { "unavailable" },
            "interface":if valid { interface.map(serde_json::Value::Bool).unwrap_or(serde_json::Value::Null) } else { serde_json::Value::Null },
            "timestamp":timestamp,"ts":timestamp,
            "firstMissingSignal":block.get("firstMissingSignal").cloned().unwrap_or(serde_json::Value::Null)
        }),
        indicator_state: None,
        unavailable_signal: (!valid).then_some(signal),
    }
}

fn project_internet_guest(block: &serde_json::Map<String, serde_json::Value>) -> IndicatorTopicSample {
    let raw_status = block.get("status").and_then(serde_json::Value::as_str);
    let valid = raw_status.is_some_and(|value| matches!(value, "connected" | "disconnected"));
    let signal = stats_signal_from_object(block, "caduceus-stats-internet-state-unavailable");
    let timestamp = block.get("ts").cloned().unwrap_or(serde_json::Value::Null);
    IndicatorTopicSample {
        snapshot: serde_json::json!({
            "ok":valid,"success":valid,
            "status":if valid { raw_status.unwrap_or("unavailable") } else { "unavailable" },
            "timestamp":timestamp,"ts":timestamp,
            "firstMissingSignal":block.get("firstMissingSignal").cloned().unwrap_or(serde_json::Value::Null)
        }),
        indicator_state: None,
        unavailable_signal: (!valid).then_some(signal),
    }
}

fn project_services(
    block: &serde_json::Map<String, serde_json::Value>,
    guest: bool,
    admin_units: Option<&serde_json::Value>,
) -> IndicatorTopicSample {
    let rows = block.get("rows").and_then(serde_json::Value::as_array);
    let metadata = caduceus_indicator_portals();
    let metadata_signal = metadata.as_ref().err().cloned();
    let portals = metadata
        .as_ref()
        .ok()
        .map(|config| config.portals.as_slice())
        .unwrap_or(&[]);
    let projected = project_service_rows(rows, portals, guest, admin_units);
    let up = projected
        .iter()
        .filter(|row| row.get("status").and_then(serde_json::Value::as_str) == Some("up"))
        .count();
    let total = projected.len();
    let all_up = total > 0 && up == total;
    let all_down = total > 0
        && projected
            .iter()
            .all(|row| row.get("status").and_then(serde_json::Value::as_str) == Some("down"));
    let all_unknown = total == 0
        || projected
            .iter()
            .all(|row| row.get("status").and_then(serde_json::Value::as_str) == Some("unknown"));
    let status = if rows.is_none() || (guest && metadata_signal.is_some()) {
        "unavailable"
    } else if all_up {
        "up"
    } else if all_down {
        "down"
    } else if all_unknown {
        "unknown"
    } else {
        "partial"
    };
    let timestamp = block.get("ts").cloned().unwrap_or(serde_json::Value::Null);
    let mut first_missing = block
        .get("firstMissingSignal")
        .and_then(serde_json::Value::as_str)
        .filter(|signal| !signal.is_empty() && *signal != "none")
        .map(str::to_owned);
    if first_missing.is_none() {
        first_missing = metadata_signal.clone();
    }
    let unavailable_signal = if rows.is_none() {
        Some(stats_signal_from_object(block, "caduceus-stats-services-rows-unavailable"))
    } else {
        metadata_signal
    };
    IndicatorTopicSample {
        snapshot: serde_json::json!({
            "ok":unavailable_signal.is_none(),"success":unavailable_signal.is_none(),
            "status":status,"services":projected,"rows":projected,
            "counts":{"up":up,"total":total},"timestamp":timestamp,"ts":timestamp,
            "firstMissingSignal":first_missing
        }),
        indicator_state: None,
        unavailable_signal,
    }
}

fn project_service_rows(
    rows: Option<&Vec<serde_json::Value>>,
    portals: &[PortalEntry],
    guest: bool,
    admin_units: Option<&serde_json::Value>,
) -> Vec<serde_json::Value> {
    rows.into_iter()
        .flatten()
        .filter_map(|row| {
            let name = row.get("name").and_then(serde_json::Value::as_str)?;
            let metadata = portals.iter().find(|portal| portal.name == name);
            if guest && metadata.is_none_or(|portal| !portal.visible) {
                return None;
            }
            let status = row
                .get("status")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("unknown");
            let declared_units = row
                .get("units")
                .and_then(serde_json::Value::as_array)
                .into_iter()
                .flatten()
                .filter_map(serde_json::Value::as_str)
                .map(str::to_owned)
                .collect::<Vec<_>>();
            let unit_names = if guest {
                metadata
                    .map(|portal| {
                        portal
                            .services
                            .iter()
                            .map(|service| super::normalize_systemd_unit(service))
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_default()
            } else {
                declared_units
            };
            let systemd_name = if unit_names.is_empty() {
                serde_json::Value::Null
            } else {
                serde_json::Value::String(unit_names.join(", "))
            };
            let description = metadata
                .map(|portal| serde_json::Value::String(portal.description.clone()))
                .unwrap_or(serde_json::Value::Null);
            let port = metadata
                .and_then(|portal| portal.port)
                .map(|port| serde_json::json!(port))
                .unwrap_or(serde_json::Value::Null);
            let script_managed = metadata.map(|portal| portal.r#type == "script");
            let mut projected = serde_json::Map::new();
            projected.insert("name".to_owned(), serde_json::Value::String(name.to_owned()));
            projected.insert("description".to_owned(), description);
            projected.insert("systemdName".to_owned(), systemd_name);
            projected.insert("isActive".to_owned(), serde_json::Value::Bool(status == "up"));
            projected.insert("status".to_owned(), serde_json::Value::String(status.to_owned()));
            projected.insert("statusDetails".to_owned(), serde_json::Value::String(status.to_owned()));
            projected.insert(
                "isScriptManaged".to_owned(),
                script_managed.map(serde_json::Value::Bool).unwrap_or(serde_json::Value::Null),
            );
            projected.insert("port".to_owned(), port);
            projected.insert(
                "needsReboot".to_owned(),
                script_managed.map(serde_json::Value::Bool).unwrap_or(serde_json::Value::Null),
            );
            projected.insert(
                "firstMissingSignal".to_owned(),
                row.get("firstMissingSignal")
                    .cloned()
                    .unwrap_or(serde_json::Value::Null),
            );
            if !guest {
                let (enabled, enabled_signal) = service_row_enabled(&unit_names, admin_units);
                projected.insert("enabled".to_owned(), enabled.clone());
                projected.insert("isEnabled".to_owned(), enabled);
                projected.insert(
                    "enabledFirstMissingSignal".to_owned(),
                    enabled_signal.map(serde_json::Value::String).unwrap_or(serde_json::Value::Null),
                );
            }
            Some(serde_json::Value::Object(projected))
        })
        .collect()
}

fn service_row_enabled(
    unit_names: &[String],
    admin_units: Option<&serde_json::Value>,
) -> (serde_json::Value, Option<String>) {
    let units = admin_units.and_then(serde_json::Value::as_array);
    let mut enabled_count = 0usize;
    let mut known_count = 0usize;
    let mut saw_disabled = false;
    let mut first_missing = None;
    for unit_name in unit_names {
        let unit = units.and_then(|units| {
            units.iter().find(|entry| {
                entry.get("unit").and_then(serde_json::Value::as_str) == Some(unit_name.as_str())
            })
        });
        let enabled = unit
            .and_then(|entry| entry.get("enabled"))
            .and_then(serde_json::Value::as_bool);
        match enabled {
            Some(true) => {
                known_count += 1;
                enabled_count += 1;
            }
            Some(false) => {
                known_count += 1;
                saw_disabled = true;
            }
            None => {
                let signal = stats_signal(unit, "caduceus-stats-services-unit-enabled-state-unobserved");
                if first_missing.is_none() {
                    first_missing = Some(signal);
                }
            }
        }
    }
    let value = if saw_disabled {
        serde_json::Value::Bool(false)
    } else if !unit_names.is_empty() && known_count == unit_names.len() && enabled_count == unit_names.len() {
        serde_json::Value::Bool(true)
    } else {
        serde_json::Value::Null
    };
    (value, first_missing)
}

fn project_vpn_guest(block: &serde_json::Map<String, serde_json::Value>) -> IndicatorTopicSample {
    let vpn = block.get("vpnStatus").and_then(serde_json::Value::as_str);
    let transmission = block.get("transmissionStatus").and_then(serde_json::Value::as_str);
    let valid = vpn.is_some_and(|value| matches!(value, "running" | "stopped"))
        && transmission.is_some_and(|value| matches!(value, "running" | "stopped"));
    let signal = stats_signal_from_object(block, "caduceus-stats-vpn-state-unavailable");
    let running = usize::from(vpn == Some("running")) + usize::from(transmission == Some("running"));
    let indicator_state = match running {
        2 => "ok",
        1 => "warn",
        _ => "error",
    };
    IndicatorTopicSample {
        snapshot: serde_json::json!({
            "ok":valid,"success":valid,"status":if valid { "available" } else { "unavailable" },
            "vpnStatus":vpn,
            "transmissionStatus":transmission,
            "timestamp":block.get("ts").cloned().unwrap_or(serde_json::Value::Null),
            "ts":block.get("ts").cloned().unwrap_or(serde_json::Value::Null),
            "firstMissingSignal":block.get("firstMissingSignal").cloned().unwrap_or(serde_json::Value::Null)
        }),
        indicator_state: Some(if valid { indicator_state } else { "unavailable" }),
        unavailable_signal: (!valid).then_some(signal),
    }
}

fn stats_signal_from_object(
    block: &serde_json::Map<String, serde_json::Value>,
    fallback: &str,
) -> String {
    block
        .get("firstMissingSignal")
        .and_then(serde_json::Value::as_str)
        .filter(|signal| !signal.is_empty() && *signal != "none")
        .unwrap_or(fallback)
        .to_owned()
}

fn merge_admin_indicator_fields(
    topic_id: &str,
    snapshot: &mut serde_json::Value,
    admin: &serde_json::Map<String, serde_json::Value>,
) {
    match topic_id {
        "tailscale.status" => {
            for field in ["ip", "tailnet", "loginUrl", "isEnabled", "enabledFirstMissingSignal"] {
                if let Some(value) = admin.get(field) {
                    snapshot[field] = value.clone();
                }
            }
        }
        "internet.status" => {
            if let Some(value) = admin.get("publicIp") {
                snapshot["publicIp"] = value.clone();
            }
            if let Some(value) = admin.get("ts") {
                snapshot["publicIpTimestamp"] = value.clone();
            }
        }
        "services.status" => {
            if let Some(value) = admin.get("units") {
                snapshot["units"] = value.clone();
            }
        }
        "vpn.status" => {
            for field in ["transmissionUnit", "enabled"] {
                if let Some(value) = admin.get(field) {
                    snapshot[field] = value.clone();
                }
            }
        }
        _ => {}
    }
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
            "firstMissingSignal": "CORONATIO_BUILD_SHA"
        }));
    }
    let path = format!(
        "/api/v1/coronatio/source-currency?buildSha={}",
        crate::CORONATIO_BUILD_SHA
    );
    let readback = caduceus_http("GET", &path);
    if !readback.ok {
        return Err(format!("{}: {path}", readback.first_missing_signal));
    }
    let Some(object) = readback.body.as_object() else {
        return Err(format!("caduceus-invalid-json: {path}"));
    };
    if session == Session::Admin {
        return Ok(serde_json::Value::Object(object.clone()));
    }
    let mut projection = serde_json::Map::new();
    for field in ["ok", "schema", "originMainSha", "buildSha", "relation", "firstMissingSignal"] {
        if let Some(value) = object.get(field) {
            projection.insert(field.to_owned(), value.clone());
        }
    }
    projection.entry("ok".to_owned()).or_insert(serde_json::Value::Bool(true));
    Ok(serde_json::Value::Object(projection))
}
