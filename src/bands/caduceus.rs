use std::time::Instant;

async fn caduceus_status_route(_headers: axum::http::HeaderMap) -> impl IntoResponse {
    let (health, update, staff) = load_trace::spawn_blocking_with_trace(|| (
        caduceus_http("GET", "/health"),
        caduceus_http("GET", "/api/v1/update/status"),
        caduceus_http("GET", "/api/v1/doors"),
    )).await.unwrap_or_else(|_| (
        caduceus_failure_readback("/health", "caduceus-status-task-failed"),
        caduceus_failure_readback("/api/v1/update/status", "caduceus-status-task-failed"),
        caduceus_failure_readback("/api/v1/doors", "caduceus-status-task-failed"),
    ));
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
            "caduceusBase": caduceus_transport_readback(),
            "health": health,
            "update": update,
            "staff": staff,
            "firstMissingSignal": first_missing_signal
        })),
    )
}

async fn caduceus_update_check_route(headers: axum::http::HeaderMap) -> impl IntoResponse {
    caduceus_mutation_route(&headers, "update_check", "/api/v1/update/check")
}

async fn caduceus_update_now_route(headers: axum::http::HeaderMap) -> impl IntoResponse {
    caduceus_mutation_route(&headers, "update_now", "/api/v1/update/now")
}

async fn caduceus_update_service_status_route(headers: axum::http::HeaderMap) -> impl IntoResponse {
    let path = "/api/v1/update/service/status";
    let readback = admin_fragment_caduceus_request(&headers, "GET", path);
    let status = mutation_response_status(&readback);
    let ok = readback.ok;
    let first_missing_signal = readback.first_missing_signal.clone();
    (
        status,
        Json(serde_json::json!({
            "schema": "coronatio.caduceus.update.service.status.v1",
            "ok": ok,
            "readback": readback,
            "firstMissingSignal": first_missing_signal,
        })),
    )
}

async fn caduceus_update_service_toggle_route(
    headers: axum::http::HeaderMap,
    Json(body): Json<serde_json::Value>,
) -> impl IntoResponse {
    let path = "/api/v1/update/service/toggle";
    let readback = caduceus_actuate_json(
        &mutation_authority(),
        &headers,
        path,
        body,
    );
    let status = mutation_response_status(&readback);
    let ok = readback.ok;
    let first_missing_signal = readback.first_missing_signal.clone();
    (
        status,
        Json(serde_json::json!({
            "schema": "coronatio.caduceus.mutation.v1",
            "ok": ok,
            "accepted": ok,
            "route": "update_service_toggle",
            "readback": readback,
            "firstMissingSignal": first_missing_signal,
        })),
    )
}

fn update_manager_read(headers: &axum::http::HeaderMap, path: &str) -> (StatusCode, Json<serde_json::Value>) {
    let readback = admin_fragment_caduceus_request(headers, "GET", path);
    let status = mutation_response_status(&readback);
    let ok = readback.ok;
    (status, Json(serde_json::json!({"ok": ok, "readback": readback})))
}

async fn update_manager_status(headers: axum::http::HeaderMap) -> impl IntoResponse { update_manager_read(&headers, "/api/v1/update/status") }
async fn update_manager_modules(headers: axum::http::HeaderMap) -> impl IntoResponse { update_manager_read(&headers, "/api/v1/update/modules") }
async fn update_manager_history(headers: axum::http::HeaderMap) -> impl IntoResponse { update_manager_read(&headers, "/api/v1/update/history") }
async fn update_manager_interactables(headers: axum::http::HeaderMap) -> impl IntoResponse { update_manager_read(&headers, "/api/v1/interactables") }

fn update_manager_segment(id: &str) -> Option<String> {
    if id.is_empty() || id.len() > 512 || id.chars().any(char::is_control) { return None; }
    let mut encoded = String::new();
    for b in id.bytes() {
        if b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'~') { encoded.push(b as char); }
        else { encoded.push_str(&format!("%{b:02X}")); }
    }
    Some(encoded)
}

fn update_manager_mutation(headers: &axum::http::HeaderMap, path: &str, body: Option<serde_json::Value>) -> (StatusCode, Json<serde_json::Value>) {
    let authority = mutation_authority();
    let readback = match body {
        Some(body) => caduceus_actuate_json(&authority, headers, path, body),
        None => caduceus_actuate(&authority, headers, path),
    };
    let status = mutation_response_status(&readback);
    let ok = readback.ok;
    (status, Json(serde_json::json!({"ok": ok, "accepted": ok, "readback": readback})))
}

async fn update_manager_module(headers: axum::http::HeaderMap, Path(id): Path<String>, Json(body): Json<serde_json::Value>) -> (StatusCode, Json<serde_json::Value>) {
    let Some(id) = update_manager_segment(&id) else { return (StatusCode::BAD_REQUEST, Json(serde_json::json!({"ok":false}))); };
    let Some(enabled) = body.get("enabled").and_then(serde_json::Value::as_bool) else { return (StatusCode::BAD_REQUEST, Json(serde_json::json!({"ok":false}))); };
    update_manager_mutation(&headers, &format!("/api/v1/update/modules/{id}"), Some(serde_json::json!({"enabled":enabled})))
}

