// Caduceus successor access seam for mounted Crown session and mutation routes.
// It owns only private-loopback mint, prove, clear, and capability transport.

const CADUCEUS_ACCESS_DEFAULT_BASE: &str = "http://127.0.0.1:3014";
const CADUCEUS_ACCESS_TIMEOUT: Duration = Duration::from_secs(3);
const CADUCEUS_ACCESS_MAX_RESPONSE: usize = 16 * 1024;
const CADUCEUS_SESSION_COOKIE: &str = "caduceus_session";

#[derive(Clone)]
pub(crate) struct CaduceusAccessClient {
    base: String,
    timeout: Duration,
}

impl Default for CaduceusAccessClient {
    fn default() -> Self {
        #[cfg(test)]
        {
            return Self::new(test_fixture::base());
        }
        #[cfg(not(test))]
        Self::new(
            env::var("CADUCEUS_ACCESS_BASE_URL")
                .or_else(|_| env::var("CADUCEUS_BASE_URL"))
                .unwrap_or_else(|_| CADUCEUS_ACCESS_DEFAULT_BASE.to_string()),
        )
    }
}

impl CaduceusAccessClient {
    pub(crate) fn new(base: impl Into<String>) -> Self {
        Self {
            base: normalize_access_base(base.into()),
            timeout: CADUCEUS_ACCESS_TIMEOUT,
        }
    }

    #[cfg(test)]
    fn with_timeout(base: impl Into<String>, timeout: Duration) -> Self {
        Self { base: normalize_access_base(base.into()), timeout }
    }

    #[cfg(test)]
    pub(crate) fn test_base(&self) -> &str { &self.base }

    pub(crate) fn session_mint(&self, pin: &str) -> AccessCall {
        self.call(AccessOperation::SessionMint, serde_json::json!({"pin": pin}))
    }

    pub(crate) fn session_prove(&self, ticket: &SessionTicket) -> AccessCall {
        self.call(AccessOperation::SessionProve, serde_json::json!({"ticket": ticket.expose_for_transport()}))
    }

    pub(crate) fn session_clear(&self, ticket: &SessionTicket) -> AccessCall {
        self.call(AccessOperation::SessionClear, serde_json::json!({"ticket": ticket.expose_for_transport()}))
    }

    pub(crate) fn capability_mint(&self, ticket: &SessionTicket, action: &str, target: &str) -> AccessCall {
        self.call(AccessOperation::CapabilityMint, serde_json::json!({
            "ticket": ticket.expose_for_transport(), "action": action, "target": target,
        }))
    }

    fn call(&self, operation: AccessOperation, body: serde_json::Value) -> AccessCall {
        let path = operation.path();
        let encoded = match serde_json::to_vec(&body) {
            Ok(value) if value.len() <= 4096 => value,
            _ => return AccessCall::refused(operation, 0, "caduceus-access-request-invalid"),
        };
        let authority = match access_authority(&self.base) {
            Some(authority) => authority,
            None => return AccessCall::refused(operation, 0, "caduceus-access-base-invalid"),
        };
        let mut stream = match TcpStream::connect_timeout(&authority, self.timeout) {
            Ok(stream) => stream,
            Err(_) => return AccessCall::refused(operation, 0, "caduceus-access-unavailable"),
        };
        if stream.set_read_timeout(Some(self.timeout)).is_err() || stream.set_write_timeout(Some(self.timeout)).is_err() {
            return AccessCall::refused(operation, 0, "caduceus-access-unavailable");
        }
        let request = format!(
            "POST {path} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\nContent-Type: application/json\r\nAccept: application/json\r\nContent-Length: {}\r\n\r\n",
            access_host_header(&self.base), encoded.len(),
        );
        if stream.write_all(request.as_bytes()).and_then(|_| stream.write_all(&encoded)).is_err() {
            return AccessCall::refused(operation, 0, "caduceus-access-unavailable");
        }
        parse_access_http_response(operation, &mut stream)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AccessOperation {
    SessionMint,
    SessionProve,
    SessionClear,
    CapabilityMint,
}

impl AccessOperation {
    fn name(self) -> &'static str {
        match self {
            Self::SessionMint => "session.mint",
            Self::SessionProve => "session.prove",
            Self::SessionClear => "session.clear",
            Self::CapabilityMint => "capability.mint",
        }
    }

    fn path(self) -> &'static str {
        match self {
            Self::SessionMint => "/api/v1/access/sessions/mint",
            Self::SessionProve => "/api/v1/access/sessions/prove",
            Self::SessionClear => "/api/v1/access/sessions/clear",
            Self::CapabilityMint => "/api/v1/access/capabilities/mint",
        }
    }

