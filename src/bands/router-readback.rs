async fn topics_route() -> impl IntoResponse {
    Json(topic_catalog_readback())
}

async fn monitor_pulse_route() -> impl IntoResponse {
    Json(monitor_pulse_readback())
}

async fn service_data_route(headers: axum::http::HeaderMap) -> Response {
    let raw = service_data_readback();
    match session_from_headers(&headers) {
        Session::Admin => Json(project_service_data_admin(&raw)).into_response(),
        Session::Guest => Json(project_service_data_guest(&raw)).into_response(),
    }
}

async fn frontend_storage_route() -> impl IntoResponse {
    Json(frontend_storage_readback())
}

async fn boundary_route() -> impl IntoResponse {
    Json(boundary_readback())
}

async fn installer_route() -> impl IntoResponse {
    Json(installer_readback())
}

async fn route_boundary_fallback(method: Method, uri: Uri) -> impl IntoResponse {
    let normalized = uri.path().to_string();
    if normalized.starts_with("/api/") {
        return (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "schema": "coronatio.api.error.v1",
                "error": "api route not found",
                "path": normalized,
                "method": method.as_str(),
                "policy": "API clients receive JSON 404; website endpoints must be explicitly registered Rust routes"
            })),
        )
            .into_response();
    }
    Html(render_crown_shell()).into_response()
}

async fn pane_route(Path(pane_id): Path<String>) -> impl IntoResponse {
    if !is_safe_tab_id(&pane_id) {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "invalid pane id"})),
        )
            .into_response();
    }

    match native_crown_panes()
        .into_iter()
        .find(|pane| pane.id == pane_id)
    {
        Some(pane) => Json(pane).into_response(),
        None => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "pane not found"})),
        )
            .into_response(),
    }
}

async fn tabs_route(State(state): State<AppState>) -> impl IntoResponse {
    match load_tab_manifests(&state.tab_root).await {
        Ok(tabs) => Json(TabList {
            schema: "coronatio.tabs.v1".to_string(),
            tab_root: state.tab_root.display().to_string(),
            native_panes: native_crown_panes(),
            tabs,
        })
        .into_response(),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": error.to_string()})),
        )
            .into_response(),
    }
}

async fn tab_manifest_route(
    State(state): State<AppState>,
    Path(tab_id): Path<String>,
) -> impl IntoResponse {
    if !is_safe_tab_id(&tab_id) {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "invalid tab id"})),
        )
            .into_response();
    }

    match load_tab_manifest(&state.tab_root, &tab_id).await {
        Ok(Some(manifest)) => Json(manifest).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "tab not found"})),
        )
            .into_response(),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": error.to_string()})),
        )
            .into_response(),
    }
}

async fn load_tab_manifests(tab_root: &PathBuf) -> Result<Vec<TabManifest>, std::io::Error> {
    let mut tabs = Vec::new();
    let mut entries = match fs::read_dir(tab_root).await {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(tabs),
        Err(error) => return Err(error),
    };

    while let Some(entry) = entries.next_entry().await? {
        if !entry.file_type().await?.is_dir() {
            continue;
        }
        let tab_id = entry.file_name().to_string_lossy().to_string();
        if !is_safe_tab_id(&tab_id) {
            continue;
        }
        if let Some(manifest) = load_tab_manifest(tab_root, &tab_id).await? {
            tabs.push(manifest);
        }
    }
    tabs.sort_by(|left, right| left.order.cmp(&right.order).then(left.id.cmp(&right.id)));
    Ok(tabs)
}

async fn load_tab_manifest(
    tab_root: &std::path::Path,
    tab_id: &str,
) -> Result<Option<TabManifest>, std::io::Error> {
    let manifest_path = tab_root.join(tab_id).join("tab.json");
    let raw = match fs::read_to_string(manifest_path).await {
        Ok(raw) => raw,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(error),
    };
    let manifest = serde_json::from_str::<TabManifest>(&raw)
        .map_err(|error| std::io::Error::new(std::io::ErrorKind::InvalidData, error))?;
    if manifest.id != tab_id || !is_safe_tab_id(&manifest.id) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "tab id mismatch or unsafe tab id",
        ));
    }
    validate_tab_manifest(&manifest)
        .map_err(|error| std::io::Error::new(std::io::ErrorKind::InvalidData, error))?;
    Ok(Some(manifest))
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
enum CartridgeFaultKind {
    Timeout,
    UpstreamError,
    TabNotFound,
    ProxyUnreachable,
}

