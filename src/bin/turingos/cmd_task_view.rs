//! TRACE_MATRIX FC2-N16: turingos task view handler (lean_market view-task wrapper)

use std::process::ExitCode;

use crate::common::run_external;

/// TRACE_MATRIX FC2-N16: `task view` short-help
pub(crate) const SHORT_HELP: &str =
    "Show task status by replaying the chaintape (lean_market view-task)";

/// TRACE_MATRIX FC2-N16: `task view` full --help text
pub(crate) const FULL_HELP: &str = r#"turingos task view — Show task status

USAGE:
    turingos task view [OPTIONS]

DESCRIPTION:
    Thin shell-out wrapper around `lean_market view-task`. All arguments
    are passed through to lean_market after the `view-task` subcommand.

    Run `lean_market view-task --help` for the canonical option list.

    No sequencer call. Read-only chaintape replay.

    Wraps: lean_market view-task ...
"#;

/// TRACE_MATRIX FC2-N16: `task view` dispatch entry
pub(crate) fn run(args: &[String]) -> ExitCode {
    if args.iter().any(|a| a == "-h" || a == "--help") && args.len() == 1 {
        println!("{}", FULL_HELP);
        return ExitCode::SUCCESS;
    }
    // Prepend the lean_market subcommand name
    let mut prepended: Vec<String> = vec!["view-task".to_string()];
    prepended.extend_from_slice(args);
    run_external("lean_market", &prepended)
}
