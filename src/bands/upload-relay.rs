const UPLOAD_CHUNK_BODY_LIMIT: usize = 8_454_144;
const UPLOAD_RELAY_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(3);
const UPLOAD_PROJECTION_TTL: std::time::Duration = std::time::Duration::from_secs(30 * 60);
const UPLOAD_START_BODY_LIMIT: usize = 2_097_152;
const UPLOAD_COMPLETION_BODY_CAPTURE_LIMIT: usize = 2 * 1024;
const UPLOAD_METADATA_BODY_LIMIT: usize = 1024;
const UPLOAD_COMPLETION_INTENT_LIMIT: usize = 2 * 1024;
const UPLOAD_RESPONSE_BODY_CAPTURE_LIMIT: usize = 64 * 1024;

#[derive(Clone)]
struct UploadDigestProjection {
    filename: String,
    destination: String,
    total_bytes: u64,
    hashed_bytes: u64,
    next_index: u64,
    hasher: sha2::Sha256,
    expected_sha256: Option<String>,
    expires_at: std::time::Instant,
}

fn upload_digest_projections() -> &'static Mutex<std::collections::HashMap<String, UploadDigestProjection>> {
    static PROJECTIONS: OnceLock<Mutex<std::collections::HashMap<String, UploadDigestProjection>>> = OnceLock::new();
    PROJECTIONS.get_or_init(|| Mutex::new(std::collections::HashMap::new()))
}

fn purge_upload_digest_projections() {
    let now = std::time::Instant::now();
    if let Ok(mut projections) = upload_digest_projections().lock() {
        projections.retain(|_, projection| projection.expires_at > now);
    }
}

fn start_upload_digest_projection_purger() {
    static STARTED: OnceLock<()> = OnceLock::new();
    STARTED.get_or_init(|| {
        tokio::spawn(async {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(60));
            loop {
                interval.tick().await;
                purge_upload_digest_projections();
            }
        });
    });
}

fn upload_projection_from_start(value: &serde_json::Value) -> Option<UploadDigestProjection> {
    use sha2::Digest as _;
    let filename = value.get("filename")?.as_str()?.to_string();
    let total_bytes = value
        .get("total_size")
        .or_else(|| value.get("totalSize"))
        .or_else(|| value.get("bytes"))
        .or_else(|| value.get("size"))?
        .as_u64()?;
    let destination = value
        .get("target_dir")
        .or_else(|| value.get("targetDir"))
        .or_else(|| value.get("destination"))?
        .as_str()?
        .to_string();
    Some(UploadDigestProjection {
        filename,
        destination,
        total_bytes,
        hashed_bytes: 0,
        next_index: 0,
        hasher: sha2::Sha256::new(),
        expected_sha256: None,
        expires_at: std::time::Instant::now() + UPLOAD_PROJECTION_TTL,
    })
}

fn store_upload_digest_projection(upload_id: String, projection: UploadDigestProjection) {
    purge_upload_digest_projections();
    start_upload_digest_projection_purger();
    if let Ok(mut projections) = upload_digest_projections().lock() {
        projections.insert(upload_id, projection);
    }
}

fn set_upload_projection_expected_sha256(upload_id: &str, sha256: String) {
    purge_upload_digest_projections();
    if let Ok(mut projections) = upload_digest_projections().lock() {
        if let Some(projection) = projections.get_mut(upload_id) {
            projection.expected_sha256 = Some(sha256);
            projection.expires_at = std::time::Instant::now() + UPLOAD_PROJECTION_TTL;
        }
    }
}

fn clear_upload_digest_projection(upload_id: &str) {
    if let Ok(mut projections) = upload_digest_projections().lock() {
        projections.remove(upload_id);
    }
}

fn upload_projection_for_chunk(upload_id: &str, index: u64) -> Option<UploadDigestProjection> {
    purge_upload_digest_projections();
    upload_digest_projections()
        .lock()
        .ok()?
        .get(upload_id)
        .filter(|projection| projection.next_index == index)
        .cloned()
}

fn upload_digest_hex(hasher: sha2::Sha256) -> String {
    use sha2::Digest as _;
    hasher.finalize().iter().map(|byte| format!("{byte:02x}")).collect()
}

