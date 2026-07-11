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
        assert_eq!(registry.starred_tab, "stats");
        assert_eq!(registry.default_route_tab, "stats");
        assert_eq!(registry.visible_tabs_user, ["portals", "upload", "stats", "backblaze", "wake-on-lan", "test"]);
        assert_eq!(
            registry.visible_tabs_admin,
            ["admin", "portals", "upload", "stats", "backblaze", "wake-on-lan", "test", "dhcp"]
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
        assert_eq!(startup.initial_tab, "stats");
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
    fn indicator_modals_hydrate_route_reads_instead_of_endless_loading() {
        let shell = render_crown_shell();
        assert!(shell.contains("async function hydrateModalRouteReads(kind)"));
        assert!(shell.contains("function routeReadLabel(route, data)"));
        assert!(shell.contains("hydrateModalRouteReads(kind)"));
        for route in [
            "/api/status/tailscale",
            "/api/status",
            "/api/status/services",
            "/api/status/vpn/pia",
            "/api/status/vpn/transmission",
            "/api/status/power/usage",
        ] {
            assert!(shell.contains(route), "indicator modal read route missing: {route}");
        }
        for marker in [
            r#"data-modal-kind-body="tailscale""#,
            r#"data-modal-kind-body="internet""#,
            r#"data-modal-kind-body="services""#,
            r#"data-modal-kind-body="openvpn""#,
            r#"data-modal-kind-body="power-meter""#,
            "node.dataset.hydrated = 'true'",
            "Status unavailable: ",
        ] {
            assert!(shell.contains(marker), "hydration marker missing: {marker}");
        }
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
        assert!(shell.contains(r#"headerState.isAdmin ? `<div data-admin-only data-admin-surface="indicator-modal""#));
        assert!(shell.contains("!headerState.isAdmin && button.closest('[data-admin-only]')"));
        for admin_action in ["Update Tailnet", "Authenticate", "Run Speed Test", "Create PIA Key", "Create Transmission", "Enable Transmission over PIA VPN", "PIA Key Exists", "Service Data"] {
            assert!(shell.contains(admin_action), "missing gated admin action {admin_action}");
        }
    }

    #[test]
    fn tailscale_indicator_ports_react_modal_control_and_login_grammar() {
        let shell = render_crown_shell();
        let tailscale = indicators::render_indicator_modal("tailscale", Session::Guest).unwrap();
        for marker in [
            r#"data-flask-react-quarry="TailscaleIndicator""#,
            "LOADING...",
            "Authentication Required",
            "Tailscale service is running but needs authentication. Click the link below to complete login:",
            "Copy URL",
            "Click the authentication link above (opens in new tab)",
            "Sign in to your Tailscale account",
            "Authorize this device",
            "Return here - the status should update automatically",
            "Connect",
            "Disconnect",
            "Enable Service",
            "Disable Service",
            "Current Tailnet:",
            "Enter Tailnet name",
            "Update Tailnet",
            "Unique name used for DNS entries and TLS certificates.",
            "Alternative:",
            "If the login link isn't working, you can use an auth key instead.",
            "Enter your tskey-auth-... or tskey-client-... key",
            "Authenticate",
            "Get your auth key from the Tailscale admin console under Settings → Keys.",
            "/api/status/tailscale/update-tailnet",
            "/api/status/tailscale/authkey",
        ] {
            assert!(tailscale.contains(marker), "missing Tailscale one-to-one marker: {marker}");
        }
        assert!(shell.contains("function hydrateTailscaleModal(data)"));
        assert!(shell.contains("function modalRequestBody(button)"));
        assert!(shell.contains("tailnetName"));
        assert!(shell.contains("authKey"));
        assert!(shell.contains(r#"data-operation-label="Connecting...""#));
    }

    #[test]
    fn power_indicator_modal_has_no_invented_admin_refresh_control() {
        let power = indicators::render_indicator_modal("power-meter", Session::Guest).unwrap();
        assert!(!power.contains("data-modal-fetch"));
        assert!(!power.contains("Refresh"));
        assert!(!power.contains("data-admin-only"));
    }

    #[test]
    fn power_indicator_polls_live_watts_and_formats_react_display_value() {
        let shell = render_crown_shell();
        for marker in [
            "data-power-indicator-value",
            "power-value-small-number",
            "const POWER_DISPLAY_FACTOR = 1.6",
            "function refreshPowerIndicator()",
            "const powerChartState = { labels: [], watts: [], chart: null }",
            "function pushPowerChartPoint(label, watts)",
            "function renderPowerModal()",
            "function hydratePowerHistoryUI()",
            "data-power-average=\"5\"",
            "data-power-average=\"30\"",
            "data-power-average=\"60\"",
            "data-power-chart",
            "powerChartState.watts.slice(-seconds)",
            "pushPowerChartPoint(formatChartTime(), Number(formatPowerWatts(data.current)))",
            "new EventSource('/api/core/events')",
            "coreTopicIds.forEach",
            "if (watts < 1) return 'var(--statusUp)'",
            "if (watts < 5) return 'var(--statusPartial)'",
            "return 'var(--statusDown)'",
            "Power Usage ' + display + ' Watts",
        ] {
            assert!(shell.contains(marker), "missing power indicator marker: {marker}");
        }
    }

    #[test]
    fn power_chart_resolves_theme_colors_and_has_stable_canvas_sizing() {
        let shell = render_crown_shell();
        for marker in [
            "function themeCssColor(token, fallback)",
            "getPropertyValue(token).trim()",
            "themeCssColor('--border', '#1E293B')",
            "themeCssColor('--accent', '#90cff3')",
            "chartTicks('--hiddenTabText'",
            r#"class="chart-container power-chart-container""#,
            r#"class="coronatio-chart-canvas""#,
            "infoBackdrop.classList.contains('open')",
            "powerChartState.chart) renderPowerModal()",
            ".power-graph-container .chart-container",
            ".power-graph-container .coronatio-chart-canvas",
        ] {
            assert!(shell.contains(marker), "missing dark-mode power chart marker: {marker}");
        }
        assert!(!shell.contains("lineDataset('Power', powerChartState.watts, 'var("));
    }

    #[test]
    fn power_route_is_rust_rapl_readback_not_generic_route_ack() {
        let source = format!(
            "{}\n{}",
            std::fs::read_to_string("src/bands/full-rust-routes.rs").unwrap(),
            std::fs::read_to_string("src/bands/full-rust-routes/power.rs").unwrap()
        );
        for marker in [
            "fn power_usage_response(method: &str, path: &str) -> Response",
            "schema\": \"coronatio.power.usage.v1",
            "Coronatio Rust RAPL read route",
            "current\": sample.current_watts",
            "historical\": sample.history_watts",
            "unit\": \"W\"",
            "/sys/class/powercap/intel-rapl:0:0/energy_uj",
            "/sys/class/powercap/intel-rapl:0:1/energy_uj",
            "rapl-energy-readable-file-missing",
        ] {
            assert!(source.contains(marker), "missing Rust RAPL marker: {marker}");
        }
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
    fn internet_status_indicator_matches_react_runtime_contract() {
        let shell = render_crown_shell();
        for marker in [
            r#"data-internet-status-indicator"#,
            r#"class="indicator loading internet-indicator""#,
            r#"data-packed-icon="spinner""#,
            "Checking internet connection...",
            "const internetState = { status: 'loading'",
            "function setInternetIndicatorState(data)",
            "Internet: ${internetState.status} (${internetState.publicIp})",
            "function internetStatusModalText()",
            "CHECKING...",
            "String(internetState.status || 'loading').toUpperCase()",
            "function internetAdminDetailsHtml()",
            "details.city && details.region",
            "details.org",
            "details.timezone",
            "data-speed-test-button",
            "Running Speed Test...",
            "internetState.speedTestResults = { download: parsed.download, upload: parsed.upload, latency: parsed.latency }",
            "new EventSource('/api/core/events')",
            "applyCoreTopic(topicId, envelope)",
        ] {
            assert!(shell.contains(marker), "missing React InternetIndicator port marker: {marker}");
        }
        let internet = indicators::render_indicator_modal("internet", Session::Guest).unwrap();
        assert!(!internet.contains("Location:</strong> —"));
        assert!(!internet.contains("Download: — Mbps"));
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
        assert!(shell.contains("const headerState = { theme: savedPreferredTheme || savedHeaderState.theme || 'light', isAdmin: false };"));
        assert!(shell.contains("localStorage.setItem(headerStateKey, JSON.stringify({ theme: headerState.theme }))"));
        assert!(!shell.contains("Object.assign({ theme: savedPreferredTheme || savedHeaderState.theme || 'dark', isAdmin: false }, savedHeaderState"));
        assert!(!shell.contains("localStorage.setItem(headerStateKey, JSON.stringify(headerState))"));
        assert!(shell.contains("appRoot.dataset.adminMode = headerState.isAdmin ? 'true' : 'false'"));
        assert!(shell.contains("tabBar.dataset.adminMode = headerState.isAdmin ? 'true' : 'false'"));
        for viewport in ["admin", "stats", "portals", "upload", "test"] {
            assert!(shell.contains(&format!(r#"data-admin-viewport="{}""#, viewport)), "missing admin viewport {viewport}");
        }
        for admin_action in ["Hard Drive Test", "Auto Sync", "Hide CPU Usage & Load", "PIN requirement", "Blacklist"] {
            assert!(shell.contains(admin_action), "missing {admin_action}");
        }
        assert!(!shell.contains("data-admin-quarry"));
        assert!(shell.contains("History"));
        let portals_start = shell.find(r#"id="pane-portals""#).unwrap();
        let portals_end = shell[portals_start..].find(r#"id="pane-upload""#).unwrap() + portals_start;
        let portals = &shell[portals_start..portals_end];
        for non_quarry in [
            "Open main HomeServer",
            "Read service contract",
            "Factory portals",
            "Add portal",
            "portals-readout",
            r#"href="https://home.arpa/""#,
            r#"data-fetch="/api/services/data""#,
            r#"data-fetch="/api/portals/factory""#,
        ] {
            assert!(!portals.contains(non_quarry), "non-quarry portals control survived: {non_quarry}");
        }
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
        let session_value = serde_json::to_value(&session).unwrap();
        let mut fields = json_field_census(&session_value);
        fields.sort();
        assert_eq!(
            fields,
            vec![
                "caduceusMembrane",
                "caduceusMembrane.caduceusRole",
                "caduceusMembrane.coronatioRole",
                "caduceusMembrane.firstMissingSignal",
                "caduceusMembrane.privilegedMutations",
                "caduceusMembrane.schema",
                "keepaliveRoute",
                "logoutRoute",
                "pinValidation",
                "schema",
                "sessionTimeoutSeconds",
                "tokenHeader",
                "tokenPolicy",
            ]
        );
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

