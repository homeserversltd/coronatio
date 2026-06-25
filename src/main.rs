use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{env, net::SocketAddr, path::PathBuf, sync::Arc};
use tokio::fs;
use tower_http::services::ServeDir;

const DEFAULT_TAB_ROOT: &str = "/var/lib/coronatio/tabs";
const PRIMARY_TABS: [&str; 4] = ["admin", "stats", "portals", "upload"];

#[derive(Clone)]
struct AppState {
    tab_root: Arc<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct CoronatioRoot {
    schema: String,
    kind: String,
    product: String,
    routes: Vec<String>,
    tab_root: String,
    primary_tabs: Vec<String>,
    first_party_panes: Vec<CrownPane>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct CrownPane {
    id: String,
    title: String,
    role: String,
    summary: String,
    order: i64,
    admin_only: bool,
    install_mode: InstallMode,
    route: String,
    state_route: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct TabManifest {
    id: String,
    title: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    icon: String,
    #[serde(default)]
    order: i64,
    #[serde(default)]
    admin_only: bool,
    #[serde(default)]
    route_prefix: String,
    #[serde(default)]
    static_dir: String,
    #[serde(default)]
    service_url: Option<String>,
    #[serde(default)]
    health_route: Option<String>,
    #[serde(default)]
    install_mode: InstallMode,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
enum InstallMode {
    #[default]
    DynamicCartridge,
    SourceInjectionRecompile,
    FirstPartyNative,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct TabList {
    schema: String,
    tab_root: String,
    native_panes: Vec<CrownPane>,
    tabs: Vec<TabManifest>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct StatsSnapshot {
    schema: String,
    pane_id: String,
    product: String,
    transport: StatsTransport,
    telemetry: StatsTelemetry,
    next_routes: StatsNextRoutes,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct StatsTransport {
    snapshot_route: String,
    event_route: String,
    renew_route: String,
    stream_status: String,
    stream_reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct StatsTelemetry {
    load1: Option<f64>,
    cpu_temperature_celsius: Option<f64>,
    service_health: Option<String>,
    storage_posture: Option<String>,
    first_missing_signal: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct StatsNextRoutes {
    snapshot: String,
    events: String,
    renew: String,
}

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
        .route("/api/stats", get(stats_route))
        .route("/api/tabs", get(tabs_route))
        .route("/api/tabs/:tab_id/manifest", get(tab_manifest_route))
        .nest_service("/tabs", ServeDir::new((*state.tab_root).clone()))
        .with_state(state)
}

async fn crown_shell_route() -> impl IntoResponse {
    Html(render_crown_shell())
}

async fn health_route() -> impl IntoResponse {
    Json(serde_json::json!({
        "ok": true,
        "service": "coronatio",
        "schema": "coronatio.health.v1"
    }))
}

async fn api_root_route(State(state): State<AppState>) -> impl IntoResponse {
    Json(CoronatioRoot {
        schema: "coronatio.api.root.v1".to_string(),
        kind: "coronatio-root".to_string(),
        product: "Coronatio".to_string(),
        routes: vec![
            "/".to_string(),
            "/health".to_string(),
            "/api".to_string(),
            "/api/panes".to_string(),
            "/api/panes/:pane_id".to_string(),
            "/api/stats".to_string(),
            "/api/tabs".to_string(),
            "/api/tabs/:tab_id/manifest".to_string(),
            "/tabs/<tab-id>/static/...".to_string(),
        ],
        tab_root: state.tab_root.display().to_string(),
        primary_tabs: PRIMARY_TABS.iter().map(|tab| (*tab).to_string()).collect(),
        first_party_panes: native_crown_panes(),
    })
}

async fn panes_route() -> impl IntoResponse {
    Json(serde_json::json!({
        "schema": "coronatio.panes.v1",
        "product": "Coronatio",
        "panes": native_crown_panes()
    }))
}

async fn stats_route() -> impl IntoResponse {
    Json(stats_snapshot())
}

async fn pane_route(Path(pane_id): Path<String>) -> impl IntoResponse {
    if !is_safe_tab_id(&pane_id) {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "invalid pane id"})),
        )
            .into_response();
    }

    match native_crown_panes()
        .into_iter()
        .find(|pane| pane.id == pane_id)
    {
        Some(pane) => Json(pane).into_response(),
        None => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "pane not found"})),
        )
            .into_response(),
    }
}

async fn tabs_route(State(state): State<AppState>) -> impl IntoResponse {
    match load_tab_manifests(&state.tab_root).await {
        Ok(tabs) => Json(TabList {
            schema: "coronatio.tabs.v1".to_string(),
            tab_root: state.tab_root.display().to_string(),
            native_panes: native_crown_panes(),
            tabs,
        })
        .into_response(),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": error.to_string()})),
        )
            .into_response(),
    }
}

