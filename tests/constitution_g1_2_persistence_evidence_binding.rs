//! TB-G G1.2-5 (Option B+ orchestration ruling 2026-05-11; binding
//! directive `handover/directives/2026-05-11_TB_G_G1_2_OPTION_B_PLUS_RULING.md`
//! §3.4 + charter §1 G1.2-5) — persistence-evidence binding gates
//! SG-G1.2-5.1..SG-G1.2-5.6.
//!
//! Charter §0 kill_criteria_tested #1 verbatim:
//!
//! > If post-G-Phase batch evidence on the 9-problem set shows
//! > per-problem genesis reset (balances reset, positions cleared,
//! > reputation zeroed) between problems, reject — G1 ship-gate
//! > violation.
//!
//! The library under test (`runtime::persistence_evidence`) classifies
//! each of six architect-required persisted fields as
//! `Witnessed | Empty | Reset`. CI gate `report.is_passing()` ↔ no
//! `Reset` verdict. Per-architect §3.5 the Empty verdict is permitted
//! as clean-negative for low-activity batches.
//!
//! Six gates:
//!  1. binding_witnesses_balance_mutation_in_two_task_synthetic_batch
//!  2. binding_emits_clean_negative_on_empty_batch
//!  3. binding_detects_balance_reset_between_tasks
//!  4. binding_detects_autopsy_capsule_monotonicity_violation
//!  5. binding_model_identity_witnessed_when_manifest_carries_model
//!  6. binding_n_witnessed_at_least_one_for_real_two_task_batch
//!
//! FC-trace: FC2-Boot (cross-task continuity) + FC3-Markov (CAS-derived
//! view; never LLM self-report).

use tempfile::TempDir;
use turingosv4::bottom_white::cas::schema::Cid;
use turingosv4::bus::{BusConfig, TuringBus};
use turingosv4::economy::money::MicroCoin;
use turingosv4::kernel::Kernel;
use turingosv4::runtime::adapter::{
    genesis_with_balances, make_synthetic_escrow_lock, make_synthetic_task_open,
};
use turingosv4::runtime::batch_continuation_manifest::{
    BatchContinuationManifest, TaskContinuationEntry,
};
use turingosv4::runtime::persistence_evidence::{
    bind_persistence, FieldVerdict, PersistenceBindingReport,
};
use turingosv4::runtime::{build_chaintape_sequencer_with_initial_q, RuntimeChaintapeConfig};
use turingosv4::state::q_state::{AgentId, Hash, QState, TaskId};
use turingosv4::state::typed_tx::EventId;

fn cfg(tmp: &TempDir, run_id: &str, resume: bool) -> RuntimeChaintapeConfig {
    RuntimeChaintapeConfig {
        runtime_repo_path: tmp.path().join("runtime_repo"),
        cas_path: tmp.path().join("cas"),
        run_id: run_id.into(),
        queue_capacity: 16,
        resume_existing_chain: resume,
    }
}

fn mk_entry(idx: u64, problem: &str) -> TaskContinuationEntry {
    TaskContinuationEntry {
        task_index: idx,
        problem_id: problem.into(),
        start_head_t_hex: String::new(),
        end_head_t_hex: String::new(),
        start_chain_length: idx,
        end_chain_length: idx + 1,
        subprocess_command_sha256: String::new(),
        run_summary_cid_hex: String::new().into(),
        terminal_tx_id: None,
        exit_code: 0,
        started_at_unix_s: 0,
        finished_at_unix_s: 0,
    }
}

fn mk_manifest(model: &str, tasks: Vec<TaskContinuationEntry>) -> BatchContinuationManifest {
    BatchContinuationManifest {
        schema_version: "g1_2_v1".into(),
        batch_id: "g1_2_5_test".into(),
        runtime_repo: ".".into(),
        cas_root: ".".into(),
        model: model.into(),
        n_agents: 1,
        initial_head_t_hex: String::new(),
        agent_registry_cid_hex: None,
        system_pubkeys_cid_hex: None,
        model_manifest_cid_hex: None,
        role_assignment_manifest_cid_hex: None,
        tasks,
        terminated_reason: None,
    }
}

