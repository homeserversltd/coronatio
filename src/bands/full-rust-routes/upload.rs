#[derive(Debug, Deserialize)]
struct UploadBrowseQuery {
    path: Option<String>,
    expand: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct UploadTreeQuery {
    path: Option<String>,
    depth: Option<usize>,
    selected: Option<String>,
    expanded: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct UploadDirectoryEntry {
    name: String,
    path: String,
    #[serde(rename = "type")]
    entry_type: String,
    has_children: bool,
    is_expanded: bool,
    children: Option<Vec<UploadDirectoryEntry>>,
}

fn upload_root_path() -> PathBuf {
    env::var("CORONATIO_UPLOAD_ROOT")
        .ok()
        .filter(|path| !path.trim().is_empty())
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("/mnt/nas"))
}

fn upload_display_root(root: &FsPath) -> String {
    if env::var("CORONATIO_UPLOAD_ROOT").is_ok() {
        root.display().to_string()
    } else {
        "/mnt/nas".to_string()
    }
}

fn upload_resolve_path(root: &FsPath, display_root: &str, requested: &str) -> Option<PathBuf> {
    if requested == display_root || requested == "/mnt/nas" {
        return Some(root.to_path_buf());
    }
    let suffix = requested
        .strip_prefix(&(display_root.to_string() + "/"))
        .or_else(|| requested.strip_prefix("/mnt/nas/"))?;
    if suffix.split('/').any(|part| part.is_empty() || part == "." || part == "..") {
        return None;
    }
    Some(root.join(suffix))
}

fn upload_display_path(root: &FsPath, display_root: &str, real_path: &FsPath) -> String {
    match real_path.strip_prefix(root) {
        Ok(relative) if relative.as_os_str().is_empty() => display_root.to_string(),
        Ok(relative) => format!("{}/{}", display_root.trim_end_matches('/'), relative.display()),
        Err(_) => display_root.to_string(),
    }
}

fn upload_directory_has_children(path: &FsPath) -> bool {
    std::fs::read_dir(path)
        .ok()
        .into_iter()
        .flat_map(|entries| entries.filter_map(Result::ok))
        .any(|entry| entry.file_type().map(|kind| kind.is_dir()).unwrap_or(false))
}

fn upload_immediate_children(root: &FsPath, display_root: &str, path: &FsPath) -> Vec<UploadDirectoryEntry> {
    let mut entries = std::fs::read_dir(path)
        .ok()
        .into_iter()
        .flat_map(|entries| entries.filter_map(Result::ok))
        .filter(|entry| entry.file_type().map(|kind| kind.is_dir()).unwrap_or(false))
        .filter(|entry| !upload_path_blacklisted(&upload_display_path(root, display_root, &entry.path())))
        .map(|entry| {
            let real_path = entry.path();
            UploadDirectoryEntry {
                name: entry.file_name().to_string_lossy().to_string(),
                path: upload_display_path(root, display_root, &real_path),
                entry_type: "directory".to_string(),
                has_children: upload_directory_has_children(&real_path),
                is_expanded: false,
                children: None,
            }
        })
        .collect::<Vec<_>>();
    entries.sort_by_key(|entry| entry.name.to_lowercase());
    entries
}

fn upload_html_escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

fn upload_query_escape(value: &str) -> String {
    let mut encoded = String::new();
    for byte in value.bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b'~') {
            encoded.push(byte as char);
        } else {
            encoded.push_str(&format!("%{byte:02X}"));
        }
    }
    encoded
}

fn upload_expanded_set(raw: Option<&str>) -> BTreeSet<String> {
    raw.unwrap_or("")
        .split(',')
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
        .collect()
}

fn upload_expanded_csv(expanded: &BTreeSet<String>) -> String {
    expanded.iter().cloned().collect::<Vec<_>>().join(",")
}

fn upload_tree_url(path: &str, depth: usize) -> String {
    format!("/admit/upload/tree?path={}&depth={}", upload_query_escape(path), depth)
}

