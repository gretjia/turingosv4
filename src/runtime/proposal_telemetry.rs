//! TB-7 Atom 1.5 — `ProposalTelemetry` CAS object writer.
//!
//! Per ARCHITECT_RULING 2026-05-01 D5 + TB-7 charter §4.5: every `WorkTx`
//! routed through `bus.submit_typed_tx` carries a `proposal_cid` pointing to
//! a CAS-resident `ProposalTelemetry` object. This is the precondition for
//! `golden_path_token_count` to be claimed chain-derived (charter §4.4 Atom 5
//! `ChainDerivedRunFacts`).
//!
//! Schema (binding per ruling D5):
//! ```text
//! {
//!   "agent_id": "<string>",
//!   "prompt_context_hash": "<hex>",
//!   "proposal_artifact_cid": "<cid>",
//!   "candidate_tactic": "<string>",
//!   "token_counts": {
//!     "prompt_tokens": 0,
//!     "completion_tokens": 0,
//!     "tool_tokens": 0
//!   },
//!   "tool_calls": [],
//!   "branch_id": "<string>",
//!   "parent_tx": "<TxId or null>"
//! }
//! ```
//!
//! Distinction from `AgentProposalRecord` (TB-6 Atom 5):
//! - `AgentProposalRecord` records what the Agent **saw** + **submitted** +
//!   how the system **judged** (predicate results, accept/reject).
//! - `ProposalTelemetry` records LLM-driven proposal metadata: token usage,
//!   tool-call manifest, branch chronology, candidate tactic. This is
//!   evidence the chain itself does NOT bind into state but DOES bind via
//!   `WorkTx.proposal_cid` so chain-derived run facts (golden_path_token_count
//!   etc.) can be byte-deterministically reconstructed.
//! - Both live in CAS and are content-addressed.
//!
//! TRACE_MATRIX FC1-N14 (wtool / authoritative state-mutation path; CAS
//! object that grounds `WorkTx.proposal_cid` per TB-7 §4.5 / Gate 5 / Atom 4
//! retrievability test).
//!
//! Storage shape mirrors the proven `AgentProposalRecord` CAS pattern:
//! - canonical-encoded (bincode v2 BE + fixed-int) for byte-stable CID
//! - put via `CasStore::put` with `ObjectType::Generic` + schema id
//!   `turingosv4.proposal_telemetry.v1`
//! - retrievable via `read_from_cas` for replay / `verify_chaintape` extension

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::Path;

use crate::bottom_white::cas::schema::{Cid, ObjectType};
use crate::bottom_white::cas::store::{CasError, CasStore};
use crate::bottom_white::ledger::transition_ledger::{canonical_decode, canonical_encode};
use crate::state::q_state::{AgentId, Hash, TxId};

const PROPOSAL_TELEMETRY_SCHEMA_ID: &str = "turingosv4.proposal_telemetry.v1";

// ── Token counts ────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC1-N14: per-proposal token-usage record. Sourced from the
/// LLM SDK / wrapper at proposal time; carried into `golden_path_token_count`
/// computation in `ChainDerivedRunFacts` (Atom 5).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TokenCounts {
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub tool_tokens: u64,
}

impl TokenCounts {
    /// TRACE_MATRIX FC1-N14: total tokens (prompt + completion + tool).
    /// Consumed by `golden_path_token_count` aggregator in `ChainDerivedRunFacts`.
    pub const fn total(&self) -> u64 {
        self.prompt_tokens + self.completion_tokens + self.tool_tokens
    }
}

// ── Tool call manifest ──────────────────────────────────────────────────────

/// TRACE_MATRIX FC1-N14: a single tool-call from the LLM trajectory. Records
/// only the structural shape (tool name + 32-byte argument hash + 32-byte
/// result hash). The full argument / result bytes go into separate CAS objects
/// or stay agent-internal — the audit trail records the call **happened**, not
/// the chain-of-thought that picked it.
///
/// Forbidden contents (TB-6 charter §6 #11 + TB-7 §6 inheritance): raw model
/// deliberation, raw tool transcripts, internal reasoning, raw prompt/completion
/// strings. Hashes only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ToolCallRecord {
    pub tool_id: String,
    pub args_hash: Hash,
    pub result_hash: Hash,
}

// ── Telemetry record ────────────────────────────────────────────────────────

