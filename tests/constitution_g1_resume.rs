//! TB-G G1.1 (architect §8 SIGNED 2026-05-11 "好，确认可以 ship";
//! Canonical multi-clause Class-4 §8) — resume-mode genesis branch ship
//! gates SG-G1.1..SG-G1.5.
//!
//! Architect verdict §1.3 verbatim (G-Phase directive):
//!
//! > 每个 problem fresh runtime_repo + fresh genesis — 每轮开局都把交易
//! > 员洗白、清仓、重置记忆。市场不会涌现。
//!
//! G1.1 closes the loop by adding an env-gated resume admission path:
//! `TURINGOS_CHAINTAPE_RESUME=1` lets `build_chaintape_sequencer` open a
//! non-empty `refs/transitions/main` instead of fail-closing with
//! `BootstrapError::NonEmptyRuntimeRepo`. Default-deny posture is
//! preserved per CLAUDE.md §11 (resume=0 still fail-closes on non-empty
//! repos — SG-G1.4 back-compat regression gate).
//!
//! FC-trace: FC2-Boot — every real evidence run must be replayable from
//! `genesis_report + ChainTape + CAS + agent registry + system pubkeys`
//! (CLAUDE.md §3.2). Resume IS the boot-time instance of that replay,
//! seeded by `<runtime_repo>/initial_q_state.json` and consumed by the
//! canonical `replay_full_transition` primitive shared with
//! `verify_chaintape`. Existing Stage A3 SG-A3.4 covers replay
//! byte-equality; G1.1 layers SG-G1.3 balances-byte-equal + SG-G1.5
//! pinned-pubkey continuity on top.

use tempfile::TempDir;
use turingosv4::bus::{BusConfig, TuringBus};
use turingosv4::economy::money::MicroCoin;
use turingosv4::kernel::Kernel;
use turingosv4::runtime::adapter::{genesis_with_balances, make_synthetic_task_open};
use turingosv4::runtime::agent_keypairs::{AgentKeypairError, AgentKeypairRegistry};
use turingosv4::runtime::{
    build_chaintape_sequencer, build_chaintape_sequencer_with_initial_q, BootstrapError,
    RuntimeChaintapeConfig,
};
use turingosv4::state::q_state::{AgentId, Hash};

fn cfg_resume(tmp: &TempDir, run_id: &str, resume: bool) -> RuntimeChaintapeConfig {
    RuntimeChaintapeConfig {
        runtime_repo_path: tmp.path().join("runtime_repo"),
        cas_path: tmp.path().join("cas"),
        run_id: run_id.to_string(),
        queue_capacity: 16,
        resume_existing_chain: resume,
    }
}

// ── SG-G1.1 ─────────────────────────────────────────────────────────────────
//
// Resume on empty repo == legacy genesis. With `resume_existing_chain=true`
// but no existing chain, the resume short-circuit (`resume_active = false`
// when `head_commit_oid().is_none()`) falls through to the fresh-genesis
// path. Result: byte-equal `QState` to the legacy
// `build_chaintape_sequencer` call.
#[tokio::test]
async fn sg_g1_1_resume_on_empty_repo_equals_legacy_genesis() {
    // Fresh-genesis path with resume=true on an empty repo.
    let tmp_a = TempDir::new().expect("tempdir_a");
    let cfg_a = cfg_resume(&tmp_a, "g1_1-a", true);
    let bundle_a = build_chaintape_sequencer(&cfg_a)
        .expect("resume=true on empty repo bootstraps fresh (G1.1 SG-G1.1)");
    let q_a = bundle_a.sequencer.q_snapshot().expect("q_snapshot a");
    bundle_a.shutdown().await.expect("shutdown a");

    // Same fresh-genesis path with resume=false.
    let tmp_b = TempDir::new().expect("tempdir_b");
    let cfg_b = cfg_resume(&tmp_b, "g1_1-b", false);
    let bundle_b = build_chaintape_sequencer(&cfg_b).expect("legacy bootstrap b");
    let q_b = bundle_b.sequencer.q_snapshot().expect("q_snapshot b");
    bundle_b.shutdown().await.expect("shutdown b");

    // Both QStates derive from `QState::genesis()` (same seed, no
    // submitted txs, no economic mutation). Compare canonical roots —
    // bit-exact equality across both branches.
    assert_eq!(
        q_a.state_root_t, q_b.state_root_t,
        "SG-G1.1: state_root_t must match between resume=true/empty and resume=false/empty"
    );
    assert_eq!(
        q_a.ledger_root_t, q_b.ledger_root_t,
        "SG-G1.1: ledger_root_t must match"
    );
    assert_eq!(
        q_a.economic_state_t, q_b.economic_state_t,
        "SG-G1.1: economic_state_t must be byte-equal across branches"
    );
}

