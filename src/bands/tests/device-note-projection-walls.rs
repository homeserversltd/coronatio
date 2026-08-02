#[tokio::test]
async fn device_note_put_forwards_exact_json_and_attendance_to_caduceus() {
    let _guard = CADUCEUS_ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
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
                if name.eq_ignore_ascii_case("content-length") { content_length = value.trim().parse().unwrap(); }
            }
            request.push_str(&line);
            if line == "\r\n" { break; }
        }
        let mut body = vec![0; content_length];
        reader.read_exact(&mut body).unwrap();
        request.push_str(std::str::from_utf8(&body).unwrap());
        let response_body = r#"{"ok":true,"completed":true,"notes":{"aa:bb:cc:dd:ee:ff":"Desk & <lamp>"}}"#;
        stream.write_all(format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}", response_body.len(), response_body).as_bytes()).unwrap();
        request
    });
    std::env::set_var("CADUCEUS_BASE_URL", format!("http://{address}"));
    let router = app(AppState { tab_root: Arc::new(test_tab_root("device-note-put")) });
    let response = router.oneshot(successor_admin_request(
        Request::builder().method("PUT").uri("/api/network/notes").header("content-type", "application/json")
            .body(Body::from(r#"{"mac":"aa:bb:cc:dd:ee:ff","note":"Desk & <lamp>"}"#)).unwrap()
    )).await.unwrap();
    std::env::remove_var("CADUCEUS_BASE_URL");
    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = serde_json::from_slice(&axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap()).unwrap();
    assert_eq!(body["notes"]["aa:bb:cc:dd:ee:ff"], "Desk & <lamp>");
    let request = witness.join().unwrap();
    assert!(request.starts_with("PUT /api/v1/network/notes HTTP/1.1\r\n"), "{request}");
    assert!(request.contains("x-caduceus-attendance: test-attendance\r\n"), "{request}");
    assert!(request.ends_with(r#"{"mac":"aa:bb:cc:dd:ee:ff","note":"Desk & <lamp>"}"#), "{request}");
}

#[tokio::test]
async fn device_note_get_projects_caduceus_notes_and_stats_shell_preserves_leases_on_note_failure() {
    let _guard = CADUCEUS_ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let witness = std::thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let mut reader = BufReader::new(stream.try_clone().unwrap());
        let mut request = String::new();
        reader.read_line(&mut request).unwrap();
        loop { let mut line = String::new(); reader.read_line(&mut line).unwrap(); if line == "\r\n" { break; } }
        let response_body = r#"{"ok":true,"notes":{"aa:bb:cc:dd:ee:ff":"Saved note"}}"#;
        stream.write_all(format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}", response_body.len(), response_body).as_bytes()).unwrap();
        request
    });
    std::env::set_var("CADUCEUS_BASE_URL", format!("http://{address}"));
    let response = app(AppState { tab_root: Arc::new(test_tab_root("device-note-get")) }).oneshot(Request::builder().uri("/api/network/notes").body(Body::empty()).unwrap()).await.unwrap();
    std::env::remove_var("CADUCEUS_BASE_URL");
    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = serde_json::from_slice(&axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap()).unwrap();
    assert_eq!(body["notes"]["aa:bb:cc:dd:ee:ff"], "Saved note");
    assert_eq!(witness.join().unwrap(), "GET /api/v1/network/notes HTTP/1.1\r\n");
    let shell = std::fs::read_to_string("src/bands/shell/document-4.rs").unwrap();
    assert!(shell.contains("notesResponse.ok ? normalizeNetworkNotes(await notesResponse.json()) : {}"));
    assert!(shell.contains("node.textContent = note"));
    assert!(shell.contains("escapeHtml(note)"));
}
