//! TB-8 Minimal Payout / FinalizeRewardTx — integration tests.
//!
//! Charter: `handover/tracer_bullets/TB-8_charter_2026-05-02.md`.
//! Ratification: `handover/audits/CHARTER_RATIFICATION_TB_8_2026-05-02.md`.
//!
//! Atom 1 (claims_t writer) tests at top.
//! Atom 3 (FinalizeReward dispatch arm) tests follow.
//! Atom 4 (evaluator OMEGA-branch caller) is exercised end-to-end via
//! `experiments/minif2f_v4/src/bin/evaluator.rs` smoke runs (Atom 5
//! evidence directory). The integration tests in this file are the
//! deterministic in-process counterpart to the smoke ladder.
//!
//! Constitutional anchor: `Art.III.4` (no fake accepted; payout_sum ≤ escrow)
//! + `WP-§14.1` (FinalizeRewardTx already a TypedTx variant; no new variant).

use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, RwLock};

use tempfile::TempDir;

use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::bottom_white::ledger::rejection_evidence::RejectionEvidenceWriter;
use turingosv4::bottom_white::ledger::system_keypair::{
    Ed25519Keypair, PinnedSystemPubkeys, SystemEpoch,
};
use turingosv4::bottom_white::ledger::transition_ledger::{
    InMemoryLedgerWriter, LedgerWriter, TxKind,
};
use turingosv4::bottom_white::tools::registry::ToolRegistry;
use turingosv4::economy::money::{MicroCoin, StakeMicroCoin};
use turingosv4::state::q_state::{
    AgentId, ClaimStatus, Hash, QState, TaskId, TxId,
};
use turingosv4::state::sequencer::{
    Sequencer, SubmissionEnvelope, SystemEmitCommand,
};
use turingosv4::state::typed_tx::{
    AgentSignature, BoolWithProof, ClaimId, EscrowLockTx, PredicateId, PredicateResultsBundle,
    ReadKey, SafetyOrCreation, TaskOpenTx, TransitionError, TypedTx, VerifyTx, VerifyVerdict,
    WorkTx, WriteKey,
};
use turingosv4::top_white::predicates::registry::PredicateRegistry;

// ────────────────────────────────────────────────────────────────────────────
// Harness — mirror of TB-4 / TB-5 pattern.
// ────────────────────────────────────────────────────────────────────────────

struct Harness {
    _tmp: TempDir,
    seq: Sequencer,
    rx: tokio::sync::mpsc::Receiver<SubmissionEnvelope>,
    ledger_writer: Arc<RwLock<dyn LedgerWriter>>,
}

fn fresh_harness(initial_q: QState) -> Harness {
    let tmp = TempDir::new().expect("tempdir");
    let cas = Arc::new(RwLock::new(CasStore::open(tmp.path()).expect("cas")));
    let keypair = Arc::new(
        Ed25519Keypair::generate_with_secure_entropy().expect("keypair"),
    );
    let writer: Arc<RwLock<dyn LedgerWriter>> =
        Arc::new(RwLock::new(InMemoryLedgerWriter::new()));
    let rejection_writer = Arc::new(RwLock::new(RejectionEvidenceWriter::default()));
    let preds = Arc::new(PredicateRegistry::new());
    let tools = Arc::new(ToolRegistry::new());
    let epoch = SystemEpoch::new(1);
    let mut pinned = PinnedSystemPubkeys::new();
    pinned.insert(epoch, keypair.public_key());
    let pinned_pubkeys = Arc::new(pinned);
    let (seq, rx) = Sequencer::new(
        cas,
        keypair,
        epoch,
        writer.clone(),
        rejection_writer,
        preds,
        tools,
        pinned_pubkeys,
        initial_q,
        16,
    );
    Harness { _tmp: tmp, seq, rx, ledger_writer: writer }
}

fn genesis_with_balances(pairs: &[(&str, i64)]) -> QState {
    let mut q = QState::genesis();
    for (name, coin) in pairs {
        q.economic_state_t.balances_t.0.insert(
            AgentId((*name).into()),
            MicroCoin::from_coin(*coin).unwrap(),
        );
    }
    q
}

