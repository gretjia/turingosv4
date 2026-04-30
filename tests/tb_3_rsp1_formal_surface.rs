//! TB-3 RSP-1 formal-tx-surface — integration tests through `Sequencer::submit`.
//!
//! Charter: `handover/tracer_bullets/TB-3_charter_2026-04-30.md` (DRAFT v2).
//! Preflight: `handover/ai-direct/TB-3_RSP1_FORMAL_TX_SURFACE_2026-04-30.md`.
//!
//! Per charter § 4.7 + preflight § 5.3, this file holds I20-I30 — every test
//! goes through the public `Sequencer::submit` path. L4.E rows are observed
//! via the constructor-injected `Arc<RwLock<RejectionEvidenceWriter>>` clone
//! the test retains.
//!
//! Atom 4 covers I20 (TaskOpen accepted appends to canonical L4).
//! Atoms 5+ add I21-I30.

use std::collections::BTreeSet;
use std::sync::{Arc, RwLock};

use tempfile::TempDir;

use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::bottom_white::ledger::rejection_evidence::RejectionEvidenceWriter;
use turingosv4::bottom_white::ledger::system_keypair::{Ed25519Keypair, SystemEpoch};
use turingosv4::bottom_white::ledger::transition_ledger::{InMemoryLedgerWriter, LedgerWriter, TxKind};
use turingosv4::bottom_white::tools::registry::ToolRegistry;
use turingosv4::economy::money::MicroCoin;
use turingosv4::state::q_state::{AgentId, Hash, QState, TaskId, TxId};
use turingosv4::state::sequencer::{
    escrow_lock_accept_state_root, task_open_accept_state_root, Sequencer, SubmissionEnvelope,
};
use turingosv4::state::typed_tx::{AgentSignature, EscrowLockTx, TaskOpenTx, TypedTx};
use turingosv4::top_white::predicates::registry::PredicateRegistry;

// ────────────────────────────────────────────────────────────────────────────
// Fixtures
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

/// Seed sponsor balance directly into genesis QState (test-only helper). Real
/// production seeding comes from on_init_tx; for TB-3 RSP-1 admission tests
/// we just inject a starting balance.
fn genesis_with_balance(sponsor: &str, balance_coin: i64) -> QState {
    let mut q = QState::genesis();
    q.economic_state_t.balances_t.0.insert(
        AgentId(sponsor.into()),
        MicroCoin::from_coin(balance_coin).unwrap(),
    );
    q
}

fn l4e_row_count(writer: &Arc<RwLock<RejectionEvidenceWriter>>) -> usize {
    writer.read().expect("writer read").records().len()
}

fn l4_row_count(writer: &Arc<RwLock<dyn LedgerWriter>>) -> u64 {
    writer.read().expect("writer read").len()
}

// ────────────────────────────────────────────────────────────────────────────
// I20 — TaskOpen submitted through Sequencer::submit appends to canonical L4
// ────────────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn submit_task_open_tx_appends_to_canonical_l4() {
    let mut h = fresh_harness(QState::genesis());

    let pre_l4 = l4_row_count(&h.ledger_writer);
    let pre_l4e = l4e_row_count(&h.rejection_writer);
    assert_eq!(pre_l4, 0);
    assert_eq!(pre_l4e, 0);

    let tx = make_task_open("task-i20", "sponsor-alice", Hash::ZERO, "i20");
    let receipt = h.seq.submit(tx.clone()).await.expect("submit accepted");
    assert_eq!(receipt.submit_id, 1);

    let drained = h.seq.try_apply_one(&mut h.rx).expect("envelope was queued");
    assert!(drained.is_ok(), "TaskOpen on genesis must accept; got {:?}", drained);

    // Charter § 7 Proof 1 ingredient: 1 canonical L4 row, zero L4.E rows.
    assert_eq!(l4_row_count(&h.ledger_writer), 1, "TaskOpen accepted must append exactly 1 L4 row");
    assert_eq!(l4e_row_count(&h.rejection_writer), 0, "Accepted TaskOpen must not write to L4.E");

    // Q_t now has the TaskMarketEntry; balances untouched (metadata-only per charter § 3.3).
    let q_after = h.seq.q_snapshot().expect("q_snapshot");
    let entry = q_after.economic_state_t.task_markets_t.0
        .get(&TaskId("task-i20".into()))
        .expect("TaskMarketEntry should be inserted by accepted TaskOpen");
    assert_eq!(entry.publisher, AgentId("sponsor-alice".into()));
    assert_eq!(entry.total_escrow.micro_units(), 0);
    assert!(entry.escrow_lock_tx_ids.is_empty());
    assert!(q_after.economic_state_t.balances_t.0.is_empty());
    assert!(q_after.economic_state_t.escrows_t.0.is_empty());

    // state_root_t advanced via TASK_OPEN_DOMAIN_V1.
    let expected = task_open_accept_state_root(&Hash::ZERO, &tx);
    assert_eq!(q_after.state_root_t, expected);

    // logical_t incremented (accepted spine).
    assert_eq!(h.seq.next_logical_t_peek(), 1);

    // Sanity: the L4 row's tx_kind is TaskOpen (charter § 4.1 + transition_ledger.rs TxKind::TaskOpen).
    let entry = drained.expect("entry");
    assert_eq!(entry.tx_kind, TxKind::TaskOpen);

    // Suppress unused-import warning for BTreeSet / MicroCoin.
    let _ = (BTreeSet::<TxId>::new(), MicroCoin::zero());
}

