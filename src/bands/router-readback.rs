async fn topics_route() -> impl IntoResponse {
    Json(topic_catalog_readback())
}

async fn monitor_pulse_route() -> impl IntoResponse {
    Json(monitor_pulse_readback())
}

async fn service_data_route(headers: axum::http::HeaderMap) -> Response {
    let raw = service_data_readback();
    match session_projection_from_headers(&headers).await {
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
            native_tab_contracts: registry_readback().native_tab_contracts,
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
) -> Response {
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
    PackCssRefused,
}

impl CartridgeFaultKind {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Timeout => "timeout",
            Self::UpstreamError => "upstream-error",
            Self::TabNotFound => "tab-not-found",
            Self::ProxyUnreachable => "proxy-unreachable",
            Self::PackCssRefused => "pack-css-refused",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct CartridgeFaultReceipt {
    tab_id: String,
    fault_kind: CartridgeFaultKind,
    occurred_at: u64,
    first_missing_signal: String,
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
    let signal = fault_kind.as_str().to_string();
    record_cartridge_fault_signal(tab_id, fault_kind, &signal)
}

fn record_cartridge_fault_signal(tab_id: &str, fault_kind: CartridgeFaultKind, first_missing_signal: &str) -> CartridgeFaultReceipt {
    let receipt = CartridgeFaultReceipt {
        tab_id: tab_id.to_string(),
        fault_kind,
        occurred_at: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_secs())
            .unwrap_or(0),
        first_missing_signal: first_missing_signal.to_string(),
    };
    caduceus_hyalos_reflect_best_effort(
        if native_crown_panes().into_iter().any(|pane| pane.id == tab_id) {
            "crown-cartridge-fault"
        } else {
            "xenia"
        },
        "error".to_string(),
        format!("cartridge fault: {}", receipt.fault_kind.as_str()),
        Some(tab_id.to_string()),
        Some(serde_json::json!({
            "fault_kind": receipt.fault_kind.as_str(),
            "first_missing_signal": receipt.first_missing_signal,
            "occurred_at": receipt.occurred_at,
            "phase": "cartridge-fault",
        })),
    );
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

fn xenia_text(tab: &CoronatioTabContract, field: &str) -> Option<String> {
    tab.xenia_entry.as_ref()?.get(field)?.as_str().map(str::to_string)
}

fn xenia_kind(tab: &CoronatioTabContract) -> Option<String> {
    tab.kind.as_ref().and_then(serde_json::Value::as_str).map(str::to_string)
}

fn xenia_client_class(tab: &CoronatioTabContract) -> Option<String> {
    tab.client_class.as_ref().and_then(serde_json::Value::as_str).map(str::to_string)
}

fn xenia_static_dir(tab: &CoronatioTabContract) -> Option<String> {
    let entry = tab.xenia_entry.as_ref()?;
    entry.get("static_dir").or_else(|| entry.get("staticDir"))
        .or_else(|| entry.get("install").and_then(|install| install.get("static_dir").or_else(|| install.get("staticDir"))))
        .and_then(serde_json::Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty() && !value.starts_with('/') && !value.split('/').any(|part| part == ".."))
        .map(str::to_string)
}

fn xenia_row_url(tab_id: &str) -> Option<String> {
    let value = registry_config_value();
    let row = value.get("tabs")?.get(tab_id)?;
    row.get("url").or_else(|| row.get("config").and_then(|config| config.get("url")))
        .and_then(serde_json::Value::as_str).and_then(safe_cartridge_url)
}

fn xenia_static_url(tab_id: &str, static_dir: &str, leaf: &str) -> String {
    let leaf = leaf.trim_start_matches('/');
    format!("/tabs/{}/{}/{}", tab_id, static_dir, leaf)
}

fn xenia_visible_recovery(tab_id: &str, rung: &str, message: &str) -> Response {
    let kind = if rung.contains("timeout") {
        CartridgeFaultKind::Timeout
    } else if rung == "pack-css" || rung.starts_with("pack-css-") {
        CartridgeFaultKind::PackCssRefused
    } else if matches!(rung, "declared" | "listening" | "probe") || rung.contains("unreachable") {
        CartridgeFaultKind::ProxyUnreachable
    } else {
        CartridgeFaultKind::UpstreamError
    };
    fragment_fault_with_signal(StatusCode::SERVICE_UNAVAILABLE, tab_id, kind, rung, message)
}

fn xenia_static_admission(tab: &CoronatioTabContract, static_dir: &str, client_class: &str) -> Response {
    match client_class {
        "fragment" => Html(format!(
            r#"<div class="cartridge-viewport" data-cartridge-id="{}" hx-get="{}" hx-trigger="load" hx-swap="innerHTML"></div>"#,
            html_escape(&tab.id), html_escape(&xenia_static_url(&tab.id, static_dir, "fragment")),
        )).into_response(),
        "iframe" => Html(format!(
            r#"<div class="cartridge-viewport" data-cartridge-id="{}"><iframe src="{}" title="{}" sandbox="allow-scripts allow-same-origin allow-forms" referrerpolicy="same-origin"></iframe></div>"#,
            html_escape(&tab.id), html_escape(&xenia_static_url(&tab.id, static_dir, "")), html_escape(&tab.display_name),
        )).into_response(),
        _ => xenia_visible_recovery(&tab.id, "client-class", "The guest presentation class is not renderable."),
    }
}

async fn paired_xenos(tab_id: &str) -> Option<CoronatioTabContract> {
    let tab_id = tab_id.to_string();
    tokio::task::spawn_blocking(move || {
        let value = registry_config_value();
        let status = xenia_status(true);
        build_tab_contracts(&value, &status).into_iter()
            .find(|tab| tab.id == tab_id && tab.xenia_entry.is_some())
    }).await.ok().flatten()
}

async fn admit_xenos(tab: CoronatioTabContract) -> Response {
    let kind = xenia_kind(&tab).unwrap_or_default();
    let client_class = xenia_client_class(&tab).unwrap_or_else(|| "fragment".to_string());
    match kind.as_str() {
        "iframe" => match (client_class.as_str(), xenia_row_url(&tab.id)) {
            ("iframe", Some(url)) => Html(format!(
                r#"<div class="cartridge-viewport" data-cartridge-id="{}"><iframe src="{}" title="{}" sandbox="allow-scripts allow-same-origin allow-forms" referrerpolicy="same-origin"></iframe></div>"#,
                html_escape(&tab.id), html_escape(&url), html_escape(&tab.display_name),
            )).into_response(),
            ("iframe", None) => xenia_visible_recovery(&tab.id, "iframe-row-url", "The guest row has no usable URL."),
            _ => xenia_visible_recovery(&tab.id, "iframe-client-class", "An iframe guest must use the iframe presentation class."),
        },
        "cartridge-static" => match xenia_static_dir(&tab) {
            Some(static_dir) => xenia_static_admission(&tab, &static_dir, &client_class),
            None => xenia_visible_recovery(&tab.id, "static-dir", "The static guest has no safe static directory."),
        },
        "cartridge-process" => {
            let discovery = xenia_discovery::discover(&tab).await;
            if discovery.content_kind.as_deref() == Some("static") {
                return discovery.static_dir.as_deref()
                    .map(|static_dir| xenia_static_admission(&tab, static_dir, &client_class))
                    .unwrap_or_else(|| xenia_visible_recovery(&tab.id, "static-dir", "Static fallback has no safe directory."));
            }
            let Some(endpoint) = discovery.endpoint.as_deref() else {
                let fault = discovery.fault_signal.as_deref().unwrap_or("none:no-process-transport");
                let mut parts = fault.splitn(2, ':');
                let rung = parts.next().unwrap_or("none");
                let message = parts.next().unwrap_or("No process transport answered.");
                return xenia_visible_recovery(&tab.id, rung, message);
            };
            if discovery.content_kind.as_deref() != Some("html") {
                return xenia_visible_recovery(&tab.id, "content-kind", "The discovered response is not renderable HTML.");
            }
            match client_class.as_str() {
                "iframe" => Html(format!(
                    r#"<div class="cartridge-viewport" data-cartridge-id="{}"><iframe src="/api/tabs/{}/" title="{}" sandbox="allow-scripts allow-same-origin allow-forms" referrerpolicy="same-origin"></iframe></div>"#,
                    html_escape(&tab.id), html_escape(&tab.id), html_escape(&tab.display_name),
                )).into_response(),
                "fragment" => {
                    let path = xenia_text(&tab, "fragment_path").or_else(|| xenia_text(&tab, "fragmentPath")).unwrap_or_else(|| "/fragment".to_string());
                    match xenia_discovery::proxy(&tab.id, endpoint, Method::GET, &path, &axum::http::HeaderMap::new(), Body::empty()).await {
                        Ok(response) if response.status().is_success() && response.headers().get(header::CONTENT_TYPE).and_then(|value| value.to_str().ok()).is_some_and(|value| value.to_ascii_lowercase().starts_with("text/html")) => response,
                        Ok(_) => xenia_visible_recovery(&tab.id, "fragment-nonrenderable", "The fragment route did not return renderable HTML."),
                        Err(signal) => xenia_visible_recovery(&tab.id, signal, "The fragment route did not answer within its boundary."),
                    }
                }
                _ => xenia_visible_recovery(&tab.id, "client-class", "The guest presentation class is not renderable."),
            }
        }
        _ => xenia_visible_recovery(&tab.id, "kind", "The guest installation kind is not supported."),
    }
}

async fn admit_tab_route(headers: axum::http::HeaderMap, Path(tab_id): Path<String>) -> impl IntoResponse {
    let session = session_projection_from_headers(&headers).await;
    let mut response = if !is_safe_tab_id(&tab_id) {
        fragment_fault(StatusCode::BAD_REQUEST, &tab_id, CartridgeFaultKind::UpstreamError)
    } else if tab_id == "linker" {
        linker_fragment_route(headers.clone(), Query(LinkerQuery::default())).await
    } else if let Some(cartridge) = appliance_cartridge(&tab_id) {
        let facts = load_iris_facts_sync();
        let visible = iris::plan(&facts, session).tabs.into_iter().any(|grant| grant.tab_id == tab_id && grant.state == RenderState::Visible);
        if !visible { fragment_fault(StatusCode::NOT_FOUND, &tab_id, CartridgeFaultKind::TabNotFound) }
        else if tab_id == "my-devices" && my_devices_proxy::is_target(&cartridge.url)
            && (!my_devices_proxy::admitted() || !my_devices_proxy::activation_ready().await) {
            fragment_fault(StatusCode::SERVICE_UNAVAILABLE, &tab_id, CartridgeFaultKind::ProxyUnreachable)
        } else { Html(render_cartridge_iframe_fragment(&cartridge)).into_response() }
    } else if native_crown_panes().into_iter().any(|pane| pane.id == tab_id) {
        Html(render_og_pane_fragment(&tab_id, session)).into_response()
    } else if let Some(xenos) = paired_xenos(&tab_id).await {
        let facts = load_iris_facts_sync();
        let visible = iris::plan(&facts, session).tabs.into_iter().any(|grant| grant.tab_id == tab_id && grant.state == RenderState::Visible);
        if visible { admit_xenos(xenos).await } else { fragment_fault(StatusCode::NOT_FOUND, &tab_id, CartridgeFaultKind::TabNotFound) }
    } else {
        fragment_fault(StatusCode::NOT_FOUND, &tab_id, CartridgeFaultKind::TabNotFound)
    };
    response.headers_mut().insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    response.headers_mut().insert(header::CONTENT_SECURITY_POLICY, HeaderValue::from_static(CROWN_CONTENT_SECURITY_POLICY));
    response
}

async fn xenia_proxy_response(tab_id: String, path: String, method: Method, uri: Uri, headers: axum::http::HeaderMap, body: Body) -> Response {
    if !is_safe_tab_id(&tab_id) {
        return fragment_fault(StatusCode::BAD_REQUEST, &tab_id, CartridgeFaultKind::UpstreamError);
    }
    if let Err(signal) = xenia_discovery::validate_proxy_path(&path) {
        return xenia_visible_recovery(&tab_id, signal, "The guest path is not valid for the crown proxy.");
    }
    let Some(tab) = paired_xenos(&tab_id).await else {
        return fragment_fault(StatusCode::NOT_FOUND, &tab_id, CartridgeFaultKind::TabNotFound);
    };
    let session = session_projection_from_headers(&headers).await;
    let facts = load_iris_facts_sync();
    if !iris::plan(&facts, session).tabs.into_iter().any(|grant| grant.tab_id == tab_id && grant.state == RenderState::Visible) {
        return fragment_fault(StatusCode::NOT_FOUND, &tab_id, CartridgeFaultKind::TabNotFound);
    }
    if xenia_kind(&tab).as_deref() != Some("cartridge-process") {
        return fragment_fault(StatusCode::NOT_FOUND, &tab_id, CartridgeFaultKind::TabNotFound);
    }
    let discovery = xenia_discovery::discover(&tab).await;
    let Some(endpoint) = discovery.endpoint.as_deref() else {
        let fault = discovery.fault_signal.as_deref().unwrap_or("none:no-process-transport");
        let mut parts = fault.splitn(2, ':');
        return xenia_visible_recovery(
            &tab_id,
            parts.next().unwrap_or("none"),
            parts.next().unwrap_or("No process transport answered."),
        );
    };
    let mut path_and_query = path;
    if let Some(query) = uri.query() { path_and_query.push('?'); path_and_query.push_str(query); }
    let fragment_path = xenia_text(&tab, "fragment_path")
        .or_else(|| xenia_text(&tab, "fragmentPath"))
        .unwrap_or_else(|| "/fragment".to_string());
    let fragment_response = path_and_query.split('?').next() == Some(fragment_path.as_str());
    match xenia_discovery::proxy(&tab_id, endpoint, method, &path_and_query, &headers, body).await {
        Ok(mut response) => {
            if fragment_response && response.headers().get(header::CONTENT_TYPE).and_then(|value| value.to_str().ok()).is_some_and(|value| value.to_ascii_lowercase().starts_with("text/html")) {
                response.headers_mut().insert(header::CONTENT_SECURITY_POLICY, HeaderValue::from_static(CROWN_CONTENT_SECURITY_POLICY));
            }
            no_store(response)
        }
        Err(signal) => xenia_visible_recovery(&tab_id, signal, "The guest route did not answer within its boundary."),
    }
}

async fn xenia_proxy_default_route(Path(tab_id): Path<String>, method: Method, uri: Uri, headers: axum::http::HeaderMap, body: Body) -> Response {
    let path = paired_xenos(&tab_id).await
        .and_then(|tab| xenia_text(&tab, "fragment_path").or_else(|| xenia_text(&tab, "fragmentPath")))
        .unwrap_or_else(|| "/fragment".to_string());
    xenia_proxy_response(tab_id, path, method, uri, headers, body).await
}

async fn xenia_proxy_root_route(Path(tab_id): Path<String>, method: Method, uri: Uri, headers: axum::http::HeaderMap, body: Body) -> Response {
    xenia_proxy_response(tab_id, "/".to_string(), method, uri, headers, body).await
}

async fn xenia_proxy_path_route(Path((tab_id, _path)): Path<(String, String)>, method: Method, uri: Uri, headers: axum::http::HeaderMap, body: Body) -> Response {
    let prefix = format!("/api/tabs/{tab_id}");
    let path = uri
        .path()
        .strip_prefix(&prefix)
        .filter(|suffix| suffix.starts_with('/'))
        .unwrap_or("")
        .to_string();
    xenia_proxy_response(tab_id, path, method, uri, headers, body).await
}

fn render_cartridge_pane_hosts() -> String {
    load_appliance_cartridges().into_iter().map(|cartridge| format!(
        r#"<section class="pane" id="pane-{}" data-pane-panel="{}" data-view-panel="{}" role="tabpanel" aria-label="{}"></section>"#,
        html_escape(&cartridge.id), html_escape(&cartridge.id), html_escape(&cartridge.id), html_escape(&cartridge.title)
    )).collect::<Vec<_>>().join("")
}

fn render_cartridge_iframe_fragment(cartridge: &ApplianceCartridge) -> String {
    format!(r#"<div class="cartridge-viewport" data-cartridge-id="{}"><iframe src="{}" title="{}" sandbox="allow-scripts allow-same-origin allow-forms" referrerpolicy="same-origin"></iframe></div>"#,
        html_escape(&cartridge.id), html_escape(if cartridge.id == "my-devices" && my_devices_proxy::is_target(&cartridge.url) { my_devices_proxy::PATH } else { &cartridge.url }), html_escape(&cartridge.title))
}

fn fragment_fault(status: StatusCode, tab_id: &str, fault_kind: CartridgeFaultKind) -> Response {
    let signal = fault_kind.as_str().to_string();
    fragment_fault_with_signal(status, tab_id, fault_kind, &signal, "This pane is unavailable.")
}

fn fragment_fault_with_signal(status: StatusCode, tab_id: &str, fault_kind: CartridgeFaultKind, signal: &str, message: &str) -> Response {
    let receipt = record_cartridge_fault_signal(tab_id, fault_kind, signal);
    let fault = receipt.fault_kind.as_str();
    let body = format!(
        r#"<div hidden data-cartridge-fault="true" data-cartridge-fault-kind="{}" data-first-missing-signal="{}" data-tab-id="{}" data-cartridge-fault-occurred-at="{}"></div><section class="pane-content"><h1>{} is unavailable</h1><p>{}</p><button class="ui-button ui-button--secondary" type="button" hx-get="/admit/{}" hx-target="closest [data-view-panel]" hx-swap="innerHTML">Try again</button></section>"#,
        fault, html_escape(signal), html_escape(tab_id), receipt.occurred_at,
        html_escape(tab_id), html_escape(message), html_escape(tab_id),
    );
    let mut response = (status, [("x-coronatio-fault", "cartridge-fragment")], Html(body)).into_response();
    response.headers_mut().insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    response.headers_mut().insert(header::CONTENT_SECURITY_POLICY, HeaderValue::from_static(CROWN_CONTENT_SECURITY_POLICY));
    response
}

fn no_store(mut response: Response) -> Response {
    response.headers_mut().insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    response
}

fn render_og_pane_fragment(tab_id: &str, session: Session) -> String {
    let shell = render_crown_shell_for_session(session);
    let fragment = extract_pane_inner_html(&shell, tab_id).unwrap_or_else(|| {
        record_cartridge_fault(tab_id, CartridgeFaultKind::TabNotFound);
        format!(
            r#"<div hidden data-cartridge-fault="true" data-cartridge-fault-kind="tab-not-found" data-tab-id="{}"></div>"#,
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
    let call = crate::caduceus_access::CaduceusAccessClient::default().attendance_validate(&attendance, &document);
    if !call.receipt.ok { crate::caduceus_access::bust_attendance_projection(&attendance, &document, "validation-refused"); }
    call.receipt.ok.then_some(Session::Admin).unwrap_or(Session::Guest)
}

async fn session_projection_from_headers(headers: &axum::http::HeaderMap) -> Session {
    let (Some(document), Some(attendance)) = (crate::caduceus_access::document_incarnation_from_headers(headers), crate::caduceus_access::attendance_from_headers(headers)) else { return Session::Guest; };
    crate::caduceus_access::attendance_projection_call(attendance, document).await.receipt.ok.then_some(Session::Admin).unwrap_or(Session::Guest)
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
    let call = crate::caduceus_access::CaduceusAccessClient::default().attendance_open_async(pin.to_string(), document.clone()).await;
    let status = attendance_failure_status(&call);
    attendance_projection_response(&headers, ROUTE, status, document_admission_projection(call), Some(&document))
}

async fn caduceus_attendance_validate_route(headers: axum::http::HeaderMap) -> Response {
    const ROUTE: &str = "/api/v1/attendance/validate";
    let Some(document) = crate::caduceus_access::document_incarnation_from_headers(&headers) else { return attendance_projection_response(&headers, ROUTE, StatusCode::BAD_REQUEST, guest_session_projection("caduceus-attendance-document-required"), None); };
    let Some(attendance) = crate::caduceus_access::attendance_from_headers(&headers) else { return attendance_projection_response(&headers, ROUTE, StatusCode::UNAUTHORIZED, guest_session_projection("caduceus-attendance-required"), Some(&document)); };
    let call = crate::caduceus_access::attendance_projection_call(attendance, document.clone()).await;
    let status = attendance_failure_status(&call);
    attendance_projection_response(&headers, ROUTE, status, session_projection(call), Some(&document))
}

async fn caduceus_attendance_touch_route(headers: axum::http::HeaderMap) -> Response {
    const ROUTE: &str = "/api/v1/attendance/touch";
    let Some(document) = crate::caduceus_access::document_incarnation_from_headers(&headers) else { return attendance_projection_response(&headers, ROUTE, StatusCode::BAD_REQUEST, guest_session_projection("caduceus-attendance-document-required"), None); };
    let Some(attendance) = crate::caduceus_access::attendance_from_headers(&headers) else { return attendance_projection_response(&headers, ROUTE, StatusCode::UNAUTHORIZED, guest_session_projection("caduceus-attendance-required"), Some(&document)); };
    let call = crate::caduceus_access::CaduceusAccessClient::default().attendance_touch_and_bust_async(attendance, document.clone()).await;
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
    let call = crate::caduceus_access::CaduceusAccessClient::default().attendance_change_pin_and_bust_async(attendance, document.clone(), current_pin.to_string(), new_pin.to_string()).await;
    let status = attendance_failure_status(&call);
    attendance_projection_response(&headers, ROUTE, status, session_projection(call), Some(&document))
}

async fn caduceus_attendance_invalidate_route(headers: axum::http::HeaderMap) -> Response {
    const ROUTE: &str = "/api/v1/attendance/invalidate";
    let Some(document) = crate::caduceus_access::document_incarnation_from_headers(&headers) else { return attendance_projection_response(&headers, ROUTE, StatusCode::BAD_REQUEST, guest_session_projection("caduceus-attendance-document-required"), None); };
    let Some(attendance) = crate::caduceus_access::attendance_from_headers(&headers) else { return attendance_projection_response(&headers, ROUTE, StatusCode::UNAUTHORIZED, guest_session_projection("caduceus-attendance-required"), Some(&document)); };
    crate::caduceus_access::fence_attendance_projection(&attendance, &document);
    let call = crate::caduceus_access::CaduceusAccessClient::default().attendance_invalidate_and_bust_async(attendance, document.clone()).await;
    let invalidated = call.receipt.ok;
    pulse::downgrade_document(&document);
    indicators::downgrade_core_document(&document);
    let status = attendance_failure_status(&call);
    let projection = if invalidated { guest_session_projection("none") } else { session_projection(call) };
    attendance_projection_response(&headers, ROUTE, status, projection, Some(&document))
}
