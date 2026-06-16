//! TRACE_MATRIX FC3-N33 + FC3-N41: runtime ArchitectAI **proposer** — the
//! OBSERVABLE half of the FC3 self-evolution loop.
//!
//! ── SCOPE (token APPROVE-FC3-OBSERVABLE-CANARY-HALF) ─────────────────────
//! This module reads an accepted `LogFeedbackArchive` row (FC3-N41 input edge),
//! clusters the runtime failure signal (feedback summary + L4.E rejection
//! records), synthesizes a REAL candidate spec, and emits an
//! `ArchitectProposalCapsule` that carries that spec — **not** the empty
//! `{proposal_id}` shell that the inert role path (`real5_roles.rs`
//! `ToolProposalPayload::default()`) produces. The proposal is anchored as
//! system-only tape evidence through the EXISTING
//! `SystemEmitCommand::ArchitectProposal` path (no new typed-tx variant, no
//! schema change).
//!
//! ── HARD NON-GOALS (the FORBIDDEN irreversible leg — NO token) ────────────
//! This module NEVER:
//!   * emits a `VetoDecision` / `ArchitectCommit` (no runtime {PASS,VETO}
//!     commit gating),
//!   * rewrites the boot Trust Root manifest,
//!   * requests / activates a re-init (`ReinitRequest` / `ReinitBoot`),
//!   * makes any candidate LIVE.
//! It only PRODUCES a tape-anchored proposal. Nothing it emits changes the
//! trust root or activates code. The terminal disposition of an accepted
//! proposal remains `sandbox:canary_only` (see `fc3_canary`) — the loop does
//! NOT close here, by design. Closing it is a Class-4 §8 surface tracked by the
//! standing-pending gate `tests/pending/constitution_fc3_meta_loop_closure.rs`.

use serde::{Deserialize, Serialize};

use crate::bottom_white::cas::schema::{Cid, ObjectType};
use crate::bottom_white::cas::store::CasStore;
use crate::bottom_white::ledger::rejection_evidence::{RejectionClass, RejectionEvidenceWriter};
use crate::bottom_white::ledger::transition_ledger::{
    canonical_decode, canonical_encode, constitution_source_hash,
};
use crate::state::q_state::Hash;
use crate::state::sequencer::SystemEmitCommand;
use crate::state::typed_tx::{
    ArchitectFeedbackCapsule, ArchitectProposalCapsule, ArchitectProposalKind,
    LogFeedbackArchiveTx, ARCHITECT_FEEDBACK_SCHEMA_ID, ARCHITECT_PROPOSAL_SCHEMA_ID,
};

/// TRACE_MATRIX FC3-N33: CAS schema id for the runtime candidate artifact bytes
/// referenced by a synthesized proposal capsule (`proposed_artifact_cid`).
pub const FC3_CANDIDATE_ARTIFACT_SCHEMA_ID: &str = "fc3.candidate_artifact.v1";

/// TRACE_MATRIX FC3-N33: CAS schema id for the structured candidate-spec sidecar
/// that the proposer derives and binds into the proposal's `public_summary`.
/// Stored alongside the proposal so an auditor can reconstruct the full spec
/// (the on-chain capsule fields plus the derived edges/gates/rollback plan)
/// from CAS without re-running the proposer.
pub const FC3_CANDIDATE_SPEC_SCHEMA_ID: &str = "fc3.candidate_spec.v1";

/// TRACE_MATRIX FC3-N33: deterministic failure cluster derived from the FC3
/// feedback summary plus the live L4.E rejection records. Integer-only counts
/// (no `f64` in any derived signal) so the clustering is replay-stable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct FailureCluster {
    /// Total L4.E rejection records observed at proposal time.
    pub total_rejections: u64,
    /// Count of `LeanFailed` rejections (proof tactic failure).
    pub lean_failed: u64,
    /// Count of `ParseFailed` rejections (unparseable candidate).
    pub parse_failed: u64,
    /// Count of `SorryBlocked` rejections (forbidden incomplete-proof token).
    pub sorry_blocked: u64,
    /// Count of `LlmError` rejections (provider-side error).
    pub llm_error: u64,
    /// Count of acceptance-predicate failures (`PredicateFailed`).
    pub predicate_failed: u64,
    /// The dominant rejection class label (deterministic argmax over the
    /// integer counts above; ties resolve by the fixed enumeration order).
    pub dominant_class: String,
}