fn make_task_open(task: &str, sponsor: &str, parent: Hash, suffix: &str) -> TypedTx {
    TypedTx::TaskOpen(TaskOpenTx {
        tx_id: TxId(format!("taskopen-{}-{}", task, suffix)),
        task_id: TaskId(task.into()),
        parent_state_root: parent,
        sponsor_agent: AgentId(sponsor.into()),
        verifier_quorum: 1,
        max_reuse_royalty_fraction_basis_points: 1000,
        settlement_rule_hash: Hash::ZERO,
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: 1,
    })
}

fn make_escrow_lock(task: &str, sponsor: &str, amount_micro: i64, parent: Hash, suffix: &str) -> TypedTx {
    TypedTx::EscrowLock(EscrowLockTx {
        tx_id: TxId(format!("escrowlock-{}-{}", task, suffix)),
        task_id: TaskId(task.into()),
        parent_state_root: parent,
        sponsor_agent: AgentId(sponsor.into()),
        amount: MicroCoin::from_micro_units(amount_micro),
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: 1,
    })
}

fn make_worktx(task: &str, agent: &str, parent: Hash, stake_micro: i64, suffix: &str) -> TypedTx {
    let mut acceptance = BTreeMap::new();
    acceptance.insert(
        PredicateId("acc1".into()),
        BoolWithProof { value: true, proof_cid: None },
    );
    TypedTx::Work(WorkTx {
        tx_id: TxId(format!("worktx-{task}-{suffix}")),
        task_id: TaskId(task.into()),
        parent_state_root: parent,
        agent_id: AgentId(agent.into()),
        read_set: [ReadKey("k.read".into())].into_iter().collect::<BTreeSet<_>>(),
        write_set: [WriteKey("k.write".into())].into_iter().collect::<BTreeSet<_>>(),
        proposal_cid: Default::default(),
        predicate_results: PredicateResultsBundle {
            acceptance,
            settlement: BTreeMap::new(),
            safety_class: SafetyOrCreation::Safety,
        },
        stake: StakeMicroCoin::from_micro_units(stake_micro),
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: 1,
    })
}

fn make_verify_tx(
    target_work_tx_id: &str,
    verifier: &str,
    bond_micro: i64,
    parent: Hash,
    suffix: &str,
    verdict: VerifyVerdict,
    timestamp_logical: u64,
) -> TypedTx {
    TypedTx::Verify(VerifyTx {
        tx_id: TxId(format!("verifytx-{target_work_tx_id}-{suffix}")),
        parent_state_root: parent,
        target_work_tx: TxId(target_work_tx_id.into()),
        verifier_agent: AgentId(verifier.into()),
        bond: StakeMicroCoin::from_micro_units(bond_micro),
        verdict,
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical,
    })
}

/// Sequence: TaskOpen → EscrowLock → WorkTx accepted → state_root post-work.
async fn apply_task_funded_with_accepted_worktx(
    h: &mut Harness,
    task: &str,
    sponsor: &str,
    solver: &str,
    escrow_coin: i64,
    stake_coin: i64,
    suffix: &str,
) -> (TxId, Hash) {
    let pre = h.seq.q_snapshot().expect("snap").state_root_t;
    let open = make_task_open(task, sponsor, pre, suffix);
    h.seq.submit(open).await.expect("open submit");
    let _ = h.seq.try_apply_one(&mut h.rx).expect("open env").expect("open ok");

    let parent = h.seq.q_snapshot().expect("post-open").state_root_t;
    let lock = make_escrow_lock(task, sponsor, escrow_coin * 1_000_000, parent, suffix);
    h.seq.submit(lock).await.expect("lock submit");
    let _ = h.seq.try_apply_one(&mut h.rx).expect("lock env").expect("lock ok");

    let parent = h.seq.q_snapshot().expect("post-lock").state_root_t;
    let work = make_worktx(task, solver, parent, stake_coin * 1_000_000, suffix);
    let work_tx_id = match &work {
        TypedTx::Work(w) => w.tx_id.clone(),
        _ => unreachable!(),
    };
    h.seq.submit(work).await.expect("work submit");
    let _ = h.seq.try_apply_one(&mut h.rx).expect("work env").expect("work ok");

    let post = h.seq.q_snapshot().expect("post-work").state_root_t;
    (work_tx_id, post)
}

// ────────────────────────────────────────────────────────────────────────────
// Atom 1 — claims_t writer at VerifyTx OMEGA-Confirm acceptance.
// ────────────────────────────────────────────────────────────────────────────

