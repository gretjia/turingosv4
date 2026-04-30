//! TB-4 RSP-2 admission-surface — integration tests through `Sequencer::submit`.
//!
//! Charter: `handover/tracer_bullets/TB-4_charter_2026-04-30.md` DRAFT v2.
//! Preflight: `handover/ai-direct/TB-4_RSP2_ADMISSION_SURFACE_2026-04-30.md`.
//!
//! Per charter § 4.7 + preflight § 7.1, this file holds I31-I40 + I43 + I44.
//! Every test goes through the public `Sequencer::submit` path. L4.E rows are
//! observed via the constructor-injected `Arc<RwLock<RejectionEvidenceWriter>>`
//! clone the test retains.
//!
//! Atom 4 covers Verify-side: I31, I33, I35, I37.
//! Atom 5 covers Challenge-side: I32, I34, I36, I38.
//! Atom 6 covers multi-challenger + window-anchor + L4.E-no-mutation: I39, I40, I43.
//! Atom 7 covers anti-drift CI: I44.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, RwLock};

use tempfile::TempDir;

use turingosv4::bottom_white::cas::schema::Cid;
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::bottom_white::ledger::rejection_evidence::{
    RejectionClass as L4ERejectionClass, RejectionEvidenceWriter,
};
use turingosv4::bottom_white::ledger::system_keypair::{Ed25519Keypair, SystemEpoch};
use turingosv4::bottom_white::ledger::transition_ledger::{
    InMemoryLedgerWriter, LedgerWriter, TxKind,
};
use turingosv4::bottom_white::tools::registry::ToolRegistry;
use turingosv4::economy::money::{MicroCoin, StakeMicroCoin};
use turingosv4::state::q_state::{AgentId, Hash, QState, TaskId, TxId};
use turingosv4::state::sequencer::{
    challenge_accept_state_root, escrow_lock_accept_state_root, task_open_accept_state_root,
    verify_accept_state_root, Sequencer, SubmissionEnvelope,
};
use turingosv4::state::typed_tx::{
    AgentSignature, BoolWithProof, ChallengeTx, EscrowLockTx, PredicateId, PredicateResultsBundle,
    ReadKey, SafetyOrCreation, TaskOpenTx, TypedTx, VerifyTx, VerifyVerdict, WorkTx, WriteKey,
};
use turingosv4::top_white::predicates::registry::PredicateRegistry;

// ────────────────────────────────────────────────────────────────────────────
// Harness (mirrors tests/tb_3_rsp1_formal_surface.rs)
// ────────────────────────────────────────────────────────────────────────────

struct Harness {
    _tmp: TempDir,
    seq: Sequencer,
    rx: tokio::sync::mpsc::Receiver<SubmissionEnvelope>,
    rejection_writer: Arc<RwLock<RejectionEvidenceWriter>>,
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
    let (seq, rx) = Sequencer::new(
        cas.clone(),
        keypair,
        epoch,
        writer.clone(),
        rejection_writer.clone(),
        preds,
        tools,
        initial_q,
        16,
    );
    Harness {
        _tmp: tmp,
        seq,
        rx,
        rejection_writer,
        ledger_writer: writer,
    }
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

fn make_worktx(
    task: &str,
    agent: &str,
    parent: Hash,
    stake_micro: i64,
    suffix: &str,
) -> TypedTx {
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
) -> TypedTx {
    TypedTx::Verify(VerifyTx {
        tx_id: TxId(format!("verifytx-{target_work_tx_id}-{suffix}")),
        parent_state_root: parent,
        target_work_tx: TxId(target_work_tx_id.into()),
        verifier_agent: AgentId(verifier.into()),
        bond: StakeMicroCoin::from_micro_units(bond_micro),
        verdict: VerifyVerdict::Confirm,
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: 1,
    })
}

fn make_challenge_tx(
    target_work_tx_id: &str,
    challenger: &str,
    stake_micro: i64,
    counterexample: Cid,
    parent: Hash,
    suffix: &str,
) -> TypedTx {
    TypedTx::Challenge(ChallengeTx {
        tx_id: TxId(format!("challengetx-{target_work_tx_id}-{suffix}")),
        parent_state_root: parent,
        target_work_tx: TxId(target_work_tx_id.into()),
        challenger_agent: AgentId(challenger.into()),
        stake: StakeMicroCoin::from_micro_units(stake_micro),
        counterexample_cid: counterexample,
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: 1,
    })
}

/// Apply TaskOpen → EscrowLock → WorkTx via Sequencer::submit so the canonical
/// L4 has the work tx accepted and `stakes_t` carries the target's YES stake
/// (the liveness anchor TB-4 Verify/Challenge admission relies on).
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
    let _ = h.seq.try_apply_one(&mut h.rx).expect("open env").expect("open accepted");

