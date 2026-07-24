mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    static CADUCEUS_ENV_LOCK: std::sync::OnceLock<std::sync::Mutex<()>> =
        std::sync::OnceLock::new();
    static HX_EXEMPLAR_ENV_LOCK: std::sync::OnceLock<std::sync::Mutex<()>> =
        std::sync::OnceLock::new();

    fn test_tab_root(name: &str) -> std::path::PathBuf {
        let root = std::env::temp_dir().join(format!("coronatio-test-{name}-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&root).unwrap();
        root
    }

    fn successor_admin_request(mut request: Request<Body>) -> Request<Body> {
        request.headers_mut().insert("host", "home.arpa".parse().unwrap());
        request.headers_mut().insert("origin", "https://home.arpa".parse().unwrap());
        request.headers_mut().insert("x-caduceus-document", "test-document".parse().unwrap());
        request.headers_mut().insert("x-caduceus-attendance", "test-attendance".parse().unwrap());
        request
    }

    fn successor_session_request(mut request: Request<Body>, admin: bool) -> Request<Body> {
        request.headers_mut().insert("host", "home.arpa".parse().unwrap());
        request.headers_mut().insert("origin", "https://home.arpa".parse().unwrap());
        request.headers_mut().insert("x-caduceus-document", "test-document".parse().unwrap());
        if admin { request.headers_mut().insert("x-caduceus-attendance", "test-attendance".parse().unwrap()); }
        request
    }

    include!("tests/part-01.rs");

    include!("tests/part-03.rs");
    include!("tests/part-04.rs");
    include!("tests/part-05.rs");
    include!("tests/part-06.rs");
    include!("tests/starred-tab-route-walls.rs");
    include!("tests/hx-proof-walls.rs");
    include!("tests/iris-walls.rs");
    include!("tests/attendance-projection-walls.rs");
    include!("tests/pulse-walls.rs");
    include!("tests/stats-projection-walls.rs");
    include!("tests/stats-adoption-walls.rs");
    include!("tests/internet-status-projection-walls.rs");

    include!("tests/dhcp-read-walls.rs");
    include!("tests/dhcp-pane-walls.rs");

    // Legacy ticket/capability authority tests were retired with Slice D.
    include!("tests/test-tranch-05-upload-bedrock-walls.rs");
    include!("tests/star-guest-caduceus-walls.rs");
    include!("tests/hyalos-consumer-walls.rs");
    include!("tests/debug-emitter-walls.rs");

    include!("tests/portals-htmx-mirror-walls.rs");
    include!("tests/portals-currentness-walls.rs");

    include!("tests/portals-ui-wire-walls.rs");
    include!("tests/theme-net-author-face-walls.rs");
    include!("tests/indicator-infinite-infinite-walls.rs");
    include!("tests/immortal-floor-walls.rs");

}
