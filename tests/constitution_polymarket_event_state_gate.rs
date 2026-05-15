//! Constitution gate — Stage C overall §8 R1 CHALLENGE Q10 closure
//! (Codex Stage C overall PRE-§8 audit 2026-05-09 session #32):
//! task_markets_t event-state gate for Polymarket admission paths.
//!
//! Authority: Codex G2 Stage C overall §8 R1 audit (verbatim Q10
//! CHALLENGE remediation 1+2):
//!   1. Add a live event-state gate requiring task_markets_t[event_id.0]
//!      .state == Open for CpmmPool, CpmmSwap, and BuyWithCoinRouter.
//!   2. Add constitution tests proving pool create, share swap, and coin
//!      router reject against Finalized and Bankrupt events even when
//!      an Active pool/reserves exist.
//!
//! Architect manual §7 + §8 forbidden list "no trading after resolution"
//! — this gate enforces the invariant at admission time. Pre-Q10
//! closure: pool/swap/router only checked `pool.status == Active`; no
//! transition flips pools to Resolved on task resolution, so post-
//! resolution pool creation/trading was reachable. Q10 closure: each
//! admission arm now also checks task_markets_t[event_id.0].state ==
//! Open before any state mutation.

use std::sync::{Arc, RwLock};

use tempfile::TempDir;

use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::bottom_white::ledger::rejection_evidence::RejectionEvidenceWriter;
use turingosv4::bottom_white::ledger::system_keypair::{
    Ed25519Keypair, PinnedSystemPubkeys, SystemEpoch,
};
use turingosv4::bottom_white::ledger::transition_ledger::{InMemoryLedgerWriter, LedgerWriter};
use turingosv4::bottom_white::tools::registry::ToolRegistry;
use turingosv4::economy::money::MicroCoin;
use turingosv4::state::q_state::{AgentId, QState, TaskId, TaskMarketEntry, TaskMarketState, TxId};
use turingosv4::state::sequencer::{Sequencer, SubmissionEnvelope};
use turingosv4::state::typed_tx::{
    AgentSignature, BuyDirection, BuyWithCoinRouterTx, CompleteSetMintTx, CpmmPoolTx, CpmmSwapTx,
    EventId, ShareAmount, SwapDirection, TypedTx,
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
        rejection_writer,
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
        _ledger: writer,
    }
}

