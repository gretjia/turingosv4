//! TRACE_MATRIX FC2-N16: turingos preflight handler (resume_preflight wrapper)
//!
//! Phase 6.1 W1c.11 atom. Thin shell-out wrapper around `resume_preflight`.
//! All args passed 1:1; no subcommand prepend; `--help` short-circuited to
//! print FULL_HELP inline only when --help is the sole arg.
//!
//! §8 packet 2026-05-17 (TISR Phase 6.0/6.1 separate charter).

use std::process::ExitCode;

use crate::common::run_external;

/// TRACE_MATRIX FC2-N16: short help shown in `turingos --help` listing
pub(crate) const SHORT_HELP: &str =
    "Run runner preflight gates (clean tree, binary mtime, evidence immutability)";

/// TRACE_MATRIX FC2-N16: full help printed by `turingos preflight --help`
pub(crate) const FULL_HELP: &str = r#"turingos preflight — Run runner preflight gates

USAGE:
    turingos preflight [OPTIONS]

DESCRIPTION:
    Thin shell-out wrapper around `resume_preflight`. All arguments are
    passed through 1:1 to `resume_preflight`.

    Runs the 7-stage pre-runner gate check before any script that mutates
    handover/evidence/ or evaluates real problems:

      1. Clean / understood worktree
      2. Fresh binary vs current source / HEAD (mtime check)
      3. Evidence immutability (existing evidence not overwritten)
      4. Risk class classification
      5. FC trace declared (FC1/FC2/FC3 nodes)
      6. Charter / directive completeness
      7. Audit-round state

    Exit 0  — all preflight gates pass; safe to proceed
    Exit 1  — one or more gates failed; do not run the target script
    Exit 2  — invalid args / I/O failure

    Wraps: resume_preflight (in same target dir as turingos itself).

EXAMPLE:
    turingos preflight \
      --runtime-repo  ./run/runtime_repo \
      --evidence-dir  handover/evidence \
      --risk-class    3 \
      --fc-trace      FC1,FC2
"#;

/// TRACE_MATRIX FC2-N16: entry point for `turingos preflight`
///
/// Checks for `--help` as the sole arg and prints FULL_HELP locally; otherwise
/// delegates to `resume_preflight` with all args forwarded 1:1.
pub(crate) fn run(args: &[String]) -> ExitCode {
    // Short-circuit --help only when it is the sole argument, so that flags
    // like `--help --some-other-flag` are forwarded to resume_preflight.
    if args.len() == 1 && args[0] == "--help" {
        print!("{FULL_HELP}");
        return ExitCode::SUCCESS;
    }
    // No subcommand prepend: resume_preflight is a standalone binary.
    run_external("resume_preflight", args)
}
