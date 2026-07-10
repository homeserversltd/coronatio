fn normalize_systemd_unit(service: &str) -> String {
    let service = service.trim();
    if service.ends_with(".service") {
        service.to_string()
    } else {
        format!("{service}.service")
    }
}

fn local_port_is_open(port: u16) -> bool {
    #[cfg(test)]
    if let Ok(path) = std::env::var("CORONATIO_PORT_PROBE_FIXTURE") {
        return std::fs::read_to_string(path)
            .ok()
            .and_then(|fixture| serde_json::from_str::<BTreeMap<String, String>>(&fixture).ok())
            .and_then(|states| states.get(&port.to_string()).cloned())
            .is_some_and(|state| state == "open");
    }

    format!("127.0.0.1:{port}")
        .parse::<SocketAddr>()
        .ok()
        .and_then(|address| TcpStream::connect_timeout(&address, Duration::from_millis(200)).ok())
        .is_some()
}

fn service_unit_is_active(unit: &str, port: Option<u16>) -> Option<bool> {
    if let Some(state) = systemctl_is_active(unit) {
        return Some(state == "active");
    }
    port.map(local_port_is_open)
}

fn derive_portal_currentness(portal: &PortalEntry) -> &'static str {
    if portal.r#type == "link" || portal.services.is_empty() {
        return "unknown";
    }

    let states = portal
        .services
        .iter()
        .map(|service| normalize_systemd_unit(service))
        .filter_map(|unit| service_unit_is_active(&unit, portal.port));
    let (checked, active) = states.fold((0_usize, 0_usize), |(checked, active), state| {
        (checked + 1, active + usize::from(state))
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
