#[test]
fn chunked_upload_route_inventory_and_mapping_are_exact() {
    let expected = [
        ("/api/files/upload/start", &["post"][..]),
        ("/api/files/upload/:upload_id/chunk/:index", &["post"][..]),
        ("/api/files/upload/:upload_id/complete", &["post"][..]),
        ("/api/files/upload/:upload_id", &["delete"][..]),
    ];
    for (path, methods) in expected {
        assert_eq!(
            full_rust_route_inventory()
                .iter()
                .find(|(candidate, _)| *candidate == path)
                .map(|(_, registered)| *registered),
            Some(methods),
            "missing or mismatched route {path}"
        );
    }
    assert_eq!(UPLOAD_CHUNK_BODY_LIMIT, 8_454_144);
}

#[test]
fn chunked_upload_relay_keeps_streaming_seam_and_scoped_limit() {
    let relay = include_str!("../upload-relay.rs");
    assert!(relay.contains("hyper::client::conn::http1::handshake"));
    assert!(relay.contains("axum::body::Body"));
    assert!(relay.contains("axum::body::Body::new(body)"));
    assert!(!relay.contains("to_bytes"));
    assert!(!relay.contains("collect"));
    assert!(!relay.contains("Vec"));
    let routes = include_str!("../full-rust-routes.rs");
    let chunk_route = routes
        .split("/api/files/upload/:upload_id/chunk/:index")
        .nth(1)
        .expect("chunk route registration");
    assert!(chunk_route.contains("RequestBodyLimitLayer::new(UPLOAD_CHUNK_BODY_LIMIT)"));
    assert!(!routes
        .split(".route(\"/api/files/upload/start\"")
        .next()
        .unwrap_or("")
        .contains("RequestBodyLimitLayer"));
}

#[test]
fn chunked_upload_relay_preserves_upstream_refusal_parts() {
    let relay = include_str!("../upload-relay.rs");
    assert!(relay.contains("\"POST\",\n        \"/api/v1/file/ingress/start\""));
    assert!(relay.contains("\"POST\",\n        format!(\"/api/v1/file/ingress/{upload_id}/chunk/{index}\")"));
    assert!(relay.contains("\"POST\",\n        format!(\"/api/v1/file/ingress/{upload_id}/complete\")"));
    assert!(relay.contains("\"DELETE\",\n        format!(\"/api/v1/file/ingress/{upload_id}\")"));
    assert!(relay.contains("x-caduceus-attendance"));
    assert!(relay.contains("x-caduceus-document"));
    assert!(relay.contains("let (parts, body) = response.into_parts();"));
    assert!(relay.contains("axum::http::Response::from_parts(parts"));
    assert!(relay.contains("Ok(response) => response"));
}

#[test]
fn chunked_upload_frontend_is_sequential_and_remove_aborts_by_id() {
    let shell = shell_document_4();
    assert!(shell.contains("const CHUNK_SIZE = 4194304"));
    assert!(shell.contains("filename: file.name, total_size: file.size, target_dir: uploadCurrentPath(), chunk_size: CHUNK_SIZE"));
    assert!(shell.contains("while (uploaded < file.size)"));
    assert!(shell.contains("file.slice(offset, end)"));
    assert!(shell.contains("xhr.upload.onprogress"));
    assert!(shell.contains("const cumulative = Math.min(file.size, offset + event.loaded)"));
    assert!(shell.contains("/api/files/upload/' + encodeURIComponent(startUpload) + '/complete"));
    assert!(shell.contains("upload.xhr?.abort()"));
    assert!(shell.contains("method: 'DELETE'"));
    assert!(shell.contains("/api/files/upload/' + encodeURIComponent(upload.uploadId)"));
    assert!(shell.contains("for (const file of uploadState.selectedFiles)"));
    assert!(shell.contains("xhr.setRequestHeader('X-Caduceus-Document'"));
}

#[test]
fn chunked_upload_frontend_guards_chunk_headers_and_remove_completion_races() {
    let shell = shell_document_4();
    assert!(shell.contains(
        "xhr.setRequestHeader('Content-Type', 'application/octet-stream');\n          xhr.send(chunk);"
    ));
    assert!(shell.contains("const uploadRecord = uploadState.activeUploads.get(file.name);"));
    assert!(shell.contains("const removed = () => uploadRecord?.removed === true;"));

    let remove_handler = shell
        .split("if (control.matches('[data-upload-remove]'))")
        .nth(1)
        .expect("upload remove handler");
    let delete_start = remove_handler
        .find("cleanup = fetch('/api/files/upload/' + encodeURIComponent(upload.uploadId)")
        .expect("delete cleanup initiation");
    let row_remove = remove_handler
        .find("uploadState.activeUploads.delete(filename); renderUploadProgress();")
        .expect("immediate row removal");
    let cleanup_await = remove_handler
        .find("if (cleanup) await cleanup;")
        .expect("network cleanup await");
    assert!(delete_start < row_remove, "DELETE must start before row removal");
    assert!(row_remove < cleanup_await, "row removal must precede cleanup await");

    assert!(shell.contains(
        "const completeText = await complete.text();\n      if (removed()) throw Object.assign(new Error('Upload removed'), { uploadRemoved: true });"
    ));
}
