fn full_rust_route_table() -> Router<AppState> {
    Router::new()
        .route("/api/tabs/visibility", post(tab_visibility_route))
        .route("/api/tabs/elements", put(element_visibility_route))
        .route("/api/tabs/elements/:tab_id", get(element_fragment_route))
        .route("/api/stats/elements", get(stats_elements_fragment_route))
        .route("/api/portals/elements", get(portals_elements_fragment_route))
        .route("/api/pre-unlock", post(admin_class_generic_mutation_route))
        .route("/api/vault/status", get(homeserver_rust_read_route))
        .route("/api/vault/unlock", post(admin_class_generic_mutation_route))
        .route("/api/system/log", post(admin_class_generic_mutation_route))
        .route("/api/system/update", post(admin_class_generic_mutation_route))
        .route("/api/admin/system/update-password", post(admin_class_generic_mutation_route))
        .route("/api/admin/logs/homeserver", get(homeserver_rust_read_route))
        .route("/api/admin/logs/homeserver/clear", post(admin_class_generic_mutation_route))
        .route("/api/admin/download-root-crt", get(hestia_bundle_download_route))
        .route("/api/crypto/getKey", get(homeserver_rust_read_route))
        .route("/api/admin/crypto/test", post(admin_class_generic_mutation_route))
        .route("/api/admin/updates/check", get(homeserver_rust_read_route))
        .route("/api/admin/updates/apply", post(admin_class_generic_mutation_route))
        .route("/api/admin/updates/force", post(admin_class_generic_mutation_route))
        .route("/api/admin/updates/modules", get(homeserver_rust_read_route))
        .route("/api/admin/updates/modules/:module_name/status", get(homeserver_rust_read_route))
        .route("/api/admin/updates/modules/:module_name/toggle", post(admin_class_generic_mutation_route))
        .route("/api/admin/updates/modules/:module_name/components/:component_name/toggle", post(admin_class_generic_mutation_route))
        .route("/api/admin/updates/modules/:module_name/branch", post(admin_class_generic_mutation_route))
        .route("/api/admin/updates/interactives", get(homeserver_rust_read_route))
        .route("/api/admin/updates/interactives/:interactive_id/run", post(admin_class_generic_mutation_route))
        .route("/api/admin/updates/logs", get(homeserver_rust_read_route))
        .route("/api/admin/updates/logfile", get(homeserver_rust_read_route))
        .route("/api/admin/updates/system-info", get(homeserver_rust_read_route))
        .route("/api/admin/updates/schedule", get(homeserver_rust_read_route).post(admin_class_generic_mutation_route))
        .route("/api/admin/ssh/status", get(homeserver_rust_read_route))
        .route("/api/admin/ssh/toggle", post(admin_class_generic_mutation_route))
        .route("/api/admin/services/hard-reset", post(admin_class_generic_mutation_route))
        .route("/api/admin/system/restart", post(admin_class_generic_mutation_route))
        .route("/api/admin/system/shutdown", post(admin_class_generic_mutation_route))
        .route("/api/admin/ssh/service", post(admin_class_generic_mutation_route))
        .route("/api/admin/ssh/service/status", get(homeserver_rust_read_route))
        .route("/api/admin/samba/service/status", get(homeserver_rust_read_route))
        .route("/api/admin/samba/service", post(admin_class_generic_mutation_route))
        .route("/api/admin/hard-drive-test/results", get(homeserver_rust_read_route))
        .route("/api/admin/hard-drive-test/progress", get(homeserver_rust_read_route))
        .route("/api/admin/hard-drive-test/start", post(admin_class_generic_mutation_route))
        .route("/api/admin/hard-drive-test/devices", get(homeserver_rust_read_route))
        .route("/api/admin/diskman/nas-compatible", get(homeserver_rust_read_route))
        .route("/api/admin/diskman/format", post(admin_class_generic_mutation_route))
        .route("/api/admin/diskman/unlock", post(admin_class_generic_mutation_route))
        .route("/api/admin/diskman/unlock-with-password", post(admin_class_generic_mutation_route))
        .route("/api/admin/diskman/encrypt", post(admin_class_generic_mutation_route))
        .route("/api/admin/diskman/mount", post(admin_class_generic_mutation_route))
        .route("/api/admin/diskman/unmount", post(admin_class_generic_mutation_route))
        .route("/api/admin/diskman/apply-permissions", post(admin_class_generic_mutation_route))
        .route("/api/admin/diskman/check-services", get(homeserver_rust_read_route))
        .route("/api/admin/diskman/manage-services", post(admin_class_generic_mutation_route))
        .route("/api/admin/diskman/sync", post(admin_class_generic_mutation_route))
        .route("/api/admin/diskman/sync-schedule", get(homeserver_rust_read_route))
        .route("/api/admin/diskman/sync-schedule-update", post(admin_class_generic_mutation_route))
        .route("/api/admin/diskman/assign-nas", post(admin_class_generic_mutation_route))
        .route("/api/admin/diskman/unassign-nas", post(admin_class_generic_mutation_route))
        .route("/api/admin/diskman/import-to-nas", post(admin_class_generic_mutation_route))
        .route("/api/admin/diskman/create-key", post(admin_class_generic_mutation_route))
        .route("/api/admin/diskman/update-key", post(admin_class_generic_mutation_route))
        .route("/api/admin/diskman/key-status", post(admin_class_generic_mutation_route))
        .route("/api/admin/diskman/vault-device", get(homeserver_rust_read_route))
        .route("/api/status/services", get(homeserver_rust_read_route))
        .route("/api/status", get(internet_status_route))
        .route("/api/uptime", get(uptime_route))
        .route("/api/status/tailscale", get(homeserver_rust_read_route))
        .route("/api/status/tailscale/connect", post(network_identity_mutation_route))
        .route("/api/status/tailscale/authkey", post(network_identity_mutation_route))
        .route("/api/status/tailscale/disconnect", post(network_identity_mutation_route))
        .route("/api/status/tailscale/enable", post(network_identity_mutation_route))
        .route("/api/status/tailscale/disable", post(network_identity_mutation_route))
        .route("/api/status/tailscale/config", get(homeserver_rust_read_route).post(network_identity_mutation_route))
        .route("/api/status/tailscale/update-tailnet", post(network_identity_mutation_route))
        .route("/api/status/vpn/pia", get(homeserver_rust_read_route))
        .route("/api/status/vpn/transmission", get(homeserver_rust_read_route))
        .route("/api/status/vpn/updatekey/pia", post(admin_class_generic_mutation_route))
        .route("/api/status/vpn/updatekey/transmission", post(admin_class_generic_mutation_route))
        .route("/api/status/vpn/pia/exists", get(homeserver_rust_read_route))
        .route("/api/status/vpn/transmission/exists", get(homeserver_rust_read_route))
        .route("/api/status/vpn/enable", post(admin_class_generic_mutation_route))
        .route("/api/status/vpn/disable", post(admin_class_generic_mutation_route))
        .route("/api/status/vpn/check-enabled", get(homeserver_rust_read_route))
        .route("/api/files/browse", get(upload_browse_hierarchical_route))
        .route("/api/files/browse-hierarchical", get(upload_browse_hierarchical_route))
        .route("/api/files/upload", post(upload_file_route))
        .route("/api/files/download", get(homeserver_rust_read_route))
        .route("/api/upload/force-permissions", post(admin_class_generic_mutation_route))
        .route("/api/upload/history", get(upload_history_admin_route))
        .route("/api/upload/history/clear", post(admin_class_generic_mutation_route))
        .route("/api/upload/default-directory", get(upload_default_directory_route).post(upload_default_directory_update_route))
        .route("/api/upload/blacklist/list", get(upload_blacklist_admin_route))
        .route("/api/upload/blacklist/update", put(admin_class_generic_mutation_route))
        .route("/api/upload/pin-required-status", get(upload_pin_required_route).post(admin_class_generic_mutation_route))
        .route("/api/portals", get(portals_config_route).post(admin_class_generic_mutation_route))
        .route("/api/portals/currentness", get(portals_currentness_route))
        .route("/api/portals/:portal_name", put(admin_class_generic_mutation_route).delete(admin_class_generic_mutation_route))
        .route("/api/portals/factory", get(portals_factory_route))
        .route("/api/service/control", post(portal_service_control_route))
        .route("/api/portals/images/:filename", get(portal_image_route))
        .route("/api/status/internet/speedtest", post(admin_class_generic_mutation_route))
        .route("/status/power/usage", get(homeserver_rust_read_route))
        .route("/api/status/power/usage", get(homeserver_rust_read_route))
        .route("/api/kea-leases", get(homeserver_rust_read_route))
        .route("/api/network/notes", get(network_notes_read_route).put(network_notes_write_route))
        .route("/api/version", get(homeserver_rust_read_route))
        .route("/api/wakeonlan/targets", get(homeserver_rust_read_route).post(network_identity_mutation_route))
        .route("/api/wakeonlan/wake", post(network_identity_mutation_route))
        .route("/api/wakeonlan/targets/:name", delete(network_identity_mutation_route))
        .route("/api/wakeonlan/status", get(homeserver_rust_read_route))
        .route("/api/dhcp/status", get(dhcp_read_route))
        .route("/api/dhcp/leases", get(dhcp_read_route))
        .route("/api/dhcp/reservations", get(dhcp_read_route).post(dhcp_reservation_create_route))
        .route("/api/dhcp/reservations/:reservation_id", put(dhcp_reservation_update_route).delete(dhcp_reservation_delete_route))
        .route("/api/dhcp/config", get(dhcp_read_route).post(network_identity_mutation_route))
        .route("/api/dhcp/health", get(dhcp_read_route))
        .route("/api/dhcp/statistics", get(dhcp_read_route))
        .route("/api/dhcp/pool-boundary", get(dhcp_read_route).post(dhcp_pool_boundary_route))
        .route("/api/dns/records", get(dns_records_get_route).post(dns_records_post_route))
        .route("/api/dns/records/:name", delete(dns_records_delete_route))
        .route("/api/firewall/status", get(firewall_read_route))
        .route("/api/firewall/policies", get(firewall_read_route))
        .route("/api/firewall/policies/:mac", get(firewall_policy_route).put(firewall_policy_put_route).delete(firewall_policy_delete_route))
        .route("/api/nasLinker/browse", get(homeserver_rust_read_route))
        .route("/api/nasLinker/deploy", post(admin_class_generic_mutation_route))
        .route("/api/nasLinker/delete", delete(admin_class_generic_mutation_route))
        .route("/api/nasLinker/rename", post(admin_class_generic_mutation_route))
        .route("/api/nasLinker/newdir", post(admin_class_generic_mutation_route))
        .route("/api/nasLinker/scan", get(homeserver_rust_read_route))
        .route("/api/nasLinker/status", get(homeserver_rust_read_route))
        .route("/api/nasLinker/config", get(homeserver_rust_read_route))
        .route("/api/backup/status", get(homeserver_rust_read_route))
        .route("/api/backup/repositories", get(homeserver_rust_read_route))
        .route("/api/backup/providers/status", get(homeserver_rust_read_route))
        .route("/api/backup/backup/run", post(admin_class_generic_mutation_route))
        .route("/api/backup/sync-now", post(admin_class_generic_mutation_route))
        .route("/api/backup/cloud/test", post(admin_class_generic_mutation_route))
        .route("/api/backup/config", get(homeserver_rust_read_route).post(admin_class_generic_mutation_route))
        .route("/api/backup/history", get(homeserver_rust_read_route))
        .route("/api/backup/backup/list/:provider_name", get(homeserver_rust_read_route))
        .route("/api/backup/schedule", get(homeserver_rust_read_route).post(admin_class_generic_mutation_route))
        .route("/api/backup/providers/schema", get(homeserver_rust_read_route))
        .route("/api/backup/providers/:provider_name/config", get(homeserver_rust_read_route).post(admin_class_generic_mutation_route))
        .route("/api/backup/providers/:provider_name/test", post(admin_class_generic_mutation_route))
        .route("/api/backup/providers/:provider_name/info", get(homeserver_rust_read_route))
        .route("/api/backup/statistics", get(homeserver_rust_read_route))
        .route("/api/backup/test/cycle", post(admin_class_generic_mutation_route))
        .route("/api/backup/cleanup", post(admin_class_generic_mutation_route))
        .route("/api/backup/schedule/config", post(admin_class_generic_mutation_route))
        .route("/api/backup/schedule/history", get(homeserver_rust_read_route))
        .route("/api/backup/schedule/templates", get(homeserver_rust_read_route))
        .route("/api/backup/schedule/cron/available", get(homeserver_rust_read_route))
        .route("/api/backup/schedule/test", post(admin_class_generic_mutation_route))
        .route("/api/backup/version", get(homeserver_rust_read_route))
        .route("/api/backup/auto-update/status", get(homeserver_rust_read_route))
        .route("/api/backup/auto-update/toggle", post(admin_class_generic_mutation_route))
        .route("/api/backup/auto-update/check", post(admin_class_generic_mutation_route))
        .route("/api/backup/keyman/services", get(homeserver_rust_read_route))
        .route("/api/backup/keyman/credentials/:service_name", get(homeserver_rust_read_route).post(admin_class_generic_mutation_route).put(admin_class_generic_mutation_route).delete(admin_class_generic_mutation_route))
        .route("/api/backup/keyman/check/:service_name", get(homeserver_rust_read_route))
        .route("/api/backup/keyman/providers", get(homeserver_rust_read_route))
        .route("/api/backup/providers/:provider_name/enable", post(admin_class_generic_mutation_route))
        .route("/api/backup/providers/:provider_name/disable", post(admin_class_generic_mutation_route))
        .route("/api/backup/debug/status", get(homeserver_rust_read_route))
        .route("/api/backup/debug/toggle", post(admin_class_generic_mutation_route))
        .route("/api/backup/key", post(admin_class_generic_mutation_route))
        .route("/api/backup/header-stats", get(homeserver_rust_read_route))
        .route("/api/backup/install", post(admin_class_generic_mutation_route))
        .route("/api/backup/restore", post(admin_class_generic_mutation_route))
        .route("/api/backup/backups/list", get(homeserver_rust_read_route))
        .route("/api/backup/uninstall", post(admin_class_generic_mutation_route))
        .route("/api/backblazeTab/browse", get(homeserver_rust_read_route))
        .route("/api/backblazeTab/buckets", get(homeserver_rust_read_route))
        .route("/api/backblazeTab/buckets/:bucket_id/tree", get(homeserver_rust_read_route))
        .route("/api/backblazeTab/buckets/:bucket_id/files", get(homeserver_rust_read_route))
        .route("/api/backblazeTab/buckets/:bucket_id/storage", get(homeserver_rust_read_route))
        .route("/api/backblazeTab/buckets/:bucket_id/sync/status", get(homeserver_rust_read_route))
        .route("/api/backblazeTab/buckets/:bucket_id/sync/start", post(admin_class_generic_mutation_route))
        .route("/api/backblazeTab/buckets/:bucket_id/sync/stop", post(admin_class_generic_mutation_route))
        .route("/api/backblazeTab/buckets/:bucket_id/sync/config", get(homeserver_rust_read_route).post(admin_class_generic_mutation_route))
        .route("/api/backblazeTab/buckets/:bucket_id/delete", delete(admin_class_generic_mutation_route))
        .route("/api/backblazeTab/buckets/:bucket_id/download", post(admin_class_generic_mutation_route))
        .route("/api/backblazeTab/ledger/events", get(homeserver_rust_read_route))
        .route("/api/backblazeTab/ledger/jobs/:job_id", get(homeserver_rust_read_route))
        .route("/api/backblazeTab/database/backup", post(admin_class_generic_mutation_route))
        .route("/api/backblazeTab/database/restore", post(admin_class_generic_mutation_route))
        .route("/api/backblazeTab/database/list", get(homeserver_rust_read_route))
        .route("/api/backblazeTab/forgejo/status", get(homeserver_rust_read_route))
        .route("/api/backblazeTab/forgejo/backups", get(homeserver_rust_read_route))
        .route("/api/backblazeTab/forgejo/backup", post(admin_class_generic_mutation_route))
        .route("/api/backblazeTab/forgejo/restore", post(admin_class_generic_mutation_route))
        .route("/api/backblazeTab/chunk-store", get(homeserver_rust_read_route))
        .route("/api/backblazeTab/chunks-registry", get(homeserver_rust_read_route))
        .route("/api/backblazeTab/chunks-registry/:chunk_id", delete(admin_class_generic_mutation_route))
        .route("/api/backblazeTab/chunks-registry/purge", post(admin_class_generic_mutation_route))
        .route("/api/miner/coins", get(homeserver_rust_read_route))
        .route("/api/miner/miners", get(homeserver_rust_read_route))
        .route("/api/miner/miners/:miner_id/claim", post(admin_class_generic_mutation_route))
        .route("/api/miner/miners/:miner_id/unclaim", post(admin_class_generic_mutation_route))
        .route("/api/miner/miners/:miner_id/restart", post(admin_class_generic_mutation_route))
        .route("/api/miner/miners/:miner_id/coins/:coin_id/status", get(homeserver_rust_read_route))
        .route("/api/miner/miners/:miner_id/coins/:coin_id/enable", post(admin_class_generic_mutation_route))
        .route("/api/miner/miners/:miner_id/coins/:coin_id/disable", post(admin_class_generic_mutation_route))
        .route("/api/miner/fleet/restart", post(admin_class_generic_mutation_route))
        .route("/api/miner/fleet/update-wallets", post(admin_class_generic_mutation_route))
        .route("/api/miner/fleet/update-system", post(admin_class_generic_mutation_route))
        .route("/api/miner/fleet/update-coins", post(admin_class_generic_mutation_route))
        .route("/api/miner/fleet/update-all", post(admin_class_generic_mutation_route))
        .route("/api/miner/fleet/sync", post(admin_class_generic_mutation_route))
        .route("/api/miner/config", get(homeserver_rust_read_route).post(admin_class_generic_mutation_route))
        .route("/api/miner/config/coin/:coin_id", post(admin_class_generic_mutation_route))
        .route("/api/miner/config/ssh-password", post(admin_class_generic_mutation_route))
        .route("/api/miner/stats", get(homeserver_rust_read_route))
        .route("/api/test/status", get(homeserver_rust_read_route))
        .route("/api/test/data/sample", get(homeserver_rust_read_route))
        .route("/api/test/analytics/process", post(admin_class_generic_mutation_route))
        .route("/api/test/external/fetch", get(homeserver_rust_read_route))
        .route("/api/test/config", get(homeserver_rust_read_route))
        .route("/api/test/health", get(homeserver_rust_read_route))
        .route("/api/conflict/status", get(homeserver_rust_read_route))
}


