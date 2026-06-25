use axum::{
    body::{to_bytes, Body},
    extract::{Path, State},
    http::{header, HeaderMap, Method, StatusCode, Uri},
    response::{Html, IntoResponse, Response},
    routing::{any, get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    env,
    io::{Read, Write},
    net::{SocketAddr, TcpStream},
    path::PathBuf,
    sync::Arc,
    thread,
    time::Duration,
};
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct InstallerReadback {
    schema: String,
    status: String,
    route: String,
    authority: String,
    root_manifest_schema: PremiumRootManifestSchema,
    component_manifest_schema: PremiumComponentManifestSchema,
    file_operation_schema: PremiumFileOperationSchema,
    validation_phases: Vec<InstallerPhase>,
    install_phases: Vec<InstallerPhase>,
    rollback_law: RollbackLaw,
    lifecycle_law: Vec<InstallerLifecycleLaw>,
    lane_mapping: Vec<InstallerLaneMapping>,
    first_missing_live_signal: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct PremiumRootManifestSchema {
    required_fields: Vec<String>,
    config_fields: Vec<String>,
    file_sections: Vec<String>,
    sample_source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct PremiumComponentManifestSchema {
    loci: Vec<String>,
    fields: Vec<String>,
    operation_types: Vec<String>,
    blueprint_marker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct PremiumFileOperationSchema {
    source_field: String,
    target_field: String,
    operation_type_field: String,
    identifier_field: String,
    marker_field: String,
    description_field: String,
    supported_operations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct InstallerPhase {
    id: String,
    sequence: u64,
    source_law: String,
    coronatio_contract: String,
    mutation_authority: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct RollbackLaw {
    schema: String,
    order: Vec<String>,
    config_restore: String,
    file_operation_reversal: String,
    service_state_restore: String,
    batch_restore: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct InstallerLifecycleLaw {
    action: String,
    sequence: Vec<String>,
    post_build_policy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct InstallerLaneMapping {
    install_mode: InstallMode,
    accepted_package: String,
    post_install_requirement: String,
    rejected_shape: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct RegistryTransactionReadback {
    schema: String,
    status: String,
    route: String,
    source_contract: String,
    transaction_sequence: Vec<RegistryTransactionPhase>,
    deep_merge_law: DeepMergeLaw,
    starred_tab_law: StarredTabLaw,
    validation_law: ConfigValidationLaw,
    persistence_law: ConfigPersistenceLaw,
    rollback_law: ConfigRollbackLaw,
    first_missing_live_signal: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct RegistryTransactionPhase {
    sequence: u64,
    id: String,
    source_law: String,
    coronatio_contract: String,
    mutation_authority: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct DeepMergeLaw {
    object_merge: String,
    scalar_merge: String,
    array_merge: String,
    tab_merge: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct StarredTabLaw {
    source_behavior: String,
    preservation_rule: String,
    invalid_starred_resolution: String,
    transaction_requirement: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct ConfigValidationLaw {
    syntax_gate: String,
    factory_fallback_gate: String,
    temp_validation: String,
    failure_posture: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct ConfigPersistenceLaw {
    backup_policy: String,
    write_policy: String,
    permission_restore: String,
    missing_config_fallback: String,
    read_only_factory_posture: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct ConfigRollbackLaw {
    backup_restore: String,
    patch_revert: String,
    complete_tab_removal: String,
    mismatch_policy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct ServiceDataReadback {
    schema: String,
    status: String,
    route: String,
    portal_schema: PortalSchema,
    service_card_schema: ServiceCardSchema,
    monitor_topics: Vec<MonitorTopicLaw>,
    broadcast_law: BroadcastLaw,
    admin_field_law: Vec<AdminFieldFilter>,
    first_missing_live_signal: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct PortalSchema {
    source_path: String,
    fields: Vec<String>,
    required_fields: Vec<String>,
    portal_types: Vec<String>,
    validation_rules: Vec<ValidationRule>,
    factory_portal_law: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct ServiceCardSchema {
    source_paths: Vec<String>,
    fields: Vec<String>,
    systemd_resolution: String,
    script_managed_resolution: String,
    enabled_cache_policy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct MonitorTopicLaw {
    topic: String,
    source_monitor: String,
    cadence_source: String,
    payload_fields: Vec<String>,
    admin_only: bool,
    admin_fields: Vec<String>,
    change_rule: String,
    coronatio_contract: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct BroadcastLaw {
    transport_replacement: String,
    regular_delivery: String,
    admin_delivery: String,
    change_detection: String,
    ui_state_law: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct FrontendStorageReadback {
    schema: String,
    status: String,
    route: String,
    quarry_sources: Vec<String>,
    persisted_stores: Vec<PersistedStoreLaw>,
    persistence_fields: Vec<PersistedFieldLaw>,
    debounce_law: Vec<DebounceLaw>,
    stale_state_law: Vec<StaleStateLaw>,
    coronatio_ownership: Vec<StateOwnershipLaw>,
    migration_path: Vec<String>,
    forbidden_persistence: Vec<String>,
    first_missing_live_signal: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct PersistedStoreLaw {
    store_name: String,
    storage_key: String,
    source_path: String,
    persisted_fields: Vec<String>,
    boundary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct PersistedFieldLaw {
    field: String,
    old_source: String,
    old_behavior: String,
    coronatio_owner: String,
    migration_rule: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct DebounceLaw {
    source: String,
    interval_ms: u64,
    purpose: String,
    coronatio_rule: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct StaleStateLaw {
    source: String,
    stale_condition: String,
    old_recovery: String,
    coronatio_rule: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct StateOwnershipLaw {
    state_family: String,
    owner: String,
    reason: String,
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
        .route("/api/registry/transaction", get(registry_transaction_route))
        .route("/api/startup", get(startup_route))
        .route("/api/lanes", get(lane_policy_route))
        .route("/api/fallback", get(fallback_route))
        .route("/api/session", get(session_route))
        .route(
            "/api/admin/session",
            get(session_route).post(session_renew_route),
        )
        .route("/api/caduceus/status", get(caduceus_status_route))
        .route(
            "/api/caduceus/update/check",
            post(caduceus_update_check_route),
        )
        .route("/api/caduceus/update/now", post(caduceus_update_now_route))
        .route(
            "/api/caduceus/receipts/latest",
            get(caduceus_receipts_latest_route),
        )
        .route("/api/topics", get(topics_route))
        .route("/api/monitor/pulse", get(monitor_pulse_route))
        .route("/api/services/data", get(service_data_route))
        .route("/api/frontend/storage", get(frontend_storage_route))
        .route("/api/boundary", get(boundary_route))
        .route("/api/installer", get(installer_route))
        .route("/api/stats/events", get(stats_events_route))
        .route("/api/stats/events/renew", post(stats_events_renew_route))
        .route("/api/stats", get(stats_route))
        .route("/api/coronatio/tabs", get(tabs_route))
        .route(
            "/api/coronatio/tabs/:tab_id/manifest",
            get(tab_manifest_route),
        )
        .route("/api/tabs", any(legacy_homeserver_proxy_route))
        .route("/api/*path", any(legacy_homeserver_proxy_route))
        .route("/assets/*path", any(legacy_homeserver_proxy_route))
        .route("/socket.io/*path", any(legacy_homeserver_proxy_route))
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
            "/api/boundary".to_string(),
            "/api/installer".to_string(),
            "/api/stats/events".to_string(),
            "/api/stats/events/renew".to_string(),
            "/api/stats".to_string(),
            "/api/coronatio/tabs".to_string(),
            "/api/coronatio/tabs/:tab_id/manifest".to_string(),
            "/api/tabs (legacy HomeServer proxy)".to_string(),
            "/api/*path (legacy HomeServer proxy)".to_string(),
            "/assets/*path (legacy HomeServer proxy)".to_string(),
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

async fn legacy_homeserver_proxy_route(
    method: Method,
    uri: Uri,
    headers: HeaderMap,
    body: Body,
) -> impl IntoResponse {
    match legacy_homeserver_proxy_response(method, uri, headers, body).await {
        Ok(response) => response,
        Err(error) => (
            StatusCode::BAD_GATEWAY,
            Json(serde_json::json!({
                "schema": "coronatio.legacy-homeserver-proxy.error.v1",
                "ok": false,
                "error": error,
                "authority": "Coronatio Rust host preserves the Flask/React HomeServer UX by proxying legacy assets and API requests"
            })),
        )
            .into_response(),
    }
}

async fn legacy_homeserver_proxy_response(
    method: Method,
    uri: Uri,
    headers: HeaderMap,
    body: Body,
) -> Result<Response, String> {
    let path_and_query = uri
        .path_and_query()
        .map(|value| value.as_str().to_string())
        .unwrap_or_else(|| uri.path().to_string());
    let body_bytes = to_bytes(body, 16 * 1024 * 1024)
        .await
        .map_err(|error| format!("read request body: {error}"))?;
    let accept = header_value(&headers, header::ACCEPT.as_str());
    let content_type = header_value(&headers, header::CONTENT_TYPE.as_str());
    let authorization = header_value(&headers, header::AUTHORIZATION.as_str());
    let upstream = tokio::task::spawn_blocking(move || {
        legacy_homeserver_http_request(
            method.as_str(),
            &path_and_query,
            &body_bytes,
            accept.as_deref(),
            content_type.as_deref(),
            authorization.as_deref(),
        )
    })
    .await
    .map_err(|error| format!("legacy proxy task: {error}"))??;
    let mut builder = Response::builder().status(upstream.status);
    for (name, value) in upstream.headers {
        builder = builder.header(name, value);
    }
    builder
        .body(Body::from(upstream.body))
        .map_err(|error| format!("build response: {error}"))
}

struct LegacyHttpResponse {
    status: StatusCode,
    headers: Vec<(String, String)>,
    body: Vec<u8>,
}

fn header_value(headers: &HeaderMap, name: &str) -> Option<String> {
    headers
        .get(name)
        .and_then(|value| value.to_str().ok())
        .map(ToString::to_string)
}

fn legacy_homeserver_proxy_host() -> String {
    env::var("CORONATIO_LEGACY_HOMESERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string())
}

fn legacy_homeserver_proxy_port() -> u16 {
    env::var("CORONATIO_LEGACY_HOMESERVER_PORT")
        .ok()
        .and_then(|value| value.parse::<u16>().ok())
        .unwrap_or(8001)
}

fn legacy_homeserver_http_request(
    method: &str,
    path_and_query: &str,
    body: &[u8],
    accept: Option<&str>,
    content_type: Option<&str>,
    authorization: Option<&str>,
) -> Result<LegacyHttpResponse, String> {
    let mut stream = TcpStream::connect((
        legacy_homeserver_proxy_host().as_str(),
        legacy_homeserver_proxy_port(),
    ))
    .map_err(|error| format!("connect legacy HomeServer: {error}"))?;
    stream
        .set_read_timeout(Some(Duration::from_secs(20)))
        .map_err(|error| format!("set read timeout: {error}"))?;
    stream
        .set_write_timeout(Some(Duration::from_secs(20)))
        .map_err(|error| format!("set write timeout: {error}"))?;
    let mut request = format!(
        "{method} {path_and_query} HTTP/1.1\r\nHost: home.arpa\r\nConnection: close\r\nContent-Length: {}\r\n",
        body.len()
    );
    if let Some(value) = accept {
        request.push_str(&format!("Accept: {value}\r\n"));
    }
    if let Some(value) = content_type {
        request.push_str(&format!("Content-Type: {value}\r\n"));
    }
    if let Some(value) = authorization {
        request.push_str(&format!("Authorization: {value}\r\n"));
    }
    request.push_str("\r\n");
    stream
        .write_all(request.as_bytes())
        .map_err(|error| format!("write legacy request headers: {error}"))?;
    if !body.is_empty() {
        stream
            .write_all(body)
            .map_err(|error| format!("write legacy request body: {error}"))?;
    }
    let mut raw = Vec::new();
    stream
        .read_to_end(&mut raw)
        .map_err(|error| format!("read legacy response: {error}"))?;
    let split = raw
        .windows(4)
        .position(|window| window == b"\r\n\r\n")
        .ok_or_else(|| "legacy response missing header boundary".to_string())?;
    let (head, body_part) = raw.split_at(split + 4);
    let head_text = String::from_utf8_lossy(head);
    let mut lines = head_text.split("\r\n");
    let status_line = lines
        .next()
        .ok_or_else(|| "legacy response missing status line".to_string())?;
    let status_code = status_line
        .split_whitespace()
        .nth(1)
        .ok_or_else(|| format!("legacy response malformed status line: {status_line}"))?
        .parse::<u16>()
        .map_err(|error| format!("legacy response status parse: {error}"))?;
    let status = StatusCode::from_u16(status_code)
        .map_err(|error| format!("legacy response status conversion: {error}"))?;
    let mut response_headers = Vec::new();
    for line in lines {
        if line.is_empty() {
            continue;
        }
        if let Some((name, value)) = line.split_once(':') {
            let normalized = name.to_ascii_lowercase();
            if matches!(
                normalized.as_str(),
                "content-type" | "cache-control" | "etag" | "last-modified"
            ) {
                response_headers.push((normalized, value.trim().to_string()));
            }
        }
    }
    Ok(LegacyHttpResponse {
        status,
        headers: response_headers,
        body: body_part.to_vec(),
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

fn registry_transaction_readback() -> RegistryTransactionReadback {
    RegistryTransactionReadback {
        schema: "coronatio.registry.transaction.v1".to_string(),
        status: "contract-only".to_string(),
        route: "/api/registry/transaction".to_string(),
        source_contract: "premium/utils/config_manager.py apply_config_patch, deep_merge, deep_merge_tabs, validate_config_with_factory_fallback, restore_backup, revert_config_patch".to_string(),
        transaction_sequence: registry_transaction_phases(),
        deep_merge_law: DeepMergeLaw {
            object_merge: "when target and patch values are objects, merge recursively".to_string(),
            scalar_merge: "when either side is not an object, patch value replaces target value".to_string(),
            array_merge: "arrays are scalar values in this quarry law and are replaced, not appended".to_string(),
            tab_merge: "tabs object uses deep_merge_tabs so tab entries merge while starred is preserved and restored after new tab keys".to_string(),
        },
        starred_tab_law: StarredTabLaw {
            source_behavior: "deep_merge_tabs pops existing tabs.starred before merging source tabs and restores it at the end".to_string(),
            preservation_rule: "a package patch may add or update tab objects without displacing the existing starred pointer".to_string(),
            invalid_starred_resolution: "registry/startup law resolves invalid, hidden, disabled, or missing starred to first visible tab then fallback".to_string(),
            transaction_requirement: "later live mutation must preserve starred unless the patch explicitly carries an authorized starred transition".to_string(),
        },
        validation_law: ConfigValidationLaw {
            syntax_gate: "JSON syntax must parse before merge and before write".to_string(),
            factory_fallback_gate: "factoryFallback.sh equivalent validates candidate config and rejects .factory fallback output".to_string(),
            temp_validation: "candidate is written to a temp path, temporarily validated through the factory fallback path, then original config is restored before promote".to_string(),
            failure_posture: "validation failure removes candidate temp file and leaves current config standing".to_string(),
        },
        persistence_law: ConfigPersistenceLaw {
            backup_policy: "create timestamped backup before mutation when current config exists".to_string(),
            write_policy: "write merged candidate to temp config, validate it, then move temp into the live config path".to_string(),
            permission_restore: "after promote or restore, set owner www-data:www-data and mode 664 in the old host; Coronatio records desired owner/mode and leaves privileged chmod/chown to Caduceus".to_string(),
            missing_config_fallback: "if live config is absent, read /etc/homeserver.factory; if absent, use minimal tabs/global.cors.allowed_origins structure".to_string(),
            read_only_factory_posture: "factory fallback is source material for a candidate, not a durable replacement for the live config unless validation and promotion succeed".to_string(),
        },
        rollback_law: ConfigRollbackLaw {
            backup_restore: "restore backup to target path and restore permissions".to_string(),
            patch_revert: "remove keys matching patch values; nested objects recurse".to_string(),
            complete_tab_removal: "under tabs, patch-owned tab keys are removed as whole tab entries during revert".to_string(),
            mismatch_policy: "if current value differs from the patch value, do not remove it; report mismatch rather than deleting user state".to_string(),
        },
        first_missing_live_signal: "Caduceus registry transaction actuator is not wired; Coronatio does not write homeserver.json, run factoryFallback.sh, chown, chmod, or move temp configs".to_string(),
    }
}

fn registry_transaction_phases() -> Vec<RegistryTransactionPhase> {
    vec![
        registry_transaction_phase(1, "backup-current", "create_backup copies current config to /tmp/<name>.installer_backup.<timestamp>", "record backup policy and receipt fields before mutation", "Caduceus later"),
        registry_transaction_phase(2, "load-current-or-factory", "read homeserver.json, else /etc/homeserver.factory, else minimal valid structure", "classify live config, factory config, or minimal recovery candidate", "read-only Coronatio contract now; Caduceus later"),
        registry_transaction_phase(3, "deep-merge-patch", "deep_merge recursively merges objects and replaces scalars; tabs.starred is popped and restored", "merge patch into candidate while preserving default route law", "pure typed transaction primitive later"),
        registry_transaction_phase(4, "write-temp-candidate", "json.dump candidate to homeserver.json.temp", "candidate lives outside live path until validation succeeds", "Caduceus later"),
        registry_transaction_phase(5, "validate-candidate", "validate_config_with_factory_fallback(temp_config) rejects factory fallback output", "candidate must pass syntax and factory fallback validation before promotion", "Caduceus later"),
        registry_transaction_phase(6, "atomic-promote", "shutil.move(temp_config, homeserver_config_path)", "promote only the validated candidate into the live registry path", "Caduceus later"),
        registry_transaction_phase(7, "restore-owner-mode", "chown www-data:www-data and chmod 664", "permission restoration is part of the transaction receipt, not an afterthought", "Caduceus only"),
    ]
}

fn registry_transaction_phase(
    sequence: u64,
    id: &str,
    source_law: &str,
    coronatio_contract: &str,
    mutation_authority: &str,
) -> RegistryTransactionPhase {
    RegistryTransactionPhase {
        sequence,
        id: id.to_string(),
        source_law: source_law.to_string(),
        coronatio_contract: coronatio_contract.to_string(),
        mutation_authority: mutation_authority.to_string(),
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

fn frontend_storage_readback() -> FrontendStorageReadback {
    FrontendStorageReadback {
        schema: "coronatio.frontend-storage.contract.v1".to_string(),
        status: "contract-only".to_string(),
        route: "/api/frontend/storage".to_string(),
        quarry_sources: vec![
            "src/store/index.ts".to_string(),
            "src/store/slices/themeSlice.ts".to_string(),
            "src/store/slices/tabSlice.ts".to_string(),
            "src/store/slices/visibilitySlice.ts".to_string(),
            "src/store/slices/favoriteSlice.ts".to_string(),
            "src/store/slices/startupSlice.ts".to_string(),
            "src/App.tsx".to_string(),
            "src/api/auth.ts".to_string(),
        ],
        persisted_stores: vec![
            PersistedStoreLaw {
                store_name: "main zustand store".to_string(),
                storage_key: "homeserver-store".to_string(),
                source_path: "src/store/index.ts persist(partialize)".to_string(),
                persisted_fields: vec!["theme", "visibility", "starredTab", "isInitialized", "tabs", "activeTab"].into_iter().map(String::from).collect(),
                boundary: "mixed server truth and browser preference; Coronatio splits this before migration".to_string(),
            },
            PersistedStoreLaw {
                store_name: "auth zustand store".to_string(),
                storage_key: "auth-storage".to_string(),
                source_path: "src/api/auth.ts persist(partialize)".to_string(),
                persisted_fields: vec!["isAdmin"].into_iter().map(String::from).collect(),
                boundary: "browser remembrance only; privileged authority remains token/session receipt, never localStorage".to_string(),
            },
            PersistedStoreLaw {
                store_name: "theme data cache".to_string(),
                storage_key: "themeData".to_string(),
                source_path: "src/store/slices/themeSlice.ts THEME_DATA_STORAGE_KEY".to_string(),
                persisted_fields: vec!["themeData"].into_iter().map(String::from).collect(),
                boundary: "browser cosmetic cache; server theme catalog remains authority".to_string(),
            },
        ],
        persistence_fields: frontend_persistence_fields(),
        debounce_law: vec![
            DebounceLaw {
                source: "src/store/index.ts debouncedSetItem".to_string(),
                interval_ms: 500,
                purpose: "reduce repeated homeserver-store localStorage writes and skip unchanged serialized state".to_string(),
                coronatio_rule: "write browser preference snapshots only after settle; server-owned state is not hidden behind browser debounce".to_string(),
            },
            DebounceLaw {
                source: "src/App.tsx loadTablet duplicate-load debounce".to_string(),
                interval_ms: 500,
                purpose: "avoid immediately reloading the same tablet unless recovering from fallback".to_string(),
                coronatio_rule: "pane loading may debounce duplicate dynamic-cartridge loads; current route state remains explicit".to_string(),
            },
            DebounceLaw {
                source: "src/App.tsx tablet load timeout".to_string(),
                interval_ms: 15000,
                purpose: "show fallback when a tablet module stalls".to_string(),
                coronatio_rule: "dynamic cartridge timeout becomes a typed fallback receipt, not silent local state drift".to_string(),
            },
            DebounceLaw {
                source: "src/store/slices/startupSlice.ts tab config fetch timeout".to_string(),
                interval_ms: 7000,
                purpose: "abort stale /api/tabs startup config fetch and retry before default fallback".to_string(),
                coronatio_rule: "startup config has bounded fetch/retry receipts; stale local storage never overrides admitted registry".to_string(),
            },
        ],
        stale_state_law: vec![
            StaleStateLaw {
                source: "src/store/index.ts storage.getItem".to_string(),
                stale_condition: "invalid JSON or parse failure in homeserver-store".to_string(),
                old_recovery: "log parse error and return null".to_string(),
                coronatio_rule: "ignore malformed browser snapshot and reload server registry plus safe defaults".to_string(),
            },
            StaleStateLaw {
                source: "src/utils/bootstrap.ts determineInitialTab".to_string(),
                stale_condition: "starred tab missing, hidden, disabled, admin-only without admin, or fallback-only".to_string(),
                old_recovery: "choose visible enabled server starred tab, else first visible non-admin/admin-allowed tab, else fallback".to_string(),
                coronatio_rule: "server registry decides first pane; browser activeTab is advisory and clipped to visible admitted panes".to_string(),
            },
            StaleStateLaw {
                source: "src/App.tsx stale load detection".to_string(),
                stale_condition: "async tablet load completes after a newer load id or after unmount".to_string(),
                old_recovery: "discard stale module result".to_string(),
                coronatio_rule: "dynamic cartridge load receipts carry generation id and stale completions do not mutate visible pane".to_string(),
            },
            StaleStateLaw {
                source: "src/store/slices/visibilitySlice.ts updateTabVisibility".to_string(),
                stale_condition: "backend visibility update fails".to_string(),
                old_recovery: "revert local visibility to previous value and recalculate starred fallback".to_string(),
                coronatio_rule: "server visibility transaction is authority; browser optimistic state rolls back on failed receipt".to_string(),
            },
        ],
        coronatio_ownership: vec![
            StateOwnershipLaw { state_family: "tabs".to_string(), owner: "server registry /api/registry and dynamic cartridge manifests".to_string(), reason: "tab catalog and native/dynamic lane admission are product truth".to_string() },
            StateOwnershipLaw { state_family: "visibility".to_string(), owner: "server registry transaction".to_string(), reason: "visibility controls appliance access and fallback law".to_string() },
            StateOwnershipLaw { state_family: "starredTab".to_string(), owner: "server registry with browser advisory bootstrap".to_string(), reason: "first pane must survive device changes and visibility changes".to_string() },
            StateOwnershipLaw { state_family: "activeTab".to_string(), owner: "browser session preference clipped by server registry".to_string(), reason: "current pane is local navigation, not product configuration".to_string() },
            StateOwnershipLaw { state_family: "theme".to_string(), owner: "browser preference using server theme catalog".to_string(), reason: "theme is cosmetic unless a server profile later claims it".to_string() },
            StateOwnershipLaw { state_family: "isInitialized".to_string(), owner: "runtime startup phase receipt".to_string(), reason: "initialization is live process state and must not be trusted from stale storage".to_string() },
            StateOwnershipLaw { state_family: "isAdmin".to_string(), owner: "Caduceus/session token receipt".to_string(), reason: "admin authority cannot be granted by localStorage".to_string() },
        ],
        migration_path: vec![
            "read existing homeserver-store snapshot if present".to_string(),
            "drop malformed JSON and all credential/token-shaped values".to_string(),
            "accept theme only when present in server theme catalog".to_string(),
            "accept activeTab only when admitted, visible, enabled, and accessible to current role".to_string(),
            "migrate starredTab to server registry only when visible enabled non-admin tab or fallback".to_string(),
            "migrate visibility/tabs from server registry, not from stale browser snapshot".to_string(),
            "write new Coronatio browser preference key after successful server readback".to_string(),
            "leave old homeserver-store readable for one migration pass, then ignore".to_string(),
        ],
        forbidden_persistence: vec![
            "adminToken".to_string(),
            "PIN".to_string(),
            "password".to_string(),
            "session token".to_string(),
            "API key".to_string(),
            "credential-bearing theme or tab data".to_string(),
        ],
        first_missing_live_signal: "Coronatio storage migration adapter and browser preference key are not wired; this tranche exposes the contract only".to_string(),
    }
}

fn frontend_persistence_fields() -> Vec<PersistedFieldLaw> {
    vec![
        field_law(
            "theme",
            "src/store/index.ts + themeSlice.ts",
            "persisted in homeserver-store and themeData cache",
            "browser preference",
            "preserve if theme exists in server catalog, otherwise default",
        ),
        field_law(
            "visibility",
            "src/store/index.ts + visibilitySlice.ts",
            "persisted locally but also posted to /tabs/visibility",
            "server registry transaction",
            "reload from server; local snapshot is fallback evidence only",
        ),
        field_law(
            "starredTab",
            "src/store/index.ts + favoriteSlice.ts",
            "persisted locally and posted to /setstarredtab",
            "server registry",
            "preserve only if visible/enabled and role-accessible, else first visible tab/fallback",
        ),
        field_law(
            "isInitialized",
            "src/store/index.ts + tabSlice.ts",
            "persisted as initialization flag",
            "startup receipt",
            "do not trust from storage; recompute during startup",
        ),
        field_law(
            "tabs",
            "src/store/index.ts + startupSlice.ts",
            "persisted full tab state",
            "server registry /api/tabs",
            "treat browser copy as stale cache; server response wins",
        ),
        field_law(
            "activeTab",
            "src/store/index.ts + App.tsx",
            "persisted current navigation target",
            "browser session preference",
            "clip to admitted visible tab; never load hidden/disabled/admin-forbidden tab",
        ),
        field_law(
            "isAdmin",
            "src/api/auth.ts auth-storage",
            "persisted boolean",
            "Caduceus/session token receipt",
            "discard as authority; require valid token/session proof",
        ),
        field_law(
            "themeData",
            "src/store/slices/themeSlice.ts",
            "persisted theme object under themeData",
            "browser cosmetic cache",
            "validate hex color schema and catalog membership before reuse",
        ),
    ]
}

fn field_law(
    field: &str,
    old_source: &str,
    old_behavior: &str,
    coronatio_owner: &str,
    migration_rule: &str,
) -> PersistedFieldLaw {
    PersistedFieldLaw {
        field: field.to_string(),
        old_source: old_source.to_string(),
        old_behavior: old_behavior.to_string(),
        coronatio_owner: coronatio_owner.to_string(),
        migration_rule: migration_rule.to_string(),
    }
}

fn service_data_readback() -> ServiceDataReadback {
    ServiceDataReadback {
        schema: "coronatio.service-data.contract.v1".to_string(),
        status: "contract-only".to_string(),
        route: "/api/services/data".to_string(),
        portal_schema: PortalSchema {
            source_path: "homeserver.json tabs.portals.data.portals[]".to_string(),
            fields: vec!["name", "description", "services", "type", "port", "localURL", "remoteURL"].into_iter().map(String::from).collect(),
            required_fields: vec!["name", "description", "localURL"].into_iter().map(String::from).collect(),
            portal_types: vec!["systemd", "script", "link"].into_iter().map(String::from).collect(),
            validation_rules: vec![
                ValidationRule { field: "name".to_string(), rule: "non-empty unique portal name".to_string() },
                ValidationRule { field: "services".to_string(), rule: "non-link portals require at least one service".to_string() },
                ValidationRule { field: "port".to_string(), rule: "non-link portals require unique integer port 1..65535".to_string() },
                ValidationRule { field: "localURL".to_string(), rule: "non-empty local URL is the primary appliance link".to_string() },
                ValidationRule { field: "factory".to_string(), rule: "factory portals are read-only and cannot be deleted by custom portal mutation".to_string() },
            ],
            factory_portal_law: "factory portals are loaded from factory config for readback and protected from deletion; custom portals mutate only through later Caduceus config transactions".to_string(),
        },
        service_card_schema: ServiceCardSchema {
            source_paths: vec![
                "backend/portals/routes.py /api/portals".to_string(),
                "backend/monitors/services.py ServicesMonitor.collect_status".to_string(),
                "backend/utils/utils.py get_service_status".to_string(),
            ],
            fields: vec!["name", "systemdName", "isEnabled", "isActive", "status", "statusDetails", "isScriptManaged", "port", "needsReboot"].into_iter().map(String::from).collect(),
            systemd_resolution: "portal services resolve through exact service mapping, normalized name mapping, then .service suffix; active and enabled use systemctl".to_string(),
            script_managed_resolution: "script-managed portal services use port reachability as running proxy, assume enabled, and mark needsReboot true".to_string(),
            enabled_cache_policy: "is-enabled results are cached for 60 seconds in the old monitor; Coronatio records cache TTL as contract, not live systemctl state".to_string(),
        },
        monitor_topics: monitor_topic_laws(),
        broadcast_law: BroadcastLaw {
            transport_replacement: "old eventlet/Socket.IO broadcasters become Coronatio topic contracts and SSE/readback routes".to_string(),
            regular_delivery: "regular clients receive non-admin payload fields for core service, power, internet, tailscale, vpn, sync, and hard-drive-test topics".to_string(),
            admin_delivery: "admin sessions may receive registered admin fields and admin-only disk/system topics through Caduceus-gated capability".to_string(),
            change_detection: "topic-specific comparison rules decide broadcast eligibility; realtime power/system always pulse, services compare set/status/enabled, sync compares status/job/progress/keepalive".to_string(),
            ui_state_law: "monitor data becomes current service cards, indicator chips, and pane snapshots; unavailable collectors surface firstMissingSignal instead of fake green".to_string(),
        },
        admin_field_law: admin_field_filters(),
        first_missing_live_signal: "service collectors and monitor broadcasters are not wired; Coronatio does not run systemctl, ping, tailscale, vpn, disk, rsync, smartctl, or power sensors in this tranche".to_string(),
    }
}

fn monitor_topic_laws() -> Vec<MonitorTopicLaw> {
    vec![
        monitor_topic("services.status", "ServicesMonitor.broadcast_status", "SERVICES_CHECK_INTERVAL", vec!["name", "systemdName", "isActive", "status", "statusDetails", "isScriptManaged", "port", "needsReboot"], false, vec!["isEnabled"], "broadcast when service count/name/status/enabled state changes", "service cards and portal currentness readback"),
        monitor_topic("power.status", "PowerMonitor.broadcast_power_data", "POWER_SAMPLE_INTERVAL", vec!["current", "historical", "unit", "timestamp"], false, vec![], "always broadcast realtime power samples", "power indicator/currentness readback"),
        monitor_topic("system.stats", "SystemStatsMonitor.broadcast_stats", "STATS_INTERVAL", vec!["load", "cpu", "memory", "disk", "network", "timestamp"], false, vec!["processes", "users", "networkConnections"], "always broadcast realtime system stats", "stats pane snapshot/SSE payload"),
        monitor_topic("internet.status", "InternetStatusMonitor.broadcast_status", "INTERNET_CHECK_INTERVAL", vec!["status", "timestamp"], false, vec!["publicIp", "ipDetails", "dnsServers"], "broadcast on connectivity/public IP/DNS/error-validity changes", "internet indicator readback"),
        monitor_topic("tailscale.status", "TailscaleMonitor.broadcast_status", "TAILSCALE_CHECK_INTERVAL", vec!["status", "interface", "timestamp"], false, vec!["ip", "tailnet", "isEnabled", "loginUrl"], "broadcast on status/interface/admin-field/login URL changes", "tailscale indicator readback"),
        monitor_topic("vpn.status", "VPNMonitor.broadcast_status", "VPN_CHECK_INTERVAL", vec!["vpnStatus", "transmissionStatus", "timestamp"], false, vec!["connectionDetails", "credentials", "isEnabled"], "broadcast on vpn/transmission/enabled changes", "vpn indicator readback with credentials redacted"),
        monitor_topic("sync.status", "sync_monitor.broadcast_status", "2 seconds", vec!["status", "id", "progress", "timestamp", "message"], false, vec![], "broadcast on status/job/progress >=10% or working keepalive", "sync indicator and admin action readback"),
        monitor_topic("hard-drive-test.status", "HardDriveTestMonitor.broadcast_status", "DRIVE_TEST_INTERVAL", vec!["status", "device", "testType", "progress", "message", "timestamp"], false, vec![], "broadcast starting/done and working keepalive", "drive-test indicator/action readback"),
        monitor_topic("admin.disk.info", "DiskMonitor.broadcast_disk_info", "DISK_CHECK_INTERVAL", vec!["blockDevices", "diskUsage", "encryptionInfo", "nasCompatibleDevices", "timestamp"], true, vec![], "broadcast on device/encryption/NAS/mount/filesystem/UUID/usage/error changes or 5 minute pulse", "admin disk appliance readback"),
        monitor_topic("admin.system", "SystemStatsMonitor.broadcast_admin_stats", "ADMIN_STATS_INTERVAL", vec!["system stats plus admin detail fields"], true, vec![], "admin-only realtime system pulse", "admin diagnostics readback"),
    ]
}

fn monitor_topic(
    topic: &str,
    source_monitor: &str,
    cadence_source: &str,
    payload_fields: Vec<&str>,
    admin_only: bool,
    admin_fields: Vec<&str>,
    change_rule: &str,
    coronatio_contract: &str,
) -> MonitorTopicLaw {
    MonitorTopicLaw {
        topic: topic.to_string(),
        source_monitor: source_monitor.to_string(),
        cadence_source: cadence_source.to_string(),
        payload_fields: payload_fields.into_iter().map(String::from).collect(),
        admin_only,
        admin_fields: admin_fields.into_iter().map(String::from).collect(),
        change_rule: change_rule.to_string(),
        coronatio_contract: coronatio_contract.to_string(),
    }
}

fn boundary_readback() -> BoundaryReadback {
    BoundaryReadback {
        schema: "coronatio.route-boundary.v1".to_string(),
        api_unknown_path_policy: "legacy HomeServer /api/* paths proxy to the Flask/React authority so the served UX is identical; Coronatio-native contract routes stay exact under /api/coronatio/* and named /api routes".to_string(),
        static_shell_policy: "non-API unknown GET paths return the exact Flask/React HomeServer shell for client-side routing".to_string(),
        cartridge_static_policy: "/tabs/<tab-id>/... is served from the configured tab root through safe tab ids and manifest validation; legacy /assets/* proxy to HomeServer build assets".to_string(),
        cors_source: "homeserver.json global.cors.allowed_origins becomes Coronatio config law in the later config tranche".to_string(),
        premium_blueprint_replacement: "dynamic Flask blueprint injection is replaced by dynamic-cartridge, source-injection-recompile, or first-party-native lanes".to_string(),
    }
}

fn installer_readback() -> InstallerReadback {
    InstallerReadback {
        schema: "coronatio.installer.contract.v1".to_string(),
        status: "contract-only".to_string(),
        route: "/api/installer".to_string(),
        authority: "Coronatio exposes typed installer law readbacks; live host mutation remains behind a later Caduceus actuator membrane".to_string(),
        root_manifest_schema: PremiumRootManifestSchema {
            required_fields: vec!["name", "version", "config", "files"].into_iter().map(String::from).collect(),
            config_fields: vec!["repository.url", "repository.branch", "git_managed"].into_iter().map(String::from).collect(),
            file_sections: vec!["backend", "frontend", "permissions", "system", "config", "readme", "license"].into_iter().map(String::from).collect(),
            sample_source: "premium/youtube/index.json".to_string(),
        },
        component_manifest_schema: PremiumComponentManifestSchema {
            loci: vec!["frontend/index.json", "backend/index.json"].into_iter().map(String::from).collect(),
            fields: vec!["name", "version", "files[]", "source", "target", "type", "identifier", "marker", "description"].into_iter().map(String::from).collect(),
            operation_types: vec!["copy", "append", "symlink"].into_iter().map(String::from).collect(),
            blueprint_marker: "PREMIUM TAB BLUEPRINTS".to_string(),
        },
        file_operation_schema: PremiumFileOperationSchema {
            source_field: "source path relative to package root".to_string(),
            target_field: "absolute old-host target or Coronatio lane target declared by later actuator".to_string(),
            operation_type_field: "type defaults to copy; append uses marker/identifier".to_string(),
            identifier_field: "tab/package identifier for injected blocks and rollback".to_string(),
            marker_field: "append insertion marker such as PREMIUM TAB BLUEPRINTS".to_string(),
            description_field: "human receipt text for operation intent".to_string(),
            supported_operations: vec!["copy", "append", "symlink"].into_iter().map(String::from).collect(),
        },
        validation_phases: installer_validation_phases(),
        install_phases: installer_install_phases(),
        rollback_law: installer_rollback_law(),
        lifecycle_law: installer_lifecycle_law(),
        lane_mapping: installer_lane_mapping(),
        first_missing_live_signal: "Caduceus installer actuator and receipt ledger are not wired; no package files, dependencies, config, build, or service state are mutated by Coronatio".to_string(),
    }
}

fn installer_validation_phases() -> Vec<InstallerPhase> {
    vec![
        installer_phase(1, "current-config-validation", "validate_config_with_factory_fallback before package work", "read current Coronatio registry/config and reject invalid baseline before staging mutation", "read-only Coronatio contract now; Caduceus later"),
        installer_phase(2, "package-manifest-validation", "validate_package_manifest over root/component manifests", "require root package schema and frontend/backend operation schema before admission", "read-only Coronatio contract now; Caduceus later"),
        installer_phase(3, "name-collision", "check_name_collision unless reinstall/batch post-build restoration skips installed check", "reject duplicate visible tab/package identity unless reinstall authority is explicit", "read-only Coronatio contract now; Caduceus later"),
        installer_phase(4, "version-conflict", "SemanticVersionChecker.validate_premium_tab_dependencies", "surface version conflicts as typed blockers before file/package mutation", "read-only Coronatio contract now; Caduceus later"),
        installer_phase(5, "dependency-validation", "backend requirements, frontend package patch, and system dependencies are detected before install", "classify dependency families without installing them in Coronatio", "read-only Coronatio contract now; Caduceus later"),
    ]
}

fn installer_install_phases() -> Vec<InstallerPhase> {
    vec![
        installer_phase(1, "backend-file-operations", "root files.backend and backend/index.json copy/append operations, including blueprint marker injection", "map backend operations to dynamic service boundary or source-injection lane; Flask blueprint injection is quarry only", "Caduceus later"),
        installer_phase(2, "frontend-file-operations", "root files.frontend and frontend/index.json copy operations into src/tablets/<tab>", "map frontend payload to dynamic cartridge static assets or source-injection recompile", "Caduceus later"),
        installer_phase(3, "permissions-files", "files.permissions copied to /etc/sudoers.d in the old installer", "permissions require privileged Caduceus policy and receipt before any live write", "Caduceus only"),
        installer_phase(4, "root-files", "root config/readme/license and other files copied under the old tablet root", "non-code payloads become manifest/docs/license artifacts inside the installed cartridge", "Caduceus later"),
        installer_phase(5, "package-installations", "backend/requirements.txt, frontend/package.patch.json, system/dependencies.json", "dependency mutation is declared but not executed by Coronatio contract route", "Caduceus only"),
        installer_phase(6, "config-patches", "homeserver.patch.json applied to homeserver.json", "later registry transaction law owns deep merge, validation, atomic write, and permissions", "Caduceus later"),
        installer_phase(7, "tab-hooks", "tab-specific hooks such as backupTab venv note and chiaTab key ownership", "hooks must become typed per-tab Caduceus actions, never ad-hoc Python side effects", "Caduceus only"),
        installer_phase(8, "frontend-rebuild", "BuildManager.rebuild_frontend unless batch mode defers it", "source-injection-recompile lane requires build/test/admission before restart", "Caduceus plus Cibation for repo-backed source"),
        installer_phase(9, "service-restart", "ServiceManager.restart_homeserver_services unless batch mode defers it", "restart is an explicit post-build live-body proof, not part of this readback route", "Caduceus only"),
    ]
}

fn installer_phase(
    sequence: u64,
    id: &str,
    source_law: &str,
    coronatio_contract: &str,
    mutation_authority: &str,
) -> InstallerPhase {
    InstallerPhase {
        id: id.to_string(),
        sequence,
        source_law: source_law.to_string(),
        coronatio_contract: coronatio_contract.to_string(),
        mutation_authority: mutation_authority.to_string(),
    }
}

fn installer_rollback_law() -> RollbackLaw {
    RollbackLaw {
        schema: "coronatio.installer.rollback.v1".to_string(),
        order: vec!["config rollback", "package rollback", "file operation rollback", "service state rollback"].into_iter().map(String::from).collect(),
        config_restore: "restore config backup before package/file cleanup; restore-patches reapplies installed tab homeserver.patch.json files alphabetically for post-build recovery".to_string(),
        file_operation_reversal: "copy/symlink outputs are removed or restored from backups; append blocks are removed by marker and identifier".to_string(),
        service_state_restore: "captured service states are restored after file/package rollback".to_string(),
        batch_restore: "batch mode may defer build/restart and may fall back to individual tab installation while preserving success/failure lists".to_string(),
    }
}

fn installer_lifecycle_law() -> Vec<InstallerLifecycleLaw> {
    vec![
        InstallerLifecycleLaw {
            action: "install".to_string(),
            sequence: vec![
                "validate current config",
                "validate manifest",
                "check collision",
                "validate dependencies",
                "perform files",
                "install packages",
                "apply config patches",
                "run hooks",
                "rebuild frontend",
                "restart service",
            ]
            .into_iter()
            .map(String::from)
            .collect(),
            post_build_policy:
                "single install rebuilds/restarts immediately unless batch mode defers".to_string(),
        },
        InstallerLifecycleLaw {
            action: "uninstall".to_string(),
            sequence: vec![
                "find installed tab",
                "remove registered package/file/config effects",
                "optionally skip build/restart during batch/reinstall",
                "rebuild/restart after final operation",
            ]
            .into_iter()
            .map(String::from)
            .collect(),
            post_build_policy:
                "uninstall manager owns cleanup; Coronatio only reads law until Caduceus exists"
                    .to_string(),
        },
        InstallerLifecycleLaw {
            action: "reinstall".to_string(),
            sequence: vec![
                "prove installed state or locate available package",
                "uninstall with skip_build_and_restart",
                "install with name collision bypass",
                "batch reinstall defers final build/restart",
            ]
            .into_iter()
            .map(String::from)
            .collect(),
            post_build_policy:
                "reinstall preserves one final build/restart boundary after successful replacement"
                    .to_string(),
        },
        InstallerLifecycleLaw {
            action: "restore-patches".to_string(),
            sequence: vec![
                "list installed premium tabs",
                "find homeserver.patch.json",
                "sort by tab name",
                "apply each patch deterministically",
            ]
            .into_iter()
            .map(String::from)
            .collect(),
            post_build_policy:
                "post-build config recovery only; it does not reinstall files or dependencies"
                    .to_string(),
        },
    ]
}

fn installer_lane_mapping() -> Vec<InstallerLaneMapping> {
    vec![
        InstallerLaneMapping { install_mode: InstallMode::DynamicCartridge, accepted_package: "manifest + static assets + optional localhost service boundary".to_string(), post_install_requirement: "reload registry and serve /tabs/<tab-id>/... without Coronatio recompile".to_string(), rejected_shape: "Flask blueprint injection as live runtime authority".to_string() },
        InstallerLaneMapping { install_mode: InstallMode::SourceInjectionRecompile, accepted_package: "trusted frontend/backend source operations requiring host rebuild".to_string(), post_install_requirement: "copy source in a governed lane, run Rust/source proof, publish/admit source, then restart through Caduceus when requested".to_string(), rejected_shape: "unreviewed source mutation in canonical checkout".to_string() },
        InstallerLaneMapping { install_mode: InstallMode::FirstPartyNative, accepted_package: "none for user packages; only Coronatio source may define this lane".to_string(), post_install_requirement: "Cibation-admitted Rust source and tests".to_string(), rejected_shape: "premium package claiming first-party-native".to_string() },
    ]
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
    r####"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <meta name="theme-color" content="#000000" />
    <meta
      name="description"
      content="HomeServer Admin Interface"
    />
    <meta name="csrf-token" content="" /> <!-- Will be populated by Flask -->
    <link rel="icon" type="image/x-icon" href="/assets/favicon-CHgY6yiq.ico" />
    <link rel="apple-touch-icon" sizes="180x180" href="/assets/apple-touch-icon-CgumePGS.png" />
    <link rel="icon" type="image/png" sizes="32x32" href="/assets/favicon-32x32-C1pw8DCa.png" />
    <link rel="icon" type="image/png" sizes="16x16" href="/assets/favicon-16x16-B9kc5FdD.png" />
    <link rel="icon" type="image/png" sizes="192x192" href="/assets/android-chrome-192x192-BAMQ6pez.png" />
    <link rel="icon" type="image/png" sizes="512x512" href="/assets/android-chrome-512x512-C9kCmYN6.png" />
    <title>HomeServer</title>
    <style>

      body {
        background-color: var(--background);
        margin: 0;
      }
      .app {
        visibility: hidden;
      }
      html.theme-loaded .app {
        visibility: visible;
      }
    </style>
    <script type="module" crossorigin src="/assets/index-BRoXzIjg.js"></script>
    <link rel="stylesheet" crossorigin href="/assets/index-Co-PYpJ8.css">
  </head>
  <body>
    <noscript>You need to enable JavaScript to run this app.</noscript>
    <div id="root"></div>
  </body>
</html> "####.to_string()
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
        assert!(body.contains("<title>HomeServer</title>"));
        assert!(body.contains("HomeServer Admin Interface"));
        assert!(body.contains("/assets/index-BRoXzIjg.js"));
        assert!(body.contains("/assets/index-Co-PYpJ8.css"));
        assert!(body.contains("<div id=\"root\"></div>"));
        assert!(!body.contains("Coronatio crown shell"));
        assert!(!body.contains("data-source-material=\"homeserver-main-site\""));
        assert!(!body.contains("class=\"tab-bar\""));
        assert!(!body.contains("Admitted services"));
        assert!(!body.contains("Safe file ingress"));
        assert!(!body.contains("Arcadia"));
    }

    #[test]
    fn native_pane_bodies_are_not_placeholder_cards() {
        let shell = render_crown_shell();
        assert!(shell.contains("<title>HomeServer</title>"));
        assert!(shell.contains("/assets/index-BRoXzIjg.js"));
        assert!(shell.contains("/assets/index-Co-PYpJ8.css"));
        assert!(shell.contains("<div id=\"root\"></div>"));
        assert!(!shell.contains("Admin authority"));
        assert!(!shell.contains("System telemetry"));
        assert!(!shell.contains("Coronatio crown shell"));
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
                .uri("/api/coronatio/tabs")
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
        assert!(shell_body.contains("<title>HomeServer</title>"));
        assert!(shell_body.contains("/assets/index-BRoXzIjg.js"));

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
        assert!(boundary.api_unknown_path_policy.contains("proxy"));
        assert!(boundary.static_shell_policy.contains("exact Flask/React"));
        assert_eq!(legacy_homeserver_proxy_host(), "127.0.0.1".to_string());
        assert_eq!(legacy_homeserver_proxy_port(), 8001);
    }

    #[tokio::test]
    async fn installer_route_encodes_premium_installer_law_without_live_mutation() {
        let temp = test_tab_root("installer-law");
        let response = app(AppState {
            tab_root: Arc::new(temp),
        })
        .oneshot(
            Request::builder()
                .uri("/api/installer")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let installer: InstallerReadback = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(installer.schema, "coronatio.installer.contract.v1");
        assert_eq!(installer.status, "contract-only");
        assert!(installer
            .root_manifest_schema
            .required_fields
            .contains(&"name".to_string()));
        assert!(installer
            .component_manifest_schema
            .operation_types
            .contains(&"append".to_string()));
        assert!(installer
            .validation_phases
            .iter()
            .any(|phase| phase.id == "version-conflict"));
        assert!(installer
            .install_phases
            .iter()
            .any(|phase| phase.id == "frontend-rebuild"));
        assert_eq!(
            installer.rollback_law.order,
            [
                "config rollback",
                "package rollback",
                "file operation rollback",
                "service state rollback"
            ]
        );
        assert!(installer
            .first_missing_live_signal
            .contains("Caduceus installer actuator"));
        assert!(installer
            .lane_mapping
            .iter()
            .any(
                |mapping| mapping.install_mode == InstallMode::FirstPartyNative
                    && mapping.rejected_shape.contains("premium package")
            ));
    }

    #[tokio::test]
    async fn frontend_storage_route_encodes_browser_persistence_and_migration_law() {
        let temp = test_tab_root("frontend-storage");
        let response = app(AppState {
            tab_root: Arc::new(temp),
        })
        .oneshot(
            Request::builder()
                .uri("/api/frontend/storage")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let data: FrontendStorageReadback = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(data.schema, "coronatio.frontend-storage.contract.v1");
        assert_eq!(data.status, "contract-only");
        assert!(data
            .persisted_stores
            .iter()
            .any(|store| store.storage_key == "homeserver-store"
                && store.persisted_fields.contains(&"activeTab".to_string())));
        assert!(data
            .persisted_stores
            .iter()
            .any(|store| store.storage_key == "auth-storage"
                && store.boundary.contains("never localStorage")));
        assert!(data
            .persistence_fields
            .iter()
            .any(|field| field.field == "isInitialized"
                && field.coronatio_owner == "startup receipt"));
        assert!(data
            .debounce_law
            .iter()
            .any(|law| law.interval_ms == 500 && law.source.contains("debouncedSetItem")));
        assert!(data
            .stale_state_law
            .iter()
            .any(|law| law.coronatio_rule.contains("malformed browser snapshot")));
        assert!(data
            .forbidden_persistence
            .contains(&"adminToken".to_string()));
        assert!(data
            .first_missing_live_signal
            .contains("storage migration adapter"));
    }

    #[tokio::test]
    async fn service_data_route_encodes_portal_monitor_and_broadcast_law() {
        let temp = test_tab_root("service-data");
        let response = app(AppState {
            tab_root: Arc::new(temp),
        })
        .oneshot(
            Request::builder()
                .uri("/api/services/data")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let data: ServiceDataReadback = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(data.schema, "coronatio.service-data.contract.v1");
        assert_eq!(data.status, "contract-only");
        assert!(data.portal_schema.fields.contains(&"remoteURL".to_string()));
        assert!(data
            .portal_schema
            .portal_types
            .contains(&"link".to_string()));
        assert!(data
            .service_card_schema
            .fields
            .contains(&"isScriptManaged".to_string()));
        assert!(data
            .monitor_topics
            .iter()
            .any(|topic| topic.topic == "admin.disk.info" && topic.admin_only));
        assert!(data
            .monitor_topics
            .iter()
            .any(|topic| topic.topic == "services.status"
                && topic.admin_fields.contains(&"isEnabled".to_string())));
        assert!(data.broadcast_law.transport_replacement.contains("SSE"));
        assert!(data
            .first_missing_live_signal
            .contains("service collectors and monitor broadcasters are not wired"));
    }

    #[tokio::test]
    async fn registry_transaction_route_encodes_config_patch_persistence_law() {
        let temp = test_tab_root("registry-transaction");
        let response = app(AppState {
            tab_root: Arc::new(temp),
        })
        .oneshot(
            Request::builder()
                .uri("/api/registry/transaction")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let transaction: RegistryTransactionReadback = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(transaction.schema, "coronatio.registry.transaction.v1");
        assert_eq!(transaction.status, "contract-only");
        assert!(transaction.deep_merge_law.tab_merge.contains("starred"));
        assert!(transaction
            .starred_tab_law
            .preservation_rule
            .contains("without displacing"));
        assert!(transaction
            .validation_law
            .factory_fallback_gate
            .contains("factoryFallback"));
        assert!(transaction
            .persistence_law
            .permission_restore
            .contains("www-data:www-data"));
        assert!(transaction
            .rollback_law
            .mismatch_policy
            .contains("do not remove"));
        assert!(transaction
            .transaction_sequence
            .iter()
            .any(|phase| phase.id == "atomic-promote" && phase.source_law.contains("shutil.move")));
        assert!(transaction
            .first_missing_live_signal
            .contains("Caduceus registry transaction actuator"));
    }

    #[tokio::test]
    async fn api_root_declares_installer_contract_route() {
        let temp = test_tab_root("installer-root-route");
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
        assert!(root.routes.contains(&"/api/installer".to_string()));
    }

    fn test_tab_root(name: &str) -> PathBuf {
        let root =
            std::env::temp_dir().join(format!("coronatio-test-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).unwrap();
        root
    }
}
