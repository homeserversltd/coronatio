fn successor_fixture_request(path: &str, with_cookie: bool) -> Request<Body> {
    let mut request = Request::post(path).body(Body::empty()).expect("fixture request");
    *request.headers_mut() = crate::caduceus_access::test_fixture::same_origin_headers(with_cookie);
    request
}

#[tokio::test]
async fn successor_fixture_mounts_session_prove_and_refuses_missing_cookie_or_origin() {
    let client = CaduceusAccessClient::default();
    let minted = client.session_mint("fixture-only-input");
    let minted_ticket = minted.take_ticket().expect("fixture session ticket");
    assert!(client.session_clear(&minted_ticket).receipt.ok);
    let state = AppState { tab_root: Arc::new(test_tab_root("caduceus-successor-prove")) };
    let proved = app(state.clone())
        .oneshot(successor_fixture_request("/api/session/prove", true))
        .await
        .expect("mounted prove response");
    assert_eq!(proved.status(), StatusCode::OK);

    let missing_cookie = app(state.clone())
        .oneshot(successor_fixture_request("/api/session/prove", false))
        .await
        .expect("missing cookie response");
    assert_eq!(missing_cookie.status(), StatusCode::UNAUTHORIZED);

    let missing_origin = caduceus_session_mint_route(
        axum::http::HeaderMap::new(),
        axum::body::Bytes::from_static(br#"{"pin":"test-only"}"#),
    )
    .await;
    assert_eq!(missing_origin.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn successor_fixture_records_exact_scope_and_one_use_downstream_capability() {
    let mark = crate::caduceus_access::test_fixture::mark();
    let state = AppState { tab_root: Arc::new(test_tab_root("caduceus-successor-mutation")) };
    let response = app(state)
        .oneshot(successor_fixture_request("/api/caduceus/update/now", true))
        .await
        .expect("mounted mutation response");
    assert_eq!(response.status(), StatusCode::OK);

    let records = crate::caduceus_access::test_fixture::records_since(mark);
    assert!(records.iter().any(|record| {
        record.path == "/api/v1/access/capabilities/mint"
            && record.action.as_deref() == Some("update now")
            && record.target.as_deref() == Some("local")
    }), "records: {records:?}");
    assert!(records.iter().any(|record| {
        record.path == "/api/v1/update/now" && record.capability_present
    }), "records: {records:?}");
    let rendered = format!("{records:?}");
    assert!(!rendered.contains(crate::caduceus_access::test_fixture::opaque_ticket()));
    assert!(!rendered.contains("caduceus-test-capability"));
}

#[test]
fn successor_fixture_is_process_lifetime_and_safe_for_concurrent_clients_without_environment() {
    let barrier = Arc::new(std::sync::Barrier::new(3));
    let mut workers = Vec::new();
    for _ in 0..2 {
        let barrier = Arc::clone(&barrier);
        workers.push(std::thread::spawn(move || {
            barrier.wait();
            let client = CaduceusAccessClient::default();
            assert_eq!(client.test_base(), crate::caduceus_access::test_fixture::base());
            let ticket = SessionTicket::parse(crate::caduceus_access::test_fixture::opaque_ticket()).expect("fixture ticket");
            assert!(client.session_prove(&ticket).receipt.ok);
            assert!(client.capability_mint(&ticket, "concurrent-test", "fixture").receipt.ok);
        }));
    }
    barrier.wait();
    for worker in workers {
        worker.join().expect("concurrent fixture worker");
    }
}

#[test]
fn successor_fixture_keeps_explicit_client_bases_and_request_helpers_narrow() {
    let explicit = CaduceusAccessClient::new("http://127.0.0.1:9");
    assert_eq!(explicit.test_base(), "http://127.0.0.1:9");
    let headers = crate::caduceus_access::test_fixture::same_origin_headers(true);
    assert_eq!(headers.len(), 4);
    assert!(headers.contains_key(header::HOST));
    assert!(headers.contains_key("x-forwarded-proto"));
    assert!(headers.contains_key(header::ORIGIN));
    assert_eq!(headers.get(header::COOKIE).and_then(|value| value.to_str().ok()), Some("caduceus_session=caduceus-test-session-ticket"));
    for forbidden in ["x-admin-token", "authorization", "x-caduceus-capability"] {
        assert!(!headers.contains_key(forbidden));
    }
}

#[tokio::test]
async fn session_routes_order_origin_before_body_and_clear_browser_authority_on_guest_paths() {
    let state = AppState { tab_root: Arc::new(test_tab_root("session-state-transition-walls")) };
    let mark = crate::caduceus_access::test_fixture::mark();
    for body in [Body::from("not-json"), Body::from(vec![b'x'; CADUCEUS_SESSION_BODY_MAX + 1])] {
        let response = app(state.clone())
            .oneshot(Request::builder().method("POST").uri("/api/session/mint").header(header::CONTENT_TYPE, "application/json").body(body).unwrap())
            .await.unwrap();
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
        assert!(response.headers().get(header::SET_COOKIE).is_none());
        let rendered = String::from_utf8(axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        assert!(rendered.contains("caduceus-access-origin-refused"));
        assert!(!rendered.contains("fixture-only-input"));
    }
    assert!(crate::caduceus_access::test_fixture::records_since(mark).is_empty(), "origin refusal contacted Caduceus");

    let mut cross_origin_clear = Request::builder().method("POST").uri("/api/session/clear").body(Body::empty()).unwrap();
    cross_origin_clear.headers_mut().insert(header::COOKIE, HeaderValue::from_static("caduceus_session=caduceus-test-session-ticket"));
    let refusal = app(state.clone()).oneshot(cross_origin_clear).await.unwrap();
    assert_eq!(refusal.status(), StatusCode::FORBIDDEN);
    assert!(refusal.headers().get(header::SET_COOKIE).is_none());
    assert!(crate::caduceus_access::test_fixture::records_since(mark).is_empty(), "cross-origin clear contacted Caduceus");

    let missing = app(state.clone()).oneshot(successor_session_request(Request::builder().method("POST").uri("/api/session/prove").body(Body::empty()).unwrap(), false)).await.unwrap();
    assert_eq!(missing.status(), StatusCode::UNAUTHORIZED);
    assert_eq!(missing.headers().get(header::SET_COOKIE).and_then(|value| value.to_str().ok()), Some("caduceus_session=; HttpOnly; Secure; SameSite=Strict; Path=/; Max-Age=0"));
    assert!(crate::caduceus_access::test_fixture::records_since(mark).is_empty(), "missing or malformed cookie contacted Caduceus");

    let mut expired = successor_fixture_request("/api/session/prove", false);
    expired.headers_mut().insert(header::COOKIE, HeaderValue::from_static("caduceus_session=caduceus-test-expired-ticket"));
    let expired = app(state.clone()).oneshot(expired).await.unwrap();
    assert_eq!(expired.status(), StatusCode::UNAUTHORIZED);
    assert_eq!(expired.headers().get(header::SET_COOKIE).and_then(|value| value.to_str().ok()), Some("caduceus_session=; HttpOnly; Secure; SameSite=Strict; Path=/; Max-Age=0"));
    let expired_body = String::from_utf8(axum::body::to_bytes(expired.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
    assert!(expired_body.contains("caduceus-access-refused"));
    assert!(!expired_body.contains("caduceus-test-expired-ticket"));
    let records = crate::caduceus_access::test_fixture::records_since(mark);
    assert_eq!(records.iter().filter(|record| record.path == "/api/v1/access/sessions/prove").count(), 1, "{records:?}");
    assert_eq!(records.iter().filter(|record| record.path == "/api/v1/access/sessions/clear").count(), 1, "{records:?}");

    let mut clear_fault = successor_fixture_request("/api/session/clear", false);
    clear_fault.headers_mut().insert(header::COOKIE, HeaderValue::from_static("caduceus_session=caduceus-test-clear-refused-ticket"));
    let clear_fault = app(state.clone()).oneshot(clear_fault).await.unwrap();
    assert_eq!(clear_fault.status(), StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(clear_fault.headers().get(header::SET_COOKIE).and_then(|value| value.to_str().ok()), Some("caduceus_session=; HttpOnly; Secure; SameSite=Strict; Path=/; Max-Age=0"));

    let proved = app(state).oneshot(successor_fixture_request("/api/session/prove", true)).await.unwrap();
    assert_eq!(proved.status(), StatusCode::OK);
    assert!(proved.headers().get(header::SET_COOKIE).is_none());
}

#[tokio::test]
async fn direct_session_route_walls_refuse_before_secret_parse_or_clear() {
    let mark = crate::caduceus_access::test_fixture::mark();
    let mint = caduceus_session_mint_route(
        axum::http::HeaderMap::new(),
        axum::body::Bytes::from(vec![b'x'; CADUCEUS_SESSION_BODY_MAX + 1]),
    ).await;
    assert_eq!(mint.status(), StatusCode::FORBIDDEN);
    assert!(mint.headers().get(header::SET_COOKIE).is_none());

    let mut headers = axum::http::HeaderMap::new();
    headers.insert(header::COOKIE, HeaderValue::from_static("caduceus_session=caduceus-test-session-ticket"));
    let clear = caduceus_session_clear_route(headers).await;
    assert_eq!(clear.status(), StatusCode::FORBIDDEN);
    assert!(clear.headers().get(header::SET_COOKIE).is_none());
    assert!(crate::caduceus_access::test_fixture::records_since(mark).is_empty());

    let mut unavailable_headers = crate::caduceus_access::test_fixture::same_origin_headers(false);
    unavailable_headers.insert(header::COOKIE, HeaderValue::from_static("caduceus_session=caduceus-test-unavailable-ticket"));
    let unavailable = caduceus_session_prove_with_client(
        unavailable_headers,
        crate::caduceus_access::CaduceusAccessClient::new("http://127.0.0.1:9"),
    ).await;
    assert_eq!(unavailable.status(), StatusCode::UNAUTHORIZED);
    assert_eq!(unavailable.headers().get(header::SET_COOKIE).and_then(|value| value.to_str().ok()), Some("caduceus_session=; HttpOnly; Secure; SameSite=Strict; Path=/; Max-Age=0"));
    let unavailable_body = String::from_utf8(axum::body::to_bytes(unavailable.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
    assert!(unavailable_body.contains("caduceus-access-unavailable"));
    assert!(!unavailable_body.contains("caduceus-test-unavailable-ticket"));
}
