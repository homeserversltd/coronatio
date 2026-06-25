    #[test]
    fn admin_pane_stubs_original_flask_react_admin_button_inventory() {
        let shell = render_crown_shell();
        assert!(shell.contains(r#"data-admin-quarry="flask-react-admin""#));
        assert!(shell.contains(r#"data-admin-quarry-button-total="87""#));
        assert_eq!(shell.matches("data-admin-quarry-button").count(), 88);
        assert_eq!(shell.matches("data-admin-quarry-index=").count(), 87);
        for (group, count) in [
            ("system-controls", 7),
            ("disk-manager", 12),
            ("key-manager", 4),
            ("admin-password-modal", 2),
            ("create-key-modal", 2),
            ("hard-drive-test-modal", 6),
            ("log-viewer-modal", 6),
            ("password-input-modal", 3),
            ("premium-tab-modal", 16),
            ("root-ca-modal", 5),
            ("sync-schedule-modal", 2),
            ("system-action-modal", 1),
            ("update-key-modal", 2),
            ("update-manager-modal", 19),
        ] {
            assert!(shell.contains(&format!(r#"data-admin-quarry-group="{}""#, group)));
            assert!(shell.contains(&format!("{} buttons", count)));
        }
        for label in [
            "Hard Drive Test",
            "Restart Website",
            "Install Certificate",
            "Assign as primary NAS",
            "Auto Sync Schedule",
            "Create New Key",
            "View Full Guide &amp; Critical Warnings",
            "Validate &amp; Clone",
            "Force Update",
        ] {
            assert!(shell.contains(label), "missing admin quarry button label: {}", label);
        }
        assert!(shell.contains(r#"data-admin-visual-port="one-to-one-best-effort""#));
        assert!(shell.contains("system-controls-btn"));
        assert!(shell.contains("key-manager-content"));
        assert!(shell.contains("disk-manager-container"));
        assert!(shell.contains("modal-window update-manager-modal"));
        assert!(shell.contains("view-tabs"));
        assert!(shell.contains("modules-table"));
        assert!(shell.contains("data-stub-action=\"true\""));
        assert!(!shell.contains("WebSocket Subscriptions"));
        assert!(!shell.contains("debug-subscriptions"));
        assert!(!shell.contains("components/DebugSubscriptions.tsx"));
        assert!(!shell.contains("subscription-debug-panel"));
        assert!(!shell.contains("Front-end stubs mirror the original Flask/React admin-page button inventory from the quarry."));
    }

    #[tokio::test]
    async fn themes_route_reads_homeserver_json_theme_selection() {
        let temp = test_tab_root("homeserver-json-theme");
        let response = app(AppState {
            tab_root: Arc::new(temp),
        })
        .oneshot(Request::builder().uri("/api/themes").body(Body::empty()).unwrap())
        .await
        .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let body = String::from_utf8(bytes.to_vec()).unwrap();
        assert!(body.contains("coronatio.theme-catalog.response.v1"));
        assert!(body.contains("homeserver.json"));
        assert!(body.contains("global.theme.name"));
        assert!(!body.contains("static/themes/theme.json"));
        assert!(!body.contains("CORONATIO_THEME_JSON"));
        assert!(body.contains("radioactive"));
        assert!(body.contains("color-primary"));
        assert!(body.contains("bg-primary"));
        assert!(body.contains("font-family"));
    }

    #[tokio::test]
    async fn caduceus_routes_are_exposed_by_coronatio_api_root() {
        let temp = test_tab_root("caduceus-routes");
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
        assert!(body.contains("/api/caduceus/status"));
        assert!(body.contains("/api/caduceus/update/check"));
        assert!(body.contains("/api/caduceus/update/now"));
        assert!(body.contains("/api/caduceus/receipts/latest"));
    }

    #[tokio::test]
    async fn caduceus_update_now_acknowledges_self_restart_dispatch() {
        let temp = test_tab_root("caduceus-dispatch");
        let response = app(AppState {
            tab_root: Arc::new(temp),
        })
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/caduceus/update/now")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
        assert_eq!(response.status(), StatusCode::ACCEPTED);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let body = String::from_utf8(bytes.to_vec()).unwrap();
        assert!(body.contains("coronatio.caduceus.dispatch.v1"));
        assert!(body.contains("update_now"));
        assert!(body.contains("/api/v1/update/now"));
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
                .uri("/api/tabs")
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
        assert_eq!(snapshot.transport.stream_status, "available");
        assert!(snapshot.doctrine.preserved_sections.contains(&"resources".to_string()));
        assert!(snapshot.doctrine.preserved_sections.contains(&"storage".to_string()));
        assert!(snapshot.doctrine.preserved_sections.contains(&"network".to_string()));
        assert!(snapshot.doctrine.preserved_sections.contains(&"services".to_string()));
        assert_eq!(snapshot.doctrine.refresh_seconds, 5);
        assert!(!snapshot.storage.is_empty());
        assert!(!snapshot.io.devices.is_empty());
        assert!(snapshot.services.iter().any(|service| service.name == "Coronatio"));
        assert!(snapshot.telemetry.service_health.is_some());
        assert!(snapshot.telemetry.storage_posture.is_some());
    }

    #[test]
    fn stats_native_pane_points_to_stats_snapshot_route() {
        let stats = native_crown_panes()
            .into_iter()
            .find(|pane| pane.id == "stats")
            .unwrap();
        assert_eq!(stats.state_route, "/api/stats");
    }

    #[test]
    fn stats_viewport_preserves_resources_storage_network_services_and_stream_controls() {
        let shell = render_crown_shell();
        for marker in [
            r#"data-stats-viewport"#,
            r#"class="stats-section resources""#,
            r#"class="stats-section drives""#,
            r#"class="stats-section network""#,
            r#"class="stats-section services""#,
            r#"data-stats-connections"#,
            r#"/api/stats/events"#,
            r#"/api/stats/events/renew"#,
            r#"function fmtBytes(value)"#,
            r#"data.resources?.memory"#,
            r#"data-chart-dependency="chartjs-4.4.0""#,
            r#"data-chart-dependency="chartjs-plugin-datalabels-2.2.0""#,
            r#"<canvas id="cpuChart""#,
            r#"<canvas id="io-chart""#,
            r#"<canvas id="networkChart""#,
            r#"new Chart(ctx"#,
            r#"label: 'CPU Usage'"#,
            r#"label: 'Upload Speed'"#,
            r#"label: 'Download Speed'"#,
            r#"label: 'Temperature'"#,
            r#"data-io-drive-selector"#,
            r#"data-io-chart-legend"#,
            r#"label: `${device.mount} Read`"#,
            r#"label: `${device.mount} Write`"#,
            r#"setInterval(hydrateStats, 5000)"#,
        ] {
            assert!(shell.contains(marker), "stats viewport marker missing: {}", marker);
        }
        for placeholder in [
            r#"Stats stream state pending.</p><div class="button-row""#,
            r#"System telemetry</h2><div class="metric" id="stats-load">—</div><p>Load average</p>"#,
            r#"stats collectors not wired"#,
            r#"id="cpu-gauge""#,
            r#"id="memory-chart""#,
            r#"type: 'doughnut'"#,
        ] {
            assert!(!shell.contains(placeholder), "old stats scaffold survived: {}", placeholder);
        }
    }



    #[test]
    fn static_root_prefers_installed_source_and_allows_env_override() {
        std::env::remove_var("CORONATIO_STATIC_ROOT");
        let root = static_root();
        assert!(
            root == PathBuf::from(INSTALLED_STATIC_ROOT) || root == PathBuf::from(DEFAULT_STATIC_ROOT),
            "unexpected static root: {}",
            root.display()
        );
        std::env::set_var("CORONATIO_STATIC_ROOT", "/tmp/coronatio-static-test");
        assert_eq!(static_root(), PathBuf::from("/tmp/coronatio-static-test"));
        std::env::remove_var("CORONATIO_STATIC_ROOT");
    }

    #[tokio::test]
    async fn chartjs_dependency_is_served_as_first_party_static_asset() {
        let temp = test_tab_root("chartjs-static");
        let router = app(AppState {
            tab_root: Arc::new(temp),
        });
        let response = router
            .oneshot(
                Request::builder()
                    .uri("/static/vendor/chart.umd.min.js")
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
        assert!(body.contains("Chart.js"));
        assert!(body.contains("DoughnutController"));
    }



    #[test]
    fn docs_and_shell_do_not_advertise_sidecar_config_authority() {
        let shell = ["src/bands/shell.rs", "src/bands/shell/document-1.rs", "src/bands/shell/document-2.rs", "src/bands/shell/document-3.rs", "src/bands/shell/document-4.rs", "src/bands/shell/render.rs", "src/bands/shell/runtime.rs"]
            .iter()
            .map(|path| std::fs::read_to_string(path).unwrap())
            .collect::<Vec<_>>()
            .join("\n");
        let readme = std::fs::read_to_string("README.md").unwrap();
        let north_star = std::fs::read_to_string("docs/coronatio-north-star-contract.md").unwrap();
        let bands = std::fs::read_to_string("src/bands/README.md").unwrap();
        let theme_doc = std::fs::read_to_string("static/themes/README.md").unwrap();
        let favorites_doc = std::fs::read_to_string("static/favorites/README.md").unwrap();
        for (name, text) in [
            ("shell", shell),
            ("readme", readme),
            ("north_star", north_star),
            ("bands", bands),
            ("theme_doc", theme_doc),
            ("favorites_doc", favorites_doc),
        ] {
            assert!(text.contains("homeserver.json"), "{name} must name homeserver.json authority");
            assert!(text.contains("one-to-one port"), "{name} must name the one-to-one port doctrine");
            assert!(
                text.contains("before any Coronatio-local fallback")
                    || text.contains("before any Coronatio local fallback")
                    || text.contains("before any Coronatio-local fallback or firmware default"),
                "{name} must name homeserver.json before local fallback authority"
            );
            assert!(!text.contains("static/themes/theme.json"), "{name} advertises obsolete theme sidecar");
            assert!(!text.contains("static/favorites/favorites.json"), "{name} advertises obsolete favorites sidecar");
            assert!(!text.contains("CORONATIO_THEME_JSON"), "{name} advertises obsolete theme env sidecar");
            assert!(!text.contains("CORONATIO_FAVORITES_JSON"), "{name} advertises obsolete favorites env sidecar");
        }
    }

    #[test]
    fn coronatio_config_authority_is_single_homeserver_json_not_sidecar_jsons() {
        let mut contracts = std::fs::read_to_string("src/bands/contracts.rs").unwrap();
        for child in std::fs::read_dir("src/bands/contracts").unwrap() {
            let path = child.unwrap().path();
            if path.extension().is_some_and(|extension| extension == "rs") {
                contracts.push_str(&std::fs::read_to_string(path).unwrap());
            }
        }
        let routes = std::fs::read_to_string("src/bands/routes.rs").unwrap();
        assert!(contracts.contains("INSTALLED_HOMESERVER_JSON"));
        assert!(contracts.contains("LEGACY_HOMESERVER_JSON"));
        assert!(routes.contains("fn homeserver_json_path()"));
        assert!(routes.contains("homeserver.json tabs.{config,visibility,starred}"));
        assert!(routes.contains("global.theme.name"));
        for obsolete in [
            "DEFAULT_THEME_JSON",
            "INSTALLED_THEME_JSON",
            "DEFAULT_FAVORITES_JSON",
            "INSTALLED_FAVORITES_JSON",
            "CORONATIO_THEME_JSON",
            "CORONATIO_FAVORITES_JSON",
            "theme_catalog_path()",
            "favorite_manifest_path()",
        ] {
            assert!(!contracts.contains(obsolete), "obsolete config authority survived in contracts: {obsolete}");
            assert!(!routes.contains(obsolete), "obsolete config authority survived in routes: {obsolete}");
        }
    }

    #[tokio::test]
    async fn favorite_manifest_drives_original_first_load_starred_tab() {
        let temp = test_tab_root("favorite-manifest");
        let app = app(AppState { tab_root: Arc::new(temp) });
        let response = app.clone().oneshot(Request::builder().uri("/api/favorites").body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body = String::from_utf8(bytes.to_vec()).unwrap();
        assert!(body.contains("coronatio.favorite-manifest.response.v1"));
        assert!(body.contains("\"starredTab\":\"portals\""));
        assert!(body.contains("homeserver.json"));
        assert!(body.contains("tabs.{config,visibility,starred}"));
        assert!(!body.contains("static/favorites"));
        assert!(!body.contains("CORONATIO_FAVORITES_JSON"));
        assert!(body.contains("get_starred_tab() or get_first_visible_tab()"));
        let response = app.clone().oneshot(Request::builder().uri("/api/get_starred_tab").body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body = String::from_utf8(bytes.to_vec()).unwrap();
        assert!(body.contains("coronatio.starred-tab.response.v1"));
        assert!(body.contains("\"starred_tab\":\"portals\""));
        let response = app.oneshot(Request::builder().uri("/").body(Body::empty()).unwrap()).await.unwrap();
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let shell = String::from_utf8(bytes.to_vec()).unwrap();
        assert!(shell.contains("const tabState = Object.assign({ starredTab: 'upload'"));
        assert!(shell.contains("fetch('/api/favorites')"));
        assert!(shell.contains("fetch('/api/set_starred_tab'"));
        assert!(shell.contains("Upload tab is starred"));
    }

