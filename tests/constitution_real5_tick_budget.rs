//! REAL-5 Atom 5 — Generation Cost / TickBudget gates.

use turingosv4::runtime::real5_roles::{derive_tick_budget, TickBudget, TickEvent};
use turingosv4::state::q_state::AgentId;

#[test]
fn sg_r5_5_externalized_generation_consumes_ticks_without_coin_charge() {
    let agent = AgentId("Agent_0".into());
    let start = TickBudget {
        agent_id: agent.clone(),
        remaining_ticks: 3,
        spent_ticks: 0,
        regenerated_ticks: 0,
    };
    let budget = derive_tick_budget(
        start,
        &[
            TickEvent::ReadOnlyView,
            TickEvent::ExternalizedAction,
            TickEvent::InvalidGenerationPenalty,
            TickEvent::AcceptedGenerationReward,
        ],
    )
    .unwrap();
    assert_eq!(budget.agent_id, agent);
    assert_eq!(budget.spent_ticks, 2);
    assert_eq!(budget.regenerated_ticks, 1);
    assert_eq!(budget.remaining_ticks, 2);
}