/// I100 — VerifyTx{verdict=Confirm} on funded task creates exactly one
/// ClaimEntry with status=Open and amount = task's total_escrow.
#[tokio::test]
async fn omega_confirm_creates_claim_entry_with_total_escrow_amount() {
    let mut h = fresh_harness(genesis_with_balances(&[
        ("sponsor-i100", 100),
        ("solver-i100", 10),
        ("verifier-i100", 10),
    ]));
    let (work_tx_id, parent_after_work) = apply_task_funded_with_accepted_worktx(
        &mut h, "task-i100", "sponsor-i100", "solver-i100", 50, 3, "i100"
    ).await;

    // Pre-condition: claims_t empty.
    let pre = h.seq.q_snapshot().expect("snap");
    assert!(pre.economic_state_t.claims_t.0.is_empty(), "claims_t starts empty");

    // Submit OMEGA-Confirm VerifyTx.
    let verify = make_verify_tx(
        &work_tx_id.0, "verifier-i100", 2_000_000, parent_after_work,
        "i100", VerifyVerdict::Confirm, 7,
    );
    h.seq.submit(verify).await.expect("verify submit");
    let entry = h.seq.try_apply_one(&mut h.rx).expect("env").expect("verify accepted");
    assert_eq!(entry.tx_kind, TxKind::Verify);

    // Post-condition: exactly one ClaimEntry, amount = total_escrow (50 * 1_000_000),
    // claimant = solver, status = Open, challenge_window_close_logical_t == verify.ts.
    let post = h.seq.q_snapshot().expect("snap");
    assert_eq!(post.economic_state_t.claims_t.0.len(), 1, "exactly one claim created");
    let claim = post.economic_state_t.claims_t.0.values().next().unwrap();
    assert_eq!(claim.amount.micro_units(), 50 * 1_000_000, "claim.amount == total_escrow");
    assert_eq!(claim.claimant.0, "solver-i100", "claimant == work_tx solver");
    assert_eq!(claim.task_id.0, "task-i100");
    assert_eq!(claim.work_tx_id.0, work_tx_id.0);
    assert_eq!(claim.status, ClaimStatus::Open);
    assert_eq!(claim.challenge_window_close_logical_t, 0,
        "zero-window MVP: window field literally 0 (window-closed-immediately marker)");
}

/// I101 — VerifyTx{verdict=Doubt} does NOT create a ClaimEntry. The
/// Doubt verdict still locks the bond into stakes_t (existing TB-4
/// behavior preserved); only OMEGA-Confirm path triggers claim creation.
#[tokio::test]
async fn doubt_verdict_does_not_create_claim_entry() {
    let mut h = fresh_harness(genesis_with_balances(&[
        ("sponsor-i101", 100),
        ("solver-i101", 10),
        ("verifier-i101", 10),
    ]));
    let (work_tx_id, parent_after_work) = apply_task_funded_with_accepted_worktx(
        &mut h, "task-i101", "sponsor-i101", "solver-i101", 50, 3, "i101"
    ).await;

    let verify = make_verify_tx(
        &work_tx_id.0, "verifier-i101", 2_000_000, parent_after_work,
        "i101", VerifyVerdict::Doubt, 7,
    );
    h.seq.submit(verify).await.expect("verify submit");
    let _ = h.seq.try_apply_one(&mut h.rx).expect("env").expect("verify accepted");

    let post = h.seq.q_snapshot().expect("snap");
    assert!(post.economic_state_t.claims_t.0.is_empty(),
        "Doubt verdict must NOT create claim");
}

/// I102 — claim_id derivation is deterministic: claim_id == "claim-<verify.tx_id>"
/// per ratification §2.2. Replay invariant check.
#[tokio::test]
async fn claim_id_derivation_is_deterministic() {
    let mut h = fresh_harness(genesis_with_balances(&[
        ("sponsor-i102", 100),
        ("solver-i102", 10),
        ("verifier-i102", 10),
    ]));
    let (work_tx_id, parent_after_work) = apply_task_funded_with_accepted_worktx(
        &mut h, "task-i102", "sponsor-i102", "solver-i102", 50, 3, "i102"
    ).await;

    let verify = make_verify_tx(
        &work_tx_id.0, "verifier-i102", 2_000_000, parent_after_work,
        "i102", VerifyVerdict::Confirm, 7,
    );
    let verify_tx_id = match &verify {
        TypedTx::Verify(v) => v.tx_id.0.clone(),
        _ => unreachable!(),
    };
    h.seq.submit(verify).await.expect("submit");
    let _ = h.seq.try_apply_one(&mut h.rx).expect("env").expect("ok");

    let post = h.seq.q_snapshot().expect("snap");
    let expected_claim_id = format!("claim-{verify_tx_id}");
    assert!(
        post.economic_state_t.claims_t.0.contains_key(&TxId(expected_claim_id.clone())),
        "claim_id == 'claim-<verify.tx_id>' per ratification §2.2 (got keys: {:?})",
        post.economic_state_t.claims_t.0.keys().collect::<Vec<_>>()
    );
}

