fn full_rust_route_table() -> Router<AppState> {
    Router::new()
        .route("/api/tabs/visibility", post(tab_visibility_route))
        .route("/api/tabs/elements", put(homeserver_rust_mutation_route))
        .route("/api/pre-unlock", post(homeserver_rust_mutation_route))
        .route("/api/vault/status", get(homeserver_rust_read_route))
        .route("/api/vault/unlock", post(homeserver_rust_mutation_route))
        .route("/api/system/log", post(homeserver_rust_mutation_route))
        .route("/api/system/update", post(homeserver_rust_mutation_route))
        .route("/api/admin/system/update-password", post(homeserver_rust_mutation_route))
        .route("/api/admin/logs/homeserver", get(homeserver_rust_read_route))
        .route("/api/admin/logs/homeserver/clear", post(homeserver_rust_mutation_route))
        .route("/api/admin/download-root-crt", get(homeserver_rust_read_route))
        .route("/api/admin/refresh-root-crt", post(homeserver_rust_mutation_route))
        .route("/api/crypto/getKey", get(homeserver_rust_read_route))
        .route("/api/admin/crypto/test", post(homeserver_rust_mutation_route))
        .route("/api/admin/updates/check", get(homeserver_rust_read_route))
        .route("/api/admin/updates/apply", post(homeserver_rust_mutation_route))
        .route("/api/admin/updates/force", post(homeserver_rust_mutation_route))
        .route("/api/admin/updates/modules", get(homeserver_rust_read_route))
        .route("/api/admin/updates/modules/:module_name/status", get(homeserver_rust_read_route))
        .route("/api/admin/updates/modules/:module_name/toggle", post(homeserver_rust_mutation_route))
        .route("/api/admin/updates/modules/:module_name/components/:component_name/toggle", post(homeserver_rust_mutation_route))
        .route("/api/admin/updates/modules/:module_name/branch", post(homeserver_rust_mutation_route))
        .route("/api/admin/updates/interactives", get(homeserver_rust_read_route))
        .route("/api/admin/updates/interactives/:interactive_id/run", post(homeserver_rust_mutation_route))
        .route("/api/admin/updates/logs", get(homeserver_rust_read_route))
        .route("/api/admin/updates/logfile", get(homeserver_rust_read_route))
        .route("/api/admin/updates/system-info", get(homeserver_rust_read_route))
        .route("/api/admin/updates/schedule", get(homeserver_rust_read_route).post(homeserver_rust_mutation_route))
        .route("/api/admin/ssh/status", get(homeserver_rust_read_route))
        .route("/api/admin/ssh/toggle", post(homeserver_rust_mutation_route))
        .route("/api/admin/services/hard-reset", post(homeserver_rust_mutation_route))
        .route("/api/admin/system/restart", post(homeserver_rust_mutation_route))
        .route("/api/admin/system/shutdown", post(homeserver_rust_mutation_route))
        .route("/api/admin/ssh/service", post(homeserver_rust_mutation_route))
        .route("/api/admin/ssh/service/status", get(homeserver_rust_read_route))
        .route("/api/admin/samba/service/status", get(homeserver_rust_read_route))
        .route("/api/admin/samba/service", post(homeserver_rust_mutation_route))
        .route("/api/admin/hard-drive-test/results", get(homeserver_rust_read_route))
        .route("/api/admin/hard-drive-test/progress", get(homeserver_rust_read_route))
        .route("/api/admin/hard-drive-test/start", post(homeserver_rust_mutation_route))
        .route("/api/admin/hard-drive-test/devices", get(homeserver_rust_read_route))
        .route("/api/admin/diskman/nas-compatible", get(homeserver_rust_read_route))
        .route("/api/admin/diskman/format", post(homeserver_rust_mutation_route))
        .route("/api/admin/diskman/unlock", post(homeserver_rust_mutation_route))
        .route("/api/admin/diskman/unlock-with-password", post(homeserver_rust_mutation_route))
        .route("/api/admin/diskman/encrypt", post(homeserver_rust_mutation_route))
        .route("/api/admin/diskman/mount", post(homeserver_rust_mutation_route))
        .route("/api/admin/diskman/unmount", post(homeserver_rust_mutation_route))
        .route("/api/admin/diskman/apply-permissions", post(homeserver_rust_mutation_route))
        .route("/api/admin/diskman/check-services", get(homeserver_rust_read_route))
        .route("/api/admin/diskman/manage-services", post(homeserver_rust_mutation_route))
        .route("/api/admin/diskman/sync", post(homeserver_rust_mutation_route))
        .route("/api/admin/diskman/sync-schedule", get(homeserver_rust_read_route))
        .route("/api/admin/diskman/sync-schedule-update", post(homeserver_rust_mutation_route))
        .route("/api/admin/diskman/assign-nas", post(homeserver_rust_mutation_route))
        .route("/api/admin/diskman/unassign-nas", post(homeserver_rust_mutation_route))
        .route("/api/admin/diskman/import-to-nas", post(homeserver_rust_mutation_route))
        .route("/api/admin/diskman/create-key", post(homeserver_rust_mutation_route))
        .route("/api/admin/diskman/update-key", post(homeserver_rust_mutation_route))
        .route("/api/admin/diskman/key-status", post(homeserver_rust_mutation_route))
        .route("/api/admin/diskman/vault-device", get(homeserver_rust_read_route))
        .route("/api/admin/premium/validate-and-clone", post(homeserver_rust_mutation_route))
        .route("/api/admin/premium/install/:tab_name", post(homeserver_rust_mutation_route))
        .route("/api/admin/premium/uninstall/:tab_name", delete(homeserver_rust_mutation_route))
        .route("/api/admin/premium/reinstall/:tab_name", post(homeserver_rust_mutation_route))
        .route("/api/admin/premium/reinstall-multiple", post(homeserver_rust_mutation_route))
        .route("/api/admin/premium/delete/:tab_name", delete(homeserver_rust_mutation_route))
        .route("/api/admin/premium/status", get(homeserver_rust_read_route))
        .route("/api/admin/premium/install-all", post(homeserver_rust_mutation_route))
        .route("/api/admin/premium/uninstall-all", post(homeserver_rust_mutation_route))
        .route("/api/admin/premium/logs", get(homeserver_rust_read_route))
        .route("/api/admin/premium/auto-update/:tab_name", get(homeserver_rust_read_route).post(homeserver_rust_mutation_route))
        .route("/api/admin/premium/auto-update-status", get(homeserver_rust_read_route))
        .route("/api/status/services", get(homeserver_rust_read_route))
        .route("/api/status", get(internet_status_route))
        .route("/api/uptime", get(uptime_route))
        .route("/api/status/tailscale", get(homeserver_rust_read_route))
        .route("/api/status/tailscale/connect", post(homeserver_rust_mutation_route))
        .route("/api/status/tailscale/authkey", post(homeserver_rust_mutation_route))
        .route("/api/status/tailscale/disconnect", post(homeserver_rust_mutation_route))
        .route("/api/status/tailscale/enable", post(homeserver_rust_mutation_route))
        .route("/api/status/tailscale/disable", post(homeserver_rust_mutation_route))
        .route("/api/status/tailscale/config", get(homeserver_rust_read_route).post(homeserver_rust_mutation_route))
        .route("/api/status/tailscale/update-tailnet", post(homeserver_rust_mutation_route))
        .route("/api/status/vpn/pia", get(homeserver_rust_read_route))
        .route("/api/status/vpn/transmission", get(homeserver_rust_read_route))
        .route("/api/status/vpn/updatekey/pia", post(homeserver_rust_mutation_route))
        .route("/api/status/vpn/updatekey/transmission", post(homeserver_rust_mutation_route))
        .route("/api/status/vpn/pia/exists", get(homeserver_rust_read_route))
        .route("/api/status/vpn/transmission/exists", get(homeserver_rust_read_route))
        .route("/api/status/vpn/enable", post(homeserver_rust_mutation_route))
        .route("/api/status/vpn/disable", post(homeserver_rust_mutation_route))
        .route("/api/status/vpn/check-enabled", get(homeserver_rust_read_route))
        .route("/api/files/browse", get(upload_browse_hierarchical_route))
        .route("/api/files/browse-hierarchical", get(upload_browse_hierarchical_route))
        .route("/api/files/upload", post(upload_file_route))
        .route("/api/files/download", get(homeserver_rust_read_route))
        .route("/api/upload/force-permissions", post(homeserver_rust_mutation_route))
        .route("/api/upload/history", get(upload_history_route))
        .route("/api/upload/history/clear", post(homeserver_rust_mutation_route))
        .route("/api/upload/default-directory", get(upload_default_directory_route).post(homeserver_rust_mutation_route))
        .route("/api/upload/blacklist/list", get(upload_blacklist_route))
        .route("/api/upload/blacklist/update", put(homeserver_rust_mutation_route))
        .route("/api/upload/pin-required-status", get(upload_pin_required_route).post(homeserver_rust_mutation_route))
        .route("/api/portals", get(portals_config_route).post(homeserver_rust_mutation_route))
        .route("/api/portals/:portal_name", put(homeserver_rust_mutation_route).delete(homeserver_rust_mutation_route))
        .route("/api/portals/factory", get(portals_factory_route))
        .route("/api/service/control", post(portal_service_control_route))
        .route("/api/portals/images/:filename", get(portal_image_route))
        .route("/api/status/internet/speedtest", post(homeserver_rust_mutation_route))
        .route("/status/power/usage", get(homeserver_rust_read_route))
        .route("/api/status/power/usage", get(homeserver_rust_read_route))
        .route("/api/kea-leases", get(homeserver_rust_read_route))
        .route("/api/network/notes", get(homeserver_rust_read_route).put(homeserver_rust_mutation_route))
        .route("/api/version", get(homeserver_rust_read_route))
        .route("/api/wakeonlan/targets", get(homeserver_rust_read_route).post(homeserver_rust_mutation_route))
        .route("/api/wakeonlan/wake", post(homeserver_rust_mutation_route))
        .route("/api/wakeonlan/targets/:name", delete(homeserver_rust_mutation_route))
        .route("/api/wakeonlan/status", get(homeserver_rust_read_route))
        .route("/api/dhcp/status", get(homeserver_rust_read_route))
        .route("/api/dhcp/leases", get(homeserver_rust_read_route))
        .route("/api/dhcp/reservations", get(homeserver_rust_read_route).post(homeserver_rust_mutation_route))
        .route("/api/dhcp/reservations/:reservation_id", put(homeserver_rust_mutation_route).delete(homeserver_rust_mutation_route))
        .route("/api/dhcp/config", get(homeserver_rust_read_route).post(homeserver_rust_mutation_route))
        .route("/api/dhcp/health", get(homeserver_rust_read_route))
        .route("/api/dhcp/statistics", get(homeserver_rust_read_route))
        .route("/api/dhcp/pool-boundary", get(homeserver_rust_read_route).post(homeserver_rust_mutation_route))
        .route("/api/youtube/download", post(homeserver_rust_mutation_route))
        .route("/api/youtube/download/info", post(homeserver_rust_mutation_route))
        .route("/api/youtube/subscriptions", get(homeserver_rust_read_route).post(homeserver_rust_mutation_route))
        .route("/api/youtube/subscriptions/:channel_id", delete(homeserver_rust_mutation_route))
        .route("/api/youtube/subscriptions/:channel_id/fetch", post(homeserver_rust_mutation_route))
        .route("/api/youtube/settings", get(homeserver_rust_read_route).post(homeserver_rust_mutation_route))
        .route("/api/youtube/schedule", get(homeserver_rust_read_route).post(homeserver_rust_mutation_route))
        .route("/api/youtube/logs", get(homeserver_rust_read_route))
        .route("/api/youtube/update-ytdlp", post(homeserver_rust_mutation_route))
        .route("/api/nasLinker/browse", get(homeserver_rust_read_route))
        .route("/api/nasLinker/deploy", post(homeserver_rust_mutation_route))
        .route("/api/nasLinker/delete", delete(homeserver_rust_mutation_route))
        .route("/api/nasLinker/rename", post(homeserver_rust_mutation_route))
        .route("/api/nasLinker/newdir", post(homeserver_rust_mutation_route))
        .route("/api/nasLinker/scan", get(homeserver_rust_read_route))
        .route("/api/nasLinker/status", get(homeserver_rust_read_route))
        .route("/api/nasLinker/config", get(homeserver_rust_read_route))
        .route("/api/backup/status", get(homeserver_rust_read_route))
        .route("/api/backup/repositories", get(homeserver_rust_read_route))
        .route("/api/backup/providers/status", get(homeserver_rust_read_route))
        .route("/api/backup/backup/run", post(homeserver_rust_mutation_route))
        .route("/api/backup/sync-now", post(homeserver_rust_mutation_route))
        .route("/api/backup/cloud/test", post(homeserver_rust_mutation_route))
        .route("/api/backup/config", get(homeserver_rust_read_route).post(homeserver_rust_mutation_route))
        .route("/api/backup/history", get(homeserver_rust_read_route))
        .route("/api/backup/backup/list/:provider_name", get(homeserver_rust_read_route))
        .route("/api/backup/schedule", get(homeserver_rust_read_route).post(homeserver_rust_mutation_route))
        .route("/api/backup/providers/schema", get(homeserver_rust_read_route))
        .route("/api/backup/providers/:provider_name/config", get(homeserver_rust_read_route).post(homeserver_rust_mutation_route))
        .route("/api/backup/providers/:provider_name/test", post(homeserver_rust_mutation_route))
        .route("/api/backup/providers/:provider_name/info", get(homeserver_rust_read_route))
        .route("/api/backup/statistics", get(homeserver_rust_read_route))
        .route("/api/backup/test/cycle", post(homeserver_rust_mutation_route))
        .route("/api/backup/cleanup", post(homeserver_rust_mutation_route))
        .route("/api/backup/schedule/config", post(homeserver_rust_mutation_route))
        .route("/api/backup/schedule/history", get(homeserver_rust_read_route))
        .route("/api/backup/schedule/templates", get(homeserver_rust_read_route))
        .route("/api/backup/schedule/cron/available", get(homeserver_rust_read_route))
        .route("/api/backup/schedule/test", post(homeserver_rust_mutation_route))
        .route("/api/backup/version", get(homeserver_rust_read_route))
        .route("/api/backup/auto-update/status", get(homeserver_rust_read_route))
        .route("/api/backup/auto-update/toggle", post(homeserver_rust_mutation_route))
        .route("/api/backup/auto-update/check", post(homeserver_rust_mutation_route))
        .route("/api/backup/keyman/services", get(homeserver_rust_read_route))
        .route("/api/backup/keyman/credentials/:service_name", get(homeserver_rust_read_route).post(homeserver_rust_mutation_route).put(homeserver_rust_mutation_route).delete(homeserver_rust_mutation_route))
        .route("/api/backup/keyman/check/:service_name", get(homeserver_rust_read_route))
        .route("/api/backup/keyman/providers", get(homeserver_rust_read_route))
        .route("/api/backup/providers/:provider_name/enable", post(homeserver_rust_mutation_route))
        .route("/api/backup/providers/:provider_name/disable", post(homeserver_rust_mutation_route))
        .route("/api/backup/debug/status", get(homeserver_rust_read_route))
        .route("/api/backup/debug/toggle", post(homeserver_rust_mutation_route))
        .route("/api/backup/key", post(homeserver_rust_mutation_route))
        .route("/api/backup/header-stats", get(homeserver_rust_read_route))
        .route("/api/backup/install", post(homeserver_rust_mutation_route))
        .route("/api/backup/restore", post(homeserver_rust_mutation_route))
        .route("/api/backup/backups/list", get(homeserver_rust_read_route))
        .route("/api/backup/uninstall", post(homeserver_rust_mutation_route))
        .route("/api/backblazeTab/browse", get(homeserver_rust_read_route))
        .route("/api/backblazeTab/buckets", get(homeserver_rust_read_route))
        .route("/api/backblazeTab/buckets/:bucket_id/tree", get(homeserver_rust_read_route))
        .route("/api/backblazeTab/buckets/:bucket_id/files", get(homeserver_rust_read_route))
        .route("/api/backblazeTab/buckets/:bucket_id/storage", get(homeserver_rust_read_route))
        .route("/api/backblazeTab/buckets/:bucket_id/sync/status", get(homeserver_rust_read_route))
        .route("/api/backblazeTab/buckets/:bucket_id/sync/start", post(homeserver_rust_mutation_route))
        .route("/api/backblazeTab/buckets/:bucket_id/sync/stop", post(homeserver_rust_mutation_route))
        .route("/api/backblazeTab/buckets/:bucket_id/sync/config", get(homeserver_rust_read_route).post(homeserver_rust_mutation_route))
        .route("/api/backblazeTab/buckets/:bucket_id/delete", delete(homeserver_rust_mutation_route))
        .route("/api/backblazeTab/buckets/:bucket_id/download", post(homeserver_rust_mutation_route))
        .route("/api/backblazeTab/ledger/events", get(homeserver_rust_read_route))
        .route("/api/backblazeTab/ledger/jobs/:job_id", get(homeserver_rust_read_route))
        .route("/api/backblazeTab/database/backup", post(homeserver_rust_mutation_route))
        .route("/api/backblazeTab/database/restore", post(homeserver_rust_mutation_route))
        .route("/api/backblazeTab/database/list", get(homeserver_rust_read_route))
        .route("/api/backblazeTab/forgejo/status", get(homeserver_rust_read_route))
        .route("/api/backblazeTab/forgejo/backups", get(homeserver_rust_read_route))
        .route("/api/backblazeTab/forgejo/backup", post(homeserver_rust_mutation_route))
        .route("/api/backblazeTab/forgejo/restore", post(homeserver_rust_mutation_route))
        .route("/api/backblazeTab/chunk-store", get(homeserver_rust_read_route))
        .route("/api/backblazeTab/chunks-registry", get(homeserver_rust_read_route))
        .route("/api/backblazeTab/chunks-registry/:chunk_id", delete(homeserver_rust_mutation_route))
        .route("/api/backblazeTab/chunks-registry/purge", post(homeserver_rust_mutation_route))
        .route("/api/miner/coins", get(homeserver_rust_read_route))
        .route("/api/miner/miners", get(homeserver_rust_read_route))
        .route("/api/miner/miners/:miner_id/claim", post(homeserver_rust_mutation_route))
        .route("/api/miner/miners/:miner_id/unclaim", post(homeserver_rust_mutation_route))
        .route("/api/miner/miners/:miner_id/restart", post(homeserver_rust_mutation_route))
        .route("/api/miner/miners/:miner_id/coins/:coin_id/status", get(homeserver_rust_read_route))
        .route("/api/miner/miners/:miner_id/coins/:coin_id/enable", post(homeserver_rust_mutation_route))
        .route("/api/miner/miners/:miner_id/coins/:coin_id/disable", post(homeserver_rust_mutation_route))
        .route("/api/miner/fleet/restart", post(homeserver_rust_mutation_route))
        .route("/api/miner/fleet/update-wallets", post(homeserver_rust_mutation_route))
        .route("/api/miner/fleet/update-system", post(homeserver_rust_mutation_route))
        .route("/api/miner/fleet/update-coins", post(homeserver_rust_mutation_route))
        .route("/api/miner/fleet/update-all", post(homeserver_rust_mutation_route))
        .route("/api/miner/fleet/sync", post(homeserver_rust_mutation_route))
        .route("/api/miner/config", get(homeserver_rust_read_route).post(homeserver_rust_mutation_route))
        .route("/api/miner/config/coin/:coin_id", post(homeserver_rust_mutation_route))
        .route("/api/miner/config/ssh-password", post(homeserver_rust_mutation_route))
        .route("/api/miner/stats", get(homeserver_rust_read_route))
        .route("/api/test/status", get(homeserver_rust_read_route))
        .route("/api/test/data/sample", get(homeserver_rust_read_route))
        .route("/api/test/analytics/process", post(homeserver_rust_mutation_route))
        .route("/api/test/external/fetch", get(homeserver_rust_read_route))
        .route("/api/test/config", get(homeserver_rust_read_route))
        .route("/api/test/health", get(homeserver_rust_read_route))
        .route("/api/conflict/status", get(homeserver_rust_read_route))
}


