// Document-attendance projection seam. Coronatio never owns a session, ticket, or capability.
// PIN material crosses this membrane once to Caduceus; the returned attendance proof is
// carried by the current document and its opaque value may appear only in a short-lived,
// non-authoritative process-memory projection key—never persistent storage or mutation authority.
use std::collections::HashMap;
const CADUCEUS_STAFF_SOCKET_DEFAULT: &str = "/run/caduceus/staff.sock";
const CADUCEUS_ACCESS_TIMEOUT: Duration = Duration::from_secs(3);
const ATTENDANCE_PROJECTION_TTL: Duration = Duration::from_secs(5);
const CADUCEUS_ACCESS_MAX_RESPONSE: usize = 16 * 1024;

#[derive(Clone)]
pub(crate) struct CaduceusAccessClient { socket: PathBuf, timeout: Duration }
impl Default for CaduceusAccessClient {
    fn default() -> Self {
        Self::new(staff_socket_path())
    }
}
impl CaduceusAccessClient {
    pub(crate) fn new(socket: impl Into<PathBuf>) -> Self { Self { socket: socket.into(), timeout: CADUCEUS_ACCESS_TIMEOUT } }
    pub(crate) fn attendance_open(&self, pin: &str, document: &str) -> AttendanceCall { self.call(AttendanceOperation::Open, serde_json::json!({"pin":pin,"documentId":document,"documentIncarnation":document})) }
    pub(crate) fn attendance_open_raw_envelope(&self, raw_envelope: &[u8]) -> AttendanceCall { self.call_raw(AttendanceOperation::Open, raw_envelope) }
    pub(crate) fn attendance_validate(&self, attendance: &AttendanceProof, document: &str) -> AttendanceCall { self.call(AttendanceOperation::Validate, serde_json::json!({"attendance":attendance.expose(),"documentId":document,"documentIncarnation":document})) }
    pub(crate) fn attendance_open_scoped(&self, attendance: &AttendanceProof, document: &str, target: &str) -> AttendanceCall {
        self.call(AttendanceOperation::Open, serde_json::json!({
            "schema": "caduceus.staff.v1",
            "intent_id": format!("coronatio-exousia-{}", uuid::Uuid::new_v4()),
            "transition": "exousia.open",
            "version": {"future": true},
            "timestamp": std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|value| value.as_secs().to_string())
                .unwrap_or_else(|_| "0".to_string()),
            "target": {"document": target},
            "flags": {"exousia": {
                "attendance": attendance.expose(),
                "documentId": document,
                "documentIncarnation": document,
            }},
            "payload": {},
        }))
    }
    pub(crate) fn attendance_touch(&self, attendance: &AttendanceProof, document: &str) -> AttendanceCall { self.call(AttendanceOperation::Touch, serde_json::json!({"attendance":attendance.expose(),"documentId":document,"documentIncarnation":document})) }
    pub(crate) fn attendance_change_pin(&self, attendance: &AttendanceProof, document: &str, current_pin: &str, new_pin: &str) -> AttendanceCall { self.call(AttendanceOperation::ChangePin, serde_json::json!({"attendance":attendance.expose(),"documentId":document,"documentIncarnation":document,"currentPin":current_pin,"newPin":new_pin})) }
    pub(crate) fn attendance_invalidate(&self, attendance: &AttendanceProof, document: &str) -> AttendanceCall { self.call(AttendanceOperation::Invalidate, serde_json::json!({"attendance":attendance.expose(),"documentId":document,"documentIncarnation":document})) }
    async fn call_async(&self, operation: AttendanceOperation, body: serde_json::Value) -> AttendanceCall {
        let client = self.clone();
        tokio::task::spawn_blocking(move || client.call(operation, body))
            .await
            .unwrap_or_else(|_| AttendanceCall::refused(operation, 0, "caduceus-attendance-task-failed"))
    }
    pub(crate) async fn attendance_open_async(&self, pin: String, document: String) -> AttendanceCall {
        self.call_async(AttendanceOperation::Open, serde_json::json!({"pin":pin,"documentId":document,"documentIncarnation":document})).await
    }
    pub(crate) async fn attendance_validate_async(&self, attendance: AttendanceProof, document: String) -> AttendanceCall {
        self.call_async(AttendanceOperation::Validate, serde_json::json!({"attendance":attendance.expose(),"documentId":document,"documentIncarnation":document})).await
    }
    pub(crate) async fn attendance_touch_and_bust_async(&self, attendance: AttendanceProof, document: String) -> AttendanceCall {
        let client = self.clone();
        tokio::task::spawn_blocking(move || {
            let call = client.attendance_touch(&attendance, &document);
            if !call.receipt.ok { bust_attendance_projection(&attendance, &document, "attendance-ended"); }
            call
        }).await.unwrap_or_else(|_| AttendanceCall::refused(AttendanceOperation::Touch, 0, "caduceus-attendance-task-failed"))
    }
    pub(crate) async fn attendance_change_pin_and_bust_async(&self, attendance: AttendanceProof, document: String, current_pin: String, new_pin: String) -> AttendanceCall {
        let client = self.clone();
        tokio::task::spawn_blocking(move || {
            let call = client.attendance_change_pin(&attendance, &document, &current_pin, &new_pin);
            if call.receipt.ok { bust_all_attendance_projections("attendance-ended"); }
            call
        }).await.unwrap_or_else(|_| AttendanceCall::refused(AttendanceOperation::ChangePin, 0, "caduceus-attendance-task-failed"))
    }
    pub(crate) async fn attendance_invalidate_and_bust_async(&self, attendance: AttendanceProof, document: String) -> AttendanceCall {
        let client = self.clone();
        tokio::task::spawn_blocking(move || {
            let call = client.attendance_invalidate(&attendance, &document);
            bust_attendance_projection(&attendance, &document, "attendance-invalidated");
            call
        }).await.unwrap_or_else(|_| AttendanceCall::refused(AttendanceOperation::Invalidate, 0, "caduceus-attendance-task-failed"))
    }
    fn call(&self, operation: AttendanceOperation, body: serde_json::Value) -> AttendanceCall {
        let encoded = match serde_json::to_vec(&body) { Ok(v) => v, _ => return AttendanceCall::refused(operation, 0, "caduceus-attendance-request-invalid") };
        self.call_raw(operation, &encoded)
    }
    fn call_raw(&self, operation: AttendanceOperation, encoded: &[u8]) -> AttendanceCall {
        if encoded.len() > 4096 {
            return AttendanceCall::refused(operation, 0, "caduceus-attendance-request-invalid");
        }
        let mut stream = match UnixStream::connect(&self.socket) {
            Ok(stream) => stream,
            Err(error) => return AttendanceCall::refused(operation, 0, attendance_io_code("connect", &error)),
        };
        if stream.set_read_timeout(Some(self.timeout)).is_err() || stream.set_write_timeout(Some(self.timeout)).is_err() {
            return AttendanceCall::refused(operation, 0, "caduceus-attendance-socket-config-failed");
        }
        let request = format!("POST {} HTTP/1.1\r\nHost: caduceus.local\r\nConnection: close\r\nContent-Type: application/json\r\nAccept: application/json\r\nContent-Length: {}\r\n\r\n", operation.path(), encoded.len());
        if let Err(error) = stream.write_all(request.as_bytes()).and_then(|_| stream.write_all(encoded)) {
            return AttendanceCall::refused(operation, 0, attendance_io_code("write", &error));
        }
        parse_attendance_response(operation, &mut stream)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AttendanceOperation { Open, Validate, Touch, ChangePin, Invalidate }
impl AttendanceOperation { fn name(self)->&'static str { match self { Self::Open=>"attendance.open",Self::Validate=>"attendance.validate",Self::Touch=>"attendance.touch",Self::ChangePin=>"attendance.change-pin",Self::Invalidate=>"attendance.invalidate" } } fn path(self)->&'static str { match self { Self::Open=>"/api/v1/exousia/open",Self::Validate=>"/api/v1/exousia/validate",Self::Touch=>"/api/v1/exousia/touch",Self::ChangePin=>"/api/v1/exousia/change-pin",Self::Invalidate=>"/api/v1/exousia/invalidate" } } fn returns_proof(self)->bool { matches!(self,Self::Open) } }
#[derive(Clone, PartialEq, Eq)]
pub(crate) struct AttendanceProof(pub(crate) String);
impl std::fmt::Debug for AttendanceProof { fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result { f.write_str("AttendanceProof([redacted])") } }
impl AttendanceProof { pub(crate) fn parse(raw:&str)->Option<Self>{ opaque_secret(raw).then(||Self(raw.to_string())) } pub(crate) fn expose(&self)->&str{&self.0} }
fn opaque_secret(raw:&str)->bool { !raw.is_empty() && raw.len()<=1024 && raw.bytes().all(|b| b.is_ascii_alphanumeric()||matches!(b,b'-'|b'_'|b'.')) }
#[derive(Debug,Clone,PartialEq,Eq)]
pub(crate) struct AttendanceReceipt { pub(crate) operation:&'static str,pub(crate) ok:bool,pub(crate) status:u16,pub(crate) code:String }
#[derive(Debug,Clone)]
pub(crate) struct AttendanceCall { pub(crate) receipt:AttendanceReceipt,pub(crate) proof:Option<AttendanceProof> }
impl AttendanceCall { fn refused(op:AttendanceOperation,status:u16,code:&str)->Self{Self{receipt:AttendanceReceipt{operation:op.name(),ok:false,status,code:safe_access_code(code)},proof:None}} pub(crate) fn take_proof(self)->Option<AttendanceProof>{self.proof} }

#[derive(Clone, Hash, PartialEq, Eq)]
struct AttendanceProjectionKey { document: String, attendance: String }

struct AttendanceProjectionEntry { receipt: AttendanceReceipt, expires_at: std::time::Instant }

#[derive(Clone)]
struct AttendanceProjectionResult { receipt: AttendanceReceipt, generation: u64 }

struct AttendanceProjectionFlight { sender: tokio::sync::watch::Sender<Option<AttendanceProjectionResult>> }

struct AttendanceProjectionSlot {
    entry: Option<AttendanceProjectionEntry>,
    flight: Option<Arc<AttendanceProjectionFlight>>,
    generation: u64,
}

static ATTENDANCE_PROJECTION_CACHE: OnceLock<Mutex<HashMap<AttendanceProjectionKey, AttendanceProjectionSlot>>> = OnceLock::new();

fn attendance_projection_cache() -> &'static Mutex<HashMap<AttendanceProjectionKey, AttendanceProjectionSlot>> {
    ATTENDANCE_PROJECTION_CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

fn projection_event(action: &str, reason: &str, document: &str) {
    let safe_reason = match reason {
        "ttl-expired" | "validation-refused" | "attendance-invalidated" | "attendance-ended" | "stream-downgraded" => reason,
        _ => "projection-state-change",
    };
    let safe_document = if document.len() <= 128 && document.bytes().all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.')) { document } else { "redacted" };
    eprintln!("{}", serde_json::json!({"event":"coronatio.attendance.projection","action":action,"reason":safe_reason,"documentId":safe_document}));
}

fn purge_expired_projection_entries(cache: &mut HashMap<AttendanceProjectionKey, AttendanceProjectionSlot>) {
    let now = std::time::Instant::now();
    let expired_documents = cache.iter_mut().filter_map(|(key, slot)| {
        let expired = slot.entry.as_ref().is_some_and(|entry| entry.expires_at <= now);
        if expired { slot.entry = None; Some(key.document.clone()) } else { None }
    }).collect::<Vec<_>>();
    cache.retain(|_, slot| slot.entry.is_some() || slot.flight.is_some());
    for document in expired_documents { projection_event("expiry", "ttl-expired", &document); }
}

fn clear_attendance_projection(attendance: &AttendanceProof, document: &str) {
    let key = AttendanceProjectionKey { document: document.to_string(), attendance: attendance.expose().to_string() };
    let mut cache = attendance_projection_cache().lock().expect("attendance projection cache lock");
    purge_expired_projection_entries(&mut cache);
    let remove = if let Some(slot) = cache.get_mut(&key) {
        slot.entry = None;
        slot.generation = slot.generation.wrapping_add(1);
        slot.flight.is_none()
    } else {
        false
    };
    if remove { cache.remove(&key); }
}

pub(crate) fn fence_attendance_projection(attendance: &AttendanceProof, document: &str) {
    clear_attendance_projection(attendance, document);
}

pub(crate) fn bust_attendance_projection(attendance: &AttendanceProof, document: &str, reason: &str) {
    clear_attendance_projection(attendance, document);
    projection_event("bust", reason, document);
}

pub(crate) fn bust_all_attendance_projections(reason: &str) {
    let mut cache = attendance_projection_cache().lock().expect("attendance projection cache lock");
    purge_expired_projection_entries(&mut cache);
    for slot in cache.values_mut() {
        slot.entry = None;
        slot.generation = slot.generation.wrapping_add(1);
    }
    cache.retain(|_, slot| slot.flight.is_some());
    projection_event("bust", reason, "all");
}

fn projection_is_current(key: &AttendanceProjectionKey, generation: u64) -> bool {
    let mut cache = attendance_projection_cache().lock().expect("attendance projection cache lock");
    purge_expired_projection_entries(&mut cache);
    cache.get(key).is_some_and(|slot| slot.generation == generation && slot.entry.is_some())
}

async fn complete_attendance_projection(
    key: AttendanceProjectionKey,
    attendance: AttendanceProof,
    document: String,
    flight: Arc<AttendanceProjectionFlight>,
    generation: u64,
) {
    let call = CaduceusAccessClient::default().attendance_validate_async(attendance, document.clone()).await;
    let mut returned = call.clone();
    let result_generation;
    {
        let mut cache = attendance_projection_cache().lock().expect("attendance projection cache lock");
        let mut remove = false;
        if let Some(slot) = cache.get_mut(&key) {
            let same_flight = slot.flight.as_ref().is_some_and(|current| Arc::ptr_eq(current, &flight));
            if same_flight && slot.generation == generation && call.receipt.ok {
                slot.entry = Some(AttendanceProjectionEntry { receipt: call.receipt.clone(), expires_at: std::time::Instant::now() + ATTENDANCE_PROJECTION_TTL });
            } else if same_flight && slot.generation == generation && !call.receipt.ok {
                slot.entry = None;
                slot.generation = slot.generation.wrapping_add(1);
                projection_event("bust", "validation-refused", &key.document);
            } else if call.receipt.ok {
                returned = AttendanceCall::refused(AttendanceOperation::Validate, 401, "caduceus-attendance-invalidated");
            }
            result_generation = slot.generation;
            if same_flight { slot.flight = None; }
            remove = slot.entry.is_none() && slot.flight.is_none();
        } else {
            result_generation = generation;
            returned = AttendanceCall::refused(AttendanceOperation::Validate, 401, "caduceus-attendance-invalidated");
        }
        if remove { cache.remove(&key); }
    }
    flight.sender.send_replace(Some(AttendanceProjectionResult { receipt: returned.receipt, generation: result_generation }));
}

pub(crate) async fn attendance_projection_call(attendance: AttendanceProof, document: String) -> AttendanceCall {
    let key = AttendanceProjectionKey { document: document.clone(), attendance: attendance.expose().to_string() };
    let (flight, leader, generation) = {
        let mut cache = attendance_projection_cache().lock().expect("attendance projection cache lock");
        purge_expired_projection_entries(&mut cache);
        let slot = cache.entry(key.clone()).or_insert_with(|| AttendanceProjectionSlot { entry: None, flight: None, generation: 0 });
        if let Some(entry) = slot.entry.as_ref().filter(|entry| entry.expires_at > std::time::Instant::now()) {
            return AttendanceCall { receipt: entry.receipt.clone(), proof: None };
        }
        if let Some(flight) = slot.flight.as_ref() {
            (Arc::clone(flight), false, slot.generation)
        } else {
            let (sender, _) = tokio::sync::watch::channel(None);
            let flight = Arc::new(AttendanceProjectionFlight { sender });
            let generation = slot.generation;
            slot.flight = Some(Arc::clone(&flight));
            (flight, true, generation)
        }
    };
    if leader {
        let flight_for_task = Arc::clone(&flight);
        tokio::spawn(complete_attendance_projection(key.clone(), attendance, document, flight_for_task, generation));
    }
    let mut receiver = flight.sender.subscribe();
    let pending = receiver.borrow().is_none();
    if pending && receiver.changed().await.is_err() {
        return AttendanceCall::refused(AttendanceOperation::Validate, 503, "caduceus-attendance-projection-coalescing-failed");
    }
    let Some(result) = receiver.borrow().clone() else { return AttendanceCall::refused(AttendanceOperation::Validate, 503, "caduceus-attendance-projection-coalescing-failed"); };
    let receipt = if result.receipt.ok && !projection_is_current(&key, result.generation) { AttendanceCall::refused(AttendanceOperation::Validate, 401, "caduceus-attendance-invalidated").receipt } else { result.receipt };
    AttendanceCall { receipt, proof: None }
}
pub(crate) fn safe_access_code(value:&str)->String { if value.len()<=100&&value.bytes().all(|b|b.is_ascii_alphabetic()||b.is_ascii_digit()||b==b'-'){value.to_string()}else{"caduceus-attendance-refused".to_string()} }
fn attendance_io_code(stage: &str, error: &std::io::Error) -> &'static str {
    match error.kind() {
        std::io::ErrorKind::ConnectionRefused if stage == "connect" => "caduceus-attendance-connect-refused",
        std::io::ErrorKind::TimedOut | std::io::ErrorKind::WouldBlock => "caduceus-attendance-timeout",
        _ if stage == "connect" => "caduceus-attendance-connect-failed",
        _ => "caduceus-attendance-write-failed",
    }
}

fn read_failure_code(error: &std::io::Error) -> &'static str {
    match error.kind() {
        std::io::ErrorKind::TimedOut | std::io::ErrorKind::WouldBlock => "caduceus-attendance-timeout",
        _ => "caduceus-attendance-bad-receipt",
    }
}

