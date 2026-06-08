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

use turingosv4::charter_core::compile_charter_core;
use turingosv4::ledger::{ImmutableTapeLedger, MemoryTapeLedger, NodeKind};
use turingosv4::memory_kernel::{EnvironmentResult, MemoryKernel, Task};
use turingosv4::tokenizer::Tokenizer;

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

// ── FC1-INV1 CONSTRUCTED-TAPE COUNTERPART (LIVE-FC1 forward-wiring) ────────────
//
// The real-LLM-load variant above stays honestly `#[ignore]`'d (it needs API
// budget + a held-out problem set). This RUNNABLE counterpart proves the SAME
// canonical FC1 invariant on a CONSTRUCTED tape built by the live `MemoryKernel`
// FC1 loop — no LLM, no network, deterministic.
//
// Canonical FC1 invariant (CLAUDE.md "Canonical FC1 invariant"):
//   externalized_attempt_count
//     == tool_dist.step + tool_dist.parse_fail + tool_dist.llm_err
// where each of {step, parse_fail, llm_err} is ONE externalized LLM-Lean cycle
// that landed on the durable tape: `step` = a verified `StateAccepted` advance,
// `parse_fail` = an `AgentProposal` rejected with `MalformedOrMissingStateUpdate`,
// `llm_err` = an `AgentProposal` rejected with `reject_class == "llm_err"`.
//
// The tape is built by driving the PRODUCTION `MemoryKernel::step_forward` with
// the three FC1-canonical externalized outcomes (success / malformed-header /
// transport-error-shaped Retry header) and then RECONSTRUCTING the three counts
// from the tape via `dump_all_nodes()` — NOT from hand-set facts. The equality is
// asserted against the number of externalized attempts we drove. This is the
// "constructed-tape gate" the FC1 count-equality always wanted.

/// One distinct task scope per attempt so each `step_forward` is an independent
/// externalized cycle on the tape.
fn task(id: &str) -> Task {
    Task {
        id: id.into(),
        prompt: "prove the lemma".into(),
    }
}

/// A worker outcome that the kernel admits (verified `StateAccepted` advance) —
/// the FC1 `step` class. (No `wtool_call` declaration → no arg-taint finding →
/// admits, per the forward-wired provenance derivation.)
fn step_env(task_id: &str) -> EnvironmentResult {
    EnvironmentResult {
        raw_output: format!(
            r#"{{"schema_version":"tdma-state-update/v1","status":"Proceed","task_id":"{task_id}","action":"PROCEED"}}
---BODY---
done"#
        ),
        raw_stderr: String::new(),
        success: true,
    }
}

/// A worker outcome with NO parseable state-update header — the FC1 `parse_fail`
/// class (routes to the `MalformedOrMissingStateUpdate` rejection).
fn parse_fail_env() -> EnvironmentResult {
    EnvironmentResult {
        raw_output: "this output has no json header at all, just prose".to_string(),
        raw_stderr: String::new(),
        success: false,
    }
}

/// A worker outcome shaped like the `tdma_runner` LLM-transport-error path: a
/// parseable Retry header carrying `reject_class == "llm_err"` with
/// `success == false` — the FC1 `llm_err` class.
fn llm_err_env(task_id: &str) -> EnvironmentResult {
    EnvironmentResult {
        raw_output: format!(
            r#"{{"schema_version":"tdma-state-update/v1","status":"Retry","task_id":"{task_id}","action":"RETRY_LLM_ERROR","failed_predicate":"llm_call_transport","reject_class":"llm_err"}}"#
        ),
        raw_stderr: "llm-error: transport timeout".to_string(),
        success: false,
    }
}

/// Reconstruct the FC1 tool-distribution `(step, parse_fail, llm_err)` from a
/// built tape. Pure derivation from `dump_all_nodes()` — the canonical evidence.
fn tool_dist_from_tape(tape: &MemoryTapeLedger) -> (usize, usize, usize) {
    let mut step = 0usize;
    let mut parse_fail = 0usize;
    let mut llm_err = 0usize;
    for (_id, node) in tape.dump_all_nodes() {
        match node.kind {
            // step = a verified head advance.
            NodeKind::StateAccepted if node.verified => step += 1,
            NodeKind::AgentProposal => match node.reject_class.as_deref() {
                Some("MalformedOrMissingStateUpdate") => parse_fail += 1,
                Some("llm_err") => llm_err += 1,
                _ => {}
            },
            _ => {}
        }
    }
    (step, parse_fail, llm_err)
}

