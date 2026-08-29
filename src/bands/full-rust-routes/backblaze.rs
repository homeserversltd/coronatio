const BACKBLAZE_STATE_PATH: &str = "/var/lib/coronatio/backblaze_state.json";
const BACKBLAZE_KEYMAN_NEWKEY: &str = "/vault/keyman/newkey.sh";
const BACKBLAZE_KEYMAN_EXPORT: &str = "/vault/keyman/exportkey.sh";
const BACKBLAZE_KEY_EXCHANGE: &str = "/mnt/keyexchange";
const BACKBLAZE_RESTIC_SALT: &[u8] = b"coronatio_backblaze_restic_v1";
static BACKBLAZE_RUN_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct BackblazeState {
    #[serde(default)]
    buckets: BTreeMap<String, BackblazeBucketState>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct BackblazeBucketState {
    running: bool,
    last_run_at: Option<u64>,
    last_run_ok: Option<bool>,
    #[serde(default)]
    last_error: Option<String>,
}
#[derive(Debug, Clone)]
struct BackblazeCredentials {
    key_id: String,
    application_key: String,
}
#[derive(Debug, Clone)]
struct B2Authorization {
    api_url: String,
    token: String,
    bucket_id: String,
}

fn backblaze_state_path() -> PathBuf {
    env::var("CORONATIO_BACKBLAZE_STATE")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(BACKBLAZE_STATE_PATH))
}
fn backblaze_state() -> BackblazeState {
    std::fs::read_to_string(backblaze_state_path())
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}
fn write_backblaze_state(state: &BackblazeState) -> Result<(), String> {
    let path = backblaze_state_path();
    let parent = path.parent().ok_or("invalid state path")?;
    std::fs::create_dir_all(parent).map_err(|_| "state directory unavailable")?;
    let tmp = parent.join(format!(".backblaze.{}.tmp", std::process::id()));
    std::fs::write(
        &tmp,
        serde_json::to_vec(state).map_err(|_| "state encode failed")?,
    )
    .map_err(|_| "state write failed")?;
    std::fs::rename(tmp, path).map_err(|_| "state promote failed".to_string())
}
fn normalized_service(bucket: &str) -> String {
    bucket.replace('-', "_")
}
fn keyman_service(bucket: &str) -> String {
    normalized_service(bucket)
}

fn backblaze_config_value() -> Result<serde_json::Value, String> {
    let (_, root) = load_homeserver_json_sync()?;
    let config = root
        .get("tabs")
        .and_then(|v| v.get("backblaze"))
        .and_then(|v| v.get("config"))
        .cloned()
        .unwrap_or_else(|| serde_json::json!({}));
    if config.get("buckets").and_then(|v| v.as_array()).is_some() {
        return Ok(config);
    }
    let bucket = config
        .get("bucket")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim();
    if bucket.is_empty() {
        return Ok(serde_json::json!({"buckets": []}));
    }
    let prefix = config.get("prefix").and_then(|v| v.as_str()).unwrap_or("");
    let items = config
        .get("paths")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .filter_map(|v| v.as_str().map(|path| serde_json::json!({"path": path})))
        .collect::<Vec<_>>();
    Ok(
        serde_json::json!({"buckets": [{"bucket": bucket, "prefix": prefix, "encrypted": true, "items": items}]}),
    )
}
fn normalized_config(raw: serde_json::Value) -> Result<serde_json::Value, String> {
    let buckets = raw
        .get("buckets")
        .and_then(|v| v.as_array())
        .ok_or("buckets must be an array")?;
    let mut out = Vec::new();
    for b in buckets {
        let bucket = b
            .get("bucket")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .trim();
        if bucket.is_empty() || bucket.contains('/') {
            return Err("bucket is invalid".into());
        }
        let prefix = b
            .get("prefix")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .trim()
            .trim_matches('/');
        let encrypted = b.get("encrypted").and_then(|v| v.as_bool()).unwrap_or(true);
        let items = b
            .get("items")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .filter_map(|i| {
                let p = i.get("path").and_then(|v| v.as_str()).unwrap_or("").trim();
                (!p.is_empty() && p.starts_with('/')).then(|| serde_json::json!({"path": p}))
            })
            .collect::<Vec<_>>();
        let forgejo = b
            .get("managers")
            .and_then(|v| v.get("forgejo"))
            .cloned()
            .unwrap_or_else(|| serde_json::json!({}));
        let managers = serde_json::json!({"forgejo": {
            "localNas": forgejo.get("localNas").and_then(|v| v.as_bool()).unwrap_or(false),
            "backblazeBackup": forgejo.get("backblazeBackup").and_then(|v| v.as_bool()).unwrap_or(false),
        }});
        out.push(serde_json::json!({"bucket": bucket, "prefix": prefix, "encrypted": encrypted, "items": items, "managers": managers}));
    }
    Ok(serde_json::json!({"buckets": out}))
}
fn save_backblaze_config(config: serde_json::Value) -> Result<(), String> {
    let path = homeserver_json_path();
    let raw = std::fs::read_to_string(&path).map_err(|_| "homeserver.json unreadable")?;
    let mut root: serde_json::Value =
        serde_json::from_str(&raw).map_err(|_| "homeserver.json invalid")?;
    root["tabs"]["backblaze"]["config"] = config;
    std::fs::write(
        path,
        serde_json::to_vec_pretty(&root).map_err(|_| "config encode failed")?,
    )
    .map_err(|_| "config write failed".into())
}
fn backblaze_bucket(config: &serde_json::Value, name: &str) -> Option<serde_json::Value> {
    config
        .get("buckets")?
        .as_array()?
        .iter()
        .find(|b| b.get("bucket").and_then(|v| v.as_str()) == Some(name))
        .cloned()
}