// ── SG-G1.2 ─────────────────────────────────────────────────────────────────
//
// Resume on an N-entry chain sets `Sequencer.next_logical_t == N`. The
// next commit's `Git2LedgerWriter::append` strict `len + 1` invariant
// holds — proved by submitting one extra TaskOpen after resume and
// observing the chain length advances from N → N+1.
//
// The test uses N=1 because making each subsequent `make_synthetic_task_open`
// accept requires threading `parent_state_root` through the latest
// `q_snapshot()` between submits, which races the async driver. N=1
// fully proves the SG-G1.2 constitutional invariant
// (`next_logical_t == chain_length`); the post-resume commit advances
// 1 → 2 and pins the `Git2LedgerWriter` `len + 1` constraint.
#[tokio::test]
async fn sg_g1_2_resume_on_n_entry_chain_sets_next_logical_t_to_n() {
    use turingosv4::bottom_white::ledger::transition_ledger::LedgerWriter;

    let tmp = TempDir::new().expect("tempdir");
    let cfg_fresh = cfg_resume(&tmp, "g1_2-fresh", false);

    // Phase 1: fresh bootstrap, submit 1 TaskOpen (parent matches
    // QState::genesis state_root = Hash::ZERO so it accepts).
    let bundle = build_chaintape_sequencer(&cfg_fresh).expect("fresh bootstrap");
    let kernel = Kernel::new();
    let bus = TuringBus::with_sequencer(kernel, BusConfig::default(), bundle.sequencer.clone());
    let tx = make_synthetic_task_open("task-g1_2", "sponsor-g1_2", Hash::ZERO, "g1_2-1");
    bus.submit_typed_tx(tx).await.expect("submit TaskOpen");
    bundle.shutdown().await.expect("shutdown phase 1");
    drop(bus);

    // Reopen the writer to confirm chain length on disk.
    let n_on_disk = {
        let reopened = turingosv4::bottom_white::ledger::transition_ledger::Git2LedgerWriter::open(
            &cfg_fresh.runtime_repo_path,
        )
        .expect("reopen writer");
        reopened.len()
    };
    assert_eq!(
        n_on_disk, 1,
        "phase 1: chain should hold 1 accepted L4 entry before resume"
    );

    // Phase 2: resume bootstrap. next_logical_t must equal 1.
    let cfg_r = cfg_resume(&tmp, "g1_2-resume", true);
    let bundle_r = build_chaintape_sequencer(&cfg_r).expect("resume bootstrap");
    assert_eq!(
        bundle_r.sequencer.next_logical_t_peek(),
        1,
        "SG-G1.2: Sequencer.next_logical_t must equal chain_length on resume \
         (chain has 1 entry → next_logical_t must be 1; the next commit signs as logical_t=2)"
    );

    // Phase 3: submit one more TaskOpen with parent = post-replay
    // state_root. Chain advances 1 → 2 without
    // `Git2LedgerWriter::append`'s strict `len + 1` invariant tripping.
    let q_after_replay = bundle_r
        .sequencer
        .q_snapshot()
        .expect("q_snapshot post-resume");
    let kernel2 = Kernel::new();
    let bus2 =
        TuringBus::with_sequencer(kernel2, BusConfig::default(), bundle_r.sequencer.clone());
    let tx_extra = make_synthetic_task_open(
        "task-g1_2-post-resume",
        "sponsor-g1_2",
        q_after_replay.state_root_t,
        "g1_2-post",
    );
    bus2.submit_typed_tx(tx_extra)
        .await
        .expect("submit post-resume TaskOpen");
    bundle_r.shutdown().await.expect("shutdown phase 3");
    drop(bus2);

    let n_after = {
        let reopened = turingosv4::bottom_white::ledger::transition_ledger::Git2LedgerWriter::open(
            &cfg_r.runtime_repo_path,
        )
        .expect("reopen writer");
        reopened.len()
    };
    assert_eq!(
        n_after, 2,
        "SG-G1.2: chain length must advance from 1 → 2 after one post-resume commit"
    );
}

