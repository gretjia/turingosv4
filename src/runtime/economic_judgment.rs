//! REAL-12 — CAS-backed EconomicJudgment.
//!
//! This is the missing middle layer between market visibility and router
//! action: did a Bull/Bear role form a public, typed economic judgment, or
//! abstain for a structured reason? It is a generic CAS evidence object and
//! does not change typed transaction schema, sequencer admission, wallet
//! semantics, or Lean predicates.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::bottom_white::cas::schema::{Cid, ObjectType};
use crate::bottom_white::cas::store::{CasError, CasStore};
use crate::economy::money::MicroCoin;
use crate::runtime::real5_roles::{
    read_role_turn_trace_from_cas, role_turn_trace_cids, AgentRole, MarketSide, RationalPrice,
};
use crate::state::q_state::{AgentId, TaskId};
use crate::state::typed_tx::EventId;

/// TRACE_MATRIX FC1-N41 + FC3-N43: CAS schema id for tape-visible REAL-12
/// economic judgments.
pub const ECONOMIC_JUDGMENT_SCHEMA_ID: &str = "real12.economic_judgment.v1";

/// TRACE_MATRIX FC1-N41: typed Bull/Bear economic output class before any
/// router action is admitted by policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum EconomicJudgmentAction {
    Buy,
    Short,
    Abstain,
}

/// TRACE_MATRIX FC1-N41 + FC3-N43: structured public reason for trade or
/// abstain, used by reports instead of private reasoning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum EconomicReason {
    NoPerceivedEdge,
    NoActionableMarket,
    InsufficientBalance,
    RiskCapExceeded,
    LiquidityTooLow,
    ExpectedValueNegative,
    PromptBudgetExceeded,
    UnresolvedOracleRisk,
    RolePolicyBlocked,
    Unknown,
}

/// TRACE_MATRIX FC1-N41: integer/public expected-value sign supplied by the
/// role action parser, not fabricated by the harness.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ExpectedValueSign {
    Positive,
    Zero,
    Negative,
    Unknown,
}

/// TRACE_MATRIX FC1-N41: basis-point probability interval for public EV
/// evidence; avoids float persistence in market/economy paths.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProbabilityBand {
    pub lower_bps: u16,
    pub upper_bps: u16,
}

/// TRACE_MATRIX FC1-N41 + FC2-N31 + FC3-N43: CAS-backed economic judgment
/// linking role, scoped view/prompt capsule, public EV basis, and action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EconomicJudgment {
    pub schema_version: String,
    pub agent_id: AgentId,
    pub role: AgentRole,
    pub task_id: TaskId,
    pub head_t: String,
    pub visible_markets: Vec<EventId>,
    pub chosen_market: Option<EventId>,
    pub intended_side: Option<MarketSide>,
    pub intended_amount: Option<MicroCoin>,
    pub action: EconomicJudgmentAction,
    pub reason: EconomicReason,
    pub observed_price: Option<RationalPrice>,
    pub estimated_probability_band: Option<ProbabilityBand>,
    pub expected_value_sign: ExpectedValueSign,
    pub liquidity_depth: Option<MicroCoin>,
    pub balance_available: MicroCoin,
    pub risk_cap: MicroCoin,
    pub oracle_or_deadline_risk: Option<String>,
    pub prompt_capsule_cid: Cid,
    pub public_summary: String,
}

/// TRACE_MATRIX FC1-N41: fail-closed validator for role/side/EV/shielding
/// constraints before EconomicJudgment can become CAS evidence.
pub fn validate_economic_judgment(judgment: &EconomicJudgment) -> Result<(), String> {
    if judgment.schema_version != ECONOMIC_JUDGMENT_SCHEMA_ID {
        return Err(format!(
            "unexpected EconomicJudgment schema_version={}",
            judgment.schema_version
        ));
    }
    if judgment.public_summary.trim().is_empty() {
        return Err("EconomicJudgment public_summary must be non-empty".into());
    }
    if contains_forbidden_private_material(&judgment.public_summary) {
        return Err("EconomicJudgment public_summary contains private/raw material".into());
    }

    match judgment.action {
        EconomicJudgmentAction::Buy | EconomicJudgmentAction::Short => {
            validate_positive_ev_basis(judgment)?;
            match judgment.role {
                AgentRole::BullTrader => {
                    if judgment.intended_side != Some(MarketSide::Yes) {
                        return Err("BullTrader cannot choose NO side".into());
                    }
                    if judgment.action != EconomicJudgmentAction::Buy {
                        return Err("BullTrader buy action must be Buy".into());
                    }
                }
                AgentRole::BearTrader => {
                    if judgment.intended_side != Some(MarketSide::No) {
                        return Err("BearTrader cannot choose YES side".into());
                    }
                    if judgment.action != EconomicJudgmentAction::Short {
                        return Err("BearTrader buy-no action must be Short".into());
                    }
                }
                other => {
                    return Err(format!(
                        "EconomicJudgment buy/short requires BullTrader/BearTrader, got {}",
                        other.label()
                    ));
                }
            }
        }
        EconomicJudgmentAction::Abstain => {
            if judgment.reason == EconomicReason::Unknown {
                return Err("Abstain requires structured reason, not Unknown".into());
            }
        }
    }
    Ok(())
}

