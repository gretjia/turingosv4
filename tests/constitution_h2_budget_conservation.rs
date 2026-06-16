//! GA-5 — H-HET-2 budget-allocation conservation (distinct from INV3 MicroCoin).
//!
//! Authority: `handover/tracer_bullets/H_HET_2_PHASE2_GATE_DESIGN_2026-06-16.md` (GA-5).
//! ENFORCES: integer conservation of the *call budget* across router decisions — the substrate
//! the dynamic model-budget MARKET allocates. This is NOT MicroCoin conservation (that is
//! `walkthrough_inv3_conservation`); it is the budget the treatment routes, so a leak here would
//! let the economic claim ("higher union coverage at ≤ budget") be measured against a budget that
//! silently grew.
//!
//! UNIT (audit-critical, fixed 2026-06-16): the per-tick budget BALANCE is denominated in
//! PROPOSAL-CALL units — the SAME unit as `rt_total_budget = effective_rounds * agents.len()` and
//! the SAME unit `routing_policy::score_and_select`'s 3rd arg expects (it compares `remaining`
//! against `Σ floor_quota_remaining`, where `floor_quota = floor(ε * rt_total_budget)` is also in
//! CALL units). One routing tick funds exactly ONE proposal call, so the conserved quantity is
//!   `budget_remaining_before − allocated_proposal_budget == budget_remaining_after`.
//! The PRIOR gate subtracted `allocated_token_budget` (a TOKEN-units reservation field) from the
//! CALL-units balance — `remaining − 900 == remaining − 1` — which is FALSE on real-run records.
//! That bug was masked because the gate only ever fed itself synthetic records with matching
//! fields; it never saw the values the run path actually emits. `run_path_helper_conserves`
//! below closes that gap by feeding the EXACT `budget_alloc_fields` helper the run path calls.
//!
//! The token field is conserved on a SEPARATE channel: `allocated_token_budget` is a per-tick
//! token RESERVATION (the proposal's max_tokens cap), bounded by
//!   `Σ allocated_token_budget ≤ rt_total_budget × MAX_PROPOSAL_TOKENS`.
//!
//! Failable predicate (per decision, u64, NO float):
//!   `budget_remaining_before − allocated_proposal_budget == budget_remaining_after`,
//!   `router_overhead_cid` is non-null (overhead is counted, §4), and
//!   `Σ allocated_token_budget ≤ rt_total_budget × MAX_PROPOSAL_TOKENS` over a decision sequence.
//!
//! FAILABLE: `a_leaking_decision_is_caught` builds a record whose after-balance does not equal
//! before−allocated and asserts the gate rejects it (not vacuously green).

use turingosv4::bottom_white::cas::schema::Cid;
use turingosv4::runtime::budget_allocation_telemetry::{
    budget_alloc_fields, BudgetAllocationTelemetry, MAX_PROPOSAL_TOKENS,
};
use turingosv4::runtime::routing_policy::{self, ModelInput, RoutingPolicyConfig};
use turingosv4::state::q_state::Hash;

/// The conservation predicate, exactly as a tape replayer would check it (integer-only).
/// The conserved unit is the CALL budget: the balance draws down by `allocated_proposal_budget`
/// each tick, NOT by the token-reservation field.
fn conserves(rec: &BudgetAllocationTelemetry) -> bool {
    rec.budget_remaining_before
        .checked_sub(rec.allocated_proposal_budget)
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

/// Build a record (rows via the actual mechanism). Budget fields default to a CONSERVING tick
/// (`after == before - allocated_proposal_budget`); `proposal_leak != 0` conjures balance to
/// prove the predicate bites. `allocated_token` sets the separate token-reservation field.
fn record(before: u64, allocated_token: u64, allocated_proposal: u64, proposal_leak: u64) -> BudgetAllocationTelemetry {
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
        allocated_proposal_budget: allocated_proposal,
        allocated_token_budget: allocated_token,
        budget_remaining_before: before,
        budget_remaining_after: (before - allocated_proposal) + proposal_leak,
        router_overhead_cid: Cid([4u8; 32]),
        rng_seed: None,
        rng_draw: None,
    }
}

