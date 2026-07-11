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
