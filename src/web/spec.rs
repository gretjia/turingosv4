/// TRACE_MATRIX FC1-N5 + FC2-N16: Phase 5 — `/build` driven-grill endpoint.
///
/// Single route exposed:
///   POST /api/spec/turn  → drive one Meta-AI grill turn (LLM picks next Q
///                          based on prior answers + canonical slot coverage).
///                          On `done=true`: in-process A6 spec.md synthesis +
///                          SpecCapsule write to CAS + WS `SpecGrillComplete`
///                          broadcast.
///
/// The static 8-question batch path (`GET /api/spec/questions` +
/// `POST /api/spec/submit`) was removed in Phase 5 (2026-05-22). Real
/// Software 3.0 surface — every question is an LLM runtime decision; the
/// prompt-as-program (asset `grill_meta_v1.md`, embedded via include_bytes!)
/// is the source of truth. No "fixed N questions" toggle.
///
/// FC-trace: FC1-N5 (read-view shielding at trust boundary) +
///           FC2-N16 (write action via existing Phase 6.3 CLI shellout; no new
///           Class-4 admission; spec capsule anchoring is a CAS-N write through
///           the Phase 6.3 substrate).
/// Risk class: Class 2.
///
/// # API key contract
///
/// `SILICONFLOW_API_KEY` must be set in the environment when the backend
/// process starts (or supplied through `/welcome` step 3 in-memory). The
/// handler passes it through to the spawned `turingos llm` child process via
/// process inheritance. The key is NEVER written to disk.
///
/// # Session workspace layout
///
/// Each driven session creates a per-session subdirectory under the base workspace:
///   <workspace>/sessions/<session_id>/
///   <workspace>/sessions/<session_id>/cas/                    ← per-session CAS
///   <workspace>/sessions/<session_id>/spec.md                 ← written on done
///   <workspace>/sessions/<session_id>/spec_transcript.jsonl   ← per-turn log
///
/// # Embedded prompts
///
/// `assets/prompts/grill_meta_v1.md` (and triage + synthesis) are embedded at
/// compile time via `runtime::embedded_prompts` and materialized into
/// `<workspace>/assets/prompts/` by `welcome_init_handler`. PromotionReceipt
/// seeded at init satisfies the C10 guard.
///
/// # Binary override (for tests)
///
/// Setting `TURINGOS_BACKEND_OVERRIDE` replaces the default binary (`turingos`).
/// Same resolution order as write.rs.
#[cfg(feature = "web")]
use axum::{extract::State, http::StatusCode, Json};
#[cfg(feature = "web")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "web")]
use std::path::PathBuf;
#[cfg(feature = "web")]
use std::time::Duration;

#[cfg(feature = "web")]
use super::ws::{AppState, WsBroadcastMsg};
#[cfg(feature = "web")]
use turingosv4::runtime::grill_envelope::CANONICAL_SLOTS;
#[cfg(feature = "web")]
use turingosv4::sdk::sanitized_runner::{
    env_allowlist_from_current, run_sanitized, SanitizedCommand, SanitizedOutput,
};

// ---------------------------------------------------------------------------
// Request / Response / Error types
// ---------------------------------------------------------------------------

/// TRACE_MATRIX FC1-N5: error response for spec endpoints.
///
/// `kind` values:
/// - `"invalid_input"`:        field validation failed (400)
/// - `"shellout_failed"`:      CLI exited non-zero (500)
/// - `"spec_md_missing"`:      CLI succeeded but spec.md not found (500)
/// - `"prompt_asset_missing"`: meta-prompt asset (assets/prompts/grill_meta_v1.md)
///                             unreadable from the workspace CWD (500). Added in
///                             fix F4 so the failure mode is no longer
///                             misattributed to `shellout_failed`.
#[cfg(feature = "web")]
#[derive(Debug, Serialize)]
pub(crate) struct SpecError {
    pub(crate) reason: String,
    pub(crate) kind: &'static str,
}


/// Returns `true` if `s` is a safe session ID: `^[a-zA-Z0-9_-]{1,128}$`.
///
/// Session IDs are used as directory names under `sessions/`, so they must
/// not contain path-traversal characters (`.`, `/`, `\`) or shell metacharacters.
#[cfg(feature = "web")]
fn is_safe_session_id(s: &str) -> bool {
    if s.is_empty() || s.len() > 128 {
        return false;
    }
    s.chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
}

// ---------------------------------------------------------------------------
// Session ID generation (no UUID crate)
// ---------------------------------------------------------------------------

/// Generate a session ID as `<unix_secs>_<hex8>`.
///
/// Uses a FNV-1a hash of the current time in nanoseconds for the hex suffix,
/// producing IDs like `1716000000_3f8a1b2c`. Collision probability is
/// negligible for the expected request rate.
#[cfg(feature = "web")]
fn generate_session_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let secs = now.as_secs();
    let nanos = now.subsec_nanos();
    // FNV-1a 32-bit on nanos for short hex suffix.
    let mut h: u32 = 2_166_136_261;
    for &b in nanos.to_le_bytes().iter() {
        h ^= b as u32;
        h = h.wrapping_mul(16_777_619);
    }
    format!("{secs}_{h:08x}")
}

// ---------------------------------------------------------------------------
// CAS CID parser
// ---------------------------------------------------------------------------

/// Parse the CAS capsule CID from `turingos spec` stdout.
///
/// The CLI emits a line like:
/// ```
///   CAS capsule CID    -> <cid_hex>
/// ```
/// We scan for `CAS capsule CID` and extract the hex string after `->`.
#[cfg(feature = "web")]
fn parse_capsule_cid_from_stdout(stdout: &str) -> Option<String> {
    for line in stdout.lines() {
        // Match "  CAS capsule CID    -> <hex>"
        if line.contains("CAS capsule CID") {
            if let Some(pos) = line.find("->") {
                let cid = line[pos + 2..].trim().to_string();
                if !cid.is_empty() {
                    return Some(cid);
                }
            }
        }
    }
    None
}

// ---------------------------------------------------------------------------
// Binary / workspace resolution helpers (shared pattern with write.rs)
// ---------------------------------------------------------------------------

/// Resolve which binary to invoke for `turingos`.
///
/// Resolution order:
///   1. `TURINGOS_BACKEND_OVERRIDE` env var (full path; for tests)
///   2. Sibling `turingos` next to the running `turingos_web` binary
///   3. Bare `"turingos"` (PATH lookup)
#[cfg(feature = "web")]
fn resolve_turingos_bin() -> String {
    if let Ok(v) = std::env::var("TURINGOS_BACKEND_OVERRIDE") {
        if !v.is_empty() {
            return v;
        }
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let sibling = dir.join("turingos");
            if sibling.exists() {
                return sibling.to_string_lossy().into_owned();
            }
        }
    }
    "turingos".to_string()
}

/// Resolve the TuringOS workspace directory.
///
/// Resolution order:
///   1. `TURINGOS_WEB_WORKSPACE` env var (explicit operator config)
///   2. `tmp/phase7_active` (W8.1: harmonized with welcome.rs default;
///      previously fell back to `current_dir()` which caused session dirs
///      to land in the repo root — see W8 Validation Round 1 finding P2.)
#[cfg(feature = "web")]
fn resolve_workspace() -> String {
    if let Ok(v) = std::env::var("TURINGOS_WEB_WORKSPACE") {
        if !v.is_empty() {
            return v;
        }
    }
    "tmp/phase7_active".to_string()
}

#[cfg(feature = "web")]
async fn run_web_command(
    bin: String,
    cwd: PathBuf,
    args: Vec<String>,
    env: std::collections::BTreeMap<String, String>,
    timeout: Duration,
) -> Result<SanitizedOutput, String> {
    let cwd = explicit_cwd(cwd);
    let command = SanitizedCommand {
        program: PathBuf::from(bin),
        args,
        cwd,
        env,
        stdin: None,
        timeout,
    };
    tokio::task::spawn_blocking(move || run_sanitized(command))
        .await
        .map_err(|e| format!("spawn_blocking: {e}"))?
        .map_err(|e| e.to_string())
}

#[cfg(feature = "web")]
fn explicit_cwd(path: PathBuf) -> PathBuf {
    if path.is_dir() {
        path
    } else {
        std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
    }
}

#[cfg(feature = "web")]
fn web_llm_child_env() -> std::collections::BTreeMap<String, String> {
    env_allowlist_from_current(&[
        "PATH",
        "SILICONFLOW_API_KEY",
        "DEEPSEEK_API_KEY",
        "DEEPSEEK_API_KEY_WORKER",
        "OPENROUTER_API_KEY",
        "OPENAI_API_KEY",
        "TURINGOS_SILICONFLOW_ENDPOINT",
    ])
}

#[cfg(feature = "web")]
fn combine_truncated(stdout: &[u8], stderr: &[u8]) -> String {
    let stdout_s = String::from_utf8_lossy(stdout);
    let stderr_s = String::from_utf8_lossy(stderr);
    let combined = format!("stdout: {stdout_s} | stderr: {stderr_s}");
    if combined.len() > 512 {
        format!("{}…", &combined[..512])
    } else {
        combined
    }
}

// ===========================================================================
// Phase 6.3.x: POST /api/spec/turn — driven-mode grill turn handler (W7)
// ===========================================================================
//
// TRACE_MATRIX FC2-N16 + FC1-N5: W7 atom. Web-layer driven-mode turn loop.
// Mirrors W6 (cmd_spec.rs run_driven_mode) at the HTTP layer, using
// AppState.sessions for process-local session state and shelling out to the
// turingos binary for all LLM + CAS operations.
// Risk class: Class 2.

// ---------------------------------------------------------------------------
// W7 request / response / error types
// ---------------------------------------------------------------------------

/// POST /api/spec/turn request body.
#[cfg(feature = "web")]
#[derive(Debug, Deserialize)]
pub(crate) struct SpecTurnRequest {
    pub(crate) session_id: String,
    /// None on turn-1 setup call (server creates the session and emits Q1).
    pub(crate) user_answer: Option<String>,
    /// Only honoured on session creation. "zh" | "en". Default: "zh".
    pub(crate) lang: Option<String>,
}

/// POST /api/spec/turn success response body.
#[cfg(feature = "web")]
#[derive(Debug, Serialize)]
pub(crate) struct SpecTurnResponse {
    pub(crate) turn_index: u32,
    pub(crate) question_text: String,
    pub(crate) covered_slots: Vec<String>,
    pub(crate) open_slots: Vec<String>,
    pub(crate) confidence: f64,
    pub(crate) done: bool,
    /// Populated only when done == true.
    pub(crate) playback: Option<serde_json::Value>,
    pub(crate) terminated: bool,
    /// Populated only when terminated == true (clean synthesis).
    pub(crate) spec_capsule_cid: Option<String>,
    /// CID of the turn capsule just written (shell-out produces this).
    pub(crate) turn_capsule_cid: Option<String>,
    /// FIX F6 (2026-05-18): when `terminated == true` AND `spec_capsule_cid`
    /// is None, this string carries the reason — e.g.
    /// `"turn_ceiling_15_no_spec"`, `"user_input_unparseable_no_spec"`,
    /// `"predicate_double_fail_no_spec"`. The client/UI uses this to render
    /// "session ended without producing a spec" instead of silently treating
    /// the empty CID as "spec just hasn't loaded yet".
    ///
    /// Pre-F6 the handler shelled out to a fictional `turingos spec
    /// --synthesize-only` flag that the CLI silently ignored (the CLI arg
    /// parser has no match for `--synthesize-only` / `--session` /
    /// `--termination-reason`, so the `_ => {}` catch-all dropped them and
    /// run_driven_mode started a fresh interactive interview which then
    /// blocked on the meta-prompt asset / SiliconFlow API). The handler
    /// discarded the subprocess output, parsed nothing useful, and returned
    /// `spec_capsule_cid: None` — the user could not tell whether the spec
    /// was being synthesised asynchronously, had silently failed, or was
    /// never going to exist. F6 removes the broken shellout and surfaces
    /// the truth via this field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) termination_reason: Option<String>,
    /// When non-None, this HTTP response is a triage bounce-back (the user's
    /// answer was classified as off_topic / gibberish / abusive). `question_text`
    /// carries the previous question again; this field carries the triage class
    /// so the client can render the appropriate nudge text WITHOUT depending on
    /// the WebSocket `SpecTurnTriageReject` event (which may not arrive if the
    /// WS connection is absent or delayed).
    ///
    /// Values: "off_topic" | "gibberish" | "abusive". None on all normal paths.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) triage_class: Option<String>,
}

