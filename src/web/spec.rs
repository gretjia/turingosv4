/// TRACE_MATRIX FC1-N5 + FC2-N16: Phase 7 W5 — spec interview endpoints
///
/// Two routes exposed:
///   GET  /api/spec/questions  → return the 8 canonical interview questions
///   POST /api/spec/submit     → accept all 8 answers, shell out to
///                               `turingos spec --workspace <session-dir>
///                                             --answers-file <session-dir>/answers.json
///                                             --lang zh`
///                               on exit 0: read spec.md, parse CAS CID from
///                               stdout, broadcast SpecComplete to WS channel.
///
/// FC-trace: FC1-N5 (read-view shielding at trust boundary) +
///           FC2-N16 (write action via existing Phase 6.3 CLI shellout; no new
///           Class-4 admission; spec capsule anchoring is a CAS-N write through
///           the Phase 6.3 substrate).
/// Risk class: Class 2-3.
///
/// # The 8 spec questions
///
/// The canonical questions live in `src/bin/turingos/cmd_spec.rs` inside
/// `canonical_questions(Lang::Zh)`. They are duplicated here as a static
/// const array so the web server can serve them via GET /api/spec/questions
/// without depending on the CLI binary being in PATH at query time.
///
/// /// TRACE_MATRIX FC1-N5: duplication rationale — source of truth is
/// cmd_spec.rs; this copy is a read-only materialized view for the frontend.
/// If the questions are ever updated in cmd_spec.rs, update this array too.
///
/// # API key contract
///
/// `SILICONFLOW_API_KEY` must be set in the environment when the backend
/// process starts. The handler inherits this env var and passes it through to
/// the spawned `turingos spec` child process via process inheritance (the child
/// inherits the parent's environment; we do not explicitly `.env()` it here to
/// avoid logging the value). The key is NEVER written to disk.
///
/// # Session workspace layout
///
/// Each spec session creates a per-session subdirectory under the base workspace:
///   <workspace>/sessions/<session_id>/
///   <workspace>/sessions/<session_id>/answers.json   ← POST body write
///   <workspace>/sessions/<session_id>/spec.md        ← written by CLI
///   <workspace>/sessions/<session_id>/spec_transcript.jsonl (CLI output)
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
use super::ws::{AppState, WsBroadcastMsg};

// ---------------------------------------------------------------------------
// Canonical 8 questions (Zh; sourced from cmd_spec.rs canonical_questions(Lang::Zh))
/// TRACE_MATRIX FC1-N5: duplication from cmd_spec.rs; web-only read path.
// ---------------------------------------------------------------------------

#[cfg(feature = "web")]
pub(crate) const SPEC_QUESTIONS_ZH: [&str; 8] = [
    // Q1 — The Job (JTBD opener)
    "先不用想程序怎么做。能跟我说说你最近遇到了什么事，让你觉得『要是有个小工具就好了』？\
比如『我妈每周要算一次社区团购账，Excel 太麻烦』。你的故事是什么？",
    // Q2 — The Anchor
    "有没有哪个网站 / App / 小工具，跟你想要的『有点像』？不用一模一样，一两个相似的地方就行。\
（如果想不出来：那纸笔 / Excel / 微信群里现在是怎么做的？）",
    // Q3 — Data model in plain words
    "想象关掉电脑明天再打开，这个工具应该还『记得』哪些东西？比如团购账本会记得：\
每个人的名字、买了什么、付了多少、还欠多少。你的工具要记得什么？",
    // Q4 — First-click walkthrough
    "假设我是你的用户，第一次打开这个工具——我看到什么？然后我点什么？然后呢？\
一步一步告诉我，直到我完成一件事。",
    // Q5 — Weird-user test
    "如果有个奇怪的用户，故意乱点乱填——比如把『金额』填成『哈哈哈』，\
或者同一个名字录入 50 遍——你希望工具怎么办？报错？忽略？还是有别的反应？",
    // Q6 — Disappointment boundary
    "如果这个工具突然多了一个功能，你反而会觉得『搞这个干嘛，反而把简单的事弄复杂了』——\
是什么功能？说两三个。",
    // Q7 — Success test
    "用了一个月之后，你怎么判断『这个工具是有用的』？不是『感觉不错』那种——\
是具体能数出来或看得见的事。比如：『我妈现在不用每周日花两小时算账了。』",
    // Q8 — Playback / mirror
    "（最后一题）下面我会把前面听到的复述一遍，请你看看哪里我听错了或听漏了——\
别客气，挑错就是帮我。如果你想直接补充什么，请在这里写出来。",
];

