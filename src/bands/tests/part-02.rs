    
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
        assert!(body.contains("font-mono"));
        assert!(body.contains("font-size-2xl"));
        assert!(body.contains("spacing-2xl"));
        assert!(body.contains("control-height"));
        assert!(body.contains("content-padding"));
        assert!(body.contains("card-radius"));
        assert!(body.contains("primaryHover"));
        assert!(body.contains("hiddenTabBackground"));
        assert!(body.contains("#A78BFA"), "dark accent must come from literal dark.json");
        assert!(body.contains("#323840"), "dark primary must come from literal dark.json, not green");
        assert!(body.contains("#6B7280"), "dark primaryHover must come from literal dark.json");
        assert!(body.contains("#F87171"), "dark statusDown must come from literal dark.json");
        assert!(body.contains("gradient-accent"));
        assert!(body.contains("highlight-strong"));
        assert!(body.contains("role-primary"));
        assert!(body.contains("component-button-container"));
        assert!(body.contains("flag-gradients"));
        assert!(body.contains("#A0AEC0"), "light primary must come from literal light.json");
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
        assert_eq!(list.native_panes.len(), 5);
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
        for section in ["cpu-chart", "network", "io-section", "memory", "disk-usage", "kea-leases", "process-usage"] {
            assert!(
                snapshot.doctrine.preserved_sections.contains(&section.to_string()),
                "missing React Stats preserved section {section}"
            );
        }
        assert!(!snapshot.doctrine.preserved_sections.contains(&"services".to_string()));
        assert_eq!(snapshot.doctrine.refresh_seconds, 5);
        assert!(!snapshot.storage.is_empty());
        assert!(!snapshot.io.devices.is_empty());
        assert!(snapshot.leases.len() <= 20);
        assert!(snapshot.processes.len() <= 10);
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
    fn shell_projects_expanded_theme_tokens_and_legacy_aliases() {
        let shell = ["src/bands/shell/document-1.rs", "src/bands/shell/document-2.rs", "src/bands/shell/document-3.rs"]
            .iter()
            .map(|path| std::fs::read_to_string(path).unwrap())
            .collect::<Vec<_>>()
            .join("\n");
        for marker in [
            "--theme-control-height",
            "--theme-content-padding",
            "--theme-card-radius",
            "--theme-font-mono",
            "--primary: var(--theme-primary)",
            "--theme-primary: #323840",
            "--primaryHover",
            "--hiddenTabBackground",
            "aliasMap",
            "--theme-gradient-accent",
            "--theme-highlight-strong",
            "--theme-role-primary",
            "--theme-component-button-container",
        ] {
            assert!(shell.contains(marker), "expanded theme marker missing: {marker}");
        }
    }


    #[test]
    fn ux_theme_system_docs_are_infinite_infinite_camel_case_band() {
        for path in [
            "docs/uxThemeSystem/index.json",
            "docs/uxThemeSystem/index.md",
            "docs/uxThemeSystem/observeMatureThemeSystems/index.json",
            "docs/uxThemeSystem/declareJsonTokenGrammar/index.json",
            "docs/uxThemeSystem/proveUxLibraryExpansion/index.json",
        ] {
            assert!(std::path::Path::new(path).exists(), "missing UX theme docs band path: {path}");
        }
        let index = std::fs::read_to_string("docs/uxThemeSystem/index.json").unwrap();
        assert!(index.contains("observeMatureThemeSystems"));
        assert!(index.contains("declareJsonTokenGrammar"));
        assert!(index.contains("proveUxLibraryExpansion"));
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
        assert!(shell.contains("data-crown-shell=\"maud\""));
        assert!(shell.contains("data-crown-underlay=\"fallback\""));
        assert!(shell.contains(CROWN_SHELL_SCRIPT_PATH));
        assert!(!shell.contains("fetch("));
    }

