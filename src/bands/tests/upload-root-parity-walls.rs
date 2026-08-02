#[tokio::test(flavor = "current_thread")]
async fn upload_root_viewport_lists_quarry_filtered_children_without_repeating_nas_root() {
    use std::os::unix::fs::PermissionsExt;

    let _guard = HX_EXEMPLAR_ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
    let fixture = test_tab_root("upload-root-parity");
    let root = fixture.join("nas");
    let media = root.join("media");
    let films = media.join("films");
    let unreadable_trash = root.join(".Trash-0");
    let readable_trash = root.join(".Trash-1000");
    let blacklisted = root.join("blocked");
    for path in [&films, &unreadable_trash, &readable_trash, &blacklisted] {
        std::fs::create_dir_all(path).unwrap();
    }
    std::fs::set_permissions(&unreadable_trash, std::fs::Permissions::from_mode(0o000)).unwrap();
    let config = fixture.join("homeserver.json");
    std::fs::write(
        &config,
        serde_json::json!({
            "tabs": {"upload": {"data": {
                "blacklist": [blacklisted.display().to_string()],
                "default-directory": root.display().to_string()
            }}}
        }).to_string(),
    ).unwrap();
    std::env::set_var("CORONATIO_UPLOAD_ROOT", &root);
    std::env::set_var("CORONATIO_HOMESERVER_JSON", &config);
    let router = app(AppState { tab_root: Arc::new(test_tab_root("upload-root-parity-app")) });

    let response = router.clone().oneshot(Request::builder().uri("/admit/upload").body(Body::empty()).unwrap()).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let root_view = String::from_utf8(axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
    assert_eq!(root_view.matches(">nas</button>").count(), 1, "the current NAS directory belongs in the breadcrumb exactly once: {root_view}");
    let tree = root_view.split(r#"data-upload-tree role="tree">"#).nth(1).unwrap().split("</div></div><div class=\"file-upload-section\"").next().unwrap();
    assert!(!tree.contains(&format!(r#"data-directory-path="{}""#, root.display())), "the NAS root must not be emitted as its own tree child: {tree}");
    assert!(tree.contains(r#"class="entry-name">media</span>"#), "root children remain visible: {tree}");
    assert!(tree.contains(&format!(r#"data-upload-root-path value="{}""#, root.display())), "the renderer preserves the actual tree root for breadcrumb synchronization: {tree}");
    assert!(tree.contains(r#"class="entry-name">.Trash-1000</span>"#), "readable dot directories are not hidden by name: {tree}");
    assert!(!tree.contains(".Trash-0"), "unreadable quarry-filtered directories stay absent: {tree}");
    assert!(!tree.contains("blocked"), "configured blacklist filtering stays general: {tree}");

    let nested_uri = format!(
        "/admit/upload/tree?path={}&depth=0&selected={}&expanded={}",
        upload_query_escape(&films.display().to_string()),
        upload_query_escape(&films.display().to_string()),
        upload_query_escape(&media.display().to_string()),
    );
    let response = router.oneshot(Request::builder().uri(nested_uri).body(Body::empty()).unwrap()).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let nested = String::from_utf8(axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
    assert!(nested.contains(&format!(r#"data-directory-path="{}" role="treeitem" aria-selected="true""#, films.display())), "nested selection remains navigable: {nested}");
    assert!(nested.contains(&format!(r#"name="selected" data-upload-current-path value="{}""#, films.display())), "selected path remains available to breadcrumb synchronization: {nested}");
    assert!(nested.contains(r#"class="entry-name">films</span>"#), "expanded nested child remains rendered: {nested}");

    std::fs::set_permissions(&unreadable_trash, std::fs::Permissions::from_mode(0o700)).unwrap();
    std::env::remove_var("CORONATIO_HOMESERVER_JSON");
    std::env::remove_var("CORONATIO_UPLOAD_ROOT");
}
