async fn power_usage_route(headers: axum::http::HeaderMap, method: Method, uri: Uri) -> Response {
    let method = method.to_string();
    power_usage_response(&headers, &method, uri.path()).await
}

async fn power_usage_response(headers: &axum::http::HeaderMap, method: &str, path: &str) -> Response {
    let (_, sample) = indicators::collect_indicator_topic_for_request("power.status", headers).await;
    let unavailable = sample.unavailable_signal;
    let mut body = sample.snapshot;
    if let Some(object) = body.as_object_mut() {
        object.insert("schema".to_owned(), serde_json::Value::String("coronatio.power.usage.v1".to_owned()));
        object.insert("ok".to_owned(), serde_json::Value::Bool(unavailable.is_none()));
        object.insert("success".to_owned(), serde_json::Value::Bool(unavailable.is_none()));
        object.insert("method".to_owned(), serde_json::Value::String(method.to_owned()));
        object.insert("path".to_owned(), serde_json::Value::String(path.to_owned()));
        object.insert("family".to_owned(), serde_json::Value::String("power-monitor".to_owned()));
        object.insert("authority".to_owned(), serde_json::Value::String("Caduceus appliance stats and raw stats history".to_owned()));
        let has_signal = object.get("firstMissingSignal")
            .and_then(serde_json::Value::as_str)
            .is_some_and(|signal| !signal.is_empty() && signal != "none");
        if !has_signal {
            object.insert(
                "firstMissingSignal".to_owned(),
                serde_json::Value::String(unavailable.clone().unwrap_or_else(|| "none".to_owned())),
            );
        }
    } else {
        return power_usage_unavailable_response(method, path, "caduceus-stats-power-projection-invalid");
    }
    (
        if unavailable.is_some() { StatusCode::SERVICE_UNAVAILABLE } else { StatusCode::OK },
        Json(body),
    ).into_response()
}

fn power_usage_unavailable_response(method: &str, path: &str, signal: &str) -> Response {
    (
        StatusCode::SERVICE_UNAVAILABLE,
        Json(serde_json::json!({
            "schema":"coronatio.power.usage.v1","ok":false,"success":false,
            "method":method,"path":path,"family":"power-monitor",
            "authority":"Caduceus appliance stats and raw stats history",
            "current":null,"historical":[],"averages":{"5s":null,"30s":null,"60s":null},
            "calibration":1.0,"unit":"W","timestamp":null,"status":"unavailable",
            "firstMissingSignal":signal
        })),
    ).into_response()
}

async fn power_calibration_get_route() -> Response {
    let result = tokio::task::spawn_blocking(indicators::caduceus_power_calibration).await;
    let (factor, signal) = result.unwrap_or_else(|_| (1.0, Some("power-calibration-config-read-failed".to_owned())));
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "schema":"coronatio.power.calibration.v1","ok":signal.is_none(),
            "factor":factor,"minimum":0.1,"maximum":10.0,
            "firstMissingSignal":signal.unwrap_or_else(|| "none".to_owned())
        })),
    ).into_response()
}

async fn power_calibration_post_route(headers: axum::http::HeaderMap, body: Bytes) -> Response {
    if session_projection_from_headers(&headers).await != Session::Admin {
        return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({
            "schema":"coronatio.power.calibration.v1","ok":false,"accepted":false,
            "firstMissingSignal":"admin-session-required"
        }))).into_response();
    }
    let payload = match serde_json::from_slice::<serde_json::Value>(&body) {
        Ok(value) => value,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(serde_json::json!({
            "schema":"coronatio.power.calibration.v1","ok":false,"accepted":false,
            "firstMissingSignal":"power-calibration-payload-invalid"
        }))).into_response(),
    };
    let factor = payload.get("factor").and_then(serde_json::Value::as_f64);
    let Some(factor) = factor.filter(|value| value.is_finite() && (0.1..=10.0).contains(value)) else {
        return (StatusCode::BAD_REQUEST, Json(serde_json::json!({
            "schema":"coronatio.power.calibration.v1","ok":false,"accepted":false,
            "firstMissingSignal":"power-calibration-factor-out-of-range"
        }))).into_response();
    };
    let request_headers = headers.clone();
    // The config-set door receives the viewer's validated document attendance unchanged.
    let readback = tokio::task::spawn_blocking(move || caduceus_actuate_json_with_timeout(
        &mutation_authority(), &request_headers,
        MutationActionTarget::attended_parent("coronatio.power.calibration", "/api/v1/config/set"),
        "/api/v1/config/set",
        serde_json::json!({"path": "global.power.calibration", "value": factor}),
        std::time::Duration::from_secs(30),
    )).await;
    let readback = match readback {
        Ok(readback) => readback,
        Err(_) => return (StatusCode::SERVICE_UNAVAILABLE, Json(serde_json::json!({
            "schema":"coronatio.power.calibration.v1","ok":false,"accepted":false,
            "firstMissingSignal":"caduceus-config-set-task-failed"
        }))).into_response(),
    };
    if readback.ok { indicators::invalidate_indicator_household_config(); }
    let status = caduceus_source_status(&readback);
    (status, Json(readback.body)).into_response()
}