fn upload_completion_projection(upload_id: &str) -> Option<UploadDigestProjection> {
    purge_upload_digest_projections();
    upload_digest_projections()
        .lock()
        .ok()?
        .get(upload_id)
        .cloned()
}

fn upload_completion_metadata(
    projection: &UploadDigestProjection,
) -> Option<serde_json::Value> {
    if projection.hashed_bytes != projection.total_bytes {
        return None;
    }
    let sha256 = upload_digest_hex(projection.hasher.clone());
    if projection
        .expected_sha256
        .as_deref()
        .is_some_and(|expected| expected != sha256.as_str())
    {
        return None;
    }
    let metadata = serde_json::json!({
        "filename": projection.filename.as_str(),
        "bytes": projection.total_bytes,
        "sha256": sha256,
        "destination": projection.destination.as_str(),
    });
    serde_json::to_vec(&metadata)
        .is_ok_and(|encoded| encoded.len() < UPLOAD_METADATA_BODY_LIMIT)
        .then_some(metadata)
}

async fn capture_upload_body(
    body: axum::body::Body,
    limit: usize,
) -> (axum::body::Body, Option<axum::body::Bytes>) {
    use futures_util::StreamExt as _;
    let mut stream = body.into_data_stream();
    let mut buffered = Vec::new();
    let mut buffered_len = 0usize;
    while let Some(frame) = stream.next().await {
        match frame {
            Ok(bytes) if bytes.is_empty() => {}
            Ok(bytes) if bytes.len() <= limit.saturating_sub(buffered_len) => {
                buffered_len += bytes.len();
                buffered.push(bytes);
            }
            Ok(bytes) => {
                let replay = futures_util::stream::iter(
                    buffered
                        .into_iter()
                        .map(Ok::<axum::body::Bytes, axum::Error>),
                )
                .chain(futures_util::stream::once(async move {
                    Ok::<axum::body::Bytes, axum::Error>(bytes)
                }))
                .chain(stream);
                return (axum::body::Body::from_stream(replay), None);
            }
            Err(error) => {
                let replay = futures_util::stream::iter(
                    buffered
                        .into_iter()
                        .map(Ok::<axum::body::Bytes, axum::Error>),
                )
                .chain(futures_util::stream::once(async move {
                    Err::<axum::body::Bytes, axum::Error>(error)
                }))
                .chain(stream);
                return (axum::body::Body::from_stream(replay), None);
            }
        }
    }
    let mut combined = Vec::with_capacity(buffered_len);
    for frame in buffered {
        combined.extend_from_slice(&frame);
    }
    let bytes = axum::body::Bytes::from(combined);
    (axum::body::Body::from(bytes.clone()), Some(bytes))
}

async fn upload_completion_body_with_metadata(
    body: axum::body::Body,
    headers: &mut axum::http::HeaderMap,
    metadata: Option<serde_json::Value>,
) -> axum::body::Body {
    let Some(metadata) = metadata else {
        return body;
    };
    let (body, bytes) = capture_upload_body(body, UPLOAD_COMPLETION_BODY_CAPTURE_LIMIT).await;
    let Some(bytes) = bytes else {
        return body;
    };
    let mut object = if bytes.is_empty() {
        serde_json::Map::new()
    } else {
        match serde_json::from_slice::<serde_json::Value>(&bytes) {
            Ok(serde_json::Value::Object(object)) => object,
            _ => return body,
        }
    };
    if let Some(existing) = object.get_mut("metadata") {
        let Some(existing) = existing.as_object_mut() else {
            return body;
        };
        let Some(metadata) = metadata.as_object() else {
            return body;
        };
        if metadata
            .iter()
            .any(|(key, value)| existing.get(key).is_some_and(|current| current != value))
        {
            return body;
        }
        for (key, value) in metadata {
            existing.entry(key.clone()).or_insert_with(|| value.clone());
        }
    } else {
        object.insert("metadata".to_string(), metadata);
    }
    let Ok(enriched) = serde_json::to_vec(&serde_json::Value::Object(object)) else {
        return body;
    };
    if enriched.len() >= UPLOAD_COMPLETION_INTENT_LIMIT {
        return body;
    }
    headers.remove(axum::http::header::CONTENT_LENGTH);
    headers.insert(
        axum::http::header::CONTENT_TYPE,
        axum::http::HeaderValue::from_static("application/json"),
    );
    axum::body::Body::from(enriched)
}

