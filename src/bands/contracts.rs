const DEFAULT_TAB_ROOT: &str = "/var/lib/coronatio/tabs";
const INSTALLED_HOMESERVER_JSON: &str = "/etc/homeserver.json";
const LEGACY_HOMESERVER_JSON: &str = "/var/www/homeserver/src/config/homeserver.json";
const QUARRY_HOMESERVER_JSON: &str = "../homeserver/initialization/flask/inject/src/config/homeserver.json";
const LOCAL_QUARRY_HOMESERVER_JSON: &str = "/fulcrum/attachments/homeserver/initialization/flask/inject/src/config/homeserver.json";
const INSTALLED_STATIC_ROOT: &str = "/opt/coronatio/source/static";
const DEFAULT_STATIC_ROOT: &str = "static";
const PRIMARY_TABS: [&str; 4] = ["admin", "stats", "portals", "upload"];

const REQUIRED_THEME_KEYS: &[&str] = &[
    "color-primary",
    "color-secondary",
    "bg-primary",
    "bg-secondary",
    "bg-tertiary",
    "bg-hover",
    "bg-active",
    "text-primary",
    "text-secondary",
    "text-tertiary",
    "text-disabled",
    "text-accent",
    "status-success",
    "status-error",
    "status-warning",
    "status-info",
    "spacing-xs",
    "spacing-sm",
    "spacing-md",
    "spacing-lg",
    "spacing-xl",
    "font-family",
    "font-mono",
    "font-size-xs",
    "font-size-sm",
    "font-size-base",
    "font-size-md",
    "font-size-lg",
    "font-size-xl",
    "font-weight-normal",
    "font-weight-medium",
    "font-weight-bold",
    "line-height-tight",
    "line-height-normal",
    "line-height-loose",
    "transition-fast",
    "transition-normal",
    "transition-slow",
    "shadow-sm",
    "shadow-md",
    "shadow-lg",
    "radius",
];


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FavoriteManifest {
    schema: String,
    starred_tab: String,
    source_quarry: Vec<String>,
    tabs: Vec<FavoriteTabManifest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FavoriteTabManifest {
    id: String,
    display_name: String,
    starred: bool,
    visible: bool,
    admin_only: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct FavoriteManifestResponse {
    schema: String,
    source: String,
    starred_tab: String,
    source_quarry: Vec<String>,
    tabs: Vec<FavoriteTabManifest>,
    first_load_law: String,
}

#[derive(Debug, Clone, Serialize)]
struct StarredTabResponse {
    schema: String,
    success: bool,
    starred_tab: String,
    source: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SetStarredTabRequest {
    tab_name: Option<String>,
    tab: Option<String>,
    is_starred: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ThemeCatalog {
    schema: String,
    default: String,
    themes: BTreeMap<String, BTreeMap<String, String>>,
}

#[derive(Debug, Clone, Serialize)]
struct ThemeCatalogResponse {
    schema: String,
    source: String,
    default: String,
    required: Vec<String>,
    themes: BTreeMap<String, BTreeMap<String, String>>,
}

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
    doctrine: StatsViewportDoctrine,
    transport: StatsTransport,
    resources: StatsResources,
    storage: Vec<StatsDrive>,
    network: StatsNetwork,
    io: StatsIo,
    services: Vec<StatsService>,
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct StatsViewportDoctrine {
    quarry_sources: Vec<String>,
    preserved_sections: Vec<String>,
    refresh_seconds: u64,
    authority: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct StatsResources {
    load: StatsLoad,
    memory: StatsMemory,
    swap: StatsMemory,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct StatsLoad {
    one: Option<f64>,
    five: Option<f64>,
    fifteen: Option<f64>,
    cpu_temperature_celsius: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct StatsMemory {
    total_bytes: Option<u64>,
    used_bytes: Option<u64>,
    free_bytes: Option<u64>,
    percent: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct StatsDrive {
    name: String,
    mount: String,
    total_bytes: Option<u64>,
    used_bytes: Option<u64>,
    free_bytes: Option<u64>,
    usage_percent: Option<u8>,
    source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct StatsNetwork {
    interfaces: Vec<StatsNetworkInterface>,
    connections: StatsConnectionCounts,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct StatsNetworkInterface {
    name: String,
    status: String,
    rx_bytes: u64,
    tx_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct StatsConnectionCounts {
    established: u64,
    listening: u64,
    total: u64,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct StatsIo {
    devices: Vec<StatsIoDevice>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct StatsIoDevice {
    device: String,
    mount: String,
    read_bytes: u64,
    write_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct StatsService {
    name: String,
    status: String,
    details: String,
    route: String,
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

