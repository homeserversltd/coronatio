    #[test]
    fn uxport_002_ux_order_wall_matches_index_children() {
        let index: serde_json::Value = serde_json::from_str(SHELL_UX_INDEX_JSON).unwrap();
        assert_eq!(index["schema"], "coronatio.shell.ux.index.v1");
        let children: Vec<&str> = index["children"]
            .as_array()
            .unwrap()
            .iter()
            .map(|child| child.as_str().unwrap())
            .collect();
        assert_eq!(children, SHELL_UX_CHILDREN);
        assert_eq!(SHELL_UX_CHILDREN.len(), SHELL_UX_CONTENTS.len());
    }

    #[test]
    fn uxport_002_ux_presence_wall_files_exist_and_are_served() {
        let shell = render_crown_shell();
        for (child, content) in SHELL_UX_CHILDREN.iter().zip(SHELL_UX_CONTENTS.iter()) {
            let path = std::path::Path::new("src/bands/shell/ux").join(child);
            assert!(path.exists(), "ux child missing on disk: {}", path.display());
            assert!(!content.is_empty(), "ux child is empty: {child}");
            assert!(shell.contains(content), "served shell missing ux child content: {child}");
        }
        assert!(std::path::Path::new("src/bands/shell/ux/index.json").exists());
        assert!(std::path::Path::new("src/bands/shell/ux/README.md").exists());
    }

    #[test]
    fn uxport_002_ux_doctrine_wall_names_three_kinds_and_og_names() {
        for marker in [
            "`library/` holds og `styles/common/ui` mirrors, one file per og source",
            "Og underscore names are kept one-to-one for traceability",
            "`packs/` holds per-pane absorbed domain packs",
            "og inline-style fold",
            "Og truth = tab CSS + TSX inline styles + element defaults",
            "`shell/` holds crown chrome/base CSS",
            "Adding a crown pane",
        ] {
            assert!(SHELL_UX_README.contains(marker), "ux README missing doctrine marker: {marker}");
        }
    }

    #[test]
    fn uxport_003_stats_source_library_and_holding_pen_walls() {
        let html = render_crown_shell();
        let map = std::fs::read_to_string("docs/great-porting-map.md").unwrap();
        for citation in [
            "src/tablets/stats/index.tsx",
            "components/{MemoryRadialBar,ProcessUsageList,DiskUsageChart,DiskIoChart,NetworkSpeedChart,KeaLeasesTable,StatChart,CpuStatChart}.tsx",
            "stats.css",
            "src/styles/common/ui/{_progress-bar,_table,_checkbox,_visibility-toggle,_editable-field}.css",
        ] {
            assert!(html.contains(citation) || map.contains(citation), "missing stats og citation {citation}");
        }
        assert!(html.contains("UXPORT-003 LIBRARY band: og src/tablets/stats stats domain pack"));
        for receipt in [
            "stat-card=ABSORB",
            "chart-hosts=ABSORB",
            "load-averages=ABSORB",
            "memory-bars=FAITHFUL-after-diff",
            "process-bars=FAITHFUL-after-diff",
            "disk-usage-bars=FAITHFUL-after-diff",
            "network-interfaces-table=FAITHFUL-after-diff",
            "kea-leases-table=FAITHFUL-after-diff",
            "disk-io-checkboxes=ABSORB",
            "visibility-toggle=DEFERRED",
        ] {
            assert!(html.contains(receipt) || map.contains(receipt), "missing stats declaration-diff receipt {receipt}");
        }
        let holding_pen = std::fs::read_to_string("src/bands/shell/ux/shell/document-2-css.css").unwrap();
        for drained in [".stats-tablet", ".stat-element", ".memory-bar", ".network-interfaces-table", ".kea-leases-table", ".process-bar", ".disk-usage-bar", ".device-controls", ".load-averages", ".coronatio-chart-canvas"] {
            assert!(!holding_pen.contains(drained), "stats selector remained in holding pen: {drained}");
            assert!(html.contains(drained), "drained stats selector not served from stats pack: {drained}");
        }
    }

    #[test]
    fn uxport_003_stats_markup_preserves_og_classes_and_defers_tabbar() {
        let html = render_crown_shell();
        let stats_start = html.find(r#"class="stats-tablet" data-stats-viewport"#).unwrap();
        let stats_end = html.find(r#"id="pane-portals""#).unwrap();
        let stats = &html[stats_start..stats_end];
        for required in [
            r#"class="stat-element""#,
            r#"class="stat-header""#,
            r#"class="stat-title""#,
            r#"class="stat-content""#,
            r#"class="memory-bar""#,
            r#"class="memory-bar-fill""#,
            r#"class="network-interfaces-table""#,
            r#"class="kea-leases-table""#,
            r#"class="disk-usage-stats""#,
            r#"class="process-usage-list""#,
            r#"class="device-controls""#,
            r#"class="coronatio-chart-canvas""#,
        ] {
            assert!(stats.contains(required), "stats markup missing og class stack {required}");
        }
        for forbidden in ["ui-progress-bar__", "ui-table", "ui-checkbox__", "ui-visibility-toggle"] {
            assert!(!stats.contains(forbidden), "stats body renamed og class into shared ui vocabulary: {forbidden}");
        }
        assert!(std::fs::read_to_string("src/bands/crown-law/stats-tabbar.rs").unwrap().contains("star-button"), "tabbar file read only; eye/star campaign deferred");
    }

    #[test]
    fn uxport_004_portals_source_library_and_drain_walls() {
        let html = render_crown_shell();
        let map = std::fs::read_to_string("docs/great-porting-map.md").unwrap();
        for citation in [
            "src/tablets/portals/index.tsx",
            "components/{PortalCard,AddPortalCard,AddPortalModal,ServiceStatusModal}.tsx",
            "PortalCard.css",
        ] {
            assert!(html.contains(citation) || map.contains(citation), "missing portals og citation {citation}");
        }
        assert!(html.contains("UXPORT-004 LIBRARY band: og src/tablets/portals portals domain pack"));
        for receipt in [
            "pane-root=ABSORB",
            "grid=ABSORB",
            "portal-card=ABSORB",
            "card-interior=ABSORB",
            "admin-controls=ABSORB",
            "add-portal-card=ABSORB",
            "portal-modal-form=ABSORB",
            "service-status-modal=ABSORB",
            "visibility-toggle=DEFERRED-to-VIS",
        ] {
            assert!(html.contains(receipt) || map.contains(receipt), "missing portals declaration-diff receipt {receipt}");
        }
        let holding_pen = std::fs::read_to_string("src/bands/shell/ux/shell/document-2-css.css").unwrap();
        let shell_base = std::fs::read_to_string("src/bands/shell/ux/shell/base-and-chrome.css").unwrap();
        for drained in [".portals-tablet", ".portals-grid", ".portal-grid", ".portal-card", ".portal-admin-controls", ".add-portal-card", ".portal-modal-overlay", ".service-status-modal"] {
            assert!(!holding_pen.contains(drained), "portal selector remained in holding pen: {drained}");
            assert!(!shell_base.contains(drained), "portal selector remained in shell base: {drained}");
            if drained != ".portal-grid" && drained != ".portal-admin-controls" {
                assert!(html.contains(drained), "drained portal selector not served from portals pack: {drained}");
            }
        }
        assert!(SHELL_UX_CHILDREN.contains(&"packs/portals.css"));
    }

    #[test]
    fn uxport_004_portals_markup_restores_og_structure_and_defers_vis() {
        let html = render_crown_shell();
        let portals_start = html.find("class=\"portals-tablet\" data-portals-viewport").unwrap();
        let portals_end = html.find("id=\"pane-upload\"").unwrap();
        let portals = &html[portals_start..portals_end];
        for required in [
            "class=\"portals-tablet\"",
            "class=\"portals-grid\"",
            "class=\"portal-card add-portal-card\"",
            "class=\"add-portal-content\"",
            "class=\"add-portal-icon\"",
            "class=\"add-portal-title\"",
            "class=\"add-portal-description\"",
            "class=\"portal-modal-overlay\"",
            "class=\"portal-modal-content\"",
            "class=\"add-portal-modal\"",
            "class=\"portal-form\"",
            "class=\"service-status-modal\"",
            "class=\"service-status-content\"",
            "class=\"copy-button\"",
        ] {
            let haystack = if required.contains("add-portal") { html.as_str() } else { portals };
            assert!(haystack.contains(required), "portals markup missing og class stack {required}");
        }
        let script = &html[html.find("function renderPortalCard").unwrap()..html.find("async function hydrateFavoriteManifest").unwrap()];
        assert!(script.contains("<div class=\"admin-controls\""));
        assert!(!script.contains("portal-admin-controls"));
        assert!(!script.contains("<article class=\"card portal-card"));
        assert!(script.contains("class=\"visibility-toggle\""), "VIS state-machine markup remains present for later campaign");
        assert!(portals.contains("data-portal-create-not-wired=\"true\""));
        assert!(portals.contains("aria-disabled=\"true\""));
    }