fn backblaze_credential_candidates(service: &str) -> Vec<PathBuf> {
    let root = PathBuf::from(BACKBLAZE_KEY_EXCHANGE);
    let mut candidates = [
        service.to_string(),
        format!("{service}.env"),
        format!("{service}.json"),
        format!("{service}.key"),
        "backblaze".to_string(),
        "backblaze.env".to_string(),
        "backblaze.json".to_string(),
        "backblaze.key".to_string(),
    ]
    .into_iter()
    .map(|name| root.join(name))
    .filter(|path| path.is_file())
    .collect::<Vec<_>>();
    if let Ok(entries) = std::fs::read_dir(&root) {
        candidates.extend(entries.flatten().map(|entry| entry.path()).filter(|path| {
            path.is_file()
                && path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .is_some_and(|name| name.starts_with(service) || name.starts_with("backblaze"))
        }));
    }
    candidates.sort();
    candidates.dedup();
    candidates
}

fn credential_field(value: &serde_json::Value, names: &[&str]) -> Option<String> {
    names
        .iter()
        .find_map(|name| value.get(*name).and_then(serde_json::Value::as_str))
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
}

fn parse_keyman_credentials(raw: &str) -> Option<BackblazeCredentials> {
    if let Ok(value) = serde_json::from_str::<serde_json::Value>(raw) {
        let key_id = credential_field(
            &value,
            &["B2_ACCOUNT_ID", "key_id", "accountId", "key_id", "keyId"],
        );
        let application_key = credential_field(
            &value,
            &["B2_ACCOUNT_KEY", "application_key", "applicationKey", "key"],
        );
        if let (Some(key_id), Some(application_key)) = (key_id, application_key) {
            return Some(BackblazeCredentials {
                key_id,
                application_key,
            });
        }
    }
    let values = raw
        .lines()
        .filter_map(|line| line.split_once('='))
        .map(|(key, value)| {
            (
                key.trim(),
                value.trim().trim_matches('"').trim_matches('\''),
            )
        })
        .collect::<BTreeMap<_, _>>();
    let key_id = ["B2_ACCOUNT_ID", "B2_KEY_ID", "key_id", "key_id"]
        .into_iter()
        .find_map(|key| values.get(key).copied())
        .filter(|value| !value.is_empty())?;
    let application_key = ["B2_ACCOUNT_KEY", "B2_APPLICATION_KEY", "application_key"]
        .into_iter()
        .find_map(|key| values.get(key).copied())
        .filter(|value| !value.is_empty())?;
    Some(BackblazeCredentials {
        key_id: key_id.to_string(),
        application_key: application_key.to_string(),
    })
}

