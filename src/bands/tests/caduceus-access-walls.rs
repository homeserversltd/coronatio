#[derive(Clone)]
struct AccessFixtureReply {
    status: u16,
    body: String,
    delay: Duration,
    declared_length: Option<usize>,
}

impl AccessFixtureReply {
    fn json(body: serde_json::Value) -> Self {
        Self {
            status: 200,
            body: body.to_string(),
            delay: Duration::ZERO,
            declared_length: None,
        }
    }
}

struct AccessFixture {
    base: String,
    seen: Arc<Mutex<Vec<String>>>,
    worker: Option<std::thread::JoinHandle<()>>,
}

impl AccessFixture {
    fn start(replies: Vec<AccessFixtureReply>) -> Self {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let base = format!("http://{}", listener.local_addr().unwrap());
        let seen = Arc::new(Mutex::new(Vec::new()));
        let witness = Arc::clone(&seen);
        let worker = std::thread::spawn(move || {
            for reply in replies {
                let (stream, _) = listener.accept().unwrap();
                let mut reader = BufReader::new(stream.try_clone().unwrap());
                let mut request = String::new();
                loop {
                    let mut line = String::new();
                    reader.read_line(&mut line).unwrap();
                    request.push_str(&line);
                    if line == "\r\n" {
                        break;
                    }
                }
                let length = request
                    .lines()
                    .find_map(|line| line.strip_prefix("Content-Length: "))
                    .and_then(|raw| raw.parse::<usize>().ok())
                    .unwrap_or(0);
                let mut body = vec![0u8; length];
                reader.read_exact(&mut body).unwrap();
                request.push_str(&String::from_utf8_lossy(&body));
                witness.lock().unwrap().push(request);
                if !reply.delay.is_zero() {
                    std::thread::sleep(reply.delay);
                }
                let declared = reply.declared_length.unwrap_or(reply.body.len());
                let response = format!(
                    "HTTP/1.1 {} Fixture\r\nContent-Type: application/json\r\nContent-Length: {declared}\r\nConnection: close\r\n\r\n{}",
                    reply.status, reply.body
                );
                let mut stream = stream;
                let _ = stream.write_all(response.as_bytes());
            }
        });
        Self { base, seen, worker: Some(worker) }
    }

    fn finish(mut self) -> Vec<String> {
        self.worker.take().unwrap().join().unwrap();
        self.seen.lock().unwrap().clone()
    }
}

fn access_success(operation: AccessOperation) -> AccessFixtureReply {
    let body = match operation {
        AccessOperation::SessionMint => serde_json::json!({"ok":true,"code":"none","ticket":"fixture-session-ticket"}),
        AccessOperation::CapabilityMint => serde_json::json!({"ok":true,"code":"none","capability":"fixture-capability-token"}),
        _ => serde_json::json!({"ok":true,"code":"none"}),
    };
    AccessFixtureReply::json(body)
}

#[test]
fn successor_access_operations_map_to_exact_private_caduceus_routes() {
    let operations = [
        AccessOperation::SessionMint,
        AccessOperation::SessionProve,
        AccessOperation::SessionClear,
        AccessOperation::CapabilityMint,
    ];
    let fixture = AccessFixture::start(operations.into_iter().map(access_success).collect());
    let client = CaduceusAccessClient::new(fixture.base.clone());
    let ticket = SessionTicket::parse("fixture-session-ticket").unwrap();
    let calls = [
        client.session_mint("fixture-pin-981"),
        client.session_prove(&ticket),
        client.session_clear(&ticket),
        client.capability_mint(&ticket, "update-now", "local"),
    ];
    for (call, operation) in calls.iter().zip(operations) {
        assert!(call.receipt.ok, "{:?}", call.receipt);
        assert_eq!(call.receipt.operation, operation.name());
    }
    assert!(calls[0].ticket.is_some());
    assert!(calls[3].capability.is_some());
    let requests = fixture.finish();
    for (request, operation) in requests.iter().zip(operations) {
        assert!(request.starts_with(&format!("POST {} HTTP/1.1", operation.path())));
        assert!(request.contains("\r\nContent-Type: application/json\r\n"));
    }
    assert!(requests[0].contains("{\"pin\":\"fixture-pin-981\"}"));
    assert!(requests[1].contains("{\"ticket\":\"fixture-session-ticket\"}"));
    assert!(requests[3].contains("\"action\":\"update-now\""));
}

