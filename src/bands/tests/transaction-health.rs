    #[tokio::test]
    async fn health_route_types_exact_source_and_build_sha() {
        let response = health_route().await.into_response();
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let value: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(value["schema"], "coronatio.health.v1");
        assert_eq!(value["service"], "coronatio");
        assert_eq!(value["source_sha"], CORONATIO_SOURCE_SHA);
        assert_eq!(value["build_sha"], CORONATIO_BUILD_SHA);
    }

    #[test]
    fn health_payload_requires_exact_explicit_identity() {
        let good = "a".repeat(40);
        let (status, payload) = health_response_for(&good, &good);
        assert_eq!(status, StatusCode::OK);
        assert_eq!(payload["ok"], true);
        let bad = "b".repeat(40);
        for (source, build) in [("", ""), ("A", "A"), (&good, bad.as_str())] {
            let (status, payload) = health_response_for(source, build);
            assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
            assert_eq!(payload["ok"], false);
        }
    }
