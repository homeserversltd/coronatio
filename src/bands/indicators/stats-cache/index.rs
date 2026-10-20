const CADUCEUS_STATS_PATH: &str = "/api/v1/appliance/stats";
const CADUCEUS_STATS_ADMIN_PATH: &str = "/api/v1/appliance/stats/admin";
const CADUCEUS_STATS_HISTORY_PATH: &str = "/api/v1/appliance/stats/history?limit=60";
const STATS_SAMPLE_TTL: Duration = Duration::from_secs(1);
const STATS_HISTORY_TTL: Duration = Duration::from_secs(5);
const HOUSEHOLD_METADATA_TTL: Duration = Duration::from_secs(30);

#[derive(Clone)]
pub(crate) struct CaduceusStatsReadback {
    pub(crate) ok: bool,
    pub(crate) status: u16,
    pub(crate) body: serde_json::Value,
    pub(crate) first_missing_signal: String,
    pub(crate) sampled_at: Option<Instant>,
    pub(crate) sampled_at_unix_ms: Option<u64>,
    pub(crate) sample_id: Option<String>,
    pub(crate) generation: u64,
    pub(crate) stale: bool,
    pub(crate) age: Option<Duration>,
}

impl From<CaduceusHttpReadback> for CaduceusStatsReadback {
    fn from(readback: CaduceusHttpReadback) -> Self {
        Self {
            ok: readback.ok,
            status: readback.status,
            body: readback.body,
            first_missing_signal: readback.first_missing_signal,
            sampled_at: None,
            sampled_at_unix_ms: None,
            sample_id: None,
            generation: 0,
            stale: false,
            age: None,
        }
    }
}

#[derive(Default)]
struct StatsReadCacheCell {
    value: Option<(Instant, CaduceusStatsReadback)>,
    last_good: Option<CaduceusStatsReadback>,
    loading: bool,
    generation: u64,
}

#[derive(Default)]
struct StatsReadCache {
    current: StatsReadCacheCell,
    history: StatsReadCacheCell,
}

#[derive(Clone, Copy)]
enum StatsReadCacheKind {
    Current,
    History,
}

static STATS_READ_CACHE: OnceLock<(Mutex<StatsReadCache>, std::sync::Condvar)> = OnceLock::new();

fn stats_read_cache() -> &'static (Mutex<StatsReadCache>, std::sync::Condvar) {
    STATS_READ_CACHE.get_or_init(|| (Mutex::new(StatsReadCache::default()), std::sync::Condvar::new()))
}

fn stats_cache_cell(cache: &mut StatsReadCache, kind: StatsReadCacheKind) -> &mut StatsReadCacheCell {
    match kind {
        StatsReadCacheKind::Current => &mut cache.current,
        StatsReadCacheKind::History => &mut cache.history,
    }
}

fn stats_cache_cell_ref(cache: &StatsReadCache, kind: StatsReadCacheKind) -> &StatsReadCacheCell {
    match kind {
        StatsReadCacheKind::Current => &cache.current,
        StatsReadCacheKind::History => &cache.history,
    }
}

fn stats_cache_source(kind: StatsReadCacheKind) -> &'static str {
    match kind {
        StatsReadCacheKind::Current => "stats.current",
        StatsReadCacheKind::History => "stats.history",
    }
}

fn stats_cache_unix_millis() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis().min(u64::MAX as u128) as u64
}

fn failed_stats_observation(_kind: StatsReadCacheKind, signal: &str, _generation: u64, last_good: Option<&CaduceusStatsReadback>) -> CaduceusStatsReadback {
    if let Some(last_good) = last_good {
        let mut stale = last_good.clone();
        stale.ok = false;
        stale.stale = true;
        stale.first_missing_signal = signal.to_owned();
        stale.age = stale.sampled_at.map(|at| at.elapsed());
        stale
    } else {
        CaduceusStatsReadback {
            ok: false,
            status: 503,
            body: serde_json::json!({"ok":false,"firstMissingSignal":signal}),
            first_missing_signal: signal.to_owned(),
            sampled_at: None,
            sampled_at_unix_ms: None,
            sample_id: None,
            generation: 0,
            stale: true,
            age: None,
        }
    }
}

