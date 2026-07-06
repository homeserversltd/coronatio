    fn pulse_test_lock() -> &'static tokio::sync::Mutex<()> {
        static LOCK: OnceLock<tokio::sync::Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| tokio::sync::Mutex::new(()))
    }

    #[tokio::test]
    async fn pulse_wall_stream_route_stays_open_instead_of_single_frame_close() {
        let _guard = pulse_test_lock().lock().await;
        let temp = test_tab_root("pulse-open-route");
        let router = app(AppState { tab_root: Arc::new(temp) });
        let response = router
            .oneshot(Request::builder().uri("/api/stats/events").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.headers().get(header::CONTENT_TYPE).unwrap(), "text/event-stream");
        let body = response.into_body();
        let closed = tokio::time::timeout(Duration::from_millis(50), axum::body::to_bytes(body, usize::MAX)).await;
        assert!(closed.is_err(), "pulse stream closed like the retired one-frame stub");
    }

    #[tokio::test]
    async fn pulse_wall_tabs_changed_poke_reaches_subscribed_stream_data_free() {
        let _guard = pulse_test_lock().lock().await;
        use futures_util::StreamExt;
        let (_stream_id, mut stream) = pulse::subscribe_stream(Session::Guest, Duration::from_secs(1));
        let open = stream.next().await.unwrap();
        assert_eq!(open.event, "pulse.open");
        pulse::poke(pulse::PokeTopic::TabsChanged);
        let frame = tokio::time::timeout(Duration::from_millis(200), stream.next()).await.unwrap().unwrap();
        assert_eq!(frame.event, "tabs.changed");
        assert_eq!(frame.data, "{}");
    }

    #[tokio::test]
    async fn pulse_wall_stats_ticker_pokes_subscribed_stream_once_per_process_cadence() {
        let _guard = pulse_test_lock().lock().await;
        use futures_util::StreamExt;
        pulse::set_stats_ticker_enabled_for_test(true);
        let temp = test_tab_root("pulse-003b-stats-ticker");
        let state = AppState { tab_root: Arc::new(temp) };
        let _first_router = app(state.clone());
        let _second_router = app(state);
        let (_stream_id, mut stream) = pulse::subscribe_stream(Session::Guest, Duration::from_secs(3));
        assert_eq!(stream.next().await.unwrap().event, "pulse.open");

        let frame = tokio::time::timeout(
            Duration::from_millis((pulse::STATS_INTERVAL_SECONDS * 1000) + 300),
            stream.next(),
        )
        .await
        .unwrap()
        .unwrap();
        assert_eq!(frame.event, "stats.tick");
        assert_eq!(frame.data, "{}");

        let doubled = tokio::time::timeout(Duration::from_millis(150), stream.next()).await;
        assert!(doubled.is_err(), "second app construction doubled the stats.tick cadence");
        pulse::set_stats_ticker_enabled_for_test(false);
    }

    #[tokio::test]
    async fn pulse_wall_admin_topic_lanes_are_selected_at_subscription_construction() {
        let _guard = pulse_test_lock().lock().await;
        use futures_util::StreamExt;
        let (_guest_id, mut guest) = pulse::subscribe_stream(Session::Guest, Duration::from_secs(1));
        let (_admin_id, mut admin) = pulse::subscribe_stream(Session::Admin, Duration::from_secs(1));
        assert_eq!(guest.next().await.unwrap().event, "pulse.open");
        assert_eq!(admin.next().await.unwrap().event, "pulse.open");
        pulse::poke(pulse::PokeTopic::AdminSystem);
        let admin_frame = tokio::time::timeout(Duration::from_millis(200), admin.next()).await.unwrap().unwrap();
        assert_eq!(admin_frame.event, "admin.system");
        let guest_frame = tokio::time::timeout(Duration::from_millis(100), guest.next()).await;
        assert!(guest_frame.is_err(), "guest stream received admin-only topic");
    }

    #[tokio::test]
    async fn pulse_wall_poke_frames_never_contain_config_marker_values() {
        let _guard = pulse_test_lock().lock().await;
        use futures_util::StreamExt;
        let (_stream_id, mut stream) = pulse::subscribe_stream(Session::Admin, Duration::from_secs(1));
        let _open = stream.next().await.unwrap();
        pulse::poke(pulse::PokeTopic::TabsChanged);
        pulse::poke(pulse::PokeTopic::AdminSystem);
        let tabs = stream.next().await.unwrap().wire_text();
        let admin = stream.next().await.unwrap().wire_text();
        for marker in ["global.admin.pin", "ULTRA_SECRET_CONFIG_MARKER", "processes", "networkConnections", "stats-system-bootstrap-1"] {
            assert!(!tabs.contains(marker), "tabs poke leaked marker {marker}: {tabs}");
            assert!(!admin.contains(marker), "admin poke leaked marker {marker}: {admin}");
        }
        assert!(tabs.contains("data: {}"));
        assert!(admin.contains("data: {}"));
    }

    #[tokio::test]
    async fn pulse_wall_unrenewed_stream_expires_and_renewed_stream_survives_original_lease() {
        let _guard = pulse_test_lock().lock().await;
        use futures_util::StreamExt;
        let (expired_id, mut expired) = pulse::subscribe_stream(Session::Guest, Duration::from_millis(80));
        assert_eq!(expired.next().await.unwrap().event, "pulse.open");
        let expired_frame = tokio::time::timeout(Duration::from_millis(250), expired.next()).await.unwrap().unwrap();
        assert_eq!(expired_frame.event, "pulse.expired");
        assert_eq!(expired_frame.id.as_deref(), Some(expired_id.as_str()));
        assert!(expired.next().await.is_none());

        let (renewed_id, mut renewed) = pulse::subscribe_stream(Session::Guest, Duration::from_millis(80));
        assert_eq!(renewed.next().await.unwrap().event, "pulse.open");
        assert!(pulse::renew_stream(&renewed_id, Duration::from_millis(250)));
        let early = tokio::time::timeout(Duration::from_millis(120), renewed.next()).await;
        assert!(early.is_err(), "renewed stream expired at the original lease boundary");
        let renewed_expired = tokio::time::timeout(Duration::from_millis(250), renewed.next()).await.unwrap().unwrap();
        assert_eq!(renewed_expired.event, "pulse.expired");
    }

    #[tokio::test]
    async fn pulse_wall_renew_route_extends_known_stream_and_rejects_unknown_stream() {
        let _guard = pulse_test_lock().lock().await;
        let (stream_id, _stream) = pulse::subscribe_stream(Session::Guest, Duration::from_secs(1));
        let temp = test_tab_root("pulse-renew-route");
        let router = app(AppState { tab_root: Arc::new(temp) });
        let ok = router
            .clone()
            .oneshot(Request::builder().method("POST").uri(format!("/api/stats/events/renew?streamId={stream_id}")).body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(ok.status(), StatusCode::OK);
        let renew: LeaseRenewalReadback = serde_json::from_slice(&axum::body::to_bytes(ok.into_body(), usize::MAX).await.unwrap()).unwrap();
        assert_eq!(renew.schema, "coronatio.stats.events.renewal.v1");
        assert_eq!(renew.stream_id, stream_id);
        assert_eq!(renew.status, "renewed");

        let missing = router
            .oneshot(Request::builder().method("POST").uri("/api/stats/events/renew?streamId=missing-pulse-stream").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(missing.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn pulse_wall_renewal_during_parked_wait_extends_quiet_stream() {
        let _guard = pulse_test_lock().lock().await;
        use futures_util::StreamExt;

        let (stream_id, mut stream) = pulse::subscribe_stream(Session::Guest, Duration::from_millis(80));
        assert_eq!(stream.next().await.unwrap().event, "pulse.open");

        let renewal_id = stream_id.clone();
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(40)).await;
            assert!(pulse::renew_stream(&renewal_id, Duration::from_millis(170)));
        });

        let early = tokio::time::timeout(Duration::from_millis(130), stream.next()).await;
        assert!(early.is_err(), "quiet stream expired at the original lease boundary while parked in next_frame");

        let expired = tokio::time::timeout(Duration::from_millis(180), stream.next()).await.unwrap().unwrap();
        assert_eq!(expired.event, "pulse.expired");
        assert_eq!(expired.id.as_deref(), Some(stream_id.as_str()));
    }

    #[tokio::test]
    async fn pulse_wall_dropped_subscription_removes_lease_entry_without_expiry() {
        let _guard = pulse_test_lock().lock().await;
        use futures_util::StreamExt;

        let (stream_id, mut stream) = pulse::subscribe_stream(Session::Guest, Duration::from_secs(5));
        assert_eq!(stream.next().await.unwrap().event, "pulse.open");
        assert!(pulse::lease_exists_for_test(&stream_id));

        drop(stream);

        assert!(!pulse::lease_exists_for_test(&stream_id), "dropped stream orphaned its pulse lease entry");
    }

    #[tokio::test(flavor = "current_thread")]
    async fn pulse_002_wall_visibility_write_pokes_guest_and_admin_streams() {
        let _env_guard = HX_EXEMPLAR_ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
        let _guard = pulse_test_lock().lock().await;
        use futures_util::StreamExt;
        let temp = test_tab_root("pulse-002-route-poke");
        let config = temp.join("homeserver.json");
        vis_002_fixture_config(&config);
        std::env::set_var("CORONATIO_HOMESERVER_JSON", &config);
        let router = app(AppState { tab_root: Arc::new(test_tab_root("pulse-002-route-poke-app")) });
        let token = authorize_test_admin_token();
        let (_guest_id, mut guest) = pulse::subscribe_stream(Session::Guest, Duration::from_secs(1));
        let (_admin_id, mut admin) = pulse::subscribe_stream(Session::Admin, Duration::from_secs(1));
        assert_eq!(guest.next().await.unwrap().event, "pulse.open");
        assert_eq!(admin.next().await.unwrap().event, "pulse.open");

        let response = router
            .oneshot(Request::builder().method("POST").uri("/api/tabs/visibility").header("X-Admin-Token", token).header("content-type", "application/json").body(Body::from(r#"{"tab":"youtube","visible":true}"#)).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let guest_frame = tokio::time::timeout(Duration::from_millis(200), guest.next()).await.unwrap().unwrap();
        let admin_frame = tokio::time::timeout(Duration::from_millis(200), admin.next()).await.unwrap().unwrap();
        assert_eq!(guest_frame.event, "tabs.changed");
        assert_eq!(admin_frame.event, "tabs.changed");
        assert_eq!(guest_frame.data, "{}");
        assert_eq!(admin_frame.data, "{}");
        std::env::remove_var("CORONATIO_HOMESERVER_JSON");
    }

    #[tokio::test(flavor = "current_thread")]
    async fn pulse_002_wall_rejected_visibility_write_pokes_nothing() {
        let _env_guard = HX_EXEMPLAR_ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
        let _guard = pulse_test_lock().lock().await;
        use futures_util::StreamExt;
        let temp = test_tab_root("pulse-002-rejected-no-poke");
        let config = temp.join("homeserver.json");
        vis_002_fixture_config(&config);
        std::env::set_var("CORONATIO_HOMESERVER_JSON", &config);
        let router = app(AppState { tab_root: Arc::new(test_tab_root("pulse-002-rejected-no-poke-app")) });
        let (_stream_id, mut stream) = pulse::subscribe_stream(Session::Guest, Duration::from_secs(1));
        assert_eq!(stream.next().await.unwrap().event, "pulse.open");

        let response = router
            .oneshot(Request::builder().method("POST").uri("/api/tabs/visibility").header("content-type", "application/json").body(Body::from(r#"{"tab":"youtube","visible":true}"#)).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        let frame = tokio::time::timeout(Duration::from_millis(100), stream.next()).await;
        assert!(frame.is_err(), "unauthorized visibility write emitted a poke");
        std::env::remove_var("CORONATIO_HOMESERVER_JSON");
    }

    #[tokio::test(flavor = "current_thread")]
    async fn pulse_002_wall_guest_pull_after_admin_poke_stays_session_projected() {
        let _env_guard = HX_EXEMPLAR_ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
        let _guard = pulse_test_lock().lock().await;
        use futures_util::StreamExt;
        let temp = test_tab_root("pulse-002-guest-purity");
        let config = temp.join("homeserver.json");
        std::fs::write(&config, serde_json::json!({
            "global": { "admin": { "pin": "1234" }, "theme": { "name": "light" } },
            "tabs": {
                "starred": "stats",
                "admin": { "config": { "displayName": "ADMIN_ONLY_TAB_MARKER", "isEnabled": true, "adminOnly": true }, "visibility": { "tab": true, "elements": {} } },
                "stats": { "config": { "displayName": "Stats", "isEnabled": true, "adminOnly": false }, "visibility": { "tab": true, "elements": {} } },
                "portals": { "config": { "displayName": "Portals", "isEnabled": true, "adminOnly": false }, "visibility": { "tab": true, "elements": { "ADMIN_HIDDEN_ELEMENT_MARKER": false } } },
                "youtube": { "config": { "displayName": "YouTube Coherent", "isEnabled": true, "adminOnly": false }, "visibility": { "tab": false, "elements": {} } }
            }
        }).to_string()).unwrap();
        std::env::set_var("CORONATIO_HOMESERVER_JSON", &config);
        let router = app(AppState { tab_root: Arc::new(test_tab_root("pulse-002-guest-purity-app")) });
        let token = authorize_test_admin_token();
        let (_stream_id, mut guest_stream) = pulse::subscribe_stream(Session::Guest, Duration::from_secs(1));
        assert_eq!(guest_stream.next().await.unwrap().event, "pulse.open");

        let response = router
            .clone()
            .oneshot(Request::builder().method("POST").uri("/api/tabs/visibility").header("X-Admin-Token", token).header("content-type", "application/json").body(Body::from(r#"{"tab":"youtube","visible":true}"#)).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let poke = tokio::time::timeout(Duration::from_millis(200), guest_stream.next()).await.unwrap().unwrap();
        assert_eq!(poke.event, "tabs.changed");
        assert_eq!(poke.data, "{}");

        let guest = router.oneshot(Request::builder().uri("/api/tab-bar?active=youtube").body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(guest.status(), StatusCode::OK);
        let fragment = String::from_utf8(axum::body::to_bytes(guest.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        assert!(fragment.contains(r#"data-tab-id="youtube""#), "{fragment}");
        assert!(fragment.contains(r#"aria-selected="true" data-pane="youtube""#), "{fragment}");
        for forbidden in ["ADMIN_ONLY_TAB_MARKER", "ADMIN_HIDDEN_ELEMENT_MARKER", "data-admin-only=\"true\"", "data-visibility=\"hidden\""] {
            assert!(!fragment.contains(forbidden), "guest tab-bar leaked {forbidden}: {fragment}");
        }
        std::env::remove_var("CORONATIO_HOMESERVER_JSON");
    }

    #[test]
    fn pulse_002_wall_shell_rider_is_data_free_eventsource_pull_only() {
        let chrome = crown_chrome_js();
        assert!(chrome.contains("new EventSource('/api/stats/events')"));
        assert!(chrome.contains("pulseStream.addEventListener('pulse.open'"));
        assert!(chrome.contains("pulseStream.addEventListener('tabs.changed'"));
        assert!(chrome.contains("pulseStream.addEventListener('pulse.expired'"));
        assert!(chrome.contains("fetch(renewRoute, { method: 'POST', cache: 'no-store' })"));
        assert!(chrome.contains("}, 15000)"), "renewal cadence must stay comfortably before the 20s readback contract");
        assert!(chrome.contains("refreshTabBar(active)"));
        assert!(!chrome.contains("event.data.visibility"));
        assert!(!chrome.contains("event.data.tabId"));
    }
