//! TB-6 Atom 2 — chaintape adapter helpers.
//!
//! Constructors + seeding helpers for routing Agent proposals / candidate
//! proofs through the production `Sequencer` via `bus.submit_typed_tx`.
//! Used by:
//! - `tests/tb_6_runtime_chaintape_bootstrap.rs` T10+ (synthetic fixture proof
//!   that L4 + L4.E entries appear on disk).
//! - `experiments/minif2f_v4/src/bin/evaluator.rs` Atom 3 hook (when chaintape
//!   mode is on, emit a `WorkTx` per evaluator decision).
//!
//! Per architect ruling 2026-05-01 § 3.6 Atom 2: "First version (do NOT
//! rewrite evaluator at once). Adapter only: Agent proposal → WorkTx; Lean
//! accept → accepted WorkTx path; Lean fail / predicate fail → rejected WorkTx
//! / L4.E path. Minimum: 1 accepted + 1 rejected WorkTx."
//!
//! This module is `pub use`-d from `src/runtime/mod.rs` so callers reach it
//! as `turingosv4::runtime::adapter::*`.

use std::collections::{BTreeMap, BTreeSet};

use crate::economy::money::{MicroCoin, StakeMicroCoin};
use crate::state::q_state::{AgentId, Hash, QState, TaskId, TxId};
use crate::state::typed_tx::{
    AgentSignature, BoolWithProof, EscrowLockTx, PredicateId, PredicateResultsBundle, ReadKey,
    SafetyOrCreation, TaskOpenTx, TypedTx, WorkTx, WriteKey,
};

/// TRACE_MATRIX FC3-N1: TB-6 Atom 2 adapter — pre-seed initial QState with sponsor balances.
///
/// Mirrors `tests/tb_3_rsp1_formal_surface.rs::genesis_with_balances` in
/// shape. Returns a `QState::genesis()` with `balances_t` pre-populated; callers
/// pass this into `build_chaintape_sequencer_with_initial_q`.
///
/// **Test-fixture / Atom 3 smoke only**. Real production seeding goes through
/// `on_init_tx` per WP § 14.1; this helper is the synthetic alternative.
pub fn genesis_with_balances(pairs: &[(AgentId, MicroCoin)]) -> QState {
    let mut q = QState::genesis();
    for (agent, balance) in pairs {
        q.economic_state_t
            .balances_t
            .0
            .insert(agent.clone(), *balance);
    }
    q
}

/// TRACE_MATRIX FC3-N1: TB-6 Atom 2 adapter — synthetic TaskOpenTx constructor.
pub fn make_synthetic_task_open(
    task: &str,
    sponsor: &str,
    parent_state_root: Hash,
    suffix: &str,
) -> TypedTx {
    TypedTx::TaskOpen(TaskOpenTx {
        tx_id: TxId(format!("taskopen-{}-{}", task, suffix)),
        task_id: TaskId(task.into()),
        parent_state_root,
        sponsor_agent: AgentId(sponsor.into()),
        verifier_quorum: 1,
        max_reuse_royalty_fraction_basis_points: 1000,
        settlement_rule_hash: Hash::ZERO,
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: 1,
    })
}

/// TRACE_MATRIX FC3-N1: TB-6 Atom 2 adapter — synthetic EscrowLockTx constructor.
pub fn make_synthetic_escrow_lock(
    task: &str,
    sponsor: &str,
    amount_micro: i64,
    parent_state_root: Hash,
    suffix: &str,
) -> TypedTx {
    TypedTx::EscrowLock(EscrowLockTx {
        tx_id: TxId(format!("escrowlock-{}-{}", task, suffix)),
        task_id: TaskId(task.into()),
        parent_state_root,
        sponsor_agent: AgentId(sponsor.into()),
        amount: MicroCoin::from_micro_units(amount_micro),
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: 1,
    })
}

/// TRACE_MATRIX FC3-N1: TB-6 Atom 2 adapter — synthetic WorkTx constructor.
///
/// `predicate_passes = true` exercises the accepted L4 path; `predicate_passes
/// = false` triggers L4.E `PredicateFailed` (or `StakeInsufficient` if
/// `stake_micro = 0`). For Atom 3 hooks, `predicate_passes` mirrors the
/// evaluator's accept/reject decision per Lean check.
pub fn make_synthetic_worktx(
    task: &str,
    agent: &str,
    parent_state_root: Hash,
    stake_micro: i64,
    suffix: &str,
    predicate_passes: bool,
) -> TypedTx {
    let mut acceptance = BTreeMap::new();
    acceptance.insert(
        PredicateId("acc1".into()),
        BoolWithProof {
            value: predicate_passes,
            proof_cid: None,
        },
    );
    TypedTx::Work(WorkTx {
        tx_id: TxId(format!("worktx-{}-{}", task, suffix)),
        task_id: TaskId(task.into()),
        parent_state_root,
        agent_id: AgentId(agent.into()),
        read_set: [ReadKey("k.read".into())]
            .into_iter()
            .collect::<BTreeSet<_>>(),
        write_set: [WriteKey("k.write".into())]
            .into_iter()
            .collect::<BTreeSet<_>>(),
        proposal_cid: Default::default(),
        predicate_results: PredicateResultsBundle {
            acceptance,
            settlement: BTreeMap::new(),
            safety_class: SafetyOrCreation::Safety,
        },
        stake: StakeMicroCoin::from_micro_units(stake_micro),
        signature: AgentSignature::from_bytes([0u8; 64]),
        timestamp_logical: 1,
    })
}
