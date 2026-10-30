async fn crown_shell_route() -> Response {
    let rendered = tokio::task::spawn_blocking(|| match face_stage() {
        FaceStage::Vault => render_stage_document(),
        FaceStage::Face => render_crown_shell(),
    })
    .await;
    match rendered {
        Ok(markup) => face_stage_html_response(markup),
        Err(_) => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({ "code": "face-stage-render-unavailable" })),
        )
            .into_response(),
    }
}

async fn crown_htmx_script_route() -> impl IntoResponse {
    ([(header::CONTENT_TYPE, "application/javascript; charset=utf-8")], CROWN_HTMX_JS)
}

async fn crown_chrome_script_route() -> impl IntoResponse {
    ([(header::CONTENT_TYPE, "application/javascript; charset=utf-8")], crown_chrome_js())
}

const CORONATIO_SOURCE_SHA: &str = match option_env!("CORONATIO_SOURCE_SHA") {
    Some(value) => value,
    None => "",
};
const CORONATIO_BUILD_SHA: &str = match option_env!("CORONATIO_BUILD_SHA") {
    Some(value) => value,
    None => "",
};

fn valid_coronatio_sha(value: &str) -> bool {
    value.len() == 40 && value.bytes().all(|byte| matches!(byte, b'0'..=b'9' | b'a'..=b'f'))
}

fn health_response_for(source_sha: &str, build_sha: &str) -> (StatusCode, serde_json::Value) {
    let identity_ok = valid_coronatio_sha(source_sha) && source_sha == build_sha;
    let status = if identity_ok { StatusCode::OK } else { StatusCode::SERVICE_UNAVAILABLE };
    (status, serde_json::json!({
        "ok": identity_ok,
        "service": "coronatio",
        "schema": "coronatio.health.v1",
        "source_sha": source_sha,
        "build_sha": build_sha,
    }))
}

