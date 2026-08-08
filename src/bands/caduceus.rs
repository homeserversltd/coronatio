async fn caduceus_status_route() -> impl IntoResponse {
    let health = caduceus_http("GET", "/api/v1/health");
    let update = caduceus_http("GET", "/api/v1/update/status");
    let staff = caduceus_http("GET", "/api/v1/staff/status");
    let ok = health.ok && update.ok && staff.ok;
    let first_missing_signal = if ok {
        "none".to_string()
    } else if !health.ok {
        health.first_missing_signal.clone()
    } else if !update.ok {
        update.first_missing_signal.clone()
    } else {
        staff.first_missing_signal.clone()
    };
    (
        if ok {
            StatusCode::OK
        } else {
            StatusCode::SERVICE_UNAVAILABLE
        },
        Json(serde_json::json!({
            "schema": "coronatio.caduceus.status.v1",
            "ok": ok,
            "caduceusBase": caduceus_safe_base_readback(),
            "health": health,
            "update": update,
            "staff": staff,
            "firstMissingSignal": first_missing_signal
        })),
    )
}

async fn caduceus_update_check_route(headers: axum::http::HeaderMap) -> impl IntoResponse {
    caduceus_mutation_route(&headers, "update_check", "/api/v1/update/check", "update check", "local")
}

async fn caduceus_update_now_route(headers: axum::http::HeaderMap) -> impl IntoResponse {
    caduceus_mutation_route(&headers, "update_now", "/api/v1/update/now", "update now", "local")
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct KeymanDoorRequest {
    #[serde(default)]
    target: Option<String>,
    #[serde(default)]
    device: Option<String>,
    #[serde(default)]
    strategy: Option<String>,
    #[serde(default)]
    password: Option<String>,
    #[serde(default, alias = "current_password")]
    current_password: Option<String>,
    #[serde(default, alias = "old_password")]
    old_password: Option<String>,
    #[serde(default, alias = "new_password")]
    new_password: Option<String>,
    #[serde(default)]
    planned: bool,
}

fn keyman_text(value: Option<String>) -> Option<String> {
    value.filter(|value| !value.trim().is_empty()).map(|value| value.trim().to_string())
}

fn redact_keyman_receipt(value: serde_json::Value) -> serde_json::Value {
    match value {
        serde_json::Value::Array(values) => serde_json::Value::Array(values.into_iter().map(redact_keyman_receipt).collect()),
        serde_json::Value::Object(values) => {
            let values = values.into_iter().filter_map(|(key, value)| {
                let sensitive = ["password", "secret", "token", "credential", "key_material"]
                    .iter()
                    .any(|needle| key.to_ascii_lowercase().contains(needle));
                (!sensitive).then(|| (key, redact_keyman_receipt(value)))
            }).collect();
            serde_json::Value::Object(values)
        }
        value => value,
    }
}

fn keyman_response(readback: CaduceusHttpReadback) -> (StatusCode, Json<serde_json::Value>) {
    let status = mutation_response_status(&readback);
    let ok = readback.ok;
    let first_missing_signal = readback.first_missing_signal.clone();
    let receipt = redact_keyman_receipt(readback.body);
    let receipt_family = receipt.get("receiptFamily").or_else(|| receipt.get("receipt_family")).cloned();
    (
        status,
        Json(serde_json::json!({
            "schema": "coronatio.caduceus.keyman.v1",
            "ok": ok,
            "receipt": receipt,
            "receiptFamily": receipt_family,
            "firstMissingSignal": first_missing_signal,
        })),
    )
}

fn keyman_input_error(signal: &str) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::BAD_REQUEST,
        Json(serde_json::json!({
            "schema": "coronatio.caduceus.keyman.v1",
            "ok": false,
            "firstMissingSignal": signal,
        })),
    )
}

async fn caduceus_keyman_create_key_route(headers: axum::http::HeaderMap, Json(request): Json<KeymanDoorRequest>) -> impl IntoResponse {
    let (Some(target), Some(strategy), Some(password)) = (keyman_text(request.target), keyman_text(request.strategy), request.password.filter(|value| !value.is_empty())) else {
        return keyman_input_error("keyman-create-fields-required");
    };
    keyman_response(caduceus_actuate_json(
        &mutation_authority(), &headers,
        MutationActionTarget::caduceus("keyman.create-key", "/api/v1/keyman/create-key"),
        "/api/v1/keyman/create-key",
        serde_json::json!({"target": target, "strategy": strategy, "password": password, "planned": request.planned}),
    ))
}