fn upload_tree_state_inputs(selected: &str, expanded: &BTreeSet<String>) -> String {
    format!(
        r#"<input type="hidden" name="selected" data-upload-current-path value="{}"><input type="hidden" name="expanded" data-upload-expanded-paths value="{}">"#,
        upload_html_escape(selected),
        upload_html_escape(&upload_expanded_csv(expanded))
    )
}

fn upload_tree_subtree_id(path: &str) -> String {
    let mut id = String::from("upload-subtree-");
    for byte in path.bytes() {
        if byte.is_ascii_alphanumeric() { id.push(byte as char); }
        else { id.push_str(&format!("-{byte:02x}")); }
    }
    id
}

fn upload_tree_row_html(entry: &UploadDirectoryEntry, depth: usize, selected: &str, expanded: &BTreeSet<String>, _display_root: &str) -> String {
    let is_selected = entry.path == selected;
    let is_expanded = expanded.contains(&entry.path);
    let subtree_id = upload_tree_subtree_id(&entry.path);
    let expand_url = upload_tree_url(&entry.path, depth + 1);
    let selection_url = upload_tree_url(&entry.path, 0);
    let indent = 24 * depth + 12;
    let caret = if entry.has_children {
        format!(
            r##"<button type="button" class="expand-control" aria-label="{}" aria-expanded="{}" hx-get="{}" hx-include="closest [data-upload-tree]" hx-target="[data-upload-tree]" hx-swap="innerHTML" hx-trigger="click consume">{}</button>"##,
            if is_expanded { "Collapse" } else { "Expand" },
            is_expanded,
            upload_html_escape(&expand_url),
            if is_expanded { "▼" } else { "▶" }
        )
    } else {
        r#"<span class="expand-control" aria-label="No child folders"></span>"#.to_string()
    };
    format!(
        r#"<div class="directory-entry{}" data-directory-path="{}" role="treeitem" aria-selected="{}" aria-expanded="{}" style="padding-left: {}px" hx-get="{}" hx-include="closest [data-upload-tree]" hx-target="[data-upload-tree]" hx-swap="innerHTML">{}<span class="entry-icon">📁</span><span class="entry-name">{}</span><span class="entry-selected" aria-hidden="true"{}>✓</span></div><div id="{}" class="directory-subtree" data-upload-subtree="{}">{}</div>"#,
        if is_selected { " selected" } else { "" },
        upload_html_escape(&entry.path),
        is_selected,
        if entry.has_children { is_expanded.to_string() } else { "false".to_string() },
        indent,
        upload_html_escape(&selection_url),
        caret,
        upload_html_escape(&entry.name),
        if is_selected { "" } else { " hidden" },
        upload_html_escape(&subtree_id),
        upload_html_escape(&entry.path),
        if is_expanded { render_upload_tree_rows_for_path(&entry.path, depth + 1, selected, expanded) } else { String::new() }
    )
}

fn render_upload_tree_rows_for_path(path: &str, depth: usize, selected: &str, expanded: &BTreeSet<String>) -> String {
    let root = upload_root_path();
    let display_root = upload_display_root(&root);
    let Some(real_path) = upload_resolve_path(&root, &display_root, path) else {
        return r#"<div class="directory-error" data-upload-tree-error="invalid-path">Invalid directory</div>"#.to_string();
    };
    if !real_path.is_dir() {
        return String::new();
    }
    upload_immediate_children(&root, &display_root, &real_path)
        .iter()
        .map(|entry| upload_tree_row_html(entry, depth, selected, expanded, &display_root))
        .collect::<Vec<_>>()
        .join("")
}

