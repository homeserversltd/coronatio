const NAS_SETUP_UPSTREAM_TIMEOUT: Duration = Duration::from_secs(600);
const VAULT_UNLOCK_UPSTREAM_TIMEOUT: Duration = Duration::from_secs(600);
const DRIVE_STAFF_UPSTREAM_TIMEOUT: Duration = Duration::from_secs(600);

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct StorageDeviceRequest { device: String }

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct StorageDiskTestRequest { device: String, mode: String }

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct StorageRoleRequest { role: String }

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct StorageRebuildRequest {}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct StorageKeyProbeRequest { key_role: String, partition: String }

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct StorageKeyRekeyRequest {
    key_role: String,
    partition: String,
    #[serde(rename = "oldPassword")]
    old_password: String,
    #[serde(rename = "keepOld", default)]
    keep_old: bool,
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct StoragePartlabelRequest { current_label: String, label: String }

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct StoragePinVerifyRequest { pin: String }

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct StorageVaultUnlockRequest {
    password: String,
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct StorageNasSetupRequest {
    device: String,
    role: String,
    confirmation: String,
    #[serde(default)]
    flash: bool,
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct StorageNasPrefireRequest { device: String, role: String }

async fn storage_nas_prefire_route(Json(request): Json<StorageNasPrefireRequest>) -> Response {
    const PATH: &str = "/api/v1/storage/nas/prefire";
    let Some(device) = normalized_storage_device(&request.device) else { return storage_drive_refusal("drive-device-name-invalid"); };
    if !matches!(request.role.as_str(), "primary" | "backup") { return storage_drive_refusal("drive-role-invalid"); }
    let reply = tokio::task::spawn_blocking(move || caduceus_http_json_with_attendance_and_document_timeout(
        "POST", PATH, serde_json::json!({"device": device, "role": request.role}), None, None, Duration::from_secs(28)
    )).await.unwrap_or_else(|_| mutation_task_failure_readback(PATH));
    backups_pair_response(reply)
}

fn storage_disk_census_route_status(readback: &CaduceusHttpReadback) -> StatusCode {
    if readback.ok { StatusCode::OK } else { mutation_response_status(readback) }
}

fn storage_project_local_mutation_refusal(readback: &mut CaduceusHttpReadback) {
    let first_missing_signal = readback.first_missing_signal.clone();
    if !readback.ok
        && first_missing_signal != "caduceus-http-not-ok"
        && readback.body == serde_json::json!({ "error": "caduceus-mutation-refused" })
    {
        if let Some(body) = readback.body.as_object_mut() {
            body.insert(
                "firstMissingSignal".to_string(),
                serde_json::Value::String(first_missing_signal),
            );
        }
    }
}

async fn storage_disk_census_route(headers: axum::http::HeaderMap) -> Response {
    let path = "/api/v1/storage/disk/census";
    let authority = mutation_authority();
    let context = MutationRequestContext::attended_document_from_headers(&headers);
    let mut readback = match authority.authorize(&context) {
        Ok(attendance) => caduceus_http_with_attendance_and_document(
                "GET",
                path,
                Some(&attendance.proof),
                Some(&attendance.document),
            ),
        Err(refusal) => mutation_refusal_readback(path, refusal),
    };
    storage_project_local_mutation_refusal(&mut readback);
    (storage_disk_census_route_status(&readback), Json(readback.body)).into_response()
}

fn storage_receipt_has_failure(body: &serde_json::Value) -> bool {
    let explicit_false = ["ok", "success", "converged"]
        .iter()
        .any(|field| body.get(*field).and_then(serde_json::Value::as_bool) == Some(false));
    let failure_reported = body.get("failure").is_some_and(|failure| {
        !failure.is_null()
            && failure != &serde_json::Value::Bool(false)
            && failure.as_str().map_or(true, |value| !value.is_empty())
    });
    explicit_false || failure_reported || body.get("rolledBack").and_then(serde_json::Value::as_bool) == Some(true)
}

fn storage_receipt_reports_success(body: &serde_json::Value) -> bool {
    body.get("ok").and_then(serde_json::Value::as_bool) == Some(true)
        || body.get("success").and_then(serde_json::Value::as_bool) == Some(true)
}

fn storage_operation_response(mut readback: CaduceusHttpReadback, receipt_ok: bool, redact_secrets: bool) -> Response {
    let status = if !readback.ok {
        mutation_response_status(&readback)
    } else if receipt_ok {
        StatusCode::OK
    } else {
        StatusCode::BAD_GATEWAY
    };
    storage_project_local_mutation_refusal(&mut readback);
    let body = if redact_secrets { redact_keyman_receipt(readback.body) } else { readback.body };
    (status, Json(body)).into_response()
}

async fn storage_vault_unlock_route(
    headers: axum::http::HeaderMap,
    Json(request): Json<StorageVaultUnlockRequest>,
) -> Response {
    let result = tokio::task::spawn_blocking(move || {
        let StorageVaultUnlockRequest { password } = request;
        if password.is_empty() {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "schema": "coronatio.storage.vault.unlock.refusal.v1",
                    "ok": false,
                    "firstMissingSignal": "vault-password-required"
                })),
            )
                .into_response();
        }
        let path = "/api/v1/storage/vault/unlock";
        let mut readback = match face_stage() {
            FaceStage::Vault => caduceus_http_json_with_attendance_and_document_timeout(
                "POST",
                path,
                serde_json::json!({ "password": password }),
                None,
                None,
                VAULT_UNLOCK_UPSTREAM_TIMEOUT,
            ),
            FaceStage::Face => storage_actuate_json_timeout(
                &mutation_authority(),
                &headers,
                path,
                serde_json::json!({ "password": password }),
                VAULT_UNLOCK_UPSTREAM_TIMEOUT,
            ),
        };
        if !readback.ok
            && matches!(readback.first_missing_signal.as_str(), "caduceus-connect-timeout" | "caduceus-write-timeout" | "caduceus-read-timeout")
            && readback.body == serde_json::json!({ "error": "caduceus-upstream-failed" })
        {
            readback.body["firstMissingSignal"] = serde_json::Value::String(readback.first_missing_signal.clone());
        }
        let receipt_ok = readback.ok
            && storage_receipt_reports_success(&readback.body)
            && !storage_receipt_has_failure(&readback.body);
        storage_operation_response(readback, receipt_ok, true)
    })
    .await;
    result.unwrap_or_else(|_| {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({
                "schema": "coronatio.storage.vault.unlock.error.v1",
                "ok": false,
                "firstMissingSignal": "vault-unlock-worker-failed"
            })),
        )
            .into_response()
    })
}