async fn update_manager_interactable(headers: axum::http::HeaderMap, Path((id, action)): Path<(String, String)>) -> (StatusCode, Json<serde_json::Value>) {
    let Some(id) = update_manager_segment(&id) else { return (StatusCode::BAD_REQUEST, Json(serde_json::json!({"ok":false}))); };
    if !matches!(action.as_str(), "run" | "silence" | "unsilence") { return (StatusCode::NOT_FOUND, Json(serde_json::json!({"ok":false}))); }
    update_manager_mutation(&headers, &format!("/api/v1/interactables/{id}/{action}"), None)
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
        "/api/v1/keyman/admin-password",
        serde_json::json!({"old_password": old_password, "new_password": new_password, "planned": request.planned}),
    ))
}

async fn caduceus_keyman_key_status_route(headers: axum::http::HeaderMap, Json(request): Json<KeymanDoorRequest>) -> impl IntoResponse {
    keyman_response(caduceus_actuate_json(
        &mutation_authority(), &headers,
        "/api/v1/keyman/key-status",
        serde_json::json!({"planned": request.planned}),
    ))
}

async fn caduceus_receipts_latest_route() -> impl IntoResponse {
    let readback = caduceus_http("GET", "/api/v1/log/receipts");
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

fn caduceus_mutation_readback(headers: &axum::http::HeaderMap, path: &str) -> CaduceusHttpReadback {
    caduceus_actuate(
        &mutation_authority(),
        &headers,
        path,
    )
}

fn caduceus_mutation_route(headers: &axum::http::HeaderMap, route: &str, path: &str) -> (StatusCode, Json<serde_json::Value>) {
    let readback = caduceus_mutation_readback(headers, path);
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
fn caduceus_guest_star_set(headers: &axum::http::HeaderMap, value: serde_json::Value) -> CaduceusHttpReadback {
    if crate::caduceus_access::attendance_from_headers(headers).is_some() {
        return caduceus_config_set(headers, "tabs.starred", value);
    }
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

fn caduceus_socket_path() -> std::path::PathBuf {
    crate::caduceus_access::staff_socket_path()
}

#[repr(C)]
struct CoronatioSockAddrUn {
    family: u16,
    path: [i8; 108],
}

#[repr(C)]
struct CoronatioPollFd {
    fd: i32,
    events: i16,
    revents: i16,
}

#[link(name = "c")]
extern "C" {
    fn socket(domain: i32, socket_type: i32, protocol: i32) -> i32;
    fn connect(fd: i32, address: *const CoronatioSockAddrUn, length: u32) -> i32;
    fn poll(fds: *mut CoronatioPollFd, count: usize, timeout_ms: i32) -> i32;
    fn getsockopt(fd: i32, level: i32, option: i32, value: *mut i32, length: *mut u32) -> i32;
    fn close(fd: i32) -> i32;
}

fn bounded_unix_connect(path: &std::path::Path, timeout: Duration) -> std::io::Result<UnixStream> {
    use std::{ffi::CString, os::unix::{ffi::OsStrExt, io::FromRawFd}};
    let path = CString::new(path.as_os_str().as_bytes())
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidInput, "socket path contains NUL"))?;
    let bytes = path.as_bytes_with_nul();
    if bytes.len() > 108 { return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "socket path too long")); }
    let fd = unsafe { socket(1, 1 | 0o4000 | 0o2000000, 0) };
    if fd < 0 { return Err(std::io::Error::last_os_error()); }
    let mut address = CoronatioSockAddrUn { family: 1, path: [0; 108] };
    for (slot, byte) in address.path.iter_mut().zip(bytes.iter().copied()) { *slot = byte as i8; }
    let started = Instant::now();
    let connected = unsafe { connect(fd, &address, std::mem::size_of::<CoronatioSockAddrUn>() as u32) };
    if connected < 0 {
        let error = std::io::Error::last_os_error();
        if !matches!(error.raw_os_error(), Some(11 | 115)) {
            unsafe { close(fd); }
            return Err(error);
        }
        loop {
            let Some(remaining) = timeout.checked_sub(started.elapsed()) else {
                unsafe { close(fd); }
                return Err(std::io::Error::new(std::io::ErrorKind::TimedOut, "Caduceus socket connect timed out"));
            };
            let millis = remaining.as_millis().max(1).min(i32::MAX as u128) as i32;
            let mut poll_fd = CoronatioPollFd { fd, events: 0x0004, revents: 0 };
            let result = unsafe { poll(&mut poll_fd, 1, millis) };
            if result == 0 {
                unsafe { close(fd); }
                return Err(std::io::Error::new(std::io::ErrorKind::TimedOut, "Caduceus socket connect timed out"));
            }
            if result < 0 {
                let error = std::io::Error::last_os_error();
                if error.raw_os_error() == Some(4) { continue; }
                unsafe { close(fd); }
                return Err(error);
            }
            let mut socket_error = 0_i32;
            let mut length = std::mem::size_of::<i32>() as u32;
            if unsafe { getsockopt(fd, 1, 4, &mut socket_error, &mut length) } < 0 {
                let error = std::io::Error::last_os_error();
                unsafe { close(fd); }
                return Err(error);
            }
            if socket_error != 0 {
                unsafe { close(fd); }
                return Err(std::io::Error::from_raw_os_error(socket_error));
            }
            break;
        }
    }
    let stream = unsafe { UnixStream::from_raw_fd(fd) };
    stream.set_nonblocking(false)?;
    Ok(stream)
}

fn remaining_io_timeout(started: Instant, timeout: Duration) -> std::io::Result<Duration> {
    timeout.checked_sub(started.elapsed()).filter(|remaining| !remaining.is_zero())
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::TimedOut, "Caduceus request deadline elapsed"))
}

