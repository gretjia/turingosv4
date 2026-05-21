//! REAL-11 Atom 2 — router positive-control fixture.
//!
//! Risk: Class 3, because this proves economic transition evidence. This file
//! deliberately stays in the test harness and does not modify sequencer,
//! typed-tx, CAS schema, or signing payload surfaces.
//!
//! FC mapping:
//! - FC1: scripted Trader action routes to L4 on predicate/admission pass and
//!   to L4.E on admission failure.
//! - FC2: runner evidence must be materialized under
//!   `handover/evidence/real11_router_positive_control_<UTC>/`.
//! - FC3: scripted positive control is substrate wire proof only, not E2.

use std::sync::{Arc, RwLock};

use tempfile::TempDir;
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::bottom_white::ledger::rejection_evidence::{
    RejectionClass, RejectionEvidenceWriter,
};
use turingosv4::bottom_white::ledger::system_keypair::{
    Ed25519Keypair, PinnedSystemPubkeys, SystemEpoch,
};
use turingosv4::bottom_white::ledger::transition_ledger::{
    InMemoryLedgerWriter, LedgerWriter, TxKind,
};
use turingosv4::bottom_white::tools::registry::ToolRegistry;
use turingosv4::economy::monetary_invariant::{
    assert_complete_set_balanced, assert_total_ctf_conserved,
};
use turingosv4::economy::money::MicroCoin;
use turingosv4::runtime::adapter::{tb_n3_invest_to_router_tx, InvestRouteError};
use turingosv4::runtime::agent_keypairs::AgentKeypairRegistry;
use turingosv4::runtime::agent_pnl::compute_agent_pnl;
use turingosv4::runtime::market_decision_trace::NoTradeReason;
use turingosv4::state::q_state::{
    AgentId, CpmmPool, Hash, LpShareAmount, PoolStatus, QState, TaskId, TaskMarketEntry,
    TaskMarketState, TxId,
};
use turingosv4::state::sequencer::{Sequencer, SubmissionEnvelope};
use turingosv4::state::typed_tx::{
    AgentSignature, BuyDirection, BuyWithCoinRouterTx, CompleteSetMintTx, CpmmPoolTx, EventId,
    ShareAmount, TypedTx,
};
use turingosv4::top_white::predicates::registry::PredicateRegistry;

struct Harness {
    _tmp: TempDir,
    seq: Sequencer,
    rx: tokio::sync::mpsc::Receiver<SubmissionEnvelope>,
    ledger: Arc<RwLock<dyn LedgerWriter>>,
    rejection_writer: Arc<RwLock<RejectionEvidenceWriter>>,
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
        rejection_writer.clone(),
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
        ledger: writer,
        rejection_writer,
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
        .map(|_| ())
        .map_err(|e| format!("apply error: {e:?}"))
}

fn build_mint(parent: Hash, owner: &str, task: &str, micro: i64, seq_no: u64) -> TypedTx {
    TypedTx::CompleteSetMint(CompleteSetMintTx {
        tx_id: TxId(format!("real11-mint-{owner}-{task}-{seq_no}")),
        parent_state_root: parent,
        event_id: EventId(TaskId(task.into())),
        owner: AgentId(owner.into()),
        amount: MicroCoin::from_micro_units(micro),
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: 1100 + seq_no,
    })
}

fn build_pool(parent: Hash, provider: &str, task: &str, seed_units: u128, seq_no: u64) -> TypedTx {
    TypedTx::CpmmPool(CpmmPoolTx {
        tx_id: TxId(format!("real11-pool-{provider}-{task}-{seq_no}")),
        parent_state_root: parent,
        event_id: EventId(TaskId(task.into())),
        provider: AgentId(provider.into()),
        seed_yes: ShareAmount::from_units(seed_units),
        seed_no: ShareAmount::from_units(seed_units),
        signature: AgentSignature::from_bytes([0u8; 64]),
    })
}

