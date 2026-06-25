async fn legacy_homeserver_proxy_route(
    method: Method,
    uri: Uri,
    headers: HeaderMap,
    body: Body,
) -> impl IntoResponse {
    match legacy_homeserver_proxy_response(method, uri, headers, body).await {
        Ok(response) => response,
        Err(error) => (
            StatusCode::BAD_GATEWAY,
            Json(serde_json::json!({
                "schema": "coronatio.legacy-homeserver-proxy.error.v1",
                "ok": false,
                "error": error,
                "authority": "Coronatio Rust host preserves the Flask/React HomeServer UX by proxying legacy assets and API requests"
            })),
        )
            .into_response(),
    }
}

async fn legacy_homeserver_proxy_response(
    method: Method,
    uri: Uri,
    headers: HeaderMap,
    body: Body,
) -> Result<Response, String> {
    let path_and_query = uri
        .path_and_query()
        .map(|value| value.as_str().to_string())
        .unwrap_or_else(|| uri.path().to_string());
    let body_bytes = to_bytes(body, 16 * 1024 * 1024)
        .await
        .map_err(|error| format!("read request body: {error}"))?;
    let accept = header_value(&headers, header::ACCEPT.as_str());
    let content_type = header_value(&headers, header::CONTENT_TYPE.as_str());
    let authorization = header_value(&headers, header::AUTHORIZATION.as_str());
    let upstream = tokio::task::spawn_blocking(move || {
        legacy_homeserver_http_request(
            method.as_str(),
            &path_and_query,
            &body_bytes,
            accept.as_deref(),
            content_type.as_deref(),
            authorization.as_deref(),
        )
    })
    .await
    .map_err(|error| format!("legacy proxy task: {error}"))??;
    let mut builder = Response::builder().status(upstream.status);
    for (name, value) in upstream.headers {
        builder = builder.header(name, value);
    }
    builder
        .body(Body::from(upstream.body))
        .map_err(|error| format!("build response: {error}"))
}

struct LegacyHttpResponse {
    status: StatusCode,
    headers: Vec<(String, String)>,
    body: Vec<u8>,
}

fn header_value(headers: &HeaderMap, name: &str) -> Option<String> {
    headers
        .get(name)
        .and_then(|value| value.to_str().ok())
        .map(ToString::to_string)
}

trait ReadWrite: Read + Write {}
impl<T: Read + Write> ReadWrite for T {}

