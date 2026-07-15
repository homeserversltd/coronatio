// Single successor authority and actuator seam for all Coronatio state changes.
// Browser callers provide only same-origin request context and the opaque Caduceus
// session cookie. A one-use capability exists only during the one downstream call.
use crate::caduceus_access::{
    safe_access_code, same_origin_state_change, session_ticket_from_cookie, CapabilityTicket,
    CaduceusAccessClient, SessionTicket,
};

#[derive(Clone, Debug, PartialEq, Eq)]
struct MutationActionTarget {
    action: String,
    target: String,
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
        })
    }

    fn config(target: &str) -> Self {
        Self {
            action: "coronatio.config.set".to_string(),
            target: target.to_string(),
        }
    }

    fn caduceus(action: impl Into<String>, target: impl Into<String>) -> Self {
        Self {
            action: action.into(),
            target: target.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct MutationRequestContext {
    same_origin: bool,
    session: Option<SessionTicket>,
}

impl MutationRequestContext {
    fn from_headers(headers: &axum::http::HeaderMap) -> Self {
        let same_origin = same_origin_state_change(headers);
        if !same_origin {
            return Self { same_origin: false, session: None };
        }
        Self { same_origin: true, session: session_ticket_from_cookie(headers) }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct MutationCapability { token: CapabilityTicket }

impl MutationCapability {
    fn expose_for_one_request(&self) -> &str { &self.token.0 }
}

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
    ) -> Result<MutationCapability, MutationRefusal> {
        if !context.same_origin {
            return Err(MutationRefusal { code: "caduceus-access-origin-refused".to_string(), status: 403 });
        }
        let Some(ticket) = context.session.as_ref() else {
            return Err(MutationRefusal { code: "caduceus-access-session-required".to_string(), status: 401 });
        };
        let call = self
            .access
            .capability_mint(ticket, &mapping.action, &mapping.target);
        match (call.receipt.ok, call.capability) {
            (true, Some(token)) => Ok(MutationCapability { token }),
            _ => Err(MutationRefusal { code: call.receipt.code, status: call.receipt.status }),
        }
    }
}

fn mutation_authority() -> MutationAuthority { MutationAuthority::default() }

fn mutation_context_refusal(headers: &axum::http::HeaderMap) -> Option<MutationRefusal> {
    let context = MutationRequestContext::from_headers(headers);
    if !context.same_origin {
        Some(MutationRefusal { code: "caduceus-access-origin-refused".to_string(), status: 403 })
    } else if context.session.is_none() {
        Some(MutationRefusal { code: "caduceus-access-session-required".to_string(), status: 401 })
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
    if signal == "caduceus-access-origin-refused" {
        axum::http::StatusCode::FORBIDDEN
    } else if signal == "caduceus-access-session-required"
        || matches!(signal, "caduceus-access-refused" | "caduceus-session-refused" | "caduceus-capability-refused")
        || ((signal.contains("access") || signal.contains("session") || signal.contains("capability"))
            && (signal.ends_with("-refused") || signal.ends_with("-required")))
    {
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
    match authority.authorize(&MutationRequestContext::from_headers(headers), mapping) {
        Ok(capability) => caduceus_http_json_with_capability("POST", path, body, Some(capability.expose_for_one_request())),
        Err(refusal) => mutation_refusal_readback(path, refusal),
    }
}

fn caduceus_actuate(
    authority: &MutationAuthority,
    headers: &axum::http::HeaderMap,
    mapping: MutationActionTarget,
    path: &str,
) -> CaduceusHttpReadback {
    match authority.authorize(&MutationRequestContext::from_headers(headers), mapping) {
        Ok(capability) => caduceus_http_with_capability("POST", path, Some(capability.expose_for_one_request())),
        Err(refusal) => mutation_refusal_readback(path, refusal),
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
        return mutation_refusal_readback("/api/v1/staff/intent", MutationRefusal { code: "coronatio-mutation-method-unmapped".to_string(), status: 0 });
    };
    caduceus_actuate_json(
        authority,
        headers,
        mapping,
        "/api/v1/staff/intent",
        serde_json::json!({"method": method, "route": route, "classification": classification, "metadata": metadata}),
    )
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

#[cfg(test)]
mod mutation_authority_tests {
    use super::*;

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
            MutationRequestContext { same_origin: false, session: SessionTicket::parse("opaque-session") },
            MutationRequestContext { same_origin: true, session: None },
        ] {
            let refusal = authority.authorize(&context, mapping.clone()).unwrap_err();
            assert!(!refusal.code.contains("opaque-session"));
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
        assert!(context.session.is_none());
        assert!(mutation_context_refusal(&headers).is_some());
        assert!(crate::caduceus_access::test_fixture::records_since(mark).is_empty());
    }

    #[test]
    fn central_mutation_status_classifies_refusal_and_fault_signals() {
        for (signal, expected) in [
            ("caduceus-access-origin-refused", axum::http::StatusCode::FORBIDDEN),
            ("caduceus-access-session-required", axum::http::StatusCode::UNAUTHORIZED),
            ("caduceus-capability-refused", axum::http::StatusCode::UNAUTHORIZED),
            ("caduceus-capability-required", axum::http::StatusCode::UNAUTHORIZED),
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
