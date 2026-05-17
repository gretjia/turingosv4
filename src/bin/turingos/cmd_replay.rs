//! TRACE_MATRIX FC2-N16: turingos replay handler (lean_market view-replay wrapper)
//!
//! Phase 6.1 W1c.12 atom. Read-only shell-out to `lean_market view-replay`.
//! 0 sequencer call; 0 typed_tx; 0 CAS write; 0 ChainTape advance.
//!
//! FC-trace: FC2-N16 (boot / genesis / tape replay view).
//! TB-10: 7-indicator chaintape replay verification.

use std::process::ExitCode;

use crate::common::run_external;

// ─────────────────────────────────────────────────────────────────────
// Public surface — pub(crate) only; never escapes the turingos binary.
// ─────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC2-N16: short help string for dispatch table
pub(crate) const SHORT_HELP: &str =
    "Run 7-indicator chaintape replay verification (lean_market view-replay)";

/// TRACE_MATRIX FC2-N16: full help text printed on --help
pub(crate) const FULL_HELP: &str = r#"turingos replay — 7-indicator chaintape replay verification

USAGE:
    turingos replay [OPTIONS]

DESCRIPTION:
    Shells out to `lean_market view-replay` and forwards all arguments
    verbatim (prepending the `view-replay` subcommand token). This is a
    read-only view; no sequencer call is made and no ChainTape state is
    advanced.

    The underlying command (`lean_market view-replay`) replays the ChainTape
    read-only and prints the 7-indicator verify report. Exits 0 if all
    indicators are GREEN, non-zero otherwise.

OPTIONS (forwarded to lean_market view-replay):
    --chaintape <path>   Path to the ChainTape directory (required by
                         lean_market; see lean_market --help for details)
    --help, -h           Print this help and exit (handled before shell-out)

EXAMPLES:
    turingos replay --chaintape ./handover/evidence/run001/chaintape

SEE ALSO:
    lean_market view-replay --help   Upstream command help
    turingos report run --help       Show run summary
    turingos verify chaintape --help Chaintape structural verification
"#;

// ─────────────────────────────────────────────────────────────────────
// Entry point
// ─────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC2-N16: handler for `turingos replay`
///
/// Short-circuits on `--help` when it is the sole argument (args.len() == 1).
/// All other arguments are prepended with `view-replay` and forwarded to
/// `lean_market`.
pub(crate) fn run(args: &[String]) -> ExitCode {
    // --help short-circuit: single --help arg exits 0 with full help.
    if args.len() == 1 && args[0] == "--help" {
        print!("{FULL_HELP}");
        return ExitCode::SUCCESS;
    }

    // Prepend the lean_market subcommand token, then forward remaining args.
    let mut forwarded: Vec<String> = Vec::with_capacity(args.len() + 1);
    forwarded.push("view-replay".to_owned());
    forwarded.extend_from_slice(args);

    run_external("lean_market", &forwarded)
}