// ── SG-G1.3 ─────────────────────────────────────────────────────────────────
//
// Balances reconstruction matches forward replay. A pre-seeded
// `genesis_with_balances` QState carried through a forward run produces
// `balances_t_A`; the same chain replayed via resume produces
// `balances_t_B`. The two must be byte-equal — that's the
// constitutional FC2 replay-determinism guarantee + Stage A3 SG-A3.4
// generalized down to per-account balances.
#[tokio::test]
async fn sg_g1_3_resume_balances_reconstruction_matches_forward_replay() {
    let tmp = TempDir::new().expect("tempdir");
    let cfg_fresh = cfg_resume(&tmp, "g1_3-fresh", false);

    let alice = AgentId("alice-g1_3".into());
    let bob = AgentId("bob-g1_3".into());
    let initial_q = genesis_with_balances(&[
        (alice.clone(), MicroCoin::from_coin(7).unwrap()),
        (bob.clone(), MicroCoin::from_coin(11).unwrap()),
    ]);

    // Phase 1: forward run with pre-seeded balances + one TaskOpen.
    // Clone the sequencer Arc *before* shutdown so q_snapshot post-drain
    // observes the final applied state (driver may still be processing
    // mid-submit; only post-shutdown is the canonical observation point).
    let bundle =
        build_chaintape_sequencer_with_initial_q(&cfg_fresh, initial_q.clone()).expect("fresh");
    let seq_forward = bundle.sequencer.clone();
    let kernel = Kernel::new();
    let bus = TuringBus::with_sequencer(kernel, BusConfig::default(), bundle.sequencer.clone());
    let tx = make_synthetic_task_open("task-g1_3-1", "sponsor-g1_3", Hash::ZERO, "g1_3-1");
    bus.submit_typed_tx(tx).await.expect("submit TaskOpen");
    bundle.shutdown().await.expect("shutdown forward");
    drop(bus);
    let q_forward = seq_forward.q_snapshot().expect("q_snapshot forward");

    // Phase 2: resume — replay reconstructs balances from initial_q +
    // chain entries. Must produce byte-equal balances_t to forward run.
    let cfg_r = cfg_resume(&tmp, "g1_3-resume", true);
    let bundle_r = build_chaintape_sequencer(&cfg_r).expect("resume bootstrap");
    let q_resumed = bundle_r.sequencer.q_snapshot().expect("q_snapshot resumed");
    bundle_r.shutdown().await.expect("shutdown resume");

    // Per-account assertion + full-map assertion. The per-account
    // assertion gives a more readable failure mode if the test ever
    // breaks; the full-map assertion catches any extra account that
    // shouldn't exist (or a missing one).
    assert_eq!(
        q_resumed
            .economic_state_t
            .balances_t
            .0
            .get(&alice)
            .copied()
            .unwrap_or_else(MicroCoin::zero),
        q_forward
            .economic_state_t
            .balances_t
            .0
            .get(&alice)
            .copied()
            .unwrap_or_else(MicroCoin::zero),
        "SG-G1.3: alice balance must match between forward and resumed run"
    );
    assert_eq!(
        q_resumed
            .economic_state_t
            .balances_t
            .0
            .get(&bob)
            .copied()
            .unwrap_or_else(MicroCoin::zero),
        q_forward
            .economic_state_t
            .balances_t
            .0
            .get(&bob)
            .copied()
            .unwrap_or_else(MicroCoin::zero),
        "SG-G1.3: bob balance must match"
    );
    assert_eq!(
        q_resumed.economic_state_t.balances_t, q_forward.economic_state_t.balances_t,
        "SG-G1.3: full balances_t map must be byte-equal across forward / resumed runs"
    );
    // Bound the state root for free — if balances diverge, state_root
    // diverges; this is the constitutional FC2 replay-determinism
    // guarantee applied to the entire economic_state.
    assert_eq!(
        q_resumed.state_root_t, q_forward.state_root_t,
        "SG-G1.3: state_root_t must be byte-equal between forward and resumed run"
    );
}

