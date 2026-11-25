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
    verb: String,
    target_mac: String,
}

#[derive(Clone, PartialEq, Eq)]
struct NotificationsView {
    items: Vec<NotificationView>,
    available: bool,
    own_mac: Option<String>,
}

struct NotificationsCache {
    observed: Option<(Instant, NotificationsView)>,
    revision: u64,
}

static NOTIFICATIONS_CACHE: OnceLock<Mutex<NotificationsCache>> = OnceLock::new();
static NOTIFICATIONS_OWN_MAC: OnceLock<String> = OnceLock::new();

fn notifications_own_mac() -> Option<String> {
    if let Some(mac) = NOTIFICATIONS_OWN_MAC.get() { return Some(mac.clone()); }
    let reply = caduceus_http_timeout("GET", "/api/v1/ruyi?seat=1", Duration::from_secs(2));
    let body = &reply.body;
    let schema = body.get("schema").and_then(serde_json::Value::as_str);
    if !reply.ok || body.get("ok").and_then(serde_json::Value::as_bool) != Some(true)
        || !(schema == Some("caduceus.ruyi.v1") || (schema == Some("caduceus.staff.receipt.v1")
            && body.pointer("/stop/noun").and_then(serde_json::Value::as_str) == Some("ruyi"))) { return None; }
    let mac = body.pointer("/seat/mac").and_then(serde_json::Value::as_str)?.to_ascii_lowercase();
    let parts = mac.split(':').collect::<Vec<_>>();
    if parts.len() != 6 || !parts.iter().all(|part| part.len() == 2 && part.bytes().all(|b| b.is_ascii_hexdigit())) { return None; }
    let _ = NOTIFICATIONS_OWN_MAC.set(mac.clone());
    Some(mac)
}

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
    let mut view = notifications_view(&readback);
    view.own_mac = notifications_own_mac();
    if cache.observed.as_ref().is_none_or(|(_, prior)| prior != &view) {
        cache.revision = cache.revision.wrapping_add(1);
    }
    cache.observed = Some((Instant::now(), view.clone()));
    (view, cache.revision)
}

fn notifications_view(readback: &CaduceusHttpReadback) -> NotificationsView {
    let quiet = || NotificationsView { items: Vec::new(), available: true, own_mac: None };
    let unavailable = || NotificationsView { items: Vec::new(), available: false, own_mac: None };
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
            act: row.get("act").and_then(|act| {
                let verb = act.get("verb")?.as_str()?;
                let label = act.get("label")?.as_str()?;
                let target = act.pointer("/target/mac")?.as_str()?.to_ascii_lowercase();
                let from = row.pointer("/from/mac")?.as_str()?.to_ascii_lowercase();
                (verb == "update-now" && !label.is_empty() && !target.is_empty() && target == from).then(|| NotificationAction {
                    label: label.to_owned(), fields: serde_json::Value::Null, verb: verb.to_owned(), target_mac: target,
                })
            }).or_else(|| row.get("actions").and_then(serde_json::Value::as_array).and_then(|actions| {
                actions.iter().find_map(|action| {
                    let label = action.get("label").and_then(serde_json::Value::as_str)?;
                    (action.get("verb").and_then(serde_json::Value::as_str) == Some("accept") && !label.is_empty()).then(|| NotificationAction {
                        label: label.to_owned(), fields: action.get("fields").cloned().unwrap_or_default(), verb: "accept".into(), target_mac: String::new(),
                    })
                })
            })),
        });
    }
    NotificationsView { items, available: true, own_mac: None }
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

const NOTIFICATION_TARGET: &str = "closest .notifications-modal";
const NOTIFICATION_SWAP: &str = "outerHTML";
const NOTIFICATION_SYNC: &str = "closest .notifications-modal:drop";
const NOTIFICATION_DISABLED: &str = ".notifications-modal button";

fn notification_post_path(item: &NotificationView, verb: &str, hidden: bool) -> String {
    let query = if hidden { "?view=hidden" } else { "" };
    format!("/api/indicators/notifications/{}/{verb}{query}", notification_segment(&item.id))
}

