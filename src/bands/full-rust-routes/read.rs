#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct HomeserverReadGuestResponse { schema: &'static str, ok: bool, success: bool, status: &'static str, first_missing_signal: &'static str }

async fn homeserver_logs_hyalos_response(headers: &axum::http::HeaderMap, method: &str, path: &str) -> Response {
    match session_projection_from_headers(headers).await {
        Session::Admin => {
            let readback = hyalos_tail_readback(None, 100);
            let events = readback
                .body
                .get("events")
                .and_then(serde_json::Value::as_array)
                .cloned()
                .unwrap_or_default();
            let logs = events
                .iter()
                .filter_map(|event| event.get("message").and_then(serde_json::Value::as_str))
                .collect::<Vec<_>>()
                .join("\n");
            let ok = readback.ok;
            (
                if ok { StatusCode::OK } else { StatusCode::SERVICE_UNAVAILABLE },
                Json(serde_json::json!({
                    "schema": "coronatio.admin.logs.homeserver.v1",
                    "ok": ok,
                    "success": ok,
                    "method": method,
                    "path": path,
                    "family": homeserver_route_family(path),
                    "logs": logs,
                    "events": events,
                    "authority": "Caduceus Hyalos tail membrane",
                    "caduceus": readback,
                    "firstMissingSignal": if ok { "none" } else { readback.first_missing_signal.as_str() }
                })),
            )
                .into_response()
        }
        Session::Guest => (
            StatusCode::OK,
            Json(HomeserverReadGuestResponse {
                schema: "coronatio.homeserver.route.read.guest.v1",
                ok: true,
                success: true,
                status: "rust-route",
                first_missing_signal: "none",
            }),
        )
            .into_response(),
    }
}

async fn homeserver_read_response(headers: &axum::http::HeaderMap, method: &str, path: &str) -> Response {
    if path == "/api/status/power/usage" || path == "/status/power/usage" {
        return power_usage_response(headers, method, path).await;
    }
    if path == "/api/status/tailscale/config" {
        return tailscale_config_read_response(headers).await;
    }
    if path == "/api/admin/logs/homeserver" {
        return homeserver_logs_hyalos_response(headers, method, path).await;
    }
    let topic = match path {
        "/api/status" => Some("internet.status"),
        "/api/status/services" => Some("services.status"),
        "/api/status/tailscale" => Some("tailscale.status"),
        "/api/status/vpn/pia" | "/api/status/vpn/transmission" => Some("vpn.status"),
        _ => None,
    };
    if let Some(topic) = topic {
        let (session, sample) = collect_indicator_topic_for_request(topic, headers).await;
        return indicator_projection_response(path, topic, session, sample);
    }
    match session_projection_from_headers(headers).await {
        Session::Admin => (
            StatusCode::OK,
            Json(serde_json::json!({
                "schema":"coronatio.homeserver.route.read.v1","ok":true,"success":true,
                "method":method,"path":path,"family":homeserver_route_family(path),
                "status":"rust-route","authority":"Coronatio Rust route","firstMissingSignal":"none"
            })),
        ).into_response(),
        Session::Guest => (StatusCode::OK, Json(HomeserverReadGuestResponse {
            schema:"coronatio.homeserver.route.read.guest.v1", ok:true, success:true,
            status:"rust-route", first_missing_signal:"none",
        })).into_response(),
    }
}

async fn indicator_legacy_read_route(headers: axum::http::HeaderMap, uri: Uri) -> Response {
    let path = uri.path();
    let topic = match path {
        "/api/status/services" => "services.status",
        "/api/status/tailscale" => "tailscale.status",
        "/api/status/vpn/pia" | "/api/status/vpn/transmission" => "vpn.status",
        _ => return (StatusCode::NOT_FOUND, Json(serde_json::json!({"firstMissingSignal":"coronatio-indicator-route-unknown"}))).into_response(),
    };
    let (session, sample) = collect_indicator_topic_for_request(topic, &headers).await;
    indicator_projection_response(path, topic, session, sample)
}

async fn internet_status_route(headers: axum::http::HeaderMap) -> Response {
    let (session, sample) = collect_indicator_topic_for_request("internet.status", &headers).await;
    indicator_projection_response("/api/status", "internet.status", session, sample)
}

