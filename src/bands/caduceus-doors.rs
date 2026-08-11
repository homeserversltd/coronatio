// Caduceus owns the door seat. Coronatio keeps its raw, last-good seat beside
// typed rows so unknown future fields remain carried through the cache.
const CADUCEUS_DOOR_CACHE_PATH: &str = "/var/lib/coronatio/doors-cache.json";

#[derive(Clone, Debug, Deserialize)]
struct CaduceusDoorSeat {
    schema: String,
    doors: Vec<CaduceusDoorRow>,
}

#[derive(Clone, Debug, Deserialize)]
struct CaduceusDoorRow {
    method: String,
    path: String,
    family: String,
    snake: String,
    posture: String,
    #[serde(default)]
    crown_alias: Option<String>,
    #[serde(default)]
    crown_aliases: Option<Vec<String>>,
    #[serde(default)]
    actuator: Option<String>,
}

#[derive(Clone, Debug)]
struct CaduceusDoorCache {
    raw_seat_bytes: Vec<u8>,
    seat: CaduceusDoorSeat,
}

#[derive(Clone, Debug)]
struct ResolvedCaduceusDoor {
    method: String,
    path: String,
    posture: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CaduceusDoorResolutionFailure {
    Unmapped,
    Unavailable,
}

static CADUCEUS_DOOR_CACHE: OnceLock<Mutex<Option<CaduceusDoorCache>>> = OnceLock::new();

fn caduceus_door_cache() -> &'static Mutex<Option<CaduceusDoorCache>> {
    CADUCEUS_DOOR_CACHE.get_or_init(|| Mutex::new(None))
}

fn parse_caduceus_door_seat(raw_seat_bytes: Vec<u8>) -> Option<CaduceusDoorCache> {
    let seat = serde_json::from_slice::<CaduceusDoorSeat>(&raw_seat_bytes).ok()?;
    (seat.schema == "caduceus.doors.v1").then_some(CaduceusDoorCache {
        raw_seat_bytes,
        seat,
    })
}

fn cached_caduceus_door_seat() -> Option<CaduceusDoorCache> {
    if let Some(cache) = caduceus_door_cache().lock().ok()?.clone() {
        return Some(cache);
    }
    let cache = parse_caduceus_door_seat(std::fs::read(CADUCEUS_DOOR_CACHE_PATH).ok()?)?;
    if let Ok(mut held) = caduceus_door_cache().lock() {
        *held = Some(cache.clone());
    }
    Some(cache)
}

fn persist_caduceus_door_seat(raw_seat_bytes: &[u8]) {
    let root = FsPath::new(CADUCEUS_DOOR_CACHE_PATH)
        .parent()
        .unwrap_or_else(|| FsPath::new("/var/lib/coronatio"));
    if std::fs::create_dir_all(root).is_err() {
        return;
    }
    let temporary = root.join("doors-cache.json.tmp");
    if std::fs::write(&temporary, raw_seat_bytes).is_ok() {
        let _ = std::fs::rename(temporary, CADUCEUS_DOOR_CACHE_PATH);
    }
}

fn refresh_caduceus_door_seat() -> Option<CaduceusDoorCache> {
    let Some(authority) = caduceus_authority() else {
        return cached_caduceus_door_seat();
    };
    let mut stream = TcpStream::connect(&authority).ok()?;
    let _ = stream.set_read_timeout(Some(Duration::from_secs(4)));
    let _ = stream.set_write_timeout(Some(Duration::from_secs(4)));
    let request = format!(
        "GET /api/v1/doors HTTP/1.1\r\nHost: {authority}\r\nConnection: close\r\nContent-Length: 0\r\n\r\n"
    );
    stream.write_all(request.as_bytes()).ok()?;
    let mut response = Vec::new();
    stream.read_to_end(&mut response).ok()?;
    let separator = b"\r\n\r\n";
    let body_start = response
        .windows(separator.len())
        .position(|window| window == separator)?
        + separator.len();
    let response_body = std::str::from_utf8(&response[body_start..]).ok()?;
    #[derive(Deserialize)]
    struct DoorReadback<'a> {
        schema: &'a str,
        ok: bool,
        #[serde(borrow)]
        seat: &'a serde_json::value::RawValue,
    }
    let readback = serde_json::from_str::<DoorReadback<'_>>(response_body).ok()?;
    if readback.schema != "caduceus.doors.readback.v1" || !readback.ok {
        return cached_caduceus_door_seat();
    }
    let cache = parse_caduceus_door_seat(readback.seat.get().as_bytes().to_vec())?;
    persist_caduceus_door_seat(&cache.raw_seat_bytes);
    if let Ok(mut held) = caduceus_door_cache().lock() {
        *held = Some(cache.clone());
    }
    Some(cache)
}

fn caduceus_door_shape_matches(shape: &str, path: &str) -> Option<Vec<(String, String)>> {
    let shape_segments = shape.trim_matches('/').split('/').collect::<Vec<_>>();
    let path_segments = path.trim_matches('/').split('/').collect::<Vec<_>>();
    if shape_segments.len() != path_segments.len() {
        return None;
    }
    let mut values = Vec::new();
    for (shape_segment, path_segment) in shape_segments.iter().zip(path_segments) {
        let name = shape_segment
            .strip_prefix('{')
            .and_then(|segment| segment.strip_suffix('}'))
            .or_else(|| shape_segment.strip_prefix(':'));
        if let Some(name) = name {
            if name.is_empty() || path_segment.is_empty() {
                return None;
            }
            values.push((name.to_string(), path_segment.to_string()));
        } else if shape_segment != &path_segment {
            return None;
        }
    }
    Some(values)
}

fn caduceus_door_path_with_values(shape: &str, values: &[(String, String)]) -> String {
    values
        .iter()
        .fold(shape.to_string(), |path, (name, value)| {
            path.replace(&format!("{{{name}}}"), value)
                .replace(&format!(":{name}"), value)
        })
}

fn resolve_from_caduceus_door_seat(
    seat: &CaduceusDoorSeat,
    method: &str,
    crown_path: &str,
) -> Option<ResolvedCaduceusDoor> {
    let method = method.to_ascii_uppercase();
    let mut candidates = seat
        .doors
        .iter()
        .filter(|row| row.method.eq_ignore_ascii_case(&method))
        .filter_map(|row| {
            let values = match row.crown_aliases.as_deref() {
                Some(aliases) => aliases
                    .iter()
                    .find_map(|alias| caduceus_door_shape_matches(alias, crown_path)),
                None => row
                    .crown_alias
                    .as_deref()
                    .and_then(|alias| caduceus_door_shape_matches(alias, crown_path)),
            }?;
            Some((row, values))
        })
        .collect::<Vec<_>>();
    candidates.sort_by_key(|(row, values)| (usize::from(!values.is_empty()), row.path.len()));
    candidates
        .into_iter()
        .next()
        .map(|(row, values)| ResolvedCaduceusDoor {
            method: row.method.to_ascii_uppercase(),
            path: caduceus_door_path_with_values(&row.path, &values),
            posture: row.posture.clone(),
        })
}

fn resolve_caduceus_door(
    method: &str,
    crown_path: &str,
) -> Result<ResolvedCaduceusDoor, CaduceusDoorResolutionFailure> {
    if let Some(cache) = cached_caduceus_door_seat() {
        if let Some(door) = resolve_from_caduceus_door_seat(&cache.seat, method, crown_path) {
            return Ok(door);
        }
    }
    let Some(cache) = refresh_caduceus_door_seat() else {
        return Err(CaduceusDoorResolutionFailure::Unavailable);
    };
    resolve_from_caduceus_door_seat(&cache.seat, method, crown_path)
        .ok_or(CaduceusDoorResolutionFailure::Unmapped)
}

fn preload_caduceus_door_seat() {
    let _ = refresh_caduceus_door_seat();
}
