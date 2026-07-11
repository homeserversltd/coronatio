    #[tokio::test(flavor = "current_thread")]
    async fn admin_update_sends_caduceus_accepted_signed_capability() {
        use ed25519_dalek::{Signature, Verifier};
        use std::net::TcpListener;

        let _guard = CADUCEUS_ENV_LOCK
            .get_or_init(|| std::sync::Mutex::new(()))
            .lock()
            .unwrap();
        let seed_hex = "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60";
        std::env::set_var("CORONATIO_CADUCEUS_SIGNING_KEY", seed_hex);

        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        std::env::set_var("CADUCEUS_URL", format!("http://127.0.0.1:{port}"));
        let expected_key = household_signing_key().unwrap().verifying_key();
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
                .expect("signed capability header");
            let (payload_b64, signature_b64) = header.split_once('.').unwrap();
            let payload = URL_SAFE_NO_PAD.decode(payload_b64).unwrap();
            let signature = Signature::from_slice(&URL_SAFE_NO_PAD.decode(signature_b64).unwrap()).unwrap();
            expected_key.verify(&payload, &signature).expect("Caduceus verifying key accepts capability");
            let payload: serde_json::Value = serde_json::from_slice(&payload).unwrap();
            assert_eq!(payload["actor"], "coronatio-admin-session");
            assert_eq!(payload["action"], "staff intent");
            assert_eq!(payload["target"], "/api/admin/updates/apply");
            assert!(request.contains("\"route\":\"/api/admin/updates/apply\""));
            stream.write_all(b"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: 39\r\nConnection: close\r\n\r\n{\"ok\":true,\"firstMissingSignal\":\"none\"}").unwrap();
        });

        let token = authorize_test_admin_token();
        let response = app(AppState { tab_root: Arc::new(test_tab_root("update-capability")) })
            .oneshot(Request::builder().method("POST").uri("/admit/admin/action/update").header("X-Admin-Token", token).body(Body::empty()).unwrap())
            .await
            .unwrap();
        server.join().unwrap();
        std::env::remove_var("CADUCEUS_URL");
        std::env::remove_var("CORONATIO_CADUCEUS_SIGNING_KEY");

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
        assert!(!source.contains("fn caduceus_dispatch_route"));
        assert!(!source.contains("thread::spawn"));
    }
