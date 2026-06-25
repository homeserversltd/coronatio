fn service_data_readback() -> ServiceDataReadback {
    ServiceDataReadback {
        schema: "coronatio.service-data.contract.v1".to_string(),
        status: "contract-only".to_string(),
        route: "/api/services/data".to_string(),
        portal_schema: PortalSchema {
            source_path: "homeserver.json tabs.portals.data.portals[]".to_string(),
            fields: vec!["name", "description", "services", "type", "port", "localURL", "remoteURL"].into_iter().map(String::from).collect(),
            required_fields: vec!["name", "description", "localURL"].into_iter().map(String::from).collect(),
            portal_types: vec!["systemd", "script", "link"].into_iter().map(String::from).collect(),
            validation_rules: vec![
                ValidationRule { field: "name".to_string(), rule: "non-empty unique portal name".to_string() },
                ValidationRule { field: "services".to_string(), rule: "non-link portals require at least one service".to_string() },
                ValidationRule { field: "port".to_string(), rule: "non-link portals require unique integer port 1..65535".to_string() },
                ValidationRule { field: "localURL".to_string(), rule: "non-empty local URL is the primary appliance link".to_string() },
                ValidationRule { field: "factory".to_string(), rule: "factory portals are read-only and cannot be deleted by custom portal mutation".to_string() },
            ],
            factory_portal_law: "factory portals are loaded from factory config for readback and protected from deletion; custom portals mutate only through later Caduceus config transactions".to_string(),
        },
        service_card_schema: ServiceCardSchema {
            source_paths: vec![
                "backend/portals/routes.py /api/portals".to_string(),
                "backend/monitors/services.py ServicesMonitor.collect_status".to_string(),
                "backend/utils/utils.py get_service_status".to_string(),
            ],
            fields: vec!["name", "systemdName", "isEnabled", "isActive", "status", "statusDetails", "isScriptManaged", "port", "needsReboot"].into_iter().map(String::from).collect(),
            systemd_resolution: "portal services resolve through exact service mapping, normalized name mapping, then .service suffix; active and enabled use systemctl".to_string(),
            script_managed_resolution: "script-managed portal services use port reachability as running proxy, assume enabled, and mark needsReboot true".to_string(),
            enabled_cache_policy: "is-enabled results are cached for 60 seconds in the old monitor; Coronatio records cache TTL as contract, not live systemctl state".to_string(),
        },
        monitor_topics: monitor_topic_laws(),
        broadcast_law: BroadcastLaw {
            transport_replacement: "old eventlet/Socket.IO broadcasters become Coronatio topic contracts and SSE/readback routes".to_string(),
            regular_delivery: "regular clients receive non-admin payload fields for core service, power, internet, tailscale, vpn, sync, and hard-drive-test topics".to_string(),
            admin_delivery: "admin sessions may receive registered admin fields and admin-only disk/system topics through Caduceus-gated capability".to_string(),
            change_detection: "topic-specific comparison rules decide broadcast eligibility; realtime power/system always pulse, services compare set/status/enabled, sync compares status/job/progress/keepalive".to_string(),
            ui_state_law: "monitor data becomes current service cards, indicator chips, and pane snapshots; unavailable collectors surface firstMissingSignal instead of fake green".to_string(),
        },
        admin_field_law: admin_field_filters(),
        first_missing_live_signal: "service collectors and monitor broadcasters are not wired; Coronatio does not run systemctl, ping, tailscale, vpn, disk, rsync, smartctl, or power sensors in this tranche".to_string(),
    }
}

