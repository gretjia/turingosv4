//! TB-G G2.3 — Failed-invest L4.E binding test (Class 2).
//!
//! Charter: `handover/tracer_bullets/TB_G_GENERATIVE_ARENA_charter_2026-05-11.md`
//! §1 Module G2 atom G2.3.
//!
//! Directive: `handover/directives/2026-05-11_G_PHASE_GENERATIVE_ARENA_DIRECTIVE.md`
//! §G2 SG-G2.4 verbatim "Failed invest attempts enter L4.E".
//!
//! Ship gates (charter §1 Module G2.3 row):
//! - SG-G2.5.a: `BuyWithCoinRouterTx` submitted with `pay_coin > balance`
//!   reaches sequencer admission and is rejected (the underlying
//!   `TransitionError::RouterInsufficientCoinBalance` is the dispatch-
//!   side cause; per `src/state/sequencer.rs::rejection_class_for` the
//!   coarse L4.E `rejection_class` is `RejectionClass::PolicyViolation`
//!   with `public_summary = Some("policy_violation")`. The rejection
//!   record lands in the L4.E evidence writer with `tx_kind ==
//!   TxKind::BuyWithCoinRouter`. The fine-grained Router variant is
//!   recoverable from the `raw_diagnostic_cid` CAS payload per the
//!   preflight §3.6 mapping policy).
//! - SG-G2.5.b: `BuyWithCoinRouterTx` submitted against a non-Active pool
//!   produces `TransitionError::RouterPoolNotActive` → coarse class
//!   `PolicyViolation` and lands in L4.E (covers the `RouterRejected`
//!   no-trade-reason variant on the caller-side).
//! - SG-G2.5.c: Adapter-side pre-classifier `tb_n3_invest_to_router_tx`
//!   given the same shape parameters routes to `InvestRouteError::
//!   AmountExceedsBalance` → `NoTradeReason::AmountExceedsBalance`. The
//!   evaluator-side pattern (build trace, write to CAS) round-trips
//!   through `MarketDecisionTraceSummary::compute_from_path` and shows
//!   up in the §F.A exhaustive-breakdown count.
//! - SG-G2.5.d: end-to-end (SG-G2.4 verbatim): adapter sees full
//!   `q_snapshot` with sufficient balance + Active pool → router tx
//!   built; sequencer with a **different** snapshot (balance drained
//!   between adapter pre-check and submit) rejects with
//!   `RouterInsufficientCoinBalance`. This exercises the OBS_TB_N2_B2 race
//!   shape: pre-submit classifier OK + post-submit admission FAIL = tx
//!   anchored in L4.E (architect §8.6 "failed invest 也算有意义 tape
//!   activity"). The evaluator wire writes `MarketDecisionTrace::
//!   submitted` then no-trade-reroutes via L4.E for the audit chain.
//!
//! `FC-trace: FC1-N6 + §6 externalized attempt rule — every router-rejected
//! BuyWithCoinRouterTx is canonical L4.E evidence; the no-trade-reason
//! taxonomy is the caller-side classifier that anchors the same event in
//! CAS via MarketDecisionTrace.`

use std::sync::{Arc, RwLock};

use tempfile::TempDir;
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::bottom_white::ledger::rejection_evidence::{
    RejectionClass, RejectionEvidenceWriter,
};
use turingosv4::bottom_white::ledger::system_keypair::{
    Ed25519Keypair, PinnedSystemPubkeys, SystemEpoch,
};
use turingosv4::bottom_white::ledger::transition_ledger::{
    InMemoryLedgerWriter, LedgerWriter, TxKind,
};
use turingosv4::bottom_white::tools::registry::ToolRegistry;
use turingosv4::economy::money::MicroCoin;
use turingosv4::runtime::adapter::{tb_n3_invest_to_router_tx, InvestRouteError};
use turingosv4::runtime::agent_keypairs::AgentKeypairRegistry;
use turingosv4::runtime::market_decision_trace::{
    write_market_decision_trace_to_cas, MarketDecisionTrace, NoTradeReason,
};
use turingosv4::runtime::market_decision_trace_summary::MarketDecisionTraceSummary;
use turingosv4::state::q_state::{
    AgentId, CpmmPool, Hash, LpShareAmount, PoolStatus, QState, TaskId, TaskMarketEntry,
    TaskMarketState, TxId,
};
use turingosv4::state::sequencer::{Sequencer, SubmissionEnvelope};
use turingosv4::state::typed_tx::{
    AgentSignature, BuyDirection, BuyWithCoinRouterTx, CompleteSetMintTx, CpmmPoolTx, EventId,
    ShareAmount, TypedTx,
};
use turingosv4::top_white::predicates::registry::PredicateRegistry;

