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

fn admin_runtime_readback() -> AdminRuntimeReadback {
    let mounts = read_proc_mounts();
    let devices = admin_block_devices_from_mounts(&mounts);
    let mount_destinations = admin_mount_destinations_from_mounts(&mounts);
    let services = vec![
        ssh_password_auth_state(),
        systemd_service_state("ssh-service", "SSH Service", &["sshd", "ssh"]),
        systemd_service_state("samba-file-sharing", "Samba File Sharing", &["smb", "smbd", "samba"]),
    ];
    AdminRuntimeReadback {
        devices,
        mount_destinations,
        services,
        source: "/proc/mounts + /sys/block + df -B1 -P + systemctl + sshd_config readback".to_string(),
    }
}

#[derive(Debug, Clone)]
struct ProcMountReadback {
    device: String,
    mount: String,
    filesystem: String,
}

fn read_proc_mounts() -> Vec<ProcMountReadback> {
    let raw = std::env::var("CORONATIO_PROC_MOUNTS_FIXTURE")
        .ok()
        .and_then(|path| std::fs::read_to_string(path).ok())
        .or_else(|| std::fs::read_to_string("/proc/mounts").ok())
        .unwrap_or_default();
    raw.lines()
        .filter_map(|line| {
            let mut parts = line.split_whitespace();
            let device = parts.next()?.replace("\\040", " ");
            let mount = parts.next()?.replace("\\040", " ");
            let filesystem = parts.next()?.to_string();
            Some(ProcMountReadback { device, mount, filesystem })
        })
        .collect()
}

fn admin_block_devices_from_mounts(mounts: &[ProcMountReadback]) -> Vec<AdminBlockDeviceReadback> {
    let mut devices = Vec::new();
    let mut seen = std::collections::BTreeSet::new();
    for mount in mounts {
        if !mount.device.starts_with("/dev/") || mount.mount.starts_with("/snap/") {
            continue;
        }
        let real_device = canonical_device_path(&mount.device);
        if !seen.insert((real_device.clone(), mount.mount.clone())) {
            continue;
        }
        let space = df_space_for_mount(&mount.mount);
        let mapper = mapper_name(&mount.device);
        let encrypted = mapper.is_some() || device_has_dm_holder(&real_device);
        devices.push(AdminBlockDeviceReadback {
            name: best_device_name(&mount.device, &real_device, &mount.mount),
            device: short_device_name(&real_device),
            mount: Some(mount.mount.clone()),
            role: nas_role_for_mount(&mount.mount).map(str::to_string),
            filesystem: Some(mount.filesystem.clone()),
            total_bytes: space.as_ref().and_then(|space| space.total_bytes),
            used_bytes: space.as_ref().and_then(|space| space.used_bytes),
            free_bytes: space.as_ref().and_then(|space| space.free_bytes),
            usage_percent: space.as_ref().and_then(|space| space.usage_percent),
            encrypted,
            mapper,
            lock_state: if encrypted { "Unlocked" } else { "Available" }.to_string(),
        });
    }
    devices.sort_by(|left, right| {
        let left_rank = if left.role.is_some() { 0 } else { 1 };
        let right_rank = if right.role.is_some() { 0 } else { 1 };
        (left_rank, left.mount.clone()).cmp(&(right_rank, right.mount.clone()))
    });
    devices
}

fn admin_mount_destinations_from_mounts(mounts: &[ProcMountReadback]) -> Vec<AdminMountDestinationReadback> {
    [("Primary NAS", "/mnt/nas"), ("NAS Backup", "/mnt/nas_backup")]
        .into_iter()
        .map(|(role, path)| {
            let mounted = mounts.iter().find(|mount| mount.mount == path);
            let space = mounted.and_then(|mount| df_space_for_mount(&mount.mount));
            AdminMountDestinationReadback {
                role: role.to_string(),
                path: path.to_string(),
                device: mounted.map(|mount| short_device_name(&canonical_device_path(&mount.device))),
                filesystem: mounted.map(|mount| mount.filesystem.clone()),
                total_bytes: space.as_ref().and_then(|space| space.total_bytes),
                used_bytes: space.as_ref().and_then(|space| space.used_bytes),
                free_bytes: space.as_ref().and_then(|space| space.free_bytes),
                usage_percent: space.as_ref().and_then(|space| space.usage_percent),
                in_use: mounted.is_some(),
            }
        })
        .collect()
}

#[derive(Debug, Clone)]
struct DfSpaceReadback {
    total_bytes: Option<u64>,
    used_bytes: Option<u64>,
    free_bytes: Option<u64>,
    usage_percent: Option<u8>,
}

