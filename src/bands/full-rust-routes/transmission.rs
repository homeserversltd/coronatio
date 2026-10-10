const TRANSMISSION_KEYS_READ_TARGET: &str = "/api/v1/transmission/keys";
const TRANSMISSION_KEYS_REPLACE_TARGET: &str = "/api/v1/transmission/keys/replace";
const TRANSMISSION_KEYS_ROTATE_TARGET: &str = "/api/v1/transmission/keys/rotate";

async fn transmission_keys_route() -> Response {
    transmission_keys_read_response(caduceus_http("GET", TRANSMISSION_KEYS_READ_TARGET))
}

fn transmission_keys_read_response(readback: CaduceusHttpReadback) -> Response {
    let presence = readback.body.get("presence")
        .or_else(|| readback.body.get("keys"))
        .unwrap_or(&readback.body);
    let pia = transmission_key_present(presence.get("pia"));
    let transmission = transmission_key_present(presence.get("transmission"));
    let complete = pia.is_some() && transmission.is_some();
    let ok = readback.ok && complete;
    let signal = if ok {
        "none".to_string()
    } else if readback.ok {
        "transmission-keys-presence-unavailable".to_string()
    } else {
        transmission_named_signal(&readback.body, &readback.first_missing_signal, &[])
    };
    let status = if ok {
        StatusCode::OK
    } else if (400..=599).contains(&readback.status) {
        StatusCode::from_u16(readback.status).unwrap_or(StatusCode::SERVICE_UNAVAILABLE)
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };
    (
        status,
        Json(serde_json::json!({
            "schema": "coronatio.transmission.keys.read.v1",
            "ok": ok,
            "presence": {"pia": pia, "transmission": transmission},
            "firstMissingSignal": signal
        })),
    ).into_response()
}

fn transmission_key_present(value: Option<&serde_json::Value>) -> Option<bool> {
    let value = value?;
    if let Some(present) = value.as_bool() {
        return Some(present);
    }
    if let Some(status) = value.as_str() {
        return match status {
            "present" => Some(true),
            "absent" => Some(false),
            _ => None,
        };
    }
    if let Some(present) = value.get("present").and_then(serde_json::Value::as_bool) {
        return Some(present);
    }
    match value.get("status").and_then(serde_json::Value::as_str) {
        Some("present") => Some(true),
        Some("absent") => Some(false),
        _ => None,
    }
}

async fn transmission_keys_replace_route(headers: axum::http::HeaderMap, body: Bytes) -> Response {
    let payload: serde_json::Value = match serde_json::from_slice(&body) {
        Ok(value) => value,
        Err(_) => return transmission_keys_invalid_request_response(),
    };
    let service = payload.get("service").and_then(serde_json::Value::as_str);
    let username = payload.get("username").and_then(serde_json::Value::as_str);
    let password = payload.get("password").and_then(serde_json::Value::as_str);
    let (Some("pia"), Some(username), Some(password)) = (service, username, password) else {
        return transmission_keys_invalid_request_response();
    };
    if username.is_empty() || password.is_empty() {
        return transmission_keys_invalid_request_response();
    }

    let downstream = serde_json::json!({"service":"pia","username":username,"password":password});
    let readback = tokio::task::spawn_blocking(move || {
        caduceus_actuate_json_with_timeout(
            &mutation_authority(),
            &headers,
            MutationActionTarget::attended_parent("coronatio.transmission.keys.replace", TRANSMISSION_KEYS_REPLACE_TARGET),
            TRANSMISSION_KEYS_REPLACE_TARGET,
            downstream,
            std::time::Duration::from_secs(180),
        )
    })
    .await
    .unwrap_or_else(|_| mutation_task_failure_readback(TRANSMISSION_KEYS_REPLACE_TARGET));
    transmission_keys_mutation_response(readback, &[username, password])
}

async fn transmission_keys_rotate_route(headers: axum::http::HeaderMap, _body: Bytes) -> Response {
    let readback = tokio::task::spawn_blocking(move || {
        caduceus_actuate_json_with_timeout(
            &mutation_authority(),
            &headers,
            MutationActionTarget::attended_parent("coronatio.transmission.keys.rotate", TRANSMISSION_KEYS_ROTATE_TARGET),
            TRANSMISSION_KEYS_ROTATE_TARGET,
            serde_json::json!({"service":"transmission"}),
            std::time::Duration::from_secs(180),
        )
    })
    .await
    .unwrap_or_else(|_| mutation_task_failure_readback(TRANSMISSION_KEYS_ROTATE_TARGET));
    transmission_keys_mutation_response(readback, &[])
}

fn transmission_keys_invalid_request_response() -> Response {
    (
        StatusCode::BAD_REQUEST,
        Json(serde_json::json!({
            "schema": "coronatio.transmission.keys.mutation.v1",
            "ok": false,
            "accepted": false,
            "firstMissingSignal": "transmission-keys-request-invalid"
        })),
    ).into_response()
}

fn transmission_keys_mutation_response(readback: CaduceusHttpReadback, forbidden: &[&str]) -> Response {
    let semantic_success = readback.body.get("ok")
        .and_then(serde_json::Value::as_bool)
        .or_else(|| readback.body.get("success").and_then(serde_json::Value::as_bool))
        .unwrap_or(readback.ok);
    let ok = readback.ok && semantic_success;
    let signal = if ok {
        "none".to_string()
    } else {
        transmission_named_signal(&readback.body, &readback.first_missing_signal, forbidden)
    };
    let status = if ok {
        StatusCode::OK
    } else if (400..=599).contains(&readback.status) {
        StatusCode::from_u16(readback.status).unwrap_or(StatusCode::SERVICE_UNAVAILABLE)
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };
    (
        status,
        Json(serde_json::json!({
            "schema": "coronatio.transmission.keys.mutation.v1",
            "ok": ok,
            "accepted": ok,
            "firstMissingSignal": signal
        })),
    ).into_response()
}

fn transmission_named_signal(body: &serde_json::Value, fallback: &str, forbidden: &[&str]) -> String {
    let candidate = body.get("firstMissingSignal")
        .or_else(|| body.get("first_missing_signal"))
        .and_then(serde_json::Value::as_str)
        .unwrap_or(fallback);
    let named_downstream_signal = ["caduceus-", "transmission-", "agathodaimon-"]
        .iter()
        .any(|prefix| candidate.starts_with(prefix) && candidate.len() > prefix.len());
    let safe = named_downstream_signal
        && !candidate.is_empty()
        && candidate != "none"
        && candidate.len() <= 160
        && candidate.bytes().all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b':'))
        && !forbidden.iter().any(|secret| !secret.is_empty() && candidate.contains(secret));
    if safe { candidate.to_string() } else { "transmission-keys-refused".to_string() }
}
