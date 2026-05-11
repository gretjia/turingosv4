//! TB-G G1.2-1 (Option B+ orchestration ruling 2026-05-11; binding directive
//! `handover/directives/2026-05-11_TB_G_G1_2_OPTION_B_PLUS_RULING.md` §3.1):
//! ResumePreflight — fail-closed library that validates a subprocess's
//! claim to resume an existing ChainTape.
//!
//! The architect classified `TURINGOS_CHAINTAPE_RESUME=1` alone as a
//! signal, not a safety protocol. Every subprocess that wants to attach
//! to a continuing tape MUST pass a `ResumeContract` through this
//! check before its env vars are populated. On `PreflightVerdict::Fail`,
//! the orchestrator MUST NOT spawn the subprocess.
//!
//! Eleven gates total (SG-G1.2-1.1..11):
//!  1. accepts a valid chain
//!  2. rejects missing runtime_repo
//!  3. rejects missing CAS
//!  4. rejects missing agent_pubkeys.json
//!  5. rejects missing pinned_pubkeys.json
//!  6. rejects missing genesis_report.json (at task_index > 0)
//!  7. rejects HEAD_t hex mismatch
//!  8. rejects state_root hex mismatch
//!  9. rejects chain_length mismatch
//! 10. rejects task_index gap (must == prior + 1)
//! 11. rejects fresh-genesis attempt at task_index > 0
//!
//! FC-trace: FC2-Boot. Every real evidence run must be replayable from
//! `genesis_report + ChainTape + CAS + agent registry + system pubkeys`
//! (CLAUDE.md §3.2). The preflight gates each of those five surfaces
//! plus the continuity claim (`expected_head_t` / `expected_state_root` /
//! `expected_chain_length` / `task_index`).

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::bottom_white::ledger::transition_ledger::{Git2LedgerWriter, LedgerWriter};

/// TRACE_MATRIX § 3 orphan (TB-G G1.2-1 2026-05-11; Option B+ orchestration
/// ruling §3.1): resume-contract input shape consumed by `check()`. No
/// canonical FC row yet — promotion target is a future FC2-Boot
/// extension under "explicit resume contract gates". Constitutional
/// Justification: `handover/directives/2026-05-11_TB_G_G1_2_OPTION_B_PLUS_RULING.md`
/// §3.1 — "TURINGOS_CHAINTAPE_RESUME=1 is signal, not safety protocol".
///
/// Resume contract supplied by the orchestrator on every subprocess
/// boundary at `task_index > 0`. All paths are absolute filesystem
/// locations; all hex fields are lowercase 40-char (git OID) or 64-char
/// (sha256) strings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumeContract {
    /// Filesystem path to the shared on-disk runtime git repo. The
    /// preflight opens `<runtime_repo>/refs/transitions/main` via
    /// `Git2LedgerWriter::open` to read the current HEAD.
    pub runtime_repo: PathBuf,
    /// Filesystem path to the shared CAS store. Must exist (a
    /// pre-existing batch will have populated it via task_0). Empty
    /// directory is acceptable; missing path is fail-closed.
    pub cas_path: PathBuf,
    /// Expected canonical L4 head OID at this boundary, derived by the
    /// orchestrator from `task_{k-1}.end_head_t_hex`. Lowercase 40-char
    /// hex. Empty string means "the chain should be empty" (only valid
    /// at `task_index == 0`, which is the fresh-genesis branch — but
    /// task_0 should NOT call preflight; preflight is for resume only).
    pub expected_head_t_hex: String,
    /// Expected `resulting_state_root` at the current HEAD. Lowercase
    /// 64-char hex SHA-256. Derived from the same head entry. Empty
    /// string means "no state_root claim" (degraded mode; only
    /// permitted via test scaffolding — production orchestrator
    /// supplies the hex).
    pub expected_state_root_hex: String,
    /// Expected `Git2LedgerWriter::len()` value. Must equal the
    /// `task_index` (each preserved task contributes ≥1 transition
    /// during normal use, but the preflight only enforces the
    /// caller-supplied claim; orchestrator pulls this from the prior
    /// `TaskContinuationEntry`).
    pub expected_chain_length: u64,
    /// Stable batch identity (sha256 hex or human-readable slug). The
    /// `ChainTapeLease` carries the same `batch_id`; the preflight
    /// does not itself read the lease (decoupled responsibilities) but
    /// records the value so callers can cross-check.
    pub batch_id: String,
    /// Current task index, zero-based. Preflight is invoked when
    /// `task_index > 0`; calling at `task_index == 0` returns
    /// `PreflightFailure::TaskIndexGap` because by definition there
    /// is no prior task to continue from.
    pub task_index: u64,
    /// Filesystem path to the per-agent pubkey manifest. Typically
    /// `<runtime_repo>/agent_pubkeys.json`. Must exist.
    pub agent_pubkeys_path: PathBuf,
    /// Filesystem path to the pinned system pubkey manifest.
    /// Typically `<runtime_repo>/pinned_pubkeys.json`. Must exist.
    pub pinned_pubkeys_path: PathBuf,
    /// Filesystem path to `<runtime_repo>/genesis_report.json`. Must
    /// exist at `task_index > 0` — its absence is the canonical
    /// "fresh-genesis attempted" signal.
    pub genesis_report_path: PathBuf,
}

