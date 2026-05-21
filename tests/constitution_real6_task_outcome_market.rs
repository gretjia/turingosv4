//! REAL-6A — TaskOutcomeMarket at task open / escrow lock.
//!
//! Architect route 2026-05-15:
//! - SG-6A.1 TaskOutcomeMarket exists before first WorkTx.
//! - SG-6A.2 TraderView contains active TaskOutcomeMarket.
//! - SG-6A.3 NoPool no longer dominates when task market exists.
//! - SG-6A.4 Scripted trader can Buy YES/NO on TaskOutcomeMarket.
//! - SG-6A.5 Real LLM trader emits MarketDecisionTrace or classified NoTradeReason.
//! - SG-6A.6 EventResolveTx YES if verified proof before budget/deadline.
//! - SG-6A.7 EventResolveTx NO if exhausted/deadline without verified proof.
//! - SG-6A.8 No ghost liquidity.
//! - SG-6A.9 CTF conserved.
//! - SG-6A.10 Price never affects Lean predicate.
//!
//! FC-trace: FC1 externalized market action -> L4/L4.E, FC2 task-open
//! boot market replay, FC3 materialized TraderView/report; price is signal,
//! never predicate truth.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, RwLock};

use serde::{Deserialize, Serialize};
use tempfile::TempDir;

use turingosv4::bottom_white::cas::schema::Cid;
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::bottom_white::ledger::rejection_evidence::RejectionEvidenceWriter;
use turingosv4::bottom_white::ledger::system_keypair::{
    Ed25519Keypair, PinnedSystemPubkeys, SystemEpoch, SystemSignature,
};
use turingosv4::bottom_white::ledger::transition_ledger::{
    canonical_decode, canonical_encode, InMemoryLedgerWriter, LedgerWriter,
};
use turingosv4::bottom_white::tools::registry::ToolRegistry;
use turingosv4::economy::monetary_invariant::total_supply_micro;
use turingosv4::economy::money::MicroCoin;
use turingosv4::runtime::adapter::{
    tb_real6a_emit_task_outcome_no_after_exhaustion, tb_real6a_invest_task_outcome_to_router_tx,
};
use turingosv4::runtime::agent_keypairs::AgentKeypairRegistry;
use turingosv4::runtime::real5_roles::{
    derive_role_view, AgentRole, DerivedViewInput, DerivedViewRequest,
};
use turingosv4::runtime::real6_task_outcome::{
    task_outcome_event_for_task, task_outcome_price_signal, TaskOutcomeMarketKind,
};
use turingosv4::state::q_state::{AgentId, Hash, QState, TaskId, TaskMarketState, TxId};
use turingosv4::state::sequencer::{Sequencer, SubmissionEnvelope, SystemEmitCommand};
use turingosv4::state::typed_tx::{
    AgentSignature, BoolWithProof, BuyDirection, BuyWithCoinRouterTx, ChallengeResolveTx,
    ChallengeTx, CompleteSetMergeTx, CompleteSetMintTx, CompleteSetRedeemTx, CpmmPoolTx,
    CpmmSwapTx, EscrowLockTx, EventId, EventResolveTx, FinalizeRewardTx, MarketSeedTx, OutcomeSide,
    PredicateId, PredicateResultsBundle, ReadKey, ReuseTx, SafetyOrCreation, ShareAmount,
    TaskBankruptcyTx, TaskExpireTx, TaskOpenTx, TerminalSummaryTx, TypedTx, VerifyTx, WorkTx,
    WriteKey,
};
use turingosv4::top_white::predicates::registry::PredicateRegistry;

