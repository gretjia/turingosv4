//! REAL-14G — PositiveEVIgnored action-conversion summary.
//!
//! This is a CAS-derived materialized view over `PolicyTraderTrace` and
//! `EVDecisionTrace`. It explains counterfactual positive-EV opportunities
//! that live Bull/Bear traders voluntarily abstained from. It does not force
//! trades, count PolicyTrader as E2, or read dashboard text as truth.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::bottom_white::cas::store::{CasError, CasStore};
use crate::runtime::ev_decision_trace::{
    read_ev_decision_trace_from_cas, EVAction, EVDecisionTrace, EVReason,
};
use crate::runtime::policy_trader_trace::{
    policy_trader_trace_cids, read_policy_trader_trace_from_cas, PolicyTraderComparison,
    PolicyTraderTrace,
};

/// TRACE_MATRIX FC1/FC3: bounded mechanism taxonomy for positive-EV abstains.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum PositiveEVIgnoredBucket {
    InsufficientSalience,
    RiskFramingTooConservative,
    PnLFeedbackAbsent,
    BCASTDigestAbsentOrWeak,
    RoleInstructionAmbiguous,
    ParserOrSchemaFriction,
    LiquidityOrSlippageConcern,
    BalanceOrRiskCapConcern,
    ModelAbstentionDespiteClearBasis,
    Unknown,
}

impl PositiveEVIgnoredBucket {
    /// TRACE_MATRIX FC3: exhaustive bucket list for zero-count dashboard rows.
    pub const fn all() -> [Self; 10] {
        [
            Self::InsufficientSalience,
            Self::RiskFramingTooConservative,
            Self::PnLFeedbackAbsent,
            Self::BCASTDigestAbsentOrWeak,
            Self::RoleInstructionAmbiguous,
            Self::ParserOrSchemaFriction,
            Self::LiquidityOrSlippageConcern,
            Self::BalanceOrRiskCapConcern,
            Self::ModelAbstentionDespiteClearBasis,
            Self::Unknown,
        ]
    }
}

/// TRACE_MATRIX FC3: one reconstructed positive-EV abstain row, sourced from
/// a PolicyTraderTrace pointing at the source EVDecisionTrace CID.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PositiveEVIgnoredRow {
    pub source_policy_trader_trace_cid: String,
    pub source_ev_decision_trace_cid: String,
    pub prompt_capsule_cid: String,
    pub market_snapshot_cid: String,
    pub agent_id: String,
    pub role: String,
    pub side: String,
    pub task_id: String,
    pub event_id: String,
    pub action: String,
    pub ev_reason: String,
    pub quoted_price_num: Option<u64>,
    pub quoted_price_den: Option<u64>,
    pub implied_probability_bps: Option<i64>,
    pub policy_probability_bps: Option<i64>,
    pub policy_edge_bps: Option<i64>,
    pub policy_expected_value_micro: Option<i128>,
    pub amount_micro: Option<i64>,
    pub liquidity_depth_micro: Option<i64>,
    pub available_balance_micro: i64,
    pub risk_cap_micro: i64,
    pub risk_cap_triggered: bool,
    pub gateway_blocked: bool,
    pub bucket: PositiveEVIgnoredBucket,
}

/// TRACE_MATRIX FC3: aggregate action-conversion view over positive-EV policy
/// opportunities. PolicyTrader remains counterfactual and excluded from E2.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PositiveEVIgnoredSummary {
    pub policy_positive_ev_count: u64,
    pub executed_positive_ev_count: u64,
    pub ignored_count: u64,
    pub action_conversion_rate_bps: u64,
    pub unknown_count: u64,
    pub by_bucket: BTreeMap<PositiveEVIgnoredBucket, u64>,
    pub rows: Vec<PositiveEVIgnoredRow>,
}

impl PositiveEVIgnoredSummary {
    fn empty() -> Self {
        Self {
            policy_positive_ev_count: 0,
            executed_positive_ev_count: 0,
            ignored_count: 0,
            action_conversion_rate_bps: 0,
            unknown_count: 0,
            by_bucket: PositiveEVIgnoredBucket::all()
                .into_iter()
                .map(|bucket| (bucket, 0))
                .collect(),
            rows: Vec::new(),
        }
    }
}

