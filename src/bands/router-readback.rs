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

async fn admit_tab_route(State(state): State<AppState>, Path(tab_id): Path<String>) -> impl IntoResponse {
    if !is_safe_tab_id(&tab_id) {
        return fragment_fault(StatusCode::BAD_REQUEST, &tab_id, "invalid-tab-id");
    }

    if let Some(pane) = native_crown_panes().into_iter().find(|pane| pane.id == tab_id) {
        return Html(render_native_pane_fragment(&pane)).into_response();
    }

    match load_tab_manifest(&state.tab_root, &tab_id).await {
        Ok(Some(manifest)) => admit_registry_manifest(&state, manifest).await,
        Ok(None) => fragment_fault(StatusCode::NOT_FOUND, &tab_id, "tab-not-found"),
        Err(error) => fragment_fault(StatusCode::INTERNAL_SERVER_ERROR, &tab_id, &format!("manifest-error:{error}")),
    }
}

async fn admit_registry_manifest(state: &AppState, manifest: TabManifest) -> Response {
    if manifest.client_class != ClientClass::Fragment {
        return fragment_fault(StatusCode::BAD_REQUEST, &manifest.id, "unsupported-client-class");
    }
    if let Some(service_url) = manifest.service_url.as_deref().filter(|value| !value.trim().is_empty()) {
        let url = format!("{}{}", service_url.trim_end_matches('/'), manifest.fragment_path.as_str());
        return fetch_cartridge_fragment(&manifest.id, &url).await;
    }
    read_static_cartridge_fragment(state, &manifest).await
}

async fn fetch_cartridge_fragment(tab_id: &str, url: &str) -> Response {
    let client = match reqwest::Client::builder().timeout(Duration::from_secs(2)).build() {
        Ok(client) => client,
        Err(error) => return fragment_fault(StatusCode::INTERNAL_SERVER_ERROR, tab_id, &format!("client-build:{error}")),
    };
    match client.get(url).send().await {
        Ok(response) => {
            let status = StatusCode::from_u16(response.status().as_u16()).unwrap_or(StatusCode::BAD_GATEWAY);
            match response.text().await {
                Ok(body) if status.is_success() => Html(body).into_response(),
                Ok(body) => fragment_fault(status, tab_id, &format!("cartridge-status:{}:{}", status.as_u16(), body.chars().take(120).collect::<String>())),
                Err(error) => fragment_fault(StatusCode::BAD_GATEWAY, tab_id, &format!("cartridge-body:{error}")),
            }
        }
        Err(error) => fragment_fault(StatusCode::BAD_GATEWAY, tab_id, &format!("cartridge-fetch:{error}")),
    }
}

async fn read_static_cartridge_fragment(state: &AppState, manifest: &TabManifest) -> Response {
    let fragment_rel = manifest.fragment_path.trim_start_matches('/');
    let path = state.tab_root.join(&manifest.id).join(fragment_rel);
    match fs::read_to_string(&path).await {
        Ok(body) => Html(body).into_response(),
        Err(error) => fragment_fault(StatusCode::BAD_GATEWAY, &manifest.id, &format!("static-fragment:{}:{error}", path.display())),
    }
}

fn fragment_fault(status: StatusCode, tab_id: &str, fault: &str) -> Response {
    let escaped_fault = fault.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;");
    let body = maud::html! {
        section data-cartridge-fault="true" data-cartridge-fault-kind=(escaped_fault) data-tab-id=(tab_id) {
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
        "stats" => render_json_fragment(&pane.title, "coronatio.stats.fragment.v1", serde_json::to_value(stats_snapshot()).unwrap_or(serde_json::Value::Null)),
        "portals" => render_json_fragment(&pane.title, "coronatio.portals.fragment.v1", serde_json::to_value(read_portals_config().unwrap_or_else(|signal| PortalConfigResponse {
            schema: "coronatio.portals.config.v1".to_string(),
            route: "/api/portals".to_string(),
            success: false,
            source: homeserver_config_candidates().into_iter().map(|p| p.display().to_string()).collect::<Vec<_>>().join(" | "),
            factory_source: None,
            portals: Vec::new(),
            factory_portals: Vec::new(),
            first_missing_signal: signal,
        })).unwrap_or(serde_json::Value::Null)),
        "upload" => render_json_fragment(&pane.title, "coronatio.upload.fragment.v1", serde_json::json!({"schema":"coronatio.upload.history.v1","ok":true,"history":[],"firstMissingSignal":"none"})),
        "admin" => render_json_fragment(&pane.title, "coronatio.admin.fragment.v1", serde_json::to_value(admin_session_readback()).unwrap_or(serde_json::Value::Null)),
        _ => render_json_fragment(&pane.title, "coronatio.native.fragment.v1", serde_json::to_value(pane).unwrap_or(serde_json::Value::Null)),
    }
}

fn render_json_fragment(title: &str, schema: &str, value: serde_json::Value) -> String {
    let pretty = serde_json::to_string_pretty(&value).unwrap_or_else(|_| "{}".to_string());
    maud::html! {
        article .crown-fragment data-fragment-schema=(schema) {
            h2 { (title) }
            pre data-native-readback="json" { (pretty) }
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