async fn upload_file_route(headers: axum::http::HeaderMap, mut multipart: Multipart) -> impl IntoResponse {
    if upload_pin_required() && session_from_headers(&headers) != Session::Admin {
        return (
            StatusCode::PRECONDITION_REQUIRED,
            Json(serde_json::json!({
                "schema": "coronatio.upload.pin_required.refusal.v1",
                "ok": false,
                "accepted": false,
                "error": "upload-pin-required",
                "firstMissingSignal": "upload-pin-required"
            })),
        )
            .into_response();
    }
    let mut filename = "upload.bin".to_string();
    let mut destination = "/mnt/nas".to_string();
    let mut payload = Vec::new();
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
                Ok(data) => payload = data.to_vec(),
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

    if payload.is_empty() {
        return (StatusCode::BAD_REQUEST, Json(serde_json::json!({
            "schema": "coronatio.upload.error.v1",
            "ok": false,
            "error": "file field is empty or missing",
            "firstMissingSignal": "upload-file-missing"
        }))).into_response();
    }

    let root = upload_root_path();
    let display_root = upload_display_root(&root);
    let filename_path = std::path::Path::new(&filename);
    if filename_path.file_name().and_then(|value| value.to_str()) != Some(filename.as_str())
        || upload_resolve_path(&root, &display_root, &destination).is_none()
        || upload_path_blacklisted(&destination)
        || !upload_root_available(&root)
    {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "schema":"coronatio.upload.error.v1", "ok":false,
                "firstMissingSignal":"upload-validation-failed"
            })),
        )
            .into_response();
    }
    let byte_count = payload.len();

    let caduceus = mutation_staff_intent(
        &mutation_authority(),
        &headers,
        "POST",
        "/api/files/upload",
        "file-ingress",
        serde_json::json!({
            "filename": filename,
            "bytes": byte_count,
            "contentType": content_type,
            "destination": destination,
            "payload": payload
        }),
    );
    (
        if caduceus.ok { StatusCode::OK } else { mutation_response_status(&caduceus) },
        Json(serde_json::json!({
            "schema": "coronatio.upload.submit.v1",
            "ok": caduceus.ok,
            "accepted": caduceus.ok,
            "filename": filename,
            "bytes": byte_count,
            "destination": destination,
            "authority": "Coronatio Rust upload route to Caduceus",
            "caduceus": caduceus,
            "firstMissingSignal": if caduceus.ok { "none".to_string() } else { caduceus.first_missing_signal }
        })),
    ).into_response()
}


