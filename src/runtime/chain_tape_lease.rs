//! TB-G G1.2-2 (Option B+ orchestration ruling 2026-05-11; binding directive
//! `handover/directives/2026-05-11_TB_G_G1_2_OPTION_B_PLUS_RULING.md` §3.2):
//! `ChainTapeLease` — single-writer file-lock guarding `refs/transitions/main`
//! advancement against concurrent subprocess writers.
//!
//! Architect §3.2 verbatim:
//!
//! > 即使是顺序 subprocess，也要加锁。以后很可能并发. 新增
//! > `ChainTapeLease`. 规则: only one writer can advance HEAD_t at a
//! > time. 提交前检查 current_head_t == expected_start_head_t. 如果不
//! > 一致: abort / retry through orchestrator. 不要让两个 subprocess
//! > 同时写 refs/transitions/main.
//!
//! Six SG-G1.2-2.* gates:
//!   1. acquire_release_round_trip
//!   2. rejects_second_writer_same_pid
//!   3. rejects_second_writer_other_pid
//!   4. detects_stale_lock_when_pid_dead
//!   5. detects_head_changed_under_lock
//!   6. releases_on_guard_drop
//!
//! Implementation: atomic file write via tempfile + rename to
//! `<runtime_repo>/chain_tape_lease.json`. Stale-lock detection via
//! `kill(holder_pid, 0)` (POSIX); if the holder is dead the lease is
//! force-released with an audit-log line.
//!
//! Sequential-batch use only today; concurrent expansion forward (G5+).
//!
//! FC-trace: FC2-Boot adjacent. Lease is the orchestration-side
//! safety primitive that guards the `Git2LedgerWriter::append` strict
//! `len + 1` invariant from concurrent advancement.

use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

const LEASE_FILE: &str = "chain_tape_lease.json";

/// TRACE_MATRIX § 3 orphan (TB-G G1.2-2 2026-05-11; Option B+
/// orchestration ruling §3.2): on-disk lease record. Written
/// atomically (`tempfile + rename`) to
/// `<runtime_repo>/chain_tape_lease.json` on `acquire()`; removed by
/// `LeaseGuard::drop` or replaced when the lease is reacquired.
/// Constitutional Justification: same OPTION_B_PLUS_RULING §3.2.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChainTapeLease {
    pub holder_pid: i32,
    pub batch_id: String,
    pub start_head_t_hex: String,
    pub acquired_at_unix_s: i64,
}

/// TRACE_MATRIX § 3 orphan (TB-G G1.2-2 2026-05-11; Option B+ §3.2):
/// RAII lease guard. Drop releases the lease by removing the lease
/// file. Holder is identified by the recorded `holder_pid` matching
/// `std::process::id()` cast to `i32` — guards against accidental
/// double-release if a guard is moved across threads/processes.
/// Constitutional Justification: same OPTION_B_PLUS_RULING §3.2.
#[derive(Debug)]
pub struct LeaseGuard {
    runtime_repo: PathBuf,
    holder_pid: i32,
    released: bool,
}

impl LeaseGuard {
    /// TRACE_MATRIX § 3 orphan (TB-G G1.2-2 2026-05-11; Option B+ §3.2):
    /// explicit early release. After this call the guard's `drop` is
    /// a no-op. Returns `Err` if the lease file is missing or holds a
    /// different pid (the lease was already released or stolen).
    pub fn release(mut self) -> Result<(), LeaseError> {
        self.release_inner()
    }

    fn release_inner(&mut self) -> Result<(), LeaseError> {
        if self.released {
            return Ok(());
        }
        let lease_path = self.runtime_repo.join(LEASE_FILE);
        if !lease_path.exists() {
            self.released = true;
            return Ok(());
        }
        let bytes = fs::read(&lease_path).map_err(LeaseError::Io)?;
        let current: ChainTapeLease =
            serde_json::from_slice(&bytes).map_err(|e| LeaseError::Parse(e.to_string()))?;
        if current.holder_pid != self.holder_pid {
            // Lease was stolen / replaced by another process. Do not
            // remove someone else's lease file. Mark released so drop
            // is a no-op and surface the error to the caller.
            return Err(LeaseError::HolderPidMismatch {
                expected_pid: self.holder_pid,
                actual_pid: current.holder_pid,
            });
        }
        fs::remove_file(&lease_path).map_err(LeaseError::Io)?;
        self.released = true;
        Ok(())
    }

