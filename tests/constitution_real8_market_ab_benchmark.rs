//! REAL-8 Formal Market A/B Benchmark gates.
//!
//! These tests pin the runner contract before any benchmark evidence can be
//! claimed. The runner itself performs real ChainTape/CAS runs.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::sync::{Arc, RwLock};

use tempfile::TempDir;

use turingosv4::bottom_white::cas::schema::Cid;
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::bottom_white::ledger::rejection_evidence::RejectionEvidenceWriter;
use turingosv4::bottom_white::ledger::system_keypair::{
    Ed25519Keypair, PinnedSystemPubkeys, SystemEpoch,
};
use turingosv4::bottom_white::ledger::transition_ledger::{InMemoryLedgerWriter, LedgerWriter};
use turingosv4::bottom_white::tools::registry::ToolRegistry;
use turingosv4::economy::money::{MicroCoin, StakeMicroCoin};
use turingosv4::state::q_state::{AgentId, Hash, QState, TaskId, TxId};
use turingosv4::state::sequencer::{ApplyError, Sequencer, SubmissionEnvelope};
use turingosv4::state::typed_tx::{
    AgentSignature, BoolWithProof, CompleteSetMintTx, EventId, PredicateId, PredicateResultsBundle,
    ReadKey, SafetyOrCreation, TaskOpenTx, TypedTx, VerifyTx, VerifyVerdict, WorkTx, WriteKey,
};
use turingosv4::state::TransitionError;
use turingosv4::top_white::predicates::registry::PredicateRegistry;

