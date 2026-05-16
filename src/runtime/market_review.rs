//! REAL-13B — Market Review Turn sidecars.
//!
//! These records are evidence sidecars for evaluator/orchestrator scheduling.
//! They do not introduce sequencer admission rules.

use serde::{Deserialize, Serialize};

use crate::bottom_white::cas::schema::{Cid, ObjectType};
use crate::bottom_white::cas::store::{CasError, CasStore};
use crate::runtime::real5_roles::AgentRole;
use crate::state::q_state::{AgentId, TxId};
use crate::state::typed_tx::EventId;

/// TRACE_MATRIX FC1/FC3: schema tag for MarketReviewWindow CAS sidecars.
pub const MARKET_REVIEW_WINDOW_SCHEMA_ID: &str = "real13b.market_review_window.v1";
/// TRACE_MATRIX FC1/FC3: schema tag for MarketReviewResponse CAS sidecars.
pub const MARKET_REVIEW_RESPONSE_SCHEMA_ID: &str = "real13b.market_review_response.v1";
/// TRACE_MATRIX FC1/FC3: schema tag for MarketReviewSummary CAS sidecars.
pub const MARKET_REVIEW_SUMMARY_SCHEMA_ID: &str = "real13b.market_review_summary.v1";

/// TRACE_MATRIX FC1: review-window execution mode; sequential is the ship
/// default, barriered async is scaffold-only, full async is unsafe research.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MarketReviewMode {
    SequentialRound,
    BarrieredAsync,
    FullAsyncExperimental,
}

impl MarketReviewMode {
    /// TRACE_MATRIX FC1: guards unrestricted full async behind unsafe research.
    pub const fn requires_unsafe_research(self) -> bool {
        matches!(self, MarketReviewMode::FullAsyncExperimental)
    }
}

/// TRACE_MATRIX FC1/FC3: deterministic market review window describing who
/// must produce EVDecisionTrace evidence for an active market signal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketReviewWindow {
    pub window_id: TxId,
    pub event_id: EventId,
    pub opened_at_head_t: String,
    pub market_snapshot_cid: Cid,
    pub eligible_agents: Vec<AgentId>,
    pub deadline_logical_t: u64,
    pub mode: MarketReviewMode,
}

/// TRACE_MATRIX FC1/FC3: per-agent response sidecar linking a review turn to
/// an EVDecisionTrace or explicit no-response trace.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketReviewResponse {
    pub window_id: TxId,
    pub response_id: String,
    pub agent_id: AgentId,
    pub role: AgentRole,
    pub ev_decision_trace_cid: Option<Cid>,
    pub no_response_trace_cid: Option<Cid>,
    pub action: String,
    pub submitted_tx_id: Option<TxId>,
}

/// TRACE_MATRIX FC3: CAS-backed summary of a review window for reports; it
/// does not mutate L4, L4.E, or sequencer state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketReviewSummary {
    pub window_id: TxId,
    pub event_id: EventId,
    pub response_count: u64,
    pub buy_count: u64,
    pub short_count: u64,
    pub abstain_count: u64,
    pub missing_count: u64,
    pub response_cids: Vec<Cid>,
    pub committed_tx_ids: Vec<TxId>,
}

/// TRACE_MATRIX FC1: deterministic response ordering used by sequential and
/// barriered-async modes to keep replay stable.
pub fn deterministic_response_order(
    mut responses: Vec<MarketReviewResponse>,
) -> Vec<MarketReviewResponse> {
    responses.sort_by(|a, b| {
        a.agent_id
            .cmp(&b.agent_id)
            .then_with(|| a.response_id.cmp(&b.response_id))
    });
    responses
}

/// TRACE_MATRIX FC1/FC3: fail-closed response validator requiring Bull/Bear
/// role and trace linkage before a response can enter CAS.
pub fn validate_market_review_response(response: &MarketReviewResponse) -> Result<(), String> {
    if response.ev_decision_trace_cid.is_none() && response.no_response_trace_cid.is_none() {
        return Err(
            "MarketReviewResponse must link EVDecisionTrace CID or NoResponseTrace CID".into(),
        );
    }
    if !matches!(response.role, AgentRole::BullTrader | AgentRole::BearTrader) {
        return Err(format!(
            "MarketReviewResponse requires BullTrader/BearTrader, got {}",
            response.role.label()
        ));
    }
    Ok(())
}

