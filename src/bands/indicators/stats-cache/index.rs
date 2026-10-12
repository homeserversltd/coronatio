const CADUCEUS_STATS_PATH: &str = "/api/v1/appliance/stats";
const CADUCEUS_STATS_ADMIN_PATH: &str = "/api/v1/appliance/stats/admin";
const CADUCEUS_STATS_HISTORY_PATH: &str = "/api/v1/appliance/stats/history?tier=raw&limit=60";
const STATS_SAMPLE_TTL: Duration = Duration::from_secs(1);
const STATS_HISTORY_TTL: Duration = Duration::from_secs(5);
const HOUSEHOLD_METADATA_TTL: Duration = Duration::from_secs(30);

#[derive(Clone)]
pub(crate) struct CaduceusStatsReadback {
    pub(crate) ok: bool,
    pub(crate) status: u16,
    pub(crate) body: serde_json::Value,
    pub(crate) first_missing_signal: String,
}

impl From<CaduceusHttpReadback> for CaduceusStatsReadback {
    fn from(readback: CaduceusHttpReadback) -> Self {
        Self {
            ok: readback.ok,
            status: readback.status,
            body: readback.body,
            first_missing_signal: readback.first_missing_signal,
        }
    }
}

#[derive(Default)]
struct StatsReadCacheCell {
    value: Option<(Instant, CaduceusHttpReadback)>,
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

fn cached_caduceus_readback(
    kind: StatsReadCacheKind,
    ttl: Duration,
    fetch: impl FnOnce() -> CaduceusHttpReadback,
) -> CaduceusHttpReadback {
    let (lock, ready) = stats_read_cache();
    let mut fetch = Some(fetch);
    let mut cache = lock.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    loop {
        let cell = stats_cache_cell(&mut cache, kind);
        if let Some((sampled_at, value)) = cell.value.as_ref() {
            if sampled_at.elapsed() < ttl {
                return value.clone();
            }
        }
        if cell.loading {
            cache = ready.wait(cache).unwrap_or_else(|poisoned| poisoned.into_inner());
            continue;
        }
        cell.loading = true;
        let generation = cell.generation;
        let read = fetch.take().expect("one Caduceus read per cache miss");
        drop(cache);
        let readback = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(read)) {
            Ok(readback) => readback,
            Err(panic) => {
                // Never leave waiters blocked on a fetch that will not finish.
                let mut cache = lock.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
                stats_cache_cell(&mut cache, kind).loading = false;
                ready.notify_all();
                drop(cache);
                std::panic::resume_unwind(panic);
            }
        };
        cache = lock.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        let cell = stats_cache_cell(&mut cache, kind);
        if cell.generation == generation {
            cell.value = Some((Instant::now(), readback.clone()));
        }
        cell.loading = false;
        ready.notify_all();
        return readback;
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
    .into()
}

#[derive(Default)]
pub(crate) struct ScopedAdminStatsCache {
    state: Mutex<ScopedAdminStatsCacheState>,
    ready: std::sync::Condvar,
}

#[derive(Default)]
struct ScopedAdminStatsCacheState {
    value: Option<(Instant, CaduceusStatsReadback)>,
    loading: bool,
}

pub(crate) fn scoped_caduceus_admin_stats_sample(
    cache: &ScopedAdminStatsCache,
    fetch: impl FnOnce() -> CaduceusHttpReadback,
) -> CaduceusStatsReadback {
    let mut fetch = Some(fetch);
    let mut state = cache
        .state
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    loop {
        if let Some((sampled_at, value)) = state.value.as_ref() {
            if sampled_at.elapsed() < STATS_SAMPLE_TTL {
                return value.clone();
            }
        }
        if state.loading {
            state = cache
                .ready
                .wait(state)
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            continue;
        }
        state.loading = true;
        drop(state);
        let read = fetch
            .take()
            .expect("one Caduceus admin read per scoped cache miss");
        let readback: CaduceusStatsReadback = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(read)) {
            Ok(readback) => readback.into(),
            Err(panic) => {
                let mut state = cache.state.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
                state.loading = false;
                cache.ready.notify_all();
                drop(state);
                std::panic::resume_unwind(panic);
            }
        };
        state = cache
            .state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        state.value = Some((Instant::now(), readback.clone()));
        state.loading = false;
        cache.ready.notify_all();
        return readback;
    }
}

pub(crate) fn caduceus_stats_history() -> CaduceusStatsReadback {
    cached_caduceus_readback(StatsReadCacheKind::History, STATS_HISTORY_TTL, || {
        caduceus_http("GET", CADUCEUS_STATS_HISTORY_PATH)
    })
    .into()
}

pub(crate) fn caduceus_stats_value() -> serde_json::Value {
    caduceus_stats_sample().body
}

static PORTAL_METADATA_CACHE: OnceLock<Mutex<Option<(Instant, Result<PortalConfigResponse, String>)>>> = OnceLock::new();

pub(crate) fn caduceus_indicator_portals() -> Result<PortalConfigResponse, String> {
    let cache = PORTAL_METADATA_CACHE.get_or_init(|| Mutex::new(None));
    let mut slot = cache.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    if let Some((loaded_at, result)) = slot.as_ref() {
        if loaded_at.elapsed() < HOUSEHOLD_METADATA_TTL {
            return result.clone();
        }
    }
    let result = read_portals_config();
    *slot = Some((Instant::now(), result.clone()));
    result
}

static HOUSEHOLD_CONFIG_CACHE: OnceLock<Mutex<Option<(Instant, Result<serde_json::Value, String>)>>> = OnceLock::new();

fn household_config_cached() -> Result<serde_json::Value, String> {
    let cache = HOUSEHOLD_CONFIG_CACHE.get_or_init(|| Mutex::new(None));
    let mut slot = cache.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    if let Some((loaded_at, result)) = slot.as_ref() {
        if loaded_at.elapsed() < HOUSEHOLD_METADATA_TTL {
            return result.clone();
        }
    }
    let result = load_homeserver_json_sync().map(|(_, value)| value);
    *slot = Some((Instant::now(), result.clone()));
    result
}

pub(crate) fn caduceus_power_calibration() -> (f64, Option<String>) {
    let default = 1.0_f64;
    let config = match household_config_cached() {
        Ok(value) => value,
        Err(_) => return (default, Some("power-calibration-config-unavailable".to_owned())),
    };
    let Some(value) = config.pointer("/global/power/calibration") else {
        return (default, None);
    };
    match value.as_f64().filter(|factor| factor.is_finite() && (0.1..=10.0).contains(factor)) {
        Some(factor) => (factor, None),
        None => (default, Some("power-calibration-config-invalid".to_owned())),
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