fn monitor_topic_laws() -> Vec<MonitorTopicLaw> {
    vec![
        monitor_topic("services.status", "ServicesMonitor.broadcast_status", "SERVICES_CHECK_INTERVAL", vec!["name", "systemdName", "isActive", "status", "statusDetails", "isScriptManaged", "port", "needsReboot"], false, vec!["isEnabled"], "broadcast when service count/name/status/enabled state changes", "service cards and portal currentness readback"),
        monitor_topic("power.status", "PowerMonitor.broadcast_power_data", "POWER_SAMPLE_INTERVAL", vec!["current", "historical", "unit", "timestamp"], false, vec![], "always broadcast realtime power samples", "power indicator/currentness readback"),
        monitor_topic("system.stats", "SystemStatsMonitor.broadcast_stats", "STATS_INTERVAL", vec!["load", "cpu", "memory", "disk", "network", "timestamp"], false, vec!["processes", "users", "networkConnections"], "always broadcast realtime system stats", "stats pane snapshot/SSE payload"),
        monitor_topic("internet.status", "InternetStatusMonitor.broadcast_status", "INTERNET_CHECK_INTERVAL", vec!["status", "timestamp"], false, vec!["publicIp", "ipDetails", "dnsServers"], "broadcast on connectivity/public IP/DNS/error-validity changes", "internet indicator readback"),
        monitor_topic("tailscale.status", "TailscaleMonitor.broadcast_status", "TAILSCALE_CHECK_INTERVAL", vec!["status", "interface", "timestamp"], false, vec!["ip", "tailnet", "isEnabled", "loginUrl"], "broadcast on status/interface/admin-field/login URL changes", "tailscale indicator readback"),
        monitor_topic("vpn.status", "VPNMonitor.broadcast_status", "VPN_CHECK_INTERVAL", vec!["vpnStatus", "transmissionStatus", "timestamp"], false, vec!["connectionDetails", "credentials", "isEnabled"], "broadcast on vpn/transmission/enabled changes", "vpn indicator readback with credentials redacted"),
        monitor_topic("sync.status", "sync_monitor.broadcast_status", "2 seconds", vec!["status", "id", "progress", "timestamp", "message"], false, vec![], "broadcast on status/job/progress >=10% or working keepalive", "sync indicator and admin action readback"),
        monitor_topic("hard-drive-test.status", "HardDriveTestMonitor.broadcast_status", "DRIVE_TEST_INTERVAL", vec!["status", "device", "testType", "progress", "message", "timestamp"], false, vec![], "broadcast starting/done and working keepalive", "drive-test indicator/action readback"),
        monitor_topic("admin.disk.info", "DiskMonitor.broadcast_disk_info", "DISK_CHECK_INTERVAL", vec!["blockDevices", "diskUsage", "encryptionInfo", "nasCompatibleDevices", "timestamp"], true, vec![], "broadcast on device/encryption/NAS/mount/filesystem/UUID/usage/error changes or 5 minute pulse", "admin disk appliance readback"),
        monitor_topic("admin.system", "SystemStatsMonitor.broadcast_admin_stats", "ADMIN_STATS_INTERVAL", vec!["system stats plus admin detail fields"], true, vec![], "admin-only realtime system pulse", "admin diagnostics readback"),
    ]
}

fn monitor_topic(
    topic: &str,
    source_monitor: &str,
    cadence_source: &str,
    payload_fields: Vec<&str>,
    admin_only: bool,
    admin_fields: Vec<&str>,
    change_rule: &str,
    coronatio_contract: &str,
) -> MonitorTopicLaw {
    MonitorTopicLaw {
        topic: topic.to_string(),
        source_monitor: source_monitor.to_string(),
        cadence_source: cadence_source.to_string(),
        payload_fields: payload_fields.into_iter().map(String::from).collect(),
        admin_only,
        admin_fields: admin_fields.into_iter().map(String::from).collect(),
        change_rule: change_rule.to_string(),
        coronatio_contract: coronatio_contract.to_string(),
    }
}

fn boundary_readback() -> BoundaryReadback {
    BoundaryReadback {
        schema: "coronatio.route-boundary.v1".to_string(),
        api_unknown_path_policy: "/api/* misses return JSON 404 with coronatio.api.error.v1, never shell HTML".to_string(),
        static_shell_policy: "non-API unknown GET paths return the Coronatio shell for client-side routing".to_string(),
        cartridge_static_policy: "/tabs/<tab-id>/... is served from the configured tab root through safe tab ids and manifest validation".to_string(),
        cors_source: "homeserver.json global.cors.allowed_origins becomes Coronatio config law in the later config tranche".to_string(),
        premium_blueprint_replacement: "dynamic Flask blueprint injection is replaced by dynamic-cartridge, source-injection-recompile, or first-party-native lanes".to_string(),
    }
}