fn parse_attendance_response(op: AttendanceOperation, stream: &mut UnixStream) -> AttendanceCall {
    let mut reader = BufReader::new(stream);
    let mut status_line = String::new();
    match reader.read_line(&mut status_line) {
        Ok(n) if n > 0 => {}
        Ok(_) => return AttendanceCall::refused(op, 0, "caduceus-attendance-bad-receipt"),
        Err(error) => return AttendanceCall::refused(op, 0, read_failure_code(&error)),
    }
    let status = status_line.split_whitespace().nth(1).and_then(|value| value.parse().ok()).unwrap_or(0);
    if status == 0 {
        return AttendanceCall::refused(op, 0, "caduceus-attendance-bad-receipt");
    }
    let mut length = None;
    let mut header_bytes = 0;
    loop {
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(0) => return AttendanceCall::refused(op, status, "caduceus-attendance-bad-receipt"),
            Err(error) => return AttendanceCall::refused(op, status, read_failure_code(&error)),
            Ok(_) => {}
        }
        header_bytes += line.len();
        if header_bytes > 4096 {
            return AttendanceCall::refused(op, status, "caduceus-attendance-bad-receipt");
        }
        if line == "\r\n" {
            break;
        }
        if let Some((name, value)) = line.split_once(':') {
            if name.eq_ignore_ascii_case("content-length") {
                length = value.trim().parse().ok();
            }
        }
    }
    let Some(length) = length else { return AttendanceCall::refused(op, status, "caduceus-attendance-bad-receipt"); };
    if length > CADUCEUS_ACCESS_MAX_RESPONSE {
        return AttendanceCall::refused(op, status, "caduceus-attendance-bad-receipt");
    }
    let mut body = vec![0; length];
    if let Err(error) = reader.read_exact(&mut body) {
        return AttendanceCall::refused(op, status, read_failure_code(&error));
    }
    let Ok(value) = serde_json::from_slice::<serde_json::Value>(&body) else { return AttendanceCall::refused(op, status, "caduceus-attendance-bad-receipt"); };
    let Some(object) = value.as_object() else { return AttendanceCall::refused(op, status, "caduceus-attendance-bad-receipt"); };
    let ok = object.get("ok").and_then(|value| value.as_bool()).unwrap_or(false);
    let code = object
        .get("code")
        .or_else(|| object.get("firstMissingSignal"))
        .or_else(|| object.get("first_missing_signal"))
        .and_then(|value| value.as_str())
        .unwrap_or(if ok { "none" } else { "caduceus-attendance-refused" });
    if status < 200 || status >= 300 || !ok {
        return AttendanceCall::refused(op, status, code);
    }
    let proof = if op.returns_proof() {
        object.get("attendance").or_else(|| object.get("proof")).and_then(|value| value.as_str()).and_then(AttendanceProof::parse)
    } else {
        None
    };
    if op.returns_proof() && proof.is_none() {
        return AttendanceCall::refused(op, status, "caduceus-attendance-bad-receipt");
    }
    AttendanceCall { receipt: AttendanceReceipt { operation: op.name(), ok: true, status, code: "none".to_string() }, proof }
}
pub(crate) fn staff_socket_path() -> PathBuf {
    if let Some(path) = env::var_os("CADUCEUS_STAFF_SOCKET").filter(|value| !value.is_empty()) {
        return PathBuf::from(path);
    }
    #[cfg(test)]
    {
        return test_fixture::socket_path();
    }
    #[cfg(not(test))]
    {
        PathBuf::from(CADUCEUS_STAFF_SOCKET_DEFAULT)
    }
}