fn keyman_credentials(bucket: &str) -> Result<BackblazeCredentials, String> {
    let service = keyman_service(bucket);
    let exporter = PathBuf::from(BACKBLAZE_KEYMAN_EXPORT);
    if !exporter.is_file() {
        return Err("backblaze Keyman export is not configured".to_string());
    }
    let export = Command::new(exporter)
        .arg(&service)
        .output()
        .map_err(|_| "backblaze Keyman export could not start".to_string())?;
    if !export.status.success() {
        return Err("backblaze Keyman credentials are not available".to_string());
    }
    backblaze_credential_candidates(&service)
        .into_iter()
        .find_map(|path| {
            std::fs::read_to_string(path)
                .ok()
                .and_then(|raw| parse_keyman_credentials(&raw))
        })
        .ok_or_else(|| "backblaze Keyman credentials are not available".to_string())
}

fn b2_authorize(credentials: &BackblazeCredentials) -> Result<B2Authorization, String> {
    let basic = format!("{}:{}", credentials.key_id, credentials.application_key);
    let output = Command::new("curl")
        .args([
            "-fsS",
            "-u",
            &basic,
            "https://api.backblazeb2.com/b2api/v2/b2_authorize_account",
        ])
        .output()
        .map_err(|_| "B2 authorize failed")?;
    if !output.status.success() {
        return Err("B2 authorize failed".into());
    }
    let value: serde_json::Value = serde_json::from_slice(&output.stdout)
        .map_err(|_| "B2 authorize returned invalid response")?;
    Ok(B2Authorization {
        api_url: value["apiUrl"]
            .as_str()
            .ok_or("B2 authorize returned no API URL")?
            .to_owned(),
        token: value["authorizationToken"]
            .as_str()
            .ok_or("B2 authorize returned no token")?
            .to_owned(),
        bucket_id: String::new(),
    })
}
fn authorize_and_list(
    bucket: &str,
    credentials: &BackblazeCredentials,
) -> Result<B2Authorization, String> {
    let auth = b2_authorize(credentials)?;
    let url = format!("{}/b2api/v2/b2_list_buckets", auth.api_url);
    let output = Command::new("curl")
        .args([
            "-fsS",
            "-H",
            &format!("Authorization: {}", auth.token),
            "-H",
            "Content-Type: application/json",
            "-d",
            &serde_json::json!({"accountId": credentials.key_id, "bucketName": bucket}).to_string(),
            &url,
        ])
        .output()
        .map_err(|_| "B2 bucket listing failed")?;
    if !output.status.success() {
        return Err("B2 bucket listing failed".into());
    }
    let value: serde_json::Value = serde_json::from_slice(&output.stdout)
        .map_err(|_| "B2 bucket listing returned invalid response")?;
    if let Some(found) = value
        .get("buckets")
        .and_then(|v| v.as_array())
        .and_then(|bs| {
            bs.iter()
                .find(|b| b.get("bucketName").and_then(|v| v.as_str()) == Some(bucket))
        })
    {
        let mut auth = auth;
        auth.bucket_id = found
            .get("bucketId")
            .and_then(|v| v.as_str())
            .ok_or("B2 bucket listing returned no bucket ID")?
            .to_owned();
        Ok(auth)
    } else {
        Err("B2 bucket is not reachable".into())
    }
}
fn authorize_bucket(bucket: &str, submitted: &serde_json::Value) -> Result<(), String> {
    let key_id = submitted
        .get("keyId")
        .and_then(|v| v.as_str())
        .or_else(|| submitted.get("key_id").and_then(|v| v.as_str()))
        .ok_or("keyId is required")?;
    let application_key = submitted
        .get("applicationKey")
        .and_then(|v| v.as_str())
        .or_else(|| submitted.get("application_key").and_then(|v| v.as_str()))
        .ok_or("applicationKey is required")?;
    let credentials = BackblazeCredentials {
        key_id: key_id.to_owned(),
        application_key: application_key.to_owned(),
    };
    let _ = authorize_and_list(bucket, &credentials)?;
    let service = keyman_service(bucket);
    let seed = Command::new(BACKBLAZE_KEYMAN_NEWKEY)
        .args([&service, key_id, application_key])
        .output()
        .map_err(|_| "Keyman newkey service unavailable")?;
    if !seed.status.success() {
        return Err("Keyman newkey service unavailable".into());
    }
    Ok(())
}

