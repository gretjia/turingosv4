//! Constitution gate — Stage C P-M7 / Phase F.6 PriceIndex from CPMM /
//! exposure (architect manual §7.8).
//!
//! Authority: `handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_en.md`
//! §7.8 (verbatim "Price is signal only" + 4 mandatory tests).
//! Companion: `handover/directives/2026-05-09_STAGE_C_POLYMARKET_VETO_REMEDIATION_DIRECTIVE.md`
//! §1.C row 6 (Class 1-2; no §8; view-only quote).
//!
//! Test names mirror architect §7.8 verbatim list:
//!   1. price_quote_does_not_change_state
//!   2. price_signal_not_predicate
//!   3. price_does_not_make_failed_node_accepted
//!   4. low_liquidity_warning
//!
//! All tests exercise the live router quote function over real `CpmmPool`
//! state. Test 2 is a source-grep gate enforcing architect §7.8 explicit
//! "Do not use price to decide predicate truth" — the sequencer admission
//! arms must not import or call any function from the router_quote module.

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
use turingosv4::state::router_quote::{
    quote_buy_with_coin_router, LiquidityWarning, QuoteDirection,
};
use turingosv4::state::sequencer::{Sequencer, SubmissionEnvelope};
use turingosv4::state::typed_tx::{
    AgentSignature, BuyDirection, BuyWithCoinRouterTx, CompleteSetMintTx, CpmmPoolTx, EventId,
    ShareAmount, TypedTx,
};
use turingosv4::top_white::predicates::registry::PredicateRegistry;

// ── Harness (mirrors P-M5 / P-M6 patterns) ──────────────────────────────────

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

// ── Architect §7.8 verbatim test 1 ──────────────────────────────────────────

/// price_quote_does_not_change_state — architect §7.8 explicit "Price is
/// signal only". Quote function takes `&CpmmPool` (immutable ref); state
/// must be byte-identical pre/post quote call.
#[tokio::test]
async fn price_quote_does_not_change_state() {
    let q0 = genesis_with_balances_and_open_task(&[("alice", 50)], "evt-1");
    let mut h = fresh_harness(q0);

    // Seed pool.
    let p = h.seq.q_snapshot().unwrap().state_root_t;
    submit_and_apply(&mut h, build_mint(p, "alice", "evt-1", 5_000_000, 1))
        .await
        .expect("mint accepted");
    let p = h.seq.q_snapshot().unwrap().state_root_t;
    submit_and_apply(&mut h, build_pool(p, "alice", "evt-1", 5_000_000, 1))
        .await
        .expect("pool accepted");

    // Capture pre-quote state.
    let q_pre = h.seq.q_snapshot().unwrap();
    let state_root_pre = q_pre.state_root_t;
    let pool_pre = q_pre
        .economic_state_t
        .cpmm_pools_t
        .0
        .get(&EventId(TaskId("evt-1".into())))
        .cloned()
        .expect("pool present");

    // Issue many quotes — none should mutate state.
    for &payC in &[100_000_i64, 1_000_000, 5_000_000, 10_000_000] {
        for &dir in &[QuoteDirection::BuyYes, QuoteDirection::BuyNo] {
            let q = quote_buy_with_coin_router(&pool_pre, MicroCoin::from_micro_units(payC), dir);
            let _ = q; // explicitly discard — we only care that no mutation happened
        }
    }

    // Snapshot post-quote and verify byte-identical.
    let q_post = h.seq.q_snapshot().unwrap();
    assert_eq!(
        q_post.state_root_t, state_root_pre,
        "architect §7.8: price quote must not change state_root"
    );
    let pool_post = q_post
        .economic_state_t
        .cpmm_pools_t
        .0
        .get(&EventId(TaskId("evt-1".into())))
        .cloned()
        .expect("pool still present");
    assert_eq!(pool_post, pool_pre, "pool reserves unchanged by quote");
}

// ── Architect §7.8 verbatim test 2 ──────────────────────────────────────────

