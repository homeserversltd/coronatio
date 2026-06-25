fn native_crown_panes() -> Vec<CrownPane> {
    vec![
        CrownPane {
            id: "admin".to_string(),
            title: "Admin".to_string(),
            role: "session and crown policy".to_string(),
            summary: "PIN depth, capability posture, install receipts, and controlled mutation entrypoints.".to_string(),
            order: 0,
            admin_only: true,
            install_mode: InstallMode::FirstPartyNative,
            route: "/#admin".to_string(),
            state_route: "/api/panes/admin".to_string(),
        },
        CrownPane {
            id: "stats".to_string(),
            title: "Stats".to_string(),
            role: "machine telemetry".to_string(),
            summary: "System load, service health, storage posture, and later SSE live readback.".to_string(),
            order: 10,
            admin_only: false,
            install_mode: InstallMode::FirstPartyNative,
            route: "/#stats".to_string(),
            state_route: "/api/stats".to_string(),
        },
        CrownPane {
            id: "portals".to_string(),
            title: "Portals".to_string(),
            role: "service ingress".to_string(),
            summary: "Admitted HOMESERVER services, local ingress, remote ingress, and service currentness.".to_string(),
            order: 20,
            admin_only: false,
            install_mode: InstallMode::FirstPartyNative,
            route: "/#portals".to_string(),
            state_route: "/api/panes/portals".to_string(),
        },
        CrownPane {
            id: "upload".to_string(),
            title: "Upload".to_string(),
            role: "file ingress".to_string(),
            summary: "Safe file admission into HOMESERVER storage with policy and receipt readbacks.".to_string(),
            order: 30,
            admin_only: false,
            install_mode: InstallMode::FirstPartyNative,
            route: "/#upload".to_string(),
            state_route: "/api/panes/upload".to_string(),
        },
    ]
}

fn default_enabled() -> bool {
    true
}

fn validate_tab_manifest(manifest: &TabManifest) -> Result<(), String> {
    if !is_safe_tab_id(&manifest.id) {
        return Err("id must be lowercase hyphen-case ascii".to_string());
    }
    if manifest.title.trim().is_empty() {
        return Err("title must be present".to_string());
    }
    if manifest.install_mode == InstallMode::FirstPartyNative {
        return Err(
            "first-party-native panes are compiled crown law, not user cartridge manifests"
                .to_string(),
        );
    }
    if manifest.order < 0 {
        return Err("order must be non-negative".to_string());
    }
    if manifest.route_prefix.is_empty() {
        return Err("routePrefix must be present".to_string());
    }
    let expected = format!("/api/tabs/{}", manifest.id);
    if manifest.route_prefix != expected {
        return Err(format!("routePrefix must equal {expected}"));
    }
    if manifest.static_dir.contains("..") || manifest.static_dir.starts_with('/') {
        return Err("staticDir must be relative and stay inside the tab root".to_string());
    }
    Ok(())
}

fn registry_readback() -> RegistryReadback {
    let native_tab_contracts = native_tab_contracts();
    RegistryReadback {
        schema: "coronatio.registry.v1".to_string(),
        source_contract: "homeserver.json tabs.{config,visibility,data,starred}".to_string(),
        starred_tab: "portals".to_string(),
        default_route_tab: initial_tab(true, None, false),
        force_tab_bar_visibility: false,
        visible_tabs_user: visible_tab_ids(&native_tab_contracts, false),
        visible_tabs_admin: visible_tab_ids(&native_tab_contracts, true),
        validation_rules: registry_validation_rules(),
        native_tab_contracts,
    }
}

fn native_tab_contracts() -> Vec<CoronatioTabContract> {
    native_crown_panes()
        .into_iter()
        .map(|pane| CoronatioTabContract {
            id: pane.id.clone(),
            display_name: pane.title.clone(),
            order: pane.order,
            enabled: true,
            admin_only: pane.admin_only,
            visibility: TabVisibility::default(),
            install_mode: pane.install_mode,
            route: pane.route,
            state_route: pane.state_route,
        })
        .collect()
}

