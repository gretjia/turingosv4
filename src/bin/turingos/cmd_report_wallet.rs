//! TRACE_MATRIX FC2-N16: turingos `report wallet` handler
//!
//! Replays the agent EconomicState (wallet balances) from a ChainTape
//! evidence directory. Task-type agnostic — applies to proof / polymarket /
//! multi-agent / any future task type that uses the TuringOS task-market
//! pattern. Implementation currently shells out to `TASK_RUNNER_BIN`
//! (see `common.rs`); this is intentionally NOT surfaced in user help.

use std::process::ExitCode;

use crate::common::{run_external, TASK_RUNNER_BIN};

/// TRACE_MATRIX FC2-N16: `report wallet` short-help
pub(crate) const SHORT_HELP: &str =
    "Replay agent wallet balances from a ChainTape evidence directory";

/// TRACE_MATRIX FC2-N16: `report wallet` full --help text
pub(crate) const FULL_HELP: &str = r#"turingos report wallet — Replay agent wallet balances

USAGE:
    turingos report wallet [OPTIONS]

DESCRIPTION:
    Replays the EconomicState from a ChainTape evidence directory and prints
    every agent's wallet balance. Read-only. No sequencer call. No CAS write.
    No ChainTape advance.

    Works for any task type (proof / polymarket / multi-agent / future).

OPTIONS:
    Pass through any flags the task-runner backend accepts; see the
    task-runner's `--help` for the canonical option reference. Common flags:
    `--chaintape <PATH>` (evidence directory), `--out <FORMAT>` (text/json).
"#;

/// TRACE_MATRIX FC2-N16: `report wallet` dispatch entry
pub(crate) fn run(args: &[String]) -> ExitCode {
    if args.iter().any(|a| a == "-h" || a == "--help") && args.len() == 1 {
        println!("{FULL_HELP}");
        return ExitCode::SUCCESS;
    }
    // Prepend the task-runner subcommand token for the wallet view operation.
    let mut prepended: Vec<String> = vec!["view-wallet".to_string()];
    prepended.extend_from_slice(args);
    run_external(TASK_RUNNER_BIN, &prepended)
}