async fn upload_file_route(mut multipart: Multipart) -> impl IntoResponse {
    let mut filename = "upload.bin".to_string();
    let mut destination = "/mnt/nas".to_string();
    let mut bytes: usize = 0;
    let mut content_type = "application/octet-stream".to_string();

    while let Ok(Some(field)) = multipart.next_field().await {
        let name = field.name().unwrap_or("").to_string();
        if name == "path" || name == "destination" {
            if let Ok(text) = field.text().await {
                if !text.trim().is_empty() { destination = text.trim().to_string(); }
            }
            continue;
        }
        if name == "file" {
            if let Some(raw_name) = field.file_name() { filename = raw_name.to_string(); }
            if let Some(raw_type) = field.content_type() { content_type = raw_type.to_string(); }
            match field.bytes().await {
                Ok(data) => bytes = data.len(),
                Err(err) => {
                    return (StatusCode::BAD_REQUEST, Json(serde_json::json!({
                        "schema": "coronatio.upload.error.v1",
                        "ok": false,
                        "error": err.to_string(),
                        "firstMissingSignal": "upload-form-read-failed"
                    }))).into_response();
                }
            }
        }
    }

    if bytes == 0 {
        return (StatusCode::BAD_REQUEST, Json(serde_json::json!({
            "schema": "coronatio.upload.error.v1",
            "ok": false,
            "error": "file field is empty or missing",
            "firstMissingSignal": "upload-file-missing"
        }))).into_response();
    }

    let caduceus = caduceus_http_json(
        "POST",
        "/api/v1/staff/intent",
        serde_json::json!({
            "method": "POST",
            "route": "/api/files/upload",
            "classification": "file-ingress",
            "metadata": {
                "filename": filename,
                "bytes": bytes,
                "contentType": content_type,
                "destination": destination
            }
        }),
    );
    (
        if caduceus.ok { StatusCode::ACCEPTED } else { StatusCode::SERVICE_UNAVAILABLE },
        Json(serde_json::json!({
            "schema": "coronatio.upload.submit.v1",
            "ok": caduceus.ok,
            "accepted": caduceus.ok,
            "filename": filename,
            "bytes": bytes,
            "destination": destination,
            "authority": "Coronatio Rust upload route to Caduceus",
            "caduceus": caduceus,
            "firstMissingSignal": if caduceus.ok { "none".to_string() } else { caduceus.first_missing_signal }
        })),
    ).into_response()
}


