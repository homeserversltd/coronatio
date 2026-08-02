#[test]
fn firewall_pane_is_native_admin_only_and_discloses_dns_limits() {
    let pane = include_str!("../shell/document-2.rs");
    let start = pane.find("id=\"pane-firewall\"").expect("Firewall pane exists");
    let firewall = &pane[start..];
    for required in ["data-firewall-tablet", "data-admin-only=\"true\"", "data-admin-viewport=\"firewall\"", "data-firewall-device", "data-firewall-enabled", "data-firewall-add-site-form", "data-firewall-sites", "data-firewall-state", "DNS website policy", "DoH", "HTTPS hostnames", "CDN"] {
        assert!(firewall.contains(required), "firewall pane missing {required}");
    }
    assert!(!firewall.contains("data-og-stub-pane=\"firewall\""));
}

#[test]
fn firewall_registry_css_and_client_are_composed() {
    let panes = native_crown_panes();
    let firewall = panes.iter().find(|pane| pane.id == "firewall").expect("firewall registered");
    assert!(firewall.admin_only);
    assert_eq!(firewall.route, "/#firewall");
    let admin = render_crown_shell_for_session(Session::Admin);
    let guest = render_crown_shell_for_session(Session::Guest);
    let chrome = crown_chrome_js();
    assert!(admin.contains("data-firewall-tablet"));
    assert!(admin.contains("data-pane-panel=\"firewall\""));
    assert!(!guest.contains("<div class=\"firewall-tablet\""));
    assert!(!guest.contains("data-pane-panel=\"firewall\""));
    assert!(chrome.contains("hydrateFirewall"));
    assert!(chrome.contains("viewportFamilyAdmitted('firewall')"));
    let css = include_str!("../shell/ux/packs/firewall.css");
    assert!(css.contains(".firewall-tablet"));
    assert!(include_str!("../shell/render.rs").contains("packs/firewall.css"));
}

#[test]
fn firewall_policy_requests_are_canonical_and_bounded() {
    let policy: FirewallPolicyWrite = serde_json::from_value(serde_json::json!({
        "schema":"caduceus.network.firewall.policy.v1", "mac":"aa-bb-cc-dd-ee-ff", "mode":"allow-only", "sites":["Example.COM."], "expectedRevision":7, "enabled":true, "enforcement":"dns-policy"
    })).unwrap();
    let request = firewall_policy_request(policy, "AA:BB:CC:DD:EE:FF").unwrap();
    assert_eq!(request["mac"], "AA:BB:CC:DD:EE:FF");
    assert_eq!(request["sites"], serde_json::json!(["example.com"]));
    assert_eq!(request["expectedRevision"], 7);
    assert!(canonical_firewall_mac("aa:bb-cc:dd:ee:ff").is_none());
    assert!(canonical_firewall_site("not a hostname").is_none());
    let delete: FirewallPolicyDelete = serde_json::from_value(serde_json::json!({
        "schema":"caduceus.network.firewall.policy.delete.v1", "mac":"aa-bb-cc-dd-ee-ff", "expectedRevision":7
    })).unwrap();
    assert_eq!(firewall_delete_request(delete, "AA:BB:CC:DD:EE:FF").unwrap(), serde_json::json!({
        "schema":"caduceus.network.firewall.policy.delete.v1", "mac":"AA:BB:CC:DD:EE:FF", "expectedRevision":7
    }));
    assert!(serde_json::from_value::<FirewallPolicyDelete>(serde_json::json!({"schema":"caduceus.network.firewall.policy.delete.v1", "mac":"AA:BB:CC:DD:EE:FF", "expectedRevision":7, "enabled":true})).is_err());
}

