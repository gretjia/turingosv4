//! TRACE_MATRIX FC2-N16: turingos `report bankruptcy` handler
//!
//! Replays the RunExhausted / Bankruptcy condition from a ChainTape +
//! CAS evidence directory. Task-type agnostic — applies to any task that
//! enters TaskMarketState::Bankrupt. Implementation currently shells out
//! to `TASK_RUNNER_BIN`; not surfaced in user help.

use std::process::ExitCode;

use crate::common::{run_external, TASK_RUNNER_BIN};

/// TRACE_MATRIX FC2-N16: `report bankruptcy` short-help
pub(crate) const SHORT_HELP: &str =
    "Replay RunExhausted / Bankruptcy evidence from a ChainTape evidence directory";

/// TRACE_MATRIX FC2-N16: `report bankruptcy` full --help text
pub(crate) const FULL_HELP: &str = r#"turingos report bankruptcy — RunExhausted / Bankruptcy evidence viewer

USAGE:
    turingos report bankruptcy [OPTIONS]

DESCRIPTION:
    Replays the ChainTape + CAS to enumerate tasks that entered
    TaskMarketState::Bankrupt or RunExhausted, as defined by the
    EvidenceCapsule / NodePositionsIndex substrate. Read-only. No sequencer
    call, no typed_tx, no CAS write, no ChainTape advance.

    Works for any task type that uses the TuringOS bankruptcy condition.

OPTIONS:
    Pass through flags accepted by the task-runner backend; common:
    `--chaintape <PATH>` (evidence directory).

EXAMPLES:
    turingos report bankruptcy --chaintape ./handover/evidence/run001/chaintape

SEE ALSO:
    turingos report run --help           Show run summary
    turingos report wallet --help        Show wallet balances
"#;

/// TRACE_MATRIX FC2-N16: `report bankruptcy` dispatch entry
pub(crate) fn run(args: &[String]) -> ExitCode {
    if args.iter().any(|a| a == "--help" || a == "-h") {
        print!("{FULL_HELP}");
        return ExitCode::SUCCESS;
    }
    let mut forwarded: Vec<String> = Vec::with_capacity(args.len() + 1);
    forwarded.push("view-bankruptcy".to_owned());
    forwarded.extend_from_slice(args);
    run_external(TASK_RUNNER_BIN, &forwarded)
}
