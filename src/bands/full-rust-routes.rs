fn full_rust_route_table() -> Router<AppState> {
    Router::new()
        .route("/api/tabs/visibility", post(tab_visibility_route))
        .route("/api/tabs/elements", put(element_visibility_route))
        .route("/api/tabs/elements/:tab_id", get(element_fragment_route))
        .route("/api/stats/elements", get(stats_elements_fragment_route))
        .route("/api/portals/elements", get(portals_elements_fragment_route))
        .route("/api/vault/status", get(homeserver_rust_read_route))
        .route("/api/v1/storage/disk/census", get(storage_disk_census_route))
        .route("/api/v1/storage/vault/unlock", post(storage_vault_unlock_route))
        .route("/api/v1/storage/nas/setup", post(storage_nas_setup_route))
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

        .route("/api/status/services", get(indicator_legacy_read_route))
        .route("/api/status", get(internet_status_route))
        .route("/api/uptime", get(uptime_route))
        .route("/api/status/tailscale", get(indicator_legacy_read_route))
        .route("/api/status/tailscale/connect", post(tailscale_mutation_route))
        .route("/api/status/tailscale/authkey", post(tailscale_mutation_route))
        .route("/api/status/tailscale/disconnect", post(tailscale_mutation_route))
        .route("/api/status/tailscale/enable", post(tailscale_mutation_route))
        .route("/api/status/tailscale/disable", post(tailscale_mutation_route))
        .route("/api/status/tailscale/config", get(tailscale_config_route).post(tailscale_mutation_route))
        .route("/api/status/tailscale/update-tailnet", post(tailscale_mutation_route))
        .route("/api/status/vpn/pia", get(indicator_legacy_read_route))
        .route("/api/status/vpn/transmission", get(indicator_legacy_read_route))
        .route("/api/transmission/keys", get(transmission_keys_route))
        .route("/api/transmission/keys/replace", post(transmission_keys_replace_route))
        .route("/api/transmission/keys/rotate", post(transmission_keys_rotate_route))
        .route("/api/v1/transmission/vpn/settings", get(transmission_vpn_settings_get_route).post(transmission_vpn_settings_post_route))
        .route("/api/files/browse", get(upload_browse_hierarchical_route))
        .route("/api/files/browse-hierarchical", get(upload_browse_hierarchical_route))
        .route("/api/files/upload", post(upload_file_route).layer(axum::extract::DefaultBodyLimit::disable()))
        .route("/api/files/upload/start", post(upload_start_route))
        .route(
            "/api/files/upload/:upload_id/chunk/:index",
            post(upload_chunk_route)
                .layer(tower_http::limit::RequestBodyLimitLayer::new(UPLOAD_CHUNK_BODY_LIMIT)),
        )
        .route("/api/files/upload/:upload_id/complete", post(upload_complete_route))
        .route("/api/files/upload/:upload_id", delete(upload_delete_route))
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
        .route("/api/status/internet/speedtest", post(internet_speedtest_route))
        .route("/api/status/power/usage", get(power_usage_route))
        .route("/status/power/usage", get(power_usage_route))
        .route("/api/status/power/calibration", get(power_calibration_get_route).post(power_calibration_post_route))
        .route("/api/kea-leases", get(homeserver_rust_read_route))
        .route("/api/network/notes", get(network_notes_read_route).put(network_notes_write_route))
        .route("/api/version", get(homeserver_rust_read_route))
        .route("/api/wakeonlan/devices", get(wake_on_lan_devices_route))
        .route("/api/wakeonlan/wake", post(wake_on_lan_wake_route))
        .route("/api/wakeonlan/status", get(wake_on_lan_status_route))
        .route("/api/network/device", get(device_roster_route))
        .route("/api/network/device/claim", post(device_claim_route))
        .route("/api/network/dhcp/boundary", get(device_boundary_route))
        .route("/api/network/dhcp/leases", get(device_leases_route))
        .route("/api/network/dhcp/reservations", get(device_reservations_route))
        .route("/api/network/dns/read", get(device_dns_read_route))
        .route("/api/dhcp/status", get(dhcp_read_route))
        .route("/api/dhcp/leases", get(dhcp_read_route))
        .route("/api/dhcp/reservations", get(dhcp_read_route).post(dhcp_reservation_create_route))
        .route("/api/dhcp/reservations/:reservation_id", put(dhcp_reservation_update_route).delete(dhcp_reservation_delete_route))
        .route("/api/dhcp/config", get(dhcp_read_route).post(network_identity_mutation_route))
        .route("/api/dhcp/health", get(dhcp_read_route))
        .route("/api/dhcp/statistics", get(dhcp_read_route))
        .route("/api/dhcp/pool-boundary", get(dhcp_read_route).post(dhcp_pool_boundary_route))
        .route("/api/dns/records", get(dns_records_get_route).post(dns_records_post_route))
        .route("/api/dns/records/status", post(dns_records_status_post_route))
        .route("/api/dns/records/:name", delete(dns_records_delete_route))
        .route("/api/v1/network/dns/read", get(dns_caduceus_read_route))
        .route("/api/v1/network/dns/resolver/status", get(dns_caduceus_read_route))
        .route("/api/v1/network/dns/device-name/create", post(dns_caduceus_mutation_route))
        .route("/api/v1/network/dns/device-name/remove", post(dns_caduceus_mutation_route))
        .route("/api/v1/network/dns/blocklist/update", post(dns_caduceus_mutation_route))
        .route("/api/v1/network/dns/adblock", post(dns_caduceus_mutation_route))
        .route("/api/v1/network/dns/upstream", post(dns_caduceus_mutation_route))
        .route("/api/firewall/observed", get(firewall_observed_route))
        .route("/api/firewall/children", get(firewall_children_route).post(firewall_register_route))
        .route("/api/firewall/children/:mac", delete(firewall_unregister_route))
        .route("/api/firewall/children/:mac/whitelist", get(firewall_whitelist_get_route).put(firewall_whitelist_set_route))
        .route("/api/backblaze/status", get(backblaze_status_route))
        .route("/api/backblaze/snapshots", get(backblaze_snapshots_route))
        .route("/api/backblaze/run", post(backblaze_run_route))
        .route("/api/backblaze/config", get(backblaze_config_get_route).post(backblaze_config_post_route))
        .route("/api/backblaze/buckets", get(backblaze_buckets_get_route).post(backblaze_bucket_post_route))
        .route("/api/backblaze/buckets/:bucket", delete(backblaze_bucket_delete_route))
        .route("/api/backblaze/buckets/:bucket/verify", post(backblaze_bucket_verify_route))
        .route("/api/backblaze/buckets/:bucket/items", get(backblaze_items_get_route).post(backblaze_item_post_route).delete(backblaze_item_delete_route))

        .route("/api/backblaze/buckets/:bucket/toggle", post(backblaze_toggle_route))
        .route("/api/backblaze/buckets/:bucket/run", post(backblaze_run_bucket_route))
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


const MULTIPART_UPLOAD_CHUNK_SIZE: u64 = 4 * 1024 * 1024;
const MULTIPART_UPLOAD_FRAME_SIZE: usize = 64 * 1024;

struct TransientUploadSpool {
    path: PathBuf,
}

impl Drop for TransientUploadSpool {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}

fn create_transient_upload_spool() -> Result<(TransientUploadSpool, tokio::fs::File), String> {
    use std::os::unix::fs::OpenOptionsExt as _;
    let path = env::temp_dir().join(format!(
        "coronatio-upload-{}-{}",
        std::process::id(),
        uuid::Uuid::new_v4()
    ));
    let mut options = std::fs::OpenOptions::new();
    options.write(true).create_new(true).mode(0o600);
    let file = options
        .open(&path)
        .map_err(|_| "upload-spool-create-failed".to_string())?;
    Ok((TransientUploadSpool { path }, tokio::fs::File::from_std(file)))
}

async fn multipart_upload_body_range(
    path: &FsPath,
    offset: u64,
    length: u64,
) -> Result<Body, String> {
    let mut file = tokio::fs::File::open(path)
        .await
        .map_err(|_| "upload-spool-open-failed".to_string())?;
    tokio::io::AsyncSeekExt::seek(&mut file, std::io::SeekFrom::Start(offset))
        .await
        .map_err(|_| "upload-spool-seek-failed".to_string())?;
    let stream = futures_util::stream::try_unfold((file, length), |(mut file, remaining)| async move {
        if remaining == 0 {
            return Ok(None);
        }
        let mut buffer = vec![0u8; remaining.min(MULTIPART_UPLOAD_FRAME_SIZE as u64) as usize];
        let read = tokio::io::AsyncReadExt::read(&mut file, &mut buffer).await?;
        if read == 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "upload spool ended before the declared chunk range",
            ));
        }
        buffer.truncate(read);
        Ok(Some((Bytes::from(buffer), (file, remaining - read as u64))))
    });
    Ok(Body::from_stream(stream))
}

