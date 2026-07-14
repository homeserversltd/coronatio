    #[test]
    fn admin_pane_matches_original_flask_react_div_skeleton() {
        let shell = render_crown_shell();
        let admin_start = shell.find("class=\"admin-tablet\"").unwrap();
        let admin_end = shell.find("id=\"pane-stats\"").unwrap();
        let admin = &shell[admin_start..admin_end];

        for class in [
            "admin-tablet", "system-controls-container", "system-controls",
            "system-controls-btn", "system-service-controls", "ssh-controls",
            "ssh-control", "ssh-status", "ssh-toggle", "samba-control",
            "samba-status", "samba-toggle", "key-manager", "key-manager-content",
            "key-manager-left", "security-status", "status-item", "status-details",
            "action-button info-button", "key-manager-right", "key-actions",
            "disk-manager", "disk-manager-container", "disk-column", "disk-list",
            "disk-item", "disk-actions",
        ] {
            assert!(admin.contains(&format!("class=\"{class}")), "missing og admin class: {class}");
        }
        assert_eq!(admin.matches("class=\"system-controls-btn\"").count(), 8);
        assert_eq!(admin.matches("class=\"disk-column\"").count(), 2);
        assert_eq!(admin.matches("class=\"action-button ").count(), 16);
        for label in [
            "Hard Drive Test", "Rotate Capability Key", "Restart Website", "Install Certificate",
            "View Full Guide &amp; Critical Warnings", "+ Create New Key",
            "⟳ Update Key on Drive", "🔒 Admin Password", "Format", "Encrypt",
            "Assign as primary NAS", "Assign as NAS Backup", "Unassign drive",
            "Import to NAS", "Setup NAS", "Unlock", "Mount", "Unmount",
            "Sync Now", "Auto Sync",
        ] {
            assert!(admin.contains(label), "missing og admin label: {label}");
        }
        assert!(admin.contains("/mnt/nas"));
        assert!(admin.contains("/mnt/nas_backup"));
        assert!(admin.find("class=\"system-controls\"").unwrap()
            < admin.find("class=\"system-service-controls\"").unwrap());
        for label in ["SSH Password Authentication", "SSH Service", "Samba File Sharing"] {
            assert!(admin.contains(&format!("<h3>{label}</h3>")), "missing static service heading: {label}");
        }
        assert_eq!(admin.matches("class=\"toggle-switch\"").count(), 3);
        assert_eq!(admin.matches("class=\"toggle-slider\"").count(), 3);
        assert!(admin.find("class=\"system-service-controls\"").unwrap()
            < admin.find("<div class=\"update-status-container\" data-admin-action-result").unwrap());
        assert!(!admin.contains("__ADMIN_SSH_PASSWORD_CARD__"));
        assert!(!admin.contains("admin-modal-shelf"));
        assert!(!admin.contains("data-admin-quarry"));
        assert!(!admin.contains("data-stub-action"));
        assert!(!admin.contains("admin-quarry-note"));
        assert!(!admin.contains(">Ready</div>"));
        assert!(!admin.contains("WebSocket Subscriptions"));
        assert!(!admin.contains("debug-subscriptions"));
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
        assert!(body.contains("firmware catalog"));
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
    async fn caduceus_update_now_refuses_guest_instead_of_faking_dispatch_success() {
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
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let body = String::from_utf8(bytes.to_vec()).unwrap();
        assert!(body.contains("admin-session-required"));
        assert!(!body.contains("\"ok\":true"));
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
                .header("X-Admin-Token", authorize_test_admin_token())
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let snapshot: SystemStatsAdminProjection = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(snapshot.schema, "coronatio.stats.snapshot.v1");
        assert_eq!(snapshot.topic, "system.stats");
        assert_eq!(snapshot.pane_id, "stats");
        assert_eq!(snapshot.product, "Coronatio");
        assert_eq!(snapshot.transport.snapshot_route, "/api/stats");
        assert_eq!(snapshot.transport.event_route, "/api/stats/pulse");
        assert_eq!(snapshot.transport.renew_route, "/api/stats/pulse/renew");
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
            r#"data-stat-element-id="network-chart""#,
            r#"data-stat-element-id="io-section""#,
            r#"data-stat-element-id="memory-usage""#,
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
            "const name = diskDisplayName(device), readName = `read-${name}`, writeName = `write-${name}`",
            "checked.has(readName)",
            "checked.has(writeName)",
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
            r#"class="drive-checkbox"><input type="checkbox" name="${escapeHtml(readName)}""#,
            r#"class="drive-checkbox"><input type="checkbox" name="${escapeHtml(writeName)}""#,
            "checked = new Map",
            "checked.has(readName)",
            "checked.has(writeName)",
            "function meaningfulInterface(iface)",
            "name === 'docker0'",
            "name.startsWith('br-')",
            "if (name.startsWith('wl')) return 'Wi-Fi';",
            "if (name.startsWith('en')) return 'Ethernet';",
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
        let shell = [
            "src/bands/shell/document-1.rs",
            "src/bands/shell/document-2.rs",
            "src/bands/shell/document-3.rs",
            "src/bands/shell/ux/shell/base-and-chrome.css",
        ]
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
            "--secondary: var(--theme-secondary)",
            "--primaryHover: var(--theme-primaryHover)",
            "--status-up: var(--theme-statusUp)",
            "--theme-gradient-accent",
            "--theme-highlight-strong",
            "--theme-role-primary",
            "--theme-component-button-container",
        ] {
            assert!(shell.contains(marker), "expanded theme marker missing: {marker}");
        }
    }


    #[test]
    fn theme_projection_hoists_derived_author_face_values_and_live_shell_paint() {
        let projector = std::fs::read_to_string("src/bands/shell/document-2.rs").unwrap();
        let chrome = std::fs::read_to_string("src/bands/shell/ux/shell/base-and-chrome.css").unwrap();
        let root = chrome.split("* { box-sizing").next().unwrap();

        for marker in [
            "hexToRgb(theme.primary)",
            "hexToRgb(theme.background)",
            "--theme-accent-soft: color-mix",
            "--theme-primary-rgb: ",
            "--theme-background-rgb: ",
        ] {
            assert!(projector.contains(marker), "derived theme projection missing: {marker}");
        }
        assert!(!root.contains('#'), "static :root must not bake catalog paint");
        for forbidden in ["#FFC107", "#cdefff", "--primary-hover"] {
            assert!(!chrome.contains(forbidden), "stale shell paint survived: {forbidden}");
        }
        for binding in [
            ".star-button.fas { color: var(--theme-accent-warm)",
            "background: var(--theme-surface-1); border: 1px solid var(--theme-outline-variant)",
            "color: var(--theme-accent-cool)",
            ".indicator:hover { background: var(--theme-highlight-subtle)",
        ] {
            assert!(chrome.contains(binding), "live shell binding missing: {binding}");
        }
    }

    #[test]
    fn public_theme_guide_maps_catalog_to_contributor_workflow() {
        let guide = std::fs::read_to_string("docs/development/theme-tokens.md").unwrap();
        for marker in [
            "src/bands/theme/catalog.json",
            "--theme-<key>",
            "global.theme.name",
            "cargo test theme_net",
            "cargo fmt --check",
        ] {
            assert!(guide.contains(marker), "public theme guide missing {marker}");
        }
        assert!(!std::path::Path::new("docs/uxThemeSystem").exists());
    }

    #[test]
    fn theme_catalog_is_one_embedded_firmware_source_and_shell_materials_are_live() {
        let routes = std::fs::read_to_string("src/bands/routes.rs").unwrap();
        let catalog = std::fs::read_to_string("src/bands/theme/catalog.json").unwrap();
        let chrome = std::fs::read_to_string("src/bands/shell/ux/shell/base-and-chrome.css").unwrap();
        assert!(routes.contains("include_str!(\"theme/catalog.json\")"));
        assert!(routes.contains("serde_json::from_str(FIRMWARE_THEME_CATALOG)"));
        assert!(catalog.contains("coronatio.theme-catalog.v1"));
        assert!(catalog.contains("gradient-primary"));
        assert!(catalog.contains("elevation-3"));
        assert!(catalog.contains("flag-gradients"));
        assert!(!std::path::Path::new("static/themes/theme.json").exists());
        assert!(!routes.contains("insert_mature_theme_tokens"));
        assert!(!routes.contains("insert_legacy_alias_tokens"));
        assert!(!routes.contains("static/themes/theme.json"));
        assert!(!routes.contains("CORONATIO_THEME_JSON"));
        assert!(!std::fs::read_to_string("src/bands/shell/document-2.rs").unwrap().contains("aliasMap"));
        for selector_binding in [
            ".top-bar {",
            "background: var(--theme-gradient-primary)",
            ".tab-bar {",
            "background: var(--theme-gradient-surface)",
            ".card {",
            "box-shadow: var(--theme-elevation-1)",
            ".modal {",
            "box-shadow: var(--theme-elevation-3)",
            ".header-control, .admin-button, .theme-button, .change-admin-pin-button",
            "background: var(--theme-gradient-accent)",
        ] {
            assert!(chrome.contains(selector_binding), "missing live Theme Net material binding: {selector_binding}");
        }
    }

    #[test]
    fn docs_and_shell_do_not_advertise_sidecar_config_authority() {
        let shell = ["src/bands/shell.rs", "src/bands/shell/document-1.rs", "src/bands/shell/document-2.rs", "src/bands/shell/document-3.rs", "src/bands/shell/document-4.rs", "src/bands/shell/render.rs", "src/bands/shell/runtime.rs"]
            .iter()
            .map(|path| std::fs::read_to_string(path).unwrap())
            .collect::<Vec<_>>()
            .join("\n");
        let readme = std::fs::read_to_string("README.md").unwrap();
        let architecture = std::fs::read_to_string("docs/architecture.md").unwrap();
        let bands = std::fs::read_to_string("src/bands/README.md").unwrap();
        let theme_doc = std::fs::read_to_string("static/themes/README.md").unwrap();
        let favorites_doc = std::fs::read_to_string("static/favorites/README.md").unwrap();
        for (name, text) in [
            ("shell", shell),
            ("readme", readme),
            ("architecture", architecture),
            ("bands", bands),
            ("theme_doc", theme_doc),
            ("favorites_doc", favorites_doc),
        ] {
            assert!(text.contains("homeserver.json"), "{name} must name homeserver.json authority");
            if name != "readme" && name != "architecture" {
                assert!(text.contains("one-to-one port"), "{name} must name the one-to-one port doctrine");
                assert!(
                    text.contains("before any Coronatio-local fallback")
                        || text.contains("before any Coronatio local fallback")
                        || text.contains("before any Coronatio-local fallback or firmware default"),
                    "{name} must name homeserver.json before local fallback authority"
                );
            }
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
        assert!(contracts.contains("FLASK_HOMESERVER_JSON"));
        assert!(contracts.contains("FACTORY_HOMESERVER_JSON"));
        assert!(contracts.contains("/etc/homeserver/config.json"));
        assert!(contracts.contains("/etc/homeserver.json"));
        assert!(contracts.contains("/var/www/homeserver/src/config/homeserver.json"));
        assert!(contracts.contains("/etc/homeserver.factory"));
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
        let expected_starred = load_homeserver_json().await.unwrap().1["tabs"]["starred"].as_str().unwrap().to_string();
        assert!(body.contains(&format!("\"starredTab\":\"{expected_starred}\"")), "{body}");
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
        assert!(body.contains(&format!("\"starred_tab\":\"{expected_starred}\"")), "{body}");
        let response = app.oneshot(Request::builder().uri("/").body(Body::empty()).unwrap()).await.unwrap();
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let shell = String::from_utf8(bytes.to_vec()).unwrap();
        assert!(shell.contains("const tabState = Object.assign({ starredTab: 'stats'"));
        assert!(shell.contains("fetch('/api/favorites')"));
        assert!(shell.contains("fetch('/api/set_starred_tab'"));
        assert_eq!(shell.matches("class=\"star-button fas fa-star\"").count(), 1);
        assert!(shell.contains("class=\"star-button far fa-star\""));
        assert!(!shell.contains("<span aria-hidden=\"true\">★</span>"));
    }