fn caduceus_source_status(readback: &CaduceusHttpReadback) -> StatusCode {
    if readback.status != 0 {
        StatusCode::from_u16(readback.status).unwrap_or(StatusCode::SERVICE_UNAVAILABLE)
    } else {
        mutation_response_status(readback)
    }
}

fn indicator_projection_response(path: &str, topic: &str, session: Session, sample: IndicatorTopicSample) -> Response {
    let unavailable = sample.unavailable_signal;
    let mut body = sample.snapshot;
    if let Some(object) = body.as_object_mut() {
        object.insert("schema".to_owned(), serde_json::Value::String("coronatio.indicator.topic.v1".to_owned()));
        object.insert("ok".to_owned(), serde_json::Value::Bool(unavailable.is_none()));
        object.insert("success".to_owned(), serde_json::Value::Bool(unavailable.is_none()));
        object.insert("topicId".to_owned(), serde_json::Value::String(topic.to_owned()));
        object.insert("path".to_owned(), serde_json::Value::String(path.to_owned()));
        if let Some(signal) = unavailable.as_ref() {
            let existing = object.get("firstMissingSignal").and_then(serde_json::Value::as_str);
            if existing.is_none_or(|value| value.is_empty() || value == "none") {
                object.insert("firstMissingSignal".to_owned(), serde_json::Value::String(signal.clone()));
            } else if session == Session::Admin {
                object.insert("adminFirstMissingSignal".to_owned(), serde_json::Value::String(signal.clone()));
            }
        }
        object.entry("firstMissingSignal".to_owned()).or_insert(serde_json::Value::String("none".to_owned()));
    }
    (
        if unavailable.is_some() { StatusCode::SERVICE_UNAVAILABLE } else { StatusCode::OK },
        Json(body),
    ).into_response()
}

async fn tailscale_config_route(headers: axum::http::HeaderMap) -> Response {
    tailscale_config_read_response(&headers).await
}

async fn tailscale_config_read_response(headers: &axum::http::HeaderMap) -> Response {
    let (session, sample) = collect_indicator_topic_for_request("tailscale.status", headers).await;
    if session != Session::Admin {
        return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({
            "schema":"coronatio.tailscale.config.v1","ok":false,"tailnet":null,
            "firstMissingSignal":sample.unavailable_signal.unwrap_or_else(|| "caduceus-attendance-admin-scope-required".to_owned())
        }))).into_response();
    }
    let signal = sample.unavailable_signal;
    let tailnet = sample.snapshot.get("tailnet").cloned().unwrap_or(serde_json::Value::Null);
    let ok = signal.is_none() && !tailnet.is_null();
    (
        if ok { StatusCode::OK } else { StatusCode::SERVICE_UNAVAILABLE },
        Json(serde_json::json!({
            "schema":"coronatio.tailscale.config.v1","ok":ok,"tailnet":tailnet,
            "firstMissingSignal":signal.unwrap_or_else(|| if tailnet.is_null() { "caduceus-stats-tailscale-tailnet-unavailable".to_owned() } else { "none".to_owned() })
        })),
    ).into_response()
}

