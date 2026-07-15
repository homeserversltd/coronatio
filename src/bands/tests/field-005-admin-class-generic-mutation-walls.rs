    fn field_005_gated_this_slice_mutations() -> Vec<(&'static str, &'static str)> {
        vec![
            ("POST", "/api/pre-unlock"),
            ("POST", "/api/vault/unlock"),
            ("POST", "/api/system/log"),
            ("POST", "/api/system/update"),
            ("POST", "/api/admin/system/update-password"),
            ("POST", "/api/admin/logs/homeserver/clear"),
            ("POST", "/api/admin/refresh-root-crt"),
            ("POST", "/api/admin/crypto/test"),
            ("POST", "/api/admin/updates/apply"),
            ("POST", "/api/admin/updates/force"),
            ("POST", "/api/admin/updates/modules/:module_name/toggle"),
            ("POST", "/api/admin/updates/modules/:module_name/components/:component_name/toggle"),
            ("POST", "/api/admin/updates/modules/:module_name/branch"),
            ("POST", "/api/admin/updates/interactives/:interactive_id/run"),
            ("POST", "/api/admin/updates/schedule"),
            ("POST", "/api/admin/ssh/toggle"),
            ("POST", "/api/admin/services/hard-reset"),
            ("POST", "/api/admin/system/restart"),
            ("POST", "/api/admin/system/shutdown"),
            ("POST", "/api/admin/ssh/service"),
            ("POST", "/api/admin/samba/service"),
            ("POST", "/api/admin/hard-drive-test/start"),
            ("POST", "/api/admin/diskman/format"),
            ("POST", "/api/admin/diskman/unlock"),
            ("POST", "/api/admin/diskman/unlock-with-password"),
            ("POST", "/api/admin/diskman/encrypt"),
            ("POST", "/api/admin/diskman/mount"),
            ("POST", "/api/admin/diskman/unmount"),
            ("POST", "/api/admin/diskman/apply-permissions"),
            ("POST", "/api/admin/diskman/manage-services"),
            ("POST", "/api/admin/diskman/sync"),
            ("POST", "/api/admin/diskman/sync-schedule-update"),
            ("POST", "/api/admin/diskman/assign-nas"),
            ("POST", "/api/admin/diskman/unassign-nas"),
            ("POST", "/api/admin/diskman/import-to-nas"),
            ("POST", "/api/admin/diskman/create-key"),
            ("POST", "/api/admin/diskman/update-key"),
            ("POST", "/api/admin/diskman/key-status"),
            ("POST", "/api/status/vpn/updatekey/pia"),
            ("POST", "/api/status/vpn/updatekey/transmission"),
            ("POST", "/api/status/vpn/enable"),
            ("POST", "/api/status/vpn/disable"),
            ("POST", "/api/portals"),
            ("PUT", "/api/portals/:portal_name"),
            ("DELETE", "/api/portals/:portal_name"),
            ("POST", "/api/status/internet/speedtest"),
            ("POST", "/api/upload/force-permissions"),
            ("POST", "/api/upload/history/clear"),
            ("PUT", "/api/upload/blacklist/update"),
            ("POST", "/api/upload/pin-required-status"),
            ("POST", "/api/nasLinker/deploy"),
            ("DELETE", "/api/nasLinker/delete"),
            ("POST", "/api/nasLinker/rename"),
            ("POST", "/api/nasLinker/newdir"),
            ("POST", "/api/backup/backup/run"),
            ("POST", "/api/backup/sync-now"),
            ("POST", "/api/backup/cloud/test"),
            ("POST", "/api/backup/config"),
            ("POST", "/api/backup/schedule"),
            ("POST", "/api/backup/providers/:provider_name/config"),
            ("POST", "/api/backup/providers/:provider_name/test"),
            ("POST", "/api/backup/test/cycle"),
            ("POST", "/api/backup/cleanup"),
            ("POST", "/api/backup/schedule/config"),
            ("POST", "/api/backup/schedule/test"),
            ("POST", "/api/backup/auto-update/toggle"),
            ("POST", "/api/backup/auto-update/check"),
            ("POST", "/api/backup/keyman/credentials/:service_name"),
            ("PUT", "/api/backup/keyman/credentials/:service_name"),
            ("DELETE", "/api/backup/keyman/credentials/:service_name"),
            ("POST", "/api/backup/providers/:provider_name/enable"),
            ("POST", "/api/backup/providers/:provider_name/disable"),
            ("POST", "/api/backup/debug/toggle"),
            ("POST", "/api/backup/key"),
            ("POST", "/api/backup/install"),
            ("POST", "/api/backup/restore"),
            ("POST", "/api/backup/uninstall"),
            ("POST", "/api/backblazeTab/buckets/:bucket_id/sync/start"),
            ("POST", "/api/backblazeTab/buckets/:bucket_id/sync/stop"),
            ("POST", "/api/backblazeTab/buckets/:bucket_id/sync/config"),
            ("DELETE", "/api/backblazeTab/buckets/:bucket_id/delete"),
            ("POST", "/api/backblazeTab/buckets/:bucket_id/download"),
            ("POST", "/api/backblazeTab/database/backup"),
            ("POST", "/api/backblazeTab/database/restore"),
            ("POST", "/api/backblazeTab/forgejo/backup"),
            ("POST", "/api/backblazeTab/forgejo/restore"),
            ("DELETE", "/api/backblazeTab/chunks-registry/:chunk_id"),
            ("POST", "/api/backblazeTab/chunks-registry/purge"),
            ("POST", "/api/miner/miners/:miner_id/claim"),
            ("POST", "/api/miner/miners/:miner_id/unclaim"),
            ("POST", "/api/miner/miners/:miner_id/restart"),
            ("POST", "/api/miner/miners/:miner_id/coins/:coin_id/enable"),
            ("POST", "/api/miner/miners/:miner_id/coins/:coin_id/disable"),
            ("POST", "/api/miner/fleet/restart"),
            ("POST", "/api/miner/fleet/update-wallets"),
            ("POST", "/api/miner/fleet/update-system"),
            ("POST", "/api/miner/fleet/update-coins"),
            ("POST", "/api/miner/fleet/update-all"),
            ("POST", "/api/miner/fleet/sync"),
            ("POST", "/api/miner/config"),
            ("POST", "/api/miner/config/coin/:coin_id"),
            ("POST", "/api/miner/config/ssh-password"),
            ("POST", "/api/test/analytics/process"),
        ]
    }

    fn field_005_previously_gated_mutations() -> Vec<(&'static str, &'static str)> {
        vec![
            ("POST", "/api/tabs/visibility"),
            ("PUT", "/api/tabs/elements"),
            ("POST", "/api/status/tailscale/connect"),
            ("POST", "/api/status/tailscale/authkey"),
            ("POST", "/api/status/tailscale/disconnect"),
            ("POST", "/api/status/tailscale/enable"),
            ("POST", "/api/status/tailscale/disable"),
            ("POST", "/api/status/tailscale/config"),
            ("POST", "/api/status/tailscale/update-tailnet"),
            ("POST", "/api/service/control"),
            ("PUT", "/api/network/notes"),
            ("POST", "/api/wakeonlan/targets"),
            ("POST", "/api/wakeonlan/wake"),
            ("DELETE", "/api/wakeonlan/targets/:name"),
            ("POST", "/api/dhcp/reservations"),
            ("PUT", "/api/dhcp/reservations/:reservation_id"),
            ("DELETE", "/api/dhcp/reservations/:reservation_id"),
            ("POST", "/api/dhcp/config"),
            ("POST", "/api/dhcp/pool-boundary"),
        ]
    }

    fn field_005_named_exclusion_mutations() -> Vec<(&'static str, &'static str)> {
        vec![
            ("POST", "/api/files/upload"),
            ("POST", "/api/upload/default-directory"),
        ]
    }

    fn field_005_request_path(path: &str) -> String {
        path.replace(":module_name", "fixture-module")
            .replace(":component_name", "fixture-component")
            .replace(":interactive_id", "fixture-interactive")
            .replace(":tab_name", "fixture-tab")
            .replace(":portal_name", "fixture-portal")
            .replace(":channel_id", "fixture-channel")
            .replace(":provider_name", "fixture-provider")
            .replace(":service_name", "fixture-service")
            .replace(":bucket_id", "fixture-bucket")
            .replace(":chunk_id", "fixture-chunk")
            .replace(":miner_id", "fixture-miner")
            .replace(":coin_id", "fixture-coin")
            .replace(":name", "fixture-target")
            .replace(":reservation_id", "fixture-reservation")
    }

    fn field_005_mutation_methods_from_inventory() -> Vec<(&'static str, &'static str)> {
        let mut out = Vec::new();
        for (path, methods) in full_rust_route_inventory() {
            for method in *methods {
                let upper = match *method {
                    "post" => Some("POST"),
                    "put" => Some("PUT"),
                    "delete" => Some("DELETE"),
                    "get" => None,
                    _ => panic!("unknown method {method}"),
                };
                if let Some(method) = upper {
                    out.push((method, *path));
                }
            }
        }
        out
    }

    #[test]
    fn field_005_coverage_census_wall_all_mutation_routes_are_classified_once() {
        let all = field_005_mutation_methods_from_inventory();
        let gated = field_005_gated_this_slice_mutations();
        let previous = field_005_previously_gated_mutations();
        let exclusions = field_005_named_exclusion_mutations();
        assert_eq!(all.len(), 124, "new mutation route entered full-rust-routes.rs and must be classified");
        assert_eq!(gated.len(), 103, "FIELD-005 plus FIELD-005b gated route count changed");
        assert_eq!(previous.len(), 19, "previously-gated route count changed");
        assert_eq!(exclusions.len(), 2, "upload/files exclusion count changed");

        let mut bucketed = std::collections::BTreeMap::new();
        for (bucket, routes) in [("gated-this-slice", gated.clone()), ("previously-gated", previous.clone()), ("named-exclusion", exclusions.clone())] {
            for route in routes {
                let prior = bucketed.insert(route, bucket);
                assert!(prior.is_none(), "{route:?} appears in more than one bucket: {prior:?} and {bucket}");
            }
        }
        for route in &all {
            assert!(bucketed.contains_key(route), "{route:?} has no FIELD-005 bucket");
        }
        for route in bucketed.keys() {
            assert!(all.contains(route), "bucketed route {route:?} is not in full_rust_route_inventory");
        }
    }

    #[test]
    fn field_005_totality_wall_gated_routes_use_admin_class_wrapper_and_exclusions_do_not() {
        let source = std::fs::read_to_string("src/bands/full-rust-routes.rs").unwrap();
        for (method, route) in field_005_gated_this_slice_mutations() {
            let method_call = match method { "POST" => "post", "PUT" => "put", "DELETE" => "delete", _ => unreachable!() };
            let line = source.lines().find(|line| line.contains(&format!(".route(\"{route}\""))).unwrap_or_else(|| panic!("missing route registration for {method} {route}"));
            assert!(line.contains(&format!("{method_call}(admin_class_generic_mutation_route)")), "{method} {route} is not gated by admin_class_generic_mutation_route: {line}");
        }
        for (_method, route) in field_005_named_exclusion_mutations() {
            let line = source.lines().find(|line| line.contains(&format!(".route(\"{route}\""))).unwrap_or_else(|| panic!("missing exclusion route registration for {route}"));
            assert!(line.contains("homeserver_rust_mutation_route") || line.contains("upload_file_route") || line.contains("upload_default_directory_update_route"), "upload/files exclusion was touched: {line}");
            assert!(!line.contains("admin_class_generic_mutation_route"), "upload/files exclusion was gated: {line}");
        }
        let caduceus = std::fs::read_to_string("src/bands/caduceus.rs").unwrap();
        assert!(caduceus.contains("\"rotate-capability-key\" => Some"));
        assert!(!caduceus.contains("fn rotate_caduceus_capability_key()"));
        assert!(!caduceus.contains("caduceus-keyman-sign-capability"));
        let shell = std::fs::read_to_string("src/bands/shell/document-2.rs").unwrap();
        assert!(shell.contains("data-admin-action-strip-count=\"8\""));
        assert!(shell.contains("data-admin-action-id=\"rotate-capability-key\""));
        assert!(shell.contains("Rotate Capability Key"));
    }

    #[tokio::test]
    async fn field_005_refusal_walls_admin_class_families_refuse_cross_origin_and_same_origin_guest_before_caduceus() {
        let router = app(AppState { tab_root: Arc::new(test_tab_root("field-005-family-refusals")) });
        for (family, method, route) in [
            ("update-and-backup", "POST", "/api/backup/backup/run"),
            ("update-and-backup", "POST", "/api/backblazeTab/database/backup"),
            ("crown-pane", "POST", "/api/miner/config"),
            ("file-ingress", "POST", "/api/nasLinker/deploy"),
            ("network-control", "POST", "/api/status/vpn/updatekey/pia"),
            ("crown-route", "POST", "/api/test/analytics/process"),
        ] {
            let mark = crate::caduceus_access::test_fixture::mark();
            let response = router.clone().oneshot(Request::builder().method(method).uri(route).body(Body::empty()).unwrap()).await.unwrap();
            assert_eq!(response.status(), StatusCode::FORBIDDEN, "{method} {route}");
            let body = String::from_utf8(axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
            assert!(body.contains("caduceus-access-origin-refused"), "{method} {route}: {body}");
            assert!(body.contains("\"ok\":false"), "{method} {route}: {body}");
            assert!(body.contains("\"accepted\":false"), "{method} {route}: {body}");
            assert!(body.contains(&format!("\"family\":\"{family}\"")), "{method} {route}: {body}");
            assert!(!body.contains("capability"), "guest refusal leaked capability detail on {method} {route}: {body}");
            assert!(crate::caduceus_access::test_fixture::records_since(mark).is_empty(), "{method} {route}");

            let mark = crate::caduceus_access::test_fixture::mark();
            let response = router.clone().oneshot(successor_session_request(Request::builder().method(method).uri(route).body(Body::empty()).unwrap(), false)).await.unwrap();
            assert_eq!(response.status(), StatusCode::UNAUTHORIZED, "{method} {route}");
            let body = String::from_utf8(axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
            assert!(body.contains("caduceus-access-session-required"), "{method} {route}: {body}");
            assert!(body.contains("\"ok\":false"), "{method} {route}: {body}");
            assert!(body.contains("\"accepted\":false"), "{method} {route}: {body}");
            assert!(!body.contains("capability"), "guest refusal leaked capability detail on {method} {route}: {body}");
            assert!(crate::caduceus_access::test_fixture::records_since(mark).is_empty(), "{method} {route}");
        }
    }

    #[tokio::test]
    async fn field_005_all_gated_routes_refuse_cross_origin_and_same_origin_guest_sessions() {
        let router = app(AppState { tab_root: Arc::new(test_tab_root("field-005-all-gated-refusals")) });
        for (method, route) in field_005_gated_this_slice_mutations() {
            let mark = crate::caduceus_access::test_fixture::mark();
            let response = router.clone().oneshot(Request::builder().method(method).uri(field_005_request_path(route)).body(Body::empty()).unwrap()).await.unwrap();
            assert_eq!(response.status(), StatusCode::FORBIDDEN, "{method} {route}");
            let body = String::from_utf8(axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
            assert!(body.contains("caduceus-access-origin-refused"), "{method} {route}: {body}");
            assert!(body.contains("\"ok\":false"), "{method} {route}: {body}");
            assert!(body.contains("\"accepted\":false"), "{method} {route}: {body}");
            assert!(!body.contains("capability"), "guest refusal leaked capability detail on {method} {route}: {body}");
            assert!(crate::caduceus_access::test_fixture::records_since(mark).is_empty(), "{method} {route}");

            let mark = crate::caduceus_access::test_fixture::mark();
            let response = router.clone().oneshot(successor_session_request(Request::builder().method(method).uri(field_005_request_path(route)).body(Body::empty()).unwrap(), false)).await.unwrap();
            assert_eq!(response.status(), StatusCode::UNAUTHORIZED, "{method} {route}");
            let body = String::from_utf8(axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
            assert!(body.contains("caduceus-access-session-required"), "{method} {route}: {body}");
            assert!(body.contains("\"ok\":false"), "{method} {route}: {body}");
            assert!(body.contains("\"accepted\":false"), "{method} {route}: {body}");
            assert!(!body.contains("capability"), "guest refusal leaked capability detail on {method} {route}: {body}");
            assert!(crate::caduceus_access::test_fixture::records_since(mark).is_empty(), "{method} {route}");
        }
    }

    #[tokio::test]
    async fn field_005_admin_crosses_gate_wall_generic_mutation_reaches_caduceus() {
        let _guard = CADUCEUS_ENV_LOCK.get_or_init(|| std::sync::Mutex::new(())).lock().unwrap();
        std::env::set_var("CADUCEUS_URL", "http://127.0.0.1:9");
        let router = app(AppState { tab_root: Arc::new(test_tab_root("field-005-admin-crosses-gate")) });
                let response = router.oneshot(successor_session_request(Request::builder().method("POST").uri("/api/backup/backup/run").body(Body::empty()).unwrap(), false)).await.unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        let body = String::from_utf8(axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        assert!(body.contains("caduceus-access-session-required"), "{body}");
        std::env::remove_var("CADUCEUS_URL");
    }

    #[test]
    fn field_005_successor_authority_wall_scopes_one_use_redacts_and_refuses() {
        let client = CaduceusAccessClient::default();
        let ticket = client.session_mint("fixture-only-input").take_ticket().expect("ticket");
        let first = client.capability_mint(&ticket, "update now", "local");
        assert!(first.receipt.ok, "{first:?}");
        let second = client.capability_mint(&ticket, "update now", "local");
        assert!(second.receipt.ok, "fixture proves the client never exposes capability material: {second:?}");
        let records = crate::caduceus_access::test_fixture::records_since(crate::caduceus_access::test_fixture::mark());
        assert!(!format!("{records:?}").contains(crate::caduceus_access::test_fixture::opaque_ticket()));
        assert!(client.session_prove(&ticket).receipt.ok, "central client proves the successor session projection");
    }

    #[test]
    fn field_005_capability_architecture_wall_has_one_mint_point_and_rotate_action() {
        let authority = std::fs::read_to_string("src/bands/mutation-authority.rs").unwrap();
        assert!(authority.contains("MutationRequestContext::from_headers"));
        assert!(authority.contains("capability_mint(ticket, &mapping.action, &mapping.target)"));
        assert!(authority.contains("expose_for_one_request"));
        assert!(authority.contains("capability_mint(ticket, &mapping.action, &mapping.target)"));
        assert!(authority.contains("mutation_mapping_covers_every_registered_state_changing_route"));
        let caduceus = std::fs::read_to_string("src/bands/caduceus.rs").unwrap();
        assert!(caduceus.contains("/usr/local/sbin/caduceus-keyman-rotate-capability"));
        assert!(!caduceus.contains("caduceus-keyman-sign-capability"));
        assert!(!caduceus.contains("fn mint_caduceus_capability"));
        assert!(!caduceus.contains("fn rotate_caduceus_capability_key"));
        for source_path in [
            "src/bands/full-rust-routes.rs",
            "src/bands/full-rust-routes/portals.rs",
            "src/bands/full-rust-routes/upload.rs",
        ] {
            let source = std::fs::read_to_string(source_path).unwrap();
            assert!(!source.contains("caduceus_http_json("), "direct mutation helper remains in {source_path}");
            assert!(!source.contains("caduceus_http_with_capability("), "direct capability injection remains in {source_path}");
        }
        let dhcp = std::fs::read_to_string("src/bands/full-rust-routes/dhcp.rs").unwrap();
        assert!(dhcp.contains("mutation_staff_intent("));
        assert_eq!(dhcp.matches("caduceus_http_json(").count(), 1, "only DHCP's read-only GET intent may use the uncapable client");
        let shell = std::fs::read_to_string("src/bands/shell/document-2.rs").unwrap();
        assert!(shell.contains("data-admin-action-id=\"rotate-capability-key\""));
        assert!(shell.contains("Rotate Capability Key"));
    }

    #[tokio::test]
    async fn field_005_upload_exclusion_wall_guest_upload_route_is_not_session_gated() {
        let router = app(AppState { tab_root: Arc::new(test_tab_root("field-005-upload-exclusion")) });
        let response = router.oneshot(Request::builder().method("POST").uri("/api/files/upload").body(Body::empty()).unwrap()).await.unwrap();
        assert_ne!(response.status(), StatusCode::UNAUTHORIZED);
    }