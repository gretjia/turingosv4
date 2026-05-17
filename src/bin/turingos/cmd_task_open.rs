//! TRACE_MATRIX FC2-N16: turingos `task open` handler
//!
//! Bootstraps a new TaskOpen + EscrowLock pair on a fresh ChainTape and
//! launches the configured task-runner backend. Task-type agnostic — the
//! caller specifies the task adapter / problem-ID via backend args; the
//! TuringOS task-market pattern itself is domain-neutral (works for proof,
//! polymarket, multi-agent, future generic compute, etc.).
//!
//! Class 2 write-capable: signs and posts TaskOpen + EscrowLock via the
//! TB-9 durable keystore and forks the evaluator child process. This
//! wrapper only routes; it never calls the sequencer or CAS directly.

use std::process::ExitCode;

use crate::common::{run_external, TASK_RUNNER_BIN};

/// TRACE_MATRIX FC2-N16: `task open` short-help
pub(crate) const SHORT_HELP: &str =
    "Open a task on a fresh ChainTape (signs TaskOpen + EscrowLock, forks evaluator)";

/// TRACE_MATRIX FC2-N16: `task open` full --help text
pub(crate) const FULL_HELP: &str = r#"turingos task open — Open a task on a fresh ChainTape

USAGE:
    turingos task open [OPTIONS]

DESCRIPTION:
    Bootstraps a fresh ChainTape, signs and posts TaskOpen + EscrowLock
    transitions using the Agent_user_0 TB-9 durable keystore, then forks
    the evaluator child process to run the task-checking loop on the
    specified problem.

    Class 2 write-capable. Task-type agnostic — the specific task adapter
    is selected via backend args (e.g. proof / polymarket / multi-agent /
    future generic compute). The TuringOS task-market pattern itself is
    domain-neutral.

    FC-trace: FC2-N16 (bootstrap / genesis gate — TaskOpen + EscrowLock
    are the canonical on_init-style tape anchors for a new task).

OPTIONS:
    Pass through flags accepted by the task-runner backend; common:
    `--problem <ID>`, `--bounty <MICRO>`, `--chaintape <PATH>`,
    `--max-tx <N>`, `--max-secs <N>`.
"#;

/// TRACE_MATRIX FC2-N16: `task open` dispatch entry
pub(crate) fn run(args: &[String]) -> ExitCode {
    if args.iter().any(|a| a == "--help" || a == "-h") {
        print!("{FULL_HELP}");
        return ExitCode::SUCCESS;
    }
    let mut prepended: Vec<String> = Vec::with_capacity(args.len() + 1);
    prepended.push("run-task".to_string());
    prepended.extend_from_slice(args);
    run_external(TASK_RUNNER_BIN, &prepended)
}
