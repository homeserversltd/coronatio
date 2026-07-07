    fn field_003_in_scope_read_routes() -> Vec<&'static str> {
        vec![
            "/api/status/tailscale",
            "/api/status/tailscale/config",
            "/api/dhcp/status",
            "/api/dhcp/leases",
            "/api/dhcp/reservations",
            "/api/dhcp/config",
            "/api/dhcp/health",
            "/api/dhcp/statistics",
            "/api/dhcp/pool-boundary",
            "/api/kea-leases",
            "/api/network/notes",
            "/api/wakeonlan/targets",
            "/api/wakeonlan/status",
        ]
    }

    fn field_003_in_scope_mutations() -> Vec<(&'static str, &'static str)> {
        vec![
            ("POST", "/api/status/tailscale/connect"),
            ("POST", "/api/status/tailscale/authkey"),
            ("POST", "/api/status/tailscale/disconnect"),
            ("POST", "/api/status/tailscale/enable"),
            ("POST", "/api/status/tailscale/disable"),
            ("POST", "/api/status/tailscale/config"),
            ("POST", "/api/status/tailscale/update-tailnet"),
            ("PUT", "/api/network/notes"),
            ("POST", "/api/wakeonlan/targets"),
            ("POST", "/api/wakeonlan/wake"),
            ("DELETE", "/api/wakeonlan/targets/fixture-target"),
            ("POST", "/api/dhcp/reservations"),
            ("PUT", "/api/dhcp/reservations/fixture-reservation"),
            ("DELETE", "/api/dhcp/reservations/fixture-reservation"),
            ("POST", "/api/dhcp/config"),
            ("POST", "/api/dhcp/pool-boundary"),
        ]
    }

    #[test]
    fn field_003_census_wall_routes_are_generic_membranes_not_live_fact_bodies() {
        let source = std::fs::read_to_string("src/bands/full-rust-routes.rs").unwrap();
        for route in field_003_in_scope_read_routes() {
            let registration = if route == "/api/status/tailscale/config" {
                format!(".route(\"{route}\", get(homeserver_rust_read_route).post(network_identity_mutation_route))")
            } else if matches!(route, "/api/network/notes" | "/api/wakeonlan/targets" | "/api/dhcp/reservations" | "/api/dhcp/config" | "/api/dhcp/pool-boundary") {
                format!(".route(\"{route}\", get(homeserver_rust_read_route)")
            } else {
                format!(".route(\"{route}\", get(homeserver_rust_read_route))")
            };
            assert!(source.contains(&registration), "{route} is no longer registered as a generic read membrane; perform live-field projection census before widening FIELD-003");
        }
        let stats_source = std::fs::read_to_string("src/bands/crown-law/stats-tabbar.rs").unwrap();
        assert!(stats_source.contains("fn stats_kea_leases() -> Vec<StatsKeaLease>"), "the only live Kea fact collector citation moved");
    }

    #[tokio::test]
    async fn field_003_route_membrane_wall_generic_reads_return_only_route_membrane_shape() {
        let router = app(AppState { tab_root: Arc::new(test_tab_root("field-003-generic-reads")) });
        for route in field_003_in_scope_read_routes() {
            let token = authorize_test_admin_token();
            let response = router.clone().oneshot(Request::builder().uri(route).header("X-Admin-Token", token).body(Body::empty()).unwrap()).await.unwrap();
            assert_eq!(response.status(), StatusCode::OK, "{route}");
            let body = String::from_utf8(axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
            assert!(body.contains("\"schema\":\"coronatio.homeserver.route.read.v1\""), "{route}: {body}");
            assert!(body.contains("\"family\":\"network-control\""), "{route}: {body}");
            for denied in ["DENY-", "tailnet", "loginUrl", "interface", "hostname", "mac", "clientId", "reservationId", "wakeTargets"] {
                assert!(!body.contains(denied), "generic route membrane leaked identity marker {denied} on {route}: {body}");
            }
        }
    }

    #[test]
    fn field_003_guest_type_purity_wall_no_network_identity_guest_projection_exists_for_generic_scope() {
        let source = std::fs::read_to_string("src/bands/contracts/core.rs").unwrap();
        for forbidden in ["TailscaleGuestProjection", "DhcpGuestProjection", "KeaLeasesGuestProjection", "WakeOnLanGuestProjection"] {
            assert!(!source.contains(forbidden), "generic FIELD-003 scope grew a typed projection without a live fact census: {forbidden}");
        }
    }

    #[test]
    fn field_003_totality_wall_all_in_scope_mutations_use_session_membrane() {
        let source = std::fs::read_to_string("src/bands/full-rust-routes.rs").unwrap();
        assert!(source.contains("fn network_identity_mutation_refusal_response"));
        for (method, route) in field_003_in_scope_mutations() {
            let method_call = match method { "POST" => "post", "PUT" => "put", "DELETE" => "delete", _ => unreachable!() };
            let registration = if route.contains("fixture-target") {
                ".route(\"/api/wakeonlan/targets/:name\", delete(network_identity_mutation_route))".to_string()
            } else if route.contains("fixture-reservation") && method == "PUT" {
                ".route(\"/api/dhcp/reservations/:reservation_id\", put(network_identity_mutation_route).delete(network_identity_mutation_route))".to_string()
            } else if route.contains("fixture-reservation") {
                ".route(\"/api/dhcp/reservations/:reservation_id\", put(network_identity_mutation_route).delete(network_identity_mutation_route))".to_string()
            } else if matches!(route, "/api/status/tailscale/config" | "/api/network/notes" | "/api/wakeonlan/targets" | "/api/dhcp/reservations" | "/api/dhcp/config" | "/api/dhcp/pool-boundary") {
                format!(".route(\"{route}\", get(homeserver_rust_read_route).{method_call}(network_identity_mutation_route))")
            } else {
                format!(".route(\"{route}\", {method_call}(network_identity_mutation_route))")
            };
            assert!(source.contains(&registration), "{method} {route} is not wired through network_identity_mutation_route");
        }
    }

    #[tokio::test]
    async fn field_003_mutation_refusal_wall_network_identity_mutations_refuse_guest_sessions() {
        let router = app(AppState { tab_root: Arc::new(test_tab_root("field-003-guest-mutation-refusal")) });
        for (method, route) in field_003_in_scope_mutations() {
            let response = router.clone().oneshot(Request::builder().method(method).uri(route).body(Body::empty()).unwrap()).await.unwrap();
            assert_eq!(response.status(), StatusCode::UNAUTHORIZED, "{method} {route}");
            let body = String::from_utf8(axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
            assert!(body.contains("coronatio.network-identity.mutation.refusal.v1"), "{method} {route}: {body}");
            assert!(body.contains("admin-session-required"), "{method} {route}: {body}");
            assert!(body.contains("\"accepted\":false"), "{method} {route}: {body}");
        }
    }

    #[tokio::test]
    async fn field_003_admin_mutation_wall_reaches_caduceus_membrane_after_session_check() {
        std::env::set_var("CADUCEUS_URL", "http://127.0.0.1:9");
        let router = app(AppState { tab_root: Arc::new(test_tab_root("field-003-admin-mutation-membrane")) });
        let token = authorize_test_admin_token();
        let response = router.oneshot(Request::builder().method("POST").uri("/api/wakeonlan/wake").header("X-Admin-Token", token).body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
        let body = String::from_utf8(axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        assert!(body.contains("coronatio.homeserver.route.mutation.v1"), "{body}");
        assert!(body.contains("Caduceus staff intent membrane"), "{body}");
        assert!(body.contains("caduceus-unreachable"), "{body}");
        std::env::remove_var("CADUCEUS_URL");
    }

    #[test]
    fn field_003_audit_wall_birth_under_projection_rows_are_citation_true() {
        let audit = std::fs::read_to_string("docs/field-projection-audit.md").unwrap();
        for required in [
            "FIELD-003 finding: no in-scope tailscale, DHCP/Kea, network notes, or WOL route serves a domain fact body today",
            "tailscale_status",
            "deferred-until-ported under BIRTH-UNDER-PROJECTION",
            "DHCP/Kea/WOL identity surfaces",
            "generic route membrane only; mutation guest refusal now stands",
            "src/bands/full-rust-routes.rs:82-89",
            "src/bands/full-rust-routes.rs:118-132",
            "src/bands/crown-law/stats-tabbar.rs:534-570",
        ] {
            assert!(audit.contains(required), "FIELD-003 audit amendment missing {required}");
        }
    }