    fn returns_ticket(self) -> bool { matches!(self, Self::SessionMint) }
    fn returns_capability(self) -> bool { matches!(self, Self::CapabilityMint) }
}

#[derive(Clone, PartialEq, Eq)]
pub(crate) struct SessionTicket(pub(crate) String);

impl std::fmt::Debug for SessionTicket {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { formatter.write_str("SessionTicket([redacted])") }
}

impl SessionTicket {
    pub(crate) fn parse(raw: &str) -> Option<Self> { opaque_secret(raw).then(|| Self(raw.to_string())) }
    fn expose_for_transport(&self) -> &str { &self.0 }
}

#[derive(Clone, PartialEq, Eq)]
pub(crate) struct CapabilityTicket(pub(crate) String);

impl std::fmt::Debug for CapabilityTicket {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { formatter.write_str("CapabilityTicket([redacted])") }
}

impl CapabilityTicket {
    pub(crate) fn parse(raw: &str) -> Option<Self> { opaque_secret(raw).then(|| Self(raw.to_string())) }
}

fn opaque_secret(raw: &str) -> bool {
    !raw.is_empty() && raw.len() <= 1024 && raw.bytes().all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct AccessReceipt {
    operation: &'static str,
    pub(crate) ok: bool,
    pub(crate) status: u16,
    pub(crate) code: String,
}

#[derive(Debug, Clone)]
pub(crate) struct AccessCall {
    pub(crate) receipt: AccessReceipt,
    ticket: Option<SessionTicket>,
    pub(crate) capability: Option<CapabilityTicket>,
}

impl AccessCall {
    pub(crate) fn take_ticket(self) -> Option<SessionTicket> { self.ticket }

    fn refused(operation: AccessOperation, status: u16, code: &str) -> Self {
        Self { receipt: AccessReceipt { operation: operation.name(), ok: false, status, code: safe_access_code(code) }, ticket: None, capability: None }
    }
}

pub(crate) fn safe_access_code(value: &str) -> String {
    let safe = value.len() <= 100 && value.bytes().all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-');
    if safe { value.to_string() } else { "caduceus-access-refused".to_string() }
}

fn parse_access_http_response(operation: AccessOperation, stream: &mut TcpStream) -> AccessCall {
    let mut reader = BufReader::new(stream);
    let mut status_line = String::new();
    if reader.read_line(&mut status_line).ok().filter(|read| *read > 0).is_none() {
        return AccessCall::refused(operation, 0, "caduceus-access-malformed-response");
    }
    let status = status_line.split_whitespace().nth(1).and_then(|value| value.parse::<u16>().ok()).unwrap_or(0);
    let mut content_length = None;
    let mut headers = 0usize;
    loop {
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(0) | Err(_) => return AccessCall::refused(operation, status, "caduceus-access-malformed-response"),
            Ok(_) => {}
        }
        headers += line.len();
        if headers > 4096 { return AccessCall::refused(operation, status, "caduceus-access-oversized-response"); }
        if line == "\r\n" { break; }
        if let Some((name, value)) = line.split_once(':') {
            if name.eq_ignore_ascii_case("content-length") {
                content_length = value.trim().parse::<usize>().ok();
            }
        }
    }
    let Some(length) = content_length else { return AccessCall::refused(operation, status, "caduceus-access-malformed-response"); };
    if length > CADUCEUS_ACCESS_MAX_RESPONSE { return AccessCall::refused(operation, status, "caduceus-access-oversized-response"); }
    let mut body = vec![0u8; length];
    if reader.read_exact(&mut body).is_err() { return AccessCall::refused(operation, status, "caduceus-access-malformed-response"); }
    parse_access_body(operation, status, &body)
}

