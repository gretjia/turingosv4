// PPUT-CCL Phase B B4 — post-hoc verifier + dual PPUT (runtime vs verified).
//
// Spec:
//   handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md § B4
//   handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md § 5 (Progress definition)
//   PREREG § 3 anti-Goodhart conformance #7
//     (test_golden_path_requires_ground_truth_acceptance)
//
// Why two PPUT fields:
//   pput_runtime  = Progress_runtime  / (C_i × T_i / 1000)
//   pput_verified = Progress_verified / (C_i × T_i / 1000)   ← North Star
//
//   Progress_runtime  = 1 iff the evaluator's runtime accept gate fired.
//   Progress_verified = 1 iff the post-hoc Lean call returns Ok((true, _))
//                       on the golden_path_payload.
//
//   In current implementation (Phase B), the runtime gate IS the Lean call
//   (no Soft Law mode yet) — so the two agree on every solved run.
//
//   The split exists for Phase C ablation: Soft Law mode fakes runtime
//   acceptance without running Lean. Under Soft Law:
//     pput_runtime  may be > 0 (fake accept inflates Progress_runtime)
//     pput_verified MUST be 0  (Lean reject means Progress_verified = 0)
//   The divergence between the two is the H1 detection mechanism.
//
//   pput_verified is the only North Star metric for H-VPPUT. pput_runtime
//   is emitted only as the divergence signal.

use crate::jsonl_schema::RunAggregate;
use crate::lean4_oracle::Lean4Oracle;

/// Run the post-hoc Lean verification gate on a golden_path_payload.
/// ALWAYS runs the real Lean call. There is NO Soft Law short-circuit
/// here — that's the entire point of the post-hoc layer.
///
/// In current Phase B wiring, the OMEGA-accept code path has already run
/// `verify_omega_detailed` on the same payload and gotten `Ok((true, _))`,
/// so this function is logically guaranteed to return `true` on solved runs.
/// Calling it would just double the Lean cost. Phase C diverges the call
/// site: runtime accept becomes a flag, post-hoc verify stays mandatory.
///
/// Use `compute_progress_verified` instead when the runtime gate already
/// IS the Lean call — it propagates the same truth value without paying
/// for a second Lean process.
pub fn verify_post_hoc(oracle: &Lean4Oracle, golden_path_payload: &str) -> bool {
    matches!(
        oracle.verify_omega_detailed(golden_path_payload),
        Ok((true, _))
    )
}

/// Compute Progress_verified from a (runtime, verified) pair.
///
/// Returns 1 only when both runtime and verified say accept.
/// - Phase B: runtime == verified always (runtime IS Lean). Returns 1 on
///   any accepted run, 0 otherwise.
/// - Phase C Soft Law: runtime can be true while verified is false. The
///   AND collapses to the verified leg, which is the North Star truth.
pub fn compute_progress_verified(runtime_accepted: bool, post_hoc_verified: bool) -> u8 {
    if runtime_accepted && post_hoc_verified {
        1
    } else {
        0
    }
}

/// Compute Progress_runtime from the runtime accept signal alone.
/// Inflates under Soft Law when fake-accept fires without Lean.
pub fn compute_progress_runtime(runtime_accepted: bool) -> u8 {
    if runtime_accepted {
        1
    } else {
        0
    }
}

/// Wrap RunAggregate::compute_pput_verified for callers in evaluator that
/// only have (progress, c_i, t_i_ms). Same math, single source of truth.
pub fn compute_pput(progress: u8, c_i: u64, t_i_ms: u64) -> f64 {
    RunAggregate::compute_pput_verified(progress, c_i, t_i_ms)
}