/// TRACE_MATRIX FC1-N14: the per-WorkTx LLM proposal telemetry object pointed
/// to by `WorkTx.proposal_cid`. Schema per ARCHITECT_RULING 2026-05-01 D5 +
/// TB-7 charter §4.5.
///
/// **Field set (binding; do NOT add fields without architect ratification)**:
/// 1. `agent_id` — must match `WorkTx.agent_id`
/// 2. `prompt_context_hash` — 32-byte sha256; same as `AgentProposalRecord.prompt_context_hash`
/// 3. `proposal_artifact_cid` — CID of the actual proposal payload bytes
///    (proof artifact / candidate tactic body / tool program); separate from
///    this telemetry record's own CID
/// 4. `candidate_tactic` — short identifier for the proposed tactic
///    (e.g. "nlinarith", "ring", "rfl", "induction"); aggregated by
///    `tactic_diversity` in `ChainDerivedRunFacts`
/// 5. `token_counts` — prompt / completion / tool token counts; aggregated
///    by `golden_path_token_count` in `ChainDerivedRunFacts`
/// 6. `tool_calls` — ordered manifest of tool invocations during proposal
///    construction; aggregated by `tool_dist` in `ChainDerivedRunFacts`
/// 7. `branch_id` — short branch label (e.g. "n1.b0", "swarm_a.b3"); used by
///    `failed_branch_count` aggregator
/// 8. `parent_tx` — `TxId` of the parent WorkTx if this proposal was
///    derivative; `None` for root proposals
/// 9. **TB-7.7 D4**: `verification_result_cid` — optional CID to a
///    `VerificationResult` CAS object recording the Lean oracle's
///    verdict (exit code + verified flag + proof artifact hash).
///    `None` for proposals not yet Lean-verified (append-branch
///    intermediate steps); `Some(cid)` for OMEGA-accept proposals
///    where the evaluator has run Lean and recorded the verdict.
///    Replay readers use this to compute `chain_oracle_verified`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProposalTelemetry {
    pub agent_id: AgentId,
    pub prompt_context_hash: Hash,
    pub proposal_artifact_cid: Cid,
    pub candidate_tactic: String,
    pub token_counts: TokenCounts,
    pub tool_calls: Vec<ToolCallRecord>,
    pub branch_id: String,
    pub parent_tx: Option<TxId>,
    /// TB-7.7 D4: optional CID of the matching `VerificationResult` CAS
    /// object (`runtime::verification_result::VerificationResult`).
    /// Schema-additive; `None` preserves backward compat with pre-TB-7.7
    /// telemetry.
    #[serde(default)]
    pub verification_result_cid: Option<Cid>,
}

impl ProposalTelemetry {
    /// TRACE_MATRIX FC1-N14: convenience constructor for the common case where
    /// the proposal has no parent (root proposal). Used by Atom 2 evaluator
    /// hooks that don't yet track branch lineage.
    pub fn new_root(
        agent_id: AgentId,
        prompt_context_hash: Hash,
        proposal_artifact_cid: Cid,
        candidate_tactic: String,
        token_counts: TokenCounts,
        branch_id: String,
    ) -> Self {
        Self {
            agent_id,
            prompt_context_hash,
            proposal_artifact_cid,
            candidate_tactic,
            token_counts,
            tool_calls: Vec::new(),
            branch_id,
            parent_tx: None,
            verification_result_cid: None,
        }
    }

    /// TRACE_MATRIX FC1-N14: TB-7 Atom 2 — high-level builder for the
    /// evaluator hot path.
    ///
    /// **TB-7.7 fix (2026-05-01)**: this function now ACTUALLY WRITES the
    /// proposal payload bytes to CAS. Pre-TB-7.7 it computed
    /// `proposal_artifact_cid = sha256(payload_bytes)` but never stored
    /// the bytes — meaning a chain reader could verify "a payload with
    /// this hash existed" but could not recover the payload content from
    /// ChainTape + CAS alone (architect ruling 2026-05-01 ultrathink turn
    /// flagged this as the #1 hidden hole in real chaintape).
    ///
    /// Now `proposal_artifact_cid` is the CID returned by
    /// `cas.put(payload_bytes, ObjectType::ProposalPayload, ...)`. The
    /// bytes are durably stored under that CID.
    ///
    /// `parent_tx` is `None` here for backward compat; callers that want
    /// to record branch lineage should use
    /// [`build_for_evaluator_append_with_parent`] instead.
    pub fn build_for_evaluator_append(
        cas_store: &mut CasStore,
        run_id: &str,
        agent_id: &str,
        proposal_index: u64,
        payload_bytes: &[u8],
        candidate_tactic: &str,
        token_counts: TokenCounts,
        creator: &str,
        logical_t: u64,
    ) -> Result<Self, ProposalTelemetryError> {
        Self::build_for_evaluator_append_with_parent(
            cas_store,
            run_id,
            agent_id,
            proposal_index,
            payload_bytes,
            candidate_tactic,
            token_counts,
            creator,
            logical_t,
            None,
        )
    }