/// Build a real 2-task in-process batch where:
///   task_0 — TaskOpen advances state_root + inserts a TaskMarketEntry
///            (balances unchanged; preseed sponsor balance carried).
///   task_1 — resume bootstrap then EscrowLock debits the sponsor's
///            balance against the prior task's TaskMarketEntry.
///
/// Splitting the two txs across task boundaries dodges the strict
/// `dispatch_transition` parent_state_root check (each new tx needs to
/// be built against the live `state_root_t`, which is only readable
/// post-shutdown per the chaintape driver's async drain semantics).
/// Returns `(tmp_dir, initial_q, task_end_snapshots)`.
async fn run_two_task_escrow_batch() -> (TempDir, QState, Vec<QState>) {
    let tmp = TempDir::new().expect("tempdir");
    let sponsor = AgentId("g1_2_5-sponsor".into());

    // Pre-seed the sponsor with 10 base coin so EscrowLock has funds to
    // debit in task_1.
    let initial_q = genesis_with_balances(&[(
        sponsor.clone(),
        MicroCoin::from_coin(10).expect("from_coin(10)"),
    )]);

    // ─── Task 0 — fresh; one TaskOpen ──────────────────────────────
    let cfg_fresh = cfg(&tmp, "g1_2_5-t0", false);
    let bundle =
        build_chaintape_sequencer_with_initial_q(&cfg_fresh, initial_q.clone()).expect("fresh");
    let seq0 = bundle.sequencer.clone();
    let kernel = Kernel::new();
    let bus = TuringBus::with_sequencer(kernel, BusConfig::default(), bundle.sequencer.clone());
    let open0 = make_synthetic_task_open("g1_2_5-shared-task", &sponsor.0, Hash::ZERO, "t0-open");
    bus.submit_typed_tx(open0)
        .await
        .expect("submit TaskOpen t0");
    bundle.shutdown().await.expect("shutdown t0");
    drop(bus);
    let q_end_task_0 = seq0.q_snapshot().expect("q_snapshot end t0");

    // ─── Task 1 — resume; one EscrowLock against task_0's market ───
    let cfg_resume = cfg(&tmp, "g1_2_5-t1", true);
    let bundle_r = build_chaintape_sequencer_with_initial_q(&cfg_resume, initial_q.clone())
        .expect("resume bootstrap");
    let seq1 = bundle_r.sequencer.clone();
    let q_after_resume = seq1.q_snapshot().expect("q_snapshot post-resume");
    let kernel1 = Kernel::new();
    let bus1 = TuringBus::with_sequencer(kernel1, BusConfig::default(), bundle_r.sequencer.clone());
    let lock1 = make_synthetic_escrow_lock(
        "g1_2_5-shared-task",
        &sponsor.0,
        1_000_000,
        q_after_resume.state_root_t,
        "t1-lock",
    );
    bus1.submit_typed_tx(lock1)
        .await
        .expect("submit EscrowLock t1");
    bundle_r.shutdown().await.expect("shutdown t1");
    drop(bus1);
    let q_end_task_1 = seq1.q_snapshot().expect("q_snapshot end t1");

    (tmp, initial_q, vec![q_end_task_0, q_end_task_1])
}

// ── SG-G1.2-5.1 ─────────────────────────────────────────────────────
//
// Real 2-task in-process batch with EscrowLock mutates the sponsor's
// `balances_t` across task boundaries. Binding must verdict
// `Witnessed` for balances + `n_witnessed >= 1`.
#[tokio::test]
async fn binding_witnesses_balance_mutation_in_two_task_synthetic_batch() {
    let (_tmp, initial_q, snapshots) = run_two_task_escrow_batch().await;
    let manifest = mk_manifest(
        "synthetic-test-model",
        vec![mk_entry(0, "g1_2_5-task0"), mk_entry(1, "g1_2_5-task1")],
    );

    let report = bind_persistence(&initial_q, &snapshots, &manifest);
    assert!(
        matches!(report.balances, FieldVerdict::Witnessed(_)),
        "SG-G1.2-5.1: balances must be Witnessed after 2-task EscrowLock batch; got {:?}",
        report.balances
    );
    assert!(
        report.is_passing(),
        "SG-G1.2-5.1: real batch must have no Reset verdicts; report={report:?}"
    );
    assert!(
        report.n_witnessed() >= 1,
        "SG-G1.2-5.1: real batch must witness at least one field; got {}",
        report.n_witnessed()
    );
}

// ── SG-G1.2-5.2 ─────────────────────────────────────────────────────
//
// Empty batch — zero tasks, zero snapshots — must emit Empty for every
// field and pass the gate (architect §3.5 clean-negative permitted).
#[test]
fn binding_emits_clean_negative_on_empty_batch() {
    let manifest = mk_manifest("", vec![]);
    let report = bind_persistence(&QState::genesis(), &[], &manifest);
    assert!(report.is_passing(), "SG-G1.2-5.2: empty batch must pass");
    assert_eq!(
        report.n_witnessed(),
        0,
        "SG-G1.2-5.2: empty batch must have zero witnessed fields"
    );
    for v in [
        &report.balances,
        &report.positions,
        &report.reputation,
        &report.pnl,
        &report.autopsy,
        &report.model_identity,
    ] {
        assert!(
            matches!(v, FieldVerdict::Empty(_)),
            "SG-G1.2-5.2: every verdict must be Empty on empty batch; got {v:?}"
        );
    }
}

