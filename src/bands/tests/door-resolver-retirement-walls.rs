fn door_resolver_repo_file(relative: &str) -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative)
}

fn door_resolver_source(relative: &str) -> String {
    std::fs::read_to_string(door_resolver_repo_file(relative)).unwrap()
}

fn door_resolver_production_rust_files(root: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
    for entry in std::fs::read_dir(root).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            door_resolver_production_rust_files(&path, out);
        } else if path.extension().and_then(|value| value.to_str()) == Some("rs") {
            let relative = path.strip_prefix(env!("CARGO_MANIFEST_DIR")).unwrap();
            if !relative.starts_with(std::path::Path::new("src/bands/tests")) {
                out.push(path);
            }
        }
    }
}

fn door_resolver_csv_records(raw: &str) -> Vec<Vec<String>> {
    let bytes = raw.as_bytes();
    let mut records = Vec::new();
    let mut record = Vec::new();
    let mut field = String::new();
    let mut quoted = false;
    let mut index = 0;
    while index < bytes.len() {
        match bytes[index] {
            b'"' if quoted && index + 1 < bytes.len() && bytes[index + 1] == b'"' => {
                field.push('"');
                index += 2;
            }
            b'"' => {
                quoted = !quoted;
                index += 1;
            }
            b',' if !quoted => {
                record.push(std::mem::take(&mut field));
                index += 1;
            }
            b'\n' if !quoted => {
                record.push(std::mem::take(&mut field));
                records.push(std::mem::take(&mut record));
                index += 1;
            }
            b'\r' if !quoted => index += 1,
            byte => {
                field.push(byte as char);
                index += 1;
            }
        }
    }
    if !field.is_empty() || !record.is_empty() {
        record.push(field);
        records.push(record);
    }
    records
}

#[test]
fn door_resolver_retirement_forbids_dead_production_symbols() {
    let mut files = Vec::new();
    door_resolver_production_rust_files(&door_resolver_repo_file("src"), &mut files);
    let forbidden = [
        "caduceus_staff_door",
        "caduceus_staff_transition",
        "caduceus_staff_transition_with_mapping",
        "resolve_caduceus_door",
        "preload_caduceus_door_seat",
        "CaduceusDoorSeat",
        "CADUCEUS_DOOR_CACHE_PATH",
        "crown_alias",
        "doors-cache.json",
        "caduceus.doors.v1",
    ];
    let mut resolver_occurrences = 0;
    for path in files {
        let source = std::fs::read_to_string(&path).unwrap();
        for symbol in forbidden {
            assert!(!source.contains(symbol), "retired {symbol} remains in {}", path.display());
        }
        if source.contains("resolve_cartridge_door") {
            resolver_occurrences += source.matches("resolve_cartridge_door").count();
            assert_eq!(
                path.strip_prefix(env!("CARGO_MANIFEST_DIR")).unwrap(),
                std::path::Path::new("src/bands/routes.rs"),
                "resolve_cartridge_door escaped its one permitted owner"
            );
        }
    }
    assert!(resolver_occurrences > 0, "the narrow cartridge resolver disappeared");
    assert!(!door_resolver_repo_file("src/bands/caduceus-doors.rs").exists());
}

