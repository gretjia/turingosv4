//! TRACE_MATRIX FC2-N16: turingos task tick handler (lean_market tick wrapper)
//!
//! Phase 6.1 W3.2 atom. Thin shell-out wrapper around `lean_market tick`.
//! Prepends the `tick` subcommand token to user-supplied args before
//! delegating to `run_external`; `--help` is short-circuited to print
//! FULL_HELP inline.
//!
//! §8 packet 2026-05-17 (TISR Phase 6.0/6.1 separate charter).
//! [R-022-skip: see handover/alignment/OBS_R022_TISR_PHASE6_1_CLI_DISPATCH.md]

use std::process::ExitCode;

use crate::common::run_external;

/// TRACE_MATRIX FC2-N16: short help shown in `turingos --help` listing
pub(crate) const SHORT_HELP: &str = "Run TB-11 G3 carry-forward tick (lean_market tick)";

/// TRACE_MATRIX FC2-N16: full help printed by `turingos task tick --help`
pub(crate) const FULL_HELP: &str = r#"turingos task tick — Run TB-11 G3 carry-forward tick (lean_market tick)

USAGE:
    turingos task tick [OPTIONS]

DESCRIPTION:
    Thin shell-out wrapper around `lean_market tick`.
    Prepends the `tick` subcommand token, then forwards all remaining
    user-supplied args to `lean_market`.

    Class 2 write-capable: lean_market tick may emit system tx via the
    existing TB-10 path (ChainTape state advance, CAS evidence writes).
    Run `lean_market tick --help` for the canonical option reference.

    Wraps: lean_market tick [OPTIONS]
"#;

/// TRACE_MATRIX FC2-N16: entry point for `turingos task tick`
///
/// Short-circuits `--help` / `-h` to print FULL_HELP locally (preserving
/// the FC2-N16 trace reference in the help output). Otherwise prepends
/// `tick` as the first argument and delegates to `lean_market`.
pub(crate) fn run(args: &[String]) -> ExitCode {
    // Short-circuit --help / -h before invoking the wrapped binary so the
    // user sees the wrapper's canonical help (which includes FC2-N16 trace).
    if args.iter().any(|a| a == "--help" || a == "-h") {
        print!("{FULL_HELP}");
        return ExitCode::SUCCESS;
    }

    // Prepend the `tick` subcommand token that lean_market expects.
    let mut prepended: Vec<String> = Vec::with_capacity(args.len() + 1);
    prepended.push("tick".to_string());
    prepended.extend_from_slice(args);

    run_external("lean_market", &prepended)
}
