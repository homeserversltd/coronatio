const NOTIFICATIONS_TOPIC: &str = "notifications.status";
const NOTIFICATIONS_PATH: &str = "/api/v1/notifications";
const NOTIFICATIONS_TTL: Duration = Duration::from_secs(30);
const NOTIFICATIONS_ICON: &str = r#"<svg class="indicator-icon" viewBox="0 0 24 24" aria-hidden="true"><path d="M12 2a2 2 0 0 1 2 2v.3A7 7 0 0 1 19 11v5l2 2v1H3v-1l2-2v-5a7 7 0 0 1 5-6.7V4a2 2 0 0 1 2-2zm0 4a5 5 0 0 0-5 5v5.8L6.8 17h10.4l-.2-.2V11a5 5 0 0 0-5-5zm-2 14h4a2 2 0 0 1-4 0z"/></svg>"#;

#[derive(Clone, PartialEq, Eq)]
struct NotificationView {
    id: String,
    #[allow(dead_code)]
    kind: String,
    #[allow(dead_code)]
    state: String,
    title: String,
    hostname: String,
    names: Vec<String>,
    color: String,
    body: Option<String>,
    hidden: bool,
    act: Option<NotificationAction>,
}

#[derive(Clone, PartialEq, Eq)]
struct NotificationAction {
    label: String,
    fields: serde_json::Value,
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
        != Some("caduceus.notifications.v1")
        && !caduceus_receipt_matches_path(&readback.body, &readback.path)
    {
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
            hostname: row.pointer("/from/hostname").and_then(serde_json::Value::as_str)
                .or_else(|| row.pointer("/from/body").and_then(serde_json::Value::as_str)).unwrap_or("").to_owned(),
            names: names.into_iter().flatten().filter_map(serde_json::Value::as_str).map(str::to_owned).collect(),
            color: text("color"),
            body: row.get("body").and_then(serde_json::Value::as_str).map(str::to_owned),
            hidden: row.get("hidden").and_then(serde_json::Value::as_bool).unwrap_or(false),
            act: row.get("actions").and_then(serde_json::Value::as_array).and_then(|actions| {
                actions.iter().find_map(|action| {
                    let label = action.get("label").and_then(serde_json::Value::as_str)?;
                    (action.get("verb").and_then(serde_json::Value::as_str) == Some("accept") && !label.is_empty()).then(|| NotificationAction {
                        label: label.to_owned(), fields: action.get("fields").cloned().unwrap_or_default(),
                    })
                })
            }),
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
    let count = view.items.iter().filter(|item| !item.hidden).count();
    if count == 0 { return NOTIFICATIONS_ICON.to_owned(); }
    format!(r#"{NOTIFICATIONS_ICON}<span class="notifications-count" data-notifications-count aria-label="{count} notifications" aria-live="polite">{count}</span>"#)
}

fn notifications_collector(_session: Session) -> Result<serde_json::Value, String> {
    let (view, revision) = notifications_snapshot(false);
    Ok(serde_json::json!({"count":view.items.iter().filter(|item| !item.hidden).count(),"available":view.available,"revision":revision}))
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

fn notification_seat(name: &str, value: String) -> xenia_kit::fragment::DataSeat {
    xenia_kit::fragment::DataSeat::new(name, Some(value))
        .unwrap_or_else(|error| panic!("invalid {name} seat: {error}"))
}

fn notification_post_seats(item: &NotificationView, verb: &str, hidden: bool, include: bool) -> Vec<xenia_kit::fragment::DataSeat> {
    let query = if hidden { "?view=hidden" } else { "" };
    let mut seats = vec![
        notification_seat("data-hx-post", format!("/api/indicators/notifications/{}/{verb}{query}", notification_segment(&item.id))),
        notification_seat("data-hx-target", "closest .notifications-modal".into()),
        notification_seat("data-hx-swap", "outerHTML".into()),
        notification_seat("data-hx-sync", "closest .notifications-modal:drop".into()),
        notification_seat("data-hx-disabled-elt", ".notifications-modal button".into()),
    ];
    if include { seats.push(notification_seat("data-hx-include", "closest .ui-notification-tile".into())); }
    seats
}

fn notifications_modal_html(
    view: &NotificationsView,
    session: Session,
    notice: Option<(&str, bool)>,
    edited: Option<(&str, &[String])>,
    hidden: bool,
) -> String {
    use maud::Render as _;
    let tiles = view.items.iter().filter(|item| item.hidden == hidden).map(|item| {
        let admin_act = item.act.as_ref().filter(|_| session == Session::Admin);
        let fields = if let Some(action) = admin_act {
            if action.fields.get("names").is_some() {
                let names = edited.filter(|(id, _)| *id == item.id).map(|(_, names)| names).unwrap_or(&item.names);
                Some(maud::html! { @for (index, name) in names.iter().enumerate() {
                    (xenia_kit::ux::Input { label: Some(if index == 0 { "Portal name".into() } else { format!("Alias {index}") }), name: Some("names".into()), value: Some(name.clone()), ..Default::default() })
                } })
            } else { None }
        } else if item.act.is_some() && session != Session::Admin {
            Some(maud::html! { p { "The owner can act on this in Admin Mode" } })
        } else { None };
        let mut seats = xenia_kit::composite::NotificationTileSeats::default();
        seats.tile.push(notification_seat("data-notification-id", item.id.clone()));
        if admin_act.is_some() { seats.act = notification_post_seats(item, "accept", hidden, true); }
        seats.hide = notification_post_seats(item, if hidden { "unhide" } else { "hide" }, hidden, false);
        xenia_kit::composite::NotificationTile {
            tag: item.hostname.clone(), tone: xenia_kit::composite::NoticeTone::from_color(&item.color),
            title: if item.title.is_empty() { "Notification".into() } else { item.title.clone() },
            body: item.body.clone().or_else(|| (!item.names.is_empty()).then(|| item.names.join(", "))),
            fields, act: admin_act.map(|action| action.label.clone()), hidden, seats,
        }
    }).collect::<Vec<_>>();
    let empty = tiles.is_empty();
    let list = xenia_kit::composite::NotificationTiles { label: if hidden { "Hidden notifications" } else { "Notifications" }.into(), tiles };
    let button = xenia_kit::ux::Button {
        text: if hidden { "Back".into() } else { format!("Hidden ({})", view.items.iter().filter(|item| item.hidden).count()) },
        tone: Some(xenia_kit::ux::ButtonTone::Secondary), size: Some(xenia_kit::ux::Size::Small), button_type: true,
        hx: xenia_kit::ux::Hx { get: Some(if hidden { "/api/indicators/notifications" } else { "/api/indicators/notifications/hidden" }.into()),
            target: Some("closest .notifications-modal".into()), swap: Some(xenia_kit::ux::Swap::OuterHtml), ..Default::default() },
        ..Default::default()
    };
    maud::html! {
        div class="notifications-modal modal-content" data-modal-kind-body="notifications" data-indicator-fragment {
            div {
                @if let Some((message, ok)) = notice { p class="notifications-result" data-notifications-result data-ok=(ok) role="status" { (message) } }
                @if hidden { (button.render()) }
                @if empty {
                    @if !view.available { p { "Notifications are unavailable right now" } }
                    @else if hidden { p { "Nothing is hidden." } }
                    @else { p { "No notifications" } }
                } @else { (list.render()) }
                @if !hidden { (button.render()) }
            }
        }
    }.into_string()
}

fn notifications_message(signal: &str) -> &'static str {
    match signal {
        "notification-absent" => "That notification is already gone",
        "link-up-name-taken" => "That name is already used on your network",
        "link-up-name-invalid" => "Choose a valid name ending in .home.arpa, such as noesis.home.arpa",
        "link-up-admin-required" => "Enter Admin Mode to manage this request",
        "link-up-accept-on-homeserver" => "Accept this request on your HomeServer",
        signal if signal.starts_with("caduceus-attendance-") || signal == "caduceus-access-refused"
            || signal == "exousia-pin-required"
            || signal == "exousia-pin-wrong" => "Enter Admin Mode to manage this request",
        _ => "This request could not be completed. Please try again.",
    }
}

pub(crate) fn notifications_routes() -> Router<AppState> {
    Router::new()
        .route("/api/indicators/notifications", get(notifications_modal_route))
        .route("/api/indicators/notifications/hidden", get(notifications_hidden_route))
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
    notification_html_response(notifications_modal_html(&view, session, None, None, false))
}

async fn notifications_hidden_route(headers: axum::http::HeaderMap) -> Response {
    let session = session_projection_from_headers(&headers).await;
    let (view, _) = tokio::task::spawn_blocking(|| notifications_snapshot(false)).await
        .unwrap_or((NotificationsView { items: Vec::new(), available: false }, 0));
    notification_html_response(notifications_modal_html(&view, session, None, None, true))
}

fn notification_html_response(html: String) -> Response {
    let mut response = Html(html).into_response();
    response.headers_mut().insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    response
}

async fn notifications_action_route(
    headers: axum::http::HeaderMap,
    Path((id, action)): Path<(String, String)>,
    axum::extract::Query(query): axum::extract::Query<std::collections::HashMap<String, String>>,
    body: Bytes,
) -> Response {
    if !matches!(action.as_str(), "accept" | "dismiss" | "hide" | "unhide") { return StatusCode::NOT_FOUND.into_response(); }
    let started = Instant::now();
    let names = if headers.get(header::CONTENT_TYPE).and_then(|v| v.to_str().ok()).is_some_and(|v| v.starts_with("application/json")) {
        serde_json::from_slice::<serde_json::Value>(&body).ok()
            .and_then(|value| value.get("names").and_then(serde_json::Value::as_array).cloned())
            .unwrap_or_default().iter().filter_map(serde_json::Value::as_str).map(str::to_owned).collect::<Vec<_>>()
    } else {
        url::form_urlencoded::parse(&body).filter(|(key, _)| key == "names").map(|(_, value)| value.into_owned()).collect::<Vec<_>>()
    };
    let forwarded_names: Vec<&str> = names.iter().map(|name| name.trim()).filter(|name| !name.is_empty()).collect();
    let path = format!("{NOTIFICATIONS_PATH}/{}/{action}", notification_segment(&id));
    let request_body = if action == "accept" { serde_json::json!({"names":forwarded_names}) } else { serde_json::json!({}) };
    let request_headers = headers.clone();
    let request_path = path.clone();
    let unattended = matches!(action.as_str(), "hide" | "unhide");
    let readback = tokio::task::spawn_blocking(move || {
        if unattended { caduceus_http("POST", &request_path) }
        else { caduceus_actuate_json(&mutation_authority(), &request_headers, &request_path, request_body) }
    }).await.unwrap_or_else(|_| mutation_task_failure_readback(&path));
    let ok = readback.ok && readback.body.get("ok").and_then(serde_json::Value::as_bool) == Some(true)
        && readback.body.get("converged").and_then(serde_json::Value::as_bool) != Some(false)
        && (!unattended || caduceus_receipt_matches_path(&readback.body, &readback.path));
    let signal = readback.body.get("signal").or_else(|| readback.body.get("firstMissingSignal"))
        .or_else(|| readback.body.get("first_missing_signal")).or_else(|| readback.body.get("code"))
        .and_then(serde_json::Value::as_str).unwrap_or(&readback.first_missing_signal);
    let message = if ok {
        match action.as_str() { "accept" => "Request accepted", "dismiss" => "Request dismissed", "hide" => "Hidden", _ => "Back on your bell" }
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
    let (view, _) = tokio::task::spawn_blocking(move || notifications_snapshot(ok || unattended)).await
        .unwrap_or((NotificationsView { items: Vec::new(), available: false }, 0));
    let session = session_projection_from_headers(&headers).await;
    let mut response = notification_html_response(notifications_modal_html(
        &view, session, Some((message, ok)), (!ok && action == "accept").then_some((id.as_str(), names.as_slice())),
        query.get("view").is_some_and(|value| value == "hidden"),
    ));
    if ok {
        response.headers_mut().insert("hx-trigger", HeaderValue::from_static("notifications.status"));
    }
    response
}