#[test]
fn door_resolver_retirement_census_is_exact() {
    let raw = std::fs::read_to_string(door_resolver_repo_file("docs/door-resolver-retirement-census.csv")).unwrap();
    let records = door_resolver_csv_records(&raw);
    let header = [
        "id", "method", "public_path", "resolver_path", "source", "disposition",
        "historical_target", "direct_target", "candidate_note",
    ];
    assert_eq!(records.len(), 138);
    assert_eq!(records[0], header.iter().map(|value| (*value).to_string()).collect::<Vec<_>>());
    let rows = &records[1..];
    assert_eq!(rows.len(), 137);
    let mut ids = std::collections::BTreeSet::new();
    let mut counts = std::collections::BTreeMap::<String, usize>::new();
    for (offset, row) in rows.iter().enumerate() {
        assert_eq!(row.len(), header.len(), "CSV row {} has the wrong width", offset + 1);
        assert_eq!(row[0], format!("R{:03}", offset + 1));
        assert!(ids.insert(row[0].clone()), "duplicate census id {}", row[0]);
        *counts.entry(row[5].clone()).or_default() += 1;
    }
    assert_eq!(ids.len(), 137);
    assert_eq!(counts.get("translation-debt"), Some(&123));
    assert_eq!(counts.get("migrate-identity"), Some(&2));
    assert_eq!(counts.get("migrate-explicit-historical-target"), Some(&9));
    assert_eq!(counts.get("preserve-cartridge-existence-check"), Some(&3));
    assert_eq!(counts.values().sum::<usize>(), 137);

    let r110 = rows.iter().find(|row| row[0] == "R110").unwrap();
    assert_eq!(r110[1], "GET");
    assert_eq!(r110[2], "/api/dhcp/config");
    assert_eq!(r110[6], "/api/v1/network/dhcp");
    let dhcp = door_resolver_source("src/bands/full-rust-routes/dhcp.rs");
    let readback_start = dhcp.find("fn dhcp_readback").unwrap();
    let readback_end = dhcp[readback_start..].find("\nfn strip_dhcp_identity").unwrap() + readback_start;
    assert!(
        dhcp[readback_start..readback_end]
            .contains("\"/api/dhcp/config\" => Some(\"/api/v1/network/dhcp\")"),
        "R110 historical target diverged from dhcp_readback",
    );

    let proven = [
        ("R006", "/api/v1/log/clear"),
        ("R131", "/api/v1/network/dns/read"),
        ("R105", "/api/v1/network/device/claim"),
        ("R106", "/api/v1/network/dhcp/boundary"),
        ("R107", "/api/v1/network/dhcp/leases"),
        ("R109", "/api/v1/network/dns/read"),
        ("R111", "/api/v1/network/dhcp/health"),
        ("R112", "/api/v1/network/dhcp/leases"),
        ("R113", "/api/v1/network/dhcp/boundary"),
        ("R119", "/api/v1/network/dhcp/statistics"),
        ("R120", "/api/v1/network/dhcp/status"),
    ];
    let live_direct_targets = [
        "/api/v1/log/clear",
        "/api/v1/network/dns/read",
        "/api/v1/network/device/claim",
        "/api/v1/network/dhcp/boundary",
        "/api/v1/network/dhcp/leases",
        "/api/v1/network/dns/read",
        "/api/v1/network/dhcp/health",
        "/api/v1/network/dhcp/leases",
        "/api/v1/network/dhcp/boundary",
        "/api/v1/network/dhcp/statistics",
        "/api/v1/network/dhcp/status",
    ];
    for ((id, target), live_target) in proven.iter().copied().zip(live_direct_targets) {
        let row = rows.iter().find(|row| row[0] == id).unwrap();
        assert_eq!(row[7], target, "direct target changed for {id}");
        assert_eq!(row[7], live_target, "direct target is outside the proven live list for {id}");
        assert!(row[5] == "migrate-identity" || row[5] == "migrate-explicit-historical-target");
        assert!(row[8].starts_with("proven direct target:"));
    }
    for row in rows.iter().filter(|row| row[5] == "preserve-cartridge-existence-check") {
        assert!(row[8].starts_with("proven cartridge existence check:"));
    }
    for row in rows.iter().filter(|row| row[5] == "translation-debt") {
        assert!(row[7].is_empty(), "debt row {} acquired a direct target", row[0]);
        assert!(row[8] == "none" || row[8].starts_with("candidate-only:"), "unsafe candidate note on {}", row[0]);
    }
}

