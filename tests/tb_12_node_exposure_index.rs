//! TB-12 Atom 3 + Atom 5 integration tests — Node exposure index per
//! architect 2026-05-03 ruling §3 + §8 Atom 3 + §9.3 SG-12.1..8.
//!
//! "Node exposure index" — NOT trading market. NodePosition is an
//! IMMUTABLE EXPOSURE RECORD per architect §10. NO trading / NO price /
//! NO settlement / NO close / NO transfer in TB-12.
//!
//! Coverage maps to architect SG-12.1..8 + halting triggers from
//! charter §7 (CTF conservation / position-tx mismatch / coin-counting /
//! replay divergence / VETO).
//!
//! - SG-12.1 accepted_worktx_creates_firstlong_position
//! - SG-12.2 accepted_challengetx_creates_challengeshort_position
//! - SG-12.3 verifytx_does_not_create_node_position
//! - SG-12.4 node_positions_do_not_change_total_supply
//! - SG-12.5 replay_reconstructs_node_positions
//! - SG-12.6 dashboard_view_positions_works (deferred to Atom 4)
//! - SG-12.7 no_market_trading_variants_introduced (compile-time + grep)
//! - SG-12.8 no_node_market_entry_as_canonical_state (covered in
//!           q_state.rs unit tests; this file additionally asserts
//!           runtime QState shape).
//!
//! /// TRACE_MATRIX TB-12 Atom 3 + Atom 5 (architect 2026-05-03 ruling §8
//! Atom 3 + Atom 5; SG-12.1..8).

