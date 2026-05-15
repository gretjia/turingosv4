//! TB-G G1.2-2 (Option B+ orchestration ruling 2026-05-11) —
//! ChainTapeLease single-writer lock gates SG-G1.2-2.1..G1.2-2.6.
//!
//! Architect §3.2 verbatim:
//!
//! > 即使是顺序 subprocess，也要加锁。以后很可能并发. 新增
//! > `ChainTapeLease`. only one writer can advance HEAD_t at a time.
//! > 不要让两个 subprocess 同时写 refs/transitions/main.
//!
//! Six gates:
//!  1. acquire_release_round_trip
//!  2. rejects_second_writer_same_pid
//!  3. rejects_second_writer_other_pid (synth: simulate via lease file forgery)
//!  4. detects_stale_lock_when_pid_dead
//!  5. detects_head_changed_under_lock
//!  6. releases_on_guard_drop
//!
//! FC-trace: FC2-Boot adjacent.

use std::fs;
use std::path::PathBuf;

use tempfile::TempDir;
use turingosv4::bus::{BusConfig, TuringBus};
use turingosv4::kernel::Kernel;
use turingosv4::runtime::adapter::make_synthetic_task_open;
use turingosv4::runtime::chain_tape_lease::{acquire, read_lease, ChainTapeLease, LeaseError};
use turingosv4::runtime::resume_preflight::snapshot_head_t;
use turingosv4::runtime::{build_chaintape_sequencer, RuntimeChaintapeConfig};
use turingosv4::state::q_state::Hash;

async fn bootstrap_one_entry_chain(tmp: &TempDir, run_id: &str) -> PathBuf {
    let runtime_repo = tmp.path().join("runtime_repo");
    let cfg = RuntimeChaintapeConfig {
        runtime_repo_path: runtime_repo.clone(),
        cas_path: tmp.path().join("cas"),
        run_id: run_id.into(),
        queue_capacity: 16,
        resume_existing_chain: false,
    };
    let bundle = build_chaintape_sequencer(&cfg).expect("bootstrap");
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
    runtime_repo
}

// ── SG-G1.2-2.1 ─────────────────────────────────────────────────────────────
#[test]
fn lease_acquire_release_round_trip() {
    let tmp = TempDir::new().expect("tempdir");
    let repo = tmp.path().join("runtime_repo");
    fs::create_dir_all(&repo).expect("mkdir");

    let guard = acquire(&repo, "batch_round_trip", "").expect("acquire");
    let on_disk = read_lease(&repo)
        .expect("read_lease")
        .expect("lease should be present");
    assert_eq!(on_disk.batch_id, "batch_round_trip");
    assert_eq!(on_disk.holder_pid, std::process::id() as i32);

    guard.release().expect("explicit release");
    let after = read_lease(&repo).expect("read_lease post-release");
    assert!(
        after.is_none(),
        "SG-G1.2-2.1: lease file must be removed on release"
    );
}

// ── SG-G1.2-2.2 ─────────────────────────────────────────────────────────────
//
// Second `acquire()` from the same live process must report
// `AlreadyHeld` carrying the existing lease record.
#[test]
fn lease_rejects_second_writer_same_pid() {
    let tmp = TempDir::new().expect("tempdir");
    let repo = tmp.path().join("runtime_repo");
    fs::create_dir_all(&repo).expect("mkdir");

    let _guard = acquire(&repo, "batch_same_pid_1", "").expect("first acquire");
    match acquire(&repo, "batch_same_pid_2", "") {
        Err(LeaseError::AlreadyHeld { existing }) => {
            assert_eq!(existing.batch_id, "batch_same_pid_1");
            assert_eq!(existing.holder_pid, std::process::id() as i32);
        }
        other => panic!("SG-G1.2-2.2: expected AlreadyHeld, got {other:?}"),
    }
}

