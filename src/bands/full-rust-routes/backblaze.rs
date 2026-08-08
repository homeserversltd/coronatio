const BACKBLAZE_STATE_PATH: &str = "/var/lib/coronatio/backblaze_state.json";
const BACKBLAZE_KEYMAN_EXPORT: &str = "/vault/keyman/exportkey.sh";
const BACKBLAZE_KEY_EXCHANGE: &str = "/mnt/keyexchange";
const BACKBLAZE_RESTIC_SALT: &[u8] = b"coronatio_backblaze_restic_v1";
static BACKBLAZE_RUN_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct BackblazeState {
    last_run_at: Option<u64>,
    last_run_ok: Option<bool>,
    last_backup_size_bytes: Option<u64>,
    #[serde(default)]
    running: bool,
}

#[derive(Debug, Clone)]
struct BackblazeConfig {
    repository: String,
    paths: Vec<String>,
}

#[derive(Debug, Clone)]
struct BackblazeCredentials {
    account_id: String,
    application_key: String,
}

fn backblaze_state_path() -> PathBuf {
    env::var("CORONATIO_BACKBLAZE_STATE")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(BACKBLAZE_STATE_PATH))
}

fn backblaze_state() -> BackblazeState {
    std::fs::read_to_string(backblaze_state_path())
        .ok()
        .and_then(|raw| serde_json::from_str(&raw).ok())
        .unwrap_or_default()
}

fn write_backblaze_state(state: &BackblazeState) -> Result<(), String> {
    let path = backblaze_state_path();
    let parent = path
        .parent()
        .ok_or_else(|| "backblaze-state-path-invalid".to_string())?;
    std::fs::create_dir_all(parent)
        .map_err(|_| "backblaze-state-directory-unwritable".to_string())?;
    let encoded =
        serde_json::to_vec(state).map_err(|_| "backblaze-state-encode-failed".to_string())?;
    let temporary = parent.join(format!(".backblaze_state.{}.tmp", std::process::id()));
    std::fs::write(&temporary, encoded).map_err(|_| "backblaze-state-unwritable".to_string())?;
    std::fs::rename(&temporary, path).map_err(|_| "backblaze-state-promote-failed".to_string())
}

fn backblaze_config() -> Result<BackblazeConfig, String> {
    let (_, homeserver) = load_homeserver_json_sync()?;
    let config = homeserver
        .get("tabs")
        .and_then(|tabs| tabs.get("backblaze"))
        .and_then(|tab| tab.get("config"))
        .and_then(serde_json::Value::as_object)
        .ok_or_else(|| {
            "backblaze configuration is missing from tabs.backblaze.config".to_string()
        })?;
    let bucket = config
        .get("bucket")
        .and_then(serde_json::Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| "backblaze bucket is missing from tabs.backblaze.config".to_string())?;
    let paths = config
        .get("paths")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| "backblaze paths are missing from tabs.backblaze.config".to_string())?
        .iter()
        .map(serde_json::Value::as_str)
        .map(|value| {
            value
                .map(str::trim)
                .filter(|path| !path.is_empty())
                .map(str::to_string)
        })
        .collect::<Option<Vec<_>>>()
        .filter(|paths| !paths.is_empty())
        .ok_or_else(|| "backblaze paths must be a non-empty list of absolute paths".to_string())?;
    if paths.iter().any(|path| !path.starts_with('/')) {
        return Err("backblaze paths must be absolute".to_string());
    }
    let prefix = config
        .get("prefix")
        .and_then(serde_json::Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty());
    let repository = match prefix {
        Some(prefix) => format!("b2:{bucket}:{prefix}"),
        None => format!("b2:{bucket}:"),
    };
    Ok(BackblazeConfig { repository, paths })
}

fn restic_available() -> bool {
    env::var_os("PATH")
        .is_some_and(|paths| env::split_paths(&paths).any(|path| path.join("restic").is_file()))
}

