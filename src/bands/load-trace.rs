use axum::{
    body::Body,
    extract::Request,
    http::{header, HeaderMap, HeaderName, HeaderValue, Method},
    middleware::Next,
    response::Response,
};
use base64::engine::general_purpose::{URL_SAFE, URL_SAFE_NO_PAD};
use base64::Engine as _;
use futures_util::{stream, StreamExt};
use serde_json::{json, Map, Value};
use std::{
    cell::RefCell,
    collections::BTreeMap,
    sync::{Arc, Mutex, OnceLock},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
use uuid::Uuid;

const CONTEXT_SCHEMA: &str = "appliance.load.context.v1";
const BENCHMARK_SCHEMA: &str = "caduceus.benchmark.v1";
const CORRELATION_HEADER: &str = "x-correlation-id";
const CONTEXT_HEADER: &str = "x-alkahest-context";
const MAX_CONTEXT_BYTES: usize = 16_384;
const MAX_CONTEXT_HEADER_BYTES: usize = 24 * 1024;
const MAX_CORRELATION_BYTES: usize = 256;
const MAX_STAMPS: usize = 64;
const MAX_STAMP_BYTES: usize = 2_048;
const MAX_BENCHMARK_ROWS_PER_STAMP: usize = 1;
const MAX_OBSERVED_RESPONSE_BYTES: usize = 1024 * 1024;
const MAX_CACHE_LINKS: usize = 8;

const DETAILED_STAGES: [&str; 7] = [
    "admission",
    "queue",
    "cache_wait",
    "upstream_io",
    "decode",
    "projection",
    "reply_preparation",
];

tokio::task_local! {
    static ACTIVE_TRACE: TraceScope;
}

thread_local! {
    // Set only for the duration of one explicitly captured blocking closure.
    static BLOCKING_TRACE: RefCell<Option<TraceScope>> = const { RefCell::new(None) };
}

#[derive(Clone)]
struct SchemaSeats {
    context_schema: Value,
    benchmark_schema: Value,
    context_bytes: usize,
    stamps: usize,
    stamp_bytes: usize,
    benchmark_rows: usize,
    context_required: Vec<String>,
    stamp_required: Vec<String>,
}

static SCHEMA_SEATS: OnceLock<tokio::sync::OnceCell<Result<Arc<SchemaSeats>, String>>> =
    OnceLock::new();

fn schema_cell() -> &'static tokio::sync::OnceCell<Result<Arc<SchemaSeats>, String>> {
    SCHEMA_SEATS.get_or_init(tokio::sync::OnceCell::new)
}

pub(crate) fn start_schema_initialization() {
    if let Ok(handle) = tokio::runtime::Handle::try_current() {
        handle.spawn(async { let _ = loaded_schema_seats().await; });
    }
}

async fn loaded_schema_seats() -> Result<Arc<SchemaSeats>, String> {
    schema_cell()
        .get_or_init(|| async {
            match tokio::task::spawn_blocking(read_schema_seats).await {
                Ok(result) => result.map(Arc::new),
                Err(_) => Err("load-trace-schema-reader-task-failed".to_owned()),
            }
        })
        .await
        .clone()
}

fn read_schema_seats() -> Result<SchemaSeats, String> {
    let context = super::caduceus_http("GET", "/api/v1/schema/appliance.load.context.v1");
    if !context.ok {
        return Err("load-trace-context-schema-unavailable".to_owned());
    }
    let benchmark = super::caduceus_http("GET", "/api/v1/schema/caduceus.benchmark.v1");
    if !benchmark.ok {
        return Err("load-trace-benchmark-schema-unavailable".to_owned());
    }
    let context_schema = context.body;
    let benchmark_schema = benchmark.body;
    if schema_id(&context_schema) != Some(CONTEXT_SCHEMA)
        || schema_id(&benchmark_schema) != Some(BENCHMARK_SCHEMA)
    {
        return Err("load-trace-schema-id-mismatch:expected-appliance.load.context.v1+caduceus.benchmark.v1".to_owned());
    }

    let context_required = required_fields(&context_schema, "required")
        .ok_or_else(|| "load-trace-context-schema-required-fields-missing".to_owned())?;
    let stamp_required = required_fields(&context_schema, "fields.stamps.items.required")
        .ok_or_else(|| "load-trace-context-stamp-required-fields-missing".to_owned())?;
    let benchmark_required = required_fields(&benchmark_schema, "required")
        .ok_or_else(|| "load-trace-benchmark-schema-required-fields-missing".to_owned())?;
    if !["schema", "correlation_id", "stamps"].iter().all(|name| context_required.iter().any(|field| field == name))
        || !["namespace", "hop", "span_id", "invocation_id", "operation", "received_at_utc", "returned_at_utc", "at_utc", "build_identity", "outcome", "road"].iter().all(|name| stamp_required.iter().any(|field| field == name))
        || !["schema", "timings"].iter().all(|name| benchmark_required.iter().any(|field| field == name))
    {
        return Err("load-trace-schema-required-fields-desynchronized".to_owned());
    }

    let budgets = context_schema.get("budgets").and_then(Value::as_object)
        .ok_or_else(|| "load-trace-context-schema-budgets-missing".to_owned())?;
    let context_bytes = budget(budgets, "encoded_context_bytes")?;
    let stamps = budget(budgets, "stamps")?;
    let stamp_bytes = budget(budgets, "serialized_stamp_bytes")?;
    let benchmark_rows = budget(budgets, "benchmark_rows_per_stamp")?;
    if context_bytes == 0 || stamps == 0 || stamp_bytes == 0 || benchmark_rows == 0 {
        return Err("load-trace-context-schema-budget-invalid:budgets-must-be-positive".to_owned());
    }
    Ok(SchemaSeats {
        context_schema,
        benchmark_schema,
        context_bytes,
        stamps,
        stamp_bytes,
        benchmark_rows,
        context_required,
        stamp_required,
    })
}

fn budget(budgets: &Map<String, Value>, name: &str) -> Result<usize, String> {
    budgets.get(name).and_then(Value::as_u64).and_then(|value| usize::try_from(value).ok())
        .ok_or_else(|| format!("load-trace-context-schema-budget-missing:{name}"))
}

fn required_fields(schema: &Value, path: &str) -> Option<Vec<String>> {
    let value = match path {
        "required" => schema.get("required"),
        "fields.stamps.items.required" => schema.pointer("/fields/stamps/items/required"),
        _ => None,
    }?;
    value.as_array()?.iter().map(Value::as_str).map(|field| field.map(str::to_owned)).collect()
}

fn schema_id(value: &Value) -> Option<&str> {
    value.get("schema").and_then(Value::as_str)
}

fn schema_loaded_now() -> Option<Arc<SchemaSeats>> {
    schema_cell().get().and_then(|result| result.as_ref().ok().cloned())
}

#[derive(Clone)]
pub(crate) struct TraceScope(Arc<Mutex<Invocation>>);