fn visible_tab_ids(tabs: &[CoronatioTabContract], is_admin: bool) -> Vec<String> {
    let mut visible = tabs
        .iter()
        .filter(|tab| tab.enabled && tab.visibility.tab && (!tab.admin_only || is_admin))
        .collect::<Vec<_>>();
    visible.sort_by(|left, right| left.order.cmp(&right.order).then(left.id.cmp(&right.id)));
    visible.into_iter().map(|tab| tab.id.clone()).collect()
}

fn registry_validation_rules() -> Vec<ValidationRule> {
    vec![
        ValidationRule { field: "id".to_string(), rule: "lowercase ascii letters, digits, and hyphen only; @ is normalized away before lookup".to_string() },
        ValidationRule { field: "displayName/title".to_string(), rule: "human visible title is required".to_string() },
        ValidationRule { field: "visibility.tab".to_string(), rule: "regular users see only enabled visible non-admin tabs; admins see enabled visible tabs including admin-only".to_string() },
        ValidationRule { field: "order".to_string(), rule: "non-negative sort key; ties sort by tab id for deterministic recovery".to_string() },
        ValidationRule { field: "starred".to_string(), rule: "default route pointer; invalid, disabled, hidden, or fallback resolves to first visible tab then fallback".to_string() },
        ValidationRule { field: "installMode".to_string(), rule: "dynamic-cartridge for runtime tabs, source-injection-recompile for trusted source lanes, first-party-native only for compiled crown panes".to_string() },
    ]
}

fn initial_tab(connection_ok: bool, forced_tab: Option<&str>, is_admin: bool) -> String {
    if let Some(tab) = forced_tab {
        return normalize_tab_id(tab);
    }
    if !connection_ok {
        return "fallback".to_string();
    }
    let contracts = native_tab_contracts();
    let visible = visible_tab_ids(&contracts, is_admin);
    let starred = normalize_tab_id("portals");
    if visible.iter().any(|tab| tab == &starred) {
        starred
    } else {
        visible
            .first()
            .cloned()
            .unwrap_or_else(|| "fallback".to_string())
    }
}

fn registry_transaction_readback() -> RegistryTransactionReadback {
    RegistryTransactionReadback {
        schema: "coronatio.registry.transaction.v1".to_string(),
        status: "contract-only".to_string(),
        route: "/api/registry/transaction".to_string(),
        source_contract: "premium/utils/config_manager.py apply_config_patch, deep_merge, deep_merge_tabs, validate_config_with_factory_fallback, restore_backup, revert_config_patch".to_string(),
        transaction_sequence: registry_transaction_phases(),
        deep_merge_law: DeepMergeLaw {
            object_merge: "when target and patch values are objects, merge recursively".to_string(),
            scalar_merge: "when either side is not an object, patch value replaces target value".to_string(),
            array_merge: "arrays are scalar values in this quarry law and are replaced, not appended".to_string(),
            tab_merge: "tabs object uses deep_merge_tabs so tab entries merge while starred is preserved and restored after new tab keys".to_string(),
        },
        starred_tab_law: StarredTabLaw {
            source_behavior: "deep_merge_tabs pops existing tabs.starred before merging source tabs and restores it at the end".to_string(),
            preservation_rule: "a package patch may add or update tab objects without displacing the existing starred pointer".to_string(),
            invalid_starred_resolution: "registry/startup law resolves invalid, hidden, disabled, or missing starred to first visible tab then fallback".to_string(),
            transaction_requirement: "later live mutation must preserve starred unless the patch explicitly carries an authorized starred transition".to_string(),
        },
        validation_law: ConfigValidationLaw {
            syntax_gate: "JSON syntax must parse before merge and before write".to_string(),
            factory_fallback_gate: "factoryFallback.sh equivalent validates candidate config and rejects .factory fallback output".to_string(),
            temp_validation: "candidate is written to a temp path, temporarily validated through the factory fallback path, then original config is restored before promote".to_string(),
            failure_posture: "validation failure removes candidate temp file and leaves current config standing".to_string(),
        },
        persistence_law: ConfigPersistenceLaw {
            backup_policy: "create timestamped backup before mutation when current config exists".to_string(),
            write_policy: "write merged candidate to temp config, validate it, then move temp into the live config path".to_string(),
            permission_restore: "after promote or restore, set owner www-data:www-data and mode 664 in the old host; Coronatio records desired owner/mode and leaves privileged chmod/chown to Caduceus".to_string(),
            missing_config_fallback: "if live config is absent, read /etc/homeserver.factory; if absent, use minimal tabs/global.cors.allowed_origins structure".to_string(),
            read_only_factory_posture: "factory fallback is source material for a candidate, not a durable replacement for the live config unless validation and promotion succeed".to_string(),
        },
        rollback_law: ConfigRollbackLaw {
            backup_restore: "restore backup to target path and restore permissions".to_string(),
            patch_revert: "remove keys matching patch values; nested objects recurse".to_string(),
            complete_tab_removal: "under tabs, patch-owned tab keys are removed as whole tab entries during revert".to_string(),
            mismatch_policy: "if current value differs from the patch value, do not remove it; report mismatch rather than deleting user state".to_string(),
        },
        first_missing_live_signal: "Caduceus registry transaction actuator is not wired; Coronatio does not write homeserver.json, run factoryFallback.sh, chown, chmod, or move temp configs".to_string(),
    }
}

