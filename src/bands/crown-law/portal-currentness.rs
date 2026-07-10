fn normalize_systemd_unit(service: &str) -> String {
    let service = service.trim();
    if service.ends_with(".service") {
        service.to_string()
    } else {
        format!("{service}.service")
    }
}

fn derive_portal_currentness(portal: &PortalEntry) -> &'static str {
    if portal.r#type == "link" || portal.services.is_empty() {
        return "unknown";
    }

    let states = portal
        .services
        .iter()
        .map(|service| normalize_systemd_unit(service))
        .filter_map(|unit| systemctl_is_active(&unit));
    let (checked, active) = states.fold((0_usize, 0_usize), |(checked, active), state| {
        (checked + 1, active + usize::from(state == "active"))
    });

    match (checked, active) {
        (0, _) => "unknown",
        (checked, active) if checked == active => "up",
        (_, 0) => "down",
        _ => "partial",
    }
}

fn collect_portals_currentness(portals: &[PortalEntry]) -> BTreeMap<String, String> {
    portals
        .iter()
        .map(|portal| (portal.name.clone(), derive_portal_currentness(portal).to_string()))
        .collect()
}