async fn capture_upload_response(
    response: axum::response::Response,
) -> (axum::response::Response, Option<serde_json::Value>) {
    let (parts, body) = response.into_parts();
    let (body, bytes) = capture_upload_body(body, UPLOAD_RESPONSE_BODY_CAPTURE_LIMIT).await;
    let value = bytes
        .as_ref()
        .and_then(|bytes| serde_json::from_slice(bytes).ok());
    (axum::http::Response::from_parts(parts, body), value)
}

fn upload_relay_transport_failed(value: Option<&serde_json::Value>) -> bool {
    value
        .and_then(|value| value.get("schema"))
        .and_then(serde_json::Value::as_str)
        == Some("coronatio.upload.relay.error.v1")
}

fn upload_response_ok(status: axum::http::StatusCode, value: Option<&serde_json::Value>) -> bool {
    status.is_success()
        && value
            .and_then(|value| value.get("ok"))
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(true)
}

fn upload_refusal_signal(value: Option<&serde_json::Value>) -> String {
    value
        .and_then(|value| value.get("firstMissingSignal"))
        .and_then(serde_json::Value::as_str)
        .unwrap_or("caduceus-upload-relay-refused")
        .to_string()
}

#[derive(Clone)]
struct ObservedUploadChunk {
    hasher: sha2::Sha256,
    bytes: u64,
}

fn observe_upload_chunk_body(
    body: axum::body::Body,
    projection: &UploadDigestProjection,
) -> (axum::body::Body, Arc<Mutex<Option<ObservedUploadChunk>>>) {
    use sha2::Digest as _;
    use futures_util::StreamExt as _;
    let observed = Arc::new(Mutex::new(Some(ObservedUploadChunk {
        hasher: projection.hasher.clone(),
        bytes: 0,
    })));
    let stream_state = observed.clone();
    let mut hasher = projection.hasher.clone();
    let mut byte_count = 0u64;
    let stream = body.into_data_stream().map(move |frame| {
        if let Ok(bytes) = &frame {
            hasher.update(bytes);
            byte_count = byte_count.saturating_add(bytes.len() as u64);
            if let Ok(mut state) = stream_state.lock() {
                *state = Some(ObservedUploadChunk { hasher: hasher.clone(), bytes: byte_count });
            }
        }
        frame
    });
    (axum::body::Body::from_stream(stream), observed)
}

fn commit_upload_chunk_projection(
    upload_id: &str,
    index: u64,
    observed: Option<ObservedUploadChunk>,
) {
    let Some(observed) = observed else {
        clear_upload_digest_projection(upload_id);
        return;
    };
    purge_upload_digest_projections();
    if let Ok(mut projections) = upload_digest_projections().lock() {
        let Some((current_index, current_bytes, total_bytes)) = projections
            .get(upload_id)
            .map(|projection| (projection.next_index, projection.hashed_bytes, projection.total_bytes))
        else {
            return;
        };
        if current_index != index {
            return;
        }
        let Some(hashed_bytes) = current_bytes.checked_add(observed.bytes) else {
            projections.remove(upload_id);
            return;
        };
        if hashed_bytes > total_bytes {
            projections.remove(upload_id);
            return;
        }
        let Some(next_index) = index.checked_add(1) else {
            projections.remove(upload_id);
            return;
        };
        if let Some(projection) = projections.get_mut(upload_id) {
            projection.hashed_bytes = hashed_bytes;
            projection.next_index = next_index;
            projection.hasher = observed.hasher;
            projection.expires_at = std::time::Instant::now() + UPLOAD_PROJECTION_TTL;
        }
    }
}

fn upload_pin_required_refusal() -> axum::response::Response {
    (
        axum::http::StatusCode::PRECONDITION_REQUIRED,
        axum::Json(serde_json::json!({
            "schema": "coronatio.upload.pin_required.refusal.v1",
            "ok": false,
            "accepted": false,
            "error": "upload-pin-required",
            "firstMissingSignal": "upload-pin-required"
        })),
    )
        .into_response()
}

