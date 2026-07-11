    #[test]
    fn dhcp_routes_are_typed_and_no_longer_use_the_generic_read_stub() {
        let source = std::fs::read_to_string("src/bands/full-rust-routes.rs").unwrap();
        for route in [
            "/api/dhcp/status",
            "/api/dhcp/leases",
            "/api/dhcp/reservations",
            "/api/dhcp/config",
            "/api/dhcp/health",
            "/api/dhcp/statistics",
            "/api/dhcp/pool-boundary",
        ] {
            let needle = format!(".route(\"{route}\", get(dhcp_read_route)");
            assert!(source.contains(&needle), "{route} is not wired to dhcp_read_route");
        }
        let dhcp = std::fs::read_to_string("src/bands/full-rust-routes/dhcp.rs").unwrap();
        assert!(dhcp.contains("/api/v1/network/dhcp/status"));
        assert!(dhcp.contains("/api/v1/staff/intent"));
        assert!(!dhcp.contains("sudo"));
    }

    #[test]
    fn dhcp_guest_projection_strips_identity_recursively() {
        let raw = serde_json::json!({
            "ok": true,
            "statistics": {"activeLeases": 2, "reservationCount": 1},
            "leases": [{"hostname": "nas", "mac": "aa:bb", "ipAddress": "192.168.123.8"}],
            "nested": {"hostName": "console", "hwAddress": "cc:dd", "healthy": true}
        });
        let projected = strip_dhcp_identity(&raw);
        let text = serde_json::to_string(&projected).unwrap();
        assert!(text.contains("activeLeases"));
        assert!(text.contains("healthy"));
        for denied in ["nas", "aa:bb", "192.168.123.8", "console", "cc:dd", "hostname", "mac", "ipAddress", "hwAddress"] {
            assert!(!text.contains(denied), "guest projection leaked {denied}: {text}");
        }
    }

    #[tokio::test]
    async fn dhcp_guest_identity_routes_refuse_without_contacting_caduceus() {
        std::env::set_var("CADUCEUS_URL", "http://127.0.0.1:9");
        let router = app(AppState { tab_root: Arc::new(test_tab_root("dhcp-guest-refusal")) });
        for route in ["/api/dhcp/leases", "/api/dhcp/reservations", "/api/dhcp/config", "/api/dhcp/pool-boundary"] {
            let response = router.clone().oneshot(Request::builder().uri(route).body(Body::empty()).unwrap()).await.unwrap();
            assert_eq!(response.status(), StatusCode::UNAUTHORIZED, "{route}");
            let body = String::from_utf8(axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
            assert!(body.contains("coronatio.dhcp.read.refusal.v1"), "{route}: {body}");
            assert!(body.contains("admin-session-required"), "{route}: {body}");
        }
        std::env::remove_var("CADUCEUS_URL");
    }
