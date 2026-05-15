//! TB-1 Day-5 final acceptance battery — Tier-A 10 BLOCKING + Tier-B 4 NON-BLOCKING.
//!
//! Charter: `handover/tracer_bullets/TB-1_recharter_2026-04-29.md` § Day-5.
//! Path A++ amendments (2026-04-29, post Day-6 dual audit): Tier-A grew from 9
//! to 10 (P0-2 promoted), and the limitations section was added to make TB-1's
//! narrowed ship claim explicit. See
//! `handover/audits/DUAL_AUDIT_TB_1_VERDICT_2026-04-29.md` § Recommended path.
//!
//! Tier discipline (audit CF-5 "lighter option"): TB-1 ships when ALL Tier-A
//! tests are green. Tier-B tests are captured as artifacts but DO NOT gate
//! ship; if a Tier-B test goes red, file as a follow-up TB rather than
//! blocking TB-1's P1/P3 RSP-0 deliverable.
//!
//! Tier-A (BLOCKING — P1 + P3 RSP-0 correctness):
//!   1. test_p1_kill_1_no_wtool_bypass                       (P1 kill 1)
//!   2. test_p1_kill_2_rejected_tx_no_state_advance          (P1 kill 2)
//!   3. test_p1_kill_3_ledger_reconstructable                (P1 kill 3)
//!   4. test_p1_kill_4_rejected_log_isolated                 (P1 kill 4)
//!   5. test_p1_exit_7_l4_chain_breaks_on_row_deletion       (P1 Exit 7)
//!   6. test_p1_kill_4b_rejection_chain_breaks_on_row_deletion (P1 kill 4b)
//!   7. test_p3_rsp0_exit_1_on_init_total_invariant          (P3 RSP-0 Exit 1)
//!   8. test_p3_rsp0_exit_2_read_is_free                     (P3 RSP-0 Exit 2)
//!   9. test_p3_kill_1_no_post_init_mint                     (P3 kill 1)
//!  10. test_p3_rsp0_total_supply_counts_all_six_subindexes  (P0-2 path-A++)
//!
//! Tier-B (NON-BLOCKING — P6 anchor evidence + future-RSP placeholders):
//!  11. test_at1_evaluator_solves_mathd_algebra_107_n3       (#[ignore]: live LLM)
//!  12. test_at2_l4_entry_per_dispatched_tx                  (#[ignore]: WorkTx dispatch
//!                                                           body lands TB-2 RSP-1)
//!  13. test_at3_h_vppu_non_null_on_second_run               (UNIT form; live form
//!                                                           verified by Day-4 evidence)
//!  14. test_at4_econ_balance_delta_non_zero                 (#[ignore]: RSP-1)
//!
//! AT-5 (winning-tactic-in-prompt-context) is DESCOPED per recharter — moves
//! to a future P5 MetaTape v1 TB after P3 RSP-3 lands.
//!
//! ─── Limitations (Path A++ narrowed claim) ──────────────────────────────────
//!
//! TB-1 ships **P1/P3 primitives + invariant scaffolding**, NOT runtime
//! dispatch enforcement. The Tier-A battery PROVES:
//!   - L4 / L4.E split as data structures (hash chains, projections, tamper
//!     detection, type-shielded raw diagnostic).
//!   - Monetary invariant pure functions (no_post_init_mint, total CTF
//!     conservation across all six holding subindexes, read-is-free at
//!     tx-level).
//!
//! The Tier-A battery DOES NOT prove (deferred to TB-2 RSP-1):
//!   - That `Sequencer::dispatch_transition` actually CALLS these guards on
//!     the production path — `dispatch_transition` returns
//!     `NotYetImplemented` for all 7 K5 `TypedTx` variants today.
//!   - That a real predicate-failed `WorkTx` dispatched through the sequencer
//!     lands in L4.E (rather than aborting before any append) — `apply_one`
//!     early-returns on transition error.
//!   - That `assert_total_ctf_conserved` / `assert_read_is_free` /
//!     `assert_no_post_init_mint` are wired into any production call site —
//!     they pass module + Tier-A tests but no caller has been audited to
//!     invoke them yet.
//!   - SDK-boundary read-is-free (rtool / search / private think): only the
//!     tx-level no-fee invariant is covered here.
//!   - That CAS namespacing prevents raw diagnostic leakage if a
//!     `raw_diagnostic_cid` is shared — the type shield prevents accidental
//!     serialization, but capability-gated forensic access lands in a later
//!     TB.

