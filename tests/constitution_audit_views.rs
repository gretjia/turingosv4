//! Constitution gate — Stage C P-M8 / Phase F.7 audit views (architect manual §7.9).
//!
//! Authority: `handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_en.md`
//! §7.9 (verbatim "Add: audit_tape view-shares / view-pools / view-prices /
//! view-positions. Must show: owner YES/NO shares, conditional collateral,
//! pool reserves, LP shares, NodePositions, price signal").
//!
//! Test names mirror architect §7.9 verbatim list:
//!   1. audit_view_shares_matches_state
//!   2. audit_view_pools_matches_state
//!   3. dashboard_regenerates_market_view
//!
//! All tests apply real txs (mint + pool + router) through the live
//! sequencer + harness, then call view aggregator pure-fns and assert
//! the views match the underlying `EconomicState` directly. Per
//! `feedback_tape_first_real_tests`: tape-first; views derived from
//! canonical state, not stdout / private logs.

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
use turingosv4::runtime::audit_views::{
    audit_view_pools, audit_view_positions, audit_view_prices,
    audit_view_shares, PriceLiquidityWarning,
};
use turingosv4::state::q_state::{
    AgentId, QState, TaskId, TaskMarketEntry, TaskMarketState, TxId,
};
use turingosv4::state::sequencer::{Sequencer, SubmissionEnvelope};
use turingosv4::state::typed_tx::{
    AgentSignature, BuyDirection, BuyWithCoinRouterTx, CompleteSetMintTx,
    CpmmPoolTx, EventId, ShareAmount, TypedTx,
};
use turingosv4::top_white::predicates::registry::PredicateRegistry;

// ── Harness ──────────────────────────────────────────────────────────────

struct Harness {
    _tmp: TempDir,
    seq: Sequencer,
    rx: tokio::sync::mpsc::Receiver<SubmissionEnvelope>,
    _ledger: Arc<RwLock<dyn LedgerWriter>>,
}

fn fresh_harness(initial_q: QState) -> Harness {
    let tmp = TempDir::new().expect("tempdir");
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
    let (seq, rx) = Sequencer::new(
        cas, keypair, epoch, writer.clone(), rejection_writer, preds, tools,
        pinned_pubkeys, initial_q, 16,
    );
    Harness { _tmp: tmp, seq, rx, _ledger: writer }
}

fn genesis_with_balances_and_open_task(
    pairs: &[(&str, i64)],
    task: &str,
) -> QState {
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
        .map(|_ledger_entry| ())
        .map_err(|e| format!("apply error: {e:?}"))
}

fn build_mint(
    parent: turingosv4::state::q_state::Hash,
    owner: &str,
    task: &str,
    micro: i64,
    seq_no: u64,
) -> TypedTx {
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
    parent: turingosv4::state::q_state::Hash,
    provider: &str,
    task: &str,
    seed_units: u128,
    seq_no: u64,
) -> TypedTx {
    TypedTx::CpmmPool(CpmmPoolTx {
        tx_id: TxId(format!("pool-{provider}-{task}-{seq_no}")),
        parent_state_root: parent,
        event_id: EventId(TaskId(task.into())),
        provider: AgentId(provider.into()),
        seed_yes: ShareAmount::from_units(seed_units),
        seed_no: ShareAmount::from_units(seed_units),
        signature: AgentSignature::from_bytes([0u8; 64]),
    })
}

fn build_router(
    parent: turingosv4::state::q_state::Hash,
    buyer: &str,
    task: &str,
    pay_micro: i64,
    seq_no: u64,
) -> TypedTx {
    TypedTx::BuyWithCoinRouter(BuyWithCoinRouterTx {
        tx_id: TxId(format!("router-{buyer}-{task}-{seq_no}")),
        parent_state_root: parent,
        event_id: EventId(TaskId(task.into())),
        buyer: AgentId(buyer.into()),
        direction: BuyDirection::BuyYes,
        pay_coin: MicroCoin::from_micro_units(pay_micro),
        min_out_shares: ShareAmount::from_units(0),
        signature: AgentSignature::from_bytes([0u8; 64]),
    })
}

// ── Architect §7.9 verbatim test 1 ──────────────────────────────────────────