use std::collections::{BTreeMap, BTreeSet};
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
use turingosv4::economy::money::{MicroCoin, StakeMicroCoin};
use turingosv4::economy::monetary_invariant::{
    assert_no_post_init_mint, assert_total_ctf_conserved,
};
use turingosv4::state::q_state::{
    AgentId, EscrowEntry, Hash, NodePositionsIndex, QState, TaskId, TxId,
};
use turingosv4::state::sequencer::{Sequencer, SubmissionEnvelope};
use turingosv4::state::typed_tx::{
    AgentSignature, BoolWithProof, ChallengeTx, EscrowLockTx, PositionKind, PositionSide,
    PredicateId, PredicateResultsBundle, ReadKey, SafetyOrCreation, TaskOpenTx,
    TypedTx, VerifyTx, VerifyVerdict, WorkTx, WriteKey,
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

async fn open_task(h: &mut Harness, sponsor: &str, task: &str) {
    let parent = h.seq.q_snapshot().unwrap().state_root_t;
    let tx = TypedTx::TaskOpen(TaskOpenTx {
        tx_id: TxId(format!("taskopen-{task}")),
        task_id: TaskId(task.into()),
        parent_state_root: parent,
        sponsor_agent: AgentId(sponsor.into()),
        verifier_quorum: 1,
        max_reuse_royalty_fraction_basis_points: 1000,
        settlement_rule_hash: Hash::ZERO,
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: 1,
    });
    h.seq.submit_agent_tx(tx).await.expect("submit open");
    let _ = h.seq.try_apply_one(&mut h.rx).expect("env").expect("ok");
}

async fn lock_escrow(h: &mut Harness, sponsor: &str, task: &str, micro: i64) {
    let parent = h.seq.q_snapshot().unwrap().state_root_t;
    let tx = TypedTx::EscrowLock(EscrowLockTx {
        tx_id: TxId(format!("lock-{task}")),
        task_id: TaskId(task.into()),
        parent_state_root: parent,
        sponsor_agent: AgentId(sponsor.into()),
        amount: MicroCoin::from_micro_units(micro),
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: 2,
    });
    h.seq.submit_agent_tx(tx).await.expect("submit lock");
    let _ = h.seq.try_apply_one(&mut h.rx).expect("env").expect("ok");
}

async fn submit_work(
    h: &mut Harness,
    task: &str,
    agent: &str,
    stake_micro: i64,
    suffix: &str,
) -> TxId {
    let parent = h.seq.q_snapshot().unwrap().state_root_t;
    let tx_id = TxId(format!("work-{task}-{suffix}"));
    let mut acceptance = BTreeMap::new();
    acceptance.insert(
        PredicateId("acc1".into()),
        BoolWithProof { value: true, proof_cid: None },
    );
    let work = WorkTx {
        tx_id: tx_id.clone(),
        task_id: TaskId(task.into()),
        parent_state_root: parent,
        agent_id: AgentId(agent.into()),
        read_set: [ReadKey("k.r".into())].into_iter().collect::<BTreeSet<_>>(),
        write_set: [WriteKey("k.w".into())].into_iter().collect::<BTreeSet<_>>(),
        proposal_cid: Default::default(),
        predicate_results: PredicateResultsBundle {
            acceptance, settlement: BTreeMap::new(),
            safety_class: SafetyOrCreation::Safety,
        },
        stake: StakeMicroCoin::from_micro_units(stake_micro),
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: 3,
    };
    h.seq.submit_agent_tx(TypedTx::Work(work)).await.expect("submit work");
    let _ = h.seq.try_apply_one(&mut h.rx).expect("env").expect("ok work");
    tx_id
}

async fn submit_challenge(
    h: &mut Harness,
    challenger: &str,
    target_work_tx: &TxId,
    stake_micro: i64,
    suffix: &str,
) -> TxId {
    let parent = h.seq.q_snapshot().unwrap().state_root_t;
    let tx_id = TxId(format!("challenge-{suffix}"));
    let tx = TypedTx::Challenge(ChallengeTx {
        tx_id: tx_id.clone(),
        parent_state_root: parent,
        target_work_tx: target_work_tx.clone(),
        challenger_agent: AgentId(challenger.into()),
        stake: StakeMicroCoin::from_micro_units(stake_micro),
        counterexample_cid: turingosv4::bottom_white::cas::schema::Cid([0xabu8; 32]),
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: 4,
    });
    h.seq.submit_agent_tx(tx).await.expect("submit challenge");
    let _ = h.seq.try_apply_one(&mut h.rx).expect("env").expect("ok challenge");
    tx_id
}

async fn submit_verify(
    h: &mut Harness,
    verifier: &str,
    target_work_tx: &TxId,
    bond_micro: i64,
    suffix: &str,
) -> TxId {
    let parent = h.seq.q_snapshot().unwrap().state_root_t;
    let tx_id = TxId(format!("verify-{suffix}"));
    let tx = TypedTx::Verify(VerifyTx {
        tx_id: tx_id.clone(),
        parent_state_root: parent,
        target_work_tx: target_work_tx.clone(),
        verifier_agent: AgentId(verifier.into()),
        bond: StakeMicroCoin::from_micro_units(bond_micro),
        verdict: VerifyVerdict::Confirm,
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: 5,
    });
    h.seq.submit_agent_tx(tx).await.expect("submit verify");
    let _ = h.seq.try_apply_one(&mut h.rx).expect("env").expect("ok verify");
    tx_id
}

// ── SG-12.1 — accepted_worktx_creates_firstlong_position ────────────────────

#[tokio::test]
async fn sg_12_1_accepted_worktx_creates_firstlong_position() {
    let q = genesis_with_balances(&[("sponsor-A", 10), ("solver-A", 10)]);
    let mut h = fresh_harness(q);
    open_task(&mut h, "sponsor-A", "task-A").await;
    lock_escrow(&mut h, "sponsor-A", "task-A", 200_000).await;
    let work_tx_id = submit_work(&mut h, "task-A", "solver-A", 50_000, "1").await;

    let q_after = h.seq.q_snapshot().unwrap();
    let positions = &q_after.economic_state_t.node_positions_t.0;
    assert_eq!(positions.len(), 1, "exactly 1 NodePosition created");
    let pos = positions
        .get(&work_tx_id)
        .expect("FirstLong NodePosition keyed by work_tx_id");
    assert_eq!(pos.kind, PositionKind::FirstLong);
    assert_eq!(pos.side, PositionSide::Long);
    assert_eq!(pos.node_id, work_tx_id);
    assert_eq!(pos.source_tx, work_tx_id);
    assert_eq!(pos.position_id, work_tx_id);
    assert_eq!(pos.owner, AgentId("solver-A".into()));
    assert_eq!(pos.amount, MicroCoin::from_micro_units(50_000));
    assert_eq!(pos.task_id, TaskId("task-A".into()));
}

// ── SG-12.2 — accepted_challengetx_creates_challengeshort_position ──────────

#[tokio::test]
async fn sg_12_2_accepted_challengetx_creates_challengeshort_position() {
    let q = genesis_with_balances(&[
        ("sponsor-B", 10),
        ("solver-B", 10),
        ("challenger-B", 10),
    ]);
    let mut h = fresh_harness(q);
    open_task(&mut h, "sponsor-B", "task-B").await;
    lock_escrow(&mut h, "sponsor-B", "task-B", 200_000).await;
    let work_tx_id = submit_work(&mut h, "task-B", "solver-B", 30_000, "1").await;
    let chal_tx_id =
        submit_challenge(&mut h, "challenger-B", &work_tx_id, 20_000, "1").await;

    let q_after = h.seq.q_snapshot().unwrap();
    let positions = &q_after.economic_state_t.node_positions_t.0;
    assert_eq!(positions.len(), 2, "FirstLong + ChallengeShort = 2");

    let short = positions.get(&chal_tx_id).expect("ChallengeShort keyed by challenge tx_id");
    assert_eq!(short.kind, PositionKind::ChallengeShort);
    assert_eq!(short.side, PositionSide::Short);
    assert_eq!(short.node_id, work_tx_id, "FR-12.5 node_id == challenge.target_work_tx");
    assert_eq!(short.source_tx, chal_tx_id);
    assert_eq!(short.position_id, chal_tx_id);
    assert_eq!(short.owner, AgentId("challenger-B".into()));
    assert_eq!(short.amount, MicroCoin::from_micro_units(20_000));
    assert_eq!(short.task_id, TaskId("task-B".into()));
}

// ── SG-12.3 — verifytx_does_not_create_node_position ────────────────────────

#[tokio::test]
async fn sg_12_3_verifytx_does_not_create_node_position() {
    let q = genesis_with_balances(&[
        ("sponsor-C", 10),
        ("solver-C", 10),
        ("verifier-C", 10),
    ]);
    let mut h = fresh_harness(q);
    open_task(&mut h, "sponsor-C", "task-C").await;
    lock_escrow(&mut h, "sponsor-C", "task-C", 200_000).await;
    let work_tx_id = submit_work(&mut h, "task-C", "solver-C", 30_000, "1").await;
    let positions_before = h
        .seq
        .q_snapshot()
        .unwrap()
        .economic_state_t
        .node_positions_t
        .0
        .len();
    assert_eq!(positions_before, 1, "FirstLong from WorkTx");

    let _verify_tx_id = submit_verify(&mut h, "verifier-C", &work_tx_id, 5_000, "1").await;

    let q_after = h.seq.q_snapshot().unwrap();
    let positions = &q_after.economic_state_t.node_positions_t.0;
    assert_eq!(
        positions.len(),
        1,
        "VerifyTx must NOT create a NodePosition (FR-12.3 + CR-12.8)"
    );
    // Sanity: existing FirstLong unchanged.
    let long = positions.get(&work_tx_id).unwrap();
    assert_eq!(long.kind, PositionKind::FirstLong);
}

// ── SG-12.4 — node_positions_do_not_change_total_supply ─────────────────────

#[tokio::test]
async fn sg_12_4_node_positions_do_not_change_total_supply() {
    let q_initial = genesis_with_balances(&[
        ("sponsor-D", 10),
        ("solver-D", 10),
        ("challenger-D", 10),
    ]);
    let mut h = fresh_harness(q_initial.clone());
    open_task(&mut h, "sponsor-D", "task-D").await;
    lock_escrow(&mut h, "sponsor-D", "task-D", 200_000).await;
    let work_tx_id = submit_work(&mut h, "task-D", "solver-D", 50_000, "1").await;
    let _chal = submit_challenge(&mut h, "challenger-D", &work_tx_id, 20_000, "1").await;

    let q_after = h.seq.q_snapshot().unwrap();

    // CR-12.2 enforcement via assert_total_ctf_conserved: the 5-holding sum
    // is invariant across all economic mutations on the path
    // initial→…→q_after. assert_total_ctf_conserved already takes care of
    // verifying the holding-sum equality; if NodePosition.amount were
    // accidentally counted as a holding, the sum would diverge.
    assert_total_ctf_conserved(
        &q_initial.economic_state_t,
        &q_after.economic_state_t,
        &[],
    )
    .expect("CR-12.2: total_supply_micro MUST be invariant across NodePosition derivation");

    // Sanity: positions exist (long + short).
    assert_eq!(q_after.economic_state_t.node_positions_t.0.len(), 2);
}

// ── SG-12.5 — replay_reconstructs_node_positions (architect §9.3 exact name) ─

#[tokio::test]
async fn sg_12_5_replay_reconstructs_node_positions() {
    // Two harnesses with identical inputs MUST produce identical
    // node_positions_t. NodePosition is derived from typed-tx fields with
    // no environmental input; replay-deterministic by construction.
    async fn run(label: &str) -> NodePositionsIndex {
        let q = genesis_with_balances(&[
            (&format!("sponsor-{label}"), 10),
            (&format!("solver-{label}"), 10),
            (&format!("challenger-{label}"), 10),
        ]);
        let mut h = fresh_harness(q);
        open_task(&mut h, &format!("sponsor-{label}"), "task-Z").await;
        lock_escrow(&mut h, &format!("sponsor-{label}"), "task-Z", 200_000).await;
        let work_tx_id =
            submit_work(&mut h, "task-Z", &format!("solver-{label}"), 50_000, "1").await;
        let _chal =
            submit_challenge(&mut h, &format!("challenger-{label}"), &work_tx_id, 20_000, "1")
                .await;
        h.seq
            .q_snapshot()
            .unwrap()
            .economic_state_t
            .node_positions_t
            .clone()
    }
    // Two distinct harnesses, identical agent name suffixes ensure identical
    // tx_ids and therefore identical NodePositions.
    let a = run("X").await;
    let b = run("X").await;
    assert_eq!(a, b, "node_positions_t must be replay-deterministic");
}

// ── SG-12.7 — no_market_trading_variants_introduced (architect §9.3 exact name) ─

/// SG-12.7 enforces at runtime that the only PositionKind values used in
/// production dispatch are FirstLong + ChallengeShort. Future MarketBuy /
/// MarketSell variants would require explicit charter ratification.
/// Compile-time + grep audit additionally enforces the absence of Market*
/// trading typed_tx variants.
#[tokio::test]
async fn sg_12_7_no_market_trading_variants_introduced() {
    let q = genesis_with_balances(&[
        ("sponsor-E", 10),
        ("solver-E", 10),
        ("challenger-E", 10),
    ]);
    let mut h = fresh_harness(q);
    open_task(&mut h, "sponsor-E", "task-E").await;
    lock_escrow(&mut h, "sponsor-E", "task-E", 200_000).await;
    let work_tx_id = submit_work(&mut h, "task-E", "solver-E", 30_000, "1").await;
    let _chal = submit_challenge(&mut h, "challenger-E", &work_tx_id, 20_000, "1").await;

    let q_after = h.seq.q_snapshot().unwrap();
    for (_pid, position) in q_after.economic_state_t.node_positions_t.0.iter() {
        match position.kind {
            PositionKind::FirstLong | PositionKind::ChallengeShort => {}
            // Unreachable in TB-12 — schema only ships these two.
            // If a future variant lands without charter ratification, this
            // pattern catches it at the test boundary.
        }
        // Side discipline.
        match (position.kind, position.side) {
            (PositionKind::FirstLong, PositionSide::Long) => {}
            (PositionKind::ChallengeShort, PositionSide::Short) => {}
            (k, s) => panic!(
                "PositionKind::{k:?} paired with PositionSide::{s:?} — \
                 architectural invariant violation"
            ),
        }
    }
}

// Note: a "WorkTx with stake==0 → no NodePosition" negative test is
// architecturally redundant — TB-3's WorkTx accept arm rejects
// `stake.micro_units() == 0` upstream with Transition(StakeInsufficient)
// before the NodePosition derivation runs. The Atom 2 gate
// `if work.stake.micro_units() > 0` is therefore defense-in-depth
// (graceful degradation if upstream checks ever weaken). FR-12.1 "with
// stake" clause is satisfied by the upstream gate; the additional gate
// in the NodePosition write site protects against future regressions.

// ── Halting trigger: position fields match source_tx fields exactly ──────────

/// Architect §7 halting trigger: WorkTx/ChallengeTx position mismatch
/// MUST stop execution immediately. This test locks the field-derivation
/// rules.
#[tokio::test]
async fn position_fields_derived_from_source_tx_exactly() {
    let q = genesis_with_balances(&[
        ("sponsor-G", 10),
        ("solver-G", 10),
        ("challenger-G", 10),
    ]);
    let mut h = fresh_harness(q);
    open_task(&mut h, "sponsor-G", "task-G").await;
    lock_escrow(&mut h, "sponsor-G", "task-G", 200_000).await;
    let work_tx_id = submit_work(&mut h, "task-G", "solver-G", 75_000, "1").await;
    let chal_tx_id =
        submit_challenge(&mut h, "challenger-G", &work_tx_id, 33_000, "1").await;

    let q_after = h.seq.q_snapshot().unwrap();

    // FirstLong invariant chain: position_id == source_tx == node_id == work.tx_id.
    let long = q_after
        .economic_state_t
        .node_positions_t
        .0
        .get(&work_tx_id)
        .unwrap();
    assert_eq!(long.position_id, work_tx_id);
    assert_eq!(long.source_tx, work_tx_id);
    assert_eq!(long.node_id, work_tx_id);
    assert_eq!(long.amount.micro_units(), 75_000);

    // ChallengeShort invariant chain: position_id == source_tx == challenge.tx_id;
    // node_id == challenge.target_work_tx (NOT challenge's own tx_id).
    let short = q_after
        .economic_state_t
        .node_positions_t
        .0
        .get(&chal_tx_id)
        .unwrap();
    assert_eq!(short.position_id, chal_tx_id);
    assert_eq!(short.source_tx, chal_tx_id);
    assert_eq!(short.node_id, work_tx_id, "ChallengeShort node_id targets the WorkTx");
    assert_ne!(short.node_id, chal_tx_id, "ChallengeShort node_id is NOT its own tx_id");
    assert_eq!(short.amount.micro_units(), 33_000);
}

// ── Ensure NodePosition does not break existing CTF assertions ──────────────

#[tokio::test]
async fn ctf_invariant_unchanged_across_position_derivation() {
    let q = genesis_with_balances(&[
        ("sponsor-H", 10),
        ("solver-H", 10),
        ("challenger-H", 10),
    ]);
    let mut h = fresh_harness(q.clone());
    open_task(&mut h, "sponsor-H", "task-H").await;
    lock_escrow(&mut h, "sponsor-H", "task-H", 200_000).await;
    let work_tx_id = submit_work(&mut h, "task-H", "solver-H", 50_000, "1").await;
    let _chal = submit_challenge(&mut h, "challenger-H", &work_tx_id, 20_000, "1").await;

    let q_after = h.seq.q_snapshot().unwrap();

    // assert_total_ctf_conserved between genesis and post-positions QState.
    // Both must satisfy the 5-holding sum invariant; NodePosition not in sum.
    assert_total_ctf_conserved(&q.economic_state_t, &q_after.economic_state_t, &[])
        .expect("CTF preserved across NodePosition derivation");

    // assert_no_post_init_mint on a synthetic future tx (use a TaskOpen on the
    // current state; should pass — TaskOpen doesn't mint).
    let dummy = TypedTx::TaskOpen(TaskOpenTx {
        tx_id: TxId("dummy".into()),
        task_id: TaskId("dummy-task".into()),
        parent_state_root: Hash::ZERO,
        sponsor_agent: AgentId("sponsor-H".into()),
        verifier_quorum: 1,
        max_reuse_royalty_fraction_basis_points: 1000,
        settlement_rule_hash: Hash::ZERO,
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: 1,
    });
    assert_no_post_init_mint(&dummy, &q_after).expect("no mint");
}

// ── SG-12.8 — no_node_market_entry_as_canonical_state (architect §9.3 exact name) ─

/// SG-12.8 (architect 2026-05-03 ruling §3 + §9.3): the EconomicState MUST
/// NOT have `node_market_t` as a canonical sub-field. NodeMarketEntry is
/// TB-14 derived view, not TB-12 canonical state. This test mirrors the
/// q_state.rs unit test but lives at the architect-mandated SG-12.8 name.
#[test]
fn sg_12_8_no_node_market_entry_as_canonical_state() {
    let q = QState::genesis();
    let s = serde_json::to_value(&q.economic_state_t).unwrap();
    let obj = s.as_object().unwrap();
    assert!(
        !obj.contains_key("node_market_t"),
        "TB-12 architect §3 ruling: node_market_t MUST NOT be a canonical \
         EconomicState field. NodeMarketEntry is TB-14 derived view only."
    );
    // Positive: node_positions_t IS the canonical TB-12 sub-field.
    assert!(
        obj.contains_key("node_positions_t"),
        "TB-12 canonical state: node_positions_t (flat NodePositionsIndex)"
    );
}

#[allow(dead_code)]
fn _suppress_unused() -> Vec<EscrowEntry> {
    vec![]
}