    /// TRACE_MATRIX FC1-N14: TB-7.7 Deliverable 2 — variant that records
    /// `parent_tx` for branch lineage / DAG-edge reconstruction.
    /// Evaluator hot path passes `Some(last_tx_id)` for the same
    /// (agent_id, branch_id) pair after at least one prior submission.
    #[allow(clippy::too_many_arguments)]
    pub fn build_for_evaluator_append_with_parent(
        cas_store: &mut CasStore,
        run_id: &str,
        agent_id: &str,
        proposal_index: u64,
        payload_bytes: &[u8],
        candidate_tactic: &str,
        token_counts: TokenCounts,
        creator: &str,
        logical_t: u64,
        parent_tx: Option<TxId>,
    ) -> Result<Self, ProposalTelemetryError> {
        let mut hctx = Sha256::new();
        hctx.update(b"turingosv4.tb7.atom2.prompt_context.v1");
        hctx.update(run_id.as_bytes());
        hctx.update(agent_id.as_bytes());
        hctx.update(proposal_index.to_be_bytes());
        let prompt_context_hash = Hash(hctx.finalize().into());

        // TB-7.7 D1: actually store the payload bytes in CAS. The returned
        // CID IS proposal_artifact_cid — content-addressed and durably
        // retrievable via cas_store.get(cid).
        let proposal_artifact_cid = cas_store.put(
            payload_bytes,
            ObjectType::ProposalPayload,
            creator,
            logical_t,
            Some("turingosv4.proposal_payload.v1".into()),
        )?;

        Ok(Self {
            agent_id: AgentId(agent_id.to_string()),
            prompt_context_hash,
            proposal_artifact_cid,
            candidate_tactic: candidate_tactic.to_string(),
            token_counts,
            tool_calls: Vec::new(),
            branch_id: format!("{}.b{}", agent_id, proposal_index),
            parent_tx,
            verification_result_cid: None,
        })
    }

    /// TRACE_MATRIX FC1-N14: TB-7.7 D4 — attach a `VerificationResult`
    /// CAS object's CID after Lean has run. Used by evaluator OMEGA-accept
    /// hot path to record the oracle verdict before the WorkTx is
    /// submitted. Pre-existing telemetry (without this method having been
    /// called) keeps `verification_result_cid: None`.
    pub fn with_verification_result(mut self, cid: Cid) -> Self {
        self.verification_result_cid = Some(cid);
        self
    }
}

// ── Errors ──────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC1-N14: proposal-telemetry CAS error taxonomy.
#[derive(Debug)]
pub enum ProposalTelemetryError {
    Cas(CasError),
    Codec(String),
    Io(std::io::Error),
}

impl std::fmt::Display for ProposalTelemetryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cas(e) => write!(f, "cas error: {e}"),
            Self::Codec(s) => write!(f, "codec error: {s}"),
            Self::Io(e) => write!(f, "io error: {e}"),
        }
    }
}

impl std::error::Error for ProposalTelemetryError {}

impl From<CasError> for ProposalTelemetryError {
    fn from(e: CasError) -> Self {
        Self::Cas(e)
    }
}

impl From<std::io::Error> for ProposalTelemetryError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

// ── CAS storage ─────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC1-N14: canonical-encode the telemetry record + CAS put.
/// Returns the content-addressed CID. Idempotent (same record → same CID),
/// so `WorkTx.proposal_cid` is byte-stable across runs as long as the
/// telemetry payload is byte-identical.
pub fn write_to_cas(
    cas: &mut CasStore,
    record: &ProposalTelemetry,
    creator: &str,
    logical_t: u64,
) -> Result<Cid, ProposalTelemetryError> {
    let bytes =
        canonical_encode(record).map_err(|e| ProposalTelemetryError::Codec(e.to_string()))?;
    let cid = cas.put(
        &bytes,
        ObjectType::Generic,
        creator,
        logical_t,
        Some(PROPOSAL_TELEMETRY_SCHEMA_ID.to_string()),
    )?;
    Ok(cid)
}

