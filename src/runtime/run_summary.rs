//! TB-6 Atom 6 — Branch / fork visibility summary.
//!
//! Architect ruling 2026-05-01 § 3.6 Atom 6:
//! > Record: `tx_count`, `failed_branch_count`, `rollback_count`, candidate
//! > proposal CIDs, accepted tx_id, rejected tx_ids.
//! > **Records proposal-level fork, not chain-of-thought.**
//!
//! The summary aggregates the run's chaintape state at exit time. Sources:
//! - `Git2LedgerWriter::len()` for `tx_count` accepted side + `read_at(t)` to
//!   pull each entry's `tx_payload_cid`. CAS lookup + `canonical_decode`
//!   yields the `TypedTx` and the variant's `tx_id` field.
//! - `RejectionEvidenceWriter::open_jsonl` for the rejected side. Same
//!   CAS-lookup-and-decode shape for tx_id extraction.
//! - `failed_branch_count` + `rollback_count` are caller-supplied (the
//!   evaluator already computes both inside its accumulator state; the
//!   aggregator threads them through).
//!
//! Emitted at end of run alongside `PputResult` as
//! `<runtime_repo>/run_summary.json` and / or `<evidence_dir>/run_summary.json`.
//!
//! Per charter § 4.2: NO chain-of-thought / private model deliberation.
//! `RunSummary` records what was submitted (tx_id sets) and what failed
//! (`failed_branch_count`); it does not record the deliberation that led
//! to those outcomes. Agent audit records (`AgentProposalRecord`, Atom 5)
//! are the linked-evidence per-proposal layer.

use std::collections::BTreeSet;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::bottom_white::cas::schema::Cid;
use crate::bottom_white::cas::store::CasStore;
use crate::bottom_white::ledger::rejection_evidence::RejectionEvidenceWriter;
use crate::bottom_white::ledger::transition_ledger::{
    canonical_decode, Git2LedgerWriter, LedgerWriter,
};
use crate::state::q_state::TxId;
use crate::state::typed_tx::TypedTx;

const REJECTIONS_JSONL_FILENAME: &str = "rejections.jsonl";
const RUN_SUMMARY_FILENAME: &str = "run_summary.json";

// ── Errors ──────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC3-N1: TB-6 Atom 6 — RunSummary aggregation error class.
#[derive(Debug)]
pub enum RunSummaryError {
    Io(std::io::Error),
    LedgerWriter(String),
    Cas(String),
    Decode(String),
    L4eOpen(String),
}

impl std::fmt::Display for RunSummaryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "io error: {e}"),
            Self::LedgerWriter(s) => write!(f, "ledger writer error: {s}"),
            Self::Cas(s) => write!(f, "cas error: {s}"),
            Self::Decode(s) => write!(f, "decode error: {s}"),
            Self::L4eOpen(s) => write!(f, "rejections.jsonl open error: {s}"),
        }
    }
}

impl std::error::Error for RunSummaryError {}

impl From<std::io::Error> for RunSummaryError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

// ── Wire format ─────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC3-N1: TB-6 Atom 6 — branch / fork visibility summary.
///
/// Wire format (`run_summary.json` in evidence dir / runtime_repo). Stable
/// JSON shape suitable for downstream tooling + Atom 7 ship audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunSummary {
    /// Identifier echoed from the run; matches `pinned_pubkeys.json.run_id`
    /// when populated, or the evaluator's run_id otherwise.
    pub run_id: String,
    /// Total submission count seen by the chaintape side
    /// (accepted L4 + rejected L4.E). Other sequencer rejection paths
    /// (e.g. signature-decode failure pre-queue) are NOT reflected here —
    /// those don't produce L4.E records.
    pub tx_count: u64,
    /// Caller-supplied count of LLM proposals that failed branch-acceptance
    /// in the run loop (mirrors `PputResult.failed_branch_count`). Distinct
    /// from `rejected_tx_ids.len()`, which counts only chaintape-side L4.E
    /// rejections.
    pub failed_branch_count: u64,
    /// Caller-supplied count of explicit rollbacks in the run loop
    /// (mirrors `PputResult.rollback_count`).
    pub rollback_count: u64,
    /// Sorted, deduplicated list of `tx_id`s that landed in L4 (accepted).
    pub accepted_tx_ids: Vec<TxId>,
    /// Sorted, deduplicated list of `tx_id`s that landed in L4.E (rejected).
    pub rejected_tx_ids: Vec<TxId>,
    /// Candidate proposal CIDs — the union of `tx_payload_cid`s referenced
    /// by L4 + L4.E entries. CAS-content-addressed; same proposal payload
    /// across runs collides to the same CID by design (idempotent CAS).
    pub candidate_proposal_cids: Vec<Cid>,
    /// L4 chain length at run-exit.
    pub l4_entries: u64,
    /// L4.E chain length at run-exit.
    pub l4e_entries: u64,
}

