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
        resume_existing_chain: false,
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
    // TB-7.7 D7 (2026-05-01): build_chaintape_sequencer_with_initial_q now
    // always persists initial_q to <runtime_repo>/initial_q_state.json so
    // verify_chaintape replay can pick up pre-seeded balances + open task
    // markets. The base build_chaintape_sequencer factory delegates here
    // with QState::genesis(), so the file is now also written for genesis
    // bundles — this assertion flipped from `false` to `true`. The
    // genesis-equivalent initial_q is replay-idempotent.
    assert!(report.detail.initial_q_state_loaded_from_disk);
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

// ════════════════════════════════════════════════════════════════════
// TB-7.6 / Codex audit cc7b3dd action #6 — disk-level tamper battery
// ════════════════════════════════════════════════════════════════════
//
// I90d/e/f extend the I90c pattern (pinned-pubkey tampering) to cover
// the remaining disk-level tamper surfaces flagged by the TB-6 Codex
// audit:
//
// - I90d — tamper a CAS index sidecar (`.turingos_cas_index.jsonl`) →
//   verify_chaintape returns Err(Cas) at open time (catches L4 entries
//   whose `tx_payload_cid` references CAS objects with mutated metadata).
// - I90e — tamper a rejections.jsonl row → RejectionEvidenceWriter chain
//   integrity check fails on open.
// - I90f — delete L4.E rejections.jsonl after successful chain → verify
//   trivially passes (empty L4.E is the legitimate empty-chain case);
//   THIS is the negative control proving the difference between "no
//   rejections" and "tampered rejections".
//
// TRACE_MATRIX FC1-N14: Class 2 production wire-up tamper hardening.