fn normalized_storage_device(display_name: &str) -> Option<String> {
    let name = display_name.strip_prefix("/dev/").unwrap_or(display_name);
    if name.is_empty()
        || name == "."
        || name == ".."
        || name.contains('/')
        || !name.chars().all(|character| character.is_ascii_alphanumeric() || matches!(character, '_' | '-' | '.'))
    {
        return None;
    }
    Some(format!("/dev/{name}"))
}

const NAS_SETUP_PATH: &str = "/api/v1/storage/nas/setup";

fn storage_nas_receipt_ok(body: &serde_json::Value) -> bool {
    let projection = match body.get("schema").and_then(serde_json::Value::as_str) {
        Some("caduceus.chord.receipt.v1") => {
            if body.get("routine_id").and_then(serde_json::Value::as_str) != Some("storage/nas/setup")
                || body.get("ok").and_then(serde_json::Value::as_bool) != Some(true)
                || storage_receipt_has_failure(body)
                || match body.get("failedStage") {
                    None | Some(serde_json::Value::Null) => false,
                    Some(serde_json::Value::String(stage)) => !stage.is_empty(),
                    Some(_) => true,
                }
            {
                return false;
            }
            match body.get("face") {
                Some(face) if face.get("schema").and_then(serde_json::Value::as_str) == Some("caduceus.nas.setup.v1") => face,
                _ => return false,
            }
        }
        _ if caduceus_receipt_matches_path(body, NAS_SETUP_PATH) => body,
        _ => return false,
    };
    storage_nas_projection_ok(projection)
}

fn storage_nas_projection_ok(body: &serde_json::Value) -> bool {
    storage_receipt_reports_success(body)
        && !storage_receipt_has_failure(body)
        && !body
            .get("steps")
            .and_then(serde_json::Value::as_array)
            .is_some_and(|steps| {
                steps.iter().any(|step| step.get("ok").and_then(serde_json::Value::as_bool) == Some(false))
            })
}

