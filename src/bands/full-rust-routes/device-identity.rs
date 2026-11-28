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

fn device_identity_read(path: &str, target: Option<&str>, headers: &axum::http::HeaderMap) -> Response {
    if let Some(refusal) = device_identity_admin(headers, path) {
        return refusal;
    }
    let verbatim = matches!(path, "/api/network/dhcp/boundary" | "/api/network/dns/read");
    let readback = match target {
        Some(target) if path == "/api/network/dhcp/reservations" => caduceus_http_json_with_attendance_and_document(
            "POST", target, serde_json::json!({}), attendance_from_headers(headers).as_ref(),
            document_incarnation_from_headers(headers).as_deref(),
        ),
        Some(target) if path == "/api/network/dhcp/leases" => caduceus_http_json("POST", target, serde_json::json!({}), None),
        Some(target) if verbatim => caduceus_http_json("POST", target, serde_json::json!({}), None),
        Some(target) => caduceus_http("GET", target),
        None => translation_debt_readback(
            "GET",
            path,
            match path {
                "/api/network/device" => Some("/api/v1/network/device"),
                "/api/network/dhcp/reservations" => Some("/api/v1/network/dhcp/reservations"),
                _ => None,
            },
            None,
        ),
    };
    if readback.ok {
        let payload = if verbatim {
            readback.body.get("result").cloned().unwrap_or(readback.body)
        } else {
            device_identity_payload(readback.body)
        };
        let payload = if path == "/api/network/dhcp/boundary" {
            payload
                .as_array()
                .filter(|entries| entries.len() == 1)
                .and_then(|entries| entries.first())
                .filter(|entry| entry.is_object())
                .cloned()
                .unwrap_or(payload)
        } else {
            payload
        };
        return (StatusCode::OK, Json(payload)).into_response();
    }
    (StatusCode::SERVICE_UNAVAILABLE, Json(serde_json::json!({
        "schema": "coronatio.network.identity.read.error.v1", "ok": false, "path": path,
        "authority": "Caduceus network identity", "firstMissingSignal": readback.first_missing_signal
    }))).into_response()
}

async fn device_roster_route(headers: axum::http::HeaderMap) -> Response {
    device_identity_read("/api/network/device", Some("/api/v1/network/device"), &headers)
}
async fn device_boundary_route(headers: axum::http::HeaderMap) -> Response {
    device_identity_read("/api/network/dhcp/boundary", Some("/api/v1/network/kea-dhcp/read-the-pool-boundary-of-the-kea-dhcp4-subnet"), &headers)
}
async fn device_leases_route(headers: axum::http::HeaderMap) -> Response {
    device_identity_read("/api/network/dhcp/leases", Some("/api/v1/network/kea-dhcp/list-the-dhcp-leases-from-the-kea-lease-file"), &headers)
}
async fn device_reservations_route(headers: axum::http::HeaderMap) -> Response {
    device_identity_read("/api/network/dhcp/reservations", Some("/api/v1/network/kea-dhcp/list-the-dhcp-reservations-from-the-kea-configuration"), &headers)
}
async fn device_dns_read_route(headers: axum::http::HeaderMap) -> Response {
    device_identity_read("/api/network/dns/read", Some("/api/v1/network/unbound-dns-records/list-every-local-dns-record-with-where-it-is-declared"), &headers)
}

async fn device_claim_route(
    headers: axum::http::HeaderMap,
    payload: Option<Json<serde_json::Value>>,
) -> Response {
    let path = "/api/network/device/claim";
    let direct_path = "/api/v1/network/device/claim";
    if let Some(refusal) = device_identity_admin(&headers, path) {
        return refusal;
    }
    let readback = caduceus_actuate_json(
        &mutation_authority(),
        &headers,
        direct_path,
        payload
            .map(|Json(v)| v)
            .unwrap_or_else(|| serde_json::json!({})),
    );
    if readback.ok {
        return (StatusCode::OK, Json(device_identity_payload(readback.body))).into_response();
    }
    (mutation_response_status(&readback), Json(readback.body)).into_response()
}
