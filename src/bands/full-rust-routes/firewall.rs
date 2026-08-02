#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct FirewallPolicyWrite {
    schema: String,
    mac: String,
    mode: String,
    sites: Vec<String>,
    expected_revision: String,
    enabled: bool,
    enforcement: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct FirewallPolicyDelete {
    schema: String,
    mac: String,
    expected_revision: String,
}

fn firewall_guest_refusal(path: &str) -> Response {
    (
        StatusCode::UNAUTHORIZED,
        Json(serde_json::json!({
            "schema": "coronatio.network.firewall.refusal.v1",
            "ok": false,
            "success": false,
            "path": path,
            "error": "admin-session-required",
            "firstMissingSignal": "admin-session-required"
        })),
    )
        .into_response()
}

fn firewall_invalid_payload_response(signal: &str) -> Response {
    (
        StatusCode::BAD_REQUEST,
        Json(serde_json::json!({
            "schema": "coronatio.network.firewall.policy.refusal.v1",
            "ok": false,
            "success": false,
            "accepted": false,
            "error": signal,
            "firstMissingSignal": signal
        })),
    )
        .into_response()
}

fn canonical_firewall_mac(raw: &str) -> Option<String> {
    canonical_network_note_mac(raw)
}

fn canonical_firewall_site(raw: &str) -> Option<String> {
    let site = raw.trim().trim_end_matches('.').to_ascii_lowercase();
    if site.is_empty() || site.len() > 253 || !site.contains('.') {
        return None;
    }
    let labels: Vec<&str> = site.split('.').collect();
    if labels.iter().any(|label| {
        label.is_empty()
            || label.len() > 63
            || label.starts_with('-')
            || label.ends_with('-')
            || !label.bytes().all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
    }) {
        return None;
    }
    Some(site)
}

fn canonical_firewall_revision(raw: &str) -> Option<String> {
    if raw.len() != 64 || !raw.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return None;
    }
    Some(raw.to_ascii_lowercase())
}

fn firewall_policy_request(policy: FirewallPolicyWrite, path_mac: &str) -> Result<serde_json::Value, &'static str> {
    let mac = canonical_firewall_mac(&policy.mac).ok_or("firewall-policy-mac-invalid")?;
    if mac != path_mac {
        return Err("firewall-policy-mac-mismatch");
    }
    if policy.schema != "caduceus.network.firewall.policy.v1"
        || policy.mode != "allow-only"
        || policy.enforcement != "dns-policy"
        || policy.sites.len() > 64
    {
        return Err("firewall-policy-invalid");
    }
    let expected_revision = canonical_firewall_revision(&policy.expected_revision)
        .ok_or("firewall-policy-invalid")?;
    let mut sites = Vec::with_capacity(policy.sites.len());
    for site in policy.sites {
        let site = canonical_firewall_site(&site).ok_or("firewall-policy-site-invalid")?;
        if sites.contains(&site) {
            return Err("firewall-policy-site-duplicate");
        }
        sites.push(site);
    }
    Ok(serde_json::json!({
        "schema": "caduceus.network.firewall.policy.v1",
        "mac": mac,
        "mode": "allow-only",
        "sites": sites,
        "expectedRevision": expected_revision,
        "enabled": policy.enabled,
        "enforcement": "dns-policy"
    }))
}

fn firewall_delete_request(policy: FirewallPolicyDelete, path_mac: &str) -> Result<serde_json::Value, &'static str> {
    let mac = canonical_firewall_mac(&policy.mac).ok_or("firewall-policy-mac-invalid")?;
    if mac != path_mac {
        return Err("firewall-policy-mac-mismatch");
    }
    if policy.schema != "caduceus.network.firewall.policy.delete.v1" {
        return Err("firewall-policy-delete-invalid");
    }
    let expected_revision = canonical_firewall_revision(&policy.expected_revision)
        .ok_or("firewall-policy-delete-invalid")?;
    Ok(serde_json::json!({
        "schema": "caduceus.network.firewall.policy.delete.v1",
        "mac": mac,
        "expectedRevision": expected_revision,
    }))
}

fn firewall_upstream_response(readback: CaduceusHttpReadback, method: &str, path: &str) -> Response {
    if !readback.body.is_null() {
        let status = StatusCode::from_u16(readback.status)
            .unwrap_or_else(|_| mutation_response_status(&readback));
        return (status, Json(readback.body)).into_response();
    }
    (
        mutation_response_status(&readback),
        Json(serde_json::json!({
            "schema": "coronatio.network.firewall.upstream-error.v1",
            "ok": false,
            "success": false,
            "accepted": false,
            "method": method,
            "path": path,
            "authority": "Caduceus DNS website policy",
            "caduceusStatus": readback.status,
            "firstMissingSignal": readback.first_missing_signal
        })),
    )
        .into_response()
}