fn cached_caduceus_readback(
    kind: StatsReadCacheKind,
    ttl: Duration,
    fetch: impl FnOnce() -> CaduceusHttpReadback,
) -> CaduceusStatsReadback {
    let (lock, ready) = stats_read_cache();
    let deadline = Instant::now() + Duration::from_secs(4);
    let mut fetch = Some(fetch);
    loop {
        let mut cache = lock.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        let cell = stats_cache_cell(&mut cache, kind);
        if let Some((observed_at, value)) = cell.value.as_ref() {
            if observed_at.elapsed() < ttl {
                let mut result = value.clone();
                result.age = result.sampled_at.map(|at| at.elapsed());
                let state = if result.stale { "stale" } else { "hit" };
                let sample_id = result.sample_id.clone();
                let age = result.age;
                drop(cache);
                crate::load_trace::record_cache_state(stats_cache_source(kind), state, sample_id, age);
                return result;
            }
        }
        if cell.loading {
            let Some(remaining) = deadline.checked_duration_since(Instant::now()) else {
                let result = failed_stats_observation(kind, "stats-cache-wait-timeout", cell.generation, cell.last_good.as_ref());
                drop(cache);
                crate::load_trace::record_cache_state(stats_cache_source(kind), "timeout", result.sample_id.clone(), result.age);
                return result;
            };
            let (next, timeout) = ready.wait_timeout(cache, remaining).unwrap_or_else(|poisoned| poisoned.into_inner());
            cache = next;
            if timeout.timed_out() && stats_cache_cell_ref(&cache, kind).loading {
                let cell = stats_cache_cell_ref(&cache, kind);
                let result = failed_stats_observation(kind, "stats-cache-wait-timeout", cell.generation, cell.last_good.as_ref());
                drop(cache);
                crate::load_trace::record_cache_state(stats_cache_source(kind), "timeout", result.sample_id.clone(), result.age);
                return result;
            }
            drop(cache);
            continue;
        }
        let generation = cell.generation;
        stats_cache_cell(&mut cache, kind).loading = true;
        let read = fetch.take().expect("one Caduceus read per cache miss");
        drop(cache);
        let fetched = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(read)) {
            Ok(readback) => CaduceusStatsReadback::from(readback),
            Err(panic) => {
                let mut cache = lock.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
                stats_cache_cell(&mut cache, kind).loading = false;
                ready.notify_all();
                drop(cache);
                std::panic::resume_unwind(panic);
            }
        };
        let observed_at = Instant::now();
        let mut cache = lock.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        let cell = stats_cache_cell(&mut cache, kind);
        let mut result = fetched;
        if cell.generation == generation {
            let completed_generation = generation.wrapping_add(1);
            if result.ok {
                result.generation = completed_generation;
                result.sampled_at = Some(observed_at);
                result.sampled_at_unix_ms = Some(stats_cache_unix_millis());
                result.sample_id = Some(format!("{}-{completed_generation}", stats_cache_source(kind)));
                result.age = Some(Duration::ZERO);
                result.stale = false;
                cell.last_good = Some(result.clone());
            } else {
                result.stale = true;
                result.sample_id = None;
                result.sampled_at = None;
                result.sampled_at_unix_ms = None;
                result.age = None;
                result.generation = 0;
                if let Some(last_good) = cell.last_good.as_ref() {
                    let mut stale = last_good.clone();
                    stale.ok = false;
                    stale.status = result.status;
                    stale.first_missing_signal = result.first_missing_signal.clone();
                    stale.stale = true;
                    stale.age = stale.sampled_at.map(|at| at.elapsed());
                    result = stale;
                }
            }
            cell.value = Some((observed_at, result.clone()));
            cell.generation = completed_generation;
        }
        cell.loading = false;
        ready.notify_all();
        let state = if result.stale { "stale" } else { "miss" };
        let sample_id = result.sample_id.clone();
        let age = result.age;
        drop(cache);
        crate::load_trace::record_cache_state(stats_cache_source(kind), state, sample_id, age);
        return result;
    }
}

pub(crate) fn peek_caduceus_stats_sample() -> Option<CaduceusStatsReadback> {
    peek_stats_readback(StatsReadCacheKind::Current)
}