fn installer_readback() -> InstallerReadback {
    InstallerReadback {
        schema: "coronatio.installer.contract.v1".to_string(),
        status: "contract-only".to_string(),
        route: "/api/installer".to_string(),
        authority: "Coronatio exposes typed installer law readbacks; live host mutation remains behind a later Caduceus actuator membrane".to_string(),
        root_manifest_schema: PremiumRootManifestSchema {
            required_fields: vec!["name", "version", "config", "files"].into_iter().map(String::from).collect(),
            config_fields: vec!["repository.url", "repository.branch", "git_managed"].into_iter().map(String::from).collect(),
            file_sections: vec!["backend", "frontend", "permissions", "system", "config", "readme", "license"].into_iter().map(String::from).collect(),
            sample_source: "premium/youtube/index.json".to_string(),
        },
        component_manifest_schema: PremiumComponentManifestSchema {
            loci: vec!["frontend/index.json", "backend/index.json"].into_iter().map(String::from).collect(),
            fields: vec!["name", "version", "files[]", "source", "target", "type", "identifier", "marker", "description"].into_iter().map(String::from).collect(),
            operation_types: vec!["copy", "append", "symlink"].into_iter().map(String::from).collect(),
            blueprint_marker: "PREMIUM TAB BLUEPRINTS".to_string(),
        },
        file_operation_schema: PremiumFileOperationSchema {
            source_field: "source path relative to package root".to_string(),
            target_field: "absolute old-host target or Coronatio lane target declared by later actuator".to_string(),
            operation_type_field: "type defaults to copy; append uses marker/identifier".to_string(),
            identifier_field: "tab/package identifier for injected blocks and rollback".to_string(),
            marker_field: "append insertion marker such as PREMIUM TAB BLUEPRINTS".to_string(),
            description_field: "human receipt text for operation intent".to_string(),
            supported_operations: vec!["copy", "append", "symlink"].into_iter().map(String::from).collect(),
        },
        validation_phases: installer_validation_phases(),
        install_phases: installer_install_phases(),
        rollback_law: installer_rollback_law(),
        lifecycle_law: installer_lifecycle_law(),
        lane_mapping: installer_lane_mapping(),
        first_missing_live_signal: "Caduceus installer actuator and receipt ledger are not wired; no package files, dependencies, config, build, or service state are mutated by Coronatio".to_string(),
    }
}

fn installer_validation_phases() -> Vec<InstallerPhase> {
    vec![
        installer_phase(1, "current-config-validation", "validate_config_with_factory_fallback before package work", "read current Coronatio registry/config and reject invalid baseline before staging mutation", "read-only Coronatio contract now; Caduceus later"),
        installer_phase(2, "package-manifest-validation", "validate_package_manifest over root/component manifests", "require root package schema and frontend/backend operation schema before admission", "read-only Coronatio contract now; Caduceus later"),
        installer_phase(3, "name-collision", "check_name_collision unless reinstall/batch post-build restoration skips installed check", "reject duplicate visible tab/package identity unless reinstall authority is explicit", "read-only Coronatio contract now; Caduceus later"),
        installer_phase(4, "version-conflict", "SemanticVersionChecker.validate_premium_tab_dependencies", "surface version conflicts as typed blockers before file/package mutation", "read-only Coronatio contract now; Caduceus later"),
        installer_phase(5, "dependency-validation", "backend requirements, frontend package patch, and system dependencies are detected before install", "classify dependency families without installing them in Coronatio", "read-only Coronatio contract now; Caduceus later"),
    ]
}

fn installer_install_phases() -> Vec<InstallerPhase> {
    vec![
        installer_phase(1, "backend-file-operations", "root files.backend and backend/index.json copy/append operations, including blueprint marker injection", "map backend operations to dynamic service boundary or source-injection lane; Flask blueprint injection is quarry only", "Caduceus later"),
        installer_phase(2, "frontend-file-operations", "root files.frontend and frontend/index.json copy operations into src/tablets/<tab>", "map frontend payload to dynamic cartridge static assets or source-injection recompile", "Caduceus later"),
        installer_phase(3, "permissions-files", "files.permissions copied to /etc/sudoers.d in the old installer", "permissions require privileged Caduceus policy and receipt before any live write", "Caduceus only"),
        installer_phase(4, "root-files", "root config/readme/license and other files copied under the old tablet root", "non-code payloads become manifest/docs/license artifacts inside the installed cartridge", "Caduceus later"),
        installer_phase(5, "package-installations", "backend/requirements.txt, frontend/package.patch.json, system/dependencies.json", "dependency mutation is declared but not executed by Coronatio contract route", "Caduceus only"),
        installer_phase(6, "config-patches", "homeserver.patch.json applied to homeserver.json", "later registry transaction law owns deep merge, validation, atomic write, and permissions", "Caduceus later"),
        installer_phase(7, "tab-hooks", "tab-specific hooks such as backupTab venv note and chiaTab key ownership", "hooks must become typed per-tab Caduceus actions, never ad-hoc Python side effects", "Caduceus only"),
        installer_phase(8, "frontend-rebuild", "BuildManager.rebuild_frontend unless batch mode defers it", "source-injection-recompile lane requires build/test/admission before restart", "Caduceus plus Cibation for repo-backed source"),
        installer_phase(9, "service-restart", "ServiceManager.restart_homeserver_services unless batch mode defers it", "restart is an explicit post-build live-body proof, not part of this readback route", "Caduceus only"),
    ]
}

