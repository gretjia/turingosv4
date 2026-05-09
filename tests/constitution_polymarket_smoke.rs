//! Constitution gate — Stage C P-M9 / Phase F.8 Controlled market smoke
//! (architect manual §7.10).
//!
//! Authority: `handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_en.md`
//! §7.10 (verbatim scenario + 5 mandatory gates).
//!
//! Architect §7.10 verbatim scenario:
//! ```
//! Lean task
//! Agent A WorkTx FirstLong
//! Agent B ChallengeTx Short
//! MarketSeedTx by sponsor or treasury
//! BuyYesWithCoin
//! BuyNoWithCoin
//! PriceIndex update
//! Task resolved
//! Redeem / merge
//! Autopsy if loss
//! ```
//!
//! Architect §7.10 verbatim gates:
//! ```
//! - no ghost liquidity
//! - total coin conserved
//! - no price-as-truth
//! - no raw log broadcast
//! - all activity replayable
//! ```
//!
//! This test exercises the Polymarket-specific path (mint + pool + 2
//! router buys + price quote + audit views). Lean task / WorkTx /
//! ChallengeTx / TerminalSummary lifecycle is already covered by TB-3 +
//! TB-4 + TB-7 + TB-11 + TB-18R suites; the smoke deliberately focuses on
//! the novel-to-Stage-C surface (P-M3 MarketSeed + P-M4 CpmmPool + P-M6
//! Router + P-M7 quote + P-M8 audit views) plus the global invariants
//! that span all atoms.

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
use turingosv4::economy::monetary_invariant::{
    assert_complete_set_balanced, assert_total_ctf_conserved,
};
use turingosv4::runtime::audit_views::{
    audit_view_pools, audit_view_positions, audit_view_prices,
    audit_view_shares, PriceLiquidityWarning,
};
use turingosv4::state::q_state::{
    AgentId, EconomicState, QState, TaskId, TaskMarketEntry, TaskMarketState, TxId,
};
use turingosv4::state::router_quote::{
    quote_buy_with_coin_router, LiquidityWarning, QuoteDirection,
};
use turingosv4::state::sequencer::{Sequencer, SubmissionEnvelope};
use turingosv4::state::typed_tx::{
    AgentSignature, BuyDirection, BuyWithCoinRouterTx, CompleteSetMintTx,
    CpmmPoolTx, EventId, ShareAmount, TypedTx,
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
    direction: BuyDirection,
    pay_micro: i64,
    seq_no: u64,
) -> TypedTx {
    TypedTx::BuyWithCoinRouter(BuyWithCoinRouterTx {
        tx_id: TxId(format!("router-{buyer}-{task}-{seq_no}-{:?}", direction)),
        parent_state_root: parent,
        event_id: EventId(TaskId(task.into())),
        buyer: AgentId(buyer.into()),
        direction,
        pay_coin: MicroCoin::from_micro_units(pay_micro),
        min_out_shares: ShareAmount::from_units(0),
        signature: AgentSignature::from_bytes([0u8; 64]),
    })
}

// Aggregate sums for a given event across (traders + pool).
fn sum_yes_for_event(econ: &EconomicState, task: &str) -> u128 {
    let event_id = EventId(TaskId(task.into()));
    let mut s: u128 = 0;
    for owner_map in econ.conditional_share_balances_t.0.values() {
        if let Some(pair) = owner_map.get(&event_id) {
            s += pair.yes.units;
        }
    }
    if let Some(pool) = econ.cpmm_pools_t.0.get(&event_id) {
        s += pool.pool_yes.units;
    }
    s
}
fn sum_no_for_event(econ: &EconomicState, task: &str) -> u128 {
    let event_id = EventId(TaskId(task.into()));
    let mut s: u128 = 0;
    for owner_map in econ.conditional_share_balances_t.0.values() {
        if let Some(pair) = owner_map.get(&event_id) {
            s += pair.no.units;
        }
    }
    if let Some(pool) = econ.cpmm_pools_t.0.get(&event_id) {
        s += pool.pool_no.units;
    }
    s
}

// ── Architect §7.10 verbatim smoke + 5 gate-invariant battery ───────────────

