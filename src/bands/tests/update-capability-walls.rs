    #[tokio::test(flavor = "current_thread")]
    async fn admin_update_sends_caduceus_update_now_capability() {
        use std::net::TcpListener;

        fn mock_keyman_mint(action: &str, target: &str) -> Result<String, String> {
            Ok(format!("keyman-mock::{action}::{target}"))
        }

        let _guard = CADUCEUS_ENV_LOCK
            .get_or_init(|| std::sync::Mutex::new(()))
            .lock()
            .unwrap();
        *KEYMAN_CAPABILITY_MINT_MOCK
            .get_or_init(|| std::sync::Mutex::new(None))
            .lock()
            .unwrap() = Some(mock_keyman_mint);

        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        std::env::set_var("CADUCEUS_URL", format!("http://127.0.0.1:{port}"));
        let server = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            stream.set_read_timeout(Some(Duration::from_secs(2))).unwrap();
            let mut request = String::new();
            stream.read_to_string(&mut request).ok();
            let header = request
                .lines()
                .find_map(|line| {
                    let (name, value) = line.split_once(':')?;
                    name.eq_ignore_ascii_case("x-caduceus-capability")
                        .then(|| value.trim().to_string())
                })
                .expect("Keyman-minted capability header");
            assert_eq!(header, "keyman-mock::update now::local");
            assert!(request.starts_with("POST /api/v1/update/now HTTP/1.1"), "{request}");
            stream.write_all(b"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: 39\r\nConnection: close\r\n\r\n{\"ok\":true,\"firstMissingSignal\":\"none\"}").unwrap();
        });

        let token = authorize_test_admin_token();
        let response = app(AppState { tab_root: Arc::new(test_tab_root("update-capability")) })
            .oneshot(Request::builder().method("POST").uri("/admit/admin/action/update").header("X-Admin-Token", token).body(Body::empty()).unwrap())
            .await
            .unwrap();
        server.join().unwrap();
        std::env::remove_var("CADUCEUS_URL");
        *KEYMAN_CAPABILITY_MINT_MOCK.get().unwrap().lock().unwrap() = None;

        assert_eq!(response.status(), StatusCode::OK);
        let body = String::from_utf8(axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        assert!(body.contains("Caduceus accepted the action."), "{body}");
        assert!(body.contains("<code>none</code>"), "{body}");
    }

    #[test]
    fn update_mutation_clients_name_capability_header_and_no_fire_and_forget_success() {
        let source = std::fs::read_to_string("src/bands/caduceus.rs").unwrap();
        assert!(source.contains("x-caduceus-capability"));
        assert!(source.contains("caduceus_http_json_with_capability"));
        assert!(source.contains("caduceus_http_with_capability"));
        assert!(source.contains("/usr/local/sbin/caduceus-keyman-sign-capability"));
        assert!(!source.contains("household_signing_key"));
        assert!(!source.contains("CORONATIO_CADUCEUS_SIGNING_KEY"));
        assert!(!source.contains("/etc/caduceus/household-signing.key"));
        assert!(!source.contains("fn caduceus_dispatch_route"));
        assert!(!source.contains("thread::spawn"));
    }

    #[test]
    fn config_set_mints_exact_capability_and_posts_json_without_local_write() {
        use std::net::TcpListener;
        let _guard = CADUCEUS_ENV_LOCK.get_or_init(|| std::sync::Mutex::new(())).lock().unwrap();
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let server = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            stream.set_read_timeout(Some(Duration::from_millis(250))).unwrap();
            let mut request = String::new();
            stream.read_to_string(&mut request).ok();
            assert!(request.starts_with("POST /api/v1/config/set HTTP/1.1"), "{request}");
            assert!(request.contains("x-caduceus-capability: config-test-token\r\n"), "{request}");
            assert!(request.contains(r#"{"path":"tabs.starred","value":"portals"}"#), "{request}");
            stream.write_all(b"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: 11\r\nConnection: close\r\n\r\n{\"ok\":true}").unwrap();
        });
        std::env::set_var("CADUCEUS_URL", format!("http://{address}"));
        std::env::set_var("CORONATIO_TEST_CAPABILITY_TOKEN", "config-test-token");
        let readback = caduceus_config_set("tabs.starred", serde_json::json!("portals"));
        server.join().unwrap();
        std::env::remove_var("CORONATIO_TEST_CAPABILITY_TOKEN");
        std::env::remove_var("CADUCEUS_URL");
        assert!(readback.ok, "{readback:?}");
        let routes = std::fs::read_to_string("src/bands/routes.rs").unwrap();
        assert!(!routes.contains("persist_iris_facts"));
        assert!(!routes.contains("std::fs::write(&tmp"));
    }

    #[test]
    fn keyman_mint_uses_last_json_line_and_keeps_bare_token_fallback() {
        let polluted = "Acquired key for caduceus_household\nAcquired key for caduceus_household\n{\"ok\":true,\"capability\":\"signed-token\",\"firstMissingSignal\":\"none\"}\n";
        assert_eq!(parse_keyman_capability_stdout(polluted).unwrap(), "signed-token");
        assert_eq!(
            parse_keyman_capability_stdout("diagnostic\nBearer bare-token\n").unwrap(),
            "bare-token"
        );
    }