pub(crate) fn document_incarnation_from_headers(headers:&axum::http::HeaderMap)->Option<String>{headers.get("x-caduceus-document").and_then(|v|v.to_str().ok()).map(str::trim).filter(|v|!v.is_empty()&&v.len()<=128&&v.bytes().all(|b|b.is_ascii_alphanumeric()||matches!(b,b'-'|b'_'|b'.'))).map(ToOwned::to_owned)}
pub(crate) fn attendance_from_headers(headers:&axum::http::HeaderMap)->Option<AttendanceProof>{headers.get("x-caduceus-attendance").and_then(|v|v.to_str().ok()).and_then(AttendanceProof::parse)}
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct MutationOriginPolicy { allowed_origins: Vec<BrowserOrigin> }

#[derive(Clone, Debug, PartialEq, Eq)]
struct BrowserOrigin { scheme: String, host: String, port: u16 }

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum MutationOriginConfigError { HomeserverJson(String), MissingAllowedOrigins, EmptyAllowedOrigins, InvalidAllowedOrigins, InvalidOrigin(String) }

pub(crate) fn mutation_origin_policy_from_homeserver(value: &serde_json::Value) -> Result<MutationOriginPolicy, MutationOriginConfigError> {
    let origins = value.get("global").and_then(serde_json::Value::as_object).and_then(|global| global.get("cors")).and_then(serde_json::Value::as_object).and_then(|cors| cors.get("allowed_origins")).ok_or(MutationOriginConfigError::MissingAllowedOrigins)?;
    let origins = origins.as_array().ok_or(MutationOriginConfigError::InvalidAllowedOrigins)?;
    if origins.is_empty() { return Err(MutationOriginConfigError::EmptyAllowedOrigins); }
    let allowed_origins = origins.iter().map(|entry| {
        let raw = entry.as_str().ok_or(MutationOriginConfigError::InvalidAllowedOrigins)?;
        parse_browser_origin(raw).ok_or_else(|| MutationOriginConfigError::InvalidOrigin(raw.to_string()))
    }).collect::<Result<Vec<_>, _>>()?;
    Ok(MutationOriginPolicy { allowed_origins })
}