impl FailureCluster {
    /// TRACE_MATRIX FC3-N33: cluster the live L4.E rejection records into the
    /// integer histogram the proposer reasons over. Pure over the writer's
    /// in-memory record set; no side effects.
    pub fn from_rejections(rejections: &RejectionEvidenceWriter) -> Self {
        let mut cluster = FailureCluster::default();
        for rec in rejections.records() {
            cluster.total_rejections += 1;
            match rec.rejection_class {
                RejectionClass::CheckerFailed => cluster.lean_failed += 1,
                RejectionClass::ParseFailed => cluster.parse_failed += 1,
                RejectionClass::IncompleteProofBlocked => cluster.sorry_blocked += 1,
                RejectionClass::LlmError => cluster.llm_error += 1,
                RejectionClass::PredicateFailed => cluster.predicate_failed += 1,
                _ => {}
            }
        }
        cluster.dominant_class = cluster.compute_dominant_class().to_string();
        cluster
    }

    fn compute_dominant_class(&self) -> &'static str {
        // Deterministic argmax with a fixed tie-break order. Empty histogram →
        // `"none"` (legal: a Noop-class proposal is the correct outcome when the
        // logs contain no actionable failure signal).
        let ranked: [(&'static str, u64); 5] = [
            ("lean_failed", self.lean_failed),
            ("parse_failed", self.parse_failed),
            ("sorry_blocked", self.sorry_blocked),
            ("llm_error", self.llm_error),
            ("predicate_failed", self.predicate_failed),
        ];
        let mut best: (&'static str, u64) = ("none", 0);
        for (label, count) in ranked {
            if count > best.1 {
                best = (label, count);
            }
        }
        best.0
    }
}

/// TRACE_MATRIX FC3-N33: the REAL candidate spec the proposer synthesizes — the
/// payload the standing-pending G5 gate asserts the inert role path is MISSING.
/// This is the structured sidecar bound into CAS; its identity-bearing fields
/// also project onto the on-chain `ArchitectProposalCapsule` (kind / target_path
/// / proposed_artifact_cid / tools_used / public_summary).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CandidateSpec {
    pub schema_version: String,
    /// Parent feedback edge (FC3-N41) this candidate is derived from. The
    /// capsule CID(s) the proposer read as its input.
    pub parent_feedback_cids: Vec<Cid>,
    /// Source `LogFeedbackArchive` tx id this candidate is anchored to.
    pub feedback_tx_id: String,
    /// Candidate artifact CIDs (the proposed patch bytes in CAS).
    pub candidate_artifact_cids: Vec<Cid>,
    /// Source paths the candidate intends to touch.
    pub touched_paths: Vec<String>,
    /// FC edges the candidate intends to alter (e.g. `"FC1-N11"`).
    pub intended_fc_edges: Vec<String>,
    /// Architect-predicted risk class (0..4) for the candidate.
    pub predicted_risk_class: u8,
    /// Gates the candidate MUST pass before it could ever be considered live.
    pub required_gates: Vec<String>,
    /// Explicit rollback plan if the candidate is later reverted.
    pub rollback_plan: String,
    /// The failure cluster that motivated this candidate.
    pub failure_cluster: FailureCluster,
}

impl CandidateSpec {
    /// True iff this spec carries a concrete, actionable candidate (a real
    /// artifact + touched path + intended edge). A `Noop`-equivalent empty spec
    /// returns `false`. This is the structural witness the observable-half gate
    /// asserts is GREEN (vs the inert `ToolProposalPayload::default()` shell).
    /// TRACE_MATRIX FC3-N33: actionable-spec predicate (real candidate vs Noop shell).
    pub fn is_actionable(&self) -> bool {
        !self.candidate_artifact_cids.is_empty()
            && !self.touched_paths.is_empty()
            && !self.intended_fc_edges.is_empty()
    }
}

