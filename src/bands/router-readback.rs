async fn topics_route() -> impl IntoResponse {
    Json(topic_catalog_readback())
}

async fn monitor_pulse_route() -> impl IntoResponse {
    Json(monitor_pulse_readback())
}

async fn service_data_route() -> impl IntoResponse {
    Json(service_data_readback())
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

async fn stats_events_route() -> impl IntoResponse {
    let event = stats_event_payload();
    let payload = serde_json::to_string(&event).expect("serialize stats event");
    let body = format!(
        "event: stats.system\nid: {}\ndata: {}\n\n",
        event.event_id, payload
    );
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/event-stream")
        .header(header::CACHE_CONTROL, "no-cache")
        .header("x-coronatio-schema", "coronatio.stats.events.v1")
        .body(body)
        .expect("build stats event response")
}

async fn stats_events_renew_route() -> impl IntoResponse {
    Json(LeaseRenewalReadback {
        schema: "coronatio.stats.events.renewal.v1".to_string(),
        topic: "stats.system".to_string(),
        route: "/api/stats/events/renew".to_string(),
        lease_seconds: 30,
        status: "renewed-contract".to_string(),
        next_renewal_before_seconds: 20,
    })
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
    while receipts.len() >= CARTRIDGE_FAULT_RECEIPT_CAPACITY {
        receipts.pop_front();
    }
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

async fn admit_tab_route(State(state): State<AppState>, headers: HeaderMap, Path(tab_id): Path<String>) -> impl IntoResponse {
    let mut response = if !is_safe_tab_id(&tab_id) {
        fragment_fault(StatusCode::BAD_REQUEST, &tab_id, CartridgeFaultKind::UpstreamError)
    } else if let Some(pane) = native_crown_panes().into_iter().find(|pane| pane.id == tab_id) {
        Html(render_native_pane_fragment(&pane)).into_response()
    } else {
        match load_tab_manifest(&state.tab_root, &tab_id).await {
            Ok(Some(manifest)) => admit_registry_manifest(&state, &headers, manifest).await,
            Ok(None) => fragment_fault(StatusCode::NOT_FOUND, &tab_id, CartridgeFaultKind::TabNotFound),
            Err(_error) => fragment_fault(StatusCode::INTERNAL_SERVER_ERROR, &tab_id, CartridgeFaultKind::UpstreamError),
        }
    };
    response.headers_mut().insert(header::CACHE_CONTROL, header::HeaderValue::from_static("no-store"));
    response
}

async fn admit_registry_manifest(state: &AppState, headers: &HeaderMap, manifest: TabManifest) -> Response {
    match manifest.client_class {
        ClientClass::Fragment => {
            if let Some(service_url) = manifest.service_url.as_deref().filter(|value| !value.trim().is_empty()) {
                let url = format!("{}{}", service_url.trim_end_matches('/'), manifest.fragment_path.as_str());
                return fetch_cartridge_fragment(&manifest.id, &url).await;
            }
            read_static_cartridge_fragment(state, &manifest).await
        }
        ClientClass::Iframe => Html(render_iframe_guest_fragment_for_request(&manifest, headers)).into_response(),
    }
}

fn iframe_guest_src(manifest: &TabManifest) -> String {
    if let Some(service_url) = manifest.service_url.as_deref().filter(|value| !value.trim().is_empty()) {
        return format!("{}{}", service_url.trim_end_matches('/'), manifest.fragment_path.as_str());
    }
    format!("/tabs/{}/{}", manifest.id, manifest.fragment_path.trim_start_matches('/'))
}

fn render_iframe_guest_fragment(manifest: &TabManifest) -> String {
    render_iframe_guest_fragment_with_crown_origin(manifest, None)
}

fn render_iframe_guest_fragment_for_request(manifest: &TabManifest, headers: &HeaderMap) -> String {
    render_iframe_guest_fragment_with_crown_origin(manifest, crown_origin_from_headers(headers))
}

fn render_iframe_guest_fragment_with_crown_origin(manifest: &TabManifest, crown_origin: Option<String>) -> String {
    let title = if manifest.title.trim().is_empty() { &manifest.id } else { &manifest.title };
    let src = iframe_guest_src(manifest);
    let sandbox = iframe_guest_sandbox(&src, crown_origin.as_deref());
    maud::html! {
        article .crown-fragment .crown-iframe-guest data-fragment-schema="coronatio.iframe-guest.v1" data-reference-cartridge=(manifest.id) data-client-class="iframe" {
            header .crown-iframe-guest__chrome {
                h2 { (title) }
                span .crown-iframe-guest__chip { "iframe guest" }
            }
            iframe
                class="crown-iframe-guest__frame"
                title=(title)
                src=(src)
                sandbox=(sandbox)
                referrerpolicy="no-referrer" {}
        }
    }.into_string()
}

fn crown_origin_from_headers(headers: &HeaderMap) -> Option<String> {
    let host = headers.get(header::HOST)?.to_str().ok()?.trim();
    if host.is_empty() {
        return None;
    }
    let scheme = headers
        .get("x-forwarded-proto")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.split(',').next())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("http");
    Some(format!("{}://{}", scheme, host))
}

fn iframe_guest_sandbox(src: &str, crown_origin: Option<&str>) -> &'static str {
    if iframe_src_is_same_origin(src, crown_origin) {
        "allow-scripts allow-forms"
    } else {
        "allow-scripts allow-same-origin allow-forms"
    }
}

