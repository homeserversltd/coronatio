// Single successor authority and actuator seam for all Coronatio state changes.
// The current document incarnation and its opaque attendance proof are carried only by that document.
use crate::caduceus_access::{attendance_from_headers, document_incarnation_from_headers, safe_access_code, AttendanceProof, CaduceusAccessClient};

#[derive(Clone, Debug, PartialEq, Eq)]
struct MutationRequestContext {
    document: Option<String>,
    attendance: Option<AttendanceProof>,
}

impl MutationRequestContext {
    fn from_headers(headers: &axum::http::HeaderMap) -> Self {
        Self::attended_document_from_headers(headers)
    }

    fn attended_document_from_headers(headers: &axum::http::HeaderMap) -> Self {
        Self { document: document_incarnation_from_headers(headers), attendance: attendance_from_headers(headers) }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct MutationAttendance { proof: AttendanceProof, document: String }


#[derive(Clone, Debug)]
struct MutationRefusal {
    code: String,
    status: u16,
}

#[derive(Clone)]
struct MutationAuthority { access: CaduceusAccessClient }

impl Default for MutationAuthority {
    fn default() -> Self { Self { access: CaduceusAccessClient::default() } }
}

impl MutationAuthority {
    fn authorize(
        &self,
        context: &MutationRequestContext,
    ) -> Result<MutationAttendance, MutationRefusal> {
        let Some(document) = context.document.as_ref() else {
            return Err(MutationRefusal { code: "caduceus-attendance-document-required".to_string(), status: 400 });
        };
        let Some(attendance) = context.attendance.as_ref() else {
            return Err(MutationRefusal { code: "caduceus-attendance-required".to_string(), status: 401 });
        };
        let call = self.access.attendance_validate(attendance, document);
        if !call.receipt.ok {
            crate::caduceus_access::bust_attendance_projection(attendance, document, "validation-refused");
            return Err(MutationRefusal { code: call.receipt.code, status: call.receipt.status });
        }
        Ok(MutationAttendance { proof: attendance.clone(), document: document.clone() })
    }
}

fn mutation_authority() -> MutationAuthority { MutationAuthority::default() }

fn mutation_context_refusal(headers: &axum::http::HeaderMap) -> Option<MutationRefusal> {
    let context = MutationRequestContext::from_headers(headers);
    if context.document.is_none() {
        Some(MutationRefusal { code: "caduceus-attendance-document-required".to_string(), status: 400 })
    } else if context.attendance.is_none() {
        Some(MutationRefusal { code: "caduceus-attendance-required".to_string(), status: 401 })
    } else {
        None
    }
}

fn admin_fragment_context_refusal(headers: &axum::http::HeaderMap) -> Option<MutationRefusal> {
    let context = MutationRequestContext::from_headers(headers);
    if context.document.is_none() {
        Some(MutationRefusal { code: "caduceus-attendance-document-required".to_string(), status: 400 })
    } else if context.attendance.is_none() {
        Some(MutationRefusal { code: "caduceus-attendance-required".to_string(), status: 401 })
    } else {
        None
    }
}

fn mutation_refusal_readback(path: &str, refusal: MutationRefusal) -> CaduceusHttpReadback {
    CaduceusHttpReadback {
        ok: false,
        status: refusal.status,
        path: path.to_string(),
        body: serde_json::json!({"error": "caduceus-mutation-refused"}),
        first_missing_signal: safe_access_code(&refusal.code),
    }
}

fn mutation_response_status(readback: &CaduceusHttpReadback) -> axum::http::StatusCode {
    if readback.ok {
        return axum::http::StatusCode::OK;
    }
    let signal = readback.first_missing_signal.as_str();
    if matches!(
        signal,
        "caduceus-attendance-wrong-document"
            | "caduceus-attendance-document-wrong"
            | "caduceus-attendance-document-mismatch"
            | "caduceus-attendance-document-incarnation-mismatch"
    ) {
        axum::http::StatusCode::BAD_REQUEST
    } else if matches!(
        signal,
        "caduceus-access-refused"
            | "caduceus-attendance-refused"
            | "caduceus-attendance-pin-refused"
            | "caduceus-attendance-not-current"
            | "caduceus-attendance-invalid"
            | "caduceus-attendance-required"
            | "caduceus-stale-incarnation"
            | "caduceus-attendance-stale-incarnation"
            | "caduceus-attendance-pin-wrong"
            | "exousia-pin-required"
            | "exousia-pin-wrong"
            | "caduceus-attendance-stale-document"
            | "caduceus-attendance-incarnation-stale"
    ) {
        axum::http::StatusCode::UNAUTHORIZED
    } else if matches!(
        signal,
        "caduceus-attendance-unbound"
            | "caduceus-attendance-verifier-unbound"
            | "caduceus-signer-stale-derived"
            | "caduceus-signer-current-bind-unavailable"
            | "caduceus-signer-verification-unavailable"
    ) {
        axum::http::StatusCode::SERVICE_UNAVAILABLE
    } else {
        axum::http::StatusCode::SERVICE_UNAVAILABLE
    }
}

fn caduceus_actuate_json(
    authority: &MutationAuthority,
    headers: &axum::http::HeaderMap,
    path: &str,
    body: serde_json::Value,
) -> CaduceusHttpReadback {
    caduceus_actuate_json_with_timeout(
        authority,
        headers,
        path,
        body,
        std::time::Duration::from_secs(4),
    )
}

fn caduceus_actuate_json_with_timeout(
    authority: &MutationAuthority,
    headers: &axum::http::HeaderMap,
    path: &str,
    body: serde_json::Value,
    timeout: std::time::Duration,
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

fn mutation_task_failure_readback(path: &str) -> CaduceusHttpReadback {
    mutation_refusal_readback(
        path,
        MutationRefusal {
            code: "caduceus-mutation-task-failed".to_string(),
            status: 503,
        },
    )
}

fn caduceus_actuate(
    authority: &MutationAuthority,
    headers: &axum::http::HeaderMap,
    path: &str,
) -> CaduceusHttpReadback {
    caduceus_actuate_with_timeout(
        authority,
        headers,
        path,
        std::time::Duration::from_secs(4),
    )
}

fn caduceus_actuate_with_timeout(
    authority: &MutationAuthority,
    headers: &axum::http::HeaderMap,
    path: &str,
    timeout: std::time::Duration,
) -> CaduceusHttpReadback {
    let context = MutationRequestContext::from_headers(headers);
    match authority.authorize(&context) {
        Ok(attendance) => caduceus_http_with_attendance_and_document_timeout(
                "POST",
                path,
                Some(&attendance.proof),
                Some(&attendance.document),
                timeout,
            ),
        Err(refusal) => mutation_refusal_readback(path, refusal),
    }
}

fn admin_fragment_caduceus_request(headers: &axum::http::HeaderMap, method: &str, path: &str) -> CaduceusHttpReadback {
    let authority = mutation_authority();
    match authority.authorize(
        &MutationRequestContext::attended_document_from_headers(headers),
    ) {
        Ok(attendance) => caduceus_http_with_attendance_and_document(
                method,
                path,
                Some(&attendance.proof),
                Some(&attendance.document),
            ),
        Err(refusal) => mutation_refusal_readback(path, refusal),
    }
}

fn admin_fragment_caduceus_json_request(
    headers: &axum::http::HeaderMap,
    method: &str,
    path: &str,
    body: serde_json::Value,
) -> CaduceusHttpReadback {
    let authority = mutation_authority();
    match authority.authorize(
        &MutationRequestContext::attended_document_from_headers(headers),
    ) {
        Ok(attendance) => caduceus_http_json_with_attendance_and_document(
                method,
                path,
                body,
                Some(&attendance.proof),
                Some(&attendance.document),
            ),
        Err(refusal) => mutation_refusal_readback(path, refusal),
    }
}

fn translation_debt_readback(
    method: &str,
    path: &str,
    historical_target: Option<&str>,
    current_candidate: Option<&str>,
) -> CaduceusHttpReadback {
    let mut body = serde_json::json!({
        "ok": false,
        "method": method,
        "path": path,
        "firstMissingSignal": "coronatio-caduceus-route-translation-required",
    });
    if let Some(target) = historical_target {
        body["historicalTarget"] = serde_json::Value::String(target.to_string());
    }
    if let Some(candidate) = current_candidate {
        body["currentCandidate"] = serde_json::Value::String(candidate.to_string());
    }
    CaduceusHttpReadback {
        ok: false,
        status: 0,
        path: path.to_string(),
        body,
        first_missing_signal: "coronatio-caduceus-route-translation-required".to_string(),
    }
}

fn caduceus_translation_debt(
    authority: &MutationAuthority,
    headers: &axum::http::HeaderMap,
    method: &str,
    path: &str,
    historical_target: Option<&str>,
    current_candidate: Option<&str>,
) -> CaduceusHttpReadback {
    let context = MutationRequestContext::from_headers(headers);
    match authority.authorize(&context) {
        Ok(_) => translation_debt_readback(method, path, historical_target, current_candidate),
        Err(refusal) => mutation_refusal_readback(path, refusal),
    }
}

fn route_translation_debt(
    authority: &MutationAuthority,
    headers: &axum::http::HeaderMap,
    method: &str,
    path: &str,
    historical_target: Option<&str>,
    current_candidate: Option<&str>,
) -> CaduceusHttpReadback {
    match method {
        "POST" | "PUT" | "DELETE" => {},
        _ => return mutation_refusal_readback(path, MutationRefusal { code: "coronatio-mutation-method-unmapped".to_string(), status: 0 }),
    }
    caduceus_translation_debt(authority, headers, method, path, historical_target, current_candidate)
}

fn admin_fragment_translation_debt(
    headers: &axum::http::HeaderMap,
    method: &str,
    path: &str,
) -> CaduceusHttpReadback {
    caduceus_translation_debt(
        &mutation_authority(),
        headers,
        method,
        path,
        None,
        None,
    )
}

async fn portal_edit_mutation_route(
    headers: axum::http::HeaderMap,
    uri: Uri,
    decoded_name: Result<Path<String>, axum::extract::rejection::PathRejection>,
    body: Bytes,
) -> Response {
    tokio::task::spawn_blocking(move || {
        let path = uri.path();
        let context = MutationRequestContext::from_headers(&headers);
        let attendance = match mutation_authority().authorize(&context) {
            Ok(attendance) => attendance,
            Err(refusal) => {
                // Keep the old homeserver_mutation_response refusal on the attendance boundary.
                let caduceus = mutation_refusal_readback(path, refusal);
                return (
                    mutation_response_status(&caduceus),
                    Json(serde_json::json!({
                        "schema": "coronatio.homeserver.route.mutation.v1",
                        "ok": caduceus.ok,
                        "accepted": caduceus.ok,
                        "method": "PUT",
                        "path": path,
                        "family": homeserver_route_family(path),
                        "authority": "Caduceus staff transition membrane",
                        "caduceus": caduceus,
                        "firstMissingSignal": caduceus.first_missing_signal,
                    })),
                ).into_response();
            }
        };
        let bad_request = |signal| {
            (StatusCode::BAD_REQUEST, Json(serde_json::json!({
                "ok": false,
                "firstMissingSignal": signal,
            }))).into_response()
        };
        let Ok(Path(portal_name)) = decoded_name else {
            return bad_request("coronatio-portal-edit-path-invalid");
        };
        let Ok(body) = serde_json::from_slice::<serde_json::Value>(&body) else {
            return bad_request("coronatio-portal-edit-body-invalid");
        };
        let Some(name) = body.get("name").and_then(serde_json::Value::as_str) else {
            return bad_request("coronatio-portal-edit-body-invalid");
        };
        if name != portal_name {
            return bad_request("coronatio-portal-edit-name-mismatch");
        }

        // Match Caduceus routes/portals/index.rs:name_slug, including edge separators and Unicode.
        let mut slug = String::new();
        let mut separator = false;
        for ch in name.to_lowercase().chars() {
            if ch.is_alphanumeric() {
                slug.push(ch);
                separator = false;
            } else if !separator {
                slug.push('-');
                separator = true;
            }
        }
        // The slug contains only alphanumerics and hyphens; encode its non-ASCII bytes for the HTTP path.
        let slug: String = url::form_urlencoded::byte_serialize(slug.as_bytes()).collect();
        let target = format!("/api/v1/portals/{slug}");
        let readback = caduceus_http_json_with_attendance_and_document(
            "PUT", &target, body, Some(&attendance.proof), Some(&attendance.document),
        );
        if readback.ok && readback.body.get("ok").and_then(serde_json::Value::as_bool) == Some(true) {
            crate::indicators::invalidate_indicator_portal_metadata();
        }
        let status = if !readback.ok && (400..=599).contains(&readback.status) {
            StatusCode::from_u16(readback.status).unwrap_or_else(|_| mutation_response_status(&readback))
        } else {
            mutation_response_status(&readback)
        };
        let body = if readback.status == 0 {
            serde_json::json!({"ok": false, "firstMissingSignal": readback.first_missing_signal})
        } else {
            readback.body
        };
        (status, Json(body)).into_response()
    }).await.unwrap_or_else(|_| {
        (StatusCode::SERVICE_UNAVAILABLE, Json(serde_json::json!({
            "ok": false,
            "firstMissingSignal": "caduceus-mutation-task-failed",
        }))).into_response()
    })
}

fn mutation_config_set(
    authority: &MutationAuthority,
    headers: &axum::http::HeaderMap,
    path: &str,
    value: serde_json::Value,
) -> CaduceusHttpReadback {
    let readback = caduceus_actuate_json(
        authority,
        headers,
        "/api/v1/config/set",
        serde_json::json!({"path": path, "value": value}),
    );
    if readback.ok && path.starts_with("tabs.portals.visibility.") {
        crate::indicators::invalidate_indicator_portal_metadata();
    }
    readback
}
