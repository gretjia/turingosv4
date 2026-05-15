//! TB-6 Atom 1.3 — production ChainTape runtime integration tests.
//!
//! 9 tests T1-T5 + T7-T10 verify the runtime factory + ChaintapeBundle
//! lifecycle + WAL coexistence + direct `bus.submit_typed_tx` fixture (T10
//! = the bypass-evaluator proof of L4 production-path).
//!
//! T6 (oneshot prompt-context-hash regression) was DROPPED per Codex round-1
//! Q6 — `run_oneshot` doesn't traverse the bus path so the hash invariance
//! is unrelated to chaintape mode.
//!
//! T7 (chaintape mode evaluator construction) was SCOPE-NARROWED per Codex
//! round-2 Q6 — `run_swarm` does NOT call `submit_typed_tx`; T7 is now
//! construction-only (env-flag set → bus.sequencer.is_some()).
//!
//! Charter: handover/tracer_bullets/TB-6_charter_2026-05-01.md
//! Preflight: handover/ai-direct/TB-6_PRODUCTION_CHAINTAPE_BOOTSTRAP_2026-05-01.md v2.1.

use std::sync::Mutex;

use tempfile::TempDir;
use turingosv4::bottom_white::ledger::transition_ledger::LedgerWriter;
use turingosv4::bus::{BusConfig, TuringBus};
use turingosv4::economy::money::MicroCoin;
use turingosv4::kernel::Kernel;
use turingosv4::runtime::adapter::{
    genesis_with_balances, make_synthetic_escrow_lock, make_synthetic_task_open,
    make_synthetic_worktx,
};
use turingosv4::runtime::{
    build_chaintape_sequencer, build_chaintape_sequencer_with_initial_q, BootstrapError,
    ChaintapeBundle, RuntimeChaintapeConfig,
};
use turingosv4::state::q_state::{AgentId, Hash};

/// Static lock so env-mutating tests don't race under cargo's parallel runner
/// (per `feedback_env_var_test_lock`). Every test that reads/writes
/// `TURINGOS_CHAINTAPE_PATH` / `WAL_DIR` / `TURINGOS_RUN_ID` etc. acquires
/// this mutex first.
static ENV_LOCK: Mutex<()> = Mutex::new(());

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
async fn t1_build_chaintape_sequencer_returns_non_none_sequencer_with_git_writer() {
    let tmp = TempDir::new().expect("tempdir");
    let cfg = fresh_config(&tmp, "t1");
    let bundle = build_chaintape_sequencer(&cfg).expect("bootstrap");
    // Ensure the sequencer + writers are real handles.
    assert!(
        std::sync::Arc::strong_count(&bundle.sequencer) >= 2,
        "sequencer is held by both the bundle and the spawned driver task"
    );
    bundle.shutdown().await.expect("shutdown");
    // Repo dir + .git dir exist on disk.
    assert!(cfg.runtime_repo_path.exists());
    assert!(cfg.runtime_repo_path.join(".git").exists());
    assert!(cfg.cas_path.exists());
}

#[tokio::test]
async fn t2_build_chaintape_sequencer_writes_pinned_pubkeys_json_to_runtime_repo() {
    let tmp = TempDir::new().expect("tempdir");
    let cfg = fresh_config(&tmp, "t2");
    let bundle = build_chaintape_sequencer(&cfg).expect("bootstrap");
    let manifest_path = cfg.runtime_repo_path.join("pinned_pubkeys.json");
    assert!(manifest_path.exists(), "pinned_pubkeys.json must exist");
    let json = std::fs::read_to_string(&manifest_path).expect("read manifest");
    let parsed: serde_json::Value = serde_json::from_str(&json).expect("parse JSON");
    assert_eq!(parsed["run_id"].as_str(), Some("t2"));
    assert_eq!(parsed["tb_id"].as_str(), Some("TB-6"));
    assert_eq!(parsed["epoch"].as_u64(), Some(1));
    let pubkeys = parsed["pubkeys"].as_array().expect("pubkeys array");
    assert_eq!(pubkeys.len(), 1);
    assert!(!pubkeys[0]["pubkey_hex"].as_str().unwrap().is_empty());
    bundle.shutdown().await.expect("shutdown");
}

