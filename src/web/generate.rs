/// TRACE_MATRIX FC1-N5 + FC2-N16: Phase 7 W5 — generate endpoint
///
/// Route exposed:
///   POST /api/generate → accept session_id + options, shell out to
///                        `turingos generate --workspace <session-dir>
///                                           [--from-capsule]
///                                           [--max-files <N>]`
///                        on exit 0: walk artifacts/, return list, broadcast
///                        GenerateComplete to WS channel.
///
/// FC-trace: FC1-N5 (read-view shielding at trust boundary) +
///           FC2-N16 (write action via existing Phase 6.3 CLI shellout; no new
///           Class-4 admission; artifacts are a derived Class-1 write from the
///           spec capsule via Blackbox LLM).
/// Risk class: Class 2-3.
///
/// # API key contract
///
/// `SILICONFLOW_API_KEY` must be set in the environment when the backend
/// process starts. The handler inherits this env var and passes it through to
/// the spawned `turingos generate` child process via process inheritance.
/// The key is NEVER written to disk or logged.
///
/// # Session workspace layout (read from, written to by CLI)
///
///   <workspace>/sessions/<session_id>/spec.md           ← required input
///   <workspace>/sessions/<session_id>/artifacts/        ← written by CLI
///   <workspace>/sessions/<session_id>/artifacts/index.html (typical UI output)
///
/// # Binary override (for tests)
///
/// Setting `TURINGOS_BACKEND_OVERRIDE` replaces the default binary.
/// Same resolution order as write.rs.
#[cfg(feature = "web")]
use axum::{extract::State, http::StatusCode, Json};
#[cfg(feature = "web")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "web")]
use std::path::PathBuf;

#[cfg(feature = "web")]
use super::spec::SpecError;
#[cfg(feature = "web")]
use super::verify::verify_artifact_html;
#[cfg(feature = "web")]
use super::ws::{AppState, WsBroadcastMsg};

// ---------------------------------------------------------------------------
// W8 — auto-retry constants
// ---------------------------------------------------------------------------

/// TRACE_MATRIX FC1-N5 + FC1-N10: maximum number of generate attempts.
///
/// Each attempt = one full shellout to `turingos generate` + one heuristic
/// verification pass. Phase 7 W8 Real-LLM E2E observed Qwen3-Coder produce
/// a broken artifact on attempt 1 about 50% of the time; with N=3 retries
/// the probability of three consecutive failures drops to ~12% which is
/// acceptable for a non-developer end user.
#[cfg(feature = "web")]
pub(crate) const MAX_GENERATE_ATTEMPTS: u8 = 3;

// ---------------------------------------------------------------------------
// Request / Response types
// ---------------------------------------------------------------------------

/// TRACE_MATRIX FC1-N5 + FC2-N16: POST /api/generate request body.
///
/// `session_id`: identifies the session directory under
///   `<workspace>/sessions/<session_id>/`. Must be a safe identifier.
/// `from_capsule`: if true, pass `--from-capsule` to the CLI (reads spec from
///   CAS rather than spec.md on disk).
/// `max_files`: optional cap passed as `--max-files <N>` to the CLI.
#[cfg(feature = "web")]
#[derive(Debug, Deserialize)]
pub(crate) struct GenerateRequest {
    pub(crate) session_id: String,
    #[serde(default)]
    pub(crate) from_capsule: bool,
    #[serde(default)]
    pub(crate) max_files: Option<u32>,
}

/// TRACE_MATRIX FC1-N5 + FC2-N16: POST /api/generate success response.
///
/// `artifacts`: list of artifact files written under
///   `<session-dir>/artifacts/`, capped at 32 entries.
/// `transcript_excerpt`: first 2048 chars of stdout from the CLI (optional).
/// `total_attempts` (W8): how many attempts were needed before the artifact
///   passed heuristic verification. `1` means single-shot success; `>=2`
///   means at least one retry happened. The frontend can show
///   "✓ (经过 N 次尝试)" if `total_attempts > 1`.
#[cfg(feature = "web")]
#[derive(Debug, Serialize)]
pub(crate) struct GenerateResponse {
    pub(crate) session_id: String,
    pub(crate) artifacts: Vec<ArtifactEntry>,
    pub(crate) transcript_excerpt: Option<String>,
    pub(crate) total_attempts: u8,
}

