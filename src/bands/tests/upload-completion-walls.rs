#[test]
fn upload_config_roundtrip_and_blacklist_filter_use_homeserver_json() {
    let _guard = HX_EXEMPLAR_ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
    let root = test_tab_root("upload-config-roundtrip");
    let config = root.join("homeserver.json");
    std::fs::write(&config, r#"{"tabs":{"upload":{"data":{"blacklist":[],"default-directory":"/mnt/nas","isPinRequired":false}}}}"#).unwrap();
    std::env::set_var("CORONATIO_HOMESERVER_JSON", &config);
    std::env::set_var("CORONATIO_UPLOAD_ROOT", &root);
    std::fs::create_dir_all(root.join("allowed")).unwrap();
    std::fs::create_dir_all(root.join("blocked")).unwrap();
    update_upload_config("blacklist", serde_json::json!([root.join("blocked").display().to_string()])).unwrap();
    update_upload_config("default-directory", serde_json::json!("/mnt/nas/allowed")).unwrap();
    update_upload_config("isPinRequired", serde_json::json!(true)).unwrap();
    let value = upload_config_value();
    assert_eq!(upload_data(&value).unwrap()["default-directory"], "/mnt/nas/allowed");
    assert_eq!(upload_data(&value).unwrap()["isPinRequired"], true);
    let children = upload_immediate_children(&root, &upload_display_root(&root), &root);
    assert_eq!(children.iter().map(|entry| entry.name.as_str()).collect::<Vec<_>>(), vec!["allowed"]);
    std::env::remove_var("CORONATIO_HOMESERVER_JSON");
    std::env::remove_var("CORONATIO_UPLOAD_ROOT");
}

#[tokio::test]
async fn upload_default_directory_post_accepts_directory_and_returns_og_shape() {
    let _guard = HX_EXEMPLAR_ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
    let root = test_tab_root("upload-default-directory-post");
    let config = root.join("homeserver.json");
    std::fs::write(&config, r#"{"tabs":{"upload":{"data":{"default-directory":"/mnt/nas"}}}}"#).unwrap();
    std::env::set_var("CORONATIO_HOMESERVER_JSON", &config);
    let response = app(AppState { tab_root: Arc::new(root) })
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/upload/default-directory")
                .header("X-Admin-Token", authorize_test_admin_token())
                .header("content-type", "application/json")
                .body(Body::from(r#"{"directory":"/mnt/nas/media"}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body: serde_json::Value = serde_json::from_slice(&axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap()).unwrap();
    assert_eq!(body["success"], true);
    assert_eq!(body["directory"], "/mnt/nas/media");
    let stored: serde_json::Value = serde_json::from_slice(&std::fs::read(&config).unwrap()).unwrap();
    assert_eq!(stored["tabs"]["upload"]["data"]["default-directory"], "/mnt/nas/media");
    std::env::remove_var("CORONATIO_HOMESERVER_JSON");
}

#[tokio::test]
async fn upload_force_permissions_post_accepts_directory_as_caduceus_destination() {
    let _guard = CADUCEUS_ENV_LOCK.get_or_init(|| std::sync::Mutex::new(())).lock().unwrap();
    let body = serde_json::json!({"directory":"/mnt/nas/media"});
    assert_eq!(upload_force_permissions_destination(&body), "/mnt/nas/media");
    std::env::set_var("CADUCEUS_URL", "http://127.0.0.1:9");
    let response = app(AppState { tab_root: Arc::new(test_tab_root("upload-force-permissions-post")) })
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/upload/force-permissions")
                .header("X-Admin-Token", authorize_test_admin_token())
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    std::env::remove_var("CADUCEUS_URL");
}