fn df_space_for_mount(mount: &str) -> Option<DfSpaceReadback> {
    let output = Command::new("df").args(["-B1", "-P", mount]).output().ok()?;
    if !output.status.success() {
        return None;
    }
    let raw = String::from_utf8_lossy(&output.stdout);
    let line = raw.lines().skip(1).last()?;
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 6 {
        return None;
    }
    Some(DfSpaceReadback {
        total_bytes: parts[1].parse::<u64>().ok(),
        used_bytes: parts[2].parse::<u64>().ok(),
        free_bytes: parts[3].parse::<u64>().ok(),
        usage_percent: parts[4].trim_end_matches('%').parse::<u8>().ok(),
    })
}

fn nas_role_for_mount(mount: &str) -> Option<&'static str> {
    match mount {
        "/mnt/nas" => Some("Primary NAS"),
        "/mnt/nas_backup" => Some("NAS Backup"),
        _ => None,
    }
}

fn canonical_device_path(device: &str) -> String {
    std::fs::canonicalize(device)
        .ok()
        .map(|path| path.display().to_string())
        .unwrap_or_else(|| device.to_string())
}

fn short_device_name(device: &str) -> String {
    FsPath::new(device)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or(device)
        .to_string()
}

fn best_device_name(device: &str, real_device: &str, mount: &str) -> String {
    if let Some(role) = nas_role_for_mount(mount) {
        return match role {
            "Primary NAS" => "homeserver-primary-nas".to_string(),
            "NAS Backup" => "homeserver-backup-nas".to_string(),
            _ => short_device_name(real_device),
        };
    }
    if device.starts_with("/dev/disk/by-partlabel/") {
        return short_device_name(device);
    }
    short_device_name(real_device)
}

fn mapper_name(device: &str) -> Option<String> {
    if device.starts_with("/dev/mapper/") {
        Some(short_device_name(device))
    } else {
        None
    }
}

fn device_has_dm_holder(real_device: &str) -> bool {
    let Some(name) = FsPath::new(real_device).file_name().and_then(|name| name.to_str()) else { return false; };
    let holders = format!("/sys/class/block/{name}/holders");
    std::fs::read_dir(holders)
        .ok()
        .map(|mut entries| entries.next().is_some())
        .unwrap_or(false)
}

fn ssh_password_auth_state() -> AdminServiceStateReadback {
    let state = read_sshd_password_auth()
        .map(|enabled| if enabled { "Enabled" } else { "Disabled" }.to_string())
        .unwrap_or_else(|| "Unknown".to_string());
    AdminServiceStateReadback {
        id: "ssh-password-authentication".to_string(),
        label: "SSH Password Authentication".to_string(),
        enabled: state == "Enabled",
        state,
        source: "sshd_config PasswordAuthentication readback".to_string(),
    }
}

fn read_sshd_password_auth() -> Option<bool> {
    let paths = sshd_config_paths();
    let mut value = None;
    for path in paths {
        let Ok(raw) = std::fs::read_to_string(path) else { continue; };
        for line in raw.lines() {
            let line = line.split('#').next().unwrap_or("").trim();
            let mut parts = line.split_whitespace();
            let Some(key) = parts.next() else { continue; };
            if key.eq_ignore_ascii_case("PasswordAuthentication") {
                match parts.next().map(|raw| raw.to_ascii_lowercase()) {
                    Some(raw) if raw == "yes" => value = Some(true),
                    Some(raw) if raw == "no" => value = Some(false),
                    _ => {}
                }
            }
        }
    }
    value
}

fn sshd_config_paths() -> Vec<PathBuf> {
    if let Ok(path) = std::env::var("CORONATIO_SSHD_CONFIG_FIXTURE") {
        return vec![PathBuf::from(path)];
    }
    let mut paths = vec![PathBuf::from("/etc/ssh/sshd_config")];
    if let Ok(entries) = std::fs::read_dir("/etc/ssh/sshd_config.d") {
        let mut dropins: Vec<PathBuf> = entries.filter_map(|entry| entry.ok().map(|entry| entry.path())).collect();
        dropins.sort();
        paths.extend(dropins);
    }
    paths
}