/// I103 — replay determinism: two fresh harnesses receiving the identical
/// tx sequence produce byte-identical claims_t snapshots.
#[tokio::test]
async fn claim_creation_is_replay_deterministic() {
    async fn run(suffix: &'static str) -> String {
        let mut h = fresh_harness(genesis_with_balances(&[
            ("sponsor-i103", 100),
            ("solver-i103", 10),
            ("verifier-i103", 10),
        ]));
        let (work_tx_id, parent) = apply_task_funded_with_accepted_worktx(
            &mut h, "task-i103", "sponsor-i103", "solver-i103", 50, 3, suffix
        ).await;
        let verify = make_verify_tx(
            &work_tx_id.0, "verifier-i103", 2_000_000, parent,
            suffix, VerifyVerdict::Confirm, 11,
        );
        h.seq.submit(verify).await.expect("submit");
        let _ = h.seq.try_apply_one(&mut h.rx).expect("env").expect("ok");
        let snap = h.seq.q_snapshot().expect("snap");
        serde_json::to_string(&snap.economic_state_t.claims_t).expect("serialize claims_t")
    }

    let a = run("i103a").await;
    let b = run("i103a").await;
    assert_eq!(a, b, "claims_t serialization must be byte-identical across replays");
}

// ────────────────────────────────────────────────────────────────────────────
// Atom 1 — _l4_count is observed: claim creation does NOT add an L4 row by
// itself; it's a Q-side write piggybacked on the VerifyTx accept.
// ────────────────────────────────────────────────────────────────────────────

/// I104 — claim creation piggybacks on VerifyTx accept (NOT a separate L4 row).
/// Ledger advances by 1 (the VerifyTx itself), Q gains 1 ClaimEntry.
#[tokio::test]
async fn claim_creation_piggybacks_on_verify_no_extra_l4_row() {
    let mut h = fresh_harness(genesis_with_balances(&[
        ("sponsor-i104", 100),
        ("solver-i104", 10),
        ("verifier-i104", 10),
    ]));
    let (work_tx_id, parent) = apply_task_funded_with_accepted_worktx(
        &mut h, "task-i104", "sponsor-i104", "solver-i104", 50, 3, "i104"
    ).await;
    let pre_l4 = h.ledger_writer.read().expect("r").len();

    let verify = make_verify_tx(
        &work_tx_id.0, "verifier-i104", 2_000_000, parent,
        "i104", VerifyVerdict::Confirm, 7,
    );
    h.seq.submit(verify).await.expect("submit");
    let _ = h.seq.try_apply_one(&mut h.rx).expect("env").expect("ok");
    let post_l4 = h.ledger_writer.read().expect("r").len();

    assert_eq!(post_l4, pre_l4 + 1, "exactly +1 L4 row (the VerifyTx)");
    let snap = h.seq.q_snapshot().expect("snap");
    assert_eq!(snap.economic_state_t.claims_t.0.len(), 1, "exactly +1 claim");
}

// ────────────────────────────────────────────────────────────────────────────
// Atom 2 — SystemEmitCommand::FinalizeReward ingress + emit_system_tx round-trip.
// ────────────────────────────────────────────────────────────────────────────

