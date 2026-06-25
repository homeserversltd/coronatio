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
                "expected": DEFAULT_FAVORITES_JSON,
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

async fn load_favorite_manifest() -> Result<(String, FavoriteManifest), String> {
    let path = favorite_manifest_path();
    let raw = fs::read_to_string(&path).await.map_err(|error| format!("favorite manifest unreadable at {}: {}", path.display(), error))?;
    let manifest: FavoriteManifest = serde_json::from_str(&raw).map_err(|error| format!("favorite manifest invalid at {}: {}", path.display(), error))?;
    validate_favorite_manifest(&manifest)?;
    Ok((path.display().to_string(), manifest))
}

async fn save_favorite_manifest(manifest: &FavoriteManifest) -> Result<(), String> {
    let path = favorite_manifest_path();
    let body = serde_json::to_string_pretty(manifest).map_err(|error| error.to_string())? + "\n";
    fs::write(&path, body).await.map_err(|error| format!("favorite manifest not writable at {}: {}", path.display(), error))
}

fn favorite_manifest_path() -> PathBuf {
    if let Ok(path) = env::var("CORONATIO_FAVORITES_JSON") { return PathBuf::from(path); }
    let local = PathBuf::from(DEFAULT_FAVORITES_JSON);
    if local.exists() { return local; }
    PathBuf::from(INSTALLED_FAVORITES_JSON)
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
                "expected": DEFAULT_THEME_JSON,
            })),
        )
            .into_response(),
    }
}

async fn load_theme_catalog() -> Result<(String, ThemeCatalog), String> {
    let path = theme_catalog_path();
    let raw = fs::read_to_string(&path)
        .await
        .map_err(|error| format!("theme json unreadable at {}: {}", path.display(), error))?;
    let catalog: ThemeCatalog = serde_json::from_str(&raw)
        .map_err(|error| format!("theme json invalid at {}: {}", path.display(), error))?;
    validate_theme_catalog(&catalog)?;
    Ok((path.display().to_string(), catalog))
}

fn theme_catalog_path() -> PathBuf {
    if let Ok(path) = env::var("CORONATIO_THEME_JSON") {
        return PathBuf::from(path);
    }
    let local = PathBuf::from(DEFAULT_THEME_JSON);
    if local.exists() {
        return local;
    }
    PathBuf::from(INSTALLED_THEME_JSON)
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