async fn health_route() -> impl IntoResponse {
    let (status, payload) = health_response_for(CORONATIO_SOURCE_SHA, CORONATIO_BUILD_SHA);
    (status, Json(payload))
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
            "/api/attendance".to_string(),
            "/api/caduceus/status".to_string(),
            "/api/caduceus/update/check".to_string(),
            "/api/caduceus/update/now".to_string(),
            "/api/caduceus/update/service/status".to_string(),
            "/api/caduceus/update/service/toggle".to_string(),
            "/api/caduceus/update/status".to_string(),
            "/api/caduceus/update/modules".to_string(),
            "/api/caduceus/update/modules/:id".to_string(),
            "/api/caduceus/update/history".to_string(),
            "/api/caduceus/interactables".to_string(),
            "/api/caduceus/interactables/:id/:action".to_string(),
            "/api/caduceus/receipts/latest".to_string(),
            "/api/topics".to_string(),
            "/api/monitor/pulse".to_string(),
            "/api/services/data".to_string(),
            "/api/frontend/storage".to_string(),
            "/api/themes".to_string(),
            "/api/favorites".to_string(),
            "/api/get_starred_tab".to_string(),
            "/api/set_starred_tab".to_string(),
            "/api/boundary".to_string(),
            "/api/installer".to_string(),
            "/api/core/pulse".to_string(),
            "/api/core/pulse/renew".to_string(),
            "/api/stats/pulse".to_string(),
            "/api/stats/pulse/renew".to_string(),
            "/api/stats".to_string(),
            "/api/stats/current".to_string(),
            "/api/stats/roster".to_string(),
            "/api/stats/history".to_string(),

            "/api/indicators/admin".to_string(),
            "/api/faults".to_string(),
            "/admit/:tab_id".to_string(),
            "/admit/admin/toggle/:toggle_id".to_string(),
            "/admit/admin/action/:action_id".to_string(),
            "/admit/upload/tree".to_string(),
            "/api/files/browse-hierarchical".to_string(),
            "/api/tabs".to_string(),
            "/api/tabs/:tab_id/manifest".to_string(),
            "/static/vendor/htmx.min.js".to_string(),
            "/static/crown/chrome.js".to_string(),
            "/static/vendor/chart.umd.min.js".to_string(),
            "/static/vendor/chartjs-plugin-datalabels.min.js".to_string(),
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

async fn stats_history_route() -> impl IntoResponse { stats_history().await }

async fn stats_current_route() -> Response {
    let result = load_trace::spawn_blocking_with_trace(|| {
        let current = indicators::caduceus_stats_sample();
        let projections = indicators::completed_current_indicator_projections(current.clone());
        let has_completed_sample = current.sample_id.is_some();
        let available = current.ok || has_completed_sample;
        let sample = if available { current.body } else { serde_json::Value::Null };
        let first_missing_signal = if current.first_missing_signal.is_empty() || current.first_missing_signal == "none" {
            None
        } else {
            Some(current.first_missing_signal)
        };
        let payload = serde_json::json!({
            "ok": current.ok,
            "status": current.status,
            "sampleId": current.sample_id,
            "generation": current.generation,
            "sampledAt": current.sampled_at_unix_ms,
            "sampleAgeMs": current.age.map(|age| age.as_millis().min(u64::MAX as u128) as u64),
            "stale": current.stale || !current.ok,
            "firstMissingSignal": first_missing_signal,
            "sample": sample,
            "indicators": projections,
        });
        (if available { StatusCode::OK } else { StatusCode::SERVICE_UNAVAILABLE }, payload)
    }).await;
    match result {
        Ok((status, payload)) => (status, Json(payload)).into_response(),
        Err(_) => (StatusCode::SERVICE_UNAVAILABLE, Json(serde_json::json!({
            "ok": false,
            "stale": true,
            "firstMissingSignal": "stats-current-task-failed",
            "sampleId": null,
            "sampledAt": null,
            "sampleAgeMs": null,
            "sample": null,
            "indicators": null,
        }))).into_response(),
    }
}

async fn stats_roster_route() -> Response {
    let (identity_result, leases_result) = tokio::join!(
        load_trace::spawn_blocking_with_trace(|| stats_identity_roster_cached()),
        load_trace::spawn_blocking_with_trace(|| stats_kea_leases_cached()),
    );
    let identity = identity_result.ok();
    let leases = leases_result.ok();
    let identity_known = identity.as_ref().is_some_and(|(_, roster, _)| roster.status == "available");
    let identity_signal = identity.as_ref()
        .and_then(|(_, _, signal)| (!signal.trim().is_empty()).then(|| signal.clone()))
        .or_else(|| identity.is_none().then(|| "stats-roster-cache-task-failed".to_owned()));
    let leases_known = leases.as_ref().is_some_and(|readback| readback.sample_id.is_some());
    let leases_signal = leases.as_ref()
        .and_then(|readback| (!readback.first_missing_signal.trim().is_empty()).then(|| readback.first_missing_signal.clone()))
        .or_else(|| leases.as_ref().is_some_and(|readback| readback.stale).then(|| "stats-kea-leases-stale".to_owned()))
        .or_else(|| leases.is_none().then(|| "stats-kea-cache-task-failed".to_owned()));
    let identity_entries = identity.as_ref()
        .map(|(_, roster, _)| serde_json::to_value(&roster.entries).unwrap_or(serde_json::Value::Null))
        .unwrap_or(serde_json::Value::Null);
    let raw_lease_entries = leases.as_ref()
        .filter(|readback| readback.sample_id.is_some())
        .map(|readback| serde_json::to_value(&readback.entries).unwrap_or(serde_json::Value::Null))
        .unwrap_or(serde_json::Value::Null);
    let identity_age_ms = identity.as_ref().filter(|_| identity_known)
        .map(|(sampled_at, _, _)| sampled_at.elapsed().as_millis().min(u64::MAX as u128) as u64);
    let identity_sampled_at = identity_age_ms.map(|age| {
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default()
            .as_millis().min(u64::MAX as u128) as u64;
        now.saturating_sub(age)
    });
    let lease_age_ms = leases.as_ref().and_then(|readback| readback.sampled_at)
        .map(|at| at.elapsed().as_millis().min(u64::MAX as u128) as u64);
    let lease_sampled_at = lease_age_ms.map(|age| {
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default()
            .as_millis().min(u64::MAX as u128) as u64;
        now.saturating_sub(age)
    });
    let stale = identity_signal.is_some() || leases_signal.is_some() || !identity_known || !leases_known;
    let first_missing_signal = identity_signal.clone().or_else(|| leases_signal.clone());
    let status = if identity_known || leases_known { StatusCode::OK } else { StatusCode::SERVICE_UNAVAILABLE };
    let payload = serde_json::json!({
        "ok": !stale,
        "stale": stale,
        "firstMissingSignal": first_missing_signal,
        "identityRoster": identity_entries.clone(),
        "entries": identity_entries,
        "identityStatus": identity.as_ref().map(|(_, roster, _)| roster.status.as_str()).unwrap_or("unavailable"),
        "identitySampleId": null,
        "identitySampledAt": identity_sampled_at,
        "identitySampleAgeMs": identity_age_ms,
        "rawLeases": raw_lease_entries.clone(),
        "leases": raw_lease_entries,
        "rawLeasesSampleId": leases.as_ref().and_then(|readback| readback.sample_id.as_deref()),
        "rawLeasesGeneration": leases.as_ref().map(|readback| readback.generation).unwrap_or(0),
        "rawLeasesSampledAt": lease_sampled_at,
        "rawLeasesSampleAgeMs": lease_age_ms,
        "rawLeasesStale": leases.as_ref().map(|readback| readback.stale).unwrap_or(true),
        "rawLeasesFirstMissingSignal": leases_signal,
        "lastMissedAt": leases.as_ref().and_then(|readback| readback.last_missed_at),
    });
    (status, Json(payload)).into_response()
}

async fn indicator_admin_route(headers: axum::http::HeaderMap) -> Response {
    if session_projection_from_headers(&headers).await != Session::Admin {
        return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({
            "ok": false,
            "firstMissingSignal": "caduceus-attendance-required",
            "sample": null,
            "sampleId": null,
        }))).into_response();
    }
    let Some(document) = crate::caduceus_access::document_incarnation_from_headers(&headers) else {
        return (StatusCode::BAD_REQUEST, Json(serde_json::json!({
            "ok": false,
            "firstMissingSignal": "caduceus-attendance-document-required",
            "sample": null,
            "sampleId": null,
        }))).into_response();
    };
    let Some(attendance) = crate::caduceus_access::attendance_from_headers(&headers) else {
        return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({
            "ok": false,
            "firstMissingSignal": "caduceus-attendance-required",
            "sample": null,
            "sampleId": null,
        }))).into_response();
    };
    let readback = load_trace::spawn_blocking_with_trace(move || {
        indicators::read_scoped_admin_stats_sample(&document, attendance)
    }).await;
    match readback {
        Ok(readback) => {
            let status = if readback.ok { StatusCode::OK } else {
                StatusCode::from_u16(readback.status).ok().filter(|status| status.is_client_error() || status.is_server_error())
                    .unwrap_or(StatusCode::BAD_GATEWAY)
            };
            (status, Json(serde_json::json!({
                "ok": readback.ok,
                "status": readback.status,
                "firstMissingSignal": readback.first_missing_signal,
                "stale": readback.stale,
                "sample": if readback.ok { Some(readback.body) } else { None },
                "sampleId": readback.sample_id,
                "generation": readback.generation,
                "sampledAt": readback.sampled_at_unix_ms,
                "sampleAgeMs": readback.age.map(|age| age.as_millis().min(u64::MAX as u128) as u64),
            }))).into_response()
        }
        Err(_) => (StatusCode::SERVICE_UNAVAILABLE, Json(serde_json::json!({
            "ok": false,
            "firstMissingSignal": "caduceus-attendance-task-failed",
            "sample": null,
            "sampleId": null,
        }))).into_response(),
    }
}

