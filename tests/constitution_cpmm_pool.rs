//! Constitution gate — Stage C P-M4 / Phase F.3 CpmmPool LiquidityPool
//! state struct + creation tx semantics per architect manual §7.5.
//!
//! Authority: `handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_en.md`
//! §7.5 (verbatim 5-field STATE struct + 4-rule semantics block + 4
//! mandatory tests).
//! Companion: `handover/directives/2026-05-09_STAGE_C_POLYMARKET_VETO_REMEDIATION_DIRECTIVE.md`
//! §1.C row 3 (Class-4 STEP_B rebuild post Stage C VETO; defect 4 prevention
//! `event_id` NOT `event_id_kind`; mechanism gates Phase E.1 + E.2 + E.3
//! armed).
//!
//! Test names mirror architect §7.5 verbatim list:
//!   1. pool_created_from_seed_inventory
//!   2. pool_reserves_not_counted_as_coin
//!   3. lp_shares_not_counted_as_coin
//!   4. pool_cannot_exist_without_collateralized_shares
//!
//! Each test exercises the actual sequencer accept arm via `submit_and_apply`
//! against a real `Sequencer` + `InMemoryLedgerWriter`. No fixture forgery —
//! the post-state under test is reached by submitting CompleteSetMintTx to
//! seed provider's YES + NO inventory and CpmmPoolTx to create the pool
//! through the live ingress path. Per `feedback_tape_first_real_tests`: no
//! tape activity = not a TuringOS test; these tests advance state_root_t
//! through real transitions.

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
use turingosv4::state::q_state::{
    AgentId, LpShareAmount, PoolStatus, QState, TaskId, TaskMarketEntry, TaskMarketState, TxId,
};
use turingosv4::state::sequencer::{Sequencer, SubmissionEnvelope};
use turingosv4::state::typed_tx::{
    AgentSignature, CompleteSetMintTx, CpmmPoolTx, EventId, ShareAmount, TypedTx,
};
use turingosv4::top_white::predicates::registry::PredicateRegistry;

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

// ── Architect §7.5 verbatim test 1 ──────────────────────────────────────────

/// pool_created_from_seed_inventory — architect §7.5 verbatim: pool is
/// created from existing collateralized share inventory. Provider mints
/// YES + NO via CompleteSetMint, then submits CpmmPoolTx pulling shares
/// into pool reserves. Post-create: cpmm_pools_t entry exists with
/// status=Active, pool_yes==seed_yes, pool_no==seed_no, lp_total_shares
/// equals seed_yes; provider's conditional_share_balances_t debited; LP
/// shares credited 1:1.
#[tokio::test]
async fn pool_created_from_seed_inventory() {
    let q0 = genesis_with_balances_and_open_task(&[("alice", 10)], "evt-1");
    let mut h = fresh_harness(q0);

    // Step 1: alice mints 5_000_000 conditional shares (YES + NO).
    let parent = h.seq.q_snapshot().unwrap().state_root_t;
    submit_and_apply(&mut h, build_mint(parent, "alice", "evt-1", 5_000_000, 1))
        .await
        .expect("mint accepted");

    // Sanity: alice holds 5M YES + 5M NO; no pool exists yet.
    {
        let q = h.seq.q_snapshot().unwrap();
        let pair = q
            .economic_state_t
            .conditional_share_balances_t
            .0
            .get(&AgentId("alice".into()))
            .and_then(|m| m.get(&EventId(TaskId("evt-1".into()))))
            .copied()
            .unwrap();
        assert_eq!(pair.yes.units, 5_000_000_u128);
        assert_eq!(pair.no.units, 5_000_000_u128);
        assert!(
            !q.economic_state_t
                .cpmm_pools_t
                .0
                .contains_key(&EventId(TaskId("evt-1".into()))),
            "no pool exists pre-CpmmPoolTx"
        );
    }

    // Step 2: create pool seeded with 3M YES + 3M NO (symmetric).
    let parent_after_mint = h.seq.q_snapshot().unwrap().state_root_t;
    submit_and_apply(
        &mut h,
        build_pool(parent_after_mint, "alice", "evt-1", 3_000_000, 3_000_000, 1),
    )
    .await
    .expect("pool creation accepted");

    let q = h.seq.q_snapshot().unwrap();

    // Assert: pool entry exists with verbatim 5-field shape.
    let pool = q
        .economic_state_t
        .cpmm_pools_t
        .0
        .get(&EventId(TaskId("evt-1".into())))
        .expect("CpmmPool entry must exist post-CpmmPoolTx");
    assert_eq!(pool.event_id, EventId(TaskId("evt-1".into())));
    assert_eq!(
        pool.pool_yes.units, 3_000_000_u128,
        "pool_yes must equal seed_yes"
    );
    assert_eq!(
        pool.pool_no.units, 3_000_000_u128,
        "pool_no must equal seed_no"
    );
    assert_eq!(
        pool.lp_total_shares.units, 3_000_000_u128,
        "lp_total_shares = seed_yes (symmetric init formula)"
    );
    assert_eq!(pool.status, PoolStatus::Active, "pool starts Active");

    // Assert: provider's conditional_share_balances_t debited by seed.
    let pair = q
        .economic_state_t
        .conditional_share_balances_t
        .0
        .get(&AgentId("alice".into()))
        .and_then(|m| m.get(&EventId(TaskId("evt-1".into()))))
        .copied()
        .unwrap();
    assert_eq!(
        pair.yes.units, 2_000_000_u128,
        "provider YES debited by seed_yes"
    );
    assert_eq!(
        pair.no.units, 2_000_000_u128,
        "provider NO debited by seed_no"
    );

    // Assert: provider credited LP shares 1:1 with seed_yes.
    let lp = q
        .economic_state_t
        .lp_share_balances_t
        .0
        .get(&(AgentId("alice".into()), EventId(TaskId("evt-1".into()))))
        .copied()
        .unwrap();
    assert_eq!(
        lp.units, 3_000_000_u128,
        "provider LP shares = seed_yes.units (symmetric init)"
    );
}

