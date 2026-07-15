    fn field_004_service_routes() -> Vec<&'static str> {
        vec!["/api/status/services", "/api/services/data", "/api/service/control"]
    }

    fn maximal_service_data_fixture() -> ServiceDataReadback {
        ServiceDataReadback {
            schema: "coronatio.service-data.contract.v1".to_string(),
            status: "contract-only".to_string(),
            route: "/api/services/data".to_string(),
            portal_schema: PortalSchema {
                source_path: "DENY-homeserver-json-tabs-portals".to_string(),
                fields: vec!["DENY-portal-field".to_string(), "port".to_string()],
                required_fields: vec!["DENY-required".to_string()],
                portal_types: vec!["systemd".to_string()],
                validation_rules: vec![ValidationRule { field: "DENY-field".to_string(), rule: "DENY-rule".to_string() }],
                factory_portal_law: "DENY-factory-law".to_string(),
            },
            service_card_schema: ServiceCardSchema {
                source_paths: vec!["DENY-source-path".to_string()],
                fields: vec!["name".to_string(), "systemdName".to_string(), "isEnabled".to_string(), "port".to_string(), "statusDetails".to_string(), "isScriptManaged".to_string(), "needsReboot".to_string()],
                systemd_resolution: "DENY-systemd-resolution".to_string(),
                script_managed_resolution: "DENY-script-resolution".to_string(),
                enabled_cache_policy: "DENY-enabled-cache".to_string(),
            },
            monitor_topics: vec![MonitorTopicLaw {
                topic: "services.status".to_string(),
                source_monitor: "DENY-ServicesMonitor.broadcast_status".to_string(),
                cadence_source: "DENY-SERVICES_CHECK_INTERVAL".to_string(),
                payload_fields: vec!["name".to_string(), "systemdName".to_string(), "port".to_string(), "statusDetails".to_string(), "isScriptManaged".to_string(), "needsReboot".to_string()],
                admin_only: false,
                admin_fields: vec!["isEnabled".to_string()],
                change_rule: "DENY-service-count-name-status-enabled".to_string(),
                coronatio_contract: "DENY-service-card-readback".to_string(),
            }],
            broadcast_law: BroadcastLaw {
                transport_replacement: "DENY-transport".to_string(),
                regular_delivery: "DENY-regular".to_string(),
                admin_delivery: "DENY-admin".to_string(),
                change_detection: "DENY-change".to_string(),
                ui_state_law: "DENY-ui".to_string(),
            },
            admin_runtime: AdminRuntimeReadback {
                devices: Vec::new(),
                mount_destinations: Vec::new(),
                services: vec![
                    AdminServiceStateReadback { id: "DENY-ssh-service".to_string(), label: "DENY-SSH Service".to_string(), enabled: true, state: "Running".to_string(), source: "DENY-systemctl ssh.service".to_string() },
                    AdminServiceStateReadback { id: "DENY-samba-file-sharing".to_string(), label: "DENY-Samba".to_string(), enabled: false, state: "Stopped".to_string(), source: "DENY-systemctl smbd.service".to_string() },
                    AdminServiceStateReadback { id: "DENY-unknown".to_string(), label: "DENY-Unknown".to_string(), enabled: false, state: "Unavailable".to_string(), source: "DENY-systemctl missing.service".to_string() },
                ],
                source: "DENY-/proc/mounts-and-systemctl".to_string(),
            },
            first_missing_live_signal: "none".to_string(),
        }
    }

    #[test]
    fn field_004_census_wall_routes_are_classified_with_citations() {
        let full = std::fs::read_to_string("src/bands/full-rust-routes.rs").unwrap();
        let runtime = std::fs::read_to_string("src/bands/runtime.rs").unwrap();
        let portals = std::fs::read_to_string("src/bands/full-rust-routes/portals.rs").unwrap();
        assert!(full.contains(".route(\"/api/status/services\", get(homeserver_rust_read_route))"));
        assert!(runtime.contains(".route(\"/api/services/data\", get(service_data_route))"));
        assert!(full.contains(".route(\"/api/service/control\", post(portal_service_control_route))"));
        assert!(portals.contains("/api/v1/staff/intent"), "service control must remain a real Caduceus staff intent membrane");
        assert_eq!(field_004_service_routes().len(), 3);
    }

    #[test]
    fn field_004_deny_marker_fixture_wall_guest_service_data_serialization_is_clean() {
        let raw = maximal_service_data_fixture();
        let guest = project_service_data_guest(&raw);
        let body = serde_json::to_string(&guest).unwrap();
        for denied in [
            "DENY-",
            "systemdName",
            "port",
            "statusDetails",
            "isScriptManaged",
            "needsReboot",
            "isEnabled",
            "adminRuntime",
            "serviceCardSchema",
            "monitorTopics",
            "ssh.service",
            "smbd.service",
        ] {
            assert!(!body.contains(denied), "guest service data leaked denied marker {denied}: {body}");
        }
        for expected in ["serviceCount", "runningCount", "stoppedCount", "unavailableCount", "needsAttentionCount", "degraded"] {
            assert!(body.contains(expected), "guest service aggregate omitted {expected}: {body}");
        }
    }

    #[test]
    fn field_004_guest_type_purity_wall_cannot_represent_service_identifiers_or_enablement() {
        let value = serde_json::to_value(project_service_data_guest(&maximal_service_data_fixture())).unwrap();
        let census = json_field_census(&value);
        for denied in ["name", "systemdName", "port", "statusDetails", "isScriptManaged", "needsReboot", "isEnabled", "adminRuntime", "serviceCardSchema", "monitorTopics", "adminFieldLaw"] {
            assert!(!census.iter().any(|field| field == denied), "guest service projection can represent denied field {denied}: {census:?}");
        }
        assert_eq!(census, vec!["firstMissingSignal", "needsAttentionCount", "ok", "route", "runningCount", "schema", "serviceCount", "status", "stoppedCount", "success", "unavailableCount"]);
    }

    #[tokio::test]
    async fn field_004_route_membrane_wall_service_reads_project_or_remain_generic_by_session() {
        let router = app(AppState { tab_root: Arc::new(test_tab_root("field-004-services-reads")) });
        let generic = router.clone().oneshot(Request::builder().uri("/api/status/services").header("X-Admin-Token", authorize_test_admin_token()).body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(generic.status(), StatusCode::OK);
        let generic_body = String::from_utf8(axum::body::to_bytes(generic.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        assert!(generic_body.contains("coronatio.homeserver.route.read.v1"), "{generic_body}");
        for denied in ["systemdName", "\"port\"", "isEnabled", "statusDetails", "isScriptManaged", "needsReboot", "DENY-"] {
            assert!(!generic_body.contains(denied), "generic /api/status/services leaked {denied}: {generic_body}");
        }

        let guest = router.clone().oneshot(Request::builder().uri("/api/services/data").body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(guest.status(), StatusCode::OK);
        let guest_body = String::from_utf8(axum::body::to_bytes(guest.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        assert!(guest_body.contains("coronatio.service-data.guest-projection.v1"), "{guest_body}");
        for denied in ["systemdName", "\"port\"", "isEnabled", "statusDetails", "isScriptManaged", "needsReboot", "ssh-service", "samba-file-sharing", "adminRuntime", "DENY-"] {
            assert!(!guest_body.contains(denied), "guest /api/services/data leaked {denied}: {guest_body}");
        }

        let token = authorize_test_admin_token();
        let admin = router.oneshot(Request::builder().uri("/api/services/data").header("X-Admin-Token", token).body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(admin.status(), StatusCode::OK);
        let admin_body = String::from_utf8(axum::body::to_bytes(admin.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        for expected in ["coronatio.service-data.contract.v1", "serviceCardSchema", "systemdName", "isEnabled", "isScriptManaged", "needsReboot", "adminRuntime", "ssh-service", "samba-file-sharing"] {
            assert!(admin_body.contains(expected), "admin /api/services/data omitted {expected}: {admin_body}");
        }
        assert!(!admin_body.contains("adminFieldLaw"), "admin /api/services/data still advertises denylist law: {admin_body}");
    }

    #[tokio::test]
    async fn field_004_mutation_refusal_wall_service_control_refuses_guest_before_caduceus() {
        let router = app(AppState { tab_root: Arc::new(test_tab_root("field-004-service-control-guest")) });
        let response = router.oneshot(Request::builder().method("POST").uri("/api/service/control").header("content-type", "application/json").body(Body::from(r#"{"service":"ssh","action":"restart"}"#)).unwrap()).await.unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        let body = String::from_utf8(axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        assert!(body.contains("coronatio.services.mutation.refusal.v1"), "{body}");
        assert!(body.contains("admin-session-required"), "{body}");
        assert!(body.contains("\"accepted\":false"), "{body}");
        assert!(!body.contains("systemdService"), "guest refusal leaked service diagnostics: {body}");
    }

    #[tokio::test]
    async fn field_004_admin_mutation_wall_service_control_crosses_caduceus_after_session_gate() {
        let _env_guard = HX_EXEMPLAR_ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
        let _guard = CADUCEUS_ENV_LOCK.get_or_init(|| std::sync::Mutex::new(())).lock().unwrap();
        std::env::set_var("CADUCEUS_URL", "http://127.0.0.1:9");
        let config_path = std::env::temp_dir().join(format!("coronatio-portals-allowlist-{}.json", std::process::id()));
        std::fs::write(
            &config_path,
            r#"{"tabs":{"portals":{"data":{"portals":[{"name":"SSH","services":["ssh.service"],"localURL":"https://ssh.home.arpa"}]}}}}"#,
        )
        .unwrap();
        std::env::set_var("CORONATIO_HOMESERVER_JSON", &config_path);
        let router = app(AppState { tab_root: Arc::new(test_tab_root("field-004-service-control-admin")) });
        let token = authorize_test_admin_token();
        let response = router.oneshot(Request::builder().method("POST").uri("/api/service/control").header("X-Admin-Token", token).header("content-type", "application/json").body(Body::from(r#"{"service":"ssh","action":"restart"}"#)).unwrap()).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = String::from_utf8(axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        assert!(body.contains("\"success\":false"), "{body}");
        assert!(body.contains("\"message\":"), "{body}");
        assert!(body.contains("\"output\":\"caduceus-unreachable\""), "{body}");
        assert!(body.contains("\"active\":false"), "{body}");
        std::env::remove_var("CADUCEUS_URL");
        std::env::remove_var("CORONATIO_HOMESERVER_JSON");
        std::fs::remove_file(config_path).unwrap();
    }

    #[test]
    fn field_004_portal_service_allowlist_wall_precedes_caduceus_dispatch() {
        let portals = vec![PortalEntry {
            name: "Files".to_string(),
            description: String::new(),
            services: vec!["smbd.service".to_string(), "syncthing".to_string()],
            r#type: "systemd".to_string(),
            port: None,
            local_url: "https://files.home.arpa".to_string(),
            remote_url: None,
            status: None,
            visible: true,
        }];

        assert!(portal_service_is_allowlisted("smbd", &portals));
        assert!(portal_service_is_allowlisted("smbd.service", &portals));
        assert!(portal_service_is_allowlisted("syncthing.service", &portals));
        assert!(!portal_service_is_allowlisted("ssh", &portals));

        let source = std::fs::read_to_string("src/bands/full-rust-routes/portals.rs").unwrap();
        let allowlist = source.find("if !portal_service_is_allowlisted").unwrap();
        let dispatch = source.find("let caduceus = caduceus_http_json").unwrap();
        assert!(allowlist < dispatch, "portal allow-list must fail closed before Caduceus dispatch");
    }