struct Invocation {
    active: bool,
    finalized: bool,
    revision: u64,
    correlation_id: Option<String>,
    echo_id: Option<HeaderValue>,
    context: Option<Value>,
    raw_context: Option<Value>,
    root_span_id: String,
    root_invocation_id: String,
    root_parent_span_id: Option<String>,
    received_at_utc: String,
    started: Instant,
    detail: bool,
    unavailable: Option<String>,
    incomplete: Option<String>,
    outcome: Option<String>,
    source_state: String,
    cache_state: String,
    stages: BTreeMap<String, StageRecord>,
    cache_links: Vec<Value>,
    seats: Option<Arc<SchemaSeats>>,
}

#[derive(Clone)]
struct StageRecord {
    elapsed: Option<Duration>,
    state: &'static str,
    signal: Option<String>,
}

struct FinalSnapshot {
    revision: u64,
    correlation_id: Option<String>,
    echo_id: Option<HeaderValue>,
    context: Option<Value>,
    raw_context: Option<Value>,
    root_span_id: String,
    root_invocation_id: String,
    root_parent_span_id: Option<String>,
    received_at_utc: String,
    started: Instant,
    detail: bool,
    unavailable: Option<String>,
    incomplete: Option<String>,
    outcome: Option<String>,
    source_state: String,
    cache_state: String,
    stages: BTreeMap<String, StageRecord>,
    cache_links: Vec<Value>,
    seats: Option<Arc<SchemaSeats>>,
}

struct TraceOutput {
    correlation: Option<HeaderValue>,
    context_header: Option<String>,
    body_context: Value,
}

impl TraceScope {
    fn new(headers: &HeaderMap, seats: Option<Arc<SchemaSeats>>, received: Instant) -> Option<Self> {
        let correlations = headers.get_all(CORRELATION_HEADER);
        let contexts = headers.get_all(CONTEXT_HEADER);
        let correlation_values: Vec<&HeaderValue> = correlations.iter().collect();
        let context_values: Vec<&HeaderValue> = contexts.iter().collect();
        let generated_id = (correlation_values.is_empty() && context_values.is_empty())
            .then(|| Uuid::new_v4().simple().to_string());

        let header_id = correlation_values.first().and_then(|value| valid_id(value.as_bytes()));
        let mut unavailable = None;
        if correlation_values.len() > 1 || context_values.len() > 1 {
            unavailable = Some("load-trace-header-duplicated".to_owned());
        }
        if !correlation_values.is_empty() && header_id.is_none() {
            unavailable.get_or_insert_with(|| "load-trace-correlation-id-invalid".to_owned());
        }
        if seats.is_none() {
            unavailable.get_or_insert_with(|| "load-trace-schema-seats-pending-or-unavailable".to_owned());
        }

        let decoded = context_values.first().map(|value| decode_context(value));
        let raw_context = decoded.as_ref().and_then(|result| result.as_ref().ok()).cloned();
        if let Some(Err(signal)) = decoded.as_ref() {
            unavailable.get_or_insert_with(|| (*signal).to_owned());
        }

        let mut context = None;
        let mut context_id = raw_context.as_ref()
            .and_then(|raw| raw.get("correlation_id"))
            .and_then(Value::as_str)
            .map(str::to_owned);
        if context_id.as_deref().is_some_and(|id| valid_id(id.as_bytes()).is_none()) {
            context_id = None;
            unavailable.get_or_insert_with(|| "load-trace-context-correlation-id-invalid".to_owned());
        }
        if let (Some(raw), Some(seats)) = (raw_context.as_ref(), seats.as_ref()) {
            if valid_context(raw, seats) {
                context = Some(raw.clone());
            } else {
                let signal = if schema_id(raw) != Some(CONTEXT_SCHEMA) {
                    "load-trace-context-schema-mismatch"
                } else {
                    "load-trace-context-shape-invalid"
                };
                unavailable.get_or_insert_with(|| signal.to_owned());
            }
        }
        if let (Some(header_id), Some(context_id)) = (header_id.as_deref(), context_id.as_deref()) {
            if header_id != context_id {
                unavailable.get_or_insert_with(|| "load-trace-correlation-id-mismatch".to_owned());
                context = None;
            }
        }
        if context_values.is_empty() && unavailable.is_none() {
            if let (Some(seats), Some(correlation_id)) = (seats.as_ref(), header_id.or(generated_id.as_deref())) {
                let schema = schema_id(&seats.context_schema).unwrap_or(CONTEXT_SCHEMA);
                context = Some(json!({"schema": schema, "correlation_id": correlation_id, "stamps": []}));
            }
        }
        if !context_values.is_empty() && raw_context.is_none() {
            context = None;
        }
        if unavailable.is_some() {
            context = None;
        }

        let correlation_id = match (header_id, context_id) {
            (Some(header_id), Some(context_id)) if header_id == context_id => Some(header_id.to_owned()),
            (Some(header_id), None) => Some(header_id.to_owned()),
            (None, Some(context_id)) => valid_id(context_id.as_bytes()).map(str::to_owned),
            _ => generated_id,
        };
        let echo_id = correlation_values.first().cloned().filter(|_| header_id.is_some()).cloned().or_else(|| {
            correlation_id.as_deref().and_then(|value| HeaderValue::from_bytes(value.as_bytes()).ok())
        });
        let root_parent_span_id = context.as_ref().and_then(context_parent);
        let detail = context.as_ref().and_then(|value| value.get("benchmark"))
            .and_then(Value::as_bool).unwrap_or(false);
        let root_id = Uuid::new_v4().simple().to_string();

        Some(Self(Arc::new(Mutex::new(Invocation {
            active: true,
            finalized: false,
            revision: 0,
            correlation_id,
            echo_id,
            context,
            raw_context,
            root_span_id: root_id.clone(),
            root_invocation_id: root_id,
            root_parent_span_id,
            received_at_utc: utc_now(),
            started: received,
            detail,
            unavailable,
            incomplete: None,
            outcome: None,
            source_state: "http-observation".to_owned(),
            cache_state: "not-observed".to_owned(),
            stages: BTreeMap::new(),
            cache_links: Vec::new(),
            seats,
        }))))
    }

    pub(crate) fn detail_requested(&self) -> bool {
        self.0.lock().unwrap_or_else(|error| error.into_inner()).detail
    }

    pub(crate) fn stage_start(&self) -> Option<Instant> {
        self.detail_requested().then(Instant::now)
    }

    pub(crate) fn stage_finish(&self, name: &str, started: Option<Instant>, state: &'static str, signal: Option<&str>) {
        let Some(started) = started else { return; };
        self.stage_elapsed(name, started.elapsed(), state, signal);
    }

    pub(crate) fn stage_elapsed(&self, name: &str, elapsed: Duration, state: &'static str, signal: Option<&str>) {
        let Ok(mut invocation) = self.0.lock() else { return; };
        if !invocation.active || !invocation.detail || !DETAILED_STAGES.contains(&name) {
            return;
        }
        let stage = invocation.stages.entry(name.to_owned()).or_insert(StageRecord {
            elapsed: None,
            state,
            signal: None,
        });
        stage.elapsed = Some(stage.elapsed.unwrap_or_default().saturating_add(elapsed));
        stage.state = state;
        if let Some(signal) = signal {
            stage.signal = Some(signal.to_owned());
        }
    }

