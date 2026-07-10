    #[tokio::test]
    async fn full_rust_read_routes_are_registered() {
        let temp = test_tab_root("full-rust-read-routes");
        let app = app(AppState { tab_root: Arc::new(temp) });
        for route in [
            "/api/themes",
            "/api/uptime",
            "/api/portals",
            "/api/admin/updates/modules/example/status",
        ] {
            let response = app.clone().oneshot(Request::builder().uri(route).body(Body::empty()).unwrap()).await.unwrap();
            assert_eq!(response.status(), StatusCode::OK, "{route}");
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
            let body = String::from_utf8(bytes.to_vec()).unwrap();
            assert!(
                body.contains("coronatio.homeserver.route.read.guest.v1")
                    || body.contains("coronatio.uptime.v1")
                    || body.contains("coronatio.theme-catalog.response.v1")
                    || body.contains("coronatio.portals.config.v1"),
                "{body}"
            );
            if !body.contains("coronatio.homeserver.route.read.guest.v1") {
                assert!(body.contains(route) || route == "/api/themes", "{body}");
            }
            if route == "/api/uptime" { assert!(body.contains("uptimeSeconds") && body.contains("uptime"), "{body}"); }
        }
    }

    #[tokio::test]
    async fn full_rust_mutation_routes_enter_caduceus_membrane() {
        let temp = test_tab_root("full-rust-mutation-routes");
        let token = authorize_test_admin_token();
        let response = app(AppState { tab_root: Arc::new(temp) })
            .oneshot(Request::builder().method("POST").uri("/api/admin/system/restart").header("X-Admin-Token", token).body(Body::empty()).unwrap())
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
        assert!(html.contains(".portals-grid {"));
        assert!(html.contains("grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));"));
        assert!(html.contains(".portal-icon {"));
        assert!(html.contains("width: 120px;"));
        assert!(html.contains("height: 120px;"));
        assert!(html.contains(".portal-name {"));
        assert!(html.contains("font-size: 1.2rem;"));
        assert!(html.contains("font-weight: 500;"));
        assert!(html.contains(".portal-description {"));
        assert!(html.contains("line-height: 1.4;"));
        assert!(html.contains(r#"<img src="/api/portals/images/${encodeURIComponent(portal.name)}.png"#));
        assert!(html.contains("<h2 class=\"portal-name\">${escapeHtml(portal.name)}</h2>"));
        assert!(html.contains("<p class=\"portal-description\">${escapeHtml(portal.description || '')}</p>"));
        let card = &html[html.find("function renderPortalCard").unwrap()..html.find("async function handlePortalServiceAction").unwrap()];
        for forbidden in ["portal-service-row", "portal-chip", ":${escapeHtml(portal.port)}", "factory</span>", "custom</span>", "class=\"card portal-card"] {
            assert!(!card.contains(forbidden), "portal face still carries non-quarry chip/detail: {forbidden}");
        }
    }

    #[tokio::test]
    async fn upload_browse_hierarchical_returns_real_directory_json() {
        let _guard = HX_EXEMPLAR_ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
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
        assert!(html.contains(r#"type="file" multiple data-upload-file aria-label="Upload files""#));
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
        let upload_start = html.find(r#"class="upload-tablet" data-upload-viewport"#).unwrap();
        let upload_end = html.find(r#"data-upload-history-modal"#).unwrap();
        let upload = &html[upload_start..upload_end];
        let script = &html[html.find("function renderUploadProgress").unwrap()..html.find("function setUpload").unwrap()];
        for forbidden_restore in ["ui-breadcrumbs__item--current", "ui-file-input__display", "ui-progress-bar__fill"] {
            assert!(!upload.contains(forbidden_restore), "downgraded RESTORE class remains in upload markup: {forbidden_restore}");
            assert!(!script.contains(forbidden_restore), "downgraded RESTORE class remains in upload script: {forbidden_restore}");
        }
        for required in [
            "new XMLHttpRequest()",
            "xhr.upload.onprogress",
            "uploadState.selectedFiles",
            "uploadState.currentPath",
            "renderUploadProgress",
            "setUploadDirectoryError",
            "uploadCurrentPath()",
            "syncUploadTreeSelection",
            "/admit/upload/tree?path=%2Fmnt%2Fnas",
            "/api/upload/blacklist/update",
            "/api/upload/history/clear",
            "/api/upload/pin-required-status",
            "WARNING: This will override security settings",
        ] {
            assert!(html.contains(required), "missing upload workflow marker {required}");
        }
    }



    #[test]
    fn uxport_001_upload_source_and_library_walls_carry_og_citations() {
        let map = std::fs::read_to_string("docs/great-porting-map.md").unwrap();
        for citation in [
            "src/tablets/upload/index.tsx",
            "components/DirectoryBrowser.tsx",
            "components/UploadProgress.tsx",
            "upload.css",
            "src/components/ui/{Breadcrumbs,FileInput,ProgressBar}.tsx",
            "src/styles/common/ui/{_breadcrumbs,_file-input,_progress-bar}.css",
        ] {
            assert!(map.contains(citation), "missing og source citation {citation}");
        }
        let html = render_crown_shell();
        assert!(html.contains("UXPORT-001 LIBRARY band: og src/tablets/upload upload domain pack"));
        for selector in [
            ".upload-tablet { display: flex; flex-direction: column; height: 100%; min-height: 0; overflow-y: auto; gap: 12px; scrollbar-width: inherit; }",
            ".upload-controls { display: flex; flex-direction: column; gap: 16px; overflow: visible; }",
            ".upload-progress { background: var(--hiddenTabBackground); border-radius: 8px; padding: 12px; margin: 8px 0; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1); border: 1px solid var(--border); transition: transform 0.2s ease, box-shadow 0.2s ease; }",
            ".file-upload-section { display: flex; flex-wrap: wrap; gap: 8px; align-items: center; }",
            ".breadcrumb-item.current { color: var(--primary); background-color: var(--primaryHover); cursor: default; }",
            ".progress-bar-container { width: 100%; height: 20px; background: var(--hiddenTabBackground); color: var(--text); border-radius: 10px; overflow: hidden; position: relative; }",
            ".file-upload-section input[type=\"file\"] { background: var(--primary); border: none; border-radius: var(--border-radius); padding: 8px 12px; color: var(--text); cursor: pointer; transition: background var(--transition-fast); font-size: var(--font-size-sm); margin-right: 8px; margin-bottom: 8px; appearance: button; }",
        ] {
            assert!(html.contains(selector), "library band missing absorbed selector/declaration: {selector}");
        }
        for receipt in ["breadcrumbs=ABSORB", "file-picker=ABSORB", "upload-progress=ABSORB"] {
            assert!(html.contains(receipt), "missing declaration-diff receipt {receipt}");
        }
    }

    #[test]
    fn uxport_001_upload_markup_wall_uses_library_vocabulary_for_changed_elements() {
        let html = render_crown_shell();
        let upload_start = html.find(r#"class="upload-tablet" data-upload-viewport"#).unwrap();
        let upload_end = html.find(r#"data-upload-history-modal"#).unwrap();
        let upload = &html[upload_start..upload_end];
        for required in [
            r#"class="breadcrumb-navigation" data-upload-breadcrumbs"#,
            r#"class="breadcrumb-item current"#,
            r#"<input type="file" multiple data-upload-file aria-label="Upload files">"#,
            r#"<button type="button" data-upload-submit disabled>Upload Selected Files</button>"#,
        ] {
            assert!(upload.contains(required), "upload ABSORB markup missing og vocabulary {required}");
        }
        let script = &html[html.find("function renderUploadProgress").unwrap()..html.find("function setUpload").unwrap()];
        for required in ["progress-bar-container", "class=\"progress-bar\"", "progress-text"] {
            assert!(script.contains(required), "progress renderer missing ABSORB og vocabulary {required}");
        }
        for retired in ["ui-breadcrumbs__item--current", "ui-file-input__display", "ui-progress-bar__container", "ui-progress-bar__fill", "ui-progress-bar__text"] {
            assert!(!upload.contains(retired), "upload static markup kept downgraded RESTORE vocabulary {retired}");
            assert!(!script.contains(retired), "upload progress renderer kept downgraded RESTORE vocabulary {retired}");
        }
    }

    #[test]
    fn uxport_001_upload_non_drift_wall_names_old_and_new_class_stacks() {
        let html = render_crown_shell();
        let comparisons: [(&str, &[&str]); 3] = [
            (
                "breadcrumb-navigation > breadcrumb-item.current + breadcrumb-separator",
                &["breadcrumb-navigation", "breadcrumb-item current", "breadcrumb-separator"],
            ),
            (
                "native input[type=file] + button",
                &["file-upload-section", "data-upload-file", "data-upload-submit"],
            ),
            (
                "progress-bar-container > progress-bar > progress-text",
                &["progress-bar-container", "progress-bar", "progress-text"],
            ),
        ];
        for (old, absorbed_stack) in comparisons {
            for class_name in absorbed_stack {
                assert!(html.contains(class_name), "ABSORB class stack absent for {old}: missing {class_name}");
            }
        }
        let upload = &html[html.find("function renderUploadProgress").unwrap()..html.find("function setUpload").unwrap()];
        assert!(upload.contains("upload-progress ${upload.status}"), "absorbed upload outer stack missing");
        assert!(upload.contains("upload-stats"), "absorbed upload status stack missing");
        assert!(upload.contains("error-message"), "absorbed upload error stack missing");
    }

    #[test]
    fn uxport_001_power_quarry_gap_is_closed_in_map_without_implementation() {
        let map = std::fs::read_to_string("docs/great-porting-map.md").unwrap();
        assert!(map.contains("Runtime DOM truth supplied by operator"));
        assert!(map.contains(r#"<div class="modal" role="dialog" ...><div class="modal-title">Power Consumption</div>"#));
        assert!(map.contains("src/components/StatusIndicators/PowerMeterIndicator.tsx"));
        assert!(map.contains("src/components/StatusIndicators/indicators.css"));
        assert!(map.contains("src/components/Modal/index.tsx"));
        assert!(map.contains("src/components/Popup/PopupManager.tsx"));
        assert!(map.contains("Reclassify as a power-meter domain pack inside the shared modal substrate"));
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


    #[tokio::test]
    async fn hx_001_seats_vendored_htmx_csp_and_external_chrome() {
        let temp = test_tab_root("hx-001-htmx");
        let router = app(AppState { tab_root: Arc::new(temp) });
        let response = router.clone().oneshot(Request::builder().uri("/").body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let csp = response.headers().get(header::CONTENT_SECURITY_POLICY).and_then(|value| value.to_str().ok()).unwrap();
        assert!(csp.contains("script-src 'self'"));
        assert!(csp.contains("style-src 'self' 'unsafe-inline'"));
        let body = String::from_utf8(axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        assert!(body.contains(r#"<script defer src="/static/vendor/htmx.min.js" data-htmx-organ="2.0.10"></script>"#));
        assert!(body.contains(r#"<script defer src="/static/crown/chrome.js" data-crown-chrome="og-htmx"></script>"#));
        assert!(body.contains(r#"<template data-crown-chrome-source="externalized-for-csp">"#));
        let htmx = router.clone().oneshot(Request::builder().uri("/static/vendor/htmx.min.js").body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(htmx.status(), StatusCode::OK);
        let htmx_body = String::from_utf8(axum::body::to_bytes(htmx.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        assert!(htmx_body.starts_with("var htmx=function()"));
        assert!(htmx_body.contains("version:\"2.0.10\""));
        let chrome = router.oneshot(Request::builder().uri("/static/crown/chrome.js").body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(chrome.status(), StatusCode::OK);
        let chrome_body = String::from_utf8(axum::body::to_bytes(chrome.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        assert!(chrome_body.contains("htmxOrgan.config.allowScriptTags = false"));
        assert!(chrome_body.contains("htmxOrgan.config.selfRequestsOnly = true"));
        assert!(chrome_body.contains("htmx:afterSwap"));
        assert!(chrome_body.contains("if (id === 'stats') hydrateStats();"));
    }


    #[tokio::test]
    async fn test_tab_uses_generic_tab_scope_and_delegated_chrome() {
        let temp = test_tab_root("test-002-tab-scope");
        let router = app(AppState { tab_root: Arc::new(temp) });
        let response = router.clone().oneshot(Request::builder().uri("/").body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let shell = String::from_utf8(axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        for marker in [
            r#"data-tab-scope="test""#,
            r#"data-tab-id="showcase""#,
            r#"data-tab-panel="showcase""#,
            r#"data-tab-scope="showcase""#,
            r#"data-tab-id="buttons""#,
            r#"data-tab-panel="buttons""#,
        ] {
            assert!(shell.contains(marker), "served shell missing generic tab-scope marker: {marker}");
        }
        assert!(shell.contains(".showcase-section[data-tab-panel] { display:none; }"));
        assert!(shell.contains(".showcase-section[data-tab-panel].active { display:block; }"));
        for retired_hook in ["data-test-tab", "data-showcase-tab", "data-showcase-panel", "data-test-panel"] {
            assert!(!shell.contains(retired_hook), "served shell still carries retired tab hook: {retired_hook}");
        }
        assert!(!shell.contains("[data-showcase-panel]"));
        let chrome = router.oneshot(Request::builder().uri("/static/crown/chrome.js").body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(chrome.status(), StatusCode::OK);
        let chrome_body = String::from_utf8(axum::body::to_bytes(chrome.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        assert!(chrome_body.contains("function switchScopedTabs(tabButton)"));
        assert!(chrome_body.contains("closest('[data-tab-scope]')"));
        assert!(chrome_body.contains("event.target.closest('[data-tab-id]')"));
        assert!(chrome_body.contains("event.target.closest('[data-test-health-check]')"));
        for retired_hook in ["closest('[data-test-tab]')", "closest('[data-showcase-tab]')", "[data-showcase-panel]", "[data-test-panel]"] {
            assert!(!chrome_body.contains(retired_hook), "chrome still carries retired tab hook: {retired_hook}");
        }
        assert!(!chrome_body.contains("document.querySelectorAll('[data-test-health-check]').forEach"));
    }

    #[tokio::test]
    async fn hx_001_admit_route_serves_og_pane_fragments_fresh_and_records_faults() {
        let _guard = HX_EXEMPLAR_ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
        std::env::remove_var("CORONATIO_UPLOAD_ROOT");
        let temp = test_tab_root("hx-001-admit");
        let router = app(AppState { tab_root: Arc::new(temp) });
        let shell = render_crown_shell();
        for pane in ["portals", "stats", "upload", "admin", "test"] {
            let response = router.clone().oneshot(Request::builder().uri(format!("/admit/{pane}")).body(Body::empty()).unwrap()).await.unwrap();
            assert_eq!(response.status(), StatusCode::OK, "{pane} admits");
            assert_eq!(response.headers().get(header::CACHE_CONTROL).and_then(|value| value.to_str().ok()), Some("no-store"));
            let body = String::from_utf8(axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
            let expected = extract_pane_inner_html(&shell, pane).unwrap();
            assert_eq!(body, expected, "{pane} fragment is exact og pane body");
        }
        let missing = router.clone().oneshot(Request::builder().uri("/admit/missing-tab").body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(missing.status(), StatusCode::NOT_FOUND);
        let fault_body = String::from_utf8(axum::body::to_bytes(missing.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        assert!(fault_body.contains("data-cartridge-fault=\"true\""));
        assert!(fault_body.contains("data-cartridge-fault-kind=\"tab-not-found\""));
        let faults = router.clone().oneshot(Request::builder().uri("/api/faults").body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(faults.status(), StatusCode::OK);
        let body = String::from_utf8(axum::body::to_bytes(faults.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        assert!(body.contains("coronatio.cartridge-faults.v1"));
        assert!(body.contains("missing-tab"));
        let retired = router.clone().oneshot(Request::builder().uri(format!("/admit/{}", ["test", "tab"].concat())).body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(retired.status(), StatusCode::NOT_FOUND, "retired route must stay absent");
    }



    static HX_EXEMPLAR_ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

    #[tokio::test(flavor = "current_thread")]
    async fn hx_exemplar_subtree_fragment_returns_og_rows_for_fixture_dir() {
        let _guard = HX_EXEMPLAR_ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
        let root = test_tab_root("hx-exemplar-subtree-root");
        let media = root.join("media");
        std::fs::create_dir_all(media.join("films")).unwrap();
        std::fs::create_dir_all(media.join("shows")).unwrap();
        std::env::set_var("CORONATIO_UPLOAD_ROOT", &root);
        let route = format!("/admit/upload/tree?path={}&depth=1&selected={}", upload_query_escape(&format!("{}/media", root.display())), upload_query_escape(&root.display().to_string()));
        let response = app(AppState { tab_root: Arc::new(test_tab_root("hx-exemplar-subtree-app")) })
            .oneshot(Request::builder().uri(route).body(Body::empty()).unwrap())
            .await.unwrap();
        std::env::remove_var("CORONATIO_UPLOAD_ROOT");
        assert_eq!(response.status(), StatusCode::OK);
        let body = String::from_utf8(axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        assert!(body.contains(r#"class="directory-entry"#), "{body}");
        assert!(body.contains(r#"class="expand-control""#), "{body}");
        assert!(body.contains(r#"class="entry-icon""#), "{body}");
        assert!(body.contains(r#"class="entry-name">films</span>"#), "{body}");
        assert!(body.contains(r#"style="padding-left: 36px""#), "{body}");
    }

    #[tokio::test(flavor = "current_thread")]
    async fn hx_exemplar_caret_markup_carries_request_time_state_pointer() {
        let _guard = HX_EXEMPLAR_ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
        let root = test_tab_root("hx-exemplar-caret-root");
        std::fs::create_dir_all(root.join("media/films")).unwrap();
        std::env::set_var("CORONATIO_UPLOAD_ROOT", &root);
        let response = app(AppState { tab_root: Arc::new(test_tab_root("hx-exemplar-caret-app")) })
            .oneshot(Request::builder().uri("/admit/upload/tree").body(Body::empty()).unwrap())
            .await.unwrap();
        std::env::remove_var("CORONATIO_UPLOAD_ROOT");
        assert_eq!(response.status(), StatusCode::OK);
        let body = String::from_utf8(axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        assert!(body.contains(r#"name="selected" data-upload-current-path"#), "{body}");
        assert!(body.contains(r#"name="expanded" data-upload-expanded-paths"#), "{body}");
        assert!(body.contains(r#"hx-get="/admit/upload/tree?path="#), "{body}");
        assert!(body.contains("depth=1"), "{body}");
        assert!(body.contains(r#"hx-include="closest [data-upload-tree]""#), "{body}");
        assert!(body.contains(r#"hx-target="[data-upload-tree]""#), "{body}");
        assert!(body.contains("hx-swap=\"innerHTML\""), "{body}");
        assert!(body.contains(r#"hx-trigger="click consume""#), "{body}");
        for fragment in body.split("hx-get=\"").skip(1) {
            let url = fragment.split('"').next().unwrap_or("");
            if url.starts_with("/admit/upload/tree?") {
                assert!(!url.contains("selected="), "row URL baked stale selected state: {url}");
                assert!(!url.contains("expanded="), "row URL baked stale expanded state: {url}");
            }
        }
    }

    #[tokio::test(flavor = "current_thread")]
    async fn hx_exemplar_selection_echo_keeps_expanded_tree_state() {
        let _guard = HX_EXEMPLAR_ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
        let root = test_tab_root("hx-exemplar-selection-root");
        std::fs::create_dir_all(root.join("media/films")).unwrap();
        let media = format!("{}/media", root.display());
        let films = format!("{}/media/films", root.display());
        std::env::set_var("CORONATIO_UPLOAD_ROOT", &root);
        let route = format!(
            "/admit/upload/tree?path={}&depth=0&selected={}&expanded={}",
            upload_query_escape(&root.display().to_string()),
            upload_query_escape(&films),
            upload_query_escape(&media)
        );
        let response = app(AppState { tab_root: Arc::new(test_tab_root("hx-exemplar-selection-app")) })
            .oneshot(Request::builder().uri(route).body(Body::empty()).unwrap())
            .await.unwrap();
        std::env::remove_var("CORONATIO_UPLOAD_ROOT");
        assert_eq!(response.status(), StatusCode::OK);
        let body = String::from_utf8(axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        assert!(body.contains(&format!("data-directory-path=\"{}\" role=\"treeitem\" aria-selected=\"true\"", films)), "{body}");
        assert!(body.contains(&format!("name=\"selected\" data-upload-current-path value=\"{}\"", films)), "{body}");
        assert!(body.contains("name=\"expanded\" data-upload-expanded-paths"), "{body}");
        assert!(body.contains(r#"class="entry-name">films</span>"#), "selection must not collapse expanded media subtree: {body}");
    }

    #[test]
    fn hx_exemplar_tree_dialect_is_htmx_only_and_upload_xhr_is_allowlisted() {
        let _guard = HX_EXEMPLAR_ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
        let root = test_tab_root("hx-exemplar-dialect-root");
        std::fs::create_dir_all(root.join("media")).unwrap();
        std::env::set_var("CORONATIO_UPLOAD_ROOT", &root);
        let chrome = crown_chrome_js();
        assert!(!chrome.contains("renderDirectoryEntries"), "client-side tree renderer still present");
        assert!(!chrome.contains("loadUploadDirectory"), "client-side tree loader still present");
        assert!(!chrome.contains("/api/files/browse-hierarchical?path="), "tree lane still fetches browse JSON");
        assert!(!chrome.contains("const uploadTree = document.querySelector('[data-upload-tree]');"), "upload tree state must use the live swapped tree, not a stale captured node");
        assert!(chrome.contains("const activeUploadTree = document.querySelector('[data-upload-tree]');"), "upload tree selection sync must read the current swapped tree");
        assert!(chrome.contains("new XMLHttpRequest()"), "owned upload progress XHR must stay");
        assert!(chrome.contains("xhr.upload.onprogress"), "owned upload progress XHR must keep progress chrome");
        let shell = render_crown_shell();
        std::env::remove_var("CORONATIO_UPLOAD_ROOT");
        assert!(shell.contains(r#"data-upload-tree role="tree""#));
        assert!(shell.contains("hx-get=\"/admit/upload/tree"));
        assert!(shell.contains("hx-include=\"closest [data-upload-tree]\""));
    }

    #[tokio::test(flavor = "current_thread")]
    async fn hx_exemplar_re_expand_reads_fresh_subtree_from_server() {
        let _guard = HX_EXEMPLAR_ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
        let root = test_tab_root("hx-exemplar-fresh-root");
        let media = root.join("media");
        std::fs::create_dir_all(media.join("films")).unwrap();
        let media_display = format!("{}/media", root.display());
        let route = format!("/admit/upload/tree?path={}&depth=1&selected={}", upload_query_escape(&media_display), upload_query_escape(&root.display().to_string()));
        let router = app(AppState { tab_root: Arc::new(test_tab_root("hx-exemplar-fresh-app")) });
        std::env::set_var("CORONATIO_UPLOAD_ROOT", &root);
        let first = router.clone().oneshot(Request::builder().uri(&route).body(Body::empty()).unwrap()).await.unwrap();
        let first_body = String::from_utf8(axum::body::to_bytes(first.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        assert!(first_body.contains("films"), "{first_body}");
        assert!(!first_body.contains("concerts"), "{first_body}");
        std::fs::create_dir_all(media.join("concerts")).unwrap();
        let second = router.oneshot(Request::builder().uri(&route).body(Body::empty()).unwrap()).await.unwrap();
        std::env::remove_var("CORONATIO_UPLOAD_ROOT");
        let second_body = String::from_utf8(axum::body::to_bytes(second.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        assert!(second_body.contains("films"), "{second_body}");
        assert!(second_body.contains("concerts"), "server did not resolve fresh subtree on re-expand: {second_body}");
    }

    #[tokio::test(flavor = "current_thread")]
    async fn hx_exemplar_hx_request_expand_rerenders_sibling_child_rows_from_live_state() {
        let _guard = HX_EXEMPLAR_ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
        let root = test_tab_root("hx-exemplar-live-state-root");
        let music = root.join("music");
        std::fs::create_dir_all(music.join("albums")).unwrap();
        std::fs::create_dir_all(music.join("live")).unwrap();
        let root_display = root.display().to_string();
        let music_display = format!("{}/music", root.display());
        let router = app(AppState { tab_root: Arc::new(test_tab_root("hx-exemplar-live-state-app")) });
        std::env::set_var("CORONATIO_UPLOAD_ROOT", &root);
        let expand_route = format!(
            "/admit/upload/tree?path={}&depth=1&selected={}&expanded={}",
            upload_query_escape(&music_display),
            upload_query_escape(&root_display),
            upload_query_escape(&root_display)
        );
        let expanded = router
            .clone()
            .oneshot(Request::builder().uri(&expand_route).header("hx-request", "true").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(expanded.status(), StatusCode::OK);
        let expanded_body = String::from_utf8(axum::body::to_bytes(expanded.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        assert!(expanded_body.contains("data-upload-expanded-paths"), "{expanded_body}");
        assert!(expanded_body.contains(r#"class="entry-name">albums</span>"#), "{expanded_body}");
        assert!(expanded_body.contains(r#"class="entry-name">live</span>"#), "{expanded_body}");
        let music_row = expanded_body.find(&format!("data-directory-path=\"{}\"", music_display)).unwrap();
        let subtree = expanded_body[music_row..].find(&format!("data-upload-subtree=\"{}\"", music_display)).map(|idx| music_row + idx).unwrap();
        let albums = expanded_body.find(r#"class="entry-name">albums</span>"#).unwrap();
        assert!(music_row < subtree && subtree < albums, "expanded children must render in the sibling subtree after the parent row: {expanded_body}");
        let select_route = format!(
            "/admit/upload/tree?path={}&depth=0&selected={}&expanded={}",
            upload_query_escape(&format!("{}/music/albums", root.display())),
            upload_query_escape(&root_display),
            upload_query_escape(&format!("{},{}", root_display, music_display))
        );
        let selected = router
            .clone()
            .oneshot(Request::builder().uri(&select_route).header("hx-request", "true").body(Body::empty()).unwrap())
            .await
            .unwrap();
        let selected_body = String::from_utf8(axum::body::to_bytes(selected.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        assert!(selected_body.contains(&format!("data-directory-path=\"{}/music/albums\" role=\"treeitem\" aria-selected=\"true\"", root.display())), "{selected_body}");
        let collapse_route = format!(
            "/admit/upload/tree?path={}&depth=1&selected={}&expanded={}",
            upload_query_escape(&music_display),
            upload_query_escape(&format!("{}/music/albums", root.display())),
            upload_query_escape(&format!("{},{}", root_display, music_display))
        );
        let collapsed = router
            .clone()
            .oneshot(Request::builder().uri(&collapse_route).header("hx-request", "true").body(Body::empty()).unwrap())
            .await
            .unwrap();
        let collapsed_body = String::from_utf8(axum::body::to_bytes(collapsed.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        assert!(!collapsed_body.contains(r#"class="entry-name">albums</span>"#), "collapse must hide children: {collapsed_body}");
        std::fs::create_dir_all(music.join("sets")).unwrap();
        let reexpand_route = format!(
            "/admit/upload/tree?path={}&depth=1&selected={}&expanded={}",
            upload_query_escape(&music_display),
            upload_query_escape(&format!("{}/music/albums", root.display())),
            upload_query_escape(&root_display)
        );
        let reexpanded = router.oneshot(Request::builder().uri(&reexpand_route).header("hx-request", "true").body(Body::empty()).unwrap()).await.unwrap();
        std::env::remove_var("CORONATIO_UPLOAD_ROOT");
        let reexpanded_body = String::from_utf8(axum::body::to_bytes(reexpanded.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        assert!(reexpanded_body.contains(r#"class="entry-name">albums</span>"#), "{reexpanded_body}");
        assert!(reexpanded_body.contains(r#"class="entry-name">sets</span>"#), "re-expand must fresh-fetch server children: {reexpanded_body}");
    }

    #[test]
    fn hx_001_tabs_are_hypermedia_activation_controls() {
        let shell = render_crown_shell();
        for pane in ["portals", "stats", "upload"] {
            assert!(shell.contains(&format!(r#"hx-get="/admit/{pane}""#)));
            assert!(shell.contains(&format!(r#"hx-target="[data-view-panel='{pane}']""#)));
            assert!(shell.contains("hx-swap=\"innerHTML\""));
        }
        let nav_start = shell.find("<nav class=\"tab-bar\"").expect("tab bar starts");
        let nav_end = shell[nav_start..].find("</nav>").map(|offset| nav_start + offset).expect("tab bar ends");
        let nav = &shell[nav_start..nav_end];
        assert_eq!(nav.matches(r#"aria-selected="true""#).count(), 1);
        assert_eq!(nav.matches(r#"class="tab active""#).count(), 1);
        assert!(nav.contains(r#"hx-swap="innerHTML" hx-trigger="load, click""#));
    }



    #[tokio::test(flavor = "current_thread")]
    async fn hx_exemplar_admin_toggle_post_rerenders_real_state_card() {
        let _guard = HX_EXEMPLAR_ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
        let temp = test_tab_root("hx-admin-toggle");
        let sshd = temp.join("sshd_config");
        std::fs::write(&sshd, "PasswordAuthentication no\n").unwrap();
        std::env::set_var("CORONATIO_SSHD_CONFIG_FIXTURE", &sshd);
        std::env::set_var("CADUCEUS_URL", "http://127.0.0.1:9");
        let response = app(AppState { tab_root: Arc::new(test_tab_root("hx-admin-toggle-app")) })
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/admit/admin/toggle/ssh-password-authentication")
                    .header("X-Admin-Token", authorize_test_admin_token())
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        std::env::remove_var("CORONATIO_SSHD_CONFIG_FIXTURE");
        std::env::remove_var("CADUCEUS_URL");
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.headers().get(header::CACHE_CONTROL).and_then(|value| value.to_str().ok()), Some("no-store"));
        let body = String::from_utf8(axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        assert!(body.contains(r#"data-admin-toggle-card="ssh-password-authentication""#), "{body}");
        assert!(body.contains(r#"data-real-state="Disabled""#), "{body}");
        assert!(body.contains("sshd_config PasswordAuthentication readback"), "{body}");
        assert!(body.contains(r#"hx-post="/admit/admin/toggle/ssh-password-authentication""#), "{body}");
        assert!(body.contains("caduceus-unreachable"), "{body}");
        assert!(body.contains(r#"data-og-affordance="toast-mapped-to-result-strip""#), "{body}");
    }

    #[tokio::test(flavor = "current_thread")]
    async fn hx_exemplar_admin_non_admin_mutation_returns_membrane_refusal_fragment() {
        let response = app(AppState { tab_root: Arc::new(test_tab_root("hx-admin-refusal")) })
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/admit/admin/toggle/ssh-service")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        let body = String::from_utf8(axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        assert!(body.contains(r#"data-admin-membrane-refusal="true""#), "{body}");
        assert!(body.contains("Enter Admin Mode"), "{body}");
        assert!(body.contains("admin-session-required"), "{body}");
    }

    #[tokio::test(flavor = "current_thread")]
    async fn hx_exemplar_admin_action_strip_routes_and_og_affordance_markup() {
        let _guard = HX_EXEMPLAR_ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
        std::env::set_var("CADUCEUS_URL", "http://127.0.0.1:9");
        let router = app(AppState { tab_root: Arc::new(test_tab_root("hx-admin-actions")) });
        for action in ["hard-drive-test", "update", "restart", "shutdown", "restart-website", "view-logs", "install-certificate"] {
            let method = if action == "view-logs" { "GET" } else { "POST" };
            let response = router
                .clone()
                .oneshot(
                    Request::builder()
                        .method(method)
                        .uri(format!("/admit/admin/action/{action}"))
                        .header("X-Admin-Token", authorize_test_admin_token())
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();
            assert_eq!(response.status(), StatusCode::OK, "{action}");
            let body = String::from_utf8(axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
            assert!(body.contains(r#"data-admin-action-result-fragment="#), "{action}: {body}");
            assert!(body.contains(r#"data-og-affordance="toast-mapped-to-result-strip""#), "{action}: {body}");
            assert!(body.contains("caduceus-unreachable") || body.contains("caduceus-http-not-ok"), "{action}: {body}");
        }
        std::env::remove_var("CADUCEUS_URL");
        let api = router.oneshot(Request::builder().uri("/api").body(Body::empty()).unwrap()).await.unwrap();
        let body = String::from_utf8(axum::body::to_bytes(api.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        assert!(body.contains("/admit/admin/toggle/:toggle_id"));
        assert!(body.contains("/admit/admin/action/:action_id"));
    }

    #[test]
    fn hx_exemplar_admin_shell_uses_hx_dialect_and_documents_allowlisted_chrome_fetches() {
        let shell = render_crown_shell();
        for toggle in ["ssh-password-authentication", "ssh-service", "samba-file-sharing"] {
            assert!(shell.contains(&format!(r#"hx-post="/admit/admin/toggle/{toggle}""#)), "missing {toggle}");
        }
        for action in ["hard-drive-test", "update", "restart", "shutdown", "restart-website", "install-certificate"] {
            assert!(shell.contains(&format!(r#"hx-post="/admit/admin/action/{action}""#)), "missing {action}");
        }
        assert!(shell.contains(r#"hx-get="/admit/admin/action/view-logs""#));
        assert!(shell.contains(r#"data-admin-action-result data-og-affordance="toast-mapped-to-result-strip""#));
        assert!(shell.contains("hx-confirm=\"Double Click to Restart"));
        assert!(shell.contains("hx-confirm=\"Double Click to Shut Down"));
        let chrome = crown_chrome_js();
        assert!(chrome.contains("htmx:configRequest"));
        assert!(chrome.contains("X-Admin-Token"));
        assert!(chrome.contains("new XMLHttpRequest()"), "upload progress XHR is consciously allowlisted chrome");
        assert!(chrome.contains("fetch('/api/stats')"), "chart data fetch lane is consciously allowlisted chrome");
        assert!(chrome.contains("fetch('/api/validatePin'"), "PIN/session entry remains crown chrome and is consciously allowlisted");
        assert!(!chrome.contains("fetch('/api/admin/ssh/toggle'"));
        assert!(!chrome.contains("fetch('/api/admin/ssh/service'"));
        assert!(!chrome.contains("fetch('/api/admin/samba/service'"));
        assert!(!chrome.contains("fetch('/api/admin/hard-drive-test/start'"));
        assert!(!chrome.contains("fetch('/api/admin/system/restart'"));
        assert!(!chrome.contains("fetch('/api/admin/system/shutdown'"));
    }
