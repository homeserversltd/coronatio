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
        assert_eq!(registry.starred_tab, "upload");
        assert_eq!(registry.default_route_tab, "upload");
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
        assert_eq!(startup.initial_tab, "upload");
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
        assert!(shell.contains("const headerState = { theme: savedPreferredTheme || savedHeaderState.theme || 'dark', isAdmin: false };"));
        assert!(shell.contains("localStorage.setItem(headerStateKey, JSON.stringify({ theme: headerState.theme }))"));
        assert!(!shell.contains("Object.assign({ theme: savedPreferredTheme || savedHeaderState.theme || 'dark', isAdmin: false }, savedHeaderState"));
        assert!(!shell.contains("localStorage.setItem(headerStateKey, JSON.stringify(headerState))"));
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

