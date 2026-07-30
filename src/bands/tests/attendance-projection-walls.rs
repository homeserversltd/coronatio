#[test]
fn slice_d_uses_document_attendance_not_legacy_authority() {
    let access = include_str!("../caduceus-access.rs");
    let authority = include_str!("../mutation-authority.rs");
    let router = include_str!("../router-readback.rs");
    let runtime = include_str!("../runtime.rs");
    assert!(access.contains("/api/v1/attendance/open"));
    assert!(access.contains("/api/v1/attendance/validate"));
    assert!(access.contains("/api/v1/attendance/invalidate"));
    assert!(access.contains("\"documentId\":document"));
    assert!(access.contains("\"documentIncarnation\":document"));
    assert!(!access.contains("\"document\":document"));
    assert!(access.contains("body.get(\"documentId\")"));
    assert!(access.contains("body.get(\"documentIncarnation\")"));
    assert!(!access.contains("body.get(\"document\")"));
    assert!(access.contains("matches!(self,Self::Open)"));
    assert!(!access.contains("matches!(self,Self::Open|Self::Validate)"));
    assert!(access.contains("x-caduceus-document"));
    assert!(!access.contains("SessionTicket"));
    assert!(!access.contains("CapabilityTicket"));
    assert!(authority.contains("attendance_validate"));
    assert!(
        authority.contains("x-caduceus-attendance") || access.contains("x-caduceus-attendance")
    );
    assert!(!router.contains("legacy authority route"));
    assert!(runtime.contains("/api/v1/attendance/open"));
    assert!(runtime.contains("/api/v1/attendance/validate"));
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
    assert!(shell.contains("/api/v1/attendance/open"));
    assert!(shell.contains("upgradeOpenStreams"));
    assert!(shell.contains("downgradeOpenStreams"));
    assert!(shell.contains("setStreamMembership('stats', pulseStreamId, 'upgrade')"));
    assert!(shell.contains("setStreamMembership('core', coreStreamId, 'upgrade')"));
    assert!(shell.contains("/pulse/${action}?streamId=${encodeURIComponent(streamId)}"));
    assert!(shell.contains("headers.set('X-Caduceus-Attendance', currentAttendance)"));
    assert!(!shell.contains("attendance=${encodeURIComponent"));
    assert!(shell.contains("15 * 60 * 1000"));
    assert!(shell.contains("You have been disconnected due to inactivity."));
    assert!(!shell.contains("localStorage.setItem('coronatioAdminToken'"));
    assert!(!shell.contains(&["X-Admin", "Token"].concat()));
}
