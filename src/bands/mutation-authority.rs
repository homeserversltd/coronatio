// Single successor authority and actuator seam for all Coronatio state changes.
// The current document incarnation and its opaque attendance proof are carried only by that document.
use crate::caduceus_access::{attendance_from_headers, document_incarnation_from_headers, safe_access_code, AttendanceProof, CaduceusAccessClient};

// Route/action labels remain descriptive metadata; they do not select an attendance scope.
#[derive(Clone, Debug, PartialEq, Eq)]
struct MutationActionTarget {
    _action: String,
    _target: String,
}

impl MutationActionTarget {
    fn route(method: &str, target: &str) -> Option<Self> {
        let action = match method {
            "POST" => "coronatio.route.post",
            "PUT" => "coronatio.route.put",
            "DELETE" => "coronatio.route.delete",
            _ => return None,
        };
        Some(Self {
            _action: action.to_string(),
            _target: target.to_string(),
        })
    }

    fn config(target: &str) -> Self {
        Self {
            _action: "coronatio.config.set".to_string(),
            _target: target.to_string(),
        }
    }

    fn caduceus(action: impl Into<String>, target: impl Into<String>) -> Self {
        Self { _action: action.into(), _target: target.into() }
    }

    fn attended_parent(action: impl Into<String>, target: impl Into<String>) -> Self {
        Self::caduceus(action, target)
    }

    fn request_context(&self, headers: &axum::http::HeaderMap) -> MutationRequestContext {
        MutationRequestContext::from_headers(headers)
    }
}

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
        _mapping: MutationActionTarget,
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
    mapping: MutationActionTarget,
    path: &str,
    body: serde_json::Value,
) -> CaduceusHttpReadback {
    caduceus_actuate_json_with_timeout(
        authority,
        headers,
        mapping,
        path,
        body,
        std::time::Duration::from_secs(4),
    )
}

fn caduceus_actuate_json_with_timeout(
    authority: &MutationAuthority,
    headers: &axum::http::HeaderMap,
    mapping: MutationActionTarget,
    path: &str,
    body: serde_json::Value,
    timeout: std::time::Duration,
) -> CaduceusHttpReadback {
    let context = mapping.request_context(headers);
    match authority.authorize(&context, mapping) {
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
    mapping: MutationActionTarget,
    path: &str,
) -> CaduceusHttpReadback {
    caduceus_actuate_with_timeout(
        authority,
        headers,
        mapping,
        path,
        std::time::Duration::from_secs(4),
    )
}

fn caduceus_actuate_with_timeout(
    authority: &MutationAuthority,
    headers: &axum::http::HeaderMap,
    mapping: MutationActionTarget,
    path: &str,
    timeout: std::time::Duration,
) -> CaduceusHttpReadback {
    let context = mapping.request_context(headers);
    match authority.authorize(&context, mapping) {
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
        MutationActionTarget::attended_parent("coronatio.admin.fragment", path),
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
        MutationActionTarget::attended_parent("coronatio.admin.fragment", path),
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
    mapping: MutationActionTarget,
    method: &str,
    path: &str,
    historical_target: Option<&str>,
    current_candidate: Option<&str>,
) -> CaduceusHttpReadback {
    let context = mapping.request_context(headers);
    match authority.authorize(&context, mapping) {
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
    let Some(mapping) = MutationActionTarget::route(method, path) else {
        return mutation_refusal_readback(path, MutationRefusal { code: "coronatio-mutation-method-unmapped".to_string(), status: 0 });
    };
    caduceus_translation_debt(authority, headers, mapping, method, path, historical_target, current_candidate)
}

fn admin_fragment_translation_debt(
    headers: &axum::http::HeaderMap,
    method: &str,
    path: &str,
) -> CaduceusHttpReadback {
    caduceus_translation_debt(
        &mutation_authority(),
        headers,
        MutationActionTarget::caduceus("coronatio.admin.fragment", path),
        method,
        path,
        None,
        None,
    )
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
        MutationActionTarget::config(path),
        "/api/v1/config/set",
        serde_json::json!({"path": path, "value": value}),
    );
    if readback.ok && path.starts_with("tabs.portals.visibility.") {
        crate::indicators::invalidate_indicator_portal_metadata();
    }
    readback
}
