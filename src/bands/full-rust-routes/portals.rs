async fn portals_config_route() -> impl IntoResponse {
    match read_portals_config() {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err(signal) => (
            StatusCode::OK,
            Json(PortalConfigResponse {
                schema: "coronatio.portals.config.v1".to_string(),
                route: "/api/portals".to_string(),
                success: false,
                source: homeserver_json_path().display().to_string(),
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
    let (source, value) = read_json(&homeserver_json_path())?;
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

fn homeserver_factory_path() -> PathBuf {
    PathBuf::from(APPLIANCE_FACTORY_CONFIG_JSON)
}

fn read_json(path: &PathBuf) -> Result<(PathBuf, serde_json::Value), String> {
    match std::fs::read_to_string(path) {
        Ok(text) => serde_json::from_str::<serde_json::Value>(&text)
            .map(|value| (path.clone(), value))
            .map_err(|error| format!("homeserver-config-invalid-json:{}:{error}", path.display())),
        Err(_) => Err(format!("homeserver-config-missing:{}", path.display())),
    }
}

fn read_factory_portal_names() -> (Option<String>, Vec<String>, String) {
    match read_json(&homeserver_factory_path()) {
        Ok((source, value)) => {
            let names = extract_portals(&value).into_iter().map(|portal| portal.name).collect::<Vec<_>>();
            (Some(source.display().to_string()), names, "none".to_string())
        }
        Err(signal) if signal.starts_with("homeserver-config-missing:") => (None, Vec::new(), "none".to_string()),
        Err(signal) => (None, Vec::new(), signal),
    }
}

const PORTAL_ICON_MAX_EDGE: u32 = 240;

async fn portal_image_route(
    Path(filename): Path<String>,
    headers: axum::http::HeaderMap,
) -> Response {
    if filename.contains('/') || filename.contains("..") || filename.is_empty() {
        return StatusCode::BAD_REQUEST.into_response();
    }

    // File reads and PNG transforms must not occupy the async worker.
    tokio::task::spawn_blocking(move || serve_portal_image(&filename, &headers))
        .await
        .unwrap_or_else(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())
}

fn serve_portal_image(filename: &str, headers: &axum::http::HeaderMap) -> Response {
    let served_filename = match read_portal_image(filename) {
        Ok(Some(image)) => return portal_image_response(filename, image, headers),
        Ok(None) if filename == "default.png" => return StatusCode::NOT_FOUND.into_response(),
        Ok(None) => "default.png",
        Err(()) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    match read_portal_image(served_filename) {
        Ok(Some(image)) => portal_image_response(served_filename, image, headers),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(()) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

struct PortalImage {
    bytes: Vec<u8>,
    modified: Option<SystemTime>,
}

fn read_portal_image(filename: &str) -> Result<Option<PortalImage>, ()> {
    for root in portal_image_roots() {
        let path = root.join(filename);
        if path.is_file() {
            let mut file = std::fs::File::open(path).map_err(|_| ())?;
            let mut bytes = Vec::new();
            file.read_to_end(&mut bytes).map_err(|_| ())?;
            let modified = file.metadata().and_then(|metadata| metadata.modified()).ok();
            return Ok(Some(PortalImage { bytes, modified }));
        }
    }
    Ok(None)
}

fn portal_image_response(
    filename: &str,
    image: PortalImage,
    headers: &axum::http::HeaderMap,
) -> Response {
    use sha2::{Digest as _, Sha256};

    let is_png = filename.ends_with(".png");
    let (variant, content_type) = if is_png {
        ("png-240-v1", "image/png")
    } else {
        ("raw-v1", "application/octet-stream")
    };
    // A weak validator ties the result to the source snapshot and transform version
    // without having to decode it on each successful revalidation.
    let modified_ns = image.modified.map(|time| match time.duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_nanos() as i128,
        Err(error) => -(error.duration().as_nanos() as i128),
    });
    let mut digest = Sha256::new();
    digest.update([u8::from(modified_ns.is_some())]);
    digest.update(modified_ns.unwrap_or_default().to_be_bytes());
    digest.update(&image.bytes);
    let quoted_tag = format!("\"portal-{variant}-{:x}\"", digest.finalize());
    let cache_headers = [
        (header::ETAG, format!("W/{quoted_tag}")),
        (header::CACHE_CONTROL, "no-cache".to_owned()),
    ];
    if portal_image_etag_matches(headers, quoted_tag.as_bytes()) {
        return (StatusCode::NOT_MODIFIED, cache_headers).into_response();
    }

    let bytes = if is_png {
        scale_portal_png(&image.bytes).ok().flatten().unwrap_or(image.bytes)
    } else {
        image.bytes
    };
    (StatusCode::OK, cache_headers, [(header::CONTENT_TYPE, content_type)], bytes).into_response()
}

fn portal_image_etag_matches(headers: &axum::http::HeaderMap, quoted_tag: &[u8]) -> bool {
    headers.get_all(header::IF_NONE_MATCH).iter().any(|value| {
        let value = value.as_bytes().trim_ascii();
        value == b"*" || value.split(|&byte| byte == b',').any(|candidate| {
            let candidate = candidate.trim_ascii();
            candidate.strip_prefix(b"W/").unwrap_or(candidate) == quoted_tag
        })
    })
}

fn scale_portal_png(bytes: &[u8]) -> image::ImageResult<Option<Vec<u8>>> {
    use image::{codecs::png::PngDecoder, DynamicImage, ImageDecoder, ImageFormat, Limits};
    use std::io::Cursor;

    let mut limits = Limits::default();
    let mut decoder = PngDecoder::with_limits(Cursor::new(bytes), limits.clone())?;
    let (width, height) = decoder.dimensions();
    if width.max(height) <= PORTAL_ICON_MAX_EDGE {
        return Ok(None);
    }

    limits.reserve(decoder.total_bytes())?;
    decoder.set_limits(limits)?;
    let scaled = DynamicImage::from_decoder(decoder)?
        .thumbnail(PORTAL_ICON_MAX_EDGE, PORTAL_ICON_MAX_EDGE);
    let mut encoded = Cursor::new(Vec::new());
    scaled.write_to(&mut encoded, ImageFormat::Png)?;
    Ok(Some(encoded.into_inner()))
}

fn portal_image_roots() -> Vec<PathBuf> {
    if let Ok(path) = env::var("CORONATIO_PORTAL_IMAGE_ROOT") {
        return vec![PathBuf::from(path)];
    }
    vec![
        PathBuf::from("/var/www/homeserver/src/tablets/portals/images"),
    ]
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


async fn portal_service_control_route(headers: axum::http::HeaderMap, Json(payload): Json<serde_json::Value>) -> Response {
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
    if let Some(refusal) = mutation_context_refusal(&headers) {
        return services_mutation_context_refusal_response("POST", "/api/service/control", refusal);
    }
    let portals = match read_portals_config() {
        Ok(config) => config.portals,
        Err(signal) => return portal_service_allowlist_refusal_response(&service, signal),
    };
    if !portal_service_is_allowlisted(&service, &portals) {
        return portal_service_allowlist_refusal_response(
            &service,
            "portal-service-not-allowlisted".to_string(),
        );
    }
    let path = format!("/api/v1/appliance/service/{service}/{action}");
    // Slow service mutations get a bounded long socket timeout; status and fallback actions stay short.
    let timeout = match action.as_str() {
        "start" | "stop" | "restart" | "enable" | "disable" => std::time::Duration::from_secs(180),
        "status" => std::time::Duration::from_secs(4),
        _ => std::time::Duration::from_secs(4),
    };
    let refusal_path = path.clone();
    let caduceus = tokio::task::spawn_blocking(move || {
        caduceus_actuate_json_with_timeout(
            &mutation_authority(),
            &headers,
            &path,
            serde_json::json!({}),
            timeout,
        )
    })
    .await
    .unwrap_or_else(|_| mutation_task_failure_readback(&refusal_path));
    portal_service_mutation_response(caduceus)
}

fn portal_service_mutation_response(caduceus: CaduceusHttpReadback) -> Response {
    let body = &caduceus.body;
    let portal_service = body.get("portalService").filter(|projection| !projection.is_null());
    let semantic_success = if let Some(projection) = portal_service {
        // A successful stop does not imply a successful portal service action.
        projection
            .get("ok")
            .and_then(serde_json::Value::as_bool)
            .or_else(|| projection.get("success").and_then(serde_json::Value::as_bool))
            .unwrap_or(false)
    } else {
        body.get("success")
            .and_then(serde_json::Value::as_bool)
            .or_else(|| body.get("ok").and_then(serde_json::Value::as_bool))
            .unwrap_or(caduceus.ok)
    };
    let success = caduceus.ok && semantic_success;
    let first_missing_signal = body
        .get("firstMissingSignal")
        .or_else(|| body.get("first_missing_signal"))
        .and_then(serde_json::Value::as_str)
        .unwrap_or(caduceus.first_missing_signal.as_str());
    let read_text = |keys: &[&str]| {
        portal_service
            .and_then(|projection| {
                keys.iter()
                    .find_map(|key| projection.get(*key).and_then(serde_json::Value::as_str))
            })
            .or_else(|| keys.iter().find_map(|key| body.get(*key)).and_then(serde_json::Value::as_str))
    };
    let message = read_text(&["message", "error", "output", "stdout"])
        .unwrap_or(if success { "Service action completed" } else { first_missing_signal });
    let output = read_text(&["output", "stdout", "message", "error"])
        .unwrap_or(first_missing_signal);
    let active = body.get("active").and_then(serde_json::Value::as_bool).unwrap_or(false);
    let enabled = body.get("enabled").and_then(serde_json::Value::as_bool);
    let status = if success {
        StatusCode::OK
    } else if (400..=599).contains(&caduceus.status) {
        StatusCode::from_u16(caduceus.status).unwrap_or(StatusCode::SERVICE_UNAVAILABLE)
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };
    (
        status,
        Json(serde_json::json!({
            "schema": "coronatio.portals.service_control.v1",
            "success": success,
            "ok": success,
            "message": message,
            "output": output,
            "active": active,
            "enabled": enabled,
            "firstMissingSignal": first_missing_signal
        })),
    )
        .into_response()
}

fn portal_service_allowlist_refusal_response(service: &str, signal: String) -> Response {
    (
        StatusCode::FORBIDDEN,
        Json(serde_json::json!({
            "schema": "coronatio.portals.service_control.refusal.v1",
            "success": false,
            "ok": false,
            "accepted": false,
            "service": service,
            "error": "portal-service-not-allowlisted",
            "firstMissingSignal": signal
        })),
    )
        .into_response()
}

fn portal_service_is_allowlisted(service: &str, portals: &[PortalEntry]) -> bool {
    let requested = service.strip_suffix(".service").unwrap_or(service);
    portals.iter().flat_map(|portal| &portal.services).any(|allowed| {
        let allowed = allowed.trim();
        allowed == service || allowed.strip_suffix(".service").unwrap_or(allowed) == requested
    })
}

fn services_mutation_context_refusal_response(method: &str, path: &str, refusal: MutationRefusal) -> Response {
    let signal = safe_access_code(&refusal.code);
    (
        StatusCode::from_u16(refusal.status).unwrap_or(StatusCode::SERVICE_UNAVAILABLE),
        Json(serde_json::json!({
            "schema": "coronatio.services.mutation.refusal.v1",
            "success": false,
            "ok": false,
            "accepted": false,
            "method": method,
            "path": path,
            "family": "portal-service",
            "error": signal,
            "firstMissingSignal": signal
        })),
    ).into_response()
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

