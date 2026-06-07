//! LIVE CONSTITUTION GATE (M07) — single-admission anti-duplication witness.
//!
//! STATUS: LIVE / GREEN. Added under the user's §8 token
//! `APPROVE-M07-A4-SINGLE-ADMISSION-PREDICATE-GATE` (2026-06-07) alongside the
//! route-A single-admission predicate gate.
//!
//! ── WHAT THIS GATE ENFORCES (the single-admission invariant) ──────────────
//! M07 route A extracts ONE shared predicate-admission contract
//! (`src/predicate_admission.rs::decide_admission`) that BOTH admission
//! authorities call:
//!   * the sequencer `WorkTx` leg (`src/state/sequencer.rs` zero-root branch),
//!   * the memory-kernel header leg (`src/memory_kernel.rs` Proceed branch),
//!     which gates its `set_verified_head` advance on the verdict.
//!
//! The single-admission invariant is structural: the verdict-trusting zero-root
//! boolean scan must have EXACTLY ONE home (`decide_admission`), with both legs
//! as call sites — never a second inline copy of the boolean logic. A second
//! copy would let one authority drift from the other (the dual-admission split
//! G2 demonstrates). This gate is a SOURCE-STRUCTURAL witness (same family as
//! `tests/constitution_predicate_gate.rs:104-144`): it greps the canonical
//! source files, so it cannot be satisfied by a vacuous `assert!(true)` and
//! fails RED if the shared contract is removed, bypassed, or re-duplicated.
//!
//! ── TRIPLE-COUPLING ──────────────────────────────────────────────────────
//! Registered in `scripts/constitution_gates.manifest.toml`
//! (`constitution_single_admission_contract`) and referenced in
//! `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md`. Discovered by the flat
//! `ls tests/constitution_*.rs` glob in `scripts/run_constitution_gates.sh`.

use std::fs;

fn predicate_admission_src() -> String {
    fs::read_to_string("src/predicate_admission.rs").expect("src/predicate_admission.rs readable")
}
fn sequencer_src() -> String {
    fs::read_to_string("src/state/sequencer.rs").expect("src/state/sequencer.rs readable")
}
fn memory_kernel_src() -> String {
    fs::read_to_string("src/memory_kernel.rs").expect("src/memory_kernel.rs readable")
}

/// The shared admission contract is defined in exactly one place:
/// `src/predicate_admission.rs` exposes `pub fn decide_admission`.
#[test]
fn shared_admission_contract_is_defined_in_predicate_admission_module() {
    let src = predicate_admission_src();
    assert!(
        src.contains("pub fn decide_admission"),
        "M07 single-admission violation: src/predicate_admission.rs no longer \
         defines `pub fn decide_admission`. The shared predicate-admission \
         contract is the single home for the verdict-trusting zero-root logic; \
         removing it dissolves the single-admission invariant."
    );
}

/// Both admission authorities CALL the shared contract. Neither re-implements
/// the zero-root verdict logic inline.
#[test]
fn both_authorities_call_the_shared_admission_contract() {
    let seq = sequencer_src();
    assert!(
        seq.contains("predicate_admission::decide_admission("),
        "M07 single-admission violation: src/state/sequencer.rs does not call \
         `predicate_admission::decide_admission(` in its zero-root branch. The \
         sequencer leg must route its zero-root admission through the shared \
         contract, not a re-implemented inline boolean scan."
    );

    let kernel = memory_kernel_src();
    // The kernel gates its head advance on the shared contract, either directly
    // (`decide_admission(`) or via the arg-taint-aware wrapper
    // (`decide_admission_with_taint(`), which itself calls `decide_admission`
    // (defined ONLY in src/predicate_admission.rs — asserted below). Both are the
    // ONE shared contract; the kernel must call one of them, not an inline scan.
    assert!(
        kernel.contains("decide_admission(") || kernel.contains("decide_admission_with_taint("),
        "M07 single-admission violation: src/memory_kernel.rs does not call \
         `decide_admission(` (or its arg-taint wrapper `decide_admission_with_taint(`) \
         in its Proceed branch. The kernel leg must gate its head advance on the \
         shared predicate-admission contract."
    );
}