pub(crate) fn load_mutation_origin_policy_sync() -> Result<MutationOriginPolicy, MutationOriginConfigError> {
    let (_, value) = crate::load_homeserver_json_sync().map_err(MutationOriginConfigError::HomeserverJson)?;
    mutation_origin_policy_from_homeserver(&value)
}

pub(crate) fn same_origin_state_change_with_policy(headers: &axum::http::HeaderMap, policy: &MutationOriginPolicy) -> bool {
    let origin = match headers.get(header::ORIGIN) {
        Some(value) => match value.to_str().ok().and_then(parse_browser_origin) {
            Some(origin) => origin,
            None => return false,
        },
        None => return headers
            .get("sec-fetch-site")
            .and_then(|value| value.to_str().ok())
            .is_some_and(|site| site == "same-origin"),
    };
    let Some(matched) = policy.allowed_origins.iter().find(|allowed| **allowed == origin) else { return false; };
    let Some(host) = forwarded_first(headers, "x-forwarded-host").or_else(|| headers.get(header::HOST).and_then(|value| value.to_str().ok())).and_then(parse_host_authority) else { return false; };
    if host.0 != matched.host || host.1.unwrap_or(matched.port) != matched.port { return false; }
    match forwarded_first(headers, "x-forwarded-proto") { Some(proto) => proto.eq_ignore_ascii_case(&matched.scheme), None => true }
}

