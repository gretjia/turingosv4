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

/// TRACE_MATRIX § 5.2.1 / CO1.7-impl A2+A3 — L4 sequencer + dispatch_transition.
pub mod sequencer;

/// TRACE_MATRIX TB-14 Atom 2 (FC3-N42; architect §5.1 + charter §3 Atom 2):
/// derived-view price index. `compute_price_index(econ)` is the pure-fn
/// view of long / short interest + share depth per node (architect §5.2);
/// never stored as canonical state (no second source-of-truth).
pub mod price_index;

pub use q_state::{
    AgentId, AgentSwarmState, AgentVisibleProjection, BalancesIndex, BudgetSnapshot,
    ChallengeCase, ChallengeCasesIndex, ClaimEntry, ClaimsIndex, EconomicState, EscrowEntry,
    EscrowsIndex, Hash, NodeId, NodePositionsIndex, PerAgentState, QState,
    Reputation, ReputationsIndex, RoyaltyEdge, RoyaltyGraph, RunSummaryEntry, RunsIndex,
    StakeEntry, StakesIndex, TaskId, TaskMarketEntry, TaskMarketState, TaskMarketsIndex, TxId,
};

/// TB-14 Atom 2: derived-view price types. `BoltzmannMaskPolicy` is added
/// in Atom 4 (env loader) and re-exported here at that time.
pub use price_index::{compute_price_index, NodeMarketEntry, RationalPrice};

pub use typed_tx::{
    AgentSignature, BankruptcyReason, BoolWithProof, CapsulePrivacyPolicy,
    ChallengeSigningPayload, ChallengeTx, ClaimId, ExhaustionReason, ExpireReason,
    FinalizeRewardSigningPayload, FinalizeRewardTx, HasSubmitter, NodePosition, PositionKind,
    PositionSide, PredicateId, PredicateResultsBundle, ReadKey, RejectionClass, ReuseTx,
    RunExhaustedTx, RunId, RunOutcome, SafetyOrCreation, SignalBundle, SignalKind,
    SlashEvidenceCid, TaskBankruptcySigningPayload, TaskBankruptcyTx, TaskExpireSigningPayload,
    TaskExpireTx, TerminalSummarySigningPayload, TerminalSummaryTx, ToolId, TransitionError,
    TxStatus, TypedTx, VerifySigningPayload, VerifyTx, VerifyVerdict, WorkSigningPayload,
    WorkTx, WriteKey,
};