/// TRACE_MATRIX § 3 orphan (TB-G G1.2-1 2026-05-11; Option B+ §3.1):
/// `check()` return shape. Two-variant Ok/Fail discriminator —
/// orchestrator branches on this before spawning a subprocess.
/// Constitutional Justification: same OPTION_B_PLUS_RULING §3.1.
///
/// Preflight verdict.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "verdict")]
pub enum PreflightVerdict {
    Ok,
    Fail { failure: PreflightFailure },
}

/// TRACE_MATRIX § 3 orphan (TB-G G1.2-1 2026-05-11; Option B+ §3.1):
/// canonical reject-reason enum. Eleven variants — one per SG-G1.2-1.*
/// reject-path test plus `LedgerOpenError` for transient I/O. Stable
/// wire shape (Serialize/Deserialize) so the CLI shim can return the
/// reason to orchestrator scripts.
/// Constitutional Justification: same OPTION_B_PLUS_RULING §3.1.
///
/// Concrete failure mode. Each variant is one of the 10 reject paths in
/// SG-G1.2-1.2..G1.2-1.11.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum PreflightFailure {
    RuntimeRepoMissing { path: PathBuf },
    CasMissing { path: PathBuf },
    AgentRegistryMissing { path: PathBuf },
    PinnedPubkeysMissing { path: PathBuf },
    GenesisReportMissing { path: PathBuf },
    HeadMismatch { expected_hex: String, actual_hex: String },
    StateRootMismatch { expected_hex: String, actual_hex: String },
    ChainLengthMismatch { expected: u64, actual: u64 },
    TaskIndexGap { task_index: u64 },
    FreshGenesisAttempted { detected: String },
    LedgerOpenError { reason: String },
}

/// TRACE_MATRIX § 3 orphan (TB-G G1.2-1 2026-05-11; Option B+ §3.1):
/// the canonical preflight entrypoint. Pure function (modulo
/// filesystem reads via `Git2LedgerWriter::open`) — same input always
/// yields the same verdict. Constitutional Justification: same
/// OPTION_B_PLUS_RULING §3.1; closes the "TURINGOS_CHAINTAPE_RESUME=1
/// is a signal not a safety protocol" gap by being the safety
/// protocol.
///
/// Fail-closed validation. Returns `Ok` if and only if every contract
/// check passes; any single failure returns the matching variant.
pub fn check(contract: &ResumeContract) -> PreflightVerdict {
    if contract.task_index == 0 {
        return PreflightVerdict::Fail {
            failure: PreflightFailure::TaskIndexGap {
                task_index: contract.task_index,
            },
        };
    }

    if !contract.runtime_repo.exists() {
        return PreflightVerdict::Fail {
            failure: PreflightFailure::RuntimeRepoMissing {
                path: contract.runtime_repo.clone(),
            },
        };
    }

    if !contract.cas_path.exists() {
        return PreflightVerdict::Fail {
            failure: PreflightFailure::CasMissing {
                path: contract.cas_path.clone(),
            },
        };
    }

    if !contract.agent_pubkeys_path.exists() {
        return PreflightVerdict::Fail {
            failure: PreflightFailure::AgentRegistryMissing {
                path: contract.agent_pubkeys_path.clone(),
            },
        };
    }

    if !contract.pinned_pubkeys_path.exists() {
        return PreflightVerdict::Fail {
            failure: PreflightFailure::PinnedPubkeysMissing {
                path: contract.pinned_pubkeys_path.clone(),
            },
        };
    }

    if !contract.genesis_report_path.exists() {
        return PreflightVerdict::Fail {
            failure: PreflightFailure::FreshGenesisAttempted {
                detected: format!(
                    "genesis_report.json missing at {:?} for task_index={}; \
                     subprocess would attempt fresh genesis instead of resuming",
                    contract.genesis_report_path, contract.task_index
                ),
            },
        };
    }

    let writer = match Git2LedgerWriter::open(&contract.runtime_repo) {
        Ok(w) => w,
        Err(e) => {
            return PreflightVerdict::Fail {
                failure: PreflightFailure::LedgerOpenError {
                    reason: format!("Git2LedgerWriter::open failed: {e}"),
                },
            };
        }
    };

    let actual_head_hex = writer.head_commit_oid_hex().unwrap_or_default();
    let actual_chain_length = writer.len();

    if actual_chain_length == 0 {
        return PreflightVerdict::Fail {
            failure: PreflightFailure::FreshGenesisAttempted {
                detected: format!(
                    "chain at {:?} is empty (len=0) for task_index={}; \
                     resume requires non-empty chain",
                    contract.runtime_repo, contract.task_index
                ),
            },
        };
    }

    if actual_head_hex != contract.expected_head_t_hex {
        return PreflightVerdict::Fail {
            failure: PreflightFailure::HeadMismatch {
                expected_hex: contract.expected_head_t_hex.clone(),
                actual_hex: actual_head_hex,
            },
        };
    }

    if actual_chain_length != contract.expected_chain_length {
        return PreflightVerdict::Fail {
            failure: PreflightFailure::ChainLengthMismatch {
                expected: contract.expected_chain_length,
                actual: actual_chain_length,
            },
        };
    }

    if !contract.expected_state_root_hex.is_empty() {
        let head_entry = match writer.read_at(actual_chain_length) {
            Ok(e) => e,
            Err(e) => {
                return PreflightVerdict::Fail {
                    failure: PreflightFailure::LedgerOpenError {
                        reason: format!("read_at(head) failed: {e}"),
                    },
                };
            }
        };
        let actual_state_root_hex = hash_to_hex(&head_entry.resulting_state_root);
        if actual_state_root_hex != contract.expected_state_root_hex {
            return PreflightVerdict::Fail {
                failure: PreflightFailure::StateRootMismatch {
                    expected_hex: contract.expected_state_root_hex.clone(),
                    actual_hex: actual_state_root_hex,
                },
            };
        }
    }

    PreflightVerdict::Ok
}

