    fn html_response_has_csp(response: &axum::http::Response<Body>, route: &str) {
        let csp = response
            .headers()
            .get(header::CONTENT_SECURITY_POLICY)
            .and_then(|value| value.to_str().ok())
            .unwrap_or("");
        assert!(csp.contains("script-src 'self'"), "{route} missing self-only script CSP: {csp}");
    }

    fn assert_no_hx_trigger_bracket_filters(source_name: &str, source: &str) {
        for fragment in source.split("hx-trigger=").skip(1) {
            let Some(quote) = fragment.chars().next() else { continue; };
            if quote != '"' && quote != '\'' { continue; }
            let trigger = fragment[1..].split(quote).next().unwrap_or("");
            assert!(!trigger.contains('['), "{source_name} uses CSP-forbidden hx-trigger bracket filter: {trigger}");
        }
    }

    fn fetch_url_at(source: &str, start: usize) -> Option<String> {
        let tail = &source[start + "fetch(".len()..];
        let quote = tail.chars().find(|ch| !ch.is_whitespace())?;
        if quote != '\'' && quote != '"' && quote != '`' { return Some("<dynamic>".to_string()); }
        Some(tail[1..].split(quote).next().unwrap_or("").to_string())
    }

    fn assert_fetch_allowlist(source_name: &str, source: &str) {
        let allowlist = [
            ("/api/status/power/usage", "PIN/session chrome header power state"),
            ("/api/status", "PIN/session chrome internet state"),
            ("/api/status/internet/speedtest", "PIN/session chrome speedtest action"),
            ("/api/themes", "PIN/session chrome theme bootstrap"),
            ("/api/validatePin", "PIN/session chrome"),
            ("/api/verifyPin", "upload PIN-gated submission verification"),
            ("/api/set_starred_tab", "PIN/session chrome favorite mutation"),
            ("/api/tabs/visibility", "PIN/session chrome visibility mutation"),
            ("/api/tabs/elements", "PIN/session chrome element visibility mutation"),
            ("/api/stats/elements", "PIN/session chrome stats element projection"),
            ("/api/portals/elements", "PIN/session chrome portal element projection"),
            ("/api/tab-bar", "PIN/session chrome tab-bar projection"),
            ("/api/logout", "PIN/session chrome logout invalidation"),
            ("/api/admin/ping", "PIN/session bootstrap authority validation"),
            ("/api/upload/history", "upload chrome modal read"),
            ("/api/upload/blacklist/list", "upload chrome modal read"),
            ("/api/upload/force-permissions", "upload chrome owned admin control"),
            ("/api/upload/default-directory", "upload chrome owned admin control"),
            ("/api/upload/blacklist/update", "upload chrome owned admin control"),
            ("/api/upload/history/clear", "upload chrome owned admin control"),
            ("/api/upload/pin-required-status", "upload chrome owned admin control"),
            ("/api/uptime", "PIN/session chrome uptime chip"),
            ("/api/stats", "Chart.js data bootstrap"),
            ("/api/network/notes", "Stats device-note read and admin edit"),
            ("/api/service/control", "owned portal chrome service action"),
            ("/api/portals", "owned portal chrome data bootstrap"),
            ("/api/portals/${encodeURIComponent(name)}", "owned portal chrome custom deletion"),
            ("/api/portals/factory", "owned portal chrome factory classification"),
            ("/api/portals/currentness", "owned portal chrome currentness readback"),
            ("/api/favorites", "PIN/session chrome favorite bootstrap"),
        ];
        let mut start = 0;
        while let Some(offset) = source[start..].find("fetch(") {
            let absolute = start + offset;
            let url = fetch_url_at(source, absolute).unwrap_or_else(|| "<parse-miss>".to_string());
            let allowed = allowlist.iter().any(|(allowed, _why)| url == *allowed || url == "<dynamic>");
            assert!(allowed, "{source_name} has fetch() outside documented owned chrome allowlist: {url}");
            start = absolute + "fetch(".len();
        }
        let xhr_count = source.matches("new XMLHttpRequest()").count();
        assert!(xhr_count <= 1, "{source_name} has non-upload XHR candidates: {xhr_count}");
    }

    #[test]
    fn flood_001_wall_fragment_refresh_does_not_reenter_admin_network_refresh() {
        let chrome = crown_chrome_js();
        assert!(chrome.contains("function applyAdminDomState()"), "admin DOM projection must be separated from network refresh");
        assert!(chrome.contains("refreshElementFragment('stats');"), "session change still refreshes stats fragment once");
        assert!(!chrome.contains("refreshElementFragment('portals');\n        if (selectedTab) showPane(selectedTab, { refresh: true });"), "session reconciliation must not eagerly refresh hidden Portals before its admission");
        for function_name in ["refreshElementFragment", "toggleElementVisibility"] {
            let start = chrome.find(&format!("async function {function_name}")).expect(function_name);
            let tail = &chrome[start..];
            let end = tail.find("\n    async function ").or_else(|| tail.find("\n    function ")).unwrap_or(tail.len());
            let body = &tail[..end];
            assert!(body.contains("applyAdminDomState();"), "{function_name} should reapply DOM-only admin visibility after swaps");
            assert!(!body.contains("setAdminMode(headerState.isAdmin)"), "{function_name} must not re-enter session/tab/fragment refresh graph");
        }
    }

    #[tokio::test(flavor = "current_thread")]
    async fn flood_001_wall_tab_bar_fragment_is_acyclic_no_load_trigger() {
        let router = app(AppState { tab_root: Arc::new(test_tab_root("flood-001-tab-bar-fragment")) });
        let response = router.oneshot(Request::builder().uri("/api/tab-bar?active=stats").body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let fragment = String::from_utf8(axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        assert!(fragment.contains("hx-get=\"/admit/stats\""), "fragment must preserve controller-driven pane admission: {fragment}");
        assert!(!fragment.contains("hx-trigger=\"load"), "served /api/tab-bar fragment must not reseat HTMX load triggers: {fragment}");
        assert!(render_crown_shell().contains("hx-trigger=\"immortal-floor-admit\""), "the Immortal Floor controller owns initial and repeated admission");
        assert!(!render_crown_shell().contains("hx-trigger=\"load, click\""), "page load must not bypass the BootFloor readiness gate");
    }

    #[tokio::test(flavor = "current_thread")]
    async fn hx_005_wall_staleness_admit_refetches_and_fragments_are_no_store() {
        let router = app(AppState { tab_root: Arc::new(test_tab_root("hx-005-staleness")) });
        let mut stats_admit_requests = 0;
        for route in ["/admit/stats", "/admit/stats"] {
            stats_admit_requests += 1;
            let response = router.clone().oneshot(Request::builder().uri(route).body(Body::empty()).unwrap()).await.unwrap();
            assert_eq!(response.status(), StatusCode::OK);
            assert_eq!(response.headers().get(header::CACHE_CONTROL).and_then(|value| value.to_str().ok()), Some("no-store"));
            html_response_has_csp(&response, route);
            let body = String::from_utf8(axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
            assert!(body.contains("data-stats-viewport") || body.contains("Chart"), "stats fragment body was not served: {body}");
        }
        assert_eq!(stats_admit_requests, 2, "test-server request count proves re-admission re-fetches the native stats pane");
        for pane in ["admin", "portals", "upload"] {
            let route = format!("/admit/{pane}");
            let response = router.clone().oneshot(Request::builder().uri(&route).body(Body::empty()).unwrap()).await.unwrap();
            assert_eq!(response.headers().get(header::CACHE_CONTROL).and_then(|value| value.to_str().ok()), Some("no-store"), "{route}");
            html_response_has_csp(&response, &route);
        }
    }

    #[tokio::test(flavor = "current_thread")]
    async fn hx_005_wall_dialect_served_markup_and_js_stay_htmx_or_owned_chrome_allowlist() {
        let _guard = HX_EXEMPLAR_ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
        let root = test_tab_root("hx-005-dialect-root");
        std::fs::create_dir_all(root.join("media")).unwrap();
        std::env::set_var("CORONATIO_UPLOAD_ROOT", &root);
        let router = app(AppState { tab_root: Arc::new(test_tab_root("hx-005-dialect")) });
        let mut served = vec![("shell".to_string(), render_crown_shell()), ("chrome".to_string(), crown_chrome_js())];
        for route in ["/admit/admin", "/admit/stats", "/admit/portals", "/admit/upload", "/admit/upload/tree"] {
            let response = router.clone().oneshot(Request::builder().uri(route).body(Body::empty()).unwrap()).await.unwrap();
            let body = String::from_utf8(axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
            served.push((route.to_string(), body));
        }
        for (name, source) in served {
            assert_no_hx_trigger_bracket_filters(&name, &source);
            assert_fetch_allowlist(&name, &source);
        }
        let shell = render_crown_shell();
        assert!(shell.contains("hx-get=\"/admit/stats\""));
        assert!(shell.contains("hx-get=\"/admit/upload/tree"));
        assert!(shell.contains("hx-post=\"/admit/admin/toggle/ssh-service\""));
        std::env::remove_var("CORONATIO_UPLOAD_ROOT");
    }

    #[tokio::test(flavor = "current_thread")]
    async fn hx_005_wall_script_inertness_sets_csp_on_documents_fragments_and_pins_htmx_sha() {
        let router = app(AppState { tab_root: Arc::new(test_tab_root("hx-005-script")) });
        for route in ["/", "/admit/admin", "/admit/stats", "/admit/portals", "/admit/upload", "/admit/upload/tree"] {
            let response = router.clone().oneshot(Request::builder().uri(route).body(Body::empty()).unwrap()).await.unwrap();
            html_response_has_csp(&response, route);
        }
        for (method, route) in [("POST", "/admit/admin/toggle/ssh-service"), ("POST", "/admit/admin/action/restart"), ("GET", "/admit/admin/action/view-logs")] {
            let response = router.clone().oneshot(Request::builder().method(method).uri(route).body(Body::empty()).unwrap()).await.unwrap();
            html_response_has_csp(&response, route);
        }
        let bytes = std::fs::read("static/vendor/htmx.min.js").unwrap();
        let sha = std::process::Command::new("sha256sum").arg("static/vendor/htmx.min.js").output().unwrap();
        let digest = String::from_utf8(sha.stdout).unwrap();
        assert!(!bytes.is_empty());
        assert!(digest.starts_with("71ea67185bfa8c98c39d31717c6fce5d852370fcdfd129db4543774d3145c0de"), "{digest}");
    }

    #[tokio::test(flavor = "current_thread")]
    async fn hx_005_wall_fault_typed_og_fragment_receipt_siblings_and_recovery() {
        let router = app(AppState { tab_root: Arc::new(test_tab_root("hx-005-fault")) });
        let fault = router.clone().oneshot(Request::builder().uri("/admit/missing-tab").body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(fault.status(), StatusCode::NOT_FOUND);
        assert_eq!(fault.headers().get("x-coronatio-fault").and_then(|value| value.to_str().ok()), Some("cartridge-fragment"));
        assert_eq!(fault.headers().get(header::CACHE_CONTROL).and_then(|value| value.to_str().ok()), Some("no-store"));
        html_response_has_csp(&fault, "/admit/missing-tab");
        let fault_body = String::from_utf8(axum::body::to_bytes(fault.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        assert!(fault_body.contains("data-cartridge-fault=\"true\""), "{fault_body}");
        assert!(fault_body.contains("data-cartridge-fault-kind=\"tab-not-found\""), "{fault_body}");
        assert!(fault_body.contains("class=\"card error-message\""), "{fault_body}");
        let sibling = router.clone().oneshot(Request::builder().uri("/admit/upload").body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(sibling.status(), StatusCode::OK);
        let sibling_body = String::from_utf8(axum::body::to_bytes(sibling.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        assert!(sibling_body.contains("data-upload-viewport"), "sibling upload pane unaffected: {sibling_body}");
        let recovered = router.clone().oneshot(Request::builder().uri("/admit/stats").body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(recovered.status(), StatusCode::OK);
        let faults = router.oneshot(Request::builder().uri("/api/faults").body(Body::empty()).unwrap()).await.unwrap();
        let faults_body = String::from_utf8(axum::body::to_bytes(faults.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        assert!(faults_body.contains("coronatio.cartridge-faults.v1"));
        assert!(faults_body.contains("missing-tab"));
        assert!(faults_body.contains("tabNotFound") || faults_body.contains("tab-not-found"), "{faults_body}");
    }

    #[tokio::test(flavor = "current_thread")]
    async fn hx_005_wall_mutation_honesty_rereads_state_and_refuses_non_admin_admin_mutations() {
        let _guard = HX_EXEMPLAR_ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
        let temp = test_tab_root("hx-005-mutation");
        let sshd = temp.join("sshd_config");
        std::fs::write(&sshd, "PasswordAuthentication no\n").unwrap();
        std::env::set_var("CORONATIO_SSHD_CONFIG_FIXTURE", &sshd);
        let _caduceus_guard = CADUCEUS_ENV_LOCK.get_or_init(|| std::sync::Mutex::new(())).lock().unwrap();
        std::env::set_var("CADUCEUS_URL", "http://127.0.0.1:9");
        let router = app(AppState { tab_root: Arc::new(test_tab_root("hx-005-mutation-app")) });
        let response = router.clone().oneshot(Request::builder().method("POST").uri("/admit/admin/toggle/ssh-password-authentication").header("X-Admin-Token", authorize_test_admin_token()).body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = String::from_utf8(axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
        assert!(body.contains("data-real-state=\"Disabled\""), "{body}");
        assert!(body.contains("sshd_config PasswordAuthentication readback"), "{body}");
        let source = std::fs::read_to_string("src/bands/caduceus.rs").unwrap();
        assert!(source.contains("admin_staff_intent(\"POST\", path, \"admin-service-toggle\")"));
        assert!(source.contains("render_admin_service_card_result_html(&toggle_id, Some(&result))"));
        for (method, route) in [("POST", "/admit/admin/toggle/ssh-password-authentication"), ("POST", "/admit/admin/toggle/ssh-service"), ("POST", "/admit/admin/toggle/samba-file-sharing"), ("POST", "/admit/admin/action/hard-drive-test"), ("POST", "/admit/admin/action/update"), ("POST", "/admit/admin/action/rotate-capability-key"), ("POST", "/admit/admin/action/restart"), ("POST", "/admit/admin/action/shutdown"), ("POST", "/admit/admin/action/restart-website"), ("POST", "/admit/admin/action/install-certificate")] {
            let response = router.clone().oneshot(Request::builder().method(method).uri(route).body(Body::empty()).unwrap()).await.unwrap();
            assert_eq!(response.status(), StatusCode::UNAUTHORIZED, "{route}");
            html_response_has_csp(&response, route);
            let body = String::from_utf8(axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap();
            assert!(body.contains("data-admin-membrane-refusal=\"true\""), "{route}: {body}");
            assert!(body.contains("admin-session-required"), "{route}: {body}");
        }
        std::env::remove_var("CORONATIO_SSHD_CONFIG_FIXTURE");
        std::env::remove_var("CADUCEUS_URL");
    }
