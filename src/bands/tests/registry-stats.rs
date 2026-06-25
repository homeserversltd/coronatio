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