#[test]
fn door_resolver_retirement_direct_literals_are_seated() {
    let dhcp = door_resolver_source("src/bands/full-rust-routes/dhcp.rs");
    for (public_path, target) in [
        ("/api/dhcp/status", "/api/v1/network/dhcp/status"),
        ("/api/dhcp/leases", "/api/v1/network/dhcp/leases"),
        ("/api/dhcp/health", "/api/v1/network/dhcp/health"),
        ("/api/dhcp/statistics", "/api/v1/network/dhcp/statistics"),
        ("/api/dhcp/pool-boundary", "/api/v1/network/dhcp/boundary"),
    ] {
        assert!(dhcp.contains(&format!("\"{public_path}\" => Some(\"{target}\")")));
    }
    let device = door_resolver_source("src/bands/full-rust-routes/device-identity.rs");
    for needle in [
        "device_boundary_route(headers: axum::http::HeaderMap) -> Response",
        "device_identity_read(\"/api/network/dhcp/boundary\", Some(\"/api/v1/network/dhcp/boundary\"), &headers)",
        "device_identity_read(\"/api/network/dhcp/leases\", Some(\"/api/v1/network/dhcp/leases\"), &headers)",
        "device_identity_read(\"/api/network/dns/read\", Some(\"/api/v1/network/dns/read\"), &headers)",
    ] {
        assert!(device.contains(needle), "device direct mapping literal missing: {needle}");
    }
    let claim_start = device.find("async fn device_claim_route").unwrap();
    let claim = &device[claim_start..];
    let admin_gate = claim.find("device_identity_admin").unwrap();
    let actuation_start = claim.find("caduceus_actuate_json").unwrap();
    let actuation = &claim[actuation_start..];
    assert!(admin_gate < actuation_start, "device claim attendance gate moved behind actuation");
    assert!(claim.contains("let direct_path = \"/api/v1/network/device/claim\";"));
    assert!(actuation.contains("MutationActionTarget::caduceus(\"caduceus.network.device.claim\", direct_path)"));
    assert!(actuation.contains("\n        direct_path,"));
    let unbound = door_resolver_source("src/bands/full-rust-routes/unbound.rs");
    assert!(unbound.contains("if path == \"/api/v1/network/dns/read\""));
    assert!(unbound.contains("caduceus_http(\"GET\", \"/api/v1/network/dns/read\")"));
    let caduceus = door_resolver_source("src/bands/caduceus.rs");
    assert!(caduceus.contains("admin_fragment_caduceus_json_request(&headers, \"POST\", \"/api/v1/log/clear\""));
    let authority = door_resolver_source("src/bands/mutation-authority.rs");
    assert!(authority.contains("coronatio-caduceus-route-translation-required"));
}

#[test]
fn translation_debt_readback_preserves_truthful_debt_fields() {
    let readback = translation_debt_readback(
        "PATCH",
        "/api/retired/example",
        Some("/api/v1/historical/example"),
        Some("/api/v1/candidate/example"),
    );
    assert!(!readback.ok);
    assert_eq!(readback.status, 0);
    assert_eq!(readback.path, "/api/retired/example");
    assert_eq!(readback.first_missing_signal, "coronatio-caduceus-route-translation-required");
    assert_eq!(readback.body["ok"], false);
    assert_eq!(readback.body["method"], "PATCH");
    assert_eq!(readback.body["path"], "/api/retired/example");
    assert_eq!(readback.body["historicalTarget"], "/api/v1/historical/example");
    assert_eq!(readback.body["currentCandidate"], "/api/v1/candidate/example");
    let absent = translation_debt_readback("GET", "/api/retired/no-target", None, None);
    assert!(absent.body.get("historicalTarget").is_none());
    assert!(absent.body.get("currentCandidate").is_none());
}

fn door_resolver_attendance_headers(attended: bool) -> axum::http::HeaderMap {
    let mut headers = axum::http::HeaderMap::new();
    headers.insert("host", axum::http::HeaderValue::from_static("home.arpa"));
    headers.insert("origin", axum::http::HeaderValue::from_static("https://home.arpa"));
    headers.insert("x-caduceus-document", axum::http::HeaderValue::from_static("test-document"));
    if attended {
        headers.insert("x-caduceus-attendance", axum::http::HeaderValue::from_static("test-attendance"));
    }
    headers
}

fn door_resolver_attendance_fixture() -> (std::path::PathBuf, std::thread::JoinHandle<String>) {
    use std::io::Write;
    use std::os::unix::net::UnixListener;
    let socket = std::env::temp_dir().join(format!("coronatio-door-debt-{}-{}.sock", std::process::id(), uuid::Uuid::new_v4()));
    let listener = UnixListener::bind(&socket).unwrap();
    listener.set_nonblocking(true).unwrap();
    let thread = std::thread::spawn(move || {
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(3);
        let (mut stream, _) = loop {
            match listener.accept() {
                Ok(value) => break value,
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                    assert!(std::time::Instant::now() < deadline, "attendance accept deadline expired");
                    std::thread::sleep(std::time::Duration::from_millis(5));
                }
                Err(error) => panic!("attendance accept failed: {error}"),
            }
        };
        let request = door_resolver_read_uds_request(&mut stream);
        let body = b"{\"ok\":true}";
        let response = format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}", body.len(), std::str::from_utf8(body).unwrap());
        stream.write_all(response.as_bytes()).unwrap();
        request
    });
    (socket, thread)
}

