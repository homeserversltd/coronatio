#[derive(Debug, Clone, Copy)]
pub(crate) struct IndicatorRenderContext {
    pub(crate) session: Session,
}

pub(crate) type IndicatorRenderer = fn(IndicatorRenderContext) -> String;
pub(crate) type IndicatorCollector = fn(Session) -> Result<serde_json::Value, String>;

#[derive(Clone, Copy)]
pub(crate) struct IndicatorManifest {
    pub(crate) id: &'static str,
    pub(crate) topic_id: &'static str,
    pub(crate) order: u16,
    pub(crate) title: &'static str,
    pub(crate) icon_id: &'static str,
    pub(crate) initial_state: &'static str,
    pub(crate) admin_interactive: bool,
    pub(crate) render_indicator: IndicatorRenderer,
    pub(crate) render_modal: IndicatorRenderer,
    pub(crate) collector: Option<IndicatorCollector>,
}

const TOPIC_IDS: &[&str] = &["tailscale.status", "internet.status", "vpn.status", "services.status", "power.status"];

pub(crate) fn catalog() -> Vec<IndicatorManifest> {
    let mut entries = vec![tailscale_manifest(), internet_manifest(), openvpn_manifest(), services_manifest(), power_meter_manifest()];
    validate_catalog(&entries).expect("invalid compiled indicator catalog");
    entries.sort_by_key(|entry| (entry.order, entry.id));
    entries
}

pub(crate) fn validate_catalog(entries: &[IndicatorManifest]) -> Result<(), String> {
    let mut ids = std::collections::BTreeSet::new();
    let mut orders = std::collections::BTreeSet::new();
    for entry in entries {
        if entry.id.is_empty() || !entry.id.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-') {
            return Err(format!("invalid indicator id: {}", entry.id));
        }
        if !ids.insert(entry.id) { return Err(format!("duplicate indicator id: {}", entry.id)); }
        if !orders.insert(entry.order) { return Err(format!("duplicate indicator order: {}", entry.order)); }
        if !TOPIC_IDS.contains(&entry.topic_id) { return Err(format!("unknown indicator topic: {}", entry.topic_id)); }
    }
    Ok(())
}

pub(crate) fn render_indicator_strip(session: Session) -> String {
    let ctx = IndicatorRenderContext { session };
    catalog().into_iter().map(|entry| (entry.render_indicator)(ctx)).collect::<Vec<_>>().join("
")
}

pub(crate) fn render_indicator_modal(id: &str, session: Session) -> Option<String> {
    let ctx = IndicatorRenderContext { session };
    catalog().into_iter().find(|entry| entry.id == id).map(|entry| (entry.render_modal)(ctx))
}

pub(crate) fn render_indicator_modal_registry(session: Session) -> String {
    let rows = catalog().into_iter().filter_map(|entry| {
        let body = render_indicator_modal(entry.id, session)?;
        Some(format!("{}: () => `{}`", serde_json::to_string(entry.id).unwrap(), body))
    }).collect::<Vec<_>>().join(",
");
    format!("const indicatorModalTemplates = {{\n{}\n}};", rows)
}
