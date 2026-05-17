//! TRACE_MATRIX FC2-N16: turingos report run handler (gen_run_summary wrapper)

use std::process::ExitCode;

use crate::common::run_external;

/// TRACE_MATRIX FC2-N16: `report run` short-help
pub(crate) const SHORT_HELP: &str =
    "Generate a run summary report from a finished evidence directory";

/// TRACE_MATRIX FC2-N16: `report run` full --help text
pub(crate) const FULL_HELP: &str = r#"turingos report run — Generate a run-summary report

USAGE:
    turingos report run [OPTIONS]

DESCRIPTION:
    Thin shell-out wrapper around the `gen_run_summary` binary. All arguments
    are passed through 1:1 to `gen_run_summary`. Run `turingos report run --help`
    OR `gen_run_summary --help` for the canonical option list.

    No sequencer call. No CAS write. Read-only over an existing evidence
    directory.

    Wraps: gen_run_summary (in same target dir as turingos itself).
"#;

/// TRACE_MATRIX FC2-N16: `report run` dispatch entry
pub(crate) fn run(args: &[String]) -> ExitCode {
    // Local --help short-circuit (so users get our description; then they can
    // re-invoke with their own flags). Actual gen_run_summary --help also works.
    if args.iter().any(|a| a == "-h" || a == "--help") && args.len() == 1 {
        println!("{}", FULL_HELP);
        return ExitCode::SUCCESS;
    }
    run_external("gen_run_summary", args)
}