async fn firewall_read_route(headers: axum::http::HeaderMap, uri: Uri) -> Response {
    let path = uri.path();
    if session_from_headers(&headers) != Session::Admin {
        return firewall_guest_refusal(path);
    }
    let upstream = match path {
        "/api/firewall/status" => "/api/v1/network/firewall/status",
        "/api/firewall/policies" => "/api/v1/network/firewall/policies",
        _ => return firewall_invalid_payload_response("firewall-route-invalid"),
    };
    firewall_upstream_response(caduceus_http("GET", upstream), "GET", path)
}

async fn firewall_policy_route(headers: axum::http::HeaderMap, Path(raw_mac): Path<String>) -> Response {
    let path = format!("/api/firewall/policies/{raw_mac}");
    if session_from_headers(&headers) != Session::Admin {
        return firewall_guest_refusal(&path);
    }
    let mac = match canonical_firewall_mac(&raw_mac) {
        Some(mac) => mac,
        None => return firewall_invalid_payload_response("firewall-policy-mac-invalid"),
    };
    let upstream = format!("/api/v1/network/firewall/policies/{mac}");
    firewall_upstream_response(caduceus_http("GET", &upstream), "GET", &path)
}

async fn firewall_policy_put_route(
    headers: axum::http::HeaderMap,
    Path(raw_mac): Path<String>,
    payload: Option<Json<FirewallPolicyWrite>>,
) -> Response {
    let path = format!("/api/firewall/policies/{raw_mac}");
    if session_from_headers(&headers) != Session::Admin {
        return firewall_guest_refusal(&path);
    }
    let mac = match canonical_firewall_mac(&raw_mac) {
        Some(mac) => mac,
        None => return firewall_invalid_payload_response("firewall-policy-mac-invalid"),
    };
    let request = match payload.and_then(|Json(policy)| firewall_policy_request(policy, &mac).ok()) {
        Some(request) => request,
        None => return firewall_invalid_payload_response("firewall-policy-invalid"),
    };
    let authority = mutation_authority();
    let attendance = match authority.authorize(
        &MutationRequestContext::from_headers(&headers),
        MutationActionTarget::caduceus("coronatio.network.firewall.put", "/api/v1/network/firewall/policies/{mac}"),
    ) {
        Ok(attendance) => attendance,
        Err(refusal) => return firewall_upstream_response(mutation_refusal_readback(&path, refusal), "PUT", &path),
    };
    let upstream = format!("/api/v1/network/firewall/policies/{mac}");
    firewall_upstream_response(
        caduceus_http_json_with_attendance_and_document("PUT", &upstream, request, Some(&attendance.proof), Some(&attendance.document)),
        "PUT",
        &path,
    )
}

async fn firewall_policy_delete_route(
    headers: axum::http::HeaderMap,
    Path(raw_mac): Path<String>,
    payload: Option<Json<FirewallPolicyDelete>>,
) -> Response {
    let path = format!("/api/firewall/policies/{raw_mac}");
    if session_from_headers(&headers) != Session::Admin {
        return firewall_guest_refusal(&path);
    }
    let mac = match canonical_firewall_mac(&raw_mac) {
        Some(mac) => mac,
        None => return firewall_invalid_payload_response("firewall-policy-mac-invalid"),
    };
    let request = match payload.and_then(|Json(policy)| firewall_delete_request(policy, &mac).ok()) {
        Some(request) => request,
        None => return firewall_invalid_payload_response("firewall-policy-delete-invalid"),
    };
    let authority = mutation_authority();
    let attendance = match authority.authorize(
        &MutationRequestContext::from_headers(&headers),
        MutationActionTarget::caduceus("coronatio.network.firewall.delete", "/api/v1/network/firewall/policies/{mac}"),
    ) {
        Ok(attendance) => attendance,
        Err(refusal) => return firewall_upstream_response(mutation_refusal_readback(&path, refusal), "DELETE", &path),
    };
    let upstream = format!("/api/v1/network/firewall/policies/{mac}");
    firewall_upstream_response(
        caduceus_http_json_with_attendance_and_document("DELETE", &upstream, request, Some(&attendance.proof), Some(&attendance.document)),
        "DELETE",
        &path,
    )
}
