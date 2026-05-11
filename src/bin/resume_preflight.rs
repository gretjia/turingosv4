//! TB-G G1.2-1 CLI shim — wraps `runtime::resume_preflight::check` so an
//! orchestrator script (e.g. `scripts/run_g_phase_batch.sh`) can invoke
//! the preflight without linking the library.
//!
//! Usage:
//!
//! ```text
//! resume_preflight --contract <path/to/contract.json>
//! ```
//!
//! Output: writes `{"verdict":"Ok"}` or
//! `{"verdict":"Fail","failure":{"kind":"...","..."}}` to stdout.
//! Exit code: 0 on `Ok`, 1 on `Fail`, 2 on argv/IO error.
//!
//! Constitutional Justification: `handover/directives/2026-05-11_TB_G_G1_2_OPTION_B_PLUS_RULING.md` §2 Q2 (resume contract must be explicit and fail-closed).

use std::process::ExitCode;

use turingosv4::runtime::resume_preflight::{check, PreflightVerdict, ResumeContract};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let contract_path = match parse_argv(&args) {
        Ok(p) => p,
        Err(msg) => {
            eprintln!("{msg}");
            return ExitCode::from(2);
        }
    };

    let bytes = match std::fs::read(&contract_path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!(
                "resume_preflight: failed to read contract at {contract_path:?}: {e}"
            );
            return ExitCode::from(2);
        }
    };

    let contract: ResumeContract = match serde_json::from_slice(&bytes) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("resume_preflight: failed to parse contract JSON: {e}");
            return ExitCode::from(2);
        }
    };

    let verdict = check(&contract);
    let json = match serde_json::to_string(&verdict) {
        Ok(j) => j,
        Err(e) => {
            eprintln!("resume_preflight: failed to serialize verdict: {e}");
            return ExitCode::from(2);
        }
    };
    println!("{json}");

    match verdict {
        PreflightVerdict::Ok => ExitCode::from(0),
        PreflightVerdict::Fail { .. } => ExitCode::from(1),
    }
}

fn parse_argv(args: &[String]) -> Result<String, String> {
    let mut iter = args.iter().skip(1);
    while let Some(a) = iter.next() {
        if a == "--contract" {
            return iter
                .next()
                .cloned()
                .ok_or_else(|| "resume_preflight: --contract expects a path".into());
        }
    }
    Err(
        "resume_preflight: missing --contract <path>; usage: resume_preflight --contract <path.json>"
            .into(),
    )
}
