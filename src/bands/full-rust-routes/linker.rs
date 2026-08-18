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
    Result,
    Fault,
}
impl LinkerMode {
    fn as_str(self) -> &'static str {
        match self {
            Self::Browsing => "Browsing",
            Self::Naming => "Naming",
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
    feedback: String,
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
    let rows = if s.entries.is_empty() {
        "<p>No entries observed.</p>".to_string()
    } else {
        s.entries.iter().map(|e| {
        let checked=if s.selected.iter().any(|p|p==&e.path){"checked"}else{""}; let nav=if e.is_dir{format!("hx-get=\"/admit/linker?path={}\" hx-target=\"#linker-fragment\" hx-swap=\"outerHTML\"",linker_url(&e.path))}else{String::new()}; let linked=if e.is_hardlink||e.nlink>1{format!("<span class=\"linker-linked\">LINKED ×{}</span>",e.nlink)}else{String::new()};
        format!("<div class=\"{}\" data-linker-entry data-linker-directory=\"{}\" data-linker-path=\"{}\" tabindex=\"-1\" {}><span class=\"linker-cursor\">›</span><form method=\"post\" action=\"/admit/linker/select\"><input type=\"hidden\" name=\"path\" value=\"{}\"><input type=\"checkbox\" name=\"checked\" value=\"on\" aria-label=\"Select {}\" {} hx-post=\"/admit/linker/select\" hx-target=\"#linker-fragment\" hx-swap=\"outerHTML\" hx-trigger=\"click consume\" hx-include=\"closest form\" onclick=\"event.stopPropagation()\"></form><span class=\"entry-icon\">{}</span><span class=\"entry-name\">{}</span>{}</div>",if e.is_dir{"linker-row linker-directory"}else{"linker-row"},e.is_dir,linker_escape(&e.path),nav,linker_escape(&e.path),linker_escape(&e.name),checked,if e.is_dir{"📁"}else{"📄"},linker_escape(&e.name),linked)
    }).collect::<Vec<_>>().join("")
    };
    let segments = s
        .path
        .split('/')
        .filter(|p| !p.is_empty())
        .collect::<Vec<_>>();
    let mut prefix = String::new();
    let trail=segments.iter().enumerate().map(|(i,n)|{prefix.push('/');prefix.push_str(n);if i+1==segments.len(){linker_escape(n)}else{format!("<a href=\"/admit/linker?path={}\" hx-get=\"/admit/linker?path={}\" hx-target=\"#linker-fragment\" hx-swap=\"outerHTML\">{}</a>",linker_url(&prefix),linker_url(&prefix),linker_escape(n))}}).collect::<Vec<_>>().join(" / ");
    let tree=s.entries.iter().filter(|e|e.is_dir).map(|e|format!("<div class=\"directory-entry\" role=\"treeitem\" hx-get=\"/admit/linker?path={}\" hx-target=\"#linker-fragment\" hx-swap=\"outerHTML\"><span class=\"entry-icon\">📁</span><span class=\"entry-name\">{}</span></div>",linker_url(&e.path),linker_escape(&e.name))).collect::<Vec<_>>().join("");
    let naming = if s.mode == LinkerMode::Naming {
        if s.rename_naming {
            "<form class=\"linker-inline-form\" method=\"post\" action=\"/admit/linker/rename\" hx-post=\"/admit/linker/rename\" hx-target=\"#linker-fragment\" hx-swap=\"outerHTML\"><input name=\"new_name\" required><button>Rename</button><button formaction=\"/admit/linker/cancel\" hx-post=\"/admit/linker/cancel\" hx-target=\"#linker-fragment\" hx-swap=\"outerHTML\">Cancel</button></form>".into()
        } else {
            "<form class=\"linker-inline-form\" method=\"post\" action=\"/admit/linker/mkdir\" hx-post=\"/admit/linker/mkdir\" hx-target=\"#linker-fragment\" hx-swap=\"outerHTML\"><input name=\"new_dir_name\" required><button>Create</button><button formaction=\"/admit/linker/cancel\" hx-post=\"/admit/linker/cancel\" hx-target=\"#linker-fragment\" hx-swap=\"outerHTML\">Cancel</button></form>".into()
        }
    } else {
        String::new()
    };
    let disabled_one = if s.selected.len() == 1 {
        ""
    } else {
        "disabled title=\"Select exactly one entry\""
    };
    let disabled_delete = if s.selected.is_empty() {
        "disabled title=\"Select at least one entry to delete\""
    } else if s
        .selected
        .iter()
        .any(|p| !s.selected_delete_eligible.get(p).copied().unwrap_or(false))
    {
        "disabled title=\"Delete disabled: selected files must be hardlinked; directories are allowed\""
    } else {
        ""
    };
    let script = r#"<style>.linker-cursor{visibility:hidden}.linker-row.cursor{outline:2px solid currentColor;background:color-mix(in srgb,currentColor 10%,transparent)}.linker-row.cursor>.linker-cursor{visibility:visible}</style><script>(function(){const x=document.getElementById('linker-fragment');if(!x)return;let r=[...x.querySelectorAll('[data-linker-entry]')],i=0;const next=window.__linkerCursorNext;if(next){const n=r.findIndex(z=>z.dataset.linkerPath===next);if(n>=0)i=(n+1)%r.length;window.__linkerCursorNext=null;}const f=()=>{r=[...x.querySelectorAll('[data-linker-entry]')];if(r.length){i=(i+r.length)%r.length;r.forEach((z,n)=>z.classList.toggle('cursor',n===i));r[i].focus()}};f();x.addEventListener('keydown',e=>{if(e.target.closest('input,textarea,select,button,[contenteditable]'))return;r=[...x.querySelectorAll('[data-linker-entry]')];if(!r.length)return;const k=e.key.toLowerCase(),cur=r[i],go=p=>htmx.ajax('GET','/admit/linker?path='+encodeURIComponent(p),{target:'#linker-fragment',swap:'outerHTML'});if(k==='j'||k==='arrowdown'){e.preventDefault();i=(i+1)%r.length;f()}else if(k==='k'||k==='arrowup'){e.preventDefault();i=(i+r.length-1)%r.length;f()}else if(k==='h'||k==='arrowleft'){e.preventDefault();const p=x.dataset.linkerPath||'/';if(p!=='/'){const q=p.replace(/\/+$/,'').split('/');q.pop();go(q.join('/')||'/')}}else if((k==='l'||k==='arrowright'||k==='enter')&&cur.dataset.linkerDirectory==='true'){e.preventDefault();cur.click()}else if(k===' '){e.preventDefault();window.__linkerCursorNext=cur.dataset.linkerPath;cur.querySelector('input[type=checkbox]').click()}})})();</script>"#;
    format!(
        r###"<section id="linker-fragment" data-linker-state="{}" data-linker-path="{}"><header><h2>Linker</h2><nav aria-label="Ancestor trail"><a href="/admit/linker" hx-get="/admit/linker" hx-target="#linker-fragment" hx-swap="outerHTML">/</a>{}</nav><p>Current path: {} · Visible: {} · Selected: {} · Last feedback: <span aria-live="polite">{}</span></p></header><nav class="directory-tree-container" role="tree" data-linker-tree aria-label="Child directories">{}</nav><form method="get" action="/admit/linker" hx-get="/admit/linker" hx-target="#linker-fragment" hx-swap="outerHTML"><label>Name filter <input name="filter" type="search" value="{}"></label><input type="hidden" name="path" value="{}"><button>Filter</button></form><div class="linker-list" role="list">{}</div>{}<form class="linker-action-shelf" method="post"><button formaction="/admit/linker/deploy" hx-post="/admit/linker/deploy" hx-target="#linker-fragment" hx-swap="outerHTML" {}>Deploy selected</button><button formaction="/admit/linker/naming" hx-post="/admit/linker/naming" hx-target="#linker-fragment" hx-swap="outerHTML">New directory</button><button formaction="/admit/linker/rename" hx-post="/admit/linker/rename" hx-target="#linker-fragment" hx-swap="outerHTML" {}>Rename selected</button><button formaction="/admit/linker/delete" hx-post="/admit/linker/delete" hx-target="#linker-fragment" hx-swap="outerHTML" {}>Delete selected</button></form>{}{}<p hidden>Browsing Naming Result Fault</p></section>"###,
        s.mode.as_str(),
        linker_escape(&s.path),
        trail,
        linker_escape(&s.path),
        s.entries.len(),
        s.selected.len(),
        linker_escape(&s.feedback),
        tree,
        linker_escape(&s.filter),
        linker_escape(&s.path),
        rows,
        naming,
        if s.selected.is_empty() {
            "disabled title=\"Select sources first\""
        } else {
            ""
        },
        disabled_one,
        disabled_delete,
        readback.map(linker_receipt_html).unwrap_or_default(),
        script
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
    h: &axum::http::HeaderMap,
    s: &mut LinkerState,
    r: CaduceusHttpReadback,
    label: &str,
    clear_selection: bool,
) -> Response {
    if r.ok {
        if clear_selection {
            s.selected.clear();
            s.selected_delete_eligible.clear();
            if let Err(browse) = linker_browse(h, s) {
                s.mode = LinkerMode::Fault;
                s.feedback = browse.first_missing_signal.clone();
                return linker_response(s, Some(&browse), mutation_response_status(&browse));
            }
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
                    let eligible = s
                        .entries
                        .iter()
                        .find(|e| e.path == p)
                        .map(|e| e.is_dir || e.nlink > 1)
                        .unwrap_or(false);
                    s.selected.push(p.clone());
                    s.selected_delete_eligible.insert(p, eligible);
                }
            } else {
                s.selected.retain(|x| x != &p);
                s.selected_delete_eligible.remove(&p);
            }
            s.mode = LinkerMode::Browsing;
            match linker_browse(&h, s) {
                Ok(r) => linker_response(s, Some(&r), StatusCode::OK),
                Err(r) => {
                    s.mode = LinkerMode::Fault;
                    s.feedback = r.first_missing_signal.clone();
                    linker_response(s, Some(&r), mutation_response_status(&r))
                }
            }
        }
        "browse" => {
            s.mode = LinkerMode::Browsing;
            match linker_browse(&h, s) {
                Ok(r) => linker_response(s, Some(&r), StatusCode::OK),
                Err(r) => {
                    s.mode = LinkerMode::Fault;
                    s.feedback = r.first_missing_signal.clone();
                    linker_response(s, Some(&r), mutation_response_status(&r))
                }
            }
        }
        "naming" => {
            s.mode = LinkerMode::Naming;
            s.rename_naming = false;
            linker_response(s, None, StatusCode::OK)
        }
        "cancel" => {
            s.mode = LinkerMode::Browsing;
            s.rename_naming = false;
            s.feedback = "Cancelled.".into();
            linker_response(s, None, StatusCode::OK)
        }
        "mkdir" => {
            let r = linker_call(
                &h,
                "coronatio.linker.mkdir",
                "/api/v1/linker/mkdir",
                serde_json::json!({"path":s.path,"new_dir_name":f.new_dir_name.unwrap_or_default()}),
            );
            linker_finish(&h, s, r, "New directory", false)
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
            linker_finish(&h, s, r, "Rename", true)
        }
        "delete" => {
            if s.selected.is_empty() {
                s.feedback = "Delete requires at least one selected entry.".into();
                return linker_response(s, None, StatusCode::BAD_REQUEST);
            }
            if s.selected
                .iter()
                .any(|p| !s.selected_delete_eligible.get(p).copied().unwrap_or(false))
            {
                s.feedback = "Delete is disabled for a selected entry that is not eligible.".into();
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
            linker_finish(&h, s, last.unwrap(), "Delete", all_succeeded)
        }
        "deploy" => {
            if s.selected.is_empty() {
                s.feedback = "Select sources first.".into();
                return linker_response(s, None, StatusCode::BAD_REQUEST);
            }
            let r = linker_call(
                &h,
                "coronatio.linker.deploy",
                "/api/v1/linker/deploy",
                serde_json::json!({"sources":s.selected,"destination":s.path,"conflict_strategy":"rename"}),
            );
            linker_finish(&h, s, r, "Deploy", true)
        }
        _ => {
            s.mode = LinkerMode::Fault;
            s.feedback = format!("Unknown linker action: {}", action);
            linker_response(s, None, StatusCode::BAD_REQUEST)
        }
    }
}
