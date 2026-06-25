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
            "/api/stats/events".to_string(),
            "/api/stats/events/renew".to_string(),
            "/api/stats".to_string(),
            "/api/tabs".to_string(),
            "/api/tabs/:tab_id/manifest".to_string(),
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

async fn set_starred_tab_route(Json(request): Json<SetStarredTabRequest>) -> impl IntoResponse {
    let requested = request.tab_name.or(request.tab).unwrap_or_default();
    let requested = normalize_tab_id(&requested);
    match load_favorite_manifest().await {
        Ok((source, mut manifest)) => {
            if request.is_starred == Some(false) {
                return Json(serde_json::json!({
                    "schema": "coronatio.starred-tab.mutation.v1",
                    "success": false,
                    "error": "one-to-one port preserves one active favorite; choose another visible non-admin tab instead of clearing all favorites",
                    "starred_tab": manifest.starred_tab,
                    "source": source,
                })).into_response();
            }
            let allowed = manifest.tabs.iter().any(|tab| tab.id == requested && tab.visible && !tab.admin_only);
            if !allowed {
                return (StatusCode::BAD_REQUEST, Json(serde_json::json!({
                    "schema": "coronatio.starred-tab.mutation.v1",
                    "success": false,
                    "error": "favorite target must be visible, enabled, and non-admin",
                    "requested": requested,
                    "starred_tab": manifest.starred_tab,
                    "source": source,
                }))).into_response();
            }
            manifest.starred_tab = requested.clone();
            for tab in manifest.tabs.iter_mut() { tab.starred = tab.id == requested; }
            let write_result = save_favorite_manifest(&manifest).await;
            Json(serde_json::json!({
                "schema": "coronatio.starred-tab.mutation.v1",
                "success": write_result.is_ok(),
                "starred_tab": requested,
                "source": source,
                "write_error": write_result.err(),
            })).into_response()
        }
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
            "schema": "coronatio.starred-tab.mutation.v1",
            "success": false,
            "error": error,
        }))).into_response(),
    }
}

