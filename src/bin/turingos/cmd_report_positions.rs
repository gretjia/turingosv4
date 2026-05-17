//! TRACE_MATRIX FC2-N16: turingos report positions handler (lean_market view-positions wrapper)
//!
//! Phase 6.1 W1a.3 atom. Shell-out wrapper around `lean_market view-positions`.
//! Read-only ChainTape replay view — no sequencer call, no typed_tx, no CAS write,
//! no ChainTape advance. Forwards all user args after prepending `view-positions`.

use std::process::ExitCode;

use crate::common::run_external;

/// TRACE_MATRIX FC2-N16: report positions subcommand short-help (registry display)
pub(crate) const SHORT_HELP: &str = "Show NodePositionsIndex exposure record (TB-12 view)";

/// TRACE_MATRIX FC2-N16: report positions subcommand --help text
pub(crate) const FULL_HELP: &str = r#"turingos report positions — Show NodePositionsIndex exposure record (TB-12 view)

USAGE:
    turingos report positions [OPTIONS]

DESCRIPTION:
    Wrapper around `lean_market view-positions`. Prepends `view-positions` to
    any additional args and shell-outs to the lean_market binary located in the
    same target directory as turingos (release preferred, then debug).

    Read-only ChainTape replay view. No sequencer call. No typed_tx. No CAS
    write. No ChainTape advance.

    NodePositionsIndex is an exposure index (TB-12): it records agent YES/NO
    share holdings reconstructed from accepted L4 WorkTx events. It is NOT a
    trading market or Coin balance — it is a view derived from ChainTape.

OPTIONS:
    Any flags accepted by `lean_market view-positions` are passed through.

    -h, --help              Print this help (handled by turingos wrapper;
                            does not reach lean_market).

EXAMPLES:
    turingos report positions
    turingos report positions --agent agent_0
"#;

/// TRACE_MATRIX FC2-N16: report positions subcommand dispatch entry
pub(crate) fn run(args: &[String]) -> ExitCode {
    // Short-circuit --help / -h before shell-out.
    if args.iter().any(|a| a == "-h" || a == "--help") {
        print!("{}", FULL_HELP);
        return ExitCode::SUCCESS;
    }

    // Prepend the lean_market subcommand token, then forward remaining user args.
    let mut forwarded = Vec::with_capacity(args.len() + 1);
    forwarded.push("view-positions".to_string());
    forwarded.extend_from_slice(args);

    run_external("lean_market", &forwarded)
}