fn installer_phase(
    sequence: u64,
    id: &str,
    source_law: &str,
    coronatio_contract: &str,
    mutation_authority: &str,
) -> InstallerPhase {
    InstallerPhase {
        id: id.to_string(),
        sequence,
        source_law: source_law.to_string(),
        coronatio_contract: coronatio_contract.to_string(),
        mutation_authority: mutation_authority.to_string(),
    }
}

fn installer_rollback_law() -> RollbackLaw {
    RollbackLaw {
        schema: "coronatio.installer.rollback.v1".to_string(),
        order: vec!["config rollback", "package rollback", "file operation rollback", "service state rollback"].into_iter().map(String::from).collect(),
        config_restore: "restore config backup before package/file cleanup; restore-patches reapplies installed tab homeserver.patch.json files alphabetically for post-build recovery".to_string(),
        file_operation_reversal: "copy/symlink outputs are removed or restored from backups; append blocks are removed by marker and identifier".to_string(),
        service_state_restore: "captured service states are restored after file/package rollback".to_string(),
        batch_restore: "batch mode may defer build/restart and may fall back to individual tab installation while preserving success/failure lists".to_string(),
    }
}

fn installer_lifecycle_law() -> Vec<InstallerLifecycleLaw> {
    vec![
        InstallerLifecycleLaw {
            action: "install".to_string(),
            sequence: vec![
                "validate current config",
                "validate manifest",
                "check collision",
                "validate dependencies",
                "perform files",
                "install packages",
                "apply config patches",
                "run hooks",
                "rebuild frontend",
                "restart service",
            ]
            .into_iter()
            .map(String::from)
            .collect(),
            post_build_policy:
                "single install rebuilds/restarts immediately unless batch mode defers".to_string(),
        },
        InstallerLifecycleLaw {
            action: "uninstall".to_string(),
            sequence: vec![
                "find installed tab",
                "remove registered package/file/config effects",
                "optionally skip build/restart during batch/reinstall",
                "rebuild/restart after final operation",
            ]
            .into_iter()
            .map(String::from)
            .collect(),
            post_build_policy:
                "uninstall manager owns cleanup; Coronatio only reads law until Caduceus exists"
                    .to_string(),
        },
        InstallerLifecycleLaw {
            action: "reinstall".to_string(),
            sequence: vec![
                "prove installed state or locate available package",
                "uninstall with skip_build_and_restart",
                "install with name collision bypass",
                "batch reinstall defers final build/restart",
            ]
            .into_iter()
            .map(String::from)
            .collect(),
            post_build_policy:
                "reinstall preserves one final build/restart boundary after successful replacement"
                    .to_string(),
        },
        InstallerLifecycleLaw {
            action: "restore-patches".to_string(),
            sequence: vec![
                "list installed premium tabs",
                "find homeserver.patch.json",
                "sort by tab name",
                "apply each patch deterministically",
            ]
            .into_iter()
            .map(String::from)
            .collect(),
            post_build_policy:
                "post-build config recovery only; it does not reinstall files or dependencies"
                    .to_string(),
        },
    ]
}

fn installer_lane_mapping() -> Vec<InstallerLaneMapping> {
    vec![
        InstallerLaneMapping { install_mode: InstallMode::DynamicCartridge, accepted_package: "manifest + static assets + optional localhost service boundary".to_string(), post_install_requirement: "reload registry and serve /tabs/<tab-id>/... without Coronatio recompile".to_string(), rejected_shape: "Flask blueprint injection as live runtime authority".to_string() },
        InstallerLaneMapping { install_mode: InstallMode::SourceInjectionRecompile, accepted_package: "trusted frontend/backend source operations requiring host rebuild".to_string(), post_install_requirement: "copy source in a governed lane, run Rust/source proof, publish/admit source, then restart through Caduceus when requested".to_string(), rejected_shape: "unreviewed source mutation in canonical checkout".to_string() },
        InstallerLaneMapping { install_mode: InstallMode::FirstPartyNative, accepted_package: "none for user packages; only Coronatio source may define this lane".to_string(), post_install_requirement: "Cibation-admitted Rust source and tests".to_string(), rejected_shape: "premium package claiming first-party-native".to_string() },
    ]
}

