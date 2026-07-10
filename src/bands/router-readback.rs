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
    tab_root: &PathBuf,
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

async fn admit_tab_route(Path(tab_id): Path<String>) -> impl IntoResponse {
    let mut response = if !is_safe_tab_id(&tab_id) {
        fragment_fault(StatusCode::BAD_REQUEST, &tab_id, CartridgeFaultKind::UpstreamError)
    } else if native_crown_panes().into_iter().any(|pane| pane.id == tab_id) {
        Html(render_og_pane_fragment(&tab_id)).into_response()
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

fn render_og_pane_fragment(tab_id: &str) -> String {
    let shell = render_crown_shell();
    extract_pane_inner_html(&shell, tab_id).unwrap_or_else(|| {
        record_cartridge_fault(tab_id, CartridgeFaultKind::TabNotFound);
        format!(
            r#"<section class="card error-message" data-cartridge-fault="true" data-cartridge-fault-kind="tab-not-found" data-tab-id="{}"><h2>Cartridge fault</h2><p>Pane not found.</p></section>"#,
            tab_id
        )
    })
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

#[derive(Default)]
struct LockoutState {
    failed_attempts: u32,
    locked_until_ms: u128,
}

#[derive(Default)]
struct SessionMembrane {
    tokens: HashSet<String>,
    lockouts: BTreeMap<String, LockoutState>,
}

static SESSION_MEMBRANE: OnceLock<Mutex<SessionMembrane>> = OnceLock::new();

fn session_membrane() -> &'static Mutex<SessionMembrane> {
    SESSION_MEMBRANE.get_or_init(|| Mutex::new(SessionMembrane::default()))
}

fn now_ms() -> u128 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis()
}

fn lockout_ms(attempt: u32) -> u128 {
    let exponent = attempt.saturating_sub(1).min(8);
    (1000u128.saturating_mul(1u128 << exponent)).min(5 * 60 * 1000)
}

fn mint_admin_token() -> String {
    let uuid = uuid::Uuid::new_v4();
    let mut random = [0u8; 32];
    if let Ok(mut file) = std::fs::File::open("/dev/urandom") {
        let _ = file.read_exact(&mut random);
    }
    let random_hex = random.iter().map(|byte| format!("{byte:02x}")).collect::<String>();
    format!("coronatio-admin-session-{uuid}-{random_hex}")
}

fn token_from_headers(headers: &axum::http::HeaderMap) -> Option<String> {
    headers
        .get("x-admin-token")
        .and_then(|value| value.to_str().ok())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
}

fn admin_token_authorized(token: &str) -> bool {
    session_membrane()
        .lock()
        .map(|store| store.tokens.contains(token))
        .unwrap_or(false)
}


#[cfg(test)]
fn authorize_test_admin_token() -> String {
    let token = mint_admin_token();
    session_membrane().lock().unwrap().tokens.insert(token.clone());
    token
}

fn session_from_headers(headers: &axum::http::HeaderMap) -> Session {
    token_from_headers(headers)
        .filter(|token| admin_token_authorized(token))
        .map(|_| Session::Admin)
        .unwrap_or(Session::Guest)
}

async fn homeserver_validate_pin_route(Json(body): Json<serde_json::Value>) -> impl IntoResponse {
    let supplied = body
        .get("pin")
        .or_else(|| body.get("password"))
        .map(|value| value.as_str().map(str::to_string).unwrap_or_else(|| value.to_string().trim_matches('"').to_string()))
        .unwrap_or_default();
    let configured = configured_admin_pin();
    let source = configured
        .as_ref()
        .map(|(path, _)| format!("{} global.admin.pin", path.display()))
        .unwrap_or_else(|| {
            homeserver_pin_config_candidates()
                .into_iter()
                .map(|path| path.display().to_string())
                .collect::<Vec<_>>()
                .join(" | ")
        });
    let now = now_ms();
    let valid = configured.as_ref().map(|(_, pin)| pin == &supplied).unwrap_or(false);
    if !valid {
        let store = session_membrane().lock().unwrap();
        if let Some(lockout) = store.lockouts.get(&source) {
            if lockout.locked_until_ms > now {
                let remaining_ms = lockout.locked_until_ms - now;
                return (StatusCode::TOO_MANY_REQUESTS, Json(serde_json::json!({
                "schema": "coronatio.homeserver.auth.pin.v1",
                "success": false,
                "verified": false,
                "valid": false,
                "locked": true,
                "remainingMs": remaining_ms,
                "remainingSeconds": ((remaining_ms + 999) / 1000),
                "source": source,
                "firstMissingSignal": "pin-lockout-active"
                }))).into_response();
            }
        }
    }
    if valid {
        let token = mint_admin_token();
        let mut store = session_membrane().lock().unwrap();
        store.lockouts.remove(&source);
        store.tokens.insert(token.clone());
        return (StatusCode::OK, Json(serde_json::json!({
            "schema": "coronatio.homeserver.auth.pin.v1",
            "success": true,
            "verified": true,
            "valid": true,
            "token": token,
            "expiresIn": 1800,
            "source": source,
            "firstMissingSignal": "none"
        }))).into_response();
    }
    let mut store = session_membrane().lock().unwrap();
    let lockout = store.lockouts.entry(source.clone()).or_default();
    lockout.failed_attempts = lockout.failed_attempts.saturating_add(1);
    let delay = lockout_ms(lockout.failed_attempts);
    lockout.locked_until_ms = now.saturating_add(delay);
    (StatusCode::UNAUTHORIZED, Json(serde_json::json!({
        "schema": "coronatio.homeserver.auth.pin.v1",
        "success": false,
        "verified": false,
        "valid": false,
        "locked": true,
        "remainingMs": delay,
        "remainingSeconds": ((delay + 999) / 1000),
        "attempts": lockout.failed_attempts,
        "source": source,
        "firstMissingSignal": if configured.is_some() { "pin-mismatch" } else { "homeserver-config-pin-missing" }
    }))).into_response()
}

async fn homeserver_logout_route(headers: axum::http::HeaderMap) -> impl IntoResponse {
    if let Some(token) = token_from_headers(&headers) {
        if let Ok(mut store) = session_membrane().lock() {
            store.tokens.remove(&token);
        }
    }
    Json(serde_json::json!({
        "schema": "coronatio.homeserver.auth.logout.v1",
        "success": true,
        "ok": true,
        "message": "session cleared by server"
    }))
}


async fn homeserver_admin_ping_route() -> impl IntoResponse {
    Json(serde_json::json!({
        "schema": "coronatio.homeserver.admin.ping.v1",
        "success": true,
        "ok": true,
        "authenticated": true
    }))
}



fn homeserver_pin_config_candidates() -> Vec<PathBuf> {
    let mut paths = Vec::new();
    if let Ok(path) = env::var("CORONATIO_HOMESERVER_JSON") {
        paths.push(PathBuf::from(path));
    }
    paths.push(PathBuf::from("/etc/homeserver/config.json"));
    paths.push(PathBuf::from("/etc/homeserver.json"));
    paths.push(PathBuf::from("/var/www/homeserver/src/config/homeserver.json"));
    paths.push(PathBuf::from("/etc/homeserver.factory"));
    paths
}

fn configured_admin_pin() -> Option<(PathBuf, String)> {
    for path in homeserver_pin_config_candidates() {
        let text = match std::fs::read_to_string(&path) {
            Ok(text) => text,
            Err(_) => continue,
        };
        let value: serde_json::Value = match serde_json::from_str(&text) {
            Ok(value) => value,
            Err(_) => continue,
        };
        if let Some(pin_value) = value
            .get("global")
            .and_then(|global| global.get("admin"))
            .and_then(|admin| admin.get("pin"))
        {
            let pin = pin_value
                .as_str()
                .map(str::to_string)
                .unwrap_or_else(|| pin_value.to_string().trim_matches('"').to_string());
            return Some((path, pin));
        }
    }
    None
}

