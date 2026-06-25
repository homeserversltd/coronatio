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

