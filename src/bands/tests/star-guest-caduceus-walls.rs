#[tokio::test]
async fn guest_star_set_reaches_caduceus_without_capability_or_attendance() {
    let _config_guard = HX_EXEMPLAR_ENV_LOCK
        .get_or_init(|| std::sync::Mutex::new(()))
        .lock()
        .unwrap();
    let _caduceus_guard = CADUCEUS_ENV_LOCK
        .get_or_init(|| std::sync::Mutex::new(()))
        .lock()
        .unwrap();
    use std::io::{Read, Write};
    use std::net::{Shutdown, TcpListener};
    use std::sync::{Arc, Mutex};
    use std::thread;

    let root = test_tab_root("guest-star-caduceus");
    let config = root.join("homeserver.json");
    std::fs::write(
        &config,
        serde_json::json!({
            "tabs": {
                "starred": "stats",
                "stats": {"config": {"isEnabled": true, "adminOnly": false}, "visibility": {"tab": true}},
                "portals": {"config": {"isEnabled": true, "adminOnly": false}, "visibility": {"tab": true}}
            }
        })
        .to_string(),
    )
    .unwrap();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    let captured = Arc::new(Mutex::new(String::new()));
    let captured_thread = captured.clone();
    let server = thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let mut buffer = [0_u8; 4096];
        let count = stream.read(&mut buffer).unwrap();
        *captured_thread.lock().unwrap() = String::from_utf8_lossy(&buffer[..count]).to_string();
        let body = r#"{"ok":true,"firstMissingSignal":"none"}"#;
        stream
            .write_all(
                format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nConnection: close\r\nContent-Length: {}\r\n\r\n{}",
                    body.len(), body
                )
                .as_bytes(),
            )
            .unwrap();
        stream.shutdown(Shutdown::Write).unwrap();
    });
    std::env::set_var("CORONATIO_HOMESERVER_JSON", &config);
    std::env::set_var("CADUCEUS_BASE_URL", format!("http://127.0.0.1:{port}"));

    let response = app(AppState { tab_root: Arc::new(root) })
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/set_starred_tab")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"tabName":"portals"}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    server.join().unwrap();
    std::env::remove_var("CADUCEUS_BASE_URL");
    std::env::remove_var("CORONATIO_HOMESERVER_JSON");

    let request = captured.lock().unwrap().clone();
    assert!(request.starts_with("POST /api/v1/config/set HTTP/1.1"), "{request}");
    assert!(request.contains(r#"{"path":"tabs.starred","value":"portals"}"#), "{request}");
    assert!(!request.to_ascii_lowercase().contains("x-caduceus-capability:"), "{request}");
    assert!(!request.to_ascii_lowercase().contains("x-caduceus-attendance:"), "{request}");
}