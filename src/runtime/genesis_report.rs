//! TRACE_MATRIX § 3 orphan (TB-7R 2026-05-02; see
//! `handover/alignment/OBS_R022_TRACE_MATRIX_TB7R_ORPHANS_2026-05-02.md`):
//! TB-7R Deliverable C — genesis-report emitter for ChainTape-mode runs.
//! No canonical TRACE_MATRIX row exists yet (FC2 is canonically
//! Append/Submit, NOT Boot/Genesis); promotion target is a future
//! TRACE_MATRIX revision under the Article IV Boot heading.
//!
//! Per architect verdict 2026-05-01 §6.1: every ChainTape smoke must
//! produce a `genesis_report.json` capturing the constitution + runtime
//! repo + CAS path + system pubkey + agent pubkeys manifest path +
//! initial economic state + (when preseed enabled) the `TaskOpenTx` /
//! `EscrowLockTx` that established the run's task and escrow on-chain.
//!
//! The report is **going-forward only** per verdict B4 — historical
//! evidence dirs receive a README grandfathering note instead of a
//! fabricated `genesis_report.json`. Callers MUST construct the report
//! at run-time, not synthesize it from a finished evidence dir.
//!
//! `FC-trace: Art.IV Boot (Bootstrap 公理 — 创世状态) + Art.I.1 + Art.III.4
//! + WP-§5.L0 (Constitution Root) + WP-§11 Boot`.

use std::path::Path;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// TRACE_MATRIX § 3 orphan (see module docstring + OBS_R022_TRACE_MATRIX_TB7R_ORPHANS_2026-05-02):
/// on-disk shape of the run's genesis report. Written to
/// `<runtime_repo>/genesis_report.json` after chaintape bootstrap
/// (and after any pre-seed TaskOpen + EscrowLock submission, when
/// applicable). Public fields below inherit this struct's TRACE_MATRIX
/// backlink rather than each carrying their own (per OBS § public-field
/// doc-comment policy).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenesisReport {
    /// SHA-256 of `constitution.md` at run time (hex). When the run is
    /// inside a checked-out repo with a `constitution.md`, the file is
    /// read and hashed live. When the file cannot be read (sandbox
    /// scenarios, packaged binary), this is `None`.
    pub constitution_hash: Option<String>,

    /// Filesystem path to the on-disk runtime git repo
    /// (`<runtime_repo>`). Same value as `ChaintapeBundle::runtime_repo_path`.
    pub runtime_repo: String,

    /// Filesystem path to the CAS store. Same value as
    /// `ChaintapeBundle::cas_path`.
    pub cas_path: String,

    /// SHA-256 (hex) of the system pubkey at the active epoch — read
    /// from `<runtime_repo>/pinned_pubkeys.json`. Allows post-hoc
    /// cross-check that signature verification was anchored at the
    /// expected system epoch. `None` if the manifest is unreadable.
    pub system_pubkey_hash: Option<String>,

    /// Filesystem path (relative to `runtime_repo`) of the per-agent
    /// pubkey manifest — populated at first agent registration.
    pub agent_pubkeys_path: String,

    /// Initial agent balances seeded into the genesis QState
    /// (preseed-enabled runs only). Empty vec when preseed disabled.
    /// Each entry is `(agent_id, micro_units)` — micro-coin scale.
    pub initial_balances: Vec<(String, i64)>,

    /// Task ID established by the preseed TaskOpen transaction, when
    /// the run pre-seeds a task / escrow. `None` when preseed disabled
    /// (no task is opened by the bootstrap; runs operate without a
    /// formal task / escrow).
    pub task_id: Option<String>,

    /// `tx_id` of the preseed `TaskOpenTx` submitted at bootstrap.
    /// `None` when preseed disabled.
    pub task_open_tx: Option<String>,

    /// `tx_id` of the preseed `EscrowLockTx` submitted at bootstrap.
    /// `None` when preseed disabled.
    pub escrow_lock_tx: Option<String>,
}

