    use std::io::{Read, Write};
    use std::net::{Shutdown, TcpListener};
    use std::sync::Arc;
    use std::thread;

    struct MockCaduceusHyalosServer {
        port: u16,
        _handle: thread::JoinHandle<()>,
    }

    impl MockCaduceusHyalosServer {
        fn spawn(events: serde_json::Value) -> Self {
            let listener = TcpListener::bind("127.0.0.1:0").expect("mock caduceus bind");
            let port = listener.local_addr().expect("mock caduceus addr").port();
            let body = Arc::new(
                serde_json::json!({
                    "schema": "caduceus.hyalos.tail.v1",
                    "ok": true,
                    "events": events,
                    "firstMissingSignal": "none"
                })
                .to_string(),
            );
            let handle = thread::spawn(move || {
                for stream in listener.incoming().flatten() {
                    let mut stream = stream;
                    let mut buf = [0u8; 4096];
                    let _ = stream.read(&mut buf);
                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nConnection: close\r\nContent-Length: {}\r\n\r\n{}",
                        body.len(),
                        body
                    );
                    let _ = stream.write_all(response.as_bytes());
                    let _ = stream.shutdown(Shutdown::Write);
                }
            });
            Self { port, _handle: handle }
        }
    }

    #[tokio::test]
    async fn hyalos_consumer_upload_history_reads_tail_filters_noise_and_reverses() {
        let _guard = CADUCEUS_ENV_LOCK.get_or_init(|| std::sync::Mutex::new(())).lock().unwrap();
        let server = MockCaduceusHyalosServer::spawn(serde_json::json!([
            {"kind": "upload", "organ": "file-ingress", "message": "old success", "ok": true},
            {"kind": "upload", "organ": "file-ingress", "message": "[ERROR] hidden", "ok": false},
            {"kind": "upload", "organ": "file-ingress", "message": "failed to copy", "ok": false},
            {"kind": "upload", "organ": "file-ingress", "message": "[SYSTEM] rotate", "ok": true},
            {"kind": "upload", "organ": "file-ingress", "message": "new success", "ok": true},
        ]));
        std::env::set_var("CADUCEUS_BASE_URL", format!("http://127.0.0.1:{}", server.port));
        let response = app(AppState {
            tab_root: Arc::new(test_tab_root("hyalos-upload-history-app")),
        })
        .oneshot(
            Request::builder()
                .uri("/api/upload/history")
                .header("X-Admin-Token", authorize_test_admin_token())
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body: serde_json::Value =
            serde_json::from_slice(&axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap()).unwrap();
        assert_eq!(body["schema"], "coronatio.upload.history.v1");
        assert_eq!(body["history"], serde_json::json!(["new success", "old success"]));
        std::env::remove_var("CADUCEUS_BASE_URL");
    }

    #[tokio::test]
    async fn hyalos_consumer_clear_routes_refuse_append_only_channel() {
        let _guard = CADUCEUS_ENV_LOCK.get_or_init(|| std::sync::Mutex::new(())).lock().unwrap();
        let router = app(AppState {
            tab_root: Arc::new(test_tab_root("hyalos-clear-refusal-app")),
        });
        let token = authorize_test_admin_token();
        for (path, schema) in [
            ("/api/upload/history/clear", "coronatio.upload.history.clear.v1"),
            (
                "/api/admin/logs/homeserver/clear",
                "coronatio.admin.logs.homeserver.clear.v1",
            ),
        ] {
            let response = router
                .clone()
                .oneshot(
                    Request::builder()
                        .method("POST")
                        .uri(path)
                        .header("X-Admin-Token", token.clone())
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();
            assert_eq!(response.status(), StatusCode::GONE, "{path}");
            let body: serde_json::Value = serde_json::from_slice(
                &axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap(),
            )
            .unwrap();
            assert_eq!(body["schema"], schema, "{path}");
            assert_eq!(body["error"], "hyalos-channel-append-only", "{path}");
            assert_eq!(body["firstMissingSignal"], "hyalos-channel-append-only", "{path}");
        }
    }

    #[tokio::test]
    async fn hyalos_consumer_homeserver_logs_reads_broad_tail() {
        let _guard = CADUCEUS_ENV_LOCK.get_or_init(|| std::sync::Mutex::new(())).lock().unwrap();
        let server = MockCaduceusHyalosServer::spawn(serde_json::json!([
            {"kind": "system", "organ": "coronatio", "message": "boot complete", "ok": true},
            {"kind": "upload", "organ": "file-ingress", "message": "file saved", "ok": true},
        ]));
        std::env::set_var("CADUCEUS_BASE_URL", format!("http://127.0.0.1:{}", server.port));
        let response = app(AppState {
            tab_root: Arc::new(test_tab_root("hyalos-homeserver-logs-app")),
        })
        .oneshot(
            Request::builder()
                .uri("/api/admin/logs/homeserver")
                .header("X-Admin-Token", authorize_test_admin_token())
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body: serde_json::Value =
            serde_json::from_slice(&axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap()).unwrap();
        assert_eq!(body["schema"], "coronatio.admin.logs.homeserver.v1");
        assert_eq!(body["logs"], "boot complete\nfile saved");
        assert_eq!(body["events"].as_array().map(Vec::len), Some(2));
        std::env::remove_var("CADUCEUS_BASE_URL");
    }