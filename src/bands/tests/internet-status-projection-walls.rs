    fn maximal_internet_status_fixture() -> InternetStatusSnapshot {
        InternetStatusSnapshot {
            schema: "coronatio.internet.status.v1".to_string(),
            ok: true,
            success: true,
            status: "connected".to_string(),
            timestamp: 1783380000.25,
            authority: "DENY-authority-provenance-marker".to_string(),
            hosts: vec!["DENY-1.1.1.1".to_string(), "DENY-8.8.8.8".to_string(), "DENY-208.67.222.222".to_string()],
            timeout_seconds: 3,
            first_missing_signal: "none".to_string(),
        }
    }

    #[test]
    fn field_001r_census_wall_every_admin_internet_field_has_one_bucket() {
        let raw = maximal_internet_status_fixture();
        let admin = serde_json::to_value(project_internet_status_admin(&raw)).unwrap();
        let admin_fields = json_field_census(&admin);
        let guest_projected = ["schema", "ok", "success", "status", "timestamp", "firstMissingSignal"];
        let named_deny = ["hosts", "timeoutSeconds", "authority"];
        let mut bucketed = guest_projected
            .iter()
            .chain(named_deny.iter())
            .map(|field| (*field).to_string())
            .collect::<Vec<_>>();
        bucketed.sort();
        assert_eq!(admin_fields, bucketed);
    }

    #[test]
    fn field_001r_deny_marker_fixture_wall_guest_serialization_is_clean() {
        let raw = maximal_internet_status_fixture();
        let guest = project_internet_status_guest(&raw);
        let body = serde_json::to_string(&guest).unwrap();
        for marker in [
            "DENY-authority-provenance-marker",
            "DENY-1.1.1.1",
            "DENY-8.8.8.8",
            "DENY-208.67.222.222",
            "hosts",
            "timeoutSeconds",
            "authority",
            "DENY-",
        ] {
            assert!(!body.contains(marker), "guest internet projection leaked denied marker {marker}: {body}");
        }
        assert!(body.contains("\"schema\":\"coronatio.internet.status.v1\""), "{body}");
        assert!(body.contains("\"status\":\"connected\""), "{body}");
    }

    #[test]
    fn field_001r_guest_type_purity_wall_cannot_represent_denied_fields() {
        let value = serde_json::to_value(project_internet_status_guest(&maximal_internet_status_fixture())).unwrap();
        let census = json_field_census(&value);
        for denied in ["hosts", "timeoutSeconds", "authority"] {
            assert!(!census.iter().any(|field| field == denied), "guest type can represent denied field {denied}: {census:?}");
        }
        assert_eq!(census, vec!["firstMissingSignal", "ok", "schema", "status", "success", "timestamp"]);
    }

    #[tokio::test]
    async fn field_001r_route_membrane_wall_status_projects_by_session_headers() {
        let router = app(AppState { tab_root: Arc::new(test_tab_root("field-001r-internet-status-route")) });
        let guest = router
            .clone()
            .oneshot(Request::builder().uri("/api/status").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(guest.status(), StatusCode::OK);
        let guest_body = String::from_utf8(axum::body::to_bytes(guest.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        for denied in ["\"hosts\"", "\"timeoutSeconds\"", "\"authority\"", "DENY-"] {
            assert!(!guest_body.contains(denied), "guest /api/status leaked {denied}: {guest_body}");
        }
        for expected in ["\"schema\"", "\"ok\"", "\"success\"", "\"status\"", "\"timestamp\"", "\"firstMissingSignal\""] {
            assert!(guest_body.contains(expected), "guest /api/status omitted {expected}: {guest_body}");
        }

        let token = authorize_test_admin_token();
        let admin = router
            .oneshot(Request::builder().uri("/api/status").header("X-Admin-Token", token).body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(admin.status(), StatusCode::OK);
        let admin_body = String::from_utf8(axum::body::to_bytes(admin.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        for expected in [
            "\"schema\"",
            "\"ok\"",
            "\"success\"",
            "\"status\"",
            "\"timestamp\"",
            "\"authority\"",
            "\"hosts\"",
            "\"timeoutSeconds\"",
            "\"firstMissingSignal\"",
            "1.1.1.1",
            "8.8.8.8",
            "208.67.222.222",
            "Coronatio Rust route port of Flask InternetStatusMonitor.check_connectivity",
        ] {
            assert!(admin_body.contains(expected), "admin /api/status omitted og field {expected}: {admin_body}");
        }
    }

    #[test]
    fn field_001r_totality_wall_projectors_do_not_widen_or_drop_lifecycle_fields() {
        let raw = maximal_internet_status_fixture();
        let guest = project_internet_status_guest(&raw);
        let admin = project_internet_status_admin(&raw);
        assert_eq!(guest.schema, raw.schema);
        assert_eq!(guest.ok, raw.ok);
        assert_eq!(guest.success, raw.success);
        assert_eq!(guest.status, raw.status);
        assert_eq!(guest.timestamp, raw.timestamp);
        assert_eq!(guest.first_missing_signal, raw.first_missing_signal);
        assert_eq!(admin.hosts, raw.hosts);
        assert_eq!(admin.timeout_seconds, raw.timeout_seconds);
        assert_eq!(admin.authority, raw.authority);
        let guest_value = serde_json::to_value(guest).unwrap();
        assert!(guest_value.get("timestamp").is_some());
        assert!(guest_value.get("firstMissingSignal").is_some());
    }
