//! TB-6 Atom 4 — `verify_chaintape` integration tests.
//!
//! Architect ruling 2026-05-01 § 3.6 Atom 4: the replay verifier MUST be
//! demonstrably end-to-end — a fresh production-mode bootstrap + synthetic
//! TaskOpen + zero-stake WorkTx → shutdown drain → re-open the repo from
//! disk → `verify_chaintape` reports all 7 architect-mandated boolean
//! indicators true.
//!
//! - I90: end-to-end happy path (≥1 L4 + ≥1 L4.E + all indicators pass).
//! - I90b: empty chain (no submissions) — replay reports zero entries +
//!   all indicators true (vacuous chain integrity holds).
//! - I90c: tamper detection — corrupt the on-disk pinned_pubkey hex →
//!   verifier reports `system_signatures_verified=false`.
//!
//! Charter: handover/tracer_bullets/TB-6_charter_2026-05-01.md
//! Atom 3 smoke evidence: handover/evidence/tb_6_chaintape_smoke_2026-05-01/

use tempfile::TempDir;
use turingosv4::bus::{BusConfig, TuringBus};
use turingosv4::kernel::Kernel;
use turingosv4::runtime::adapter::{make_synthetic_task_open, make_synthetic_worktx};
use turingosv4::runtime::verify::{verify_chaintape, VerifyOptions};
use turingosv4::runtime::{build_chaintape_sequencer, RuntimeChaintapeConfig};
use turingosv4::state::q_state::Hash;

fn fresh_config(tmp: &TempDir, run_id: &str) -> RuntimeChaintapeConfig {
    RuntimeChaintapeConfig {
        runtime_repo_path: tmp.path().join("runtime_repo"),
        cas_path: tmp.path().join("cas"),
        run_id: run_id.to_string(),
        queue_capacity: 16,
    }
}

#[tokio::test]
async fn i90_end_to_end_taskopen_plus_zero_stake_worktx_replay_passes_all_indicators() {
    let tmp = TempDir::new().expect("tempdir");
    let cfg = fresh_config(&tmp, "i90");
    let bundle = build_chaintape_sequencer(&cfg).expect("bootstrap");
    let kernel = Kernel::new();
    let bus = TuringBus::with_sequencer(kernel, BusConfig::default(), bundle.sequencer.clone());

    // Submit a synthetic TaskOpen → expected to land as ≥1 L4 entry.
    let task_open = make_synthetic_task_open("task-i90", "sponsor-i90", Hash::ZERO, "i90-1");
    bus.submit_typed_tx(task_open)
        .await
        .expect("submit TaskOpen");

    // Submit a zero-stake WorkTx → expected to land as ≥1 L4.E rejection.
    let bad_worktx =
        make_synthetic_worktx("task-i90", "agent-i90", Hash::ZERO, 0, "i90-rej", true);
    bus.submit_typed_tx(bad_worktx)
        .await
        .expect("submit zero-stake WorkTx");

    bundle.shutdown().await.expect("shutdown");
    drop(bus);

    let report =
        verify_chaintape(&cfg.runtime_repo_path, &cfg.cas_path, &VerifyOptions::default())
            .expect("verify");

    assert!(
        report.l4_entries >= 1,
        "≥1 L4 entry expected; got {}",
        report.l4_entries
    );
    assert!(
        report.l4e_entries >= 1,
        "≥1 L4.E entry expected; got {}",
        report.l4e_entries
    );
    assert!(report.ledger_root_verified, "ledger_root_verified");
    assert!(
        report.system_signatures_verified,
        "system_signatures_verified"
    );
    assert!(report.state_reconstructed, "state_reconstructed");
    assert!(
        report.economic_state_reconstructed,
        "economic_state_reconstructed"
    );
    assert!(report.cas_payloads_retrievable, "cas_payloads_retrievable");
    assert!(report.all_indicators_pass());
    assert_eq!(report.run_id, "i90");
    assert_eq!(report.epoch, 1);
    assert!(report.detail.head_commit_oid_hex.is_some());
    assert!(report.detail.final_state_root_hex.is_some());
    assert!(report.detail.final_ledger_root_hex.is_some());
    assert!(report.detail.replay_failure.is_none());
    assert!(!report.detail.initial_q_state_loaded_from_disk);
}

#[tokio::test]
async fn i90b_empty_chain_replay_reports_zero_entries_and_all_indicators_pass() {
    let tmp = TempDir::new().expect("tempdir");
    let cfg = fresh_config(&tmp, "i90b");
    let bundle = build_chaintape_sequencer(&cfg).expect("bootstrap");
    bundle.shutdown().await.expect("shutdown");

    let report =
        verify_chaintape(&cfg.runtime_repo_path, &cfg.cas_path, &VerifyOptions::default())
            .expect("verify");

    assert_eq!(report.l4_entries, 0);
    assert_eq!(report.l4e_entries, 0);
    // Vacuous chain integrity: zero entries → no divergence possible.
    assert!(report.all_indicators_pass());
    assert!(report.detail.head_commit_oid_hex.is_none());
}

#[tokio::test]
async fn i90c_tampered_pinned_pubkey_breaks_signature_verification() {
    let tmp = TempDir::new().expect("tempdir");
    let cfg = fresh_config(&tmp, "i90c");
    let bundle = build_chaintape_sequencer(&cfg).expect("bootstrap");
    let kernel = Kernel::new();
    let bus = TuringBus::with_sequencer(kernel, BusConfig::default(), bundle.sequencer.clone());

    let task_open = make_synthetic_task_open("task-i90c", "sponsor-i90c", Hash::ZERO, "i90c-1");
    bus.submit_typed_tx(task_open)
        .await
        .expect("submit TaskOpen");
    bundle.shutdown().await.expect("shutdown");
    drop(bus);

    // Sanity: untampered chain passes.
    let pre = verify_chaintape(&cfg.runtime_repo_path, &cfg.cas_path, &VerifyOptions::default())
        .expect("pre-tamper verify");
    assert!(pre.system_signatures_verified);

    // Flip a single byte in the pinned-pubkey hex string. This re-keys the
    // verifier; signatures recorded under the original key will fail.
    let manifest_path = cfg.runtime_repo_path.join("pinned_pubkeys.json");
    let raw = std::fs::read_to_string(&manifest_path).expect("read manifest");
    let mut parsed: serde_json::Value = serde_json::from_str(&raw).expect("parse");
    let pubkeys = parsed["pubkeys"].as_array_mut().expect("pubkeys array");
    let entry = pubkeys[0].as_object_mut().expect("pubkeys[0] object");
    let mut hex = entry["pubkey_hex"]
        .as_str()
        .expect("pubkey_hex string")
        .to_string();
    // Flip the lowest nibble of the first byte (e.g. "cc..." → "cd...").
    let first = hex.chars().nth(1).unwrap();
    let flipped = match first {
        '0'..='8' | 'a'..='e' => char::from_u32(first as u32 + 1).unwrap(),
        _ => '0',
    };
    hex.replace_range(1..2, &flipped.to_string());
    entry.insert(
        "pubkey_hex".into(),
        serde_json::Value::String(hex),
    );
    std::fs::write(&manifest_path, serde_json::to_string_pretty(&parsed).unwrap())
        .expect("write tampered manifest");

    let post =
        verify_chaintape(&cfg.runtime_repo_path, &cfg.cas_path, &VerifyOptions::default())
            .expect("verify with tampered pubkey");
    assert!(
        !post.system_signatures_verified,
        "tampered pubkey must break signature verification (got {:?})",
        post.detail.replay_failure
    );
    assert!(!post.all_indicators_pass());
}
