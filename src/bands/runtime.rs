use tracing::{
    field::{Field, Visit},
    span::{Attributes, Id},
    Event, Level, Subscriber,
};
use tracing_subscriber::{
    layer::{Context, SubscriberExt},
    registry::LookupSpan,
    util::SubscriberInitExt,
    Layer,
};

#[derive(Default)]
struct HyalosMessageVisitor {
    message: Option<String>,
}

impl Visit for HyalosMessageVisitor {
    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" {
            self.message = Some(format!("{value:?}"));
        }
    }

    fn record_str(&mut self, field: &Field, value: &str) {
        if field.name() == "message" {
            self.message = Some(value.to_string());
        }
    }
}

struct HyalosTracingLayer;

impl<S> Layer<S> for HyalosTracingLayer
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    fn on_event(&self, event: &Event<'_>, _context: Context<'_, S>) {
        let metadata = event.metadata();
        if *metadata.level() > Level::INFO {
            return;
        }
        let mut fields = HyalosMessageVisitor::default();
        event.record(&mut fields);
        caduceus_hyalos_reflect_best_effort(
            "tracing-event",
            metadata.level().as_str().to_ascii_lowercase(),
            fields.message.unwrap_or_else(|| metadata.name().to_string()),
        );
    }

    fn on_new_span(&self, attributes: &Attributes<'_>, _id: &Id, _context: Context<'_, S>) {
        let metadata = attributes.metadata();
        if *metadata.level() > Level::INFO {
            return;
        }
        caduceus_hyalos_reflect_best_effort(
            "tracing-span",
            metadata.level().as_str().to_ascii_lowercase(),
            metadata.name().to_string(),
        );
    }
}

#[tokio::main]
async fn main() {
    let filter = tracing_subscriber::EnvFilter::builder()
        .with_default_directive(tracing_subscriber::filter::LevelFilter::INFO.into())
        .from_env_lossy();
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().with_filter(filter))
        .with(HyalosTracingLayer.with_filter(tracing_subscriber::filter::LevelFilter::INFO))
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
    pulse::ensure_stats_ticker_started();
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
        .route("/api/v1/attendance/open", post(caduceus_attendance_open_route))
        .route("/api/v1/attendance/validate", post(caduceus_attendance_validate_route))
        .route("/api/v1/attendance/touch", post(caduceus_attendance_touch_route))
        .route("/api/v1/attendance/change-pin", post(caduceus_attendance_change_pin_route))
        .route("/api/v1/attendance/invalidate", post(caduceus_attendance_invalidate_route))
        .route("/api/caduceus/status", get(caduceus_status_route))
        .route(
            "/api/caduceus/update/check",
            post(caduceus_update_check_route),
        )
        .route("/api/caduceus/update/now", post(caduceus_update_now_route))
        .route("/api/caduceus/keyman/create-key", post(caduceus_keyman_create_key_route))
        .route("/api/caduceus/keyman/update-key", post(caduceus_keyman_update_key_route))
        .route("/api/caduceus/keyman/admin-password", post(caduceus_keyman_admin_password_route))
        .route("/api/caduceus/keyman/key-status", post(caduceus_keyman_key_status_route))
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
        .route("/api/tab-bar", get(tab_bar_fragment_route))
        .route("/api/boundary", get(boundary_route))
        .route("/api/installer", get(installer_route))
        .route("/api/core/pulse", get(indicators::core_pulse_route))
        .route("/api/core/pulse/renew", post(indicators::core_pulse_renew_route))
        .route("/api/core/pulse/upgrade", post(indicators::core_pulse_upgrade_route))
        .route("/api/core/pulse/downgrade", post(indicators::core_pulse_downgrade_route))
        .route("/api/stats/pulse", get(pulse::stats_pulse_route))
        .route("/api/stats/pulse/renew", post(pulse::stats_pulse_renew_route))
        .route("/api/stats/pulse/upgrade", post(pulse::stats_pulse_upgrade_route))
        .route("/api/stats/pulse/downgrade", post(pulse::stats_pulse_downgrade_route))
        .route("/api/stats", get(stats_route))
        .route("/api/faults", get(faults_route))
        .route("/admit/:tab_id", get(admit_tab_route))
        .route("/admit/admin/service/:toggle_id", get(admin_service_card_fragment_route))
        .route("/admit/admin/toggle/:toggle_id", post(admin_toggle_fragment_route))
        .route("/admit/admin/action/view-logs", get(admin_logs_fragment_route))
        .route("/admit/admin/action/view-logs-clear", post(admin_logs_clear_fragment_route))
        .route("/admit/admin/action/:action_id", post(admin_action_fragment_route).get(admin_action_fragment_route))
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