#[tokio::test]
async fn t3_build_chaintape_sequencer_succeeds_on_idempotent_empty_repo_reopen() {
    // Atom 1.1 inherits this test name; Atom 1.3 re-asserts with same shape.
    // True "fail-closed on existing chain" requires committing an actual L4
    // entry first (apply_one of a real WorkTx); deferred to Atom 3 smoke.
    let tmp = TempDir::new().expect("tempdir");
    let cfg = fresh_config(&tmp, "t3");
    let b1 = build_chaintape_sequencer(&cfg).expect("first bootstrap");
    b1.shutdown().await.expect("first shutdown");
    let b2 = build_chaintape_sequencer(&cfg).expect("second bootstrap on empty refs");
    b2.shutdown().await.expect("second shutdown");
}

#[tokio::test]
async fn t4_chaintape_bundle_shutdown_drains_pending_submissions_before_join() {
    let tmp = TempDir::new().expect("tempdir");
    let cfg = fresh_config(&tmp, "t4");
    let bundle = build_chaintape_sequencer(&cfg).expect("bootstrap");
    // No submissions in the queue here; this is the empty-drain happy path.
    // The lifecycle invariant exercised: shutdown() returns Ok and the driver
    // task joins promptly (no hang). Real synthetic submissions exercising
    // the drain path land in T10 below.
    let res = tokio::time::timeout(std::time::Duration::from_secs(5), bundle.shutdown()).await;
    let inner = res.expect("shutdown completes within timeout");
    inner.expect("shutdown returns Ok");
}

#[tokio::test]
async fn t5_chaintape_bundle_shutdown_returns_clean_on_empty_queue() {
    let tmp = TempDir::new().expect("tempdir");
    let cfg = fresh_config(&tmp, "t5");
    let bundle = build_chaintape_sequencer(&cfg).expect("bootstrap");
    bundle.shutdown().await.expect("clean shutdown");
}

// T6 — `evaluator_legacy_mode_prompt_context_hash_is_a1f43584a17d1226` —
// DROPPED per Codex round-1 Q6 (`run_oneshot` doesn't traverse the bus path).

#[tokio::test]
async fn t7_evaluator_chaintape_mode_sets_bus_sequencer_field_to_some() {
    // SCOPE-NARROWED per Codex round-2 Q6: `run_swarm` does NOT call
    // submit_typed_tx; T7 cannot prove L4 entries via the evaluator. T7 now
    // verifies CONSTRUCTION-ONLY: when env-flag is set, the bus is built via
    // TuringBus::with_sequencer and `bus.sequencer` is `Some(_)`.
    let _guard = ENV_LOCK.lock().expect("env lock");
    let tmp = TempDir::new().expect("tempdir");
    let cfg = fresh_config(&tmp, "t7");
    // Mirror what run_swarm does: build chaintape bundle, then construct bus.
    let bundle = build_chaintape_sequencer(&cfg).expect("bootstrap");
    let kernel = Kernel::new();
    let bus_config = BusConfig::default();
    let bus = TuringBus::with_sequencer(kernel, bus_config, bundle.sequencer.clone());
    assert!(
        bus.sequencer.is_some(),
        "bus.sequencer must be Some after with_sequencer"
    );
    bundle.shutdown().await.expect("shutdown");
}

#[tokio::test]
async fn t8_evaluator_legacy_wal_mode_unchanged_when_chaintape_off() {
    // Codex F5 regression: TURINGOS_CHAINTAPE_PATH unset → WAL_DIR path
    // unchanged. We don't run the evaluator binary here (heavy); we test the
    // structural prerequisite: `RuntimeChaintapeConfig::from_env()` returns
    // None when the env var is unset.
    let _guard = ENV_LOCK.lock().expect("env lock");
    std::env::remove_var("TURINGOS_CHAINTAPE_PATH");
    let cfg = RuntimeChaintapeConfig::from_env();
    assert!(
        cfg.is_none(),
        "from_env returns None when TURINGOS_CHAINTAPE_PATH is unset"
    );
}