/// Error body for /api/spec/turn.
#[cfg(feature = "web")]
#[derive(Debug, Serialize)]
pub(crate) struct ErrorBody {
    pub(crate) error: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) kind: Option<&'static str>,
}

// Convenience constructor
#[cfg(feature = "web")]
impl ErrorBody {
    fn new(error: impl Into<String>) -> Self {
        Self {
            error: error.into(),
            kind: None,
        }
    }
    fn with_kind(error: impl Into<String>, kind: &'static str) -> Self {
        Self {
            error: error.into(),
            kind: Some(kind),
        }
    }
}

// ---------------------------------------------------------------------------
// Helper: parse TurnPayload from turingos llm complete stdout
// ---------------------------------------------------------------------------

/// Parse a TurnPayload out of the JSON blob emitted by `turingos llm complete`.
/// The blob has shape: `{ ok: bool, content: "...<json>...", parsed_envelope: {...}, ... }`.
/// We first try `parsed_envelope` (the pre-parsed form), then fall back to
/// parsing `content` directly.
#[cfg(feature = "web")]
fn parse_turn_payload_from_llm_output(stdout: &str) -> Result<serde_json::Value, String> {
    let v: serde_json::Value =
        serde_json::from_str(stdout.trim()).map_err(|e| format!("parse llm complete JSON: {e}"))?;

    // Check ok flag
    let ok = v.get("ok").and_then(|x| x.as_bool()).unwrap_or(false);
    if !ok {
        let content = v
            .get("content")
            .and_then(|x| x.as_str())
            .unwrap_or("<empty>");
        return Err(format!(
            "llm complete returned ok=false; content={}",
            &content[..content.len().min(200)]
        ));
    }

    // Prefer pre-parsed envelope
    if let Some(env) = v.get("parsed_envelope") {
        if !env.is_null() {
            return Ok(env.clone());
        }
    }

    // Fall back: parse content string as JSON
    let content = v
        .get("content")
        .and_then(|x| x.as_str())
        .ok_or_else(|| "llm complete: missing content field".to_string())?;
    serde_json::from_str(content.trim()).map_err(|e| format!("parse content as envelope JSON: {e}"))
}

/// Parse the triage class from `turingos llm triage` stdout JSON.
///
/// Success shape: `{ ok: true, class: "relevant"|"off_topic"|"abusive"|"gibberish", ... }`.
/// Failure shape: `{ ok: false, error: { kind, detail } }`  (set by
/// `cmd_llm::complete_err_exit`; e.g. SiliconFlow HTTP 5xx, transport
/// timeout, JSON parse failure on the Blackbox response).
///
/// FIX F6 (2026-05-18): the pre-F6 version did NOT check the `ok` flag and
/// fell through to `class.unwrap_or("gibberish")` on the failure shape — so
/// every transient triage subprocess failure was silently re-interpreted as
/// a "gibberish" verdict, which the handler then treated as non-relevant.
/// Two consecutive triage subprocess failures would prematurely terminate
/// the session (`non_relevant_count >= 2` abort branch) with
/// `spec_capsule_cid: null`. That is the W1.2 p1_backend "5 consecutive
/// shellout failures → terminated with empty state, no spec_capsule_cid"
/// failure mode (re-classified after F6 from "LLM shellout failure" to
/// "triage subprocess failure mishandled by parser").
///
/// Post-F6: an `ok=false` payload returns `Err(...)`. The caller (in
/// `spec_turn_handler` step 9) propagates this to HTTP 500 with
/// `kind: "triage_shellout_failed"` — the client can then retry the same
/// session_id without burning a triage-non-relevant slot.
#[cfg(feature = "web")]
fn parse_triage_class_from_output(stdout: &str) -> Result<String, String> {
    let v: serde_json::Value =
        serde_json::from_str(stdout.trim()).map_err(|e| format!("parse triage JSON: {e}"))?;
    // FIX F6: check ok flag first; treat ok=false as a subprocess failure,
    // not as a "gibberish" classification.
    let ok = v.get("ok").and_then(|x| x.as_bool()).unwrap_or(false);
    if !ok {
        let kind = v
            .get("error")
            .and_then(|e| e.get("kind"))
            .and_then(|x| x.as_str())
            .unwrap_or("unknown");
        let detail = v
            .get("error")
            .and_then(|e| e.get("detail"))
            .and_then(|x| x.as_str())
            .unwrap_or("<no detail>");
        // Clip detail to keep the error body small and human-grep-able.
        return Err(format!(
            "triage returned ok=false; kind={kind}; detail={}",
            &detail[..detail.len().min(200)]
        ));
    }
    let class = v
        .get("class")
        .and_then(|x| x.as_str())
        .ok_or_else(|| "triage ok=true but class field missing".to_string())?
        .to_string();
    Ok(class)
}

/// Parse the turn CID emitted by `turingos llm complete`.
/// Looks for `"turn_capsule_cid"` in the JSON blob.
#[cfg(feature = "web")]
fn parse_turn_cid_from_llm_output(stdout: &str) -> Option<String> {
    let v: serde_json::Value = serde_json::from_str(stdout.trim()).ok()?;
    v.get("turn_capsule_cid")
        .or_else(|| v.get("capsule_cid"))
        .and_then(|x| x.as_str())
        .map(|s| s.to_string())
}

/// Extract a string field from a JSON value, with a default.
#[cfg(feature = "web")]
fn jstr<'a>(v: &'a serde_json::Value, key: &str, default: &'a str) -> &'a str {
    v.get(key).and_then(|x| x.as_str()).unwrap_or(default)
}

// ---------------------------------------------------------------------------
// Helper: build a fake-CID placeholder for shell-out failures
// ---------------------------------------------------------------------------
//
// FIX F6 (2026-05-18): retained but no longer called. The pre-F6 termination
// path (`if done { … }`) used `parse_capsule_cid_from_stdout(&synth_stdout)
// .or_else(|| Some(placeholder_cid()))` to synthesise a fake CID when the
// (broken) `--synthesize-only` shellout returned no parsable CID. That fake
// CID looked real to clients but pointed at nothing in CAS. F6 removes the
// broken shellout and surfaces `spec_capsule_cid: None` + a
// `termination_reason` string instead, so this helper has no caller. Kept
// in place (allow(dead_code)) so the F6 regression test
// `placeholder_cid_format_stays_stable` can pin the pre-F6 shape for
// future audit; a follow-up cleanup atom may remove it.

#[cfg(feature = "web")]
#[allow(dead_code)]
fn placeholder_cid() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let t = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    format!("placeholder_{t:016x}")
}

// ---------------------------------------------------------------------------
// Helper: extract covered/open slots from parsed envelope
// ---------------------------------------------------------------------------

