//! TB-N3 A2 invest-routing constitution gate tests.
//!
//! SG-N3.1 — A1/A2 fixture: parsed invest payload + existing pool →
//! `tb_n3_invest_to_router_tx` returns `Ok(BuyWithCoinRouterTx)` whose
//! `event_id` is `node_survive:`-namespaced (architect ruling 2026-05-11
//! amendment 1).
//!
//! SG-N3.2 — A2 missing-pool path: parsed invest payload + no pool in
//! snapshot → `Err(InvestRouteError::UnknownEvent)` (NOT silent drop;
//! caller anchors `MarketDecisionTrace::no_trade(NoPool)` per architect §8.6).
//!
//! Plus negative-path coverage: zero amount, negative amount, malformed
//! node, balance shortfall, pool-not-active. Each variant maps 1:1 to a
//! `NoTradeReason` per architect §8.2.

use tempfile::TempDir;
use turingosv4::economy::money::MicroCoin;
use turingosv4::runtime::adapter::{
    make_real_complete_set_mint_signed_by, make_real_cpmm_pool_signed_by,
    make_real_market_seed_signed_by, tb_n3_invest_to_router_tx, InvestRouteError,
};
use turingosv4::runtime::agent_keypairs::AgentKeypairRegistry;
use turingosv4::runtime::market_decision_trace::NoTradeReason;
use turingosv4::state::q_state::{
    AgentId, CpmmPool, EconomicState, Hash, LpShareAmount, PoolStatus, QState, TaskId,
    TaskMarketEntry, TaskMarketState, TxId,
};
use turingosv4::state::typed_tx::{BuyDirection, EventId, ShareAmount, TypedTx};

/// Build a snapshot QState with a single Active pool keyed at
/// `node_survive:<work_tx_id>` and a pre-loaded buyer balance. Synthetic;
/// bypasses sequencer admission (we're testing the adapter helper, not
/// admission). Sequencer-side admission tests live alongside Stage C
/// `constitution_router_buy_with_coin.rs`.
fn snapshot_with_pool_and_balance(
    work_tx_id: &str,
    pool_seed_units: u128,
    buyer: &str,
    buyer_balance_micro: i64,
) -> QState {
    let mut q = QState::default();
    q.economic_state_t.balances_t.0.insert(
        AgentId(buyer.into()),
        MicroCoin::from_micro_units(buyer_balance_micro),
    );
    let event_id = turingosv4::state::typed_tx::node_survive_event_id(&TxId(work_tx_id.into()));
    q.economic_state_t.cpmm_pools_t.0.insert(
        event_id.clone(),
        CpmmPool {
            event_id,
            pool_yes: ShareAmount::from_units(pool_seed_units),
            pool_no: ShareAmount::from_units(pool_seed_units),
            lp_total_shares: LpShareAmount::from_units(pool_seed_units),
            status: PoolStatus::Active,
        },
    );
    q
}

fn snapshot_with_balance_no_pool(buyer: &str, buyer_balance_micro: i64) -> QState {
    let mut q = QState::default();
    q.economic_state_t.balances_t.0.insert(
        AgentId(buyer.into()),
        MicroCoin::from_micro_units(buyer_balance_micro),
    );
    q
}

fn snapshot_with_closed_pool(work_tx_id: &str, buyer: &str, buyer_balance_micro: i64) -> QState {
    let mut q = QState::default();
    q.economic_state_t.balances_t.0.insert(
        AgentId(buyer.into()),
        MicroCoin::from_micro_units(buyer_balance_micro),
    );
    let event_id = turingosv4::state::typed_tx::node_survive_event_id(&TxId(work_tx_id.into()));
    q.economic_state_t.cpmm_pools_t.0.insert(
        event_id.clone(),
        CpmmPool {
            event_id,
            pool_yes: ShareAmount::from_units(1_000_000),
            pool_no: ShareAmount::from_units(1_000_000),
            lp_total_shares: LpShareAmount::from_units(1_000_000),
            status: PoolStatus::Closed,
        },
    );
    q
}