fn validate_positive_ev_basis(judgment: &EconomicJudgment) -> Result<(), String> {
    if judgment.chosen_market.is_none() {
        return Err("Buy/Short requires chosen_market".into());
    }
    if judgment
        .intended_amount
        .map(|m| m.micro_units())
        .unwrap_or(0)
        <= 0
    {
        return Err("Buy/Short requires positive intended_amount".into());
    }
    if judgment.observed_price.is_none() {
        return Err("Buy/Short requires observed_price".into());
    }
    let Some(band) = judgment.estimated_probability_band else {
        return Err("Buy/Short requires estimated_probability_band".into());
    };
    if band.lower_bps > band.upper_bps || band.upper_bps > 10_000 {
        return Err("estimated_probability_band must be ordered basis points".into());
    }
    if judgment.expected_value_sign != ExpectedValueSign::Positive {
        return Err("Buy/Short requires positive EV basis".into());
    }
    Ok(())
}

fn contains_forbidden_private_material(summary: &str) -> bool {
    let lower = summary.to_ascii_lowercase();
    [
        "raw_prompt",
        "raw completion",
        "raw_completion",
        "private cot",
        "chain of thought",
        "raw_log",
        "raw log",
    ]
    .iter()
    .any(|needle| lower.contains(needle))
}

/// TRACE_MATRIX FC1-N41 + FC3-N43: persist validated EconomicJudgment as a
/// CAS object so reports can regenerate from evidence.
pub fn write_economic_judgment_to_cas(
    cas: &mut CasStore,
    judgment: &EconomicJudgment,
    suffix: &str,
    logical_t: u64,
) -> Result<Cid, CasError> {
    validate_economic_judgment(judgment)
        .map_err(|e| CasError::BackendCorruption(format!("economic judgment invalid: {e}")))?;
    let bytes = serde_json::to_vec(judgment)
        .map_err(|e| CasError::BackendCorruption(format!("economic judgment encode: {e}")))?;
    cas.put(
        &bytes,
        ObjectType::Generic,
        &format!("real12-economic-judgment-{suffix}"),
        logical_t,
        Some(ECONOMIC_JUDGMENT_SCHEMA_ID.to_string()),
    )
}

/// TRACE_MATRIX FC3-N43: decode and revalidate a CAS-backed EconomicJudgment
/// for audit/dashboard materialized views.
pub fn read_economic_judgment_from_cas(
    cas: &CasStore,
    cid: &Cid,
) -> Result<EconomicJudgment, CasError> {
    let bytes = cas.get(cid)?;
    let judgment: EconomicJudgment = serde_json::from_slice(&bytes)
        .map_err(|e| CasError::BackendCorruption(format!("economic judgment decode: {e}")))?;
    validate_economic_judgment(&judgment)
        .map_err(|e| CasError::BackendCorruption(format!("economic judgment invalid: {e}")))?;
    Ok(judgment)
}

/// TRACE_MATRIX FC3-N43: enumerate EconomicJudgment CAS objects by schema id,
/// never by stdout or dashboard counters.
pub fn economic_judgment_cids(cas: &CasStore) -> Vec<Cid> {
    cas.list_all_cids()
        .into_iter()
        .filter(|cid| {
            cas.metadata(cid).and_then(|meta| meta.schema_id.as_deref())
                == Some(ECONOMIC_JUDGMENT_SCHEMA_ID)
        })
        .collect()
}

