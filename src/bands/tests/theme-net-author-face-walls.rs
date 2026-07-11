    fn author_face_names() -> std::collections::HashSet<String> {
        let source = std::fs::read_to_string("src/bands/shell/ux/author-face.json").unwrap();
        let value: serde_json::Value = serde_json::from_str(&source).unwrap();
        value["variables"]
            .as_array()
            .unwrap()
            .iter()
            .map(|item| item.as_str().unwrap().to_string())
            .collect()
    }

    fn css_var_calls(css: &str) -> Vec<String> {
        let mut calls = Vec::new();
        let mut tail = css;
        while let Some(start) = tail.find("var(--") {
            let after_prefix = &tail[start + 6..];
            let end = after_prefix
                .find(|character: char| character == ')' || character == ',' || character.is_whitespace())
                .unwrap();
            calls.push(after_prefix[..end].to_string());
            tail = &after_prefix[end..];
        }
        calls
    }

    fn pack_css() -> Vec<(String, String)> {
        let mut packs = std::fs::read_dir("src/bands/shell/ux/packs")
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .filter(|path| path.extension().is_some_and(|extension| extension == "css"))
            .map(|path| {
                let name = path.display().to_string();
                (name, std::fs::read_to_string(path).unwrap())
            })
            .collect::<Vec<_>>();
        packs.sort_by(|left, right| left.0.cmp(&right.0));
        packs
    }

    #[test]
    fn theme_net_author_face_wall_allowlist_is_machine_readable_and_rebound_by_shell() {
        let names = author_face_names();
        assert!((16..=48).contains(&names.len()), "author face must remain shallow: {}", names.len());
        for required in ["background", "text-muted", "info", "status-up", "statusUp", "monoFont", "tabBorder"] {
            assert!(names.contains(required), "missing author face name {required}");
        }
        let shell = std::fs::read_to_string("src/bands/shell/ux/shell/base-and-chrome.css").unwrap();
        for name in names {
            assert!(
                shell.contains(&format!("--{name}: var(--theme-")),
                "author face name {name} must be a pure Theme Net rebind"
            );
        }
    }

    #[test]
    fn theme_net_author_face_wall_packs_use_only_shallow_names_not_deep_lattice() {
        let names = author_face_names();
        for (path, css) in pack_css() {
            assert!(!css.contains("var(--theme-"), "{path} reaches through the author face into the deep lattice");
            for name in css_var_calls(&css) {
                let locally_declared = css.contains(&format!("--{name}:"));
                assert!(
                    names.contains(&name) || locally_declared,
                    "{path} uses non-author-face, non-local var(--{name})"
                );
            }
        }
    }

    #[test]
    fn theme_net_author_face_wall_packs_have_no_hex_or_colored_rgba_paint() {
        for (path, css) in pack_css() {
            let bytes = css.as_bytes();
            for index in 0..bytes.len() {
                if bytes[index] == b'#' {
                    let suffix = &css[index + 1..];
                    let hex_count = suffix.chars().take_while(|character| character.is_ascii_hexdigit()).count();
                    assert!(
                        !matches!(hex_count, 3 | 6 | 8),
                        "{path} contains hex paint {}",
                        &css[index..index + 1 + hex_count]
                    );
                }
            }
            let mut tail = css.as_str();
            while let Some(index) = tail.find("rgba(") {
                let call = &tail[index..];
                assert!(
                    call.starts_with("rgba(0,0,")
                        || call.starts_with("rgba(0, 0,")
                        || call.starts_with("rgba(255,255,")
                        || call.starts_with("rgba(255, 255,")
                        || call.starts_with("rgba(var(--") && call.contains("-rgb),"),
                    "{path} contains non-neutral rgba paint: {}",
                    call.lines().next().unwrap_or(call)
                );
                tail = &call[5..];
            }
        }
    }

    #[test]
    fn theme_net_author_face_cookbook_is_linked_and_teaches_class_first_ladder() {
        let readme = std::fs::read_to_string("src/bands/shell/ux/README.md").unwrap();
        let cookbook = std::fs::read_to_string("src/bands/shell/ux/TAB-AUTHOR.md").unwrap();
        assert!(readme.contains("TAB-AUTHOR.md"));
        for required in [".ux-*", "author-face.json", "data-domain", "Do not invent hex", "--theme-role-*", "--theme-gradient-*", "pali:coronatio-original-website-firmware-port-law"] {
            assert!(cookbook.contains(required), "cookbook misses {required}");
        }
    }
