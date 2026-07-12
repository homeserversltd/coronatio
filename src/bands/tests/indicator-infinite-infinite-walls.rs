    use futures_util::StreamExt;

    fn sixth_indicator(_: indicators::IndicatorRenderContext) -> String {
        r#"<button data-indicator="test-sixth">Sixth</button>"#.to_string()
    }

    fn sixth_modal(_: indicators::IndicatorRenderContext) -> String {
        r#"<div data-modal-kind-body="test-sixth">Sixth</div>"#.to_string()
    }

    #[test]
    fn indicator_spine_renders_five_shipped_bands_and_no_inline_shell_buttons() {
        let shell = render_crown_shell_for_session(Session::Admin);
        assert!(shell.contains(r#"data-indicator-spine="coronatio.indicators.v1""#));
        for id in ["tailscale", "internet", "openvpn", "services", "power-meter"] {
            assert!(shell.contains(&format!(r#"data-indicator="{id}""#)));
        }
        let document = std::fs::read_to_string("src/bands/shell/document-2.rs").unwrap();
        assert!(document.contains("__INDICATOR_SPINE__"));
        assert!(!document.contains("data-indicator=\""));
    }

    #[test]
    fn indicator_catalog_validates_identity_order_and_topic_walls() {
        let catalog = indicators::catalog();
        assert_eq!(catalog.len(), 5);
        assert!(indicators::validate_catalog(&catalog).is_ok());
        assert!(catalog.iter().all(|entry| !entry.title.is_empty() && !entry.icon_id.is_empty() && !entry.initial_state.is_empty()));
        assert_eq!(catalog.iter().filter(|entry| entry.admin_interactive).count(), 4);

        let mut duplicate_id = catalog.clone();
        duplicate_id[1].id = duplicate_id[0].id;
        assert!(indicators::validate_catalog(&duplicate_id).unwrap_err().contains("duplicate indicator id"));
        let mut duplicate_order = catalog.clone();
        duplicate_order[1].order = duplicate_order[0].order;
        assert!(indicators::validate_catalog(&duplicate_order).unwrap_err().contains("duplicate indicator order"));
        let mut unknown_topic = catalog.clone();
        unknown_topic[0].topic_id = "unknown.status";
        assert!(indicators::validate_catalog(&unknown_topic).unwrap_err().contains("unknown indicator topic"));
    }

    #[test]
    fn test_only_sixth_manifest_composes_without_shell_or_dispatcher_branch() {
        let mut catalog = indicators::catalog();
        catalog.push(indicators::IndicatorManifest {
            id: "test-sixth",
            topic_id: "services.status",
            order: 60,
            title: "Test Sixth",
            icon_id: "test",
            initial_state: "loading",
            admin_interactive: false,
            render_indicator: sixth_indicator,
            render_modal: sixth_modal,
            collector: None,
        });
        assert!(indicators::validate_catalog(&catalog).is_ok());
        let rendered = (catalog[5].render_indicator)(indicators::IndicatorRenderContext { session: Session::Guest });
        assert!(rendered.contains("test-sixth"));
        let shell_source = std::fs::read_to_string("src/bands/shell/document-2.rs").unwrap();
        let dispatcher_source = std::fs::read_to_string("src/bands/shell/document-3.rs").unwrap();
        assert!(!shell_source.contains("test-sixth"));
        assert!(!dispatcher_source.contains("test-sixth"));
    }

    #[tokio::test]
    async fn core_pulse_route_is_sse_and_emits_all_topic_snapshots() {
        let router = app(AppState { tab_root: Arc::new(test_tab_root("core-events")) });
        let response = router.oneshot(Request::builder().uri("/api/core/pulse").body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.headers().get(axum::http::header::CONTENT_TYPE).unwrap(), "text/event-stream");
        let mut body = response.into_body().into_data_stream();
        let first = tokio::time::timeout(Duration::from_secs(5), body.next()).await.unwrap().unwrap().unwrap();
        let wire = String::from_utf8_lossy(&first);
        assert!(wire.contains("core.open") || wire.contains("internet.status"));

        let (_id, mut stream) = indicators::subscribe_core_stream(Session::Guest, Duration::from_secs(2));
        assert_eq!(stream.next().await.unwrap().0, "core.open");
        let mut topics = std::collections::BTreeSet::new();
        for _ in 0..5 { topics.insert(stream.next().await.unwrap().0); }
        assert_eq!(topics, ["internet.status", "power.status", "services.status", "tailscale.status", "vpn.status"].into_iter().map(str::to_string).collect());
    }

    #[tokio::test]
    async fn core_events_renew_accepts_stream_id() {
        let (stream_id, _stream) = indicators::subscribe_core_stream(Session::Guest, Duration::from_secs(1));
        let router = app(AppState { tab_root: Arc::new(test_tab_root("core-renew")) });
        let response = router.oneshot(Request::builder().method("POST").uri(format!("/api/core/pulse/renew?streamId={stream_id}")).body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[test]
    fn crown_chrome_mounts_one_persistent_generic_core_eventsource() {
        let chrome = crown_chrome_js();
        assert_eq!(chrome.matches("new EventSource('/api/core/pulse')").count(), 1);
        assert!(chrome.contains("coreTopicIds.forEach"));
        assert!(!chrome.contains("showPane(id) {\n      if (coreStream) coreStream.close()"));
    }