fn restic_password(key: &str) -> String {
    let mut derived = [0u8; 32];
    pbkdf2::pbkdf2_hmac::<sha2::Sha256>(
        key.as_bytes(),
        BACKBLAZE_RESTIC_SALT,
        100_000,
        &mut derived,
    );
    base64::Engine::encode(&base64::engine::general_purpose::STANDARD, derived)
}
fn restic_init(repo: &str, credentials: &BackblazeCredentials) -> Result<(), String> {
    let out = Command::new("restic")
        .args(["--repo", repo, "init"])
        .env(
            "RESTIC_PASSWORD",
            restic_password(&credentials.application_key),
        )
        .env("B2_ACCOUNT_ID", &credentials.key_id)
        .env("B2_ACCOUNT_KEY", &credentials.application_key)
        .output()
        .map_err(|_| "restic init failed")?;
    out.status
        .success()
        .then_some(())
        .ok_or_else(|| "restic init failed".into())
}
fn restic_uninitialized(stderr: &[u8]) -> bool {
    let s = String::from_utf8_lossy(stderr).to_ascii_lowercase();
    s.contains("not initialized")
        || s.contains("repository does not exist")
        || s.contains("is there a repository")
}
fn restic_backup(
    repo: &str,
    paths: &[String],
    credentials: &BackblazeCredentials,
) -> Result<(), String> {
    let run = || {
        Command::new("restic")
            .args(["--repo", repo, "backup", "--json"])
            .args(paths)
            .env(
                "RESTIC_PASSWORD",
                restic_password(&credentials.application_key),
            )
            .env("B2_ACCOUNT_ID", &credentials.key_id)
            .env("B2_ACCOUNT_KEY", &credentials.application_key)
            .output()
    };
    let output = run().map_err(|_| "restic backup could not start")?;
    let output = if !output.status.success() && restic_uninitialized(&output.stderr) {
        restic_init(repo, credentials)?;
        run().map_err(|_| "restic backup could not start")?
    } else {
        output
    };
    output
        .status
        .success()
        .then_some(())
        .ok_or_else(|| "restic backup failed".into())
}
fn collect_files(path: &std::path::Path, out: &mut Vec<PathBuf>) -> Result<(), String> {
    let meta = std::fs::symlink_metadata(path)
        .map_err(|_| format!("cannot read backup path {}", path.display()))?;
    if meta.is_file() {
        out.push(path.to_owned());
        return Ok(());
    }
    if meta.is_dir() {
        for entry in std::fs::read_dir(path)
            .map_err(|_| format!("cannot read backup directory {}", path.display()))?
        {
            collect_files(
                &entry
                    .map_err(|_| "cannot enumerate backup directory")?
                    .path(),
                out,
            )?;
        }
    }
    Ok(())
}
fn b2_upload_files(auth: &B2Authorization, paths: &[String], prefix: &str) -> Result<u64, String> {
    let url = format!("{}/b2api/v2/b2_get_upload_url", auth.api_url);
    let info = Command::new("curl")
        .args([
            "-fsS",
            "-H",
            &format!("Authorization: {}", auth.token),
            "-H",
            "Content-Type: application/json",
            "-d",
            &serde_json::json!({"bucketId": auth.bucket_id}).to_string(),
            &url,
        ])
        .output()
        .map_err(|_| "B2 upload URL failed")?;
    if !info.status.success() {
        return Err("B2 upload URL failed".into());
    }
    let v: serde_json::Value =
        serde_json::from_slice(&info.stdout).map_err(|_| "B2 upload URL invalid")?;
    let upload_url = v["uploadUrl"].as_str().ok_or("B2 upload URL missing")?;
    let token = v["authorizationToken"]
        .as_str()
        .ok_or("B2 upload token missing")?;
    let mut files = Vec::new();
    for path in paths {
        collect_files(std::path::Path::new(path), &mut files)?;
    }
    let mut total = 0;
    for file in files {
        let bytes = std::fs::read(&file).map_err(|_| "cannot read backup file")?;
        let name = format!(
            "{}{}",
            prefix.trim_matches('/').to_owned() + if prefix.is_empty() { "" } else { "/" },
            file.to_string_lossy().trim_start_matches('/')
        );
        let size = bytes.len().to_string();
        let sha1_output = Command::new("sha1sum")
            .arg(&file)
            .output()
            .map_err(|_| "sha1sum unavailable")?;
        if !sha1_output.status.success() {
            return Err("cannot hash backup file".into());
        }
        let sha1 = String::from_utf8_lossy(&sha1_output.stdout)
            .split_whitespace()
            .next()
            .ok_or("invalid file hash")?
            .to_owned();
        let out = Command::new("curl")
            .args([
                "-fsS",
                "-X",
                "POST",
                upload_url,
                "-H",
                &format!("Authorization: {}", token),
                "-H",
                &format!("X-Bz-File-Name: {}", name),
                "-H",
                "Content-Type: application/octet-stream",
                "-H",
                &format!("Content-Length: {}", size),
                "-H",
                &format!("X-Bz-Content-Sha1: {}", sha1),
                "--data-binary",
                &format!("@{}", file.display()),
            ])
            .output()
            .map_err(|_| "B2 upload failed")?;
        if !out.status.success() {
            return Err("B2 upload failed".into());
        }
        total += bytes.len() as u64;
    }
    Ok(total)
}

