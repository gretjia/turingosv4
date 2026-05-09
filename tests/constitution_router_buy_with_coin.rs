//! Constitution gate — Stage C P-M6 / Phase F.5 BuyWithCoinRouter Mint-and-Swap
//! composite tx semantics per architect manual §7.7.
//!
//! Authority: `handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_en.md`
//! §7.7 (verbatim BuyYesWithCoinRouter / BuyNoWithCoinRouter 9-step composite
//! + 9 mandatory tests + integer formulas).
//! Companion: `handover/directives/2026-05-09_STAGE_C_POLYMARKET_VETO_REMEDIATION_DIRECTIVE.md`
//! §1.C row 5 (Class-4 STEP_B rebuild post Stage C VETO; Defect 1 + Defect 2
//! patches).
//!
//! Test names mirror architect §7.7 verbatim list:
//!   1. buy_yes_with_coin_matches_formula
//!   2. buy_no_with_coin_matches_symmetric_formula
//!   3. buy_yes_debits_coin_locks_collateral
//!   4. buy_yes_mints_complete_set
//!   5. buy_yes_transfers_retained_yes_plus_swap_yes
//!   6. buy_yes_respects_min_yes_out
//!   7. buy_yes_no_f64
//!   8. buy_yes_no_ghost_liquidity
//!   9. router_atomic_rollback_on_failure
//!
//! Each behavioral test exercises the actual sequencer accept arm via
//! `submit_and_apply` against a real `Sequencer` + `InMemoryLedgerWriter`.
//! Per `feedback_tape_first_real_tests`: no tape activity = not a TuringOS
//! test; these tests advance state_root_t through real transitions.
//!
//! Test 9 (`router_atomic_rollback_on_failure`) uses the cfg(test) failure-
//! injection hook (`TURINGOS_TEST_ROUTER_FAIL_AT_STEP` env var) per E.2
//! atomic-rollback witness gate — Codex G2 audit 2026-05-09 defect 2
//! mandates that this test exercise the mid-mutation rollback path, not a
//! pre-mutation rejection.

use std::sync::{Arc, Mutex, RwLock};

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
use turingosv4::state::q_state::{
    AgentId, QState, TaskId, TaskMarketEntry, TaskMarketState, TxId,
};
use turingosv4::state::sequencer::{Sequencer, SubmissionEnvelope};
use turingosv4::state::typed_tx::{
    AgentSignature, BuyDirection, BuyWithCoinRouterTx, CompleteSetMintTx,
    CpmmPoolTx, EventId, ShareAmount, TypedTx,
};
use turingosv4::top_white::predicates::registry::PredicateRegistry;

// Static lock for env-var manipulation in atomic-rollback test (per
// `feedback_env_var_test_lock` — process-global env-var mutation needs
// serialization across parallel tests).
static ENV_LOCK: Mutex<()> = Mutex::new(());

// ── Harness (mirrors constitution_cpmm_swap.rs pattern) ─────────────────────

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
    parent: turingosv4::state::q_state::Hash,
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

// Seed pattern: provider mints + creates symmetric pool. Returns the
// post-seed state_root for the next router tx to use as parent_state_root.
async fn seed_pool(
    h: &mut Harness,
    provider: &str,
    task: &str,
    pool_seed_units: u128,
) -> turingosv4::state::q_state::Hash {
    let p = h.seq.q_snapshot().unwrap().state_root_t;
    submit_and_apply(
        h,
        build_mint(p, provider, task, pool_seed_units as i64, 1),
    )
    .await
    .expect("provider mint accepted");
    let p = h.seq.q_snapshot().unwrap().state_root_t;
    submit_and_apply(
        h,
        build_pool(p, provider, task, pool_seed_units, pool_seed_units, 1),
    )
    .await
    .expect("pool creation accepted");
    h.seq.q_snapshot().unwrap().state_root_t
}

// ── Architect §7.7 verbatim test 1 ──────────────────────────────────────────