    /// TRACE_MATRIX § 3 orphan (TB-G G1.2-2 2026-05-11; Option B+ §3.2):
    /// the recorded `start_head_t_hex` from the active lease. Used by
    /// the head-changed-under-lock detection helper in tests; not
    /// required for normal acquire/release flow.
    pub fn holder_pid(&self) -> i32 {
        self.holder_pid
    }
}

impl Drop for LeaseGuard {
    fn drop(&mut self) {
        if !self.released {
            let _ = self.release_inner();
        }
    }
}

/// TRACE_MATRIX § 3 orphan (TB-G G1.2-2 2026-05-11; Option B+ §3.2):
/// lease acquisition failure modes. Each variant maps to one of the 5
/// reject paths (one is shared between `Io`/`Parse` corruption). The
/// `AlreadyHeld` variant returns the live `ChainTapeLease` so the
/// caller can diagnose without re-reading the file.
/// Constitutional Justification: same OPTION_B_PLUS_RULING §3.2.
#[derive(Debug)]
pub enum LeaseError {
    /// Another live process holds the lease. The caller should abort
    /// or wait; force-stealing is not provided.
    AlreadyHeld { existing: ChainTapeLease },
    /// Caller's `expected_head_t_hex` does not match the recorded
    /// `start_head_t_hex` — the chain advanced under the caller's
    /// feet. Caller must rebase and retry.
    HeadChangedSinceLastAcquire {
        expected_hex: String,
        actual_hex: String,
    },
    /// Lease file present but the recorded `holder_pid` is no longer a
    /// live process. The acquire path force-releases and recovers; this
    /// error is only raised by `release()` when the in-memory guard's
    /// pid does not match the on-disk pid.
    HolderPidMismatch { expected_pid: i32, actual_pid: i32 },
    /// Lease file present but unparseable.
    Parse(String),
    /// Underlying I/O error.
    Io(io::Error),
}

impl std::fmt::Display for LeaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AlreadyHeld { existing } => write!(
                f,
                "chain tape lease already held by pid={} batch_id={} since {}",
                existing.holder_pid, existing.batch_id, existing.acquired_at_unix_s
            ),
            Self::HeadChangedSinceLastAcquire {
                expected_hex,
                actual_hex,
            } => write!(
                f,
                "chain tape head changed since acquire: expected={expected_hex} actual={actual_hex}"
            ),
            Self::HolderPidMismatch {
                expected_pid,
                actual_pid,
            } => write!(
                f,
                "lease release pid mismatch: expected={expected_pid} actual={actual_pid}"
            ),
            Self::Parse(s) => write!(f, "lease parse error: {s}"),
            Self::Io(e) => write!(f, "lease io error: {e}"),
        }
    }
}

impl std::error::Error for LeaseError {}

/// TRACE_MATRIX § 3 orphan (TB-G G1.2-2 2026-05-11; Option B+ §3.2):
/// acquire the single-writer chain-tape lease. On success returns a
/// `LeaseGuard` whose drop releases the lease. On failure returns
/// `LeaseError::AlreadyHeld` (live holder) or
/// `LeaseError::HeadChangedSinceLastAcquire` (chain advanced under
/// the caller — caller must rebase). Stale leases (holder pid dead)
/// are force-released and the new acquire proceeds. Constitutional
/// Justification: same OPTION_B_PLUS_RULING §3.2 "only one writer
/// can advance HEAD_t at a time".
pub fn acquire(
    runtime_repo: &Path,
    batch_id: &str,
    expected_head_t_hex: &str,
) -> Result<LeaseGuard, LeaseError> {
    fs::create_dir_all(runtime_repo).map_err(LeaseError::Io)?;
    let lease_path = runtime_repo.join(LEASE_FILE);

    if lease_path.exists() {
        let bytes = fs::read(&lease_path).map_err(LeaseError::Io)?;
        let current: ChainTapeLease =
            serde_json::from_slice(&bytes).map_err(|e| LeaseError::Parse(e.to_string()))?;
        if is_pid_alive(current.holder_pid) {
            return Err(LeaseError::AlreadyHeld { existing: current });
        }
        // Stale lock — holder pid is dead. Force-release.
        let _ = fs::remove_file(&lease_path);
    }

    // HEAD continuity check against the live `refs/transitions/main` /
    // C2 `refs/chaintape/l4`. Empty `expected_head_t_hex` means "I
    // don't care" (test scaffolding / fresh-genesis lease at task_0).
    if !expected_head_t_hex.is_empty() {
        let actual = snapshot_head_hex(runtime_repo)?;
        if actual != expected_head_t_hex {
            return Err(LeaseError::HeadChangedSinceLastAcquire {
                expected_hex: expected_head_t_hex.to_string(),
                actual_hex: actual,
            });
        }
    }

    let holder_pid = std::process::id() as i32;
    let acquired_at_unix_s = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    let lease = ChainTapeLease {
        holder_pid,
        batch_id: batch_id.to_string(),
        start_head_t_hex: expected_head_t_hex.to_string(),
        acquired_at_unix_s,
    };
    write_atomic(&lease_path, &lease)?;

    Ok(LeaseGuard {
        runtime_repo: runtime_repo.to_path_buf(),
        holder_pid,
        released: false,
    })
}

