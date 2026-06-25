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
                    || body.contains("coronatio.upload.history.v1")
                    || body.contains("coronatio.theme-catalog.response.v1")
                    || body.contains("coronatio.portals.config.v1"),
                "{body}"
            );
            assert!(body.contains(route) || route == "/api/upload/history" || route == "/api/themes", "{body}");
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
    fn upload_viewport_has_regular_upload_and_admin_enhancement_split() {
        let html = render_crown_shell();
        assert!(html.contains("data-upload-regular=\"file-ingress\""));
        assert!(html.contains("data-upload-form"));
        assert!(html.contains("data-upload-file"));
        assert!(html.contains(">Upload</button>"));
        assert!(html.contains("data-upload-regular=\"directory-browser\""));
        assert!(html.contains("Browse directory"));
        assert!(html.contains("Refresh tree"));
        assert!(html.contains("data-upload-regular=\"progress\""));
        assert!(html.contains("/api/files/upload"));
        assert!(html.contains("/api/files/browse-hierarchical"));
        assert!(html.contains("/api/files/browse"));
        assert!(html.contains("data-admin-only data-admin-viewport=\"upload\""));
        for enhanced in [
            "Force Allow Upload",
            "Default directory",
            "Set Default Directory",
            "PIN requirement",
            "Manage Blacklist",
            "Upload History",
            "Clear History",
        ] {
            assert!(html.contains(enhanced), "missing upload admin enhancement {enhanced}");
        }
    }

    #[test]
    fn upload_admin_controls_are_hidden_until_admin_mode_but_regular_upload_remains() {
        let html = render_crown_shell();
        assert!(html.contains(r#"class="card upload-admin-card" data-admin-only data-admin-viewport="upload""#));
        assert!(html.contains(r#"[data-admin-mode="false"] [data-admin-only]:not([data-admin-only="false"])"#));
        let regular_upload = html.find(r#"data-upload-regular="file-ingress""#).expect("regular upload card present");
        let admin_upload = html.find(r#"class="card upload-admin-card""#).expect("admin upload card present");
        assert!(regular_upload < admin_upload);
        let regular_region = &html[regular_upload..admin_upload];
        assert!(!regular_region.contains("data-admin-only"), "regular upload surface must not be admin-gated");
        assert!(regular_region.contains("Destination"));
        assert!(regular_region.contains("data-upload-file"));
        assert!(regular_region.contains(">Upload</button>"));
    }

