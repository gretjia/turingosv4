//! Constitution gate — Phase F.2 P-M3 MarketSeedTx hardening.
//!
//! Authority: `handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_en.md`
//! §7.4 (lines 753-787) — 5 verbatim mandated test names + collateral-backed
//! semantics ("MarketSeedTx must be collateral-backed; provider deposits
//! seedC Coin; CompleteSetMint-like operation creates seedC YES + seedC NO;
//! YES/NO shares go to pool inventory; collateral locks seedC").
//!
//! Authority: `handover/directives/2026-05-09_STAGE_C_POLYMARKET_VETO_REMEDIATION_DIRECTIVE.md`
//! §1.C row 2 — "P-M3 MarketSeed (re-apply); Class 3; n/a (was correct);
//! per-atom §8 NO". Sub-option A2 framing per session #29 close prompt:
//! TB-13 era 7-field impl preserved as ratified state; §7.4 verbatim names
//! bound to first-class test bodies (this file) for workspace test surface
//! visibility. Pre-P-M4, inventory is held on provider's
//! `conditional_share_balances_t`; post-P-M4 it will shift to CpmmPool
//! reserves. Both cases satisfy `assert_complete_set_balanced`
//! (location-agnostic 1 collateral = 1 YES + 1 NO).
//!
//! Mirrors P-M2 `constitution_completeset_merge.rs` pattern: dedicated gate
//! file, verbatim architect-spec test names, real semantics (no delegation
//! markers), per `feedback_no_workarounds_strict_constitution`.
//!
//! /// TRACE_MATRIX Stage C P-M3 / Phase F.2 (architect manual §7.4 verbatim;
//! remediation directive §1.C row 2; Sub-option A2 framing).

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
use turingosv4::state::q_state::{AgentId, QState, TaskId, TaskMarketEntry, TaskMarketState, TxId};
use turingosv4::state::sequencer::{Sequencer, SubmissionEnvelope};
use turingosv4::state::typed_tx::{AgentSignature, EventId, MarketSeedTx, TypedTx};
use turingosv4::top_white::predicates::registry::PredicateRegistry;

// ── Harness (mirrors tb_13_complete_set.rs) ────────────────────────────────

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

fn build_seed(
    parent: turingosv4::state::q_state::Hash,
    provider: &str,
    task: &str,
    micro: i64,
    seq_no: u64,
) -> TypedTx {
    TypedTx::MarketSeed(MarketSeedTx {
        tx_id: TxId(format!("seed-{provider}-{task}-{seq_no}")),
        parent_state_root: parent,
        event_id: EventId(TaskId(task.into())),
        provider: AgentId(provider.into()),
        collateral_amount: MicroCoin::from_micro_units(micro),
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: 4000 + seq_no,
    })
}

// ── §7.4 verbatim mandated tests ────────────────────────────────────────────

/// §7.4 #1 — `market_seed_debits_provider`.
///
/// Provider deposits `collateral_amount` Coin → provider's `balances_t`
/// must be debited by exactly `collateral_amount`. Closes the surface
/// where a seed could mint inventory without touching the provider's
/// Coin balance.
#[tokio::test]
async fn market_seed_debits_provider() {
    let q0 = genesis_with_open_task(&[("provider", 100)], "task-DEBIT");
    let mut h = fresh_harness(q0);
    let parent = h.seq.q_snapshot().unwrap().state_root_t;

    submit_and_apply(
        &mut h,
        build_seed(parent, "provider", "task-DEBIT", 7_500_000, 1),
    )
    .await
    .expect("seed accepted");

    let q = h.seq.q_snapshot().unwrap();
    let provider_bal = q
        .economic_state_t
        .balances_t
        .0
        .get(&AgentId("provider".into()))
        .copied()
        .unwrap();
    assert_eq!(
        provider_bal.micro_units(),
        100_i64 * 1_000_000 - 7_500_000,
        "provider balance MUST be debited by exactly collateral_amount"
    );
}

/// §7.4 #2 — `market_seed_creates_yes_no_inventory`.
///
/// Architect spec: "CompleteSetMint-like operation creates seedC YES +
/// seedC NO; YES/NO shares go to pool inventory."
///
/// Pre-P-M4 (TB-13 era), inventory is held on provider's
/// `conditional_share_balances_t[event] = ShareSidePair { yes: seedC,
/// no: seedC }`. Post-P-M4 it will shift to `CpmmPool.pool_yes /
/// pool_no` reserves. Both cases must produce `seedC` units on each
/// side. This test asserts the TB-13-era invariant; the P-M4 ship gate
/// will add the pool-side assertion.
#[tokio::test]
async fn market_seed_creates_yes_no_inventory() {
    let q0 = genesis_with_open_task(&[("provider", 100)], "task-INV");
    let mut h = fresh_harness(q0);
    let parent = h.seq.q_snapshot().unwrap().state_root_t;

    submit_and_apply(
        &mut h,
        build_seed(parent, "provider", "task-INV", 4_242_424, 2),
    )
    .await
    .expect("seed accepted");

    let q = h.seq.q_snapshot().unwrap();
    let pair = q
        .economic_state_t
        .conditional_share_balances_t
        .0
        .get(&AgentId("provider".into()))
        .and_then(|m| m.get(&EventId(TaskId("task-INV".into()))))
        .copied()
        .expect("provider must hold YES + NO inventory after seed");
    assert_eq!(
        pair.yes.units, 4_242_424_u128,
        "YES inventory MUST equal collateral_amount"
    );
    assert_eq!(
        pair.no.units, 4_242_424_u128,
        "NO inventory MUST equal collateral_amount"
    );

    // Collateral must be locked (matches inventory created).
    let collateral = q
        .economic_state_t
        .conditional_collateral_t
        .0
        .get(&EventId(TaskId("task-INV".into())))
        .copied()
        .unwrap();
    assert_eq!(
        collateral.micro_units(),
        4_242_424,
        "collateral MUST lock exactly collateral_amount (1 collateral = 1 YES + 1 NO)"
    );
}

