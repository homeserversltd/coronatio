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

