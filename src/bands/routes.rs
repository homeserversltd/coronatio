async fn crown_shell_route() -> impl IntoResponse {
    Html(render_crown_shell())
}

async fn health_route() -> impl IntoResponse {
    Json(serde_json::json!({
        "ok": true,
        "service": "coronatio",
        "schema": "coronatio.health.v1"
    }))
}

async fn api_root_route(State(state): State<AppState>) -> impl IntoResponse {
    Json(CoronatioRoot {
        schema: "coronatio.api.root.v1".to_string(),
        kind: "coronatio-root".to_string(),
        product: "Coronatio".to_string(),
        routes: vec![
            "/".to_string(),
            "/health".to_string(),
            "/api".to_string(),
            "/api/panes".to_string(),
            "/api/panes/:pane_id".to_string(),
            "/api/registry".to_string(),
            "/api/registry/transaction".to_string(),
            "/api/startup".to_string(),
            "/api/lanes".to_string(),
            "/api/fallback".to_string(),
            "/api/session".to_string(),
            "/api/validatePin".to_string(),
            "/api/verifyPin".to_string(),
            "/api/logout".to_string(),
            "/api/admin/session".to_string(),
            "/api/caduceus/status".to_string(),
            "/api/caduceus/update/check".to_string(),
            "/api/caduceus/update/now".to_string(),
            "/api/caduceus/receipts/latest".to_string(),
            "/api/topics".to_string(),
            "/api/monitor/pulse".to_string(),
            "/api/services/data".to_string(),
            "/api/frontend/storage".to_string(),
            "/api/boundary".to_string(),
            "/api/installer".to_string(),
            "/api/stats/events".to_string(),
            "/api/stats/events/renew".to_string(),
            "/api/stats".to_string(),
            "/api/coronatio/tabs".to_string(),
            "/api/coronatio/tabs/:tab_id/manifest".to_string(),
            "/api/tabs (legacy HomeServer proxy)".to_string(),
            "/api/*path (legacy HomeServer proxy)".to_string(),
            "/assets/*path (legacy HomeServer build assets)".to_string(),
            "/socket.io/*path (legacy HomeServer proxy)".to_string(),
            "/tabs/<tab-id>/static/...".to_string(),
        ],
        tab_root: state.tab_root.display().to_string(),
        primary_tabs: PRIMARY_TABS.iter().map(|tab| (*tab).to_string()).collect(),
        first_party_panes: native_crown_panes(),
    })
}

async fn panes_route() -> impl IntoResponse {
    Json(serde_json::json!({
        "schema": "coronatio.panes.v1",
        "product": "Coronatio",
        "panes": native_crown_panes()
    }))
}

async fn stats_route() -> impl IntoResponse {
    Json(stats_snapshot())
}

async fn registry_route() -> impl IntoResponse {
    Json(registry_readback())
}

async fn registry_transaction_route() -> impl IntoResponse {
    Json(registry_transaction_readback())
}

async fn startup_route() -> impl IntoResponse {
    Json(startup_readback())
}

async fn lane_policy_route() -> impl IntoResponse {
    Json(lane_policy_readback())
}

async fn fallback_route() -> impl IntoResponse {
    Json(fallback_readback())
}

async fn session_route() -> impl IntoResponse {
    Json(admin_session_readback())
}

async fn session_renew_route() -> impl IntoResponse {
    Json(serde_json::json!({
        "schema": "coronatio.admin.session.renewal.v1",
        "status": "contract-only",
        "leaseSeconds": 1800,
        "authority": "Caduceus must mint or refresh privileged mutation capability before live mutation is enabled"
    }))
}

async fn validate_pin_route(Json(request): Json<ValidatePinRequest>) -> impl IntoResponse {
    match homeserver_admin_pin() {
        Ok(expected_pin) if request.pin == expected_pin => Json(serde_json::json!({
            "success": true,
            "token": mint_admin_token(),
            "sessionTimeout": ADMIN_SESSION_TIMEOUT_SECONDS,
            "schema": "coronatio.admin.pin.validation.v1",
            "configSource": homeserver_config_path().display().to_string()
        }))
        .into_response(),
        Ok(_) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "success": false,
                "schema": "coronatio.admin.pin.validation.v1",
                "error": "invalid-pin",
                "configSource": homeserver_config_path().display().to_string()
            })),
        )
            .into_response(),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "success": false,
                "schema": "coronatio.admin.pin.validation.v1",
                "error": "pin-config-unavailable",
                "detail": error,
                "configSource": homeserver_config_path().display().to_string()
            })),
        )
            .into_response(),
    }
}

async fn logout_route() -> impl IntoResponse {
    Json(serde_json::json!({
        "success": true,
        "schema": "coronatio.admin.logout.v1"
    }))
}

fn homeserver_config_path() -> PathBuf {
    env::var("CORONATIO_HOMESERVER_CONFIG")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(DEFAULT_HOMESERVER_CONFIG))
}

