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

fn firewall_seated_read(method: &str, caduceus_path: &str) -> CaduceusHttpReadback {
    caduceus_http(method, caduceus_path)
}

fn firewall_payload(
    payload: Option<Json<serde_json::Value>>,
    schema: &str,
    path_mac: Option<&str>,
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
    fields.insert("schema".to_string(), serde_json::json!(schema));
    if let Some(mac) = path_mac {
        fields.insert("mac".to_string(), serde_json::json!(mac));
    }
    Ok(metadata)
}

fn firewall_seated_mutation(
    headers: &axum::http::HeaderMap,
    caduceus_path: &str,
    method: &str,
    body: Option<serde_json::Value>,
) -> CaduceusHttpReadback {
    let authority = mutation_authority();
    let mapping = MutationActionTarget::caduceus("caduceus_staff.child_device", caduceus_path);
    let context = mapping.request_context(headers);
    match authority.authorize(&context, mapping) {
        Ok(attendance) => {
            let readback = match body {
                Some(body) => caduceus_http_json_with_attendance_and_document(
                    method,
                    caduceus_path,
                    body,
                    Some(&attendance.proof),
                    Some(&attendance.document),
                ),
                None => caduceus_http_with_attendance_and_document(
                    method,
                    caduceus_path,
                    Some(&attendance.proof),
                    Some(&attendance.document),
                ),
            };
            invalidate_scoped_attendance(&authority, &attendance, readback)
        }
        Err(refusal) => mutation_refusal_readback(caduceus_path, refusal),
    }
}

fn firewall_staff_response(readback: CaduceusHttpReadback, path: &str) -> Response {
    if readback.ok {
        return (StatusCode::OK, Json(readback.body)).into_response();
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
        firewall_seated_read("GET", "/api/v1/network/firewall/observed"),
        crown_path,
    )
}

async fn firewall_children_route(headers: axum::http::HeaderMap) -> Response {
    let crown_path = "/api/firewall/children";
    if let Some(refusal) = firewall_admin(&headers, crown_path) {
        return refusal;
    }
    firewall_staff_response(
        firewall_seated_read("GET", "/api/v1/network/firewall/children"),
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
    let body = match firewall_payload(
        payload,
        "caduceus.network.firewall.child.v1",
        None,
        crown_path,
    ) {
        Ok(body) => body,
        Err(refusal) => return refusal,
    };
    firewall_staff_response(
        firewall_seated_mutation(
            &headers,
            "/api/v1/network/firewall/children",
            "POST",
            Some(body),
        ),
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
    let caduceus_path = format!("/api/v1/network/firewall/children/{mac}");
    firewall_staff_response(
        firewall_seated_mutation(&headers, &caduceus_path, "DELETE", None),
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
    let caduceus_path = format!("/api/v1/network/firewall/children/{mac}/whitelist");
    firewall_staff_response(firewall_seated_read("GET", &caduceus_path), &crown_path)
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
    let body = match firewall_payload(
        payload,
        "caduceus.network.firewall.whitelist.v1",
        Some(&mac),
        &crown_path,
    ) {
        Ok(body) => body,
        Err(refusal) => return refusal,
    };
    let caduceus_path = format!("/api/v1/network/firewall/children/{mac}/whitelist");
    firewall_staff_response(
        firewall_seated_mutation(&headers, &caduceus_path, "PUT", Some(body)),
        &crown_path,
    )
}
