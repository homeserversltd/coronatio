#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let port: u16 = env::var("CORONATIO_PORT")
        .ok()
        .and_then(|raw| raw.parse().ok())
        .unwrap_or(8090);
    let tab_root = env::var("CORONATIO_TAB_ROOT").unwrap_or_else(|_| DEFAULT_TAB_ROOT.to_string());
    let state = AppState {
        tab_root: Arc::new(PathBuf::from(tab_root)),
    };

    let app = app(state);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("bind coronatio listener");
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("serve coronatio");
}

fn app(state: AppState) -> Router {
    Router::new()
        .route("/", get(crown_shell_route))
        .route("/health", get(health_route))
        .route("/api", get(api_root_route))
        .route("/api/panes", get(panes_route))
        .route("/api/panes/:pane_id", get(pane_route))
        .route("/api/registry", get(registry_route))
        .route("/api/registry/transaction", get(registry_transaction_route))
        .route("/api/startup", get(startup_route))
        .route("/api/lanes", get(lane_policy_route))
        .route("/api/fallback", get(fallback_route))
        .route("/api/session", get(session_route))
        .route("/api/validatePin", post(homeserver_validate_pin_route))
        .route("/api/verifyPin", post(homeserver_validate_pin_route))
        .route("/api/logout", post(homeserver_logout_route))
        .route("/api/admin/ping", get(homeserver_admin_ping_route))
        .route("/api/admin/pin", post(homeserver_rust_mutation_route))
        .route(
            "/api/admin/session",
            get(session_route).post(session_renew_route),
        )
        .route("/api/caduceus/status", get(caduceus_status_route))
        .route(
            "/api/caduceus/update/check",
            post(caduceus_update_check_route),
        )
        .route("/api/caduceus/update/now", post(caduceus_update_now_route))
        .route(
            "/api/caduceus/receipts/latest",
            get(caduceus_receipts_latest_route),
        )
        .route("/api/topics", get(topics_route))
        .route("/api/monitor/pulse", get(monitor_pulse_route))
        .route("/api/services/data", get(service_data_route))
        .route("/api/frontend/storage", get(frontend_storage_route))
        .route("/api/themes", get(themes_route))
        .route("/api/favorites", get(favorites_route))
        .route("/api/get_starred_tab", get(get_starred_tab_route))
        .route("/api/set_starred_tab", post(set_starred_tab_route))
        .route("/api/boundary", get(boundary_route))
        .route("/api/installer", get(installer_route))
        .route("/api/stats/events", get(stats_events_route))
        .route("/api/stats/events/renew", post(stats_events_renew_route))
        .route("/api/stats", get(stats_route))
        .route("/api/faults", get(faults_route))
        .route("/admit/:tab_id", get(admit_tab_route))
        .route("/admit/upload/tree", get(upload_tree_fragment_route))
        .route(CROWN_HTMX_SCRIPT_PATH, get(crown_htmx_script_route))
        .route(CROWN_CHROME_SCRIPT_PATH, get(crown_chrome_script_route))
        .route("/api/tabs", get(tabs_route))
        .route("/api/tabs/:tab_id/manifest", get(tab_manifest_route))
        .merge(full_rust_route_table())
        .nest_service("/static", ServeDir::new(static_root()))
        .nest_service("/tabs", ServeDir::new((*state.tab_root).clone()))
        .fallback(route_boundary_fallback)
        .with_state(state)
}

fn static_root() -> PathBuf {
    env::var("CORONATIO_STATIC_ROOT")
        .ok()
        .map(PathBuf::from)
        .or_else(|| {
            let installed = PathBuf::from(INSTALLED_STATIC_ROOT);
            installed.exists().then_some(installed)
        })
        .unwrap_or_else(|| PathBuf::from(DEFAULT_STATIC_ROOT))
}
