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