// ── SG-G1.2-5.3 ─────────────────────────────────────────────────────
//
// Synthetic reset: task_0 ends with non-empty balances; task_1 returns
// to an empty balances map. The binding must verdict `Reset` for
// balances and the report must fail.
#[test]
fn binding_detects_balance_reset_between_tasks() {
    let initial_q = QState::genesis();
    let mut q_task_0 = QState::genesis();
    q_task_0
        .economic_state_t
        .balances_t
        .0
        .insert(AgentId("alice".into()), MicroCoin::from_coin(7).unwrap());

    // task_1 snapshot is back to empty balances — the reset scenario.
    let q_task_1 = QState::genesis();

    let manifest = mk_manifest(
        "test-model",
        vec![mk_entry(0, "task_with_balance"), mk_entry(1, "task_reset")],
    );
    let report = bind_persistence(&initial_q, &[q_task_0, q_task_1], &manifest);
    assert!(
        matches!(report.balances, FieldVerdict::Reset(_)),
        "SG-G1.2-5.3: balances must be Reset when post-task snapshot drops \
         back to empty after a non-empty intermediate; got {:?}",
        report.balances
    );
    assert!(
        !report.is_passing(),
        "SG-G1.2-5.3: report with Reset must fail is_passing()"
    );
}

// ── SG-G1.2-5.4 ─────────────────────────────────────────────────────
//
// Autopsy monotonicity violation: task_0 ends with autopsy count 2;
// task_1 ends with count 1. Binding must verdict `Reset` for autopsy
// (kill-criterion #1).
#[test]
fn binding_detects_autopsy_capsule_monotonicity_violation() {
    let initial_q = QState::genesis();
    let mut q_task_0 = QState::genesis();
    q_task_0.economic_state_t.agent_autopsies_t.0.insert(
        EventId(TaskId("event_a".into())),
        vec![Cid::default(), Cid::default()],
    );

    let mut q_task_1 = QState::genesis();
    q_task_1
        .economic_state_t
        .agent_autopsies_t
        .0
        .insert(EventId(TaskId("event_a".into())), vec![Cid::default()]);

    let manifest = mk_manifest(
        "test-model",
        vec![
            mk_entry(0, "task_with_2_autopsies"),
            mk_entry(1, "task_with_1"),
        ],
    );
    let report = bind_persistence(&initial_q, &[q_task_0, q_task_1], &manifest);
    assert!(
        matches!(report.autopsy, FieldVerdict::Reset(_)),
        "SG-G1.2-5.4: autopsy must be Reset when capsule count decreases; \
         got {:?}",
        report.autopsy
    );
    assert!(
        !report.is_passing(),
        "SG-G1.2-5.4: report with autopsy Reset must fail is_passing()"
    );
}

// ── SG-G1.2-5.5 ─────────────────────────────────────────────────────
//
// Model identity stability: a non-empty `manifest.model` on a non-empty
// batch must be `Witnessed`. The kill-criterion is the inverse — an
// empty model string on a non-empty batch is `Reset` (covered by the
// library's unit test `unit_non_empty_batch_missing_model_is_reset`).
#[test]
fn binding_model_identity_witnessed_when_manifest_carries_model() {
    let q = QState::genesis();
    let manifest = mk_manifest("deepseek-chat", vec![mk_entry(0, "p"), mk_entry(1, "q")]);
    let report = bind_persistence(&q, &[q.clone(), q.clone()], &manifest);
    match &report.model_identity {
        FieldVerdict::Witnessed(detail) => {
            assert!(
                detail.contains("deepseek-chat"),
                "SG-G1.2-5.5: model identity detail must name model; got {detail}"
            );
        }
        other => panic!(
            "SG-G1.2-5.5: model identity must be Witnessed for stable non-empty \
             manifest.model; got {other:?}"
        ),
    }
}

// ── SG-G1.2-5.6 ─────────────────────────────────────────────────────
//
// Aggregate dual-bound to SG-G1.2-5.1: for a real 2-task batch with
// EscrowLock + model carried in manifest, `n_witnessed` must be ≥2
// (balances + pnl from EscrowLock activity, plus model identity).
// Defends against the binding silently classifying every field as
// Empty on a real batch.
#[tokio::test]
async fn binding_n_witnessed_at_least_one_for_real_two_task_batch() {
    let (_tmp, initial_q, snapshots) = run_two_task_escrow_batch().await;
    let manifest = mk_manifest(
        "synthetic-test-model",
        vec![mk_entry(0, "g1_2_5-task0"), mk_entry(1, "g1_2_5-task1")],
    );
    let report: PersistenceBindingReport = bind_persistence(&initial_q, &snapshots, &manifest);
    assert!(
        report.n_witnessed() >= 2,
        "SG-G1.2-5.6: real 2-task EscrowLock batch with named model must \
         witness ≥2 fields (balances + pnl + model_identity); got {} — {report:?}",
        report.n_witnessed()
    );
    assert!(
        matches!(report.model_identity, FieldVerdict::Witnessed(_)),
        "SG-G1.2-5.6: model identity must be Witnessed on named-model batch"
    );

    // Defense-in-depth: the per-task trace records the balance drop.
    // If this assertion fails, the binding read different fields than
    // expected from `economic_state_t`.
    let trace_0 = &report.per_task[0];
    let trace_1 = &report.per_task[1];
    assert!(
        trace_1.balances_total_micro <= trace_0.balances_total_micro,
        "SG-G1.2-5.6: task_1 balance total must be <= task_0 (EscrowLock \
         debited sponsor); trace_0={trace_0:?} trace_1={trace_1:?}"
    );
}