fn stats_snapshot() -> StatsSnapshot {
    let resources = stats_resources();
    let storage = stats_storage();
    let network = stats_network();
    let io = stats_io(&storage);
    let services = stats_services();
    let first_missing_signal = stats_first_missing_signal(&storage, &services);
    StatsSnapshot {
        schema: "coronatio.stats.snapshot.v1".to_string(),
        pane_id: "stats".to_string(),
        product: "Coronatio".to_string(),
        doctrine: StatsViewportDoctrine {
            quarry_sources: vec![
                "serverGenesis original var-www-homeserver app/tabs/stats/stats.js".to_string(),
                "serverGenesis original serverbox basic tabs stats.html".to_string(),
                "Coronatio North Star first-party native lane".to_string(),
            ],
            preserved_sections: vec![
                "resources".to_string(),
                "storage".to_string(),
                "network".to_string(),
                "connections".to_string(),
                "disk I/O chart".to_string(),
                "services".to_string(),
                "SSE lease controls".to_string(),
            ],
            refresh_seconds: 5,
            authority: "read-only Rust snapshot from /proc and df; host mutation remains behind Caduceus".to_string(),
        },
        transport: StatsTransport {
            snapshot_route: "/api/stats".to_string(),
            event_route: "/api/stats/events".to_string(),
            renew_route: "/api/stats/events/renew".to_string(),
            stream_status: "available".to_string(),
            stream_reason: "stats SSE event frame and renewal route are registered; the viewport can read snapshot and event authority".to_string(),
        },
        resources: resources.clone(),
        storage: storage.clone(),
        network: network.clone(),
        io,
        services: services.clone(),
        telemetry: StatsTelemetry {
            load1: resources.load.one,
            cpu_temperature_celsius: resources.load.cpu_temperature_celsius,
            service_health: Some(service_health_summary(&services)),
            storage_posture: Some(storage_posture_summary(&storage)),
            first_missing_signal,
        },
        next_routes: StatsNextRoutes {
            snapshot: "/api/stats".to_string(),
            events: "/api/stats/events".to_string(),
            renew: "/api/stats/events/renew".to_string(),
        },
    }
}

fn stats_resources() -> StatsResources {
    let load = std::fs::read_to_string("/proc/loadavg")
        .ok()
        .and_then(|raw| {
            let mut parts = raw.split_whitespace();
            Some(StatsLoad {
                one: parts.next().and_then(|value| value.parse().ok()),
                five: parts.next().and_then(|value| value.parse().ok()),
                fifteen: parts.next().and_then(|value| value.parse().ok()),
                cpu_temperature_celsius: read_cpu_temperature_celsius(),
            })
        })
        .unwrap_or(StatsLoad {
            one: None,
            five: None,
            fifteen: None,
            cpu_temperature_celsius: read_cpu_temperature_celsius(),
        });
    let meminfo = parse_meminfo();
    StatsResources {
        load,
        memory: memory_from_meminfo(&meminfo, "MemTotal", "MemAvailable"),
        swap: memory_from_meminfo(&meminfo, "SwapTotal", "SwapFree"),
    }
}

fn parse_meminfo() -> BTreeMap<String, u64> {
    let mut map = BTreeMap::new();
    if let Ok(raw) = std::fs::read_to_string("/proc/meminfo") {
        for line in raw.lines() {
            if let Some((key, rest)) = line.split_once(':') {
                if let Some(value) = rest.split_whitespace().next().and_then(|v| v.parse::<u64>().ok()) {
                    map.insert(key.to_string(), value * 1024);
                }
            }
        }
    }
    map
}

fn memory_from_meminfo(meminfo: &BTreeMap<String, u64>, total_key: &str, free_key: &str) -> StatsMemory {
    let total = meminfo.get(total_key).copied();
    let free = meminfo.get(free_key).copied();
    let used = match (total, free) {
        (Some(total), Some(free)) if total >= free => Some(total - free),
        _ => None,
    };
    let percent = match (used, total) {
        (Some(used), Some(total)) if total > 0 => Some(((used.saturating_mul(100)) / total).min(100) as u8),
        _ => None,
    };
    StatsMemory { total_bytes: total, used_bytes: used, free_bytes: free, percent }
}

fn read_cpu_temperature_celsius() -> Option<f64> {
    for path in [
        "/sys/class/thermal/thermal_zone0/temp",
        "/sys/class/hwmon/hwmon0/temp1_input",
        "/sys/class/hwmon/hwmon1/temp1_input",
    ] {
        if let Ok(raw) = std::fs::read_to_string(path) {
            if let Ok(value) = raw.trim().parse::<f64>() {
                return Some((value / 1000.0 * 10.0).round() / 10.0);
            }
        }
    }
    None
}