    let parent = h.seq.q_snapshot().expect("post-open").state_root_t;
    let lock = make_escrow_lock(task, sponsor, escrow_coin * 1_000_000, parent, suffix);
    h.seq.submit(lock).await.expect("lock submit");
    let _ = h.seq.try_apply_one(&mut h.rx).expect("lock env").expect("lock accepted");

    let parent = h.seq.q_snapshot().expect("post-lock").state_root_t;
    let work = make_worktx(task, solver, parent, stake_coin * 1_000_000, suffix);
    let work_tx_id = match &work {
        TypedTx::Work(w) => w.tx_id.clone(),
        _ => unreachable!(),
    };
    h.seq.submit(work).await.expect("work submit");
    let _ = h.seq.try_apply_one(&mut h.rx).expect("work env").expect("work accepted");

    let post = h.seq.q_snapshot().expect("post-work").state_root_t;
    (work_tx_id, post)
}

fn last_l4e_class(writer: &Arc<RwLock<RejectionEvidenceWriter>>) -> Option<L4ERejectionClass> {
    let g = writer.read().expect("writer read");
    g.records().last().map(|r| r.rejection_class)
}

fn l4e_row_count(writer: &Arc<RwLock<RejectionEvidenceWriter>>) -> usize {
    writer.read().expect("writer read").records().len()
}

fn l4_row_count(writer: &Arc<RwLock<dyn LedgerWriter>>) -> u64 {
    writer.read().expect("writer read").len()
}

// ────────────────────────────────────────────────────────────────────────────
// I31 — VerifyTx submitted through Sequencer::submit appends to canonical L4
// ────────────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn submit_verify_tx_appends_to_canonical_l4_and_locks_bond() {
    let mut h = fresh_harness(genesis_with_balances(&[
        ("sponsor-i31", 100),
        ("solver-i31", 10),
        ("verifier-i31", 10),
    ]));

    // Build live target via TaskOpen → EscrowLock → WorkTx accepted.
    let (target_work_tx_id, parent_after_work) = apply_task_funded_with_accepted_worktx(
        &mut h, "task-i31", "sponsor-i31", "solver-i31", 50, 3, "i31"
    ).await;

    let pre_l4 = l4_row_count(&h.ledger_writer);
    let pre_l4e = l4e_row_count(&h.rejection_writer);

    let verify_tx = make_verify_tx(
        &target_work_tx_id.0, "verifier-i31", 2_000_000, parent_after_work, "i31"
    );
    h.seq.submit(verify_tx.clone()).await.expect("verify submit");
    let drained = h.seq.try_apply_one(&mut h.rx).expect("verify env");
    assert!(drained.is_ok(), "VerifyTx with positive bond + live target + solvent verifier must accept; got {:?}", drained);

    // Charter § 8 Proof 1: 1 new L4 row, zero L4.E.
    assert_eq!(l4_row_count(&h.ledger_writer), pre_l4 + 1);
    assert_eq!(l4e_row_count(&h.rejection_writer), pre_l4e);

    let entry = drained.expect("entry");
    assert_eq!(entry.tx_kind, TxKind::Verify);
}

