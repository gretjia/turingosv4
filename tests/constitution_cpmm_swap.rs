//! Constitution gate — Stage C P-M5 / Phase F.4 CpmmSwap pure-share-swap
//! semantics per architect manual §7.6.
//!
//! Authority: `handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_en.md`
//! §7.6 (verbatim Buy YES with NO / Buy NO with YES formulas + 6 mandatory
//! tests).
//! Companion: `handover/directives/2026-05-09_STAGE_C_POLYMARKET_VETO_REMEDIATION_DIRECTIVE.md`
//! §1.C row 4 (Class-3 re-apply post Stage C VETO; "n/a (was correct)").
//!
//! Test names mirror architect §7.6 verbatim list:
//!   1. swap_no_for_yes_constant_product_non_decreasing
//!   2. swap_yes_for_no_constant_product_non_decreasing
//!   3. swap_fails_zero_input
//!   4. swap_fails_insufficient_pool_output
//!   5. swap_respects_min_out_slippage
//!   6. swap_uses_integer_math_no_f64
//!
//! Each behavioral test exercises the actual sequencer accept arm via
//! `submit_and_apply` against a real `Sequencer` + `InMemoryLedgerWriter`
//! after seeding via `CompleteSetMintTx` + `CpmmPoolTx` through the live
//! ingress path. Per `feedback_tape_first_real_tests`: no tape activity =
//! not a TuringOS test; these tests advance state_root_t through real
//! transitions. Test 6 (`swap_uses_integer_math_no_f64`) is a source-grep
//! gate — architect §7.6 rule "integer rounding only".

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
use turingosv4::state::q_state::{
    AgentId, QState, TaskId, TaskMarketEntry, TaskMarketState, TxId,
};
use turingosv4::state::sequencer::{Sequencer, SubmissionEnvelope};
use turingosv4::state::typed_tx::{
    AgentSignature, CompleteSetMintTx, CpmmPoolTx, CpmmSwapTx, EventId,
    ShareAmount, SwapDirection, TypedTx,
};
use turingosv4::top_white::predicates::registry::PredicateRegistry;

// ── Harness (mirrors constitution_cpmm_pool.rs pattern) ─────────────────────

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

fn build_swap(
    parent: turingosv4::state::q_state::Hash,
    trader: &str,
    task: &str,
    direction: SwapDirection,
    amount_in_units: u128,
    min_out_units: u128,
    seq_no: u64,
) -> TypedTx {
    TypedTx::CpmmSwap(CpmmSwapTx {
        tx_id: TxId(format!("swap-{trader}-{task}-{seq_no}")),
        parent_state_root: parent,
        event_id: EventId(TaskId(task.into())),
        trader: AgentId(trader.into()),
        direction,
        amount_in: ShareAmount::from_units(amount_in_units),
        min_out: ShareAmount::from_units(min_out_units),
        signature: AgentSignature::from_bytes([0u8; 64]),
    })
}

// Seed pattern: provider mints + creates symmetric pool; trader mints (as a
// distinct holder) so they have YES + NO inventory to swap. Returns the
// post-seed state_root for the next swap to use as parent_state_root.
async fn seed_pool_and_trader(
    h: &mut Harness,
    provider: &str,
    trader: &str,
    task: &str,
    pool_seed_units: u128,
    trader_mint_units: u128,
) -> turingosv4::state::q_state::Hash {
    // Provider mints + seeds pool.
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

    // Trader mints inventory so they hold YES + NO.
    let p = h.seq.q_snapshot().unwrap().state_root_t;
    submit_and_apply(
        h,
        build_mint(p, trader, task, trader_mint_units as i64, 2),
    )
    .await
    .expect("trader mint accepted");

    h.seq.q_snapshot().unwrap().state_root_t
}

// ── Architect §7.6 verbatim test 1 ──────────────────────────────────────────