/// TRACE_MATRIX FC1-N5: one artifact file entry in the generate response.
#[cfg(feature = "web")]
#[derive(Debug, Serialize)]
pub(crate) struct ArtifactEntry {
    /// Path relative to `<session-dir>/artifacts/` (e.g. "index.html").
    pub(crate) path: String,
    /// File size in bytes.
    pub(crate) size_bytes: u64,
    /// MIME type sniffed by extension.
    pub(crate) content_type: &'static str,
}

// ---------------------------------------------------------------------------
// POST /api/generate handler
// ---------------------------------------------------------------------------

/// TRACE_MATRIX FC1-N5 + FC2-N16: POST /api/generate handler.
///
/// Validates session + spec.md existence, shells out to `turingos generate`,
/// walks artifacts dir on success, broadcasts GenerateComplete.
#[cfg(feature = "web")]
pub(crate) async fn generate_handler(
    State(state): State<AppState>,
    Json(req): Json<GenerateRequest>,
) -> Result<Json<GenerateResponse>, (StatusCode, Json<SpecError>)> {
    // Step 1: validate session_id format.
    if !is_safe_session_id(&req.session_id) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(SpecError {
                reason: format!(
                    "session_id {:?} is invalid; must match ^[a-zA-Z0-9_-]{{1,128}}$",
                    req.session_id
                ),
                kind: "invalid_input",
            }),
        ));
    }

    // Step 2: resolve workspace and session dir.
    let workspace = resolve_workspace();
    let session_dir = PathBuf::from(&workspace)
        .join("sessions")
        .join(&req.session_id);

    // Step 3: validate session dir exists.
    if !session_dir.exists() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(SpecError {
                reason: format!(
                    "session {:?} not found at {:?}; run spec/submit first",
                    req.session_id, session_dir
                ),
                kind: "invalid_input",
            }),
        ));
    }

    // Step 4: validate spec.md exists.
    let spec_md_path = session_dir.join("spec.md");
    if !spec_md_path.exists() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(SpecError {
                reason: format!(
                    "spec.md not found at {:?}; run spec/submit first",
                    spec_md_path
                ),
                kind: "spec_md_missing",
            }),
        ));
    }

    // Step 5: resolve binary; we will shell out inside the retry loop.
    let bin = resolve_turingos_bin();
    let session_dir_str = session_dir.to_string_lossy().into_owned();
    let artifacts_dir = session_dir.join("artifacts");

    log::info!(
        "generate_handler: bin={:?} session_id={:?} session_dir={:?} from_capsule={} max_files={:?} max_attempts={}",
        bin,
        req.session_id,
        session_dir_str,
        req.from_capsule,
        req.max_files,
        MAX_GENERATE_ATTEMPTS,
    );

    // Step 6: W8 retry loop — attempt up to MAX_GENERATE_ATTEMPTS times.
    let mut last_failure_reason: String = String::new();
    let mut last_failure_kind: &str = "shellout_failed";
    let mut last_artifact_path_for_inspection: Option<String> = None;
    let mut last_stdout: String = String::new();

    for attempt in 1u8..=MAX_GENERATE_ATTEMPTS {
        // Broadcast attempt start (UX progress chip).
        let _ = state
            .broadcast_tx
            .send(WsBroadcastMsg::GenerateAttemptStarted {
                session_id: req.session_id.clone(),
                attempt,
                max_attempts: MAX_GENERATE_ATTEMPTS,
            });

        // Clean any artifacts left from a prior failed attempt so the file
        // walker doesn't pick up stale broken output. Skip on attempt 1 to
        // preserve pre-existing fixtures (the CLI itself overwrites on
        // run; only retries need clearing).
        if attempt > 1 && artifacts_dir.exists() {
            let _ = clear_dir_contents(&artifacts_dir);
        }

        // Build the command fresh for each attempt (Command isn't Clone).
        let mut cmd = tokio::process::Command::new(&bin);
        cmd.arg("generate").arg("--workspace").arg(&session_dir_str);
        if req.from_capsule {
            cmd.arg("--from-capsule");
        }
        if let Some(max_files) = req.max_files {
            cmd.arg("--max-files").arg(max_files.to_string());
        }
        // W7: inject SILICONFLOW_API_KEY from AppState if set. Value lives
        // in memory only; we do not log it. If unset, child inherits parent.
        if let Ok(guard) = state.api_key.lock() {
            if let Some(key) = guard.as_ref() {
                cmd.env("SILICONFLOW_API_KEY", key);
            }
        }

        let output = match cmd.output().await {
            Ok(o) => o,
            Err(e) => {
                let reason = format!("failed to spawn {:?}: {e}", bin);
                let _ = state
                    .broadcast_tx
                    .send(WsBroadcastMsg::GenerateAttemptFailed {
                        session_id: req.session_id.clone(),
                        attempt,
                        max_attempts: MAX_GENERATE_ATTEMPTS,
                        reason: reason.clone(),
                    });
                last_failure_reason = reason;
                last_failure_kind = "shellout_failed";
                if attempt < MAX_GENERATE_ATTEMPTS {
                    continue;
                }
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(SpecError {
                        reason: last_failure_reason,
                        kind: last_failure_kind,
                    }),
                ));
            }
        };

        // Capture stdout for the eventual response (winning attempt wins).
        let stdout_str = String::from_utf8_lossy(&output.stdout).into_owned();
        last_stdout = stdout_str.clone();

        // Non-zero exit → failure, broadcast and (maybe) retry.
        if !output.status.success() {
            let stderr_str = String::from_utf8_lossy(&output.stderr).into_owned();
            let exit_code = output.status.code().unwrap_or(-1);
            let combined = format!("stdout: {} | stderr: {}", stdout_str, stderr_str);
            let truncated = if combined.len() > 512 {
                format!("{}…", &combined[..512])
            } else {
                combined
            };
            let reason = format!("shellout_exit_{exit_code}: {truncated}");
            let _ = state
                .broadcast_tx
                .send(WsBroadcastMsg::GenerateAttemptFailed {
                    session_id: req.session_id.clone(),
                    attempt,
                    max_attempts: MAX_GENERATE_ATTEMPTS,
                    reason: reason.clone(),
                });
            last_failure_reason = reason;
            last_failure_kind = "shellout_failed";
            if attempt < MAX_GENERATE_ATTEMPTS {
                continue;
            }
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(SpecError {
                    reason: last_failure_reason,
                    kind: last_failure_kind,
                }),
            ));
        }

        // Exit zero → walk artifacts and run heuristic verification.
        let artifact_entries = walk_artifacts_dir(&artifacts_dir).await;
        let index_html = artifact_entries
            .iter()
            .find(|e| e.path.eq_ignore_ascii_case("index.html"));

        let verify_outcome = match index_html {
            Some(entry) => {
                let abs_path = artifacts_dir.join(&entry.path);
                last_artifact_path_for_inspection = Some(entry.path.clone());
                match verify_artifact_html(&abs_path) {
                    Ok(outcome) => Some(outcome),
                    Err(e) => {
                        log::warn!("verify_artifact_html failed: {e}");
                        None
                    }
                }
            }
            None => {
                // No index.html — there is no heuristic surface (could be a
                // Python-only artifact, or zero artifacts in CLI smoke
                // contexts). Preserve the pre-W8 success contract: accept.
                None
            }
        };

        // If we have a verify outcome, gate on it. If no index.html but
        // artifacts exist, fall through to success.
        if let Some(outcome) = verify_outcome.as_ref() {
            if !outcome.passed {
                let reason = outcome.failure_reasons.join("; ");
                let _ = state
                    .broadcast_tx
                    .send(WsBroadcastMsg::GenerateAttemptFailed {
                        session_id: req.session_id.clone(),
                        attempt,
                        max_attempts: MAX_GENERATE_ATTEMPTS,
                        reason: reason.clone(),
                    });
                last_failure_reason = reason;
                last_failure_kind = "generate_quality_failed";
                if attempt < MAX_GENERATE_ATTEMPTS {
                    continue;
                }
                // Final failure — return 500 with the last artifact path so
                // the user can still inspect what came out.
                let final_reason = if let Some(path) = last_artifact_path_for_inspection.as_ref() {
                    format!(
                        "{} | last_artifact={}/artifacts/{}",
                        last_failure_reason, req.session_id, path
                    )
                } else {
                    last_failure_reason.clone()
                };
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(SpecError {
                        reason: final_reason,
                        kind: "generate_quality_failed",
                    }),
                ));
            }
        }

        // Heuristic passed (or not applicable) → success path.
        let transcript_excerpt = if last_stdout.is_empty() {
            None
        } else {
            let excerpt = if last_stdout.len() > 2048 {
                format!("{}…", &last_stdout[..2048])
            } else {
                last_stdout.clone()
            };
            Some(excerpt)
        };

        // Broadcast GenerateComplete (existing contract preserved).
        let artifact_paths: Vec<String> = artifact_entries.iter().map(|e| e.path.clone()).collect();
        let _ = state.broadcast_tx.send(WsBroadcastMsg::GenerateComplete {
            session_id: req.session_id.clone(),
            artifacts: artifact_paths,
        });

        return Ok(Json(GenerateResponse {
            session_id: req.session_id,
            artifacts: artifact_entries,
            transcript_excerpt,
            total_attempts: attempt,
        }));
    }

    // Loop exited without success — unreachable given the explicit returns
    // inside the loop, but cover it defensively for the type-checker.
    Err((
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(SpecError {
            reason: if last_failure_reason.is_empty() {
                "generate_retry_exhausted".to_string()
            } else {
                last_failure_reason
            },
            kind: last_failure_kind,
        }),
    ))
}