fn render_upload_tree_fragment(selected: Option<&str>, expanded: Option<&str>) -> String {
    let root = upload_root_path();
    let display_root = upload_display_root(&root);
    let configured = upload_config_value();
    let configured_default = upload_data(&configured)
        .and_then(|data| data.get("default-directory").or_else(|| data.get("defaultDirectory")))
        .and_then(serde_json::Value::as_str)
        .unwrap_or(&display_root);
    let selected = selected.unwrap_or(configured_default);
    if !upload_root_available(&root) {
        return format!(
            r#"{}<div class="directory-error nas-unavailable" data-nas-unavailable="true" data-upload-directory-error>⚠️ NAS Storage Unavailable</div><div class="directory-entry selected" data-directory-path="{}" role="treeitem" aria-selected="true" aria-expanded="false" style="padding-left: 12px"><span class="expand-control" aria-label="No child folders"></span><span class="entry-icon">📁</span><span class="entry-name">nas</span><span class="entry-selected" aria-hidden="true">✓</span></div>"#,
            upload_tree_state_inputs(selected, &BTreeSet::new()),
            upload_html_escape(&display_root)
        );
    }
    let mut expanded = upload_expanded_set(expanded);
    expanded.insert(display_root.clone());
    let root_entry = UploadDirectoryEntry {
        name: root.file_name().map(|name| name.to_string_lossy().to_string()).filter(|name| !name.is_empty()).unwrap_or_else(|| "nas".to_string()),
        path: display_root.clone(),
        entry_type: "directory".to_string(),
        has_children: upload_directory_has_children(&root),
        is_expanded: true,
        children: None,
    };
    format!(
        r#"{}<div class="directory-error nas-unavailable" data-nas-unavailable="true" data-upload-directory-error hidden>⚠️ NAS Storage Unavailable</div>{}"#,
        upload_tree_state_inputs(selected, &expanded),
        upload_tree_row_html(&root_entry, 0, selected, &expanded, &display_root)
    )
}

/// Canonical hypermedia exemplar: the DOM is the index, hx-* attributes are child pointers, and the server is the resolver.
async fn upload_tree_fragment_route(headers: axum::http::HeaderMap, Query(query): Query<UploadTreeQuery>) -> impl IntoResponse {
    let root = upload_root_path();
    let display_root = upload_display_root(&root);
    let path = query.path.as_deref().unwrap_or(&display_root);
    let depth = query.depth.unwrap_or(0);
    let is_hx_request = headers.get("hx-request").and_then(|value| value.to_str().ok()).map(|value| value.eq_ignore_ascii_case("true")).unwrap_or(false);
    let body = if depth == 0 {
        let selected = if path == display_root { query.selected.as_deref().unwrap_or(path) } else { path };
        render_upload_tree_fragment(Some(selected), query.expanded.as_deref())
    } else if is_hx_request {
        let selected = query.selected.as_deref().unwrap_or(&display_root);
        let mut expanded = upload_expanded_set(query.expanded.as_deref());
        expanded.insert(display_root.clone());
        if path != display_root {
            if !expanded.remove(path) { expanded.insert(path.to_string()); }
        }
        let expanded_csv = upload_expanded_csv(&expanded);
        render_upload_tree_fragment(Some(selected), Some(&expanded_csv))
    } else {
        let selected = query.selected.as_deref().unwrap_or(&display_root);
        let expanded = upload_expanded_set(query.expanded.as_deref());
        render_upload_tree_rows_for_path(path, depth, selected, &expanded)
    };
    let mut response = Html(body).into_response();
    response.headers_mut().insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    response.headers_mut().insert(header::CONTENT_SECURITY_POLICY, HeaderValue::from_static(CROWN_CONTENT_SECURITY_POLICY));
    response
}