// ---------------------------------------------------------------------------
// Request / Response / Error types
// ---------------------------------------------------------------------------

/// TRACE_MATRIX FC1-N5 + FC2-N16: GET /api/spec/questions response.
///
/// Returns the 8 canonical interview questions so the frontend can render
/// them before the user has typed anything. The array order matches the
/// interview flow documented in cmd_spec.rs FULL_HELP.
#[cfg(feature = "web")]
#[derive(Debug, Serialize)]
pub(crate) struct SpecQuestionsResponse {
    pub(crate) questions: Vec<String>,
}

/// TRACE_MATRIX FC1-N5 + FC2-N16: POST /api/spec/submit request body.
///
/// `answers`: exactly 8 answers, one per question (order must match the
/// canonical question array). Each answer: non-empty, max 4096 chars.
///
/// `session_id`: optional client-supplied session identifier. If absent,
/// the server generates one as `<unix_secs>_<hex8>`. Session IDs are used
/// as subdirectory names under `<workspace>/sessions/` so they are
/// validated as safe filesystem identifiers.
#[cfg(feature = "web")]
#[derive(Debug, Deserialize)]
pub(crate) struct SpecSubmitRequest {
    pub(crate) answers: Vec<String>,
    pub(crate) session_id: Option<String>,
}

/// TRACE_MATRIX FC1-N5 + FC2-N16: POST /api/spec/submit success response.
///
/// `spec_md`: full content of `<session-dir>/spec.md` written by the CLI.
/// `capsule_cid`: hex CID parsed from `CAS capsule CID    -> <cid>` in stdout.
/// `transcript_jsonl`: optional content of `spec_transcript.jsonl` (may be
///   None if the CLI didn't write it).
#[cfg(feature = "web")]
#[derive(Debug, Serialize)]
pub(crate) struct SpecSubmitResponse {
    pub(crate) session_id: String,
    pub(crate) spec_md: String,
    pub(crate) capsule_cid: Option<String>,
    pub(crate) transcript_jsonl: Option<String>,
}

/// TRACE_MATRIX FC1-N5: error response for spec endpoints.
///
/// `kind` values:
/// - `"invalid_input"`:    field validation failed (400)
/// - `"shellout_failed"`:  CLI exited non-zero (500)
/// - `"spec_md_missing"`:  CLI succeeded but spec.md not found (500)
#[cfg(feature = "web")]
#[derive(Debug, Serialize)]
pub(crate) struct SpecError {
    pub(crate) reason: String,
    pub(crate) kind: &'static str,
}

// ---------------------------------------------------------------------------
// GET /api/spec/questions handler
// ---------------------------------------------------------------------------

/// TRACE_MATRIX FC1-N5 + FC2-N16: return the 8 canonical spec questions.
///
/// Read-only; no shell-out; no state mutation.
#[cfg(feature = "web")]
pub(crate) async fn spec_questions_handler() -> Json<SpecQuestionsResponse> {
    Json(SpecQuestionsResponse {
        questions: SPEC_QUESTIONS_ZH.iter().map(|s| s.to_string()).collect(),
    })
}

// ---------------------------------------------------------------------------
// POST /api/spec/submit handler
// ---------------------------------------------------------------------------

