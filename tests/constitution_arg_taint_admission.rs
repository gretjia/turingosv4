//! LIVE CONSTITUTION GATE — arg-taint sub-article: a tainted wtool argument that
//! flows into a privileged sink is REJECTED at admission (the confused-deputy
//! hard-gate), and a clean/Trusted-arg call is ADMITTED (positive control).
//!
//! STATUS: LIVE / GREEN. Added under the §8 token
//! `APPROVE-ARG-TAINT-ADMISSION-SUBARTICLE` — value-level taint + admission
//! rejection of tainted-arg-into-privileged-sink.
//!
//! ── WHAT THIS GATE PROVES (non-vacuous, both legs) ───────────────────────────
//!   (1) VALUE-LEVEL TAINT IS COMPUTED: `arg_taint_v1` over a wtool call whose
//!       argument value carries a tainted provenance label (`UntrustedExternal`
//!       / `ToolOutput` / `AgentGenerated`) and whose target is a privileged sink
//!       (economic-wallet / system-only tool, or a `wallet/` / `system/`
//!       write-set namespace) produces a tainted-arg → privileged-sink finding.
//!
//!   (2) THE HARD-GATE REJECTS AT ADMISSION: the live UNPINNED memory-kernel leg
//!       (`MemoryKernel::step_forward_with_taint`) drives a worker `Proceed`
//!       carrying that finding through the SHARED admission contract
//!       (`predicate_admission::decide_admission_with_taint`). The verdict is
//!       `Fail` → `handle_rejection` → the verified head is NOT advanced (no
//!       `Q_{t+1}`). The rejection is stamped with the `arg_taint_v1[...]`
//!       failed-predicate marker + the `ArgTaintIntoPrivilegedSink` reject_class,
//!       reconstructable from the rejection receipt alone.
//!
//!   (3) POSITIVE CONTROL — IT DOES NOT REJECT EVERYTHING: the SAME kernel leg,
//!       fed the SAME worker `Proceed` with a TRUSTED-arg call (empty findings),
//!       ADMITS and advances the verified head. Without this control the
//!       hard-gate could be satisfied by a predicate that rejects all Proceeds;
//!       the control proves it discriminates on argument taint.
//!
//! ── ZERO-PINNED-FILE DISCIPLINE ──────────────────────────────────────────────
//! The taint module is nested as a `#[path]` submodule of the UNPINNED
//! `src/predicate_admission.rs`; the hard-gate is a SEPARATE
//! `decide_admission_with_taint` wrapper (the pinned sequencer keeps calling the
//! unchanged `decide_admission`); the kernel seam is the UNPINNED
//! `src/memory_kernel.rs`; the taint reject reuses the existing
//! `AcceptancePredicateFalse` reason (no new `AdmissionFailReason` variant, so the
//! genesis-pinned `sequencer.rs` exhaustive match stays valid). No genesis-pinned
//! file is touched.
//!
//! ── TRIPLE-COUPLING ──────────────────────────────────────────────────────────
//! Registered in `scripts/constitution_gates.manifest.toml`
//! (`constitution_arg_taint_admission`) and referenced in
//! `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md`. Discovered by the flat
//! `ls tests/constitution_*.rs` glob in `scripts/run_constitution_gates.sh` and
//! built by `cargo test --workspace`.

use turingosv4::bottom_white::tools::registry::{
    Capability, DeterminismClass, PermissionPolicy, SideEffectClass, ToolMetadata,
};
use turingosv4::charter_core::compile_charter_core;
use turingosv4::ledger::{ImmutableTapeLedger, MemoryTapeLedger};
use turingosv4::memory_kernel::{EnvironmentResult, MemoryKernel, Task};
use turingosv4::predicate_admission::arg_taint::{
    arg_taint_v1, ArgTaint, LabeledArg, SinkReason, WtoolCall,
};
use turingosv4::predicate_admission::{
    decide_admission_with_taint, zero_root_hex, AdmissionFailReason, AdmissionVerdict,
    PredicateClaimSet, ARG_TAINT_FAILED_PREDICATE_PREFIX,
};
use turingosv4::tokenizer::Tokenizer;

// ── fixtures ─────────────────────────────────────────────────────────────────

/// A worker `EnvironmentResult` the kernel treats as a happy-path candidate
/// (`success == true` + a parseable `status:"Proceed"` prefix-JSON header). The
/// admission decision is taken AFTER this, on the supplied taint findings.
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

fn wallet_tool() -> ToolMetadata {
    ToolMetadata {
        tool_id: "wallet.transfer".into(),
        version: 1,
        capability: Capability::EconomicWallet,
        permission_policy: PermissionPolicy::Open,
        determinism_class: DeterminismClass::IdempotentWrite,
        side_effect_class: SideEffectClass::None,
        schema: "wallet/transfer/v1".into(),
        creator: "system".into(),
        code_hash: [0u8; 32],
        test_suite_hash: [0u8; 32],
        reuse_royalty_share_micro: 0,
    }
}