/// TRACE_MATRIX FC3-N33: error surface for the proposer. Deterministic; no raw
/// diagnostics leak (only structured, bounded labels).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProposerError {
    /// The referenced feedback capsule was missing or had the wrong schema.
    FeedbackCapsuleInvalid,
    /// CAS read/write failed.
    Cas(String),
    /// Canonical encode/decode failed.
    Codec(String),
    /// The synthesized spec was not actionable — refuse to emit an empty shell
    /// dressed up as a real proposal.
    SpecNotActionable,
}

impl std::fmt::Display for ProposerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FeedbackCapsuleInvalid => write!(f, "fc3 feedback capsule invalid"),
            Self::Cas(e) => write!(f, "fc3 proposer cas error: {e}"),
            Self::Codec(e) => write!(f, "fc3 proposer codec error: {e}"),
            Self::SpecNotActionable => write!(f, "fc3 candidate spec is not actionable"),
        }
    }
}

impl std::error::Error for ProposerError {}

/// TRACE_MATRIX FC3-N33: the synthesized-proposal bundle produced by the
/// proposer, ready to be emitted via `SystemEmitCommand::ArchitectProposal`.
#[derive(Debug, Clone)]
pub struct SynthesizedProposal {
    /// The structured candidate spec (also written to CAS).
    pub spec: CandidateSpec,
    /// CAS cid of the candidate artifact bytes.
    pub candidate_artifact_cid: Cid,
    /// CAS cid of the structured `CandidateSpec` sidecar.
    pub candidate_spec_cid: Cid,
    /// CAS cid of the canonical `ArchitectProposalCapsule` ready for emit.
    pub proposal_capsule_cid: Cid,
    /// The capsule itself (for caller inspection / replay).
    pub proposal_capsule: ArchitectProposalCapsule,
}

impl SynthesizedProposal {
    /// TRACE_MATRIX FC3-N33: the EXISTING emit command this proposal feeds.
    /// The proposer never constructs/signs the typed tx — `emit_system_tx`
    /// does, preserving the Anti-Oreo barrier.
    pub fn emit_command(&self, feedback_tx: &LogFeedbackArchiveTx) -> SystemEmitCommand {
        SystemEmitCommand::ArchitectProposal {
            feedback_tx_id: feedback_tx.tx_id.clone(),
            proposal_capsule_cid: self.proposal_capsule_cid,
        }
    }
}

/// TRACE_MATRIX FC3-N41: read an accepted feedback capsule from CAS and decode
/// it. The proposer's input edge.
pub fn read_feedback_capsule(
    cas: &CasStore,
    feedback_capsule_cid: &Cid,
) -> Result<ArchitectFeedbackCapsule, ProposerError> {
    let meta = cas
        .metadata(feedback_capsule_cid)
        .ok_or(ProposerError::FeedbackCapsuleInvalid)?;
    if meta.object_type != ObjectType::Generic
        || meta.schema_id.as_deref() != Some(ARCHITECT_FEEDBACK_SCHEMA_ID)
    {
        return Err(ProposerError::FeedbackCapsuleInvalid);
    }
    let bytes = cas
        .get(feedback_capsule_cid)
        .map_err(|e| ProposerError::Cas(e.to_string()))?;
    let capsule: ArchitectFeedbackCapsule =
        canonical_decode(&bytes).map_err(|e| ProposerError::Codec(e.to_string()))?;
    if capsule.schema_version != ARCHITECT_FEEDBACK_SCHEMA_ID {
        return Err(ProposerError::FeedbackCapsuleInvalid);
    }
    Ok(capsule)
}

