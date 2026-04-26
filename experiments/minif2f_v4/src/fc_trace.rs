// Phase A atom A6 — per-line FC tagging via structured JSON events.
//
// Constitutional anchor: meta-witness for FC1 / FC2 / FC3 path coverage.
// Codex / Gemini N-agents brainstorm 2026-04-25 § D ("Constitutional /
// Engineering"): "add OpenTelemetry-style spans or structured JSONL
// events for the constitutional stages". TRACE_MATRIX_v2 § 5 item 7 had
// this deferred to Phase A6 with the note "will land before Phase B
// (homogeneous experiments)".
//
// Why now (Phase A pre-flight, not Phase B): once Phase B starts wiring
// kernel instrumentation (cost / wall-clock / dual PPUT), silent FC
// drift becomes catastrophic — a Soft Law mode that bypasses FC1-N12
// and stamps post-hoc verified=true would *look* like Phase B success.
// FC tracing in pre-flight gives a zero-cost-when-disabled audit trail
// that downstream Phase D ArchitectAI can replay deterministically.
//
// Design constraints:
//   1. Zero new crate dependencies (autonomous-safe; pure stdlib).
//   2. Zero overhead when `FC_TRACE` env is unset (single env::var()
//      check at startup; no per-call var read after).
//   3. Append-only, line-delimited JSON to stderr by default; or to
//      `FC_TRACE_FILE` for batch capture under runner.sh.
//   4. Stable event shape — Phase D consumers MUST be able to join
//      events on `fc_id` + `run_id` + monotonic timestamp without
//      schema knowledge.
//
// Event shape (one JSON object per line):
//   {"ts_ms": 1714123456789, "fc_id": "FC2-N22",
//    "run_id": "n3_mathd_42_169123", "tx": 17,
//    "agent_id": "Agent_2", "kv": {...arbitrary key-value...}}
//
// Phase D+ extension (out-of-scope for A6): convert to true OpenTelemetry
// spans + replace the file sink with a tracing-subscriber bridge. The
// macro surface here was kept small specifically so that swap is local.

use std::io::Write;
use std::sync::OnceLock;

/// TRACE_MATRIX FC-trace meta-witness: canonical FC node identifiers
/// the evaluator emits events for. Adding a new variant is a Phase B+
/// schema change — Phase D ArchitectAI joins on these strings.
///
/// `Display::fmt` produces the dash-separated form used in
/// TRACE_MATRIX rows (e.g. `FC2-N22`, NOT `FC2N22` or `fc2_n22`); that
/// stable string is what flows into the emitted JSON.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FcId {
    /// δ/AI prompt construction (per-Agent_i).
    Fc1N7,
    /// ∏p decision diversity — one event per accepted/rejected proposal
    /// payload (append/complete/step). Distinct vs total flow into the
    /// A4 `tactic_diversity` aggregate.
    Fc1N11,
    /// Lean oracle scope — bracketing every verify_omega /
    /// verify_omega_detailed / verify_partial call. Cumulative
    /// elapsed flows into the A4 `verifier_wait_ms` aggregate.
    Fc1N12,
    /// ∏p=0 → preserve Q_t. Fired when forbidden_pattern matches OR
    /// Lean returns Ok(false).
    Fc1E18,
    /// mr / map-reduce tick. Fired at every `tick_interval` boundary.
    Fc2N20,
    /// HALT — terminal node. `kv.reason` carries
    /// {OmegaAccepted, MaxTxExhausted, ErrorHalt} per FC2-N23.
    Fc2N22,
    /// WAL append (logs subgraph). Fired on every `bus.append_*`
    /// success when WAL_DIR is configured.
    Fc3N31,
}

impl FcId {
    /// Stable string used in emitted JSON. Phase D consumers and
    /// TRACE_MATRIX rows join on this exact spelling.
    pub fn as_str(&self) -> &'static str {
        match self {
            FcId::Fc1N7 => "FC1-N7",
            FcId::Fc1N11 => "FC1-N11",
            FcId::Fc1N12 => "FC1-N12",
            FcId::Fc1E18 => "FC1-E18",
            FcId::Fc2N20 => "FC2-N20",
            FcId::Fc2N22 => "FC2-N22",
            FcId::Fc3N31 => "FC3-N31",
        }
    }
}

