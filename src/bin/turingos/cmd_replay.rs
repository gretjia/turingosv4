//! TRACE_MATRIX FC2-N16: turingos `replay` handler
//!
//! Runs the 7-indicator ChainTape replay verification on an evidence
//! directory. Task-type agnostic — applies to any ChainTape regardless of
//! task domain (proof / polymarket / multi-agent / future). Implementation
//! currently shells out to `TASK_RUNNER_BIN`; not surfaced in user help.

use std::process::ExitCode;

use crate::common::{run_external, TASK_RUNNER_BIN};

/// TRACE_MATRIX FC2-N16: `replay` short-help
pub(crate) const SHORT_HELP: &str = "Run 7-indicator ChainTape replay verification";

/// TRACE_MATRIX FC2-N16: `replay` full --help text
pub(crate) const FULL_HELP: &str = r#"turingos replay — 7-indicator ChainTape replay verification

USAGE:
    turingos replay [OPTIONS]

DESCRIPTION:
    Replays the ChainTape read-only and prints the 7-indicator verify
    report. Exits 0 if all indicators are GREEN, non-zero otherwise.

    Read-only. No sequencer call. No ChainTape advance. Works for any
    task type.

OPTIONS:
    Pass through flags accepted by the task-runner backend; common:
    `--chaintape <PATH>` (evidence directory).

EXAMPLES:
    turingos replay --chaintape ./handover/evidence/run001/chaintape

SEE ALSO:
    turingos report run --help        Show run summary
    turingos verify chaintape --help  ChainTape structural verification
"#;

/// TRACE_MATRIX FC2-N16: `replay` dispatch entry
pub(crate) fn run(args: &[String]) -> ExitCode {
    if args.len() == 1 && args[0] == "--help" {
        print!("{FULL_HELP}");
        return ExitCode::SUCCESS;
    }
    let mut forwarded: Vec<String> = Vec::with_capacity(args.len() + 1);
    forwarded.push("view-replay".to_owned());
    forwarded.extend_from_slice(args);
    run_external(TASK_RUNNER_BIN, &forwarded)
}