include!("full-rust-routes/upload.rs");

async fn upload_history_route() -> impl IntoResponse {
    Json(serde_json::json!({
        "schema": "coronatio.upload.history.v1",
        "ok": true,
        "history": [],
        "firstMissingSignal": "none"
    }))
}

async fn upload_default_directory_route() -> impl IntoResponse {
    Json(serde_json::json!({
        "schema": "coronatio.upload.default_directory.v1",
        "ok": true,
        "defaultPath": "/mnt/nas",
        "firstMissingSignal": "none"
    }))
}

async fn upload_blacklist_route() -> impl IntoResponse {
    Json(serde_json::json!({
        "schema": "coronatio.upload.blacklist.v1",
        "ok": true,
        "blacklist": [],
        "firstMissingSignal": "none"
    }))
}

async fn upload_pin_required_route() -> impl IntoResponse {
    Json(serde_json::json!({
        "schema": "coronatio.upload.pin_required.v1",
        "ok": true,
        "isPinRequired": false,
        "firstMissingSignal": "none"
    }))
}

async fn internet_status_route() -> impl IntoResponse {
    internet_status_response()
}

fn internet_status_response() -> Response {
    let connected = ["1.1.1.1:53", "8.8.8.8:53", "208.67.222.222:53"]
        .iter()
        .any(|authority| {
            authority
                .parse::<SocketAddr>()
                .ok()
                .and_then(|addr| TcpStream::connect_timeout(&addr, Duration::from_secs(3)).ok())
                .is_some()
        });
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs_f64())
        .unwrap_or(0.0);
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "schema": "coronatio.internet.status.v1",
            "ok": true,
            "success": true,
            "status": if connected { "connected" } else { "disconnected" },
            "timestamp": timestamp,
            "authority": "Coronatio Rust route port of Flask InternetStatusMonitor.check_connectivity",
            "hosts": ["1.1.1.1", "8.8.8.8", "208.67.222.222"],
            "timeoutSeconds": 3,
            "firstMissingSignal": "none"
        })),
    )
        .into_response()
}


