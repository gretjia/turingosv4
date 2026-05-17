//! TRACE_MATRIX FC2-N16: turingos `task tick` handler
//!
//! Advances the G3 carry-forward epoch by emitting a system tick. Task-type
//! agnostic — applies to any ChainTape that maintains an epoch / clock.
//! Class 2 write-capable: may emit system tx (ChainTape state advance,
//! CAS evidence writes). Implementation currently shells out to
//! `TASK_RUNNER_BIN`; not surfaced in user help.

use std::process::ExitCode;

use crate::common::{run_external, TASK_RUNNER_BIN};

/// TRACE_MATRIX FC2-N16: `task tick` short-help
pub(crate) const SHORT_HELP: &str = "Advance the G3 carry-forward epoch (system tick)";

/// TRACE_MATRIX FC2-N16: `task tick` full --help text
pub(crate) const FULL_HELP: &str = r#"turingos task tick — Advance the carry-forward epoch

USAGE:
    turingos task tick [OPTIONS]

DESCRIPTION:
    Emits a system tick to advance the G3 carry-forward epoch on a
    ChainTape. Class 2 write-capable: may emit system tx via the existing
    sequencer path (ChainTape state advance, CAS evidence writes).

    Task-type agnostic — works for any task that maintains a clock.

OPTIONS:
    Pass through flags accepted by the task-runner backend; common:
    `--chaintape <PATH>`, `--epochs <N>` (number of epochs to advance).
"#;

/// TRACE_MATRIX FC2-N16: `task tick` dispatch entry
pub(crate) fn run(args: &[String]) -> ExitCode {
    if args.iter().any(|a| a == "--help" || a == "-h") {
        print!("{FULL_HELP}");
        return ExitCode::SUCCESS;
    }
    let mut prepended: Vec<String> = Vec::with_capacity(args.len() + 1);
    prepended.push("tick".to_string());
    prepended.extend_from_slice(args);
    run_external(TASK_RUNNER_BIN, &prepended)
}
