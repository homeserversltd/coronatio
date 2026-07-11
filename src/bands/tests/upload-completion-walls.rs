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
