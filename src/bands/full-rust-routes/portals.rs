async fn portals_config_route() -> impl IntoResponse {
    match read_portals_config() {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err(signal) => (
            StatusCode::OK,
            Json(PortalConfigResponse {
                schema: "coronatio.portals.config.v1".to_string(),
                route: "/api/portals".to_string(),
                success: false,
                source: homeserver_config_candidates().into_iter().map(|p| p.display().to_string()).collect::<Vec<_>>().join(" | "),
                factory_source: None,
                portals: Vec::new(),
                factory_portals: Vec::new(),
                first_missing_signal: signal,
            }),
        ).into_response(),
    }
}

async fn portals_factory_route() -> impl IntoResponse {
    let (source, factory_portals, first_missing_signal) = read_factory_portal_names();
    (
        StatusCode::OK,
        Json(PortalFactoryResponse {
            schema: "coronatio.portals.factory.v1".to_string(),
            success: first_missing_signal == "none",
            source,
            factory_portals,
            first_missing_signal,
        }),
    )
}

fn read_portals_config() -> Result<PortalConfigResponse, String> {
    let (source, value) = read_first_json(&homeserver_config_candidates())?;
    let portals = extract_portals(&value);
    let (factory_source, factory_portals, factory_signal) = read_factory_portal_names();
    Ok(PortalConfigResponse {
        schema: "coronatio.portals.config.v1".to_string(),
        route: "/api/portals".to_string(),
        success: true,
        source: source.display().to_string(),
        factory_source,
        portals,
        factory_portals,
        first_missing_signal: if factory_signal == "none" { "none".to_string() } else { factory_signal },
    })
}

fn homeserver_config_candidates() -> Vec<PathBuf> {
    let mut paths = Vec::new();
    if let Ok(path) = env::var("CORONATIO_HOMESERVER_JSON") {
        paths.push(PathBuf::from(path));
    }
    paths.push(PathBuf::from("/etc/homeserver.json"));
    paths.push(PathBuf::from("/var/www/homeserver/src/config/homeserver.json"));
    paths.push(PathBuf::from("/fulcrum/attachments/homeserver/initialization/flask/inject/src/config/homeserver.json"));
    paths.push(PathBuf::from("/fulcrum/attachments/homeserver/initialization/flask/src/config/homeserver.json"));
    paths
}

fn homeserver_factory_candidates() -> Vec<PathBuf> {
    let mut paths = Vec::new();
    if let Ok(path) = env::var("CORONATIO_HOMESERVER_FACTORY_JSON") {
        paths.push(PathBuf::from(path));
    }
    paths.push(PathBuf::from("/etc/homeserver.factory"));
    paths.push(PathBuf::from("/var/www/homeserver/src/config/homeserver.factory"));
    paths
}

fn read_first_json(candidates: &[PathBuf]) -> Result<(PathBuf, serde_json::Value), String> {
    let mut missing = Vec::new();
    for path in candidates {
        match std::fs::read_to_string(path) {
            Ok(text) => match serde_json::from_str::<serde_json::Value>(&text) {
                Ok(value) => return Ok((path.clone(), value)),
                Err(error) => return Err(format!("homeserver-config-invalid-json:{}:{error}", path.display())),
            },
            Err(_) => missing.push(path.display().to_string()),
        }
    }
    Err(format!("homeserver-config-missing:{}", missing.join("|")))
}

fn read_factory_portal_names() -> (Option<String>, Vec<String>, String) {
    match read_first_json(&homeserver_factory_candidates()) {
        Ok((source, value)) => {
            let names = extract_portals(&value).into_iter().map(|portal| portal.name).collect::<Vec<_>>();
            (Some(source.display().to_string()), names, "none".to_string())
        }
        Err(signal) if signal.starts_with("homeserver-config-missing:") => (None, Vec::new(), "none".to_string()),
        Err(signal) => (None, Vec::new(), signal),
    }
}

async fn portal_image_route(Path(filename): Path<String>) -> impl IntoResponse {
    if filename.contains('/') || filename.contains("..") || filename.is_empty() {
        return StatusCode::BAD_REQUEST.into_response();
    }
    for root in portal_image_roots() {
        let path = root.join(&filename);
        if path.is_file() {
            match std::fs::read(&path) {
                Ok(bytes) => {
                    let content_type = if filename.ends_with(".png") { "image/png" } else { "application/octet-stream" };
                    return (StatusCode::OK, [(header::CONTENT_TYPE, content_type)], bytes).into_response();
                }
                Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            }
        }
    }
    StatusCode::NOT_FOUND.into_response()
}

fn portal_image_roots() -> Vec<PathBuf> {
    let mut roots = Vec::new();
    if let Ok(path) = env::var("CORONATIO_PORTAL_IMAGE_ROOT") {
        roots.push(PathBuf::from(path));
    }
    roots.push(PathBuf::from("/var/www/homeserver/src/tablets/portals/images"));
    roots.push(PathBuf::from("/fulcrum/attachments/homeserver/initialization/flask/inject/src/tablets/portals/images"));
    roots
}

fn extract_portals(value: &serde_json::Value) -> Vec<PortalEntry> {
    value
        .get("tabs")
        .and_then(|tabs| tabs.get("portals"))
        .and_then(|portals| portals.get("data"))
        .and_then(|data| data.get("portals"))
        .and_then(serde_json::Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(|item| serde_json::from_value::<PortalEntry>(item.clone()).ok())
                .filter(|portal| !portal.name.trim().is_empty() && !portal.local_url.trim().is_empty())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}

