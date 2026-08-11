fn dhcp_guest_refusal(path: &str) -> Response {
    (
        StatusCode::UNAUTHORIZED,
        Json(serde_json::json!({
            "schema": "coronatio.dhcp.read.refusal.v1",
            "ok": false,
            "success": false,
            "path": path,
            "error": "admin-session-required",
            "firstMissingSignal": "admin-session-required"
        })),
    )
        .into_response()
}

fn dhcp_readback(_headers: &axum::http::HeaderMap, path: &str) -> CaduceusHttpReadback {
    match resolve_caduceus_door("GET", path) {
        Ok(door) => caduceus_http(&door.method, &door.path),
        Err(CaduceusDoorResolutionFailure::Unmapped) => mutation_refusal_readback(path, MutationRefusal { code: "coronatio-caduceus-door-unmapped".to_string(), status: 0 }),
        Err(CaduceusDoorResolutionFailure::Unavailable) => mutation_refusal_readback(path, MutationRefusal { code: "caduceus-doors-unavailable".to_string(), status: 0 }),
    }
}

fn strip_dhcp_identity(value: &serde_json::Value) -> serde_json::Value {
    match value {
        serde_json::Value::Object(fields) => serde_json::Value::Object(
            fields
                .iter()
                .filter(|(key, _)| {
                    !matches!(
                        key.as_str(),
                        "hostname"
                            | "hostName"
                            | "mac"
                            | "macAddress"
                            | "hw-address"
                            | "hwAddress"
                            | "ip"
                            | "ipAddress"
                            | "clientId"
                            | "reservationId"
                            | "leases"
                            | "reservations"
                    )
                })
                .map(|(key, value)| (key.clone(), strip_dhcp_identity(value)))
                .collect(),
        ),
        serde_json::Value::Array(values) => {
            serde_json::Value::Array(values.iter().map(strip_dhcp_identity).collect())
        }
        other => other.clone(),
    }
}

async fn dhcp_read_route(headers: axum::http::HeaderMap, uri: Uri) -> Response {
    let path = uri.path();
    let session = session_from_headers(&headers);
    if session == Session::Guest
        && matches!(
            path,
            "/api/dhcp/leases"
                | "/api/dhcp/reservations"
                | "/api/dhcp/config"
                | "/api/dhcp/pool-boundary"
        )
    {
        return dhcp_guest_refusal(path);
    }

    let readback = dhcp_readback(&headers, path);
    if !readback.ok {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({
                "schema": "coronatio.dhcp.read.error.v1",
                "ok": false,
                "success": false,
                "path": path,
                "authority": "Caduceus network dhcp",
                "caduceusStatus": readback.status,
                "firstMissingSignal": readback.first_missing_signal
            })),
        )
            .into_response();
    }

    let body = device_identity_payload(readback.body);
    let body = match session {
        Session::Admin => body,
        Session::Guest => strip_dhcp_identity(&body),
    };
    (StatusCode::OK, Json(body)).into_response()
}

fn dhcp_mutation_response(
    headers: &axum::http::HeaderMap,
    method: &str,
    path: &str,
    metadata: serde_json::Value,
) -> Response {
    let readback = mutation_staff_intent(
        &mutation_authority(),
        &headers,
        method,
        path,
        "network-control",
        metadata,
    );
    dhcp_mutation_result_response(method, path, readback)
}

fn dhcp_mutation_result_response(
    method: &str,
    path: &str,
    readback: CaduceusHttpReadback,
) -> Response {
    if readback.ok {
        return (StatusCode::OK, Json(device_identity_payload(readback.body))).into_response();
    }
    (
        mutation_response_status(&readback),
        Json(serde_json::json!({
            "schema": "coronatio.dhcp.mutation.error.v1",
            "ok": false,
            "success": false,
            "accepted": false,
            "method": method,
            "path": path,
            "authority": "Caduceus DHCP staff execution",
            "caduceusStatus": readback.status,
            "firstMissingSignal": readback.first_missing_signal
        })),
    )
        .into_response()
}

async fn dhcp_reservation_create_route(
    headers: axum::http::HeaderMap,
    payload: Option<Json<serde_json::Value>>,
) -> Response {
    dhcp_mutation_response(
        &headers,
        "POST",
        "/api/dhcp/reservations",
        payload
            .map(|Json(value)| value)
            .unwrap_or_else(|| serde_json::json!({})),
    )
}

async fn dhcp_reservation_update_route(
    Path(reservation_id): Path<String>,
    headers: axum::http::HeaderMap,
    payload: Option<Json<serde_json::Value>>,
) -> Response {
    let path = format!("/api/dhcp/reservations/{reservation_id}");
    let metadata = payload
        .map(|Json(value)| value)
        .unwrap_or_else(|| serde_json::json!({}));
    dhcp_mutation_response(&headers, "PUT", &path, metadata)
}

async fn dhcp_reservation_delete_route(
    Path(reservation_id): Path<String>,
    headers: axum::http::HeaderMap,
) -> Response {
    let path = format!("/api/dhcp/reservations/{reservation_id}");
    dhcp_mutation_response(&headers, "DELETE", &path, serde_json::json!({}))
}

async fn dhcp_pool_boundary_route(
    headers: axum::http::HeaderMap,
    payload: Option<Json<serde_json::Value>>,
) -> Response {
    dhcp_mutation_response(
        &headers,
        "POST",
        "/api/dhcp/pool-boundary",
        payload
            .map(|Json(value)| value)
            .unwrap_or_else(|| serde_json::json!({})),
    )
}
