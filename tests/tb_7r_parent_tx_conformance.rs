//! TB-7R parent_tx conformance tests — architect-mandated 2026-05-02.
//!
//! Per architect verdict 2026-05-02 (parent_tx ParentTx/DAG/Smoke ruling),
//! TB-7R may ship if a deterministic conformance test demonstrates the
//! parent_tx plumbing. Natural smoke under verdict A1=B′ + complete-tool
//! one-shot solve has parent_tx_edges=0 with
//! `ParentTxState::SingletonGoldenPathValid` — that is correct, not a
//! defect. The plumbing must therefore be proven via synthetic fixtures,
//! not by fabricating natural-smoke edges.
//!
//! TRACE_MATRIX § 3 orphan (TB-7R 2026-05-02; see
//! `handover/alignment/OBS_R022_TRACE_MATRIX_TB7R_ORPHANS_2026-05-02.md`).
//!
//! `FC-trace: Art.III.4 (selective broadcasting) + Art.IV (terminate states)
//! + WP-§5.L4 + verdict 2026-05-02 §"Required tests"`.

use std::time::{Duration, Instant};

use tempfile::TempDir;

use turingosv4::bottom_white::cas::{schema::ObjectType, store::CasStore};
use turingosv4::bus::{BusConfig, TuringBus};
use turingosv4::economy::money::MicroCoin;
use turingosv4::kernel::Kernel;
use turingosv4::runtime::adapter::{
    genesis_with_balances, make_real_worktx_signed_by, make_synthetic_escrow_lock,
    make_synthetic_task_open,
};
use turingosv4::runtime::agent_keypairs::AgentKeypairRegistry;
use turingosv4::runtime::chain_derived_run_facts::{compute_run_facts_from_chain, ParentTxState};
use turingosv4::runtime::proposal_telemetry::{
    write_to_cas as write_telemetry, ProposalTelemetry, TokenCounts,
};
use turingosv4::runtime::verification_result::{write_to_cas as write_vr, VerificationResult};
use turingosv4::runtime::{
    build_chaintape_sequencer_with_initial_q, ChaintapeBundle, RuntimeChaintapeConfig,
};
use turingosv4::state::q_state::{AgentId, Hash, TxId};
use turingosv4::state::typed_tx::TypedTx;

// ── Test fixture helpers ───────────────────────────────────────────────

fn fresh_config(tmp: &TempDir, run_id: &str) -> RuntimeChaintapeConfig {
    RuntimeChaintapeConfig {
        runtime_repo_path: tmp.path().join("runtime_repo"),
        cas_path: tmp.path().join("cas"),
        run_id: run_id.into(),
        queue_capacity: 16,
        resume_existing_chain: false,
    }
}

/// Build a fresh bundle with preseeded balances (sponsor + N agents).
fn fresh_bundle_with_preseed(cfg: &RuntimeChaintapeConfig, agent_count: usize) -> ChaintapeBundle {
    let mut pairs: Vec<(AgentId, MicroCoin)> = vec![(
        AgentId("test-sponsor".into()),
        MicroCoin::from_micro_units(10_000_000),
    )];
    for i in 0..agent_count {
        pairs.push((
            AgentId(format!("Agent_{i}")),
            MicroCoin::from_micro_units(1_000_000),
        ));
    }
    let initial_q = genesis_with_balances(&pairs);
    build_chaintape_sequencer_with_initial_q(cfg, initial_q).expect("bootstrap")
}

/// Submit on-chain TaskOpen + EscrowLock and return the post-EscrowLock state_root.
async fn seed_task_and_escrow(bus: &TuringBus, bundle: &ChaintapeBundle, task_id: &str) -> Hash {
    // TaskOpen
    let boot_root = bundle
        .sequencer
        .q_snapshot()
        .expect("post-activation q")
        .state_root_t;
    let task_open = make_synthetic_task_open(task_id, "test-sponsor", boot_root, "test-seed");
    bus.submit_typed_tx(task_open)
        .await
        .expect("TaskOpen submit");
    // Wait state_root advance
    let parent_root = wait_state_root_advance(bundle, boot_root).await;
    // EscrowLock
    let escrow_lock =
        make_synthetic_escrow_lock(task_id, "test-sponsor", 100_000, parent_root, "test-escrow");
    bus.submit_typed_tx(escrow_lock)
        .await
        .expect("EscrowLock submit");
    wait_state_root_advance(bundle, parent_root).await
}