    fn echo_id(&self) -> Option<HeaderValue> {
        self.0.lock().unwrap_or_else(|error| error.into_inner()).echo_id.clone()
    }

    fn mark_incomplete(&self, signal: &str) {
        if let Ok(mut invocation) = self.0.lock() {
            if invocation.active {
                invocation.incomplete.get_or_insert_with(|| signal.to_owned());
            }
        }
    }

    fn set_outcome(&self, outcome: &str) {
        if let Ok(mut invocation) = self.0.lock() {
            if invocation.active && matches!(outcome, "failed" | "timeout" | "cancelled") {
                invocation.outcome = Some(outcome.to_owned());
            }
        }
    }

    fn abandon(&self) {
        if let Ok(mut invocation) = self.0.lock() {
            invocation.active = false;
            invocation.finalized = true;
            invocation.revision = invocation.revision.wrapping_add(1);
        }
    }

    fn add_cache_link(&self, source: &str, state: &str, sample_id: Option<String>, age: Option<Duration>) {
        let Ok(mut invocation) = self.0.lock() else { return; };
        if !invocation.active {
            return;
        }
        invocation.source_state = bounded_label(source, 96);
        invocation.cache_state = bounded_label(state, 32);
        if invocation.cache_links.len() < MAX_CACHE_LINKS {
            let source_state = invocation.source_state.clone();
            let cache_state = invocation.cache_state.clone();
            invocation.cache_links.push(json!({
                "source": source_state,
                "state": cache_state,
                "sample_id": sample_id,
                "age_ms": age.map(duration_ms)
            }));
        } else if invocation.detail {
            invocation.incomplete.get_or_insert_with(|| "load-trace-cache-link-capacity".to_owned());
        }
    }

    fn finalize(&self, requested_outcome: &str) -> TraceOutput {
        let snapshot = {
            let Ok(mut invocation) = self.0.lock() else {
                return TraceOutput { correlation: None, context_header: None, body_context: diagnostic_context("load-trace-state-unavailable", None, None) };
            };
            if invocation.finalized {
                let saved = (
                    invocation.echo_id.clone(),
                    invocation.context.clone(),
                    invocation.unavailable.clone(),
                    invocation.incomplete.clone(),
                    invocation.correlation_id.clone(),
                    invocation.raw_context.clone(),
                    invocation.seats.clone(),
                );
                drop(invocation);
                return output_from_saved(saved);
            }
            invocation.active = false;
            invocation.finalized = true;
            invocation.revision = invocation.revision.wrapping_add(1);
            FinalSnapshot {
                revision: invocation.revision,
                correlation_id: invocation.correlation_id.clone(),
                echo_id: invocation.echo_id.clone(),
                context: invocation.context.clone(),
                raw_context: invocation.raw_context.clone(),
                root_span_id: invocation.root_span_id.clone(),
                root_invocation_id: invocation.root_invocation_id.clone(),
                root_parent_span_id: invocation.root_parent_span_id.clone(),
                received_at_utc: invocation.received_at_utc.clone(),
                started: invocation.started,
                detail: invocation.detail,
                unavailable: invocation.unavailable.clone(),
                incomplete: invocation.incomplete.clone(),
                outcome: invocation.outcome.clone(),
                source_state: invocation.source_state.clone(),
                cache_state: invocation.cache_state.clone(),
                stages: invocation.stages.clone(),
                cache_links: invocation.cache_links.clone(),
                seats: invocation.seats.clone(),
            }
        };

        let outcome = match requested_outcome {
            "ok" => snapshot.outcome.as_deref().unwrap_or("ok"),
            "failed" => snapshot.outcome.as_deref().filter(|value| matches!(*value, "timeout" | "cancelled")).unwrap_or("failed"),
            "timeout" => "timeout",
            "cancelled" => "cancelled",
            "not-traversed" => "not-traversed",
            _ => "failed",
        };
        let elapsed = snapshot.started.elapsed();
        let returned_at_utc = utc_now();
        let stamp = root_stamp(&snapshot, &returned_at_utc, elapsed, outcome);
        let mut candidate = snapshot.context.clone();
        let mut append_signal = None;
        if let (Some(context), Some(seats)) = (candidate.as_mut(), snapshot.seats.as_deref()) {
            match append_stamp(context, stamp, seats) {
                Ok(()) => {}
                Err(signal) => append_signal = Some(signal.to_owned()),
            }
        }
        let context_with_incomplete = if let (Some(context), Some(signal)) = (candidate.as_ref(), append_signal.as_deref().or(snapshot.incomplete.as_deref())) {
            add_incomplete_if_fits(context, signal, snapshot.seats.as_deref())
        } else {
            candidate.clone()
        };

        let mut body_context = if let Some(context) = context_with_incomplete.as_ref() {
            context.clone()
        } else {
            diagnostic_context(
                snapshot.unavailable.as_deref().or(append_signal.as_deref()).or(snapshot.incomplete.as_deref()).unwrap_or("load-trace-context-unavailable"),
                snapshot.correlation_id.as_deref(),
                candidate.as_ref().or(snapshot.raw_context.as_ref()),
            )
        };
        if body_context.is_null() {
            body_context = diagnostic_context("load-trace-context-unavailable", snapshot.correlation_id.as_deref(), snapshot.raw_context.as_ref());
        }

        let commit_candidate = context_with_incomplete.clone().or_else(|| candidate.clone());
        let (final_context, revision) = {
            let Ok(invocation) = self.0.lock() else {
                return TraceOutput { correlation: snapshot.echo_id, context_header: None, body_context };
            };
            (invocation.context.clone(), invocation.revision)
        };
        let mut accepted = final_context;
        if let (Some(updated), Some(seats)) = (commit_candidate, snapshot.seats.as_deref()) {
            if serialize_context(&updated, seats).is_some() {
                if let Ok(mut invocation) = self.0.lock() {
                    if invocation.revision == revision {
                        invocation.context = Some(updated.clone());
                        invocation.revision = invocation.revision.wrapping_add(1);
                        accepted = Some(updated);
                    }
                }
            }
        }
        if let Some(signal) = append_signal.as_deref() {
            if let Ok(mut invocation) = self.0.lock() {
                invocation.incomplete.get_or_insert_with(|| signal.to_owned());
            }
        }

        let context_header = accepted.as_ref().and_then(|context| {
            let seats = snapshot.seats.as_deref()?;
            let bytes = serialize_context(context, seats)?;
            Some(URL_SAFE_NO_PAD.encode(bytes))
        });
        if let Some(context) = accepted {
            if snapshot.incomplete.is_none() && append_signal.is_none() {
                body_context = context;
            } else if context_with_incomplete.as_ref().is_some_and(|diagnostic| diagnostic == &context) {
                body_context = context;
            }
        }
        TraceOutput { correlation: snapshot.echo_id, context_header, body_context }
    }
}

