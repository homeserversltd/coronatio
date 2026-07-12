    fn portals_currentness_fixture(path: &FsPath) {
        std::fs::write(path, serde_json::json!({
            "global": { "admin": { "pin": "1234" } },
            "tabs": { "starred": "portals", "portals": {
                "config": { "displayName": "Portals", "isEnabled": true, "adminOnly": false },
                "visibility": { "tab": true, "elements": { "Jellyfin": true, "Transmission": true, "Relay": true, "Docs": true } },
                "data": { "portals": [
                    { "name": "Jellyfin", "description": "Media", "type": "systemd", "localURL": "https://jellyfin.home.arpa", "port": 8096, "services": ["jellyfin"] },
                    { "name": "Transmission", "description": "Downloads", "type": "systemd", "localURL": "https://transmission.home.arpa", "port": 9091, "services": ["transmission"] },
                    { "name": "Relay", "description": "Mixed", "type": "systemd", "localURL": "https://relay.home.arpa", "port": 4040, "services": ["relay", "vpn"] },
                    { "name": "Docs", "description": "Reference", "type": "link", "localURL": "https://docs.home.arpa", "services": [] }
                ] }
            }, "stats": { "config": { "displayName": "Stats", "isEnabled": true, "adminOnly": false }, "visibility": { "tab": true, "elements": {} } } }
        }).to_string()).unwrap();
    }

    #[tokio::test(flavor = "current_thread")]
    async fn portals_currentness_projects_systemd_fixture_into_fragment_and_route() {
        let _guard = HX_EXEMPLAR_ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
        let temp = test_tab_root("portals-currentness");
        let config = temp.join("homeserver.json");
        let systemctl = temp.join("systemctl.json");
        portals_currentness_fixture(&config);
        std::fs::write(&systemctl, r#"{"jellyfin":"active","transmission":"inactive","relay":"inactive","vpn":"active"}"#).unwrap();
        std::env::set_var("CORONATIO_HOMESERVER_JSON", &config);
        std::env::set_var("CORONATIO_SYSTEMCTL_FIXTURE", &systemctl);

        let router = app(AppState { tab_root: Arc::new(test_tab_root("portals-currentness-app")) });
        let fragment = router.clone().oneshot(Request::builder().method("GET").uri("/api/portals/elements").header("Host", "home.arpa").body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(fragment.status(), StatusCode::OK);
        let fragment = String::from_utf8(axum::body::to_bytes(fragment.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        assert!(fragment.contains(r#"portal-card up"#), "active service must render up: {fragment}");
        assert!(fragment.contains(r#"portal-card down"#), "inactive service must render down: {fragment}");
        assert!(fragment.contains(r#"portal-card partial"#), "mixed services must render partial: {fragment}");
        assert!(fragment.contains(r#"data-portal-services="[&quot;jellyfin&quot;]""#), "card owns its service mapping: {fragment}");

        let currentness = router.oneshot(Request::builder().method("GET").uri("/api/portals/currentness").body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(currentness.status(), StatusCode::OK);
        let currentness: serde_json::Value = serde_json::from_slice(&axum::body::to_bytes(currentness.into_body(), usize::MAX).await.unwrap()).unwrap();
        assert_eq!(currentness["schema"], "coronatio.portals.currentness.v1");
        assert_eq!(currentness["success"], true);
        assert_eq!(currentness["portals"]["Jellyfin"], "up");
        assert_eq!(currentness["portals"]["Transmission"], "down");
        assert_eq!(currentness["portals"]["Relay"], "partial");
        assert_eq!(currentness["portals"]["Docs"], "unknown");
        assert_eq!(currentness["firstMissingSignal"], "none");

        std::env::remove_var("CORONATIO_SYSTEMCTL_FIXTURE");
        std::env::remove_var("CORONATIO_HOMESERVER_JSON");
    }


    #[test]
    fn portals_currentness_shell_refreshes_visible_portals_only() {
        let document = [include_str!("../shell/document-3.rs"), include_str!("../shell/document-4.rs"), include_str!("../shell/document-4-tail.rs")].join("\n");
        assert!(document.contains("async function refreshPortalCurrentness()"));
        assert!(document.contains("/api/portals/currentness"));
        assert!(document.contains("const statuses = ['up', 'down', 'partial', 'unknown']"));
        assert!(document.contains("if (current !== next)"));
        assert!(document.contains("card.classList.remove(...statuses)"));
        assert!(document.contains("if (active === 'portals') { hydratePortals(); refreshPortalCurrentness(); }"));
        assert!(document.contains("document.visibilityState !== 'visible'"));
        assert!(!document.contains("setInterval(refreshPortalCurrentness"));
    }