pub(crate) fn same_origin_state_change(headers: &axum::http::HeaderMap) -> bool {
    load_mutation_origin_policy_sync().map(|policy| same_origin_state_change_with_policy(headers, &policy)).unwrap_or(false)
}

fn parse_browser_origin(raw: &str) -> Option<BrowserOrigin> {
    let url = url::Url::parse(raw.trim()).ok()?;
    if !matches!(url.scheme(), "http" | "https") || url.cannot_be_a_base() || url.username() != "" || url.password().is_some() || url.path() != "/" || url.query().is_some() || url.fragment().is_some() { return None; }
    Some(BrowserOrigin { scheme: url.scheme().to_ascii_lowercase(), host: url.host_str()?.to_ascii_lowercase(), port: url.port_or_known_default()? })
}

fn parse_host_authority(raw: &str) -> Option<(String, Option<u16>)> {
    let raw = raw.trim();
    if raw.is_empty() || raw.contains(['/', '?', '#', '@']) { return None; }
    let url = url::Url::parse(&format!("http://{raw}/")).ok()?;
    if url.username() != "" || url.password().is_some() || url.path() != "/" || url.query().is_some() || url.fragment().is_some() { return None; }
    Some((url.host_str()?.to_ascii_lowercase(), url.port()))
}

fn forwarded_first<'a>(headers:&'a axum::http::HeaderMap,name:&str)->Option<&'a str>{headers.get(name)?.to_str().ok()?.split(',').next().map(str::trim).filter(|v|!v.is_empty())}

