//! LIVE-FC1 Phase 5 — BUDGET HARD-CEILING admission gate (the Turing fuel = FC2-HALT).
//!
//! §8 token: `APPROVE-BUDGET-HARD-CEILING-FROM-MANIFEST`
//! (`handover/section8/APPROVE_BUDGET_HARD_CEILING_FROM_MANIFEST_2026-06-08.md`).
//!
//! A Turing-complete substrate cannot self-halt (the halting problem). An
//! EXTERNAL integer resource bound forces termination. This gate proves the
//! budget hard-ceiling is **ENFORCING, not advisory** — and that it composes
//! from the existing seams with ZERO new pinned discriminant:
//!
//!   1. **FORWARD-ONLY** — a zero ceiling (`MicroCoin::zero()`, today's default)
//!      is UNLIMITED: an expensive run with a huge tape-derived spend still
//!      ADVANCES the verified head, exactly as before. No budget reject is ever
//!      produced when the ceiling is unarmed.
//!
//!   2. **ON-EXCEED = REJECT, NO HEAD ADVANCE (FC2-HALT)** — with a POSITIVE
//!      ceiling, once cumulative tape-derived spend reaches it, the next proposal
//!      is REJECTED (routed to the non-advancing rejection path), the verified
//!      head does NOT move, and the rejection receipt carries the PINNED
//!      `RejectionClass::BudgetExceeded` label. Because the head is frozen and
//!      spend only grows, EVERY subsequent proposal also rejects — the run halts.
//!      (Mutation: flip the membrane `>=` to a no-op / drop the pre-check → the
//!      proposal ADVANCES and `step.is_proceed()` flips this assert RED.)
//!
//!   3. **CHECKPOINT-RESUME** — the tape is append-only and no head moved on the
//!      halt, so RAISING the ceiling (a new approved budget manifest) lets the
//!      PREVIOUSLY-HALTED proposal admit from the same head on the next tick.
//!      (Mutation: keep the low ceiling → the proposal stays rejected and the
//!      resume assert flips RED.)
//!
//!   4. **CEILING DERIVES FROM THE SIGNED MANIFEST (not a const)** — the ceiling
//!      is read from a budget-manifest TOML FILE; mutating the manifest's integer
//!      moves the halt boundary. Integer-only: a float ceiling is a parse error.
//!
//!   5. **REUSES THE PINNED DISCRIMINANT** — the reject label is provably the
//!      existing `RejectionClass::BudgetExceeded` (`typed_tx.rs:174`); NO new
//!      `RejectionClass` / `RunOutcome` / `HaltReason` variant is introduced.
//!
//!   6. **TAPE-DERIVED INTEGER SPEND (reuses VPPUT C_i)** — spend is the integer
//!      token sum over the tape (failed branches counted), the same `C_i`
//!      quantity the Phase-2 VPPUT reconstruction sums. No `f64`, no sidecar.
//!
//! Non-vacuity: every green assert has a paired mutation (named above) that flips
//! a dedicated assert RED, so the gate cannot be satisfied by a constant
//! (`feedback_single_site_gate_illusion`). ZERO genesis-pinned-file edits — the
//! mechanism lives in the UNPINNED `src/runtime/budget_ceiling.rs` (nested under
//! the UNPINNED `src/runtime/agent_scheduler.rs`) + the UNPINNED
//! `src/memory_kernel.rs` membrane; `genesis_payload.toml` / `q_state.rs` /
//! `typed_tx.rs` / `sequencer.rs` are NOT in the diff.

use turingosv4::charter_core::compile_charter_core;
use turingosv4::economy::money::MicroCoin;
use turingosv4::ledger::{ImmutableTapeLedger, MemoryTapeLedger, NodeKind};
use turingosv4::memory_kernel::{EnvironmentResult, KernelStep, MemoryKernel, Task};
use turingosv4::runtime::agent_scheduler::budget_ceiling::{
    budget_check, live_tape_spend_tokens, reject_class_label, BudgetManifest, BudgetVerdict,
};
use turingosv4::state::typed_tx::RejectionClass;
use turingosv4::tokenizer::Tokenizer;

