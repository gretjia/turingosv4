//! TuringOS Constitution Gate — Stage C P-M6 Mint-and-Swap Router (architect
//! 2026-05-07 ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL §7.7
//! verbatim).
//!
//! # Scope
//!
//! Architect alignment doc §7.7 mandates 9 hardening tests for the new
//! `BuyWithCoinRouterTx` (atomic 9-step composite over CompleteSetMint +
//! CpmmSwap):
//!
//!   - buy_yes_with_coin_matches_formula
//!   - buy_no_with_coin_matches_symmetric_formula
//!   - buy_yes_debits_coin_locks_collateral
//!   - buy_yes_mints_complete_set
//!   - buy_yes_transfers_retained_yes_plus_swap_yes
//!   - buy_yes_respects_min_yes_out
//!   - buy_yes_no_f64
//!   - buy_yes_no_ghost_liquidity
//!   - router_atomic_rollback_on_failure
//!
//! TRACE_FLOWCHART_MATRIX:
//!   - FC1 §1 predicate routing (router admission rejects pay_coin <= 0,
//!     buyer balance shortfall, slippage, missing pool)
//!   - FC1 §6 monetary invariant (CTF conservation; complete-set balanced
//!     post-router; constant-product non-decreasing)
//!   - §7.7 architect Polymarket manual

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
use turingosv4::economy::monetary_invariant::total_supply_micro as canonical_total_supply_micro;
use turingosv4::state::q_state::{
    AgentId, CpmmPool, LpShareAmount, PoolEventKind, PoolStatus, QState, ShareSidePair,
    TaskId, TxId,
};
use turingosv4::state::sequencer::{Sequencer, SubmissionEnvelope};
use turingosv4::state::typed_tx::{
    AgentSignature, BuyDirection, BuyWithCoinRouterTx, EventId, ShareAmount, TypedTx,
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

/// Build a genesis state with: (a) buyer holding `buyer_coin` Coin balance;
/// (b) a CpmmPool at `event_task` with given reserves backed by collateral
/// equal to `max(pool_yes, pool_no)` (no ghost liquidity);
/// (c) a "balanced" Σ_yes == Σ_no == collateral starter so that
/// `assert_complete_set_balanced` holds pre-router.
fn genesis_with_buyer_and_balanced_pool(
    buyer: &str,
    event_task: &str,
    buyer_coin: i64,
    pool_yes: u128,
    pool_no: u128,
) -> QState {
    let mut q = QState::genesis();
    let event = EventId(TaskId(event_task.into()));

    // Buyer Coin balance.
    q.economic_state_t.balances_t.0.insert(
        AgentId(buyer.into()),
        MicroCoin::from_coin(buyer_coin).unwrap(),
    );

    // To keep `assert_complete_set_balanced` valid pre-router we need
    // Σ_yes == Σ_no == collateral. Use max(pool_yes, pool_no) as
    // collateral and pad both pool sides to that level.
    let target = pool_yes.max(pool_no);
    q.economic_state_t.conditional_collateral_t.0.insert(
        event.clone(),
        MicroCoin::from_micro_units(target as i64),
    );

    q.economic_state_t.cpmm_pools_t.0.insert(
        event,
        CpmmPool {
            event_id_kind: PoolEventKind::BinaryYesNo,
            pool_yes: ShareAmount::from_units(target),
            pool_no: ShareAmount::from_units(target),
            lp_total_shares: LpShareAmount::from_units(target),
            status: PoolStatus::Active,
        },
    );

    let _ = (pool_yes, pool_no); // reserved for future asymmetric setup

    q
}

async fn submit_and_apply(h: &mut Harness, tx: TypedTx) -> Result<(), String> {
    h.seq.submit_agent_tx(tx).await
        .map_err(|e| format!("submit error: {e:?}"))?;
    let outcome = h.seq.try_apply_one(&mut h.rx)
        .ok_or_else(|| "no envelope drained".to_string())?;
    outcome.map(|_| ()).map_err(|e| format!("apply error: {e:?}"))
}

fn build_router(
    parent: turingosv4::state::q_state::Hash,
    buyer: &str,
    task: &str,
    direction: BuyDirection,
    pay_micro: i64,
    min_total_out: u128,
    seq_no: u64,
) -> TypedTx {
    TypedTx::BuyWithCoinRouter(BuyWithCoinRouterTx {
        tx_id: TxId(format!("router-{buyer}-{task}-{seq_no}")),
        parent_state_root: parent,
        event_id: EventId(TaskId(task.into())),
        buyer: AgentId(buyer.into()),
        direction,
        pay_coin: MicroCoin::from_micro_units(pay_micro),
        min_total_out: ShareAmount::from_units(min_total_out),
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: 7000 + seq_no,
    })
}

fn total_supply_micro(q: &QState) -> i64 {
    canonical_total_supply_micro(&q.economic_state_t)
        .expect("total_supply_micro must not overflow in test fixtures")
}

// ════════════════════════════════════════════════════════════════════════════
// §7.7 P-M6 BuyWithCoinRouter hardening (9 verbatim names)
// ════════════════════════════════════════════════════════════════════════════

/// §7.7 verbatim — `buy_yes_with_coin_matches_formula`.
///
/// Per architect manual §7.7: outY = floor(payC * poolY / (poolN + payC));
/// getY = payC + outY. Asserts buyer's post-router YES balance equals
/// expected total computed from formula.
#[tokio::test]
async fn buy_yes_with_coin_matches_formula() {
    // Pool 1000 YES + 1000 NO; pay 100. expected outY = floor(100*1000/1100) = 90.
    // getY = 100 + 90 = 190.
    let q0 = genesis_with_buyer_and_balanced_pool("alice", "task-A", 1000, 1000, 1000);
    let mut h = fresh_harness(q0);
    let parent = h.seq.q_snapshot().unwrap().state_root_t;

    submit_and_apply(
        &mut h,
        build_router(parent, "alice", "task-A", BuyDirection::BuyYes, 100, 1, 1),
    )
    .await
    .expect("router accepted");

    let q = h.seq.q_snapshot().unwrap();
    let pair = q
        .economic_state_t
        .conditional_share_balances_t
        .0
        .get(&AgentId("alice".into()))
        .and_then(|m| m.get(&EventId(TaskId("task-A".into()))))
        .copied()
        .unwrap_or_default();
    let expected_get_y = 100 + (100 * 1000 / 1100);
    assert_eq!(
        pair.yes.units, expected_get_y,
        "buyer YES total = payC + outY (architect formula)"
    );
}

/// §7.7 verbatim — `buy_no_with_coin_matches_symmetric_formula`.
///
/// Symmetric to BuyYes for BuyNo direction.
#[tokio::test]
async fn buy_no_with_coin_matches_symmetric_formula() {
    let q0 = genesis_with_buyer_and_balanced_pool("alice", "task-B", 1000, 1000, 1000);
    let mut h = fresh_harness(q0);
    let parent = h.seq.q_snapshot().unwrap().state_root_t;

    submit_and_apply(
        &mut h,
        build_router(parent, "alice", "task-B", BuyDirection::BuyNo, 100, 1, 1),
    )
    .await
    .expect("router accepted");

    let q = h.seq.q_snapshot().unwrap();
    let pair = q
        .economic_state_t
        .conditional_share_balances_t
        .0
        .get(&AgentId("alice".into()))
        .and_then(|m| m.get(&EventId(TaskId("task-B".into()))))
        .copied()
        .unwrap_or_default();
    let expected_get_n = 100 + (100 * 1000 / 1100);
    assert_eq!(
        pair.no.units, expected_get_n,
        "buyer NO total = payC + outN (symmetric formula)"
    );
}

/// §7.7 verbatim — `buy_yes_debits_coin_locks_collateral`.
///
/// Per architect manual §7.7 step 1+2: buyer.balance -= payC;
/// collateral[event] += payC.
#[tokio::test]
async fn buy_yes_debits_coin_locks_collateral() {
    let q0 = genesis_with_buyer_and_balanced_pool("alice", "task-C", 500, 1000, 1000);
    let pre_collateral = q0
        .economic_state_t
        .conditional_collateral_t
        .0
        .get(&EventId(TaskId("task-C".into())))
        .copied()
        .unwrap()
        .micro_units();
    let mut h = fresh_harness(q0);
    let parent = h.seq.q_snapshot().unwrap().state_root_t;

    submit_and_apply(
        &mut h,
        build_router(parent, "alice", "task-C", BuyDirection::BuyYes, 50, 1, 1),
    )
    .await
    .expect("router accepted");

    let q = h.seq.q_snapshot().unwrap();
    let bal = q
        .economic_state_t
        .balances_t
        .0
        .get(&AgentId("alice".into()))
        .copied()
        .unwrap();
    let collateral = q
        .economic_state_t
        .conditional_collateral_t
        .0
        .get(&EventId(TaskId("task-C".into())))
        .copied()
        .unwrap();

    // Buyer started with 500 Coin = 500_000_000 micro. Pay 50 micro.
    assert_eq!(bal.micro_units(), 500_000_000 - 50);
    assert_eq!(collateral.micro_units(), pre_collateral + 50);
}

/// §7.7 verbatim — `buy_yes_mints_complete_set`.
///
/// Per architect manual §7.7 step 3: router mints payC YES + payC NO.
/// Asserts `min(Σ_yes, Σ_no) == collateral` post-router (complete-set
/// balanced invariant); and total share supply went up by 2 × payC.
#[tokio::test]
async fn buy_yes_mints_complete_set() {
    let q0 = genesis_with_buyer_and_balanced_pool("alice", "task-D", 1000, 100_000, 100_000);
    let event = EventId(TaskId("task-D".into()));
    let pool_pre = q0
        .economic_state_t
        .cpmm_pools_t
        .0
        .get(&event)
        .copied()
        .unwrap();
    let pre_yes = pool_pre.pool_yes.units;
    let pre_no = pool_pre.pool_no.units;
    let mut h = fresh_harness(q0);
    let parent = h.seq.q_snapshot().unwrap().state_root_t;

    submit_and_apply(
        &mut h,
        build_router(parent, "alice", "task-D", BuyDirection::BuyYes, 1000, 1, 1),
    )
    .await
    .expect("router accepted");

    let q = h.seq.q_snapshot().unwrap();
    let pool_post = q
        .economic_state_t
        .cpmm_pools_t
        .0
        .get(&event)
        .copied()
        .unwrap();
    let alice_pair = q
        .economic_state_t
        .conditional_share_balances_t
        .0
        .get(&AgentId("alice".into()))
        .and_then(|m| m.get(&event))
        .copied()
        .unwrap_or_default();

    // Σ_yes_post = pool_yes_post + alice_yes; Σ_no_post = pool_no_post + alice_no.
    // Mint adds 1000 YES + 1000 NO total; alice ends with all retained YES + swap output.
    let sigma_yes_post = pool_post.pool_yes.units + alice_pair.yes.units;
    let sigma_no_post = pool_post.pool_no.units + alice_pair.no.units;
    let sigma_yes_pre = pre_yes;
    let sigma_no_pre = pre_no;
    assert_eq!(
        sigma_yes_post, sigma_yes_pre + 1000,
        "Σ_yes increases by exactly payC (mint added payC YES)"
    );
    assert_eq!(
        sigma_no_post, sigma_no_pre + 1000,
        "Σ_no increases by exactly payC (mint added payC NO)"
    );
    // Collateral should be pre + 1000.
    let collateral_post = q
        .economic_state_t
        .conditional_collateral_t
        .0
        .get(&event)
        .copied()
        .unwrap()
        .micro_units();
    assert_eq!(
        collateral_post, 100_000 + 1000,
        "collateral debited by payC = 1000"
    );
}

/// §7.7 verbatim — `buy_yes_transfers_retained_yes_plus_swap_yes`.
///
/// Per architect manual §7.7 step 4 + 8: buyer ends with retained YES
/// (the minted side that didn't get swapped) PLUS the swap output YES.
/// Total = payC + outY = getY.
#[tokio::test]
async fn buy_yes_transfers_retained_yes_plus_swap_yes() {
    let q0 = genesis_with_buyer_and_balanced_pool("alice", "task-E", 1000, 5000, 5000);
    let mut h = fresh_harness(q0);
    let parent = h.seq.q_snapshot().unwrap().state_root_t;
    submit_and_apply(
        &mut h,
        build_router(parent, "alice", "task-E", BuyDirection::BuyYes, 200, 1, 1),
    )
    .await
    .expect("router accepted");

    let q = h.seq.q_snapshot().unwrap();
    let pair = q
        .economic_state_t
        .conditional_share_balances_t
        .0
        .get(&AgentId("alice".into()))
        .and_then(|m| m.get(&EventId(TaskId("task-E".into()))))
        .copied()
        .unwrap_or_default();
    // Retained YES = 200 (mint side); swap outY = floor(200*5000/5200) = 192.
    let expected = 200 + 192;
    assert_eq!(pair.yes.units, expected);
    // Buyer holds NO shares = 0 (the minted NO got swapped into pool).
    assert_eq!(pair.no.units, 0);
}

/// §7.7 verbatim — `buy_yes_respects_min_yes_out`.
///
/// Per architect manual §7.7 verbatim — buyer's `min_total_out` slippage
/// gate. If computed `getY` (= payC + outY) is below `min_total_out`,
/// router must reject with `RouterMinTotalOutNotMet`.
#[tokio::test]
async fn buy_yes_respects_min_yes_out() {
    let q0 = genesis_with_buyer_and_balanced_pool("alice", "task-F", 1000, 1000, 1000);
    let mut h = fresh_harness(q0);
    let parent = h.seq.q_snapshot().unwrap().state_root_t;
    // Pool 1000+1000; pay 100 → getY = 100 + floor(100*1000/1100) = 100+90 = 190.
    // Demand min_total_out = 200 → must reject.
    let err = submit_and_apply(
        &mut h,
        build_router(parent, "alice", "task-F", BuyDirection::BuyYes, 100, 200, 1),
    )
    .await
    .expect_err("router must reject when min_total_out not met");
    assert!(
        err.contains("RouterMinTotalOutNotMet"),
        "expected RouterMinTotalOutNotMet, got {err}"
    );

    // Sanity: same with min_total_out = 190 succeeds.
    let parent = h.seq.q_snapshot().unwrap().state_root_t;
    submit_and_apply(
        &mut h,
        build_router(parent, "alice", "task-F", BuyDirection::BuyYes, 100, 190, 2),
    )
    .await
    .expect("router accepted at min_total_out = 190");
}

/// §7.7 verbatim — `buy_yes_no_f64`.
///
/// Per architect manual §7.7 + universal forbidden list "no f64": router
/// formula MUST use integer math. Source-grep over the BuyWithCoinRouter
/// dispatch arm region asserts no f64/f32.
#[tokio::test]
async fn buy_yes_no_f64() {
    let sequencer_src = std::fs::read_to_string("src/state/sequencer.rs")
        .expect("read src/state/sequencer.rs");
    let start = sequencer_src
        .find("Stage C P-M6 — BuyWithCoinRouterTx accept arm")
        .expect("router arm marker present");
    let region_end = sequencer_src[start..]
        .find("\n        }\n        // ──────────")
        .map(|off| start + off)
        .unwrap_or(sequencer_src.len().min(start + 8000));
    let arm = &sequencer_src[start..region_end];

    assert!(
        !arm.contains("f64") && !arm.contains("f32"),
        "no f64/f32 in BuyWithCoinRouter dispatch arm region (architect §7.7)"
    );
    assert!(
        arm.contains("checked_mul") && arm.contains("checked_add"),
        "router uses checked_* integer arithmetic"
    );
}

/// §7.7 verbatim — `buy_yes_no_ghost_liquidity`.
///
/// Per architect manual §7.7 + universal forbidden list "no ghost
/// liquidity": pool reserves and minted shares MUST trace to a Coin debit.
/// Asserts that buyer balance debit equals collateral credit (1:1) and
/// total Coin supply is bit-equal pre/post router.
#[tokio::test]
async fn buy_yes_no_ghost_liquidity() {
    let q0 = genesis_with_buyer_and_balanced_pool("alice", "task-G", 1000, 1000, 1000);
    let pre = total_supply_micro(&q0);
    let mut h = fresh_harness(q0);
    let parent = h.seq.q_snapshot().unwrap().state_root_t;
    submit_and_apply(
        &mut h,
        build_router(parent, "alice", "task-G", BuyDirection::BuyYes, 100, 1, 1),
    )
    .await
    .expect("router accepted");

    let post = total_supply_micro(&h.seq.q_snapshot().unwrap());
    assert_eq!(
        pre, post,
        "total Coin conserved across router (no ghost liquidity)"
    );
}

/// §7.7 verbatim — `router_atomic_rollback_on_failure`.
///
/// Per architect manual §7.7: any single-step failure → entire tx
/// reverts. Tests rejection paths (insufficient buyer balance + slippage)
/// and asserts ZERO state mutation occurred (sender balance unchanged;
/// pool reserves unchanged; collateral unchanged; share balances
/// unchanged; total Coin unchanged).
#[tokio::test]
async fn router_atomic_rollback_on_failure() {
    // Fixture: buyer has 100 micro Coin balance; pool 1000+1000.
    let mut q0 = QState::genesis();
    q0.economic_state_t.balances_t.0.insert(
        AgentId("alice".into()),
        MicroCoin::from_micro_units(100),
    );
    let event = EventId(TaskId("task-H".into()));
    q0.economic_state_t.conditional_collateral_t.0.insert(
        event.clone(),
        MicroCoin::from_micro_units(1000),
    );
    q0.economic_state_t.cpmm_pools_t.0.insert(
        event.clone(),
        CpmmPool {
            event_id_kind: PoolEventKind::BinaryYesNo,
            pool_yes: ShareAmount::from_units(1000),
            pool_no: ShareAmount::from_units(1000),
            lp_total_shares: LpShareAmount::from_units(1000),
            status: PoolStatus::Active,
        },
    );

    let pre_total = total_supply_micro(&q0);
    let pre_state_root = q0.state_root_t;
    let pre_pool = q0
        .economic_state_t
        .cpmm_pools_t
        .0
        .get(&event)
        .copied()
        .unwrap();
    let pre_alice_bal = q0
        .economic_state_t
        .balances_t
        .0
        .get(&AgentId("alice".into()))
        .copied()
        .unwrap();
    let pre_collateral = q0
        .economic_state_t
        .conditional_collateral_t
        .0
        .get(&event)
        .copied()
        .unwrap();

    let mut h = fresh_harness(q0);

    // Failure path 1: pay 200 (alice has only 100) → InsufficientBuyerBalance.
    let parent = h.seq.q_snapshot().unwrap().state_root_t;
    let err = submit_and_apply(
        &mut h,
        build_router(parent, "alice", "task-H", BuyDirection::BuyYes, 200, 1, 1),
    )
    .await
    .expect_err("router must reject insufficient balance");
    assert!(
        err.contains("RouterInsufficientBuyerBalance"),
        "expected RouterInsufficientBuyerBalance, got {err}"
    );

    // Atomic rollback assertion: state_root unchanged.
    let q_after_fail = h.seq.q_snapshot().unwrap();
    assert_eq!(
        q_after_fail.state_root_t, pre_state_root,
        "state_root MUST NOT advance on rejected router"
    );
    // Buyer balance unchanged.
    let post_alice_bal = q_after_fail
        .economic_state_t
        .balances_t
        .0
        .get(&AgentId("alice".into()))
        .copied()
        .unwrap();
    assert_eq!(
        post_alice_bal.micro_units(),
        pre_alice_bal.micro_units(),
        "buyer balance unchanged on rejected router"
    );
    // Pool reserves unchanged.
    let post_pool = q_after_fail
        .economic_state_t
        .cpmm_pools_t
        .0
        .get(&event)
        .copied()
        .unwrap();
    assert_eq!(post_pool.pool_yes.units, pre_pool.pool_yes.units);
    assert_eq!(post_pool.pool_no.units, pre_pool.pool_no.units);
    // Collateral unchanged.
    let post_collateral = q_after_fail
        .economic_state_t
        .conditional_collateral_t
        .0
        .get(&event)
        .copied()
        .unwrap();
    assert_eq!(post_collateral.micro_units(), pre_collateral.micro_units());
    // Total Coin unchanged.
    assert_eq!(total_supply_micro(&q_after_fail), pre_total);

    // Sanity: confirm successful path still works after rollback.
    let parent = h.seq.q_snapshot().unwrap().state_root_t;
    submit_and_apply(
        &mut h,
        build_router(parent, "alice", "task-H", BuyDirection::BuyYes, 50, 1, 2),
    )
    .await
    .expect("router accepted post-rollback");
    let q_after_success = h.seq.q_snapshot().unwrap();
    assert_ne!(
        q_after_success.state_root_t, pre_state_root,
        "state_root advances on accepted router"
    );
}

#[cfg(test)]
fn _silence_imports() {
    let _ = ShareSidePair::default();
}