/// swap_no_for_yes_constant_product_non_decreasing — architect §7.6 verbatim
/// "Buy YES with NO" formula:
///
/// ```
/// outY = floor(dN * poolY / (poolN + dN))
/// poolN1 = poolN + dN
/// poolY1 = poolY - outY
/// ```
///
/// With integer rounding: `poolY1 * poolN1 >= poolY * poolN` (architect
/// explicit `>=` because floor leaves dust in pool). Trader submits dN = 1M
/// NO against a 5M/5M pool; receives outY = floor(1M * 5M / 6M) =
/// floor(5_000_000 * 1_000_000 / 6_000_000) = 833333 YES (i.e., not 833334
/// because 5_000_000_000_000 / 6_000_000 = 833_333.333…). Pool post-state:
/// poolY1 = 5_000_000 - 833_333 = 4_166_667, poolN1 = 6_000_000. k_post =
/// 4_166_667 * 6_000_000 = 25_000_002_000_000 >= 25_000_000_000_000 = k_pre.
/// `assert_complete_set_balanced` (P-M4 extended to count pool reserves)
/// MUST hold post-swap (sum YES + sum NO preserved across rotation).
#[tokio::test]
async fn swap_no_for_yes_constant_product_non_decreasing() {
    let q0 = genesis_with_balances_and_open_task(
        &[("alice", 50), ("bob", 50)],
        "evt-1",
    );
    let mut h = fresh_harness(q0);

    // Alice provides 5M/5M pool; Bob has 5M YES + 5M NO inventory to trade.
    let pre_swap_root = seed_pool_and_trader(
        &mut h, "alice", "bob", "evt-1", 5_000_000, 5_000_000,
    )
    .await;

    // Capture pre-swap state for invariant + k_pre comparison.
    let q_pre = h.seq.q_snapshot().unwrap();
    let pool_pre = q_pre
        .economic_state_t
        .cpmm_pools_t
        .0
        .get(&EventId(TaskId("evt-1".into())))
        .cloned()
        .expect("pool present pre-swap");
    let k_pre = pool_pre.pool_yes.units * pool_pre.pool_no.units;
    let bob_pair_pre = q_pre
        .economic_state_t
        .conditional_share_balances_t
        .0
        .get(&AgentId("bob".into()))
        .and_then(|m| m.get(&EventId(TaskId("evt-1".into()))))
        .cloned()
        .expect("bob has shares pre-swap");
    assert_eq!(bob_pair_pre.yes.units, 5_000_000);
    assert_eq!(bob_pair_pre.no.units, 5_000_000);

    // Bob: Buy YES with NO, dN = 1M, min_out = 0 (no slippage cap).
    submit_and_apply(
        &mut h,
        build_swap(
            pre_swap_root,
            "bob",
            "evt-1",
            SwapDirection::BuyYesWithNo,
            1_000_000,
            0,
            1,
        ),
    )
    .await
    .expect("swap accepted");

    let q_post = h.seq.q_snapshot().unwrap();
    let pool_post = q_post
        .economic_state_t
        .cpmm_pools_t
        .0
        .get(&EventId(TaskId("evt-1".into())))
        .cloned()
        .expect("pool present post-swap");

    // outY = floor(1_000_000 * 5_000_000 / 6_000_000) = 833_333.
    let expected_out_y: u128 = 833_333;
    let expected_pool_n: u128 = 6_000_000;
    let expected_pool_y: u128 = 5_000_000 - expected_out_y;
    assert_eq!(pool_post.pool_no.units, expected_pool_n);
    assert_eq!(pool_post.pool_yes.units, expected_pool_y);

    // Architect §7.6 verbatim: k1 >= k0.
    let k_post = pool_post.pool_yes.units * pool_post.pool_no.units;
    assert!(
        k_post >= k_pre,
        "architect §7.6 constant-product: pool_yes1 * pool_no1 >= pool_yes * pool_no \
         (k_post={k_post}, k_pre={k_pre})"
    );

    // Bob's balances: -1M NO, +833_333 YES.
    let bob_pair_post = q_post
        .economic_state_t
        .conditional_share_balances_t
        .0
        .get(&AgentId("bob".into()))
        .and_then(|m| m.get(&EventId(TaskId("evt-1".into()))))
        .cloned()
        .expect("bob shares post-swap");
    assert_eq!(bob_pair_post.no.units, 5_000_000 - 1_000_000);
    assert_eq!(bob_pair_post.yes.units, 5_000_000 + expected_out_y);

    // Conservation: total Coin unchanged; complete-set balanced (pool +
    // traders share counts match collateral on both sides post-rotation).
    assert_total_ctf_conserved(
        &q_pre.economic_state_t,
        &q_post.economic_state_t,
        &[],
    )
    .expect("total Coin conserved across CpmmSwap (pure share rotation)");
    assert_complete_set_balanced(&q_post.economic_state_t)
        .expect("complete-set balanced post-swap");
}

