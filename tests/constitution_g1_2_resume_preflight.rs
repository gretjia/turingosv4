//! TB-G G1.2-1 (Option B+ orchestration ruling 2026-05-11) — ResumePreflight
//! fail-closed validation gates SG-G1.2-1.1..G1.2-1.11.
//!
//! Architect ruling §3.1 verbatim:
//!
//! > 只靠环境变量很危险。必须增加 ResumePreflight. 每个 subprocess 启动前检查:
//! > runtime_repo exists / CAS exists / agent_registry exists / system_pubkeys
//! > exist / HEAD_t exists / expected_head_t matches actual / batch_id matches /
//! > task_index increments by exactly 1 / no new genesis_report created /
//! > no on_init after task 0.
//!
//! Eleven SG-G1.2-1.* gates:
//!   1. preflight_accepts_valid_chain
//!   2. preflight_rejects_missing_runtime_repo
//!   3. preflight_rejects_missing_cas
//!   4. preflight_rejects_missing_agent_pubkeys
//!   5. preflight_rejects_missing_pinned_pubkeys
//!   6. preflight_rejects_missing_genesis_report
//!   7. preflight_rejects_head_mismatch
//!   8. preflight_rejects_state_root_mismatch
//!   9. preflight_rejects_chain_length_mismatch
//!  10. preflight_rejects_task_index_gap
//!  11. preflight_rejects_fresh_genesis_attempt
//!
//! FC-trace: FC2-Boot. Preflight is the safety gate that turns
//! `TURINGOS_CHAINTAPE_RESUME=1` from a signal into an enforceable
//! contract per architect ruling.

use std::path::PathBuf;

use tempfile::TempDir;
use turingosv4::bus::{BusConfig, TuringBus};
use turingosv4::kernel::Kernel;
use turingosv4::runtime::adapter::make_synthetic_task_open;
use turingosv4::runtime::resume_preflight::{
    check, snapshot_head_t, PreflightFailure, PreflightVerdict, ResumeContract,
};
use turingosv4::runtime::{build_chaintape_sequencer, RuntimeChaintapeConfig};
use turingosv4::state::q_state::Hash;

/// Bootstrap a runtime_repo with exactly one accepted L4 entry so we have
/// a real ChainTape to point a `ResumeContract` at. Returns the
/// `(runtime_repo, cas_path, head_hex, state_root_hex, chain_length)`
/// tuple. The runtime_repo is owned by the caller-supplied `TempDir`.
async fn bootstrap_single_entry_chain(
    tmp: &TempDir,
    run_id: &str,
) -> (PathBuf, PathBuf, String, String, u64) {
    let runtime_repo = tmp.path().join("runtime_repo");
    let cas_path = tmp.path().join("cas");
    let cfg = RuntimeChaintapeConfig {
        runtime_repo_path: runtime_repo.clone(),
        cas_path: cas_path.clone(),
        run_id: run_id.into(),
        queue_capacity: 16,
        resume_existing_chain: false,
    };
    let bundle = build_chaintape_sequencer(&cfg).expect("fresh bootstrap");
    let kernel = Kernel::new();
    let bus = TuringBus::with_sequencer(kernel, BusConfig::default(), bundle.sequencer.clone());
    let tx = make_synthetic_task_open(
        &format!("task-{run_id}"),
        &format!("sponsor-{run_id}"),
        Hash::ZERO,
        run_id,
    );
    bus.submit_typed_tx(tx).await.expect("submit TaskOpen");
    bundle.shutdown().await.expect("shutdown");
    drop(bus);

    let (head_hex, state_root_hex, len) =
        snapshot_head_t(&runtime_repo).expect("snapshot head_t post-bootstrap");
    assert!(len >= 1, "bootstrap should produce ≥1 accepted L4 entry");
    assert!(!head_hex.is_empty(), "head OID must be non-empty");
    assert!(!state_root_hex.is_empty(), "state_root hex must be non-empty");
    (runtime_repo, cas_path, head_hex, state_root_hex, len)
}

