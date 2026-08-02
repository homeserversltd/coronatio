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
fn firewall_policy_request_canonicalizes_mac_and_bounds_dns_policy() {
    let policy: FirewallPolicyWrite = serde_json::from_value(serde_json::json!({
        "schema":"caduceus.network.firewall.policy.v1", "mac":"aa-bb-cc-dd-ee-ff", "mode":"allow-only", "sites":["Example.COM."], "expectedRevision":7, "enabled":true, "enforcement":"dns-policy"
    })).unwrap();
    let request = firewall_policy_request(policy, "AA:BB:CC:DD:EE:FF").unwrap();
    assert_eq!(request["mac"], "AA:BB:CC:DD:EE:FF");
    assert_eq!(request["sites"], serde_json::json!(["example.com"]));
    assert_eq!(request["expectedRevision"], 7);
    assert!(canonical_firewall_mac("aa:bb-cc:dd:ee:ff").is_none());
    assert!(canonical_firewall_site("not a hostname").is_none());
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
async fn firewall_preserves_authoritative_receipt_and_first_missing_signal() {
    let receipt = serde_json::json!({"ok":true,"changed":false,"receipt":{"bindingVerified":true},"firstMissingSignal":"dns-validation-pending"});
    let response = firewall_upstream_response(CaduceusHttpReadback { ok: true, status: 200, path: "/api/v1/network/firewall/policies/AA:BB:CC:DD:EE:FF".to_string(), body: receipt.clone(), first_missing_signal: "dns-validation-pending".to_string() }, "PUT", "/api/firewall/policies/AA:BB:CC:DD:EE:FF");
    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = serde_json::from_slice(&axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap()).unwrap();
    assert_eq!(body, receipt);
    let refusal = firewall_upstream_response(CaduceusHttpReadback { ok: false, status: 409, path: "upstream".to_string(), body: serde_json::json!({}), first_missing_signal: "revision-conflict".to_string() }, "PUT", "/api/firewall/policies/AA:BB:CC:DD:EE:FF");
    let body: serde_json::Value = serde_json::from_slice(&axum::body::to_bytes(refusal.into_body(), usize::MAX).await.unwrap()).unwrap();
    assert_eq!(body["firstMissingSignal"], "revision-conflict");
}