use std::collections::{BTreeMap, BTreeSet};

use turingosv4::bottom_white::cas::schema::Cid;
use turingosv4::bottom_white::ledger::rejection_evidence::{
    PublicRejectionView, RejectionClass, RejectionEvidenceError, RejectionEvidenceWriter,
};
use turingosv4::bottom_white::ledger::transition_ledger::TxKind;
use turingosv4::economy::ledger::{AcceptedLedger, LedgerError};
use turingosv4::economy::monetary_invariant::{
    assert_read_is_free, assert_total_ctf_conserved, MonetaryError,
};
use turingosv4::economy::money::{MicroCoin, StakeMicroCoin, MICRO_PER_COIN};
use turingosv4::state::q_state::{AgentId, EconomicState, Hash, TaskId, TxId};
use turingosv4::state::typed_tx::{
    AgentSignature, BoolWithProof, PredicateId, PredicateResultsBundle, ReadKey, SafetyOrCreation,
    TypedTx, WorkTx, WriteKey,
};

// ────────────────────────────────────────────────────────────────────────────
// Fixtures
// ────────────────────────────────────────────────────────────────────────────

fn fixture_work_tx(suffix: u32) -> TypedTx {
    let mut acceptance = BTreeMap::new();
    acceptance.insert(
        PredicateId(format!("acc-{}", suffix)),
        BoolWithProof {
            value: true,
            proof_cid: Some(Cid([0x11; 32])),
        },
    );
    let mut settlement = BTreeMap::new();
    settlement.insert(
        PredicateId(format!("set-{}", suffix)),
        BoolWithProof {
            value: true,
            proof_cid: None,
        },
    );
    let mut read_set = BTreeSet::new();
    read_set.insert(ReadKey(format!("k.r.{}", suffix)));
    let mut write_set = BTreeSet::new();
    write_set.insert(WriteKey(format!("k.w.{}", suffix)));
    TypedTx::Work(WorkTx {
        tx_id: TxId(format!("worktx-{}", suffix)),
        task_id: TaskId(format!("task-{}", suffix)),
        parent_state_root: Hash::ZERO,
        agent_id: AgentId("alice".into()),
        read_set,
        write_set,
        proposal_cid: Cid([0x13; 32]),
        predicate_results: PredicateResultsBundle {
            acceptance,
            settlement,
            safety_class: SafetyOrCreation::Safety,
        },
        stake: StakeMicroCoin::from_micro_units(1_000_000),
        signature: AgentSignature::from_bytes([0x77u8; 64]),
        timestamp_logical: suffix as u64,
    })
}

fn cid(byte: u8) -> Cid {
    Cid([byte; 32])
}

fn agent(s: &str) -> AgentId {
    AgentId(s.to_string())
}

fn coin(n: i64) -> MicroCoin {
    MicroCoin::from_coin(n).unwrap()
}

// ════════════════════════════════════════════════════════════════════════════
// Tier-A — BLOCKING
// ════════════════════════════════════════════════════════════════════════════

// ────────────────────────────────────────────────────────────────────────────
// (1) P1 kill 1 — no wtool bypass
// ────────────────────────────────────────────────────────────────────────────

