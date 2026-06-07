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
