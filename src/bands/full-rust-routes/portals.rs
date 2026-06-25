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
    let portals_tab = value.get("tabs").and_then(|tabs| tabs.get("portals"));
    let tab_visible = portals_tab
        .and_then(|portals| portals.get("visibility"))
        .and_then(|visibility| visibility.get("tab"))
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(true);
    let element_visibility = portals_tab
        .and_then(|portals| portals.get("visibility"))
        .and_then(|visibility| visibility.get("elements"))
        .and_then(serde_json::Value::as_object);
    portals_tab
        .and_then(|portals| portals.get("data"))
        .and_then(|data| data.get("portals"))
        .and_then(serde_json::Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(|item| serde_json::from_value::<PortalEntry>(item.clone()).ok())
                .map(|mut portal| {
                    let element_visible = element_visibility
                        .and_then(|elements| elements.get(&portal.name))
                        .and_then(serde_json::Value::as_bool)
                        .unwrap_or(true);
                    portal.visible = tab_visible && element_visible;
                    portal
                })
                .filter(|portal| !portal.name.trim().is_empty() && !portal.local_url.trim().is_empty())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}


async fn portal_service_control_route(Json(payload): Json<serde_json::Value>) -> impl IntoResponse {
    let service = match payload.get("service").and_then(serde_json::Value::as_str).map(str::trim) {
        Some(service) if is_safe_service_name(service) => service.to_string(),
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "schema": "coronatio.portals.service_control.v1",
                    "success": false,
                    "ok": false,
                    "accepted": false,
                    "error": "Invalid service name",
                    "firstMissingSignal": "invalid-service-name"
                })),
            )
                .into_response();
        }
    };
    let action = match payload.get("action").and_then(serde_json::Value::as_str).map(str::trim) {
        Some(action) if portal_service_action_allowed(action) => action.to_string(),
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "schema": "coronatio.portals.service_control.v1",
                    "success": false,
                    "ok": false,
                    "accepted": false,
                    "service": service,
                    "error": "Invalid action",
                    "firstMissingSignal": "invalid-service-action"
                })),
            )
                .into_response();
        }
    };
    let systemd_service = if service.ends_with(".service") { service.clone() } else { format!("{service}.service") };
    let caduceus = caduceus_http_json(
        "POST",
        "/api/v1/staff/intent",
        serde_json::json!({
            "method": "POST",
            "route": "/api/service/control",
            "classification": "portal-service",
            "metadata": {
                "service": service,
                "action": action,
                "systemdService": systemd_service,
                "source": "coronatio-portals-admin-mode",
                "originalQuarry": "Flask portals service_control execute_systemctl_command"
            }
        }),
    );
    let accepted = caduceus.ok;
    let status = if accepted { StatusCode::ACCEPTED } else { StatusCode::SERVICE_UNAVAILABLE };
    (
        status,
        Json(serde_json::json!({
            "schema": "coronatio.portals.service_control.v1",
            "success": accepted,
            "ok": accepted,
            "accepted": accepted,
            "message": if accepted { format!("Service {action} accepted for {service}") } else { format!("Service {action} for {service} could not reach Caduceus staff") },
            "route": "/api/service/control",
            "classification": "portal-service",
            "service": service,
            "action": action,
            "systemdService": systemd_service,
            "active": serde_json::Value::Null,
            "output": "Caduceus staff intent membrane accepted the portal service command".to_string(),
            "authority": "Caduceus staff intent membrane",
            "caduceus": caduceus,
            "firstMissingSignal": if accepted { "none".to_string() } else { caduceus.first_missing_signal }
        })),
    )
        .into_response()
}

fn portal_service_action_allowed(action: &str) -> bool {
    matches!(action, "start" | "stop" | "restart" | "enable" | "disable" | "status")
}

fn is_safe_service_name(service: &str) -> bool {
    !service.is_empty()
        && !service.contains("..")
        && service
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b'@'))
}