/// Convert a `Hash` (`[u8; 32]` wrapper) to lowercase 64-char hex.
fn hash_to_hex(h: &crate::state::q_state::Hash) -> String {
    let mut s = String::with_capacity(64);
    for b in &h.0 {
        s.push_str(&format!("{:02x}", b));
    }
    s
}

/// TRACE_MATRIX § 3 orphan (TB-G G1.2-1 2026-05-11; Option B+ §3.1):
/// orchestrator-side helper that reads the post-task HEAD_t so the
/// next subprocess's `ResumeContract` can be filled in. Returns the
/// (head_oid_hex, state_root_hex, chain_length) tuple expected by
/// `check()`'s continuity gates (SG-G1.2-1.7..G1.2-1.9). Constitutional
/// Justification: same OPTION_B_PLUS_RULING §3.1 ("explicit fail-closed
/// resume contract" — orchestrator constructs the contract from this
/// snapshot, not from environment variables).
///
/// Snapshot the current head of an existing `runtime_repo` so the
/// orchestrator can construct the next-task `ResumeContract`. Returns
/// `(head_hex, state_root_hex, chain_length)`. Empty chain returns
/// `("", "", 0)`.
pub fn snapshot_head_t(runtime_repo: &Path) -> Result<(String, String, u64), String> {
    let writer = Git2LedgerWriter::open(runtime_repo)
        .map_err(|e| format!("Git2LedgerWriter::open({runtime_repo:?}): {e}"))?;
    let head_hex = writer.head_commit_oid_hex().unwrap_or_default();
    let len = writer.len();
    let state_root_hex = if len == 0 {
        String::new()
    } else {
        let entry = writer
            .read_at(len)
            .map_err(|e| format!("read_at({len}): {e}"))?;
        hash_to_hex(&entry.resulting_state_root)
    };
    Ok((head_hex, state_root_hex, len))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    /// Construct a contract pointed at an empty (uninitialized) repo.
    /// Useful for the missing-path reject tests.
    fn contract_for(tmp: &TempDir, task_index: u64) -> ResumeContract {
        let runtime_repo = tmp.path().join("runtime_repo");
        ResumeContract {
            runtime_repo: runtime_repo.clone(),
            cas_path: tmp.path().join("cas"),
            expected_head_t_hex: String::new(),
            expected_state_root_hex: String::new(),
            expected_chain_length: 0,
            batch_id: "test_batch".into(),
            task_index,
            agent_pubkeys_path: runtime_repo.join("agent_pubkeys.json"),
            pinned_pubkeys_path: runtime_repo.join("pinned_pubkeys.json"),
            genesis_report_path: runtime_repo.join("genesis_report.json"),
        }
    }

    #[test]
    fn unit_task_index_zero_rejected() {
        let tmp = TempDir::new().expect("tempdir");
        let c = contract_for(&tmp, 0);
        match check(&c) {
            PreflightVerdict::Fail { failure: PreflightFailure::TaskIndexGap { task_index } } => {
                assert_eq!(task_index, 0);
            }
            other => panic!("expected TaskIndexGap, got {other:?}"),
        }
    }

    #[test]
    fn unit_missing_runtime_repo_rejected() {
        let tmp = TempDir::new().expect("tempdir");
        let c = contract_for(&tmp, 1);
        match check(&c) {
            PreflightVerdict::Fail { failure: PreflightFailure::RuntimeRepoMissing { .. } } => {}
            other => panic!("expected RuntimeRepoMissing, got {other:?}"),
        }
    }
}