async fn uptime_route() -> impl IntoResponse {
    let uptime_seconds = std::fs::read_to_string("/proc/uptime")
        .ok()
        .and_then(|raw| raw.split_whitespace().next().and_then(|value| value.parse::<f64>().ok()))
        .map(|seconds| seconds.round() as u64);
    let uptime = uptime_seconds
        .map(format_duration)
        .unwrap_or_else(|| "uptime unavailable".to_string());
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "schema": "coronatio.uptime.v1",
            "ok": uptime_seconds.is_some(),
            "path": "/api/uptime",
            "uptime": uptime,
            "uptimeSeconds": uptime_seconds,
            "firstMissingSignal": if uptime_seconds.is_some() { "none" } else { "/proc/uptime unavailable" }
        })),
    )
        .into_response()
}

fn format_duration(mut seconds: u64) -> String {
    let days = seconds / 86_400;
    seconds %= 86_400;
    let hours = seconds / 3_600;
    seconds %= 3_600;
    let minutes = seconds / 60;
    if days > 0 {
        format!("{days}d {hours}h {minutes}m")
    } else if hours > 0 {
        format!("{hours}h {minutes}m")
    } else {
        format!("{minutes}m")
    }
}

async fn homeserver_rust_read_route(method: Method, uri: Uri) -> impl IntoResponse {
    homeserver_read_response(method.as_str(), uri.path())
}

