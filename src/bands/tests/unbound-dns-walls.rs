    #[test]
    fn unbound_shell_target_survives_guest_to_admin_document_attendance() {
        let tab = native_tab_contracts().into_iter().find(|tab| tab.id == "unbound").unwrap();
        assert_eq!(tab.display_name, "DNS");
        assert!(tab.admin_only);

        let guest = render_crown_shell_for_session(Session::Guest);
        assert!(
            !guest.contains(r#"data-tab-id="unbound""#),
            "guest shell must not declare the DNS tab"
        );
        for forbidden in [
            r#"id="pane-unbound""#,
            r#"data-view-panel="unbound""#,
            r#"data-admin-viewport="unbound""#,
        ] {
            assert!(
                !guest.contains(forbidden),
                "guest shell must omit protected DNS pane marker {forbidden}"
            );
        }

        let admin = render_crown_shell_for_session(Session::Admin);
        assert!(admin.contains(r#"data-tab-id="unbound""#));
        assert_eq!(admin.matches(r#"id="pane-unbound""#).count(), 1);
        assert_eq!(admin.matches(r#"data-view-panel="unbound""#).count(), 1);
        assert_eq!(admin.matches(r#"data-admin-viewport="unbound""#).count(), 1);
        for required in ["Local DNS", "data-dns-form", "data-dns-records", "data-dns-refresh"] {
            assert!(admin.contains(required), "admin missing {required}");
        }
    }

    #[test]
    fn unbound_client_is_externalized_delegated_and_safe() {
        let client = include_str!("../shell/unbound-client.rs");
        for required in ["hydrateDns", "viewportFamilyAdmitted('unbound')", "document.visibilityState !== 'visible'", "document.body.addEventListener('submit'", "document.body.addEventListener('click'", "textContent", "/api/dns/records"] {
            assert!(client.contains(required), "DNS client missing {required}");
        }
        for forbidden in ["setInterval(hydrateDns", "innerHTML", "sudo", "/usr/local/sbin", "/etc/unbound"] {
            assert!(!client.contains(forbidden), "DNS client retained forbidden {forbidden}");
        }
        let chrome = crown_chrome_js();
        assert!(chrome.contains("hydrateDns"));
        assert!(chrome.contains("unbound: Object.freeze({ topics: ['admin.dns'], snapshotRoutes: ['/api/dns/records'], eventRoute: null, renewRoute: null, authClass: 'admin' })"));
        assert!(chrome.contains("if (!pane || !viewportFamilyAdmitted('unbound') || document.visibilityState !== 'visible') return;"));
    }

    #[tokio::test]
    async fn unbound_guests_refuse_before_caduceus_contact() {
        let router = app(AppState { tab_root: Arc::new(test_tab_root("unbound-guest-refusal")) });
        for request in [
            Request::builder().uri("/api/dns/records").body(Body::empty()).unwrap(),
            Request::builder().method("POST").uri("/api/dns/records").header("content-type", "application/json").body(Body::from(r#"{"name":"app.home.arpa","address":"192.168.123.2"}"#)).unwrap(),
            Request::builder().method("DELETE").uri("/api/dns/records/app.home.arpa").body(Body::empty()).unwrap(),
        ] {
            let response = router.clone().oneshot(request).await.unwrap();
            assert_eq!(response.status(), StatusCode::FORBIDDEN);
            let body = String::from_utf8(axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
            assert!(body.contains("caduceus-access-origin-refused"), "{body}");
        }
    }

    #[test]
    fn unbound_route_contract_is_bounded_and_receipt_preserving() {
        let source = std::fs::read_to_string("src/bands/full-rust-routes/unbound.rs").unwrap();
        for action in [r#"{"action": "status"}"#, "ensure-local-data", "\"action\": \"remove\"", "/api/v1/network/dns"] { assert!(source.contains(action), "missing {action}"); }
        for forbidden in ["sudo", "/usr/local/sbin", "/etc/unbound", "setInterval", "action_path"] { assert!(!source.contains(forbidden), "forbidden {forbidden}"); }
        let inventory = full_rust_route_inventory();
        assert!(inventory.iter().any(|(path, methods)| *path == "/api/dns/records" && *methods == ["get", "post"]));
        assert!(inventory.iter().any(|(path, methods)| *path == "/api/dns/records/:name" && *methods == ["delete"]));
        let response = dns_response("/api/dns/records", CaduceusHttpReadback { ok: false, status: 422, path: "/api/v1/network/dns".to_string(), body: serde_json::json!({"ok":false,"firstMissingSignal":"address-private-required","validation":{"address":"private"}}), first_missing_signal: "address-private-required".to_string() });
        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }
