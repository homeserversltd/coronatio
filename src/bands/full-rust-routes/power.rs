fn power_usage_response(method: &str, path: &str) -> Response {
    match read_power_usage_sample() {
        Ok(sample) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "schema": "coronatio.power.usage.v1",
                "ok": true,
                "success": true,
                "method": method,
                "path": path,
                "family": "power-monitor",
                "authority": "Coronatio Rust RAPL read route",
                "current": sample.current_watts,
                "historical": sample.history_watts,
                "unit": "W",
                "timestamp": sample.timestamp_secs,
                "status": format!("{:.2}", sample.current_watts),
                "firstMissingSignal": "none"
            })),
        )
            .into_response(),
        Err(first_missing_signal) => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({
                "schema": "coronatio.power.usage.v1",
                "ok": false,
                "success": false,
                "method": method,
                "path": path,
                "family": "power-monitor",
                "authority": "Coronatio Rust RAPL read route",
                "error": "Initializing power monitoring",
                "firstMissingSignal": first_missing_signal
            })),
        )
            .into_response(),
    }
}

struct PowerUsageSample {
    current_watts: f64,
    history_watts: Vec<f64>,
    timestamp_secs: f64,
}

static POWER_HISTORY_WATTS: std::sync::Mutex<Vec<f64>> = std::sync::Mutex::new(Vec::new());

fn read_power_usage_sample() -> Result<PowerUsageSample, &'static str> {
    let first = read_rapl_energy_total_uj()?;
    let first_time = std::time::Instant::now();
    std::thread::sleep(std::time::Duration::from_millis(250));
    let second = read_rapl_energy_total_uj()?;
    let elapsed = first_time.elapsed().as_secs_f64();
    if elapsed <= 0.0 {
        return Err("rapl-time-delta-missing");
    }

    let energy_delta_uj = second.saturating_sub(first);
    let watts = (energy_delta_uj as f64 / elapsed) / 1_000_000.0;
    if !(0.0..=100.0).contains(&watts) {
        return Err("rapl-watts-implausible");
    }

    let mut history = POWER_HISTORY_WATTS
        .lock()
        .map_err(|_| "rapl-history-lock-poisoned")?;
    history.push(watts);
    if history.len() > 60 {
        let drain = history.len() - 60;
        history.drain(0..drain);
    }

    Ok(PowerUsageSample {
        current_watts: watts,
        history_watts: history.clone(),
        timestamp_secs: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|_| "system-clock-before-epoch")?
            .as_secs_f64(),
    })
}

fn read_rapl_energy_total_uj() -> Result<u64, &'static str> {
    let child_paths = [
        "/sys/class/powercap/intel-rapl:0:0/energy_uj",
        "/sys/class/powercap/intel-rapl:0:1/energy_uj",
    ];
    let mut child_total = 0_u64;
    let mut child_count = 0_u64;
    for path in child_paths {
        if let Some(value) = read_unsigned_sysfs(path) {
            child_total = child_total.saturating_add(value);
            child_count += 1;
        }
    }
    if child_count > 0 {
        return Ok(child_total);
    }

    read_unsigned_sysfs("/sys/class/powercap/intel-rapl:0/energy_uj")
        .ok_or("rapl-energy-readable-file-missing")
}

fn read_unsigned_sysfs(path: &str) -> Option<u64> {
    std::fs::read_to_string(path).ok()?.trim().parse::<u64>().ok()
}
