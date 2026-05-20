//! TuringOS Constitution Gate — §5.3 CompleteSet hardening (architect 2026-05-07
//! ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL §5.3 verbatim).
//!
//! # Scope
//!
//! Architect alignment doc §2.4 explicitly authorizes "CompleteSet 当前实现审计"
//! (audit of current implementation) as work allowed-now alongside Stage B M2
//! batch. §5.3 verbatim mandates 8 hardening tests grouped Mint (3) + Redeem (5):
//!
//! ## §5.3 Mint
//!   - mint_one_coin_creates_one_yes_one_no
//!   - mint_conserves_total_coin
//!   - shares_not_counted_as_coin
//!
//! ## §5.3 Redeem
//!   - redeem_unavailable_before_resolution
//!   - redeem_yes_after_yes_pays_yes_not_no
//!   - redeem_no_after_no_pays_no_not_yes
//!   - redeem_cannot_exceed_share_balance
//!   - redeem_debits_collateral
//!
//! # Why a separate gate file (vs delegation to TB-13)
//!
//! TB-13 SG-13.* names cover the same semantic ground in
//! `tests/tb_13_complete_set.rs` but use TB-13-internal names and are NOT
//! registered in `scripts/run_constitution_gates.sh GATES=()` array — i.e.,
//! they ship-gate TB-13 but do NOT constitution-gate §5.3. Per
//! `feedback_no_workarounds_strict_constitution` ("我不要凑活"), this file
//! binds architect-verbatim §5.3 names directly to live sequencer dispatch
//! on `CompleteSetMintTx` / `CompleteSetRedeemTx` (fresh implementation,
//! NOT delegation-marker), making §5.3 first-class constitution gates
//! independent of TB-13's reorganization.
//!
//! TRACE_FLOWCHART_MATRIX:
//!   - FC1 §1 predicate routing (mint validation arm rejects non-Open / over-balance)
//!   - FC1 §6 monetary invariant (assert_complete_set_balanced live in dispatch)
//!   - §5.3 architect Polymarket manual

use std::sync::{Arc, RwLock};

use tempfile::TempDir;

use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::bottom_white::ledger::rejection_evidence::RejectionEvidenceWriter;
use turingosv4::bottom_white::ledger::system_keypair::{
    Ed25519Keypair, PinnedSystemPubkeys, SystemEpoch,
};
use turingosv4::bottom_white::ledger::transition_ledger::{InMemoryLedgerWriter, LedgerWriter};
use turingosv4::bottom_white::tools::registry::ToolRegistry;
use turingosv4::economy::monetary_invariant::total_supply_micro as canonical_total_supply_micro;
use turingosv4::economy::money::MicroCoin;
use turingosv4::state::q_state::{
    AgentId, QState, ShareSidePair, TaskId, TaskMarketEntry, TaskMarketState, TxId,
};
use turingosv4::state::sequencer::{Sequencer, SubmissionEnvelope};
use turingosv4::state::typed_tx::{
    AgentSignature, CompleteSetMintTx, CompleteSetRedeemTx, EventId, OutcomeSide, ShareAmount,
    TypedTx,
};
use turingosv4::top_white::predicates::registry::PredicateRegistry;

mod support;
use support::{fresh_harness, Harness};

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
    q.economic_state_t
        .task_markets_t
        .0
        .insert(TaskId(task.into()), entry);
}

fn genesis_with_open_task(pairs: &[(&str, i64)], task: &str) -> QState {
    let mut q = genesis_with_balances(pairs);
    seed_task_market(&mut q, task, TaskMarketState::Open);
    q
}

/// Build a post-mint snapshot directly (mint dispatch requires task=Open;
/// redeem-focused tests need task=Finalized/Bankrupt).
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
    q.economic_state_t
        .conditional_share_balances_t
        .0
        .insert(agent_id, owner_shares);
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

/// Wrap the canonical 6-holding production conservation sum
/// (`src/economy/monetary_invariant.rs::total_supply_micro`) — single
/// source of truth per architect §5.1 reasoning. Don't inline a second
/// definition (which would drift).
fn total_supply_micro(q: &QState) -> i64 {
    canonical_total_supply_micro(&q.economic_state_t)
        .expect("total_supply_micro must not overflow in test fixtures")
}

