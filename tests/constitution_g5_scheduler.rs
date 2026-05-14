//! TB-G G5 — Opportunity Scheduler observe-only gates.

use turingosv4::runtime::agent_scheduler::{
    schedule_next_agent, AgentScheduleDecision, SchedulerMode,
};
use turingosv4::state::q_state::AgentId;

#[test]
fn sg_g5_1_default_round_robin_back_compat() {
    let agents = vec![
        AgentId("Agent_0".into()),
        AgentId("Agent_1".into()),
        AgentId("Agent_2".into()),
    ];

    let selected: Vec<String> = (0..5)
        .map(|turn| schedule_next_agent(&agents, turn, SchedulerMode::RoundRobin).agent_id)
        .map(|id| id.map(|a| a.0).unwrap_or_else(|| "None".into()))
        .collect();

    assert_eq!(
        selected,
        vec!["Agent_0", "Agent_1", "Agent_2", "Agent_0", "Agent_1"],
        "G5 must preserve default round-robin ordering"
    );
}

#[test]
fn sg_g5_2_scheduler_can_return_abstain_for_empty_agent_set() {
    let decision = schedule_next_agent(&[], 0, SchedulerMode::RoundRobin);
    assert_eq!(
        decision,
        AgentScheduleDecision::abstain("no_agents_available"),
        "G5 kill condition: scheduler must be able to return Abstain"
    );
}

#[test]
fn sg_g5_3_observe_only_mode_does_not_override_selected_agent() {
    let agents = vec![AgentId("Agent_0".into()), AgentId("Agent_1".into())];
    let decision = schedule_next_agent(&agents, 1, SchedulerMode::ObserveOnly);
    assert_eq!(
        decision.agent_id.as_ref().map(|a| a.0.as_str()),
        Some("Agent_1")
    );
    assert_eq!(decision.mode, SchedulerMode::ObserveOnly);
    assert!(
        decision.observe_only,
        "observe-only scheduler records the decision without becoming a state mutator"
    );
}
