#[cfg(test)]
mod exousia_agent_tests {
    use super::*;
    use std::io::{BufRead, BufReader, Read, Write};
    use std::os::unix::fs::PermissionsExt;
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

    fn agent_peer() -> axum::extract::ConnectInfo<std::net::SocketAddr> {
        axum::extract::ConnectInfo(std::net::SocketAddr::from(([192, 0, 2, 200], 3013)))
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
        let response = caduceus_agent_service_route(agent_peer(), headers(), axum::body::Bytes::from(fixture_request().to_vec())).await;
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

        let response = caduceus_agent_service_route(agent_peer(), headers(), axum::body::Bytes::from(fixture_request().to_vec())).await;

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
            let response = caduceus_agent_service_route(agent_peer(), headers(), axum::body::Bytes::from(raw.to_vec())).await;
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
            for _ in 0..4 {
                let (stream, _) = listener.accept().unwrap();
                let (head, body, stream) = read_request(stream);
                let validate = head.starts_with("POST /api/v1/exousia/validate HTTP/1.1");
                let open = head.starts_with("POST /api/v1/exousia/open HTTP/1.1");
                let action = head.starts_with("POST /api/v1/appliance/service/fixture/restart HTTP/1.1");
                captured_thread.lock().unwrap().push((head, body));
                if validate {
                    reply(stream, r#"{"ok":true,"firstMissingSignal":"none"}"#);
                } else if open {
                    reply(stream, r#"{"ok":true,"attendance":"scoped-browser-attendance","firstMissingSignal":"none"}"#);
                } else if action {
                    reply(stream, r#"{"ok":true,"success":true,"active":true,"firstMissingSignal":"none"}"#);
                } else {
                    reply(stream, r#"{"ok":true,"firstMissingSignal":"none"}"#);
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
        assert_eq!(captured.len(), 4);
        assert!(captured[0].0.starts_with("POST /api/v1/exousia/validate HTTP/1.1"));
        let validation: serde_json::Value = serde_json::from_slice(&captured[0].1).unwrap();
        assert_eq!(validation["attendance"], attendance);
        assert_eq!(validation["documentId"], document);
        assert_eq!(validation["documentIncarnation"], document);
        assert!(captured[1].0.starts_with("POST /api/v1/exousia/open HTTP/1.1"));
        let scoped_open: serde_json::Value = serde_json::from_slice(&captured[1].1).unwrap();
        assert_eq!(scoped_open["target"]["document"], "/api/v1/appliance/service/{service}/restart");
        assert_eq!(scoped_open["flags"]["exousia"]["attendance"], attendance);
        assert_eq!(scoped_open["flags"]["exousia"]["documentId"], document);
        assert_eq!(scoped_open["flags"]["exousia"]["documentIncarnation"], document);
        assert!(scoped_open["flags"]["exousia"].get("pin").is_none());
        assert!(captured[2].0.starts_with("POST /api/v1/appliance/service/fixture/restart HTTP/1.1"));
        assert!(captured[2].0.contains("x-caduceus-document: /api/v1/appliance/service/{service}/restart"));
        assert!(captured[2].0.contains("x-caduceus-attendance: scoped-browser-attendance"));
        assert!(!captured[2].0.contains(&format!("x-caduceus-attendance: {attendance}")));
        assert_eq!(captured[2].1, br#"{}"#);
        assert!(captured[3].0.starts_with("POST /api/v1/exousia/invalidate HTTP/1.1"));
        let invalidation: serde_json::Value = serde_json::from_slice(&captured[3].1).unwrap();
        assert_eq!(invalidation["attendance"], "scoped-browser-attendance");
        assert_eq!(invalidation["documentId"], "/api/v1/appliance/service/{service}/restart");
        assert!(!captured.iter().skip(2).any(|(_, body)| body.windows(attendance.len()).any(|window| window == attendance.as_bytes())));
    }

    #[tokio::test]
    async fn agent_peer_rate_limit_blocks_sixth_before_uds_and_keeps_other_peer_eligible() {
        let _guard = crate::CADUCEUS_ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
        let _config_guard = HX_EXEMPLAR_ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
        let config_path = std::env::temp_dir().join(format!("coronatio-agent-rate-{}.json", uuid::Uuid::new_v4()));
        std::fs::write(
            &config_path,
            r#"{"global":{"cors":{"allowed_origins":["https://home.arpa"]}},"tabs":{"portals":{"data":{"portals":[{"name":"Fixture","services":["fixture"],"localURL":"https://fixture.home.arpa"}]}}}}"#,
        )
        .unwrap();
        let socket = std::env::temp_dir().join(format!("coronatio-agent-rate-{}-{}.sock", std::process::id(), uuid::Uuid::new_v4()));
        let listener = UnixListener::bind(&socket).unwrap();
        let worker = std::thread::spawn(move || {
            for _ in 0..18 {
                let (stream, _) = listener.accept().unwrap();
                let (head, _, stream) = read_request(stream);
                let response = if head.starts_with("POST /api/v1/exousia/open HTTP/1.1") {
                    r#"{"ok":true,"attendance":"rate-scoped-attendance","firstMissingSignal":"none"}"#
                } else {
                    r#"{"ok":true,"success":true,"active":true,"firstMissingSignal":"none"}"#
                };
                reply(stream, response);
            }
        });
        std::env::set_var("CORONATIO_HOMESERVER_JSON", &config_path);
        std::env::set_var("CADUCEUS_STAFF_SOCKET", &socket);
        let peer = axum::extract::ConnectInfo(std::net::SocketAddr::from(([192, 0, 2, 44], 3013)));
        for _ in 0..5 {
            let response = caduceus_agent_service_route(peer, headers(), axum::body::Bytes::from(fixture_request().to_vec())).await;
            assert_ne!(response.status(), StatusCode::TOO_MANY_REQUESTS);
        }
        let blocked = caduceus_agent_service_route(peer, headers(), axum::body::Bytes::from(fixture_request().to_vec())).await;
        assert_eq!(blocked.status(), StatusCode::TOO_MANY_REQUESTS);
        let body = axum::body::to_bytes(blocked.into_body(), usize::MAX).await.unwrap();
        let value: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(value["firstMissingSignal"], "coronatio-exousia-agent-rate-limited");
        let other_peer = axum::extract::ConnectInfo(std::net::SocketAddr::from(([192, 0, 2, 45], 3013)));
        let eligible = caduceus_agent_service_route(other_peer, headers(), axum::body::Bytes::from(fixture_request().to_vec())).await;
        assert_eq!(eligible.status(), StatusCode::OK);
        worker.join().unwrap();
        std::env::remove_var("CADUCEUS_STAFF_SOCKET");
        std::env::remove_var("CORONATIO_HOMESERVER_JSON");
        let _ = std::fs::remove_file(&socket);
        let _ = std::fs::remove_file(&config_path);
    }

    #[test]
    fn agent_peer_limiter_allows_five_blocks_sixth_is_peer_local_and_expires_without_sleep() {
        let mut limiter = AgentServicePeerLimiter::default();
        let start = std::time::Instant::now();
        let first = "192.0.2.10".parse().unwrap();
        let second = "192.0.2.11".parse().unwrap();
        for _ in 0..5 {
            assert!(limiter.allow_at(first, start));
        }
        assert!(!limiter.allow_at(first, start));
        assert!(limiter.allow_at(second, start));
        assert!(limiter.allow_at(first, start + AGENT_SERVICE_RATE_LIMIT_WINDOW));
    }

    #[test]
    fn agent_lockout_persists_reload_caps_secrets_and_expires_per_peer() {
        let path = std::env::temp_dir().join(format!("coronatio-agent-lockout-{}.json", uuid::Uuid::new_v4()));
        let first = "198.51.100.10".parse().unwrap();
        let second = "198.51.100.11".parse().unwrap();
        let mut store = AgentServiceLockoutStore::default();
        store.reload(&path).unwrap();
        for expected in 1..=AGENT_SERVICE_LOCKOUT_FAILURES {
            let status = store.record_wrong_pin(first, 1_000 + u64::from(expected)).unwrap();
            assert_eq!(status.consecutive_failures, expected);
            assert_eq!(status.lockout_deadline_unix_seconds.is_some(), expected == AGENT_SERVICE_LOCKOUT_FAILURES);
        }
        let bytes = std::fs::read_to_string(&path).unwrap();
        assert!(!bytes.contains("fixture-pin"));
        assert!(!bytes.contains("fixture-attendance"));
        assert_eq!(std::fs::metadata(&path).unwrap().permissions().mode() & 0o777, 0o600);

        let mut reloaded = AgentServiceLockoutStore::default();
        reloaded.reload(&path).unwrap();
        let blocked = reloaded.blocked(first, 1_010).unwrap();
        assert_eq!(blocked.consecutive_failures, AGENT_SERVICE_LOCKOUT_FAILURES);
        assert!(reloaded.blocked(second, 1_010).is_none(), "another source peer remains eligible");
        let expired_at = blocked.lockout_deadline_unix_seconds.unwrap() + 1;
        assert!(reloaded.blocked(first, expired_at).is_none());
        reloaded.reset_peer(first).unwrap();
        let final_state: AgentServiceLockoutState = serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(final_state.schema, AGENT_SERVICE_LOCKOUT_STATE_SCHEMA);
        assert!(!final_state.peers.contains_key(&first), "a successful PIN open reset removes the expired peer state");
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn agent_lockout_reload_rejects_malformed_missing_and_foreign_schema() {
        for (label, contents) in [
            ("malformed", "{"),
            ("missing", r#"{"peers":{}}"#),
            ("foreign", r#"{"schema":"coronatio.exousia.agent.lockout.v0","peers":{}}"#),
        ] {
            let path = std::env::temp_dir().join(format!("coronatio-agent-lockout-{label}-{}.json", uuid::Uuid::new_v4()));
            std::fs::write(&path, contents).unwrap();
            let mut store = AgentServiceLockoutStore::default();
            assert!(store.reload(&path).is_err(), "{label} lockout state must fail closed");
            std::fs::remove_file(path).unwrap();
        }
    }

    #[test]
    fn agent_lockout_reload_rejects_directory_state_path() {
        let path = std::env::temp_dir().join(format!("coronatio-agent-lockout-directory-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir(&path).unwrap();
        let mut store = AgentServiceLockoutStore::default();
        assert!(store.reload(&path).is_err(), "a directory cannot be accepted as lockout state");
        std::fs::remove_dir(path).unwrap();
    }

    #[test]
    fn agent_lockout_persist_rejects_promotion_failure_and_cleans_temp() {
        let path = std::env::temp_dir().join(format!("coronatio-agent-lockout-promotion-{}.json", uuid::Uuid::new_v4()));
        let parent = path.parent().unwrap().to_path_buf();
        let name = path.file_name().unwrap().to_str().unwrap().to_string();
        let temp_prefix = format!(".{name}.tmp-");
        let mut store = AgentServiceLockoutStore::default();
        store.reload(&path).unwrap();
        std::fs::create_dir(&path).unwrap();

        assert!(store.persist().is_err(), "an existing directory cannot be atomically promoted over");
        let leftovers: Vec<_> = std::fs::read_dir(&parent)
            .unwrap()
            .filter_map(Result::ok)
            .map(|entry| entry.file_name())
            .filter(|entry| entry.to_str().is_some_and(|entry| entry.starts_with(&temp_prefix)))
            .collect();
        assert!(leftovers.is_empty(), "failed promotion must remove its temporary state file");
        std::fs::remove_dir(path).unwrap();
    }

    #[tokio::test]
    async fn agent_wrong_pin_lockout_blocks_before_uds_and_reflects_each_failure() {
        let _guard = crate::CADUCEUS_ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
        let _config_guard = HX_EXEMPLAR_ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
        let config_path = std::env::temp_dir().join(format!("coronatio-agent-lockout-config-{}.json", uuid::Uuid::new_v4()));
        let state_path = std::env::temp_dir().join(format!("coronatio-agent-lockout-state-{}.json", uuid::Uuid::new_v4()));
        std::fs::write(
            &config_path,
            r#"{"global":{"cors":{"allowed_origins":["https://home.arpa"]}},"tabs":{"portals":{"data":{"portals":[{"name":"Fixture","services":["fixture"],"localURL":"https://fixture.home.arpa"}]}}}}"#,
        )
        .unwrap();
        let socket = std::env::temp_dir().join(format!("coronatio-agent-lockout-{}-{}.sock", std::process::id(), uuid::Uuid::new_v4()));
        let listener = UnixListener::bind(&socket).unwrap();
        let captured = Arc::new(Mutex::new(Vec::<(String, Vec<u8>)>::new()));
        let captured_thread = Arc::clone(&captured);
        let worker = std::thread::spawn(move || {
            for _ in 0..11 {
                let (stream, _) = listener.accept().unwrap();
                let (head, body, stream) = read_request(stream);
                captured_thread.lock().unwrap().push((head.clone(), body));
                if head.starts_with("POST /api/v1/exousia/open HTTP/1.1") {
                    reply(stream, r#"{"ok":false,"firstMissingSignal":"caduceus-attendance-pin-wrong"}"#);
                } else {
                    assert!(head.starts_with("POST /api/v1/log/reflect HTTP/1.1"));
                    reply(stream, r#"{"ok":true,"firstMissingSignal":"none"}"#);
                }
            }
        });
        std::env::set_var("CORONATIO_HOMESERVER_JSON", &config_path);
        std::env::set_var("CORONATIO_EXOUSIA_AGENT_LOCKOUT_PATH", &state_path);
        std::env::set_var("CADUCEUS_STAFF_SOCKET", &socket);
        let peer = axum::extract::ConnectInfo(std::net::SocketAddr::from(([198, 51, 100, 30], 3013)));
        for _ in 0..5 {
            let response = caduceus_agent_service_route(peer, headers(), axum::body::Bytes::from(fixture_request().to_vec())).await;
            assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        }
        let blocked = caduceus_agent_service_route(peer, headers(), axum::body::Bytes::from(fixture_request().to_vec())).await;
        assert_eq!(blocked.status(), StatusCode::TOO_MANY_REQUESTS);
        let blocked_body = axum::body::to_bytes(blocked.into_body(), usize::MAX).await.unwrap();
        let blocked_value: serde_json::Value = serde_json::from_slice(&blocked_body).unwrap();
        assert_eq!(blocked_value["firstMissingSignal"], AGENT_SERVICE_LOCKED_OUT);
        worker.join().unwrap();

        let captured = captured.lock().unwrap();
        let open_count = captured.iter().filter(|(head, _)| head.starts_with("POST /api/v1/exousia/open HTTP/1.1")).count();
        let reflect_events: Vec<serde_json::Value> = captured
            .iter()
            .filter(|(head, _)| head.starts_with("POST /api/v1/log/reflect HTTP/1.1"))
            .map(|(_, body)| serde_json::from_slice(body).unwrap())
            .collect();
        assert_eq!(open_count, 5, "the sixth request is blocked before UDS/PIN work");
        assert_eq!(reflect_events.len(), 6, "five failures and one blocked lockout reached Hyalos");
        assert_eq!(reflect_events.iter().filter(|event| event["kind"] == AGENT_SERVICE_PIN_FAILURE_EVENT).count(), 5);
        assert_eq!(reflect_events.iter().filter(|event| event["kind"] == AGENT_SERVICE_LOCKOUT_EVENT).count(), 1);
        for event in &reflect_events {
            assert_eq!(event["organ"], "coronatio");
            assert!(event["attributes_redacted"]["source_ip"].is_string());
            assert!(event["attributes_redacted"]["consecutive_failures"].is_number());
            assert!(!serde_json::to_string(event).unwrap().contains("fixture-pin"));
            assert!(!serde_json::to_string(event).unwrap().contains("fixture-attendance"));
        }
        drop(captured);
        let state_bytes = std::fs::read_to_string(&state_path).unwrap();
        assert!(!state_bytes.contains("fixture-pin"));
        assert!(!state_bytes.contains("fixture-attendance"));
        let mut reloaded = AgentServiceLockoutStore::default();
        reloaded.reload(&state_path).unwrap();
        assert!(reloaded.blocked(peer.0.ip(), agent_service_now()).is_some());

        std::env::remove_var("CADUCEUS_STAFF_SOCKET");
        std::env::remove_var("CORONATIO_EXOUSIA_AGENT_LOCKOUT_PATH");
        std::env::remove_var("CORONATIO_HOMESERVER_JSON");
        let _ = std::fs::remove_file(&socket);
        let _ = std::fs::remove_file(&state_path);
        let _ = std::fs::remove_file(&config_path);
    }

    #[tokio::test]
    async fn agent_expired_lockout_successful_open_clears_peer_before_action() {
        let _guard = crate::CADUCEUS_ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
        let _config_guard = HX_EXEMPLAR_ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
        let config_path = std::env::temp_dir().join(format!("coronatio-agent-expired-config-{}.json", uuid::Uuid::new_v4()));
        let state_path = std::env::temp_dir().join(format!("coronatio-agent-expired-state-{}.json", uuid::Uuid::new_v4()));
        let socket = std::env::temp_dir().join(format!("coronatio-agent-expired-{}-{}.sock", std::process::id(), uuid::Uuid::new_v4()));
        std::fs::write(
            &config_path,
            r#"{"global":{"cors":{"allowed_origins":["https://home.arpa"]}},"tabs":{"portals":{"data":{"portals":[{"name":"Fixture","services":["fixture"],"localURL":"https://fixture.home.arpa"}]}}}}"#,
        )
        .unwrap();
        let peer = "198.51.100.31".parse().unwrap();
        let mut seeded = AgentServiceLockoutStore::default();
        seeded.reload(&state_path).unwrap();
        seeded.state.peers.insert(peer, AgentServiceLockoutPeer {
            consecutive_failures: AGENT_SERVICE_LOCKOUT_FAILURES,
            lockout_deadline_unix_seconds: Some(0),
            last_seen_unix_seconds: 1,
        });
        seeded.persist().unwrap();

        let listener = UnixListener::bind(&socket).unwrap();
        let worker = std::thread::spawn(move || {
            let expected_paths = [
                "POST /api/v1/exousia/open HTTP/1.1",
                "POST /api/v1/appliance/service/fixture.service/restart HTTP/1.1",
                "POST /api/v1/exousia/invalidate HTTP/1.1",
            ];
            let responses = [
                r#"{"ok":true,"attendance":"expired-attendance","firstMissingSignal":"none"}"#,
                r#"{"ok":true,"success":true,"active":true,"firstMissingSignal":"none"}"#,
                r#"{"ok":true,"firstMissingSignal":"none"}"#,
            ];
            let mut operational_requests = 0;
            while operational_requests < responses.len() {
                let (stream, _) = listener.accept().unwrap();
                let (head, _, stream) = read_request(stream);
                if head.starts_with("POST /api/v1/log/reflect HTTP/1.1") {
                    reply(stream, r#"{"ok":true,"firstMissingSignal":"none"}"#);
                    continue;
                }
                assert!(head.starts_with(expected_paths[operational_requests]));
                reply(stream, responses[operational_requests]);
                operational_requests += 1;
            }
        });
        std::env::set_var("CORONATIO_HOMESERVER_JSON", &config_path);
        std::env::set_var("CORONATIO_EXOUSIA_AGENT_LOCKOUT_PATH", &state_path);
        std::env::set_var("CADUCEUS_STAFF_SOCKET", &socket);
        let connect_info = axum::extract::ConnectInfo(std::net::SocketAddr::from(([198, 51, 100, 31], 3013)));
        let response = caduceus_agent_service_route(connect_info, headers(), axum::body::Bytes::from(fixture_request().to_vec())).await;
        assert_eq!(response.status(), StatusCode::OK);
        worker.join().unwrap();

        let mut reloaded = AgentServiceLockoutStore::default();
        reloaded.reload(&state_path).unwrap();
        assert!(reloaded.state.peers.get(&peer).is_none(), "successful PIN open after expiry clears durable failure state");
        assert!(!std::fs::read_to_string(&state_path).unwrap().contains("expired-attendance"));
        std::env::remove_var("CADUCEUS_STAFF_SOCKET");
        std::env::remove_var("CORONATIO_EXOUSIA_AGENT_LOCKOUT_PATH");
        std::env::remove_var("CORONATIO_HOMESERVER_JSON");
        let _ = std::fs::remove_file(&socket);
        let _ = std::fs::remove_file(&state_path);
        let _ = std::fs::remove_file(&config_path);
    }

    #[test]
    fn agent_lockout_state_has_stable_fail_closed_signal() {
        assert!(AGENT_SERVICE_LOCKOUT_STATE_FAILURE.starts_with("coronatio-"));
        assert!(AGENT_SERVICE_LOCKED_OUT.starts_with("coronatio-"));
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
