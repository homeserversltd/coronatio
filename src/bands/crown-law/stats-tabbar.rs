const STATS_COMMAND_TIMEOUT: std::time::Duration = std::time::Duration::from_millis(900);

fn run_bounded(
    mut command: std::process::Command,
    timeout: std::time::Duration,
) -> Option<std::process::Output> {
    use std::io::Read;

    let mut child = command.stdout(std::process::Stdio::piped()).stderr(std::process::Stdio::piped()).spawn().ok()?;
    let stdout_reader = child.stdout.take().map(|mut stdout| std::thread::spawn(move || {
        let mut bytes = Vec::new();
        let _ = stdout.read_to_end(&mut bytes);
        bytes
    }));
    let stderr_reader = child.stderr.take().map(|mut stderr| std::thread::spawn(move || {
        let mut bytes = Vec::new();
        let _ = stderr.read_to_end(&mut bytes);
        bytes
    }));
    let start = std::time::Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(status)) => return Some(std::process::Output {
                status,
                stdout: stdout_reader.and_then(|reader| reader.join().ok()).unwrap_or_default(),
                stderr: stderr_reader.and_then(|reader| reader.join().ok()).unwrap_or_default(),
            }),
            Ok(None) if start.elapsed() < timeout => std::thread::sleep(std::time::Duration::from_millis(25)),
            Ok(None) | Err(_) => {
                let _ = child.kill();
                let _ = child.wait();
                let _ = stdout_reader.and_then(|reader| reader.join().ok());
                let _ = stderr_reader.and_then(|reader| reader.join().ok());
                return None;
            }
        }
    }
}



