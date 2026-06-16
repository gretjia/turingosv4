//! GA-6 — H-HET-2 router-decision provenance completeness.
//!
//! Authority: `handover/tracer_bullets/H_HET_2_PHASE2_GATE_DESIGN_2026-06-16.md` (GA-6).
//! ENFORCES (§4): NO untraced allocation enters the primary metric. Every dynamic
//! model-budget allocation on the tape must attribute to a REAL scored candidate — its
//! `selected_model_id` must be non-empty AND appear among the `candidates` rows the router
//! actually scored. A "ghost" allocation (selected model not among the scored set) would let
//! budget flow to a model with no recorded score/price/UCB basis, breaking the audit chain
//! the economic claim rests on.
//!
//! Scope note: the carrier's `AttemptNode.action_source/decision_source` live in the
//! `lean_market_agent` bin (not importable by an integration test). This gate witnesses the
//! lib-level routing provenance (`BudgetAllocationTelemetry.selected_model_id` ⊆ scored
//! `candidates`), which is the tape-resident, replayable form of the same invariant.
//!
//! FAILABLE: `a_ghost_allocation_is_caught` injects a selected model absent from the scored
//! candidates and asserts the completeness check rejects it.

use turingosv4::bottom_white::cas::schema::Cid;
use turingosv4::runtime::budget_allocation_telemetry::BudgetAllocationTelemetry;
use turingosv4::runtime::routing_policy::{self, ModelInput, RoutingPolicyConfig};
use turingosv4::state::q_state::Hash;

/// An allocation is attributed iff its selected model is non-empty and is one of the models
/// the router actually scored (present in `candidates`).
fn attributed(rec: &BudgetAllocationTelemetry) -> bool {
    !rec.selected_model_id.is_empty()
        && rec.candidates.iter().any(|c| c.model_id == rec.selected_model_id)
}

fn inputs() -> Vec<ModelInput> {
    vec![
        ModelInput { model_id: "deepseek".into(), pull_count: 2, verify_count: 0, hard_failure_streak: 1, price_prior_bps: 0, floor_quota_remaining: 0 },
        ModelInput { model_id: "qwen397".into(), pull_count: 3, verify_count: 2, hard_failure_streak: 0, price_prior_bps: 0, floor_quota_remaining: 0 },
        ModelInput { model_id: "glm".into(), pull_count: 1, verify_count: 0, hard_failure_streak: 0, price_prior_bps: 0, floor_quota_remaining: 1 },
    ]
}

fn record(seed_id: u64, selected_override: Option<&str>) -> BudgetAllocationTelemetry {
    let cfg = RoutingPolicyConfig::default();
    let inp = inputs();
    let sel = routing_policy::score_and_select(&cfg, &inp, 5_000);
    let total: u32 = sel.rows.iter().map(|r| r.pull_count_model_target_before).sum();
    BudgetAllocationTelemetry {
        policy_family: cfg.policy_family.clone(),
        policy_hash: cfg.policy_hash(),
        policy_version: cfg.policy_version.clone(),
        target_id: "lm_det_mul".into(),
        seed_id,
        eligible_model_set_hash: Hash([9u8; 32]),
        input_state_cid: Cid([1u8; 32]),
        price_vector_cid: Cid([2u8; 32]),
        abstracted_failure_features_cid: Cid([3u8; 32]),
        total_pulls_target_before: total,
        candidates: sel.rows,
        selected_model_id: selected_override
            .map(|s| s.to_string())
            .unwrap_or(sel.selected_model_id),
        selection_reason: sel.reason,
        allocated_proposal_budget: 1,
        allocated_token_budget: 900,
        budget_remaining_before: 5_000,
        budget_remaining_after: 4_100,
        router_overhead_cid: Cid([4u8; 32]),
        rng_seed: None,
        rng_draw: None,
    }
}

#[test]
fn every_allocation_attributes_to_a_scored_candidate() {
    let tape: Vec<BudgetAllocationTelemetry> = (0..5).map(|i| record(i, None)).collect();
    let total = tape.len();
    let attributed_count = tape.iter().filter(|r| attributed(r)).count();
    assert_eq!(
        attributed_count, total,
        "{}/{} allocations are untraced (selected model not among scored candidates) — \
         a ghost allocation would enter the primary metric without a routing basis",
        total - attributed_count,
        total
    );
}

/// FAILABILITY: a selected model absent from the scored candidates must be flagged untraced.
#[test]
fn a_ghost_allocation_is_caught() {
    let ghost = record(99, Some("phantom-model-not-scored"));
    assert!(
        !attributed(&ghost),
        "completeness check accepted a ghost allocation (selected model not in candidates) — \
         GA-6 would not catch an untraced allocation"
    );
}