/// TRACE_MATRIX FC1-N14: CAS fetch + canonical-decode. Used by Atom 4
/// `verify_chaintape` extension to retrieve and validate
/// `WorkTx.proposal_cid` references during replay.
pub fn read_from_cas(
    cas: &CasStore,
    cid: &Cid,
) -> Result<ProposalTelemetry, ProposalTelemetryError> {
    let bytes = cas.get(cid)?;
    canonical_decode::<ProposalTelemetry>(&bytes)
        .map_err(|e| ProposalTelemetryError::Codec(e.to_string()))
}

/// TRACE_MATRIX FC1-N14: convenience — open a CAS at `cas_path` and read the
/// telemetry record at `cid`. Used by `verify_chaintape` CLI which needs to
/// resolve `WorkTx.proposal_cid` references with only the on-disk paths.
pub fn read_from_cas_path(
    cas_path: &Path,
    cid: &Cid,
) -> Result<ProposalTelemetry, ProposalTelemetryError> {
    let cas = CasStore::open(cas_path)?;
    read_from_cas(&cas, cid)
}

// ── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use sha2::{Digest, Sha256};
    use tempfile::TempDir;

    fn fresh_cas() -> (TempDir, CasStore) {
        let dir = TempDir::new().expect("tempdir");
        let cas = CasStore::open(dir.path()).expect("open cas");
        (dir, cas)
    }

    fn fresh_record(agent: &str, branch: &str) -> ProposalTelemetry {
        let mut h = Sha256::new();
        h.update(b"telemetry.test.prompt_context");
        h.update(agent.as_bytes());
        let prompt_hash = Hash(h.finalize().into());
        ProposalTelemetry::new_root(
            AgentId(agent.into()),
            prompt_hash,
            Cid([7u8; 32]),
            "nlinarith".into(),
            TokenCounts {
                prompt_tokens: 100,
                completion_tokens: 50,
                tool_tokens: 0,
            },
            branch.into(),
        )
    }

    /// U-A1.5.a — write + read round-trip yields the same record.
    #[test]
    fn write_read_round_trip() {
        let (_dir, mut cas) = fresh_cas();
        let record = fresh_record("n1", "n1.b0");
        let cid = write_to_cas(&mut cas, &record, "tb7-atom1.5-test", 1).expect("write");
        let loaded = read_from_cas(&cas, &cid).expect("read");
        assert_eq!(record, loaded);
    }

    /// U-A1.5.b — same record yields the same CID across calls (content
    /// addressing is byte-deterministic). This is the property that lets
    /// `WorkTx.proposal_cid` be byte-stable across reruns of the same
    /// proposal.
    #[test]
    fn cid_determinism() {
        let (_dir, mut cas) = fresh_cas();
        let record = fresh_record("n1", "n1.b0");
        let cid1 = write_to_cas(&mut cas, &record, "tb7-atom1.5-test", 1).expect("write1");
        let cid2 = write_to_cas(&mut cas, &record, "tb7-atom1.5-test", 1).expect("write2");
        assert_eq!(cid1, cid2);
    }

    /// U-A1.5.c — different records yield different CIDs (basic anti-collision).
    #[test]
    fn distinct_records_distinct_cids() {
        let (_dir, mut cas) = fresh_cas();
        let r1 = fresh_record("n1", "n1.b0");
        let r2 = fresh_record("swarm_a", "swarm_a.b1");
        let cid1 = write_to_cas(&mut cas, &r1, "tb7-atom1.5-test", 1).expect("w1");
        let cid2 = write_to_cas(&mut cas, &r2, "tb7-atom1.5-test", 1).expect("w2");
        assert_ne!(cid1, cid2);
    }

    /// U-A1.5.d — schema validity: the 8 binding fields per ruling D5 +
    /// 1 TB-7.7 D4 additive field (`verification_result_cid`).
    /// **TB-7.7 D4 update**: was 8 fields pre-TB-7.7; now 9 with the
    /// schema-additive `verification_result_cid: Option<Cid>` (default
    /// `None`). The original 8 ruling-D5 fields are unchanged; this is
    /// purely additive.
    #[test]
    fn schema_validity_nine_fields_with_verification_result() {
        let record = fresh_record("n1", "n1.b0");
        let json = serde_json::to_value(&record).expect("serialize");
        let obj = json.as_object().expect("object");
        assert_eq!(
            obj.len(),
            9,
            "ProposalTelemetry must have 9 fields (8 ruling-D5 + 1 TB-7.7 D4 verification_result_cid)"
        );
        assert!(obj.contains_key("agent_id"));
        assert!(obj.contains_key("prompt_context_hash"));
        assert!(obj.contains_key("proposal_artifact_cid"));
        assert!(obj.contains_key("candidate_tactic"));
        assert!(obj.contains_key("token_counts"));
        assert!(obj.contains_key("tool_calls"));
        assert!(obj.contains_key("branch_id"));
        assert!(obj.contains_key("parent_tx"));
        assert!(obj.contains_key("verification_result_cid"));
        // Forbidden field guard: telemetry must NOT contain chain-of-thought
        // or raw deliberation per TB-6 charter §6 #11 inheritance.
        for forbidden in [
            "chain_of_thought",
            "model_deliberation",
            "tool_transcript",
            "raw_prompt",
            "raw_completion",
            "internal_reasoning",
        ] {
            assert!(
                !obj.contains_key(forbidden),
                "ProposalTelemetry has forbidden field {forbidden}"
            );
        }
    }

    /// U-A1.5.e — token_counts.total() arithmetic correctness; this feeds
    /// `golden_path_token_count` aggregation in Atom 5.
    #[test]
    fn token_counts_total() {
        let tc = TokenCounts {
            prompt_tokens: 13,
            completion_tokens: 19,
            tool_tokens: 5,
        };
        assert_eq!(tc.total(), 37);
    }

    /// TB-7.7 D1 — proposal_artifact_cid produced by build_for_evaluator_append
    /// must resolve to the original payload bytes via cas_store.get(cid).
    /// Pre-TB-7.7 the CID was a hash without storage; this is the structural
    /// witness that bytes are now durably stored.
    #[test]
    fn build_for_evaluator_append_stores_payload_in_cas() {
        let (_dir, mut cas) = fresh_cas();
        let payload_bytes = b"calc\n  f 1 = 5 * 1 + 4 := by rw [h_]\n  _ = 5 + 4 := by ring\n  _ = 9 := by norm_num";
        let pt = ProposalTelemetry::build_for_evaluator_append(
            &mut cas,
            "tb7-7-d1-test",
            "Agent_0",
            42,
            payload_bytes,
            "complete",
            TokenCounts {
                prompt_tokens: 100,
                completion_tokens: 50,
                tool_tokens: 0,
            },
            "tb7-7-d1-creator",
            7,
        )
        .expect("build with cas");
        // The CID in the telemetry must resolve in CAS to the original bytes.
        let recovered = cas.get(&pt.proposal_artifact_cid).expect("cas get");
        assert_eq!(recovered, payload_bytes);
    }

    /// TB-7.7 D1 — same payload bytes always yield the same CID
    /// (content-addressing determinism). This is what makes the chain
    /// byte-stable across reruns.
    #[test]
    fn build_for_evaluator_append_cid_is_deterministic_per_payload() {
        let (_dir, mut cas) = fresh_cas();
        let payload = b"by ring";
        let pt1 = ProposalTelemetry::build_for_evaluator_append(
            &mut cas, "r", "a", 1, payload, "ring", TokenCounts::default(),
            "creator", 1,
        )
        .expect("p1");
        let pt2 = ProposalTelemetry::build_for_evaluator_append(
            &mut cas, "r", "a", 1, payload, "ring", TokenCounts::default(),
            "creator", 1,
        )
        .expect("p2");
        assert_eq!(pt1.proposal_artifact_cid, pt2.proposal_artifact_cid);
    }

    /// TB-7.7 D2 — build_for_evaluator_append_with_parent records parent_tx
    /// when supplied; default builder leaves parent_tx = None.
    #[test]
    fn build_with_parent_records_parent_tx() {
        let (_dir, mut cas) = fresh_cas();
        let payload = b"by rfl";
        let parent = TxId("worktx-task-r-p0".into());
        let pt = ProposalTelemetry::build_for_evaluator_append_with_parent(
            &mut cas, "r", "a", 1, payload, "rfl", TokenCounts::default(),
            "creator", 1,
            Some(parent.clone()),
        )
        .expect("with parent");
        assert_eq!(pt.parent_tx, Some(parent));

        // Default builder yields None.
        let pt2 = ProposalTelemetry::build_for_evaluator_append(
            &mut cas, "r", "a", 2, payload, "rfl", TokenCounts::default(),
            "creator", 1,
        )
        .expect("without parent");
        assert!(pt2.parent_tx.is_none());
    }
}