fn output_from_saved(saved: (Option<HeaderValue>, Option<Value>, Option<String>, Option<String>, Option<String>, Option<Value>, Option<Arc<SchemaSeats>>)) -> TraceOutput {
    let (correlation, context, unavailable, incomplete, correlation_id, raw_context, seats) = saved;
    let body_context = context.clone().unwrap_or_else(|| diagnostic_context(
        unavailable.as_deref().or(incomplete.as_deref()).unwrap_or("load-trace-context-unavailable"),
        correlation_id.as_deref(),
        raw_context.as_ref(),
    ));
    let context_header = context.as_ref().and_then(|context| {
        let seats = seats.as_deref()?;
        Some(URL_SAFE_NO_PAD.encode(serialize_context(context, seats)?))
    });
    TraceOutput { correlation, context_header, body_context }
}

fn root_stamp(snapshot: &FinalSnapshot, returned_at_utc: &str, elapsed: Duration, outcome: &str) -> Value {
    let mut stamp = json!({
        "namespace": "coronatio.load_trace.v1",
        "hop": "coronatio",
        "span_id": snapshot.root_span_id,
        "invocation_id": snapshot.root_invocation_id,
        "operation": "coronatio.http.get",
        "received_at_utc": snapshot.received_at_utc,
        "returned_at_utc": returned_at_utc,
        "at_utc": returned_at_utc,
        "build_identity": {
            "package_version": env!("CARGO_PKG_VERSION"),
            "build_sha": option_env!("CORONATIO_BUILD_SHA").unwrap_or("unset")
        },
        "outcome": outcome,
        "road": "axum-http",
        "source_state": snapshot.source_state,
        "cache_state": snapshot.cache_state,
        "benchmark": benchmark_report(snapshot, "coronatio.http.get", elapsed, outcome == "ok")
    });
    if let Some(parent) = snapshot.root_parent_span_id.as_deref() {
        stamp["parent_span_id"] = Value::String(parent.to_owned());
    }
    stamp
}

fn benchmark_report(snapshot: &FinalSnapshot, locus: &str, elapsed: Duration, ok: bool) -> Value {
    let mut row = json!({
        "locus": locus,
        "elapsed_ms": duration_ms_f64(elapsed),
        "ok": ok
    });
    let timing_rows = if snapshot.seats.as_ref().map_or(MAX_BENCHMARK_ROWS_PER_STAMP, |seats| seats.benchmark_rows) == 1 {
        json!([row])
    } else {
        json!([])
    };
    let mut report = json!({
        "schema": snapshot.seats.as_ref().and_then(|seats| schema_id(&seats.benchmark_schema)).unwrap_or(BENCHMARK_SCHEMA),
        "timings": timing_rows
    });
    if !snapshot.cache_links.is_empty() {
        report["cache_links"] = Value::Array(snapshot.cache_links.iter().take(3).cloned().collect());
    }
    if snapshot.detail {
        report["total_elapsed_ms"] = json!(duration_ms_f64(elapsed));
        report["detail_requested"] = Value::Bool(true);
        row["semantics"] = Value::String("inclusive".to_owned());
        let stages = DETAILED_STAGES.iter().map(|name| {
            let stage = snapshot.stages.get(*name);
            let elapsed = stage.and_then(|stage| stage.elapsed).map(duration_ms_f64);
            let mut item = json!({
                "locus": format!("coronatio.{name}"),
                "elapsed_ms": elapsed,
                "ok": stage.is_some_and(|stage| matches!(stage.state, "completed" | "ok"))
            });
            if let Some(stage) = stage {
                if let Some(signal) = stage.signal.as_deref() {
                    item["signal"] = Value::String(signal.to_owned());
                } else if stage.elapsed.is_none() {
                    item["signal"] = Value::String(stage.state.to_owned());
                }
                item["semantics"] = Value::String("exclusive".to_owned());
            } else {
                item["signal"] = Value::String("not-traversed".to_owned());
                item["semantics"] = Value::String("exclusive".to_owned());
            }
            item
        }).collect::<Vec<_>>();
        row["stages"] = Value::Array(stages);
        if !snapshot.cache_links.is_empty() {
            row["cache_links"] = Value::Array(snapshot.cache_links.clone());
        }
        report["timings"] = json!([row]);
    }
    report
}

fn append_stamp(context: &mut Value, stamp: Value, seats: &SchemaSeats) -> Result<(), &'static str> {
    if !valid_context(context, seats) {
        return Err("load-trace-context-shape-invalid");
    }
    if serialized_len(&stamp) > seats.stamp_bytes {
        return Err("load-trace-stamp-byte-capacity");
    }
    let Some(stamps) = context.get_mut("stamps").and_then(Value::as_array_mut) else {
        return Err("load-trace-context-stamps-invalid");
    };
    if stamps.len() >= seats.stamps {
        return Err("load-trace-context-stamp-capacity");
    }
    let mut candidate = context.clone();
    candidate.get_mut("stamps").and_then(Value::as_array_mut).ok_or("load-trace-context-stamps-invalid")?.push(stamp);
    if serialize_context(&candidate, seats).is_none() {
        return Err("load-trace-context-byte-capacity");
    }
    *context = candidate;
    Ok(())
}

fn add_incomplete_if_fits(context: &Value, signal: &str, seats: Option<&SchemaSeats>) -> Option<Value> {
    let seats = seats?;
    let mut candidate = context.clone();
    let object = candidate.as_object_mut()?;
    object.entry("trace_incomplete".to_owned()).or_insert_with(|| Value::String(signal.to_owned()));
    serialize_context(&candidate, seats).map(|_| candidate)
}

fn serialize_context(context: &Value, seats: &SchemaSeats) -> Option<Vec<u8>> {
    let bytes = serde_json::to_vec(context).ok()?;
    (bytes.len() <= seats.context_bytes).then_some(bytes)
}

fn valid_context(value: &Value, seats: &SchemaSeats) -> bool {
    if !value.is_object() || schema_id(value) != Some(CONTEXT_SCHEMA) {
        return false;
    }
    if !["schema", "correlation_id", "stamps"].iter().all(|field| value.get(field).is_some()) {
        return false;
    }
    let Some(id) = value.get("correlation_id").and_then(Value::as_str) else { return false; };
    if valid_id(id.as_bytes()).is_none() {
        return false;
    }
    let Some(stamps) = value.get("stamps").and_then(Value::as_array) else { return false; };
    if stamps.len() > seats.stamps || stamps.len() > MAX_STAMPS {
        return false;
    }
    stamps.iter().all(|stamp| {
        stamp.as_object().is_some_and(|object| {
            ["namespace", "hop", "span_id", "invocation_id", "operation", "received_at_utc", "returned_at_utc", "at_utc", "build_identity", "outcome", "road"].iter().all(|field| object.contains_key(*field))
                && serialized_len(stamp) <= seats.stamp_bytes
        })
    }) && serialize_context(value, seats).is_some()
}

fn valid_id(bytes: &[u8]) -> Option<&str> {
    let value = std::str::from_utf8(bytes).ok()?;
    if value.is_empty() || value.len() > MAX_CORRELATION_BYTES || HeaderValue::from_bytes(bytes).is_err() {
        return None;
    }
    Some(value)
}

fn context_parent(value: &Value) -> Option<String> {
    value.get("stamps").and_then(Value::as_array)
        .and_then(|stamps| stamps.iter().rev().find_map(|stamp| stamp.get("span_id")))
        .and_then(Value::as_str).map(str::to_owned)
        .or_else(|| value.get("parent_span_id").and_then(Value::as_str).map(str::to_owned))
}

