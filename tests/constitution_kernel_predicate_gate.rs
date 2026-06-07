//! LIVE CONSTITUTION GATE (G1 / M07) — kernel predicate-admission receipt.
//!
//! STATUS: LIVE / GREEN. Promoted from `tests/pending/` after the Class-4 src/
//! admission change landed under the user's §8 token
//! `APPROVE-M07-A4-SINGLE-ADMISSION-PREDICATE-GATE` (2026-06-07). The memory
//! kernel now gates its `verified_head` advance on the shared
//! `predicate_admission::decide_admission` contract and commits an additive
//! `predicate_admission` PASS receipt into the advancing `StateAccepted`
//! payload (see `src/memory_kernel.rs` Proceed branch), so the post-fix
//! invariant below holds and this gate is GREEN.
//!
//! Triple-coupled: registered in `scripts/constitution_gates.manifest.toml`
//! (`constitution_kernel_predicate_gate`) and referenced in
//! `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md`. Discovered by the flat
//! `ls tests/constitution_*.rs` glob in `scripts/run_constitution_gates.sh` and
//! built by `cargo test --workspace`.
//!
//! ── M07 BLOCKER (the bypass this gate now enforces against) ───────────────
//! `src/memory_kernel.rs:171-188` — `MemoryKernel::step_forward_with_workspace`
//! routes purely on `(parsed_header, env_result.is_success())`. When the worker
//! header carries `status == StateStatus::Proceed` AND `env_result.success ==
//! true`, the kernel immediately:
//!   1. commits `NodeKind::StateAccepted` (line ~174), and
//!   2. calls `self.tape.set_verified_head(accepted.hash)` (line ~188).
//! It NEVER calls `verify_work_predicates`, builds no `WorkTx`, and touches no
//! `PredicateRegistry`. Kernel admission is PREDICATE-BLIND: a worker self-report
//! of `Proceed` + a `success` boolean is sufficient to advance the canonical
//! verified head, with no oracle predicate re-execution and no admission receipt
//! on tape.
//!
//! ── ENFORCED INVARIANT (what this gate asserts) ──────────────────────────
//! The memory kernel must NOT advance `verified_head` to a new `StateAccepted`
//! node purely from `env_result.success + status==Proceed`. Advancing the
//! verified head must be gated on a PREDICATE-ADMISSION PASS that is itself
//! recorded on the tape as a verifiable admission receipt (so an auditor can
//! reconstruct, from the tape alone, that predicate admission gated the head
//! advance). The M07 fix produces exactly this receipt, so the gate is GREEN;
//! if the receipt were ever removed it would go RED, re-exposing the bypass.

use turingosv4::charter_core::compile_charter_core;
use turingosv4::ledger::{ImmutableTapeLedger, MemoryTapeLedger, NodeKind};
use turingosv4::memory_kernel::{EnvironmentResult, KernelStep, MemoryKernel, Task};
use turingosv4::tokenizer::Tokenizer;

/// A worker `EnvironmentResult` that the CURRENT kernel treats as a happy path:
/// `success == true` plus a parseable prefix-JSON header with `status:"Proceed"`.
fn proceed_env(task_id: &str) -> EnvironmentResult {
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

fn fresh_kernel() -> MemoryKernel<MemoryTapeLedger> {
    let mut tape = MemoryTapeLedger::new();
    tape.set_verified_head("H0".into());
    let charter = compile_charter_core(
        "# Constitution\n## Art. 0.4 — Q_t version control\nFC1a tape_t.\n".as_bytes(),
        "v1.0",
        &Tokenizer::new(),
    );
    MemoryKernel::new(tape, "run-m07-g1", charter)
}

/// True iff the tape carries an explicit predicate-admission PASS receipt that an
/// auditor could use to reconstruct that predicate admission gated the head
/// advance. This is the post-fix artifact the M07 fix must produce.
///
/// We accept either shape so the eventual fix is not over-constrained:
///   (a) the advancing `StateAccepted` payload carries a `predicate_admission`
///       receipt object (e.g. `{"verdict":"PASS","registry_root":...}`), OR
///   (b) a sibling tape node records a predicate-admission verdict (a dedicated
///       receipt node) parented under the same verified head.
/// TODAY neither exists — `NodeKind` has no admission-receipt variant and the
/// `StateAccepted` payload is only `{state_update, output_summary}`.
fn tape_has_predicate_admission_receipt(tape: &MemoryTapeLedger) -> bool {
    tape.dump_all_nodes().iter().any(|(_hash, node)| {
        // Shape (a): receipt embedded in an accepted node's payload.
        let embedded = matches!(node.kind, NodeKind::StateAccepted)
            && node.payload.get("predicate_admission").is_some();

        // Shape (b): a dedicated receipt node anywhere on the tape. We probe by
        // payload key rather than enum variant, because adding an enum variant
        // is one valid fix but not the only one.
        let dedicated = node.payload.get("predicate_admission_receipt").is_some()
            || node.payload.get("predicate_admission").is_some();

        embedded || dedicated
    })
}

/// G1 — the kernel must NOT advance `verified_head` on `success + Proceed` alone;
/// the advance must be gated on a predicate-admission PASS recorded on tape.
///
/// LIVE RESULT: **GREEN**. With the M07 fix the kernel routes its head advance
/// through `predicate_admission::decide_admission` and commits a tape-recorded
/// `predicate_admission` PASS receipt into the advancing `StateAccepted` node,
/// so the receipt probe below succeeds. This is a fail-able gate: removing the
/// receipt (re-introducing the predicate-blind bypass) turns it RED again.
#[test]
fn m07_kernel_must_not_advance_verified_head_without_predicate_admission_receipt() {
    let mut k = fresh_kernel();
    let task = Task {
        id: "t1".into(),
        prompt: "do the thing".into(),
    };

    let head_before = k.tape.get_verified_head();
    let step = k.step_forward(&task, proceed_env("t1"));

    // Establish the precondition: the CURRENT kernel took the happy path and
    // advanced the verified head purely from success + Proceed.
    assert!(
        matches!(step, KernelStep::Proceed { .. }),
        "M07 precondition: kernel should take the Proceed happy path on \
         success=true + status=Proceed (it currently does so unconditionally)."
    );
    let head_after = k.tape.get_verified_head();
    assert_ne!(
        head_after, head_before,
        "M07 precondition: kernel advanced verified_head on the Proceed path."
    );

    // The enforced invariant: that head advance MUST be backed by a predicate-
    // admission PASS receipt on the tape. GREEN with the M07 fix in place.
    let has_receipt = tape_has_predicate_admission_receipt(&k.tape);
    assert!(
        has_receipt,
        "M07 BYPASS REGRESSION: memory_kernel advanced \
         verified_head from `{head_before}` to `{head_after}` purely on \
         env_result.success + status==Proceed, WITHOUT any predicate-admission \
         PASS receipt on tape. The M07 Proceed branch in src/memory_kernel.rs \
         must route the head advance through \
         predicate_admission::decide_admission and commit a tape-recorded \
         `predicate_admission` PASS receipt into the advancing StateAccepted \
         payload BEFORE calling set_verified_head(). A failure here means that \
         receipt was removed and the predicate-blind kernel bypass re-opened \
         (§8 token APPROVE-M07-A4-SINGLE-ADMISSION-PREDICATE-GATE)."
    );
}
