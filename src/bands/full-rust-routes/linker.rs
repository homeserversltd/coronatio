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
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LinkerMode {
    Browsing,
    Naming,
    DeleteReview,
    DeployReview,
    Result,
    Fault,
}
impl LinkerMode {
    fn as_str(self) -> &'static str {
        match self {
            Self::Browsing => "Browsing",
            Self::Naming => "Naming",
            Self::DeleteReview => "DeleteReview",
            Self::DeployReview => "DeployReview",
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
    destination: Option<String>,
    filter: String,
    selected: Vec<String>,
    entries: Vec<LinkerEntry>,
    mode: LinkerMode,
    rename_naming: bool,
    feedback: String,
}
impl Default for LinkerState {
    fn default() -> Self {
        Self {
            path: "/mnt/nas".into(),
            destination: None,
            filter: String::new(),
            selected: Vec::new(),
            entries: Vec::new(),
            mode: LinkerMode::Browsing,
            rename_naming: false,
            feedback: "Ready to browse admitted sources.".into(),
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
    linker_escape(
        &v.replace(' ', "%20")
            .replace('#', "%23")
            .replace('?', "%3F"),
    )
}
fn linker_entries(body: &serde_json::Value, filter: &str) -> Vec<LinkerEntry> {
    let Some(items) = body
        .get("receipt")
        .and_then(|v| v.get("entries"))
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
fn linker_receipt_html(r: &CaduceusHttpReadback) -> String {
    format!(
        "<details><summary>Caduceus receipt</summary><pre>{}</pre></details>",
        linker_escape(&r.body.to_string())
    )
}
fn linker_fragment(s: &LinkerState, readback: Option<&CaduceusHttpReadback>) -> String {
    let dest = s.destination.as_deref().unwrap_or("unset");
    let rows = if s.entries.is_empty() {
        "<p>No entries observed.</p>".into()
    } else {
        s.entries.iter().map(|e|{let checked=s.selected.iter().any(|p|p==&e.path);let linked=e.is_hardlink||e.nlink>1;format!("<div class=\"linker-row\"><form method=\"post\" action=\"/admit/linker/select\" hx-post=\"/admit/linker/select\" hx-target=\"#linker-fragment\" hx-swap=\"outerHTML\"><input type=\"hidden\" name=\"path\" value=\"{}\"><input type=\"checkbox\" name=\"checked\" value=\"on\" aria-label=\"Select {}\" {} hx-post=\"/admit/linker/select\" hx-include=\"closest form\"><span>{}</span>{}</form></div>",linker_escape(&e.path),linker_escape(&e.name),if checked{"checked"}else{""},linker_escape(&e.name),if linked{format!("<span class=\"linker-linked\">LINKED ×{}</span>",e.nlink)}else{String::new()})}).collect::<Vec<_>>().join("")
    };
    let segments = s
        .path
        .split(char::from_u32(47).unwrap())
        .filter(|p| !p.is_empty())
        .collect::<Vec<_>>();
    let mut prefix = String::new();
    let trail=segments.iter().enumerate().map(|(i,n)|{prefix.push(char::from_u32(47).unwrap());prefix.push_str(n);if i+1==segments.len(){linker_escape(n)}else{format!("<a href=\"/admit/linker?path={}\" hx-get=\"/admit/linker?path={}\" hx-target=\"#linker-fragment\" hx-swap=\"outerHTML\">{}</a>",linker_url(&prefix),linker_url(&prefix),linker_escape(n))}}).collect::<Vec<_>>().join(" / ");
    let naming = if s.mode == LinkerMode::Naming {
        if s.rename_naming {
            "<form class=\"linker-inline-form\" method=\"post\" action=\"/admit/linker/rename\" hx-post=\"/admit/linker/rename\" hx-target=\"#linker-fragment\" hx-swap=\"outerHTML\"><label>New name <input name=\"new_name\" required></label><button>Rename</button><button formaction=\"/admit/linker/cancel\">Cancel</button></form>".into()
        } else {
            "<form class=\"linker-inline-form\" method=\"post\" action=\"/admit/linker/mkdir\" hx-post=\"/admit/linker/mkdir\" hx-target=\"#linker-fragment\" hx-swap=\"outerHTML\"><label>New directory <input name=\"new_dir_name\" required></label><button>Create</button><button formaction=\"/admit/linker/cancel\">Cancel</button></form>".into()
        }
    } else {
        String::new()
    };
    let delete_review = if s.mode == LinkerMode::DeleteReview {
        format!("<section class=\"linker-review\"><p>Targets: {}</p><p>A file may be deleted. An empty directory may be deleted. A directory may be deleted only when it contains hardlink files only. A directory containing non-hardlink content is refused.</p><form method=\"post\" action=\"/admit/linker/delete\" hx-post=\"/admit/linker/delete\" hx-target=\"#linker-fragment\" hx-swap=\"outerHTML\"><button>Delete</button><button formaction=\"/admit/linker/cancel\">Cancel</button></form></section>",linker_escape(&s.selected.join(", ")))
    } else {
        String::new()
    };
    let deploy_review = if s.mode == LinkerMode::DeployReview {
        format!("<section class=\"linker-review\"><p>Sources: {}</p><p>Destination: {}</p><p>Policy: rename</p><p>Rename applies recursively to files only. A top-level directory conflict is refused.</p><form method=\"post\" action=\"/admit/linker/deploy\" hx-post=\"/admit/linker/deploy\" hx-target=\"#linker-fragment\" hx-swap=\"outerHTML\"><button {}>Deploy</button><button formaction=\"/admit/linker/cancel\">Cancel</button></form></section>",linker_escape(&s.selected.join(", ")),linker_escape(dest),if s.destination.is_none()||s.selected.is_empty(){"disabled title=\"Select sources and set a destination first\""}else{""})
    } else {
        String::new()
    };
    let result = if s.mode == LinkerMode::Result {
        "<form method=\"get\" action=\"/admit/linker\" hx-get=\"/admit/linker\" hx-target=\"#linker-fragment\" hx-swap=\"outerHTML\"><button>Refresh listing</button></form>"
    } else {
        ""
    };
    let disabled_select = if s.selected.is_empty() {
        "disabled title=\"Select at least one entry\""
    } else {
        ""
    };
    let disabled_one = if s.selected.len() != 1 {
        "disabled title=\"Select exactly one entry\""
    } else {
        ""
    };
    let disabled_deploy = if s.selected.is_empty() || s.destination.is_none() {
        "disabled title=\"Select sources and set a destination first\""
    } else {
        ""
    };
    let receipt = readback.map(linker_receipt_html).unwrap_or_default();
    format!(
        r###"<section id="linker-fragment" data-linker-state="{}"><header><h2>Linker</h2><nav aria-label="Ancestor trail"><a href="/admit/linker">/</a>{}</nav><p>Current path: {} · Visible: {} · Selected: {} · Destination: {} · Last feedback: <span aria-live="polite">{}</span></p></header><form method="get" action="/admit/linker" hx-get="/admit/linker" hx-target="#linker-fragment" hx-swap="outerHTML"><label>Name filter <input name="filter" type="search" value="{}"></label><input type="hidden" name="path" value="{}"><button>Filter</button></form><div class="linker-list" role="list">{}</div>{}<form class="linker-action-shelf" method="post"><button formaction="/admit/linker/destination" hx-post="/admit/linker/destination" hx-target="#linker-fragment" hx-swap="outerHTML">Set current directory as destination</button><button formaction="/admit/linker/deploy-review" hx-post="/admit/linker/deploy-review" hx-target="#linker-fragment" hx-swap="outerHTML" {}>Deploy selected</button><button formaction="/admit/linker/naming" hx-post="/admit/linker/naming" hx-target="#linker-fragment" hx-swap="outerHTML">New directory</button><button formaction="/admit/linker/rename" hx-post="/admit/linker/rename" hx-target="#linker-fragment" hx-swap="outerHTML" {}>Rename selected</button><button formaction="/admit/linker/delete-review" hx-post="/admit/linker/delete-review" hx-target="#linker-fragment" hx-swap="outerHTML" {}>Delete selected</button></form>{}{}{}{}<p hidden>Browsing Naming DeleteReview DeployReview Result Fault</p></section>"###,
        s.mode.as_str(),
        trail,
        linker_escape(&s.path),
        s.entries.len(),
        s.selected.len(),
        linker_escape(dest),
        linker_escape(&s.feedback),
        linker_escape(&s.filter),
        linker_escape(&s.path),
        rows,
        naming,
        disabled_deploy,
        disabled_one,
        disabled_select,
        delete_review,
        deploy_review,
        result,
        receipt
    )
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
    a: &str,
    p: &str,
    b: serde_json::Value,
) -> CaduceusHttpReadback {
    caduceus_actuate_json(
        &mutation_authority(),
        h,
        MutationActionTarget::caduceus(a, p),
        p,
        b,
    )
}
fn linker_read(h: &axum::http::HeaderMap, path: &str, query: &[(&str, &str)]) -> CaduceusHttpReadback {
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
    Ok(r)
}
async fn linker_fragment_route(h: axum::http::HeaderMap, Query(q): Query<LinkerQuery>) -> Response {
    let k = linker_key(&h);
    let mut all = linker_states().lock().expect("linker states lock");
    let s = all.entry(k).or_default();
    if let Some(p) = q.path.filter(|p| !p.trim().is_empty()) {
        s.path = p
    }
    if let Some(f) = q.filter {
        s.filter = f
    }
    s.mode = LinkerMode::Browsing;
    match linker_browse(&h, s) {
        Ok(r) => {
            s.feedback = format!("Browse complete at {}.", s.path);
            linker_response(s, Some(&r), StatusCode::OK)
        }
        Err(r) => {
            s.mode = LinkerMode::Fault;
            s.feedback = r.first_missing_signal.clone();
            linker_response(s, Some(&r), mutation_response_status(&r))
        }
    }
}
fn linker_finish(
    s: &mut LinkerState,
    r: CaduceusHttpReadback,
    label: &str,
    clear_selection: bool,
) -> Response {
    if r.ok {
        if clear_selection {
            s.selected.clear()
        }
        s.mode = LinkerMode::Result;
        s.feedback = format!("{} completed.", label)
    } else {
        s.mode = LinkerMode::Fault;
        s.feedback = r.first_missing_signal.clone()
    }
    let status = if r.ok {
        StatusCode::OK
    } else {
        mutation_response_status(&r)
    };
    linker_response(s, Some(&r), status)
}
async fn linker_transition_route(
    h: axum::http::HeaderMap,
    Path(action): Path<String>,
    Form(f): Form<LinkerForm>,
) -> Response {
    let k = linker_key(&h);
    let mut all = linker_states().lock().expect("linker states lock");
    let s = all.entry(k).or_default();
    if let Some(x) = mutation_context_refusal(&h) {
        s.mode = LinkerMode::Fault;
        s.feedback = x.code;
        return linker_response(s, None, StatusCode::UNAUTHORIZED);
    }
    match action.as_str() {
        "select" => {
            let Some(p) = f.path.filter(|p| !p.trim().is_empty()) else {
                s.feedback = "A path is required.".into();
                return linker_response(s, None, StatusCode::BAD_REQUEST);
            };
            if f.checked.as_deref() == Some("on") {
                if !s.selected.iter().any(|x| x == &p) {
                    s.selected.push(p)
                }
            } else {
                s.selected.retain(|x| x != &p)
            }
            s.mode = LinkerMode::Browsing;
            match linker_browse(&h, s) {
                Ok(r) => linker_response(s, Some(&r), StatusCode::OK),
                Err(r) => {
                    s.mode = LinkerMode::Fault;
                    linker_response(s, Some(&r), mutation_response_status(&r))
                }
            }
        }
        "browse" | "destination" => {
            if action == "destination" {
                s.destination = Some(s.path.clone());
                s.feedback = format!("Destination set to {}.", s.path)
            }
            s.mode = LinkerMode::Browsing;
            match linker_browse(&h, s) {
                Ok(r) => linker_response(s, Some(&r), StatusCode::OK),
                Err(r) => {
                    s.mode = LinkerMode::Fault;
                    linker_response(s, Some(&r), mutation_response_status(&r))
                }
            }
        }
        "naming" => {
            s.mode = LinkerMode::Naming;
            s.rename_naming = false;
            linker_response(s, None, StatusCode::OK)
        }
        "delete-review" => {
            s.mode = LinkerMode::DeleteReview;
            linker_response(s, None, StatusCode::OK)
        }
        "deploy-review" => {
            s.mode = LinkerMode::DeployReview;
            linker_response(s, None, StatusCode::OK)
        }
        "cancel" => {
            s.mode = LinkerMode::Browsing;
            s.rename_naming = false;
            s.feedback = "Review cancelled.".into();
            linker_response(s, None, StatusCode::OK)
        }
        "mkdir" => {
            let r = linker_call(
                &h,
                "coronatio.linker.mkdir",
                "/api/v1/linker/mkdir",
                serde_json::json!({"path":s.path,"new_dir_name":f.new_dir_name.unwrap_or_default()}),
            );
            linker_finish(s, r, "New directory", false)
        }
        "rename" => {
            if f.new_name.is_none() {
                if s.selected.len() != 1 {
                    s.feedback = "Rename requires exactly one selected entry.".into();
                    return linker_response(s, None, StatusCode::BAD_REQUEST);
                }
                s.mode = LinkerMode::Naming;
                s.rename_naming = true;
                return linker_response(s, None, StatusCode::OK);
            }
            if s.selected.len() != 1 {
                s.feedback = "Rename requires exactly one selected entry.".into();
                return linker_response(s, None, StatusCode::BAD_REQUEST);
            }
            let r = linker_call(
                &h,
                "coronatio.linker.rename",
                "/api/v1/linker/rename",
                serde_json::json!({"path":s.selected[0],"new_name":f.new_name.unwrap_or_default()}),
            );
            linker_finish(s, r, "Rename", true)
        }
        "delete" => {
            if s.selected.is_empty() {
                s.feedback = "Delete requires at least one selected entry.".into();
                return linker_response(s, None, StatusCode::BAD_REQUEST);
            }
            let mut last = None;
            let mut all_succeeded = true;
            for p in s.selected.clone() {
                let r = linker_call(
                    &h,
                    "coronatio.linker.delete",
                    "/api/v1/linker/delete",
                    serde_json::json!({"path":p}),
                );
                if !r.ok {
                    all_succeeded = false;
                    last = Some(r);
                } else if last.is_none() {
                    last = Some(r);
                }
            }
            linker_finish(s, last.unwrap(), "Delete", all_succeeded)
        }
        "deploy" => {
            if s.selected.is_empty() || s.destination.is_none() {
                s.feedback = "Select sources and set a destination first.".into();
                return linker_response(s, None, StatusCode::BAD_REQUEST);
            }
            let r = linker_call(
                &h,
                "coronatio.linker.deploy",
                "/api/v1/linker/deploy",
                serde_json::json!({"sources":s.selected,"destination":s.destination,"conflict_strategy":"rename"}),
            );
            linker_finish(s, r, "Deploy", true)
        }
        _ => {
            s.mode = LinkerMode::Fault;
            s.feedback = format!("Unknown linker action: {}", action);
            linker_response(s, None, StatusCode::BAD_REQUEST)
        }
    }
}
