//! TB-18 Atom A — Per-LLM-call budget end-to-end regression tests.
//!
//! Architect ruling 2026-05-05 §4 verbatim:
//!
//! > FR-18.2: Per-LLM-call budget is enforced and emits
//! > RunOutcome::DegradedLLM, not silent timeout.
//! > FR-18.3: DegradedLLM emits EvidenceCapsule and TerminalSummary.
//!
//! These tests validate the budget tracker's behavior end-to-end on
//! synthetic input streams that mirror the OBS_M0_DEEPSEEK_DRIFT §3 P02
//! drift signature (30+ consecutive 14-output-token responses).
//!
//! End-to-end EvidenceCapsule emission test (DegradedLLM halt → capsule
//! written with outcome=DegradedLLM) is covered indirectly: Atom E's
//! propagation pipeline writes `terminal_exhaustion_reason.to_run_outcome()`
//! which projects DegradedLLM → DegradedLLM (verified by the projection
//! contract test in tb_18_evidence_capsule_outcome_propagation.rs); the
//! evaluator binary's atom-A wiring sets `terminal_exhaustion_reason =
//! ExhaustionReason::DegradedLLM` on `BudgetVerdict::HaltDegradedLLM`
//! (verified by the static integration check below).

use std::fs;
use std::path::PathBuf;

use minif2f_v4::per_call_budget::{BudgetVerdict, LLMCallBudgetTracker, PerCallBudget};

// `CARGO_MANIFEST_DIR` for this test crate is `experiments/minif2f_v4`,
// so the evaluator source path is relative to that.
const EVALUATOR_SRC: &str = "src/bin/evaluator.rs";

/// SG-18.4 (FR-18.2 + OBS_M0 §3 reference scenario) — synthetic
/// 30-consecutive-trivial-response stream MUST halt with DegradedLLM
/// well before the 30th response. With default cap=10, halt fires at the
/// 10th trivial response.
#[test]
fn tb_18_a_drift_signature_halts_at_default_cap() {
    let mut t = LLMCallBudgetTracker::new(PerCallBudget::default());

    // Simulate the OBS_M0 P02 drift: 30 consecutive 14-token responses
    // (the actual P02 hang had 30+ in 5 minutes).
    let mut halted_at_iteration: Option<u32> = None;
    for i in 1..=30 {
        match t.on_response(14) {
            BudgetVerdict::HaltDegradedLLM {
                consecutive_trivial,
            } => {
                halted_at_iteration = Some(i);
                assert_eq!(
                    consecutive_trivial, 10,
                    "default cap=10; halt fires at exactly the 10th"
                );
                break;
            }
            BudgetVerdict::Continue => {
                // Still continuing; expected for first 9 iterations.
            }
            other => panic!("unexpected verdict at iter {i}: {other:?}"),
        }
    }
    assert_eq!(
        halted_at_iteration,
        Some(10),
        "halt MUST fire at 10th iteration (default cap), not later"
    );
}

/// SG-18.4 — substantive response intermittently RESETS the consecutive
/// counter; sparse drift (mixed substantive + trivial) does NOT halt.
/// This guards against false-positive halts on noisy LLM responses where
/// some calls are short answers but the run is still progressing.
#[test]
fn tb_18_a_intermittent_trivial_does_not_halt() {
    let mut t = LLMCallBudgetTracker::new(PerCallBudget::default());

    // Pattern: 5 trivial → 1 substantive → 5 trivial → 1 substantive ...
    // Counter resets each cycle; never reaches cap=10.
    for cycle in 0..10 {
        for _ in 0..5 {
            assert_eq!(
                t.on_response(14),
                BudgetVerdict::Continue,
                "cycle {cycle} trivial → continue"
            );
        }
        assert_eq!(
            t.on_response(500),
            BudgetVerdict::Continue,
            "cycle {cycle} substantive → continue, counter resets"
        );
        assert_eq!(t.consecutive_trivial(), 0);
    }
    // Total: 50 trivial calls + 10 substantive; no halt.
    assert_eq!(t.total_calls(), 60);
    assert_eq!(t.trivial_calls(), 50);
}

/// SG-18.4 — substantive responses NEVER halt regardless of count.
/// Guards against false positives on healthy runs.
#[test]
fn tb_18_a_substantive_only_never_halts() {
    let mut t = LLMCallBudgetTracker::new(PerCallBudget::default());
    for _ in 0..1000 {
        assert_eq!(t.on_response(500), BudgetVerdict::Continue);
    }
    assert_eq!(t.consecutive_trivial(), 0);
    assert_eq!(t.trivial_calls(), 0);
    assert_eq!(t.total_calls(), 1000);
}

