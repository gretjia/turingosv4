//! GA-8 — arm-parity / Goodhart shield (§4 economic dominance fairness).
//!
//! Authority: H-HET-2 Phase-2 gate design GA-8
//! (`handover/tracer_bullets/H_HET_2_PHASE2_GATE_DESIGN_2026-06-16.md`).
//!
//! ENFORCES: the TREATMENT arm (dynamic UCB budget market) must NOT silently exceed the
//! controls' budget. Three arms — treatment=UCB, control1=round_robin, control2=best_single
//! — each represented as `ArmBudget { tokens, microusd, proposal_calls }`, all starting from
//! equal budget `B_target`. The parity predicate asserts:
//!
//!   treatment.tokens        <= min(controls[*].tokens)
//!   treatment.microusd      <= min(controls[*].microusd)
//!   treatment.proposal_calls <= min(controls[*].proposal_calls)
//!   AND every arm (treatment + controls) ≤ B_target on every resource dimension.
//!
//! FAILABLE: four failing tests mutate the fixture so that the treatment arm exceeds a control
//! or exceeds B_target on each resource dimension, and assert the predicate returns false.
//! The gate cannot be vacuously green.
//!
//! No src/ changes. No new crate module. All fixtures and logic are test-local.

// ── Types ─────────────────────────────────────────────────────────────────────

/// Resource consumption of one experiment arm within a single budget slot.
#[derive(Debug, Clone, Copy)]
struct ArmBudget {
    /// LLM token spend (prompt + completion) for this arm's allocations.
    tokens: u64,
    /// Monetary cost (integer micro-USD, no f64 on the economic path).
    microusd: u64,
    /// Proposal/model-call count consumed by this arm.
    proposal_calls: u64,
}

// ── Parity predicate ─────────────────────────────────────────────────────────

/// The gate predicate — returns `true` iff ALL parity conditions hold:
///
///   1. Every arm (treatment AND every control) stays within `b_target` on each dimension.
///   2. `treatment.tokens        <= min(control.tokens        for control in controls)`
///   3. `treatment.microusd      <= min(control.microusd      for control in controls)`
///   4. `treatment.proposal_calls <= min(control.proposal_calls for control in controls)`
///
/// Integer-only; no float.
fn parity_holds(treatment: ArmBudget, controls: &[ArmBudget], b_target: u64) -> bool {
    // ── Rule 1: every arm ≤ B_target on every dimension ──────────────────────
    for arm in std::iter::once(treatment).chain(controls.iter().copied()) {
        if arm.tokens > b_target || arm.microusd > b_target || arm.proposal_calls > b_target {
            return false;
        }
    }

    if controls.is_empty() {
        return true; // vacuous parity with no controls
    }

    // ── Rules 2-4: treatment ≤ min(controls) per dimension ──────────────────
    let min_tokens = controls.iter().map(|c| c.tokens).min().unwrap();
    let min_microusd = controls.iter().map(|c| c.microusd).min().unwrap();
    let min_proposals = controls.iter().map(|c| c.proposal_calls).min().unwrap();

    treatment.tokens <= min_tokens
        && treatment.microusd <= min_microusd
        && treatment.proposal_calls <= min_proposals
}

// ── Happy-path: arms within equal budget, treatment ≤ controls ───────────────

/// Baseline: treatment and controls all under B_target, treatment ≤ every control
/// on every dimension. Parity must hold.
#[test]
fn equal_budget_treatment_under_controls_parity_holds() {
    let b_target: u64 = 10_000;
    // treatment=UCB, control1=round_robin, control2=best_single
    let treatment = ArmBudget { tokens: 8_500, microusd: 7_000, proposal_calls: 90 };
    let controls = [
        ArmBudget { tokens: 9_000, microusd: 7_500, proposal_calls: 100 }, // round_robin
        ArmBudget { tokens: 9_200, microusd: 8_000, proposal_calls: 105 }, // best_single
    ];
    assert!(
        parity_holds(treatment, &controls, b_target),
        "treatment ≤ all controls on every dimension and every arm ≤ B_target \
         — parity must hold"
    );
}

