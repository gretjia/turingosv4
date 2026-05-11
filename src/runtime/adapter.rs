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
// TB-16 Atom 7 R1 Step 3 — Real-signature constructors for arena drivers
// (architect §7.3 FR-16.3 + FR-16.4 + sandbox forbidden-list compliance).
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC1-N36 (TB-16 arena driver): real-signature
/// `ChallengeTx` constructor signed by `challenger`.
#[allow(clippy::too_many_arguments)]
pub fn make_real_challengetx_signed_by(
    keypairs: &mut AgentKeypairRegistry,
    parent_state_root: Hash,
    target_work_tx: TxId,
    challenger: &str,
    stake_micro: i64,
    counterexample_cid: Cid,
    suffix: &str,
    timestamp_logical: u64,
) -> Result<TypedTx, AgentKeypairError> {
    use crate::state::typed_tx::{ChallengeSigningPayload, ChallengeTx};
    let challenger_id = AgentId(challenger.into());
    let tx_id = TxId(format!("challengetx-{}-{}", challenger, suffix));
    let stake = StakeMicroCoin::from_micro_units(stake_micro);

    let payload = ChallengeSigningPayload {
        tx_id: tx_id.clone(),
        parent_state_root,
        target_work_tx: target_work_tx.clone(),
        challenger_agent: challenger_id.clone(),
        stake,
        counterexample_cid,
        timestamp_logical,
    };
    let digest = payload.canonical_digest();
    let signature = keypairs.sign(&challenger_id, digest)?;

    Ok(TypedTx::Challenge(ChallengeTx {
        tx_id,
        parent_state_root,
        target_work_tx,
        challenger_agent: challenger_id,
        stake,
        counterexample_cid,
        signature,
        timestamp_logical,
    }))
}

/// TRACE_MATRIX FC1-N36 (TB-16 arena driver): real-signature
/// `MarketSeedTx` constructor — Agent_user_0 boots the
/// CompleteSet inventory at task entry (FR-16.4).
#[allow(clippy::too_many_arguments)]
pub fn make_real_market_seed_signed_by(
    keypairs: &mut AgentKeypairRegistry,
    parent_state_root: Hash,
    event_task: &str,
    provider: &str,
    collateral_amount_micro: i64,
    suffix: &str,
    timestamp_logical: u64,
) -> Result<TypedTx, AgentKeypairError> {
    use crate::state::typed_tx::{
        EventId, MarketSeedSigningPayload, MarketSeedTx,
    };
    let provider_id = AgentId(provider.into());
    let tx_id = TxId(format!("marketseedtx-{}-{}", provider, suffix));
    let event_id = EventId(crate::state::q_state::TaskId(event_task.into()));
    let collateral_amount = MicroCoin::from_micro_units(collateral_amount_micro);

    let payload = MarketSeedSigningPayload {
        tx_id: tx_id.clone(),
        parent_state_root,
        event_id: event_id.clone(),
        provider: provider_id.clone(),
        collateral_amount,
        timestamp_logical,
    };
    let digest = payload.canonical_digest();
    let signature = keypairs.sign(&provider_id, digest)?;

    Ok(TypedTx::MarketSeed(MarketSeedTx {
        tx_id,
        parent_state_root,
        event_id,
        provider: provider_id,
        collateral_amount,
        signature,
        timestamp_logical,
    }))
}

/// TRACE_MATRIX FC1-N36 (TB-16 arena driver): real-signature
/// `CompleteSetMintTx` constructor — owner mints 1 Coin → 1 YES + 1 NO
/// shares against the event collateral pool (FR-16.4).
#[allow(clippy::too_many_arguments)]
pub fn make_real_complete_set_mint_signed_by(
    keypairs: &mut AgentKeypairRegistry,
    parent_state_root: Hash,
    event_task: &str,
    owner: &str,
    amount_micro: i64,
    suffix: &str,
    timestamp_logical: u64,
) -> Result<TypedTx, AgentKeypairError> {
    use crate::state::typed_tx::{
        CompleteSetMintSigningPayload, CompleteSetMintTx, EventId,
    };
    let owner_id = AgentId(owner.into());
    let tx_id = TxId(format!("csmint-{}-{}", owner, suffix));
    let event_id = EventId(crate::state::q_state::TaskId(event_task.into()));
    let amount = MicroCoin::from_micro_units(amount_micro);

    let payload = CompleteSetMintSigningPayload {
        tx_id: tx_id.clone(),
        parent_state_root,
        event_id: event_id.clone(),
        owner: owner_id.clone(),
        amount,
        timestamp_logical,
    };
    let digest = payload.canonical_digest();
    let signature = keypairs.sign(&owner_id, digest)?;

    Ok(TypedTx::CompleteSetMint(CompleteSetMintTx {
        tx_id,
        parent_state_root,
        event_id,
        owner: owner_id,
        amount,
        signature,
        timestamp_logical,
    }))
}