fn upload_forward_headers(
    source: &axum::http::HeaderMap,
    content_type: Option<&'static str>,
) -> axum::http::HeaderMap {
    let mut headers = source.clone();
    headers.remove(axum::http::header::CONTENT_LENGTH);
    headers.remove(axum::http::header::CONTENT_TYPE);
    if let Some(content_type) = content_type {
        headers.insert(
            axum::http::header::CONTENT_TYPE,
            axum::http::HeaderValue::from_static(content_type),
        );
    }
    headers
}

fn upload_submit_response(
    status: StatusCode,
    ok: bool,
    filename: &str,
    byte_count: u64,
    destination: &str,
    caduceus: serde_json::Value,
) -> Response {
    let status = if !ok && status.is_success() {
        StatusCode::SERVICE_UNAVAILABLE
    } else {
        status
    };
    let signal = if ok {
        "none".to_string()
    } else {
        upload_refusal_signal(Some(&caduceus))
    };
    (
        status,
        Json(serde_json::json!({
            "schema": "coronatio.upload.submit.v1",
            "ok": ok,
            "accepted": ok,
            "filename": filename,
            "bytes": byte_count,
            "destination": destination,
            "authority": "Coronatio Rust upload route to Caduceus",
            "caduceus": caduceus,
            "firstMissingSignal": signal
        })),
    )
        .into_response()
}

