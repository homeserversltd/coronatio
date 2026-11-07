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
    let Some(target) = (match path {
        "/api/dhcp/status" => Some("/api/v1/network/dhcp/status"),
        "/api/dhcp/leases" => Some("/api/v1/network/dhcp/leases"),
        "/api/dhcp/health" => Some("/api/v1/network/dhcp/health"),
        "/api/dhcp/statistics" => Some("/api/v1/network/dhcp/statistics"),
        "/api/dhcp/pool-boundary" => Some("/api/v1/network/dhcp/boundary"),
        _ => None,
    }) else {
        let historical_target = match path {
            "/api/dhcp/reservations" => Some("/api/v1/network/dhcp/reservations"),
            "/api/dhcp/config" => Some("/api/v1/network/dhcp"),
            _ => None,
        };
        return translation_debt_readback("GET", path, historical_target, None);
    };
    caduceus_http("GET", target)
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
    let session = session_projection_from_headers(&headers).await;
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
    target: &str,
    metadata: serde_json::Value,
) -> Response {
    if session_from_headers(headers) != Session::Admin {
        return dhcp_guest_refusal(path);
    }
    let readback = admin_fragment_caduceus_json_request(headers, method, target, metadata);
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
    if readback.status > 0 && readback.body != serde_json::json!({"error": "caduceus-mutation-refused"}) {
        let status = StatusCode::from_u16(readback.status)
            .ok()
            .filter(|status| status.is_client_error() || status.is_server_error())
            .unwrap_or(StatusCode::BAD_GATEWAY);
        return (status, Json(readback.body)).into_response();
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
        "/api/v1/network/dhcp/reservations",
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
    let encoded_id: String = url::form_urlencoded::byte_serialize(reservation_id.as_bytes()).collect();
    let target = format!("/api/v1/network/dhcp/reservations/{encoded_id}");
    dhcp_mutation_response(&headers, "PUT", &path, &target, metadata)
}

async fn dhcp_reservation_delete_route(
    Path(reservation_id): Path<String>,
    headers: axum::http::HeaderMap,
) -> Response {
    let path = format!("/api/dhcp/reservations/{reservation_id}");
    let encoded_id: String = url::form_urlencoded::byte_serialize(reservation_id.as_bytes()).collect();
    let target = format!("/api/v1/network/dhcp/reservations/{encoded_id}");
    dhcp_mutation_response(&headers, "DELETE", &path, &target, serde_json::json!({}))
}

async fn dhcp_pool_boundary_route(
    headers: axum::http::HeaderMap,
    payload: Option<Json<serde_json::Value>>,
) -> Response {
    dhcp_mutation_response(
        &headers,
        "POST",
        "/api/dhcp/pool-boundary",
        "/api/v1/network/dhcp/pool-boundary",
        payload
            .map(|Json(value)| value)
            .unwrap_or_else(|| serde_json::json!({})),
    )
}
