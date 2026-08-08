#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct HomeserverReadGuestResponse { schema: &'static str, ok: bool, success: bool, status: &'static str, first_missing_signal: &'static str }

fn homeserver_logs_hyalos_response(headers: &axum::http::HeaderMap, method: &str, path: &str) -> Response {
    match session_from_headers(headers) {
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

fn homeserver_read_response(headers: &axum::http::HeaderMap, method: &str, path: &str) -> Response {
    if path == "/api/status/power/usage" || path == "/status/power/usage" {
        return power_usage_response(method, path);
    }
    if path == "/api/admin/logs/homeserver" {
        return homeserver_logs_hyalos_response(headers, method, path);
    }
    let topic = match path {
        "/api/status/services" => Some("services.status"),
        "/api/status/tailscale" => Some("tailscale.status"),
        "/api/status/vpn/pia" | "/api/status/vpn/transmission" => Some("vpn.status"),
        _ => None,
    };
    if let Some(topic) = topic {
        return match collect_indicator_topic(topic, session_from_headers(headers)) {
            Ok(snapshot) => (StatusCode::OK, Json(snapshot)).into_response(),
            Err(fault) => (StatusCode::SERVICE_UNAVAILABLE, Json(serde_json::json!({
                "schema": "coronatio.indicator.topic.fault.v1",
                "ok": false,
                "success": false,
                "topicId": topic,
                "path": path,
                "firstMissingSignal": fault,
            }))).into_response(),
        };
    }

    match session_from_headers(headers) {
        Session::Admin => (
            StatusCode::OK,
            Json(serde_json::json!({
                "schema": "coronatio.homeserver.route.read.v1",
                "ok": true,
                "success": true,
                "method": method,
                "path": path,
                "family": homeserver_route_family(path),
                "status": "rust-route",
                "authority": "Coronatio Rust route",
                "firstMissingSignal": "none"
            })),
        ).into_response(),
        Session::Guest => (StatusCode::OK, Json(HomeserverReadGuestResponse {
            schema: "coronatio.homeserver.route.read.guest.v1", ok: true, success: true, status: "rust-route", first_missing_signal: "none",
        })).into_response(),
    }
}

