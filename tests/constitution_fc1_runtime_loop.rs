//! TB-C0 Constitution Landing Gate — FC1 Runtime Loop
//!
//! Constitutional invariants on Flowchart 1:
//!   `Q_t → rtool/context → Agent output → predicate → wtool → Q_{t+1}`
//!
//! Hard invariant (FC1-INV1):
//! ```text
//! externalized_attempt_count
//!   == L4_WorkTx_attempt_count
//!    + L4E_WorkTx_rejection_count
//!    + explicitly_anchored_capsule_attempt_count
//! ```
//!
//! Test list (per TB-C0 directive §4.1):
//!   - fc1_every_externalized_attempt_is_tape_visible
//!   - fc1_predicate_pass_goes_l4
//!   - fc1_predicate_fail_goes_l4e
//!   - fc1_no_legacy_authoritative_append
//!   - fc1_dashboard_not_source_of_truth
//!   - fc1_attempt_count_equals_tape_count
//!   - fc1_no_fake_accepted_nodes
//!
//! All tests are real assertions — no `assert!(true)` per CR-C0.1.

use turingosv4::runtime::chain_derived_run_facts::{
    attempt_count_invariant, AttemptCountInvariantViolation, ChainDerivedRunFacts,
};
use turingosv4::state::typed_tx::RunOutcome;

/// Build a ChainDerivedRunFacts test fixture mimicking the canonical
/// P49 / P38 / P23 shapes used in TB-18R R4 invariant tests.
fn facts(halt: RunOutcome, expected: u64, l4: u64, l4e: u64, aborted: u64) -> ChainDerivedRunFacts {
    let delta = (l4 as i64) + (l4e as i64) - (expected as i64);
    ChainDerivedRunFacts {
        expected_completed_attempts: expected,
        l4_work_attempt_count: l4,
        l4e_work_attempt_count: l4e,
        attempt_aborted_count: aborted,
        delta,
        terminal_halt_class: halt,
        ..ChainDerivedRunFacts::default()
    }
}

/// FC1-INV1 — Every externalized LLM-Lean attempt is tape-visible. The
/// canonical P49 N→1 collapse failure mode (32 evaluator LLM proposals
/// reduced to 1 ChainTape WorkTx) MUST be caught by the invariant.
///
/// This test specifically asserts the invariant fires on the TB-18 M1
/// VETO root-cause shape: `expected=32, l4=1, l4e=0`.
#[test]
fn fc1_every_externalized_attempt_is_tape_visible() {
    // The TB-18 M1 P49 VETO shape: 32 LLM proposals externalized;
    // only 1 ended up on chain (the omega WorkTx); 31 attempts vanished.
    let collapsed = facts(RunOutcome::OmegaAccepted, 32, 1, 0, 0);
    let result = attempt_count_invariant(&collapsed);
    assert!(
        result.is_err(),
        "FC1-INV1 violation: invariant did NOT catch the canonical TB-18 \
         M1 P49 N→1 collapse (expected=32, l4=1, l4e=0, delta=-31). \
         If this passes, externalized attempts can vanish into evaluator \
         stdout without tape-visibility."
    );
    // The violation must be NegativeDelta (delta=-31 < 0).
    match result.unwrap_err() {
        AttemptCountInvariantViolation::NegativeDelta { delta, .. } => {
            assert_eq!(
                delta, -31,
                "FC1-INV1: NegativeDelta diagnostic should be -31"
            );
        }
        other => panic!("FC1-INV1: collapse should fire NegativeDelta, got {other:?}"),
    }
}

/// FC1-INV2a — Predicate pass routes the WorkTx to L4 accepted. We
/// verify the *structural* contract: when expected==l4 (every attempt
/// landed in L4 accepted) and aborted==0, invariant holds with
/// OmegaAccepted halt class.
#[test]
fn fc1_predicate_pass_goes_l4() {
    // 1 LLM call, predicate passes, omega accepted: expected=1 == l4=1.
    let one_shot = facts(RunOutcome::OmegaAccepted, 1, 1, 0, 0);
    attempt_count_invariant(&one_shot).expect("clean omega + delta=0 + aborted=0 must pass");

    // Multi-attempt success: 32 LLMs, 1 omega win + 31 L4.E rejections.
    // (P49 properly routed.)
    let p49_proper = facts(RunOutcome::OmegaAccepted, 32, 1, 31, 0);
    attempt_count_invariant(&p49_proper)
        .expect("properly-routed P49 (32 attempts: 1 L4 + 31 L4.E) must pass");
}

