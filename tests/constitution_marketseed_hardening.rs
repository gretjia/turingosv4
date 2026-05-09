//! TuringOS Constitution Gate — Stage C P-M3 MarketSeed hardening (architect
//! 2026-05-07 ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL §7.4
//! verbatim).
//!
//! # Scope
//!
//! Architect alignment doc §7.4 mandates 5 hardening tests for `MarketSeedTx`
//! (provider deposits seedC Coin → CompleteSetMint-like operation creates
//! seedC YES + seedC NO; collateral locks seedC):
//!
//!   - market_seed_debits_provider
//!   - market_seed_creates_yes_no_inventory
//!   - market_seed_fails_insufficient_balance
//!   - market_seed_no_ghost_liquidity
//!   - market_seed_conserves_total_coin
//!
//! # Why a separate gate file (vs delegation to TB-13 SG-13.*)
//!
//! TB-13 ships SG-13.3 + SG-13.4 in `tests/tb_13_complete_set.rs` covering the
//! same semantic ground but using TB-13-internal SG-13.* names — those names
//! are NOT registered in `scripts/run_constitution_gates.sh GATES=()` array,
//! i.e. they ship-gate TB-13 but do NOT constitution-gate §7.4. Per
//! `feedback_no_workarounds_strict_constitution` ("我不要凑活"), this file
//! binds architect-verbatim §7.4 names directly to live sequencer dispatch
//! on `MarketSeedTx`, making §7.4 first-class constitution gates independent
//! of TB-13's reorganization. Closes the D.4 PARTIAL-W manifest row.
//!
//! TRACE_FLOWCHART_MATRIX:
//!   - FC1 §1 predicate routing (seed admission rejects under-balance / zero collateral)
//!   - FC1 §6 monetary invariant (assert_complete_set_balanced live in dispatch)
//!   - §7.4 architect Polymarket manual

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
    AgentId, QState, TaskId, TaskMarketEntry, TaskMarketState, TxId,
};
use turingosv4::state::sequencer::{Sequencer, SubmissionEnvelope};
use turingosv4::state::typed_tx::{
    AgentSignature, EventId, MarketSeedTx, TypedTx,
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

fn genesis_with_open_task(pairs: &[(&str, i64)], task: &str) -> QState {
    let mut q = QState::genesis();
    for (name, coin) in pairs {
        q.economic_state_t.balances_t.0.insert(
            AgentId((*name).into()),
            MicroCoin::from_coin(*coin).unwrap(),
        );
    }
    let mut entry = TaskMarketEntry::default();
    entry.state = TaskMarketState::Open;
    q.economic_state_t.task_markets_t.0.insert(TaskId(task.into()), entry);
    q
}

async fn submit_and_apply(h: &mut Harness, tx: TypedTx) -> Result<(), String> {
    h.seq.submit_agent_tx(tx).await
        .map_err(|e| format!("submit error: {e:?}"))?;
    let outcome = h.seq.try_apply_one(&mut h.rx)
        .ok_or_else(|| "no envelope drained".to_string())?;
    outcome.map(|_| ()).map_err(|e| format!("apply error: {e:?}"))
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
        timestamp_logical: 5000 + seq_no,
    })
}

/// Single-source-of-truth conservation sum (mirrors hardening test pattern).
fn total_supply_micro(q: &QState) -> i64 {
    canonical_total_supply_micro(&q.economic_state_t)
        .expect("total_supply_micro must not overflow in test fixtures")
}

// ════════════════════════════════════════════════════════════════════════════
// §7.4 P-M3 MarketSeed hardening (5 verbatim names)
// ════════════════════════════════════════════════════════════════════════════

/// §7.4 verbatim — `market_seed_debits_provider`.
///
/// Per architect manual §7.4: provider deposits `seedC` Coin → balance debit.
/// Asserts post-seed provider balance decreases by exactly `collateral_amount`.
#[tokio::test]
async fn market_seed_debits_provider() {
    let q0 = genesis_with_open_task(&[("alice", 100)], "task-A");
    let mut h = fresh_harness(q0);
    let parent = h.seq.q_snapshot().unwrap().state_root_t;

    submit_and_apply(&mut h, build_seed(parent, "alice", "task-A", 30_000_000, 1))
        .await
        .expect("seed accepted");

    let q = h.seq.q_snapshot().unwrap();
    let alice = AgentId("alice".into());
    let bal = q.economic_state_t.balances_t.0.get(&alice).copied().unwrap();
    assert_eq!(
        bal.micro_units(),
        100_000_000 - 30_000_000,
        "provider balance debited by collateral_amount exactly"
    );
}

