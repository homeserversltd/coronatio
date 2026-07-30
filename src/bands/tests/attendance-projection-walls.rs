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
fn browser_attendance_runtime_precedes_theme_iteration_and_has_one_owner() {
    let chrome = crown_chrome_js();
    let runtime = chrome.find("var coronatioAttendanceRuntimeKey").expect("attendance runtime");
    let theme_iteration = chrome.find("function renderThemeChoices()").expect("theme renderer");
    assert!(runtime < theme_iteration, "attendance owner must be created at script scope before theme callbacks execute");
    assert_eq!(chrome.matches("window.fetch = decoratedFetch").count(), 1);
    assert_eq!(chrome.matches("document.addEventListener('htmx:configRequest'").count(), 1);
    assert_eq!(chrome.matches("activityCensusInstallCount++").count(), 1);
    assert_eq!(chrome.matches("/api/v1/attendance/touch").count(), 1);
    assert!(!chrome.contains("let currentAttendance = null"));
    assert!(!chrome.contains("const documentIncarnation ="));

    let change_start = chrome.find("if (modalMode === 'change')").unwrap();
    let change_end = chrome[change_start..].find("if (modalMode === 'enter'").unwrap() + change_start;
    let change = &chrome[change_start..change_end];
    for forbidden in ["coronatioAttendanceRuntime.currentAttendance =", "headerState.isAdmin =", "setAdminMode(", "upgradeOpenStreams", "downgradeOpenStreams", "/api/v1/attendance/touch"] { assert!(!change.contains(forbidden), "PIN change altered surviving attendance posture through {forbidden}"); }
}