async fn caduceus_keyman_update_key_route(headers: axum::http::HeaderMap, Json(request): Json<KeymanDoorRequest>) -> impl IntoResponse {
    let (Some(device), Some(strategy), Some(current_password)) = (keyman_text(request.device), keyman_text(request.strategy), request.current_password.filter(|value| !value.is_empty())) else {
        return keyman_input_error("keyman-update-fields-required");
    };
    keyman_response(caduceus_actuate_json(
        &mutation_authority(), &headers,
        MutationActionTarget::caduceus("keyman.update-key", "/api/v1/keyman/update-key"),
        "/api/v1/keyman/update-key",
        serde_json::json!({"device": device, "strategy": strategy, "current_password": current_password, "planned": request.planned}),
    ))
}

async fn caduceus_keyman_admin_password_route(headers: axum::http::HeaderMap, Json(request): Json<KeymanDoorRequest>) -> impl IntoResponse {
    let (Some(old_password), Some(new_password)) = (request.old_password.filter(|value| !value.is_empty()), request.new_password.filter(|value| !value.is_empty())) else {
        return keyman_input_error("keyman-admin-password-fields-required");
    };
    keyman_response(caduceus_actuate_json(
        &mutation_authority(), &headers,
        MutationActionTarget::caduceus("keyman.admin-password", "/api/v1/keyman/admin-password"),
        "/api/v1/keyman/admin-password",
        serde_json::json!({"old_password": old_password, "new_password": new_password, "planned": request.planned}),
    ))
}

async fn caduceus_keyman_key_status_route(headers: axum::http::HeaderMap, Json(request): Json<KeymanDoorRequest>) -> impl IntoResponse {
    keyman_response(caduceus_actuate_json(
        &mutation_authority(), &headers,
        MutationActionTarget::caduceus("keyman.key-status", "/api/v1/keyman/key-status"),
        "/api/v1/keyman/key-status",
        serde_json::json!({"planned": request.planned}),
    ))
}

async fn caduceus_receipts_latest_route() -> impl IntoResponse {
    let readback = caduceus_http("GET", "/api/v1/receipts/latest");
    (
        if readback.ok {
            StatusCode::OK
        } else {
            StatusCode::SERVICE_UNAVAILABLE
        },
        Json(serde_json::json!({
            "schema": "coronatio.caduceus.receipts.latest.v1",
            "ok": readback.ok,
            "readback": readback,
            "firstMissingSignal": readback.first_missing_signal
        })),
    )
}

fn caduceus_mutation_readback(headers: &axum::http::HeaderMap, path: &str, action: &str, target: &str) -> CaduceusHttpReadback {
    caduceus_actuate(
        &mutation_authority(),
        &headers,
        MutationActionTarget::caduceus(action, target),
        path,
    )
}

fn caduceus_mutation_route(headers: &axum::http::HeaderMap, route: &str, path: &str, action: &str, target: &str) -> (StatusCode, Json<serde_json::Value>) {
    let readback = caduceus_mutation_readback(headers, path, action, target);
    (
        mutation_response_status(&readback),
        Json(serde_json::json!({
            "schema": "coronatio.caduceus.mutation.v1",
            "ok": readback.ok,
            "accepted": readback.ok,
            "route": route,
            "readback": readback,
            "firstMissingSignal": readback.first_missing_signal
        })),
    )
}

fn caduceus_config_set(headers: &axum::http::HeaderMap, path: &str, value: serde_json::Value) -> CaduceusHttpReadback {
    mutation_config_set(
        &mutation_authority(),
        &headers,
        path,
        value,
    )
}

// IRIS T01/T02 permits a guest to choose an already guest-visible regular tab.
// This is deliberately separate from caduceus_config_set: every other config
// mutation remains bound to the Coronatio attendance authority.
fn caduceus_guest_star_set(value: serde_json::Value) -> CaduceusHttpReadback {
    caduceus_http_json(
        "POST",
        "/api/v1/config/set",
        serde_json::json!({"path": "tabs.starred", "value": value}),
        None,
    )
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct CaduceusHttpReadback {
    ok: bool,
    status: u16,
    path: String,
    body: serde_json::Value,
    first_missing_signal: String,
}

fn caduceus_base() -> String {
    #[cfg(test)]
    {
        return env::var("CADUCEUS_BASE_URL")
            .or_else(|_| env::var("CADUCEUS_URL"))
            .unwrap_or_else(|_| crate::caduceus_access::test_fixture::base());
    }
    #[cfg(not(test))]
    env::var("CADUCEUS_BASE_URL")
        .or_else(|_| env::var("CADUCEUS_URL"))
        .unwrap_or_else(|_| "http://127.0.0.1:3014".to_string())
}

fn caduceus_authority() -> Option<String> {
    caduceus_authority_from_base(&caduceus_base())
}

fn caduceus_authority_from_base(base: &str) -> Option<String> {
    let authority = base.strip_prefix("http://")?;
    if authority.contains(['/', '?', '#', '@']) {
        return None;
    }
    let address = authority.parse::<SocketAddr>().ok()?;
    address.ip().is_loopback().then(|| address.to_string())
}

fn caduceus_safe_base_readback() -> &'static str {
    if caduceus_authority().is_some() { "caduceus-loopback" } else { "caduceus-loopback-required" }
}

