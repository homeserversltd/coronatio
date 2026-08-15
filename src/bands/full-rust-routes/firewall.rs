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

fn firewall_seated_read(caduceus_path: &str, metadata: serde_json::Value) -> CaduceusHttpReadback {
    match resolve_caduceus_door("POST", caduceus_path) {
        Ok(door) => caduceus_http_json(&door.method, &door.path, metadata, None),
        Err(CaduceusDoorResolutionFailure::Unmapped) => mutation_refusal_readback(
            caduceus_path,
            MutationRefusal {
                code: "coronatio-caduceus-door-unmapped".to_string(),
                status: 0,
            },
        ),
        Err(CaduceusDoorResolutionFailure::Unavailable) => mutation_refusal_readback(
            caduceus_path,
            MutationRefusal {
                code: "caduceus-doors-unavailable".to_string(),
                status: 0,
            },
        ),
    }
}

fn firewall_seated_mutation(
    headers: &axum::http::HeaderMap,
    crown_path: &str,
    caduceus_path: &str,
    metadata: serde_json::Value,
) -> CaduceusHttpReadback {
    caduceus_staff_transition_with_mapping(
        &mutation_authority(),
        headers,
        MutationActionTarget::caduceus("caduceus_staff.child_device", crown_path),
        "POST",
        caduceus_path,
        "child-device",
        metadata,
    )
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
    let path = "/api/firewall/observed";
    if let Some(refusal) = firewall_admin(&headers, path) {
        return refusal;
    }
    firewall_staff_response(
        firewall_seated_read("/api/admin/firewall/observed", serde_json::json!({})),
        path,
    )
}

async fn firewall_children_route(headers: axum::http::HeaderMap) -> Response {
    let path = "/api/firewall/children";
    if let Some(refusal) = firewall_admin(&headers, path) {
        return refusal;
    }
    firewall_staff_response(
        firewall_seated_read("/api/admin/firewall/list", serde_json::json!({})),
        path,
    )
}

async fn firewall_register_route(
    headers: axum::http::HeaderMap,
    payload: Option<Json<serde_json::Value>>,
) -> Response {
    let path = "/api/firewall/children";
    if let Some(refusal) = firewall_admin(&headers, path) {
        return refusal;
    }
    firewall_staff_response(
        firewall_seated_mutation(
            &headers,
            path,
            "/api/admin/firewall/register",
            payload
                .map(|Json(value)| value)
                .unwrap_or_else(|| serde_json::json!({})),
        ),
        path,
    )
}

async fn firewall_unregister_route(
    headers: axum::http::HeaderMap,
    Path(mac): Path<String>,
) -> Response {
    let path = format!("/api/firewall/children/{mac}");
    if let Some(refusal) = firewall_admin(&headers, &path) {
        return refusal;
    }
    firewall_staff_response(
        firewall_seated_mutation(
            &headers,
            &path,
            "/api/admin/firewall/unregister",
            serde_json::json!({"mac": mac}),
        ),
        &path,
    )
}

async fn firewall_whitelist_get_route(
    headers: axum::http::HeaderMap,
    Path(mac): Path<String>,
) -> Response {
    let path = format!("/api/firewall/children/{mac}/whitelist");
    if let Some(refusal) = firewall_admin(&headers, &path) {
        return refusal;
    }
    firewall_staff_response(
        firewall_seated_read(
            "/api/admin/firewall/whitelist-get",
            serde_json::json!({"mac": mac}),
        ),
        &path,
    )
}

async fn firewall_whitelist_set_route(
    headers: axum::http::HeaderMap,
    Path(mac): Path<String>,
    payload: Option<Json<serde_json::Value>>,
) -> Response {
    let path = format!("/api/firewall/children/{mac}/whitelist");
    if let Some(refusal) = firewall_admin(&headers, &path) {
        return refusal;
    }
    let mut metadata = payload
        .map(|Json(value)| value)
        .unwrap_or_else(|| serde_json::json!({}));
    if let Some(fields) = metadata.as_object_mut() {
        fields.insert("mac".to_string(), serde_json::json!(mac));
    }
    firewall_staff_response(
        firewall_seated_mutation(
            &headers,
            &path,
            "/api/admin/firewall/whitelist-set",
            metadata,
        ),
        &path,
    )
}