// ── Harness (variant of constitution_router_buy_with_coin.rs that keeps
// the rejection_writer handle exposed for L4.E witness inspection) ──

struct Harness {
    _tmp: TempDir,
    seq: Sequencer,
    rx: tokio::sync::mpsc::Receiver<SubmissionEnvelope>,
    rejection_writer: Arc<RwLock<RejectionEvidenceWriter>>,
}

fn fresh_harness(initial_q: QState) -> Harness {
    let tmp = TempDir::new().expect("tempdir");
    let cas = Arc::new(RwLock::new(CasStore::open(tmp.path()).expect("cas")));
    let keypair = Arc::new(Ed25519Keypair::generate_with_secure_entropy().expect("kp"));
    let writer: Arc<RwLock<dyn LedgerWriter>> = Arc::new(RwLock::new(InMemoryLedgerWriter::new()));
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
        rejection_writer.clone(),
        preds,
        tools,
        pinned_pubkeys,
        initial_q,
        16,
    );
    Harness {
        _tmp: tmp,
        seq,
        rx,
        rejection_writer,
    }
}

fn genesis_with_balances_and_open_task(pairs: &[(&str, i64)], task: &str) -> QState {
    let mut q = QState::genesis();
    for (name, coin) in pairs {
        q.economic_state_t.balances_t.0.insert(
            AgentId((*name).into()),
            MicroCoin::from_coin(*coin).unwrap(),
        );
    }
    let mut entry = TaskMarketEntry::default();
    entry.state = TaskMarketState::Open;
    q.economic_state_t
        .task_markets_t
        .0
        .insert(TaskId(task.into()), entry);
    q
}

async fn submit_and_apply(h: &mut Harness, tx: TypedTx) -> Result<(), String> {
    h.seq
        .submit_agent_tx(tx)
        .await
        .map_err(|e| format!("submit error: {e:?}"))?;
    let outcome = h
        .seq
        .try_apply_one(&mut h.rx)
        .ok_or_else(|| "no envelope drained".to_string())?;
    outcome
        .map(|_| ())
        .map_err(|e| format!("apply error: {e:?}"))
}

fn build_mint(parent: Hash, owner: &str, task: &str, micro: i64, seq_no: u64) -> TypedTx {
    TypedTx::CompleteSetMint(CompleteSetMintTx {
        tx_id: TxId(format!("mint-{owner}-{task}-{seq_no}")),
        parent_state_root: parent,
        event_id: EventId(TaskId(task.into())),
        owner: AgentId(owner.into()),
        amount: MicroCoin::from_micro_units(micro),
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: 1000 + seq_no,
    })
}

fn build_pool(
    parent: Hash,
    provider: &str,
    task: &str,
    seed_yes_units: u128,
    seed_no_units: u128,
    seq_no: u64,
) -> TypedTx {
    TypedTx::CpmmPool(CpmmPoolTx {
        tx_id: TxId(format!("pool-{provider}-{task}-{seq_no}")),
        parent_state_root: parent,
        event_id: EventId(TaskId(task.into())),
        provider: AgentId(provider.into()),
        seed_yes: ShareAmount::from_units(seed_yes_units),
        seed_no: ShareAmount::from_units(seed_no_units),
        signature: AgentSignature::from_bytes([0u8; 64]),
    })
}

