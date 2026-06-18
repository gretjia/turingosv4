//! Gate 5.4 (H-HET-2 charter §5.4 + routing-policy ruling 2026-06-15): the dynamic
//! model-budget routing DECISION must be tape-canonical (Art 0.2). The treatment is HOW
//! budget is routed, so a replayer must rebuild each allocation from the frozen tape:
//! `allocation_view == derive_from_tape(tape)`.
//!
//! This gate is failable: it (1) round-trips BudgetAllocationTelemetry through CAS,
//! (2) RECOMPUTES the selection from the tape-recorded candidate rows via the SAME generic
//! `routing_policy::score_and_select` mechanism and asserts it byte-matches the recorded
//! selection (§17.1-G1 recompute-from-tape, not a sidecar read), (3) proves a TAMPERED
//! record's recompute diverges (the gate can catch a lying tape), (4) confirms the policy
//! DISTRIBUTES (not argmax-collapse, §17.3), and (5) pins the frozen-policy provenance.

use tempfile::TempDir;
use turingosv4::bottom_white::cas::schema::Cid;
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::runtime::budget_allocation_telemetry::{
    self as bat, BudgetAllocationTelemetry,
};
use turingosv4::runtime::routing_policy::{
    self, ModelInput, RoutingPolicyConfig, RoutingPolicyGenesisPin,
};
use turingosv4::state::q_state::Hash;

/// Reconstruct the generic ModelInputs from a tape-recorded record's candidate rows.
/// price_prior_bps = the row's (already-clamped) price_bps: re-clamping is idempotent and the
/// cold-start gate keys off pull/verify which are on the row, so the recompute is exact.
fn inputs_from_record(rec: &BudgetAllocationTelemetry) -> Vec<ModelInput> {
    rec.candidates
        .iter()
        .map(|r| ModelInput {
            model_id: r.model_id.clone(),
            pull_count: r.pull_count_model_target_before,
            verify_count: r.verify_count_model_target_before,
            hard_failure_streak: r.hard_failure_streak_before,
            price_prior_bps: r.price_bps,
            floor_quota_remaining: r.floor_quota_remaining_before,
        })
        .collect()
}

/// Build a real BudgetAllocationTelemetry by running the actual mechanism, so its rows are
/// internally consistent with score_and_select (as the carrier emits them).
fn make_record(
    cfg: &RoutingPolicyConfig,
    inputs: &[ModelInput],
    remaining: u64,
) -> BudgetAllocationTelemetry {
    let sel = routing_policy::score_and_select(cfg, inputs, remaining);
    let total: u32 = sel
        .rows
        .iter()
        .map(|r| r.pull_count_model_target_before)
        .sum();
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
        allocated_token_budget: 900,
        budget_remaining_before: remaining,
        budget_remaining_after: remaining.saturating_sub(1),
        router_overhead_cid: Cid([4u8; 32]),
        rng_seed: None,
        rng_draw: None,
    }
}

fn sample_inputs() -> Vec<ModelInput> {
    vec![
        ModelInput { model_id: "deepseek".into(), pull_count: 2, verify_count: 0, hard_failure_streak: 2, price_prior_bps: 0, floor_quota_remaining: 0 },
        ModelInput { model_id: "qwen32".into(), pull_count: 1, verify_count: 0, hard_failure_streak: 1, price_prior_bps: 3000, floor_quota_remaining: 1 },
        ModelInput { model_id: "glm".into(), pull_count: 1, verify_count: 0, hard_failure_streak: 0, price_prior_bps: 0, floor_quota_remaining: 1 },
        ModelInput { model_id: "qwen397".into(), pull_count: 3, verify_count: 2, hard_failure_streak: 0, price_prior_bps: 0, floor_quota_remaining: 0 },
    ]
}

/// (1) tape-canonical storage: round-trip through CAS is byte-identical.
#[test]
fn budget_allocation_round_trips_through_cas() {
    let dir = TempDir::new().unwrap();
    let mut cas = CasStore::open(dir.path()).unwrap();
    let cfg = RoutingPolicyConfig::default();
    let rec = make_record(&cfg, &sample_inputs(), 10);
    let cid = bat::write_to_cas(&mut cas, &rec, "gate54", 1).unwrap();
    assert_eq!(bat::read_from_cas(&cas, &cid).unwrap(), rec);
}

