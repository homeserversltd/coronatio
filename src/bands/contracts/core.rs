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

