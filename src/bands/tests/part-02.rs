    #[test]
    fn admin_pane_stubs_original_flask_react_admin_button_inventory() {
        let shell = render_crown_shell();
        assert!(shell.contains(r#"data-admin-quarry="flask-react-admin""#));
        assert!(shell.contains(r#"data-admin-quarry-button-total="74""#));
        assert_eq!(shell.matches("data-admin-quarry-button").count(), 75);
        assert_eq!(shell.matches("data-admin-quarry-index=").count(), 74);
        for (group, count) in [
            ("system-controls", 7),
            ("key-manager", 3),
            ("admin-password-modal", 2),
            ("create-key-modal", 2),
            ("hard-drive-test-modal", 6),
            ("log-viewer-modal", 6),
            ("password-input-modal", 3),
            ("premium-tab-modal", 16),
            ("root-ca-modal", 5),
            ("sync-schedule-modal", 2),
            ("system-action-modal", 1),
            ("update-key-modal", 2),
            ("update-manager-modal", 19),
        ] {
            assert!(shell.contains(&format!(r#"data-admin-quarry-group="{}""#, group)));
            assert!(shell.contains(&format!("{} buttons", count)));
        }
        for label in [
            "Hard Drive Test",
            "Restart Website",
            "Install Certificate",
            "+ Create New Key",
            "⟳ Update Key on Drive",
            "🔒 Admin Password",
            "Auto Sync Schedule",
            "Validate &amp; Clone",
            "Force Update",
        ] {
            assert!(shell.contains(label), "missing admin quarry button label: {}", label);
        }
        assert!(shell.contains(r#"data-admin-action-strip="single-row""#));
        assert!(shell.contains(r#"data-admin-action-strip-count="7""#));
        assert!(shell.contains("SSH Password Authentication"));
        assert!(shell.contains(r#"data-service-card="ssh-password-authentication""#));
        assert!(shell.contains("SSH Service"));
        assert!(shell.contains(r#"data-service-card="ssh-service""#));
        assert!(shell.contains("Samba File Sharing"));
        assert!(shell.contains(r#"data-service-card="samba-file-sharing""#));
        assert!(shell.contains(r#"data-state-source="/api/services/data""#));
        assert!(shell.contains("Key Management"));
        assert!(shell.contains("This is the key to your vault. When you boot your HOMESERVER and visit home.arpa, this is what unlocks your encrypted storage system - just like unlocking your smartphone. Your /vault partition contains the sensitive keys stored on the device. Unlock the vault and everything HOMESERVER specifically stores is accessible. This is the device's master key."));
        assert!(shell.contains("Available Devices"));
        assert!(shell.contains("Mount Destinations"));
        assert!(shell.contains("homeserver-primary-nas"));
        assert!(shell.contains("Primary NAS"));
        assert!(shell.contains("Mounted at:</strong> /mnt/nas"));
        assert!(shell.contains("3.7T - XFS (encrypted)"));
        assert!(shell.contains("Space:</strong> 1.7T/3.7T (45%) - 2.1T free"));
        assert!(shell.contains("Mapper: sdb1_crypt"));
        assert!(shell.contains("Unlocked <span class=\"filesystem-label\">(xfs)</span>"));
        assert!(shell.contains("Device: <span class=\"device-label\">sdb</span>"));
        assert!(shell.contains("In Use"));
        assert!(!shell.contains("<h3>Key Manager</h3>"));
        assert!(!shell.contains("<h3>Disk Manager</h3>"));
        assert!(!shell.contains("Available Drives"));
        assert!(!shell.contains("Drive Actions"));
        assert!(!shell.contains("Format Drive"));
        assert!(!shell.contains("Assign as primary NAS"));
        assert!(shell.contains(r#"data-admin-visual-port="one-to-one-best-effort""#));
        assert!(shell.contains("system-controls-btn"));
        assert!(shell.contains("key-manager-content"));
        assert!(shell.contains("disk-manager-container"));
        assert!(shell.contains("modal-window update-manager-modal"));
        assert!(shell.contains("view-tabs"));
        assert!(shell.contains("modules-table"));
        assert!(shell.contains("data-stub-action=\"true\""));
        assert!(!shell.contains("WebSocket Subscriptions"));
        assert!(!shell.contains("debug-subscriptions"));
        assert!(!shell.contains("components/DebugSubscriptions.tsx"));
        assert!(!shell.contains("subscription-debug-panel"));
        assert!(!shell.contains("Front-end stubs mirror the original Flask/React admin-page button inventory from the quarry."));
    }

    #[tokio::test]
    async fn themes_route_reads_homeserver_json_theme_selection() {
        let temp = test_tab_root("homeserver-json-theme");
        let response = app(AppState {
            tab_root: Arc::new(temp),
        })
        .oneshot(Request::builder().uri("/api/themes").body(Body::empty()).unwrap())
        .await
        .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let body = String::from_utf8(bytes.to_vec()).unwrap();
        assert!(body.contains("coronatio.theme-catalog.response.v1"));
        assert!(body.contains("homeserver.json"));
        assert!(body.contains("global.theme.name"));
        assert!(!body.contains("static/themes/theme.json"));
        assert!(!body.contains("CORONATIO_THEME_JSON"));
        assert!(body.contains("radioactive"));
        assert!(body.contains("color-primary"));
        assert!(body.contains("bg-primary"));
        assert!(body.contains("font-family"));
        assert!(body.contains("font-mono"));
        assert!(body.contains("font-size-2xl"));
        assert!(body.contains("spacing-2xl"));
        assert!(body.contains("control-height"));
        assert!(body.contains("content-padding"));
        assert!(body.contains("card-radius"));
        assert!(body.contains("primaryHover"));
        assert!(body.contains("hiddenTabBackground"));
        assert!(body.contains("#A78BFA"), "dark accent must come from literal dark.json");
        assert!(body.contains("#323840"), "dark primary must come from literal dark.json, not green");
        assert!(body.contains("#6B7280"), "dark primaryHover must come from literal dark.json");
        assert!(body.contains("#F87171"), "dark statusDown must come from literal dark.json");
        assert!(body.contains("gradient-accent"));
        assert!(body.contains("highlight-strong"));
        assert!(body.contains("role-primary"));
        assert!(body.contains("component-button-container"));
        assert!(body.contains("flag-gradients"));
        assert!(body.contains("#A0AEC0"), "light primary must come from literal light.json");
    }

    #[tokio::test]
    async fn caduceus_routes_are_exposed_by_coronatio_api_root() {
        let temp = test_tab_root("caduceus-routes");
        let response = app(AppState {
            tab_root: Arc::new(temp),
        })
        .oneshot(Request::builder().uri("/api").body(Body::empty()).unwrap())
        .await
        .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let body = String::from_utf8(bytes.to_vec()).unwrap();
        assert!(body.contains("/api/caduceus/status"));
        assert!(body.contains("/api/caduceus/update/check"));
        assert!(body.contains("/api/caduceus/update/now"));
        assert!(body.contains("/api/caduceus/receipts/latest"));
    }

    #[tokio::test]
    async fn caduceus_update_now_acknowledges_self_restart_dispatch() {
        let temp = test_tab_root("caduceus-dispatch");
        let response = app(AppState {
            tab_root: Arc::new(temp),
        })
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/caduceus/update/now")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
        assert_eq!(response.status(), StatusCode::ACCEPTED);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let body = String::from_utf8(bytes.to_vec()).unwrap();
        assert!(body.contains("coronatio.caduceus.dispatch.v1"));
        assert!(body.contains("update_now"));
        assert!(body.contains("/api/v1/update/now"));
    }

    #[tokio::test]
    async fn loads_dynamic_cartridge_manifests_without_recompile() {
        let temp = test_tab_root("dynamic-tabs");
        let tab_dir = temp.join("service-card");
        std::fs::create_dir_all(&tab_dir).unwrap();
        std::fs::write(
            tab_dir.join("tab.json"),
            r#"{
              "id":"service-card",
              "title":"Service Card",
              "order":90,
              "adminOnly":true,
              "routePrefix":"/api/tabs/service-card",
              "staticDir":"static",
              "serviceUrl":"http://127.0.0.1:9910",
              "healthRoute":"/health",
              "installMode":"dynamic-cartridge"
            }"#,
        )
        .unwrap();

        let response = app(AppState {
            tab_root: Arc::new(temp),
        })
        .oneshot(
            Request::builder()
                .uri("/api/tabs")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let list: TabList = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(list.native_panes.len(), PRIMARY_TABS.len());
        assert_eq!(list.tabs.len(), 1);
        assert_eq!(list.tabs[0].id, "service-card");
        assert_eq!(list.tabs[0].install_mode, InstallMode::DynamicCartridge);
    }

    #[tokio::test]
    async fn stats_snapshot_is_honest_first_party_readback() {
        let temp = test_tab_root("stats-snapshot");
        let response = app(AppState {
            tab_root: Arc::new(temp),
        })
        .oneshot(
            Request::builder()
                .uri("/api/stats")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let snapshot: StatsSnapshot = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(snapshot.schema, "coronatio.stats.snapshot.v1");
        assert_eq!(snapshot.pane_id, "stats");
        assert_eq!(snapshot.product, "Coronatio");
        assert_eq!(snapshot.transport.snapshot_route, "/api/stats");
        assert_eq!(snapshot.transport.event_route, "/api/stats/events");
        assert_eq!(snapshot.transport.renew_route, "/api/stats/events/renew");
        assert_eq!(snapshot.transport.stream_status, "available");
        for section in ["cpu-chart", "network", "io-section", "memory", "disk-usage", "kea-leases", "process-usage"] {
            assert!(
                snapshot.doctrine.preserved_sections.contains(&section.to_string()),
                "missing React Stats preserved section {section}"
            );
        }
        assert!(!snapshot.doctrine.preserved_sections.contains(&"services".to_string()));
        assert_eq!(snapshot.doctrine.refresh_seconds, 5);
        assert!(!snapshot.storage.is_empty());
        assert!(!snapshot.io.devices.is_empty());
        assert!(snapshot.leases.len() <= 20);
        assert!(snapshot.processes.len() <= 10);
        assert!(snapshot.services.iter().any(|service| service.name == "Coronatio"));
        assert!(snapshot.telemetry.service_health.is_some());
        assert!(snapshot.telemetry.storage_posture.is_some());
    }

    #[test]
    fn stats_native_pane_points_to_stats_snapshot_route() {
        let stats = native_crown_panes()
            .into_iter()
            .find(|pane| pane.id == "stats")
            .unwrap();
        assert_eq!(stats.state_route, "/api/stats");
    }

    #[test]
    fn stats_viewport_is_react_tablet_one_to_one_inventory() {
        let shell = render_crown_shell();
        for marker in [
            r#"class="stats-tablet""#,
            r#"data-react-quarry="StatsTablet""#,
            r#"data-identity-standard="one-to-one""#,
            r#"data-stat-element-id="cpu-chart""#,
            r#"data-stat-element-id="network""#,
            r#"data-stat-element-id="io-section""#,
            r#"data-stat-element-id="memory""#,
            r#"data-stat-element-id="disk-usage""#,
            r#"data-stat-element-id="kea-leases""#,
            r#"data-stat-element-id="process-usage""#,
            r#"class="stat-header""#,
            r#"class="stat-title""#,
            r#"class="stat-content""#,
            r#"CPU Usage &amp; Load"#,
            r#"Network Traffic (WAN)"#,
            r#"Disk I/O"#,
            r#"Memory Usage"#,
            r#"Disk Usage"#,
            r#"DHCP Leases"#,
            r#"CPU Usage by Process"#,
            r#"class="cpu-stats-container""#,
            r#"class="cpu-chart""#,
            r#"class="load-averages""#,
            r#"1 min:"#,
            r#"5 min:"#,
            r#"15 min:"#,
            r#"class="network-stats-container""#,
            r#"class="network-speed-chart""#,
            r#"class="network-interfaces-table""#,
            r#"<th>Interface</th><th>Total Received</th><th>Total Sent</th>"#,
            r#"class="disk-io-chart""#,
            r#"class="device-controls""#,
            r#"name="read-"#,
            r#"name="write-"#,
            r#"class="memory-stats""#,
            r#"class="memory-current""#,
            r#"class="memory-label">RAM"#,
            r#"class="memory-label">Swap"#,
            r#"class="disk-usage-stats""#,
            r#"class="disk-usage-item"#,
            r#"class="kea-leases-table""#,
            r#"<th>Device Note</th><th>Hostname</th><th>IP Address</th><th>MAC Address</th>"#,
            r#"class="process-usage-list""#,
            r#"class="process-bar"#,
            r#"createCPUChart"#,
            r#"createNetworkChart"#,
            r#"createIOChart"#,
            r#"data-chartjs-chart="cpu""#,
            r#"data-chartjs-chart="network""#,
            r#"data-full-width-canvas="true""#,
            r#"class="chart-container" id="cpu-chart-container""#,
            r#"class="chart-container" id="network-chart-container""#,
            r#"class="chart-container" id="disk-io-chart-container""#,
        ] {
            assert!(shell.contains(marker), "React Stats identity marker missing: {}", marker);
        }
        assert_eq!(shell.matches("class=\"stat-element\"").count(), 7, "Stats must render exactly seven React StatElement blocks");
        for extra_or_old in [
            r#"class="stats-section services""#,
            r#"aria-label="Stats stream""#,
            r#"Stream lane"#,
            r#"Read event frame"#,
            r#"Renew lease"#,
            r#"stats-readout"#,
            r#"id="cpu-gauge""#,
            r#"id="memory-chart""#,
            r#"type: 'doughnut'"#,
            r#"class="stats-section resources""#,
            r#"class="stats-section drives""#,
            r#"class="stats-section network""#,
        ] {
            assert!(!shell.contains(extra_or_old), "non-React Stats divergence survived: {}", extra_or_old);
        }
    }


    #[test]
    fn stats_charts_port_original_chartjs_dual_axes_full_width_and_tooltips() {
        let shell = render_crown_shell();
        for marker in [
            r#"<script src="/static/vendor/chart.umd.min.js" data-chart-dependency="chartjs-4.4.0""#,
            r#"<canvas id="cpuChart" class="coronatio-chart-canvas" data-full-width-canvas="true" data-chart-left-axis="percent-suffix" data-chart-right-axis="celsius-suffix""#,
            r#"<canvas id="networkChart" class="coronatio-chart-canvas" data-full-width-canvas="true" data-chart-left-axis="byte-rate-suffix" data-chart-right-axis="byte-rate-suffix" data-synchronized-axes="true""#,
            r#"<canvas id="io-chart" class="coronatio-chart-canvas" data-full-width-canvas="true""#,
            "maintainAspectRatio: false",
            "interaction: { mode: 'index', intersect: false }",
            "tooltip: chartTooltip",
            "lineDataset('CPU Usage', cpuData, '#4A5568', 'y-cpu')",
            "lineDataset('Temperature', tempData, '#90cff3', 'y-temp')",
            "lineDataset('Download Speed', downloadData, '#4A5568', 'y')",
            "lineDataset('Upload Speed', uploadData, '#90cff3', 'y-right')",
            "fill: false",
            "pointRadius: 0",
            "legend: { position: 'bottom', align: 'center'",
            "value => Number(value).toFixed(0) + '%'",
            "value => Number(value).toFixed(0) + '°C'",
            "callback: value => fmtBytes(value) + '/s'",
            "function formatChartTime(value = Date.now())",
            "const networkMax = Math.max(1, ...downloadData, ...uploadData) * 1.1",
            "'y-cpu': { type: 'linear', display: true, position: 'left'",
            "'y-temp': { type: 'linear', display: true, position: 'right'",
            "'y-right': { beginAtZero: true, suggestedMin: 0, max: networkMax, position: 'right'",
        ] {
            assert!(shell.contains(marker), "missing Chart.js parity marker: {marker}");
        }
        for drift in [
            "rgb(75, 192, 192)",
            "rgb(255, 99, 132)",
            "rgba(75, 192, 192, 0.1)",
            "rgba(255, 99, 132, 0.1)",
            "fill: true",
            "legend: { position: 'top' }",
            "title: { display: true",
            "CPU Usage (%)",
            "Temperature (°C)",
            "Speed (B/s)",
            "toLocaleTimeString()",
        ] {
            assert!(!shell.contains(drift), "Chart.js quarry drift survived: {drift}");
        }
    }

    #[test]
    fn stats_disk_io_and_interface_filters_match_original_detail_shape() {
        let shell = render_crown_shell();
        for marker in [
            r#"id="io-drive-selector" data-device-controls data-original-control="drive-checkbox""#,
            r#"class="drive-checkbox"><input type="checkbox" name="read-${name}" value="${name}" checked>Read"#,
            r#"class="drive-checkbox"><input type="checkbox" name="write-${name}" value="${name}" checked>Write"#,
            "function meaningfulInterface(iface)",
            "name === 'docker0'",
            "name.startsWith('br-')",
            "if (name === 'tailscale0') return 'Tailscale VPN';",
            "if (mount === '/mnt/nas') return 'nas';",
            "if (mount === '/mnt/nasbackup') return 'nasbackup';",
            "if ((device.device || '').includes('sda6')) return 'sda6';",
        ] {
            assert!(shell.contains(marker), "missing Stats detail parity marker: {marker}");
        }
    }

    #[test]
    fn static_root_prefers_installed_source_and_allows_env_override() {
        std::env::remove_var("CORONATIO_STATIC_ROOT");
        let root = static_root();
        assert!(
            root == PathBuf::from(INSTALLED_STATIC_ROOT) || root == PathBuf::from(DEFAULT_STATIC_ROOT),
            "unexpected static root: {}",
            root.display()
        );
        std::env::set_var("CORONATIO_STATIC_ROOT", "/tmp/coronatio-static-test");
        assert_eq!(static_root(), PathBuf::from("/tmp/coronatio-static-test"));
        std::env::remove_var("CORONATIO_STATIC_ROOT");
    }

    #[tokio::test]
    async fn chartjs_dependency_is_served_as_first_party_static_asset() {
        let temp = test_tab_root("chartjs-static");
        let router = app(AppState {
            tab_root: Arc::new(temp),
        });
        let response = router
            .oneshot(
                Request::builder()
                    .uri("/static/vendor/chart.umd.min.js")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let body = String::from_utf8(bytes.to_vec()).unwrap();
        assert!(body.contains("Chart.js"));
        assert!(body.contains("DoughnutController"));
    }



    #[test]
    fn shell_projects_expanded_theme_tokens_and_legacy_aliases() {
        let shell = ["src/bands/shell/document-1.rs", "src/bands/shell/document-2.rs", "src/bands/shell/document-3.rs"]
            .iter()
            .map(|path| std::fs::read_to_string(path).unwrap())
            .collect::<Vec<_>>()
            .join("\n");
        for marker in [
            "--theme-control-height",
            "--theme-content-padding",
            "--theme-card-radius",
            "--theme-font-mono",
            "--primary: var(--theme-primary)",
            "--theme-primary: #A0AEC0",
            "--primaryHover",
            "--hiddenTabBackground",
            "aliasMap",
            "--theme-gradient-accent",
            "--theme-highlight-strong",
            "--theme-role-primary",
            "--theme-component-button-container",
        ] {
            assert!(shell.contains(marker), "expanded theme marker missing: {marker}");
        }
    }


    #[test]
    fn ux_theme_system_docs_are_infinite_infinite_camel_case_band() {
        for path in [
            "docs/uxThemeSystem/index.json",
            "docs/uxThemeSystem/index.md",
            "docs/uxThemeSystem/observeMatureThemeSystems/index.json",
            "docs/uxThemeSystem/declareJsonTokenGrammar/index.json",
            "docs/uxThemeSystem/proveUxLibraryExpansion/index.json",
        ] {
            assert!(std::path::Path::new(path).exists(), "missing UX theme docs band path: {path}");
        }
        let index = std::fs::read_to_string("docs/uxThemeSystem/index.json").unwrap();
        assert!(index.contains("observeMatureThemeSystems"));
        assert!(index.contains("declareJsonTokenGrammar"));
        assert!(index.contains("proveUxLibraryExpansion"));
    }

    #[test]
    fn docs_and_shell_do_not_advertise_sidecar_config_authority() {
        let shell = ["src/bands/shell.rs", "src/bands/shell/document-1.rs", "src/bands/shell/document-2.rs", "src/bands/shell/document-3.rs", "src/bands/shell/document-4.rs", "src/bands/shell/render.rs", "src/bands/shell/runtime.rs"]
            .iter()
            .map(|path| std::fs::read_to_string(path).unwrap())
            .collect::<Vec<_>>()
            .join("\n");
        let readme = std::fs::read_to_string("README.md").unwrap();
        let north_star = std::fs::read_to_string("docs/coronatio-north-star-contract.md").unwrap();
        let bands = std::fs::read_to_string("src/bands/README.md").unwrap();
        let theme_doc = std::fs::read_to_string("static/themes/README.md").unwrap();
        let favorites_doc = std::fs::read_to_string("static/favorites/README.md").unwrap();
        for (name, text) in [
            ("shell", shell),
            ("readme", readme),
            ("north_star", north_star),
            ("bands", bands),
            ("theme_doc", theme_doc),
            ("favorites_doc", favorites_doc),
        ] {
            assert!(text.contains("homeserver.json"), "{name} must name homeserver.json authority");
            assert!(text.contains("one-to-one port"), "{name} must name the one-to-one port doctrine");
            assert!(
                text.contains("before any Coronatio-local fallback")
                    || text.contains("before any Coronatio local fallback")
                    || text.contains("before any Coronatio-local fallback or firmware default"),
                "{name} must name homeserver.json before local fallback authority"
            );
            assert!(!text.contains("static/themes/theme.json"), "{name} advertises obsolete theme sidecar");
            assert!(!text.contains("static/favorites/favorites.json"), "{name} advertises obsolete favorites sidecar");
            assert!(!text.contains("CORONATIO_THEME_JSON"), "{name} advertises obsolete theme env sidecar");
            assert!(!text.contains("CORONATIO_FAVORITES_JSON"), "{name} advertises obsolete favorites env sidecar");
        }
    }

    #[test]
    fn coronatio_config_authority_is_single_homeserver_json_not_sidecar_jsons() {
        let mut contracts = std::fs::read_to_string("src/bands/contracts.rs").unwrap();
        for child in std::fs::read_dir("src/bands/contracts").unwrap() {
            let path = child.unwrap().path();
            if path.extension().is_some_and(|extension| extension == "rs") {
                contracts.push_str(&std::fs::read_to_string(path).unwrap());
            }
        }
        let routes = std::fs::read_to_string("src/bands/routes.rs").unwrap();
        assert!(contracts.contains("INSTALLED_HOMESERVER_JSON"));
        assert!(contracts.contains("LEGACY_HOMESERVER_JSON"));
        assert!(routes.contains("fn homeserver_json_path()"));
        assert!(routes.contains("homeserver.json tabs.{config,visibility,starred}"));
        assert!(routes.contains("global.theme.name"));
        for obsolete in [
            "DEFAULT_THEME_JSON",
            "INSTALLED_THEME_JSON",
            "DEFAULT_FAVORITES_JSON",
            "INSTALLED_FAVORITES_JSON",
            "CORONATIO_THEME_JSON",
            "CORONATIO_FAVORITES_JSON",
            "theme_catalog_path()",
            "favorite_manifest_path()",
        ] {
            assert!(!contracts.contains(obsolete), "obsolete config authority survived in contracts: {obsolete}");
            assert!(!routes.contains(obsolete), "obsolete config authority survived in routes: {obsolete}");
        }
    }

    #[tokio::test]
    async fn favorite_manifest_drives_original_first_load_starred_tab() {
        let temp = test_tab_root("favorite-manifest");
        let app = app(AppState { tab_root: Arc::new(temp) });
        let response = app.clone().oneshot(Request::builder().uri("/api/favorites").body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body = String::from_utf8(bytes.to_vec()).unwrap();
        assert!(body.contains("coronatio.favorite-manifest.response.v1"));
        assert!(body.contains("\"starredTab\":\"stats\""));
        assert!(body.contains("homeserver.json"));
        assert!(body.contains("tabs.{config,visibility,starred}"));
        assert!(!body.contains("static/favorites"));
        assert!(!body.contains("CORONATIO_FAVORITES_JSON"));
        assert!(body.contains("get_starred_tab() or get_first_visible_tab()"));
        let response = app.clone().oneshot(Request::builder().uri("/api/get_starred_tab").body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body = String::from_utf8(bytes.to_vec()).unwrap();
        assert!(body.contains("coronatio.starred-tab.response.v1"));
        assert!(body.contains("\"starred_tab\":\"stats\""));
        let response = app.oneshot(Request::builder().uri("/").body(Body::empty()).unwrap()).await.unwrap();
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let shell = String::from_utf8(bytes.to_vec()).unwrap();
        assert!(shell.contains("const tabState = Object.assign({ starredTab: 'stats'"));
        assert!(shell.contains("fetch('/api/favorites')"));
        assert!(shell.contains("fetch('/api/set_starred_tab'"));
        assert!(shell.contains("Stats tab is starred"));
    }

