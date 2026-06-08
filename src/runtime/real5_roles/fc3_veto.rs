//! TRACE_MATRIX FC3-N32/N43: runtime **Veto-AI** deterministic constitutionality
//! clause-walker — the IRREVERSIBLE-LEG gate (A-ALLOW-1).
//!
//! ── SCOPE (token APPROVE-FC3-RUNTIME-VETO-AND-TRUSTROOT-REINIT) ───────────
//! This module is the runtime Veto-AI `{PASS,VETO}` gate that decides whether a
//! synthesized ArchitectAI candidate proposal may proceed to an
//! `ArchitectCommit` + trust-root recompute + re-init (`fc3_commit_reinit`).
//! It walks a FIXED, enumerated set of constitutionality checks over the
//! candidate's on-chain `ArchitectProposalCapsule` and returns exactly one of
//! `VetoVerdict::{Accept, Reject}` — the EXISTING two-valued runtime verdict
//! enum (`super::VetoVerdict`, the same one `proposal_activation_status`
//! consumes and the G5 gate probes). There is NO score, ranking, confidence, or
//! any third value.
//!
//! ── CONSTITUTIONAL GUARDS (binding) ──────────────────────────────────────
//!   * G-GUARD-1: deterministic + whitelisted to CONSTITUTIONALITY ONLY. Each
//!     clause below is a constitutional admissibility check (Art. V.1.1 /
//!     V.1.2 / V.1.3). NO subjective quality / performance / coverage /
//!     architecture-preference judgment enters the walker — those belong to the
//!     independent clean-context auditor, never to the runtime gate. Replay-
//!     stable: same proposal capsule -> same verdict, no probabilistic model.
//!   * G-GUARD-2: `constitution.md` is OUT of the rewrite range (Art. V.1.1).
//!     A candidate whose `target_path` (or any touched path) is `constitution.md`
//!     is REJECTED here — constitution amendment requires human sudo, never an
//!     auto-commit.
//!   * G-GUARD-5: FAIL-CLOSED. Any ambiguity, missing evidence, mismatched
//!     constitution hash, out-of-range target, or unknown proposal kind ->
//!     `Reject`. The default disposition of an unrecognized condition is VETO,
//!     never PASS (admission fail-closed default).
//!
//! ── HARD NON-GOALS ───────────────────────────────────────────────────────
//! This module NEVER mutates state, never writes CAS, never drives the
//! sequencer, never touches the trust root, and never re-inits. It is a PURE
//! decision function. The PASS-gated commit/recompute/re-init lives in the
//! sibling `fc3_commit_reinit` module and only runs on `VetoVerdict::Accept`.

use crate::bottom_white::ledger::transition_ledger::constitution_source_hash;
use crate::state::q_state::Hash;
use crate::state::typed_tx::{ArchitectProposalCapsule, ArchitectProposalKind};

use super::VetoVerdict;

/// TRACE_MATRIX FC3-N32: the constitution path that is ALWAYS out of the
/// ArchitectAI rewrite range (Art. V.1.1 — human sudo only). A candidate that
/// names this path in `target_path` is rejected by the walker.
pub const CONSTITUTION_OUT_OF_RANGE_PATH: &str = "constitution.md";

/// TRACE_MATRIX FC3-N32: a single deterministic constitutionality clause the
/// Veto-AI walks. The enumeration is FIXED and whitelisted to constitutionality;
/// no quality / performance / coverage clause may be added here. Each variant is
/// a constitutional admissibility check, not a taste judgment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VetoClause {
    /// Art. V.1.1: the candidate must NOT propose mutating `constitution.md`.
    ConstitutionNotMutated,
    /// Art. V.1.1: the candidate's bound constitution hash must equal the live
    /// `constitution_source_hash()` (the candidate reasoned over the current
    /// axiom text, not a forged/stale constitution).
    ConstitutionHashMatchesAxiom,
    /// Art. V.1.2: the candidate must carry real, actionable evidence — a
    /// concrete touched path AND a referenced candidate artifact CID. An empty
    /// `Noop`/shell proposal carries no committable evidence.
    ProposalCarriesEvidence,
    /// Art. V.1.2: the proposal kind must be within the ArchitectAI commit range
    /// (kernel / oracle / predicate-registry / tool-registry / storage payload).
    /// A `Noop` (no-op) or a `TrustRootManifestPatch` that itself rewrites the
    /// manifest authority is NOT auto-committable through this runtime gate.
    ProposalKindInCommitRange,
}