fn systemd_service_state(id: &str, label: &str, units: &[&str]) -> AdminServiceStateReadback {
    let active_unit = units.iter().find_map(|unit| systemctl_is_active(unit).map(|state| (unit, state)));
    let state = active_unit
        .as_ref()
        .map(|(_, state)| if state == "active" { "Running" } else { "Stopped" }.to_string())
        .unwrap_or_else(|| "Unavailable".to_string());
    AdminServiceStateReadback {
        id: id.to_string(),
        label: label.to_string(),
        enabled: state == "Running",
        state,
        source: active_unit
            .map(|(unit, _)| format!("systemctl is-active {unit}"))
            .unwrap_or_else(|| "systemctl readback unavailable".to_string()),
    }
}

fn systemctl_is_active(unit: &str) -> Option<String> {
    if let Ok(path) = std::env::var("CORONATIO_SYSTEMCTL_FIXTURE") {
        let fixture = std::fs::read_to_string(path).ok()?;
        let states = serde_json::from_str::<BTreeMap<String, String>>(&fixture).ok()?;
        return states
            .get(unit)
            .or_else(|| unit.strip_suffix(".service").and_then(|service| states.get(service)))
            .cloned()
            .filter(|state| !state.is_empty() && state != "unknown");
    }
    let output = Command::new("systemctl").args(["is-active", unit]).output().ok()?;
    let state = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if state.is_empty() || state == "unknown" {
        None
    } else {
        Some(state)
    }
}

fn html_escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn format_admin_bytes(bytes: Option<u64>) -> String {
    let Some(bytes) = bytes else { return "—".to_string(); };
    let units = [(1_u64 << 40, "T"), (1_u64 << 30, "G"), (1_u64 << 20, "M"), (1_u64 << 10, "K")];
    for (unit, label) in units {
        if bytes >= unit {
            let value = bytes as f64 / unit as f64;
            return format!("{value:.1}{label}");
        }
    }
    format!("{bytes}B")
}

fn format_space(used: Option<u64>, total: Option<u64>, free: Option<u64>, percent: Option<u8>) -> String {
    format!(
        "{}/{} ({}) - {} free",
        format_admin_bytes(used),
        format_admin_bytes(total),
        percent.map(|value| format!("{value}%")).unwrap_or_else(|| "—".to_string()),
        format_admin_bytes(free),
    )
}

fn render_admin_service_card_html(id: &str) -> String {
    let runtime = admin_runtime_readback();
    let Some(service) = runtime.services.into_iter().find(|service| service.id == id) else {
        return "<div class=\"ssh-status\"><h3>Service</h3><div class=\"ssh-toggle\"><span class=\"toggle-label\">Unavailable</span></div></div>".to_string();
    };
    let checked = if service.enabled { " checked" } else { "" };
    let icon_class = if service.enabled { "enabled" } else { "disabled" };
    let icon = match id {
        "ssh-password-authentication" if service.enabled => "🔓",
        "ssh-password-authentication" => "🔒",
        "samba-file-sharing" => "↗",
        _ if service.enabled => "▶",
        _ => "■",
    };
    let outer_class = if id == "samba-file-sharing" { "samba-status" } else { "ssh-status" };
    let toggle_class = if id == "samba-file-sharing" { "samba-toggle" } else { "ssh-toggle" };
    let icon_span_class = if id == "samba-file-sharing" { "samba-icon" } else { "ssh-icon" };
    format!(
        "<div class=\"{outer_class}\" data-admin-toggle-card=\"{}\" data-real-state=\"{}\" data-real-state-source=\"{}\"><h3>{}</h3><div class=\"{toggle_class}\"><label class=\"toggle-switch\"><input type=\"checkbox\"{checked} data-state-source=\"{}\" hx-post=\"/admit/admin/toggle/{}\" hx-target=\"closest [data-service-card]\" hx-swap=\"innerHTML\" hx-disabled-elt=\"this\"><span class=\"toggle-slider\"></span></label><span class=\"toggle-label\">{}</span><span class=\"{icon_span_class} {icon_class}\">{icon}</span></div></div>",
        html_escape(id),
        html_escape(&service.state),
        html_escape(&service.source),
        html_escape(&service.label),
        html_escape(&service.source),
        html_escape(id),
        html_escape(&service.state),
    )
}

fn render_admin_service_card_result_html(id: &str, result: Option<&AdminMutationResult>) -> String {
    let mut card = render_admin_service_card_html(id);
    if let Some(result) = result {
        let class = if result.ok { "success" } else { "error" };
        card.push_str(&format!(
            "<div class=\"update-status-container {class}\" data-admin-mutation-result=\"{}\" data-og-affordance=\"toast-mapped-to-result-strip\"><strong>{}</strong><span>{}</span><code>{}</code></div>",
            html_escape(&result.action),
            html_escape(&result.title),
            html_escape(&result.message),
            html_escape(&result.first_missing_signal),
        ));
    }
    card
}

