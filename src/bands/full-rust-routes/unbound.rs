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
        "/api/v1/network/dns/read" => caduceus_http("GET", path),
        "/api/v1/network/dns/resolver/status" => caduceus_http_json(
            "POST",
            "/api/v1/network/unbound-resolver/status-the-unbound-resolver-upstreams-dot-adblock-and-unit-state",
            serde_json::json!({}),
            None,
        ),
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
    let mut body = payload.map(|Json(value)| value).unwrap_or_else(|| serde_json::json!({}));
    let target = match path {
        "/api/v1/network/dns/device-name/create" => "/api/v1/network/unbound-dns-device-name/add-one-device-name-by-its-hostname-and-ip",
        "/api/v1/network/dns/device-name/remove" => "/api/v1/network/unbound-dns-device-name/remove-one-device-name-by-its-hostname-and-ip",
        "/api/v1/network/identity/device/plan" => "/api/v1/network/lan-device-identity/verify-whether-a-device-identity-would-take-by-its-mac-hostname-and-ip-without-writing",
        "/api/v1/network/dns/blocklist/source/add" => "/api/v1/network/unbound-dns-blocklist/add-one-blocklist-source-by-its-url",
        "/api/v1/network/dns/blocklist/source/remove" => "/api/v1/network/unbound-dns-blocklist/remove-one-blocklist-source-by-its-url",
        "/api/v1/network/dns/upstream" => "/api/v1/network/unbound-resolver/set-the-unbound-upstream-resolvers-by-preset-or-custom-list-and-dns-over-tls",
        "/api/v1/network/dns/adblock" => {
            let target = match body.get("enabled").and_then(serde_json::Value::as_bool) {
                Some(true) => "/api/v1/network/unbound-resolver/set-the-unbound-adblock-blocklist-include-on",
                Some(false) => "/api/v1/network/unbound-resolver/set-the-unbound-adblock-blocklist-include-off",
                None => return dns_refusal(path, mutation_refusal_readback(path, MutationRefusal {
                    code: "resolver-adblock-enabled-invalid".to_string(),
                    status: 400,
                })),
            };
            body = serde_json::json!({});
            target
        }
        _ => path,
    };
    let readback = if let Some(timeout) = timeout {
        let target = target.to_owned();
        tokio::task::spawn_blocking(move || {
            if target == "/api/v1/network/dns/blocklist/update" {
                caduceus_actuate_with_timeout(&mutation_authority(), &headers, &target, timeout)
            } else {
                caduceus_actuate_json_with_timeout(&mutation_authority(), &headers, &target, body, timeout)
            }
        })
        .await
        .unwrap_or_else(|_| mutation_task_failure_readback(path))
    } else {
        caduceus_actuate_json(&mutation_authority(), &headers, target, body)
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
