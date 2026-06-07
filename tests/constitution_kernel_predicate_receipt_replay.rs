//! LIVE CONSTITUTION GATE (M07) — kernel predicate-admission receipt REPLAY /
//! RECONSTRUCTION witness (spec §10, strongest form of G1).
//!
//! STATUS: LIVE / GREEN. Added under the user's §8 token
//! `APPROVE-M07-A4-SINGLE-ADMISSION-PREDICATE-GATE` (2026-06-07) alongside the
//! route-A single-admission predicate gate.
//!
//! ── WHAT THIS GATE PROVES (beyond G1's existence probe) ───────────────────
//! `tests/constitution_kernel_predicate_gate.rs` proves a `predicate_admission`
//! receipt EXISTS somewhere on the tape. This gate is the stronger replay
//! witness the M07 spec §10 demands: it drives a real kernel `step_forward`
//! happy path, then — from `dump_all_nodes()` ALONE (no kernel-internal state) —
//!   1. locates the ADVANCING `StateAccepted` node (the one whose content hash
//!      equals the current `verified_head`),
//!   2. reads the additive `predicate_admission` PASS receipt out of that node's
//!      hash-covered payload, and
//!   3. RE-EXECUTES `predicate_admission::decide_admission` on the SAME claim set
//!      the kernel admitted under, then asserts the receipt's verdict and
//!      `registry_root` match the re-execution.
//!
//! i.e. it proves the kernel admission leg reached exactly the verdict the
//! shared contract (and therefore the sequencer leg, which calls the same
//! contract) would reach for that claim set — the single-admission invariant,
//! reconstructed from tape evidence rather than asserted from kernel internals.
//!
//! Both the empty-claim-set 3-arg path (legacy callers) and an explicit
//! multi-claim `step_forward_with_claims` path are exercised, so the
//! verdict-match is non-vacuous.
//!
//! ── TRIPLE-COUPLING ──────────────────────────────────────────────────────
//! Registered in `scripts/constitution_gates.manifest.toml`
//! (`constitution_kernel_predicate_receipt_replay`) and referenced in
//! `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md`. Discovered by the flat
//! `ls tests/constitution_*.rs` glob in `scripts/run_constitution_gates.sh` and
//! built by `cargo test --workspace`.

use turingosv4::charter_core::compile_charter_core;
use turingosv4::ledger::{ImmutableTapeLedger, MemoryTapeLedger, NodeKind, TapeNode};
use turingosv4::memory_kernel::{EnvironmentResult, KernelStep, MemoryKernel, Task};
use turingosv4::predicate_admission::{
    decide_admission, hash_to_hex, AdmissionVerdict, PredicateClaim, PredicateClaimSet,
};
use turingosv4::state::q_state::Hash;
use turingosv4::state::typed_tx::PredicateId;
use turingosv4::tokenizer::Tokenizer;

/// A worker `EnvironmentResult` the kernel treats as a happy path: `success`
/// plus a parseable prefix-JSON header with `status:"Proceed"`.
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

/// A fresh zero-root kernel (the defaulting `new` constructor: empty registry,
/// `Hash::ZERO` root, empty CAS — `os_qualified == false`).
fn fresh_kernel() -> MemoryKernel<MemoryTapeLedger> {
    let mut tape = MemoryTapeLedger::new();
    tape.set_verified_head("H0".into());
    let charter = compile_charter_core(
        "# Constitution\n## Art. 0.4 — Q_t version control\nFC1a tape_t.\n".as_bytes(),
        "v1.0",
        &Tokenizer::new(),
    );
    MemoryKernel::new(tape, "run-m07-replay", charter)
}

/// Reconstruct, from `dump_all_nodes()` alone, the node whose content hash
/// equals the current verified head. This is the *advancing* `StateAccepted`
/// node — found purely from tape evidence, exactly as an offline auditor would.
fn advancing_node_from_tape(tape: &MemoryTapeLedger) -> TapeNode {
    let head = tape.get_verified_head();
    let node = tape
        .dump_all_nodes()
        .into_iter()
        .map(|(_h, n)| n)
        .find(|n| n.hash == head)
        .unwrap_or_else(|| {
            panic!(
                "M07 replay failure: no tape node has content hash == verified_head `{head}`. \
                 The advancing StateAccepted node must be reconstructable from \
                 dump_all_nodes() alone."
            )
        });
    assert!(
        matches!(node.kind, NodeKind::StateAccepted),
        "M07 replay failure: the verified-head node is {:?}, expected StateAccepted. \
         The head advance must land on a receipt-bearing accepted node.",
        node.kind
    );
    node
}

/// Pull the embedded `predicate_admission` receipt object out of an accepted
/// node's hash-covered payload. Panics with an M07 message if absent — that is
/// the bypass-reopened condition.
fn receipt_of<'a>(node: &'a TapeNode) -> &'a serde_json::Value {
    node.payload.get("predicate_admission").unwrap_or_else(|| {
        panic!(
            "M07 replay failure: advancing StateAccepted node `{}` carries NO \
             `predicate_admission` receipt in its payload. The predicate-blind \
             kernel bypass has re-opened.",
            node.hash
        )
    })
}

