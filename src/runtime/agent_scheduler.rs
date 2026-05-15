//! TB-G G5 — observe-only opportunity scheduler helper.
//!
//! This module is intentionally pure. It records which agent would be selected
//! under a scheduler mode; it does not mutate QState or replace sequencer
//! admission.

use serde::{Deserialize, Serialize};

use crate::bottom_white::cas::schema::{Cid, ObjectType};
use crate::bottom_white::cas::store::{CasError, CasStore};
use crate::runtime::real5_roles::{AgentRole, HeadT, PriceSignal};
use crate::state::q_state::{AgentId, TxId};

pub const SCHEDULER_DECISION_TRACE_SCHEMA_ID: &str = "real6.scheduler_decision_trace.v1";

/// TRACE_MATRIX FC1-N7 + FC3-N43: G5 closeout scheduler mode is a
/// materialized runtime/reporting helper only; it does not mutate QState or
/// sequencer admission.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchedulerMode {
    RoundRobin,
    ObserveOnly,
}

/// TRACE_MATRIX FC1-N7 + FC3-N43: public schedule decision witness used by
/// tests and reports to prove observe-only scheduling without hidden market
/// authority.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentScheduleDecision {
    pub agent_id: Option<AgentId>,
    pub mode: SchedulerMode,
    pub observe_only: bool,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchedulerPnlSignal {
    pub agent_id: AgentId,
    pub realized_pnl: i64,
    pub unrealized_pnl: i64,
    pub available_micro: i64,
    pub risk_cap_micro: i64,
}

/// TRACE_MATRIX FC1-N7 + FC3-N43: REAL-6D observe-only recommendation
/// record. It may carry price/PnL signals into reporting, but it never
/// changes admission, predicates, or task verification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchedulerDecisionTrace {
    pub head_t: HeadT,
    pub visible_agents: Vec<AgentId>,
    pub visible_nodes: Vec<TxId>,
    pub price_signals: Vec<PriceSignal>,
    pub pnl_signals: Vec<SchedulerPnlSignal>,
    pub recommended_agent: Option<AgentId>,
    pub recommended_role: Option<AgentRole>,
    pub recommended_action: Option<String>,
    pub observe_only: bool,
}

pub fn write_scheduler_decision_trace_to_cas(
    cas: &mut CasStore,
    trace: &SchedulerDecisionTrace,
    suffix: &str,
    logical_t: u64,
) -> Result<Cid, CasError> {
    let bytes = serde_json::to_vec(trace)
        .map_err(|e| CasError::BackendCorruption(format!("scheduler trace encode: {e}")))?;
    cas.put(
        &bytes,
        ObjectType::Generic,
        &format!("real6-scheduler-decision-trace-{suffix}"),
        logical_t,
        Some(SCHEDULER_DECISION_TRACE_SCHEMA_ID.to_string()),
    )
}

pub fn read_scheduler_decision_trace_from_cas(
    cas: &CasStore,
    cid: &Cid,
) -> Result<SchedulerDecisionTrace, CasError> {
    let bytes = cas.get(cid)?;
    serde_json::from_slice(&bytes)
        .map_err(|e| CasError::BackendCorruption(format!("scheduler trace decode: {e}")))
}

pub fn scheduler_decision_trace_cids(cas: &CasStore) -> Vec<Cid> {
    cas.list_all_cids()
        .into_iter()
        .filter(|cid| {
            cas.metadata(cid).and_then(|meta| meta.schema_id.as_deref())
                == Some(SCHEDULER_DECISION_TRACE_SCHEMA_ID)
        })
        .collect()
}

impl AgentScheduleDecision {
    /// TRACE_MATRIX FC1-N7 + FC3-N43: explicit abstain witness for empty or
    /// non-actionable agent sets; no ChainTape mutation is performed here.
    pub fn abstain(reason: impl Into<String>) -> Self {
        Self {
            agent_id: None,
            mode: SchedulerMode::RoundRobin,
            observe_only: true,
            reason: Some(reason.into()),
        }
    }
}