#[cfg(test)]
mod mutation_origin_policy_tests {
    use super::*;
    fn policy(origins: serde_json::Value) -> MutationOriginPolicy { mutation_origin_policy_from_homeserver(&serde_json::json!({"global":{"cors":{"allowed_origins":origins}}})).unwrap() }
    fn headers(origin: &str, host: &str, proto: Option<&str>) -> axum::http::HeaderMap { let mut headers=axum::http::HeaderMap::new(); headers.insert(header::ORIGIN, origin.parse().unwrap()); headers.insert(header::HOST, host.parse().unwrap()); if let Some(proto)=proto { headers.insert("x-forwarded-proto", proto.parse().unwrap()); } headers }
    #[test]
    fn configured_origins_require_one_matching_origin_host_proto_tuple() {
        let policy=policy(serde_json::json!(["https://home.arpa", "http://home.arpa:3013"]));
        assert!(same_origin_state_change_with_policy(&headers("https://home.arpa:443", "home.arpa", Some("https")), &policy));
        assert!(same_origin_state_change_with_policy(&headers("http://home.arpa:3013", "home.arpa:3013", None), &policy));
        assert!(!same_origin_state_change_with_policy(&headers("https://evil.example", "evil.example", Some("https")), &policy));
        assert!(!same_origin_state_change_with_policy(&headers("https://home.arpa", "evil.example", Some("https")), &policy));
        assert!(!same_origin_state_change_with_policy(&headers("https://home.arpa", "home.arpa", Some("http")), &policy));
    }
    #[test]
    fn malformed_absent_or_empty_configuration_is_refused() { for value in [serde_json::json!({}), serde_json::json!({"global":{"cors":{"allowed_origins":[]}}}), serde_json::json!({"global":{"cors":{"allowed_origins":["https://home.arpa/path"]}}})] { assert!(mutation_origin_policy_from_homeserver(&value).is_err()); } }
}
#[cfg(test)]
pub(crate) mod test_fixture {
    use super::*;
    use std::io::{BufRead, BufReader, Read, Write};
    use std::os::unix::net::{UnixListener, UnixStream};
    use std::sync::{Mutex, OnceLock};

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub(crate) struct TestRecord {
        pub(crate) path: String,
        pub(crate) action: Option<String>,
        pub(crate) target: Option<String>,
    }

