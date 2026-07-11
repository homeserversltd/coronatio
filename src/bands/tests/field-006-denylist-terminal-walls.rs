    fn field_006_rust_source_files(root: &str) -> Vec<std::path::PathBuf> {
        fn visit(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
            for entry in std::fs::read_dir(dir).unwrap() {
                let path = entry.unwrap().path();
                if path.is_dir() {
                    visit(&path, out);
                } else if path.extension().and_then(|ext| ext.to_str()) == Some("rs") {
                    out.push(path);
                }
            }
        }
        let mut out = Vec::new();
        visit(std::path::Path::new(root), &mut out);
        out
    }

    #[test]
    fn field_006_source_wall_denylist_genome_symbols_do_not_exist() {
        let forbidden = [
            format!("{}{}", "admin_field_", "filters"),
            format!("{}{}{}", "Admin", "Field", "Filter"),
            format!("{}{}", "filter_", "admin"),
        ];
        for path in field_006_rust_source_files("src") {
            let body = std::fs::read_to_string(&path).unwrap();
            for token in &forbidden {
                assert!(!body.contains(token), "denylist genome token {token} survives in {}", path.display());
            }
        }
    }

    #[test]
    fn field_006_category_wall_fact_routes_are_census_classified() {
        assert_eq!(field_005b_typed_projection_get_routes().len(), 12, "read-census typed-projection bucket moved");
        assert_eq!(field_005b_generic_projected_get_routes().len(), 88, "read-census generic bucket moved");
        assert_eq!(field_005b_real_body_exception_get_routes().len(), 10, "read-census real-body bucket moved");
        assert_eq!(field_005b_og_admin_gated_get_routes().len(), 2, "read-census og-admin bucket moved");
        assert_eq!(field_005_gated_this_slice_mutations().len(), 103, "mutation-census gated bucket moved");
        assert_eq!(field_005_previously_gated_mutations().len(), 19, "mutation-census previous bucket moved");
        assert_eq!(field_005_named_exclusion_mutations().len(), 2, "mutation-census exclusion bucket moved");
        let audit = std::fs::read_to_string("docs/field-projection-audit.md").unwrap();
        for marker in ["Route census rows: 13", "Field rows classified in section C: 23", "F-question ids minted: F1-F13"] {
            assert!(audit.contains(marker), "field-census audit marker missing {marker}");
        }
    }

    #[test]
    fn field_006_registry_and_service_admin_schema_no_longer_advertise_denylist_law() {
        let session = serde_json::to_value(admin_session_readback()).unwrap();
        let service = serde_json::to_value(project_service_data_admin(&service_data_readback())).unwrap();
        assert!(session.get("adminEnhancedFiltering").is_none(), "registry readback kept denylist field: {session}");
        assert!(service.get("adminFieldLaw").is_none(), "service admin projection kept denylist field: {service}");
    }

    #[test]
    fn field_006_audit_wall_terminal_state_and_quarry_topic_law_distinction_are_recorded() {
        let audit = std::fs::read_to_string("docs/field-projection-audit.md").unwrap();
        for required in [
            "FIELD-006 terminal amendment",
            "RETIRED by FIELD-006",
            "merge citations: FIELD-000, FIELD-001R, FIELD-003, FIELD-004, FIELD-005, FIELD-005b, FIELD-006",
            "QUARRY-CONTRACT DOCUMENTATION",
            "not a runtime filter mechanism",
            "operator-blocked F11 upload product slice",
            "covenant-deferred ports",
            "denylist genome symbols in src/: 0",
        ] {
            assert!(audit.contains(required), "FIELD-006 audit terminal marker missing {required}");
        }
    }