// ─────────────────────────────────────────────────────────────────────────
// Harness — a real MemoryKernel over a MemoryTapeLedger (the live FC1 loop).
// ─────────────────────────────────────────────────────────────────────────

fn fresh_kernel(ceiling: MicroCoin) -> MemoryKernel<MemoryTapeLedger> {
    let mut tape = MemoryTapeLedger::new();
    tape.set_verified_head("H0".into());
    let charter = compile_charter_core(
        "# Constitution\n## Art. 0.4 — Q_t version control\nFC1a tape_t.\n".as_bytes(),
        "v1.0",
        &Tokenizer::new(),
    );
    MemoryKernel::new(tape, "run-budget", charter).with_cost_ceiling(ceiling)
}

fn ok_header(task: &str) -> String {
    format!(
        r#"{{"schema_version":"tdma-state-update/v1","status":"Proceed","task_id":"{task}","action":"PROCEED"}}
---BODY---
done"#
    )
}

fn ok_env(task: &str) -> EnvironmentResult {
    EnvironmentResult {
        raw_output: ok_header(task),
        raw_stderr: String::new(),
        success: true,
    }
}

fn is_proceed(step: &KernelStep) -> bool {
    matches!(step, KernelStep::Proceed { .. })
}

/// The reject_class string recorded on the most-recently-committed
/// AgentProposal (verified:false) node — the rejection receipt's class.
fn latest_reject_class(k: &MemoryKernel<MemoryTapeLedger>) -> Option<String> {
    k.tape
        .dump_all_nodes()
        .into_iter()
        .filter(|(_, n)| n.kind == NodeKind::AgentProposal && !n.verified)
        .max_by_key(|(_, n)| n.created_at_unix_ms)
        .and_then(|(_, n)| n.reject_class)
}

// ─────────────────────────────────────────────────────────────────────────
// GATE 1 — FORWARD-ONLY: a zero ceiling is UNLIMITED (no budget reject ever).
// ─────────────────────────────────────────────────────────────────────────
#[test]
fn zero_ceiling_is_unlimited_run_advances_as_before() {
    // Ceiling unarmed (the default for every legacy run).
    let mut k = fresh_kernel(MicroCoin::zero());
    let task = Task {
        id: "t".into(),
        prompt: "do".into(),
    };

    // Drive several COSTLY accepted steps (each records a large token_count, so
    // the tape-derived spend is genuinely huge) and confirm the head keeps
    // advancing — a zero ceiling never halts, no matter the spend.
    let mut last_head = k.tape.get_verified_head();
    for _ in 0..5 {
        let step = k.step_forward_with_budget(
            &task,
            ok_env("t"),
            Default::default(),
            &Default::default(),
            1_000_000,
        );
        assert!(
            is_proceed(&step),
            "FORWARD-ONLY: zero ceiling must admit (proceed) regardless of spend"
        );
        let new_head = k.tape.get_verified_head();
        assert_ne!(
            new_head, last_head,
            "head must advance under a zero ceiling"
        );
        last_head = new_head;
    }

    // The spend really is huge — proving the no-reject was NOT because spend was 0.
    let spend = live_tape_spend_tokens(&k.tape);
    assert!(
        spend >= 5_000_000,
        "spend must be large (>=5M) to prove forward-only is non-vacuous, got {spend}"
    );
    // And no AgentProposal rejection was ever committed.
    assert!(
        latest_reject_class(&k).is_none(),
        "FORWARD-ONLY: a zero ceiling must produce NO budget rejection"
    );
}

