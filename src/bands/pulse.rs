mod pulse {
    use super::*;
    use axum::response::sse::{Event, KeepAlive, Sse};
    use futures_util::{stream, StreamExt};
    use std::{collections::HashMap, convert::Infallible, sync::{atomic::{AtomicBool, Ordering}, Arc, OnceLock, RwLock}, time::Instant};
    use tokio::sync::{broadcast, mpsc};
    use tokio::time::{interval, sleep_until, Instant as TokioInstant};

    pub(crate) const STATS_INTERVAL_SECONDS: u64 = 1;
    const PULSE_LEASE_SECONDS: u64 = 30;
    const PULSE_KEEP_ALIVE_SECONDS: u64 = 10;
    static STATS_TICKER_STARTED: AtomicBool = AtomicBool::new(false);
    #[cfg(test)]
    static STATS_TICKER_TEST_ENABLED: AtomicBool = AtomicBool::new(false);

    pub(crate) struct HeldStats {
        pub(crate) stats: serde_json::Value,
        pub(crate) stats_at: Instant,
        pub(crate) history: serde_json::Value,
        pub(crate) history_at: Instant,
        pub(crate) history_status: u16,
        pub(crate) roster: StatsKeaLeases,
        pub(crate) roster_at: Instant,
    }
    static HELD_STATS: OnceLock<Arc<RwLock<Option<HeldStats>>>> = OnceLock::new();
    static STATS_REFRESH_GUARD: OnceLock<std::sync::Mutex<()>> = OnceLock::new();
    fn held_stats() -> &'static Arc<RwLock<Option<HeldStats>>> { HELD_STATS.get_or_init(|| Arc::new(RwLock::new(None))) }
    fn refresh_stats_pool(seed_only: bool) {
        let _guard=STATS_REFRESH_GUARD.get_or_init(Default::default).lock().unwrap();
        if seed_only && held_stats().read().unwrap().is_some() { return; }
        let (refresh_history,now)={let held=held_stats().read().unwrap(); let now=Instant::now(); (held.as_ref().is_none_or(|h| h.history_at.elapsed()>=Duration::from_secs(60)),now)};
        let stats=super::caduceus_stats_value("/api/v1/appliance/stats");
        let history=refresh_history.then(||super::caduceus_http("GET","/api/v1/appliance/stats/history"));
        let (roster_at,roster)=super::stats_identity_roster_cached();
        let mut held=held_stats().write().unwrap();
        let held=held.get_or_insert_with(||HeldStats{stats:serde_json::Value::Null,stats_at:now,history:serde_json::Value::Null,history_at:now,history_status:502,roster:StatsKeaLeases{status:"unavailable".to_string(),entries:Vec::new()},roster_at:now});
        held.stats=stats; held.stats_at=now; if let Some(history)=history { held.history=history.body; held.history_status=history.status; held.history_at=now; } held.roster=roster; held.roster_at=roster_at;
    }
    pub(crate) async fn stats_snapshot() -> StatsSnapshot {
        if held_stats().read().unwrap().is_none() { let _=tokio::task::spawn_blocking(|| refresh_stats_pool(true)).await; }
        let held=held_stats().read().unwrap(); let held=held.as_ref().expect("stats pool seeded"); super::stats_caduceus_snapshot(held.stats.clone(),held.roster.clone())
    }
    pub(crate) async fn stats_history() -> impl IntoResponse {
        if held_stats().read().unwrap().is_none() { let _=tokio::task::spawn_blocking(|| refresh_stats_pool(true)).await; }
        let held=held_stats().read().unwrap(); let held=held.as_ref().expect("stats pool seeded"); (StatusCode::from_u16(held.history_status).unwrap_or(StatusCode::BAD_GATEWAY),Json(held.history.clone()))
    }
    async fn collect_stats() { if tokio::task::spawn_blocking(|| refresh_stats_pool(false)).await.is_ok() { poke(PokeTopic::StatsTick); } }

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
            ticker.tick().await;
            loop {
                ticker.tick().await;
                #[cfg(test)]
                if !STATS_TICKER_TEST_ENABLED.load(Ordering::SeqCst) {
                    STATS_TICKER_STARTED.store(false, Ordering::SeqCst);
                    break;
                }
                let live=bus().memberships.lock().unwrap().values().any(|m|m.deadline>Instant::now());
                if live { collect_stats().await; }
            }
        });
    }

    #[cfg(test)]
    pub(crate) fn set_stats_ticker_enabled_for_test(enabled: bool) {
        STATS_TICKER_TEST_ENABLED.store(enabled, Ordering::SeqCst);
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
        let session = session_from_headers(&headers);
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
        if !validate_host_membership(&query.stream_id, &headers) {
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
        if session_from_headers(&headers) != Session::Admin {
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

    fn validate_host_membership(stream_id: &str, headers: &axum::http::HeaderMap) -> bool {
        let is_host = bus().memberships.lock().unwrap().get(stream_id).is_some_and(|membership| membership.session == Session::Admin);
        if is_host && session_from_headers(headers) != Session::Admin {
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