fn caduceus_loopback_refusal(path: &str) -> CaduceusHttpReadback {
    CaduceusHttpReadback {
        ok: false,
        status: 0,
        path: path.to_string(),
        body: serde_json::json!({"error": "caduceus-loopback-required"}),
        first_missing_signal: "caduceus-loopback-required".to_string(),
    }
}

fn caduceus_http(method: &str, path: &str) -> CaduceusHttpReadback {
    caduceus_http_with_attendance(method, path, None)
}

fn caduceus_http_with_attendance(method: &str, path: &str, attendance: Option<&crate::caduceus_access::AttendanceProof>) -> CaduceusHttpReadback {
    caduceus_http_with_attendance_and_document(method, path, attendance, None)
}

fn caduceus_http_with_attendance_and_document(method: &str, path: &str, attendance: Option<&crate::caduceus_access::AttendanceProof>, document: Option<&str>) -> CaduceusHttpReadback {
    let Some(authority) = caduceus_authority() else {
        return caduceus_loopback_refusal(path);
    };
    #[cfg(test)]
    if authority == "127.0.0.1:9" {
        return CaduceusHttpReadback { ok: true, status: 200, path: path.to_string(), body: serde_json::json!({"ok": true}), first_missing_signal: "none".to_string() };
    }
    let mut stream = match TcpStream::connect(&authority) {
        Ok(stream) => stream,
        Err(_err) => {
            return CaduceusHttpReadback {
                ok: false,
                status: 0,
                path: path.to_string(),
                body: serde_json::json!({"error": "caduceus-upstream-failed"}),
                first_missing_signal: "caduceus-unreachable".to_string(),
            };
        }
    };
    let _ = stream.set_read_timeout(Some(Duration::from_secs(4)));
    let _ = stream.set_write_timeout(Some(Duration::from_secs(4)));
    let attendance_header = attendance
            .map(|proof| format!("x-caduceus-attendance: {}\r\n", proof.expose()))
        .unwrap_or_default();
    let document_header = document
        .map(|value| format!("x-caduceus-document: {value}\r\n"))
        .unwrap_or_default();
    let request = format!(
        "{method} {path} HTTP/1.1\r\nHost: {authority}\r\nConnection: close\r\n{document_header}{attendance_header}Content-Length: 0\r\n\r\n"
    );
    if let Err(_err) = stream.write_all(request.as_bytes()) {
        return CaduceusHttpReadback {
            ok: false,
            status: 0,
            path: path.to_string(),
            body: serde_json::json!({"error": "caduceus-upstream-failed"}),
            first_missing_signal: "caduceus-write-failed".to_string(),
        };
    }
    let mut response = String::new();
    if let Err(_err) = stream.read_to_string(&mut response) {
        return CaduceusHttpReadback {
            ok: false,
            status: 0,
            path: path.to_string(),
            body: serde_json::json!({"error": "caduceus-upstream-failed"}),
            first_missing_signal: "caduceus-read-failed".to_string(),
        };
    }
    let (head, body_text) = response
        .split_once("\r\n\r\n")
        .unwrap_or(("", response.as_str()));
    let status = head
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .and_then(|raw| raw.parse::<u16>().ok())
        .unwrap_or(0);
    let body =
        serde_json::from_str(body_text).unwrap_or_else(|_| serde_json::json!({"raw": body_text}));
    let body_ok = body
        .get("ok")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(status < 400);
    let first_missing_signal = body
        .get("firstMissingSignal")
        .or_else(|| body.get("first_missing_signal"))
        .and_then(serde_json::Value::as_str)
        .unwrap_or(if status < 400 && body_ok {
            "none"
        } else {
            "caduceus-http-not-ok"
        })
        .to_string();
    CaduceusHttpReadback {
        ok: status < 400 && body_ok,
        status,
        path: path.to_string(),
        body,
        first_missing_signal,
    }
}

fn caduceus_http_json(
    method: &str,
    path: &str,
    body: serde_json::Value,
    attendance: Option<&crate::caduceus_access::AttendanceProof>,
) -> CaduceusHttpReadback {
    caduceus_http_json_with_attendance(method, path, body, attendance)
}

fn caduceus_http_json_with_attendance(method: &str, path: &str, body: serde_json::Value, attendance: Option<&crate::caduceus_access::AttendanceProof>) -> CaduceusHttpReadback {
    caduceus_http_json_with_attendance_and_document(method, path, body, attendance, None)
}