fn decode_context(encoded: &HeaderValue) -> Result<Value, String> {
    let text = encoded.to_str().map_err(|_| "load-trace-context-header-not-utf8".to_owned())?;
    if text.len() > MAX_CONTEXT_HEADER_BYTES {
        return Err("load-trace-context-header-too-large".to_owned());
    }
    let mut bytes = vec![0u8; MAX_CONTEXT_BYTES];
    let decoded_len = match URL_SAFE_NO_PAD.decode_slice(text, &mut bytes) {
        Ok(len) => len,
        Err(_) => {
            bytes.fill(0);
            URL_SAFE.decode_slice(text, &mut bytes)
                .map_err(|_| "load-trace-context-base64url-invalid".to_owned())?
        }
    };
    let raw = &bytes[..decoded_len];
    let value = serde_json::from_slice(raw).map_err(|_| "load-trace-context-json-invalid".to_owned())?;
    Ok(value)
}

fn diagnostic_context(signal: &str, correlation_id: Option<&str>, raw: Option<&Value>) -> Value {
    let mut diagnostic = Map::new();
    diagnostic.insert("trace_unavailable".to_owned(), json!({
        "signal": signal,
        "expected_schema": CONTEXT_SCHEMA,
        "benchmark_schema": BENCHMARK_SCHEMA
    }));
    if let Some(correlation_id) = correlation_id {
        diagnostic.insert("correlation_id".to_owned(), Value::String(correlation_id.to_owned()));
    }
    if let Some(raw) = raw {
        diagnostic.insert("raw_context".to_owned(), raw.clone());
    }
    Value::Object(diagnostic)
}

fn serialized_len(value: &Value) -> usize {
    serde_json::to_vec(value).map(|bytes| bytes.len()).unwrap_or(usize::MAX)
}

fn duration_ms(duration: Duration) -> u64 {
    duration.as_millis().min(u64::MAX as u128) as u64
}

fn duration_ms_f64(duration: Duration) -> f64 {
    duration.as_secs_f64() * 1000.0
}

fn bounded_label(value: &str, limit: usize) -> String {
    value.chars().take(limit).collect()
}

fn utc_now() -> String {
    let elapsed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default();
    let seconds = elapsed.as_secs();
    let days = (seconds / 86_400) as i64;
    let day_seconds = seconds % 86_400;
    let z = days + 719_468;
    let era = z / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let mut year = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = mp + if mp < 10 { 3 } else { -9 };
    year += if month <= 2 { 1 } else { 0 };
    let hour = day_seconds / 3_600;
    let minute = (day_seconds % 3_600) / 60;
    let second = day_seconds % 60;
    let millis = elapsed.subsec_millis();
    format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}.{millis:03}Z")
}

#[derive(Clone)]
pub(crate) struct Dispatch {
    scope: Option<TraceScope>,
    sent_context: Option<Value>,
    correlation_id: Option<String>,
    header_lines: String,
}

impl Dispatch {
    pub(crate) fn header_lines(&self) -> &str {
        &self.header_lines
    }
}

pub(crate) fn begin_dispatch() -> Dispatch {
    let Some(scope) = current() else {
        return Dispatch { scope: None, sent_context: None, correlation_id: None, header_lines: String::new() };
    };
    let (correlation_id, seats, root_id, active) = {
        let Ok(invocation) = scope.0.lock() else {
            return Dispatch { scope: None, sent_context: None, correlation_id: None, header_lines: String::new() };
        };
        (invocation.correlation_id.clone(), invocation.seats.clone(), invocation.root_span_id.clone(), invocation.active)
    };
    let mut header_lines = String::new();
    if let Some(correlation_id) = correlation_id.as_deref() {
        header_lines.push_str(&format!("X-Correlation-Id: {correlation_id}\r\n"));
    }
    let Some(seats) = seats else {
        return Dispatch { scope: Some(scope), sent_context: None, correlation_id, header_lines };
    };
    if !active {
        return Dispatch { scope: Some(scope), sent_context: None, correlation_id, header_lines };
    }

    let prepare_started = Instant::now();
    let span_id = Uuid::new_v4().simple().to_string();
    let invocation_id = span_id.clone();
    let received_at = utc_now();
    let returned_at = utc_now();
    let mut stamp = json!({
        "namespace": "coronatio.load_trace.v1",
        "hop": "coronatio",
        "span_id": span_id,
        "invocation_id": invocation_id,
        "parent_span_id": root_id,
        "operation": "coronatio.caduceus.transport.prepare",
        "received_at_utc": received_at,
        "returned_at_utc": returned_at,
        "at_utc": returned_at,
        "build_identity": {
            "package_version": env!("CARGO_PKG_VERSION"),
            "build_sha": option_env!("CORONATIO_BUILD_SHA").unwrap_or("unset")
        },
        "outcome": "ok",
        "road": "caduceus-uds",
        "source_state": "upstream-dispatch",
        "cache_state": "not-observed",
        "benchmark": {"schema": BENCHMARK_SCHEMA, "timings": [{"locus":"coronatio.caduceus.transport.prepare","elapsed_ms":0.0,"ok":true}]}
    });
    let prepared = append_prepare(&scope, &mut stamp, &seats, prepare_started);
    if let Some(context) = prepared {
        if let Some(bytes) = serialize_context(&context, &seats) {
            header_lines.push_str(&format!("X-Alkahest-Context: {}\r\n", URL_SAFE_NO_PAD.encode(bytes)));
            return Dispatch { scope: Some(scope), sent_context: Some(context), correlation_id, header_lines };
        }
    }
    Dispatch { scope: Some(scope), sent_context: None, correlation_id, header_lines }
}

fn append_prepare(scope: &TraceScope, stamp: &mut Value, seats: &SchemaSeats, started: Instant) -> Option<Value> {
    loop {
        let (base, revision, active) = {
            let invocation = scope.0.lock().ok()?;
            (invocation.context.clone()?, invocation.revision, invocation.active)
        };
        if !active {
            return None;
        }
        let mut candidate = base.clone();
        if append_stamp(&mut candidate, stamp.clone(), seats).is_err() {
            scope.mark_incomplete("load-trace-prepare-stamp-capacity");
            return None;
        }
        let bytes = serialize_context(&candidate, seats)?;
        let _header_encoded = URL_SAFE_NO_PAD.encode(&bytes);
        let elapsed = started.elapsed();
        let returned_at = utc_now();
        stamp["returned_at_utc"] = Value::String(returned_at.clone());
        stamp["at_utc"] = Value::String(returned_at);
        if let Some(row) = stamp.pointer_mut("/benchmark/timings/0") {
            row["elapsed_ms"] = json!(duration_ms_f64(elapsed));
        }
        candidate = base;
        if append_stamp(&mut candidate, stamp.clone(), seats).is_err() {
            scope.mark_incomplete("load-trace-prepare-stamp-capacity");
            return None;
        }
        let final_bytes = serialize_context(&candidate, seats)?;
        let _final_header_encoded = URL_SAFE_NO_PAD.encode(&final_bytes);
        let Ok(mut invocation) = scope.0.lock() else { return None; };
        if !invocation.active {
            return None;
        }
        if invocation.revision != revision {
            continue;
        }
        invocation.context = Some(candidate.clone());
        invocation.revision = invocation.revision.wrapping_add(1);
        return Some(candidate);
    }
}

