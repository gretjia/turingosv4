//! GA-7 — §17.3 name-lie gate: `VERIFY_UCB_PRICE_PRIOR_FLOOR_V1` must BE a live
//! UCB-price-floor, not an argmax-collapse wrapper with a UCB label.
//!
//! Authority: `handover/tracer_bullets/H_HET_2_PHASE2_GATE_DESIGN_2026-06-16.md` (GA-7).
//!
//! TWO sub-gates:
//!
//! (a) UCB count-bonus varies with pull_count — proving the isqrt bonus is a live
//!     count-dependent signal, not a constant.  We feed two ModelInputs that are
//!     IDENTICAL except their pull_count differs (one is a fresh arm, the other is
//!     more explored), run score_and_select, and look up each model's `bonus_bps`
//!     in `sel.rows` by model_id.  The two bonus values MUST differ.
//!
//!     FAILABILITY: if bonus_bps were constant (i.e. "UCB" were a name-lie), the two
//!     values would be equal, and `bonus_bps_differs_with_pull_count` would panic.
//!
//! (b) ε exploration floor is reachable (not dead code) — we construct a state where
//!     budget is tight relative to the owed floor quota, assert the reason is Floor,
//!     and confirm `exploration_active` is true on the winning row.
//!
//!     FAILABILITY: `floor_fires_when_budget_equals_owed_quota` would panic if the
//!     floor arm were unreachable (e.g. always bypassed by a score-path shortcut).
//!
//! No src/ changes; all helpers and fixtures are test-local.

use turingosv4::runtime::budget_allocation_telemetry::SelectionReason;
use turingosv4::runtime::routing_policy::{score_and_select, ModelInput, RoutingPolicyConfig};

/// Tiny helper — mirrors the `mi` helper inside routing_policy.rs unit tests.
fn mi(id: &str, pull: u32, ver: u32, hf: u32, price: u64, floor: u64) -> ModelInput {
    ModelInput {
        model_id: id.into(),
        pull_count: pull,
        verify_count: ver,
        hard_failure_streak: hf,
        price_prior_bps: price,
        floor_quota_remaining: floor,
    }
}

/// Locate a row in sel.rows by model_id.
fn row_bonus(rows: &[turingosv4::runtime::budget_allocation_telemetry::ModelScoreRow], id: &str) -> u64 {
    rows.iter()
        .find(|r| r.model_id == id)
        .unwrap_or_else(|| panic!("model_id '{}' not found in sel.rows", id))
        .bonus_bps
}

// ── Sub-gate (a): UCB bonus varies with pull_count ───────────────────────────

/// §17.3 UCB claim: the count-bonus (isqrt term) is a live function of pull_count.
/// Feeds two models that differ ONLY in pull_count (zero vs non-zero exploration).
/// Asserts their bonus_bps differs — the "UCB" name is not a lie.
#[test]
fn bonus_bps_differs_with_pull_count() {
    let cfg = RoutingPolicyConfig::default();

    // "fresh" arm: pull_count = 0 (never pulled on this target).
    // "explored" arm: pull_count = 5 (more pulls → lower exploration bonus).
    // All other parameters identical: same verify, hf, price, floor, ample budget.
    let models = vec![
        mi("fresh", 0, 0, 0, 0, 0),
        mi("explored", 5, 0, 0, 0, 0),
    ];

    let sel = score_and_select(&cfg, &models, 100);

    let bonus_fresh = row_bonus(&sel.rows, "fresh");
    let bonus_explored = row_bonus(&sel.rows, "explored");

    // The UCB isqrt bonus for "fresh" (pull=0) must exceed that of "explored" (pull=5)
    // because isqrt((total+1)/(0+1)) > isqrt((total+1)/(5+1)) when total >= 5.
    // If they were equal the "UCB" label would be a name-lie (§17.3).
    assert_ne!(
        bonus_fresh, bonus_explored,
        "GA-7(a) FAIL: bonus_bps is identical for pull=0 and pull=5 \
         — the UCB count-bonus is not varying with pull_count; \
         'VERIFY_UCB_PRICE_PRIOR_FLOOR_V1' would be a name-lie (§17.3). \
         bonus_fresh={bonus_fresh}, bonus_explored={bonus_explored}"
    );

    // Additional direction check: fresh arm should have the HIGHER exploration bonus.
    assert!(
        bonus_fresh > bonus_explored,
        "GA-7(a): expected bonus_fresh ({bonus_fresh}) > bonus_explored ({bonus_explored}) \
         — less-explored model should have higher UCB exploration incentive"
    );
}

/// FAILABLE inverse: if we artificially make both pull_counts equal the bonuses
/// ARE equal, proving the gate above is not vacuously true.
#[test]
fn bonus_bps_is_equal_when_pull_counts_are_equal() {
    let cfg = RoutingPolicyConfig::default();

    // Both models have pull_count = 3; all else equal.
    let models = vec![
        mi("alpha", 3, 0, 0, 0, 0),
        mi("beta", 3, 0, 0, 0, 0),
    ];

    let sel = score_and_select(&cfg, &models, 100);

    let bonus_alpha = row_bonus(&sel.rows, "alpha");
    let bonus_beta = row_bonus(&sel.rows, "beta");

    // When pull counts are equal, bonuses must be equal — proving the mechanism
    // is genuinely pull-count-sensitive (not always equal, not always different).
    assert_eq!(
        bonus_alpha, bonus_beta,
        "GA-7 symmetry check: equal pull counts must yield equal bonuses. \
         bonus_alpha={bonus_alpha}, bonus_beta={bonus_beta}"
    );
}