async fn tailscale_mutation_route(headers: axum::http::HeaderMap, uri: Uri, body: Bytes) -> Response {
    let mut authkey_for_redaction = None;
    let (action, caduceus_path, request_body) = match uri.path() {
        "/api/status/tailscale/connect" => ("connect", "/api/v1/network/tailscale/up", None),
        "/api/status/tailscale/disconnect" => ("disconnect", "/api/v1/network/tailscale/down", None),
        "/api/status/tailscale/enable" => ("enable", "/api/v1/appliance/service/tailscaled/enable", None),
        "/api/status/tailscale/disable" => ("disable", "/api/v1/appliance/service/tailscaled/disable", None),
        "/api/status/tailscale/authkey" => {
            let payload = match serde_json::from_slice::<serde_json::Value>(&body) {
                Ok(value) => value,
                Err(_) => return tailscale_mutation_refusal("tailscale-authkey-payload-invalid"),
            };
            let key = payload.get("authKey").or_else(|| payload.get("key")).and_then(serde_json::Value::as_str);
            let Some(key) = key.filter(|value| !value.is_empty()) else { return tailscale_mutation_refusal("tailscale-authkey-required"); };
            authkey_for_redaction = Some(key.to_owned());
            ("authkey", "/api/v1/network/tailscale/authkey", Some(serde_json::json!({"key":key})))
        }
        "/api/status/tailscale/update-tailnet" | "/api/status/tailscale/config" => {
            let payload = match serde_json::from_slice::<serde_json::Value>(&body) {
                Ok(value) => value,
                Err(_) => return tailscale_mutation_refusal("tailscale-tailnet-payload-invalid"),
            };
            let name = payload.get("tailnetName").or_else(|| payload.get("name")).and_then(serde_json::Value::as_str);
            let Some(name) = name.filter(|value| !value.trim().is_empty()) else { return tailscale_mutation_refusal("tailscale-tailnet-name-required"); };
            ("update-tailnet", "/api/v1/network/tailscale/tailnet", Some(serde_json::json!({"name":name})))
        }
        _ => return tailscale_mutation_refusal("coronatio-tailscale-mutation-route-unknown"),
    };
    let timeout = match action {
        "authkey" => Duration::from_secs(35),
        "update-tailnet" => Duration::from_secs(365),
        "enable" | "disable" => Duration::from_secs(180),
        _ => Duration::from_secs(40),
    };
    let headers = headers.clone();
    let caduceus_path = caduceus_path.to_owned();
    let call = tokio::task::spawn_blocking(move || {
        let authority = mutation_authority();
        if matches!(action, "enable" | "disable") {
            // The appliance service door admits a child scoped to its own document and requires `{}`,
            // the same road as portal service control.
            return caduceus_actuate_json_with_timeout(
                &authority, &headers,
                MutationActionTarget::caduceus("coronatio.tailscale.service", &caduceus_path),
                &caduceus_path, serde_json::json!({}), timeout,
            );
        }
        let mapping = MutationActionTarget::attended_parent(format!("coronatio.tailscale.{action}"), caduceus_path.clone());
        match request_body {
            Some(request_body) => caduceus_actuate_json_with_timeout(
                &authority, &headers, mapping, &caduceus_path, request_body, timeout,
            ),
            None => caduceus_actuate_with_timeout(
                &authority, &headers, mapping, &caduceus_path, timeout,
            ),
        }
    }).await;
    let readback = match call {
        Ok(readback) => readback,
        Err(_) => return tailscale_mutation_refusal("caduceus-mutation-task-failed"),
    };
    if readback.ok {
        indicators::invalidate_caduceus_stats_sample();
        if action == "update-tailnet" {
            indicators::invalidate_indicator_metadata();
        }
    }
    let status = caduceus_source_status(&readback);
    let ok = readback.ok;
    let real_signal = readback.first_missing_signal.clone();
    let body = if let Some(key) = authkey_for_redaction.as_deref() {
        // Keep the real refusal signal; only the submitted key is redacted.
        let body = redact_tailscale_authkey_value(readback.body, key);
        if !ok && !body.is_object() {
            serde_json::json!({"ok":false,"receipt":body,"firstMissingSignal":redact_tailscale_authkey_value(serde_json::Value::String(real_signal), key)})
        } else {
            body
        }
    } else {
        readback.body
    };
    (status, Json(body)).into_response()
}

fn redact_tailscale_authkey_value(value: serde_json::Value, key: &str) -> serde_json::Value {
    match value {
        serde_json::Value::String(value) => serde_json::Value::String(value.replace(key, "[redacted]")),
        serde_json::Value::Array(values) => serde_json::Value::Array(
            values.into_iter().map(|value| redact_tailscale_authkey_value(value, key)).collect(),
        ),
        serde_json::Value::Object(values) => serde_json::Value::Object(
            values.into_iter()
                .map(|(name, value)| (
                    name.replace(key, "[redacted]"),
                    redact_tailscale_authkey_value(value, key),
                ))
                .collect(),
        ),
        value => value,
    }
}


fn tailscale_mutation_refusal(signal: &str) -> Response {
    (
        StatusCode::BAD_REQUEST,
        Json(serde_json::json!({"schema":"coronatio.tailscale.mutation.v1","ok":false,"success":false,"firstMissingSignal":signal})),
    ).into_response()
}