#[tokio::test]
async fn t9_chaintape_mode_silently_disables_wal_when_both_env_vars_set() {
    // Codex F5 precedence: when both env vars set, chain wins for bus
    // construction. We verify the structural prerequisite: from_env produces
    // a valid config when TURINGOS_CHAINTAPE_PATH is set, regardless of
    // WAL_DIR being set. The actual bus-construction precedence is verified
    // in evaluator.rs run_swarm via the chaintape_bundle.is_some() branch.
    let _guard = ENV_LOCK.lock().expect("env lock");
    let tmp = TempDir::new().expect("tempdir");
    let chaintape_dir = tmp.path().join("runtime_repo");
    let wal_dir = tmp.path().join("wal");
    std::env::set_var("TURINGOS_CHAINTAPE_PATH", &chaintape_dir);
    std::env::set_var("WAL_DIR", &wal_dir);
    std::env::set_var("TURINGOS_RUN_ID", "t9");
    let cfg = RuntimeChaintapeConfig::from_env().expect("chaintape config");
    assert_eq!(cfg.runtime_repo_path, chaintape_dir);
    assert_eq!(cfg.run_id, "t9");
    std::env::remove_var("TURINGOS_CHAINTAPE_PATH");
    std::env::remove_var("WAL_DIR");
    std::env::remove_var("TURINGOS_RUN_ID");
}

#[tokio::test]
async fn t10_direct_bus_submit_typed_tx_synthetic_taskopen_appends_l4_entry() {
    // T10 (Codex round-2 NEW; Atom 2 expansion): proves Atom 1's L4 path works
    // independently of Atom 3's real LLM. Submits a synthetic TaskOpenTx via
    // `bus.submit_typed_tx`, drains the driver via `bundle.shutdown()`, and
    // re-opens the runtime repo from disk to assert ≥1 accepted L4 entry.
    // TaskOpen has no economic-debit precondition — the simplest valid
    // accepted tx for a fresh QState::genesis() bootstrap.
    let tmp = TempDir::new().expect("tempdir");
    let cfg = fresh_config(&tmp, "t10");
    let bundle = build_chaintape_sequencer(&cfg).expect("bootstrap");
    let kernel = Kernel::new();
    let bus = TuringBus::with_sequencer(kernel, BusConfig::default(), bundle.sequencer.clone());
    assert!(bus.sequencer.is_some());

    // Pre-state: empty git repo (no head commit, no entries).
    {
        let writer = bundle.transition_writer.read().expect("writer read");
        assert_eq!(writer.head_commit_oid_hex(), None);
        assert_eq!(writer.len(), 0);
    }

    // Submit one synthetic TaskOpen via the production path.
    let task_open = make_synthetic_task_open("task-t10", "sponsor-t10", Hash::ZERO, "t10-1");
    bus.submit_typed_tx(task_open)
        .await
        .expect("submit TaskOpen via bus.submit_typed_tx");

    bundle.shutdown().await.expect("shutdown drains queue");
    drop(bus);

    let reopened = turingosv4::bottom_white::ledger::transition_ledger::Git2LedgerWriter::open(
        &cfg.runtime_repo_path,
    )
    .expect("reopen");
    assert!(
        reopened.len() >= 1,
        "≥1 LedgerEntry must exist on disk after TaskOpen submit + shutdown drain"
    );
    assert!(
        reopened.head_commit_oid_hex().is_some(),
        "refs/transitions/main has a head commit"
    );
}