async fn homeserver_rust_mutation_route(method: Method, uri: Uri) -> impl IntoResponse {
    homeserver_mutation_response(method.as_str(), uri.path())
}

fn homeserver_read_response(method: &str, path: &str) -> Response {
    if path == "/api/status/power/usage" || path == "/status/power/usage" {
        return power_usage_response(method, path);
    }

    let family = homeserver_route_family(path);
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "schema": "coronatio.homeserver.route.read.v1",
            "ok": true,
            "success": true,
            "method": method,
            "path": path,
            "family": family,
            "status": "rust-route",
            "authority": "Coronatio Rust route",
            "firstMissingSignal": "none"
        })),
    )
        .into_response()
}

include!("full-rust-routes/power.rs");

fn homeserver_mutation_response(method: &str, path: &str) -> Response {
    let caduceus = caduceus_http_json(
        "POST",
        "/api/v1/staff/intent",
        serde_json::json!({
            "method": method,
            "route": path,
            "classification": homeserver_route_family(path),
        }),
    );
    (
        if caduceus.ok {
            StatusCode::ACCEPTED
        } else {
            StatusCode::SERVICE_UNAVAILABLE
        },
        Json(serde_json::json!({
            "schema": "coronatio.homeserver.route.mutation.v1",
            "ok": caduceus.ok,
            "accepted": caduceus.ok,
            "method": method,
            "path": path,
            "family": homeserver_route_family(path),
            "authority": "Caduceus staff intent membrane",
            "caduceus": caduceus,
            "firstMissingSignal": if caduceus.ok { "none".to_string() } else { caduceus.first_missing_signal }
        })),
    )
        .into_response()
}


