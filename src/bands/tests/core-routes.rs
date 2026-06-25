    #[test]
    fn tab_ids_are_forward_safe() {
        for tab_id in PRIMARY_TABS {
            assert!(is_safe_tab_id(tab_id));
        }
        assert!(is_safe_tab_id("backblaze-tab"));
        assert!(!is_safe_tab_id("../escape"));
        assert!(!is_safe_tab_id("CamelCase"));
        assert!(!is_safe_tab_id(""));
    }

    #[test]
    fn native_panes_are_lawful_crown_tabs() {
        let panes = native_crown_panes();
        let ids: Vec<_> = panes.iter().map(|pane| pane.id.as_str()).collect();
        assert_eq!(ids, PRIMARY_TABS);
        assert!(panes
            .iter()
            .all(|pane| pane.install_mode == InstallMode::FirstPartyNative));
        assert!(panes
            .iter()
            .any(|pane| pane.admin_only && pane.id == "admin"));
    }

    #[tokio::test]
    async fn api_root_names_coronatio_not_arcadia() {
        let temp = test_tab_root("api-root");
        let response = app(AppState {
            tab_root: Arc::new(temp),
        })
        .oneshot(Request::builder().uri("/api").body(Body::empty()).unwrap())
        .await
        .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let body = String::from_utf8(bytes.to_vec()).unwrap();
        assert!(body.contains("coronatio.api.root.v1"));
        assert!(body.contains("Coronatio"));
        assert!(!body.contains("Arcadia"));
    }

    #[tokio::test]
    async fn api_root_declares_lawful_primary_tabs() {
        let temp = test_tab_root("primary-tabs");
        let response = app(AppState {
            tab_root: Arc::new(temp),
        })
        .oneshot(Request::builder().uri("/api").body(Body::empty()).unwrap())
        .await
        .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let root: CoronatioRoot = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(root.primary_tabs, ["admin", "stats", "portals", "upload"]);
        assert_eq!(root.first_party_panes.len(), 4);
    }

    #[tokio::test]
    async fn panes_route_exposes_first_party_crown_shell() {
        let temp = test_tab_root("panes");
        let response = app(AppState {
            tab_root: Arc::new(temp),
        })
        .oneshot(
            Request::builder()
                .uri("/api/panes")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let body = String::from_utf8(bytes.to_vec()).unwrap();
        assert!(body.contains("coronatio.panes.v1"));
        assert!(body.contains("first-party-native"));
        assert!(body.contains("Admin"));
        assert!(body.contains("Stats"));
        assert!(body.contains("Portals"));
        assert!(body.contains("Upload"));
        assert!(!body.contains("YouTube"));
    }

    #[tokio::test]
    async fn crown_shell_renders_primary_tabs_without_platform_brand_nav() {
        let temp = test_tab_root("shell");
        let response = app(AppState {
            tab_root: Arc::new(temp),
        })
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let body = String::from_utf8(bytes.to_vec()).unwrap();
        assert!(body.contains("<title>HomeServer</title>"));
        assert!(body.contains("HomeServer Admin Interface"));
        assert!(body.contains("/assets/index-BRoXzIjg.js"));
        assert!(body.contains("/assets/index-Co-PYpJ8.css"));
        assert!(body.contains("data-coronatio-identical-socket-bridge=\"home-arpa\""));
        assert!(body.contains("wss://home.arpa"));
        assert!(body.contains("<div id=\"root\"></div>"));
        assert!(!body.contains("Coronatio crown shell"));
        assert!(!body.contains("data-source-material=\"homeserver-main-site\""));
        assert!(!body.contains("class=\"tab-bar\""));
        assert!(!body.contains("Admitted services"));
        assert!(!body.contains("Safe file ingress"));
        assert!(!body.contains("Arcadia"));
    }

    #[test]
    fn native_pane_bodies_are_not_placeholder_cards() {
        let shell = render_crown_shell();
        assert!(shell.contains("<title>HomeServer</title>"));
        assert!(shell.contains("/assets/index-BRoXzIjg.js"));
        assert!(shell.contains("/assets/index-Co-PYpJ8.css"));
        assert!(shell.contains("<div id=\"root\"></div>"));
        assert!(!shell.contains("Admin authority"));
        assert!(!shell.contains("System telemetry"));
        assert!(!shell.contains("Coronatio crown shell"));
    }

    #[test]
    fn crown_shell_sets_first_paint_theme_before_bundle_loads() {
        let shell = render_crown_shell();
        assert!(shell.contains("<meta name=\"theme-color\" content=\"#"));
        assert!(shell.contains("--background:"));
        assert!(shell.contains("--primary:"));
        assert!(shell.contains("--hiddenTabBackground:"));
        assert!(!shell.contains("content=\"#000000\""));
        assert!(
            shell.find("--background:").unwrap() < shell.find("/assets/index-BRoXzIjg.js").unwrap()
        );
    }

    #[test]
    fn homeserver_config_pin_reader_uses_global_admin_pin() {
        let temp = test_tab_root("homeserver-config-pin");
        let config_path = temp.join("homeserver.json");
        std::fs::write(
            &config_path,
            r#"{"global":{"admin":{"pin":"2468"},"theme":{"name":"light"}}}"#,
        )
        .unwrap();
        std::env::set_var("CORONATIO_HOMESERVER_CONFIG", &config_path);
        assert_eq!(homeserver_admin_pin().unwrap(), "2468");
        std::env::remove_var("CORONATIO_HOMESERVER_CONFIG");
    }

    #[tokio::test]
    async fn validate_pin_route_reads_homeserver_config() {
        let temp = test_tab_root("validate-pin");
        let config_path = temp.join("homeserver.json");
        std::fs::write(
            &config_path,
            r#"{"global":{"admin":{"pin":"2468"},"theme":{"name":"dark"}}}"#,
        )
        .unwrap();
        std::env::set_var("CORONATIO_HOMESERVER_CONFIG", &config_path);
        let response = app(AppState {
            tab_root: Arc::new(temp),
        })
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/validatePin")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"pin":"2468"}"#))
                .unwrap(),
        )
        .await
        .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(body["success"], true);
        assert_eq!(body["sessionTimeout"], ADMIN_SESSION_TIMEOUT_SECONDS);
        assert!(body["token"].as_str().unwrap().starts_with("coronatio-"));
        std::env::remove_var("CORONATIO_HOMESERVER_CONFIG");
    }

    #[tokio::test]
    async fn loads_dynamic_cartridge_manifests_without_recompile() {
        let temp = test_tab_root("dynamic-tabs");
        let tab_dir = temp.join("service-card");
        std::fs::create_dir_all(&tab_dir).unwrap();
        std::fs::write(
            tab_dir.join("tab.json"),
            r#"{
              "id":"service-card",
              "title":"Service Card",
              "order":90,
              "adminOnly":true,
              "routePrefix":"/api/tabs/service-card",
              "staticDir":"static",
              "serviceUrl":"http://127.0.0.1:9910",
              "healthRoute":"/health",
              "installMode":"dynamic-cartridge"
            }"#,
        )
        .unwrap();

        let response = app(AppState {
            tab_root: Arc::new(temp),
        })
        .oneshot(
            Request::builder()
                .uri("/api/coronatio/tabs")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let list: TabList = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(list.native_panes.len(), 4);
        assert_eq!(list.tabs.len(), 1);
        assert_eq!(list.tabs[0].id, "service-card");
        assert_eq!(list.tabs[0].install_mode, InstallMode::DynamicCartridge);
    }

    #[tokio::test]
    async fn stats_snapshot_is_honest_first_party_readback() {
        let temp = test_tab_root("stats-snapshot");
        let response = app(AppState {
            tab_root: Arc::new(temp),
        })
        .oneshot(
            Request::builder()
                .uri("/api/stats")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let snapshot: StatsSnapshot = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(snapshot.schema, "coronatio.stats.snapshot.v1");
        assert_eq!(snapshot.pane_id, "stats");
        assert_eq!(snapshot.product, "Coronatio");
        assert_eq!(snapshot.transport.snapshot_route, "/api/stats");
        assert_eq!(snapshot.transport.event_route, "/api/stats/events");
        assert_eq!(snapshot.transport.renew_route, "/api/stats/events/renew");
        assert_eq!(snapshot.transport.stream_status, "planned");
        assert_eq!(snapshot.telemetry.load1, None);
        assert_eq!(snapshot.telemetry.cpu_temperature_celsius, None);
        assert_eq!(
            snapshot.telemetry.first_missing_signal,
            "stats collectors not wired"
        );
    }

    #[test]
    fn stats_native_pane_points_to_stats_snapshot_route() {
        let stats = native_crown_panes()
            .into_iter()
            .find(|pane| pane.id == "stats")
            .unwrap();
        assert_eq!(stats.state_route, "/api/stats");
    }