fn upload_mutation_refusal(refusal: MutationRefusal) -> axum::response::Response {
    let status = axum::http::StatusCode::from_u16(refusal.status)
        .unwrap_or(axum::http::StatusCode::SERVICE_UNAVAILABLE);
    (
        status,
        axum::Json(serde_json::json!({
            "schema": "coronatio.upload.relay.refusal.v1",
            "ok": false,
            "accepted": false,
            "error": "caduceus-upload-relay-refused",
            "firstMissingSignal": safe_access_code(&refusal.code)
        })),
    )
        .into_response()
}

fn authorize_upload(
    headers: &axum::http::HeaderMap,
) -> Result<MutationAttendance, axum::response::Response> {
    if upload_pin_required() && session_from_headers(headers) != Session::Admin {
        return Err(upload_pin_required_refusal());
    }
    let authority = mutation_authority();
    authority
        .authorize(&MutationRequestContext::from_headers(headers))
        .map_err(upload_mutation_refusal)
}

fn upload_relay_headers(
    source: &axum::http::HeaderMap,
    attendance: &MutationAttendance,
) -> axum::http::HeaderMap {
    let mut headers = axum::http::HeaderMap::new();
    for (name, value) in source {
        if matches!(
            name.as_str(),
            "connection" | "host" | "keep-alive" | "proxy-authenticate" | "proxy-authorization"
                | "te" | "trailer" | "transfer-encoding" | "upgrade"
        ) {
            continue;
        }
        headers.append(name.clone(), value.clone());
    }
    headers.insert(
        axum::http::header::HOST,
        axum::http::HeaderValue::from_static("caduceus.local"),
    );
    headers.insert(
        "x-caduceus-attendance",
        axum::http::HeaderValue::from_str(attendance.proof.expose())
            .expect("validated attendance proof is a header value"),
    );
    headers.insert(
        "x-caduceus-document",
        axum::http::HeaderValue::from_str(&attendance.document)
            .expect("validated document incarnation is a header value"),
    );
    headers
}

async fn relay_upload_body(
    method: &str,
    path: &str,
    source_headers: &axum::http::HeaderMap,
    attendance: &MutationAttendance,
    body: axum::body::Body,
) -> Result<axum::response::Response, String> {
    let socket = crate::caduceus_access::staff_socket_path();
    let stream = tokio::time::timeout(UPLOAD_RELAY_TIMEOUT, tokio::net::UnixStream::connect(socket))
        .await
        .map_err(|_| "caduceus-upload-relay-timeout".to_string())?
        .map_err(|error| format!("caduceus-upload-relay-connect:{error}"))?;
    let io = hyper_util::rt::TokioIo::new(stream);
    let (mut sender, connection) = hyper::client::conn::http1::handshake(io)
        .await
        .map_err(|error| format!("caduceus-upload-relay-handshake:{error}"))?;
    tokio::spawn(async move {
        let _ = connection.await;
    });

    let mut request = hyper::Request::new(body);
    *request.method_mut() = method
        .parse()
        .map_err(|_| "caduceus-upload-relay-method-invalid".to_string())?;
    *request.uri_mut() = path
        .parse()
        .map_err(|_| "caduceus-upload-relay-path-invalid".to_string())?;
    *request.headers_mut() = upload_relay_headers(source_headers, attendance);
    let response = sender
        .send_request(request)
        .await
        .map_err(|error| format!("caduceus-upload-relay-request:{error}"))?;
    let (parts, body) = response.into_parts();
    Ok(axum::http::Response::from_parts(
        parts,
        axum::body::Body::new(body),
    ))
}

fn upload_relay_failure_response(error: &str) -> axum::response::Response {
    (
        axum::http::StatusCode::SERVICE_UNAVAILABLE,
        axum::Json(serde_json::json!({
            "schema": "coronatio.upload.relay.error.v1",
            "ok": false,
            "accepted": false,
            "error": "caduceus-upload-relay-unavailable",
            "firstMissingSignal": safe_access_code(error)
        })),
    )
        .into_response()
}

async fn upload_relay_response(
    method: &str,
    path: String,
    headers: axum::http::HeaderMap,
    attendance: MutationAttendance,
    body: axum::body::Body,
) -> axum::response::Response {
    match relay_upload_body(method, &path, &headers, &attendance, body).await {
        Ok(response) => response,
        Err(error) => upload_relay_failure_response(&error),
    }
}