/// TRACE_MATRIX FC1-N5 + FC2-N16: POST /api/spec/submit handler.
///
/// Validates 8 answers, creates per-session workspace, shells out to
/// `turingos spec`, reads spec.md on success, broadcasts SpecComplete.
#[cfg(feature = "web")]
pub(crate) async fn spec_submit_handler(
    State(state): State<AppState>,
    Json(req): Json<SpecSubmitRequest>,
) -> Result<Json<SpecSubmitResponse>, (StatusCode, Json<SpecError>)> {
    // Step 1: validate answers at trust boundary (FC1-N5 shielding).
    validate_answers(&req.answers).map_err(|e| (StatusCode::BAD_REQUEST, Json(e)))?;

    // Step 2: validate / generate session_id.
    let session_id = match req.session_id.as_deref() {
        Some(sid) if !sid.is_empty() => {
            if !is_safe_session_id(sid) {
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(SpecError {
                        reason: format!(
                            "session_id {:?} is invalid; must match ^[a-zA-Z0-9_-]{{1,128}}$",
                            sid
                        ),
                        kind: "invalid_input",
                    }),
                ));
            }
            sid.to_string()
        }
        _ => generate_session_id(),
    };

    // Step 3: resolve workspace dir.
    let workspace = resolve_workspace();

    // Step 4: create per-session subdir.
    let session_dir = PathBuf::from(&workspace).join("sessions").join(&session_id);
    {
        let dir = session_dir.clone();
        tokio::task::spawn_blocking(move || std::fs::create_dir_all(&dir))
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(SpecError {
                        reason: format!("spawn_blocking error: {e}"),
                        kind: "shellout_failed",
                    }),
                )
            })?
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(SpecError {
                        reason: format!("failed to create session dir {:?}: {e}", session_dir),
                        kind: "shellout_failed",
                    }),
                )
            })?;
    }

    // Step 5: write answers.json (JSON array of 8 strings).
    let answers_path = session_dir.join("answers.json");
    let answers_json = serde_json::to_string(&req.answers).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(SpecError {
                reason: format!("failed to serialize answers: {e}"),
                kind: "shellout_failed",
            }),
        )
    })?;
    {
        let path = answers_path.clone();
        let json = answers_json.clone();
        tokio::task::spawn_blocking(move || std::fs::write(&path, &json))
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(SpecError {
                        reason: format!("spawn_blocking error: {e}"),
                        kind: "shellout_failed",
                    }),
                )
            })?
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(SpecError {
                        reason: format!("failed to write answers.json: {e}"),
                        kind: "shellout_failed",
                    }),
                )
            })?;
    }

    // Step 6: resolve binary and shell out — exec-style, no sh -c.
    let bin = resolve_turingos_bin();
    let session_dir_str = session_dir.to_string_lossy().into_owned();
    let answers_path_str = answers_path.to_string_lossy().into_owned();

    log::info!(
        "spec_submit_handler: bin={:?} session_id={:?} session_dir={:?}",
        bin,
        session_id,
        session_dir_str,
    );

    let mut cmd = tokio::process::Command::new(&bin);
    cmd.arg("spec")
        .arg("--workspace")
        .arg(&session_dir_str)
        .arg("--answers-file")
        .arg(&answers_path_str)
        .arg("--lang")
        .arg("zh");
    // W7: inject SILICONFLOW_API_KEY from AppState if set. Value lives in
    // memory only; we do not log it. If unset, the child inherits the parent
    // env unchanged (which may or may not carry the key from the shell).
    if let Ok(guard) = state.api_key.lock() {
        if let Some(key) = guard.as_ref() {
            cmd.env("SILICONFLOW_API_KEY", key);
        }
    }
    let output = cmd.output().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(SpecError {
                reason: format!("failed to spawn {:?}: {e}", bin),
                kind: "shellout_failed",
            }),
        )
    })?;

    // Step 7: check exit code.
    if !output.status.success() {
        let stdout_str = String::from_utf8_lossy(&output.stdout);
        let stderr_str = String::from_utf8_lossy(&output.stderr);
        let combined = format!("stdout: {} | stderr: {}", stdout_str, stderr_str);
        let truncated = if combined.len() > 512 {
            format!("{}…", &combined[..512])
        } else {
            combined
        };
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(SpecError {
                reason: truncated,
                kind: "shellout_failed",
            }),
        ));
    }

    // Step 8: read spec.md written by the CLI.
    let spec_md_path = session_dir.join("spec.md");
    let spec_md = {
        let path = spec_md_path.clone();
        tokio::task::spawn_blocking(move || std::fs::read_to_string(&path))
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(SpecError {
                        reason: format!("spawn_blocking error: {e}"),
                        kind: "spec_md_missing",
                    }),
                )
            })?
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(SpecError {
                        reason: format!(
                            "CLI exited 0 but spec.md not found at {:?}: {e}",
                            spec_md_path
                        ),
                        kind: "spec_md_missing",
                    }),
                )
            })?
    };

    // Step 9: parse CAS capsule CID from stdout.
    let stdout_str = String::from_utf8_lossy(&output.stdout).into_owned();
    let capsule_cid = parse_capsule_cid_from_stdout(&stdout_str);

    // Step 10: read transcript (optional; do not fail if absent).
    let transcript_path = session_dir.join("spec_transcript.jsonl");
    let transcript_jsonl = {
        let path = transcript_path.clone();
        tokio::task::spawn_blocking(move || std::fs::read_to_string(&path))
            .await
            .ok()
            .and_then(|r| r.ok())
    };

    // Step 11: broadcast SpecComplete to all connected WS clients.
    let _ = state.broadcast_tx.send(WsBroadcastMsg::SpecComplete {
        session_id: session_id.clone(),
        capsule_cid: capsule_cid.clone(),
    });

    // Step 12: respond 200.
    Ok(Json(SpecSubmitResponse {
        session_id,
        spec_md,
        capsule_cid,
        transcript_jsonl,
    }))
}