include!("full-rust-routes/upload.rs");

fn upload_admin_read_refusal_response(path: &str) -> Response {
    (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"schema": "coronatio.upload.admin.read.refusal.v1", "success": false, "accepted": false, "family": "file-ingress", "error": "admin-session-required", "firstMissingSignal": "admin-session-required", "path": path}))).into_response()
}

async fn upload_history_admin_route(headers: axum::http::HeaderMap) -> Response {
    if session_from_headers(&headers) != Session::Admin { return upload_admin_read_refusal_response("/api/upload/history"); }
    upload_history_route().await.into_response()
}

async fn upload_history_route() -> impl IntoResponse {
    let readback = hyalos_tail_readback(Some("upload"), 100);
    let history = hyalos_upload_history_from_tail(&readback);
    let ok = readback.ok;
    (
        if ok { StatusCode::OK } else { StatusCode::SERVICE_UNAVAILABLE },
        Json(serde_json::json!({
            "schema": "coronatio.upload.history.v1",
            "ok": ok,
            "history": history,
            "authority": "Caduceus Hyalos tail membrane",
            "caduceus": readback,
            "firstMissingSignal": if ok { "none" } else { readback.first_missing_signal.as_str() }
        })),
    )
}

fn clear_upload_history_route() -> Response {
    hyalos_channel_clear_refusal_response("coronatio.upload.history.clear.v1", "/api/upload/history/clear")
}

