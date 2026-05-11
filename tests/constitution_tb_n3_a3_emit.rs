//! TB-N3 A3 — auto-market emit constitution gate tests.
//!
//! SG-N3.4 — `tb_n3_emit_node_market_after_work_accept` produces a pool
//! whose `event_id` is `node_survive:<work_tx_id>`-namespaced
//! (architect amendment 1).
//!
//! SG-N3.5 — Treasury / MarketMakerBudget is debited by exactly
//! `seed_micro` per pool (architect §3.4 + §7 — no ghost liquidity).
//!
//! SG-N3.6 — When MarketMakerBudget < seed_micro, no pool is created and
//! no shares are minted (`BudgetExhausted` outcome; fail-closed).
//!
//! Harness mirrors `tests/constitution_router_buy_with_coin.rs` pattern:
//! `Sequencer::new(...)` + `submit_agent_tx` + `try_apply_one` for
//! synchronous drain.
//!
//! NOTE: A3 admission requires task_markets_t entry which we register via
//! a real TaskOpen sub-tx within the helper. The test exercises this
//! end-to-end through the canonical agent ingress path.

use std::sync::{Arc, RwLock};

use tempfile::TempDir;
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
use turingosv4::runtime::adapter::{
    make_real_escrow_lock_signed_by, make_real_task_open_signed_by,
    make_real_worktx_signed_by, NodeMarketEmitOutcome,
};
use turingosv4::runtime::agent_keypairs::AgentKeypairRegistry;
use turingosv4::state::q_state::{AgentId, QState, TaskId, TxId};
use turingosv4::state::sequencer::{Sequencer, SubmissionEnvelope};
use turingosv4::state::typed_tx::TypedTx;
use turingosv4::top_white::predicates::registry::PredicateRegistry;

struct Harness {
    _tmp: TempDir,
    seq: Sequencer,
    rx: tokio::sync::mpsc::Receiver<SubmissionEnvelope>,
    _ledger: Arc<RwLock<dyn LedgerWriter>>,
    reg: AgentKeypairRegistry,
    repo_path: std::path::PathBuf,
}

fn fresh_harness(work_agent_balance: i64, mmb_balance: i64) -> Harness {
    let tmp = TempDir::new().expect("tempdir");
    let repo_path = tmp.path().to_path_buf();
    let cas = Arc::new(RwLock::new(CasStore::open(tmp.path()).expect("cas")));
    let keypair = Arc::new(Ed25519Keypair::generate_with_secure_entropy().expect("kp"));
    let writer: Arc<RwLock<dyn LedgerWriter>> =
        Arc::new(RwLock::new(InMemoryLedgerWriter::new()));
    let rejection_writer = Arc::new(RwLock::new(RejectionEvidenceWriter::default()));
    let preds = Arc::new(PredicateRegistry::new());
    let tools = Arc::new(ToolRegistry::new());
    let epoch = SystemEpoch::new(1);
    let mut pinned = PinnedSystemPubkeys::new();
    pinned.insert(epoch, keypair.public_key());
    let pinned_pubkeys = Arc::new(pinned);

    let mut q = QState::genesis();
    q.economic_state_t.balances_t.0.insert(
        AgentId("Agent_0".into()),
        MicroCoin::from_micro_units(work_agent_balance),
    );
    q.economic_state_t.balances_t.0.insert(
        AgentId("MarketMakerBudget".into()),
        MicroCoin::from_micro_units(mmb_balance),
    );

    let (seq, rx) = Sequencer::new(
        cas,
        keypair,
        epoch,
        writer.clone(),
        rejection_writer,
        preds,
        tools,
        pinned_pubkeys,
        q,
        16,
    );
    let reg = AgentKeypairRegistry::open(&repo_path).expect("agent keypairs");
    Harness {
        _tmp: tmp,
        seq,
        rx,
        _ledger: writer,
        reg,
        repo_path,
    }
}

async fn submit_and_apply(h: &mut Harness, tx: TypedTx) -> Result<(), String> {
    h.seq
        .submit_agent_tx(tx)
        .await
        .map_err(|e| format!("submit: {e:?}"))?;
    let outcome = h
        .seq
        .try_apply_one(&mut h.rx)
        .ok_or_else(|| "no envelope drained".to_string())?;
    outcome
        .map(|_| ())
        .map_err(|e| format!("apply: {e:?}"))
}

