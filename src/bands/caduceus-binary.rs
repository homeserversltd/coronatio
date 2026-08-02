const HESTIA_BUNDLE_HEADER_LIMIT: usize = 32 * 1024;
const HESTIA_BUNDLE_BODY_LIMIT: usize = 1024 * 1024;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum HestiaPlatform {
    Windows,
    Android,
    ChromeOs,
    Linux,
    MacOs,
}

impl HestiaPlatform {
    fn parse(value: Option<&str>) -> Option<Self> {
        match value.unwrap_or("linux") {
            "windows" => Some(Self::Windows),
            "android" => Some(Self::Android),
            "chromeos" => Some(Self::ChromeOs),
            "linux" => Some(Self::Linux),
            "macos" => Some(Self::MacOs),
            _ => None,
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::Windows => "windows",
            Self::Android => "android",
            Self::ChromeOs => "chromeos",
            Self::Linux => "linux",
            Self::MacOs => "macos",
        }
    }

    fn filename(self) -> String {
        let suffix = if self == Self::Windows {
            ".cer"
        } else {
            ".crt"
        };
        format!("homeserver-house-ca-{}{suffix}", self.as_str())
    }
}

#[derive(Debug, Deserialize)]
struct HestiaBundleQuery {
    platform: Option<String>,
}