async fn internet_status_route(headers: axum::http::HeaderMap) -> Response {
    let raw = internet_status_snapshot();
    match session_from_headers(&headers) {
        Session::Admin => Json(project_internet_status_admin(&raw)).into_response(),
        Session::Guest => Json(project_internet_status_guest(&raw)).into_response(),
    }
}

fn internet_status_snapshot() -> InternetStatusSnapshot {
    const INTERNET_STATUS_PROBE_HOSTS: [&str; 3] = ["1.1.1.1", "8.8.8.8", "208.67.222.222"];
    const INTERNET_STATUS_TIMEOUT_SECONDS: u64 = 3;
    let connected = INTERNET_STATUS_PROBE_HOSTS
        .iter()
        .map(|host| format!("{host}:53"))
        .any(|authority| {
            authority
                .parse::<SocketAddr>()
                .ok()
                .and_then(|addr| TcpStream::connect_timeout(&addr, Duration::from_secs(INTERNET_STATUS_TIMEOUT_SECONDS)).ok())
                .is_some()
        });
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs_f64())
        .unwrap_or(0.0);
    InternetStatusSnapshot {
        schema: "coronatio.internet.status.v1".to_string(),
        ok: true,
        success: true,
        status: if connected { "connected" } else { "disconnected" }.to_string(),
        timestamp,
        authority: "Coronatio Rust route port of Flask InternetStatusMonitor.check_connectivity".to_string(),
        hosts: INTERNET_STATUS_PROBE_HOSTS.iter().map(|host| (*host).to_string()).collect(),
        timeout_seconds: INTERNET_STATUS_TIMEOUT_SECONDS,
        first_missing_signal: "none".to_string(),
    }
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