fn caduceus_http_json_with_attendance_and_document(
    method: &str,
    path: &str,
    body: serde_json::Value,
    attendance: Option<&crate::caduceus_access::AttendanceProof>,
    document: Option<&str>,
) -> CaduceusHttpReadback {
    let Some(authority) = caduceus_authority() else {
        return caduceus_loopback_refusal(path);
    };
    #[cfg(test)]
    if authority == "127.0.0.1:9" {
        return CaduceusHttpReadback { ok: true, status: 200, path: path.to_string(), body: serde_json::json!({"ok": true}), first_missing_signal: "none".to_string() };
    }
    let mut stream = match TcpStream::connect(&authority) {
        Ok(stream) => stream,
        Err(_err) => {
            return CaduceusHttpReadback {
                ok: false,
                status: 0,
                path: path.to_string(),
                body: serde_json::json!({"error": "caduceus-upstream-failed"}),
                first_missing_signal: "caduceus-unreachable".to_string(),
            };
        }
    };
    let _ = stream.set_read_timeout(Some(Duration::from_secs(4)));
    let _ = stream.set_write_timeout(Some(Duration::from_secs(4)));
    let body_text = serde_json::to_string(&body).unwrap_or_else(|_| "{}".to_string());
    let attendance_header = attendance
            .map(|proof| format!("x-caduceus-attendance: {}\r\n", proof.expose()))
        .unwrap_or_default();
    let document_header = document
        .map(|value| format!("x-caduceus-document: {value}\r\n"))
        .unwrap_or_default();
    let request = format!(
        "{method} {path} HTTP/1.1\r\nHost: {authority}\r\nConnection: close\r\nContent-Type: application/json\r\n{document_header}{attendance_header}Content-Length: {}\r\n\r\n{}",
        body_text.len(), body_text
    );
    if let Err(_err) = stream.write_all(request.as_bytes()) {
        return CaduceusHttpReadback {
            ok: false,
            status: 0,
            path: path.to_string(),
            body: serde_json::json!({"error": "caduceus-upstream-failed"}),
            first_missing_signal: "caduceus-write-failed".to_string(),
        };
    }
    let mut response = String::new();
    if let Err(_err) = stream.read_to_string(&mut response) {
        return CaduceusHttpReadback {
            ok: false,
            status: 0,
            path: path.to_string(),
            body: serde_json::json!({"error": "caduceus-upstream-failed"}),
            first_missing_signal: "caduceus-read-failed".to_string(),
        };
    }
    parse_caduceus_response(path, &response)
}

fn parse_caduceus_response(path: &str, response: &str) -> CaduceusHttpReadback {
    let (head, body_text) = response
        .split_once("\r\n\r\n")
        .unwrap_or(("", response));
    let status = head
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .and_then(|raw| raw.parse::<u16>().ok())
        .unwrap_or(0);
    let body = serde_json::from_str(body_text).unwrap_or_else(|_| serde_json::json!({"raw": body_text}));
    let body_ok = body
        .get("ok")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(status < 400);
    let first_missing_signal = body
        .get("firstMissingSignal")
        .or_else(|| body.get("first_missing_signal"))
        .and_then(serde_json::Value::as_str)
        .unwrap_or(if status < 400 && body_ok {
            "none"
        } else {
            "caduceus-http-not-ok"
        })
        .to_string();
    CaduceusHttpReadback {
        ok: status < 400 && body_ok,
        status,
        path: path.to_string(),
        body,
        first_missing_signal,
    }
}


fn hyalos_tail_path(kind: Option<&str>, count: usize) -> String {
    let count = count.clamp(1, 1000);
    match kind {
        Some(kind) => format!("/api/v1/hyalos/tail?kind={kind}&count={count}"),
        None => format!("/api/v1/hyalos/tail?count={count}"),
    }
}

fn hyalos_tail_readback(kind: Option<&str>, count: usize) -> CaduceusHttpReadback {
    caduceus_http("GET", &hyalos_tail_path(kind, count))
}

fn hyalos_upload_history_message_visible(message: &str) -> bool {
    let normalized = message.to_ascii_lowercase();
    !normalized.contains("[error]")
        && !normalized.contains("failed to")
        && !normalized.contains("[system]")
}

fn hyalos_event_is_upload(event: &serde_json::Value) -> bool {
    event.get("kind").and_then(serde_json::Value::as_str) == Some("upload")
        || event.get("organ").and_then(serde_json::Value::as_str) == Some("file-ingress")
}