#[derive(Debug)]
struct HestiaUpstreamResponse {
    status: u16,
    headers: Vec<(&'static str, HeaderValue)>,
    body: Vec<u8>,
}

fn hestia_public_failure(status: StatusCode, signal: &'static str) -> Response {
    (
        status,
        Json(serde_json::json!({
            "schema": "coronatio.hestia.bundle.failure.v1",
            "ok": false,
            "message": "The household certificate bundle is unavailable.",
            "firstMissingSignal": signal,
        })),
    )
        .into_response()
}

fn hestia_parse_head(
    head: &[u8],
) -> Result<(u16, Vec<(&'static str, HeaderValue)>, Option<usize>), &'static str> {
    let head = std::str::from_utf8(head).map_err(|_| "caduceus-response-invalid")?;
    let mut lines = head.split("\r\n");
    let status = lines
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .and_then(|value| value.parse::<u16>().ok())
        .ok_or("caduceus-response-invalid")?;
    let mut forwarded = Vec::new();
    let mut content_length = None;
    for line in lines {
        let (name, value) = line.split_once(':').ok_or("caduceus-response-invalid")?;
        let value = value.trim();
        match name.to_ascii_lowercase().as_str() {
            "content-type" => forwarded.push((
                "content-type",
                HeaderValue::from_str(value).map_err(|_| "caduceus-response-invalid")?,
            )),
            "content-disposition" => forwarded.push((
                "content-disposition",
                HeaderValue::from_str(value).map_err(|_| "caduceus-response-invalid")?,
            )),
            "content-length" => {
                if content_length.is_some() {
                    return Err("caduceus-response-invalid");
                }
                let length = value
                    .parse::<usize>()
                    .map_err(|_| "caduceus-response-invalid")?;
                content_length = Some(length);
                forwarded.push((
                    "content-length",
                    HeaderValue::from_str(value).map_err(|_| "caduceus-response-invalid")?,
                ));
            }
            "transfer-encoding" => return Err("caduceus-response-framing-unsupported"),
            _ => {}
        }
    }
    Ok((status, forwarded, content_length))
}

fn hestia_read_upstream(stream: &mut TcpStream) -> Result<HestiaUpstreamResponse, &'static str> {
    let mut raw = Vec::new();
    let mut buffer = [0_u8; 8192];
    let head_end = loop {
        let count = stream
            .read(&mut buffer)
            .map_err(|_| "caduceus-read-failed")?;
        if count == 0 {
            return Err("caduceus-response-invalid");
        }
        raw.extend_from_slice(&buffer[..count]);
        if let Some(index) = raw.windows(4).position(|window| window == b"\r\n\r\n") {
            break index;
        }
        if raw.len() > HESTIA_BUNDLE_HEADER_LIMIT {
            return Err("caduceus-response-headers-too-large");
        }
    };
    if head_end > HESTIA_BUNDLE_HEADER_LIMIT {
        return Err("caduceus-response-headers-too-large");
    }
    let (status, headers, content_length) = hestia_parse_head(&raw[..head_end])?;
    if content_length.is_some_and(|length| length > HESTIA_BUNDLE_BODY_LIMIT) {
        return Err("caduceus-response-too-large");
    }
    let mut body = raw[(head_end + 4)..].to_vec();
    if body.len() > HESTIA_BUNDLE_BODY_LIMIT
        || content_length.is_some_and(|length| body.len() > length)
    {
        return Err("caduceus-response-too-large");
    }
    loop {
        if content_length.is_some_and(|length| body.len() == length) {
            break;
        }
        let count = stream
            .read(&mut buffer)
            .map_err(|_| "caduceus-read-failed")?;
        if count == 0 {
            break;
        }
        body.extend_from_slice(&buffer[..count]);
        if body.len() > HESTIA_BUNDLE_BODY_LIMIT
            || content_length.is_some_and(|length| body.len() > length)
        {
            return Err("caduceus-response-too-large");
        }
    }
    if content_length.is_some_and(|length| body.len() != length) {
        return Err("caduceus-response-invalid");
    }
    Ok(HestiaUpstreamResponse {
        status,
        headers,
        body,
    })
}

fn caduceus_hestia_bundle_get(
    platform: HestiaPlatform,
) -> Result<HestiaUpstreamResponse, &'static str> {
    let authority = caduceus_authority().ok_or("caduceus-loopback-required")?;
    let mut stream = TcpStream::connect(&authority).map_err(|_| "caduceus-unreachable")?;
    let _ = stream.set_read_timeout(Some(Duration::from_secs(4)));
    let _ = stream.set_write_timeout(Some(Duration::from_secs(4)));
    let path = format!(
        "/api/v1/cert/bundle/download?platform={}",
        platform.as_str()
    );
    let request = format!("GET {path} HTTP/1.1\r\nHost: {authority}\r\nConnection: close\r\nAccept: application/x-x509-ca-cert\r\n\r\n");
    stream
        .write_all(request.as_bytes())
        .map_err(|_| "caduceus-write-failed")?;
    hestia_read_upstream(&mut stream)
}

async fn hestia_bundle_download_route(Query(query): Query<HestiaBundleQuery>) -> Response {
    let Some(platform) = HestiaPlatform::parse(query.platform.as_deref()) else {
        return hestia_public_failure(StatusCode::BAD_REQUEST, "certificate-platform-invalid");
    };
    let upstream = match caduceus_hestia_bundle_get(platform) {
        Ok(upstream) => upstream,
        Err(signal) => {
            let status = if signal.starts_with("caduceus-response-") {
                StatusCode::BAD_GATEWAY
            } else {
                StatusCode::SERVICE_UNAVAILABLE
            };
            return hestia_public_failure(status, signal);
        }
    };
    let status = StatusCode::from_u16(upstream.status).unwrap_or(StatusCode::BAD_GATEWAY);
    if !status.is_success() {
        return hestia_public_failure(
            if status.is_client_error() || status.is_server_error() {
                status
            } else {
                StatusCode::BAD_GATEWAY
            },
            "caduceus-bundle-refused",
        );
    }
    let expected_disposition = format!("attachment; filename=\"{}\"", platform.filename());
    let content_types: Vec<_> = upstream
        .headers
        .iter()
        .filter(|(name, _)| *name == "content-type")
        .collect();
    let dispositions: Vec<_> = upstream
        .headers
        .iter()
        .filter(|(name, _)| *name == "content-disposition")
        .collect();
    let Some((_, content_type)) = content_types.first() else {
        return hestia_public_failure(StatusCode::BAD_GATEWAY, "caduceus-response-invalid");
    };
    let Some((_, disposition)) = dispositions.first() else {
        return hestia_public_failure(StatusCode::BAD_GATEWAY, "caduceus-response-invalid");
    };
    // Do not let duplicate allowlisted headers smuggle an untrusted value into
    // the browser response. The upstream contract is one exact CA type and one
    // exact, platform-bound attachment name.
    if status != StatusCode::OK
        || upstream.body.is_empty()
        || content_types.len() != 1
        || dispositions.len() != 1
        || *content_type != "application/x-x509-ca-cert"
        || *disposition != expected_disposition.as_str()
    {
        return hestia_public_failure(StatusCode::BAD_GATEWAY, "caduceus-response-invalid");
    }
    let mut response = (status, upstream.body).into_response();
    for (name, value) in upstream.headers {
        response.headers_mut().insert(name, value);
    }
    response
}