fn registry_transaction_phases() -> Vec<RegistryTransactionPhase> {
    vec![
        registry_transaction_phase(1, "backup-current", "create_backup copies current config to /tmp/<name>.installer_backup.<timestamp>", "record backup policy and receipt fields before mutation", "Caduceus later"),
        registry_transaction_phase(2, "load-current-or-factory", "read homeserver.json, else /etc/homeserver.factory, else minimal valid structure", "classify live config, factory config, or minimal recovery candidate", "read-only Coronatio contract now; Caduceus later"),
        registry_transaction_phase(3, "deep-merge-patch", "deep_merge recursively merges objects and replaces scalars; tabs.starred is popped and restored", "merge patch into candidate while preserving default route law", "pure typed transaction primitive later"),
        registry_transaction_phase(4, "write-temp-candidate", "json.dump candidate to homeserver.json.temp", "candidate lives outside live path until validation succeeds", "Caduceus later"),
        registry_transaction_phase(5, "validate-candidate", "validate_config_with_factory_fallback(temp_config) rejects factory fallback output", "candidate must pass syntax and factory fallback validation before promotion", "Caduceus later"),
        registry_transaction_phase(6, "atomic-promote", "shutil.move(temp_config, homeserver_config_path)", "promote only the validated candidate into the live registry path", "Caduceus later"),
        registry_transaction_phase(7, "restore-owner-mode", "chown www-data:www-data and chmod 664", "permission restoration is part of the transaction receipt, not an afterthought", "Caduceus only"),
    ]
}

fn registry_transaction_phase(
    sequence: u64,
    id: &str,
    source_law: &str,
    coronatio_contract: &str,
    mutation_authority: &str,
) -> RegistryTransactionPhase {
    RegistryTransactionPhase {
        sequence,
        id: id.to_string(),
        source_law: source_law.to_string(),
        coronatio_contract: coronatio_contract.to_string(),
        mutation_authority: mutation_authority.to_string(),
    }
}

fn startup_readback() -> StartupReadback {
    StartupReadback {
        schema: "coronatio.startup.v1".to_string(),
        phases: vec!["idle", "loading-config", "connecting-events", "app-ready", "fallback", "error"]
            .into_iter()
            .map(String::from)
            .collect(),
        current_phase: "app-ready".to_string(),
        connection_status: "snapshot-only-ready".to_string(),
        initial_tab: initial_tab(true, None, false),
        default_route_law: "forced tab wins; failed connection uses fallback; valid visible starred tab wins; else first visible tab by order; else fallback".to_string(),
        fallback_tab: "fallback".to_string(),
        tab_bar_law: "admin sessions show the tab bar; regular sessions show it when more than two tabs are visible unless forceTabBarVisibility overrides".to_string(),
    }
}

