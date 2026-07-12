    fn vis_003_fixture_config(path: &FsPath) {
        std::fs::write(
            path,
            serde_json::json!({
                "global": { "admin": { "pin": "1234" }, "theme": { "name": "light" } },
                "tabs": {
                    "starred": "stats",
                    "stats": {
                        "config": { "displayName": "Stats", "isEnabled": true, "adminOnly": false },
                        "visibility": { "tab": true, "elements": {
                            "cpu-chart": true,
                            "network-chart": false,
                            "disk-usage": true,
                            "memory-usage": true,
                            "process-list": true,
                            "io-section": true,
                            "kea-leases": true
                        } }
                    },
                    "portals": {
                        "config": { "displayName": "Portals", "isEnabled": true, "adminOnly": false },
                        "visibility": { "tab": true, "elements": { "Home": true, "Jellyfin": false } },
                        "data": { "portals": [
                            { "name": "Home", "description": "Home portal", "type": "link", "localURL": "https://home.arpa/", "services": [] },
                            { "name": "Jellyfin", "description": "Media", "type": "systemd", "localURL": "https://jellyfin.home.arpa/", "services": ["jellyfin"] }
                        ] }
                    },
                    "admin": { "config": { "displayName": "Admin", "isEnabled": true, "adminOnly": true }, "visibility": { "tab": true, "elements": {} } }
                }
            })
            .to_string(),
        )
        .unwrap();
    }

    #[tokio::test(flavor = "current_thread")]
    async fn vis_003_element_write_is_admin_gated_persists_and_returns_element_fragment() {
        let _guard = HX_EXEMPLAR_ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
        let _pulse_guard = pulse_test_lock().lock().await;
        let temp = test_tab_root("vis-003-write");
        let config = temp.join("homeserver.json");
        vis_003_fixture_config(&config);
        std::env::set_var("CORONATIO_HOMESERVER_JSON", &config);
        let router = app(AppState { tab_root: Arc::new(test_tab_root("vis-003-write-app")) });

        let guest = router
            .clone()
            .oneshot(Request::builder().method("PUT").uri("/api/tabs/elements").header("content-type", "application/json").body(Body::from(r#"{"tabId":"stats","elementId":"process-usage","visibility":false}"#)).unwrap())
            .await
            .unwrap();
        assert_eq!(guest.status(), StatusCode::UNAUTHORIZED);
        let guest_body = String::from_utf8(axum::body::to_bytes(guest.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        assert!(guest_body.contains("data-element-visibility-refusal=\"admin-session-required\""));

        let token = authorize_test_admin_token();
        let response = router
            .clone()
            .oneshot(Request::builder().method("PUT").uri("/api/tabs/elements").header("X-Admin-Token", token).header("content-type", "application/json").body(Body::from(r#"{"tabId":"stats","elementId":"process-usage","visibility":false}"#)).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let fragment = String::from_utf8(axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        assert!(fragment.contains(r#"data-stat-element-id="process-usage" data-visible="false""#), "{fragment}");
        assert!(fragment.contains(r#"data-stat-visibility-toggle="process-usage" data-visible="false""#), "{fragment}");
        assert!(fragment.contains("fa-eye-slash"), "{fragment}");
        let toggle_start = fragment.find(r#"data-stat-visibility-toggle="process-usage""#).unwrap();
        let toggle_end = fragment[toggle_start..].find('>').unwrap() + toggle_start;
        let toggle_tag = &fragment[toggle_start..toggle_end];
        assert_eq!(toggle_tag.matches("data-visible=").count(), 1, "{toggle_tag}");
        let value: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(&config).unwrap()).unwrap();
        assert!(value["tabs"]["stats"]["visibility"]["elements"]["process-usage"].is_null(), "the crown fixture remains read-only; Caduceus owns persistence");
        assert!(value["tabs"]["stats"]["visibility"]["elements"].get("network").is_none());
        assert!(value["tabs"]["stats"]["visibility"]["elements"].get("process-list").is_some(), "local read fixture is not rewritten or canonicalized by Coronatio");
        assert!(std::fs::read_dir(&temp).unwrap().all(|entry| !entry.unwrap().file_name().to_string_lossy().contains("tmp")));
        std::env::remove_var("CORONATIO_HOMESERVER_JSON");
    }

    #[tokio::test(flavor = "current_thread")]
    async fn vis_003_stats_and_portals_fragments_obey_table_c_guest_admin_rows() {
        let _guard = HX_EXEMPLAR_ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
        let temp = test_tab_root("vis-003-fragments");
        let config = temp.join("homeserver.json");
        vis_003_fixture_config(&config);
        std::env::set_var("CORONATIO_HOMESERVER_JSON", &config);
        let token = authorize_test_admin_token();
        let router = app(AppState { tab_root: Arc::new(test_tab_root("vis-003-fragments-app")) });

        let stats_guest = router.clone().oneshot(Request::builder().uri("/api/stats/elements").body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(stats_guest.status(), StatusCode::OK);
        let stats_guest = String::from_utf8(axum::body::to_bytes(stats_guest.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        assert!(stats_guest.contains(r#"data-stat-element-id="cpu-chart""#), "{stats_guest}");
        assert!(!stats_guest.contains(r#"data-stat-element-id="network-chart""#), "{stats_guest}");
        assert!(!stats_guest.contains(r#"data-stat-element-id="network""#), "{stats_guest}");

        let stats_admin = router.clone().oneshot(Request::builder().uri("/api/stats/elements").header("X-Admin-Token", token.as_str()).body(Body::empty()).unwrap()).await.unwrap();
        let stats_admin = String::from_utf8(axum::body::to_bytes(stats_admin.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        assert!(stats_admin.contains(r#"data-stat-element-id="network-chart" data-visible="false""#), "{stats_admin}");
        assert!(stats_admin.contains(r#"data-stat-visibility-toggle="network-chart" data-visible="false""#), "{stats_admin}");
        assert!(stats_admin.contains("fa-eye-slash"), "{stats_admin}");
        assert!(stats_admin.contains("fa-eye"), "{stats_admin}");
        assert!(!stats_admin.contains("👁"), "{stats_admin}");
        let toggle_start = stats_admin.find(r#"data-stat-visibility-toggle="network-chart""#).unwrap();
        let toggle_end = stats_admin[toggle_start..].find('>').unwrap() + toggle_start;
        let toggle_tag = &stats_admin[toggle_start..toggle_end];
        assert_eq!(toggle_tag.matches("data-visible=").count(), 1, "{toggle_tag}");

        let portals_guest = router.clone().oneshot(Request::builder().uri("/api/portals/elements").body(Body::empty()).unwrap()).await.unwrap();
        let portals_guest = String::from_utf8(axum::body::to_bytes(portals_guest.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        assert!(portals_guest.contains(r#"data-portal-name="Home""#), "{portals_guest}");
        assert!(!portals_guest.contains(r#"data-portal-name="Jellyfin""#), "{portals_guest}");

        let portals_admin = router.clone().oneshot(Request::builder().uri("/api/portals/elements").header("X-Admin-Token", token).body(Body::empty()).unwrap()).await.unwrap();
        let portals_admin = String::from_utf8(axum::body::to_bytes(portals_admin.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        assert!(portals_admin.contains(r#"data-portal-name="Jellyfin""#), "{portals_admin}");
        assert!(portals_admin.contains(r#"data-portal-visibility-toggle="Jellyfin" data-visible="false""#), "{portals_admin}");
        assert!(portals_admin.contains("fa-eye-slash"), "{portals_admin}");
        std::env::remove_var("CORONATIO_HOMESERVER_JSON");
    }

    #[tokio::test(flavor = "current_thread")]
    async fn admin_ping_reports_live_session_authority() {
        let router = app(AppState { tab_root: Arc::new(test_tab_root("admin-ping-app")) });
        let guest = router.clone().oneshot(Request::builder().uri("/api/admin/ping").body(Body::empty()).unwrap()).await.unwrap();
        let guest: serde_json::Value = serde_json::from_slice(&axum::body::to_bytes(guest.into_body(), usize::MAX).await.unwrap()).unwrap();
        assert_eq!(guest["authenticated"], false);

        let admin = router.oneshot(Request::builder().uri("/api/admin/ping").header("X-Admin-Token", authorize_test_admin_token()).body(Body::empty()).unwrap()).await.unwrap();
        let admin: serde_json::Value = serde_json::from_slice(&axum::body::to_bytes(admin.into_body(), usize::MAX).await.unwrap()).unwrap();
        assert_eq!(admin["authenticated"], true);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn portals_visibility_put_returns_toggled_fragment() {
        let _guard = HX_EXEMPLAR_ENV_LOCK
            .get_or_init(|| Mutex::new(()))
            .lock()
            .unwrap();
        let _pulse_guard = pulse_test_lock().lock().await;
        let temp = test_tab_root("portals-visibility-write");
        let config = temp.join("homeserver.json");
        vis_003_fixture_config(&config);
        std::env::set_var("CORONATIO_HOMESERVER_JSON", &config);
        let token = authorize_test_admin_token();
        let response = app(AppState {
            tab_root: Arc::new(test_tab_root("portals-visibility-app")),
        })
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri("/api/tabs/elements")
                .header("X-Admin-Token", token)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"tabId":"portals","elementId":"Home","visibility":false}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let fragment = String::from_utf8(
            axum::body::to_bytes(response.into_body(), usize::MAX)
                .await
                .unwrap()
                .to_vec(),
        )
        .unwrap();
        assert!(fragment.contains(r#"data-portal-name="Home""#), "{fragment}");
        assert!(
            fragment.contains(r#"data-portal-element data-visible="false""#),
            "{fragment}"
        );
        assert!(
            fragment.contains(r#"data-portal-visibility-toggle="Home" data-visible="false""#),
            "{fragment}"
        );
        std::env::remove_var("CORONATIO_HOMESERVER_JSON");
    }

    #[test]
    fn vis_003_e1_dimmed_hidden_css_and_e2_canonical_stat_key_wall() {
        let shell_css = std::fs::read_to_string("src/bands/shell/ux/shell/base-and-chrome.css").unwrap();
        let stats_css = std::fs::read_to_string("src/bands/shell/ux/packs/stats.css").unwrap();
        let portals_css = std::fs::read_to_string("src/bands/shell/ux/packs/portals.css").unwrap();
        let hidden_tab_dim = r#"[data-admin-mode="true"] .tab[data-visibility="hidden"] { display: grid; background: var(--hiddenTabBackground); color: var(--hiddenTabText); opacity: .7; }"#;
        let hidden_stat_dim = r#"[data-admin-mode="true"] [data-stat-element-id][data-visible="false"]"#;
        let hidden_portal_dim = r#"[data-admin-mode="true"] [data-portal-element][data-visible="false"] { display: block; opacity: .7; }"#;
        assert!(shell_css.contains(hidden_tab_dim), "missing hidden tab dim selector: {hidden_tab_dim}");
        assert!(stats_css.contains(hidden_stat_dim), "missing hidden stat element dim selector: {hidden_stat_dim}");
        assert!(stats_css.contains("opacity: .7;"), "missing hidden stat element dim value");
        assert!(portals_css.contains(hidden_portal_dim), "missing hidden portal element dim selector: {hidden_portal_dim}");
        let retired_dim = format!(".{}", 48);
        for (name, css) in [("shell", &shell_css), ("stats", &stats_css), ("portals", &portals_css)] {
            assert!(!css.contains(&retired_dim), "{name} retained retired dim value {retired_dim}");
        }
        let shell = render_crown_shell_for_session(Session::Admin);
        assert!(shell.contains(r#"data-stat-element-id="network-chart""#));
        assert!(shell.contains(r#"data-stat-visibility-toggle="network-chart""#));
        assert!(!shell.contains(r#"data-stat-element-id="network""#));
        assert!(!shell.contains(r#"data-stat-visibility-toggle="network""#));
    }
