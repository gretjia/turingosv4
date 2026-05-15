//! TB-G G7 — structural run6-equivalent smoke evaluator.

use serde::{Deserialize, Serialize};

use crate::bottom_white::cas::schema::{Cid, ObjectType};
use crate::bottom_white::cas::store::{CasError, CasStore};
use crate::state::q_state::TaskId;

pub const G7_STRUCTURAL_GUARD_SCHEMA_ID: &str = "real7.structural_smoke_guard.v1";

/// TRACE_MATRIX FC3-N43: public G7 minimum-tier input witness for structural
/// smoke reporting; this is a report contract, not a new runtime authority.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct G7SmokeInput {
    pub one_runtime_repo: bool,
    pub multi_agent: bool,
    pub persistent_state: bool,
    pub agent_count: u64,
    pub active_role_count: u64,
    pub task_count: u64,
    pub task_outcome_market_count: u64,
    pub scripted_attempt_prediction_market_count: u64,
    pub buy_yes_router_count: u64,
    pub buy_no_or_short_count: u64,
    pub verify_tx_count: u64,
    pub challenge_tx_or_no_challenge_reason_count: u64,
    pub event_resolve_count: u64,
    pub pnl_delta_count: u64,
    pub loss_occurred: bool,
    pub autopsy_capsule_count: u64,
    pub forced_live_investment: bool,
    pub market_actions_chain_visible: bool,
    pub no_ghost_liquidity: bool,
    pub clean_v3_comparison: bool,
    pub proof_related_actions: u64,
    pub market_visible_actions: u64,
    pub no_trade_reason_count: u64,
    pub role_classifier_output: bool,
    pub price_observe_only: bool,
    pub no_price_as_truth: bool,
    pub dashboard_regenerated: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct G7StructuralGuard {
    pub schema_version: String,
    pub task_id: TaskId,
    pub no_forced_live_investment: bool,
    pub price_observe_only: bool,
    pub no_price_as_truth: bool,
    pub no_ghost_liquidity: bool,
    pub clean_v3_comparison: bool,
    pub public_summary: String,
}

