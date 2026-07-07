mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    include!("tests/part-01.rs");
    include!("tests/part-02.rs");
    include!("tests/part-03.rs");
    include!("tests/part-04.rs");
    include!("tests/part-05.rs");
    include!("tests/part-06.rs");
    include!("tests/hx-proof-walls.rs");
    include!("tests/iris-walls.rs");
    include!("tests/vis-002.rs");
    include!("tests/vis-003.rs");
    include!("tests/pulse-walls.rs");
    include!("tests/stats-projection-walls.rs");
    include!("tests/internet-status-projection-walls.rs");
    include!("tests/field-003-network-identity-walls.rs");
}