// ── Architect §7.6 verbatim test 2 ──────────────────────────────────────────

/// swap_yes_for_no_constant_product_non_decreasing — architect §7.6 verbatim
/// "Symmetric Buy NO with YES" formula:
///
/// ```
/// outN = floor(dY * poolN / (poolY + dY))
/// poolY1 = poolY + dY
/// poolN1 = poolN - outN
/// ```
///
/// Mirror of test 1. Constant-product invariant `>=` holds for the
/// symmetric direction. Trader submits dY = 2M YES against 4M/4M pool;
/// outN = floor(2M * 4M / 6M) = floor(8_000_000_000_000 / 6_000_000) =
/// 1_333_333 NO.
#[tokio::test]
async fn swap_yes_for_no_constant_product_non_decreasing() {
    let q0 = genesis_with_balances_and_open_task(
        &[("alice", 50), ("bob", 50)],
        "evt-2",
    );
    let mut h = fresh_harness(q0);

    let pre_swap_root = seed_pool_and_trader(
        &mut h, "alice", "bob", "evt-2", 4_000_000, 4_000_000,
    )
    .await;

    let q_pre = h.seq.q_snapshot().unwrap();
    let pool_pre = q_pre
        .economic_state_t
        .cpmm_pools_t
        .0
        .get(&EventId(TaskId("evt-2".into())))
        .cloned()
        .expect("pool present pre-swap");
    let k_pre = pool_pre.pool_yes.units * pool_pre.pool_no.units;

    // Bob: Buy NO with YES, dY = 2M.
    submit_and_apply(
        &mut h,
        build_swap(
            pre_swap_root,
            "bob",
            "evt-2",
            SwapDirection::BuyNoWithYes,
            2_000_000,
            0,
            1,
        ),
    )
    .await
    .expect("swap accepted");

    let q_post = h.seq.q_snapshot().unwrap();
    let pool_post = q_post
        .economic_state_t
        .cpmm_pools_t
        .0
        .get(&EventId(TaskId("evt-2".into())))
        .cloned()
        .expect("pool present post-swap");

    // outN = floor(2_000_000 * 4_000_000 / 6_000_000) = 1_333_333.
    let expected_out_n: u128 = 1_333_333;
    let expected_pool_y: u128 = 6_000_000;
    let expected_pool_n: u128 = 4_000_000 - expected_out_n;
    assert_eq!(pool_post.pool_yes.units, expected_pool_y);
    assert_eq!(pool_post.pool_no.units, expected_pool_n);

    let k_post = pool_post.pool_yes.units * pool_post.pool_no.units;
    assert!(
        k_post >= k_pre,
        "architect §7.6 constant-product: pool_yes1 * pool_no1 >= pool_yes * pool_no \
         (k_post={k_post}, k_pre={k_pre})"
    );

    // Bob: -2M YES, +1_333_333 NO.
    let bob_pair_post = q_post
        .economic_state_t
        .conditional_share_balances_t
        .0
        .get(&AgentId("bob".into()))
        .and_then(|m| m.get(&EventId(TaskId("evt-2".into()))))
        .cloned()
        .expect("bob shares post-swap");
    assert_eq!(bob_pair_post.yes.units, 4_000_000 - 2_000_000);
    assert_eq!(bob_pair_post.no.units, 4_000_000 + expected_out_n);

    assert_total_ctf_conserved(
        &q_pre.economic_state_t,
        &q_post.economic_state_t,
        &[],
    )
    .expect("total Coin conserved across CpmmSwap (pure share rotation)");
    assert_complete_set_balanced(&q_post.economic_state_t)
        .expect("complete-set balanced post-swap");
}