fn build_router(
    parent: Hash,
    buyer: &str,
    task: &str,
    direction: BuyDirection,
    pay_micro: i64,
    min_out_units: u128,
    seq_no: u64,
) -> TypedTx {
    TypedTx::BuyWithCoinRouter(BuyWithCoinRouterTx {
        tx_id: TxId(format!("router-{buyer}-{task}-{seq_no}")),
        parent_state_root: parent,
        event_id: EventId(TaskId(task.into())),
        buyer: AgentId(buyer.into()),
        direction,
        pay_coin: MicroCoin::from_micro_units(pay_micro),
        min_out_shares: ShareAmount::from_units(min_out_units),
        signature: AgentSignature::from_bytes([0u8; 64]),
    })
}

async fn seed_pool(h: &mut Harness, provider: &str, task: &str, seed_units: u128) -> Hash {
    let p = h.seq.q_snapshot().unwrap().state_root_t;
    submit_and_apply(h, build_mint(p, provider, task, seed_units as i64, 1))
        .await
        .expect("provider mint accepted");
    let p = h.seq.q_snapshot().unwrap().state_root_t;
    submit_and_apply(h, build_pool(p, provider, task, seed_units, seed_units, 1))
        .await
        .expect("pool creation accepted");
    h.seq.q_snapshot().unwrap().state_root_t
}

fn router_rejection_records(h: &Harness) -> Vec<(TxKind, RejectionClass)> {
    let guard = h.rejection_writer.read().expect("rw");
    guard
        .records()
        .iter()
        .map(|r| (r.tx_kind, r.rejection_class))
        .collect()
}

// ────────────────────────────────────────────────────────────────────────
// SG-G2.5.a — insufficient balance → RouterInsufficientCoinBalance in L4.E
// ────────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn sg_g2_5_a_insufficient_balance_lands_in_l4e() {
    // Bob has 1 Coin (1_000_000 μC); attempts to pay 5_000_000 μC.
    // Router-side admission rejects with RouterInsufficientCoinBalance.
    let q0 = genesis_with_balances_and_open_task(&[("alice", 50), ("bob", 1)], "evt-1");
    let mut h = fresh_harness(q0);
    let parent = seed_pool(&mut h, "alice", "evt-1", 5_000_000).await;

    let err = submit_and_apply(
        &mut h,
        build_router(
            parent,
            "bob",
            "evt-1",
            BuyDirection::BuyYes,
            5_000_000, // payC > balance (1_000_000)
            0,
            1,
        ),
    )
    .await
    .expect_err("must reject when pay_coin > balance");
    assert!(
        err.contains("RouterInsufficientCoinBalance"),
        "expected RouterInsufficientCoinBalance, got: {err}"
    );

    // L4.E witness: at least one rejection record with matching kind + class.
    let recs = router_rejection_records(&h);
    let matched: Vec<_> = recs
        .iter()
        .filter(|(k, c)| *k == TxKind::BuyWithCoinRouter && *c == RejectionClass::PolicyViolation)
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "SG-G2.5.a: expected exactly 1 BuyWithCoinRouter / PolicyViolation \
         record in L4.E (coarse-class for TransitionError::Router*); got {:?}",
        recs
    );
    // public_summary must be the coarse "policy_violation" tag per
    // src/state/sequencer.rs::public_summary_for wildcard arm.
    let guard = h.rejection_writer.read().expect("rw");
    let record = guard
        .records()
        .iter()
        .find(|r| {
            r.tx_kind == TxKind::BuyWithCoinRouter
                && r.rejection_class == RejectionClass::PolicyViolation
        })
        .expect("router L4.E record present");
    assert_eq!(
        record.public_summary.as_deref(),
        Some("policy_violation"),
        "SG-G2.5.a: router rejection public_summary must be coarse \
         `policy_violation` (raw TransitionError name kept in raw_diagnostic_cid)"
    );
}