async fn stats_route(headers: axum::http::HeaderMap) -> Response {
    let raw = stats_snapshot().await;
    match session_projection_from_headers(&headers).await {
        Session::Admin => Json(project_system_stats_admin(&raw)).into_response(),
        Session::Guest => {
            let facts = load_iris_facts_cached_status_sync();
            Json(project_system_stats_guest(&raw, &facts)).into_response()
        }
    }
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



async fn favorites_route() -> impl IntoResponse {
    match load_favorite_manifest().await {
        Ok((source, manifest)) => Json(FavoriteManifestResponse {
            schema: "coronatio.favorite-manifest.response.v1".to_string(),
            source,
            starred_tab: manifest.starred_tab,
            source_quarry: manifest.source_quarry,
            tabs: manifest.tabs,
            first_load_law: "first load is a one-shot ladder: lawful URL hash, browser-local non-admin favorite, appliance default, then lockdown; the appliance value is reported without reassignment".to_string(),
        })
        .into_response(),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "schema": "coronatio.favorite-manifest.error.v1",
                "ok": false,
                "error": error,
                "expected": "homeserver.json tabs.starred",
            })),
        )
            .into_response(),
    }
}

async fn get_starred_tab_route() -> impl IntoResponse {
    match load_favorite_manifest().await {
        Ok((source, manifest)) => Json(StarredTabResponse {
            schema: "coronatio.starred-tab.response.v1".to_string(),
            success: true,
            starred_tab: manifest.starred_tab,
            source,
        })
        .into_response(),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "schema": "coronatio.starred-tab.error.v1",
                "success": false,
                "error": error,
            })),
        )
            .into_response(),
    }
}

async fn set_starred_tab_route(headers: axum::http::HeaderMap, Json(request): Json<SetStarredTabRequest>) -> impl IntoResponse {
    let requested = normalize_tab_id(&request.tab_name.or(request.tab).unwrap_or_default());
    let (source, facts) = match load_iris_facts().await {
        Ok(value) => value,
        Err(error) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"success": false, "error": error}))).into_response(),
    };
    if request.is_starred == Some(false) {
        return (StatusCode::BAD_REQUEST, Json(serde_json::json!({
            "schema": "coronatio.starred-tab.mutation.v1",
            "success": false,
            "error": "one-to-one port preserves one active favorite; choose another visible non-admin tab instead of clearing all favorites",
            "starred_tab": facts.starred,
            "source": source,
        }))).into_response();
    }
    let next = match iris::apply_star(&facts, &requested) {
        Ok(next) => next,
        Err(error) => return (StatusCode::BAD_REQUEST, Json(serde_json::json!({
            "schema": "coronatio.starred-tab.mutation.v1",
            "success": false,
            "error": format!("{:?}", error),
            "requested": requested,
            "starred_tab": facts.starred,
            "source": source,
        }))).into_response(),
    };
    let persisted = caduceus_guest_star_set(&headers, serde_json::Value::String(next.starred.clone()));
    if !persisted.ok {
        return caduceus_config_failure_response(persisted);
    }
    pulse::poke(pulse::PokeTopic::TabsChanged);
    tab_bar_html_response_from_facts(session_from_headers(&headers), &next)
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TabVisibilityRequest {
    tab: Option<String>,
    tab_id: Option<String>,
    id: Option<String>,
    visible: Option<bool>,
    visibility: Option<bool>,
}

