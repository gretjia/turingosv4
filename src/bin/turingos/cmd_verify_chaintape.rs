//! TRACE_MATRIX FC2-N16: turingos verify chaintape handler (verify_chaintape wrapper)

use std::process::ExitCode;

use crate::common::run_external;

/// TRACE_MATRIX FC2-N16: `verify chaintape` short-help
pub(crate) const SHORT_HELP: &str = "Verify ChainTape integrity + replay consistency";

/// TRACE_MATRIX FC2-N16: `verify chaintape` full --help text
pub(crate) const FULL_HELP: &str = r#"turingos verify chaintape — Verify ChainTape integrity + replay consistency

USAGE:
    turingos verify chaintape --repo <runtime_repo_path> --cas <cas_path> \
        [--run-id <id>] [--out <path>]

DESCRIPTION:
    Thin shell-out wrapper around the `verify_chaintape` binary. All arguments
    are passed through 1:1 to `verify_chaintape`. Run
    `turingos verify chaintape --help` OR `verify_chaintape --help` for the
    canonical option list.

    Read-only over an existing evidence directory (runtime_repo + CAS). No
    sequencer call. No CAS write.

    Replays the L4 chain through `replay_full_transition` (I-DETHASH witness),
    verifies every entry's `system_signature` against the persisted
    `pinned_pubkeys.json`, and emits a `replay_report.json` to stdout or to
    `--out <path>` if provided.

EXIT CODES:
    0  — every architect-mandated boolean indicator passed.
    1  — at least one indicator failed (replay_report.json still emitted).
    2  — verifier could not start (manifest missing / CAS unreadable / I/O).

    Wraps: verify_chaintape (in same target dir as turingos itself).
"#;

/// TRACE_MATRIX FC2-N16: `verify chaintape` dispatch entry
pub(crate) fn run(args: &[String]) -> ExitCode {
    // Local --help short-circuit (when only --help is given) so users get our
    // description; they can re-invoke with their own flags.
    // Actual verify_chaintape --help also works.
    if args.iter().any(|a| a == "-h" || a == "--help") && args.len() == 1 {
        println!("{}", FULL_HELP);
        return ExitCode::SUCCESS;
    }
    run_external("verify_chaintape", args)
}