// ---------------------------------------------------------------------------
// Validation helpers
// ---------------------------------------------------------------------------

/// Validate the 8-answer array at the trust boundary.
///
/// Rules:
/// - Exactly 8 answers required.
/// - Each answer: non-empty, max 4096 chars (generous; users may give long answers).
#[cfg(feature = "web")]
fn validate_answers(answers: &[String]) -> Result<(), SpecError> {
    if answers.len() != 8 {
        return Err(SpecError {
            reason: format!("expected exactly 8 answers, got {}", answers.len()),
            kind: "invalid_input",
        });
    }
    for (i, answer) in answers.iter().enumerate() {
        if answer.is_empty() {
            return Err(SpecError {
                reason: format!("answer {} is empty; all 8 answers are required", i + 1),
                kind: "invalid_input",
            });
        }
        if answer.len() > 4096 {
            return Err(SpecError {
                reason: format!(
                    "answer {} is too long ({} chars); max is 4096",
                    i + 1,
                    answer.len()
                ),
                kind: "invalid_input",
            });
        }
    }
    Ok(())
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
/// Shape: `{ ok: bool, class: "relevant"|"off_topic"|"abusive"|"gibberish" }`.
#[cfg(feature = "web")]
fn parse_triage_class_from_output(stdout: &str) -> Result<String, String> {
    let v: serde_json::Value =
        serde_json::from_str(stdout.trim()).map_err(|e| format!("parse triage JSON: {e}"))?;
    let class = v
        .get("class")
        .and_then(|x| x.as_str())
        .unwrap_or("gibberish")
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

#[cfg(feature = "web")]
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
    // CANONICAL_SLOTS from grill_envelope (8 slots)
    let all_slots = [
        "job_story",
        "anchor",
        "data_model",
        "first_click",
        "weird_user",
        "disappointment_boundary",
        "success_test",
        "playback",
    ];

    let covered: Vec<String> = envelope
        .get("covered_slots")
        .and_then(|x| x.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default();

    let open: Vec<String> = all_slots
        .iter()
        .filter(|s| !covered.iter().any(|c| c.as_str() == **s))
        .map(|s| s.to_string())
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
        parent_turn_cid_snap,
        non_relevant_count,
        last_prev_covered_snap,
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
        }));
    }

    // ── Step 9: triage user_answer if present (subsequent turns) ─────────────
    let prev_question: String = last_3_turns_snap
        .back()
        .map(|(q, _)| q.clone())
        .unwrap_or_default();

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
        let bin2 = bin.clone();
        let ws2 = workspace.clone();
        let user_answer_owned = user_answer.to_string();
        let prev_q_owned = prev_question.clone();
        let lang2 = lang.clone();
        let sid2 = req.session_id.clone();
        let caps_dir2 = capsules_dir.clone();

        let triage_stdout = tokio::task::spawn_blocking(move || {
            std::process::Command::new(&bin2)
                .arg("llm")
                .arg("triage")
                .arg("--workspace")
                .arg(&ws2)
                .arg("--user-answer")
                .arg(&user_answer_owned)
                .arg("--question")
                .arg(&prev_q_owned)
                .arg("--lang")
                .arg(&lang2)
                .arg("--capsule-dir")
                .arg(&caps_dir2)
                .arg("--turn-id")
                .arg(&triage_turn_id)
                .output()
                .map(|o| String::from_utf8_lossy(&o.stdout).into_owned())
                .unwrap_or_default()
        })
        .await
        .unwrap_or_default();

        let triage_class = parse_triage_class_from_output(&triage_stdout)
            .unwrap_or_else(|_| "gibberish".to_string());

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
                // Shell out to write session capsule with termination_reason
                let ws3 = workspace.clone();
                let sid3 = req.session_id.clone();
                let bin3 = bin.clone();
                tokio::task::spawn_blocking(move || {
                    let _ = std::process::Command::new(&bin3)
                        .arg("spec")
                        .arg("--workspace")
                        .arg(&ws3)
                        .arg("--session")
                        .arg(&sid3)
                        .arg("--mode")
                        .arg("driven")
                        .arg("--synthesize-only")
                        .arg("--termination-reason")
                        .arg("user_input_unparseable")
                        .output();
                })
                .await
                .ok();

                let _ = state
                    .broadcast_tx
                    .send(super::ws::WsBroadcastMsg::SpecGrillComplete {
                        session_id: req.session_id.clone(),
                        spec_capsule_cid: String::new(),
                    });

                return Ok(Json(SpecTurnResponse {
                    turn_index: turn_count + 1,
                    question_text: String::new(),
                    covered_slots: vec![],
                    open_slots: vec![],
                    confidence: 0.0,
                    done: false,
                    playback: None,
                    terminated: true,
                    spec_capsule_cid: None,
                    turn_capsule_cid: None,
                }));
            }

            // Not yet at abort threshold — just bounce back a "please try again" response
            return Ok(Json(SpecTurnResponse {
                turn_index: turn_count + 1,
                question_text: prev_question.clone(),
                covered_slots: vec![],
                open_slots: vec![],
                confidence: 0.0,
                done: false,
                playback: None,
                terminated: false,
                spec_capsule_cid: None,
                turn_capsule_cid: None,
            }));
        }

        // Relevant — record answer; update triage counter
        {
            let mut sessions = state.sessions.lock().unwrap_or_else(|e| e.into_inner());
            if let Some(sess) = sessions.get_mut(&req.session_id) {
                sess.triage_calls_relevant += 1;
                sess.non_relevant_count = 0; // reset consecutive counter on relevant answer
                                             // Push accepted (prev_question, answer) to rolling last-3 window
                if sess.last_3_turns.len() == 3 {
                    sess.last_3_turns.pop_front();
                }
                sess.last_3_turns
                    .push_back((prev_question.clone(), user_answer.to_string()));
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

    // Build the prompt JSON and write to disk
    let (coverage_summary, last_3_for_prompt) = {
        let sessions = state.sessions.lock().unwrap_or_else(|e| e.into_inner());
        let sess = sessions.get(&req.session_id).unwrap();
        let summary = build_coverage_summary(&sess.coverage_state, turn_count);
        let last3 = sess.last_3_turns.clone();
        (summary, last3)
    };

    let meta_prompt_path = PathBuf::from(&workspace).join("assets/prompts/grill_meta_v1.md");
    let prompt_json =
        build_web_turn_prompt_json(&coverage_summary, &last_3_for_prompt, new_turn_index, None);
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
    let bin2 = bin.clone();
    let ws2 = workspace.clone();
    let pf2 = prompt_file_path.clone();
    let cd2 = capsules_dir.clone();
    let tid2 = turn_id.clone();
    let lang2 = lang.clone();
    let mp2 = meta_prompt_path.clone();

    let complete_stdout = tokio::task::spawn_blocking(move || {
        std::process::Command::new(&bin2)
            .arg("llm")
            .arg("complete")
            .arg("--workspace")
            .arg(&ws2)
            .arg("--role")
            .arg("meta")
            .arg("--prompt-file")
            .arg(&pf2)
            .arg("--strict-json")
            .arg("--capsule-dir")
            .arg(&cd2)
            .arg("--turn-id")
            .arg(&tid2)
            .arg("--lang")
            .arg(&lang2)
            .arg("--meta-prompt")
            .arg(&mp2)
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).into_owned())
            .unwrap_or_default()
    })
    .await
    .unwrap_or_default();

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
            if let Some(ref cid) = turn_capsule_cid {
                sess.turn_cids.push(cid.clone());
                sess.parent_turn_cid = Some(cid.clone());
            }
            // Update coverage state
            for slot in &covered_slots {
                sess.coverage_state
                    .insert(slot.clone(), SlotState::Satisfied);
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

    if done {
        // Shell out for synthesis
        let ws3 = workspace.clone();
        let sid3 = req.session_id.clone();
        let bin3 = bin.clone();

        let synth_stdout = tokio::task::spawn_blocking(move || {
            std::process::Command::new(&bin3)
                .arg("spec")
                .arg("--workspace")
                .arg(&ws3)
                .arg("--session")
                .arg(&sid3)
                .arg("--mode")
                .arg("driven")
                .arg("--synthesize-only")
                .output()
                .map(|o| String::from_utf8_lossy(&o.stdout).into_owned())
                .unwrap_or_default()
        })
        .await
        .unwrap_or_default();

        // Try to parse spec_capsule_cid from synthesis output
        spec_capsule_cid =
            parse_capsule_cid_from_stdout(&synth_stdout).or_else(|| Some(placeholder_cid()));

        is_terminated = true;

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
                spec_capsule_cid: spec_capsule_cid.clone().unwrap_or_default(),
            });
    }

    // ── Step 14: hard turn ceiling post-check ────────────────────────────────
    if new_turn_index >= 15 && !done {
        is_terminated = true;
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
    }))
}

