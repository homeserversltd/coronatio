mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

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

    #[test]
    fn docs_inscribe_one_to_one_port_doctrine() {
        let readme = std::fs::read_to_string("README.md").unwrap();
        let north_star = std::fs::read_to_string("docs/coronatio-north-star-contract.md").unwrap();
        let bands = std::fs::read_to_string("src/bands/README.md").unwrap();
        for doc in [&readme, &north_star, &bands] {
            assert!(doc.contains("one-to-one port"));
            assert!(doc.contains("not a reinterpretation, redesign, summary, scaffold, or inspired-by rebuild"));
            assert!(doc.contains("directly queries the original Flask/React source and live quarry"));
            assert!(doc.contains("indistinguishable to the user under the same viewport, theme, session/admin state, configuration, and data state"));
        }
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
        assert!(body.contains("data-product=\"Coronatio\""));
        assert!(body.contains("data-source-material=\"homeserver-main-site\""));
        assert!(body.contains("class=\"tab-bar\""));
        assert!(body.contains("role=\"tablist\""));
        assert!(body.contains("data-pane=\"admin\""));
        assert!(body.contains("data-pane=\"stats\""));
        assert!(body.contains("data-pane=\"portals\""));
        assert!(body.contains("data-pane=\"upload\""));
        assert!(body.contains("data-pane-panel=\"admin\""));
        assert!(body.contains("data-pane-panel=\"stats\""));
        assert!(body.contains("data-pane-panel=\"portals\""));
        assert!(body.contains("data-pane-panel=\"upload\""));
        assert!(body.contains("function showPane(id)"));
        assert!(body.contains("fetch('/api/stats')"));
        assert!(body.contains(r#"data-admin-quarry-button-total="90""#));
        assert!(body.contains("Hard Drive Test"));
        assert!(body.contains("Force Update"));
        assert!(body.contains("HomeServer"));
        assert!(body.contains("Admitted services"));
        assert!(body.contains("Safe file ingress"));
        assert!(!body.contains("Coronatio crown shell"));
        assert!(!body.contains("class=\"crown-card\""));
        assert!(!body.contains("Arcadia"));
        assert!(!body.contains("YouTube"));
    }

    #[test]
    fn native_pane_bodies_are_not_placeholder_cards() {
        let shell = render_crown_shell();
        for pane in PRIMARY_TABS {
            assert!(shell.contains(&format!("data-pane-panel=\"{}\"", pane)));
            assert!(shell.contains(&format!("data-tab-id=\"{}\"", pane)));
        }
        assert!(shell.contains("Read service contract"));
        assert!(shell.contains("data-stats-viewport"));
        assert!(shell.contains("data-stats-drives"));
        assert!(shell.contains("data-stats-network"));
        assert!(shell.contains("data-stats-services"));
        assert!(shell.contains("Fetching /api/stats"));
        assert!(shell.contains(r#"data-admin-quarry="flask-react-admin""#));
        assert!(shell.contains(r#"data-admin-quarry-button-total="90""#));
        assert!(shell.contains("Upload through Caduceus"));
        assert!(!shell.contains("First-party panes are native Rust crown law. Installed services enter through governed cartridges or source-injection recompiles."));
    }



    #[test]
    fn normal_mode_keeps_primary_tabs_visible_and_admin_only_enhances_controls() {
        let shell = render_crown_shell();
        for pane in ["stats", "portals", "upload"] {
            let marker = format!(r#"data-tab-id="{}""#, pane);
            let start = shell.find(&marker).expect("normal tab marker present");
            let tab_start = shell[..start]
                .rfind("<div class=\"tab")
                .expect("tab starts before marker");
            let tag_end = shell[start..]
                .find('>')
                .map(|n| start + n)
                .expect("tab opening tag closes after marker");
            let opening_tag = &shell[tab_start..tag_end];
            assert!(
                !opening_tag.contains("data-admin-only"),
                "{pane} tab element itself must be visible outside admin mode"
            );
            assert!(
                !opening_tag.contains("hidden"),
                "{pane} tab element must not start hidden outside admin mode"
            );
            assert!(
                shell.contains(&format!(r#"data-tab-star="{}""#, pane)),
                "{pane} keeps normal star/default control"
            );
        }
        assert!(shell.contains(r#"data-tab-id="admin" data-visibility="visible" data-admin-only="true""#));
        for pane in ["stats", "portals", "upload"] {
            assert!(shell.contains(&format!(r#"data-admin-only="true" data-tab-visibility-toggle="{}""#, pane)), "{pane} eye control is admin enhancement");
        }
        assert!(shell.contains(r#"[data-admin-mode="false"] [data-admin-only]:not([data-admin-only="false"])"#));
        assert!(shell.contains(r#"querySelectorAll('[data-admin-only]:not([data-admin-only="false"])')"#));
    }

    #[test]
    fn crown_tabbar_recreates_flask_react_star_eye_and_hide_controls() {
        let shell = render_crown_shell();
        assert!(shell.contains("class=\"tab-bar\""));
        assert!(shell.contains("data-admin-mode=\"true\""));
        for pane in ["admin", "stats", "portals", "upload"] {
            assert!(shell.contains("class=\"tab active\""));
            assert!(shell.contains(&format!("data-tab-id=\"{}\"", pane)));
            assert!(shell.contains(&format!("data-pane=\"{}\"", pane)));
        }
        for pane in ["stats", "portals", "upload"] {
            assert!(shell.contains(&format!("data-tab-visibility-toggle=\"{}\"", pane)));
            assert!(shell.contains(&format!("data-tab-star=\"{}\"", pane)));
        }
        assert!(shell.contains("class=\"visibility-toggle\""));
        assert!(shell.contains("class=\"star-button fas fa-star\""));
        assert!(shell.contains("class=\"star-button far fa-star\""));
        assert!(shell.contains("data-visibility=\"visible\""));
        assert!(shell.contains("hiddenTabs"));
        assert!(shell.contains("firstVisibleTab()"));
        assert!(shell.contains("setStarredTab"));
        assert!(shell.contains("applyVisibilityState"));
        assert!(shell.contains("🙈"));
        assert!(!shell.contains("class=\"tab-button\""));
    }

    #[test]
    fn crown_header_recreates_flask_react_indicators_pin_and_theme_controls() {
        let shell = render_crown_shell();
        assert!(shell.contains(r#"data-flask-react-quarry="Header""#));
        assert!(shell.contains(r#"class="status-indicators""#));
        for indicator in ["tailscale", "internet", "openvpn", "services", "power-meter"] {
            assert!(shell.contains(&format!(r#"data-indicator="{}""#, indicator)));
        }
        assert!(shell.contains(r#"class="indicator ok tailscale-indicator""#));
        assert!(shell.contains(r#"data-packed-icon="network-wired""#));
        assert!(shell.contains(r#"data-modal-title="Tailscale Status""#));
        assert!(shell.contains(r#"data-modal-kind="tailscale""#));
        assert!(shell.contains("data-info-modal-backdrop"));
        assert!(shell.contains(".theme-choice-row[hidden]"));
        assert!(shell.contains("modalTemplate(kind)"));
        assert!(shell.contains("wireModalFetches"));
        assert!(shell.contains("openInfoModal"));
        assert!(shell.contains("document.querySelectorAll('[data-indicator]')"));
        assert!(shell.contains("data-uptime-indicator"));
        assert!(shell.contains(r#"class="theme-button""#));
        assert!(shell.contains("data-theme-button"));
        assert!(shell.contains("Click to switch theme"));
        assert!(!shell.contains("Open theme selector"));
        assert!(shell.contains(r#"data-theme-json-source="/api/themes""#));
        assert!(shell.contains("loadThemeCatalog()"));
        assert!(shell.contains("fetch('/api/themes')"));
        assert!(!shell.contains(r#"data-theme-choice="blue""#));
        assert!(shell.contains("Current theme:"));
        assert!(shell.contains(r#"class="admin-button""#));
        assert!(shell.contains("Enter Admin Mode"));
        assert!(shell.contains("Exit Admin Mode"));
        assert!(shell.contains(r#"class="change-admin-pin-button""#));
        assert!(shell.contains("Change PIN"));
        assert!(shell.contains(r#"id="pin-modal-title""#));
        assert!(shell.contains("Enter Admin Mode"));
        assert!(shell.contains("Change Admin PIN"));
        assert!(shell.contains(r#"placeholder="Enter PIN""#));
        assert!(shell.contains(r#"placeholder="Current PIN""#));
        assert!(shell.contains(r#"placeholder="New PIN""#));
        assert!(shell.contains(r#"placeholder="Confirm new PIN""#));
        assert!(shell.contains("coronatio.flask-react-header.v1"));
        assert!(shell.contains("setAdminMode"));
        assert!(shell.contains("applyTheme"));
        for preserved in [
            "Tailscale Status",
            "Current Tailnet:",
            "Enter Tailnet name",
            "Update Tailnet",
            "Authenticate",
            "/api/status/tailscale/connect",
            "/api/status/tailscale/authkey",
            "Internet Status",
            "Run Speed Test",
            "/api/status/internet/speedtest",
            "Services Status",
            "service-status-list",
            "/api/status/services",
            "VPN & Transmission Configuration",
            "VPN Status:",
            "Transmission Status:",
            "PIA Username",
            "Create PIA Key",
            "Enable Transmission over PIA VPN",
            "/api/status/vpn/updatekey/pia",
            "Power Consumption",
            "power-meter-modal",
            "5s average:",
            "30s average:",
            "60s average:",
        ] {
            assert!(shell.contains(preserved), "original header feature missing: {}", preserved);
        }
        for invented in [
            "Rust crown online",
            "Caduceus boundary protected",
            "Source quarry: main HomeServer",
            "Live Rust header control",
            "Detailed backend wiring continues through Coronatio/Caduceus routes",
            "Power meter readback, energy telemetry, and device availability",
            "Choose the active HOMESERVER theme.",
        ] {
            assert!(!shell.contains(invented), "invented visible prose survived: {}", invented);
        }
    }

    #[test]
    fn crown_theme_system_uses_legacy_react_variable_membrane_across_panes() {
        let shell = render_crown_shell();
        for preserved in [
            "--theme-color-primary",
            "--theme-bg-primary",
            "--theme-bg-secondary",
            "--theme-text-primary",
            "--theme-status-success",
            "--theme-spacing-md",
            "--theme-transition-fast",
            "--theme-shadow-md",
            "style[data-theme-styles]",
            "themeToCss(theme)",
            "themeCatalog",
            "preferred-theme",
            "themeData",
            "/api/themes",
            "data-theme-json-source",
        ] {
            assert!(shell.contains(preserved), "theme membrane marker missing: {}", preserved);
        }
        for pane in ["admin", "stats", "portals", "upload"] {
            assert!(shell.contains(&format!(r#"data-pane-panel="{}""#, pane)));
        }
        assert!(shell.contains("document.documentElement.dataset.theme = headerState.theme"));
        assert!(shell.contains("aria-pressed"));
        assert!(!shell.contains("const themeCatalog = {"));
        assert!(!shell.contains(r#":root[data-theme="light"]"#));
        assert!(!shell.contains("Choose the active HOMESERVER theme."));
    }

    #[test]
    fn admin_pane_stubs_original_flask_react_admin_button_inventory() {
        let shell = render_crown_shell();
        assert!(shell.contains(r#"data-admin-quarry="flask-react-admin""#));
        assert!(shell.contains(r#"data-admin-quarry-button-total="90""#));
        assert_eq!(shell.matches("data-admin-quarry-button").count(), 91);
        assert_eq!(shell.matches("data-admin-quarry-index=").count(), 90);
        for (group, count) in [
            ("system-controls", 7),
            ("disk-manager", 12),
            ("key-manager", 4),
            ("debug-subscriptions", 3),
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
            "View Full Guide & Critical Warnings",
            "Validate & Clone",
            "Force Update",
        ] {
            assert!(shell.contains(label), "missing admin quarry button label: {}", label);
        }
        assert!(shell.contains("Buttons are intentionally disabled until their Rust/Caduceus handlers are wired."));    }

    #[tokio::test]
    async fn themes_route_reads_runtime_theme_json_catalog() {
        let temp = test_tab_root("theme-json-catalog");
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
        assert!(body.contains("static/themes/theme.json"));
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
            r#"<canvas id="cpu-gauge""#,
            r#"<canvas id="memory-chart""#,
            r#"<canvas id="network-chart""#,
            r#"new Chart(ctx"#,
            r#"type: 'doughnut'"#,
            r#"label: 'Download'"#,
            r#"setInterval(hydrateStats, 5000)"#,
        ] {
            assert!(shell.contains(marker), "stats viewport marker missing: {}", marker);
        }
        for placeholder in [
            r#"Stats stream state pending.</p><div class="button-row""#,
            r#"System telemetry</h2><div class="metric" id="stats-load">—</div><p>Load average</p>"#,
            r#"stats collectors not wired"#,
        ] {
            assert!(!shell.contains(placeholder), "old stats scaffold survived: {}", placeholder);
        }
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

    #[tokio::test]
    async fn registry_route_encodes_tab_visibility_and_starred_law() {
        let temp = test_tab_root("registry-law");
        let response = app(AppState {
            tab_root: Arc::new(temp),
        })
        .oneshot(
            Request::builder()
                .uri("/api/registry")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let registry: RegistryReadback = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(registry.schema, "coronatio.registry.v1");
        assert_eq!(registry.starred_tab, "portals");
        assert_eq!(registry.default_route_tab, "portals");
        assert_eq!(registry.visible_tabs_user, ["stats", "portals", "upload"]);
        assert_eq!(
            registry.visible_tabs_admin,
            ["admin", "stats", "portals", "upload"]
        );
        assert!(registry
            .validation_rules
            .iter()
            .any(|rule| rule.field == "starred"));
    }

    #[tokio::test]
    async fn startup_route_encodes_initial_tab_and_fallback_law() {
        let temp = test_tab_root("startup-law");
        let response = app(AppState {
            tab_root: Arc::new(temp),
        })
        .oneshot(
            Request::builder()
                .uri("/api/startup")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let startup: StartupReadback = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(startup.schema, "coronatio.startup.v1");
        assert_eq!(startup.initial_tab, "portals");
        assert_eq!(initial_tab(false, None, false), "fallback");
        assert_eq!(initial_tab(true, Some("@stats"), false), "stats");
        assert!(startup.default_route_law.contains("forced tab wins"));
    }

    #[tokio::test]
    async fn lane_policy_route_decides_dynamic_source_and_native_failures() {
        let temp = test_tab_root("lane-policy");
        let response = app(AppState {
            tab_root: Arc::new(temp),
        })
        .oneshot(
            Request::builder()
                .uri("/api/lanes")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let lanes: LanePolicyReadback = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(lanes.schema, "coronatio.lane-policy.v1");
        assert_eq!(lanes.policies.len(), 3);
        assert!(lanes.policies.iter().any(|policy| policy.install_mode
            == InstallMode::DynamicCartridge
            && policy.failure_contract.contains("tab-local error")));
        assert!(lanes.policies.iter().any(|policy| policy.install_mode
            == InstallMode::SourceInjectionRecompile
            && policy.success_contract.contains("Cibation admits")));
        assert!(lanes.policies.iter().any(|policy| policy.install_mode
            == InstallMode::FirstPartyNative
            && policy.failure_contract.contains("build/test failure")));
    }

    #[tokio::test]
    async fn fallback_route_encodes_safe_pane_and_recovery_receipt() {
        let temp = test_tab_root("fallback-law");
        let response = app(AppState {
            tab_root: Arc::new(temp),
        })
        .oneshot(
            Request::builder()
                .uri("/api/fallback")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let fallback: FallbackReadback = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(fallback.schema, "coronatio.fallback.v1");
        assert_eq!(fallback.safe_pane, "fallback");
        assert!(fallback
            .activation_reasons
            .contains(&"no_visible_tabs".to_string()));
        assert!(fallback
            .activation_reasons
            .contains(&"module_load_error".to_string()));
        assert!(fallback.receipt_fields.contains(&"selectedTab".to_string()));
    }

    #[test]
    fn cartridge_manifest_validation_rejects_unsafe_and_native_shapes() {
        let mut manifest = TabManifest {
            id: "service-card".to_string(),
            title: "Service Card".to_string(),
            description: String::new(),
            icon: String::new(),
            display_name: String::new(),
            order: 9,
            enabled: true,
            admin_only: false,
            visibility: TabVisibility::default(),
            data: serde_json::Value::Null,
            route_prefix: "/api/tabs/service-card".to_string(),
            static_dir: "static".to_string(),
            service_url: None,
            health_route: None,
            install_mode: InstallMode::DynamicCartridge,
        };
        assert!(validate_tab_manifest(&manifest).is_ok());
        manifest.route_prefix = "/wrong".to_string();
        assert!(validate_tab_manifest(&manifest)
            .unwrap_err()
            .contains("routePrefix"));
        manifest.route_prefix = "/api/tabs/service-card".to_string();
        manifest.install_mode = InstallMode::FirstPartyNative;
        assert!(validate_tab_manifest(&manifest)
            .unwrap_err()
            .contains("compiled crown law"));
    }






    #[test]
    fn all_modals_close_on_backdrop_outside_click_only() {
        let shell = render_crown_shell();
        assert!(shell.contains(r#"data-pin-modal-backdrop"#));
        assert!(shell.contains(r#"data-info-modal-backdrop"#));
        assert!(shell.contains("function closeModalOnOutsideClick(backdrop, closeModal)"));
        assert!(shell.contains("event.target === event.currentTarget"));
        assert!(shell.contains("closeModalOnOutsideClick(modalBackdrop, closePinModal)"));
        assert!(shell.contains("closeModalOnOutsideClick(infoBackdrop, closeInfoModal)"));
        assert!(shell.contains("document.querySelector('[data-info-modal-close]')?.addEventListener('click', closeInfoModal)"));
        assert!(shell.contains("document.querySelector('[data-pin-cancel]')?.addEventListener('click', closePinModal)"));
    }

    #[test]
    fn indicator_modals_gate_admin_enhancements_from_regular_mode() {
        let shell = render_crown_shell();
        assert!(shell.contains("function indicatorAdminSection(inner)"));
        assert!(shell.contains(r#"data-admin-surface="indicator-modal""#));
        assert!(shell.contains(r#"headerState.isAdmin ? `<div class="status-item" data-admin-only"#));
        assert!(shell.contains("!headerState.isAdmin && button.closest('[data-admin-only]')"));
        for admin_action in ["Update Tailnet", "Authenticate", "Run Speed Test", "Create PIA Key", "Create Transmission", "Enable Transmission over PIA VPN", "PIA Key Exists", "Service Data"] {
            assert!(shell.contains(admin_action), "missing gated admin action {admin_action}");
        }
    }

    #[test]
    fn power_indicator_modal_has_no_invented_admin_refresh_control() {
        let shell = render_crown_shell();
        let power_start = shell.find("if (kind === 'power-meter')").unwrap();
        let power_end = shell[power_start..].find("if (kind === 'theme')").unwrap() + power_start;
        let power = &shell[power_start..power_end];
        assert!(!power.contains("data-modal-fetch"));
        assert!(!power.contains("Refresh"));
        assert!(!power.contains("data-admin-only"));
    }

    #[test]
    fn header_status_indicators_are_packed_react_icon_port_not_text_pills() {
        let shell = render_crown_shell();
        for icon in ["network-wired", "plug", "lock", "server", "bolt"] {
            assert!(shell.contains(&format!(r#"data-packed-icon="{}""#, icon)), "missing icon {icon}");
        }
        assert!(shell.contains("<svg"));
        assert!(shell.contains("<path"));
        assert!(!shell.contains("Font Awesome"));
        assert!(!shell.contains("data-fa-icon"));
        for glyph in ["&#xf6ff;", "&#xf1e6;", "&#xf023;", "&#xf233;", "&#xf0e7;"] {
            assert!(!shell.contains(glyph), "unpacked font glyph leaked: {glyph}");
        }
        for visible_text in [">Tailscale</button>", ">Internet</button>", ">OpenVPN</button>", ">Services</button>", ">Power Meter</button>"] {
            assert!(!shell.contains(visible_text), "indicator rendered as text pill: {visible_text}");
        }
    }

    #[test]
    fn header_obliterates_non_quarry_coronatio_branding() {
        let shell = render_crown_shell();
        assert!(shell.contains(r#"data-flask-react-quarry="Header""#));
        assert!(shell.contains(r#"class="header-left""#));
        assert!(shell.contains(r#"class="header-center""#));
        assert!(shell.contains(r#"class="header-right""#));
        assert!(shell.contains(r#"aria-label="Tailscale Status""#));
        assert!(shell.contains(r#"data-packed-icon="network-wired""#));
        assert!(shell.contains(r#"data-packed-icon="plug""#));
        assert!(shell.contains(r#"data-packed-icon="lock""#));
        assert!(shell.contains(r#"data-packed-icon="server""#));
        assert!(shell.contains(r#"data-packed-icon="bolt""#));
        assert!(shell.contains("Enter Admin Mode"));
        assert!(!shell.contains("brand-mark"));
        assert!(!shell.contains("⌂"));
        assert!(!shell.contains("/ Coronatio"));
        assert!(!shell.contains(r#"HomeServer</span><span class="muted">"#));
        assert!(!shell.contains(" ? 'live' :"));
    }

    #[test]
    fn admin_mode_binary_contract_gates_viewport_enhancements() {
        let shell = render_crown_shell();
        assert!(shell.contains(r#"data-admin-mode="false""#));
        assert!(shell.contains(r#"[data-admin-mode="false"] [data-admin-only]:not([data-admin-only="false"])"#));
        assert!(shell.contains(r#"querySelectorAll('[data-admin-only]:not([data-admin-only="false"])')"#));
        assert!(shell.contains("appRoot.dataset.adminMode = headerState.isAdmin ? 'true' : 'false'"));
        assert!(shell.contains("tabBar.dataset.adminMode = headerState.isAdmin ? 'true' : 'false'"));
        for viewport in ["admin", "stats", "portals", "upload"] {
            assert!(shell.contains(&format!(r#"data-admin-viewport="{}""#, viewport)), "missing admin viewport {viewport}");
        }
        for admin_action in ["Hard Drive Test", "Force Update", "Renew lease", "Add portal", "PIN requirement", "Blacklist"] {
            assert!(shell.contains(admin_action), "missing {admin_action}");
        }
        assert!(shell.contains("90 buttons"));
        assert!(shell.contains("History"));
        assert!(shell.contains("Open main HomeServer"));
    }

    #[tokio::test]
    async fn session_route_encodes_admin_and_caduceus_membrane() {
        let temp = test_tab_root("session-law");
        let response = app(AppState {
            tab_root: Arc::new(temp),
        })
        .oneshot(
            Request::builder()
                .uri("/api/session")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let session: AdminSessionReadback = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(session.schema, "coronatio.admin.session.v1");
        assert_eq!(session.session_timeout_seconds, 1800);
        assert_eq!(session.token_header, "X-Admin-Token");
        assert!(session
            .admin_enhanced_filtering
            .iter()
            .any(|filter| filter.topic == "system_stats"
                && filter.admin_fields.contains(&"processes".to_string())));
        assert_eq!(
            session.caduceus_membrane.schema,
            "coronatio.caduceus.membrane.v1"
        );
        assert!(session
            .caduceus_membrane
            .privileged_mutations
            .contains(&"service restart".to_string()));
    }

    #[tokio::test]
    async fn topics_route_replaces_socketio_with_sse_lease_contracts() {
        let temp = test_tab_root("topics-law");
        let response = app(AppState {
            tab_root: Arc::new(temp),
        })
        .oneshot(
            Request::builder()
                .uri("/api/topics")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let topics: TopicCatalogReadback = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(topics.schema, "coronatio.topic-catalog.v1");
        assert!(topics.transport.contains("SSE EventSource"));
        assert!(topics
            .core_topics
            .iter()
            .any(|topic| topic.id == "services.status"));
        assert!(topics
            .admin_topics
            .iter()
            .any(|topic| topic.id == "admin.disk.info" && topic.admin_only));
        let stats = topics
            .tab_topics
            .iter()
            .find(|topic| topic.pane_id == "stats")
            .unwrap();
        assert_eq!(stats.event_route, "/api/stats/events");
        assert_eq!(stats.renew_route, "/api/stats/events/renew");
    }

    #[tokio::test]
    async fn stats_sse_and_monitor_pulse_prove_first_topic() {
        let temp = test_tab_root("stats-sse");
        let router = app(AppState {
            tab_root: Arc::new(temp),
        });
        let pulse_response = router
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/api/monitor/pulse")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(pulse_response.status(), StatusCode::OK);
        let pulse_bytes = axum::body::to_bytes(pulse_response.into_body(), usize::MAX)
            .await
            .unwrap();
        let pulse: MonitorPulseReadback = serde_json::from_slice(&pulse_bytes).unwrap();
        assert_eq!(pulse.schema, "coronatio.monitor-pulse.v1");
        assert_eq!(pulse.topic.id, "stats.system");
        assert_eq!(pulse.first_event.schema, "coronatio.stats.event.v1");
        assert_eq!(pulse.event_route, "/api/stats/events");

        let event_response = router
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/api/stats/events")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(event_response.status(), StatusCode::OK);
        assert_eq!(
            event_response.headers().get(header::CONTENT_TYPE).unwrap(),
            "text/event-stream"
        );
        let event_body = String::from_utf8(
            axum::body::to_bytes(event_response.into_body(), usize::MAX)
                .await
                .unwrap()
                .to_vec(),
        )
        .unwrap();
        assert!(event_body.contains("event: stats.system"));
        assert!(event_body.contains("coronatio.stats.event.v1"));

        let renew_response = router
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/stats/events/renew")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(renew_response.status(), StatusCode::OK);
        let renew: LeaseRenewalReadback = serde_json::from_slice(
            &axum::body::to_bytes(renew_response.into_body(), usize::MAX)
                .await
                .unwrap(),
        )
        .unwrap();
        assert_eq!(renew.schema, "coronatio.stats.events.renewal.v1");
        assert_eq!(renew.topic, "stats.system");
    }

    #[tokio::test]
    async fn route_boundary_returns_json_for_api_misses_and_shell_for_static_fallback() {
        let temp = test_tab_root("boundary-law");
        let router = app(AppState {
            tab_root: Arc::new(temp),
        });
        let api_response = router
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/api/missing-route")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(api_response.status(), StatusCode::NOT_FOUND);
        let api_body = String::from_utf8(
            axum::body::to_bytes(api_response.into_body(), usize::MAX)
                .await
                .unwrap()
                .to_vec(),
        )
        .unwrap();
        assert!(api_body.contains("coronatio.api.error.v1"));
        assert!(!api_body.contains("<html"));

        let shell_response = router
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/some/client/route")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(shell_response.status(), StatusCode::OK);
        let shell_body = String::from_utf8(
            axum::body::to_bytes(shell_response.into_body(), usize::MAX)
                .await
                .unwrap()
                .to_vec(),
        )
        .unwrap();
        assert!(shell_body.contains("data-product=\"Coronatio\""));

        let boundary_response = router
            .oneshot(
                Request::builder()
                    .uri("/api/boundary")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(boundary_response.status(), StatusCode::OK);
        let boundary: BoundaryReadback = serde_json::from_slice(
            &axum::body::to_bytes(boundary_response.into_body(), usize::MAX)
                .await
                .unwrap(),
        )
        .unwrap();
        assert_eq!(boundary.schema, "coronatio.route-boundary.v1");
        assert!(boundary.api_unknown_path_policy.contains("JSON 404"));
    }

    #[tokio::test]
    async fn installer_route_encodes_premium_installer_law_without_live_mutation() {
        let temp = test_tab_root("installer-law");
        let response = app(AppState {
            tab_root: Arc::new(temp),
        })
        .oneshot(
            Request::builder()
                .uri("/api/installer")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let installer: InstallerReadback = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(installer.schema, "coronatio.installer.contract.v1");
        assert_eq!(installer.status, "contract-only");
        assert!(installer
            .root_manifest_schema
            .required_fields
            .contains(&"name".to_string()));
        assert!(installer
            .component_manifest_schema
            .operation_types
            .contains(&"append".to_string()));
        assert!(installer
            .validation_phases
            .iter()
            .any(|phase| phase.id == "version-conflict"));
        assert!(installer
            .install_phases
            .iter()
            .any(|phase| phase.id == "frontend-rebuild"));
        assert_eq!(
            installer.rollback_law.order,
            [
                "config rollback",
                "package rollback",
                "file operation rollback",
                "service state rollback"
            ]
        );
        assert!(installer
            .first_missing_live_signal
            .contains("Caduceus installer actuator"));
        assert!(installer
            .lane_mapping
            .iter()
            .any(
                |mapping| mapping.install_mode == InstallMode::FirstPartyNative
                    && mapping.rejected_shape.contains("premium package")
            ));
    }

    #[tokio::test]
    async fn frontend_storage_route_encodes_browser_persistence_and_migration_law() {
        let temp = test_tab_root("frontend-storage");
        let response = app(AppState {
            tab_root: Arc::new(temp),
        })
        .oneshot(
            Request::builder()
                .uri("/api/frontend/storage")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let data: FrontendStorageReadback = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(data.schema, "coronatio.frontend-storage.contract.v1");
        assert_eq!(data.status, "contract-only");
        assert!(data
            .persisted_stores
            .iter()
            .any(|store| store.storage_key == "homeserver-store"
                && store.persisted_fields.contains(&"activeTab".to_string())));
        assert!(data
            .persisted_stores
            .iter()
            .any(|store| store.storage_key == "auth-storage"
                && store.boundary.contains("never localStorage")));
        assert!(data
            .persistence_fields
            .iter()
            .any(|field| field.field == "isInitialized"
                && field.coronatio_owner == "startup receipt"));
        assert!(data
            .debounce_law
            .iter()
            .any(|law| law.interval_ms == 500 && law.source.contains("debouncedSetItem")));
        assert!(data
            .stale_state_law
            .iter()
            .any(|law| law.coronatio_rule.contains("malformed browser snapshot")));
        assert!(data
            .forbidden_persistence
            .contains(&"adminToken".to_string()));
        assert!(data
            .first_missing_live_signal
            .contains("storage migration adapter"));
    }

    #[tokio::test]
    async fn service_data_route_encodes_portal_monitor_and_broadcast_law() {
        let temp = test_tab_root("service-data");
        let response = app(AppState {
            tab_root: Arc::new(temp),
        })
        .oneshot(
            Request::builder()
                .uri("/api/services/data")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let data: ServiceDataReadback = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(data.schema, "coronatio.service-data.contract.v1");
        assert_eq!(data.status, "contract-only");
        assert!(data.portal_schema.fields.contains(&"remoteURL".to_string()));
        assert!(data
            .portal_schema
            .portal_types
            .contains(&"link".to_string()));
        assert!(data
            .service_card_schema
            .fields
            .contains(&"isScriptManaged".to_string()));
        assert!(data
            .monitor_topics
            .iter()
            .any(|topic| topic.topic == "admin.disk.info" && topic.admin_only));
        assert!(data
            .monitor_topics
            .iter()
            .any(|topic| topic.topic == "services.status"
                && topic.admin_fields.contains(&"isEnabled".to_string())));
        assert!(data.broadcast_law.transport_replacement.contains("SSE"));
        assert!(data
            .first_missing_live_signal
            .contains("service collectors and monitor broadcasters are not wired"));
    }

    #[tokio::test]
    async fn registry_transaction_route_encodes_config_patch_persistence_law() {
        let temp = test_tab_root("registry-transaction");
        let response = app(AppState {
            tab_root: Arc::new(temp),
        })
        .oneshot(
            Request::builder()
                .uri("/api/registry/transaction")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let transaction: RegistryTransactionReadback = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(transaction.schema, "coronatio.registry.transaction.v1");
        assert_eq!(transaction.status, "contract-only");
        assert!(transaction.deep_merge_law.tab_merge.contains("starred"));
        assert!(transaction
            .starred_tab_law
            .preservation_rule
            .contains("without displacing"));
        assert!(transaction
            .validation_law
            .factory_fallback_gate
            .contains("factoryFallback"));
        assert!(transaction
            .persistence_law
            .permission_restore
            .contains("www-data:www-data"));
        assert!(transaction
            .rollback_law
            .mismatch_policy
            .contains("do not remove"));
        assert!(transaction
            .transaction_sequence
            .iter()
            .any(|phase| phase.id == "atomic-promote" && phase.source_law.contains("shutil.move")));
        assert!(transaction
            .first_missing_live_signal
            .contains("Caduceus registry transaction actuator"));
    }

    #[tokio::test]
    async fn api_root_declares_installer_contract_route() {
        let temp = test_tab_root("installer-root-route");
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
        assert!(root.routes.contains(&"/api/installer".to_string()));
    }

    fn test_tab_root(name: &str) -> PathBuf {
        let root =
            std::env::temp_dir().join(format!("coronatio-test-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).unwrap();
        root
    }
    #[tokio::test]
    async fn full_rust_read_routes_are_registered() {
        let temp = test_tab_root("full-rust-read-routes");
        let app = app(AppState { tab_root: Arc::new(temp) });
        for route in [
            "/api/themes",
            "/api/uptime",
            "/api/portals",
            "/api/upload/history",
            "/api/admin/updates/modules/example/status",
        ] {
            let response = app.clone().oneshot(Request::builder().uri(route).body(Body::empty()).unwrap()).await.unwrap();
            assert_eq!(response.status(), StatusCode::OK, "{route}");
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
            let body = String::from_utf8(bytes.to_vec()).unwrap();
            assert!(
                body.contains("coronatio.homeserver.route.read.v1")
                    || body.contains("coronatio.upload.history.v1")
                    || body.contains("coronatio.theme-catalog.response.v1"),
                "{body}"
            );
            assert!(body.contains(route) || route == "/api/upload/history" || route == "/api/themes", "{body}");
        }
    }

    #[tokio::test]
    async fn full_rust_mutation_routes_enter_caduceus_membrane() {
        let temp = test_tab_root("full-rust-mutation-routes");
        let response = app(AppState { tab_root: Arc::new(temp) })
            .oneshot(Request::builder().method("POST").uri("/api/admin/system/restart").body(Body::empty()).unwrap())
            .await.unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body = String::from_utf8(bytes.to_vec()).unwrap();
        assert!(body.contains("coronatio.homeserver.route.mutation.v1"));
        assert!(body.contains("Caduceus staff intent membrane"));
        assert!(body.contains("/api/admin/system/restart"));
    }

    #[test]
    fn main_rs_is_thin_infinite_infinite_face() {
        let main_rs = std::fs::read_to_string("src/main.rs").unwrap();
        assert!(main_rs.contains(r#"include!("bands/contracts.rs")"#));
        assert!(main_rs.contains(r#"include!("bands/shell.rs")"#));
        assert!(main_rs.lines().count() < 80);
        assert!(std::path::Path::new("src/bands/index.json").exists());
    }

    #[test]
    fn website_endpoint_inventory_is_explicit_rust_routes() {
        let inventory = full_rust_route_inventory();
        assert!(inventory.len() > 130, "expected broad website endpoint route table");
        for required in [
            "/api/files/upload",
            "/api/service/control",
            "/api/admin/diskman/mount",
            "/api/admin/updates/modules/:module_name/status",
            "/api/wakeonlan/targets",
            "/api/dhcp/reservations/:reservation_id",
            "/api/youtube/download",
            "/api/backup/status",
            "/api/miner/stats",
        ] {
            assert!(inventory.iter().any(|(path, _)| *path == required), "missing {required}");
        }
    }

    #[tokio::test]
    async fn upload_viewport_posts_to_caduceus_route() {
        let temp = test_tab_root("upload-viewport");
        let response = app(AppState { tab_root: Arc::new(temp) })
            .oneshot(Request::builder().method("POST").uri("/api/files/upload").header("content-type", "multipart/form-data; boundary=X").body(Body::from("--X\r\nContent-Disposition: form-data; name=\"path\"\r\n\r\n/mnt/nas\r\n--X\r\nContent-Disposition: form-data; name=\"file\"; filename=\"proof.txt\"\r\nContent-Type: text/plain\r\n\r\nhello\r\n--X--\r\n")).unwrap())
            .await.unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body = String::from_utf8(bytes.to_vec()).unwrap();
        assert!(body.contains("coronatio.upload.submit.v1"), "{body}");
        assert!(body.contains("proof.txt"), "{body}");
        assert!(body.contains("Coronatio Rust upload route to Caduceus"), "{body}");
    }

    #[test]
    fn upload_viewport_has_file_picker_and_caduceus_button() {
        let html = render_crown_shell();
        assert!(html.contains("data-upload-form"));
        assert!(html.contains("data-upload-file"));
        assert!(html.contains("Upload through Caduceus"));
        assert!(html.contains("/api/files/upload"));
        assert!(html.contains("/api/upload/history"));
    }

}