impl GenesisReport {
    /// TRACE_MATRIX § 3 orphan (see module docstring + OBS_R022_TRACE_MATRIX_TB7R_ORPHANS_2026-05-02):
    /// write the report to `<runtime_repo>/genesis_report.json` as
    /// pretty-printed JSON.
    /// Caller MUST ensure `runtime_repo` exists. Overwrites any prior
    /// report at the same path.
    pub fn write_to_runtime_repo(&self, runtime_repo: &Path) -> std::io::Result<()> {
        let path = runtime_repo.join("genesis_report.json");
        let json = serde_json::to_string_pretty(self).map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("genesis_report serialize: {e}"),
            )
        })?;
        std::fs::write(path, json)
    }

    /// TRACE_MATRIX § 3 orphan (see module docstring + OBS_R022_TRACE_MATRIX_TB7R_ORPHANS_2026-05-02):
    /// hash a constitution.md file to the hex SHA-256 used in
    /// `constitution_hash`. Returns `None` if the file cannot be read.
    pub fn hash_constitution_md(constitution_path: &Path) -> Option<String> {
        let bytes = std::fs::read(constitution_path).ok()?;
        let mut hasher = Sha256::new();
        hasher.update(&bytes);
        Some(hex_encode(&hasher.finalize()))
    }

    /// TRACE_MATRIX § 3 orphan (see module docstring + OBS_R022_TRACE_MATRIX_TB7R_ORPHANS_2026-05-02):
    /// hash the contents of `pinned_pubkeys.json` to derive a stable
    /// identifier for the system epoch. Returns `None` if the file
    /// cannot be read.
    pub fn hash_system_pubkey_manifest(runtime_repo: &Path) -> Option<String> {
        let bytes = std::fs::read(runtime_repo.join("pinned_pubkeys.json")).ok()?;
        let mut hasher = Sha256::new();
        hasher.update(&bytes);
        Some(hex_encode(&hasher.finalize()))
    }
}

fn hex_encode(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        s.push_str(&format!("{:02x}", b));
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn write_round_trips_via_serde_json() {
        let tmp = TempDir::new().expect("tempdir");
        let report = GenesisReport {
            constitution_hash: Some("abc123".into()),
            runtime_repo: tmp.path().display().to_string(),
            cas_path: tmp.path().join("cas").display().to_string(),
            system_pubkey_hash: Some("def456".into()),
            agent_pubkeys_path: "agent_pubkeys.json".into(),
            initial_balances: vec![("Agent_0".into(), 1_000_000)],
            task_id: Some("task-runX".into()),
            task_open_tx: Some("taskopen-task-runX-seed".into()),
            escrow_lock_tx: Some("escrowlock-task-runX-escrow".into()),
        };

        report
            .write_to_runtime_repo(tmp.path())
            .expect("write should succeed");

        let read = std::fs::read_to_string(tmp.path().join("genesis_report.json"))
            .expect("read should succeed");
        let round: GenesisReport =
            serde_json::from_str(&read).expect("should round-trip via serde_json");

        assert_eq!(round.constitution_hash, Some("abc123".into()));
        assert_eq!(round.runtime_repo, tmp.path().display().to_string());
        assert_eq!(round.task_id, Some("task-runX".into()));
        assert_eq!(round.initial_balances.len(), 1);
        assert_eq!(round.initial_balances[0].0, "Agent_0");
        assert_eq!(round.initial_balances[0].1, 1_000_000);
    }

    #[test]
    fn hash_constitution_md_returns_some_for_existing_file() {
        let tmp = TempDir::new().expect("tempdir");
        let path = tmp.path().join("constitution.md");
        std::fs::write(&path, b"tiny test constitution body").expect("write");

        let h = GenesisReport::hash_constitution_md(&path).expect("hash should succeed");
        // SHA-256 of "tiny test constitution body" — deterministic.
        assert_eq!(h.len(), 64);
        assert!(h.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn hash_constitution_md_returns_none_for_missing_file() {
        let tmp = TempDir::new().expect("tempdir");
        let path = tmp.path().join("does-not-exist.md");
        assert!(GenesisReport::hash_constitution_md(&path).is_none());
    }

    #[test]
    fn no_preseed_means_optional_fields_are_none() {
        let tmp = TempDir::new().expect("tempdir");
        let report = GenesisReport {
            constitution_hash: None,
            runtime_repo: tmp.path().display().to_string(),
            cas_path: tmp.path().join("cas").display().to_string(),
            system_pubkey_hash: None,
            agent_pubkeys_path: "agent_pubkeys.json".into(),
            initial_balances: vec![],
            task_id: None,
            task_open_tx: None,
            escrow_lock_tx: None,
        };

        report
            .write_to_runtime_repo(tmp.path())
            .expect("write should succeed even with all None preseed fields");
        let read = std::fs::read_to_string(tmp.path().join("genesis_report.json")).unwrap();
        assert!(read.contains("\"task_id\": null"));
        assert!(read.contains("\"task_open_tx\": null"));
        assert!(read.contains("\"escrow_lock_tx\": null"));
    }
}