/// 10^6 × pput. Display unit per PREREG § 5.
pub fn compute_pput_m(progress: u8, c_i: u64, t_i_ms: u64) -> f64 {
    RunAggregate::compute_pput_m_verified(progress, c_i, t_i_ms)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// PREREG § 3 anti-Goodhart conformance #7
    /// (test_golden_path_requires_ground_truth_acceptance):
    /// "synthesize a run that records runtime accept + Lean reject →
    ///  assert progress = 0, pput_verified = 0.0"
    ///
    /// This is the H1 detection gate that makes Soft Law impossible to
    /// hide behind. Without this test failing on a runtime/verified
    /// disagreement, an attacker could inflate North Star PPUT by faking
    /// runtime accepts without paying real Lean verification cost.
    #[test]
    fn test_pput_verified_zero_when_lean_rejects() {
        // Soft Law-style scenario: runtime gate fired, but post-hoc Lean
        // rejected the same payload. The North Star MUST collapse the
        // run's progress to 0.
        let runtime_accepted = true;
        let post_hoc_verified = false;
        let c_i: u64 = 5_000; // tokens
        let t_i_ms: u64 = 30_000; // 30 seconds wall

        let progress_runtime = compute_progress_runtime(runtime_accepted);
        let progress_verified = compute_progress_verified(runtime_accepted, post_hoc_verified);

        let pput_runtime = compute_pput(progress_runtime, c_i, t_i_ms);
        let pput_verified = compute_pput(progress_verified, c_i, t_i_ms);
        let pput_m_verified = compute_pput_m(progress_verified, c_i, t_i_ms);

        assert_eq!(
            progress_runtime, 1u8,
            "runtime gate fired → progress_runtime = 1"
        );
        assert_eq!(
            progress_verified, 0u8,
            "Lean rejected → progress_verified MUST be 0 (North Star truth)"
        );
        assert!(
            pput_runtime > 0.0,
            "pput_runtime inflates under runtime accept (Soft Law signal)"
        );
        assert_eq!(
            pput_verified, 0.0,
            "pput_verified MUST be 0 when Lean rejects — North Star Goodhart shield"
        );
        assert_eq!(
            pput_m_verified, 0.0,
            "pput_m_verified must collapse with pput_verified"
        );

        // Sanity: divergence is detectable. pput_runtime - pput_verified > 0
        // is the H1 signal Phase C scans for.
        assert!(
            pput_runtime - pput_verified > 0.0,
            "(pput_runtime - pput_verified) > 0 ⟺ Soft Law divergence detected"
        );
    }

    #[test]
    fn test_pput_verified_matches_runtime_when_both_accept() {
        // Phase B reality: runtime IS Lean, so on any solved run the two
        // metrics MUST agree. This test guards against accidentally
        // diverging them in B4 wiring (e.g., a typo that AND'ed the wrong
        // booleans and made pput_verified always 0).
        let c_i: u64 = 5_000;
        let t_i_ms: u64 = 30_000;

        let progress_runtime = compute_progress_runtime(true);
        let progress_verified = compute_progress_verified(true, true);

        assert_eq!(
            progress_runtime, progress_verified,
            "Phase B: runtime == verified on solved runs"
        );
        assert_eq!(
            compute_pput(progress_runtime, c_i, t_i_ms),
            compute_pput(progress_verified, c_i, t_i_ms),
            "pput fields must agree when runtime == verified"
        );
    }

    #[test]
    fn test_no_runtime_accept_zeros_both_pput() {
        // No-OMEGA exit: neither runtime nor verified fired.
        let c_i: u64 = 50_000;
        let t_i_ms: u64 = 600_000;

        let progress_runtime = compute_progress_runtime(false);
        let progress_verified = compute_progress_verified(false, false);

        assert_eq!(compute_pput(progress_runtime, c_i, t_i_ms), 0.0);
        assert_eq!(compute_pput(progress_verified, c_i, t_i_ms), 0.0);
    }

    #[test]
    fn test_post_hoc_verified_without_runtime_still_zero_progress() {
        // Defensive: a post-hoc verifier that says "yes" but runtime never
        // fired is a wiring bug, not an honest progress signal. Progress
        // is gated on BOTH runtime initiation AND verified result, so this
        // pathological case must clamp to 0.
        assert_eq!(
            compute_progress_verified(false, true),
            0u8,
            "verified without runtime accept = wiring bug, must clamp to 0"
        );
    }
}