fn hyalos_upload_history_from_tail(readback: &CaduceusHttpReadback) -> Vec<String> {
    readback
        .body
        .get("events")
        .and_then(serde_json::Value::as_array)
        .map(|events| {
            events
                .iter()
                .filter(|event| hyalos_event_is_upload(event))
                .filter_map(|event| event.get("message").and_then(serde_json::Value::as_str))
                .filter(|message| hyalos_upload_history_message_visible(message))
                .map(str::to_string)
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect()
        })
        .unwrap_or_default()
}

fn hyalos_channel_clear_refusal_response(schema: &str, path: &str) -> Response {
    (
        StatusCode::GONE,
        Json(serde_json::json!({
            "schema": schema,
            "ok": false,
            "success": false,
            "accepted": false,
            "error": "hyalos-channel-append-only",
            "message": "Hyalos channel is append-only; log truncation is not permitted.",
            "path": path,
            "authority": "Coronatio Hyalos consumer",
            "firstMissingSignal": "hyalos-channel-append-only"
        })),
    )
        .into_response()
}


#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CrownDebugEmitRequest {
    kind: String,
    event: String,
    #[serde(default, alias = "correlationId")]
    correlation_id: Option<String>,
    attributes: Option<serde_json::Value>,
}

fn debug_key_sensitive(key: &str) -> bool {
    let key = key.to_ascii_lowercase();
    key.contains("token")
        || key.contains("pin")
        || key.contains("password")
        || key.contains("secret")
        || key.contains("capability")
        || key.contains("connection")
        || key.contains("body")
        || key.contains("payload")
        || key == "headers"
        || key.contains("localstorage")
        || key.contains("dom")
        || key.contains("snapshot")
        || key.contains("source")
}

fn debug_safe_kebab(value: &str) -> bool {
    let bytes = value.as_bytes();
    !bytes.is_empty()
        && bytes.len() <= 80
        && bytes.iter().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || *b == b'-')
        && !bytes.starts_with(b"-")
        && !bytes.ends_with(b"-")
        && !value.contains("--")
}

fn trim_debug_value(value: serde_json::Value, depth: usize) -> serde_json::Value {
    if depth > 4 {
        return serde_json::json!("[trimmed]");
    }
    match value {
        serde_json::Value::String(mut text) => {
            if text.len() > 512 {
                text.truncate(512);
                text.push('…');
            }
            serde_json::Value::String(text)
        }
        serde_json::Value::Array(items) => serde_json::Value::Array(
            items
                .into_iter()
                .take(32)
                .map(|item| trim_debug_value(item, depth + 1))
                .collect(),
        ),
        serde_json::Value::Object(map) => {
            let mut out = serde_json::Map::new();
            for (key, value) in map.into_iter().take(48) {
                if debug_key_sensitive(&key) {
                    continue;
                }
                out.insert(key, trim_debug_value(value, depth + 1));
            }
            serde_json::Value::Object(out)
        }
        other => other,
    }
}

fn caduceus_hyalos_reflect_debug(request: CrownDebugEmitRequest) -> CaduceusHttpReadback {
    if !debug_safe_kebab(&request.kind) {
        return CaduceusHttpReadback {
            ok: false,
            status: 400,
            path: "/api/v1/hyalos/reflect".to_string(),
            body: serde_json::json!({"error": "debug-kind-not-kebab-case"}),
            first_missing_signal: "debug-kind-not-kebab-case".to_string(),
        };
    }
    let event = request.event.trim().chars().take(120).collect::<String>();
    if event.is_empty() {
        return CaduceusHttpReadback {
            ok: false,
            status: 400,
            path: "/api/v1/hyalos/reflect".to_string(),
            body: serde_json::json!({"error": "debug-event-missing"}),
            first_missing_signal: "debug-event-missing".to_string(),
        };
    }
    let attributes_redacted = trim_debug_value(request.attributes.unwrap_or_else(|| serde_json::json!({})), 0);
    let correlation_id = request
        .correlation_id
        .or_else(|| attributes_redacted.get("correlationId").and_then(serde_json::Value::as_str).map(str::to_string))
        .or_else(|| attributes_redacted.get("bootId").and_then(serde_json::Value::as_str).map(str::to_string))
        .or_else(|| attributes_redacted.get("runId").and_then(serde_json::Value::as_str).map(str::to_string));
    caduceus_http_json(
        "POST",
        "/api/v1/hyalos/reflect",
        serde_json::json!({
            "organ": "coronatio",
            "kind": request.kind,
            "level": "debug",
            "message": event,
            "correlation_id": correlation_id,
            "attributes_redacted": attributes_redacted,
        }),
        None,
    )
}

#[derive(Debug, Clone)]
struct AdminMutationResult {
    action: String,
    title: String,
    message: String,
    ok: bool,
    first_missing_signal: String,
}


