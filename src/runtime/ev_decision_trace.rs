//! REAL-13A — CAS-backed expected-value decision traces.
//!
//! `EVDecisionTrace` is a public, typed market-review sidecar. It is not a
//! typed transaction and does not modify sequencer admission; missing traces
//! make REAL-13 evidence invalid, not L4 admission invalid.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::bottom_white::cas::schema::{Cid, ObjectType};
use crate::bottom_white::cas::store::{CasError, CasStore};
use crate::economy::money::MicroCoin;
use crate::runtime::real5_roles::{AgentRole, MarketSide, RationalPrice};
use crate::state::q_state::{AgentId, TaskId};
use crate::state::typed_tx::EventId;

/// TRACE_MATRIX FC1/FC3: schema tag for CAS-backed externalized economic
/// judgment evidence rendered by dashboards as a materialized view.
pub const EV_DECISION_TRACE_SCHEMA_ID: &str = "real13a.ev_decision_trace.v1";

/// TRACE_MATRIX FC1: typed Bull/Bear market-review action emitted before any
/// optional router transaction; buy/short remains voluntary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum EVAction {
    BuyYes,
    BuyNo,
    Abstain,
}

impl EVAction {
    /// TRACE_MATRIX FC1/FC3: exhaustive action enumerator used by dashboard and
    /// verifier summaries so buy/abstain categories cannot silently drift.
    pub const fn all() -> [Self; 3] {
        [Self::BuyYes, Self::BuyNo, Self::Abstain]
    }
}

/// TRACE_MATRIX FC1/FC3: structured explanation for Buy/Short/Abstain so
/// no-trade outcomes are audit-visible instead of stdout-only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum EVReason {
    PositiveEV,
    NegativeEV,
    EdgeBelowThreshold,
    RiskCapBlocked,
    BalanceBlocked,
    LiquidityTooLow,
    SlippageTooHigh,
    ParserOrGatewayFailed,
    WindowClosed,
    PositiveEVIgnored,
    InsufficientConfidence,
    ProbabilityUncalibrated,
    NoActionableMarket,
    Unknown,
}

impl EVReason {
    /// TRACE_MATRIX FC1/FC3: exhaustive reason enumerator for EV/no-trade
    /// dashboard summaries and fail-closed taxonomy checks.
    pub const fn all() -> [Self; 14] {
        [
            Self::PositiveEV,
            Self::NegativeEV,
            Self::EdgeBelowThreshold,
            Self::RiskCapBlocked,
            Self::BalanceBlocked,
            Self::LiquidityTooLow,
            Self::SlippageTooHigh,
            Self::ParserOrGatewayFailed,
            Self::WindowClosed,
            Self::PositiveEVIgnored,
            Self::InsufficientConfidence,
            Self::ProbabilityUncalibrated,
            Self::NoActionableMarket,
            Self::Unknown,
        ]
    }
}

/// TRACE_MATRIX FC1/FC3: CAS-backed expected-value decision fossil for a
/// BullTrader/BearTrader market review turn; contains public fields only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EVDecisionTrace {
    pub schema_version: String,
    pub review_window_id: String,
    pub review_response_id: String,
    pub run_id: String,
    pub batch_id: String,
    pub agent_id: AgentId,
    pub role: AgentRole,
    pub task_id: TaskId,
    pub event_id: EventId,
    pub side: MarketSide,
    #[serde(default)]
    pub quoted_price: Option<RationalPrice>,
    #[serde(default)]
    pub implied_probability_bps: Option<i64>,
    #[serde(default)]
    pub agent_probability_bps: Option<i64>,
    #[serde(default)]
    pub edge_bps: Option<i64>,
    #[serde(default)]
    pub expected_value_micro: Option<i128>,
    #[serde(default)]
    pub amount: Option<MicroCoin>,
    pub max_risk: MicroCoin,
    pub available_balance: MicroCoin,
    pub risk_cap: MicroCoin,
    #[serde(default)]
    pub liquidity_depth: Option<MicroCoin>,
    #[serde(default)]
    pub slippage_bps: Option<i64>,
    pub risk_cap_triggered: bool,
    pub action: EVAction,
    pub reason: EVReason,
    pub prompt_capsule_cid: Cid,
    pub market_snapshot_cid: Cid,
    pub model_assignment_cid: Option<Cid>,
    pub model_family: Option<String>,
    pub private_alpha_cid: Option<Cid>,
    pub tool_result_cid: Option<Cid>,
    pub parent_state_root: String,
    pub created_at_head_t: String,
    pub public_summary: String,
}

