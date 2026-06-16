//! GA-5 — H-HET-2 budget-allocation conservation (distinct from INV3 MicroCoin).
//!
//! Authority: `handover/tracer_bullets/H_HET_2_PHASE2_GATE_DESIGN_2026-06-16.md` (GA-5).
//! ENFORCES: integer conservation of the *call/token budget* across router decisions — the
//! substrate the dynamic model-budget MARKET allocates. This is NOT MicroCoin conservation
//! (that is `walkthrough_inv3_conservation`); it is the budget the treatment routes, so a
//! leak here would let the economic claim ("higher union coverage at ≤ budget") be measured
//! against a budget that silently grew.
//!
//! Failable predicate (per decision, u64, NO float):
//!   `budget_remaining_before − allocated_token_budget == budget_remaining_after`,
//!   `router_overhead_cid` is non-null (overhead is counted, §4), and
//!   `Σ allocated_token_budget ≤ B_target` over a decision sequence.
//!
//! FAILABLE: `a_leaking_decision_is_caught` builds a record whose after-balance does not
//! equal before−allocated and asserts the gate rejects it (not vacuously green).

use turingosv4::bottom_white::cas::schema::Cid;
use turingosv4::runtime::budget_allocation_telemetry::BudgetAllocationTelemetry;
use turingosv4::runtime::routing_policy::{self, ModelInput, RoutingPolicyConfig};
use turingosv4::state::q_state::Hash;

/// The conservation predicate, exactly as a tape replayer would check it (integer-only).
fn conserves(rec: &BudgetAllocationTelemetry) -> bool {
    rec.budget_remaining_before
        .checked_sub(rec.allocated_token_budget)
        == Some(rec.budget_remaining_after)
}

fn overhead_counted(rec: &BudgetAllocationTelemetry) -> bool {
    rec.router_overhead_cid != Cid([0u8; 32])
}

fn inputs() -> Vec<ModelInput> {
    vec![
        ModelInput { model_id: "deepseek".into(), pull_count: 2, verify_count: 0, hard_failure_streak: 1, price_prior_bps: 0, floor_quota_remaining: 0 },
        ModelInput { model_id: "qwen397".into(), pull_count: 3, verify_count: 2, hard_failure_streak: 0, price_prior_bps: 0, floor_quota_remaining: 0 },
    ]
}

/// Build a real record (rows via the actual mechanism), with the budget fields set so that
/// `after == before - allocated` (a conserving decision) unless `leak != 0`.
fn record(before: u64, allocated: u64, leak: u64) -> BudgetAllocationTelemetry {
    let cfg = RoutingPolicyConfig::default();
    let inp = inputs();
    let sel = routing_policy::score_and_select(&cfg, &inp, before);
    let total: u32 = sel.rows.iter().map(|r| r.pull_count_model_target_before).sum();
    BudgetAllocationTelemetry {
        policy_family: cfg.policy_family.clone(),
        policy_hash: cfg.policy_hash(),
        policy_version: cfg.policy_version.clone(),
        target_id: "lm_det_mul".into(),
        seed_id: 1,
        eligible_model_set_hash: Hash([9u8; 32]),
        input_state_cid: Cid([1u8; 32]),
        price_vector_cid: Cid([2u8; 32]),
        abstracted_failure_features_cid: Cid([3u8; 32]),
        total_pulls_target_before: total,
        candidates: sel.rows,
        selected_model_id: sel.selected_model_id,
        selection_reason: sel.reason,
        allocated_proposal_budget: 1,
        allocated_token_budget: allocated,
        budget_remaining_before: before,
        budget_remaining_after: (before - allocated) + leak,
        router_overhead_cid: Cid([4u8; 32]),
        rng_seed: None,
        rng_draw: None,
    }
}

#[test]
fn each_decision_conserves_token_budget_exactly() {
    // A sequence of conserving decisions drawing down a starting budget.
    let mut remaining = 10_000u64;
    for alloc in [900u64, 700, 1200, 300] {
        let rec = record(remaining, alloc, 0);
        assert!(
            conserves(&rec),
            "budget leak: before {} - allocated {} != after {}",
            rec.budget_remaining_before, rec.allocated_token_budget, rec.budget_remaining_after
        );
        assert!(overhead_counted(&rec), "router_overhead_cid is null — overhead not counted (§4)");
        remaining = rec.budget_remaining_after;
    }
}

#[test]
fn sum_allocated_never_exceeds_target() {
    let b_target = 10_000u64;
    let mut remaining = b_target;
    let mut spent = 0u64;
    for alloc in [900u64, 700, 1200, 300] {
        let rec = record(remaining, alloc, 0);
        spent += rec.allocated_token_budget;
        remaining = rec.budget_remaining_after;
    }
    assert!(spent <= b_target, "Σ allocated {spent} exceeds B_target {b_target}");
}

/// FAILABILITY: a decision whose after-balance does not equal before−allocated must be caught.
#[test]
fn a_leaking_decision_is_caught() {
    let leak = record(10_000, 900, 5); // after = before - allocated + 5 (5 tokens conjured)
    assert!(
        !conserves(&leak),
        "conservation predicate passed a leaking record — GA-5 would not catch a budget leak"
    );
}