async fn tab_visibility_route(headers: axum::http::HeaderMap, Json(request): Json<TabVisibilityRequest>) -> impl IntoResponse {
    if let Some(refusal) = mutation_context_refusal(&headers) {
        return caduceus_config_failure_response(mutation_refusal_readback("/api/v1/config/set", refusal));
    }
    let tab = normalize_tab_id(&request.tab.or(request.tab_id).or(request.id).unwrap_or_default());
    let visible = request.visible.or(request.visibility).unwrap_or(true);
    let (_source, facts) = match load_iris_facts().await {
        Ok(value) => value,
        Err(error) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"success": false, "error": error}))).into_response(),
    };
    let next = iris::apply_tab_visibility(&facts, &tab, visible);
    let visibility_path = format!("tabs.{tab}.visibility.tab");
    let persisted = caduceus_config_set(&headers, &visibility_path, serde_json::Value::Bool(visible));
    if !persisted.ok {
        return caduceus_config_failure_response(persisted);
    }
    if next.starred != facts.starred {
        let starred = caduceus_config_set(&headers, "tabs.starred", serde_json::Value::String(next.starred.clone()));
        if !starred.ok {
            return caduceus_config_failure_response(starred);
        }
    }
    pulse::poke(pulse::PokeTopic::TabsChanged);
    tab_bar_html_response_from_facts(Session::Admin, &next)
}


#[derive(Debug, Clone, Deserialize)]
struct TabBarFragmentQuery {
    active: Option<String>,
}