fn admin_runtime_readback() -> AdminRuntimeReadback {
    let mounts = read_proc_mounts();
    let mount_destinations = admin_mount_destinations_from_mounts(&mounts);
    let services = vec![
        ssh_password_auth_state(),
        systemd_service_state("ssh-service", "SSH Service", &["sshd", "ssh"]),
        systemd_service_state("samba-file-sharing", "Samba File Sharing", &["smb", "smbd", "samba"]),
    ];
    AdminRuntimeReadback {
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
    let mut command = Command::new("systemctl");
    command.args(["is-active", unit]);
    let output = run_bounded(command, STATS_COMMAND_TIMEOUT)?;
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

fn caduceus_service_bool(body: &serde_json::Value, keys: &[&str]) -> Option<bool> {
    keys.iter().find_map(|key| body.get(*key).and_then(serde_json::Value::as_bool)
        .or_else(|| body.get("status").and_then(|status| status.get(*key)).and_then(serde_json::Value::as_bool)))
}

fn caduceus_absent_units(body: &serde_json::Value) -> Vec<String> {
    for key in ["absentUnits", "absent_units"] {
        if let Some(units) = body.get(key).and_then(serde_json::Value::as_array) {
            return units.iter().filter_map(serde_json::Value::as_str).map(str::to_string).collect();
        }
    }
    body.get("units").and_then(serde_json::Value::as_array).map(|units| units.iter().filter_map(|unit| {
        let name = unit.get("unit").or_else(|| unit.get("name")).and_then(serde_json::Value::as_str)?;
        let absent = unit.get("present").and_then(serde_json::Value::as_bool) == Some(false)
            || unit.get("loadState").or_else(|| unit.get("load_state")).and_then(serde_json::Value::as_str) == Some("not-found");
        absent.then(|| name.to_string())
    }).collect()).unwrap_or_default()
}

fn caduceus_admin_service_state(id: &str, readback: &CaduceusHttpReadback) -> AdminServiceStateReadback {
    let (label, enabled, state) = match id {
        "ssh-password-authentication" => {
            let enabled = caduceus_service_bool(&readback.body, &["passwordAuthentication", "password_authentication", "enabled"]);
            ("SSH Password Authentication", enabled.unwrap_or(false), enabled.map(|value| if value { "Enabled" } else { "Disabled" }.to_string()).unwrap_or_else(|| "Unavailable".to_string()))
        }
        "ssh-service" => {
            let running = caduceus_service_bool(&readback.body, &["running", "active"]);
            ("SSH Service", running.unwrap_or(false), running.map(|value| if value { "Running" } else { "Stopped" }.to_string()).unwrap_or_else(|| "Unavailable".to_string()))
        }
        "samba-file-sharing" => {
            let enabled = caduceus_service_bool(&readback.body, &["allEnabled", "all_enabled"]);
            let running = caduceus_service_bool(&readback.body, &["allRunning", "all_running"]);
            let absent = caduceus_absent_units(&readback.body);
            let state = match (running, absent.is_empty()) {
                (Some(true), true) => "Running".to_string(),
                (Some(true), false) => format!("Running (absent: {})", absent.join(", ")),
                (Some(false), true) => "Stopped".to_string(),
                (Some(false), false) => format!("Stopped (absent: {})", absent.join(", ")),
                (None, false) => format!("Unavailable (absent: {})", absent.join(", ")),
                (None, true) => "Unavailable".to_string(),
            };
            ("Samba File Sharing", enabled.unwrap_or(false), state)
        }
        _ => ("Service", false, "Unavailable".to_string()),
    };
    AdminServiceStateReadback {
        id: id.to_string(),
        label: label.to_string(),
        enabled,
        state,
        source: if readback.ok { format!("Caduceus {} systemctl readback", readback.path) } else { format!("Caduceus {} unavailable: {}", readback.path, readback.first_missing_signal) },
    }
}

fn render_admin_service_card_html(id: &str, readback: Option<&CaduceusHttpReadback>) -> String {
    let service = readback.map(|readback| caduceus_admin_service_state(id, readback)).or_else(|| {
        admin_runtime_readback().services.into_iter().find(|service| service.id == id)
    });
    let Some(service) = service else {
        return "<div class=\"ssh-status\"><h3>Service</h3><div class=\"ssh-toggle\"><span class=\"toggle-label\">Unavailable</span></div></div>".to_string();
    };
    let checked = if service.enabled { " checked" } else { "" };
    let fault_kind = readback
        .filter(|readback| !readback.ok)
        .map(|readback| format!(" data-cartridge-fault-kind=\"{}\"", html_escape(&readback.first_missing_signal)))
        .unwrap_or_default();
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
        "<div class=\"{outer_class}\" data-admin-toggle-card=\"{}\"{fault_kind} data-real-state=\"{}\" data-real-state-source=\"{}\"><h3>{}</h3><div class=\"{toggle_class}\"><label class=\"ui-toggle ui-toggle--medium toggle-switch\"><span class=\"ui-toggle__switch\"><input type=\"checkbox\" class=\"ui-toggle__input\"{checked} data-state-source=\"{}\" hx-post=\"/admit/admin/toggle/{}\" hx-target=\"closest [data-service-card]\" hx-swap=\"innerHTML\" hx-disabled-elt=\"this\"><span class=\"ui-toggle__slider toggle-slider\"></span></span></label><span class=\"toggle-label\">{}</span><span class=\"{icon_span_class} {icon_class}\">{icon}</span></div></div>",
        html_escape(id),
        html_escape(&service.state),
        html_escape(&service.source),
        html_escape(&service.label),
        html_escape(&service.source),
        html_escape(id),
        html_escape(&service.state),
    )
}

fn render_admin_service_card_result_html(id: &str, readback: Option<&CaduceusHttpReadback>, result: Option<&AdminMutationResult>) -> String {
    let mut card = render_admin_service_card_html(id, readback);
    if let Some(result) = result {
        let class = if result.ok { "success" } else { "error" };
        card.push_str(&format!(
            "<div class=\"update-status-container {class}\" data-admin-mutation-result=\"{}\"{} data-og-affordance=\"toast-mapped-to-result-strip\"><strong>{}</strong><span>{}</span><code>{}</code></div>",
            html_escape(&result.action),
            if result.ok { String::new() } else { format!(" data-cartridge-fault-kind=\"{}\"", html_escape(&result.first_missing_signal)) },
            html_escape(&result.title),
            html_escape(&result.message),
            html_escape(&result.first_missing_signal),
        ));
    }
    card
}

fn render_admin_mount_destinations_html() -> String {
    let runtime = admin_runtime_readback();
    [
        ("NAS", "/mnt/nas"),
        ("NAS Backup", "/mnt/nas_backup"),
    ]
    .into_iter()
    .map(|(role, path)| {
        let mounted = runtime.mount_destinations.iter().find(|dest| dest.path == path && dest.in_use);
        let state_class = if mounted.is_some() { " mounted locked" } else { "" };
        let mount = mounted.map(|dest| {
            format!(
                "<div class=\"disk-mount-info\">Device: <span class=\"device-label\">{}</span><div class=\"disk-space-usage\"><strong>Space:</strong> {}</div></div>",
                html_escape(dest.device.as_deref().unwrap_or("unknown")),
                format_space(dest.used_bytes, dest.total_bytes, dest.free_bytes, dest.usage_percent),
            )
        }).unwrap_or_default();
        format!(
            "<div class=\"disk-item{state_class}\"><span class=\"disk-icon\">▦</span><div class=\"disk-info\"><div class=\"disk-name\">{role}</div><div class=\"disk-details\">{path}</div>{mount}</div></div>"
        )
    })
    .collect::<Vec<_>>()
    .join("")
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
                        parts.first().unwrap_or(&"").to_string(),
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

const STATS_IDENTITY_ROSTER_TTL: std::time::Duration = std::time::Duration::from_secs(30);
const STATS_IDENTITY_ROSTER_FAILED_TTL: std::time::Duration = std::time::Duration::from_secs(5);
static STATS_IDENTITY_ROSTER_CACHE: OnceLock<Mutex<Option<(std::time::Instant, StatsKeaLeases)>>> =
    OnceLock::new();

fn stats_identity_roster() -> StatsKeaLeases {
    let mut cache = STATS_IDENTITY_ROSTER_CACHE
        .get_or_init(Default::default)
        .lock()
        .unwrap();
    if let Some((cached_at, cached_roster)) = cache.as_ref() {
        let ttl = if cached_roster.status == "available" {
            STATS_IDENTITY_ROSTER_TTL
        } else {
            STATS_IDENTITY_ROSTER_FAILED_TTL
        };
        if cached_at.elapsed() < ttl {
            return cached_roster.clone();
        }
    }

    let roster = caduceus_http("GET", "/api/v1/network/device/list");
    let result = if !roster.ok {
        StatsKeaLeases {
            status: "unavailable".to_string(),
            entries: Vec::new(),
        }
    } else {
        let notes = caduceus_http("GET", "/api/v1/network/notes");
        if !notes.ok {
            StatsKeaLeases {
                status: "unavailable".to_string(),
                entries: Vec::new(),
            }
        } else {
            let notes = notes
                .body
                .get("notes")
                .and_then(serde_json::Value::as_object);
            let payload = device_identity_payload(roster.body);
            let rows = payload
                .as_array()
                .cloned()
                .or_else(|| {
                    payload
                        .get("devices")
                        .or_else(|| payload.get("roster"))
                        .or_else(|| payload.get("data").and_then(|data| data.get("devices")))
                        .and_then(serde_json::Value::as_array)
                        .cloned()
                })
                .unwrap_or_default();
            let entries = rows
                .into_iter()
                .map(|mut row| {
                    let note = row
                        .get("mac")
                        .and_then(serde_json::Value::as_str)
                        .and_then(canonical_network_note_mac)
                        .and_then(|mac| notes.and_then(|notes| notes.get(&mac)))
                        .cloned()
                        .unwrap_or(serde_json::Value::String(String::new()));
                    if let Some(object) = row.as_object_mut() {
                        object.insert("note".to_string(), note);
                    }
                    row
                })
                .collect();
            StatsKeaLeases {
                status: "available".to_string(),
                entries,
            }
        }
    };
    *cache = Some((std::time::Instant::now(), result.clone()));
    result
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
    render_plan_tabbar_projection_from_facts(&facts, session, active, active_load_trigger)
}

fn render_plan_tabbar_projection_from_facts(facts: &IrisFacts, session: Session, active: Option<&str>, active_load_trigger: bool) -> String {
    let plan = iris::plan(facts, session);
    let active_tab = active
        .map(normalize_tab_id)
        .filter(|tab| !tab.is_empty())
        .map(|tab| iris::landing_after_session_change(&plan, &plan, &tab))
        .unwrap_or_else(|| iris::initial_tab(&plan));
    let names = tab_display_names_from_facts(facts);
    plan.tabs
        .into_iter()
        .filter(|grant| grant.tab_id != "fallback")
        .map(|grant| render_plan_tab_grant(&grant, &names, &active_tab, active_load_trigger))
        .chain((session == Session::Admin).then(|| r#"<button type="button" class="tab add-tab-button" data-admin-only="true" data-add-tab-button title="Add tab" aria-label="Add tab"><span class="tab-name">+</span></button>"#.to_string()))
        .collect::<Vec<_>>()
        .join("")
}

fn load_iris_facts_sync() -> Option<IrisFacts> {
    let value = std::fs::read_to_string(homeserver_json_path())
        .ok()
        .and_then(|raw| serde_json::from_str::<serde_json::Value>(&raw).ok())
        .unwrap_or_else(|| serde_json::json!({}));
    Some(iris_facts_from_homeserver_value(&value))
}

fn tab_display_names_from_facts(facts: &IrisFacts) -> BTreeMap<String, String> {
    let mut names = native_crown_panes().into_iter().map(|pane| (pane.id, pane.title)).collect::<BTreeMap<_, _>>();
    for cartridge in load_appliance_cartridges() { names.insert(cartridge.id, cartridge.title); }
    if let Ok(raw) = std::fs::read_to_string(homeserver_json_path()) {
        if let Ok(value) = serde_json::from_str::<serde_json::Value>(&raw) {
            if let Some(tabs) = value.get("tabs").and_then(serde_json::Value::as_object) {
                for fact in &facts.tabs {
                    if let Some(name) = tabs.get(&fact.id).and_then(|tab| tab.get("config")).and_then(|config| config.get("displayName")).and_then(serde_json::Value::as_str) { names.insert(fact.id.clone(), name.to_string()); }
                }
            }
        }
    }
    names
}

fn render_plan_tab_grant(grant: &TabGrant, names: &BTreeMap<String, String>, active_tab: &str, _active_load_trigger: bool) -> String {
    let id = &grant.tab_id;
    let title = html_escape(&names.get(id).cloned().unwrap_or_else(|| id.to_string()));
    let visibility = match grant.state { RenderState::DimmedHidden => "hidden", _ => "visible" };
    let visible_bool = grant.state != RenderState::DimmedHidden;
    let eye = if visible_bool { "fa-eye" } else { "fa-eye-slash" };
    let verb = if visible_bool { "Hide" } else { "Show" };
    let visibility_button = if grant.eye {
        format!(r##"<div class="tab-visibility-column"><button type="button" class="visibility-toggle ui-visibility-toggle" data-admin-only="true" data-tab-visibility-toggle="{id}" data-visible="{visible_bool}" aria-pressed="{visible_bool}" aria-label="{verb} {title} tab" title="{verb} {title} tab"><i class="fas {eye}" aria-hidden="true"></i></button></div>"##)
    } else {
        r##"<div class="tab-visibility-column" aria-hidden="true"></div>"##.to_string()
    };
    let star_button = if grant.star_eligible {
        let star_class = if grant.star { "fas" } else { "far" };
        let label = if grant.star { format!("{} tab is starred", title) } else { format!("Star {} tab", title) };
        format!(r##"<div class="tab-star-column"><button type="button" class="star-button {star_class} fa-star" data-tab-star="{id}" aria-pressed="{pressed}" aria-label="{label}" title="{label}"></button></div>"##, pressed = grant.star)
    } else {
        r##"<div class="tab-star-column" aria-hidden="true"></div>"##.to_string()
    };
    let active = grant.tab_id == active_tab;
    let active_class = if active { "active" } else { "" };
    format!(
        r##"<div class="tab {active_class}" role="tab" tabindex="0" aria-controls="pane-{id}" aria-selected="{selected}" data-pane="{id}" data-tab-id="{id}" data-visibility="{visibility}" hx-get="/admit/{id}" hx-target="[data-view-panel='{id}']" hx-swap="innerHTML" hx-trigger="{hx_trigger}">{visibility_button}<span class="tab-name">{title}</span>{star_button}</div>"##,
        selected = active,
        hx_trigger = "immortal-floor-admit",
    )
}
