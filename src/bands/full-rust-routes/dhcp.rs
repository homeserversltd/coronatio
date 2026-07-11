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

fn dhcp_readback(path: &str) -> CaduceusHttpReadback {
    if path == "/api/dhcp/status" {
        return caduceus_http("GET", "/api/v1/network/dhcp/status");
    }
    caduceus_http_json(
        "POST",
        "/api/v1/staff/intent",
        serde_json::json!({
            "method": "GET",
            "route": path,
            "classification": "network-control",
            "metadata": {}
        }),
        None,
    )
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

async fn dhcp_read_route(
    headers: axum::http::HeaderMap,
    uri: Uri,
) -> Response {
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

    let readback = dhcp_readback(path);
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

    let body = match session {
        Session::Admin => readback.body,
        Session::Guest => strip_dhcp_identity(&readback.body),
    };
    (StatusCode::OK, Json(body)).into_response()
}

fn dhcp_mutation_response(method: &str, path: &str, metadata: serde_json::Value) -> Response {
    let readback = caduceus_http_json(
        "POST",
        "/api/v1/staff/intent",
        serde_json::json!({
            "method": method,
            "route": path,
            "classification": "network-control",
            "metadata": metadata
        }),
        None,
    );
    dhcp_mutation_result_response(method, path, readback)
}

fn dhcp_mutation_result_response(
    method: &str,
    path: &str,
    readback: CaduceusHttpReadback,
) -> Response {
    if readback.ok {
        return (StatusCode::OK, Json(readback.body)).into_response();
    }
    (
        StatusCode::SERVICE_UNAVAILABLE,
        Json(serde_json::json!({
            "schema": "coronatio.dhcp.mutation.error.v1",
            "ok": false,
            "success": false,
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
    if session_from_headers(&headers) != Session::Admin {
        return network_identity_mutation_refusal_response("POST", "/api/dhcp/reservations");
    }
    dhcp_mutation_response(
        "POST",
        "/api/dhcp/reservations",
        payload.map(|Json(value)| value).unwrap_or_else(|| serde_json::json!({})),
    )
}

async fn dhcp_reservation_update_route(
    Path(reservation_id): Path<String>,
    headers: axum::http::HeaderMap,
    payload: Option<Json<serde_json::Value>>,
) -> Response {
    let path = format!("/api/dhcp/reservations/{reservation_id}");
    if session_from_headers(&headers) != Session::Admin {
        return network_identity_mutation_refusal_response("PUT", &path);
    }
    let mut metadata = payload.map(|Json(value)| value).unwrap_or_else(|| serde_json::json!({}));
    if let Some(fields) = metadata.as_object_mut() {
        fields.insert("reservationId".to_string(), serde_json::json!(reservation_id));
    }
    dhcp_mutation_response("PUT", &path, metadata)
}

async fn dhcp_reservation_delete_route(
    Path(reservation_id): Path<String>,
    headers: axum::http::HeaderMap,
) -> Response {
    let path = format!("/api/dhcp/reservations/{reservation_id}");
    if session_from_headers(&headers) != Session::Admin {
        return network_identity_mutation_refusal_response("DELETE", &path);
    }
    dhcp_mutation_response(
        "DELETE",
        &path,
        serde_json::json!({"reservationId": reservation_id}),
    )
}

async fn dhcp_pool_boundary_route(
    headers: axum::http::HeaderMap,
    payload: Option<Json<serde_json::Value>>,
) -> Response {
    if session_from_headers(&headers) != Session::Admin {
        return network_identity_mutation_refusal_response("POST", "/api/dhcp/pool-boundary");
    }
    dhcp_mutation_response(
        "POST",
        "/api/dhcp/pool-boundary",
        payload.map(|Json(value)| value).unwrap_or_else(|| serde_json::json!({})),
    )
}
