#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct HomeserverReadGuestResponse { schema: &'static str, ok: bool, success: bool, status: &'static str, first_missing_signal: &'static str }

fn homeserver_read_response(headers: &axum::http::HeaderMap, method: &str, path: &str) -> Response {
    if path == "/api/status/power/usage" || path == "/status/power/usage" {
        return power_usage_response(method, path);
    }

    match session_from_headers(headers) {
        Session::Admin => (
            StatusCode::OK,
            Json(serde_json::json!({
                "schema": "coronatio.homeserver.route.read.v1",
                "ok": true,
                "success": true,
                "method": method,
                "path": path,
                "family": homeserver_route_family(path),
                "status": "rust-route",
                "authority": "Coronatio Rust route",
                "firstMissingSignal": "none"
            })),
        ).into_response(),
        Session::Guest => (StatusCode::OK, Json(HomeserverReadGuestResponse {
            schema: "coronatio.homeserver.route.read.guest.v1", ok: true, success: true, status: "rust-route", first_missing_signal: "none",
        })).into_response(),
    }
}