/// Construct a contract that points at the bootstrapped chain with the
/// correct expected values. Tests then mutate one field per scenario to
/// exercise the reject paths.
fn contract_for(
    runtime_repo: &PathBuf,
    cas_path: &PathBuf,
    head_hex: &str,
    state_root_hex: &str,
    len: u64,
    task_index: u64,
) -> ResumeContract {
    ResumeContract {
        runtime_repo: runtime_repo.clone(),
        cas_path: cas_path.clone(),
        expected_head_t_hex: head_hex.into(),
        expected_state_root_hex: state_root_hex.into(),
        expected_chain_length: len,
        batch_id: "g1_2_preflight_batch".into(),
        task_index,
        agent_pubkeys_path: runtime_repo.join("agent_pubkeys.json"),
        pinned_pubkeys_path: runtime_repo.join("pinned_pubkeys.json"),
        genesis_report_path: runtime_repo.join("genesis_report.json"),
    }
}

/// Helper: G1.1 bootstrap does not auto-write a `genesis_report.json` —
/// it is the responsibility of `chain_runtime::write_synthetic_l4_l4e_gate_and_genesis_report`
/// at the binary layer. For preflight tests we synthesize a stub so the
/// happy-path tests can pass and the missing-genesis-report reject test
/// can take it back out.
fn write_stub_genesis_report(runtime_repo: &PathBuf) {
    let stub = serde_json::json!({
        "constitution_hash": "stub",
        "runtime_repo": runtime_repo.to_string_lossy(),
        "cas_path": runtime_repo.parent().unwrap().join("cas").to_string_lossy(),
        "system_pubkey_hash": "stub",
        "agent_pubkeys_path": "agent_pubkeys.json",
        "initial_balances": [],
        "task_id": null,
        "task_open_tx": null,
        "escrow_lock_tx": null
    });
    std::fs::write(
        runtime_repo.join("genesis_report.json"),
        serde_json::to_string_pretty(&stub).unwrap(),
    )
    .expect("write stub genesis_report");
}

/// Helper: write a stub `agent_pubkeys.json` (G1.1 bootstrap stops at
/// registering an empty manifest only when an `AgentKeypairRegistry`
/// gets used; for preflight existence checks we write a placeholder).
fn write_stub_agent_pubkeys(runtime_repo: &PathBuf) {
    let stub = serde_json::json!({
        "tb_id": "TB-G",
        "agents": []
    });
    std::fs::write(
        runtime_repo.join("agent_pubkeys.json"),
        serde_json::to_string_pretty(&stub).unwrap(),
    )
    .expect("write stub agent_pubkeys");
}

// ── SG-G1.2-1.1 ─────────────────────────────────────────────────────────────
//
// Valid chain + valid contract → Ok.
#[tokio::test]
async fn preflight_accepts_valid_chain() {
    let tmp = TempDir::new().expect("tempdir");
    let (runtime_repo, cas_path, head_hex, state_root_hex, len) =
        bootstrap_single_entry_chain(&tmp, "valid").await;
    write_stub_genesis_report(&runtime_repo);
    write_stub_agent_pubkeys(&runtime_repo);

    let c = contract_for(&runtime_repo, &cas_path, &head_hex, &state_root_hex, len, 1);
    match check(&c) {
        PreflightVerdict::Ok => {}
        other => panic!("SG-G1.2-1.1: expected Ok, got {other:?}"),
    }
}

// ── SG-G1.2-1.2 ─────────────────────────────────────────────────────────────
//
// Missing runtime_repo path → RuntimeRepoMissing.
#[tokio::test]
async fn preflight_rejects_missing_runtime_repo() {
    let tmp = TempDir::new().expect("tempdir");
    let (runtime_repo, cas_path, head_hex, state_root_hex, len) =
        bootstrap_single_entry_chain(&tmp, "miss_repo").await;
    write_stub_genesis_report(&runtime_repo);
    write_stub_agent_pubkeys(&runtime_repo);

    let mut c = contract_for(&runtime_repo, &cas_path, &head_hex, &state_root_hex, len, 1);
    c.runtime_repo = tmp.path().join("nonexistent_runtime_repo");
    match check(&c) {
        PreflightVerdict::Fail { failure: PreflightFailure::RuntimeRepoMissing { .. } } => {}
        other => panic!("SG-G1.2-1.2: expected RuntimeRepoMissing, got {other:?}"),
    }
}