/// FR-18.2 + architect §2.5 (DegradedLLM cannot be evidence-skip backdoor):
/// the evaluator binary's atom-A wiring MUST set
/// `terminal_exhaustion_reason = ExhaustionReason::DegradedLLM` on
/// `BudgetVerdict::HaltDegradedLLM` so the canonical Atom E propagation
/// pipeline emits an EvidenceCapsule + TerminalSummary with
/// `outcome = DegradedLLM` (not silently skipped).
///
/// This is a static check on the wiring: greps the evaluator source for
/// the literal mutation `terminal_exhaustion_reason = ExhaustionReason::DegradedLLM`.
/// If atom A's HaltDegradedLLM branch is removed or the assignment
/// regresses to no-op, this test fails and ship gates SG-18.2 + SG-18.4
/// block.
#[test]
fn tb_18_a_swarm_loop_sets_terminal_exhaustion_on_degraded_llm() {
    let path = workspace_relative(EVALUATOR_SRC);
    let src = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));

    let pattern = "terminal_exhaustion_reason = ExhaustionReason::DegradedLLM";
    let count = src.matches(pattern).count();
    assert!(
        count >= 1,
        "TB-18 Atom A swarm loop MUST set \
         `terminal_exhaustion_reason = ExhaustionReason::DegradedLLM` \
         on BudgetVerdict::HaltDegradedLLM in {}; found {} occurrences. \
         Without this assignment, DegradedLLM halt becomes evidence-skip \
         backdoor (architect §2.5 violation; FR-18.3 broken).",
        path.display(),
        count
    );
}

/// FR-18.2 — same check for WallClockCap halt path. The aggregate
/// per-run wall-clock cap is the internal enforcement that converts the
/// external `timeout 600` from primary cap to safety net (architect §1
/// Q5 + OBS_M0 §5.1 last bullet).
#[test]
fn tb_18_a_swarm_loop_sets_terminal_exhaustion_on_wall_clock_cap() {
    let path = workspace_relative(EVALUATOR_SRC);
    let src = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));

    let pattern = "terminal_exhaustion_reason = ExhaustionReason::WallClockCap";
    let count = src.matches(pattern).count();
    assert!(
        count >= 1,
        "TB-18 Atom A swarm loop MUST set \
         `terminal_exhaustion_reason = ExhaustionReason::WallClockCap` \
         on BudgetVerdict::HaltWallClockCap in {}; found {} occurrences. \
         Without this assignment, the aggregate per-run cap doesn't \
         convert from external-timeout to internal-halt (FR-18.2 broken).",
        path.display(),
        count
    );
}

/// FR-18.2 verbatim ("Per-LLM-call budget is enforced") — the swarm loop
/// MUST construct an LLMCallBudgetTracker before iterating. Static check
/// on the wiring: tracker construction site must exist.
#[test]
fn tb_18_a_swarm_loop_constructs_budget_tracker() {
    let path = workspace_relative(EVALUATOR_SRC);
    let src = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));

    assert!(
        src.contains("LLMCallBudgetTracker::new"),
        "TB-18 Atom A: evaluator.rs MUST construct \
         `LLMCallBudgetTracker::new(...)` before the swarm tx loop. \
         Not found in {}.",
        path.display()
    );
    assert!(
        src.contains("PerCallBudget::from_env"),
        "TB-18 Atom A: evaluator.rs MUST use `PerCallBudget::from_env()` \
         (FAIL-CLOSED on malformed env vars per \
         feedback_no_workarounds_strict_constitution). Not found in {}.",
        path.display()
    );
    assert!(
        src.contains("llm_budget_tracker.on_response"),
        "TB-18 Atom A: evaluator.rs MUST call \
         `llm_budget_tracker.on_response(...)` after every successful \
         client.generate() — without this call, budget enforcement is \
         a no-op. Not found in {}.",
        path.display()
    );
}

/// FR-18.2 — env-var configurability. Operators must be able to override
/// budget per architect §B.9.3 M-ladder configurability needs (atom H
/// will use this to tighten budget for M0 preflight + M1 batch).
#[test]
fn tb_18_a_per_call_budget_env_override_works() {
    // Note: env var manipulation is process-global; we use unique var
    // names per test to avoid cross-contamination (or use a Mutex per
    // feedback_env_var_test_lock; the per_call_budget reads each var
    // independently so there's no cross-test ordering issue here as long
    // as we set+unset within one test).
    //
    // To avoid interfering with parallel tests reading the same env, we
    // verify the parser logic via direct construction instead of env
    // manipulation. The from_env path is exercised by the unit tests in
    // per_call_budget.rs's tests module.
    let custom = PerCallBudget {
        per_call_wallclock_seconds: 30,
        token_floor_threshold: 50,
        consecutive_trivial_response_cap: 5,
        aggregate_per_run_wallclock_seconds: 300,
    };
    let mut t = LLMCallBudgetTracker::new(custom);
    // 4 trivials (49 < 50): all continue.
    for _ in 0..4 {
        assert_eq!(t.on_response(49), BudgetVerdict::Continue);
    }
    // 5th trivial: halt.
    match t.on_response(49) {
        BudgetVerdict::HaltDegradedLLM {
            consecutive_trivial,
        } => {
            assert_eq!(consecutive_trivial, 5);
        }
        other => panic!("expected HaltDegradedLLM at custom cap=5, got {other:?}"),
    }
}

fn workspace_relative(rel: &str) -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(manifest_dir).join(rel)
}