/// I90d — Tampering with `.turingos_cas_index.jsonl` causes
/// `verify_chaintape` to return Err at CAS-open time. (Pre-fix, this
/// was the EXACT symptom that runs 2 + 5 of the TB-7 real-LLM smoke
/// hit organically; here we synthesize it.)
#[tokio::test]
async fn i90d_tampered_cas_index_breaks_verify_chaintape() {
    let tmp = TempDir::new().expect("tempdir");
    let cfg = fresh_config(&tmp, "i90d");
    let bundle = build_chaintape_sequencer(&cfg).expect("bootstrap");
    let bus = TuringBus::with_sequencer(
        Kernel::new(),
        BusConfig::default(),
        bundle.sequencer.clone(),
    );
    let task_open = make_synthetic_task_open("task-i90d", "sponsor-i90d", Hash::ZERO, "i90d-1");
    bus.submit_typed_tx(task_open).await.expect("submit");
    bundle.shutdown().await.expect("shutdown");
    drop(bus);

    // Sanity: pre-tamper verify passes.
    let pre = verify_chaintape(&cfg.runtime_repo_path, &cfg.cas_path, &VerifyOptions::default())
        .expect("pre-tamper verify");
    assert!(pre.all_indicators_pass());

    // Tamper the CAS index sidecar by injecting trailing junk on the first
    // line — same shape as the runs-2/5 corruption (concatenation without
    // separator).
    let cas_index = cfg.cas_path.join(".turingos_cas_index.jsonl");
    let raw = std::fs::read_to_string(&cas_index).expect("read cas index");
    // Concatenate first line + a bogus extra JSON object on the same line.
    let mut lines: Vec<String> = raw.lines().map(|s| s.to_string()).collect();
    if lines.is_empty() {
        // Empty index — ensure there's at least one record before we tamper.
        panic!("expected ≥1 CAS index line on Atom 3 synthetic seed run");
    }
    lines[0].push_str(r#"{"cid":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],"backend_oid_hex":"deadbeef","object_type":"Generic","creator":"tamper","created_at_logical_t":99,"schema_id":null,"size_bytes":0}"#);
    std::fs::write(&cas_index, lines.join("\n") + "\n").expect("write tamper");

    let result =
        verify_chaintape(&cfg.runtime_repo_path, &cfg.cas_path, &VerifyOptions::default());
    assert!(
        result.is_err(),
        "tampered CAS index must break verify_chaintape at CAS-open time; got Ok({result:?})"
    );
}

/// I90e — Tampering with `rejections.jsonl` row hash breaks the L4.E
/// chain check at `RejectionEvidenceWriter::open_jsonl` time, surfaced
/// as `VerifyError::L4eOpen`.
#[tokio::test]
async fn i90e_tampered_l4e_row_breaks_chain_open() {
    let tmp = TempDir::new().expect("tempdir");
    let cfg = fresh_config(&tmp, "i90e");
    let bundle = build_chaintape_sequencer(&cfg).expect("bootstrap");
    let bus = TuringBus::with_sequencer(
        Kernel::new(),
        BusConfig::default(),
        bundle.sequencer.clone(),
    );
    // Submit a synthetic zero-stake WorkTx → L4.E row.
    let bad_work = make_synthetic_worktx(
        "task-i90e",
        "agent-i90e",
        Hash::ZERO,
        0,
        "i90e-1",
        true,
    );
    bus.submit_typed_tx(bad_work).await.expect("submit");
    bundle.shutdown().await.expect("shutdown");
    drop(bus);

    let pre = verify_chaintape(&cfg.runtime_repo_path, &cfg.cas_path, &VerifyOptions::default())
        .expect("pre-tamper verify");
    assert!(pre.l4e_entries >= 1, "expected ≥1 L4.E row to tamper");

    // Mutate the row's `prev_hash` field to break the spine.
    let rejections_path = cfg.runtime_repo_path.join("rejections.jsonl");
    let raw = std::fs::read_to_string(&rejections_path).expect("read rejections");
    let mut lines: Vec<String> = raw.lines().map(|s| s.to_string()).collect();
    let mut row: serde_json::Value =
        serde_json::from_str(&lines[0]).expect("parse rejection row");
    // Replace prev_hash with all-ones (or whatever != recorded value).
    row["prev_hash"] = serde_json::Value::Array(
        (0..32u32).map(|_| serde_json::Value::Number(1u8.into())).collect(),
    );
    lines[0] = serde_json::to_string(&row).expect("reserialize");
    std::fs::write(&rejections_path, lines.join("\n") + "\n").expect("write tamper");

    let result =
        verify_chaintape(&cfg.runtime_repo_path, &cfg.cas_path, &VerifyOptions::default());
    assert!(
        result.is_err(),
        "tampered L4.E row must break verify_chaintape at L4.E-open time; got Ok({result:?})"
    );
}

/// I90f — DELETING `rejections.jsonl` (negative control) is **NOT**
/// the same as tampering: an absent file means "no rejections" and
/// `verify_chaintape` treats it as an empty L4.E writer (legitimate
/// empty-chain case). This test pins the difference between
/// "honest absent" and "tampered present" so future refactors don't
/// accidentally treat absence as failure (which would break legacy
/// pre-Atom-1.2 evidence dirs).
#[tokio::test]
async fn i90f_absent_l4e_is_legitimate_empty_chain_not_tamper() {
    let tmp = TempDir::new().expect("tempdir");
    let cfg = fresh_config(&tmp, "i90f");
    let bundle = build_chaintape_sequencer(&cfg).expect("bootstrap");
    let bus = TuringBus::with_sequencer(
        Kernel::new(),
        BusConfig::default(),
        bundle.sequencer.clone(),
    );
    let task_open = make_synthetic_task_open("task-i90f", "sponsor-i90f", Hash::ZERO, "i90f-1");
    bus.submit_typed_tx(task_open).await.expect("submit");
    bundle.shutdown().await.expect("shutdown");
    drop(bus);

    // Delete rejections.jsonl. (Production systems should never do this;
    // the test exists to pin the "absence is legitimate" semantics.)
    let rejections_path = cfg.runtime_repo_path.join("rejections.jsonl");
    if rejections_path.exists() {
        std::fs::remove_file(&rejections_path).expect("remove rejections");
    }

    let report = verify_chaintape(&cfg.runtime_repo_path, &cfg.cas_path, &VerifyOptions::default())
        .expect("absent L4.E is legitimate empty-chain case");
    assert_eq!(report.l4e_entries, 0);
    assert!(report.all_indicators_pass(), "absent L4.E should NOT fail any indicator");
}
