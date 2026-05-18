/// TRACE_MATRIX FC1-N10: write-path integration — POST /api/task/open
///
/// W4 write path: browser form → backend POST → `turingos task open` shell-out
/// → WebSocket broadcast → frontend re-renders task list.
///
/// FC-trace: FC1-N10 (write action surfaces via existing CLI; no new admission).
/// Risk class: Class 2-3.
///
/// # Input validation (FC1-N5 shielding)
///
/// All three request fields are validated at the trust boundary before any
/// shell-out is attempted:
///   - `problem_id`: `^[a-zA-Z0-9_-]{1,64}$` (char-by-char; no regex crate)
///   - `agent_id`:   `^[a-zA-Z0-9_-]{1,64}$` (same rule)
///   - `bounty`:     u64 in range (0, 10_000_000) exclusive — both ends rejected
///
/// # Shell-out safety
///
/// The handler uses exec-style `tokio::process::Command::arg()` calls exclusively.
/// Shell interpolation (`sh -c`, `bash -c`, `eval`) is NEVER used. Each flag and
/// value is passed as a separate argument so the OS kernel handles quoting.
///
/// # Workspace discovery
///
/// The handler reads the `TURINGOS_WEB_WORKSPACE` environment variable at
/// request time. If unset, it falls back to the current working directory
/// (`std::env::current_dir()`). The resolved path is passed to the CLI as
/// `--chaintape <PATH>` (the ChainTape directory).
///
/// # Binary override (for tests)
///
/// Setting `TURINGOS_BACKEND_OVERRIDE` replaces the default binary (`turingos`).
/// The default resolves to a sibling `turingos` binary next to the running
/// `turingos_web` process (via `std::env::current_exe()`), falling back to
/// plain `"turingos"` (PATH lookup) if the sibling does not exist.
///
/// # WS broadcast
///
/// On exit code 0, the handler sends a `WsEnvelope::TaskCreated { … }` on the
/// broadcast channel. Each connected `/ws` handler subscribes to this channel
/// and forwards the message to its client socket.
#[cfg(feature = "web")]
use axum::{extract::State, http::StatusCode, Json};
#[cfg(feature = "web")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "web")]
use super::store::TaskEntry;
#[cfg(feature = "web")]
use super::ws::{AppState, WsBroadcastMsg};

// ---------------------------------------------------------------------------
// Request / Response / Error types
// ---------------------------------------------------------------------------

/// TRACE_MATRIX FC1-N10: validated POST body for /api/task/open.
///
/// All three fields are required. Missing fields → serde 422; invalid values
/// → explicit 400 from `validate()`.
#[cfg(feature = "web")]
#[derive(Debug, Deserialize)]
pub(crate) struct TaskOpenRequest {
    pub(crate) problem_id: String,
    pub(crate) bounty: u64,
    pub(crate) agent_id: String,
}

/// TRACE_MATRIX FC1-N10: 200 OK response for /api/task/open.
#[cfg(feature = "web")]
#[derive(Debug, Serialize)]
pub(crate) struct TaskOpenResponse {
    pub(crate) task_id: String,
    pub(crate) status: &'static str,
}

/// TRACE_MATRIX FC1-N10: error response for /api/task/open.
///
/// `kind` values:
/// - `"invalid_input"`: field failed validation (400)
/// - `"shellout_failed"`: CLI exited non-zero (500)
/// - `"task_id_parse_failed"`: CLI succeeded but stdout unparseable (200 w/ warning)
#[cfg(feature = "web")]
#[derive(Debug, Serialize)]
pub(crate) struct TaskOpenError {
    pub(crate) reason: String,
    pub(crate) kind: &'static str,
}

// ---------------------------------------------------------------------------
// Validation helpers
// ---------------------------------------------------------------------------

/// Returns `true` if `s` matches `^[a-zA-Z0-9_-]{1,64}$`.
///
/// Implemented as a char-by-char scan to avoid pulling in the `regex` crate.
#[cfg(feature = "web")]
fn is_valid_identifier(s: &str) -> bool {
    if s.is_empty() || s.len() > 64 {
        return false;
    }
    s.chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
}