impl RunSummary {
    /// TRACE_MATRIX FC3-N1: TB-6 Atom 6 — re-open `runtime_repo` + `cas_path`
    /// at end-of-run, decode every L4 + L4.E payload from CAS, extract the
    /// `tx_id` from the typed payload, and aggregate into a `RunSummary`.
    ///
    /// `failed_branch_count` and `rollback_count` are caller-supplied because
    /// they're properties of the LLM run loop (not of the chaintape).
    /// Mirrors `PputResult` so smoke evidence stays cross-consistent.
    pub fn from_chaintape(
        runtime_repo_path: &Path,
        cas_path: &Path,
        run_id: &str,
        failed_branch_count: u64,
        rollback_count: u64,
    ) -> Result<Self, RunSummaryError> {
        let writer = Git2LedgerWriter::open(runtime_repo_path)
            .map_err(|e| RunSummaryError::LedgerWriter(e.to_string()))?;
        let l4_entries = writer.len();

        let cas = CasStore::open(cas_path).map_err(|e| RunSummaryError::Cas(e.to_string()))?;

        let mut accepted_tx_ids: BTreeSet<TxId> = BTreeSet::new();
        let mut candidate_cids: BTreeSet<Cid> = BTreeSet::new();
        for t in 1..=l4_entries {
            let entry = writer
                .read_at(t)
                .map_err(|e| RunSummaryError::LedgerWriter(e.to_string()))?;
            candidate_cids.insert(entry.tx_payload_cid);
            let bytes = cas
                .get(&entry.tx_payload_cid)
                .map_err(|e| RunSummaryError::Cas(e.to_string()))?;
            let typed_tx: TypedTx = canonical_decode(&bytes)
                .map_err(|e| RunSummaryError::Decode(e.to_string()))?;
            accepted_tx_ids.insert(extract_tx_id(&typed_tx));
        }

        let rejections_path = runtime_repo_path.join(REJECTIONS_JSONL_FILENAME);
        let l4e_writer = if rejections_path.exists() {
            RejectionEvidenceWriter::open_jsonl(rejections_path)
                .map_err(|e| RunSummaryError::L4eOpen(e.to_string()))?
        } else {
            RejectionEvidenceWriter::new()
        };
        let l4e_entries = l4e_writer.len() as u64;

        let mut rejected_tx_ids: BTreeSet<TxId> = BTreeSet::new();
        for record in l4e_writer.records() {
            candidate_cids.insert(record.tx_payload_cid);
            // CAS lookup may fail if the rejected tx_payload was emitted
            // pre-CAS-write (paranoid path); skip with no error rather than
            // failing the whole summary.
            if let Ok(bytes) = cas.get(&record.tx_payload_cid) {
                if let Ok(typed_tx) = canonical_decode::<TypedTx>(&bytes) {
                    rejected_tx_ids.insert(extract_tx_id(&typed_tx));
                }
            }
        }

        let tx_count = l4_entries + l4e_entries;
        Ok(Self {
            run_id: run_id.to_string(),
            tx_count,
            failed_branch_count,
            rollback_count,
            accepted_tx_ids: accepted_tx_ids.into_iter().collect(),
            rejected_tx_ids: rejected_tx_ids.into_iter().collect(),
            candidate_proposal_cids: candidate_cids.into_iter().collect(),
            l4_entries,
            l4e_entries,
        })
    }