impl VetoClause {
    /// TRACE_MATRIX FC3-N32: the fixed, ordered clause set the Veto-AI walks.
    /// Ordering is deterministic so the first failing clause is replay-stable.
    pub const ALL: &'static [VetoClause] = &[
        VetoClause::ConstitutionNotMutated,
        VetoClause::ConstitutionHashMatchesAxiom,
        VetoClause::ProposalCarriesEvidence,
        VetoClause::ProposalKindInCommitRange,
    ];

    /// TRACE_MATRIX FC3-N32: stable label for tape/audit reason text.
    pub const fn label(self) -> &'static str {
        match self {
            VetoClause::ConstitutionNotMutated => "constitution_not_mutated",
            VetoClause::ConstitutionHashMatchesAxiom => "constitution_hash_matches_axiom",
            VetoClause::ProposalCarriesEvidence => "proposal_carries_evidence",
            VetoClause::ProposalKindInCommitRange => "proposal_kind_in_commit_range",
        }
    }
}

/// TRACE_MATRIX FC3-N32/N43: the deterministic Veto-AI verdict bundle. The
/// authority-bearing output is `verdict` (the two-valued `VetoVerdict`); the
/// `failed_clause` / `reason` fields are bounded audit text only — they carry no
/// score, ranking, or subjective grade.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VetoWalkOutcome {
    /// The two-valued verdict — the ONLY authority-bearing output.
    pub verdict: VetoVerdict,
    /// The first clause that failed (None iff every clause passed -> Accept).
    pub failed_clause: Option<VetoClause>,
    /// Bounded, deterministic reason label (no raw diagnostics).
    pub reason: String,
}

impl VetoWalkOutcome {
    fn accept() -> Self {
        Self {
            verdict: VetoVerdict::Accept,
            failed_clause: None,
            reason: "all constitutionality clauses passed".to_string(),
        }
    }

    fn reject(clause: VetoClause, reason: impl Into<String>) -> Self {
        Self {
            verdict: VetoVerdict::Reject,
            failed_clause: Some(clause),
            reason: reason.into(),
        }
    }

    /// TRACE_MATRIX FC3-N32: true iff the verdict is a PASS that permits the
    /// downstream `ArchitectCommit` + re-init leg to run.
    pub fn permits_commit(&self) -> bool {
        self.verdict == VetoVerdict::Accept
    }
}

