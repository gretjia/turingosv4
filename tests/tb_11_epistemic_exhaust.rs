//! TB-11 Atom 2 integration tests — TaskExpire + TerminalSummary +
//! TaskBankruptcy via emit_system_tx → apply_one (architect §6.2 ruling
//! 2026-05-02 Epistemic Exhaust & Capital Liberation).
//!
//! Mirrors TB-8 harness pattern (`tests/tb_8_minimal_payout.rs`):
//! constructs a Sequencer, opens + funds a task via TaskOpen + EscrowLock
//! agent ingress, then emits TerminalSummary / TaskBankruptcy / TaskExpire
//! via emit_system_tx and applies through apply_one.
//!
//! Coverage:
//!   I-TB11-1  TerminalSummary anchors RunsIndex with evidence_capsule_cid.
//!   I-TB11-2  TaskExpire refunds escrow → sponsor balance; CTF preserved.
//!   I-TB11-3  TaskBankruptcy flips task_markets_t state → Bankrupt.
//!
//! /// TRACE_MATRIX TB-11 Atom 2 (architect §6.2 ruling 2026-05-02).

use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, RwLock};

use tempfile::TempDir;

use turingosv4::bottom_white::cas::schema::Cid;
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::bottom_white::ledger::rejection_evidence::RejectionEvidenceWriter;
use turingosv4::bottom_white::ledger::system_keypair::{
    Ed25519Keypair, PinnedSystemPubkeys, SystemEpoch,
};
use turingosv4::bottom_white::ledger::transition_ledger::{
    InMemoryLedgerWriter, LedgerWriter,
};
use turingosv4::bottom_white::tools::registry::ToolRegistry;
use turingosv4::economy::money::MicroCoin;
use turingosv4::state::q_state::{
    AgentId, Hash, QState, RunSummaryEntry, TaskId, TaskMarketState, TxId,
};
use turingosv4::state::sequencer::{
    Sequencer, SubmissionEnvelope, SystemEmitCommand,
};
use turingosv4::state::typed_tx::{
    AgentSignature, BankruptcyReason, EscrowLockTx, ExpireReason, RejectionClass, RunId,
    RunOutcome, TaskOpenTx, TypedTx,
};
use turingosv4::top_white::predicates::registry::PredicateRegistry;

// ── Harness ─────────────────────────────────────────────────────────────────

struct Harness {
    _tmp: TempDir,
    seq: Sequencer,
    rx: tokio::sync::mpsc::Receiver<SubmissionEnvelope>,
    _ledger: Arc<RwLock<dyn LedgerWriter>>,
}

fn fresh_harness(initial_q: QState) -> Harness {
    let tmp = TempDir::new().expect("tempdir");
    let cas = Arc::new(RwLock::new(CasStore::open(tmp.path()).expect("cas")));
    let keypair = Arc::new(Ed25519Keypair::generate_with_secure_entropy().expect("keypair"));
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
    Harness { _tmp: tmp, seq, rx, _ledger: writer }
}

fn genesis_with_sponsor(sponsor: &str, balance_coins: i64) -> QState {
    let mut q = QState::genesis();
    q.economic_state_t.balances_t.0.insert(
        AgentId(sponsor.into()),
        MicroCoin::from_coin(balance_coins).unwrap(),
    );
    q
}

async fn open_and_fund(h: &mut Harness, sponsor: &str, task: &str, amount_micro: i64) {
    let parent_root = h.seq.q_snapshot().expect("q").state_root_t;
    let open_tx = TypedTx::TaskOpen(TaskOpenTx {
        tx_id: TxId(format!("taskopen-{}", task)),
        task_id: TaskId(task.into()),
        parent_state_root: parent_root,
        sponsor_agent: AgentId(sponsor.into()),
        verifier_quorum: 1,
        max_reuse_royalty_fraction_basis_points: 1000,
        settlement_rule_hash: Hash::ZERO,
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: 1,
    });
    h.seq.submit_agent_tx(open_tx).await.expect("submit open");
    let _ = h.seq.try_apply_one(&mut h.rx).expect("env open").expect("ok open");

    let parent_root2 = h.seq.q_snapshot().expect("q").state_root_t;
    let lock_tx = TypedTx::EscrowLock(EscrowLockTx {
        tx_id: TxId(format!("escrowlock-{}", task)),
        task_id: TaskId(task.into()),
        parent_state_root: parent_root2,
        sponsor_agent: AgentId(sponsor.into()),
        amount: MicroCoin::from_micro_units(amount_micro),
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: 2,
    });
    h.seq.submit_agent_tx(lock_tx).await.expect("submit lock");
    let _ = h.seq.try_apply_one(&mut h.rx).expect("env lock").expect("ok lock");
}

// ── I-TB11-1 — TerminalSummary anchors RunsIndex ────────────────────────────