#[cfg(feature = "web")]
fn extract_slots(envelope: &serde_json::Value) -> (Vec<String>, Vec<String>) {
    // Vocabulary must mirror the canonical grill substrate; importing
    // CANONICAL_SLOTS keeps the dependency explicit and tracks the source
    // forever. The previous inline list used Researcher-C's draft vocabulary
    // (job_story/data_model/first_click/...) which never matched what the LLM
    // actually emits (job/memory/first_run/...). Result: the WS broadcast's
    // open_slots field was always wrong, breaking the frontend progress hint.
    let covered: Vec<String> = envelope
        .get("covered_slots")
        .and_then(|x| x.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default();

    let open: Vec<String> = CANONICAL_SLOTS
        .iter()
        .filter(|s| !covered.iter().any(|c| c.as_str() == **s))
        .map(|s| (*s).to_string())
        .collect();

    (covered, open)
}

// ---------------------------------------------------------------------------
// POST /api/spec/turn handler
// ---------------------------------------------------------------------------

/// TRACE_MATRIX FC2-N16 + FC1-N5: Phase 6.3.x W7 — POST /api/spec/turn handler.
///
/// Implements the per-request driven-mode turn state machine. Session state is
/// held in `AppState.sessions` (process-local). CAS capsules are written via
/// shell-out to `turingos llm complete` / `turingos spec --synthesize-only`.
/// Per R2 §A14: no session-resume on server restart (sessions HashMap is empty
/// after restart; client receives 404).
#[cfg(feature = "web")]
pub(crate) async fn spec_turn_handler(
    State(state): State<AppState>,
    Json(req): Json<SpecTurnRequest>,
) -> Result<Json<SpecTurnResponse>, (StatusCode, Json<ErrorBody>)> {
    use super::ws::{GrillSession, SlotState};
    use std::time::{SystemTime, UNIX_EPOCH};

    // ── Step 1: validate session_id ───────────────────────────────────────────
    if !is_safe_session_id(&req.session_id) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorBody::with_kind(
                format!(
                    "session_id {:?} is invalid; must match ^[a-zA-Z0-9_-]{{1,128}}$",
                    req.session_id
                ),
                "invalid_input",
            )),
        ));
    }

    // ── Step 2: validate user_answer length if present ───────────────────────
    if let Some(ans) = req.user_answer.as_deref() {
        if ans.len() > 4096 {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorBody::with_kind(
                    format!("user_answer is too long ({} chars); max is 4096", ans.len()),
                    "invalid_input",
                )),
            ));
        }
    }

    // ── Step 3: resolve workspace and binary ─────────────────────────────────
    let workspace = resolve_workspace();
    let bin = resolve_turingos_bin();

    // ── Step 4: fetch or create GrillSession ─────────────────────────────────
    let is_new_session;
    {
        let mut sessions = state.sessions.lock().map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorBody::new(format!("sessions lock poisoned: {e}"))),
            )
        })?;

        is_new_session = !sessions.contains_key(&req.session_id);
        if is_new_session {
            let lang = req.lang.as_deref().unwrap_or("zh").to_string();
            let now_unix = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0);
            sessions.insert(
                req.session_id.clone(),
                GrillSession {
                    session_id: req.session_id.clone(),
                    turn_count: 0,
                    lang,
                    coverage_state: std::collections::HashMap::new(),
                    last_3_turns: std::collections::VecDeque::new(),
                    turn_cids: vec![],
                    terminated: false,
                    parent_turn_cid: None,
                    created_at_unix: now_unix,
                    non_relevant_count: 0,
                    last_prev_covered: vec![],
                    meta_turns_accepted: 0,
                    meta_turns_rejected: 0,
                    triage_calls_relevant: 0,
                    triage_calls_non_relevant: 0,
                    // FIX F6 (2026-05-18): persist the Meta-emitted question
                    // across requests so the next turn's triage call gets
                    // real Q/A context. Pre-F6 this value was never stored
                    // and `prev_question` defaulted to "", which biased
                    // triage toward non-relevant for terse answers and
                    // produced spurious HTTP 200 empty-zero bounces.
                    last_question_emitted: String::new(),
                    // FIX A6 (2026-05-19): full triage-relevant answer history
                    // for in-process spec.md synthesis at done=true (step 13).
                    // Mirrors `cmd_spec::DrivenState::all_user_answers`.
                    all_user_answers: Vec::new(),
                    // FIX F10 (2026-05-19): slot→answer map populated by the
                    // step-11 covered-slot delta. Backs the new
                    // slot-keyed spec.md synthesiser (D-NEW-3a fix).
                    slot_evidence: std::collections::BTreeMap::new(),
                },
            );
        }
    }

    // ── Step 5: check if already terminated ──────────────────────────────────
    {
        let sessions = state.sessions.lock().map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorBody::new(format!("sessions lock poisoned: {e}"))),
            )
        })?;
        if let Some(sess) = sessions.get(&req.session_id) {
            if sess.terminated {
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(ErrorBody::with_kind(
                        "session already terminated",
                        "session_terminated",
                    )),
                ));
            }
        }
    }

    // ── Step 6: session-not-found guard (only fires on subsequent turns for a
    //    session that was never created, i.e. user_answer provided but no prior
    //    null-answer call was ever made) ───────────────────────────────────────
    if !is_new_session && req.user_answer.is_none() {
        // A null user_answer on an already-existing session is treated as a
        // re-init attempt; reject it to keep the state machine unambiguous.
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorBody::with_kind(
                "session already exists; null user_answer only valid on first turn",
                "invalid_input",
            )),
        ));
    }
    if is_new_session && req.user_answer.is_some() {
        // Can't provide an answer without a prior question.
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorBody::with_kind(
                "session does not exist; send user_answer=null to start a new session",
                "invalid_input",
            )),
        ));
    }

    // ── Step 7: read current session state snapshot ──────────────────────────
    let (
        turn_count,
        lang,
        last_3_turns_snap,
        _parent_turn_cid_snap,
        _non_relevant_count,
        last_prev_covered_snap,
        last_question_emitted_snap,
    ) = {
        let sessions = state.sessions.lock().map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorBody::new(format!("sessions lock poisoned: {e}"))),
            )
        })?;
        let sess = sessions.get(&req.session_id).unwrap(); // safe: we just inserted
        (
            sess.turn_count,
            sess.lang.clone(),
            sess.last_3_turns.clone(),
            sess.parent_turn_cid.clone(),
            sess.non_relevant_count,
            sess.last_prev_covered.clone(),
            sess.last_question_emitted.clone(),
        )
    };

    // ── Step 8: hard turn ceiling check ─────────────────────────────────────
    if turn_count >= 15 {
        // Force terminate
        let _ = state
            .broadcast_tx
            .send(super::ws::WsBroadcastMsg::SpecGrillComplete {
                session_id: req.session_id.clone(),
                spec_capsule_cid: String::new(),
            });
        {
            let mut sessions = state.sessions.lock().unwrap_or_else(|e| e.into_inner());
            if let Some(sess) = sessions.get_mut(&req.session_id) {
                sess.terminated = true;
            }
        }
        // FIX F6 (2026-05-18): surface the truth — the web layer cannot
        // in-process write a partial-spec capsule (the spec_capsule
        // module lives under src/bin/turingos/ and is not available to
        // src/web/; out-of-scope for F6 to library-ize it). Be explicit
        // that no spec was synthesised so the UI does not wait forever
        // for `spec_capsule_cid` to materialise.
        return Ok(Json(SpecTurnResponse {
            turn_index: turn_count,
            question_text: String::new(),
            covered_slots: vec![],
            open_slots: vec![],
            confidence: 0.0,
            done: false,
            playback: None,
            terminated: true,
            spec_capsule_cid: None,
            turn_capsule_cid: None,
            termination_reason: Some("turn_ceiling_15_no_spec".to_string()),
            triage_class: None,
        }));
    }

    // ── Step 9: triage user_answer if present (subsequent turns) ─────────────
    //
    // FIX F6 (2026-05-18): use the persisted `last_question_emitted` field —
    // the question_text emitted by the most recent accepted Meta turn.
    //
    // Pre-F6 this was derived from `last_3_turns_snap.back().0`, but that
    // entry stores the prev_question observed BEFORE the previous accepted
    // turn (one-turn-stale), and on turn 1 it's empty entirely. Result:
    // triage always ran with `--question ""`, biasing the Blackbox model
    // toward non-relevant for terse Chinese answers — surfacing as HTTP 200
    // with all-zero fields (the triage non-relevant bounce-back at line
    // ~1040) and, after two consecutive bounces, premature `terminated:true`
    // with `spec_capsule_cid:null`. See W1.1 mrs_chen turns 2/6/7 and
    // W1.2 p1_backend turn 5 retry3 for the live failure shape.
    //
    // Fallback to the rolling-last-3 value only if the new field is empty
    // (e.g. an in-process session created before the F6 deploy — defensive).
    let prev_question: String = if !last_question_emitted_snap.is_empty() {
        last_question_emitted_snap.clone()
    } else {
        last_3_turns_snap
            .back()
            .map(|(q, _)| q.clone())
            .unwrap_or_default()
    };

    if let Some(user_answer) = req.user_answer.as_deref() {
        // Triage the answer
        let session_dir = PathBuf::from(&workspace)
            .join("sessions")
            .join(&req.session_id);
        let capsules_dir = session_dir.join("capsules");
        {
            let dir = capsules_dir.clone();
            tokio::task::spawn_blocking(move || std::fs::create_dir_all(&dir))
                .await
                .ok();
        }

        let triage_turn_id = format!("turn-{}-triage", turn_count + 1);
        let triage_output = run_web_command(
            bin.clone(),
            session_dir.clone(),
            vec![
                "llm".into(),
                "triage".into(),
                "--workspace".into(),
                workspace.clone(),
                "--user-answer".into(),
                user_answer.to_string(),
                "--question".into(),
                prev_question.clone(),
                "--lang".into(),
                lang.clone(),
                "--capsule-dir".into(),
                capsules_dir.to_string_lossy().into_owned(),
                "--turn-id".into(),
                triage_turn_id.clone(),
            ],
            web_llm_child_env(),
            Duration::from_secs(240),
        )
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorBody::with_kind(
                    format!("triage shellout failed: {e}"),
                    "triage_shellout_failed",
                )),
            )
        })?;
        if !triage_output.success() {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorBody::with_kind(
                    format!(
                        "triage exit {:?}: {}",
                        triage_output.exit_code,
                        combine_truncated(&triage_output.stdout, &triage_output.stderr)
                    ),
                    "triage_shellout_failed",
                )),
            ));
        }
        let triage_stdout = String::from_utf8_lossy(&triage_output.stdout).into_owned();

        // FIX F6 (2026-05-18): treat triage subprocess failure (ok=false,
        // empty stdout, malformed JSON) as a 500, NOT as a "gibberish"
        // verdict. Pre-F6 a transient SiliconFlow HTTP 5xx on the triage
        // call silently bumped non_relevant_count and could abort the
        // session after two consecutive transient failures — w/ no
        // spec_capsule_cid (W1.2 p1_backend). The client can retry the
        // same session_id on a 500; it cannot recover from a falsely-
        // recorded non-relevant slot.
        let triage_class = match parse_triage_class_from_output(&triage_stdout) {
            Ok(c) => c,
            Err(e) => {
                log::warn!("spec_turn_handler: triage parse error: {e}");
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorBody::with_kind(
                        format!("triage failed: {e}"),
                        "triage_shellout_failed",
                    )),
                ));
            }
        };

        if triage_class != "relevant" {
            // Non-relevant: increment counter, maybe terminate
            let new_non_relevant;
            {
                let mut sessions = state.sessions.lock().unwrap_or_else(|e| e.into_inner());
                let sess = sessions.get_mut(&req.session_id).unwrap();
                sess.non_relevant_count += 1;
                sess.triage_calls_non_relevant += 1;
                new_non_relevant = sess.non_relevant_count;
            }

            // Broadcast SpecTurnTriageReject
            let _ = state
                .broadcast_tx
                .send(super::ws::WsBroadcastMsg::SpecTurnTriageReject {
                    session_id: req.session_id.clone(),
                    turn_index: turn_count + 1,
                    triage_class: triage_class.clone(),
                    non_relevant_count: new_non_relevant,
                });

            if new_non_relevant >= 2 {
                // Terminate session
                {
                    let mut sessions = state.sessions.lock().unwrap_or_else(|e| e.into_inner());
                    if let Some(sess) = sessions.get_mut(&req.session_id) {
                        sess.terminated = true;
                    }
                }
                // FIX F6 (2026-05-18): the pre-F6 "shell out to write session
                // capsule with termination_reason" call invoked `turingos spec
                // --workspace ... --session ... --mode driven --synthesize-only
                // --termination-reason user_input_unparseable`. NONE of
                // `--session`, `--synthesize-only`, or `--termination-reason`
                // are recognised by `cmd_spec::run_inner` — the catch-all
                // `_ => {}` arm in the arg parser silently drops them. The
                // CLI then sees only `--workspace ... --mode driven` and
                // calls `run_driven_mode`, which opens a fresh session,
                // re-reads the meta-prompt asset, and re-enters the LLM-
                // driven interview loop (blocking on the SiliconFlow API for
                // up to ~15s per turn, see W1.2 p1_backend turn-6 probe
                // 15-second elapsed time — that was the abort branch
                // shelling out and waiting on the LLM, not synthesising
                // anything related to the original session). The handler
                // then discarded the output (the bare `let _ = …` after
                // `.output()` ignored both stdout and exit code).
                //
                // No spec_capsule_cid was ever written, no useful work was
                // done, and the user paid a 10-15s latency + one full LLM
                // API call on every abort. Remove the broken shellout
                // entirely and surface the termination reason on the
                // response so the UI can render "session ended without
                // producing a spec" honestly. In-process synthesis is
                // blocked by spec_capsule module visibility (it lives
                // under src/bin/turingos/ and the F6 charter forbids
                // editing the binary surface), so library-ization is a
                // separate atom.
                let _ = state
                    .broadcast_tx
                    .send(super::ws::WsBroadcastMsg::SpecGrillComplete {
                        session_id: req.session_id.clone(),
                        spec_capsule_cid: String::new(),
                    });

                return Ok(Json(SpecTurnResponse {
                    // FIX F6: do NOT advance turn_index past turn_count — the
                    // user's last submission was rejected by triage and no
                    // Meta turn happened. Pre-F6 returned `turn_count + 1`
                    // which falsely claimed advancement (and made the
                    // response indistinguishable from a normal Meta turn
                    // that happened to come back empty).
                    turn_index: turn_count,
                    question_text: String::new(),
                    covered_slots: vec![],
                    open_slots: vec![],
                    confidence: 0.0,
                    done: false,
                    playback: None,
                    terminated: true,
                    spec_capsule_cid: None,
                    turn_capsule_cid: None,
                    termination_reason: Some("user_input_unparseable_no_spec".to_string()),
                    triage_class: None,
                }));
            }

            // Not yet at abort threshold — just bounce back a "please try again" response.
            //
            // FIX F6 (2026-05-18): do NOT advance turn_index past turn_count.
            // The triage rejected the user's submission and no new Meta turn
            // happened; turn_count stays where it was. Pre-F6 returned
            // `turn_count + 1` here, which was the smoking-gun symptom in
            // W1.1 mrs_chen turn 2 (response turn_index=3, elapsed 1035ms,
            // all-zero fields). The empty `question_text` (== empty
            // `prev_question`, due to the Fix-A1 root cause above) made this
            // path indistinguishable to the client from a real Meta turn
            // returning a degenerate envelope, and the false turn_index
            // advance burned the user's mental model of "how far am I
            // through the interview?".
            return Ok(Json(SpecTurnResponse {
                turn_index: turn_count,
                question_text: prev_question.clone(),
                covered_slots: vec![],
                open_slots: vec![],
                confidence: 0.0,
                done: false,
                playback: None,
                terminated: false,
                spec_capsule_cid: None,
                turn_capsule_cid: None,
                termination_reason: None,
                // Carry the triage class so the client can render the bounce-back
                // nudge from the HTTP response alone, without depending on the WS
                // SpecTurnTriageReject event (which may be absent if WS is down).
                triage_class: Some(triage_class.clone()),
            }));
        }

        // Relevant — update triage counters only. DO NOT push to
        // `last_3_turns` / `all_user_answers` here.
        //
        // FIX F9 (2026-05-19): pre-F9 this branch eagerly pushed the
        // (prev_question, user_answer) pair into `last_3_turns` AND
        // appended `user_answer` to `all_user_answers` BEFORE the
        // downstream `turingos llm complete` shellout (Step 10) ran. If
        // that shellout returned `ok=false` (e.g. SiliconFlow transient
        // 5xx, which D8 evidence shows happens 5-43% of the time), the
        // handler returned HTTP 500 to the client WITHOUT rolling back
        // the push. The client's natural recovery is to re-POST
        // `/api/spec/turn` with the same session_id + same user_answer,
        // and the retry pushed the SAME (q, a) again — corrupting the
        // rolling window with a duplicate entry.
        //
        // Symptom in Π4.2 P5 evidence
        // (`handover/evidence/phase6_3_x_universality_1779111375/pi4/
        // p5_codeswitch/turn5_main_evidence_capsule.json`): turn-5's
        // on-disk prompt JSON shows assistant-T4-mirror + user-T4-answer
        // appearing TWICE in sequence in `messages[4..7]`. The Meta LLM,
        // seeing the same user turn repeated, emitted an empty/malformed
        // envelope → F6 silent-zero handling → kernel double-fail
        // short-circuit → `user_input_unparseable_no_spec` termination.
        //
        // Fix: defer the push to AFTER `llm complete` succeeds (Step 11
        // below), matching the CLI driven path
        // (cmd_spec.rs:1288-1289 — the CLI loop pushes only after a
        // successful `payload` parse). Counter updates
        // (`triage_calls_relevant`, `non_relevant_count` reset) are safe
        // to do here because they are just bookkeeping and not consumed
        // by the prompt builder. The current turn's (q, a) is passed
        // into the prompt-build snapshot below via a local
        // `pending_qa` clone so the Meta LLM still sees the latest
        // exchange for THIS turn — it just isn't persisted to the
        // session until we know the turn succeeded.
        {
            let mut sessions = state.sessions.lock().unwrap_or_else(|e| e.into_inner());
            if let Some(sess) = sessions.get_mut(&req.session_id) {
                sess.triage_calls_relevant += 1;
                sess.non_relevant_count = 0; // reset consecutive counter on relevant answer
            }
        }
    }

    // ── Step 10: call `turingos llm complete` for next Meta turn ─────────────
    let new_turn_index = turn_count + 1;
    let session_dir = PathBuf::from(&workspace)
        .join("sessions")
        .join(&req.session_id);
    let capsules_dir = session_dir.join("capsules");
    {
        let dir = capsules_dir.clone();
        tokio::task::spawn_blocking(move || std::fs::create_dir_all(&dir))
            .await
            .ok();
    }

    // Build the prompt JSON and write to disk.
    //
    // FIX F9 (2026-05-19): build `last_3_for_prompt` as a transient
    // snapshot that appends the CURRENT (prev_question, user_answer)
    // pair to the session's persisted `last_3_turns` without mutating
    // the session. The persisted vector is only updated in Step 11
    // after `llm complete` succeeds. This makes the prompt content for
    // a given turn-index deterministic w.r.t. (session_state +
    // user_answer) regardless of how many transient-500 retries
    // happen on the client side. Pre-F9 the persisted `last_3_turns`
    // was mutated here, which double-pushed on retry and corrupted the
    // Meta LLM's history (see Π4.2 P5 evidence).
    let (coverage_summary, last_3_for_prompt) = {
        let sessions = state.sessions.lock().unwrap_or_else(|e| e.into_inner());
        let sess = sessions.get(&req.session_id).unwrap();
        let summary = build_coverage_summary(&sess.coverage_state, turn_count);
        let mut last3 = sess.last_3_turns.clone();
        if let Some(user_answer) = req.user_answer.as_deref() {
            if last3.len() == 3 {
                last3.pop_front();
            }
            last3.push_back((prev_question.clone(), user_answer.to_string()));
        }
        (summary, last3)
    };

    // FIX F4 (2026-05-18): load meta-prompt content here so it can be prepended
    // as messages[0] of the prompt JSON. The CLI driven path does the same
    // (see cmd_spec.rs::run_driven_mode step "── 2. Read meta-prompt content").
    // Path resolution mirrors the CLI default: `<workspace>/assets/prompts/
    // grill_meta_v1.md`. The web binary's CWD is the repo root per W10-R1 O4
    // + W9 evidence, so `workspace = "."` resolves correctly.
    let meta_prompt_path = PathBuf::from(&workspace).join("assets/prompts/grill_meta_v1.md");
    let meta_prompt_content = {
        let mpp = meta_prompt_path.clone();
        match tokio::task::spawn_blocking(move || std::fs::read_to_string(&mpp))
            .await
            .unwrap_or_else(|e| Err(std::io::Error::other(format!("join error: {e}"))))
        {
            Ok(s) => s,
            Err(e) => {
                log::warn!(
                    "spec_turn_handler: read meta-prompt {}: {e}",
                    meta_prompt_path.display()
                );
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorBody::with_kind(
                        format!("read meta-prompt asset {}: {e}", meta_prompt_path.display()),
                        "prompt_asset_missing",
                    )),
                ));
            }
        }
    };
    let prompt_json = build_web_turn_prompt_json(
        &meta_prompt_content,
        &coverage_summary,
        &last_3_for_prompt,
        new_turn_index,
        None,
    );
    let prompt_file_path = session_dir.join(format!("turn-{new_turn_index}-prompt.json"));
    {
        let pf = prompt_file_path.clone();
        let pj = prompt_json.clone();
        tokio::task::spawn_blocking(move || {
            let _ = std::fs::create_dir_all(pf.parent().unwrap_or(std::path::Path::new(".")));
            std::fs::write(&pf, &pj)
        })
        .await
        .ok();
    }

    let turn_id = format!("turn-{new_turn_index}");
    // FIX F5 (2026-05-18): pass the WORKSPACE-RELATIVE meta-prompt path to the
    // subprocess. `cmd_llm::complete_action` resolves any non-absolute
    // `--meta-prompt` value via `workspace.join(mp_path)` (informational sha256
    // for PromptCapsule.system_prompt_template_hash). F4 passed the already-
    // prefixed `<workspace>/assets/prompts/grill_meta_v1.md` here, which caused
    // the subprocess to re-prefix and ENOENT on
    // `<workspace>/<workspace>/assets/prompts/grill_meta_v1.md` →
    // `{kind: shellout_failed}`. Hardcode the canonical asset-relative path to
    // decouple from `meta_prompt_path` (which remains the CWD-relative read
    // path used above by F4).
    const META_PROMPT_REL: &str = "assets/prompts/grill_meta_v1.md";

    let complete_output = run_web_command(
        bin.clone(),
        session_dir.clone(),
        vec![
            "llm".into(),
            "complete".into(),
            "--workspace".into(),
            workspace.clone(),
            "--role".into(),
            "meta".into(),
            "--prompt-file".into(),
            prompt_file_path.to_string_lossy().into_owned(),
            "--strict-json".into(),
            "--capsule-dir".into(),
            capsules_dir.to_string_lossy().into_owned(),
            "--turn-id".into(),
            turn_id.clone(),
            "--lang".into(),
            lang.clone(),
            "--meta-prompt".into(),
            META_PROMPT_REL.into(),
        ],
        web_llm_child_env(),
        Duration::from_secs(240),
    )
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorBody::with_kind(
                format!("llm complete shellout failed: {e}"),
                "shellout_failed",
            )),
        )
    })?;
    if !complete_output.success() {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorBody::with_kind(
                format!(
                    "llm complete exit {:?}: {}",
                    complete_output.exit_code,
                    combine_truncated(&complete_output.stdout, &complete_output.stderr)
                ),
                "shellout_failed",
            )),
        ));
    }
    let complete_stdout = String::from_utf8_lossy(&complete_output.stdout).into_owned();

    // Parse the envelope
    let envelope = match parse_turn_payload_from_llm_output(&complete_stdout) {
        Ok(e) => e,
        Err(e) => {
            log::warn!("spec_turn_handler: llm complete parse error: {e}");
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorBody::with_kind(
                    format!("llm complete failed: {e}"),
                    "shellout_failed",
                )),
            ));
        }
    };

    // Extract fields from envelope
    let question_text = jstr(&envelope, "question", "").to_string();
    let (covered_slots, open_slots) = extract_slots(&envelope);
    let confidence = envelope
        .get("confidence")
        .and_then(|x| x.as_f64())
        .unwrap_or(0.0);
    let done = envelope
        .get("done")
        .and_then(|x| x.as_bool())
        .unwrap_or(false);
    let playback = if done {
        envelope.get("playback").cloned()
    } else {
        None
    };

    // Parse the turn_capsule_cid if present in stdout
    let turn_capsule_cid = parse_turn_cid_from_llm_output(&complete_stdout);

    // ── Step 11: update session state ─────────────────────────────────────────
    {
        let mut sessions = state.sessions.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(sess) = sessions.get_mut(&req.session_id) {
            sess.turn_count = new_turn_index;
            sess.meta_turns_accepted += 1;
            sess.last_prev_covered = covered_slots.clone();
            // FIX F6 (2026-05-18): persist the Meta-emitted question so the
            // next POST /api/spec/turn can pass it as `--question` to triage.
            // See the Step 9 prev_question derivation for the failure mode
            // this fixes.
            sess.last_question_emitted = question_text.clone();
            if let Some(ref cid) = turn_capsule_cid {
                sess.turn_cids.push(cid.clone());
                sess.parent_turn_cid = Some(cid.clone());
            }
            // Update coverage state
            for slot in &covered_slots {
                sess.coverage_state
                    .insert(slot.clone(), SlotState::Satisfied);
            }
            // FIX F9 (2026-05-19): persist the just-completed (q, a) pair
            // NOW — after `llm complete` succeeded — instead of before
            // the call (the pre-F9 location at Step 9's relevant
            // branch). This is the rollback-safe location: a transient
            // upstream failure on the call above returns HTTP 500 from
            // an earlier `?` without reaching this block, leaving
            // `last_3_turns` and `all_user_answers` untouched, so the
            // client's natural retry replays cleanly without
            // duplicating the user turn in the next prompt JSON.
            //
            // The push order mirrors the CLI driven path
            // (`cmd_spec.rs:1288-1289`): rolling-3 window first, then
            // the full ordered history used by A6 in-process spec.md
            // synthesis (step 13 below).
            if let Some(user_answer) = req.user_answer.as_deref() {
                if sess.last_3_turns.len() == 3 {
                    sess.last_3_turns.pop_front();
                }
                sess.last_3_turns
                    .push_back((prev_question.clone(), user_answer.to_string()));
                sess.all_user_answers.push(user_answer.to_string());

                // FIX F10 (2026-05-19): attribute this turn's user_answer to
                // the slot(s) the Meta LLM just credited via its `covered_slots`
                // delta. `covered_slots` is cumulative + monotonic (Meta-prompt
                // OUTPUT CONTRACT line ~70), so the slot(s) NEWLY appearing
                // this turn (vs `last_prev_covered_snap` taken before the
                // call) are exactly the slot(s) the user just populated.
                //
                // This is the load-bearing fix for D-NEW-3a: positional
                // indexing into `all_user_answers` collapses when the LLM
                // asks slots in non-canonical adaptive order (Π4.3 P7 +
                // Π4.4 S11 evidence). The slot-keyed map drives the new
                // `synthesise_spec_md_no_llm_by_slot` path in step 13.
                //
                // If multiple slots are newly covered in one turn (rare —
                // the meta prompt forbids it but the LLM can violate),
                // the same user_answer is recorded under each. If zero
                // are newly covered (turn rejected by the LLM as too vague
                // — it asked a follow-up under the SAME slot), nothing is
                // recorded for this turn; the next turn's user answer
                // will overwrite when it lands the slot.
                let prev_set: std::collections::HashSet<&str> =
                    last_prev_covered_snap.iter().map(|s| s.as_str()).collect();
                for slot in &covered_slots {
                    if !prev_set.contains(slot.as_str()) {
                        sess.slot_evidence
                            .insert(slot.clone(), user_answer.to_string());
                    }
                }
            }
        }
    }

    // ── Step 12: broadcast SpecTurnAdvanced ──────────────────────────────────
    let _ = state
        .broadcast_tx
        .send(super::ws::WsBroadcastMsg::SpecTurnAdvanced {
            session_id: req.session_id.clone(),
            turn_index: new_turn_index,
            question_text: question_text.clone(),
        });

    // ── Step 13: termination check ────────────────────────────────────────────
    let mut spec_capsule_cid: Option<String> = None;
    let mut is_terminated = false;
    let mut termination_reason: Option<String> = None;

    if done {
        // FIX A6 (2026-05-19): close the F6 deferred atom by synthesising
        // the spec.md and writing the SpecCapsule + GrillSessionCapsule
        // IN-PROCESS, using the library-ized `turingosv4::runtime::spec_capsule`
        // + `spec_synthesis` surfaces. Pre-A6, this branch returned
        // `spec_capsule_cid: None` + `termination_reason:
        // "predicate_done_no_spec_pending_synthesis"` because the synthesis
        // helpers + CAS writers lived inside the `turingos` binary crate and
        // were unreachable from `turingos_web`. The library-ization atom
        // moved `spec_capsule.rs` to `src/runtime/spec_capsule.rs` and lifted
        // the pure synthesis helpers to `src/runtime/spec_synthesis.rs`; this
        // handler now calls them the same way `cmd_spec::run_driven_mode`
        // does (cmd_spec.rs:1305-1378).
        //
        // We do NOT call the Meta synthesis LLM here — the web path stays
        // LLM-less to keep the request bounded and deterministic. The CLI
        // driven path tries LLM-first with `synthesise_spec_md_no_llm` as
        // the fallback; we use the fallback directly. The resulting spec.md
        // is byte-identical to what the CLI emits in --skip-llm mode.
        //
        // History notes:
        //   - F6 (2026-05-18) removed the pre-F6 broken shellout that
        //     synthesised fake `placeholder_<unix_secs_hex>` CIDs.
        //   - A6 (2026-05-19) finishes the wire-up by giving the web layer
        //     a real in-process path.
        use std::time::{SystemTime, UNIX_EPOCH};
        let logical_t = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        // Snapshot the session's lang + full answer history + slot_evidence
        // under one lock.
        let (lang_str, answers_snap, slot_evidence_snap, turn_cids_snap, total_turns_snap) = {
            let sessions = state.sessions.lock().unwrap_or_else(|e| e.into_inner());
            match sessions.get(&req.session_id) {
                Some(sess) => (
                    sess.lang.clone(),
                    sess.all_user_answers.clone(),
                    sess.slot_evidence.clone(),
                    sess.turn_cids.clone(),
                    sess.turn_count,
                ),
                None => (
                    "zh".to_string(),
                    Vec::new(),
                    std::collections::BTreeMap::new(),
                    Vec::new(),
                    new_turn_index,
                ),
            }
        };

        let lang = match lang_str.as_str() {
            "en" | "english" => turingosv4::runtime::grill_predicates::Lang::En,
            _ => turingosv4::runtime::grill_predicates::Lang::Zh,
        };
        let questions = turingosv4::runtime::spec_synthesis::canonical_questions(lang);
        let answers = turingosv4::runtime::spec_synthesis::pad_answers_to_8(answers_snap);

        // FIX F10 (2026-05-19): use the slot-keyed synthesiser, which looks up
        // each canonical slot in `slot_evidence` rather than indexing
        // positionally into `answers`. This eliminates the D-NEW-3a slot-shift
        // bug surfaced by Π4.3 P7 + Π4.4 S11. The Q/A appendix below still
        // uses the positional `(questions, answers)` pair so the audit trail
        // preserves the chronological interview transcript.
        let body = turingosv4::runtime::spec_synthesis::synthesise_spec_md_no_llm_by_slot(
            lang,
            &slot_evidence_snap,
        );
        let spec_md = turingosv4::runtime::spec_synthesis::wrap_spec_md(
            &body,
            &questions,
            &answers,
            "web-skip-llm",
            true,
        );

        // Persist spec.md to the session-scoped subdirectory for parity with
        // the CLI driven path (cmd_spec.rs:1353-1355 writes
        // `<workspace>/spec.md` — here we use the per-session subdir so the
        // top-level workspace stays usable for other sessions).
        let session_dir = std::path::PathBuf::from(&workspace)
            .join("sessions")
            .join(&req.session_id);
        // Best-effort: missing parent dir or write failure is logged but
        // does NOT block CAS-capsule creation (CAS is the canonical record).
        let _ = std::fs::create_dir_all(&session_dir);
        let spec_md_path = session_dir.join("spec.md");
        if let Err(e) = std::fs::write(&spec_md_path, &spec_md) {
            eprintln!(
                "[a6 synth] WARN: failed to write spec.md to {}: {e}",
                spec_md_path.display()
            );
        }

        // Write SpecCapsule to CAS.
        let workspace_path = std::path::Path::new(&workspace);
        let synth_outcome = turingosv4::runtime::spec_capsule::write_spec_capsule(
            workspace_path,
            &spec_md,
            "grill_driven_web",
            logical_t,
        );

        let final_spec_capsule_cid_hex: String = match synth_outcome {
            Ok(cid_hex) => {
                eprintln!(
                    "[a6 synth] wrote SpecCapsule CID={cid_hex} session_id={}",
                    req.session_id
                );
                spec_capsule_cid = Some(cid_hex.clone());
                cid_hex
            }
            Err(e) => {
                eprintln!(
                    "[a6 synth] ERROR: write_spec_capsule failed for session_id={}: {e}",
                    req.session_id
                );
                spec_capsule_cid = None;
                String::new()
            }
        };

        // Mirror the SpecCapsule into the session-scoped CAS so that
        // `turingos generate --workspace <workspace>/sessions/<session_id>
        // --from-capsule` can resolve it via `spec_capsule::latest_spec_capsule_cid`,
        // which probes `<workspace_arg>/cas/`. The generate web handler
        // (src/web/generate.rs) shells out with the per-session workspace, so
        // without this mirror the lookup returns `NoSpec("no spec capsule in
        // <session_dir>/cas")` even though the capsule was just written to the
        // global CAS above. CID is content-addressed → both copies share the
        // same CID; the duplicated bytes are the cost of keeping the two CLI
        // entry points (per-session generate vs global welcome/audit) reading
        // from disjoint CAS roots.
        if !final_spec_capsule_cid_hex.is_empty() {
            if let Err(e) = turingosv4::runtime::spec_capsule::write_spec_capsule(
                &session_dir,
                &spec_md,
                "grill_driven_web",
                logical_t,
            ) {
                eprintln!(
                    "[a6 synth] WARN: failed to mirror SpecCapsule into session CAS {}: {e}",
                    session_dir.display()
                );
            }
        }

        // Build + write GrillSessionCapsule (manifest of turn CIDs + tally).
        // Mirrors `cmd_spec.rs:1366-1378`.
        let (tally_snap, partial_session) = {
            let sessions = state.sessions.lock().unwrap_or_else(|e| e.into_inner());
            match sessions.get(&req.session_id) {
                Some(sess) => (
                    turingosv4::runtime::spec_capsule::GrillAttemptTally {
                        meta_turns_accepted: sess.meta_turns_accepted,
                        meta_turns_rejected: sess.meta_turns_rejected,
                        triage_calls_relevant: sess.triage_calls_relevant,
                        triage_calls_non_relevant: sess.triage_calls_non_relevant,
                        synthesis_calls: if final_spec_capsule_cid_hex.is_empty() {
                            0
                        } else {
                            1
                        },
                    },
                    final_spec_capsule_cid_hex.is_empty(),
                ),
                None => (
                    turingosv4::runtime::spec_capsule::GrillAttemptTally::default(),
                    final_spec_capsule_cid_hex.is_empty(),
                ),
            }
        };

        let session_body = turingosv4::runtime::spec_capsule::GrillSessionCapsuleBody {
            session_id: req.session_id.clone(),
            turn_cids: turn_cids_snap,
            final_spec_capsule_cid: final_spec_capsule_cid_hex.clone(),
            termination_reason: if final_spec_capsule_cid_hex.is_empty() {
                "predicate_done_synth_failed".to_string()
            } else {
                "llm_done_predicate_pass".to_string()
            },
            total_turns: total_turns_snap,
            partial_session,
            lang: lang_str.clone(),
            grill_attempt_tally: tally_snap,
            logical_t,
        };
        if let Err(e) = turingosv4::runtime::spec_capsule::write_grill_session_capsule(
            workspace_path,
            &session_body,
        ) {
            eprintln!(
                "[a6 synth] WARN: write_grill_session_capsule failed for session_id={}: {e}",
                req.session_id
            );
        }

        is_terminated = true;
        termination_reason = Some(if final_spec_capsule_cid_hex.is_empty() {
            // Synthesis fired but CAS write failed — surface that distinctly
            // rather than the pre-A6 "_pending_synthesis" placeholder.
            "predicate_done_synth_failed".to_string()
        } else {
            "llm_done_predicate_pass".to_string()
        });

        {
            let mut sessions = state.sessions.lock().unwrap_or_else(|e| e.into_inner());
            if let Some(sess) = sessions.get_mut(&req.session_id) {
                sess.terminated = true;
            }
        }

        let _ = state
            .broadcast_tx
            .send(super::ws::WsBroadcastMsg::SpecGrillComplete {
                session_id: req.session_id.clone(),
                spec_capsule_cid: final_spec_capsule_cid_hex,
            });
    }

    // ── Step 14: hard turn ceiling post-check ────────────────────────────────
    if new_turn_index >= 15 && !done {
        is_terminated = true;
        // FIX F6 (2026-05-18): be explicit about WHICH termination this is.
        // Pre-F6 the response carried `terminated: true` with no signal
        // about cause, leaving the UI to guess between "ceiling fired",
        // "predicate fail", and "user terminated".
        termination_reason = Some("turn_ceiling_15_no_spec".to_string());
        {
            let mut sessions = state.sessions.lock().unwrap_or_else(|e| e.into_inner());
            if let Some(sess) = sessions.get_mut(&req.session_id) {
                sess.terminated = true;
            }
        }
        let _ = state
            .broadcast_tx
            .send(super::ws::WsBroadcastMsg::SpecGrillComplete {
                session_id: req.session_id.clone(),
                spec_capsule_cid: String::new(),
            });
    }

    Ok(Json(SpecTurnResponse {
        turn_index: new_turn_index,
        question_text,
        covered_slots,
        open_slots,
        confidence,
        done,
        playback,
        terminated: is_terminated,
        spec_capsule_cid,
        turn_capsule_cid,
        termination_reason,
        triage_class: None,
    }))
}

