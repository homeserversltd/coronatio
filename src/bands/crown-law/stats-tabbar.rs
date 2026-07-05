fn stats_storage() -> Vec<StatsDrive> {
    let output = Command::new("df").args(["-B1", "-P", "/", "/home", "/vault", "/mnt/nas"]).output();
    let mut drives = Vec::new();
    if let Ok(output) = output {
        let raw = String::from_utf8_lossy(&output.stdout);
        for line in raw.lines().skip(1) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 6 {
                continue;
            }
            let total = parts[1].parse::<u64>().ok();
            let used = parts[2].parse::<u64>().ok();
            let free = parts[3].parse::<u64>().ok();
            let percent = parts[4].trim_end_matches('%').parse::<u8>().ok();
            let mount = parts[5].to_string();
            if drives.iter().any(|drive: &StatsDrive| drive.mount == mount) {
                continue;
            }
            drives.push(StatsDrive {
                name: parts[0].to_string(),
                mount,
                total_bytes: total,
                used_bytes: used,
                free_bytes: free,
                usage_percent: percent,
                source: "df -B1 -P".to_string(),
            });
        }
    }
    if drives.is_empty() {
        drives.push(StatsDrive {
            name: "root".to_string(),
            mount: "/".to_string(),
            total_bytes: None,
            used_bytes: None,
            free_bytes: None,
            usage_percent: None,
            source: "df unavailable".to_string(),
        });
    }
    drives
}


fn stats_io(storage: &[StatsDrive]) -> StatsIo {
    let mut mount_by_device = BTreeMap::new();
    for drive in storage {
        let device = drive.name.rsplit('/').next().unwrap_or(&drive.name).to_string();
        mount_by_device.insert(device, drive.mount.clone());
    }
    let mut devices = Vec::new();
    if let Ok(raw) = std::fs::read_to_string("/proc/diskstats") {
        for line in raw.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 14 {
                continue;
            }
            let device = parts[2].to_string();
            let Some(mount) = mount_by_device.get(&device).cloned() else { continue; };
            let sectors_read = parts[5].parse::<u64>().unwrap_or(0);
            let sectors_written = parts[9].parse::<u64>().unwrap_or(0);
            devices.push(StatsIoDevice {
                device,
                mount,
                read_bytes: sectors_read.saturating_mul(512),
                write_bytes: sectors_written.saturating_mul(512),
            });
        }
    }
    if devices.is_empty() {
        for drive in storage {
            devices.push(StatsIoDevice {
                device: drive.name.rsplit('/').next().unwrap_or(&drive.name).to_string(),
                mount: drive.mount.clone(),
                read_bytes: 0,
                write_bytes: 0,
            });
        }
    }
    StatsIo { devices }
}

fn stats_network() -> StatsNetwork {
    StatsNetwork {
        interfaces: network_interfaces(),
        connections: connection_counts(),
    }
}

fn network_interfaces() -> Vec<StatsNetworkInterface> {
    let mut interfaces = Vec::new();
    if let Ok(raw) = std::fs::read_to_string("/proc/net/dev") {
        for line in raw.lines().skip(2) {
            let Some((name, rest)) = line.split_once(':') else { continue; };
            let name = name.trim();
            if name.is_empty() {
                continue;
            }
            let values: Vec<&str> = rest.split_whitespace().collect();
            if values.len() < 16 {
                continue;
            }
            if is_unmeaningful_stats_interface(name) {
                continue;
            }
            let rx_bytes = values[0].parse::<u64>().unwrap_or(0);
            let tx_bytes = values[8].parse::<u64>().unwrap_or(0);
            let operstate = std::fs::read_to_string(format!("/sys/class/net/{name}/operstate"))
                .unwrap_or_else(|_| "unknown".to_string())
                .trim()
                .to_string();
            interfaces.push(StatsNetworkInterface { name: name.to_string(), status: operstate, rx_bytes, tx_bytes });
        }
    }
    interfaces.sort_by(|left, right| left.name.cmp(&right.name));
    interfaces
}

fn is_unmeaningful_stats_interface(name: &str) -> bool {
    name == "lo"
        || name == "docker0"
        || name.starts_with("br-")
        || name.starts_with("virbr")
        || name.starts_with("vnet")
        || name.starts_with("zt")
        || name.is_empty()
}

