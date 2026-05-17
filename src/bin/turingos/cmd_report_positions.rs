//! TRACE_MATRIX FC2-N16: turingos `report positions` handler
//!
//! Replays the NodePositionsIndex (exposure record) from a ChainTape
//! evidence directory. Task-type agnostic — applies to any market that
//! uses the TuringOS exposure-record pattern. Implementation currently
//! shells out to `TASK_RUNNER_BIN`; not surfaced in user help.

use std::process::ExitCode;

use crate::common::{run_external, TASK_RUNNER_BIN};

/// TRACE_MATRIX FC2-N16: `report positions` short-help
pub(crate) const SHORT_HELP: &str =
    "Replay NodePositionsIndex exposure record from a ChainTape evidence directory";

/// TRACE_MATRIX FC2-N16: `report positions` full --help text
pub(crate) const FULL_HELP: &str = r#"turingos report positions — Replay exposure record

USAGE:
    turingos report positions [OPTIONS]

DESCRIPTION:
    Replays the NodePositionsIndex from a ChainTape evidence directory and
    prints each agent's exposure. NodePositionsIndex is an exposure index
    (Art. II.2): it records agent YES/NO share holdings reconstructed from
    accepted L4 WorkTx events. It is NOT a trading market or Coin balance —
    it is a view derived from ChainTape.

    Read-only. No sequencer call. No typed_tx. No CAS write. No ChainTape
    advance. Works for any task type that maintains a positions index.

OPTIONS:
    Pass through flags accepted by the task-runner backend; common:
    `--chaintape <PATH>` (evidence directory), `--agent <ID>` (filter).

EXAMPLES:
    turingos report positions
    turingos report positions --agent agent_0
"#;

/// TRACE_MATRIX FC2-N16: `report positions` dispatch entry
pub(crate) fn run(args: &[String]) -> ExitCode {
    if args.iter().any(|a| a == "-h" || a == "--help") {
        print!("{FULL_HELP}");
        return ExitCode::SUCCESS;
    }
    let mut forwarded = Vec::with_capacity(args.len() + 1);
    forwarded.push("view-positions".to_string());
    forwarded.extend_from_slice(args);
    run_external(TASK_RUNNER_BIN, &forwarded)
}
