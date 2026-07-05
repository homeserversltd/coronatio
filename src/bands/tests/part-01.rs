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
        assert!(body.contains(CROWN_HTMX_SCRIPT_PATH));
        assert!(body.contains(CROWN_SHELL_SCRIPT_PATH));
        assert!(body.find(CROWN_HTMX_SCRIPT_PATH).unwrap() < body.find(CROWN_SHELL_SCRIPT_PATH).unwrap());
        assert!(!body.contains("fetch("));
        assert!(body.contains("hx-get=\"/admit/admin\""));
        assert!(body.contains("hx-target=\"#viewport-admin\""));
        assert!(body.contains("hx-swap=\"innerHTML\""));
        assert!(body.contains("hx-trigger=\"click\""));
        assert!(!body.contains("keyup["));
        assert!(!body.contains(" style="));
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
        assert!(shell.contains(CROWN_HTMX_SCRIPT_PATH));
        assert!(shell.contains(CROWN_SHELL_SCRIPT_PATH));
        assert!(shell.find(CROWN_HTMX_SCRIPT_PATH).unwrap() < shell.find(CROWN_SHELL_SCRIPT_PATH).unwrap());
        assert!(!shell.contains("fetch("));
        for pane in PRIMARY_TABS {
            assert!(shell.contains(&format!("hx-get=\"/admit/{}\"", pane)));
            assert!(shell.contains(&format!("hx-target=\"#viewport-{}\"", pane)));
        }
        assert!(shell.contains("hx-swap=\"innerHTML\""));
        assert!(shell.contains("hx-trigger=\"click\""));
        assert!(!shell.contains("keyup["));
        assert!(!shell.contains(" style="));
        assert!(shell.contains(r#"data-underlay-state="visible""#));
        assert!(shell.contains(r#"data-underlay-fault-kind="none""#));
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
            fragment_path: default_fragment_path(),
            client_class: ClientClass::Fragment,
            install_mode: InstallMode::DynamicCartridge,
        }];
        let shell = render_crown_shell_with_registry(&registry);
        assert!(shell.contains(r#"data-crown-tab="service-card""#));
        assert!(shell.contains(r#"data-view-panel="service-card""#));
        assert!(shell.contains("registry tab"));
        assert!(shell.find(r#"data-crown-tab="testtab""#).unwrap() < shell.find(r#"data-crown-tab="service-card""#).unwrap());
    }


    #[tokio::test]
    async fn coro_003_manifest_defaults_fragment_fields() {
        let raw = r#"{
          "id":"service-card",
          "title":"Service Card",
          "routePrefix":"/api/tabs/service-card"
        }"#;
        let manifest: TabManifest = serde_json::from_str(raw).unwrap();
        assert_eq!(manifest.fragment_path, "/fragment");
        assert_eq!(manifest.client_class, ClientClass::Fragment);
        let serialized = serde_json::to_string(&manifest).unwrap();
        assert!(serialized.contains("fragmentPath"));
        assert!(serialized.contains("clientClass"));
    }

    #[tokio::test]
    async fn coro_003_admit_native_pane_returns_fragment_html() {
        let temp = test_tab_root("coro-003-native-admit");
        let response = app(AppState { tab_root: Arc::new(temp) })
            .oneshot(Request::builder().uri("/admit/stats").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = String::from_utf8(axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        assert!(body.contains("data-fragment-schema=\"coronatio.stats.fragment.v1\""));
        assert!(body.contains("coronatio.stats.snapshot.v1"));
        assert!(body.contains("data-native-readback=\"json\""));
    }

    #[tokio::test]
    async fn coro_004_proxy_connect_failure_returns_fault_fragment_and_receipt() {
        let temp = test_tab_root("coro-004-connect-fault");
        let tab_dir = temp.join("dead-service");
        std::fs::create_dir_all(&tab_dir).unwrap();
        std::fs::write(tab_dir.join("tab.json"), r#"{
          "id":"dead-service",
          "title":"Dead Service",
          "routePrefix":"/api/tabs/dead-service",
          "serviceUrl":"http://127.0.0.1:9",
          "fragmentPath":"/fragment"
        }"#).unwrap();
        let router = app(AppState { tab_root: Arc::new(temp) });
        let response = router.clone()
            .oneshot(Request::builder().uri("/admit/dead-service").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::BAD_GATEWAY);
        assert_eq!(response.headers().get("x-coronatio-fault").unwrap(), "cartridge-fragment");
        let body = String::from_utf8(axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        assert!(body.contains("data-cartridge-fault=\"true\""));
        assert!(body.contains("data-cartridge-fault-kind=\"proxy-unreachable\""));
        assert!(body.contains("data-cartridge-fault-occurred-at="));

        let readback = router.oneshot(Request::builder().uri("/api/faults").body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(readback.status(), StatusCode::OK);
        let body = String::from_utf8(axum::body::to_bytes(readback.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        assert!(body.contains("coronatio.cartridge-faults.v1"));
        assert!(body.contains("occurred_at-unix-seconds") || body.contains("occurredAt"));
        assert!(body.contains("dead-service"));
        assert!(body.contains("proxy-unreachable"));
    }

    #[tokio::test]
    async fn coro_004_upstream_non_2xx_returns_fault_fragment_and_receipt() {
        let upstream = spawn_one_shot_http_response(503, "upstream unavailable", Duration::ZERO);
        let temp = test_tab_root("coro-004-upstream-fault");
        let tab_dir = temp.join("bad-upstream");
        std::fs::create_dir_all(&tab_dir).unwrap();
        std::fs::write(tab_dir.join("tab.json"), format!(r#"{{
          "id":"bad-upstream",
          "title":"Bad Upstream",
          "routePrefix":"/api/tabs/bad-upstream",
          "serviceUrl":"{}",
          "fragmentPath":"/fragment"
        }}"#, upstream)).unwrap();
        let router = app(AppState { tab_root: Arc::new(temp) });
        let response = router.clone().oneshot(Request::builder().uri("/admit/bad-upstream").body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
        let body = String::from_utf8(axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        assert!(body.contains("data-cartridge-fault-kind=\"upstream-error\""));
        let readback = router.oneshot(Request::builder().uri("/api/faults").body(Body::empty()).unwrap()).await.unwrap();
        let body = String::from_utf8(axum::body::to_bytes(readback.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        assert!(body.contains("bad-upstream"));
        assert!(body.contains("upstream-error"));
    }

    #[tokio::test]
    async fn coro_004_proxy_timeout_returns_fault_fragment_and_receipt() {
        let upstream = spawn_one_shot_http_response(200, "<article>late</article>", Duration::from_secs(3));
        let temp = test_tab_root("coro-004-timeout-fault");
        let tab_dir = temp.join("slow-upstream");
        std::fs::create_dir_all(&tab_dir).unwrap();
        std::fs::write(tab_dir.join("tab.json"), format!(r#"{{
          "id":"slow-upstream",
          "title":"Slow Upstream",
          "routePrefix":"/api/tabs/slow-upstream",
          "serviceUrl":"{}",
          "fragmentPath":"/fragment"
        }}"#, upstream)).unwrap();
        let router = app(AppState { tab_root: Arc::new(temp) });
        let response = router.clone().oneshot(Request::builder().uri("/admit/slow-upstream").body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(response.status(), StatusCode::BAD_GATEWAY);
        let body = String::from_utf8(axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        assert!(body.contains("data-cartridge-fault-kind=\"timeout\""));
        let readback = router.oneshot(Request::builder().uri("/api/faults").body(Body::empty()).unwrap()).await.unwrap();
        let body = String::from_utf8(axum::body::to_bytes(readback.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        assert!(body.contains("slow-upstream"));
        assert!(body.contains("timeout"));
    }

    #[tokio::test]
    async fn coro_004_tab_not_found_returns_typed_fault_receipt() {
        let temp = test_tab_root("coro-004-tab-missing");
        let router = app(AppState { tab_root: Arc::new(temp) });
        let response = router.clone().oneshot(Request::builder().uri("/admit/missing-tab").body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        let body = String::from_utf8(axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        assert!(body.contains("data-cartridge-fault-kind=\"tab-not-found\""));
        let readback = router.oneshot(Request::builder().uri("/api/faults").body(Body::empty()).unwrap()).await.unwrap();
        let body = String::from_utf8(axum::body::to_bytes(readback.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        assert!(body.contains("missing-tab"));
        assert!(body.contains("tab-not-found"));
    }

    #[tokio::test]
    async fn coro_003_static_reference_cartridge_admits_fragment() {
        let temp = test_tab_root("coro-003-static-reference");
        let tab_dir = temp.join("inert-fragment").join("static");
        std::fs::create_dir_all(&tab_dir).unwrap();
        std::fs::write(temp.join("inert-fragment").join("tab.json"), r#"{
          "id":"inert-fragment",
          "title":"Inert Fragment",
          "routePrefix":"/api/tabs/inert-fragment",
          "fragmentPath":"/static/fragment.html",
          "clientClass":"fragment"
        }"#).unwrap();
        std::fs::write(tab_dir.join("fragment.html"), r#"<article data-reference-cartridge="inert-fragment"><h2>Inert Fragment</h2></article>"#).unwrap();
        let response = app(AppState { tab_root: Arc::new(temp) })
            .oneshot(Request::builder().uri("/admit/inert-fragment").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = String::from_utf8(axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        assert!(body.contains("data-reference-cartridge=\"inert-fragment\""));
    }

    #[test]
    fn coro_003_rail_markup_fetches_on_every_activation() {
        let shell = render_crown_shell();
        for pane in PRIMARY_TABS {
            assert!(shell.contains(&format!("hx-get=\"/admit/{}\"", pane)));
            assert!(shell.contains(&format!("hx-target=\"#viewport-{}\"", pane)));
        }
        assert!(shell.contains("hx-trigger=\"click\""));
        assert!(!shell.contains("keyup["));
        assert!(!shell.contains("hx-trigger=\"load"));
        assert!(!shell.contains("hx-trigger=\"revealed"));
    }

    #[test]
    fn coro_004_chrome_fault_handler_preserves_stage_children_and_scopes_fault_readout() {
        let shell = render_crown_shell();
        let chrome = CROWN_SHELL_JS;
        assert!(shell.contains(r#"data-crown-underlay="fallback""#));
        assert!(shell.contains(r#"data-underlay-fault-kind="none""#));
        for pane in PRIMARY_TABS {
            assert!(shell.contains(&format!(r#"section class="crown-view-panel" id="viewport-{}""#, pane))
                || shell.contains(&format!(r#"id="viewport-{}""#, pane)));
            assert!(shell.contains(&format!(r#"data-view-panel="{}""#, pane)));
        }
        assert!(chrome.contains("panel.replaceChildren();"));
        assert!(chrome.contains("underlay.querySelector('[data-underlay-fault-kind]')"));
        assert!(chrome.contains("writeUnderlayFault(faultKind);"));
        assert!(!chrome.contains("stage.dataset.underlayFaultKind"));
        assert!(!chrome.contains("document.querySelector('[data-underlay-fault-kind]')"));
        assert!(!chrome.contains("stage.textContent"));
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
        assert!(js_body.contains("htmxOrgan.config.allowScriptTags = false"));
        assert!(js_body.contains("htmxOrgan.config.selfRequestsOnly = true"));
        assert!(js_body.contains("emitCartridgeFaultReceipt"));
        assert!(js_body.contains("stage.dataset.underlayState = 'visible'"));
        assert!(js_body.contains("stage.dataset.underlayState = panelIsEmptyOrFaulted(activePanel) ? 'visible' : 'occupied'"));
        assert!(js_body.contains("htmx:timeout"));
        assert!(js_body.contains("htmx:responseError"));
        assert!(!js_body.contains("fetch("));
    }

    #[tokio::test]
    async fn coro_002_crown_seats_vendored_htmx_and_csp_walls() {
        let temp = test_tab_root("coro-002-htmx");
        let router = app(AppState { tab_root: Arc::new(temp) });
        let shell_response = router.clone()
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(shell_response.status(), StatusCode::OK);
        let csp = shell_response
            .headers()
            .get(header::CONTENT_SECURITY_POLICY)
            .and_then(|value| value.to_str().ok())
            .unwrap();
        assert!(csp.contains("default-src 'self'"));
        assert!(csp.contains("script-src 'self'"));
        assert!(csp.contains("style-src 'self'"));
        let shell = String::from_utf8(axum::body::to_bytes(shell_response.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        assert!(shell.contains(&format!(r#"script defer src="{}""#, CROWN_HTMX_SCRIPT_PATH)));
        assert!(shell.find(CROWN_HTMX_SCRIPT_PATH).unwrap() < shell.find(CROWN_SHELL_SCRIPT_PATH).unwrap());
        assert!(shell.contains("hx-get=\"/admit/admin\""));
        assert!(shell.contains("hx-target=\"#viewport-admin\""));
        assert!(shell.contains("hx-swap=\"innerHTML\""));

        let htmx = router.clone()
            .oneshot(Request::builder().uri(CROWN_HTMX_SCRIPT_PATH).body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(htmx.status(), StatusCode::OK);
        let content_type = htmx.headers().get(header::CONTENT_TYPE).and_then(|value| value.to_str().ok()).unwrap();
        assert!(content_type.starts_with("application/javascript"));
        let htmx_body = String::from_utf8(axum::body::to_bytes(htmx.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        assert!(htmx_body.starts_with("var htmx=function()"));
        assert!(htmx_body.contains("allowScriptTags"));

        let chrome = router
            .oneshot(Request::builder().uri(CROWN_SHELL_SCRIPT_PATH).body(Body::empty()).unwrap())
            .await
            .unwrap();
        let chrome_body = String::from_utf8(axum::body::to_bytes(chrome.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        assert!(chrome_body.contains("htmxOrgan.config.allowScriptTags = false"));
        assert!(chrome_body.contains("htmxOrgan.config.selfRequestsOnly = true"));
        assert!(chrome_body.contains("emitCartridgeFaultReceipt"));
    }