pub(crate) fn peek_caduceus_stats_history() -> Option<CaduceusStatsReadback> {
    peek_stats_readback(StatsReadCacheKind::History)
}

fn peek_stats_readback(kind: StatsReadCacheKind) -> Option<CaduceusStatsReadback> {
    let (lock, _) = stats_read_cache();
    let cache = lock.try_lock().ok()?;
    let mut value = stats_cache_cell_ref(&cache, kind).value.as_ref()?.1.clone();
    value.age = value.sampled_at.map(|at| at.elapsed());
    Some(value)
}

const SOURCE_CURRENCY_TTL: Duration = Duration::from_secs(60);
const SOURCE_CURRENCY_FAILED_TTL: Duration = Duration::from_secs(5);

#[derive(Clone)]
pub(crate) struct SourceCurrencyCachedReadback {
    pub(crate) value: Result<serde_json::Value, String>,
    pub(crate) sampled_at: Option<Instant>,
    pub(crate) sample_id: Option<String>,
    pub(crate) age: Option<Duration>,
    pub(crate) stale: bool,
}

#[derive(Default)]
struct SourceCurrencyCache {
    value: Option<(Instant, SourceCurrencyCachedReadback)>,
    last_good: Option<SourceCurrencyCachedReadback>,
    loading: bool,
    generation: u64,
}

static SOURCE_CURRENCY_CACHE: OnceLock<(Mutex<SourceCurrencyCache>, std::sync::Condvar)> = OnceLock::new();

pub(crate) fn cached_source_currency(
    fetch: impl FnOnce() -> Result<serde_json::Value, String>,
) -> SourceCurrencyCachedReadback {
    let (lock, ready) = SOURCE_CURRENCY_CACHE.get_or_init(|| (Mutex::new(SourceCurrencyCache::default()), std::sync::Condvar::new()));
    let deadline = Instant::now() + Duration::from_secs(4);
    let mut fetch = Some(fetch);
    loop {
        let mut cache = lock.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        if let Some((observed_at, value)) = cache.value.as_ref() {
            let ttl = if value.stale { SOURCE_CURRENCY_FAILED_TTL } else { SOURCE_CURRENCY_TTL };
            if observed_at.elapsed() < ttl {
                let mut result = value.clone();
                result.age = result.sampled_at.map(|at| at.elapsed());
                let state = if result.stale { "stale" } else { "hit" };
                let sample_id = result.sample_id.clone();
                let age = result.age;
                drop(cache);
                crate::load_trace::record_cache_state("indicator.source-currency", state, sample_id, age);
                return result;
            }
        }
        if cache.loading {
            let Some(remaining) = deadline.checked_duration_since(Instant::now()) else {
                let result = failed_currency_observation(&cache, "source-currency-cache-wait-timeout");
                drop(cache);
                crate::load_trace::record_cache_state("indicator.source-currency", "timeout", result.sample_id.clone(), result.age);
                return result;
            };
            let (next, timeout) = ready.wait_timeout(cache, remaining).unwrap_or_else(|poisoned| poisoned.into_inner());
            cache = next;
            if timeout.timed_out() && cache.loading {
                let result = failed_currency_observation(&cache, "source-currency-cache-wait-timeout");
                drop(cache);
                crate::load_trace::record_cache_state("indicator.source-currency", "timeout", result.sample_id.clone(), result.age);
                return result;
            }
            drop(cache);
            continue;
        }
        let generation = cache.generation;
        cache.loading = true;
        let read = fetch.take().expect("one source-currency read per cache miss");
        drop(cache);
        let fetched = std::panic::catch_unwind(std::panic::AssertUnwindSafe(read));
        let observed_at = Instant::now();
        let mut cache = lock.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        let mut result = match fetched {
            Ok(Ok(value)) if value.get("ok").and_then(serde_json::Value::as_bool) == Some(false) => {
                let signal = value.get("firstMissingSignal").or_else(|| value.get("first_missing_signal"))
                    .and_then(serde_json::Value::as_str)
                    .filter(|signal| !signal.is_empty() && *signal != "none")
                    .unwrap_or("source-currency-not-ok");
                failed_currency_observation(&cache, signal)
            }
            Ok(Ok(value)) => SourceCurrencyCachedReadback {
                value: Ok(value), sampled_at: None, sample_id: None, age: None, stale: false,
            },
            Ok(Err(signal)) => failed_currency_observation(&cache, &signal),
            Err(_) => failed_currency_observation(&cache, "source-currency-collector-panicked"),
        };
        if cache.generation == generation {
            let next_generation = generation.wrapping_add(1);
            if result.value.is_ok() && !result.stale {
                result.sample_id = Some(format!("indicator.source-currency-{next_generation}"));
                result.sampled_at = Some(observed_at);
                result.age = Some(Duration::ZERO);
                cache.last_good = Some(result.clone());
            } else {
                result.age = result.sampled_at.map(|at| at.elapsed());
            }
            cache.generation = next_generation;
            cache.value = Some((observed_at, result.clone()));
        }
        cache.loading = false;
        ready.notify_all();
        let state = if result.stale { "stale" } else { "miss" };
        let sample_id = result.sample_id.clone();
        let age = result.age;
        drop(cache);
        crate::load_trace::record_cache_state("indicator.source-currency", state, sample_id, age);
        return result;
    }
}

