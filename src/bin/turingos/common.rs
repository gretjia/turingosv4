//! TRACE_MATRIX FC2-N16: turingos CLI shared helpers
//!
//! Phase 6.0/6.1 W0 foundation atom. Holds helpers shared across
//! `src/bin/turingos/cmd_*.rs` submodules. All public surface scoped
//! `pub(crate)` — never escapes the `turingos` binary crate.

use std::path::Path;
use std::process::{Command, ExitCode, Stdio};

/// TRACE_MATRIX FC2-N16: shell-escape paths for stdout `cd` hints
///
/// Single-quotes when whitespace or shell-special characters appear.
/// Embedded single-quotes are escaped via `'\''`.
pub(crate) fn shell_quote_path(path: &Path) -> String {
    let raw = path.to_string_lossy();
    let needs_quote = raw.is_empty()
        || raw.chars().any(|c| {
            c.is_whitespace()
                || matches!(
                    c,
                    '"' | '\''
                        | '$'
                        | '`'
                        | '\\'
                        | '!'
                        | '#'
                        | '&'
                        | '('
                        | ')'
                        | '*'
                        | '<'
                        | '>'
                        | '?'
                        | '['
                        | ']'
                        | '{'
                        | '}'
                        | '|'
                        | ';'
                )
        });
    if needs_quote {
        format!("'{}'", raw.replace('\'', r"'\''"))
    } else {
        raw.to_string()
    }
}

/// TRACE_MATRIX FC2-N16: generic task-runner backend binary (Phase 6.1)
///
/// Phase 6.1 implementation note (NOT user-facing): TuringOS today wraps the
/// TB-10-era `lean_market` binary because it currently hosts the generic
/// ChainTape replay / wallet / positions / bankruptcy / task-lifecycle
/// kernel operations. The "lean_" prefix is historical — the wrapped
/// operations are NOT Lean-specific; they apply to any task type that uses
/// the TuringOS task-market pattern (proof / polymarket / multi-agent /
/// future generic compute / future agent collaboration).
///
/// Phase 7+ generalization plan: these operations move into a generic
/// `agent_runner` binary OR in-process library calls. When that lands,
/// this constant is the single point of change — and the wrapped operations
/// themselves stay the same.
///
/// User-facing `turingos --help` / `turingos report wallet --help` etc.
/// must NEVER mention `lean_market`, `Lean`, or `TB-10` — those are
/// implementation details, not the user API.
pub(crate) const TASK_RUNNER_BIN: &str = "lean_market";

/// TRACE_MATRIX FC2-N16: invoke an external project binary (shell-out wrapper)
///
/// Resolves `bin_name` relative to the turingos binary's parent dir. Tries
/// release first, then debug. Inherits stdin/stdout/stderr. Returns child
/// exit code (preserving exit semantics for replay).
///
/// For multi-token wrapped subcommands, pass the subcommand as args[0]
/// from the caller. For the generic task-runner backend, prefer the
/// `TASK_RUNNER_BIN` constant over hard-coding the binary name.
///
/// W0 foundation atom; first consumer lands in Wave 1.
#[allow(dead_code)]
pub(crate) fn run_external(bin_name: &str, args: &[String]) -> ExitCode {
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_default();
    // Try same dir as turingos itself first; fallback to release; then debug
    let candidates = [
        exe_dir.join(bin_name),
        exe_dir.join("../release").join(bin_name),
        exe_dir.join("../debug").join(bin_name),
    ];
    let bin_path = candidates
        .iter()
        .find(|p| p.exists())
        .cloned()
        .unwrap_or_else(|| {
            // Allow override via env var
            std::env::var("TURINGOS_BIN_DIR")
                .ok()
                .map(|d| std::path::PathBuf::from(d).join(bin_name))
                .unwrap_or_else(|| std::path::PathBuf::from(bin_name))
        });
    let status = Command::new(&bin_path)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();
    match status {
        Ok(s) => ExitCode::from(s.code().unwrap_or(1) as u8),
        Err(e) => {
            eprintln!("turingos: failed to invoke '{}': {}", bin_path.display(), e);
            ExitCode::from(2)
        }
    }
}