#[derive(Clone, Copy)]
struct DoorResolverMockReply {
    request_prefix: &'static str,
    status: &'static str,
    body: &'static str,
}

fn door_resolver_read_uds_request(stream: &mut std::os::unix::net::UnixStream) -> String {
    use std::io::{BufRead, BufReader, Read};
    stream.set_read_timeout(Some(std::time::Duration::from_millis(500))).unwrap();
    stream.set_write_timeout(Some(std::time::Duration::from_millis(500))).unwrap();
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut request = String::new();
    let mut content_length = 0usize;
    loop {
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        assert!(!line.is_empty(), "Caduceus mock received a truncated request");
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
    request
}

fn door_resolver_uds_fixture(
    expected_requests: usize,
    replies: Vec<DoorResolverMockReply>,
) -> (std::path::PathBuf, std::thread::JoinHandle<Vec<String>>) {
    use std::io::Write;
    use std::os::unix::net::UnixListener;
    let socket = std::env::temp_dir().join(format!(
        "coronatio-door-router-{}-{}.sock",
        std::process::id(),
        uuid::Uuid::new_v4()
    ));
    let listener = UnixListener::bind(&socket).unwrap();
    listener.set_nonblocking(true).unwrap();
    let thread = std::thread::spawn(move || {
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(3);
        let mut requests = Vec::new();
        while requests.len() < expected_requests && std::time::Instant::now() < deadline {
            let (mut stream, _) = match listener.accept() {
                Ok(value) => value,
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                    std::thread::sleep(std::time::Duration::from_millis(5));
                    continue;
                }
                Err(error) => panic!("Caduceus mock accept failed: {error}"),
            };
            let request = door_resolver_read_uds_request(&mut stream);
            let request_line = request.lines().next().unwrap_or_default();
            let reply = replies
                .iter()
                .find(|reply| request_line.starts_with(reply.request_prefix))
                .unwrap_or_else(|| panic!("unexpected Caduceus request: {request_line}"));
            stream
                .write_all(
                    format!(
                        "HTTP/1.1 {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                        reply.status,
                        reply.body.len(),
                        reply.body,
                    )
                    .as_bytes(),
                )
                .unwrap();
            requests.push(request);
        }
        assert_eq!(
            requests.len(),
            expected_requests,
            "Caduceus mock accept deadline expired",
        );
        requests
    });
    (socket, thread)
}

fn door_resolver_validate_reply() -> DoorResolverMockReply {
    DoorResolverMockReply {
        request_prefix: "POST /api/v1/exousia/validate HTTP/1.1",
        status: "200 OK",
        body: r#"{"ok":true,"code":"none"}"#,
    }
}

fn door_resolver_request_json(request: &str) -> serde_json::Value {
    serde_json::from_str(request.split_once("\r\n\r\n").unwrap().1).unwrap()
}

#[test]
fn attended_translation_debt_validates_then_never_posts_debt_path() {
    let _guard = crate::CADUCEUS_ENV_LOCK.get_or_init(|| std::sync::Mutex::new(())).lock().unwrap();
    let (socket, server) = door_resolver_attendance_fixture();
    std::env::set_var("CADUCEUS_STAFF_SOCKET", &socket);
    let authority = mutation_authority();
    let readback = caduceus_translation_debt(
        &authority,
        &door_resolver_attendance_headers(true),
        MutationActionTarget::caduceus("coronatio.linker.browse", "/api/retired/example"),
        "POST",
        "/api/retired/example",
        Some("/api/v1/historical/example"),
        Some("/api/v1/candidate/example"),
    );
    let request = server.join().unwrap();
    std::env::remove_var("CADUCEUS_STAFF_SOCKET");
    let _ = std::fs::remove_file(socket);
    assert!(!readback.ok);
    assert_eq!(readback.status, 0);
    assert_eq!(readback.first_missing_signal, "coronatio-caduceus-route-translation-required");
    assert!(request.starts_with("POST /api/v1/exousia/validate HTTP/1.1"), "attendance validation was not performed: {request}");
    assert!(request.contains("test-document"));
    assert!(request.contains("test-attendance"));
    assert!(!request.contains("/api/retired/example"));
    assert!(!request.contains("/api/v1/candidate/example"));
}