#[cfg(feature = "web")]
fn validate(req: &TaskOpenRequest) -> Result<(), TaskOpenError> {
    if !is_valid_identifier(&req.problem_id) {
        return Err(TaskOpenError {
            reason: format!(
                "problem_id {:?} is invalid; must match ^[a-zA-Z0-9_-]{{1,64}}$",
                req.problem_id
            ),
            kind: "invalid_input",
        });
    }
    if req.bounty == 0 || req.bounty >= 10_000_000 {
        return Err(TaskOpenError {
            reason: format!(
                "bounty {} is out of range; must be in (0, 10_000_000) exclusive",
                req.bounty
            ),
            kind: "invalid_input",
        });
    }
    if !is_valid_identifier(&req.agent_id) {
        return Err(TaskOpenError {
            reason: format!(
                "agent_id {:?} is invalid; must match ^[a-zA-Z0-9_-]{{1,64}}$",
                req.agent_id
            ),
            kind: "invalid_input",
        });
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// task_id parser
// ---------------------------------------------------------------------------

/// Scan stdout for a task_id. Tries three patterns in order:
///   1. Line containing `task_id: <id>` (TOML/key-value style)
///   2. Line containing `task: <id>`
///   3. JSON snippet `"task_id":"<id>"` or `"task_id": "<id>"`
///
/// Returns the first matched id. If no match, returns `None` (caller decides
/// whether to synthesize a fallback id).
#[cfg(feature = "web")]
fn parse_task_id_from_stdout(stdout: &str) -> Option<String> {
    for line in stdout.lines() {
        // Pattern 1: `task_id: <id>` (key-value, optional whitespace around colon)
        if let Some(rest) = line.strip_prefix("task_id:") {
            let id = rest.trim().to_string();
            if !id.is_empty() {
                return Some(id);
            }
        }
        // Pattern 2: `task: <id>`
        if let Some(rest) = line.strip_prefix("task:") {
            let id = rest.trim().to_string();
            if !id.is_empty() {
                return Some(id);
            }
        }
    }
    // Pattern 3: JSON `"task_id":"<id>"` anywhere in stdout (single-pass scan)
    let s = stdout;
    if let Some(pos) = s.find("\"task_id\"") {
        let after_key = &s[pos + 9..]; // skip `"task_id"`
                                       // skip whitespace and colon
        let after_colon = after_key.trim_start_matches(|c: char| c == ':' || c == ' ');
        if after_colon.starts_with('"') {
            let inner = &after_colon[1..];
            if let Some(end) = inner.find('"') {
                let id = inner[..end].to_string();
                if !id.is_empty() {
                    return Some(id);
                }
            }
        }
    }
    None
}

// ---------------------------------------------------------------------------
// Binary resolution helpers
// ---------------------------------------------------------------------------

/// Resolve which binary to invoke for `turingos`.
///
/// Resolution order:
///   1. `TURINGOS_BACKEND_OVERRIDE` env var (full path; for tests)
///   2. Sibling `turingos` next to the running `turingos_web` binary
///   3. Bare `"turingos"` (PATH lookup)
#[cfg(feature = "web")]
fn resolve_turingos_bin() -> String {
    // 1. Test override
    if let Ok(v) = std::env::var("TURINGOS_BACKEND_OVERRIDE") {
        if !v.is_empty() {
            return v;
        }
    }
    // 2. Sibling binary
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let sibling = dir.join("turingos");
            if sibling.exists() {
                return sibling.to_string_lossy().into_owned();
            }
        }
    }
    // 3. PATH fallback
    "turingos".to_string()
}

/// Resolve the TuringOS workspace directory.
///
/// Resolution order:
///   1. `TURINGOS_WEB_WORKSPACE` env var (explicit operator config)
///   2. `tmp/phase7_active` (W8.1: harmonized with welcome.rs default;
///      previously fell back to `current_dir()` which caused task-open
///      ChainTape dirs to land in the repo root.)
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
// Handler
// ---------------------------------------------------------------------------