fn admin_membrane_refusal_fragment(surface: &str, first_missing_signal: &str) -> String {
    format!(
        r#"<div class="update-status-container error" data-admin-membrane-refusal="true" data-og-affordance="toast-mapped-to-result-strip"><strong>Enter Admin Mode</strong><span>{}</span><code>{}</code></div>"#,
        html_escape(surface),
        html_escape(first_missing_signal),
    )
}

fn admin_html_fragment_response(status: StatusCode, body: String) -> Response {
    let mut response = (status, Html(body)).into_response();
    response.headers_mut().insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    response.headers_mut().insert(header::CONTENT_SECURITY_POLICY, HeaderValue::from_static(CROWN_CONTENT_SECURITY_POLICY));
    response
}

fn admin_toggle_target(toggle_id: &str) -> Option<(&'static str, &'static str)> {
    match toggle_id {
        "ssh-password-authentication" => Some(("SSH Password Authentication", "/api/admin/ssh/toggle")),
        "ssh-service" => Some(("SSH Service", "/api/admin/ssh/service")),
        "samba-file-sharing" => Some(("Samba File Sharing", "/api/admin/samba/service")),
        _ => None,
    }
}

fn admin_service_status_target(toggle_id: &str) -> Option<&'static str> {
    match toggle_id {
        "ssh-password-authentication" => Some("/api/admin/ssh/status"),
        "ssh-service" => Some("/api/admin/ssh/service/status"),
        "samba-file-sharing" => Some("/api/admin/samba/status"),
        _ => None,
    }
}

fn admin_action_target(action_id: &str) -> Option<(&'static str, &'static str, &'static str, bool)> {
    match action_id {
        "hard-drive-test" => Some(("Hard Drive Test", "POST", "/api/admin/hard-drive-test/start", true)),
        "update" => Some(("Update", "POST", "/api/v1/update/now", true)),
        "rotate-capability-key" => Some(("Rotate Capability Key", "POST", "/usr/local/sbin/caduceus-keyman-rotate-capability", true)),
        "restart" => Some(("Restart", "POST", "/api/admin/system/restart", true)),
        "shutdown" => Some(("Shutdown", "POST", "/api/admin/system/shutdown", true)),
        "restart-website" => Some(("Restart Website", "POST", "/api/admin/services/hard-reset", true)),
        "view-logs" => Some(("View Logs", "GET", "/api/admin/logs/homeserver", false)),
        _ => None,
    }
}

fn admin_staff_intent(headers: &axum::http::HeaderMap, method: &str, path: &str, classification: &str) -> CaduceusHttpReadback {
    mutation_staff_intent(
        &mutation_authority(),
        &headers,
        method,
        path,
        classification,
        serde_json::json!({}),
    )
}

const ADMIN_LOG_PAGE_LIMIT: usize = 100;

fn admin_log_pagination(uri: &Uri) -> (usize, usize) {
    let mut offset = 0;
    let mut limit = ADMIN_LOG_PAGE_LIMIT;
    for pair in uri.query().unwrap_or_default().split('&') {
        let Some((key, value)) = pair.split_once('=') else { continue; };
        match key {
            "offset" => offset = value.parse::<usize>().unwrap_or(0),
            "limit" => limit = value.parse::<usize>().unwrap_or(ADMIN_LOG_PAGE_LIMIT).clamp(1, 5000),
            _ => {}
        }
    }
    (offset, limit)
}