fn failed_currency_observation(cache: &SourceCurrencyCache, signal: &str) -> SourceCurrencyCachedReadback {
    if let Some(last_good) = cache.last_good.as_ref() {
        let mut stale = last_good.clone();
        if let Ok(value) = &mut stale.value {
            if let Some(object) = value.as_object_mut() {
                object.insert("ok".to_owned(), serde_json::Value::Bool(false));
                object.insert("firstMissingSignal".to_owned(), serde_json::Value::String(signal.to_owned()));
            }
        }
        stale.stale = true;
        stale.age = stale.sampled_at.map(|at| at.elapsed());
        stale
    } else {
        SourceCurrencyCachedReadback {
            value: Err(signal.to_owned()), sampled_at: None,
            sample_id: None,
            age: None, stale: true,
        }
    }
}

fn invalidate_stats_cache_cell(kind: StatsReadCacheKind) {
    let (lock, ready) = stats_read_cache();
    let mut cache = lock.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    let cell = stats_cache_cell(&mut cache, kind);
    cell.value = None;
    cell.generation = cell.generation.wrapping_add(1);
    ready.notify_all();
}

pub(crate) fn invalidate_caduceus_stats_sample() {
    invalidate_stats_cache_cell(StatsReadCacheKind::Current);
}

pub(crate) fn caduceus_stats_sample() -> CaduceusStatsReadback {
    cached_caduceus_readback(StatsReadCacheKind::Current, STATS_SAMPLE_TTL, || {
        caduceus_http("GET", CADUCEUS_STATS_PATH)
    })
}

#[derive(Default)]
pub(crate) struct ScopedAdminStatsCache {
    state: Mutex<ScopedAdminStatsCacheState>,
    ready: std::sync::Condvar,
}

#[derive(Default)]
struct ScopedAdminStatsCacheState {
    value: Option<(Instant, CaduceusStatsReadback)>,
    last_good: Option<CaduceusStatsReadback>,
    loading: bool,
    generation: u64,
}