fn render_admin_available_devices_html() -> String {
    let runtime = admin_runtime_readback();
    if runtime.devices.is_empty() {
        return "<div class=\"disk-item empty\"><span class=\"disk-icon\">▣</span><div class=\"disk-info\"><div class=\"disk-name\">No block devices mounted</div><div class=\"disk-details\">No /dev-backed mounts were observed on this body.</div></div></div>".to_string();
    }
    runtime.devices.into_iter().map(|device| {
        let role = device.role.as_ref().map(|role| {
            let class = if role == "Primary NAS" { "nas-role-primary" } else { "nas-role-backup" };
            format!(" <span class=\"nas-role-badge {class}\">{}</span>", html_escape(role))
        }).unwrap_or_default();
        let mount = device.mount.as_ref().map(|mount| format!("<div class=\"disk-mount-info prominent\"><strong>Mounted at:</strong> {}{}</div>", html_escape(mount), if device.role.is_some() { " <span class=\"destination-label\">(NAS)</span>" } else { "" })).unwrap_or_default();
        let fs = device.filesystem.clone().unwrap_or_else(|| "unknown".to_string());
        let encrypted_label = if device.encrypted { " (encrypted)" } else { "" };
        let mapper = device.mapper.as_ref().map(|mapper| format!("<div class=\"mapper-info\">Mapper: {}</div>", html_escape(mapper))).unwrap_or_default();
        let lock_class = if device.encrypted { "unlocked" } else { "available" };
        let lock_icon = if device.encrypted { "🔓" } else { "▣" };
        format!(
            "<div class=\"disk-item selected available{}\"><span class=\"lock-icon\">{lock_icon}</span><span class=\"disk-icon\">▣</span><div class=\"disk-info\"><div class=\"disk-name\">{}{role}</div>{mount}<div class=\"disk-details\">{} - {}{encrypted_label}</div><div class=\"disk-space-usage\"><strong>Space:</strong> {}</div>{mapper}<div class=\"encryption-status {lock_class}\">{lock_icon} {} <span class=\"filesystem-label\">({})</span></div></div></div>",
            if device.role.is_some() { " nas-compatible" } else { "" },
            html_escape(&device.name),
            format_admin_bytes(device.total_bytes),
            html_escape(&fs.to_uppercase()),
            format_space(device.used_bytes, device.total_bytes, device.free_bytes, device.usage_percent),
            html_escape(&device.lock_state),
            html_escape(&fs),
        )
    }).collect::<Vec<_>>().join("")
}