// ── Sub-gate (b): ε floor is reachable (not dead code) ───────────────────────

/// §17.3 FLOOR claim: the ε exploration floor arm must be live code, not dead.
/// We construct a state where `remaining_target_budget == owed_floor_total`,
/// which satisfies `must_spend_floor`, and verify:
///   - selection reason == Floor
///   - the winning row has exploration_active == true
///   - its floor_quota_remaining_before > 0 (it was genuinely owed)
#[test]
fn floor_fires_when_budget_equals_owed_quota() {
    let cfg = RoutingPolicyConfig::default();

    // One arm with a strong verify record (would win on score alone, pull=4, ver=4).
    // One arm with zero verifies but an owed floor quota (floor=1).
    // The "greedy" arm has no floor quota (floor=0); the "floor_arm" has quota=1.
    //
    // remaining_target_budget = 1 = owed_total(=1) → must_spend_floor = true.
    let models = vec![
        mi("greedy",    4, 4, 0, 0, 0), // strong score, no floor owed
        mi("floor_arm", 0, 0, 0, 0, 1), // low score, has owed floor quota
    ];

    let sel = score_and_select(&cfg, &models, /* remaining = */ 1);

    assert!(
        matches!(sel.reason, SelectionReason::Floor),
        "GA-7(b) FAIL: expected SelectionReason::Floor when budget == owed_quota. \
         Got: {:?}. The ε-floor arm may be dead code — 'FLOOR' in the name is a lie.",
        sel.reason
    );

    // Confirm the selected model is the floor-owed arm, not the greedy high-scorer.
    assert_eq!(
        sel.selected_model_id, "floor_arm",
        "GA-7(b): floor arm must win when budget is exhausted down to the floor quota"
    );

    // Confirm exploration_active is true on the winning row (floor only fires for
    // active models — hard_failure_streak < n_hard_fail).
    let winner_row = sel
        .rows
        .iter()
        .find(|r| r.model_id == "floor_arm")
        .expect("floor_arm row must exist");
    assert!(
        winner_row.exploration_active,
        "GA-7(b): winning floor model must have exploration_active == true"
    );
    assert!(
        winner_row.floor_quota_remaining_before > 0,
        "GA-7(b): winning floor model must have had quota before this tick"
    );
    // After winning the tick, floor quota should have decremented.
    assert_eq!(
        winner_row.floor_quota_remaining_after,
        winner_row.floor_quota_remaining_before - 1,
        "GA-7(b): floor_quota_remaining_after must be floor_quota_remaining_before - 1"
    );
}

/// FAILABLE: when budget is ample (> owed_total), the floor arm does NOT fire —
/// proving the floor check is a live predicate (it CAN be false), not always true.
#[test]
fn floor_does_not_fire_when_budget_is_ample() {
    let cfg = RoutingPolicyConfig::default();

    // Same models as above, but remaining = 100 >> owed_total(=1).
    // The strong verify arm should win on score.
    let models = vec![
        mi("greedy",    4, 4, 0, 0, 0),
        mi("floor_arm", 0, 0, 0, 0, 1),
    ];

    let sel = score_and_select(&cfg, &models, /* remaining = */ 100);

    // With ample budget, the floor must NOT fire — the high-scorer wins.
    assert!(
        !matches!(sel.reason, SelectionReason::Floor),
        "GA-7(b) inverse: floor MUST NOT fire when budget is ample. \
         Got SelectionReason::Floor unexpectedly."
    );

    assert_eq!(
        sel.selected_model_id, "greedy",
        "GA-7(b) inverse: greedy high-scorer must win when budget is ample"
    );
}

// ── Combined §17.3 predicate: mechanism name is not a lie ────────────────────

/// Concise single-assertion §17.3 gate: the two structural properties that make
/// 'VERIFY_UCB_PRICE_PRIOR_FLOOR_V1' a truthful name both hold simultaneously:
///   1. UCB bonus varies with pull_count (count-aware exploration).
///   2. The floor arm fires when owed_total == remaining_budget (floor is live).
#[test]
fn verify_ucb_price_prior_floor_v1_name_is_not_a_lie() {
    let cfg = RoutingPolicyConfig::default();

    // Property 1: bonus_bps must vary with pull_count.
    let two_arms = vec![mi("low", 0, 0, 0, 0, 0), mi("high", 8, 0, 0, 0, 0)];
    let sel1 = score_and_select(&cfg, &two_arms, 100);
    let b_low  = row_bonus(&sel1.rows, "low");
    let b_high = row_bonus(&sel1.rows, "high");
    assert_ne!(
        b_low, b_high,
        "§17.3 name-lie: UCB bonus must vary with pull_count (got equal: {b_low})"
    );

    // Property 2: floor selection fires when budget == owed_total.
    let floor_arms = vec![
        mi("scorer",   5, 5, 0, 0, 0),
        mi("floored",  0, 0, 0, 0, 2),
    ];
    let sel2 = score_and_select(&cfg, &floor_arms, /* remaining = owed_total = */ 2);
    assert!(
        matches!(sel2.reason, SelectionReason::Floor),
        "§17.3 name-lie: FLOOR must fire when budget == owed_total; got {:?}",
        sel2.reason
    );
}