fn storage_actuate_json_timeout(
    authority: &MutationAuthority,
    headers: &axum::http::HeaderMap,
    path: &str,
    body: serde_json::Value,
    timeout: Duration,
) -> CaduceusHttpReadback {
    let context = MutationRequestContext::from_headers(headers);
    match authority.authorize(&context) {
        Ok(attendance) => caduceus_http_json_with_attendance_and_document_timeout(
                "POST",
                path,
                body,
                Some(&attendance.proof),
                Some(&attendance.document),
                timeout,
            ),
        Err(refusal) => mutation_refusal_readback(path, refusal),
    }
}

async fn storage_nas_setup_route(
    headers: axum::http::HeaderMap,
    Json(request): Json<StorageNasSetupRequest>,
) -> Response {
    if request.confirmation != request.device {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "schema": "coronatio.storage.nas.setup.refusal.v1",
                "ok": false,
                "firstMissingSignal": "nas-device-confirmation-mismatch"
            })),
        )
            .into_response();
    }
    let Some(device) = normalized_storage_device(&request.device) else {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "schema": "coronatio.storage.nas.setup.refusal.v1",
                "ok": false,
                "firstMissingSignal": "nas-device-name-invalid"
            })),
        )
            .into_response();
    };
    if !matches!(request.role.as_str(), "primary" | "backup") {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "schema": "coronatio.storage.nas.setup.refusal.v1",
                "ok": false,
                "firstMissingSignal": "nas-role-invalid"
            })),
        )
            .into_response();
    }

    let path = NAS_SETUP_PATH;
    let body = serde_json::json!({ "device": device, "role": request.role, "flash": request.flash });
    let result = tokio::task::spawn_blocking(move || {
        storage_actuate_json_timeout(
            &mutation_authority(),
            &headers,
            path,
            body,
            NAS_SETUP_UPSTREAM_TIMEOUT,
        )
    })
    .await;

    match result {
        Ok(readback) => {
            let receipt_ok = readback.ok && storage_nas_receipt_ok(&readback.body);
            storage_operation_response(readback, receipt_ok, false)
        }
        Err(_) => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({
                "schema": "coronatio.storage.nas.setup.error.v1",
                "ok": false,
                "converged": false,
                "failure": "The storage setup request did not return a Caduceus receipt.",
                "firstMissingSignal": "nas-setup-worker-failed"
            })),
        )
            .into_response(),
    }
}

fn storage_drive_refusal(signal: &str) -> Response {
    (StatusCode::BAD_REQUEST, Json(serde_json::json!({
        "schema": "coronatio.storage.drive.refusal.v1",
        "ok": false,
        "firstMissingSignal": signal
    }))).into_response()
}

async fn storage_forward_staff_door(
    headers: axum::http::HeaderMap,
    path: &'static str,
    body: serde_json::Value,
    timeout: Duration,
) -> Response {
    let result = tokio::task::spawn_blocking(move || {
        storage_actuate_json_timeout(&mutation_authority(), &headers, path, body, timeout)
    }).await;
    match result {
        Ok(readback) if matches!(readback.status, 404 | 405) => (
            StatusCode::NOT_IMPLEMENTED,
            Json(serde_json::json!({
                "schema": "coronatio.storage.drive.unavailable.v1",
                "ok": false,
                "firstMissingSignal": "coronatio-caduceus-route-translation-required",
                "path": path
            })),
        ).into_response(),
        Ok(readback) => {
            let receipt_ok = readback.ok
                && readback.body.get("ok").and_then(serde_json::Value::as_bool) == Some(true)
                && matches!(readback.first_missing_signal.as_str(), "" | "none")
                && !storage_receipt_has_failure(&readback.body);
            storage_operation_response(readback, receipt_ok, false)
        }
        Err(_) => (StatusCode::SERVICE_UNAVAILABLE, Json(serde_json::json!({
            "schema": "coronatio.storage.drive.error.v1",
            "ok": false,
            "firstMissingSignal": "drive-worker-failed"
        }))).into_response(),
    }
}

fn storage_strip_envelopes(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::Object(map) => {
            map.remove("raw_envelope");
            map.remove("envelope");
            for child in map.values_mut() { storage_strip_envelopes(child); }
        }
        serde_json::Value::Array(items) => for child in items { storage_strip_envelopes(child); },
        _ => {}
    }
}

