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
}