fn valid_returned_context(returned: &Value, sent: &Value, seats: &SchemaSeats) -> bool {
    if !valid_context(returned, seats)
        || returned.get("correlation_id") != sent.get("correlation_id")
    {
        return false;
    }
    let (Some(sent_object), Some(returned_object)) = (sent.as_object(), returned.as_object()) else { return false; };
    for (key, value) in sent_object {
        if key == "stamps" || key == "next_hop_ancestry" {
            continue;
        }
        if returned_object.get(key) != Some(value) {
            return false;
        }
    }
    if let Some(sent_ancestry) = sent.get("next_hop_ancestry") {
        let Some(returned_ancestry) = returned.get("next_hop_ancestry").and_then(Value::as_array) else { return false; };
        let Some(sent_ancestry) = sent_ancestry.as_array() else { return false; };
        if returned_ancestry.len() < sent_ancestry.len()
            || returned_ancestry.iter().zip(sent_ancestry).any(|(left, right)| left != right)
        {
            return false;
        }
    }
    let sent_stamps = sent.get("stamps").and_then(Value::as_array);
    let returned_stamps = returned.get("stamps").and_then(Value::as_array);
    matches!((sent_stamps, returned_stamps), (Some(sent), Some(returned))
        if returned.len() >= sent.len() && sent.iter().zip(returned).all(|(left, right)| left == right))
}

fn merge_returned_context(scope: &TraceScope, sent: &Value, returned: &Value, seats: &SchemaSeats) {
    if !valid_returned_context(returned, sent, seats) {
        scope.mark_incomplete("load-trace-upstream-prefix-or-context-mismatch");
        return;
    }
    let returned_stamps = returned.get("stamps").and_then(Value::as_array).cloned().unwrap_or_default();
    let sent_count = sent.get("stamps").and_then(Value::as_array).map_or(0, Vec::len);
    loop {
        let (base, revision, active) = {
            let Ok(invocation) = scope.0.lock() else { return; };
            (invocation.context.clone(), invocation.revision, invocation.active)
        };
        if !active {
            return;
        }
        let Some(mut candidate) = base else {
            scope.mark_incomplete("load-trace-local-context-unavailable");
            return;
        };
        if !merge_non_stamp_fields(&mut candidate, returned) {
            scope.mark_incomplete("load-trace-upstream-context-field-mismatch");
            return;
        }
        let Some(current_stamps) = candidate.get_mut("stamps").and_then(Value::as_array_mut) else {
            scope.mark_incomplete("load-trace-local-stamps-invalid");
            return;
        };
        let mut mismatch = false;
        for stamp in returned_stamps.iter().skip(sent_count) {
            let id = stamp.get("span_id").and_then(Value::as_str);
            if let Some(id) = id {
                if let Some(existing) = current_stamps.iter().find(|existing| existing.get("span_id").and_then(Value::as_str) == Some(id)) {
                    if existing != stamp { mismatch = true; break; }
                    continue;
                }
            }
            current_stamps.push(stamp.clone());
        }
        if mismatch {
            scope.mark_incomplete("load-trace-upstream-span-id-collision");
            return;
        }
        if candidate.get("stamps").and_then(Value::as_array).is_none_or(|stamps| stamps.len() > seats.stamps)
            || candidate.get("stamps").and_then(Value::as_array).is_none_or(|stamps| stamps.iter().any(|stamp| serialized_len(stamp) > seats.stamp_bytes))
            || serialize_context(&candidate, seats).is_none()
        {
            scope.mark_incomplete("load-trace-upstream-context-capacity");
            return;
        }
        let Ok(mut invocation) = scope.0.lock() else { return; };
        if !invocation.active {
            return;
        }
        if invocation.revision != revision {
            continue;
        }
        invocation.context = Some(candidate);
        invocation.revision = invocation.revision.wrapping_add(1);
        return;
    }
}

fn merge_non_stamp_fields(target: &mut Value, returned: &Value) -> bool {
    let (Some(target), Some(returned)) = (target.as_object_mut(), returned.as_object()) else { return false; };
    for (key, value) in returned {
        if key == "stamps" { continue; }
        if key == "next_hop_ancestry" {
            let Some(returned_links) = value.as_array() else { return false; };
            let links = target.entry(key.clone()).or_insert_with(|| Value::Array(Vec::new()));
            let Some(target_links) = links.as_array_mut() else { return false; };
            for link in returned_links {
                if !target_links.contains(link) {
                    target_links.push(link.clone());
                }
            }
            continue;
        }
        match target.get(key) {
            Some(existing) if existing == value => {}
            Some(_) => return false,
            None => { target.insert(key.clone(), value.clone()); }
        }
    }
    true
}

#[derive(Default)]
pub(crate) struct ProviderHeaders {
    pub(crate) correlation: Option<String>,
    pub(crate) context: Option<Value>,
    pub(crate) signal: Option<String>,
}

pub(crate) fn parse_provider_headers(headers: &str) -> ProviderHeaders {
    let mut result = ProviderHeaders::default();
    let mut correlation_seen = false;
    let mut context_seen = false;
    for line in headers.lines() {
        let Some((name, value)) = line.split_once(':') else { continue; };
        if name.trim().eq_ignore_ascii_case(CORRELATION_HEADER) {
            if correlation_seen {
                result.signal.get_or_insert_with(|| "load-trace-provider-correlation-duplicated".to_owned());
                continue;
            }
            correlation_seen = true;
            let value = value.trim();
            if valid_id(value.as_bytes()).is_some() {
                result.correlation = Some(value.to_owned());
            } else {
                result.signal.get_or_insert_with(|| "load-trace-provider-correlation-invalid".to_owned());
            }
        } else if name.trim().eq_ignore_ascii_case(CONTEXT_HEADER) {
            if context_seen {
                result.signal.get_or_insert_with(|| "load-trace-provider-context-duplicated".to_owned());
                continue;
            }
            context_seen = true;
            let value = value.trim();
            if value.len() > MAX_CONTEXT_HEADER_BYTES {
                result.signal.get_or_insert_with(|| "load-trace-provider-context-header-too-large".to_owned());
                continue;
            }
            match decode_context(&HeaderValue::from_str(value).unwrap_or_else(|_| HeaderValue::from_static(""))) {
                Ok(context) => result.context = Some(context),
                Err(signal) => { result.signal.get_or_insert(signal); }
            }
        }
    }
    result
}

pub(crate) fn capture_provider_body(mut body: Value) -> (Value, Option<Value>) {
    let mut context = None;
    let mut unwrapped_result = None;
    if let Some(object) = body.as_object_mut() {
        if object.get("load_context").is_some_and(Value::is_object) {
            context = object.remove("load_context");
        }
        let canonical_wrapper = context.is_some() && object.len() == 1 && object.contains_key("result");
        if canonical_wrapper {
            unwrapped_result = object.remove("result");
        }
    }
    if let Some(result) = unwrapped_result {
        body = result;
    }
    (body, context)
}

