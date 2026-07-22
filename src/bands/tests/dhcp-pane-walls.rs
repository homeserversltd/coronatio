#[test]
fn dhcp_pane_replaces_stub_with_native_tablet_controls() {
    let pane = include_str!("../shell/document-2.rs");
    let start = pane.find("id=\"pane-dhcp\"").expect("DHCP pane exists");
    let dhcp = &pane[start..];
    assert!(dhcp.contains("class=\"dhcp-tablet\""));
    assert!(!dhcp.contains("data-og-stub-pane=\"dhcp\""));
    for required in [
        "data-dhcp-info-banner",
        "data-dhcp-items",
        "data-dhcp-add-form",
        "data-dhcp-boundary",
        "data-dhcp-anonymize",
        "ui-toggle__input",
        "ui-button ui-button--primary",
        "ui-input ui-input--medium",
        "ui-badge ui-badge--success",
        "loading-spinner medium",
        "data-dhcp-modal-backdrop",
        "data-admin-viewport=\"dhcp\"",
    ] {
        assert!(dhcp.contains(required), "DHCP pane missing {required}");
    }
    for retired in [
        "dhcp-action-button",
        "anonymize-toggle-input",
        "mac-input",
        "ip-input",
        "pinned-badge",
    ] {
        assert!(
            !dhcp.contains(retired),
            "DHCP pane retains legacy control {retired}"
        );
    }
}

#[test]
fn dhcp_pack_is_absorbed_and_served_by_the_shell() {
    let pack = include_str!("../shell/ux/packs/dhcp.css");
    for selector in [
        ".dhcp-tablet",
        ".dhcp-info-banner",
        ".dhcp-list-item",
        ".reservation-slider",
        ".dhcp-banner--loading",
        "og color-literal → author-face token fold",
    ] {
        assert!(pack.contains(selector), "DHCP pack missing {selector}");
    }
    let render = include_str!("../shell/render.rs");
    assert!(render.contains("packs/dhcp.css"));
}

#[test]
fn dhcp_client_is_composed_into_served_crown_chrome() {
    let shell = render_crown_shell_for_session(Session::Admin);
    let chrome = crown_chrome_js();
    assert!(shell.contains("data-dhcp-tablet"));
    assert!(!shell.contains("__DHCP_CLIENT__"));
    assert!(chrome.contains("hydrateDhcp"));
    assert!(chrome.contains("/api/dhcp/reservations"));
    assert!(!chrome.contains("__DHCP_CLIENT__"));
}

#[test]
fn dhcp_client_declares_admin_family_and_hydrates_only_when_active_visible() {
    let client = [
        include_str!("../shell/dhcp-client.rs"),
        include_str!("../shell/document-3.rs"),
        include_str!("../shell/document-4.rs"),
        include_str!("../shell/document-4-tail.rs"),
    ]
    .join("\n");
    for required in [
        "hydrateDhcp()",
        "'/api/dhcp/leases'",
        "'/api/dhcp/reservations'",
        "'/api/dhcp/statistics'",
        "'/api/dhcp/pool-boundary'",
        "data-dhcp-pin",
        "data-dhcp-edit",
        "data-dhcp-remove",
        "viewportFamilyAdmitted('dhcp')",
        "authClass: 'admin'",
        "document.visibilityState !== 'visible'",
        "showCoronatioToast(successMessage, 'success')",
        "showCoronatioToast(failure?.message || 'DHCP request failed', 'error')",
        "openDhcpModal('Remove reservation'",
        "openDhcpModal('Edit reserved address'",
        "data-dhcp-boundary-min",
        "data-dhcp-boundary-max",
        "Delegated DHCP bindings survive pane swaps (Upload cure)",
        "document.body.addEventListener('input'",
        "document.addEventListener('click'",
        "}, true);",
    ] {
        assert!(client.contains(required), "DHCP client missing {required}");
    }
    assert!(!client.contains("setInterval(hydrateDhcp"));
    assert!(!client.contains("hydratePortals(); hydrateDhcp()"));
}
