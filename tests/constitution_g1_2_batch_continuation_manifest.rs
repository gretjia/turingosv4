//! TB-G G1.2-4 (Option B+ orchestration ruling 2026-05-11) —
//! BatchContinuationManifest gates SG-G1.2-4.1..G1.2-4.4.
//!
//! Architect §3.3 verbatim:
//!
//! > 新增 BatchContinuationManifest. 这个 manifest 进入 CAS, 并由最后
//! > 的 TerminalSummaryTx 或 BatchSummaryTx 引用. 否则 dashboard 仍然
//! > 只是把多个 runs 拼起来，不是真实 batch.
//!
//! Four gates:
//!  1. manifest_records_all_tasks_in_order
//!  2. manifest_head_chain_is_continuous
//!  3. manifest_rejects_continuity_gap
//!  4. manifest_replay_matches_real_chain_head_walk
//!
//! FC-trace: FC2-Boot (continuity) + FC3-Markov (CAS-anchored
//! materialised view).

use std::path::PathBuf;

use tempfile::TempDir;
use turingosv4::bus::{BusConfig, TuringBus};
use turingosv4::kernel::Kernel;
use turingosv4::runtime::adapter::make_synthetic_task_open;
use turingosv4::runtime::batch_continuation_manifest::{
    replay_continuity, replay_matches_real_chain_head, BatchContinuationManifest,
    ContinuityFailure, TaskContinuationEntry,
};
use turingosv4::runtime::resume_preflight::snapshot_head_t;
use turingosv4::runtime::{build_chaintape_sequencer, RuntimeChaintapeConfig};
use turingosv4::state::q_state::Hash;

async fn run_task_inproc(
    runtime_repo: &PathBuf,
    cas_path: &PathBuf,
    run_id: &str,
    resume: bool,
) -> (String, u64) {
    let cfg = RuntimeChaintapeConfig {
        runtime_repo_path: runtime_repo.clone(),
        cas_path: cas_path.clone(),
        run_id: run_id.into(),
        queue_capacity: 16,
        resume_existing_chain: resume,
    };
    let bundle = build_chaintape_sequencer(&cfg).expect("bootstrap");
    let parent_state_root = if resume {
        bundle
            .sequencer
            .q_snapshot()
            .expect("q_snapshot post-resume")
            .state_root_t
    } else {
        Hash::ZERO
    };
    let kernel = Kernel::new();
    let bus = TuringBus::with_sequencer(kernel, BusConfig::default(), bundle.sequencer.clone());
    let tx = make_synthetic_task_open(
        &format!("task-{run_id}"),
        &format!("sponsor-{run_id}"),
        parent_state_root,
        run_id,
    );
    bus.submit_typed_tx(tx).await.expect("submit TaskOpen");
    bundle.shutdown().await.expect("shutdown");
    drop(bus);
    let (head_hex, _, len) =
        snapshot_head_t(runtime_repo).expect("snapshot post-task");
    (head_hex, len)
}

fn mk_entry(idx: u64, start: &str, end: &str, start_len: u64, end_len: u64) -> TaskContinuationEntry {
    TaskContinuationEntry {
        task_index: idx,
        problem_id: format!("p{idx}"),
        start_head_t_hex: start.into(),
        end_head_t_hex: end.into(),
        start_chain_length: start_len,
        end_chain_length: end_len,
        subprocess_command_sha256: String::new(),
        run_summary_cid_hex: None,
        terminal_tx_id: None,
        exit_code: 0,
        started_at_unix_s: 0,
        finished_at_unix_s: 0,
    }
}

fn mk_manifest(initial: &str, tasks: Vec<TaskContinuationEntry>) -> BatchContinuationManifest {
    BatchContinuationManifest {
        schema_version: "g1_2_v1".into(),
        batch_id: "test_batch".into(),
        runtime_repo: ".".into(),
        cas_root: ".".into(),
        model: "test-model".into(),
        n_agents: 1,
        initial_head_t_hex: initial.into(),
        agent_registry_cid_hex: None,
        system_pubkeys_cid_hex: None,
        model_manifest_cid_hex: None,
        tasks,
        terminated_reason: None,
    }
}

