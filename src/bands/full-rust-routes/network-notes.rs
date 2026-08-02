#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct NetworkNoteWrite {
    mac: String,
    note: String,
}

fn canonical_network_note_mac(raw: &str) -> Option<String> {
    let bytes = raw.as_bytes();
    if bytes.len() != 17 {
        return None;
    }
    let separator = bytes[2];
    if separator != b':' && separator != b'-' {
        return None;
    }
    let mut canonical = String::with_capacity(17);
    for octet in 0..6 {
        let offset = octet * 3;
        if !bytes[offset].is_ascii_hexdigit() || !bytes[offset + 1].is_ascii_hexdigit() {
            return None;
        }
        if octet > 0 {
            if bytes[offset - 1] != separator {
                return None;
            }
            canonical.push(':');
        }
        canonical.push((bytes[offset] as char).to_ascii_uppercase());
        canonical.push((bytes[offset + 1] as char).to_ascii_uppercase());
    }
    Some(canonical)
}

async fn network_notes_read_route() -> Response {
    let readback = caduceus_http("GET", "/api/v1/network/notes");
    let notes = readback.body.get("notes").cloned();
    match (readback.ok, notes) {
        (true, Some(notes)) if notes.is_object() => (
            StatusCode::OK,
            Json(serde_json::json!({
                "schema": "coronatio.network.notes.projection.v1",
                "ok": true,
                "notes": notes,
                "authority": "Caduceus durable notes map",
                "firstMissingSignal": "none"
            })),
        )
            .into_response(),
        _ => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({
                "schema": "coronatio.network.notes.projection.v1",
                "ok": false,
                "notes": {},
                "authority": "Caduceus durable notes map",
                "firstMissingSignal": readback.first_missing_signal
            })),
        )
            .into_response(),
    }
}

async fn network_notes_write_route(headers: axum::http::HeaderMap, body: Bytes) -> Response {
    let payload: NetworkNoteWrite = match serde_json::from_slice::<NetworkNoteWrite>(&body) {
        Ok(payload) => payload,
        _ => return network_note_invalid_payload_response(),
    };
    let mac = match canonical_network_note_mac(&payload.mac) {
        Some(mac) => mac,
        None => return network_note_invalid_payload_response(),
    };
    let authority = mutation_authority();
    let attendance = match authority.authorize(
        &MutationRequestContext::from_headers(&headers),
        MutationActionTarget::caduceus("coronatio.network.notes.put", "/api/v1/network/notes"),
    ) {
        Ok(attendance) => attendance,
        Err(refusal) => return network_note_write_response(mutation_refusal_readback("/api/v1/network/notes", refusal), false),
    };
    let requested = serde_json::json!({"mac": mac, "note": payload.note});
    let readback = caduceus_http_json_with_attendance_and_document(
        "PUT",
        "/api/v1/network/notes",
        requested.clone(),
        Some(&attendance.proof),
        Some(&attendance.document),
    );
    let completed = readback.ok
        && readback.status != StatusCode::ACCEPTED.as_u16()
        && readback.body.get("completed").and_then(serde_json::Value::as_bool) == Some(true)
        && readback.body.get("notes").and_then(serde_json::Value::as_object)
            .and_then(|notes| notes.get(requested["mac"].as_str().unwrap_or_default()))
            == Some(&requested["note"]);
    network_note_write_response(readback, completed)
}

fn network_note_invalid_payload_response() -> Response {
    (
        StatusCode::BAD_REQUEST,
        Json(serde_json::json!({
            "schema": "coronatio.network.notes.mutation.v1",
            "ok": false,
            "accepted": false,
            "error": "network-note-payload-invalid",
            "firstMissingSignal": "network-note-payload-invalid"
        })),
    )
        .into_response()
}

fn network_note_write_response(readback: CaduceusHttpReadback, completed: bool) -> Response {
    let status = if completed {
        StatusCode::OK
    } else if readback.ok {
        StatusCode::SERVICE_UNAVAILABLE
    } else {
        mutation_response_status(&readback)
    };
    (
        status,
        Json(serde_json::json!({
            "schema": "coronatio.network.notes.mutation.v1",
            "ok": completed,
            "accepted": completed,
            "completed": completed,
            "notes": if completed { readback.body.get("notes").cloned().unwrap_or_else(|| serde_json::json!({})) } else { serde_json::json!({}) },
            "authority": "Caduceus durable notes map",
            "firstMissingSignal": if completed { "none".to_string() } else { readback.first_missing_signal }
        })),
    )
        .into_response()
}
