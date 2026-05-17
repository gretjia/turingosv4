//! TRACE_MATRIX FC2-N16: turingos audit tamper handler (audit_tape_tamper wrapper)
//!
//! Phase 6.1 W1b.10 atom. Thin shell-out wrapper around `audit_tape_tamper`.
//! All args passed 1:1; no subcommand prepend; `--help` short-circuited to
//! print FULL_HELP inline before delegating.
//!
//! §8 packet 2026-05-17 (TISR Phase 6.0/6.1 separate charter).

use std::process::ExitCode;

use crate::common::run_external;

/// TRACE_MATRIX FC2-N16: short help shown in `turingos --help` listing
pub(crate) const SHORT_HELP: &str = "Probe ChainTape tamper-resistance (audit_tape_tamper wrapper)";

/// TRACE_MATRIX FC2-N16: full help printed by `turingos audit tamper --help`
pub(crate) const FULL_HELP: &str = r#"turingos audit tamper — Probe ChainTape tamper-resistance

USAGE:
    turingos audit tamper [OPTIONS]

DESCRIPTION:
    Thin shell-out wrapper around `audit_tape_tamper`. All arguments are
    passed through 1:1 to `audit_tape_tamper`. Read-only audit: forks the
    input tape into temp copies, introduces one corruption per copy, then
    re-runs `audit_tape` over each. Emits `tamper_report.json`.

    Three tamper classes probed:
      1. Flip 1 byte in a random L4 row  → verdict must be BLOCK
      2. Flip 1 byte in a random CAS object → verdict must be BLOCK
      3. Remove a random L4 row (ref truncation) → verdict must be BLOCK

    Exit 0  — all 3 corruptions detected (BLOCK on each tampered copy)
    Exit 1  — at least 1 corruption not detected (HALT per architect §7.7)
    Exit 2  — invalid args / I/O failure

    Wraps: audit_tape_tamper (in same target dir as turingos itself).

EXAMPLE:
    turingos audit tamper \
      --runtime-repo  ./run/runtime_repo \
      --cas-dir       ./run/cas \
      --agent-pubkeys ./run/agent_pubkeys.json \
      --pinned-pubkeys ./system_pubkeys.json \
      --genesis       ./run/genesis_report.json \
      --constitution  ./constitution.md \
      --tamper-dir    /tmp/tamper_work \
      --out           ./tamper_report.json
"#;

/// TRACE_MATRIX FC2-N16: entry point for `turingos audit tamper`
///
/// Checks for `--help` / `-h` first and prints FULL_HELP locally; otherwise
/// delegates to `audit_tape_tamper` with all args forwarded 1:1.
pub(crate) fn run(args: &[String]) -> ExitCode {
    // Short-circuit --help / -h before invoking the wrapped binary so the
    // user sees the wrapper's canonical help string (which includes the
    // FC2-N16 trace reference) rather than raw binary output.
    if args.iter().any(|a| a == "--help" || a == "-h") {
        print!("{FULL_HELP}");
        return ExitCode::SUCCESS;
    }
    // No subcommand prepend: audit_tape_tamper is a standalone binary.
    run_external("audit_tape_tamper", args)
}
