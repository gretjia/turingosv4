//! TRACE_MATRIX FC2-N16: turingos task open handler (lean_market run-task wrapper)
//!
//! Phase 6.1 W1c.13 atom. Thin shell-out wrapper around `lean_market run-task`.
//! Prepends the `run-task` subcommand name to all user-supplied args before
//! delegating to `lean_market`. All other args pass through 1:1.
//!
//! Class 2 risk: `lean_market run-task` bootstraps a fresh chaintape and writes
//! TaskOpen + EscrowLock transitions — it is write-capable. This wrapper itself
//! adds NO new sequencer admission; it is a pure CLI routing shim.
//!
//! §8 packet 2026-05-17 (TISR Phase 6.0/6.1 separate charter).

use std::process::ExitCode;

use crate::common::run_external;

/// TRACE_MATRIX FC2-N16: short help shown in `turingos --help` listing
pub(crate) const SHORT_HELP: &str =
    "Open a Lean proof task on the chaintape (lean_market run-task)";

/// TRACE_MATRIX FC2-N16: full help printed by `turingos task open --help`
pub(crate) const FULL_HELP: &str = r#"turingos task open — Open a Lean proof task on the chaintape

USAGE:
    turingos task open [OPTIONS]

DESCRIPTION:
    Thin shell-out wrapper around `lean_market run-task`. Prepends the
    `run-task` subcommand name and forwards all remaining args to lean_market.

    Class 2 write-capable: lean_market run-task bootstraps a fresh chaintape,
    signs and posts TaskOpen + EscrowLock transitions using the Agent_user_0
    TB-9 durable keystore, then forks the evaluator child process to run the
    Lean proof-checking loop on the specified problem. This wrapper only
    routes; it never calls the sequencer or CAS directly.

    FC-trace: FC2-N16 (bootstrap / genesis gate — TaskOpen + EscrowLock are
    the canonical on_init-style tape anchors for a new proof task).

    Run `lean_market run-task --help` for the full canonical option list.

    Wraps: lean_market run-task ...
"#;

/// TRACE_MATRIX FC2-N16: entry point for `turingos task open`
///
/// Short-circuits `--help` / `-h` to print FULL_HELP locally (preserving
/// the FC2-N16 trace reference in the help text). All other invocations
/// prepend `run-task` to the args slice and delegate to `lean_market`.
pub(crate) fn run(args: &[String]) -> ExitCode {
    // Short-circuit --help / -h before invoking the wrapped binary so the
    // user sees the wrapper's FC2-N16-annotated help string.
    if args.iter().any(|a| a == "--help" || a == "-h") {
        print!("{FULL_HELP}");
        return ExitCode::SUCCESS;
    }

    // Prepend `run-task` so that `lean_market run-task <user-args...>` is
    // the actual invocation regardless of how the user called us.
    let mut prepended: Vec<String> = Vec::with_capacity(args.len() + 1);
    prepended.push("run-task".to_string());
    prepended.extend_from_slice(args);

    run_external("lean_market", &prepended)
}
