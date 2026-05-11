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

/// TRACE_MATRIX FC1-N7 TB-N3 A4 (architect ruling 2026-05-11 amendment 5
/// + Q7 + §8.4) — same-task `node_survive:*` market-context renderer
/// consumed by `evaluator.rs::build_agent_prompt(market_ticker, …)`.
/// Filters `cpmm_pools_t` to events whose underlying WorkTx is on the
/// caller-supplied accepted-WorkTx-for-task list. Integer-rational price
/// (NEVER decimal) per architect "no price as truth"; sort by depth desc
/// then recency desc; top-K cap (`TURINGOS_TB_N3_MARKET_CONTEXT_K` env,
/// default 10). Suffix banner `"price is signal, not truth"` per
/// CLAUDE.md §17 + audit_dashboard §14.
pub mod market_context;
