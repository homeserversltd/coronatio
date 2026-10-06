const NAS_SETUP_UPSTREAM_TIMEOUT: Duration = Duration::from_secs(600);

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
}

fn storage_disk_census_route_status(readback: &CaduceusHttpReadback) -> StatusCode {
    if readback.ok { StatusCode::OK } else { mutation_response_status(readback) }
}

async fn storage_disk_census_route(headers: axum::http::HeaderMap) -> Response {
    let path = "/api/v1/storage/disk/census";
    let readback = admin_fragment_caduceus_request(&headers, "GET", path);
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

fn storage_operation_response(readback: CaduceusHttpReadback, receipt_ok: bool, redact_secrets: bool) -> Response {
    let status = if !readback.ok {
        mutation_response_status(&readback)
    } else if receipt_ok {
        StatusCode::OK
    } else {
        StatusCode::BAD_GATEWAY
    };
    let body = if redact_secrets { redact_keyman_receipt(readback.body) } else { readback.body };
    (status, Json(body)).into_response()
}

async fn storage_vault_unlock_route(
    headers: axum::http::HeaderMap,
    Json(request): Json<StorageVaultUnlockRequest>,
) -> Response {
    if request.password.is_empty() {
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
    let readback = caduceus_actuate_json(
        &mutation_authority(),
        &headers,
        MutationActionTarget::caduceus("coronatio.storage.vault.unlock", path),
        path,
        serde_json::json!({ "password": request.password }),
    );
    let receipt_ok = readback.ok
        && storage_receipt_reports_success(&readback.body)
        && !storage_receipt_has_failure(&readback.body);
    storage_operation_response(readback, receipt_ok, true)
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

fn storage_nas_receipt_ok(body: &serde_json::Value) -> bool {
    body.get("schema").and_then(serde_json::Value::as_str) == Some("caduceus.nas.setup.v1")
        && storage_receipt_reports_success(body)
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
    mapping: MutationActionTarget,
    path: &str,
    body: serde_json::Value,
    timeout: Duration,
) -> CaduceusHttpReadback {
    let context = mapping.request_context(headers);
    match authority.authorize(&context, mapping) {
        Ok(attendance) => invalidate_scoped_attendance(
            authority,
            &attendance,
            caduceus_http_json_with_attendance_and_document_timeout(
                "POST",
                path,
                body,
                Some(&attendance.proof),
                Some(&attendance.document),
                timeout,
            ),
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

    let path = "/api/v1/storage/nas/setup";
    let body = serde_json::json!({ "device": device, "role": request.role });
    let result = tokio::task::spawn_blocking(move || {
        storage_actuate_json_timeout(
            &mutation_authority(),
            &headers,
            MutationActionTarget::caduceus("coronatio.storage.nas.setup", path),
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
