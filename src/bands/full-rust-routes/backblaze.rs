// Backblaze is a face for the Caduceus doors; the config seat is read-only here.
const BACKBLAZE_DOOR: &str = "/api/v1/storage/backup/backblaze";
const BACKBLAZE_CACHE_TTL: std::time::Duration = std::time::Duration::from_secs(10);

#[derive(Clone)]
struct BackblazeStatusRow { bucket: String, running: bool, last_run_at: Option<serde_json::Value>, last_run_ok: Option<bool>, connected: Option<bool> }
#[derive(Clone)]
struct BackblazeSnapshot { id: String, time: String, paths: Vec<String>, tags: Vec<String> }
static BACKBLAZE_STATUS_CACHE: OnceLock<Mutex<Option<(Instant, Vec<BackblazeStatusRow>)>>> = OnceLock::new();
static BACKBLAZE_SNAPSHOT_CACHE: OnceLock<Mutex<Option<(Instant, String, Vec<BackblazeSnapshot>)>>> = OnceLock::new();

fn backblaze_error(status: StatusCode, reason: &str) -> Response {
    (status, Json(serde_json::json!({"ok": false, "reason": reason}))).into_response()
}
fn backblaze_readback_response(readback: CaduceusHttpReadback) -> Response {
    let status = StatusCode::from_u16(readback.status).ok().filter(|s| readback.ok || s.is_client_error() || s.is_server_error())
        .unwrap_or_else(|| if readback.ok { StatusCode::OK } else { mutation_response_status(&readback) });
    let mut body = readback.body;
    if !readback.ok && body.get("reason").is_none() {
        if let Some(object) = body.as_object_mut() { object.insert("ok".into(), false.into()); object.insert("reason".into(), readback.first_missing_signal.into()); }
    }
    (status, Json(body)).into_response()
}
fn backblaze_redact_credentials(value: &mut serde_json::Value, key: &str, secret: &str) {
    match value {
        serde_json::Value::String(text) => {
            *text = text.replace(secret, "[redacted]").replace(key, "[redacted]");
        }
        serde_json::Value::Array(values) => for value in values { backblaze_redact_credentials(value, key, secret); },
        serde_json::Value::Object(values) => {
            let old = std::mem::take(values);
            for (field, mut value) in old {
                let name = field.replace(secret, "[redacted]").replace(key, "[redacted]");
                if matches!(field.as_str(), "keyId" | "key_id" | "applicationKey" | "application_key") { value = "[redacted]".into(); }
                else { backblaze_redact_credentials(&mut value, key, secret); }
                values.insert(name, value);
            }
        }
        _ => {}
    }
}
fn backblaze_same_origin(headers: &axum::http::HeaderMap) -> bool {
    let origin = headers.get(axum::http::header::ORIGIN).and_then(|v| v.to_str().ok()).and_then(|v| url::Url::parse(v).ok());
    let host = headers.get(axum::http::header::HOST).and_then(|v| v.to_str().ok());
    match (origin, host) {
        (Some(origin), Some(host)) if matches!(origin.scheme(), "http" | "https") && origin.path() == "/" && origin.query().is_none() && origin.fragment().is_none() && origin.username().is_empty() && origin.password().is_none() => {
            let expected = url::Url::parse(&format!("{}://{host}/", origin.scheme())).ok();
            expected.is_some_and(|expected| expected.host_str() == origin.host_str() && expected.port_or_known_default() == origin.port_or_known_default())
        }
        _ => false,
    }
}
fn backblaze_invalidate() {
    *BACKBLAZE_STATUS_CACHE.get_or_init(|| Mutex::new(None)).lock().unwrap_or_else(|e| e.into_inner()) = None;
    *BACKBLAZE_SNAPSHOT_CACHE.get_or_init(|| Mutex::new(None)).lock().unwrap_or_else(|e| e.into_inner()) = None;
}
fn backblaze_act(headers: &axum::http::HeaderMap, path: &str, body: serde_json::Value) -> CaduceusHttpReadback {
    if !backblaze_same_origin(headers) {
        return CaduceusHttpReadback { ok: false, status: 403, path: path.to_owned(), body: serde_json::json!({"ok":false,"reason":"same-origin-required"}), first_missing_signal: "same-origin-required".into() };
    }
    let result = caduceus_actuate_json(&mutation_authority(), headers, path, body);
    if result.ok { backblaze_invalidate(); }
    result
}
fn backblaze_bucket_identifier(row: &serde_json::Value) -> Result<&str, String> {
    let name = row.get("bucket").and_then(serde_json::Value::as_str).ok_or("bucket row must include a string identifier")?;
    if name.trim() != name || name.is_empty() || name.contains('/') { return Err("bucket identifier is invalid".into()); }
    Ok(name)
}
fn backblaze_config_value() -> Result<serde_json::Value, String> {
    let (_, root) = load_homeserver_json_sync()?;
    let root = root.as_object().ok_or("backblaze config is unavailable")?;
    let Some(tabs) = root.get("tabs") else { return Ok(serde_json::json!({"buckets":[]})); };
    let tabs = tabs.as_object().ok_or("backblaze config is unavailable")?;
    let Some(backblaze) = tabs.get("backblaze") else { return Ok(serde_json::json!({"buckets":[]})); };
    let backblaze = backblaze.as_object().ok_or("backblaze config is unavailable")?;
    let Some(config) = backblaze.get("config") else { return Ok(serde_json::json!({"buckets":[]})); };
    let mut config = config.as_object().ok_or("backblaze config must be an object")?.clone();
    if !config.contains_key("buckets") {
        if let Some(bucket) = config.get("bucket") {
            let mut row = serde_json::Map::new(); row.insert("bucket".into(), bucket.clone());
            for field in ["prefix", "items", "managers"] { if let Some(value) = config.get(field) { row.insert(field.into(), value.clone()); } }
            if !row.contains_key("items") { if let Some(paths) = config.get("paths").and_then(serde_json::Value::as_array) { row.insert("items".into(), paths.iter().map(|p| serde_json::json!({"path":p})).collect()); } }
            config.insert("buckets".into(), serde_json::json!([row]));
        } else { config.insert("buckets".into(), serde_json::json!([])); }
    }
    let rows = config.get_mut("buckets").and_then(serde_json::Value::as_array_mut).ok_or("buckets must be an array")?;
    for row in rows {
        backblaze_bucket_identifier(row)?;
        if let Some(object) = row.as_object_mut() { object.remove("encrypted"); }
        if let Some(forgejo) = row.get_mut("managers").and_then(|v| v.get_mut("forgejo")).and_then(serde_json::Value::as_object_mut) {
            if !forgejo.contains_key("skip") { forgejo.insert("skip".into(), serde_json::json!([])); }
        }
    }
    config.remove("encrypted");
    Ok(serde_json::Value::Object(config))
}
fn backblaze_bucket_rows(config: &serde_json::Value) -> Result<&[serde_json::Value], String> {
    config.get("buckets").and_then(serde_json::Value::as_array).map(Vec::as_slice).ok_or_else(|| "buckets must be an array".into())
}
fn backblaze_bucket(config: &serde_json::Value, name: &str) -> Option<serde_json::Value> {
    backblaze_bucket_rows(config).ok()?.iter().find(|v| v.get("bucket").and_then(serde_json::Value::as_str) == Some(name)).cloned()
}
fn forgejo_skip(value: Option<&serde_json::Value>) -> Result<Vec<String>, String> {
    let Some(value) = value else { return Ok(Vec::new()); };
    let values = value.as_array().ok_or("forgejo skip must be an array")?;
    let mut result = Vec::new();
    for value in values {
        let text = value.as_str().ok_or_else(|| format!("invalid forgejo skip entry: {value}"))?.trim();
        let parts: Vec<&str> = text.split('/').collect();
        if parts.len() > 2 || parts.iter().any(|p| p.is_empty() || !p.bytes().all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.'))) { return Err(format!("invalid forgejo skip entry: {text}")); }
        if result.iter().any(|v| v == text) { return Err(format!("duplicate forgejo skip entry: {text}")); }
        result.push(text.to_owned());
    }
    Ok(result)
}
fn backblaze_save_body(row: &serde_json::Value) -> Result<serde_json::Value, String> {
    let bucket = backblaze_bucket_identifier(row)?;
    let items = row.get("items").and_then(serde_json::Value::as_array).ok_or("bucket item list is unknown")?;
    let mut normalized_items = Vec::new();
    for item in items {
        let path = item.get("path").and_then(serde_json::Value::as_str).ok_or("bucket item path is unknown")?;
        if path.trim() != path || !path.starts_with('/') { return Err("bucket item path is incomplete or unsafe".into()); }
        normalized_items.push(item.clone());
    }
    let forgejo = row.get("managers").and_then(|v| v.get("forgejo")).and_then(serde_json::Value::as_object).ok_or("bucket manager configuration is unknown")?;
    forgejo.get("localNas").and_then(serde_json::Value::as_bool).ok_or("bucket manager flags are unknown")?;
    forgejo.get("backblazeBackup").and_then(serde_json::Value::as_bool).ok_or("bucket manager flags are unknown")?;
    let skip = forgejo_skip(forgejo.get("skip"))?;
    let mut managers = row.get("managers").cloned().ok_or("bucket manager configuration is unknown")?;
    managers["forgejo"]["skip"] = serde_json::json!(skip);
    Ok(serde_json::json!({"bucket":bucket,"items":normalized_items,"managers":managers}))
}
fn backblaze_config_response() -> Response {
    match backblaze_config_value() {
        Ok(config) => (StatusCode::OK, Json(serde_json::json!({"buckets":config["buckets"]}))).into_response(),
        Err(reason) => backblaze_error(StatusCode::SERVICE_UNAVAILABLE, &reason),
    }
}
async fn backblaze_buckets_get_route() -> Response { backblaze_config_response() }
async fn backblaze_config_get_route() -> Response { backblaze_config_response() }
async fn backblaze_items_get_route(Path(bucket): Path<String>) -> Response {
    match backblaze_config_value() {
        Ok(config) => match backblaze_bucket(&config, &bucket) { Some(row) => (StatusCode::OK, Json(row)).into_response(), None => backblaze_error(StatusCode::NOT_FOUND, "bucket not configured") },
        Err(reason) => backblaze_error(StatusCode::SERVICE_UNAVAILABLE, &reason),
    }
}
async fn backblaze_bucket_post_route(headers: axum::http::HeaderMap, Json(body): Json<serde_json::Value>) -> Response {
    let bucket = match backblaze_bucket_identifier(&body) { Ok(v) => v.to_owned(), Err(e) => return backblaze_error(StatusCode::BAD_REQUEST, &e) };
    let config = match backblaze_config_value() { Ok(v) => v, Err(e) => return backblaze_error(StatusCode::SERVICE_UNAVAILABLE, &e) };
    let path = if backblaze_bucket(&config, &bucket).is_some() { format!("{BACKBLAZE_DOOR}/bucket/credentials") } else { format!("{BACKBLAZE_DOOR}/bucket/add") };
    let key = body.get("keyId").or_else(|| body.get("key_id")).and_then(serde_json::Value::as_str);
    let secret = body.get("applicationKey").or_else(|| body.get("application_key")).and_then(serde_json::Value::as_str);
    if key.is_none_or(str::is_empty) || secret.is_none_or(str::is_empty) { return backblaze_error(StatusCode::BAD_REQUEST, "credentials are required"); }
    let mut receipt = backblaze_act(&headers, &path, serde_json::json!({"bucket":bucket,"keyId":key,"applicationKey":secret}));
    backblaze_redact_credentials(&mut receipt.body, key.unwrap_or_default(), secret.unwrap_or_default());
    receipt.first_missing_signal = receipt.first_missing_signal.replace(secret.unwrap_or_default(), "[redacted]").replace(key.unwrap_or_default(), "[redacted]");
    backblaze_readback_response(receipt)
}
async fn backblaze_bucket_verify_route(headers: axum::http::HeaderMap, Path(bucket): Path<String>) -> Response {
    backblaze_readback_response(backblaze_act(&headers, &format!("{BACKBLAZE_DOOR}/bucket/verify"), serde_json::json!({"bucket":bucket})))
}
async fn backblaze_bucket_delete_route(headers: axum::http::HeaderMap, Path(bucket): Path<String>) -> Response {
    backblaze_readback_response(backblaze_act(&headers, &format!("{BACKBLAZE_DOOR}/bucket/remove"), serde_json::json!({"bucket":bucket})))
}
async fn backblaze_item_post_route(headers: axum::http::HeaderMap, Path(bucket): Path<String>, Json(item): Json<serde_json::Value>) -> Response { backblaze_item_mutate(headers, bucket, item, "add") }
async fn backblaze_item_delete_route(headers: axum::http::HeaderMap, Path(bucket): Path<String>, Json(item): Json<serde_json::Value>) -> Response { backblaze_item_mutate(headers, bucket, item, "remove") }
fn backblaze_item_mutate(headers: axum::http::HeaderMap, bucket: String, item: serde_json::Value, op: &str) -> Response {
    let Some(path) = item.get("path").and_then(serde_json::Value::as_str).filter(|p| p.trim() == *p && p.starts_with('/')) else { return backblaze_error(StatusCode::BAD_REQUEST, "path must be absolute"); };
    backblaze_readback_response(backblaze_act(&headers, &format!("{BACKBLAZE_DOOR}/bucket/items"), serde_json::json!({"bucket":bucket,"path":path,"op":op})))
}
async fn backblaze_config_post_route(headers: axum::http::HeaderMap, Json(body): Json<serde_json::Value>) -> Response {
    let current = match backblaze_config_value() { Ok(v) => v, Err(e) => return backblaze_error(StatusCode::SERVICE_UNAVAILABLE, &e) };
    let old_rows = match backblaze_bucket_rows(&current) { Ok(v) => v, Err(e) => return backblaze_error(StatusCode::BAD_REQUEST, &e) };
    let Some(rows) = body.get("buckets").and_then(serde_json::Value::as_array) else { return backblaze_error(StatusCode::BAD_REQUEST, "buckets must be an array"); };
    let expected_rows = match body.get("expected") { None | Some(serde_json::Value::Null) => None, Some(value) => match value.as_array() { Some(values) if values.len() == rows.len() => Some(values), _ => return backblaze_error(StatusCode::BAD_REQUEST, "expected bucket state is incomplete") } };
    let mut changes = Vec::new(); let mut names = std::collections::HashSet::new();
    for (index, row) in rows.iter().enumerate() {
        let save = match backblaze_save_body(row) { Ok(v) => v, Err(e) => return backblaze_error(StatusCode::BAD_REQUEST, &e) };
        let name = save["bucket"].as_str().unwrap_or_default();
        if !names.insert(name.to_owned()) { return backblaze_error(StatusCode::BAD_REQUEST, "duplicate bucket"); }
        let Some(old) = old_rows.iter().find(|v| v.get("bucket").and_then(serde_json::Value::as_str) == Some(name)) else { return backblaze_error(StatusCode::BAD_REQUEST, "bucket is not configured"); };
        let old_save = match backblaze_save_body(old) { Ok(v) => v, Err(e) => return backblaze_error(StatusCode::BAD_REQUEST, &e) };
        if let Some(expected_rows) = expected_rows {
            let expected = match backblaze_save_body(&expected_rows[index]) { Ok(v) => v, Err(e) => return backblaze_error(StatusCode::BAD_REQUEST, &e) };
            if expected != old_save { return backblaze_error(StatusCode::CONFLICT, "bucket changed; refresh before saving"); }
        }
        if save != old_save { changes.push(save); }
    }
    let mut last = None; let mut applied: Vec<String> = Vec::new();
    for body in changes {
        let name = body.get("bucket").and_then(serde_json::Value::as_str).unwrap_or_default().to_owned();
        let receipt = backblaze_act(&headers, &format!("{BACKBLAZE_DOOR}/bucket/save"), body);
        if !receipt.ok {
            let mut receipt = receipt;
            if let Some(object) = receipt.body.as_object_mut() { object.insert("appliedBuckets".into(), serde_json::json!(applied)); }
            return backblaze_readback_response(receipt);
        }
        applied.push(name);
        last = Some(receipt);
    }
    match last { Some(receipt) => backblaze_readback_response(receipt), None => (StatusCode::OK, Json(serde_json::json!({"ok":true}))).into_response() }
}
async fn backblaze_run_bucket_route(headers: axum::http::HeaderMap, Path(bucket): Path<String>) -> Response {
    backblaze_readback_response(backblaze_act(&headers, &format!("{BACKBLAZE_DOOR}/bucket/run"), serde_json::json!({"bucket":bucket})))
}
async fn backblaze_run_route(headers: axum::http::HeaderMap, Json(body): Json<serde_json::Value>) -> Response {
    let bucket = match body.get("bucket") {
        Some(value) => match value.as_str() { Some(v) => v.to_owned(), None => return backblaze_error(StatusCode::BAD_REQUEST, "bucket must be a string") },
        None => match backblaze_config_value().and_then(|v| backblaze_bucket_rows(&v).map(|rows| rows.first().and_then(|r| r.get("bucket")).and_then(serde_json::Value::as_str).map(str::to_owned))) {
            Ok(Some(v)) => v, Ok(None) => return backblaze_error(StatusCode::BAD_REQUEST, "no bucket configured"), Err(e) => return backblaze_error(StatusCode::SERVICE_UNAVAILABLE, &e),
        },
    };
    let (path, payload) = if body.get("plan").and_then(serde_json::Value::as_bool) == Some(true) { ("/api/v1/storage/backup/forgejo/backup-b2".to_owned(), serde_json::json!({"bucket":bucket,"plan":true})) } else { (format!("{BACKBLAZE_DOOR}/bucket/run"), serde_json::json!({"bucket":bucket})) };
    backblaze_readback_response(backblaze_act(&headers, &path, payload))
}
fn backblaze_status_rows() -> Result<Vec<BackblazeStatusRow>, CaduceusHttpReadback> {
    let mut cache = BACKBLAZE_STATUS_CACHE.get_or_init(|| Mutex::new(None)).lock().unwrap_or_else(|e| e.into_inner());
    if let Some((time, rows)) = &*cache { if time.elapsed() < BACKBLAZE_CACHE_TTL { return Ok(rows.clone()); } }
    let receipt = caduceus_http("GET", &format!("{BACKBLAZE_DOOR}/status"));
    if !receipt.ok { return Err(receipt); }
    let Some(values) = receipt.body.get("buckets").and_then(serde_json::Value::as_array) else { return Err(CaduceusHttpReadback { ok:false, status:503, path:receipt.path, body:serde_json::json!({"ok":false,"reason":"backup status unavailable"}), first_missing_signal:"backup status unavailable".into() }); };
    if values.len() > 256 { return Err(CaduceusHttpReadback { ok:false, status:503, path:receipt.path, body:serde_json::json!({"ok":false,"reason":"too many backup status rows"}), first_missing_signal:"too many backup status rows".into() }); }
    let mut rows = Vec::new();
    for value in values {
        let Some(bucket) = value.get("bucket").and_then(serde_json::Value::as_str) else { return Err(CaduceusHttpReadback {ok:false,status:503,path:receipt.path,body:serde_json::json!({"ok":false,"reason":"invalid backup status row"}),first_missing_signal:"invalid backup status row".into()}); };
        let Some(running) = value.get("running").and_then(serde_json::Value::as_bool) else { return Err(CaduceusHttpReadback {ok:false,status:503,path:receipt.path,body:serde_json::json!({"ok":false,"reason":"invalid backup status row"}),first_missing_signal:"invalid backup status row".into()}); };
        rows.push(BackblazeStatusRow {bucket:bucket.to_owned(), running, last_run_at:value.get("last_run_at").cloned().filter(|v| v.is_number() || v.is_string()), last_run_ok:value.get("last_run_ok").and_then(serde_json::Value::as_bool), connected:value.get("connected").and_then(serde_json::Value::as_bool)});
    }
    *cache = Some((Instant::now(),rows.clone())); Ok(rows)
}
async fn backblaze_status_route() -> Response {
    let config = match backblaze_config_value() { Ok(v) => v, Err(e) => return backblaze_error(StatusCode::SERVICE_UNAVAILABLE, &e) };
    let statuses = match backblaze_status_rows() { Ok(v) => v, Err(e) => return backblaze_readback_response(e) };
    let mut buckets = match backblaze_bucket_rows(&config) { Ok(v) => v.to_vec(), Err(e) => return backblaze_error(StatusCode::SERVICE_UNAVAILABLE, &e) };
    for row in &mut buckets {
        let name = match backblaze_bucket_identifier(row) { Ok(v) => v.to_owned(), Err(e) => return backblaze_error(StatusCode::SERVICE_UNAVAILABLE, &e) };
        let status = statuses.iter().find(|s| s.bucket == name);
        let Some(status) = status else { return backblaze_error(StatusCode::SERVICE_UNAVAILABLE, "bucket status unavailable"); };
        row["status"] = serde_json::json!({"running":status.running,"last_run_at":status.last_run_at,"last_run_ok":status.last_run_ok});
        row["connected"] = status.connected.into();
    }
    let running = statuses.iter().any(|s| s.running);
    let last = statuses.iter().filter(|s| s.last_run_at.is_some()).max_by_key(|s| s.last_run_at.as_ref().and_then(serde_json::Value::as_u64).unwrap_or(0));
    (StatusCode::OK, Json(serde_json::json!({"ok":true,"buckets":buckets,"running":running,"last_run_at":last.and_then(|s|s.last_run_at.clone()),"last_run_ok":last.and_then(|s|s.last_run_ok)}))).into_response()
}
#[derive(Debug, Deserialize)]
struct BackblazeSnapshotsQuery { bucket: Option<String> }
async fn backblaze_snapshots_route(Query(query): Query<BackblazeSnapshotsQuery>) -> Response {
    let Some(bucket) = query.bucket.filter(|b| !b.is_empty()) else { return backblaze_error(StatusCode::BAD_REQUEST, "bucket is required"); };
    let mut cache = BACKBLAZE_SNAPSHOT_CACHE.get_or_init(|| Mutex::new(None)).lock().unwrap_or_else(|e| e.into_inner());
    let cached = cache.as_ref().filter(|(time,name,_)| *name == bucket && time.elapsed() < BACKBLAZE_CACHE_TTL).map(|(_,_,rows)| rows.clone());
    let rows = if let Some(rows) = cached { rows } else {
        let encoded: String = url::form_urlencoded::byte_serialize(bucket.as_bytes()).collect();
        let receipt = caduceus_http("GET", &format!("{BACKBLAZE_DOOR}/snapshots?bucket={encoded}"));
        if !receipt.ok { return backblaze_readback_response(receipt); }
        let Some(values) = receipt.body.get("snapshots").and_then(serde_json::Value::as_array) else { return backblaze_error(StatusCode::SERVICE_UNAVAILABLE, "snapshots unavailable"); };
        if values.len() > 512 { return backblaze_error(StatusCode::SERVICE_UNAVAILABLE, "too many snapshots"); }
        let mut rows = Vec::new();
        for value in values {
            let (Some(id), Some(time), Some(paths), Some(tags)) = (value.get("id").and_then(serde_json::Value::as_str), value.get("time").and_then(serde_json::Value::as_str), value.get("paths").and_then(serde_json::Value::as_array), value.get("tags").and_then(serde_json::Value::as_array)) else { return backblaze_error(StatusCode::SERVICE_UNAVAILABLE, "invalid snapshot row"); };
            let Some(paths) = paths.iter().map(|v|v.as_str().map(str::to_owned)).collect::<Option<Vec<_>>>() else { return backblaze_error(StatusCode::SERVICE_UNAVAILABLE, "invalid snapshot paths"); };
            let Some(tags) = tags.iter().map(|v|v.as_str().map(str::to_owned)).collect::<Option<Vec<_>>>() else { return backblaze_error(StatusCode::SERVICE_UNAVAILABLE, "invalid snapshot tags"); };
            rows.push(BackblazeSnapshot{id:id.to_owned(),time:time.to_owned(),paths,tags});
        }
        *cache = Some((Instant::now(), bucket.clone(), rows.clone())); rows
    };
    let snapshots: Vec<_> = rows.into_iter().map(|row|serde_json::json!({"id":row.id,"time":row.time,"paths":row.paths,"tags":row.tags})).collect();
    (StatusCode::OK, Json(serde_json::json!({"ok":true,"snapshots":snapshots}))).into_response()
}
