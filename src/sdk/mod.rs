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
/// TRACE_MATRIX FC1-N7 + §15 + G-Phase directive §0.6 amendment G-2 —
/// TB-G G2P.1 (charter §1 Module G2P): per-viewer renderer for the
/// `=== Pending Peer Reviews ===` prompt block. Surfaces the queue of
/// accepted peer WorkTxs the viewer agent can `verify_peer` against,
/// filtered to exclude self-WorkTxs and already-verified targets.
/// Closes user 2026-05-12 病灶3 "0 verify" mechanism gap.
pub mod pending_peer_reviews;
/// TRACE_MATRIX FC1-N7 + §15 + Art. III.2 (TB-G G3.3 2026-05-12; charter
/// §1 Module G3 atom G3.3 + G-Phase directive §G3 verbatim 7-field
/// `AgentMarketState` shape + Drucker framing): per-viewer
/// `=== Your Position ===` prompt block. Renders the agent's own 7-field
/// `AgentMarketStateView` (balance / open_positions / realized_pnl /
/// unrealized_pnl / solvency_status / reputation_score) with the
/// architect-verbatim Drucker framing string at the head. Per-viewer:
/// never aggregates across agents. Empty-string-suppression contract
/// mirrors `econ_position` + `pending_peer_reviews`.
pub mod your_position;
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
