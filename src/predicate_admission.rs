//! TRACE_MATRIX FC1a-predicates + FC1b-Q_{t+1}: shared predicate-admission
//! contract (M07). ONE oracle, two adapters — the sequencer `WorkTx` leg and
//! the kernel header leg both build a claim-level [`PredicateClaimSet`] and call
//! [`decide_admission`]. Route-A transitional: only the zero-root boolean branch
//! is reachable from the kernel today; the bound-root oracle path stays
//! sequencer-only (it needs `&PredicateRegistry` + `&dyn PredicateCasView` +
//! `PredicateWorkView::from_work_tx`, none of which the kernel can supply).
//!
//! The zero-root boolean scan moved here VERBATIM from
//! `src/state/sequencer.rs:1232-1242`: acceptance first, then settlement, fail
//! on the FIRST false value, in `BTreeMap` iteration order. The sequencer maps
//! the returned [`AdmissionVerdict::Fail`] back to its own `TransitionError`
//! variants at the boundary so no error type crosses the trust-root pin.
//!
//! On-disk design: handover/design/M07_SINGLE_ADMISSION_IMPLEMENTATION_SPEC_2026-06-07.md

use crate::bottom_white::cas::schema::Cid;
use crate::state::q_state::Hash;
use crate::state::typed_tx::PredicateId;

// arg-taint sub-article — value-level taint labelling + tainted-arg →
// privileged-sink detection. Nested here as a `#[path]` submodule of THIS
// UNPINNED parent (mirroring `src/runtime/real5_roles.rs`) so the taint findings
// reach the ONE admission oracle (`decide_admission`) with ZERO genesis-pinned
// file edits. Declaring it under the pinned `src/lib.rs` / predicate registry
// would force a Trust-Root pin rehash (out-of-scope Class-4).
/// TRACE_MATRIX FC1a-predicates: value-level argument taint module.
#[path = "predicate_admission/arg_taint.rs"]
pub mod arg_taint;

pub use arg_taint::{ArgTaintFinding, WtoolCall};

/// Stable prefix the arg-taint hard-gate stamps into the admission
/// `failed_predicate` field. A receipt whose failed-predicate name starts with
/// this prefix was rejected by the tainted-arg → privileged-sink gate (not an
/// ordinary acceptance predicate). The gate test + auditors branch on this.
/// TRACE_MATRIX FC1a-predicates: arg-taint rejection receipt marker.
pub const ARG_TAINT_FAILED_PREDICATE_PREFIX: &str = "arg_taint_v1";

/// Fold the arg-taint findings into the synthetic `failed_predicate` name carried
/// in the admission receipt. Format: `arg_taint_v1[<reason>;<reason>;...]`. The
/// reasons are the redact-safe per-finding strings (labels + sink identity, never
/// raw value bytes), so the gate is reconstructable from the rejection receipt.
/// TRACE_MATRIX FC1a-predicates: arg-taint rejection receipt encoding.
pub fn arg_taint_failed_predicate(findings: &[ArgTaintFinding]) -> String {
    let joined = findings
        .iter()
        .map(|f| f.reason())
        .collect::<Vec<_>>()
        .join(";");
    format!("{ARG_TAINT_FAILED_PREDICATE_PREFIX}[{joined}]")
}

/// Lowercase-hex of a 32-byte [`Hash`] — the canonical string form a receipt
/// embeds and `decide_admission` branches on. `Hash` exposes no hex method, so
/// this is the single conversion site.
/// TRACE_MATRIX FC1a-predicates: canonical hex form for the admission receipt.
pub fn hash_to_hex(h: &Hash) -> String {
    let mut s = String::with_capacity(64);
    for byte in h.0.iter() {
        s.push_str(&format!("{:02x}", byte));
    }
    s
}

/// The all-zero registry root rendered as hex — selects the legacy boolean
/// (verdict-trusting) branch.
/// TRACE_MATRIX FC1a-predicates: zero-root selector for the admission branch.
pub fn zero_root_hex() -> String {
    hash_to_hex(&Hash::ZERO)
}

/// One predicate claim, decoupled from the `WorkTx` wire layout. Both legs build
/// this from their own context. `proof_cid` is carried so the bound-root oracle
/// path can resolve it; the kernel leg always leaves it `None`.
/// TRACE_MATRIX FC1a-predicates: one predicate claim feeding Pi-p admission.
#[derive(Debug, Clone)]
pub struct PredicateClaim {
    pub id: PredicateId,
    pub value: bool,
    pub proof_cid: Option<Cid>,
}

