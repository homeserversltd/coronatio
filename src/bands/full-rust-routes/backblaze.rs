// Backblaze is a face for the Caduceus doors; the config seat is read-only here.
const BACKBLAZE_DOOR: &str = "/api/v1/storage/backup/backblaze";
const BACKBLAZE_CACHE_TTL: std::time::Duration = std::time::Duration::from_secs(10);

#[derive(Clone)]
struct BackblazeStatusRow { bucket: String, running: bool, last_run_at: Option<serde_json::Value>, last_run_ok: Option<bool>, connected: Option<bool> }
#[derive(Clone)]
struct BackblazeSnapshot { id: String, time: String, paths: Vec<String>, tags: Vec<String>, bucket: Option<String> }
#[derive(Clone)]
struct BackblazeSnapshotError { bucket: String, step: String, reason: String }
static BACKBLAZE_STATUS_CACHE: OnceLock<Mutex<Option<(Instant, Vec<BackblazeStatusRow>)>>> = OnceLock::new();
static BACKBLAZE_SNAPSHOT_CACHE: OnceLock<Mutex<Option<(Instant, String, Vec<BackblazeSnapshot>, Vec<BackblazeSnapshotError>)>>> = OnceLock::new();
static BACKUP_SCHEDULE_CACHE: OnceLock<Mutex<Vec<(Instant, String, Option<serde_json::Value>, Option<serde_json::Value>)>>> = OnceLock::new();
const BACKUP_SCHEDULE_DOOR: &str = "/api/v1/pid2/schedule";

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