// ── Architect §7.6 verbatim test 3 ──────────────────────────────────────────

/// swap_fails_zero_input — architect §7.6 verbatim "input: dN > 0" /
/// "input: dY > 0". Zero-input swap is degenerate; admission rejects with
/// `SwapZeroInput`. State unchanged post-rejection (no pool drift, no
/// trader balance shift).
#[tokio::test]
async fn swap_fails_zero_input() {
    let q0 = genesis_with_balances_and_open_task(
        &[("alice", 50), ("bob", 50)],
        "evt-3",
    );
    let mut h = fresh_harness(q0);

    let pre_swap_root = seed_pool_and_trader(
        &mut h, "alice", "bob", "evt-3", 3_000_000, 2_000_000,
    )
    .await;

    let q_pre = h.seq.q_snapshot().unwrap();
    let pool_pre = q_pre
        .economic_state_t
        .cpmm_pools_t
        .0
        .get(&EventId(TaskId("evt-3".into())))
        .cloned()
        .expect("pool present pre-swap");

    // Bob: zero-input swap.
    let err = submit_and_apply(
        &mut h,
        build_swap(
            pre_swap_root,
            "bob",
            "evt-3",
            SwapDirection::BuyYesWithNo,
            0,
            0,
            1,
        ),
    )
    .await
    .expect_err("zero-input swap must be rejected");
    assert!(
        err.contains("SwapZeroInput"),
        "expected SwapZeroInput, got: {err}"
    );

    // State UNCHANGED post-rejection.
    let q_post = h.seq.q_snapshot().unwrap();
    let pool_post = q_post
        .economic_state_t
        .cpmm_pools_t
        .0
        .get(&EventId(TaskId("evt-3".into())))
        .cloned()
        .expect("pool still present");
    assert_eq!(pool_post.pool_yes.units, pool_pre.pool_yes.units);
    assert_eq!(pool_post.pool_no.units, pool_pre.pool_no.units);

    assert_total_ctf_conserved(
        &q_pre.economic_state_t,
        &q_post.economic_state_t,
        &[],
    )
    .expect("rejected tx leaves total_supply_micro untouched");
}

// ── Architect §7.6 verbatim test 4 ──────────────────────────────────────────