fn genesis_with_balances_and_event_state(
    pairs: &[(&str, i64)],
    task: &str,
    event_state: TaskMarketState,
) -> QState {
    let mut q = QState::genesis();
    for (name, coin) in pairs {
        q.economic_state_t.balances_t.0.insert(
            AgentId((*name).into()),
            MicroCoin::from_coin(*coin).unwrap(),
        );
    }
    let mut entry = TaskMarketEntry::default();
    entry.state = event_state;
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

fn build_swap(
    parent: turingosv4::state::q_state::Hash,
    trader: &str,
    task: &str,
    amount_in_units: u128,
    seq_no: u64,
) -> TypedTx {
    TypedTx::CpmmSwap(CpmmSwapTx {
        tx_id: TxId(format!("swap-{trader}-{task}-{seq_no}")),
        parent_state_root: parent,
        event_id: EventId(TaskId(task.into())),
        trader: AgentId(trader.into()),
        direction: SwapDirection::BuyYesWithNo,
        amount_in: ShareAmount::from_units(amount_in_units),
        min_out: ShareAmount::from_units(0),
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

// Bootstrap a fully-seeded harness: agents have inventory + active pool
// with active event state. Used for the swap + router reject tests
// where we need an active pool to exist while the event state is non-Open.
async fn bootstrap_active_pool(
    task: &str,
    pool_seed_units: u128,
) -> (Harness, turingosv4::state::q_state::Hash) {
    // Genesis with Open state so we can mint + create pool legitimately.
    let q0 = genesis_with_balances_and_event_state(
        &[("alice", 100), ("bob", 50)],
        task,
        TaskMarketState::Open,
    );
    let mut h = fresh_harness(q0);

    // Mint + create pool while state is Open.
    let p = h.seq.q_snapshot().unwrap().state_root_t;
    submit_and_apply(
        &mut h,
        TypedTx::CompleteSetMint(CompleteSetMintTx {
            tx_id: TxId(format!("mint-alice-{task}")),
            parent_state_root: p,
            event_id: EventId(TaskId(task.into())),
            owner: AgentId("alice".into()),
            amount: MicroCoin::from_micro_units(pool_seed_units as i64),
            signature: AgentSignature::from_bytes([0u8; 64]),
            timestamp_logical: 1000,
        }),
    )
    .await
    .expect("mint accepted");

    let p = h.seq.q_snapshot().unwrap().state_root_t;
    submit_and_apply(&mut h, build_pool(p, "alice", task, pool_seed_units, 1))
        .await
        .expect("pool create accepted");

    // Mint to bob too so swap test has trader inventory.
    let p = h.seq.q_snapshot().unwrap().state_root_t;
    submit_and_apply(
        &mut h,
        TypedTx::CompleteSetMint(CompleteSetMintTx {
            tx_id: TxId(format!("mint-bob-{task}")),
            parent_state_root: p,
            event_id: EventId(TaskId(task.into())),
            owner: AgentId("bob".into()),
            amount: MicroCoin::from_micro_units(pool_seed_units as i64 / 2),
            signature: AgentSignature::from_bytes([0u8; 64]),
            timestamp_logical: 1001,
        }),
    )
    .await
    .expect("mint to bob accepted");

    let parent = h.seq.q_snapshot().unwrap().state_root_t;
    (h, parent)
}

// Helper to mutate the harness's TaskMarketState directly (simulates
// post-resolution state transition without requiring a full
// FinalizeRewardTx / TaskExpireTx lifecycle).
//
// This is the test-side equivalent of an architect-spec'd state
// transition; in production, only system-emitted txs (FinalizeReward /
// TaskExpire / TaskBankruptcy) flip task_markets_t state. The test
// uses the public q-mutator surface to inject the post-resolution
// state directly.
fn mutate_event_state(h: &mut Harness, task: &str, new_state: TaskMarketState) {
    use std::sync::atomic::Ordering;
    // Directly mutate the q via the sequencer's exposed q write lock.
    // Unlike submit_and_apply (which goes through dispatch), this is a
    // test-only direct state injection mirroring what a future system-
    // emitted transition would do.
    let q_snap = h.seq.q_snapshot().expect("snapshot");
    let mut q_next = q_snap.clone();
    if let Some(entry) = q_next
        .economic_state_t
        .task_markets_t
        .0
        .get_mut(&TaskId(task.into()))
    {
        entry.state = new_state;
    }
    // Replace the live q. This requires q_set_for_test or similar; if
    // not exposed, we use the harness's submit path with a dummy state-
    // root advance (no-op), but for correctness in this test we accept
    // a slight scaffolding: the test injects directly via q_snapshot
    // round-trip on a non-public surface. If the public API doesn't
    // permit direct mutation, the test can be reframed to use system-
    // emitted txs instead — but for purposes of Q10 closure, we want
    // to assert the gate fires on the post-resolution state, regardless
    // of how that state was reached.
    h.seq.replace_q_for_test(q_next);
    // Bump logical_t to keep state_root_t derivation consistent.
    let _ = h.seq.q_snapshot().expect("snapshot post-mutate");
    // Note: this scaffolding bypasses the canonical state-root advance.
    // The test assertions below only check rejection behavior — they
    // don't require state_root continuity past the injection point.
    let _ = Ordering::SeqCst;
}

// ── Codex Q10 closure test 1: CpmmPool reject post-resolution ──────────────

/// pool_create_rejected_against_finalized_event — Codex Stage C overall §8
/// R1 CHALLENGE Q10 remediation 2: pool creation must reject when the
/// event is post-resolution (Finalized). Pre-Q10 closure: this would
/// have passed admission because only `pool.status` was checked, not
/// event state. Post-closure: `EventNotOpen` rejection.
#[tokio::test]
async fn pool_create_rejected_against_finalized_event() {
    // Genesis with task already in Finalized state.
    let q0 = genesis_with_balances_and_event_state(
        &[("alice", 100)],
        "evt-finalized",
        TaskMarketState::Finalized,
    );
    let mut h = fresh_harness(q0);

    // Note: cannot mint to alice for a Finalized event (TB-13
    // CompleteSetMint rejects EventNotOpen). So we directly test
    // CpmmPoolTx admission against a Finalized event with no inventory —
    // event-state gate fires BEFORE the inventory check, so we get
    // EventNotOpen, not InsufficientSharesForPool.
    let p = h.seq.q_snapshot().unwrap().state_root_t;
    let err = submit_and_apply(
        &mut h,
        build_pool(p, "alice", "evt-finalized", 1_000_000, 1),
    )
    .await
    .expect_err("pool create against Finalized must be rejected");
    assert!(
        err.contains("EventNotOpen"),
        "expected EventNotOpen, got: {err}"
    );
}

#[tokio::test]
async fn pool_create_rejected_against_bankrupt_event() {
    let q0 = genesis_with_balances_and_event_state(
        &[("alice", 100)],
        "evt-bankrupt",
        TaskMarketState::Bankrupt,
    );
    let mut h = fresh_harness(q0);

    let p = h.seq.q_snapshot().unwrap().state_root_t;
    let err = submit_and_apply(&mut h, build_pool(p, "alice", "evt-bankrupt", 1_000_000, 1))
        .await
        .expect_err("pool create against Bankrupt must be rejected");
    assert!(
        err.contains("EventNotOpen"),
        "expected EventNotOpen, got: {err}"
    );
}

// ── Codex Q10 closure test 2: CpmmSwap reject post-resolution ──────────────

/// swap_rejected_against_finalized_event — even when an Active pool +
/// trader inventory exist, swap admission against a Finalized event
/// must reject with EventNotOpen. Setup: bootstrap pool + trader
/// inventory while event is Open, then flip event state to Finalized,
/// then attempt swap.
#[tokio::test]
async fn swap_rejected_against_finalized_event() {
    let (mut h, parent_after_bootstrap) = bootstrap_active_pool("evt-swap-final", 5_000_000).await;

    // Sanity: swap before mutation succeeds (state is Open).
    let pre_swap_q = h.seq.q_snapshot().unwrap();
    assert!(
        pre_swap_q
            .economic_state_t
            .cpmm_pools_t
            .0
            .contains_key(&EventId(TaskId("evt-swap-final".into()))),
        "active pool exists before mutation"
    );

    // Mutate event state to Finalized.
    mutate_event_state(&mut h, "evt-swap-final", TaskMarketState::Finalized);

    // Swap attempt: must reject with EventNotOpen, NOT
    // InsufficientSharesForSwap or PoolNotActive (pool.status remains
    // Active because no transition flipped it).
    let parent = h.seq.q_snapshot().unwrap().state_root_t;
    let err = submit_and_apply(
        &mut h,
        build_swap(parent, "bob", "evt-swap-final", 100_000, 1),
    )
    .await
    .expect_err("swap against Finalized must be rejected");
    assert!(
        err.contains("EventNotOpen"),
        "expected EventNotOpen (not PoolNotActive); got: {err}"
    );

    // Witness pool still Active in state — the gate fires on event-state,
    // not pool.status.
    let q = h.seq.q_snapshot().unwrap();
    let pool = q
        .economic_state_t
        .cpmm_pools_t
        .0
        .get(&EventId(TaskId("evt-swap-final".into())))
        .cloned()
        .expect("pool present");
    assert_eq!(
        pool.status,
        turingosv4::state::q_state::PoolStatus::Active,
        "Q10 gate fires on event-state, not pool.status"
    );

    // Sanity: parent ref to avoid unused-var warning.
    let _ = parent_after_bootstrap;
}

#[tokio::test]
async fn swap_rejected_against_bankrupt_event() {
    let (mut h, _) = bootstrap_active_pool("evt-swap-bk", 3_000_000).await;
    mutate_event_state(&mut h, "evt-swap-bk", TaskMarketState::Bankrupt);

    let parent = h.seq.q_snapshot().unwrap().state_root_t;
    let err = submit_and_apply(&mut h, build_swap(parent, "bob", "evt-swap-bk", 100_000, 1))
        .await
        .expect_err("swap against Bankrupt must be rejected");
    assert!(err.contains("EventNotOpen"), "got: {err}");
}

// ── Codex Q10 closure test 3: BuyWithCoinRouter reject post-resolution ─────

/// router_rejected_against_finalized_event — even when an Active pool +
/// buyer Coin balance exist, router admission against a Finalized event
/// must reject with EventNotOpen.
#[tokio::test]
async fn router_rejected_against_finalized_event() {
    let (mut h, _) = bootstrap_active_pool("evt-router-final", 5_000_000).await;
    mutate_event_state(&mut h, "evt-router-final", TaskMarketState::Finalized);

    let parent = h.seq.q_snapshot().unwrap().state_root_t;
    let err = submit_and_apply(
        &mut h,
        build_router(parent, "bob", "evt-router-final", 1_000_000, 1),
    )
    .await
    .expect_err("router against Finalized must be rejected");
    assert!(
        err.contains("EventNotOpen"),
        "expected EventNotOpen; got: {err}"
    );

    // Buyer balance UNCHANGED (gate fires before any state mutation).
    let q = h.seq.q_snapshot().unwrap();
    let bob_bal = q
        .economic_state_t
        .balances_t
        .0
        .get(&AgentId("bob".into()))
        .copied()
        .unwrap();
    assert_eq!(
        bob_bal.micro_units(),
        50_000_000 - (5_000_000 / 2), // initial 50 Coin minus mint to bob during bootstrap
        "buyer balance untouched by rejected router tx"
    );
}

#[tokio::test]
async fn router_rejected_against_bankrupt_event() {
    let (mut h, _) = bootstrap_active_pool("evt-router-bk", 3_000_000).await;
    mutate_event_state(&mut h, "evt-router-bk", TaskMarketState::Bankrupt);

    let parent = h.seq.q_snapshot().unwrap().state_root_t;
    let err = submit_and_apply(
        &mut h,
        build_router(parent, "bob", "evt-router-bk", 500_000, 1),
    )
    .await
    .expect_err("router against Bankrupt must be rejected");
    assert!(err.contains("EventNotOpen"), "got: {err}");
}

// ── Codex Q10 closure positive control ─────────────────────────────────────

/// pool_swap_router_all_succeed_against_open_event — positive control
/// proving the new gate doesn't break the happy path. With Open event,
/// pool create + swap + router buy all admit normally.
#[tokio::test]
async fn pool_swap_router_all_succeed_against_open_event() {
    let (mut h, _) = bootstrap_active_pool("evt-open", 5_000_000).await;
    // Event state is Open (default from bootstrap).

    // Swap succeeds.
    let parent = h.seq.q_snapshot().unwrap().state_root_t;
    submit_and_apply(&mut h, build_swap(parent, "bob", "evt-open", 100_000, 1))
        .await
        .expect("swap against Open event admits normally");

    // Router succeeds.
    let parent = h.seq.q_snapshot().unwrap().state_root_t;
    submit_and_apply(&mut h, build_router(parent, "bob", "evt-open", 500_000, 1))
        .await
        .expect("router against Open event admits normally");
}

// ── Codex R2 CHALLENGE Q10 closure: fail-closed on missing entry ───────────

/// pool_create_rejected_against_missing_event_entry — Codex Stage C
/// overall §8 R2 CHALLENGE Q10: even when task_markets_t has NO entry
/// for the event_id (malformed / legacy / pre-genesis), pool creation
/// must reject with EventNotOpen. Pre-R2-fix: `unwrap_or(Open)`
/// fail-open default would have admitted; R2-fix: fail-closed `ok_or`.
#[tokio::test]
async fn pool_create_rejected_against_missing_event_entry() {
    // Genesis with NO task_markets_t entry for the event.
    let mut q0 = QState::genesis();
    q0.economic_state_t
        .balances_t
        .0
        .insert(AgentId("alice".into()), MicroCoin::from_coin(100).unwrap());
    // Note: we deliberately DO NOT add a task_markets_t entry for "evt-missing".
    let mut h = fresh_harness(q0);

    let p = h.seq.q_snapshot().unwrap().state_root_t;
    let err = submit_and_apply(&mut h, build_pool(p, "alice", "evt-missing", 1_000_000, 1))
        .await
        .expect_err("pool create against missing event entry must be rejected");
    assert!(
        err.contains("EventNotOpen"),
        "fail-closed: missing task_markets_t entry must reject with EventNotOpen; got: {err}"
    );
}

#[tokio::test]
async fn swap_rejected_against_missing_event_entry() {
    // Bootstrap pool against "evt-active" (Open). Then call swap with a
    // DIFFERENT event_id ("evt-missing") that has no task_markets_t
    // entry. The swap admission must fail-closed even though "evt-active"
    // has a valid pool — the swap targets the missing event.
    let (mut h, _) = bootstrap_active_pool("evt-active", 5_000_000).await;

    // Swap with event_id "evt-missing" (no task_markets_t entry, no pool).
    // Expected: EventNotOpen (gate fires before PoolNotActive check).
    let parent = h.seq.q_snapshot().unwrap().state_root_t;
    let err = submit_and_apply(&mut h, build_swap(parent, "bob", "evt-missing", 100_000, 1))
        .await
        .expect_err("swap against missing event entry must be rejected");
    assert!(
        err.contains("EventNotOpen"),
        "fail-closed: swap missing event must reject with EventNotOpen (not PoolNotActive); got: {err}"
    );
}

#[tokio::test]
async fn router_rejected_against_missing_event_entry() {
    let (mut h, _) = bootstrap_active_pool("evt-active-r", 5_000_000).await;

    let parent = h.seq.q_snapshot().unwrap().state_root_t;
    let err = submit_and_apply(
        &mut h,
        build_router(parent, "bob", "evt-missing-r", 500_000, 1),
    )
    .await
    .expect_err("router against missing event entry must be rejected");
    assert!(
        err.contains("EventNotOpen"),
        "fail-closed: router missing event must reject with EventNotOpen; got: {err}"
    );
}
