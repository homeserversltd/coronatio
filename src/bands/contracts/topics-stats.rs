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