fn parse_access_body(operation: AccessOperation, status: u16, body: &[u8]) -> AccessCall {
    let Ok(value) = serde_json::from_slice::<serde_json::Value>(body) else { return AccessCall::refused(operation, status, "caduceus-access-malformed-response"); };
    let Some(object) = value.as_object() else { return AccessCall::refused(operation, status, "caduceus-access-malformed-response"); };
    let ok = object.get("ok").and_then(serde_json::Value::as_bool).unwrap_or(false);
    let code = object.get("code").or_else(|| object.get("firstMissingSignal")).and_then(serde_json::Value::as_str).unwrap_or(if ok { "none" } else { "caduceus-access-refused" });
    if status < 200 || status >= 300 || !ok {
        return AccessCall::refused(operation, status, code);
    }
    let ticket = if operation.returns_ticket() {
        match object.get("ticket").and_then(serde_json::Value::as_str).and_then(SessionTicket::parse) {
            Some(ticket) => Some(ticket),
            None => return AccessCall::refused(operation, status, "caduceus-access-malformed-response"),
        }
    } else { None };
    let capability = if operation.returns_capability() {
        match object.get("capability").and_then(serde_json::Value::as_str).and_then(CapabilityTicket::parse) {
            Some(capability) => Some(capability),
            None => return AccessCall::refused(operation, status, "caduceus-access-malformed-response"),
        }
    } else { None };
    AccessCall { receipt: AccessReceipt { operation: operation.name(), ok: true, status, code: "none".to_string() }, ticket, capability }
}

fn normalize_access_base(base: String) -> String { base.trim().trim_end_matches('/').to_string() }

fn access_authority(base: &str) -> Option<SocketAddr> {
    let raw = base.strip_prefix("http://")?;
    if raw.contains('/') { return None; }
    let address: SocketAddr = raw.parse().ok()?;
    address.ip().is_loopback().then_some(address)
}

fn access_host_header(base: &str) -> &str { base.strip_prefix("http://").unwrap_or("127.0.0.1:3014") }

pub(crate) fn session_cookie(ticket: &SessionTicket) -> String {
    format!("{CADUCEUS_SESSION_COOKIE}={}; HttpOnly; Secure; SameSite=Strict; Path=/; Max-Age=1800", ticket.expose_for_transport())
}

pub(crate) fn clear_session_cookie() -> String {
    format!("{CADUCEUS_SESSION_COOKIE}=; HttpOnly; Secure; SameSite=Strict; Path=/; Max-Age=0")
}

pub(crate) fn session_ticket_from_cookie(headers: &axum::http::HeaderMap) -> Option<SessionTicket> {
    let raw = headers.get(header::COOKIE)?.to_str().ok()?;
    raw.split(';').map(str::trim).find_map(|pair| {
        let (name, value) = pair.split_once('=')?;
        (name == CADUCEUS_SESSION_COOKIE).then(|| SessionTicket::parse(value)).flatten()
    })
}

pub(crate) fn same_origin_state_change(headers: &axum::http::HeaderMap) -> bool {
    let Some(host) = forwarded_first(headers, "x-forwarded-host")
        .or_else(|| headers.get(header::HOST).and_then(|value| value.to_str().ok()))
    else {
        return false;
    };
    let proto = forwarded_first(headers, "x-forwarded-proto").unwrap_or("https");
    let Some(origin) = headers
        .get(header::ORIGIN)
        .and_then(|value| value.to_str().ok())
    else {
        return false;
    };
    proto.eq_ignore_ascii_case("https") && trusted_home_host(host) && trusted_home_origin(origin)
}

fn forwarded_first<'a>(headers: &'a axum::http::HeaderMap, name: &str) -> Option<&'a str> {
    headers.get(name)?.to_str().ok()?.split(',').next().map(str::trim).filter(|value| !value.is_empty())
}

fn trusted_home_host(value: &str) -> bool { matches!(value.trim().to_ascii_lowercase().as_str(), "home.arpa" | "home.arpa:443") }
fn trusted_home_origin(value: &str) -> bool { matches!(value.trim().to_ascii_lowercase().as_str(), "https://home.arpa" | "https://home.arpa:443") }

#[cfg(test)]
pub(crate) mod test_fixture {
    use super::*;

    const TEST_TICKET: &str = "caduceus-test-session-ticket";
    const TEST_CAPABILITY: &str = "caduceus-test-capability";

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub(crate) struct RequestRecord {
        pub(crate) path: String,
        pub(crate) action: Option<String>,
        pub(crate) target: Option<String>,
        pub(crate) capability_present: bool,
    }

    struct Fixture {
        base: String,
        records: Arc<Mutex<Vec<RequestRecord>>>,
    }

    static FIXTURE: OnceLock<Fixture> = OnceLock::new();