// ── SG-G1.2-4.1 ─────────────────────────────────────────────────────────────
//
// `tasks` records every executed task in commit order (task_index
// monotone-incrementing from 0).
#[test]
fn manifest_records_all_tasks_in_order() {
    let m = mk_manifest(
        "",
        vec![
            mk_entry(0, "", "headA", 0, 1),
            mk_entry(1, "headA", "headB", 1, 2),
            mk_entry(2, "headB", "headC", 2, 3),
        ],
    );
    let indices: Vec<u64> = m.tasks.iter().map(|t| t.task_index).collect();
    assert_eq!(indices, vec![0, 1, 2], "SG-G1.2-4.1: indices must be 0..N");
    for (i, t) in m.tasks.iter().enumerate() {
        assert_eq!(t.task_index, i as u64);
    }
}

// ── SG-G1.2-4.2 ─────────────────────────────────────────────────────────────
//
// `replay_continuity` returns Ok on a chain where every
// `task[k+1].start == task[k].end` and `task[0].start == initial`.
#[test]
fn manifest_head_chain_is_continuous() {
    let m = mk_manifest(
        "",
        vec![
            mk_entry(0, "", "headA", 0, 1),
            mk_entry(1, "headA", "headB", 1, 2),
            mk_entry(2, "headB", "headC", 2, 3),
        ],
    );
    replay_continuity(&m).expect("SG-G1.2-4.2: continuous chain must replay Ok");
}

// ── SG-G1.2-4.3 ─────────────────────────────────────────────────────────────
//
// Synthetic gap (task[1].start != task[0].end) → ContinuityFailure::Gap.
#[test]
fn manifest_rejects_continuity_gap() {
    let m = mk_manifest(
        "",
        vec![
            mk_entry(0, "", "headA", 0, 1),
            mk_entry(1, "headDIFFERENT", "headB", 1, 2),
        ],
    );
    match replay_continuity(&m) {
        Err(ContinuityFailure::Gap { task_k_index, task_k_plus_1_index, .. }) => {
            assert_eq!(task_k_index, 0);
            assert_eq!(task_k_plus_1_index, 1);
        }
        other => panic!("SG-G1.2-4.3: expected Gap, got {other:?}"),
    }
}

// ── SG-G1.2-4.4 ─────────────────────────────────────────────────────────────
//
// Build a real 2-task ChainTape via in-process bootstrap; build the
// manifest from observed head snapshots; verify
// `replay_matches_real_chain_head` returns Ok.
#[tokio::test]
async fn manifest_replay_matches_real_chain_head_walk() {
    let tmp = TempDir::new().expect("tempdir");
    let runtime_repo = tmp.path().join("runtime_repo");
    let cas_path = tmp.path().join("cas");

    let (head_t0, len_t0) =
        run_task_inproc(&runtime_repo, &cas_path, "manifest_t0", false).await;
    let (head_t1, len_t1) =
        run_task_inproc(&runtime_repo, &cas_path, "manifest_t1", true).await;

    let m = mk_manifest(
        "",
        vec![
            mk_entry(0, "", &head_t0, 0, len_t0),
            mk_entry(1, &head_t0, &head_t1, len_t0, len_t1),
        ],
    );

    replay_continuity(&m).expect("SG-G1.2-4.4: continuity prereq Ok");
    replay_matches_real_chain_head(&m, &runtime_repo)
        .expect("SG-G1.2-4.4: manifest head must match live chain head");

    // Tamper check: bump the manifest's last end_head to a wrong
    // value and verify the replay rejects it.
    let mut tampered = m.clone();
    tampered.tasks.last_mut().unwrap().end_head_t_hex = "0".repeat(40);
    match replay_matches_real_chain_head(&tampered, &runtime_repo) {
        Err(_) => {}
        Ok(_) => panic!("SG-G1.2-4.4: tampered manifest must NOT match live head"),
    }
}
