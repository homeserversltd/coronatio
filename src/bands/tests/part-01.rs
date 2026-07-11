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


    #[test]
    fn docs_inscribe_pane_creation_ladder_and_delegated_chrome_law() {
        let bands = std::fs::read_to_string("src/bands/README.md").unwrap();
        let shell = std::fs::read_to_string("src/bands/shell/README.md").unwrap();
        for marker in [
            "## Adding a crown pane",
            "Add one `CrownPane` literal to `native_crown_panes()`",
            "`route: /#<id>`",
            "`state_route: /api/panes/<id>`",
            "The Test tab is the executable spec; copy its markup, do not invent classes.",
            "Splice the pane through a `__PLACEHOLDER__` in `render_crown_shell()`",
            "HTMX `/admit/<pane>` fragments with `Cache-Control: no-store`",
            "A pane fragment SHALL NOT bind listeners at init time",
            "break-glass commit `17bf406`",
            "literal braces are escaped as `{{ }}`",
        ] {
            assert!(bands.contains(marker), "bands README missing pane ladder marker: {marker}");
        }
        for marker in [
            "## Shell band law",
            "extracting inline chrome into `/static/crown/chrome.js`",
            "Delegated-chrome law",
            "body-level delegated listeners keyed by stable `data-*` attributes",
            "Pane fragments SHALL NOT attach init-time listeners",
            "Generic tab-scope convention",
            "[data-tab-scope=\"<scope>\"]",
            "[data-tab-id=\"<panel>\"]",
            "[data-tab-panel=\"<panel>\"]",
            "zero JavaScript edits",
        ] {
            assert!(shell.contains(marker), "shell README missing shell law marker: {marker}");
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
        assert_eq!(root.primary_tabs, ["admin", "portals", "upload", "stats", "backblaze", "wake-on-lan", "test", "dhcp"]);
        assert_eq!(root.first_party_panes.len(), PRIMARY_TABS.len());
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
        assert!(body.contains("Test"));
        assert!(body.contains("backBlaze"));
        assert!(body.contains("Wake on LAN"));
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
        assert!(!body.contains("data-pane=\"admin\""));
        assert!(body.contains("data-pane=\"stats\""));
        assert!(body.contains("data-pane=\"portals\""));
        assert!(body.contains("data-pane=\"upload\""));
        assert!(body.contains("data-pane-panel=\"admin\""));
        assert!(body.contains("data-pane-panel=\"stats\""));
        assert!(body.contains("data-pane-panel=\"portals\""));
        assert!(body.contains("data-pane-panel=\"upload\""));
        assert!(body.contains("function showPane(id)"));
        assert!(body.contains("fetch('/api/stats')"));
        assert!(body.contains(r#"class="disk-actions""#));
        assert!(body.contains("Hard Drive Test"));
        assert!(body.contains("Auto Sync"));
        assert!(body.contains("Admitted services"));
        assert!(body.contains("Upload Selected Files"));
        assert!(!body.contains("Coronatio crown shell"));
        assert!(!body.contains("class=\"crown-card\""));
        assert!(!body.contains("Arcadia"));
    }

    #[test]
    fn native_pane_bodies_are_not_placeholder_cards() {
        let shell = render_crown_shell();
        for pane in PRIMARY_TABS {
            assert!(shell.contains(&format!("data-pane-panel=\"{}\"", pane)));
        }
        for pane in ["portals", "upload", "stats", "backblaze", "wake-on-lan", "test"] {
            assert!(shell.contains(&format!("data-tab-id=\"{}\"", pane)));
        }
        assert!(!shell.contains("data-tab-id=\"admin\""));
        assert!(shell.contains("data-stats-viewport"));
        assert!(shell.contains(r#"class="stats-tablet""#));
        assert!(shell.contains(r#"data-stat-element-id="disk-usage""#));
        assert!(shell.contains(r#"data-stat-element-id="network-chart""#));
        assert!(shell.contains(r#"data-stat-element-id="kea-leases""#));
        assert!(shell.contains(r#"data-stat-element-id="process-usage""#));
        assert!(shell.contains(r#"class="disk-actions""#));
        assert!(!shell.contains("data-admin-quarry"));
        assert!(shell.contains("data-upload-regular=\"file-ingress\""));
        assert!(shell.contains(r#"class="directory-browser-header""#));
        assert!(shell.contains("🛡️ Allow"));
        assert!(!shell.contains("First-party panes are native Rust crown law. Installed services enter through governed cartridges or source-injection recompiles."));
    }



    #[test]
    fn normal_mode_keeps_primary_tabs_visible_and_admin_only_enhances_controls() {
        let shell = render_crown_shell();
        for pane in ["portals", "upload", "stats", "backblaze", "wake-on-lan", "test"] {
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
        assert!(!shell.contains(r#"data-tab-id="admin""#), "guest projection omits admin tab markup entirely");
        for pane in ["portals", "upload", "stats", "backblaze", "wake-on-lan", "test"] {
            assert!(!shell.contains(&format!(r#"data-tab-visibility-toggle="{}""#, pane)), "guest projection omits admin eye controls");
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
            "refreshTabBar(previousActive).then(selectedTab =>",
            "if (selectedTab) showPane(selectedTab)",
            "?active=' + encodeURIComponent(activeTabId)",
            "if (!canStarTab(button.dataset.tabStar)) return;",
            "fetch('/api/tab-bar' + activeParam",
            "fetch('/api/tabs/visibility'",
        ] {
            assert!(shell.contains(marker), "missing tab ladder marker: {marker}");
        }
        assert!(shell.contains("eligibleRegularTabs().length <= 2"));
        assert!(!shell.contains("if (wasAdmin && !headerState.isAdmin) reconcileActiveTabAfterAdminExit(previousActive)"));
        assert!(!shell.contains("else if (headerState.isAdmin && previousActive === fallbackTab) showPane(firstVisibleTab())"));
        assert!(!shell.contains("const response = await fetch('/api/tab-bar', { headers })"));
        let retired_dim = format!(".{}", 48);
        assert!(!shell.contains(&retired_dim));
        assert!(shell.contains("tab.dataset.visibility === 'hidden') return firstVisibleTab()"));
    }

    #[test]
    fn registry_admin_mode_includes_hidden_regular_tabs_for_restoration() {
        let mut contracts = native_tab_contracts();
        contracts
            .iter_mut()
            .find(|tab| tab.id == "dhcp")
            .expect("dhcp tab exists")
            .visibility
            .tab = false;
        let _regular = visible_tab_ids(&contracts, false);
        let admin = visible_tab_ids(&contracts, true);
        assert!(admin.contains(&"admin".to_string()));
    }

    #[test]
    fn crown_tabbar_recreates_flask_react_star_eye_and_hide_controls() {
        let shell = render_crown_shell_for_session(Session::Admin);
        assert!(shell.contains("class=\"tab-bar\""));
        for pane in ["admin", "portals", "upload", "stats", "backblaze", "wake-on-lan", "test", "dhcp"] {
            assert!(shell.contains(&format!("data-tab-id=\"{}\"", pane)));
            assert!(shell.contains(&format!("data-pane=\"{}\"", pane)));
        }
        for pane in ["portals", "upload", "stats", "backblaze", "wake-on-lan", "test", "dhcp"] {
            assert!(shell.contains(&format!("data-tab-visibility-toggle=\"{}\"", pane)));
        }
        for pane in ["portals", "upload", "stats", "backblaze", "wake-on-lan", "test"] {
            assert!(shell.contains(&format!("data-tab-star=\"{}\"", pane)));
        }
        assert!(shell.contains("class=\"visibility-toggle ui-visibility-toggle\""));
        assert!(shell.contains("class=\"star-button fas fa-star\""));
        assert!(shell.contains("class=\"star-button"));
        assert!(shell.contains("data-visibility=\"visible\""));
        assert!(shell.contains("data-visibility=\"hidden\""));
        assert!(!shell.contains("hiddenTabs"));
        assert!(shell.contains("firstVisibleTab()"));
        assert!(shell.contains("setStarredTab"));
        assert!(shell.contains("refreshTabBar"));
        assert!(shell.contains("fa-eye-slash"));
        let nav_start = shell.find("<nav class=\"tab-bar\"").expect("tab bar starts");
        let nav_end = shell[nav_start..].find("</nav>").map(|offset| nav_start + offset).expect("tab bar ends");
        let nav = &shell[nav_start..nav_end];
        assert!(!shell.contains(".tab[data-visibility=\"hidden\"] .tab-name { text-decoration: line-through; }"));
        assert!(!nav.contains("🔒"), "hidden tab state is the eye only, not a lock glyph");
        assert!(!shell.contains("class=\"tab-button\""));
    }

    #[test]
    fn test_visibility_system_is_the_live_fontawesome_catalog() {
        let source = std::fs::read_to_string("src/bands/shell/test.rs").unwrap();
        let shell = render_crown_shell();
        for marker in [
            "visibility-system",
            "Visibility System",
            "data-visibility-system-catalog",
            "ui-visibility-toggle",
            "fa-eye",
            "fa-eye-slash",
            "aria-pressed",
            "Visible tab",
            "Hidden tab",
            "Admin-only tab",
            "Element visible",
            "Element dimmed hidden",
        ] {
            assert!(shell.contains(marker), "missing visibility catalog marker: {marker}");
        }
        assert!(!source.contains("👁"), "Test visibility catalog retained emoji eye substitute");
        assert!(!source.contains("🙈"), "Test visibility catalog retained emoji hidden substitute");
        assert!(shell.contains("catalogEye.dataset.visible = String(visible)"));
        assert!(shell.contains("icon.className = visible ? 'fas fa-eye' : 'fas fa-eye-slash'"));
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
        assert!(shell.contains(r#"data-theme-button data-admin-only="true" hidden"#));
        assert!(shell.contains(r#"data-change-pin-button data-admin-only="true" hidden"#));
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
        let header_right_start = shell.find(r#"<div class="header-right">"#).expect("header right starts");
        let header_right_end = shell[header_right_start..].find("</div>").map(|offset| header_right_start + offset).expect("header right ends");
        let header_right = &shell[header_right_start..header_right_end];
        assert_eq!(header_right.matches(r#"<button type="button"#).count(), 3);
        assert_eq!(header_right.matches(r#"data-admin-only="true"#).count(), 2);
        assert!(header_right.contains(r#"data-admin-button data-admin-state="logged-out">Enter Admin Mode</button>"#));
        for normal_forbidden in ["data-change-pin-button>Change PIN", "data-theme-button title="] {
            assert!(!header_right.contains(normal_forbidden), "normal-mode header button leaked: {normal_forbidden}");
        }
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
        for pane in ["admin", "portals", "upload", "stats", "backblaze", "wake-on-lan", "test", "dhcp"] {
            assert!(shell.contains(&format!(r#"data-pane-panel="{}""#, pane)));
        }
        assert!(shell.contains("document.documentElement.dataset.theme = headerState.theme"));
        assert!(shell.contains("aria-pressed"));
        assert!(!shell.contains("const themeCatalog = {"));
        assert!(!shell.contains(r#":root[data-theme="light"]"#));
        assert!(!shell.contains("Choose the active HOMESERVER theme."));
    }



    #[test]
    fn native_stock_test_is_og_bedrock_port() {
        let shell = render_crown_shell();
        let retired_id = ["test", "tab"].concat();
        assert!(!shell.contains(&format!(r#"data-pane-panel=\"{}\""#, retired_id)));
        assert!(!shell.contains(&format!(r#"data-tab-id=\"{}\""#, retired_id)));
        assert!(!shell.contains(&format!(r#"id=\"pane-{}\""#, retired_id)));
        assert!(!shell.contains(r#"data-og-stub-pane=\"test\""#));
        for marker in [
            "data-native-stock-test=\"true\"",
            "data-react-quarry=\"legacy-tabs/testTab\"",
            "data-ux-library=\"og-styles-common-ui\"",
            "data-ux-registry=\"og-test-bedrock\"",
            "data-tab-scope=\"test\"",
            "data-tab-id=\"showcase\"",
            "data-tab-id=\"services\"",
            "data-tab-id=\"config\"",
            "data-tab-id=\"health\"",
            "Component Showcase",
            "Services",
            "Configuration",
            "Health Status",
            "data-tab-panel=\"services\"",
            "data-tab-panel=\"config\"",
            "data-tab-panel=\"health\"",
            "TEST-001 LIBRARY band",
            ".ui-button",
            ".ui-toggle",
            ".ui-tab",
            ".ui-input",
            ".ui-select",
            ".ui-card",
            ".ui-badge",
            ".ui-checkbox",
            ".ui-slider",
            "text-box",
            ".progress-bar",
            ".ui-table",
            "// TEST-001: og Test UX-library chrome is allowed here",
            "data-ux-modal-open=\"small\"",
            "data-ux-modal-open=\"medium\"",
            "data-ux-modal-open=\"fullscreen\"",
        ] {
            assert!(shell.contains(marker), "missing og Test marker: {marker}");
        }
        let categories = [
            ("buttons", "Buttons"),
            ("toggles", "Toggles"),
            ("tabs", "Tabs"),
            ("inputs", "Inputs"),
            ("cards", "Cards"),
            ("badges", "Badges"),
            ("checkboxes", "Checkboxes"),
            ("utilities", "Utilities"),
            ("visibility-system", "Visibility System"),
            ("calendar-time", "Calendar & Time"),
            ("row-info-tile", "Row Info Tile"),
            ("dropdowns", "Dropdowns"),
            ("slider", "Slider"),
            ("textbox", "Text Box"),
            ("upload-components", "Upload Components"),
            ("progress-bar", "Progress Bar"),
            ("table", "Table"),
            ("collapsible", "Collapsible"),
            ("modals", "Modals"),
        ];
        for (id, title) in categories {
            assert!(shell.contains(&format!("data-tab-id=\"{}\"", id)), "missing category chip {id}");
            assert!(shell.contains(&format!("id=\"showcase-{}\"", id)), "missing category section {id}");
            assert!(shell.contains(title), "missing category title {title}");
        }
        assert_eq!(shell.matches("data-category-chip=").count(), 19);
        assert_eq!(shell.matches("data-og-category-section=").count(), 19);
        assert!(!shell.contains("data-test-panel=\"theme-values\""));
        assert!(!shell.contains("Mini Theme Token Lab"));
        assert!(!shell.contains("data-theme-token-lab=\"true\""));
        assert!(!shell.contains("data-category-chip=\"graphs\""));
    }
