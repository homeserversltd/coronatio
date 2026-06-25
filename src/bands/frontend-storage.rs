fn frontend_storage_readback() -> FrontendStorageReadback {
    FrontendStorageReadback {
        schema: "coronatio.frontend-storage.contract.v1".to_string(),
        status: "contract-only".to_string(),
        route: "/api/frontend/storage".to_string(),
        quarry_sources: vec![
            "src/store/index.ts".to_string(),
            "src/store/slices/themeSlice.ts".to_string(),
            "src/store/slices/tabSlice.ts".to_string(),
            "src/store/slices/visibilitySlice.ts".to_string(),
            "src/store/slices/favoriteSlice.ts".to_string(),
            "src/store/slices/startupSlice.ts".to_string(),
            "src/App.tsx".to_string(),
            "src/api/auth.ts".to_string(),
        ],
        persisted_stores: vec![
            PersistedStoreLaw {
                store_name: "main zustand store".to_string(),
                storage_key: "homeserver-store".to_string(),
                source_path: "src/store/index.ts persist(partialize)".to_string(),
                persisted_fields: vec!["theme", "visibility", "starredTab", "isInitialized", "tabs", "activeTab"].into_iter().map(String::from).collect(),
                boundary: "mixed server truth and browser preference; Coronatio splits this before migration".to_string(),
            },
            PersistedStoreLaw {
                store_name: "auth zustand store".to_string(),
                storage_key: "auth-storage".to_string(),
                source_path: "src/api/auth.ts persist(partialize)".to_string(),
                persisted_fields: vec!["isAdmin"].into_iter().map(String::from).collect(),
                boundary: "browser remembrance only; privileged authority remains token/session receipt, never localStorage".to_string(),
            },
            PersistedStoreLaw {
                store_name: "theme data cache".to_string(),
                storage_key: "themeData".to_string(),
                source_path: "src/store/slices/themeSlice.ts THEME_DATA_STORAGE_KEY".to_string(),
                persisted_fields: vec!["themeData"].into_iter().map(String::from).collect(),
                boundary: "browser cosmetic cache; server theme catalog remains authority".to_string(),
            },
        ],
        persistence_fields: frontend_persistence_fields(),
        debounce_law: vec![
            DebounceLaw {
                source: "src/store/index.ts debouncedSetItem".to_string(),
                interval_ms: 500,
                purpose: "reduce repeated homeserver-store localStorage writes and skip unchanged serialized state".to_string(),
                coronatio_rule: "write browser preference snapshots only after settle; server-owned state is not hidden behind browser debounce".to_string(),
            },
            DebounceLaw {
                source: "src/App.tsx loadTablet duplicate-load debounce".to_string(),
                interval_ms: 500,
                purpose: "avoid immediately reloading the same tablet unless recovering from fallback".to_string(),
                coronatio_rule: "pane loading may debounce duplicate dynamic-cartridge loads; current route state remains explicit".to_string(),
            },
            DebounceLaw {
                source: "src/App.tsx tablet load timeout".to_string(),
                interval_ms: 15000,
                purpose: "show fallback when a tablet module stalls".to_string(),
                coronatio_rule: "dynamic cartridge timeout becomes a typed fallback receipt, not silent local state drift".to_string(),
            },
            DebounceLaw {
                source: "src/store/slices/startupSlice.ts tab config fetch timeout".to_string(),
                interval_ms: 7000,
                purpose: "abort stale /api/tabs startup config fetch and retry before default fallback".to_string(),
                coronatio_rule: "startup config has bounded fetch/retry receipts; stale local storage never overrides admitted registry".to_string(),
            },
        ],
        stale_state_law: vec![
            StaleStateLaw {
                source: "src/store/index.ts storage.getItem".to_string(),
                stale_condition: "invalid JSON or parse failure in homeserver-store".to_string(),
                old_recovery: "log parse error and return null".to_string(),
                coronatio_rule: "ignore malformed browser snapshot and reload server registry plus safe defaults".to_string(),
            },
            StaleStateLaw {
                source: "src/utils/bootstrap.ts determineInitialTab".to_string(),
                stale_condition: "starred tab missing, hidden, disabled, admin-only without admin, or fallback-only".to_string(),
                old_recovery: "choose visible enabled server starred tab, else first visible non-admin/admin-allowed tab, else fallback".to_string(),
                coronatio_rule: "server registry decides first pane; browser activeTab is advisory and clipped to visible admitted panes".to_string(),
            },
            StaleStateLaw {
                source: "src/App.tsx stale load detection".to_string(),
                stale_condition: "async tablet load completes after a newer load id or after unmount".to_string(),
                old_recovery: "discard stale module result".to_string(),
                coronatio_rule: "dynamic cartridge load receipts carry generation id and stale completions do not mutate visible pane".to_string(),
            },
            StaleStateLaw {
                source: "src/store/slices/visibilitySlice.ts updateTabVisibility".to_string(),
                stale_condition: "backend visibility update fails".to_string(),
                old_recovery: "revert local visibility to previous value and recalculate starred fallback".to_string(),
                coronatio_rule: "server visibility transaction is authority; browser optimistic state rolls back on failed receipt".to_string(),
            },
        ],
        coronatio_ownership: vec![
            StateOwnershipLaw { state_family: "tabs".to_string(), owner: "server registry /api/registry and dynamic cartridge manifests".to_string(), reason: "tab catalog and native/dynamic lane admission are product truth".to_string() },
            StateOwnershipLaw { state_family: "visibility".to_string(), owner: "server registry transaction".to_string(), reason: "visibility controls appliance access and fallback law".to_string() },
            StateOwnershipLaw { state_family: "starredTab".to_string(), owner: "server registry with browser advisory bootstrap".to_string(), reason: "first pane must survive device changes and visibility changes".to_string() },
            StateOwnershipLaw { state_family: "activeTab".to_string(), owner: "browser session preference clipped by server registry".to_string(), reason: "current pane is local navigation, not product configuration".to_string() },
            StateOwnershipLaw { state_family: "theme".to_string(), owner: "browser preference using server theme catalog".to_string(), reason: "theme is cosmetic unless a server profile later claims it".to_string() },
            StateOwnershipLaw { state_family: "isInitialized".to_string(), owner: "runtime startup phase receipt".to_string(), reason: "initialization is live process state and must not be trusted from stale storage".to_string() },
            StateOwnershipLaw { state_family: "isAdmin".to_string(), owner: "Caduceus/session token receipt".to_string(), reason: "admin authority cannot be granted by localStorage".to_string() },
        ],
        migration_path: vec![
            "read existing homeserver-store snapshot if present".to_string(),
            "drop malformed JSON and all credential/token-shaped values".to_string(),
            "accept theme only when present in server theme catalog".to_string(),
            "accept activeTab only when admitted, visible, enabled, and accessible to current role".to_string(),
            "migrate starredTab to server registry only when visible enabled non-admin tab or fallback".to_string(),
            "migrate visibility/tabs from server registry, not from stale browser snapshot".to_string(),
            "write new Coronatio browser preference key after successful server readback".to_string(),
            "leave old homeserver-store readable for one migration pass, then ignore".to_string(),
        ],
        forbidden_persistence: vec![
            "adminToken".to_string(),
            "PIN".to_string(),
            "password".to_string(),
            "session token".to_string(),
            "API key".to_string(),
            "credential-bearing theme or tab data".to_string(),
        ],
        first_missing_live_signal: "Coronatio storage migration adapter and browser preference key are not wired; this tranche exposes the contract only".to_string(),
    }
}