async fn upload_file_route(headers: axum::http::HeaderMap, mut multipart: Multipart) -> Response {
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
    let mut spool: Option<TransientUploadSpool> = None;
    let mut byte_count = 0u64;
    let mut payload_sha256: Option<String> = None;

    loop {
        let mut field = match multipart.next_field().await {
            Ok(Some(field)) => field,
            Ok(None) => break,
            Err(error) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(serde_json::json!({
                        "schema": "coronatio.upload.error.v1",
                        "ok": false,
                        "error": error.to_string(),
                        "firstMissingSignal": "upload-form-read-failed"
                    })),
                )
                    .into_response();
            }
        };
        let name = field.name().unwrap_or("").to_string();
        if name == "path" || name == "destination" {
            if let Ok(text) = field.text().await {
                if !text.trim().is_empty() {
                    destination = text.trim().to_string();
                }
            }
            continue;
        }
        if name == "file" {
            if let Some(raw_name) = field.file_name() {
                filename = raw_name.to_string();
            }
            let (next_spool, mut output) = match create_transient_upload_spool() {
                Ok(spool) => spool,
                Err(signal) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({
                            "schema": "coronatio.upload.error.v1",
                            "ok": false,
                            "firstMissingSignal": signal
                        })),
                    )
                        .into_response();
                }
            };
            use sha2::Digest as _;
            let mut hasher = sha2::Sha256::new();
            let mut next_byte_count = 0u64;
            loop {
                let chunk = match field.chunk().await {
                    Ok(Some(chunk)) => chunk,
                    Ok(None) => break,
                    Err(error) => {
                        return (
                            StatusCode::BAD_REQUEST,
                            Json(serde_json::json!({
                                "schema": "coronatio.upload.error.v1",
                                "ok": false,
                                "error": error.to_string(),
                                "firstMissingSignal": "upload-form-read-failed"
                            })),
                        )
                            .into_response();
                    }
                };
                if tokio::io::AsyncWriteExt::write_all(&mut output, &chunk).await.is_err() {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({
                            "schema": "coronatio.upload.error.v1",
                            "ok": false,
                            "firstMissingSignal": "upload-spool-write-failed"
                        })),
                    )
                        .into_response();
                }
                let Some(count) = next_byte_count.checked_add(chunk.len() as u64) else {
                    return (
                        StatusCode::BAD_REQUEST,
                        Json(serde_json::json!({
                            "schema": "coronatio.upload.error.v1",
                            "ok": false,
                            "firstMissingSignal": "upload-size-overflow"
                        })),
                    )
                        .into_response();
                };
                next_byte_count = count;
                hasher.update(&chunk);
            }
            if tokio::io::AsyncWriteExt::flush(&mut output).await.is_err() {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({
                        "schema": "coronatio.upload.error.v1",
                        "ok": false,
                        "firstMissingSignal": "upload-spool-flush-failed"
                    })),
                )
                    .into_response();
            }
            drop(output);
            spool = Some(next_spool);
            byte_count = next_byte_count;
            payload_sha256 = Some(hasher.finalize().iter().map(|byte| format!("{byte:02x}")).collect());
        }
    }

    if spool.is_none() || byte_count == 0 {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "schema": "coronatio.upload.error.v1",
                "ok": false,
                "error": "file field is empty or missing",
                "firstMissingSignal": "upload-file-missing"
            })),
        )
            .into_response();
    }

    let root = upload_root_path();
    let display_root = upload_display_root(&root);
    let filename_path = FsPath::new(&filename);
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

    let start_body = serde_json::json!({
        "filename": filename,
        "total_size": byte_count,
        "target_dir": destination,
        "chunk_size": MULTIPART_UPLOAD_CHUNK_SIZE
    });
    let start_headers = upload_forward_headers(&headers, Some("application/json"));
    let start_response = upload_start_route(start_headers, Body::from(start_body.to_string())).await;
    let (start_response, start_readback) = capture_upload_response(start_response).await;
    let start_status = start_response.status();
    let Some(start_readback) = start_readback else {
        return upload_submit_response(
            start_status,
            false,
            &filename,
            byte_count,
            &destination,
            serde_json::json!({"ok":false,"firstMissingSignal":"upload-start-response-invalid"}),
        );
    };
    if !start_status.is_success() {
        return upload_submit_response(
            start_status,
            false,
            &filename,
            byte_count,
            &destination,
            start_readback,
        );
    }
    let upload_id = start_readback
        .get("upload_id")
        .or_else(|| start_readback.get("uploadId"))
        .or_else(|| start_readback.get("id"))
        .and_then(serde_json::Value::as_str)
        .map(str::to_string);
    let Some(upload_id) = upload_id else {
        return upload_submit_response(
            StatusCode::BAD_GATEWAY,
            false,
            &filename,
            byte_count,
            &destination,
            serde_json::json!({"ok":false,"firstMissingSignal":"upload-start-id-missing"}),
        );
    };
    if let Some(sha256) = payload_sha256 {
        set_upload_projection_expected_sha256(&upload_id, sha256);
    }

    let spool = spool.expect("a non-empty file field created a transient spool");
    let mut offset = 0u64;
    let mut index = 0u64;
    while offset < byte_count {
        let chunk_bytes = (byte_count - offset).min(MULTIPART_UPLOAD_CHUNK_SIZE);
        let body = match multipart_upload_body_range(&spool.path, offset, chunk_bytes).await {
            Ok(body) => body,
            Err(signal) => {
                let abort_headers = upload_forward_headers(&headers, None);
                let _ = upload_delete_route(
                    Path(upload_id.clone()),
                    abort_headers,
                    Body::empty(),
                )
                .await;
                return upload_submit_response(
                    StatusCode::SERVICE_UNAVAILABLE,
                    false,
                    &filename,
                    byte_count,
                    &destination,
                    serde_json::json!({"ok":false,"firstMissingSignal":signal}),
                );
            }
        };
        let chunk_headers = upload_forward_headers(&headers, Some("application/octet-stream"));
        let chunk_response = upload_chunk_route(
            Path((upload_id.clone(), index.to_string())),
            chunk_headers,
            body,
        )
        .await;
        let (chunk_response, chunk_readback) = capture_upload_response(chunk_response).await;
        if !chunk_response.status().is_success() {
            let abort_headers = upload_forward_headers(&headers, None);
            let _ = upload_delete_route(
                Path(upload_id.clone()),
                abort_headers,
                Body::empty(),
            )
            .await;
            return upload_submit_response(
                chunk_response.status(),
                false,
                &filename,
                byte_count,
                &destination,
                chunk_readback.unwrap_or_else(|| serde_json::json!({
                    "ok": false,
                    "firstMissingSignal": "upload-chunk-response-invalid"
                })),
            );
        }
        offset += chunk_bytes;
        index += 1;
    }

    let complete_headers = upload_forward_headers(&headers, None);
    let complete_response = upload_complete_route(
        Path(upload_id),
        complete_headers,
        Body::empty(),
    )
    .await;
    let (complete_response, complete_readback) = capture_upload_response(complete_response).await;
    let complete_status = complete_response.status();
    let complete_readback = complete_readback.unwrap_or_else(|| serde_json::json!({
        "ok": false,
        "firstMissingSignal": "upload-complete-response-invalid"
    }));
    let ok = complete_status.is_success()
        && complete_readback.get("ok").and_then(serde_json::Value::as_bool) == Some(true);
    upload_submit_response(
        complete_status,
        ok,
        &filename,
        byte_count,
        &destination,
        complete_readback,
    )
}