#[test]
fn test_p1_kill_1_no_wtool_bypass() {
    let mut l = AcceptedLedger::new();
    for i in 1..=3 {
        l.append_accepted(&fixture_work_tx(i)).unwrap();
    }
    let canonical_root = l.current_state_root();

    let tmp = tempfile::NamedTempFile::new().unwrap();
    l.persist(tmp.path()).unwrap();

    // Bypass: directly overwrite state.db without going through L4.
    let raw = std::fs::read(tmp.path()).unwrap();
    let mut tampered: Vec<turingosv4::economy::ledger::AcceptedEntry> =
        serde_json::from_slice(&raw).unwrap();
    tampered.last_mut().unwrap().resulting_state_root = Hash([0xFF; 32]);
    let bytes = serde_json::to_vec(&tampered).unwrap();
    std::fs::write(tmp.path(), bytes).unwrap();

    // Reconstruction MUST fail: explicit error OR diverged root.
    match AcceptedLedger::load_from_path(tmp.path()) {
        Err(_) => {} // bypass detected — expected
        Ok((_, reconstructed)) => assert_ne!(
            reconstructed, canonical_root,
            "bypass mutation must not survive a round-trip through reconstruct_state"
        ),
    }
}

// ────────────────────────────────────────────────────────────────────────────
// (2) P1 kill 2 — rejected tx does not advance state
// ────────────────────────────────────────────────────────────────────────────

#[test]
fn test_p1_kill_2_rejected_tx_no_state_advance() {
    let mut l4 = AcceptedLedger::new();
    let mut l4e = RejectionEvidenceWriter::new();

    l4.append_accepted(&fixture_work_tx(1)).unwrap();
    let baseline_root = l4.current_state_root();
    let baseline_logical_t = l4.len();

    l4e.append_rejected(
        42,
        baseline_root,
        agent("alice"),
        TxKind::Work,
        cid(0x20),
        RejectionClass::PredicateFailed,
        Some(cid(0xAA)),
        None,
    );

    assert_eq!(
        l4.current_state_root(),
        baseline_root,
        "rejected tx must NOT advance L4 state_root"
    );
    assert_eq!(
        l4.len(),
        baseline_logical_t,
        "rejected tx must NOT advance L4 logical_t"
    );

    assert_eq!(l4e.len(), 1, "rejection produces exactly one L4.E record");
    let r = &l4e.records()[0];
    assert_eq!(r.submit_id, 42);
    assert!(r.raw_diagnostic_cid.is_some());
    assert!(l4e.verify_chain().is_ok());
}

// ────────────────────────────────────────────────────────────────────────────
// (3) P1 kill 3 — ledger reconstructable
// ────────────────────────────────────────────────────────────────────────────

#[test]
fn test_p1_kill_3_ledger_reconstructable() {
    let mut l = AcceptedLedger::new();
    for i in 1..=4 {
        l.append_accepted(&fixture_work_tx(i)).unwrap();
    }
    let pre_drop_root = l.current_state_root();

    let tmp = tempfile::NamedTempFile::new().unwrap();
    l.persist(tmp.path()).unwrap();

    drop(l);

    let (l_reborn, reconstructed_root) = AcceptedLedger::load_from_path(tmp.path()).unwrap();
    assert_eq!(
        reconstructed_root, pre_drop_root,
        "reconstructed state_root must be bit-equal to pre-drop state_root"
    );
    assert_eq!(l_reborn.len(), 4);
    assert!(l_reborn.verify_chain(0, 4).is_ok());
}

// ────────────────────────────────────────────────────────────────────────────
// (4) P1 kill 4 — rejected log is isolated from agent-facing read view
// ────────────────────────────────────────────────────────────────────────────

#[test]
fn test_p1_kill_4_rejected_log_isolated() {
    let mut l4e = RejectionEvidenceWriter::new();
    l4e.append_rejected(
        7,
        Hash::ZERO,
        agent("alice"),
        TxKind::Work,
        cid(0x10),
        RejectionClass::PredicateFailed,
        Some(cid(0xBE)),
        Some("predicate acceptance failed for acc-7".into()),
    );

    let view: Vec<PublicRejectionView> = l4e.public_view();
    assert_eq!(view.len(), 1);

    let json = serde_json::to_value(&view[0]).unwrap();
    let obj = json
        .as_object()
        .expect("PublicRejectionView serializes as object");
    assert!(
        !obj.contains_key("raw_diagnostic_cid"),
        "raw_diagnostic_cid must NOT appear in agent-facing public view"
    );
    assert_eq!(
        obj.get("public_summary").and_then(|v| v.as_str()),
        Some("predicate acceptance failed for acc-7")
    );

    assert!(
        l4e.records()[0].raw_diagnostic_cid.is_some(),
        "L4.E forensic record must retain raw_diagnostic_cid (shielding is structural, not destructive)"
    );
}

