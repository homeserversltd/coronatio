    #[tokio::test]
    async fn full_rust_read_routes_are_registered() {
        let temp = test_tab_root("full-rust-read-routes");
        let app = app(AppState { tab_root: Arc::new(temp) });
        for route in [
            "/api/themes",
            "/api/uptime",
            "/api/portals",
            "/api/upload/history",
            "/api/admin/updates/modules/example/status",
        ] {
            let response = app.clone().oneshot(Request::builder().uri(route).body(Body::empty()).unwrap()).await.unwrap();
            assert_eq!(response.status(), StatusCode::OK, "{route}");
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
            let body = String::from_utf8(bytes.to_vec()).unwrap();
            assert!(
                body.contains("coronatio.homeserver.route.read.v1")
                    || body.contains("coronatio.uptime.v1")
                    || body.contains("coronatio.upload.history.v1")
                    || body.contains("coronatio.theme-catalog.response.v1")
                    || body.contains("coronatio.portals.config.v1"),
                "{body}"
            );
            assert!(body.contains(route) || route == "/api/upload/history" || route == "/api/themes", "{body}");
            if route == "/api/uptime" { assert!(body.contains("uptimeSeconds") && body.contains("uptime"), "{body}"); }
        }
    }

    #[tokio::test]
    async fn full_rust_mutation_routes_enter_caduceus_membrane() {
        let temp = test_tab_root("full-rust-mutation-routes");
        let response = app(AppState { tab_root: Arc::new(temp) })
            .oneshot(Request::builder().method("POST").uri("/api/admin/system/restart").body(Body::empty()).unwrap())
            .await.unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body = String::from_utf8(bytes.to_vec()).unwrap();
        assert!(body.contains("coronatio.homeserver.route.mutation.v1"));
        assert!(body.contains("Caduceus staff intent membrane"));
        assert!(body.contains("/api/admin/system/restart"));
    }

    #[test]
    fn main_rs_is_thin_infinite_infinite_face() {
        let main_rs = std::fs::read_to_string("src/main.rs").unwrap();
        assert!(main_rs.contains(r#"include!("bands/contracts.rs")"#));
        assert!(main_rs.contains(r#"include!("bands/shell.rs")"#));
        assert!(main_rs.lines().count() < 80);
        assert!(std::path::Path::new("src/bands/index.json").exists());
    }

    #[test]
    fn rust_source_files_stay_under_line_pressure() {
        fn walk(path: &std::path::Path, failures: &mut Vec<String>) {
            for entry in std::fs::read_dir(path).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();
                if path.is_dir() {
                    walk(&path, failures);
                } else if path.extension().is_some_and(|extension| extension == "rs") {
                    let line_count = std::fs::read_to_string(&path).unwrap().lines().count();
                    if line_count > 800 {
                        failures.push(format!("{} has {line_count} lines", path.display()));
                    }
                }
            }
        }
        let mut failures = Vec::new();
        walk(std::path::Path::new("src"), &mut failures);
        assert!(failures.is_empty(), "{}", failures.join("\n"));
    }

    #[test]
    fn website_endpoint_inventory_is_explicit_rust_routes() {
        let inventory = full_rust_route_inventory();
        assert!(inventory.len() > 130, "expected broad website endpoint route table");
        for required in [
            "/api/files/upload",
            "/api/service/control",
            "/api/admin/diskman/mount",
            "/api/admin/updates/modules/:module_name/status",
            "/api/wakeonlan/targets",
            "/api/dhcp/reservations/:reservation_id",
            "/api/youtube/download",
            "/api/backup/status",
            "/api/miner/stats",
        ] {
            assert!(inventory.iter().any(|(path, _)| *path == required), "missing {required}");
        }
    }

    #[tokio::test]
    async fn upload_viewport_posts_to_caduceus_route() {
        let temp = test_tab_root("upload-viewport");
        let response = app(AppState { tab_root: Arc::new(temp) })
            .oneshot(Request::builder().method("POST").uri("/api/files/upload").header("content-type", "multipart/form-data; boundary=X").body(Body::from("--X\r\nContent-Disposition: form-data; name=\"path\"\r\n\r\n/mnt/nas\r\n--X\r\nContent-Disposition: form-data; name=\"file\"; filename=\"proof.txt\"\r\nContent-Type: text/plain\r\n\r\nhello\r\n--X--\r\n")).unwrap())
            .await.unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body = String::from_utf8(bytes.to_vec()).unwrap();
        assert!(body.contains("coronatio.upload.submit.v1"), "{body}");
        assert!(body.contains("proof.txt"), "{body}");
        assert!(body.contains("Coronatio Rust upload route to Caduceus"), "{body}");
    }

    #[test]
    fn portals_grid_and_card_face_match_quarry_without_chips() {
        let html = render_crown_shell();
        assert!(html.contains(".portal-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(300px, 1fr)); gap: 20px; align-items: stretch; width: 100%; }"));
        assert!(html.contains(".portal-icon { width: 120px; height: 120px; object-fit: contain; border-radius: 8px;"));
        assert!(html.contains(".portal-name { margin: .25rem 0 0; padding: 0; font-size: 1.2rem; font-weight: 500;"));
        assert!(html.contains(".portal-description { margin: 0; color: var(--text); opacity: .8; font-size: .9rem; line-height: 1.4;"));
        assert!(html.contains(r#"<img src="/api/portals/images/${encodeURIComponent(portal.name)}.png"#));
        assert!(html.contains("<h2 class=\"portal-name\">${escapeHtml(portal.name)}</h2>"));
        assert!(html.contains("<p class=\"portal-description\">${escapeHtml(portal.description || '')}</p>"));
        let card = &html[html.find("function renderPortalCard").unwrap()..html.find("async function handlePortalServiceAction").unwrap()];
        for forbidden in ["portal-service-row", "portal-chip", ":${escapeHtml(portal.port)}", "factory</span>", "custom</span>"] {
            assert!(!card.contains(forbidden), "portal face still carries non-quarry chip/detail: {forbidden}");
        }
    }

    #[tokio::test]
    async fn upload_browse_hierarchical_returns_real_directory_json() {
        let root = test_tab_root("upload-browse-root");
        let media = root.join("media");
        let films = media.join("films");
        std::fs::create_dir_all(&films).unwrap();
        std::fs::create_dir_all(root.join("empty")).unwrap();
        std::env::set_var("CORONATIO_UPLOAD_ROOT", &root);
        let response = app(AppState { tab_root: Arc::new(test_tab_root("upload-browse-app")) })
            .oneshot(Request::builder().uri("/api/files/browse-hierarchical?path=/mnt/nas&expand=true").body(Body::empty()).unwrap())
            .await.unwrap();
        std::env::remove_var("CORONATIO_UPLOAD_ROOT");
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(body["schema"], "coronatio.upload.browse_hierarchical.v1");
        assert_eq!(body["path"], root.display().to_string());
        let entries = body["entries"].as_array().unwrap();
        assert_eq!(entries[0]["name"], "empty");
        assert_eq!(entries[1]["name"], "media");
        assert_eq!(entries[1]["hasChildren"], true);
        assert_eq!(entries[1]["path"], format!("{}/media", root.display()));
    }

    #[test]
    fn upload_viewport_ports_react_tablet_dom_grammar() {
        let html = render_crown_shell();
        assert!(html.contains(r#"class="upload-tablet" data-upload-viewport data-react-quarry="UploadTablet" data-identity-standard="one-to-one""#));
        assert!(html.contains(r#"class="upload-progress-list" data-upload-progress-list"#));
        assert!(html.contains(r#"class="upload-controls""#));
        assert!(html.contains(r#"class="directory-browser" data-upload-regular="directory-browser" data-directory-browser"#));
        assert!(html.contains(r#"class="directory-browser-header""#));
        assert!(html.contains(r#"title="Refresh Directory Tree""#));
        assert!(html.contains("🛡️ Allow"));
        assert!(html.contains("📌 Default"));
        assert!(html.contains("🚫 Blacklist"));
        assert!(html.contains("📜 History"));
        assert!(html.contains(r#"class="toggle-pin-button""#));
        assert!(html.contains(r#"class="directory-breadcrumb-container""#));
        assert!(html.contains(r#"class="breadcrumb-navigation""#));
        assert!(html.contains(r#"class="directory-tree-container""#));
        assert!(html.contains(r#"class="directory-entry selected""#));
        assert!(html.contains(r#"class="expand-control""#));
        assert!(html.contains(r#"class="entry-icon""#));
        assert!(html.contains(r#"class="entry-name""#));
        assert!(html.contains(r#"class="entry-selected""#));
        assert!(html.contains(r#"class="file-upload-section" data-upload-regular="file-ingress""#));
        assert!(html.contains(r#"type="file" multiple data-upload-file"#));
        assert!(html.contains("Upload Selected Files"));
        assert!(html.contains(r#"data-upload-history-modal"#));
        assert!(html.contains("No upload history available"));
        assert!(html.contains(r#"class="clear-history-button""#));
        assert!(html.contains(r#"data-upload-blacklist-modal"#));
        assert!(html.contains(r#"class="blacklist-manager""#));
        assert!(html.contains("Enter path to blacklist"));
        assert!(html.contains(r#"data-upload-pin-modal"#));
        assert!(html.contains("Admin PIN Required"));
        assert!(html.contains("Please enter the admin PIN to proceed with the upload."));
    }

    #[test]
    fn upload_scaffold_cards_are_obliterated_from_living_viewport() {
        let html = render_crown_shell();
        for forbidden in [
            "upload-progress-card",
            "upload-admin-card",
            "upload-directory-card",
            "upload-card",
            "Selected file and Caduceus upload intent readback appear here.",
            "Admin upload controls",
            "Browse directory",
            "Refresh tree",
            ">Upload</button>",
            "Destination <input",
            "Default directory</button>",
            "PIN requirement</button>",
        ] {
            assert!(!html.contains(forbidden), "legacy upload scaffold still present: {forbidden}");
        }
    }

    #[test]
    fn upload_admin_controls_are_header_enhancements_and_regular_upload_remains() {
        let html = render_crown_shell();
        assert!(html.contains(r#"[data-admin-mode="false"] [data-admin-only]:not([data-admin-only="false"])"#));
        let header = html.find(r#"class="directory-browser-header""#).expect("directory browser header present");
        let tree = html.find(r#"class="directory-tree-container""#).expect("directory tree present");
        let header_region = &html[header..tree];
        for admin_header_control in ["🛡️ Allow", "📌 Default", "🚫 Blacklist", "📜 History", "toggle-pin-button"] {
            assert!(header_region.contains(admin_header_control), "missing upload admin header control {admin_header_control}");
        }
        let file_section = html.find(r#"class="file-upload-section""#).expect("file upload section present");
        let pin_modal = html.find(r#"data-upload-pin-modal"#).expect("pin modal present");
        let regular_region = &html[file_section..pin_modal];
        assert!(!regular_region.contains("data-admin-only"), "regular upload file section must not be admin-gated");
        assert!(regular_region.contains("data-upload-file"));
        assert!(regular_region.contains("multiple"));
        assert!(regular_region.contains("Upload Selected Files"));
    }

    #[test]
    fn upload_script_uses_xhr_progress_and_react_workflows() {
        let html = render_crown_shell();
        for required in [
            "new XMLHttpRequest()",
            "xhr.upload.onprogress",
            "uploadState.selectedFiles",
            "uploadState.currentPath",
            "renderUploadProgress",
            "/api/files/browse-hierarchical?path=",
            "/api/upload/blacklist/update",
            "/api/upload/history/clear",
            "/api/upload/pin-required-status",
            "WARNING: This will override security settings",
        ] {
            assert!(html.contains(required), "missing upload workflow marker {required}");
        }
    }


    #[test]
    fn css_sizing_ports_quarry_verbatim_values_for_port_005_and_retrofit() {
        let html = render_crown_shell();
        for required in [
            "padding: 0 1rem;",
            "height: calc(var(--theme-header-height) - 8px);",
            "padding: 0 20px;",
            "min-height: 48px;",
            "width: 120px; height: 120px;",
            "grid-template-columns: repeat(auto-fill, minmax(300px, 1fr)); gap: 20px;",
            "height: 180px; margin: 0;",
            "min-width: 200px; padding: 8px 12px; border: 1px solid var(--border);",
            "max-height: 70vh; width: 100%;",
            "padding: 4px 8px; cursor: pointer; border-radius: 4px;",
        ] {
            assert!(html.contains(required), "missing verbatim quarry CSS value {required}");
        }
        let directory_rule = html.split(".directory-entry { ").nth(1).unwrap().split(" }").next().unwrap();
        assert!(!directory_rule.contains("min-height"), "directory rows must not invent non-quarry row height: {directory_rule}");
    }
