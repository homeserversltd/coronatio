#[test]
fn crown_debug_emitter_is_runtime_gated_and_inert_when_disabled() {
    let chrome = crown_chrome_js();
    let emitter = chrome.find("window.crownDebug = crownDebug;").expect("crown debug emitter exposed");
    let floor = chrome.find("const immortalFloorStates = Object.freeze").expect("immortal floor iife follows emitter");
    assert!(emitter < floor, "emitter must be seated before ImmortalFloor");
    assert!(chrome.contains("params.has('debug')"));
    assert!(chrome.contains("storageValue('coronatioDebug')"));
    assert!(chrome.contains("storageValue('coronatioDebugKinds')"));
    assert!(chrome.contains("if (!enabled(kind)) return null;"), "begin must not allocate ids/rings while disabled");
    assert!(chrome.contains("if (!enabled(kind)) return false;"), "emit must not fetch while disabled");
    assert!(chrome.contains("fetch(endpoint"));
    assert!(chrome.contains("const endpoint = '/api/debug/emit', ttlMs"));
    assert!(!chrome.contains("/api/v1/hyalos/"), "client shell must never call Hyalos directly");
    assert!(!chrome.contains("channel.jsonl"), "client shell must never write or name channel jsonl");
}

#[test]
fn crown_diagnostics_request_and_layout_producers_are_bounded_and_non_recursive() {
    let chrome = crown_chrome_js();
    assert!(chrome.contains("installCrownRequestDiagnostics(crownDebug)"));
    assert!(chrome.contains("installCrownLayoutDiagnostics(crownDebug)"));
    for phase in ["htmx:beforeRequest", "htmx:afterRequest", "htmx:responseError", "htmx:sendError", "htmx:beforeSwap", "htmx:afterSwap"] {
        assert!(chrome.contains(phase), "missing HTMX diagnostics phase {phase}");
    }
    assert!(chrome.contains("crownDebug.enabled('crown-requests')"));
    assert!(chrome.contains("crownDebug.enabled('crown-layout')"));
    assert!(chrome.contains("url.origin === window.location.origin && url.pathname !== '/api/debug/emit'"));
    assert!(chrome.contains("['paint', 'layout-shift', 'longtask'].forEach(observe)"));
    assert!(chrome.contains("entryType: type"));
    assert!(chrome.contains("hadRecentInput = Boolean(entry.hadRecentInput)"));
    assert!(chrome.contains("attrs.value = Math.max(0, Number(entry.value || 0))"));
    assert!(!chrome.contains("entry.sources"), "layout diagnostics must not emit DOM attribution");
    assert!(chrome.contains("const attrs = { phase, pathname: path };"));
}

#[test]
fn immortal_floor_uses_crown_debug_as_first_consumer_without_new_machine_state() {
    let chrome = crown_chrome_js();
    assert_eq!(chrome.matches("const immortalFloorStates = Object.freeze(['BootFloor', 'Seated', 'GuestRevolution', 'BareFloor']);").count(), 1);
    assert!(chrome.contains("crownDebug.begin('immortal-floor-boot'"));
    for mark in ["admission-trigger", "after-swap", "admission-fault", "admission-timeout", "hydration-timeout"] {
        assert!(chrome.contains(&format!("'{mark}'")), "missing debug mark {mark}");
    }
    assert!(chrome.contains("crownDebug.settle(floorDebugHandle, true"));
    assert!(chrome.contains("crownDebug.settle(floorDebugHandle, false"));
    assert!(chrome.contains("phase: 'bare-floor'"));
    assert!(!chrome.contains("immortal-floor-boot-admission"), "phase belongs in attributes, not kind");
}