// ── SG-G1.4 ─────────────────────────────────────────────────────────────────
//
// Back-compat regression gate. With `resume_existing_chain=false`, a
// non-empty `refs/transitions/main` still produces the original TB-6
// `BootstrapError::NonEmptyRuntimeRepo`. All TB-N* / Stage C / Wave 3
// 50p / TB-N3 Phase 2 smoke runs hit this branch unchanged.
#[tokio::test]
async fn sg_g1_4_non_empty_runtime_repo_only_fires_when_resume_false() {
    let tmp = TempDir::new().expect("tempdir");
    let cfg = cfg_resume(&tmp, "g1_4-fresh", false);

    // Phase 1: fresh bootstrap, submit one TaskOpen to make the chain non-empty.
    let bundle = build_chaintape_sequencer(&cfg).expect("fresh bootstrap");
    let kernel = Kernel::new();
    let bus = TuringBus::with_sequencer(kernel, BusConfig::default(), bundle.sequencer.clone());
    let tx = make_synthetic_task_open("task-g1_4", "sponsor-g1_4", Hash::ZERO, "g1_4");
    bus.submit_typed_tx(tx).await.expect("submit TaskOpen");
    bundle.shutdown().await.expect("shutdown");
    drop(bus);

    // Phase 2: re-bootstrap with resume=false on the non-empty repo.
    // Must fail-closed with NonEmptyRuntimeRepo (the original TB-6 gate).
    let cfg_reboot = cfg_resume(&tmp, "g1_4-reboot", false);
    let result = build_chaintape_sequencer(&cfg_reboot);
    match result {
        Err(BootstrapError::NonEmptyRuntimeRepo {
            path,
            existing_head,
        }) => {
            assert_eq!(
                path, cfg_reboot.runtime_repo_path,
                "SG-G1.4: NonEmptyRuntimeRepo must echo the rejected runtime_repo_path"
            );
            assert!(
                !existing_head.is_empty(),
                "SG-G1.4: existing_head must be set when fail-closing"
            );
        }
        Err(other) => panic!(
            "SG-G1.4: expected BootstrapError::NonEmptyRuntimeRepo, got {other:?}"
        ),
        Ok(_) => panic!(
            "SG-G1.4: resume=false on non-empty repo must NOT bootstrap successfully \
             (would mask the TB-6 fail-closed gate for all prior smoke runs)"
        ),
    }

    // Phase 3: same non-empty repo bootstraps cleanly when resume=true.
    // Pins that the gap between "fail-closed" and "succeed" is exactly
    // the `resume_existing_chain` field.
    let cfg_resume = cfg_resume(&tmp, "g1_4-resume", true);
    let bundle_r =
        build_chaintape_sequencer(&cfg_resume).expect("resume=true on non-empty repo");
    bundle_r.shutdown().await.expect("resume shutdown");
}

