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

