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
    expanded.iter().map(|path| upload_query_escape(path)).collect::<Vec<_>>().join(",")
}

fn upload_tree_url(path: &str, depth: usize, selected: &str, expanded: &BTreeSet<String>) -> String {
    format!(
        "/admit/upload/tree?path={}&depth={}&selected={}&expanded={}",
        upload_query_escape(path),
        depth,
        upload_query_escape(selected),
        upload_expanded_csv(expanded)
    )
}

fn upload_tree_selection_url(selected: &str, expanded: &BTreeSet<String>, display_root: &str) -> String {
    format!(
        "/admit/upload/tree?path={}&depth=0&selected={}&expanded={}",
        upload_query_escape(display_root),
        upload_query_escape(selected),
        upload_expanded_csv(expanded)
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

fn upload_tree_row_html(entry: &UploadDirectoryEntry, depth: usize, selected: &str, expanded: &BTreeSet<String>, display_root: &str) -> String {
    let is_selected = entry.path == selected;
    let is_expanded = expanded.contains(&entry.path);
    let subtree_id = upload_tree_subtree_id(&entry.path);
    let mut next_expanded = expanded.clone();
    next_expanded.insert(entry.path.clone());
    let expand_url = upload_tree_url(&entry.path, depth + 1, selected, &next_expanded);
    let selection_url = upload_tree_selection_url(&entry.path, expanded, display_root);
    let indent = 24 * depth + 12;
    let caret = if entry.has_children {
        format!(
            r##"<button type="button" class="expand-control" aria-label="{}" aria-expanded="{}" hx-get="{}" hx-target="#{}" hx-swap="innerHTML">{}</button>"##,
            if is_expanded { "Refresh subtree" } else { "Expand" },
            is_expanded,
            upload_html_escape(&expand_url),
            upload_html_escape(&subtree_id),
            if is_expanded { "▼" } else { "▶" }
        )
    } else {
        r#"<span class="expand-control" aria-label="No child folders"></span>"#.to_string()
    };
    format!(
        r#"<div class="directory-entry{}" data-directory-path="{}" role="treeitem" aria-selected="{}" aria-expanded="{}" style="padding-left: {}px" hx-get="{}" hx-target="[data-upload-tree]" hx-swap="innerHTML">{}<span class="entry-icon">📁</span><span class="entry-name">{}</span><span class="entry-selected" aria-hidden="true"{}>✓</span></div><div id="{}" class="directory-subtree" data-upload-subtree="{}">{}</div>"#,
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
    let selected = selected.unwrap_or(&display_root);
    if !root.is_dir() {
        return format!(
            r#"<input type="hidden" data-upload-current-path value="{}"><div class="directory-error nas-unavailable" data-nas-unavailable="true" data-upload-directory-error>⚠️ NAS Storage Unavailable</div><div class="directory-entry selected" data-directory-path="{}" role="treeitem" aria-selected="true" aria-expanded="false" style="padding-left: 12px"><span class="expand-control" aria-label="No child folders"></span><span class="entry-icon">📁</span><span class="entry-name">nas</span><span class="entry-selected" aria-hidden="true">✓</span></div>"#,
            upload_html_escape(selected),
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
        r#"<input type="hidden" data-upload-current-path value="{}"><div class="directory-error nas-unavailable" data-nas-unavailable="true" data-upload-directory-error hidden>⚠️ NAS Storage Unavailable</div>{}"#,
        upload_html_escape(selected),
        upload_tree_row_html(&root_entry, 0, selected, &expanded, &display_root)
    )
}

/// Canonical hypermedia exemplar: the DOM is the index, hx-* attributes are child pointers, and the server is the resolver.
async fn upload_tree_fragment_route(Query(query): Query<UploadTreeQuery>) -> impl IntoResponse {
    let root = upload_root_path();
    let display_root = upload_display_root(&root);
    let selected = query.selected.as_deref().unwrap_or(&display_root);
    let path = query.path.as_deref().unwrap_or(&display_root);
    let depth = query.depth.unwrap_or(0);
    let body = if depth == 0 && path == display_root {
        render_upload_tree_fragment(Some(selected), query.expanded.as_deref())
    } else {
        let expanded = upload_expanded_set(query.expanded.as_deref());
        render_upload_tree_rows_for_path(path, depth, selected, &expanded)
    };
    let mut response = Html(body).into_response();
    response.headers_mut().insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    response
}

async fn upload_browse_hierarchical_route(Query(query): Query<UploadBrowseQuery>) -> impl IntoResponse {
    let root = upload_root_path();
    let display_root = upload_display_root(&root);
    let requested = query.path.as_deref().unwrap_or(&display_root);
    let expand = query.expand.unwrap_or(false);
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