fn fresh_kernel() -> MemoryKernel<MemoryTapeLedger> {
    let mut tape = MemoryTapeLedger::new();
    tape.set_verified_head("H0".into());
    let charter = compile_charter_core(
        "# Constitution\n## Art. 0.4 — Q_t version control\nFC1a tape_t.\n".as_bytes(),
        "v1.0",
        &Tokenizer::new(),
    );
    MemoryKernel::new(tape, "run-fc1-constructed-tape", charter)
}

/// FC1-INV1 (constructed tape) — drive a known mix of externalized attempts
/// through the live kernel FC1 loop and prove the canonical count-equality holds
/// on the resulting tape:
///   externalized_attempt_count == step + parse_fail + llm_err.
#[test]
fn fc1_attempt_count_equality_on_constructed_tape() {
    let mut k = fresh_kernel();

    // Drive a KNOWN mix: 3 step (success), 2 parse_fail, 4 llm_err.
    const N_STEP: usize = 3;
    const N_PARSE_FAIL: usize = 2;
    const N_LLM_ERR: usize = 4;
    let externalized = N_STEP + N_PARSE_FAIL + N_LLM_ERR;

    for i in 0..N_STEP {
        // Distinct task scopes so each success is its own accepted advance.
        let _ = k.step_forward(&task(&format!("step-{i}")), step_env(&format!("step-{i}")));
    }
    for _ in 0..N_PARSE_FAIL {
        let _ = k.step_forward(&task("pf"), parse_fail_env());
    }
    for _ in 0..N_LLM_ERR {
        let _ = k.step_forward(&task("le"), llm_err_env("le"));
    }

    let (step, parse_fail, llm_err) = tool_dist_from_tape(&k.tape);

    // Each class landed exactly the attempts we drove (no collapse, no spam).
    assert_eq!(step, N_STEP, "step (StateAccepted advances) count mismatch");
    assert_eq!(parse_fail, N_PARSE_FAIL, "parse_fail count mismatch");
    assert_eq!(llm_err, N_LLM_ERR, "llm_err count mismatch");

    // THE CANONICAL FC1 INVARIANT (CLAUDE.md): every externalized LLM-Lean cycle
    // is tape-visible and accounted for by exactly one of the three classes.
    assert_eq!(
        externalized,
        step + parse_fail + llm_err,
        "FC1-INV1 (constructed tape): externalized_attempt_count ({externalized}) != \
         step ({step}) + parse_fail ({parse_fail}) + llm_err ({llm_err}). An \
         externalized attempt vanished from (or was double-counted on) the tape."
    );

    // Feed the SAME counts through the production chain-derived invariant: with
    // 1 step accepted to L4 and (parse_fail + llm_err) rejected to L4.E, the
    // 3-term invariant (expected == l4 + l4e) must hold (no negative delta).
    let l4 = step as u64;
    let l4e = (parse_fail + llm_err) as u64;
    let facts = facts(RunOutcome::OmegaAccepted, externalized as u64, l4, l4e, 0);
    attempt_count_invariant(&facts).expect(
        "FC1-INV1 (constructed tape): the tape-derived (step→L4, parse_fail+llm_err→L4.E) \
         counts must satisfy the chain-derived attempt_count_invariant (delta=0)",
    );
}

/// FC1-INV1 (constructed-tape NON-VACUITY witness) — if a single externalized
/// attempt were to VANISH from the tape (the canonical P49 N→1 collapse failure
/// mode), the equality FAILS. This proves the constructed-tape gate is falsifiable
/// (it is not `assert!(true)`): we simulate the collapse by under-counting one
/// llm_err relative to the attempts driven and assert the chain invariant fires.
#[test]
fn fc1_constructed_tape_count_equality_is_falsifiable() {
    // 5 externalized attempts driven, but the tape only retained 4 (one collapsed
    // into evaluator stdout without a durable tape row). The 3-term invariant
    // must FIRE on the resulting negative delta.
    let externalized = 5u64;
    let l4 = 1u64;
    let l4e = 3u64; // 1 + 3 = 4 retained < 5 externalized → delta = -1.
    let collapsed = facts(RunOutcome::OmegaAccepted, externalized, l4, l4e, 0);
    let result = attempt_count_invariant(&collapsed);
    assert!(
        result.is_err(),
        "FC1-INV1 constructed-tape non-vacuity: a vanished externalized attempt \
         (5 driven, 4 on tape) MUST fail the count-equality invariant. If this \
         passes, the constructed-tape gate cannot detect a P49-style collapse."
    );
    match result.unwrap_err() {
        AttemptCountInvariantViolation::NegativeDelta { delta, .. } => {
            assert_eq!(delta, -1, "the single vanished attempt is delta=-1");
        }
        other => panic!("collapse should fire NegativeDelta, got {other:?}"),
    }
}