fn legacy_homeserver_asset_root() -> PathBuf {
    env::var("CORONATIO_LEGACY_HOMESERVER_ASSET_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(LEGACY_HOMESERVER_ASSET_ROOT))
}

fn legacy_homeserver_build_root() -> PathBuf {
    env::var("CORONATIO_LEGACY_HOMESERVER_BUILD_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(LEGACY_HOMESERVER_BUILD_ROOT))
}

fn legacy_homeserver_build_file(name: &str) -> ServeFile {
    ServeFile::new(legacy_homeserver_build_root().join(name))
}

fn legacy_homeserver_proxy_socket() -> PathBuf {
    env::var("CORONATIO_LEGACY_HOMESERVER_SOCKET")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/mnt/ramdisk/homeserver.sock"))
}

fn legacy_homeserver_proxy_host() -> String {
    env::var("CORONATIO_LEGACY_HOMESERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string())
}

fn legacy_homeserver_proxy_port() -> u16 {
    env::var("CORONATIO_LEGACY_HOMESERVER_PORT")
        .ok()
        .and_then(|value| value.parse::<u16>().ok())
        .unwrap_or(8001)
}

fn legacy_homeserver_http_request(
    method: &str,
    path_and_query: &str,
    body: &[u8],
    accept: Option<&str>,
    content_type: Option<&str>,
    authorization: Option<&str>,
) -> Result<LegacyHttpResponse, String> {
    let socket = legacy_homeserver_proxy_socket();
    let mut stream: Box<dyn ReadWrite> = if socket.exists() {
        let stream = UnixStream::connect(&socket).map_err(|error| {
            format!(
                "connect legacy HomeServer socket {}: {error}",
                socket.display()
            )
        })?;
        stream
            .set_read_timeout(Some(Duration::from_secs(20)))
            .map_err(|error| format!("set socket read timeout: {error}"))?;
        stream
            .set_write_timeout(Some(Duration::from_secs(20)))
            .map_err(|error| format!("set socket write timeout: {error}"))?;
        Box::new(stream)
    } else {
        let stream = TcpStream::connect((
            legacy_homeserver_proxy_host().as_str(),
            legacy_homeserver_proxy_port(),
        ))
        .map_err(|error| format!("connect legacy HomeServer: {error}"))?;
        stream
            .set_read_timeout(Some(Duration::from_secs(20)))
            .map_err(|error| format!("set tcp read timeout: {error}"))?;
        stream
            .set_write_timeout(Some(Duration::from_secs(20)))
            .map_err(|error| format!("set tcp write timeout: {error}"))?;
        Box::new(stream)
    };
    let mut request = format!(
        "{method} {path_and_query} HTTP/1.1\r\nHost: home.arpa\r\nConnection: close\r\nX-Forwarded-Proto: https\r\nOrigin: https://home.arpa\r\nReferer: https://home.arpa/\r\nContent-Length: {}\r\n",
        body.len()
    );
    if let Some(value) = accept {
        request.push_str(&format!("Accept: {value}\r\n"));
    }
    if let Some(value) = content_type {
        request.push_str(&format!("Content-Type: {value}\r\n"));
    }
    if let Some(value) = authorization {
        request.push_str(&format!("Authorization: {value}\r\n"));
    }
    request.push_str("\r\n");
    stream
        .write_all(request.as_bytes())
        .map_err(|error| format!("write legacy request headers: {error}"))?;
    if !body.is_empty() {
        stream
            .write_all(body)
            .map_err(|error| format!("write legacy request body: {error}"))?;
    }
    let mut raw = Vec::new();
    stream
        .read_to_end(&mut raw)
        .map_err(|error| format!("read legacy response: {error}"))?;
    let split = raw
        .windows(4)
        .position(|window| window == b"\r\n\r\n")
        .ok_or_else(|| "legacy response missing header boundary".to_string())?;
    let (head, body_part) = raw.split_at(split + 4);
    let head_text = String::from_utf8_lossy(head);
    let mut lines = head_text.split("\r\n");
    let status_line = lines
        .next()
        .ok_or_else(|| "legacy response missing status line".to_string())?;
    let status_code = status_line
        .split_whitespace()
        .nth(1)
        .ok_or_else(|| format!("legacy response malformed status line: {status_line}"))?
        .parse::<u16>()
        .map_err(|error| format!("legacy response status parse: {error}"))?;
    let status = StatusCode::from_u16(status_code)
        .map_err(|error| format!("legacy response status conversion: {error}"))?;
    let mut response_headers = Vec::new();
    let mut chunked = false;
    for line in lines {
        if line.is_empty() {
            continue;
        }
        if let Some((name, value)) = line.split_once(':') {
            let normalized = name.to_ascii_lowercase();
            let trimmed = value.trim().to_string();
            if normalized == "transfer-encoding" && trimmed.to_ascii_lowercase().contains("chunked")
            {
                chunked = true;
                continue;
            }
            if matches!(
                normalized.as_str(),
                "content-type" | "cache-control" | "etag" | "last-modified"
            ) {
                response_headers.push((normalized, trimmed));
            }
        }
    }
    let body = if chunked {
        decode_chunked_body(body_part)?
    } else {
        body_part.to_vec()
    };
    Ok(LegacyHttpResponse {
        status,
        headers: response_headers,
        body,
    })
}

fn decode_chunked_body(input: &[u8]) -> Result<Vec<u8>, String> {
    let mut cursor = 0usize;
    let mut decoded = Vec::new();
    loop {
        let line_end = input[cursor..]
            .windows(2)
            .position(|window| window == b"\r\n")
            .ok_or_else(|| "chunked response missing chunk-size delimiter".to_string())?
            + cursor;
        let size_text = String::from_utf8_lossy(&input[cursor..line_end]);
        let size_token = size_text.split(';').next().unwrap_or("").trim();
        let size = usize::from_str_radix(size_token, 16)
            .map_err(|error| format!("chunked response size parse: {error}"))?;
        cursor = line_end + 2;
        if size == 0 {
            break;
        }
        if cursor + size > input.len() {
            return Err("chunked response body shorter than declared chunk".to_string());
        }
        decoded.extend_from_slice(&input[cursor..cursor + size]);
        cursor += size;
        if input.get(cursor..cursor + 2) != Some(b"\r\n") {
            return Err("chunked response missing trailing CRLF".to_string());
        }
        cursor += 2;
    }
    Ok(decoded)
}