fn backblaze_public(config: serde_json::Value) -> serde_json::Value {
    let state = backblaze_state();
    let buckets = config
        .get("buckets")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .map(|mut b| {
            let name = b["bucket"].as_str().unwrap_or("").to_owned();
            b["status"] = serde_json::json!(state.buckets.get(&name).cloned().unwrap_or_default());
            b["connected"] = serde_json::json!(keyman_credentials(&name)
                .and_then(|credentials| authorize_and_list(&name, &credentials))
                .is_ok());
            b
        })
        .collect::<Vec<_>>();
    serde_json::json!({"buckets": buckets})
}
async fn backblaze_buckets_get_route() -> Response {
    match backblaze_config_value() {
        Ok(c) => (StatusCode::OK, Json(backblaze_public(c))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"ok": false, "reason": e})),
        )
            .into_response(),
    }
}
async fn backblaze_bucket_post_route(
    headers: axum::http::HeaderMap,
    Json(body): Json<serde_json::Value>,
) -> Response {
    let mut intent_body = body.clone();
    if let Some(object) = intent_body.as_object_mut() {
        object.remove("keyId");
        object.remove("key_id");
        object.remove("applicationKey");
        object.remove("application_key");
    }
    let r = caduceus_staff_transition(
        &mutation_authority(),
        &headers,
        "POST",
        "/api/backblaze/buckets",
        "backblaze bucket",
        intent_body,
    );
    if !r.ok {
        return (
            mutation_response_status(&r),
            Json(serde_json::json!({"ok": false, "reason": r.first_missing_signal})),
        )
            .into_response();
    }
    let bucket = body["bucket"].as_str().unwrap_or("").trim().to_owned();
    if bucket.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"ok": false, "reason": "bucket is required"})),
        )
            .into_response();
    }
    if let Err(e) = authorize_bucket(&bucket, &body) {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"ok": false, "reason": e})),
        )
            .into_response();
    }
    let mut config = match backblaze_config_value() {
        Ok(v) => v,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({"ok": false, "reason": e})),
            )
                .into_response()
        }
    };
    let mut bucket_value = body;
    if let Some(object) = bucket_value.as_object_mut() {
        object.remove("keyId");
        object.remove("key_id");
        object.remove("applicationKey");
        object.remove("application_key");
    }
    bucket_value["bucket"] = serde_json::json!(bucket);
    if bucket_value.get("encrypted").is_none() {
        bucket_value["encrypted"] = serde_json::json!(true);
    }
    if bucket_value.get("items").is_none() {
        bucket_value["items"] = serde_json::json!([]);
    }
    let mut buckets = config["buckets"].as_array().cloned().unwrap_or_default();
    buckets.retain(|b| b["bucket"].as_str() != Some(&bucket));
    buckets.push(bucket_value);
    config = normalized_config(serde_json::json!({"buckets": buckets})).unwrap_or(config);
    match save_backblaze_config(config) {
        Ok(()) => (StatusCode::OK, Json(serde_json::json!({"ok": true}))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"ok": false, "reason": e})),
        )
            .into_response(),
    }
}
async fn backblaze_bucket_delete_route(
    headers: axum::http::HeaderMap,
    Path(bucket): Path<String>,
) -> Response {
    let r = caduceus_staff_transition(
        &mutation_authority(),
        &headers,
        "DELETE",
        "/api/backblaze/buckets/:bucket",
        "backblaze bucket",
        serde_json::json!({"bucket": bucket}),
    );
    if !r.ok {
        return (
            mutation_response_status(&r),
            Json(serde_json::json!({"ok": false, "reason": r.first_missing_signal})),
        )
            .into_response();
    }
    let config = match backblaze_config_value() {
        Ok(v) => v,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({"ok": false, "reason": e})),
            )
                .into_response()
        }
    };
    let buckets = config["buckets"]
        .as_array()
        .unwrap_or(&Vec::new())
        .iter()
        .filter(|b| b["bucket"].as_str() != Some(bucket.as_str()))
        .cloned()
        .collect::<Vec<_>>();
    match save_backblaze_config(serde_json::json!({"buckets": buckets})) {
        Ok(()) => (StatusCode::OK, Json(serde_json::json!({"ok": true}))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"ok": false, "reason": e})),
        )
            .into_response(),
    }
}
async fn backblaze_items_get_route(Path(bucket): Path<String>) -> Response {
    match backblaze_config_value()
        .ok()
        .and_then(|c| backblaze_bucket(&c, &bucket))
    {
        Some(b) => (StatusCode::OK, Json(b)).into_response(),
        None => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"ok": false, "reason": "bucket not configured"})),
        )
            .into_response(),
    }
}
async fn backblaze_item_post_route(
    headers: axum::http::HeaderMap,
    Path(bucket): Path<String>,
    Json(item): Json<serde_json::Value>,
) -> Response {
    backblaze_item_mutate(headers, bucket, item, false).await
}
async fn backblaze_item_delete_route(
    headers: axum::http::HeaderMap,
    Path(bucket): Path<String>,
    Json(item): Json<serde_json::Value>,
) -> Response {
    backblaze_item_mutate(headers, bucket, item, true).await
}
async fn backblaze_item_mutate(
    headers: axum::http::HeaderMap,
    bucket: String,
    item: serde_json::Value,
    remove: bool,
) -> Response {
    let r = caduceus_staff_transition(
        &mutation_authority(),
        &headers,
        "POST",
        "/api/backblaze/buckets/:bucket/items",
        "backblaze item",
        item.clone(),
    );
    if !r.ok {
        return (
            mutation_response_status(&r),
            Json(serde_json::json!({"ok": false, "reason": r.first_missing_signal})),
        )
            .into_response();
    }
    let c = match backblaze_config_value() {
        Ok(v) => v,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({"ok": false, "reason": e})),
            )
                .into_response()
        }
    };
    let Some(mut b) = backblaze_bucket(&c, &bucket) else {
        return (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"ok": false})),
        )
            .into_response();
    };
    let path = item["path"].as_str().unwrap_or("");
    if !path.starts_with('/') {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"ok": false, "reason": "path must be absolute"})),
        )
            .into_response();
    }
    let mut items = b["items"].as_array().cloned().unwrap_or_default();
    items.retain(|i| i["path"].as_str() != Some(path));
    if !remove {
        items.push(serde_json::json!({"path": path}));
    }
    b["items"] = serde_json::json!(items);
    let buckets = c["buckets"]
        .as_array()
        .unwrap_or(&Vec::new())
        .iter()
        .map(|x| {
            if x["bucket"].as_str() == Some(bucket.as_str()) {
                b.clone()
            } else {
                x.clone()
            }
        })
        .collect::<Vec<_>>();
    match save_backblaze_config(serde_json::json!({"buckets": buckets})) {
        Ok(()) => (StatusCode::OK, Json(serde_json::json!({"ok": true}))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"ok": false, "reason": e})),
        )
            .into_response(),
    }
}
async fn backblaze_toggle_route(
    headers: axum::http::HeaderMap,
    Path(bucket): Path<String>,
    Json(body): Json<serde_json::Value>,
) -> Response {
    let r = caduceus_staff_transition(
        &mutation_authority(),
        &headers,
        "POST",
        "/api/backblaze/buckets/:bucket/toggle",
        "backblaze encryption",
        body.clone(),
    );
    if !r.ok {
        return (
            mutation_response_status(&r),
            Json(serde_json::json!({"ok": false})),
        )
            .into_response();
    }
    let c = match backblaze_config_value() {
        Ok(v) => v,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({"ok": false, "reason": e})),
            )
                .into_response()
        }
    };
    let buckets = c["buckets"]
        .as_array()
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .map(|mut b| {
            if b["bucket"].as_str() == Some(bucket.as_str()) {
                b["encrypted"] = serde_json::json!(body["encrypted"].as_bool().unwrap_or(true));
            }
            b
        })
        .collect::<Vec<_>>();
    match save_backblaze_config(serde_json::json!({"buckets": buckets})) {
        Ok(()) => (StatusCode::OK, Json(serde_json::json!({"ok": true}))).into_response(),
        Err(_e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"ok": false})),
        )
            .into_response(),
    }
}