/// SG-N3.1 — fixture pool present + sufficient balance → router tx
/// constructed; event_id is `node_survive:<work_tx_id>` namespaced.
#[test]
fn sg_n3_1_fixture_pool_present_router_accepts() {
    let repo = TempDir::new().expect("tempdir");
    let mut reg = AgentKeypairRegistry::open(repo.path()).expect("open");
    let q = snapshot_with_pool_and_balance(
        "worktx-Agent_2-mathd_algebra_107-7",
        4_000_000,
        "Agent_3",
        1_000_000,
    );

    let tx = tb_n3_invest_to_router_tx(
        &mut reg,
        Hash::ZERO,
        Some(&q),
        "Agent_3",
        "worktx-Agent_2-mathd_algebra_107-7",
        BuyDirection::BuyYes,
        500_000,
        0,
        "smoke-1",
    )
    .expect("router tx must build with fixture pool");

    let router = match tx {
        TypedTx::BuyWithCoinRouter(r) => r,
        other => panic!("expected BuyWithCoinRouter, got {other:?}"),
    };
    assert_eq!(router.buyer.0, "Agent_3");
    assert_eq!(router.direction, BuyDirection::BuyYes);
    assert_eq!(router.pay_coin.micro_units(), 500_000);
    assert!(
        router.event_id.0 .0.starts_with("node_survive:"),
        "SG-N3.4 event_id namespace: must start with `node_survive:`, got {:?}",
        router.event_id.0 .0
    );
    assert_eq!(
        router.event_id.0 .0, "node_survive:worktx-Agent_2-mathd_algebra_107-7",
        "event_id encodes the work_tx_id verbatim after the prefix"
    );
    assert_ne!(
        *router.signature.as_bytes(),
        [0u8; 64],
        "signature non-zero"
    );
}

/// SG-N3.2 — missing-pool path: returns `Err(UnknownEvent)`, NOT silent drop.
/// Verifies the error maps to `NoTradeReason::NoPool` for trace anchoring.
#[test]
fn sg_n3_2_missing_pool_routes_to_l4e_no_pool() {
    let repo = TempDir::new().expect("tempdir");
    let mut reg = AgentKeypairRegistry::open(repo.path()).expect("open");
    let q = snapshot_with_balance_no_pool("Agent_3", 1_000_000);

    let err = tb_n3_invest_to_router_tx(
        &mut reg,
        Hash::ZERO,
        Some(&q),
        "Agent_3",
        "worktx-Agent_2-mathd_algebra_107-7",
        BuyDirection::BuyYes,
        500_000,
        0,
        "smoke-2",
    )
    .expect_err("must reject when no pool present");

    assert!(matches!(err, InvestRouteError::UnknownEvent));
    assert_eq!(err.to_no_trade_reason(), NoTradeReason::NoPool);
    let summary = err.public_summary();
    assert!(!summary.is_empty(), "summary anchored for trace");
    assert!(summary.len() <= 120, "summary fits trace bound");
}

/// SG-N3.2-aux — pool exists but Closed status: same NoPool route.
#[test]
fn closed_pool_routes_to_no_pool() {
    let repo = TempDir::new().expect("tempdir");
    let mut reg = AgentKeypairRegistry::open(repo.path()).expect("open");
    let q = snapshot_with_closed_pool("worktx-Agent_2-mathd_algebra_107-7", "Agent_3", 1_000_000);

    let err = tb_n3_invest_to_router_tx(
        &mut reg,
        Hash::ZERO,
        Some(&q),
        "Agent_3",
        "worktx-Agent_2-mathd_algebra_107-7",
        BuyDirection::BuyYes,
        500_000,
        0,
        "smoke-2b",
    )
    .expect_err("closed pool must reject");

    assert!(matches!(err, InvestRouteError::PoolNotActive));
    assert_eq!(err.to_no_trade_reason(), NoTradeReason::NoPool);
}

/// SG-N3.2-zero — zero-amount invest: returns ZeroAmount before any
/// snapshot lookup (parser-induced no-trade).
#[test]
fn zero_amount_invest_returns_zero_amount() {
    let repo = TempDir::new().expect("tempdir");
    let mut reg = AgentKeypairRegistry::open(repo.path()).expect("open");
    let err = tb_n3_invest_to_router_tx(
        &mut reg,
        Hash::ZERO,
        None,
        "Agent_3",
        "worktx-Agent_2-mathd_algebra_107-7",
        BuyDirection::BuyYes,
        0,
        0,
        "smoke-3",
    )
    .expect_err("amount=0 rejected");
    assert!(matches!(err, InvestRouteError::ZeroAmount));
    assert_eq!(err.to_no_trade_reason(), NoTradeReason::ZeroAmount);
}