/// The kernel consults the shared contract BEFORE advancing the verified head.
/// The `decide_admission(` call must appear earlier in the file than the
/// `self.tape.set_verified_head(` advance, so the head advance is gated on the
/// verdict (not committed first and verified after).
#[test]
fn kernel_decides_admission_before_advancing_head() {
    let kernel = memory_kernel_src();
    // Match the shared-contract call site (direct `decide_admission(` or the
    // arg-taint wrapper `decide_admission_with_taint(`, which calls it). Both
    // gate the advance on the shared verdict.
    let decide_at = kernel
        .find("decide_admission_with_taint(")
        .or_else(|| kernel.find("decide_admission("))
        .expect("kernel must call decide_admission( (or decide_admission_with_taint()");
    let advance_at = kernel
        .find("self.tape.set_verified_head(")
        .expect("kernel must advance head via self.tape.set_verified_head(");
    assert!(
        decide_at < advance_at,
        "M07 single-admission violation: in src/memory_kernel.rs the \
         `decide_admission(` call (offset {decide_at}) must precede the \
         `self.tape.set_verified_head(` advance (offset {advance_at}). The head \
         advance must be GATED on the admission verdict, not performed before it."
    );
}

/// Anti-duplication: the verdict-trusting zero-root boolean loop body lives in
/// EXACTLY ONE file. We use the moved contract's canonical loop signature
/// (`for claim in &claims.acceptance` with `if !claim.value`) as the structural
/// fingerprint. It must appear in `src/predicate_admission.rs` and NOT in
/// `src/state/sequencer.rs` or `src/memory_kernel.rs` (which only call the
/// contract). A second copy anywhere is the duplication M07 forbids.
#[test]
fn zero_root_verdict_loop_has_exactly_one_home() {
    const LOOP_FINGERPRINT: &str = "for claim in &claims.acceptance";
    const VALUE_GUARD: &str = "if !claim.value";

    let pa = predicate_admission_src();
    assert!(
        pa.contains(LOOP_FINGERPRINT) && pa.contains(VALUE_GUARD),
        "M07 single-admission violation: src/predicate_admission.rs no longer \
         contains the verdict-trusting zero-root boolean scan \
         (`{LOOP_FINGERPRINT}` + `{VALUE_GUARD}`). The shared contract must be \
         the one home for this logic."
    );

    let pa_loop_count = pa.matches(LOOP_FINGERPRINT).count();
    assert_eq!(
        pa_loop_count, 1,
        "M07 single-admission violation: the verdict-trusting zero-root loop \
         `{LOOP_FINGERPRINT}` appears {pa_loop_count} times in \
         src/predicate_admission.rs; it must appear exactly once (one contract \
         body, not a copy)."
    );

    for (path, src) in [
        ("src/state/sequencer.rs", sequencer_src()),
        ("src/memory_kernel.rs", memory_kernel_src()),
    ] {
        assert!(
            !src.contains(LOOP_FINGERPRINT),
            "M07 single-admission violation: {path} re-implements the \
             verdict-trusting zero-root boolean scan `{LOOP_FINGERPRINT}` \
             inline. That logic must live ONLY in \
             src/predicate_admission.rs::decide_admission; {path} must CALL the \
             shared contract, not duplicate it. A second copy is the \
             dual-admission split the single-admission invariant forbids."
        );
    }

    // The shared contract must also NOT be re-defined in either authority: only
    // src/predicate_admission.rs owns `pub fn decide_admission`.
    for (path, src) in [
        ("src/state/sequencer.rs", sequencer_src()),
        ("src/memory_kernel.rs", memory_kernel_src()),
    ] {
        assert!(
            !src.contains("pub fn decide_admission"),
            "M07 single-admission violation: {path} defines its own \
             `pub fn decide_admission`. The shared contract has exactly one \
             definition home (src/predicate_admission.rs)."
        );
    }
}