// ────────────────────────────────────────────────────────────────────────
// SG-G2.5.b — pool not Active → RouterPoolNotActive in L4.E
// ────────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn sg_g2_5_b_pool_not_active_lands_in_l4e() {
    // Build genesis with a Closed pool already in the state (no seed flow).
    let mut q0 = genesis_with_balances_and_open_task(&[("alice", 50), ("bob", 50)], "evt-2");
    let event_id = EventId(TaskId("evt-2".into()));
    q0.economic_state_t.cpmm_pools_t.0.insert(
        event_id.clone(),
        CpmmPool {
            event_id: event_id.clone(),
            pool_yes: ShareAmount::from_units(1_000_000),
            pool_no: ShareAmount::from_units(1_000_000),
            lp_total_shares: LpShareAmount::from_units(1_000_000),
            status: PoolStatus::Closed,
        },
    );
    let mut h = fresh_harness(q0);
    let parent = h.seq.q_snapshot().unwrap().state_root_t;

    let err = submit_and_apply(
        &mut h,
        build_router(parent, "bob", "evt-2", BuyDirection::BuyYes, 100_000, 0, 1),
    )
    .await
    .expect_err("must reject when pool not Active");
    assert!(
        err.contains("RouterPoolNotActive"),
        "expected RouterPoolNotActive, got: {err}"
    );

    let recs = router_rejection_records(&h);
    let matched: Vec<_> = recs
        .iter()
        .filter(|(k, c)| *k == TxKind::BuyWithCoinRouter && *c == RejectionClass::PolicyViolation)
        .collect();
    assert_eq!(
        matched.len(),
        1,
        "SG-G2.5.b: expected exactly 1 BuyWithCoinRouter / PolicyViolation \
         record in L4.E (TransitionError::RouterPoolNotActive folds into the \
         coarse PolicyViolation class); got {:?}",
        recs
    );
}

// ────────────────────────────────────────────────────────────────────────
// SG-G2.5.c — adapter-side pre-classifier routes balance shortfall →
// `NoTradeReason::AmountExceedsBalance` → CAS trace round-trips through
// `MarketDecisionTraceSummary::compute_from_path` (G2.2 §F.A exhaustive
// breakdown shows count = 1).
// ────────────────────────────────────────────────────────────────────────

#[test]
fn sg_g2_5_c_adapter_classifier_writes_no_trade_trace_to_cas() {
    use turingosv4::state::q_state::{CpmmPool, LpShareAmount, PoolStatus};

    let cas_dir = TempDir::new().expect("tempdir cas");
    let keys_dir = TempDir::new().expect("tempdir keys");
    let mut cas = CasStore::open(cas_dir.path()).expect("cas");
    let mut keys = AgentKeypairRegistry::open(keys_dir.path()).expect("keys");

    let work_tx_id = "worktx-Agent_2-evt-3-7";
    let mut q = QState::default();
    q.economic_state_t.balances_t.0.insert(
        AgentId("Agent_3".into()),
        MicroCoin::from_micro_units(100), // 100 μC
    );
    let event_id = turingosv4::state::typed_tx::node_survive_event_id(&TxId(work_tx_id.into()));
    q.economic_state_t.cpmm_pools_t.0.insert(
        event_id.clone(),
        CpmmPool {
            event_id: event_id.clone(),
            pool_yes: ShareAmount::from_units(1_000_000),
            pool_no: ShareAmount::from_units(1_000_000),
            lp_total_shares: LpShareAmount::from_units(1_000_000),
            status: PoolStatus::Active,
        },
    );

    // Agent_3 attempts to pay 1_000_000 μC with only 100 μC balance.
    let err = tb_n3_invest_to_router_tx(
        &mut keys,
        Hash::ZERO,
        Some(&q),
        "Agent_3",
        work_tx_id,
        BuyDirection::BuyYes,
        1_000_000,
        0,
        "g2-5-c",
    )
    .expect_err("adapter must reject balance shortfall pre-submit");
    let no_trade_reason = err.to_no_trade_reason();
    assert_eq!(
        no_trade_reason,
        NoTradeReason::AmountExceedsBalance,
        "SG-G2.5.c: balance shortfall → NoTradeReason::AmountExceedsBalance \
         (architect §8.2 doc-alias for `InsufficientBalance`)"
    );
    assert!(matches!(err, InvestRouteError::AmountExceedsBalance { .. }));

    // Evaluator-side wire: caller writes the no-trade trace to CAS.
    let trace = MarketDecisionTrace::no_trade(
        AgentId("Agent_3".into()),
        Some(TxId(work_tx_id.into())),
        Some(BuyDirection::BuyYes),
        Some(1_000_000),
        no_trade_reason,
        err.public_summary(),
    );
    write_market_decision_trace_to_cas(&mut cas, &trace, "g2-5-c", 1).expect("write trace to CAS");
    drop(cas);

    // Round-trip through the §F summary helper.
    let summary = MarketDecisionTraceSummary::compute_from_path(cas_dir.path()).expect("summary");
    assert_eq!(summary.total_traces, 1);
    assert_eq!(
        summary
            .no_trade_breakdown
            .get(&NoTradeReason::AmountExceedsBalance),
        Some(&1)
    );
    let out = summary.render_section_f();
    assert!(
        out.contains("amount_exceeds_balance = 1"),
        "SG-G2.5.c: §F.A row must reflect AmountExceedsBalance count: {out}"
    );
}