/// buy_yes_with_coin_matches_formula — architect §7.7 verbatim
/// "BuyYesWithCoinRouter":
///
/// ```
/// outY = floor(payC * poolY / (poolN + payC))
/// poolN1 = poolN + payC
/// poolY1 = poolY - outY
/// getY = payC + outY
/// ```
///
/// Setup: 5M/5M pool. Buyer pays payC = 1_000_000 micro-Coin. outY =
/// floor(1M * 5M / 6M) = 833_333. getY = 1_000_000 + 833_333 = 1_833_333
/// YES held by buyer post-state. Pool: poolN1 = 6_000_000; poolY1 =
/// 4_166_667. k_post >= k_pre (architect integer invariant `>=`).
#[tokio::test]
async fn buy_yes_with_coin_matches_formula() {
    let q0 = genesis_with_balances_and_open_task(
        &[("alice", 50), ("bob", 50)],
        "evt-1",
    );
    let mut h = fresh_harness(q0);

    let parent = seed_pool(&mut h, "alice", "evt-1", 5_000_000).await;

    let q_pre = h.seq.q_snapshot().unwrap();
    let pool_pre = q_pre
        .economic_state_t
        .cpmm_pools_t
        .0
        .get(&EventId(TaskId("evt-1".into())))
        .cloned()
        .expect("pool present pre-router");
    let k_pre = pool_pre.pool_yes.units * pool_pre.pool_no.units;
    let bob_balance_pre = q_pre
        .economic_state_t
        .balances_t
        .0
        .get(&AgentId("bob".into()))
        .copied()
        .unwrap();
    assert_eq!(bob_balance_pre.micro_units(), 50_000_000);

    // Bob buys YES with payC = 1_000_000 micro-Coin.
    submit_and_apply(
        &mut h,
        build_router(
            parent,
            "bob",
            "evt-1",
            BuyDirection::BuyYes,
            1_000_000,
            0, // no slippage cap
            1,
        ),
    )
    .await
    .expect("router tx accepted");

    let q_post = h.seq.q_snapshot().unwrap();
    let pool_post = q_post
        .economic_state_t
        .cpmm_pools_t
        .0
        .get(&EventId(TaskId("evt-1".into())))
        .cloned()
        .expect("pool present post-router");

    let expected_out_y: u128 = 833_333;
    let expected_pool_n: u128 = 6_000_000;
    let expected_pool_y: u128 = 5_000_000 - expected_out_y;
    assert_eq!(pool_post.pool_no.units, expected_pool_n);
    assert_eq!(pool_post.pool_yes.units, expected_pool_y);

    // Architect §7.7 verbatim: k1 >= k0.
    let k_post = pool_post.pool_yes.units * pool_post.pool_no.units;
    assert!(
        k_post >= k_pre,
        "architect §7.7 constant-product: pool_yes1 * pool_no1 >= pool_yes * pool_no"
    );

    // Buyer's YES balance: getY = payC + outY = 1_833_333.
    let bob_pair = q_post
        .economic_state_t
        .conditional_share_balances_t
        .0
        .get(&AgentId("bob".into()))
        .and_then(|m| m.get(&EventId(TaskId("evt-1".into()))))
        .copied()
        .expect("bob shares post-router");
    assert_eq!(
        bob_pair.yes.units,
        1_000_000 + expected_out_y,
        "buyer holds payC + outY YES"
    );
    assert_eq!(bob_pair.no.units, 0, "BuyYes: buyer's NO inventory unchanged (zero)");
}

// ── Architect §7.7 verbatim test 2 ──────────────────────────────────────────

/// buy_no_with_coin_matches_symmetric_formula — architect §7.7 verbatim
/// "BuyNoWithCoinRouter" symmetric:
///
/// ```
/// outN = floor(payC * poolN / (poolY + payC))
/// poolY1 = poolY + payC
/// poolN1 = poolN - outN
/// getN = payC + outN
/// ```
///
/// Setup: 4M/4M pool. Buyer pays payC = 2_000_000. outN = floor(2M * 4M /
/// 6M) = 1_333_333. getN = 3_333_333. Pool: poolY1 = 6M; poolN1 =
/// 4M - 1_333_333 = 2_666_667.
#[tokio::test]
async fn buy_no_with_coin_matches_symmetric_formula() {
    let q0 = genesis_with_balances_and_open_task(
        &[("alice", 50), ("bob", 50)],
        "evt-2",
    );
    let mut h = fresh_harness(q0);

    let parent = seed_pool(&mut h, "alice", "evt-2", 4_000_000).await;

    let q_pre = h.seq.q_snapshot().unwrap();
    let pool_pre = q_pre
        .economic_state_t
        .cpmm_pools_t
        .0
        .get(&EventId(TaskId("evt-2".into())))
        .cloned()
        .expect("pool present pre-router");
    let k_pre = pool_pre.pool_yes.units * pool_pre.pool_no.units;

    submit_and_apply(
        &mut h,
        build_router(
            parent,
            "bob",
            "evt-2",
            BuyDirection::BuyNo,
            2_000_000,
            0,
            1,
        ),
    )
    .await
    .expect("router tx accepted");

    let q_post = h.seq.q_snapshot().unwrap();
    let pool_post = q_post
        .economic_state_t
        .cpmm_pools_t
        .0
        .get(&EventId(TaskId("evt-2".into())))
        .cloned()
        .expect("pool present post-router");

    let expected_out_n: u128 = 1_333_333;
    assert_eq!(pool_post.pool_yes.units, 6_000_000);
    assert_eq!(pool_post.pool_no.units, 4_000_000 - expected_out_n);

    let k_post = pool_post.pool_yes.units * pool_post.pool_no.units;
    assert!(k_post >= k_pre, "architect §7.7 constant-product");

    let bob_pair = q_post
        .economic_state_t
        .conditional_share_balances_t
        .0
        .get(&AgentId("bob".into()))
        .and_then(|m| m.get(&EventId(TaskId("evt-2".into()))))
        .copied()
        .expect("bob shares post-router");
    assert_eq!(bob_pair.no.units, 2_000_000 + expected_out_n);
    assert_eq!(bob_pair.yes.units, 0);
}