// ────────────────────────────────────────────────────────────────────────────
// I33 — Verify admission is atomic balance → stakes_t transfer
// ────────────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn verify_admission_atomic_balance_to_stakes_transfer() {
    let mut h = fresh_harness(genesis_with_balances(&[
        ("sponsor-i33", 100),
        ("solver-i33", 10),
        ("verifier-i33", 5),
    ]));

    let (target_work_tx_id, parent_after_work) = apply_task_funded_with_accepted_worktx(
        &mut h, "task-i33", "sponsor-i33", "solver-i33", 50, 3, "i33"
    ).await;

    let pre = h.seq.q_snapshot().expect("pre-verify");
    let pre_verifier_bal = pre.economic_state_t.balances_t.0
        .get(&AgentId("verifier-i33".into())).copied().unwrap();
    assert_eq!(pre_verifier_bal.micro_units(), 5_000_000);

    let verify_tx = make_verify_tx(
        &target_work_tx_id.0, "verifier-i33", 2_000_000, parent_after_work, "i33"
    );
    let verify_tx_id = match &verify_tx {
        TypedTx::Verify(v) => v.tx_id.clone(),
        _ => unreachable!(),
    };
    h.seq.submit(verify_tx).await.expect("submit");
    let _ = h.seq.try_apply_one(&mut h.rx).expect("env").expect("accepted");

    let post = h.seq.q_snapshot().expect("post-verify");
    let post_verifier_bal = post.economic_state_t.balances_t.0
        .get(&AgentId("verifier-i33".into())).copied().unwrap();
    assert_eq!(post_verifier_bal.micro_units(), 5_000_000 - 2_000_000,
               "verifier balance debited by bond amount");

    // stakes_t entry created at verify_tx_id with task_id binding.
    let stake_entry = post.economic_state_t.stakes_t.0
        .get(&verify_tx_id)
        .expect("stakes_t entry at verify.tx_id");
    assert_eq!(stake_entry.amount.micro_units(), 2_000_000);
    assert_eq!(stake_entry.staker, AgentId("verifier-i33".into()));
    assert_eq!(stake_entry.task_id, TaskId("task-i33".into()),
               "task_id binding inherited from target's stakes_t entry (charter § 3.4)");

    // CTF conserved (debit balance = credit stakes).
    let pre_total: i64 = pre.economic_state_t.balances_t.0.values().map(|v| v.micro_units()).sum::<i64>()
        + pre.economic_state_t.stakes_t.0.values().map(|e| e.amount.micro_units()).sum::<i64>()
        + pre.economic_state_t.escrows_t.0.values().map(|e| e.amount.micro_units()).sum::<i64>();
    let post_total: i64 = post.economic_state_t.balances_t.0.values().map(|v| v.micro_units()).sum::<i64>()
        + post.economic_state_t.stakes_t.0.values().map(|e| e.amount.micro_units()).sum::<i64>()
        + post.economic_state_t.escrows_t.0.values().map(|e| e.amount.micro_units()).sum::<i64>();
    assert_eq!(pre_total, post_total, "CTF conserved across Verify accept");

    // state_root advanced via VERIFY_ACCEPT_DOMAIN_V1.
    let expected = verify_accept_state_root(&parent_after_work, &make_verify_tx(
        &target_work_tx_id.0, "verifier-i33", 2_000_000, parent_after_work, "i33"
    ));
    assert_eq!(post.state_root_t, expected);
}

// ────────────────────────────────────────────────────────────────────────────
// I35 — Verify against a target NOT in stakes_t routes to L4.E TargetWorkInactive
// ────────────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn verify_against_inactive_target_appends_l4e_target_inactive() {
    let mut h = fresh_harness(genesis_with_balances(&[
        ("verifier-i35", 10),
    ]));

    // No TaskOpen / EscrowLock / WorkTx — so stakes_t is empty.
    let pre_l4 = l4_row_count(&h.ledger_writer);
    let pre_l4e = l4e_row_count(&h.rejection_writer);

    let parent = h.seq.q_snapshot().expect("snap").state_root_t;
    let verify_tx = make_verify_tx("nonexistent-work-tx", "verifier-i35", 2_000_000, parent, "i35");
    h.seq.submit(verify_tx).await.expect("submit");
    let drained = h.seq.try_apply_one(&mut h.rx).expect("env");
    assert!(drained.is_err(), "Verify against inactive target must reject");

    // No L4 row, exactly 1 L4.E row.
    assert_eq!(l4_row_count(&h.ledger_writer), pre_l4);
    assert_eq!(l4e_row_count(&h.rejection_writer), pre_l4e + 1);
    // L4ERejectionClass is PolicyViolation (charter § 4.5; finer-grained
    // TargetWorkInactive recoverable from raw_diagnostic_cid CAS payload).
    assert_eq!(last_l4e_class(&h.rejection_writer), Some(L4ERejectionClass::PolicyViolation));

    // L4.E does NOT mutate economic_state (charter § 5 #10 inherited).
    let q_after = h.seq.q_snapshot().expect("snap after reject");
    let bal_after = q_after.economic_state_t.balances_t.0
        .get(&AgentId("verifier-i35".into())).copied().unwrap();
    assert_eq!(bal_after.micro_units(), 10_000_000, "L4.E never mutates balances_t");
    assert!(q_after.economic_state_t.stakes_t.0.is_empty(),
            "L4.E never mutates stakes_t");
}

