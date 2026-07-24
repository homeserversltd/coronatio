#[tokio::test]
async fn starred_tab_routes_reach_the_real_handlers_in_the_served_app() {
    let _hx_guard = HX_EXEMPLAR_ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
    let root = test_tab_root("starred-tab-routes");
    let config = root.join("homeserver.json");
    std::fs::write(
        &config,
        br#"{"tabs":{"starred":"stats","stats":{"config":{"isEnabled":true,"adminOnly":false},"visibility":{"tab":true}},"portals":{"config":{"isEnabled":true,"adminOnly":false},"visibility":{"tab":true}}}}"#,
    )
    .unwrap();
    std::env::set_var("CORONATIO_HOMESERVER_JSON", &config);
    let router = app(AppState { tab_root: Arc::new(root) });

    let get = router
        .clone()
        .oneshot(Request::builder().uri("/api/get_starred_tab").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(get.status(), StatusCode::OK);
    let get_body = String::from_utf8(axum::body::to_bytes(get.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
    assert!(get_body.contains("coronatio.starred-tab.response.v1"), "{get_body}");
    assert!(get_body.contains("\"starred_tab\":\"stats\""), "{get_body}");

    let set = router
        .oneshot(successor_admin_request(
            Request::builder()
                .method("POST")
                .uri("/api/set_starred_tab")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"tabName":"portals"}"#))
                .unwrap(),
        ))
        .await
        .unwrap();
    assert_eq!(set.status(), StatusCode::OK);
    let set_body = String::from_utf8(axum::body::to_bytes(set.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
    assert!(set_body.contains("data-tab-star=\"portals\""), "{set_body}");
    assert!(set_body.contains("aria-pressed=\"true\""), "{set_body}");
    assert!(!set_body.contains("coronatio.api.error.v1"), "{set_body}");
    std::env::remove_var("CORONATIO_HOMESERVER_JSON");
}
