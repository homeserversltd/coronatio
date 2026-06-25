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