async fn tab_bar_fragment_route(
    headers: axum::http::HeaderMap,
    Query(query): Query<TabBarFragmentQuery>,
) -> impl IntoResponse {
    tab_bar_html_response_with_active(session_projection_from_headers(&headers).await, query.active.as_deref())
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct CartridgeMutationRequest {
    id: String,
    #[serde(default)]
    title: String,
    #[serde(default)]
    url: String,
    #[serde(default)]
    guest_class: String,
    #[serde(default)]
    admin_only: bool,
    #[serde(flatten)]
    _extra: BTreeMap<String, serde_json::Value>,
}

fn cartridge_proxy_response(readback: CaduceusHttpReadback) -> Response {
    let status = if readback.ok { StatusCode::OK } else { mutation_response_status(&readback) };
    (status, Json(serde_json::json!({"ok": readback.ok, "cartridges": readback.body, "firstMissingSignal": readback.first_missing_signal}))).into_response()
}

#[derive(Clone, Debug)]
struct CartridgeResolvedRoute {
    method: String,
    path: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CartridgeRouteFailure {
    Unmapped,
    Unavailable,
}

// Compatibility is deliberately confined to the already-live cartridge lane.
// A flat route advertisement supplies reachability, never attendance or admin rights.
fn resolve_cartridge_door(method: &str, route: &str) -> Result<CartridgeResolvedRoute, CartridgeRouteFailure> {
    let advertised = match (method, route) {
        ("GET", "/api/v1/cartridges") => "/api/v1/cartridges/list",
        ("POST", "/api/v1/cartridges/admit") => "/api/v1/cartridges/admit",
        ("POST", "/api/v1/cartridges/remove") => "/api/v1/cartridges/remove",
        _ => return Err(CartridgeRouteFailure::Unmapped),
    };
    let readback = caduceus_http("GET", "/api/v1/doors");
    if !readback.ok {
        return Err(CartridgeRouteFailure::Unavailable);
    }
    if readback.body.get("schema").and_then(|value| value.as_str()) != Some("caduceus.doors.readback.v1") {
        return Err(CartridgeRouteFailure::Unavailable);
    }
    let Some(routes) = readback.body.get("routes").and_then(|value| value.as_array()) else {
        return Err(CartridgeRouteFailure::Unavailable);
    };
    if !routes.iter().any(|value| value.as_str() == Some(advertised)) {
        return Err(CartridgeRouteFailure::Unmapped);
    }
    Ok(CartridgeResolvedRoute {
        method: method.to_string(),
        // Preserve the public GET transport after checking the current list advertisement.
        path: route.to_string(),
    })
}

async fn cartridges_read_proxy_route() -> Response {
    let route = "/api/v1/cartridges";
    let readback = match resolve_cartridge_door("GET", route) {
        Ok(door) => caduceus_http(&door.method, &door.path),
        Err(CartridgeRouteFailure::Unmapped) => mutation_refusal_readback(route, MutationRefusal { code: "coronatio-caduceus-door-unmapped".to_string(), status: 0 }),
        Err(CartridgeRouteFailure::Unavailable) => mutation_refusal_readback(route, MutationRefusal { code: "caduceus-doors-unavailable".to_string(), status: 0 }),
    };
    cartridge_proxy_response(readback)
}

async fn cartridges_admit_proxy_route(headers: axum::http::HeaderMap, Json(request): Json<CartridgeMutationRequest>) -> Response {
    cartridge_mutation_proxy_response(headers, "/api/v1/cartridges/admit", request, true)
}

async fn cartridges_remove_proxy_route(headers: axum::http::HeaderMap, Json(request): Json<CartridgeMutationRequest>) -> Response {
    cartridge_mutation_proxy_response(headers, "/api/v1/cartridges/remove", request, false)
}

fn cartridge_mutation_proxy_response(headers: axum::http::HeaderMap, route: &str, request: CartridgeMutationRequest, admitting: bool) -> Response {
    if session_from_headers(&headers) != Session::Admin {
        return cartridge_proxy_response(mutation_refusal_readback(route, MutationRefusal { code: "caduceus-attendance-required".to_string(), status: 401 }));
    }
    let id = if admitting { normalize_tab_id(&request.title) } else { normalize_tab_id(&request.id) };
    let valid = is_safe_tab_id(&id) && (!admitting || (!request.title.trim().is_empty() && safe_cartridge_url(&request.url).is_some() && request.guest_class == "iframe"));
    if !valid {
        return (StatusCode::BAD_REQUEST, Json(serde_json::json!({"ok": false, "firstMissingSignal": "invalid-cartridge-request"}))).into_response();
    }
    // Caduceus admits strict raw Json<Cartridge> / Json<CartridgeRemoveBody>
    // bodies here, not a caduceus.staff.v1 envelope. Coronatio may tolerate
    // flattened browser extras, but this seam never forwards them upstream.
    let mut body = serde_json::json!({"id": id});
    if admitting {
        body["title"] = serde_json::json!(request.title.trim());
        body["url"] = serde_json::json!(request.url);
        body["guest_class"] = serde_json::json!("iframe");
        body["admin_only"] = serde_json::json!(request.admin_only);
    }
    let context = MutationRequestContext::from_headers(&headers);
    let authority = mutation_authority();
    let attendance = match authority.authorize(&context) {
        Ok(attendance) => attendance,
        Err(refusal) => return cartridge_proxy_response(mutation_refusal_readback(route, refusal)),
    };
    let door = match resolve_cartridge_door("POST", route) {
        Ok(door) => door,
        Err(CartridgeRouteFailure::Unmapped) => return cartridge_proxy_response(mutation_refusal_readback(route, MutationRefusal { code: "coronatio-caduceus-door-unmapped".to_string(), status: 0 })),
        Err(CartridgeRouteFailure::Unavailable) => return cartridge_proxy_response(mutation_refusal_readback(route, MutationRefusal { code: "caduceus-doors-unavailable".to_string(), status: 0 })),
    };
    let readback = caduceus_http_json_with_attendance_and_document(
        &door.method,
        &door.path,
        body,
        Some(&attendance.proof),
        Some(&attendance.document),
    );
    cartridge_proxy_response(readback)
}

fn tab_bar_html_response_from_facts(session: Session, facts: &IrisFacts) -> Response {
    let body = render_plan_tabbar_projection_from_facts(facts, session, None, false);
    let mut response = (StatusCode::OK, Html(body)).into_response();
    response.headers_mut().insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    response.headers_mut().insert(header::CONTENT_SECURITY_POLICY, HeaderValue::from_static(CROWN_CONTENT_SECURITY_POLICY));
    response
}

fn tab_bar_html_response_with_active(session: Session, active: Option<&str>) -> Response {
    let body = render_plan_tabbar_fragment_with_active(session, active);
    let mut response = (StatusCode::OK, Html(body)).into_response();
    response.headers_mut().insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    response.headers_mut().insert(header::CONTENT_SECURITY_POLICY, HeaderValue::from_static(CROWN_CONTENT_SECURITY_POLICY));
    response
}

pub(crate) fn homeserver_json_path() -> PathBuf {
    env::var("CORONATIO_HOMESERVER_JSON")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(APPLIANCE_CONFIG_JSON))
}

pub(crate) fn load_homeserver_json_sync() -> Result<(String, serde_json::Value), String> {
    let path = homeserver_json_path();
    let raw = std::fs::read_to_string(&path)
        .map_err(|error| format!("homeserver.json unreadable at {}: {}", path.display(), error))?;
    let value: serde_json::Value = serde_json::from_str(&raw)
        .map_err(|error| format!("homeserver.json invalid at {}: {}", path.display(), error))?;
    Ok((path.display().to_string(), value))
}

async fn load_homeserver_json() -> Result<(String, serde_json::Value), String> {
    let path = homeserver_json_path();
    let raw = fs::read_to_string(&path)
        .await
        .map_err(|error| format!("homeserver.json unreadable at {}: {}", path.display(), error))?;
    let value: serde_json::Value = serde_json::from_str(&raw)
        .map_err(|error| format!("homeserver.json invalid at {}: {}", path.display(), error))?;
    Ok((path.display().to_string(), value))
}

const CARTRIDGE_REGISTRY_PATH: &str = "/etc/appliance/cartridges.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ApplianceCartridgeRegistry {
    schema: String,
    #[serde(default, alias = "rows")]
    cartridges: Vec<ApplianceCartridge>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ApplianceCartridge {
    id: String,
    title: String,
    url: String,
    guest_class: String,
    admin_only: bool,
}

fn cartridge_registry_path() -> PathBuf {
    env::var("CORONATIO_CARTRIDGE_REGISTRY")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(CARTRIDGE_REGISTRY_PATH))
}

fn safe_cartridge_url(raw: &str) -> Option<String> {
    let parsed = url::Url::parse(raw).ok()?;
    matches!(parsed.scheme(), "http" | "https").then(|| raw.to_string())
}

