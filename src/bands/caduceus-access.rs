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
    pub(crate) fn attendance_open(&self, pin: &str, document: &str) -> AttendanceCall { self.call(AttendanceOperation::Open, serde_json::json!({"pin":pin,"document":document})) }
    pub(crate) fn attendance_validate(&self, attendance: &AttendanceProof, document: &str) -> AttendanceCall { self.call(AttendanceOperation::Validate, serde_json::json!({"attendance":attendance.expose(),"document":document})) }
    pub(crate) fn attendance_invalidate(&self, attendance: &AttendanceProof, document: &str) -> AttendanceCall { self.call(AttendanceOperation::Invalidate, serde_json::json!({"attendance":attendance.expose(),"document":document})) }
    fn call(&self, operation: AttendanceOperation, body: serde_json::Value) -> AttendanceCall {
        #[cfg(test)]
        if self.base == test_fixture::base() { return test_fixture::attendance_call(operation, &body); }
        let encoded = match serde_json::to_vec(&body) { Ok(v) if v.len() <= 4096 => v, _ => return AttendanceCall::refused(operation, 0, "caduceus-attendance-request-invalid") };
        let Some(authority) = access_authority(&self.base) else { return AttendanceCall::refused(operation, 0, "caduceus-attendance-base-invalid") };
        let mut stream = match TcpStream::connect_timeout(&authority, self.timeout) { Ok(s) => s, Err(_) => return AttendanceCall::refused(operation, 0, "caduceus-attendance-unavailable") };
        if stream.set_read_timeout(Some(self.timeout)).is_err() || stream.set_write_timeout(Some(self.timeout)).is_err() { return AttendanceCall::refused(operation, 0, "caduceus-attendance-unavailable"); }
        let request = format!("POST {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\nContent-Type: application/json\r\nAccept: application/json\r\nContent-Length: {}\r\n\r\n", operation.path(), access_host_header(&self.base), encoded.len());
        if stream.write_all(request.as_bytes()).and_then(|_| stream.write_all(&encoded)).is_err() { return AttendanceCall::refused(operation, 0, "caduceus-attendance-unavailable"); }
        parse_attendance_response(operation, &mut stream)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AttendanceOperation { Open, Validate, Invalidate }
impl AttendanceOperation { fn name(self)->&'static str { match self { Self::Open=>"attendance.open",Self::Validate=>"attendance.validate",Self::Invalidate=>"attendance.invalidate" } } fn path(self)->&'static str { match self { Self::Open=>"/api/v1/attendance/open",Self::Validate=>"/api/v1/attendance/validate",Self::Invalidate=>"/api/v1/attendance/invalidate" } } fn returns_proof(self)->bool { matches!(self,Self::Open|Self::Validate) } }
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
fn parse_attendance_response(op:AttendanceOperation,stream:&mut TcpStream)->AttendanceCall { let mut reader=BufReader::new(stream); let mut status_line=String::new(); if reader.read_line(&mut status_line).ok().filter(|n|*n>0).is_none(){return AttendanceCall::refused(op,0,"caduceus-attendance-malformed-response");} let status=status_line.split_whitespace().nth(1).and_then(|v|v.parse().ok()).unwrap_or(0); let mut length=None; let mut header_bytes=0; loop { let mut line=String::new(); match reader.read_line(&mut line){Ok(0)|Err(_)=>return AttendanceCall::refused(op,status,"caduceus-attendance-malformed-response"),Ok(_)=>{}} header_bytes+=line.len(); if header_bytes>4096{return AttendanceCall::refused(op,status,"caduceus-attendance-oversized-response")} if line=="\r\n"{break} if let Some((n,v))=line.split_once(':'){if n.eq_ignore_ascii_case("content-length"){length=v.trim().parse().ok();}} } let Some(length)=length else{return AttendanceCall::refused(op,status,"caduceus-attendance-malformed-response")}; if length>CADUCEUS_ACCESS_MAX_RESPONSE{return AttendanceCall::refused(op,status,"caduceus-attendance-oversized-response")}; let mut body=vec![0;length]; if reader.read_exact(&mut body).is_err(){return AttendanceCall::refused(op,status,"caduceus-attendance-malformed-response")} let Ok(value)=serde_json::from_slice::<serde_json::Value>(&body) else{return AttendanceCall::refused(op,status,"caduceus-attendance-malformed-response")}; let Some(obj)=value.as_object() else{return AttendanceCall::refused(op,status,"caduceus-attendance-malformed-response")}; let ok=obj.get("ok").and_then(|v|v.as_bool()).unwrap_or(false); let code=obj.get("code").or_else(||obj.get("firstMissingSignal")).and_then(|v|v.as_str()).unwrap_or(if ok{"none"}else{"caduceus-attendance-refused"}); if status<200||status>=300||!ok{return AttendanceCall::refused(op,status,code)} let proof=if op.returns_proof(){obj.get("attendance").or_else(||obj.get("proof")).and_then(|v|v.as_str()).and_then(AttendanceProof::parse)}else{None}; if op.returns_proof()&&proof.is_none(){return AttendanceCall::refused(op,status,"caduceus-attendance-malformed-response")} AttendanceCall{receipt:AttendanceReceipt{operation:op.name(),ok:true,status,code:"none".to_string()},proof} }
fn normalize_access_base(base:String)->String{base.trim().trim_end_matches('/').to_string()}
fn access_authority(base:&str)->Option<SocketAddr>{let raw=base.strip_prefix("http://")?;if raw.contains('/') {return None} let address:SocketAddr=raw.parse().ok()?;address.ip().is_loopback().then_some(address)}
fn access_host_header(base:&str)->&str{base.strip_prefix("http://").unwrap_or("127.0.0.1:3014")}
pub(crate) fn document_incarnation_from_headers(headers:&axum::http::HeaderMap)->Option<String>{headers.get("x-caduceus-document").and_then(|v|v.to_str().ok()).map(str::trim).filter(|v|!v.is_empty()&&v.len()<=128&&v.bytes().all(|b|b.is_ascii_alphanumeric()||matches!(b,b'-'|b'_'|b'.'))).map(ToOwned::to_owned)}
pub(crate) fn attendance_from_headers(headers:&axum::http::HeaderMap)->Option<AttendanceProof>{headers.get("x-caduceus-attendance").and_then(|v|v.to_str().ok()).and_then(AttendanceProof::parse)}
pub(crate) fn same_origin_state_change(headers:&axum::http::HeaderMap)->bool{let Some(host)=forwarded_first(headers,"x-forwarded-host").or_else(||headers.get(header::HOST).and_then(|v|v.to_str().ok()))else{return false};let proto=forwarded_first(headers,"x-forwarded-proto").unwrap_or("https");let Some(origin)=headers.get(header::ORIGIN).and_then(|v|v.to_str().ok())else{return false};proto.eq_ignore_ascii_case("https")&&matches!(host.trim().to_ascii_lowercase().as_str(),"home.arpa"|"home.arpa:443")&&matches!(origin.trim().to_ascii_lowercase().as_str(),"https://home.arpa"|"https://home.arpa:443")}
fn forwarded_first<'a>(headers:&'a axum::http::HeaderMap,name:&str)->Option<&'a str>{headers.get(name)?.to_str().ok()?.split(',').next().map(str::trim).filter(|v|!v.is_empty())}
#[cfg(test)]
pub(crate) mod test_fixture {
    use super::*;
    pub(crate) fn base() -> String { "http://127.0.0.1:9".to_string() }
    pub(super) fn attendance_call(operation: AttendanceOperation, body: &serde_json::Value) -> AttendanceCall {
        let document = body.get("document").and_then(|v| v.as_str());
        let attendance = body.get("attendance").and_then(|v| v.as_str());
        let valid = document == Some("test-document") && match operation {
            AttendanceOperation::Open => body.get("pin").and_then(|v| v.as_str()).is_some_and(|pin| !pin.is_empty()),
            AttendanceOperation::Validate | AttendanceOperation::Invalidate => attendance == Some("test-attendance"),
        };
        if !valid { return AttendanceCall::refused(operation, 401, "caduceus-attendance-refused"); }
        AttendanceCall { receipt: AttendanceReceipt { operation: operation.name(), ok: true, status: 200, code: "none".to_string() }, proof: operation.returns_proof().then(|| AttendanceProof("test-attendance".to_string())) }
    }
}
