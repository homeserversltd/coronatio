#[derive(Debug, Deserialize)]
struct UploadBrowseQuery {
    path: Option<String>,
    expand: Option<bool>,
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
