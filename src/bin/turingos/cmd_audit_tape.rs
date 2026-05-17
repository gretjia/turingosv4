//! TRACE_MATRIX FC2-N16: turingos audit tape handler (audit_tape wrapper)
//!
//! Phase 6.1 W1b.9 atom. Thin shell-out wrapper around `audit_tape`.
//! All args passed 1:1; no subcommand prepend; `--help` short-circuited to
//! print FULL_HELP inline before delegating.
//!
//! §8 packet 2026-05-17 (TISR Phase 6.0/6.1 separate charter).

use std::process::ExitCode;

use crate::common::run_external;

/// TRACE_MATRIX FC2-N16: short help shown in `turingos --help` listing
pub(crate) const SHORT_HELP: &str = "Audit ChainTape transitions for replay consistency";

/// TRACE_MATRIX FC2-N16: full help printed by `turingos audit tape --help`
pub(crate) const FULL_HELP: &str = r#"turingos audit tape — Audit ChainTape transitions for replay consistency

USAGE:
    turingos audit tape [OPTIONS]

DESCRIPTION:
    Thin shell-out wrapper around `audit_tape`. Args passed 1:1.
    Read-only audit: inspects the ChainTape and CAS evidence for
    replay consistency without mutating any state.
    Run `audit_tape --help` for canonical options.

    Wraps: audit_tape ...
"#;

/// TRACE_MATRIX FC2-N16: entry point for `turingos audit tape`
///
/// Checks for `--help` / `-h` first and prints FULL_HELP locally; otherwise
/// delegates to `audit_tape` with all args forwarded 1:1.
pub(crate) fn run(args: &[String]) -> ExitCode {
    // Short-circuit --help / -h before invoking the wrapped binary so the
    // user sees the wrapper's canonical help string (which includes the
    // FC2-N16 trace reference) rather than raw binary output.
    if args.iter().any(|a| a == "--help" || a == "-h") {
        print!("{FULL_HELP}");
        return ExitCode::SUCCESS;
    }
    // No subcommand prepend: audit_tape is a standalone binary.
    run_external("audit_tape", args)
}