fn frontend_persistence_fields() -> Vec<PersistedFieldLaw> {
    vec![
        field_law(
            "theme",
            "src/store/index.ts + themeSlice.ts",
            "persisted in homeserver-store and themeData cache",
            "browser preference",
            "preserve if theme exists in server catalog, otherwise default",
        ),
        field_law(
            "visibility",
            "src/store/index.ts + visibilitySlice.ts",
            "persisted locally but also posted to /tabs/visibility",
            "server registry transaction",
            "reload from server; local snapshot is fallback evidence only",
        ),
        field_law(
            "starredTab",
            "src/store/index.ts + favoriteSlice.ts",
            "persisted locally and posted to /setstarredtab",
            "server registry",
            "preserve only if visible/enabled and role-accessible, else first visible tab/fallback",
        ),
        field_law(
            "isInitialized",
            "src/store/index.ts + tabSlice.ts",
            "persisted as initialization flag",
            "startup receipt",
            "do not trust from storage; recompute during startup",
        ),
        field_law(
            "tabs",
            "src/store/index.ts + startupSlice.ts",
            "persisted full tab state",
            "server registry /api/tabs",
            "treat browser copy as stale cache; server response wins",
        ),
        field_law(
            "activeTab",
            "src/store/index.ts + App.tsx",
            "persisted current navigation target",
            "browser session preference",
            "clip to admitted visible tab; never load hidden/disabled/admin-forbidden tab",
        ),
        field_law(
            "isAdmin",
            "src/api/auth.ts auth-storage",
            "persisted boolean",
            "Caduceus/session token receipt",
            "discard as authority; require valid token/session proof",
        ),
        field_law(
            "themeData",
            "src/store/slices/themeSlice.ts",
            "persisted theme object under themeData",
            "browser cosmetic cache",
            "validate hex color schema and catalog membership before reuse",
        ),
    ]
}

fn field_law(
    field: &str,
    old_source: &str,
    old_behavior: &str,
    coronatio_owner: &str,
    migration_rule: &str,
) -> PersistedFieldLaw {
    PersistedFieldLaw {
        field: field.to_string(),
        old_source: old_source.to_string(),
        old_behavior: old_behavior.to_string(),
        coronatio_owner: coronatio_owner.to_string(),
        migration_rule: migration_rule.to_string(),
    }
}

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

