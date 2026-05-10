pub mod protocol;
pub mod snapshot;
pub mod prompt;
pub mod prompt_guard;
/// TRACE_MATRIX FC1-N7 + §13: TB-N1-AGENT-ECONOMY A2 (session #35
/// 2026-05-10) — renderer for the agent-perceived economic position
/// block embedded in build_agent_prompt under
/// `=== Your Economic Position ===` heading. Reads canonical
/// `EconomicState` (balances_t / stakes_t / claims_t / reputations_t)
/// to close session #35 smoke-evidence finding "n=1 economy
/// structurally landed but invisible to agent at prompt layer".
pub mod econ_position;
pub mod tool;
pub mod actor;
pub mod sandbox;
pub mod tools;
pub mod error_abstraction;