// ── Architect §7.5 verbatim test 2 ──────────────────────────────────────────

/// pool_reserves_not_counted_as_coin — architect §7.5 rule 2: "pool reserves
/// are not Coin". The 6-holding total_supply_micro sum (architect TB-13
/// CR-13.3 + SG-13.2 carry-forward) MUST NOT include CpmmPool.pool_yes /
/// pool_no. Witness: assert_total_ctf_conserved with empty exempt-list
/// passes across the CpmmPoolTx → the conservation check would FAIL if
/// pool reserves were double-counted as Coin (because pool_yes + pool_no
/// would inflate the post-state Coin total by 6_000_000).
#[tokio::test]
async fn pool_reserves_not_counted_as_coin() {
    let q0 = genesis_with_balances_and_open_task(&[("bob", 8)], "evt-2");
    let mut h = fresh_harness(q0);

    // Mint to seed bob's inventory.
    let parent = h.seq.q_snapshot().unwrap().state_root_t;
    submit_and_apply(&mut h, build_mint(parent, "bob", "evt-2", 4_000_000, 1))
        .await
        .expect("mint accepted");

    // Snapshot pre-pool econ state for the conservation comparison.
    let q_pre_pool_econ = h.seq.q_snapshot().unwrap().economic_state_t.clone();

    // Create pool with 3M / 3M.
    let parent_after_mint = h.seq.q_snapshot().unwrap().state_root_t;
    submit_and_apply(
        &mut h,
        build_pool(parent_after_mint, "bob", "evt-2", 3_000_000, 3_000_000, 1),
    )
    .await
    .expect("pool creation accepted");

    let q_post = h.seq.q_snapshot().unwrap();

    // Pool reserves witnessed: 3M YES + 3M NO held inside CpmmPool.
    let pool = q_post
        .economic_state_t
        .cpmm_pools_t
        .0
        .get(&EventId(TaskId("evt-2".into())))
        .expect("pool exists");
    assert_eq!(pool.pool_yes.units, 3_000_000_u128);
    assert_eq!(pool.pool_no.units, 3_000_000_u128);

    // Assert: 6-holding CTF total preserved across CpmmPoolTx. If
    // total_supply_micro included pool_yes / pool_no, this would fail
    // (post-state would show 6M extra Coin out of nowhere).
    assert_total_ctf_conserved(&q_pre_pool_econ, &q_post.economic_state_t, &[])
        .expect("total Coin conserved across CpmmPoolTx (pool reserves NOT Coin)");
    // Complete-set invariant also holds post-pool (pool is the new holder
    // of 3M YES + 3M NO; collateral lock unchanged; provider retains 1M+1M).
    assert_complete_set_balanced(&q_post.economic_state_t)
        .expect("complete-set balanced post-pool-create");
}

// ── Architect §7.5 verbatim test 3 ──────────────────────────────────────────