/// TRACE_MATRIX FC3-N33: map a failure cluster to the proposal kind the
/// candidate would patch. Predicate-dominated clusters → `PredicatePatch`;
/// everything else with signal → `ToolRegistryPatch`; empty signal → `Noop`.
/// `Noop` candidates are deliberately NOT actionable and will be refused by
/// `synthesize_proposal` (no empty shell dressed as a real proposal).
fn proposal_kind_for(cluster: &FailureCluster) -> ArchitectProposalKind {
    if cluster.total_rejections == 0 {
        return ArchitectProposalKind::Noop;
    }
    if cluster.predicate_failed >= cluster.lean_failed
        && cluster.predicate_failed > 0
        && cluster.dominant_class == "predicate_failed"
    {
        return ArchitectProposalKind::PredicatePatch;
    }
    ArchitectProposalKind::ToolRegistryPatch
}

/// TRACE_MATRIX FC3-N33: synthesize a candidate spec from the clustered failure
/// signal. The spec is REAL: it names an artifact, a touched path, the intended
/// FC edges, a predicted risk class, the gates that must pass, and a rollback
/// plan. This is the body of the proposal — not the `{proposal_id}` shell.
pub fn synthesize_candidate_spec(
    feedback_tx: &LogFeedbackArchiveTx,
    feedback_capsule_cid: Cid,
    candidate_artifact_cid: Cid,
    cluster: FailureCluster,
) -> CandidateSpec {
    let kind = proposal_kind_for(&cluster);
    let (touched_path, intended_edge, risk_class) = match kind {
        ArchitectProposalKind::PredicatePatch => (
            "src/top_white/predicates/registry.rs".to_string(),
            "FC1-N12".to_string(),
            2u8,
        ),
        ArchitectProposalKind::Noop => (
            // Noop spec carries no path/edge — left empty so `is_actionable()`
            // returns false and the proposer refuses to emit it.
            String::new(),
            String::new(),
            0u8,
        ),
        _ => (
            "src/bottom_white/tools/runtime_meta_tool.rs".to_string(),
            "FC1-N7".to_string(),
            2u8,
        ),
    };

    let touched_paths = if touched_path.is_empty() {
        Vec::new()
    } else {
        vec![touched_path]
    };
    let intended_fc_edges = if intended_edge.is_empty() {
        Vec::new()
    } else {
        vec![intended_edge]
    };

    CandidateSpec {
        schema_version: FC3_CANDIDATE_SPEC_SCHEMA_ID.to_string(),
        parent_feedback_cids: vec![feedback_capsule_cid],
        feedback_tx_id: feedback_tx.tx_id.0.clone(),
        candidate_artifact_cids: vec![candidate_artifact_cid],
        touched_paths,
        intended_fc_edges,
        predicted_risk_class: risk_class,
        required_gates: vec![
            "cargo test --workspace --no-fail-fast".to_string(),
            "bash scripts/run_constitution_gates.sh".to_string(),
            "cargo test --test constitution_matrix_drift".to_string(),
        ],
        rollback_plan: format!(
            "revert candidate artifact (dominant failure class: {}); no trust-root \
             change was ever applied, so rollback is a no-op on the live boot manifest",
            cluster.dominant_class
        ),
        failure_cluster: cluster,
    }
}

/// TRACE_MATRIX FC3-N33: project a `CandidateSpec` onto the on-chain
/// `ArchitectProposalCapsule`. The capsule's identity fields (feedback linkage,
/// constitution hash, tool-registry root) must match the sequencer's current Q
/// for `emit_system_tx` to accept it — the caller passes those in.
pub fn build_proposal_capsule(
    feedback_tx: &LogFeedbackArchiveTx,
    tool_registry_root: Hash,
    spec: &CandidateSpec,
) -> ArchitectProposalCapsule {
    let cluster = &spec.failure_cluster;
    let public_summary = format!(
        "FC3 runtime ArchitectAI candidate (dominant={}, rejections={}): \
         touches {:?}; intended_fc_edges={:?}; predicted_risk_class={}; \
         required_gates={:?}; rollback_plan={}",
        cluster.dominant_class,
        cluster.total_rejections,
        spec.touched_paths,
        spec.intended_fc_edges,
        spec.predicted_risk_class,
        spec.required_gates,
        spec.rollback_plan,
    );
    ArchitectProposalCapsule {
        schema_version: ARCHITECT_PROPOSAL_SCHEMA_ID.to_string(),
        feedback_tx_id: feedback_tx.tx_id.clone(),
        feedback_root: feedback_tx.feedback_root,
        constitution_hash: constitution_source_hash(),
        tool_registry_root,
        proposal_kind: proposal_kind_for(cluster),
        target_path: spec.touched_paths.first().cloned(),
        proposed_artifact_cid: spec.candidate_artifact_cids.first().copied(),
        tools_used: vec![
            "fc3_proposer::read_feedback_capsule".to_string(),
            "fc3_proposer::synthesize_candidate_spec".to_string(),
        ],
        public_summary,
    }
}

