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
        assert_eq!(root.primary_tabs, ["admin", "stats", "portals", "upload", "testtab"]);
        assert_eq!(root.first_party_panes.len(), 5);
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
        assert!(body.contains("TestTab"));
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
        assert!(body.contains(r#"data-admin-quarry-button-total="87""#));
        assert!(body.contains("Hard Drive Test"));
        assert!(body.contains("Force Update"));
        assert!(body.contains("Admitted services"));
        assert!(body.contains("Upload Selected Files"));
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
        assert!(shell.contains("data-stats-viewport"));
        assert!(shell.contains(r#"class="stats-tablet""#));
        assert!(shell.contains(r#"data-stat-element-id="disk-usage""#));
        assert!(shell.contains(r#"data-stat-element-id="network""#));
        assert!(shell.contains(r#"data-stat-element-id="kea-leases""#));
        assert!(shell.contains(r#"data-stat-element-id="process-usage""#));
        assert!(shell.contains(r#"data-admin-quarry="flask-react-admin""#));
        assert!(shell.contains(r#"data-admin-quarry-button-total="87""#));
        assert!(shell.contains("data-upload-regular=\"file-ingress\""));
        assert!(shell.contains(r#"class="directory-browser-header""#));
        assert!(shell.contains("🛡️ Allow"));
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
    fn admin_regular_transition_ladder_is_cemented_in_rust_shell() {
        let shell = render_crown_shell();
        for marker in [
            "function eligibleRegularTabs()",
            "function lawfulPaneCandidate(id)",
            "function reconcileActiveTabAfterAdminExit(previousActive)",
            "function applyTabBarVisibility()",
            "if (wasAdmin && !headerState.isAdmin) reconcileActiveTabAfterAdminExit(previousActive)",
            "if (!canStarTab(button.dataset.tabStar)) return;",
            r#"[data-admin-mode="false"] .tab[data-visibility="hidden"] { display: none; }"#,
            r#"[data-admin-mode="true"] .tab[data-visibility="hidden"] { display: grid; }"#,
        ] {
            assert!(shell.contains(marker), "missing tab ladder marker: {marker}");
        }
        assert!(shell.contains("eligibleRegularTabs().length <= 2"));
        assert!(shell.contains("tab.dataset.adminOnly === 'true') return headerState.isAdmin ? id : firstVisibleTab()"));
        assert!(shell.contains("tab.dataset.visibility === 'hidden') return firstVisibleTab()"));
    }

    #[test]
    fn registry_admin_mode_includes_hidden_regular_tabs_for_restoration() {
        let mut contracts = native_tab_contracts();
        contracts
            .iter_mut()
            .find(|tab| tab.id == "upload")
            .expect("upload tab exists")
            .visibility
            .tab = false;
        let regular = visible_tab_ids(&contracts, false);
        let admin = visible_tab_ids(&contracts, true);
        assert!(!regular.contains(&"upload".to_string()));
        assert!(admin.contains(&"upload".to_string()));
        assert!(admin.contains(&"admin".to_string()));
        assert!(!eligible_starred_tab_ids(&contracts).contains(&"upload".to_string()));
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
    fn native_stock_testtab_is_composed_from_ux_library() {
        let shell = render_crown_shell();
        let registry = ux_component_registry();
        for marker in [
            "coronatio-composable-ux.v1",
            "data-native-stock-testtab=\"true\"",
            "data-react-quarry=\"premium/testTab\"",
            "data-ux-registry=\"rust-native\"",
            "data-showcase-tab=\"buttons\"",
            "data-showcase-tab=\"modals\"",
            "data-testtab-panel=\"theme-values\"",
            "data-theme-values-panel=\"true\"",
            "data-ux-registry-count",
            "data-ux-component=\"theme-gradients\"",
            "data-ux-component=\"theme-highlights\"",
            "data-ux-component=\"theme-accents\"",
            "data-ux-component=\"theme-role-pairs\"",
            "data-ux-component=\"tabs-plain\"",
            "data-ux-component=\"tabs-favorite\"",
            "data-ux-component=\"tabs-favorite-visibility\"",
            "data-ux-tab-affordance=\"plain\"",
            "data-ux-tab-affordance=\"favorite\"",
            "data-ux-tab-affordance=\"favorite-visibility\"",
            "ux-tab-star",
            "ux-tab-eye",
            "ux-tab-faded",
            "data-hidden-tab=\"true\"",
            "Favorite + hide/fade strip",
            ".ux-tab-faded, .ux-tab[data-hidden-tab=\"true\"]",
            "expanded JSON gradient-accent",
            "ux-button",
            "ux-card",
            "ux-tabs",
            "ux-field",
            "ux-badge",
            "ux-badge-button",
            "ux-table",
            "ux-table-shell",
            "ux-table-sortable",
            "ux-table-selectable",
            "ux-progress",
            "data-showcase-tab=\"graphs\"",
            "data-ux-component=\"graph-line-area\"",
            "data-ux-component=\"graph-bar\"",
            "data-ux-component=\"graph-donut\"",
            "data-ux-component=\"graph-sparkline\"",
            "ux-chart-line-path",
            "ux-chart-bars",
            "ux-donut",
            "ux-sparkline",
            "ux-card-button",
            "ux-interactive",
            "--theme-component-button-container",
            "--theme-elevation-2",
            ".ux-tab:focus-visible",
            ".ux-toggle:hover",
            "<code>--primary</code>, <code>--primaryHover</code>, <code>--theme-component-button-container</code>",
            "Theme Values",
            "live CSS token map",
            "data-theme-value-family=\"core\"",
            "data-theme-value-family=\"actions\"",
            "data-theme-value-family=\"gradients\"",
            "data-theme-value-family=\"highlights\"",
            "data-theme-value-family=\"accents\"",
            "data-theme-value-family=\"roles\"",
            "--theme-component-button-container",
            "--theme-highlight-ring",
            "--theme-radius-pill",
        ] {
            assert!(shell.contains(marker), "missing TestTab UX marker: {marker}");
        }
        assert!(shell.contains(".ux-button {"));
        assert!(!shell.contains("data-testtab-tab=\"services\""));
        assert!(!shell.contains("data-testtab-tab=\"config\""));
        assert!(!shell.contains("data-testtab-tab=\"health\""));
        assert!(!shell.contains("data-testtab-panel=\"services\""));
        assert!(!shell.contains("data-testtab-panel=\"config\""));
        assert!(!shell.contains("data-testtab-panel=\"health\""));
        assert!(!shell.contains("Service Tests"));
        assert!(!shell.contains("Configuration</button>"));
        assert!(!shell.contains("Health Status"));
        assert!(shell.contains("background: var(--primary); color: var(--text);"));
        assert!(shell.contains(".ux-button.secondary { background: var(--theme-surface-1); color: var(--text); }"));
        assert!(shell.contains(".ux-button.secondary:hover { background: var(--theme-component-button-hover-container);"));
        assert!(shell.contains(".ux-button.success { background: var(--success);"));
        assert!(shell.contains("class=\"ux-card ux-card-button clickable\""));
        assert!(shell.contains("class=\"ux-badge ux-badge-button primary\""));
        assert!(shell.contains("aria-label=\"Interactive badge buttons\""));
        assert!(shell.contains(".ux-badge-button:hover"));
        assert!(shell.contains(".ux-badge-button:focus-visible"));
        assert!(shell.contains(".ux-badge-button[aria-pressed=\"true\"]"));
        assert!(shell.contains(".ux-card.clickable:hover, .ux-card-button:hover"));
        assert_eq!(shell.matches("data-ux-component=").count(), registry.len());
        for component in registry {
            assert!(shell.contains(&format!("data-ux-component=\"{}\"", component.id)), "missing registered component {}", component.id);
        }
    }
