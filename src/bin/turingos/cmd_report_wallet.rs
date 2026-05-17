//! TRACE_MATRIX FC2-N16: turingos report wallet handler (lean_market view-wallet wrapper)

use std::process::ExitCode;

use crate::common::run_external;

/// TRACE_MATRIX FC2-N16: `report wallet` short-help
pub(crate) const SHORT_HELP: &str = "Show agent wallet balances by replaying the chaintape";

/// TRACE_MATRIX FC2-N16: `report wallet` full --help text
pub(crate) const FULL_HELP: &str = r#"turingos report wallet — Show wallet balances

USAGE:
    turingos report wallet [OPTIONS]

DESCRIPTION:
    Thin shell-out wrapper around `lean_market view-wallet`. All arguments
    are passed through to lean_market after the `view-wallet` subcommand.

    Run `lean_market view-wallet --help` for the canonical option list.

    No sequencer call. Read-only chaintape replay.

    Wraps: lean_market view-wallet ...
"#;

/// TRACE_MATRIX FC2-N16: `report wallet` dispatch entry
pub(crate) fn run(args: &[String]) -> ExitCode {
    if args.iter().any(|a| a == "-h" || a == "--help") && args.len() == 1 {
        println!("{}", FULL_HELP);
        return ExitCode::SUCCESS;
    }
    // Prepend the lean_market subcommand name
    let mut prepended: Vec<String> = vec!["view-wallet".to_string()];
    prepended.extend_from_slice(args);
    run_external("lean_market", &prepended)
}