/// Helper: drive harness to an Open claim ready for finalize. Returns the
/// claim_id, the solver agent_id, and the expected reward (= total_escrow).
async fn drive_to_open_claim(
    h: &mut Harness,
    task: &str,
    sponsor: &str,
    solver: &str,
    verifier: &str,
    escrow_coin: i64,
    suffix: &str,
) -> (ClaimId, AgentId, MicroCoin) {
    let (work_tx_id, parent) = apply_task_funded_with_accepted_worktx(
        h, task, sponsor, solver, escrow_coin, 3, suffix
    ).await;
    let verify = make_verify_tx(
        &work_tx_id.0, verifier, 2_000_000, parent,
        suffix, VerifyVerdict::Confirm, 7,
    );
    let verify_tx_id = match &verify {
        TypedTx::Verify(v) => v.tx_id.0.clone(),
        _ => unreachable!(),
    };
    h.seq.submit(verify).await.expect("submit");
    let _ = h.seq.try_apply_one(&mut h.rx).expect("env").expect("ok");
    let claim_id = ClaimId(TxId(format!("claim-{verify_tx_id}")));
    let reward = MicroCoin::from_micro_units(escrow_coin * 1_000_000);
    (claim_id, AgentId(solver.into()), reward)
}

/// I110 — emit_system_tx FinalizeReward → apply_one accepts → canonical L4
/// has 1 FinalizeReward row + claim flips Open→Finalized.
#[tokio::test]
async fn emit_finalize_reward_round_trip_appends_to_l4_and_finalizes_claim() {
    let mut h = fresh_harness(genesis_with_balances(&[
        ("sponsor-i110", 100), ("solver-i110", 10), ("verifier-i110", 10),
    ]));
    let (claim_id, _solver, _reward) = drive_to_open_claim(
        &mut h, "task-i110", "sponsor-i110", "solver-i110", "verifier-i110", 50, "i110"
    ).await;

    let pre_l4 = h.ledger_writer.read().expect("r").len();
    h.seq.emit_system_tx(SystemEmitCommand::FinalizeReward {
        claim_id: claim_id.clone(),
    }).await.expect("emit ok");
    let entry = h.seq.try_apply_one(&mut h.rx).expect("env").expect("finalize accepted");
    assert_eq!(entry.tx_kind, TxKind::FinalizeReward);
    let post_l4 = h.ledger_writer.read().expect("r").len();
    assert_eq!(post_l4, pre_l4 + 1, "exactly +1 L4 row (the FinalizeRewardTx)");

    let post = h.seq.q_snapshot().expect("snap");
    let claim = post.economic_state_t.claims_t.0.get(claim_id.as_tx_id()).expect("claim still present");
    assert_eq!(claim.status, ClaimStatus::Finalized, "claim status flipped to Finalized");
}

/// I112 — emit_system_tx with unknown claim_id → ClaimNotFound (no L4 row).
#[tokio::test]
async fn emit_finalize_reward_with_unknown_claim_id_returns_claim_not_found() {
    let mut h = fresh_harness(genesis_with_balances(&[("a", 1)]));
    let r = h.seq.emit_system_tx(SystemEmitCommand::FinalizeReward {
        claim_id: ClaimId(TxId("claim-does-not-exist".into())),
    }).await;
    assert!(matches!(r, Err(turingosv4::state::sequencer::EmitSystemError::ClaimNotFound)),
        "expected ClaimNotFound, got {r:?}");
}

// ────────────────────────────────────────────────────────────────────────────
// Atom 3 — TypedTx::FinalizeReward dispatch arm + atomic mutation.
// ────────────────────────────────────────────────────────────────────────────