// ────────────────────────────────────────────────────────────────────────────
// I37 — Verify with bond.micro_units() == 0 routes to L4.E BondInsufficient
// ────────────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn verify_with_zero_bond_appends_l4e_bond_insufficient() {
    let mut h = fresh_harness(genesis_with_balances(&[
        ("sponsor-i37", 100),
        ("solver-i37", 10),
        ("verifier-i37", 10),
    ]));

    let (target_work_tx_id, parent_after_work) = apply_task_funded_with_accepted_worktx(
        &mut h, "task-i37", "sponsor-i37", "solver-i37", 50, 3, "i37"
    ).await;

    let pre_l4 = l4_row_count(&h.ledger_writer);
    let pre_l4e = l4e_row_count(&h.rejection_writer);

    let verify_tx = make_verify_tx(&target_work_tx_id.0, "verifier-i37", 0, parent_after_work, "i37");
    h.seq.submit(verify_tx).await.expect("submit");
    let drained = h.seq.try_apply_one(&mut h.rx).expect("env");
    assert!(drained.is_err(), "Verify with zero bond must reject");

    assert_eq!(l4_row_count(&h.ledger_writer), pre_l4);
    assert_eq!(l4e_row_count(&h.rejection_writer), pre_l4e + 1);
    assert_eq!(last_l4e_class(&h.rejection_writer), Some(L4ERejectionClass::PolicyViolation));

    // Verifier balance untouched.
    let q_after = h.seq.q_snapshot().expect("snap");
    let bal_after = q_after.economic_state_t.balances_t.0
        .get(&AgentId("verifier-i37".into())).copied().unwrap();
    assert_eq!(bal_after.micro_units(), 10_000_000);
}

// ────────────────────────────────────────────────────────────────────────────
// I32 — ChallengeTx submitted through Sequencer::submit appends to canonical L4
// ────────────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn submit_challenge_tx_appends_to_canonical_l4_and_opens_case() {
    let mut h = fresh_harness(genesis_with_balances(&[
        ("sponsor-i32", 100),
        ("solver-i32", 10),
        ("challenger-i32", 10),
    ]));

    let (target_work_tx_id, parent_after_work) = apply_task_funded_with_accepted_worktx(
        &mut h, "task-i32", "sponsor-i32", "solver-i32", 50, 3, "i32"
    ).await;

    let pre_l4 = l4_row_count(&h.ledger_writer);
    let pre_l4e = l4e_row_count(&h.rejection_writer);

    let counterex = Cid([0xABu8; 32]);
    let chal_tx = make_challenge_tx(
        &target_work_tx_id.0, "challenger-i32", 4_000_000, counterex, parent_after_work, "i32"
    );
    let chal_tx_id = match &chal_tx {
        TypedTx::Challenge(c) => c.tx_id.clone(),
        _ => unreachable!(),
    };
    h.seq.submit(chal_tx).await.expect("challenge submit");
    let drained = h.seq.try_apply_one(&mut h.rx).expect("challenge env");
    assert!(drained.is_ok(), "ChallengeTx must accept; got {:?}", drained);

    assert_eq!(l4_row_count(&h.ledger_writer), pre_l4 + 1);
    assert_eq!(l4e_row_count(&h.rejection_writer), pre_l4e);

    let entry = drained.expect("entry");
    assert_eq!(entry.tx_kind, TxKind::Challenge);

    // ChallengeCase row inserted with target back-ref.
    let q_after = h.seq.q_snapshot().expect("snap");
    let case = q_after.economic_state_t.challenge_cases_t.0
        .get(&chal_tx_id)
        .expect("ChallengeCase at challenge.tx_id");
    assert_eq!(case.target_work_tx, target_work_tx_id);
    assert_eq!(case.bond.micro_units(), 4_000_000);
}

