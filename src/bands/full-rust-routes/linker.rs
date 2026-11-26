#[derive(Debug, Clone, Default, Deserialize)]
struct LinkerQuery {
    path: Option<String>,
    filter: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize)]
struct LinkerForm {
    path: Option<String>,
    checked: Option<String>,
    new_dir_name: Option<String>,
    new_name: Option<String>,
    confirm: Option<String>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LinkerMode {
    Browsing,
    Naming,
    Review,
    Result,
    Fault,
}
impl LinkerMode {
    fn as_str(self) -> &'static str {
        match self {
            Self::Browsing => "Browsing",
            Self::Naming => "Naming",
            Self::Review => "Review",
            Self::Result => "Result",
            Self::Fault => "Fault",
        }
    }
}
#[derive(Debug, Clone)]
struct LinkerEntry {
    name: String,
    path: String,
    is_dir: bool,
    is_hardlink: bool,
    nlink: u64,
}
#[derive(Debug, Clone)]
struct LinkerState {
    path: String,
    filter: String,
    selected: Vec<String>,
    selected_delete_eligible: BTreeMap<String, bool>,
    entries: Vec<LinkerEntry>,
    mode: LinkerMode,
    rename_naming: bool,
    naming_refused: bool,
    rename_target: Option<String>,
    review_targets: Vec<(String, Option<String>)>,
    feedback: String,
    retry_read: Option<(String, String)>,
}
impl Default for LinkerState {
    fn default() -> Self {
        Self {
            path: "/mnt/nas".into(),
            filter: String::new(),
            selected: Vec::new(),
            selected_delete_eligible: BTreeMap::new(),
            entries: Vec::new(),
            mode: LinkerMode::Browsing,
            rename_naming: false,
            naming_refused: false,
            rename_target: None,
            review_targets: Vec::new(),
            feedback: "Ready to browse admitted sources.".into(),
            retry_read: None,
        }
    }
}
static LINKER_STATES: OnceLock<Mutex<BTreeMap<String, LinkerState>>> = OnceLock::new();
fn linker_states() -> &'static Mutex<BTreeMap<String, LinkerState>> {
    LINKER_STATES.get_or_init(|| Mutex::new(BTreeMap::new()))
}
fn linker_key(h: &axum::http::HeaderMap) -> String {
    document_incarnation_from_headers(h).unwrap_or_else(|| "__curl_fallback__".into())
}
fn linker_escape(v: &str) -> String {
    v.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
fn linker_url(v: &str) -> String {
    let mut encoded = String::new();
    for byte in v.as_bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b'~') {
            encoded.push(*byte as char);
        } else {
            encoded.push_str(&format!("%{byte:02X}"));
        }
    }
    linker_escape(&encoded)
}
fn linker_entries(body: &serde_json::Value, filter: &str) -> Vec<LinkerEntry> {
    let Some(items) = body
        .get("entries")
        .and_then(serde_json::Value::as_array)
    else {
        return Vec::new();
    };
    let needle = filter.to_ascii_lowercase();
    let mut out = items
        .iter()
        .filter_map(|v| {
            let o = v.as_object()?;
            let name = o.get("name")?.as_str()?;
            if !needle.is_empty() && !name.to_ascii_lowercase().contains(&needle) {
                return None;
            }
            Some(LinkerEntry {
                name: name.into(),
                path: o.get("path")?.as_str()?.into(),
                is_dir: o
                    .get("is_dir")
                    .and_then(serde_json::Value::as_bool)
                    .unwrap_or(false),
                is_hardlink: o
                    .get("is_hardlink")
                    .and_then(serde_json::Value::as_bool)
                    .unwrap_or(false),
                nlink: o
                    .get("nlink")
                    .and_then(serde_json::Value::as_u64)
                    .unwrap_or(0),
            })
        })
        .collect::<Vec<_>>();
    out.sort_by_key(|e| (!e.is_dir, e.name.to_ascii_lowercase()));
    out
}
fn linker_seat(name: &str, value: impl Into<String>) -> xenia_kit::fragment::DataSeat {
    xenia_kit::fragment::DataSeat::new(name, Some(value.into())).expect("valid linker seat")
}
fn linker_hx(verb: &str, url: String) -> Vec<xenia_kit::fragment::DataSeat> {
    vec![linker_seat(verb, url), linker_seat("data-hx-target", "#linker-fragment"), linker_seat("data-hx-swap", "outerHTML")]
}
fn linker_act(act: &str, verb: &str, url: String) -> Vec<xenia_kit::fragment::DataSeat> {
    let mut seats = linker_hx(verb, url);
    seats.push(linker_seat("data-linker-act", act));
    seats
}
fn linker_result(s: &LinkerState) -> String {
    use maud::Render;
    use xenia_kit::review::{ResultLine, ResultLineSeats, ResultMark};
    let mut button = Vec::new();
    if let Some((path, filter)) = &s.retry_read {
        button = linker_hx("data-hx-get", format!("/admit/linker?path={}&filter={}", linker_url(path), linker_url(filter)));
        button.push(linker_seat("data-linker-retry", ""));
    }
    ResultLine {
        mark: if s.feedback.starts_with("Outcome unknown") { Some(ResultMark::Unknown) } else if s.naming_refused && s.mode == LinkerMode::Naming { Some(ResultMark::Refused) } else { match s.mode { LinkerMode::Fault => Some(ResultMark::Refused), LinkerMode::Result => Some(ResultMark::Done), _ => None } },
        words: s.feedback.clone(), button: s.retry_read.as_ref().map(|_| "Retry".into()),
        seats: ResultLineSeats { words: vec![linker_seat("data-linker-read-status", "")], button, ..Default::default() },
    }.render().into_string()
}
fn linker_fragment(s: &LinkerState, _readback: Option<&CaduceusHttpReadback>) -> String {
    use maud::Render;
    use xenia_kit::listing::{ListingKind, ListingRow, ListingRowSeats, ListingRows, Crumb, PocketItem, PathLine, PathLineSeats};
    use xenia_kit::review::{InlineReview, InlineReviewSeats, ReviewTarget, KeyHint, KeyHints};
    let root = LinkerState::default().path;
    let suffix = s.path.strip_prefix(&root).filter(|p| p.is_empty() || p.starts_with('/')).unwrap_or("");
    let segments: Vec<_> = suffix.split('/').filter(|p| !p.is_empty()).collect();
    let mut crumbs = Vec::new();
    let mut prefix = root.clone();
    crumbs.push(Crumb { text: "nas".into(), current: segments.is_empty(), seats: if segments.is_empty() { vec![] } else { linker_hx("data-hx-get", format!("/admit/linker?path={}", linker_url(&prefix))) } });
    for (i, part) in segments.iter().enumerate() {
        prefix.push('/'); prefix.push_str(part);
        let current = i + 1 == segments.len();
        crumbs.push(Crumb { text: (*part).into(), current, seats: if current { vec![] } else { linker_hx("data-hx-get", format!("/admit/linker?path={}", linker_url(&prefix))) } });
    }
    let pocket = s.selected.iter().map(|path| {
        let mut seats = linker_hx("data-hx-post", "/admit/linker/select".into());
        seats.push(linker_seat("data-hx-vals", serde_json::json!({"path":path}).to_string()));
        PocketItem { path: path.clone(), seats }
    }).collect();
    let pathline = PathLine { label: "Where you are".into(), trail: crumbs, pocket, drop_key: "d".into(), seats: PathLineSeats { nav: vec![linker_seat("data-linker-tree", "")], empty: linker_act("empty", "data-hx-post", "/admit/linker/empty".into()), ..Default::default() } }.render().into_string();
    let rows = s.entries.iter().map(|e| {
        let mut input = linker_hx("data-hx-post", "/admit/linker/select".into());
        input.push(linker_seat("data-hx-vals", serde_json::json!({"path":e.path}).to_string()));
        input.push(linker_seat("data-hx-trigger", "click consume"));
        ListingRow { kind: if e.is_dir { ListingKind::Folder } else { ListingKind::File }, name: e.name.clone(), count: if e.is_dir { None } else { Some(e.nlink) }, checked: s.selected.contains(&e.path), cursor: false, input_name: Some("checked".into()), input_value: Some("on".into()), seats: ListingRowSeats { row: vec![linker_seat("data-linker-entry", ""), linker_seat("data-linker-directory", e.is_dir.to_string()), linker_seat("data-linker-path", e.path.clone())], input, name: if e.is_dir { linker_hx("data-hx-get", format!("/admit/linker?path={}", linker_url(&e.path))) } else { vec![] } } }
    }).collect();
    let folder = s.path.rsplit('/').next().unwrap_or("nas");
    let listing = ListingRows { label: format!("Files in {folder}"), rows, seats: vec![] }.render().into_string();
    let review = if s.mode == LinkerMode::Review {
        let eligible = s.review_targets.iter().filter(|(_, note)| note.is_none()).count();
        let mut confirm = linker_act("confirm", "data-hx-post", "/admit/linker/delete".into());
        confirm.push(linker_seat("data-hx-vals", r#"{"confirm":"yes"}"#));
        InlineReview { question: format!("Delete {eligible}?"), targets: s.review_targets.iter().map(|(name, note)| ReviewTarget { name: name.clone(), note: note.clone(), seats: vec![] }).collect(), confirm: "Delete".into(), confirm_key: "y".into(), cancel: "Cancel".into(), cancel_key: "n".into(), seats: InlineReviewSeats { confirm, cancel: linker_act("cancel", "data-hx-post", "/admit/linker/cancel".into()), ..Default::default() } }.render().into_string()
    } else { String::new() };
    let naming = if s.mode == LinkerMode::Naming {
        let (path, field, label, submit) = if s.rename_naming { ("rename", "new_name", "New name", "Rename") } else { ("mkdir", "new_dir_name", "Folder name", "Create") };
        let input = xenia_kit::ux::Input { label: Some(label.into()), name: Some(field.into()), ..Default::default() }.render().into_string();
        format!(r##"<form data-linker-naming data-hx-post="/admit/linker/{path}" data-hx-target="#linker-fragment" data-hx-swap="outerHTML">{input}<button type="submit">{submit}</button><button type="button" data-linker-act="cancel" data-hx-post="/admit/linker/cancel" data-hx-target="#linker-fragment" data-hx-swap="outerHTML">Cancel</button></form>"##)
    } else { String::new() };
    let hint = |key: &str, word: &str, action: Option<(&str, &str, String)>| {
        KeyHint { key: key.into(), word: word.into(), button: action.is_some(), seats: action.map(|(act, verb, url)| linker_act(act, verb, url)).unwrap_or_default() }
    };
    let up = if s.path.starts_with(&(root.clone()+"/")) { let parent = s.path.rsplit_once('/').map(|(p,_)| p).unwrap_or(&root); Some(("up", "data-hx-get", format!("/admit/linker?path={}", linker_url(parent)))) } else { None };
    let post = |act: &'static str, route: &str| Some((act, "data-hx-post", format!("/admit/linker/{route}")));
    let hints = KeyHints { label: "Keys on the Linker tab".into(), rows: vec![vec![hint("h", "up", up), hint("j/k", "move", None), hint("l/Enter", "open", None), hint("space", "gather", None), hint("d", "drop", post("drop", "deploy"))], vec![hint("del", "delete", post("delete", "delete")), hint("r", "rename", post("rename", "rename")), hint("n", "new folder", post("new", "naming")), hint("Esc", "close", None)]] }.render().into_string();
    format!(r##"<section id="linker-fragment" data-linker-state="{}" data-linker-path="{}" data-linker-root="{}">{}<div class="linker-list">{}</div>{}{}{}{}</section>"##, s.mode.as_str(), linker_escape(&s.path), linker_escape(&root), pathline, listing, review, naming, linker_result(s), hints)
}
fn linker_response(
    s: &LinkerState,
    r: Option<&CaduceusHttpReadback>,
    status: StatusCode,
) -> Response {
    let mut x = (status, Html(linker_fragment(s, r))).into_response();
    x.headers_mut()
        .insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    x
}
fn linker_call(
    h: &axum::http::HeaderMap,
    p: &str,
    b: serde_json::Value,
) -> CaduceusHttpReadback {
    caduceus_actuate_json(
        &mutation_authority(),
        h,
        p,
        b,
    )
}
fn linker_read_refusal(s: &mut LinkerState, r: &CaduceusHttpReadback, path: String, filter: String) -> Response {
    s.mode = LinkerMode::Fault;
    s.feedback = r.first_missing_signal.clone();
    if !r.ok && r.first_missing_signal.starts_with("linker-") {
        let mut fault = s.clone();
        fault.retry_read = Some((path, filter));
        linker_response(&fault, Some(r), StatusCode::OK)
    } else {
        linker_response(s, Some(r), mutation_response_status(r))
    }
}
fn linker_read(
    h: &axum::http::HeaderMap,
    path: &str,
    query: &[(&str, &str)],
) -> CaduceusHttpReadback {
    let mut encoded = url::form_urlencoded::Serializer::new(String::new());
    for (name, value) in query {
        encoded.append_pair(name, value);
    }
    let request_path = format!("{path}?{}", encoded.finish());
    caduceus_http_json_with_attendance_and_document(
        "GET",
        &request_path,
        serde_json::json!({}),
        attendance_from_headers(h).as_ref(),
        document_incarnation_from_headers(h).as_deref(),
    )
}
fn linker_browse(
    h: &axum::http::HeaderMap,
    s: &mut LinkerState,
) -> Result<CaduceusHttpReadback, CaduceusHttpReadback> {
    let r = linker_read(h, "/api/v1/linker/browse", &[("path", &s.path)]);
    if !r.ok {
        return Err(r);
    }
    s.entries = linker_entries(&r.body, &s.filter);
    let scan = linker_read(h, "/api/v1/linker/hardlink-scan", &[("path", &s.path)]);
    if !scan.ok {
        return Err(scan);
    }
    for x in linker_entries(&scan.body, "") {
        if let Some(e) = s.entries.iter_mut().find(|e| e.path == x.path) {
            e.is_hardlink |= x.is_hardlink;
            e.nlink = e.nlink.max(x.nlink)
        }
    }
    for e in &s.entries {
        if s.selected.iter().any(|p| p == &e.path) {
            s.selected_delete_eligible
                .insert(e.path.clone(), e.is_dir || e.nlink > 1);
        }
    }
    Ok(r)
}
// None means eligible on this scan; every other disposition keeps the target.
fn linker_delete_check(h: &axum::http::HeaderMap, target: &str) -> Result<Option<String>, CaduceusHttpReadback> {
    let Some((parent, _)) = target.rsplit_once('/') else { return Ok(Some("kept: not found".into())); };
    let scan = linker_read(h, "/api/v1/linker/hardlink-scan", &[("path", parent)]);
    if !scan.ok { return Err(scan); }
    match linker_entries(&scan.body, "").into_iter().find(|entry| entry.path == target) {
        None => Ok(Some("kept: not found".into())),
        Some(entry) if entry.is_dir || entry.nlink > 1 => Ok(None),
        Some(_) => Ok(Some("kept: last copy".into())),
    }
}
async fn linker_fragment_route(h: axum::http::HeaderMap, Query(q): Query<LinkerQuery>) -> Response {
    let session = session_projection_from_headers(&h).await;
    let facts = load_iris_facts_sync();
    let visible = iris::plan(&facts, session).tabs.into_iter().any(|grant| grant.tab_id == "linker" && grant.state == RenderState::Visible);
    if !visible { return (StatusCode::NOT_FOUND, Html(String::new())).into_response(); }
    if q.path.is_none() && q.filter.is_none() {
        let mut skeleton = LinkerState::default();
        skeleton.feedback = "Directory data loads after this view is admitted.".into();
        let body = linker_fragment(&skeleton, None).replacen(
            "<section id=\"linker-fragment\"",
            &format!(r#"<section id="linker-fragment" data-linker-readback hx-get="/admit/linker?path={}" hx-trigger="linker-readback" hx-target="this" hx-swap="outerHTML" hx-request='{{"timeout":8000}}'"#, linker_url(&skeleton.path)),
            1,
        );
        let mut response = (StatusCode::OK, Html(body)).into_response();
        response.headers_mut().insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
        return response;
    }
    let k = linker_key(&h);
    let mut all = linker_states().lock().expect("linker states lock");
    let s = all.entry(k).or_default();
    let mut staged = s.clone();
    if let Some(p) = q.path.filter(|p| !p.trim().is_empty()) {
        staged.path = p
    }
    if let Some(f) = q.filter {
        staged.filter = f
    }
    staged.mode = LinkerMode::Browsing;
    match linker_browse(&h, &mut staged) {
        Ok(r) => {
            staged.feedback = format!("Browse complete at {}.", staged.path);
            staged.retry_read = None;
            *s = staged;
            linker_response(s, Some(&r), StatusCode::OK)
        }
        Err(r) => linker_read_refusal(s, &r, staged.path, staged.filter),
    }
}
fn linker_finish(h: &axum::http::HeaderMap, s: &mut LinkerState, r: CaduceusHttpReadback, label: &str, clear_selection: bool) -> Response {
    if !r.ok && !r.first_missing_signal.starts_with("linker-") {
        s.mode = LinkerMode::Fault;
        s.feedback = r.first_missing_signal.clone();
        return linker_response(s, Some(&r), mutation_response_status(&r));
    }
    let unknown = matches!(r.first_missing_signal.as_str(), "linker-deploy-outcome-unknown" | "linker-mutation-outcome-unknown");
    if r.ok && clear_selection { s.selected.clear(); s.selected_delete_eligible.clear(); }
    if let Err(browse) = linker_browse(h, s) {
        return linker_read_refusal(s, &browse, s.path.clone(), s.filter.clone());
    }
    if unknown {
        s.mode = LinkerMode::Fault;
        s.feedback = format!("Outcome unknown ({}) · the folder was read again", r.first_missing_signal);
    } else if r.ok {
        s.mode = LinkerMode::Result;
        s.naming_refused = false;
        s.feedback = format!("{label} completed.");
    } else {
        if s.mode != LinkerMode::Naming { s.mode = LinkerMode::Fault; }
        else { s.naming_refused = true; }
        s.feedback = r.first_missing_signal.clone();
    }
    linker_response(s, Some(&r), StatusCode::OK)
}
fn linker_local_refusal(s: &mut LinkerState, message: &str) -> Response {
    s.mode = LinkerMode::Fault;
    s.feedback = message.into();
    linker_response(s, None, StatusCode::OK)
}
async fn linker_transition_route(h: axum::http::HeaderMap, Path(action): Path<String>, Form(f): Form<LinkerForm>) -> Response {
    let k = linker_key(&h);
    let mut all = linker_states().lock().expect("linker states lock");
    let s = all.entry(k).or_default();
    if let Some(x) = mutation_context_refusal(&h) {
        s.mode = LinkerMode::Fault; s.feedback = x.code;
        return linker_response(s, None, StatusCode::UNAUTHORIZED);
    }
    match action.as_str() {
        "select" => {
            let Some(p) = f.path.filter(|p| !p.trim().is_empty()) else { return linker_local_refusal(s, "A path is required."); };
            if f.checked.as_deref() == Some("on") {
                if !s.selected.contains(&p) {
                    let eligible = s.entries.iter().find(|e| e.path == p).map(|e| e.is_dir || e.nlink > 1).unwrap_or(false);
                    s.selected.push(p.clone()); s.selected_delete_eligible.insert(p, eligible);
                }
            } else { s.selected.retain(|x| x != &p); s.selected_delete_eligible.remove(&p); }
            let mut staged = s.clone(); staged.mode = LinkerMode::Browsing;
            match linker_browse(&h, &mut staged) {
                Ok(r) => { staged.feedback = "Pocket updated.".into(); staged.retry_read = None; *s = staged; linker_response(s, Some(&r), StatusCode::OK) },
                Err(r) => linker_read_refusal(s, &r, staged.path, staged.filter),
            }
        }
        "browse" => {
            let mut staged = s.clone(); staged.mode = LinkerMode::Browsing;
            match linker_browse(&h, &mut staged) {
                Ok(r) => { staged.retry_read = None; *s = staged; linker_response(s, Some(&r), StatusCode::OK) },
                Err(r) => linker_read_refusal(s, &r, staged.path, staged.filter),
            }
        }
        "empty" => { s.selected.clear(); s.selected_delete_eligible.clear(); s.mode = LinkerMode::Result; s.feedback = "Pocket emptied.".into(); linker_response(s, None, StatusCode::OK) }
        "naming" => { s.mode = LinkerMode::Naming; s.naming_refused = false; s.rename_naming = false; s.rename_target = None; linker_response(s, None, StatusCode::OK) }
        "cancel" => { s.mode = LinkerMode::Browsing; s.rename_naming = false; s.rename_target = None; s.review_targets.clear(); s.feedback = "Cancelled.".into(); linker_response(s, None, StatusCode::OK) }
        "mkdir" => {
            let name = f.new_dir_name.unwrap_or_default();
            if name.trim().is_empty() { s.mode = LinkerMode::Naming; s.naming_refused = true; s.feedback = "Folder name is required.".into(); return linker_response(s, None, StatusCode::OK); }
            let r = linker_call(&h, "/api/v1/linker/mkdir", serde_json::json!({"path":s.path,"new_dir_name":name}));
            linker_finish(&h, s, r, "New directory", false)
        }
        "rename" => {
            if f.new_name.is_none() {
                let target = f.path.filter(|p| s.entries.iter().any(|e| e.is_dir && e.path == *p)).or_else(|| if s.selected.len() == 1 { Some(s.selected[0].clone()) } else { None });
                let Some(target) = target else { return linker_local_refusal(s, "No folder named for rename."); };
                s.rename_target = Some(target); s.mode = LinkerMode::Naming; s.naming_refused = false; s.rename_naming = true;
                return linker_response(s, None, StatusCode::OK);
            }
            let Some(target) = s.rename_target.clone() else { return linker_local_refusal(s, "No folder named for rename."); };
            let name = f.new_name.unwrap_or_default();
            if name.trim().is_empty() { s.mode = LinkerMode::Naming; s.naming_refused = true; s.feedback = "New name is required.".into(); return linker_response(s, None, StatusCode::OK); }
            let r = linker_call(&h, "/api/v1/linker/rename", serde_json::json!({"path":target,"new_name":name}));
            linker_finish(&h, s, r, "Rename", true)
        }
        "delete" => {
            if f.confirm.as_deref() != Some("yes") {
                if s.selected.is_empty() { return linker_local_refusal(s, "Pocket is empty."); }
                let mut targets = Vec::new();
                for path in s.selected.clone() {
                    let note = match linker_delete_check(&h, &path) {
                        Ok(note) => note,
                        Err(r) if r.first_missing_signal.starts_with("linker-") => Some(format!("kept: {}", r.first_missing_signal)),
                        Err(r) => {
                            s.mode = LinkerMode::Fault;
                            s.feedback = format!("{path}: kept: {}", r.first_missing_signal);
                            return linker_response(s, Some(&r), mutation_response_status(&r));
                        },
                    };
                    targets.push((path, note));
                }
                s.review_targets = targets;
                s.mode = LinkerMode::Review;
                return linker_response(s, None, StatusCode::OK);
            }
            if s.mode != LinkerMode::Review { return linker_local_refusal(s, "Delete requires review."); }
            let targets = s.review_targets.clone();
            let original_selected = s.selected.clone();
            let original_eligibility = s.selected_delete_eligible.clone();
            let mut outcomes = Vec::new();
            let mut refused = false;
            for (path, review_note) in targets {
                let fresh_note = match linker_delete_check(&h, &path) {
                    Ok(note) => note,
                    Err(r) if r.first_missing_signal.starts_with("linker-") => { refused = true; Some(format!("kept: {}", r.first_missing_signal)) },
                    Err(r) => {
                        outcomes.push(format!("{path}: kept: {}", r.first_missing_signal));
                        s.mode = LinkerMode::Fault;
                        s.feedback = outcomes.join(" · ");
                        return linker_response(s, Some(&r), mutation_response_status(&r));
                    },
                };
                if let Some(note) = fresh_note.or(review_note) { outcomes.push(format!("{path}: {note}")); continue; }
                let r = linker_call(&h, "/api/v1/linker/delete", serde_json::json!({"path":path}));
                if !r.ok && !r.first_missing_signal.starts_with("linker-") { return linker_response(s, Some(&r), mutation_response_status(&r)); }
                if matches!(r.first_missing_signal.as_str(), "linker-deploy-outcome-unknown" | "linker-mutation-outcome-unknown") {
                    s.selected = original_selected;
                    s.selected_delete_eligible = original_eligibility;
                    return linker_finish(&h, s, r, "Delete", false);
                }
                if r.ok { s.selected.retain(|p| p != &path); s.selected_delete_eligible.remove(&path); outcomes.push(format!("{path}: deleted")); }
                else { refused = true; outcomes.push(format!("{path}: refused ({})", r.first_missing_signal)); }
            }
            s.review_targets.clear();
            if let Err(r) = linker_browse(&h, s) { return linker_read_refusal(s, &r, s.path.clone(), s.filter.clone()); }
            s.mode = if refused { LinkerMode::Fault } else { LinkerMode::Result }; s.feedback = outcomes.join(" · ");
            linker_response(s, None, StatusCode::OK)
        }
        "deploy" => {
            if s.selected.is_empty() { return linker_local_refusal(s, "Pocket is empty"); }
            let selected = s.selected.clone();
            let r = linker_call(&h, "/api/v1/linker/deploy", serde_json::json!({"sources":selected,"destination":s.path,"conflict_strategy":"rename"}));
            if !r.ok && !r.first_missing_signal.starts_with("linker-") { return linker_response(s, Some(&r), mutation_response_status(&r)); }
            let unknown = matches!(r.first_missing_signal.as_str(), "linker-deploy-outcome-unknown" | "linker-mutation-outcome-unknown");
            if unknown { return linker_finish(&h, s, r, "Drop", false); }
            let results = r.body.get("details").and_then(|d| d.get("results")).and_then(|v| v.as_array());
            if let Some(results) = results {
                let mut failed = Vec::new(); let mut succeeded = 0;
                for (i, path) in selected.iter().enumerate() {
                    if results.get(i).and_then(|v| v.get("coreResult")).and_then(|v| v.as_bool()) == Some(true) { succeeded += 1; s.selected.retain(|p| p != path); s.selected_delete_eligible.remove(path); }
                    else { failed.push(path.rsplit('/').next().unwrap_or(path).to_string()); }
                }
                if let Err(browse) = linker_browse(&h, s) { return linker_read_refusal(s, &browse, s.path.clone(), s.filter.clone()); }
                s.mode = if failed.is_empty() { LinkerMode::Result } else { LinkerMode::Fault };
                s.feedback = format!("Dropped {} into {} · {} linked · {} failed{}", selected.len(), s.path, succeeded, failed.len(), if failed.is_empty() { String::new() } else { format!(": {}", failed.join(", ")) });
                linker_response(s, Some(&r), StatusCode::OK)
            } else { linker_finish(&h, s, r, "Drop", false) }
        }
        _ => linker_local_refusal(s, "Unknown linker action."),
    }
}
