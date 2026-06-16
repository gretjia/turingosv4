//! TRACE_MATRIX FC3-N33 + FC1-N12: runtime **canary** evaluator — the
//! SAFETY-EVALUATION half of the FC3 self-evolution loop.
//!
//! ── SCOPE (token APPROVE-FC3-OBSERVABLE-CANARY-HALF) ─────────────────────
//! Given a candidate artifact (the `proposed_artifact_cid` carried by a
//! `fc3_proposer` proposal), the canary scores it against the EXISTING
//! executable `Predicate` / Lean oracle (`top_white::predicates::registry`):
//! it builds a `PredicateContext` whose proposal points at the candidate
//! bytes, runs `Predicate::evaluate` + `Predicate::verify_proof`, and converts
//! the pass/fail verdict into an integer-only `MetricEstimate` that is written
//! to CAS as system-only tape evidence (a `MetricEstimateCapsule`).
//!
//! The brief named `src/sdk/sandbox.rs`; that module does not exist. The real
//! candidate-execution surface is the `Predicate` trait (whose
//! `LeanArtifactPredicate` shells out to the Lean checker). The canary drives
//! that trait — the actual ground-truth evaluation mechanism — rather than a
//! nonexistent sandbox runner.
//!
//! ── HARD NON-GOALS (the FORBIDDEN irreversible leg — NO token) ────────────
//! The canary NEVER:
//!   * rewrites the boot Trust Root manifest,
//!   * re-inits the process (`ReinitRequest` / `ReinitBoot`),
//!   * gates any commit (no runtime {PASS,VETO} commit authority),
//!   * makes the candidate LIVE.
//! Its only output is a tape-anchored `MetricEstimate`. The terminal
//! disposition of even a *passing* candidate is `CANARY_ONLY_TERMINAL_STATUS`
//! (`"sandbox:canary_only"`) — the loop does NOT close. `closes_fc3_loop`
//! exists so the observable-half gate can assert that non-closure explicitly.

use serde::{Deserialize, Serialize};

use crate::bottom_white::cas::schema::{Cid, ObjectType};
use crate::bottom_white::cas::store::CasStore;
use crate::bottom_white::ledger::transition_ledger::canonical_encode;
use crate::economy::money::StakeMicroCoin;
// `MetricEstimate` now lives in the PARENT module (`real5_roles`); this file is
// a nested submodule of it, so reach it via `super` rather than the old
// absolute `crate::runtime::real5_roles::MetricEstimate` path.
use super::MetricEstimate;
use crate::state::q_state::{AgentId, Hash, TaskId, TxId};
use crate::state::typed_tx::{BoolWithProof, PredicateId};
use crate::top_white::predicates::registry::{
    PredicateContext, PredicateRegistry, PredicateVerifyError, PredicateWorkView,
};

/// TRACE_MATRIX FC3-N33: CAS schema id for the canary's tape-anchored metric
/// evidence capsule.
pub const FC3_METRIC_ESTIMATE_SCHEMA_ID: &str = "fc3.metric_estimate.v1";

/// TRACE_MATRIX FC3-N33: the terminal disposition of a canary-scored candidate.
/// Even a PASSING candidate terminates here — the canary is observe-only; it
/// does NOT close the FC3 loop. This is the same dead-end the standing-pending
/// G5 gate (`tests/pending/constitution_fc3_meta_loop_closure.rs`) asserts must
/// remain until the Class-4 irreversible-commit leg is §8-ratified.
pub const CANARY_ONLY_TERMINAL_STATUS: &str = "sandbox:canary_only";

/// TRACE_MATRIX FC3-N33: true iff a terminal status CLOSES the FC3 loop (a
/// tape-visible re-init / irreversible meta-commit). The canary's terminal
/// status must NOT close the loop. Mirrors the probe in the G5 pending gate so
/// the observable-half gate can assert non-closure with the same predicate.
pub fn closes_fc3_loop(status: &str) -> bool {
    status.contains("reinit") || status.contains("committed") || status.contains("re-init")
}

/// TRACE_MATRIX FC3-N33: a request to canary-score a candidate against one
/// registered predicate.
#[derive(Debug, Clone)]
pub struct CanaryRequest {
    /// The candidate artifact CID (the proposal's `proposed_artifact_cid`).
    pub candidate_artifact_cid: Cid,
    /// The predicate to score the candidate against (must be registered).
    pub predicate_id: PredicateId,
    /// The metric label recorded on the `MetricEstimate`.
    pub metric_label: String,
}

