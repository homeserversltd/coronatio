#[test]
fn upload_config_reads_homeserver_json_without_local_mutation() {
    let _guard = HX_EXEMPLAR_ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
    let root = test_tab_root("upload-config-roundtrip");
    let config = root.join("homeserver.json");
    let bytes = br#"{"tabs":{"upload":{"data":{"blacklist":["/mnt/nas/blocked"],"default-directory":"/mnt/nas/allowed","isPinRequired":true}}}}"#;
    std::fs::write(&config, bytes).unwrap();
    std::env::set_var("CORONATIO_HOMESERVER_JSON", &config);
    std::env::set_var("CORONATIO_UPLOAD_ROOT", &root);
    std::fs::create_dir_all(root.join("allowed")).unwrap();
    std::fs::create_dir_all(root.join("blocked")).unwrap();
    let value = upload_config_value();
    assert_eq!(upload_data(&value).unwrap()["default-directory"], "/mnt/nas/allowed");
    assert_eq!(upload_data(&value).unwrap()["isPinRequired"], true);
    assert!(upload_path_blacklisted("/mnt/nas/blocked"));
    assert_eq!(std::fs::read(&config).unwrap(), bytes);
    std::env::remove_var("CORONATIO_HOMESERVER_JSON");
    std::env::remove_var("CORONATIO_UPLOAD_ROOT");
}

#[tokio::test]
async fn upload_mutations_use_one_scoped_caduceus_config_actuation_and_never_write_local_json() {
    let _guard = HX_EXEMPLAR_ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
    let root = test_tab_root("upload-default-directory-post");
    let config = root.join("homeserver.json");
    let bytes = br#"{"tabs":{"upload":{"data":{"default-directory":"/mnt/nas"}}}}"#;
    std::fs::write(&config, bytes).unwrap();
    std::env::set_var("CORONATIO_HOMESERVER_JSON", &config);
    let router = app(AppState { tab_root: Arc::new(root) });
    let mark = crate::caduceus_access::test_fixture::mark();
    for (method, path, body, target) in [
        ("POST", "/api/upload/default-directory", r#"{"directory":"/mnt/nas/media"}"#, "tabs.upload.data.default-directory"),
        ("PUT", "/api/upload/blacklist/update", r#"{"blacklist":["/mnt/nas/blocked"]}"#, "tabs.upload.data.blacklist"),
        ("POST", "/api/upload/pin-required-status", r#"{"isPinRequired":true}"#, "tabs.upload.data.isPinRequired"),
    ] {
        let response = router.clone().oneshot(successor_admin_request(Request::builder().method(method).uri(path).header("content-type", "application/json").body(Body::from(body)).unwrap())).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK, "{method} {path}");
        let body: serde_json::Value = serde_json::from_slice(&axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap()).unwrap();
        assert_eq!(body["ok"], true, "{method} {path}: {body}");
        let records = crate::caduceus_access::test_fixture::records_since(mark);
        assert_eq!(records.iter().filter(|record| record.path == "/api/v1/config/set" && record.action.as_deref() == Some("coronatio.config.set") && record.target.as_deref() == Some(target)).count(), 1, "{records:?}");
    }
    assert_eq!(std::fs::read(&config).unwrap(), bytes);
    std::env::remove_var("CORONATIO_HOMESERVER_JSON");
}

#[tokio::test]
async fn upload_mutation_refusals_and_downstream_fault_leave_config_bytes_unchanged() {
    let _hx_guard = HX_EXEMPLAR_ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
    let _caduceus_guard = CADUCEUS_ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
    let root = test_tab_root("upload-mutation-refusal-no-write");
    let config = root.join("homeserver.json");
    let bytes = br#"{"tabs":{"upload":{"data":{"default-directory":"/mnt/nas"}}}}"#;
    std::fs::write(&config, bytes).unwrap();
    std::env::set_var("CORONATIO_HOMESERVER_JSON", &config);
    let router = app(AppState { tab_root: Arc::new(root) });
    let mark = crate::caduceus_access::test_fixture::mark();
    let missing = router.clone().oneshot(Request::builder().method("POST").uri("/api/upload/default-directory").header("content-type", "application/json").body(Body::from(r#"{"directory":"/mnt/nas/media"}"#)).unwrap()).await.unwrap();
    assert_eq!(missing.status(), StatusCode::FORBIDDEN);
    let missing_body = String::from_utf8(axum::body::to_bytes(missing.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
    assert!(missing_body.contains("caduceus-access-origin-refused"));
    assert!(crate::caduceus_access::test_fixture::records_since(mark).is_empty());
    assert_eq!(std::fs::read(&config).unwrap(), bytes);
    std::env::set_var("CADUCEUS_STAFF_SOCKET", guaranteed_absent_caduceus_socket());
    let fault = router.oneshot(successor_admin_request(Request::builder().method("POST").uri("/api/upload/default-directory").header("content-type", "application/json").body(Body::from(r#"{"directory":"/mnt/nas/media"}"#)).unwrap())).await.unwrap();
    assert_eq!(fault.status(), StatusCode::SERVICE_UNAVAILABLE);
    let fault_body = String::from_utf8(axum::body::to_bytes(fault.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
    assert!(fault_body.contains("caduceus-unreachable"));
    assert!(!fault_body.contains("\"ok\":true"));
    assert_eq!(std::fs::read(&config).unwrap(), bytes);
    std::env::remove_var("CADUCEUS_STAFF_SOCKET");
    std::env::remove_var("CORONATIO_HOMESERVER_JSON");
}

#[tokio::test]
async fn upload_force_permissions_post_accepts_directory_as_caduceus_destination() {
    let _guard = CADUCEUS_ENV_LOCK.get_or_init(|| std::sync::Mutex::new(())).lock().unwrap();
    let body = serde_json::json!({"directory":"/mnt/nas/media"});
    assert_eq!(upload_force_permissions_destination(&body), "/mnt/nas/media");
    std::env::set_var("CADUCEUS_STAFF_SOCKET", guaranteed_absent_caduceus_socket());
    let response = app(AppState { tab_root: Arc::new(test_tab_root("upload-force-permissions-post")) })
        .oneshot(
            successor_admin_request(
                Request::builder()
                    .method("POST")
                    .uri("/api/upload/force-permissions")
                    .header("content-type", "application/json")
                    .body(Body::from(body.to_string()))
                    .unwrap(),
            ),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    std::env::remove_var("CADUCEUS_STAFF_SOCKET");
}