/// price_signal_not_predicate — architect §7.8 verbatim "Do not use price
/// to decide predicate truth." Source-grep gate: the sequencer admission
/// arms (`src/state/sequencer.rs`) MUST NOT import or call any function
/// from the `router_quote` module. The dispatch loop is predicate-gated
/// by typed transitions only; price quotes are pure derived views and
/// have no role in admission.
#[test]
fn price_signal_not_predicate() {
    use std::path::PathBuf;
    let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let sequencer_src = workspace.join("src/state/sequencer.rs");

    let seq_text = std::fs::read_to_string(&sequencer_src).expect("read src/state/sequencer.rs");

    // Forbidden imports / call patterns that would imply quote influences
    // admission decisions.
    let forbidden_patterns = [
        "use crate::state::router_quote",
        "router_quote::quote_buy_with_coin_router",
        "router_quote::quote_buy_with_coin_router_with_liquidity_threshold",
        "router_quote::QuoteDirection",
        "router_quote::RouterQuote",
        "router_quote::LiquidityWarning",
    ];

    for pat in &forbidden_patterns {
        assert!(
            !seq_text.contains(pat),
            "architect §7.8: sequencer.rs must not reference router_quote (`{pat}`); price is signal only"
        );
    }

    // Defense-in-depth: also check predicate registry + bus surfaces.
    let predicates_src = workspace.join("src/top_white/predicates");
    if predicates_src.is_dir() {
        for entry in std::fs::read_dir(&predicates_src).expect("predicates dir") {
            let path = entry.expect("entry").path();
            if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                let text = std::fs::read_to_string(&path).expect("read predicate file");
                for pat in &forbidden_patterns {
                    assert!(
                        !text.contains(pat),
                        "architect §7.8: predicate file {} must not reference router_quote (`{pat}`)",
                        path.display()
                    );
                }
            }
        }
    }
}

// ── Architect §7.8 verbatim test 3 ──────────────────────────────────────────

/// price_does_not_make_failed_node_accepted — architect §7.8 verbatim "Do
/// not use price to decide predicate truth." A router tx that fails
/// admission (e.g., RouterInsufficientCoinBalance, RouterSlippageExceeded)
/// MUST be rejected regardless of how favorable the price quote looks.
/// Witness: dispatch a router tx that would have a great quote (high
/// out_shares) but is doomed by a failing admission gate; assert it lands
/// in the rejection path, not L4 accepted.
#[tokio::test]
async fn price_does_not_make_failed_node_accepted() {
    let q0 = genesis_with_balances_and_open_task(
        &[("alice", 50), ("bob", 1)], // bob has only 1 Coin = 1M micro
        "evt-3",
    );
    let mut h = fresh_harness(q0);

    let p = h.seq.q_snapshot().unwrap().state_root_t;
    submit_and_apply(&mut h, build_mint(p, "alice", "evt-3", 5_000_000, 1))
        .await
        .expect("mint accepted");
    let p = h.seq.q_snapshot().unwrap().state_root_t;
    submit_and_apply(&mut h, build_pool(p, "alice", "evt-3", 5_000_000, 1))
        .await
        .expect("pool accepted");

    let q_after_seed = h.seq.q_snapshot().unwrap();
    let pool = q_after_seed
        .economic_state_t
        .cpmm_pools_t
        .0
        .get(&EventId(TaskId("evt-3".into())))
        .cloned()
        .expect("pool present");

    // Compute a quote for payC = 5_000_000 (5 Coin) — this would yield
    // a healthy price + non-zero out_shares.
    let payC: i64 = 5_000_000;
    let quote = quote_buy_with_coin_router(
        &pool,
        MicroCoin::from_micro_units(payC),
        QuoteDirection::BuyYes,
    )
    .expect("quote computed");
    assert!(
        quote.out_shares.units > 0,
        "quote shows non-zero out_shares"
    );
    assert_eq!(quote.liquidity_warning, LiquidityWarning::None);

    // Bob has only 1M Coin balance. Submitting a router tx for payC=5M
    // would fail at admission step Pre-4 (RouterInsufficientCoinBalance),
    // regardless of how attractive the quote is.
    let parent = h.seq.q_snapshot().unwrap().state_root_t;
    let err = submit_and_apply(
        &mut h,
        TypedTx::BuyWithCoinRouter(BuyWithCoinRouterTx {
            tx_id: TxId("bob-overdraft".into()),
            parent_state_root: parent,
            event_id: EventId(TaskId("evt-3".into())),
            buyer: AgentId("bob".into()),
            direction: BuyDirection::BuyYes,
            pay_coin: MicroCoin::from_micro_units(payC),
            min_out_shares: ShareAmount::from_units(0),
            signature: AgentSignature::from_bytes([0u8; 64]),
        }),
    )
    .await
    .expect_err("router tx must fail despite good quote");

    assert!(
        err.contains("RouterInsufficientCoinBalance"),
        "expected RouterInsufficientCoinBalance, got: {err}"
    );

    // State unchanged post-rejection.
    let q_post = h.seq.q_snapshot().unwrap();
    let pool_post = q_post
        .economic_state_t
        .cpmm_pools_t
        .0
        .get(&EventId(TaskId("evt-3".into())))
        .cloned()
        .expect("pool still present");
    assert_eq!(pool_post.pool_yes.units, pool.pool_yes.units);
    assert_eq!(pool_post.pool_no.units, pool.pool_no.units);
}

