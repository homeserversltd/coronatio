const UPLOAD_CHUNK_BODY_LIMIT: usize = 8_454_144;
const UPLOAD_RELAY_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(3);

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

fn authorize_upload(headers: &axum::http::HeaderMap) -> Result<MutationAttendance, axum::response::Response> {
    if upload_pin_required() && session_from_headers(headers) != Session::Admin {
        return Err(upload_pin_required_refusal());
    }
    let authority = mutation_authority();
    let mapping = MutationActionTarget::caduceus("coronatio.file.ingress", "/api/v1/file/ingress");
    authority
        .authorize(&mapping.request_context(headers), mapping)
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

async fn upload_relay_response(
    method: &str,
    path: String,
    headers: axum::http::HeaderMap,
    attendance: MutationAttendance,
    body: axum::body::Body,
) -> axum::response::Response {
    match relay_upload_body(method, &path, &headers, &attendance, body).await {
        Ok(response) => response,
        Err(error) => (
            axum::http::StatusCode::SERVICE_UNAVAILABLE,
            axum::Json(serde_json::json!({
                "schema": "coronatio.upload.relay.error.v1",
                "ok": false,
                "accepted": false,
                "error": "caduceus-upload-relay-unavailable",
                "firstMissingSignal": safe_access_code(&error)
            })),
        )
            .into_response(),
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
    upload_relay_response(
        "POST",
        "/api/v1/file/ingress/start".to_string(),
        headers,
        attendance,
        body,
    )
    .await
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
    upload_relay_response(
        "POST",
        format!("/api/v1/file/ingress/{upload_id}/chunk/{index}"),
        headers,
        attendance,
        body,
    )
    .await
}

async fn upload_complete_route(
    axum::extract::Path(upload_id): axum::extract::Path<String>,
    headers: axum::http::HeaderMap,
    body: axum::body::Body,
) -> axum::response::Response {
    let attendance = match authorize_upload(&headers) {
        Ok(attendance) => attendance,
        Err(response) => return response,
    };
    upload_relay_response(
        "POST",
        format!("/api/v1/file/ingress/{upload_id}/complete"),
        headers,
        attendance,
        body,
    )
    .await
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
    upload_relay_response(
        "DELETE",
        format!("/api/v1/file/ingress/{upload_id}"),
        headers,
        attendance,
        body,
    )
    .await
}
