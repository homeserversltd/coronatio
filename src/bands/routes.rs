async fn crown_shell_route(headers: axum::http::HeaderMap) -> impl IntoResponse {
    (
        [(header::CONTENT_SECURITY_POLICY, CROWN_CONTENT_SECURITY_POLICY)],
        Html(render_crown_shell_for_session(session_from_headers(&headers))),
    )
}

async fn crown_htmx_script_route() -> impl IntoResponse {
    ([(header::CONTENT_TYPE, "application/javascript; charset=utf-8")], CROWN_HTMX_JS)
}

async fn crown_chrome_script_route() -> impl IntoResponse {
    ([(header::CONTENT_TYPE, "application/javascript; charset=utf-8")], crown_chrome_js())
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
            "/api/admin/session".to_string(),
            "/api/caduceus/status".to_string(),
            "/api/caduceus/update/check".to_string(),
            "/api/caduceus/update/now".to_string(),
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
            "/api/core/events".to_string(),
            "/api/core/events/renew".to_string(),
            "/api/stats/events".to_string(),
            "/api/stats/events/renew".to_string(),
            "/api/stats".to_string(),
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

async fn stats_route(headers: axum::http::HeaderMap) -> Response {
    let raw = stats_snapshot();
    match session_from_headers(&headers) {
        Session::Admin => Json(project_system_stats_admin(&raw)).into_response(),
        Session::Guest => Json(project_system_stats_guest(&raw)).into_response(),
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

async fn favorites_route() -> impl IntoResponse {
    match load_favorite_manifest().await {
        Ok((source, manifest)) => Json(FavoriteManifestResponse {
            schema: "coronatio.favorite-manifest.response.v1".to_string(),
            source,
            starred_tab: manifest.starred_tab,
            source_quarry: manifest.source_quarry,
            tabs: manifest.tabs,
            first_load_law: "original Flask root loads get_starred_tab() or get_first_visible_tab(); Coronatio opens the manifest starred tab unless an explicit hash names a valid visible tab".to_string(),
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
    let persisted = caduceus_config_set("tabs.starred", serde_json::Value::String(next.starred.clone()));
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
    if session_from_headers(&headers) != Session::Admin {
        return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({
            "schema": "coronatio.tabs.visibility.refusal.v1",
            "success": false,
            "error": "admin-session-required",
        }))).into_response();
    }
    let tab = normalize_tab_id(&request.tab.or(request.tab_id).or(request.id).unwrap_or_default());
    let visible = request.visible.or(request.visibility).unwrap_or(true);
    let (_source, facts) = match load_iris_facts().await {
        Ok(value) => value,
        Err(error) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"success": false, "error": error}))).into_response(),
    };
    let next = iris::apply_tab_visibility(&facts, &tab, visible);
    let visibility_path = format!("tabs.{tab}.visibility.tab");
    let persisted = caduceus_config_set(&visibility_path, serde_json::Value::Bool(visible));
    if !persisted.ok {
        return caduceus_config_failure_response(persisted);
    }
    if next.starred != facts.starred {
        let starred = caduceus_config_set("tabs.starred", serde_json::Value::String(next.starred.clone()));
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
    tab_bar_html_response_with_active(session_from_headers(&headers), query.active.as_deref())
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

fn homeserver_json_path() -> PathBuf {
    if let Ok(path) = env::var("CORONATIO_HOMESERVER_JSON") {
        return PathBuf::from(path);
    }
    for candidate in [
        INSTALLED_HOMESERVER_JSON,
        LEGACY_HOMESERVER_JSON,
        FLASK_HOMESERVER_JSON,
        FACTORY_HOMESERVER_JSON,
        QUARRY_HOMESERVER_JSON,
        LOCAL_QUARRY_HOMESERVER_JSON,
    ] {
        let path = PathBuf::from(candidate);
        if path.exists() {
            return path;
        }
    }
    PathBuf::from(INSTALLED_HOMESERVER_JSON)
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
    let mut manifest = FavoriteManifest {
        schema: "coronatio.favorite-manifest.v1".to_string(),
        starred_tab,
        source_quarry: vec![source, "homeserver.json tabs.{config,visibility,starred}".to_string()],
        tabs,
    };
    if !manifest.tabs.iter().any(|tab| tab.id == manifest.starred_tab && tab.visible && !tab.admin_only) {
        if let Some(tab) = manifest.tabs.iter().find(|tab| tab.visible && !tab.admin_only) {
            manifest.starred_tab = tab.id.clone();
        }
    }
    for tab in manifest.tabs.iter_mut() {
        tab.starred = tab.id == manifest.starred_tab;
    }
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
    let mut starred_valid = false;
    for tab in &manifest.tabs {
        if !is_safe_tab_id(&tab.id) { return Err(format!("favorite tab id {} is not forward-safe", tab.id)); }
        if tab.starred { starred_count += 1; }
        if tab.id == manifest.starred_tab && tab.visible && !tab.admin_only { starred_valid = true; }
    }
    if starred_count != 1 { return Err(format!("favorite manifest must carry exactly one starred tab, found {}", starred_count)); }
    if !starred_valid { return Err(format!("starred tab {} is absent, hidden, or admin-only", manifest.starred_tab)); }
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
    let tabs_obj = value.get("tabs").and_then(serde_json::Value::as_object);
    let starred = tabs_obj
        .and_then(|tabs| tabs.get("starred"))
        .and_then(serde_json::Value::as_str)
        .unwrap_or("stats")
        .to_string();
    let mut tabs = native_tab_contracts();
    if let Some(tabs_obj) = tabs_obj {
        for tab in tabs.iter_mut() {
            let Some(raw) = tabs_obj.get(&tab.id) else { continue; };
            if let Some(config) = raw.get("config").and_then(serde_json::Value::as_object) {
                if let Some(name) = config.get("displayName").and_then(serde_json::Value::as_str) { tab.display_name = name.to_string(); }
                if let Some(enabled) = config.get("isEnabled").and_then(serde_json::Value::as_bool) { tab.enabled = enabled; }
                if let Some(admin_only) = config.get("adminOnly").and_then(serde_json::Value::as_bool) { tab.admin_only = admin_only; }
            }
            if let Some(visibility) = raw.get("visibility").and_then(serde_json::Value::as_object) {
                if let Some(visible) = visibility.get("tab").and_then(serde_json::Value::as_bool) { tab.visibility.tab = visible; }
                if let Some(elements) = visibility.get("elements").and_then(serde_json::Value::as_object) {
                    tab.visibility.elements = elements.iter().filter_map(|(id, v)| {
                        v.as_bool().map(|visible| {
                            let canonical = if tab.id == "stats" { canonical_stats_element_id(id) } else { id.as_str() };
                            (canonical.to_string(), visible)
                        })
                    }).collect();
                }
            }
        }
    }
    iris::from_coronatio_contracts(&tabs, &starred)
}

fn caduceus_config_failure_response(readback: CaduceusHttpReadback) -> Response {
    let status = if readback.status == 0 { StatusCode::SERVICE_UNAVAILABLE } else { StatusCode::BAD_GATEWAY };
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
