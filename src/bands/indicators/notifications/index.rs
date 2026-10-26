const NOTIFICATIONS_TOPIC: &str = "notifications.status";
const NOTIFICATIONS_PATH: &str = "/api/v1/notifications";
const NOTIFICATIONS_TTL: Duration = Duration::from_secs(30);
const NOTIFICATIONS_ICON: &str = r#"<svg class="indicator-icon" viewBox="0 0 24 24" aria-hidden="true"><path d="M12 2a2 2 0 0 1 2 2v.3A7 7 0 0 1 19 11v5l2 2v1H3v-1l2-2v-5a7 7 0 0 1 5-6.7V4a2 2 0 0 1 2-2zm0 4a5 5 0 0 0-5 5v5.8L6.8 17h10.4l-.2-.2V11a5 5 0 0 0-5-5zm-2 14h4a2 2 0 0 1-4 0z"/></svg>"#;

#[derive(Clone, PartialEq, Eq)]
struct NotificationView {
    id: String,
    kind: String,
    state: String,
    title: String,
    hostname: String,
    names: Vec<String>,
}

#[derive(Clone, PartialEq, Eq)]
struct NotificationsView {
    items: Vec<NotificationView>,
    available: bool,
}

struct NotificationsCache {
    observed: Option<(Instant, NotificationsView)>,
    revision: u64,
}

static NOTIFICATIONS_CACHE: OnceLock<Mutex<NotificationsCache>> = OnceLock::new();

fn notifications_cache() -> &'static Mutex<NotificationsCache> {
    NOTIFICATIONS_CACHE.get_or_init(|| Mutex::new(NotificationsCache { observed: None, revision: 0 }))
}

// One bounded, recomputable view, never the upstream body or a notification ledger.
// The lock coalesces simultaneous viewers; the Caduceus client bounds I/O and bytes.
fn notifications_snapshot(invalidate: bool) -> (NotificationsView, u64) {
    let mut cache = notifications_cache().lock().unwrap();
    if !invalidate {
        if let Some((at, view)) = &cache.observed {
            if at.elapsed() < NOTIFICATIONS_TTL {
                return (view.clone(), cache.revision);
            }
        }
    }
    let readback = caduceus_http("GET", NOTIFICATIONS_PATH);
    let view = notifications_view(&readback);
    if cache.observed.as_ref().is_none_or(|(_, prior)| prior != &view) {
        cache.revision = cache.revision.wrapping_add(1);
    }
    cache.observed = Some((Instant::now(), view.clone()));
    (view, cache.revision)
}

fn notifications_view(readback: &CaduceusHttpReadback) -> NotificationsView {
    let quiet = || NotificationsView { items: Vec::new(), available: true };
    let unavailable = || NotificationsView { items: Vec::new(), available: false };
    if matches!(readback.status, 404 | 405 | 501) {
        return quiet();
    }
    if !readback.ok { return unavailable(); }
    if readback.body.get("schema").and_then(serde_json::Value::as_str)
        .is_some_and(|schema| schema != "caduceus.notifications.v1") {
        return unavailable();
    }
    // Missing optional fields select the primitive empty view; unknown fields are ignored.
    let Some(rows) = readback.body.get("notifications").and_then(serde_json::Value::as_array) else {
        return quiet();
    };
    if rows.len() > 256 { return unavailable(); }
    let mut items = Vec::new();
    for row in rows.iter().filter(|row| row.is_object()) {
        let text = |key: &str| row.get(key).and_then(serde_json::Value::as_str).unwrap_or("").to_owned();
        let names = row.get("names").and_then(serde_json::Value::as_array);
        if names.is_some_and(|names| names.len() > 64) { return unavailable(); }
        items.push(NotificationView {
            id: text("id"), kind: text("kind"), state: text("state"), title: text("title"),
            hostname: row.pointer("/from/hostname").and_then(serde_json::Value::as_str).unwrap_or("").to_owned(),
            names: names.into_iter().flatten().filter_map(serde_json::Value::as_str).map(str::to_owned).collect(),
        });
    }
    NotificationsView { items, available: true }
}

pub(crate) fn notifications_manifest() -> IndicatorManifest {
    IndicatorManifest {
        id: "notifications", topic_id: NOTIFICATIONS_TOPIC, order: 70, title: "Notifications",
        icon_id: "bell", initial_state: "quiet", admin_interactive: true,
        render_indicator: render_notifications_indicator, render_modal: render_notifications_modal,
        collector: Some(notifications_collector),
    }
}