impl CartridgeFaultKind {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Timeout => "timeout",
            Self::UpstreamError => "upstream-error",
            Self::TabNotFound => "tab-not-found",
            Self::ProxyUnreachable => "proxy-unreachable",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct CartridgeFaultReceipt {
    tab_id: String,
    fault_kind: CartridgeFaultKind,
    occurred_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct CartridgeFaultReadback {
    schema: String,
    readback_lane: String,
    capacity: usize,
    receipts: Vec<CartridgeFaultReceipt>,
}

const CARTRIDGE_FAULT_RECEIPT_CAPACITY: usize = 32;
static CARTRIDGE_FAULT_RECEIPTS: OnceLock<Mutex<VecDeque<CartridgeFaultReceipt>>> = OnceLock::new();

fn cartridge_fault_receipts() -> &'static Mutex<VecDeque<CartridgeFaultReceipt>> {
    CARTRIDGE_FAULT_RECEIPTS.get_or_init(|| Mutex::new(VecDeque::with_capacity(CARTRIDGE_FAULT_RECEIPT_CAPACITY)))
}

fn record_cartridge_fault(tab_id: &str, fault_kind: CartridgeFaultKind) -> CartridgeFaultReceipt {
    let receipt = CartridgeFaultReceipt {
        tab_id: tab_id.to_string(),
        fault_kind,
        occurred_at: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_secs())
            .unwrap_or(0),
    };
    let mut receipts = cartridge_fault_receipts().lock().expect("cartridge fault receipts lock");
    while receipts.len() >= CARTRIDGE_FAULT_RECEIPT_CAPACITY { receipts.pop_front(); }
    receipts.push_back(receipt.clone());
    receipt
}

async fn faults_route() -> impl IntoResponse {
    let receipts = cartridge_fault_receipts()
        .lock()
        .expect("cartridge fault receipts lock")
        .iter()
        .cloned()
        .collect();
    Json(CartridgeFaultReadback {
        schema: "coronatio.cartridge-faults.v1".to_string(),
        readback_lane: "occurred_at-unix-seconds".to_string(),
        capacity: CARTRIDGE_FAULT_RECEIPT_CAPACITY,
        receipts,
    })
}

async fn admit_tab_route(headers: axum::http::HeaderMap, Path(tab_id): Path<String>) -> impl IntoResponse {
    let session = session_from_headers(&headers);
    let mut response = if !is_safe_tab_id(&tab_id) {
        fragment_fault(StatusCode::BAD_REQUEST, &tab_id, CartridgeFaultKind::UpstreamError)
    } else if native_crown_panes().into_iter().any(|pane| pane.id == tab_id) {
        Html(render_og_pane_fragment(&tab_id, session)).into_response()
    } else {
        fragment_fault(StatusCode::NOT_FOUND, &tab_id, CartridgeFaultKind::TabNotFound)
    };
    response.headers_mut().insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    response.headers_mut().insert(header::CONTENT_SECURITY_POLICY, HeaderValue::from_static(CROWN_CONTENT_SECURITY_POLICY));
    response
}

fn fragment_fault(status: StatusCode, tab_id: &str, fault_kind: CartridgeFaultKind) -> Response {
    let receipt = record_cartridge_fault(tab_id, fault_kind);
    let fault = receipt.fault_kind.as_str();
    let body = format!(
        r#"<section class="card error-message" data-cartridge-fault="true" data-cartridge-fault-kind="{}" data-tab-id="{}" data-cartridge-fault-occurred-at="{}"><h2>Cartridge fault</h2><p>The pane stayed inside the og shell while admission failed.</p></section>"#,
        fault, tab_id, receipt.occurred_at
    );
    (status, [("x-coronatio-fault", "cartridge-fragment")], Html(body)).into_response()
}