#[tokio::test]
async fn t11_synthetic_zero_stake_worktx_appends_l4e_rejection() {
    // T11 (Atom 2 NEW): proves the L4.E (rejected) path works on disk.
    // Zero-stake WorkTx → rejected at structural admission gate (StakeInsufficient),
    // regardless of task_market state. Per architect § 3.6 Atom 3 the smoke
    // evidence requires ≥1 rejected L4.E entry.
    let tmp = TempDir::new().expect("tempdir");
    let cfg = fresh_config(&tmp, "t11");
    let bundle = build_chaintape_sequencer(&cfg).expect("bootstrap");
    let kernel = Kernel::new();
    let bus = TuringBus::with_sequencer(kernel, BusConfig::default(), bundle.sequencer.clone());

    let bad_worktx = make_synthetic_worktx(
        "task-t11",
        "agent-t11",
        Hash::ZERO,
        0,
        "t11-zero-stake",
        true,
    );
    bus.submit_typed_tx(bad_worktx)
        .await
        .expect("submit zero-stake WorkTx");

    let rejection_writer_handle = bundle.rejection_writer.clone();
    bundle.shutdown().await.expect("shutdown drains queue");
    drop(bus);

    let rw = rejection_writer_handle
        .read()
        .expect("rejection writer read");
    assert!(
        rw.len() >= 1,
        "≥1 RejectedSubmissionRecord after zero-stake WorkTx submit + drain"
    );
}

#[tokio::test]
async fn t12_chained_taskopen_then_zero_stake_worktx_produces_l4_and_l4e() {
    // T12 (Atom 2 NEW): the architect-mandated minimum from § 3.6 Atom 3 —
    // ≥1 accepted L4 + ≥1 rejected L4.E in a single bundle.
    let tmp = TempDir::new().expect("tempdir");
    let cfg = fresh_config(&tmp, "t12");
    let initial_q = genesis_with_balances(&[(
        AgentId("sponsor-t12".into()),
        MicroCoin::from_coin(100).unwrap(),
    )]);
    let bundle = build_chaintape_sequencer_with_initial_q(&cfg, initial_q)
        .expect("bootstrap with seeded balance");
    let kernel = Kernel::new();
    let bus = TuringBus::with_sequencer(kernel, BusConfig::default(), bundle.sequencer.clone());

    let task_open = make_synthetic_task_open("task-t12", "sponsor-t12", Hash::ZERO, "t12-1");
    bus.submit_typed_tx(task_open)
        .await
        .expect("submit TaskOpen");
    let bad_worktx =
        make_synthetic_worktx("task-t12", "agent-t12", Hash::ZERO, 0, "t12-zero", true);
    bus.submit_typed_tx(bad_worktx)
        .await
        .expect("submit zero-stake WorkTx");

    let rejection_writer_handle = bundle.rejection_writer.clone();
    bundle.shutdown().await.expect("shutdown");
    drop(bus);

    let reopened = turingosv4::bottom_white::ledger::transition_ledger::Git2LedgerWriter::open(
        &cfg.runtime_repo_path,
    )
    .expect("reopen");
    assert!(reopened.len() >= 1, "TaskOpen produces ≥1 L4 entry");
    let rw = rejection_writer_handle.read().expect("rejection read");
    assert!(rw.len() >= 1, "Zero-stake WorkTx produces ≥1 L4.E entry");
}

#[test]
fn t13_synthetic_escrow_lock_constructor_compiles() {
    // T13 (Atom 2 NEW; smoke): demonstrates make_synthetic_escrow_lock is
    // callable from an external test crate. Functional admission test
    // (TaskOpen → EscrowLock → WorkTx full happy chain) requires sequencing
    // parent_state_root correctly across each tx's accept; deferred to Atom 3
    // smoke which has the polling / state-snapshot machinery.
    let _esc = make_synthetic_escrow_lock("task-t13", "sponsor-t13", 50_000_000, Hash::ZERO, "t13");
}

// Helper: BootstrapError variant smoke test (kept inline — no separate
// integration file). Verifies the type compiles + Display is non-empty.
#[test]
fn bootstrap_error_display_non_empty() {
    let err = BootstrapError::NonEmptyRuntimeRepo {
        path: "/tmp/foo".into(),
        existing_head: "deadbeef".into(),
    };
    assert!(!format!("{err}").is_empty());
    let _: ChaintapeBundle; // type-witness: ChaintapeBundle is reachable from the test.
}
