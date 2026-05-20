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
use turingosv4::bottom_white::ledger::transition_ledger::{InMemoryLedgerWriter, LedgerWriter};
use turingosv4::bottom_white::tools::registry::ToolRegistry;
use turingosv4::economy::monetary_invariant::{
    assert_complete_set_balanced, assert_total_ctf_conserved,
};
use turingosv4::economy::money::MicroCoin;
use turingosv4::runtime::audit_views::{
    audit_view_pools, audit_view_positions, audit_view_prices, audit_view_shares,
    PriceLiquidityWarning,
};
use turingosv4::state::q_state::{
    AgentId, EconomicState, QState, TaskId, TaskMarketEntry, TaskMarketState, TxId,
};
use turingosv4::state::router_quote::{
    quote_buy_with_coin_router, LiquidityWarning, QuoteDirection,
};
use turingosv4::state::sequencer::{Sequencer, SubmissionEnvelope, SystemEmitCommand};
use turingosv4::state::typed_tx::{
    AgentSignature, BuyDirection, BuyWithCoinRouterTx, CompleteSetMintTx, CompleteSetRedeemTx,
    CpmmPoolTx, EventId, OutcomeSide, ShareAmount, TypedTx,
};
use turingosv4::top_white::predicates::registry::PredicateRegistry;

// ── Harness ─────────────────────────────────────────────────────────────────

mod support;
use support::{fresh_harness, Harness};

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
        .map(|_ledger_entry| ())
        .map_err(|e| format!("apply error: {e:?}"))
}

async fn expect_router_rejection_preserves_state(
    h: &mut Harness,
    tx: TypedTx,
    expected_error: &str,
) {
    let q_pre = h.seq.q_snapshot().expect("snapshot before rejection");
    let coin_pre = total_coin_micro(&q_pre.economic_state_t);
    let err = submit_and_apply(h, tx)
        .await
        .expect_err("router tx must reject");
    assert!(
        err.contains(expected_error),
        "expected {expected_error}, got {err}"
    );
    let q_post = h.seq.q_snapshot().expect("snapshot after rejection");
    assert_eq!(
        q_post.state_root_t, q_pre.state_root_t,
        "rejected router tx must not advance state root"
    );
    assert_eq!(
        total_coin_micro(&q_post.economic_state_t),
        coin_pre,
        "rejected router tx must not mutate total Coin supply"
    );
}