fn render_og_pane_fragment(tab_id: &str, session: Session) -> String {
    let shell = render_crown_shell_for_session(session);
    let fragment = extract_pane_inner_html(&shell, tab_id).unwrap_or_else(|| {
        record_cartridge_fault(tab_id, CartridgeFaultKind::TabNotFound);
        format!(
            r#"<section class="card error-message" data-cartridge-fault="true" data-cartridge-fault-kind="tab-not-found" data-tab-id="{}"><h2>Cartridge fault</h2><p>Pane not found.</p></section>"#,
            tab_id
        )
    });
    if tab_id == "portals" {
        fragment.replacen(
            "data-portals-fragment=\"/api/portals/elements\"",
            "data-portals-fragment=\"/api/portals/elements\" hx-get=\"/api/portals/elements\" hx-trigger=\"load\" hx-swap=\"innerHTML\"",
            1,
        )
    } else {
        fragment
    }
}

fn extract_pane_inner_html(shell: &str, tab_id: &str) -> Option<String> {
    let marker = format!(r#"data-pane-panel="{}""#, tab_id);
    let marker_at = shell.find(&marker)?;
    let tag_start = shell[..marker_at].rfind("<section")?;
    let tag_end = shell[marker_at..].find('>').map(|offset| marker_at + offset + 1)?;
    let mut depth = 1usize;
    let mut scan = tag_end;
    while depth > 0 {
        let next_open = shell[scan..].find("<section").map(|offset| scan + offset);
        let next_close = shell[scan..].find("</section>").map(|offset| scan + offset);
        match (next_open, next_close) {
            (_, Some(close)) if next_open.map(|open| close < open).unwrap_or(true) => {
                depth -= 1;
                if depth == 0 { return Some(shell[tag_end..close].to_string()); }
                scan = close + "</section>".len();
            }
            (Some(open), Some(_)) => {
                depth += 1;
                scan = open + "<section".len();
            }
            _ => return None,
        }
    }
    let _ = tag_start;
    None
}

fn session_from_headers(headers: &axum::http::HeaderMap) -> Session {
    let (Some(document), Some(attendance)) = (crate::caduceus_access::document_incarnation_from_headers(headers), crate::caduceus_access::attendance_from_headers(headers)) else { return Session::Guest; };
    crate::caduceus_access::CaduceusAccessClient::default().attendance_validate(&attendance, &document).receipt.ok.then_some(Session::Admin).unwrap_or(Session::Guest)
}

const CADUCEUS_SESSION_BODY_MAX: usize = 4 * 1024;

fn session_projection(call: crate::caduceus_access::AttendanceCall) -> serde_json::Value {
    serde_json::json!({"schema":"coronatio.caduceus.attendance.projection.v1","ok":call.receipt.ok,"admin":call.receipt.ok,"attendance":call.proof.map(|proof| proof.0),"firstMissingSignal":call.receipt.code})
}

fn guest_session_projection(signal: &str) -> serde_json::Value {
    serde_json::json!({"schema":"coronatio.caduceus.attendance.projection.v1","ok":false,"admin":false,"firstMissingSignal":crate::caduceus_access::safe_access_code(signal)})
}

fn document_admission_projection(call: crate::caduceus_access::AttendanceCall) -> serde_json::Value {
    let mut projection = session_projection(call);
    if projection.get("ok").and_then(serde_json::Value::as_bool) == Some(true) {
        let object = projection.as_object_mut().expect("attendance projection object");
        object.insert("adminPatch".to_string(), serde_json::Value::String(render_admin_document_patch()));
        object.insert("adminTabs".to_string(), serde_json::Value::String(render_plan_tabbar(Session::Admin)));
    }
    projection
}

fn session_response(status: StatusCode, projection: serde_json::Value, clear_cookie: bool) -> Response {
    let mut response = (status, Json(projection)).into_response();
    let _ = clear_cookie;
    response
}

fn json_content_type(headers: &axum::http::HeaderMap) -> bool {
    headers
        .get(header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.split(';').next())
        .map(str::trim)
        .is_some_and(|value| value.eq_ignore_ascii_case("application/json") || value.to_ascii_lowercase().ends_with("+json"))
}

fn attendance_failure_status(call: &crate::caduceus_access::AttendanceCall) -> StatusCode {
    if call.receipt.ok {
        return StatusCode::OK;
    }
    match call.receipt.code.as_str() {
        "caduceus-access-origin-refused" | "caduceus-attendance-origin-refused" => StatusCode::FORBIDDEN,
        "caduceus-access-refused"
        | "caduceus-attendance-refused"
        | "caduceus-attendance-pin-refused"
        | "caduceus-attendance-not-current"
        | "caduceus-attendance-invalid"
        | "caduceus-attendance-required"
        | "caduceus-stale-incarnation"
        | "caduceus-attendance-stale-incarnation" => StatusCode::UNAUTHORIZED,
        _ => StatusCode::SERVICE_UNAVAILABLE,
    }
}

fn attendance_projection_response(
    headers: &axum::http::HeaderMap,
    route: &str,
    status: StatusCode,
    projection: serde_json::Value,
    document: Option<&str>,
) -> Response {
    let code = projection.get("firstMissingSignal").and_then(serde_json::Value::as_str).unwrap_or("none");
    let origin = headers.get(header::ORIGIN).and_then(|value| value.to_str().ok());
    eprintln!("{}", serde_json::json!({
        "event": "coronatio.attendance.projection",
        "route": route,
        "upstreamOutcomeCode": code,
        "mappedHttpStatus": status.as_u16(),
        "origin": origin,
        "documentId": document,
    }));
    session_response(status, projection, false)
}

async fn caduceus_attendance_open_route(headers: axum::http::HeaderMap, body: axum::body::Bytes) -> Response {
    const ROUTE: &str = "/api/v1/attendance/open";
    if !crate::caduceus_access::same_origin_state_change(&headers) {
        return attendance_projection_response(&headers, ROUTE, StatusCode::FORBIDDEN, guest_session_projection("caduceus-access-origin-refused"), None);
    }
    if !json_content_type(&headers) || body.len() > CADUCEUS_SESSION_BODY_MAX {
        return attendance_projection_response(&headers, ROUTE, StatusCode::BAD_REQUEST, guest_session_projection("caduceus-access-request-invalid"), None);
    }
    let Ok(body) = serde_json::from_slice::<serde_json::Value>(&body) else {
        return attendance_projection_response(&headers, ROUTE, StatusCode::BAD_REQUEST, guest_session_projection("caduceus-access-request-invalid"), None);
    };
    let Some(pin) = body.get("pin").and_then(serde_json::Value::as_str).filter(|pin| !pin.is_empty() && pin.len() <= 256) else {
        return attendance_projection_response(&headers, ROUTE, StatusCode::BAD_REQUEST, guest_session_projection("caduceus-access-pin-required"), None);
    };
    let Some(document) = crate::caduceus_access::document_incarnation_from_headers(&headers) else { return attendance_projection_response(&headers, ROUTE, StatusCode::BAD_REQUEST, guest_session_projection("caduceus-attendance-document-required"), None); };
    let call = crate::caduceus_access::CaduceusAccessClient::default().attendance_open(pin, &document);
    let status = attendance_failure_status(&call);
    attendance_projection_response(&headers, ROUTE, status, document_admission_projection(call), Some(&document))
}

async fn caduceus_attendance_validate_route(headers: axum::http::HeaderMap) -> Response {
    const ROUTE: &str = "/api/v1/attendance/validate";
    let Some(document) = crate::caduceus_access::document_incarnation_from_headers(&headers) else { return attendance_projection_response(&headers, ROUTE, StatusCode::BAD_REQUEST, guest_session_projection("caduceus-attendance-document-required"), None); };
    let Some(attendance) = crate::caduceus_access::attendance_from_headers(&headers) else { return attendance_projection_response(&headers, ROUTE, StatusCode::UNAUTHORIZED, guest_session_projection("caduceus-attendance-required"), Some(&document)); };
    let call = crate::caduceus_access::CaduceusAccessClient::default().attendance_validate(&attendance, &document);
    let status = attendance_failure_status(&call);
    attendance_projection_response(&headers, ROUTE, status, session_projection(call), Some(&document))
}

async fn caduceus_attendance_touch_route(headers: axum::http::HeaderMap) -> Response {
    const ROUTE: &str = "/api/v1/attendance/touch";
    let Some(document) = crate::caduceus_access::document_incarnation_from_headers(&headers) else { return attendance_projection_response(&headers, ROUTE, StatusCode::BAD_REQUEST, guest_session_projection("caduceus-attendance-document-required"), None); };
    let Some(attendance) = crate::caduceus_access::attendance_from_headers(&headers) else { return attendance_projection_response(&headers, ROUTE, StatusCode::UNAUTHORIZED, guest_session_projection("caduceus-attendance-required"), Some(&document)); };
    let call = crate::caduceus_access::CaduceusAccessClient::default().attendance_touch(&attendance, &document);
    let status = attendance_failure_status(&call);
    attendance_projection_response(&headers, ROUTE, status, session_projection(call), Some(&document))
}

async fn caduceus_attendance_change_pin_route(headers: axum::http::HeaderMap, body: axum::body::Bytes) -> Response {
    const ROUTE: &str = "/api/v1/attendance/change-pin";
    let Some(document) = crate::caduceus_access::document_incarnation_from_headers(&headers) else { return attendance_projection_response(&headers, ROUTE, StatusCode::BAD_REQUEST, guest_session_projection("caduceus-attendance-document-required"), None); };
    let Some(attendance) = crate::caduceus_access::attendance_from_headers(&headers) else { return attendance_projection_response(&headers, ROUTE, StatusCode::UNAUTHORIZED, guest_session_projection("caduceus-attendance-required"), Some(&document)); };
    if !json_content_type(&headers) || body.len() > CADUCEUS_SESSION_BODY_MAX {
        return attendance_projection_response(&headers, ROUTE, StatusCode::BAD_REQUEST, guest_session_projection("caduceus-attendance-request-invalid"), Some(&document));
    }
    let Ok(body) = serde_json::from_slice::<serde_json::Value>(&body) else {
        return attendance_projection_response(&headers, ROUTE, StatusCode::BAD_REQUEST, guest_session_projection("caduceus-attendance-request-invalid"), Some(&document));
    };
    let Some(current_pin) = body.get("currentPin").and_then(serde_json::Value::as_str).filter(|pin| !pin.is_empty() && pin.len() <= 512) else {
        return attendance_projection_response(&headers, ROUTE, StatusCode::BAD_REQUEST, guest_session_projection("caduceus-attendance-currentPin-missing"), Some(&document));
    };
    let Some(new_pin) = body.get("newPin").and_then(serde_json::Value::as_str).filter(|pin| !pin.is_empty() && pin.len() <= 512) else {
        return attendance_projection_response(&headers, ROUTE, StatusCode::BAD_REQUEST, guest_session_projection("caduceus-attendance-newPin-missing"), Some(&document));
    };
    let call = crate::caduceus_access::CaduceusAccessClient::default().attendance_change_pin(&attendance, &document, current_pin, new_pin);
    let status = attendance_failure_status(&call);
    attendance_projection_response(&headers, ROUTE, status, session_projection(call), Some(&document))
}

async fn caduceus_attendance_invalidate_route(headers: axum::http::HeaderMap) -> Response {
    const ROUTE: &str = "/api/v1/attendance/invalidate";
    let Some(document) = crate::caduceus_access::document_incarnation_from_headers(&headers) else { return attendance_projection_response(&headers, ROUTE, StatusCode::BAD_REQUEST, guest_session_projection("caduceus-attendance-document-required"), None); };
    let Some(attendance) = crate::caduceus_access::attendance_from_headers(&headers) else { return attendance_projection_response(&headers, ROUTE, StatusCode::UNAUTHORIZED, guest_session_projection("caduceus-attendance-required"), Some(&document)); };
    let call = crate::caduceus_access::CaduceusAccessClient::default().attendance_invalidate(&attendance, &document);
    let invalidated = call.receipt.ok;
    pulse::downgrade_document(&document);
    indicators::downgrade_core_document(&document);
    let status = attendance_failure_status(&call);
    let projection = if invalidated { guest_session_projection("none") } else { session_projection(call) };
    attendance_projection_response(&headers, ROUTE, status, projection, Some(&document))
}
