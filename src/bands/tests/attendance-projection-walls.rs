#[test]
fn slice_d_uses_document_attendance_not_legacy_authority() {
    let access = include_str!("../caduceus-access.rs");
    let authority = include_str!("../mutation-authority.rs");
    let router = include_str!("../router-readback.rs");
    let runtime = include_str!("../runtime.rs");
    assert!(access.contains("attendance/open"));
    assert!(access.contains("attendance/validate"));
    assert!(access.contains("attendance/invalidate"));
    assert!(access.contains("x-caduceus-document"));
    assert!(!access.contains("SessionTicket"));
    assert!(!access.contains("CapabilityTicket"));
    assert!(!access.contains("caduceus_session"));
    assert!(!access.contains("capability.mint"));
    assert!(authority.contains("attendance_validate"));
    assert!(authority.contains("x-caduceus-attendance") || access.contains("x-caduceus-attendance"));
    assert!(!authority.contains("capability_mint"));
    assert!(!router.contains("/api/session/"));
    assert!(runtime.contains("/api/attendance/open"));
    assert!(runtime.contains("/api/attendance/validate"));
}

#[test]
fn visibility_projection_shape_is_tab_and_elements() {
    let routes = include_str!("../routes.rs");
    let fragments = include_str!("../crown-law/element-fragments.rs");
    assert!(routes.contains("tabs.{tab}.visibility.tab"));
    assert!(routes.contains("visibility.elements"));
    assert!(fragments.contains("tabs.{tab}.visibility.elements.{element}"));
}

#[test]
fn browser_attendance_is_memory_only_and_inactivity_is_exact() {
    let shell = include_str!("../shell/document-3.rs");
    assert!(shell.contains("let currentAttendance = null"));
    assert!(shell.contains("/api/attendance/open"));
    assert!(shell.contains("15 * 60 * 1000"));
    assert!(shell.contains("You have been disconnected due to inactivity."));
    assert!(!shell.contains("/api/session/"));
    assert!(!shell.contains("localStorage.setItem('coronatioAdminToken'"));
    assert!(!shell.contains("X-Admin-Token"));
}
