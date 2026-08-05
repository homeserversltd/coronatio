fn device_identity_refusal(path: &str) -> Response {
    (StatusCode::UNAUTHORIZED, Json(serde_json::json!({
        "schema": "coronatio.network.identity.refusal.v1", "ok": false,
        "error": "admin-session-required", "firstMissingSignal": "admin-session-required", "path": path
    }))).into_response()
}

fn device_identity_admin(headers: &axum::http::HeaderMap, path: &str) -> Option<Response> {
    (session_from_headers(headers) != Session::Admin).then(|| device_identity_refusal(path))
}

// Caduceus recast routes carry successful results in the actuator envelope.
// Coronatio owns this crossing and returns only the stable client projection.
fn device_identity_payload(body: serde_json::Value) -> serde_json::Value {
    body.get("payload")
        .and_then(|payload| {
            payload
                .get("result")
                .cloned()
                .or_else(|| Some(payload.clone()))
        })
        .unwrap_or(body)
}

fn device_identity_read(
    path: &str,
    caduceus_path: &str,
    headers: &axum::http::HeaderMap,
) -> Response {
    if let Some(refusal) = device_identity_admin(headers, path) {
        return refusal;
    }
    let readback = caduceus_http("GET", caduceus_path);
    if readback.ok {
        return (StatusCode::OK, Json(device_identity_payload(readback.body))).into_response();
    }
    (StatusCode::SERVICE_UNAVAILABLE, Json(serde_json::json!({
        "schema": "coronatio.network.identity.read.error.v1", "ok": false, "path": path,
        "authority": "Caduceus network identity", "firstMissingSignal": readback.first_missing_signal
    }))).into_response()
}

async fn device_roster_route(headers: axum::http::HeaderMap) -> Response {
    device_identity_read(&"/api/network/device", "/api/v1/network/device", &headers)
}
async fn device_boundary_route(headers: axum::http::HeaderMap) -> Response {
    device_identity_read(
        "/api/network/dhcp/boundary",
        "/api/v1/network/dhcp/boundary",
        &headers,
    )
}
async fn device_leases_route(headers: axum::http::HeaderMap) -> Response {
    device_identity_read(
        "/api/network/dhcp/leases",
        "/api/v1/network/dhcp/leases",
        &headers,
    )
}
async fn device_reservations_route(headers: axum::http::HeaderMap) -> Response {
    device_identity_read(
        "/api/network/dhcp/reservations",
        "/api/v1/network/dhcp/reservations",
        &headers,
    )
}
async fn device_dns_read_route(headers: axum::http::HeaderMap) -> Response {
    device_identity_read(
        "/api/network/dns/read",
        "/api/v1/network/dns/read",
        &headers,
    )
}

async fn device_claim_route(
    headers: axum::http::HeaderMap,
    payload: Option<Json<serde_json::Value>>,
) -> Response {
    let path = "/api/network/device/claim";
    if let Some(refusal) = device_identity_admin(&headers, path) {
        return refusal;
    }
    // One identity binding is one Caduceus call: DHCP and DNS stay coordinated by staff.
    let readback = caduceus_http_json(
        "POST",
        "/api/v1/network/device/claim",
        payload
            .map(|Json(v)| v)
            .unwrap_or_else(|| serde_json::json!({})),
        None,
    );
    if readback.ok {
        return (StatusCode::OK, Json(device_identity_payload(readback.body))).into_response();
    }
    (mutation_response_status(&readback), Json(readback.body)).into_response()
}