/// polymarket_controlled_market_smoke — architect §7.10 verbatim end-to-end
/// scenario over the Stage C Polymarket sequence (P-M3 + P-M4 + P-M6 +
/// P-M7 + P-M8). Drives:
/// 1. MarketSeedTx-equivalent: provider mints + creates symmetric pool.
/// 2. Two trader router buys (BuyYes + BuyNo) by distinct agents.
/// 3. PriceIndex updates (router quote signal).
/// 4. Audit views regenerate from canonical state.
///
/// Verifies architect §7.10 5 verbatim gates (post-smoke state):
/// - "no ghost liquidity": sum YES (traders + pool) == sum NO (traders +
///   pool) == collateral; no shares without locked Coin.
/// - "total coin conserved": assert_total_ctf_conserved with empty
///   exempt-list passes pre→post for each tx.
/// - "no price-as-truth": price_quote does not change state; the
///   sequencer admission arms have no router_quote import (witnessed
///   indirectly by P-M7 source-grep gate).
/// - "no raw log broadcast": this smoke does not exercise raw-log paths;
///   shielding gates land separately (TB-15 + Wave-3 binding).
/// - "all activity replayable": state_root advances monotonically;
///   audit views regenerate byte-identical from any snapshot.
#[tokio::test]
async fn polymarket_controlled_market_smoke() {
    // === Setup: 4-actor sandbox (provider + 2 traders + sponsor) ===
    let q0 = genesis_with_balances_and_open_task(
        &[
            ("alice", 100), // provider (will mint + seed pool)
            ("bob", 50),    // BuyYes trader
            ("carol", 50),  // BuyNo trader
        ],
        "polymarket-evt",
    );
    let mut h = fresh_harness(q0);

    // Capture genesis state for replay-determinism baseline.
    let q_genesis = h.seq.q_snapshot().unwrap();
    let state_root_genesis = q_genesis.state_root_t;
    let total_coin_pre_smoke = total_coin_micro(&q_genesis.economic_state_t);

    // === Step 1: provider mints 10M conditional shares (collateral lock) ===
    let p = h.seq.q_snapshot().unwrap().state_root_t;
    let q_pre_mint = h.seq.q_snapshot().unwrap();
    submit_and_apply(
        &mut h,
        build_mint(p, "alice", "polymarket-evt", 10_000_000, 1),
    )
    .await
    .expect("mint accepted");

    // Per-step gate witnesses.
    let q_post_mint = h.seq.q_snapshot().unwrap();
    assert_total_ctf_conserved(
        &q_pre_mint.economic_state_t,
        &q_post_mint.economic_state_t,
        &[],
    )
    .expect("Coin conserved across MarketSeed-equivalent mint");
    assert_complete_set_balanced(&q_post_mint.economic_state_t)
        .expect("complete-set balanced post-mint");

    // === Step 2: provider creates 5M/5M pool ===
    let p = h.seq.q_snapshot().unwrap().state_root_t;
    let q_pre_pool = h.seq.q_snapshot().unwrap();
    submit_and_apply(
        &mut h,
        build_pool(p, "alice", "polymarket-evt", 5_000_000, 1),
    )
    .await
    .expect("pool accepted");

    let q_post_pool = h.seq.q_snapshot().unwrap();
    assert_total_ctf_conserved(
        &q_pre_pool.economic_state_t,
        &q_post_pool.economic_state_t,
        &[],
    )
    .expect("Coin conserved across pool create");
    assert_complete_set_balanced(&q_post_pool.economic_state_t)
        .expect("complete-set balanced post-pool");

    // Architect §7.10 gate 1: "no ghost liquidity" — sum YES == sum NO ==
    // collateral. Witnessed by the symmetric branch of
    // assert_complete_set_balanced (above). Direct cross-check:
    let coll_post_pool = q_post_pool
        .economic_state_t
        .conditional_collateral_t
        .0
        .get(&EventId(TaskId("polymarket-evt".into())))
        .copied()
        .unwrap();
    assert_eq!(coll_post_pool.micro_units(), 10_000_000);
    let sum_yes = sum_yes_for_event(&q_post_pool.economic_state_t, "polymarket-evt");
    let sum_no = sum_no_for_event(&q_post_pool.economic_state_t, "polymarket-evt");
    assert_eq!(sum_yes, sum_no);
    assert_eq!(sum_yes, 10_000_000);

    // === Step 3: Bob BuyYesWithCoin (payC = 1M) ===
    let parent = h.seq.q_snapshot().unwrap().state_root_t;
    let q_pre_bob = h.seq.q_snapshot().unwrap();
    submit_and_apply(
        &mut h,
        build_router(
            parent,
            "bob",
            "polymarket-evt",
            BuyDirection::BuyYes,
            1_000_000,
            1,
        ),
    )
    .await
    .expect("BuyYesWithCoin accepted");

    let q_post_bob = h.seq.q_snapshot().unwrap();
    assert_total_ctf_conserved(
        &q_pre_bob.economic_state_t,
        &q_post_bob.economic_state_t,
        &[],
    )
    .expect("Coin conserved across BuyYesWithCoin");
    assert_complete_set_balanced(&q_post_bob.economic_state_t)
        .expect("complete-set balanced post BuyYes");

    // === Step 4: Carol BuyNoWithCoin (payC = 500K) ===
    let parent = h.seq.q_snapshot().unwrap().state_root_t;
    let q_pre_carol = h.seq.q_snapshot().unwrap();
    submit_and_apply(
        &mut h,
        build_router(
            parent,
            "carol",
            "polymarket-evt",
            BuyDirection::BuyNo,
            500_000,
            2,
        ),
    )
    .await
    .expect("BuyNoWithCoin accepted");

    let q_post_carol = h.seq.q_snapshot().unwrap();
    assert_total_ctf_conserved(
        &q_pre_carol.economic_state_t,
        &q_post_carol.economic_state_t,
        &[],
    )
    .expect("Coin conserved across BuyNoWithCoin");
    assert_complete_set_balanced(&q_post_carol.economic_state_t)
        .expect("complete-set balanced post BuyNo");

    // === Step 5: PriceIndex / quote update (P-M7) ===
    let q_post = q_post_carol.clone();
    let pool_post = q_post
        .economic_state_t
        .cpmm_pools_t
        .0
        .get(&EventId(TaskId("polymarket-evt".into())))
        .cloned()
        .expect("pool present post-router");

    // Quote both directions for a sample payC. Quote MUST NOT mutate state
    // (architect §7.8 + P-M7 gate).
    let state_root_pre_quote = q_post.state_root_t;
    for &dir in &[QuoteDirection::BuyYes, QuoteDirection::BuyNo] {
        let q = quote_buy_with_coin_router(
            &pool_post,
            MicroCoin::from_micro_units(1_000_000),
            dir,
        )
        .expect("healthy quote");
        assert!(q.out_shares.units > 0);
        assert!(q.price_effective.is_some());
        assert_eq!(q.liquidity_warning, LiquidityWarning::None);
    }
    let q_after_quote = h.seq.q_snapshot().unwrap();
    assert_eq!(
        q_after_quote.state_root_t, state_root_pre_quote,
        "architect §7.10 gate 'no price-as-truth': quote does not advance state"
    );

    // === Step 6: Audit views regenerate from canonical state (P-M8) ===
    let view_shares = audit_view_shares(&q_post.economic_state_t);
    let view_pools = audit_view_pools(&q_post.economic_state_t);
    let view_prices = audit_view_prices(
        &q_post.economic_state_t,
        &[
            MicroCoin::from_micro_units(100_000),
            MicroCoin::from_micro_units(1_000_000),
        ],
    );
    let view_positions = audit_view_positions(&q_post.economic_state_t);

    // Shares view: bob has YES from BuyYes (1M + outY); carol has NO from
    // BuyNo (500K + outN); alice has 0/0 for this event (pool seed drained
    // her inventory completely). Filtered rows mean alice's empty entry
    // doesn't appear.
    let bob_row = view_shares
        .owner_shares
        .iter()
        .find(|r| r.owner == AgentId("bob".into()) && r.event_id == EventId(TaskId("polymarket-evt".into())))
        .expect("bob has shares");
    assert!(bob_row.yes_units > 1_000_000, "bob got payC + outY YES");
    assert_eq!(bob_row.no_units, 0);
    let carol_row = view_shares
        .owner_shares
        .iter()
        .find(|r| r.owner == AgentId("carol".into()) && r.event_id == EventId(TaskId("polymarket-evt".into())))
        .expect("carol has shares");
    assert!(carol_row.no_units > 500_000, "carol got payC + outN NO");
    assert_eq!(carol_row.yes_units, 0);

    // Pools view: 1 active pool with k_product non-decreasing across both
    // router buys (architect §7.6 floor invariant; preserved via integer
    // math).
    assert_eq!(view_pools.pools.len(), 1);
    let pool_row = &view_pools.pools[0];
    let k_post_smoke = pool_row.k_product;
    let k_pool_seed = 5_000_000_u128 * 5_000_000_u128;
    assert!(
        k_post_smoke >= k_pool_seed,
        "architect §7.6 constant-product invariant: k must be non-decreasing across swaps"
    );

    // LP holdings: alice (provider) holds 5M LP units 1:1 with seed.
    assert_eq!(view_pools.lp_holdings.len(), 1);
    assert_eq!(view_pools.lp_holdings[0].lp_units, 5_000_000);
    assert_eq!(
        view_pools.lp_holdings[0].provider,
        AgentId("alice".into())
    );

    // Prices view: 1 active pool × 2 sample sizes × 2 directions = 4 rows.
    assert_eq!(view_prices.price_quotes.len(), 4);
    for row in &view_prices.price_quotes {
        // Price quote MUST be defined (asymmetric pool ratio post-2-router
        // buys is non-degenerate).
        assert!(
            matches!(row.liquidity_warning, PriceLiquidityWarning::None | PriceLiquidityWarning::LowLiquidity),
            "post-smoke pool yields a defined quote (no NoOutput warning)"
        );
        assert!(row.out_shares_units > 0);
        assert!(row.get_shares_units > row.pay_coin_micro as u128);
    }

    // Positions view: empty (no WorkTx / ChallengeTx in this smoke).
    assert!(view_positions.positions.is_empty());

    // === Architect §7.10 verbatim 5-gate battery ===

    // Gate 1: "no ghost liquidity"
    let coll = q_post
        .economic_state_t
        .conditional_collateral_t
        .0
        .get(&EventId(TaskId("polymarket-evt".into())))
        .copied()
        .unwrap();
    let sum_yes_post = sum_yes_for_event(&q_post.economic_state_t, "polymarket-evt");
    let sum_no_post = sum_no_for_event(&q_post.economic_state_t, "polymarket-evt");
    assert_eq!(
        sum_yes_post, sum_no_post,
        "no ghost liquidity (sum YES == sum NO)"
    );
    assert_eq!(
        sum_yes_post, coll.micro_units() as u128,
        "no ghost liquidity (sum YES == collateral)"
    );

    // Gate 2: "total coin conserved" — already witnessed at each step;
    // verify global pre-smoke → post-smoke too.
    let total_coin_post_smoke = total_coin_micro(&q_post.economic_state_t);
    assert_eq!(
        total_coin_post_smoke, total_coin_pre_smoke,
        "total coin conserved across full smoke (pre-smoke == post-smoke)"
    );

    // Gate 3: "no price-as-truth" — quote does not advance state (above).
    // Source-grep gate (P-M7 `price_signal_not_predicate` test) verifies
    // sequencer/predicate code does not import router_quote module.

    // Gate 4: "no raw log broadcast" — this smoke does not exercise any
    // raw-log paths; shielding is enforced by separate Wave-3 + TB-15
    // binding gates.

    // Gate 5: "all activity replayable" — state_root advanced
    // monotonically; views regenerate byte-identical from same snapshot.
    assert_ne!(q_post.state_root_t, state_root_genesis);
    let view_pools_again = audit_view_pools(&q_post.economic_state_t);
    assert_eq!(
        view_pools, view_pools_again,
        "audit views regenerate byte-identical (replay-deterministic)"
    );
    let view_shares_again = audit_view_shares(&q_post.economic_state_t);
    assert_eq!(view_shares, view_shares_again);
    let view_prices_again = audit_view_prices(
        &q_post.economic_state_t,
        &[
            MicroCoin::from_micro_units(100_000),
            MicroCoin::from_micro_units(1_000_000),
        ],
    );
    assert_eq!(view_prices, view_prices_again);
}

// total_coin_micro — sum of the 5 Coin-bearing holdings in EconomicState
// (mirrors `monetary_invariant::total_supply_micro` semantics for the
// smoke test cross-check; intentionally NOT calling private helper to
// keep the smoke independent from the invariant module's internal shape).
fn total_coin_micro(econ: &EconomicState) -> i128 {
    let mut s: i128 = 0;
    for v in econ.balances_t.0.values() {
        s += v.micro_units() as i128;
    }
    for esc in econ.escrows_t.0.values() {
        s += esc.amount.micro_units() as i128;
    }
    for stk in econ.stakes_t.0.values() {
        s += stk.amount.micro_units() as i128;
    }
    for cc in econ.challenge_cases_t.0.values() {
        s += cc.bond.micro_units() as i128;
    }
    for v in econ.conditional_collateral_t.0.values() {
        s += v.micro_units() as i128;
    }
    s
}
