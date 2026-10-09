#[derive(serde::Deserialize)]
struct DnsRecordInput {
    name: String,
    address: String,
}

fn dns_refusal(path: &str, readback: CaduceusHttpReadback) -> Response {
    let status = if readback.status > 0 {
        StatusCode::from_u16(readback.status).ok()
    } else {
        None
    }
    .unwrap_or_else(|| mutation_response_status(&readback));
    (
        status,
        Json(serde_json::json!({
            "schema": "coronatio.unbound.refusal.v1",
            "ok": false,
            "success": false,
            "accepted": false,
            "path": path,
            "error": readback.first_missing_signal,
            "firstMissingSignal": readback.first_missing_signal,
        })),
    )
        .into_response()
}

fn dns_is_local_mutation_refusal(readback: &CaduceusHttpReadback) -> bool {
    !readback.ok && readback.body == serde_json::json!({"error": "caduceus-mutation-refused"})
}

fn dns_response(path: &str, readback: CaduceusHttpReadback) -> Response {
    if dns_is_local_mutation_refusal(&readback) {
        return dns_refusal(path, readback);
    }
    if readback.status > 0 {
        if let Ok(status) = StatusCode::from_u16(readback.status) {
            return (status, Json(readback.body)).into_response();
        }
    }
    dns_refusal(path, readback)
}

fn dns_intent(headers: &axum::http::HeaderMap, path: &str, intent: serde_json::Value) -> Response {
    if let Some(refusal) = mutation_context_refusal(headers) {
        return dns_refusal(path, mutation_refusal_readback("/api/v1/network/dns", refusal));
    }
    let readback = caduceus_actuate_json(
        &mutation_authority(),
        headers,
        MutationActionTarget::caduceus("caduceus.network.dns", "/api/v1/network/dns"),
        "/api/v1/network/dns",
        intent,
    );
    dns_response(path, readback)
}

async fn dns_caduceus_read_route(uri: axum::http::Uri) -> Response {
    let path = uri.path();
    let readback = match path {
        "/api/v1/network/dns/read" | "/api/v1/network/dns/resolver/status" => caduceus_http("GET", path),
        _ => translation_debt_readback("GET", path, Some(path), None),
    };
    dns_response(path, readback)
}

async fn dns_caduceus_mutation_route(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
    payload: Option<Json<serde_json::Value>>,
) -> Response {
    let path = uri.path();
    if let Some(refusal) = mutation_context_refusal(&headers) {
        return dns_refusal(path, mutation_refusal_readback("/api/v1/network/dns", refusal));
    }
    let mapping = MutationActionTarget::caduceus("caduceus.network.dns", path);
    let readback = if path == "/api/v1/network/dns/blocklist/update" {
        caduceus_actuate(&mutation_authority(), &headers, mapping, path)
    } else {
        caduceus_actuate_json(
            &mutation_authority(),
            &headers,
            mapping,
            path,
            payload.map(|Json(value)| value).unwrap_or_else(|| serde_json::json!({})),
        )
    };
    dns_response(path, readback)
}

async fn dns_records_get_route(headers: axum::http::HeaderMap) -> Response {
    dns_intent(&headers, "/api/dns/records", serde_json::json!({"action": "status"}))
}

async fn dns_records_status_post_route(headers: axum::http::HeaderMap) -> Response {
    dns_intent(
        &headers,
        "/api/dns/records/status",
        serde_json::json!({"action": "status"}),
    )
}

async fn dns_records_post_route(
    headers: axum::http::HeaderMap,
    payload: Option<Json<DnsRecordInput>>,
) -> Response {
    let Some(Json(record)) = payload else {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"ok": false, "error": "dns-record-required", "firstMissingSignal": "dns-record-required"})),
        ).into_response();
    };
    dns_intent(&headers, "/api/dns/records", serde_json::json!({
        "action": "ensure-local-data", "name": record.name, "address": record.address
    }))
}

async fn dns_records_delete_route(Path(name): Path<String>, headers: axum::http::HeaderMap) -> Response {
    dns_intent(&headers, &format!("/api/dns/records/{name}"), serde_json::json!({"action": "remove", "name": name}))
}
