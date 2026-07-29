// Document-attendance projection seam. Coronatio never owns a session, ticket, or capability.
// PIN material crosses this membrane once to Caduceus; the returned attendance proof is
// retained only in the current document's memory and is forwarded to Caduceus actions.
const CADUCEUS_ACCESS_DEFAULT_BASE: &str = "http://127.0.0.1:3014";
const CADUCEUS_ACCESS_TIMEOUT: Duration = Duration::from_secs(3);
const CADUCEUS_ACCESS_MAX_RESPONSE: usize = 16 * 1024;

#[derive(Clone)]
pub(crate) struct CaduceusAccessClient { base: String, timeout: Duration }
impl Default for CaduceusAccessClient {
    fn default() -> Self {
        #[cfg(test)] { return Self::new(test_fixture::base()); }
        #[cfg(not(test))]
        Self::new(env::var("CADUCEUS_ACCESS_BASE_URL").or_else(|_| env::var("CADUCEUS_BASE_URL")).unwrap_or_else(|_| CADUCEUS_ACCESS_DEFAULT_BASE.to_string()))
    }
}
impl CaduceusAccessClient {
    pub(crate) fn new(base: impl Into<String>) -> Self { Self { base: normalize_access_base(base.into()), timeout: CADUCEUS_ACCESS_TIMEOUT } }
    pub(crate) fn attendance_open(&self, pin: &str, document: &str) -> AttendanceCall { self.call(AttendanceOperation::Open, serde_json::json!({"pin":pin,"documentId":document,"documentIncarnation":document})) }
    pub(crate) fn attendance_validate(&self, attendance: &AttendanceProof, document: &str) -> AttendanceCall { self.call(AttendanceOperation::Validate, serde_json::json!({"attendance":attendance.expose(),"documentId":document,"documentIncarnation":document})) }
    pub(crate) fn attendance_invalidate(&self, attendance: &AttendanceProof, document: &str) -> AttendanceCall { self.call(AttendanceOperation::Invalidate, serde_json::json!({"attendance":attendance.expose(),"documentId":document,"documentIncarnation":document})) }
    fn call(&self, operation: AttendanceOperation, body: serde_json::Value) -> AttendanceCall {
        #[cfg(test)]
        if self.base == test_fixture::base() { return test_fixture::attendance_call(operation, &body); }
        let encoded = match serde_json::to_vec(&body) { Ok(v) if v.len() <= 4096 => v, _ => return AttendanceCall::refused(operation, 0, "caduceus-attendance-request-invalid") };
        let Some(authority) = access_authority(&self.base) else { return AttendanceCall::refused(operation, 0, "caduceus-attendance-base-invalid") };
        let mut stream = match TcpStream::connect_timeout(&authority, self.timeout) {
            Ok(stream) => stream,
            Err(error) => return AttendanceCall::refused(operation, 0, attendance_io_code("connect", &error)),
        };
        if stream.set_read_timeout(Some(self.timeout)).is_err() || stream.set_write_timeout(Some(self.timeout)).is_err() {
            return AttendanceCall::refused(operation, 0, "caduceus-attendance-socket-config-failed");
        }
        let request = format!("POST {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\nContent-Type: application/json\r\nAccept: application/json\r\nContent-Length: {}\r\n\r\n", operation.path(), access_host_header(&self.base), encoded.len());
        if let Err(error) = stream.write_all(request.as_bytes()).and_then(|_| stream.write_all(&encoded)) {
            return AttendanceCall::refused(operation, 0, attendance_io_code("write", &error));
        }
        parse_attendance_response(operation, &mut stream)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AttendanceOperation { Open, Validate, Invalidate }
impl AttendanceOperation { fn name(self)->&'static str { match self { Self::Open=>"attendance.open",Self::Validate=>"attendance.validate",Self::Invalidate=>"attendance.invalidate" } } fn path(self)->&'static str { match self { Self::Open=>"/api/v1/attendance/open",Self::Validate=>"/api/v1/attendance/validate",Self::Invalidate=>"/api/v1/attendance/invalidate" } } fn returns_proof(self)->bool { matches!(self,Self::Open) } }
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
pub(crate) fn safe_access_code(value:&str)->String { if value.len()<=100&&value.bytes().all(|b|b.is_ascii_lowercase()||b.is_ascii_digit()||b==b'-'){value.to_string()}else{"caduceus-attendance-refused".to_string()} }
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

fn parse_attendance_response(op: AttendanceOperation, stream: &mut TcpStream) -> AttendanceCall {
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
fn normalize_access_base(base:String)->String{base.trim().trim_end_matches('/').to_string()}
fn access_authority(base:&str)->Option<SocketAddr>{let raw=base.strip_prefix("http://")?;if raw.contains('/') {return None} let address:SocketAddr=raw.parse().ok()?;address.ip().is_loopback().then_some(address)}
fn access_host_header(base:&str)->&str{base.strip_prefix("http://").unwrap_or("127.0.0.1:3014")}
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
    let Some(origin) = headers.get(header::ORIGIN).and_then(|value| value.to_str().ok()).and_then(parse_browser_origin) else { return false; };
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
    pub(crate) fn base() -> String { "http://127.0.0.1:9".to_string() }
    pub(super) fn attendance_call(operation: AttendanceOperation, body: &serde_json::Value) -> AttendanceCall {
        let document_id = body.get("documentId").and_then(|v| v.as_str());
        let document_incarnation = body.get("documentIncarnation").and_then(|v| v.as_str());
        let attendance = body.get("attendance").and_then(|v| v.as_str());
        let valid = document_id == Some("test-document") && document_incarnation == Some("test-document") && match operation {
            AttendanceOperation::Open => body.get("pin").and_then(|v| v.as_str()).is_some_and(|pin| !pin.is_empty()),
            AttendanceOperation::Validate | AttendanceOperation::Invalidate => attendance == Some("test-attendance"),
        };
        if !valid { return AttendanceCall::refused(operation, 401, "caduceus-attendance-refused"); }
        AttendanceCall { receipt: AttendanceReceipt { operation: operation.name(), ok: true, status: 200, code: "none".to_string() }, proof: operation.returns_proof().then(|| AttendanceProof("test-attendance".to_string())) }
    }
}