async fn upload_browse_hierarchical_route(Query(query): Query<UploadBrowseQuery>) -> impl IntoResponse {
    let root = upload_root_path();
    let display_root = upload_display_root(&root);
    let requested = query.path.as_deref().unwrap_or(&display_root);
    let expand = query.expand.unwrap_or(false);
    if upload_path_blacklisted(requested) {
        return (StatusCode::FORBIDDEN, Json(serde_json::json!({
            "schema":"coronatio.upload.browse_hierarchical.v1","ok":false,"success":false,
            "entries":[],"firstMissingSignal":"upload-path-blacklisted"
        }))).into_response();
    }
    let Some(real_path) = upload_resolve_path(&root, &display_root, requested) else {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "schema": "coronatio.upload.browse_hierarchical.v1",
                "ok": false,
                "success": false,
                "path": requested,
                "entries": [],
                "parent": null,
                "hasChildren": false,
                "error": "Invalid directory",
                "firstMissingSignal": "invalid-upload-path"
            })),
        )
            .into_response();
    };
    if !real_path.is_dir() {
        return Json(serde_json::json!({
            "schema": "coronatio.upload.browse_hierarchical.v1",
            "ok": true,
            "success": true,
            "nasUnavailable": !root.is_dir(),
            "path": requested,
            "root": display_root,
            "entries": [],
            "parent": null,
            "hasChildren": false,
            "firstMissingSignal": "upload-path-absent"
        }))
        .into_response();
    }
    let entries = upload_immediate_children(&root, &display_root, &real_path);
    Json(serde_json::json!({
        "schema": "coronatio.upload.browse_hierarchical.v1",
        "ok": true,
        "success": true,
        "path": upload_display_path(&root, &display_root, &real_path),
        "root": display_root,
        "entries": entries,
        "parent": if real_path == root { serde_json::Value::Null } else { serde_json::Value::String(upload_display_path(&root, &display_root, real_path.parent().unwrap_or(&root))) },
        "hasChildren": !entries.is_empty(),
        "expanded": expand,
        "firstMissingSignal": "none"
    }))
    .into_response()
}



fn upload_config_value() -> serde_json::Value {
    read_first_json(&homeserver_config_candidates()).map(|(_, value)| value).unwrap_or_else(|_| serde_json::json!({}))
}

fn upload_data(value: &serde_json::Value) -> Option<&serde_json::Value> {
    value.pointer("/tabs/upload/data")
}

fn upload_blacklist() -> Vec<String> {
    let value = upload_config_value();
    upload_data(&value).and_then(|data| data.get("blacklist")).and_then(serde_json::Value::as_array)
        .map(|items| items.iter().filter_map(serde_json::Value::as_str).map(str::to_string).collect()).unwrap_or_default()
}

fn upload_path_blacklisted(path: &str) -> bool {
    upload_blacklist().iter().any(|blocked| path == blocked || path.starts_with(&(blocked.trim_end_matches('/').to_string() + "/")))
}

fn upload_root_available(root: &std::path::Path) -> bool {
    if !root.is_dir() { return false; }
    if std::env::var("CORONATIO_UPLOAD_ROOT").is_ok() { return true; }
    std::process::Command::new("mountpoint").args(["-q", root.to_string_lossy().as_ref()]).status().map(|status| status.success()).unwrap_or(false)
}

fn update_upload_config(key: &str, value: serde_json::Value) -> Result<(), String> {
    let candidates = homeserver_config_candidates();
    let (path, mut document) = read_first_json(&candidates)?;
    let root = document.as_object_mut().ok_or_else(|| "homeserver-config-root-invalid".to_string())?;
    let tabs = root.entry("tabs").or_insert_with(|| serde_json::json!({})).as_object_mut().ok_or_else(|| "homeserver-config-tabs-invalid".to_string())?;
    let upload = tabs.entry("upload").or_insert_with(|| serde_json::json!({})).as_object_mut().ok_or_else(|| "homeserver-config-upload-invalid".to_string())?;
    let data = upload.entry("data").or_insert_with(|| serde_json::json!({})).as_object_mut().ok_or_else(|| "homeserver-config-upload-data-invalid".to_string())?;
    data.insert(key.to_string(), value);
    let rendered = serde_json::to_string_pretty(&document).map_err(|error| error.to_string())? + "\n";
    std::fs::write(&path, rendered).map_err(|error| format!("{}: {error}", path.display()))
}

fn append_upload_log(line: &str) -> Result<(), String> {
    use std::io::Write;
    let log_dir = std::env::var("HOMESERVER_LOG_DIR").unwrap_or_else(|_| "/var/log/homeserver".to_string());
    std::fs::create_dir_all(&log_dir).map_err(|error| error.to_string())?;
    let mut log = std::fs::OpenOptions::new().create(true).append(true).open(std::path::Path::new(&log_dir).join("upload.log")).map_err(|error| error.to_string())?;
    writeln!(log, "{line}").map_err(|error| error.to_string())
}