// ── SG-G1.5 ─────────────────────────────────────────────────────────────────
//
// Pinned-pubkey continuity. After resume, the original epoch's pubkey
// entry MUST still be present in `pinned_pubkeys.json` (so prior L4
// entries continue to verify). The manifest gains a NEW entry for the
// new epoch — because Ed25519 secret keys are not persisted to disk,
// resume cannot reuse the prior signing key; instead it generates a
// new keypair for a new epoch + appends to the manifest. This is the
// only correct way to preserve verification continuity for older
// entries while letting the resumed sequencer sign new ones.
#[tokio::test]
async fn sg_g1_5_pinned_pubkeys_preserved_across_resume() {
    let tmp = TempDir::new().expect("tempdir");
    let cfg = cfg_resume(&tmp, "g1_5-fresh", false);

    // Phase 1: fresh bootstrap writes the initial manifest.
    let bundle = build_chaintape_sequencer(&cfg).expect("fresh bootstrap");
    let manifest_path = cfg.runtime_repo_path.join("pinned_pubkeys.json");
    let original_json = std::fs::read_to_string(&manifest_path).expect("read original manifest");
    let original: serde_json::Value =
        serde_json::from_str(&original_json).expect("parse original manifest");
    let original_epoch = original["epoch"].as_u64().expect("epoch u64");
    assert_eq!(original_epoch, 1, "fresh manifest pins epoch=1");
    let original_pubkey_hex = original["pubkeys"][0]["pubkey_hex"]
        .as_str()
        .expect("pubkey_hex")
        .to_string();

    // Submit one TaskOpen so the chain is non-empty (resume admission requires it).
    let kernel = Kernel::new();
    let bus = TuringBus::with_sequencer(kernel, BusConfig::default(), bundle.sequencer.clone());
    let tx = make_synthetic_task_open("task-g1_5", "sponsor-g1_5", Hash::ZERO, "g1_5");
    bus.submit_typed_tx(tx).await.expect("submit TaskOpen");
    bundle.shutdown().await.expect("shutdown phase 1");
    drop(bus);

    // Phase 2: resume — manifest must still contain the original entry
    // (epoch=1 pubkey unchanged) plus one new entry (epoch=2 with a
    // different pubkey).
    let cfg_r = cfg_resume(&tmp, "g1_5-resume", true);
    let bundle_r = build_chaintape_sequencer(&cfg_r).expect("resume bootstrap");
    let resumed_json =
        std::fs::read_to_string(&manifest_path).expect("read manifest after resume");
    let resumed: serde_json::Value =
        serde_json::from_str(&resumed_json).expect("parse manifest after resume");
    let resumed_pubkeys = resumed["pubkeys"]
        .as_array()
        .expect("pubkeys after resume must be an array");

    assert!(
        resumed_pubkeys.len() >= 2,
        "SG-G1.5: manifest must gain at least one entry on resume (had {} after resume)",
        resumed_pubkeys.len()
    );
    let original_still_present = resumed_pubkeys.iter().any(|e| {
        e["epoch"].as_u64() == Some(original_epoch)
            && e["pubkey_hex"].as_str() == Some(original_pubkey_hex.as_str())
    });
    assert!(
        original_still_present,
        "SG-G1.5: original (epoch={original_epoch}, pubkey={original_pubkey_hex}) MUST still be \
         present in pinned_pubkeys.json after resume — prior L4 entries depend on this \
         pubkey for system_signature verification"
    );

    let new_epoch = resumed["epoch"].as_u64().expect("epoch u64 after resume");
    assert!(
        new_epoch > original_epoch,
        "SG-G1.5: top-level manifest epoch must advance on resume (was {original_epoch}, \
         after resume {new_epoch})"
    );
    let new_entry_exists = resumed_pubkeys.iter().any(|e| {
        e["epoch"].as_u64() == Some(new_epoch)
            && e["pubkey_hex"].as_str().is_some()
            && e["pubkey_hex"].as_str() != Some(original_pubkey_hex.as_str())
    });
    assert!(
        new_entry_exists,
        "SG-G1.5: a NEW (epoch={new_epoch}) entry with a distinct pubkey must be appended \
         on resume — Ed25519 secrets aren't persisted, so the new sequencer must sign \
         new L4 entries with a freshly generated keypair"
    );

    bundle_r.shutdown().await.expect("resume shutdown");
}