/// Poll q_snapshot until state_root_t advances past `prev` or 5s elapse.
async fn wait_state_root_advance(bundle: &ChaintapeBundle, prev: Hash) -> Hash {
    let deadline = Instant::now() + Duration::from_secs(5);
    while Instant::now() < deadline {
        if let Ok(q) = bundle.sequencer.q_snapshot() {
            if q.state_root_t != prev {
                return q.state_root_t;
            }
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    panic!("state_root did not advance from {prev:?} within 5s");
}

/// Build a ProposalTelemetry + WorkTx pair, write telemetry to CAS, return tel_cid + WorkTx.
/// `with_vr_verified` controls whether a verified VerificationResult is attached.
async fn submit_worktx(
    bus: &TuringBus,
    bundle: &ChaintapeBundle,
    cas: &mut CasStore,
    reg: &mut AgentKeypairRegistry,
    task_id: &str,
    agent_id: &str,
    branch_id: &str,
    parent_tx: Option<TxId>,
    suffix: &str,
    with_vr_verified: bool,
    parent_state_root: Hash,
) -> (TxId, Hash) {
    // Build telemetry root (without VR yet — we may attach below).
    let proposal_artifact_cid = cas
        .put(
            format!("theorem {task_id}_{suffix} : True := by trivial").as_bytes(),
            ObjectType::ProposalPayload,
            agent_id,
            1,
            Some("tb7r.parent_tx.proposal.v1".into()),
        )
        .expect("write proposal artifact");
    let tel_root = ProposalTelemetry::new_root(
        AgentId(agent_id.into()),
        Hash([0xaa; 32]),
        proposal_artifact_cid,
        "step_complete".into(),
        TokenCounts {
            prompt_tokens: 100,
            completion_tokens: 50,
            tool_tokens: 0,
        },
        branch_id.into(),
    );
    let mut tel = ProposalTelemetry {
        parent_tx,
        ..tel_root
    };

    // Build expected work_tx_id deterministically (matches make_real_worktx_signed_by format).
    let expected_work_tx_id = TxId(format!("worktx-{task_id}-{suffix}"));

    // Optionally attach a verified VR (for SingletonGoldenPathValid case).
    if with_vr_verified {
        let vr = VerificationResult::from_lean_run(
            expected_work_tx_id.clone(),
            AgentId(agent_id.into()),
            0, // exit code 0 → verified=true
            proposal_artifact_cid,
            "test-proof.lean",
            b"calc 1=1 := rfl",
        );
        let vr_cid = write_vr(cas, &vr, "test-vr", 1).expect("write VR");
        tel = tel.with_verification_result(vr_cid);
    }
    let tel_cid = write_telemetry(cas, &tel, "test-telemetry", 1).expect("write telemetry");

    let work_tx = make_real_worktx_signed_by(
        reg,
        task_id,
        agent_id,
        parent_state_root,
        1_000, // stake = 1000 micro = 0.001 coin (matches preseed escrow defaults)
        suffix,
        tel_cid,
        true,
        1,
    )
    .expect("worktx build");
    let work_tx_id = match &work_tx {
        TypedTx::Work(w) => w.tx_id.clone(),
        _ => panic!("expected Work variant"),
    };
    bus.submit_typed_tx(work_tx).await.expect("Work submit");
    let new_root = wait_state_root_advance(bundle, parent_state_root).await;
    (work_tx_id, new_root)
}

// ── 6 conformance tests ────────────────────────────────────────────────

/// Test 1 — `singleton_golden_path_has_zero_edges_and_is_valid`
///
/// Build a fixture: 1 L4 accepted WorkTx, branch_id="Agent_0.b1",
/// parent_tx=None, ProposalTelemetry has verification_result_cid pointing
/// to a VerificationResult with verified=true. Verify:
///  - chain_oracle_verified=true
///  - parent_tx_state == SingletonGoldenPathValid
///  - branch_lineage edges = 0
#[tokio::test]
async fn singleton_golden_path_has_zero_edges_and_is_valid() {
    let tmp = TempDir::new().expect("tempdir");
    let cfg = fresh_config(&tmp, "tb7r-conformance-1");
    let bundle = fresh_bundle_with_preseed(&cfg, 1);
    let bus = TuringBus::with_sequencer(
        Kernel::new(),
        BusConfig::default(),
        bundle.sequencer.clone(),
    );
    let task_id = format!("task-{}", cfg.run_id);
    let post_escrow_root = seed_task_and_escrow(&bus, &bundle, &task_id).await;
    let mut reg = AgentKeypairRegistry::open(&cfg.runtime_repo_path).expect("reg");
    let mut cas = CasStore::open(&cfg.cas_path).expect("cas");

    // Singleton solve.
    let (_tx_id, _) = submit_worktx(
        &bus,
        &bundle,
        &mut cas,
        &mut reg,
        &task_id,
        "Agent_0",
        "Agent_0.b1",
        None, // root attempt
        "p1",
        true, // attach verified VR
        post_escrow_root,
    )
    .await;

    bundle.shutdown().await.expect("shutdown");

    let facts = compute_run_facts_from_chain(&cfg.runtime_repo_path, &cfg.cas_path).expect("facts");
    assert!(
        facts.chain_oracle_verified,
        "singleton run with verified VR must have chain_oracle_verified=true"
    );
    assert_eq!(
        facts.parent_tx_state,
        ParentTxState::SingletonGoldenPathValid,
        "singleton solved run must label parent_tx_state=SingletonGoldenPathValid; got {:?}",
        facts.parent_tx_state
    );
}

/// Test 2 — `second_attempt_same_branch_has_parent_tx`
///
/// Build a fixture: 2 L4 accepted WorkTx, same (Agent_0, Agent_0.b1).
/// First with parent_tx=None (root). Second with parent_tx=Some(first.tx_id).
/// Verify dashboard reconstructs the edge — parent_tx_state == MultiAttemptDagValid.
#[tokio::test]
async fn second_attempt_same_branch_has_parent_tx() {
    let tmp = TempDir::new().expect("tempdir");
    let cfg = fresh_config(&tmp, "tb7r-conformance-2");
    let bundle = fresh_bundle_with_preseed(&cfg, 1);
    let bus = TuringBus::with_sequencer(
        Kernel::new(),
        BusConfig::default(),
        bundle.sequencer.clone(),
    );
    let task_id = format!("task-{}", cfg.run_id);
    let post_escrow = seed_task_and_escrow(&bus, &bundle, &task_id).await;
    let mut reg = AgentKeypairRegistry::open(&cfg.runtime_repo_path).expect("reg");
    let mut cas = CasStore::open(&cfg.cas_path).expect("cas");

    let (attempt_1_id, post_attempt_1) = submit_worktx(
        &bus,
        &bundle,
        &mut cas,
        &mut reg,
        &task_id,
        "Agent_0",
        "Agent_0.b1",
        None,
        "p1",
        false, // intermediate progress, no VR yet
        post_escrow,
    )
    .await;

    let (_attempt_2_id, _) = submit_worktx(
        &bus,
        &bundle,
        &mut cas,
        &mut reg,
        &task_id,
        "Agent_0",
        "Agent_0.b1",
        Some(attempt_1_id.clone()), // parent_tx wired
        "p2",
        false,
        post_attempt_1,
    )
    .await;

    bundle.shutdown().await.expect("shutdown");

    let facts = compute_run_facts_from_chain(&cfg.runtime_repo_path, &cfg.cas_path).expect("facts");
    assert_eq!(
        facts.parent_tx_state,
        ParentTxState::MultiAttemptDagValid,
        "two attempts on same (agent,branch) with parent_tx wired must label MultiAttemptDagValid; got {:?}",
        facts.parent_tx_state
    );
    assert!(
        facts.proposal_count >= 2,
        "at least 2 proposals on chain; got {}",
        facts.proposal_count
    );
}

/// Test 3 — `missing_parent_on_nonroot_attempt_is_violation`
///
/// Build a fixture: 2 L4 accepted WorkTx, same (Agent_0, Agent_0.b1).
/// First with parent_tx=None (root). Second WITH parent_tx=None (BUG —
/// non-root attempt missing parent_tx). Verify dashboard flags as violation.
#[tokio::test]
async fn missing_parent_on_nonroot_attempt_is_violation() {
    let tmp = TempDir::new().expect("tempdir");
    let cfg = fresh_config(&tmp, "tb7r-conformance-3");
    let bundle = fresh_bundle_with_preseed(&cfg, 1);
    let bus = TuringBus::with_sequencer(
        Kernel::new(),
        BusConfig::default(),
        bundle.sequencer.clone(),
    );
    let task_id = format!("task-{}", cfg.run_id);
    let post_escrow = seed_task_and_escrow(&bus, &bundle, &task_id).await;
    let mut reg = AgentKeypairRegistry::open(&cfg.runtime_repo_path).expect("reg");
    let mut cas = CasStore::open(&cfg.cas_path).expect("cas");

    let (_attempt_1_id, post_attempt_1) = submit_worktx(
        &bus,
        &bundle,
        &mut cas,
        &mut reg,
        &task_id,
        "Agent_0",
        "Agent_0.b1",
        None,
        "p1",
        false,
        post_escrow,
    )
    .await;

    let (_attempt_2_id, _) = submit_worktx(
        &bus,
        &bundle,
        &mut cas,
        &mut reg,
        &task_id,
        "Agent_0",
        "Agent_0.b1",
        None, // BUG: non-root attempt missing parent_tx
        "p2",
        false,
        post_attempt_1,
    )
    .await;

    bundle.shutdown().await.expect("shutdown");

    let facts = compute_run_facts_from_chain(&cfg.runtime_repo_path, &cfg.cas_path).expect("facts");
    assert_eq!(
        facts.parent_tx_state,
        ParentTxState::MissingParentTxViolation,
        "two attempts on same branch with attempt_2.parent_tx=None must label MissingParentTxViolation; got {:?}",
        facts.parent_tx_state
    );
}

/// Test 4 — `dashboard_renders_singleton_golden_path`
///
/// The audit_dashboard binary's text output for a singleton solved run
/// must include the depth=0 [ORACLE] line. Calls compute_run_facts +
/// inspects the rendered §7 golden path and §6 parent_tx_state.
#[tokio::test]
async fn dashboard_renders_singleton_golden_path() {
    use std::process::Command;
    let tmp = TempDir::new().expect("tempdir");
    let cfg = fresh_config(&tmp, "tb7r-conformance-4");
    let bundle = fresh_bundle_with_preseed(&cfg, 1);
    let bus = TuringBus::with_sequencer(
        Kernel::new(),
        BusConfig::default(),
        bundle.sequencer.clone(),
    );
    let task_id = format!("task-{}", cfg.run_id);
    let post_escrow = seed_task_and_escrow(&bus, &bundle, &task_id).await;
    let mut reg = AgentKeypairRegistry::open(&cfg.runtime_repo_path).expect("reg");
    let mut cas = CasStore::open(&cfg.cas_path).expect("cas");

    submit_worktx(
        &bus,
        &bundle,
        &mut cas,
        &mut reg,
        &task_id,
        "Agent_0",
        "Agent_0.b1",
        None,
        "p1",
        true, // verified VR for singleton golden path
        post_escrow,
    )
    .await;
    bundle.shutdown().await.expect("shutdown");

    // Invoke audit_dashboard binary and capture output.
    let dashboard_bin = env!("CARGO_BIN_EXE_audit_dashboard");
    let output = Command::new(dashboard_bin)
        .arg("--repo")
        .arg(&cfg.runtime_repo_path)
        .arg("--cas")
        .arg(&cfg.cas_path)
        .output()
        .expect("audit_dashboard run");
    assert!(output.status.success(), "audit_dashboard exited non-zero");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(
        stdout.contains("parent_tx_state: SingletonGoldenPathValid"),
        "dashboard must render SingletonGoldenPathValid for singleton solve; got:\n{stdout}"
    );
    assert!(
        stdout.contains("depth=0"),
        "dashboard must render depth=0 golden path; got:\n{stdout}"
    );
    assert!(
        stdout.contains("[ORACLE]"),
        "dashboard must render [ORACLE] marker; got:\n{stdout}"
    );
    assert!(
        stdout.contains("chain_oracle_verified   : true"),
        "dashboard must report chain_oracle_verified=true; got:\n{stdout}"
    );
}

/// Test 5 — `unsolved_runs_have_no_fake_accepted_nodes`
///
/// Build a fixture: TaskOpen + EscrowLock only (no Work). Verify:
///  - chain_oracle_verified=false
///  - 0 L4 accepted Work entries (no fake accepted node)
///  - parent_tx_state = NoMultiAttemptObserved (zero externalized proposals)
#[tokio::test]
async fn unsolved_runs_have_no_fake_accepted_nodes() {
    let tmp = TempDir::new().expect("tempdir");
    let cfg = fresh_config(&tmp, "tb7r-conformance-5");
    let bundle = fresh_bundle_with_preseed(&cfg, 1);
    let bus = TuringBus::with_sequencer(
        Kernel::new(),
        BusConfig::default(),
        bundle.sequencer.clone(),
    );
    let task_id = format!("task-{}", cfg.run_id);
    let _ = seed_task_and_escrow(&bus, &bundle, &task_id).await;
    bundle.shutdown().await.expect("shutdown");

    let facts = compute_run_facts_from_chain(&cfg.runtime_repo_path, &cfg.cas_path).expect("facts");
    assert!(
        !facts.chain_oracle_verified,
        "unsolved run must NOT claim chain_oracle_verified=true; got true"
    );
    assert_eq!(
        facts.proposal_count, 0,
        "unsolved run with no Work submitted must have proposal_count=0; got {}",
        facts.proposal_count
    );
    assert_eq!(
        facts.parent_tx_state,
        ParentTxState::NoMultiAttemptObserved,
        "no externalized proposals → NoMultiAttemptObserved; got {:?}",
        facts.parent_tx_state
    );
}

/// Test 6 — `proposal_count_chain_equals_externalized_proposal_count`
///
/// Build a fixture with N=3 distinct WorkTx submitted via submit_typed_tx.
/// Verify proposal_count == 3.
#[tokio::test]
async fn proposal_count_chain_equals_externalized_proposal_count() {
    let tmp = TempDir::new().expect("tempdir");
    let cfg = fresh_config(&tmp, "tb7r-conformance-6");
    let bundle = fresh_bundle_with_preseed(&cfg, 1);
    let bus = TuringBus::with_sequencer(
        Kernel::new(),
        BusConfig::default(),
        bundle.sequencer.clone(),
    );
    let task_id = format!("task-{}", cfg.run_id);
    let post_escrow = seed_task_and_escrow(&bus, &bundle, &task_id).await;
    let mut reg = AgentKeypairRegistry::open(&cfg.runtime_repo_path).expect("reg");
    let mut cas = CasStore::open(&cfg.cas_path).expect("cas");

    let (a1, root1) = submit_worktx(
        &bus,
        &bundle,
        &mut cas,
        &mut reg,
        &task_id,
        "Agent_0",
        "Agent_0.b1",
        None,
        "p1",
        false,
        post_escrow,
    )
    .await;
    let (a2, root2) = submit_worktx(
        &bus,
        &bundle,
        &mut cas,
        &mut reg,
        &task_id,
        "Agent_0",
        "Agent_0.b1",
        Some(a1),
        "p2",
        false,
        root1,
    )
    .await;
    submit_worktx(
        &bus,
        &bundle,
        &mut cas,
        &mut reg,
        &task_id,
        "Agent_0",
        "Agent_0.b1",
        Some(a2),
        "p3",
        false,
        root2,
    )
    .await;
    bundle.shutdown().await.expect("shutdown");

    let facts = compute_run_facts_from_chain(&cfg.runtime_repo_path, &cfg.cas_path).expect("facts");
    assert_eq!(
        facts.proposal_count, 3,
        "3 submit_typed_tx calls must produce proposal_count=3; got {}",
        facts.proposal_count
    );
    // 3 attempts on same (agent, branch) with parent_tx wired → MultiAttemptDagValid.
    assert_eq!(
        facts.parent_tx_state,
        ParentTxState::MultiAttemptDagValid,
        "3-deep DAG with parent_tx wired must label MultiAttemptDagValid; got {:?}",
        facts.parent_tx_state
    );
}