async fn tab_manifest_route(
    State(state): State<AppState>,
    Path(tab_id): Path<String>,
) -> impl IntoResponse {
    if !is_safe_tab_id(&tab_id) {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "invalid tab id"})),
        )
            .into_response();
    }

    match load_tab_manifest(&state.tab_root, &tab_id).await {
        Ok(Some(manifest)) => Json(manifest).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "tab not found"})),
        )
            .into_response(),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": error.to_string()})),
        )
            .into_response(),
    }
}

async fn load_tab_manifests(tab_root: &PathBuf) -> Result<Vec<TabManifest>, std::io::Error> {
    let mut tabs = Vec::new();
    let mut entries = match fs::read_dir(tab_root).await {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(tabs),
        Err(error) => return Err(error),
    };

    while let Some(entry) = entries.next_entry().await? {
        if !entry.file_type().await?.is_dir() {
            continue;
        }
        let tab_id = entry.file_name().to_string_lossy().to_string();
        if !is_safe_tab_id(&tab_id) {
            continue;
        }
        if let Some(manifest) = load_tab_manifest(tab_root, &tab_id).await? {
            tabs.push(manifest);
        }
    }
    tabs.sort_by(|left, right| left.order.cmp(&right.order).then(left.id.cmp(&right.id)));
    Ok(tabs)
}

async fn load_tab_manifest(
    tab_root: &PathBuf,
    tab_id: &str,
) -> Result<Option<TabManifest>, std::io::Error> {
    let manifest_path = tab_root.join(tab_id).join("tab.json");
    let raw = match fs::read_to_string(manifest_path).await {
        Ok(raw) => raw,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(error),
    };
    let manifest = serde_json::from_str::<TabManifest>(&raw)
        .map_err(|error| std::io::Error::new(std::io::ErrorKind::InvalidData, error))?;
    if manifest.id != tab_id || !is_safe_tab_id(&manifest.id) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "tab id mismatch or unsafe tab id",
        ));
    }
    Ok(Some(manifest))
}

fn native_crown_panes() -> Vec<CrownPane> {
    vec![
        CrownPane {
            id: "admin".to_string(),
            title: "Admin".to_string(),
            role: "session and crown policy".to_string(),
            summary: "PIN depth, capability posture, install receipts, and controlled mutation entrypoints.".to_string(),
            order: 0,
            admin_only: true,
            install_mode: InstallMode::FirstPartyNative,
            route: "/#admin".to_string(),
            state_route: "/api/panes/admin".to_string(),
        },
        CrownPane {
            id: "stats".to_string(),
            title: "Stats".to_string(),
            role: "machine telemetry".to_string(),
            summary: "System load, service health, storage posture, and later SSE live readback.".to_string(),
            order: 10,
            admin_only: false,
            install_mode: InstallMode::FirstPartyNative,
            route: "/#stats".to_string(),
            state_route: "/api/stats".to_string(),
        },
        CrownPane {
            id: "portals".to_string(),
            title: "Portals".to_string(),
            role: "service ingress".to_string(),
            summary: "Admitted HOMESERVER services, local ingress, remote ingress, and service currentness.".to_string(),
            order: 20,
            admin_only: false,
            install_mode: InstallMode::FirstPartyNative,
            route: "/#portals".to_string(),
            state_route: "/api/panes/portals".to_string(),
        },
        CrownPane {
            id: "upload".to_string(),
            title: "Upload".to_string(),
            role: "file ingress".to_string(),
            summary: "Safe file admission into HOMESERVER storage with policy and receipt readbacks.".to_string(),
            order: 30,
            admin_only: false,
            install_mode: InstallMode::FirstPartyNative,
            route: "/#upload".to_string(),
            state_route: "/api/panes/upload".to_string(),
        },
    ]
}

fn stats_snapshot() -> StatsSnapshot {
    StatsSnapshot {
        schema: "coronatio.stats.snapshot.v1".to_string(),
        pane_id: "stats".to_string(),
        product: "Coronatio".to_string(),
        transport: StatsTransport {
            snapshot_route: "/api/stats".to_string(),
            event_route: "/api/stats/events".to_string(),
            renew_route: "/api/stats/events/renew".to_string(),
            stream_status: "planned".to_string(),
            stream_reason: "stats SSE lease route is the next Coronatio tranche; snapshot is the current authority".to_string(),
        },
        telemetry: StatsTelemetry {
            load1: None,
            cpu_temperature_celsius: None,
            service_health: None,
            storage_posture: None,
            first_missing_signal: "stats collectors not wired".to_string(),
        },
        next_routes: StatsNextRoutes {
            snapshot: "/api/stats".to_string(),
            events: "/api/stats/events".to_string(),
            renew: "/api/stats/events/renew".to_string(),
        },
    }
}