async fn storage_forward_secret_door(headers: axum::http::HeaderMap, path: &'static str, body: serde_json::Value) -> Response {
    let result = tokio::task::spawn_blocking(move || storage_actuate_json_timeout(&mutation_authority(), &headers, path, body, VAULT_UNLOCK_UPSTREAM_TIMEOUT)).await;
    match result {
        Ok(readback) if matches!(readback.status, 404 | 405) => (StatusCode::NOT_IMPLEMENTED, Json(serde_json::json!({"ok":false,"firstMissingSignal":"coronatio-caduceus-route-translation-required","path":path}))).into_response(),
        Ok(mut readback) => {
            let receipt_ok = readback.ok && readback.body.get("ok").and_then(serde_json::Value::as_bool) == Some(true)
                && matches!(readback.first_missing_signal.as_str(), "" | "none") && !storage_receipt_has_failure(&readback.body);
            storage_strip_envelopes(&mut readback.body);
            storage_operation_response(readback, receipt_ok, true)
        }
        Err(_) => (StatusCode::SERVICE_UNAVAILABLE, Json(serde_json::json!({"ok":false,"firstMissingSignal":"drive-worker-failed"}))).into_response(),
    }
}

async fn storage_key_probe_route(headers: axum::http::HeaderMap, request: Result<Json<StorageKeyProbeRequest>, axum::extract::rejection::JsonRejection>) -> Response {
    let Ok(Json(request)) = request else { return storage_drive_refusal("drive-request-invalid"); };
    storage_forward_staff_door(headers, "/api/v1/storage/nas/key/probe", serde_json::json!({"key_role":request.key_role,"partition":request.partition}), DRIVE_STAFF_UPSTREAM_TIMEOUT).await
}

async fn storage_key_rekey_route(headers: axum::http::HeaderMap, request: Result<Json<StorageKeyRekeyRequest>, axum::extract::rejection::JsonRejection>) -> Response {
    let Ok(Json(request)) = request else { return storage_drive_refusal("drive-request-invalid"); };
    if request.old_password.is_empty() { return storage_drive_refusal("nas-rekey-password-required"); }
    storage_forward_secret_door(headers, "/api/v1/storage/nas/key/rekey", serde_json::json!({"key_role":request.key_role,"partition":request.partition,"oldPassword":request.old_password,"keepOld":request.keep_old})).await
}

async fn storage_partlabel_assign_route(headers: axum::http::HeaderMap, request: Result<Json<StoragePartlabelRequest>, axum::extract::rejection::JsonRejection>) -> Response {
    let Ok(Json(request)) = request else { return storage_drive_refusal("drive-request-invalid"); };
    storage_forward_staff_door(headers, "/api/v1/storage/disk/partlabel/assign", serde_json::json!({"current_label":request.current_label,"label":request.label}), DRIVE_STAFF_UPSTREAM_TIMEOUT).await
}

async fn storage_forge_restore_route(headers: axum::http::HeaderMap, request: Result<Json<StorageRebuildRequest>, axum::extract::rejection::JsonRejection>) -> Response {
    let Ok(Json(_)) = request else { return storage_drive_refusal("drive-request-invalid"); };
    storage_forward_secret_door(headers, "/api/v1/storage/backup/forgejo/nas-restore", serde_json::json!({})).await
}

async fn storage_pin_verify_route(mut headers: axum::http::HeaderMap, request: Result<Json<StoragePinVerifyRequest>, axum::extract::rejection::JsonRejection>) -> Response {
    let Ok(Json(request)) = request else { return storage_drive_refusal("drive-request-invalid"); };
    headers.remove(axum::http::header::CONTENT_LENGTH);
    headers.insert(axum::http::header::CONTENT_TYPE, axum::http::HeaderValue::from_static("application/json"));
    let body = Body::from(serde_json::json!({"pin": request.pin}).to_string());
    let unavailable = |signal: &str| (StatusCode::SERVICE_UNAVAILABLE, Json(serde_json::json!({
        "verified": false, "unavailable": true, "firstMissingSignal": signal
    }))).into_response();
    match tokio::time::timeout(VAULT_UNLOCK_UPSTREAM_TIMEOUT, async {
        match relay_upload_body("POST", "/api/v1/exousia/verify", &headers, &(), body).await {
            Ok(response) => {
                let (parts, body) = response.into_parts();
                let (body, captured) = capture_upload_body(body, UPLOAD_RESPONSE_BODY_CAPTURE_LIMIT).await;
                // Oversized replies remain streamed, not rejected or buffered without bound.
                if let Some(bytes) = captured {
                    let receipt = serde_json::from_slice::<serde_json::Value>(&bytes).ok();
                    if receipt.as_ref().and_then(|value| value.get("schema")).and_then(serde_json::Value::as_str)
                        != Some("caduceus.staff.receipt.v1") {
                        return unavailable("caduceus-verify-receipt-unavailable");
                    }
                }
                axum::http::Response::from_parts(parts, body)
            }
            Err(_) => unavailable("caduceus-verify-unavailable"),
        }
    }).await {
        Ok(response) => response,
        Err(_) => unavailable("caduceus-verify-unavailable"),
    }
}