/// Assert that the receipt recorded on tape matches a fresh re-execution of the
/// shared admission contract on `claims` under a zero (non-OS-qualified) root.
/// This is the core replay assertion: the tape-recorded verdict equals what the
/// sequencer leg would independently compute for the same claim set.
fn assert_receipt_matches_reexecution(receipt: &serde_json::Value, claims: &PredicateClaimSet) {
    // Re-execute the SAME contract both admission legs call, under the SAME
    // zero-root / non-OS-qualified context the kernel admitted under.
    let root_hex = hash_to_hex(&Hash::ZERO);
    let reexec = decide_admission(&root_hex, claims, false);

    let expected_registry_root = match &reexec {
        AdmissionVerdict::Pass { registry_root_hex } => registry_root_hex.clone(),
        AdmissionVerdict::Fail { .. } => panic!(
            "M07 replay test bug: the happy-path claim set must re-execute to PASS, got {reexec:?}"
        ),
    };

    // (1) verdict string
    let recorded_verdict = receipt
        .get("verdict")
        .and_then(|v| v.as_str())
        .expect("receipt must carry a string `verdict`");
    assert_eq!(
        recorded_verdict, "PASS",
        "M07 replay mismatch: tape receipt verdict `{recorded_verdict}` != re-executed verdict \
         PASS for the same claim set."
    );

    // (2) registry root the decision was taken under (Hash::ZERO hex here)
    let recorded_root = receipt
        .get("registry_root")
        .and_then(|v| v.as_str())
        .expect("receipt must carry a string `registry_root`");
    assert_eq!(
        recorded_root, expected_registry_root,
        "M07 replay mismatch: tape receipt registry_root `{recorded_root}` != re-executed \
         registry_root `{expected_registry_root}`."
    );

    // (3) os_qualified must be false on the zero-root legacy path (must NOT
    //     over-claim oracle re-execution; see constitution_predicate_zero_root_is_not_oracle).
    let recorded_os_qualified = receipt
        .get("os_qualified")
        .and_then(|v| v.as_bool())
        .expect("receipt must carry a bool `os_qualified`");
    assert!(
        !recorded_os_qualified,
        "M07 replay mismatch: zero-root receipt claims os_qualified=true; the legacy \
         verdict-trusting branch must record os_qualified=false."
    );

    // (4) the receipt's recorded claim pids must equal the claim set the kernel
    //     admitted under, so the re-execution is over the SAME claims (not a
    //     different set that happens to also PASS).
    let recorded_acceptance: Vec<String> = receipt
        .get("acceptance_pids")
        .and_then(|v| v.as_array())
        .map(|a| {
            a.iter()
                .filter_map(|x| x.as_str().map(str::to_string))
                .collect()
        })
        .expect("receipt must carry an `acceptance_pids` array");
    let expected_acceptance: Vec<String> =
        claims.acceptance.iter().map(|c| c.id.0.clone()).collect();
    assert_eq!(
        recorded_acceptance, expected_acceptance,
        "M07 replay mismatch: receipt acceptance_pids {recorded_acceptance:?} != the claim set \
         the kernel admitted under {expected_acceptance:?}. The recorded receipt must describe \
         the exact claims the verdict was taken over."
    );

    let recorded_settlement: Vec<String> = receipt
        .get("settlement_pids")
        .and_then(|v| v.as_array())
        .map(|a| {
            a.iter()
                .filter_map(|x| x.as_str().map(str::to_string))
                .collect()
        })
        .expect("receipt must carry a `settlement_pids` array");
    let expected_settlement: Vec<String> =
        claims.settlement.iter().map(|c| c.id.0.clone()).collect();
    assert_eq!(
        recorded_settlement, expected_settlement,
        "M07 replay mismatch: receipt settlement_pids {recorded_settlement:?} != the claim set \
         the kernel admitted under {expected_settlement:?}."
    );
}

/// REPLAY WITNESS — empty-claim-set path (the 3-arg shim every legacy caller
/// uses). The advancing StateAccepted node, reconstructed from tape alone,
/// carries a PASS receipt that matches a re-execution of `decide_admission` on
/// the (empty) claim set.
#[test]
fn replay_reconstructs_kernel_admission_receipt_empty_claims() {
    let mut k = fresh_kernel();
    let task = Task {
        id: "t-empty".into(),
        prompt: "do the thing".into(),
    };
    let head_before = k.tape.get_verified_head();

    // 3-arg shim => PredicateClaimSet::default() (empty) under a zero root.
    let step = k.step_forward(&task, proceed_env("t-empty"));
    assert!(
        matches!(step, KernelStep::Proceed { .. }),
        "happy path must Proceed"
    );
    assert_ne!(
        k.tape.get_verified_head(),
        head_before,
        "Proceed must advance the verified head"
    );

    // Reconstruct from tape evidence ALONE.
    let node = advancing_node_from_tape(&k.tape);
    let receipt = receipt_of(&node);

    // The claim set the 3-arg shim admits under is the empty default.
    let admitted_claims = PredicateClaimSet::default();
    assert_receipt_matches_reexecution(receipt, &admitted_claims);

    // The Proceed evidence_hash must equal the advancing node's hash — the same
    // node an offline auditor reconstructs as the receipt carrier.
    if let KernelStep::Proceed { evidence_hash } = step {
        assert_eq!(
            evidence_hash, node.hash,
            "Proceed evidence_hash must equal the advancing StateAccepted node's content hash."
        );
    }
}