async fn homeserver_rust_read_route(headers: axum::http::HeaderMap, method: Method, uri: Uri) -> impl IntoResponse { homeserver_read_response(&headers, method.as_str(), uri.path()) }

async fn homeserver_rust_mutation_route(headers: axum::http::HeaderMap, method: Method, uri: Uri) -> impl IntoResponse { homeserver_mutation_response(&headers, method.as_str(), uri.path()) }

async fn admin_class_generic_mutation_route(headers: axum::http::HeaderMap, method: Method, uri: Uri, body: Bytes) -> Response {
    if mutation_context_refusal(&headers).is_some() {
        return homeserver_mutation_response(&headers, method.as_str(), uri.path());
    }
    if uri.path() == "/api/upload/history/clear" { return clear_upload_history_route(); }
    if uri.path() == "/api/admin/logs/homeserver/clear" {
        return hyalos_channel_clear_refusal_response("coronatio.admin.logs.homeserver.clear.v1", uri.path());
    }
    let payload = serde_json::from_slice(&body).unwrap_or_else(|_| serde_json::json!({}));
    if uri.path() == "/api/upload/blacklist/update" { return upload_blacklist_update(&headers, payload); }
    if uri.path() == "/api/upload/pin-required-status" { return upload_pin_required_update(&headers, payload); }
    if uri.path() == "/api/upload/force-permissions" { return upload_force_permissions(&headers, payload); }
    homeserver_mutation_response(&headers, method.as_str(), uri.path())
}