// ─────────────────────────────────────────────────────────────────────────
// GATE 2 — ON-EXCEED = REJECT, NO HEAD ADVANCE (the FC2-HALT). A positive
// ceiling halts the run once tape-derived spend reaches it.
// ─────────────────────────────────────────────────────────────────────────
#[test]
fn positive_ceiling_halts_run_with_no_head_advance() {
    // Arm a ceiling of 100 micro-units (= 100 tokens at 1 micro/token).
    let mut k = fresh_kernel(MicroCoin::from_micro_units(100));
    let task = Task {
        id: "t".into(),
        prompt: "do".into(),
    };

    // Step 1: 60 tokens. Prior spend = 0 < 100 → admits, head advances, records 60.
    let step1 = k.step_forward_with_budget(
        &task,
        ok_env("t"),
        Default::default(),
        &Default::default(),
        60,
    );
    assert!(is_proceed(&step1), "first step within budget must proceed");
    let head_after_1 = k.tape.get_verified_head();
    assert_eq!(
        live_tape_spend_tokens(&k.tape),
        60,
        "spend after step 1 = 60"
    );

    // Step 2: another 60 tokens. Prior spend = 60 < 100 → admits, records 60 more
    // → cumulative spend = 120.
    let step2 = k.step_forward_with_budget(
        &task,
        ok_env("t"),
        Default::default(),
        &Default::default(),
        60,
    );
    assert!(
        is_proceed(&step2),
        "second step (prior spend 60 < 100) must proceed"
    );
    let head_after_2 = k.tape.get_verified_head();
    assert_ne!(head_after_2, head_after_1, "head advanced on step 2");
    assert_eq!(
        live_tape_spend_tokens(&k.tape),
        120,
        "cumulative spend = 120 >= ceiling 100"
    );

    // Step 3: prior spend 120 >= ceiling 100 → THE FC2-HALT. The proposal is a
    // valid Proceed header, yet it is REJECTED purely on budget. The head MUST
    // NOT advance.
    let step3 = k.step_forward_with_budget(
        &task,
        ok_env("t"),
        Default::default(),
        &Default::default(),
        5,
    );
    assert!(
        !is_proceed(&step3),
        "FC2-HALT: a positive-ceiling breach must NOT proceed (mutation: drop the \
         pre-check → this proceeds → RED)"
    );
    let head_after_3 = k.tape.get_verified_head();
    assert_eq!(
        head_after_3, head_after_2,
        "FC2-HALT: the verified head must NOT advance on a budget reject"
    );

    // The rejection receipt carries the PINNED RejectionClass::BudgetExceeded label.
    let reject_class = latest_reject_class(&k).expect("a budget rejection receipt exists");
    assert_eq!(
        reject_class,
        reject_class_label(),
        "budget reject must be stamped with the BudgetExceeded label"
    );

    // STICKY HALT: a further proposal also rejects (the run is out of fuel).
    let step4 = k.step_forward_with_budget(
        &task,
        ok_env("t"),
        Default::default(),
        &Default::default(),
        1,
    );
    assert!(
        !is_proceed(&step4),
        "halt is sticky: every further proposal rejects"
    );
    assert_eq!(
        k.tape.get_verified_head(),
        head_after_2,
        "head stays frozen across the sticky halt"
    );
}

// ─────────────────────────────────────────────────────────────────────────
// GATE 3 — CHECKPOINT-RESUME: raising the ceiling lets the previously-halted
// proposal admit from the last accepted head (append-only tape, no head moved).
// ─────────────────────────────────────────────────────────────────────────
#[test]
fn raising_ceiling_resumes_the_halted_run() {
    // Arm a tight ceiling so the run halts after one accepted step.
    let mut k = fresh_kernel(MicroCoin::from_micro_units(50));
    let task = Task {
        id: "t".into(),
        prompt: "do".into(),
    };

    // Step 1: 50 tokens (prior spend 0 < 50) → admits, spend becomes 50.
    let step1 = k.step_forward_with_budget(
        &task,
        ok_env("t"),
        Default::default(),
        &Default::default(),
        50,
    );
    assert!(is_proceed(&step1));
    let checkpoint_head = k.tape.get_verified_head();
    assert_eq!(live_tape_spend_tokens(&k.tape), 50);

    // Step 2: prior spend 50 >= ceiling 50 → HALT, head frozen at the checkpoint.
    let halted = k.step_forward_with_budget(
        &task,
        ok_env("t"),
        Default::default(),
        &Default::default(),
        10,
    );
    assert!(!is_proceed(&halted), "run halts at the tight ceiling");
    assert_eq!(
        k.tape.get_verified_head(),
        checkpoint_head,
        "head frozen at the checkpoint while halted"
    );

    // RESUME: the architect approves a raised ceiling (a new budget manifest).
    // The tape is append-only and the head never moved, so the previously-halted
    // proposal now admits FROM THE SAME CHECKPOINT HEAD.
    k.cost_ceiling_microcoin = MicroCoin::from_micro_units(10_000);
    let resumed = k.step_forward_with_budget(
        &task,
        ok_env("t"),
        Default::default(),
        &Default::default(),
        10,
    );
    assert!(
        is_proceed(&resumed),
        "CHECKPOINT-RESUME: a raised ceiling must let the halted proposal admit \
         (mutation: keep the low ceiling → stays rejected → RED)"
    );
    let resumed_head = k.tape.get_verified_head();
    assert_ne!(
        resumed_head, checkpoint_head,
        "after resume the head advances PAST the checkpoint"
    );
}