/// lp_shares_not_counted_as_coin — architect §7.5 rule 3: "lp shares are not
/// Coin". The 6-holding total_supply_micro sum MUST NOT include
/// `lp_share_balances_t.values().*.units`. Witness: pool creation credits
/// 3M LP shares to provider; assert_total_ctf_conserved with empty
/// exempt-list passes — the check would FAIL if LP shares were summed
/// into the Coin total.
#[tokio::test]
async fn lp_shares_not_counted_as_coin() {
    let q0 = genesis_with_balances_and_open_task(&[("carol", 9)], "evt-3");
    let mut h = fresh_harness(q0);

    let parent = h.seq.q_snapshot().unwrap().state_root_t;
    submit_and_apply(&mut h, build_mint(parent, "carol", "evt-3", 5_000_000, 1))
        .await
        .expect("mint accepted");

    let q_pre_pool_econ = h.seq.q_snapshot().unwrap().economic_state_t.clone();

    // Pre-pool: lp_share_balances_t empty.
    assert!(
        q_pre_pool_econ.lp_share_balances_t.0.is_empty(),
        "no LP shares pre-pool-create"
    );

    let parent_after_mint = h.seq.q_snapshot().unwrap().state_root_t;
    submit_and_apply(
        &mut h,
        build_pool(parent_after_mint, "carol", "evt-3", 4_000_000, 4_000_000, 1),
    )
    .await
    .expect("pool creation accepted");

    let q_post = h.seq.q_snapshot().unwrap();

    // Witness: 4M LP shares now exist (credited to carol).
    let lp = q_post
        .economic_state_t
        .lp_share_balances_t
        .0
        .get(&(AgentId("carol".into()), EventId(TaskId("evt-3".into()))))
        .copied()
        .expect("LP shares credited");
    assert_eq!(lp.units, 4_000_000_u128);
    let lp_total: u128 = q_post
        .economic_state_t
        .lp_share_balances_t
        .0
        .values()
        .map(|x| x.units)
        .sum();
    assert_eq!(lp_total, 4_000_000_u128, "LP supply = 4M");

    // Assert: total Coin conserved despite 4M LP shares minted. If LP
    // shares were Coin, the post-state total would exceed the pre-state
    // total by 4M (the LP supply).
    assert_total_ctf_conserved(&q_pre_pool_econ, &q_post.economic_state_t, &[])
        .expect("total Coin conserved across CpmmPoolTx (LP shares NOT Coin)");
}

// ── Architect §7.5 verbatim test 4 ──────────────────────────────────────────

/// pool_cannot_exist_without_collateralized_shares — architect §7.5: pool
/// reserves must come from collateralized share inventory (architect
/// implies provider must hold YES + NO from a prior CompleteSetMintTx /
/// MarketSeedTx before pool creation; pool cannot conjure shares from
/// thin air). Provider with zero conditional shares submits CpmmPoolTx
/// → must be rejected with `InsufficientSharesForPool`. State unchanged
/// post-rejection.
#[tokio::test]
async fn pool_cannot_exist_without_collateralized_shares() {
    let q0 = genesis_with_balances_and_open_task(&[("dave", 10)], "evt-4");
    let q_pre_econ = q0.economic_state_t.clone();
    let mut h = fresh_harness(q0);

    // Pre-state sanity: dave has Coin balance but ZERO conditional shares.
    {
        let q = h.seq.q_snapshot().unwrap();
        let dave_bal = q
            .economic_state_t
            .balances_t
            .0
            .get(&AgentId("dave".into()))
            .copied()
            .unwrap();
        assert_eq!(dave_bal.micro_units(), 10_000_000);
        let pair = q
            .economic_state_t
            .conditional_share_balances_t
            .0
            .get(&AgentId("dave".into()))
            .and_then(|m| m.get(&EventId(TaskId("evt-4".into()))))
            .copied()
            .unwrap_or_default();
        assert_eq!(pair.yes.units, 0, "dave has zero YES inventory");
        assert_eq!(pair.no.units, 0, "dave has zero NO inventory");
    }

    // Attempt pool creation with seed_yes = seed_no = 1_000_000. Must fail
    // because dave's conditional_share_balances_t is empty → both
    // preconditions (yes >= seed_yes; no >= seed_no) trip with
    // `InsufficientSharesForPool`.
    let parent = h.seq.q_snapshot().unwrap().state_root_t;
    let err = submit_and_apply(
        &mut h,
        build_pool(parent, "dave", "evt-4", 1_000_000, 1_000_000, 1),
    )
    .await
    .expect_err("pool creation must fail without collateralized shares");
    assert!(
        err.contains("InsufficientSharesForPool"),
        "expected InsufficientSharesForPool, got: {err}"
    );

    // Assert: state UNCHANGED post-rejection. No pool entry created. No LP
    // shares credited. dave's balances + (empty) shares unchanged.
    let q_post = h.seq.q_snapshot().unwrap();
    assert!(
        q_post.economic_state_t.cpmm_pools_t.0.is_empty(),
        "no pool entry created on rejected CpmmPoolTx"
    );
    assert!(
        q_post.economic_state_t.lp_share_balances_t.0.is_empty(),
        "no LP shares credited on rejected CpmmPoolTx"
    );
    let dave_bal = q_post
        .economic_state_t
        .balances_t
        .0
        .get(&AgentId("dave".into()))
        .copied()
        .unwrap();
    assert_eq!(
        dave_bal.micro_units(),
        10_000_000,
        "dave balance unchanged on rejected CpmmPoolTx"
    );
    // Conservation invariants hold trivially on rejected tx (state not
    // mutated by accept arm).
    assert_total_ctf_conserved(&q_pre_econ, &q_post.economic_state_t, &[])
        .expect("rejected tx leaves total_supply_micro untouched");

    // Sanity: LpShareAmount API surface — zero(), from_units().
    let zero = LpShareAmount::zero();
    assert_eq!(zero.units, 0);
    let lp = LpShareAmount::from_units(123);
    assert_eq!(lp.units, 123);
}