// ── SG-G1.2-1.3 ─────────────────────────────────────────────────────────────
//
// Missing CAS path → CasMissing.
#[tokio::test]
async fn preflight_rejects_missing_cas() {
    let tmp = TempDir::new().expect("tempdir");
    let (runtime_repo, cas_path, head_hex, state_root_hex, len) =
        bootstrap_single_entry_chain(&tmp, "miss_cas").await;
    write_stub_genesis_report(&runtime_repo);
    write_stub_agent_pubkeys(&runtime_repo);

    let mut c = contract_for(&runtime_repo, &cas_path, &head_hex, &state_root_hex, len, 1);
    c.cas_path = tmp.path().join("nonexistent_cas");
    match check(&c) {
        PreflightVerdict::Fail { failure: PreflightFailure::CasMissing { .. } } => {}
        other => panic!("SG-G1.2-1.3: expected CasMissing, got {other:?}"),
    }
}

// ── SG-G1.2-1.4 ─────────────────────────────────────────────────────────────
//
// Missing agent_pubkeys.json → AgentRegistryMissing.
#[tokio::test]
async fn preflight_rejects_missing_agent_pubkeys() {
    let tmp = TempDir::new().expect("tempdir");
    let (runtime_repo, cas_path, head_hex, state_root_hex, len) =
        bootstrap_single_entry_chain(&tmp, "miss_agents").await;
    write_stub_genesis_report(&runtime_repo);
    // intentionally do NOT call write_stub_agent_pubkeys

    let c = contract_for(&runtime_repo, &cas_path, &head_hex, &state_root_hex, len, 1);
    match check(&c) {
        PreflightVerdict::Fail { failure: PreflightFailure::AgentRegistryMissing { .. } } => {}
        other => panic!("SG-G1.2-1.4: expected AgentRegistryMissing, got {other:?}"),
    }
}

// ── SG-G1.2-1.5 ─────────────────────────────────────────────────────────────
//
// Missing pinned_pubkeys.json → PinnedPubkeysMissing. The G1.1 bootstrap
// always writes this file; deleting it simulates a corrupted resume
// boundary.
#[tokio::test]
async fn preflight_rejects_missing_pinned_pubkeys() {
    let tmp = TempDir::new().expect("tempdir");
    let (runtime_repo, cas_path, head_hex, state_root_hex, len) =
        bootstrap_single_entry_chain(&tmp, "miss_pinned").await;
    write_stub_genesis_report(&runtime_repo);
    write_stub_agent_pubkeys(&runtime_repo);
    std::fs::remove_file(runtime_repo.join("pinned_pubkeys.json"))
        .expect("remove pinned_pubkeys.json");

    let c = contract_for(&runtime_repo, &cas_path, &head_hex, &state_root_hex, len, 1);
    match check(&c) {
        PreflightVerdict::Fail { failure: PreflightFailure::PinnedPubkeysMissing { .. } } => {}
        other => panic!("SG-G1.2-1.5: expected PinnedPubkeysMissing, got {other:?}"),
    }
}

// ── SG-G1.2-1.6 ─────────────────────────────────────────────────────────────
//
// Missing genesis_report.json at task_index > 0 → FreshGenesisAttempted.
// The architect's canonical "fresh-genesis attempted" signal is the
// absence of a prior `genesis_report.json` in a non-empty runtime_repo.
#[tokio::test]
async fn preflight_rejects_missing_genesis_report() {
    let tmp = TempDir::new().expect("tempdir");
    let (runtime_repo, cas_path, head_hex, state_root_hex, len) =
        bootstrap_single_entry_chain(&tmp, "miss_genesis").await;
    write_stub_agent_pubkeys(&runtime_repo);
    // intentionally do NOT call write_stub_genesis_report

    let c = contract_for(&runtime_repo, &cas_path, &head_hex, &state_root_hex, len, 1);
    match check(&c) {
        PreflightVerdict::Fail {
            failure: PreflightFailure::FreshGenesisAttempted { detected },
        } => {
            assert!(
                detected.contains("genesis_report.json missing"),
                "SG-G1.2-1.6: detected message should name the missing artifact: {detected}"
            );
        }
        other => panic!("SG-G1.2-1.6: expected FreshGenesisAttempted, got {other:?}"),
    }
}