#[test]
fn attended_translation_debt_refuses_missing_attendance() {
    let _guard = crate::CADUCEUS_ENV_LOCK.get_or_init(|| std::sync::Mutex::new(())).lock().unwrap();
    let absent = guaranteed_absent_caduceus_socket();
    std::env::set_var("CADUCEUS_STAFF_SOCKET", &absent);
    let readback = caduceus_translation_debt(
        &mutation_authority(),
        &door_resolver_attendance_headers(false),
        MutationActionTarget::caduceus("coronatio.linker.browse", "/api/retired/example"),
        "POST",
        "/api/retired/example",
        None,
        None,
    );
    std::env::remove_var("CADUCEUS_STAFF_SOCKET");
    assert!(!readback.ok);
    assert_eq!(readback.status, 401);
    assert_eq!(readback.first_missing_signal, "caduceus-attendance-required");
    assert_eq!(readback.body["error"], "caduceus-mutation-refused");
}

#[test]
fn cartridge_resolver_is_narrow_and_ordered() {
    let source = door_resolver_source("src/bands/routes.rs");
    let start = source.find("fn cartridge_mutation_proxy_response").unwrap();
    let end = source[start..].find("\nfn tab_bar_html_response").unwrap() + start;
    let mutation = &source[start..end];
    let authorize = mutation.find("authority.authorize").unwrap();
    let resolve = mutation.find("resolve_cartridge_door").unwrap();
    let caduceus = mutation.find("caduceus_http_json_with_attendance_and_document").unwrap();
    assert!(authorize < resolve && resolve < caduceus, "cartridge order crossed an authority boundary");

    let resolver_start = source.find("fn resolve_cartridge_door").unwrap();
    let resolver_end = source[resolver_start..].find("\nasync fn cartridges_read_proxy_route").unwrap() + resolver_start;
    let resolver = &source[resolver_start..resolver_end];
    assert!(resolver.contains("caduceus.doors.readback.v1"));
    assert!(resolver.contains("routes.iter().any"));
    for route in [
        "(\"GET\", \"/api/v1/cartridges\") => \"/api/v1/cartridges/list\"",
        "(\"POST\", \"/api/v1/cartridges/admit\") => \"/api/v1/cartridges/admit\"",
        "(\"POST\", \"/api/v1/cartridges/remove\") => \"/api/v1/cartridges/remove\"",
    ] {
        assert!(resolver.contains(route), "flat cartridge route missing: {route}");
    }
    assert!(resolver.contains("_ => return Err(CartridgeRouteFailure::Unmapped)"));
    for retired in ["nested", "seat", "caduceus.doors.v1", "CADUCEUS_DOOR_CACHE_PATH", "doors-cache.json"] {
        assert!(!resolver.contains(retired), "narrow resolver retained retired fallback {retired}");
    }
}

#[test]
fn device_identity_browser_uses_the_attendance_decorator_and_stable_failure_order() {
    let document = door_resolver_source("src/bands/shell/document-2.rs");
    assert_eq!(document.matches("window.fetch = decoratedFetch").count(), 1);
    assert!(document.contains("headers.set('X-Caduceus-Document', coronatioAttendanceRuntime.documentIncarnation)"));
    assert!(document.contains("headers.set('X-Caduceus-Attendance', coronatioAttendanceRuntime.currentAttendance)"));

    let client = door_resolver_source("src/bands/shell/dhcp-client.rs");
    let identity_start = client.find("async function identityJson").unwrap();
    let identity_end = client[identity_start..].find("\n    function identityIp").unwrap() + identity_start;
    let identity = &client[identity_start..identity_end];
    assert!(identity.contains("const response = await fetch(route"));
    assert!(!identity.contains("schema"), "identityJson must not inspect a response schema");
    let boundary = identity.find("first_failing_boundary").unwrap();
    let missing = identity.find("firstMissingSignal").unwrap();
    let error = identity[missing + "firstMissingSignal".len()..].find("body.error").unwrap()
        + missing
        + "firstMissingSignal".len();
    assert!(boundary < missing && missing < error);
    let claim_start = client.find("async function saveIdentityClaim").unwrap();
    let claim_end = client[claim_start..].find("\n    async function pinDhcpLease").unwrap() + claim_start;
    let claim = &client[claim_start..claim_end];
    assert!(claim.contains("identityJson('/api/network/device/claim'"));
    assert_eq!(client.matches("'/api/network/device/claim'").count(), 1);
}