/// TRACE_MATRIX FC1-N36 (TB-16.x.2.3 arena driver): real-signature
/// `CompleteSetRedeemTx` constructor — owner redeems winning-side
/// shares 1:1 against the resolved event collateral pool (FR-13.4..5;
/// SG-16.x.2.3). Mirrors `make_real_complete_set_mint_signed_by` shape.
///
/// Sequencer (TB-13 Atom 2 admission, sequencer.rs:1736):
/// 1. event must be Finalized (Yes wins) or Bankrupt (No wins).
/// 2. owner's winning-side share balance ≥ share_units.
/// 3. event collateral ≥ share_units.
///
/// Effect: 1 winning share = 1 MicroCoin (architect §4.3 verbatim).
#[allow(clippy::too_many_arguments)]
pub fn make_real_complete_set_redeem_signed_by(
    keypairs: &mut AgentKeypairRegistry,
    parent_state_root: Hash,
    event_task: &str,
    owner: &str,
    outcome: crate::state::typed_tx::OutcomeSide,
    share_units: u128,
    suffix: &str,
    timestamp_logical: u64,
) -> Result<TypedTx, AgentKeypairError> {
    use crate::state::typed_tx::{
        CompleteSetRedeemSigningPayload, CompleteSetRedeemTx, EventId, ShareAmount,
    };
    let owner_id = AgentId(owner.into());
    let tx_id = TxId(format!("csredeem-{}-{}", owner, suffix));
    let event_id = EventId(crate::state::q_state::TaskId(event_task.into()));
    let share_amount = ShareAmount::from_units(share_units);

    let payload = CompleteSetRedeemSigningPayload {
        tx_id: tx_id.clone(),
        parent_state_root,
        event_id: event_id.clone(),
        owner: owner_id.clone(),
        outcome,
        share_amount,
        timestamp_logical,
    };
    let digest = payload.canonical_digest();
    let signature = keypairs.sign(&owner_id, digest)?;

    Ok(TypedTx::CompleteSetRedeem(CompleteSetRedeemTx {
        tx_id,
        parent_state_root,
        event_id,
        owner: owner_id,
        outcome,
        share_amount,
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

/// TRACE_MATRIX TB-N2 B2 (TB_N2_POLYMARKET_CPMM_LIFECYCLE charter §3 B2;
/// 2026-05-11) — emit an `EventResolveTx` after a successful
/// `FinalizeReward` to flip `task_markets_t[task_id].state` from Open →
/// Finalized.
///
/// **Why a poll-then-emit helper**: mirrors `tb8_emit_finalize_after_verify`.
/// The just-emitted `FinalizeRewardTx` queues; the `Sequencer::run` driver
/// applies asynchronously. The B2 emit must capture a `parent_state_root`
/// that matches the apply-time `state_root_t` — otherwise dispatch
/// rejects with `StaleParent`. Polling `task_markets_t[task_id].state ==
/// Open` is insufficient (FinalizeReward does NOT touch task_markets_t),
/// so we ALSO poll `claims_t[claim_id].status == Finalized` to witness
/// that the FinalizeReward dispatch arm has applied (advancing
/// `state_root_t`).
///
/// **R2 race fix (R1 audit Q8 VETO closure 2026-05-11)**: prior R1 helper
/// polled only `task_markets_t.state == Open` and emitted EventResolve
/// immediately. Since FinalizeReward applies asynchronously AFTER tb8
/// helper returns Ok (tb8 returns on `emit_system_tx` Ok at
/// `adapter.rs:638`, NOT on apply), the EventResolve construction
/// captured a pre-FinalizeReward `parent_state_root R_0`. By apply-time
/// the state_root had advanced to R_1 (FinalizeReward applied first),
/// and dispatch Step-0 parent-root check rejected with `StaleParent` →
/// L4.E `stale_parent_root`. Smoke evidence: cell 2 rejections.jsonl
/// entry 9 at `handover/evidence/stage_b3_smoke_b2_20260511T012401Z/`
/// (Codex G2 R1 Q8 VETO 2026-05-11). R2 fix: caller passes
/// `verify_tx_id` so we derive `claim_id = "claim-{verify_tx_id}"`
/// (mirrors tb8 helper's `claim_id_inner` at
/// `tb8_emit_finalize_after_verify:622`), then poll
/// `claims_t[claim_id].status == Finalized` ALONGSIDE the existing
/// `task_markets_t.state == Open` poll BEFORE emitting EventResolve.
/// Once BOTH conditions are met, the FinalizeReward apply has completed
/// (state_root advanced) and the subsequent `emit_system_tx` captures
/// the post-FinalizeReward state_root, so apply-time parent-root check
/// passes.
///
/// **Resolution authority** (Option 1 per charter §5): the FinalizeReward
/// just-emitted IS the resolution evidence. No external oracle required;
/// proof-task acceptance = market resolves YES per architect Part C §2.1
/// + TB-13 redeem mapping (typed_tx.rs:1244 `Finalized → Yes wins`).
///
/// Returns:
/// - `Ok(true)` when the EventResolve was emitted successfully.
/// - `Ok(false)` when the poll budget expired before BOTH conditions
///   (claim Finalized + task still Open) were observed, OR task is
///   already non-Open (resolved/bankrupt/expired — emit would reject).
///   Caller logs but does NOT fail the run; matches the
///   `tb8_emit_finalize_after_verify` best-effort pattern for solo-run
///   MVP.
/// - `Err(_)` when `emit_system_tx` returns an unexpected error (e.g.,
///   `EventResolveTaskNotFound` — defense-in-depth).
pub async fn tb_n2_emit_event_resolve_after_finalize(
    sequencer: &crate::state::sequencer::Sequencer,
    task_id: TaskId,
    verify_tx_id: &TxId,
    poll_budget_ms: u64,
) -> Result<bool, crate::state::sequencer::EmitSystemError> {
    use std::time::{Duration, Instant};
    // R2 race fix: derive claim_id from verify_tx_id (mirrors
    // `tb8_emit_finalize_after_verify`'s `claim_id_inner` at
    // `adapter.rs:622`). The FinalizeReward dispatch arm flips
    // `claims_t[claim_id].status` Open → Finalized at the dispatch
    // arm in `sequencer.rs`; that flip is the witness that
    // FinalizeReward has applied and `state_root_t` has advanced.
    let claim_id_inner = TxId(format!("claim-{}", verify_tx_id.0));
    let deadline = Instant::now() + Duration::from_millis(poll_budget_ms);
    let mut both_ready = false;
    while Instant::now() < deadline {
        if let Ok(q) = sequencer.q_snapshot() {
            // Witness FinalizeReward apply via claim.status == Finalized.
            // NOT just claim presence — claims_t insert happens at TB-8
            // Atom 1 OMEGA-Confirm VerifyTx accept, which precedes the
            // FinalizeReward dispatch arm we are waiting for.
            let claim_finalized = q
                .economic_state_t
                .claims_t
                .0
                .get(&claim_id_inner)
                .map(|c| {
                    matches!(c.status, crate::state::q_state::ClaimStatus::Finalized)
                })
                .unwrap_or(false);
            if let Some(tm) = q.economic_state_t.task_markets_t.0.get(&task_id) {
                match tm.state {
                    crate::state::q_state::TaskMarketState::Open => {
                        // R2: ALSO require claim Finalized — otherwise
                        // we race FinalizeReward apply and capture a
                        // stale state_root, producing StaleParent L4.E.
                        if claim_finalized {
                            both_ready = true;
                            break;
                        }
                        // else fall through to sleep + retry
                    }
                    // Already resolved/bankrupt/expired — B2 emit would
                    // reject as EventAlreadyResolved. Treat as
                    // "nothing to do"; mirrors
                    // `tb8_emit_finalize_after_verify`'s best-effort
                    // idempotent contract.
                    crate::state::q_state::TaskMarketState::Finalized
                    | crate::state::q_state::TaskMarketState::Bankrupt
                    | crate::state::q_state::TaskMarketState::Expired => {
                        return Ok(false);
                    }
                }
            }
        }
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
    if !both_ready {
        return Ok(false);
    }
    sequencer
        .emit_system_tx(crate::state::sequencer::SystemEmitCommand::EventResolve {
            task_id,
        })
        .await
        .map(|_| true)
}

// ────────────────────────────────────────────────────────────────────────────
// TB-11 Atom 4 — Runtime emission helpers (architect §6.2 ruling 2026-05-02)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX TB-11 Atom 4 (architect §6.2): emit a TerminalSummaryTx
/// (≡ RunExhaustedTx) for a failed evaluator run. Caller passes the
/// run-summary fields directly + an optional pre-written
/// `evidence_capsule_cid` (callers should write the EvidenceCapsule first
/// via `evidence_capsule::write_evidence_capsule`).
///
/// Returns `Ok(receipt)` on success; `Err` carries the same EmitSystemError
/// taxonomy as `tb8_emit_finalize_after_verify`.
///
/// **Architect mandate** (TB-11 charter §3 Atom 4 + ship gate G4):
/// evaluator on MAX_TX exhausted / timeout / solver give-up should:
///   1. Build EvidenceCapsule via `evidence_capsule::write_evidence_capsule`.
///   2. Call this helper with `evidence_capsule_cid = Some(capsule.capsule_id)`.
///
/// For OmegaAccepted runs, evidence_capsule_cid is `None` (success path
/// has no failure evidence).
pub async fn tb11_emit_terminal_summary_for_run(
    sequencer: &crate::state::sequencer::Sequencer,
    run_id: crate::state::typed_tx::RunId,
    task_id: TaskId,
    run_outcome: crate::state::typed_tx::RunOutcome,
    total_attempts: u32,
    failure_class_histogram:
        std::collections::BTreeMap<crate::state::typed_tx::RejectionClass, u32>,
    last_logical_t: u64,
    solver_agent: Option<AgentId>,
    evidence_capsule_cid: Option<crate::bottom_white::cas::schema::Cid>,
) -> Result<crate::state::sequencer::SystemEmitReceipt, crate::state::sequencer::EmitSystemError> {
    sequencer
        .emit_system_tx(crate::state::sequencer::SystemEmitCommand::TerminalSummary {
            run_id,
            task_id,
            run_outcome,
            total_attempts,
            failure_class_histogram,
            last_logical_t,
            solver_agent,
            evidence_capsule_cid,
        })
        .await
}

/// TRACE_MATRIX TB-11 Atom 4 (architect §6.2 + §7.4 capital-must-flow):
/// scan task_markets_t for tasks past the expiry-policy deadline + emit
/// TaskExpire for each eligible escrow.
///
/// Eligibility (TB-11 MVP per charter §7 Q1):
///   - task_markets_t[task_id].state ∈ { Open, Bankrupt }
///   - current_logical_t - opened_at_logical_t > expiry_delta_logical_t
///   - no Finalized claim against this task
///   - (no open challenge_cases targeting this task — enforced by
///     dispatch arm; helper does not pre-filter to keep policy logic
///     centralized)
///
/// For each eligible (task_id, escrow_tx_id) pair, emits one
/// TaskExpireTx via `emit_system_tx`. Returns the count of expirations
/// emitted + the total micro-coin refunded.
///
/// **Reason policy**:
///   - state == Open      → ExpireReason::Deadline
///   - state == Bankrupt  → ExpireReason::BankruptcyTriggered
///
/// Returns Ok((count, total_micro)) on success.
pub async fn tb11_emit_expire_for_eligible(
    sequencer: &crate::state::sequencer::Sequencer,
    current_logical_t: u64,
    expiry_delta_logical_t: u64,
) -> Result<(u32, i64), crate::state::sequencer::EmitSystemError> {
    let q = match sequencer.q_snapshot() {
        Ok(q) => q,
        Err(_) => return Err(crate::state::sequencer::EmitSystemError::InternalLockPoisoned),
    };

    // Pre-collect candidates so we can drop the q_snapshot before emitting
    // (avoid holding a snapshot view across the await boundary).
    use crate::state::q_state::TaskMarketState;
    let mut candidates: Vec<(
        TaskId,
        TxId,
        crate::state::typed_tx::ExpireReason,
    )> = Vec::new();
    for (task_id, entry) in q.economic_state_t.task_markets_t.0.iter() {
        // Skip terminal states.
        let reason = match entry.state {
            TaskMarketState::Open => crate::state::typed_tx::ExpireReason::Deadline,
            TaskMarketState::Bankrupt => {
                crate::state::typed_tx::ExpireReason::BankruptcyTriggered
            }
            TaskMarketState::Expired | TaskMarketState::Finalized => continue,
        };

        // Deadline policy.
        let elapsed = current_logical_t.saturating_sub(entry.opened_at_logical_t);
        if elapsed <= expiry_delta_logical_t {
            continue;
        }

        // No Finalized claim against this task.
        let has_finalized = q.economic_state_t.claims_t.0.values().any(|c| {
            c.task_id == *task_id
                && c.status == crate::state::q_state::ClaimStatus::Finalized
        });
        if has_finalized {
            continue;
        }

        // For each escrow row contributing to this task, queue an expiry.
        for escrow_tx_id in entry.escrow_lock_tx_ids.iter() {
            candidates.push((task_id.clone(), escrow_tx_id.clone(), reason));
        }
    }
    drop(q);

    let mut count: u32 = 0;
    let mut total_refunded: i64 = 0;
    for (task_id, escrow_tx_id, reason) in candidates {
        // Q-derive the refund amount from current escrows_t (defensive
        // re-read after each emit; total_refunded reflects what would be
        // refunded if the dispatch arm proceeds from the current Q).
        if let Ok(q_now) = sequencer.q_snapshot() {
            if let Some(esc) = q_now.economic_state_t.escrows_t.0.get(&escrow_tx_id) {
                total_refunded += esc.amount.micro_units();
            }
        }
        match sequencer
            .emit_system_tx(crate::state::sequencer::SystemEmitCommand::TaskExpire {
                task_id,
                escrow_tx_id,
                reason,
            })
            .await
        {
            Ok(_) => count += 1,
            // ClaimNotFound here means escrow was concurrently consumed; skip.
            Err(crate::state::sequencer::EmitSystemError::ClaimNotFound) => continue,
            Err(other) => return Err(other),
        }
    }
    Ok((count, total_refunded))
}

/// TRACE_MATRIX TB-16.x.2.2 (umbrella charter §2 Atom 2.2 + FR-16.3 challenge
/// tx fired): scheduler-tick over open challenge cases. Emits one
/// ChallengeResolveTx per eligible (Open, past-window) case.
///
/// **Eligibility** (post-zero-window MVP; per charter §2):
///   - challenge_cases_t[case_id].status == Open
///   - q.q_t.current_round - case.opened_at_round >= window_delta_logical_t
///
/// `window_delta_logical_t = 0` makes every Open case immediately eligible
/// (the FORCE_CHALLENGE_RESOLVE arena profile uses this). Note: differs in
/// time-unit from `tb11_emit_expire_for_eligible(.., expiry_delta=0)` —
/// tb11 uses caller-supplied `current_logical_t` (auto-advances per-tx);
/// this helper uses `q.q_t.current_round` (NOT auto-advanced per-tx — the
/// only in-tree mutator at HEAD is the `seed_q_for_challenge` test helper
/// at `src/state/sequencer.rs:~4185`; no production round-advance mechanism
/// is wired yet). Hence the `>= delta` boundary, NOT `> delta`
/// (TB-16.x.2.2.fix Patch E 2026-05-05; doc-only follow-up TB-16.x.2.2.fix.r2
/// Patch F4 2026-05-05 corrects the prior docstring's reference to a
/// nonexistent `Sequencer::set_current_round_for_test` symbol).
///
/// `default_resolution` selects the policy applied to every eligible case.
/// `Released` (the charter default) refunds the challenger bond + flips
/// status. `UpheldDeferred` is marker-only (bond preserved for TB-6 RSP-3.2
/// slash routing); use only when policy upstream determined the challenge
/// has merit. The helper does NOT decide per-case policy — caller picks one.
///
/// Returns `Ok((count, bonds_released_micro))` on success — `count` is the
/// number of resolves emitted; `bonds_released_micro` is the sum of bonds
/// returned to challengers under `Released` (zero under `UpheldDeferred`).
pub async fn tb16_emit_challenge_resolve_for_eligible(
    sequencer: &crate::state::sequencer::Sequencer,
    window_delta_logical_t: u64,
    default_resolution: crate::state::typed_tx::ChallengeResolution,
) -> Result<(u32, i64), crate::state::sequencer::EmitSystemError> {
    use crate::state::q_state::ChallengeStatus;
    let q = match sequencer.q_snapshot() {
        Ok(q) => q,
        Err(_) => return Err(crate::state::sequencer::EmitSystemError::InternalLockPoisoned),
    };

    // Pre-collect candidates so we can drop the q_snapshot before emitting
    // (mirror tb11_emit_expire_for_eligible — avoid holding snapshot across
    // await boundary). bonds_planned is the per-case bond amount as
    // observed in the snapshot; total_planned tracks the planned refund
    // sum under Released. UpheldDeferred contributes 0.
    let current_round = q.q_t.current_round;
    let mut candidates: Vec<(crate::state::q_state::TxId, i64)> = Vec::new();
    for (case_id, case) in q.economic_state_t.challenge_cases_t.0.iter() {
        if case.status != ChallengeStatus::Open {
            continue;
        }
        let elapsed = current_round.saturating_sub(case.opened_at_round);
        // TB-16.x.2.2.fix Patch E: was `elapsed <= window_delta_logical_t` →
        // `< window_delta_logical_t`. Original semantic required elapsed > delta,
        // i.e. delta=0 demanded ≥1 round to pass — but `current_round` is NOT
        // auto-advanced per-tx (unlike `current_logical_t` used by
        // tb11_emit_expire_for_eligible). On the OMEGA-Confirm path the case
        // is opened and scanned within the same round, so `delta=0` skipped
        // every case — contradicting the docstring claim "delta=0 makes every
        // Open case immediately eligible". Change to `< delta` makes delta=0
        // include elapsed=0 (immediate eligibility, matching docstring intent
        // and the FORCE_CHALLENGE_RESOLVE arena profile's expectation). For
        // delta≥1 the boundary shifts by one round, which the FORCE_-prefixed
        // arena hooks are the sole callers of today (no production caller
        // depends on the prior off-by-one boundary).
        if elapsed < window_delta_logical_t {
            continue;
        }
        candidates.push((case_id.clone(), case.bond.micro_units()));
    }
    drop(q);

    // emit_system_tx for ChallengeResolve does NOT pre-check case existence
    // / status (see sequencer.rs:2590 — construction is unconditional). A
    // dispatch-time AlreadyResolved (sequencer.rs:1432) or ChallengeNotFound
    // surfaces as a rejection on the L4.E ledger, NOT as an Err here. So
    // there is no skip-pattern parallel to tb11_emit_expire_for_eligible's
    // ClaimNotFound — emit_system_tx errors here are construction failures
    // (queue full, internal lock poisoned, signature construction) and
    // propagate as-is.
    let mut count: u32 = 0;
    let mut bonds_released_micro: i64 = 0;
    let releasing = matches!(
        default_resolution,
        crate::state::typed_tx::ChallengeResolution::Released
    );
    for (case_id, planned_bond_micro) in candidates {
        sequencer
            .emit_system_tx(crate::state::sequencer::SystemEmitCommand::ChallengeResolve {
                target_challenge_tx_id: case_id,
                resolution: default_resolution,
            })
            .await?;
        count += 1;
        if releasing {
            bonds_released_micro += planned_bond_micro;
        }
    }
    Ok((count, bonds_released_micro))
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
