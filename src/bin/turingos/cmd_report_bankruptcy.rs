//! TRACE_MATRIX FC2-N16: turingos report bankruptcy handler (lean_market view-bankruptcy wrapper)
//!
//! Phase 6.1 W1a.4 atom. Read-only shell-out to `lean_market view-bankruptcy`.
//! 0 sequencer call; 0 typed_tx; 0 CAS write; 0 ChainTape advance.
//!
//! FC-trace: FC2-N16 (boot / genesis / tape replay view).
//! TB-11 / TB-12: RunExhausted / Bankruptcy evidence viewer.

use std::process::ExitCode;

use crate::common::run_external;

// ─────────────────────────────────────────────────────────────────────
// Public surface — pub(crate) only; never escapes the turingos binary.
// ─────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC2-N16: short help string for dispatch table
pub(crate) const SHORT_HELP: &str = "Show RunExhausted / Bankruptcy evidence (TB-11/TB-12 view)";

/// TRACE_MATRIX FC2-N16: full help text printed on --help
pub(crate) const FULL_HELP: &str = r#"turingos report bankruptcy — RunExhausted / Bankruptcy evidence viewer

USAGE:
    turingos report bankruptcy [OPTIONS]

DESCRIPTION:
    Shells out to `lean_market view-bankruptcy` and forwards all arguments
    verbatim. This is a read-only view; no sequencer call is made and no
    ChainTape state is advanced.

    The underlying command (`lean_market view-bankruptcy`) reads the ChainTape
    and CAS to enumerate tasks that entered TaskMarketState::Bankrupt or
    RunExhausted, as defined by the TB-11 EvidenceCapsule / TB-12
    NodePositionsIndex substrate.

OPTIONS (forwarded to lean_market view-bankruptcy):
    --chaintape <path>   Path to the ChainTape directory (required by
                         lean_market; see lean_market --help for details)
    --help, -h           Print this help and exit (handled before shell-out)

EXAMPLES:
    turingos report bankruptcy --chaintape ./handover/evidence/run001/chaintape

SEE ALSO:
    lean_market view-bankruptcy --help   Upstream command help
    turingos report run --help           Show run summary
    turingos report wallet --help        Show wallet balances
"#;

// ─────────────────────────────────────────────────────────────────────
// Entry point
// ─────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC2-N16: handler for `turingos report bankruptcy`
///
/// Short-circuits on `--help` / `-h` before shelling out. All other
/// arguments are prepended with `view-bankruptcy` and forwarded to
/// `lean_market`.
pub(crate) fn run(args: &[String]) -> ExitCode {
    // --help / -h short-circuit: print full help, exit 0.
    if args.iter().any(|a| a == "--help" || a == "-h") {
        print!("{FULL_HELP}");
        return ExitCode::SUCCESS;
    }

    // Prepend the lean_market subcommand token, then forward remaining args.
    let mut forwarded: Vec<String> = Vec::with_capacity(args.len() + 1);
    forwarded.push("view-bankruptcy".to_owned());
    forwarded.extend_from_slice(args);

    run_external("lean_market", &forwarded)
}
