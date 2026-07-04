async fn crown_shell_route(State(state): State<AppState>) -> impl IntoResponse {
    let tabs = load_tab_manifests(&state.tab_root).await.unwrap_or_default();
    Html(render_crown_shell_with_registry(&tabs))
}

async fn crown_stylesheet_route() -> impl IntoResponse {
    ([(header::CONTENT_TYPE, "text/css; charset=utf-8")], CROWN_SHELL_CSS)
}

async fn crown_chrome_script_route() -> impl IntoResponse {
    ([(header::CONTENT_TYPE, "application/javascript; charset=utf-8")], CROWN_SHELL_JS)
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

fn insert_theme_tokens(theme: &mut BTreeMap<String, String>, tokens: &[(&str, &str)]) {
    for (key, value) in tokens {
        theme.insert((*key).to_string(), (*value).to_string());
    }
}

fn insert_system_theme_tokens(theme: &mut BTreeMap<String, String>) {
    insert_theme_tokens(theme, &[
        ("spacing-xxs", "0.125rem"),
        ("spacing-xs", "0.25rem"),
        ("spacing-sm", "0.5rem"),
        ("spacing-md", "1rem"),
        ("spacing-lg", "1.5rem"),
        ("spacing-xl", "2rem"),
        ("spacing-2xl", "3rem"),
        ("font-family", "system-ui, -apple-system, BlinkMacSystemFont, \"Segoe UI\", Roboto, sans-serif"),
        ("font-mono", "ui-monospace, SFMono-Regular, Menlo, Consolas, monospace"),
        ("font-size-xs", "12px"),
        ("font-size-sm", "0.875rem"),
        ("font-size-base", "16px"),
        ("font-size-md", "16px"),
        ("font-size-lg", "1.125rem"),
        ("font-size-xl", "24px"),
        ("font-size-2xl", "32px"),
        ("font-weight-normal", "400"),
        ("font-weight-medium", "500"),
        ("font-weight-bold", "600"),
        ("line-height-tight", "1.2"),
        ("line-height-normal", "1.5"),
        ("line-height-loose", "1.8"),
        ("transition-fast", "150ms ease"),
        ("transition-normal", "250ms ease"),
        ("transition-slow", "350ms ease"),
        ("shadow-sm", "0 1px 2px rgba(0,0,0,0.1)"),
        ("shadow-md", "0 2px 4px rgba(0,0,0,0.1)"),
        ("shadow-lg", "0 4px 8px rgba(0,0,0,0.1)"),
        ("radius", "4px"),
        ("radius-sm", "4px"),
        ("radius-md", "6px"),
        ("radius-lg", "8px"),
        ("radius-pill", "999px"),
        ("border-width", "1px"),
        ("focus-ring", "0 0 0 2px color-mix(in srgb, var(--theme-color-primary) 30%, transparent)"),
        ("header-height", "48px"),
        ("tab-height", "48px"),
        ("control-height", "34px"),
        ("control-padding-x", "0.9rem"),
        ("control-padding-y", "0.55rem"),
        ("content-padding", "20px"),
        ("card-padding", "1rem"),
        ("card-min-height", "112px"),
        ("card-radius", "8px"),
        ("modal-radius", "10px"),
        ("portal-icon-size", "96px"),
        ("chart-height", "200px"),
        ("grid-gap", "16px"),
    ]);
}

fn insert_legacy_alias_tokens(
    theme: &mut BTreeMap<String, String>,
    background: &str,
    text: &str,
    primary: &str,
    primary_hover: &str,
    secondary: &str,
    accent: &str,
    error: &str,
    success: &str,
    warning: &str,
    border: &str,
    status_unknown: &str,
    hidden_tab_background: &str,
    hidden_tab_text: &str,
) {
    insert_theme_tokens(theme, &[
        ("background", background),
        ("text", text),
        ("primary", primary),
        ("primaryHover", primary_hover),
        ("secondary", secondary),
        ("accent", accent),
        ("error", error),
        ("success", success),
        ("warning", warning),
        ("border", border),
        ("statusUp", success),
        ("statusDown", error),
        ("statusPartial", warning),
        ("statusUnknown", status_unknown),
        ("hiddenTabBackground", hidden_tab_background),
        ("hiddenTabText", hidden_tab_text),
    ]);
}


fn insert_mature_theme_tokens(theme: &mut BTreeMap<String, String>, mode: &str) {
    let tokens: &[(&str, &str)] = match mode {
        "light" => &[
            ("role-primary", "#A0AEC0"), ("role-on-primary", "#1A1A1A"), ("role-primary-container", "#E2E8F0"), ("role-on-primary-container", "#1A1A1A"),
            ("role-secondary", "#4A5568"), ("role-on-secondary", "#FFFFFF"), ("role-secondary-container", "#BCCCDC"), ("role-on-secondary-container", "#1A1A1A"),
            ("role-tertiary", "#90cff3"), ("role-on-tertiary", "#0A0A0A"), ("role-tertiary-container", "#ddf4ff"), ("role-on-tertiary-container", "#1A1A1A"),
            ("surface-0", "#FFFFFF"), ("surface-1", "#F7F7F7"), ("surface-2", "#E2E8F0"), ("surface-3", "#BCCCDC"), ("surface-inverse", "#1A1A1A"),
            ("on-surface", "#1A1A1A"), ("on-surface-muted", "#4A5568"), ("outline", "#A0AEC0"), ("outline-variant", "#E5E7EB"),
            ("highlight-subtle", "rgba(144,207,243,0.18)"), ("highlight-strong", "rgba(144,207,243,0.34)"), ("highlight-ring", "0 0 0 3px rgba(144,207,243,0.32)"),
            ("accent-warm", "#F59E0B"), ("accent-cool", "#90cff3"), ("accent-neutral", "#A0AEC0"), ("accent-critical", "#df0a3f"),
            ("gradient-primary", "linear-gradient(135deg, #A0AEC0 0%, #BCCCDC 100%)"), ("gradient-accent", "linear-gradient(135deg, #90cff3 0%, #A78BFA 100%)"),
            ("gradient-surface", "linear-gradient(180deg, #FFFFFF 0%, #F7F7F7 100%)"), ("gradient-highlight", "radial-gradient(circle at 20% 20%, rgba(144,207,243,0.34), transparent 55%)"),
            ("elevation-1", "0 1px 2px rgba(0,0,0,0.10)"), ("elevation-2", "0 4px 10px rgba(0,0,0,0.12)"), ("elevation-3", "0 12px 24px rgba(0,0,0,0.16)"),
            ("overlay-scrim", "rgba(15,23,42,0.42)"), ("overlay-tint", "rgba(144,207,243,0.10)"), ("focus-color", "#90cff3"),
            ("component-button-container", "#A0AEC0"), ("component-button-on-container", "#1A1A1A"), ("component-button-hover-container", "#BCCCDC"),
            ("component-card-container", "#FFFFFF"), ("component-card-outline", "#E5E7EB"), ("contrast-mode", "standard"),
        ],
        "radioactive" => &[
            ("role-primary", "#00d084"), ("role-on-primary", "#050805"), ("role-primary-container", "#163d16"), ("role-on-primary-container", "#f3fff2"),
            ("role-secondary", "#39ff14"), ("role-on-secondary", "#050805"), ("role-secondary-container", "#0b210b"), ("role-on-secondary-container", "#b6f5b1"),
            ("role-tertiary", "#2196f3"), ("role-on-tertiary", "#050805"), ("role-tertiary-container", "#123112"), ("role-on-tertiary-container", "#f3fff2"),
            ("surface-0", "#050805"), ("surface-1", "#101510"), ("surface-2", "#0b210b"), ("surface-3", "#123112"), ("surface-inverse", "#f3fff2"),
            ("on-surface", "#f3fff2"), ("on-surface-muted", "#b6f5b1"), ("outline", "rgba(57,255,20,0.44)"), ("outline-variant", "rgba(57,255,20,0.22)"),
            ("highlight-subtle", "rgba(57,255,20,0.16)"), ("highlight-strong", "rgba(57,255,20,0.34)"), ("highlight-ring", "0 0 0 3px rgba(57,255,20,0.38)"),
            ("accent-warm", "#ff9800"), ("accent-cool", "#2196f3"), ("accent-neutral", "#7ccf76"), ("accent-critical", "#f44336"),
            ("gradient-primary", "linear-gradient(135deg, #00d084 0%, #39ff14 100%)"), ("gradient-accent", "linear-gradient(135deg, #39ff14 0%, #2196f3 100%)"),
            ("gradient-surface", "linear-gradient(180deg, #101510 0%, #050805 100%)"), ("gradient-highlight", "radial-gradient(circle at 20% 20%, rgba(57,255,20,0.34), transparent 55%)"),
            ("elevation-1", "0 1px 4px rgba(57,255,20,0.14)"), ("elevation-2", "0 4px 12px rgba(57,255,20,0.18)"), ("elevation-3", "0 12px 32px rgba(57,255,20,0.22)"),
            ("overlay-scrim", "rgba(0,0,0,0.72)"), ("overlay-tint", "rgba(57,255,20,0.12)"), ("focus-color", "#39ff14"),
            ("component-button-container", "#00d084"), ("component-button-on-container", "#050805"), ("component-button-hover-container", "#123112"),
            ("component-card-container", "#101510"), ("component-card-outline", "rgba(57,255,20,0.28)"), ("contrast-mode", "display"),
        ],
        _ => &[
            ("role-primary", "#323840"), ("role-on-primary", "#E0E0E0"), ("role-primary-container", "#1E293B"), ("role-on-primary-container", "#E0E0E0"),
            ("role-secondary", "#9CA3AF"), ("role-on-secondary", "#0A0A0A"), ("role-secondary-container", "#6B7280"), ("role-on-secondary-container", "#E0E0E0"),
            ("role-tertiary", "#A78BFA"), ("role-on-tertiary", "#0A0A0A"), ("role-tertiary-container", "#2a2140"), ("role-on-tertiary-container", "#F4F0FF"),
            ("surface-0", "#0A0A0A"), ("surface-1", "#111827"), ("surface-2", "#1E293B"), ("surface-3", "#323840"), ("surface-inverse", "#E0E0E0"),
            ("on-surface", "#E0E0E0"), ("on-surface-muted", "#9CA3AF"), ("outline", "#6B7280"), ("outline-variant", "#1E293B"),
            ("highlight-subtle", "rgba(167,139,250,0.16)"), ("highlight-strong", "rgba(167,139,250,0.34)"), ("highlight-ring", "0 0 0 3px rgba(167,139,250,0.34)"),
            ("accent-warm", "#FBBF24"), ("accent-cool", "#90cff3"), ("accent-neutral", "#9CA3AF"), ("accent-critical", "#df0a3f"),
            ("gradient-primary", "linear-gradient(135deg, #323840 0%, #6B7280 100%)"), ("gradient-accent", "linear-gradient(135deg, #A78BFA 0%, #90cff3 100%)"),
            ("gradient-surface", "linear-gradient(180deg, #111827 0%, #0A0A0A 100%)"), ("gradient-highlight", "radial-gradient(circle at 20% 20%, rgba(167,139,250,0.34), transparent 55%)"),
            ("elevation-1", "0 1px 4px rgba(0,0,0,0.28)"), ("elevation-2", "0 6px 16px rgba(0,0,0,0.34)"), ("elevation-3", "0 18px 40px rgba(0,0,0,0.44)"),
            ("overlay-scrim", "rgba(0,0,0,0.68)"), ("overlay-tint", "rgba(167,139,250,0.10)"), ("focus-color", "#A78BFA"),
            ("component-button-container", "#323840"), ("component-button-on-container", "#E0E0E0"), ("component-button-hover-container", "#6B7280"),
            ("component-card-container", "#111827"), ("component-card-outline", "#1E293B"), ("contrast-mode", "standard"),
        ],
    };
    insert_theme_tokens(theme, tokens);
    insert_theme_tokens(theme, &[
        ("state-hover-opacity", ".08"), ("state-focus-opacity", ".12"), ("state-pressed-opacity", ".12"), ("state-drag-opacity", ".16"), ("state-selected-opacity", ".10"),
        ("focus-width", "2px"), ("focus-offset", "2px"), ("contrast-minimum", "4.5"), ("density", "comfortable"), ("elevation-0", "none"),
        ("flag-gradients", "enabled"), ("flag-highlights", "enabled"), ("flag-accent-stripes", "enabled"), ("flag-state-layers", "enabled"), ("flag-density-scale", "enabled"),
    ]);
}

fn firmware_theme_catalog(default: String) -> ThemeCatalog {
    let mut themes = BTreeMap::new();
    {
        let mut theme = BTreeMap::new();
        insert_theme_tokens(&mut theme, &[
            ("background", "#F7F7F7"),
            ("text", "#1A1A1A"),
            ("primary", "#A0AEC0"),
            ("primaryHover", "#BCCCDC"),
            ("secondary", "#4A5568"),
            ("accent", "#90cff3"),
            ("error", "#df0a3f"),
            ("success", "#059669"),
            ("warning", "#F59E0B"),
            ("border", "#E5E7EB"),
            ("statusUp", "#059669"),
            ("statusDown", "#EF4444"),
            ("statusPartial", "#F59E0B"),
            ("statusUnknown", "#6B7280"),
            ("hiddenTabBackground", "#E2E8F0"),
            ("hiddenTabText", "#A0AEC0"),
            ("color-primary", "#A0AEC0"),
            ("color-secondary", "#4A5568"),
            ("bg-primary", "#F7F7F7"),
            ("bg-secondary", "#F7F7F7"),
            ("bg-tertiary", "#E2E8F0"),
            ("bg-hover", "#BCCCDC"),
            ("bg-active", "#BCCCDC"),
            ("text-primary", "#1A1A1A"),
            ("text-secondary", "#4A5568"),
            ("text-tertiary", "#A0AEC0"),
            ("text-disabled", "#6B7280"),
            ("text-accent", "#90cff3"),
            ("status-success", "#059669"),
            ("status-error", "#EF4444"),
            ("status-warning", "#F59E0B"),
            ("status-info", "#90cff3"),
        ]);
        insert_system_theme_tokens(&mut theme);
        insert_mature_theme_tokens(&mut theme, "light");
        themes.insert("light".to_string(), theme);
    }
    {
        let mut theme = BTreeMap::new();
        insert_theme_tokens(&mut theme, &[
            ("background", "#0A0A0A"),
            ("text", "#E0E0E0"),
            ("primary", "#323840"),
            ("primaryHover", "#6B7280"),
            ("secondary", "#9CA3AF"),
            ("accent", "#A78BFA"),
            ("error", "#df0a3f"),
            ("success", "#10B981"),
            ("warning", "#FBBF24"),
            ("border", "#1E293B"),
            ("statusUp", "#10B981"),
            ("statusDown", "#F87171"),
            ("statusPartial", "#FBBF24"),
            ("statusUnknown", "#94A3B8"),
            ("hiddenTabBackground", "#1E293B"),
            ("hiddenTabText", "#A0AEC0"),
            ("color-primary", "#323840"),
            ("color-secondary", "#9CA3AF"),
            ("bg-primary", "#0A0A0A"),
            ("bg-secondary", "#0A0A0A"),
            ("bg-tertiary", "#1E293B"),
            ("bg-hover", "#6B7280"),
            ("bg-active", "#6B7280"),
            ("text-primary", "#E0E0E0"),
            ("text-secondary", "#9CA3AF"),
            ("text-tertiary", "#A0AEC0"),
            ("text-disabled", "#94A3B8"),
            ("text-accent", "#A78BFA"),
            ("status-success", "#10B981"),
            ("status-error", "#F87171"),
            ("status-warning", "#FBBF24"),
            ("status-info", "#A78BFA"),
        ]);
        insert_system_theme_tokens(&mut theme);
        insert_mature_theme_tokens(&mut theme, "dark");
        theme.insert("shadow-md".to_string(), "0 2px 4px rgba(0,0,0,0.35)".to_string());
        themes.insert("dark".to_string(), theme);
    }
    {
        let mut theme = BTreeMap::new();
        insert_theme_tokens(&mut theme, &[
            ("color-primary", "#39ff14"),
            ("color-secondary", "#00d084"),
            ("bg-primary", "#101510"),
            ("bg-secondary", "#050805"),
            ("bg-tertiary", "#0b210b"),
            ("bg-hover", "#123112"),
            ("bg-active", "#163d16"),
            ("text-primary", "#f3fff2"),
            ("text-secondary", "#b6f5b1"),
            ("text-tertiary", "#7ccf76"),
            ("text-disabled", "#477047"),
            ("text-accent", "#39ff14"),
            ("status-success", "#4CAF50"),
            ("status-error", "#f44336"),
            ("status-warning", "#ff9800"),
            ("status-info", "#2196f3"),
        ]);
        insert_system_theme_tokens(&mut theme);
        insert_mature_theme_tokens(&mut theme, "radioactive");
        theme.insert("shadow-md".to_string(), "0 2px 8px rgba(57,255,20,0.18)".to_string());
        insert_legacy_alias_tokens(&mut theme, "#050805", "#f3fff2", "#00d084", "#123112", "#b6f5b1", "#39ff14", "#f44336", "#4CAF50", "#ff9800", "rgba(57,255,20,0.28)", "#477047", "#0b210b", "#7ccf76");
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