// ─────────────────────────────────────────────────────────────────────────
// GATE 4 — the ceiling DERIVES FROM A SIGNED MANIFEST FILE (not a const), and
// mutating the manifest's integer MOVES the halt boundary. Integer-only.
// ─────────────────────────────────────────────────────────────────────────
#[test]
fn ceiling_derives_from_signed_manifest_file_and_moves_the_boundary() {
    let dir = tempfile::tempdir().expect("tempdir");
    let manifest_path = dir.path().join("budget_manifest.toml");

    // A signed/user-approved budget manifest FILE — a SEPARATE TOML, never
    // genesis_payload.toml. Write a ceiling of 100 micro-units.
    std::fs::write(&manifest_path, "cost_ceiling_micro_units = 100\n").expect("write manifest");
    let manifest = BudgetManifest::from_file(&manifest_path).expect("load signed manifest");
    assert_eq!(manifest.ceiling_micro(), MicroCoin::from_micro_units(100));

    // The kernel reads the ceiling from the manifest at run init (unpinned runner
    // path → kernel field; we do NOT edit q_state).
    let mut k = fresh_kernel(manifest.ceiling_micro());
    let task = Task {
        id: "t".into(),
        prompt: "do".into(),
    };
    // Spend 100 (== ceiling) over one step, then the next halts.
    let _ = k.step_forward_with_budget(
        &task,
        ok_env("t"),
        Default::default(),
        &Default::default(),
        100,
    );
    let halted = k.step_forward_with_budget(
        &task,
        ok_env("t"),
        Default::default(),
        &Default::default(),
        1,
    );
    assert!(
        !is_proceed(&halted),
        "spend 100 >= manifest ceiling 100 → halt"
    );

    // MUTATE the manifest file to a LARGER ceiling: the SAME spend now admits —
    // proving the boundary is the manifest's integer, not a hardcoded const.
    std::fs::write(&manifest_path, "cost_ceiling_micro_units = 100000\n")
        .expect("rewrite manifest");
    let raised = BudgetManifest::from_file(&manifest_path).expect("reload manifest");
    let mut k2 = fresh_kernel(raised.ceiling_micro());
    let _ = k2.step_forward_with_budget(
        &task,
        ok_env("t"),
        Default::default(),
        &Default::default(),
        100,
    );
    let admitted = k2.step_forward_with_budget(
        &task,
        ok_env("t"),
        Default::default(),
        &Default::default(),
        1,
    );
    assert!(
        is_proceed(&admitted),
        "mutating the manifest ceiling 100→100000 moves the boundary (mutation \
         witness: a hardcoded const would not move) "
    );

    // Integer-only: a float ceiling in the manifest is a parse error — no f64.
    let float_err = BudgetManifest::from_toml_str("cost_ceiling_micro_units = 1.5");
    assert!(
        float_err.is_err(),
        "a float ceiling must be rejected (no f64 on the money path)"
    );
}

