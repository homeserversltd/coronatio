struct ScopedEnv {
    key: &'static str,
    previous: Option<std::ffi::OsString>,
}

impl ScopedEnv {
    fn set(key: &'static str, value: impl AsRef<std::ffi::OsStr>) -> Self {
        let previous = std::env::var_os(key);
        std::env::set_var(key, value);
        Self { key, previous }
    }
}

impl Drop for ScopedEnv {
    fn drop(&mut self) {
        match self.previous.take() {
            Some(value) => std::env::set_var(self.key, value),
            None => std::env::remove_var(self.key),
        }
    }
}

fn note_test_origin(root: &std::path::Path) -> ScopedEnv {
    let config = root.join("homeserver.json");
    std::fs::write(
        &config,
        r#"{"global":{"cors":{"allowed_origins":["https://home.arpa"]}}}"#,
    )
    .unwrap();
    ScopedEnv::set("CORONATIO_HOMESERVER_JSON", config.as_os_str())
}

#[tokio::test]
async fn device_note_put_forwards_exact_json_document_and_attendance_to_caduceus() {
    let _guard = CADUCEUS_ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
    let root = test_tab_root("device-note-put");
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
        let response_body = r#"{"ok":true,"completed":true,"notes":{"AA:BB:CC:DD:EE:FF":"Desk & <lamp>"}}"#;
        stream
            .write_all(
                format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}", response_body.len(), response_body).as_bytes(),
            )
            .unwrap();
        request
    });
    let _base = ScopedEnv::set("CADUCEUS_BASE_URL", format!("http://{address}"));
    let response = app(AppState { tab_root: Arc::new(root) })
        .oneshot(successor_admin_request(
            Request::builder()
                .method("PUT")
                .uri("/api/network/notes")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"mac":"aa-bb-cc-dd-ee-ff","note":"Desk & <lamp>"}"#))
                .unwrap(),
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = serde_json::from_slice(&axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap()).unwrap();
    assert_eq!(body["notes"]["AA:BB:CC:DD:EE:FF"], "Desk & <lamp>");
    assert!(body.get("document").is_none());
    assert!(body.get("attendance").is_none());
    let request = witness.join().unwrap();
    assert!(request.starts_with("PUT /api/v1/network/notes HTTP/1.1\r\n"));
    assert!(request.contains("x-caduceus-document: test-document\r\n"));
    assert!(request.contains("x-caduceus-attendance: test-attendance\r\n"));
    assert!(request.ends_with(r#"{"mac":"AA:BB:CC:DD:EE:FF","note":"Desk & <lamp>"}"#));
}

#[tokio::test]
async fn device_note_put_refuses_accepted_only_caduceus_readback() {
    let _guard = CADUCEUS_ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
    let root = test_tab_root("device-note-accepted-only");
    let _origin = note_test_origin(&root);
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let witness = std::thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let response_body = r#"{"ok":true,"completed":true,"notes":{"aa:bb:cc:dd:ee:ff":"Desk"}}"#;
        stream
            .write_all(
                format!("HTTP/1.1 202 Accepted\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}", response_body.len(), response_body).as_bytes(),
            )
            .unwrap();
    });
    let _base = ScopedEnv::set("CADUCEUS_BASE_URL", format!("http://{address}"));
    let response = app(AppState { tab_root: Arc::new(root) })
        .oneshot(successor_admin_request(
            Request::builder()
                .method("PUT")
                .uri("/api/network/notes")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"mac":"aa:bb:cc:dd:ee:ff","note":"Desk"}"#))
                .unwrap(),
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    let body: serde_json::Value = serde_json::from_slice(&axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap()).unwrap();
    assert_eq!(body["ok"], false);
    assert_eq!(body["accepted"], false);
    assert_eq!(body["completed"], false);
    assert_eq!(body["notes"], serde_json::json!({}));
    witness.join().unwrap();
}

#[tokio::test]
async fn device_note_put_refuses_guest_before_outbound_caduceus_call() {
    let _guard = CADUCEUS_ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
    let root = test_tab_root("device-note-guest-refusal");
    let _origin = note_test_origin(&root);
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let address = listener.local_addr().unwrap();
    let outbound = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let observed = outbound.clone();
    let witness = std::thread::spawn(move || {
        let deadline = std::time::Instant::now() + std::time::Duration::from_millis(200);
        while std::time::Instant::now() < deadline {
            match listener.accept() {
                Ok(_) => {
                    observed.store(true, std::sync::atomic::Ordering::SeqCst);
                    return;
                }
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => std::thread::sleep(std::time::Duration::from_millis(5)),
                Err(error) => panic!("unexpected listener error: {error}"),
            }
        }
    });
    let _base = ScopedEnv::set("CADUCEUS_BASE_URL", format!("http://{address}"));
    let response = app(AppState { tab_root: Arc::new(root) })
        .oneshot(successor_session_request(
            Request::builder()
                .method("PUT")
                .uri("/api/network/notes")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"mac":"aa:bb:cc:dd:ee:ff","note":"Guest"}"#))
                .unwrap(),
            false,
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    let body: serde_json::Value = serde_json::from_slice(&axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap()).unwrap();
    assert_eq!(body["ok"], false);
    assert_eq!(body["accepted"], false);
    assert_eq!(body["completed"], false);
    witness.join().unwrap();
    assert!(!outbound.load(std::sync::atomic::Ordering::SeqCst));
}

#[tokio::test]
async fn device_note_put_refuses_invalid_mac_before_outbound_caduceus_call() {
    let _guard = CADUCEUS_ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let address = listener.local_addr().unwrap();
    let outbound = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let observed = outbound.clone();
    let witness = std::thread::spawn(move || {
        let deadline = std::time::Instant::now() + std::time::Duration::from_millis(200);
        while std::time::Instant::now() < deadline {
            match listener.accept() {
                Ok(_) => {
                    observed.store(true, std::sync::atomic::Ordering::SeqCst);
                    return;
                }
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => std::thread::sleep(std::time::Duration::from_millis(5)),
                Err(error) => panic!("unexpected listener error: {error}"),
            }
        }
    });
    let _base = ScopedEnv::set("CADUCEUS_BASE_URL", format!("http://{address}"));
    let response = app(AppState { tab_root: Arc::new(test_tab_root("device-note-invalid-mac")) })
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri("/api/network/notes")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"mac":"aa:bb-cc:dd:ee:ff","note":"Invalid"}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body: serde_json::Value = serde_json::from_slice(&axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap()).unwrap();
    assert_eq!(body["error"], "network-note-payload-invalid");
    witness.join().unwrap();
    assert!(!outbound.load(std::sync::atomic::Ordering::SeqCst));
}

#[tokio::test]
async fn device_note_get_projects_caduceus_notes_and_stats_liveness_roster_keeps_the_note_capability() {
    let _guard = CADUCEUS_ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let witness = std::thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let mut reader = BufReader::new(stream.try_clone().unwrap());
        let mut request = String::new();
        reader.read_line(&mut request).unwrap();
        loop {
            let mut line = String::new();
            reader.read_line(&mut line).unwrap();
            if line == "\r\n" {
                break;
            }
        }
        let response_body = r#"{"ok":true,"notes":{"AA:BB:CC:DD:EE:FF":"Saved note"}}"#;
        stream
            .write_all(
                format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}", response_body.len(), response_body).as_bytes(),
            )
            .unwrap();
        request
    });
    let _base = ScopedEnv::set("CADUCEUS_BASE_URL", format!("http://{address}"));
    let response = app(AppState { tab_root: Arc::new(test_tab_root("device-note-get")) })
        .oneshot(Request::builder().uri("/api/network/notes").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = serde_json::from_slice(&axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap()).unwrap();
    assert_eq!(body["notes"]["AA:BB:CC:DD:EE:FF"], "Saved note");
    assert_eq!(witness.join().unwrap(), "GET /api/v1/network/notes HTTP/1.1\r\n");
    let shell = std::fs::read_to_string("src/bands/shell/document-4.rs").unwrap();
    let roster = std::fs::read_to_string("src/bands/shell/dhcp-client.rs").unwrap();
    assert!(shell.contains("fetch('/api/network/notes', { cache: 'no-store' })"));
    assert!(shell.contains("function ensureNoteModal()"));
    assert!(shell.contains("function openNoteModal(mac, note)"));
    assert!(shell.contains("fetch('/api/network/notes', { method: 'PUT'"));
    assert!(shell.contains("JSON.stringify({ mac, note })"));
    assert!(shell.contains("identityState.notes[canonicalMac] = note; renderStatsRoster()"));
    assert!(roster.contains("<th>Note</th>"));
    assert!(roster.contains("class=\"device-note-cell\""));
    assert!(roster.contains("data-edit-note-button"));
    assert!(roster.contains("headerState.isAdmin ? `<button"));
    assert!(roster.contains("${dhcpMasked(note, 'hostname')}"));
}