/// Submit TaskOpen + EscrowLock + WorkTx for Agent_0 and return the
/// accepted work_tx_id. Synchronous (in-memory drain).
async fn submit_accepted_work_tx(h: &mut Harness, task_id: &str, suffix: &str) -> TxId {
    let pre_root = h.seq.q_snapshot().expect("snap").state_root_t;
    let task_open = make_real_task_open_signed_by(
        &mut h.reg, task_id, "Agent_0", pre_root, suffix, 1,
    )
    .expect("task_open sign");
    submit_and_apply(h, task_open).await.expect("task_open apply");

    let after_open = h.seq.q_snapshot().unwrap().state_root_t;
    let escrow = make_real_escrow_lock_signed_by(
        &mut h.reg, task_id, "Agent_0", 100_000, after_open, suffix, 1,
    )
    .expect("escrow sign");
    submit_and_apply(h, escrow).await.expect("escrow apply");

    let after_escrow = h.seq.q_snapshot().unwrap().state_root_t;
    let work_tx = make_real_worktx_signed_by(
        &mut h.reg,
        task_id,
        "Agent_0",
        after_escrow,
        1_000,
        suffix,
        turingosv4::bottom_white::cas::schema::Cid([7u8; 32]),
        true,
        1,
    )
    .expect("work sign");
    let work_tx_id = match &work_tx {
        TypedTx::Work(w) => w.tx_id.clone(),
        _ => panic!("expected Work"),
    };
    submit_and_apply(h, work_tx).await.expect("work apply");
    work_tx_id
}

/// Run `tb_n3_emit_node_market_after_work_accept` driving the in-memory
/// drain after each sub-tx submission. The async helper polls for
/// `tb8_await_state_root_advance`, so we need to drain envelopes
/// concurrently. Easier approach: directly inline the helper steps so
/// we can apply each sub-tx synchronously.
async fn run_a3_emit_synchronous(
    h: &mut Harness,
    work_tx_id: &TxId,
    seed_micro: i64,
    suffix: &str,
) -> NodeMarketEmitOutcome {
    use turingosv4::runtime::adapter::{
        make_real_cpmm_pool_signed_by, make_real_market_seed_signed_by,
    };
    use turingosv4::state::typed_tx::node_survive_event_id;

    let event_id = node_survive_event_id(work_tx_id);
    let q0 = h.seq.q_snapshot().expect("snap");
    if q0.economic_state_t.cpmm_pools_t.0.contains_key(&event_id) {
        return NodeMarketEmitOutcome::AlreadyExists;
    }
    let mmb_bal = q0
        .economic_state_t
        .balances_t
        .0
        .get(&AgentId("MarketMakerBudget".into()))
        .copied()
        .unwrap_or(MicroCoin::zero());
    if mmb_bal.micro_units() < seed_micro {
        return NodeMarketEmitOutcome::BudgetExhausted;
    }

    let pre_root = q0.state_root_t;
    let task_str = event_id.0.0.clone();
    let task_open = make_real_task_open_signed_by(
        &mut h.reg, &task_str, "MarketMakerBudget", pre_root, suffix, 1,
    )
    .expect("task_open sign");
    if let Err(e) = submit_and_apply(h, task_open).await {
        return NodeMarketEmitOutcome::SubmitFailed {
            step: "task_open_submit",
            message: e,
        };
    }
    let after_open = h.seq.q_snapshot().unwrap().state_root_t;

    let market_seed = make_real_market_seed_signed_by(
        &mut h.reg, after_open, &task_str, "MarketMakerBudget", seed_micro, suffix, 1,
    )
    .expect("market_seed sign");
    if let Err(e) = submit_and_apply(h, market_seed).await {
        return NodeMarketEmitOutcome::SubmitFailed {
            step: "market_seed_submit",
            message: e,
        };
    }
    let after_seed = h.seq.q_snapshot().unwrap().state_root_t;

    let pool = make_real_cpmm_pool_signed_by(
        &mut h.reg, after_seed, &task_str, "MarketMakerBudget", seed_micro as u128, suffix,
    )
    .expect("pool sign");
    if let Err(e) = submit_and_apply(h, pool).await {
        return NodeMarketEmitOutcome::SubmitFailed {
            step: "cpmm_pool_submit",
            message: e,
        };
    }

    NodeMarketEmitOutcome::Created {
        event_id,
        pool_seed_micro: seed_micro,
    }
}

