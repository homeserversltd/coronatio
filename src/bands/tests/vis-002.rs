    fn vis_002_fixture_config(path: &FsPath) {
        std::fs::write(path, serde_json::json!({
            "global": { "admin": { "pin": "1234" }, "theme": { "name": "light" } },
            "tabs": {
                "starred": "stats",
                "admin": { "config": { "displayName": "Admin", "isEnabled": true, "adminOnly": true }, "visibility": { "tab": true, "elements": {} } },
                "stats": { "config": { "displayName": "Stats", "isEnabled": true, "adminOnly": false }, "visibility": { "tab": true, "elements": {} } },
                "portals": { "config": { "displayName": "Portals", "isEnabled": true, "adminOnly": false }, "visibility": { "tab": true, "elements": {} } },
            }
        }).to_string()).unwrap();
    }

    #[tokio::test(flavor = "current_thread")]
    async fn vis_002_validate_pin_mints_server_session_and_refuses_pin_shaped_tokens() {
        let _guard = HX_EXEMPLAR_ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
        let temp = test_tab_root("vis-002-session");
        let config = temp.join("homeserver.json");
        vis_002_fixture_config(&config);
        std::env::set_var("CORONATIO_HOMESERVER_JSON", &config);
        let router = app(AppState { tab_root: Arc::new(test_tab_root("vis-002-session-app")) });
        let response = router.clone().oneshot(Request::builder().method("POST").uri("/api/validatePin").header("content-type", "application/json").body(Body::from(r#"{"pin":"1234"}"#)).unwrap()).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body: serde_json::Value = serde_json::from_slice(&axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap()).unwrap();
        let token = body.get("token").and_then(serde_json::Value::as_str).unwrap();
        assert!(token.starts_with("coronatio-admin-session-"));
        assert_ne!(token, "1234");
        assert!(!token.starts_with("1234"));
        assert!(!token.contains("1234"));
        let ok = router.clone().oneshot(Request::builder().method("POST").uri("/admit/admin/toggle/ssh-service").header("X-Admin-Token", token).body(Body::empty()).unwrap()).await.unwrap();
        assert_ne!(ok.status(), StatusCode::UNAUTHORIZED);
        for bad in ["1234", "1234-deadbeef", "x1234x"] {
            let response = router.clone().oneshot(Request::builder().method("POST").uri("/admit/admin/toggle/ssh-service").header("X-Admin-Token", bad).body(Body::empty()).unwrap()).await.unwrap();
            assert_eq!(response.status(), StatusCode::UNAUTHORIZED, "{bad}");
        }
        std::env::remove_var("CORONATIO_HOMESERVER_JSON");
    }


    #[tokio::test(flavor = "current_thread")]
    async fn vis_002_logout_invalidates_server_session_token() {
        let _guard = HX_EXEMPLAR_ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
        let token = authorize_test_admin_token();
        let router = app(AppState { tab_root: Arc::new(test_tab_root("vis-002-logout-app")) });
        let before = router.clone().oneshot(Request::builder().method("POST").uri("/admit/admin/toggle/ssh-service").header("X-Admin-Token", token.as_str()).body(Body::empty()).unwrap()).await.unwrap();
        assert_ne!(before.status(), StatusCode::UNAUTHORIZED);
        let logout = router.clone().oneshot(Request::builder().method("POST").uri("/api/logout").header("X-Admin-Token", token.as_str()).body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(logout.status(), StatusCode::OK);
        let after = router.clone().oneshot(Request::builder().method("POST").uri("/admit/admin/toggle/ssh-service").header("X-Admin-Token", token.as_str()).body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(after.status(), StatusCode::UNAUTHORIZED);
    }

    #[test]
    fn vis_002_lockout_ladder_matches_og_schedule() {
        assert_eq!(lockout_ms(1), 1_000);
        assert_eq!(lockout_ms(2), 2_000);
        assert_eq!(lockout_ms(3), 4_000);
        assert_eq!(lockout_ms(9), 256_000);
        assert_eq!(lockout_ms(10), 256_000);
        assert_eq!(lockout_ms(99), 256_000);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn vis_002_visibility_write_persists_config_and_returns_new_plan_fragment() {
        let _guard = HX_EXEMPLAR_ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
        let _pulse_guard = pulse_test_lock().lock().await;
        let temp = test_tab_root("vis-002-visibility");
        let config = temp.join("homeserver.json");
        vis_002_fixture_config(&config);
        std::env::set_var("CORONATIO_HOMESERVER_JSON", &config);
        let token = authorize_test_admin_token();
        let response = app(AppState { tab_root: Arc::new(test_tab_root("vis-002-visibility-app")) })
            .oneshot(Request::builder().method("POST").uri("/api/tabs/visibility").header("X-Admin-Token", token).header("content-type", "application/json").body(Body::from(r#"{"tab":"portals","visible":true}"#)).unwrap())
            .await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let fragment = String::from_utf8(axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        assert!(fragment.contains("data-visibility=\"visible\""), "{fragment}");
        let _value: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(&config).unwrap()).unwrap();
        assert!(std::fs::read_dir(&temp).unwrap().all(|entry| !entry.unwrap().file_name().to_string_lossy().contains("tmp")));
        std::env::remove_var("CORONATIO_HOMESERVER_JSON");
    }

    #[tokio::test(flavor = "current_thread")]
    async fn vis_002_star_write_uses_iris_invariant_and_returns_fragment() {
        let _guard = HX_EXEMPLAR_ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
        let _pulse_guard = pulse_test_lock().lock().await;
        let temp = test_tab_root("vis-002-star");
        let config = temp.join("homeserver.json");
        vis_002_fixture_config(&config);
        std::env::set_var("CORONATIO_HOMESERVER_JSON", &config);
        let response = app(AppState { tab_root: Arc::new(test_tab_root("vis-002-star-app")) })
            .oneshot(Request::builder().method("POST").uri("/api/set_starred_tab").header("content-type", "application/json").body(Body::from(r#"{"tabName":"portals"}"#)).unwrap())
            .await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let fragment = String::from_utf8(axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        assert!(fragment.contains("data-tab-star=\"portals\""), "{fragment}");
        assert!(fragment.contains("aria-pressed=\"true\""), "{fragment}");
        let value: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(&config).unwrap()).unwrap();
        assert_eq!(value["tabs"]["starred"], serde_json::Value::String("stats".to_string()), "the crown fixture remains read-only; Caduceus owns persistence");
        std::env::remove_var("CORONATIO_HOMESERVER_JSON");
    }

    #[tokio::test(flavor = "current_thread")]
    async fn vis_002_tab_bar_fragment_has_one_active_selection_and_lands_requested_tab_lawfully() {
        let _guard = HX_EXEMPLAR_ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
        let temp = test_tab_root("vis-002-tab-bar-active");
        let config = temp.join("homeserver.json");
        vis_002_fixture_config(&config);
        std::env::set_var("CORONATIO_HOMESERVER_JSON", &config);
        let router = app(AppState { tab_root: Arc::new(test_tab_root("vis-002-tab-bar-active-app")) });

        for (uri, expected) in [
            ("/api/tab-bar", "stats"),
            ("/api/tab-bar?active=portals", "portals"),
            ("/api/tab-bar?active=admin", "stats"),
        ] {
            let response = router.clone().oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap()).await.unwrap();
            assert_eq!(response.status(), StatusCode::OK, "{uri}");
            let fragment = String::from_utf8(axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
            assert_eq!(fragment.matches("aria-selected=\"true\"").count(), 1, "{uri}: {fragment}");
            assert_eq!(fragment.matches("class=\"tab active\"").count(), 1, "{uri}: {fragment}");
            assert!(fragment.contains(&format!("aria-selected=\"true\" data-pane=\"{expected}\"")), "{uri}: {fragment}");
        }

        std::env::remove_var("CORONATIO_HOMESERVER_JSON");
    }

    #[test]
    fn vis_002_guest_and_admin_tabbar_projection_wall() {
        let facts = iris::from_coronatio_contracts(&native_tab_contracts(), "stats");
        let guest = iris::plan(&facts, Session::Guest);
        let admin = iris::plan(&facts, Session::Admin);
        assert!(!guest.tabs.iter().any(|grant| grant.tab_id == "admin" || grant.state == RenderState::DimmedHidden));
        assert!(admin.tabs.iter().any(|grant| grant.tab_id == "admin"));
        assert!(admin.tabs.iter().any(|grant| grant.state == RenderState::DimmedHidden));
    }