/// W8 helper: best-effort recursive removal of all entries inside `dir`.
/// We do NOT remove `dir` itself — the path needs to stay so the CLI can
/// write fresh artifacts. Returns Ok(()) on full success; ignores errors
/// for individual entries.
#[cfg(feature = "web")]
fn clear_dir_contents(dir: &std::path::Path) -> std::io::Result<()> {
    let rd = std::fs::read_dir(dir)?;
    for entry in rd.flatten() {
        let path = entry.path();
        if path.is_dir() {
            let _ = std::fs::remove_dir_all(&path);
        } else {
            let _ = std::fs::remove_file(&path);
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Artifact directory walker
// ---------------------------------------------------------------------------

/// Walk `<artifacts_dir>`, collect up to 32 entries with MIME type.
///
/// Uses synchronous `std::fs::read_dir` inside `tokio::task::spawn_blocking`
/// to avoid blocking the async executor on filesystem I/O. Returns an empty
/// Vec if the directory doesn't exist (generate may write 0 artifacts in edge
/// cases).
#[cfg(feature = "web")]
async fn walk_artifacts_dir(artifacts_dir: &std::path::Path) -> Vec<ArtifactEntry> {
    let dir = artifacts_dir.to_path_buf();
    tokio::task::spawn_blocking(move || walk_artifacts_dir_sync(&dir))
        .await
        .unwrap_or_default()
}

#[cfg(feature = "web")]
fn walk_artifacts_dir_sync(artifacts_dir: &std::path::Path) -> Vec<ArtifactEntry> {
    if !artifacts_dir.exists() {
        return Vec::new();
    }
    let mut out = Vec::new();
    collect_dir_entries(artifacts_dir, artifacts_dir, &mut out, 0);
    // Sort by path for deterministic ordering.
    out.sort_by(|a, b| a.path.cmp(&b.path));
    // Cap at 32 entries.
    out.truncate(32);
    out
}

#[cfg(feature = "web")]
fn collect_dir_entries(
    base: &std::path::Path,
    dir: &std::path::Path,
    out: &mut Vec<ArtifactEntry>,
    depth: usize,
) {
    // Safety: cap recursion depth to avoid unexpected deep trees.
    if depth > 5 {
        return;
    }
    if out.len() >= 32 {
        return;
    }
    let rd = match std::fs::read_dir(dir) {
        Ok(rd) => rd,
        Err(_) => return,
    };
    for entry in rd.flatten() {
        let ft = match entry.file_type() {
            Ok(ft) => ft,
            Err(_) => continue,
        };
        let path = entry.path();
        if ft.is_dir() {
            collect_dir_entries(base, &path, out, depth + 1);
        } else if ft.is_file() {
            // Compute path relative to artifacts_dir.
            let rel = match path.strip_prefix(base) {
                Ok(r) => r.to_string_lossy().into_owned(),
                Err(_) => continue,
            };
            let size_bytes = std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
            let content_type = mime_by_extension(&path);
            out.push(ArtifactEntry {
                path: rel,
                size_bytes,
                content_type,
            });
            if out.len() >= 32 {
                return;
            }
        }
    }
}

/// Sniff MIME type by file extension.
#[cfg(feature = "web")]
fn mime_by_extension(path: &std::path::Path) -> &'static str {
    match path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase()
        .as_str()
    {
        "html" | "htm" => "text/html",
        "js" | "mjs" => "application/javascript",
        "py" => "text/x-python",
        "css" => "text/css",
        "json" => "application/json",
        "txt" | "md" => "text/plain",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "svg" => "image/svg+xml",
        "ts" | "tsx" => "text/typescript",
        _ => "application/octet-stream",
    }
}

// ---------------------------------------------------------------------------
// Validation / resolution helpers
// ---------------------------------------------------------------------------

/// Returns `true` if `s` is a safe session ID: `^[a-zA-Z0-9_-]{1,128}$`.
#[cfg(feature = "web")]
fn is_safe_session_id(s: &str) -> bool {
    if s.is_empty() || s.len() > 128 {
        return false;
    }
    s.chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
}

/// Resolve the `turingos` binary path.
/// Resolution order: TURINGOS_BACKEND_OVERRIDE → sibling → PATH.
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
/// Resolution order: TURINGOS_WEB_WORKSPACE → current_dir.
#[cfg(feature = "web")]
fn resolve_workspace() -> String {
    if let Ok(v) = std::env::var("TURINGOS_WEB_WORKSPACE") {
        if !v.is_empty() {
            return v;
        }
    }
    std::env::current_dir()
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|_| ".".to_string())
}

// ---------------------------------------------------------------------------
// Unit tests (no I/O)
// ---------------------------------------------------------------------------

#[cfg(all(feature = "web", test))]
mod tests {
    use super::*;

    #[test]
    fn mime_html() {
        assert_eq!(
            mime_by_extension(std::path::Path::new("index.html")),
            "text/html"
        );
    }

    #[test]
    fn mime_js() {
        assert_eq!(
            mime_by_extension(std::path::Path::new("app.js")),
            "application/javascript"
        );
    }

    #[test]
    fn mime_py() {
        assert_eq!(
            mime_by_extension(std::path::Path::new("main.py")),
            "text/x-python"
        );
    }

    #[test]
    fn mime_unknown() {
        assert_eq!(
            mime_by_extension(std::path::Path::new("foo.xyz")),
            "application/octet-stream"
        );
    }

    #[test]
    fn is_safe_session_id_valid() {
        assert!(is_safe_session_id("1716000000_3f8a1b2c"));
        assert!(is_safe_session_id("abc-def_123"));
    }

    #[test]
    fn is_safe_session_id_rejects_dot() {
        assert!(!is_safe_session_id("../bad"));
        assert!(!is_safe_session_id("a.b"));
    }
}
