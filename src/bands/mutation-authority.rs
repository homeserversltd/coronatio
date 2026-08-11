// Single successor authority and actuator seam for all Coronatio state changes.
// Browser callers provide same-origin context, the current document incarnation,
// and the opaque attendance proof held only by that document.
use crate::caduceus_access::{attendance_from_headers, document_incarnation_from_headers, safe_access_code, same_origin_state_change, AttendanceProof, CaduceusAccessClient};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MutationAuthorization { SameOrigin, AttendedDocument }

#[derive(Clone, Debug, PartialEq, Eq)]
struct MutationActionTarget {
    action: String,
    target: String,
    authorization: MutationAuthorization,
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
            action: action.to_string(),
            target: target.to_string(),
            authorization: MutationAuthorization::SameOrigin,
        })
    }

    fn config(target: &str) -> Self {
        Self {
            action: "coronatio.config.set".to_string(),
            target: target.to_string(),
            authorization: MutationAuthorization::SameOrigin,
        }
    }

    fn caduceus(action: impl Into<String>, target: impl Into<String>) -> Self {
        let action = action.into();
        let authorization = match action.as_str() {
            "coronatio.linker.browse" | "coronatio.linker.hardlink-scan" => MutationAuthorization::AttendedDocument,
            _ => MutationAuthorization::SameOrigin,
        };
        Self { action, target: target.into(), authorization }
    }

    fn request_context(&self, headers: &axum::http::HeaderMap) -> MutationRequestContext {
        match self.authorization {
            MutationAuthorization::SameOrigin => MutationRequestContext::from_headers(headers),
            MutationAuthorization::AttendedDocument => MutationRequestContext::attended_document_from_headers(headers),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct MutationRequestContext {
    same_origin: bool,
    document: Option<String>,
    attendance: Option<AttendanceProof>,
}

impl MutationRequestContext {
    fn from_headers(headers: &axum::http::HeaderMap) -> Self {
        let same_origin = same_origin_state_change(headers);
        if !same_origin {
            return Self { same_origin: false, document: None, attendance: None };
        }
        Self { same_origin: true, document: document_incarnation_from_headers(headers), attendance: attendance_from_headers(headers) }
    }

    fn attended_document_from_headers(headers: &axum::http::HeaderMap) -> Self {
        Self {
            same_origin: true,
            document: document_incarnation_from_headers(headers),
            attendance: attendance_from_headers(headers),
        }
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
        mapping: MutationActionTarget,
    ) -> Result<MutationAttendance, MutationRefusal> {
        if !context.same_origin {
            return Err(MutationRefusal { code: "caduceus-access-origin-refused".to_string(), status: 403 });
        }
        let Some(document) = context.document.as_ref() else {
            return Err(MutationRefusal { code: "caduceus-attendance-document-required".to_string(), status: 400 });
        };
        let Some(attendance) = context.attendance.as_ref() else {
            return Err(MutationRefusal { code: "caduceus-attendance-required".to_string(), status: 401 });
        };
        let call = self.access.attendance_validate(attendance, document);
        if call.receipt.ok { Ok(MutationAttendance { proof: attendance.clone(), document: document.clone() }) }
        else { Err(MutationRefusal { code: call.receipt.code, status: call.receipt.status }) }
    }
}

fn mutation_authority() -> MutationAuthority { MutationAuthority::default() }

fn mutation_context_refusal(headers: &axum::http::HeaderMap) -> Option<MutationRefusal> {
    let context = MutationRequestContext::from_headers(headers);
    if !context.same_origin {
        Some(MutationRefusal { code: "caduceus-access-origin-refused".to_string(), status: 403 })
    } else if context.document.is_none() {
        Some(MutationRefusal { code: "caduceus-attendance-document-required".to_string(), status: 400 })
    } else if context.attendance.is_none() {
        Some(MutationRefusal { code: "caduceus-attendance-required".to_string(), status: 401 })
    } else {
        None
    }
}

fn admin_fragment_context_refusal(headers: &axum::http::HeaderMap) -> Option<MutationRefusal> {
    let context = MutationRequestContext::attended_document_from_headers(headers);
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
    if matches!(signal, "caduceus-access-origin-refused" | "caduceus-attendance-origin-refused") {
        axum::http::StatusCode::FORBIDDEN
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
    ) {
        axum::http::StatusCode::UNAUTHORIZED
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
    let context = mapping.request_context(headers);
    match authority.authorize(&context, mapping) {
        Ok(attendance) => caduceus_http_json_with_attendance_and_document(
            "POST",
            path,
            body,
            Some(&attendance.proof),
            Some(&attendance.document),
        ),
        Err(refusal) => mutation_refusal_readback(path, refusal),
    }
}

fn caduceus_actuate(
    authority: &MutationAuthority,
    headers: &axum::http::HeaderMap,
    mapping: MutationActionTarget,
    path: &str,
) -> CaduceusHttpReadback {
    let context = mapping.request_context(headers);
    match authority.authorize(&context, mapping) {
        Ok(attendance) => caduceus_http_with_attendance("POST", path, Some(&attendance.proof)),
        Err(refusal) => mutation_refusal_readback(path, refusal),
    }
}

fn admin_fragment_caduceus_request(headers: &axum::http::HeaderMap, method: &str, path: &str) -> CaduceusHttpReadback {
    let authority = mutation_authority();
    match authority.authorize(
        &MutationRequestContext::attended_document_from_headers(headers),
        MutationActionTarget::caduceus("coronatio.admin.fragment", path),
    ) {
        Ok(attendance) => caduceus_http_with_attendance_and_document(method, path, Some(&attendance.proof), Some(&attendance.document)),
        Err(refusal) => mutation_refusal_readback(path, refusal),
    }
}

fn admin_fragment_staff_intent(headers: &axum::http::HeaderMap, method: &str, route: &str, classification: &str) -> CaduceusHttpReadback {
    mutation_staff_intent_with_mapping(
        &mutation_authority(),
        headers,
        MutationActionTarget::caduceus("coronatio.admin.fragment", route),
        method,
        route,
        classification,
        serde_json::json!({}),
    )
}

fn mutation_staff_door(method: &str, route: &str, _classification: &str) -> Option<(&'static str, String)> {
    match (method, route) {
        ("POST", "/api/files/upload") => Some(("POST", "/api/v1/file/ingress".to_string())),
        ("POST", "/api/backblaze/config") => Some(("POST", "/api/v1/backblaze/config".to_string())),
        ("POST", "/api/upload/force-permissions") => Some(("POST", "/api/v1/upload/force-permissions".to_string())),
        ("POST", "/api/service/control") => Some(("POST", "/api/v1/service/control".to_string())),
        ("POST", "/api/dhcp/reservations") => Some(("POST", "/api/v1/network/dhcp/reservations".to_string())),
        ("PUT", route) if route.starts_with("/api/dhcp/reservations/") => Some(("PUT", route.replacen("/api/dhcp", "/api/v1/network/dhcp", 1))),
        ("DELETE", route) if route.starts_with("/api/dhcp/reservations/") => Some(("DELETE", route.replacen("/api/dhcp", "/api/v1/network/dhcp", 1))),
        ("POST", "/api/dhcp/pool-boundary") => Some(("POST", "/api/v1/network/dhcp/pool-boundary".to_string())),
        ("POST", "/api/dhcp/config") => Some(("POST", "/api/v1/network/dhcp".to_string())),
        ("POST", "/usr/local/sbin/caduceus-keyman-rotate-capability") => Some(("POST", "/api/v1/keyman/rotate-capability".to_string())),
        ("POST", "/api/admin/ssh/status")
        | ("POST", "/api/admin/ssh/toggle")
        | ("POST", "/api/admin/ssh/service/status")
        | ("POST", "/api/admin/ssh/service")
        | ("POST", "/api/admin/samba/status")
        | ("POST", "/api/admin/samba/service")
        | ("POST", "/api/admin/system/restart")
        | ("POST", "/api/admin/system/shutdown")
        | ("POST", "/api/admin/services/hard-reset")
        | ("POST", "/api/admin/hard-drive-test/start") => Some(("POST", route.to_string())),
        _ => None,
    }
}

fn mutation_staff_intent_with_mapping(
    authority: &MutationAuthority,
    headers: &axum::http::HeaderMap,
    mapping: MutationActionTarget,
    method: &str,
    route: &str,
    classification: &str,
    metadata: serde_json::Value,
) -> CaduceusHttpReadback {
    let Some((door_method, door_path)) = mutation_staff_door(method, route, classification) else {
        return mutation_refusal_readback(route, MutationRefusal { code: "coronatio-caduceus-door-unmapped".to_string(), status: 0 });
    };
    let context = mapping.request_context(headers);
    match authority.authorize(&context, mapping) {
        Ok(attendance) => caduceus_http_json_with_attendance_and_document(
            door_method,
            &door_path,
            metadata,
            Some(&attendance.proof),
            Some(&attendance.document),
        ),
        Err(refusal) => mutation_refusal_readback(&door_path, refusal),
    }
}

fn mutation_staff_intent(
    authority: &MutationAuthority,
    headers: &axum::http::HeaderMap,
    method: &str,
    route: &str,
    classification: &str,
    metadata: serde_json::Value,
) -> CaduceusHttpReadback {
    let Some(mapping) = MutationActionTarget::route(method, route) else {
        return mutation_refusal_readback(route, MutationRefusal { code: "coronatio-mutation-method-unmapped".to_string(), status: 0 });
    };
    mutation_staff_intent_with_mapping(authority, headers, mapping, method, route, classification, metadata)
}

fn mutation_config_set(
    authority: &MutationAuthority,
    headers: &axum::http::HeaderMap,
    path: &str,
    value: serde_json::Value,
) -> CaduceusHttpReadback {
    caduceus_actuate_json(
        authority,
        headers,
        MutationActionTarget::config(path),
        "/api/v1/config/set",
        serde_json::json!({"path": path, "value": value}),
    )
}

#[cfg(test)]
fn mutation_mapping_table() -> Vec<(String, String, String)> {
    let mut table = full_rust_route_inventory()
        .iter()
        .flat_map(|(path, methods)| {
            methods.iter().filter_map(move |method| {
                MutationActionTarget::route(method.to_ascii_uppercase().as_str(), path)
                    .map(|mapping| ((*method).to_string(), (*path).to_string(), mapping.action))
            })
        })
        .collect::<Vec<_>>();
    table.extend([
        ("post".to_string(), "/api/set_starred_tab".to_string(), "coronatio.config.set".to_string()),
        ("post".to_string(), "/api/caduceus/update/check".to_string(), "caduceus.update.check".to_string()),
        ("post".to_string(), "/api/caduceus/update/now".to_string(), "caduceus.update.now".to_string()),
        ("post".to_string(), "/admit/admin/toggle/:toggle_id".to_string(), "coronatio.route.post".to_string()),
        ("post".to_string(), "/admit/admin/action/:action_id".to_string(), "coronatio.route.post".to_string()),
    ]);
    table
}

#[cfg(any())]
mod mutation_authority_tests {

    #[test]
    fn mutation_mapping_covers_every_registered_state_changing_route() {
        let table = mutation_mapping_table();
        let mut unique = std::collections::BTreeSet::new();
        for (method, path, _) in &table {
            assert!(
                unique.insert((method.clone(), path.clone())),
                "duplicate mutation mapping {method} {path}"
            );
        }
        for (path, methods) in full_rust_route_inventory() {
            for method in *methods {
                if *method != "get" {
                    assert!(table.iter().any(|(mapped_method, mapped_path, _)| mapped_method == method && mapped_path == path), "unmapped mutation {method} {path}");
                }
            }
        }
        assert_eq!(table.len(), unique.len());
    }

    #[test]
    fn successor_refuses_missing_origin_or_cookie_without_ticket_leak() {
        let authority = MutationAuthority { access: CaduceusAccessClient::new("http://127.0.0.1:9") };
        let mapping = MutationActionTarget::caduceus("caduceus.update.now", "/api/v1/update/now");
        for context in [
            MutationRequestContext { same_origin: false, document: Some("test-document".to_string()), attendance: AttendanceProof::parse("test-attendance") },
            MutationRequestContext { same_origin: true, document: Some("test-document".to_string()), attendance: None },
        ] {
            let refusal = authority.authorize(&context, mapping.clone()).unwrap_err();
            assert!(!refusal.code.contains("test-attendance"));
            assert!(refusal.code.contains("refused") || refusal.code.contains("required"));
        }
    }

    #[test]
    fn ordered_context_never_extracts_cross_origin_cookie_or_calls_upstream() {
        let mark = crate::caduceus_access::test_fixture::mark();
        let mut headers = crate::caduceus_access::test_fixture::same_origin_headers(true);
        headers.insert(axum::http::header::ORIGIN, axum::http::HeaderValue::from_static("https://evil.example"));
        let context = MutationRequestContext::from_headers(&headers);
        assert!(!context.same_origin);
        assert!(context.attendance.is_none());
        assert!(mutation_context_refusal(&headers).is_some());
        assert!(crate::caduceus_access::test_fixture::records_since(mark).is_empty());
    }

    #[test]
    fn central_mutation_status_classifies_refusal_and_fault_signals() {
        for (signal, expected) in [
            ("caduceus-access-origin-refused", axum::http::StatusCode::FORBIDDEN),
            ("caduceus-attendance-required", axum::http::StatusCode::UNAUTHORIZED),
            ("caduceus-attendance-refused", axum::http::StatusCode::UNAUTHORIZED),
            ("caduceus-attendance-invalid", axum::http::StatusCode::UNAUTHORIZED),
            ("caduceus-access-unavailable", axum::http::StatusCode::SERVICE_UNAVAILABLE),
            ("caduceus-access-malformed-response", axum::http::StatusCode::SERVICE_UNAVAILABLE),
            ("caduceus-loopback-required", axum::http::StatusCode::SERVICE_UNAVAILABLE),
        ] {
            let readback = CaduceusHttpReadback {
                ok: false,
                status: 0,
                path: "/test".to_string(),
                body: serde_json::json!({}),
                first_missing_signal: signal.to_string(),
            };
            assert_eq!(mutation_response_status(&readback), expected, "{signal}");
        }
    }
}
