//! TuringOS Constitution Gate — Stage C P-M2 CompleteSetMerge hardening
//! (architect 2026-05-07 ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL
//! §7.3 verbatim).
//!
//! # Scope
//!
//! Architect alignment doc §7.3 mandates 5 hardening tests for the new
//! `CompleteSetMergeTx` (1 YES + 1 NO → 1 Coin pre-resolution; inverse of
//! `CompleteSetMint`):
//!
//!   - merge_yes_no_returns_coin
//!   - merge_requires_both_sides
//!   - merge_conserves_total_coin
//!   - merge_reduces_collateral
//!   - merge_unavailable_after_final_redeem_if_shares_exhausted
//!
//! # Strict-constitution doctrine (per `feedback_no_workarounds_strict_constitution`)
//!
//! Tests bind to live sequencer dispatch on `CompleteSetMergeTx` — no synthetic
//! state mutation, no test-only paths. Mirrors the §5.3 CompleteSet hardening
//! pattern (`tests/constitution_completeset_hardening.rs`).
//!
//! TRACE_FLOWCHART_MATRIX:
//!   - FC1 §1 predicate routing (merge admission rejects amount==0 / over-balance)
//!   - FC1 §6 monetary invariant (assert_complete_set_balanced live in dispatch)
//!   - §7.3 architect Polymarket manual

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
    AgentId, QState, ShareSidePair, TaskId, TaskMarketEntry, TaskMarketState, TxId,
};
use turingosv4::state::sequencer::{Sequencer, SubmissionEnvelope};
use turingosv4::state::typed_tx::{
    AgentSignature, CompleteSetMergeTx, CompleteSetRedeemTx, EventId, OutcomeSide,
    ShareAmount, TypedTx,
};
use turingosv4::top_white::predicates::registry::PredicateRegistry;

// ── Harness (mirrors constitution_completeset_hardening.rs verbatim) ────────

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

fn genesis_with_balances(pairs: &[(&str, i64)]) -> QState {
    let mut q = QState::genesis();
    for (name, coin) in pairs {
        q.economic_state_t.balances_t.0.insert(
            AgentId((*name).into()),
            MicroCoin::from_coin(*coin).unwrap(),
        );
    }
    q
}

fn seed_task_market(q: &mut QState, task: &str, state: TaskMarketState) {
    let mut entry = TaskMarketEntry::default();
    entry.state = state;
    q.economic_state_t.task_markets_t.0.insert(TaskId(task.into()), entry);
}

/// Build a post-mint snapshot directly (mint dispatch requires task=Open;
/// merge tests need owner already holding YES + NO shares without paying
/// twice through Mint dispatch in the test harness).
fn genesis_post_mint(
    pairs: &[(&str, i64)],
    mint_owner: &str,
    task: &str,
    mint_amount_micro: i64,
    final_state: TaskMarketState,
) -> QState {
    let mut q = genesis_with_balances(pairs);
    seed_task_market(&mut q, task, final_state);

    let agent_id = AgentId(mint_owner.into());
    let event_id = EventId(TaskId(task.into()));

    let bal = q
        .economic_state_t
        .balances_t
        .0
        .get(&agent_id)
        .copied()
        .unwrap_or(MicroCoin::zero());
    q.economic_state_t.balances_t.0.insert(
        agent_id.clone(),
        MicroCoin::from_micro_units(bal.micro_units() - mint_amount_micro),
    );
    q.economic_state_t.conditional_collateral_t.0.insert(
        event_id.clone(),
        MicroCoin::from_micro_units(mint_amount_micro),
    );
    let mut owner_shares = std::collections::BTreeMap::new();
    owner_shares.insert(
        event_id,
        ShareSidePair {
            yes: ShareAmount::from_units(mint_amount_micro as u128),
            no: ShareAmount::from_units(mint_amount_micro as u128),
        },
    );
    q.economic_state_t.conditional_share_balances_t.0.insert(agent_id, owner_shares);
    q
}

async fn submit_and_apply(h: &mut Harness, tx: TypedTx) -> Result<(), String> {
    h.seq.submit_agent_tx(tx).await
        .map_err(|e| format!("submit error: {e:?}"))?;
    let outcome = h.seq.try_apply_one(&mut h.rx)
        .ok_or_else(|| "no envelope drained".to_string())?;
    outcome.map(|_| ()).map_err(|e| format!("apply error: {e:?}"))
}

fn build_merge(
    parent: turingosv4::state::q_state::Hash,
    owner: &str,
    task: &str,
    units: u128,
    seq_no: u64,
) -> TypedTx {
    TypedTx::CompleteSetMerge(CompleteSetMergeTx {
        tx_id: TxId(format!("merge-{owner}-{task}-{seq_no}")),
        parent_state_root: parent,
        event_id: EventId(TaskId(task.into())),
        owner: AgentId(owner.into()),
        amount: ShareAmount::from_units(units),
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: 3000 + seq_no,
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
        timestamp_logical: 4000 + seq_no,
    })
}