pub(crate) fn scoped_caduceus_admin_stats_sample(
    cache: &ScopedAdminStatsCache,
    fetch: impl FnOnce() -> CaduceusHttpReadback,
) -> CaduceusStatsReadback {
    let deadline = Instant::now() + Duration::from_secs(4);
    let mut fetch = Some(fetch);
    loop {
        let mut state = cache.state.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        if let Some((observed_at, value)) = state.value.as_ref() {
            if observed_at.elapsed() < STATS_SAMPLE_TTL {
                let mut result = value.clone();
                result.age = result.sampled_at.map(|at| at.elapsed());
                let sample_id = result.sample_id.clone();
                let age = result.age;
                let status = if result.stale { "stale" } else { "hit" };
                drop(state);
                crate::load_trace::record_cache_state("stats.admin", status, sample_id, age);
                return result;
            }
        }
        if state.loading {
            let Some(remaining) = deadline.checked_duration_since(Instant::now()) else {
                let result = failed_scoped_admin_observation(&state, "stats-admin-cache-wait-timeout");
                drop(state);
                crate::load_trace::record_cache_state("stats.admin", "timeout", result.sample_id.clone(), result.age);
                return result;
            };
            let (next, timeout) = cache.ready.wait_timeout(state, remaining).unwrap_or_else(|poisoned| poisoned.into_inner());
            state = next;
            if timeout.timed_out() && state.loading {
                let result = failed_scoped_admin_observation(&state, "stats-admin-cache-wait-timeout");
                drop(state);
                crate::load_trace::record_cache_state("stats.admin", "timeout", result.sample_id.clone(), result.age);
                return result;
            }
            drop(state);
            continue;
        }
        state.loading = true;
        let generation = state.generation;
        drop(state);
        let read = fetch.take().expect("one Caduceus admin read per scoped cache miss");
        let fetched: CaduceusStatsReadback = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(read)) {
            Ok(readback) => readback.into(),
            Err(panic) => {
                let mut state = cache.state.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
                state.loading = false;
                cache.ready.notify_all();
                drop(state);
                std::panic::resume_unwind(panic);
            }
        };
        let now = Instant::now();
        let mut state = cache.state.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        let mut result = fetched;
        if state.generation == generation {
            let next_generation = generation.wrapping_add(1);
            if result.ok {
                result.generation = next_generation;
                result.sampled_at = Some(now);
                result.sampled_at_unix_ms = Some(stats_cache_unix_millis());
                result.sample_id = Some(format!("stats.admin-{next_generation}"));
                result.age = Some(Duration::ZERO);
                result.stale = false;
                state.last_good = Some(result.clone());
            } else {
                result.stale = true;
                result.sample_id = None;
                result.sampled_at = None;
                result.sampled_at_unix_ms = None;
                result.age = None;
                result.generation = 0;
                if let Some(last_good) = state.last_good.as_ref() {
                    let mut stale = last_good.clone();
                    stale.ok = false;
                    stale.status = result.status;
                    stale.first_missing_signal = result.first_missing_signal.clone();
                    stale.stale = true;
                    stale.age = stale.sampled_at.map(|at| at.elapsed());
                    result = stale;
                }
            }
            state.value = Some((now, result.clone()));
            state.generation = next_generation;
        }
        state.loading = false;
        cache.ready.notify_all();
        let status = if result.stale { "stale" } else { "miss" };
        let sample_id = result.sample_id.clone();
        let age = result.age;
        drop(state);
        crate::load_trace::record_cache_state("stats.admin", status, sample_id, age);
        return result;
    }
}

fn failed_scoped_admin_observation(state: &ScopedAdminStatsCacheState, signal: &str) -> CaduceusStatsReadback {
    if let Some(last_good) = state.last_good.as_ref() {
        let mut stale = last_good.clone();
        stale.ok = false;
        stale.stale = true;
        stale.first_missing_signal = signal.to_owned();
        stale.age = stale.sampled_at.map(|at| at.elapsed());
        stale
    } else {
        CaduceusStatsReadback {
            ok: false,
            status: 503,
            body: serde_json::json!({"ok":false,"firstMissingSignal":signal}),
            first_missing_signal: signal.to_owned(),
            sampled_at: None,
            sampled_at_unix_ms: None,
            sample_id: None,
            generation: 0,
            stale: true,
            age: None,
        }
    }
}

pub(crate) fn completed_stats_history_peek() -> Option<CaduceusStatsReadback> {
    let (lock, _) = stats_read_cache();
    let cache = lock.try_lock().ok()?;
    stats_cache_cell_ref(&cache, StatsReadCacheKind::History).value.as_ref().map(|(_, result)| result.clone())
}

pub(crate) fn caduceus_stats_history() -> CaduceusStatsReadback {
    cached_caduceus_readback(StatsReadCacheKind::History, STATS_HISTORY_TTL, || {
        caduceus_http("GET", CADUCEUS_STATS_HISTORY_PATH)
    })
}

pub(crate) fn caduceus_stats_value() -> serde_json::Value {
    caduceus_stats_sample().body
}