fn execute_backblaze_bucket(bucket: String) {
    let _guard = BACKBLAZE_RUN_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|p| p.into_inner());
    let result = (|| -> Result<(), String> {
        let config = backblaze_config_value()?
            .get("buckets")
            .and_then(|v| v.as_array())
            .and_then(|bs| {
                bs.iter()
                    .find(|b| b["bucket"].as_str() == Some(bucket.as_str()))
                    .cloned()
            })
            .ok_or("bucket not configured")?;
        let credentials = keyman_credentials(&bucket)?;
        let auth = authorize_and_list(&bucket, &credentials)?;
        let paths = config["items"]
            .as_array()
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .filter_map(|i| i["path"].as_str().map(str::to_owned))
            .collect::<Vec<_>>();
        if config["encrypted"].as_bool().unwrap_or(true) {
            restic_backup(
                &format!("b2:{}:{}", bucket, config["prefix"].as_str().unwrap_or("")),
                &paths,
                &credentials,
            )
        } else {
            b2_upload_files(&auth, &paths, config["prefix"].as_str().unwrap_or("")).map(|_| ())
        }
    })();
    let mut state = backblaze_state();
    let entry = state.buckets.entry(bucket).or_default();
    entry.running = false;
    entry.last_run_at = Some(now_unix_seconds());
    entry.last_run_ok = Some(result.is_ok());
    entry.last_error = result.err();
    let _ = write_backblaze_state(&state);
}
async fn backblaze_run_bucket_route(
    headers: axum::http::HeaderMap,
    Path(bucket): Path<String>,
) -> Response {
    let r = caduceus_staff_transition(
        &mutation_authority(),
        &headers,
        "POST",
        "/api/backblaze/buckets/:bucket/run",
        "backblaze backup",
        serde_json::json!({"bucket": bucket}),
    );
    if !r.ok {
        return (
            mutation_response_status(&r),
            Json(serde_json::json!({"ok": false})),
        )
            .into_response();
    }
    let _guard = BACKBLAZE_RUN_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let mut state = backblaze_state();
    let entry = state.buckets.entry(bucket.clone()).or_default();
    if entry.running {
        return (
            StatusCode::CONFLICT,
            Json(serde_json::json!({"ok": false, "reason": "backup already running"})),
        )
            .into_response();
    }
    entry.running = true;
    entry.last_error = None;
    entry.last_run_at = Some(now_unix_seconds());
    if let Err(e) = write_backblaze_state(&state) {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({"ok": false, "reason": e})),
        )
            .into_response();
    }
    std::thread::spawn(move || execute_backblaze_bucket(bucket));
    (
        StatusCode::ACCEPTED,
        Json(serde_json::json!({"ok": true, "status": "queued"})),
    )
        .into_response()
}
async fn backblaze_status_route() -> Response {
    match backblaze_config_value() {
        Ok(config) => {
            let state = backblaze_state();
            let buckets = config.get("buckets").and_then(|v| v.as_array()).cloned().unwrap_or_default();
            let running = buckets.iter().any(|b| b["bucket"].as_str().and_then(|n| state.buckets.get(n)).is_some_and(|x| x.running));
            let last = buckets.iter().filter_map(|b| b["bucket"].as_str().and_then(|n| state.buckets.get(n))).max_by_key(|x| x.last_run_at.unwrap_or(0));
            (StatusCode::OK, Json(serde_json::json!({"buckets": backblaze_public(config)["buckets"], "running": running, "last_run_at": last.and_then(|x| x.last_run_at), "last_run_ok": last.and_then(|x| x.last_run_ok)}))).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"ok": false, "reason": e}))).into_response(),
    }
}