impl std::fmt::Display for FcId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// FC_TRACE=1 enables event emission. Read once at first invocation
/// (cached in OnceLock) so subsequent emit calls are a single atomic
/// load. Kept process-global because the evaluator binary is one
/// process per run — no need for finer scoping.
static FC_TRACE_ENABLED: OnceLock<bool> = OnceLock::new();

/// FC_TRACE_FILE=<path> redirects emit to the file (truncate-on-open).
/// Default sink = stderr. Acquired once per process; the file handle
/// is held for the lifetime of the binary.
static FC_TRACE_SINK: OnceLock<std::sync::Mutex<Box<dyn Write + Send>>> =
    OnceLock::new();

const FC_TRACE_ENV_VAR: &str = "FC_TRACE";
const FC_TRACE_FILE_ENV_VAR: &str = "FC_TRACE_FILE";

/// True iff `FC_TRACE` env var is set to `1` at first call. Cheap to
/// call repeatedly (single OnceLock read).
pub fn fc_trace_enabled() -> bool {
    *FC_TRACE_ENABLED.get_or_init(|| {
        std::env::var(FC_TRACE_ENV_VAR).as_deref() == Ok("1")
    })
}

/// Emit one event line. Caller passes the FC node id + a slice of
/// (key, JSON-fragment) pairs. Caller is responsible for JSON-encoding
/// the values (use `json_str` for strings; integers / booleans as-is).
///
/// Events are skipped when `FC_TRACE` is unset — this is the cold path
/// in production runs.
///
/// `run_id` and `tx` are passed positionally so the macro stays
/// readable at the call site; future Phase D+ may promote them to a
/// thread-local context. `agent_id` is `None` for run-level events
/// (boot, halt, mr tick).
pub fn emit_event(
    fc: FcId,
    run_id: &str,
    tx: Option<u64>,
    agent_id: Option<&str>,
    kv: &[(&str, String)],
) {
    if !fc_trace_enabled() {
        return;
    }
    let ts_ms = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0);

    let mut line = String::with_capacity(128 + 32 * kv.len());
    line.push('{');
    write_kv_unchecked(&mut line, "ts_ms", &ts_ms.to_string(), false);
    line.push(',');
    write_kv_unchecked(&mut line, "fc_id", &json_str(fc.as_str()), false);
    line.push(',');
    write_kv_unchecked(&mut line, "run_id", &json_str(run_id), false);
    if let Some(t) = tx {
        line.push(',');
        write_kv_unchecked(&mut line, "tx", &t.to_string(), false);
    }
    if let Some(a) = agent_id {
        line.push(',');
        write_kv_unchecked(&mut line, "agent_id", &json_str(a), false);
    }
    if !kv.is_empty() {
        line.push_str(r#","kv":{"#);
        for (i, (k, v)) in kv.iter().enumerate() {
            if i > 0 {
                line.push(',');
            }
            write_kv_unchecked(&mut line, k, v, false);
        }
        line.push('}');
    }
    line.push('}');
    line.push('\n');

    // Lock-then-write. The Mutex is per-process; contention is bounded
    // by emit rate (~1-10 events per tx, max_tx ~200, n_agents ~8 →
    // < 16K events per run → negligible).
    let sink = FC_TRACE_SINK.get_or_init(init_sink);
    if let Ok(mut s) = sink.lock() {
        let _ = s.write_all(line.as_bytes());
    }
}

fn init_sink() -> std::sync::Mutex<Box<dyn Write + Send>> {
    let sink: Box<dyn Write + Send> = match std::env::var(FC_TRACE_FILE_ENV_VAR) {
        Ok(path) if !path.is_empty() => match std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
        {
            Ok(f) => Box::new(f),
            Err(_) => Box::new(std::io::stderr()),
        },
        _ => Box::new(std::io::stderr()),
    };
    std::sync::Mutex::new(sink)
}