// ---------------------------------------------------------------------------
// W7 helpers
// ---------------------------------------------------------------------------

/// Build a coverage summary string (injected into the prompt JSON).
///
/// FIX F3 (2026-05-18): vocabulary must mirror the canonical grill substrate.
/// The previous inline list used Researcher-C's draft vocabulary
/// (`job_story / data_model / first_click / weird_user /
/// disappointment_boundary / success_test / playback`). The `coverage_state`
/// HashMap is keyed by the canonical names the LLM actually emits
/// (`job / anchor / memory / first_run / robustness / scope / acceptance /
/// mirror`, from `grill_envelope::CANONICAL_SLOTS`), so the lookup
/// `coverage_state.get("job_story")` always returned `None` and every slot
/// was rendered to the LLM as `[ ]` (uncovered) regardless of actual coverage.
/// That broke the LLM's view of progress on the prompt-assembly path: the
/// model could never see what it had already covered, so it asked redundant
/// questions and could not confidently emit `done=true`.
///
/// All 8 canonical slots are rendered (including `mirror`, which is canonical
/// but optional per `REQUIRED_SLOTS`). The LLM needs to see the full slate to
/// decide both which slots to push on next and whether `mirror` has been
/// touched.
#[cfg(feature = "web")]
fn build_coverage_summary(
    coverage_state: &std::collections::HashMap<String, super::ws::SlotState>,
    turn_count: u32,
) -> String {
    let mut parts = Vec::new();
    for slot in CANONICAL_SLOTS {
        let mark = match coverage_state.get(*slot) {
            Some(super::ws::SlotState::Satisfied) => "[x]",
            Some(super::ws::SlotState::Partial) => "[~]",
            _ => "[ ]",
        };
        parts.push(format!("{mark} {slot}"));
    }
    format!(
        "Coverage state (turn {}):\n{}\nTurns used: {}",
        turn_count,
        parts.join("\n"),
        turn_count
    )
}

