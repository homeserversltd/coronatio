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

