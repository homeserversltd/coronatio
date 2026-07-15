    #[tokio::test(flavor = "current_thread")]
    async fn admin_update_mints_one_exact_caduceus_update_now_capability_and_returns_receipt() {
        let mark = crate::caduceus_access::test_fixture::mark();
        let response = app(AppState { tab_root: Arc::new(test_tab_root("update-capability")) })
            .oneshot(successor_admin_request(
                Request::builder()
                    .method("POST")
                    .uri("/api/caduceus/update/now")
                    .body(Body::empty())
                    .unwrap(),
            ))
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = String::from_utf8(axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        assert!(body.contains("\"schema\":\"coronatio.caduceus.mutation.v1\""), "{body}");
        assert!(body.contains("\"route\":\"update_now\""), "{body}");
        assert!(body.contains("\"firstMissingSignal\":\"none\""), "{body}");

        let records = crate::caduceus_access::test_fixture::records_since(mark);
        assert_eq!(records.iter().filter(|record| record.path == "/api/v1/access/capabilities/mint").count(), 1, "{records:?}");
        assert!(records.iter().any(|record| {
            record.path == "/api/v1/access/capabilities/mint"
                && record.action.as_deref() == Some("update now")
                && record.target.as_deref() == Some("local")
        }), "{records:?}");
        assert!(records.iter().any(|record| record.path == "/api/v1/update/now" && record.capability_present), "{records:?}");
        let rendered = format!("{records:?}");
        assert!(!rendered.contains(crate::caduceus_access::test_fixture::opaque_ticket()));
        assert!(!rendered.contains("caduceus-test-capability"));
    }

    #[test]
    fn update_mutation_authority_has_a_single_cookie_scoped_capability_path_and_no_legacy_signer() {
        let authority = std::fs::read_to_string("src/bands/mutation-authority.rs").unwrap();
        let actuator = std::fs::read_to_string("src/bands/caduceus.rs").unwrap();
        assert!(authority.contains("MutationRequestContext::from_headers"));
        assert!(authority.contains("capability_mint(ticket, &mapping.action, &mapping.target)"));
        assert!(authority.contains("expose_for_one_request"));
        for retired in [
            "household_signing_key",
            "CORONATIO_CADUCEUS_SIGNING_KEY",
            "/etc/caduceus/household-signing.key",
            "caduceus-keyman-sign-capability",
        ] {
            assert!(!authority.contains(retired), "legacy signer survived: {retired}");
        }
        assert!(!actuator.contains("fn caduceus_dispatch_route"));
        assert!(!actuator.contains("thread::spawn"));
    }