/// Build the prompt JSON for a driven-mode web turn.
///
/// FIX F4 (2026-05-18): the previous implementation dropped the meta-prompt
/// from the `messages` array on the assumption that the shell-out to
/// `turingos llm complete --meta-prompt <path>` would inject it server-side.
/// It does not — `cmd_llm.rs` treats `--meta-prompt` as informational only
/// (the path's sha256 lands in the prompt capsule as
/// `system_prompt_template_hash`, but the bytes are never spliced into the
/// outgoing `chat_messages`). The CLI driven path at
/// `cmd_spec.rs::build_turn_prompt_json` correctly prepends the meta-prompt
/// content as `messages[0]` with role `system`; this function now mirrors
/// that exactly. Without it, the LLM receives only the coverage summary +
/// turn instruction, has no contract to follow, and the strict-JSON parser
/// rejects whatever free-form prose it emits — surfacing as HTTP 500
/// `{kind: shellout_failed}` on the very first POST `/api/spec/turn` (W9
/// real-LLM verdict: FAIL).
#[cfg(feature = "web")]
fn build_web_turn_prompt_json(
    meta_prompt_content: &str,
    coverage_summary: &str,
    last_3_turns: &std::collections::VecDeque<(String, String)>,
    turn_index: u32,
    extra_system: Option<&str>,
) -> String {
    let mut messages: Vec<serde_json::Value> = Vec::new();

    // 1. System: meta-prompt content (the interviewer contract).
    messages.push(serde_json::json!({
        "role": "system",
        "content": meta_prompt_content,
    }));

    // 2. System: coverage state summary.
    messages.push(serde_json::json!({
        "role": "system",
        "content": coverage_summary,
    }));

    // 3. Optional extra system message (e.g. predicate failure nudge).
    if let Some(extra) = extra_system {
        messages.push(serde_json::json!({
            "role": "system",
            "content": extra,
        }));
    }

    // 4. Last 3 accepted turns as alternating assistant/user pairs.
    for (q, a) in last_3_turns.iter() {
        messages.push(serde_json::json!({
            "role": "assistant",
            "content": q,
        }));
        messages.push(serde_json::json!({
            "role": "user",
            "content": a,
        }));
    }

    // 5. Final user instruction.
    messages.push(serde_json::json!({
        "role": "user",
        "content": format!("Produce your turn-{turn_index} output per the contract."),
    }));

    serde_json::json!({ "messages": messages }).to_string()
}