fn build_router(
    parent: Hash,
    buyer: &str,
    task: &str,
    direction: BuyDirection,
    pay_micro: i64,
    seq_no: u64,
) -> TypedTx {
    TypedTx::BuyWithCoinRouter(BuyWithCoinRouterTx {
        tx_id: TxId(format!(
            "real11-router-{buyer}-{task}-{seq_no}-{direction:?}"
        )),
        parent_state_root: parent,
        event_id: EventId(TaskId(task.into())),
        buyer: AgentId(buyer.into()),
        direction,
        pay_coin: MicroCoin::from_micro_units(pay_micro),
        min_out_shares: ShareAmount::from_units(0),
        signature: AgentSignature::from_bytes([0u8; 64]),
    })
}

async fn seed_pool(h: &mut Harness, provider: &str, task: &str, seed_units: u128) -> Hash {
    let p = h.seq.q_snapshot().unwrap().state_root_t;
    submit_and_apply(h, build_mint(p, provider, task, seed_units as i64, 1))
        .await
        .expect("provider mint accepted");
    let p = h.seq.q_snapshot().unwrap().state_root_t;
    submit_and_apply(h, build_pool(p, provider, task, seed_units, 1))
        .await
        .expect("pool creation accepted");
    h.seq.q_snapshot().unwrap().state_root_t
}

fn ledger_kinds(h: &Harness) -> Vec<TxKind> {
    let guard = h.ledger.read().expect("ledger");
    (1..=guard.len())
        .map(|logical_t| guard.read_at(logical_t).expect("entry").tx_kind)
        .collect()
}

fn router_rejection_public_summaries(h: &Harness) -> Vec<(TxKind, RejectionClass, Option<String>)> {
    let guard = h.rejection_writer.read().expect("rw");
    guard
        .records()
        .iter()
        .map(|r| (r.tx_kind, r.rejection_class, r.public_summary.clone()))
        .collect()
}

fn assert_no_ghost_liquidity(q: &QState, task: &str) {
    let event_id = EventId(TaskId(task.into()));
    let collateral = q
        .economic_state_t
        .conditional_collateral_t
        .0
        .get(&event_id)
        .copied()
        .expect("collateral exists")
        .micro_units() as u128;
    let mut yes = 0u128;
    let mut no = 0u128;
    for owner_map in q.economic_state_t.conditional_share_balances_t.0.values() {
        if let Some(pair) = owner_map.get(&event_id) {
            yes += pair.yes.units;
            no += pair.no.units;
        }
    }
    if let Some(pool) = q.economic_state_t.cpmm_pools_t.0.get(&event_id) {
        yes += pool.pool_yes.units;
        no += pool.pool_no.units;
    }
    assert_eq!(yes, no, "REAL-11 SG-11.2.6 no ghost liquidity");
    assert_eq!(yes, collateral, "REAL-11 SG-11.2.6 shares match collateral");
}

#[tokio::test]
async fn sg_11_2_1_scripted_buy_yes_router_enters_l4_and_updates_pnl() {
    let q0 = genesis_with_balances_and_open_task(&[("maker", 50), ("trader_yes", 10)], "evt-yes");
    let mut h = fresh_harness(q0);
    let parent = seed_pool(&mut h, "maker", "evt-yes", 5_000_000).await;
    let q_pre = h.seq.q_snapshot().unwrap();

    submit_and_apply(
        &mut h,
        build_router(
            parent,
            "trader_yes",
            "evt-yes",
            BuyDirection::BuyYes,
            1_000_000,
            1,
        ),
    )
    .await
    .expect("scripted BuyYesWithCoinRouterTx accepted");

    let kinds = ledger_kinds(&h);
    assert!(
        kinds.iter().any(|k| *k == TxKind::BuyWithCoinRouter),
        "SG-11.2.1 scripted BuyYesWithCoinRouterTx must enter L4"
    );
    let q_post = h.seq.q_snapshot().unwrap();
    assert_total_ctf_conserved(&q_pre.economic_state_t, &q_post.economic_state_t, &[])
        .expect("SG-11.2.5 CTF conserved across BuyYes router");
    assert_complete_set_balanced(&q_post.economic_state_t)
        .expect("SG-11.2.6 complete-set balanced post BuyYes");
    assert_no_ghost_liquidity(&q_post, "evt-yes");

    let pnl = compute_agent_pnl(&q_post, &AgentId("trader_yes".into()), 10_000_000);
    assert!(
        pnl.realized_pnl < 0,
        "scripted positive control must expose cash/PnL movement after router buy"
    );
    assert!(
        pnl.open_positions
            .iter()
            .any(|p| format!("{p:?}").contains("ConditionalShare")),
        "router buy must create an open conditional-share position"
    );
}