/// audit_view_shares_matches_state — the SharesView aggregator MUST mirror
/// the underlying `EconomicState.conditional_share_balances_t` and
/// `conditional_collateral_t` exactly. Witness: apply real
/// CompleteSetMintTx; SharesView contains the minted shares; collateral
/// matches mint amount.
#[tokio::test]
async fn audit_view_shares_matches_state() {
    let q0 = genesis_with_balances_and_open_task(&[("alice", 50)], "evt-1");
    let mut h = fresh_harness(q0);

    let p = h.seq.q_snapshot().unwrap().state_root_t;
    submit_and_apply(&mut h, build_mint(p, "alice", "evt-1", 5_000_000, 1))
        .await
        .expect("mint accepted");

    let q = h.seq.q_snapshot().unwrap();
    let view = audit_view_shares(&q.economic_state_t);

    // Owner shares: alice has 5M YES + 5M NO post-mint.
    assert_eq!(view.owner_shares.len(), 1);
    let row = &view.owner_shares[0];
    assert_eq!(row.owner, AgentId("alice".into()));
    assert_eq!(row.event_id, EventId(TaskId("evt-1".into())));
    assert_eq!(row.yes_units, 5_000_000);
    assert_eq!(row.no_units, 5_000_000);

    // Conditional collateral: 5M micro-Coin locked.
    assert_eq!(view.conditional_collateral.len(), 1);
    assert_eq!(
        view.conditional_collateral[0].locked_micro_coin,
        5_000_000
    );
    assert_eq!(
        view.conditional_collateral[0].event_id,
        EventId(TaskId("evt-1".into()))
    );

    // Cross-check: view fields match underlying state directly.
    let direct_pair = q
        .economic_state_t
        .conditional_share_balances_t
        .0
        .get(&AgentId("alice".into()))
        .and_then(|m| m.get(&EventId(TaskId("evt-1".into()))))
        .copied()
        .unwrap();
    assert_eq!(row.yes_units, direct_pair.yes.units);
    assert_eq!(row.no_units, direct_pair.no.units);
    let direct_coll = q
        .economic_state_t
        .conditional_collateral_t
        .0
        .get(&EventId(TaskId("evt-1".into())))
        .copied()
        .unwrap();
    assert_eq!(
        view.conditional_collateral[0].locked_micro_coin,
        direct_coll.micro_units()
    );
}

// ── Architect §7.9 verbatim test 2 ──────────────────────────────────────────

/// audit_view_pools_matches_state — the PoolsView aggregator MUST mirror
/// the underlying `EconomicState.cpmm_pools_t` and `lp_share_balances_t`
/// exactly. Witness: apply CompleteSetMint + CpmmPoolTx; PoolsView
/// contains the pool reserves + LP holdings; k_product computed
/// correctly.
#[tokio::test]
async fn audit_view_pools_matches_state() {
    let q0 = genesis_with_balances_and_open_task(&[("alice", 50)], "evt-2");
    let mut h = fresh_harness(q0);

    let p = h.seq.q_snapshot().unwrap().state_root_t;
    submit_and_apply(&mut h, build_mint(p, "alice", "evt-2", 5_000_000, 1))
        .await
        .expect("mint accepted");
    let p = h.seq.q_snapshot().unwrap().state_root_t;
    submit_and_apply(&mut h, build_pool(p, "alice", "evt-2", 5_000_000, 1))
        .await
        .expect("pool accepted");

    let q = h.seq.q_snapshot().unwrap();
    let view = audit_view_pools(&q.economic_state_t);

    // Pools: 1 active pool with 5M/5M reserves; k = 25e12.
    assert_eq!(view.pools.len(), 1);
    let pool_row = &view.pools[0];
    assert_eq!(pool_row.event_id, EventId(TaskId("evt-2".into())));
    assert_eq!(pool_row.pool_yes_units, 5_000_000);
    assert_eq!(pool_row.pool_no_units, 5_000_000);
    assert_eq!(pool_row.lp_total_shares_units, 5_000_000);
    assert_eq!(pool_row.k_product, 5_000_000_u128 * 5_000_000_u128);

    // LP holdings: alice (provider) holds 5M LP units 1:1 with seed.
    assert_eq!(view.lp_holdings.len(), 1);
    let lp_row = &view.lp_holdings[0];
    assert_eq!(lp_row.provider, AgentId("alice".into()));
    assert_eq!(lp_row.event_id, EventId(TaskId("evt-2".into())));
    assert_eq!(lp_row.lp_units, 5_000_000);

    // Cross-check pool view against state directly.
    let direct_pool = q
        .economic_state_t
        .cpmm_pools_t
        .0
        .get(&EventId(TaskId("evt-2".into())))
        .cloned()
        .unwrap();
    assert_eq!(pool_row.pool_yes_units, direct_pool.pool_yes.units);
    assert_eq!(pool_row.pool_no_units, direct_pool.pool_no.units);
    assert_eq!(pool_row.lp_total_shares_units, direct_pool.lp_total_shares.units);
}

// ── Architect §7.9 verbatim test 3 ──────────────────────────────────────────