fn homeserver_admin_pin() -> Result<String, String> {
    let path = homeserver_config_path();
    let raw = std::fs::read_to_string(&path)
        .map_err(|error| format!("read {}: {error}", path.display()))?;
    let config: serde_json::Value =
        serde_json::from_str(&raw).map_err(|error| format!("parse {}: {error}", path.display()))?;
    config
        .pointer("/global/admin/pin")
        .and_then(|value| value.as_str())
        .filter(|pin| !pin.is_empty())
        .map(ToString::to_string)
        .ok_or_else(|| format!("{} missing global.admin.pin", path.display()))
}

fn homeserver_theme_name() -> String {
    let path = homeserver_config_path();
    std::fs::read_to_string(&path)
        .ok()
        .and_then(|raw| serde_json::from_str::<serde_json::Value>(&raw).ok())
        .and_then(|config| {
            config
                .pointer("/global/theme/name")
                .and_then(|value| value.as_str())
                .map(ToString::to_string)
        })
        .unwrap_or_else(|| "dark".to_string())
}

fn first_paint_theme() -> (&'static str, &'static str, &'static str, &'static str) {
    match homeserver_theme_name().as_str() {
        "light" => ("#f8fafc", "#2563eb", "#334155", "#e2e8f0"),
        _ => ("#1f2937", "#3b82f6", "#e5e7eb", "#374151"),
    }
}

fn mint_admin_token() -> String {
    let mut random = [0u8; 24];
    if let Ok(mut file) = std::fs::File::open("/dev/urandom") {
        let _ = file.read_exact(&mut random);
    }
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let mut token = format!("coronatio-{now:x}-{:x}-", std::process::id());
    for byte in random {
        token.push_str(&format!("{byte:02x}"));
    }
    token
}

async fn caduceus_status_route() -> impl IntoResponse {
    let health = caduceus_http("GET", "/api/v1/health");
    let update = caduceus_http("GET", "/api/v1/update/status");
    let staff = caduceus_http("GET", "/api/v1/staff/status");
    let ok = health.ok && update.ok && staff.ok;
    let first_missing_signal = if ok {
        "none".to_string()
    } else if !health.ok {
        health.first_missing_signal.clone()
    } else if !update.ok {
        update.first_missing_signal.clone()
    } else {
        staff.first_missing_signal.clone()
    };
    (
        if ok {
            StatusCode::OK
        } else {
            StatusCode::SERVICE_UNAVAILABLE
        },
        Json(serde_json::json!({
            "schema": "coronatio.caduceus.status.v1",
            "ok": ok,
            "caduceusBase": caduceus_base(),
            "health": health,
            "update": update,
            "staff": staff,
            "firstMissingSignal": first_missing_signal
        })),
    )
}

async fn caduceus_update_check_route() -> impl IntoResponse {
    caduceus_mutation_route("update_check", "/api/v1/update/check")
}

async fn caduceus_update_now_route() -> impl IntoResponse {
    caduceus_dispatch_route("update_now", "/api/v1/update/now")
}

async fn caduceus_receipts_latest_route() -> impl IntoResponse {
    let readback = caduceus_http("GET", "/api/v1/receipts/latest");
    (
        if readback.ok {
            StatusCode::OK
        } else {
            StatusCode::SERVICE_UNAVAILABLE
        },
        Json(serde_json::json!({
            "schema": "coronatio.caduceus.receipts.latest.v1",
            "ok": readback.ok,
            "readback": readback,
            "firstMissingSignal": readback.first_missing_signal
        })),
    )
}

fn caduceus_dispatch_route(
    route: &'static str,
    path: &'static str,
) -> (StatusCode, Json<serde_json::Value>) {
    thread::spawn(move || {
        let _ = caduceus_http("POST", path);
    });
    (
        StatusCode::ACCEPTED,
        Json(serde_json::json!({
            "schema": "coronatio.caduceus.dispatch.v1",
            "ok": true,
            "route": route,
            "accepted": true,
            "path": path,
            "firstMissingSignal": "none"
        })),
    )
}