/// I115 — happy path: finalize debits escrow, credits solver, conserves CTF.
#[tokio::test]
async fn finalize_reward_happy_path_debits_escrow_credits_solver_conserves_ctf() {
    let mut h = fresh_harness(genesis_with_balances(&[
        ("sponsor-i115", 100), ("solver-i115", 10), ("verifier-i115", 10),
    ]));
    let (claim_id, solver, reward) = drive_to_open_claim(
        &mut h, "task-i115", "sponsor-i115", "solver-i115", "verifier-i115", 50, "i115"
    ).await;

    let pre = h.seq.q_snapshot().expect("snap");
    let pre_solver_bal = pre.economic_state_t.balances_t.0.get(&solver).copied()
        .unwrap_or_else(MicroCoin::zero);
    let pre_total: i64 = pre.economic_state_t.balances_t.0.values().map(|v| v.micro_units()).sum::<i64>()
        + pre.economic_state_t.escrows_t.0.values().map(|e| e.amount.micro_units()).sum::<i64>()
        + pre.economic_state_t.stakes_t.0.values().map(|e| e.amount.micro_units()).sum::<i64>()
        + pre.economic_state_t.challenge_cases_t.0.values().map(|c| c.bond.micro_units()).sum::<i64>();

    h.seq.emit_system_tx(SystemEmitCommand::FinalizeReward {
        claim_id: claim_id.clone(),
    }).await.expect("emit ok");
    let _ = h.seq.try_apply_one(&mut h.rx).expect("env").expect("ok");

    let post = h.seq.q_snapshot().expect("snap");
    let post_solver_bal = post.economic_state_t.balances_t.0.get(&solver).copied().unwrap();
    assert_eq!(
        post_solver_bal.micro_units(),
        pre_solver_bal.micro_units() + reward.micro_units(),
        "solver credited by reward"
    );

    let post_total: i64 = post.economic_state_t.balances_t.0.values().map(|v| v.micro_units()).sum::<i64>()
        + post.economic_state_t.escrows_t.0.values().map(|e| e.amount.micro_units()).sum::<i64>()
        + post.economic_state_t.stakes_t.0.values().map(|e| e.amount.micro_units()).sum::<i64>()
        + post.economic_state_t.challenge_cases_t.0.values().map(|c| c.bond.micro_units()).sum::<i64>();
    assert_eq!(post_total, pre_total, "CTF conserved across finalize");

    // Claim status flipped.
    let claim = post.economic_state_t.claims_t.0.get(claim_id.as_tx_id()).unwrap();
    assert_eq!(claim.status, ClaimStatus::Finalized);
}

/// I118 — re-finalize on Finalized claim → ClaimAlreadyFinalized + L4.E
/// (no second L4 row).
#[tokio::test]
async fn refinalize_on_finalized_claim_rejects_with_claim_already_finalized() {
    let mut h = fresh_harness(genesis_with_balances(&[
        ("sponsor-i118", 100), ("solver-i118", 10), ("verifier-i118", 10),
    ]));
    let (claim_id, _, _) = drive_to_open_claim(
        &mut h, "task-i118", "sponsor-i118", "solver-i118", "verifier-i118", 50, "i118"
    ).await;
    // First finalize: ok.
    h.seq.emit_system_tx(SystemEmitCommand::FinalizeReward {
        claim_id: claim_id.clone(),
    }).await.expect("emit 1");
    let _ = h.seq.try_apply_one(&mut h.rx).expect("env 1").expect("ok 1");
    let l4_after_first = h.ledger_writer.read().expect("r").len();

    // Second finalize: rejected with ClaimAlreadyFinalized; no L4 advance.
    h.seq.emit_system_tx(SystemEmitCommand::FinalizeReward {
        claim_id: claim_id.clone(),
    }).await.expect("emit 2");
    let r = h.seq.try_apply_one(&mut h.rx).expect("env 2");
    assert!(matches!(
        r,
        Err(turingosv4::state::sequencer::ApplyError::Transition(TransitionError::ClaimAlreadyFinalized))
    ), "expected ClaimAlreadyFinalized, got {r:?}");

    let l4_after_second = h.ledger_writer.read().expect("r").len();
    assert_eq!(l4_after_second, l4_after_first,
        "rejected re-finalize must NOT advance L4");
}

