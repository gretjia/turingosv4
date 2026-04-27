//! TRACE_MATRIX Art 0.1: 四要素映射 (Tape / Input-Tape / Q / State).
//! TRACE_MATRIX Art 0.4: Q_t version-controlled state vector.
//! TRACE_MATRIX WP § 4: 9-component system state Q_t.
//! TRACE_MATRIX WP § 0 axiom 1: state monotonicity.
//!
//! Atom: CO1.2 (Q_t struct) — implements `STATE_TRANSITION_SPEC v1.4 § 1.1`.
//! All public re-exports below are surface for the same TRACE_MATRIX rows.

/// TRACE_MATRIX Art 0.4 / WP § 4 — Q_t module: implements all 9 system state fields.
pub mod q_state;

pub use q_state::{
    AgentId, AgentSwarmState, AgentVisibleProjection, BalancesIndex, BudgetSnapshot,
    ChallengeCase, ChallengeCasesIndex, ClaimEntry, ClaimsIndex, EconomicState, EscrowEntry,
    EscrowsIndex, Hash, NodeId, PerAgentState, PriceIndex, QState, Reputation, ReputationsIndex,
    RoyaltyEdge, RoyaltyGraph, StakeEntry, StakesIndex, TaskMarketEntry, TaskMarketsIndex, TxId,
};
