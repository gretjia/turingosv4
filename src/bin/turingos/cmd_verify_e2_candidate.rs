//! TRACE_MATRIX FC2-N16: turingos verify e2-candidate handler (real14_e2_candidate_verifier wrapper)
//!
//! Phase 6.1 W1b.7 atom. Read-only shell-out to `real14_e2_candidate_verifier`.
//! 0 sequencer call; 0 typed_tx; 0 CAS write; 0 ChainTape advance.
//!
//! FC-trace: FC2-N16 (boot / genesis / tape replay view).
//! REAL-14: E2 candidate verification over finished run evidence.

use std::process::ExitCode;

use crate::common::run_external;

// ─────────────────────────────────────────────────────────────────────
// Public surface — pub(crate) only; never escapes the turingos binary.
// ─────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC2-N16: short help string for dispatch table
pub(crate) const SHORT_HELP: &str = "Run the REAL-14 E2 candidate verifier on a finished run";

/// TRACE_MATRIX FC2-N16: full help text printed on --help
pub(crate) const FULL_HELP: &str = r#"turingos verify e2-candidate — REAL-14 E2 candidate verifier

USAGE:
    turingos verify e2-candidate [OPTIONS]

DESCRIPTION:
    Thin shell-out wrapper around the `real14_e2_candidate_verifier` binary.
    All arguments are passed through 1:1 to `real14_e2_candidate_verifier`.
    Run `turingos verify e2-candidate --help` OR
    `real14_e2_candidate_verifier --help` for the canonical option list.

    This is a Class 1 read-only verification tool: no sequencer call is made
    and no ChainTape state is advanced. The verifier reads ChainTape and CAS
    evidence from a finished run directory.

    Wraps: real14_e2_candidate_verifier (in same target dir as turingos itself).

OPTIONS (forwarded to real14_e2_candidate_verifier):
    --repo <path>        Path to the run evidence repo (required)
    --cas <path>         Path to the CAS directory
    --expect-count <n>   Expected E2 candidate count (optional assertion)
    --json-out <path>    Write JSON verdict to file
    --md-out <path>      Write Markdown verdict to file
    --help, -h           Print this help and exit (handled before shell-out)

EXAMPLES:
    turingos verify e2-candidate --repo ./handover/evidence/run001 --cas ./handover/evidence/run001/cas

SEE ALSO:
    real14_e2_candidate_verifier --help   Upstream command help
    turingos report run --help            Show run summary
"#;

// ─────────────────────────────────────────────────────────────────────
// Entry point
// ─────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC2-N16: handler for `turingos verify e2-candidate`
///
/// Short-circuits on `--help` / `-h` before shelling out. All other
/// arguments are forwarded 1:1 to `real14_e2_candidate_verifier` (no
/// prepend).
pub(crate) fn run(args: &[String]) -> ExitCode {
    // --help / -h short-circuit: print full help, exit 0.
    if args.iter().any(|a| a == "--help" || a == "-h") {
        print!("{FULL_HELP}");
        return ExitCode::SUCCESS;
    }

    // Direct pass-through: no arg prepend (unlike lean_market wrappers).
    run_external("real14_e2_candidate_verifier", args)
}