#[test]
fn each_decision_conserves_call_budget_exactly() {
    // A sequence of conserving decisions drawing down a starting CALL budget by 1 per tick.
    let mut remaining = 24u64; // e.g. effective_rounds=6 * agents=4 = 24 proposal calls
    for _ in 0..4 {
        let rec = record(remaining, MAX_PROPOSAL_TOKENS, 1, 0);
        assert!(
            conserves(&rec),
            "call-budget leak: before {} - allocated_proposal {} != after {}",
            rec.budget_remaining_before, rec.allocated_proposal_budget, rec.budget_remaining_after
        );
        assert!(overhead_counted(&rec), "router_overhead_cid is null — overhead not counted (§4)");
        remaining = rec.budget_remaining_after;
    }
}

/// The SEPARATE token-reservation channel: Σ allocated_token_budget over the whole run must not
/// exceed rt_total_budget × MAX_PROPOSAL_TOKENS (each tick reserves at most one proposal's cap).
#[test]
fn sum_allocated_token_never_exceeds_reservation_bound() {
    let rt_total_budget = 24u64;
    let bound = rt_total_budget * MAX_PROPOSAL_TOKENS;
    let mut remaining = rt_total_budget;
    let mut token_spent = 0u64;
    for _ in 0..rt_total_budget {
        let rec = record(remaining, MAX_PROPOSAL_TOKENS, 1, 0);
        token_spent += rec.allocated_token_budget;
        remaining = rec.budget_remaining_after;
    }
    assert!(
        token_spent <= bound,
        "Σ allocated_token {token_spent} exceeds reservation bound {bound}"
    );
}

/// THE GAP-CLOSER: feed the EXACT helper the run path uses (`budget_alloc_fields`) and assert the
/// produced fields conserve on a real-shaped tick. This is what the prior gate never did — it
/// built only its own synthetic records, so the run-path unit mismatch went undetected.
#[test]
fn run_path_helper_conserves() {
    let rt_total_budget = 24u64; // effective_rounds * agents.len()
    for step_idx in 0..rt_total_budget {
        let (proposal, token, before, after) = budget_alloc_fields(rt_total_budget, step_idx);
        // Build the record EXACTLY as the run path does: helper-produced budget fields.
        let cfg = RoutingPolicyConfig::default();
        let sel = routing_policy::score_and_select(&cfg, &inputs(), before);
        let total: u32 = sel.rows.iter().map(|r| r.pull_count_model_target_before).sum();
        let rec = BudgetAllocationTelemetry {
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
            allocated_proposal_budget: proposal,
            allocated_token_budget: token,
            budget_remaining_before: before,
            budget_remaining_after: after,
            router_overhead_cid: Cid([4u8; 32]),
            rng_seed: None,
            rng_draw: None,
        };
        assert!(
            conserves(&rec),
            "run-path helper produced a non-conserving tick at step {step_idx}: {} - {} != {}",
            rec.budget_remaining_before, rec.allocated_proposal_budget, rec.budget_remaining_after
        );
        // The token field is the reservation cap, NOT a balance term.
        assert_eq!(rec.allocated_token_budget, MAX_PROPOSAL_TOKENS);
        assert_eq!(rec.allocated_proposal_budget, 1);
    }
}

/// REGRESSION PIN: the prior (buggy) predicate subtracted the TOKEN field from the CALL balance.
/// On a real-shaped run-path record that pairing is FALSE — this proves the unit was wrong and
/// the corrected `conserves()` is the only one that holds on the helper's output.
#[test]
fn token_unit_predicate_would_fail_on_real_record() {
    let (_proposal, token, before, after) = budget_alloc_fields(24, 0);
    // Old buggy predicate: before - allocated_token_budget == after  →  24 - 900 == 23  → false.
    assert_ne!(
        before.checked_sub(token),
        Some(after),
        "the token-units predicate must NOT hold on a real record (that was the GA-5 defect)"
    );
}

/// FAILABILITY: a decision whose after-balance does not equal before−allocated_proposal must be
/// caught (the gate is not vacuously green).
#[test]
fn a_leaking_decision_is_caught() {
    let leak = record(24, MAX_PROPOSAL_TOKENS, 1, 5); // after = before - 1 + 5 (5 calls conjured)
    assert!(
        !conserves(&leak),
        "conservation predicate passed a leaking record — GA-5 would not catch a budget leak"
    );
}