#[tokio::test]
async fn firewall_guest_routes_refuse_before_caduceus_contact() {
    let _guard = CADUCEUS_ENV_LOCK.get_or_init(|| std::sync::Mutex::new(())).lock().unwrap();
    std::env::set_var("CADUCEUS_URL", "http://127.0.0.1:9");
    let router = app(AppState { tab_root: Arc::new(test_tab_root("firewall-guest-refusal")) });
    for route in ["/api/firewall/status", "/api/firewall/policies", "/api/firewall/policies/AA:BB:CC:DD:EE:FF"] {
        let response = router.clone().oneshot(Request::builder().uri(route).body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED, "{route}");
        let body = String::from_utf8(axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        assert!(body.contains("admin-session-required"));
    }
    std::env::remove_var("CADUCEUS_URL");
}

#[tokio::test]
async fn firewall_preserves_authoritative_failure_body_status_and_signal() {
    let receipt = serde_json::json!({"ok":true,"changed":false,"receipt":{"bindingVerified":true},"firstMissingSignal":"dns-validation-pending"});
    let response = firewall_upstream_response(CaduceusHttpReadback { ok: true, status: 200, path: "upstream".to_string(), body: receipt.clone(), first_missing_signal: "dns-validation-pending".to_string() }, "PUT", "/api/firewall/policies/AA:BB:CC:DD:EE:FF");
    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = serde_json::from_slice(&axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap()).unwrap();
    assert_eq!(body, receipt);
    let upstream_refusal = serde_json::json!({"ok":false,"accepted":false,"firstMissingSignal":"revision-conflict","rollback":"not-needed"});
    let refusal = firewall_upstream_response(CaduceusHttpReadback { ok: false, status: 409, path: "upstream".to_string(), body: upstream_refusal.clone(), first_missing_signal: "revision-conflict".to_string() }, "PUT", "/api/firewall/policies/AA:BB:CC:DD:EE:FF");
    assert_eq!(refusal.status(), StatusCode::CONFLICT);
    let body: serde_json::Value = serde_json::from_slice(&axum::body::to_bytes(refusal.into_body(), usize::MAX).await.unwrap()).unwrap();
    assert_eq!(body, upstream_refusal);
}

#[test]
fn firewall_client_refuses_semantic_failures_and_false_positive_enforcement() {
    let client = include_str!("../shell/firewall-client.rs");
    for required in ["body?.ok === false", "body?.success === false", "body?.accepted === false", "body?.firstMissingSignal", "policy.enabled === true", "receiptMac === mac", "!missing", "Select a device first", "caduceus.network.firewall.policy.delete.v1", "No policy change"] {
        assert!(client.contains(required), "firewall client missing {required}");
    }
    for stale_success in ["Policy change read back", "Policy removal read back"] {
        assert!(!client.contains(stale_success), "optimistic copy retained {stale_success}");
    }
}

#[tokio::test]
async fn firewall_delete_rejects_missing_revision_or_mismatched_mac_before_authorization() {
    let router = app(AppState { tab_root: Arc::new(test_tab_root("firewall-delete-rejection")) });
    for body in [
        Body::empty(),
        Body::from(r#"{"schema":"caduceus.network.firewall.policy.delete.v1","mac":"AA:BB:CC:DD:EE:00","expectedRevision":7}"#),
    ] {
        let response = router.clone().oneshot(successor_admin_request(Request::builder()
            .method("DELETE")
            .uri("/api/firewall/policies/AA:BB:CC:DD:EE:FF")
            .header("content-type", "application/json")
            .body(body)
            .unwrap())).await.unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let body: serde_json::Value = serde_json::from_slice(&axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap()).unwrap();
        assert_eq!(body["firstMissingSignal"], "firewall-policy-delete-invalid");
    }
}

#[tokio::test]
async fn firewall_delete_forwards_exact_revision_document_after_authorization() {
    use std::io::{BufRead, BufReader, Read, Write};
    let _guard = CADUCEUS_ENV_LOCK.get_or_init(|| std::sync::Mutex::new(())).lock().unwrap();
    let root = test_tab_root("firewall-delete-forward");
    let config = root.join("homeserver.json");
    std::fs::write(&config, r#"{"global":{"cors":{"allowed_origins":["https://home.arpa"]}}}"#).unwrap();
    let _origin = ScopedEnv::set("CORONATIO_HOMESERVER_JSON", config.as_os_str());
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let witness = std::thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let mut reader = BufReader::new(stream.try_clone().unwrap());
        let mut request = String::new();
        let mut length = 0usize;
        loop {
            let mut line = String::new();
            reader.read_line(&mut line).unwrap();
            if let Some((name, value)) = line.split_once(':') {
                if name.eq_ignore_ascii_case("content-length") { length = value.trim().parse().unwrap(); }
            }
            request.push_str(&line);
            if line == "\r\n" { break; }
        }
        let mut body = vec![0; length];
        reader.read_exact(&mut body).unwrap();
        request.push_str(std::str::from_utf8(&body).unwrap());
        let response = r#"{"ok":true,"changed":true,"receipt":{"mac":"AA:BB:CC:DD:EE:FF"},"firstMissingSignal":"none"}"#;
        stream.write_all(format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}", response.len(), response).as_bytes()).unwrap();
        request
    });
    let _base = ScopedEnv::set("CADUCEUS_BASE_URL", format!("http://{address}"));
    let response = app(AppState { tab_root: Arc::new(root) }).oneshot(successor_admin_request(Request::builder().method("DELETE").uri("/api/firewall/policies/aa-bb-cc-dd-ee-ff").header("content-type", "application/json").body(Body::from(r#"{"schema":"caduceus.network.firewall.policy.delete.v1","mac":"aa-bb-cc-dd-ee-ff","expectedRevision":7}"#)).unwrap())).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let request = witness.join().unwrap();
    assert!(request.starts_with("DELETE /api/v1/network/firewall/policies/AA:BB:CC:DD:EE:FF HTTP/1.1\r\n"));
    assert!(request.ends_with(r#"{"expectedRevision":7,"mac":"AA:BB:CC:DD:EE:FF","schema":"caduceus.network.firewall.policy.delete.v1"}"#));
}
