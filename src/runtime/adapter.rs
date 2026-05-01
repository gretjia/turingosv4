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

use crate::bottom_white::cas::schema::Cid;
use crate::economy::money::{MicroCoin, StakeMicroCoin};
use crate::runtime::agent_keypairs::{AgentKeypairError, AgentKeypairRegistry};
use crate::state::q_state::{AgentId, Hash, QState, TaskId, TxId};
use crate::state::typed_tx::{
    AgentSignature, BoolWithProof, EscrowLockTx, PredicateId, PredicateResultsBundle, ReadKey,
    SafetyOrCreation, TaskOpenTx, TypedTx, WorkSigningPayload, WorkTx, WriteKey,
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

/// TRACE_MATRIX FC1-N14: TB-7 Atom 2 — real-signature WorkTx constructor.
///
/// Builds a `WorkTx` and signs it via the per-run `AgentKeypairRegistry`.
/// Mirrors `make_synthetic_worktx` shape but:
///
/// 1. Takes `proposal_cid` as a real CAS reference (the
///    `ProposalTelemetry` object written by Atom 1.5 `proposal_telemetry`).
/// 2. Computes `WorkSigningPayload::canonical_digest()` and signs via
///    `AgentKeypairRegistry::sign(agent_id, digest)` — a real Ed25519
///    signature, not a zero placeholder.
/// 3. The `AgentSignature` is verifiable post-replay against the
///    on-disk `agent_pubkeys.json` manifest (Atom 4 verify_chaintape
///    extension; Gate 4).
///
/// This is the AUTHORITATIVE per-LLM-proposal WorkTx for TB-7 Frame B
/// closure (charter §4.0 + §8 Gate 1). Atom 2 evaluator hook calls this
/// for every meaningful real LLM proposal in the append branch.
#[allow(clippy::too_many_arguments)]
pub fn make_real_worktx_signed_by(
    keypairs: &mut AgentKeypairRegistry,
    task: &str,
    agent: &str,
    parent_state_root: Hash,
    stake_micro: i64,
    suffix: &str,
    proposal_cid: Cid,
    predicate_passes: bool,
    timestamp_logical: u64,
) -> Result<TypedTx, AgentKeypairError> {
    let mut acceptance = BTreeMap::new();
    acceptance.insert(
        PredicateId("acc1".into()),
        BoolWithProof {
            value: predicate_passes,
            proof_cid: None,
        },
    );

    let agent_id = AgentId(agent.into());
    let task_id = TaskId(task.into());
    let tx_id = TxId(format!("worktx-{}-{}", task, suffix));
    let read_set: BTreeSet<ReadKey> = [ReadKey("k.read".into())].into_iter().collect();
    let write_set: BTreeSet<WriteKey> = [WriteKey("k.write".into())].into_iter().collect();
    let predicate_results = PredicateResultsBundle {
        acceptance,
        settlement: BTreeMap::new(),
        safety_class: SafetyOrCreation::Safety,
    };
    let stake = StakeMicroCoin::from_micro_units(stake_micro);

    // Build the SigningPayload (10 fields; signature excluded per typed_tx.rs §3).
    let payload = WorkSigningPayload {
        tx_id: tx_id.clone(),
        task_id: task_id.clone(),
        parent_state_root,
        agent_id: agent_id.clone(),
        read_set: read_set.clone(),
        write_set: write_set.clone(),
        proposal_cid,
        predicate_results: predicate_results.clone(),
        stake,
        timestamp_logical,
    };
    let digest = payload.canonical_digest();
    let signature = keypairs.sign(&agent_id, digest)?;

    Ok(TypedTx::Work(WorkTx {
        tx_id,
        task_id,
        parent_state_root,
        agent_id,
        read_set,
        write_set,
        proposal_cid,
        predicate_results,
        stake,
        signature,
        timestamp_logical,
    }))
}

#[cfg(test)]
mod adapter_tests_atom2 {
    use super::*;
    use tempfile::TempDir;

    /// U-A2.a — make_real_worktx_signed_by produces a non-zero signature
    /// that verifies against the agent's pinned pubkey via the manifest.
    #[test]
    fn real_worktx_signature_is_nonzero_and_verifies() {
        use crate::runtime::agent_keypairs::{verify_agent_signature, AgentPubkeyManifest};
        let repo = TempDir::new().expect("tempdir");
        let mut reg = AgentKeypairRegistry::open(repo.path()).expect("open");
        let tx = make_real_worktx_signed_by(
            &mut reg,
            "task-a2a",
            "n1",
            Hash::ZERO,
            1_000_000,
            "u1",
            Cid([7u8; 32]),
            true,
            1,
        )
        .expect("build real worktx");
        let work = match &tx {
            TypedTx::Work(w) => w.clone(),
            _ => panic!("expected Work"),
        };
        // Signature is non-zero (Ed25519 over canonical digest).
        assert_ne!(*work.signature.as_bytes(), [0u8; 64]);
        // Signature verifies via the manifest (= what verify_chaintape will do).
        let payload = WorkSigningPayload {
            tx_id: work.tx_id.clone(),
            task_id: work.task_id.clone(),
            parent_state_root: work.parent_state_root,
            agent_id: work.agent_id.clone(),
            read_set: work.read_set.clone(),
            write_set: work.write_set.clone(),
            proposal_cid: work.proposal_cid,
            predicate_results: work.predicate_results.clone(),
            stake: work.stake,
            timestamp_logical: work.timestamp_logical,
        };
        let digest = payload.canonical_digest();
        let manifest = AgentPubkeyManifest::load(reg.manifest_path()).expect("load manifest");
        let pubkey = manifest.get(&work.agent_id).expect("pubkey for n1");
        verify_agent_signature(&work.signature, &digest, &pubkey).expect("verify");
    }

    /// U-A2.b — same record, same registry → same signature byte-for-byte
    /// (deterministic signing of the canonical digest).
    #[test]
    fn signing_same_payload_same_registry_is_deterministic() {
        let repo = TempDir::new().expect("tempdir");
        let mut reg = AgentKeypairRegistry::open(repo.path()).expect("open");
        let tx1 = make_real_worktx_signed_by(
            &mut reg,
            "task-a2b",
            "n1",
            Hash::ZERO,
            1_000_000,
            "u1",
            Cid([7u8; 32]),
            true,
            1,
        )
        .expect("tx1");
        let tx2 = make_real_worktx_signed_by(
            &mut reg,
            "task-a2b",
            "n1",
            Hash::ZERO,
            1_000_000,
            "u1",
            Cid([7u8; 32]),
            true,
            1,
        )
        .expect("tx2");
        let s1 = match &tx1 {
            TypedTx::Work(w) => *w.signature.as_bytes(),
            _ => panic!(),
        };
        let s2 = match &tx2 {
            TypedTx::Work(w) => *w.signature.as_bytes(),
            _ => panic!(),
        };
        assert_eq!(s1, s2);
    }
}