/// Treatment exactly equal to the control minimum on each dimension: still holds
/// (the predicate uses ≤, not strict <).
#[test]
fn treatment_equal_to_min_control_parity_holds() {
    let b_target: u64 = 10_000;
    let treatment = ArmBudget { tokens: 9_000, microusd: 7_500, proposal_calls: 100 };
    let controls = [
        ArmBudget { tokens: 9_000, microusd: 7_500, proposal_calls: 100 }, // ties treatment
        ArmBudget { tokens: 9_200, microusd: 8_000, proposal_calls: 110 },
    ];
    assert!(
        parity_holds(treatment, &controls, b_target),
        "treatment equal to minimum control — parity must hold (≤ is satisfied)"
    );
}

// ── Failability tests — each proves the predicate is not vacuously green ──────

/// FAILABILITY (primary): treatment.tokens exceeds the minimum control.tokens.
/// The predicate must return false — this is the canonical GA-8 failure mode.
#[test]
fn treatment_over_control_tokens_is_rejected() {
    let b_target: u64 = 10_000;
    let treatment = ArmBudget {
        tokens: 9_500, // exceeds control1 (round_robin) tokens = 9_000
        microusd: 7_000,
        proposal_calls: 90,
    };
    let controls = [
        ArmBudget { tokens: 9_000, microusd: 8_000, proposal_calls: 100 }, // round_robin
        ArmBudget { tokens: 9_200, microusd: 8_200, proposal_calls: 105 }, // best_single
    ];
    assert!(
        !parity_holds(treatment, &controls, b_target),
        "treatment.tokens {} > min(controls.tokens) {} — parity predicate must return false",
        treatment.tokens,
        controls.iter().map(|c| c.tokens).min().unwrap()
    );
}

/// FAILABILITY: treatment.microusd exceeds the minimum control.microusd.
/// The economic cost dimension of the Goodhart shield.
#[test]
fn treatment_over_control_microusd_is_rejected() {
    let b_target: u64 = 10_000;
    let treatment = ArmBudget {
        tokens: 8_000,
        microusd: 9_100, // exceeds control2 (best_single) microusd = 9_000
        proposal_calls: 90,
    };
    let controls = [
        ArmBudget { tokens: 9_000, microusd: 9_500, proposal_calls: 100 }, // round_robin
        ArmBudget { tokens: 8_500, microusd: 9_000, proposal_calls: 95 },  // best_single
    ];
    assert!(
        !parity_holds(treatment, &controls, b_target),
        "treatment.microusd {} > min(controls.microusd) {} — parity predicate must return false",
        treatment.microusd,
        controls.iter().map(|c| c.microusd).min().unwrap()
    );
}

/// FAILABILITY: treatment.proposal_calls exceeds the minimum control.proposal_calls.
/// Proves the model-call dimension is guarded, not just tokens or microusd.
#[test]
fn treatment_over_control_proposal_calls_is_rejected() {
    let b_target: u64 = 10_000;
    let treatment = ArmBudget {
        tokens: 8_000,
        microusd: 7_000,
        proposal_calls: 120, // exceeds control1 (round_robin) proposal_calls = 100
    };
    let controls = [
        ArmBudget { tokens: 9_000, microusd: 8_000, proposal_calls: 100 }, // round_robin
        ArmBudget { tokens: 9_200, microusd: 8_200, proposal_calls: 130 }, // best_single
    ];
    assert!(
        !parity_holds(treatment, &controls, b_target),
        "treatment.proposal_calls {} > min(controls.proposal_calls) {} \
         — parity predicate must return false",
        treatment.proposal_calls,
        controls.iter().map(|c| c.proposal_calls).min().unwrap()
    );
}

/// FAILABILITY: treatment.tokens exceeds B_target itself (independent of controls).
/// Even if controls are also under their caps, a treatment arm blowing through B_target
/// must be caught — this is the "budget cap" half of the Goodhart shield.
#[test]
fn treatment_exceeds_b_target_is_rejected() {
    let b_target: u64 = 10_000;
    let treatment = ArmBudget {
        tokens: 11_000, // exceeds B_target
        microusd: 8_000,
        proposal_calls: 90,
    };
    let controls = [
        ArmBudget { tokens: 9_500, microusd: 8_500, proposal_calls: 100 },
        ArmBudget { tokens: 9_800, microusd: 8_200, proposal_calls: 95 },
    ];
    assert!(
        !parity_holds(treatment, &controls, b_target),
        "treatment.tokens {} > B_target {} — parity predicate must return false",
        treatment.tokens, b_target
    );
}