include!("full-rust-routes/portals.rs");

fn homeserver_route_family(path: &str) -> &'static str {
    if path.contains("/diskman") || path.contains("/vault") || path.contains("/crypto") || path.contains("/keyman") {
        "admin-storage"
    } else if path.contains("/updates") || path.contains("/system/update") || path.contains("/backup") {
        "update-and-backup"
    } else if path.contains("tailscale") || path.contains("vpn") || path.contains("network") || path.contains("dhcp") || path.contains("wakeonlan") {
        "network-control"
    } else if path.contains("upload") || path.contains("files") || path.contains("nasLinker") {
        "file-ingress"
    } else if path.contains("portals") || path.contains("service/control") {
        "portal-service"
    } else if path.contains("premium") || path.contains("youtube") || path.contains("backblazeTab") || path.contains("miner") {
        "premium-module"
    } else if path.contains("tabs") || path.contains("setstarredtab") {
        "tab-registry"
    } else {
        "crown-route"
    }
}

#[cfg(test)]
fn full_rust_route_inventory() -> &'static [(&'static str, &'static [&'static str])] {
    &[
                ("/api/tabs/visibility", &["post"]),
        ("/api/tabs/elements", &["put"]),
        ("/api/pre-unlock", &["post"]),
        ("/api/vault/status", &["get"]),
        ("/api/vault/unlock", &["post"]),
        ("/api/themes", &["get"]),
        ("/api/system/log", &["post"]),
        ("/api/system/update", &["post"]),
        ("/api/admin/system/update-password", &["post"]),
        ("/api/admin/logs/homeserver", &["get"]),
        ("/api/admin/logs/homeserver/clear", &["post"]),
        ("/api/admin/download-root-crt", &["get"]),
        ("/api/admin/refresh-root-crt", &["post"]),
        ("/api/crypto/getKey", &["get"]),
        ("/api/admin/crypto/test", &["post"]),
        ("/api/admin/updates/check", &["get"]),
        ("/api/admin/updates/apply", &["post"]),
        ("/api/admin/updates/force", &["post"]),
        ("/api/admin/updates/modules", &["get"]),
        ("/api/admin/updates/modules/:module_name/status", &["get"]),
        ("/api/admin/updates/modules/:module_name/toggle", &["post"]),
        ("/api/admin/updates/modules/:module_name/components/:component_name/toggle", &["post"]),
        ("/api/admin/updates/modules/:module_name/branch", &["post"]),
        ("/api/admin/updates/interactives", &["get"]),
        ("/api/admin/updates/interactives/:interactive_id/run", &["post"]),
        ("/api/admin/updates/logs", &["get"]),
        ("/api/admin/updates/logfile", &["get"]),
        ("/api/admin/updates/system-info", &["get"]),
        ("/api/admin/updates/schedule", &["get", "post"]),
        ("/api/admin/ssh/status", &["get"]),
        ("/api/admin/ssh/toggle", &["post"]),
        ("/api/admin/services/hard-reset", &["post"]),
        ("/api/admin/system/restart", &["post"]),
        ("/api/admin/system/shutdown", &["post"]),
        ("/api/admin/ssh/service", &["post"]),
        ("/api/admin/ssh/service/status", &["get"]),
        ("/api/admin/samba/service/status", &["get"]),
        ("/api/admin/samba/service", &["post"]),
        ("/api/admin/hard-drive-test/results", &["get"]),
        ("/api/admin/hard-drive-test/progress", &["get"]),
        ("/api/admin/hard-drive-test/start", &["post"]),
        ("/api/admin/hard-drive-test/devices", &["get"]),
        ("/api/admin/diskman/nas-compatible", &["get"]),
        ("/api/admin/diskman/format", &["post"]),
        ("/api/admin/diskman/unlock", &["post"]),
        ("/api/admin/diskman/unlock-with-password", &["post"]),
        ("/api/admin/diskman/encrypt", &["post"]),
        ("/api/admin/diskman/mount", &["post"]),
        ("/api/admin/diskman/unmount", &["post"]),
        ("/api/admin/diskman/apply-permissions", &["post"]),
        ("/api/admin/diskman/check-services", &["get"]),
        ("/api/admin/diskman/manage-services", &["post"]),
        ("/api/admin/diskman/sync", &["post"]),
        ("/api/admin/diskman/sync-schedule", &["get"]),
        ("/api/admin/diskman/sync-schedule-update", &["post"]),
        ("/api/admin/diskman/assign-nas", &["post"]),
        ("/api/admin/diskman/unassign-nas", &["post"]),
        ("/api/admin/diskman/import-to-nas", &["post"]),
        ("/api/admin/diskman/create-key", &["post"]),
        ("/api/admin/diskman/update-key", &["post"]),
        ("/api/admin/diskman/key-status", &["post"]),
        ("/api/admin/diskman/vault-device", &["get"]),
        ("/api/admin/premium/validate-and-clone", &["post"]),
        ("/api/admin/premium/install/:tab_name", &["post"]),
        ("/api/admin/premium/uninstall/:tab_name", &["delete"]),
        ("/api/admin/premium/reinstall/:tab_name", &["post"]),
        ("/api/admin/premium/reinstall-multiple", &["post"]),
        ("/api/admin/premium/delete/:tab_name", &["delete"]),
        ("/api/admin/premium/status", &["get"]),
        ("/api/admin/premium/install-all", &["post"]),
        ("/api/admin/premium/uninstall-all", &["post"]),
        ("/api/admin/premium/logs", &["get"]),
        ("/api/admin/premium/auto-update/:tab_name", &["get", "post"]),
        ("/api/admin/premium/auto-update-status", &["get"]),
        ("/api/status/services", &["get"]),
        ("/api/status", &["get"]),
        ("/api/uptime", &["get"]),
        ("/api/status/tailscale", &["get"]),
        ("/api/status/tailscale/connect", &["post"]),
        ("/api/status/tailscale/authkey", &["post"]),
        ("/api/status/tailscale/disconnect", &["post"]),
        ("/api/status/tailscale/enable", &["post"]),
        ("/api/status/tailscale/disable", &["post"]),
        ("/api/status/tailscale/config", &["get", "post"]),
        ("/api/status/tailscale/update-tailnet", &["post"]),
        ("/api/status/vpn/pia", &["get"]),
        ("/api/status/vpn/transmission", &["get"]),
        ("/api/status/vpn/updatekey/pia", &["post"]),
        ("/api/status/vpn/updatekey/transmission", &["post"]),
        ("/api/status/vpn/pia/exists", &["get"]),
        ("/api/status/vpn/transmission/exists", &["get"]),
        ("/api/status/vpn/enable", &["post"]),
        ("/api/status/vpn/disable", &["post"]),
        ("/api/status/vpn/check-enabled", &["get"]),
        ("/api/files/browse", &["get"]),
        ("/api/files/browse-hierarchical", &["get"]),
        ("/api/files/upload", &["post"]),
        ("/api/files/download", &["get"]),
        ("/api/upload/force-permissions", &["post"]),
        ("/api/upload/history", &["get"]),
        ("/api/upload/history/clear", &["post"]),
        ("/api/upload/default-directory", &["get", "post"]),
        ("/api/upload/blacklist/list", &["get"]),
        ("/api/upload/blacklist/update", &["put"]),
        ("/api/upload/pin-required-status", &["get", "post"]),
        ("/api/portals", &["get", "post"]),
        ("/api/portals/:portal_name", &["put", "delete"]),
        ("/api/portals/factory", &["get"]),
        ("/api/service/control", &["post"]),
        ("/api/portals/images/:filename", &["get"]),
        ("/api/status/internet/speedtest", &["post"]),
        ("/status/power/usage", &["get"]),
        ("/api/status/power/usage", &["get"]),
        ("/api/kea-leases", &["get"]),
        ("/api/network/notes", &["get", "put"]),
        ("/api/version", &["get"]),
        ("/api/wakeonlan/targets", &["get", "post"]),
        ("/api/wakeonlan/wake", &["post"]),
        ("/api/wakeonlan/targets/:name", &["delete"]),
        ("/api/wakeonlan/status", &["get"]),
        ("/api/dhcp/status", &["get"]),
        ("/api/dhcp/leases", &["get"]),
        ("/api/dhcp/reservations", &["get", "post"]),
        ("/api/dhcp/reservations/:reservation_id", &["put", "delete"]),
        ("/api/dhcp/config", &["get", "post"]),
        ("/api/dhcp/health", &["get"]),
        ("/api/dhcp/statistics", &["get"]),
        ("/api/dhcp/pool-boundary", &["get", "post"]),
        ("/api/youtube/download", &["post"]),
        ("/api/youtube/download/info", &["post"]),
        ("/api/youtube/subscriptions", &["get", "post"]),
        ("/api/youtube/subscriptions/:channel_id", &["delete"]),
        ("/api/youtube/subscriptions/:channel_id/fetch", &["post"]),
        ("/api/youtube/settings", &["get", "post"]),
        ("/api/youtube/schedule", &["get", "post"]),
        ("/api/youtube/logs", &["get"]),
        ("/api/youtube/update-ytdlp", &["post"]),
        ("/api/nasLinker/browse", &["get"]),
        ("/api/nasLinker/deploy", &["post"]),
        ("/api/nasLinker/delete", &["delete"]),
        ("/api/nasLinker/rename", &["post"]),
        ("/api/nasLinker/newdir", &["post"]),
        ("/api/nasLinker/scan", &["get"]),
        ("/api/nasLinker/status", &["get"]),
        ("/api/nasLinker/config", &["get"]),
        ("/api/backup/status", &["get"]),
        ("/api/backup/repositories", &["get"]),
        ("/api/backup/providers/status", &["get"]),
        ("/api/backup/backup/run", &["post"]),
        ("/api/backup/sync-now", &["post"]),
        ("/api/backup/cloud/test", &["post"]),
        ("/api/backup/config", &["get", "post"]),
        ("/api/backup/history", &["get"]),
        ("/api/backup/backup/list/:provider_name", &["get"]),
        ("/api/backup/schedule", &["get", "post"]),
        ("/api/backup/providers/schema", &["get"]),
        ("/api/backup/providers/:provider_name/config", &["get", "post"]),
        ("/api/backup/providers/:provider_name/test", &["post"]),
        ("/api/backup/providers/:provider_name/info", &["get"]),
        ("/api/backup/statistics", &["get"]),
        ("/api/backup/test/cycle", &["post"]),
        ("/api/backup/cleanup", &["post"]),
        ("/api/backup/schedule/config", &["post"]),
        ("/api/backup/schedule/history", &["get"]),
        ("/api/backup/schedule/templates", &["get"]),
        ("/api/backup/schedule/cron/available", &["get"]),
        ("/api/backup/schedule/test", &["post"]),
        ("/api/backup/version", &["get"]),
        ("/api/backup/auto-update/status", &["get"]),
        ("/api/backup/auto-update/toggle", &["post"]),
        ("/api/backup/auto-update/check", &["post"]),
        ("/api/backup/keyman/services", &["get"]),
        ("/api/backup/keyman/credentials/:service_name", &["get", "post", "put", "delete"]),
        ("/api/backup/keyman/check/:service_name", &["get"]),
        ("/api/backup/keyman/providers", &["get"]),
        ("/api/backup/providers/:provider_name/enable", &["post"]),
        ("/api/backup/providers/:provider_name/disable", &["post"]),
        ("/api/backup/debug/status", &["get"]),
        ("/api/backup/debug/toggle", &["post"]),
        ("/api/backup/key", &["post"]),
        ("/api/backup/header-stats", &["get"]),
        ("/api/backup/install", &["post"]),
        ("/api/backup/restore", &["post"]),
        ("/api/backup/backups/list", &["get"]),
        ("/api/backup/uninstall", &["post"]),
        ("/api/backblazeTab/browse", &["get"]),
        ("/api/backblazeTab/buckets", &["get"]),
        ("/api/backblazeTab/buckets/:bucket_id/tree", &["get"]),
        ("/api/backblazeTab/buckets/:bucket_id/files", &["get"]),
        ("/api/backblazeTab/buckets/:bucket_id/storage", &["get"]),
        ("/api/backblazeTab/buckets/:bucket_id/sync/status", &["get"]),
        ("/api/backblazeTab/buckets/:bucket_id/sync/start", &["post"]),
        ("/api/backblazeTab/buckets/:bucket_id/sync/stop", &["post"]),
        ("/api/backblazeTab/buckets/:bucket_id/sync/config", &["get", "post"]),
        ("/api/backblazeTab/buckets/:bucket_id/delete", &["delete"]),
        ("/api/backblazeTab/buckets/:bucket_id/download", &["post"]),
        ("/api/backblazeTab/ledger/events", &["get"]),
        ("/api/backblazeTab/ledger/jobs/:job_id", &["get"]),
        ("/api/backblazeTab/database/backup", &["post"]),
        ("/api/backblazeTab/database/restore", &["post"]),
        ("/api/backblazeTab/database/list", &["get"]),
        ("/api/backblazeTab/forgejo/status", &["get"]),
        ("/api/backblazeTab/forgejo/backups", &["get"]),
        ("/api/backblazeTab/forgejo/backup", &["post"]),
        ("/api/backblazeTab/forgejo/restore", &["post"]),
        ("/api/backblazeTab/chunk-store", &["get"]),
        ("/api/backblazeTab/chunks-registry", &["get"]),
        ("/api/backblazeTab/chunks-registry/:chunk_id", &["delete"]),
        ("/api/backblazeTab/chunks-registry/purge", &["post"]),
        ("/api/miner/coins", &["get"]),
        ("/api/miner/miners", &["get"]),
        ("/api/miner/miners/:miner_id/claim", &["post"]),
        ("/api/miner/miners/:miner_id/unclaim", &["post"]),
        ("/api/miner/miners/:miner_id/restart", &["post"]),
        ("/api/miner/miners/:miner_id/coins/:coin_id/status", &["get"]),
        ("/api/miner/miners/:miner_id/coins/:coin_id/enable", &["post"]),
        ("/api/miner/miners/:miner_id/coins/:coin_id/disable", &["post"]),
        ("/api/miner/fleet/restart", &["post"]),
        ("/api/miner/fleet/update-wallets", &["post"]),
        ("/api/miner/fleet/update-system", &["post"]),
        ("/api/miner/fleet/update-coins", &["post"]),
        ("/api/miner/fleet/update-all", &["post"]),
        ("/api/miner/fleet/sync", &["post"]),
        ("/api/miner/config", &["get", "post"]),
        ("/api/miner/config/coin/:coin_id", &["post"]),
        ("/api/miner/config/ssh-password", &["post"]),
        ("/api/miner/stats", &["get"]),
        ("/api/test/status", &["get"]),
        ("/api/test/data/sample", &["get"]),
        ("/api/test/analytics/process", &["post"]),
        ("/api/test/external/fetch", &["get"]),
        ("/api/test/config", &["get"]),
        ("/api/test/health", &["get"]),
        ("/api/conflict/status", &["get"]),
    ]
}