fn iframe_src_is_same_origin(src: &str, crown_origin: Option<&str>) -> bool {
    if src.starts_with('/') {
        return true;
    }
    let Some(crown_origin) = crown_origin else {
        return false;
    };
    let Ok(src_uri) = src.parse::<Uri>() else {
        return true;
    };
    let Ok(crown_uri) = crown_origin.parse::<Uri>() else {
        return true;
    };
    src_uri.scheme_str() == crown_uri.scheme_str() && src_uri.authority() == crown_uri.authority()
}

async fn fetch_cartridge_fragment(tab_id: &str, url: &str) -> Response {
    let client = match reqwest::Client::builder().timeout(Duration::from_secs(2)).build() {
        Ok(client) => client,
        Err(_error) => return fragment_fault(StatusCode::INTERNAL_SERVER_ERROR, tab_id, CartridgeFaultKind::UpstreamError),
    };
    match client.get(url).send().await {
        Ok(response) => {
            let status = StatusCode::from_u16(response.status().as_u16()).unwrap_or(StatusCode::BAD_GATEWAY);
            match response.text().await {
                Ok(body) if status.is_success() => Html(body).into_response(),
                Ok(_body) => fragment_fault(status, tab_id, CartridgeFaultKind::UpstreamError),
                Err(_error) => fragment_fault(StatusCode::BAD_GATEWAY, tab_id, CartridgeFaultKind::UpstreamError),
            }
        }
        Err(error) => {
            let fault_kind = if error.is_timeout() {
                CartridgeFaultKind::Timeout
            } else {
                CartridgeFaultKind::ProxyUnreachable
            };
            fragment_fault(StatusCode::BAD_GATEWAY, tab_id, fault_kind)
        },
    }
}

async fn read_static_cartridge_fragment(state: &AppState, manifest: &TabManifest) -> Response {
    let fragment_rel = manifest.fragment_path.trim_start_matches('/');
    let path = state.tab_root.join(&manifest.id).join(fragment_rel);
    match fs::read_to_string(&path).await {
        Ok(body) => Html(body).into_response(),
        Err(_error) => fragment_fault(StatusCode::BAD_GATEWAY, &manifest.id, CartridgeFaultKind::UpstreamError),
    }
}

fn fragment_fault(status: StatusCode, tab_id: &str, fault_kind: CartridgeFaultKind) -> Response {
    let receipt = record_cartridge_fault(tab_id, fault_kind);
    let fault = receipt.fault_kind.as_str();
    let body = maud::html! {
        section data-cartridge-fault="true" data-cartridge-fault-kind=(fault) data-tab-id=(tab_id) data-cartridge-fault-occurred-at=(receipt.occurred_at) {
            h2 { "Cartridge fault" }
            p { "The crown kept the underlay standing while this fragment failed admission." }
        }
    }.into_string();
    (
        status,
        [("x-coronatio-fault", "cartridge-fragment")],
        Html(body),
    ).into_response()
}