/// TRACE_MATRIX FC3: fail-closed summary validator ensuring reported counts
/// match the referenced response CIDs.
pub fn validate_market_review_summary(summary: &MarketReviewSummary) -> Result<(), String> {
    if summary.response_count != summary.response_cids.len() as u64 {
        return Err(format!(
            "response_count={} does not match response_cids={}",
            summary.response_count,
            summary.response_cids.len()
        ));
    }
    let counted =
        summary.buy_count + summary.short_count + summary.abstain_count + summary.missing_count;
    if counted != summary.response_count {
        return Err(format!(
            "market review counted outcomes={counted} does not match response_count={}",
            summary.response_count
        ));
    }
    Ok(())
}

/// TRACE_MATRIX FC1/FC3: writes MarketReviewWindow as Generic CAS sidecar
/// without adding a new transaction type or admission rule.
pub fn write_market_review_window_to_cas(
    cas: &mut CasStore,
    window: &MarketReviewWindow,
    suffix: &str,
    logical_t: u64,
) -> Result<Cid, CasError> {
    if window.eligible_agents.is_empty() {
        return Err(CasError::BackendCorruption(
            "MarketReviewWindow eligible_agents must be non-empty".into(),
        ));
    }
    let bytes = serde_json::to_vec(window)
        .map_err(|e| CasError::BackendCorruption(format!("MarketReviewWindow encode: {e}")))?;
    cas.put(
        &bytes,
        ObjectType::Generic,
        &format!("real13b-market-review-window-{suffix}"),
        logical_t,
        Some(MARKET_REVIEW_WINDOW_SCHEMA_ID.to_string()),
    )
}

/// TRACE_MATRIX FC1/FC3: writes MarketReviewResponse as Generic CAS sidecar
/// after role/trace validation.
pub fn write_market_review_response_to_cas(
    cas: &mut CasStore,
    response: &MarketReviewResponse,
    suffix: &str,
    logical_t: u64,
) -> Result<Cid, CasError> {
    validate_market_review_response(response)
        .map_err(|e| CasError::BackendCorruption(format!("MarketReviewResponse invalid: {e}")))?;
    let bytes = serde_json::to_vec(response)
        .map_err(|e| CasError::BackendCorruption(format!("MarketReviewResponse encode: {e}")))?;
    cas.put(
        &bytes,
        ObjectType::Generic,
        &format!("real13b-market-review-response-{suffix}"),
        logical_t,
        Some(MARKET_REVIEW_RESPONSE_SCHEMA_ID.to_string()),
    )
}

/// TRACE_MATRIX FC3: writes validated MarketReviewSummary as Generic CAS
/// evidence for dashboard/run-report regeneration.
pub fn write_market_review_summary_to_cas(
    cas: &mut CasStore,
    summary: &MarketReviewSummary,
    suffix: &str,
    logical_t: u64,
) -> Result<Cid, CasError> {
    validate_market_review_summary(summary)
        .map_err(|e| CasError::BackendCorruption(format!("MarketReviewSummary invalid: {e}")))?;
    let bytes = serde_json::to_vec(summary)
        .map_err(|e| CasError::BackendCorruption(format!("MarketReviewSummary encode: {e}")))?;
    cas.put(
        &bytes,
        ObjectType::Generic,
        &format!("real13b-market-review-summary-{suffix}"),
        logical_t,
        Some(MARKET_REVIEW_SUMMARY_SCHEMA_ID.to_string()),
    )
}

/// TRACE_MATRIX FC3: enumerates MarketReviewSummary records from CAS metadata,
/// not stdout or report text.
pub fn market_review_summary_cids(cas: &CasStore) -> Vec<Cid> {
    cas.list_all_cids()
        .into_iter()
        .filter(|cid| {
            cas.metadata(cid).and_then(|meta| meta.schema_id.as_deref())
                == Some(MARKET_REVIEW_SUMMARY_SCHEMA_ID)
        })
        .collect()
}