/// TRACE_MATRIX FC3-N33: integer-only tape-anchored canary metric evidence. No
/// `f64` anywhere (constitution money/metric integer rule). The `MetricEstimate`
/// is the existing `real5_roles` type; this capsule binds it to the candidate +
/// predicate + the canary terminal status.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MetricEstimateCapsule {
    pub schema_version: String,
    pub candidate_artifact_cid: Cid,
    pub predicate_id: PredicateId,
    /// The executable predicate's code hash at evaluation time (binds the
    /// metric to a specific predicate implementation).
    pub predicate_code_hash: [u8; 32],
    pub registry_root: Hash,
    /// The predicate verdict the metric was derived from.
    pub predicate_passed: bool,
    /// The integer-only metric estimate (numerator_delta / denominator).
    pub metric: MetricEstimate,
    /// Terminal disposition — always `CANARY_ONLY_TERMINAL_STATUS`. Recorded so
    /// an auditor can verify on tape that the loop did NOT close.
    pub terminal_status: String,
}

/// TRACE_MATRIX FC3-N33: the result of a canary run (metric + where it landed).
#[derive(Debug, Clone)]
pub struct CanaryOutcome {
    /// The evidence capsule written to CAS.
    pub capsule: MetricEstimateCapsule,
    /// CAS cid of the written `MetricEstimateCapsule`.
    pub metric_capsule_cid: Cid,
    /// Convenience copy of the underlying metric.
    pub metric: MetricEstimate,
}

impl CanaryOutcome {
    /// TRACE_MATRIX FC3-N33: structural guarantee that the canary did NOT close
    /// the FC3 loop. Always true for a canary outcome by construction; exposed
    /// so callers/gates can assert it.
    pub fn loop_stays_open(&self) -> bool {
        self.capsule.terminal_status == CANARY_ONLY_TERMINAL_STATUS
            && !closes_fc3_loop(&self.capsule.terminal_status)
    }
}

/// TRACE_MATRIX FC3-N33: error surface for the canary. Deterministic; bounded
/// labels only (no raw Lean stderr / autopsy leak into agent-readable text).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CanaryError {
    /// The requested predicate is not registered.
    PredicateNotRegistered(String),
    /// The predicate's `verify_proof` errored (class label only — no raw bytes).
    PredicateVerify(String),
    /// CAS read/write failed.
    Cas(String),
    /// Canonical encode failed.
    Codec(String),
}

impl std::fmt::Display for CanaryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PredicateNotRegistered(id) => {
                write!(f, "fc3 canary: predicate not registered: {id}")
            }
            Self::PredicateVerify(class) => {
                write!(f, "fc3 canary: predicate verify failed: {class}")
            }
            Self::Cas(e) => write!(f, "fc3 canary cas error: {e}"),
            Self::Codec(e) => write!(f, "fc3 canary codec error: {e}"),
        }
    }
}

impl std::error::Error for CanaryError {}

/// TRACE_MATRIX FC1-N12: bounded, raw-diagnostic-shielded class label for a
/// predicate verify error. Never surfaces the inner Lean stderr / counterexample
/// bytes — only the enum-variant family.
fn verify_error_class(err: &PredicateVerifyError) -> &'static str {
    match err {
        PredicateVerifyError::MissingProofCid => "missing_proof_cid",
        PredicateVerifyError::Cas(_) => "cas",
        PredicateVerifyError::ProofObjectType { .. } => "proof_object_type",
        PredicateVerifyError::ProofSchema { .. } => "proof_schema",
        PredicateVerifyError::Decode(_) => "decode",
        PredicateVerifyError::PredicateIdMismatch { .. } => "predicate_id_mismatch",
        PredicateVerifyError::RegistryRootMismatch { .. } => "registry_root_mismatch",
        PredicateVerifyError::CodeHashMismatch { .. } => "code_hash_mismatch",
        PredicateVerifyError::ProposalCidMismatch { .. } => "proposal_cid_mismatch",
        PredicateVerifyError::ClaimValueMismatch { .. } => "claim_value_mismatch",
        PredicateVerifyError::ContextHashMismatch { .. } => "context_hash_mismatch",
        PredicateVerifyError::ProofKindMismatch { .. } => "proof_kind_mismatch",
        PredicateVerifyError::ExpectedStatementHashMismatch { .. } => {
            "expected_statement_hash_mismatch"
        }
        PredicateVerifyError::ProofArtifactHashMismatch => "proof_artifact_hash_mismatch",
        PredicateVerifyError::ProposalPayloadMissing => "proposal_payload_missing",
        PredicateVerifyError::ProposalPayloadDecode(_) => "proposal_payload_decode",
        PredicateVerifyError::ForbiddenPattern(_) => "forbidden_pattern",
        PredicateVerifyError::PayloadTooLarge { .. } => "payload_too_large",
        PredicateVerifyError::PayloadTooManyLines { .. } => "payload_too_many_lines",
        PredicateVerifyError::ExternalCheckerFailed(_) => "lean_checker_failed",
        PredicateVerifyError::ExternalCheckerUnavailable(_) => "lean_checker_unavailable",
    }
}