/// TRACE_MATRIX FC1-N10: POST /api/task/open handler.
///
/// Validates input, shells out to `turingos task open`, parses task_id from
/// stdout, broadcasts `TaskCreated` to all connected WS clients, returns 200.
///
/// Error paths:
/// - 400 `invalid_input`: field validation failed
/// - 500 `shellout_failed`: CLI exited non-zero
/// - 200 with `status: "created"` even if task_id parse fails (task was created;
///   the id is synthesized from a hex hash of stdout bytes in that case).
#[cfg(feature = "web")]
pub(crate) async fn task_open_handler(
    State(state): State<AppState>,
    Json(req): Json<TaskOpenRequest>,
) -> Result<Json<TaskOpenResponse>, (StatusCode, Json<TaskOpenError>)> {
    // Step 1: validate at trust boundary (FC1-N5 shielding).
    validate(&req).map_err(|e| (StatusCode::BAD_REQUEST, Json(e)))?;

    // Step 2: resolve workspace and binary.
    let workspace = resolve_workspace();
    let bin = resolve_turingos_bin();
    log::info!(
        "task_open_handler: bin={:?} workspace={:?} problem_id={:?} agent_id={:?} bounty={}",
        bin,
        workspace,
        req.problem_id,
        req.agent_id,
        req.bounty,
    );

    // Step 3: exec-style shell-out — NEVER sh -c interpolation.
    // CLI invocation: `turingos task open --problem <ID> --bounty <N> --agent-id <A> --chaintape <WORKSPACE>`
    // The `turingos task open` handler prepends `run-task` to these args and
    // passes them to `lean_market`. Flags `--problem`, `--bounty`, `--chaintape`
    // are documented in cmd_task_open.rs FULL_HELP. `--agent-id` is passed
    // through to the backend for agent assignment.
    let output = tokio::process::Command::new(&bin)
        .arg("task")
        .arg("open")
        .arg("--problem")
        .arg(&req.problem_id)
        .arg("--bounty")
        .arg(req.bounty.to_string())
        .arg("--agent-id")
        .arg(&req.agent_id)
        .arg("--chaintape")
        .arg(&workspace)
        .output()
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(TaskOpenError {
                    reason: format!("failed to spawn {:?}: {e}", bin),
                    kind: "shellout_failed",
                }),
            )
        })?;

    // Step 4: check exit code.
    if !output.status.success() {
        let stdout_str = String::from_utf8_lossy(&output.stdout);
        let stderr_str = String::from_utf8_lossy(&output.stderr);
        // Truncate to first 512 chars to avoid log flooding.
        let combined = format!("stdout: {} | stderr: {}", stdout_str, stderr_str);
        let truncated = if combined.len() > 512 {
            format!("{}…", &combined[..512])
        } else {
            combined
        };
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(TaskOpenError {
                reason: truncated,
                kind: "shellout_failed",
            }),
        ));
    }

    // Step 5: parse task_id from stdout.
    let stdout_str = String::from_utf8_lossy(&output.stdout).into_owned();
    let (task_id, parse_warning) = match parse_task_id_from_stdout(&stdout_str) {
        Some(id) => (id, false),
        None => {
            // Fallback: hash stdout bytes for a deterministic but unique id.
            let hash = simple_hash(output.stdout.as_slice());
            (format!("t_hash_{hash:016x}"), true)
        }
    };

    if parse_warning {
        log::warn!(
            "task_open_handler: task_id parse failed; using hash fallback. stdout={:?}",
            &stdout_str[..stdout_str.len().min(256)],
        );
    }

    // Step 6: push to in-memory store BEFORE broadcasting so that a WS
    // subscriber racing to re-fetch /api/tasks immediately after receiving
    // the broadcast will already see the entry in the store.
    let created_at_unix = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    state.task_store.push(TaskEntry {
        task_id: task_id.clone(),
        agent_id: req.agent_id.clone(),
        problem_id: req.problem_id.clone(),
        bounty: req.bounty,
        created_at_unix,
    });

    // Step 7: broadcast TaskCreated to all connected WS clients.
    let msg = WsBroadcastMsg::TaskCreated {
        task_id: task_id.clone(),
        agent_id: req.agent_id.clone(),
        problem_id: req.problem_id.clone(),
        bounty: req.bounty,
    };
    // Ignore send error: zero subscribers is not an error (no clients connected).
    let _ = state.broadcast_tx.send(msg);

    // Step 8: respond 200.
    Ok(Json(TaskOpenResponse {
        task_id,
        status: "created",
    }))
}