impl EVDecisionTrace {
    /// TRACE_MATRIX FC3: complete public EV basis exists when all public
    /// basis fields are present.
    /// TRACE_MATRIX FC3: actionable public EV basis exists when all public
    /// price/probability/amount/liquidity fields are present and the public
    /// amount/depth are non-placeholder values. Missing or zero basis is
    /// evidence, not an instruction to fabricate 50/50 or zero-liquidity data.
    pub fn policy_basis_available(&self) -> bool {
        let amount_micro = self.amount.map(|amount| amount.micro_units());
        let liquidity_depth_micro = self.liquidity_depth.map(|depth| depth.micro_units());
        self.quoted_price.is_some()
            && self.implied_probability_bps.is_some()
            && self.agent_probability_bps.is_some()
            && self.edge_bps.is_some()
            && self.expected_value_micro.is_some()
            && amount_micro.map(|amount| amount > 0).unwrap_or(false)
            && liquidity_depth_micro
                .zip(amount_micro)
                .map(|(depth, amount)| depth >= amount)
                .unwrap_or(false)
    }
}

/// TRACE_MATRIX FC1/FC3: public positive-EV predicate used to diagnose an
/// abstain without forcing a router action. It depends only on typed public EV
/// basis and integer risk constraints, never on private rationale or self-
/// declared EV sign.
pub fn public_positive_ev_constraints_pass(
    edge_bps: Option<i64>,
    expected_value_micro: Option<i128>,
    amount: Option<MicroCoin>,
    available_balance: MicroCoin,
    risk_cap: MicroCoin,
    liquidity_depth: Option<MicroCoin>,
    risk_cap_triggered: bool,
    threshold_bps: i64,
) -> bool {
    let threshold_bps = threshold_bps.clamp(0, 10_000);
    let amount_micro = match amount {
        Some(amount) => amount.micro_units(),
        None => return false,
    };
    if amount_micro <= 0 || risk_cap_triggered {
        return false;
    }
    if available_balance.micro_units() < amount_micro || risk_cap.micro_units() < amount_micro {
        return false;
    }
    if liquidity_depth
        .map(|depth| depth.micro_units() < amount_micro)
        .unwrap_or(true)
    {
        return false;
    }

    edge_bps.map(|edge| edge > threshold_bps).unwrap_or(false)
        && expected_value_micro.map(|ev| ev > 0).unwrap_or(false)
}

/// TRACE_MATRIX FC1/FC3: fail-closed validator enforcing role-side, bps,
/// no-private-material, and non-forced-trade invariants for EV traces.
pub fn validate_ev_decision_trace(trace: &EVDecisionTrace) -> Result<(), String> {
    if trace.schema_version != EV_DECISION_TRACE_SCHEMA_ID {
        return Err(format!(
            "unexpected EVDecisionTrace schema_version={}",
            trace.schema_version
        ));
    }
    if trace.review_window_id.trim().is_empty() {
        return Err("EVDecisionTrace review_window_id must be non-empty".into());
    }
    if trace.review_response_id.trim().is_empty() {
        return Err("EVDecisionTrace review_response_id must be non-empty".into());
    }
    if let Some(value) = trace.implied_probability_bps {
        validate_bps("implied_probability_bps", value)?;
    }
    if let Some(value) = trace.agent_probability_bps {
        validate_bps("agent_probability_bps", value)?;
    }
    if let Some(value) = trace.slippage_bps {
        validate_bps("slippage_bps", value)?;
    }
    if trace
        .quoted_price
        .map(|price| price.denominator == 0)
        .unwrap_or(false)
    {
        return Err("quoted_price denominator must be non-zero".into());
    }
    let basis_complete = trace.policy_basis_available();
    if trace.public_summary.trim().is_empty() {
        return Err("EVDecisionTrace public_summary must be non-empty".into());
    }
    if contains_forbidden_private_material(&trace.public_summary) {
        return Err("EVDecisionTrace public_summary contains private/raw material".into());
    }
    match trace.role {
        AgentRole::BullTrader => {
            if trace.side != MarketSide::Yes || trace.action == EVAction::BuyNo {
                return Err("BullTrader EVDecisionTrace cannot choose NO side".into());
            }
        }
        AgentRole::BearTrader => {
            if trace.side != MarketSide::No || trace.action == EVAction::BuyYes {
                return Err("BearTrader EVDecisionTrace cannot choose YES side".into());
            }
        }
        other => {
            return Err(format!(
                "EVDecisionTrace requires BullTrader/BearTrader, got {}",
                other.label()
            ));
        }
    }
    if trace.action == EVAction::Abstain && trace.reason == EVReason::Unknown {
        return Err("Abstain requires structured EV reason, not Unknown".into());
    }
    if matches!(trace.action, EVAction::BuyYes | EVAction::BuyNo)
        && trace.reason != EVReason::PositiveEV
    {
        return Err("Buy/Short requires PositiveEV reason".into());
    }
    if matches!(trace.action, EVAction::BuyYes | EVAction::BuyNo) && !basis_complete {
        return Err("Buy/Short requires complete public EV basis".into());
    }
    Ok(())
}

fn validate_bps(field: &str, value: i64) -> Result<(), String> {
    if !(0..=10_000).contains(&value) {
        return Err(format!("{field} must be integer bps in [0,10000]"));
    }
    Ok(())
}