/// The abstract claim set an admission decision is taken over.
///
/// NOTE (arg-taint sub-article): the taint findings are deliberately NOT a field
/// here. `PredicateClaimSet` is constructed with an all-fields-named struct
/// literal by the genesis-pinned `sequencer.rs::work_tx_to_claim_set` (a
/// Trust-Root-pinned file we must not edit / rehash); adding a field would break
/// that literal and force a pin rehash. The taint findings instead flow as a
/// SEPARATE argument into [`decide_admission_with_taint`], which the UNPINNED
/// memory-kernel leg calls. The pinned sequencer keeps calling the original
/// [`decide_admission`] unchanged.
/// TRACE_MATRIX FC1a-predicates: claim set both admission legs decide over.
#[derive(Debug, Clone, Default)]
pub struct PredicateClaimSet {
    pub acceptance: Vec<PredicateClaim>,
    pub settlement: Vec<PredicateClaim>,
}

/// Admission verdict. `Pass` carries the registry root the decision was taken
/// under (zero-hex for the legacy boolean branch); the receipt embeds this.
/// TRACE_MATRIX FC1a-predicates + FC1b-Q_{t+1}: admission verdict gating advance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdmissionVerdict {
    Pass { registry_root_hex: String },
    Fail {
        failed_predicate: String,
        reason: AdmissionFailReason,
    },
}

/// Why an admission decision failed. The sequencer maps these onto its existing
/// `TransitionError` variants at the boundary.
/// TRACE_MATRIX FC1a-predicates: admission-failure reason (no head advance).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdmissionFailReason {
    AcceptancePredicateFalse,
    SettlementPredicateFalse,
    /// Zero registry root supplied by an OS-qualified run (G3 path). Such a run
    /// must carry a NON-ZERO bound root so the oracle re-executes, not trust the
    /// self-reported booleans.
    ZeroRootRefusedForOsQualifiedRun,
    // Bound-root variants land when the kernel bound path is wired (out of
    // route-A scope; the bound oracle stays in sequencer.rs today).
    //
    // NOTE (arg-taint sub-article): the tainted-arg → privileged-sink hard-gate
    // does NOT add a new variant here. `AdmissionFailReason` is matched
    // EXHAUSTIVELY by the genesis-pinned `sequencer.rs::admission_fail_to_transition`
    // (a Trust-Root-pinned file we must not edit / rehash), so a new variant
    // would break that match and force a pin rehash. Instead the taint reject
    // reuses `AcceptancePredicateFalse` with `failed_predicate == "arg_taint_v1"`
    // and the per-finding detail folded into that predicate-name string — see
    // `decide_admission`. This keeps the pinned exhaustive match valid with ZERO
    // pinned edits while still failing admission (no head advance).
}

/// THE single admission contract. Pure, deterministic, no I/O on the zero-root
/// path. `registry_root_hex` selects the branch:
///   * zero-hex  → legacy boolean branch (verdict-trusting) UNLESS
///                 `os_qualified == true`, in which case REFUSE (G3).
///   * non-zero  → bound-root oracle path. The shared module does NOT own the
///                 oracle (it needs registry + CAS); it returns
///                 `Pass { registry_root_hex }` so the SEQUENCER runs its
///                 existing re-execution (`sequencer.rs` bound branch). The
///                 kernel never reaches here in route A — its claim set is
///                 always taken under a zero root.
/// TRACE_MATRIX FC1a-predicates + FC1b-Q_{t+1}: the single admission contract.
pub fn decide_admission(
    registry_root_hex: &str,
    claims: &PredicateClaimSet,
    os_qualified: bool,
) -> AdmissionVerdict {
    let is_zero_root = registry_root_hex == zero_root_hex();

    if is_zero_root && os_qualified {
        // G3: an OS-qualified run must not verdict-trust a zero registry root.
        return AdmissionVerdict::Fail {
            failed_predicate: String::new(),
            reason: AdmissionFailReason::ZeroRootRefusedForOsQualifiedRun,
        };
    }

    if is_zero_root {
        // Legacy boolean branch — moved verbatim from sequencer.rs:1232-1242.
        // Acceptance first, then settlement, fail on the FIRST false value.
        for claim in &claims.acceptance {
            if !claim.value {
                return AdmissionVerdict::Fail {
                    failed_predicate: claim.id.0.clone(),
                    reason: AdmissionFailReason::AcceptancePredicateFalse,
                };
            }
        }
        for claim in &claims.settlement {
            if !claim.value {
                return AdmissionVerdict::Fail {
                    failed_predicate: claim.id.0.clone(),
                    reason: AdmissionFailReason::SettlementPredicateFalse,
                };
            }
        }
        return AdmissionVerdict::Pass {
            registry_root_hex: registry_root_hex.to_string(),
        };
    }

    // Non-zero bound root: the caller (sequencer) owns the oracle re-execution.
    // We only confirm the branch selection by returning Pass with the bound
    // root; the sequencer keeps its existing registry/CAS re-execution path.
    AdmissionVerdict::Pass {
        registry_root_hex: registry_root_hex.to_string(),
    }
}

