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
    AgentSignature, BoolWithProof, EscrowLockSigningPayload, EscrowLockTx, PredicateId,
    PredicateResultsBundle, ReadKey, SafetyOrCreation, TaskOpenSigningPayload, TaskOpenTx, TypedTx,
    VerifySigningPayload, VerifyTx, VerifyVerdict, WorkSigningPayload, WorkTx, WriteKey,
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

/// TRACE_MATRIX FC1-N14: TB-7 Atom 3 — real-signature VerifyTx constructor for
/// OMEGA-branch routing.
///
/// Builds a `VerifyTx` paired with an accepted `WorkTx` for the OMEGA path
/// (Lean oracle accepted the proof → verifier confirms via VerifyTx). Signs
/// via the same `AgentKeypairRegistry` as the WorkTx side. Produces a
/// `VerifyVerdict::Confirm` when `verdict_confirms = true`.
///
/// **OMEGA scope NARROWED per ARCHITECT_RULING D3 + charter §4.3**: WorkTx
/// + VerifyTx pair only; ChallengeWindow stays OPEN; NO FinalizeRewardTx,
/// NO SlashTx, NO settlement. Settlement is RSP-4 / TB-9 territory.
#[allow(clippy::too_many_arguments)]
pub fn make_real_verifytx_signed_by(
    keypairs: &mut AgentKeypairRegistry,
    parent_state_root: Hash,
    target_work_tx: TxId,
    verifier_agent: &str,
    bond_micro: i64,
    suffix: &str,
    verdict_confirms: bool,
    timestamp_logical: u64,
) -> Result<TypedTx, AgentKeypairError> {
    let verifier_id = AgentId(verifier_agent.into());
    let tx_id = TxId(format!("verifytx-{}-{}", verifier_agent, suffix));
    let bond = StakeMicroCoin::from_micro_units(bond_micro);
    let verdict = if verdict_confirms {
        VerifyVerdict::Confirm
    } else {
        VerifyVerdict::Doubt
    };

    let payload = VerifySigningPayload {
        tx_id: tx_id.clone(),
        parent_state_root,
        target_work_tx: target_work_tx.clone(),
        verifier_agent: verifier_id.clone(),
        bond,
        verdict,
        timestamp_logical,
    };
    let digest = payload.canonical_digest();
    let signature = keypairs.sign(&verifier_id, digest)?;

    Ok(TypedTx::Verify(VerifyTx {
        tx_id,
        parent_state_root,
        target_work_tx,
        verifier_agent: verifier_id,
        bond,
        verdict,
        signature,
        timestamp_logical,
    }))
}

// ────────────────────────────────────────────────────────────────────────────
// TB-10 Atom 1 — Real-signature constructors for user-driven TaskOpen + EscrowLock.
//
// The synthetic constructors above use `AgentSignature::from_bytes([0u8; 64])`
// because the evaluator's preseed sponsor (`tb7-7-sponsor`) is not in the
// durable keystore — its ledger entries pre-date TB-7's Ed25519 wiring.
//
// TB-10 introduces a NEW caller class (a human user invoking `lean_market`)
// who DOES carry a durable Ed25519 keypair via TB-9's keystore (Agent_user_0).
// User-driven TaskOpen + EscrowLock SHOULD carry real signatures so the chain
// has cryptographic attestation of sponsor identity — even though the kernel
// dispatch arms (sequencer.rs:1054 + 1095) do not currently verify these
// fields. This is forward-compatible with future TB-12+ kernel hardening.
//
// Per TB-10 charter §2.1 + ratification §2.1.
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC1-N14: TB-10 Atom 1 — real-signature TaskOpenTx constructor.
///
/// Builds a `TaskOpenTx` and signs it via `AgentKeypairRegistry::sign(&sponsor, digest)`.
/// Mirrors `make_synthetic_task_open` shape but produces a non-zero Ed25519 signature
/// over `TaskOpenSigningPayload::canonical_digest()`.
#[allow(clippy::too_many_arguments)]
pub fn make_real_task_open_signed_by(
    keypairs: &mut AgentKeypairRegistry,
    task: &str,
    sponsor: &str,
    parent_state_root: Hash,
    suffix: &str,
    timestamp_logical: u64,
) -> Result<TypedTx, AgentKeypairError> {
    let sponsor_id = AgentId(sponsor.into());
    let task_id = TaskId(task.into());
    let tx_id = TxId(format!("taskopen-{}-{}", task, suffix));
    let payload = TaskOpenSigningPayload {
        tx_id: tx_id.clone(),
        task_id: task_id.clone(),
        parent_state_root,
        sponsor_agent: sponsor_id.clone(),
        verifier_quorum: 1,
        max_reuse_royalty_fraction_basis_points: 1000,
        settlement_rule_hash: Hash::ZERO,
        timestamp_logical,
    };
    let digest = payload.canonical_digest();
    let signature = keypairs.sign(&sponsor_id, digest)?;
    Ok(TypedTx::TaskOpen(TaskOpenTx {
        tx_id,
        task_id,
        parent_state_root,
        sponsor_agent: sponsor_id,
        verifier_quorum: 1,
        max_reuse_royalty_fraction_basis_points: 1000,
        settlement_rule_hash: Hash::ZERO,
        signature,
        timestamp_logical,
    }))
}