/// (2) §17.1-G1 recompute-from-tape: the allocation recomputes from the tape-recorded rows
/// via the same mechanism — allocation_view == derive_from_tape(tape).
#[test]
fn allocation_recomputes_from_tape() {
    let dir = TempDir::new().unwrap();
    let mut cas = CasStore::open(dir.path()).unwrap();
    let cfg = RoutingPolicyConfig::default();
    let rec = make_record(&cfg, &sample_inputs(), 10);
    let cid = bat::write_to_cas(&mut cas, &rec, "gate54", 1).unwrap();
    let loaded = bat::read_from_cas(&cas, &cid).unwrap();

    let recomputed =
        routing_policy::score_and_select(&cfg, &inputs_from_record(&loaded), loaded.budget_remaining_before);
    assert_eq!(
        recomputed.selected_model_id, loaded.selected_model_id,
        "Art 0.2/§17.1-G1: selected model must recompute from the frozen tape rows"
    );
    assert_eq!(
        recomputed.reason, loaded.selection_reason,
        "selection reason must recompute from the tape"
    );
    for (rc, rec_row) in recomputed.rows.iter().zip(loaded.candidates.iter()) {
        assert_eq!(rc.model_id, rec_row.model_id);
        assert_eq!(
            rc.score_bps, rec_row.score_bps,
            "per-model score_bps for {} must recompute byte-equal",
            rc.model_id
        );
    }
    // tape-internal consistency: header total == Σ candidate pulls.
    assert_eq!(loaded.candidate_pull_sum(), loaded.total_pulls_target_before);
}

/// (3) the gate CAN fail: a tampered selected_model_id no longer matches the recompute.
#[test]
fn tampered_selection_is_caught_by_recompute() {
    let cfg = RoutingPolicyConfig::default();
    let mut rec = make_record(&cfg, &sample_inputs(), 10);
    // Flip the recorded winner to some other candidate.
    let other = rec
        .candidates
        .iter()
        .map(|r| r.model_id.clone())
        .find(|m| *m != rec.selected_model_id)
        .unwrap();
    rec.selected_model_id = other;
    let recomputed = routing_policy::score_and_select(&cfg, &inputs_from_record(&rec), rec.budget_remaining_before);
    assert_ne!(
        recomputed.selected_model_id, rec.selected_model_id,
        "recompute MUST diverge from a tampered selection — else the gate is blind"
    );
}

/// (4) §17.3: the named mechanism (UCB + floor) DISTRIBUTES, not argmax-collapse — over equal
/// fresh arms with floor quotas, every model is funded across the budget.
#[test]
fn policy_distributes_not_argmax_collapse() {
    let cfg = RoutingPolicyConfig::default();
    let ids = ["a", "b", "c", "d"];
    let mut floors = [1u64; 4];
    let mut funded = std::collections::BTreeSet::new();
    for tick in 0..4u64 {
        let inputs: Vec<ModelInput> = (0..4)
            .map(|i| ModelInput {
                model_id: ids[i].into(),
                pull_count: 0,
                verify_count: 0,
                hard_failure_streak: 0,
                price_prior_bps: 0,
                floor_quota_remaining: floors[i],
            })
            .collect();
        let sel = routing_policy::score_and_select(&cfg, &inputs, 4 - tick);
        let wi = ids.iter().position(|&x| x == sel.selected_model_id).unwrap();
        if floors[wi] > 0 {
            floors[wi] -= 1;
        }
        funded.insert(sel.selected_model_id);
    }
    assert_eq!(funded.len(), 4, "argmax-collapse: not all models funded");
}

/// (5) frozen-policy provenance: the GenesisPin round-trips and its policy_hash == config hash.
#[test]
fn genesis_pin_pins_the_frozen_policy() {
    let dir = TempDir::new().unwrap();
    let mut cas = CasStore::open(dir.path()).unwrap();
    let cfg = RoutingPolicyConfig::default();
    let cfg_cid = routing_policy::write_policy_config_to_cas(&mut cas, &cfg, "gate54", 1).unwrap();
    let pin = RoutingPolicyGenesisPin {
        policy_family: cfg.policy_family.clone(),
        policy_version: cfg.policy_version.clone(),
        policy_hash: cfg.policy_hash(),
        canonical_policy_config_cid: cfg_cid,
        eligible_model_set_hash: Hash([1u8; 32]),
        target_pool_hash: Hash([2u8; 32]),
        budget_caps_hash: Hash([3u8; 32]),
        rng_mode: "deterministic_none".into(),
        art_0_4_path: "B".into(),
    };
    let cid = routing_policy::write_genesis_pin_to_cas(&mut cas, &pin, "gate54", 2).unwrap();
    let loaded = routing_policy::read_genesis_pin_from_cas(&cas, &cid).unwrap();
    assert_eq!(loaded, pin);
    assert_eq!(loaded.policy_hash, cfg.policy_hash(), "pin must bind the frozen config hash");
    assert_eq!(loaded.rng_mode, "deterministic_none", "v1 is deterministic (no RNG)");
    assert_eq!(loaded.art_0_4_path, "B", "Art 0.4 Path-B declared");
}