/// I119 — finalize blocked by UpheldDeferred challenge against the work_tx.
/// Requires seeding a challenge_cases_t entry with target_work_tx == claim.work_tx_id
/// and status = UpheldDeferred. Demonstrates the upheld-challenge gate.
#[tokio::test]
async fn finalize_blocked_when_challenge_upheld_against_work_tx() {
    let mut h = fresh_harness(genesis_with_balances(&[
        ("sponsor-i119", 100), ("solver-i119", 10), ("verifier-i119", 10),
    ]));
    let (claim_id, _, _) = drive_to_open_claim(
        &mut h, "task-i119", "sponsor-i119", "solver-i119", "verifier-i119", 50, "i119"
    ).await;

    // Read claim's work_tx_id then seed an UpheldDeferred challenge against it.
    // We seed via direct Q mutation through `seq.q_snapshot` clone + a new harness
    // would be cleaner, but for this test we exercise the gate by reflecting the
    // upheld state into a fresh sequencer state.
    let snap = h.seq.q_snapshot().expect("snap");
    let claim = snap.economic_state_t.claims_t.0.get(claim_id.as_tx_id()).unwrap().clone();

    // Build a fresh harness whose initial Q already has the matching claim
    // + an UpheldDeferred challenge against the claim's work_tx_id.
    let mut q = QState::genesis();
    q.economic_state_t.balances_t.0.insert(
        AgentId("sponsor-i119".into()), MicroCoin::from_coin(100).unwrap(),
    );
    // Seed the matching escrow (so the intent-vs-backing invariant passes).
    q.economic_state_t.escrows_t.0.insert(
        claim.escrow_lock_tx_id.clone(),
        turingosv4::state::q_state::EscrowEntry {
            amount: claim.amount,
            depositor: AgentId("sponsor-i119".into()),
            task_id: claim.task_id.clone(),
        },
    );
    let mut tm = turingosv4::state::q_state::TaskMarketEntry::default();
    tm.total_escrow = claim.amount;
    tm.escrow_lock_tx_ids.insert(claim.escrow_lock_tx_id.clone());
    q.economic_state_t.task_markets_t.0.insert(claim.task_id.clone(), tm);
    q.economic_state_t.claims_t.0.insert(
        claim_id.as_tx_id().clone(),
        claim.clone(),
    );
    q.economic_state_t.challenge_cases_t.0.insert(
        TxId("ct-upheld-i119".into()),
        turingosv4::state::q_state::ChallengeCase {
            challenger: AgentId("ch-i119".into()),
            bond: MicroCoin::from_micro_units(1_000_000),
            opened_at_round: 1,
            target_work_tx: claim.work_tx_id.clone(),
            status: turingosv4::state::q_state::ChallengeStatus::UpheldDeferred,
        },
    );

    let mut h2 = fresh_harness(q);
    h2.seq.emit_system_tx(SystemEmitCommand::FinalizeReward {
        claim_id: claim_id.clone(),
    }).await.expect("emit ok");
    let r = h2.seq.try_apply_one(&mut h2.rx).expect("env");
    // Upheld-challenge gate should reject with SettlementPredicateFailed
    // bearing the predicate "challenge_window_closed_with_no_upheld_challenge".
    assert!(matches!(
        r,
        Err(turingosv4::state::sequencer::ApplyError::Transition(
            TransitionError::SettlementPredicateFailed(_)
        ))
    ), "upheld-challenge must block finalize, got {r:?}");
}

/// I121 — agent cannot submit a forged FinalizeRewardTx through the agent
/// ingress path. Anti-Oreo barrier (TB-5 Atom 2 precedent: rejection at
/// submit_agent_tx pre-queue with SystemTxForbiddenOnAgentIngress).
#[tokio::test]
async fn agent_cannot_submit_finalize_reward_through_agent_ingress() {
    let h = fresh_harness(QState::genesis());
    let forged = TypedTx::FinalizeReward(turingosv4::state::typed_tx::FinalizeRewardTx::default());
    let r = h.seq.submit(forged).await;
    assert!(matches!(
        r,
        Err(turingosv4::state::sequencer::SubmitError::SystemTxForbiddenOnAgentIngress)
    ), "agent ingress must reject FinalizeReward, got {r:?}");
}

// ────────────────────────────────────────────────────────────────────────────
// Atom 1 round-2 (Codex VETO RQ4 fix) — one-claim-per-work_tx_id idempotency.
// ────────────────────────────────────────────────────────────────────────────