// ── Architect §7.7 verbatim test 3 ──────────────────────────────────────────

/// buy_yes_debits_coin_locks_collateral — architect §7.7 step 1 + step 2:
/// `balances_t[buyer] -= payC` AND `conditional_collateral_t[event] +=
/// payC`. Witness: bob's balance pre/post + collateral pre/post. Coin
/// conservation: balances_t -1 payC, collateral +1 payC ⇒ total Coin
/// preserved.
#[tokio::test]
async fn buy_yes_debits_coin_locks_collateral() {
    let q0 = genesis_with_balances_and_open_task(
        &[("alice", 50), ("bob", 50)],
        "evt-3",
    );
    let mut h = fresh_harness(q0);

    let parent = seed_pool(&mut h, "alice", "evt-3", 3_000_000).await;

    let q_pre = h.seq.q_snapshot().unwrap();
    let bob_balance_pre = q_pre
        .economic_state_t
        .balances_t
        .0
        .get(&AgentId("bob".into()))
        .copied()
        .unwrap();
    let collateral_pre = q_pre
        .economic_state_t
        .conditional_collateral_t
        .0
        .get(&EventId(TaskId("evt-3".into())))
        .copied()
        .unwrap_or(MicroCoin::zero());

    let pay_coin: i64 = 500_000;
    submit_and_apply(
        &mut h,
        build_router(parent, "bob", "evt-3", BuyDirection::BuyYes, pay_coin, 0, 1),
    )
    .await
    .expect("router tx accepted");

    let q_post = h.seq.q_snapshot().unwrap();
    let bob_balance_post = q_post
        .economic_state_t
        .balances_t
        .0
        .get(&AgentId("bob".into()))
        .copied()
        .unwrap();
    let collateral_post = q_post
        .economic_state_t
        .conditional_collateral_t
        .0
        .get(&EventId(TaskId("evt-3".into())))
        .copied()
        .unwrap();

    // Step 1 witness: buyer Coin debited by payC.
    assert_eq!(
        bob_balance_post.micro_units(),
        bob_balance_pre.micro_units() - pay_coin,
        "balances_t[buyer] -= payC"
    );
    // Step 2 witness: collateral locked by payC.
    assert_eq!(
        collateral_post.micro_units(),
        collateral_pre.micro_units() + pay_coin,
        "conditional_collateral_t[event] += payC"
    );

    // Coin conservation (Defect-1 patch enforced strict): total Coin
    // preserved across the router tx.
    assert_total_ctf_conserved(
        &q_pre.economic_state_t,
        &q_post.economic_state_t,
        &[],
    )
    .expect("total Coin conserved across router (pure Coin→collateral migration)");
}

// ── Architect §7.7 verbatim test 4 ──────────────────────────────────────────