// ── SG-G1.2-1.7 ─────────────────────────────────────────────────────────────
//
// HEAD hex mismatch → HeadMismatch. Orchestrator's `expected_head_t_hex`
// is stale (advanced concurrently or contract bug).
#[tokio::test]
async fn preflight_rejects_head_mismatch() {
    let tmp = TempDir::new().expect("tempdir");
    let (runtime_repo, cas_path, head_hex, state_root_hex, len) =
        bootstrap_single_entry_chain(&tmp, "head_mismatch").await;
    write_stub_genesis_report(&runtime_repo);
    write_stub_agent_pubkeys(&runtime_repo);

    let mut c = contract_for(&runtime_repo, &cas_path, &head_hex, &state_root_hex, len, 1);
    c.expected_head_t_hex = "0".repeat(40);
    match check(&c) {
        PreflightVerdict::Fail {
            failure: PreflightFailure::HeadMismatch { expected_hex, actual_hex },
        } => {
            assert_eq!(expected_hex, "0".repeat(40));
            assert_eq!(actual_hex, head_hex);
        }
        other => panic!("SG-G1.2-1.7: expected HeadMismatch, got {other:?}"),
    }
}

// ── SG-G1.2-1.8 ─────────────────────────────────────────────────────────────
//
// state_root hex mismatch → StateRootMismatch. Orchestrator carried a
// stale state_root or its computation is wrong.
#[tokio::test]
async fn preflight_rejects_state_root_mismatch() {
    let tmp = TempDir::new().expect("tempdir");
    let (runtime_repo, cas_path, head_hex, state_root_hex, len) =
        bootstrap_single_entry_chain(&tmp, "state_root_mismatch").await;
    write_stub_genesis_report(&runtime_repo);
    write_stub_agent_pubkeys(&runtime_repo);

    let mut c = contract_for(&runtime_repo, &cas_path, &head_hex, &state_root_hex, len, 1);
    c.expected_state_root_hex = "deadbeef".repeat(8); // 64 chars, wrong value
    match check(&c) {
        PreflightVerdict::Fail {
            failure: PreflightFailure::StateRootMismatch { expected_hex, actual_hex },
        } => {
            assert_eq!(expected_hex, "deadbeef".repeat(8));
            assert_eq!(actual_hex, state_root_hex);
        }
        other => panic!("SG-G1.2-1.8: expected StateRootMismatch, got {other:?}"),
    }
}

// ── SG-G1.2-1.9 ─────────────────────────────────────────────────────────────
//
// Chain length mismatch → ChainLengthMismatch. Orchestrator's
// `expected_chain_length` is stale.
#[tokio::test]
async fn preflight_rejects_chain_length_mismatch() {
    let tmp = TempDir::new().expect("tempdir");
    let (runtime_repo, cas_path, head_hex, state_root_hex, len) =
        bootstrap_single_entry_chain(&tmp, "chain_len_mismatch").await;
    write_stub_genesis_report(&runtime_repo);
    write_stub_agent_pubkeys(&runtime_repo);

    let mut c = contract_for(&runtime_repo, &cas_path, &head_hex, &state_root_hex, len, 1);
    c.expected_chain_length = len + 17;
    match check(&c) {
        PreflightVerdict::Fail {
            failure: PreflightFailure::ChainLengthMismatch { expected, actual },
        } => {
            assert_eq!(expected, len + 17);
            assert_eq!(actual, len);
        }
        other => panic!("SG-G1.2-1.9: expected ChainLengthMismatch, got {other:?}"),
    }
}