/// dashboard_regenerates_market_view — architect §7.9 + dashboard
/// discipline (CLAUDE.md §17): views are pure-fn over `EconomicState`;
/// regenerating the views from any post-tx snapshot yields the same
/// result. Witness: take 3 snapshots (pre-pool / post-pool / post-router);
/// regenerate views from each; assert views differ across snapshots
/// (state changed) AND each snapshot's view is reproducible.
#[tokio::test]
async fn dashboard_regenerates_market_view() {
    let q0 = genesis_with_balances_and_open_task(
        &[("alice", 50), ("bob", 50)],
        "evt-3",
    );
    let mut h = fresh_harness(q0);

    // Snapshot 1: pre-anything (empty state).
    let q_pre = h.seq.q_snapshot().unwrap();
    let view_pre = audit_view_pools(&q_pre.economic_state_t);
    assert!(view_pre.pools.is_empty());

    // Apply mint + pool.
    let p = h.seq.q_snapshot().unwrap().state_root_t;
    submit_and_apply(&mut h, build_mint(p, "alice", "evt-3", 5_000_000, 1))
        .await
        .expect("mint accepted");
    let p = h.seq.q_snapshot().unwrap().state_root_t;
    submit_and_apply(&mut h, build_pool(p, "alice", "evt-3", 5_000_000, 1))
        .await
        .expect("pool accepted");

    // Snapshot 2: post-pool.
    let q_post_pool = h.seq.q_snapshot().unwrap();
    let view_post_pool = audit_view_pools(&q_post_pool.economic_state_t);
    assert_eq!(view_post_pool.pools.len(), 1);
    assert_eq!(view_post_pool.pools[0].pool_yes_units, 5_000_000);
    assert_eq!(view_post_pool.pools[0].pool_no_units, 5_000_000);

    // Apply router (BuyYes payC = 1M; outY = 833_333).
    let parent = h.seq.q_snapshot().unwrap().state_root_t;
    submit_and_apply(
        &mut h,
        build_router(parent, "bob", "evt-3", 1_000_000, 1),
    )
    .await
    .expect("router accepted");

    // Snapshot 3: post-router.
    let q_post_router = h.seq.q_snapshot().unwrap();
    let view_post_router = audit_view_pools(&q_post_router.economic_state_t);
    assert_eq!(view_post_router.pools.len(), 1);
    assert_eq!(
        view_post_router.pools[0].pool_yes_units,
        5_000_000 - 833_333,
        "post-router pool_yes shifted by outY"
    );
    assert_eq!(
        view_post_router.pools[0].pool_no_units,
        6_000_000,
        "post-router pool_no shifted by payC"
    );

    // Regenerate views from same snapshots — must be byte-identical.
    let view_post_pool_again = audit_view_pools(&q_post_pool.economic_state_t);
    assert_eq!(
        view_post_pool, view_post_pool_again,
        "view regenerates byte-identical from same snapshot"
    );
    let view_post_router_again = audit_view_pools(&q_post_router.economic_state_t);
    assert_eq!(
        view_post_router, view_post_router_again,
        "view regenerates byte-identical from same snapshot"
    );

    // Cross-view: prices view from post-router state must reflect the new
    // (asymmetric) pool ratio.
    let prices_view = audit_view_prices(
        &q_post_router.economic_state_t,
        &[MicroCoin::from_micro_units(1_000_000)],
    );
    // 1 active pool × 1 pay_coin sample × 2 directions = 2 rows.
    assert_eq!(prices_view.price_quotes.len(), 2);
    for row in &prices_view.price_quotes {
        assert_eq!(row.pay_coin_micro, 1_000_000);
        assert!(row.out_shares_units > 0);
        assert!(matches!(
            row.liquidity_warning,
            PriceLiquidityWarning::None | PriceLiquidityWarning::LowLiquidity
        ));
    }

    // Positions view: should be empty (no WorkTx / ChallengeTx submitted
    // in this test).
    let positions_view = audit_view_positions(&q_post_router.economic_state_t);
    assert!(positions_view.positions.is_empty());

    // Shares view: alice has remaining shares (post pool seed); bob has
    // gained from router.
    let shares_view = audit_view_shares(&q_post_router.economic_state_t);
    let alice_row = shares_view
        .owner_shares
        .iter()
        .find(|r| r.owner == AgentId("alice".into()));
    // After mint(5M) + pool seed(5M debit), alice has 0/0 — empty rows
    // are filtered by audit_view_shares.
    assert!(alice_row.is_none() || alice_row.unwrap().yes_units == 0);
    let bob_row = shares_view
        .owner_shares
        .iter()
        .find(|r| r.owner == AgentId("bob".into()))
        .expect("bob has shares post-router");
    // BuyYes payC=1M: bob gets payC + outY = 1_833_333 YES; 0 NO.
    assert_eq!(bob_row.yes_units, 1_833_333);
    assert_eq!(bob_row.no_units, 0);
}