fn load_appliance_cartridges() -> Vec<ApplianceCartridge> {
    let Ok(raw) = std::fs::read_to_string(cartridge_registry_path()) else { return Vec::new(); };
    let Ok(registry) = serde_json::from_str::<ApplianceCartridgeRegistry>(&raw) else { return Vec::new(); };
    if registry.schema != "appliance.cartridges.v1" { return Vec::new(); }
    let mut seen = BTreeSet::new();
    registry.cartridges.into_iter().filter_map(|mut cartridge| {
        cartridge.id = normalize_tab_id(&cartridge.id);
        cartridge.title = cartridge.title.trim().to_string();
        cartridge.url = safe_cartridge_url(&cartridge.url)?;
        (is_safe_tab_id(&cartridge.id) && !cartridge.title.is_empty() && cartridge.guest_class == "iframe" && seen.insert(cartridge.id.clone())).then_some(cartridge)
    }).collect()
}

fn appliance_cartridge(tab_id: &str) -> Option<ApplianceCartridge> {
    load_appliance_cartridges().into_iter().find(|cartridge| cartridge.id == tab_id)
}

// Keep the last good register snapshot across failed reads and tab changes.
// Refreshes are serialized separately so pulse readers can consume held facts
// while the bounded Caduceus UDS read is in flight.
const XENIA_STATUS_TTL: Duration = Duration::from_secs(30);
#[derive(Default)]
struct XeniaStatusCache {
    last_good: Option<(std::time::Instant, serde_json::Value)>,
    stale: bool,
    invalidation_generation: u64,
}
static XENIA_STATUS_CACHE: OnceLock<Mutex<XeniaStatusCache>> = OnceLock::new();
static XENIA_STATUS_REFRESH: OnceLock<Mutex<()>> = OnceLock::new();

fn xenia_status_cache() -> &'static Mutex<XeniaStatusCache> {
    XENIA_STATUS_CACHE.get_or_init(|| Mutex::new(XeniaStatusCache::default()))
}

fn xenia_status_refresh_lock() -> &'static Mutex<()> {
    XENIA_STATUS_REFRESH.get_or_init(|| Mutex::new(()))
}

fn invalidate_xenia_status_cache() {
    let mut cache = xenia_status_cache().lock().unwrap_or_else(|error| error.into_inner());
    cache.stale = true;
    cache.invalidation_generation = cache.invalidation_generation.wrapping_add(1);
}

fn xenia_status(refresh: bool) -> serde_json::Value {
    if !refresh {
        return xenia_status_cache().lock().unwrap_or_else(|error| error.into_inner())
            .last_good.as_ref().map(|(_, value)| value.clone())
            .unwrap_or(serde_json::Value::Null);
    }

    // Coalesce concurrent pane entries across the bounded Caduceus UDS read.
    let _refresh = xenia_status_refresh_lock().lock().unwrap_or_else(|error| error.into_inner());
    let generation = {
        let cache = xenia_status_cache().lock().unwrap_or_else(|error| error.into_inner());
        if let Some((at, value)) = cache.last_good.as_ref() {
            if !cache.stale && at.elapsed() < XENIA_STATUS_TTL { return value.clone(); }
        }
        cache.invalidation_generation
    };

    let readback = caduceus_http("GET", "/api/v1/xenia/status");
    let mut cache = xenia_status_cache().lock().unwrap_or_else(|error| error.into_inner());
    if !readback.ok {
        cache.stale = true;
        eprintln!("{}", serde_json::json!({
            "event": "coronatio.xenia.status.refresh.failed",
            "method": "GET",
            "route": "/api/v1/xenia/status",
            "status": readback.status,
            "first_missing_signal": crate::caduceus_access::safe_access_code(&readback.first_missing_signal),
        }));
        return cache.last_good.as_ref().map(|(_, value)| value.clone())
            .unwrap_or(serde_json::Value::Null);
    }

    let value = readback.body;
    cache.last_good = Some((std::time::Instant::now(), value.clone()));
    cache.stale = cache.invalidation_generation != generation;
    value
}

fn registry_config_value() -> serde_json::Value {
    load_homeserver_json_sync().map(|(_, value)| value).unwrap_or_else(|_| serde_json::json!({}))
}

fn registry_starred(value: &serde_json::Value) -> &str {
    value.get("tabs").and_then(|tabs| tabs.get("starred")).and_then(serde_json::Value::as_str).unwrap_or("stats")
}

