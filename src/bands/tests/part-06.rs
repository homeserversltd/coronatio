    #[test]
    fn indranet_animation_lab_and_flash_cure_walls() {
        let html = render_crown_shell();
        let css = std::fs::read_to_string("src/bands/shell/ux/packs/test-animations.css").unwrap();
        let services_css = std::fs::read_to_string("src/bands/shell/ux/packs/test-services.css").unwrap();
        let chrome = std::fs::read_to_string("src/bands/shell/document-4.rs").unwrap();
        for marker in [
            r#"data-tab-id="animations""#,
            r#"data-animation-lab"#,
            r#"data-motion-phase="REST""#,
            r#"data-motion-reduce-preview"#,
            "Band A — Motion Atoms",
            "Band B — Composed Motion",
            "Band C — Yijing Lifecycle",
            "Band D — Accessibility",
        ] {
            assert!(html.contains(marker), "animation lab missing {marker}");
        }
        assert!(!css.contains("transition: all"));
        assert!(css.contains("prefers-reduced-motion: reduce"));
        assert!(css.contains("[data-animation-lab] .motion-spinner.is-running"));
        assert!(services_css.contains("[data-test-services-grid] .portal-card::before"));
        assert!(services_css.contains("transition-property: border-color"));
        assert!(chrome.contains("document.querySelector('[data-portals-grid]')"));
        assert!(!chrome.contains("document.querySelector('[data-test-services-grid]')"));
        for marker in ["data-animation-play", "data-hover-specimen", "data-motion-phase-readback"] {
            assert!(html.contains(marker));
        }
        assert!(html.contains("data-motion-stillness"));
        assert!(chrome.contains("data-motion-stillness"));
        for forbidden in [".motion-button:hover { transform", ".motion-card:hover { transform", ".motion-card:hover { scale"] { assert!(!css.contains(forbidden)); }
    }

    #[test]
    fn portals_hover_lift_reflects_motion_tranch_01() {
        let html = render_crown_shell();
        let lab_css = std::fs::read_to_string("src/bands/shell/ux/packs/test-animations.css").unwrap();
        let portals_css = std::fs::read_to_string("src/bands/shell/ux/packs/portals.css").unwrap();

        for marker in [
            r#"data-animation-catalog-id="MOTION-TRANCH-01""#,
            "REST → ENTER (pointer or keyboard focus) → HOLD → EXIT → REST",
            r#"class="motion-card-stage" tabindex="0""#,
        ] {
            assert!(html.contains(marker), "hover-lift specimen missing {marker}");
        }

        for token in [
            "--motion-hover-lift-transform",
            "--motion-hover-lift-shadow",
            "--motion-hover-lift-duration",
            "--motion-hover-lift-easing",
        ] {
            assert!(lab_css.contains(token), "Animation Lab does not define {token}");
            assert!(portals_css.contains(&format!("var({token})")), "Portals does not consume {token}");
        }

        assert!(lab_css.contains(".motion-card-stage:is(:hover, :focus-visible) .motion-card"));
        assert!(portals_css.contains(".portal-element:is(:hover, :focus-within) > .portal-card"));
        assert!(portals_css.contains("MOTION-TRANCH-01 => MOTION-ATOM(transform, box-shadow) => MOTION-COMPOSE(hover-lift) => MOTION-REFLECT(Portals cards)"));
        assert!(portals_css.contains("@media (prefers-reduced-motion: reduce)"));
        assert!(portals_css.contains("transition-duration: .001ms !important"));
        let lift_start = portals_css.find("IndraNet reflection: MOTION-TRANCH-01").unwrap();
        let lift_end = portals_css.find(".portal-card-header").unwrap();
        let lift_path = &portals_css[lift_start..lift_end];
        assert!(!lift_path.contains("transition: all"));
        assert!(!lift_path.contains("animation:"));
        assert!(!lift_path.contains("infinite"));
        assert!(!lift_path.contains("translateZ"));
        assert!(!portals_css.contains(".portal-card:hover .portal-icon"));
        assert!(!portals_css.contains(".add-portal-card:hover .add-portal-icon"));
    }

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
            assert!(html.contains(receipt), "missing stats declaration-diff receipt {receipt}");
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
        for forbidden in ["ui-progress-bar__", "ui-table", "ui-checkbox__"] {
            assert!(!stats.contains(forbidden), "stats body renamed og class into shared ui vocabulary: {forbidden}");
        }
        assert!(std::fs::read_to_string("src/bands/crown-law/stats-tabbar.rs").unwrap().contains("star-button"), "tabbar file read only; eye/star campaign deferred");
    }

    #[test]
    fn uxport_004_portals_source_library_and_drain_walls() {
        let html = render_crown_shell();
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
            assert!(html.contains(receipt), "missing portals declaration-diff receipt {receipt}");
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
        assert!(script.contains("class=\"visibility-toggle ui-visibility-toggle\""), "VIS state-machine markup uses the catalog vocabulary");
        assert!(portals.contains("data-portal-create-not-wired=\"true\""));
        assert!(portals.contains("aria-disabled=\"true\""));
    }



    #[test]
    fn uxport_005_admin_source_library_and_drain_walls() {
        let html = render_crown_shell();
        assert!(html.contains("UXPORT-005 LIBRARY band: og src/tablets/admin admin domain pack"));
        for receipt in [
            "pane-root=ABSORB",
            "admin-index-inline-fold=ABSORB",
            "system-controls=ABSORB",
            "service-toggle=ABSORB",
            "key-manager-root-layout=ABSORB",
            "security-status=ABSORB",
            "key-action-buttons=ABSORB",
            "disk-manager-root-layout=ABSORB",
            "disk-item-state-family=ABSORB",
            "disk-metadata-badges=ABSORB",
            "disk-action-buttons=ABSORB",
            "admin-modals=CONTINUATION",
            "modal-basic-buttons=CONTINUATION",
        ] {
            assert!(html.contains(receipt), "missing admin declaration-diff receipt {receipt}");
        }
        let holding_pen = std::fs::read_to_string("src/bands/shell/ux/shell/document-2-css.css").unwrap();
        for drained in [".admin-tablet", ".system-controls", ".system-controls-btn", ".ssh-control", ".samba-control", ".key-manager", ".security-status", ".disk-manager", ".disk-item", ".disk-actions"] {
            assert!(!holding_pen.contains(drained), "admin system/key/disk selector remained in document-2 holding pen: {drained}");
            assert!(html.contains(drained), "drained admin selector not served from admin pack: {drained}");
        }
        assert!(!html.contains("admin-visual-port"), "Rust-only admin visual helper remained after admin pack absorption");
        assert!(SHELL_UX_CHILDREN.contains(&"packs/admin.css"));
    }

    #[test]
    fn uxport_005_admin_markup_restores_og_structures_without_static_modal_shelf() {
        let html = render_crown_shell();
        let admin_start = html.find("class=\"admin-tablet\"").unwrap();
        let admin_end = html.find("id=\"pane-stats\"").unwrap();
        let admin = &html[admin_start..admin_end];
        for required in [
            "class=\"admin-tablet\"",
            "class=\"mb-6\" style=\"margin-bottom: 0.5rem\"",
            "class=\"system-controls-container\"",
            "class=\"system-controls\"",
            "class=\"system-controls-btn\"",
            "class=\"ssh-controls\"",
            "class=\"ssh-status\"",
            "class=\"ssh-toggle\"",
            "class=\"toggle-switch\"",
            "class=\"toggle-slider\"",
            "class=\"toggle-label\"",
            "class=\"samba-status\"",
            "class=\"samba-toggle\"",
            "class=\"key-manager-content\"",
            "class=\"key-manager-left\"",
            "class=\"key-manager-right\"",
            "class=\"security-status\"",
            "class=\"status-icon secure\"",
            "class=\"action-button create-button\"",
            "class=\"action-button update-button\"",
            "class=\"action-button admin-password-button\"",
            "class=\"disk-manager-container\"",
            "class=\"disk-column\"",
            "class=\"disk-list\"",
            "disk-item",
            "disk-space-usage",
        ] {
            assert!(admin.contains(required), "admin markup missing og class stack {required}");
        }
        assert!(admin.contains("class=\"disk-actions\""));
        assert!(admin.contains("class=\"action-button info-button\""));
        assert!(!admin.contains("admin-modal-shelf"));
        assert!(!admin.contains("data-admin-quarry"));
        assert!(!admin.contains("data-stub-action"));
    }

    #[tokio::test]
    async fn test_tab_secondary_panels_are_full_catalog_specimens() {
        let temp = test_tab_root("test-003-secondary-panels");
        let router = app(AppState { tab_root: Arc::new(temp) });
        let response = router
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let shell = String::from_utf8(
            axum::body::to_bytes(response.into_body(), usize::MAX)
                .await
                .unwrap()
                .to_vec(),
        )
        .unwrap();
        for marker in [
            r#"data-catalog-specimen="test-services-portal-grid""#,
            r#"data-config-pattern="readback""#,
            r#"data-config-pattern="form""#,
            r#"data-config-pattern="grid""#,
            r#"data-health-viewport="summary""#,
            r#"data-health-viewport="services""#,
            r#"data-health-viewport="diagnostics""#,
            "add-portal-card",
            "admin-controls",
            "health-timeline",
            "health-results",
            "data-test-health-output",
        ] {
            assert!(shell.contains(marker), "missing Test specimen marker: {marker}");
        }
        for status in ["up", "down", "partial", "unknown"] {
            assert!(shell.contains(&format!("portal-card {status}")));
        }
        assert!(shell.matches("portal-element").count() >= 7);
        assert!(shell.matches("health-service-card").count() >= 5);
        for pack in ["test-services.css", "test-config.css", "test-health.css"] {
            assert!(SHELL_UX_INDEX_JSON.contains(pack));
        }
    }
