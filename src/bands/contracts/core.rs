const DEFAULT_TAB_ROOT: &str = "/var/lib/coronatio/tabs";
const PRIMARY_TABS: [&str; 4] = ["admin", "stats", "portals", "upload"];
const LEGACY_HOMESERVER_ASSET_ROOT: &str = "/var/www/homeserver/build/assets";
const LEGACY_HOMESERVER_BUILD_ROOT: &str = "/var/www/homeserver/build";
const DEFAULT_HOMESERVER_CONFIG: &str = "/etc/homeserver.json";
const ADMIN_SESSION_TIMEOUT_SECONDS: u64 = 1800;

#[derive(Clone)]
struct AppState {
    tab_root: Arc<PathBuf>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ValidatePinRequest {
    pin: String,
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