/// REPLAY WITNESS — explicit multi-claim path. The judge seam in `tdma_runner`
/// builds a real claim set and calls `step_forward_with_claims`; this exercises
/// the same path and proves the tape receipt records the EXACT claims and a
/// verdict matching a re-execution over them (non-vacuous: the claim set has
/// content, all-true so it PASSes, and the recorded pids must round-trip).
#[test]
fn replay_reconstructs_kernel_admission_receipt_with_explicit_claims() {
    let mut k = fresh_kernel();
    let task = Task {
        id: "t-claims".into(),
        prompt: "do the thing".into(),
    };

    // An all-true claim set: zero-root + non-OS-qualified => PASS. The pids are
    // chosen out of BTreeMap-sort order to prove the receipt records the exact
    // Vec order the kernel admitted under (not a re-sorted set).
    let admitted_claims = PredicateClaimSet {
        acceptance: vec![
            PredicateClaim {
                id: PredicateId("judge::lean_pass".into()),
                value: true,
                proof_cid: None,
            },
            PredicateClaim {
                id: PredicateId("acc::schema_ok".into()),
                value: true,
                proof_cid: None,
            },
        ],
        settlement: vec![PredicateClaim {
            id: PredicateId("set::reward_bounds".into()),
            value: true,
            proof_cid: None,
        }],
    };

    let head_before = k.tape.get_verified_head();
    let step = k.step_forward_with_claims(&task, proceed_env("t-claims"), admitted_claims.clone());
    assert!(
        matches!(step, KernelStep::Proceed { .. }),
        "all-true claim set must Proceed"
    );
    assert_ne!(
        k.tape.get_verified_head(),
        head_before,
        "Proceed must advance the verified head"
    );

    let node = advancing_node_from_tape(&k.tape);
    let receipt = receipt_of(&node);
    assert_receipt_matches_reexecution(receipt, &admitted_claims);
}

/// Negative replay: a claim set with a FALSE acceptance predicate must NOT
/// advance the head and must NOT mint a PASS receipt-bearing StateAccepted node.
/// This proves the receipt path is verdict-driven (the re-execution would Fail),
/// not unconditional — i.e. the gate is fail-able.
#[test]
fn replay_false_claim_does_not_mint_pass_receipt() {
    let mut k = fresh_kernel();
    let task = Task {
        id: "t-false".into(),
        prompt: "do the thing".into(),
    };

    let failing_claims = PredicateClaimSet {
        acceptance: vec![PredicateClaim {
            id: PredicateId("judge::lean_pass".into()),
            value: false,
            proof_cid: None,
        }],
        settlement: vec![],
    };

    // Sanity: the shared contract independently rejects this claim set.
    let reexec = decide_admission(&hash_to_hex(&Hash::ZERO), &failing_claims, false);
    assert!(
        matches!(reexec, AdmissionVerdict::Fail { .. }),
        "test bug: a false acceptance claim must re-execute to Fail"
    );

    let head_before = k.tape.get_verified_head();
    let step = k.step_forward_with_claims(&task, proceed_env("t-false"), failing_claims);

    // Head must stay frozen — the admission FAIL routes to the non-advancing
    // rejection path, NOT a head advance.
    assert_eq!(
        k.tape.get_verified_head(),
        head_before,
        "M07 replay failure: a false predicate claim advanced the verified head. The admission \
         FAIL must route to the non-advancing rejection path."
    );
    assert!(
        !matches!(step, KernelStep::Proceed { .. }),
        "M07 replay failure: a false predicate claim returned KernelStep::Proceed."
    );

    // No StateAccepted node may carry a PASS receipt under the still-frozen head.
    let pass_accepted = k.tape.dump_all_nodes().into_iter().any(|(_h, n)| {
        matches!(n.kind, NodeKind::StateAccepted)
            && n.payload
                .get("predicate_admission")
                .and_then(|r| r.get("verdict"))
                .and_then(|v| v.as_str())
                == Some("PASS")
    });
    assert!(
        !pass_accepted,
        "M07 replay failure: a PASS predicate-admission StateAccepted receipt was minted for a \
         claim set that re-executes to Fail."
    );
}