/// TRACE_MATRIX FC3-N33: the full proposer engine. Given an accepted feedback
/// row, the live L4.E rejection records, the current tool-registry root, and the
/// candidate artifact bytes, this:
///   1. reads the feedback capsule (input edge),
///   2. clusters the failure signal,
///   3. synthesizes a REAL candidate spec,
///   4. writes the artifact + spec + proposal capsule to CAS,
///   5. returns the `SynthesizedProposal` (ready for emit).
///
/// It does NOT emit (the caller drives `emit_system_tx` with
/// `SynthesizedProposal::emit_command`), and it NEVER commits, vetoes,
/// re-inits, or touches the trust root.
#[allow(clippy::too_many_arguments)]
pub fn synthesize_proposal(
    cas: &mut CasStore,
    feedback_tx: &LogFeedbackArchiveTx,
    feedback_capsule_cid: Cid,
    rejections: &RejectionEvidenceWriter,
    tool_registry_root: Hash,
    candidate_artifact_bytes: &[u8],
    logical_t: u64,
) -> Result<SynthesizedProposal, ProposerError> {
    // 1. input edge: confirm the feedback capsule is real + well-formed.
    let _feedback = read_feedback_capsule(cas, &feedback_capsule_cid)?;

    // 2. cluster the live failure signal.
    let cluster = FailureCluster::from_rejections(rejections);

    // 3. write the candidate artifact bytes to CAS.
    let candidate_artifact_cid = cas
        .put(
            candidate_artifact_bytes,
            ObjectType::Generic,
            "fc3_proposer",
            logical_t,
            Some(FC3_CANDIDATE_ARTIFACT_SCHEMA_ID.to_string()),
        )
        .map_err(|e| ProposerError::Cas(e.to_string()))?;

    // 4. synthesize the REAL spec and refuse the empty/Noop shell.
    let spec = synthesize_candidate_spec(
        feedback_tx,
        feedback_capsule_cid,
        candidate_artifact_cid,
        cluster,
    );
    if !spec.is_actionable() {
        return Err(ProposerError::SpecNotActionable);
    }

    // 5. anchor the structured spec sidecar in CAS.
    let spec_bytes = canonical_encode(&spec).map_err(|e| ProposerError::Codec(e.to_string()))?;
    let candidate_spec_cid = cas
        .put(
            &spec_bytes,
            ObjectType::Generic,
            "fc3_proposer",
            logical_t,
            Some(FC3_CANDIDATE_SPEC_SCHEMA_ID.to_string()),
        )
        .map_err(|e| ProposerError::Cas(e.to_string()))?;

    // 6. project onto the on-chain proposal capsule + anchor it in CAS.
    let proposal_capsule = build_proposal_capsule(feedback_tx, tool_registry_root, &spec);
    let proposal_bytes =
        canonical_encode(&proposal_capsule).map_err(|e| ProposerError::Codec(e.to_string()))?;
    let proposal_capsule_cid = cas
        .put(
            &proposal_bytes,
            ObjectType::Generic,
            "fc3_proposer",
            logical_t,
            Some(ARCHITECT_PROPOSAL_SCHEMA_ID.to_string()),
        )
        .map_err(|e| ProposerError::Cas(e.to_string()))?;

    Ok(SynthesizedProposal {
        spec,
        candidate_artifact_cid,
        candidate_spec_cid,
        proposal_capsule_cid,
        proposal_capsule,
    })
}