fn lane_policy_readback() -> LanePolicyReadback {
    LanePolicyReadback {
        schema: "coronatio.lane-policy.v1".to_string(),
        policies: vec![
            InstallLanePolicy { install_mode: InstallMode::DynamicCartridge, success_contract: "manifest validates, staticDir remains inside tab root, optional service health is read separately, routePrefix is /api/tabs/<id>".to_string(), failure_contract: "missing manifest/static asset/service health yields tab-local error and fallback recovery candidate, not host failure".to_string(), recovery_contract: "choose next visible accessible tab; if none, activate fallback receipt".to_string() },
            InstallLanePolicy { install_mode: InstallMode::SourceInjectionRecompile, success_contract: "trusted package injects declared source, host rebuilds, cargo fmt/test/build passes, and Cibation admits source".to_string(), failure_contract: "compile/test/admission failure rejects the package and preserves previous admitted host".to_string(), recovery_contract: "keep old host binary standing; emit source-injection failure receipt".to_string() },
            InstallLanePolicy { install_mode: InstallMode::FirstPartyNative, success_contract: "pane is compiled into Coronatio and present in native registry".to_string(), failure_contract: "native pane missing is a build/test failure, not a runtime cartridge problem".to_string(), recovery_contract: "fallback pane remains reachable while source repair proceeds through Cibation".to_string() },
        ],
    }
}

fn fallback_readback() -> FallbackReadback {
    FallbackReadback {
        schema: "coronatio.fallback.v1".to_string(),
        safe_pane: "fallback".to_string(),
        activation_reasons: vec![
            "connection_failed",
            "startup_timeout",
            "no_visible_tabs",
            "module_load_error",
            "invalid_native_pane",
            "critical_startup_error",
        ]
        .into_iter()
        .map(String::from)
        .collect(),
        recovery_sequence: vec![
            "classify reason",
            "preserve previous active tab in receipt",
            "choose starred visible tab if lawful",
            "choose first visible accessible tab",
            "activate fallback when no lawful pane exists",
            "emit recovery receipt",
        ]
        .into_iter()
        .map(String::from)
        .collect(),
        receipt_fields: vec![
            "schema",
            "reason",
            "previousTab",
            "candidateTab",
            "selectedTab",
            "connectionStatus",
            "startupPhase",
            "timestamp",
        ]
        .into_iter()
        .map(String::from)
        .collect(),
    }
}

fn normalize_tab_id(tab_id: &str) -> String {
    tab_id.strip_prefix('@').unwrap_or(tab_id).to_string()
}

fn admin_session_readback() -> AdminSessionReadback {
    AdminSessionReadback {
        schema: "coronatio.admin.session.v1".to_string(),
        pin_validation: "POST /api/validatePin compared request pin to homeserver.json global.admin.pin and returned a generated session token".to_string(),
        session_timeout_seconds: 30 * 60,
        keepalive_route: "/api/admin/session".to_string(),
        logout_route: "/api/logout".to_string(),
        token_header: "X-Admin-Token".to_string(),
        token_policy: vec![
            "tokens are generated from random bytes, timestamp, and uuid".to_string(),
            "token expiry refreshes on validation".to_string(),
            "logout invalidates the token".to_string(),
            "PIN fallback compatibility is retired behind Caduceus before privileged mutation".to_string(),
        ],
        admin_enhanced_filtering: admin_field_filters(),
        caduceus_membrane: CaduceusMembrane {
            schema: "coronatio.caduceus.membrane.v1".to_string(),
            privileged_mutations: vec![
                "pin change".to_string(),
                "service restart".to_string(),
                "premium/source installation".to_string(),
                "disk/vault/key operations".to_string(),
            ],
            coronatio_role: "session readback, capability request shaping, and non-secret UI state".to_string(),
            caduceus_role: "privileged host mutation, token mint/refresh, command execution, and mutation receipts".to_string(),
            first_missing_signal: "live Caduceus admin token minting endpoint not wired".to_string(),
        },
    }
}

