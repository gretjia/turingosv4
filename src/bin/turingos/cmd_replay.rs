//! TRACE_MATRIX FC1 + FC2: turingos `replay` handler.
//!
//! Two modes:
//!   - Default: 7-indicator ChainTape replay verification (shells out to
//!     TASK_RUNNER_BIN). Task-type agnostic.
//!   - `--offline`: CAS-only build-session reconstruction via
//!     `runtime::replay::reconstruct_session`. No LLM, no network.
//!     Used by C9 to audit a session without re-running it.
//!
//! FC-trace: FC1 (replay loop), FC2 (boot reconstruction)
//! Risk class: Class 2

use std::path::PathBuf;
use std::process::ExitCode;

use crate::common::{run_external, TASK_RUNNER_BIN};

/// TRACE_MATRIX FC2-N16: `replay` short-help
pub(crate) const SHORT_HELP: &str =
    "Run 7-indicator ChainTape replay (default) or CAS-only session replay (--offline)";

/// TRACE_MATRIX FC2-N16: `replay` full --help text
pub(crate) const FULL_HELP: &str = r#"turingos replay — replay verification

USAGE:
    turingos replay [OPTIONS]
    turingos replay --offline --workspace <PATH> --session <ID>

DESCRIPTION:
    Default mode: 7-indicator ChainTape replay verification (shell out to
    task-runner backend). Read-only. Works for any task type.

    --offline mode (added 2026-05-21 by C9): CAS-only reconstruction of a
    build session. Reads `turingos-spec-capsule-v1`,
    `turingos-spec-grill-turn-v1`, `turingos-spec-grill-session-v1`,
    `turingos-generation-attempt-v1`, `turingos-artifact-bundle-v1`,
    `turingos-preview-run-v1`, `turingos-generate-rejection-v1` from CAS;
    builds an ordered transcript; verifies all cross-CID references resolve.
    No LLM calls. No network. Exits non-zero if dangling references found.

OPTIONS:
    --offline                Run offline CAS-only replay (no shell-out).
    --workspace <PATH>       Workspace dir (required with --offline).
    --session <ID>           Session ID (required with --offline).
    --chaintape <PATH>       (default mode) Evidence directory to replay.
    -h, --help               Print this help.

EXAMPLES:
    turingos replay --chaintape ./handover/evidence/run001/chaintape
    turingos replay --offline --workspace /data/my_ws --session abc123

SEE ALSO:
    turingos report run --help        Show run summary
    turingos verify chaintape --help  ChainTape structural verification
    turingos spec audit --help        Spec-only offline audit
"#;

/// TRACE_MATRIX FC2-N16: `replay` dispatch entry
pub(crate) fn run(args: &[String]) -> ExitCode {
    if args.iter().any(|a| a == "-h" || a == "--help") {
        print!("{FULL_HELP}");
        return ExitCode::SUCCESS;
    }

    if args.iter().any(|a| a == "--offline") {
        return run_offline(args);
    }

    let mut forwarded: Vec<String> = Vec::with_capacity(args.len() + 1);
    forwarded.push("view-replay".to_owned());
    forwarded.extend_from_slice(args);
    run_external(TASK_RUNNER_BIN, &forwarded)
}

/// TRACE_MATRIX FC1 + FC2: C9 offline replay path.
///
/// Parses `--workspace <PATH>` and `--session <ID>`, calls
/// `runtime::replay::reconstruct_session`, prints a step-by-step transcript,
/// and exits non-zero if `dangling_cid_errors` is non-empty.
fn run_offline(args: &[String]) -> ExitCode {
    let mut workspace: Option<PathBuf> = None;
    let mut session_id: Option<String> = None;

    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--offline" => {}
            "--workspace" => {
                if let Some(v) = iter.next() {
                    workspace = Some(PathBuf::from(v));
                }
            }
            "--session" => {
                if let Some(v) = iter.next() {
                    session_id = Some(v.to_string());
                }
            }
            _ => {}
        }
    }

    let workspace = match workspace {
        Some(w) => w,
        None => {
            eprintln!("error: --offline requires --workspace <PATH>");
            return ExitCode::from(2);
        }
    };
    let session_id = match session_id {
        Some(s) => s,
        None => {
            eprintln!("error: --offline requires --session <ID>");
            return ExitCode::from(2);
        }
    };

    let result = match turingosv4::runtime::replay::reconstruct_session(&workspace, &session_id) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("error: replay failed: {}", e);
            return ExitCode::from(2);
        }
    };

    println!("session_id={}", result.session_id);
    println!("step_count={}", result.steps.len());
    for (idx, step) in result.steps.iter().enumerate() {
        match serde_json::to_string(step) {
            Ok(s) => println!("step[{}]={}", idx, s),
            Err(_) => println!("step[{}]=<unserializable>", idx),
        }
    }

    if !result.dangling_cid_errors.is_empty() {
        eprintln!("FAIL: {} dangling CID reference(s)", result.dangling_cid_errors.len());
        for err in &result.dangling_cid_errors {
            eprintln!("  - {}", err);
        }
        return ExitCode::from(1);
    }

    println!("OK: replay complete, no dangling references");
    ExitCode::SUCCESS
}