/// I130 — A second Confirm VerifyTx targeting the SAME WorkTx must NOT
/// create a second claim row. The duplicate Confirm is itself accepted on
/// L4 (its bond locks; verdict rides L4) but the claim writer suppresses
/// the second claim creation. Without this gate, two Open claims could
/// share one escrow row, making post-finalize backing assertions fail
/// (denial-of-payout attack).
#[tokio::test]
async fn duplicate_confirm_verify_does_not_create_second_claim() {
    let mut h = fresh_harness(genesis_with_balances(&[
        ("sponsor-i130", 100),
        ("solver-i130", 10),
        ("verifier-A-i130", 10),
        ("verifier-B-i130", 10),
    ]));
    let (work_tx_id, parent) = apply_task_funded_with_accepted_worktx(
        &mut h, "task-i130", "sponsor-i130", "solver-i130", 50, 3, "i130"
    ).await;

    // First Confirm by verifier-A — creates one Open claim.
    let v1 = make_verify_tx(
        &work_tx_id.0, "verifier-A-i130", 2_000_000, parent,
        "i130-A", VerifyVerdict::Confirm, 7,
    );
    h.seq.submit(v1).await.expect("v1 submit");
    let _ = h.seq.try_apply_one(&mut h.rx).expect("v1 env").expect("v1 ok");
    let after_v1 = h.seq.q_snapshot().expect("snap1");
    assert_eq!(after_v1.economic_state_t.claims_t.0.len(), 1,
        "first Confirm creates one claim");

    // Second Confirm by verifier-B targeting the SAME WorkTx — accepted on
    // L4 (bond locks) but claim writer must suppress.
    let parent2 = after_v1.state_root_t;
    let v2 = make_verify_tx(
        &work_tx_id.0, "verifier-B-i130", 2_000_000, parent2,
        "i130-B", VerifyVerdict::Confirm, 8,
    );
    h.seq.submit(v2).await.expect("v2 submit");
    let r = h.seq.try_apply_one(&mut h.rx).expect("v2 env");
    assert!(r.is_ok(), "second VerifyTx itself accepts (bond locks); only claim creation is suppressed");

    let after_v2 = h.seq.q_snapshot().expect("snap2");
    assert_eq!(
        after_v2.economic_state_t.claims_t.0.len(),
        1,
        "duplicate Confirm against same work_tx_id MUST NOT create a second claim row"
    );

    // Aggregate Open-claim amount per backing escrow remains within capacity.
    let total_open: i64 = after_v2
        .economic_state_t
        .claims_t
        .0
        .values()
        .filter(|c| c.status == ClaimStatus::Open)
        .map(|c| c.amount.micro_units())
        .sum();
    let backing_escrow: i64 = after_v2
        .economic_state_t
        .escrows_t
        .0
        .values()
        .map(|e| e.amount.micro_units())
        .sum();
    assert!(total_open <= backing_escrow,
        "aggregate Open-claim amount must not exceed backing escrow");
}

/// I131 — even with the duplicate-Confirm idempotency in place, the
/// finalize succeeds (the OMEGA emit derives the canonical claim_id from
/// the FIRST verify; second Confirm doesn't add a new claim row, so no
/// orphan claim blocks finalize).
#[tokio::test]
async fn finalize_succeeds_after_duplicate_confirm_attempt() {
    let mut h = fresh_harness(genesis_with_balances(&[
        ("sponsor-i131", 100),
        ("solver-i131", 10),
        ("verifier-A-i131", 10),
        ("verifier-B-i131", 10),
    ]));
    let (work_tx_id, parent) = apply_task_funded_with_accepted_worktx(
        &mut h, "task-i131", "sponsor-i131", "solver-i131", 50, 3, "i131"
    ).await;

    let v1 = make_verify_tx(
        &work_tx_id.0, "verifier-A-i131", 2_000_000, parent,
        "i131-A", VerifyVerdict::Confirm, 7,
    );
    let v1_id = match &v1 { TypedTx::Verify(v) => v.tx_id.0.clone(), _ => unreachable!() };
    h.seq.submit(v1).await.expect("submit v1");
    let _ = h.seq.try_apply_one(&mut h.rx).expect("env v1").expect("ok v1");

    let parent2 = h.seq.q_snapshot().expect("s").state_root_t;
    let v2 = make_verify_tx(
        &work_tx_id.0, "verifier-B-i131", 2_000_000, parent2,
        "i131-B", VerifyVerdict::Confirm, 8,
    );
    h.seq.submit(v2).await.expect("submit v2");
    let _ = h.seq.try_apply_one(&mut h.rx).expect("env v2").expect("ok v2");

    let claim_id = ClaimId(TxId(format!("claim-{v1_id}")));
    h.seq.emit_system_tx(SystemEmitCommand::FinalizeReward { claim_id: claim_id.clone() })
        .await.expect("emit ok");
    let r = h.seq.try_apply_one(&mut h.rx).expect("env finalize").expect("finalize accepts");
    assert_eq!(r.tx_kind, TxKind::FinalizeReward);

    let post = h.seq.q_snapshot().expect("snap");
    let claim = post.economic_state_t.claims_t.0.get(claim_id.as_tx_id()).unwrap();
    assert_eq!(claim.status, ClaimStatus::Finalized);
}