/// buy_yes_mints_complete_set — architect §7.7 step 3: synthetic mint of
/// `payC YES + payC NO` per the locked collateral. Verified through the
/// strict complete-set balance invariant (Defect-1 patch): post-router
/// state must satisfy `sum_yes == sum_no == collateral` on the symmetric
/// branch — i.e., the complete set was minted in symmetric pairs.
///
/// Trace: pre-state has provider's pool with `5M YES + 5M NO == 5M
/// collateral`. Post-router with payC=1M:
///   collateral_post = 5M + 1M = 6M
///   sum_yes_post = (alice's 5M-3M=2M) + buyer.yes (1M+833_333) +
///                  pool.pool_yes (5M-833_333) = 2M + 1_833_333 + 4_166_667 = 8M? Wait
///   Actually let me re-derive. Pre-pool state after seed:
///     alice mints 5M micro → alice.yes = 5M, alice.no = 5M.
///     pool created with seed_yes=5M, seed_no=5M → alice.yes -= 5M, alice.no -= 5M; pool.pool_yes=5M, pool.pool_no=5M; alice.lp += 5M.
///   So: alice.yes = 0, alice.no = 0. Sum_yes_pre = 0 + pool.pool_yes(5M) = 5M.
///   Sum_no_pre = 0 + pool.pool_no(5M) = 5M.
///   collateral_pre = 5M (locked at mint time).
///   Post-router (BuyYes, payC=1M): out_y = 833_333.
///     alice.yes/no unchanged.
///     bob.yes = 1M + 833_333 = 1_833_333.
///     pool.pool_yes = 5M - 833_333 = 4_166_667.
///     pool.pool_no = 5M + 1M = 6M.
///     collateral = 5M + 1M = 6M.
///   Sum_yes = 0 + 1_833_333 + 4_166_667 = 6M.
///   Sum_no  = 0 + 0 + 6M = 6M.
///   Sum_yes == sum_no == collateral ✓ (complete-set mint witnessed).
#[tokio::test]
async fn buy_yes_mints_complete_set() {
    let q0 = genesis_with_balances_and_open_task(
        &[("alice", 50), ("bob", 50)],
        "evt-4",
    );
    let mut h = fresh_harness(q0);

    let parent = seed_pool(&mut h, "alice", "evt-4", 5_000_000).await;

    submit_and_apply(
        &mut h,
        build_router(parent, "bob", "evt-4", BuyDirection::BuyYes, 1_000_000, 0, 1),
    )
    .await
    .expect("router tx accepted");

    let q_post = h.seq.q_snapshot().unwrap();

    // Post-state symmetric-branch strict equality check: the only way this
    // holds is if the synthetic complete-set mint added equal YES + NO
    // claims (architect §7.7 step 3) AND the swap rotated symmetrically
    // within sides (steps 5-8).
    assert_complete_set_balanced(&q_post.economic_state_t)
        .expect("complete-set balance preserved post-router (strict symmetric)");

    // Direct-witness sums.
    let collateral = q_post
        .economic_state_t
        .conditional_collateral_t
        .0
        .get(&EventId(TaskId("evt-4".into())))
        .copied()
        .unwrap();
    assert_eq!(collateral.micro_units(), 6_000_000, "collateral = pre + payC");

    let sum_yes = sum_event_yes(&q_post, "evt-4");
    let sum_no = sum_event_no(&q_post, "evt-4");
    assert_eq!(sum_yes, sum_no, "post-router YES + NO totals symmetric");
    assert_eq!(sum_yes, 6_000_000_u128, "totals match collateral.micro_units");
}

// Helpers for sum across all (traders + pool) for an event.
fn sum_event_yes(q: &QState, task: &str) -> u128 {
    let event_id = EventId(TaskId(task.into()));
    let mut s: u128 = 0;
    for owner_map in q.economic_state_t.conditional_share_balances_t.0.values() {
        if let Some(pair) = owner_map.get(&event_id) {
            s += pair.yes.units;
        }
    }
    if let Some(pool) = q.economic_state_t.cpmm_pools_t.0.get(&event_id) {
        s += pool.pool_yes.units;
    }
    s
}
fn sum_event_no(q: &QState, task: &str) -> u128 {
    let event_id = EventId(TaskId(task.into()));
    let mut s: u128 = 0;
    for owner_map in q.economic_state_t.conditional_share_balances_t.0.values() {
        if let Some(pair) = owner_map.get(&event_id) {
            s += pair.no.units;
        }
    }
    if let Some(pool) = q.economic_state_t.cpmm_pools_t.0.get(&event_id) {
        s += pool.pool_no.units;
    }
    s
}

// ── Architect §7.7 verbatim test 5 ──────────────────────────────────────────

