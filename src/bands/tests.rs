#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;


    include!("tests/core-routes.rs");
    include!("tests/registry-stats.rs");
    include!("tests/topics-storage.rs");
    include!("tests/service-transactions.rs");
    include!("tests/helpers.rs");
}