/// arg-taint sub-article HARD-GATE wrapper around [`decide_admission`].
///
/// Runs the tainted-arg → privileged-sink check FIRST: if `taint_findings` is
/// non-empty (any flow surfaced by [`arg_taint::arg_taint_v1`]), admission is
/// REFUSED outright — independent of, and prior to, the self-reported predicate
/// booleans and the registry-root branch selection. A clean (empty) findings set
/// delegates to the unchanged [`decide_admission`], so non-taint admission is
/// byte-identical.
///
/// The reject reuses the EXISTING `AcceptancePredicateFalse` reason (NOT a new
/// `AdmissionFailReason` variant), so the genesis-pinned
/// `sequencer.rs::admission_fail_to_transition` exhaustive match stays valid with
/// ZERO pinned edits. The synthetic `failed_predicate` name
/// ([`arg_taint_failed_predicate`]) encodes the arg-taint verdict + the
/// redact-safe per-finding reasons, so a tape auditor reconstructs the gate from
/// the rejection receipt alone (labels + sink identity, never raw value bytes).
///
/// Only the UNPINNED memory-kernel leg calls this; the pinned sequencer keeps
/// calling [`decide_admission`] directly. That is the deliberate seam that wires
/// the hard-gate without touching any pinned file.
/// TRACE_MATRIX FC1a-predicates + FC1b-Q_{t+1}: arg-taint hard-gate admission.
pub fn decide_admission_with_taint(
    registry_root_hex: &str,
    claims: &PredicateClaimSet,
    os_qualified: bool,
    taint_findings: &[ArgTaintFinding],
) -> AdmissionVerdict {
    if !taint_findings.is_empty() {
        return AdmissionVerdict::Fail {
            failed_predicate: arg_taint_failed_predicate(taint_findings),
            reason: AdmissionFailReason::AcceptancePredicateFalse,
        };
    }
    decide_admission(registry_root_hex, claims, os_qualified)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn claim(id: &str, value: bool) -> PredicateClaim {
        PredicateClaim {
            id: PredicateId(id.into()),
            value,
            proof_cid: None,
        }
    }

    #[test]
    fn zero_root_non_qualified_passes_all_true() {
        let claims = PredicateClaimSet {
            acceptance: vec![claim("acc1", true)],
            settlement: vec![],
        };
        assert_eq!(
            decide_admission(&zero_root_hex(), &claims, false),
            AdmissionVerdict::Pass {
                registry_root_hex: zero_root_hex()
            }
        );
    }

    #[test]
    fn zero_root_fails_on_first_false_acceptance() {
        let claims = PredicateClaimSet {
            acceptance: vec![claim("acc1", false)],
            settlement: vec![],
        };
        assert_eq!(
            decide_admission(&zero_root_hex(), &claims, false),
            AdmissionVerdict::Fail {
                failed_predicate: "acc1".into(),
                reason: AdmissionFailReason::AcceptancePredicateFalse,
            }
        );
    }

    #[test]
    fn zero_root_fails_on_false_settlement_after_true_acceptance() {
        let claims = PredicateClaimSet {
            acceptance: vec![claim("acc1", true)],
            settlement: vec![claim("set1", false)],
        };
        assert_eq!(
            decide_admission(&zero_root_hex(), &claims, false),
            AdmissionVerdict::Fail {
                failed_predicate: "set1".into(),
                reason: AdmissionFailReason::SettlementPredicateFalse,
            }
        );
    }

    #[test]
    fn empty_claim_set_passes_zero_root() {
        let claims = PredicateClaimSet::default();
        assert_eq!(
            decide_admission(&zero_root_hex(), &claims, false),
            AdmissionVerdict::Pass {
                registry_root_hex: zero_root_hex()
            }
        );
    }

    #[test]
    fn os_qualified_zero_root_is_refused() {
        let claims = PredicateClaimSet {
            acceptance: vec![claim("self_asserted_acc", true)],
            settlement: vec![],
        };
        assert_eq!(
            decide_admission(&zero_root_hex(), &claims, true),
            AdmissionVerdict::Fail {
                failed_predicate: String::new(),
                reason: AdmissionFailReason::ZeroRootRefusedForOsQualifiedRun,
            }
        );
    }

    #[test]
    fn non_zero_root_selects_bound_branch_pass() {
        let root_hex = hash_to_hex(&Hash::from_bytes([7u8; 32]));
        let claims = PredicateClaimSet::default();
        assert_eq!(
            decide_admission(&root_hex, &claims, true),
            AdmissionVerdict::Pass {
                registry_root_hex: root_hex
            }
        );
    }
}