// ---------------------------------------------------------------------------
// Minimal hash (no external crates)
// ---------------------------------------------------------------------------

/// FNV-1a 64-bit hash for fallback task_id synthesis.
/// This is a stable, deterministic, dependency-free hash.
#[cfg(feature = "web")]
fn simple_hash(data: &[u8]) -> u64 {
    let mut h: u64 = 14_695_981_039_346_656_037;
    for &b in data {
        h ^= b as u64;
        h = h.wrapping_mul(1_099_511_628_211);
    }
    h
}

// ---------------------------------------------------------------------------
// Unit tests (no I/O)
// ---------------------------------------------------------------------------

#[cfg(all(feature = "web", test))]
mod tests {
    use super::*;

    #[test]
    fn is_valid_identifier_accepts_valid() {
        assert!(is_valid_identifier("abc"));
        assert!(is_valid_identifier("problem-01"));
        assert!(is_valid_identifier("agent_x"));
        assert!(is_valid_identifier("A1-b_2"));
        // exactly 64 chars
        assert!(is_valid_identifier(&"a".repeat(64)));
    }

    #[test]
    fn is_valid_identifier_rejects_invalid() {
        assert!(!is_valid_identifier("")); // empty
        assert!(!is_valid_identifier(&"a".repeat(65))); // too long
        assert!(!is_valid_identifier("../../etc/passwd")); // path traversal
        assert!(!is_valid_identifier("has space")); // space
        assert!(!is_valid_identifier("has\nnewline")); // newline
        assert!(!is_valid_identifier("semicolon;")); // shell metachar
    }

    #[test]
    fn validate_rejects_zero_bounty() {
        let req = TaskOpenRequest {
            problem_id: "prob1".into(),
            bounty: 0,
            agent_id: "agent1".into(),
        };
        let err = validate(&req).unwrap_err();
        assert_eq!(err.kind, "invalid_input");
        assert!(err.reason.contains("bounty"));
    }

    #[test]
    fn validate_rejects_max_bounty() {
        let req = TaskOpenRequest {
            problem_id: "prob1".into(),
            bounty: 10_000_000,
            agent_id: "agent1".into(),
        };
        let err = validate(&req).unwrap_err();
        assert_eq!(err.kind, "invalid_input");
    }

    #[test]
    fn validate_accepts_valid() {
        let req = TaskOpenRequest {
            problem_id: "prob-001".into(),
            bounty: 1_000,
            agent_id: "agent_0".into(),
        };
        assert!(validate(&req).is_ok());
    }

    #[test]
    fn parse_task_id_key_value() {
        let stdout = "opening task...\ntask_id: t_abc123\ndone\n";
        assert_eq!(
            parse_task_id_from_stdout(stdout),
            Some("t_abc123".to_string())
        );
    }

    #[test]
    fn parse_task_id_task_prefix() {
        let stdout = "task: t_xyz\n";
        assert_eq!(parse_task_id_from_stdout(stdout), Some("t_xyz".to_string()));
    }

    #[test]
    fn parse_task_id_json() {
        let stdout = r#"{"task_id":"t_json01","status":"ok"}"#;
        assert_eq!(
            parse_task_id_from_stdout(stdout),
            Some("t_json01".to_string())
        );
    }

    #[test]
    fn parse_task_id_none_on_empty() {
        assert_eq!(parse_task_id_from_stdout("no match here\n"), None);
    }
}