// ────────────────────────────────────────────────────────────────────────────
// I21 — EscrowLock submitted through Sequencer::submit appends to canonical L4
// ────────────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn submit_escrow_lock_tx_appends_to_canonical_l4() {
    let mut h = fresh_harness(genesis_with_balance("sponsor-i21", 100));

    // First, open the task.
    let pre = h.seq.q_snapshot().expect("snapshot").state_root_t;
    let open_tx = make_task_open("task-i21", "sponsor-i21", pre, "i21");
    h.seq.submit(open_tx).await.expect("open submit");
    let _ = h.seq.try_apply_one(&mut h.rx).expect("open envelope").expect("open accepted");

    let pre_lock_state = h.seq.q_snapshot().expect("snapshot").state_root_t;
    let lock_tx = make_escrow_lock("task-i21", "sponsor-i21", 50_000_000, pre_lock_state, "i21");
    let receipt = h.seq.submit(lock_tx.clone()).await.expect("lock submit");

    let pre_l4 = l4_row_count(&h.ledger_writer);
    let drained = h.seq.try_apply_one(&mut h.rx).expect("lock envelope was queued");
    assert!(drained.is_ok(), "EscrowLock with sufficient balance + open task must accept; got {:?}", drained);

    // Charter § 7 Proof 1 ingredient.
    assert_eq!(l4_row_count(&h.ledger_writer), pre_l4 + 1, "EscrowLock accepted appends 1 L4 row");
    assert_eq!(l4e_row_count(&h.rejection_writer), 0, "Accepted EscrowLock must not write to L4.E");

    let entry = drained.expect("entry");
    assert_eq!(entry.tx_kind, TxKind::EscrowLock);

    // logical_t now 2 (TaskOpen + EscrowLock).
    assert_eq!(h.seq.next_logical_t_peek(), 2);
    let _ = receipt.submit_id; // submit_id alloc verified at envelope level
}

// ────────────────────────────────────────────────────────────────────────────
// I22 — EscrowLock atomic balance → escrow transfer + cache=truth invariant
// ────────────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn escrow_lock_atomic_balance_to_escrow_transfer() {
    let mut h = fresh_harness(genesis_with_balance("sponsor-i22", 100));

    // Pre-state: balance 100, no escrow, no task.
    let q0 = h.seq.q_snapshot().expect("q0");
    assert_eq!(
        q0.economic_state_t.balances_t.0.get(&AgentId("sponsor-i22".into()))
            .expect("seeded").micro_units(),
        100_000_000,
    );

    // Open + lock.
    let open_tx = make_task_open("task-i22", "sponsor-i22", q0.state_root_t, "i22");
    h.seq.submit(open_tx).await.expect("open");
    let _ = h.seq.try_apply_one(&mut h.rx).expect("open env").expect("open accepted");

    let parent = h.seq.q_snapshot().expect("snap").state_root_t;
    let lock_tx = make_escrow_lock("task-i22", "sponsor-i22", 30_000_000, parent, "i22");
    let lock_tx_id_str = "escrowlock-task-i22-i22";
    h.seq.submit(lock_tx.clone()).await.expect("lock submit");
    let _ = h.seq.try_apply_one(&mut h.rx).expect("lock env").expect("lock accepted");

    // Charter § 3.2 + § 7 Proof 1: atomic transfer. balance debited, escrow
    // credited, cache=truth holds.
    let q_after = h.seq.q_snapshot().expect("q_after");

    let bal = q_after.economic_state_t.balances_t.0
        .get(&AgentId("sponsor-i22".into())).expect("balance");
    assert_eq!(bal.micro_units(), 70_000_000, "100 - 30 = 70 coin");

    let escrow = q_after.economic_state_t.escrows_t.0
        .get(&TxId(lock_tx_id_str.into())).expect("escrow row by lock_tx_id");
    assert_eq!(escrow.amount.micro_units(), 30_000_000);
    assert_eq!(escrow.task_id, TaskId("task-i22".into()));

    let market = q_after.economic_state_t.task_markets_t.0
        .get(&TaskId("task-i22".into())).expect("market");
    assert_eq!(market.total_escrow.micro_units(), 30_000_000,
        "cache reflects truth: total_escrow == sum of escrow_locks");
    assert!(market.escrow_lock_tx_ids.contains(&TxId(lock_tx_id_str.into())));

    // Cache=truth invariant holds (this is enforced inside dispatch arm; here
    // we double-check by walking escrows_t and summing).
    let derived: i64 = q_after.economic_state_t.escrows_t.0.values()
        .filter(|e| e.task_id == TaskId("task-i22".into()))
        .map(|e| e.amount.micro_units())
        .sum();
    assert_eq!(market.total_escrow.micro_units(), derived);

    // CTF conservation: pre-genesis (100 in balances) → post (70 in balances + 30 in escrows) = 100 invariant.
    let total_after: i64 = q_after.economic_state_t.balances_t.0.values().map(|v| v.micro_units()).sum::<i64>()
        + q_after.economic_state_t.escrows_t.0.values().map(|e| e.amount.micro_units()).sum::<i64>();
    assert_eq!(total_after, 100_000_000, "CTF conserved: 100 coin total before and after");

    // state_root advanced via ESCROW_LOCK_DOMAIN_V1.
    let expected = escrow_lock_accept_state_root(&parent, &lock_tx);
    assert_eq!(q_after.state_root_t, expected);
}