async fn upload_start_route(
    headers: axum::http::HeaderMap,
    body: axum::body::Body,
) -> axum::response::Response {
    let attendance = match authorize_upload(&headers) {
        Ok(attendance) => attendance,
        Err(response) => return response,
    };
    let (body, captured_body) = capture_upload_body(body, UPLOAD_START_BODY_LIMIT).await;
    let start_metadata = captured_body
        .and_then(|bytes| serde_json::from_slice::<serde_json::Value>(&bytes).ok());
    let response = match relay_upload_body(
        "POST",
        "/api/v1/file/ingress/start",
        &headers,
        &attendance,
        body,
    )
    .await
    {
        Ok(response) => response,
        Err(error) => return upload_relay_failure_response(&error),
    };
    let (response, readback) = capture_upload_response(response).await;
    if response.status().is_success() {
        let upload_id = readback
            .as_ref()
            .and_then(|value| value.get("upload_id").or_else(|| value.get("uploadId")).or_else(|| value.get("id")))
            .and_then(serde_json::Value::as_str);
        if let (Some(upload_id), Some(projection)) = (
            upload_id,
            start_metadata.as_ref().and_then(upload_projection_from_start),
        ) {
            store_upload_digest_projection(upload_id.to_string(), projection);
        }
    }
    response
}

async fn upload_chunk_route(
    axum::extract::Path((upload_id, index)): axum::extract::Path<(String, String)>,
    headers: axum::http::HeaderMap,
    body: axum::body::Body,
) -> axum::response::Response {
    let attendance = match authorize_upload(&headers) {
        Ok(attendance) => attendance,
        Err(response) => return response,
    };
    let Ok(index_number) = index.parse::<u64>() else {
        return upload_relay_response(
            "POST",
            format!("/api/v1/file/ingress/{upload_id}/chunk/{index}"),
            headers,
            attendance,
            body,
        )
        .await;
    };
    let projection = upload_projection_for_chunk(&upload_id, index_number);
    let (body, observed) = match projection.as_ref() {
        Some(projection) => {
            let (body, observed) = observe_upload_chunk_body(body, projection);
            (body, Some(observed))
        }
        None => (body, None),
    };
    let response = upload_relay_response(
        "POST",
        format!("/api/v1/file/ingress/{upload_id}/chunk/{index}"),
        headers,
        attendance,
        body,
    )
    .await;
    let (response, readback) = capture_upload_response(response).await;
    if upload_relay_transport_failed(readback.as_ref()) {
        clear_upload_digest_projection(&upload_id);
    } else if upload_response_ok(response.status(), readback.as_ref()) {
        if let Some(observed) = observed {
            let observed = observed.lock().ok().and_then(|state| state.clone());
            commit_upload_chunk_projection(&upload_id, index_number, observed);
        }
    }
    response
}

async fn upload_complete_route(
    axum::extract::Path(upload_id): axum::extract::Path<String>,
    mut headers: axum::http::HeaderMap,
    body: axum::body::Body,
) -> axum::response::Response {
    let attendance = match authorize_upload(&headers) {
        Ok(attendance) => attendance,
        Err(response) => return response,
    };
    let metadata = upload_completion_projection(&upload_id)
        .as_ref()
        .and_then(upload_completion_metadata);
    let body = upload_completion_body_with_metadata(body, &mut headers, metadata).await;
    let response = upload_relay_response(
        "POST",
        format!("/api/v1/file/ingress/{upload_id}/complete"),
        headers,
        attendance,
        body,
    )
    .await;
    let (response, readback) = capture_upload_response(response).await;
    if upload_response_ok(response.status(), readback.as_ref()) {
        clear_upload_digest_projection(&upload_id);
    }
    response
}

async fn upload_delete_route(
    axum::extract::Path(upload_id): axum::extract::Path<String>,
    headers: axum::http::HeaderMap,
    body: axum::body::Body,
) -> axum::response::Response {
    let attendance = match authorize_upload(&headers) {
        Ok(attendance) => attendance,
        Err(response) => return response,
    };
    clear_upload_digest_projection(&upload_id);
    upload_relay_response(
        "DELETE",
        format!("/api/v1/file/ingress/{upload_id}"),
        headers,
        attendance,
        body,
    )
    .await
}