/// A wtool call where an UntrustedExternal argument value flows into a privileged
/// (economic-wallet) sink AND a `wallet/` write namespace — the confused-deputy
/// hazard the sub-article forbids.
fn tainted_privileged_call() -> WtoolCall {
    WtoolCall {
        args: vec![LabeledArg::new(
            "recipient",
            b"attacker-controlled-address-from-the-web".to_vec(),
            ArgTaint::UntrustedExternal,
        )],
        target_tools: vec![wallet_tool()],
        write_keys: vec!["wallet/agent-7/balance".to_string()],
    }
}

/// The SAME wtool call shape but with a TRUSTED argument — the clean positive
/// control. A trusted value into the same wallet sink is NOT a hazard.
fn trusted_privileged_call() -> WtoolCall {
    WtoolCall {
        args: vec![LabeledArg::new(
            "recipient",
            b"operator-configured-treasury-address".to_vec(),
            ArgTaint::Trusted,
        )],
        target_tools: vec![wallet_tool()],
        write_keys: vec!["wallet/agent-7/balance".to_string()],
    }
}

/// Drive the kernel `Proceed` path WITH the given taint findings and report
/// `(admitted, final_verified_head)`. "Admitted" == the kernel advanced its
/// verified head (committed a new accepted state). The advance is gated on the
/// shared `decide_admission_with_taint` contract: a non-empty findings set yields
/// a non-advancing rejection (`admitted == false`, head stays `H0`); empty
/// findings + a passing (empty) claim set advance (`admitted == true`).
fn kernel_admits_with_taint(call: &WtoolCall) -> (bool, String) {
    let findings = arg_taint_v1(call);

    let mut tape = MemoryTapeLedger::new();
    tape.set_verified_head("H0".into());
    let charter = compile_charter_core(
        "# Constitution\n## Art. 0.4 — Q_t version control\nFC1a tape_t.\n".as_bytes(),
        "v1.0",
        &Tokenizer::new(),
    );
    let mut k = MemoryKernel::new(tape, "run-arg-taint-admission", charter);
    let task = Task {
        id: "t-taint".into(),
        prompt: "transfer funds".into(),
    };
    let before = k.tape.get_verified_head();
    let _step = k.step_forward_with_taint(
        &task,
        proceed_env("t-taint"),
        PredicateClaimSet::default(),
        &Default::default(),
        &findings,
    );
    let after = k.tape.get_verified_head();
    (after != before, after)
}

// ── GATE (1): the analysis computes the tainted→privileged finding ────────────

/// arg_taint_v1 surfaces the tainted-arg → privileged-sink flow for the tainted
/// call, and the finding identifies BOTH the wallet capability sink and the
/// privileged write namespace.
#[test]
fn arg_taint_v1_flags_tainted_arg_into_privileged_sink() {
    let findings = arg_taint_v1(&tainted_privileged_call());
    // One tainted arg × two privileged sinks (wallet tool + wallet/ write key).
    assert_eq!(
        findings.len(),
        2,
        "arg_taint_v1 must flag the tainted arg against BOTH privileged sinks; \
         got {findings:?}"
    );
    assert!(
        findings
            .iter()
            .all(|f| f.arg_name == "recipient" && f.arg_taint == ArgTaint::UntrustedExternal),
        "every finding must name the tainted recipient arg: {findings:?}"
    );
    assert!(
        findings
            .iter()
            .any(|f| f.sink == "wallet.transfer"
                && f.sink_reason == SinkReason::PrivilegedCapability),
        "the economic-wallet tool must be a privileged-capability sink: {findings:?}"
    );
    assert!(
        findings.iter().any(|f| f.sink == "wallet/agent-7/balance"
            && f.sink_reason == SinkReason::PrivilegedWriteNamespace),
        "the wallet/ write key must be a privileged-write-namespace sink: {findings:?}"
    );
}

// ── GATE (2): the SHARED admission oracle REJECTS the tainted call ────────────

/// `decide_admission_with_taint` returns `Fail` for a non-empty findings set, with
/// the `arg_taint_v1[...]` failed-predicate marker, REGARDLESS of an otherwise
/// passing (empty) claim set under a clean zero root. This is the oracle-level
/// hard-gate.
#[test]
fn decide_admission_with_taint_rejects_tainted_privileged_flow() {
    let findings = arg_taint_v1(&tainted_privileged_call());
    assert!(!findings.is_empty(), "fixture must produce findings");

    let verdict = decide_admission_with_taint(
        &zero_root_hex(),
        &PredicateClaimSet::default(), // would PASS on its own (empty claim set)
        false,
        &findings,
    );

    match verdict {
        AdmissionVerdict::Fail {
            failed_predicate,
            reason,
        } => {
            assert!(
                failed_predicate.starts_with(ARG_TAINT_FAILED_PREDICATE_PREFIX),
                "taint rejection receipt must carry the arg_taint_v1 marker; got {failed_predicate}"
            );
            assert!(
                failed_predicate.contains("tainted_arg_into_privileged_sink"),
                "marker must encode the redact-safe finding reason: {failed_predicate}"
            );
            assert_eq!(
                reason,
                AdmissionFailReason::AcceptancePredicateFalse,
                "taint reject reuses the existing AcceptancePredicateFalse reason \
                 (no new variant → genesis-pinned sequencer match stays valid)"
            );
        }
        AdmissionVerdict::Pass { .. } => panic!(
            "ARG-TAINT HARD-GATE REGRESSION: decide_admission_with_taint ADMITTED a \
             tainted-arg → privileged-sink flow. A tainted wtool argument reaching a \
             privileged sink must be REFUSED at admission."
        ),
    }
}

