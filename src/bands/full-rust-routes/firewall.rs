fn firewall_guest_refusal(path: &str) -> Response {
    (
        StatusCode::UNAUTHORIZED,
        Json(serde_json::json!({
            "schema": "coronatio.firewall.refusal.v1",
            "ok": false,
            "path": path,
            "error": "admin-session-required",
            "firstMissingSignal": "admin-session-required"
        })),
    )
        .into_response()
}

fn firewall_seated_read(caduceus_path: &str, body: serde_json::Value) -> CaduceusHttpReadback {
    caduceus_http_json("POST", caduceus_path, body, None)
}

fn firewall_payload(
    payload: Option<Json<serde_json::Value>>,
    crown_path: &str,
) -> Result<serde_json::Value, Response> {
    let mut metadata = payload
        .map(|Json(value)| value)
        .unwrap_or_else(|| serde_json::json!({}));
    let Some(fields) = metadata.as_object_mut() else {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "schema": "coronatio.firewall.refusal.v1",
                "ok": false,
                "path": crown_path,
                "error": "firewall-payload-invalid",
                "firstMissingSignal": "firewall-payload-invalid"
            })),
        )
            .into_response());
    };
    fields.remove("schema");
    Ok(metadata)
}

fn firewall_seated_mutation(
    headers: &axum::http::HeaderMap,
    caduceus_path: &str,
    method: &str,
    body: Option<serde_json::Value>,
) -> CaduceusHttpReadback {
    let authority = mutation_authority();
    let context = MutationRequestContext::from_headers(headers);
    match authority.authorize(&context) {
        Ok(attendance) => {
            match body {
                Some(body) => caduceus_http_json_with_attendance_and_document_timeout(
                    method,
                    caduceus_path,
                    body,
                    Some(&attendance.proof),
                    Some(&attendance.document),
                    std::time::Duration::from_secs(25),
                ),
                None => caduceus_http_with_attendance_and_document_timeout(
                    method,
                    caduceus_path,
                    Some(&attendance.proof),
                    Some(&attendance.document),
                    std::time::Duration::from_secs(25),
                ),
            }
        }
        Err(refusal) => mutation_refusal_readback(caduceus_path, refusal),
    }
}

fn firewall_staff_response(readback: CaduceusHttpReadback, path: &str) -> Response {
    if readback.ok {
        return (StatusCode::OK, Json(readback.body)).into_response();
    }
    if readback.body.get("ok").and_then(serde_json::Value::as_bool) == Some(false) {
        if let Some(signal) = readback.body.get("firstMissingSignal").and_then(serde_json::Value::as_str) {
            let status = if signal == "firewall-revision-conflict" {
                StatusCode::CONFLICT
            } else {
                StatusCode::UNPROCESSABLE_ENTITY
            };
            return (status, Json(readback.body)).into_response();
        }
    }
    if readback.status != 0 {
        let status = StatusCode::from_u16(readback.status).unwrap_or(StatusCode::BAD_GATEWAY);
        return (status, Json(readback.body)).into_response();
    }
    (
        StatusCode::SERVICE_UNAVAILABLE,
        Json(serde_json::json!({
            "schema": "coronatio.firewall.staff-unavailable.v1",
            "ok": false,
            "path": path,
            "message": "Child-device controls are unavailable because the seated Caduceus door did not answer.",
            "authority": "Caduceus staff child-device",
            "caduceusStatus": readback.status,
            "firstMissingSignal": readback.first_missing_signal
        })),
    )
        .into_response()
}

fn firewall_admin(headers: &axum::http::HeaderMap, path: &str) -> Option<Response> {
    (session_from_headers(headers) != Session::Admin).then(|| firewall_guest_refusal(path))
}

async fn firewall_observed_route(headers: axum::http::HeaderMap) -> Response {
    let crown_path = "/api/firewall/observed";
    if let Some(refusal) = firewall_admin(&headers, crown_path) {
        return refusal;
    }
    firewall_staff_response(
        firewall_seated_read("/api/v1/network/nftables-child-filter/list-the-devices-observed-on-the-lan-with-their-child-filter-policy", serde_json::json!({})),
        crown_path,
    )
}