fn connection_counts() -> StatsConnectionCounts {
    let (tcp_established, tcp_listening, tcp_total) = connection_counts_for("/proc/net/tcp");
    let (tcp6_established, tcp6_listening, tcp6_total) = connection_counts_for("/proc/net/tcp6");
    StatsConnectionCounts {
        established: tcp_established + tcp6_established,
        listening: tcp_listening + tcp6_listening,
        total: tcp_total + tcp6_total,
    }
}

fn connection_counts_for(path: &str) -> (u64, u64, u64) {
    let Ok(raw) = std::fs::read_to_string(path) else { return (0, 0, 0); };
    let mut established = 0;
    let mut listening = 0;
    let mut total = 0;
    for line in raw.lines().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() <= 3 {
            continue;
        }
        total += 1;
        match parts[3] {
            "01" => established += 1,
            "0A" => listening += 1,
            _ => {}
        }
    }
    (established, listening, total)
}


fn stats_kea_leases() -> Vec<StatsKeaLease> {
    let mut leases = Vec::new();
    let mut seen = std::collections::BTreeSet::new();
    for path in ["/var/lib/kea/kea-leases4.csv", "/var/lib/kea/dhcp4.leases"] {
        if let Ok(raw) = std::fs::read_to_string(path) {
            for line in raw.lines() {
                let line = line.trim();
                if line.is_empty() || line.starts_with("address") || line.starts_with('#') {
                    continue;
                }
                let parts: Vec<&str> = line.split(',').map(|part| part.trim().trim_matches('"')).collect();
                if parts.len() < 2 {
                    continue;
                }
                let (ip, mac, hostname) = if parts.first().is_some_and(|value| value.contains('.')) {
                    (
                        parts.first().unwrap_or(&"").to_string(),
                        parts.get(1).unwrap_or(&"").to_string(),
                        parts.get(8).or_else(|| parts.get(3)).unwrap_or(&"N/A").to_string(),
                    )
                } else {
                    (
                        parts.get(2).unwrap_or(&"").to_string(),
                        parts.get(0).unwrap_or(&"").to_string(),
                        parts.get(1).unwrap_or(&"N/A").to_string(),
                    )
                };
                if ip.is_empty() || !seen.insert((ip.clone(), mac.clone())) {
                    continue;
                }
                leases.push(StatsKeaLease { hostname, ip, mac, note: String::new() });
                if leases.len() >= 20 { break; }
            }
            if !leases.is_empty() { break; }
        }
    }
    leases
}

fn stats_processes() -> Vec<StatsProcess> {
    let output = Command::new("ps").args(["-eo", "comm,pcpu,rss", "--sort=-pcpu"]).output();
    let mut processes = Vec::new();
    if let Ok(output) = output {
        let raw = String::from_utf8_lossy(&output.stdout);
        for line in raw.lines().skip(1) {
            let mut parts = line.split_whitespace();
            let Some(name) = parts.next() else { continue; };
            if matches!(name, "ps" | "sh" | "bash" | "sudo" | "python3") {
                continue;
            }
            let cpu_percent = parts.next().and_then(|value| value.parse::<f64>().ok()).unwrap_or(0.0);
            let memory_bytes = parts.next().and_then(|value| value.parse::<u64>().ok()).unwrap_or(0).saturating_mul(1024);
            if cpu_percent <= 0.0 && memory_bytes == 0 {
                continue;
            }
            processes.push(StatsProcess { name: name.to_string(), cpu_percent, memory_bytes, process_count: 1 });
            if processes.len() >= 10 { break; }
        }
    }
    processes
}

fn stats_services() -> Vec<StatsService> {
    [
        ("Coronatio", "/health"),
        ("Caduceus", "/api/caduceus/status"),
        ("Harmonia", "/api/caduceus/receipts/latest"),
        ("HomeServer", "/api/services/data"),
    ]
    .into_iter()
    .map(|(name, route)| StatsService {
        name: name.to_string(),
        status: if name == "Coronatio" { "running" } else { "readback" }.to_string(),
        details: if name == "Coronatio" {
            "same-process Rust crown health route is registered".to_string()
        } else {
            "status resolves through the named Coronatio/Caduceus route; privileged systemctl polling is a later Caduceus collector".to_string()
        },
        route: route.to_string(),
    })
    .collect()
}

