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
        "tabs.changed",
        "tab:stats",
        30,
        false,
        vec![],
        "data-free poke; clients pull session-projected tab/element/star state after receipt",
        "unconditional poke; predicate law deferred to the PULSE-003 data plane",
    )
}

fn monitor_pulse_readback() -> MonitorPulseReadback {
    MonitorPulseReadback {
        schema: "coronatio.monitor-pulse.v1".to_string(),
        topic: stats_topic_contract(),
        snapshot_route: "/api/stats".to_string(),
        event_route: "/api/stats/events".to_string(),
        renew_route: "/api/stats/events/renew".to_string(),
        stream_contract: pulse_stream_contract(),
        proof_policy: vec![
            "initial subscriber receives stream identity and lease metadata only".to_string(),
            "pokes are data-free invalidations; no config/product data travels over pulse".to_string(),
            "admin-only lanes are selected at stream construction from session capability".to_string(),
            "SSE keepalive/renew/expiry replaces Socket.IO subscription diffing".to_string(),
        ],
    }
}

fn pulse_stream_contract() -> PulseStreamContract {
    PulseStreamContract {
        schema: "coronatio.pulse.stream.v1".to_string(),
        first_event: "pulse.open".to_string(),
        poke_data: "{}".to_string(),
        lease_seconds: 30,
        identity: "stream id appears only in the first frame and renew query".to_string(),
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

