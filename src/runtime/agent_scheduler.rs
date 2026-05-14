//! TB-G G5 — observe-only opportunity scheduler helper.
//!
//! This module is intentionally pure. It records which agent would be selected
//! under a scheduler mode; it does not mutate QState or replace sequencer
//! admission.

use crate::state::q_state::AgentId;

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
