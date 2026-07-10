    fn portals_mirror_fixture(path: &FsPath) {
        std::fs::write(path, serde_json::json!({
            "global": { "admin": { "pin": "1234" }, "theme": { "name": "light" } },
            "tabs": { "starred": "portals", "portals": {
                "config": { "displayName": "Portals", "isEnabled": true, "adminOnly": false },
                "visibility": { "tab": true, "elements": { "Jellyfin": true, "Docs": true } },
                "data": { "portals": [
                    { "name": "Jellyfin", "description": "Media", "type": "systemd", "port": 8096, "localURL": "https://jellyfin.home.arpa", "remoteURL": "https://home.tail13aff.ts.net/jellyfin/", "services": ["jellyfin"] },
                    { "name": "Docs", "description": "Reference", "type": "link", "localURL": "https://docs.home.arpa/", "remoteURL": "https://home.tail13aff.ts.net/docs/", "services": [] }
                ] }
            }, "stats": { "config": { "displayName": "Stats", "isEnabled": true, "adminOnly": false }, "visibility": { "tab": true, "elements": {} } } }
        }).to_string()).unwrap();
    }

    #[tokio::test(flavor = "current_thread")]
    async fn portals_htmx_mirror_001_lan_host_uses_local_url_not_slash_remote() {
        let _guard = HX_EXEMPLAR_ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
        let temp = test_tab_root("portals-htmx-mirror-lan"); let config = temp.join("homeserver.json"); portals_mirror_fixture(&config);
        std::env::set_var("CORONATIO_HOMESERVER_JSON", &config);
        let router = app(AppState { tab_root: Arc::new(test_tab_root("portals-htmx-mirror-lan-app")) });
        let response = router.oneshot(Request::builder().method("GET").uri("/api/portals/elements").header("Host", "home.arpa").body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = String::from_utf8(axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        assert!(body.contains(r#"data-portal-url="https://jellyfin.home.arpa""#), "LAN must use full localURL, got: {body}");
        assert!(!body.contains("data-portal-url=\"https://home.tail13aff.ts.net/jellyfin/\""), "must not prefer slash remoteURL on LAN: {body}");
        assert!(body.contains(r#"data-portal-url="https://docs.home.arpa/""#), "link localURL on LAN: {body}");
        std::env::remove_var("CORONATIO_HOMESERVER_JSON");
    }

    #[tokio::test(flavor = "current_thread")]
    async fn portals_htmx_mirror_001_ts_host_uses_dynamic_port_prefix_not_slash_remote() {
        let _guard = HX_EXEMPLAR_ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
        let temp = test_tab_root("portals-htmx-mirror-ts"); let config = temp.join("homeserver.json"); portals_mirror_fixture(&config);
        std::env::set_var("CORONATIO_HOMESERVER_JSON", &config);
        let router = app(AppState { tab_root: Arc::new(test_tab_root("portals-htmx-mirror-ts-app")) });
        let response = router.oneshot(Request::builder().method("GET").uri("/api/portals/elements").header("Host", "home.tail13aff.ts.net").body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = String::from_utf8(axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        assert!(body.contains(r#"data-portal-url="https://home.tail13aff.ts.net:18096/""#), "ts.net systemd uses 1{{port}} dynamic URL: {body}");
        assert!(!body.contains("/jellyfin/\""), "must not use slash-path remoteURL: {body}");
        std::env::remove_var("CORONATIO_HOMESERVER_JSON");
    }

    #[test]
    fn portals_htmx_mirror_001_shell_declares_htmx_fragment_pull() {
        let document2 = include_str!("../shell/document-2.rs");
        assert!(document2.contains("hx-get=\"/api/portals/elements\""), "portals grid must hx-get fragment");
        assert!(document2.contains("data-portals-source=\"/api/portals/elements\""), "source points at fragment not JSON list");
        let document4 = include_str!("../shell/document-4.rs");
        assert!(document4.contains("window.htmx") || document4.contains("htmx.trigger"), "hydratePortals prefers HTMX trigger");
        assert!(document4.contains("return portal.localURL || '#'") || document4.contains("localURL || '#'"), "client fallback local-first");
    }

    #[test]
    fn portals_cards_keep_the_quiet_base_border_for_every_status() {
        let css = std::fs::read_to_string("src/bands/shell/ux/packs/portals.css").unwrap();
        assert!(css.contains("border: 1px solid var(--border);"), "portal cards need the Theme Net base border");
        assert!(
            !css.contains("border-color: var(--status"),
            "service status must not recolor the full portal-card perimeter"
        );
    }