fn write_request_until(stream: &mut UnixStream, request: &[u8], started: Instant, timeout: Duration) -> std::io::Result<()> {
    let mut written = 0;
    while written < request.len() {
        stream.set_write_timeout(Some(remaining_io_timeout(started, timeout)?))?;
        match std::io::Write::write(stream, &request[written..]) {
            Ok(0) => return Err(std::io::Error::new(std::io::ErrorKind::WriteZero, "Caduceus request write returned zero")),
            Ok(count) => written += count,
            Err(error) if error.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(error) => return Err(error),
        }
    }
    Ok(())
}

fn read_response_until(stream: &mut UnixStream, started: Instant, timeout: Duration) -> std::io::Result<String> {
    let mut response = Vec::new();
    let mut chunk = [0_u8; 8192];
    loop {
        stream.set_read_timeout(Some(remaining_io_timeout(started, timeout)?))?;
        match std::io::Read::read(stream, &mut chunk) {
            Ok(0) => break,
            Ok(count) => response.extend_from_slice(&chunk[..count]),
            Err(error) if error.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(error) => return Err(error),
        }
    }
    String::from_utf8(response).map_err(|error| std::io::Error::new(std::io::ErrorKind::InvalidData, error))
}

fn deadline_error(error: &std::io::Error, started: Instant, timeout: Duration) -> bool {
    error.kind() == std::io::ErrorKind::TimedOut
        || (error.kind() == std::io::ErrorKind::WouldBlock && remaining_io_timeout(started, timeout).is_err())
}

fn caduceus_transport_readback() -> &'static str { "caduceus-uds" }

fn caduceus_http(method: &str, path: &str) -> CaduceusHttpReadback {
    caduceus_http_timeout(method, path, Duration::from_secs(4))
}

fn caduceus_http_timeout(method: &str, path: &str, timeout: Duration) -> CaduceusHttpReadback {
    caduceus_http_with_attendance_and_document_timeout(method, path, None, None, timeout)
}

fn caduceus_http_with_attendance(method: &str, path: &str, attendance: Option<&crate::caduceus_access::AttendanceProof>) -> CaduceusHttpReadback {
    caduceus_http_with_attendance_and_document(method, path, attendance, None)
}

fn caduceus_http_with_attendance_and_document(
    method: &str,
    path: &str,
    attendance: Option<&crate::caduceus_access::AttendanceProof>,
    document: Option<&str>,
) -> CaduceusHttpReadback {
    caduceus_http_with_attendance_and_document_timeout(
        method,
        path,
        attendance,
        document,
        Duration::from_secs(4),
    )
}