/// Single-source-of-truth conservation sum (mirrors hardening test pattern).
fn total_supply_micro(q: &QState) -> i64 {
    canonical_total_supply_micro(&q.economic_state_t)
        .expect("total_supply_micro must not overflow in test fixtures")
}

// ════════════════════════════════════════════════════════════════════════════
// §7.3 P-M2 CompleteSetMerge hardening (5 verbatim names)
// ════════════════════════════════════════════════════════════════════════════

/// §7.3 verbatim — `merge_yes_no_returns_coin`.
///
/// 1 YES + 1 NO + 1 collateral lock → 1 Coin (architect manual §7.3
/// "allow 1 YES + 1 NO -> 1 Coin"). Asserts post-merge owner balance
/// increases by `amount` and YES + NO share balances each decrease by
/// `amount`.
#[tokio::test]
async fn merge_yes_no_returns_coin() {
    // Owner starts with 100 Coin, mints 60 → leaves 40 Coin balance + 60
    // collateral + 60 YES + 60 NO. We then merge 25 → expect 65 Coin
    // balance + 35 collateral + 35 YES + 35 NO.
    let q0 = genesis_post_mint(&[("alice", 100)], "alice", "task-A", 60_000_000, TaskMarketState::Open);
    let mut h = fresh_harness(q0);
    let parent = h.seq.q_snapshot().unwrap().state_root_t;

    let tx = build_merge(parent, "alice", "task-A", 25_000_000, 1);
    submit_and_apply(&mut h, tx).await.expect("merge accepted");

    let q = h.seq.q_snapshot().unwrap();
    let alice = AgentId("alice".into());
    let event = EventId(TaskId("task-A".into()));

    let bal = q.economic_state_t.balances_t.0.get(&alice).copied().unwrap();
    assert_eq!(bal.micro_units(), 40_000_000 + 25_000_000, "owner Coin balance += merge amount");

    let collat = q
        .economic_state_t
        .conditional_collateral_t
        .0
        .get(&event)
        .copied()
        .unwrap();
    assert_eq!(collat.micro_units(), 60_000_000 - 25_000_000, "collateral -= merge amount");

    let pair = q
        .economic_state_t
        .conditional_share_balances_t
        .0
        .get(&alice)
        .and_then(|m| m.get(&event))
        .copied()
        .unwrap_or_default();
    assert_eq!(pair.yes.units, (60_000_000 - 25_000_000) as u128, "YES shares -= amount");
    assert_eq!(pair.no.units, (60_000_000 - 25_000_000) as u128, "NO shares -= amount");
}

/// §7.3 verbatim — `merge_requires_both_sides`.
///
/// If owner holds YES_amount but lacks NO_amount (or vice versa), merge
/// MUST be rejected with `MergeMoreThanOwned`. This blocks one-sided
/// "merge" gaming where an agent could claim a Coin payout while still
/// holding a winning side claim.
#[tokio::test]
async fn merge_requires_both_sides() {
    // Owner has 100 YES + 100 NO at task-B (post-mint of 100 micro).
    // Burn one side artificially via a redeem-after-resolution (Bankrupt
    // → NO wins), leaving 100 YES + 0 NO. Attempt merge → should fail
    // because NO < amount.
    let q0 = genesis_post_mint(
        &[("alice", 100)],
        "alice",
        "task-B",
        100, // 100 micro-units (so we can redeem cheaply)
        TaskMarketState::Bankrupt,
    );
    let mut h = fresh_harness(q0);

    // Step 1: redeem all 100 NO shares (NO wins under Bankrupt). After
    // this, alice has 100 YES + 0 NO.
    let parent_a = h.seq.q_snapshot().unwrap().state_root_t;
    let redeem_no = build_redeem(parent_a, "alice", "task-B", OutcomeSide::No, 100, 1);
    submit_and_apply(&mut h, redeem_no)
        .await
        .expect("redeem NO accepted under Bankrupt");

    // Step 2: attempt merge of 50 — owner has YES=100 ≥ 50 but NO=0 < 50.
    let parent_b = h.seq.q_snapshot().unwrap().state_root_t;
    let tx = build_merge(parent_b, "alice", "task-B", 50, 2);
    let err = submit_and_apply(&mut h, tx).await.expect_err("merge must reject one-sided");
    assert!(
        err.contains("MergeMoreThanOwned"),
        "expected MergeMoreThanOwned, got {err}"
    );
}