    /// TRACE_MATRIX FC3-N1: TB-6 Atom 6 — write the summary to the canonical
    /// `<runtime_repo>/run_summary.json` location (or any path the caller
    /// chooses). Pretty-printed for human + tooling forensics.
    pub fn write_json(&self, path: &Path) -> Result<(), RunSummaryError> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| RunSummaryError::Decode(e.to_string()))?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, json)?;
        Ok(())
    }

    /// TRACE_MATRIX FC3-N1: TB-6 Atom 6 — convenience: write to the canonical
    /// `<runtime_repo>/run_summary.json` location.
    pub fn write_canonical(&self, runtime_repo_path: &Path) -> Result<(), RunSummaryError> {
        self.write_json(&runtime_repo_path.join(RUN_SUMMARY_FILENAME))
    }
}

// ── Helpers ─────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC3-N1: TB-6 Atom 6 — extract the `tx_id` field from each
/// `TypedTx` variant. Inlined here to avoid mutating `typed_tx.rs` (which is
/// kernel-adjacent and we want to keep the TB-6 footprint minimal).
fn extract_tx_id(tx: &TypedTx) -> TxId {
    match tx {
        TypedTx::Work(t) => t.tx_id.clone(),
        TypedTx::Verify(t) => t.tx_id.clone(),
        TypedTx::Challenge(t) => t.tx_id.clone(),
        TypedTx::Reuse(t) => t.tx_id.clone(),
        TypedTx::FinalizeReward(t) => t.tx_id.clone(),
        TypedTx::TaskExpire(t) => t.tx_id.clone(),
        TypedTx::TerminalSummary(t) => t.tx_id.clone(),
        TypedTx::TaskOpen(t) => t.tx_id.clone(),
        TypedTx::EscrowLock(t) => t.tx_id.clone(),
        TypedTx::ChallengeResolve(t) => t.tx_id.clone(),
    }
}

// ── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn run_summary_serde_round_trip_preserves_all_fields() {
        let s = RunSummary {
            run_id: "test".into(),
            tx_count: 5,
            failed_branch_count: 2,
            rollback_count: 1,
            accepted_tx_ids: vec![TxId("worktx-1".into())],
            rejected_tx_ids: vec![TxId("worktx-2".into()), TxId("worktx-3".into())],
            candidate_proposal_cids: vec![Cid([1u8; 32]), Cid([2u8; 32])],
            l4_entries: 1,
            l4e_entries: 2,
        };
        let json = serde_json::to_string(&s).unwrap();
        let decoded: RunSummary = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded, s);
    }

    #[test]
    fn write_json_creates_parent_dirs_and_file() {
        let tmp = TempDir::new().unwrap();
        let s = RunSummary {
            run_id: "wjt".into(),
            tx_count: 0,
            failed_branch_count: 0,
            rollback_count: 0,
            accepted_tx_ids: vec![],
            rejected_tx_ids: vec![],
            candidate_proposal_cids: vec![],
            l4_entries: 0,
            l4e_entries: 0,
        };
        let target = tmp.path().join("nested").join("dir").join("run_summary.json");
        s.write_json(&target).unwrap();
        assert!(target.exists());
        let bytes = std::fs::read_to_string(&target).unwrap();
        assert!(bytes.contains("\"run_id\""));
    }

    #[test]
    fn extract_tx_id_handles_all_variants() {
        // Spot-check: TaskOpen + Work variants share the inner field name.
        // Trying every variant would require constructing each, which the
        // adapter helpers do not cover. The Atom 3 + I90 integration tests
        // exercise the real CAS round-trip on TaskOpen / Work; this unit
        // test covers the synthetic-construction path.
        use crate::runtime::adapter::{make_synthetic_task_open, make_synthetic_worktx};
        let to = make_synthetic_task_open("t-eo", "s", crate::state::q_state::Hash::ZERO, "extract");
        let wo = make_synthetic_worktx("t-eo", "a", crate::state::q_state::Hash::ZERO, 0, "extract", true);
        assert_eq!(extract_tx_id(&to).0, "taskopen-t-eo-extract");
        assert_eq!(extract_tx_id(&wo).0, "worktx-t-eo-extract");
    }
}