// ────────────────────────────────────────────────────────────────────────────
// I34 — Challenge admission is atomic balance → challenge_cases_t transfer
// ────────────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn challenge_admission_atomic_balance_to_challenge_cases_transfer() {
    let mut h = fresh_harness(genesis_with_balances(&[
        ("sponsor-i34", 100),
        ("solver-i34", 10),
        ("challenger-i34", 8),
    ]));

    let (target_work_tx_id, parent_after_work) = apply_task_funded_with_accepted_worktx(
        &mut h, "task-i34", "sponsor-i34", "solver-i34", 50, 3, "i34"
    ).await;

    let pre = h.seq.q_snapshot().expect("pre");
    let pre_bal = pre.economic_state_t.balances_t.0
        .get(&AgentId("challenger-i34".into())).copied().unwrap();
    assert_eq!(pre_bal.micro_units(), 8_000_000);

    let counterex = Cid([0xBBu8; 32]);
    let chal_tx = make_challenge_tx(
        &target_work_tx_id.0, "challenger-i34", 3_000_000, counterex, parent_after_work, "i34"
    );
    let chal_tx_id = match &chal_tx {
        TypedTx::Challenge(c) => c.tx_id.clone(),
        _ => unreachable!(),
    };
    h.seq.submit(chal_tx).await.expect("submit");
    let _ = h.seq.try_apply_one(&mut h.rx).expect("env").expect("accepted");

    let post = h.seq.q_snapshot().expect("post");
    let post_bal = post.economic_state_t.balances_t.0
        .get(&AgentId("challenger-i34".into())).copied().unwrap();
    assert_eq!(post_bal.micro_units(), 8_000_000 - 3_000_000,
               "challenger balance debited by stake amount");

    let case = post.economic_state_t.challenge_cases_t.0
        .get(&chal_tx_id)
        .expect("ChallengeCase");
    assert_eq!(case.bond.micro_units(), 3_000_000);
    assert_eq!(case.challenger, AgentId("challenger-i34".into()));
    assert_eq!(case.target_work_tx, target_work_tx_id);

    // CTF conserved (debit balance = credit challenge_cases.bond).
    let pre_total: i64 = pre.economic_state_t.balances_t.0.values().map(|v| v.micro_units()).sum::<i64>()
        + pre.economic_state_t.stakes_t.0.values().map(|e| e.amount.micro_units()).sum::<i64>()
        + pre.economic_state_t.escrows_t.0.values().map(|e| e.amount.micro_units()).sum::<i64>()
        + pre.economic_state_t.challenge_cases_t.0.values().map(|e| e.bond.micro_units()).sum::<i64>();
    let post_total: i64 = post.economic_state_t.balances_t.0.values().map(|v| v.micro_units()).sum::<i64>()
        + post.economic_state_t.stakes_t.0.values().map(|e| e.amount.micro_units()).sum::<i64>()
        + post.economic_state_t.escrows_t.0.values().map(|e| e.amount.micro_units()).sum::<i64>()
        + post.economic_state_t.challenge_cases_t.0.values().map(|e| e.bond.micro_units()).sum::<i64>();
    assert_eq!(pre_total, post_total, "CTF conserved across Challenge accept");

    // state_root advanced via CHALLENGE_ACCEPT_DOMAIN_V1.
    let expected = challenge_accept_state_root(&parent_after_work, &make_challenge_tx(
        &target_work_tx_id.0, "challenger-i34", 3_000_000, counterex, parent_after_work, "i34"
    ));
    assert_eq!(post.state_root_t, expected);
}

