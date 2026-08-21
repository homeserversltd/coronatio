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
        admin_runtime: admin_runtime_readback(),
        first_missing_live_signal: "service collectors and monitor broadcasters are not wired; Coronatio reads admin disk and service display facts directly from the local host while mutation remains behind Caduceus".to_string(),
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

#[allow(clippy::too_many_arguments)]
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
        flask_quarry_blueprint_replacement: "dynamic Flask blueprint injection is replaced by dynamic-cartridge, source-injection-recompile, or first-party-native lanes".to_string(),
    }
}

fn installer_readback() -> InstallerReadback {
    InstallerReadback {
        schema: "coronatio.installer.contract.v1".to_string(),
        status: "contract-only".to_string(),
        route: "/api/installer".to_string(),
        authority: "Coronatio exposes typed installer law readbacks; live host mutation remains behind a later Caduceus actuator membrane".to_string(),
        root_manifest_schema: LegacyInstallerRootManifestSchema {
            required_fields: vec!["name", "version", "config", "files"].into_iter().map(String::from).collect(),
            config_fields: vec!["repository.url", "repository.branch", "git_managed"].into_iter().map(String::from).collect(),
            file_sections: vec!["backend", "frontend", "permissions", "system", "config", "readme", "license"].into_iter().map(String::from).collect(),
            sample_source: "legacy-tabs/quarry/index.json".to_string(),
        },
        component_manifest_schema: LegacyInstallerComponentManifestSchema {
            loci: vec!["frontend/index.json", "backend/index.json"].into_iter().map(String::from).collect(),
            fields: vec!["name", "version", "files[]", "source", "target", "type", "identifier", "marker", "description"].into_iter().map(String::from).collect(),
            operation_types: vec!["copy", "append", "symlink"].into_iter().map(String::from).collect(),
            blueprint_marker: "LEGACY INSTALLER BLUEPRINTS".to_string(),
        },
        file_operation_schema: LegacyInstallerFileOperationSchema {
            source_field: "source path relative to package root".to_string(),
            target_field: "absolute old-host target or Coronatio lane target declared by later actuator".to_string(),
            operation_type_field: "type defaults to copy; append uses marker/identifier".to_string(),
            identifier_field: "tab/package identifier for injected blocks and rollback".to_string(),
            marker_field: "append insertion marker such as LEGACY INSTALLER BLUEPRINTS".to_string(),
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
        installer_phase(4, "version-conflict", "SemanticVersionChecker.validate_legacy_installer_tab_dependencies", "surface version conflicts as typed blockers before file/package mutation", "read-only Coronatio contract now; Caduceus later"),
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
        installer_phase(7, "tab-hooks", "tab-specific hooks such as backupTab venv note", "hooks must become typed per-tab Caduceus actions, never ad-hoc Python side effects", "Caduceus only"),
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
                "list recorded quarry tab patches",
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
        InstallerLaneMapping { install_mode: InstallMode::FirstPartyNative, accepted_package: "none for user packages; only Coronatio source may define this lane".to_string(), post_install_requirement: "Cibation-admitted Rust source and tests".to_string(), rejected_shape: "legacy installer package claiming first-party-native".to_string() },
    ]
}

fn caduceus_stats_value(path: &str) -> serde_json::Value { caduceus_http("GET", path).body }

fn stats_number(value: Option<&serde_json::Value>) -> Option<f64> { value.and_then(|value| value.as_f64().or_else(|| value.as_u64().map(|value| value as f64))) }
fn stats_u64(value: Option<&serde_json::Value>) -> Option<u64> { value.and_then(|value| value.as_u64().or_else(|| value.as_f64().and_then(|value| (value >= 0.0).then_some(value as u64)))) }
fn stats_memory(memory: Option<&serde_json::Value>, total_key: &str, free_key: &str, used_key: Option<&str>) -> StatsMemory {
    let total=stats_u64(memory.and_then(|v|v.get(total_key))); let free=stats_u64(memory.and_then(|v|v.get(free_key))); let used=used_key.and_then(|k|stats_u64(memory.and_then(|v|v.get(k)))).or_else(||total.zip(free).map(|(t,f)|t.saturating_sub(f))); let percent=total.filter(|t|*t>0).and_then(|t|used.map(|u|((u.saturating_mul(100)/t).min(100)) as u8)); StatsMemory{total_bytes:total,used_bytes:used,free_bytes:free,percent}
}
fn stats_caduceus_snapshot(body: serde_json::Value) -> StatsSnapshot {
    let load=body.get("load"); let memory=body.get("memory"); let temperature=body.pointer("/temperature/celsius");
    let resources=StatsResources{load:StatsLoad{one:stats_number(load.and_then(|v|v.get("one"))),five:stats_number(load.and_then(|v|v.get("five"))),fifteen:stats_number(load.and_then(|v|v.get("fifteen"))),cpu_temperature_celsius:stats_number(temperature)},memory:stats_memory(memory,"MemTotal","MemAvailable",Some("usedBytes")),swap:stats_memory(memory,"SwapTotal","SwapFree",Some("usedBytesSwap"))};
    let interfaces=body.pointer("/network/interfaces").and_then(serde_json::Value::as_array).map(|rows|rows.iter().filter_map(|row|Some(StatsNetworkInterface{name:row.get("name")?.as_str()?.to_string(),status:row.get("operstate").and_then(|v|v.as_str()).unwrap_or("unknown").to_string(),rx_bytes:stats_u64(row.get("rxBytes")).unwrap_or(0),tx_bytes:stats_u64(row.get("txBytes")).unwrap_or(0)})).collect()).unwrap_or_default();
    let tcp=body.get("tcp"); let established=stats_u64(tcp.and_then(|v|v.get("established"))).unwrap_or(0); let listening=stats_u64(tcp.and_then(|v|v.get("listen"))).unwrap_or(0); let total=tcp.and_then(serde_json::Value::as_object).map(|v|v.values().filter_map(serde_json::Value::as_u64).sum()).unwrap_or(0);
    let storage: Vec<StatsDrive>=body.pointer("/disk/usage").and_then(serde_json::Value::as_array).map(|rows|rows.iter().map(|r|StatsDrive{name:r.get("filesystem").and_then(|v|v.as_str()).unwrap_or("storage").to_string(),mount:r.get("path").and_then(|v|v.as_str()).unwrap_or("").to_string(),total_bytes:stats_u64(r.get("totalBytes")),used_bytes:stats_u64(r.get("usedBytes")),free_bytes:stats_u64(r.get("availableBytes")),usage_percent:r.get("usePercent").and_then(|v|v.as_str()).and_then(|v|v.trim_end_matches('%').parse().ok()),source:"Caduceus appliance stats".to_string()}).collect()).unwrap_or_default();
    let io=StatsIo{devices:body.pointer("/disk/io").and_then(serde_json::Value::as_array).map(|rows|rows.iter().map(|r|StatsIoDevice{device:r.get("device").and_then(|v|v.as_str()).unwrap_or("disk").to_string(),mount:r.get("mount").and_then(|v|v.as_str()).unwrap_or("").to_string(),read_bytes:stats_u64(r.get("readBytes")).unwrap_or(0),write_bytes:stats_u64(r.get("writeBytes")).unwrap_or(0)}).collect()).unwrap_or_default()};
    let processes=body.get("processes").and_then(serde_json::Value::as_array).map(|rows|rows.iter().map(|r|StatsProcess{name:r.get("command").or_else(||r.get("name")).and_then(|v|v.as_str()).unwrap_or("process").to_string(),cpu_percent:stats_number(r.get("cpuPercent")).unwrap_or(0.0),memory_bytes:stats_u64(r.get("rssBytes").or_else(||r.get("memoryBytes"))).unwrap_or(0),process_count:stats_u64(r.get("processCount")).unwrap_or(1)}).collect()).unwrap_or_default();
    let network=StatsNetwork{interfaces,connections:StatsConnectionCounts{established,listening,total}}; let services=stats_services();
    StatsSnapshot{schema:"coronatio.stats.snapshot.v1".to_string(),pane_id:"stats".to_string(),product:"Coronatio".to_string(),doctrine:StatsViewportDoctrine{quarry_sources:vec!["Caduceus appliance stats readback".to_string()],preserved_sections:vec!["cpu-chart".to_string(),"network".to_string(),"io-section".to_string(),"memory".to_string(),"disk-usage".to_string(),"kea-leases".to_string(),"process-usage".to_string()],refresh_seconds:5,authority:"read-only Caduceus appliance stats snapshot".to_string()},transport:StatsTransport{snapshot_route:"/api/stats".to_string(),event_route:"/api/stats/pulse".to_string(),renew_route:"/api/stats/pulse/renew".to_string(),stream_status:"available".to_string(),stream_reason:"persistent SSE pulse stream and renewal route are registered; pokes carry no stats payload".to_string()},resources,storage:storage.clone(),network,io,leases:stats_kea_leases(),kea_leases:stats_identity_roster(),processes,services:services.clone(),telemetry:StatsTelemetry{load1:stats_number(load.and_then(|v|v.get("one"))),cpu_temperature_celsius:stats_number(temperature),service_health:Some(service_health_summary(&services)),storage_posture:Some(storage_posture_summary(&storage)),first_missing_signal:String::new()},next_routes:StatsNextRoutes{snapshot:"/api/stats".to_string(),events:"/api/stats/pulse".to_string(),renew:"/api/stats/pulse/renew".to_string()}}
}
async fn stats_snapshot() -> StatsSnapshot { stats_caduceus_snapshot(caduceus_stats_value("/api/v1/appliance/stats")) }
async fn stats_history() -> impl IntoResponse { let readback=caduceus_http("GET","/api/v1/appliance/stats/history"); (StatusCode::from_u16(readback.status).unwrap_or(StatusCode::BAD_GATEWAY),Json(readback.body)) }