/// SG-N3.2-neg — negative amount returns NegativeAmount → NoParsedInvest.
#[test]
fn negative_amount_invest_returns_negative_amount() {
    let repo = TempDir::new().expect("tempdir");
    let mut reg = AgentKeypairRegistry::open(repo.path()).expect("open");
    let err = tb_n3_invest_to_router_tx(
        &mut reg,
        Hash::ZERO,
        None,
        "Agent_3",
        "worktx-Agent_2-mathd_algebra_107-7",
        BuyDirection::BuyNo,
        -500_000,
        0,
        "smoke-4",
    )
    .expect_err("negative amount rejected");
    assert!(matches!(err, InvestRouteError::NegativeAmount));
    assert_eq!(err.to_no_trade_reason(), NoTradeReason::NoParsedInvest);
}

/// SG-N3.2-empty — empty node_str returns MalformedNode.
#[test]
fn empty_node_str_returns_malformed_node() {
    let repo = TempDir::new().expect("tempdir");
    let mut reg = AgentKeypairRegistry::open(repo.path()).expect("open");
    let err = tb_n3_invest_to_router_tx(
        &mut reg,
        Hash::ZERO,
        None,
        "Agent_3",
        "",
        BuyDirection::BuyYes,
        500_000,
        0,
        "smoke-5",
    )
    .expect_err("empty node_str rejected");
    assert!(matches!(
        err,
        InvestRouteError::MalformedNode { reason: "empty" }
    ));
    assert_eq!(err.to_no_trade_reason(), NoTradeReason::MalformedNode);
}

/// SG-N3.2-bal — balance shortfall returns AmountExceedsBalance.
#[test]
fn balance_shortfall_returns_amount_exceeds_balance() {
    let repo = TempDir::new().expect("tempdir");
    let mut reg = AgentKeypairRegistry::open(repo.path()).expect("open");
    let q = snapshot_with_pool_and_balance(
        "worktx-Agent_2-mathd_algebra_107-7",
        4_000_000,
        "Agent_3",
        100_000,
    );
    let err = tb_n3_invest_to_router_tx(
        &mut reg,
        Hash::ZERO,
        Some(&q),
        "Agent_3",
        "worktx-Agent_2-mathd_algebra_107-7",
        BuyDirection::BuyYes,
        500_000,
        0,
        "smoke-6",
    )
    .expect_err("amount > balance rejected");
    match err {
        InvestRouteError::AmountExceedsBalance {
            amount_micro,
            balance_micro,
        } => {
            assert_eq!(amount_micro, 500_000);
            assert_eq!(balance_micro, 100_000);
        }
        other => panic!("expected AmountExceedsBalance, got {other:?}"),
    }
}

/// TB-N3 A2 — same payload twice yields the same canonical signature
/// (deterministic Ed25519 over canonical digest; same as Stage C P-M6
/// Phase F.5 `make_real_buy_with_coin_router` shape contract).
#[test]
fn same_invest_payload_yields_deterministic_signature() {
    let repo = TempDir::new().expect("tempdir");
    let mut reg = AgentKeypairRegistry::open(repo.path()).expect("open");
    let q = snapshot_with_pool_and_balance("worktx-Agent_2-evt-1", 4_000_000, "Agent_3", 1_000_000);
    let make = |reg: &mut AgentKeypairRegistry| {
        tb_n3_invest_to_router_tx(
            reg,
            Hash::ZERO,
            Some(&q),
            "Agent_3",
            "worktx-Agent_2-evt-1",
            BuyDirection::BuyYes,
            300_000,
            0,
            "det",
        )
        .expect("ok")
    };
    let tx1 = make(&mut reg);
    let tx2 = make(&mut reg);
    let s1 = match &tx1 {
        TypedTx::BuyWithCoinRouter(r) => *r.signature.as_bytes(),
        _ => panic!(),
    };
    let s2 = match &tx2 {
        TypedTx::BuyWithCoinRouter(r) => *r.signature.as_bytes(),
        _ => panic!(),
    };
    assert_eq!(s1, s2, "deterministic signing same payload");
}

// ── Suppress unused-import warnings for shared imports ─────────────────────

#[test]
fn _suppress_unused_imports() {
    let _ = MicroCoin::zero();
    let _ = make_real_market_seed_signed_by;
    let _ = make_real_complete_set_mint_signed_by;
    let _ = make_real_cpmm_pool_signed_by;
    let _: TaskMarketEntry = TaskMarketEntry::default();
    let _ = TaskMarketState::Open;
    let _ = TaskId("x".into());
    let _ = EconomicState::default();
    let _ = EventId(TaskId("x".into()));
}