// ────────────────────────────────────────────────────────────────────────────
// I36 — Challenge against a target NOT in stakes_t routes to L4.E
// ────────────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn challenge_against_inactive_target_appends_l4e_target_inactive() {
    let mut h = fresh_harness(genesis_with_balances(&[
        ("challenger-i36", 10),
    ]));

    let pre_l4 = l4_row_count(&h.ledger_writer);
    let pre_l4e = l4e_row_count(&h.rejection_writer);

    let parent = h.seq.q_snapshot().expect("snap").state_root_t;
    let chal_tx = make_challenge_tx(
        "nonexistent-work-tx", "challenger-i36", 2_000_000,
        Cid([0xCCu8; 32]), parent, "i36"
    );
    h.seq.submit(chal_tx).await.expect("submit");
    let drained = h.seq.try_apply_one(&mut h.rx).expect("env");
    assert!(drained.is_err());

    assert_eq!(l4_row_count(&h.ledger_writer), pre_l4);
    assert_eq!(l4e_row_count(&h.rejection_writer), pre_l4e + 1);
    assert_eq!(last_l4e_class(&h.rejection_writer), Some(L4ERejectionClass::PolicyViolation));

    // L4.E does NOT mutate economic_state.
    let q_after = h.seq.q_snapshot().expect("snap");
    let bal_after = q_after.economic_state_t.balances_t.0
        .get(&AgentId("challenger-i36".into())).copied().unwrap();
    assert_eq!(bal_after.micro_units(), 10_000_000, "L4.E never mutates balances_t");
    assert!(q_after.economic_state_t.challenge_cases_t.0.is_empty(),
            "L4.E never mutates challenge_cases_t");
}

// ────────────────────────────────────────────────────────────────────────────
// I38 — Challenge with stake.micro_units() == 0 routes to L4.E StakeInsufficient
// ────────────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn challenge_with_zero_stake_appends_l4e_stake_insufficient() {
    let mut h = fresh_harness(genesis_with_balances(&[
        ("sponsor-i38", 100),
        ("solver-i38", 10),
        ("challenger-i38", 10),
    ]));

    let (target_work_tx_id, parent_after_work) = apply_task_funded_with_accepted_worktx(
        &mut h, "task-i38", "sponsor-i38", "solver-i38", 50, 3, "i38"
    ).await;

    let pre_l4 = l4_row_count(&h.ledger_writer);
    let pre_l4e = l4e_row_count(&h.rejection_writer);

    let chal_tx = make_challenge_tx(
        &target_work_tx_id.0, "challenger-i38", 0,
        Cid([0xDDu8; 32]), parent_after_work, "i38"
    );
    h.seq.submit(chal_tx).await.expect("submit");
    let drained = h.seq.try_apply_one(&mut h.rx).expect("env");
    assert!(drained.is_err());

    assert_eq!(l4_row_count(&h.ledger_writer), pre_l4);
    assert_eq!(l4e_row_count(&h.rejection_writer), pre_l4e + 1);
    assert_eq!(last_l4e_class(&h.rejection_writer), Some(L4ERejectionClass::PolicyViolation));

    // Challenger balance untouched.
    let q_after = h.seq.q_snapshot().expect("snap");
    let bal_after = q_after.economic_state_t.balances_t.0
        .get(&AgentId("challenger-i38".into())).copied().unwrap();
    assert_eq!(bal_after.micro_units(), 10_000_000);
}