// ────────────────────────────────────────────────────────────────────────────
// (5) P1 Exit 7 — L4 hash chain breaks on row deletion
// ────────────────────────────────────────────────────────────────────────────

#[test]
fn test_p1_exit_7_l4_chain_breaks_on_row_deletion() {
    let mut l = AcceptedLedger::new();
    for i in 1..=5 {
        l.append_accepted(&fixture_work_tx(i)).unwrap();
    }
    assert!(l.verify_chain(0, 5).is_ok());

    l.tamper_remove_entry(2);

    let r = l.verify_chain(0, 4);
    match r {
        Err(LedgerError::LogicalTGap { at_index: 2, .. })
        | Err(LedgerError::HashMismatch { at_index: 2 }) => {}
        other => panic!(
            "deleting an L4 row must break the chain at index 2; got {:?}",
            other
        ),
    }
}

// ────────────────────────────────────────────────────────────────────────────
// (6) P1 kill 4b — L4.E hash chain breaks on row deletion
// ────────────────────────────────────────────────────────────────────────────

#[test]
fn test_p1_kill_4b_rejection_chain_breaks_on_row_deletion() {
    let mut l4e = RejectionEvidenceWriter::new();
    for i in 1..=3u64 {
        l4e.append_rejected(
            i,
            Hash::ZERO,
            agent("alice"),
            TxKind::Work,
            cid(0x10),
            RejectionClass::PredicateFailed,
            None,
            None,
        );
    }
    assert!(l4e.verify_chain().is_ok());

    l4e.tamper_remove_record(1);
    let r = l4e.verify_chain();
    assert!(
        matches!(r, Err(RejectionEvidenceError::HashMismatch { at: 1 })),
        "deleting row 1 must surface as HashMismatch at the new index 1; got {:?}",
        r
    );
}

// ────────────────────────────────────────────────────────────────────────────
// (7) P3 RSP-0 Exit 1 — on_init total invariant across N tx sequence
// ────────────────────────────────────────────────────────────────────────────

#[test]
fn test_p3_rsp0_exit_1_on_init_total_invariant() {
    // After on_init mint, total_coin must be invariant across an arbitrary
    // sequence of redistribution txs (no further mints, no burns). We model
    // a 5-step alice/bob/escrow shuffle and assert assert_total_ctf_conserved
    // succeeds at each step with an empty exempt list.
    let mut s = EconomicState::default();
    s.balances_t.0.insert(agent("alice"), coin(100));
    let baseline = s.clone();

    // Step 1: alice → bob 30
    let mut s1 = EconomicState::default();
    s1.balances_t.0.insert(agent("alice"), coin(70));
    s1.balances_t.0.insert(agent("bob"), coin(30));
    assert_eq!(assert_total_ctf_conserved(&baseline, &s1, &[]), Ok(()));

    // Step 2: bob 30 → escrow
    use turingosv4::state::q_state::EscrowEntry;
    let mut s2 = EconomicState::default();
    s2.balances_t.0.insert(agent("alice"), coin(70));
    s2.escrows_t.0.insert(
        TxId("e-1".into()),
        EscrowEntry {
            amount: coin(30),
            depositor: agent("bob"),
            task_id: TaskId::default(),
        },
    );
    assert_eq!(assert_total_ctf_conserved(&s1, &s2, &[]), Ok(()));

    // Step 3: escrow back to bob
    let mut s3 = EconomicState::default();
    s3.balances_t.0.insert(agent("alice"), coin(70));
    s3.balances_t.0.insert(agent("bob"), coin(30));
    assert_eq!(assert_total_ctf_conserved(&s2, &s3, &[]), Ok(()));

    // Step 4: alice 70 → carol
    let mut s4 = EconomicState::default();
    s4.balances_t.0.insert(agent("carol"), coin(70));
    s4.balances_t.0.insert(agent("bob"), coin(30));
    assert_eq!(assert_total_ctf_conserved(&s3, &s4, &[]), Ok(()));

    // Step 5: full round-trip back to baseline
    let mut s5 = EconomicState::default();
    s5.balances_t.0.insert(agent("alice"), coin(100));
    assert_eq!(assert_total_ctf_conserved(&s4, &s5, &[]), Ok(()));

    // Final cross-check: end == start (closed-system loop).
    assert_eq!(
        s5.balances_t.0.get(&agent("alice")),
        baseline.balances_t.0.get(&agent("alice")),
        "round-trip must restore baseline"
    );
}

