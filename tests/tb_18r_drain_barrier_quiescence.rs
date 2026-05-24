//! TB-18R R4 — drain barrier quiescence witness.
//!
//! Asserts `verify_chain_quiescent_post_drain()` PASS after
//! `ChaintapeBundle::shutdown().await` per charter §1.2 FR-18R.3 v2 drain
//! barrier semantics + Codex Q4 remediation. Negative path covered via
//! the `DrainBarrierViolation` error type round-trip (an actual
//! pre-drain non-quiescent assertion would race the driver task and is
//! not deterministically observable in test).
//!
//! See `handover/ai-direct/TB-18R_R4_STEP_B_invariant.md` §5 test plan.

use tempfile::TempDir;

use turingosv4::bottom_white::cas::schema::Cid;
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::bus::{BusConfig, TuringBus};
use turingosv4::kernel::Kernel;
use turingosv4::runtime::adapter::{make_real_worktx_signed_by, make_synthetic_task_open};
use turingosv4::runtime::agent_keypairs::AgentKeypairRegistry;
use turingosv4::runtime::chain_derived_run_facts::{
    verify_chain_quiescent_post_drain, ChainDerivedError, DrainBarrierViolation,
};
use turingosv4::runtime::proposal_telemetry::{
    write_to_cas as write_telemetry, ProposalTelemetry, TokenCounts,
};
use turingosv4::runtime::{build_chaintape_sequencer, RuntimeChaintapeConfig};
use turingosv4::state::q_state::{AgentId, Hash};
use turingosv4::state::sequencer::task_open_accept_state_root;

fn fresh_config(tmp: &TempDir, run_id: &str) -> RuntimeChaintapeConfig {
    RuntimeChaintapeConfig {
        runtime_repo_path: tmp.path().join("runtime_repo"),
        cas_path: tmp.path().join("cas"),
        run_id: run_id.to_string(),
        queue_capacity: 32,
        resume_existing_chain: false,
    }
}

/// FR-18R.3 v2: after `bundle.shutdown().await`, every submitted typed-tx
/// has reached terminal state on chain or L4.E. The witness function
/// asserts `next_submit_id - 1 == l4_count + l4e_count`.
#[tokio::test]
async fn quiescent_post_shutdown_passes() {
    let tmp = TempDir::new().expect("tempdir");
    let cfg = fresh_config(&tmp, "tb18r-r4-quiescent");
    let bundle = build_chaintape_sequencer(&cfg).expect("bootstrap");
    let bus = TuringBus::with_sequencer(
        Kernel::new(),
        BusConfig::default(),
        bundle.sequencer.clone(),
    );

    // Submit 1 TaskOpen + 3 zero-stake WorkTx → 4 total submissions →
    // 1 L4 (TaskOpen) + 3 L4.E (zero-stake rejections) = 4 chain entries.
    let boot_root = bundle
        .sequencer
        .q_snapshot()
        .expect("post-activation q")
        .state_root_t;
    let task_open = make_synthetic_task_open(
        "task-r4-quiescent",
        "tb18r-r4-sponsor",
        boot_root,
        "quiescent-seed",
    );
    let post_task_open_root = task_open_accept_state_root(&boot_root, &task_open);
    bus.submit_typed_tx(task_open)
        .await
        .expect("TaskOpen submit");

    let mut reg = AgentKeypairRegistry::open(&cfg.runtime_repo_path).expect("open keypairs");
    let mut cas_store = CasStore::open(&cfg.cas_path).expect("open cas");
    for idx in 0..3 {
        let agent = format!("agent_{idx}");
        let pt = ProposalTelemetry::new_root(
            AgentId(agent.clone()),
            Hash([0x10 + idx as u8; 32]),
            Cid([0x20 + idx as u8; 32]),
            "rfl".into(),
            TokenCounts::default(),
            format!("{agent}.b{idx}"),
        );
        let tel_cid = write_telemetry(&mut cas_store, &pt, "tb18r-r4-quiescent", 1).expect("write");
        let work_tx = make_real_worktx_signed_by(
            &mut reg,
            "task-r4-quiescent",
            &agent,
            post_task_open_root,
            0,
            &format!("q{idx}"),
            tel_cid,
            true,
            (idx + 1) as u64,
        )
        .expect("real WorkTx");
        bus.submit_typed_tx(work_tx).await.expect("WorkTx submit");
    }

    let sequencer = bundle.sequencer.clone();
    let runtime_repo = cfg.runtime_repo_path.clone();
    bundle.shutdown().await.expect("drain");

    // Post-drain: next_submit_id-1 == l4 + l4e.
    verify_chain_quiescent_post_drain(&sequencer, &runtime_repo)
        .expect("post-shutdown chain must be quiescent");
}

/// Genesis empty chain: next_submit_id_peek=1, l4=0, l4e=0 →
/// `1 - 1 == 0 + 0` → quiescent (vacuously).
#[tokio::test]
async fn quiescent_genesis_empty_chain() {
    let tmp = TempDir::new().expect("tempdir");
    let cfg = fresh_config(&tmp, "tb18r-r4-genesis");
    let bundle = build_chaintape_sequencer(&cfg).expect("bootstrap");
    let sequencer = bundle.sequencer.clone();
    let runtime_repo = cfg.runtime_repo_path.clone();
    bundle.shutdown().await.expect("drain");
    verify_chain_quiescent_post_drain(&sequencer, &runtime_repo)
        .expect("genesis empty chain must be vacuously quiescent");
}

/// Negative path: a `DrainBarrierViolation::QuiescenceCountMismatch`
/// formats with the expected fields so ship-gate evidence can record the
/// exact mismatch.
#[test]
fn drain_barrier_violation_error_round_trip() {
    let v = DrainBarrierViolation::QuiescenceCountMismatch {
        next_submit_id_minus_one: 5,
        l4_count: 2,
        l4e_count: 1,
    };
    let display = format!("{v}");
    assert!(display.contains("drain barrier"));
    assert!(display.contains("next_submit_id-1=5"));
    assert!(display.contains("l4=2"));
    assert!(display.contains("l4e=1"));

    let err = ChainDerivedError::DrainBarrier(v.clone());
    let err_display = format!("{err}");
    assert_eq!(err_display, display);
}
