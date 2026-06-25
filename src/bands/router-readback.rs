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
        if let Some(response) = legacy_api_route_response(&method, &normalized) {
            return response;
        }
        return (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "schema": "coronatio.api.error.v1",
                "error": "api route not found",
                "path": normalized,
                "method": method.as_str(),
                "policy": "API clients receive JSON 404 unless the path is a preserved Flask/HomeServer compatibility route"
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

async fn validate_pin_route(Json(body): Json<serde_json::Value>) -> impl IntoResponse {
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
            "schema": "coronatio.legacy.auth.pin.v1",
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

async fn legacy_logout_route() -> impl IntoResponse {
    Json(serde_json::json!({
        "schema": "coronatio.legacy.auth.logout.v1",
        "success": true,
        "ok": true,
        "message": "session cleared by client"
    }))
}

async fn legacy_admin_ping_route() -> impl IntoResponse {
    Json(serde_json::json!({
        "schema": "coronatio.legacy.admin.ping.v1",
        "success": true,
        "ok": true,
        "authenticated": true
    }))
}

async fn legacy_staff_intent_route(method: Method, uri: Uri) -> impl IntoResponse {
    legacy_mutation_response(method.as_str(), uri.path())
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

fn legacy_api_route_response(method: &Method, path: &str) -> Option<Response> {
    if !is_preserved_legacy_api_path(path) {
        return None;
    }
    if method == Method::GET || method == Method::HEAD || method == Method::OPTIONS {
        Some(legacy_readback_response(method.as_str(), path))
    } else {
        Some(legacy_mutation_response(method.as_str(), path))
    }
}

fn legacy_readback_response(method: &str, path: &str) -> Response {
    let family = legacy_route_family(path);
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "schema": "coronatio.legacy.api.compat.v1",
            "ok": true,
            "success": true,
            "method": method,
            "path": path,
            "family": family,
            "status": "readback-compatible",
            "sourceMaterial": "Flask/React HomeServer route quarry",
            "authority": "Coronatio Rust compatibility route",
            "firstMissingSignal": "none"
        })),
    )
        .into_response()
}

fn legacy_mutation_response(method: &str, path: &str) -> Response {
    let caduceus = caduceus_http_json(
        "POST",
        "/api/v1/staff/intent",
        serde_json::json!({
            "method": method,
            "route": path,
            "classification": legacy_route_family(path),
        }),
    );
    (
        if caduceus.ok { StatusCode::ACCEPTED } else { StatusCode::SERVICE_UNAVAILABLE },
        Json(serde_json::json!({
            "schema": "coronatio.legacy.api.mutation.v1",
            "ok": caduceus.ok,
            "accepted": caduceus.ok,
            "method": method,
            "path": path,
            "family": legacy_route_family(path),
            "authority": "Caduceus staff intent membrane",
            "caduceus": caduceus,
            "firstMissingSignal": if caduceus.ok { "none".to_string() } else { caduceus.first_missing_signal }
        })),
    )
        .into_response()
}

fn legacy_route_family(path: &str) -> &'static str {
    if path.contains("/diskman") || path.contains("/vault") || path.contains("/crypto") { "admin-storage" }
    else if path.contains("/updates") || path.contains("/system/update") { "harmonia-update" }
    else if path.contains("/status/tailscale") || path.contains("/status/vpn") || path.contains("/network/") { "network-control" }
    else if path.contains("/upload") || path.contains("/files/") { "file-ingress" }
    else if path.contains("/portals") || path.contains("/service/control") { "portal-service" }
    else if path.contains("/premium") { "premium-installer" }
    else if path.contains("/tabs") || path.contains("setstarredtab") { "tab-registry" }
    else { "crown-legacy" }
}

