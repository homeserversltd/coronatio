    fn field_005b_typed_projection_get_routes() -> Vec<&'static str> {
        vec![
            "/api/portals",
            "/api/portals/elements",
            "/api/stats/elements",
            "/api/status",
            "/api/tabs/elements/:tab_id",
            "/api/dhcp/status",
            "/api/dhcp/leases",
            "/api/dhcp/reservations",
            "/api/dhcp/config",
            "/api/dhcp/health",
            "/api/dhcp/statistics",
            "/api/dhcp/pool-boundary",
        ]
    }

    fn field_005b_real_body_exception_get_routes() -> Vec<&'static str> {
        vec![
            "/api/files/browse",
            "/api/files/browse-hierarchical",
            "/api/portals/factory",
            "/api/portals/images/:filename",
            "/api/status/power/usage",
            "/api/themes",
            "/api/upload/default-directory",
            "/api/upload/pin-required-status",
            "/api/uptime",
            "/status/power/usage",
        ]
    }

    fn field_005b_og_admin_gated_get_routes() -> Vec<&'static str> {
        vec![
            "/api/upload/blacklist/list",
            "/api/upload/history",
        ]
    }

    fn field_005b_generic_projected_get_routes() -> Vec<&'static str> {
        vec![
            "/api/vault/status",
            "/api/admin/logs/homeserver",
            "/api/admin/download-root-crt",
            "/api/crypto/getKey",
            "/api/admin/updates/check",
            "/api/admin/updates/modules",
            "/api/admin/updates/modules/:module_name/status",
            "/api/admin/updates/interactives",
            "/api/admin/updates/logs",
            "/api/admin/updates/logfile",
            "/api/admin/updates/system-info",
            "/api/admin/updates/schedule",
            "/api/admin/ssh/status",
            "/api/admin/ssh/service/status",
            "/api/admin/samba/service/status",
            "/api/admin/hard-drive-test/results",
            "/api/admin/hard-drive-test/progress",
            "/api/admin/hard-drive-test/devices",
            "/api/admin/diskman/nas-compatible",
            "/api/admin/diskman/check-services",
            "/api/admin/diskman/sync-schedule",
            "/api/admin/diskman/vault-device",
            "/api/status/services",
            "/api/status/tailscale",
            "/api/status/tailscale/config",
            "/api/status/vpn/pia",
            "/api/status/vpn/transmission",
            "/api/status/vpn/pia/exists",
            "/api/status/vpn/transmission/exists",
            "/api/status/vpn/check-enabled",
            "/api/files/download",
            "/api/kea-leases",
            "/api/network/notes",
            "/api/version",
            "/api/wakeonlan/targets",
            "/api/wakeonlan/status",
            "/api/nasLinker/browse",
            "/api/nasLinker/scan",
            "/api/nasLinker/status",
            "/api/nasLinker/config",
            "/api/backup/status",
            "/api/backup/repositories",
            "/api/backup/providers/status",
            "/api/backup/config",
            "/api/backup/history",
            "/api/backup/backup/list/:provider_name",
            "/api/backup/schedule",
            "/api/backup/providers/schema",
            "/api/backup/providers/:provider_name/config",
            "/api/backup/providers/:provider_name/info",
            "/api/backup/statistics",
            "/api/backup/schedule/history",
            "/api/backup/schedule/templates",
            "/api/backup/schedule/cron/available",
            "/api/backup/version",
            "/api/backup/auto-update/status",
            "/api/backup/keyman/services",
            "/api/backup/keyman/credentials/:service_name",
            "/api/backup/keyman/check/:service_name",
            "/api/backup/keyman/providers",
            "/api/backup/debug/status",
            "/api/backup/header-stats",
            "/api/backup/backups/list",
            "/api/backblazeTab/browse",
            "/api/backblazeTab/buckets",
            "/api/backblazeTab/buckets/:bucket_id/tree",
            "/api/backblazeTab/buckets/:bucket_id/files",
            "/api/backblazeTab/buckets/:bucket_id/storage",
            "/api/backblazeTab/buckets/:bucket_id/sync/status",
            "/api/backblazeTab/buckets/:bucket_id/sync/config",
            "/api/backblazeTab/ledger/events",
            "/api/backblazeTab/ledger/jobs/:job_id",
            "/api/backblazeTab/database/list",
            "/api/backblazeTab/forgejo/status",
            "/api/backblazeTab/forgejo/backups",
            "/api/backblazeTab/chunk-store",
            "/api/backblazeTab/chunks-registry",
            "/api/miner/coins",
            "/api/miner/miners",
            "/api/miner/miners/:miner_id/coins/:coin_id/status",
            "/api/miner/config",
            "/api/miner/stats",
            "/api/test/status",
            "/api/test/data/sample",
            "/api/test/external/fetch",
            "/api/test/config",
            "/api/test/health",
            "/api/conflict/status",
        ]
    }

    #[test]
    fn field_005b_read_census_wall_all_full_rust_get_routes_are_classified_once() {
        let mut all = Vec::new();
        for (path, methods) in full_rust_route_inventory() {
            if methods.contains(&"get") { all.push(*path); }
        }
        let typed = field_005b_typed_projection_get_routes();
        let generic = field_005b_generic_projected_get_routes();
        let real = field_005b_real_body_exception_get_routes();
        let og = field_005b_og_admin_gated_get_routes();
        assert_eq!(all.len(), 112, "new GET route entered full-rust-routes.rs and must be FIELD-005b-classified");
        assert_eq!(typed.len(), 12, "typed-projection bucket changed");
        assert_eq!(generic.len(), 88, "generic-projected-this-slice bucket changed");
        assert_eq!(real.len(), 10, "real-body-exception bucket changed");
        assert_eq!(og.len(), 2, "og-admin-gated GET bucket changed");

        let mut bucketed = std::collections::BTreeMap::new();
        for (bucket, routes) in [
            ("typed-projection", typed.clone()),
            ("generic-projected-this-slice", generic.clone()),
            ("real-body-exception", real.clone()),
            ("og-admin-gated", og.clone()),
        ] {
            for route in routes {
                let prior = bucketed.insert(route, bucket);
                assert!(prior.is_none(), "{route} appears in more than one read bucket: {prior:?} and {bucket}");
            }
        }
        for route in &all {
            assert!(bucketed.contains_key(route), "{route} has no FIELD-005b GET bucket");
        }
        for route in bucketed.keys() {
            assert!(all.contains(route), "bucketed route {route} is not in full_rust_route_inventory");
        }
    }

    #[test]
    fn field_005b_source_wall_generic_births_use_session_projected_membrane_and_exceptions_do_not() {
        let source = std::fs::read_to_string("src/bands/full-rust-routes.rs").unwrap();
        for route in field_005b_generic_projected_get_routes() {
            let line = source.lines().find(|line| line.contains(&format!(".route(\"{route}\""))).unwrap_or_else(|| panic!("missing route registration for {route}"));
            assert!(line.contains("get(homeserver_rust_read_route)"), "generic projected route was not born through the session-projected membrane: {line}");
        }
        for route in field_005b_typed_projection_get_routes().into_iter().chain(field_005b_real_body_exception_get_routes()).chain(field_005b_og_admin_gated_get_routes()) {
            let Some(line) = source.lines().find(|line| line.contains(&format!(".route(\"{route}\""))) else { continue; };
            if route == "/status/power/usage" || route == "/api/status/power/usage" {
                assert!(line.contains("homeserver_rust_read_route"), "power route must stay on read membrane for its real-body exception: {line}");
            } else {
                assert!(!line.contains("get(homeserver_rust_read_route)"), "exception route still uses generic read membrane: {line}");
            }
        }
    }

    #[tokio::test]
    async fn field_005b_guest_generic_readback_is_minimal_and_topology_free() {
        let router = app(AppState { tab_root: Arc::new(test_tab_root("field-005b-guest-generic")) });
        for route in ["/api/admin/updates/modules/example/status", "/api/status/tailscale"] {
            let response = router.clone().oneshot(Request::builder().uri(route).body(Body::empty()).unwrap()).await.unwrap();
            assert_eq!(response.status(), StatusCode::OK, "{route}");
            let body: serde_json::Value = serde_json::from_slice(&axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap()).unwrap();
            assert_eq!(body["schema"], "coronatio.homeserver.route.read.guest.v1", "{route}: {body}");
            let keys = body.as_object().unwrap().keys().cloned().collect::<Vec<_>>();
            assert_eq!(keys, vec!["firstMissingSignal", "ok", "schema", "status", "success"], "guest generic read grew topology fields: {body}");
            let raw = serde_json::to_string(&body).unwrap();
            for denied in ["method", "path", "family", "authority", "network-control", "crown-pane", route, "DENY-topology-marker"] {
                assert!(!raw.contains(denied), "guest generic read leaked topology marker {denied}: {raw}");
            }
        }
    }

    #[tokio::test]
    async fn field_005b_admin_generic_readback_preserves_full_topology_shape() {
        let router = app(AppState { tab_root: Arc::new(test_tab_root("field-005b-admin-generic")) });
        let response = router.oneshot(successor_admin_request(Request::builder().uri("/api/admin/updates/modules/example/status").body(Body::empty()).unwrap())).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = String::from_utf8(axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        for expected in [
            "coronatio.homeserver.route.read.v1",
            "\"method\":\"GET\"",
            "\"path\":\"/api/admin/updates/modules/example/status\"",
            "\"family\":\"update-and-backup\"",
            "\"authority\":\"Coronatio Rust route\"",
            "\"firstMissingSignal\":\"none\"",
        ] {
            assert!(body.contains(expected), "admin generic read lost full topology marker {expected}: {body}");
        }
    }

    #[tokio::test]
    async fn field_005b_og_upload_admin_mutations_refuse_cross_origin_and_same_origin_guest_sessions_and_preserve_named_guest_routes() {
        let router = app(AppState { tab_root: Arc::new(test_tab_root("field-005b-upload-gates")) });
        for (method, route) in [
            ("GET", "/api/upload/history"),
            ("GET", "/api/upload/blacklist/list"),
        ] {
            let response = router.clone().oneshot(Request::builder().method(method).uri(route).body(Body::empty()).unwrap()).await.unwrap();
            assert_eq!(response.status(), StatusCode::UNAUTHORIZED, "{method} {route}");
        }
        for (method, route) in [
            ("POST", "/api/upload/history/clear"),
            ("PUT", "/api/upload/blacklist/update"),
            ("POST", "/api/upload/force-permissions"),
            ("POST", "/api/upload/pin-required-status"),
        ] {
            let mark = crate::caduceus_access::test_fixture::mark();
            let response = router.clone().oneshot(Request::builder().method(method).uri(route).body(Body::empty()).unwrap()).await.unwrap();
            assert_eq!(response.status(), StatusCode::FORBIDDEN, "{method} {route}");
            let body = String::from_utf8(axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
            assert!(body.contains("caduceus-access-origin-refused"), "{method} {route}: {body}");
            assert!(body.contains("\"ok\":false"), "{method} {route}: {body}");
            assert!(crate::caduceus_access::test_fixture::records_since(mark).is_empty(), "{method} {route}");

            let mark = crate::caduceus_access::test_fixture::mark();
            let response = router.clone().oneshot(successor_session_request(Request::builder().method(method).uri(route).body(Body::empty()).unwrap(), false)).await.unwrap();
            assert_eq!(response.status(), StatusCode::UNAUTHORIZED, "{method} {route}");
            let body = String::from_utf8(axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
            assert!(body.contains("caduceus-access-session-required"), "{method} {route}: {body}");
            assert!(body.contains("\"ok\":false"), "{method} {route}: {body}");
            assert!(crate::caduceus_access::test_fixture::records_since(mark).is_empty(), "{method} {route}");
        }
        for (method, route) in [
            ("GET", "/api/files/browse"),
            ("GET", "/api/files/browse-hierarchical"),
            ("POST", "/api/files/upload"),
            ("GET", "/api/upload/default-directory"),
            ("GET", "/api/upload/pin-required-status"),
        ] {
            let response = router.clone().oneshot(Request::builder().method(method).uri(route).body(Body::empty()).unwrap()).await.unwrap();
            assert_ne!(response.status(), StatusCode::UNAUTHORIZED, "{method} {route}");
        }
    }

    #[tokio::test]
    async fn field_005b_admin_upload_history_crosses_og_gate() {
        let _guard = CADUCEUS_ENV_LOCK.get_or_init(|| std::sync::Mutex::new(())).lock().unwrap();
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        let payload = serde_json::json!({
            "schema": "caduceus.hyalos.tail.v1",
            "ok": true,
            "events": [{"kind": "upload", "organ": "file-ingress", "message": "accepted", "ok": true}],
            "firstMissingSignal": "none"
        }).to_string();
        let server = std::thread::spawn(move || {
            if let Ok((mut stream, _)) = listener.accept() {
                let mut buf = [0u8; 4096];
                let _ = std::io::Read::read(&mut stream, &mut buf);
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nConnection: close\r\nContent-Length: {}\r\n\r\n{}",
                    payload.len(),
                    payload
                );
                let _ = std::io::Write::write_all(&mut stream, response.as_bytes());
            }
        });
        std::env::set_var("CADUCEUS_BASE_URL", format!("http://127.0.0.1:{port}"));
        let router = app(AppState { tab_root: Arc::new(test_tab_root("field-005b-admin-upload-history")) });
        let response = router.oneshot(successor_admin_request(Request::builder().uri("/api/upload/history").body(Body::empty()).unwrap())).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = String::from_utf8(axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        assert!(body.contains("coronatio.upload.history.v1"), "{body}");
        std::env::remove_var("CADUCEUS_BASE_URL");
        let _ = server.join();
    }