/// swap_fails_insufficient_pool_output — `floor(amount_in * pool_other /
/// (pool_input + amount_in))` returns 0 when amount_in is too small
/// relative to the pool ratio. Architect §7.6 floor formula returns zero
/// output for dust-sized inputs against an asymmetric ratio. Setup: pool
/// has poolN = 1_000_000 NO + poolY = 1 YES (extreme asymmetry); trader
/// submits dN = 1 NO. outY = floor(1 * 1 / 1_000_001) = 0 → reject with
/// `SwapInsufficientPoolOutput`. Note: this asymmetric pool is reached by
/// first creating a symmetric 1M/1M pool and then having alice swap
/// nearly all the NO away — but pool creation requires symmetric seed,
/// and pool reserves only become asymmetric via swaps. So we set up the
/// asymmetry by having bob (a different trader) swap 999_999 YES → NO
/// first, leaving a near-zero YES side, then alice tries dust.
#[tokio::test]
async fn swap_fails_insufficient_pool_output() {
    let q0 = genesis_with_balances_and_open_task(
        &[("provider", 50), ("bob", 50), ("alice", 50)],
        "evt-4",
    );
    let mut h = fresh_harness(q0);

    // Provider seeds 1M/1M pool. Bob trades to drain YES side.
    let pre_swap_root = seed_pool_and_trader(
        &mut h, "provider", "bob", "evt-4", 1_000_000, 1_500_000,
    )
    .await;

    // Bob: Buy YES with NO, large dN = 1_500_000 to drain YES side near
    // zero. outY = floor(1_500_000 * 1_000_000 / 2_500_000) = 600_000.
    // Pool post: poolY = 400_000, poolN = 2_500_000.
    submit_and_apply(
        &mut h,
        build_swap(
            pre_swap_root,
            "bob",
            "evt-4",
            SwapDirection::BuyYesWithNo,
            1_500_000,
            0,
            1,
        ),
    )
    .await
    .expect("first swap accepted");

    let q_after_bob = h.seq.q_snapshot().unwrap();
    let pool_after_bob = q_after_bob
        .economic_state_t
        .cpmm_pools_t
        .0
        .get(&EventId(TaskId("evt-4".into())))
        .cloned()
        .expect("pool present");
    assert_eq!(pool_after_bob.pool_yes.units, 400_000);
    assert_eq!(pool_after_bob.pool_no.units, 2_500_000);

    // Alice mints to gain inventory and tries a dust swap that floors to 0.
    // To make floor return 0 in `floor(dY * poolN / (poolY + dY))`, we need
    // dY * poolN < poolY + dY. With poolY = 400_000, poolN = 2_500_000, the
    // smallest valid dust is dY = 1: floor(1 * 2_500_000 / 400_001) =
    // 6 (still > 0). For BuyYesWithNo with poolY = 400_000, poolN = 2_500_000,
    // dN = 1: floor(1 * 400_000 / 2_500_001) = 0. ← THIS is the floor-to-0.
    let p = h.seq.q_snapshot().unwrap().state_root_t;
    submit_and_apply(&mut h, build_mint(p, "alice", "evt-4", 100, 1))
        .await
        .expect("alice mint accepted");

    let pre_alice_swap_root = h.seq.q_snapshot().unwrap().state_root_t;
    let err = submit_and_apply(
        &mut h,
        build_swap(
            pre_alice_swap_root,
            "alice",
            "evt-4",
            SwapDirection::BuyYesWithNo,
            1, // dN = 1; floor(1 * 400_000 / 2_500_001) = 0
            0,
            1,
        ),
    )
    .await
    .expect_err("dust swap must be rejected via SwapInsufficientPoolOutput");
    assert!(
        err.contains("SwapInsufficientPoolOutput"),
        "expected SwapInsufficientPoolOutput, got: {err}"
    );

    // Post-rejection state unchanged from post-bob-swap (alice's mint
    // landed but alice's swap rejected; pool reserves unchanged).
    let q_post = h.seq.q_snapshot().unwrap();
    let pool_post = q_post
        .economic_state_t
        .cpmm_pools_t
        .0
        .get(&EventId(TaskId("evt-4".into())))
        .cloned()
        .expect("pool present");
    assert_eq!(pool_post.pool_yes.units, 400_000);
    assert_eq!(pool_post.pool_no.units, 2_500_000);
}

// ── Architect §7.6 verbatim test 5 ──────────────────────────────────────────