fn is_preserved_legacy_api_path(path: &str) -> bool {
    const EXACT: &[&str] = &[
        "/api/pre-unlock", "/api/admin/pin", "/api/logout", "/api/vault/status", "/api/vault/unlock",
        "/api/themes", "/api/system/log", "/api/system/update", "/api/admin/system/update-password",
        "/api/admin/logs/homeserver", "/api/admin/logs/homeserver/clear", "/api/admin/ping",
        "/api/admin/download-root-crt", "/api/admin/refresh-root-crt", "/api/crypto/getKey",
        "/api/admin/crypto/test", "/api/status/services", "/api/status", "/api/uptime", "/api/version",
        "/api/files/browse", "/api/files/browse-hierarchical", "/api/files/upload", "/api/upload/force-permissions",
        "/api/upload/history", "/api/upload/history/clear", "/api/upload/default-directory",
        "/api/upload/blacklist/list", "/api/upload/blacklist/update", "/api/upload/pin-required-status",
        "/api/portals", "/api/portals/factory", "/api/service/control", "/api/status/internet/speedtest",
        "/status/power/usage", "/api/kea-leases", "/api/network/notes", "/api/status/tailscale",
        "/api/status/tailscale/connect", "/api/status/tailscale/authkey", "/api/status/tailscale/disconnect",
        "/api/status/tailscale/enable", "/api/status/tailscale/disable", "/api/status/tailscale/config",
        "/api/status/tailscale/update-tailnet", "/api/status/vpn/pia", "/api/status/vpn/transmission",
        "/api/status/vpn/updatekey/pia", "/api/status/vpn/updatekey/transmission", "/api/status/vpn/pia/exists",
        "/api/status/vpn/transmission/exists", "/api/status/vpn/enable", "/api/status/vpn/disable",
        "/api/status/vpn/check-enabled", "/api/admin/updates/check", "/api/admin/updates/apply",
        "/api/admin/updates/force", "/api/admin/updates/modules", "/api/admin/updates/interactives",
        "/api/admin/updates/logs", "/api/admin/updates/logfile", "/api/admin/updates/system-info",
        "/api/admin/updates/schedule", "/api/admin/ssh/status", "/api/admin/ssh/toggle",
        "/api/admin/services/hard-reset", "/api/admin/system/restart", "/api/admin/system/shutdown",
        "/api/admin/ssh/service", "/api/admin/ssh/service/status", "/api/admin/samba/service/status",
        "/api/admin/samba/service", "/api/admin/hard-drive-test/results", "/api/admin/hard-drive-test/progress",
        "/api/admin/hard-drive-test/start", "/api/admin/hard-drive-test/devices", "/api/admin/diskman/create-key",
        "/api/admin/diskman/update-key", "/api/admin/diskman/key-status", "/api/admin/diskman/vault-device",
        "/api/admin/diskman/nas-compatible", "/api/admin/diskman/format", "/api/admin/diskman/unlock",
        "/api/admin/diskman/unlock-with-password", "/api/admin/diskman/encrypt", "/api/admin/diskman/mount",
        "/api/admin/diskman/unmount", "/api/admin/diskman/apply-permissions", "/api/admin/diskman/check-services",
        "/api/admin/diskman/manage-services", "/api/admin/diskman/sync", "/api/admin/diskman/sync-schedule",
        "/api/admin/diskman/sync-schedule-update", "/api/admin/diskman/assign-nas", "/api/admin/diskman/unassign-nas",
        "/api/admin/diskman/import-to-nas", "/api/admin/premium/validate-and-clone",
        "/api/admin/premium/status", "/api/admin/premium/install-all", "/api/admin/premium/uninstall-all",
        "/api/admin/premium/logs", "/api/admin/premium/auto-update-status", "/api/setstarredtab",
        "/api/tabs/visibility", "/api/tabs/elements",
    ];
    const PREFIX: &[&str] = &[
        "/api/portals/", "/api/portals/images/", "/api/admin/updates/modules/",
        "/api/admin/updates/interactives/", "/api/admin/premium/install/", "/api/admin/premium/uninstall/",
        "/api/admin/premium/reinstall/", "/api/admin/premium/delete/", "/api/admin/premium/auto-update/",
        "/api/files/download",
    ];
    EXACT.contains(&path) || PREFIX.iter().any(|prefix| path.starts_with(prefix))
}