async fn firewall_children_route(headers: axum::http::HeaderMap) -> Response {
    let crown_path = "/api/firewall/children";
    if let Some(refusal) = firewall_admin(&headers, crown_path) {
        return refusal;
    }
    firewall_staff_response(
        firewall_seated_read("/api/v1/network/nftables-child-filter/list-every-filtered-child-device-with-its-way-and-hostnames", serde_json::json!({})),
        crown_path,
    )
}

async fn firewall_register_route(
    headers: axum::http::HeaderMap,
    payload: Option<Json<serde_json::Value>>,
) -> Response {
    let crown_path = "/api/firewall/children";
    if let Some(refusal) = firewall_admin(&headers, crown_path) {
        return refusal;
    }
    let body = match firewall_payload(payload, crown_path) {
        Ok(body) => body,
        Err(refusal) => return refusal,
    };
    firewall_staff_response(
        firewall_seated_mutation(
            &headers,
            "/api/v1/network/nftables-child-filter/add-one-filtered-child-device-with-its-way-and-hostnames-by-its-mac",
            "POST",
            Some(body),
        ),
        crown_path,
    )
}

async fn firewall_way_route(
    headers: axum::http::HeaderMap,
    payload: Option<Json<serde_json::Value>>,
) -> Response {
    let crown_path = "/api/firewall/way";
    if let Some(refusal) = firewall_admin(&headers, crown_path) {
        return refusal;
    }
    let body = match firewall_payload(payload, crown_path) {
        Ok(body) => body,
        Err(refusal) => return refusal,
    };
    firewall_staff_response(
        firewall_seated_mutation(&headers, "/api/v1/network/nftables-child-filter/set-the-way-of-one-filtered-child-device-by-its-mac-to-blocked-or-allowlist", "POST", Some(body)),
        crown_path,
    )
}

async fn firewall_unregister_route(
    headers: axum::http::HeaderMap,
    Path(mac): Path<String>,
) -> Response {
    let crown_path = format!("/api/firewall/children/{mac}");
    if let Some(refusal) = firewall_admin(&headers, &crown_path) {
        return refusal;
    }
    firewall_staff_response(
        firewall_seated_mutation(&headers, "/api/v1/network/nftables-child-filter/remove-one-filtered-child-device-by-its-mac", "POST", Some(serde_json::json!({"mac": mac}))),
        &crown_path,
    )
}

async fn firewall_whitelist_get_route(
    headers: axum::http::HeaderMap,
    Path(mac): Path<String>,
) -> Response {
    let crown_path = format!("/api/firewall/children/{mac}/whitelist");
    if let Some(refusal) = firewall_admin(&headers, &crown_path) {
        return refusal;
    }
    firewall_staff_response(firewall_seated_read("/api/v1/network/nftables-child-filter/read-the-hostname-whitelist-of-one-filtered-child-device-by-its-mac", serde_json::json!({"mac": mac})), &crown_path)
}

async fn firewall_whitelist_set_route(
    headers: axum::http::HeaderMap,
    Path(mac): Path<String>,
    payload: Option<Json<serde_json::Value>>,
) -> Response {
    let crown_path = format!("/api/firewall/children/{mac}/whitelist");
    if let Some(refusal) = firewall_admin(&headers, &crown_path) {
        return refusal;
    }
    let payload = match firewall_payload(payload, &crown_path) {
        Ok(body) => body,
        Err(refusal) => return refusal,
    };
    let mut body = serde_json::json!({"mac": mac});
    if let Some(hostnames) = payload.get("hostnames") {
        body["hostnames"] = hostnames.clone();
    }
    if let Some(revision) = payload.get("revision").or_else(|| payload.get("expectedRevision")) {
        body["revision"] = revision.clone();
    }
    firewall_staff_response(
        firewall_seated_mutation(&headers, "/api/v1/network/nftables-child-filter/set-the-hostname-whitelist-of-one-filtered-child-device-by-its-mac-at-its-revision", "POST", Some(body)),
        &crown_path,
    )
}
