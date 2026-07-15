    fn field_003_in_scope_read_routes() -> Vec<&'static str> {
        vec![
            "/api/status/tailscale",
            "/api/status/tailscale/config",
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
            } else if matches!(route, "/api/network/notes" | "/api/wakeonlan/targets") {
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
            let response = router.clone().oneshot(successor_admin_request(Request::builder().uri(route).body(Body::empty()).unwrap())).await.unwrap();
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
            } else if route.contains("fixture-reservation") {
                ".route(\"/api/dhcp/reservations/:reservation_id\", put(dhcp_reservation_update_route).delete(dhcp_reservation_delete_route))".to_string()
            } else if route == "/api/dhcp/reservations" {
                ".route(\"/api/dhcp/reservations\", get(dhcp_read_route).post(dhcp_reservation_create_route))".to_string()
            } else if route == "/api/dhcp/pool-boundary" {
                ".route(\"/api/dhcp/pool-boundary\", get(dhcp_read_route).post(dhcp_pool_boundary_route))".to_string()
            } else if route == "/api/dhcp/config" {
                format!(".route(\"{route}\", get(dhcp_read_route).{method_call}(network_identity_mutation_route))")
            } else if matches!(route, "/api/status/tailscale/config" | "/api/network/notes" | "/api/wakeonlan/targets") {
                format!(".route(\"{route}\", get(homeserver_rust_read_route).{method_call}(network_identity_mutation_route))")
            } else {
                format!(".route(\"{route}\", {method_call}(network_identity_mutation_route))")
            };
            assert!(source.contains(&registration), "{method} {route} is not wired through network_identity_mutation_route");
        }
    }

    #[tokio::test]
    async fn field_003_mutation_refusal_wall_network_identity_mutations_refuse_cross_origin_and_same_origin_guest_sessions() {
        let router = app(AppState { tab_root: Arc::new(test_tab_root("field-003-guest-mutation-refusal")) });
        for (method, route) in field_003_in_scope_mutations() {
            let mark = crate::caduceus_access::test_fixture::mark();
            let response = router.clone().oneshot(Request::builder().method(method).uri(route).body(Body::empty()).unwrap()).await.unwrap();
            assert_eq!(response.status(), StatusCode::FORBIDDEN, "{method} {route}");
            let body = String::from_utf8(axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
            assert!(body.contains("caduceus-access-origin-refused"), "{method} {route}: {body}");
            assert!(body.contains("\"ok\":false"), "{method} {route}: {body}");
            assert!(body.contains("\"accepted\":false"), "{method} {route}: {body}");
            assert!(crate::caduceus_access::test_fixture::records_since(mark).is_empty(), "{method} {route}");

            let mark = crate::caduceus_access::test_fixture::mark();
            let response = router.clone().oneshot(successor_session_request(Request::builder().method(method).uri(route).body(Body::empty()).unwrap(), false)).await.unwrap();
            assert_eq!(response.status(), StatusCode::UNAUTHORIZED, "{method} {route}");
            let body = String::from_utf8(axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
            assert!(body.contains("caduceus-access-session-required"), "{method} {route}: {body}");
            assert!(body.contains("\"ok\":false"), "{method} {route}: {body}");
            assert!(body.contains("\"accepted\":false"), "{method} {route}: {body}");
            assert!(crate::caduceus_access::test_fixture::records_since(mark).is_empty(), "{method} {route}");
        }
    }

    #[tokio::test]
    async fn field_003_admin_mutation_wall_reaches_caduceus_membrane_after_session_check() {
        let router = app(AppState { tab_root: Arc::new(test_tab_root("field-003-admin-mutation-membrane")) });
        let response = router.oneshot(successor_admin_request(Request::builder().method("POST").uri("/api/wakeonlan/wake").body(Body::empty()).unwrap())).await.unwrap();
        assert_eq!(response.status(), StatusCode::ACCEPTED);
        let body = String::from_utf8(axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        assert!(body.contains("coronatio.homeserver.route.mutation.v1"), "{body}");
        assert!(body.contains("Caduceus staff intent membrane"), "{body}");
        assert!(body.contains("caduceus"), "{body}");
    }
