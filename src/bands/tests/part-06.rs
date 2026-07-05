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