fn caduceus_mutation_route(route: &str, path: &str) -> (StatusCode, Json<serde_json::Value>) {
    let readback = caduceus_http("POST", path);
    (
        if readback.ok {
            StatusCode::OK
        } else {
            StatusCode::SERVICE_UNAVAILABLE
        },
        Json(serde_json::json!({
            "schema": "coronatio.caduceus.mutation.v1",
            "ok": readback.ok,
            "route": route,
            "readback": readback,
            "firstMissingSignal": readback.first_missing_signal
        })),
    )
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct CaduceusHttpReadback {
    ok: bool,
    status: u16,
    path: String,
    body: serde_json::Value,
    first_missing_signal: String,
}

fn caduceus_base() -> String {
    env::var("CADUCEUS_URL").unwrap_or_else(|_| "http://127.0.0.1:3014".to_string())
}

fn caduceus_authority() -> (String, String) {
    let base = caduceus_base();
    let without_scheme = base
        .strip_prefix("http://")
        .or_else(|| base.strip_prefix("https://"))
        .unwrap_or(base.as_str());
    let authority = without_scheme.trim_end_matches('/').to_string();
    (base, authority)
}

fn caduceus_http(method: &str, path: &str) -> CaduceusHttpReadback {
    let (_base, authority) = caduceus_authority();
    let mut stream = match TcpStream::connect(&authority) {
        Ok(stream) => stream,
        Err(err) => {
            return CaduceusHttpReadback {
                ok: false,
                status: 0,
                path: path.to_string(),
                body: serde_json::json!({"error": err.to_string()}),
                first_missing_signal: "caduceus-unreachable".to_string(),
            };
        }
    };
    let _ = stream.set_read_timeout(Some(Duration::from_secs(4)));
    let _ = stream.set_write_timeout(Some(Duration::from_secs(4)));
    let request = format!(
        "{method} {path} HTTP/1.1\r\nHost: {authority}\r\nConnection: close\r\nContent-Length: 0\r\n\r\n"
    );
    if let Err(err) = stream.write_all(request.as_bytes()) {
        return CaduceusHttpReadback {
            ok: false,
            status: 0,
            path: path.to_string(),
            body: serde_json::json!({"error": err.to_string()}),
            first_missing_signal: "caduceus-write-failed".to_string(),
        };
    }
    let mut response = String::new();
    if let Err(err) = stream.read_to_string(&mut response) {
        return CaduceusHttpReadback {
            ok: false,
            status: 0,
            path: path.to_string(),
            body: serde_json::json!({"error": err.to_string()}),
            first_missing_signal: "caduceus-read-failed".to_string(),
        };
    }
    let (head, body_text) = response
        .split_once("\r\n\r\n")
        .unwrap_or(("", response.as_str()));
    let status = head
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .and_then(|raw| raw.parse::<u16>().ok())
        .unwrap_or(0);
    let body =
        serde_json::from_str(body_text).unwrap_or_else(|_| serde_json::json!({"raw": body_text}));
    let body_ok = body
        .get("ok")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(status < 400);
    let first_missing_signal = body
        .get("firstMissingSignal")
        .or_else(|| body.get("first_missing_signal"))
        .and_then(serde_json::Value::as_str)
        .unwrap_or(if status < 400 && body_ok {
            "none"
        } else {
            "caduceus-http-not-ok"
        })
        .to_string();
    CaduceusHttpReadback {
        ok: status < 400 && body_ok,
        status,
        path: path.to_string(),
        body,
        first_missing_signal,
    }
}

async fn topics_route() -> impl IntoResponse {
    Json(topic_catalog_readback())
}

async fn monitor_pulse_route() -> impl IntoResponse {
    Json(monitor_pulse_readback())
}

async fn service_data_route() -> impl IntoResponse {
    Json(service_data_readback())
}

async fn frontend_storage_route() -> impl IntoResponse {
    Json(frontend_storage_readback())
}

async fn boundary_route() -> impl IntoResponse {
    Json(boundary_readback())
}

async fn installer_route() -> impl IntoResponse {
    Json(installer_readback())
}

async fn stats_events_route() -> impl IntoResponse {
    let event = stats_event_payload();
    let payload = serde_json::to_string(&event).expect("serialize stats event");
    let body = format!(
        "event: stats.system\nid: {}\ndata: {}\n\n",
        event.event_id, payload
    );
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/event-stream")
        .header(header::CACHE_CONTROL, "no-cache")
        .header("x-coronatio-schema", "coronatio.stats.events.v1")
        .body(body)
        .expect("build stats event response")
}

async fn stats_events_renew_route() -> impl IntoResponse {
    Json(LeaseRenewalReadback {
        schema: "coronatio.stats.events.renewal.v1".to_string(),
        topic: "stats.system".to_string(),
        route: "/api/stats/events/renew".to_string(),
        lease_seconds: 30,
        status: "renewed-contract".to_string(),
        next_renewal_before_seconds: 20,
    })
}

async fn legacy_homeserver_react_bundle_route() -> impl IntoResponse {
    let path = legacy_homeserver_asset_root().join("index-BRoXzIjg.js");
    match fs::read_to_string(&path).await {
        Ok(bundle) => {
            let patched = bundle.replace(
                "transports:[\"websocket\",\"polling\"]",
                "transports:[\"polling\"]",
            );
            (
                StatusCode::OK,
                [(
                    header::CONTENT_TYPE,
                    "application/javascript; charset=utf-8",
                )],
                patched,
            )
                .into_response()
        }
        Err(error) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "schema": "coronatio.legacy-homeserver-bundle.error.v1",
                "ok": false,
                "path": path.display().to_string(),
                "error": error.to_string()
            })),
        )
            .into_response(),
    }
}