/// §7.4 #3 — `market_seed_fails_insufficient_balance`.
///
/// Provider with no balance row (or balance < collateral_amount) MUST
/// be rejected at the dispatch arm with `InsufficientBalanceForMint`.
/// Closes the surface where a seed could conjure liquidity without
/// debiting Coin.
#[tokio::test]
async fn market_seed_fails_insufficient_balance() {
    // "ghost" has no balance row. task-NOBAL is Open so the Q13 event
    // gate is not the firing one — the balance gate is.
    let q0 = genesis_with_open_task(&[("alice", 100)], "task-NOBAL");
    let mut h = fresh_harness(q0);
    let parent = h.seq.q_snapshot().unwrap().state_root_t;

    let err = submit_and_apply(
        &mut h,
        build_seed(parent, "ghost", "task-NOBAL", 1_000_000, 3),
    )
    .await
    .expect_err("seed without provider balance MUST be rejected");
    assert!(
        err.contains("InsufficientBalanceForMint"),
        "expected InsufficientBalanceForMint, got: {err}"
    );
}

/// §7.4 #4 — `market_seed_no_ghost_liquidity`.
///
/// Architect §4.7 forbidden list: "No automatic liquidity." Seeding
/// with `collateral_amount == 0` MUST be rejected — no inventory may
/// be created without backing collateral. Closes the surface where a
/// degenerate seed could short-circuit the collateral check while
/// still creating YES + NO entries.
#[tokio::test]
async fn market_seed_no_ghost_liquidity() {
    let q0 = genesis_with_open_task(&[("alice", 100)], "task-GHOST");
    let mut h = fresh_harness(q0);
    let parent = h.seq.q_snapshot().unwrap().state_root_t;

    let err = submit_and_apply(&mut h, build_seed(parent, "alice", "task-GHOST", 0, 4))
        .await
        .expect_err("seed with zero collateral MUST be rejected");
    assert!(
        err.contains("InsufficientCollateral"),
        "expected InsufficientCollateral, got: {err}"
    );

    // No ghost inventory must have appeared.
    let q = h.seq.q_snapshot().unwrap();
    assert!(
        q.economic_state_t
            .conditional_collateral_t
            .0
            .get(&EventId(TaskId("task-GHOST".into())))
            .is_none(),
        "no collateral row may exist after rejected zero-collateral seed"
    );
    assert!(
        q.economic_state_t
            .conditional_share_balances_t
            .0
            .get(&AgentId("alice".into()))
            .is_none_or(|m| m.get(&EventId(TaskId("task-GHOST".into()))).is_none()),
        "no inventory may exist after rejected zero-collateral seed"
    );
}

/// §7.4 #5 — `market_seed_conserves_total_coin`.
///
/// `assert_total_ctf_conserved` checks the 6-holding sum (CR-13.3): Coin
/// balances + conditional_collateral + conditional_shares (YES + NO
/// counted as 1/2 collateral per side). After a seed:
///   - provider balance: -collateral
///   - conditional_collateral: +collateral
///   - YES + NO shares: +collateral on each side (= +collateral when
///     shares are valued at 1/2 collateral per side per CR-13.3)
///
/// The 6-holding sum must be bit-equal pre/post. Closes the surface
/// where a seed could leak Coin (mint > collateral, or shares not
/// 1:1:1 with collateral).
#[tokio::test]
async fn market_seed_conserves_total_coin() {
    let q0 = genesis_with_open_task(&[("provider", 100)], "task-CONS");
    let mut h = fresh_harness(q0.clone());
    let parent = h.seq.q_snapshot().unwrap().state_root_t;

    submit_and_apply(
        &mut h,
        build_seed(parent, "provider", "task-CONS", 9_876_543, 5),
    )
    .await
    .expect("seed accepted");

    let q = h.seq.q_snapshot().unwrap();
    assert_total_ctf_conserved(&q0.economic_state_t, &q.economic_state_t, &[])
        .expect("total CTF conserved across seed (1 collateral = 1 YES + 1 NO)");
    assert_complete_set_balanced(&q.economic_state_t).expect("complete-set balanced post-seed");
}
