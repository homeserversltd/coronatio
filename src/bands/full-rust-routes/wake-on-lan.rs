fn wake_on_lan_error(path: &str, error: &str, signal: &str, ip: Option<&str>) -> Response {
    (
        StatusCode::SERVICE_UNAVAILABLE,
        Json(serde_json::json!({
            "schema": "coronatio.wakeonlan.staff.receipt.v1",
            "ok": false,
            "path": path,
            "ip": ip,
            "awake": false,
            "rtt_ms": serde_json::Value::Null,
            "error": error,
            "firstMissingSignal": signal,
        })),
    )
        .into_response()
}

fn wake_on_lan_broadcast(ip: &str) -> String {
    ip.parse::<std::net::Ipv4Addr>()
        .map(|address| {
            let mut octets = address.octets();
            octets[3] = 255;
            std::net::Ipv4Addr::from(octets).to_string()
        })
        .unwrap_or_else(|_| "255.255.255.255".to_string())
}

fn wake_on_lan_fragment(sent: bool, signal: &str) -> Response {
    let result = if sent { "sent" } else { "not-sent" };
    let text = if sent { "Sent" } else { "Not sent" };
    Html(maud::html! { span data-wake-result=(result) data-wake-signal=(signal) { (text) } }.into_string()).into_response()
}

async fn wake_on_lan_wake_route(request: axum::extract::Request) -> Response {
    use axum::extract::FromRequest;
    let headers = request.headers().clone();
    let htmx = headers.get("HX-Request").and_then(|v| v.to_str().ok())
        .is_some_and(|v| v.eq_ignore_ascii_case("true"));
    let is_form = headers.get(header::CONTENT_TYPE).and_then(|v| v.to_str().ok())
        .is_some_and(|v| v.split(';').next().unwrap_or("").trim().eq_ignore_ascii_case("application/x-www-form-urlencoded"));
    let payload = if is_form {
        Form::<BTreeMap<String, String>>::from_request(request, &()).await.ok()
            .and_then(|Form(value)| serde_json::to_value(value).ok())
    } else {
        Json::<serde_json::Value>::from_request(request, &()).await.ok().map(|Json(value)| value)
    }.unwrap_or_else(|| serde_json::json!({}));
    let mac = payload.get("mac").and_then(serde_json::Value::as_str).map(str::trim).unwrap_or("");
    let ip = payload.get("ip").and_then(serde_json::Value::as_str).map(str::trim).unwrap_or("");
    if mac.is_empty() {
        let signal = "wake-on-lan-target-required";
        return if htmx { wake_on_lan_fragment(false, signal) } else {
            wake_on_lan_error("/api/wakeonlan/wake", "mac is required", signal, Some(ip))
        };
    }
    let readback = caduceus_actuate_json(
        &mutation_authority(), &headers, "/api/v1/network/device/wake",
        serde_json::json!({ "action": "send", "mac": mac, "broadcast": wake_on_lan_broadcast(ip) }),
    );
    if htmx {
        return wake_on_lan_fragment(readback.ok, if readback.ok { "" } else { &readback.first_missing_signal });
    }
    if readback.ok {
        return (StatusCode::OK, Json(readback.body)).into_response();
    }
    wake_on_lan_error("/api/wakeonlan/wake", "Wake-on-LAN staff actuator is unavailable", &readback.first_missing_signal, Some(ip))
}
