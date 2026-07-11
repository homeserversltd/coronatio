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
            "caduceusBase": caduceus_base(),
            "health": health,
            "update": update,
            "staff": staff,
            "firstMissingSignal": first_missing_signal
        })),
    )
}

async fn caduceus_update_check_route(headers: axum::http::HeaderMap) -> impl IntoResponse {
    if !admin_headers_authorized(&headers) {
        return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"ok":false,"firstMissingSignal":"admin-session-required"})));
    }
    caduceus_mutation_route("update_check", "/api/v1/update/check", "update check", "local")
}

async fn caduceus_update_now_route(headers: axum::http::HeaderMap) -> impl IntoResponse {
    if !admin_headers_authorized(&headers) {
        return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"ok":false,"firstMissingSignal":"admin-session-required"})));
    }
    caduceus_mutation_route("update_now", "/api/v1/update/now", "update now", "local")
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

fn caduceus_mutation_route(route: &str, path: &str, action: &str, target: &str) -> (StatusCode, Json<serde_json::Value>) {
    let readback = match mint_caduceus_capability(action, target) {
        Ok(capability) => caduceus_http_with_capability("POST", path, Some(&capability)),
        Err(signal) => CaduceusHttpReadback {
            ok: false,
            status: 0,
            path: path.to_string(),
            body: serde_json::json!({"error": signal}),
            first_missing_signal: signal,
        },
    };
    (
        if readback.ok {
            StatusCode::OK
        } else {
            StatusCode::SERVICE_UNAVAILABLE
        },
        Json(serde_json::json!({
            "schema": "coronatio.caduceus.mutation.v1",
            "ok": readback.ok,
            "route": route,
            "readback": readback,
            "firstMissingSignal": readback.first_missing_signal
        })),
    )
}

fn decode_signing_key(text: &str) -> Option<[u8; 32]> {
    let text = text.trim();
    let decoded = if text.len() == 64 && text.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        (0..32)
            .map(|index| u8::from_str_radix(&text[index * 2..index * 2 + 2], 16).ok())
            .collect::<Option<Vec<_>>>()?
    } else {
        STANDARD.decode(text).ok().or_else(|| URL_SAFE_NO_PAD.decode(text).ok())?
    };
    decoded.try_into().ok()
}

fn household_signing_key() -> Result<SigningKey, String> {
    let material = env::var("CORONATIO_CADUCEUS_SIGNING_KEY").ok().or_else(|| {
        let path = env::var("CORONATIO_CADUCEUS_SIGNING_KEY_FILE")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("/etc/caduceus/household-signing.key"));
        std::fs::read_to_string(path).ok()
    }).ok_or_else(|| "caduceus-household-signing-key-missing".to_string())?;
    let seed = decode_signing_key(&material)
        .ok_or_else(|| "caduceus-household-signing-key-malformed".to_string())?;
    Ok(SigningKey::from_bytes(&seed))
}

fn mint_caduceus_capability(action: &str, target: &str) -> Result<String, String> {
    let exp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| "caduceus-capability-clock".to_string())?
        .as_secs() + 60;
    let payload = serde_json::to_string(&serde_json::json!({
        "actor": "coronatio-admin-session",
        "action": action,
        "target": target,
        "exp": exp,
    })).map_err(|_| "caduceus-capability-payload".to_string())?;
    let signature = household_signing_key()?.sign(payload.as_bytes());
    Ok(format!(
        "{}.{}",
        URL_SAFE_NO_PAD.encode(payload.as_bytes()),
        URL_SAFE_NO_PAD.encode(signature.to_bytes())
    ))
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
    env::var("CADUCEUS_BASE_URL")
        .or_else(|_| env::var("CADUCEUS_URL"))
        .unwrap_or_else(|_| "http://127.0.0.1:3014".to_string())
}

