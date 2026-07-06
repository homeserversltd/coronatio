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