// One composition point; execution routing and discovery belong to R1.
fn build_tab_contracts(value: &serde_json::Value, status: &serde_json::Value) -> Vec<CoronatioTabContract> {
    let mut tabs = native_tab_contracts();
    let next_order = tabs.iter().map(|tab| tab.order).max().unwrap_or(0) + 1;
    for (offset, cartridge) in load_appliance_cartridges().into_iter().enumerate() {
        if tabs.iter().any(|tab| tab.id == cartridge.id) { continue; }
        tabs.push(CoronatioTabContract { id: cartridge.id.clone(), display_name: cartridge.title, order: next_order + offset as i64, enabled: true, admin_only: cartridge.admin_only, visibility: TabVisibility::default(), install_mode: InstallMode::DynamicCartridge, route: format!("/#{}", cartridge.id), state_route: format!("/admit/{}", cartridge.id), data: None, kind: None, client_class: None, transport: None, granted: None, installed: None, discovered: None, listeners: None, runtime: None, xenia_entry: None });
    }
    if let Some(tabs_obj) = value.get("tabs").and_then(serde_json::Value::as_object) {
        if let Some(xenoi) = status.get("xenoi").and_then(serde_json::Value::as_object) {
            for (id, entry) in xenoi {
                // Pending entries and dangling presentation rows are not guests.
                if !is_safe_tab_id(id) || !entry.is_object()
                    || entry.get("id").is_some_and(|entry_id| entry_id.as_str() != Some(id.as_str()))
                    || !tabs_obj.get(id).is_some_and(serde_json::Value::is_object)
                    || tabs.iter().any(|tab| &tab.id == id) { continue; }
                let runtime = status.get("runtime").and_then(|runtime| runtime.get(id)).cloned();
                tabs.push(CoronatioTabContract {
                    id: id.clone(), display_name: id.clone(), order: 100,
                    enabled: entry.get("enabled").and_then(serde_json::Value::as_bool).unwrap_or(true),
                    admin_only: false, visibility: TabVisibility::default(),
                    install_mode: InstallMode::DynamicCartridge,
                    route: format!("/#{id}"), state_route: format!("/admit/{id}"),
                    data: None,
                    kind: entry.get("kind").cloned(),
                    client_class: entry.get("client_class").cloned(),
                    transport: entry.get("transport").cloned(),
                    granted: entry.get("granted").cloned(),
                    installed: entry.get("installed").cloned(),
                    discovered: entry.get("discovered").cloned(),
                    listeners: runtime.as_ref().and_then(|runtime| runtime.get("listeners")).or_else(|| entry.get("listeners")).cloned(),
                    runtime,
                    // Unknown additive execution facts survive for R1.
                    xenia_entry: Some(entry.clone()),
                });
            }
        }
        for tab in tabs.iter_mut() {
            let Some(raw) = tabs_obj.get(&tab.id) else { continue; };
            if tab.xenia_entry.is_some() {
                let config = raw.get("config");
                let presentation = |key: &str| raw.get(key).or_else(|| config.and_then(|config| config.get(key)));
                if let Some(name) = presentation("displayName").and_then(serde_json::Value::as_str) { tab.display_name = name.to_string(); }
                if let Some(order) = presentation("order").and_then(serde_json::Value::as_i64) { tab.order = order; }
                if let Some(admin_only) = presentation("adminOnly").and_then(serde_json::Value::as_bool) { tab.admin_only = admin_only; }
                if let Some(enabled) = config.and_then(|config| config.get("isEnabled")).and_then(serde_json::Value::as_bool) { tab.enabled &= enabled; }
            } else if let Some(config) = raw.get("config").and_then(serde_json::Value::as_object) {
                // Keep native and cartridges.json presentation behavior unchanged.
                if let Some(name) = config.get("displayName").and_then(serde_json::Value::as_str) { tab.display_name = name.to_string(); }
                if let Some(enabled) = config.get("isEnabled").and_then(serde_json::Value::as_bool) { tab.enabled = enabled; }
                if let Some(admin_only) = config.get("adminOnly").and_then(serde_json::Value::as_bool) { tab.admin_only = admin_only; }
            }
            tab.data = raw.get("data").cloned();
            if tab.xenia_entry.is_some() && xenia_discovery::health_degraded(&tab.id) {
                match tab.installed.as_mut() {
                    Some(serde_json::Value::Object(installed)) => {
                        installed.insert("health".to_string(), serde_json::Value::String("degraded".to_string()));
                    }
                    Some(serde_json::Value::Null) | None => {
                        tab.installed = Some(serde_json::json!({"health": "degraded"}));
                    }
                    Some(_) => {}
                }
            }
            if let Some(visibility) = raw.get("visibility").and_then(serde_json::Value::as_object) {
                if let Some(visible) = visibility.get("tab").and_then(serde_json::Value::as_bool) { tab.visibility.tab = visible; }
                if let Some(elements) = visibility.get("elements").and_then(serde_json::Value::as_object) {
                    tab.visibility.elements = elements.iter().filter_map(|(id, v)| v.as_bool().map(|visible| { let canonical = if tab.id == "stats" { canonical_stats_element_id(id) } else { id.as_str() }; (canonical.to_string(), visible) })).collect();
                }
            }
        }
    }
    tabs.sort_by(|left, right| left.order.cmp(&right.order).then(left.id.cmp(&right.id)));
    tabs
}

fn favorite_manifest_from_homeserver(source: String, value: &serde_json::Value) -> Result<FavoriteManifest, String> {
    let tabs_obj = value
        .get("tabs")
        .and_then(serde_json::Value::as_object)
        .ok_or_else(|| "homeserver.json missing tabs object".to_string())?;
    let starred_tab = tabs_obj
        .get("starred")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("stats")
        .to_string();
    let mut tabs = Vec::new();
    for (id, tab) in tabs_obj {
        if id == "starred" || !is_safe_tab_id(id) {
            continue;
        }
        let config = tab.get("config").and_then(serde_json::Value::as_object);
        let visibility = tab.get("visibility").and_then(serde_json::Value::as_object);
        let visible = visibility
            .and_then(|v| v.get("tab"))
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(true);
        let enabled = config
            .and_then(|c| c.get("isEnabled"))
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(true);
        let admin_only = config
            .and_then(|c| c.get("adminOnly"))
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false);
        let display_name = config
            .and_then(|c| c.get("displayName"))
            .and_then(serde_json::Value::as_str)
            .unwrap_or(id)
            .to_string();
        tabs.push(FavoriteTabManifest {
            id: id.to_string(),
            display_name,
            starred: id == &starred_tab,
            visible: visible && enabled,
            admin_only,
        });
    }
    tabs.sort_by(|a, b| a.id.cmp(&b.id));
    let manifest = FavoriteManifest {
        schema: "coronatio.favorite-manifest.v1".to_string(),
        starred_tab,
        source_quarry: vec![source, "homeserver.json tabs.{config,visibility,starred}".to_string()],
        tabs,
    };
    validate_favorite_manifest(&manifest)?;
    Ok(manifest)
}