fn backblaze_invalidate() {
    *BACKBLAZE_STATUS_CACHE.get_or_init(|| Mutex::new(None)).lock().unwrap_or_else(|e| e.into_inner()) = None;
    *BACKBLAZE_SNAPSHOT_CACHE.get_or_init(|| Mutex::new(None)).lock().unwrap_or_else(|e| e.into_inner()) = None;
    BACKUP_SCHEDULE_CACHE.get_or_init(|| Mutex::new(Vec::new())).lock().unwrap_or_else(|e| e.into_inner()).clear();
}
fn backup_schedule(id: &str) -> Result<(Option<serde_json::Value>, Option<serde_json::Value>), CaduceusHttpReadback> {
    let mut cache = BACKUP_SCHEDULE_CACHE.get_or_init(|| Mutex::new(Vec::new())).lock().unwrap_or_else(|e| e.into_inner());
    cache.retain(|(time,_,_,_)| time.elapsed() < BACKBLAZE_CACHE_TTL);
    if let Some((_,_,row,kernel)) = cache.iter().find(|(_,name,_,_)|name == id) { return Ok((row.clone(),kernel.clone())); }
    let encoded: String = url::form_urlencoded::byte_serialize(id.as_bytes()).collect();
    let receipt = caduceus_http("GET", &format!("{BACKUP_SCHEDULE_DOOR}/status/{encoded}"));
    if !receipt.ok { return Err(receipt); }
    let payload = receipt.body.get("receiptPayload").unwrap_or(&receipt.body);
    let row = if payload.get("schedule") == Some(&serde_json::Value::Null) { None } else if let Some(schedule) = payload.get("schedule").and_then(serde_json::Value::as_object) { let field = |name: &str| schedule.get(name).cloned().unwrap_or(serde_json::Value::Null); let fire = payload.get("last_fire").filter(|v|v.is_object()).map(|v|serde_json::json!({"state":v.get("state"),"trigger":v.get("trigger"),"started_at":v.get("started_at"),"finished_at":v.get("finished_at"),"duration_ms":v.get("duration_ms"),"first_missing_signal":v.get("first_missing_signal"),"reason":v.get("reason")})); Some(serde_json::json!({"schedule":{"enabled":field("enabled"),"frequency":field("frequency"),"time":field("time"),"weekday":field("weekday"),"calendar":field("calendar")},"next_fire":payload.get("next_fire"),"last_fire":fire})) } else { return Err(CaduceusHttpReadback {ok:false,status:503,path:receipt.path,body:serde_json::json!({"ok":false,"reason":"schedule status unavailable"}),first_missing_signal:"schedule status unavailable".into()}); };
    let kernel = if receipt.body.get("receiptPayload").is_some() { Some(serde_json::json!({"schema":receipt.body.get("schema"),"version":receipt.body.get("version"),"timestamp":receipt.body.get("timestamp")})) } else { None };
    if cache.len() >= 256 { cache.remove(0); }
    cache.push((Instant::now(), id.to_owned(), row.clone(), kernel.clone())); Ok((row,kernel))
}
fn backup_id(bucket: &str) -> String { format!("backblaze-{bucket}") }
fn backup_schedule_fresh(id: &str) -> Result<(Option<serde_json::Value>, Option<serde_json::Value>), CaduceusHttpReadback> {
    BACKUP_SCHEDULE_CACHE.get_or_init(|| Mutex::new(Vec::new())).lock().unwrap_or_else(|e| e.into_inner()).retain(|(_,name,_,_)| name != id);
    backup_schedule(id)
}
fn backup_command(mut body: serde_json::Value, kernel: Option<&serde_json::Value>, transition: &str) -> serde_json::Value {
    if let (Some(object), Some(kernel)) = (body.as_object_mut(), kernel) {
        for field in ["schema", "version", "timestamp"] { if let Some(value) = kernel.get(field) { object.insert(field.into(), value.clone()); } }
        object.insert("intent_id".into(), "coronatio-backups".into());
        object.insert("transition".into(), transition.into());
    }
    body
}
fn backup_schedule_response(mut receipt: CaduceusHttpReadback) -> Response {
    if receipt.ok && receipt.body.get("ok").is_none() {
        let payload_ok = receipt.body.get("receiptPayload").and_then(|v|v.get("ok")).and_then(serde_json::Value::as_bool).unwrap_or(true);
        receipt.ok = payload_ok;
        if let Some(object) = receipt.body.as_object_mut() { object.insert("ok".into(), payload_ok.into()); }
    }
    backblaze_readback_response(receipt)
}
fn backup_schedule_save(headers: &axum::http::HeaderMap, body: &serde_json::Value) -> Response {
    let Some(id) = body.get("schedule_id").and_then(serde_json::Value::as_str) else { return backblaze_error(StatusCode::BAD_REQUEST, "schedule id is required"); };
    let is_nas = id == "nas-sync";
    let bucket = if is_nas { None } else { id.strip_prefix("backblaze-") };
    if !is_nas {
        let Some(name) = bucket else { return backblaze_error(StatusCode::BAD_REQUEST, "unknown backup job"); };
        let configured = backblaze_config_value().ok().and_then(|config| backblaze_bucket(&config, name)).is_some();
        if !configured { return backblaze_error(StatusCode::BAD_REQUEST, "bucket not configured"); }
    }
    let frequency = body.get("frequency").and_then(serde_json::Value::as_str).unwrap_or("");
    if !matches!(frequency, "manual" | "daily" | "weekly") { return backblaze_error(StatusCode::BAD_REQUEST, "invalid frequency"); }
    let time = body.get("time").and_then(serde_json::Value::as_str).unwrap_or("");
    let bytes = time.as_bytes();
    if bytes.len()!=5 || bytes[2]!=b':' || !bytes.iter().enumerate().all(|(i,b)| i==2 || b.is_ascii_digit()) || &time[..2]>"23" || &time[3..]>"59" { return backblaze_error(StatusCode::BAD_REQUEST, "invalid time"); }
    let weekday = if frequency == "weekly" { match body.get("weekday").and_then(serde_json::Value::as_str) { Some(day) if matches!(day, "Mon"|"Tue"|"Wed"|"Thu"|"Fri"|"Sat"|"Sun") => serde_json::json!(day), _ => return backblaze_error(StatusCode::BAD_REQUEST, "invalid weekday") } } else { serde_json::Value::Null };
    let calendar = serde_json::json!({"frequency":if frequency=="manual" {"daily"} else {frequency}, "time":time,"weekday":weekday});
    let (current,kernel) = match backup_schedule_fresh(id) { Ok(v) => v, Err(e) => return backblaze_readback_response(e) };
    if is_nas && current.is_none() { return backblaze_error(StatusCode::NOT_FOUND, "NAS schedule is not registered"); }
    if is_nas && frequency != "manual" && current.as_ref().and_then(|v|v.get("schedule")).and_then(|v|v.get("enabled")).and_then(serde_json::Value::as_bool) != Some(true) { return backblaze_error(StatusCode::FORBIDDEN, "NAS schedule cannot be re-armed here"); }
    if current.is_none() {
        let Some(timestamp) = body.get("timestamp").and_then(serde_json::Value::as_str) else { return backblaze_error(StatusCode::BAD_REQUEST, "timestamp is required"); };
        let envelope = serde_json::json!({"schema":"caduceus.pid2.schedule.v1","intent_id":"coronatio-backups","transition":"register","version":"1.0.0","timestamp":timestamp,"schedule":{"schedule_id":id,"calendar":calendar,"job":{"band":"storage/backup/backblaze/bucket-run","action":"run","payload":{"bucket":bucket}}}});
        let register = backblaze_act(headers, &format!("{BACKUP_SCHEDULE_DOOR}/register"), envelope);
        if !register.ok { return backup_schedule_response(register); }
        if frequency == "manual" {
            let mut disabled = backblaze_act(headers, &format!("{BACKUP_SCHEDULE_DOOR}/disable"), backup_command(serde_json::json!({"schedule_id":id}), kernel.as_ref(), "schedule-disable"));
            if !disabled.ok { if let Some(object) = disabled.body.as_object_mut() { object.insert("reason".into(), "Schedule was registered but could not be disabled; it may run. Retry Manual or disable it through the owner controls.".into()); } }
            return backup_schedule_response(disabled);
        }
        return backup_schedule_response(register);
    }
    backup_schedule_response(backblaze_act(headers, &format!("{BACKUP_SCHEDULE_DOOR}/update"), backup_command(serde_json::json!({"schedule_id":id,"calendar":calendar,"enabled":frequency!="manual"}), kernel.as_ref(), "schedule-update")))
}
fn backblaze_act(headers: &axum::http::HeaderMap, path: &str, body: serde_json::Value) -> CaduceusHttpReadback {
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
    let Some(rows) = body.get("buckets").and_then(serde_json::Value::as_array) else { return backblaze_error(StatusCode::BAD_REQUEST, "buckets must be an array"); };
    let expected_rows = match body.get("expected") { None | Some(serde_json::Value::Null) => None, Some(value) => match value.as_array() { Some(values) => Some(values), _ => return backblaze_error(StatusCode::BAD_REQUEST, "expected bucket state is incomplete") } };
    let mut changes = Vec::new(); let mut names = std::collections::HashSet::new();
    for row in rows {
        let mut save = match backblaze_save_body(row) { Ok(v) => v, Err(e) => return backblaze_error(StatusCode::BAD_REQUEST, &e) };
        let name = save["bucket"].as_str().unwrap_or_default();
        if !names.insert(name.to_owned()) { return backblaze_error(StatusCode::BAD_REQUEST, "duplicate bucket"); }
        if let Some(expected_rows) = expected_rows {
            let mut matching = expected_rows.iter().filter(|v| v.get("bucket").and_then(serde_json::Value::as_str) == Some(name));
            let Some(expected) = matching.next() else { return backblaze_error(StatusCode::BAD_REQUEST, "expected bucket state is incomplete"); };
            if matching.next().is_some() { return backblaze_error(StatusCode::BAD_REQUEST, "duplicate expected bucket"); }
            save["expected"] = expected.clone();
        }
        changes.push(save);
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
    backup_run(&headers, &backup_id(&bucket), Some(&bucket))
}
fn backup_run(headers: &axum::http::HeaderMap, id: &str, bucket: Option<&str>) -> Response {
    match backup_schedule_fresh(id) {
        Ok((Some(_),kernel)) => backup_schedule_response(backblaze_act(headers, &format!("{BACKUP_SCHEDULE_DOOR}/run"), backup_command(serde_json::json!({"schedule_id":id}), kernel.as_ref(), "schedule-run"))),
        Ok((None,_)) => match bucket { Some(bucket) => backblaze_readback_response(backblaze_act(headers, &format!("{BACKBLAZE_DOOR}/bucket/run"), serde_json::json!({"bucket":bucket}))), None => backblaze_error(StatusCode::NOT_FOUND, "NAS schedule is not registered") },
        Err(e) => backblaze_readback_response(e),
    }
}
async fn backblaze_run_route(headers: axum::http::HeaderMap, Json(body): Json<serde_json::Value>) -> Response {
    if body.get("action").and_then(serde_json::Value::as_str) == Some("schedule-save") { return backup_schedule_save(&headers, &body); }
    if body.get("schedule_id").and_then(serde_json::Value::as_str) == Some("nas-sync") { return backup_run(&headers, "nas-sync", None); }
    let bucket = match body.get("bucket") {
        Some(value) => match value.as_str() { Some(v) => v.to_owned(), None => return backblaze_error(StatusCode::BAD_REQUEST, "bucket must be a string") },
        None => match backblaze_config_value().and_then(|v| backblaze_bucket_rows(&v).map(|rows| rows.first().and_then(|r| r.get("bucket")).and_then(serde_json::Value::as_str).map(str::to_owned))) {
            Ok(Some(v)) => v, Ok(None) => return backblaze_error(StatusCode::BAD_REQUEST, "no bucket configured"), Err(e) => return backblaze_error(StatusCode::SERVICE_UNAVAILABLE, &e),
        },
    };
    if body.get("plan").and_then(serde_json::Value::as_bool) == Some(true) { return backblaze_readback_response(backblaze_act(&headers, "/api/v1/storage/backup/forgejo/backup-b2", serde_json::json!({"bucket":bucket,"plan":true}))); }
    backup_run(&headers, &backup_id(&bucket), Some(&bucket))
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
        row["status"] = status.map(|s| serde_json::json!({"running":s.running,"last_run_at":s.last_run_at,"last_run_ok":s.last_run_ok})).unwrap_or(serde_json::Value::Null);
        row["connected"] = status.and_then(|s| s.connected).into();
    }
    let running = statuses.iter().any(|s| s.running);
    let last = statuses.iter().filter(|s| s.last_run_at.as_ref().and_then(serde_json::Value::as_i64).is_some()).max_by_key(|s| s.last_run_at.as_ref().and_then(serde_json::Value::as_i64));
    let mut jobs = Vec::new();
    for bucket in &buckets {
        let name = bucket["bucket"].as_str().unwrap_or_default(); let id = backup_id(name);
        let row = match backup_schedule(&id) { Ok((row,_)) => row, Err(e) => return backblaze_readback_response(e) };
        jobs.push(serde_json::json!({"kind":"backblaze","bucket":name,"schedule_id":id,"pid2":row}));
    }
    match backup_schedule("nas-sync") {
        Ok((Some(row),_)) => jobs.push(serde_json::json!({"kind":"nas","bucket":null,"schedule_id":"nas-sync","pid2":row})),
        Ok((None,_)) => {}, Err(e) => return backblaze_readback_response(e),
    }
    (StatusCode::OK, Json(serde_json::json!({"ok":true,"buckets":buckets,"jobs":jobs,"running":running,"last_run_at":last.and_then(|s|s.last_run_at.clone()),"last_run_ok":last.and_then(|s|s.last_run_ok)}))).into_response()
}
#[derive(Debug, Deserialize)]
struct BackblazeSnapshotsQuery { bucket: Option<String> }
async fn backblaze_snapshots_route(Query(query): Query<BackblazeSnapshotsQuery>) -> Response {
    let Some(bucket) = query.bucket.filter(|b| !b.is_empty()) else { return backblaze_error(StatusCode::BAD_REQUEST, "bucket is required"); };
    let mut cache = BACKBLAZE_SNAPSHOT_CACHE.get_or_init(|| Mutex::new(None)).lock().unwrap_or_else(|e| e.into_inner());
    let cached = cache.as_ref().filter(|(time,name,_,_)| *name == bucket && time.elapsed() < BACKBLAZE_CACHE_TTL).map(|(_,_,rows,errors)| (rows.clone(),errors.clone()));
    let (rows, errors) = if let Some(rows) = cached { rows } else {
        let encoded: String = url::form_urlencoded::byte_serialize(bucket.as_bytes()).collect();
        let receipt = caduceus_http("GET", &format!("{BACKBLAZE_DOOR}/snapshots?bucket={encoded}"));
        if !receipt.ok { return backblaze_readback_response(receipt); }
        let Some(values) = receipt.body.get("snapshots").and_then(serde_json::Value::as_array) else { return backblaze_error(StatusCode::SERVICE_UNAVAILABLE, "snapshots unavailable"); };
        if values.len() > 512 { return backblaze_error(StatusCode::SERVICE_UNAVAILABLE, "too many snapshots"); }
        let mut rows = Vec::new();
        for value in values {
            let (Some(id), Some(time)) = (value.get("id").and_then(serde_json::Value::as_str), value.get("time").and_then(serde_json::Value::as_str)) else { return backblaze_error(StatusCode::SERVICE_UNAVAILABLE, "invalid snapshot row"); };
            let fields = |name: &str| -> Option<Vec<String>> { match value.get(name) { None | Some(serde_json::Value::Null) => Some(vec![]), Some(v) => v.as_array()?.iter().map(|v|v.as_str().map(str::to_owned)).collect() } };
            let (Some(paths), Some(tags)) = (fields("paths"),fields("tags")) else { return backblaze_error(StatusCode::SERVICE_UNAVAILABLE, "invalid snapshot paths or tags"); };
            rows.push(BackblazeSnapshot{id:id.to_owned(),time:time.to_owned(),paths,tags,bucket:value.get("bucket").and_then(serde_json::Value::as_str).map(str::to_owned)});
        }
        let mut errors = Vec::new();
        if let Some(values) = receipt.body.get("errors").filter(|v| !v.is_null()) {
            let Some(values) = values.as_array().filter(|v| v.len() <= 512) else { return backblaze_error(StatusCode::SERVICE_UNAVAILABLE, "invalid snapshot errors"); };
            for value in values {
                let (Some(bucket),Some(step),Some(reason)) = (value.get("bucket").and_then(serde_json::Value::as_str),value.get("step").and_then(serde_json::Value::as_str),value.get("reason").and_then(serde_json::Value::as_str)) else { return backblaze_error(StatusCode::SERVICE_UNAVAILABLE, "invalid snapshot error"); };
                errors.push(BackblazeSnapshotError { bucket:bucket.to_owned(),step:step.to_owned(),reason:reason.to_owned() });
            }
        }
        *cache = Some((Instant::now(), bucket.clone(), rows.clone(), errors.clone())); (rows, errors)
    };
    let snapshots: Vec<_> = rows.into_iter().map(|row|serde_json::json!({"id":row.id,"time":row.time,"paths":row.paths,"tags":row.tags,"bucket":row.bucket})).collect();
    let errors: Vec<_> = errors.into_iter().map(|row|serde_json::json!({"bucket":row.bucket,"step":row.step,"reason":row.reason})).collect();
    (StatusCode::OK, Json(serde_json::json!({"ok":true,"snapshots":snapshots,"errors":errors}))).into_response()
}