/// swap_respects_min_out_slippage — trader sets `min_out` floor; admission
/// computes `out` per architect formula and rejects if `out < min_out`.
/// The non-zero output is the formula's actual answer; rejection class
/// `SwapSlippageExceeded` is distinct from `SwapInsufficientPoolOutput`
/// (which is `out == 0`) so trader can distinguish "pool too thin" from
/// "pool moved against me". Setup: 5M/5M pool; trader submits dN = 1M
/// with min_out = 1_000_000 (greedy). outY = 833_333 < 1_000_000 → reject.
/// Then re-submit with min_out = 833_333 (exact) → accept; min_out =
/// 833_334 (greedy by 1) → reject.
#[tokio::test]
async fn swap_respects_min_out_slippage() {
    let q0 = genesis_with_balances_and_open_task(
        &[("alice", 50), ("bob", 50)],
        "evt-5",
    );
    let mut h = fresh_harness(q0);

    let pre_swap_root = seed_pool_and_trader(
        &mut h, "alice", "bob", "evt-5", 5_000_000, 5_000_000,
    )
    .await;

    // Attempt 1: greedy min_out = 1_000_000; expected out = 833_333. Reject.
    let err = submit_and_apply(
        &mut h,
        build_swap(
            pre_swap_root,
            "bob",
            "evt-5",
            SwapDirection::BuyYesWithNo,
            1_000_000,
            1_000_000,
            1,
        ),
    )
    .await
    .expect_err("greedy min_out swap must be rejected");
    assert!(
        err.contains("SwapSlippageExceeded"),
        "expected SwapSlippageExceeded, got: {err}"
    );

    // Pool unchanged post-rejection.
    let q_after_reject = h.seq.q_snapshot().unwrap();
    let pool_after_reject = q_after_reject
        .economic_state_t
        .cpmm_pools_t
        .0
        .get(&EventId(TaskId("evt-5".into())))
        .cloned()
        .expect("pool present");
    assert_eq!(pool_after_reject.pool_yes.units, 5_000_000);
    assert_eq!(pool_after_reject.pool_no.units, 5_000_000);

    // Attempt 2: tight min_out = 833_334 (one above formula). Reject.
    let err = submit_and_apply(
        &mut h,
        build_swap(
            pre_swap_root,
            "bob",
            "evt-5",
            SwapDirection::BuyYesWithNo,
            1_000_000,
            833_334,
            2,
        ),
    )
    .await
    .expect_err("min_out one-above-formula must be rejected");
    assert!(
        err.contains("SwapSlippageExceeded"),
        "expected SwapSlippageExceeded, got: {err}"
    );

    // Attempt 3: exact min_out = 833_333. Accept.
    submit_and_apply(
        &mut h,
        build_swap(
            pre_swap_root,
            "bob",
            "evt-5",
            SwapDirection::BuyYesWithNo,
            1_000_000,
            833_333,
            3,
        ),
    )
    .await
    .expect("exact min_out swap accepted");

    let q_post = h.seq.q_snapshot().unwrap();
    let pool_post = q_post
        .economic_state_t
        .cpmm_pools_t
        .0
        .get(&EventId(TaskId("evt-5".into())))
        .cloned()
        .expect("pool present");
    assert_eq!(pool_post.pool_yes.units, 5_000_000 - 833_333);
    assert_eq!(pool_post.pool_no.units, 6_000_000);
}

// ── Architect §7.6 verbatim test 6 ──────────────────────────────────────────