async fn expect_redeem_rejection_preserves_state(
    h: &mut Harness,
    tx: TypedTx,
    expected_error: &str,
) {
    let q_pre = h
        .seq
        .q_snapshot()
        .expect("snapshot before redeem rejection");
    let coin_pre = total_coin_micro(&q_pre.economic_state_t);
    let err = submit_and_apply(h, tx)
        .await
        .expect_err("redeem tx must reject");
    assert!(
        err.contains(expected_error),
        "expected {expected_error}, got {err}"
    );
    let q_post = h.seq.q_snapshot().expect("snapshot after redeem rejection");
    assert_eq!(
        q_post.state_root_t, q_pre.state_root_t,
        "rejected redeem tx must not advance state root"
    );
    assert_eq!(
        total_coin_micro(&q_post.economic_state_t),
        coin_pre,
        "rejected redeem tx must not mutate total Coin supply"
    );
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

fn build_redeem(
    parent: turingosv4::state::q_state::Hash,
    owner: &str,
    task: &str,
    outcome: OutcomeSide,
    units: u128,
    seq_no: u64,
) -> TypedTx {
    TypedTx::CompleteSetRedeem(CompleteSetRedeemTx {
        tx_id: TxId(format!("redeem-{owner}-{task}-{seq_no}")),
        parent_state_root: parent,
        event_id: EventId(TaskId(task.into())),
        owner: AgentId(owner.into()),
        outcome,
        share_amount: ShareAmount::from_units(units),
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: 2000 + seq_no,
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

fn agent_side_units(
    econ: &EconomicState,
    owner: &str,
    task: &str,
    direction: BuyDirection,
) -> u128 {
    let event_id = EventId(TaskId(task.into()));
    let owner = AgentId(owner.into());
    econ.conditional_share_balances_t
        .0
        .get(&owner)
        .and_then(|by_event| by_event.get(&event_id))
        .map(|pair| match direction {
            BuyDirection::BuyYes => pair.yes.units,
            BuyDirection::BuyNo => pair.no.units,
        })
        .unwrap_or(0)
}

fn pool_k(econ: &EconomicState, task: &str) -> u128 {
    let event_id = EventId(TaskId(task.into()));
    let pool = econ.cpmm_pools_t.0.get(&event_id).expect("pool present");
    pool.pool_yes.units * pool.pool_no.units
}

async fn redeem_and_assert_payout(
    h: &mut Harness,
    owner: &str,
    task: &str,
    outcome: OutcomeSide,
    direction: BuyDirection,
    share_units: u128,
    seq_no: u64,
) {
    let event_id = EventId(TaskId(task.into()));
    let q_pre = h.seq.q_snapshot().expect("snapshot before redeem");
    let coin_pre = total_coin_micro(&q_pre.economic_state_t);
    let balance_pre = q_pre
        .economic_state_t
        .balances_t
        .0
        .get(&AgentId(owner.into()))
        .copied()
        .expect("owner balance before redeem");
    let collateral_pre = q_pre
        .economic_state_t
        .conditional_collateral_t
        .0
        .get(&event_id)
        .copied()
        .expect("collateral before redeem");
    let side_pre = agent_side_units(&q_pre.economic_state_t, owner, task, direction);

    submit_and_apply(
        h,
        build_redeem(
            q_pre.state_root_t,
            owner,
            task,
            outcome,
            share_units,
            seq_no,
        ),
    )
    .await
    .expect("winning shares redeem accepted");

    let q_post = h.seq.q_snapshot().expect("snapshot after redeem");
    assert_total_ctf_conserved(&q_pre.economic_state_t, &q_post.economic_state_t, &[])
        .expect("Coin conserved across redeem");
    assert_complete_set_balanced(&q_post.economic_state_t)
        .expect("complete-set balanced after redeem");
    let balance_post = q_post
        .economic_state_t
        .balances_t
        .0
        .get(&AgentId(owner.into()))
        .copied()
        .expect("owner balance after redeem");
    assert_eq!(
        balance_post.micro_units() - balance_pre.micro_units(),
        share_units as i64,
        "winning shares redeem 1:1 to Coin"
    );
    let collateral_post = q_post
        .economic_state_t
        .conditional_collateral_t
        .0
        .get(&event_id)
        .copied()
        .expect("collateral after redeem");
    assert_eq!(
        collateral_pre.micro_units() - collateral_post.micro_units(),
        share_units as i64,
        "redeem debits event collateral 1:1"
    );
    let side_post = agent_side_units(&q_post.economic_state_t, owner, task, direction);
    assert_eq!(
        side_pre - side_post,
        share_units,
        "redeem burns winning-side shares"
    );
    assert_eq!(total_coin_micro(&q_post.economic_state_t), coin_pre);
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
        let q = quote_buy_with_coin_router(&pool_post, MicroCoin::from_micro_units(1_000_000), dir)
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
        .find(|r| {
            r.owner == AgentId("bob".into())
                && r.event_id == EventId(TaskId("polymarket-evt".into()))
        })
        .expect("bob has shares");
    assert!(bob_row.yes_units > 1_000_000, "bob got payC + outY YES");
    assert_eq!(bob_row.no_units, 0);
    let carol_row = view_shares
        .owner_shares
        .iter()
        .find(|r| {
            r.owner == AgentId("carol".into())
                && r.event_id == EventId(TaskId("polymarket-evt".into()))
        })
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
    assert_eq!(view_pools.lp_holdings[0].provider, AgentId("alice".into()));

    // Prices view: 1 active pool × 2 sample sizes × 2 directions = 4 rows.
    assert_eq!(view_prices.price_quotes.len(), 4);
    for row in &view_prices.price_quotes {
        // Price quote MUST be defined (asymmetric pool ratio post-2-router
        // buys is non-degenerate).
        assert!(
            matches!(
                row.liquidity_warning,
                PriceLiquidityWarning::None | PriceLiquidityWarning::LowLiquidity
            ),
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
        sum_yes_post,
        coll.micro_units() as u128,
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

/// REAL-17 robustness positive-control: deterministic Bull/Bear role pressure
/// drives both router directions through tiny, medium, and large orders. This
/// is a code robustness gate only. Because the sequence is harness-forced, it
/// is not E2, not voluntary emergence evidence, and not a ship claim.
#[tokio::test]
async fn red_track_forced_bull_bear_router_sequence_preserves_polymarket_invariants() {
    let task = "polymarket-forced-robustness";
    let event_id = EventId(TaskId(task.into()));
    let q0 = genesis_with_balances_and_open_task(
        &[("maker", 500), ("forced_bull", 200), ("forced_bear", 200)],
        task,
    );
    let mut h = fresh_harness(q0);

    let p = h.seq.q_snapshot().unwrap().state_root_t;
    submit_and_apply(&mut h, build_mint(p, "maker", task, 100_000_000, 1))
        .await
        .expect("maker mint accepted");
    let p = h.seq.q_snapshot().unwrap().state_root_t;
    submit_and_apply(&mut h, build_pool(p, "maker", task, 50_000_000, 1))
        .await
        .expect("maker pool accepted");

    let q_seed = h.seq.q_snapshot().unwrap();
    let total_coin_pre = total_coin_micro(&q_seed.economic_state_t);
    assert_complete_set_balanced(&q_seed.economic_state_t).expect("seed balanced");

    let forced_sequence: [(&str, BuyDirection, i64); 12] = [
        ("forced_bull", BuyDirection::BuyYes, 1),
        ("forced_bear", BuyDirection::BuyNo, 1),
        ("forced_bull", BuyDirection::BuyYes, 10_000),
        ("forced_bear", BuyDirection::BuyNo, 10_000),
        ("forced_bull", BuyDirection::BuyYes, 100_000),
        ("forced_bear", BuyDirection::BuyNo, 100_000),
        ("forced_bull", BuyDirection::BuyYes, 1_000_000),
        ("forced_bear", BuyDirection::BuyNo, 1_000_000),
        ("forced_bull", BuyDirection::BuyYes, 5_000_000),
        ("forced_bear", BuyDirection::BuyNo, 5_000_000),
        ("forced_bull", BuyDirection::BuyYes, 10_000_000),
        ("forced_bear", BuyDirection::BuyNo, 10_000_000),
    ];

    let mut previous_k = pool_k(&q_seed.economic_state_t, task);
    for (idx, (actor, direction, pay_micro)) in forced_sequence.into_iter().enumerate() {
        let q_pre = h.seq.q_snapshot().unwrap();
        let balance_pre = q_pre
            .economic_state_t
            .balances_t
            .0
            .get(&AgentId(actor.into()))
            .copied()
            .expect("actor balance");
        let side_units_pre = agent_side_units(&q_pre.economic_state_t, actor, task, direction);
        let pool_pre = q_pre
            .economic_state_t
            .cpmm_pools_t
            .0
            .get(&event_id)
            .cloned()
            .expect("pool before forced router");
        let quote_direction = match direction {
            BuyDirection::BuyYes => QuoteDirection::BuyYes,
            BuyDirection::BuyNo => QuoteDirection::BuyNo,
        };
        let quote = quote_buy_with_coin_router(
            &pool_pre,
            MicroCoin::from_micro_units(pay_micro),
            quote_direction,
        )
        .expect("forced robustness quote available");

        let parent = q_pre.state_root_t;
        let apply_result = submit_and_apply(
            &mut h,
            build_router(parent, actor, task, direction, pay_micro, idx as u64 + 1),
        )
        .await;

        if quote.liquidity_warning == LiquidityWarning::NoOutput {
            let err = apply_result.expect_err("no-output forced robustness router rejects");
            assert!(
                err.contains("RouterSwapInsufficientPoolOutput"),
                "tiny forced router should fail closed on insufficient pool output: {err}"
            );
            let q_after_reject = h.seq.q_snapshot().unwrap();
            assert_eq!(
                q_after_reject.state_root_t, q_pre.state_root_t,
                "rejected tiny forced router must not mutate state"
            );
            assert_total_ctf_conserved(
                &q_pre.economic_state_t,
                &q_after_reject.economic_state_t,
                &[],
            )
            .expect("Coin conserved across rejected forced robustness router");
            continue;
        }
        apply_result.expect("forced robustness router accepted");

        let q_post = h.seq.q_snapshot().unwrap();
        assert_total_ctf_conserved(&q_pre.economic_state_t, &q_post.economic_state_t, &[])
            .expect("Coin conserved across forced robustness router");
        assert_complete_set_balanced(&q_post.economic_state_t)
            .expect("complete-set balanced after forced robustness router");

        let balance_post = q_post
            .economic_state_t
            .balances_t
            .0
            .get(&AgentId(actor.into()))
            .copied()
            .expect("actor balance post");
        assert_eq!(
            balance_pre.micro_units() - balance_post.micro_units(),
            pay_micro,
            "router debits exactly pay_coin for {actor} {direction:?}"
        );
        let side_units_post = agent_side_units(&q_post.economic_state_t, actor, task, direction);
        assert_eq!(
            side_units_post - side_units_pre,
            quote.get_shares.units,
            "router credits pay_coin + CPMM out_shares for {actor} {direction:?}"
        );

        let pool_post = q_post
            .economic_state_t
            .cpmm_pools_t
            .0
            .get(&event_id)
            .expect("pool after forced router");
        match direction {
            BuyDirection::BuyYes => {
                assert_eq!(
                    pool_post.pool_no.units,
                    pool_pre.pool_no.units + pay_micro as u128
                );
                assert_eq!(
                    pool_post.pool_yes.units,
                    pool_pre.pool_yes.units - quote.out_shares.units
                );
            }
            BuyDirection::BuyNo => {
                assert_eq!(
                    pool_post.pool_yes.units,
                    pool_pre.pool_yes.units + pay_micro as u128
                );
                assert_eq!(
                    pool_post.pool_no.units,
                    pool_pre.pool_no.units - quote.out_shares.units
                );
            }
        }
        let next_k = pool_k(&q_post.economic_state_t, task);
        assert!(
            next_k >= previous_k,
            "integer CPMM k must be non-decreasing across forced router sequence"
        );
        previous_k = next_k;
    }

    let q_post = h.seq.q_snapshot().unwrap();
    assert_eq!(total_coin_micro(&q_post.economic_state_t), total_coin_pre);
    let coll = q_post
        .economic_state_t
        .conditional_collateral_t
        .0
        .get(&event_id)
        .copied()
        .expect("collateral present");
    assert_eq!(
        sum_yes_for_event(&q_post.economic_state_t, task),
        sum_no_for_event(&q_post.economic_state_t, task)
    );
    assert_eq!(
        sum_yes_for_event(&q_post.economic_state_t, task),
        coll.micro_units() as u128
    );

    let quotes = audit_view_prices(
        &q_post.economic_state_t,
        &[
            MicroCoin::from_micro_units(1),
            MicroCoin::from_micro_units(1_000_000),
        ],
    );
    assert_eq!(quotes.price_quotes.len(), 4);
    assert_eq!(
        h.seq.q_snapshot().unwrap().state_root_t,
        q_post.state_root_t,
        "price/quote audit views must not mutate truth state"
    );
}

#[tokio::test]
async fn extreme_forced_router_sequence_near_depletion_preserves_invariants() {
    let task = "polymarket-extreme-robustness";
    let event_id = EventId(TaskId(task.into()));
    let q0 = genesis_with_balances_and_open_task(
        &[
            ("maker", 1_000),
            ("forced_bull", 1_000),
            ("forced_bear", 1_000),
        ],
        task,
    );
    let mut h = fresh_harness(q0);

    let p = h.seq.q_snapshot().unwrap().state_root_t;
    submit_and_apply(&mut h, build_mint(p, "maker", task, 200_000_000, 1))
        .await
        .expect("maker mint accepted");
    let p = h.seq.q_snapshot().unwrap().state_root_t;
    submit_and_apply(&mut h, build_pool(p, "maker", task, 100_000_000, 1))
        .await
        .expect("maker pool accepted");

    let q_seed = h.seq.q_snapshot().unwrap();
    let total_coin_pre = total_coin_micro(&q_seed.economic_state_t);
    let mut previous_k = pool_k(&q_seed.economic_state_t, task);
    let mut accepted_count = 0usize;
    let mut no_output_rejection_count = 0usize;

    let stress_sequence: [(&str, BuyDirection, i64); 12] = [
        ("forced_bull", BuyDirection::BuyYes, 75_000_000),
        ("forced_bull", BuyDirection::BuyYes, 125_000_000),
        ("forced_bull", BuyDirection::BuyYes, 1),
        ("forced_bear", BuyDirection::BuyNo, 75_000_000),
        ("forced_bear", BuyDirection::BuyNo, 125_000_000),
        ("forced_bear", BuyDirection::BuyNo, 1),
        ("forced_bull", BuyDirection::BuyYes, 50_000_000),
        ("forced_bear", BuyDirection::BuyNo, 50_000_000),
        ("forced_bull", BuyDirection::BuyYes, 25_000_000),
        ("forced_bear", BuyDirection::BuyNo, 25_000_000),
        ("forced_bull", BuyDirection::BuyYes, 1),
        ("forced_bear", BuyDirection::BuyNo, 1),
    ];

    for (idx, (actor, direction, pay_micro)) in stress_sequence.into_iter().enumerate() {
        let q_pre = h.seq.q_snapshot().unwrap();
        let balance_pre = q_pre
            .economic_state_t
            .balances_t
            .0
            .get(&AgentId(actor.into()))
            .copied()
            .expect("actor balance");
        let side_units_pre = agent_side_units(&q_pre.economic_state_t, actor, task, direction);
        let pool_pre = q_pre
            .economic_state_t
            .cpmm_pools_t
            .0
            .get(&event_id)
            .cloned()
            .expect("pool before stress router");
        let quote_direction = match direction {
            BuyDirection::BuyYes => QuoteDirection::BuyYes,
            BuyDirection::BuyNo => QuoteDirection::BuyNo,
        };
        let quote = quote_buy_with_coin_router(
            &pool_pre,
            MicroCoin::from_micro_units(pay_micro),
            quote_direction,
        )
        .expect("stress quote available");

        let apply_result = submit_and_apply(
            &mut h,
            build_router(
                q_pre.state_root_t,
                actor,
                task,
                direction,
                pay_micro,
                idx as u64 + 1,
            ),
        )
        .await;

        if quote.liquidity_warning == LiquidityWarning::NoOutput {
            let err = apply_result.expect_err("no-output stress router rejects");
            assert!(
                err.contains("RouterSwapInsufficientPoolOutput"),
                "near-depletion/no-output router should fail closed: {err}"
            );
            let q_after_reject = h.seq.q_snapshot().unwrap();
            assert_eq!(
                q_after_reject.state_root_t, q_pre.state_root_t,
                "rejected extreme router must not mutate state"
            );
            assert_eq!(
                total_coin_micro(&q_after_reject.economic_state_t),
                total_coin_micro(&q_pre.economic_state_t),
                "rejected extreme router preserves total Coin"
            );
            no_output_rejection_count += 1;
            continue;
        }
        apply_result.expect("stress router accepted");
        accepted_count += 1;

        let q_post = h.seq.q_snapshot().unwrap();
        assert_total_ctf_conserved(&q_pre.economic_state_t, &q_post.economic_state_t, &[])
            .expect("Coin conserved across extreme forced router");
        assert_complete_set_balanced(&q_post.economic_state_t)
            .expect("complete-set balanced after extreme forced router");

        let balance_post = q_post
            .economic_state_t
            .balances_t
            .0
            .get(&AgentId(actor.into()))
            .copied()
            .expect("actor balance post");
        assert_eq!(
            balance_pre.micro_units() - balance_post.micro_units(),
            pay_micro,
            "extreme router debits exactly pay_coin"
        );
        let side_units_post = agent_side_units(&q_post.economic_state_t, actor, task, direction);
        assert_eq!(
            side_units_post - side_units_pre,
            quote.get_shares.units,
            "extreme router credits pay_coin + CPMM out_shares"
        );

        let pool_post = q_post
            .economic_state_t
            .cpmm_pools_t
            .0
            .get(&event_id)
            .expect("pool after stress router");
        match direction {
            BuyDirection::BuyYes => {
                assert_eq!(
                    pool_post.pool_no.units,
                    pool_pre.pool_no.units + pay_micro as u128
                );
                assert_eq!(
                    pool_post.pool_yes.units,
                    pool_pre.pool_yes.units - quote.out_shares.units
                );
            }
            BuyDirection::BuyNo => {
                assert_eq!(
                    pool_post.pool_yes.units,
                    pool_pre.pool_yes.units + pay_micro as u128
                );
                assert_eq!(
                    pool_post.pool_no.units,
                    pool_pre.pool_no.units - quote.out_shares.units
                );
            }
        }
        let next_k = pool_k(&q_post.economic_state_t, task);
        assert!(
            next_k >= previous_k,
            "integer CPMM k must be non-decreasing under extreme sequence"
        );
        previous_k = next_k;
    }

    assert!(
        accepted_count >= 6,
        "extreme stress should exercise multiple accepted YES/NO buys"
    );
    assert!(
        no_output_rejection_count >= 2,
        "extreme stress should exercise fail-closed no-output buys"
    );
    let q_post = h.seq.q_snapshot().unwrap();
    assert_eq!(total_coin_micro(&q_post.economic_state_t), total_coin_pre);
    let coll = q_post
        .economic_state_t
        .conditional_collateral_t
        .0
        .get(&event_id)
        .copied()
        .expect("collateral present");
    assert_eq!(
        sum_yes_for_event(&q_post.economic_state_t, task),
        sum_no_for_event(&q_post.economic_state_t, task)
    );
    assert_eq!(
        sum_yes_for_event(&q_post.economic_state_t, task),
        coll.micro_units() as u128
    );
}

#[tokio::test]
async fn router_rejection_paths_preserve_state_and_report_specific_errors() {
    let task = "polymarket-router-rejects";
    let q0 =
        genesis_with_balances_and_open_task(&[("maker", 200), ("trader", 10), ("poor", 1)], task);
    let mut h = fresh_harness(q0);

    let p = h.seq.q_snapshot().unwrap().state_root_t;
    submit_and_apply(&mut h, build_mint(p, "maker", task, 20_000_000, 1))
        .await
        .expect("maker mint accepted");
    let p = h.seq.q_snapshot().unwrap().state_root_t;
    submit_and_apply(&mut h, build_pool(p, "maker", task, 10_000_000, 1))
        .await
        .expect("maker pool accepted");

    let q_pre_slippage = h.seq.q_snapshot().unwrap();
    let pool = q_pre_slippage
        .economic_state_t
        .cpmm_pools_t
        .0
        .get(&EventId(TaskId(task.into())))
        .cloned()
        .expect("pool exists");
    let quote = quote_buy_with_coin_router(
        &pool,
        MicroCoin::from_micro_units(1_000_000),
        QuoteDirection::BuyYes,
    )
    .expect("quote exists");
    expect_router_rejection_preserves_state(
        &mut h,
        TypedTx::BuyWithCoinRouter(BuyWithCoinRouterTx {
            tx_id: TxId("router-slippage-reject".into()),
            parent_state_root: q_pre_slippage.state_root_t,
            event_id: EventId(TaskId(task.into())),
            buyer: AgentId("trader".into()),
            direction: BuyDirection::BuyYes,
            pay_coin: MicroCoin::from_micro_units(1_000_000),
            min_out_shares: ShareAmount::from_units(quote.out_shares.units + 1),
            signature: AgentSignature::from_bytes([0u8; 64]),
        }),
        "RouterSlippageExceeded",
    )
    .await;

    let q_pre_balance = h.seq.q_snapshot().unwrap();
    expect_router_rejection_preserves_state(
        &mut h,
        build_router(
            q_pre_balance.state_root_t,
            "poor",
            task,
            BuyDirection::BuyNo,
            2_000_000,
            7,
        ),
        "RouterInsufficientCoinBalance",
    )
    .await;

    let mut q_closed = genesis_with_balances_and_open_task(&[("trader", 10)], task);
    q_closed
        .economic_state_t
        .task_markets_t
        .0
        .get_mut(&TaskId(task.into()))
        .expect("task market exists")
        .state = TaskMarketState::Finalized;
    let mut closed_h = fresh_harness(q_closed);
    let closed_parent = closed_h.seq.q_snapshot().unwrap().state_root_t;
    expect_router_rejection_preserves_state(
        &mut closed_h,
        build_router(
            closed_parent,
            "trader",
            task,
            BuyDirection::BuyYes,
            100_000,
            8,
        ),
        "EventNotOpen",
    )
    .await;
}

#[tokio::test]
async fn forced_router_holdings_redeem_on_yes_and_no_resolution() {
    async fn setup_forced_position(actor: &str, direction: BuyDirection, task: &str) -> Harness {
        let q0 = genesis_with_balances_and_open_task(&[("maker", 100), (actor, 20)], task);
        let mut h = fresh_harness(q0);
        let p = h.seq.q_snapshot().unwrap().state_root_t;
        submit_and_apply(&mut h, build_mint(p, "maker", task, 10_000_000, 1))
            .await
            .expect("maker mint accepted");
        let p = h.seq.q_snapshot().unwrap().state_root_t;
        submit_and_apply(&mut h, build_pool(p, "maker", task, 5_000_000, 1))
            .await
            .expect("maker pool accepted");
        let p = h.seq.q_snapshot().unwrap().state_root_t;
        submit_and_apply(
            &mut h,
            build_router(p, actor, task, direction, 1_000_000, 1),
        )
        .await
        .expect("forced router position accepted");
        h
    }

    let mut yes_h = setup_forced_position("forced_bull", BuyDirection::BuyYes, "robust-yes").await;
    let yes_pre_resolve = yes_h.seq.q_snapshot().unwrap();
    let yes_units = agent_side_units(
        &yes_pre_resolve.economic_state_t,
        "forced_bull",
        "robust-yes",
        BuyDirection::BuyYes,
    );
    assert!(yes_units > 1_000_000);
    yes_h
        .seq
        .emit_system_tx(SystemEmitCommand::EventResolve {
            task_id: TaskId("robust-yes".into()),
            outcome: OutcomeSide::Yes,
        })
        .await
        .expect("YES EventResolve emitted");
    yes_h
        .seq
        .try_apply_one(&mut yes_h.rx)
        .expect("YES EventResolve envelope")
        .expect("YES EventResolve accepted");
    redeem_and_assert_payout(
        &mut yes_h,
        "forced_bull",
        "robust-yes",
        OutcomeSide::Yes,
        BuyDirection::BuyYes,
        yes_units,
        1,
    )
    .await;

    let mut no_h = setup_forced_position("forced_bear", BuyDirection::BuyNo, "robust-no").await;
    let no_pre_resolve = no_h.seq.q_snapshot().unwrap();
    let no_units = agent_side_units(
        &no_pre_resolve.economic_state_t,
        "forced_bear",
        "robust-no",
        BuyDirection::BuyNo,
    );
    assert!(no_units > 1_000_000);
    no_h.seq
        .emit_system_tx(SystemEmitCommand::EventResolve {
            task_id: TaskId("robust-no".into()),
            outcome: OutcomeSide::No,
        })
        .await
        .expect("NO EventResolve emitted");
    no_h.seq
        .try_apply_one(&mut no_h.rx)
        .expect("NO EventResolve envelope")
        .expect("NO EventResolve accepted");
    redeem_and_assert_payout(
        &mut no_h,
        "forced_bear",
        "robust-no",
        OutcomeSide::No,
        BuyDirection::BuyNo,
        no_units,
        1,
    )
    .await;
}

#[tokio::test]
async fn settlement_rejects_wrong_side_and_double_redeem_without_state_mutation() {
    let task = "robust-redeem-negative";
    let q0 = genesis_with_balances_and_open_task(&[("maker", 100), ("forced_bull", 20)], task);
    let mut h = fresh_harness(q0);
    let p = h.seq.q_snapshot().unwrap().state_root_t;
    submit_and_apply(&mut h, build_mint(p, "maker", task, 10_000_000, 1))
        .await
        .expect("maker mint accepted");
    let p = h.seq.q_snapshot().unwrap().state_root_t;
    submit_and_apply(&mut h, build_pool(p, "maker", task, 5_000_000, 1))
        .await
        .expect("maker pool accepted");
    let p = h.seq.q_snapshot().unwrap().state_root_t;
    submit_and_apply(
        &mut h,
        build_router(p, "forced_bull", task, BuyDirection::BuyYes, 1_000_000, 1),
    )
    .await
    .expect("forced YES router accepted");

    let pre_resolve = h.seq.q_snapshot().unwrap();
    let yes_units = agent_side_units(
        &pre_resolve.economic_state_t,
        "forced_bull",
        task,
        BuyDirection::BuyYes,
    );
    h.seq
        .emit_system_tx(SystemEmitCommand::EventResolve {
            task_id: TaskId(task.into()),
            outcome: OutcomeSide::Yes,
        })
        .await
        .expect("YES EventResolve emitted");
    h.seq
        .try_apply_one(&mut h.rx)
        .expect("YES EventResolve envelope")
        .expect("YES EventResolve accepted");

    let parent_wrong_side = h.seq.q_snapshot().unwrap().state_root_t;
    expect_redeem_rejection_preserves_state(
        &mut h,
        build_redeem(
            parent_wrong_side,
            "forced_bull",
            task,
            OutcomeSide::No,
            1,
            1,
        ),
        "InvalidResolutionRef",
    )
    .await;

    redeem_and_assert_payout(
        &mut h,
        "forced_bull",
        task,
        OutcomeSide::Yes,
        BuyDirection::BuyYes,
        yes_units,
        1,
    )
    .await;

    let parent_double = h.seq.q_snapshot().unwrap().state_root_t;
    expect_redeem_rejection_preserves_state(
        &mut h,
        build_redeem(parent_double, "forced_bull", task, OutcomeSide::Yes, 1, 2),
        "RedeemMoreThanOwned",
    )
    .await;
}

#[tokio::test]
async fn settlement_partial_redeems_preserve_conservation_until_position_empty() {
    let task = "robust-partial-redeem";
    let q0 = genesis_with_balances_and_open_task(&[("maker", 100), ("forced_bull", 20)], task);
    let mut h = fresh_harness(q0);
    let p = h.seq.q_snapshot().unwrap().state_root_t;
    submit_and_apply(&mut h, build_mint(p, "maker", task, 10_000_000, 1))
        .await
        .expect("maker mint accepted");
    let p = h.seq.q_snapshot().unwrap().state_root_t;
    submit_and_apply(&mut h, build_pool(p, "maker", task, 5_000_000, 1))
        .await
        .expect("maker pool accepted");
    let p = h.seq.q_snapshot().unwrap().state_root_t;
    submit_and_apply(
        &mut h,
        build_router(p, "forced_bull", task, BuyDirection::BuyYes, 1_000_000, 1),
    )
    .await
    .expect("forced YES router accepted");

    let pre_resolve = h.seq.q_snapshot().unwrap();
    let yes_units = agent_side_units(
        &pre_resolve.economic_state_t,
        "forced_bull",
        task,
        BuyDirection::BuyYes,
    );
    assert!(yes_units > 2);
    h.seq
        .emit_system_tx(SystemEmitCommand::EventResolve {
            task_id: TaskId(task.into()),
            outcome: OutcomeSide::Yes,
        })
        .await
        .expect("YES EventResolve emitted");
    h.seq
        .try_apply_one(&mut h.rx)
        .expect("YES EventResolve envelope")
        .expect("YES EventResolve accepted");

    let first_slice = yes_units / 2;
    let second_slice = yes_units - first_slice;
    redeem_and_assert_payout(
        &mut h,
        "forced_bull",
        task,
        OutcomeSide::Yes,
        BuyDirection::BuyYes,
        first_slice,
        1,
    )
    .await;
    let after_first = h.seq.q_snapshot().unwrap();
    assert_eq!(
        agent_side_units(
            &after_first.economic_state_t,
            "forced_bull",
            task,
            BuyDirection::BuyYes
        ),
        second_slice,
        "partial redeem leaves exact residual winning shares"
    );
    redeem_and_assert_payout(
        &mut h,
        "forced_bull",
        task,
        OutcomeSide::Yes,
        BuyDirection::BuyYes,
        second_slice,
        2,
    )
    .await;
    let after_second = h.seq.q_snapshot().unwrap();
    assert_eq!(
        agent_side_units(
            &after_second.economic_state_t,
            "forced_bull",
            task,
            BuyDirection::BuyYes
        ),
        0,
        "second partial redeem empties winning position exactly"
    );
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