/// TRACE_MATRIX FC1-N12: build the `PredicateWorkView` the canary uses to point
/// a predicate at the candidate artifact. The candidate artifact CID is the
/// `proposal_cid`, so `proposal_payload_bytes(ctx)` resolves to the candidate
/// bytes.
fn candidate_work_view(candidate_artifact_cid: Cid) -> PredicateWorkView {
    PredicateWorkView {
        tx_id: TxId("fc3-canary-candidate".to_string()),
        task_id: TaskId("fc3-canary".to_string()),
        parent_state_root: Hash([0u8; 32]),
        agent_id: AgentId("system".to_string()),
        read_set: Default::default(),
        write_set: Default::default(),
        proposal_cid: candidate_artifact_cid,
        stake: StakeMicroCoin::from_micro_units(0),
    }
}

/// TRACE_MATRIX FC3-N33: convert a predicate verdict into the integer-only
/// `MetricEstimate`. A PASS contributes `+1` expected-error-reduction over a
/// denominator of 1; a FAIL contributes `0`. Integer math only.
fn metric_for_verdict(metric_label: &str, passed: bool) -> MetricEstimate {
    MetricEstimate {
        metric: metric_label.to_string(),
        numerator_delta: if passed { 1 } else { 0 },
        denominator: 1,
    }
}

/// TRACE_MATRIX FC3-N33 + FC1-N12: run the canary. Scores the candidate against
/// the requested registered predicate via the EXISTING `Predicate` trait,
/// derives an integer `MetricEstimate`, and writes a `MetricEstimateCapsule` to
/// CAS as system-only tape evidence. The terminal status is always
/// `CANARY_ONLY_TERMINAL_STATUS` — the loop does NOT close.
pub fn run_canary(
    cas: &mut CasStore,
    registry: &PredicateRegistry,
    registry_root: Hash,
    request: &CanaryRequest,
    logical_t: u64,
) -> Result<CanaryOutcome, CanaryError> {
    let entry = registry
        .entry(&request.predicate_id)
        .ok_or_else(|| CanaryError::PredicateNotRegistered(request.predicate_id.0.clone()))?;
    let predicate = entry.impl_arc.clone();
    let predicate_code_hash = predicate.code_hash();

    // Build the predicate context pointing at the candidate artifact. The CAS
    // store itself is the proof store (`impl PredicateCasView for CasStore`).
    let work = candidate_work_view(request.candidate_artifact_cid);
    let predicate_passed = {
        let ctx = PredicateContext {
            registry_root,
            work,
            proof_store: &*cas,
        };
        // Evaluate first (records the candidate's claim), then verify it via the
        // ground-truth path. A verify error means the candidate did not satisfy
        // the predicate — recorded as a non-passing metric, NOT a hard abort,
        // so the failure is still observable on tape.
        let claim: BoolWithProof = predicate.evaluate(&ctx);
        match predicate.verify_proof(&ctx, &claim) {
            Ok(value) => value,
            Err(err) => {
                // Bounded class label only; the candidate simply fails canary.
                let _class = verify_error_class(&err);
                false
            }
        }
    };

    let metric = metric_for_verdict(&request.metric_label, predicate_passed);

    let capsule = MetricEstimateCapsule {
        schema_version: FC3_METRIC_ESTIMATE_SCHEMA_ID.to_string(),
        candidate_artifact_cid: request.candidate_artifact_cid,
        predicate_id: request.predicate_id.clone(),
        predicate_code_hash,
        registry_root,
        predicate_passed,
        metric: metric.clone(),
        terminal_status: CANARY_ONLY_TERMINAL_STATUS.to_string(),
    };

    let bytes = canonical_encode(&capsule).map_err(|e| CanaryError::Codec(e.to_string()))?;
    let metric_capsule_cid = cas
        .put(
            &bytes,
            ObjectType::Generic,
            "fc3_canary",
            logical_t,
            Some(FC3_METRIC_ESTIMATE_SCHEMA_ID.to_string()),
        )
        .map_err(|e| CanaryError::Cas(e.to_string()))?;

    Ok(CanaryOutcome {
        capsule,
        metric_capsule_cid,
        metric,
    })
}