#[tokio::test]
async fn terminal_summary_emit_then_apply_writes_runs_index() {
    let q = genesis_with_sponsor("sponsor-A", 10);
    let mut h = fresh_harness(q);
    open_and_fund(&mut h, "sponsor-A", "task-A", 500_000).await;

    let capsule_cid = Cid([0x77u8; 32]);
    let mut hist = BTreeMap::new();
    hist.insert(RejectionClass::Opaque, 132);
    h.seq
        .emit_system_tx(SystemEmitCommand::TerminalSummary {
            run_id: RunId("run-zeta-001".into()),
            task_id: TaskId("task-A".into()),
            run_outcome: RunOutcome::MaxTxExhausted,
            total_attempts: 132,
            failure_class_histogram: hist,
            last_logical_t: 1,
            solver_agent: Some(AgentId("solver-1".into())),
            evidence_capsule_cid: Some(capsule_cid),
        })
        .await
        .expect("emit terminal-summary");
    let _ = h.seq.try_apply_one(&mut h.rx).expect("env ts").expect("ok ts");

    let q_after = h.seq.q_snapshot().expect("q snapshot");
    let runs = &q_after.economic_state_t.runs_t.0;
    assert_eq!(runs.len(), 1);
    let entry: &RunSummaryEntry = runs
        .get(&RunId("run-zeta-001".into()))
        .expect("runs_t entry");
    assert_eq!(entry.task_id, TaskId("task-A".into()));
    assert_eq!(entry.attempt_count, 132);
    assert_eq!(entry.evidence_capsule_cid, Some(capsule_cid));
    assert_eq!(entry.run_outcome, RunOutcome::MaxTxExhausted);

    // No money moved.
    let bal = q_after
        .economic_state_t
        .balances_t
        .0
        .get(&AgentId("sponsor-A".into()))
        .copied()
        .unwrap();
    assert_eq!(bal.micro_units(), MicroCoin::from_coin(10).unwrap().micro_units() - 500_000);

    let escrow = q_after
        .economic_state_t
        .escrows_t
        .0
        .get(&TxId("escrowlock-task-A".into()))
        .expect("escrow preserved");
    assert_eq!(escrow.amount, MicroCoin::from_micro_units(500_000));
}

// ── I-TB11-2 — TaskExpire refunds escrow → sponsor; CTF preserved ──────────

#[tokio::test]
async fn task_expire_refunds_escrow_to_sponsor() {
    let q = genesis_with_sponsor("sponsor-B", 10);
    let mut h = fresh_harness(q);
    open_and_fund(&mut h, "sponsor-B", "task-B", 100_000).await;

    let bal_pre = h
        .seq
        .q_snapshot()
        .unwrap()
        .economic_state_t
        .balances_t
        .0
        .get(&AgentId("sponsor-B".into()))
        .copied()
        .unwrap();
    assert_eq!(
        bal_pre.micro_units(),
        MicroCoin::from_coin(10).unwrap().micro_units() - 100_000,
        "post-EscrowLock balance"
    );

    h.seq
        .emit_system_tx(SystemEmitCommand::TaskExpire {
            task_id: TaskId("task-B".into()),
            escrow_tx_id: TxId("escrowlock-task-B".into()),
            reason: ExpireReason::Deadline,
        })
        .await
        .expect("emit task-expire");
    let _ = h.seq.try_apply_one(&mut h.rx).expect("env expire").expect("ok expire");

    let q_after = h.seq.q_snapshot().unwrap();
    // Sponsor balance fully restored (10 Coin).
    let bal_post = q_after
        .economic_state_t
        .balances_t
        .0
        .get(&AgentId("sponsor-B".into()))
        .copied()
        .unwrap();
    assert_eq!(bal_post, MicroCoin::from_coin(10).unwrap());
    // Escrow row removed.
    assert!(!q_after
        .economic_state_t
        .escrows_t
        .0
        .contains_key(&TxId("escrowlock-task-B".into())));
    // task_markets_t state flipped to Expired.
    let tm = q_after
        .economic_state_t
        .task_markets_t
        .0
        .get(&TaskId("task-B".into()))
        .unwrap();
    assert_eq!(tm.state, TaskMarketState::Expired);
    assert_eq!(tm.total_escrow.micro_units(), 0);
}

// ── I-TB11-3a — tb11_emit_expire_for_eligible adapter scans + emits ────────

