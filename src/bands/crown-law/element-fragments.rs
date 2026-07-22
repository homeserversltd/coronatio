#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ElementVisibilityRequest {
    tab_id: Option<String>,
    tab: Option<String>,
    element_id: Option<String>,
    element: Option<String>,
    id: Option<String>,
    visible: Option<bool>,
    visibility: Option<bool>,
}

async fn element_visibility_route(headers: axum::http::HeaderMap, Json(request): Json<ElementVisibilityRequest>) -> impl IntoResponse {
    if session_from_headers(&headers) != Session::Admin {
        return element_refusal_fragment("admin-session-required");
    }
    let tab = normalize_tab_id(&request.tab_id.or(request.tab).unwrap_or_default());
    let element = normalize_element_id_for_tab(&tab, &request.element_id.or(request.element).or(request.id).unwrap_or_default());
    if tab.is_empty() || element.is_empty() {
        return (StatusCode::BAD_REQUEST, Html(r#"<div data-element-visibility-refusal="invalid-element-request">invalid-element-request</div>"#.to_string())).into_response();
    }
    let visible = request.visible.or(request.visibility).unwrap_or(true);
    let (_source, facts) = match load_iris_facts().await {
        Ok(value) => value,
        Err(error) => return (StatusCode::INTERNAL_SERVER_ERROR, Html(format!(r#"<div data-element-visibility-refusal="load-failed">{}</div>"#, html_escape(&error)))).into_response(),
    };
    let next = iris::apply_element_visibility(&facts, &tab, &element, visible);
    let path = format!("tabs.{tab}.visibility.elements.{element}");
    let persisted = caduceus_config_set(&path, serde_json::Value::Bool(visible));
    if !persisted.ok {
        let status = if persisted.status == 0 { StatusCode::SERVICE_UNAVAILABLE } else { StatusCode::BAD_GATEWAY };
        return (status, Html(format!(r#"<div data-element-visibility-refusal="persist-failed" data-first-missing-signal="{}">{}</div>"#, html_escape(&persisted.first_missing_signal), html_escape(&persisted.first_missing_signal)))).into_response();
    }
    pulse::poke(pulse::PokeTopic::TabsChanged);
    element_fragment_response_from_facts(Session::Admin, &tab, &next)
}

async fn element_fragment_route(headers: axum::http::HeaderMap, Path(tab_id): Path<String>) -> impl IntoResponse {
    element_fragment_response(session_from_headers(&headers), &normalize_tab_id(&tab_id))
}

async fn stats_elements_fragment_route(headers: axum::http::HeaderMap) -> impl IntoResponse {
    element_fragment_response(session_from_headers(&headers), "stats")
}

async fn portals_elements_fragment_route(headers: axum::http::HeaderMap) -> impl IntoResponse {
    let host = request_access_host(&headers);
    element_fragment_response_with_host(session_from_headers(&headers), "portals", &host)
}

fn element_refusal_fragment(reason: &str) -> Response {
    (StatusCode::UNAUTHORIZED, Html(format!(r#"<div data-element-visibility-refusal="{}">{}</div>"#, html_escape(reason), html_escape(reason)))).into_response()
}

fn request_access_host(headers: &axum::http::HeaderMap) -> String {
    // Quarry PortalCard used window.location.hostname. Crown fragment uses request Host
    // (or X-Forwarded-Host) as the same access-context signal when reverse-proxied.
    headers
        .get("x-forwarded-host")
        .or_else(|| headers.get(axum::http::header::HOST))
        .and_then(|value| value.to_str().ok())
        .map(|value| value.split(',').next().unwrap_or(value).trim().to_string())
        .unwrap_or_default()
}

fn element_fragment_response(session: Session, tab: &str) -> Response {
    element_fragment_response_with_host(session, tab, "")
}

fn element_fragment_response_from_facts(session: Session, tab: &str, facts: &IrisFacts) -> Response {
    let body = match tab {
        "stats" => render_stats_elements_fragment_from_facts(session, facts),
        "portals" => render_portals_elements_fragment_from_facts(session, "", facts),
        _ => String::new(),
    };
    (StatusCode::OK, Html(body)).into_response()
}

fn element_fragment_response_with_host(session: Session, tab: &str, host: &str) -> Response {
    let body = match tab {
        "stats" => render_stats_elements_fragment(session),
        "portals" => render_portals_elements_fragment(session, host),
        _ => String::new(),
    };
    let mut response = (StatusCode::OK, Html(body)).into_response();
    response.headers_mut().insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    response.headers_mut().insert(header::CONTENT_SECURITY_POLICY, HeaderValue::from_static(CROWN_CONTENT_SECURITY_POLICY));
    response
}

fn normalize_element_id(raw: &str) -> String {
    raw.trim().chars().filter(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.')).collect()
}

fn normalize_element_id_for_tab(tab: &str, raw: &str) -> String {
    let id = normalize_element_id(raw);
    if tab == "stats" {
        canonical_stats_element_id(&id).to_string()
    } else {
        id
    }
}

fn render_stats_elements_fragment(session: Session) -> String {
    let facts = load_iris_facts_sync().unwrap_or_else(|| iris::from_coronatio_contracts(&native_tab_contracts(), "stats"));
    render_stats_elements_fragment_from_facts(session, &facts)
}

fn render_stats_elements_fragment_from_facts(session: Session, facts: &IrisFacts) -> String {
    stat_element_templates()
        .into_iter()
        .filter_map(|(id, html)| render_stat_element_from_grant(session, facts, id, html))
        .collect::<Vec<_>>()
        .join("\n")
}

fn stat_element_templates() -> Vec<(&'static str, &'static str)> {
    const RAW: &str = r####"          <div class="stat-element" data-stat-element-id="cpu-chart" data-visible="true">
            <div class="stat-header"><button type="button" class="visibility-toggle" data-admin-only="true" data-admin-viewport="stats" data-stat-visibility-toggle="cpu-chart" data-visible="true" aria-label="Hide CPU Usage & Load">👁</button><h3 class="stat-title">CPU Usage &amp; Load</h3></div>
            <div class="stat-content"><div class="cpu-stats-container"><div class="cpu-chart" data-chartjs-chart="cpu" data-chart-authority="serverbox-original-homeserver-stats"><div class="chart-container" id="cpu-chart-container"><canvas id="cpuChart" class="coronatio-chart-canvas" data-full-width-canvas="true" data-chart-left-axis="percent-suffix" data-chart-right-axis="celsius-suffix"></canvas></div></div><div class="load-averages"><div class="load-average-values"><div class="load-average-item"><span class="load-label">1 min:</span><span class="load-value" id="load-1min">—</span></div><div class="load-average-item"><span class="load-label">5 min:</span><span class="load-value" id="load-5min">—</span></div><div class="load-average-item"><span class="load-label">15 min:</span><span class="load-value" id="load-15min">—</span></div></div></div></div></div>
          </div>
          <div class="stat-element" data-stat-element-id="network-chart" data-visible="true">
            <div class="stat-header"><button type="button" class="visibility-toggle" data-admin-only="true" data-admin-viewport="stats" data-stat-visibility-toggle="network-chart" data-visible="true" aria-label="Hide Network Traffic (WAN)">👁</button><h3 class="stat-title">Network Traffic (WAN)</h3></div>
            <div class="stat-content"><div class="network-stats-container"><div class="network-speed-chart" data-chartjs-chart="network" data-chart-authority="serverbox-original-homeserver-stats"><div class="chart-container" id="network-chart-container"><canvas id="networkChart" class="coronatio-chart-canvas" data-full-width-canvas="true" data-chart-left-axis="byte-rate-suffix" data-chart-right-axis="byte-rate-suffix" data-synchronized-axes="true"></canvas></div></div><div class="network-interfaces ui-table"><table class="network-interfaces-table"><thead><tr><th>Interface</th><th>Total Received</th><th>Total Sent</th></tr></thead><tbody data-network-interfaces></tbody></table></div></div></div>
          </div>
          <div class="stat-element" data-stat-element-id="io-section" data-visible="true">
            <div class="stat-header"><button type="button" class="visibility-toggle" data-admin-only="true" data-admin-viewport="stats" data-stat-visibility-toggle="io-section" data-visible="true" aria-label="Hide Disk I/O">👁</button><h3 class="stat-title">Disk I/O</h3></div>
            <div class="stat-content"><div class="disk-io-chart"><div class="device-controls" id="io-drive-selector" data-device-controls data-original-control="drive-checkbox"></div><div class="chart-container" id="disk-io-chart-container"><canvas id="io-chart" class="coronatio-chart-canvas" data-full-width-canvas="true"></canvas></div><div class="io-chart-legend" id="io-chart-legend"></div></div></div>
          </div>
          <div class="stat-element" data-stat-element-id="memory-usage" data-visible="true">
            <div class="stat-header"><button type="button" class="visibility-toggle" data-admin-only="true" data-admin-viewport="stats" data-stat-visibility-toggle="memory-usage" data-visible="true" aria-label="Hide Memory Usage">👁</button><h3 class="stat-title">Memory Usage</h3></div>
            <div class="stat-content"><div class="memory-stats"><div class="memory-current"><div class="memory-label">RAM</div><div class="memory-bar ui-progress-bar__container"><div class="memory-bar-fill ui-progress-bar__fill ui-progress-bar__fill--memory" id="memory-bar-fill"><span class="memory-text ui-progress-bar__text" id="memory-percent">—</span></div></div><div class="memory-details"><div id="memory-used">Used: —</div><div id="memory-available">Available: —</div><div id="memory-total">Total: —</div></div></div><div class="memory-current"><div class="memory-label">Swap</div><div class="memory-bar ui-progress-bar__container"><div class="memory-bar-fill memory-bar-fill-swap ui-progress-bar__fill ui-progress-bar__fill--swap" id="swap-bar-fill"><span class="memory-text ui-progress-bar__text" id="swap-percent">—</span></div></div><div class="memory-details"><div id="swap-used">Used: —</div><div id="swap-free">Free: —</div><div id="swap-total">Total: —</div></div></div></div></div>
          </div>
          <div class="stat-element" data-stat-element-id="disk-usage" data-visible="true">
            <div class="stat-header"><button type="button" class="visibility-toggle" data-admin-only="true" data-admin-viewport="stats" data-stat-visibility-toggle="disk-usage" data-visible="true" aria-label="Hide Disk Usage">👁</button><h3 class="stat-title">Disk Usage</h3></div>
            <div class="stat-content"><div class="disk-usage-stats" data-disk-usage-stats></div></div>
          </div>
          <div class="stat-element" data-stat-element-id="kea-leases" data-visible="true">
            <div class="stat-header"><button type="button" class="visibility-toggle" data-admin-only="true" data-admin-viewport="stats" data-stat-visibility-toggle="kea-leases" data-visible="true" aria-label="Hide DHCP Leases">👁</button><h3 class="stat-title">DHCP Leases</h3></div>
            <div class="stat-content"><div class="kea-leases-table ui-table ui-table--responsive"><table><thead><tr><th>Device Note</th><th>Hostname</th><th>IP Address</th><th>MAC Address</th></tr></thead><tbody data-kea-leases><tr><td colspan="4"><span class="loading-spinner medium" role="progressbar" aria-label="Loading Kea leases"></span>Loading Kea leases...</td></tr></tbody></table></div></div>
          </div>
          <div class="stat-element" data-stat-element-id="process-usage" data-visible="true">
            <div class="stat-header"><button type="button" class="visibility-toggle" data-admin-only="true" data-admin-viewport="stats" data-stat-visibility-toggle="process-usage" data-visible="true" aria-label="Hide CPU Usage by Process">👁</button><h3 class="stat-title">CPU Usage by Process</h3></div>
            <div class="stat-content"><div class="process-usage-list" data-process-usage-list><p>Loading process usage...</p></div></div>
          </div>
"####;
    const IDS: [&str; 7] = ["cpu-chart", "network-chart", "io-section", "memory-usage", "disk-usage", "kea-leases", "process-usage"];
    IDS.iter()
        .filter_map(|id| extract_stat_element_template(RAW, id).map(|html| (*id, html)))
        .collect()
}

fn extract_stat_element_template(raw: &'static str, id: &str) -> Option<&'static str> {
    let needle = format!(r#"<div class="stat-element" data-stat-element-id="{}""#, id);
    let start = raw.find(&needle)?;
    let rest = &raw[start + needle.len()..];
    let next_rel = rest.find(r#"<div class="stat-element" data-stat-element-id=""#);
    Some(match next_rel {
        Some(offset) => &raw[start..start + needle.len() + offset],
        None => &raw[start..],
    })
}

fn render_stat_element_from_grant(session: Session, facts: &IrisFacts, element_id: &str, template: &str) -> Option<String> {
    let grant = default_element_grant_from_facts(facts, session, "stats", element_id);
    if grant.state == RenderState::Absent { return None; }
    let visible = grant.state == RenderState::Visible;
    let eye = if visible { "fa-eye" } else { "fa-eye-slash" };
    let verb = if visible { "Hide" } else { "Show" };
    let mut html = template.to_string();
    html = replace_data_visible(&html, visible);
    html = replace_visibility_toggle(&html, "data-stat-visibility-toggle", element_id, visible, eye, verb);
    Some(html)
}

fn render_portals_elements_fragment(session: Session, host: &str) -> String {
    let facts = load_iris_facts_sync().unwrap_or_else(|| iris::from_coronatio_contracts(&native_tab_contracts(), "stats"));
    render_portals_elements_fragment_from_facts(session, host, &facts)
}

fn render_portals_elements_fragment_from_facts(session: Session, host: &str, facts: &IrisFacts) -> String {
    let plan = iris::plan(facts, session);
    match read_portals_config() {
        Ok(response) => {
            let factory_portals = response.factory_portals;
            let mut html = response.portals.into_iter()
                .filter_map(|portal| render_portal_element_from_grant(session, facts, &plan, &portal, host, &factory_portals))
                .collect::<Vec<_>>()
                .join("\n");
            if html.is_empty() {
                html.push_str(r#"<article class="portal-card portal-empty"><h2>No portals configured</h2><p>homeserver.json has no visible portal entries.</p></article>"#);
            }
            if session == Session::Admin { html.push_str(&render_add_portal_card_fragment()); }
            html
        }
        Err(_) => {
            let mut html = r#"<article class="portal-card error portal-error"><h2>Portals unavailable</h2><p>homeserver.json could not be read.</p></article>"#.to_string();
            if session == Session::Admin { html.push_str(&render_add_portal_card_fragment()); }
            html
        }
    }
}

fn portal_access_is_remote(host: &str) -> bool {
    // One-to-one with homeserver PortalCard.tsx isRemoteAccess().
    let hostname = host.split(':').next().unwrap_or(host).trim().to_ascii_lowercase();
    if hostname.is_empty() { return false; }
    hostname.contains(".ts.net") || (!hostname.contains(".home.arpa") && hostname != "home.arpa" && hostname != "localhost" && hostname != "127.0.0.1")
}

fn construct_portal_destination(portal: &PortalEntry, host: &str) -> String {
    // Quarry: remoteURL is phased out; dynamic calculation from localURL/port + access host.
    // Stored slash-path remoteURL (e.g. https://home.tail…/jellyfin/) is NOT the product link.
    if !portal_access_is_remote(host) { return portal.local_url.clone(); }
    let hostname = host.split(':').next().unwrap_or(host).trim();
    if portal.r#type == "link" {
        let local = portal.local_url.trim();
        if local.starts_with("http://") || local.starts_with("https://") {
            if !local.contains(".home.arpa") && !local.contains(".ts.net") { return local.to_string(); }
            if hostname.contains(".ts.net") {
                if let Some(caps) = regex_tailnet(hostname) {
                    let after = local.split("://").nth(1).unwrap_or(local);
                    let path = after.find('/').map(|i| &after[i..]).unwrap_or("/");
                    return format!("https://home.{caps}.ts.net{path}");
                }
            }
        }
        if hostname.contains(".ts.net") {
            if let Some(caps) = regex_tailnet(hostname) {
                let path = local.replacen("https://", "", 1);
                let path = if let Some(idx) = path.find('/') { &path[idx..] } else { "" };
                let path = if path.is_empty() { "/" } else { path };
                return format!("https://home.{caps}.ts.net{path}");
            }
        }
        return portal.local_url.clone();
    }
    if hostname.contains(".ts.net") {
        if let Some(caps) = regex_tailnet(hostname) {
            if let Some(port) = portal.port { return format!("https://home.{caps}.ts.net:1{port}/"); }
        }
    }
    String::new()
}

fn regex_tailnet(hostname: &str) -> Option<String> {
    let parts: Vec<&str> = hostname.split('.').collect();
    if parts.len() >= 4 && parts[0] == "home" && parts[parts.len()-2] == "ts" && parts[parts.len()-1] == "net" { return Some(parts[1].to_string()); }
    None
}

fn render_portal_element_from_grant(session: Session, facts: &IrisFacts, plan: &RenderPlan, portal: &PortalEntry, host: &str, factory_portals: &[String]) -> Option<String> {
    let element_id = portal.name.trim();
    if element_id.is_empty() || portal.local_url.trim().is_empty() { return None; }
    let grant = plan.elements.iter().find(|grant| grant.tab_id == "portals" && grant.element_id == element_id)
        .cloned()
        .unwrap_or_else(|| default_element_grant_from_facts(facts, session, "portals", element_id));
    if grant.state == RenderState::Absent { return None; }
    let destination_raw = construct_portal_destination(portal, host);
    if destination_raw.trim().is_empty() && portal_access_is_remote(host) { return None; }
    let visible = grant.state == RenderState::Visible;
    let eye = if visible { "fa-eye" } else { "fa-eye-slash" };
    let verb = if visible { "Hide" } else { "Show" };
    let name = html_escape(element_id);
    let description = html_escape(&portal.description);
    let destination = html_escape(if destination_raw.trim().is_empty() { &portal.local_url } else { &destination_raw });
    let status = derive_portal_currentness(portal);
    let services = html_escape(&serde_json::to_string(&portal.services).unwrap_or_else(|_| "[]".to_string()));
    // Quarry: only systemd portals expose direct service controls. Script portals
    // remain plainly actionable only through their system restart notice.
    let admin_controls = if session == Session::Admin && portal.r#type == "systemd" {
        format!(r#"<div class="admin-controls" data-admin-only data-admin-viewport="portals" data-portal-services="{}"><div class="admin-controls-row"><button data-service-action="start" title="Start service">Start</button><button data-service-action="stop" title="Stop service">Stop</button><button data-service-action="restart" title="Restart service">Restart</button></div><div class="admin-controls-row"><button data-service-action="enable" title="Enable service at boot">Enable</button><button data-service-action="disable" title="Disable service at boot">Disable</button><button data-service-action="status" title="Check service status">Status</button></div></div>"#, services)
    } else if session == Session::Admin && portal.r#type == "script" {
        r#"<div class="admin-controls script-management-notice" data-admin-only data-admin-viewport="portals"><span class="script-notice-text" title="System restart required for changes to take effect. Script-managed services are controlled through system scripts rather than direct service commands.">Script-managed Service — system restart required</span></div>"#.to_string()
    } else { String::new() };
    let delete = if session == Session::Admin && !factory_portals.iter().any(|factory| factory == element_id) {
        format!(r#"<button type="button" class="delete-portal-button" data-admin-only data-admin-viewport="portals" data-portal-delete data-portal-name="{}" title="Delete portal" aria-label="Delete {}"><i class="fas fa-trash" aria-hidden="true"></i></button>"#, name, name)
    } else { String::new() };
    let toggle = if session == Session::Admin {
        format!(r#"<button type="button" class="visibility-toggle ui-visibility-toggle" data-admin-only data-admin-viewport="portals" data-portal-visibility-toggle="{}" data-visible="{}" aria-pressed="{}" aria-label="{} {}"><i class="fas {}" aria-hidden="true"></i></button>"#, name, visible, visible, verb, name, eye)
    } else { String::new() };
    let portal_name = if session == Session::Admin {
        String::new()
    } else {
        format!(r#"<h2 class="portal-name">{}</h2>"#, name)
    };
    Some(format!(r#"<div class="portal-element" data-portal-element data-visible="{}" style="position:relative">{}{}<article class="portal-card {}" data-portal-card data-portal-name="{}" data-portal-url="{}" data-portal-services="{}" role="link" tabindex="0"><div class="portal-card-face"><div class="portal-card-header"><img src="/api/portals/images/{}.png" alt="{} icon" class="portal-icon" onerror="this.onerror=null;this.src='/api/portals/images/default.png';">{}<p class="portal-description">{}</p></div><div class="portal-meta">{}</div></div></article></div>"#, visible, toggle, delete, status, name, destination, services, name, name, portal_name, description, admin_controls))
}

fn render_add_portal_card_fragment() -> String {
    r#"<div class="portal-card add-portal-card" data-admin-only data-admin-viewport="portals" data-add-portal-open role="button" tabindex="0" aria-label="Add new portal"><div class="portal-card-face"><div class="add-portal-content"><div class="add-portal-icon"><i class="fas fa-plus"></i></div><h3 class="add-portal-title">Add Portal</h3><p class="add-portal-description">Create a new portal for your services</p></div></div></div>"#.to_string()
}

fn default_element_grant_from_facts(facts: &IrisFacts, session: Session, tab_id: &str, element_id: &str) -> ElementGrant {
    let tab = facts.tabs.iter().find(|tab| tab.id == tab_id);
    let tab_visible = tab.and_then(|tab| tab.visibility_tab).unwrap_or(false);
    let element_visible = tab
        .and_then(|tab| tab.elements.iter().find(|element| element.id == element_id))
        .and_then(|element| element.visibility)
        .unwrap_or(true);
    default_element_grant(tab_visible, element_visible, session, tab_id, element_id)
}

fn default_element_grant(tab_visible: bool, element_visible: bool, session: Session, tab_id: &str, element_id: &str) -> ElementGrant {
    let state = match session {
        Session::Guest if tab_visible && element_visible => RenderState::Visible,
        Session::Guest => RenderState::Absent,
        Session::Admin if tab_visible && element_visible => RenderState::Visible,
        Session::Admin => RenderState::DimmedHidden,
    };
    ElementGrant {
        key: format!("{}/element:{}", tab_id, element_id),
        tab_id: tab_id.to_string(),
        element_id: element_id.to_string(),
        state,
        eye: session == Session::Admin,
    }
}

fn replace_data_visible(html: &str, visible: bool) -> String {
    let replacement = format!(r#"data-visible="{}""#, visible);
    html.replacen(r#"data-visible="true""#, &replacement, 1).replacen(r#"data-visible="false""#, &replacement, 1)
}

fn replace_visibility_toggle(html: &str, attr: &str, element_id: &str, visible: bool, eye: &str, verb: &str) -> String {
    let mut next = html.to_string();
    let attr_old = format!(r#"{}="{}""#, attr, element_id);
    if !next.contains(&attr_old) { return next; }
    for old_visible in [true, false] {
        let old = format!(r#"{}="{}" data-visible="{}""#, attr, element_id, old_visible);
        let new = format!(r#"{}="{}" data-visible="{}""#, attr, element_id, visible);
        next = next.replace(&old, &new);
    }
    next = next.replace("class=\"visibility-toggle\"", "class=\"visibility-toggle ui-visibility-toggle\"");
    next = next.replace(">👁</button>", &format!(" aria-pressed=\"{}\"><i class=\"fas {}\" aria-hidden=\"true\"></i></button>", visible, eye));
    next = next.replace("Hide ", &format!("{} ", verb));
    next
}