fn caduceus_authority() -> (String, String) {
    let base = caduceus_base();
    let without_scheme = base
        .strip_prefix("http://")
        .or_else(|| base.strip_prefix("https://"))
        .unwrap_or(base.as_str());
    let authority = without_scheme.trim_end_matches('/').to_string();
    (base, authority)
}

fn caduceus_http(method: &str, path: &str) -> CaduceusHttpReadback {
    caduceus_http_with_capability(method, path, None)
}

fn caduceus_http_with_capability(method: &str, path: &str, capability: Option<&str>) -> CaduceusHttpReadback {
    let (_base, authority) = caduceus_authority();
    let mut stream = match TcpStream::connect(&authority) {
        Ok(stream) => stream,
        Err(err) => {
            return CaduceusHttpReadback {
                ok: false,
                status: 0,
                path: path.to_string(),
                body: serde_json::json!({"error": err.to_string()}),
                first_missing_signal: "caduceus-unreachable".to_string(),
            };
        }
    };
    let _ = stream.set_read_timeout(Some(Duration::from_secs(4)));
    let _ = stream.set_write_timeout(Some(Duration::from_secs(4)));
    let capability_header = capability.map(|value| format!("x-caduceus-capability: {value}\r\n")).unwrap_or_default();
    let request = format!(
        "{method} {path} HTTP/1.1\r\nHost: {authority}\r\nConnection: close\r\n{capability_header}Content-Length: 0\r\n\r\n"
    );
    if let Err(err) = stream.write_all(request.as_bytes()) {
        return CaduceusHttpReadback {
            ok: false,
            status: 0,
            path: path.to_string(),
            body: serde_json::json!({"error": err.to_string()}),
            first_missing_signal: "caduceus-write-failed".to_string(),
        };
    }
    let mut response = String::new();
    if let Err(err) = stream.read_to_string(&mut response) {
        return CaduceusHttpReadback {
            ok: false,
            status: 0,
            path: path.to_string(),
            body: serde_json::json!({"error": err.to_string()}),
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

fn caduceus_http_json(method: &str, path: &str, body: serde_json::Value) -> CaduceusHttpReadback {
    caduceus_http_json_with_capability(method, path, body, None)
}

fn caduceus_http_json_with_capability(method: &str, path: &str, body: serde_json::Value, capability: Option<&str>) -> CaduceusHttpReadback {
    let (_base, authority) = caduceus_authority();
    let mut stream = match TcpStream::connect(&authority) {
        Ok(stream) => stream,
        Err(err) => {
            return CaduceusHttpReadback {
                ok: false,
                status: 0,
                path: path.to_string(),
                body: serde_json::json!({"error": err.to_string()}),
                first_missing_signal: "caduceus-unreachable".to_string(),
            };
        }
    };
    let _ = stream.set_read_timeout(Some(Duration::from_secs(4)));
    let _ = stream.set_write_timeout(Some(Duration::from_secs(4)));
    let body_text = serde_json::to_string(&body).unwrap_or_else(|_| "{}".to_string());
    let capability_header = capability.map(|value| format!("x-caduceus-capability: {value}\r\n")).unwrap_or_default();
    let request = format!(
        "{method} {path} HTTP/1.1\r\nHost: {authority}\r\nConnection: close\r\nContent-Type: application/json\r\n{capability_header}Content-Length: {}\r\n\r\n{}",
        body_text.len(), body_text
    );
    if let Err(err) = stream.write_all(request.as_bytes()) {
        return CaduceusHttpReadback {
            ok: false,
            status: 0,
            path: path.to_string(),
            body: serde_json::json!({"error": err.to_string()}),
            first_missing_signal: "caduceus-write-failed".to_string(),
        };
    }
    let mut response = String::new();
    if let Err(err) = stream.read_to_string(&mut response) {
        return CaduceusHttpReadback {
            ok: false,
            status: 0,
            path: path.to_string(),
            body: serde_json::json!({"error": err.to_string()}),
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



#[derive(Debug, Clone)]
struct AdminMutationResult {
    action: String,
    title: String,
    message: String,
    ok: bool,
    first_missing_signal: String,
}

fn admin_headers_authorized(headers: &axum::http::HeaderMap) -> bool {
    session_from_headers(headers) == Session::Admin
}

fn admin_membrane_refusal_fragment(surface: &str) -> String {
    format!(
        r#"<div class="update-status-container error" data-admin-membrane-refusal="true" data-og-affordance="toast-mapped-to-result-strip"><strong>Enter Admin Mode</strong><span>{}</span><code>admin-session-required</code></div>"#,
        html_escape(surface),
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

fn admin_action_target(action_id: &str) -> Option<(&'static str, &'static str, &'static str, bool)> {
    match action_id {
        "hard-drive-test" => Some(("Hard Drive Test", "POST", "/api/admin/hard-drive-test/start", true)),
        "update" => Some(("Update", "POST", "/api/admin/updates/apply", true)),
        "restart" => Some(("Restart", "POST", "/api/admin/system/restart", true)),
        "shutdown" => Some(("Shutdown", "POST", "/api/admin/system/shutdown", true)),
        "restart-website" => Some(("Restart Website", "POST", "/api/admin/services/hard-reset", true)),
        "view-logs" => Some(("View Logs", "GET", "/api/admin/logs/homeserver", false)),
        "install-certificate" => Some(("Install Certificate", "POST", "/api/admin/refresh-root-crt", true)),
        _ => None,
    }
}

fn admin_staff_intent(method: &str, path: &str, classification: &str) -> CaduceusHttpReadback {
    let capability = match mint_caduceus_capability("staff intent", path) {
        Ok(capability) => capability,
        Err(signal) => return CaduceusHttpReadback {
            ok: false,
            status: 0,
            path: path.to_string(),
            body: serde_json::json!({"error": signal}),
            first_missing_signal: signal,
        },
    };
    caduceus_http_json_with_capability(
        "POST",
        "/api/v1/staff/intent",
        serde_json::json!({
            "method": method,
            "route": path,
            "classification": classification,
        }),
        Some(&capability),
    )
}

async fn admin_toggle_fragment_route(headers: axum::http::HeaderMap, Path(toggle_id): Path<String>) -> impl IntoResponse {
    if !admin_headers_authorized(&headers) {
        return admin_html_fragment_response(StatusCode::UNAUTHORIZED, admin_membrane_refusal_fragment(&toggle_id));
    }
    let Some((label, path)) = admin_toggle_target(&toggle_id) else {
        return admin_html_fragment_response(StatusCode::NOT_FOUND, admin_membrane_refusal_fragment("unknown admin toggle"));
    };
    let readback = admin_staff_intent("POST", path, "admin-service-toggle");
    let result = AdminMutationResult {
        action: toggle_id.clone(),
        title: label.to_string(),
        message: if readback.ok { "Caduceus accepted the mutation; card re-read real state." } else { "Caduceus actuator is not wired or unavailable; card re-read unchanged real state." }.to_string(),
        ok: readback.ok,
        first_missing_signal: if readback.ok { "none".to_string() } else { readback.first_missing_signal },
    };
    admin_html_fragment_response(StatusCode::OK, render_admin_service_card_result_html(&toggle_id, Some(&result)))
}

async fn admin_action_fragment_route(headers: axum::http::HeaderMap, Path(action_id): Path<String>) -> impl IntoResponse {
    if !admin_headers_authorized(&headers) {
        return admin_html_fragment_response(StatusCode::UNAUTHORIZED, admin_membrane_refusal_fragment(&action_id));
    }
    let Some((title, method, path, mutation)) = admin_action_target(&action_id) else {
        return admin_html_fragment_response(StatusCode::NOT_FOUND, admin_membrane_refusal_fragment("unknown admin action"));
    };
    let readback = if mutation { admin_staff_intent(method, path, homeserver_route_family(path)) } else { caduceus_http(method, path) };
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
    admin_html_fragment_response(StatusCode::OK, body)
}