#[tokio::test]
async fn sg_11_2_2_scripted_buy_no_short_equivalent_has_explicit_l4_route() {
    let q0 = genesis_with_balances_and_open_task(&[("maker", 50), ("trader_no", 10)], "evt-no");
    let mut h = fresh_harness(q0);
    let parent = seed_pool(&mut h, "maker", "evt-no", 5_000_000).await;

    submit_and_apply(
        &mut h,
        build_router(
            parent,
            "trader_no",
            "evt-no",
            BuyDirection::BuyNo,
            500_000,
            1,
        ),
    )
    .await
    .expect("scripted BuyNo/short-equivalent route accepted");

    let q_post = h.seq.q_snapshot().unwrap();
    let no_units = q_post
        .economic_state_t
        .conditional_share_balances_t
        .0
        .get(&AgentId("trader_no".into()))
        .and_then(|m| m.get(&EventId(TaskId("evt-no".into()))))
        .map(|pair| pair.no.units)
        .unwrap_or(0);
    assert!(no_units > 500_000, "BuyNo trader receives NO exposure");
    assert!(
        ledger_kinds(&h)
            .iter()
            .any(|k| *k == TxKind::BuyWithCoinRouter),
        "SG-11.2.2 BuyNo/short-equivalent path has explicit L4 route"
    );
}

#[tokio::test]
async fn sg_11_2_3_insufficient_balance_is_preclassified_and_lands_l4e() {
    let mut keys = AgentKeypairRegistry::open(TempDir::new().unwrap().path()).expect("keys");
    let work_tx_id = "worktx-real11-insufficient-balance";
    let mut classifier_q = QState::default();
    classifier_q.economic_state_t.balances_t.0.insert(
        AgentId("poor_trader".into()),
        MicroCoin::from_micro_units(100_000),
    );
    let node_event = turingosv4::state::typed_tx::node_survive_event_id(&TxId(work_tx_id.into()));
    classifier_q.economic_state_t.cpmm_pools_t.0.insert(
        node_event.clone(),
        CpmmPool {
            event_id: node_event,
            pool_yes: ShareAmount::from_units(1_000_000),
            pool_no: ShareAmount::from_units(1_000_000),
            lp_total_shares: LpShareAmount::from_units(1_000_000),
            status: PoolStatus::Active,
        },
    );
    let err = tb_n3_invest_to_router_tx(
        &mut keys,
        Hash::ZERO,
        Some(&classifier_q),
        "poor_trader",
        work_tx_id,
        BuyDirection::BuyYes,
        1_000_000,
        0,
        "real11-preclassify-balance",
    )
    .expect_err("pre-submit classifier rejects balance shortfall");
    assert!(matches!(err, InvestRouteError::AmountExceedsBalance { .. }));
    assert_eq!(
        err.to_no_trade_reason(),
        NoTradeReason::AmountExceedsBalance
    );

    let q0 = genesis_with_balances_and_open_task(&[("maker", 50), ("poor_trader", 1)], "evt-poor");
    let mut h = fresh_harness(q0);
    let parent = seed_pool(&mut h, "maker", "evt-poor", 5_000_000).await;
    let err = submit_and_apply(
        &mut h,
        build_router(
            parent,
            "poor_trader",
            "evt-poor",
            BuyDirection::BuyYes,
            5_000_000,
            1,
        ),
    )
    .await
    .expect_err("submitted insufficient balance router reaches L4.E");
    assert!(err.contains("RouterInsufficientCoinBalance"), "{err}");
    assert!(
        router_rejection_public_summaries(&h)
            .iter()
            .any(|(kind, class, _)| {
                *kind == TxKind::BuyWithCoinRouter && *class == RejectionClass::PolicyViolation
            }),
        "SG-11.2.3 rejected router tx must be explicit L4.E"
    );
}

