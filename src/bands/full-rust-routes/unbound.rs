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

const DNS_BLOCKLIST_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(300);

async fn dns_caduceus_mutation_route(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
    payload: Option<Json<serde_json::Value>>,
) -> Response {
    let path = uri.path();
    if let Some(refusal) = mutation_context_refusal(&headers) {
        return dns_refusal(path, mutation_refusal_readback("/api/v1/network/dns", refusal));
    }
    let timeout = match path {
        "/api/v1/network/identity/device/add" => Some(std::time::Duration::from_secs(95)),
        "/api/v1/network/dns/device-name/create" | "/api/v1/network/dns/device-name/remove" | "/api/v1/network/identity/device/plan" => Some(std::time::Duration::from_secs(30)),
        "/api/v1/network/dns/blocklist/update" | "/api/v1/network/dns/blocklist/source/add" | "/api/v1/network/dns/blocklist/source/remove" => Some(DNS_BLOCKLIST_TIMEOUT),
        _ => None,
    };
    let readback = if let Some(timeout) = timeout {
        let target = path.to_owned();
        tokio::task::spawn_blocking(move || {
            if target == "/api/v1/network/dns/blocklist/update" {
                caduceus_actuate_with_timeout(&mutation_authority(), &headers, &target, timeout)
            } else {
                let body = payload.map(|Json(value)| value).unwrap_or_else(|| serde_json::json!({}));
                caduceus_actuate_json_with_timeout(&mutation_authority(), &headers, &target, body, timeout)
            }
        })
        .await
        .unwrap_or_else(|_| mutation_task_failure_readback(path))
    } else {
        caduceus_actuate_json(
            &mutation_authority(),
            &headers,
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
