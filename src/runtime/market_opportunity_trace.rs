//! REAL-11 MarketOpportunityTrace.
//!
//! Pure derived-view helper for answering whether a Trader turn had an
//! actionable market. This module does not add a CAS object type or mutate
//! economic state; callers may anchor/report the derived trace separately.

use serde::{Deserialize, Serialize};

use crate::bottom_white::cas::schema::{Cid, ObjectType};
use crate::bottom_white::cas::store::{CasError, CasStore};
use crate::economy::money::MicroCoin;
use crate::runtime::market_decision_trace::NoTradeReason;
use crate::runtime::real5_roles::{AgentRole, HeadT};
use crate::state::q_state::{AgentId, PoolStatus, QState, TaskId};
use crate::state::typed_tx::EventId;

/// TRACE_MATRIX FC1-N44 + FC3-N43 (REAL-11): schema id for CAS-anchored
/// Trader opportunity traces.
pub const MARKET_OPPORTUNITY_TRACE_SCHEMA_VERSION: &str = "real11.market_opportunity_trace.v1";

/// TRACE_MATRIX FC1-N44 + FC3-N43 (REAL-11): input needed to derive a
/// tape-visible market opportunity view for one Trader turn.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarketOpportunityRequest {
    pub agent_id: AgentId,
    pub role: AgentRole,
    pub task_id: TaskId,
    pub head_t: HeadT,
    pub router_available: bool,
    pub market_prompt_budget_elided: bool,
    pub prompt_capsule_cid: Option<Cid>,
}

/// TRACE_MATRIX FC1-N44 + FC3-N43 (REAL-11): CAS-serializable trace proving
/// whether a Trader turn had visible/actionable markets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketOpportunityTrace {
    pub schema_version: String,
    pub agent_id: AgentId,
    pub role: AgentRole,
    pub task_id: TaskId,
    pub head_t: HeadT,
    pub visible_markets: Vec<EventId>,
    pub actionable_markets: Vec<EventId>,
    pub available_balance: MicroCoin,
    pub router_available: bool,
    pub reason_if_no_actionable_market: Option<NoTradeReason>,
    pub prompt_capsule_cid: Option<Cid>,
}

/// TRACE_MATRIX FC1-N44 + FC3-N43 (REAL-11): derive opportunity facts from
/// QState without creating an off-tape source of truth.
pub fn derive_market_opportunity_trace(
    q: &QState,
    request: MarketOpportunityRequest,
) -> MarketOpportunityTrace {
    let available_balance = q
        .economic_state_t
        .balances_t
        .0
        .get(&request.agent_id)
        .copied()
        .unwrap_or_else(MicroCoin::zero);

    let mut visible_markets: Vec<EventId> = q
        .economic_state_t
        .cpmm_pools_t
        .0
        .iter()
        .filter_map(|(event_id, pool)| {
            if pool.status == PoolStatus::Active {
                Some(event_id.clone())
            } else {
                None
            }
        })
        .collect();
    visible_markets.sort_by(|a, b| a.0 .0.cmp(&b.0 .0));

    let mut actionable_markets = Vec::new();
    if request.router_available
        && !request.market_prompt_budget_elided
        && available_balance.micro_units() > 0
    {
        for event_id in &visible_markets {
            if let Some(pool) = q.economic_state_t.cpmm_pools_t.0.get(event_id) {
                if pool.pool_yes.units > 0 && pool.pool_no.units > 0 {
                    actionable_markets.push(event_id.clone());
                }
            }
        }
    }

    let reason_if_no_actionable_market = if !actionable_markets.is_empty() {
        None
    } else if request.market_prompt_budget_elided {
        Some(NoTradeReason::PromptBudgetExceeded)
    } else if !request.router_available {
        Some(NoTradeReason::NoPromptTool)
    } else if visible_markets.is_empty() {
        Some(NoTradeReason::NoPool)
    } else if available_balance.micro_units() <= 0 {
        Some(NoTradeReason::AmountExceedsBalance)
    } else {
        Some(NoTradeReason::RouterRejected)
    };

    MarketOpportunityTrace {
        schema_version: MARKET_OPPORTUNITY_TRACE_SCHEMA_VERSION.to_string(),
        agent_id: request.agent_id,
        role: request.role,
        task_id: request.task_id,
        head_t: request.head_t,
        visible_markets,
        actionable_markets,
        available_balance,
        router_available: request.router_available,
        reason_if_no_actionable_market,
        prompt_capsule_cid: request.prompt_capsule_cid,
    }
}

/// TRACE_MATRIX FC1-N44 + FC3-N43 (REAL-11): enforce one opportunity trace
/// for each observed Trader market turn.
pub fn verify_one_trace_per_trader_turn(
    trader_turn_count: usize,
    trace_rows: &[(String, String)],
) -> Result<(), String> {
    if trader_turn_count == 0 {
        return Err("trader_turn_count == 0".into());
    }
    if trace_rows.len() != trader_turn_count {
        return Err(format!(
            "market_opportunity_trace_count={} does not match trader_turn_count={}",
            trace_rows.len(),
            trader_turn_count
        ));
    }
    Ok(())
}

/// TRACE_MATRIX FC1-N44 + FC3-N43 (REAL-11): anchor a derived opportunity
/// trace into CAS as a generic, schema-tagged evidence object.
pub fn write_market_opportunity_trace_to_cas(
    cas: &mut CasStore,
    trace: &MarketOpportunityTrace,
    suffix: &str,
    logical_t: u64,
) -> Result<Cid, CasError> {
    let bytes = serde_json::to_vec(trace)
        .map_err(|e| CasError::BackendCorruption(format!("opportunity trace encode: {e}")))?;
    cas.put(
        &bytes,
        ObjectType::Generic,
        &format!("real11-market-opportunity-trace-{suffix}"),
        logical_t,
        Some(MARKET_OPPORTUNITY_TRACE_SCHEMA_VERSION.to_string()),
    )
}

/// TRACE_MATRIX FC1-N44 + FC3-N43 (REAL-11): list CAS records carrying the
/// REAL-11 opportunity trace schema id.
pub fn market_opportunity_trace_cids(cas: &CasStore) -> Vec<Cid> {
    cas.list_all_cids()
        .into_iter()
        .filter(|cid| {
            cas.metadata(cid).and_then(|meta| meta.schema_id.as_deref())
                == Some(MARKET_OPPORTUNITY_TRACE_SCHEMA_VERSION)
        })
        .collect()
}