// ────────────────────────────────────────────────────────────────────────────
// I39 — Multi-challenger representability (DIRECTIVE Q4 BINDING)
// Two distinct challengers submit ChallengeTx against the same target
// work_tx; both accept; challenge_cases_t carries 2 distinct rows; the
// target work_tx's stakes_t entry is unchanged. Pinpoint test against any
// future creep toward "single challenge per target" semantics.
// ────────────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn multiple_challengers_same_target_all_accepted_distinct_case_rows() {
    let mut h = fresh_harness(genesis_with_balances(&[
        ("sponsor-i39", 100),
        ("solver-i39", 10),
        ("challenger-a-i39", 10),
        ("challenger-b-i39", 10),
    ]));

    let (target_work_tx_id, parent_after_work) = apply_task_funded_with_accepted_worktx(
        &mut h, "task-i39", "sponsor-i39", "solver-i39", 50, 3, "i39"
    ).await;

    // Snapshot pre-challenges to compare target's stakes_t entry afterwards.
    let pre = h.seq.q_snapshot().expect("pre");
    let pre_target_stake = pre.economic_state_t.stakes_t.0
        .get(&target_work_tx_id).cloned()
        .expect("target stakes_t entry must exist");

    // Challenger A submits first.
    let chal_a = make_challenge_tx(
        &target_work_tx_id.0, "challenger-a-i39", 2_000_000,
        Cid([0xAAu8; 32]), parent_after_work, "i39-A"
    );
    let chal_a_id = match &chal_a {
        TypedTx::Challenge(c) => c.tx_id.clone(),
        _ => unreachable!(),
    };
    h.seq.submit(chal_a).await.expect("submit A");
    let _ = h.seq.try_apply_one(&mut h.rx).expect("env A").expect("A accepted");

    // Get the new state root after A.
    let after_a = h.seq.q_snapshot().expect("after A").state_root_t;

    // Challenger B submits — DIFFERENT challenger, SAME target.
    let chal_b = make_challenge_tx(
        &target_work_tx_id.0, "challenger-b-i39", 3_000_000,
        Cid([0xBBu8; 32]), after_a, "i39-B"
    );
    let chal_b_id = match &chal_b {
        TypedTx::Challenge(c) => c.tx_id.clone(),
        _ => unreachable!(),
    };
    h.seq.submit(chal_b).await.expect("submit B");
    let _ = h.seq.try_apply_one(&mut h.rx).expect("env B").expect("B accepted");

    // Both ChallengeCase rows present, distinct keys (challenge.tx_id).
    let post = h.seq.q_snapshot().expect("post");
    assert_eq!(post.economic_state_t.challenge_cases_t.0.len(), 2,
               "two challenges → two challenge_cases_t rows");
    let case_a = post.economic_state_t.challenge_cases_t.0.get(&chal_a_id).expect("A row");
    let case_b = post.economic_state_t.challenge_cases_t.0.get(&chal_b_id).expect("B row");

    // Same target, different challengers, distinct bonds.
    assert_eq!(case_a.target_work_tx, target_work_tx_id);
    assert_eq!(case_b.target_work_tx, target_work_tx_id,
               "DIRECTIVE Q4: two challenges against same target_work_tx must coexist");
    assert_eq!(case_a.challenger, AgentId("challenger-a-i39".into()));
    assert_eq!(case_b.challenger, AgentId("challenger-b-i39".into()));
    assert_eq!(case_a.bond.micro_units(), 2_000_000);
    assert_eq!(case_b.bond.micro_units(), 3_000_000);

    // Target work_tx's stakes_t entry is UNCHANGED by either challenge.
    let post_target_stake = post.economic_state_t.stakes_t.0
        .get(&target_work_tx_id).expect("target still in stakes_t");
    assert_eq!(post_target_stake.amount, pre_target_stake.amount,
               "target's YES stake unchanged by challenges (no slash in TB-4)");
    assert_eq!(post_target_stake.staker, pre_target_stake.staker);

    // No L4.E rows (both accepts).
    assert_eq!(l4e_row_count(&h.rejection_writer), 0);
}