// ════════════════════════════════════════════════════════════════════════════
// §5.3 Mint hardening (3 verbatim names)
// ════════════════════════════════════════════════════════════════════════════

/// §5.3 verbatim — `mint_one_coin_creates_one_yes_one_no`.
///
/// CTF identity: 1 locked Coin → exactly 1 YES_E + 1 NO_E (architect 2026-05-07
/// §5.1). Asserts post-mint share-balance has equal YES.units == NO.units ==
/// mint amount, and balance debit equals collateral credit.
#[tokio::test]
async fn mint_one_coin_creates_one_yes_one_no() {
    let q0 = genesis_with_open_task(&[("alice", 100)], "task-A");
    let mut h = fresh_harness(q0);
    let parent = h.seq.q_snapshot().unwrap().state_root_t;

    submit_and_apply(&mut h, build_mint(parent, "alice", "task-A", 4_000_000, 1))
        .await
        .expect("mint accepted");

    let q = h.seq.q_snapshot().unwrap();

    // Balance debited.
    let alice_bal = q
        .economic_state_t
        .balances_t
        .0
        .get(&AgentId("alice".into()))
        .copied()
        .unwrap();
    assert_eq!(
        alice_bal.micro_units(),
        100_000_000 - 4_000_000,
        "alice balance debited by mint amount"
    );

    // Collateral credited.
    let collateral = q
        .economic_state_t
        .conditional_collateral_t
        .0
        .get(&EventId(TaskId("task-A".into())))
        .copied()
        .unwrap();
    assert_eq!(
        collateral.micro_units(),
        4_000_000,
        "conditional_collateral_t credited by mint amount"
    );

    // Equal YES + NO shares minted to owner.
    let pair = q
        .economic_state_t
        .conditional_share_balances_t
        .0
        .get(&AgentId("alice".into()))
        .and_then(|m| m.get(&EventId(TaskId("task-A".into()))))
        .copied()
        .unwrap();
    assert_eq!(pair.yes.units, 4_000_000_u128, "YES_E shares = mint amount");
    assert_eq!(pair.no.units, 4_000_000_u128, "NO_E shares  = mint amount");
    assert_eq!(
        pair.yes.units, pair.no.units,
        "1 Coin → 1 YES_E + 1 NO_E identity"
    );
}

/// §5.3 verbatim — `mint_conserves_total_coin`.
///
/// CTF preserved: total Coin supply (balances + collateral + escrow + bonds)
/// is unchanged across mint. Mint moves Coin from balances_t to
/// conditional_collateral_t — the 6-holding sum (architect Atom 3 monetary
/// invariant extension) treats both as Coin holdings, so total is bit-for-bit
/// preserved.
#[tokio::test]
async fn mint_conserves_total_coin() {
    let q0 = genesis_with_open_task(&[("alice", 100), ("bob", 50)], "task-A");
    let total_before = total_supply_micro(&q0);
    let mut h = fresh_harness(q0);
    let parent = h.seq.q_snapshot().unwrap().state_root_t;

    submit_and_apply(&mut h, build_mint(parent, "alice", "task-A", 4_000_000, 1))
        .await
        .expect("mint accepted");

    let total_after = total_supply_micro(&h.seq.q_snapshot().unwrap());
    assert_eq!(
        total_before, total_after,
        "total_supply_micro must be conserved across CompleteSetMintTx \
         (Coin migrates balance → collateral, both Coin holdings)"
    );
}