fn caduceus_http_with_attendance_and_document_timeout(
    method: &str,
    path: &str,
    attendance: Option<&crate::caduceus_access::AttendanceProof>,
    document: Option<&str>,
    timeout: Duration,
) -> CaduceusHttpReadback {
    let started = Instant::now();
    let mut stream = match bounded_unix_connect(&caduceus_socket_path(), timeout) {
        Ok(stream) => stream,
        Err(error) => return failed_caduceus_call(path, if error.kind() == std::io::ErrorKind::TimedOut { "caduceus-connect-timeout" } else { "caduceus-unreachable" }, started, None),
    };
    let io_timeout = match remaining_io_timeout(started, timeout) {
        Ok(remaining) => remaining,
        Err(_) => return failed_caduceus_call(path, "caduceus-connect-timeout", started, None),
    };
    let _ = stream.set_read_timeout(Some(io_timeout));
    let _ = stream.set_write_timeout(Some(io_timeout));
    let attendance_header = attendance
            .map(|proof| format!("x-caduceus-attendance: {}\r\n", proof.expose()))
        .unwrap_or_default();
    let document_header = document
        .map(|value| format!("x-caduceus-document: {value}\r\n"))
        .unwrap_or_default();
    let trace_dispatch = load_trace::begin_dispatch();
    let trace_headers = trace_dispatch.header_lines();
    let request = format!(
        "{method} {path} HTTP/1.1\r\nHost: caduceus.local\r\nConnection: close\r\n{document_header}{attendance_header}{trace_headers}Content-Length: 0\r\n\r\n"
    );
    if let Err(error) = write_request_until(&mut stream, request.as_bytes(), started, timeout) {
        let signal = if deadline_error(&error, started, timeout) { "caduceus-write-timeout" } else { "caduceus-write-failed" };
        return failed_caduceus_call(path, signal, started, Some(&trace_dispatch));
    }
    let response = match read_response_until(&mut stream, started, timeout) {
        Ok(response) => response,
        Err(error) => {
            let signal = if deadline_error(&error, started, timeout) { "caduceus-read-timeout" } else { "caduceus-read-failed" };
            return failed_caduceus_call(path, signal, started, Some(&trace_dispatch));
        }
    };
    parse_caduceus_response(path, &response, started.elapsed(), trace_dispatch)
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

fn caduceus_hyalos_reflect_best_effort(
    kind: &'static str,
    level: String,
    message: String,
    correlation_id: Option<String>,
    attributes_redacted: Option<serde_json::Value>,
) {
    #[derive(Debug)]
    struct HyalosReflect {
        kind: &'static str,
        level: String,
        message: String,
        correlation_id: Option<String>,
        attributes_redacted: Option<serde_json::Value>,
    }

    static SENDER: OnceLock<std::sync::mpsc::SyncSender<HyalosReflect>> = OnceLock::new();
    let sender = SENDER.get_or_init(|| {
        let (sender, receiver) = std::sync::mpsc::sync_channel::<HyalosReflect>(128);
        let _ = std::thread::Builder::new()
            .name("coronatio-hyalos".to_string())
            .spawn(move || {
                while let Ok(event) = receiver.recv() {
                    let mut body = serde_json::json!({
                        "organ": "coronatio",
                        "kind": event.kind,
                        "level": event.level,
                        "message": event.message,
                    });
                    if let Some(correlation_id) = event.correlation_id {
                        body["correlation_id"] = serde_json::Value::String(correlation_id);
                    }
                    if let Some(attributes_redacted) = event.attributes_redacted {
                        body["attributes_redacted"] = attributes_redacted;
                    }
                    let _ = caduceus_http_json_with_attendance_and_document_timeout(
                        "POST",
                        "/api/v1/log/reflect",
                        body,
                        None,
                        None,
                        Duration::from_secs(2),
                    );
                }
            });
        sender
    });
    let _ = sender.try_send(HyalosReflect { kind, level, message, correlation_id, attributes_redacted });
}

fn caduceus_http_json_with_attendance_and_document(
    method: &str,
    path: &str,
    body: serde_json::Value,
    attendance: Option<&crate::caduceus_access::AttendanceProof>,
    document: Option<&str>,
) -> CaduceusHttpReadback {
    caduceus_http_json_with_attendance_and_document_timeout(
        method,
        path,
        body,
        attendance,
        document,
        Duration::from_secs(4),
    )
}

fn caduceus_http_json_with_attendance_and_document_timeout(
    method: &str,
    path: &str,
    body: serde_json::Value,
    attendance: Option<&crate::caduceus_access::AttendanceProof>,
    document: Option<&str>,
    timeout: Duration,
) -> CaduceusHttpReadback {
    let started = Instant::now();
    let mut stream = match bounded_unix_connect(&caduceus_socket_path(), timeout) {
        Ok(stream) => stream,
        Err(error) => return failed_caduceus_call(path, if error.kind() == std::io::ErrorKind::TimedOut { "caduceus-connect-timeout" } else { "caduceus-unreachable" }, started, None),
    };
    let io_timeout = match remaining_io_timeout(started, timeout) {
        Ok(remaining) => remaining,
        Err(_) => return failed_caduceus_call(path, "caduceus-connect-timeout", started, None),
    };
    let _ = stream.set_read_timeout(Some(io_timeout));
    let _ = stream.set_write_timeout(Some(io_timeout));
    let body_text = serde_json::to_string(&body).unwrap_or_else(|_| "{}".to_string());
    let attendance_header = attendance
            .map(|proof| format!("x-caduceus-attendance: {}\r\n", proof.expose()))
        .unwrap_or_default();
    let document_header = document
        .map(|value| format!("x-caduceus-document: {value}\r\n"))
        .unwrap_or_default();
    let trace_dispatch = load_trace::begin_dispatch();
    let trace_headers = trace_dispatch.header_lines();
    let request = format!(
        "{method} {path} HTTP/1.1\r\nHost: caduceus.local\r\nConnection: close\r\nContent-Type: application/json\r\n{document_header}{attendance_header}{trace_headers}Content-Length: {}\r\n\r\n{}",
        body_text.len(), body_text
    );
    if let Err(error) = write_request_until(&mut stream, request.as_bytes(), started, timeout) {
        let signal = if deadline_error(&error, started, timeout) { "caduceus-write-timeout" } else { "caduceus-write-failed" };
        return failed_caduceus_call(path, signal, started, Some(&trace_dispatch));
    }
    let response = match read_response_until(&mut stream, started, timeout) {
        Ok(response) => response,
        Err(error) => {
            let signal = if deadline_error(&error, started, timeout) { "caduceus-read-timeout" } else { "caduceus-read-failed" };
            return failed_caduceus_call(path, signal, started, Some(&trace_dispatch));
        }
    };
    parse_caduceus_response(path, &response, started.elapsed(), trace_dispatch)
}

fn parse_caduceus_response(path: &str, response: &str, io_duration: Duration, dispatch: load_trace::Dispatch) -> CaduceusHttpReadback {
    let (head, body_text) = response
        .split_once("\r\n\r\n")
        .unwrap_or(("", response));
    let Some(status) = head
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .and_then(|raw| raw.parse::<u16>().ok()) else {
        load_trace::record_dispatch_failure(&dispatch, io_duration, "caduceus-empty-response");
        return CaduceusHttpReadback {
            ok: false,
            status: 0,
            path: path.to_string(),
            body: serde_json::json!({"error": "caduceus-upstream-failed"}),
            first_missing_signal: "caduceus-empty-response".to_owned(),
        };
    };
    let status_valid = (100..=599).contains(&status);
    let provider = load_trace::parse_provider_headers(head);
    let decode_started = load_trace::current().and_then(|scope| scope.stage_start());
    let parsed_body = serde_json::from_str(body_text).unwrap_or_else(|_| serde_json::json!({"raw": body_text}));
    let (body, body_context) = load_trace::capture_provider_body(parsed_body);
    let body_ok = body
        .get("ok")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(status_valid && status < 400);
    let ok = status_valid && status < 400 && body_ok;
    let first_missing_signal = if !status_valid {
        "caduceus-http-status-invalid".to_owned()
    } else {
        let door_signal = (!ok)
            .then(|| body.get("doorSignal").and_then(serde_json::Value::as_str))
            .flatten()
            .filter(|signal| *signal != "none");
        door_signal
            .or_else(|| body.get("firstMissingSignal")
                .or_else(|| body.get("first_missing_signal"))
                .and_then(serde_json::Value::as_str))
            .unwrap_or(if status < 400 && body_ok { "none" } else { "caduceus-http-not-ok" })
            .to_string()
    };
    load_trace::record_provider_response(
        &dispatch,
        provider,
        body_context,
        io_duration,
        decode_started.map(|started| started.elapsed()),
        ok,
        (!ok).then_some(first_missing_signal.as_str()),
    );
    CaduceusHttpReadback {
        ok,
        status,
        path: path.to_string(),
        body,
        first_missing_signal,
    }
}

fn failed_caduceus_call(path: &str, signal: &str, started: Instant, dispatch: Option<&load_trace::Dispatch>) -> CaduceusHttpReadback {
    if let Some(dispatch) = dispatch {
        load_trace::record_dispatch_failure(dispatch, started.elapsed(), signal);
    } else {
        load_trace::record_transport_failure(path, started.elapsed(), signal);
    }
    CaduceusHttpReadback {
        ok: false,
        status: 0,
        path: path.to_string(),
        body: serde_json::json!({"error": "caduceus-upstream-failed"}),
        first_missing_signal: signal.to_owned(),
    }
}

fn caduceus_failure_readback(path: &str, signal: &str) -> CaduceusHttpReadback {
    CaduceusHttpReadback {
        ok: false,
        status: StatusCode::SERVICE_UNAVAILABLE.as_u16(),
        path: path.to_owned(),
        body: serde_json::json!({"error": "caduceus-upstream-failed"}),
        first_missing_signal: signal.to_owned(),
    }
}


fn hyalos_tail_path(kind: Option<&str>, count: usize) -> String {
    let count = count.clamp(1, 1000);
    match kind {
        Some(kind) => format!("/api/v1/log/tail?kind={kind}&count={count}"),
        None => format!("/api/v1/log/tail?count={count}"),
    }
}

fn hyalos_tail_readback(kind: Option<&str>, count: usize) -> CaduceusHttpReadback {
    caduceus_http("GET", &hyalos_tail_path(kind, count))
}

fn hyalos_upload_history_readback() -> Option<Vec<serde_json::Value>> {
    let readback = caduceus_http("GET", "/api/v1/log/tail?kind=upload&organ=file-ingress&count=100");
    if !readback.ok || !(200..300).contains(&readback.status) {
        return None;
    }
    hyalos_upload_history_from_tail(&readback)
}

fn hyalos_upload_history_from_tail(readback: &CaduceusHttpReadback) -> Option<Vec<serde_json::Value>> {
    use serde_json::Value;

    fn order_key(row: &Value) -> (&str, Option<i64>) {
        // The door speaks UTC. Strip its zone suffix so an exact second sorts before its fractions.
        let at = row.get("at").and_then(Value::as_str).unwrap_or("");
        let utc = at.trim_end_matches('Z').trim_end_matches("+00:00");
        (utc, row.get("id").and_then(Value::as_i64))
    }

    let mut uploads = readback.body.get("events")?.as_array()?.iter().filter(|row| {
        row.get("organ").and_then(Value::as_str) == Some("file-ingress")
            && row.get("verb").and_then(Value::as_str) == Some("upload")
    }).collect::<Vec<_>>();
    uploads.sort_by_key(|row| std::cmp::Reverse(order_key(row)));

    let root = upload_root_path();
    let display_root = upload_display_root(&root);
    Some(uploads.into_iter().take(100).map(|row| {
        let family = row.get("families").and_then(|value| value.get("upload"));
        let field = |key| family.and_then(|value| value.get(key));
        let filename = |value: &Value| value.as_str().filter(|name| {
            !name.trim().is_empty() && *name != "." && *name != ".." && !name.contains(['/', '\\', '\0'])
        }).map(str::to_owned);
        let name = field("filename").and_then(filename)
            .or_else(|| row.get("target").and_then(filename))
            .unwrap_or_else(|| "A file (name not recorded)".to_owned());
        let folder = field("destination").and_then(Value::as_str)
            .and_then(|path| upload_resolve_path(&root, &display_root, path))
            .and_then(|path| path.strip_prefix(&root).ok().map(|relative| relative.display().to_string()))
            .map(|relative| if relative.is_empty() { "Storage root".to_owned() } else { relative })
            .unwrap_or_else(|| "Folder not recorded".to_owned());
        let size = field("bytes").and_then(Value::as_u64)
            .map(hyalos_upload_history_size).unwrap_or_else(|| "Size not recorded".to_owned());
        serde_json::json!({
            "name": name,
            "size": size,
            "folder": folder,
            "at": row.get("at").and_then(Value::as_str),
            "success": matches!(row.get("outcome").and_then(Value::as_str), Some("ok" | "success")),
        })
    }).collect())
}

fn hyalos_upload_history_size(bytes: u64) -> String {
    const KIB: u64 = 1024;
    const MIB: u64 = 1024 * KIB;
    const GIB: u64 = 1024 * MIB;
    // At one decimal place, half a tenth of the smaller unit rounds up.
    const FIRST_MB: u64 = MIB - KIB / 20;
    const FIRST_GB: u64 = GIB - MIB / 20;

    let (scale, unit) = match bytes {
        0..=1023 => {
            let kilobytes = format!("{:.3}", bytes as f64 / KIB as f64);
            return format!("{} KB", kilobytes.trim_end_matches('0').trim_end_matches('.'));
        }
        FIRST_GB.. => (GIB, "GB"),
        FIRST_MB.. => (MIB, "MB"),
        _ => (KIB, "KB"),
    };
    format!("{:.1} {unit}", bytes as f64 / scale as f64)
}

fn hyalos_clear_upload_history(headers: &axum::http::HeaderMap) -> CaduceusHttpReadback {
    admin_fragment_caduceus_json_request(
        headers,
        "POST",
        "/api/v1/log/clear",
        serde_json::json!({"kind": "change", "verb": "upload"}),
    )
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


fn admin_membrane_refusal_fragment(surface: &str, first_missing_signal: &str) -> String {
    format!(
        r#"<div class="update-status-container error" data-admin-membrane-refusal="true" data-cartridge-fault-kind="{}" data-og-affordance="toast-mapped-to-result-strip"><strong>Enter Admin Mode</strong><span>{}</span><code>{}</code></div>"#,
        html_escape(first_missing_signal),
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
        "ssh-password-authentication" | "ssh-service" => Some("/api/v1/settings/ssh/status"),
        "samba-file-sharing" => Some("/api/v1/settings/samba/status"),
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

fn admin_logs_page_fragment(readback: &CaduceusHttpReadback, level: Option<u64>, offset: usize, limit: usize) -> String {
    let rows = readback.body.get("rows").and_then(serde_json::Value::as_array);
    let total = readback.body.get("total_lines").and_then(serde_json::Value::as_u64).unwrap_or(0) as usize;
    let returned = rows.map_or(0, Vec::len);
    let previous = offset.saturating_sub(limit);
    let next = offset.saturating_add(returned);
    let previous_disabled = if offset == 0 { " disabled" } else { "" };
    let next_disabled = if returned == 0 || next >= total { " disabled" } else { "" };
    let recording = level.map_or_else(String::new, |level| format!(" · recording level {level} of 5"));
    let page = if let Some(rows) = rows.filter(|rows| readback.ok && rows.iter().all(serde_json::Value::is_object)) {
        let content = if rows.is_empty() {
            "<p data-admin-log-empty>No appliance logs have been recorded.</p>".to_string()
        } else {
            let entries = rows.iter().map(|row| {
                let text = |key| row.get(key).and_then(serde_json::Value::as_str).unwrap_or_default();
                let at = text("at");
                let when = if at.is_empty() {
                    String::new()
                } else {
                    let utc = at.strip_suffix('Z').or_else(|| at.strip_suffix("+00:00")).unwrap_or(at);
                    format!(r#"<time datetime="{}">{} UTC</time>"#, html_escape(at), html_escape(&utc.replacen('T', " ", 1)))
                };
                let mut result = html_escape(text("outcome"));
                if row.get("trouble").and_then(serde_json::Value::as_bool) == Some(true) {
                    if !result.is_empty() { result.push_str(" · "); }
                    result.push_str(r#"<span class="ui-badge ui-badge--danger ui-badge--small">Trouble</span>"#);
                    let signal = text("signal");
                    if !signal.is_empty() {
                        result.push_str(" · ");
                        result.push_str(&html_escape(signal));
                    }
                }
                format!(
                    r#"    <tr><td data-label="When">{when}</td><td data-label="Who">{}</td><td data-label="What">{}</td><td data-label="Target">{}</td><td data-label="Outcome"><span>{result}</span></td></tr>"#,
                    html_escape(text("organ")), html_escape(text("verb")), html_escape(text("target")),
                )
            }).collect::<Vec<_>>().join("\n");
            format!(r#"<div class="ui-table ui-table--ruled ui-table--stack-list" style="overflow-x: auto; border-inline-end: 2px solid var(--border)" role="region" aria-label="Appliance log entries" tabindex="0">
  <table>
    <thead><tr><th scope="col">When</th><th scope="col">Who</th><th scope="col">What</th><th scope="col">Target</th><th scope="col">Outcome</th></tr></thead>
    <tbody>
{entries}
    </tbody>
  </table>
</div>"#)
        };
        format!(r#"<p aria-live="polite">Newest first · {returned} shown of {total}{recording}</p>
  {content}"#)
    } else if readback.ok {
        "<p class=\"error\" data-admin-log-error>The appliance did not return log entries.</p>".to_string()
    } else {
        format!(
            r#"<p class="error" data-admin-log-error>Could not read appliance logs: <code>{}</code></p>"#,
            html_escape(&readback.first_missing_signal),
        )
    };
    format!(
        r##"<section class="crown-dialog__body" data-admin-log-page tabindex="0" aria-label="Log entries">
  {page}
</section>
<div data-admin-log-clear-status role="status"></div>
<div class="crown-dialog__actions">
  <button type="button" class="ui-button ui-button--secondary" data-admin-log-pager="previous" hx-get="/admit/admin/action/view-logs?offset={previous}&amp;limit={limit}" hx-target="#admin-log-slot" hx-swap="innerHTML"{previous_disabled}>Previous</button>
  <button type="button" class="ui-button ui-button--secondary" data-admin-log-pager="next" hx-get="/admit/admin/action/view-logs?offset={next}&amp;limit={limit}" hx-target="#admin-log-slot" hx-swap="innerHTML"{next_disabled}>Next</button>
  <button type="button" class="ui-button ui-button--danger" data-admin-log-clear hx-post="/admit/admin/action/view-logs-clear" hx-target="#admin-log-slot" hx-swap="innerHTML" hx-disabled-elt="this" hx-trigger="click queue:none" hx-confirm="Clear the entire appliance log? All log entries will be permanently deleted and cannot be recovered.">Clear</button>
</div>"##,
    )
}

async fn admin_logs_fragment_route(headers: axum::http::HeaderMap, uri: Uri) -> Response {
    let (offset, limit) = admin_log_pagination(&uri);
    let path = format!("/api/v1/log/read?offset={offset}&limit={limit}");
    let context = MutationRequestContext::attended_document_from_headers(&headers);
    let (readback, level) = load_trace::spawn_blocking_with_trace(move || {
        let attendance = match mutation_authority().authorize(&context) {
            Ok(attendance) => attendance,
            Err(refusal) => return (mutation_refusal_readback(&path, refusal), None),
        };
        let proof = Some(&attendance.proof);
        let document = Some(attendance.document.as_str());
        let readback = caduceus_http_with_attendance_and_document("GET", &path, proof, document);
        let level = readback.ok.then(|| {
            // The page waits for this optional level read for at most its timeout.
            caduceus_http_with_attendance_and_document_timeout(
                "GET", "/api/v1/log/level", proof, document, Duration::from_millis(750),
            )
        }).filter(|reply| reply.ok && (200..300).contains(&reply.status))
            .and_then(|reply| reply.body.get("level").and_then(serde_json::Value::as_u64))
            .filter(|level| (1..=5).contains(level));
        (readback, level)
    }).await.unwrap_or_else(|_| (caduceus_failure_readback("/api/v1/log/read", "caduceus-log-read-task-failed"), None));
    log_admin_action_admission(&headers, "/admit/admin/action/view-logs", &readback, StatusCode::OK);
    admin_html_fragment_response(StatusCode::OK, admin_logs_page_fragment(&readback, level, offset, limit))
}

async fn admin_logs_clear_fragment_route(headers: axum::http::HeaderMap) -> Response {
    const PATH: &str = "/api/v1/log/clear";
    let attended_headers = headers.clone();
    let readback = load_trace::spawn_blocking_with_trace(move || {
        caduceus_actuate_json(&mutation_authority(), &attended_headers, PATH, serde_json::json!({}))
    }).await.unwrap_or_else(|_| mutation_task_failure_readback(PATH));
    let cleared = readback.ok
        && (200..300).contains(&readback.status)
        && readback.body.get("ok").and_then(serde_json::Value::as_bool) == Some(true);
    let status = if cleared {
        StatusCode::OK
    } else {
        StatusCode::from_u16(readback.status).ok()
            .filter(|status| status.is_client_error() || status.is_server_error())
            .unwrap_or(StatusCode::BAD_GATEWAY)
    };
    log_admin_action_admission(&headers, "/admit/admin/action/view-logs-clear", &readback, status);
    if cleared {
        // Start the new page at the beginning of what is now in the log.
        return admin_logs_fragment_route(headers, Uri::from_static("/admit/admin/action/view-logs")).await;
    }
    let message = match status {
        StatusCode::UNAUTHORIZED => "Enter Admin Mode to clear appliance logs.",
        StatusCode::BAD_REQUEST if readback.first_missing_signal == "caduceus-attendance-document-required" =>
            "Enter Admin Mode to clear appliance logs.",
        StatusCode::NOT_FOUND => "Clearing appliance logs is not available on this device yet.",
        _ if readback.status == 0 => "The appliance did not confirm the log was cleared. Reopen Appliance Logs to see what remains.",
        status if status.is_client_error() => "The appliance refused to clear the log.",
        _ => "Could not clear the appliance log. Try again.",
    };
    admin_html_fragment_response(status, format!(r#"<p data-admin-log-clear-refusal>{message}</p>"#))
}

async fn admin_toggle_fragment_route(headers: axum::http::HeaderMap, Path(toggle_id): Path<String>) -> impl IntoResponse {
    if let Some(refusal) = admin_fragment_context_refusal(&headers) {
        let readback = mutation_refusal_readback("/api/v1/appliance/report", refusal);
        return admin_html_fragment_response(
            StatusCode::OK,
            render_admin_service_card_result_html(&toggle_id, Some(&readback), None),
        );
    }
    let Some((label, path)) = admin_toggle_target(&toggle_id) else {
        let readback = mutation_refusal_readback(
            "/api/v1/appliance/report",
            MutationRefusal { code: "unknown-admin-toggle".to_string(), status: 404 },
        );
        return admin_html_fragment_response(
            StatusCode::NOT_FOUND,
            render_admin_service_card_result_html(&toggle_id, Some(&readback), None),
        );
    };
    let readback = admin_fragment_translation_debt(&headers, "POST", path);
    let status_readback = admin_service_status_target(&toggle_id)
        .map(|status_path| {
            if matches!(toggle_id.as_str(), "ssh-password-authentication" | "ssh-service" | "samba-file-sharing") {
                admin_fragment_caduceus_json_request(&headers, "GET", status_path, serde_json::json!({}))
            } else {
                admin_fragment_caduceus_request(&headers, "POST", status_path)
            }
        })
        .unwrap_or_else(|| mutation_refusal_readback(path, MutationRefusal { code: "unknown-admin-toggle".to_string(), status: 404 }));
    let result = AdminMutationResult {
        action: toggle_id.clone(),
        title: label.to_string(),
        message: if readback.ok { "Caduceus accepted the mutation; card re-read real state." } else { "Caduceus actuator is not wired or unavailable; card re-read unchanged real state." }.to_string(),
        ok: readback.ok,
        first_missing_signal: if readback.ok { "none".to_string() } else { readback.first_missing_signal.clone() },
    };
    admin_html_fragment_response(StatusCode::OK, render_admin_service_card_result_html(&toggle_id, Some(&status_readback), Some(&result)))
}

async fn admin_service_card_fragment_route(headers: axum::http::HeaderMap, Path(toggle_id): Path<String>) -> impl IntoResponse {
    if let Some(refusal) = admin_fragment_context_refusal(&headers) {
        let readback = mutation_refusal_readback("/api/v1/appliance/report", refusal);
        return admin_html_fragment_response(
            StatusCode::OK,
            render_admin_service_card_result_html(&toggle_id, Some(&readback), None),
        );
    }
    let Some(path) = admin_service_status_target(&toggle_id) else {
        let readback = mutation_refusal_readback(
            "/api/v1/appliance/report",
            MutationRefusal { code: "unknown-admin-service".to_string(), status: 404 },
        );
        return admin_html_fragment_response(
            StatusCode::NOT_FOUND,
            render_admin_service_card_result_html(&toggle_id, Some(&readback), None),
        );
    };
    let readback = if matches!(toggle_id.as_str(), "ssh-password-authentication" | "ssh-service" | "samba-file-sharing") {
        admin_fragment_caduceus_json_request(&headers, "GET", path, serde_json::json!({}))
    } else {
        admin_fragment_caduceus_json_request(&headers, "POST", path, serde_json::json!({}))
    };
    admin_html_fragment_response(StatusCode::OK, render_admin_service_card_result_html(&toggle_id, Some(&readback), None))
}

async fn admin_action_fragment_route(headers: axum::http::HeaderMap, Path(action_id): Path<String>) -> impl IntoResponse {
    let route = format!("/admit/admin/action/{action_id}");
    if let Some(refusal) = admin_fragment_context_refusal(&headers) {
        let readback = mutation_refusal_readback("/api/v1/appliance/report", refusal);
        log_admin_action_admission(&headers, &route, &readback, mutation_response_status(&readback));
        return admin_html_fragment_response(
            mutation_response_status(&readback),
            admin_membrane_refusal_fragment(&action_id, &readback.first_missing_signal),
        );
    }
    let Some((title, method, path, mutation)) = admin_action_target(&action_id) else {
        let readback = mutation_refusal_readback("/api/v1/appliance/report", MutationRefusal { code: "unknown-admin-action".to_string(), status: 404 });
        log_admin_action_admission(&headers, &route, &readback, StatusCode::NOT_FOUND);
        return admin_html_fragment_response(
            StatusCode::NOT_FOUND,
            admin_membrane_refusal_fragment("unknown admin action", "unknown-admin-action"),
        );
    };
    let readback = if action_id == "update" {
        caduceus_mutation_readback(&headers, path)
    } else if mutation {
        admin_fragment_translation_debt(&headers, method, path)
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


pub(crate) mod load_trace {
    use super::*;
    include!("load-trace.rs");
}

#[cfg(test)]
mod caduceus_uds_tests {
    use super::*;
    use std::io::{BufRead, BufReader, Write};
    use std::os::unix::net::UnixListener;
    use std::sync::{Arc, Mutex};

    #[tokio::test]
    async fn caduceus_status_route_uses_three_general_uds_requests() {
        let _guard = crate::CADUCEUS_ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
        let socket = std::env::temp_dir().join(format!("coronatio-status-{}-{}.sock", std::process::id(), uuid::Uuid::new_v4()));
        let listener = UnixListener::bind(&socket).unwrap();
        let requests = Arc::new(Mutex::new(Vec::new())); let captured = Arc::clone(&requests);
        std::env::set_var("CADUCEUS_STAFF_SOCKET", &socket);
        let worker = std::thread::spawn(move || {
            for _ in 0..3 {
                let (stream, _) = listener.accept().unwrap();
                let mut reader = BufReader::new(stream);
                let mut head = String::new();
                loop {
                    let mut line = String::new();
                    reader.read_line(&mut line).unwrap();
                    head.push_str(&line);
                    if line == "\r\n" { break; }
                }
                captured.lock().unwrap().push(head);
                let mut stream = reader.into_inner();
                let body = b"{\"ok\":true}";
                write!(stream, "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n", body.len()).unwrap();
                stream.write_all(body).unwrap();
            }
        });
        let response = caduceus_status_route().await.into_response();
        worker.join().unwrap(); std::env::remove_var("CADUCEUS_STAFF_SOCKET"); let _ = std::fs::remove_file(socket);
        assert_eq!(response.status(), StatusCode::OK); let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap(); let value: serde_json::Value = serde_json::from_slice(&body).unwrap(); assert_eq!(value["ok"], true); assert_eq!(value["caduceusBase"], "caduceus-uds");
        let requests = requests.lock().unwrap(); assert_eq!(requests.len(), 3); assert!(requests[0].starts_with("GET /health HTTP/1.1")); assert!(requests[0].contains("Host: caduceus.local"));
    }

    #[test]
    fn caduceus_general_request_reports_missing_socket() {
        let _guard = crate::CADUCEUS_ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
        let socket = std::env::temp_dir().join(format!("coronatio-status-absent-{}-{}.sock", std::process::id(), uuid::Uuid::new_v4())); let _ = std::fs::remove_file(&socket); std::env::set_var("CADUCEUS_STAFF_SOCKET", &socket);
        let readback = caduceus_http("GET", "/health"); std::env::remove_var("CADUCEUS_STAFF_SOCKET");
        assert!(!readback.ok); assert_eq!(readback.body["error"], "caduceus-upstream-failed"); assert_eq!(readback.first_missing_signal, "caduceus-unreachable");
    }
}