/// §7.4 verbatim — `market_seed_creates_yes_no_inventory`.
///
/// Per architect manual §7.4: `seedC` YES + `seedC` NO shares minted to
/// provider's share inventory. Asserts post-seed YES.units == NO.units ==
/// collateral_amount.micro_units (1:1 unit mapping).
#[tokio::test]
async fn market_seed_creates_yes_no_inventory() {
    let q0 = genesis_with_open_task(&[("alice", 50)], "task-B");
    let mut h = fresh_harness(q0);
    let parent = h.seq.q_snapshot().unwrap().state_root_t;

    submit_and_apply(&mut h, build_seed(parent, "alice", "task-B", 17_000_000, 2))
        .await
        .expect("seed accepted");

    let q = h.seq.q_snapshot().unwrap();
    let alice = AgentId("alice".into());
    let event = EventId(TaskId("task-B".into()));
    let pair = q
        .economic_state_t
        .conditional_share_balances_t
        .0
        .get(&alice)
        .and_then(|m| m.get(&event))
        .copied()
        .unwrap_or_default();
    assert_eq!(pair.yes.units, 17_000_000, "YES shares == collateral micro-units");
    assert_eq!(pair.no.units, 17_000_000, "NO shares == collateral micro-units");
    assert_eq!(pair.yes.units, pair.no.units, "1:1 YES/NO inventory parity");
}

/// §7.4 verbatim — `market_seed_fails_insufficient_balance`.
///
/// Per architect manual §7.4 + SG-13.3: provider must have ≥ collateral_amount
/// in `balances_t`. If absent (or < amount), seed must fail with
/// `InsufficientBalanceForMint`.
#[tokio::test]
async fn market_seed_fails_insufficient_balance() {
    // Bob has no balance row; alice has 100 Coin but task-C is pre-Open.
    let q0 = genesis_with_open_task(&[("alice", 100)], "task-C");
    let mut h = fresh_harness(q0);
    let parent = h.seq.q_snapshot().unwrap().state_root_t;

    let err = submit_and_apply(&mut h, build_seed(parent, "bob", "task-C", 1_000_000, 3))
        .await
        .expect_err("seed must fail without provider balance");
    assert!(
        err.contains("InsufficientBalanceForMint"),
        "expected InsufficientBalanceForMint, got {err}"
    );
}

/// §7.4 verbatim — `market_seed_no_ghost_liquidity`.
///
/// Per architect manual §7.4 + universal forbidden list "no ghost liquidity":
/// any pool / inventory / share emission MUST trace to a Coin debit. A seed
/// with `collateral_amount == 0` would mint shares without any Coin
/// movement — i.e., ghost liquidity. Sequencer must reject with
/// `InsufficientCollateral`.
#[tokio::test]
async fn market_seed_no_ghost_liquidity() {
    let q0 = genesis_with_open_task(&[("alice", 100)], "task-D");
    let mut h = fresh_harness(q0);
    let parent = h.seq.q_snapshot().unwrap().state_root_t;

    // Zero-collateral seed must be rejected.
    let err = submit_and_apply(&mut h, build_seed(parent, "alice", "task-D", 0, 4))
        .await
        .expect_err("zero-collateral seed must fail (no ghost liquidity)");
    assert!(
        err.contains("InsufficientCollateral"),
        "expected InsufficientCollateral, got {err}"
    );

    // State sanity: no shares minted to alice at task-D.
    let q = h.seq.q_snapshot().unwrap();
    let alice = AgentId("alice".into());
    let event = EventId(TaskId("task-D".into()));
    let pair = q
        .economic_state_t
        .conditional_share_balances_t
        .0
        .get(&alice)
        .and_then(|m| m.get(&event))
        .copied()
        .unwrap_or_default();
    assert_eq!(pair.yes.units, 0, "no YES shares minted post zero-collateral reject");
    assert_eq!(pair.no.units, 0, "no NO shares minted post zero-collateral reject");
}

/// §7.4 verbatim — `market_seed_conserves_total_coin`.
///
/// Per architect manual §5.1 (CTF). Total Coin (across all 6 holdings via
/// `total_supply_micro`) MUST be bit-equal pre/post seed. Seed is a balance
/// → collateral migration (no mint, no burn).
#[tokio::test]
async fn market_seed_conserves_total_coin() {
    let q0 = genesis_with_open_task(&[("alice", 80)], "task-E");
    let mut h = fresh_harness(q0);
    let pre = total_supply_micro(&h.seq.q_snapshot().unwrap());

    let parent = h.seq.q_snapshot().unwrap().state_root_t;
    submit_and_apply(&mut h, build_seed(parent, "alice", "task-E", 22_000_000, 5))
        .await
        .expect("seed accepted");

    let post = total_supply_micro(&h.seq.q_snapshot().unwrap());
    assert_eq!(pre, post, "total Coin conserved across MarketSeed");
}
