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
            id: "portals".to_string(),
            title: "Portals".to_string(),
            role: "service ingress".to_string(),
            summary: "Admitted HOMESERVER services, local ingress, remote ingress, and service currentness.".to_string(),
            order: 10,
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
            order: 20,
            admin_only: false,
            install_mode: InstallMode::FirstPartyNative,
            route: "/#upload".to_string(),
            state_route: "/api/panes/upload".to_string(),
        },
        CrownPane {
            id: "stats".to_string(),
            title: "Stats".to_string(),
            role: "machine telemetry".to_string(),
            summary: "System load, service health, storage posture, and later SSE live readback.".to_string(),
            order: 30,
            admin_only: false,
            install_mode: InstallMode::FirstPartyNative,
            route: "/#stats".to_string(),
            state_route: "/api/stats".to_string(),
        },
        CrownPane {
            id: "backblaze".to_string(),
            title: "backBlaze".to_string(),
            role: "backup service tab".to_string(),
            summary: "Original HOMESERVER backBlaze tab placeholder until the pane body is ported.".to_string(),
            order: 40,
            admin_only: false,
            install_mode: InstallMode::FirstPartyNative,
            route: "/#backblaze".to_string(),
            state_route: "/api/panes/backblaze".to_string(),
        },
        CrownPane {
            id: "wake-on-lan".to_string(),
            title: "Wake on LAN".to_string(),
            role: "network wake tab".to_string(),
            summary: "Original HOMESERVER Wake on LAN tab placeholder until the pane body is ported.".to_string(),
            order: 50,
            admin_only: false,
            install_mode: InstallMode::FirstPartyNative,
            route: "/#wake-on-lan".to_string(),
            state_route: "/api/panes/wake-on-lan".to_string(),
        },
        CrownPane {
            id: "test".to_string(),
            title: "Test".to_string(),
            role: "stock UX laboratory".to_string(),
            summary: "Native stock component showcase and theme truth surface built from composable UX primitives.".to_string(),
            order: 60,
            admin_only: false,
            install_mode: InstallMode::FirstPartyNative,
            route: "/#test".to_string(),
            state_route: "/api/panes/test".to_string(),
        },
        CrownPane {
            id: "dhcp".to_string(),
            title: "DHCP".to_string(),
            role: "hidden original tab".to_string(),
            summary: "Original hidden HOMESERVER DHCP tab placeholder until the pane body is ported.".to_string(),
            order: 80,
            admin_only: false,
            install_mode: InstallMode::FirstPartyNative,
            route: "/#dhcp".to_string(),
            state_route: "/api/panes/dhcp".to_string(),
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
        starred_tab: "stats".to_string(),
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
        .map(|pane| {
            let mut visibility = TabVisibility::default();
            if pane.id == "dhcp" {
                visibility.tab = false;
            }
            CoronatioTabContract {
                id: pane.id.clone(),
                display_name: pane.title.clone(),
                order: pane.order,
                enabled: true,
                admin_only: pane.admin_only,
                visibility,
                install_mode: pane.install_mode,
                route: pane.route,
                state_route: pane.state_route,
            }
        })
        .collect()
}

fn visible_tab_ids(tabs: &[CoronatioTabContract], is_admin: bool) -> Vec<String> {
    let facts = iris::from_coronatio_contracts(tabs, "stats");
    iris::plan(&facts, registry_session(is_admin))
        .tabs
        .into_iter()
        .filter(|grant| grant.tab_id != "fallback")
        .map(|grant| grant.tab_id)
        .collect()
}

#[allow(dead_code)]
fn tab_accessible_in_mode(tab: &CoronatioTabContract, is_admin: bool) -> bool {
    let facts = iris::from_coronatio_contracts(&[tab.clone()], "stats");
    iris::plan(&facts, registry_session(is_admin))
        .tabs
        .iter()
        .any(|grant| grant.tab_id == tab.id)
}

fn selectable_tab_ids(tabs: &[CoronatioTabContract], is_admin: bool) -> Vec<String> {
    let facts = iris::from_coronatio_contracts(tabs, "stats");
    iris::plan(&facts, registry_session(is_admin))
        .tabs
        .into_iter()
        .filter(|grant| grant.tab_id != "fallback" && grant.state == RenderState::Visible)
        .map(|grant| grant.tab_id)
        .collect()
}