pub(crate) fn record_provider_response(
    dispatch: &Dispatch,
    provider: ProviderHeaders,
    body_context: Option<Value>,
    io: Duration,
    decode: Option<Duration>,
    success: bool,
    signal: Option<&str>,
) {
    if let Some(scope) = dispatch.scope.as_ref() {
        scope.stage_elapsed("upstream_io", io, if success { "completed" } else if signal.is_some_and(|value| value.contains("timeout")) { "timeout" } else { "failed" }, signal);
        if let Some(decode) = decode {
            scope.stage_elapsed("decode", decode, if success { "completed" } else { "failed" }, signal);
        }
        if let Some(provider_signal) = provider.signal.as_deref() {
            scope.mark_incomplete(provider_signal);
        }
        let expected_id = dispatch.correlation_id.as_deref();
        let response_id_ok = provider.correlation.as_deref().is_none_or(|id| Some(id) == expected_id);
        if !response_id_ok {
            scope.mark_incomplete("load-trace-provider-correlation-mismatch");
        }
        let context = match (body_context, provider.context) {
            (Some(body), Some(header)) if body == header => Some(body),
            (Some(body), Some(_)) => {
                scope.mark_incomplete("load-trace-provider-body-header-context-mismatch");
                Some(body)
            }
            (Some(body), None) => Some(body),
            (None, Some(header)) => Some(header),
            (None, None) => None,
        };
        if success && dispatch.sent_context.is_some() && context.is_none() {
            scope.mark_incomplete("load-trace-provider-context-returned-none");
        }
        if response_id_ok {
            if let Some(returned) = context {
                match (dispatch.sent_context.as_ref(), dispatch.scope.as_ref()) {
                    (Some(sent), Some(scope)) => {
                        let seats = scope.0.lock().ok().and_then(|invocation| invocation.seats.clone());
                        if let Some(seats) = seats {
                            let context_id = returned.get("correlation_id").and_then(Value::as_str);
                            if context_id != expected_id {
                                scope.mark_incomplete("load-trace-provider-context-correlation-mismatch");
                            } else {
                                merge_returned_context(scope, sent, &returned, &seats);
                            }
                        }
                    }
                    _ => scope.mark_incomplete("load-trace-provider-context-without-dispatch-prefix"),
                }
            }
        }
        if !success {
            scope.set_outcome(if signal.is_some_and(|value| value.contains("timeout")) { "timeout" } else { "failed" });
        }
    }
}

pub(crate) fn record_dispatch_failure(dispatch: &Dispatch, io: Duration, signal: &str) {
    if let Some(scope) = dispatch.scope.as_ref() {
        let state = if signal.contains("timeout") { "timeout" } else { "failed" };
        scope.stage_elapsed("upstream_io", io, state, Some(signal));
        scope.set_outcome(state);
    }
}

pub(crate) fn record_transport_failure(_path: &str, io: Duration, signal: &str) {
    if let Some(scope) = current() {
        let state = if signal.contains("timeout") { "timeout" } else { "failed" };
        scope.stage_elapsed("upstream_io", io, state, Some(signal));
        scope.set_outcome(state);
    }
}

pub(crate) fn record_stage(name: &str, elapsed: Duration, state: &'static str, signal: Option<&str>) {
    if let Some(scope) = current() {
        scope.stage_elapsed(name, elapsed, state, signal);
    }
}

pub(crate) fn record_read_failure(signal: &str) {
    if let Some(scope) = current() {
        scope.set_outcome(if signal.contains("timeout") { "timeout" } else { "failed" });
    }
}

pub(crate) fn record_cache_wait(started: Instant, state: &'static str, signal: Option<&str>) {
    record_stage("cache_wait", started.elapsed(), state, signal);
}

pub(crate) fn record_cache_link(source: &str, sample_id: Option<String>, age: Option<Duration>) {
    if let Some(scope) = current() {
        scope.add_cache_link(source, "linked", sample_id, age);
    }
}

pub(crate) fn record_cache_state(source: &str, state: &str, sample_id: Option<String>, age: Option<Duration>) {
    if let Some(scope) = current() {
        scope.add_cache_link(source, state, sample_id, age);
    }
}

// Retained for existing callers; queue time is measured at spawn_blocking entry.
pub(crate) fn record_queue_wait(_enqueued: Instant) {}

pub(crate) fn current() -> Option<TraceScope> {
    ACTIVE_TRACE.try_with(Clone::clone).ok()
        .or_else(|| BLOCKING_TRACE.with(|scope| scope.borrow().clone()))
}

pub(crate) fn capture_current() -> Option<TraceScope> {
    current()
}

pub(crate) async fn in_scope<F: std::future::Future>(scope: TraceScope, future: F) -> F::Output {
    ACTIVE_TRACE.scope(scope, future).await
}

pub(crate) async fn spawn_blocking_with_trace<T: Send + 'static>(
    work: impl FnOnce() -> T + Send + 'static,
) -> Result<T, tokio::task::JoinError> {
    let scope = capture_current();
    let enqueued = scope.as_ref().filter(|scope| scope.detail_requested()).map(|_| Instant::now());
    tokio::task::spawn_blocking(move || {
        with_blocking_context(scope, || {
            if let (Some(scope), Some(enqueued)) = (current(), enqueued) {
                scope.stage_elapsed("queue", enqueued.elapsed(), "completed", None);
            }
            work()
        })
    }).await
}

pub(crate) fn with_blocking_context<T>(scope: Option<TraceScope>, work: impl FnOnce() -> T) -> T {
    struct Restore(Option<TraceScope>);
    impl Drop for Restore {
        fn drop(&mut self) {
            BLOCKING_TRACE.with(|slot| *slot.borrow_mut() = self.0.take());
        }
    }
    let previous = BLOCKING_TRACE.with(|slot| std::mem::replace(&mut *slot.borrow_mut(), scope));
    let _restore = Restore(previous);
    work()
}

fn known_json_observation_path(path: &str) -> bool {
    matches!(path,
        "/api/caduceus/status"
        | "/api/caduceus/update/service/status"
        | "/api/v1/exousia/agent/posture"
        | "/api/stats"
        | "/api/stats/current"
        | "/api/stats/roster"
        | "/api/stats/history"

        | "/api/indicators/admin"
        | "/api/status"
        | "/api/status/services"
        | "/api/status/tailscale"
        | "/api/status/vpn/pia"
        | "/api/status/vpn/transmission"
        | "/api/status/tailscale/config"
        | "/api/status/power/usage"
        | "/status/power/usage"
        | "/api/status/power/calibration"
        | "/api/uptime"
    )
}

fn is_json_content_type(headers: &HeaderMap) -> bool {
    let Some(value) = headers.get(header::CONTENT_TYPE).and_then(|value| value.to_str().ok()) else { return false; };
    let media_type = value.split(';').next().unwrap_or_default().trim().to_ascii_lowercase();
    media_type == "application/json" || media_type.ends_with("+json")
}