async fn network_identity_mutation_route(headers: axum::http::HeaderMap, method: Method, uri: Uri) -> Response {
    homeserver_mutation_response(&headers, method.as_str(), uri.path())
}

fn admin_class_generic_mutation_refusal_response(method: &str, path: &str) -> Response {
    (StatusCode::UNAUTHORIZED, Json(serde_json::json!({
        "schema": "coronatio.admin-class-generic.mutation.refusal.v1",
        "success": false,
        "accepted": false,
        "method": method,
        "path": path,
        "family": admin_class_generic_refusal_family(path),
        "error": "admin-session-required",
        "firstMissingSignal": "admin-session-required"
    }))).into_response()
}

fn network_identity_mutation_refusal_response(method: &str, path: &str) -> Response {
    (StatusCode::UNAUTHORIZED, Json(serde_json::json!({
        "schema": "coronatio.network-identity.mutation.refusal.v1",
        "success": false,
        "accepted": false,
        "method": method,
        "path": path,
        "family": homeserver_route_family(path),
        "error": "admin-session-required",
        "firstMissingSignal": "admin-session-required"
    }))).into_response()
}

fn admin_class_generic_refusal_family(path: &str) -> &'static str {
    if path.contains("/backblazeTab") { "backblazeTab" }
    else if path.contains("/backup") { "backup" }
    else if path.contains("/miner") { "miner" }
    else if path.contains("/nasLinker") { "nasLinker" }
    else if path.contains("/vpn") { "vpn" }
    else if path.contains("/test") { "test" }
    else { homeserver_route_family(path) }
}

