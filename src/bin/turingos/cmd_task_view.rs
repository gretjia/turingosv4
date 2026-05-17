//! TRACE_MATRIX FC2-N16: turingos `task view` handler
//!
//! Replays task status from a ChainTape evidence directory. Task-type
//! agnostic — applies to any task that follows the TuringOS task-market
//! pattern. Implementation currently shells out to `TASK_RUNNER_BIN`;
//! not surfaced in user help.

use std::process::ExitCode;

use crate::common::{run_external, TASK_RUNNER_BIN};

/// TRACE_MATRIX FC2-N16: `task view` short-help
pub(crate) const SHORT_HELP: &str = "Replay task status from a ChainTape evidence directory";

/// TRACE_MATRIX FC2-N16: `task view` full --help text
pub(crate) const FULL_HELP: &str = r#"turingos task view — Show task status

USAGE:
    turingos task view [OPTIONS]

DESCRIPTION:
    Replays the ChainTape and reports the current status of the requested
    task (open / accepted / rejected / finalized / bankrupt / expired).
    Read-only. No sequencer call. No ChainTape advance. Works for any
    task type.

OPTIONS:
    Pass through flags accepted by the task-runner backend; common:
    `--chaintape <PATH>`, `--task-id <ID>`.
"#;

/// TRACE_MATRIX FC2-N16: `task view` dispatch entry
pub(crate) fn run(args: &[String]) -> ExitCode {
    if args.iter().any(|a| a == "-h" || a == "--help") && args.len() == 1 {
        println!("{FULL_HELP}");
        return ExitCode::SUCCESS;
    }
    let mut prepended: Vec<String> = vec!["view-task".to_string()];
    prepended.extend_from_slice(args);
    run_external(TASK_RUNNER_BIN, &prepended)
}