#[tokio::test]
async fn debug_emit_route_forwards_unsigned_hyalos_reflect_with_trimmed_payload() {
    let _guard = CADUCEUS_ENV_LOCK.get_or_init(|| std::sync::Mutex::new(())).lock().unwrap();
    use std::io::{Read, Write};
    use std::net::{Shutdown, TcpListener};
    use std::sync::{Arc, Mutex};
    use std::thread;

    let listener = TcpListener::bind("127.0.0.1:0").expect("mock caduceus bind");
    let port = listener.local_addr().expect("mock caduceus addr").port();
    let captured = Arc::new(Mutex::new(String::new()));
    let captured_thread = captured.clone();
    let handle = thread::spawn(move || {
        if let Ok((mut stream, _addr)) = listener.accept() {
            let mut buf = [0u8; 8192];
            let n = stream.read(&mut buf).unwrap_or(0);
            *captured_thread.lock().unwrap() = String::from_utf8_lossy(&buf[..n]).to_string();
            let body = serde_json::json!({"ok": true, "firstMissingSignal": "none"}).to_string();
            let response = format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nConnection: close\r\nContent-Length: {}\r\n\r\n{}", body.len(), body);
            let _ = stream.write_all(response.as_bytes());
            let _ = stream.shutdown(Shutdown::Write);
        }
    });
    std::env::set_var("CADUCEUS_BASE_URL", format!("http://127.0.0.1:{}", port));
    let response = app(AppState { tab_root: Arc::new(test_tab_root("debug-emitter-app")) })
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/debug/emit")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::json!({
                    "kind": "immortal-floor-boot",
                    "event": "settle",
                    "correlationId": "boot-123",
                    "attributes": {
                        "phase": "seated",
                        "adminToken": "do-not-forward",
                        "pin": "1234",
                        "headers": {"x-caduceus-capability": "nope"},
                        "payload": {"secret": "do-not-forward"},
                        "nested": {"body": "do-not-forward", "requestBody": "do-not-forward", "responseBody": "do-not-forward", "raw_body": "do-not-forward", "payloadData": "do-not-forward", "nestedPayload": "do-not-forward", "connectionString": "do-not-forward", "snapshot": "do-not-forward"},
                        "array": [{"localStorage": "do-not-forward"}, {"safeNested": "survives"}],
                        "marks": [{"mark": "admission-trigger"}]
                    }
                }).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    handle.join().unwrap();
    std::env::remove_var("CADUCEUS_BASE_URL");
    let request = captured.lock().unwrap().clone();
    assert!(request.starts_with("POST /api/v1/hyalos/reflect HTTP/1.1"), "debug reflect must use the Hyalos route");
    assert!(!request.to_ascii_lowercase().contains("x-caduceus-capability:"), "debug reflect must be unsigned");
    let body = request.split("\r\n\r\n").nth(1).unwrap_or("");
    let body: serde_json::Value = serde_json::from_str(body).expect("forwarded json body");
    assert_eq!(body["organ"], "coronatio");
    assert_eq!(body["kind"], "immortal-floor-boot");
    assert_eq!(body["level"], "debug");
    assert_eq!(body["message"], "settle");
    assert_eq!(body["correlation_id"], "boot-123");
    assert_eq!(body["attributes_redacted"]["phase"], "seated");
    assert_eq!(body["attributes_redacted"]["marks"][0]["mark"], "admission-trigger");
    assert_eq!(body["attributes_redacted"]["array"][1]["safeNested"], "survives");
    assert!(body["attributes_redacted"].get("adminToken").is_none());
    assert!(body["attributes_redacted"].get("pin").is_none());
    assert!(body["attributes_redacted"].get("headers").is_none());
    assert!(body["attributes_redacted"].get("payload").is_none());
    assert!(body["attributes_redacted"]["nested"].get("body").is_none());
    assert!(body["attributes_redacted"]["nested"].get("requestBody").is_none());
    assert!(body["attributes_redacted"]["nested"].get("responseBody").is_none());
    assert!(body["attributes_redacted"]["nested"].get("raw_body").is_none());
    assert!(body["attributes_redacted"]["nested"].get("payloadData").is_none());
    assert!(body["attributes_redacted"]["nested"].get("nestedPayload").is_none());
    assert!(body["attributes_redacted"]["nested"].get("connectionString").is_none());
    assert!(body["attributes_redacted"]["nested"].get("snapshot").is_none());
    assert!(body["attributes_redacted"]["array"][0].get("localStorage").is_none());
    assert!(!body.to_string().contains("do-not-forward"), "forwarded payload must not retain sentinel values");
}

#[tokio::test]
async fn debug_emit_route_rejects_non_kebab_kind_taxonomy() {
    let _guard = CADUCEUS_ENV_LOCK.get_or_init(|| std::sync::Mutex::new(())).lock().unwrap();
    let response = app(AppState { tab_root: Arc::new(test_tab_root("debug-emitter-taxonomy-app")) })
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/debug/emit")
                .header("Content-Type", "application/json")
                .body(Body::from(r#"{"kind":"ImmortalFloorBoot","event":"settle","attributes":{"phase":"seated"}}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
