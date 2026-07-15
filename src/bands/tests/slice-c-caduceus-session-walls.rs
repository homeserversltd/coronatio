#[test]
fn successor_session_transport_has_only_the_three_crown_routes_and_safe_cookie() {
    let runtime = include_str!("../runtime.rs");
    for route in ["/api/session/mint", "/api/session/prove", "/api/session/clear"] {
        assert!(runtime.contains(route), "missing {route}");
    }
    assert!(!runtime.contains("/api/session\""));
    let access = include_str!("../caduceus-access.rs");
    assert!(access.contains("HttpOnly; Secure; SameSite=Strict; Path=/; Max-Age=1800"));
    assert!(access.contains("Max-Age=0"));
    for retired in ["SessionRefresh", "PinChange", "session_refresh", "pin_change"] {
        assert!(!access.contains(retired), "retired access operation survives: {retired}");
    }
}

#[test]
fn successor_uses_cookie_context_owned_mapping_and_one_scoped_internal_capability_only() {
    let authority = include_str!("../mutation-authority.rs");
    assert!(authority.contains("MutationRequestContext::from_headers"));
    assert!(authority.contains("same_origin"));
    assert!(authority.contains("capability_mint(ticket, &mapping.action, &mapping.target)"));
    assert!(authority.contains("expose_for_one_request"));
    assert!(authority.contains("action: String"));
    assert!(authority.contains("target: String"));
    for forbidden in ["Box::leak", "Transitional"] {
        assert!(!authority.contains(forbidden), "forbidden authority substrate: {forbidden}");
    }
}

#[test]
fn assembled_crown_shell_uses_cookie_native_prove_and_clear_without_bearer_artifacts() {
    let shell = render_crown_shell();
    for required in [
        "credentials: 'same-origin'",
        "fetch('/api/session/prove', { method: 'POST', cache: 'no-store' })",
        "response.ok && result.admin === true",
        "async function clearAdminMode()",
        "fetch('/api/session/clear', { method: 'POST', cache: 'no-store' })",
        "finally {\n        setAdminMode(false);",
        "const explicitPinRefusal = response.status === 401 && result?.firstMissingSignal === 'caduceus-access-refused';",
        "if (explicitPinRefusal) { modalMessage.textContent = 'Invalid PIN'; return; }",
        "if (!response.ok || result.admin !== true) { modalMessage.textContent = 'PIN check unavailable'; return; }",
    ] {
        assert!(shell.contains(required), "served shell missing {required}");
    }
    for forbidden in [
        "const token = null",
        "headers: { '':",
        "...(token ?",
        "htmx:configRequest",
        "result.authenticated",
        "PIN changed successfully",
        "Authorization",
        "X-Admin-Token",
    ] {
        assert!(!shell.contains(forbidden), "served shell retains {forbidden}");
    }
    assert!(shell.contains("PIN changes are unavailable until a successor route is declared."));
}