/// CONTROL — the SAME oracle with EMPTY findings (a trusted-arg call) delegates to
/// the unchanged `decide_admission` and PASSES under a clean zero root. Proves the
/// oracle hard-gate is non-vacuous (it does not reject empty findings).
#[test]
fn decide_admission_with_taint_admits_clean_call() {
    let findings = arg_taint_v1(&trusted_privileged_call());
    assert!(
        findings.is_empty(),
        "a Trusted arg into a privileged sink must produce NO findings (clean)"
    );
    let verdict = decide_admission_with_taint(
        &zero_root_hex(),
        &PredicateClaimSet::default(),
        false,
        &findings,
    );
    assert!(
        matches!(verdict, AdmissionVerdict::Pass { .. }),
        "a clean (no-finding) call must PASS the admission oracle; got {verdict:?}"
    );
}

// ── GATE (2)+(3): the LIVE kernel leg rejects tainted / admits clean ──────────

/// HARD-GATE (negative leg) — the live kernel REFUSES to advance the verified head
/// for a worker `Proceed` carrying a tainted-arg → privileged-sink flow. The head
/// stays frozen (no `Q_{t+1}`), and the rejection is tape-recorded with the
/// `ArgTaintIntoPrivilegedSink` reject_class.
#[test]
fn kernel_rejects_tainted_arg_into_privileged_sink() {
    let (admitted, head) = kernel_admits_with_taint(&tainted_privileged_call());
    assert!(
        !admitted,
        "ARG-TAINT HARD-GATE REGRESSION: the kernel ADMITTED (advanced \
         verified_head) a worker Proceed whose wtool argument is tainted \
         (UntrustedExternal) and flows into a privileged (economic-wallet) sink. \
         The kernel Proceed branch must route the taint findings through \
         predicate_admission::decide_admission_with_taint, which returns Fail → \
         handle_rejection (no head advance)."
    );

    // The verified head stays frozen at the genesis H0 — no `Q_{t+1}` for a
    // tainted-arg → privileged-sink Proceed. (The non-advancing rejection is
    // committed via handle_rejection with the ArgTaintIntoPrivilegedSink
    // reject_class / arg_taint_v1 failed-predicate marker.)
    assert_eq!(
        head, "H0",
        "verified head must remain at the genesis H0 (no advance)"
    );
}

/// POSITIVE CONTROL (positive leg) — the SAME kernel leg fed a TRUSTED-arg call
/// (no findings) ADMITS and advances the verified head. Proves the hard-gate does
/// NOT reject every Proceed — it discriminates on argument taint.
#[test]
fn kernel_admits_clean_trusted_arg_call() {
    let (admitted, _head) = kernel_admits_with_taint(&trusted_privileged_call());
    assert!(
        admitted,
        "ARG-TAINT control: the kernel must ADMIT (advance verified_head) a worker \
         Proceed whose wtool argument is Trusted (no tainted→privileged flow → no \
         findings → decide_admission_with_taint delegates to the passing \
         decide_admission). A failure here means the hard-gate over-rejects clean \
         calls (vacuous gate)."
    );
}

/// META-GUARD — the negative and positive legs reach OPPOSITE verdicts for calls
/// that differ ONLY in the argument's taint label. This single assertion is the
/// non-vacuity proof: same sink, same shape, taint flips the admission outcome.
#[test]
fn taint_label_is_the_sole_discriminator() {
    let (tainted_admitted, _h1) = kernel_admits_with_taint(&tainted_privileged_call());
    let (trusted_admitted, _h2) = kernel_admits_with_taint(&trusted_privileged_call());
    assert_ne!(
        tainted_admitted, trusted_admitted,
        "NON-VACUITY VIOLATION: the kernel reached the SAME admission verdict for a \
         tainted-arg call ({tainted_admitted}) and a trusted-arg call \
         ({trusted_admitted}) that differ ONLY in the argument taint label. The \
         arg-taint hard-gate must REJECT the tainted call and ADMIT the trusted \
         one — the taint label is the sole discriminator."
    );
    assert!(!tainted_admitted && trusted_admitted);
}