// ────────────────────────────────────────────────────────────────────────────
// I40 — Rejected Verify or Challenge does NOT mutate economic_state
// (charter § 5 #10 + TB-3 § 3.4 user verdict #14 inherited)
// ────────────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn rejected_verify_or_challenge_does_not_change_economic_state() {
    let mut h = fresh_harness(genesis_with_balances(&[
        ("sponsor-i40", 100),
        ("solver-i40", 10),
        ("verifier-i40", 5),
        ("challenger-i40", 5),
    ]));

    let (target_work_tx_id, parent_after_work) = apply_task_funded_with_accepted_worktx(
        &mut h, "task-i40", "sponsor-i40", "solver-i40", 50, 3, "i40"
    ).await;

    // Snapshot economic_state at this baseline.
    let baseline = h.seq.q_snapshot().expect("baseline").economic_state_t.clone();

    // Submit a Verify with bond=0 (rejects to L4.E with BondInsufficient).
    let bad_verify = make_verify_tx(
        &target_work_tx_id.0, "verifier-i40", 0, parent_after_work, "i40-bad",
    );
    h.seq.submit(bad_verify).await.expect("submit");
    let drained = h.seq.try_apply_one(&mut h.rx).expect("env");
    assert!(drained.is_err(), "bad verify must reject");

    // Submit a Challenge with stake=0 (rejects to L4.E with StakeInsufficient).
    let bad_chal = make_challenge_tx(
        &target_work_tx_id.0, "challenger-i40", 0,
        Cid([0xEEu8; 32]), parent_after_work, "i40-bad",
    );
    h.seq.submit(bad_chal).await.expect("submit");
    let drained = h.seq.try_apply_one(&mut h.rx).expect("env");
    assert!(drained.is_err(), "bad challenge must reject");

    // L4.E grew by 2 rows; economic_state is bit-identical to baseline.
    assert_eq!(l4e_row_count(&h.rejection_writer), 2,
               "two rejections → two L4.E rows");
    let post = h.seq.q_snapshot().expect("post");
    assert_eq!(post.economic_state_t, baseline,
               "L4.E never mutates economic_state (TB-3 § 3.4 user verdict #14 inherited)");
}

// ────────────────────────────────────────────────────────────────────────────
// I43 — Challenge-window anchor pinpoint (charter § 3.9)
// opened_at_round MUST equal q.q_t.current_round at the moment of accept.
// ────────────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn challenge_window_anchor_equals_q_current_round_at_accept() {
    // Seed Q with current_round = 7 (non-zero pinpoint value); same fixture
    // shape as I32, but Q starts with q.q_t.current_round = 7 so opened_at_round
    // can be checked exactly.
    let mut q = QState::genesis();
    q.q_t.current_round = 7;
    q.economic_state_t.balances_t.0.insert(
        AgentId("sponsor-i43".into()), MicroCoin::from_coin(100).unwrap()
    );
    q.economic_state_t.balances_t.0.insert(
        AgentId("solver-i43".into()), MicroCoin::from_coin(10).unwrap()
    );
    q.economic_state_t.balances_t.0.insert(
        AgentId("challenger-i43".into()), MicroCoin::from_coin(10).unwrap()
    );

    let mut h = fresh_harness(q);

    let (target_work_tx_id, parent_after_work) = apply_task_funded_with_accepted_worktx(
        &mut h, "task-i43", "sponsor-i43", "solver-i43", 50, 3, "i43"
    ).await;

    let chal_tx = make_challenge_tx(
        &target_work_tx_id.0, "challenger-i43", 2_000_000,
        Cid([0xF1u8; 32]), parent_after_work, "i43"
    );
    let chal_id = match &chal_tx {
        TypedTx::Challenge(c) => c.tx_id.clone(),
        _ => unreachable!(),
    };
    h.seq.submit(chal_tx).await.expect("submit");
    let _ = h.seq.try_apply_one(&mut h.rx).expect("env").expect("accepted");

    let post = h.seq.q_snapshot().expect("post");
    let case = post.economic_state_t.challenge_cases_t.0
        .get(&chal_id).expect("ChallengeCase");

    // The anchor MUST equal q.q_t.current_round AT THE TIME OF ACCEPT.
    // dispatch_transition reads current_round from the Q-snapshot; since
    // none of TaskOpen/EscrowLock/Work/Challenge advance current_round
    // in TB-4 (current_round is mutated only by other future tx kinds —
    // not in TB-4 scope), the value remains 7 throughout this test's
    // accepted-tx sequence.
    assert_eq!(case.opened_at_round, 7,
               "opened_at_round must equal q.q_t.current_round at admission ({} expected, got {})",
               7, case.opened_at_round);
    // target back-ref correct.
    assert_eq!(case.target_work_tx, target_work_tx_id);
}

// ────────────────────────────────────────────────────────────────────────────
// Suppress unused-import warnings for symbols used by Atom 7 tests.
// ────────────────────────────────────────────────────────────────────────────

#[allow(dead_code)]
fn _import_anchors() {
    let _ = task_open_accept_state_root;
    let _ = escrow_lock_accept_state_root;
}