#[tokio::test]
async fn tb11_emit_expire_for_eligible_scans_and_emits() {
    use turingosv4::runtime::adapter::tb11_emit_expire_for_eligible;

    let q = genesis_with_sponsor("sponsor-T", 10);
    let mut h = fresh_harness(q);
    open_and_fund(&mut h, "sponsor-T", "task-T", 200_000).await;

    // Open is at logical_t=1 (TaskOpen above); now at current_logical_t=10
    // with delta=5 the task is eligible (10 - 1 > 5).
    let (count, total_refunded) = tb11_emit_expire_for_eligible(&h.seq, 10, 5)
        .await
        .expect("eligible scan");
    assert_eq!(count, 1, "exactly one task expirable");
    assert_eq!(total_refunded, 200_000, "total refunded = escrow.amount");

    // Apply the queued TaskExpireTx.
    let _ = h
        .seq
        .try_apply_one(&mut h.rx)
        .expect("env expire")
        .expect("apply expire");

    let q_after = h.seq.q_snapshot().unwrap();
    let bal = q_after
        .economic_state_t
        .balances_t
        .0
        .get(&AgentId("sponsor-T".into()))
        .copied()
        .unwrap();
    assert_eq!(bal, MicroCoin::from_coin(10).unwrap());
    let tm = q_after
        .economic_state_t
        .task_markets_t
        .0
        .get(&TaskId("task-T".into()))
        .unwrap();
    assert_eq!(tm.state, TaskMarketState::Expired);

    // Second call: nothing eligible (already expired).
    let (count2, total2) = tb11_emit_expire_for_eligible(&h.seq, 20, 5)
        .await
        .expect("post-expire scan");
    assert_eq!(count2, 0);
    assert_eq!(total2, 0);
}

// ── I-TB11-3b — tb11_emit_terminal_summary_for_run helper ──────────────────

#[tokio::test]
async fn tb11_emit_terminal_summary_for_run_helper_writes_runs_index() {
    use turingosv4::runtime::adapter::tb11_emit_terminal_summary_for_run;

    let q = genesis_with_sponsor("sponsor-S", 10);
    let mut h = fresh_harness(q);
    open_and_fund(&mut h, "sponsor-S", "task-S", 50_000).await;

    let capsule_cid = Cid([0xefu8; 32]);
    let mut hist = BTreeMap::new();
    hist.insert(RejectionClass::Opaque, 7);
    let _receipt = tb11_emit_terminal_summary_for_run(
        &h.seq,
        RunId("run-S-001".into()),
        TaskId("task-S".into()),
        RunOutcome::WallClockCap,
        7,
        hist,
        100,
        Some(AgentId("solver-S".into())),
        Some(capsule_cid),
    )
    .await
    .expect("helper emits");
    let _ = h.seq.try_apply_one(&mut h.rx).expect("env ts").expect("apply ts");

    let q_after = h.seq.q_snapshot().unwrap();
    let entry = q_after
        .economic_state_t
        .runs_t
        .0
        .get(&RunId("run-S-001".into()))
        .expect("runs_t entry");
    assert_eq!(entry.run_outcome, RunOutcome::WallClockCap);
    assert_eq!(entry.evidence_capsule_cid, Some(capsule_cid));
}

// ── I-TB11-3 — TaskBankruptcy flips state to Bankrupt ──────────────────────

#[tokio::test]
async fn task_bankruptcy_flips_state() {
    let q = genesis_with_sponsor("sponsor-C", 10);
    let mut h = fresh_harness(q);
    open_and_fund(&mut h, "sponsor-C", "task-C", 500_000).await;

    let capsule_cid = Cid([0xaau8; 32]);
    h.seq
        .emit_system_tx(SystemEmitCommand::TaskBankruptcy {
            task_id: TaskId("task-C".into()),
            evidence_capsule_cid: capsule_cid,
            bankruptcy_reason: BankruptcyReason::MaxFailedRunCount,
            failed_run_count: 3,
        })
        .await
        .expect("emit task-bankruptcy");
    let _ = h.seq.try_apply_one(&mut h.rx).expect("env bk").expect("ok bk");

    let q_after = h.seq.q_snapshot().unwrap();
    let tm = q_after
        .economic_state_t
        .task_markets_t
        .0
        .get(&TaskId("task-C".into()))
        .unwrap();
    assert_eq!(tm.state, TaskMarketState::Bankrupt);
    assert!(tm.bankruptcy_at_logical_t.is_some());

    // Escrow remains locked (refund is a separate post-bankruptcy TaskExpire).
    let esc = q_after
        .economic_state_t
        .escrows_t
        .0
        .get(&TxId("escrowlock-task-C".into()))
        .expect("escrow preserved on bankruptcy");
    assert_eq!(esc.amount.micro_units(), 500_000);
}

// ── Avoid `unused` import warnings via #[allow(dead_code)] for test helpers
//   — silenced by Cargo's default test-target lint config.

// Suppress unused warning for BTreeSet (kept ready for future tests).
#[allow(dead_code)]
fn _suppress_unused() -> BTreeSet<TxId> {
    BTreeSet::new()
}