/// TRACE_MATRIX FC1-N14: TB-10 Atom 1 — real-signature EscrowLockTx constructor.
///
/// Builds an `EscrowLockTx` and signs it via `AgentKeypairRegistry::sign(&sponsor, digest)`.
/// Mirrors `make_synthetic_escrow_lock` shape but produces a non-zero Ed25519
/// signature over `EscrowLockSigningPayload::canonical_digest()`.
#[allow(clippy::too_many_arguments)]
pub fn make_real_escrow_lock_signed_by(
    keypairs: &mut AgentKeypairRegistry,
    task: &str,
    sponsor: &str,
    amount_micro: i64,
    parent_state_root: Hash,
    suffix: &str,
    timestamp_logical: u64,
) -> Result<TypedTx, AgentKeypairError> {
    let sponsor_id = AgentId(sponsor.into());
    let task_id = TaskId(task.into());
    let tx_id = TxId(format!("escrowlock-{}-{}", task, suffix));
    let amount = MicroCoin::from_micro_units(amount_micro);
    let payload = EscrowLockSigningPayload {
        tx_id: tx_id.clone(),
        task_id: task_id.clone(),
        parent_state_root,
        sponsor_agent: sponsor_id.clone(),
        amount,
        timestamp_logical,
    };
    let digest = payload.canonical_digest();
    let signature = keypairs.sign(&sponsor_id, digest)?;
    Ok(TypedTx::EscrowLock(EscrowLockTx {
        tx_id,
        task_id,
        parent_state_root,
        sponsor_agent: sponsor_id,
        amount,
        signature,
        timestamp_logical,
    }))
}

// ────────────────────────────────────────────────────────────────────────────
// TB-8 Atom 4 — Evaluator OMEGA-branch caller helper.
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX TB-8 charter §3 Atom 4 — block until a TxId is observed
/// in the chain via state_root advance.
///
/// **Why a separate helper**: the evaluator submits multiple txs in
/// sequence (e.g., WorkTx then VerifyTx). The sequencer is async — both
/// txs are queued, and the SECOND tx's `parent_state_root` was captured
/// BEFORE the first tx was dispatched. If the first tx commits between
/// queueing and dispatch, the second tx sees the OLD state_root and
/// is rejected with `StaleParent`.
///
/// This helper polls `state_root_t` until it advances past the supplied
/// pre-snapshot (or budget expires). Caller passes the pre-snapshot,
/// awaits this helper, then captures the new state_root for the next
/// tx's `parent_state_root` field.
///
/// Returns `Ok(new_state_root)` if state_root advanced; `Err(())` if the
/// budget expired without observation.
pub async fn tb8_await_state_root_advance(
    sequencer: &crate::state::sequencer::Sequencer,
    pre_state_root: crate::state::q_state::Hash,
    poll_budget_ms: u64,
) -> Result<crate::state::q_state::Hash, ()> {
    use std::time::{Duration, Instant};
    let deadline = Instant::now() + Duration::from_millis(poll_budget_ms);
    while Instant::now() < deadline {
        if let Ok(q) = sequencer.q_snapshot() {
            if q.state_root_t != pre_state_root {
                return Ok(q.state_root_t);
            }
        }
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
    Err(())
}

/// TRACE_MATRIX TB-8 charter §3 Atom 4 — emit FinalizeReward after an
/// OMEGA-Confirm VerifyTx commits.
///
/// **Why a poll-then-emit helper**: `bus.submit_typed_tx` queues; the
/// `Sequencer::run` driver applies asynchronously. To call
/// `emit_system_tx(SystemEmitCommand::FinalizeReward { claim_id })` we need
/// `claims_t[claim_id]` to be populated, which requires the just-submitted
/// VerifyTx to have been applied. We poll `q_snapshot` until the claim
/// appears, then emit. The poll budget defaults to 5s (mirrors the
/// pre-existing TaskOpen-poll pattern at `evaluator.rs:869-887`).
///
/// Returns:
/// - `Ok(true)` when the claim was found AND finalize was emitted.
/// - `Ok(false)` when the poll budget expired before the claim appeared
///   (caller logs but does NOT fail the run; FinalizeReward is best-effort
///   for solo-run MVP — the OMEGA path's L4 evidence is the durable signal).
/// - `Err(_)` when emit_system_tx returns an unexpected error (e.g.,
///   InvalidSystemSignatureLive — defense-in-depth).
///
/// Per ratification §1 Q3 zero-window MVP: no challenge window scheduling;
/// FinalizeReward becomes legal as soon as the claim exists.
pub async fn tb8_emit_finalize_after_verify(
    sequencer: &crate::state::sequencer::Sequencer,
    verify_tx_id: &TxId,
    poll_budget_ms: u64,
) -> Result<bool, crate::state::sequencer::EmitSystemError> {
    use std::time::{Duration, Instant};
    let claim_id_inner = TxId(format!("claim-{}", verify_tx_id.0));
    let claim_id = crate::state::typed_tx::ClaimId(claim_id_inner.clone());
    let deadline = Instant::now() + Duration::from_millis(poll_budget_ms);
    let mut found = false;
    while Instant::now() < deadline {
        if let Ok(q) = sequencer.q_snapshot() {
            if q.economic_state_t.claims_t.0.contains_key(&claim_id_inner) {
                found = true;
                break;
            }
        }
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
    if !found {
        return Ok(false);
    }
    sequencer
        .emit_system_tx(crate::state::sequencer::SystemEmitCommand::FinalizeReward {
            claim_id,
        })
        .await
        .map(|_| true)
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