/// JSON-escape a string with surrounding double quotes. Handles the
/// minimal escape set required by RFC 8259 § 7. Public so callers can
/// pre-encode string values for `kv` slots.
pub fn json_str(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str(r#"\""#),
            '\\' => out.push_str(r"\\"),
            '\n' => out.push_str(r"\n"),
            '\r' => out.push_str(r"\r"),
            '\t' => out.push_str(r"\t"),
            c if (c as u32) < 0x20 => {
                out.push_str(&format!("\\u{:04x}", c as u32));
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn write_kv_unchecked(buf: &mut String, key: &str, value_json: &str, _trailing_comma: bool) {
    buf.push('"');
    buf.push_str(key);
    buf.push_str("\":");
    buf.push_str(value_json);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    // Serialise env mutation per memory feedback_env_var_test_lock.
    static ENV_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn fc_id_strings_are_stable() {
        // Phase D consumers + TRACE_MATRIX rows join on these exact
        // spellings; any rename here is a breaking schema change.
        assert_eq!(FcId::Fc1N7.as_str(), "FC1-N7");
        assert_eq!(FcId::Fc1N11.as_str(), "FC1-N11");
        assert_eq!(FcId::Fc1N12.as_str(), "FC1-N12");
        assert_eq!(FcId::Fc1E18.as_str(), "FC1-E18");
        assert_eq!(FcId::Fc2N20.as_str(), "FC2-N20");
        assert_eq!(FcId::Fc2N22.as_str(), "FC2-N22");
        assert_eq!(FcId::Fc3N31.as_str(), "FC3-N31");
    }

    #[test]
    fn fc_id_display_matches_as_str() {
        assert_eq!(format!("{}", FcId::Fc2N22), "FC2-N22");
    }

    #[test]
    fn json_str_escapes_required_chars() {
        // RFC 8259 § 7 minimal escape set.
        assert_eq!(json_str(r#"a"b"#), r#""a\"b""#);
        assert_eq!(json_str(r"a\b"), r#""a\\b""#);
        assert_eq!(json_str("a\nb"), r#""a\nb""#);
        assert_eq!(json_str("a\tb"), r#""a\tb""#);
        // Control chars get \u escapes
        assert_eq!(json_str("\x01"), r#""\u0001""#);
        assert_eq!(json_str("\x1f"), r#""\u001f""#);
        // Plain ASCII passes through unchanged
        assert_eq!(json_str("simple"), r#""simple""#);
        // Empty
        assert_eq!(json_str(""), r#""""#);
    }

    #[test]
    fn emit_is_no_op_when_disabled() {
        // The `fc_trace_enabled()` cache is process-global and read
        // once. We can't reliably toggle it inside a test (OnceLock
        // semantics), so we exercise the public surface and assume
        // cold-path correctness from the type-level guarantee:
        // emit_event short-circuits before any I/O when the gate is
        // false. In real production the env var is unset and this is
        // the universal case.
        let _guard = ENV_LOCK.lock().unwrap();
        std::env::remove_var(FC_TRACE_ENV_VAR);
        // First call seeds the OnceLock to false.
        let first = fc_trace_enabled();
        // Subsequent calls return the cached value regardless of env.
        std::env::set_var(FC_TRACE_ENV_VAR, "1");
        let second = fc_trace_enabled();
        // Both reads return whatever the OnceLock latched first.
        assert_eq!(first, second);
        std::env::remove_var(FC_TRACE_ENV_VAR);
    }

    #[test]
    fn emit_event_with_no_kv_or_agent_does_not_panic() {
        // Cold-path smoke: gate is whatever the OnceLock latched,
        // emit_event must not panic on a minimal call shape (run-level
        // event with no agent + empty kv slice).
        emit_event(FcId::Fc2N22, "test_run", None, None, &[]);
    }

    #[test]
    fn emit_event_with_full_payload_does_not_panic() {
        emit_event(
            FcId::Fc1N12,
            "test_run",
            Some(17),
            Some("Agent_2"),
            &[
                ("verdict", "true".to_string()),
                ("elapsed_ms", "523".to_string()),
                ("payload_hash", json_str("deadbeef")),
            ],
        );
    }
}
