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
        assert_eq!(guest.matches(r#"id="pane-unbound""#).count(), 1);
        assert_eq!(guest.matches(r#"data-view-panel="unbound""#).count(), 1);
        assert_eq!(guest.matches(r#"data-admin-viewport="unbound""#).count(), 1);
        assert!(guest.contains(r#"data-admin-only="true""#));

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
        assert!(client.contains("dnsJson('/api/dns/records/status', { method: 'POST', body: JSON.stringify({}) })"));
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
            Request::builder().method("POST").uri("/api/dns/records/status").header("content-type", "application/json").body(Body::from("{}")).unwrap(),
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
        let status_handler = source
            .split("async fn dns_records_status_post_route")
            .nth(1)
            .and_then(|tail| tail.split("async fn dns_records_post_route").next())
            .expect("status handler");
        assert!(status_handler.contains("\"/api/dns/records/status\""));
        assert!(status_handler.contains("{\"action\": \"status\"}"));
        for forbidden in ["ensure-local-data", "\"action\": \"remove\""] {
            assert!(!status_handler.contains(forbidden), "status handler widened to {forbidden}");
        }
        assert!(source.contains("mutation_context_refusal(headers)"));
        for forbidden in ["sudo", "/usr/local/sbin", "/etc/unbound", "setInterval", "action_path", "Referer", "Host", "std::fs", "std::process"] { assert!(!source.contains(forbidden), "forbidden {forbidden}"); }
        let inventory = full_rust_route_inventory();
        assert!(inventory.iter().any(|(path, methods)| *path == "/api/dns/records" && *methods == ["get", "post"]));
        assert!(inventory.iter().any(|(path, methods)| *path == "/api/dns/records/status" && *methods == ["post"]));
        assert!(inventory.iter().any(|(path, methods)| *path == "/api/dns/records/:name" && *methods == ["delete"]));
        let response = dns_response("/api/dns/records", CaduceusHttpReadback { ok: false, status: 422, path: "/api/v1/network/dns".to_string(), body: serde_json::json!({"ok":false,"firstMissingSignal":"address-private-required","validation":{"address":"private"}}), first_missing_signal: "address-private-required".to_string() });
        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn dns_status_forwards_document_and_attendance_without_capability() {
        use std::io::{BufRead, BufReader, Read, Write};

        let _guard = CADUCEUS_ENV_LOCK.get_or_init(|| std::sync::Mutex::new(())).lock().unwrap();
        let root = test_tab_root("dns-status-document-forward");
        let _origin = note_test_origin(&root);
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let witness = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut reader = BufReader::new(stream.try_clone().unwrap());
            let mut request = String::new();
            let mut content_length = 0usize;
            loop {
                let mut line = String::new();
                reader.read_line(&mut line).unwrap();
                if let Some((name, value)) = line.split_once(':') {
                    if name.eq_ignore_ascii_case("content-length") {
                        content_length = value.trim().parse().unwrap();
                    }
                }
                request.push_str(&line);
                if line == "\r\n" {
                    break;
                }
            }
            let mut body = vec![0; content_length];
            reader.read_exact(&mut body).unwrap();
            request.push_str(std::str::from_utf8(&body).unwrap());
            let response_body = r#"{"ok":true,"records":[]}"#;
            stream
                .write_all(format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}", response_body.len(), response_body).as_bytes())
                .unwrap();
            request
        });
        let _base = ScopedEnv::set("CADUCEUS_BASE_URL", format!("http://{address}"));
        let response = app(AppState { tab_root: Arc::new(root) })
            .oneshot(successor_admin_request(
                Request::builder()
                    .method("POST")
                    .uri("/api/dns/records/status")
                    .header("content-type", "application/json")
                    .body(Body::from("{}"))
                    .unwrap(),
            ))
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let request = witness.join().unwrap();
        assert!(request.starts_with("POST /api/v1/network/dns HTTP/1.1\r\n"), "{request}");
        assert!(request.contains("x-caduceus-document: test-document\r\n"), "{request}");
        assert!(request.contains("x-caduceus-attendance: test-attendance\r\n"), "{request}");
        assert!(!request.to_ascii_lowercase().contains("x-caduceus-capability:"), "{request}");
        assert!(request.ends_with(r#"{"action":"status"}"#), "{request}");
    }

    #[tokio::test]
    async fn dns_status_guest_and_cross_origin_refuse_before_caduceus_contact() {
        let _guard = CADUCEUS_ENV_LOCK.get_or_init(|| std::sync::Mutex::new(())).lock().unwrap();
        let root = test_tab_root("dns-status-refusal");
        let _origin = note_test_origin(&root);
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        listener.set_nonblocking(true).unwrap();
        let address = listener.local_addr().unwrap();
        let outbound = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let observed = outbound.clone();
        let witness = std::thread::spawn(move || {
            let deadline = std::time::Instant::now() + std::time::Duration::from_millis(200);
            while std::time::Instant::now() < deadline {
                match listener.accept() {
                    Ok(_) => {
                        observed.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                    }
                    Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                        std::thread::sleep(std::time::Duration::from_millis(5));
                    }
                    Err(error) => panic!("unexpected listener error: {error}"),
                }
            }
        });
        let _base = ScopedEnv::set("CADUCEUS_BASE_URL", format!("http://{address}"));
        let router = app(AppState { tab_root: Arc::new(root) });
        let guest = router
            .clone()
            .oneshot(successor_session_request(
                Request::builder()
                    .method("POST")
                    .uri("/api/dns/records/status")
                    .header("content-type", "application/json")
                    .body(Body::from("{}"))
                    .unwrap(),
                false,
            ))
            .await
            .unwrap();
        assert_eq!(guest.status(), StatusCode::UNAUTHORIZED);
        let cross_origin = router
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/dns/records/status")
                    .header("content-type", "application/json")
                    .body(Body::from("{}"))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(cross_origin.status(), StatusCode::FORBIDDEN);
        witness.join().unwrap();
        assert_eq!(outbound.load(std::sync::atomic::Ordering::SeqCst), 0);
    }
