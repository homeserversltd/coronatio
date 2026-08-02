fn hestia_mock_response(status: &str, headers: &[(&str, &str)], body: &[u8]) -> Vec<u8> {
    let mut response = format!("HTTP/1.1 {status}\r\n").into_bytes();
    for (name, value) in headers {
        response.extend_from_slice(format!("{name}: {value}\r\n").as_bytes());
    }
    response.extend_from_slice(b"\r\n");
    response.extend_from_slice(body);
    response
}

fn spawn_hestia_mock(response: Vec<u8>) -> (u16, Arc<Mutex<Vec<u8>>>, std::thread::JoinHandle<()>) {
    use std::net::TcpListener;
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    let request = Arc::new(Mutex::new(Vec::new()));
    let captured = request.clone();
    let handle = std::thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        stream
            .set_read_timeout(Some(Duration::from_secs(2)))
            .unwrap();
        let mut bytes = Vec::new();
        let mut buffer = [0_u8; 1024];
        loop {
            let count = stream.read(&mut buffer).unwrap();
            bytes.extend_from_slice(&buffer[..count]);
            if count == 0 || bytes.windows(4).any(|window| window == b"\r\n\r\n") {
                break;
            }
        }
        *captured.lock().unwrap() = bytes;
        stream.write_all(&response).unwrap();
    });
    (port, request, handle)
}

fn unused_hestia_port() -> u16 {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    listener.local_addr().unwrap().port()
}

#[tokio::test(flavor = "current_thread")]
async fn hestia_windows_proxy_preserves_der_bytes_and_attachment_headers_only() {
    let _guard = CADUCEUS_ENV_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap();
    let der = [0x30, 0x82, 0x00, 0xff, 0x10, 0x00];
    let length = der.len().to_string();
    let response = hestia_mock_response(
        "200 OK",
        &[
            ("Content-Type", "application/x-x509-ca-cert"),
            (
                "Content-Disposition",
                "attachment; filename=\"homeserver-house-ca-windows.cer\"",
            ),
            ("Content-Length", &length),
            ("Set-Cookie", "secret=must-not-cross"),
            ("X-Caduceus-Attendance", "must-not-cross"),
        ],
        &der,
    );
    let (port, request, handle) = spawn_hestia_mock(response);
    std::env::set_var("CADUCEUS_BASE_URL", format!("http://127.0.0.1:{port}"));
    let response = app(AppState {
        tab_root: Arc::new(test_tab_root("hestia-windows")),
    })
    .oneshot(
        Request::builder()
            .uri("/api/admin/download-root-crt?platform=windows")
            .body(Body::empty())
            .unwrap(),
    )
    .await
    .unwrap();
    std::env::remove_var("CADUCEUS_BASE_URL");
    handle.join().unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers()[header::CONTENT_TYPE],
        "application/x-x509-ca-cert"
    );
    assert_eq!(
        response.headers()[header::CONTENT_DISPOSITION],
        "attachment; filename=\"homeserver-house-ca-windows.cer\""
    );
    assert_eq!(response.headers()[header::CONTENT_LENGTH], length);
    assert!(response.headers().get(header::SET_COOKIE).is_none());
    assert!(response.headers().get("x-caduceus-attendance").is_none());
    let body = axum::body::to_bytes(response.into_body(), HESTIA_BUNDLE_BODY_LIMIT)
        .await
        .unwrap();
    assert_eq!(body.as_ref(), der);
    let request = String::from_utf8(request.lock().unwrap().clone()).unwrap();
    assert!(
        request.starts_with("GET /api/v1/cert/bundle/download?platform=windows HTTP/1.1\r\n"),
        "{request}"
    );
    for forbidden in [
        "Cookie:",
        "Authorization:",
        "Attendance:",
        "X-Caduceus-Attendance:",
    ] {
        assert!(
            !request.contains(forbidden),
            "public bundle request forwarded {forbidden}: {request}"
        );
    }
}