fn render_notifications_indicator(_ctx: IndicatorRenderContext) -> String {
    format!(r#"<button type="button" class="indicator notifications-indicator" data-indicator="notifications" data-modal-kind="notifications" data-modal-title="Notifications" aria-label="Notifications" title="Notifications" hx-get="/api/indicators/notifications/badge" hx-trigger="load, notifications.status from:body" hx-swap="innerHTML">{NOTIFICATIONS_ICON}</button>"#)
}

fn render_notifications_modal(_ctx: IndicatorRenderContext) -> String {
    r#"<div class="notifications-modal" data-modal-kind-body="notifications" data-indicator-fragment hx-get="/api/indicators/notifications" hx-trigger="load" hx-swap="outerHTML"><p>No notifications</p></div>"#.to_owned()
}

fn notifications_badge(view: &NotificationsView) -> String {
    let count = view.items.len();
    if count == 0 { return NOTIFICATIONS_ICON.to_owned(); }
    format!(r#"{NOTIFICATIONS_ICON}<span class="notifications-count" data-notifications-count aria-label="{count} notifications" aria-live="polite">{count}</span>"#)
}

fn notifications_collector(_session: Session) -> Result<serde_json::Value, String> {
    let (view, revision) = notifications_snapshot(false);
    Ok(serde_json::json!({"count":view.items.len(),"available":view.available,"revision":revision}))
}

fn notification_segment(id: &str) -> String {
    let mut encoded = String::new();
    for byte in id.bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b'~') {
            encoded.push(char::from(byte));
        } else {
            encoded.push_str(&format!("%{byte:02X}"));
        }
    }
    encoded
}