fn eligible_starred_tab_ids(tabs: &[CoronatioTabContract]) -> Vec<String> {
    let facts = iris::from_coronatio_contracts(tabs, "stats");
    iris::plan(&facts, Session::Admin)
        .tabs
        .into_iter()
        .filter(|grant| grant.star_eligible)
        .map(|grant| grant.tab_id)
        .collect()
}

fn registry_session(is_admin: bool) -> Session {
    if is_admin {
        Session::Admin
    } else {
        Session::Guest
    }
}

fn registry_validation_rules() -> Vec<ValidationRule> {
    vec![
        ValidationRule { field: "id".to_string(), rule: "lowercase ascii letters, digits, and hyphen only; @ is normalized away before lookup".to_string() },
        ValidationRule { field: "displayName/title".to_string(), rule: "human visible title is required".to_string() },
        ValidationRule { field: "visibility.tab".to_string(), rule: "regular users see only enabled visible non-admin tabs; admin sessions render enabled non-fallback tabs, including hidden regular tabs as restorable rows and admin-only tabs".to_string() },
        ValidationRule { field: "order".to_string(), rule: "non-negative sort key; ties sort by tab id for deterministic recovery".to_string() },
        ValidationRule { field: "starred".to_string(), rule: "default route pointer; invalid, disabled, hidden, admin-only, or fallback resolves to first visible non-admin tab then fallback".to_string() },
        ValidationRule { field: "installMode".to_string(), rule: "dynamic-cartridge for runtime tabs, source-injection-recompile for trusted source lanes, first-party-native only for compiled crown panes".to_string() },
    ]
}

fn initial_tab(connection_ok: bool, forced_tab: Option<&str>, is_admin: bool) -> String {
    if !connection_ok {
        return "fallback".to_string();
    }
    let contracts = native_tab_contracts();
    let selectable = selectable_tab_ids(&contracts, is_admin);
    if let Some(tab) = forced_tab {
        let normalized = normalize_tab_id(tab);
        if selectable.iter().any(|candidate| candidate == &normalized) {
            return normalized;
        }
    }
    let starred = normalize_tab_id("stats");
    let starred_candidates = eligible_starred_tab_ids(&contracts);
    if starred_candidates.iter().any(|tab| tab == &starred) {
        starred
    } else {
        selectable
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
        source_contract: "legacy-installer/utils/config_manager.py apply_config_patch, deep_merge, deep_merge_tabs, validate_config_with_factory_fallback, restore_backup, revert_config_patch".to_string(),
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
        caduceus_membrane: CaduceusMembrane {
            schema: "coronatio.caduceus.membrane.v1".to_string(),
            privileged_mutations: vec![
                "pin change".to_string(),
                "service restart".to_string(),
                "legacy-installer/source installation".to_string(),
                "disk/vault/key operations".to_string(),
            ],
            coronatio_role: "session readback, capability request shaping, and non-secret UI state".to_string(),
            caduceus_role: "privileged host mutation, token mint/refresh, command execution, and mutation receipts".to_string(),
            first_missing_signal: "live Caduceus admin token minting endpoint not wired".to_string(),
        },
    }
}

fn topic_catalog_readback() -> TopicCatalogReadback {
    TopicCatalogReadback {
        schema: "coronatio.topic-catalog.v1".to_string(),
        transport: "SSE EventSource plus POST renew; Socket.IO subscribe/unsubscribe is quarry only".to_string(),
        stream_policy: "open a pane stream only while the pane is active and document is visible; core topics stay independent of active pane".to_string(),
        renew_policy: "client renews a stream id before the 30s lease expires; expired streams emit pulse.expired and close".to_string(),
        core_topics: core_topic_contracts(),
        admin_topics: admin_topic_contracts(),
        tab_topics: vec![
            TabTopicContract {
                pane_id: "stats".to_string(),
                topics: vec!["tabs.changed".to_string()],
                event_route: "/api/stats/pulse".to_string(),
                renew_route: "/api/stats/pulse/renew".to_string(),
                lifecycle: "active pane + visible document".to_string(),
            },
            TabTopicContract { pane_id: "upload".to_string(), topics: vec![], event_route: "snapshot-only".to_string(), renew_route: "snapshot-only".to_string(), lifecycle: "no live stream yet".to_string() },
            TabTopicContract { pane_id: "portals".to_string(), topics: vec![], event_route: "snapshot-only".to_string(), renew_route: "snapshot-only".to_string(), lifecycle: "no live stream yet".to_string() },
        ],
    }
}

