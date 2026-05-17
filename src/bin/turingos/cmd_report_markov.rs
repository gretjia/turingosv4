//! TRACE_MATRIX FC2-N16: turingos report markov handler (generate_markov_capsule wrapper)
//!
//! Phase 6.1 W1a.5 atom. Thin shell-out wrapper around `generate_markov_capsule`.
//! All args passed 1:1; no subcommand prepend; `--help` short-circuited to
//! print FULL_HELP inline before delegating.
//!
//! §8 packet 2026-05-17 (TISR Phase 6.0/6.1 separate charter).

use std::process::ExitCode;

use crate::common::run_external;

/// TRACE_MATRIX FC2-N16: short help shown in `turingos --help` listing
pub(crate) const SHORT_HELP: &str = "Generate Markov evidence capsule from a finished run";

/// TRACE_MATRIX FC2-N16: full help printed by `turingos report markov --help`
pub(crate) const FULL_HELP: &str = r#"turingos report markov — Generate a MarkovEvidenceCapsule

USAGE:
    turingos report markov [OPTIONS]

DESCRIPTION:
    Thin shell-out wrapper around `generate_markov_capsule`. Args passed 1:1.
    Run `generate_markov_capsule --help` for canonical options.

    Wraps: generate_markov_capsule ...
"#;

/// TRACE_MATRIX FC2-N16: entry point for `turingos report markov`
///
/// Checks for `--help` / `-h` first and prints FULL_HELP locally; otherwise
/// delegates to `generate_markov_capsule` with all args forwarded 1:1.
pub(crate) fn run(args: &[String]) -> ExitCode {
    // Short-circuit --help / -h before invoking the wrapped binary so the
    // user sees the wrapper's canonical help string (which includes the
    // FC2-N16 trace reference) rather than raw binary output.
    if args.iter().any(|a| a == "--help" || a == "-h") {
        print!("{FULL_HELP}");
        return ExitCode::SUCCESS;
    }
    // No subcommand prepend: generate_markov_capsule is a standalone binary.
    run_external("generate_markov_capsule", args)
}