struct Harness {
    _tmp: TempDir,
    seq: Sequencer,
    rx: tokio::sync::mpsc::Receiver<SubmissionEnvelope>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
struct LegacyEventResolveTxWire {
    tx_id: TxId,
    parent_state_root: Hash,
    task_id: TaskId,
    epoch: SystemEpoch,
    timestamp_logical: u64,
    system_signature: SystemSignature,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
enum LegacyTypedTxWire {
    Work(WorkTx),
    Verify(VerifyTx),
    Challenge(ChallengeTx),
    Reuse(ReuseTx),
    FinalizeReward(FinalizeRewardTx),
    TaskExpire(TaskExpireTx),
    TerminalSummary(TerminalSummaryTx),
    TaskOpen(TaskOpenTx),
    EscrowLock(EscrowLockTx),
    ChallengeResolve(ChallengeResolveTx),
    TaskBankruptcy(TaskBankruptcyTx),
    CompleteSetMint(CompleteSetMintTx),
    CompleteSetRedeem(CompleteSetRedeemTx),
    MarketSeed(MarketSeedTx),
    CompleteSetMerge(CompleteSetMergeTx),
    CpmmPool(CpmmPoolTx),
    CpmmSwap(CpmmSwapTx),
    BuyWithCoinRouter(BuyWithCoinRouterTx),
    EventResolve(LegacyEventResolveTxWire),
}

fn fresh_harness(initial_q: QState) -> Harness {
    let tmp = TempDir::new().expect("tempdir");
    let cas = Arc::new(RwLock::new(CasStore::open(tmp.path()).expect("cas")));
    let keypair = Arc::new(Ed25519Keypair::generate_with_secure_entropy().expect("keypair"));
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
        writer,
        rejection_writer,
        preds,
        tools,
        pinned_pubkeys,
        initial_q,
        32,
    );
    Harness { _tmp: tmp, seq, rx }
}

fn genesis_with_balances(pairs: &[(&str, i64)]) -> QState {
    let mut q = QState::genesis();
    for (name, coin) in pairs {
        q.economic_state_t
            .balances_t
            .0
            .insert(AgentId((*name).into()), MicroCoin::from_micro_units(*coin));
    }
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

fn make_task_open(task: &str, sponsor: &str, parent: Hash, seq_no: u64) -> TypedTx {
    TypedTx::TaskOpen(TaskOpenTx {
        tx_id: TxId(format!("taskopen-{task}-{seq_no}")),
        task_id: TaskId(task.into()),
        parent_state_root: parent,
        sponsor_agent: AgentId(sponsor.into()),
        verifier_quorum: 1,
        max_reuse_royalty_fraction_basis_points: 1000,
        settlement_rule_hash: Hash::ZERO,
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: seq_no,
    })
}

fn make_escrow_lock(
    task: &str,
    sponsor: &str,
    amount_micro: i64,
    parent: Hash,
    seq_no: u64,
) -> TypedTx {
    TypedTx::EscrowLock(EscrowLockTx {
        tx_id: TxId(format!("escrowlock-{task}-{seq_no}")),
        task_id: TaskId(task.into()),
        parent_state_root: parent,
        sponsor_agent: AgentId(sponsor.into()),
        amount: MicroCoin::from_micro_units(amount_micro),
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: seq_no,
    })
}

async fn open_and_fund_task(h: &mut Harness, task: &str, sponsor: &str) {
    let parent = h.seq.q_snapshot().expect("pre").state_root_t;
    submit_and_apply(h, make_task_open(task, sponsor, parent, 1))
        .await
        .expect("TaskOpen accepted");
    let parent = h.seq.q_snapshot().expect("post-open").state_root_t;
    submit_and_apply(h, make_escrow_lock(task, sponsor, 1_000_000, parent, 2))
        .await
        .expect("EscrowLock accepted");
}

fn make_market_seed(task: &str, provider: &str, amount_micro: i64, parent: Hash) -> TypedTx {
    TypedTx::MarketSeed(MarketSeedTx {
        tx_id: TxId(format!("marketseed-{task}-{provider}")),
        parent_state_root: parent,
        event_id: EventId(TaskId(task.into())),
        provider: AgentId(provider.into()),
        collateral_amount: MicroCoin::from_micro_units(amount_micro),
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: 3,
    })
}

fn make_cpmm_pool(task: &str, provider: &str, seed_units: u128, parent: Hash) -> TypedTx {
    TypedTx::CpmmPool(CpmmPoolTx {
        tx_id: TxId(format!("pool-{task}-{provider}")),
        parent_state_root: parent,
        event_id: EventId(TaskId(task.into())),
        provider: AgentId(provider.into()),
        seed_yes: ShareAmount::from_units(seed_units),
        seed_no: ShareAmount::from_units(seed_units),
        signature: AgentSignature::from_bytes([0u8; 64]),
    })
}

fn make_mint(task: &str, owner: &str, amount_micro: i64, parent: Hash) -> TypedTx {
    TypedTx::CompleteSetMint(CompleteSetMintTx {
        tx_id: TxId(format!("mint-{task}-{owner}")),
        parent_state_root: parent,
        event_id: EventId(TaskId(task.into())),
        owner: AgentId(owner.into()),
        amount: MicroCoin::from_micro_units(amount_micro),
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: 4,
    })
}

fn make_redeem(
    task: &str,
    owner: &str,
    outcome: OutcomeSide,
    share_units: u128,
    parent: Hash,
) -> TypedTx {
    let outcome_label = match outcome {
        OutcomeSide::Yes => "yes",
        OutcomeSide::No => "no",
    };
    TypedTx::CompleteSetRedeem(CompleteSetRedeemTx {
        tx_id: TxId(format!("redeem-{task}-{owner}-{outcome_label}")),
        parent_state_root: parent,
        event_id: EventId(TaskId(task.into())),
        owner: AgentId(owner.into()),
        outcome,
        share_amount: ShareAmount::from_units(share_units),
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: 5,
    })
}

fn make_work(task: &str, agent: &str, parent: Hash) -> TypedTx {
    let mut acceptance = BTreeMap::new();
    acceptance.insert(
        PredicateId("lean_check".into()),
        BoolWithProof {
            value: true,
            proof_cid: None,
        },
    );
    let mut settlement = BTreeMap::new();
    settlement.insert(
        PredicateId("safety".into()),
        BoolWithProof {
            value: true,
            proof_cid: None,
        },
    );
    TypedTx::Work(WorkTx {
        tx_id: TxId(format!("work-{task}-{agent}")),
        task_id: TaskId(task.into()),
        parent_state_root: parent,
        agent_id: AgentId(agent.into()),
        read_set: BTreeSet::from([ReadKey("lean-goal".into())]),
        write_set: BTreeSet::from([WriteKey("proof".into())]),
        proposal_cid: Cid::default(),
        predicate_results: PredicateResultsBundle {
            acceptance,
            settlement,
            safety_class: SafetyOrCreation::Safety,
        },
        stake: turingosv4::economy::money::StakeMicroCoin::from_micro_units(100),
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: 6,
    })
}

#[test]
fn sg_6a_task_outcome_event_schema_is_task_open_scope() {
    let event = task_outcome_event_for_task(
        "evt-real6a-task",
        TaskId("task-real6a".into()),
        9,
        MicroCoin::from_micro_units(1_000_000),
        TxId("taskopen-real6a".into()),
    );
    assert_eq!(event.kind, TaskOutcomeMarketKind::TaskOutcome);
    assert_eq!(event.task_id, TaskId("task-real6a".into()));
    assert_eq!(event.deadline_round, 9);
    assert_eq!(event.max_budget, MicroCoin::from_micro_units(1_000_000));
    assert_eq!(
        event.created_by_task_open_tx,
        TxId("taskopen-real6a".into())
    );
}

#[tokio::test]
async fn sg_6a_1_market_seed_and_pool_exist_before_first_worktx() {
    let mut h = fresh_harness(genesis_with_balances(&[
        ("sponsor", 10_000_000),
        ("MarketMakerBudget", 10_000_000),
        ("Agent_0", 1_000_000),
    ]));
    let task = "task-real6a-pre-work";
    open_and_fund_task(&mut h, task, "sponsor").await;

    let post_escrow = h.seq.q_snapshot().expect("post escrow");
    assert!(
        post_escrow
            .economic_state_t
            .cpmm_pools_t
            .0
            .get(&EventId(TaskId(task.into())))
            .is_none(),
        "pre-condition: no pool before TaskOutcomeMarket seed"
    );

    let parent = post_escrow.state_root_t;
    submit_and_apply(
        &mut h,
        make_market_seed(task, "MarketMakerBudget", 200_000, parent),
    )
    .await
    .expect("MarketSeed accepted");
    let parent = h.seq.q_snapshot().expect("post seed").state_root_t;
    submit_and_apply(
        &mut h,
        make_cpmm_pool(task, "MarketMakerBudget", 200_000, parent),
    )
    .await
    .expect("CpmmPool accepted");

    let before_work = h.seq.q_snapshot().expect("before work");
    let event_id = EventId(TaskId(task.into()));
    let pool = before_work
        .economic_state_t
        .cpmm_pools_t
        .0
        .get(&event_id)
        .expect("TaskOutcomeMarket pool exists before first WorkTx");
    assert_eq!(pool.pool_yes.units, 200_000);
    assert_eq!(pool.pool_no.units, 200_000);

    let parent = before_work.state_root_t;
    submit_and_apply(&mut h, make_work(task, "Agent_0", parent))
        .await
        .expect("first WorkTx accepted after task market exists");
}

#[tokio::test]
async fn sg_6a_8_9_no_ghost_liquidity_and_ctf_conserved() {
    let mut h = fresh_harness(genesis_with_balances(&[
        ("sponsor", 10_000_000),
        ("MarketMakerBudget", 10_000_000),
    ]));
    let task = "task-real6a-liquidity";
    open_and_fund_task(&mut h, task, "sponsor").await;
    let pre = h.seq.q_snapshot().expect("pre seed");
    let pre_total = total_supply_micro(&pre.economic_state_t).expect("pre total");
    let pre_provider_balance = pre
        .economic_state_t
        .balances_t
        .0
        .get(&AgentId("MarketMakerBudget".into()))
        .copied()
        .expect("provider balance");

    submit_and_apply(
        &mut h,
        make_market_seed(task, "MarketMakerBudget", 250_000, pre.state_root_t),
    )
    .await
    .expect("MarketSeed accepted");
    let post_seed = h.seq.q_snapshot().expect("post seed");
    submit_and_apply(
        &mut h,
        make_cpmm_pool(task, "MarketMakerBudget", 250_000, post_seed.state_root_t),
    )
    .await
    .expect("CpmmPool accepted");

    let post_pool = h.seq.q_snapshot().expect("post pool");
    let post_total = total_supply_micro(&post_pool.economic_state_t).expect("post total");
    assert_eq!(post_total, pre_total, "CTF must be conserved");
    let post_provider_balance = post_pool
        .economic_state_t
        .balances_t
        .0
        .get(&AgentId("MarketMakerBudget".into()))
        .copied()
        .expect("provider balance");
    assert_eq!(
        pre_provider_balance.micro_units() - post_provider_balance.micro_units(),
        250_000,
        "MarketMakerBudget must pay collateral; pool cannot be ghost liquidity"
    );
    assert_eq!(
        post_pool
            .economic_state_t
            .conditional_collateral_t
            .0
            .get(&EventId(TaskId(task.into())))
            .copied()
            .unwrap_or_default()
            .micro_units(),
        250_000
    );
}

#[tokio::test]
async fn sg_6a_3_4_scripted_trader_buys_yes_and_no_on_task_outcome_market() {
    let mut h = fresh_harness(genesis_with_balances(&[
        ("sponsor", 10_000_000),
        ("MarketMakerBudget", 10_000_000),
        ("Agent_0", 1_000_000),
        ("Agent_1", 1_000_000),
    ]));
    let task = "task-real6a-scripted-buy";
    open_and_fund_task(&mut h, task, "sponsor").await;
    let parent = h.seq.q_snapshot().expect("post escrow").state_root_t;
    submit_and_apply(
        &mut h,
        make_market_seed(task, "MarketMakerBudget", 300_000, parent),
    )
    .await
    .expect("MarketSeed accepted");
    let parent = h.seq.q_snapshot().expect("post seed").state_root_t;
    submit_and_apply(
        &mut h,
        make_cpmm_pool(task, "MarketMakerBudget", 300_000, parent),
    )
    .await
    .expect("CpmmPool accepted");

    let key_tmp = TempDir::new().expect("key tempdir");
    let mut registry = AgentKeypairRegistry::open(key_tmp.path()).expect("registry");
    let q_for_yes = h.seq.q_snapshot().expect("q yes");
    let yes_tx = tb_real6a_invest_task_outcome_to_router_tx(
        &mut registry,
        q_for_yes.state_root_t,
        Some(&q_for_yes),
        "Agent_0",
        task,
        BuyDirection::BuyYes,
        10_000,
        0,
        "yes",
    )
    .expect("TaskOutcomeMarket YES route should not be NoPool");
    submit_and_apply(&mut h, yes_tx)
        .await
        .expect("scripted YES BuyWithCoinRouterTx accepted");

    let q_for_no = h.seq.q_snapshot().expect("q no");
    let no_tx = tb_real6a_invest_task_outcome_to_router_tx(
        &mut registry,
        q_for_no.state_root_t,
        Some(&q_for_no),
        "Agent_1",
        task,
        BuyDirection::BuyNo,
        10_000,
        0,
        "no",
    )
    .expect("TaskOutcomeMarket NO route should not be NoPool");
    submit_and_apply(&mut h, no_tx)
        .await
        .expect("scripted NO BuyWithCoinRouterTx accepted");
}

#[tokio::test]
async fn sg_6a_6_7_event_resolve_yes_and_no_paths_exist() {
    let mut h = fresh_harness(genesis_with_balances(&[
        ("sponsor", 10_000_000),
        ("alice", 10_000_000),
    ]));
    let task_yes = "task-real6a-yes";
    open_and_fund_task(&mut h, task_yes, "sponsor").await;
    h.seq
        .emit_system_tx(SystemEmitCommand::EventResolve {
            task_id: TaskId(task_yes.into()),
            outcome: OutcomeSide::Yes,
        })
        .await
        .expect("EventResolve YES emitted");
    h.seq
        .try_apply_one(&mut h.rx)
        .expect("yes env")
        .expect("yes accepted");
    let yes_state = h
        .seq
        .q_snapshot()
        .expect("yes snapshot")
        .economic_state_t
        .task_markets_t
        .0
        .get(&TaskId(task_yes.into()))
        .expect("yes task")
        .state;
    assert_eq!(yes_state, TaskMarketState::Finalized);

    let task_no = "task-real6a-no";
    open_and_fund_task(&mut h, task_no, "sponsor").await;
    let parent = h.seq.q_snapshot().expect("pre mint").state_root_t;
    submit_and_apply(&mut h, make_mint(task_no, "alice", 300_000, parent))
        .await
        .expect("alice mint accepted");
    h.seq
        .emit_system_tx(SystemEmitCommand::EventResolve {
            task_id: TaskId(task_no.into()),
            outcome: OutcomeSide::No,
        })
        .await
        .expect("EventResolve NO emitted");
    h.seq
        .try_apply_one(&mut h.rx)
        .expect("no env")
        .expect("no accepted");
    let no_snapshot = h.seq.q_snapshot().expect("no snapshot");
    let no_state = no_snapshot
        .economic_state_t
        .task_markets_t
        .0
        .get(&TaskId(task_no.into()))
        .expect("no task")
        .state;
    assert_eq!(
        no_state,
        TaskMarketState::Bankrupt,
        "EventResolve outcome=No must resolve the event to the existing NO-wins state"
    );
    let parent = no_snapshot.state_root_t;
    submit_and_apply(
        &mut h,
        make_redeem(task_no, "alice", OutcomeSide::No, 300_000, parent),
    )
    .await
    .expect("NO-side redeem admitted after EventResolve NO");
}

#[tokio::test]
async fn sg_6a_7_adapter_helper_emits_no_event_resolve_for_exhaustion() {
    let mut h = fresh_harness(genesis_with_balances(&[("sponsor", 10_000_000)]));
    let task = "task-real6a-exhausted-no";
    open_and_fund_task(&mut h, task, "sponsor").await;

    let emitted = tb_real6a_emit_task_outcome_no_after_exhaustion(&h.seq, TaskId(task.to_string()))
        .await
        .expect("emit EventResolve NO");
    assert!(emitted, "open exhausted task should emit EventResolve NO");

    h.seq
        .try_apply_one(&mut h.rx)
        .expect("no env")
        .expect("no accepted");
    let state = h
        .seq
        .q_snapshot()
        .expect("snapshot")
        .economic_state_t
        .task_markets_t
        .0
        .get(&TaskId(task.to_string()))
        .expect("task")
        .state;
    assert_eq!(
        state,
        TaskMarketState::Bankrupt,
        "exhaustion helper must make NO resolution tape-visible via EventResolveTx"
    );
}

#[test]
fn sg_6a_event_resolve_outcome_is_in_signing_payload_digest() {
    let base = EventResolveTx {
        tx_id: TxId("system-event-resolve-test".into()),
        parent_state_root: Hash::ZERO,
        task_id: TaskId("task-digest".into()),
        epoch: SystemEpoch::new(1),
        timestamp_logical: 1,
        system_signature: SystemSignature::from_bytes([0u8; 64]),
        outcome: OutcomeSide::Yes,
    };
    let mut mutated = base.clone();
    mutated.outcome = OutcomeSide::No;
    assert_ne!(
        base.to_signing_payload().canonical_digest(),
        mutated.to_signing_payload().canonical_digest(),
        "mutating EventResolve outcome must change canonical signing payload digest"
    );
}

#[test]
fn sg_6a_event_resolve_legacy_b2_wire_decodes_as_yes() {
    let legacy = LegacyTypedTxWire::EventResolve(LegacyEventResolveTxWire {
        tx_id: TxId("system-event-resolve-legacy".into()),
        parent_state_root: Hash::ZERO,
        task_id: TaskId("task-legacy".into()),
        epoch: SystemEpoch::new(1),
        timestamp_logical: 9,
        system_signature: SystemSignature::from_bytes([7u8; 64]),
    });
    let bytes = canonical_encode(&legacy).expect("legacy encode");
    let decoded: TypedTx =
        canonical_decode(&bytes).expect("REAL-6A must grandfather TB-N2 B2 EventResolveTx bytes");
    let TypedTx::EventResolve(er) = decoded else {
        panic!("legacy bytes must decode as EventResolve");
    };
    assert_eq!(er.tx_id.0, "system-event-resolve-legacy");
    assert_eq!(er.task_id.0, "task-legacy");
    assert_eq!(er.epoch, SystemEpoch::new(1));
    assert_eq!(er.timestamp_logical, 9);
    assert_eq!(er.outcome, OutcomeSide::Yes);
}

#[test]
fn sg_6a_event_resolve_corrupt_outcome_tail_fails_closed() {
    let current = TypedTx::EventResolve(EventResolveTx {
        tx_id: TxId("system-event-resolve-corrupt-tail".into()),
        parent_state_root: Hash::ZERO,
        task_id: TaskId("task-corrupt-tail".into()),
        epoch: SystemEpoch::new(1),
        timestamp_logical: 9,
        system_signature: SystemSignature::from_bytes([7u8; 64]),
        outcome: OutcomeSide::Yes,
    });
    let mut bytes = canonical_encode(&current).expect("current EventResolve encode");
    let last = bytes.last_mut().expect("encoded bytes non-empty");
    *last = last.wrapping_add(9);

    let decoded: Result<TypedTx, _> = canonical_decode(&bytes);
    assert!(
        decoded.is_err(),
        "only missing legacy outcome may default to YES; malformed outcome tails must fail closed"
    );
}

#[test]
fn sg_6a_event_resolve_partial_outcome_tail_fails_closed() {
    let current = TypedTx::EventResolve(EventResolveTx {
        tx_id: TxId("system-event-resolve-partial-tail".into()),
        parent_state_root: Hash::ZERO,
        task_id: TaskId("task-partial-tail".into()),
        epoch: SystemEpoch::new(1),
        timestamp_logical: 9,
        system_signature: SystemSignature::from_bytes([7u8; 64]),
        outcome: OutcomeSide::No,
    });
    let mut bytes = canonical_encode(&current).expect("current EventResolve encode");
    bytes.pop();

    let decoded: Result<TypedTx, _> = canonical_decode(&bytes);
    assert!(
        decoded.is_err(),
        "partial current EventResolve outcome tails must not be grandfathered as legacy YES"
    );
}

#[test]
fn sg_6a_event_resolve_compatibility_is_exact_dual_reader_not_error_string() {
    let typed_tx_src = std::fs::read_to_string("src/state/typed_tx.rs").expect("read typed_tx");
    assert!(
        !typed_tx_src.contains("event_resolve_outcome_tail_is_missing_legacy_bincode")
            && !typed_tx_src.contains("message.contains(\"unexpected"),
        "EventResolve compatibility must not classify malformed wire bytes by error-string matching"
    );
    let ledger_src = std::fs::read_to_string("src/bottom_white/ledger/transition_ledger.rs")
        .expect("read transition_ledger");
    assert!(
        ledger_src.contains("LegacyTypedTxEventResolveWire")
            && ledger_src.contains("canonical_decode_typed_tx_current_or_legacy_event_resolve")
            && ledger_src.contains("consumed != bytes.len()"),
        "legacy EventResolve compatibility must be an exact dual-reader at canonical decode"
    );
}


#[test]
fn sg_6a_2_trader_view_contains_task_outcome_market_signal() {
    let event = task_outcome_event_for_task(
        "evt-task-view",
        TaskId("task-view".into()),
        12,
        MicroCoin::from_micro_units(1_000_000),
        TxId("taskopen-view".into()),
    );
    let signal = task_outcome_price_signal(&event, "1/2", Some(200_000));
    let view = derive_role_view(
        DerivedViewRequest {
            agent_id: AgentId("Trader_0".into()),
            role: AgentRole::Trader,
            task_id: TaskId("task-view".into()),
            head_t: "head".into(),
        },
        DerivedViewInput {
            read_set: vec![Cid::default()],
            price_signals: vec![signal.clone()],
            local_errors: vec![],
        },
    )
    .expect("derive trader view");
    assert!(
        view.price_signals
            .iter()
            .any(|s| s.event_id == signal.event_id),
        "TraderView must contain active TaskOutcomeMarket price signal"
    );
    assert_eq!(
        signal.depth,
        Some(200_000),
        "TraderView must expose pool depth"
    );
    assert_eq!(
        event.deadline_round, 12,
        "TaskOutcomeMarket view evidence must preserve deadline/budget context"
    );
    assert_eq!(
        event.max_budget.micro_units(),
        1_000_000,
        "TaskOutcomeMarket view evidence must preserve max_budget"
    );
    for required in ["pool depth", "PnL", "balance", "recent accepted WorkTx"] {
        assert!(
            view.public_sections
                .iter()
                .any(|section| section == required),
            "TraderView public sections must include {required}"
        );
    }
}


#[test]
fn sg_6a_smoke_runner_build_failure_is_fail_closed() {
    let script =
        std::fs::read_to_string("scripts/run_g_phase_batch.sh").expect("read run_g_phase_batch.sh");
    let build_marker = "cargo build --release --manifest-path experiments/minif2f_v4/Cargo.toml --bin evaluator --bin batch_evaluator; cargo build --release -p turingosv4 --bin audit_tape --bin tb_g_persistence_report";
    let build_pos = script
        .find(build_marker)
        .expect("release build command is present");
    let batch_pos = script
        .find("[batch_evaluator] launching")
        .expect("batch launch is present");
    let build_block = &script[build_pos..batch_pos];
    assert!(
        build_block.contains("if !")
            && build_block.contains("exit 6")
            && build_block.contains("release binary build failed"),
        "REAL-6A smoke evidence must fail closed on cargo build failure, not continue with stale release binaries"
    );
}


#[test]
fn sg_6a_runner_clears_only_stale_cas_lock_before_post_run_audit() {
    let script =
        std::fs::read_to_string("scripts/run_g_phase_batch.sh").expect("read run_g_phase_batch.sh");
    let batch_exit_pos = script
        .find("BATCH_EXIT=${PIPESTATUS[0]}")
        .expect("batch exit capture exists");
    let audit_pos = script
        .find("[audit_tape] running over shared runtime_repo + cas")
        .expect("post-run audit_tape section exists");
    assert!(
        batch_exit_pos < audit_pos,
        "batch_evaluator must exit before post-run audit begins"
    );

    let cleanup_block = &script[batch_exit_pos..audit_pos];
    assert!(
        cleanup_block.contains("clear_stale_cas_chain_lock \"$RUN_DIR/cas\""),
        "runner must clear stale CAS chain lock after batch exit and before audit_tape"
    );
    assert!(
        cleanup_block.contains(".turingos_cas_chain.lock")
            && cleanup_block.contains("cas_stale_lock_cleanup.log"),
        "cleanup must be exact-path and auditable"
    );
    assert!(
        cleanup_block.contains("pid=") && cleanup_block.contains("kill -0"),
        "cleanup must preserve live CAS chain locks by checking the recorded pid"
    );
    assert!(
        !cleanup_block.contains(".turingos_cas_index.jsonl")
            && !cleanup_block.contains("rm -rf")
            && !cleanup_block.contains("runtime_repo"),
        "cleanup must not delete or rewrite authoritative CAS/ChainTape evidence"
    );

    let exit_block = script
        .split("# Exit non-zero on any failure mode")
        .nth(1)
        .expect("final fail-closed exit block exists");
    assert!(
        exit_block.contains("if [[ \"$BATCH_EXIT\" -ne 0 ]]")
            && exit_block.contains("exit \"$BATCH_EXIT\""),
        "runner must not let post-cleanup audit output suppress a non-zero batch exit"
    );
}
