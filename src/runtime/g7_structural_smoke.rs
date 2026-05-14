//! TB-G G7 — structural run6-equivalent smoke evaluator.

/// TRACE_MATRIX FC3-N43: public G7 minimum-tier input witness for structural
/// smoke reporting; this is a report contract, not a new runtime authority.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct G7SmokeInput {
    pub one_runtime_repo: bool,
    pub multi_agent: bool,
    pub persistent_state: bool,
    pub proof_related_actions: u64,
    pub market_visible_actions: u64,
    pub no_trade_reason_count: u64,
    pub role_classifier_output: bool,
    pub price_observe_only: bool,
    pub no_price_as_truth: bool,
    pub dashboard_regenerated: bool,
}

/// TRACE_MATRIX FC3-N43: public §K structural-smoke result used to distinguish
/// minimum-tier GREEN, clean-negative, and forward-stub-required outcomes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G7SmokeReport {
    pub minimum_tier_green: bool,
    pub clean_negative: bool,
    pub forward_tb_stub_required: bool,
    pub input: G7SmokeInput,
}

/// TRACE_MATRIX FC3-N43: evaluate G7 minimum structural evidence without
/// claiming v3 run6 volume, emergent roles, model ranking, or market quality.
pub fn evaluate_g7_structural_smoke(input: G7SmokeInput) -> G7SmokeReport {
    let market_or_clean_negative =
        input.market_visible_actions > 0 || input.no_trade_reason_count > 0;
    let minimum_tier_green = input.one_runtime_repo
        && input.multi_agent
        && input.persistent_state
        && input.proof_related_actions > 0
        && market_or_clean_negative
        && input.role_classifier_output
        && input.price_observe_only
        && input.no_price_as_truth
        && input.dashboard_regenerated;
    let clean_negative = input.market_visible_actions == 0 && input.no_trade_reason_count > 0;
    G7SmokeReport {
        minimum_tier_green,
        clean_negative,
        forward_tb_stub_required: !minimum_tier_green,
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