fn restore_body<S>(prefix: Vec<u8>, remainder: S) -> Body
where
    S: futures_util::Stream<Item = Result<axum::body::Bytes, axum::Error>> + Send + 'static,
{
    let first = stream::once(async move { Ok::<Vec<u8>, axum::Error>(prefix) });
    let rest = remainder.map(|item| item.map(|bytes| bytes.to_vec()));
    Body::from_stream(first.chain(rest))
}

async fn collect_bounded_preserving(body: Body, limit: usize) -> Result<Vec<u8>, Body> {
    let mut stream = body.into_data_stream();
    let mut collected = Vec::new();
    while let Some(item) = stream.next().await {
        match item {
            Ok(bytes) => {
                if collected.len().saturating_add(bytes.len()) > limit {
                    collected.extend_from_slice(&bytes);
                    return Err(restore_body(collected, stream));
                }
                collected.extend_from_slice(&bytes);
            }
            Err(error) => {
                let prefix = collected;
                let first_error = stream::once(async move { Err::<axum::body::Bytes, axum::Error>(error) });
                return Err(restore_body(prefix, first_error.chain(stream)));
            }
        }
    }
    Ok(collected)
}

fn enrich_response_json(value: &mut Value, load_context: Value, seats: Option<&SchemaSeats>) {
    let Some(object) = value.as_object_mut() else {
        let business = std::mem::take(value);
        *value = json!({"result": business, "load_context": load_context});
        return;
    };
    if !object.contains_key("load_context") {
        object.insert("load_context".to_owned(), load_context);
        return;
    }
    let compatible = object.get("load_context").is_some_and(|existing| {
        seats.is_some_and(|seats| valid_context(existing, seats))
            && existing.get("correlation_id") == load_context.get("correlation_id")
    });
    if compatible {
        let merged = object.get("load_context").cloned().and_then(|mut merged| {
            if !merge_non_stamp_fields(&mut merged, &load_context) {
                return None;
            }
            let source_stamps = load_context.get("stamps").and_then(Value::as_array)?;
            let target_stamps = merged.get_mut("stamps").and_then(Value::as_array_mut)?;
            for stamp in source_stamps {
                if !target_stamps.contains(stamp) {
                    target_stamps.push(stamp.clone());
                }
            }
            Some(merged)
        });
        if let (Some(merged), Some(seats)) = (merged, seats) {
            if valid_context(&merged, seats) {
                object.insert("load_context".to_owned(), merged);
                return;
            }
        }
    }
    object.entry("caduceus_load_context".to_owned()).or_insert(load_context);
}

pub(crate) async fn load_trace_middleware_fn(request: Request, next: Next) -> Response {
    if request.method() != Method::GET || !known_json_observation_path(request.uri().path()) {
        return next.run(request).await;
    }
    let received = Instant::now();
    let headers = request.headers().clone();
    let seats = schema_loaded_now();
    let Some(scope) = TraceScope::new(&headers, seats.clone(), received) else {
        return next.run(request).await;
    };
    let _completion = TraceCompletionGuard(scope.clone());
    let mut response = in_scope(scope.clone(), next.run(request)).await;
    if let Some(correlation) = scope.echo_id() {
        response.headers_mut().insert(CORRELATION_HEADER, correlation);
    }
    if !is_json_content_type(response.headers()) {
        scope.abandon();
        return response;
    }

    let reply_started = scope.stage_start();
    let (parts, body) = response.into_parts();
    let bytes = match collect_bounded_preserving(body, MAX_OBSERVED_RESPONSE_BYTES).await {
        Ok(bytes) => bytes,
        Err(body) => {
            scope.mark_incomplete("load-trace-response-body-unreadable-or-over-limit");
            let outcome = scope.0.lock().ok().and_then(|invocation| invocation.outcome.clone()).unwrap_or_else(|| if parts.status.is_success() { "ok".to_owned() } else { "failed".to_owned() });
            let output = scope.finalize(&outcome);
            let mut response = Response::from_parts(parts, body);
            if let Some(correlation) = output.correlation {
                response.headers_mut().insert(CORRELATION_HEADER, correlation);
            }
            if let Some(context) = output.context_header.and_then(|value| HeaderValue::from_str(&value).ok()) {
                response.headers_mut().insert(CONTEXT_HEADER, context);
            }
            return response;
        }
    };
    let mut business = match serde_json::from_slice::<Value>(&bytes) {
        Ok(value) => value,
        Err(_) => {
            scope.mark_incomplete("load-trace-response-json-invalid");
            let outcome = scope.0.lock().ok().and_then(|invocation| invocation.outcome.clone()).unwrap_or_else(|| if parts.status.is_success() { "ok".to_owned() } else { "failed".to_owned() });
            let output = scope.finalize(&outcome);
            let mut response = Response::from_parts(parts, Body::from(bytes));
            if let Some(correlation) = output.correlation {
                response.headers_mut().insert(CORRELATION_HEADER, correlation);
            }
            if let Some(context) = output.context_header.and_then(|value| HeaderValue::from_str(&value).ok()) {
                response.headers_mut().insert(CONTEXT_HEADER, context);
            }
            return response;
        }
    };
    scope.stage_finish("reply_preparation", reply_started, "completed", None);
    let status_outcome = if parts.status.is_success() {
        "ok"
    } else if scope.0.lock().ok().and_then(|invocation| invocation.outcome.clone()).as_deref() == Some("timeout") {
        "timeout"
    } else {
        "failed"
    };
    let output = scope.finalize(status_outcome);
    enrich_response_json(&mut business, output.body_context.clone(), seats.as_deref());
    let serialized = match serde_json::to_vec(&business) {
        Ok(bytes) => bytes,
        Err(_) => {
            let mut response = Response::from_parts(parts, Body::from(bytes));
            if let Some(correlation) = output.correlation {
                response.headers_mut().insert(CORRELATION_HEADER, correlation);
            }
            return response;
        }
    };
    let mut response = Response::from_parts(parts, Body::from(serialized.clone()));
    if let Some(correlation) = output.correlation {
        response.headers_mut().insert(CORRELATION_HEADER, correlation);
    }
    if let Some(context) = output.context_header.and_then(|value| HeaderValue::from_str(&value).ok()) {
        response.headers_mut().insert(CONTEXT_HEADER, context);
    }
    if let Ok(length) = HeaderValue::from_str(&serialized.len().to_string()) {
        response.headers_mut().insert(header::CONTENT_LENGTH, length);
    } else {
        response.headers_mut().remove(header::CONTENT_LENGTH);
    }
    response
}

pub(crate) fn trace_header_names() -> (HeaderName, HeaderName) {
    (HeaderName::from_static(CORRELATION_HEADER), HeaderName::from_static(CONTEXT_HEADER))
}

pub(crate) fn malformed_context_signal(scope: &TraceScope) -> Option<String> {
    scope.0.lock().unwrap_or_else(|error| error.into_inner()).unavailable.clone()
}

struct TraceCompletionGuard(TraceScope);

impl Drop for TraceCompletionGuard {
    fn drop(&mut self) {
        let _ = self.0.finalize("cancelled");
    }
}