// ---------------------------------------------------------------------------
// Unit tests (no I/O)
// ---------------------------------------------------------------------------

#[cfg(all(feature = "web", test))]
mod tests {
    use super::*;

    #[test]
    fn is_safe_session_id_accepts_valid() {
        assert!(is_safe_session_id("abc123"));
        assert!(is_safe_session_id("session-01"));
        assert!(is_safe_session_id("1716000000_3f8a1b2c"));
        assert!(is_safe_session_id(&"a".repeat(128)));
    }

    #[test]
    fn is_safe_session_id_rejects_traversal() {
        assert!(!is_safe_session_id("../etc/passwd"));
        assert!(!is_safe_session_id("a/b"));
        assert!(!is_safe_session_id("a.b"));
        assert!(!is_safe_session_id(""));
        assert!(!is_safe_session_id(&"a".repeat(129)));
    }

    #[test]
    fn parse_capsule_cid_from_stdout_finds_cid() {
        let stdout = "Spec interview complete.\n  spec.md            -> /tmp/x/spec.md\n  CAS capsule CID    -> deadbeef1234\n";
        assert_eq!(
            parse_capsule_cid_from_stdout(stdout),
            Some("deadbeef1234".to_string())
        );
    }

    #[test]
    fn parse_capsule_cid_from_stdout_returns_none_on_no_match() {
        assert_eq!(parse_capsule_cid_from_stdout("no cid here\n"), None);
    }

    #[test]
    fn extract_slots_uses_canonical_vocab_and_computes_open() {
        // Regression: prior to fix F1, extract_slots used Researcher-C's draft
        // vocabulary (job_story / data_model / first_click / weird_user /
        // disappointment_boundary / success_test / playback). That set never
        // matched the LLM's emitted slot ids (job / anchor / memory /
        // first_run / robustness / scope / acceptance / mirror) defined in
        // grill_envelope::CANONICAL_SLOTS, so the WS broadcast `open_slots`
        // field was always wrong (always returned the junk list, minus
        // "anchor" if covered). This test pins the canonical vocab.
        use turingosv4::runtime::grill_envelope::CANONICAL_SLOTS as CANON;

        let envelope = serde_json::json!({
            "covered_slots": ["job", "anchor", "memory"],
        });
        let (covered, open) = extract_slots(&envelope);

        assert_eq!(
            covered,
            vec![
                "job".to_string(),
                "anchor".to_string(),
                "memory".to_string()
            ],
            "covered passthrough mismatch"
        );

        // Open must be exactly CANON minus covered, with no junk vocab.
        let open_set: std::collections::BTreeSet<String> = open.iter().cloned().collect();
        let expected_open: std::collections::BTreeSet<String> =
            ["first_run", "robustness", "scope", "acceptance", "mirror"]
                .iter()
                .map(|s| s.to_string())
                .collect();
        assert_eq!(
            open_set, expected_open,
            "open set must be CANONICAL_SLOTS minus covered, with no draft vocab"
        );
        assert_eq!(open.len(), 5, "expected 5 open slots, got {}", open.len());

        // And every returned name must be in the canonical set.
        for name in open.iter().chain(covered.iter()) {
            assert!(
                CANON.contains(&name.as_str()),
                "slot {name:?} is not in canonical vocabulary {CANON:?}"
            );
        }

        // Banned: none of the Researcher-C draft names may appear.
        for banned in [
            "job_story",
            "data_model",
            "first_click",
            "weird_user",
            "disappointment_boundary",
            "success_test",
            "playback",
        ] {
            assert!(
                !open.iter().any(|n| n == banned) && !covered.iter().any(|n| n == banned),
                "draft vocab {banned:?} leaked through extract_slots"
            );
        }
    }

    #[test]
    fn extract_slots_empty_covered_returns_full_canonical_open() {
        use turingosv4::runtime::grill_envelope::CANONICAL_SLOTS as CANON;
        let envelope = serde_json::json!({ "covered_slots": [] });
        let (covered, open) = extract_slots(&envelope);
        assert!(covered.is_empty());
        assert_eq!(open.len(), CANON.len());
        let open_set: std::collections::BTreeSet<&str> = open.iter().map(|s| s.as_str()).collect();
        let canon_set: std::collections::BTreeSet<&str> = CANON.iter().copied().collect();
        assert_eq!(open_set, canon_set);
    }

    #[test]
    fn build_coverage_summary_uses_canonical_vocab_not_draft() {
        // Regression: prior to fix F3, build_coverage_summary used Researcher-C's
        // draft slot vocabulary (job_story / data_model / first_click /
        // weird_user / disappointment_boundary / success_test / playback) to
        // iterate when rendering the coverage block injected into the LLM
        // prompt. The `coverage_state` HashMap, however, is keyed by the
        // canonical names the LLM actually emits (job / anchor / memory /
        // first_run / robustness / scope / acceptance / mirror, from
        // grill_envelope::CANONICAL_SLOTS). So every HashMap lookup returned
        // None, and the LLM was always told every slot was [ ] uncovered,
        // regardless of what had actually been collected. That forced the
        // LLM to ask redundant questions and never confidently terminate.
        use turingosv4::runtime::grill_envelope::CANONICAL_SLOTS as CANON;

        let mut coverage_state: std::collections::HashMap<String, crate::web::SlotState> =
            std::collections::HashMap::new();
        coverage_state.insert("job".to_string(), crate::web::SlotState::Satisfied);
        coverage_state.insert("anchor".to_string(), crate::web::SlotState::Partial);
        // memory / first_run / robustness / scope / acceptance / mirror
        // intentionally left out → should render as [ ].

        let summary = build_coverage_summary(&coverage_state, 3);

        // The summary must reflect ACTUAL canonical coverage, not all-empty.
        assert!(
            summary.contains("[x] job"),
            "canonical satisfied slot must render as [x]; got:\n{summary}"
        );
        assert!(
            summary.contains("[~] anchor"),
            "canonical partial slot must render as [~]; got:\n{summary}"
        );
        // Uncovered canonical slots render as [ ].
        for slot in [
            "memory",
            "first_run",
            "robustness",
            "scope",
            "acceptance",
            "mirror",
        ] {
            assert!(
                summary.contains(&format!("[ ] {slot}")),
                "canonical uncovered slot {slot:?} must render as [ ]; got:\n{summary}"
            );
        }

        // Turn count surfaces both in the header line and the trailer.
        assert!(summary.contains("turn 3"), "header must carry turn count");
        assert!(
            summary.contains("Turns used: 3"),
            "trailer must carry turn count"
        );

        // Every canonical slot id appears exactly once.
        for slot in CANON {
            let count = summary.matches(slot).count();
            assert_eq!(
                count, 1,
                "canonical slot {slot:?} must appear exactly once; got {count} in:\n{summary}"
            );
        }

        // No draft vocab may leak into the LLM's view.
        for banned in [
            "job_story",
            "data_model",
            "first_click",
            "weird_user",
            "disappointment_boundary",
            "success_test",
            "playback",
        ] {
            assert!(
                !summary.contains(banned),
                "draft vocab {banned:?} leaked into coverage_summary:\n{summary}"
            );
        }
    }

    #[test]
    fn build_coverage_summary_empty_state_lists_all_canonical_as_uncovered() {
        // Boundary: an empty coverage_state must produce all 8 canonical slots
        // rendered as [ ] (uncovered), in canonical order.
        use turingosv4::runtime::grill_envelope::CANONICAL_SLOTS as CANON;

        let coverage_state: std::collections::HashMap<String, crate::web::SlotState> =
            std::collections::HashMap::new();
        let summary = build_coverage_summary(&coverage_state, 0);

        for slot in CANON {
            assert!(
                summary.contains(&format!("[ ] {slot}")),
                "canonical slot {slot:?} must render as [ ] when state is empty; got:\n{summary}"
            );
        }
        // No [x] or [~] marks may appear when state is empty.
        assert!(
            !summary.contains("[x]"),
            "no slot should render [x] for empty state; got:\n{summary}"
        );
        assert!(
            !summary.contains("[~]"),
            "no slot should render [~] for empty state; got:\n{summary}"
        );

        // Canonical order must be preserved (job first, mirror last in the slot list).
        let job_pos = summary.find("[ ] job").expect("job must appear");
        let mirror_pos = summary.find("[ ] mirror").expect("mirror must appear");
        assert!(
            job_pos < mirror_pos,
            "canonical order must be preserved (job before mirror); got:\n{summary}"
        );
    }

    #[test]
    fn web_spec_turn_prompt_includes_meta_prompt() {
        // Regression: fix F4 (2026-05-18). Prior to this fix,
        // `build_web_turn_prompt_json` dropped the meta-prompt entirely on the
        // assumption that the shell-out to `turingos llm complete
        // --meta-prompt <path>` would inject it server-side. It does not:
        // cmd_llm.rs treats --meta-prompt as informational (hashed into the
        // prompt capsule, never spliced into the outgoing chat_messages). As
        // a result the LLM received only the coverage summary + turn
        // instruction, had no contract to follow, and the strict-JSON parser
        // rejected its prose — surfacing as HTTP 500
        // `{kind: shellout_failed}` on the first POST /api/spec/turn (W9 real-
        // LLM verdict: FAIL). This test pins the fix: the meta-prompt must
        // appear verbatim as a system message in the messages array.
        let meta = "# TuringOS Spec Grill — Meta Prompt v1\n\nYou are the interviewer.\n";
        let coverage = "Coverage state (turn 1):\n[ ] job\nTurns used: 1";
        let last3 = std::collections::VecDeque::<(String, String)>::new();

        let json_str = build_web_turn_prompt_json(meta, coverage, &last3, 1, None);
        let value: serde_json::Value = serde_json::from_str(&json_str)
            .expect("build_web_turn_prompt_json must emit valid JSON");
        let messages = value
            .get("messages")
            .and_then(|m| m.as_array())
            .expect("messages array must exist");

        let meta_found = messages.iter().any(|m| {
            m.get("role").and_then(|r| r.as_str()) == Some("system")
                && m.get("content")
                    .and_then(|c| c.as_str())
                    .map(|s| s == meta)
                    .unwrap_or(false)
        });
        assert!(
            meta_found,
            "messages array must include a system message whose content equals \
             the loaded meta-prompt; got:\n{json_str}"
        );
    }