fn contains_forbidden_private_material(summary: &str) -> bool {
    let lower = summary.to_ascii_lowercase();
    [
        "raw_prompt",
        "raw prompt",
        "raw_completion",
        "raw completion",
        "private cot",
        "chain of thought",
        "raw_log",
        "raw log",
    ]
    .iter()
    .any(|needle| lower.contains(needle))
}

/// TRACE_MATRIX FC1/FC3: writes validated EVDecisionTrace as Generic CAS
/// evidence without changing typed transaction or sequencer admission schema.
pub fn write_ev_decision_trace_to_cas(
    cas: &mut CasStore,
    trace: &EVDecisionTrace,
    suffix: &str,
    logical_t: u64,
) -> Result<Cid, CasError> {
    validate_ev_decision_trace(trace)
        .map_err(|e| CasError::BackendCorruption(format!("EVDecisionTrace invalid: {e}")))?;
    let bytes = serde_json::to_vec(trace)
        .map_err(|e| CasError::BackendCorruption(format!("EVDecisionTrace encode: {e}")))?;
    cas.put(
        &bytes,
        ObjectType::Generic,
        &format!("real13a-ev-decision-trace-{suffix}"),
        logical_t,
        Some(EV_DECISION_TRACE_SCHEMA_ID.to_string()),
    )
}

/// TRACE_MATRIX FC3: reads and validates EVDecisionTrace from CAS for audit
/// and dashboard materialized views.
pub fn read_ev_decision_trace_from_cas(
    cas: &CasStore,
    cid: &Cid,
) -> Result<EVDecisionTrace, CasError> {
    let bytes = cas.get(cid)?;
    let trace: EVDecisionTrace = serde_json::from_slice(&bytes)
        .map_err(|e| CasError::BackendCorruption(format!("EVDecisionTrace decode: {e}")))?;
    validate_ev_decision_trace(&trace)
        .map_err(|e| CasError::BackendCorruption(format!("EVDecisionTrace invalid: {e}")))?;
    Ok(trace)
}

/// TRACE_MATRIX FC3: enumerates EVDecisionTrace CIDs from CAS metadata rather
/// than stdout, reports, or dashboard counters.
pub fn ev_decision_trace_cids(cas: &CasStore) -> Vec<Cid> {
    cas.list_all_cids()
        .into_iter()
        .filter(|cid| {
            cas.metadata(cid).and_then(|meta| meta.schema_id.as_deref())
                == Some(EV_DECISION_TRACE_SCHEMA_ID)
        })
        .collect()
}

/// REAL-13H: ChainTape/CAS-derived EV decision summary for dashboards and
/// probe reports. This is a materialized view over Generic CAS objects.
/// TRACE_MATRIX FC3: aggregate view derived from EVDecisionTrace CAS objects.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EVDecisionTraceSummary {
    pub total: u64,
    pub bull_count: u64,
    pub bear_count: u64,
    pub buy_yes_count: u64,
    pub buy_no_count: u64,
    pub abstain_count: u64,
    pub public_basis_available_count: u64,
    pub public_basis_missing_count: u64,
    pub public_basis_delivery_rate_bps: u64,
    pub by_reason: BTreeMap<EVReason, u64>,
    pub by_action: BTreeMap<EVAction, u64>,
}

impl EVDecisionTraceSummary {
    /// TRACE_MATRIX FC3: folds CAS EVDecisionTrace records into report counts
    /// without becoming a source of truth.
    pub fn from_cas(cas: &CasStore) -> Result<Self, CasError> {
        let by_reason = EVReason::all()
            .into_iter()
            .map(|reason| (reason, 0))
            .collect();
        let by_action = EVAction::all()
            .into_iter()
            .map(|action| (action, 0))
            .collect();
        let mut summary = Self {
            total: 0,
            bull_count: 0,
            bear_count: 0,
            buy_yes_count: 0,
            buy_no_count: 0,
            abstain_count: 0,
            public_basis_available_count: 0,
            public_basis_missing_count: 0,
            public_basis_delivery_rate_bps: 0,
            by_reason,
            by_action,
        };
        for cid in ev_decision_trace_cids(cas) {
            let trace = read_ev_decision_trace_from_cas(cas, &cid)?;
            summary.total += 1;
            match trace.role {
                AgentRole::BullTrader => summary.bull_count += 1,
                AgentRole::BearTrader => summary.bear_count += 1,
                _ => {}
            }
            match trace.action {
                EVAction::BuyYes => summary.buy_yes_count += 1,
                EVAction::BuyNo => summary.buy_no_count += 1,
                EVAction::Abstain => summary.abstain_count += 1,
            }
            if trace.policy_basis_available() {
                summary.public_basis_available_count += 1;
            } else {
                summary.public_basis_missing_count += 1;
            }
            *summary.by_reason.entry(trace.reason).or_insert(0) += 1;
            *summary.by_action.entry(trace.action).or_insert(0) += 1;
        }
        if summary.total > 0 {
            summary.public_basis_delivery_rate_bps =
                summary.public_basis_available_count.saturating_mul(10_000) / summary.total;
        }
        Ok(summary)
    }
}