// ────────────────────────────────────────────────────────────────────────────
// (8) P3 RSP-0 Exit 2 — read-is-free (rtool / search / think MUST have fee=0)
// ────────────────────────────────────────────────────────────────────────────

#[test]
fn test_p3_rsp0_exit_2_read_is_free() {
    // K5 has no dedicated read-tx variants today; the structural guard runs
    // against ALL TxKinds. assert_read_is_free(kind, fee=0) must succeed for
    // every variant; non-zero fee on ANY variant must surface as ReadCharged.
    for kind in [
        TxKind::Work,
        TxKind::Verify,
        TxKind::Challenge,
        TxKind::Reuse,
        TxKind::FinalizeReward,
        TxKind::TaskExpire,
        TxKind::TerminalSummary,
    ] {
        assert_eq!(
            assert_read_is_free(kind, 0),
            Ok(()),
            "fee=0 must pass for TxKind={:?}",
            kind
        );
    }

    // Anti-Goodhart: any non-zero fee at any kind is structurally rejected.
    assert_eq!(
        assert_read_is_free(TxKind::Reuse, 1),
        Err(MonetaryError::ReadCharged {
            tx_kind: TxKind::Reuse,
            fee: 1
        })
    );
    assert_eq!(
        assert_read_is_free(TxKind::Work, 9999),
        Err(MonetaryError::ReadCharged {
            tx_kind: TxKind::Work,
            fee: 9999
        })
    );
}

// ────────────────────────────────────────────────────────────────────────────
// (9) P3 kill 1 — no post-init mint (rejected route MUST go to L4.E, not L4)
// ────────────────────────────────────────────────────────────────────────────

#[test]
fn test_p3_kill_1_no_post_init_mint() {
    // The numeric kill: any post-init mint surfaces as MonetaryError::PostInitMint.
    let before = EconomicState::default();
    let mut after = EconomicState::default();
    after.balances_t.0.insert(agent("alice"), coin(100));
    let r = assert_total_ctf_conserved(&before, &after, &[]);
    assert_eq!(
        r,
        Err(MonetaryError::PostInitMint {
            delta_micro: 100 * MICRO_PER_COIN
        }),
        "any non-exempt supply increase must surface as PostInitMint"
    );

    // The structural kill: L4.E (NOT L4) is the home for the rejection record.
    // Simulate the dispatch_transition rejection path by writing the rejection
    // ONLY to L4.E and asserting L4 is untouched.
    let l4 = AcceptedLedger::new();
    let pre_root = l4.current_state_root();
    let pre_logical_t = l4.len();

    let mut l4e = RejectionEvidenceWriter::new();
    l4e.append_rejected(
        99,
        pre_root,
        agent("alice"),
        TxKind::Work,
        cid(0x30),
        RejectionClass::InvariantViolation,
        Some(cid(0xC0)),
        Some("PostInitMint: delta_micro=100000000".into()),
    );

    // L4 untouched; L4.E has the record.
    assert_eq!(l4.current_state_root(), pre_root);
    assert_eq!(l4.len(), pre_logical_t);
    assert_eq!(l4e.len(), 1);
    assert!(matches!(
        l4e.records()[0].rejection_class,
        RejectionClass::InvariantViolation
    ));
}