// ── Architect §7.8 verbatim test 4 ──────────────────────────────────────────

/// low_liquidity_warning — architect §7.8 implicit: liquidity-thin pools
/// should be flagged in the quote output. Tests that:
/// - A pool with reserves below the default threshold (1_000_000 units)
///   yields `LiquidityWarning::LowLiquidity`.
/// - A floor-zero quote (dust input vs asymmetric pool) yields
///   `LiquidityWarning::NoOutput` AND `price_effective: None`.
/// - A healthy quote yields `LiquidityWarning::None`.
#[test]
fn low_liquidity_warning() {
    use turingosv4::state::q_state::{CpmmPool, LpShareAmount, PoolStatus};

    // Healthy pool — None warning.
    let healthy = CpmmPool {
        event_id: EventId(TaskId("healthy".into())),
        pool_yes: ShareAmount::from_units(5_000_000),
        pool_no: ShareAmount::from_units(5_000_000),
        lp_total_shares: LpShareAmount::from_units(5_000_000),
        status: PoolStatus::Active,
    };
    let q = quote_buy_with_coin_router(
        &healthy,
        MicroCoin::from_micro_units(1_000_000),
        QuoteDirection::BuyYes,
    )
    .expect("healthy quote");
    assert_eq!(q.liquidity_warning, LiquidityWarning::None);
    assert!(q.price_effective.is_some());

    // Thin pool — LowLiquidity warning (below default 1M threshold).
    let thin = CpmmPool {
        event_id: EventId(TaskId("thin".into())),
        pool_yes: ShareAmount::from_units(100),
        pool_no: ShareAmount::from_units(100),
        lp_total_shares: LpShareAmount::from_units(100),
        status: PoolStatus::Active,
    };
    let q = quote_buy_with_coin_router(
        &thin,
        MicroCoin::from_micro_units(50),
        QuoteDirection::BuyYes,
    )
    .expect("thin quote");
    assert_eq!(q.liquidity_warning, LiquidityWarning::LowLiquidity);

    // Asymmetric extreme — NoOutput when floor returns 0.
    let asymmetric = CpmmPool {
        event_id: EventId(TaskId("asym".into())),
        pool_yes: ShareAmount::from_units(1),
        pool_no: ShareAmount::from_units(1_000_000_000),
        lp_total_shares: LpShareAmount::from_units(1),
        status: PoolStatus::Active,
    };
    let q = quote_buy_with_coin_router(
        &asymmetric,
        MicroCoin::from_micro_units(1),
        QuoteDirection::BuyYes,
    )
    .expect("asymmetric quote");
    assert_eq!(q.out_shares.units, 0);
    assert_eq!(q.liquidity_warning, LiquidityWarning::NoOutput);
    assert!(q.price_effective.is_none(), "no-output → price None");
}