static PORTAL_METADATA_CACHE: OnceLock<Mutex<Option<(Instant, Result<PortalConfigResponse, String>)>>> = OnceLock::new();

pub(crate) fn caduceus_indicator_portals() -> Result<PortalConfigResponse, String> {
    let cache = PORTAL_METADATA_CACHE.get_or_init(|| Mutex::new(None));
    if let Some((loaded_at, result)) = cache.lock().unwrap_or_else(|poisoned| poisoned.into_inner()).as_ref() {
        if loaded_at.elapsed() < HOUSEHOLD_METADATA_TTL {
            return result.clone();
        }
    }
    let started = Instant::now();
    let result = read_portals_config();
    let completed = Instant::now();
    let mut slot = cache.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    if slot.as_ref().is_none_or(|(loaded_at, _)| *loaded_at <= started) {
        *slot = Some((completed, result.clone()));
    }
    result
}

pub(crate) fn completed_indicator_portals_peek() -> Option<Result<PortalConfigResponse, String>> {
    let cache = PORTAL_METADATA_CACHE.get_or_init(|| Mutex::new(None));
    let slot = cache.try_lock().ok()?;
    slot.as_ref().map(|(_, result)| result.clone())
}

static HOUSEHOLD_CONFIG_CACHE: OnceLock<Mutex<Option<(Instant, Result<serde_json::Value, String>)>>> = OnceLock::new();

fn household_config_cached() -> Result<serde_json::Value, String> {
    let cache = HOUSEHOLD_CONFIG_CACHE.get_or_init(|| Mutex::new(None));
    if let Some((loaded_at, result)) = cache.lock().unwrap_or_else(|poisoned| poisoned.into_inner()).as_ref() {
        if loaded_at.elapsed() < HOUSEHOLD_METADATA_TTL {
            return result.clone();
        }
    }
    let started = Instant::now();
    let result = load_homeserver_json_sync().map(|(_, value)| value);
    let completed = Instant::now();
    let mut slot = cache.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    if slot.as_ref().is_none_or(|(loaded_at, _)| *loaded_at <= started) {
        *slot = Some((completed, result.clone()));
    }
    result
}

pub(crate) fn completed_household_config_peek() -> Option<Result<serde_json::Value, String>> {
    let cache = HOUSEHOLD_CONFIG_CACHE.get_or_init(|| Mutex::new(None));
    let slot = cache.try_lock().ok()?;
    slot.as_ref().map(|(_, result)| result.clone())
}

pub(crate) fn caduceus_power_calibration() -> (f64, Option<String>) {
    match household_config_cached() {
        Ok(config) => power_calibration_from_config(&config),
        Err(_) => (1.0_f64, Some("power-calibration-config-unavailable".to_owned())),
    }
}

pub(crate) fn completed_power_calibration_peek() -> (f64, Option<String>) {
    match completed_household_config_peek() {
        Some(Ok(config)) => power_calibration_from_config(&config),
        Some(Err(_)) => (1.0_f64, Some("power-calibration-config-unavailable".to_owned())),
        None => (1.0_f64, Some("power-calibration-config-not-observed".to_owned())),
    }
}

fn power_calibration_from_config(config: &serde_json::Value) -> (f64, Option<String>) {
    let Some(value) = config.pointer("/global/power/calibration") else {
        return (1.0_f64, None);
    };
    match value.as_f64().filter(|factor| factor.is_finite() && (0.1..=10.0).contains(factor)) {
        Some(factor) => (factor, None),
        None => (1.0_f64, Some("power-calibration-config-invalid".to_owned())),
    }
}

pub(crate) fn invalidate_indicator_household_config() {
    let cache = HOUSEHOLD_CONFIG_CACHE.get_or_init(|| Mutex::new(None));
    *cache.lock().unwrap_or_else(|poisoned| poisoned.into_inner()) = None;
}

pub(crate) fn invalidate_indicator_portal_metadata() {
    let cache = PORTAL_METADATA_CACHE.get_or_init(|| Mutex::new(None));
    *cache.lock().unwrap_or_else(|poisoned| poisoned.into_inner()) = None;
}

pub(crate) fn invalidate_indicator_metadata() {
    invalidate_indicator_household_config();
    invalidate_indicator_portal_metadata();
}