#[tokio::test(flavor = "current_thread")]
async fn hestia_platform_defaults_to_linux_and_path_shaped_values_never_connect() {
    let _guard = CADUCEUS_ENV_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap();
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let port = listener.local_addr().unwrap().port();
    std::env::set_var("CADUCEUS_BASE_URL", format!("http://127.0.0.1:{port}"));
    let router = app(AppState {
        tab_root: Arc::new(test_tab_root("hestia-invalid")),
    });
    for value in [
        "../windows",
        "linux/../../secret",
        "windows%3Fpath=/etc/passwd",
        "ios",
        "",
    ] {
        let response = router
            .clone()
            .oneshot(
                Request::builder()
                    .uri(format!("/api/admin/download-root-crt?platform={value}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST, "{value}");
    }
    assert!(
        listener.accept().is_err(),
        "invalid platform contacted Caduceus"
    );
    drop(listener);

    let body = b"linux-ca";
    let length = body.len().to_string();
    let (port, request, handle) = spawn_hestia_mock(hestia_mock_response(
        "200 OK",
        &[
            ("Content-Type", "application/x-x509-ca-cert"),
            (
                "Content-Disposition",
                "attachment; filename=\"homeserver-house-ca-linux.crt\"",
            ),
            ("Content-Length", &length),
        ],
        body,
    ));
    std::env::set_var("CADUCEUS_BASE_URL", format!("http://127.0.0.1:{port}"));
    let response = router
        .oneshot(
            Request::builder()
                .uri("/api/admin/download-root-crt")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    std::env::remove_var("CADUCEUS_BASE_URL");
    handle.join().unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert!(String::from_utf8(request.lock().unwrap().clone())
        .unwrap()
        .contains("?platform=linux HTTP/1.1"));
}

#[tokio::test(flavor = "current_thread")]
async fn hestia_upstream_absence_and_error_have_safe_public_readback() {
    let _guard = CADUCEUS_ENV_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap();
    let port = unused_hestia_port();
    std::env::set_var("CADUCEUS_BASE_URL", format!("http://127.0.0.1:{port}"));
    let router = app(AppState {
        tab_root: Arc::new(test_tab_root("hestia-upstream-failure")),
    });
    let missing = router
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/admin/download-root-crt")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(missing.status(), StatusCode::SERVICE_UNAVAILABLE);
    let missing_body = String::from_utf8(
        axum::body::to_bytes(missing.into_body(), usize::MAX)
            .await
            .unwrap()
            .to_vec(),
    )
    .unwrap();
    assert!(missing_body.contains("household certificate bundle is unavailable"));
    assert!(!missing_body.contains("Connection refused"));

    let secret = b"internal upstream diagnostics must not cross";
    let length = secret.len().to_string();
    let (port, _, handle) = spawn_hestia_mock(hestia_mock_response(
        "404 Not Found",
        &[("Content-Length", &length)],
        secret,
    ));
    std::env::set_var("CADUCEUS_BASE_URL", format!("http://127.0.0.1:{port}"));
    let refused = router
        .oneshot(
            Request::builder()
                .uri("/api/admin/download-root-crt?platform=android")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    std::env::remove_var("CADUCEUS_BASE_URL");
    handle.join().unwrap();
    assert_eq!(refused.status(), StatusCode::NOT_FOUND);
    let refused_body = String::from_utf8(
        axum::body::to_bytes(refused.into_body(), usize::MAX)
            .await
            .unwrap()
            .to_vec(),
    )
    .unwrap();
    assert!(refused_body.contains("caduceus-bundle-refused"));
    assert!(!refused_body.contains("internal upstream diagnostics"));
}

#[tokio::test(flavor = "current_thread")]
async fn hestia_proxy_refuses_oversized_certificate_response() {
    let _guard = CADUCEUS_ENV_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap();
    let length = (HESTIA_BUNDLE_BODY_LIMIT + 1).to_string();
    let (port, _, handle) = spawn_hestia_mock(hestia_mock_response(
        "200 OK",
        &[
            ("Content-Type", "application/x-x509-ca-cert"),
            (
                "Content-Disposition",
                "attachment; filename=\"homeserver-house-ca-macos.pem\"",
            ),
            ("Content-Length", &length),
        ],
        b"",
    ));
    std::env::set_var("CADUCEUS_BASE_URL", format!("http://127.0.0.1:{port}"));
    let response = app(AppState {
        tab_root: Arc::new(test_tab_root("hestia-oversized")),
    })
    .oneshot(
        Request::builder()
            .uri("/api/admin/download-root-crt?platform=macos")
            .body(Body::empty())
            .unwrap(),
    )
    .await
    .unwrap();
    std::env::remove_var("CADUCEUS_BASE_URL");
    handle.join().unwrap();
    assert_eq!(response.status(), StatusCode::BAD_GATEWAY);
    let body = String::from_utf8(
        axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap()
            .to_vec(),
    )
    .unwrap();
    assert!(body.contains("caduceus-response-too-large"));
}

#[test]
fn hestia_modal_is_platform_aware_download_once_and_not_a_mutation() {
    let shell = render_crown_shell();
    for required in [
        "data-hestia-certificate-open",
        "openHestiaCertificateModal()",
        "Download Certificate",
        "Install once for this household root ring.",
        "Future service certificates beneath it need no new bundle.",
        "Trusted Root Certification Authorities",
        "Encryption & credentials",
        "chrome://settings/certificates",
        "sudo update-ca-certificates",
        "Keychain Access",
        "Firefox may use its own certificate store",
        "Chromium usually follows the operating system store",
        "homeserver-house-ca-windows.cer",
        "homeserver-house-ca-android.crt",
        "homeserver-house-ca-chromeos.crt",
        "homeserver-house-ca-linux.crt",
        "homeserver-house-ca-macos.crt",
    ] {
        assert!(
            shell.contains(required),
            "missing Hestia UX contract: {required}"
        );
    }
    assert!(shell
        .contains("const certificate = event.target.closest('[data-hestia-certificate-open]')"));
    assert!(
        shell.contains("certificate) { event.preventDefault(); openHestiaCertificateModal(); }")
    );
    assert!(
        !shell.contains(":3014"),
        "browser must stay on Coronatio same-origin routes"
    );
    assert!(!shell.contains("refresh-root-crt"));
    let hestia_source = shell
        .split("const hestiaPlatformDetails")
        .nth(1)
        .unwrap()
        .split("// UX-MIGRATION-SLICE-09B")
        .next()
        .unwrap();
    for forbidden in [
        "PKCS#12",
        ".p12",
        "homeserver bundle password",
        "Rotate Root",
        "iOS",
    ] {
        assert!(
            !hestia_source.contains(forbidden),
            "forbidden certificate behavior rendered: {forbidden}"
        );
    }
    let button = shell
        .split("data-hestia-certificate-open")
        .next()
        .unwrap()
        .rsplit("<button")
        .next()
        .unwrap();
    assert!(!button.contains("hx-post"));
    let inventory = full_rust_route_inventory();
    assert_eq!(inventory.iter().filter(|(path, methods)| *path == "/api/admin/download-root-crt" && methods == &["get"]).count(), 1);
    assert!(!inventory
        .iter()
        .any(|(path, _)| path.contains("refresh-root-crt")));
}