/// §5.3 verbatim — `shares_not_counted_as_coin`.
///
/// CR-13.3 + SG-13.2: YES/NO shares are CLAIMS, not Coin. The Coin total
/// is unchanged after mint even though new YES/NO shares are credited; a
/// regression that summed shares into total_supply_micro would inflate
/// total by 2× mint amount.
#[tokio::test]
async fn shares_not_counted_as_coin() {
    let q0 = genesis_with_open_task(&[("alice", 100)], "task-A");
    let total_before = total_supply_micro(&q0);
    let mut h = fresh_harness(q0);
    let parent = h.seq.q_snapshot().unwrap().state_root_t;

    submit_and_apply(&mut h, build_mint(parent, "alice", "task-A", 4_000_000, 1))
        .await
        .expect("mint accepted");

    let q = h.seq.q_snapshot().unwrap();
    let pair = q
        .economic_state_t
        .conditional_share_balances_t
        .0
        .get(&AgentId("alice".into()))
        .and_then(|m| m.get(&EventId(TaskId("task-A".into()))))
        .copied()
        .unwrap();

    // Sanity: shares were actually minted (~ test isn't vacuous).
    assert_eq!(pair.yes.units, 4_000_000_u128);
    assert_eq!(pair.no.units, 4_000_000_u128);

    // The shares MUST NOT be counted in total_supply_micro.
    let total_after = total_supply_micro(&q);
    assert_eq!(
        total_before, total_after,
        "YES/NO shares are claims, not Coin — total_supply_micro must \
         exclude conditional_share_balances_t (regression: shares-as-Coin)"
    );

    // Even sharper check: the difference between counting shares and not
    // counting shares is 2 × mint_amount. Confirm that gap is real (test
    // isn't vacuously asserting share supply is zero).
    let shares_total: u128 = pair.yes.units + pair.no.units;
    assert_eq!(
        shares_total, 8_000_000_u128,
        "YES + NO total = 2 × mint amount"
    );
}

// ════════════════════════════════════════════════════════════════════════════
// §5.3 Redeem hardening (5 verbatim names)
// ════════════════════════════════════════════════════════════════════════════

/// §5.3 verbatim — `redeem_unavailable_before_resolution`.
///
/// Sequencer arm Step 1 rejects redeem against task_markets_t in Open or
/// Expired state (or absent). Architect §4.3: redeem requires Finalized
/// (YES wins) or Bankrupt (NO wins).
#[tokio::test]
async fn redeem_unavailable_before_resolution() {
    // Task in Open state, post-mint snapshot — redeem must fail.
    let q0 = genesis_post_mint(
        &[("alice", 100)],
        "alice",
        "task-O",
        4_000_000,
        TaskMarketState::Open,
    );
    let mut h = fresh_harness(q0);
    let parent = h.seq.q_snapshot().unwrap().state_root_t;

    let err = submit_and_apply(
        &mut h,
        build_redeem(parent, "alice", "task-O", OutcomeSide::Yes, 4_000_000, 1),
    )
    .await
    .expect_err("redeem on Open event must reject");
    assert!(
        err.contains("RedeemBeforeResolution"),
        "expected RedeemBeforeResolution, got: {err}"
    );

    // Task in Expired state — same rejection.
    let q1 = genesis_post_mint(
        &[("bob", 100)],
        "bob",
        "task-X",
        4_000_000,
        TaskMarketState::Expired,
    );
    let mut h1 = fresh_harness(q1);
    let parent1 = h1.seq.q_snapshot().unwrap().state_root_t;
    let err = submit_and_apply(
        &mut h1,
        build_redeem(parent1, "bob", "task-X", OutcomeSide::Yes, 4_000_000, 2),
    )
    .await
    .expect_err("redeem on Expired event must reject");
    assert!(err.contains("RedeemBeforeResolution"), "got: {err}");
}

