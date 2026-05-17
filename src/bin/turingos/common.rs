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

/// TRACE_MATRIX FC2-N16: invoke an external project binary (shell-out wrapper)
///
/// Resolves `bin_name` relative to the turingos binary's parent dir. Tries
/// release first, then debug. Inherits stdin/stdout/stderr. Returns child
/// exit code (preserving exit semantics for replay).
///
/// For multi-token wrapped subcommands (e.g. `lean_market view-wallet`), pass
/// the subcommand as args[0] from the caller.
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