struct Harness {
    _tmp: TempDir,
    seq: Sequencer,
    rx: tokio::sync::mpsc::Receiver<SubmissionEnvelope>,
    _ledger: Arc<RwLock<dyn LedgerWriter>>,
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

async fn submit_and_apply(h: &mut Harness, tx: TypedTx) -> Result<(), ApplyError> {
    h.seq.submit_agent_tx(tx).await.expect("submit tx");
    h.seq
        .try_apply_one(&mut h.rx)
        .expect("submitted envelope drained")
        .map(|_| ())
}

fn genesis_with_balances() -> QState {
    let mut q = QState::genesis();
    for agent in ["sponsor", "solver", "verifier", "market-maker"] {
        q.economic_state_t.balances_t.0.insert(
            AgentId(agent.into()),
            MicroCoin::from_coin(10).expect("coin"),
        );
    }
    q
}

fn task_open(parent: Hash, task: &str) -> TypedTx {
    TypedTx::TaskOpen(TaskOpenTx {
        tx_id: TxId(format!("real8x-open-{task}")),
        task_id: TaskId(task.into()),
        parent_state_root: parent,
        sponsor_agent: AgentId("sponsor".into()),
        verifier_quorum: 1,
        max_reuse_royalty_fraction_basis_points: 1000,
        settlement_rule_hash: Hash::ZERO,
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: 1,
    })
}

fn escrow_lock(parent: Hash, task: &str) -> TypedTx {
    TypedTx::EscrowLock(turingosv4::state::typed_tx::EscrowLockTx {
        tx_id: TxId(format!("real8x-lock-{task}")),
        task_id: TaskId(task.into()),
        parent_state_root: parent,
        sponsor_agent: AgentId("sponsor".into()),
        amount: MicroCoin::from_micro_units(3_000_000),
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: 2,
    })
}

fn work_tx(parent: Hash, task: &str) -> TypedTx {
    let mut acceptance = BTreeMap::new();
    acceptance.insert(
        PredicateId("lean-verified".into()),
        BoolWithProof {
            value: true,
            proof_cid: None,
        },
    );
    TypedTx::Work(WorkTx {
        tx_id: TxId("real8x-work".into()),
        task_id: TaskId(task.into()),
        parent_state_root: parent,
        agent_id: AgentId("solver".into()),
        read_set: [ReadKey("goal".into())]
            .into_iter()
            .collect::<BTreeSet<_>>(),
        write_set: [WriteKey("proof".into())]
            .into_iter()
            .collect::<BTreeSet<_>>(),
        proposal_cid: Cid([1u8; 32]),
        predicate_results: PredicateResultsBundle {
            acceptance,
            settlement: BTreeMap::new(),
            safety_class: SafetyOrCreation::Safety,
        },
        stake: StakeMicroCoin::from_micro_units(1_000_000),
        signature: AgentSignature::from_bytes([1u8; 64]),
        timestamp_logical: 3,
    })
}

fn market_side_state_mutation(parent: Hash, task: &str) -> TypedTx {
    TypedTx::CompleteSetMint(CompleteSetMintTx {
        tx_id: TxId("real8x-market-side-mint".into()),
        parent_state_root: parent,
        event_id: EventId(TaskId(task.into())),
        owner: AgentId("market-maker".into()),
        amount: MicroCoin::from_micro_units(500_000),
        signature: AgentSignature::from_bytes([2u8; 64]),
        timestamp_logical: 4,
    })
}

fn verify_tx(parent: Hash, tx_id: &str) -> TypedTx {
    TypedTx::Verify(VerifyTx {
        tx_id: TxId(tx_id.into()),
        parent_state_root: parent,
        target_work_tx: TxId("real8x-work".into()),
        verifier_agent: AgentId("verifier".into()),
        bond: StakeMicroCoin::from_micro_units(500_000),
        verdict: VerifyVerdict::Confirm,
        signature: AgentSignature::from_bytes([3u8; 64]),
        timestamp_logical: 5,
    })
}

#[test]
fn real8_runner_preserves_architect_ab_arms() {
    let script = fs::read_to_string("scripts/run_real8_market_ab_benchmark.sh")
        .expect("REAL-8 runner exists");

    for expected in [
        "A: market disabled",
        "B: market visible, no TaskOutcomeMarket",
        "C: TaskOutcomeMarket enabled",
        "D: TaskOutcomeMarket + scripted AttemptPrediction fixture",
    ] {
        assert!(
            script.contains(expected),
            "REAL-8 runner must preserve architect arm text: {expected}"
        );
    }
}

#[test]
fn real8_runner_pins_same_problem_model_and_budget_inputs() {
    let script = fs::read_to_string("scripts/run_real8_market_ab_benchmark.sh")
        .expect("REAL-8 runner exists");

    for expected in [
        "--problems <same_problem_set_manifest>",
        "--models <same_model_assignment_manifest>",
        "--budgets <same_budget_manifest>",
        "--tasks-per-arm <N>",
        "same problem set",
        "same model assignment",
        "same budgets",
        "Same seed/config except arm toggles",
        "PROBLEMS_HASH",
        "MODELS_HASH",
        "BUDGETS_HASH",
        "REAL8X_SHARED_CONFIG_SHA256",
        "REAL8X_CONFIG_AUDIT.json",
        "arm_config_manifests",
        "arm_diff_allowlist",
        "arm_config_diff_from_A.tsv",
    ] {
        assert!(
            script.contains(expected),
            "REAL-8 runner must pin shared benchmark input: {expected}"
        );
    }
}

#[test]
fn real8x_runner_enforces_arm_diff_allowlist() {
    let script = fs::read_to_string("scripts/run_real8_market_ab_benchmark.sh")
        .expect("REAL-8 runner exists");

    for expected in [
        "validate_arm_config_diffs",
        "allowed = set(",
        "disallowed_config_drift",
        "Only arm toggles may differ",
        "TURINGOS_TB_N3_AUTO_MARKET",
        "TURINGOS_REAL6_TASK_OUTCOME_MARKET",
        "TURINGOS_REAL7_SCRIPTED_ATTEMPT_PREDICTION_FIXTURE",
    ] {
        assert!(
            script.contains(expected),
            "REAL-8X runner must fail closed on non-allowlisted arm config drift: {expected}"
        );
    }
}

#[test]
fn real8_runner_keeps_live_real6b_gated_behind_separate_ratification() {
    let script = fs::read_to_string("scripts/run_real8_market_ab_benchmark.sh")
        .expect("REAL-8 runner exists");

    for expected in [
        "TURINGOS_REAL6B_LIVE_ATTEMPT_PREDICTION",
        "live REAL-6B AttemptPrediction is not ratified for REAL-8/REAL-10",
        "use the scripted fixture only",
        "D: TaskOutcomeMarket + scripted AttemptPrediction fixture",
    ] {
        assert!(
            script.contains(expected),
            "REAL-8/REAL-10 runner must preserve live REAL-6B boundary: {expected}"
        );
    }
}

#[test]
fn real8_runner_reports_required_metrics() {
    let script = fs::read_to_string("scripts/run_real8_market_ab_benchmark.sh")
        .expect("REAL-8 runner exists");

    for expected in [
        "solve_rate",
        "wilson_ci_95",
        "verified_pput_mean",
        "mean_pput_solved",
        "false_accept_rate_mean",
        "cost_per_verified_proof_tokens",
        "cost_time_tokens_ms",
        "market_tx_count",
        "no_trade_reason_distribution",
        "pnl_dispersion_micro",
        "role_diversity_index",
        "failed_branch_count",
        "verification_latency_ms_mean",
        "wasted_attempts",
        "audit_failure_rate",
    ] {
        assert!(
            script.contains(expected),
            "REAL-8 runner must report architect metric: {expected}"
        );
    }
}

#[test]
fn real8_runner_keeps_negative_results_and_forbids_causal_overclaim() {
    let script = fs::read_to_string("scripts/run_real8_market_ab_benchmark.sh")
        .expect("REAL-8 runner exists");

    assert!(
        script.contains(
            "This report is descriptive benchmark evidence only. It does not claim causality."
        ),
        "REAL-8 report must explicitly avoid causal overclaim"
    );
    assert!(
        script.contains("Negative result is valid and documented."),
        "REAL-8 report must preserve negative results as valid evidence"
    );
    assert!(
        script.contains("undefined_no_verified_proof"),
        "REAL-8 report must retain no-verified-proof outcomes instead of fabricating cost metrics"
    );
}

#[test]
fn real8_runner_preserves_forbidden_ship_claims() {
    let script = fs::read_to_string("scripts/run_real8_market_ab_benchmark.sh")
        .expect("REAL-8 runner exists");

    for expected in [
        "no forced trades",
        "no price-as-truth",
        "no ghost liquidity",
        "no f64 economy",
        "no off-tape WAL as truth",
        "no private CoT recording",
        "no raw-log broadcast",
    ] {
        assert!(
            script.contains(expected),
            "REAL-8 report must preserve forbidden claim: {expected}"
        );
    }
}


#[tokio::test]
async fn real8_task_outcome_arm_refreshes_verify_parent_behaviorally() {
    let task = "real8x-stale-parent";
    let mut h = fresh_harness(genesis_with_balances());

    let parent0 = h.seq.q_snapshot().expect("q0").state_root_t;
    submit_and_apply(&mut h, task_open(parent0, task))
        .await
        .expect("TaskOpen accepted");

    let parent1 = h.seq.q_snapshot().expect("q1").state_root_t;
    submit_and_apply(&mut h, escrow_lock(parent1, task))
        .await
        .expect("EscrowLock accepted");

    let parent2 = h.seq.q_snapshot().expect("q2").state_root_t;
    submit_and_apply(&mut h, work_tx(parent2, task))
        .await
        .expect("WorkTx accepted");

    let post_work_root = h.seq.q_snapshot().expect("post work").state_root_t;
    submit_and_apply(&mut h, market_side_state_mutation(post_work_root, task))
        .await
        .expect("market-side state mutation accepted");

    let post_market_root = h.seq.q_snapshot().expect("post market").state_root_t;
    assert_ne!(
        post_work_root, post_market_root,
        "the market-side mutation must advance state_root_t before VerifyTx"
    );

    let stale = submit_and_apply(&mut h, verify_tx(post_work_root, "real8x-verify-stale"))
        .await
        .expect_err("VerifyTx with stale post-Work parent root must reject");
    assert!(
        matches!(stale, ApplyError::Transition(TransitionError::StaleParent)),
        "stale VerifyTx must fail with StaleParent, got {stale:?}"
    );

    let refreshed_root = h.seq.q_snapshot().expect("post stale reject").state_root_t;
    assert_eq!(
        refreshed_root, post_market_root,
        "stale rejection must not advance state; refreshed parent remains post-market root"
    );
    submit_and_apply(&mut h, verify_tx(refreshed_root, "real8x-verify-refreshed"))
        .await
        .expect("VerifyTx with refreshed q_snapshot root must accept");
}