async fn upload_default_directory_route() -> impl IntoResponse {
    let value = upload_config_value();
    let path = upload_data(&value).and_then(|data| data.get("default-directory").or_else(|| data.get("defaultDirectory"))).and_then(serde_json::Value::as_str).unwrap_or("/mnt/nas");
    Json(serde_json::json!({"schema":"coronatio.upload.default_directory.v1","ok":true,"defaultPath":path,"firstMissingSignal":"none"}))
}

async fn upload_default_directory_update_route(headers: axum::http::HeaderMap, Json(body): Json<serde_json::Value>) -> Response {
    if session_from_headers(&headers) != Session::Admin { return upload_admin_read_refusal_response("/api/upload/default-directory"); }
    let Some(path) = body.get("path").or_else(|| body.get("defaultPath")).and_then(serde_json::Value::as_str) else { return (StatusCode::BAD_REQUEST, "default-directory-missing").into_response(); };
    match update_upload_config("default-directory", serde_json::json!(path)) { Ok(()) => Json(serde_json::json!({"ok":true,"defaultPath":path,"firstMissingSignal":"none"})).into_response(), Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error).into_response() }
}

async fn upload_blacklist_admin_route(headers: axum::http::HeaderMap) -> Response {
    if session_from_headers(&headers) != Session::Admin { return upload_admin_read_refusal_response("/api/upload/blacklist/list"); }
    Json(serde_json::json!({"schema":"coronatio.upload.blacklist.v1","ok":true,"blacklist":upload_blacklist(),"firstMissingSignal":"none"})).into_response()
}

fn upload_blacklist_update(body: serde_json::Value) -> Response {
    let Some(list) = body.get("blacklist").or_else(|| body.get("paths")).and_then(serde_json::Value::as_array) else { return (StatusCode::BAD_REQUEST, "upload-blacklist-missing").into_response(); };
    if list.iter().any(|value| value.as_str().is_none()) { return (StatusCode::BAD_REQUEST, "upload-blacklist-invalid").into_response(); }
    match update_upload_config("blacklist", serde_json::Value::Array(list.clone())) { Ok(()) => Json(serde_json::json!({"ok":true,"blacklist":list,"firstMissingSignal":"none"})).into_response(), Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error).into_response() }
}

async fn upload_pin_required_route() -> impl IntoResponse {
    let value = upload_config_value();
    let required = upload_data(&value).and_then(|data| data.get("isPinRequired").or_else(|| data.get("pinRequired"))).and_then(serde_json::Value::as_bool).unwrap_or(false);
    Json(serde_json::json!({"schema":"coronatio.upload.pin_required.v1","ok":true,"isPinRequired":required,"firstMissingSignal":"none"}))
}

fn upload_pin_required_update(body: serde_json::Value) -> Response {
    let Some(required) = body.get("isPinRequired").or_else(|| body.get("required")).and_then(serde_json::Value::as_bool) else { return (StatusCode::BAD_REQUEST, "upload-pin-required-missing").into_response(); };
    match update_upload_config("isPinRequired", serde_json::json!(required)) { Ok(()) => Json(serde_json::json!({"ok":true,"isPinRequired":required,"firstMissingSignal":"none"})).into_response(), Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error).into_response() }
}

fn upload_force_permissions(body: serde_json::Value) -> Response {
    let destination = body.get("path").or_else(|| body.get("destination")).and_then(serde_json::Value::as_str).unwrap_or("/mnt/nas");
    let caduceus = caduceus_http_json("POST", "/api/v1/staff/intent", serde_json::json!({"method":"POST","route":"/api/upload/force-permissions","classification":"force-permissions","metadata":{"destination":destination}}));
    (if caduceus.ok { StatusCode::OK } else { StatusCode::SERVICE_UNAVAILABLE }, Json(serde_json::json!({"ok":caduceus.ok,"caduceus":caduceus,"firstMissingSignal":if caduceus.ok { "none".to_string() } else { caduceus.first_missing_signal }}))).into_response()
}

