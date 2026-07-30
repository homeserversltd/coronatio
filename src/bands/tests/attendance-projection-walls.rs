#[tokio::test]
async fn slice_d_uses_document_attendance_not_legacy_authority() {
    let access = include_str!("../caduceus-access.rs");
    let authority = include_str!("../mutation-authority.rs");
    let router = include_str!("../router-readback.rs");
    let runtime = include_str!("../runtime.rs");
    assert!(access.contains("/api/v1/attendance/open"));
    assert!(access.contains("/api/v1/attendance/validate"));
    assert!(access.contains("/api/v1/attendance/touch"));
    assert!(access.contains("/api/v1/attendance/change-pin"));
    assert!(access.contains("/api/v1/attendance/invalidate"));
    assert!(access.contains("\"documentId\":document"));
    assert!(access.contains("\"documentIncarnation\":document"));
    assert!(!access.contains("\"document\":document"));
    assert!(access.contains("body.get(\"documentId\")"));
    assert!(access.contains("body.get(\"documentIncarnation\")"));
    assert!(!access.contains("body.get(\"document\")"));
    assert!(access.contains("b.is_ascii_alphabetic()"));
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
    assert!(runtime.contains("/api/v1/attendance/touch"));
    assert!(runtime.contains("/api/v1/attendance/change-pin"));
    let router = app(AppState { tab_root: Arc::new(test_tab_root("attendance-change-pin")) });
    let request = successor_admin_request(Request::builder().method("POST").uri("/api/v1/attendance/change-pin").header("content-type", "application/json").body(Body::from(r#"{"currentPin":"2468","newPin":"9753"}"#)).unwrap());
    let response = router.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = serde_json::from_slice(&axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap()).unwrap();
    assert_eq!(body["schema"], "coronatio.caduceus.attendance.projection.v1");
    assert_eq!(body["ok"], true);
    let guest = successor_session_request(Request::builder().method("POST").uri("/api/v1/attendance/change-pin").header("content-type", "application/json").body(Body::from(r#"{"currentPin":"2468","newPin":"9753"}"#)).unwrap(), false);
    assert_eq!(router.oneshot(guest).await.unwrap().status(), StatusCode::UNAUTHORIZED);
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
    assert!(shell.contains("/api/v1/attendance/change-pin"));
    assert!(shell.contains("const ATTENDANCE_TOUCH_THROTTLE_MS = 60 * 1000"));
    assert_eq!(shell.matches("/api/v1/attendance/touch").count(), 1);
    assert!(shell.contains("!currentAttendance || inactivityHeadless"));
    assert!(shell.contains("document.addEventListener(type, recordEligibleActivity"));
    assert!(shell.contains("upgradeOpenStreams"));
    assert!(shell.contains("downgradeOpenStreams"));
    assert!(shell.contains("setStreamMembership('stats', pulseStreamId, 'upgrade')"));
    assert!(shell.contains("setStreamMembership('core', coreStreamId, 'upgrade')"));
    assert!(shell.contains("/pulse/${action}?streamId=${encodeURIComponent(streamId)}"));
    assert!(shell.contains("headers.set('X-Caduceus-Attendance', currentAttendance)"));
    assert!(shell.contains("htmx:configRequest"));
    assert!(shell.contains("await fetch('/api/v1/attendance/invalidate'"));
    assert!(shell.contains("'keydown', 'input'"));
    assert!(!shell.contains("attendance=${encodeURIComponent"));
    assert!(shell.contains("15 * 60 * 1000"));
    assert!(shell.contains("You have been disconnected due to inactivity."));
    assert!(!shell.contains("localStorage.setItem('coronatioAdminToken'"));
    assert!(!shell.contains(&["X-Admin", "Token"].concat()));
    for outcome in ["Please fill in all fields", "New PINs do not match", "Failed to change PIN", "PIN changed successfully"] { assert!(shell.contains(outcome), "missing OG PIN-change outcome: {outcome}"); }
    let change_start = shell.find("if (modalMode === 'change')").unwrap();
    let change_end = shell[change_start..].find("if (modalMode === 'enter'").unwrap() + change_start;
    let change = &shell[change_start..change_end];
    for forbidden in ["currentAttendance =", "headerState.isAdmin =", "setAdminMode(", "upgradeOpenStreams", "downgradeOpenStreams", "/api/v1/attendance/touch"] { assert!(!change.contains(forbidden), "PIN change altered surviving attendance posture through {forbidden}"); }
}
