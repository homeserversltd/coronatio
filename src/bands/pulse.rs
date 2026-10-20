mod pulse {
    use super::*;
    use axum::response::sse::{Event, KeepAlive, Sse};
    use futures_util::{stream, StreamExt};
    use std::{collections::HashMap, convert::Infallible, sync::{atomic::{AtomicBool, Ordering}, Arc, OnceLock, RwLock}, time::Instant};
    use tokio::sync::{broadcast, mpsc};
    use tokio::time::{interval, sleep_until, Instant as TokioInstant};

    pub(crate) const STATS_INTERVAL_SECONDS: u64 = 1;
    const PULSE_LEASE_SECONDS: u64 = 30;
    const STATS_HISTORY_ROW_LIMIT: usize = 60;
    const PULSE_KEEP_ALIVE_SECONDS: u64 = 10;
    static STATS_TICKER_STARTED: AtomicBool = AtomicBool::new(false);
    #[cfg(test)]
    static STATS_TICKER_TEST_ENABLED: AtomicBool = AtomicBool::new(false);
    static STATS_REFRESH_IN_FLIGHT: AtomicBool = AtomicBool::new(false);

    #[derive(Clone)]
    pub(crate) struct HeldStats {
        pub(crate) stats: serde_json::Value,
        pub(crate) stats_at: Option<Instant>,
        pub(crate) sampled_at: u64,
        pub(crate) stats_first_missing_signal: String,
        pub(crate) stats_last_missed_at: Option<u64>,
        pub(crate) stats_stale_sections: Vec<String>,
        pub(crate) stats_sample_id: Option<String>,
        pub(crate) stats_generation: u64,
        revision: u64,
        section_at: HashMap<String, Instant>,
    }
    #[derive(Clone)]
    pub(crate) struct StatsCurrentObservation {
        pub(crate) body: serde_json::Value,
        pub(crate) sampled_at: u64,
        pub(crate) sample_age_ms: Option<u64>,
        pub(crate) section_sample_age_ms: HashMap<String, u64>,
        pub(crate) stale_sections: Vec<String>,
        pub(crate) stale: bool,
        pub(crate) first_missing_signal: String,
        pub(crate) last_missed_at: Option<u64>,
        pub(crate) sample_id: Option<String>,
        pub(crate) generation: u64,
    }
    static HELD_STATS: OnceLock<Arc<RwLock<Option<HeldStats>>>> = OnceLock::new();
    fn held_stats() -> &'static Arc<RwLock<Option<HeldStats>>> { HELD_STATS.get_or_init(|| Arc::new(RwLock::new(None))) }
    fn unix_millis_now() -> u64 { SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis().min(u64::MAX as u128) as u64 }
    fn has_caduceus_stats_fields(body: &serde_json::Value) -> bool {
        fn has_value(value: Option<&serde_json::Value>, keys: &[&str]) -> bool {
            value.and_then(serde_json::Value::as_object).is_some_and(|object| keys.iter().any(|key| object.get(*key).is_some_and(|field| !field.is_null())))
        }
        fn has_array(value: Option<&serde_json::Value>, key: &str) -> bool {
            value.and_then(|value| value.get(key)).and_then(serde_json::Value::as_array).is_some()
        }
        has_value(body.get("load"), &["one", "five", "fifteen"])
            || has_value(body.get("memory"), &["MemTotal", "MemAvailable", "SwapTotal", "SwapFree", "usedBytes", "usedBytesSwap"])
            || body.pointer("/temperature/celsius").is_some_and(|value| !value.is_null())
            || has_value(body.get("network"), &["receivedBytes", "sentBytes"])
            || has_array(body.get("network"), "interfaces")
            || has_value(body.get("tcp"), &["established", "listen"])
            || has_array(body.get("disk"), "usage")
            || has_array(body.get("disk"), "io")
            || body.get("processes").and_then(serde_json::Value::as_array).is_some()
    }
    fn section_present(body: &serde_json::Value, section: &str) -> bool {
        match section {
            "cpu" => body.pointer("/load/one").is_some_and(|value| !value.is_null()) || body.pointer("/temperature/celsius").is_some_and(|value| !value.is_null()),
            "memory" => has_caduceus_stats_fields(&serde_json::json!({"memory":body.get("memory")})),
            "network" => has_caduceus_stats_fields(&serde_json::json!({"network":body.get("network"),"tcp":body.get("tcp")})),
            "storage" => body.pointer("/disk/usage").and_then(serde_json::Value::as_array).is_some(),
            "io" => body.pointer("/disk/io").and_then(serde_json::Value::as_array).is_some(),
            "processes" => body.get("processes").and_then(serde_json::Value::as_array).is_some(),
            _ => false,
        }
    }
    fn merge_observation(old: &mut serde_json::Value, new: &serde_json::Value) {
        match (old, new) {
            (serde_json::Value::Object(old), serde_json::Value::Object(new)) => {
                for (key, value) in new { if let Some(previous) = old.get_mut(key) { merge_observation(previous, value); } else { old.insert(key.clone(), value.clone()); } }
            }
            (_, serde_json::Value::Null) => {},
            (old, new) => *old = new.clone(),
        }
    }
    fn normalized_miss_signal(signal: &str, fallback: &str) -> String {
        let signal = signal.trim();
        if signal.is_empty() || signal.eq_ignore_ascii_case("none") { fallback.to_string() } else { signal.to_string() }
    }
    fn refresh_stats_pool() {
        let stats = super::indicators::caduceus_stats_sample();
        let sections = ["cpu", "memory", "network", "storage", "io", "processes"];
        for _ in 0..8 {
            let previous = held_stats().read().unwrap_or_else(std::sync::PoisonError::into_inner).clone();
            let revision = previous.as_ref().map(|held| held.revision).unwrap_or(0);
            if previous.as_ref().is_some_and(|held| stats.generation < held.stats_generation) { return; }
            let same_sample = previous.as_ref().is_some_and(|held| held.stats_sample_id == stats.sample_id && stats.sample_id.is_some());
            let mut next = previous.unwrap_or_else(|| HeldStats {
                stats: serde_json::Value::Null, stats_at: None, sampled_at: 0,
                stats_first_missing_signal: String::new(), stats_last_missed_at: None,
                stats_stale_sections: sections.iter().map(|section| section.to_string()).collect(),
                stats_sample_id: None, stats_generation: 0, revision, section_at: HashMap::new(),
            });
            if same_sample {
                if stats.stale {
                    let signal = normalized_miss_signal(&stats.first_missing_signal, "caduceus-stats-read-failed");
                    if next.stats_first_missing_signal != signal || next.stats_last_missed_at.is_none() {
                        next.stats_last_missed_at = Some(unix_millis_now());
                    }
                    next.stats_first_missing_signal = signal;
                    next.stats_stale_sections = sections.iter().map(|section| (*section).to_owned()).collect();
                    let mut held = held_stats().write().unwrap_or_else(std::sync::PoisonError::into_inner);
                    if held.as_ref().map(|value| value.revision).unwrap_or(0) != revision { continue; }
                    next.revision = revision.wrapping_add(1);
                    *held = Some(next);
                }
                return;
            }
            let observed_at = stats.sampled_at.unwrap_or_else(Instant::now);
            let wall_at = stats.sampled_at_unix_ms.unwrap_or_else(unix_millis_now);
            if stats.ok && has_caduceus_stats_fields(&stats.body) {
                let mut fresh = sections.iter().filter(|section| section_present(&stats.body, section)).copied().collect::<Vec<_>>();
                let new_temperature = stats.body.pointer("/temperature/celsius").is_some_and(|value| !value.is_null());
                let retained_temperature = next.stats.pointer("/temperature/celsius").is_some_and(|value| !value.is_null());
                if fresh.contains(&"cpu") && !new_temperature && retained_temperature { fresh.retain(|section| *section != "cpu"); }
                if !fresh.is_empty() {
                    if next.stats.is_null() { next.stats = stats.body.clone(); }
                    else { merge_observation(&mut next.stats, &stats.body); }
                    for section in &fresh { next.section_at.insert((*section).to_owned(), observed_at); }
                    next.stats_stale_sections = sections.iter().filter(|section| !fresh.contains(section)).map(|section| (*section).to_owned()).collect();
                    next.stats_at = Some(observed_at);
                    next.sampled_at = wall_at;
                    next.stats_sample_id = stats.sample_id.clone().or(next.stats_sample_id);
                    next.stats_generation = stats.generation.max(next.stats_generation);
                    if next.stats_stale_sections.is_empty() {
                        next.stats_first_missing_signal.clear();
                        next.stats_last_missed_at = None;
                    } else {
                        next.stats_first_missing_signal = if stats.first_missing_signal.is_empty() || stats.first_missing_signal == "none" { "caduceus-stats-partial-sample".to_owned() } else { stats.first_missing_signal.clone() };
                        next.stats_last_missed_at = Some(wall_at);
                    }
                } else {
                    next.stats_first_missing_signal = "caduceus-stats-body-empty".to_owned();
                    next.stats_last_missed_at.get_or_insert_with(unix_millis_now);
                    next.stats_stale_sections = sections.iter().map(|section| (*section).to_owned()).collect();
                }
            } else {
                next.stats_first_missing_signal = if stats.ok { "caduceus-stats-body-empty".to_owned() } else { normalized_miss_signal(&stats.first_missing_signal, "caduceus-stats-read-failed") };
                next.stats_last_missed_at.get_or_insert_with(unix_millis_now);
                next.stats_stale_sections = sections.iter().map(|section| (*section).to_owned()).collect();
            }
            let mut held = held_stats().write().unwrap_or_else(std::sync::PoisonError::into_inner);
            if held.as_ref().map(|value| value.revision).unwrap_or(0) != revision { continue; }
            if held.as_ref().is_some_and(|value| stats.generation < value.stats_generation) { return; }
            next.revision = revision.wrapping_add(1);
            *held = Some(next);
            return;
        }
    }
    pub(crate) fn stats_current_observation() -> StatsCurrentObservation {
        let held = held_stats().read().unwrap_or_else(std::sync::PoisonError::into_inner).clone();
        match held {
            Some(held) => {
                let now = Instant::now();
                let ages = held.section_at.iter().map(|(section, at)| (section.clone(), now.duration_since(*at).as_millis().min(u64::MAX as u128) as u64)).collect();
                let age = held.stats_at.map(|at| now.duration_since(at).as_millis().min(u64::MAX as u128) as u64);
                super::load_trace::record_cache_link("stats.held-current", held.stats_sample_id.clone(), age.map(Duration::from_millis));
                StatsCurrentObservation {
                    body: held.stats, sampled_at: held.sampled_at, sample_age_ms: age,
                    section_sample_age_ms: ages, stale_sections: held.stats_stale_sections.clone(),
                    stale: !held.stats_first_missing_signal.is_empty() || !held.stats_stale_sections.is_empty(),
                    first_missing_signal: held.stats_first_missing_signal, last_missed_at: held.stats_last_missed_at,
                    sample_id: held.stats_sample_id, generation: held.stats_generation,
                }
            }
            None => StatsCurrentObservation {
                body: serde_json::Value::Null, sampled_at: 0, sample_age_ms: None,
                section_sample_age_ms: HashMap::new(),
                stale_sections: vec!["cpu".to_owned(), "memory".to_owned(), "network".to_owned(), "storage".to_owned(), "io".to_owned(), "processes".to_owned()],
                stale: true, first_missing_signal: "caduceus-stats-not-sampled".to_owned(), last_missed_at: None,
                sample_id: None, generation: 0,
            },
        }
    }
    pub(crate) async fn stats_snapshot() -> StatsSnapshot {
        let result = super::load_trace::spawn_blocking_with_trace(|| {
            refresh_stats_pool();
            let current=stats_current_observation();
            let (roster_at,roster,roster_signal)=super::stats_identity_roster_cached();
            let leases=super::stats_kea_leases_cached();
            let mut snapshot=super::stats_caduceus_snapshot(current.body,roster,leases.entries,current.sampled_at);
            snapshot.telemetry.sample_age_ms=current.sample_age_ms;
            snapshot.telemetry.last_missed_at=current.last_missed_at;
            snapshot.telemetry.roster_sample_age_ms=Some(roster_at.elapsed().as_millis().min(u64::MAX as u128) as u64);
            snapshot.telemetry.roster_last_missed_at=(!roster_signal.is_empty()).then(unix_millis_now);
            snapshot.telemetry.roster_first_missing_signal=roster_signal.clone();
            snapshot.telemetry.first_missing_signal=if !current.first_missing_signal.is_empty(){current.first_missing_signal.clone()}else if !roster_signal.is_empty(){roster_signal.clone()}else if leases.stale{leases.first_missing_signal.clone()}else{String::new()};
            snapshot.telemetry.section_stale=StatsSectionStale{cpu:current.stale_sections.iter().any(|section|section=="cpu"),memory:current.stale_sections.iter().any(|section|section=="memory"),network:current.stale_sections.iter().any(|section|section=="network"),storage:current.stale_sections.iter().any(|section|section=="storage"),io:current.stale_sections.iter().any(|section|section=="io"),processes:current.stale_sections.iter().any(|section|section=="processes"),kea_leases:!roster_signal.is_empty()||leases.stale};
            snapshot.telemetry.section_sample_age_ms=current.section_sample_age_ms;
            snapshot.telemetry.stale_sections=current.stale_sections;
            snapshot
        }).await;
        result.unwrap_or_else(|_|super::stats_caduceus_snapshot(serde_json::Value::Null,StatsKeaLeases{status:"unavailable".to_owned(),entries:Vec::new()},Vec::new(),0))
    }
    pub(crate) async fn stats_history() -> impl IntoResponse {
        let history=super::load_trace::spawn_blocking_with_trace(||super::indicators::caduceus_stats_history()).await;
        match history {
            Ok(readback)=>(StatusCode::from_u16(readback.status).unwrap_or(if readback.ok{StatusCode::OK}else{StatusCode::BAD_GATEWAY}),Json(readback.body)),
            Err(_)=>(StatusCode::BAD_GATEWAY,Json(serde_json::json!({"ok":false,"firstMissingSignal":"caduceus-stats-history-task-failed"}))),
        }
    }
    pub(crate) async fn stats_current_route_observation() -> StatsCurrentObservation {
        let _ = super::load_trace::spawn_blocking_with_trace(refresh_stats_pool).await;
        stats_current_observation()
    }
    fn schedule_stats_collection() {
        if STATS_REFRESH_IN_FLIGHT.compare_exchange(false,true,Ordering::AcqRel,Ordering::Acquire).is_err() { return; }
        tokio::spawn(async {
            let _ = super::load_trace::spawn_blocking_with_trace(refresh_stats_pool).await;
            STATS_REFRESH_IN_FLIGHT.store(false,Ordering::Release);
        });
    }

    #[allow(dead_code)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub(crate) enum PokeTopic {
        TabsChanged,
        ElementsChanged,
        StatsTick,
        AdminSystem,
    }

    impl PokeTopic {
        pub(crate) fn wire_name(self) -> &'static str {
            match self {
                PokeTopic::TabsChanged => "tabs.changed",
                PokeTopic::ElementsChanged => "elements.changed",
                PokeTopic::StatsTick => "stats.tick",
                PokeTopic::AdminSystem => "admin.system",
            }
        }

        fn admin_only(self) -> bool {
            matches!(self, PokeTopic::AdminSystem)
        }
    }

    #[derive(Clone)]
    struct PulseBus {
        public_tx: broadcast::Sender<PokeTopic>,
        admin_tx: broadcast::Sender<PokeTopic>,
        memberships: Arc<Mutex<HashMap<String, PulseMembership>>>,
    }

    struct PulseMembership {
        deadline: Instant,
        session: Session,
        document: Option<String>,
        control_tx: mpsc::UnboundedSender<PulseControl>,
    }

    enum PulseControl {
        Host(broadcast::Receiver<PokeTopic>),
        Guest,
    }

    static PULSE_BUS: OnceLock<PulseBus> = OnceLock::new();

    fn bus() -> &'static PulseBus {
        PULSE_BUS.get_or_init(|| {
            let (public_tx, _) = broadcast::channel(64);
            let (admin_tx, _) = broadcast::channel(64);
            PulseBus {
                public_tx,
                admin_tx,
                memberships: Arc::new(Mutex::new(HashMap::new())),
            }
        })
    }

    pub(crate) fn poke(topic: PokeTopic) {
        if matches!(topic, PokeTopic::TabsChanged) {
            super::invalidate_xenia_status_cache();
            super::xenia_discovery::invalidate();
        }
        let bus = bus();
        if topic.admin_only() {
            let _ = bus.admin_tx.send(topic);
        } else {
            let _ = bus.public_tx.send(topic);
        }
    }

    pub(crate) fn ensure_stats_ticker_started() {
        #[cfg(test)]
        if !STATS_TICKER_TEST_ENABLED.load(Ordering::SeqCst) {
            return;
        }
        if STATS_TICKER_STARTED
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_err()
        {
            return;
        }
        let Ok(handle) = tokio::runtime::Handle::try_current() else {
            STATS_TICKER_STARTED.store(false, Ordering::SeqCst);
            return;
        };
        handle.spawn(async {
            let mut ticker = interval(Duration::from_secs(STATS_INTERVAL_SECONDS));
            ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
            ticker.tick().await;
            loop {
                ticker.tick().await;
                #[cfg(test)]
                if !STATS_TICKER_TEST_ENABLED.load(Ordering::SeqCst) {
                    STATS_TICKER_STARTED.store(false, Ordering::SeqCst);
                    break;
                }
                let live=bus().memberships.lock().unwrap().values().any(|m|m.deadline>Instant::now());
                if live { poke(PokeTopic::StatsTick); schedule_stats_collection(); }
            }
        });
    }

    #[cfg(test)]
    pub(crate) fn set_stats_ticker_enabled_for_test(enabled: bool) {
        STATS_TICKER_TEST_ENABLED.store(enabled, Ordering::SeqCst);
        if !enabled {
            STATS_TICKER_STARTED.store(false, Ordering::SeqCst);
        }
    }

    #[cfg(test)]
    pub(crate) fn reset_stats_pool_for_test() { *held_stats().write().unwrap()=None; }

    #[derive(Debug, Clone)]
    pub(crate) struct PulseWireFrame {
        pub(crate) event: String,
        pub(crate) id: Option<String>,
        pub(crate) data: String,
    }

    impl PulseWireFrame {
        fn into_event(self) -> Event {
            let mut event = Event::default().event(self.event).data(self.data);
            if let Some(id) = self.id {
                event = event.id(id);
            }
            event
        }

        #[cfg(test)]
        pub(crate) fn wire_text(&self) -> String {
            match &self.id {
                Some(id) => format!("event: {}\nid: {}\ndata: {}\n\n", self.event, id, self.data),
                None => format!("event: {}\ndata: {}\n\n", self.event, self.data),
            }
        }
    }

    struct PulseSubscription {
        stream_id: String,
        session: Session,
        public_rx: broadcast::Receiver<PokeTopic>,
        admin_rx: Option<broadcast::Receiver<PokeTopic>>,
        control_rx: mpsc::UnboundedReceiver<PulseControl>,
        memberships: Arc<Mutex<HashMap<String, PulseMembership>>>,
        opened: bool,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub(crate) struct PulseRenewQuery {
        stream_id: String,
    }

    impl Drop for PulseSubscription {
        fn drop(&mut self) {
            self.memberships.lock().unwrap().remove(&self.stream_id);
        }
    }

    pub(crate) async fn stats_pulse_route(headers: axum::http::HeaderMap) -> impl IntoResponse {
        let session = session_projection_from_headers(&headers).await;
        let (_stream_id, frames) = subscribe_stream(session, Duration::from_secs(PULSE_LEASE_SECONDS));
        Sse::new(frames.map(|frame| Ok::<Event, Infallible>(frame.into_event())))
            .keep_alive(
                KeepAlive::new()
                    .interval(Duration::from_secs(PULSE_KEEP_ALIVE_SECONDS))
                    .text("pulse.keepalive"),
            )
            .into_response()
    }

    pub(crate) async fn stats_pulse_renew_route(headers: axum::http::HeaderMap, Query(query): Query<PulseRenewQuery>) -> impl IntoResponse {
        if !validate_host_membership(&query.stream_id, &headers).await {
            return membership_response(StatusCode::UNAUTHORIZED, &query.stream_id, "attendance-refused");
        }
        if renew_stream(&query.stream_id, Duration::from_secs(PULSE_LEASE_SECONDS)) {
            Json(LeaseRenewalReadback {
                schema: "coronatio.stats.events.renewal.v1".to_string(),
                stream_id: query.stream_id,
                route: "/api/stats/pulse/renew".to_string(),
                lease_seconds: PULSE_LEASE_SECONDS,
                status: "renewed".to_string(),
                next_renewal_before_seconds: 20,
            })
            .into_response()
        } else {
            (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({
                    "schema": "coronatio.stats.events.renewal.v1",
                    "streamId": query.stream_id,
                    "status": "unknown-stream",
                })),
            )
                .into_response()
        }
    }

    pub(crate) async fn stats_pulse_upgrade_route(headers: axum::http::HeaderMap, Query(query): Query<PulseRenewQuery>) -> Response {
        let Some(document) = crate::caduceus_access::document_incarnation_from_headers(&headers) else {
            return membership_response(StatusCode::BAD_REQUEST, &query.stream_id, "document-required");
        };
        if session_projection_from_headers(&headers).await != Session::Admin {
            downgrade_stream(&query.stream_id, None);
            return membership_response(StatusCode::UNAUTHORIZED, &query.stream_id, "attendance-refused");
        }
        if upgrade_stream(&query.stream_id, document) {
            membership_response(StatusCode::OK, &query.stream_id, "upgraded")
        } else {
            membership_response(StatusCode::NOT_FOUND, &query.stream_id, "unknown-stream")
        }
    }

    pub(crate) async fn stats_pulse_downgrade_route(headers: axum::http::HeaderMap, Query(query): Query<PulseRenewQuery>) -> Response {
        let Some(document) = crate::caduceus_access::document_incarnation_from_headers(&headers) else {
            return membership_response(StatusCode::BAD_REQUEST, &query.stream_id, "document-required");
        };
        if let Some(attendance) = crate::caduceus_access::attendance_from_headers(&headers) { crate::caduceus_access::bust_attendance_projection(&attendance, &document, "stream-downgraded"); }
        if downgrade_stream(&query.stream_id, Some(&document)) {
            membership_response(StatusCode::OK, &query.stream_id, "downgraded")
        } else {
            membership_response(StatusCode::NOT_FOUND, &query.stream_id, "unknown-stream")
        }
    }

    fn membership_response(status: StatusCode, stream_id: &str, membership_status: &str) -> Response {
        (status, Json(serde_json::json!({
            "schema": "coronatio.stats.events.membership.v1",
            "streamId": stream_id,
            "status": membership_status,
        }))).into_response()
    }

    pub(crate) fn subscribe_stream(
        session: Session,
        lease_duration: Duration,
    ) -> (String, stream::BoxStream<'static, PulseWireFrame>) {
        let bus = bus();
        let stream_id = format!("pulse-{}", uuid::Uuid::new_v4());
        let deadline = Instant::now() + lease_duration;
        let (control_tx, control_rx) = mpsc::unbounded_channel();
        bus.memberships.lock().unwrap().insert(stream_id.clone(), PulseMembership {
            deadline,
            session,
            document: None,
            control_tx,
        });
        let subscription = PulseSubscription {
            stream_id: stream_id.clone(),
            session,
            public_rx: bus.public_tx.subscribe(),
            admin_rx: (session == Session::Admin).then(|| bus.admin_tx.subscribe()),
            control_rx,
            memberships: Arc::clone(&bus.memberships),
            opened: false,
        };
        let frames = stream::unfold(Some(subscription), move |state| async move {
            let mut subscription = state?;
            let frame = next_frame(&mut subscription, lease_duration).await;
            match frame.event.as_str() {
                "pulse.expired" => Some((frame, None)),
                _ => Some((frame, Some(subscription))),
            }
        })
        .boxed();
        (stream_id, frames)
    }

    pub(crate) fn renew_stream(stream_id: &str, lease_duration: Duration) -> bool {
        let mut memberships = bus().memberships.lock().unwrap();
        if let Some(membership) = memberships.get_mut(stream_id) {
            membership.deadline = Instant::now() + lease_duration;
            true
        } else {
            false
        }
    }

    #[cfg(test)]
    pub(crate) fn lease_exists_for_test(stream_id: &str) -> bool {
        bus().memberships.lock().unwrap().contains_key(stream_id)
    }

    pub(crate) fn upgrade_stream(stream_id: &str, document: String) -> bool {
        let bus = bus();
        let mut memberships = bus.memberships.lock().unwrap();
        let Some(membership) = memberships.get_mut(stream_id) else { return false; };
        let admin_rx = bus.admin_tx.subscribe();
        membership.session = Session::Admin;
        membership.document = Some(document);
        membership.control_tx.send(PulseControl::Host(admin_rx)).is_ok()
    }

    pub(crate) fn downgrade_stream(stream_id: &str, document: Option<&str>) -> bool {
        let mut memberships = bus().memberships.lock().unwrap();
        let Some(membership) = memberships.get_mut(stream_id) else { return false; };
        if document.is_some() && membership.document.as_deref() != document { return false; }
        membership.session = Session::Guest;
        membership.document = None;
        membership.control_tx.send(PulseControl::Guest).is_ok()
    }

    pub(crate) fn downgrade_document(document: &str) {
        let mut memberships = bus().memberships.lock().unwrap();
        for membership in memberships.values_mut().filter(|membership| membership.document.as_deref() == Some(document)) {
            membership.session = Session::Guest;
            membership.document = None;
            let _ = membership.control_tx.send(PulseControl::Guest);
        }
    }

    async fn validate_host_membership(stream_id: &str, headers: &axum::http::HeaderMap) -> bool {
        let is_host = bus().memberships.lock().unwrap().get(stream_id).is_some_and(|membership| membership.session == Session::Admin);
        if is_host && session_projection_from_headers(headers).await != Session::Admin {
            downgrade_stream(stream_id, None);
            false
        } else {
            true
        }
    }

    async fn next_frame(subscription: &mut PulseSubscription, lease_duration: Duration) -> PulseWireFrame {
        if let Some(frame) = open_frame(subscription, lease_duration) {
            return frame;
        }
        loop {
            while let Ok(control) = subscription.control_rx.try_recv() {
                apply_control(subscription, control);
            }
            if let Some(frame) = expired_frame_if_current_deadline_elapsed(subscription) {
                return frame;
            }
            let deadline = current_deadline(subscription);
            let sleep = sleep_until(TokioInstant::from_std(deadline));
            tokio::pin!(sleep);
            if subscription.session == Session::Admin {
                let admin_rx = subscription.admin_rx.as_mut().expect("admin receiver exists");
                tokio::select! {
                    Some(control) = subscription.control_rx.recv() => {
                        match control {
                            PulseControl::Host(admin_rx) => { subscription.session = Session::Admin; subscription.admin_rx = Some(admin_rx); }
                            PulseControl::Guest => { subscription.session = Session::Guest; subscription.admin_rx = None; }
                        }
                    }
                    topic = subscription.public_rx.recv() => {
                        if let Ok(topic) = topic { return poke_frame(topic); }
                    }
                    topic = admin_rx.recv() => {
                        if let Ok(topic) = topic { return poke_frame(topic); }
                    }
                    _ = &mut sleep => {
                        if let Some(frame) = expired_frame_if_current_deadline_elapsed(subscription) {
                            return frame;
                        }
                    }
                }
            } else {
                tokio::select! {
                    Some(control) = subscription.control_rx.recv() => {
                        apply_control(subscription, control);
                    }
                    topic = subscription.public_rx.recv() => {
                        if let Ok(topic) = topic { return poke_frame(topic); }
                    }
                    _ = &mut sleep => {
                        if let Some(frame) = expired_frame_if_current_deadline_elapsed(subscription) {
                            return frame;
                        }
                    }
                }
            }
        }
    }

    fn apply_control(subscription: &mut PulseSubscription, control: PulseControl) {
        match control {
            PulseControl::Host(admin_rx) => {
                subscription.session = Session::Admin;
                subscription.admin_rx = Some(admin_rx);
            }
            PulseControl::Guest => {
                subscription.session = Session::Guest;
                subscription.admin_rx = None;
            }
        }
    }

    fn current_deadline(subscription: &PulseSubscription) -> Instant {
        subscription
            .memberships
            .lock()
            .unwrap()
            .get(&subscription.stream_id)
            .map(|membership| membership.deadline)
            .unwrap_or_else(Instant::now)
    }

    fn expired_frame_if_current_deadline_elapsed(subscription: &PulseSubscription) -> Option<PulseWireFrame> {
        (Instant::now() >= current_deadline(subscription)).then(|| expired_frame(&subscription.stream_id))
    }

    fn open_frame(subscription: &mut PulseSubscription, _lease_duration: Duration) -> Option<PulseWireFrame> {
        let memberships = subscription.memberships.lock().unwrap();
        if !subscription.opened && memberships.contains_key(&subscription.stream_id) {
            drop(memberships);
            subscription.opened = true;
            return Some(PulseWireFrame {
                event: "pulse.open".to_string(),
                id: Some(subscription.stream_id.clone()),
                data: serde_json::json!({
                    "schema": "coronatio.pulse.stream.v1",
                    "streamId": subscription.stream_id,
                    "leaseSeconds": PULSE_LEASE_SECONDS,
                    "renewRoute": format!("/api/stats/pulse/renew?streamId={}", subscription.stream_id),
                })
                .to_string(),
            });
        }
        None
    }

    fn poke_frame(topic: PokeTopic) -> PulseWireFrame {
        PulseWireFrame {
            event: topic.wire_name().to_string(),
            id: None,
            data: "{}".to_string(),
        }
    }

    fn expired_frame(stream_id: &str) -> PulseWireFrame {
        bus().memberships.lock().unwrap().remove(stream_id);
        PulseWireFrame {
            event: "pulse.expired".to_string(),
            id: Some(stream_id.to_string()),
            data: serde_json::json!({
                "streamId": stream_id,
                "status": "expired",
            })
            .to_string(),
        }
    }
}