#[test]
fn successor_access_refuses_unavailable_timeout_oversized_and_malformed_responses() {
    let unavailable = CaduceusAccessClient::with_timeout("http://127.0.0.1:9", Duration::from_millis(20));
    assert_eq!(unavailable.session_mint("fixture-pin-981").receipt.code, "caduceus-access-unavailable");

    let timeout_fixture = AccessFixture::start(vec![AccessFixtureReply { status: 200, body: "{}".to_string(), delay: Duration::from_millis(100), declared_length: None }]);
    let timeout = CaduceusAccessClient::with_timeout(timeout_fixture.base.clone(), Duration::from_millis(10));
    assert_eq!(timeout.session_mint("fixture-pin-981").receipt.code, "caduceus-access-malformed-response");
    let _ = timeout_fixture.finish();

    let oversized_fixture = AccessFixture::start(vec![AccessFixtureReply { status: 200, body: String::new(), delay: Duration::ZERO, declared_length: Some(CADUCEUS_ACCESS_MAX_RESPONSE + 1) }]);
    let oversized = CaduceusAccessClient::new(oversized_fixture.base.clone()).session_mint("fixture-pin-981");
    assert_eq!(oversized.receipt.code, "caduceus-access-oversized-response");
    let _ = oversized_fixture.finish();

    let malformed_fixture = AccessFixture::start(vec![AccessFixtureReply { status: 200, body: "not-json".to_string(), delay: Duration::ZERO, declared_length: None }]);
    let malformed = CaduceusAccessClient::new(malformed_fixture.base.clone()).session_mint("fixture-pin-981");
    assert_eq!(malformed.receipt.code, "caduceus-access-malformed-response");
    let _ = malformed_fixture.finish();
}

#[test]
fn successor_access_maps_safe_refusals_without_reflecting_secret_response_text() {
    let fixture = AccessFixture::start(vec![AccessFixtureReply::json(serde_json::json!({
        "ok": false,
        "firstMissingSignal": "fixture-pin-981 fixture-session-ticket fixture-capability-token",
    }))]);
    let call = CaduceusAccessClient::new(fixture.base.clone()).session_mint("fixture-pin-981");
    let rendered = format!("{:?}", call);
    assert!(!rendered.contains("fixture-pin-981"));
    assert!(!rendered.contains("fixture-session-ticket"));
    assert!(!rendered.contains("fixture-capability-token"));
    assert_eq!(call.receipt.code, "caduceus-access-refused");
    let _ = fixture.finish();
}

#[test]
fn opaque_cookie_helpers_are_exact_and_never_render_ticket() {
    let ticket = SessionTicket::parse("opaque-ticket_123").unwrap();
    let set = session_cookie(&ticket);
    assert_eq!(set, "caduceus_session=opaque-ticket_123; HttpOnly; Secure; SameSite=Strict; Path=/; Max-Age=1800");
    assert_eq!(clear_session_cookie(), "caduceus_session=; HttpOnly; Secure; SameSite=Strict; Path=/; Max-Age=0");
    let mut headers = axum::http::HeaderMap::new();
    headers.insert(header::COOKIE, HeaderValue::from_static("other=value; caduceus_session=opaque-ticket_123"));
    let parsed = session_ticket_from_cookie(&headers).unwrap();
    assert_eq!(parsed.expose_for_transport(), "opaque-ticket_123");
    assert_eq!(format!("{:?}", parsed), "SessionTicket([redacted])");
    headers.insert(header::COOKIE, HeaderValue::from_static("caduceus_session=bad value"));
    assert!(session_ticket_from_cookie(&headers).is_none());
}

#[test]
fn same_origin_guard_accepts_home_https_and_refuses_absent_or_mismatched_context_without_body_access() {
    let mut direct = axum::http::HeaderMap::new();
    direct.insert(header::HOST, HeaderValue::from_static("home.arpa"));
    direct.insert(header::ORIGIN, HeaderValue::from_static("https://home.arpa"));
    assert!(same_origin_state_change(&direct));

    let mut proxied = axum::http::HeaderMap::new();
    proxied.insert(header::HOST, HeaderValue::from_static("127.0.0.1:8090"));
    proxied.insert("x-forwarded-host", HeaderValue::from_static("home.arpa, proxy.home.arpa"));
    proxied.insert("x-forwarded-proto", HeaderValue::from_static("https"));
    proxied.insert(header::ORIGIN, HeaderValue::from_static("https://home.arpa"));
    assert!(same_origin_state_change(&proxied));

    for (host, origin, proto) in [
        ("home.arpa", None, None),
        ("evil.example", Some("https://evil.example"), None),
        ("home.arpa", Some("http://home.arpa"), None),
        ("home.arpa", Some("https://home.arpa"), Some("http")),
    ] {
        let mut headers = axum::http::HeaderMap::new();
        headers.insert(header::HOST, HeaderValue::from_str(host).unwrap());
        if let Some(origin) = origin { headers.insert(header::ORIGIN, HeaderValue::from_static(origin)); }
        if let Some(proto) = proto { headers.insert("x-forwarded-proto", HeaderValue::from_static(proto)); }
        assert!(!same_origin_state_change(&headers));
    }
}

#[test]
fn successor_access_base_is_loopback_only_and_test_injectable() {
    assert_eq!(CaduceusAccessClient::default().base, test_fixture::base());
    assert!(access_authority("http://127.0.0.1:3014").is_some());
    assert!(access_authority("http://[::1]:3014").is_some());
    assert!(access_authority("http://192.0.2.1:3014").is_none());
    assert!(access_authority("https://127.0.0.1:3014").is_none());
}
