//! TRACE_MATRIX FC2-N16: turingos render handler (UI IR fixture renderer)
//!
//! Phase 6.2 W1.1 atom. Shell-out wrapper for the Python UI IR renderer at
//! `experiments/tisr_ui_spike/render.py`. Supports `--fixture <path>` and
//! `--format text|json` (passed through to Python). Pure Class 1: filesystem
//! read of fixture JSON + Python invocation; no sequencer/typed_tx/CAS write;
//! no Lean dependency; no Cargo.toml touch (Trust Root intact).
//!
//! §8 compliance: Phase 6.2 §4 allowed paths; FC2-N16 + FC3-N31.

use std::io::Write;
use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Duration;

use turingosv4::sdk::sanitized_runner::{
    env_allowlist_from_current, run_sanitized, SanitizedCommand,
};

// ─────────────────────────────────────────────────────────────────────
// Short/full help strings
// ─────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC2-N16: render subcommand short-help (registry display)
pub(crate) const SHORT_HELP: &str =
    "Render a UI IR fixture or JSON document to text/json (UI IR spike consumer)";

/// TRACE_MATRIX FC2-N16: render subcommand --help text
pub(crate) const FULL_HELP: &str = r#"turingos render — Render a UI IR fixture or JSON document

USAGE:
    turingos render [--fixture <PATH>] [--format text|json] [--help]

OPTIONS:
    --fixture <PATH>    Path to a UI IR JSON fixture file.
                        If omitted, reads from stdin.

    --format <FMT>      Output format: 'text' (default) or 'json'
                        (identity round-trip / validation).

    -h, --help          Print this help.

DESCRIPTION:
    Shell-out wrapper for experiments/tisr_ui_spike/render.py.
    Resolves render.py relative to the turingos binary (up two directory
    levels from target/{debug,release}/turingos) with a fallback to
    <current-working-directory>/experiments/tisr_ui_spike/render.py.

    Requires Python 3 to be on PATH and render.py to be present.

    FC3-N31: output is a materialized view — never authoritative over
    ChainTape / CAS / constitution gates.

    Class 1: no sequencer call; no typed_tx; no CAS write; no ChainTape
    advance; no Cargo.toml touch.

EXIT CODES:
    0   Success.
    1   Validation failure (from Python renderer).
    2   Argument error, missing Python, or render.py not found.
"#;

// ─────────────────────────────────────────────────────────────────────
// Render.py resolution
// ─────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC2-N16: resolve experiments/tisr_ui_spike/render.py
///
/// Resolution order:
///   1. `<turingos_exe>/../../../experiments/tisr_ui_spike/render.py`
///      (exe is at target/{debug,release}/turingos; two parents up = project root)
///   2. `<CWD>/experiments/tisr_ui_spike/render.py`
///
/// Returns `None` if neither candidate exists.
fn resolve_render_py() -> Option<PathBuf> {
    // Candidate 1: relative to the turingos binary location.
    if let Ok(exe) = std::env::current_exe() {
        // exe -> target/debug/turingos  => parent = target/debug  => parent = target  => parent = project root
        if let Some(project_root) = exe
            .parent() // target/{debug,release}/
            .and_then(|p| p.parent()) // target/
            .and_then(|p| p.parent())
        // project root
        {
            let candidate = project_root
                .join("experiments")
                .join("tisr_ui_spike")
                .join("render.py");
            if candidate.exists() {
                return Some(candidate);
            }
        }
    }

    // Candidate 2: CWD fallback.
    if let Ok(cwd) = std::env::current_dir() {
        let candidate = cwd
            .join("experiments")
            .join("tisr_ui_spike")
            .join("render.py");
        if candidate.exists() {
            return Some(candidate);
        }
    }

    None
}

// ─────────────────────────────────────────────────────────────────────
// Subcommand entry point
// ─────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC2-N16: render subcommand dispatch entry
pub(crate) fn run(args: &[String]) -> ExitCode {
    // Short-circuit --help before shelling out; do NOT forward to Python.
    if args.len() == 1 && (args[0] == "-h" || args[0] == "--help") {
        print!("{}", FULL_HELP);
        return ExitCode::SUCCESS;
    }
    // Also catch --help mixed with other args (consistent with other subcommands).
    if args.iter().any(|a| a == "-h" || a == "--help") {
        print!("{}", FULL_HELP);
        return ExitCode::SUCCESS;
    }

    // Resolve render.py.
    let render_py = match resolve_render_py() {
        Some(p) => p,
        None => {
            eprintln!(
                "turingos render: render.py not found.\n\
                 Expected at experiments/tisr_ui_spike/render.py relative to \
                 the project root or the current working directory.\n\
                 Is the turingosv4 project tree intact?"
            );
            return ExitCode::from(2);
        }
    };

    // Shell out: python3 <render.py> <args...>
    let mut cmd_args = Vec::with_capacity(1 + args.len());
    cmd_args.push(render_py.to_string_lossy().into_owned());
    cmd_args.extend(args.iter().cloned());
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let output = run_sanitized(SanitizedCommand {
        program: PathBuf::from("python3"),
        args: cmd_args,
        cwd,
        env: env_allowlist_from_current(&["PATH"]),
        stdin: None,
        timeout: Duration::from_secs(120),
    });

    match output {
        Ok(out) => {
            let _ = std::io::stdout().write_all(&out.stdout);
            let _ = std::io::stderr().write_all(&out.stderr);
            ExitCode::from(out.exit_code.unwrap_or(1) as u8)
        }
        Err(e) => {
            eprintln!("turingos render: failed to invoke python3: {e}");
            eprintln!("  render.py: {}", render_py.display());
            eprintln!("  Ensure python3 is on PATH.");
            ExitCode::from(2)
        }
    }
}