/// TRACE_MATRIX FC3-N32: evaluate ONE constitutionality clause over the
/// candidate capsule. Returns `Ok(())` if the clause is satisfied, or
/// `Err(reason)` (fail-closed) if it is violated or ambiguous.
fn evaluate_clause(
    clause: VetoClause,
    capsule: &ArchitectProposalCapsule,
    axiom_hash: Hash,
) -> Result<(), String> {
    match clause {
        VetoClause::ConstitutionNotMutated => {
            // G-GUARD-2 / Art. V.1.1: refuse any candidate that names
            // constitution.md as a touched path — human sudo only.
            if capsule.target_path.as_deref() == Some(CONSTITUTION_OUT_OF_RANGE_PATH) {
                return Err(format!(
                    "constitution mutation forbidden: target_path == {CONSTITUTION_OUT_OF_RANGE_PATH} \
                     (Art. V.1.1 human sudo only)"
                ));
            }
            // Defensive: the summary or tools list must not smuggle a
            // constitution edit either. Fail-closed on any reference.
            if capsule
                .target_path
                .as_deref()
                .map(|p| p.trim() == CONSTITUTION_OUT_OF_RANGE_PATH)
                .unwrap_or(false)
            {
                return Err("constitution mutation forbidden (normalized path match)".to_string());
            }
            Ok(())
        }
        VetoClause::ConstitutionHashMatchesAxiom => {
            // Art. V.1.1: the candidate must have reasoned over the live axiom
            // text. A stale/forged constitution hash is fail-closed.
            if capsule.constitution_hash != axiom_hash {
                return Err(
                    "constitution hash mismatch: candidate did not bind the live axiom text"
                        .to_string(),
                );
            }
            Ok(())
        }
        VetoClause::ProposalCarriesEvidence => {
            // Art. V.1.2: a committable candidate must name a real touched path
            // AND reference a candidate artifact CID. Empty -> fail-closed.
            let has_path = capsule
                .target_path
                .as_deref()
                .map(|p| !p.trim().is_empty())
                .unwrap_or(false);
            let has_artifact = capsule.proposed_artifact_cid.is_some();
            if !has_path || !has_artifact {
                return Err(format!(
                    "insufficient evidence: has_path={has_path} has_artifact={has_artifact} \
                     (Art. V.1.2 needs a real touched path + artifact)"
                ));
            }
            Ok(())
        }
        VetoClause::ProposalKindInCommitRange => {
            // Art. V.1.2: only ArchitectAI-range payload kinds are auto-
            // committable. `Noop` carries no change; `TrustRootManifestPatch`
            // (kind 4) is the manifest-authority surface itself and is NOT
            // auto-committed by the runtime gate (own signed v4-ratify tag,
            // G-GUARD-4). Unknown future kinds are fail-closed by the explicit
            // match below.
            match capsule.proposal_kind {
                ArchitectProposalKind::ToolRegistryPatch
                | ArchitectProposalKind::PredicatePatch
                | ArchitectProposalKind::StoragePatch => Ok(()),
                ArchitectProposalKind::Noop => {
                    Err("proposal kind Noop is not committable (no change)".to_string())
                }
                ArchitectProposalKind::TrustRootManifestPatch => Err(
                    "proposal kind TrustRootManifestPatch needs its own signed v4-ratify tag \
                     (G-GUARD-4); not auto-committable by the runtime Veto gate"
                        .to_string(),
                ),
            }
        }
    }
}

/// TRACE_MATRIX FC3-N32/N43: the deterministic Veto-AI clause-walker. Walks the
/// FIXED clause set over the candidate's on-chain `ArchitectProposalCapsule` and
/// returns exactly one of `VetoVerdict::{Accept, Reject}`.
///
/// FAIL-CLOSED (G-GUARD-5): the first clause that fails (or is ambiguous) ends
/// the walk with `Reject`; `Accept` is returned ONLY if every clause in
/// `VetoClause::ALL` passes. The verdict is replay-stable — it depends solely on
/// the capsule fields and the live `constitution_source_hash()` (passed in for
/// testability), with no I/O, randomness, or probabilistic model.
///
/// This is whitelisted to constitutionality: every clause is a constitutional
/// admissibility check (Art. V.1.1 / V.1.2 / V.1.3). No quality / performance /
/// coverage / style judgment is consulted.
pub fn veto_walk(capsule: &ArchitectProposalCapsule, axiom_hash: Hash) -> VetoWalkOutcome {
    for &clause in VetoClause::ALL {
        if let Err(reason) = evaluate_clause(clause, capsule, axiom_hash) {
            return VetoWalkOutcome::reject(clause, format!("{}: {reason}", clause.label()));
        }
    }
    VetoWalkOutcome::accept()
}

/// TRACE_MATRIX FC3-N32: convenience wrapper that binds the live axiom hash
/// (`constitution_source_hash()`) so production callers cannot accidentally pass
/// a stale or forged constitution hash. Tests may call `veto_walk` directly with
/// an explicit hash to exercise the mismatch clause.
pub fn veto_walk_live(capsule: &ArchitectProposalCapsule) -> VetoWalkOutcome {
    veto_walk(capsule, constitution_source_hash())
}