/// §7.3 verbatim — `merge_conserves_total_coin`.
///
/// Per architect manual §5.1 (CTF). Total Coin (across all 6 holdings via
/// `total_supply_micro`) MUST be bit-equal pre/post merge. Merge is a
/// symmetric balance ↔ collateral migration (no mint, no burn).
#[tokio::test]
async fn merge_conserves_total_coin() {
    let q0 = genesis_post_mint(&[("alice", 80)], "alice", "task-C", 40_000_000, TaskMarketState::Open);
    let mut h = fresh_harness(q0);
    let pre = total_supply_micro(&h.seq.q_snapshot().unwrap());

    let parent = h.seq.q_snapshot().unwrap().state_root_t;
    let tx = build_merge(parent, "alice", "task-C", 17_000_000, 1);
    submit_and_apply(&mut h, tx).await.expect("merge accepted");

    let post = total_supply_micro(&h.seq.q_snapshot().unwrap());
    assert_eq!(pre, post, "total Coin conserved across CompleteSetMerge");
}

/// §7.3 verbatim — `merge_reduces_collateral`.
///
/// Per architect manual §7.3: `conditional_collateral_t[event] -= amount`.
/// The event's locked collateral must shrink by exactly `amount` after
/// merge.
#[tokio::test]
async fn merge_reduces_collateral() {
    let q0 = genesis_post_mint(&[("alice", 50)], "alice", "task-D", 30_000_000, TaskMarketState::Open);
    let mut h = fresh_harness(q0);
    let event = EventId(TaskId("task-D".into()));

    let collat_pre = h
        .seq
        .q_snapshot()
        .unwrap()
        .economic_state_t
        .conditional_collateral_t
        .0
        .get(&event)
        .copied()
        .unwrap();
    assert_eq!(collat_pre.micro_units(), 30_000_000);

    let parent = h.seq.q_snapshot().unwrap().state_root_t;
    let tx = build_merge(parent, "alice", "task-D", 12_000_000, 1);
    submit_and_apply(&mut h, tx).await.expect("merge accepted");

    let collat_post = h
        .seq
        .q_snapshot()
        .unwrap()
        .economic_state_t
        .conditional_collateral_t
        .0
        .get(&event)
        .copied()
        .unwrap();
    assert_eq!(
        collat_post.micro_units(),
        30_000_000 - 12_000_000,
        "collateral debited by merge amount exactly"
    );
}

/// §7.3 verbatim — `merge_unavailable_after_final_redeem_if_shares_exhausted`.
///
/// Per architect manual §7.3 (operational subsumption). After both YES + NO
/// share balances are exhausted (e.g., owner redeemed everything post-
/// resolution), merge MUST fail because Step 3 (`MergeMoreThanOwned`) blocks.
/// Merge is not specially gated by market state — the share-balance check
/// is the operational guard.
#[tokio::test]
async fn merge_unavailable_after_final_redeem_if_shares_exhausted() {
    // Setup post-mint of 50 micro-units at a Finalized event (YES wins).
    let q0 = genesis_post_mint(
        &[("alice", 100)],
        "alice",
        "task-E",
        50,
        TaskMarketState::Finalized,
    );
    let mut h = fresh_harness(q0);

    // Redeem all YES shares (winning side under Finalized). Owner now has
    // YES=0 + NO=50 (NO is the losing side; cannot redeem post-Finalized).
    let parent_a = h.seq.q_snapshot().unwrap().state_root_t;
    let redeem_yes = build_redeem(parent_a, "alice", "task-E", OutcomeSide::Yes, 50, 1);
    submit_and_apply(&mut h, redeem_yes)
        .await
        .expect("redeem YES accepted under Finalized");

    // Sanity: NO shares remain but YES exhausted.
    let alice = AgentId("alice".into());
    let event = EventId(TaskId("task-E".into()));
    let pair_after_redeem = h
        .seq
        .q_snapshot()
        .unwrap()
        .economic_state_t
        .conditional_share_balances_t
        .0
        .get(&alice)
        .and_then(|m| m.get(&event))
        .copied()
        .unwrap_or_default();
    assert_eq!(pair_after_redeem.yes.units, 0, "YES exhausted by redeem");
    assert_eq!(pair_after_redeem.no.units, 50, "NO still held (loser side)");

    // Attempt merge of 25 — must fail because YES side exhausted.
    let parent_b = h.seq.q_snapshot().unwrap().state_root_t;
    let tx = build_merge(parent_b, "alice", "task-E", 25, 2);
    let err = submit_and_apply(&mut h, tx).await.expect_err(
        "merge must reject when YES exhausted by prior redeem",
    );
    assert!(
        err.contains("MergeMoreThanOwned"),
        "expected MergeMoreThanOwned post-redeem-exhaustion, got {err}"
    );
}