/// TRACE_MATRIX FC3-N43: ChainTape/CAS-derived summary of REAL-12 judgment
/// reasons and actions for dashboard/report output.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EconomicJudgmentReasonSummary {
    pub total: u64,
    pub bull_judgment_count: u64,
    pub bear_judgment_count: u64,
    pub abstain_structured_reason_count: u64,
    pub buy_count: u64,
    pub short_count: u64,
    pub by_role: BTreeMap<AgentRole, u64>,
    pub by_reason: BTreeMap<EconomicReason, u64>,
    pub by_action: BTreeMap<EconomicJudgmentAction, u64>,
}

impl EconomicJudgmentReasonSummary {
    /// TRACE_MATRIX FC3-N43: derive judgment counts from CAS objects, not from
    /// stdout logs.
    pub fn from_cas(cas: &CasStore) -> Result<Self, CasError> {
        let mut summary = Self {
            total: 0,
            bull_judgment_count: 0,
            bear_judgment_count: 0,
            abstain_structured_reason_count: 0,
            buy_count: 0,
            short_count: 0,
            by_role: BTreeMap::new(),
            by_reason: BTreeMap::new(),
            by_action: BTreeMap::new(),
        };
        for cid in economic_judgment_cids(cas) {
            let judgment = read_economic_judgment_from_cas(cas, &cid)?;
            summary.total += 1;
            *summary.by_role.entry(judgment.role).or_insert(0) += 1;
            *summary.by_reason.entry(judgment.reason).or_insert(0) += 1;
            *summary.by_action.entry(judgment.action).or_insert(0) += 1;
            match judgment.role {
                AgentRole::BullTrader => summary.bull_judgment_count += 1,
                AgentRole::BearTrader => summary.bear_judgment_count += 1,
                _ => {}
            }
            match judgment.action {
                EconomicJudgmentAction::Buy => summary.buy_count += 1,
                EconomicJudgmentAction::Short => summary.short_count += 1,
                EconomicJudgmentAction::Abstain => {
                    summary.abstain_structured_reason_count += 1;
                }
            }
        }
        Ok(summary)
    }
}

/// TRACE_MATRIX FC3-N43: coverage witness proving every Bull/Bear RoleTurnTrace
/// links to a matching EconomicJudgment CID.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EconomicJudgmentCoverageReport {
    pub required_trader_turns: u64,
    pub linked_trader_turns: u64,
    pub missing_links: u64,
    pub invalid_links: u64,
    pub mismatched_links: u64,
}

/// TRACE_MATRIX FC3-N43: fail-closed coverage check for Bull/Bear economic
/// judgment links.
pub fn verify_bull_bear_turn_judgment_coverage(
    cas: &CasStore,
) -> Result<EconomicJudgmentCoverageReport, String> {
    let mut report = EconomicJudgmentCoverageReport {
        required_trader_turns: 0,
        linked_trader_turns: 0,
        missing_links: 0,
        invalid_links: 0,
        mismatched_links: 0,
    };
    for trace_cid in role_turn_trace_cids(cas) {
        let trace = read_role_turn_trace_from_cas(cas, &trace_cid)
            .map_err(|e| format!("read RoleTurnTrace {trace_cid}: {e}"))?;
        if !matches!(trace.role, AgentRole::BullTrader | AgentRole::BearTrader) {
            continue;
        }
        report.required_trader_turns += 1;
        let Some(judgment_cid) = trace.economic_judgment_cid else {
            report.missing_links += 1;
            continue;
        };
        let judgment = match read_economic_judgment_from_cas(cas, &judgment_cid) {
            Ok(judgment) => judgment,
            Err(_) => {
                report.invalid_links += 1;
                continue;
            }
        };
        if judgment.agent_id != trace.agent_id
            || judgment.role != trace.role
            || judgment.task_id != trace.task_id
            || judgment.prompt_capsule_cid != trace.prompt_capsule_cid
        {
            report.mismatched_links += 1;
            continue;
        }
        report.linked_trader_turns += 1;
    }

    if report.missing_links == 0 && report.invalid_links == 0 && report.mismatched_links == 0 {
        Ok(report)
    } else {
        Err(format!(
            "Bull/Bear RoleTurnTrace EconomicJudgment coverage failed: \
             missing EconomicJudgment link={} invalid_links={} mismatched_links={} \
             required={} linked={}",
            report.missing_links,
            report.invalid_links,
            report.mismatched_links,
            report.required_trader_turns,
            report.linked_trader_turns
        ))
    }
}