// ─────────────────────────────────────────────────────────────────────────
// GATE 5 — REUSES THE PINNED DISCRIMINANT: the reject label is provably the
// existing RejectionClass::BudgetExceeded; NO new variant is introduced.
// ─────────────────────────────────────────────────────────────────────────
#[test]
fn reject_class_is_the_existing_pinned_budget_exceeded_variant() {
    // The label is derived FROM the pinned variant's Debug name — if the variant
    // were renamed/removed this line would fail to compile, so the label can
    // never silently drift from the pinned enum.
    assert_eq!(
        reject_class_label(),
        format!("{:?}", RejectionClass::BudgetExceeded)
    );
    assert_eq!(reject_class_label(), "BudgetExceeded");

    // And the live membrane stamps exactly this label (no second/new class).
    let mut k = fresh_kernel(MicroCoin::from_micro_units(10));
    let task = Task {
        id: "t".into(),
        prompt: "do".into(),
    };
    let _ = k.step_forward_with_budget(
        &task,
        ok_env("t"),
        Default::default(),
        &Default::default(),
        10,
    );
    let _ = k.step_forward_with_budget(
        &task,
        ok_env("t"),
        Default::default(),
        &Default::default(),
        1,
    );
    assert_eq!(
        latest_reject_class(&k).as_deref(),
        Some("BudgetExceeded"),
        "the membrane reuses the PINNED RejectionClass::BudgetExceeded — no new discriminant"
    );
}

// ─────────────────────────────────────────────────────────────────────────
// GATE 6 — DETERMINISTIC + TAPE-DERIVED INTEGER SPEND (reuses VPPUT C_i). Same
// tape + same ceiling ⇒ same verdict; failed branches count toward spend.
// ─────────────────────────────────────────────────────────────────────────
#[test]
fn spend_is_deterministic_tape_derived_and_counts_failed_branches() {
    // Two kernels driven with the SAME inputs must reach the SAME verdict.
    let drive = || {
        let mut k = fresh_kernel(MicroCoin::from_micro_units(100));
        let task = Task {
            id: "t".into(),
            prompt: "do".into(),
        };
        // One accepted step costing 60, then a FAILED step costing 50 (success:false
        // → rejection path, but its tokens still cost budget). Cumulative = 110.
        let _ = k.step_forward_with_budget(
            &task,
            ok_env("t"),
            Default::default(),
            &Default::default(),
            60,
        );
        let failed_env = EnvironmentResult {
            raw_output: r#"{"schema_version":"tdma-state-update/v1","status":"Retry","task_id":"t","action":"RETRY","failed_predicate":"x.y","reject_class":"schema-fail"}
---BODY---
nope"#
                .into(),
            raw_stderr: "boom\n".into(),
            success: false,
        };
        let _ = k.step_forward_with_budget(
            &task,
            failed_env,
            Default::default(),
            &Default::default(),
            50,
        );
        let spend = live_tape_spend_tokens(&k.tape);
        // The next proposal's verdict under the ceiling.
        let next_halts = !is_proceed(&k.step_forward_with_budget(
            &task,
            ok_env("t"),
            Default::default(),
            &Default::default(),
            1,
        ));
        (spend, next_halts)
    };
    let (spend_a, halt_a) = drive();
    let (spend_b, halt_b) = drive();
    assert_eq!(spend_a, spend_b, "DETERMINISTIC: same inputs ⇒ same spend");
    assert_eq!(halt_a, halt_b, "DETERMINISTIC: same inputs ⇒ same verdict");

    // FAILED BRANCHES COUNT: 60 (accepted) + 50 (failed) = 110, and 110 >= 100 ⇒
    // the next proposal halts. Dropping the failed branch (110→60) would NOT halt
    // — the failed branch genuinely pushed spend over the ceiling (mutation witness).
    assert_eq!(
        spend_a, 110,
        "spend counts the failed branch (60 accepted + 50 failed)"
    );
    assert!(
        halt_a,
        "spend 110 >= ceiling 100 ⇒ halt; the failed branch was decisive"
    );

    // Cross-check the pure decision function agrees with the live verdict.
    assert!(
        budget_check(spend_a, MicroCoin::from_micro_units(100)).is_exceeded(),
        "pure budget_check agrees: 110 >= 100 is Exceeded"
    );
    assert_eq!(
        budget_check(60, MicroCoin::from_micro_units(100)),
        BudgetVerdict::Within {
            spend_micro: 60,
            ceiling_micro: 100
        },
        "dropping the failed branch (spend 60) would be Within — the failed branch was decisive"
    );
}