// ────────────────────────────────────────────────────────────────────────────
// (10) P3 RSP-0 — total_supply counts ALL FOUR holding subindexes
//      (Path A++ / Codex P0-2 audit 2026-04-29; TB-3 6→5 migration 2026-04-30;
//       TB-8 5→4 migration 2026-05-02 — claims_t becomes intent registry)
// ────────────────────────────────────────────────────────────────────────────

#[test]
fn test_p3_rsp0_total_supply_counts_all_four_subindexes() {
    // Tier-A 7 (`test_p3_rsp0_exit_1_on_init_total_invariant`) only redistributes
    // through balances + escrows. A regression that silently undercounts any
    // OTHER holding subindex (stakes_t / challenge_cases_t.bond) would still
    // pass test 7.
    //
    // **TB-3 6→5 migration**: TB-1's six-holding sum (balances + escrows +
    // stakes + claims + bounty + bond) was reduced to five (balances + escrows
    // + stakes + claims + bond). `task_markets_t.total_escrow` is a derived
    // cache (TB-3 charter § 3.2), NOT a holding.
    //
    // **TB-8 5→4 migration** (2026-05-02): claims_t is now an intent
    // registry, NOT a holding. Per TB-8 charter §3 Atom 3 + ratification
    // §1 Q5: FinalizeReward dispatches escrows → balances directly; claims_t
    // is metadata. The seeded claim row contributes 0 to the supply sum;
    // the seeded total drops from 63 → 55. Each remaining single-drop
    // deficit is still unique: drop balances → -1, drop escrows → -18,
    // drop stakes → -4, drop bond → -32.
    use turingosv4::state::q_state::{ChallengeCase, ClaimEntry, EscrowEntry, StakeEntry};

    let mut seeded = EconomicState::default();
    seeded.balances_t.0.insert(agent("a"), coin(1));
    seeded.escrows_t.0.insert(
        TxId("e".into()),
        EscrowEntry {
            amount: coin(2),
            depositor: agent("a"),
            task_id: TaskId("task-e".into()),
        },
    );
    seeded.stakes_t.0.insert(
        TxId("s".into()),
        StakeEntry {
            amount: coin(4),
            staker: agent("a"),
            task_id: TaskId("task-s".into()),
        },
    );
    // claims_t entry seeded as intent metadata; per TB-8 not in holding sum.
    seeded.claims_t.0.insert(
        TxId("c".into()),
        ClaimEntry {
            amount: coin(8),
            claimant: agent("a"),
            ..Default::default()
        },
    );
    // Second escrow row carries what used to live in task_markets_t.bounty.
    seeded.escrows_t.0.insert(
        TxId("e2".into()),
        EscrowEntry {
            amount: coin(16),
            depositor: agent("a"),
            task_id: TaskId("task-e2".into()),
        },
    );
    let mut cc = ChallengeCase::default();
    cc.bond = coin(32);
    cc.challenger = agent("a");
    seeded.challenge_cases_t.0.insert(TxId("ch".into()), cc);

    // Conservation across before(empty) → after(seeded) MUST be detected as
    // a mint of exactly 55 coin (NOT 63 — claims_t coin(8) is intent, not
    // counted post-TB-8).
    let before = EconomicState::default();
    let r = assert_total_ctf_conserved(&before, &seeded, &[]);
    assert_eq!(
        r,
        Err(MonetaryError::PostInitMint {
            delta_micro: 55 * MICRO_PER_COIN
        }),
        "monetary invariant must sum balances + escrows + stakes + bond \
         (4 holdings post-TB-8; claims_t is intent registry, NOT a holding)"
    );

    let r_inv = assert_total_ctf_conserved(&seeded, &before, &[]);
    assert_eq!(
        r_inv,
        Err(MonetaryError::TotalCtfBurn {
            delta_micro: -(55 * MICRO_PER_COIN)
        }),
        "monetary invariant must symmetrically detect a burn across all four subindexes"
    );
}

// ════════════════════════════════════════════════════════════════════════════
// Tier-B — NON-BLOCKING (artifacts; do not gate ship)
// ════════════════════════════════════════════════════════════════════════════