/// TRACE_MATRIX FC1-N7 + FC3-N43: deterministic G5 scheduler helper preserving
/// round-robin back-compat while exposing observe-only mode as reportable
/// evidence.
pub fn schedule_next_agent(
    agents: &[AgentId],
    turn_index: usize,
    mode: SchedulerMode,
) -> AgentScheduleDecision {
    if agents.is_empty() {
        return AgentScheduleDecision::abstain("no_agents_available");
    }
    let idx = turn_index % agents.len();
    AgentScheduleDecision {
        agent_id: Some(agents[idx].clone()),
        mode,
        observe_only: matches!(mode, SchedulerMode::ObserveOnly),
        reason: None,
    }
}

pub fn build_observe_only_scheduler_trace(
    head_t: HeadT,
    visible_agents: Vec<AgentId>,
    visible_nodes: Vec<TxId>,
    price_signals: Vec<PriceSignal>,
    pnl_signals: Vec<SchedulerPnlSignal>,
    recommended_agent: Option<AgentId>,
    recommended_role: Option<AgentRole>,
    recommended_action: Option<String>,
) -> SchedulerDecisionTrace {
    SchedulerDecisionTrace {
        head_t,
        visible_agents,
        visible_nodes,
        price_signals,
        pnl_signals,
        recommended_agent,
        recommended_role,
        recommended_action,
        observe_only: true,
    }
}

pub fn render_scheduler_trace_section(trace: &SchedulerDecisionTrace) -> String {
    let mut out = String::new();
    out.push_str("\n## §J.1 Opportunity Scheduler recommendation (observe-only)\n");
    out.push_str("  interpretation: non-binding materialized view; price is signal, not truth\n");
    out.push_str("  recommendation does not change sequencer admission or L4/L4.E predicates\n");
    out.push_str(&format!("  head_t: {}\n", trace.head_t));
    out.push_str(&format!("  observe_only: {}\n", trace.observe_only));
    out.push_str(&format!(
        "  visible_agents: {}\n",
        trace.visible_agents.len()
    ));
    out.push_str(&format!("  visible_nodes: {}\n", trace.visible_nodes.len()));
    out.push_str(&format!("  price_signals: {}\n", trace.price_signals.len()));
    out.push_str(&format!("  pnl_signals: {}\n", trace.pnl_signals.len()));
    out.push_str(&format!(
        "  recommended_agent: {}\n",
        trace
            .recommended_agent
            .as_ref()
            .map(|a| a.0.as_str())
            .unwrap_or("None")
    ));
    out.push_str(&format!(
        "  recommended_role: {}\n",
        trace.recommended_role.map(|r| r.label()).unwrap_or("None")
    ));
    out.push_str(&format!(
        "  recommended_action: {}\n",
        trace.recommended_action.as_deref().unwrap_or("None")
    ));
    if !trace.price_signals.is_empty() {
        out.push_str("  price_signal_sample:\n");
        for signal in trace.price_signals.iter().take(3) {
            out.push_str(&format!(
                "    - event={} price={} depth_micro={}\n",
                signal.event_id,
                signal.price,
                signal
                    .depth
                    .map(|v| v.to_string())
                    .unwrap_or_else(|| "None".into())
            ));
        }
    }
    if !trace.pnl_signals.is_empty() {
        out.push_str("  pnl_signal_sample:\n");
        for signal in trace.pnl_signals.iter().take(3) {
            out.push_str(&format!(
                "    - agent={} realized_pnl={}μC unrealized_pnl={}μC available={}μC risk_cap={}μC\n",
                signal.agent_id.0,
                signal.realized_pnl,
                signal.unrealized_pnl,
                signal.available_micro,
                signal.risk_cap_micro
            ));
        }
    }
    out
}
