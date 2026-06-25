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