fn stats_first_missing_signal(storage: &[StatsDrive], services: &[StatsService]) -> String {
    if storage.iter().any(|drive| drive.total_bytes.is_none()) {
        return "storage df readback unavailable".to_string();
    }
    if services.iter().any(|service| service.status == "readback") {
        return "service systemctl collector deferred to Caduceus; route readbacks are present".to_string();
    }
    "none".to_string()
}

fn service_health_summary(services: &[StatsService]) -> String {
    let running = services.iter().filter(|service| service.status == "running").count();
    format!("{running}/{} same-process services running; remaining services use route readback", services.len())
}

fn storage_posture_summary(storage: &[StatsDrive]) -> String {
    let max_percent = storage.iter().filter_map(|drive| drive.usage_percent).max();
    match max_percent {
        Some(percent) if percent >= 90 => format!("attention: storage peak {percent}%"),
        Some(percent) => format!("ok: storage peak {percent}%"),
        None => "storage usage unknown".to_string(),
    }
}


fn render_flask_react_tabbar_quarry() -> String {
    let starred_tab = registry_readback().starred_tab;
    native_crown_panes()
        .into_iter()
        .map(|pane| {
            let hidden_by_default = matches!(pane.id.as_str(), "chia-mining" | "dhcp" | "youtube");
            let is_starred = pane.id == starred_tab;
            let active = pane.id == starred_tab;
            let visibility = if hidden_by_default { "hidden" } else { "visible" };
            let visibility_button = if pane.admin_only {
                r##"<div class="tab-visibility-column" aria-hidden="true"></div>"##.to_string()
            } else {
                format!(
                    r##"<div class="tab-visibility-column"><button type="button" class="visibility-toggle" data-admin-only="true" data-tab-visibility-toggle="{id}" data-visible="{visible}" aria-label="{verb} {title} tab" title="{verb} {title} tab"><span class="eye-icon" aria-hidden="true">{eye}</span></button></div>"##,
                    id = pane.id,
                    title = pane.title,
                    visible = !hidden_by_default,
                    verb = if hidden_by_default { "Show" } else { "Hide" },
                    eye = if hidden_by_default { "🙈" } else { "👁" },
                )
            };
            let star_button = if pane.admin_only || hidden_by_default {
                r##"<div class="tab-star-column" aria-hidden="true"></div>"##.to_string()
            } else {
                format!(
                    r##"<div class="tab-star-column"><button type="button" class="star-button {star_class} fa-star" data-tab-star="{id}" aria-pressed="{pressed}" aria-label="{label}" title="{label}"><span aria-hidden="true">★</span></button></div>"##,
                    id = pane.id,
                    star_class = if is_starred { "fas" } else { "far" },
                    pressed = is_starred,
                    label = if is_starred {
                        format!("{} tab is starred", pane.title)
                    } else {
                        format!("Star {} tab", pane.title)
                    }
                )
            };
            let admin_only_attr = if pane.admin_only { r##" data-admin-only="true" hidden"## } else { "" };
            format!(
                r##"<div class="tab {active_class}" role="tab" tabindex="0" aria-controls="pane-{id}" aria-selected="{selected}" data-pane="{id}" data-tab-id="{id}" data-visibility="{visibility}"{admin_only_attr}>{visibility_button}<span class="tab-name">{title}</span>{star_button}</div>"##,
                id = pane.id,
                title = pane.title,
                admin_only_attr = admin_only_attr,
                active_class = if active { "active" } else { "" },
                selected = active,
                visibility = visibility,
                visibility_button = visibility_button,
                star_button = star_button
            )
        })
        .chain(std::iter::once(r#"<button type="button" class="tab add-tab-button" data-admin-only="true" data-add-tab-button title="Add tab" aria-label="Add tab"><span class="tab-name">+</span></button>"#.to_string()))
        .collect::<Vec<_>>()
        .join("")
}