/// §5.3 verbatim — `redeem_yes_after_yes_pays_yes_not_no`.
///
/// Finalized state ⇒ YES wins. Owner with YES shares can redeem at 1:1
/// to MicroCoin. Owner's NO shares are NOT debited (they become worthless,
/// not transferable to Coin). Owner's balance restored from collateral.
#[tokio::test]
async fn redeem_yes_after_yes_pays_yes_not_no() {
    let q0 = genesis_post_mint(
        &[("alice", 100)],
        "alice",
        "task-Y",
        4_000_000,
        TaskMarketState::Finalized,
    );
    let mut h = fresh_harness(q0);
    let parent = h.seq.q_snapshot().unwrap().state_root_t;

    submit_and_apply(
        &mut h,
        build_redeem(parent, "alice", "task-Y", OutcomeSide::Yes, 4_000_000, 1),
    )
    .await
    .expect("redeem yes accepted on Finalized state");

    let q = h.seq.q_snapshot().unwrap();

    // Balance restored 1:1 from collateral.
    let bal = q
        .economic_state_t
        .balances_t
        .0
        .get(&AgentId("alice".into()))
        .copied()
        .unwrap();
    assert_eq!(
        bal.micro_units(),
        100_000_000,
        "balance restored = pre-mint amount"
    );

    // YES shares debited (winning side).
    let pair = q
        .economic_state_t
        .conditional_share_balances_t
        .0
        .get(&AgentId("alice".into()))
        .and_then(|m| m.get(&EventId(TaskId("task-Y".into()))))
        .copied()
        .unwrap();
    assert_eq!(
        pair.yes.units, 0,
        "YES shares debited (winning side burned)"
    );

    // NO shares preserved (losing side; worthless but not converted to Coin).
    assert_eq!(
        pair.no.units, 4_000_000,
        "NO shares preserved (losing side, no coin payout)"
    );

    // Cross-check: redeeming NO on this Finalized event must fail.
    let parent2 = h.seq.q_snapshot().unwrap().state_root_t;
    let err = submit_and_apply(
        &mut h,
        build_redeem(parent2, "alice", "task-Y", OutcomeSide::No, 1_000_000, 2),
    )
    .await
    .expect_err("redeem NO on Finalized must fail");
    assert!(err.contains("InvalidResolutionRef"), "got: {err}");
}

/// §5.3 verbatim — `redeem_no_after_no_pays_no_not_yes`.
///
/// Symmetric counterpart to redeem_yes_after_yes_pays_yes_not_no:
/// Bankrupt state ⇒ NO wins. Owner with NO shares can redeem 1:1 to Coin.
/// YES shares preserved (worthless). This file's separate test (vs TB-13's
/// embedded check inside sg_13_6) ensures both directions of the
/// resolution gate are first-class.
#[tokio::test]
async fn redeem_no_after_no_pays_no_not_yes() {
    let q0 = genesis_post_mint(
        &[("bob", 50)],
        "bob",
        "task-B",
        2_000_000,
        TaskMarketState::Bankrupt,
    );
    let mut h = fresh_harness(q0);
    let parent = h.seq.q_snapshot().unwrap().state_root_t;

    submit_and_apply(
        &mut h,
        build_redeem(parent, "bob", "task-B", OutcomeSide::No, 2_000_000, 1),
    )
    .await
    .expect("redeem no accepted on Bankrupt state");

    let q = h.seq.q_snapshot().unwrap();

    // Balance restored.
    let bal = q
        .economic_state_t
        .balances_t
        .0
        .get(&AgentId("bob".into()))
        .copied()
        .unwrap();
    assert_eq!(
        bal.micro_units(),
        50_000_000,
        "bob balance restored after NO redeem"
    );

    // NO shares debited (winning side).
    let pair = q
        .economic_state_t
        .conditional_share_balances_t
        .0
        .get(&AgentId("bob".into()))
        .and_then(|m| m.get(&EventId(TaskId("task-B".into()))))
        .copied()
        .unwrap();
    assert_eq!(
        pair.no.units, 0,
        "NO shares debited (winning side burned on Bankrupt)"
    );

    // YES shares preserved (losing side).
    assert_eq!(
        pair.yes.units, 2_000_000,
        "YES shares preserved (losing side, no payout)"
    );

    // Cross-check: redeem YES on Bankrupt must fail.
    let parent2 = h.seq.q_snapshot().unwrap().state_root_t;
    let err = submit_and_apply(
        &mut h,
        build_redeem(parent2, "bob", "task-B", OutcomeSide::Yes, 1_000_000, 2),
    )
    .await
    .expect_err("redeem YES on Bankrupt must fail");
    assert!(err.contains("InvalidResolutionRef"), "got: {err}");
}