// ── SG-G1.6 (R2 closure; Codex Q2 CHALLENGE) ────────────────────────────────
//
// `resume_existing_durable` fails closed with `ManifestAbsentInResume`
// when invoked on a runtime_repo where `agent_pubkeys.json` doesn't
// exist. Binary-layer invariant: env=1 + manifest absent must NOT
// silently degrade to fresh init. Mechanism per
// `feedback_norm_needs_mechanism`.
#[test]
fn sg_g1_6_resume_existing_durable_fails_closed_when_manifest_absent() {
    let tmp = TempDir::new().expect("tempdir");
    let runtime_repo = tmp.path().join("runtime_repo");
    std::fs::create_dir_all(&runtime_repo).expect("mkdir");
    // No agent_pubkeys.json written.
    let keystore = tmp.path().join("keystore.enc");
    let pwd = secrecy::SecretString::new("test-password".to_string().into());
    let result = AgentKeypairRegistry::resume_existing_durable(&runtime_repo, &keystore, pwd);
    match result {
        Err(AgentKeypairError::ManifestAbsentInResume { path }) => {
            assert_eq!(
                path,
                runtime_repo.join("agent_pubkeys.json"),
                "SG-G1.6: ManifestAbsentInResume must echo the expected manifest path"
            );
        }
        Err(other) => panic!(
            "SG-G1.6: expected ManifestAbsentInResume; got {other:?}"
        ),
        Ok(_) => panic!(
            "SG-G1.6: resume_existing_durable on missing manifest MUST fail-closed \
             (silent fall-through would degrade FC2 §3.2 agent_registry replay input)"
        ),
    }
}

// ── SG-G1.8 (R2 R2 closure; Codex R2 Q5+Q7+Q9 CHALLENGE) ─────────────────────
//
// `resume_existing_durable` fails closed with `ResumeKeystoreInconsistent`
// when the manifest pubkey for an agent DOES NOT MATCH the pubkey
// derived from that agent's keystore secret. Catches: manifest
// tampering (someone edited agent_pubkeys.json with a different
// pubkey while the keystore still holds the original secret),
// split-brain keystore (keystore and manifest came from different
// runs), or hash collision-style attack. The reason string MUST
// include "does NOT match" so the operator can distinguish this
// from the missing-secret path (SG-G1.7).
//
// Mechanism per `feedback_norm_needs_mechanism` (closes Codex R2
// observation that pubkey-mismatch was unbound in CI).
#[test]
fn sg_g1_8_resume_existing_durable_fails_closed_on_pubkey_mismatch() {
    use std::collections::BTreeMap;
    let tmp = TempDir::new().expect("tempdir");
    let runtime_repo = tmp.path().join("runtime_repo");
    std::fs::create_dir_all(&runtime_repo).expect("mkdir");

    // Build a deterministic 32-byte seed and derive the keypair from it
    // (so we can save the SAME seed to the durable keystore and then
    // recompute the same pubkey on resume).
    let real_secret: [u8; 32] = [0x42u8; 32];
    let real_kp = turingosv4::runtime::agent_keypairs::AgentKeypair::from_secret_bytes(real_secret);
    let real_pubkey_hex = real_kp.public_key().to_hex();

    let agent_id_str = "Agent_real_for_mismatch_test".to_string();
    let keystore_path = tmp.path().join("keystore.enc");
    let pwd = secrecy::SecretString::new("test-password".to_string().into());
    let mut secrets: BTreeMap<String, [u8; 32]> = BTreeMap::new();
    secrets.insert(agent_id_str.clone(), real_secret);
    turingosv4::runtime::agent_keystore::save(&keystore_path, &pwd, &secrets)
        .expect("save keystore with real secret");

    // Write a manifest with the SAME agent_id but a DIFFERENT pubkey hex
    // (all-zeros — easy to distinguish from the real-pubkey hex).
    let tampered_pubkey_hex = "00".repeat(32);
    assert_ne!(
        tampered_pubkey_hex, real_pubkey_hex,
        "test invariant: tampered pubkey must differ from real-derived pubkey"
    );
    let mut agents = BTreeMap::new();
    agents.insert(agent_id_str.clone(), tampered_pubkey_hex.clone());
    let manifest = serde_json::json!({ "agents": agents });
    std::fs::write(
        runtime_repo.join("agent_pubkeys.json"),
        serde_json::to_vec(&manifest).expect("serialize manifest"),
    )
    .expect("write tampered manifest");

    let result = AgentKeypairRegistry::resume_existing_durable(&runtime_repo, &keystore_path, pwd);
    match result {
        Err(AgentKeypairError::ResumeKeystoreInconsistent { agent_id, reason }) => {
            assert_eq!(
                agent_id, agent_id_str,
                "SG-G1.8: error must name the inconsistent agent_id"
            );
            assert!(
                reason.contains("does NOT match"),
                "SG-G1.8: reason should describe pubkey-mismatch case (contain \
                 'does NOT match'); got {reason:?}"
            );
        }
        Err(other) => panic!(
            "SG-G1.8: expected ResumeKeystoreInconsistent on pubkey mismatch; \
             got {other:?}"
        ),
        Ok(_) => panic!(
            "SG-G1.8: resume with tampered manifest pubkey MUST fail-closed — \
             silent boot would violate FC2 §3.2 agent_registry replay \
             determinism (signing key would no longer match the on-disk public \
             pubkey for that agent)"
        ),
    }
}