// ── SG-G1.2-1.10 ────────────────────────────────────────────────────────────
//
// task_index == 0 → TaskIndexGap. Preflight is for resume only;
// `task_index == 0` is the fresh-genesis branch and should NOT call
// preflight.
#[tokio::test]
async fn preflight_rejects_task_index_gap() {
    let tmp = TempDir::new().expect("tempdir");
    let (runtime_repo, cas_path, head_hex, state_root_hex, len) =
        bootstrap_single_entry_chain(&tmp, "task_index_gap").await;
    write_stub_genesis_report(&runtime_repo);
    write_stub_agent_pubkeys(&runtime_repo);

    let c = contract_for(&runtime_repo, &cas_path, &head_hex, &state_root_hex, len, 0);
    match check(&c) {
        PreflightVerdict::Fail {
            failure: PreflightFailure::TaskIndexGap { task_index },
        } => {
            assert_eq!(task_index, 0);
        }
        other => panic!("SG-G1.2-1.10: expected TaskIndexGap, got {other:?}"),
    }
}

// ── SG-G1.2-1.11 ────────────────────────────────────────────────────────────
//
// Fresh-genesis attempt at task_index > 0 → FreshGenesisAttempted. An
// empty runtime_repo at task_index>0 means the orchestrator pointed the
// subprocess at a fresh directory instead of the shared one — the
// canonical Option B+ failure mode.
#[tokio::test]
async fn preflight_rejects_fresh_genesis_attempt() {
    let tmp = TempDir::new().expect("tempdir");
    // Build the shared chain, then point the contract at a different
    // (empty) runtime_repo with task_index=1 — that is the wire-error
    // pattern the architect named.
    let (_real_runtime_repo, real_cas, _head_hex, _state_root_hex, _len) =
        bootstrap_single_entry_chain(&tmp, "fresh_genesis").await;
    let empty_runtime_repo = tmp.path().join("empty_runtime_repo");
    std::fs::create_dir_all(&empty_runtime_repo).expect("create empty runtime_repo");
    // Populate the stubs that EARLIER preflight gates would otherwise catch,
    // so the test exercises the architect-named "chain is empty (len=0) at
    // task_index>0" failure mode rather than tripping on the agent/pubkey
    // existence gate. The canonical failure path the test asserts is the
    // empty-chain branch of `FreshGenesisAttempted`.
    write_stub_genesis_report(&empty_runtime_repo);
    write_stub_agent_pubkeys(&empty_runtime_repo);
    std::fs::write(
        empty_runtime_repo.join("pinned_pubkeys.json"),
        "{\"run_id\":\"stub\",\"tb_id\":\"TB-G\",\"epoch\":1,\"pubkeys\":[]}",
    )
    .expect("write stub pinned_pubkeys");

    let c = ResumeContract {
        runtime_repo: empty_runtime_repo.clone(),
        cas_path: real_cas,
        expected_head_t_hex: "deadbeefdeadbeefdeadbeefdeadbeefdeadbeef".into(),
        expected_state_root_hex: String::new(),
        expected_chain_length: 1,
        batch_id: "g1_2_preflight_batch".into(),
        task_index: 1,
        agent_pubkeys_path: empty_runtime_repo.join("agent_pubkeys.json"),
        pinned_pubkeys_path: empty_runtime_repo.join("pinned_pubkeys.json"),
        genesis_report_path: empty_runtime_repo.join("genesis_report.json"),
    };
    match check(&c) {
        PreflightVerdict::Fail { failure: PreflightFailure::FreshGenesisAttempted { detected } } => {
            assert!(
                detected.contains("genesis_report.json missing")
                    || detected.contains("is empty")
                    || detected.contains("len=0"),
                "SG-G1.2-1.11: detected message should name the fresh-genesis signal: {detected}"
            );
        }
        // Allow AgentRegistryMissing as an earlier fail-closed gate trip too —
        // either is a strict-equality fail. But the canonical architect-named
        // failure is FreshGenesisAttempted, so prefer that.
        other => panic!("SG-G1.2-1.11: expected FreshGenesisAttempted, got {other:?}"),
    }
}