fn notifications_modal_html(
    view: &NotificationsView,
    session: Session,
    notice: Option<(&str, bool)>,
    edited: Option<(&str, &[String])>,
) -> String {
    let mut html = String::from(r#"<div class="notifications-modal" data-modal-kind-body="notifications" data-indicator-fragment>"#);
    if let Some((message, ok)) = notice {
        html.push_str(&format!(r#"<p class="notifications-result" data-notifications-result data-ok="{ok}" role="status">{}</p>"#, html_escape(message)));
    }
    if view.items.is_empty() {
        html.push_str(if view.available { "<p>No notifications</p>" } else { "<p>Notifications are unavailable right now</p>" });
    }
    for item in &view.items {
        let title = if item.title.is_empty() { "Notification" } else { &item.title };
        html.push_str(&format!(r#"<article class="notification-request" data-notification-id="{}"><strong>{}</strong><p class="notification-from">{}</p>"#,
            html_escape(&item.id), html_escape(title), html_escape(&item.hostname)));
        if session == Session::Admin && item.kind == "house-link-up" && item.state == "pending" && !item.id.is_empty() {
            let id = notification_segment(&item.id);
            let names = edited.filter(|(edited_id, _)| *edited_id == item.id).map(|(_, names)| names).unwrap_or(&item.names);
            html.push_str(&format!(r#"<form class="notification-names" data-admin-only="true" hx-post="/api/indicators/notifications/{id}/accept" hx-target="closest .notifications-modal" hx-swap="outerHTML" hx-sync="closest .notifications-modal:drop" hx-disabled-elt=".notifications-modal button">"#));
            for (index, name) in names.iter().enumerate() {
                let label = if index == 0 { "Portal name".to_owned() } else { format!("Alias {index}") };
                html.push_str(&format!(r#"<label>{label}<input class="ui-input" type="text" name="names" value="{}" autocomplete="off" spellcheck="false"></label>"#, html_escape(name)));
            }
            html.push_str(&format!(r#"<div class="button-row"><button type="submit" class="ui-button ui-button--primary">Accept</button><button type="button" class="ui-button ui-button--secondary" hx-post="/api/indicators/notifications/{id}/dismiss" hx-target="closest .notifications-modal" hx-swap="outerHTML">Dismiss</button></div></form>"#));
        } else {
            html.push_str("<ul class=\"notification-names\">");
            for name in &item.names { html.push_str(&format!("<li>{}</li>", html_escape(name))); }
            html.push_str("</ul>");
            if item.kind == "house-link-up" && item.state == "pending" && session != Session::Admin {
                html.push_str("<p>The owner can accept this in Admin Mode</p>");
            } else if item.state == "accepted" {
                html.push_str("<p>Accepted</p>");
            }
        }
        html.push_str("</article>");
    }
    html.push_str("</div>");
    html
}

fn notifications_message(signal: &str) -> &'static str {
    match signal {
        "link-up-name-taken" => "That name is already used on your network",
        "link-up-name-invalid" => "Choose a valid name ending in .home.arpa, such as noesis.home.arpa",
        "link-up-admin-required" => "Enter Admin Mode to manage this request",
        "link-up-accept-on-homeserver" => "Accept this request on your HomeServer",
        signal if signal.starts_with("caduceus-attendance-") || signal == "caduceus-access-refused" => "Enter Admin Mode to manage this request",
        _ => "This request could not be completed. Please try again.",
    }
}

pub(crate) fn notifications_routes() -> Router<AppState> {
    Router::new()
        .route("/api/indicators/notifications", get(notifications_modal_route))
        .route("/api/indicators/notifications/badge", get(notifications_badge_route))
        .route("/api/indicators/notifications/:id/:action", post(notifications_action_route))
}

async fn notifications_badge_route() -> Response {
    let (view, _) = tokio::task::spawn_blocking(|| notifications_snapshot(false)).await
        .unwrap_or((NotificationsView { items: Vec::new(), available: false }, 0));
    notification_html_response(notifications_badge(&view))
}

async fn notifications_modal_route(headers: axum::http::HeaderMap) -> Response {
    let session = session_projection_from_headers(&headers).await;
    let (view, _) = tokio::task::spawn_blocking(|| notifications_snapshot(false)).await
        .unwrap_or((NotificationsView { items: Vec::new(), available: false }, 0));
    notification_html_response(notifications_modal_html(&view, session, None, None))
}

fn notification_html_response(html: String) -> Response {
    let mut response = Html(html).into_response();
    response.headers_mut().insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    response
}

async fn notifications_action_route(
    headers: axum::http::HeaderMap,
    Path((id, action)): Path<(String, String)>,
    body: Bytes,
) -> Response {
    if !matches!(action.as_str(), "accept" | "dismiss") { return StatusCode::NOT_FOUND.into_response(); }
    let started = Instant::now();
    let names = if headers.get(header::CONTENT_TYPE).and_then(|v| v.to_str().ok()).is_some_and(|v| v.starts_with("application/json")) {
        serde_json::from_slice::<serde_json::Value>(&body).ok()
            .and_then(|value| value.get("names").and_then(serde_json::Value::as_array).cloned())
            .unwrap_or_default().iter().filter_map(serde_json::Value::as_str).map(str::to_owned).collect::<Vec<_>>()
    } else {
        url::form_urlencoded::parse(&body).filter(|(key, _)| key == "names").map(|(_, value)| value.into_owned()).collect::<Vec<_>>()
    };
    let path = format!("{NOTIFICATIONS_PATH}/{}/{action}", notification_segment(&id));
    let request_body = if action == "accept" { serde_json::json!({"names":names}) } else { serde_json::json!({}) };
    let request_headers = headers.clone();
    let request_path = path.clone();
    let readback = tokio::task::spawn_blocking(move || {
        caduceus_actuate_json(&mutation_authority(), &request_headers, &request_path, request_body)
    }).await.unwrap_or_else(|_| mutation_task_failure_readback(&path));
    let ok = readback.ok && readback.body.get("ok").and_then(serde_json::Value::as_bool) == Some(true)
        && readback.body.get("converged").and_then(serde_json::Value::as_bool) != Some(false);
    let signal = readback.body.get("signal").or_else(|| readback.body.get("firstMissingSignal"))
        .or_else(|| readback.body.get("first_missing_signal")).or_else(|| readback.body.get("code"))
        .and_then(serde_json::Value::as_str).unwrap_or(&readback.first_missing_signal);
    let message = if ok {
        if action == "accept" { "Request accepted" } else { "Request dismissed" }
    } else { notifications_message(signal) };
    let deed_signal = if ok { "none" } else {
        match signal {
            "link-up-name-invalid" | "link-up-name-taken" | "link-up-admin-required" | "link-up-accept-on-homeserver" => signal,
            _ => "notification-action-refused",
        }
    };
    // A change is one short deed; neither the names nor any request/receipt body is logged.
    eprintln!("{}", serde_json::json!({"actor":"coronatio","verb":action,"target":"notification",
        "id":id.chars().take(128).collect::<String>(),"outcome":if ok {"ok"} else {"refused"},
        "durationMs":started.elapsed().as_millis(),"signal":deed_signal,
        "firstMissingField":if matches!(signal, "link-up-name-invalid" | "link-up-name-taken") { Some("names") } else { None }}));
    let (view, _) = tokio::task::spawn_blocking(move || notifications_snapshot(ok)).await
        .unwrap_or((NotificationsView { items: Vec::new(), available: false }, 0));
    let session = session_projection_from_headers(&headers).await;
    let mut response = notification_html_response(notifications_modal_html(
        &view, session, Some((message, ok)), (!ok && action == "accept").then_some((id.as_str(), names.as_slice())),
    ));
    if ok {
        response.headers_mut().insert("hx-trigger", HeaderValue::from_static("notifications.status"));
    }
    response
}