/// TRACE_MATRIX FC3: reconstruct positive-EV abstains from CAS sidecars. Any
/// missing source EV trace fails closed because otherwise the mechanism report
/// would silently become dashboard-only inference.
pub fn summarize_positive_ev_ignored_from_cas(
    cas: &CasStore,
) -> Result<PositiveEVIgnoredSummary, CasError> {
    let mut summary = PositiveEVIgnoredSummary::empty();
    for policy_cid in policy_trader_trace_cids(cas) {
        let policy = read_policy_trader_trace_from_cas(cas, &policy_cid)?;
        if policy.policy_edge_bps.map(|edge| edge > 0).unwrap_or(false) {
            summary.policy_positive_ev_count += 1;
        }
        match policy.comparison {
            PolicyTraderComparison::BothBuy => {
                summary.executed_positive_ev_count += 1;
            }
            PolicyTraderComparison::PolicyPositiveEV_LLMAbstained => {
                let ev_trace =
                    read_ev_decision_trace_from_cas(cas, &policy.source_ev_decision_trace_cid)
                        .map_err(|e| {
                            CasError::BackendCorruption(format!(
                                "PositiveEVIgnored source EVDecisionTrace {} unreadable: {e}",
                                policy.source_ev_decision_trace_cid
                            ))
                        })?;
                let row =
                    build_positive_ev_ignored_row(&policy_cid.to_string(), &policy, &ev_trace);
                *summary.by_bucket.entry(row.bucket).or_insert(0) += 1;
                if row.bucket == PositiveEVIgnoredBucket::Unknown {
                    summary.unknown_count += 1;
                }
                summary.rows.push(row);
            }
            PolicyTraderComparison::PolicyNoPositiveEV
            | PolicyTraderComparison::LLMBuyPolicyNoBuy
            | PolicyTraderComparison::GatewayBlocked
            | PolicyTraderComparison::InsufficientPublicEVBasis => {}
        }
    }
    summary.ignored_count = summary.rows.len() as u64;
    if summary.policy_positive_ev_count > 0 {
        summary.action_conversion_rate_bps =
            summary.executed_positive_ev_count.saturating_mul(10_000)
                / summary.policy_positive_ev_count;
    }
    Ok(summary)
}

fn build_positive_ev_ignored_row(
    policy_cid: &str,
    policy: &PolicyTraderTrace,
    ev: &EVDecisionTrace,
) -> PositiveEVIgnoredRow {
    let bucket = classify_positive_ev_ignored(policy, ev);
    let quoted_price_num = ev.quoted_price.map(|price| price.numerator);
    let quoted_price_den = ev.quoted_price.map(|price| price.denominator);
    PositiveEVIgnoredRow {
        source_policy_trader_trace_cid: policy_cid.to_string(),
        source_ev_decision_trace_cid: policy.source_ev_decision_trace_cid.to_string(),
        prompt_capsule_cid: policy.prompt_capsule_cid.to_string(),
        market_snapshot_cid: policy.market_snapshot_cid.to_string(),
        agent_id: ev.agent_id.0.clone(),
        role: ev.role.label().to_string(),
        side: format!("{:?}", ev.side),
        task_id: ev.task_id.0.clone(),
        event_id: ev.event_id.0 .0.clone(),
        action: format!("{:?}", ev.action),
        ev_reason: format!("{:?}", ev.reason),
        quoted_price_num,
        quoted_price_den,
        implied_probability_bps: policy.implied_probability_bps,
        policy_probability_bps: policy.policy_probability_bps,
        policy_edge_bps: policy.policy_edge_bps,
        policy_expected_value_micro: policy.policy_expected_value_micro,
        amount_micro: ev.amount.map(|amount| amount.micro_units()),
        liquidity_depth_micro: ev.liquidity_depth.map(|depth| depth.micro_units()),
        available_balance_micro: ev.available_balance.micro_units(),
        risk_cap_micro: ev.risk_cap.micro_units(),
        risk_cap_triggered: ev.risk_cap_triggered,
        gateway_blocked: policy.gateway_blocked,
        bucket,
    }
}

fn classify_positive_ev_ignored(
    policy: &PolicyTraderTrace,
    ev: &EVDecisionTrace,
) -> PositiveEVIgnoredBucket {
    let amount_micro = ev.amount.map(|amount| amount.micro_units()).unwrap_or(0);
    let liquidity_depth_micro = ev
        .liquidity_depth
        .map(|depth| depth.micro_units())
        .unwrap_or(0);
    if policy.gateway_blocked || ev.risk_cap_triggered {
        if amount_micro <= 0 {
            return PositiveEVIgnoredBucket::ParserOrSchemaFriction;
        }
        if ev.available_balance.micro_units() < amount_micro
            || ev.risk_cap.micro_units() < amount_micro
            || ev.risk_cap_triggered
        {
            return PositiveEVIgnoredBucket::BalanceOrRiskCapConcern;
        }
        if liquidity_depth_micro < amount_micro {
            return PositiveEVIgnoredBucket::LiquidityOrSlippageConcern;
        }
        return PositiveEVIgnoredBucket::ParserOrSchemaFriction;
    }
    match ev.reason {
        EVReason::ParserOrGatewayFailed => PositiveEVIgnoredBucket::ParserOrSchemaFriction,
        EVReason::BalanceBlocked | EVReason::RiskCapBlocked => {
            PositiveEVIgnoredBucket::BalanceOrRiskCapConcern
        }
        EVReason::LiquidityTooLow | EVReason::SlippageTooHigh => {
            PositiveEVIgnoredBucket::LiquidityOrSlippageConcern
        }
        EVReason::InsufficientConfidence | EVReason::ProbabilityUncalibrated => {
            PositiveEVIgnoredBucket::RiskFramingTooConservative
        }
        EVReason::PositiveEVIgnored
            if ev.action == EVAction::Abstain
                && ev.policy_basis_available()
                && policy.policy_edge_bps.map(|edge| edge > 0).unwrap_or(false)
                && policy
                    .policy_expected_value_micro
                    .map(|ev| ev > 0)
                    .unwrap_or(false) =>
        {
            PositiveEVIgnoredBucket::ModelAbstentionDespiteClearBasis
        }
        _ => PositiveEVIgnoredBucket::Unknown,
    }
}