/// buy_yes_transfers_retained_yes_plus_swap_yes — architect §7.7 step 4 +
/// step 8: buyer ends with `payC + outY` YES (retained mint side + swap
/// output). Witnesses both transfers separately by computing payC and
/// outY independently and asserting the sum.
#[tokio::test]
async fn buy_yes_transfers_retained_yes_plus_swap_yes() {
    let q0 = genesis_with_balances_and_open_task(
        &[("alice", 50), ("bob", 50)],
        "evt-5",
    );
    let mut h = fresh_harness(q0);

    let parent = seed_pool(&mut h, "alice", "evt-5", 5_000_000).await;

    let pay_coin: i64 = 1_000_000;
    submit_and_apply(
        &mut h,
        build_router(parent, "bob", "evt-5", BuyDirection::BuyYes, pay_coin, 0, 1),
    )
    .await
    .expect("router tx accepted");

    let q_post = h.seq.q_snapshot().unwrap();
    let bob_pair = q_post
        .economic_state_t
        .conditional_share_balances_t
        .0
        .get(&AgentId("bob".into()))
        .and_then(|m| m.get(&EventId(TaskId("evt-5".into()))))
        .copied()
        .expect("bob shares post-router");

    // Step 4 retained: payC = 1_000_000 YES.
    // Step 7 + 8 swap output: outY = floor(1M * 5M / 6M) = 833_333.
    // Step 9 total: getY = 1_833_333.
    let expected_retained_yes: u128 = pay_coin as u128;
    let expected_swap_yes: u128 = 833_333;
    let expected_get_y: u128 = expected_retained_yes + expected_swap_yes;

    assert_eq!(
        bob_pair.yes.units, expected_get_y,
        "buyer YES = retained payC + swap outY (architect §7.7 step 9 getY)"
    );
    assert_eq!(bob_pair.no.units, 0, "BuyYes: buyer's NO untouched");
}

// ── Architect §7.7 verbatim test 6 ──────────────────────────────────────────

/// buy_yes_respects_min_yes_out — architect §7.7 implicit slippage gate:
/// trader sets `min_out_shares`; admission rejects if computed
/// out_shares < min_out_shares. Witnessed at exact-floor (accept) +
/// one-above-floor (reject) boundaries.
#[tokio::test]
async fn buy_yes_respects_min_yes_out() {
    let q0 = genesis_with_balances_and_open_task(
        &[("alice", 50), ("bob", 50)],
        "evt-6",
    );
    let mut h = fresh_harness(q0);

    let parent = seed_pool(&mut h, "alice", "evt-6", 5_000_000).await;

    // Attempt 1: greedy min_out = 1_000_000; expected outY = 833_333. Reject.
    let err = submit_and_apply(
        &mut h,
        build_router(
            parent,
            "bob",
            "evt-6",
            BuyDirection::BuyYes,
            1_000_000,
            1_000_000,
            1,
        ),
    )
    .await
    .expect_err("greedy min_out must be rejected");
    assert!(
        err.contains("RouterSlippageExceeded"),
        "expected RouterSlippageExceeded, got: {err}"
    );

    // Attempt 2: tight min_out = 833_334 (one above formula). Reject.
    let err = submit_and_apply(
        &mut h,
        build_router(
            parent,
            "bob",
            "evt-6",
            BuyDirection::BuyYes,
            1_000_000,
            833_334,
            2,
        ),
    )
    .await
    .expect_err("min_out one-above-floor must be rejected");
    assert!(err.contains("RouterSlippageExceeded"));

    // Attempt 3: exact min_out = 833_333. Accept.
    submit_and_apply(
        &mut h,
        build_router(
            parent,
            "bob",
            "evt-6",
            BuyDirection::BuyYes,
            1_000_000,
            833_333,
            3,
        ),
    )
    .await
    .expect("exact min_out accepted");

    // Witness state advanced.
    let q_post = h.seq.q_snapshot().unwrap();
    let pool_post = q_post
        .economic_state_t
        .cpmm_pools_t
        .0
        .get(&EventId(TaskId("evt-6".into())))
        .cloned()
        .expect("pool present");
    assert_eq!(pool_post.pool_yes.units, 5_000_000 - 833_333);
    assert_eq!(pool_post.pool_no.units, 6_000_000);
}

// ── Architect §7.7 verbatim test 7 ──────────────────────────────────────────