impl G7StructuralGuard {
    pub fn structural_fixture(task_id: TaskId) -> Self {
        Self {
            schema_version: G7_STRUCTURAL_GUARD_SCHEMA_ID.to_string(),
            task_id,
            no_forced_live_investment: true,
            price_observe_only: true,
            no_price_as_truth: true,
            no_ghost_liquidity: true,
            clean_v3_comparison: true,
            public_summary: "REAL-7 structural fixture: scripted market pressure only; no forced live LLM investment; price remains observe-only signal, not Lean predicate truth; compare to v3 structure without claiming identical equivalence".into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G7StructuralGuardSummary {
    pub total_guards: u64,
    pub no_forced_live_investment_all: bool,
    pub price_observe_only_all: bool,
    pub no_price_as_truth_all: bool,
    pub no_ghost_liquidity_all: bool,
    pub clean_v3_comparison_all: bool,
}

pub fn write_g7_structural_guard_to_cas(
    cas: &mut CasStore,
    guard: &G7StructuralGuard,
    suffix: &str,
    logical_t: u64,
) -> Result<Cid, CasError> {
    let bytes = serde_json::to_vec(guard)
        .map_err(|e| CasError::BackendCorruption(format!("g7 guard encode: {e}")))?;
    cas.put(
        &bytes,
        ObjectType::Generic,
        &format!("real7-structural-guard-{suffix}"),
        logical_t,
        Some(G7_STRUCTURAL_GUARD_SCHEMA_ID.to_string()),
    )
}

pub fn summarize_g7_structural_guards_from_cas(cas: &CasStore) -> G7StructuralGuardSummary {
    let mut guards = Vec::new();
    for cid in cas.list_all_cids() {
        if cas.metadata(&cid).and_then(|m| m.schema_id.as_deref())
            != Some(G7_STRUCTURAL_GUARD_SCHEMA_ID)
        {
            continue;
        }
        let Ok(bytes) = cas.get(&cid) else {
            continue;
        };
        let Ok(guard) = serde_json::from_slice::<G7StructuralGuard>(&bytes) else {
            continue;
        };
        if guard.schema_version == G7_STRUCTURAL_GUARD_SCHEMA_ID {
            guards.push(guard);
        }
    }
    let total_guards = guards.len() as u64;
    let any = total_guards > 0;
    G7StructuralGuardSummary {
        total_guards,
        no_forced_live_investment_all: any && guards.iter().all(|g| g.no_forced_live_investment),
        price_observe_only_all: any && guards.iter().all(|g| g.price_observe_only),
        no_price_as_truth_all: any && guards.iter().all(|g| g.no_price_as_truth),
        no_ghost_liquidity_all: any && guards.iter().all(|g| g.no_ghost_liquidity),
        clean_v3_comparison_all: any && guards.iter().all(|g| g.clean_v3_comparison),
    }
}

/// TRACE_MATRIX FC3-N43: public §K structural-smoke result used to distinguish
/// minimum-tier GREEN, clean-negative, and forward-stub-required outcomes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G7SmokeReport {
    pub minimum_tier_green: bool,
    pub clean_negative: bool,
    pub forward_tb_stub_required: bool,
    pub autopsy_if_loss_satisfied: bool,
    pub does_not_claim_identical_v3_equivalence: bool,
    pub input: G7SmokeInput,
}

/// TRACE_MATRIX FC3-N43: evaluate G7 minimum structural evidence without
/// claiming v3 run6 volume, emergent roles, model ranking, or market quality.
pub fn evaluate_g7_structural_smoke(input: G7SmokeInput) -> G7SmokeReport {
    let market_buy_shape = input.buy_yes_router_count > 0
        && input.buy_no_or_short_count > 0
        && input.pnl_delta_count > 0;
    let clean_negative = input.buy_yes_router_count == 0
        && input.buy_no_or_short_count == 0
        && input.no_trade_reason_count > 0;
    let market_or_clean_negative = market_buy_shape || clean_negative;
    let autopsy_if_loss_satisfied = !input.loss_occurred || input.autopsy_capsule_count > 0;
    let does_not_claim_identical_v3_equivalence = input.clean_v3_comparison;
    let minimum_tier_green = input.one_runtime_repo
        && input.multi_agent
        && input.persistent_state
        && input.agent_count >= 5
        && input.active_role_count >= 3
        && input.task_count >= 3
        && input.task_outcome_market_count > 0
        && input.scripted_attempt_prediction_market_count > 0
        && input.proof_related_actions > 0
        && input.verify_tx_count > 0
        && input.challenge_tx_or_no_challenge_reason_count > 0
        && input.event_resolve_count > 0
        && market_or_clean_negative
        && input.role_classifier_output
        && input.price_observe_only
        && input.no_price_as_truth
        && input.dashboard_regenerated
        && !input.forced_live_investment
        && input.market_actions_chain_visible
        && input.no_ghost_liquidity
        && autopsy_if_loss_satisfied
        && does_not_claim_identical_v3_equivalence;
    G7SmokeReport {
        minimum_tier_green,
        clean_negative,
        forward_tb_stub_required: !minimum_tier_green,
        autopsy_if_loss_satisfied,
        does_not_claim_identical_v3_equivalence,
        input,
    }
}

impl G7SmokeReport {
    /// TRACE_MATRIX FC3-N43: render §K as a dashboard materialized view with
    /// explicit clean-negative and forward-stub flags.
    pub fn render_section_k(&self) -> String {
        let mut out = String::new();
        out.push_str("\n## §K G7 structural smoke\n");
        out.push_str(&format!(
            "  minimum_tier_green: {}\n",
            self.minimum_tier_green
        ));
        out.push_str(&format!("  clean_negative: {}\n", self.clean_negative));
        out.push_str(&format!(
            "  forward_tb_stub_required: {}\n",
            self.forward_tb_stub_required
        ));
        out.push_str(&format!(
            "  one_runtime_repo: {}\n",
            self.input.one_runtime_repo
        ));
        out.push_str(&format!("  multi_agent: {}\n", self.input.multi_agent));
        out.push_str(&format!(
            "  persistent_state: {}\n",
            self.input.persistent_state
        ));
        out.push_str(&format!("  agent_count: {}\n", self.input.agent_count));
        out.push_str(&format!(
            "  active_role_count: {}\n",
            self.input.active_role_count
        ));
        out.push_str(&format!("  task_count: {}\n", self.input.task_count));
        out.push_str(&format!(
            "  task_outcome_market_count: {}\n",
            self.input.task_outcome_market_count
        ));
        out.push_str(&format!(
            "  scripted_attempt_prediction_market_count: {}\n",
            self.input.scripted_attempt_prediction_market_count
        ));
        out.push_str(&format!(
            "  buy_yes_router_count: {}\n",
            self.input.buy_yes_router_count
        ));
        out.push_str(&format!(
            "  buy_no_or_short_count: {}\n",
            self.input.buy_no_or_short_count
        ));
        out.push_str(&format!(
            "  verify_tx_count: {}\n",
            self.input.verify_tx_count
        ));
        out.push_str(&format!(
            "  challenge_tx_or_no_challenge_reason_count: {}\n",
            self.input.challenge_tx_or_no_challenge_reason_count
        ));
        out.push_str(&format!(
            "  event_resolve_count: {}\n",
            self.input.event_resolve_count
        ));
        out.push_str(&format!(
            "  pnl_delta_count: {}\n",
            self.input.pnl_delta_count
        ));
        out.push_str(&format!("  loss_occurred: {}\n", self.input.loss_occurred));
        out.push_str(&format!(
            "  autopsy_capsule_count: {}\n",
            self.input.autopsy_capsule_count
        ));
        out.push_str(&format!(
            "  autopsy_if_loss_satisfied: {}\n",
            self.autopsy_if_loss_satisfied
        ));
        out.push_str(&format!(
            "  no_forced_live_investment: {}\n",
            !self.input.forced_live_investment
        ));
        out.push_str(&format!(
            "  market_actions_chain_visible: {}\n",
            self.input.market_actions_chain_visible
        ));
        out.push_str(&format!(
            "  no_ghost_liquidity: {}\n",
            self.input.no_ghost_liquidity
        ));
        out.push_str(&format!(
            "  clean_v3_comparison: {}\n",
            self.input.clean_v3_comparison
        ));
        out.push_str(&format!(
            "  does_not_claim_identical_v3_equivalence: {}\n",
            self.does_not_claim_identical_v3_equivalence
        ));
        out.push_str(&format!(
            "  proof_related_actions: {}\n",
            self.input.proof_related_actions
        ));
        out.push_str(&format!(
            "  market_visible_actions: {}\n",
            self.input.market_visible_actions
        ));
        out.push_str(&format!(
            "  no_trade_reason_count: {}\n",
            self.input.no_trade_reason_count
        ));
        if self.clean_negative || self.forward_tb_stub_required {
            out.push_str("  MECHANISM BOTTLENECK:\n");
            out.push_str("  - agents may not have perceived a profitable market edge\n");
            out.push_str(
                "  - peer verification or market opportunities may have appeared too late\n",
            );
            out.push_str("  - scheduler ordering or prompt budget may have suppressed differentiated behavior\n");
        }
        out
    }
}