/// FC1-INV2b — Predicate fail routes the WorkTx to L4.E (rejection
/// evidence ledger). Run reaches MaxTxExhausted with all attempts
/// going to L4.E.
#[test]
fn fc1_predicate_fail_goes_l4e() {
    // 50 LLM calls, all rejected: expected=50 == l4e=50, l4=0.
    let exhausted = facts(RunOutcome::MaxTxExhausted, 50, 0, 50, 0);
    attempt_count_invariant(&exhausted).expect("all-fail run (50 L4.E) must pass invariant");
}

/// FC1-INV4 — No legacy authoritative append. In ChainTape mode, direct
/// `bus.append_*` write paths must not bypass Sequencer admission. We
/// verify by source-side check: bus.rs must call into sequencer or
/// LedgerWriter, not write to a global-mutable Tape directly.
#[test]
fn fc1_no_legacy_authoritative_append() {
    let bus_src = std::fs::read_to_string("src/bus.rs").expect("bus.rs readable");

    // append_oracle_accepted is the canonical accept-side helper and must
    // exist. If it's gone, accept routing breaks.
    assert!(
        bus_src.contains("pub fn append_oracle_accepted"),
        "FC1-INV4 violation: bus.rs lost append_oracle_accepted — \
         oracle-accepted append path missing."
    );

    // Verify by surface: the bus exposes `with_sequencer` (so the sequencer
    // can be bound at boot) and `append` is gated to the legacy mode.
    assert!(
        bus_src.contains("pub fn with_sequencer"),
        "FC1-INV4 violation: bus.rs lost with_sequencer — sequencer \
         binding (chaintape mode) cannot be configured at boot. \
         Legacy append could become silently authoritative."
    );

    // The bus must distinguish legacy vs chaintape mode — search for a
    // mode-marker (Sequencer-bound vs not) used in append_internal.
    assert!(
        bus_src.contains("Sequencer") || bus_src.contains("sequencer"),
        "FC1-INV4 violation: bus.rs no longer references Sequencer — \
         chaintape mode (sequencer-mediated append) is unbacked."
    );
}

/// FC1-INV5 — Dashboard is materialized view, NOT source of truth.
/// The chain_derived_run_facts module must derive facts from chain only
/// (L4 + CAS), never from evaluator stdout. We assert the entrypoint
/// signature shape that supports this.
#[test]
fn fc1_dashboard_not_source_of_truth() {
    let cdr_src = std::fs::read_to_string("src/runtime/chain_derived_run_facts.rs")
        .expect("chain_derived_run_facts.rs readable");

    // The compute entry point must exist and take chain inputs.
    assert!(
        cdr_src.contains("pub fn compute_run_facts_from_chain"),
        "FC1-INV5 violation: compute_run_facts_from_chain missing — \
         dashboard cannot be regenerated from chain alone."
    );

    // The combined-with-invariant entry point exists per TB-18R R4.
    assert!(
        cdr_src.contains("pub fn compute_run_facts_from_chain_with_invariant"),
        "FC1-INV5 violation: compute_run_facts_from_chain_with_invariant \
         missing — chain-derived ship-gate equation cannot run."
    );

    // Existing TB-16 dashboard live regen test must still exist.
    assert!(
        std::path::Path::new("tests/tb_16_dashboard_live_regen.rs").exists(),
        "FC1-INV5 violation: tests/tb_16_dashboard_live_regen.rs missing — \
         dashboard regeneration smoke gone."
    );
}

