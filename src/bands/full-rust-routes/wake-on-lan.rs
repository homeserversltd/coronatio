#[derive(Deserialize)]
struct WakeOnLanStatusQuery {
    ip: Option<String>,
}

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

fn wake_on_lan_staff_read(headers: &axum::http::HeaderMap, ip: &str) -> CaduceusHttpReadback {
    caduceus_actuate_json(
        &mutation_authority(),
        headers,
        MutationActionTarget::caduceus("wake-on-lan.probe", "/api/admin/wake-on-lan/probe"),
        "/api/admin/wake-on-lan/probe",
        serde_json::json!({ "ip": ip }),
    )
}
async fn wake_on_lan_devices_route() -> Response {
    let leases = caduceus_http("GET", "/api/v1/network/dhcp/leases");
    let reservations = caduceus_http("GET", "/api/v1/network/dhcp/reservations");
    if leases.ok && reservations.ok {
        return (
            StatusCode::OK,
            Json(serde_json::json!({
                "ok": true,
                "leases": device_identity_payload(leases.body),
                "reservations": device_identity_payload(reservations.body),
            })),
        )
            .into_response();
    }
    let signal = if !leases.ok {
        leases.first_missing_signal
    } else {
        reservations.first_missing_signal
    };
    wake_on_lan_error(
        "/api/wakeonlan/devices",
        "DHCP device read is unavailable",
        &signal,
        None,
    )
}

async fn wake_on_lan_wake_route(
    headers: axum::http::HeaderMap,
    payload: Option<Json<serde_json::Value>>,
) -> Response {
    let path = "/api/wakeonlan/wake";
    let payload = payload
        .map(|Json(value)| value)
        .unwrap_or_else(|| serde_json::json!({}));
    let mac = payload
        .get("mac")
        .and_then(serde_json::Value::as_str)
        .map(str::trim)
        .unwrap_or("");
    let ip = payload
        .get("ip")
        .and_then(serde_json::Value::as_str)
        .map(str::trim)
        .unwrap_or("");
    if mac.is_empty() || ip.is_empty() {
        return wake_on_lan_error(
            path,
            "mac and ip are required",
            "wake-on-lan-target-required",
            Some(ip),
        );
    }
    let broadcast = payload
        .get("broadcast")
        .and_then(serde_json::Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
        .unwrap_or_else(|| wake_on_lan_broadcast(ip));
    let readback = caduceus_actuate_json(
        &mutation_authority(),
        &headers,
        MutationActionTarget::caduceus("wake-on-lan.send", "/api/admin/wake-on-lan/send"),
        "/api/admin/wake-on-lan/send",
        serde_json::json!({ "mac": mac, "broadcast": broadcast }),
    );
    if readback.ok {
        return (StatusCode::OK, Json(readback.body)).into_response();
    }
    wake_on_lan_error(
        path,
        "Wake-on-LAN staff actuator is unavailable",
        &readback.first_missing_signal,
        Some(ip),
    )
}

async fn wake_on_lan_status_route(
    headers: axum::http::HeaderMap,
    Query(query): Query<WakeOnLanStatusQuery>,
) -> Response {
    let path = "/api/wakeonlan/status";
    let ip = query.ip.as_deref().map(str::trim).unwrap_or("");
    if ip.is_empty() {
        return wake_on_lan_error(path, "ip is required", "wake-on-lan-ip-required", None);
    }
    let readback = wake_on_lan_staff_read(&headers, ip);
    if readback.ok {
        return (StatusCode::OK, Json(readback.body)).into_response();
    }
    wake_on_lan_error(
        path,
        "Wake-on-LAN staff actuator is unavailable",
        &readback.first_missing_signal,
        Some(ip),
    )
}