include!("full-rust-routes/upload.rs");

fn upload_admin_read_refusal_response(path: &str) -> Response {
    (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"schema": "coronatio.upload.admin.read.refusal.v1", "success": false, "accepted": false, "family": "file-ingress", "error": "admin-session-required", "firstMissingSignal": "admin-session-required", "path": path}))).into_response()
}

async fn upload_history_admin_route(headers: axum::http::HeaderMap) -> Response {
    if session_projection_from_headers(&headers).await != Session::Admin { return upload_admin_read_refusal_response("/api/upload/history"); }
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

async fn homeserver_rust_read_route(headers: axum::http::HeaderMap, method: Method, uri: Uri) -> impl IntoResponse { homeserver_read_response(&headers, method.as_str(), uri.path()).await }

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

async fn internet_speedtest_route(headers: axum::http::HeaderMap) -> Response {
    let path = "/api/status/internet/speedtest";
    if let Some(refusal) = device_identity_admin(&headers, path) {
        return refusal;
    }
    // Caduceus mounts the speed test at GET /api/v1/network/bandwidth/measure
    // (leaf network/bandwidth/measure) and answers caduceus.network.speedtest.v1:
    // ok, download and upload in Mbps, latency in ms, mutationPerformed false,
    // firstMissingSignal. The face reads exactly download, upload, latency and
    // firstMissingSignal, so the receipt passes through unchanged. Coronatio's
    // own door stays POST behind the admin session; the downstream verb is
    // the actuator's, not a reason to relax the caller's gate.
    let readback = caduceus_http_json(
        "GET",
        "/api/v1/network/bandwidth/measure",
        serde_json::json!({}),
        None,
    );
    if readback.ok {
        return (StatusCode::OK, Json(readback.body)).into_response();
    }
    (mutation_response_status(&readback), Json(readback.body)).into_response()
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
    if path.contains("/backup") { "backup" }
    else if path.contains("/miner") { "miner" }
    else if path.contains("/nasLinker") { "nasLinker" }
    else if path.contains("/vpn") { "vpn" }
    else if path.contains("/test") { "test" }
    else { homeserver_route_family(path) }
}

include!("full-rust-routes/read.rs");

include!("full-rust-routes/dhcp.rs");
include!("full-rust-routes/device-identity.rs");
include!("full-rust-routes/unbound.rs");
include!("full-rust-routes/firewall.rs");
include!("full-rust-routes/wake-on-lan.rs");
include!("full-rust-routes/linker.rs");
include!("full-rust-routes/backblaze.rs");
include!("full-rust-routes/storage.rs");

include!("full-rust-routes/network-notes.rs");

include!("full-rust-routes/power.rs");

fn homeserver_mutation_response(headers: &axum::http::HeaderMap, method: &str, path: &str) -> Response {
    let caduceus = route_translation_debt(
        &mutation_authority(),
        headers,
        method,
        path,
        None,
        None,
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
            "authority": "Caduceus staff transition membrane",
            "caduceus": caduceus,
            "firstMissingSignal": if caduceus.ok { "none".to_string() } else { caduceus.first_missing_signal }
        })),
    )
        .into_response()
}