#[tokio::test]
async fn sg_11_2_4_missing_pool_is_no_pool_and_lands_l4e() {
    let mut keys = AgentKeypairRegistry::open(TempDir::new().unwrap().path()).expect("keys");
    let q = genesis_with_balances_and_open_task(&[("trader", 10)], "evt-missing");
    let err = tb_n3_invest_to_router_tx(
        &mut keys,
        Hash::ZERO,
        Some(&q),
        "trader",
        "worktx-real11-missing-pool",
        BuyDirection::BuyYes,
        1_000_000,
        0,
        "real11-preclassify-missing-pool",
    )
    .expect_err("pre-submit classifier rejects missing pool");
    assert!(matches!(err, InvestRouteError::UnknownEvent));
    assert_eq!(err.to_no_trade_reason(), NoTradeReason::NoPool);

    let mut h = fresh_harness(q);
    let parent = h.seq.q_snapshot().unwrap().state_root_t;
    let err = submit_and_apply(
        &mut h,
        build_router(
            parent,
            "trader",
            "evt-missing",
            BuyDirection::BuyNo,
            1_000_000,
            1,
        ),
    )
    .await
    .expect_err("submitted missing-pool router reaches L4.E");
    assert!(err.contains("RouterPoolNotActive"), "{err}");
    assert!(
        router_rejection_public_summaries(&h)
            .iter()
            .any(|(kind, class, _)| {
                *kind == TxKind::BuyWithCoinRouter && *class == RejectionClass::PolicyViolation
            }),
        "SG-11.2.4 missing pool must be explicit L4.E"
    );
}

#[test]
fn sg_11_2_7_router_money_path_has_no_executable_float_arithmetic() {
    for path in [
        "src/state/router_quote.rs",
        "src/runtime/agent_pnl.rs",
        "src/sdk/protocol.rs",
    ] {
        let source = std::fs::read_to_string(path).expect(path);
        if path == "src/sdk/protocol.rs" {
            assert!(
                source.contains("pub amount: Option<i64>"),
                "agent invest amount must parse as integer microCoin"
            );
            assert!(
                source.contains("test_parse_with_invest_float_amount_rejects"),
                "protocol must reject floating invest amounts at parser membrane"
            );
        }
        for (idx, line) in source.lines().enumerate() {
            let code = line.split("//").next().unwrap_or("");
            if path == "src/sdk/protocol.rs" {
                assert!(
                    !code.contains("amount.unwrap_or(0.0)")
                        && !code.contains("Option<f64>"),
                    "SG-11.2.7 no float-to-integer invest money path violation in {path}:{}: {line}",
                    idx + 1
                );
            } else {
                assert!(
                    !code.contains("f64") && !code.contains("f32"),
                    "SG-11.2.7 no f64/f32 money path violation in {path}:{}: {line}",
                    idx + 1
                );
            }
        }
    }
}

#[test]
fn runner_and_report_contract_make_evidence_mandatory_and_not_e2() {
    let script = std::fs::read_to_string("scripts/run_real11_router_positive_control.sh")
        .expect("REAL-11 Atom 2 runner must exist");
    assert!(script.contains("real11_router_positive_control_$(date -u"));
    assert!(script.contains("manifest.json"));
    assert!(script.contains("aggregate_verdict.json"));
    assert!(script.contains("REAL11_ROUTER_POSITIVE_CONTROL_VERDICT.json"));
    assert!(script.contains("REAL11_ROUTER_POSITIVE_CONTROL_REPORT.md"));
    assert!(script.contains("runtime_repo/"));
    assert!(script.contains("cas/"));
    assert!(script.contains("audit_dashboard_run_report.txt"));
    assert!(script.contains("TURINGOS_REAL7_SCRIPTED_TASK_OUTCOME_BUYS=\"Agent_1:Agent_2:1000\""));
    assert!(script.contains("buy_with_coin_router >= 2"));
    assert!(!script.contains("--no-evidence"));
    assert!(script.contains("scripted_positive_control_is_not_e2"));

    let report =
        std::fs::read_to_string("handover/reports/REAL11_ROUTER_POSITIVE_CONTROL_REPORT.md")
            .expect("REAL-11 Atom 2 report contract must exist");
    assert!(report.contains("scripted positive control is not E2"));
    assert!(report.contains("source evidence path"));
    assert!(report.contains("aggregate_verdict: `PROCEED`"));
}