fn render_admin_mount_destinations_html() -> String {
    let runtime = admin_runtime_readback();
    let mounted: Vec<_> = runtime.mount_destinations.into_iter().filter(|dest| dest.in_use).collect();
    if mounted.is_empty() {
        return "<div class=\"disk-item empty\"><span class=\"lock-icon\">🔒</span><span class=\"disk-icon\">▦</span><div class=\"disk-info\"><div class=\"disk-name\">NAS destinations idle</div><div class=\"disk-details\">/mnt/nas and /mnt/nas_backup are not mounted on this body.</div></div></div>".to_string();
    }
    mounted.into_iter().map(|dest| {
        format!(
            "<div class=\"disk-item selected mounted locked-pair\"><span class=\"lock-icon\">🔒</span><span class=\"disk-icon\">▦</span><div class=\"disk-info\"><div class=\"disk-name\">{}</div><div class=\"disk-details\">{}</div><div class=\"disk-mount-info\">Device: <span class=\"device-label\">{}</span><div class=\"disk-space-usage\"><strong>Space:</strong> {}</div></div><span class=\"nas-badge\">In Use</span></div></div>",
            html_escape(&dest.role),
            html_escape(&dest.path),
            html_escape(&dest.device.unwrap_or_else(|| "unknown".to_string())),
            format_space(dest.used_bytes, dest.total_bytes, dest.free_bytes, dest.usage_percent),
        )
    }).collect::<Vec<_>>().join("")
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



fn render_plan_tabbar(session: Session) -> String {
    render_plan_tabbar_with_active(session, None)
}

fn render_plan_tabbar_with_active(session: Session, active: Option<&str>) -> String {
    render_plan_tabbar_projection(session, active, true)
}

fn render_plan_tabbar_fragment_with_active(session: Session, active: Option<&str>) -> String {
    render_plan_tabbar_projection(session, active, false)
}

fn render_plan_tabbar_projection(session: Session, active: Option<&str>, active_load_trigger: bool) -> String {
    let facts = load_iris_facts_sync().unwrap_or_else(|| iris::from_coronatio_contracts(&native_tab_contracts(), "stats"));
    let plan = iris::plan(&facts, session);
    let active_tab = active
        .map(normalize_tab_id)
        .filter(|tab| !tab.is_empty())
        .map(|tab| iris::landing_after_session_change(&plan, &plan, &tab))
        .unwrap_or_else(|| iris::initial_tab(&plan));
    let names = tab_display_names_from_facts(&facts);
    plan.tabs
        .into_iter()
        .filter(|grant| grant.tab_id != "fallback")
        .map(|grant| render_plan_tab_grant(&grant, &names, &active_tab, active_load_trigger))
        .chain((session == Session::Admin).then(|| r#"<button type="button" class="tab add-tab-button" data-admin-only="true" data-add-tab-button title="Add tab" aria-label="Add tab"><span class="tab-name">+</span></button>"#.to_string()))
        .collect::<Vec<_>>()
        .join("")
}

fn load_iris_facts_sync() -> Option<IrisFacts> {
    let raw = std::fs::read_to_string(homeserver_json_path()).ok()?;
    let value: serde_json::Value = serde_json::from_str(&raw).ok()?;
    Some(iris_facts_from_homeserver_value(&value))
}

fn tab_display_names_from_facts(facts: &IrisFacts) -> BTreeMap<String, String> {
    let mut names = native_crown_panes()
        .into_iter()
        .map(|pane| (pane.id, pane.title))
        .collect::<BTreeMap<_, _>>();
    if let Ok(raw) = std::fs::read_to_string(homeserver_json_path()) {
        if let Ok(value) = serde_json::from_str::<serde_json::Value>(&raw) {
            if let Some(tabs) = value.get("tabs").and_then(serde_json::Value::as_object) {
                for fact in &facts.tabs {
                    if let Some(name) = tabs
                        .get(&fact.id)
                        .and_then(|tab| tab.get("config"))
                        .and_then(|config| config.get("displayName"))
                        .and_then(serde_json::Value::as_str)
                    {
                        names.insert(fact.id.clone(), name.to_string());
                    }
                }
            }
        }
    }
    names
}

fn render_plan_tab_grant(grant: &TabGrant, names: &BTreeMap<String, String>, active_tab: &str, active_load_trigger: bool) -> String {
    let id = &grant.tab_id;
    let title = names.get(id).cloned().unwrap_or_else(|| id.to_string());
    let visibility = match grant.state { RenderState::DimmedHidden => "hidden", _ => "visible" };
    let visible_bool = grant.state != RenderState::DimmedHidden;
    let eye = if visible_bool { "👁" } else { "🙈" };
    let verb = if visible_bool { "Hide" } else { "Show" };
    let visibility_button = if grant.eye {
        format!(r##"<div class="tab-visibility-column"><button type="button" class="visibility-toggle" data-admin-only="true" data-tab-visibility-toggle="{id}" data-visible="{visible_bool}" aria-label="{verb} {title} tab" title="{verb} {title} tab"><span class="eye-icon" aria-hidden="true">{eye}</span></button></div>"##)
    } else {
        r##"<div class="tab-visibility-column" aria-hidden="true"></div>"##.to_string()
    };
    let star_button = if grant.star_eligible {
        let star_class = if grant.star { "fas" } else { "far" };
        let label = if grant.star { format!("{} tab is starred", title) } else { format!("Star {} tab", title) };
        format!(r##"<div class="tab-star-column"><button type="button" class="star-button {star_class} fa-star" data-tab-star="{id}" aria-pressed="{pressed}" aria-label="{label}" title="{label}"><span aria-hidden="true">★</span></button></div>"##, pressed = grant.star)
    } else {
        r##"<div class="tab-star-column" aria-hidden="true"></div>"##.to_string()
    };
    let active = grant.tab_id == active_tab;
    let active_class = if active { "active" } else { "" };
    format!(
        r##"<div class="tab {active_class}" role="tab" tabindex="0" aria-controls="pane-{id}" aria-selected="{selected}" data-pane="{id}" data-tab-id="{id}" data-visibility="{visibility}" hx-get="/admit/{id}" hx-target="[data-view-panel='{id}']" hx-swap="innerHTML" hx-trigger="{hx_trigger}">{visibility_button}<span class="tab-name">{title}</span>{star_button}</div>"##,
        selected = active,
        hx_trigger = if active && active_load_trigger { "load, click" } else { "click" },
    )
}
