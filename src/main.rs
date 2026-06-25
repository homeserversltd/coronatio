use axum::{
    extract::{Path, State},
    http::{header, StatusCode, Uri},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, env, net::SocketAddr, path::PathBuf, sync::Arc};
use tokio::fs;
use tower_http::services::ServeDir;

const DEFAULT_TAB_ROOT: &str = "/var/lib/coronatio/tabs";
const PRIMARY_TABS: [&str; 4] = ["admin", "stats", "portals", "upload"];

#[derive(Clone)]
struct AppState {
    tab_root: Arc<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct CoronatioRoot {
    schema: String,
    kind: String,
    product: String,
    routes: Vec<String>,
    tab_root: String,
    primary_tabs: Vec<String>,
    first_party_panes: Vec<CrownPane>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct CrownPane {
    id: String,
    title: String,
    role: String,
    summary: String,
    order: i64,
    admin_only: bool,
    install_mode: InstallMode,
    route: String,
    state_route: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct TabManifest {
    id: String,
    title: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    icon: String,
    #[serde(default)]
    display_name: String,
    #[serde(default)]
    order: i64,
    #[serde(default = "default_enabled")]
    enabled: bool,
    #[serde(default)]
    admin_only: bool,
    #[serde(default)]
    visibility: TabVisibility,
    #[serde(default)]
    data: serde_json::Value,
    #[serde(default)]
    route_prefix: String,
    #[serde(default)]
    static_dir: String,
    #[serde(default)]
    service_url: Option<String>,
    #[serde(default)]
    health_route: Option<String>,
    #[serde(default)]
    install_mode: InstallMode,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
enum InstallMode {
    #[default]
    DynamicCartridge,
    SourceInjectionRecompile,
    FirstPartyNative,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct TabList {
    schema: String,
    tab_root: String,
    native_panes: Vec<CrownPane>,
    tabs: Vec<TabManifest>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct TabVisibility {
    tab: bool,
    elements: BTreeMap<String, bool>,
}

impl Default for TabVisibility {
    fn default() -> Self {
        Self {
            tab: true,
            elements: BTreeMap::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct RegistryReadback {
    schema: String,
    source_contract: String,
    starred_tab: String,
    default_route_tab: String,
    force_tab_bar_visibility: bool,
    visible_tabs_user: Vec<String>,
    visible_tabs_admin: Vec<String>,
    validation_rules: Vec<ValidationRule>,
    native_tab_contracts: Vec<CoronatioTabContract>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct ValidationRule {
    field: String,
    rule: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct CoronatioTabContract {
    id: String,
    display_name: String,
    order: i64,
    enabled: bool,
    admin_only: bool,
    visibility: TabVisibility,
    install_mode: InstallMode,
    route: String,
    state_route: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct StartupReadback {
    schema: String,
    phases: Vec<String>,
    current_phase: String,
    connection_status: String,
    initial_tab: String,
    default_route_law: String,
    fallback_tab: String,
    tab_bar_law: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct LanePolicyReadback {
    schema: String,
    policies: Vec<InstallLanePolicy>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct InstallLanePolicy {
    install_mode: InstallMode,
    success_contract: String,
    failure_contract: String,
    recovery_contract: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct FallbackReadback {
    schema: String,
    safe_pane: String,
    activation_reasons: Vec<String>,
    recovery_sequence: Vec<String>,
    receipt_fields: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct AdminSessionReadback {
    schema: String,
    pin_validation: String,
    session_timeout_seconds: u64,
    keepalive_route: String,
    logout_route: String,
    token_header: String,
    token_policy: Vec<String>,
    admin_enhanced_filtering: Vec<AdminFieldFilter>,
    caduceus_membrane: CaduceusMembrane,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct AdminFieldFilter {
    topic: String,
    admin_fields: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct CaduceusMembrane {
    schema: String,
    privileged_mutations: Vec<String>,
    coronatio_role: String,
    caduceus_role: String,
    first_missing_signal: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct TopicCatalogReadback {
    schema: String,
    transport: String,
    stream_policy: String,
    renew_policy: String,
    core_topics: Vec<TopicContract>,
    admin_topics: Vec<TopicContract>,
    tab_topics: Vec<TabTopicContract>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct TabTopicContract {
    pane_id: String,
    topics: Vec<String>,
    event_route: String,
    renew_route: String,
    lifecycle: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct TopicContract {
    id: String,
    scope: String,
    cadence_seconds: u64,
    admin_only: bool,
    admin_fields: Vec<String>,
    payload_schema: String,
    changed_rule: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct MonitorPulseReadback {
    schema: String,
    topic: TopicContract,
    snapshot_route: String,
    event_route: String,
    renew_route: String,
    first_event: StatsEventPayload,
    proof_policy: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct StatsEventPayload {
    schema: String,
    topic: String,
    event_id: String,
    event: String,
    lease_seconds: u64,
    payload_state: String,
    first_missing_signal: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct LeaseRenewalReadback {
    schema: String,
    topic: String,
    route: String,
    lease_seconds: u64,
    status: String,
    next_renewal_before_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct BoundaryReadback {
    schema: String,
    api_unknown_path_policy: String,
    static_shell_policy: String,
    cartridge_static_policy: String,
    cors_source: String,
    premium_blueprint_replacement: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct StatsSnapshot {
    schema: String,
    pane_id: String,
    product: String,
    transport: StatsTransport,
    telemetry: StatsTelemetry,
    next_routes: StatsNextRoutes,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct StatsTransport {
    snapshot_route: String,
    event_route: String,
    renew_route: String,
    stream_status: String,
    stream_reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct StatsTelemetry {
    load1: Option<f64>,
    cpu_temperature_celsius: Option<f64>,
    service_health: Option<String>,
    storage_posture: Option<String>,
    first_missing_signal: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct StatsNextRoutes {
    snapshot: String,
    events: String,
    renew: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let port: u16 = env::var("CORONATIO_PORT")
        .ok()
        .and_then(|raw| raw.parse().ok())
        .unwrap_or(8090);
    let tab_root = env::var("CORONATIO_TAB_ROOT").unwrap_or_else(|_| DEFAULT_TAB_ROOT.to_string());
    let state = AppState {
        tab_root: Arc::new(PathBuf::from(tab_root)),
    };

    let app = app(state);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("bind coronatio listener");
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("serve coronatio");
}

fn app(state: AppState) -> Router {
    Router::new()
        .route("/", get(crown_shell_route))
        .route("/health", get(health_route))
        .route("/api", get(api_root_route))
        .route("/api/panes", get(panes_route))
        .route("/api/panes/:pane_id", get(pane_route))
        .route("/api/registry", get(registry_route))
        .route("/api/startup", get(startup_route))
        .route("/api/lanes", get(lane_policy_route))
        .route("/api/fallback", get(fallback_route))
        .route("/api/session", get(session_route))
        .route(
            "/api/admin/session",
            get(session_route).post(session_renew_route),
        )
        .route("/api/topics", get(topics_route))
        .route("/api/monitor/pulse", get(monitor_pulse_route))
        .route("/api/boundary", get(boundary_route))
        .route("/api/stats/events", get(stats_events_route))
        .route("/api/stats/events/renew", post(stats_events_renew_route))
        .route("/api/stats", get(stats_route))
        .route("/api/tabs", get(tabs_route))
        .route("/api/tabs/:tab_id/manifest", get(tab_manifest_route))
        .nest_service("/tabs", ServeDir::new((*state.tab_root).clone()))
        .fallback(route_boundary_fallback)
        .with_state(state)
}

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
            "/api/startup".to_string(),
            "/api/lanes".to_string(),
            "/api/fallback".to_string(),
            "/api/session".to_string(),
            "/api/admin/session".to_string(),
            "/api/topics".to_string(),
            "/api/monitor/pulse".to_string(),
            "/api/boundary".to_string(),
            "/api/stats/events".to_string(),
            "/api/stats/events/renew".to_string(),
            "/api/stats".to_string(),
            "/api/tabs".to_string(),
            "/api/tabs/:tab_id/manifest".to_string(),
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

async fn topics_route() -> impl IntoResponse {
    Json(topic_catalog_readback())
}

async fn monitor_pulse_route() -> impl IntoResponse {
    Json(monitor_pulse_readback())
}

async fn boundary_route() -> impl IntoResponse {
    Json(boundary_readback())
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

async fn route_boundary_fallback(uri: Uri) -> impl IntoResponse {
    let normalized = uri.path().to_string();
    if normalized.starts_with("/api/") {
        return (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "schema": "coronatio.api.error.v1",
                "error": "api route not found",
                "path": normalized,
                "policy": "API clients receive JSON 404, never the shell HTML"
            })),
        )
            .into_response();
    }
    Html(render_crown_shell()).into_response()
}

async fn pane_route(Path(pane_id): Path<String>) -> impl IntoResponse {
    if !is_safe_tab_id(&pane_id) {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "invalid pane id"})),
        )
            .into_response();
    }

    match native_crown_panes()
        .into_iter()
        .find(|pane| pane.id == pane_id)
    {
        Some(pane) => Json(pane).into_response(),
        None => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "pane not found"})),
        )
            .into_response(),
    }
}

async fn tabs_route(State(state): State<AppState>) -> impl IntoResponse {
    match load_tab_manifests(&state.tab_root).await {
        Ok(tabs) => Json(TabList {
            schema: "coronatio.tabs.v1".to_string(),
            tab_root: state.tab_root.display().to_string(),
            native_panes: native_crown_panes(),
            tabs,
        })
        .into_response(),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": error.to_string()})),
        )
            .into_response(),
    }
}

async fn tab_manifest_route(
    State(state): State<AppState>,
    Path(tab_id): Path<String>,
) -> impl IntoResponse {
    if !is_safe_tab_id(&tab_id) {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "invalid tab id"})),
        )
            .into_response();
    }

    match load_tab_manifest(&state.tab_root, &tab_id).await {
        Ok(Some(manifest)) => Json(manifest).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "tab not found"})),
        )
            .into_response(),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": error.to_string()})),
        )
            .into_response(),
    }
}

async fn load_tab_manifests(tab_root: &PathBuf) -> Result<Vec<TabManifest>, std::io::Error> {
    let mut tabs = Vec::new();
    let mut entries = match fs::read_dir(tab_root).await {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(tabs),
        Err(error) => return Err(error),
    };

    while let Some(entry) = entries.next_entry().await? {
        if !entry.file_type().await?.is_dir() {
            continue;
        }
        let tab_id = entry.file_name().to_string_lossy().to_string();
        if !is_safe_tab_id(&tab_id) {
            continue;
        }
        if let Some(manifest) = load_tab_manifest(tab_root, &tab_id).await? {
            tabs.push(manifest);
        }
    }
    tabs.sort_by(|left, right| left.order.cmp(&right.order).then(left.id.cmp(&right.id)));
    Ok(tabs)
}

async fn load_tab_manifest(
    tab_root: &PathBuf,
    tab_id: &str,
) -> Result<Option<TabManifest>, std::io::Error> {
    let manifest_path = tab_root.join(tab_id).join("tab.json");
    let raw = match fs::read_to_string(manifest_path).await {
        Ok(raw) => raw,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(error),
    };
    let manifest = serde_json::from_str::<TabManifest>(&raw)
        .map_err(|error| std::io::Error::new(std::io::ErrorKind::InvalidData, error))?;
    if manifest.id != tab_id || !is_safe_tab_id(&manifest.id) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "tab id mismatch or unsafe tab id",
        ));
    }
    validate_tab_manifest(&manifest)
        .map_err(|error| std::io::Error::new(std::io::ErrorKind::InvalidData, error))?;
    Ok(Some(manifest))
}

fn native_crown_panes() -> Vec<CrownPane> {
    vec![
        CrownPane {
            id: "admin".to_string(),
            title: "Admin".to_string(),
            role: "session and crown policy".to_string(),
            summary: "PIN depth, capability posture, install receipts, and controlled mutation entrypoints.".to_string(),
            order: 0,
            admin_only: true,
            install_mode: InstallMode::FirstPartyNative,
            route: "/#admin".to_string(),
            state_route: "/api/panes/admin".to_string(),
        },
        CrownPane {
            id: "stats".to_string(),
            title: "Stats".to_string(),
            role: "machine telemetry".to_string(),
            summary: "System load, service health, storage posture, and later SSE live readback.".to_string(),
            order: 10,
            admin_only: false,
            install_mode: InstallMode::FirstPartyNative,
            route: "/#stats".to_string(),
            state_route: "/api/stats".to_string(),
        },
        CrownPane {
            id: "portals".to_string(),
            title: "Portals".to_string(),
            role: "service ingress".to_string(),
            summary: "Admitted HOMESERVER services, local ingress, remote ingress, and service currentness.".to_string(),
            order: 20,
            admin_only: false,
            install_mode: InstallMode::FirstPartyNative,
            route: "/#portals".to_string(),
            state_route: "/api/panes/portals".to_string(),
        },
        CrownPane {
            id: "upload".to_string(),
            title: "Upload".to_string(),
            role: "file ingress".to_string(),
            summary: "Safe file admission into HOMESERVER storage with policy and receipt readbacks.".to_string(),
            order: 30,
            admin_only: false,
            install_mode: InstallMode::FirstPartyNative,
            route: "/#upload".to_string(),
            state_route: "/api/panes/upload".to_string(),
        },
    ]
}

fn default_enabled() -> bool {
    true
}

fn validate_tab_manifest(manifest: &TabManifest) -> Result<(), String> {
    if !is_safe_tab_id(&manifest.id) {
        return Err("id must be lowercase hyphen-case ascii".to_string());
    }
    if manifest.title.trim().is_empty() {
        return Err("title must be present".to_string());
    }
    if manifest.install_mode == InstallMode::FirstPartyNative {
        return Err(
            "first-party-native panes are compiled crown law, not user cartridge manifests"
                .to_string(),
        );
    }
    if manifest.order < 0 {
        return Err("order must be non-negative".to_string());
    }
    if manifest.route_prefix.is_empty() {
        return Err("routePrefix must be present".to_string());
    }
    let expected = format!("/api/tabs/{}", manifest.id);
    if manifest.route_prefix != expected {
        return Err(format!("routePrefix must equal {expected}"));
    }
    if manifest.static_dir.contains("..") || manifest.static_dir.starts_with('/') {
        return Err("staticDir must be relative and stay inside the tab root".to_string());
    }
    Ok(())
}

fn registry_readback() -> RegistryReadback {
    let native_tab_contracts = native_tab_contracts();
    RegistryReadback {
        schema: "coronatio.registry.v1".to_string(),
        source_contract: "homeserver.json tabs.{config,visibility,data,starred}".to_string(),
        starred_tab: "portals".to_string(),
        default_route_tab: initial_tab(true, None, false),
        force_tab_bar_visibility: false,
        visible_tabs_user: visible_tab_ids(&native_tab_contracts, false),
        visible_tabs_admin: visible_tab_ids(&native_tab_contracts, true),
        validation_rules: registry_validation_rules(),
        native_tab_contracts,
    }
}

fn native_tab_contracts() -> Vec<CoronatioTabContract> {
    native_crown_panes()
        .into_iter()
        .map(|pane| CoronatioTabContract {
            id: pane.id.clone(),
            display_name: pane.title.clone(),
            order: pane.order,
            enabled: true,
            admin_only: pane.admin_only,
            visibility: TabVisibility::default(),
            install_mode: pane.install_mode,
            route: pane.route,
            state_route: pane.state_route,
        })
        .collect()
}

fn visible_tab_ids(tabs: &[CoronatioTabContract], is_admin: bool) -> Vec<String> {
    let mut visible = tabs
        .iter()
        .filter(|tab| tab.enabled && tab.visibility.tab && (!tab.admin_only || is_admin))
        .collect::<Vec<_>>();
    visible.sort_by(|left, right| left.order.cmp(&right.order).then(left.id.cmp(&right.id)));
    visible.into_iter().map(|tab| tab.id.clone()).collect()
}

fn registry_validation_rules() -> Vec<ValidationRule> {
    vec![
        ValidationRule { field: "id".to_string(), rule: "lowercase ascii letters, digits, and hyphen only; @ is normalized away before lookup".to_string() },
        ValidationRule { field: "displayName/title".to_string(), rule: "human visible title is required".to_string() },
        ValidationRule { field: "visibility.tab".to_string(), rule: "regular users see only enabled visible non-admin tabs; admins see enabled visible tabs including admin-only".to_string() },
        ValidationRule { field: "order".to_string(), rule: "non-negative sort key; ties sort by tab id for deterministic recovery".to_string() },
        ValidationRule { field: "starred".to_string(), rule: "default route pointer; invalid, disabled, hidden, or fallback resolves to first visible tab then fallback".to_string() },
        ValidationRule { field: "installMode".to_string(), rule: "dynamic-cartridge for runtime tabs, source-injection-recompile for trusted source lanes, first-party-native only for compiled crown panes".to_string() },
    ]
}

fn initial_tab(connection_ok: bool, forced_tab: Option<&str>, is_admin: bool) -> String {
    if let Some(tab) = forced_tab {
        return normalize_tab_id(tab);
    }
    if !connection_ok {
        return "fallback".to_string();
    }
    let contracts = native_tab_contracts();
    let visible = visible_tab_ids(&contracts, is_admin);
    let starred = normalize_tab_id("portals");
    if visible.iter().any(|tab| tab == &starred) {
        starred
    } else {
        visible
            .first()
            .cloned()
            .unwrap_or_else(|| "fallback".to_string())
    }
}

fn startup_readback() -> StartupReadback {
    StartupReadback {
        schema: "coronatio.startup.v1".to_string(),
        phases: vec!["idle", "loading-config", "connecting-events", "app-ready", "fallback", "error"]
            .into_iter()
            .map(String::from)
            .collect(),
        current_phase: "app-ready".to_string(),
        connection_status: "snapshot-only-ready".to_string(),
        initial_tab: initial_tab(true, None, false),
        default_route_law: "forced tab wins; failed connection uses fallback; valid visible starred tab wins; else first visible tab by order; else fallback".to_string(),
        fallback_tab: "fallback".to_string(),
        tab_bar_law: "admin sessions show the tab bar; regular sessions show it when more than two tabs are visible unless forceTabBarVisibility overrides".to_string(),
    }
}

fn lane_policy_readback() -> LanePolicyReadback {
    LanePolicyReadback {
        schema: "coronatio.lane-policy.v1".to_string(),
        policies: vec![
            InstallLanePolicy { install_mode: InstallMode::DynamicCartridge, success_contract: "manifest validates, staticDir remains inside tab root, optional service health is read separately, routePrefix is /api/tabs/<id>".to_string(), failure_contract: "missing manifest/static asset/service health yields tab-local error and fallback recovery candidate, not host failure".to_string(), recovery_contract: "choose next visible accessible tab; if none, activate fallback receipt".to_string() },
            InstallLanePolicy { install_mode: InstallMode::SourceInjectionRecompile, success_contract: "trusted package injects declared source, host rebuilds, cargo fmt/test/build passes, and Cibation admits source".to_string(), failure_contract: "compile/test/admission failure rejects the package and preserves previous admitted host".to_string(), recovery_contract: "keep old host binary standing; emit source-injection failure receipt".to_string() },
            InstallLanePolicy { install_mode: InstallMode::FirstPartyNative, success_contract: "pane is compiled into Coronatio and present in native registry".to_string(), failure_contract: "native pane missing is a build/test failure, not a runtime cartridge problem".to_string(), recovery_contract: "fallback pane remains reachable while source repair proceeds through Cibation".to_string() },
        ],
    }
}

fn fallback_readback() -> FallbackReadback {
    FallbackReadback {
        schema: "coronatio.fallback.v1".to_string(),
        safe_pane: "fallback".to_string(),
        activation_reasons: vec![
            "connection_failed",
            "startup_timeout",
            "no_visible_tabs",
            "module_load_error",
            "invalid_native_pane",
            "critical_startup_error",
        ]
        .into_iter()
        .map(String::from)
        .collect(),
        recovery_sequence: vec![
            "classify reason",
            "preserve previous active tab in receipt",
            "choose starred visible tab if lawful",
            "choose first visible accessible tab",
            "activate fallback when no lawful pane exists",
            "emit recovery receipt",
        ]
        .into_iter()
        .map(String::from)
        .collect(),
        receipt_fields: vec![
            "schema",
            "reason",
            "previousTab",
            "candidateTab",
            "selectedTab",
            "connectionStatus",
            "startupPhase",
            "timestamp",
        ]
        .into_iter()
        .map(String::from)
        .collect(),
    }
}

fn normalize_tab_id(tab_id: &str) -> String {
    tab_id.strip_prefix('@').unwrap_or(tab_id).to_string()
}

fn admin_session_readback() -> AdminSessionReadback {
    AdminSessionReadback {
        schema: "coronatio.admin.session.v1".to_string(),
        pin_validation: "POST /api/validatePin compared request pin to homeserver.json global.admin.pin and returned a generated session token".to_string(),
        session_timeout_seconds: 30 * 60,
        keepalive_route: "/api/admin/session".to_string(),
        logout_route: "/api/logout".to_string(),
        token_header: "X-Admin-Token".to_string(),
        token_policy: vec![
            "tokens are generated from random bytes, timestamp, and uuid".to_string(),
            "token expiry refreshes on validation".to_string(),
            "logout invalidates the token".to_string(),
            "PIN fallback compatibility is retired behind Caduceus before privileged mutation".to_string(),
        ],
        admin_enhanced_filtering: admin_field_filters(),
        caduceus_membrane: CaduceusMembrane {
            schema: "coronatio.caduceus.membrane.v1".to_string(),
            privileged_mutations: vec![
                "pin change".to_string(),
                "service restart".to_string(),
                "premium/source installation".to_string(),
                "disk/vault/key operations".to_string(),
            ],
            coronatio_role: "session readback, capability request shaping, and non-secret UI state".to_string(),
            caduceus_role: "privileged host mutation, token mint/refresh, command execution, and mutation receipts".to_string(),
            first_missing_signal: "live Caduceus admin token minting endpoint not wired".to_string(),
        },
    }
}

fn admin_field_filters() -> Vec<AdminFieldFilter> {
    vec![
        AdminFieldFilter {
            topic: "internet_status".to_string(),
            admin_fields: vec!["publicIp", "ipDetails", "dnsServers"]
                .into_iter()
                .map(String::from)
                .collect(),
        },
        AdminFieldFilter {
            topic: "vpn_status".to_string(),
            admin_fields: vec!["connectionDetails", "credentials"]
                .into_iter()
                .map(String::from)
                .collect(),
        },
        AdminFieldFilter {
            topic: "system_stats".to_string(),
            admin_fields: vec!["processes", "users", "networkConnections"]
                .into_iter()
                .map(String::from)
                .collect(),
        },
        AdminFieldFilter {
            topic: "tailscale_status".to_string(),
            admin_fields: vec!["ip", "tailnet", "isEnabled", "loginUrl"]
                .into_iter()
                .map(String::from)
                .collect(),
        },
        AdminFieldFilter {
            topic: "services_status".to_string(),
            admin_fields: vec!["isEnabled"].into_iter().map(String::from).collect(),
        },
    ]
}

fn topic_catalog_readback() -> TopicCatalogReadback {
    TopicCatalogReadback {
        schema: "coronatio.topic-catalog.v1".to_string(),
        transport: "SSE EventSource plus POST renew; Socket.IO subscribe/unsubscribe is quarry only".to_string(),
        stream_policy: "open a pane stream only while the pane is active and document is visible; core topics stay independent of active pane".to_string(),
        renew_policy: "client renews before lease expiry; expired streams produce an expired event and close in the live implementation".to_string(),
        core_topics: core_topic_contracts(),
        admin_topics: admin_topic_contracts(),
        tab_topics: vec![
            TabTopicContract {
                pane_id: "stats".to_string(),
                topics: vec!["stats.system".to_string()],
                event_route: "/api/stats/events".to_string(),
                renew_route: "/api/stats/events/renew".to_string(),
                lifecycle: "active pane + visible document".to_string(),
            },
            TabTopicContract { pane_id: "upload".to_string(), topics: vec![], event_route: "snapshot-only".to_string(), renew_route: "snapshot-only".to_string(), lifecycle: "no live stream yet".to_string() },
            TabTopicContract { pane_id: "portals".to_string(), topics: vec![], event_route: "snapshot-only".to_string(), renew_route: "snapshot-only".to_string(), lifecycle: "no live stream yet".to_string() },
        ],
    }
}

fn core_topic_contracts() -> Vec<TopicContract> {
    vec![
        topic_contract(
            "internet.status",
            "core",
            10,
            false,
            vec!["publicIp", "ipDetails", "dnsServers"],
            "internet status and public ingress posture",
            "status/public IP/DNS changes",
        ),
        topic_contract(
            "tailscale.status",
            "core",
            10,
            false,
            vec!["ip", "tailnet", "isEnabled", "loginUrl"],
            "tailscale status and admin login hints",
            "status/interface/admin field changes",
        ),
        topic_contract(
            "vpn.status",
            "core",
            10,
            false,
            vec!["connectionDetails", "credentials"],
            "VPN and transmission status",
            "vpnStatus/transmissionStatus/isEnabled changes",
        ),
        topic_contract(
            "services.status",
            "core",
            10,
            false,
            vec!["isEnabled"],
            "service health posture",
            "service status or enabled-state changes",
        ),
        topic_contract(
            "power.status",
            "core",
            1,
            false,
            vec![],
            "power sample",
            "always broadcast realtime power samples",
        ),
    ]
}

fn admin_topic_contracts() -> Vec<TopicContract> {
    vec![
        topic_contract(
            "admin.disk.info",
            "admin",
            30,
            true,
            vec![],
            "disk, encryption, NAS compatibility, and mount posture",
            "device/error/encryption/mount/filesystem/periodic changes",
        ),
        topic_contract(
            "admin.system",
            "admin",
            2,
            true,
            vec![],
            "admin system details",
            "admin-only system stats pulse",
        ),
        topic_contract(
            "hard-drive-test.status",
            "admin",
            5,
            true,
            vec![],
            "hard-drive-test state",
            "test status changes",
        ),
        topic_contract(
            "sync.status",
            "admin",
            2,
            true,
            vec![],
            "sync job status",
            "sync status changes",
        ),
    ]
}

fn topic_contract(
    id: &str,
    scope: &str,
    cadence_seconds: u64,
    admin_only: bool,
    admin_fields: Vec<&str>,
    payload_schema: &str,
    changed_rule: &str,
) -> TopicContract {
    TopicContract {
        id: id.to_string(),
        scope: scope.to_string(),
        cadence_seconds,
        admin_only,
        admin_fields: admin_fields.into_iter().map(String::from).collect(),
        payload_schema: payload_schema.to_string(),
        changed_rule: changed_rule.to_string(),
    }
}

fn stats_topic_contract() -> TopicContract {
    topic_contract(
        "stats.system",
        "tab:stats",
        1,
        false,
        vec!["processes", "users", "networkConnections"],
        "system_stats payload: load, cpu, memory, disk, network, process/user/admin fields",
        "always pulse realtime system stats; admin fields filtered unless session has admin capability",
    )
}

fn monitor_pulse_readback() -> MonitorPulseReadback {
    MonitorPulseReadback {
        schema: "coronatio.monitor-pulse.v1".to_string(),
        topic: stats_topic_contract(),
        snapshot_route: "/api/stats".to_string(),
        event_route: "/api/stats/events".to_string(),
        renew_route: "/api/stats/events/renew".to_string(),
        first_event: stats_event_payload(),
        proof_policy: vec![
            "initial subscriber receives first state".to_string(),
            "meaningful-change predicate decides later pulses".to_string(),
            "admin fields are filtered for non-admin sessions".to_string(),
            "SSE heartbeat/expiry replaces Socket.IO subscription diffing".to_string(),
        ],
    }
}

fn stats_event_payload() -> StatsEventPayload {
    StatsEventPayload {
        schema: "coronatio.stats.event.v1".to_string(),
        topic: "stats.system".to_string(),
        event_id: "stats-system-bootstrap-1".to_string(),
        event: "snapshot".to_string(),
        lease_seconds: 30,
        payload_state: "placeholder-unavailable".to_string(),
        first_missing_signal: "stats collectors not wired".to_string(),
    }
}

fn boundary_readback() -> BoundaryReadback {
    BoundaryReadback {
        schema: "coronatio.route-boundary.v1".to_string(),
        api_unknown_path_policy: "/api/* misses return JSON 404 with coronatio.api.error.v1, never shell HTML".to_string(),
        static_shell_policy: "non-API unknown GET paths return the Coronatio shell for client-side routing".to_string(),
        cartridge_static_policy: "/tabs/<tab-id>/... is served from the configured tab root through safe tab ids and manifest validation".to_string(),
        cors_source: "homeserver.json global.cors.allowed_origins becomes Coronatio config law in the later config tranche".to_string(),
        premium_blueprint_replacement: "dynamic Flask blueprint injection is replaced by dynamic-cartridge, source-injection-recompile, or first-party-native lanes".to_string(),
    }
}

fn stats_snapshot() -> StatsSnapshot {
    StatsSnapshot {
        schema: "coronatio.stats.snapshot.v1".to_string(),
        pane_id: "stats".to_string(),
        product: "Coronatio".to_string(),
        transport: StatsTransport {
            snapshot_route: "/api/stats".to_string(),
            event_route: "/api/stats/events".to_string(),
            renew_route: "/api/stats/events/renew".to_string(),
            stream_status: "planned".to_string(),
            stream_reason: "stats SSE lease route is the next Coronatio tranche; snapshot is the current authority".to_string(),
        },
        telemetry: StatsTelemetry {
            load1: None,
            cpu_temperature_celsius: None,
            service_health: None,
            storage_posture: None,
            first_missing_signal: "stats collectors not wired".to_string(),
        },
        next_routes: StatsNextRoutes {
            snapshot: "/api/stats".to_string(),
            events: "/api/stats/events".to_string(),
            renew: "/api/stats/events/renew".to_string(),
        },
    }
}

fn render_crown_shell() -> String {
    let nav = native_crown_panes()
        .into_iter()
        .map(|pane| {
            format!(
                r##"<a class="crown-tab" href="#{id}" data-pane="{id}">{title}</a>"##,
                id = pane.id,
                title = pane.title
            )
        })
        .collect::<Vec<_>>()
        .join("");

    format!(
        r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Coronatio</title>
  <style>
    :root {{ color-scheme: dark; font-family: Inter, ui-sans-serif, system-ui, sans-serif; background: #070a12; color: #edf2ff; }}
    body {{ margin: 0; min-height: 100vh; background: radial-gradient(circle at top, #17233a 0, #070a12 42rem); }}
    .crown-shell {{ display: grid; grid-template-columns: 14rem 1fr; min-height: 100vh; }}
    .crown-rail {{ border-right: 1px solid rgba(255,255,255,.12); padding: 1rem; background: rgba(3,6,12,.72); }}
    .crown-mark {{ font-weight: 800; letter-spacing: .08em; text-transform: uppercase; margin-bottom: 1rem; }}
    .crown-tab {{ display: block; color: #dbe7ff; text-decoration: none; padding: .75rem .85rem; border-radius: .75rem; margin-bottom: .35rem; background: rgba(255,255,255,.055); }}
    .crown-tab:hover {{ background: rgba(125,166,255,.18); }}
    .crown-stage {{ padding: 1.25rem; }}
    .crown-hero {{ border: 1px solid rgba(255,255,255,.12); border-radius: 1.1rem; padding: 1.1rem; background: rgba(8,13,25,.78); box-shadow: 0 1.5rem 4rem rgba(0,0,0,.28); }}
    .crown-grid {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(13rem, 1fr)); gap: .8rem; margin-top: 1rem; }}
    .crown-card {{ border: 1px solid rgba(255,255,255,.10); border-radius: .95rem; padding: .95rem; background: rgba(255,255,255,.045); }}
    .chip {{ display: inline-block; border: 1px solid rgba(125,166,255,.35); border-radius: 999px; padding: .18rem .55rem; font-size: .78rem; color: #aac3ff; }}
  </style>
</head>
<body>
  <main class="crown-shell" data-product="Coronatio">
    <nav class="crown-rail" aria-label="Coronatio primary tabs">
      <div class="crown-mark">Coronatio</div>
      {nav}
    </nav>
    <section class="crown-stage">
      <div class="crown-hero">
        <span class="chip">HOMESERVER Rust crown</span>
        <h1>Coronatio crown shell</h1>
        <p>First-party panes are native Rust crown law. Installed services enter through governed cartridges or source-injection recompiles.</p>
        <div class="crown-grid">
          <article class="crown-card"><strong>Admin</strong><br>Session, capability, and install authority.</article>
          <article class="crown-card"><strong>Stats</strong><br>Machine readback and live telemetry lane.</article>
          <article class="crown-card"><strong>Portals</strong><br>Admitted service ingress and currentness.</article>
          <article class="crown-card"><strong>Upload</strong><br>Safe file ingress with receipts.</article>
        </div>
      </div>
    </section>
  </main>
</body>
</html>"#
    )
}

fn is_safe_tab_id(tab_id: &str) -> bool {
    !tab_id.is_empty()
        && tab_id
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
}

async fn shutdown_signal() {
    let _ = tokio::signal::ctrl_c().await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    #[test]
    fn tab_ids_are_forward_safe() {
        for tab_id in PRIMARY_TABS {
            assert!(is_safe_tab_id(tab_id));
        }
        assert!(is_safe_tab_id("backblaze-tab"));
        assert!(!is_safe_tab_id("../escape"));
        assert!(!is_safe_tab_id("CamelCase"));
        assert!(!is_safe_tab_id(""));
    }

    #[test]
    fn native_panes_are_lawful_crown_tabs() {
        let panes = native_crown_panes();
        let ids: Vec<_> = panes.iter().map(|pane| pane.id.as_str()).collect();
        assert_eq!(ids, PRIMARY_TABS);
        assert!(panes
            .iter()
            .all(|pane| pane.install_mode == InstallMode::FirstPartyNative));
        assert!(panes
            .iter()
            .any(|pane| pane.admin_only && pane.id == "admin"));
    }

    #[tokio::test]
    async fn api_root_names_coronatio_not_arcadia() {
        let temp = test_tab_root("api-root");
        let response = app(AppState {
            tab_root: Arc::new(temp),
        })
        .oneshot(Request::builder().uri("/api").body(Body::empty()).unwrap())
        .await
        .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let body = String::from_utf8(bytes.to_vec()).unwrap();
        assert!(body.contains("coronatio.api.root.v1"));
        assert!(body.contains("Coronatio"));
        assert!(!body.contains("Arcadia"));
    }

    #[tokio::test]
    async fn api_root_declares_lawful_primary_tabs() {
        let temp = test_tab_root("primary-tabs");
        let response = app(AppState {
            tab_root: Arc::new(temp),
        })
        .oneshot(Request::builder().uri("/api").body(Body::empty()).unwrap())
        .await
        .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let root: CoronatioRoot = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(root.primary_tabs, ["admin", "stats", "portals", "upload"]);
        assert_eq!(root.first_party_panes.len(), 4);
    }

    #[tokio::test]
    async fn panes_route_exposes_first_party_crown_shell() {
        let temp = test_tab_root("panes");
        let response = app(AppState {
            tab_root: Arc::new(temp),
        })
        .oneshot(
            Request::builder()
                .uri("/api/panes")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let body = String::from_utf8(bytes.to_vec()).unwrap();
        assert!(body.contains("coronatio.panes.v1"));
        assert!(body.contains("first-party-native"));
        assert!(body.contains("Admin"));
        assert!(body.contains("Stats"));
        assert!(body.contains("Portals"));
        assert!(body.contains("Upload"));
        assert!(!body.contains("YouTube"));
    }

    #[tokio::test]
    async fn crown_shell_renders_primary_tabs_without_platform_brand_nav() {
        let temp = test_tab_root("shell");
        let response = app(AppState {
            tab_root: Arc::new(temp),
        })
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let body = String::from_utf8(bytes.to_vec()).unwrap();
        assert!(body.contains("data-product=\"Coronatio\""));
        assert!(body.contains("data-pane=\"admin\""));
        assert!(body.contains("data-pane=\"stats\""));
        assert!(body.contains("data-pane=\"portals\""));
        assert!(body.contains("data-pane=\"upload\""));
        assert!(body.contains("HOMESERVER Rust crown"));
        assert!(!body.contains("Arcadia"));
        assert!(!body.contains("YouTube"));
    }

    #[tokio::test]
    async fn loads_dynamic_cartridge_manifests_without_recompile() {
        let temp = test_tab_root("dynamic-tabs");
        let tab_dir = temp.join("service-card");
        std::fs::create_dir_all(&tab_dir).unwrap();
        std::fs::write(
            tab_dir.join("tab.json"),
            r#"{
              "id":"service-card",
              "title":"Service Card",
              "order":90,
              "adminOnly":true,
              "routePrefix":"/api/tabs/service-card",
              "staticDir":"static",
              "serviceUrl":"http://127.0.0.1:9910",
              "healthRoute":"/health",
              "installMode":"dynamic-cartridge"
            }"#,
        )
        .unwrap();

        let response = app(AppState {
            tab_root: Arc::new(temp),
        })
        .oneshot(
            Request::builder()
                .uri("/api/tabs")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let list: TabList = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(list.native_panes.len(), 4);
        assert_eq!(list.tabs.len(), 1);
        assert_eq!(list.tabs[0].id, "service-card");
        assert_eq!(list.tabs[0].install_mode, InstallMode::DynamicCartridge);
    }

    #[tokio::test]
    async fn stats_snapshot_is_honest_first_party_readback() {
        let temp = test_tab_root("stats-snapshot");
        let response = app(AppState {
            tab_root: Arc::new(temp),
        })
        .oneshot(
            Request::builder()
                .uri("/api/stats")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let snapshot: StatsSnapshot = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(snapshot.schema, "coronatio.stats.snapshot.v1");
        assert_eq!(snapshot.pane_id, "stats");
        assert_eq!(snapshot.product, "Coronatio");
        assert_eq!(snapshot.transport.snapshot_route, "/api/stats");
        assert_eq!(snapshot.transport.event_route, "/api/stats/events");
        assert_eq!(snapshot.transport.renew_route, "/api/stats/events/renew");
        assert_eq!(snapshot.transport.stream_status, "planned");
        assert_eq!(snapshot.telemetry.load1, None);
        assert_eq!(snapshot.telemetry.cpu_temperature_celsius, None);
        assert_eq!(
            snapshot.telemetry.first_missing_signal,
            "stats collectors not wired"
        );
    }

    #[test]
    fn stats_native_pane_points_to_stats_snapshot_route() {
        let stats = native_crown_panes()
            .into_iter()
            .find(|pane| pane.id == "stats")
            .unwrap();
        assert_eq!(stats.state_route, "/api/stats");
    }

    #[tokio::test]
    async fn registry_route_encodes_tab_visibility_and_starred_law() {
        let temp = test_tab_root("registry-law");
        let response = app(AppState {
            tab_root: Arc::new(temp),
        })
        .oneshot(
            Request::builder()
                .uri("/api/registry")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let registry: RegistryReadback = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(registry.schema, "coronatio.registry.v1");
        assert_eq!(registry.starred_tab, "portals");
        assert_eq!(registry.default_route_tab, "portals");
        assert_eq!(registry.visible_tabs_user, ["stats", "portals", "upload"]);
        assert_eq!(
            registry.visible_tabs_admin,
            ["admin", "stats", "portals", "upload"]
        );
        assert!(registry
            .validation_rules
            .iter()
            .any(|rule| rule.field == "starred"));
    }

    #[tokio::test]
    async fn startup_route_encodes_initial_tab_and_fallback_law() {
        let temp = test_tab_root("startup-law");
        let response = app(AppState {
            tab_root: Arc::new(temp),
        })
        .oneshot(
            Request::builder()
                .uri("/api/startup")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let startup: StartupReadback = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(startup.schema, "coronatio.startup.v1");
        assert_eq!(startup.initial_tab, "portals");
        assert_eq!(initial_tab(false, None, false), "fallback");
        assert_eq!(initial_tab(true, Some("@stats"), false), "stats");
        assert!(startup.default_route_law.contains("forced tab wins"));
    }

    #[tokio::test]
    async fn lane_policy_route_decides_dynamic_source_and_native_failures() {
        let temp = test_tab_root("lane-policy");
        let response = app(AppState {
            tab_root: Arc::new(temp),
        })
        .oneshot(
            Request::builder()
                .uri("/api/lanes")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let lanes: LanePolicyReadback = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(lanes.schema, "coronatio.lane-policy.v1");
        assert_eq!(lanes.policies.len(), 3);
        assert!(lanes.policies.iter().any(|policy| policy.install_mode
            == InstallMode::DynamicCartridge
            && policy.failure_contract.contains("tab-local error")));
        assert!(lanes.policies.iter().any(|policy| policy.install_mode
            == InstallMode::SourceInjectionRecompile
            && policy.success_contract.contains("Cibation admits")));
        assert!(lanes.policies.iter().any(|policy| policy.install_mode
            == InstallMode::FirstPartyNative
            && policy.failure_contract.contains("build/test failure")));
    }

    #[tokio::test]
    async fn fallback_route_encodes_safe_pane_and_recovery_receipt() {
        let temp = test_tab_root("fallback-law");
        let response = app(AppState {
            tab_root: Arc::new(temp),
        })
        .oneshot(
            Request::builder()
                .uri("/api/fallback")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let fallback: FallbackReadback = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(fallback.schema, "coronatio.fallback.v1");
        assert_eq!(fallback.safe_pane, "fallback");
        assert!(fallback
            .activation_reasons
            .contains(&"no_visible_tabs".to_string()));
        assert!(fallback
            .activation_reasons
            .contains(&"module_load_error".to_string()));
        assert!(fallback.receipt_fields.contains(&"selectedTab".to_string()));
    }

    #[test]
    fn cartridge_manifest_validation_rejects_unsafe_and_native_shapes() {
        let mut manifest = TabManifest {
            id: "service-card".to_string(),
            title: "Service Card".to_string(),
            description: String::new(),
            icon: String::new(),
            display_name: String::new(),
            order: 9,
            enabled: true,
            admin_only: false,
            visibility: TabVisibility::default(),
            data: serde_json::Value::Null,
            route_prefix: "/api/tabs/service-card".to_string(),
            static_dir: "static".to_string(),
            service_url: None,
            health_route: None,
            install_mode: InstallMode::DynamicCartridge,
        };
        assert!(validate_tab_manifest(&manifest).is_ok());
        manifest.route_prefix = "/wrong".to_string();
        assert!(validate_tab_manifest(&manifest)
            .unwrap_err()
            .contains("routePrefix"));
        manifest.route_prefix = "/api/tabs/service-card".to_string();
        manifest.install_mode = InstallMode::FirstPartyNative;
        assert!(validate_tab_manifest(&manifest)
            .unwrap_err()
            .contains("compiled crown law"));
    }

    #[tokio::test]
    async fn session_route_encodes_admin_and_caduceus_membrane() {
        let temp = test_tab_root("session-law");
        let response = app(AppState {
            tab_root: Arc::new(temp),
        })
        .oneshot(
            Request::builder()
                .uri("/api/session")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let session: AdminSessionReadback = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(session.schema, "coronatio.admin.session.v1");
        assert_eq!(session.session_timeout_seconds, 1800);
        assert_eq!(session.token_header, "X-Admin-Token");
        assert!(session
            .admin_enhanced_filtering
            .iter()
            .any(|filter| filter.topic == "system_stats"
                && filter.admin_fields.contains(&"processes".to_string())));
        assert_eq!(
            session.caduceus_membrane.schema,
            "coronatio.caduceus.membrane.v1"
        );
        assert!(session
            .caduceus_membrane
            .privileged_mutations
            .contains(&"service restart".to_string()));
    }

    #[tokio::test]
    async fn topics_route_replaces_socketio_with_sse_lease_contracts() {
        let temp = test_tab_root("topics-law");
        let response = app(AppState {
            tab_root: Arc::new(temp),
        })
        .oneshot(
            Request::builder()
                .uri("/api/topics")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let topics: TopicCatalogReadback = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(topics.schema, "coronatio.topic-catalog.v1");
        assert!(topics.transport.contains("SSE EventSource"));
        assert!(topics
            .core_topics
            .iter()
            .any(|topic| topic.id == "services.status"));
        assert!(topics
            .admin_topics
            .iter()
            .any(|topic| topic.id == "admin.disk.info" && topic.admin_only));
        let stats = topics
            .tab_topics
            .iter()
            .find(|topic| topic.pane_id == "stats")
            .unwrap();
        assert_eq!(stats.event_route, "/api/stats/events");
        assert_eq!(stats.renew_route, "/api/stats/events/renew");
    }

    #[tokio::test]
    async fn stats_sse_and_monitor_pulse_prove_first_topic() {
        let temp = test_tab_root("stats-sse");
        let router = app(AppState {
            tab_root: Arc::new(temp),
        });
        let pulse_response = router
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/api/monitor/pulse")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(pulse_response.status(), StatusCode::OK);
        let pulse_bytes = axum::body::to_bytes(pulse_response.into_body(), usize::MAX)
            .await
            .unwrap();
        let pulse: MonitorPulseReadback = serde_json::from_slice(&pulse_bytes).unwrap();
        assert_eq!(pulse.schema, "coronatio.monitor-pulse.v1");
        assert_eq!(pulse.topic.id, "stats.system");
        assert_eq!(pulse.first_event.schema, "coronatio.stats.event.v1");
        assert_eq!(pulse.event_route, "/api/stats/events");

        let event_response = router
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/api/stats/events")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(event_response.status(), StatusCode::OK);
        assert_eq!(
            event_response.headers().get(header::CONTENT_TYPE).unwrap(),
            "text/event-stream"
        );
        let event_body = String::from_utf8(
            axum::body::to_bytes(event_response.into_body(), usize::MAX)
                .await
                .unwrap()
                .to_vec(),
        )
        .unwrap();
        assert!(event_body.contains("event: stats.system"));
        assert!(event_body.contains("coronatio.stats.event.v1"));

        let renew_response = router
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/stats/events/renew")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(renew_response.status(), StatusCode::OK);
        let renew: LeaseRenewalReadback = serde_json::from_slice(
            &axum::body::to_bytes(renew_response.into_body(), usize::MAX)
                .await
                .unwrap(),
        )
        .unwrap();
        assert_eq!(renew.schema, "coronatio.stats.events.renewal.v1");
        assert_eq!(renew.topic, "stats.system");
    }

    #[tokio::test]
    async fn route_boundary_returns_json_for_api_misses_and_shell_for_static_fallback() {
        let temp = test_tab_root("boundary-law");
        let router = app(AppState {
            tab_root: Arc::new(temp),
        });
        let api_response = router
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/api/missing-route")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(api_response.status(), StatusCode::NOT_FOUND);
        let api_body = String::from_utf8(
            axum::body::to_bytes(api_response.into_body(), usize::MAX)
                .await
                .unwrap()
                .to_vec(),
        )
        .unwrap();
        assert!(api_body.contains("coronatio.api.error.v1"));
        assert!(!api_body.contains("<html"));

        let shell_response = router
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/some/client/route")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(shell_response.status(), StatusCode::OK);
        let shell_body = String::from_utf8(
            axum::body::to_bytes(shell_response.into_body(), usize::MAX)
                .await
                .unwrap()
                .to_vec(),
        )
        .unwrap();
        assert!(shell_body.contains("data-product=\"Coronatio\""));

        let boundary_response = router
            .oneshot(
                Request::builder()
                    .uri("/api/boundary")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(boundary_response.status(), StatusCode::OK);
        let boundary: BoundaryReadback = serde_json::from_slice(
            &axum::body::to_bytes(boundary_response.into_body(), usize::MAX)
                .await
                .unwrap(),
        )
        .unwrap();
        assert_eq!(boundary.schema, "coronatio.route-boundary.v1");
        assert!(boundary.api_unknown_path_policy.contains("JSON 404"));
    }

    fn test_tab_root(name: &str) -> PathBuf {
        let root =
            std::env::temp_dir().join(format!("coronatio-test-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).unwrap();
        root
    }
}
