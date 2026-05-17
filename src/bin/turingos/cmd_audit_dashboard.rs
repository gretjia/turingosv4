//! TRACE_MATRIX FC2-N16: turingos audit dashboard handler (audit_dashboard wrapper)
//!
//! Phase 6.1 W1b.8 atom. Thin shell-out wrapper around the `audit_dashboard`
//! binary. Reads ChainTape + CAS evidence and regenerates the audit dashboard
//! as a materialized view — never an authority (FC3-N31).

use std::process::ExitCode;

use crate::common::run_external;

/// TRACE_MATRIX FC2-N16: `audit dashboard` short-help
pub(crate) const SHORT_HELP: &str = "Regenerate audit dashboard from ChainTape + CAS evidence";

/// TRACE_MATRIX FC2-N16: `audit dashboard` full --help text
pub(crate) const FULL_HELP: &str = r#"turingos audit dashboard — Regenerate audit dashboard from ChainTape + CAS evidence

USAGE:
    turingos audit dashboard [OPTIONS]

DESCRIPTION:
    Thin shell-out wrapper around the `audit_dashboard` binary. All arguments
    are passed through 1:1 to `audit_dashboard`. Run
    `turingos audit dashboard --help` OR `audit_dashboard --help` for the
    canonical option list.

    The dashboard is a materialized view derived from ChainTape + CAS evidence.
    It is never an authority (FC3-N31): the dashboard can be deleted and
    regenerated at any time from the canonical evidence artifacts.

    Reads:
      --repo <runtime_repo>   path to the chain-backed runtime repository
      --cas  <cas>            path to the CAS directory
      --json                  emit machine-readable JSON instead of text
      --out  <path>           write output to a file instead of stdout

    No sequencer call. No CAS write. Read-only over existing evidence.

    Wraps: audit_dashboard (in same target dir as turingos itself).
"#;

/// TRACE_MATRIX FC2-N16: `audit dashboard` dispatch entry
pub(crate) fn run(args: &[String]) -> ExitCode {
    // Local --help short-circuit so users see our description with FC3-N31
    // materialized-view framing. Actual audit_dashboard --help also works.
    if args.iter().any(|a| a == "-h" || a == "--help") && args.len() == 1 {
        println!("{}", FULL_HELP);
        return ExitCode::SUCCESS;
    }
    run_external("audit_dashboard", args)
}