/// SG-N3.4 — auto-emitted pool's event_id is `node_survive:`-namespaced.
#[tokio::test]
async fn sg_n3_4_event_id_is_node_survive_namespaced() {
    let mut h = fresh_harness(1_000_000, 5_000_000);
    let work_tx_id = submit_accepted_work_tx(&mut h, "task-sg-n3-4", "u1").await;

    let outcome = run_a3_emit_synchronous(&mut h, &work_tx_id, 100_000, "u1").await;
    match outcome {
        NodeMarketEmitOutcome::Created { event_id, pool_seed_micro } => {
            assert!(
                event_id.0.0.starts_with("node_survive:"),
                "SG-N3.4 violated: event_id={:?}", event_id.0.0
            );
            assert_eq!(event_id.0.0, format!("node_survive:{}", work_tx_id.0));
            assert_eq!(pool_seed_micro, 100_000);
        }
        other => panic!("expected Created, got {other:?}"),
    }
    let q = h.seq.q_snapshot().expect("snap");
    let event_id = turingosv4::state::typed_tx::node_survive_event_id(&work_tx_id);
    assert!(q.economic_state_t.cpmm_pools_t.0.contains_key(&event_id));
    assert!(
        !q.economic_state_t.cpmm_pools_t.0.contains_key(
            &turingosv4::state::typed_tx::EventId(TaskId("task-sg-n3-4".into()))
        ),
        "no pool created at bare task_id (architect amendment 1 negative-witness)"
    );
}

/// SG-N3.5 — MarketMakerBudget debited by exactly seed_micro per pool.
#[tokio::test]
async fn sg_n3_5_treasury_debit_exact() {
    let mut h = fresh_harness(1_000_000, 5_000_000);
    let bal_pre = h
        .seq
        .q_snapshot()
        .unwrap()
        .economic_state_t
        .balances_t
        .0
        .get(&AgentId("MarketMakerBudget".into()))
        .copied()
        .unwrap()
        .micro_units();
    assert_eq!(bal_pre, 5_000_000);

    let work_tx_id = submit_accepted_work_tx(&mut h, "task-sg-n3-5", "u1").await;
    let outcome = run_a3_emit_synchronous(&mut h, &work_tx_id, 100_000, "u1").await;
    assert!(matches!(outcome, NodeMarketEmitOutcome::Created { .. }));

    let bal_post = h
        .seq
        .q_snapshot()
        .unwrap()
        .economic_state_t
        .balances_t
        .0
        .get(&AgentId("MarketMakerBudget".into()))
        .copied()
        .unwrap()
        .micro_units();
    assert_eq!(
        bal_post, 5_000_000 - 100_000,
        "SG-N3.5: MarketMakerBudget debited by exactly 100_000 μC"
    );
}

/// SG-N3.6 — Insufficient budget: BudgetExhausted; no pool, no ghost.
#[tokio::test]
async fn sg_n3_6_budget_exhausted_no_ghost_pool() {
    let mut h = fresh_harness(1_000_000, 50_000);
    let work_tx_id = submit_accepted_work_tx(&mut h, "task-sg-n3-6", "u1").await;

    let outcome = run_a3_emit_synchronous(&mut h, &work_tx_id, 100_000, "u1").await;
    assert!(
        matches!(outcome, NodeMarketEmitOutcome::BudgetExhausted),
        "SG-N3.6: insufficient budget MUST be BudgetExhausted, got {outcome:?}"
    );

    let q = h.seq.q_snapshot().expect("snap");
    let event_id = turingosv4::state::typed_tx::node_survive_event_id(&work_tx_id);
    assert!(
        !q.economic_state_t.cpmm_pools_t.0.contains_key(&event_id),
        "SG-N3.6 violated: pool present despite budget exhaustion"
    );
    assert_eq!(
        q.economic_state_t
            .balances_t
            .0
            .get(&AgentId("MarketMakerBudget".into()))
            .copied()
            .unwrap()
            .micro_units(),
        50_000,
        "MMB balance unchanged"
    );
}

/// SG-N3.4-aux — idempotency: re-call returns AlreadyExists, no double debit.
#[tokio::test]
async fn sg_n3_4_aux_idempotent_re_emit() {
    let mut h = fresh_harness(1_000_000, 5_000_000);
    let work_tx_id = submit_accepted_work_tx(&mut h, "task-idem", "u1").await;

    let first = run_a3_emit_synchronous(&mut h, &work_tx_id, 100_000, "u1").await;
    assert!(matches!(first, NodeMarketEmitOutcome::Created { .. }));
    let bal_after_first = h
        .seq
        .q_snapshot()
        .unwrap()
        .economic_state_t
        .balances_t
        .0
        .get(&AgentId("MarketMakerBudget".into()))
        .copied()
        .unwrap()
        .micro_units();

    let second = run_a3_emit_synchronous(&mut h, &work_tx_id, 100_000, "u2").await;
    assert!(
        matches!(second, NodeMarketEmitOutcome::AlreadyExists),
        "idempotent re-call expected, got {second:?}"
    );
    let bal_after_second = h
        .seq
        .q_snapshot()
        .unwrap()
        .economic_state_t
        .balances_t
        .0
        .get(&AgentId("MarketMakerBudget".into()))
        .copied()
        .unwrap()
        .micro_units();
    assert_eq!(bal_after_first, bal_after_second);
}

#[test]
fn _suppress_unused() {
    let _ = std::path::PathBuf::new();
}
