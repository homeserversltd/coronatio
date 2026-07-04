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
    async fn crown_shell_renders_compiled_vessel_without_platform_brand_nav() {
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
        assert!(body.contains("data-crown-shell=\"maud\""));
        assert!(body.contains("role=\"tablist\""));
        assert!(body.contains("data-crown-tab=\"admin\""));
        assert!(body.contains("data-crown-tab=\"stats\""));
        assert!(body.contains("data-crown-tab=\"portals\""));
        assert!(body.contains("data-crown-tab=\"upload\""));
        assert!(body.contains("data-view-panel=\"admin\""));
        assert!(body.contains("data-view-panel=\"stats\""));
        assert!(body.contains("data-view-panel=\"portals\""));
        assert!(body.contains("data-view-panel=\"upload\""));
        assert!(body.contains("data-crown-underlay=\"fallback\""));
        assert!(body.contains(CROWN_SHELL_STYLESHEET_PATH));
        assert!(body.contains(CROWN_SHELL_SCRIPT_PATH));
        assert!(!body.contains("fetch("));
        assert!(!body.contains("Arcadia"));
        assert!(!body.contains("YouTube"));
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
    fn coro_001_shell_has_layer_zero_underlay_and_native_viewport_slots() {
        let shell = render_crown_shell();
        assert!(shell.contains(r#"data-crown-shell="maud""#));
        assert!(shell.contains(r#"data-crown-underlay="fallback""#));
        assert!(shell.contains(r#"data-layer="0""#));
        assert!(shell.contains(r#"data-layer="1""#));
        for pane in PRIMARY_TABS {
            assert!(shell.contains(&format!(r#"data-crown-tab="{}""#, pane)), "missing rail tab {pane}");
            assert!(shell.contains(&format!(r#"data-view-panel="{}""#, pane)), "missing viewport slot {pane}");
        }
        assert!(shell.contains(CROWN_SHELL_STYLESHEET_PATH));
        assert!(shell.contains(CROWN_SHELL_SCRIPT_PATH));
        assert!(!shell.contains("fetch("));
        assert!(!shell.contains("hx-"));
    }

    #[test]
    fn coro_001_shell_renders_registry_tabs_after_native_rail() {
        let registry = vec![TabManifest {
            id: "service-card".to_string(),
            title: "Service Card".to_string(),
            description: String::new(),
            icon: String::new(),
            display_name: String::new(),
            order: 90,
            enabled: true,
            admin_only: true,
            visibility: TabVisibility::default(),
            data: serde_json::Value::Null,
            route_prefix: "/api/tabs/service-card".to_string(),
            static_dir: "static".to_string(),
            service_url: Some("http://127.0.0.1:9910".to_string()),
            health_route: Some("/health".to_string()),
            install_mode: InstallMode::DynamicCartridge,
        }];
        let shell = render_crown_shell_with_registry(&registry);
        assert!(shell.contains(r#"data-crown-tab="service-card""#));
        assert!(shell.contains(r#"data-view-panel="service-card""#));
        assert!(shell.contains("registry tab"));
        assert!(shell.find(r#"data-crown-tab="testtab""#).unwrap() < shell.find(r#"data-crown-tab="service-card""#).unwrap());
    }

    #[tokio::test]
    async fn coro_001_crown_assets_are_served_by_binary() {
        let temp = test_tab_root("coro-001-assets");
        let router = app(AppState { tab_root: Arc::new(temp) });
        let css = router.clone()
            .oneshot(Request::builder().uri(CROWN_SHELL_STYLESHEET_PATH).body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(css.status(), StatusCode::OK);
        let css_body = String::from_utf8(axum::body::to_bytes(css.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        assert!(css_body.contains("--ux-color-crown"));
        assert!(css_body.contains("--ux-space-4"));
        assert!(css_body.contains("--ux-radius-lg"));

        let js = router
            .oneshot(Request::builder().uri(CROWN_SHELL_SCRIPT_PATH).body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(js.status(), StatusCode::OK);
        let js_body = String::from_utf8(axum::body::to_bytes(js.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        assert!(js_body.contains("selectViewport"));
        assert!(js_body.contains("data-view-panel"));
        assert!(!js_body.contains("fetch("));
        assert!(!js_body.contains("htmx"));
    }