    struct Fixture {
        path: PathBuf,
        records: std::sync::Arc<Mutex<Vec<TestRecord>>>,
    }

    static FIXTURE: OnceLock<Fixture> = OnceLock::new();

    fn fixture() -> &'static Fixture {
        FIXTURE.get_or_init(|| {
            let path = std::env::temp_dir().join(format!(
                "coronatio-caduceus-fixture-{}-{}.sock",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .expect("system clock before unix epoch")
                    .as_nanos()
            ));
            let _ = std::fs::remove_file(&path);
            let listener = UnixListener::bind(&path).expect("bind caduceus test fixture socket");
            let records = std::sync::Arc::new(Mutex::new(Vec::new()));
            let fixture = Fixture { path, records: std::sync::Arc::clone(&records) };
            std::thread::spawn(move || {
                for stream in listener.incoming() {
                    let Ok(stream) = stream else { continue };
                    serve(stream, records.as_ref());
                }
            });
            fixture
        })
    }

    pub(crate) fn socket_path() -> PathBuf { fixture().path.clone() }

    pub(crate) fn same_origin_headers(with_attendance: bool) -> axum::http::HeaderMap {
        let mut headers = axum::http::HeaderMap::new();
        headers.insert("origin", axum::http::HeaderValue::from_static("https://home.arpa"));
        headers.insert("host", axum::http::HeaderValue::from_static("home.arpa"));
        headers.insert("x-caduceus-document", axum::http::HeaderValue::from_static("test-document"));
        if with_attendance {
            headers.insert("x-caduceus-attendance", axum::http::HeaderValue::from_static("test-attendance"));
        }
        headers
    }

    pub(crate) fn mark() -> usize { fixture().records.lock().unwrap().len() }

    pub(crate) fn records_since(mark: usize) -> Vec<TestRecord> {
        fixture().records.lock().unwrap().get(mark..).unwrap_or(&[]).to_vec()
    }

    fn serve(stream: UnixStream, records: &Mutex<Vec<TestRecord>>) {
        let mut reader = BufReader::new(stream);
        let mut head = String::new();
        loop {
            let mut line = String::new();
            if reader.read_line(&mut line).ok().filter(|count| *count > 0).is_none() { return; }
            head.push_str(&line);
            if line == "\r\n" { break; }
        }
        let length = head.lines().find_map(|line| {
            line.split_once(':').and_then(|(name, value)|
                name.eq_ignore_ascii_case("content-length").then(|| value.trim().parse::<usize>().ok()).flatten())
        }).unwrap_or(0);
        let mut body = vec![0; length];
        if reader.read_exact(&mut body).is_err() { return; }
        let path = head.split_whitespace().nth(1).unwrap_or("/").to_string();
        let value = serde_json::from_slice::<serde_json::Value>(&body).ok();
        let (action, target) = if path == "/api/v1/config/set" {
            (Some("coronatio.config.set".to_string()), value.as_ref().and_then(|v| v.get("path")).and_then(|v| v.as_str()).map(str::to_string))
        } else { (None, None) };
        records.lock().unwrap().push(TestRecord { path: path.clone(), action, target });
        let response = if path == "/api/v1/exousia/open" {
            r#"{"ok":true,"attendance":"test-attendance"}"#
        } else {
            r#"{"ok":true,"firstMissingSignal":"none"}"#
        };
        let mut stream = reader.into_inner();
        let _ = write!(stream, "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nConnection: close\r\nContent-Length: {}\r\n\r\n{}", response.len(), response);
    }
}