#[derive(Debug, Deserialize)]
struct BackblazeSnapshotsQuery {
    bucket: Option<String>,
}

fn backblaze_snapshot(value: &serde_json::Value, bucket: &str) -> serde_json::Value {
    serde_json::json!({
        "bucket": bucket,
        "id": value.get("id").and_then(serde_json::Value::as_str).unwrap_or_default(),
        "short_id": value.get("short_id").and_then(serde_json::Value::as_str).unwrap_or_default(),
        "time": value.get("time").and_then(serde_json::Value::as_str).unwrap_or_default(),
        "paths": value.get("paths").cloned().unwrap_or_else(|| serde_json::json!([])),
        "hostname": value.get("hostname").and_then(serde_json::Value::as_str).unwrap_or_default(),
        "summary": value.get("summary").cloned().unwrap_or(serde_json::Value::Null),
    })
}

async fn backblaze_snapshots_route(Query(query): Query<BackblazeSnapshotsQuery>) -> Response {
    let config = match backblaze_config_value() {
        Ok(config) => config,
        Err(reason) => return (StatusCode::OK, Json(serde_json::json!({"ok": false, "reason": reason}))).into_response(),
    };
    let buckets = config.get("buckets").and_then(|v| v.as_array()).cloned().unwrap_or_default();
    let mut snapshots = Vec::new();
    for bucket_config in buckets {
        let Some(bucket) = bucket_config.get("bucket").and_then(|v| v.as_str()) else { continue };
        if query.bucket.as_deref().is_some_and(|requested| requested != bucket) { continue; }
        if !bucket_config.get("encrypted").and_then(|v| v.as_bool()).unwrap_or(true) { continue; }
        let credentials = match keyman_credentials(bucket) {
            Ok(credentials) => credentials,
            Err(reason) => return (StatusCode::OK, Json(serde_json::json!({"ok": false, "reason": reason}))).into_response(),
        };
        let repo = format!("b2:{}:{}", bucket, bucket_config.get("prefix").and_then(|v| v.as_str()).unwrap_or(""));
        let output = match Command::new("restic")
            .args(["--repo", repo.as_str(), "snapshots", "--json"])
            .env("RESTIC_PASSWORD", restic_password(&credentials.application_key))
            .env("B2_ACCOUNT_ID", &credentials.key_id)
            .env("B2_ACCOUNT_KEY", &credentials.application_key)
            .output()
        {
            Ok(output) if output.status.success() => output,
            Ok(_) => return (StatusCode::OK, Json(serde_json::json!({"ok": false, "reason": "restic could not read snapshots"}))).into_response(),
            Err(_) => return (StatusCode::OK, Json(serde_json::json!({"ok": false, "reason": "restic could not start"}))).into_response(),
        };
        match serde_json::from_slice::<serde_json::Value>(&output.stdout) {
            Ok(serde_json::Value::Array(values)) => snapshots.extend(values.iter().map(|value| backblaze_snapshot(value, bucket))),
            _ => return (StatusCode::OK, Json(serde_json::json!({"ok": false, "reason": "restic returned an unreadable snapshots response"}))).into_response(),
        }
    }
    (StatusCode::OK, Json(serde_json::json!({"ok": true, "snapshots": snapshots}))).into_response()
}

