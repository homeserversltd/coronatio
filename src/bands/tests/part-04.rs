    #[tokio::test]
    async fn stats_sse_and_monitor_pulse_prove_first_topic() {
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
        assert_eq!(pulse.topic.id, "stats.system");
        assert_eq!(pulse.first_event.schema, "coronatio.stats.event.v1");
        assert_eq!(pulse.event_route, "/api/stats/events");

        let event_response = router
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/api/stats/events")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(event_response.status(), StatusCode::OK);
        assert_eq!(
            event_response.headers().get(header::CONTENT_TYPE).unwrap(),
            "text/event-stream"
        );
        let event_body = String::from_utf8(
            axum::body::to_bytes(event_response.into_body(), usize::MAX)
                .await
                .unwrap()
                .to_vec(),
        )
        .unwrap();
        assert!(event_body.contains("event: stats.system"));
        assert!(event_body.contains("coronatio.stats.event.v1"));

        let renew_response = router
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/stats/events/renew")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(renew_response.status(), StatusCode::OK);
        let renew: LeaseRenewalReadback = serde_json::from_slice(
            &axum::body::to_bytes(renew_response.into_body(), usize::MAX)
                .await
                .unwrap(),
        )
        .unwrap();
        assert_eq!(renew.schema, "coronatio.stats.events.renewal.v1");
        assert_eq!(renew.topic, "stats.system");
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
    async fn installer_route_encodes_premium_installer_law_without_live_mutation() {
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
                    && mapping.rejected_shape.contains("premium package")
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
            .contains(&"adminToken".to_string()));
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
        .oneshot(
            Request::builder()
                .uri("/api/services/data")
                .body(Body::empty())
                .unwrap(),
        )
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
        let temp = test_tab_root("portals-json-read");
        let config_path = temp.join("homeserver.json");
        let factory_path = temp.join("homeserver.factory");
        std::fs::write(&config_path, r#"{
          "tabs": { "portals": { "visibility": { "tab": true, "elements": { "Docs": false } }, "data": { "portals": [
            { "name": "Coronatio", "description": "Rust crown", "services": ["coronatio"], "type": "systemd", "port": 3013, "localURL": "http://home.arpa:3013/", "remoteURL": "https://home.tail13aff.ts.net:13013/" },
            { "name": "Docs", "description": "Reference", "services": [], "type": "link", "localURL": "https://docs.home.arpa/" }
          ] } } }
        }"#).unwrap();
        std::fs::write(&factory_path, r#"{
          "tabs": { "portals": { "data": { "portals": [
            { "name": "Coronatio", "description": "Rust crown", "services": ["coronatio"], "type": "systemd", "port": 3013, "localURL": "http://home.arpa:3013/" }
          ] } } }
        }"#).unwrap();
        std::env::set_var("CORONATIO_HOMESERVER_JSON", &config_path);
        std::env::set_var("CORONATIO_HOMESERVER_FACTORY_JSON", &factory_path);
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
        assert!(data.factory_portals.contains(&"Coronatio".to_string()));
        assert_eq!(data.first_missing_signal, "none");
        std::env::remove_var("CORONATIO_HOMESERVER_JSON");
        std::env::remove_var("CORONATIO_HOMESERVER_FACTORY_JSON");
    }

    #[tokio::test]
    async fn validate_pin_reads_homeserver_json_override_before_etc() {
        let _guard = HX_EXEMPLAR_ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
        let temp = test_tab_root("pin-json-override");
        let config_path = temp.join("homeserver.json");
        std::fs::write(&config_path, r#"{
          "global": { "admin": { "pin": "2468" } }
        }"#).unwrap();
        std::env::set_var("CORONATIO_HOMESERVER_JSON", &config_path);
        let response = app(AppState { tab_root: Arc::new(temp) })
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/validatePin")
                    .header(axum::http::header::CONTENT_TYPE, "application/json")
                    .body(Body::from(r#"{"pin":"2468"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let data: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(data["schema"], "coronatio.homeserver.auth.pin.v1");
        assert_eq!(data["valid"], true);
        assert_eq!(data["firstMissingSignal"], "none");
        assert_eq!(
            data["source"].as_str().unwrap(),
            format!("{} global.admin.pin", config_path.display())
        );
        std::env::remove_var("CORONATIO_HOMESERVER_JSON");
    }

    #[tokio::test]
    async fn portal_image_route_serves_original_portal_icons() {
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

    #[test]
    fn portals_viewport_hydrates_cards_from_api_not_static_scaffold() {
        let shell = render_crown_shell();
        assert!(shell.contains("data-portals-grid"));
        assert!(shell.contains("data-portals-source=\"/api/portals\""));
        assert!(shell.contains("function hydratePortals()"));
        assert!(shell.contains("renderPortalCard(portal, factoryNames)"));
        assert!(shell.contains("/api/portals/images/${encodeURIComponent(portal.name)}.png"));
        assert!(!shell.contains("Rust crown preview, port 3013"));
        assert!(!shell.contains("Privileged actuator membrane, port 3014"));
    }



    #[test]
    fn portals_visibility_uses_admin_mode_not_missing_apply_admin_mode() {
        let shell = render_crown_shell();
        assert!(!shell.contains("applyAdminMode()"));
        assert!(shell.contains("setAdminMode(headerState.isAdmin)"));
        assert!(shell.contains("data-portal-element data-visible=\"${isVisible}\""));
        assert!(shell.contains("data-portal-visibility-toggle"));
        assert!(shell.contains("portal.visible !== false"));
        assert!(shell.contains("[data-admin-mode=\"false\"] [data-portal-element][data-visible=\"false\"]"));
        assert!(shell.contains("[data-admin-mode=\"true\"] [data-portal-element][data-visible=\"false\"]"));
    }

    #[test]
    fn portals_admin_mode_ports_original_service_controls() {
        let shell = render_crown_shell();
        assert!(shell.contains("data-service-action=\"start\""));
        assert!(shell.contains("data-service-action=\"stop\""));
        assert!(shell.contains("data-service-action=\"restart\""));
        assert!(shell.contains("data-service-action=\"enable\""));
        assert!(shell.contains("data-service-action=\"disable\""));
        assert!(shell.contains("data-service-action=\"status\""));
        assert!(shell.contains("data-portal-services"));
        assert!(shell.contains("function handlePortalServiceAction(event)"));
        assert!(shell.contains("fetch('/api/service/control'"));
        assert!(shell.contains("data-admin-only data-admin-viewport=\"portals\""));
    }

    #[tokio::test]
    async fn portals_service_control_validates_and_enters_caduceus_staff_intent() {
        let temp = test_tab_root("portal-service-control");
        let response = app(AppState { tab_root: Arc::new(temp) })
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/service/control")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"service":"jellyfin","action":"restart"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body = String::from_utf8(bytes.to_vec()).unwrap();
        assert!(body.contains("coronatio.portals.service_control.v1"), "{body}");
        assert!(body.contains("/api/v1/staff/intent"), "{body}");
        assert!(body.contains("/api/service/control"), "{body}");
        assert!(body.contains("portal-service"), "{body}");
        assert!(body.contains("jellyfin.service"), "{body}");
        assert!(body.contains("caduceus-unreachable"), "{body}");
    }

    #[tokio::test]
    async fn portals_service_control_rejects_shell_injection_and_unknown_actions() {
        let temp = test_tab_root("portal-service-control-invalid");
        for body in [
            r#"{"service":"jellyfin;rm -rf /","action":"restart"}"#,
            r#"{"service":"jellyfin","action":"reformat"}"#,
        ] {
            let response = app(AppState { tab_root: Arc::new(temp.clone()) })
                .oneshot(
                    Request::builder()
                        .method("POST")
                        .uri("/api/service/control")
                        .header("content-type", "application/json")
                        .body(Body::from(body))
                        .unwrap(),
                )
                .await
                .unwrap();
            assert_eq!(response.status(), StatusCode::BAD_REQUEST);
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
            let text = String::from_utf8(bytes.to_vec()).unwrap();
            assert!(text.contains("coronatio.portals.service_control.v1"), "{text}");
            assert!(!text.contains("/api/v1/staff/intent"), "{text}");
        }
    }

    fn test_tab_root(name: &str) -> PathBuf {
        let root =
            std::env::temp_dir().join(format!("coronatio-test-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).unwrap();
        root
    }