fn homeserver_json_path() -> PathBuf {
    for candidate in [
        INSTALLED_HOMESERVER_JSON,
        LEGACY_HOMESERVER_JSON,
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
        .unwrap_or("portals")
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

async fn save_favorite_manifest(_manifest: &FavoriteManifest) -> Result<(), String> {
    Err("homeserver.json is the single config authority; starred-tab mutation must enter the Caduceus homeserver.json transaction membrane".to_string())
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
        .unwrap_or("dark")
        .to_string();
    let catalog = firmware_theme_catalog(default);
    validate_theme_catalog(&catalog)?;
    Ok((format!("{} global.theme.name", source), catalog))
}

fn firmware_theme_catalog(default: String) -> ThemeCatalog {
    let mut themes = BTreeMap::new();
    {
        let mut theme = BTreeMap::new();
        theme.insert("color-primary".to_string(), "#1976d2".to_string());
        theme.insert("color-secondary".to_string(), "#f5f5f5".to_string());
        theme.insert("bg-primary".to_string(), "#ffffff".to_string());
        theme.insert("bg-secondary".to_string(), "#f5f5f5".to_string());
        theme.insert("bg-tertiary".to_string(), "#e0e0e0".to_string());
        theme.insert("bg-hover".to_string(), "#eeeeee".to_string());
        theme.insert("bg-active".to_string(), "#d5d5d5".to_string());
        theme.insert("text-primary".to_string(), "#000000".to_string());
        theme.insert("text-secondary".to_string(), "#666666".to_string());
        theme.insert("text-tertiary".to_string(), "#999999".to_string());
        theme.insert("text-disabled".to_string(), "#cccccc".to_string());
        theme.insert("text-accent".to_string(), "#1976d2".to_string());
        theme.insert("status-success".to_string(), "#4CAF50".to_string());
        theme.insert("status-error".to_string(), "#f44336".to_string());
        theme.insert("status-warning".to_string(), "#ff9800".to_string());
        theme.insert("status-info".to_string(), "#2196f3".to_string());
        theme.insert("spacing-xs".to_string(), "0.25rem".to_string());
        theme.insert("spacing-sm".to_string(), "0.5rem".to_string());
        theme.insert("spacing-md".to_string(), "1rem".to_string());
        theme.insert("spacing-lg".to_string(), "1.5rem".to_string());
        theme.insert("spacing-xl".to_string(), "2rem".to_string());
        theme.insert("font-family".to_string(), "system-ui, -apple-system, BlinkMacSystemFont, \"Segoe UI\", Roboto, sans-serif".to_string());
        theme.insert("font-mono".to_string(), "ui-monospace, SFMono-Regular, Menlo, Consolas, monospace".to_string());
        theme.insert("font-size-xs".to_string(), "12px".to_string());
        theme.insert("font-size-sm".to_string(), "0.875rem".to_string());
        theme.insert("font-size-base".to_string(), "16px".to_string());
        theme.insert("font-size-md".to_string(), "16px".to_string());
        theme.insert("font-size-lg".to_string(), "1.125rem".to_string());
        theme.insert("font-size-xl".to_string(), "24px".to_string());
        theme.insert("font-weight-normal".to_string(), "400".to_string());
        theme.insert("font-weight-medium".to_string(), "500".to_string());
        theme.insert("font-weight-bold".to_string(), "600".to_string());
        theme.insert("line-height-tight".to_string(), "1.2".to_string());
        theme.insert("line-height-normal".to_string(), "1.5".to_string());
        theme.insert("line-height-loose".to_string(), "1.8".to_string());
        theme.insert("transition-fast".to_string(), "150ms ease".to_string());
        theme.insert("transition-normal".to_string(), "250ms ease".to_string());
        theme.insert("transition-slow".to_string(), "350ms ease".to_string());
        theme.insert("shadow-sm".to_string(), "0 1px 2px rgba(0,0,0,0.1)".to_string());
        theme.insert("shadow-md".to_string(), "0 4px 6px rgba(0,0,0,0.1)".to_string());
        theme.insert("shadow-lg".to_string(), "0 4px 8px rgba(0,0,0,0.1)".to_string());
        theme.insert("radius".to_string(), "4px".to_string());
        themes.insert("light".to_string(), theme);
    }
    {
        let mut theme = BTreeMap::new();
        theme.insert("color-primary".to_string(), "#00f2fe".to_string());
        theme.insert("color-secondary".to_string(), "#4CAF50".to_string());
        theme.insert("bg-primary".to_string(), "#2a2a2a".to_string());
        theme.insert("bg-secondary".to_string(), "#1a1a1a".to_string());
        theme.insert("bg-tertiary".to_string(), "#222222".to_string());
        theme.insert("bg-hover".to_string(), "#333333".to_string());
        theme.insert("bg-active".to_string(), "#3a3a3a".to_string());
        theme.insert("text-primary".to_string(), "#ffffff".to_string());
        theme.insert("text-secondary".to_string(), "#dddddd".to_string());
        theme.insert("text-tertiary".to_string(), "#a7a7a7".to_string());
        theme.insert("text-disabled".to_string(), "#777777".to_string());
        theme.insert("text-accent".to_string(), "#00f2fe".to_string());
        theme.insert("status-success".to_string(), "#4CAF50".to_string());
        theme.insert("status-error".to_string(), "#f44336".to_string());
        theme.insert("status-warning".to_string(), "#ff9800".to_string());
        theme.insert("status-info".to_string(), "#2196f3".to_string());
        theme.insert("spacing-xs".to_string(), "0.25rem".to_string());
        theme.insert("spacing-sm".to_string(), "0.5rem".to_string());
        theme.insert("spacing-md".to_string(), "1rem".to_string());
        theme.insert("spacing-lg".to_string(), "1.5rem".to_string());
        theme.insert("spacing-xl".to_string(), "2rem".to_string());
        theme.insert("font-family".to_string(), "system-ui, -apple-system, BlinkMacSystemFont, \"Segoe UI\", Roboto, sans-serif".to_string());
        theme.insert("font-mono".to_string(), "ui-monospace, SFMono-Regular, Menlo, Consolas, monospace".to_string());
        theme.insert("font-size-xs".to_string(), "12px".to_string());
        theme.insert("font-size-sm".to_string(), "0.875rem".to_string());
        theme.insert("font-size-base".to_string(), "16px".to_string());
        theme.insert("font-size-md".to_string(), "16px".to_string());
        theme.insert("font-size-lg".to_string(), "1.125rem".to_string());
        theme.insert("font-size-xl".to_string(), "24px".to_string());
        theme.insert("font-weight-normal".to_string(), "400".to_string());
        theme.insert("font-weight-medium".to_string(), "500".to_string());
        theme.insert("font-weight-bold".to_string(), "600".to_string());
        theme.insert("line-height-tight".to_string(), "1.2".to_string());
        theme.insert("line-height-normal".to_string(), "1.5".to_string());
        theme.insert("line-height-loose".to_string(), "1.8".to_string());
        theme.insert("transition-fast".to_string(), "150ms ease".to_string());
        theme.insert("transition-normal".to_string(), "250ms ease".to_string());
        theme.insert("transition-slow".to_string(), "350ms ease".to_string());
        theme.insert("shadow-sm".to_string(), "0 1px 2px rgba(0,0,0,0.1)".to_string());
        theme.insert("shadow-md".to_string(), "0 2px 4px rgba(0,0,0,0.35)".to_string());
        theme.insert("shadow-lg".to_string(), "0 4px 8px rgba(0,0,0,0.1)".to_string());
        theme.insert("radius".to_string(), "4px".to_string());
        themes.insert("dark".to_string(), theme);
    }
    {
        let mut theme = BTreeMap::new();
        theme.insert("color-primary".to_string(), "#39ff14".to_string());
        theme.insert("color-secondary".to_string(), "#00d084".to_string());
        theme.insert("bg-primary".to_string(), "#101510".to_string());
        theme.insert("bg-secondary".to_string(), "#050805".to_string());
        theme.insert("bg-tertiary".to_string(), "#0b210b".to_string());
        theme.insert("bg-hover".to_string(), "#123112".to_string());
        theme.insert("bg-active".to_string(), "#163d16".to_string());
        theme.insert("text-primary".to_string(), "#f3fff2".to_string());
        theme.insert("text-secondary".to_string(), "#b6f5b1".to_string());
        theme.insert("text-tertiary".to_string(), "#7ccf76".to_string());
        theme.insert("text-disabled".to_string(), "#477047".to_string());
        theme.insert("text-accent".to_string(), "#39ff14".to_string());
        theme.insert("status-success".to_string(), "#4CAF50".to_string());
        theme.insert("status-error".to_string(), "#f44336".to_string());
        theme.insert("status-warning".to_string(), "#ff9800".to_string());
        theme.insert("status-info".to_string(), "#2196f3".to_string());
        theme.insert("spacing-xs".to_string(), "0.25rem".to_string());
        theme.insert("spacing-sm".to_string(), "0.5rem".to_string());
        theme.insert("spacing-md".to_string(), "1rem".to_string());
        theme.insert("spacing-lg".to_string(), "1.5rem".to_string());
        theme.insert("spacing-xl".to_string(), "2rem".to_string());
        theme.insert("font-family".to_string(), "system-ui, -apple-system, BlinkMacSystemFont, \"Segoe UI\", Roboto, sans-serif".to_string());
        theme.insert("font-mono".to_string(), "ui-monospace, SFMono-Regular, Menlo, Consolas, monospace".to_string());
        theme.insert("font-size-xs".to_string(), "12px".to_string());
        theme.insert("font-size-sm".to_string(), "0.875rem".to_string());
        theme.insert("font-size-base".to_string(), "16px".to_string());
        theme.insert("font-size-md".to_string(), "16px".to_string());
        theme.insert("font-size-lg".to_string(), "1.125rem".to_string());
        theme.insert("font-size-xl".to_string(), "24px".to_string());
        theme.insert("font-weight-normal".to_string(), "400".to_string());
        theme.insert("font-weight-medium".to_string(), "500".to_string());
        theme.insert("font-weight-bold".to_string(), "600".to_string());
        theme.insert("line-height-tight".to_string(), "1.2".to_string());
        theme.insert("line-height-normal".to_string(), "1.5".to_string());
        theme.insert("line-height-loose".to_string(), "1.8".to_string());
        theme.insert("transition-fast".to_string(), "150ms ease".to_string());
        theme.insert("transition-normal".to_string(), "250ms ease".to_string());
        theme.insert("transition-slow".to_string(), "350ms ease".to_string());
        theme.insert("shadow-sm".to_string(), "0 1px 2px rgba(0,0,0,0.1)".to_string());
        theme.insert("shadow-md".to_string(), "0 2px 8px rgba(57,255,20,0.18)".to_string());
        theme.insert("shadow-lg".to_string(), "0 4px 8px rgba(0,0,0,0.1)".to_string());
        theme.insert("radius".to_string(), "4px".to_string());
        themes.insert("radioactive".to_string(), theme);
    }
    let selected = if themes.contains_key(&default) { default } else { "dark".to_string() };
    ThemeCatalog { schema: "coronatio.theme-catalog.v1".to_string(), default: selected, themes }
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