fn notification_post_seats(item: &NotificationView, verb: &str, hidden: bool, include: bool) -> Vec<xenia_kit::fragment::DataSeat> {
    let mut seats = vec![
        notification_seat("data-hx-post", notification_post_path(item, verb, hidden)),
        notification_seat("data-hx-target", NOTIFICATION_TARGET.into()),
        notification_seat("data-hx-swap", NOTIFICATION_SWAP.into()),
        notification_seat("data-hx-sync", NOTIFICATION_SYNC.into()),
        notification_seat("data-hx-disabled-elt", NOTIFICATION_DISABLED.into()),
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
    pin_view: Option<&str>,
) -> String {
    use maud::Render as _;
    if let Some(id) = pin_view {
        if let Some(item) = view.items.iter().find(|item| item.id == id) {
            let path = notification_post_path(item, "update-now", hidden);
            let fields = maud::html! {
                form method="post" action=(&path) data-hx-post=(&path)
                    data-hx-target=(NOTIFICATION_TARGET) data-hx-swap=(NOTIFICATION_SWAP)
                    data-hx-sync=(NOTIFICATION_SYNC) data-hx-disabled-elt=(NOTIFICATION_DISABLED)
                    style="display: contents" {
                    label class="ui-input-label" { (format!("{}'s PIN", item.hostname))
                        input class="ui-input" type="password" name="pin" autocomplete="off";
                    }
                    button type="submit" hidden {}
                }
            };
            let mut seats = xenia_kit::composite::NotificationTileSeats::default();
            seats.tile.push(notification_seat("data-notification-id", item.id.clone()));
            seats.act = notification_post_seats(item, "update-now", hidden, true);
            seats.hide = notification_post_seats(item, if hidden { "unhide" } else { "hide" }, hidden, false);
            let tile = xenia_kit::composite::NotificationTile {
                tag: item.hostname.clone(), tone: xenia_kit::composite::NoticeTone::from_color(&item.color),
                title: if item.title.is_empty() { "Notification".into() } else { item.title.clone() },
                body: item.body.clone().or_else(|| (!item.names.is_empty()).then(|| item.names.join(", "))),
                fields: Some(fields), act: Some("Update now".into()), hidden, seats,
            };
            let list = xenia_kit::composite::NotificationTiles { label: "Notifications".into(), tiles: vec![tile] };
            let back_path = if hidden { "/api/indicators/notifications/hidden" } else { "/api/indicators/notifications" };
            return maud::html! {
                div class="notifications-modal modal-content" data-modal-kind-body="notifications" data-indicator-fragment {
                    div {
                        @if let Some((message, ok)) = notice { p class="notifications-result" data-notifications-result data-ok=(ok) role="status" { (message) } }
                        button type="button" class="ui-button ui-button--secondary ui-button--small"
                            data-hx-get=(back_path) data-hx-target=(NOTIFICATION_TARGET) data-hx-swap=(NOTIFICATION_SWAP) { "Back" }
                        (list.render())
                    }
                }
            }.into_string();
        }
    }
    let tiles = view.items.iter().filter(|item| item.hidden == hidden).map(|item| {
        let visible_act = item.act.as_ref().filter(|action| action.verb != "update-now" || view.own_mac.is_some());
        let admin_act = visible_act.filter(|_| session == Session::Admin);
        let fields = if let Some(action) = admin_act {
            if action.fields.get("names").is_some() {
                let names = edited.filter(|(id, _)| *id == item.id).map(|(_, names)| names).unwrap_or(&item.names);
                Some(maud::html! {
                    form data-hx-post=(notification_post_path(item, "accept", hidden))
                        data-hx-target=(NOTIFICATION_TARGET) data-hx-swap=(NOTIFICATION_SWAP)
                        data-hx-sync=(NOTIFICATION_SYNC) data-hx-disabled-elt=(NOTIFICATION_DISABLED)
                        style="display: contents" {
                        @for (index, name) in names.iter().enumerate() {
                            (xenia_kit::ux::Input { label: Some(if index == 0 { "Portal name".into() } else { format!("Alias {index}") }), name: Some("names".into()), value: Some(name.clone()), ..Default::default() })
                        }
                        button type="submit" hidden {}
                    }
                })
            } else { None }
        } else if visible_act.is_some() && session != Session::Admin {
            Some(maud::html! { p class="ui-notification-tile__hint" { "The owner can act on this in Admin Mode" } })
        } else { None };
        let mut seats = xenia_kit::composite::NotificationTileSeats::default();
        seats.tile.push(notification_seat("data-notification-id", item.id.clone()));
        if let Some(action) = admin_act {
            if action.verb == "update-now" && view.own_mac.as_deref() != Some(action.target_mac.as_str()) {
                seats.act = vec![
                    notification_seat("data-hx-get", notification_post_path(item, "update-now", hidden)),
                    notification_seat("data-hx-target", NOTIFICATION_TARGET.into()),
                    notification_seat("data-hx-swap", NOTIFICATION_SWAP.into()),
                    notification_seat("data-hx-sync", NOTIFICATION_SYNC.into()),
                    notification_seat("data-hx-disabled-elt", NOTIFICATION_DISABLED.into()),
                ];
            } else {
                seats.act = notification_post_seats(item, &action.verb, hidden, action.verb == "accept");
            }
        }
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
        .route("/api/indicators/notifications/:id/:action", get(notifications_ask_route).post(notifications_action_route))
}

async fn notifications_badge_route() -> Response {
    let (view, _) = tokio::task::spawn_blocking(|| notifications_snapshot(false)).await
        .unwrap_or((NotificationsView { items: Vec::new(), available: false, own_mac: None }, 0));
    notification_html_response(notifications_badge(&view))
}

async fn notifications_modal_route(headers: axum::http::HeaderMap) -> Response {
    let session = session_projection_from_headers(&headers).await;
    let (view, _) = tokio::task::spawn_blocking(|| notifications_snapshot(false)).await
        .unwrap_or((NotificationsView { items: Vec::new(), available: false, own_mac: None }, 0));
    notification_html_response(notifications_modal_html(&view, session, None, None, false, None))
}

async fn notifications_hidden_route(headers: axum::http::HeaderMap) -> Response {
    let session = session_projection_from_headers(&headers).await;
    let (view, _) = tokio::task::spawn_blocking(|| notifications_snapshot(false)).await
        .unwrap_or((NotificationsView { items: Vec::new(), available: false, own_mac: None }, 0));
    notification_html_response(notifications_modal_html(&view, session, None, None, true, None))
}

fn notification_html_response(html: String) -> Response {
    let mut response = Html(html).into_response();
    response.headers_mut().insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    response
}

const UPDATE_UNREACHABLE: &str = "This could not be reached. Please try again.";
const UPDATE_INVALID: &str = "This request could not be completed. Please try again.";
const UPDATE_ADMIN: &str = "Enter Admin Mode to manage this request";

fn notification_update_row<'a>(view: &'a NotificationsView, id: &str, ask: bool) -> Result<&'a NotificationView, &'static str> {
    if !view.available || view.own_mac.is_none() { return Err(UPDATE_UNREACHABLE); }
    let item = view.items.iter().find(|item| item.id == id).ok_or("That notification is already gone")?;
    let act = item.act.as_ref().filter(|act| act.verb == "update-now").ok_or(UPDATE_INVALID)?;
    if ask && view.own_mac.as_deref() == Some(act.target_mac.as_str()) { return Err(UPDATE_INVALID); }
    Ok(item)
}

async fn notifications_ask_route(
    headers: axum::http::HeaderMap,
    Path((id, action)): Path<(String, String)>,
    axum::extract::Query(query): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> Response {
    if action != "update-now" { return StatusCode::NOT_FOUND.into_response(); }
    let hidden = query.get("view").is_some_and(|value| value == "hidden");
    let session = session_projection_from_headers(&headers).await;
    let (view, _) = tokio::task::spawn_blocking(|| notifications_snapshot(false)).await
        .unwrap_or((NotificationsView { items: Vec::new(), available: false, own_mac: None }, 0));
    let result = if session != Session::Admin { Err(UPDATE_ADMIN) } else { notification_update_row(&view, &id, true).map(|_| ()) };
    let notice = result.as_ref().err().map(|message| (*message, false));
    notification_html_response(notifications_modal_html(&view, session, notice, None, hidden, result.is_ok().then_some(id.as_str())))
}

fn notification_update_pin(headers: &axum::http::HeaderMap, body: &[u8]) -> Result<String, ()> {
    if headers.get(header::CONTENT_TYPE).and_then(|v| v.to_str().ok()).is_some_and(|v| v.starts_with("application/json")) {
        let value: serde_json::Value = serde_json::from_slice(body).map_err(|_| ())?;
        let object = value.as_object().ok_or(())?;
        match object.get("pin") { None => Ok(String::new()), Some(pin) => pin.as_str().map(str::to_owned).ok_or(()) }
    } else {
        let text = std::str::from_utf8(body).map_err(|_| ())?;
        let bytes = text.as_bytes();
        for (i, byte) in bytes.iter().enumerate() {
            if *byte == b'%' && (i + 2 >= bytes.len() || !bytes[i + 1].is_ascii_hexdigit() || !bytes[i + 2].is_ascii_hexdigit()) { return Err(()); }
        }
        Ok(url::form_urlencoded::parse(body).find(|(key, _)| key == "pin").map(|(_, value)| value.into_owned()).unwrap_or_default())
    }
}

fn notification_update_answer(readback: &CaduceusHttpReadback, remote: bool, tag: &str) -> (String, bool, String, bool) {
    let body = &readback.body;
    fn nonempty(value: Option<&serde_json::Value>) -> Option<&str> {
        value.and_then(serde_json::Value::as_str).filter(|s| !s.is_empty() && *s != "none")
    }
    let signal = (if remote { nonempty(body.pointer("/answers/0/signal")) } else { None })
        .or_else(|| nonempty(body.get("firstMissingSignal")))
        .or_else(|| (!readback.first_missing_signal.is_empty() && readback.first_missing_signal != "none").then_some(readback.first_missing_signal.as_str()))
        .unwrap_or("none");
    let update = if remote { body.pointer("/answers/0/answer/answers/0/answer") } else { Some(body) };
    let state = update.and_then(|v| v.get("state")).and_then(serde_json::Value::as_str).unwrap_or("");
    let ok = update.and_then(|v| v.get("ok")).and_then(serde_json::Value::as_bool) == Some(true);
    let pin = matches!(signal, "exousia-pin-wrong" | "exousia-pin-required" | "exousia-pin-not-yet-provisioned");
    let (message, success) = match signal {
        "exousia-pin-wrong" => (format!("That is not {tag}'s PIN"), false),
        "exousia-pin-required" => (format!("Type {tag}'s PIN"), false),
        "exousia-pin-not-yet-provisioned" => (format!("{tag} has no PIN set yet"), false),
        "target-unknown" | "seat-mac-unknown" | "seat-mac-mismatch" => (format!("This house cannot confirm {tag} right now"), false),
        "unreachable" | "timeout" => (format!("{tag} did not answer"), false),
        "act-door-absent" => (format!("{tag} runs an older build that cannot take this"), false),
        "harmonia-update-start-unconfirmed" => (format!("Handed to {tag}, not yet confirmed"), true),
        "harmonia-update-busy" => (format!("{tag} is already updating"), true),
        _ if state == "started" => (format!("Update started on {tag}"), true),
        _ if state == "failed" || (state == "unknown" && !ok) => (format!("{tag} could not start the update"), false),
        _ if readback.status == 0 || signal == "caduceus-mutation-task-failed" => (UPDATE_UNREACHABLE.into(), false),
        _ if signal.starts_with("caduceus-attendance-") || signal == "caduceus-access-refused" => (UPDATE_ADMIN.into(), false),
        _ => ("Update now was refused".into(), false),
    };
    (message, success, signal.to_owned(), pin)
}

async fn notifications_update_route(headers: axum::http::HeaderMap, id: String, query: std::collections::HashMap<String, String>, body: Bytes) -> Response {
    let started = Instant::now();
    let hidden = query.get("view").is_some_and(|value| value == "hidden");
    let session = session_projection_from_headers(&headers).await;
    let pin = if session == Session::Admin { notification_update_pin(&headers, &body).and_then(|pin| if pin.len() <= 512 { Ok(pin) } else { Err(()) }) } else { Err(()) };
    drop(body);
    let (mut view, _) = tokio::task::spawn_blocking(|| notifications_snapshot(false)).await
        .unwrap_or((NotificationsView { items: Vec::new(), available: false, own_mac: None }, 0));
    let selection = if session != Session::Admin { Err(UPDATE_ADMIN) }
        else if pin.is_err() { Err(UPDATE_INVALID) }
        else { notification_update_row(&view, &id, false) };
    let (message, success, signal, show_pin) = match selection {
        Err(message) => (message.to_owned(), false, "none".to_owned(), false),
        Ok(item) => {
            let tag = item.hostname.clone();
            let target = item.act.as_ref().unwrap().target_mac.clone();
            let remote = view.own_mac.as_deref() != Some(target.as_str());
            let pin = pin.unwrap_or_default();
            let request_headers = headers.clone();
            let path = if remote { "/api/v1/ruyi/road" } else { "/api/v1/update/now" };
            let readback = tokio::task::spawn_blocking(move || {
                if remote {
                    let payload = serde_json::json!({"schema":"caduceus.ruyi.v1", "target":target, "thing":"update-now", "flags":{"exousia":{"pin":pin}}});
                    caduceus_actuate_json_with_timeout(&mutation_authority(), &request_headers, path, payload, Duration::from_secs(8))
                } else {
                    caduceus_actuate_with_timeout(&mutation_authority(), &request_headers, path, Duration::from_secs(8))
                }
            }).await.unwrap_or_else(|_| mutation_task_failure_readback(path));
            let answer = notification_update_answer(&readback, remote, &tag);
            if readback.status != 0 && readback.first_missing_signal != "caduceus-mutation-task-failed" {
                view = tokio::task::spawn_blocking(|| notifications_snapshot(true)).await
                    .unwrap_or((NotificationsView { items: Vec::new(), available: false, own_mac: None }, 0)).0;
            }
            answer
        }
    };
    eprintln!("{}", serde_json::json!({"actor":"coronatio","verb":"update-now","target":"notification",
        "id":id.chars().take(128).collect::<String>(), "outcome":if success {"ok"} else {"refused"},
        "durationMs":started.elapsed().as_millis(), "signal":signal}));
    notification_html_response(notifications_modal_html(&view, session, Some((&message, success)), None, hidden, show_pin.then_some(id.as_str())))
}

async fn notifications_action_route(
    headers: axum::http::HeaderMap,
    Path((id, action)): Path<(String, String)>,
    axum::extract::Query(query): axum::extract::Query<std::collections::HashMap<String, String>>,
    body: Bytes,
) -> Response {
    if !matches!(action.as_str(), "accept" | "dismiss" | "hide" | "unhide" | "update-now") { return StatusCode::NOT_FOUND.into_response(); }
    if action == "update-now" { return notifications_update_route(headers, id, query, body).await; }
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
        .unwrap_or((NotificationsView { items: Vec::new(), available: false, own_mac: None }, 0));
    let session = session_projection_from_headers(&headers).await;
    let mut response = notification_html_response(notifications_modal_html(
        &view, session, Some((message, ok)), (!ok && action == "accept").then_some((id.as_str(), names.as_slice())),
        query.get("view").is_some_and(|value| value == "hidden"), None,
    ));
    if ok {
        response.headers_mut().insert("hx-trigger", HeaderValue::from_static("notifications.status"));
    }
    response
}