#[cfg(test)]
mod attendance_uds_tests {
    use super::*;
    use std::io::{BufRead, BufReader, Read, Write};
    use std::os::unix::net::UnixListener;
    use std::sync::{Arc, Mutex};

    #[test]
    fn attendance_open_uses_http_over_uds_and_parses_proof() {
        let socket = std::env::temp_dir().join(format!("coronatio-attendance-{}-{}.sock", std::process::id(), uuid::Uuid::new_v4()));
        let listener = UnixListener::bind(&socket).unwrap();
        let request = Arc::new(Mutex::new(String::new()));
        let captured = Arc::clone(&request);
        let worker = std::thread::spawn(move || {
            let (stream, _) = listener.accept().unwrap();
            let mut reader = BufReader::new(stream);
            let mut head = String::new();
            loop { let mut line = String::new(); reader.read_line(&mut line).unwrap(); head.push_str(&line); if line == "\r\n" { break; } }
            let length = head.lines().find_map(|line| line.strip_prefix("Content-Length: ").and_then(|v| v.parse::<usize>().ok())).unwrap();
            let mut body = vec![0; length]; reader.read_exact(&mut body).unwrap();
            head.push_str(std::str::from_utf8(&body).unwrap()); *captured.lock().unwrap() = head;
            let response = b"{\"ok\":true,\"attendance\":\"real-attendance-proof\"}";
            let mut stream = reader.into_inner(); write!(stream, "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n", response.len()).unwrap(); stream.write_all(response).unwrap();
        });
        let result = CaduceusAccessClient::new(&socket).attendance_open("1234", "document-1");
        worker.join().unwrap();
        let text = request.lock().unwrap().clone();
        assert!(text.starts_with("POST /api/v1/exousia/open HTTP/1.1\r\n")); assert!(text.contains("Host: caduceus.local\r\n")); assert!(text.contains("Content-Length: ")); assert!(text.ends_with("{\"pin\":\"1234\",\"documentId\":\"document-1\",\"documentIncarnation\":\"document-1\"}"));
        assert!(result.receipt.ok); assert_eq!(result.take_proof().unwrap().expose(), "real-attendance-proof"); let _ = std::fs::remove_file(socket);
    }

    #[test]
    fn attendance_open_refuses_guaranteed_absent_socket() {
        let socket = std::env::temp_dir().join(format!("coronatio-attendance-absent-{}-{}.sock", std::process::id(), uuid::Uuid::new_v4())); let _ = std::fs::remove_file(&socket);
        let result = CaduceusAccessClient::new(socket).attendance_open("1234", "document-1"); assert!(!result.receipt.ok); assert_eq!(result.receipt.code, "caduceus-attendance-connect-failed");
    }
}
