//! TRACE_MATRIX FC2-N16: turingos CLI shared helpers
//!
//! Phase 6.0/6.1 W0 foundation atom. Holds helpers shared across
//! `src/bin/turingos/cmd_*.rs` submodules. All public surface scoped
//! `pub(crate)` — never escapes the `turingos` binary crate.

use std::io::Write;
use std::path::Path;
use std::process::ExitCode;
use std::time::Duration;

use turingosv4::sdk::sanitized_runner::{
    env_allowlist_from_current, run_sanitized, SanitizedCommand,
};

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

/// TRACE_MATRIX FC2-N16: default task-runner backend binary (Phase 6.1)
///
/// Phase 6.1 implementation note (NOT user-facing): TuringOS today wraps the
/// TB-10-era `lean_market` binary because it currently hosts the generic
/// ChainTape replay / wallet / positions / bankruptcy / task-lifecycle
/// kernel operations. The "lean_" prefix is historical — the wrapped
/// operations are NOT Lean-specific.
///
/// Phase 7+ generalization plan: these operations move into a generic
/// `agent_runner` binary. When that lands, this constant is the single
/// point of change.
///
/// User-facing `turingos --help` etc. must NEVER mention this name —
/// it's an implementation detail.
///
/// Tests verify the shell-out plumbing by injecting `PATH` to redirect
/// this name to a stub binary (e.g. `/bin/echo`); production resolution
/// falls back to the same PATH lookup when `target/release|debug/` does
/// not contain the binary.
pub(crate) const TASK_RUNNER_BIN: &str = "lean_market";

/// TRACE_MATRIX FC2-N16: invoke an external project binary (shell-out wrapper)
///
/// Resolution order (most-explicit first):
///   1. `$TURINGOS_BIN_DIR/<bin_name>` if the env var is set and the file
///      exists. This is the test/operator override knob — tests point it
///      at a tempdir holding a stub like `/bin/echo` to verify shell-out
///      plumbing without needing any real backend binary on disk.
///   2. `<turingos_exe_dir>/<bin_name>` (sibling install).
///   3. `<turingos_exe_dir>/../release/<bin_name>` and `.../debug/<bin_name>`
///      (typical layout when `turingos` is in `target/debug/` or `target/release/`).
///   4. bare `bin_name` → `Command::new` PATH search.
///
/// Inherits stdin/stdout/stderr. Returns child exit code on success, or
/// `ExitCode::from(2)` with a clear stderr message if the binary cannot be
/// invoked at all (preserves user trust: not silent, not a panic).
///
/// For multi-token wrapped subcommands, callers pre-pend the subcommand
/// token to `args[0]`. For the generic task-runner backend, prefer the
/// `TASK_RUNNER_BIN` constant over hard-coding the binary name — this
/// keeps Phase 7+ generalization a single-point change.
///
/// W0 foundation atom; consumers land throughout Wave 1.
#[allow(dead_code)]
pub(crate) fn run_external(bin_name: &str, args: &[String]) -> ExitCode {
    // 1. Explicit override (tests / operators).
    let bin_path: std::path::PathBuf = if let Ok(d) = std::env::var("TURINGOS_BIN_DIR") {
        let p = std::path::PathBuf::from(d).join(bin_name);
        if p.exists() {
            p
        } else {
            // Override set but stub missing → fall through to defaults.
            resolve_default(bin_name)
        }
    } else {
        resolve_default(bin_name)
    };
    let cwd = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    let output = run_sanitized(SanitizedCommand {
        program: bin_path.clone(),
        args: args.to_vec(),
        cwd,
        env: env_allowlist_from_current(&["PATH"]),
        stdin: None,
        timeout: Duration::from_secs(300),
    });
    match output {
        Ok(out) => {
            let _ = std::io::stdout().write_all(&out.stdout);
            let _ = std::io::stderr().write_all(&out.stderr);
            ExitCode::from(out.exit_code.unwrap_or(1) as u8)
        }
        Err(e) => {
            use std::io::ErrorKind;
            if matches!(e.kind(), ErrorKind::NotFound) {
                // Backend-agnostic guidance per Phase 7 generalization posture:
                // never name the specific backend binary in the user-visible
                // message. The kernel does not assume a particular backend
                // (today's implementation happens to use one; tomorrow's
                // generic agent_runner replaces it). The 3 resolution paths
                // below are sufficient; an internal binary name in a debug
                // tail line would leak an implementation detail.
                eprintln!("turingos: a required backend binary for this command is not available.");
                eprintln!();
                eprintln!("  Resolution paths:");
                eprintln!("    1. Build all workspace binaries (recommended):");
                eprintln!("         cargo build --workspace");
                eprintln!("    2. If you only want to preview UI views (no backend needed):");
                eprintln!("         turingos render --fixture <path-to-fixture.json>");
                eprintln!("       Fixtures: experiments/tisr_ui_spike/fixtures/");
                eprintln!("    3. If you have a custom build directory, set:");
                eprintln!("         TURINGOS_BIN_DIR=<dir>");
                ExitCode::from(2)
            } else {
                eprintln!("turingos: backend invocation failed: {}", e);
                ExitCode::from(2)
            }
        }
    }
}

/// Default resolution chain for an external binary (no env override).
///
/// Walks the sibling and `target/{release,debug}/` candidates next to
/// `current_exe()`; if none exist, returns the bare name so
/// `Command::new` performs a PATH search.
fn resolve_default(bin_name: &str) -> std::path::PathBuf {
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_default();
    let candidates = [
        exe_dir.join(bin_name),
        exe_dir.join("../release").join(bin_name),
        exe_dir.join("../debug").join(bin_name),
    ];
    candidates
        .iter()
        .find(|p| p.exists())
        .cloned()
        .unwrap_or_else(|| std::path::PathBuf::from(bin_name))
}