/// buy_yes_no_f64 — architect §7.7 integer-math gate: source-grep for real
/// floating-point usage in the BuyWithCoinRouter admission arm
/// (`src/state/sequencer.rs`) and the `BuyWithCoinRouterTx` /
/// `BuyWithCoinRouterSigningPayload` surfaces (`src/state/typed_tx.rs`).
/// Architect §7.7 verbatim: integer formulas with floor rounding only;
/// floating-point would break replay determinism + introduce monetary
/// drift.
///
/// Detection patterns (mirror P-M5 source-grep):
///   `: f64` / `: f32`     — type annotations
///   `as f64` / `as f32`   — casts
///   `f64::` / `f32::`     — associated function calls
///   digit + `f64` / `f32` — float literals
#[test]
fn buy_yes_no_f64() {
    use std::path::PathBuf;
    let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let sequencer_src = workspace.join("src/state/sequencer.rs");
    let typed_tx_src = workspace.join("src/state/typed_tx.rs");

    let seq_text = std::fs::read_to_string(&sequencer_src)
        .expect("read src/state/sequencer.rs");
    let tx_text = std::fs::read_to_string(&typed_tx_src)
        .expect("read src/state/typed_tx.rs");

    let arm_start = seq_text
        .find("TypedTx::BuyWithCoinRouter(router) => {")
        .expect("BuyWithCoinRouter admission arm present");
    let arm_end = (arm_start + 12_288).min(seq_text.len());
    let arm_body = &seq_text[arm_start..arm_end];

    let forbidden_real_usage =
        [": f64", ": f32", "as f64", "as f32", "f64::", "f32::"];
    let forbidden_literal_suffix = ["f64", "f32"];

    for needle in &forbidden_real_usage {
        assert!(
            !arm_body.contains(needle),
            "architect §7.7 integer-math: BuyWithCoinRouter admission arm contains forbidden token {needle:?}"
        );
    }
    for suffix in &forbidden_literal_suffix {
        let mut search_from = 0usize;
        while let Some(idx_rel) = arm_body[search_from..].find(suffix) {
            let idx = search_from + idx_rel;
            let preceding = arm_body[..idx].chars().last();
            if matches!(preceding, Some(c) if c.is_ascii_digit() || c == '.') {
                panic!(
                    "architect §7.7 integer-math: BuyWithCoinRouter admission arm contains float literal ending in {suffix:?} at byte offset {idx}"
                );
            }
            search_from = idx + suffix.len();
        }
    }

    let tx_struct_start = tx_text
        .find("pub struct BuyWithCoinRouterTx {")
        .expect("BuyWithCoinRouterTx struct present");
    let tx_struct_end = tx_text[tx_struct_start..]
        .find("}\n")
        .map(|i| tx_struct_start + i)
        .unwrap_or(tx_text.len());
    let tx_struct_body = &tx_text[tx_struct_start..tx_struct_end];
    for needle in &forbidden_real_usage {
        assert!(
            !tx_struct_body.contains(needle),
            "architect §7.7: BuyWithCoinRouterTx struct contains forbidden token {needle:?}"
        );
    }

    let payload_start = tx_text
        .find("pub struct BuyWithCoinRouterSigningPayload {")
        .expect("BuyWithCoinRouterSigningPayload struct present");
    let payload_end = tx_text[payload_start..]
        .find("}\n")
        .map(|i| payload_start + i)
        .unwrap_or(tx_text.len());
    let payload_body = &tx_text[payload_start..payload_end];
    for needle in &forbidden_real_usage {
        assert!(
            !payload_body.contains(needle),
            "architect §7.7: BuyWithCoinRouterSigningPayload contains forbidden token {needle:?}"
        );
    }

    // Sanity: integer math markers present (u128 numer/denom division).
    assert!(
        seq_text.contains("let denom = pool_input_units_pre"),
        "BuyWithCoinRouter arm must compute denom via u128 checked_add"
    );
    assert!(
        seq_text.contains("let out_shares: u128 = numer / denom;"),
        "BuyWithCoinRouter arm must use u128 integer division for out_shares"
    );
}

// ── Architect §7.7 verbatim test 8 ──────────────────────────────────────────