    #[test]
    fn web_spec_turn_prompt_first_message_is_system_with_meta() {
        // Regression: fix F4 (2026-05-18). The meta-prompt must specifically
        // be messages[0] (role=system) so the LLM sees its contract before any
        // coverage state or user turn — this mirrors the CLI driven path in
        // cmd_spec.rs::build_turn_prompt_json exactly. We also assert message[1]
        // is the coverage summary (system) and the final message is the turn
        // instruction (user), to lock the canonical ordering.
        let meta = "# TuringOS Spec Grill — Meta Prompt v1\n\nROLE: interviewer.\n";
        let coverage = "Coverage state (turn 2):\n[x] job\n[ ] anchor\nTurns used: 2";
        let mut last3 = std::collections::VecDeque::<(String, String)>::new();
        last3.push_back(("prior question?".to_string(), "prior answer.".to_string()));

        let json_str = build_web_turn_prompt_json(meta, coverage, &last3, 2, None);
        let value: serde_json::Value =
            serde_json::from_str(&json_str).expect("must emit valid JSON");
        let messages = value
            .get("messages")
            .and_then(|m| m.as_array())
            .expect("messages array must exist");

        assert!(
            messages.len() >= 4,
            "expected at least 4 messages (meta, coverage, prior pair, final); got {}",
            messages.len()
        );

        // messages[0] MUST be the meta-prompt as a system message.
        assert_eq!(
            messages[0].get("role").and_then(|r| r.as_str()),
            Some("system"),
            "messages[0].role must be \"system\"; got:\n{json_str}"
        );
        let m0_content = messages[0]
            .get("content")
            .and_then(|c| c.as_str())
            .expect("messages[0].content must be string");
        assert!(
            m0_content.starts_with("# TuringOS Spec Grill"),
            "messages[0].content must start with meta-prompt header; got: {m0_content:?}"
        );
        assert_eq!(
            m0_content, meta,
            "messages[0].content must equal the loaded meta-prompt verbatim"
        );

        // messages[1] MUST be the coverage summary as a system message.
        assert_eq!(
            messages[1].get("role").and_then(|r| r.as_str()),
            Some("system"),
            "messages[1].role must be \"system\" (coverage summary)"
        );
        assert_eq!(
            messages[1].get("content").and_then(|c| c.as_str()),
            Some(coverage),
            "messages[1].content must be the coverage summary"
        );

        // Final message MUST be the turn instruction as user role.
        let last = messages.last().expect("must have at least one message");
        assert_eq!(
            last.get("role").and_then(|r| r.as_str()),
            Some("user"),
            "final message must be role=user"
        );
        assert!(
            last.get("content")
                .and_then(|c| c.as_str())
                .map(|s| s.contains("turn-2"))
                .unwrap_or(false),
            "final user message must reference turn-2; got: {last:?}"
        );
    }

    #[test]
    fn web_spec_turn_prompt_real_asset_loads_and_prepends() {
        // Belt-and-braces: read the actual asset that the production handler
        // reads (assets/prompts/grill_meta_v1.md). The web binary's CWD is the
        // repo root at runtime (W10-R1 O4 + W9 evidence); `cargo test` runs
        // from the repo root too, so the same relative path resolves here.
        //
        // If this test fails because the asset is missing, the production
        // handler will surface `{kind: prompt_asset_missing}` (new in F4) —
        // a far clearer signal than the prior `shellout_failed`.
        let asset_path = std::path::PathBuf::from("assets/prompts/grill_meta_v1.md");
        let meta = match std::fs::read_to_string(&asset_path) {
            Ok(s) => s,
            Err(e) => {
                // Skip silently if test is run from a non-repo-root CWD.
                eprintln!(
                    "skipping: meta-prompt asset {} not readable from cwd: {e}",
                    asset_path.display()
                );
                return;
            }
        };
        assert!(
            meta.contains("TuringOS Spec Grill"),
            "asset must contain the canonical header; got first 80 bytes: {:?}",
            &meta.chars().take(80).collect::<String>()
        );

        let coverage = "Coverage state (turn 1):\n[ ] job\nTurns used: 1";
        let last3 = std::collections::VecDeque::<(String, String)>::new();
        let json_str = build_web_turn_prompt_json(&meta, coverage, &last3, 1, None);
        let value: serde_json::Value =
            serde_json::from_str(&json_str).expect("must emit valid JSON");
        let messages = value
            .get("messages")
            .and_then(|m| m.as_array())
            .expect("messages array must exist");
        let m0 = messages[0]
            .get("content")
            .and_then(|c| c.as_str())
            .expect("messages[0].content must be string");
        assert_eq!(
            m0, meta,
            "the asset bytes must round-trip into messages[0].content verbatim"
        );
    }

    #[test]
    fn web_spec_turn_passes_workspace_relative_meta_prompt_arg() {
        // Regression: fix F5 (2026-05-18). Prior to this fix the
        // `--meta-prompt` argument forwarded to the `turingos llm complete`
        // subprocess was the same `PathBuf` produced for the in-process
        // `read_to_string` call — i.e. `<workspace>/assets/prompts/
        // grill_meta_v1.md` (F4). But `cmd_llm::complete_action` resolves any
        // non-absolute `--meta-prompt` value via `workspace.join(mp_path)`
        // (see cmd_llm.rs system_prompt_template_hash branch). That doubles
        // the workspace prefix:
        //   <workspace>/<workspace>/assets/prompts/grill_meta_v1.md  →  ENOENT
        // surfacing as HTTP 500 `{kind: shellout_failed}` on the first POST
        // /api/spec/turn (F4 web smoke FAIL).
        //
        // Fix: pass a workspace-RELATIVE literal to the subprocess. This test
        // pins that contract by mirroring the construction at the call site
        // and asserting:
        //   (a) the subprocess arg does NOT contain the workspace prefix
        //   (b) joining the arg under the workspace resolves to the same path
        //       that the in-process read_to_string uses (no double-prefix).
        use std::path::PathBuf;
        const META_PROMPT_REL: &str = "assets/prompts/grill_meta_v1.md";

        // Simulate the production-shaped workspace (matches W9/W10-R1 smoke).
        let workspace = "tmp/universality_campaign";

        // The path passed to `read_to_string` (F4 contract): full CWD-relative.
        let read_path = PathBuf::from(workspace).join(META_PROMPT_REL);

        // The path passed to the subprocess as `--meta-prompt` (F5 contract):
        // workspace-relative literal, never re-prefixed.
        let subprocess_arg: &str = META_PROMPT_REL;

        // (a) Subprocess arg must NOT carry the workspace prefix — that is
        // what produced the double-prefix ENOENT in F4.
        assert!(
            !subprocess_arg.contains(workspace),
            "subprocess --meta-prompt arg must be workspace-RELATIVE; \
             contained workspace prefix {workspace:?} which would cause \
             cmd_llm.rs to re-join → double-prefix ENOENT. arg: {subprocess_arg:?}"
        );
        assert!(
            !subprocess_arg.starts_with('/'),
            "subprocess --meta-prompt arg must NOT be absolute (cmd_llm \
             treats absolute paths as-is, but our convention is workspace-relative). \
             arg: {subprocess_arg:?}"
        );

        // (b) When cmd_llm resolves the arg via `workspace.join(mp_path)`, the
        // result must equal the F4 read path — i.e. resolve to the same asset
        // file, not a doubled-prefix path.
        let resolved = PathBuf::from(workspace).join(subprocess_arg);
        assert_eq!(
            resolved, read_path,
            "subprocess-resolved meta-prompt path must equal the in-process \
             read path; got resolved={:?} read_path={:?}",
            resolved, read_path
        );

        // Sentinel: the F4-shape (passing the full CWD-relative path) would
        // resolve to a doubled-prefix path under cmd_llm's `workspace.join`.
        // Pin that exact pathological shape so a future refactor that
        // reintroduces it trips this assertion.
        let f4_shape_doubled = PathBuf::from(workspace).join(&read_path);
        assert_ne!(
            resolved, f4_shape_doubled,
            "F5 fix must avoid the F4 double-prefix shape; got resolved={:?} \
             matched the pathological doubled path {:?}",
            resolved, f4_shape_doubled
        );
        assert!(
            f4_shape_doubled
                .to_string_lossy()
                .contains("tmp/universality_campaign/tmp/universality_campaign"),
            "sanity: the F4 shape must produce the doubled prefix path; got {:?}",
            f4_shape_doubled
        );
    }

    #[test]
    fn generate_session_id_format() {
        let sid = generate_session_id();
        // Must be safe as a directory name
        assert!(
            is_safe_session_id(&sid),
            "generated id {sid:?} must be safe"
        );
        // Format: <digits>_<8 hex chars>
        let parts: Vec<&str> = sid.splitn(2, '_').collect();
        assert_eq!(parts.len(), 2, "session_id must have underscore separator");
        assert!(
            parts[0].chars().all(|c| c.is_ascii_digit()),
            "first part must be digits; got {:?}",
            parts[0]
        );
        assert_eq!(parts[1].len(), 8, "hex suffix must be 8 chars");
        assert!(
            parts[1].chars().all(|c| c.is_ascii_hexdigit()),
            "hex suffix must be hex digits"
        );
    }

    // -----------------------------------------------------------------------
    // FIX F6 (2026-05-18) regression tests
    // -----------------------------------------------------------------------

    /// Regression: F6 Symptom A2 — `parse_triage_class_from_output` must
    /// treat `ok=false` as a subprocess failure (returning `Err`), NOT as a
    /// "gibberish" classification.
    ///
    /// Pre-F6 the parser pulled `class` via `unwrap_or("gibberish")` and
    /// happily returned `"gibberish"` on the failure shape `{ok:false,
    /// error:{kind,detail}}` — which the caller then interpreted as a
    /// triage-non-relevant verdict, bumping `non_relevant_count` and (after
    /// two consecutive transient triage subprocess failures) aborting the
    /// session with `terminated: true` + `spec_capsule_cid: null`. That is
    /// the W1.2 p1_backend failure mode at the parser-contract layer.
    #[test]
    fn parse_triage_class_treats_ok_false_as_error_not_gibberish() {
        // Shape emitted by cmd_llm::complete_err_exit on triage failure
        // (HTTP 5xx from SiliconFlow, transport timeout, JSON parse fail).
        let stdout =
            r#"{"ok":false,"error":{"kind":"http_status","detail":"HTTP 503: upstream busy"}}"#;
        let result = parse_triage_class_from_output(stdout);
        assert!(
            result.is_err(),
            "ok=false must return Err (not Ok(\"gibberish\")); got {result:?}"
        );
        // Sentinel: any future regression that re-introduces the
        // `unwrap_or("gibberish")` shape will trip this branch by returning
        // Ok("gibberish") instead of Err.
        assert!(
            !matches!(result, Ok(ref s) if s == "gibberish"),
            "ok=false must NOT silently coerce to \"gibberish\"; that was \
             the pre-F6 bug"
        );
        let err = result.unwrap_err();
        assert!(
            err.contains("ok=false"),
            "error string must surface ok=false fact; got: {err}"
        );
        assert!(
            err.contains("http_status"),
            "error string must include kind; got: {err}"
        );
        assert!(
            err.contains("503"),
            "error string must include the detail content; got: {err}"
        );
    }

    /// Regression: F6 Symptom A2 happy path — `ok=true` with a valid class
    /// must still return the class string (the new ok-check must not break
    /// the success case).
    #[test]
    fn parse_triage_class_returns_class_on_ok_true() {
        let stdout = r#"{"ok":true,"class":"relevant","confidence":0.9,"model":"X","usage":{"prompt_tokens":0,"completion_tokens":0,"total_tokens":0},"prompt_capsule_cid":null,"elapsed_ms":12}"#;
        assert_eq!(
            parse_triage_class_from_output(stdout).unwrap(),
            "relevant".to_string(),
        );

        let stdout_offtopic = r#"{"ok":true,"class":"off_topic","confidence":0.7,"model":"X","usage":{"prompt_tokens":0,"completion_tokens":0,"total_tokens":0},"prompt_capsule_cid":null,"elapsed_ms":12}"#;
        assert_eq!(
            parse_triage_class_from_output(stdout_offtopic).unwrap(),
            "off_topic".to_string(),
        );
    }