// ── SG-G1.7 (R2 closure; Codex Q1+Q8 CHALLENGE) ─────────────────────────────
//
// `resume_existing_durable` fails closed with `ResumeKeystoreInconsistent`
// when the manifest references an agent_id that has no corresponding
// secret in the durable keystore. Catches: empty keystore + populated
// manifest, wrong-password keystore reading as empty, keystore wiped
// while manifest survived.
#[test]
fn sg_g1_7_resume_existing_durable_fails_closed_on_keystore_manifest_drift() {
    use std::collections::BTreeMap;
    let tmp = TempDir::new().expect("tempdir");
    let runtime_repo = tmp.path().join("runtime_repo");
    std::fs::create_dir_all(&runtime_repo).expect("mkdir");
    // Write a manifest claiming an agent that is NOT in the durable keystore.
    let mut agents = BTreeMap::new();
    agents.insert(
        "Agent_phantom".to_string(),
        // 32-byte all-zeros pubkey hex placeholder — won't match any real key
        "00".repeat(32),
    );
    let manifest = serde_json::json!({
        "agents": agents,
    });
    std::fs::write(
        runtime_repo.join("agent_pubkeys.json"),
        serde_json::to_vec(&manifest).expect("serialize manifest"),
    )
    .expect("write manifest");

    let keystore = tmp.path().join("keystore.enc");
    // Empty keystore: never written, so `load_or_empty` returns empty.
    let pwd = secrecy::SecretString::new("test-password".to_string().into());
    let result = AgentKeypairRegistry::resume_existing_durable(&runtime_repo, &keystore, pwd);
    match result {
        Err(AgentKeypairError::ResumeKeystoreInconsistent { agent_id, reason }) => {
            assert_eq!(
                agent_id, "Agent_phantom",
                "SG-G1.7: error must name the inconsistent agent_id"
            );
            assert!(
                reason.contains("no corresponding secret"),
                "SG-G1.7: reason should describe missing-secret case; got {reason:?}"
            );
        }
        Err(other) => panic!(
            "SG-G1.7: expected ResumeKeystoreInconsistent; got {other:?}"
        ),
        Ok(_) => panic!(
            "SG-G1.7: resume_existing_durable with manifest agent but no keystore secret \
             MUST fail-closed — silent boot would lose signing capability and violate \
             FC2 §3.2 agent_registry replay determinism"
        ),
    }
}