/// buy_yes_no_ghost_liquidity — architect §6 forbidden list "No automatic
/// per-node 100 YES + 100 NO without collateral". The router LOCKS
/// collateral for every payC; YES + NO inventory created via the
/// synthetic mint is fully backed by the locked Coin. Witness: post-router
/// state has `conditional_collateral_t[event] == sum_yes == sum_no` (no
/// shares without collateral; no collateral without claims).
#[tokio::test]
async fn buy_yes_no_ghost_liquidity() {
    let q0 = genesis_with_balances_and_open_task(
        &[("alice", 50), ("bob", 50), ("carol", 50)],
        "evt-7",
    );
    let mut h = fresh_harness(q0);

    let parent = seed_pool(&mut h, "alice", "evt-7", 4_000_000).await;

    // Two router buys to stress the collateral-shares balance.
    submit_and_apply(
        &mut h,
        build_router(parent, "bob", "evt-7", BuyDirection::BuyYes, 1_500_000, 0, 1),
    )
    .await
    .expect("first router accepted");
    let parent2 = h.seq.q_snapshot().unwrap().state_root_t;
    submit_and_apply(
        &mut h,
        build_router(parent2, "carol", "evt-7", BuyDirection::BuyNo, 800_000, 0, 2),
    )
    .await
    .expect("second router accepted");

    let q_post = h.seq.q_snapshot().unwrap();
    let collateral = q_post
        .economic_state_t
        .conditional_collateral_t
        .0
        .get(&EventId(TaskId("evt-7".into())))
        .copied()
        .unwrap();
    let sum_yes = sum_event_yes(&q_post, "evt-7");
    let sum_no = sum_event_no(&q_post, "evt-7");

    // Architect §6 enforcement: shares are FULLY backed by collateral; no
    // ghost liquidity (no shares without locked Coin).
    assert_eq!(
        sum_yes, collateral.micro_units() as u128,
        "sum YES claims (traders + pool) == locked collateral"
    );
    assert_eq!(
        sum_no, collateral.micro_units() as u128,
        "sum NO claims (traders + pool) == locked collateral"
    );

    // Strict complete-set balance check (P-M4 + Phase E.3 refactored).
    assert_complete_set_balanced(&q_post.economic_state_t)
        .expect("complete-set balance preserved (no ghost liquidity)");
}

// ── Architect §7.7 verbatim test 9 (Defect-2 patch witness) ─────────────────

/// router_atomic_rollback_on_failure — architect §7.7 + Codex G2 audit
/// 2026-05-09 defect 2: the 9-step composite tx must be atomic. This test
/// uses the cfg(test) failure-injection hook
/// `TURINGOS_TEST_ROUTER_FAIL_AT_STEP` to force failure at step 5 (mid-
/// composite, AFTER steps 1-4 mutated q_next but BEFORE the swap pool
/// mutation in step 7) and witnesses that the original `q.state_root` is
/// UNCHANGED post-failure (rollback = drop q_next; q is intact).
///
/// E.2 atomic-rollback witness gate (`tests/constitution_class4_atomic_
/// rollback_witness.rs`) verifies this test invokes
/// `set_var("TURINGOS_TEST_ROUTER_FAIL_AT_STEP", ...)` — vacuous tests
/// (e.g., trigger insufficient-balance and never reach mutation) would
/// fail the static-layer scan.
#[tokio::test]
async fn router_atomic_rollback_on_failure() {
    let _guard = ENV_LOCK.lock().expect("env lock");

    let q0 = genesis_with_balances_and_open_task(
        &[("alice", 50), ("bob", 50)],
        "evt-8",
    );
    let mut h = fresh_harness(q0);

    let parent_after_seed = seed_pool(&mut h, "alice", "evt-8", 5_000_000).await;

    let q_pre = h.seq.q_snapshot().unwrap();
    let state_root_pre = q_pre.state_root_t;
    let bob_balance_pre = q_pre
        .economic_state_t
        .balances_t
        .0
        .get(&AgentId("bob".into()))
        .copied()
        .unwrap();
    let collateral_pre = q_pre
        .economic_state_t
        .conditional_collateral_t
        .0
        .get(&EventId(TaskId("evt-8".into())))
        .copied()
        .unwrap_or(MicroCoin::zero());
    let pool_pre = q_pre
        .economic_state_t
        .cpmm_pools_t
        .0
        .get(&EventId(TaskId("evt-8".into())))
        .cloned()
        .expect("pool pre-injection");

    // Force failure at step 5 (mid-composite: post Coin debit + collateral
    // lock + retained-side credit; pre swap pool mutation). Witness: state
    // is FULLY restored — Coin not debited; collateral not increased; pool
    // unchanged; state_root unchanged.
    std::env::set_var("TURINGOS_TEST_ROUTER_FAIL_AT_STEP", "5");
    let result = submit_and_apply(
        &mut h,
        build_router(
            parent_after_seed,
            "bob",
            "evt-8",
            BuyDirection::BuyYes,
            1_000_000,
            0,
            1,
        ),
    )
    .await;
    std::env::remove_var("TURINGOS_TEST_ROUTER_FAIL_AT_STEP");

    assert!(
        result.is_err(),
        "router must fail when TURINGOS_TEST_ROUTER_FAIL_AT_STEP injects step 5 failure"
    );
    let err = result.unwrap_err();
    assert!(
        err.contains("TestForcedFailure"),
        "expected TestForcedFailure, got: {err}"
    );

    // Post-rejection state: ALL pre-failure mutations rolled back.
    let q_post = h.seq.q_snapshot().unwrap();
    assert_eq!(
        q_post.state_root_t, state_root_pre,
        "atomic rollback: state_root UNCHANGED post-failure (q_next dropped)"
    );

    let bob_balance_post = q_post
        .economic_state_t
        .balances_t
        .0
        .get(&AgentId("bob".into()))
        .copied()
        .unwrap();
    let collateral_post = q_post
        .economic_state_t
        .conditional_collateral_t
        .0
        .get(&EventId(TaskId("evt-8".into())))
        .copied()
        .unwrap_or(MicroCoin::zero());
    let pool_post = q_post
        .economic_state_t
        .cpmm_pools_t
        .0
        .get(&EventId(TaskId("evt-8".into())))
        .cloned()
        .expect("pool post-injection");

    assert_eq!(
        bob_balance_post.micro_units(),
        bob_balance_pre.micro_units(),
        "atomic rollback: buyer Coin balance UNCHANGED"
    );
    assert_eq!(
        collateral_post.micro_units(),
        collateral_pre.micro_units(),
        "atomic rollback: collateral UNCHANGED"
    );
    assert_eq!(
        pool_post.pool_yes.units, pool_pre.pool_yes.units,
        "atomic rollback: pool_yes UNCHANGED"
    );
    assert_eq!(
        pool_post.pool_no.units, pool_pre.pool_no.units,
        "atomic rollback: pool_no UNCHANGED"
    );

    // Buyer's conditional_share_balances_t for evt-8 should NOT have a YES
    // credit from the rolled-back step 4 either.
    let bob_pair = q_post
        .economic_state_t
        .conditional_share_balances_t
        .0
        .get(&AgentId("bob".into()))
        .and_then(|m| m.get(&EventId(TaskId("evt-8".into()))))
        .copied()
        .unwrap_or_default();
    assert_eq!(
        bob_pair.yes.units, 0,
        "atomic rollback: buyer YES gain from step 4 reverted"
    );

    // Sanity: a follow-up router tx (without injection) succeeds and
    // advances state correctly — proves the harness wasn't poisoned.
    let parent_after_failed = q_post.state_root_t;
    submit_and_apply(
        &mut h,
        build_router(
            parent_after_failed,
            "bob",
            "evt-8",
            BuyDirection::BuyYes,
            1_000_000,
            0,
            2,
        ),
    )
    .await
    .expect("follow-up router (no injection) accepts cleanly");

    let q_final = h.seq.q_snapshot().unwrap();
    assert_ne!(
        q_final.state_root_t, state_root_pre,
        "follow-up router advanced state_root"
    );
    assert_complete_set_balanced(&q_final.economic_state_t)
        .expect("complete-set balanced post follow-up");
}

