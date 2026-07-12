    #[test]
    fn portals_ui_wire_wall_shell_enables_portal_create_and_delete_handlers() {
        let html = render_crown_shell();
        let portals_start = html.find("id=\"pane-portals\"").expect("portals pane");
        let portals_end = html.find("id=\"pane-test\"").unwrap_or(html.len());
        let portals = &html[portals_start..portals_end];
        assert!(!portals.contains("data-portal-create-not-wired"));
        assert!(!portals.contains("portal-create-not-wired"));
        assert!(html.contains("submitPortalForm"));
        assert!(html.contains("deletePortal"));
        assert!(html.contains("data-portal-add-form"));
        assert!(html.contains("factoryPortalNames"));
        assert!(html.contains("data-portal-delete"));
    }