fn render_native_pane_fragment(pane: &CrownPane) -> String {
    match pane.id.as_str() {
        "stats" => render_stats_fragment(stats_snapshot()),
        "portals" => render_portals_fragment(read_portals_config().unwrap_or_else(|signal| PortalConfigResponse {
            schema: "coronatio.portals.config.v1".to_string(),
            route: "/api/portals".to_string(),
            success: false,
            source: homeserver_config_candidates().into_iter().map(|p| p.display().to_string()).collect::<Vec<_>>().join(" | "),
            factory_source: None,
            portals: Vec::new(),
            factory_portals: Vec::new(),
            first_missing_signal: signal,
        })),
        "upload" => render_upload_fragment(serde_json::json!({"schema":"coronatio.upload.history.v1","ok":true,"history":[],"firstMissingSignal":"none"})),
        "admin" => render_admin_fragment(admin_session_readback()),
        "testtab" => render_testtab_token_lab(),
        _ => render_json_fragment(&pane.title, "coronatio.native.fragment.v1", serde_json::to_value(pane).unwrap_or(serde_json::Value::Null)),
    }
}

fn json_pretty(value: &serde_json::Value) -> String {
    serde_json::to_string_pretty(value).unwrap_or_else(|_| "{}".to_string())
}

fn render_readback_details(schema: &str, value: &serde_json::Value) -> maud::Markup {
    let pretty = json_pretty(value);
    maud::html! {
        details.crown-readback data-crown-readback="true" {
            summary { "Raw JSON readback" }
            pre data-native-readback="json" data-fragment-schema=(schema) { (pretty) }
        }
    }
}

fn format_optional_percent(value: Option<u8>) -> String {
    value.map(|percent| format!("{percent}%")).unwrap_or_else(|| "n/a".to_string())
}

fn format_optional_f64(value: Option<f64>, suffix: &str) -> String {
    value.map(|number| format!("{number:.1}{suffix}")).unwrap_or_else(|| "n/a".to_string())
}

fn format_bytes(value: Option<u64>) -> String {
    let Some(bytes) = value else { return "n/a".to_string(); };
    const GIB: f64 = 1024.0 * 1024.0 * 1024.0;
    const MIB: f64 = 1024.0 * 1024.0;
    if bytes as f64 >= GIB {
        format!("{:.1} GiB", bytes as f64 / GIB)
    } else if bytes as f64 >= MIB {
        format!("{:.1} MiB", bytes as f64 / MIB)
    } else {
        format!("{} B", bytes)
    }
}