fn backblaze_credential_candidates() -> Vec<PathBuf> {
    let root = PathBuf::from(BACKBLAZE_KEY_EXCHANGE);
    let mut candidates = [
        "backblaze",
        "backblaze.env",
        "backblaze.json",
        "backblaze.key",
    ]
    .into_iter()
    .map(|name| root.join(name))
    .filter(|path| path.is_file())
    .collect::<Vec<_>>();
    if let Ok(entries) = std::fs::read_dir(root) {
        candidates.extend(entries.flatten().map(|entry| entry.path()).filter(|path| {
            path.is_file()
                && path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .is_some_and(|name| name.starts_with("backblaze"))
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

fn parse_backblaze_credentials(raw: &str) -> Option<BackblazeCredentials> {
    if let Ok(value) = serde_json::from_str::<serde_json::Value>(raw) {
        let account_id = credential_field(
            &value,
            &[
                "B2_ACCOUNT_ID",
                "account_id",
                "accountId",
                "key_id",
                "keyId",
            ],
        );
        let application_key = credential_field(
            &value,
            &["B2_ACCOUNT_KEY", "application_key", "applicationKey", "key"],
        );
        if let (Some(account_id), Some(application_key)) = (account_id, application_key) {
            return Some(BackblazeCredentials {
                account_id,
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
    let account_id = ["B2_ACCOUNT_ID", "B2_KEY_ID", "account_id", "key_id"]
        .into_iter()
        .find_map(|key| values.get(key).copied())
        .filter(|value| !value.is_empty())?;
    let application_key = ["B2_ACCOUNT_KEY", "B2_APPLICATION_KEY", "application_key"]
        .into_iter()
        .find_map(|key| values.get(key).copied())
        .filter(|value| !value.is_empty())?;
    Some(BackblazeCredentials {
        account_id: account_id.to_string(),
        application_key: application_key.to_string(),
    })
}

fn backblaze_credentials() -> Result<BackblazeCredentials, String> {
    let exporter = PathBuf::from(BACKBLAZE_KEYMAN_EXPORT);
    if !exporter.is_file() {
        return Err("backblaze Keyman export is not configured".to_string());
    }
    let export = Command::new(exporter)
        .arg("backblaze")
        .output()
        .map_err(|_| "backblaze Keyman export could not start".to_string())?;
    if !export.status.success() {
        return Err("backblaze Keyman credentials are not available".to_string());
    }
    backblaze_credential_candidates()
        .into_iter()
        .find_map(|path| {
            std::fs::read_to_string(path)
                .ok()
                .and_then(|raw| parse_backblaze_credentials(&raw))
        })
        .ok_or_else(|| "backblaze Keyman credentials are not available".to_string())
}

fn backblaze_preflight(include_credentials: bool) -> Result<BackblazeConfig, String> {
    if !restic_available() {
        return Err("restic is not configured".to_string());
    }
    let config = backblaze_config()?;
    if include_credentials {
        backblaze_credentials()?;
    }
    Ok(config)
}

fn backblaze_status_value() -> serde_json::Value {
    let state = backblaze_state();
    let configuration = backblaze_preflight(true);
    let (configured, reason) = match configuration {
        Ok(_) => (true, "none".to_string()),
        Err(reason) => (false, reason),
    };
    serde_json::json!({
        "last_run_at": state.last_run_at,
        "last_run_ok": state.last_run_ok,
        "last_backup_size_bytes": state.last_backup_size_bytes,
        "configured": configured,
        "running": state.running,
        "reason": reason,
    })
}

async fn backblaze_status_route() -> impl IntoResponse {
    (StatusCode::OK, Json(backblaze_status_value()))
}

fn now_unix_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn restic_password(application_key: &str) -> String {
    let mut derived = [0u8; 32];
    pbkdf2::pbkdf2_hmac::<sha2::Sha256>(
        application_key.as_bytes(),
        BACKBLAZE_RESTIC_SALT,
        100_000,
        &mut derived,
    );
    base64::Engine::encode(&base64::engine::general_purpose::STANDARD, derived)
}

fn restic_backup_size(output: &[u8]) -> Option<u64> {
    String::from_utf8_lossy(output)
        .lines()
        .filter_map(|line| serde_json::from_str::<serde_json::Value>(line).ok())
        .find_map(|value| {
            value
                .get("total_bytes_processed")
                .and_then(serde_json::Value::as_u64)
        })
}

fn run_backblaze_backup(config: BackblazeConfig, credentials: BackblazeCredentials) {
    let password = restic_password(&credentials.application_key);
    let output = Command::new("restic")
        .args(["--repo", config.repository.as_str(), "backup", "--json"])
        .args(&config.paths)
        .env("RESTIC_PASSWORD", password)
        .env("B2_ACCOUNT_ID", credentials.account_id)
        .env("B2_ACCOUNT_KEY", credentials.application_key)
        .output();
    let mut state = backblaze_state();
    state.running = false;
    state.last_run_at = Some(now_unix_seconds());
    match output {
        Ok(output) if output.status.success() => {
            state.last_run_ok = Some(true);
            state.last_backup_size_bytes = restic_backup_size(&output.stdout);
        }
        _ => state.last_run_ok = Some(false),
    }
    let _ = write_backblaze_state(&state);
}

async fn backblaze_run_route() -> Response {
    let config = match backblaze_preflight(false) {
        Ok(config) => config,
        Err(reason) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({"ok": false, "message": reason})),
            )
                .into_response()
        }
    };
    let credentials = match backblaze_credentials() {
        Ok(credentials) => credentials,
        Err(reason) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({"ok": false, "message": reason})),
            )
                .into_response()
        }
    };
    let _run_guard = BACKBLAZE_RUN_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let mut state = backblaze_state();
    if state.running {
        return (
            StatusCode::CONFLICT,
            Json(serde_json::json!({"ok": false, "message": "a backup is already running"})),
        )
            .into_response();
    }
    state.running = true;
    if let Err(reason) = write_backblaze_state(&state) {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({"ok": false, "message": reason})),
        )
            .into_response();
    }
    std::thread::spawn(move || run_backblaze_backup(config, credentials));
    (StatusCode::ACCEPTED, Json(serde_json::json!({"ok": true, "message": "Backup started. This page will refresh its status."}))).into_response()
}