    fn fixture() -> &'static Fixture {
        FIXTURE.get_or_init(|| {
            let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind test Caduceus loopback");
            let base = format!("http://{}", listener.local_addr().expect("test Caduceus address"));
            let records = Arc::new(Mutex::new(Vec::new()));
            let witness = Arc::clone(&records);
            std::thread::Builder::new()
                .name("coronatio-test-caduceus".to_string())
                .spawn(move || {
                    for stream in listener.incoming().flatten() {
                        serve(stream, &witness);
                    }
                })
                .expect("start test Caduceus loopback");
            Fixture { base, records }
        })
    }

    pub(crate) fn base() -> String { fixture().base.clone() }

    pub(crate) fn mark() -> usize { fixture().records.lock().expect("fixture records").len() }

    pub(crate) fn records_since(mark: usize) -> Vec<RequestRecord> {
        fixture().records.lock().expect("fixture records").iter().skip(mark).cloned().collect()
    }

    pub(crate) fn opaque_ticket() -> &'static str { TEST_TICKET }

    pub(crate) fn same_origin_headers(with_cookie: bool) -> axum::http::HeaderMap {
        let mut headers = axum::http::HeaderMap::new();
        headers.insert(header::HOST, HeaderValue::from_static("home.arpa"));
        headers.insert("x-forwarded-proto", HeaderValue::from_static("https"));
        headers.insert(header::ORIGIN, HeaderValue::from_static("https://home.arpa"));
        if with_cookie {
            headers.insert(header::COOKIE, HeaderValue::from_static("caduceus_session=caduceus-test-session-ticket"));
        }
        headers
    }

    fn serve(mut stream: TcpStream, records: &Arc<Mutex<Vec<RequestRecord>>>) {
        let Ok(copy) = stream.try_clone() else { return; };
        let mut reader = BufReader::new(copy);
        let mut request_line = String::new();
        if reader.read_line(&mut request_line).ok().filter(|count| *count > 0).is_none() { return; }
        let path = request_line.split_whitespace().nth(1).unwrap_or("/").to_string();
        let mut content_length = 0usize;
        let mut capability_present = false;
        let mut header_bytes = 0usize;
        loop {
            let mut line = String::new();
            let Ok(count) = reader.read_line(&mut line) else { return; };
            if count == 0 { return; }
            header_bytes += count;
            if header_bytes > 4096 { return; }
            if line == "\r\n" { break; }
            if let Some((name, value)) = line.split_once(':') {
                if name.eq_ignore_ascii_case("content-length") {
                    content_length = value.trim().parse::<usize>().unwrap_or(0).min(4096);
                }
                if name.eq_ignore_ascii_case("x-caduceus-capability") && !value.trim().is_empty() {
                    capability_present = true;
                }
            }
        }
        let mut body = vec![0u8; content_length];
        if reader.read_exact(&mut body).is_err() { return; }
        let json = serde_json::from_slice::<serde_json::Value>(&body).unwrap_or_default();
        let action = json.get("action").and_then(serde_json::Value::as_str).map(str::to_string);
        let target = json.get("target").and_then(serde_json::Value::as_str).map(str::to_string);
        let ticket = json.get("ticket").and_then(serde_json::Value::as_str);
        records.lock().expect("fixture records").push(RequestRecord { path: path.clone(), action, target, capability_present });
        let response = match path.as_str() {
            "/api/v1/access/sessions/mint" => serde_json::json!({"ok":true,"code":"none","ticket":TEST_TICKET}),
            "/api/v1/access/sessions/prove" if ticket == Some(TEST_TICKET) => serde_json::json!({"ok":true,"code":"none"}),
            "/api/v1/access/sessions/prove" => serde_json::json!({"ok":false,"code":"caduceus-access-refused"}),
            "/api/v1/access/sessions/clear" if ticket == Some("caduceus-test-clear-refused-ticket") => serde_json::json!({"ok":false,"code":"caduceus-access-refused"}),
            "/api/v1/access/sessions/clear" => serde_json::json!({"ok":true,"code":"none"}),
            "/api/v1/access/capabilities/mint" => serde_json::json!({"ok":true,"code":"none","capability":TEST_CAPABILITY}),
            _ if capability_present => serde_json::json!({"ok":true,"firstMissingSignal":"none"}),
            _ => serde_json::json!({"ok":false,"firstMissingSignal":"caduceus-test-capability-required"}),
        };
        let body = response.to_string();
        let wire = format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}", body.len(), body);
        let _ = stream.write_all(wire.as_bytes());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    include!("tests/caduceus-access-walls.rs");
}