include!("full-rust-routes/portals.rs");

include!("full-rust-routes/transmission.rs");

fn homeserver_route_family(path: &str) -> &'static str {
    if path.contains("/storage/") || path.contains("/vault") || path.contains("/crypto") || path.contains("/keyman") {
        "admin-storage"
    } else if path.contains("/updates") || path.contains("/system/update") || path.contains("/backup") {
        "update-and-backup"
    } else if path.contains("tailscale") || path.contains("vpn") || path.contains("network") || path.contains("dhcp") || path.contains("kea") || path.contains("wakeonlan") {
        "network-control"
    } else if path.contains("upload") || path.contains("files") || path.contains("nasLinker") {
        "file-ingress"
    } else if path.contains("portals") || path.contains("service/control") {
        "portal-service"
    } else if path.contains("miner") {
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
        ("/api/vault/status", &["get"]),
        ("/api/v1/storage/disk/census", &["get"]),
        ("/api/v1/storage/vault/unlock", &["post"]),
        ("/api/v1/storage/nas/setup", &["post"]),
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
        ("/api/transmission/keys", &["get"]),
        ("/api/transmission/keys/replace", &["post"]),
        ("/api/transmission/keys/rotate", &["post"]),
        ("/api/v1/transmission/vpn/settings", &["get", "post"]),
        ("/api/files/browse", &["get"]),
        ("/api/files/browse-hierarchical", &["get"]),
        ("/api/files/upload", &["post"]),
        ("/api/files/upload/start", &["post"]),
        ("/api/files/upload/:upload_id/chunk/:index", &["post"]),
        ("/api/files/upload/:upload_id/complete", &["post"]),
        ("/api/files/upload/:upload_id", &["delete"]),
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
        ("/api/status/power/calibration", &["get", "post"]),
        ("/api/kea-leases", &["get"]),
        ("/api/network/notes", &["get", "put"]),
        ("/api/version", &["get"]),
        ("/api/wakeonlan/devices", &["get"]),
        ("/api/wakeonlan/wake", &["post"]),
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
        ("/api/dns/records/status", &["post"]),
        ("/api/dns/records/:name", &["delete"]),
        ("/api/firewall/observed", &["get"]),
        ("/api/firewall/children", &["get", "post"]),
        ("/api/firewall/children/:mac", &["delete"]),
        ("/api/firewall/children/:mac/whitelist", &["get", "put"]),
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