fn render_portals_fragment(response: PortalConfigResponse) -> String {
    let value = serde_json::to_value(&response).unwrap_or(serde_json::Value::Null);
    maud::html! {
        article.crown-fragment.crown-fragment--portals data-fragment-schema="coronatio.portals.fragment.v1" data-crown-block-shape="portal-card-row" {
            header.crown-status-strip {
                div { h2 { "Portals" } p { "Crown service ingress from homeserver.json." } }
                div.crown-chip-row {
                    span.crown-chip { (if response.success { "config read" } else { "config fallback" }) }
                    span.crown-chip data-posture="muted" { (response.portals.len()) " portal cards" }
                }
            }
            section.crown-block-grid aria-label="Portal cards" {
                @if response.portals.is_empty() {
                    article.crown-block-card { h3 { "No visible portals" } p { (response.first_missing_signal.clone()) } }
                } @else {
                    @for portal in &response.portals {
                        article.crown-block-card data-portal-name=(portal.name) {
                            div.crown-card-head { h3 { (portal.name) } span.crown-chip { (if portal.visible { "visible" } else { "hidden" }) } }
                            p { (portal.description) }
                            div.crown-link-row {
                                a href=(portal.local_url) { "local" }
                                @if let Some(remote) = &portal.remote_url { a href=(remote) { "remote" } }
                            }
                            dl.crown-definition-grid {
                                div.crown-definition-row { dt { "port" } dd { (portal.port.map(|port| port.to_string()).unwrap_or_else(|| "n/a".to_string())) } }
                                div.crown-definition-row { dt { "type" } dd { (portal.r#type) } }
                                div.crown-definition-row { dt { "services" } dd { (if portal.services.is_empty() { "none".to_string() } else { portal.services.join(", ") }) } }
                            }
                        }
                    }
                }
            }
            (render_readback_details("coronatio.portals.fragment.v1", &value))
        }
    }.into_string()
}

fn render_stats_fragment(snapshot: StatsSnapshot) -> String {
    let value = serde_json::to_value(&snapshot).unwrap_or(serde_json::Value::Null);
    let disk = snapshot.storage.first();
    let disk_label = disk.map(|drive| format_optional_percent(drive.usage_percent)).unwrap_or_else(|| "n/a".to_string());
    let network_total = snapshot.network.interfaces.iter().map(|iface| iface.rx_bytes.saturating_add(iface.tx_bytes)).sum::<u64>();
    maud::html! {
        article.crown-fragment.crown-fragment--stats data-fragment-schema="coronatio.stats.fragment.v1" data-crown-block-shape="stat-workbench-grid" {
            header.crown-status-strip {
                div { h2 { "Stats" } p { "Honest /proc and host readbacks shaped as crown stat cards." } }
                span.crown-chip { (snapshot.transport.stream_status) }
            }
            section.crown-block-grid aria-label="Stat workbench" {
                article.crown-block-card data-stat-card="cpu" { span.crown-headline-number { (format_optional_f64(snapshot.resources.load.one, "")) } p { "CPU load" } }
                article.crown-block-card data-stat-card="memory" { span.crown-headline-number { (format_optional_percent(snapshot.resources.memory.percent)) } p { "memory used · " (format_bytes(snapshot.resources.memory.used_bytes)) } }
                article.crown-block-card data-stat-card="disk" { span.crown-headline-number { (disk_label) } p { "disk used" } }
                article.crown-block-card data-stat-card="network" { span.crown-headline-number { (format_bytes(Some(network_total))) } p { "network rx+tx" } }
                article.crown-block-card data-stat-card="connections" { span.crown-headline-number { (snapshot.network.connections.total) } p { "connections" } }
            }
            (render_readback_details("coronatio.stats.fragment.v1", &value))
        }
    }.into_string()
}

fn render_admin_fragment(readback: AdminSessionReadback) -> String {
    let value = serde_json::to_value(&readback).unwrap_or(serde_json::Value::Null);
    maud::html! {
        article.crown-fragment.crown-fragment--admin data-fragment-schema="coronatio.admin.fragment.v1" data-crown-block-shape="status-strip-admin-cards" {
            header.crown-status-strip {
                div { h2 { "Admin" } p { "Session state and mutation membrane posture." } }
                div.crown-chip-row { span.crown-chip { "session membrane" } span.crown-chip data-posture="muted" { (readback.caduceus_membrane.first_missing_signal.clone()) } }
            }
            section.crown-block-grid aria-label="Admin topics" {
                article.crown-block-card { h3 { "Session" } dl.crown-definition-grid {
                    div.crown-definition-row { dt { "PIN route" } dd { (readback.pin_validation) } }
                    div.crown-definition-row { dt { "timeout" } dd { (readback.session_timeout_seconds) " seconds" } }
                    div.crown-definition-row { dt { "keepalive" } dd { (readback.keepalive_route) } }
                    div.crown-definition-row { dt { "logout" } dd { (readback.logout_route) } }
                } }
                article.crown-block-card { h3 { "Caduceus" } dl.crown-definition-grid {
                    div.crown-definition-row { dt { "Coronatio" } dd { (readback.caduceus_membrane.coronatio_role) } }
                    div.crown-definition-row { dt { "Caduceus" } dd { (readback.caduceus_membrane.caduceus_role) } }
                    div.crown-definition-row { dt { "mutations" } dd { (readback.caduceus_membrane.privileged_mutations.join(", ")) } }
                } }
                article.crown-block-card { h3 { "Enhanced topics" } dl.crown-definition-grid {
                    @for filter in &readback.admin_enhanced_filtering {
                        div.crown-definition-row { dt { (filter.topic) } dd { (filter.admin_fields.join(", ")) } }
                    }
                } }
            }
            (render_readback_details("coronatio.admin.fragment.v1", &value))
        }
    }.into_string()
}

fn render_upload_fragment(value: serde_json::Value) -> String {
    maud::html! {
        article.crown-fragment.crown-fragment--upload data-fragment-schema="coronatio.upload.fragment.v1" data-crown-block-shape="single-ingress-card" {
            header.crown-status-strip { div { h2 { "Upload" } p { "Single file ingress through the Caduceus mutation membrane." } } span.crown-chip { "receipt lane" } }
            section.crown-block-grid aria-label="Upload ingress" {
                article.crown-block-card { h3 { "Ingress" } dl.crown-definition-grid {
                    div.crown-definition-row { dt { "route" } dd { "/api/files/upload" } }
                    div.crown-definition-row { dt { "posture" } dd { "multipart accepted by Coronatio; privileged storage posture remains behind Caduceus" } }
                    div.crown-definition-row { dt { "receipt" } dd { "coronatio.upload.history.v1" } }
                } }
            }
            (render_readback_details("coronatio.upload.fragment.v1", &value))
        }
    }.into_string()
}

fn render_testtab_token_lab() -> String {
    let value = serde_json::json!({"schema":"coronatio.testtab.token_lab.v1","tokens":["--ux-surface-0","--ux-surface-1","--ux-color-crown","--ux-color-leaf","--ux-color-sky","--ux-text","--ux-outline"],"blockShape":"token-lab"});
    maud::html! {
        article.crown-fragment.crown-fragment--testtab data-fragment-schema="coronatio.testtab.fragment.v1" data-crown-block-shape="token-lab" {
            header.crown-status-strip { div { h2 { "TestTab" } p { "Token lab: live crown --ux-* color, type, radius, and spacing samples." } } span.crown-chip { "--ux-*" } }
            section.crown-token-grid aria-label="Live UX token swatches" {
                article.crown-token-chip.token-surface-0 { strong { "surface 0" } code { "var(--ux-surface-0)" } }
                article.crown-token-chip.token-surface-1 { strong { "surface 1" } code { "var(--ux-surface-1)" } }
                article.crown-token-chip.token-crown { strong { "crown" } code { "var(--ux-color-crown)" } }
                article.crown-token-chip.token-leaf { strong { "leaf" } code { "var(--ux-color-leaf)" } }
                article.crown-token-chip.token-sky { strong { "sky" } code { "var(--ux-color-sky)" } }
                article.crown-token-chip.token-outline { strong { "outline" } code { "var(--ux-outline)" } }
            }
            section.crown-block-grid aria-label="Type spacing and radius samples" {
                article.crown-block-card.crown-type-sample { h3 { "Type scale" } span.crown-headline-number { "Crown" } p { "body text · small label" } }
                article.crown-block-card { h3 { "Spacing" } div.crown-chip-row { span.crown-chip { "space-2" } span.crown-chip { "space-3" } span.crown-chip { "space-4" } } }
                article.crown-block-card { h3 { "Radius" } div.crown-radius-samples { span.crown-radius-sample data-radius="sm" { "sm" } span.crown-radius-sample data-radius="md" { "md" } span.crown-radius-sample data-radius="lg" { "lg" } } }
            }
            (render_readback_details("coronatio.testtab.fragment.v1", &value))
        }
    }.into_string()
}

fn render_json_fragment(title: &str, schema: &str, value: serde_json::Value) -> String {
    maud::html! {
        article.crown-fragment data-fragment-schema=(schema) data-crown-block-shape="readback-card" {
            h2 { (title) }
            (render_readback_details(schema, &value))
        }
    }.into_string()
}

async fn homeserver_validate_pin_route(Json(body): Json<serde_json::Value>) -> impl IntoResponse {
    let supplied = body
        .get("pin")
        .or_else(|| body.get("password"))
        .and_then(serde_json::Value::as_str)
        .unwrap_or("");
    let configured = configured_admin_pin();
    let valid = configured.as_deref().map(|pin| pin == supplied).unwrap_or(false);
    (
        if valid { StatusCode::OK } else { StatusCode::UNAUTHORIZED },
        Json(serde_json::json!({
            "schema": "coronatio.homeserver.auth.pin.v1",
            "success": valid,
            "verified": valid,
            "valid": valid,
            "token": if valid { "coronatio-session-token" } else { "" },
            "expiresIn": 1800,
            "source": "/etc/homeserver.json global.admin.pin",
            "firstMissingSignal": if configured.is_some() { "none" } else { "homeserver-config-pin-missing" }
        })),
    )
}

async fn homeserver_logout_route() -> impl IntoResponse {
    Json(serde_json::json!({
        "schema": "coronatio.homeserver.auth.logout.v1",
        "success": true,
        "ok": true,
        "message": "session cleared by client"
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



fn configured_admin_pin() -> Option<String> {
    let text = std::fs::read_to_string("/etc/homeserver.json").ok()?;
    let value: serde_json::Value = serde_json::from_str(&text).ok()?;
    value
        .get("global")
        .and_then(|global| global.get("admin"))
        .and_then(|admin| admin.get("pin"))
        .and_then(serde_json::Value::as_str)
        .map(str::to_string)
}