include!("full-rust-routes/read.rs");

include!("full-rust-routes/dhcp.rs");
include!("full-rust-routes/unbound.rs");
include!("full-rust-routes/firewall.rs");

include!("full-rust-routes/network-notes.rs");

include!("full-rust-routes/power.rs");

fn homeserver_mutation_response(headers: &axum::http::HeaderMap, method: &str, path: &str) -> Response {
    let caduceus = mutation_staff_intent(
        &mutation_authority(),
        &headers,
        method,
        path,
        homeserver_route_family(path),
        serde_json::json!({}),
    );
    (
        if caduceus.ok { StatusCode::ACCEPTED } else { mutation_response_status(&caduceus) },
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
    } else if path.contains("tailscale") || path.contains("vpn") || path.contains("network") || path.contains("dhcp") || path.contains("kea") || path.contains("wakeonlan") {
        "network-control"
    } else if path.contains("upload") || path.contains("files") || path.contains("nasLinker") {
        "file-ingress"
    } else if path.contains("portals") || path.contains("service/control") {
        "portal-service"
    } else if path.contains("backblazeTab") || path.contains("miner") {
        "crown-pane"
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
        ("/api/tabs/elements/:tab_id", &["get"]),
        ("/api/stats/elements", &["get"]),
        ("/api/portals/elements", &["get"]),
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
        ("/api/dns/records", &["get", "post"]),
        ("/api/dns/records/:name", &["delete"]),
        ("/api/firewall/status", &["get"]),
        ("/api/firewall/policies", &["get"]),
        ("/api/firewall/policies/:mac", &["get", "put", "delete"]),
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
