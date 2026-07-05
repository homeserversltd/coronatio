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

async fn caduceus_update_check_route() -> impl IntoResponse {
    caduceus_mutation_route("update_check", "/api/v1/update/check")
}

async fn caduceus_update_now_route() -> impl IntoResponse {
    caduceus_dispatch_route("update_now", "/api/v1/update/now")
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

fn caduceus_dispatch_route(
    route: &'static str,
    path: &'static str,
) -> (StatusCode, Json<serde_json::Value>) {
    thread::spawn(move || {
        let _ = caduceus_http("POST", path);
    });
    (
        StatusCode::ACCEPTED,
        Json(serde_json::json!({
            "schema": "coronatio.caduceus.dispatch.v1",
            "ok": true,
            "route": route,
            "accepted": true,
            "path": path,
            "firstMissingSignal": "none"
        })),
    )
}

fn caduceus_mutation_route(route: &str, path: &str) -> (StatusCode, Json<serde_json::Value>) {
    let readback = caduceus_http("POST", path);
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
    env::var("CADUCEUS_URL").unwrap_or_else(|_| "http://127.0.0.1:3014".to_string())
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
    let request = format!(
        "{method} {path} HTTP/1.1\r\nHost: {authority}\r\nConnection: close\r\nContent-Length: 0\r\n\r\n"
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
    let request = format!(
        "{method} {path} HTTP/1.1\r\nHost: {authority}\r\nConnection: close\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
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


#[derive(Debug, Clone)]
struct AdminMutationResult {
    action: String,
    title: String,
    message: String,
    ok: bool,
    first_missing_signal: String,
}

fn admin_headers_authorized(headers: &axum::http::HeaderMap) -> bool {
    headers
        .get("x-admin-token")
        .and_then(|value| value.to_str().ok())
        .map(|value| value == "coronatio-session-token")
        .unwrap_or(false)
}

fn admin_membrane_refusal_fragment(surface: &str) -> String {
    format!(
        r#"<div class="update-status-container error" data-admin-membrane-refusal="true" data-og-affordance="toast-mapped-to-result-strip"><strong>Enter Admin Mode</strong><span>{}</span><code>admin-session-required</code></div>"#,
        html_escape(surface),
    )
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
    caduceus_http_json(
        "POST",
        "/api/v1/staff/intent",
        serde_json::json!({
            "method": method,
            "route": path,
            "classification": classification,
        }),
    )
}

async fn admin_toggle_fragment_route(headers: axum::http::HeaderMap, Path(toggle_id): Path<String>) -> impl IntoResponse {
    if !admin_headers_authorized(&headers) {
        return (StatusCode::UNAUTHORIZED, Html(admin_membrane_refusal_fragment(&toggle_id))).into_response();
    }
    let Some((label, path)) = admin_toggle_target(&toggle_id) else {
        return (StatusCode::NOT_FOUND, Html(admin_membrane_refusal_fragment("unknown admin toggle"))).into_response();
    };
    let readback = admin_staff_intent("POST", path, "admin-service-toggle");
    let result = AdminMutationResult {
        action: toggle_id.clone(),
        title: label.to_string(),
        message: if readback.ok { "Caduceus accepted the mutation; card re-read real state." } else { "Caduceus actuator is not wired or unavailable; card re-read unchanged real state." }.to_string(),
        ok: readback.ok,
        first_missing_signal: if readback.ok { "none".to_string() } else { readback.first_missing_signal },
    };
    let mut response = Html(render_admin_service_card_result_html(&toggle_id, Some(&result))).into_response();
    response.headers_mut().insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    response
}

async fn admin_action_fragment_route(headers: axum::http::HeaderMap, Path(action_id): Path<String>) -> impl IntoResponse {
    if !admin_headers_authorized(&headers) {
        return (StatusCode::UNAUTHORIZED, Html(admin_membrane_refusal_fragment(&action_id))).into_response();
    }
    let Some((title, method, path, mutation)) = admin_action_target(&action_id) else {
        return (StatusCode::NOT_FOUND, Html(admin_membrane_refusal_fragment("unknown admin action"))).into_response();
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
    let mut response = Html(body).into_response();
    response.headers_mut().insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    response
}
