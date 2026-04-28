//! TRACE_MATRIX Art 0.1: 四要素映射 (Tape / Input-Tape / Q / State).
//! TRACE_MATRIX Art 0.4: Q_t version-controlled state vector.
//! TRACE_MATRIX WP § 4: 9-component system state Q_t.
//! TRACE_MATRIX WP § 0 axiom 1: state monotonicity.
//!
//! Atom: CO1.2 (Q_t struct) — implements `STATE_TRANSITION_SPEC v1.4 § 1.1`.
//! All public re-exports below are surface for the same TRACE_MATRIX rows.

/// TRACE_MATRIX Art 0.4 / WP § 4 — Q_t module: implements all 9 system state fields.
pub mod q_state;

/// TRACE_MATRIX FC2-Submit / CO1.1.4-pre1 — typed-tx ABI surface (TypedTx + per-kind structs).
pub mod typed_tx;

pub use q_state::{
    AgentId, AgentSwarmState, AgentVisibleProjection, BalancesIndex, BudgetSnapshot,
    ChallengeCase, ChallengeCasesIndex, ClaimEntry, ClaimsIndex, EconomicState, EscrowEntry,
    EscrowsIndex, Hash, NodeId, PerAgentState, PriceIndex, QState, Reputation, ReputationsIndex,
    RoyaltyEdge, RoyaltyGraph, StakeEntry, StakesIndex, TaskMarketEntry, TaskMarketsIndex, TxId,
};

pub use typed_tx::{
    AgentSignature, BoolWithProof, ChallengeTx, FinalizeRewardTx, HasSubmitter,
    PredicateId, PredicateResultsBundle, ReadKey, RejectionClass, ReuseTx, RunId, RunOutcome,
    SafetyOrCreation, SignalBundle, SignalKind, SlashEvidenceCid, TaskExpireTx, TaskId,
    ToolId, TransitionError, TxStatus, TypedTx, VerifyTx, VerifyVerdict, WorkTx, WriteKey,
};