async fn backblaze_config_get_route() -> Response {
    match backblaze_config_value() { Ok(config) => (StatusCode::OK, Json(backblaze_public(config))).into_response(), Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"ok": false, "reason": e}))).into_response() }
}

async fn backblaze_config_post_route(headers: axum::http::HeaderMap, Json(body): Json<serde_json::Value>) -> Response {
    let r = caduceus_staff_transition(&mutation_authority(), &headers, "POST", "/api/backblaze/config", "backblaze config", body.clone());
    if !r.ok { return (mutation_response_status(&r), Json(serde_json::json!({"ok": false, "reason": r.first_missing_signal}))).into_response(); }
    match normalized_config(body).and_then(save_backblaze_config) { Ok(()) => (StatusCode::OK, Json(serde_json::json!({"ok": true}))).into_response(), Err(e) => (StatusCode::BAD_REQUEST, Json(serde_json::json!({"ok": false, "reason": e}))).into_response() }
}

async fn backblaze_run_route(headers: axum::http::HeaderMap, Json(body): Json<serde_json::Value>) -> Response {
    let bucket = body.get("bucket").and_then(|v| v.as_str()).map(str::to_owned).or_else(|| backblaze_config_value().ok().and_then(|c| c["buckets"].as_array().and_then(|bs| bs.first()).and_then(|b| b["bucket"].as_str()).map(str::to_owned)));
    match bucket { Some(bucket) => backblaze_run_bucket_route(headers, Path(bucket)).await, None => (StatusCode::BAD_REQUEST, Json(serde_json::json!({"ok": false, "reason": "no bucket configured"}))).into_response() }
}

fn now_unix_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
