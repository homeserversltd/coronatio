    fn maximal_stats_snapshot_fixture() -> StatsSnapshot {
        StatsSnapshot {
            schema: "schema-marker-system-stats".to_string(),
            pane_id: "pane-marker-stats".to_string(),
            product: "product-marker-coronatio".to_string(),
            doctrine: StatsViewportDoctrine {
                quarry_sources: vec!["DENY-quarry-source-marker".to_string()],
                preserved_sections: vec!["DENY-preserved-section-marker".to_string()],
                refresh_seconds: 99,
                authority: "DENY-authority-marker".to_string(),
            },
            transport: StatsTransport {
                snapshot_route: "/api/stats".to_string(),
                event_route: "/api/stats/pulse".to_string(),
                renew_route: "/api/stats/pulse/renew".to_string(),
                stream_status: "available".to_string(),
                stream_reason: "DENY-implementation-stream-reason".to_string(),
            },
            resources: StatsResources {
                load: StatsLoad { one: Some(1.11), five: Some(5.55), fifteen: Some(15.15), cpu_temperature_celsius: Some(42.5) },
                memory: StatsMemory { total_bytes: Some(1000), used_bytes: Some(400), free_bytes: Some(600), percent: Some(40) },
                swap: StatsMemory { total_bytes: Some(2000), used_bytes: Some(500), free_bytes: Some(1500), percent: Some(25) },
            },
            storage: vec![
                StatsDrive { name: "DENY-/dev/mapper/raw-root".to_string(), mount: "DENY-/raw/mount".to_string(), total_bytes: Some(10_000), used_bytes: Some(4_000), free_bytes: Some(6_000), usage_percent: Some(40), source: "DENY-df-source".to_string() },
                StatsDrive { name: "DENY-/dev/raw-vault".to_string(), mount: "/vault".to_string(), total_bytes: Some(20_000), used_bytes: Some(5_000), free_bytes: Some(15_000), usage_percent: Some(25), source: "DENY-vault-source".to_string() },
            ],
            network: StatsNetwork {
                interfaces: vec![
                    StatsNetworkInterface { name: "DENY-tailscale0".to_string(), status: "DENY-interface-status".to_string(), rx_bytes: 123, tx_bytes: 456 },
                    StatsNetworkInterface { name: "DENY-wan0".to_string(), status: "DENY-interface-status-2".to_string(), rx_bytes: 1000, tx_bytes: 2000 },
                ],
                connections: StatsConnectionCounts { established: 7, listening: 8, total: 15 },
            },
            io: StatsIo { devices: vec![StatsIoDevice { device: "DENY-nvme0n1".to_string(), mount: "DENY-/io/mount".to_string(), read_bytes: 333, write_bytes: 444 }] },
            leases: vec![StatsKeaLease { hostname: "DENY-lease-hostname".to_string(), ip: "DENY-192.0.2.10".to_string(), mac: "DENY-aa:bb:cc:dd:ee:ff".to_string(), note: "DENY-lease-note".to_string() }],
            kea_leases: StatsKeaLeases { status: "DENY-roster-status".to_string(), entries: Vec::new() },
            processes: vec![StatsProcess { name: "DENY-process-name".to_string(), cpu_percent: 88.8, memory_bytes: 9999, process_count: 3 }],
            services: vec![StatsService { name: "DENY-service-name".to_string(), status: "DENY-service-status".to_string(), details: "DENY-service-details".to_string(), route: "DENY-service-route".to_string() }],
            telemetry: StatsTelemetry {
                load1: Some(1.11),
                cpu_temperature_celsius: Some(42.5),
                service_health: Some("DENY-service-health".to_string()),
                storage_posture: Some("storage-posture-allowed".to_string()),
                first_missing_signal: "DENY-implementation-language-firstMissingSignal".to_string(),
            },
            next_routes: StatsNextRoutes {
                snapshot: "/api/stats".to_string(),
                events: "/api/stats/pulse".to_string(),
                renew: "/api/stats/pulse/renew".to_string(),
            },
        }
    }

    fn json_field_census(value: &serde_json::Value) -> Vec<String> {
        fn walk(prefix: String, value: &serde_json::Value, out: &mut Vec<String>) {
            match value {
                serde_json::Value::Object(map) => {
                    for (key, child) in map {
                        let next = if prefix.is_empty() { key.clone() } else { format!("{prefix}.{key}") };
                        out.push(next.clone());
                        walk(next, child, out);
                    }
                }
                serde_json::Value::Array(items) => {
                    for child in items {
                        walk(format!("{prefix}[]"), child, out);
                    }
                }
                _ => {}
            }
        }
        let mut out = Vec::new();
        walk(String::new(), value, &mut out);
        out.sort();
        out.dedup();
        out
    }

    fn guest_stats_all_elements_hidden_facts() -> IrisFacts {
        IrisFacts {
            tabs: vec![IrisTabFact {
                id: "stats".to_string(),
                order: 0,
                admin_only: false,
                is_enabled: true,
                visibility_tab: Some(true),
                elements: [
                    "cpu-chart",
                    "network-chart",
                    "io-section",
                    "memory-usage",
                    "disk-usage",
                    "kea-leases",
                    "process-usage",
                ]
                .into_iter()
                .map(|id| IrisElementFact { id: id.to_string(), visibility: Some(false) })
                .collect(),
            }],
            starred: "stats".to_string(),
        }
    }

    fn guest_stats_all_elements_visible_facts() -> IrisFacts {
        IrisFacts {
            tabs: vec![IrisTabFact {
                id: "stats".to_string(), order: 0, admin_only: false, is_enabled: true,
                visibility_tab: Some(true),
                elements: ["cpu-chart", "network-chart", "io-section", "memory-usage", "disk-usage", "kea-leases", "process-usage"]
                    .into_iter().map(|id| IrisElementFact { id: id.to_string(), visibility: Some(true) }).collect(),
            }],
            starred: "stats".to_string(),
        }
    }

    #[test]
    fn stats_projection_wall_guest_all_elements_hidden_yields_only_routes() {
        let raw = maximal_stats_snapshot_fixture();
        let facts = guest_stats_all_elements_hidden_facts();
        let guest = project_system_stats_guest(&raw, &facts);
        let value = serde_json::to_value(&guest).unwrap();
        assert_eq!(json_field_census(&value), vec![
            "nextRoutes", "nextRoutes.events", "nextRoutes.renew", "nextRoutes.snapshot", "schema", "topic",
        ].into_iter().map(String::from).collect::<Vec<_>>());
        assert!(guest.resources.is_none()); assert!(guest.storage.is_none()); assert!(guest.network.is_none());
        assert!(guest.io.is_none()); assert!(guest.processes.is_none()); assert!(guest.telemetry.is_none());
        assert!(guest.kea_leases.is_none());
    }

    #[test]
    fn stats_projection_wall_guest_all_elements_visible_yields_current_allowlist() {
        let raw = maximal_stats_snapshot_fixture();
        let facts = guest_stats_all_elements_visible_facts();
        let guest = project_system_stats_guest(&raw, &facts);
        assert!(guest.resources.is_some()); assert!(guest.storage.is_some()); assert!(guest.network.is_some());
        assert!(guest.io.is_some()); assert!(guest.processes.is_some()); assert!(guest.telemetry.is_some());
        assert!(guest.kea_leases.is_some());
        let value = serde_json::to_value(&guest).unwrap();
        assert_eq!(json_field_census(&value), vec![
            "io", "io.devices", "io.devices[].device", "io.devices[].mount", "io.devices[].readBytes", "io.devices[].writeBytes",
            "keaLeases", "keaLeases.entries", "keaLeases.status", "network", "network.connections", "network.connections.established",
            "network.connections.listening", "network.connections.total", "network.interfaces", "network.interfaces[].name", "network.interfaces[].rxBytes",
            "network.interfaces[].status", "network.interfaces[].txBytes", "nextRoutes", "nextRoutes.events", "nextRoutes.renew", "nextRoutes.snapshot",
            "processes", "processes[].cpuPercent", "processes[].memoryBytes", "processes[].name", "processes[].processCount", "resources", "resources.load",
            "resources.load.cpuTemperatureCelsius", "resources.load.fifteen", "resources.load.five", "resources.load.one", "resources.memory", "resources.memory.freeBytes",
            "resources.memory.percent", "resources.memory.totalBytes", "resources.memory.usedBytes", "resources.swap", "resources.swap.freeBytes", "resources.swap.percent",
            "resources.swap.totalBytes", "resources.swap.usedBytes", "schema", "storage", "storage[].freeBytes", "storage[].mount", "storage[].name", "storage[].source",
            "storage[].totalBytes", "storage[].usagePercent", "storage[].usedBytes", "telemetry", "telemetry.cpuTemperatureCelsius", "telemetry.firstMissingSignal",
            "telemetry.load1", "telemetry.serviceHealth", "telemetry.storagePosture", "topic",
        ].into_iter().map(String::from).collect::<Vec<_>>());
        let body = serde_json::to_string(&guest).unwrap();
        for marker in ["DENY-quarry-source-marker", "DENY-preserved-section-marker", "DENY-authority-marker", "DENY-implementation-stream-reason", "DENY-lease-hostname", "DENY-192.0.2.10", "DENY-aa:bb:cc:dd:ee:ff", "DENY-lease-note", "DENY-service-name", "DENY-service-status", "DENY-service-details", "DENY-service-route", "pane-marker-stats", "product-marker-coronatio"] {
            assert!(!body.contains(marker), "guest projection leaked denied marker {marker}: {body}");
        }
    }

    #[test]
    fn stats_projection_wall_admin_carries_full_lawful_admin_table() {
        let raw = maximal_stats_snapshot_fixture();
        let admin = project_system_stats_admin(&raw);
        let body = serde_json::to_string(&admin).unwrap();
        for marker in [
            "DENY-quarry-source-marker",
            "DENY-preserved-section-marker",
            "DENY-authority-marker",
            "DENY-implementation-stream-reason",
            "DENY-/dev/mapper/raw-root",
            "DENY-/raw/mount",
            "DENY-df-source",
            "DENY-lease-hostname",
            "DENY-192.0.2.10",
            "DENY-aa:bb:cc:dd:ee:ff",
            "DENY-lease-note",
            "DENY-roster-status",
            "DENY-tailscale0",
            "DENY-nvme0n1",
            "DENY-process-name",
            "DENY-service-name",
            "DENY-service-health",
            "DENY-implementation-language-firstMissingSignal",
        ] {
            assert!(body.contains(marker), "admin projection omitted lawful admin marker {marker}: {body}");
        }
        assert_eq!(admin.topic, "system.stats");
        assert_eq!(admin.processes.len(), 1);
        assert_eq!(admin.processes[0].name, "DENY-process-name");
    }

    #[test]
    fn stats_projection_wall_guest_projection_structurally_omits_denied_fields() {
        let raw = maximal_stats_snapshot_fixture();
        let facts = guest_stats_all_elements_visible_facts();
        let value = serde_json::to_value(project_system_stats_guest(&raw, &facts)).unwrap();
        let census = json_field_census(&value);
        for denied in ["doctrine", "transport.streamReason", "leases", "services", "paneId", "product"] {
            assert!(!census.iter().any(|field| field == denied || field.starts_with(&format!("{denied}."))), "guest type can represent denied field {denied}: {census:?}");
        }
    }

    #[tokio::test]
    async fn pulse_003d_wall_stats_route_projects_by_session_headers() {
        let router = app(AppState { tab_root: Arc::new(test_tab_root("pulse-003d-stats-route")) });
        let guest = router
            .clone()
            .oneshot(Request::builder().uri("/api/stats").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(guest.status(), StatusCode::OK);
        let guest_body = String::from_utf8(axum::body::to_bytes(guest.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        for denied in ["\"processes\"", "\"leases\"", "\"interfaces\"", "\"connections\"", "\"io\"", "\"mount\"", "\"device\"", "\"mac\"", "firstMissingSignal"] {
            assert!(!guest_body.contains(denied), "guest /api/stats leaked {denied}: {guest_body}");
        }
        assert!(guest_body.contains("\"topic\":\"system.stats\""), "{guest_body}");
        assert!(guest_body.contains("\"keaLeases\""), "{guest_body}");
        assert!(guest_body.contains("\"currentness\""), "{guest_body}");

        let admin = router
            .oneshot(successor_admin_request(Request::builder().uri("/api/stats").body(Body::empty()).unwrap()))
            .await
            .unwrap();
        assert_eq!(admin.status(), StatusCode::OK);
        let admin_body = String::from_utf8(axum::body::to_bytes(admin.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        for expected in ["\"processes\"", "\"leases\"", "\"interfaces\"", "\"io\"", "\"mount\"", "\"source\""] {
            assert!(admin_body.contains(expected), "admin /api/stats omitted {expected}: {admin_body}");
        }
    }

    #[tokio::test]
    async fn pulse_003de_wall_initial_shell_stats_lane_is_session_projected() {
        let router = app(AppState { tab_root: Arc::new(test_tab_root("pulse-003de-initial-shell-stats")) });
        let guest = router
            .clone()
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(guest.status(), StatusCode::OK);
        let guest_body = String::from_utf8(axum::body::to_bytes(guest.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        for denied in ["DENY-", "tailscale0", "wan0"] {
            assert!(!guest_body.contains(denied), "guest full-page / leaked denied stats lane marker {denied}: {guest_body}");
        }
        assert!(guest_body.contains(r#"data-stat-element-id="network-chart""#), "guest shell keeps projected aggregate network element: {guest_body}");

        let admin = router
            .oneshot(successor_admin_request(Request::builder().uri("/").body(Body::empty()).unwrap()))
            .await
            .unwrap();
        assert_eq!(admin.status(), StatusCode::OK);
        let admin_body = String::from_utf8(axum::body::to_bytes(admin.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        for expected in ["data-stat-element-id=\"process-usage\"", "data-stat-element-id=\"kea-leases\"", "data-stat-element-id=\"io-section\""] {
            assert!(admin_body.contains(expected), "admin full-page / omitted lawful admin stats element {expected}: {admin_body}");
        }
    }

    #[test]
    fn pulse_003de_wall_shell_rider_and_stats_hydration_are_projection_safe() {
        let chrome = crown_chrome_js();
        for denied in ["tailscale0", "wan0"] {
            assert!(!chrome.contains(denied), "externalized crown chrome carried raw interface marker {denied}: {chrome}");
        }
        assert!(chrome.contains("pulseStream.addEventListener('stats.tick'"));
        assert!(chrome.contains("refreshElementFragment('stats').catch(() => {})"));
        assert!(chrome.contains("fetch('/api/stats', { headers, cache: 'no-store' })"));
        let lifecycle_connect = chrome.find("if (active === 'stats') { hydrateStats(); connectPulseStream(); }").expect("stats stream must enter through viewport lifecycle admission");
        for declaration in [
            "let pulseStream = null;",
            "let pulseRenewTimer = null;",
            "let pulseStreamId = null;",
        ] {
            let declaration_offset = chrome.find(declaration).unwrap_or_else(|| panic!("missing pulse rider state declaration: {declaration}"));
            assert!(declaration_offset < lifecycle_connect, "pulse rider declaration must precede lifecycle connect: {declaration}");
        }
        assert!(!chrome.contains("\n    connectPulseStream();"));
        assert!(chrome.contains("function statsNetworkTotals(data)"));
        assert!(chrome.contains("network.receivedBytes"));
        assert!(chrome.contains("drive.productLabel || drive.name || 'Storage'"));
        assert!(chrome.contains("data.keaLeases && !data.leases"));
    }