    /// Regression: F6 Symptom A2 edge — `ok=true` with no `class` field is
    /// a contract violation by the CLI; must surface as Err rather than
    /// silently degrading to "gibberish".
    #[test]
    fn parse_triage_class_errors_when_ok_true_but_class_missing() {
        let stdout = r#"{"ok":true,"confidence":0.5}"#;
        let result = parse_triage_class_from_output(stdout);
        assert!(result.is_err(), "ok=true with no class must be Err");
    }

    /// Regression: F6 Symptom A1 — `last_question_emitted` must be a field
    /// on `GrillSession` (the new persistence slot for the Meta-emitted
    /// question, used as `--question` for triage on the NEXT turn).
    ///
    /// Pre-F6, the handler derived `prev_question` from
    /// `last_3_turns.back().0` which always stored a one-turn-stale value
    /// (or "" on turn 1). With empty `--question`, the Blackbox triage model
    /// classified terse Chinese answers as non-relevant ~30-50% of the time
    /// (W1.1 mrs_chen), surfacing as HTTP 200 with all-zero response fields
    /// (the triage non-relevant bounce-back path).
    ///
    /// This test pins:
    ///   1. The field exists on `GrillSession`.
    ///   2. Default is empty string on a freshly-inserted session.
    ///   3. The field is `Clone`-able (snapshot semantics in step 7 read).
    #[test]
    fn grill_session_has_last_question_emitted_field() {
        use crate::web::ws::{GrillSession, SlotState};
        use std::collections::{HashMap, VecDeque};

        let s = GrillSession {
            session_id: "f6-test".into(),
            turn_count: 0,
            lang: "zh".into(),
            coverage_state: HashMap::<String, SlotState>::new(),
            last_3_turns: VecDeque::new(),
            turn_cids: vec![],
            terminated: false,
            parent_turn_cid: None,
            created_at_unix: 0,
            non_relevant_count: 0,
            last_prev_covered: vec![],
            meta_turns_accepted: 0,
            meta_turns_rejected: 0,
            triage_calls_relevant: 0,
            triage_calls_non_relevant: 0,
            last_question_emitted: String::new(),
            all_user_answers: Vec::new(),
            slot_evidence: std::collections::BTreeMap::new(),
        };

        // Default empty on new session.
        assert_eq!(
            s.last_question_emitted, "",
            "new GrillSession must have empty last_question_emitted"
        );
        assert!(
            s.all_user_answers.is_empty(),
            "new GrillSession must have empty all_user_answers history"
        );
        assert!(
            s.slot_evidence.is_empty(),
            "new GrillSession must have empty slot_evidence map (F10)"
        );

        // Field is clonable (needed for the step-7 snapshot read pattern).
        let snap: String = s.last_question_emitted.clone();
        assert_eq!(snap, "");
    }

    /// F10 (2026-05-19) regression: the covered-slot delta logic in
    /// `spec_turn_handler` step 11 must attribute the current turn's
    /// user_answer to the slot(s) NEWLY appearing in `covered_slots`
    /// (vs `last_prev_covered`), so the slot-keyed synthesiser receives
    /// the correct slot→answer mapping even when the LLM asks slots in
    /// non-canonical adaptive order.
    ///
    /// This pins the delta computation directly (the handler shells out
    /// to a real binary so we can't easily run the full HTTP path in a
    /// unit test).
    #[test]
    fn slot_evidence_attribution_uses_covered_slot_delta() {
        use std::collections::BTreeMap;

        // Simulate the Π4.3 P7 sequence at turn 4:
        //   - previous cumulative covered_slots: ["job", "anchor"]
        //   - this turn's cumulative covered_slots: ["job", "anchor", "memory"]
        //   - user_answer this turn: "Redis 存任务状态"
        // Expected: slot_evidence["memory"] = "Redis 存任务状态".
        let mut slot_evidence: BTreeMap<String, String> = BTreeMap::new();
        slot_evidence.insert("job".into(), "做影片转档".into());
        slot_evidence.insert("anchor".into(), "SHA256".into());

        let last_prev_covered = vec!["job".to_string(), "anchor".to_string()];
        let covered_slots = vec![
            "job".to_string(),
            "anchor".to_string(),
            "memory".to_string(),
        ];
        let user_answer = "Redis 存任务状态";

        let prev_set: std::collections::HashSet<&str> =
            last_prev_covered.iter().map(|s| s.as_str()).collect();
        for slot in &covered_slots {
            if !prev_set.contains(slot.as_str()) {
                slot_evidence.insert(slot.clone(), user_answer.to_string());
            }
        }

        assert_eq!(
            slot_evidence.get("memory").map(String::as_str),
            Some("Redis 存任务状态")
        );
        // Pre-existing slots must NOT be overwritten by the delta logic
        // (only newly-covered slots are written).
        assert_eq!(
            slot_evidence.get("job").map(String::as_str),
            Some("做影片转档")
        );
        assert_eq!(
            slot_evidence.get("anchor").map(String::as_str),
            Some("SHA256")
        );

        // Now simulate a turn where the LLM rejected the answer as too vague
        // and asked a follow-up under the SAME slot: covered_slots unchanged.
        let last_prev_covered = covered_slots.clone();
        let covered_slots = vec![
            "job".to_string(),
            "anchor".to_string(),
            "memory".to_string(),
        ];
        let user_answer = "memory follow-up answer — too vague";
        let prev_set: std::collections::HashSet<&str> =
            last_prev_covered.iter().map(|s| s.as_str()).collect();
        let prior_memory = slot_evidence.get("memory").cloned();
        for slot in &covered_slots {
            if !prev_set.contains(slot.as_str()) {
                slot_evidence.insert(slot.clone(), user_answer.to_string());
            }
        }
        // No new slot — slot_evidence["memory"] preserved.
        assert_eq!(slot_evidence.get("memory").cloned(), prior_memory);
    }

    /// Regression: F6 Symptom A1 — the `prev_question` derivation in step 9
    /// must prefer `last_question_emitted` (the freshly persisted slot)
    /// over `last_3_turns.back().0` (the pre-F6 stale source).
    ///
    /// We can't easily exercise the full handler in a unit test (it shells
    /// out to a real binary), so this test pins the derivation logic
    /// directly by replicating it: the fallback ONLY fires when
    /// `last_question_emitted_snap` is empty.
    #[test]
    fn prev_question_prefers_last_question_emitted_over_last_3_turns() {
        let last_3_turns_snap: std::collections::VecDeque<(String, String)> = {
            let mut v = std::collections::VecDeque::new();
            v.push_back((
                "STALE prior prev_question".to_string(),
                "answer N-1".to_string(),
            ));
            v
        };

        // Case 1: fresh field populated — use it, ignore the stale last_3_turns slot.
        let last_question_emitted_snap = "FRESH question just emitted by Meta".to_string();
        let prev_question = if !last_question_emitted_snap.is_empty() {
            last_question_emitted_snap.clone()
        } else {
            last_3_turns_snap
                .back()
                .map(|(q, _)| q.clone())
                .unwrap_or_default()
        };
        assert_eq!(
            prev_question, "FRESH question just emitted by Meta",
            "step 9 must prefer last_question_emitted when non-empty"
        );

        // Case 2: fresh field empty (legacy session pre-F6 deploy) — fall back.
        let last_question_emitted_snap = String::new();
        let prev_question = if !last_question_emitted_snap.is_empty() {
            last_question_emitted_snap.clone()
        } else {
            last_3_turns_snap
                .back()
                .map(|(q, _)| q.clone())
                .unwrap_or_default()
        };
        assert_eq!(
            prev_question, "STALE prior prev_question",
            "step 9 must fall back to last_3_turns when fresh field is empty"
        );

        // Case 3: both empty (first turn ever) — empty string.
        let empty_deque: std::collections::VecDeque<(String, String)> =
            std::collections::VecDeque::new();
        let last_question_emitted_snap = String::new();
        let prev_question = if !last_question_emitted_snap.is_empty() {
            last_question_emitted_snap.clone()
        } else {
            empty_deque
                .back()
                .map(|(q, _)| q.clone())
                .unwrap_or_default()
        };
        assert_eq!(prev_question, "");
    }

    /// Regression: F6 Symptom B + C — `SpecTurnResponse` must carry an
    /// optional `termination_reason` field, and it must serialize ONLY when
    /// populated (skip-if-none).
    ///
    /// Pre-F6 the response had no signal at all for "terminated without
    /// spec" — the UI had to infer it from `spec_capsule_cid == null` AND
    /// `terminated == true`, which was indistinguishable from "spec still
    /// synthesizing" (the pre-F6 happy-path shellout was async-fire-and-
    /// forget). F6 adds explicit termination signaling.
    #[test]
    fn spec_turn_response_carries_termination_reason_when_present() {
        let resp = SpecTurnResponse {
            turn_index: 7,
            question_text: String::new(),
            covered_slots: vec![],
            open_slots: vec![],
            confidence: 0.0,
            done: false,
            playback: None,
            terminated: true,
            spec_capsule_cid: None,
            turn_capsule_cid: None,
            termination_reason: Some("user_input_unparseable_no_spec".to_string()),
        };
        let json = serde_json::to_string(&resp).unwrap();
        assert!(
            json.contains("\"termination_reason\":\"user_input_unparseable_no_spec\""),
            "termination_reason must appear in JSON when Some; got: {json}"
        );

        // Skipped when None (the dominant case: normal in-progress turns).
        let resp_none = SpecTurnResponse {
            turn_index: 3,
            question_text: "...".to_string(),
            covered_slots: vec!["job".to_string()],
            open_slots: vec!["anchor".to_string()],
            confidence: 0.3,
            done: false,
            playback: None,
            terminated: false,
            spec_capsule_cid: None,
            turn_capsule_cid: None,
            termination_reason: None,
        };
        let json_none = serde_json::to_string(&resp_none).unwrap();
        assert!(
            !json_none.contains("termination_reason"),
            "termination_reason must be SKIPPED from JSON when None; got: {json_none}"
        );
    }

    /// Regression: F6 documents the canonical set of termination_reason
    /// values the handler may emit. Pin them so the frontend can switch on
    /// the string exhaustively. Any addition here is an intentional UX
    /// contract change.
    #[test]
    fn termination_reason_canonical_values() {
        // The three reasons F6 emits from spec_turn_handler. The
        // `_no_spec` suffix is intentional — it makes "terminated, no
        // spec was produced" visually obvious in logs and UI.
        const CANONICAL_REASONS: &[&str] = &[
            "turn_ceiling_15_no_spec",
            "user_input_unparseable_no_spec",
            "predicate_done_no_spec_pending_synthesis",
        ];

        for r in CANONICAL_REASONS {
            assert!(!r.is_empty(), "reason {r:?} must be non-empty");
            // All reasons end in _no_spec (advertising the lack of CID).
            assert!(
                r.ends_with("_no_spec") || r.ends_with("_no_spec_pending_synthesis"),
                "reason {r:?} must carry the _no_spec suffix"
            );
        }

        // Defensive: the broken pre-F6 shellout was trying to pass
        // "user_input_unparseable" (no suffix) on the CLI — make sure
        // the F6 string never collides with the pre-F6 ghost token.
        for r in CANONICAL_REASONS {
            assert_ne!(
                *r, "user_input_unparseable",
                "F6 reason must include the _no_spec marker to avoid \
                 collision with the dead pre-F6 CLI flag value"
            );
        }
    }

    /// Regression: F6 Symptom C — the placeholder_cid helper is no longer
    /// used in production but is kept for audit. If a future refactor
    /// re-introduces it as a fake-CID synthesizer, the placeholder format
    /// must still be recognizable as fake (prefix `placeholder_`) so audit
    /// can grep for it.
    #[test]
    fn placeholder_cid_format_stays_stable() {
        let cid = placeholder_cid();
        assert!(
            cid.starts_with("placeholder_"),
            "placeholder_cid must remain recognizably fake; got: {cid}"
        );
        // 12 chars prefix + 16 hex digits (unix secs in hex).
        assert_eq!(
            cid.len(),
            "placeholder_".len() + 16,
            "placeholder_cid length must be stable: {cid}"
        );
    }
}