fn admin_field_filters() -> Vec<AdminFieldFilter> {
    vec![
        AdminFieldFilter {
            topic: "internet_status".to_string(),
            admin_fields: vec!["publicIp", "ipDetails", "dnsServers"]
                .into_iter()
                .map(String::from)
                .collect(),
        },
        AdminFieldFilter {
            topic: "vpn_status".to_string(),
            admin_fields: vec!["connectionDetails", "credentials"]
                .into_iter()
                .map(String::from)
                .collect(),
        },
        AdminFieldFilter {
            topic: "system_stats".to_string(),
            admin_fields: vec!["processes", "users", "networkConnections"]
                .into_iter()
                .map(String::from)
                .collect(),
        },
        AdminFieldFilter {
            topic: "tailscale_status".to_string(),
            admin_fields: vec!["ip", "tailnet", "isEnabled", "loginUrl"]
                .into_iter()
                .map(String::from)
                .collect(),
        },
        AdminFieldFilter {
            topic: "services_status".to_string(),
            admin_fields: vec!["isEnabled"].into_iter().map(String::from).collect(),
        },
    ]
}

fn topic_catalog_readback() -> TopicCatalogReadback {
    TopicCatalogReadback {
        schema: "coronatio.topic-catalog.v1".to_string(),
        transport: "SSE EventSource plus POST renew; Socket.IO subscribe/unsubscribe is quarry only".to_string(),
        stream_policy: "open a pane stream only while the pane is active and document is visible; core topics stay independent of active pane".to_string(),
        renew_policy: "client renews before lease expiry; expired streams produce an expired event and close in the live implementation".to_string(),
        core_topics: core_topic_contracts(),
        admin_topics: admin_topic_contracts(),
        tab_topics: vec![
            TabTopicContract {
                pane_id: "stats".to_string(),
                topics: vec!["stats.system".to_string()],
                event_route: "/api/stats/events".to_string(),
                renew_route: "/api/stats/events/renew".to_string(),
                lifecycle: "active pane + visible document".to_string(),
            },
            TabTopicContract { pane_id: "upload".to_string(), topics: vec![], event_route: "snapshot-only".to_string(), renew_route: "snapshot-only".to_string(), lifecycle: "no live stream yet".to_string() },
            TabTopicContract { pane_id: "portals".to_string(), topics: vec![], event_route: "snapshot-only".to_string(), renew_route: "snapshot-only".to_string(), lifecycle: "no live stream yet".to_string() },
        ],
    }
}

fn core_topic_contracts() -> Vec<TopicContract> {
    vec![
        topic_contract(
            "internet.status",
            "core",
            10,
            false,
            vec!["publicIp", "ipDetails", "dnsServers"],
            "internet status and public ingress posture",
            "status/public IP/DNS changes",
        ),
        topic_contract(
            "tailscale.status",
            "core",
            10,
            false,
            vec!["ip", "tailnet", "isEnabled", "loginUrl"],
            "tailscale status and admin login hints",
            "status/interface/admin field changes",
        ),
        topic_contract(
            "vpn.status",
            "core",
            10,
            false,
            vec!["connectionDetails", "credentials"],
            "VPN and transmission status",
            "vpnStatus/transmissionStatus/isEnabled changes",
        ),
        topic_contract(
            "services.status",
            "core",
            10,
            false,
            vec!["isEnabled"],
            "service health posture",
            "service status or enabled-state changes",
        ),
        topic_contract(
            "power.status",
            "core",
            1,
            false,
            vec![],
            "power sample",
            "always broadcast realtime power samples",
        ),
    ]
}

fn admin_topic_contracts() -> Vec<TopicContract> {
    vec![
        topic_contract(
            "admin.disk.info",
            "admin",
            30,
            true,
            vec![],
            "disk, encryption, NAS compatibility, and mount posture",
            "device/error/encryption/mount/filesystem/periodic changes",
        ),
        topic_contract(
            "admin.system",
            "admin",
            2,
            true,
            vec![],
            "admin system details",
            "admin-only system stats pulse",
        ),
        topic_contract(
            "hard-drive-test.status",
            "admin",
            5,
            true,
            vec![],
            "hard-drive-test state",
            "test status changes",
        ),
        topic_contract(
            "sync.status",
            "admin",
            2,
            true,
            vec![],
            "sync job status",
            "sync status changes",
        ),
    ]
}

fn topic_contract(
    id: &str,
    scope: &str,
    cadence_seconds: u64,
    admin_only: bool,
    admin_fields: Vec<&str>,
    payload_schema: &str,
    changed_rule: &str,
) -> TopicContract {
    TopicContract {
        id: id.to_string(),
        scope: scope.to_string(),
        cadence_seconds,
        admin_only,
        admin_fields: admin_fields.into_iter().map(String::from).collect(),
        payload_schema: payload_schema.to_string(),
        changed_rule: changed_rule.to_string(),
    }
}

