//! GA-0 — H-HET-2 frozen-policy anti-drift pin.
//!
//! Authority: `handover/tracer_bullets/H_HET_2_PHASE2_GATE_DESIGN_2026-06-16.md` (GA-0) +
//! prereg `handover/preregistration/H_HET_2_DYNAMIC_MODEL_BUDGET_PREREG_2026-06-15.md:45`
//! (`FROZEN_POLICY_HASH`). Charter `TB_DYNAMIC_MODEL_BUDGET_MARKET` (frozen).
//!
//! ENFORCES (§2 frozen policy): the routing policy that will run the H-HET-2 confirmatory
//! experiment is BYTE-IDENTICAL to the one the prereg froze. Any change to a frozen
//! parameter (W_VERIFY:W_PRICE, caps, N_cold, clamps, C_UCB, ε floor, N_hard_fail,
//! tie-break) flips `RoutingPolicyConfig::default().policy_hash()` and turns this RED — so a
//! silent post-prereg drift cannot reach a paid run undetected.
//!
//! §17 G6 NOTE (pre-empt a false flag): asserting against a compile-time literal here is the
//! LEGITIMATE anti-drift use — a provenance pin on the policy INPUT, NOT a forbidden
//! pass-condition on an experimental OUTCOME. G6 forbids asserting that a *result* equals a
//! literal; this pins that the *frozen config* did not move. The two are distinct.
//!
//! FAILABLE: `mutated_config_does_not_match_the_pin` proves the pin is discriminating (a
//! one-parameter change diverges), so the gate is not vacuously green.

use turingosv4::runtime::routing_policy::RoutingPolicyConfig;

/// The prereg-frozen policy hash (FROZEN_POLICY_HASH, 2026-06-15). Do NOT edit this literal
/// to make the test pass — if `default()` legitimately changes, that is a Class-4 prereg
/// amendment, not a test fix.
const FROZEN_POLICY_HASH: &str =
    "9fb0f612df2054049a3799869aafe6c401eb8c72c27a1e581d3ed901913f263a";

fn hex(cfg: &RoutingPolicyConfig) -> String {
    cfg.policy_hash().0.iter().map(|b| format!("{b:02x}")).collect()
}

#[test]
fn default_policy_hash_equals_frozen_prereg_literal() {
    assert_eq!(
        hex(&RoutingPolicyConfig::default()),
        FROZEN_POLICY_HASH,
        "RoutingPolicyConfig::default() drifted from the prereg-frozen policy. A frozen \
         parameter changed. This is a Class-4 prereg amendment, not a test edit — re-freeze \
         via the charter, do not bump the literal."
    );
}

/// FAILABILITY proof: a mutated config (one frozen parameter changed) must NOT match the pin.
/// If this ever passes, the pin is vacuous (e.g. policy_hash ignores the parameter).
#[test]
fn mutated_config_does_not_match_the_pin() {
    let mut mutated = RoutingPolicyConfig::default();
    mutated.w_verify = mutated.w_verify.wrapping_add(1);
    assert_ne!(
        hex(&mutated),
        FROZEN_POLICY_HASH,
        "a one-parameter mutation produced the frozen hash — policy_hash() is not covering \
         w_verify, so GA-0 would not catch real drift"
    );
}