#[tokio::test]
async fn cartridge_router_sends_only_strict_raw_caduceus_bodies() {
    let _guard = crate::CADUCEUS_ENV_LOCK.get_or_init(|| std::sync::Mutex::new(())).lock().unwrap();
    let root = test_tab_root("cartridge-strict-raw-bodies");
    let _origin = note_test_origin(&root);
    for (name, route, input, expected) in [
        (
            "admit",
            "/api/v1/cartridges/admit",
            r#"{"id":"ignored","title":"media","url":"https://media.home.arpa","guest_class":"iframe","admin_only":true,"schema":"caduceus.staff.v1","intent_id":"intent-1","transition":"admit","payload":{"wrong":true},"extra":"drop-me"}"#,
            serde_json::json!({
                "id": "media",
                "title": "media",
                "url": "https://media.home.arpa",
                "guest_class": "iframe",
                "admin_only": true
            }),
        ),
        (
            "remove",
            "/api/v1/cartridges/remove",
            r#"{"id":"media","schema":"caduceus.staff.v1","intent_id":"intent-2","transition":"remove","payload":{"wrong":true},"extra":"drop-me"}"#,
            serde_json::json!({"id": "media"}),
        ),
    ] {
        let mutation_prefix: &'static str = if name == "admit" {
            "POST /api/v1/cartridges/admit HTTP/1.1"
        } else {
            "POST /api/v1/cartridges/remove HTTP/1.1"
        };
        let (socket, witness) = door_resolver_uds_fixture(
            4,
            vec![
                door_resolver_validate_reply(),
                DoorResolverMockReply {
                    request_prefix: "GET /api/v1/doors HTTP/1.1",
                    status: "200 OK",
                    body: r#"{"schema":"caduceus.doors.readback.v1","ok":true,"routes":["/api/v1/cartridges/list","/api/v1/cartridges/admit","/api/v1/cartridges/remove"]}"#,
                },
                DoorResolverMockReply {
                    request_prefix: mutation_prefix,
                    status: "200 OK",
                    body: r#"{"ok":true,"firstMissingSignal":"none"}"#,
                },
            ],
        );
        let _socket = ScopedEnv::set("CADUCEUS_STAFF_SOCKET", socket.as_os_str());
        let response = tokio::time::timeout(
            std::time::Duration::from_secs(3),
            app(AppState { tab_root: Arc::new(root.clone()) }).oneshot(successor_admin_request(
                Request::builder()
                    .method("POST")
                    .uri(route)
                    .header("content-type", "application/json")
                    .body(Body::from(input))
                    .unwrap(),
            )),
        )
        .await
        .unwrap()
        .unwrap();
        assert_eq!(response.status(), StatusCode::OK, "{name}");
        let requests = witness.join().unwrap();
        assert_eq!(
            requests
                .iter()
                .filter(|request| request.starts_with("POST /api/v1/exousia/validate HTTP/1.1"))
                .count(),
            2,
            "{name} must account for session and authority validation",
        );
        assert_eq!(
            requests
                .iter()
                .filter(|request| request.starts_with("GET /api/v1/doors HTTP/1.1"))
                .count(),
            1,
        );
        let mutation = requests
            .iter()
            .find(|request| request.starts_with(mutation_prefix))
            .unwrap();
        let actual = door_resolver_request_json(mutation);
        assert_eq!(actual, expected, "{name} sent the wrong strict raw body");
        for forbidden in ["schema", "intent_id", "transition", "payload", "extra"] {
            assert!(actual.get(forbidden).is_none(), "{name} forwarded {forbidden}");
        }
        let _ = std::fs::remove_file(socket);
    }
}

