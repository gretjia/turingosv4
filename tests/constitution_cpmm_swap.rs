//! TuringOS Constitution Gate — Stage C P-M5 CPMM share-only swap (architect
//! 2026-05-07 ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL §7.6
//! verbatim).
//!
//! # Scope
//!
//! Architect alignment doc §7.6 mandates 6 hardening tests for the new
//! `CpmmSwapTx` (share-only YES↔NO swap on a CpmmPool):
//!
//!   - swap_no_for_yes_constant_product_non_decreasing
//!   - swap_yes_for_no_constant_product_non_decreasing
//!   - swap_fails_zero_input
//!   - swap_fails_insufficient_pool_output
//!   - swap_respects_min_out_slippage
//!   - swap_uses_integer_math_no_f64
//!
//! TRACE_FLOWCHART_MATRIX:
//!   - FC1 §1 predicate routing (admission rejects zero input / pool absent / under-balance / slippage)
//!   - FC1 §6 monetary invariant (CTF conservation; constant-product non-decreasing)
//!   - §7.6 architect Polymarket manual

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
use turingosv4::state::q_state::{
    AgentId, CpmmPool, LpShareAmount, PoolEventKind, PoolStatus, QState, ShareSidePair,
    TaskId,
};
use turingosv4::state::sequencer::{Sequencer, SubmissionEnvelope};
use turingosv4::state::typed_tx::{
    AgentSignature, CpmmSwapTx, EventId, ShareAmount, SwapSide, TypedTx,
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

/// Build a genesis state with: (a) sender holding `sender_yes` + `sender_no`
/// shares at `event_id`; (b) a CpmmPool at `event_id` with given reserves;
/// (c) collateral entry covering `max(pool_yes, pool_no, sender_yes, sender_no)`
/// to satisfy the structural invariant.
fn genesis_with_pool_and_sender(
    sender: &str,
    event_task: &str,
    sender_yes: u128,
    sender_no: u128,
    pool_yes: u128,
    pool_no: u128,
) -> QState {
    let mut q = QState::genesis();
    let event = EventId(TaskId(event_task.into()));

    // Collateral covers max-side reserves (no ghost liquidity).
    let max_units = sender_yes
        .max(sender_no)
        .max(pool_yes)
        .max(pool_no)
        + sender_yes + sender_no + pool_yes + pool_no; // upper-bound to be safe
    q.economic_state_t.conditional_collateral_t.0.insert(
        event.clone(),
        MicroCoin::from_micro_units(max_units as i64),
    );

    // Sender share holdings.
    let sender_id = AgentId(sender.into());
    let mut sender_shares = std::collections::BTreeMap::new();
    sender_shares.insert(
        event.clone(),
        ShareSidePair {
            yes: ShareAmount::from_units(sender_yes),
            no: ShareAmount::from_units(sender_no),
        },
    );
    q.economic_state_t
        .conditional_share_balances_t
        .0
        .insert(sender_id, sender_shares);

    // Pool with given reserves.
    q.economic_state_t.cpmm_pools_t.0.insert(
        event,
        CpmmPool {
            event_id_kind: PoolEventKind::BinaryYesNo,
            pool_yes: ShareAmount::from_units(pool_yes),
            pool_no: ShareAmount::from_units(pool_no),
            lp_total_shares: LpShareAmount::from_units(pool_yes), // arbitrary
            status: PoolStatus::Active,
        },
    );

    q
}

async fn submit_and_apply(h: &mut Harness, tx: TypedTx) -> Result<(), String> {
    h.seq.submit_agent_tx(tx).await
        .map_err(|e| format!("submit error: {e:?}"))?;
    let outcome = h.seq.try_apply_one(&mut h.rx)
        .ok_or_else(|| "no envelope drained".to_string())?;
    outcome.map(|_| ()).map_err(|e| format!("apply error: {e:?}"))
}

fn build_swap(
    parent: turingosv4::state::q_state::Hash,
    sender: &str,
    task: &str,
    side: SwapSide,
    amount_in: u128,
    min_out: u128,
    seq_no: u64,
) -> TypedTx {
    TypedTx::CpmmSwap(CpmmSwapTx {
        tx_id: turingosv4::state::q_state::TxId(format!("swap-{sender}-{task}-{seq_no}")),
        parent_state_root: parent,
        event_id: EventId(TaskId(task.into())),
        sender: AgentId(sender.into()),
        side,
        amount_in: ShareAmount::from_units(amount_in),
        min_out: ShareAmount::from_units(min_out),
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: 6000 + seq_no,
    })
}

// ════════════════════════════════════════════════════════════════════════════
// §7.6 P-M5 CpmmSwap hardening (6 verbatim names)
// ════════════════════════════════════════════════════════════════════════════

/// §7.6 verbatim — `swap_no_for_yes_constant_product_non_decreasing`.
///
/// Per architect manual §7.6: BuyYesWithNo with input dN > 0 →
/// outY = floor(dN * poolY / (poolN + dN)); poolN1 = poolN + dN;
/// poolY1 = poolY - outY. Floor keeps dust in pool, so
/// poolY1 * poolN1 >= poolY0 * poolN0 (constant-product non-decreasing).
#[tokio::test]
async fn swap_no_for_yes_constant_product_non_decreasing() {
    // Pool 1000 YES + 1000 NO; sender has 200 NO. Swap 100 NO → some YES.
    let q0 = genesis_with_pool_and_sender("alice", "task-A", 0, 200, 1000, 1000);
    let mut h = fresh_harness(q0);
    let parent = h.seq.q_snapshot().unwrap().state_root_t;

    let pool_pre = h
        .seq
        .q_snapshot()
        .unwrap()
        .economic_state_t
        .cpmm_pools_t
        .0
        .get(&EventId(TaskId("task-A".into())))
        .copied()
        .unwrap();
    let k_pre = pool_pre.pool_yes.units * pool_pre.pool_no.units;

    submit_and_apply(
        &mut h,
        build_swap(parent, "alice", "task-A", SwapSide::BuyYesWithNo, 100, 1, 1),
    )
    .await
    .expect("swap accepted");

    let pool_post = h
        .seq
        .q_snapshot()
        .unwrap()
        .economic_state_t
        .cpmm_pools_t
        .0
        .get(&EventId(TaskId("task-A".into())))
        .copied()
        .unwrap();
    let k_post = pool_post.pool_yes.units * pool_post.pool_no.units;

    assert!(
        k_post >= k_pre,
        "constant product non-decreasing: pre={} post={}",
        k_pre, k_post
    );
    // Sanity: poolN grew by amount_in; poolY shrank by some out_units.
    assert_eq!(pool_post.pool_no.units, 1100, "poolN += 100");
    assert!(
        pool_post.pool_yes.units < 1000,
        "poolY decreased due to swap output"
    );
}

/// §7.6 verbatim — `swap_yes_for_no_constant_product_non_decreasing`.
///
/// Symmetric to the above for BuyNoWithYes direction.
#[tokio::test]
async fn swap_yes_for_no_constant_product_non_decreasing() {
    let q0 = genesis_with_pool_and_sender("alice", "task-B", 200, 0, 1000, 1000);
    let mut h = fresh_harness(q0);
    let parent = h.seq.q_snapshot().unwrap().state_root_t;

    let pool_pre = h
        .seq
        .q_snapshot()
        .unwrap()
        .economic_state_t
        .cpmm_pools_t
        .0
        .get(&EventId(TaskId("task-B".into())))
        .copied()
        .unwrap();
    let k_pre = pool_pre.pool_yes.units * pool_pre.pool_no.units;

    submit_and_apply(
        &mut h,
        build_swap(parent, "alice", "task-B", SwapSide::BuyNoWithYes, 100, 1, 1),
    )
    .await
    .expect("swap accepted");

    let pool_post = h
        .seq
        .q_snapshot()
        .unwrap()
        .economic_state_t
        .cpmm_pools_t
        .0
        .get(&EventId(TaskId("task-B".into())))
        .copied()
        .unwrap();
    let k_post = pool_post.pool_yes.units * pool_post.pool_no.units;

    assert!(k_post >= k_pre, "constant product non-decreasing");
    assert_eq!(pool_post.pool_yes.units, 1100, "poolY += 100");
    assert!(pool_post.pool_no.units < 1000, "poolN decreased");
}

/// §7.6 verbatim — `swap_fails_zero_input`.
///
/// Per architect manual §7.6: input dN > 0 strictly. Zero-input swap is
/// rejected with `SwapZeroInput` to mirror Mint/Merge/Seed amount
/// discipline.
#[tokio::test]
async fn swap_fails_zero_input() {
    let q0 = genesis_with_pool_and_sender("alice", "task-C", 100, 100, 1000, 1000);
    let mut h = fresh_harness(q0);
    let parent = h.seq.q_snapshot().unwrap().state_root_t;

    let err = submit_and_apply(
        &mut h,
        build_swap(parent, "alice", "task-C", SwapSide::BuyYesWithNo, 0, 0, 1),
    )
    .await
    .expect_err("zero-input swap must fail");
    assert!(
        err.contains("SwapZeroInput"),
        "expected SwapZeroInput, got {err}"
    );
}

/// §7.6 verbatim — `swap_fails_insufficient_pool_output`.
///
/// Per architect manual §7.6: the formula `out = floor(dN * poolY /
/// (poolN + dN))` can yield zero when input is too small relative to pool
/// (or output side too small). When `out_units == 0` or
/// `out_units >= pool_output_side.units`, swap is rejected with
/// `SwapInsufficientPoolOutput`.
#[tokio::test]
async fn swap_fails_insufficient_pool_output() {
    // Pool 1000 YES + 100_000 NO; sender 1 NO. floor(1 * 1000 / 100001) = 0.
    let q0 = genesis_with_pool_and_sender("alice", "task-D", 0, 1, 1000, 100_000);
    let mut h = fresh_harness(q0);
    let parent = h.seq.q_snapshot().unwrap().state_root_t;

    let err = submit_and_apply(
        &mut h,
        build_swap(parent, "alice", "task-D", SwapSide::BuyYesWithNo, 1, 0, 1),
    )
    .await
    .expect_err("zero-output swap must fail");
    assert!(
        err.contains("SwapInsufficientPoolOutput"),
        "expected SwapInsufficientPoolOutput, got {err}"
    );
}

/// §7.6 verbatim — `swap_respects_min_out_slippage`.
///
/// Per architect manual §7.6: slippage protection. If computed `out_units`
/// is below sender's `min_out`, swap must reject with `SwapMinOutNotMet`.
#[tokio::test]
async fn swap_respects_min_out_slippage() {
    // Pool 1000+1000; swap 100 NO → outY = floor(100 * 1000 / 1100) = 90.
    // Sender demands min_out = 100 → must reject (slippage).
    let q0 = genesis_with_pool_and_sender("alice", "task-E", 0, 200, 1000, 1000);
    let mut h = fresh_harness(q0);
    let parent = h.seq.q_snapshot().unwrap().state_root_t;

    let err = submit_and_apply(
        &mut h,
        build_swap(parent, "alice", "task-E", SwapSide::BuyYesWithNo, 100, 100, 1),
    )
    .await
    .expect_err("swap must fail when min_out not met");
    assert!(
        err.contains("SwapMinOutNotMet"),
        "expected SwapMinOutNotMet, got {err}"
    );

    // Sanity: same swap with min_out = 90 succeeds.
    let parent = h.seq.q_snapshot().unwrap().state_root_t;
    submit_and_apply(
        &mut h,
        build_swap(parent, "alice", "task-E", SwapSide::BuyYesWithNo, 100, 90, 2),
    )
    .await
    .expect("swap accepted at min_out=90");
}

/// §7.6 verbatim — `swap_uses_integer_math_no_f64`.
///
/// Per architect manual §7.6 + universal forbidden list "no f64": swap
/// formula MUST use integer math. The existing
/// `tests/constitution_market_quarantine.rs::no_f64_in_market_modules`
/// asserts this at source-grep level over Stage C polymarket-tagged
/// modules; this test repeats the assertion specifically scoped to the
/// CPMM swap surface (sequencer dispatch arm + state-root mutator).
#[tokio::test]
async fn swap_uses_integer_math_no_f64() {
    // Source-grep over the sequencer module's CpmmSwap dispatch arm region.
    // The arm is anchored by its dispatch comment marker.
    let sequencer_src = std::fs::read_to_string("src/state/sequencer.rs")
        .expect("read src/state/sequencer.rs");

    // Locate the arm region: between marker `Stage C P-M5 — CpmmSwapTx accept arm`
    // and the next dispatch arm marker (or end of dispatch_transition).
    let start = sequencer_src
        .find("Stage C P-M5 — CpmmSwapTx accept arm")
        .expect("dispatch arm marker present");
    // Region ends at the next dispatch arm marker OR at the closing of
    // dispatch_transition. We bound to a generous slice and then search.
    let region_end = sequencer_src[start..]
        .find("\n        }\n        // ──────────")
        .map(|off| start + off)
        .unwrap_or(sequencer_src.len().min(start + 8000));
    let arm = &sequencer_src[start..region_end];

    // Strict grep: `f64` and `f32` literal-types must not appear.
    assert!(
        !arm.contains("f64") && !arm.contains("f32"),
        "no f64/f32 in CpmmSwap dispatch arm region (architect §7.6 + universal forbidden list)"
    );
    // The integer floor-division operator `/` and saturating-style methods
    // (checked_mul / checked_add) are the integer-math signature.
    assert!(
        arm.contains("checked_mul") && arm.contains("checked_add"),
        "CpmmSwap dispatch arm uses checked_* integer arithmetic"
    );
}
