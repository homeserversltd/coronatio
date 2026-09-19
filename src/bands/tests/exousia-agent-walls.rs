#[cfg(test)]
mod exousia_agent_tests {
    use super::*;
    use std::io::{BufRead, BufReader, Read, Write};
    use std::os::unix::net::{UnixListener, UnixStream};
    use std::sync::{Arc, Mutex};

    fn fixture_request() -> &'static [u8] {
        br#"{"schema":"caduceus.staff.v1","intent_id":"fixture-agent","transition":"exousia.open","version":{"future":true},"timestamp":"fixture","target":{"document":"/api/v1/appliance/service/{service}/restart","service":"fixture.service","action":"restart"},"flags":{"exousia":{"pin":"fixture-pin"}},"payload":{"unknownAdditive":{"survives":true}}}"#
    }

    fn read_request(stream: UnixStream) -> (String, Vec<u8>, UnixStream) {
        let mut reader = BufReader::new(stream);
        let mut head = String::new();
        loop {
            let mut line = String::new();
            reader.read_line(&mut line).unwrap();
            head.push_str(&line);
            if line == "\r\n" { break; }
        }
        let length = head.lines().find_map(|line| {
            line.split_once(':').and_then(|(name, value)| {
                name.eq_ignore_ascii_case("content-length")
                    .then(|| value.trim().parse::<usize>().ok())
                    .flatten()
            })
        }).unwrap_or(0);
        let mut body = vec![0; length];
        reader.read_exact(&mut body).unwrap();
        (head, body, reader.into_inner())
    }

    fn reply(mut stream: UnixStream, body: &str) {
        write!(stream, "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}", body.len(), body).unwrap();
    }

    fn headers() -> axum::http::HeaderMap {
        let mut headers = axum::http::HeaderMap::new();
        headers.insert(axum::http::header::CONTENT_TYPE, axum::http::HeaderValue::from_static("application/json"));
        headers
    }

    #[tokio::test]
    async fn agent_crossing_preserves_raw_unknown_fields_and_invalidates_scoped_attendance() {
        let _guard = crate::CADUCEUS_ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
        let _config_guard = HX_EXEMPLAR_ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
        let config_path = std::env::temp_dir().join(format!("coronatio-agent-{}.json", uuid::Uuid::new_v4()));
        std::fs::write(
            &config_path,
            r#"{"global":{"cors":{"allowed_origins":["https://home.arpa"]}},"tabs":{"portals":{"data":{"portals":[{"name":"Fixture","services":["fixture"],"localURL":"https://fixture.home.arpa"}]}}}}"#,
        )
        .unwrap();
        std::env::set_var("CORONATIO_HOMESERVER_JSON", &config_path);
        let socket = std::env::temp_dir().join(format!("coronatio-agent-{}-{}.sock", std::process::id(), uuid::Uuid::new_v4()));
        let listener = UnixListener::bind(&socket).unwrap();
        let captured = Arc::new(Mutex::new(Vec::<(String, Vec<u8>)>::new()));
        let captured_thread = Arc::clone(&captured);
        let worker = std::thread::spawn(move || {
            let (stream, _) = listener.accept().unwrap();
            let (head, body, stream) = read_request(stream);
            captured_thread.lock().unwrap().push((head, body));
            reply(stream, r#"{"ok":true,"attendance":"fixture-attendance"}"#);

            let (stream, _) = listener.accept().unwrap();
            let (head, body, stream) = read_request(stream);
            captured_thread.lock().unwrap().push((head, body));
            reply(stream, r#"{"ok":true,"success":true,"active":true,"firstMissingSignal":"none","attendance":"must-not-return"}"#);

            let (stream, _) = listener.accept().unwrap();
            let (head, body, stream) = read_request(stream);
            captured_thread.lock().unwrap().push((head, body));
            reply(stream, r#"{"ok":true,"firstMissingSignal":"none"}"#);
        });
        std::env::set_var("CADUCEUS_STAFF_SOCKET", &socket);
        let response = caduceus_agent_service_route(headers(), axum::body::Bytes::from(fixture_request().to_vec())).await;
        worker.join().unwrap();
        std::env::remove_var("CADUCEUS_STAFF_SOCKET");
        std::env::remove_var("CORONATIO_HOMESERVER_JSON");
        let _ = std::fs::remove_file(&socket);
        let _ = std::fs::remove_file(&config_path);

        let captured = captured.lock().unwrap();
        assert_eq!(captured.len(), 3);
        assert!(captured[0].0.starts_with("POST /api/v1/exousia/open HTTP/1.1"));
        assert_eq!(captured[0].1, fixture_request());
        assert!(captured[0].1.windows(b"unknownAdditive".len()).any(|window| window == b"unknownAdditive"));
        assert!(captured[1].0.starts_with("POST /api/v1/appliance/service/fixture.service/restart HTTP/1.1"));
        assert!(captured[1].0.contains("x-caduceus-document: /api/v1/appliance/service/{service}/restart"));
        assert!(captured[1].0.contains("x-caduceus-attendance: fixture-attendance"));
        assert_eq!(captured[1].1, br#"{}"#);
        assert!(captured[2].0.starts_with("POST /api/v1/exousia/invalidate HTTP/1.1"));

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let value: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let text = String::from_utf8(body.to_vec()).unwrap();
        assert_eq!(value["schema"], "coronatio.exousia.agent.service.v1");
        assert_eq!(value["success"], true);
        assert_eq!(value["service"], "fixture.service");
        assert_eq!(value["action"], "restart");
        assert!(!text.contains("fixture-pin"));
        assert!(!text.contains("fixture-attendance"));
        assert!(!text.contains("rawEnvelope"));
    }

    #[tokio::test]
    async fn agent_allowlist_refuses_valid_envelope_before_uds_and_redacts_secrets() {
        let _guard = crate::CADUCEUS_ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
        let _config_guard = HX_EXEMPLAR_ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
        let config_path = std::env::temp_dir().join(format!("coronatio-agent-deny-{}.json", uuid::Uuid::new_v4()));
        std::fs::write(
            &config_path,
            r#"{"global":{"cors":{"allowed_origins":["https://home.arpa"]}},"tabs":{"portals":{"data":{"portals":[{"name":"Other","services":["other"],"localURL":"https://other.home.arpa"}]}}}}"#,
        )
        .unwrap();
        let socket = std::env::temp_dir().join(format!("coronatio-agent-deny-{}-{}.sock", std::process::id(), uuid::Uuid::new_v4()));
        let _ = std::fs::remove_file(&socket);
        std::env::set_var("CORONATIO_HOMESERVER_JSON", &config_path);
        std::env::set_var("CADUCEUS_STAFF_SOCKET", &socket);

        let response = caduceus_agent_service_route(headers(), axum::body::Bytes::from(fixture_request().to_vec())).await;

        std::env::remove_var("CADUCEUS_STAFF_SOCKET");
        std::env::remove_var("CORONATIO_HOMESERVER_JSON");
        let _ = std::fs::remove_file(&config_path);
        let _ = std::fs::remove_file(&socket);

        assert_eq!(response.status(), StatusCode::FORBIDDEN);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let value: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let text = String::from_utf8(body.to_vec()).unwrap();
        assert_eq!(value["firstMissingSignal"], "portal-service-not-allowlisted");
        assert_eq!(value["service"], "fixture.service");
        assert_eq!(value["action"], "restart");
        assert!(!socket.exists(), "non-allowlisted service must be refused before any UDS call");
        assert!(!text.contains("fixture-pin"));
        assert!(!text.contains("fixture-attendance"));
        assert!(!text.contains("rawEnvelope"));
    }

    #[tokio::test]
    async fn agent_validation_refuses_before_uds_and_posture_is_redacted() {
        let _guard = crate::CADUCEUS_ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
        let socket = std::env::temp_dir().join(format!("coronatio-agent-refuse-{}-{}.sock", std::process::id(), uuid::Uuid::new_v4()));
        let _ = std::fs::remove_file(&socket);
        std::env::set_var("CADUCEUS_STAFF_SOCKET", &socket);
        for raw in [
            br#"{"schema":"foreign.v1","intent_id":"i","transition":"exousia.open","version":1,"timestamp":1,"target":{},"flags":{"exousia":{"pin":"fixture-pin"}}}"#.as_slice(),
            br#"{"schema":"caduceus.staff.v1","intent_id":"i","transition":"exousia.open","version":1,"target":{},"flags":{"exousia":{"pin":"fixture-pin"}}}"#.as_slice(),
            br#"{"schema":"caduceus.staff.v1","intent_id":"i","transition":"wrong","version":1,"timestamp":1,"target":{},"flags":{"exousia":{"pin":"fixture-pin"}}}"#.as_slice(),
            br#"{"schema":"caduceus.staff.v1","intent_id":"i","transition":"exousia.open","version":1,"timestamp":1,"target":{"document":"/api/v1/appliance/service/{service}/format","service":"fixture.service","action":"format"},"flags":{"exousia":{"pin":"fixture-pin"}}}"#.as_slice(),
            br#"{"schema":"caduceus.staff.v1","intent_id":"i","transition":"exousia.open","version":1,"timestamp":1,"target":{"document":"/api/v1/appliance/service/{service}/restart","service":"../fixture","action":"restart"},"flags":{"exousia":{"pin":"fixture-pin"}}}"#.as_slice(),
            br#"{"schema":"caduceus.staff.v1","intent_id":"i","transition":"exousia.open","version":1,"timestamp":1,"target":{"document":"wrong","service":"fixture.service","action":"restart"},"flags":{"exousia":{"pin":"fixture-pin"}}}"#.as_slice(),
            br#"{"schema":"caduceus.staff.v1","intent_id":"i","transition":"exousia.open","version":1,"timestamp":1,"target":{"document":"/api/v1/appliance/service/{service}/restart","service":"fixture.service","action":"restart"},"flags":{"exousia":{}}}"#.as_slice(),
        ] {
            let response = caduceus_agent_service_route(headers(), axum::body::Bytes::from(raw.to_vec())).await;
            assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        }
        let posture_socket = UnixListener::bind(&socket).unwrap();
        let worker = std::thread::spawn(move || {
            let (stream, _) = posture_socket.accept().unwrap();
            let (_, _, stream) = read_request(stream);
            reply(stream, r#"{"ok":true,"posture":"DERIVED_BOUND","bound":true,"storedVerifierPresent":true,"currentPresent":false,"epochMatches":true,"active":false,"firstMissingSignal":"none","publicKey":"fixture-public","epoch":"1","pin":"fixture-pin","attendance":"fixture-attendance","flags":{"raw":true},"rawEnvelope":{"secret":"drop-me-too"},"unknown":"drop-me"}"#);
        });
        let response = caduceus_agent_posture_route().await;
        worker.join().unwrap();
        std::env::remove_var("CADUCEUS_STAFF_SOCKET");
        let _ = std::fs::remove_file(&socket);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let value: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(value.as_object().unwrap().len(), 8);
        assert_eq!(value["posture"], "DERIVED_BOUND");
        assert_eq!(value["bound"], true);
        assert_eq!(value["storedVerifierPresent"], true);
        assert_eq!(value["currentPresent"], false);
        assert_eq!(value["epochMatches"], true);
        assert!(!value["active"].is_boolean());
        let text = String::from_utf8(body.to_vec()).unwrap();
        assert!(!text.contains("fixture-public"));
        assert!(!text.contains("\"epoch\""));
        assert!(!text.contains("fixture-pin"));
        assert!(!text.contains("fixture-attendance"));
        assert!(!text.contains("flags"));
        assert!(!text.contains("rawEnvelope"));
        assert!(!text.contains("drop-me"));
    }

    #[tokio::test]
    async fn browser_portal_service_control_validates_once_and_forwards_browser_attendance() {
        let _caduceus_guard = crate::CADUCEUS_ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
        let _guard = HX_EXEMPLAR_ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
        let config_path = std::env::temp_dir().join(format!("coronatio-browser-portal-{}.json", uuid::Uuid::new_v4()));
        std::fs::write(
            &config_path,
            r#"{"global":{"cors":{"allowed_origins":["https://home.arpa"]}},"tabs":{"portals":{"data":{"portals":[{"name":"Fixture","services":["fixture"],"localURL":"https://fixture.home.arpa"}]}}}}"#,
        )
        .unwrap();
        std::env::set_var("CORONATIO_HOMESERVER_JSON", &config_path);

        let socket = std::env::temp_dir().join(format!("coronatio-browser-portal-{}-{}.sock", std::process::id(), uuid::Uuid::new_v4()));
        let listener = UnixListener::bind(&socket).unwrap();
        let captured = Arc::new(Mutex::new(Vec::<(String, Vec<u8>)>::new()));
        let captured_thread = Arc::clone(&captured);
        let worker = std::thread::spawn(move || {
            for _ in 0..2 {
                let (stream, _) = listener.accept().unwrap();
                let (head, body, stream) = read_request(stream);
                let validate = head.starts_with("POST /api/v1/exousia/validate HTTP/1.1");
                captured_thread.lock().unwrap().push((head, body));
                if validate {
                    reply(stream, r#"{"ok":true,"firstMissingSignal":"none"}"#);
                } else {
                    reply(stream, r#"{"ok":true,"success":true,"active":true,"firstMissingSignal":"none"}"#);
                }
            }
        });
        std::env::set_var("CADUCEUS_STAFF_SOCKET", &socket);

        let document = uuid::Uuid::new_v4().to_string();
        let attendance = "browser-generic-attendance";
        let mut request = Request::builder()
            .method("POST")
            .uri("/api/service/control")
            .header("content-type", "application/json")
            .body(Body::from(r#"{"service":"fixture","action":"restart"}"#))
            .unwrap();
        request.headers_mut().insert("host", "home.arpa".parse().unwrap());
        request.headers_mut().insert("origin", "https://home.arpa".parse().unwrap());
        request.headers_mut().insert("x-caduceus-document", document.parse().unwrap());
        request.headers_mut().insert("x-caduceus-attendance", attendance.parse().unwrap());
        let response = app(AppState { tab_root: Arc::new(test_tab_root("browser-portal-service-control")) })
            .oneshot(request)
            .await
            .unwrap();

        worker.join().unwrap();
        std::env::remove_var("CADUCEUS_STAFF_SOCKET");
        std::env::remove_var("CORONATIO_HOMESERVER_JSON");
        let _ = std::fs::remove_file(&socket);
        let _ = std::fs::remove_file(config_path);

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let value: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(value["schema"], "coronatio.portals.service_control.v1");
        assert_eq!(value["success"], true);

        let captured = captured.lock().unwrap();
        assert_eq!(captured.len(), 2);
        assert!(captured[0].0.starts_with("POST /api/v1/exousia/validate HTTP/1.1"));
        let validation: serde_json::Value = serde_json::from_slice(&captured[0].1).unwrap();
        assert_eq!(validation["attendance"], attendance);
        assert_eq!(validation["documentId"], document);
        assert_eq!(validation["documentIncarnation"], document);
        assert!(captured[1].0.starts_with("POST /api/v1/appliance/service/fixture/restart HTTP/1.1"));
        assert!(captured[1].0.contains(&format!("x-caduceus-document: {document}")));
        assert!(captured[1].0.contains(&format!("x-caduceus-attendance: {attendance}")));
        assert_eq!(captured[1].1, br#"{}"#);
        assert!(!captured.iter().any(|(head, _)| head.contains("/api/v1/exousia/open")));
    }

    #[test]
    fn canonical_agent_document_keeps_literal_service_token() {
        assert_eq!(agent_service_document("start"), "/api/v1/appliance/service/{service}/start");
        for (signal, expected) in [
            ("caduceus-attendance-wrong-document", StatusCode::BAD_REQUEST),
            ("caduceus-attendance-pin-wrong", StatusCode::UNAUTHORIZED),
            ("caduceus-signer-stale-derived", StatusCode::SERVICE_UNAVAILABLE),
            ("caduceus-signer-current-bind-unavailable", StatusCode::SERVICE_UNAVAILABLE),
            ("caduceus-signer-verification-unavailable", StatusCode::SERVICE_UNAVAILABLE),
        ] {
            let readback = CaduceusHttpReadback { ok: false, status: 0, path: "/test".to_string(), body: serde_json::json!({}), first_missing_signal: signal.to_string() };
            assert_eq!(mutation_response_status(&readback), expected, "{signal}");
            let call = crate::caduceus_access::AttendanceCall {
                receipt: crate::caduceus_access::AttendanceReceipt { operation: "attendance.validate", ok: false, status: 0, code: signal.to_string() },
                proof: None,
            };
            assert_eq!(attendance_failure_status(&call), expected, "{signal}");
        }
    }
}