#[tokio::test]
async fn device_claim_router_keeps_the_preexisting_attendance_gate() {
    let _guard = crate::CADUCEUS_ENV_LOCK.get_or_init(|| std::sync::Mutex::new(())).lock().unwrap();
    let _socket = ScopedEnv::set("CADUCEUS_STAFF_SOCKET", guaranteed_absent_caduceus_socket().as_os_str());
    let response = app(AppState { tab_root: Arc::new(test_tab_root("device-claim-missing-attendance")) })
        .oneshot(successor_session_request(
            Request::builder()
                .method("POST")
                .uri("/api/network/device/claim")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"mac":"AA:BB:CC:DD:EE:FF","hostname":"nas"}"#))
                .unwrap(),
            false,
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    let body: serde_json::Value = serde_json::from_slice(
        &axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap(),
    )
    .unwrap();
    assert_eq!(body["schema"], "coronatio.network.identity.refusal.v1");
    assert_eq!(body["error"], "admin-session-required");
    assert_eq!(body["firstMissingSignal"], "admin-session-required");
}

#[tokio::test]
async fn device_claim_router_dispatches_attended_raw_json_and_preserves_caduceus_failure() {
    let _guard = crate::CADUCEUS_ENV_LOCK.get_or_init(|| std::sync::Mutex::new(())).lock().unwrap();
    let root = test_tab_root("device-claim-attended-dispatch");
    let _origin = note_test_origin(&root);
    let upstream_failure = r#"{"schema":"caduceus.api.error.v1","ok":false,"command":"network device claim","firstMissingSignal":"caduceus-network-identity-claim-arguments-invalid"}"#;
    let (socket, witness) = door_resolver_uds_fixture(
        3,
        vec![
            door_resolver_validate_reply(),
            DoorResolverMockReply {
                request_prefix: "POST /api/v1/network/device/claim HTTP/1.1",
                status: "503 Service Unavailable",
                body: upstream_failure,
            },
        ],
    );
    let _socket = ScopedEnv::set("CADUCEUS_STAFF_SOCKET", socket.as_os_str());
    let payload = serde_json::json!({"mac": "AA:BB:CC:DD:EE:FF", "hostname": "nas"});
    let response = tokio::time::timeout(
        std::time::Duration::from_secs(3),
        app(AppState { tab_root: Arc::new(root) }).oneshot(successor_admin_request(
            Request::builder()
                .method("POST")
                .uri("/api/network/device/claim")
                .header("content-type", "application/json")
                .body(Body::from(payload.to_string()))
                .unwrap(),
        )),
    )
    .await
    .unwrap()
    .unwrap();
    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    let response_body: serde_json::Value = serde_json::from_slice(
        &axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap(),
    )
    .unwrap();
    assert_eq!(response_body, serde_json::from_str::<serde_json::Value>(upstream_failure).unwrap());
    let requests = witness.join().unwrap();
    assert_eq!(
        requests
            .iter()
            .filter(|request| request.starts_with("POST /api/v1/exousia/validate HTTP/1.1"))
            .count(),
        2,
    );
    let claim = requests
        .iter()
        .find(|request| request.starts_with("POST /api/v1/network/device/claim HTTP/1.1"))
        .unwrap();
    assert!(claim.contains("x-caduceus-document: test-document\r\n"));
    assert!(claim.contains("x-caduceus-attendance: test-attendance\r\n"));
    assert_eq!(door_resolver_request_json(claim), payload);
    let _ = std::fs::remove_file(socket);
}

#[tokio::test]
async fn migrated_direct_router_routes_capture_the_exact_uds_targets_and_failure_shape() {
    let _guard = crate::CADUCEUS_ENV_LOCK.get_or_init(|| std::sync::Mutex::new(())).lock().unwrap();

    let (status_socket, status_witness) = door_resolver_uds_fixture(
        1,
        vec![DoorResolverMockReply {
            request_prefix: "GET /api/v1/network/dhcp/status HTTP/1.1",
            status: "200 OK",
            body: r#"{"ok":true,"payload":{"result":{"service":"kea","state":"running"}}}"#,
        }],
    );
    let status_response = {
        let _socket = ScopedEnv::set("CADUCEUS_STAFF_SOCKET", status_socket.as_os_str());
        tokio::time::timeout(
            std::time::Duration::from_secs(3),
            app(AppState { tab_root: Arc::new(test_tab_root("direct-dhcp-status")) })
                .oneshot(Request::builder().uri("/api/dhcp/status").body(Body::empty()).unwrap()),
        )
        .await
        .unwrap()
        .unwrap()
    };
    assert_eq!(status_response.status(), StatusCode::OK);
    let status_body: serde_json::Value = serde_json::from_slice(
        &axum::body::to_bytes(status_response.into_body(), usize::MAX).await.unwrap(),
    )
    .unwrap();
    assert_eq!(status_body, serde_json::json!({"service": "kea", "state": "running"}));
    let status_requests = status_witness.join().unwrap();
    assert_eq!(status_requests[0].lines().next(), Some("GET /api/v1/network/dhcp/status HTTP/1.1"));
    let _ = std::fs::remove_file(status_socket);

    let root = test_tab_root("direct-device-boundary-failure");
    let _origin = note_test_origin(&root);
    let (boundary_socket, boundary_witness) = door_resolver_uds_fixture(
        2,
        vec![
            door_resolver_validate_reply(),
            DoorResolverMockReply {
                request_prefix: "GET /api/v1/network/dhcp/boundary HTTP/1.1",
                status: "503 Service Unavailable",
                body: r#"{"ok":false,"error":"dhcp-boundary-read-failed","firstMissingSignal":"dhcp-boundary-unavailable"}"#,
            },
        ],
    );
    let boundary_response = {
        let _socket = ScopedEnv::set("CADUCEUS_STAFF_SOCKET", boundary_socket.as_os_str());
        tokio::time::timeout(
            std::time::Duration::from_secs(3),
            app(AppState { tab_root: Arc::new(root) }).oneshot(successor_admin_request(
                Request::builder()
                    .uri("/api/network/dhcp/boundary")
                    .body(Body::empty())
                    .unwrap(),
            )),
        )
        .await
        .unwrap()
        .unwrap()
    };
    assert_eq!(boundary_response.status(), StatusCode::SERVICE_UNAVAILABLE);
    let boundary_body: serde_json::Value = serde_json::from_slice(
        &axum::body::to_bytes(boundary_response.into_body(), usize::MAX).await.unwrap(),
    )
    .unwrap();
    assert_eq!(boundary_body["schema"], "coronatio.network.identity.read.error.v1");
    assert_eq!(boundary_body["firstMissingSignal"], "dhcp-boundary-unavailable");
    let boundary_requests = boundary_witness.join().unwrap();
    assert_eq!(
        boundary_requests
            .iter()
            .filter(|request| request.starts_with("POST /api/v1/exousia/validate HTTP/1.1"))
            .count(),
        1,
    );
    assert!(boundary_requests
        .iter()
        .any(|request| request.starts_with("GET /api/v1/network/dhcp/boundary HTTP/1.1")));
    let _ = std::fs::remove_file(boundary_socket);
}

#[test]
fn backblaze_debt_wall_preserves_literals_and_local_guard() {
    let source = door_resolver_source("src/bands/full-rust-routes/backblaze.rs");
    for route in [
        "/api/backblaze/buckets",
        "/api/backblaze/buckets/:bucket",
        "/api/backblaze/buckets/:bucket/items",
        "/api/backblaze/buckets/:bucket/run",
        "/api/backblaze/buckets/:bucket/toggle",
        "/api/backblaze/buckets/:bucket/verify",
        "/api/backblaze/config",
    ] {
        let position = source.find(route).unwrap_or_else(|| panic!("missing Backblaze literal {route}"));
        let function_start = source[..position].rfind("async fn").unwrap();
        let function_end = source[position..].find("\nasync fn").map(|offset| position + offset).unwrap_or(source.len());
        assert!(source[function_start..function_end].contains("route_translation_debt("), "former crossing {route} bypasses debt");
    }
    for line in source.lines().filter(|line| line.contains("format!") && line.contains("bucket")) {
        assert!(!line.contains("/api/") && !line.contains("http"), "bucket id was formatted into an upstream path: {line}");
    }
    let post_start = source.find("async fn backblaze_bucket_post_route").unwrap();
    let post_end = source[post_start..].find("\nasync fn").map(|offset| post_start + offset).unwrap_or(source.len());
    let post = &source[post_start..post_end];
    assert!(post.find("if !r.ok").unwrap() < post.find("save_backblaze_config").unwrap());
    let verify_start = source.find("async fn backblaze_bucket_verify_route").unwrap();
    let verify_end = source[verify_start..].find("\nasync fn").map(|offset| verify_start + offset).unwrap_or(source.len());
    let verify = &source[verify_start..verify_end];
    assert!(verify.find("if !r.ok").unwrap() < verify.find("write_backblaze_state").unwrap());
}