async fn storage_drive_add_route(headers: axum::http::HeaderMap, request: Result<Json<StorageDeviceRequest>, axum::extract::rejection::JsonRejection>) -> Response {
    let Ok(Json(request)) = request else { return storage_drive_refusal("drive-request-invalid"); };
    let Some(device) = normalized_storage_device(&request.device) else { return storage_drive_refusal("drive-device-name-invalid"); };
    storage_forward_staff_door(headers, "/api/v1/storage/drive/add", serde_json::json!({"device": device}), DRIVE_STAFF_UPSTREAM_TIMEOUT).await
}

async fn storage_drive_remove_route(headers: axum::http::HeaderMap, request: Result<Json<StorageDeviceRequest>, axum::extract::rejection::JsonRejection>) -> Response {
    let Ok(Json(request)) = request else { return storage_drive_refusal("drive-request-invalid"); };
    let Some(device) = normalized_storage_device(&request.device) else { return storage_drive_refusal("drive-device-name-invalid"); };
    storage_forward_staff_door(headers, "/api/v1/storage/drive/remove", serde_json::json!({"device": device}), DRIVE_STAFF_UPSTREAM_TIMEOUT).await
}

async fn storage_nas_import_run_route(headers: axum::http::HeaderMap, request: Result<Json<StorageDeviceRequest>, axum::extract::rejection::JsonRejection>) -> Response {
    let Ok(Json(request)) = request else { return storage_drive_refusal("drive-request-invalid"); };
    let Some(device) = normalized_storage_device(&request.device) else { return storage_drive_refusal("drive-device-name-invalid"); };
    storage_forward_staff_door(headers, "/api/v1/storage/nas-import/run", serde_json::json!({"device": device}), DRIVE_STAFF_UPSTREAM_TIMEOUT).await
}

async fn storage_disk_test_route(headers: axum::http::HeaderMap, request: Result<Json<StorageDiskTestRequest>, axum::extract::rejection::JsonRejection>) -> Response {
    let Ok(Json(request)) = request else { return storage_drive_refusal("drive-request-invalid"); };
    let Some(device) = normalized_storage_device(&request.device) else { return storage_drive_refusal("drive-device-name-invalid"); };
    if !matches!(request.mode.as_str(), "quick" | "full") { return storage_drive_refusal("drive-mode-invalid"); }
    storage_forward_staff_door(headers, "/api/v1/storage/disk/test", serde_json::json!({"device": device, "mode": request.mode}), DRIVE_STAFF_UPSTREAM_TIMEOUT).await
}

async fn storage_shares_converge_route(headers: axum::http::HeaderMap, request: Result<Json<StorageRebuildRequest>, axum::extract::rejection::JsonRejection>) -> Response {
    let Ok(Json(_request)) = request else { return storage_drive_refusal("drive-request-invalid"); };
    storage_forward_staff_door(headers, "/api/v1/storage/shares/converge", serde_json::json!({}), DRIVE_STAFF_UPSTREAM_TIMEOUT).await
}

async fn storage_nas_attach_route(headers: axum::http::HeaderMap, request: Result<Json<StorageRoleRequest>, axum::extract::rejection::JsonRejection>) -> Response {
    let Ok(Json(request)) = request else { return storage_drive_refusal("drive-request-invalid"); };
    if !matches!(request.role.as_str(), "primary" | "backup") { return storage_drive_refusal("drive-role-invalid"); }
    storage_forward_staff_door(headers, "/api/v1/storage/nas/attach", serde_json::json!({"role": request.role}), DRIVE_STAFF_UPSTREAM_TIMEOUT).await
}

async fn storage_nas_detach_route(headers: axum::http::HeaderMap, request: Result<Json<StorageRoleRequest>, axum::extract::rejection::JsonRejection>) -> Response {
    let Ok(Json(request)) = request else { return storage_drive_refusal("drive-request-invalid"); };
    if !matches!(request.role.as_str(), "primary" | "backup") { return storage_drive_refusal("drive-role-invalid"); }
    storage_forward_staff_door(headers, "/api/v1/storage/nas/detach", serde_json::json!({"role": request.role}), DRIVE_STAFF_UPSTREAM_TIMEOUT).await
}
