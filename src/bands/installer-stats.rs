fn boundary_readback() -> BoundaryReadback {
    BoundaryReadback {
        schema: "coronatio.route-boundary.v1".to_string(),
        api_unknown_path_policy: "legacy HomeServer /api/* paths proxy to the Flask/React Unix-socket authority so the served UX is identical; Coronatio-native contract routes stay exact under /api/coronatio/* and named /api routes".to_string(),
        static_shell_policy: "non-API unknown GET paths return the exact Flask/React HomeServer shell for client-side routing".to_string(),
        cartridge_static_policy: "/tabs/<tab-id>/... is served from the configured tab root through safe tab ids and manifest validation; legacy /assets/* is served from the exact HomeServer build asset root".to_string(),
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