// ── Atomic-rollback witnesses across all 9 steps (defense-in-depth) ─────────

/// Companion to test 9 — exhaustively exercises injection at each step
/// 1..=9 and asserts state_root unchanged. Provides the same atomic-
/// rollback witness across the full composite, defending against the
/// possibility that step 5 happens to be the only "easy" failure point.
#[tokio::test]
async fn router_atomic_rollback_witnessed_at_every_step() {
    let _guard = ENV_LOCK.lock().expect("env lock");

    for fail_at_step in 1u8..=9 {
        let q0 = genesis_with_balances_and_open_task(
            &[("alice", 50), ("bob", 50)],
            &format!("evt-9-{fail_at_step}"),
        );
        let mut h = fresh_harness(q0);

        let task = format!("evt-9-{fail_at_step}");
        let parent = seed_pool(&mut h, "alice", &task, 5_000_000).await;

        let q_pre = h.seq.q_snapshot().unwrap();
        let state_root_pre = q_pre.state_root_t;

        std::env::set_var(
            "TURINGOS_TEST_ROUTER_FAIL_AT_STEP",
            fail_at_step.to_string(),
        );
        let result = submit_and_apply(
            &mut h,
            build_router(
                parent,
                "bob",
                &task,
                BuyDirection::BuyYes,
                1_000_000,
                0,
                1,
            ),
        )
        .await;
        std::env::remove_var("TURINGOS_TEST_ROUTER_FAIL_AT_STEP");

        assert!(
            result.is_err(),
            "step {fail_at_step}: injection must trigger failure"
        );
        assert!(
            result.unwrap_err().contains("TestForcedFailure"),
            "step {fail_at_step}: expected TestForcedFailure"
        );

        let q_post = h.seq.q_snapshot().unwrap();
        assert_eq!(
            q_post.state_root_t, state_root_pre,
            "step {fail_at_step}: atomic rollback witnessed (state_root unchanged)"
        );
    }
}