fn stats_topic_contract() -> TopicContract {
    topic_contract(
        "stats.system",
        "tab:stats",
        1,
        false,
        vec!["processes", "users", "networkConnections"],
        "system_stats payload: load, cpu, memory, disk, network, process/user/admin fields",
        "always pulse realtime system stats; admin fields filtered unless session has admin capability",
    )
}

fn monitor_pulse_readback() -> MonitorPulseReadback {
    MonitorPulseReadback {
        schema: "coronatio.monitor-pulse.v1".to_string(),
        topic: stats_topic_contract(),
        snapshot_route: "/api/stats".to_string(),
        event_route: "/api/stats/events".to_string(),
        renew_route: "/api/stats/events/renew".to_string(),
        first_event: stats_event_payload(),
        proof_policy: vec![
            "initial subscriber receives first state".to_string(),
            "meaningful-change predicate decides later pulses".to_string(),
            "admin fields are filtered for non-admin sessions".to_string(),
            "SSE heartbeat/expiry replaces Socket.IO subscription diffing".to_string(),
        ],
    }
}

fn stats_event_payload() -> StatsEventPayload {
    StatsEventPayload {
        schema: "coronatio.stats.event.v1".to_string(),
        topic: "stats.system".to_string(),
        event_id: "stats-system-bootstrap-1".to_string(),
        event: "snapshot".to_string(),
        lease_seconds: 30,
        payload_state: "placeholder-unavailable".to_string(),
        first_missing_signal: "stats collectors not wired".to_string(),
    }
}

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
    StatsSnapshot {
        schema: "coronatio.stats.snapshot.v1".to_string(),
        pane_id: "stats".to_string(),
        product: "Coronatio".to_string(),
        transport: StatsTransport {
            snapshot_route: "/api/stats".to_string(),
            event_route: "/api/stats/events".to_string(),
            renew_route: "/api/stats/events/renew".to_string(),
            stream_status: "planned".to_string(),
            stream_reason: "stats SSE lease route is the next Coronatio tranche; snapshot is the current authority".to_string(),
        },
        telemetry: StatsTelemetry {
            load1: None,
            cpu_temperature_celsius: None,
            service_health: None,
            storage_posture: None,
            first_missing_signal: "stats collectors not wired".to_string(),
        },
        next_routes: StatsNextRoutes {
            snapshot: "/api/stats".to_string(),
            events: "/api/stats/events".to_string(),
            renew: "/api/stats/events/renew".to_string(),
        },
    }
}

fn render_flask_react_tabbar_quarry() -> String {
    let starred_tab = registry_readback().starred_tab;
    native_crown_panes()
        .into_iter()
        .map(|pane| {
            let is_starred = pane.id == starred_tab;
            let active = pane.id == "admin";
            let visibility_button = if pane.admin_only {
                r##"<div class="tab-visibility-column" aria-hidden="true"></div>"##.to_string()
            } else {
                format!(
                    r##"<div class="tab-visibility-column"><button type="button" class="visibility-toggle" data-tab-visibility-toggle="{id}" data-visible="true" aria-label="Hide {title} tab" title="Hide {title} tab"><span class="eye-icon" aria-hidden="true">👁</span></button></div>"##,
                    id = pane.id,
                    title = pane.title
                )
            };
            let star_button = if pane.admin_only {
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
            format!(
                r##"<div class="tab {active_class}" role="tab" tabindex="0" aria-controls="pane-{id}" aria-selected="{selected}" data-pane="{id}" data-tab-id="{id}" data-visibility="visible" data-admin-only="{admin_only}">{visibility_button}<span class="tab-name">{title}</span>{star_button}</div>"##,
                id = pane.id,
                title = pane.title,
                admin_only = pane.admin_only,
                active_class = if active { "active" } else { "" },
                selected = active,
                visibility_button = visibility_button,
                star_button = star_button
            )
        })
        .collect::<Vec<_>>()
        .join("")
}
