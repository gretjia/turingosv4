//! TB-18R R6 helper — compute R4 chain-derived attempt-count invariant
//! from a chaintape run.
//!
//! Reads `runtime_repo` + `cas` directories produced by an evaluator run
//! and emits JSON with all 6 FR-18R.4 v2 fields + invariant verdict.
//!
//! Usage:
//!
//!   tb_18r_compute_invariant \
//!     --runtime-repo <path> \
//!     --cas <path> \
//!     --expected-completed <u64> \
//!     --halt-class <OmegaAccepted|MaxTxExhausted|WallClockCap|ComputeCap|ErrorHalt|DegradedLLM>
//!
//! Output (stdout): JSON with fields:
//!   expected_completed_attempts / l4_work_attempt_count /
//!   l4e_work_attempt_count / attempt_aborted_count / delta /
//!   terminal_halt_class / invariant_verdict (Ok|Err)
//!
//! TRACE_MATRIX FC1-N43 (TB-18R R6 evidence helper).

use std::path::PathBuf;
use std::process::ExitCode;

use turingosv4::runtime::chain_derived_run_facts::{
    attempt_count_invariant, compute_run_facts_from_chain_with_invariant,
    AttemptCountInvariantInputs,
};
use turingosv4::state::typed_tx::RunOutcome;

fn parse_halt_class(s: &str) -> Result<RunOutcome, String> {
    match s {
        "OmegaAccepted" => Ok(RunOutcome::OmegaAccepted),
        "MaxTxExhausted" => Ok(RunOutcome::MaxTxExhausted),
        "WallClockCap" => Ok(RunOutcome::WallClockCap),
        "ComputeCap" | "ComputeCapViolated" => Ok(RunOutcome::ComputeCap),
        "ErrorHalt" => Ok(RunOutcome::ErrorHalt),
        "DegradedLLM" => Ok(RunOutcome::DegradedLLM),
        other => Err(format!("unknown halt class: {other}")),
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let mut runtime_repo: Option<PathBuf> = None;
    let mut cas_path: Option<PathBuf> = None;
    let mut expected: Option<u64> = None;
    let mut halt_class: Option<RunOutcome> = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--runtime-repo" => {
                runtime_repo = Some(PathBuf::from(&args[i + 1]));
                i += 2;
            }
            "--cas" => {
                cas_path = Some(PathBuf::from(&args[i + 1]));
                i += 2;
            }
            "--expected-completed" => {
                expected = Some(args[i + 1].parse().expect("u64"));
                i += 2;
            }
            "--halt-class" => {
                halt_class = Some(parse_halt_class(&args[i + 1]).unwrap_or_else(|e| {
                    eprintln!("ERROR: {e}");
                    std::process::exit(2);
                }));
                i += 2;
            }
            _ => {
                eprintln!("ERROR: unknown arg: {}", args[i]);
                return ExitCode::from(2);
            }
        }
    }

    let runtime_repo = runtime_repo.expect("--runtime-repo required");
    let cas_path = cas_path.expect("--cas required");
    let expected = expected.expect("--expected-completed required");
    let halt_class = halt_class.expect("--halt-class required");

    let inputs = AttemptCountInvariantInputs {
        expected_completed_attempts: expected,
        terminal_halt_class: halt_class,
    };
    let facts =
        compute_run_facts_from_chain_with_invariant(&runtime_repo, &cas_path, inputs)
            .unwrap_or_else(|e| {
                eprintln!("ERROR: compute facts failed: {e}");
                std::process::exit(3);
            });
    let verdict = match attempt_count_invariant(&facts) {
        Ok(()) => "Ok".to_string(),
        Err(v) => format!("Err({v})"),
    };

    let out = serde_json::json!({
        "expected_completed_attempts": facts.expected_completed_attempts,
        "l4_work_attempt_count": facts.l4_work_attempt_count,
        "l4e_work_attempt_count": facts.l4e_work_attempt_count,
        "capsule_anchored_attempt_count": facts.capsule_anchored_attempt_count,
        "attempt_aborted_count": facts.attempt_aborted_count,
        "delta": facts.delta,
        "terminal_halt_class": format!("{:?}", facts.terminal_halt_class),
        "invariant_verdict": verdict,
        // TB-C0 strict-audit Bug 3 fix (2026-05-07): equation extended to 3-term
        // constitutional formula per CLAUDE.md PRIME OPERATING MODE FC1 hard
        // invariant. Backward-compat: pre-Bug-3 evidence runs have
        // capsule_anchored == 0; equation reduces to original 2-term shape.
        "tb_18r_r4_invariant_equation": "evaluator_reported_completed_llm_calls == l4_work_attempt_count + l4e_work_attempt_count + capsule_anchored_attempt_count",
        "preflight": "handover/ai-direct/TB-18R_R4_STEP_B_invariant.md",
        "tbc0_strict_audit_fix": "STRICT_AUDIT_TBC0_TAPE_2026-05-07.md Finding C — Bug 3 (capsule_anchored 3-term)",
    });
    println!("{}", serde_json::to_string_pretty(&out).unwrap());
    ExitCode::SUCCESS
}