/// §5.3 verbatim — `redeem_cannot_exceed_share_balance`.
///
/// Owner cannot redeem more shares than they hold. Sequencer arm Step 2:
/// owned_units < share_amount.units → RedeemMoreThanOwned.
#[tokio::test]
async fn redeem_cannot_exceed_share_balance() {
    let q0 = genesis_post_mint(
        &[("alice", 100)],
        "alice",
        "task-Y",
        4_000_000,
        TaskMarketState::Finalized,
    );
    let mut h = fresh_harness(q0);
    let parent = h.seq.q_snapshot().unwrap().state_root_t;

    // Try to redeem 5_000_001 (> 4_000_000 minted).
    let err = submit_and_apply(
        &mut h,
        build_redeem(parent, "alice", "task-Y", OutcomeSide::Yes, 5_000_001, 1),
    )
    .await
    .expect_err("redeem exceeding share balance must reject");
    assert!(
        err.contains("RedeemMoreThanOwned"),
        "expected RedeemMoreThanOwned, got: {err}"
    );

    // State unchanged after rejection.
    let q = h.seq.q_snapshot().unwrap();
    let pair = q
        .economic_state_t
        .conditional_share_balances_t
        .0
        .get(&AgentId("alice".into()))
        .and_then(|m| m.get(&EventId(TaskId("task-Y".into()))))
        .copied()
        .unwrap();
    assert_eq!(
        pair.yes.units, 4_000_000,
        "YES shares unchanged after rejection"
    );

    // Boundary: redeeming exactly the held amount succeeds.
    let parent = h.seq.q_snapshot().unwrap().state_root_t;
    submit_and_apply(
        &mut h,
        build_redeem(parent, "alice", "task-Y", OutcomeSide::Yes, 4_000_000, 2),
    )
    .await
    .expect("redeem exactly held amount must succeed");
}

/// §5.3 verbatim — `redeem_debits_collateral`.
///
/// Sequencer arm Step 4 (architect §4.3): on accepted redeem, debit
/// `conditional_collateral_t[event_id]` by share_amount and credit
/// `balances_t[owner]` 1:1. This test asserts the collateral side
/// explicitly (TB-13's sg_13_6 only checks balance + share-balance side).
#[tokio::test]
async fn redeem_debits_collateral() {
    let q0 = genesis_post_mint(
        &[("alice", 100)],
        "alice",
        "task-Y",
        4_000_000,
        TaskMarketState::Finalized,
    );

    let collateral_before = q0
        .economic_state_t
        .conditional_collateral_t
        .0
        .get(&EventId(TaskId("task-Y".into())))
        .copied()
        .unwrap()
        .micro_units();
    assert_eq!(collateral_before, 4_000_000, "collateral baseline");

    let mut h = fresh_harness(q0);
    let parent = h.seq.q_snapshot().unwrap().state_root_t;

    // Partial redeem first (1.5M of 4M).
    submit_and_apply(
        &mut h,
        build_redeem(parent, "alice", "task-Y", OutcomeSide::Yes, 1_500_000, 1),
    )
    .await
    .expect("partial redeem accepted");

    let q1 = h.seq.q_snapshot().unwrap();
    let collateral_mid = q1
        .economic_state_t
        .conditional_collateral_t
        .0
        .get(&EventId(TaskId("task-Y".into())))
        .copied()
        .unwrap()
        .micro_units();
    assert_eq!(
        collateral_mid,
        4_000_000 - 1_500_000,
        "collateral debited by partial-redeem amount"
    );

    // Full remaining redeem (2.5M).
    let parent = q1.state_root_t;
    submit_and_apply(
        &mut h,
        build_redeem(parent, "alice", "task-Y", OutcomeSide::Yes, 2_500_000, 2),
    )
    .await
    .expect("remaining redeem accepted");

    let q2 = h.seq.q_snapshot().unwrap();
    let collateral_end = q2
        .economic_state_t
        .conditional_collateral_t
        .0
        .get(&EventId(TaskId("task-Y".into())))
        .copied()
        .unwrap()
        .micro_units();
    assert_eq!(
        collateral_end, 0,
        "collateral fully debited after total redeem of YES side"
    );

    // Balance side mirrors collateral debit (1:1 conservation).
    let bal = q2
        .economic_state_t
        .balances_t
        .0
        .get(&AgentId("alice".into()))
        .copied()
        .unwrap()
        .micro_units();
    assert_eq!(
        bal, 100_000_000,
        "balance restored = collateral_before debited"
    );
}