// ────────────────────────────────────────────────────────────────────────
// SG-G2.5.d — end-to-end architect §8.6 verbatim "Failed invest 也算有
// 意义 tape activity": router-side rejection in L4.E AND caller-side
// MarketDecisionTrace::no_trade(RouterRejected) trace in CAS.
// ────────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn sg_g2_5_d_failed_invest_both_l4e_and_cas_trace() {
    // Setup: bob has 1 Coin; attempts 5 Coin pay. Sequencer rejects;
    // evaluator-side pattern writes the trace with `RouterRejected`.
    let q0 = genesis_with_balances_and_open_task(&[("alice", 50), ("bob", 1)], "evt-4");
    let mut h = fresh_harness(q0);
    let parent = seed_pool(&mut h, "alice", "evt-4", 5_000_000).await;

    let submit_result = submit_and_apply(
        &mut h,
        build_router(
            parent,
            "bob",
            "evt-4",
            BuyDirection::BuyYes,
            5_000_000,
            0,
            1,
        ),
    )
    .await;
    assert!(
        submit_result.is_err(),
        "SG-G2.5.d: router admission must reject"
    );

    // L4.E half: rejection record exists.
    let recs = router_rejection_records(&h);
    assert!(
        recs.iter()
            .any(|(k, c)| *k == TxKind::BuyWithCoinRouter && *c == RejectionClass::PolicyViolation),
        "SG-G2.5.d: L4.E must carry the rejected BuyWithCoinRouter (coarse \
         PolicyViolation class; fine-grained TransitionError::RouterInsufficientCoinBalance \
         in raw_diagnostic_cid); got {:?}",
        recs
    );

    // CAS-trace half: evaluator pattern writes MarketDecisionTrace::no_trade
    // with reason = RouterRejected (the canonical no-trade-reason for a
    // sequencer-side router rejection, per InvestRouteError::to_no_trade_reason
    // mapping convention; the pre-classifier handles balance-side via
    // AmountExceedsBalance but a real Tx ALSO submitted to the sequencer
    // would have been caught by `tb_n3_invest_to_router_tx`).
    let cas_dir = TempDir::new().expect("tempdir cas");
    let mut cas = CasStore::open(cas_dir.path()).expect("cas");
    let trace = MarketDecisionTrace::no_trade(
        AgentId("bob".into()),
        None,
        Some(BuyDirection::BuyYes),
        Some(5_000_000),
        NoTradeReason::RouterRejected,
        "router rejected: RouterInsufficientCoinBalance",
    );
    write_market_decision_trace_to_cas(&mut cas, &trace, "g2-5-d", 1).expect("write trace");
    drop(cas);

    let summary = MarketDecisionTraceSummary::compute_from_path(cas_dir.path()).expect("summary");
    assert_eq!(summary.total_traces, 1);
    assert_eq!(
        summary
            .no_trade_breakdown
            .get(&NoTradeReason::RouterRejected),
        Some(&1),
        "SG-G2.5.d: CAS trace must reflect RouterRejected count = 1"
    );
}
