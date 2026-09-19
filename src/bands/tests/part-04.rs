    #[tokio::test]
    async fn pulse_monitor_readback_names_living_stream_contract() {
        let temp = test_tab_root("stats-sse");
        let router = app(AppState {
            tab_root: Arc::new(temp),
        });
        let pulse_response = router
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/api/monitor/pulse")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(pulse_response.status(), StatusCode::OK);
        let pulse_bytes = axum::body::to_bytes(pulse_response.into_body(), usize::MAX)
            .await
            .unwrap();
        let pulse: MonitorPulseReadback = serde_json::from_slice(&pulse_bytes).unwrap();
        assert_eq!(pulse.schema, "coronatio.monitor-pulse.v1");
        assert_eq!(pulse.topic.id, "tabs.changed");
        assert_eq!(pulse.stream_contract.schema, "coronatio.pulse.stream.v1");
        assert_eq!(pulse.stream_contract.first_event, "pulse.open");
        assert_eq!(pulse.stream_contract.poke_data, "{}");
        assert_eq!(pulse.event_route, "/api/stats/pulse");
        assert!(pulse.proof_policy.iter().any(|law| law.contains("data-free invalidations")));
    }

    #[tokio::test]
    async fn route_boundary_returns_json_for_api_misses_and_shell_for_static_fallback() {
        let temp = test_tab_root("boundary-law");
        let router = app(AppState {
            tab_root: Arc::new(temp),
        });
        let api_response = router
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/api/missing-route")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(api_response.status(), StatusCode::NOT_FOUND);
        let api_body = String::from_utf8(
            axum::body::to_bytes(api_response.into_body(), usize::MAX)
                .await
                .unwrap()
                .to_vec(),
        )
        .unwrap();
        assert!(api_body.contains("coronatio.api.error.v1"));
        assert!(!api_body.contains("<html"));

        let shell_response = router
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/some/client/route")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(shell_response.status(), StatusCode::OK);
        let shell_body = String::from_utf8(
            axum::body::to_bytes(shell_response.into_body(), usize::MAX)
                .await
                .unwrap()
                .to_vec(),
        )
        .unwrap();
        assert!(shell_body.contains("data-product=\"Coronatio\""));

        let boundary_response = router
            .oneshot(
                Request::builder()
                    .uri("/api/boundary")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(boundary_response.status(), StatusCode::OK);
        let boundary: BoundaryReadback = serde_json::from_slice(
            &axum::body::to_bytes(boundary_response.into_body(), usize::MAX)
                .await
                .unwrap(),
        )
        .unwrap();
        assert_eq!(boundary.schema, "coronatio.route-boundary.v1");
        assert!(boundary.api_unknown_path_policy.contains("JSON 404"));
    }

    #[tokio::test]
    async fn installer_route_encodes_legacy_quarry_law_without_live_mutation() {
        let temp = test_tab_root("installer-law");
        let response = app(AppState {
            tab_root: Arc::new(temp),
        })
        .oneshot(
            Request::builder()
                .uri("/api/installer")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let installer: InstallerReadback = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(installer.schema, "coronatio.installer.contract.v1");
        assert_eq!(installer.status, "contract-only");
        assert!(installer
            .root_manifest_schema
            .required_fields
            .contains(&"name".to_string()));
        assert!(installer
            .component_manifest_schema
            .operation_types
            .contains(&"append".to_string()));
        assert!(installer
            .validation_phases
            .iter()
            .any(|phase| phase.id == "version-conflict"));
        assert!(installer
            .install_phases
            .iter()
            .any(|phase| phase.id == "frontend-rebuild"));
        assert_eq!(
            installer.rollback_law.order,
            [
                "config rollback",
                "package rollback",
                "file operation rollback",
                "service state rollback"
            ]
        );
        assert!(installer
            .first_missing_live_signal
            .contains("Caduceus installer actuator"));
        assert!(installer
            .lane_mapping
            .iter()
            .any(
                |mapping| mapping.install_mode == InstallMode::FirstPartyNative
                    && mapping.rejected_shape.contains("legacy installer package")
            ));
    }

    #[tokio::test]
    async fn frontend_storage_route_encodes_browser_persistence_and_migration_law() {
        let temp = test_tab_root("frontend-storage");
        let response = app(AppState {
            tab_root: Arc::new(temp),
        })
        .oneshot(
            Request::builder()
                .uri("/api/frontend/storage")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let data: FrontendStorageReadback = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(data.schema, "coronatio.frontend-storage.contract.v1");
        assert_eq!(data.status, "contract-only");
        assert!(data
            .persisted_stores
            .iter()
            .any(|store| store.storage_key == "homeserver-store"
                && store.persisted_fields.contains(&"activeTab".to_string())));
        assert!(data
            .persisted_stores
            .iter()
            .any(|store| store.storage_key == "auth-storage"
                && store.boundary.contains("never localStorage")));
        assert!(data
            .persistence_fields
            .iter()
            .any(|field| field.field == "isInitialized"
                && field.coronatio_owner == "startup receipt"));
        assert!(data
            .debounce_law
            .iter()
            .any(|law| law.interval_ms == 500 && law.source.contains("debouncedSetItem")));
        assert!(data
            .stale_state_law
            .iter()
            .any(|law| law.coronatio_rule.contains("malformed browser snapshot")));
        assert!(data
            .forbidden_persistence
            .iter()
            .any(|field| field.contains("credential")));
        assert!(data
            .first_missing_live_signal
            .contains("storage migration adapter"));
    }

    #[tokio::test]
    async fn service_data_route_encodes_portal_monitor_and_broadcast_law() {
        let temp = test_tab_root("service-data");
        let response = app(AppState {
            tab_root: Arc::new(temp),
        })
        .oneshot(successor_admin_request(
            Request::builder()
                .uri("/api/services/data")
                .body(Body::empty())
                .unwrap(),
        ))
        .await
        .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let data: ServiceDataReadback = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(data.schema, "coronatio.service-data.contract.v1");
        assert_eq!(data.status, "contract-only");
        assert!(data.portal_schema.fields.contains(&"remoteURL".to_string()));
        assert!(data
            .portal_schema
            .portal_types
            .contains(&"link".to_string()));
        assert!(data
            .service_card_schema
            .fields
            .contains(&"isScriptManaged".to_string()));
        assert!(data
            .monitor_topics
            .iter()
            .any(|topic| topic.topic == "admin.disk.info" && topic.admin_only));
        assert!(data
            .monitor_topics
            .iter()
            .any(|topic| topic.topic == "services.status"
                && topic.admin_fields.contains(&"isEnabled".to_string())));
        assert!(data.broadcast_law.transport_replacement.contains("SSE"));
        assert!(data.admin_runtime.source.contains("/proc/mounts"));
        assert_eq!(data.admin_runtime.services.len(), 3);
        assert!(data
            .admin_runtime
            .services
            .iter()
            .any(|service| service.id == "ssh-password-authentication"
                && service.source.contains("PasswordAuthentication")));
        assert!(data
            .admin_runtime
            .services
            .iter()
            .any(|service| service.id == "ssh-service"));
        assert!(data
            .admin_runtime
            .mount_destinations
            .iter()
            .any(|destination| destination.path == "/mnt/nas" && destination.role == "Primary NAS"));
        assert!(data
            .first_missing_live_signal
            .contains("service collectors and monitor broadcasters are not wired"));
    }

    #[tokio::test]
    async fn registry_transaction_route_encodes_config_patch_persistence_law() {
        let temp = test_tab_root("registry-transaction");
        let response = app(AppState {
            tab_root: Arc::new(temp),
        })
        .oneshot(
            Request::builder()
                .uri("/api/registry/transaction")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let transaction: RegistryTransactionReadback = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(transaction.schema, "coronatio.registry.transaction.v1");
        assert_eq!(transaction.status, "contract-only");
        assert!(transaction.deep_merge_law.tab_merge.contains("starred"));
        assert!(transaction
            .starred_tab_law
            .preservation_rule
            .contains("without displacing"));
        assert!(transaction
            .validation_law
            .factory_fallback_gate
            .contains("factoryFallback"));
        assert!(transaction
            .persistence_law
            .permission_restore
            .contains("www-data:www-data"));
        assert!(transaction
            .rollback_law
            .mismatch_policy
            .contains("do not remove"));
        assert!(transaction
            .transaction_sequence
            .iter()
            .any(|phase| phase.id == "atomic-promote" && phase.source_law.contains("shutil.move")));
        assert!(transaction
            .first_missing_live_signal
            .contains("Caduceus registry transaction actuator"));
    }

    #[tokio::test]
    async fn api_root_declares_installer_contract_route() {
        let temp = test_tab_root("installer-root-route");
        let response = app(AppState {
            tab_root: Arc::new(temp),
        })
        .oneshot(Request::builder().uri("/api").body(Body::empty()).unwrap())
        .await
        .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let root: CoronatioRoot = serde_json::from_slice(&bytes).unwrap();
        assert!(root.routes.contains(&"/api/installer".to_string()));
    }


    #[tokio::test]
    async fn portals_route_reads_homeserver_json_portals_like_original_surface() {
        let _guard = HX_EXEMPLAR_ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
        let temp = test_tab_root("portals-json-read");
        let config_path = temp.join("homeserver.json");
        std::fs::write(&config_path, r#"{
          "tabs": { "portals": { "visibility": { "tab": true, "elements": { "Docs": false } }, "data": { "portals": [
            { "name": "Coronatio", "description": "Rust crown", "services": ["coronatio"], "type": "systemd", "port": 3013, "localURL": "http://home.arpa:3013/", "remoteURL": "https://home.tail13aff.ts.net:13013/" },
            { "name": "Docs", "description": "Reference", "services": [], "type": "link", "localURL": "https://docs.home.arpa/" }
          ] } } }
        }"#).unwrap();
        std::env::set_var("CORONATIO_HOMESERVER_JSON", &config_path);
        let response = app(AppState { tab_root: Arc::new(temp) })
            .oneshot(Request::builder().uri("/api/portals").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let data: PortalConfigResponse = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(data.schema, "coronatio.portals.config.v1");
        assert_eq!(data.route, "/api/portals");
        assert!(data.success);
        assert_eq!(data.portals.len(), 2);
        assert_eq!(data.portals[0].name, "Coronatio");
        assert_eq!(data.portals[0].port, Some(3013));
        assert_eq!(data.portals[0].local_url, "http://home.arpa:3013/");
        assert_eq!(data.portals[0].remote_url.as_deref(), Some("https://home.tail13aff.ts.net:13013/"));
        assert_eq!(data.portals[1].r#type, "link");
        assert!(data.portals[0].visible);
        assert!(!data.portals[1].visible);
        assert_eq!(data.first_missing_signal, "none");
        std::env::remove_var("CORONATIO_HOMESERVER_JSON");
    }


    #[tokio::test]
    async fn portal_image_route_serves_original_portal_icons() {
        let _env_guard = HX_EXEMPLAR_ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
        let temp = test_tab_root("portal-images");
        let images = temp.join("images");
        std::fs::create_dir_all(&images).unwrap();
        std::fs::write(images.join("Coronatio.png"), b"png-bytes").unwrap();
        std::env::set_var("CORONATIO_PORTAL_IMAGE_ROOT", &images);
        let response = app(AppState { tab_root: Arc::new(temp) })
            .oneshot(Request::builder().uri("/api/portals/images/Coronatio.png").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get(axum::http::header::CONTENT_TYPE).unwrap(),
            "image/png"
        );
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        assert_eq!(&bytes[..], b"png-bytes");
        std::env::remove_var("CORONATIO_PORTAL_IMAGE_ROOT");
    }

    #[tokio::test]
    async fn portal_image_default_fallback_001() {
        let _env_guard = HX_EXEMPLAR_ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
        let temp = test_tab_root("portal-image-default-fallback");
        let default_bytes = b"default-png-bytes";
        std::fs::write(temp.join("default.png"), default_bytes).unwrap();
        std::fs::write(temp.join("ExistingPortal.png"), b"existing-png-bytes").unwrap();
        std::env::set_var("CORONATIO_PORTAL_IMAGE_ROOT", &temp);

        let missing_response = app(AppState { tab_root: Arc::new(temp.clone()) })
            .oneshot(Request::builder().uri("/api/portals/images/MissingPortal.png").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(missing_response.status(), StatusCode::OK);
        assert_eq!(
            missing_response.headers().get(axum::http::header::CONTENT_TYPE).unwrap(),
            "image/png"
        );
        assert_eq!(
            &axum::body::to_bytes(missing_response.into_body(), usize::MAX).await.unwrap()[..],
            default_bytes
        );

        let existing_response = app(AppState { tab_root: Arc::new(temp.clone()) })
            .oneshot(Request::builder().uri("/api/portals/images/ExistingPortal.png").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(existing_response.status(), StatusCode::OK);
        assert_eq!(
            &axum::body::to_bytes(existing_response.into_body(), usize::MAX).await.unwrap()[..],
            b"existing-png-bytes"
        );

        std::fs::remove_file(temp.join("default.png")).unwrap();
        let absent_default_response = app(AppState { tab_root: Arc::new(temp.clone()) })
            .oneshot(Request::builder().uri("/api/portals/images/default.png").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(absent_default_response.status(), StatusCode::NOT_FOUND);
        std::env::remove_var("CORONATIO_PORTAL_IMAGE_ROOT");
    }

    #[test]
    fn portals_viewport_hydrates_cards_from_api_not_static_scaffold() {
        let shell = render_crown_shell();
        assert!(shell.contains("data-portals-grid"));
        assert!(shell.contains("data-portals-source=\"/api/portals/elements\""));
        assert!(!shell.contains("data-portals-fragment=\"/api/portals/elements\" hx-get"));
        let admitted = render_og_pane_fragment("portals", Session::Guest);
        assert!(admitted.contains("hx-get=\"/api/portals/elements\""));
        assert!(admitted.contains("hx-trigger=\"load\""));
        assert!(!admitted.contains("portals-refresh"));
        assert!(!shell.contains("function hydratePortals()"));
        assert!(shell.contains("submitPortalForm"));
        assert!(shell.contains("deletePortal"));
        assert!(!shell.contains("function renderPortalCard"));
        assert!(!shell.contains("Rust crown preview, port 3013"));
        assert!(!shell.contains("Privileged actuator membrane, port 3014"));
    }



    #[test]
    fn portals_visibility_uses_admin_mode_not_missing_apply_admin_mode() {
        let shell = render_crown_shell();
        let fragment = include_str!("../crown-law/element-fragments.rs");
        assert!(!shell.contains("applyAdminMode()"));
        assert!(shell.contains("bootstrapAdminMode();"));
        assert!(shell.contains("applyAdminDomState();"));
        assert!(fragment.contains("data-portal-element data-visible=\"{}\""));
        assert!(fragment.contains("data-portal-visibility-toggle"));
        assert!(shell.contains("[data-admin-mode=\"false\"] [data-portal-element][data-visible=\"false\"]"));
        assert!(shell.contains("[data-admin-mode=\"true\"] [data-portal-element][data-visible=\"false\"]"));
    }

    #[test]
    fn portals_admin_mode_ports_original_service_controls() {
        let shell = render_crown_shell();
        let fragment = include_str!("../crown-law/element-fragments.rs");
        assert!(fragment.contains("data-service-action=\"start\""));
        assert!(fragment.contains("data-service-action=\"stop\""));
        assert!(fragment.contains("data-service-action=\"restart\""));
        assert!(fragment.contains("data-service-action=\"enable\""));
        assert!(fragment.contains("data-service-action=\"disable\""));
        assert!(fragment.contains("data-service-action=\"status\""));
        assert!(fragment.contains("data-portal-services"));
        assert!(shell.contains("function handlePortalServiceAction(event)"));
        assert!(shell.contains("function portalServiceFailureMessage(result"));
        for signal in [
            "caduceus-attendance-refused",
            "caduceus-attendance-pin-refused",
            "caduceus-attendance-not-current",
            "caduceus-attendance-invalid",
            "caduceus-attendance-required",
            "caduceus-stale-incarnation",
            "caduceus-attendance-stale-incarnation",
        ] {
            assert!(shell.contains(signal), "missing attendance expiry signal {signal}");
        }
        assert!(shell.contains("caduceus-attendance-connect-failed"));
        assert!(shell.contains("const unreachableSignals = new Set(["));
        assert!(shell.contains("Admin session expired. Enter the PIN again."));
        assert!(shell.contains("The appliance service controller is unreachable."));
        assert!(shell.contains("decoratedError"));
        assert!(shell.contains("fetch('/api/service/control'"));
        assert!(shell.contains("credentials: 'same-origin'"));
        assert!(shell.contains("const header = `=== ${result.service || 'service'} ===`"));
        assert!(shell.contains("function showCoronatioToast(message, variant = 'info')"));
        assert!(shell.contains("function dismissCoronatioToast(toast)"));
        assert!(shell.contains("data-coronatio-toast-spawn"));
        assert!(shell.contains("data-coronatio-toast"));
        assert!(shell.contains("toast-exit"));
        assert!(shell.contains("showCoronatioToast(result.message || `Successfully ${action}ed ${service}`, 'success')"));
        assert!(shell.contains("⚠️ Service Inactive/Failed:\\n"));
        assert!(!shell.contains("JSON.stringify(results)"));
        assert!(shell.contains("data-admin-only data-admin-viewport=\"portals\""));
        let portals_css = std::fs::read_to_string("src/bands/shell/ux/packs/portals.css").unwrap();
        assert_eq!(portals_css.matches(".portal-element:is(:hover, :focus-within) > .portal-card > .portal-card-face,").count(), 1);
        let card_rule = &portals_css[portals_css.find(".portal-card {").unwrap()..portals_css.find(".portal-card-face {").unwrap()];
        assert!(!card_rule.contains("transition: all"));
        assert!(!card_rule.contains("transform"), "portal-card hit target must never transform");
        assert!(!portals_css.contains(".portal-element:hover {"));
        assert!(!portals_css.contains(".portal-element:is(:hover, :focus-within) {"));
        assert!(!card_rule.contains("transition-property: border-color"));
        assert!(!card_rule.contains("transition-duration"));
        let face_rule = &portals_css[portals_css.find(".portal-card-face {").unwrap()..portals_css.find(".portal-element:is(:hover, :focus-within)").unwrap()];
        assert!(face_rule.contains("transition-property: background-color"));
        assert!(face_rule.contains("transition-duration: var(--portal-card-hover-duration)"));
        assert!(!card_rule.contains("background-color"));
        assert!(!portals_css.contains(".portal-card:hover::before"));
        assert!(!portals_css.contains("animation: infinite"));
        let emphasized_rule_start = portals_css.find(".portal-element:is(:hover, :focus-within) > .portal-card > .portal-card-face,").unwrap();
        let emphasized_rule_end = portals_css[emphasized_rule_start..].find(".portal-card-header").unwrap() + emphasized_rule_start;
        let emphasized_rule = &portals_css[emphasized_rule_start..emphasized_rule_end];
        assert!(!emphasized_rule.contains("border-color"), "portal hover steals status border authority");
        assert!(emphasized_rule.contains("background-color: var(--portal-card-hover-background)"));
        assert!(!emphasized_rule.contains("transform"), "portal hover must not move the card face");
        assert!(portals_css.contains(".portal-card-face {\n  pointer-events: none;"));
        let fragment = include_str!("../crown-law/element-fragments.rs");
        assert!(fragment.contains("<div class=\"portal-card-face\"><div class=\"portal-card-header\">"));
        assert!(fragment.contains("<div class=\"portal-card-face\"><div class=\"add-portal-content\">"));
        assert!(!emphasized_rule.contains("translateZ"));
        for forbidden in [".portal-card:hover {\n  transform", ".portal-card:hover .portal-icon", ".add-portal-card:hover {\n  transform", ".add-portal-card:hover .add-portal-icon {\n  transform"] { assert!(!portals_css.contains(forbidden), "forbidden moving hit target: {forbidden}"); }
    }

    #[tokio::test]
    async fn portals_service_control_projects_all_actions_through_direct_caduceus_paths() {
        let _env_guard = HX_EXEMPLAR_ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
        let config_path = std::env::temp_dir().join(format!("coronatio-portals-allowlist-{}.json", std::process::id()));
        std::fs::write(
            &config_path,
            r#"{"global":{"cors":{"allowed_origins":["https://home.arpa"]}},"tabs":{"portals":{"data":{"portals":[{"name":"Jellyfin","services":["jellyfin"],"localURL":"https://jellyfin.home.arpa"}]}}}}"#,
        )
        .unwrap();
        std::env::set_var("CORONATIO_HOMESERVER_JSON", &config_path);
        let router = app(AppState { tab_root: Arc::new(test_tab_root("portal-service-control-all-actions")) });
        let actions = ["status", "start", "stop", "restart", "enable", "disable"];
        let mut observed_paths = Vec::new();
        for action in actions {
            let mark = crate::caduceus_access::test_fixture::mark();
            let response = router
                .clone()
                .oneshot(successor_admin_request(
                    Request::builder()
                        .method("POST")
                        .uri("/api/service/control")
                        .header("content-type", "application/json")
                        .body(Body::from(format!(r#"{{"service":"jellyfin","action":"{action}"}}"#)))
                        .unwrap(),
                ))
                .await
                .unwrap();
            assert_eq!(response.status(), StatusCode::OK, "{action}");
            let body: serde_json::Value = serde_json::from_slice(&axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap()).unwrap();
            assert_eq!(body.as_object().unwrap().len(), 7, "{action}: {body}");
            for key in ["schema", "success", "ok", "message", "output", "active", "firstMissingSignal"] {
                assert!(body.get(key).is_some(), "missing {key} for {action}: {body}");
            }
            assert!(body["success"].as_bool().unwrap(), "{action}: {body}");
            let expected_path = format!("/api/v1/appliance/service/jellyfin/{action}");
            let records = crate::caduceus_access::test_fixture::records_since(mark);
            let matching = records.iter().filter(|record| record.path == expected_path).collect::<Vec<_>>();
            assert_eq!(matching.len(), 1, "{action} records: {records:?}");
            observed_paths.push(matching[0].path.clone());
        }
        assert_eq!(
            observed_paths,
            actions.iter().map(|action| format!("/api/v1/appliance/service/jellyfin/{action}")).collect::<Vec<_>>()
        );
        let source = std::fs::read_to_string("src/bands/full-rust-routes/portals.rs").unwrap();
        assert!(source.contains("caduceus_actuate_json"));
        assert!(!source.contains("caduceus_staff_transition"));
        assert!(!source.contains("resolve_caduceus_door"));
        std::env::remove_var("CORONATIO_HOMESERVER_JSON");
        std::fs::remove_file(config_path).unwrap();
    }

    #[tokio::test]
    async fn portals_service_control_projects_attendance_unreachable_and_systemd_failures() {
        let _env_guard = HX_EXEMPLAR_ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
        let config_path = std::env::temp_dir().join(format!("coronatio-portals-failures-{}.json", uuid::Uuid::new_v4()));
        std::fs::write(
            &config_path,
            r#"{"global":{"cors":{"allowed_origins":["https://home.arpa"]}}}"#,
        )
        .unwrap();
        std::env::set_var("CORONATIO_HOMESERVER_JSON", &config_path);
        let router = app(AppState { tab_root: Arc::new(test_tab_root("portal-service-control-failures")) });
        let missing = router
            .oneshot(successor_session_request(
                Request::builder()
                    .method("POST")
                    .uri("/api/service/control")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"service":"jellyfin","action":"restart"}"#))
                    .unwrap(),
                false,
            ))
            .await
            .unwrap();
        assert_eq!(missing.status(), StatusCode::UNAUTHORIZED);
        let missing_body: serde_json::Value = serde_json::from_slice(&axum::body::to_bytes(missing.into_body(), usize::MAX).await.unwrap()).unwrap();
        assert_eq!(missing_body["firstMissingSignal"], "caduceus-attendance-required");
        assert_eq!(missing_body["ok"], false);

        let unreachable = portal_service_mutation_response(CaduceusHttpReadback {
            ok: false,
            status: 0,
            path: "/api/v1/appliance/service/jellyfin/restart".to_string(),
            body: serde_json::json!({"ok":false,"error":"caduceus-upstream-failed","firstMissingSignal":"caduceus-attendance-connect-failed"}),
            first_missing_signal: "caduceus-attendance-connect-failed".to_string(),
        });
        assert_eq!(unreachable.status(), StatusCode::SERVICE_UNAVAILABLE);
        let unreachable_body: serde_json::Value = serde_json::from_slice(&axum::body::to_bytes(unreachable.into_body(), usize::MAX).await.unwrap()).unwrap();
        assert_eq!(unreachable_body["schema"], "coronatio.portals.service_control.v1");
        assert_eq!(unreachable_body["firstMissingSignal"], "caduceus-attendance-connect-failed");
        assert_eq!(unreachable_body["ok"], false);

        let upstream_message = "Failed to restart jellyfin.service: Unit entered failed state";
        let systemd = portal_service_mutation_response(CaduceusHttpReadback {
            ok: true,
            status: 200,
            path: "/api/v1/appliance/service/jellyfin/restart".to_string(),
            body: serde_json::json!({"schema":"caduceus.appliance.service.v1","success":false,"message":upstream_message,"error":"systemd-service-restart-failed","firstMissingSignal":"systemd-service-restart-failed"}),
            first_missing_signal: "systemd-service-restart-failed".to_string(),
        });
        assert_eq!(systemd.status(), StatusCode::SERVICE_UNAVAILABLE);
        let systemd_body: serde_json::Value = serde_json::from_slice(&axum::body::to_bytes(systemd.into_body(), usize::MAX).await.unwrap()).unwrap();
        assert_eq!(systemd_body["schema"], "coronatio.portals.service_control.v1");
        assert_eq!(systemd_body["firstMissingSignal"], "systemd-service-restart-failed");
        assert_eq!(systemd_body["message"], upstream_message);
        assert_eq!(systemd_body["success"], false);
        assert_eq!(systemd_body["ok"], false);
        assert_ne!(missing_body["firstMissingSignal"], unreachable_body["firstMissingSignal"]);
        assert_ne!(missing_body["firstMissingSignal"], systemd_body["firstMissingSignal"]);
        assert_ne!(unreachable_body["firstMissingSignal"], systemd_body["firstMissingSignal"]);
        std::env::remove_var("CORONATIO_HOMESERVER_JSON");
        std::fs::remove_file(config_path).unwrap();
    }

    #[tokio::test]
    async fn portals_service_control_rejects_shell_injection_and_unknown_actions() {
        let temp = test_tab_root("portal-service-control-invalid");
        for body in [
            r#"{"service":"jellyfin;rm -rf /","action":"restart"}"#,
            r#"{"service":"jellyfin","action":"reformat"}"#,
        ] {
            let response = app(AppState { tab_root: Arc::new(temp.clone()) })
                .oneshot(successor_admin_request(
                    Request::builder()
                        .method("POST")
                        .uri("/api/service/control")
                        .header("content-type", "application/json")
                        .body(Body::from(body))
                        .unwrap(),
                ))
                .await
                .unwrap();
            assert_eq!(response.status(), StatusCode::BAD_REQUEST);
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
            let text = String::from_utf8(bytes.to_vec()).unwrap();
            assert!(text.contains("coronatio.portals.service_control.v1"), "{text}");
            assert!(!text.contains("/api/v1/staff/intent"), "{text}");
        }
    }
