//! TB-6 Atom 6 — RunSummary integration tests.
//!
//! Architect ruling 2026-05-01 § 3.6 Atom 6: record `tx_count`,
//! `failed_branch_count`, `rollback_count`, candidate proposal CIDs,
//! accepted tx_id, rejected tx_ids. Records proposal-level fork, NOT
//! chain-of-thought.
//!
//! - I92: end-to-end. Bootstrap chaintape, submit synthetic
//!   TaskOpen + zero-stake WorkTx, shutdown, build RunSummary from the
//!   chain, verify all fields populate as expected.
//! - I92b: empty chain.
//! - I92c: write_json round-trips through serde.

use tempfile::TempDir;
use turingosv4::bus::{BusConfig, TuringBus};
use turingosv4::economy::money::MicroCoin;
use turingosv4::kernel::Kernel;
use turingosv4::runtime::adapter::{
    genesis_with_balances, make_synthetic_task_open, make_synthetic_worktx,
};
use turingosv4::runtime::run_summary::RunSummary;
use turingosv4::runtime::{build_chaintape_sequencer_with_initial_q, RuntimeChaintapeConfig};
use turingosv4::state::q_state::{AgentId, Hash, TxId};

fn fresh_config(tmp: &TempDir, run_id: &str) -> RuntimeChaintapeConfig {
    RuntimeChaintapeConfig {
        runtime_repo_path: tmp.path().join("runtime_repo"),
        cas_path: tmp.path().join("cas"),
        run_id: run_id.to_string(),
        queue_capacity: 16,
        resume_existing_chain: false,
    }
}

#[tokio::test]
async fn i92_end_to_end_run_summary_aggregates_l4_and_l4e() {
    let tmp = TempDir::new().expect("tempdir");
    let cfg = fresh_config(&tmp, "i92");
    let initial_q = genesis_with_balances(&[(
        AgentId("sponsor-i92".into()),
        MicroCoin::from_coin(100).unwrap(),
    )]);
    let bundle = build_chaintape_sequencer_with_initial_q(&cfg, initial_q)
        .expect("bootstrap with seeded balance");
    let kernel = Kernel::new();
    let bus = TuringBus::with_sequencer(kernel, BusConfig::default(), bundle.sequencer.clone());

    // ≥1 accepted (TaskOpen) + ≥1 rejected (zero-stake WorkTx).
    bus.submit_typed_tx(make_synthetic_task_open(
        "task-i92",
        "sponsor-i92",
        Hash::ZERO,
        "i92-1",
    ))
    .await
    .expect("submit TaskOpen");
    bus.submit_typed_tx(make_synthetic_worktx(
        "task-i92",
        "agent-i92",
        Hash::ZERO,
        0,
        "i92-rej",
        true,
    ))
    .await
    .expect("submit zero-stake WorkTx");

    bundle.shutdown().await.expect("shutdown");
    drop(bus);

    let summary = RunSummary::from_chaintape(
        &cfg.runtime_repo_path,
        &cfg.cas_path,
        "i92",
        7, // failed_branch_count caller-supplied
        2, // rollback_count caller-supplied
    )
    .expect("build summary");

    assert_eq!(summary.run_id, "i92");
    assert_eq!(summary.failed_branch_count, 7);
    assert_eq!(summary.rollback_count, 2);
    assert!(summary.l4_entries >= 1, "≥1 L4 entry expected");
    assert!(summary.l4e_entries >= 1, "≥1 L4.E entry expected");
    assert_eq!(summary.tx_count, summary.l4_entries + summary.l4e_entries);
    assert!(
        summary
            .accepted_tx_ids
            .contains(&TxId("taskopen-task-i92-i92-1".into())),
        "accepted set should include TaskOpen"
    );
    assert!(
        summary
            .rejected_tx_ids
            .contains(&TxId("worktx-task-i92-i92-rej".into())),
        "rejected set should include zero-stake WorkTx"
    );
    assert!(
        !summary.candidate_proposal_cids.is_empty(),
        "candidate proposal CIDs populated"
    );

    // Write to file + reload via serde for byte-stable shape.
    let path = tmp.path().join("run_summary.json");
    summary.write_json(&path).expect("write json");
    assert!(path.exists());
    let raw = std::fs::read_to_string(&path).unwrap();
    let reloaded: RunSummary = serde_json::from_str(&raw).unwrap();
    assert_eq!(reloaded, summary);
}

#[tokio::test]
async fn i92b_empty_chain_produces_zero_run_summary() {
    let tmp = TempDir::new().expect("tempdir");
    let cfg = fresh_config(&tmp, "i92b");
    use turingosv4::runtime::build_chaintape_sequencer;
    let bundle = build_chaintape_sequencer(&cfg).expect("bootstrap");
    bundle.shutdown().await.expect("shutdown");

    let summary = RunSummary::from_chaintape(&cfg.runtime_repo_path, &cfg.cas_path, "i92b", 0, 0)
        .expect("build summary");

    assert_eq!(summary.l4_entries, 0);
    assert_eq!(summary.l4e_entries, 0);
    assert_eq!(summary.tx_count, 0);
    assert!(summary.accepted_tx_ids.is_empty());
    assert!(summary.rejected_tx_ids.is_empty());
    assert!(summary.candidate_proposal_cids.is_empty());
}

#[test]
fn i92c_run_summary_canonical_path_is_run_summary_json() {
    let tmp = TempDir::new().unwrap();
    let s = RunSummary {
        run_id: "canon".into(),
        tx_count: 0,
        failed_branch_count: 0,
        rollback_count: 0,
        accepted_tx_ids: vec![],
        rejected_tx_ids: vec![],
        candidate_proposal_cids: vec![],
        l4_entries: 0,
        l4e_entries: 0,
    };
    s.write_canonical(tmp.path()).unwrap();
    assert!(tmp.path().join("run_summary.json").exists());
}