async fn load_favorite_manifest() -> Result<(String, FavoriteManifest), String> {
    let (source, value) = load_homeserver_json().await?;
    let manifest = favorite_manifest_from_homeserver(source.clone(), &value)?;
    Ok((source, manifest))
}

fn validate_favorite_manifest(manifest: &FavoriteManifest) -> Result<(), String> {
    if manifest.schema != "coronatio.favorite-manifest.v1" { return Err(format!("unexpected favorite manifest schema {}", manifest.schema)); }
    let mut starred_count = 0;
    for tab in &manifest.tabs {
        if !is_safe_tab_id(&tab.id) { return Err(format!("favorite tab id {} is not forward-safe", tab.id)); }
        if tab.starred { starred_count += 1; }
        if tab.starred != (tab.id == manifest.starred_tab) { return Err(format!("favorite manifest star flag disagrees with configured starred tab {}", manifest.starred_tab)); }
    }
    if starred_count > 1 { return Err(format!("favorite manifest must carry at most one starred tab, found {}", starred_count)); }
    Ok(())
}

async fn load_iris_facts() -> Result<(String, IrisFacts), String> {
    let (source, value) = load_homeserver_json().await?;
    Ok((source, iris_facts_from_homeserver_value(&value)))
}

fn canonical_stats_element_id(id: &str) -> &str {
    match id {
        "network" => "network-chart",
        "memory" => "memory-usage",
        "process-list" => "process-usage",
        canonical => canonical,
    }
}

fn iris_facts_from_homeserver_value(value: &serde_json::Value) -> IrisFacts {
    iris::from_coronatio_contracts(&build_tab_contracts(value, &xenia_status(true)), registry_starred(value))
}

fn caduceus_config_failure_response(readback: CaduceusHttpReadback) -> Response {
    let status = mutation_response_status(&readback);
    (status, Json(serde_json::json!({
        "success": false,
        "firstMissingSignal": readback.first_missing_signal,
        "caduceus": readback,
    }))).into_response()
}

async fn themes_route() -> impl IntoResponse {
    match load_theme_catalog().await {
        Ok((source, catalog)) => Json(ThemeCatalogResponse {
            schema: "coronatio.theme-catalog.response.v1".to_string(),
            source,
            default: catalog.default,
            required: REQUIRED_THEME_KEYS.iter().map(|key| (*key).to_string()).collect(),
            themes: catalog.themes,
        })
        .into_response(),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "schema": "coronatio.theme-catalog.error.v1",
                "ok": false,
                "error": error,
                "expected": "homeserver.json global.theme.name",
            })),
        )
            .into_response(),
    }
}

async fn load_theme_catalog() -> Result<(String, ThemeCatalog), String> {
    let (source, value) = load_homeserver_json().await?;
    let default = value
        .get("global")
        .and_then(|global| global.get("theme"))
        .and_then(|theme| theme.get("name"))
        .and_then(serde_json::Value::as_str)
        .unwrap_or("light")
        .to_string();
    let catalog = firmware_theme_catalog(default)?;
    validate_theme_catalog(&catalog)?;
    Ok((format!("{} global.theme.name; firmware catalog", source), catalog))
}

const FIRMWARE_THEME_CATALOG: &str = include_str!("theme/catalog.json");

fn firmware_theme_catalog(default: String) -> Result<ThemeCatalog, String> {
    let mut catalog: ThemeCatalog = serde_json::from_str(FIRMWARE_THEME_CATALOG)
        .map_err(|error| format!("parse embedded firmware theme catalog: {error}"))?;
    catalog.default = if catalog.themes.contains_key(&default) {
        default
    } else {
        catalog.default
    };
    Ok(catalog)
}

fn validate_theme_catalog(catalog: &ThemeCatalog) -> Result<(), String> {
    if catalog.schema != "coronatio.theme-catalog.v1" {
        return Err(format!("unexpected theme catalog schema {}", catalog.schema));
    }
    if catalog.themes.is_empty() {
        return Err("theme catalog has no themes".to_string());
    }
    if !catalog.themes.contains_key(&catalog.default) {
        return Err(format!("default theme {} is absent", catalog.default));
    }
    for (name, theme) in &catalog.themes {
        if name.trim().is_empty() || !name.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-') {
            return Err(format!("theme name {} is not forward-safe", name));
        }
        for key in REQUIRED_THEME_KEYS {
            let value = theme
                .get(*key)
                .ok_or_else(|| format!("theme {} missing key {}", name, key))?;
            if value.trim().is_empty() {
                return Err(format!("theme {} key {} is empty", name, key));
            }
        }
    }
    Ok(())
}
