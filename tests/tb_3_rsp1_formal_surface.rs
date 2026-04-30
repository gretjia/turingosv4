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
    task_open_accept_state_root, Sequencer, SubmissionEnvelope,
};
use turingosv4::state::typed_tx::{AgentSignature, TaskOpenTx, TypedTx};
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
