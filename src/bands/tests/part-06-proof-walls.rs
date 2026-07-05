    /// workflow-coronatio-viewport-fragment-payload-contract Proof boundaries:
    /// | Kill cartridge unit, activate its tab | Fault receipt; layer 0 visible; crown and siblings live |
    /// | Guest fragment carrying `<script>` | Script inert under CSP and swap config |
    /// | Fragment rendered under two themes | Guest markup follows operator skin via tokens |
    /// | Grep layer-1 sources for fetch dialects | HTMX vocabulary only |
    /// | Activate → leave → re-activate | Fresh fragment each activation; stream closed on leave |
    fn counted_http_response(status: u16, body: &'static str, max_requests: usize) -> (String, Arc<std::sync::atomic::AtomicUsize>) {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let thread_count = Arc::clone(&count);
        thread::spawn(move || {
            for _ in 0..max_requests {
                let Ok((mut stream, _)) = listener.accept() else { break; };
                thread_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                let mut buffer = [0_u8; 1024];
                let _ = stream.read(&mut buffer);
                let response = format!(
                    "HTTP/1.1 {status} Test\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                    body.len()
                );
                let _ = stream.write_all(response.as_bytes());
            }
        });
        (format!("http://{}", addr), count)
    }

    async fn body_text(response: axum::response::Response) -> String {
        String::from_utf8(axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap()
    }

    fn write_fragment_tab(root: &std::path::Path, id: &str, service_url: Option<&str>, fragment: Option<&str>) {
        let tab_dir = root.join(id);
        std::fs::create_dir_all(tab_dir.join("static")).unwrap();
        let service = service_url
            .map(|url| format!(",\n          \"serviceUrl\":\"{url}\""))
            .unwrap_or_default();
        std::fs::write(tab_dir.join("tab.json"), format!(r#"{{
          "id":"{id}",
          "title":"{id}",
          "routePrefix":"/api/tabs/{id}",
          "fragmentPath":"/fragment"{service}
        }}"#)).unwrap();
        if let Some(fragment) = fragment {
            std::fs::write(tab_dir.join("fragment"), fragment).unwrap();
        }
    }

    #[tokio::test]
    async fn wall_1_kill_cartridge_fault_preserves_underlay_and_sibling_admission() {
        let temp = test_tab_root("wall-1-kill-cartridge");
        let (one_shot, hits) = counted_http_response(200, "<article class=\"crown-fragment\" data-upstream=\"one\">one</article>", 1);
        write_fragment_tab(&temp, "kill-cartridge", Some(&one_shot), None);
        write_fragment_tab(&temp, "sibling-tab", None, Some("<article class=\"crown-fragment\" data-sibling=\"alive\">alive</article>"));
        let router = app(AppState { tab_root: Arc::new(temp) });

        let first = router.clone().oneshot(Request::builder().uri("/admit/kill-cartridge").body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(first.status(), StatusCode::OK);
        assert_eq!(hits.load(std::sync::atomic::Ordering::SeqCst), 1);
        let first_body = body_text(first).await;
        assert!(first_body.contains("data-upstream=\"one\""));

        let killed = router.clone().oneshot(Request::builder().uri("/admit/kill-cartridge").body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(killed.status(), StatusCode::BAD_GATEWAY);
        assert_eq!(killed.headers().get("x-coronatio-fault").unwrap(), "cartridge-fragment");
        assert_eq!(killed.headers().get(header::CACHE_CONTROL).unwrap(), "no-store");
        let killed_body = body_text(killed).await;
        assert!(killed_body.contains("data-cartridge-fault=\"true\""));
        assert!(killed_body.contains("data-cartridge-fault-kind=\"proxy-unreachable\""));

        let shell = router.clone().oneshot(Request::builder().uri("/").body(Body::empty()).unwrap()).await.unwrap();
        let shell = body_text(shell).await;
        assert!(shell.contains("data-crown-underlay=\"fallback\""));
        assert!(shell.contains("data-layer=\"0\""));
        assert!(shell.contains("data-view-panel=\"kill-cartridge\""));
        assert!(shell.contains("data-view-panel=\"sibling-tab\""));

        let sibling = router.clone().oneshot(Request::builder().uri("/admit/sibling-tab").body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(sibling.status(), StatusCode::OK);
        assert!(body_text(sibling).await.contains("data-sibling=\"alive\""));
        let faults = router.oneshot(Request::builder().uri("/api/faults").body(Body::empty()).unwrap()).await.unwrap();
        let faults = body_text(faults).await;
        assert!(faults.contains("kill-cartridge"));
        assert!(faults.contains("proxy-unreachable"));
    }

    #[tokio::test]
    async fn wall_2_script_inertness_csp_swap_config_and_iframe_sandbox_stand() {
        let temp = test_tab_root("wall-2-script-inertness");
        write_fragment_tab(&temp, "script-guest", None, Some("<article class=\"crown-fragment\"><script>window.__escaped = true</script><button hx-post=\"/api/noop\">Go</button></article>"));
        let iframe_dir = temp.join("same-origin-iframe").join("static");
        std::fs::create_dir_all(&iframe_dir).unwrap();
        std::fs::write(temp.join("same-origin-iframe").join("tab.json"), r#"{
          "id":"same-origin-iframe",
          "title":"Same Origin Iframe",
          "routePrefix":"/api/tabs/same-origin-iframe",
          "fragmentPath":"/static/index.html",
          "clientClass":"iframe"
        }"#).unwrap();
        std::fs::write(iframe_dir.join("index.html"), "<script>window.parent.document.body.dataset.escape='forbidden'</script>").unwrap();
        let router = app(AppState { tab_root: Arc::new(temp) });

        let shell = router.clone().oneshot(Request::builder().uri("/").body(Body::empty()).unwrap()).await.unwrap();
        let csp = shell.headers().get(header::CONTENT_SECURITY_POLICY).and_then(|value| value.to_str().ok()).unwrap();
        assert!(csp.contains("script-src 'self'"));
        let chrome = router.clone().oneshot(Request::builder().uri(CROWN_SHELL_SCRIPT_PATH).body(Body::empty()).unwrap()).await.unwrap();
        let chrome = body_text(chrome).await;
        assert!(chrome.contains("htmxOrgan.config.allowScriptTags = false"));

        let fragment = router.clone().oneshot(Request::builder().uri("/admit/script-guest").body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(fragment.status(), StatusCode::OK);
        assert_eq!(fragment.headers().get(header::CACHE_CONTROL).unwrap(), "no-store");
        let fragment = body_text(fragment).await;
        assert!(fragment.contains("<script>window.__escaped = true</script>"));
        assert!(fragment.contains("hx-post=\"/api/noop\""));

        let iframe = router.oneshot(Request::builder().uri("/admit/same-origin-iframe").body(Body::empty()).unwrap()).await.unwrap();
        let iframe = body_text(iframe).await;
        assert!(iframe.contains(r#"sandbox="allow-scripts allow-forms""#));
        assert!(!iframe.contains(r#"allow-same-origin"#));
    }

    #[test]
    fn wall_3_two_themes_change_ux_tokens_not_guest_markup() {
        let mut light = BTreeMap::new();
        insert_system_theme_tokens(&mut light);
        insert_mature_theme_tokens(&mut light, "light");
        let mut dark = BTreeMap::new();
        insert_system_theme_tokens(&mut dark);
        insert_mature_theme_tokens(&mut dark, "dark");
        assert_ne!(light.get("surface-0"), dark.get("surface-0"));
        assert_ne!(light.get("role-primary"), dark.get("role-primary"));
        assert!(light.keys().all(|key| !key.starts_with("--") || key.starts_with("--ux-")));

        let fragment = std::fs::read_to_string("tabs/inert-fragment/static/fragment.html").unwrap();
        assert!(fragment.contains("class=\"crown-fragment\""));
        assert!(fragment.contains("class=\"crown-fragment__button\""));
        assert!(!fragment.contains(" style="));
        assert!(!fragment.contains("--theme-"));
        assert!(!fragment.contains("#"));
        let light_render = fragment.clone();
        let dark_render = fragment.clone();
        assert_eq!(light_render, dark_render, "theme selection changes token values, not guest markup");
    }

    #[test]
    fn wall_4_layer_one_dialect_is_htmx_only_with_chrome_allowlist() {
        let mut layer_one_markup = vec![
            ("shell", render_crown_shell()),
            ("native-admin", render_native_pane_fragment(&native_crown_panes().into_iter().find(|pane| pane.id == "admin").unwrap())),
            ("native-stats", render_native_pane_fragment(&native_crown_panes().into_iter().find(|pane| pane.id == "stats").unwrap())),
            ("reference-fragment", std::fs::read_to_string("tabs/inert-fragment/static/fragment.html").unwrap()),
        ];
        let iframe_manifest = TabManifest {
            id: "iframe-guest".to_string(),
            title: "Iframe Guest".to_string(),
            description: String::new(),
            icon: String::new(),
            display_name: String::new(),
            order: 80,
            enabled: true,
            admin_only: false,
            visibility: TabVisibility::default(),
            data: serde_json::Value::Null,
            route_prefix: "/api/tabs/iframe-guest".to_string(),
            static_dir: "static".to_string(),
            service_url: None,
            health_route: None,
            fragment_path: "/static/index.html".to_string(),
            client_class: ClientClass::Iframe,
            install_mode: InstallMode::DynamicCartridge,
        };
        layer_one_markup.push(("iframe-admission", render_iframe_guest_fragment(&iframe_manifest)));

        let mut failures = Vec::new();
        for (name, markup) in &layer_one_markup {
            for forbidden in ["fetch(", "XMLHttpRequest", "WebSocket", "EventSource"] {
                if markup.contains(forbidden) {
                    failures.push(format!("{name} contains {forbidden}"));
                }
            }
            for trigger in markup.match_indices("hx-trigger=") {
                let tail = &markup[trigger.0..markup.len().min(trigger.0 + 96)];
                if tail.contains('[') {
                    failures.push(format!("{name} contains hx-trigger bracket filter near {tail}"));
                }
            }
        }
        for entry in std::fs::read_dir("tabs").unwrap() {
            let entry = entry.unwrap();
            let static_dir = entry.path().join("static");
            if !static_dir.exists() {
                continue;
            }
            for static_entry in std::fs::read_dir(static_dir).unwrap() {
                let path = static_entry.unwrap().path();
                if path.extension().is_some_and(|extension| extension == "html") {
                    let text = std::fs::read_to_string(&path).unwrap();
                    for forbidden in ["fetch(", "XMLHttpRequest", "WebSocket", "EventSource"] {
                        if text.contains(forbidden) {
                            failures.push(format!("{} contains {forbidden}", path.display()));
                        }
                    }
                }
            }
        }
        assert!(CROWN_SHELL_JS.contains("event.detail"), "chrome allowlist: HTMX event detail seam");
        assert!(CROWN_SHELL_JS.contains("XMLHttpRequest"), "chrome allowlist: typed htmx error response readback only");
        assert!(!CROWN_SHELL_JS.contains("fetch("), "crown chrome may not grow a parallel fetch dialect");
        assert!(!CROWN_SHELL_JS.contains("new WebSocket"));
        assert!(!CROWN_SHELL_JS.contains("new EventSource"));
        assert!(failures.is_empty(), "{}", failures.join("\n"));
    }

    #[test]
    fn coro_007_native_panes_are_composed_crown_blocks_with_details_readbacks() {
        let expected = [
            ("portals", "portal-card-row"),
            ("stats", "stat-workbench-grid"),
            ("admin", "status-strip-admin-cards"),
            ("upload", "single-ingress-card"),
            ("testtab", "token-lab"),
        ];
        for (pane_id, block_shape) in expected {
            let pane = native_crown_panes().into_iter().find(|pane| pane.id == pane_id).unwrap();
            let html = render_native_pane_fragment(&pane);
            assert!(html.contains("class=\"crown-fragment"), "{pane_id} missing crown fragment class");
            assert!(html.contains(&format!("data-crown-block-shape=\"{block_shape}\"")), "{pane_id} missing block shape {block_shape}");
            assert!(html.contains("data-crown-readback=\"true\""), "{pane_id} raw JSON not behind details");
            assert!(html.contains("data-native-readback=\"json\""), "{pane_id} raw JSON pre missing");
            assert!(!html.contains(" style="), "{pane_id} carries inline style");
            assert!(!html.trim_start().starts_with("<pre"), "{pane_id} is still pre-only");
        }
    }

    #[test]
    fn coro_007_body_scroll_is_contained_by_stage_and_iframe_husk_fills_stage() {
        assert!(CROWN_SHELL_CSS.contains("html, body { margin: 0; height: 100%; overflow: hidden"));
        assert!(CROWN_SHELL_CSS.contains(".crown-shell { height: 100vh"));
        assert!(CROWN_SHELL_CSS.contains(".crown-main { min-width: 0; min-height: 0"));
        assert!(CROWN_SHELL_CSS.contains(".crown-stage { position: relative; min-height: 0; height: 100%"));
        assert!(CROWN_SHELL_CSS.contains("overflow-y: auto"));
        assert!(CROWN_SHELL_CSS.contains(".crown-layer-one { position: relative; z-index: 1; min-height: 100%; height: 100%; display: grid"));
        assert!(CROWN_SHELL_CSS.contains(".crown-view-panel { min-height: 100%; height: 100%"));
        assert!(CROWN_SHELL_CSS.contains(".crown-iframe-guest { display: grid; grid-template-rows: auto minmax(0, 1fr); gap: var(--ux-space-3); min-height: 100%; height: 100%"));
        assert!(CROWN_SHELL_CSS.contains(".crown-iframe-guest__frame { width: 100%; height: 100%; min-height: 0"));
    }

    #[test]
    fn coro_007_portal_values_and_services_obey_chip_law_without_forced_word_breaks() {
        assert!(CROWN_SHELL_CSS.contains(".crown-definition-row { display: grid; grid-template-columns: minmax(5.5rem, .32fr) minmax(0, 1.35fr)"));
        assert!(CROWN_SHELL_CSS.contains(".crown-definition-row dd { margin: 0; color: var(--ux-text-strong); min-width: 0; overflow-wrap: normal; word-break: normal"));
        assert!(CROWN_SHELL_CSS.contains(".crown-chip { border: 1px solid var(--ux-outline); border-radius: var(--ux-radius-pill); padding: var(--ux-space-1) var(--ux-space-2); color: var(--ux-color-crown-bright); background: rgba(61, 220, 151, 0.12); font-size: var(--ux-type-small); overflow-wrap: normal; word-break: normal; white-space: nowrap"));

        let html = render_portals_fragment(PortalConfigResponse {
            schema: "coronatio.portals.config.v1".to_string(),
            route: "/api/portals".to_string(),
            success: true,
            source: "fixture".to_string(),
            factory_source: None,
            portals: vec![PortalEntry {
                name: "Media".to_string(),
                description: "Portal fixture".to_string(),
                services: vec!["transmissionPIA".to_string(), "calibre-web".to_string(), "calibre-simple-watch".to_string()],
                r#type: "systemd".to_string(),
                port: Some(9091),
                local_url: "http://media.home.arpa".to_string(),
                remote_url: None,
                status: None,
                visible: true,
            }],
            factory_portals: vec![],
            first_missing_signal: "none".to_string(),
        });
        for service in ["transmissionPIA", "calibre-web", "calibre-simple-watch"] {
            let marker = format!("data-portal-service=\"{service}\"");
            assert_eq!(html.matches(&marker).count(), 1, "service chip should render exactly once for {service}");
            assert!(html.contains(&format!("title=\"{service}\"")), "service chip should keep an ellipsis title for {service}");
        }
        assert!(!html.contains("transmissionPIA, calibre-web"), "services must not render as one comma-joined breakable text cell");
    }

    #[test]
    fn coro_007_shell_projects_selected_theme_and_underlay_recovery_posture() {
        let shell = render_crown_shell();
        assert!(shell.contains("data-theme="));
        assert!(shell.contains("data-crown-theme-projection=\"homeserver-json-default\""));
        assert!(shell.contains("--ux-surface-0:"));
        assert!(shell.contains("--ux-color-crown:"));
        assert!(shell.contains("Recovery posture"));
        assert!(shell.contains("data-underlay-startup-phase=\"app-ready\""));
        assert!(shell.contains("Service health"));
        assert!(shell.contains("data-underlay-fault-kind=\"none\""));
    }

    #[test]
    fn coro_007_testtab_is_live_ux_token_lab() {
        let pane = native_crown_panes().into_iter().find(|pane| pane.id == "testtab").unwrap();
        let html = render_native_pane_fragment(&pane);
        for token in ["--ux-surface-0", "--ux-color-crown", "--ux-color-leaf", "--ux-outline"] {
            assert!(html.contains(token), "missing token sample {token}");
        }
        assert!(html.contains("Live UX token swatches"));
        assert!(html.contains("Type scale"));
        assert!(html.contains("Radius"));
        assert!(!html.contains("style="));
    }

    #[tokio::test]
    async fn wall_5_reactivation_refetches_upstream_and_admission_is_no_store() {
        let temp = test_tab_root("wall-5-staleness");
        let (upstream, hits) = counted_http_response(200, "<article class=\"crown-fragment\" data-fresh=\"true\">fresh</article>", 2);
        write_fragment_tab(&temp, "fresh-tab", Some(&upstream), None);
        let router = app(AppState { tab_root: Arc::new(temp) });
        for activation in 1..=2 {
            let response = router.clone().oneshot(Request::builder().uri("/admit/fresh-tab").body(Body::empty()).unwrap()).await.unwrap();
            assert_eq!(response.status(), StatusCode::OK, "activation {activation}");
            assert_eq!(response.headers().get(header::CACHE_CONTROL).unwrap(), "no-store");
            assert!(body_text(response).await.contains("data-fresh=\"true\""));
        }
        assert_eq!(hits.load(std::sync::atomic::Ordering::SeqCst), 2, "every activation hits upstream");
    }