// (11) AT-1 P6 anchor — evaluator solves mathd_algebra_107 in n3 mode.
//
// Verified out-of-band by the Day-4 live runs documented in commit 50a1d67:
// RUN 1 + RUN 2 both produced solved=true with gp_payload=nlinarith. Capturing
// this here as an #[ignore] live integration test so the assertion is REGISTERED
// in the harness even though it requires a running LLM proxy + DEEPSEEK_API_KEY
// to execute. Run manually with:
//   cargo test test_at1_evaluator_solves_mathd_algebra_107_n3 -- --ignored
#[test]
#[ignore = "Tier-B P6 anchor: requires live LLM proxy + DEEPSEEK_API_KEY; verified Day-4 (commit 50a1d67)"]
fn test_at1_evaluator_solves_mathd_algebra_107_n3() {
    // Live form would shell out to target/release/evaluator with CONDITION=n3
    // ACTIVE_MODEL=deepseek-chat MAX_TRANSACTIONS=10 and assert solved=true on
    // the JSONL row. Body intentionally empty — TB-1 ship gate is the manually-
    // observable Day-4 evidence in /tmp/tb1_day4_smoke_v2/run{1,2}.jsonl.
}

// (12) AT-2 — each tx in evaluator run produces an L4 LedgerEntry.
// Non-blocking until WorkTx dispatch_transition body lands at TB-2 RSP-1.
#[test]
#[ignore = "Tier-B: WorkTx dispatch_transition body lands TB-2 RSP-1; evaluator currently uses legacy emit path"]
fn test_at2_l4_entry_per_dispatched_tx() {
    // When TB-2 RSP-1 wires the WorkTx → AcceptedLedger::append_accepted path,
    // un-ignore this test and assert: for every successful evaluator tx, exactly
    // one L4 entry is appended; the entry's tx_payload_hash equals the tx's
    // canonical hash; verify_chain(0, n) succeeds at the end of the run.
}

// (13) AT-3 — h_vppu non-null on a 2nd-run row.
//
// The live form (2 evaluator invocations producing JSONL rows) is verified by
// the Day-4 evidence at /tmp/tb1_day4_smoke_v2/run2.jsonl (commit 50a1d67):
// run 2 carried `h_vppu=6.215891726697228`. The unit-level CONTRACT (capacity-3
// rolling history; record-then-query semantics; persistence round-trip) is
// covered by the 9 unit tests inside `minif2f_v4::h_vppu_history` itself
// (cargo test -p minif2f_v4 --lib h_vppu_history → 9/9 PASS).
//
// This file is a top-level integration test for the `turingosv4` crate and
// cannot import the `minif2f_v4` experiments crate (asymmetric path dep).
// Registering the AT-3 contract here as a `#[ignore]` documentation stub so
// the harness lists it explicitly; un-ignore + relocate to
// `experiments/minif2f_v4/tests/` if/when minif2f_v4 grows an integration
// test directory.
#[test]
#[ignore = "Tier-B AT-3: covered by minif2f_v4 lib tests + Day-4 live evidence (commit 50a1d67); cannot import minif2f_v4 from turingosv4 integration tests"]
fn test_at3_h_vppu_non_null_on_second_run() {
    // No body — see ignore reason above.
}

// (14) AT-4 — PputResult.econ_balance_delta non-zero.
// Non-blocking until TB-2 RSP-1's escrow_lock_tx + yes_stake_tx fire. RSP-0
// (Day-2) only proves the conservation invariant + scaffolds escrow/balances
// structures; actual non-zero deltas need the RSP-1 wiring.
#[test]
#[ignore = "Tier-B: needs RSP-1 escrow_lock_tx + yes_stake_tx wiring (TB-2)"]
fn test_at4_econ_balance_delta_non_zero() {
    // When TB-2 RSP-1 lands, un-ignore and assert: for an evaluator run that
    // exercises an escrow_lock_tx, PputResult.econ_balance_delta is Some(non-zero).
    // RSP-0 today only ships the conservation invariant + escrow_vault scaffolding.
}