fn render_crown_shell() -> String {
    let nav = native_crown_panes()
        .into_iter()
        .map(|pane| {
            format!(
                r##"<a class="crown-tab" href="#{id}" data-pane="{id}">{title}</a>"##,
                id = pane.id,
                title = pane.title
            )
        })
        .collect::<Vec<_>>()
        .join("");

    format!(
        r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Coronatio</title>
  <style>
    :root {{ color-scheme: dark; font-family: Inter, ui-sans-serif, system-ui, sans-serif; background: #070a12; color: #edf2ff; }}
    body {{ margin: 0; min-height: 100vh; background: radial-gradient(circle at top, #17233a 0, #070a12 42rem); }}
    .crown-shell {{ display: grid; grid-template-columns: 14rem 1fr; min-height: 100vh; }}
    .crown-rail {{ border-right: 1px solid rgba(255,255,255,.12); padding: 1rem; background: rgba(3,6,12,.72); }}
    .crown-mark {{ font-weight: 800; letter-spacing: .08em; text-transform: uppercase; margin-bottom: 1rem; }}
    .crown-tab {{ display: block; color: #dbe7ff; text-decoration: none; padding: .75rem .85rem; border-radius: .75rem; margin-bottom: .35rem; background: rgba(255,255,255,.055); }}
    .crown-tab:hover {{ background: rgba(125,166,255,.18); }}
    .crown-stage {{ padding: 1.25rem; }}
    .crown-hero {{ border: 1px solid rgba(255,255,255,.12); border-radius: 1.1rem; padding: 1.1rem; background: rgba(8,13,25,.78); box-shadow: 0 1.5rem 4rem rgba(0,0,0,.28); }}
    .crown-grid {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(13rem, 1fr)); gap: .8rem; margin-top: 1rem; }}
    .crown-card {{ border: 1px solid rgba(255,255,255,.10); border-radius: .95rem; padding: .95rem; background: rgba(255,255,255,.045); }}
    .chip {{ display: inline-block; border: 1px solid rgba(125,166,255,.35); border-radius: 999px; padding: .18rem .55rem; font-size: .78rem; color: #aac3ff; }}
  </style>
</head>
<body>
  <main class="crown-shell" data-product="Coronatio">
    <nav class="crown-rail" aria-label="Coronatio primary tabs">
      <div class="crown-mark">Coronatio</div>
      {nav}
    </nav>
    <section class="crown-stage">
      <div class="crown-hero">
        <span class="chip">HOMESERVER Rust crown</span>
        <h1>Coronatio crown shell</h1>
        <p>First-party panes are native Rust crown law. Installed services enter through governed cartridges or source-injection recompiles.</p>
        <div class="crown-grid">
          <article class="crown-card"><strong>Admin</strong><br>Session, capability, and install authority.</article>
          <article class="crown-card"><strong>Stats</strong><br>Machine readback and live telemetry lane.</article>
          <article class="crown-card"><strong>Portals</strong><br>Admitted service ingress and currentness.</article>
          <article class="crown-card"><strong>Upload</strong><br>Safe file ingress with receipts.</article>
        </div>
      </div>
    </section>
  </main>
</body>
</html>"#
    )
}

fn is_safe_tab_id(tab_id: &str) -> bool {
    !tab_id.is_empty()
        && tab_id
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
}

async fn shutdown_signal() {
    let _ = tokio::signal::ctrl_c().await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    #[test]
    fn tab_ids_are_forward_safe() {
        for tab_id in PRIMARY_TABS {
            assert!(is_safe_tab_id(tab_id));
        }
        assert!(is_safe_tab_id("backblaze-tab"));
        assert!(!is_safe_tab_id("../escape"));
        assert!(!is_safe_tab_id("CamelCase"));
        assert!(!is_safe_tab_id(""));
    }

    #[test]
    fn native_panes_are_lawful_crown_tabs() {
        let panes = native_crown_panes();
        let ids: Vec<_> = panes.iter().map(|pane| pane.id.as_str()).collect();
        assert_eq!(ids, PRIMARY_TABS);
        assert!(panes
            .iter()
            .all(|pane| pane.install_mode == InstallMode::FirstPartyNative));
        assert!(panes
            .iter()
            .any(|pane| pane.admin_only && pane.id == "admin"));
    }

    #[tokio::test]
    async fn api_root_names_coronatio_not_arcadia() {
        let temp = test_tab_root("api-root");
        let response = app(AppState {
            tab_root: Arc::new(temp),
        })
        .oneshot(Request::builder().uri("/api").body(Body::empty()).unwrap())
        .await
        .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let body = String::from_utf8(bytes.to_vec()).unwrap();
        assert!(body.contains("coronatio.api.root.v1"));
        assert!(body.contains("Coronatio"));
        assert!(!body.contains("Arcadia"));
    }

    #[tokio::test]
    async fn api_root_declares_lawful_primary_tabs() {
        let temp = test_tab_root("primary-tabs");
        let response = app(AppState {
            tab_root: Arc::new(temp),
        })
        .oneshot(Request::builder().uri("/api").body(Body::empty()).unwrap())
        .await
        .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let root: CoronatioRoot = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(root.primary_tabs, ["admin", "stats", "portals", "upload"]);
        assert_eq!(root.first_party_panes.len(), 4);
    }

    #[tokio::test]
    async fn panes_route_exposes_first_party_crown_shell() {
        let temp = test_tab_root("panes");
        let response = app(AppState {
            tab_root: Arc::new(temp),
        })
        .oneshot(
            Request::builder()
                .uri("/api/panes")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let body = String::from_utf8(bytes.to_vec()).unwrap();
        assert!(body.contains("coronatio.panes.v1"));
        assert!(body.contains("first-party-native"));
        assert!(body.contains("Admin"));
        assert!(body.contains("Stats"));
        assert!(body.contains("Portals"));
        assert!(body.contains("Upload"));
        assert!(!body.contains("YouTube"));
    }

    #[tokio::test]
    async fn crown_shell_renders_primary_tabs_without_platform_brand_nav() {
        let temp = test_tab_root("shell");
        let response = app(AppState {
            tab_root: Arc::new(temp),
        })
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let body = String::from_utf8(bytes.to_vec()).unwrap();
        assert!(body.contains("data-product=\"Coronatio\""));
        assert!(body.contains("data-pane=\"admin\""));
        assert!(body.contains("data-pane=\"stats\""));
        assert!(body.contains("data-pane=\"portals\""));
        assert!(body.contains("data-pane=\"upload\""));
        assert!(body.contains("HOMESERVER Rust crown"));
        assert!(!body.contains("Arcadia"));
        assert!(!body.contains("YouTube"));
    }

    #[tokio::test]
    async fn loads_dynamic_cartridge_manifests_without_recompile() {
        let temp = test_tab_root("dynamic-tabs");
        let tab_dir = temp.join("service-card");
        std::fs::create_dir_all(&tab_dir).unwrap();
        std::fs::write(
            tab_dir.join("tab.json"),
            r#"{
              "id":"service-card",
              "title":"Service Card",
              "order":90,
              "adminOnly":true,
              "routePrefix":"/api/tabs/service-card",
              "staticDir":"static",
              "serviceUrl":"http://127.0.0.1:9910",
              "healthRoute":"/health",
              "installMode":"dynamic-cartridge"
            }"#,
        )
        .unwrap();

        let response = app(AppState {
            tab_root: Arc::new(temp),
        })
        .oneshot(
            Request::builder()
                .uri("/api/tabs")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let list: TabList = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(list.native_panes.len(), 4);
        assert_eq!(list.tabs.len(), 1);
        assert_eq!(list.tabs[0].id, "service-card");
        assert_eq!(list.tabs[0].install_mode, InstallMode::DynamicCartridge);
    }

    #[tokio::test]
    async fn stats_snapshot_is_honest_first_party_readback() {
        let temp = test_tab_root("stats-snapshot");
        let response = app(AppState {
            tab_root: Arc::new(temp),
        })
        .oneshot(
            Request::builder()
                .uri("/api/stats")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let snapshot: StatsSnapshot = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(snapshot.schema, "coronatio.stats.snapshot.v1");
        assert_eq!(snapshot.pane_id, "stats");
        assert_eq!(snapshot.product, "Coronatio");
        assert_eq!(snapshot.transport.snapshot_route, "/api/stats");
        assert_eq!(snapshot.transport.event_route, "/api/stats/events");
        assert_eq!(snapshot.transport.renew_route, "/api/stats/events/renew");
        assert_eq!(snapshot.transport.stream_status, "planned");
        assert_eq!(snapshot.telemetry.load1, None);
        assert_eq!(snapshot.telemetry.cpu_temperature_celsius, None);
        assert_eq!(
            snapshot.telemetry.first_missing_signal,
            "stats collectors not wired"
        );
    }

    #[test]
    fn stats_native_pane_points_to_stats_snapshot_route() {
        let stats = native_crown_panes()
            .into_iter()
            .find(|pane| pane.id == "stats")
            .unwrap();
        assert_eq!(stats.state_route, "/api/stats");
    }

    fn test_tab_root(name: &str) -> PathBuf {
        let root =
            std::env::temp_dir().join(format!("coronatio-test-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).unwrap();
        root
    }
}