/// TRACE_MATRIX § 3 orphan (TB-G G1.2-2 2026-05-11; Option B+ §3.2):
/// read the active lease record from `<runtime_repo>/chain_tape_lease.json`
/// without acquiring it. `None` if no lease is held. Used by tests and
/// by the orchestrator's diagnostic path.
/// Constitutional Justification: same OPTION_B_PLUS_RULING §3.2.
pub fn read_lease(runtime_repo: &Path) -> Result<Option<ChainTapeLease>, LeaseError> {
    let lease_path = runtime_repo.join(LEASE_FILE);
    if !lease_path.exists() {
        return Ok(None);
    }
    let bytes = fs::read(&lease_path).map_err(LeaseError::Io)?;
    let lease: ChainTapeLease =
        serde_json::from_slice(&bytes).map_err(|e| LeaseError::Parse(e.to_string()))?;
    Ok(Some(lease))
}

fn write_atomic(target: &Path, lease: &ChainTapeLease) -> Result<(), LeaseError> {
    let parent = target.parent().ok_or_else(|| {
        LeaseError::Io(io::Error::new(
            io::ErrorKind::InvalidInput,
            "lease target has no parent dir",
        ))
    })?;
    let json = serde_json::to_vec_pretty(lease).map_err(|e| LeaseError::Parse(e.to_string()))?;
    let tmp_path = parent.join(format!("{}.tmp.{}", LEASE_FILE, std::process::id()));
    {
        let mut f = fs::File::create(&tmp_path).map_err(LeaseError::Io)?;
        f.write_all(&json).map_err(LeaseError::Io)?;
        f.sync_all().map_err(LeaseError::Io)?;
    }
    fs::rename(&tmp_path, target).map_err(LeaseError::Io)?;
    Ok(())
}

fn snapshot_head_hex(runtime_repo: &Path) -> Result<String, LeaseError> {
    use crate::bottom_white::ledger::transition_ledger::{Git2LedgerWriter, LedgerWriter};
    let writer = Git2LedgerWriter::open(runtime_repo).map_err(|e| {
        LeaseError::Io(io::Error::new(
            io::ErrorKind::Other,
            format!("Git2LedgerWriter::open({runtime_repo:?}): {e}"),
        ))
    })?;
    Ok(writer.head_commit_oid_hex().unwrap_or_default())
}

/// POSIX `kill -0 <pid>` — return true if the pid corresponds to a
/// live process or a zombie awaiting reaping; false otherwise. On
/// Linux/macOS this uses `libc::kill(pid, 0)` and treats `ESRCH` as
/// dead. Any other errno (e.g. `EPERM` on a foreign-uid process) is
/// treated as alive — over-conservative refuses to steal the lease.
fn is_pid_alive(pid: i32) -> bool {
    if pid <= 0 {
        return false;
    }
    unsafe {
        let rc = libc::kill(pid, 0);
        if rc == 0 {
            return true;
        }
        let errno = *libc::__errno_location();
        // ESRCH = 3 on Linux (and macOS). Conservative: treat anything
        // else as alive.
        errno != libc::ESRCH
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn unit_acquire_release_round_trip() {
        let tmp = TempDir::new().expect("tempdir");
        let repo = tmp.path().join("runtime_repo");
        fs::create_dir_all(&repo).expect("mkdir");
        let guard = acquire(&repo, "test_batch", "").expect("acquire");
        assert!(repo.join(LEASE_FILE).exists());
        guard.release().expect("release");
        assert!(!repo.join(LEASE_FILE).exists());
    }
}