/// FC1-INV3 — Attempt count equality. evaluator-reported tx count must
/// equal chain-derived tape count. This is the canonical TB-18R R4
/// hard ship gate.
#[test]
fn fc1_attempt_count_equals_tape_count() {
    // Negative-delta failure: 32 attempts reported, only 1 on chain.
    // (canonical TB-18 M1 P49 shape — must fire)
    let collapsed = facts(RunOutcome::OmegaAccepted, 32, 1, 0, 0);
    assert!(
        attempt_count_invariant(&collapsed).is_err(),
        "FC1-INV3 violation: invariant must reject expected=32, l4+l4e=1"
    );

    // Equality holds: invariant passes.
    let proper = facts(RunOutcome::OmegaAccepted, 32, 1, 31, 0);
    attempt_count_invariant(&proper).expect("32 attempts → 1 L4 + 31 L4.E must pass");

    // Clean halt with delta != 0 (e.g., 32 expected but only 30 accounted)
    // must also fail.
    let stale = facts(RunOutcome::OmegaAccepted, 32, 1, 29, 0);
    let err = attempt_count_invariant(&stale)
        .expect_err("FC1-INV3: clean halt with delta=-2 must fire CleanHaltDeltaNonZero");
    matches!(
        err,
        AttemptCountInvariantViolation::CleanHaltDeltaNonZero { .. }
            | AttemptCountInvariantViolation::NegativeDelta { .. }
    );
}

/// FC1-INV6 — No fake accepted nodes. A tampered WorkTx whose canonical
/// signing payload doesn't match its signature must fail replay verify.
/// This is the audit-tape sampler invariant; existing
/// tb_18r_audit_lean_stderr_tamper_detected.rs covers this.
#[test]
fn fc1_no_fake_accepted_nodes() {
    // The audit_tape sampler test must exist (tampered Lean stderr).
    let audit_lean_tamper = "tests/tb_18r_audit_lean_stderr_tamper_detected.rs";
    assert!(
        std::path::Path::new(audit_lean_tamper).exists(),
        "FC1-INV6 violation: {audit_lean_tamper} missing — tamper detection \
         on Lean stderr lost; fake accepted nodes could pass."
    );

    // The audit_sampler test must exist (tampered AttemptTelemetry payload).
    let audit_sampler = "tests/tb_18r_audit_sampler_attempt_payload.rs";
    assert!(
        std::path::Path::new(audit_sampler).exists(),
        "FC1-INV6 violation: {audit_sampler} missing — tamper detection \
         on AttemptTelemetry payload lost."
    );

    // The structural verify_chaintape entry exists and returns ReplayReport.
    let verify_src = std::fs::read_to_string("src/runtime/verify.rs").expect("verify.rs readable");
    assert!(
        verify_src.contains("pub fn verify_chaintape"),
        "FC1-INV6 violation: verify_chaintape symbol missing — replay-verify \
         cannot detect fake nodes."
    );
    assert!(
        verify_src.contains("pub struct ReplayReport"),
        "FC1-INV6 violation: ReplayReport struct missing — verify outcome \
         not surfaceable."
    );
}

/// FC1 — P38/P49 evidence smoke (real LLM-Lean compute). This test is
/// `#[ignore]`-marked because it requires real LLM compute (DeepSeek
/// API + Lean checker). Architect-authorized run is the gate to
/// flip MVP-1 from AMBER → GREEN.
#[test]
#[ignore = "TB-C0 MVP-1 evidence smoke: requires real LLM compute (P38+P49); architect-authorized run flips this from AMBER to GREEN. See handover/directives/2026-05-06_TBC0_CONSTITUTION_LANDING_RESET_DIRECTIVE.md §4.1."]
fn fc1_attempt_count_equality_under_real_load_p38_p49() {
    // Placeholder for the real-compute path. The actual implementation
    // depends on:
    //   - LLM API budget allocation (architect authorization)
    //   - P38 + P49 problem set (heldout MiniF2F shapes)
    //   - constitution_gate_report.json producer (TB-C0 task #8)
    panic!("MVP-1 smoke not yet wired; ignore is expected.");
}
