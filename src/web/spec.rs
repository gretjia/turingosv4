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
