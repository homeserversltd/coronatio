    fn vis_002_fixture_config(path: &FsPath) {
        std::fs::write(path, serde_json::json!({
            "global": { "theme": { "name": "light" } },
            "tabs": {
                "starred": "stats",
                "admin": { "config": { "displayName": "Admin", "isEnabled": true, "adminOnly": true }, "visibility": { "tab": true, "elements": {} } },
                "stats": { "config": { "displayName": "Stats", "isEnabled": true, "adminOnly": false }, "visibility": { "tab": true, "elements": {} } },
                "portals": { "config": { "displayName": "Portals", "isEnabled": true, "adminOnly": false }, "visibility": { "tab": true, "elements": {} } },
            }
        }).to_string()).unwrap();
    }

    #[tokio::test(flavor = "current_thread")]
    async fn vis_002_session_mint_prove_and_refusal_use_only_successor_cookie_transport() {
        let _guard = HX_EXEMPLAR_ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
        let router = app(AppState { tab_root: Arc::new(test_tab_root("vis-002-session-app")) });
        let mint = router.clone().oneshot(successor_session_request(
            Request::builder().method("POST").uri("/api/session/mint").header("content-type", "application/json").body(Body::from(r#"{"pin":"fixture-only-input"}"#)).unwrap(),
            false,
        )).await.unwrap();
        assert_eq!(mint.status(), StatusCode::OK);
        let cookie = mint.headers().get(header::SET_COOKIE).and_then(|value| value.to_str().ok()).unwrap();
        assert!(cookie.contains("HttpOnly; Secure; SameSite=Strict; Path=/; Max-Age=1800"), "{cookie}");
        assert!(!cookie.contains("fixture-only-input"), "{cookie}");
        let projection: serde_json::Value = serde_json::from_slice(&axum::body::to_bytes(mint.into_body(), usize::MAX).await.unwrap()).unwrap();
        assert_eq!(projection["schema"], "coronatio.caduceus.session.projection.v1");
        assert_eq!(projection["admin"], true);
        assert!(projection.get("ticket").is_none(), "{projection}");

        let proved = router.clone().oneshot(successor_admin_request(Request::builder().method("POST").uri("/api/session/prove").body(Body::empty()).unwrap())).await.unwrap();
        assert_eq!(proved.status(), StatusCode::OK);
        let refused = router.oneshot(successor_session_request(Request::builder().method("POST").uri("/api/session/prove").body(Body::empty()).unwrap(), false)).await.unwrap();
        assert_eq!(refused.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn vis_002_session_clear_invalidates_the_browser_cookie_and_records_clear() {
        let _guard = HX_EXEMPLAR_ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
        let mark = crate::caduceus_access::test_fixture::mark();
        let router = app(AppState { tab_root: Arc::new(test_tab_root("vis-002-clear-app")) });
        let cleared = router.oneshot(successor_admin_request(Request::builder().method("POST").uri("/api/session/clear").body(Body::empty()).unwrap())).await.unwrap();
        assert_eq!(cleared.status(), StatusCode::OK);
        let cookie = cleared.headers().get(header::SET_COOKIE).and_then(|value| value.to_str().ok()).unwrap();
        assert_eq!(cookie, "caduceus_session=; HttpOnly; Secure; SameSite=Strict; Path=/; Max-Age=0");
        let records = crate::caduceus_access::test_fixture::records_since(mark);
        assert!(records.iter().any(|record| record.path == "/api/v1/access/sessions/clear"), "{records:?}");
    }

    #[tokio::test(flavor = "current_thread")]
    async fn vis_002_visibility_write_persists_config_and_returns_new_plan_fragment() {
        let _guard = HX_EXEMPLAR_ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
        let _pulse_guard = pulse_test_lock().lock().await;
        let temp = test_tab_root("vis-002-visibility");
        let config = temp.join("homeserver.json");
        vis_002_fixture_config(&config);
        std::env::set_var("CORONATIO_HOMESERVER_JSON", &config);
        let response = app(AppState { tab_root: Arc::new(test_tab_root("vis-002-visibility-app")) })
            .oneshot(successor_admin_request(Request::builder().method("POST").uri("/api/tabs/visibility").header("content-type", "application/json").body(Body::from(r#"{"tab":"portals","visible":true}"#)).unwrap()))
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
            .oneshot(successor_admin_request(Request::builder().method("POST").uri("/api/set_starred_tab").header("content-type", "application/json").body(Body::from(r#"{"tabName":"portals"}"#)).unwrap()))
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
        for (uri, expected) in [("/api/tab-bar", "stats"), ("/api/tab-bar?active=portals", "portals"), ("/api/tab-bar?active=admin", "stats")] {
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