// ---------------------------------------------------------------------------
// W7 helpers
// ---------------------------------------------------------------------------

/// Build a coverage summary string (injected into the prompt JSON).
#[cfg(feature = "web")]
fn build_coverage_summary(
    coverage_state: &std::collections::HashMap<String, super::ws::SlotState>,
    turn_count: u32,
) -> String {
    let slots = [
        "job_story",
        "anchor",
        "data_model",
        "first_click",
        "weird_user",
        "disappointment_boundary",
        "success_test",
        "playback",
    ];
    let mut parts = Vec::new();
    for slot in &slots {
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
/// Simplified version (no meta-prompt file read; the shell-out to
/// `turingos llm complete --meta-prompt` handles that server-side).
#[cfg(feature = "web")]
fn build_web_turn_prompt_json(
    coverage_summary: &str,
    last_3_turns: &std::collections::VecDeque<(String, String)>,
    turn_index: u32,
    extra_system: Option<&str>,
) -> String {
    let mut messages: Vec<serde_json::Value> = Vec::new();

    // Coverage state summary
    messages.push(serde_json::json!({
        "role": "system",
        "content": coverage_summary,
    }));

    // Optional extra system message
    if let Some(extra) = extra_system {
        messages.push(serde_json::json!({
            "role": "system",
            "content": extra,
        }));
    }

    // Last 3 accepted turns as alternating assistant/user pairs
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

    // Final user instruction
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
    fn spec_questions_has_8_entries() {
        assert_eq!(SPEC_QUESTIONS_ZH.len(), 8);
    }

    #[test]
    fn validate_answers_rejects_empty_answer() {
        let mut answers: Vec<String> = (0..8).map(|i| format!("answer {i}")).collect();
        answers[3] = "".to_string();
        let err = validate_answers(&answers).unwrap_err();
        assert_eq!(err.kind, "invalid_input");
        assert!(err.reason.contains("empty"));
    }

    #[test]
    fn validate_answers_rejects_oversized_answer() {
        let mut answers: Vec<String> = (0..8).map(|i| format!("answer {i}")).collect();
        answers[0] = "x".repeat(4097);
        let err = validate_answers(&answers).unwrap_err();
        assert_eq!(err.kind, "invalid_input");
        assert!(err.reason.contains("too long"));
    }

    #[test]
    fn validate_answers_rejects_wrong_count() {
        let answers: Vec<String> = (0..5).map(|i| format!("answer {i}")).collect();
        let err = validate_answers(&answers).unwrap_err();
        assert_eq!(err.kind, "invalid_input");
        assert!(err.reason.contains("5"));
    }

    #[test]
    fn validate_answers_accepts_valid_8() {
        let answers: Vec<String> = (0..8).map(|i| format!("valid answer {i}")).collect();
        assert!(validate_answers(&answers).is_ok());
    }

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
}