/// swap_uses_integer_math_no_f64 — architect §7.6 verbatim "with integer
/// rounding". Source-grep gate: the CpmmSwap admission arm in
/// `src/state/sequencer.rs` and the `CpmmSwapTx` / signing payload
/// surfaces in `src/state/typed_tx.rs` MUST NOT use `f64`, `f32`, `as f64`,
/// `as f32`, or any floating-point cast. The constant-product invariant
/// (`>=` not `==`) holds only because integer floor preserves
/// pool-favorable rounding; floating-point would introduce non-deterministic
/// rounding that breaks replay determinism + invites monetary defects.
#[test]
fn swap_uses_integer_math_no_f64() {
    use std::path::PathBuf;
    let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let sequencer_src = workspace.join("src/state/sequencer.rs");
    let typed_tx_src = workspace.join("src/state/typed_tx.rs");

    let seq_text = std::fs::read_to_string(&sequencer_src)
        .expect("read src/state/sequencer.rs");
    let tx_text = std::fs::read_to_string(&typed_tx_src)
        .expect("read src/state/typed_tx.rs");

    // Bound the search to the CpmmSwap admission arm in sequencer.rs.
    let arm_start = seq_text
        .find("TypedTx::CpmmSwap(swap) => {")
        .expect("CpmmSwap admission arm present in sequencer.rs");
    // Search until the next top-level `}` matching the arm — heuristic:
    // until the next `Ok((q_next, SignalBundle::default()))` followed by
    // closing `}`. Use a generous slice (8 KiB) covering the arm body.
    let arm_end = (arm_start + 8192).min(seq_text.len());
    let arm_body = &seq_text[arm_start..arm_end];

    // Source-grep for REAL floating-point usage in the arm. We look for type
    // declarations / casts / literals — not the substring "f64" alone, since
    // the architect-mandated test name `swap_uses_integer_math_no_f64`
    // literally contains the substring inside doc-comments. Forbidden tokens:
    //   `: f64` / `: f32`            — type annotations
    //   `as f64` / `as f32`          — casts
    //   `0f64` / `0f32` / `1f64` / `1f32` etc. — float literals
    //   `f64::` / `f32::`            — associated function calls
    let forbidden_real_usage =
        [": f64", ": f32", "as f64", "as f32", "f64::", "f32::"];
    let forbidden_literal_suffix = ["f64", "f32"]; // checked context-sensitively
    for needle in &forbidden_real_usage {
        assert!(
            !arm_body.contains(needle),
            "architect §7.6 integer-math: CpmmSwap admission arm contains forbidden token {needle:?}"
        );
    }
    // Float-literal check: any digit followed by f64/f32 (e.g., 0f64, 1.5f64).
    // We scan for the suffix and verify the preceding char is a digit OR `.`.
    for suffix in &forbidden_literal_suffix {
        let mut search_from = 0usize;
        while let Some(idx_rel) = arm_body[search_from..].find(suffix) {
            let idx = search_from + idx_rel;
            let preceding = arm_body[..idx].chars().last();
            if matches!(preceding, Some(c) if c.is_ascii_digit() || c == '.') {
                panic!(
                    "architect §7.6 integer-math: CpmmSwap admission arm contains float literal ending in {suffix:?} at byte offset {idx}"
                );
            }
            search_from = idx + suffix.len();
        }
    }

    // Bound search for CpmmSwapTx struct + signing payload in typed_tx.rs.
    let tx_struct_start = tx_text
        .find("pub struct CpmmSwapTx {")
        .expect("CpmmSwapTx struct present in typed_tx.rs");
    let tx_struct_end =
        tx_text[tx_struct_start..].find("}\n").map(|i| tx_struct_start + i)
            .unwrap_or(tx_text.len());
    let tx_struct_body = &tx_text[tx_struct_start..tx_struct_end];
    for needle in &forbidden_real_usage {
        assert!(
            !tx_struct_body.contains(needle),
            "architect §7.6 integer-math: CpmmSwapTx struct contains forbidden token {needle:?}"
        );
    }

    let payload_start = tx_text
        .find("pub struct CpmmSwapSigningPayload {")
        .expect("CpmmSwapSigningPayload struct present in typed_tx.rs");
    let payload_end = tx_text[payload_start..]
        .find("}\n")
        .map(|i| payload_start + i)
        .unwrap_or(tx_text.len());
    let payload_body = &tx_text[payload_start..payload_end];
    for needle in &forbidden_real_usage {
        assert!(
            !payload_body.contains(needle),
            "architect §7.6 integer-math: CpmmSwapSigningPayload contains forbidden token {needle:?}"
        );
    }

    // Sanity: the integer formula's intermediate types are u128 (stack-
    // allocated; no heap; deterministic across architectures). Locate
    // `let denom = pool_input_units` and `let numer = swap.amount_in.units`
    // markers — these prove the implementation uses checked u128 math.
    assert!(
        seq_text.contains("let denom = pool_input_units"),
        "CpmmSwap arm must compute denom via u128 checked_add"
    );
    assert!(
        seq_text.contains("let numer = swap"),
        "CpmmSwap arm must compute numer via u128 checked_mul"
    );
    assert!(
        seq_text.contains("let out_units: u128 = numer / denom;"),
        "CpmmSwap arm must use u128 integer division for out"
    );
}
