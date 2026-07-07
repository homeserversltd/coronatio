#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct ServiceDataGuestProjection {
    schema: String,
    ok: bool,
    success: bool,
    status: String,
    route: String,
    service_count: usize,
    running_count: usize,
    stopped_count: usize,
    unavailable_count: usize,
    needs_attention_count: usize,
    first_missing_signal: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct ServiceDataAdminProjection {
    schema: String,
    status: String,
    route: String,
    portal_schema: PortalSchema,
    service_card_schema: ServiceCardSchema,
    monitor_topics: Vec<MonitorTopicLaw>,
    broadcast_law: BroadcastLaw,
    admin_field_law: Vec<AdminFieldFilter>,
    admin_runtime: AdminRuntimeReadback,
    first_missing_live_signal: String,
}

fn project_service_data_guest(raw: &ServiceDataReadback) -> ServiceDataGuestProjection {
    let service_count = raw.admin_runtime.services.len();
    let running_count = raw
        .admin_runtime
        .services
        .iter()
        .filter(|service| service.state == "Running" || service.state == "Enabled")
        .count();
    let stopped_count = raw
        .admin_runtime
        .services
        .iter()
        .filter(|service| service.state == "Stopped" || service.state == "Disabled")
        .count();
    let unavailable_count = raw
        .admin_runtime
        .services
        .iter()
        .filter(|service| service.state == "Unavailable" || service.state == "Unknown")
        .count();
    ServiceDataGuestProjection {
        schema: "coronatio.service-data.guest-projection.v1".to_string(),
        ok: unavailable_count == 0,
        success: true,
        status: if unavailable_count == 0 { "ok" } else { "degraded" }.to_string(),
        route: raw.route.clone(),
        service_count,
        running_count,
        stopped_count,
        unavailable_count,
        needs_attention_count: stopped_count + unavailable_count,
        first_missing_signal: raw.first_missing_live_signal.clone(),
    }
}

fn project_service_data_admin(raw: &ServiceDataReadback) -> ServiceDataAdminProjection {
    ServiceDataAdminProjection {
        schema: raw.schema.clone(),
        status: raw.status.clone(),
        route: raw.route.clone(),
        portal_schema: raw.portal_schema.clone(),
        service_card_schema: raw.service_card_schema.clone(),
        monitor_topics: raw.monitor_topics.clone(),
        broadcast_law: raw.broadcast_law.clone(),
        admin_field_law: raw.admin_field_law.clone(),
        admin_runtime: raw.admin_runtime.clone(),
        first_missing_live_signal: raw.first_missing_live_signal.clone(),
    }
}