// ── SG-G1.2-2.3 ─────────────────────────────────────────────────────────────
//
// Forge a lease file with a different (but live) holder pid by using
// `pid=1` (init — always alive on every Linux host). Acquire must
// fail with `AlreadyHeld`.
#[test]
fn lease_rejects_second_writer_other_pid() {
    let tmp = TempDir::new().expect("tempdir");
    let repo = tmp.path().join("runtime_repo");
    fs::create_dir_all(&repo).expect("mkdir");

    let forged = ChainTapeLease {
        holder_pid: 1, // pid 1 = init; always alive
        batch_id: "forged_batch".into(),
        start_head_t_hex: String::new(),
        acquired_at_unix_s: 0,
    };
    fs::write(
        repo.join("chain_tape_lease.json"),
        serde_json::to_string_pretty(&forged).unwrap(),
    )
    .expect("write forged lease");

    match acquire(&repo, "batch_other_pid", "") {
        Err(LeaseError::AlreadyHeld { existing }) => {
            assert_eq!(existing.holder_pid, 1);
            assert_eq!(existing.batch_id, "forged_batch");
        }
        other => panic!("SG-G1.2-2.3: expected AlreadyHeld, got {other:?}"),
    }
}

// ── SG-G1.2-2.4 ─────────────────────────────────────────────────────────────
//
// Forge a lease file with a dead pid (very high pid number unlikely to
// be live). Acquire must force-release the stale lock and succeed.
#[test]
fn lease_detects_stale_lock_when_pid_dead() {
    let tmp = TempDir::new().expect("tempdir");
    let repo = tmp.path().join("runtime_repo");
    fs::create_dir_all(&repo).expect("mkdir");

    let stale = ChainTapeLease {
        holder_pid: i32::MAX, // No process will ever have this pid
        batch_id: "stale_batch".into(),
        start_head_t_hex: String::new(),
        acquired_at_unix_s: 0,
    };
    fs::write(
        repo.join("chain_tape_lease.json"),
        serde_json::to_string_pretty(&stale).unwrap(),
    )
    .expect("write stale lease");

    let guard = acquire(&repo, "batch_stale_recover", "")
        .expect("SG-G1.2-2.4: stale lease must be force-released");
    let live = read_lease(&repo)
        .expect("read live lease")
        .expect("present");
    assert_eq!(
        live.holder_pid,
        std::process::id() as i32,
        "SG-G1.2-2.4: new acquire must claim the lease as our pid"
    );
    assert_eq!(live.batch_id, "batch_stale_recover");
    drop(guard);
}

// ── SG-G1.2-2.5 ─────────────────────────────────────────────────────────────
//
// Acquire claims the lease against `expected_head_t_hex == "abc"` but
// the live `refs/transitions/main` head is something else → reject
// with `HeadChangedSinceLastAcquire`.
#[tokio::test]
async fn lease_detects_head_changed_under_lock() {
    let tmp = TempDir::new().expect("tempdir");
    let repo = bootstrap_one_entry_chain(&tmp, "head_changed").await;
    let (real_head, _, _) = snapshot_head_t(&repo).expect("snapshot");
    assert!(
        !real_head.is_empty(),
        "bootstrapped repo must have non-empty head"
    );

    // Pass a stale head — the function should refuse to acquire.
    let stale_head = "0".repeat(40);
    assert_ne!(stale_head, real_head, "tampered head must differ");

    match acquire(&repo, "batch_head_changed", &stale_head) {
        Err(LeaseError::HeadChangedSinceLastAcquire {
            expected_hex,
            actual_hex,
        }) => {
            assert_eq!(expected_hex, stale_head);
            assert_eq!(actual_hex, real_head);
        }
        other => panic!("SG-G1.2-2.5: expected HeadChangedSinceLastAcquire, got {other:?}"),
    }
}

// ── SG-G1.2-2.6 ─────────────────────────────────────────────────────────────
//
// Dropping the guard (without explicit release()) must remove the
// lease file — RAII contract.
#[test]
fn lease_releases_on_guard_drop() {
    let tmp = TempDir::new().expect("tempdir");
    let repo = tmp.path().join("runtime_repo");
    fs::create_dir_all(&repo).expect("mkdir");

    {
        let _guard = acquire(&repo, "batch_drop_release", "").expect("acquire");
        assert!(repo.join("chain_tape_lease.json").exists());
    } // guard drops here
    assert!(
        !repo.join("chain_tape_lease.json").exists(),
        "SG-G1.2-2.6: drop must remove the lease file"
    );
}