fn admin_logs_modal_fragment(readback: &CaduceusHttpReadback, offset: usize, limit: usize, notice: Option<&str>) -> String {
    let lines = readback.body.get("lines").and_then(serde_json::Value::as_array);
    let total = readback.body.get("total_lines").and_then(serde_json::Value::as_u64).unwrap_or(0) as usize;
    let returned = lines.map_or(0, Vec::len);
    let previous = offset.saturating_sub(limit);
    let next = offset.saturating_add(returned);
    let previous_disabled = if offset == 0 { " disabled" } else { "" };
    let next_disabled = if returned == 0 || next >= total { " disabled" } else { "" };
    let content = if readback.ok {
        if returned == 0 {
            "<p data-admin-log-empty>No appliance log lines are available.</p>".to_string()
        } else {
            let lines = lines.unwrap_or(&Vec::new()).iter().filter_map(serde_json::Value::as_str).map(html_escape).collect::<Vec<_>>().join("");
            format!(r#"<pre class="log-frame" data-admin-log-lines>{lines}</pre>"#)
        }
    } else {
        format!(
            r#"<p class="error" data-admin-log-error>Could not read appliance logs: <code>{}</code></p>"#,
            html_escape(&readback.first_missing_signal),
        )
    };
    let notice = notice.map(|value| format!(r#"<p class="success" data-admin-log-notice>{}</p>"#, html_escape(value))).unwrap_or_default();
    format!(
        r#"<section class="modal modal-window admin-log-modal" data-admin-log-modal data-admin-action-result-fragment="view-logs" data-admin-action-route="/api/admin/logs/homeserver" role="dialog" aria-label="Appliance logs"><h2>Appliance Logs</h2>{notice}<div class="modal-toolbar"><span>Newest first · {returned} shown of {total}</span></div><div class="modal-body">{content}</div><div class="modal-pager"><button type="button" class="secondary" hx-get="/admit/admin/action/view-logs?offset={previous}&amp;limit={limit}" hx-target="closest [data-admin-log-modal]" hx-swap="outerHTML"{previous_disabled}>Previous</button><button type="button" class="secondary" hx-get="/admit/admin/action/view-logs?offset={next}&amp;limit={limit}" hx-target="closest [data-admin-log-modal]" hx-swap="outerHTML"{next_disabled}>Next</button><button type="button" class="ui-button ui-button--danger" hx-post="/admit/admin/action/view-logs-clear?offset=0&amp;limit={limit}" hx-target="closest [data-admin-log-modal]" hx-swap="outerHTML" hx-confirm="Clear the appliance log now?">Clear</button></div></section>"#,
    )
}

async fn admin_logs_fragment_route(headers: axum::http::HeaderMap, uri: Uri) -> Response {
    let (offset, limit) = admin_log_pagination(&uri);
    let path = format!("/api/admin/logs/homeserver?offset={offset}&limit={limit}");
    let readback = admin_fragment_caduceus_request(&headers, "GET", &path);
    log_admin_action_admission(&headers, "/admit/admin/action/view-logs", &readback, StatusCode::OK);
    admin_html_fragment_response(StatusCode::OK, admin_logs_modal_fragment(&readback, offset, limit, None))
}

async fn admin_logs_clear_fragment_route(headers: axum::http::HeaderMap, uri: Uri) -> Response {
    let (_, limit) = admin_log_pagination(&uri);
    let clear = admin_fragment_caduceus_request(&headers, "POST", "/api/admin/logs/homeserver/clear");
    if !clear.ok {
        log_admin_action_admission(&headers, "/admit/admin/action/view-logs-clear", &clear, StatusCode::OK);
        return admin_html_fragment_response(StatusCode::OK, admin_logs_modal_fragment(&clear, 0, limit, None));
    }
    let path = format!("/api/admin/logs/homeserver?offset=0&limit={limit}");
    let readback = admin_fragment_caduceus_request(&headers, "GET", &path);
    log_admin_action_admission(&headers, "/admit/admin/action/view-logs-clear", &readback, StatusCode::OK);
    admin_html_fragment_response(StatusCode::OK, admin_logs_modal_fragment(&readback, 0, limit, Some("Appliance logs cleared.")))
}


async fn admin_toggle_fragment_route(headers: axum::http::HeaderMap, Path(toggle_id): Path<String>) -> impl IntoResponse {
    if let Some(refusal) = admin_fragment_context_refusal(&headers) {
        let readback = mutation_refusal_readback("/api/v1/staff/intent", refusal);
        return admin_html_fragment_response(
            mutation_response_status(&readback),
            admin_membrane_refusal_fragment(&toggle_id, &readback.first_missing_signal),
        );
    }
    let Some((label, path)) = admin_toggle_target(&toggle_id) else {
        return admin_html_fragment_response(
            StatusCode::NOT_FOUND,
            admin_membrane_refusal_fragment("unknown admin toggle", "unknown-admin-toggle"),
        );
    };
    let readback = admin_fragment_staff_intent(&headers, "POST", path, "admin-service-toggle");
    let status_readback = admin_service_status_target(&toggle_id)
        .map(|status_path| admin_fragment_caduceus_request(&headers, "POST", status_path))
        .unwrap_or_else(|| mutation_refusal_readback(path, MutationRefusal { code: "unknown-admin-toggle".to_string(), status: 404 }));
    let result = AdminMutationResult {
        action: toggle_id.clone(),
        title: label.to_string(),
        message: if readback.ok { "Caduceus accepted the mutation; card re-read real state." } else { "Caduceus actuator is not wired or unavailable; card re-read unchanged real state." }.to_string(),
        ok: readback.ok,
        first_missing_signal: if readback.ok { "none".to_string() } else { readback.first_missing_signal.clone() },
    };
    admin_html_fragment_response(mutation_response_status(&readback), render_admin_service_card_result_html(&toggle_id, Some(&status_readback), Some(&result)))
}

async fn admin_service_card_fragment_route(headers: axum::http::HeaderMap, Path(toggle_id): Path<String>) -> impl IntoResponse {
    if let Some(refusal) = admin_fragment_context_refusal(&headers) {
        let readback = mutation_refusal_readback("/api/v1/staff/intent", refusal);
        return admin_html_fragment_response(
            mutation_response_status(&readback),
            admin_membrane_refusal_fragment(&toggle_id, &readback.first_missing_signal),
        );
    }
    let Some(path) = admin_service_status_target(&toggle_id) else {
        return admin_html_fragment_response(
            StatusCode::NOT_FOUND,
            admin_membrane_refusal_fragment("unknown admin service", "unknown-admin-service"),
        );
    };
    let readback = admin_fragment_caduceus_request(&headers, "POST", path);
    admin_html_fragment_response(mutation_response_status(&readback), render_admin_service_card_result_html(&toggle_id, Some(&readback), None))
}

async fn admin_action_fragment_route(headers: axum::http::HeaderMap, Path(action_id): Path<String>) -> impl IntoResponse {
    let route = format!("/admit/admin/action/{action_id}");
    if let Some(refusal) = admin_fragment_context_refusal(&headers) {
        let readback = mutation_refusal_readback("/api/v1/staff/intent", refusal);
        log_admin_action_admission(&headers, &route, &readback, mutation_response_status(&readback));
        return admin_html_fragment_response(
            mutation_response_status(&readback),
            admin_membrane_refusal_fragment(&action_id, &readback.first_missing_signal),
        );
    }
    let Some((title, method, path, mutation)) = admin_action_target(&action_id) else {
        let readback = mutation_refusal_readback("/api/v1/staff/intent", MutationRefusal { code: "unknown-admin-action".to_string(), status: 404 });
        log_admin_action_admission(&headers, &route, &readback, StatusCode::NOT_FOUND);
        return admin_html_fragment_response(
            StatusCode::NOT_FOUND,
            admin_membrane_refusal_fragment("unknown admin action", "unknown-admin-action"),
        );
    };
    let readback = if action_id == "update" {
        caduceus_mutation_readback(&headers, path, "update now", "local")
    } else if mutation {
        admin_fragment_staff_intent(&headers, method, path, homeserver_route_family(path))
    } else {
        admin_fragment_caduceus_request(&headers, method, path)
    };
    let status = if mutation { mutation_response_status(&readback) } else { StatusCode::OK };
    log_admin_action_admission(&headers, &route, &readback, status);
    let class = if readback.ok { "success" } else { "error" };
    let message = if readback.ok {
        if mutation { "Caduceus accepted the action." } else { "Readback returned through the Caduceus/crown route." }
    } else if mutation {
        "Caduceus actuator is not wired or unavailable; no optimistic success was rendered."
    } else {
        "Readback route unavailable; no fabricated logs were rendered."
    };
    let body = format!(
        r#"<div class="update-status-container {class}" data-admin-action-result-fragment="{}" data-admin-action-route="{}" data-og-affordance="toast-mapped-to-result-strip"><strong>{}</strong><span>{}</span><code>{}</code></div>"#,
        html_escape(&action_id),
        html_escape(path),
        html_escape(title),
        html_escape(message),
        html_escape(if readback.ok { "none" } else { &readback.first_missing_signal }),
    );
    admin_html_fragment_response(status, body)
}

fn log_admin_action_admission(headers: &axum::http::HeaderMap, route: &str, readback: &CaduceusHttpReadback, status: StatusCode) {
    let origin = headers.get(header::ORIGIN).and_then(|value| value.to_str().ok());
    let document = crate::caduceus_access::document_incarnation_from_headers(headers);
    eprintln!("{}", serde_json::json!({
        "event": "coronatio.admin-action.admission",
        "route": route,
        "upstreamOutcomeCode": if readback.ok { "none" } else { readback.first_missing_signal.as_str() },
        "mappedHttpStatus": status.as_u16(),
        "origin": origin,
        "documentId": document,
    }));
}

#[cfg(test)]
mod caduceus_loopback_tests {
    use super::*;

    #[test]
    fn raw_actuator_accepts_only_literal_loopback_socket_authorities() {
        for base in ["http://127.0.0.1:3014", "http://[::1]:3014"] {
            assert!(caduceus_authority_from_base(base).is_some(), "{base}");
        }
        for base in [
            "http://localhost:3014",
            "http://user@127.0.0.1:3014",
            "http://127.0.0.1:3014/path",
            "http://127.0.0.1:3014?query",
            "http://127.0.0.1:3014#fragment",
            "https://127.0.0.1:3014",
            "http://127.0.0.1",
            "http://192.0.2.1:3014",
        ] {
            assert!(caduceus_authority_from_base(base).is_none(), "{base}");
        }
    }
}
