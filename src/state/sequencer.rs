//! L4 Sequencer + dispatch_transition (CO1.7-impl A2 + A3).
//!
//! Spec authority:
//! - `handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md` § 3 (Sequencer
//!   pseudocode, K1 dual-counter, K3 head_t deferred, C3 sign API)
//! - `handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md` § 8
//!   (dispatch_transition exhaustive enum match; K5 Slash dropped)
//!
//! Single-writer per (runtime_repo, run_id). Per spec § 5.2.1.
//!
//! **Stub state (this atom)**: every per-kind transition returns
//! `TransitionError::NotYetImplemented`; CO1.7.5 (downstream atom) fills the
//! bodies. The structural correctness of the apply path (snapshot → dispatch →
//! CAS put → sign → root fold → commit → Q_t mutation) is locked by the
//! impl + tests here; what's left is per-kind transition logic.
//!
//! /// TRACE_MATRIX § 5.2.1 + § 8 — L4 sequencer single-writer + dispatch.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};

use sha2::{Digest, Sha256};

use crate::bottom_white::cas::schema::{Cid, ObjectType};
use crate::bottom_white::cas::store::{CasError, CasStore};
use crate::bottom_white::ledger::system_keypair::{
    transition_ledger_emitter, Ed25519Keypair, KeypairError, SystemEpoch,
};
use crate::bottom_white::ledger::rejection_evidence::{
    RejectionClass as L4ERejectionClass, RejectionEvidenceWriter,
};
use crate::bottom_white::ledger::transition_ledger::{
    append, canonical_encode, LedgerEntry, LedgerEntrySigningPayload, LedgerWriter,
    LedgerWriterError,
};
use crate::bottom_white::tools::registry::ToolRegistry;
use crate::economy::monetary_invariant::{
    assert_no_post_init_mint, assert_read_is_free,
    assert_task_market_total_escrow_matches_locks, assert_total_ctf_conserved,
};
use crate::state::q_state::{AgentId, EscrowEntry, Hash, QState, TaskMarketEntry, TxId};
use crate::state::typed_tx::{HasSubmitter, SignalBundle, TransitionError, TypedTx};
use std::collections::BTreeSet;
use crate::top_white::predicates::registry::PredicateRegistry;

// ────────────────────────────────────────────────────────────────────────────
// TB-2 — WorkTx-accept state-root domain (preflight v3 §3.4 + P1-1 r2)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC3-S3: TB-2 interim WorkTx-accept state-root domain.
///
/// Real patch semantics for `q_next.state_root_t` land in P5; until then
/// TB-2 advances the state root deterministically with this domain string
/// concatenated against `q.state_root_t` and the canonical hash of the
/// accepted WorkTx. Distinct from the TB-1 toy domain
/// `b"turingosv4.l4_state_root.v1"` used by `AcceptedLedger` at
/// `src/economy/ledger.rs:350, :357` (TB-1 RSP-0 primitive vs production
/// state-root mutator separation).
pub(crate) const WORKTX_ACCEPT_DOMAIN_V1: &[u8] = b"turingosv4.worktx.accept.v1";

/// TRACE_MATRIX FC3-S3: TB-2 canonical hash helper for a `TypedTx`.
///
/// Defined locally (not in `bottom_white::ledger::transition_ledger`) because
/// `canonical_hash(tx)` is NOT a generic existing helper there — only
/// `canonical_encode` is — and TB-2 wants a single short call site that
/// includes domain separation. Codex r2 P1-2.
pub(crate) fn worktx_canonical_hash(tx: &TypedTx) -> Hash {
    let mut h = Sha256::new();
    h.update(b"turingosv4.worktx.canonical_hash.v1");
    h.update(canonical_encode(tx).expect("TypedTx is canonical-encodable"));
    let digest: [u8; 32] = h.finalize().into();
    Hash::from_bytes(digest)
}

/// TRACE_MATRIX FC3-S3: TB-2 interim state-root mutator on WorkTx accept.
///
/// `q_next.state_root_t = sha256(WORKTX_ACCEPT_DOMAIN_V1 ‖ q.state_root_t.0
/// ‖ worktx_canonical_hash(tx).0)`. P5 replaces this with real patch
/// semantics; until then this is the deterministic monotonic mutation
/// asserted by U3 / I9.
///
/// Public single-item surface for the TB-2 accept-side state-root contract.
/// Integration tests in `tests/tb_2_runtime_boundary.rs` (e.g. I9) use this
/// helper directly to recompute the expected post-accept hash WITHOUT
/// re-implementing the WORKTX_ACCEPT_DOMAIN_V1 / worktx_canonical_hash
/// composition by hand. The composing primitives stay `pub(crate)` so the
/// public surface is a single semantic helper, not the raw building blocks
/// (Phase-1c r1 Codex P0-1 remediation).
pub fn worktx_accept_state_root(prev: &Hash, tx: &TypedTx) -> Hash {
    let work_digest = worktx_canonical_hash(tx);
    let mut h = Sha256::new();
    h.update(WORKTX_ACCEPT_DOMAIN_V1);
    h.update(prev.0);
    h.update(work_digest.0);
    let digest: [u8; 32] = h.finalize().into();
    Hash::from_bytes(digest)
}

// ────────────────────────────────────────────────────────────────────────────
// TB-3 RSP-1 — TaskOpen + EscrowLock state-root domains (charter § 4.3)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX TB-3 charter § 4.3 — TaskOpen-accept state-root domain.
pub(crate) const TASK_OPEN_DOMAIN_V1: &[u8] = b"turingosv4.task_open.accept.v1";

/// TRACE_MATRIX TB-3 charter § 4.3 — EscrowLock-accept state-root domain.
pub(crate) const ESCROW_LOCK_DOMAIN_V1: &[u8] = b"turingosv4.escrow_lock.accept.v1";

/// TRACE_MATRIX TB-3 charter § 4.3 — interim state-root mutator on
/// `TaskOpenTx` accept. Mirror of `worktx_accept_state_root` with its own
/// domain prefix for SHA-256 input separation. Real patch semantics for
/// `q_next.state_root_t` land in P5; until then this is the deterministic
/// monotonic mutation. Public single-item surface for integration tests
/// to recompute the expected post-accept hash without re-implementing
/// the domain composition.
pub fn task_open_accept_state_root(prev: &Hash, tx: &TypedTx) -> Hash {
    let mut h = Sha256::new();
    h.update(TASK_OPEN_DOMAIN_V1);
    h.update(prev.0);
    h.update(canonical_encode(tx).expect("TypedTx is canonical-encodable"));
    let digest: [u8; 32] = h.finalize().into();
    Hash::from_bytes(digest)
}

/// TRACE_MATRIX TB-3 charter § 4.3 — interim state-root mutator on
/// `EscrowLockTx` accept. Mirror of `task_open_accept_state_root`.
pub fn escrow_lock_accept_state_root(prev: &Hash, tx: &TypedTx) -> Hash {
    let mut h = Sha256::new();
    h.update(ESCROW_LOCK_DOMAIN_V1);
    h.update(prev.0);
    h.update(canonical_encode(tx).expect("TypedTx is canonical-encodable"));
    let digest: [u8; 32] = h.finalize().into();
    Hash::from_bytes(digest)
}

// ────────────────────────────────────────────────────────────────────────────
// TB-4 RSP-2 — Verify + Challenge state-root domains (charter § 4.3)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX TB-4 charter § 4.3 — Verify-accept state-root domain.
pub(crate) const VERIFY_ACCEPT_DOMAIN_V1: &[u8] = b"turingosv4.verify.accept.v1";

/// TRACE_MATRIX TB-4 charter § 4.3 — Challenge-accept state-root domain.
pub(crate) const CHALLENGE_ACCEPT_DOMAIN_V1: &[u8] = b"turingosv4.challenge.accept.v1";

/// TRACE_MATRIX TB-4 charter § 4.3 — interim state-root mutator on
/// `VerifyTx` accept. Mirror of `task_open_accept_state_root` shape.
/// Public single-item surface for integration tests to recompute the
/// expected post-accept hash.
pub fn verify_accept_state_root(prev: &Hash, tx: &TypedTx) -> Hash {
    let mut h = Sha256::new();
    h.update(VERIFY_ACCEPT_DOMAIN_V1);
    h.update(prev.0);
    h.update(canonical_encode(tx).expect("TypedTx is canonical-encodable"));
    let digest: [u8; 32] = h.finalize().into();
    Hash::from_bytes(digest)
}

/// TRACE_MATRIX TB-4 charter § 4.3 — interim state-root mutator on
/// `ChallengeTx` accept. Mirror of `verify_accept_state_root`.
pub fn challenge_accept_state_root(prev: &Hash, tx: &TypedTx) -> Hash {
    let mut h = Sha256::new();
    h.update(CHALLENGE_ACCEPT_DOMAIN_V1);
    h.update(prev.0);
    h.update(canonical_encode(tx).expect("TypedTx is canonical-encodable"));
    let digest: [u8; 32] = h.finalize().into();
    Hash::from_bytes(digest)
}

// ────────────────────────────────────────────────────────────────────────────
// TB-2 Atom 4 — rejection-path helpers (preflight v3 §3.5 + §3.7)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC3-S3: TB-2 sentinel `agent_id` for rejected submissions
/// whose `HasSubmitter::submitter_id()` returns `None` (system-emitted
/// variants — none on the WorkTx arm in TB-2; reserved for future TBs).
///
/// `RejectedSubmissionRecord.agent_id: AgentId` (NOT `Option<AgentId>`) per
/// `rejection_evidence.rs:90`. The string content is internal-only and never
/// crosses the agent boundary — only `public_summary` does, per `:89-90`.
pub(crate) const SYSTEM_AGENT_ID_STR: &str = "__system__";

/// TRACE_MATRIX FC3-S3: TB-2 `TransitionError → L4ERejectionClass` mapping
/// (preflight v3 §3.7). Closed by enumeration via the documented table even
/// though the `match` uses `_` for the 19-variant tail: WorkTx-arm-reachable
/// variants are explicit; non-WorkTx-arm variants fall through to
/// `PolicyViolation` per Codex r2 P0-4 sanction.
fn rejection_class_for(e: &TransitionError) -> L4ERejectionClass {
    use TransitionError as TE;
    use L4ERejectionClass as RC;
    match e {
        TE::AcceptancePredicateFailed(_)
        | TE::VerificationPredicateFailed(_)
        | TE::SettlementPredicateFailed(_) => RC::PredicateFailed,
        TE::EscrowMissing => RC::EscrowMissing,
        TE::MonetaryInvariantViolation => RC::InvariantViolation,
        // TB-3 RSP-1 formal-tx-surface mapping (charter § 4.5):
        TE::TaskAlreadyOpen => RC::PolicyViolation,
        // TB-3 charter § 4.5: TaskNotOpen reuses EscrowMissing semantically —
        // "no open task = no funded admission path".
        TE::TaskNotOpen => RC::EscrowMissing,
        // TB-3 charter § 4.5 + § 3.5: InsufficientBalance is its OWN L4E class
        // (do NOT fold into PolicyViolation — P4 Information Loom needs the
        // discriminator).
        TE::InsufficientBalance => RC::InsufficientBalance,
        // TB-4 RSP-2 admission mapping (charter § 4.5; directive Q3 + Q7).
        // All 3 new TB-4 variants + 2 reserved variants cluster under
        // PolicyViolation at the L4ERejectionClass coarser tier; finer-grained
        // TransitionError variant name is recoverable from the L4.E
        // raw_diagnostic_cid CAS payload (preflight § 8 Q2).
        TE::BondInsufficient => RC::PolicyViolation,
        TE::TargetWorkInactive => RC::PolicyViolation,
        TE::EmptyCounterexample => RC::PolicyViolation,
        TE::TargetWorkTxNotFound => RC::PolicyViolation,
        TE::TargetWorkTxNotVerifiable => RC::PolicyViolation,
        // Non-WorkTx-arm variants documented per §3.7 mapping table — should
        // not occur on the WorkTx arm; conservative sentinel preserves L4.E
        // append correctness if a future TB adds new variants.
        _ => RC::PolicyViolation,
    }
}

/// TRACE_MATRIX FC3-S3: TB-2 agent-facing summary string for an L4.E record.
///
/// Returns a small, predicate-id-stripped class label so private predicate
/// identities never leak (TB-1 §1.4 "Opaque" discipline). The wildcard arm
/// matches the §3.7 mapping policy and is the documented sentinel for
/// non-WorkTx-arm variants per Codex r2 P0-4.
fn public_summary_for(e: &TransitionError) -> Option<String> {
    match e {
        TransitionError::StaleParent => Some("stale_parent_root".into()),
        TransitionError::StakeInsufficient => Some("stake_insufficient".into()),
        TransitionError::EscrowMissing => Some("escrow_missing".into()),
        TransitionError::MonetaryInvariantViolation => Some("monetary_invariant".into()),
        TransitionError::AcceptancePredicateFailed(_)
        | TransitionError::SettlementPredicateFailed(_) => Some("predicate_failed".into()),
        // TB-3 RSP-1 formal-tx-surface (charter § 4.5).
        TransitionError::TaskAlreadyOpen => Some("task_already_open".into()),
        TransitionError::TaskNotOpen => Some("task_not_open".into()),
        TransitionError::InsufficientBalance => Some("insufficient_balance".into()),
        // TB-4 RSP-2 admission (charter § 4.5; directive Q3 + Q7).
        TransitionError::BondInsufficient => Some("bond_insufficient".into()),
        TransitionError::TargetWorkInactive => Some("target_work_inactive".into()),
        TransitionError::EmptyCounterexample => Some("empty_counterexample".into()),
        TransitionError::TargetWorkTxNotFound => Some("target_work_not_found".into()),
        TransitionError::TargetWorkTxNotVerifiable => Some("target_work_not_verifiable".into()),
        _ => Some("policy_violation".into()),
    }
}

// ────────────────────────────────────────────────────────────────────────────
// § 8 dispatch_transition — exhaustive enum match (K5: NO Slash)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX § 8 — exhaustive dispatch over `TypedTx` variants.
///
/// **Stub state (CO1.7-impl A3)**: every variant returns
/// `TransitionError::NotYetImplemented`. CO1.7.5 fills each arm with the real
/// transition body per `STATE_TRANSITION_SPEC § 3.1-3.7`. The exhaustive match
/// itself is the contract: any future TypedTx variant addition triggers a
/// non-exhaustive-match compile error here, forcing explicit handling.
pub(crate) fn dispatch_transition(
    q: &QState,
    tx: &TypedTx,
    _predicate_registry: &PredicateRegistry,
    _tool_registry: &ToolRegistry,
) -> Result<(QState, SignalBundle), TransitionError> {
    match tx {
        TypedTx::Work(work) => {
            // TB-2 Atom 3: WorkTx pure validation per preflight v3 §3.3.
            // No I/O, no side effects, no writer calls — apply_one is the
            // only place ledger writes happen.

            // Step 1: parent-root match (Inv 5; P1:5).
            if work.parent_state_root != q.state_root_t {
                return Err(TransitionError::StaleParent);
            }

            // Step 2: acceptance predicate bundle — every entry must be true.
            for (pid, bwp) in work.predicate_results.acceptance.iter() {
                if !bwp.value {
                    return Err(TransitionError::AcceptancePredicateFailed(pid.clone()));
                }
            }

            // Step 3: settlement predicate bundle (if applicable to RSP-1).
            for (pid, bwp) in work.predicate_results.settlement.iter() {
                if !bwp.value {
                    return Err(TransitionError::SettlementPredicateFailed(pid.clone()));
                }
            }

            // Step 4: YES stake gate (RSP-1 P3:3). StakeMicroCoin newtype
            // intentionally has no integer comparison; use the const accessor.
            if work.stake.micro_units() <= 0 {
                return Err(TransitionError::StakeInsufficient);
            }

            // ──────────────────────────────────────────────────────────────
            // TB-3 Atom 6 — Bridge DELETED. Structural admission via the
            // formal RSP-1 surface: task_markets_t[task_id].total_escrow > 0.
            // The TB-2 P0-B option (a) bridge `TxId(work.task_id.0.clone())`
            // synthetic-ID + escrows_t fallback is GONE — its constitutional
            // debt is now closed. Charter § 4.3 step 6 + § 5 #14 (no bridge
            // resurrection — enforced by tests/tb_3_bridge_deletion_invariant.rs
            // in Atom 7).
            // ──────────────────────────────────────────────────────────────

            // Step 5: escrow presence gate via formal surface (charter § 4.3
            // step 6 NEW form). task_markets_t is now TaskId-keyed and
            // populated only by accepted TaskOpenTx. total_escrow is the
            // derived cache that grows only via accepted EscrowLockTx.
            let market = q.economic_state_t.task_markets_t.0.get(&work.task_id);
            let has_escrow = market.map_or(false, |m| m.total_escrow.micro_units() > 0);
            if !has_escrow {
                return Err(TransitionError::EscrowMissing);
            }

            // Step 6: solver solvency gate (charter § 4.3 step 7 NEW). Per
            // WP § 14.1 + § 18 Inv 5, accepted WorkTx commits stake by
            // debiting balance — solver must hold ≥ work.stake.coin.
            let solver_bal = q.economic_state_t.balances_t.0
                .get(&work.agent_id)
                .copied()
                .unwrap_or(crate::economy::money::MicroCoin::zero());
            if solver_bal.micro_units() < work.stake.micro_units() {
                return Err(TransitionError::InsufficientBalance);
            }

            // Step 7: monetary invariants ordering (existing TB-2; same shape).
            assert_no_post_init_mint(tx, q)
                .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
            assert_read_is_free(tx.tx_kind(), 0)
                .map_err(|_| TransitionError::MonetaryInvariantViolation)?;

            // Step 8: build q_next. **TB-3 NEW (charter § 3.4 lock-on-accept)**:
            // accepted WorkTx atomically debits balance + locks stake into
            // stakes_t. Per WP § 18 Inv 5 the YES stake is event-bound to
            // the WorkTx itself; per Law 2 ("Only Investment Costs Money")
            // investment is consumed at commitment. CTF is conserved
            // (debit balance = credit stakes); no mint, no burn.
            let mut q_next = q.clone();
            let new_bal_micro = solver_bal.micro_units() - work.stake.micro_units();
            q_next.economic_state_t.balances_t.0.insert(
                work.agent_id.clone(),
                crate::economy::money::MicroCoin::from_micro_units(new_bal_micro),
            );
            q_next.economic_state_t.stakes_t.0.insert(
                work.tx_id.clone(),
                crate::state::q_state::StakeEntry {
                    // StakeMicroCoin(pub MicroCoin) — unwrap the inner
                    // MicroCoin (StakesIndex.amount: MicroCoin per q_state.rs).
                    amount: work.stake.0,
                    staker: work.agent_id.clone(),
                    task_id: work.task_id.clone(),
                },
            );
            // state_root advance (existing TB-2; WORKTX_ACCEPT_DOMAIN_V1).
            q_next.state_root_t = worktx_accept_state_root(&q.state_root_t, tx);

            // Step 9: conservation now does REAL work — not a no-op as in
            // TB-2. The debit-to-stakes invariant is the primary CTF check
            // on the runtime spine. Production runtime ALWAYS passes `&[]`
            // (charter § 5 red line 3 / TB-2 #4 inherited).
            assert_total_ctf_conserved(
                &q.economic_state_t,
                &q_next.economic_state_t,
                &[],
            )
            .map_err(|_| TransitionError::MonetaryInvariantViolation)?;

            Ok((q_next, SignalBundle::default()))
        }
        // ──────────────────────────────────────────────────────────────────
        // TB-4 Atom 4 — Verify arm (charter § 3.4 + § 4.3 + § 3.10).
        // Verifier locks bond into stakes_t[verify.tx_id]. No verdict
        // mutation in Q_t (verdict rides L4 only — § 3.10 signal-not-judge).
        // ──────────────────────────────────────────────────────────────────
        TypedTx::Verify(verify) => {
            // Step 1: parent-root match.
            if verify.parent_state_root != q.state_root_t {
                return Err(TransitionError::StaleParent);
            }
            // Step 2: bond positivity (§ 3.4 step 2).
            if verify.bond.micro_units() == 0 {
                return Err(TransitionError::BondInsufficient);
            }
            // Step 3: target liveness — must be in stakes_t (live YES stake).
            // TB-4 minimum scope: stakes_t.contains_key is a sufficient
            // proxy for "ever accepted as live WorkTx" (charter § 4.3 step 3
            // resolution; preflight § 8 Q1).
            let target_stake = match q.economic_state_t.stakes_t.0.get(&verify.target_work_tx) {
                Some(s) => s.clone(),
                None => return Err(TransitionError::TargetWorkInactive),
            };
            // Step 4: verifier solvency (§ 3.4 step 5).
            let verifier_bal = q.economic_state_t.balances_t.0
                .get(&verify.verifier_agent)
                .copied()
                .unwrap_or(crate::economy::money::MicroCoin::zero());
            if verifier_bal.micro_units() < verify.bond.micro_units() {
                return Err(TransitionError::InsufficientBalance);
            }
            // Step 5: q_next — atomic balance → stakes_t transfer.
            let mut q_next = q.clone();
            let new_bal_micro = verifier_bal.micro_units() - verify.bond.micro_units();
            q_next.economic_state_t.balances_t.0.insert(
                verify.verifier_agent.clone(),
                crate::economy::money::MicroCoin::from_micro_units(new_bal_micro),
            );
            q_next.economic_state_t.stakes_t.0.insert(
                verify.tx_id.clone(),
                crate::state::q_state::StakeEntry {
                    amount: verify.bond.0,
                    staker: verify.verifier_agent.clone(),
                    task_id: target_stake.task_id.clone(),
                },
            );
            // Step 6: monetary invariants (debit = credit).
            assert_no_post_init_mint(tx, q)
                .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
            assert_total_ctf_conserved(
                &q.economic_state_t,
                &q_next.economic_state_t,
                &[],
            )
            .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
            // Step 7: state_root advance via VERIFY_ACCEPT_DOMAIN_V1.
            q_next.state_root_t = verify_accept_state_root(&q.state_root_t, tx);

            Ok((q_next, SignalBundle::default()))
        }
        // ──────────────────────────────────────────────────────────────────
        // TB-4 Atom 5 — Challenge arm (charter § 3.5 + § 4.3 + § 3.9).
        // Challenger locks NO stake into challenge_cases_t[challenge.tx_id].
        // opened_at_round = q.logical_t is the structural anchor (§ 3.9);
        // closure / slash / resolve are RSP-3 (§ 3.7 + § 5 #11-12).
        // ──────────────────────────────────────────────────────────────────
        TypedTx::Challenge(challenge) => {
            // Step 1: parent-root match.
            if challenge.parent_state_root != q.state_root_t {
                return Err(TransitionError::StaleParent);
            }
            // Step 2: stake positivity.
            if challenge.stake.micro_units() == 0 {
                return Err(TransitionError::StakeInsufficient);
            }
            // Step 3: target liveness — same gate as Verify arm.
            if !q.economic_state_t.stakes_t.0.contains_key(&challenge.target_work_tx) {
                return Err(TransitionError::TargetWorkInactive);
            }
            // Step 4: challenger solvency.
            let challenger_bal = q.economic_state_t.balances_t.0
                .get(&challenge.challenger_agent)
                .copied()
                .unwrap_or(crate::economy::money::MicroCoin::zero());
            if challenger_bal.micro_units() < challenge.stake.micro_units() {
                return Err(TransitionError::InsufficientBalance);
            }
            // Step 5: counterexample non-empty (charter § 3.5 step 6 +
            // directive Q7).
            if challenge.counterexample_cid == Cid([0u8; 32]) {
                return Err(TransitionError::EmptyCounterexample);
            }
            // Step 6: q_next — atomic balance → challenge_cases_t transfer.
            // opened_at_round = q.logical_t (challenge-window structural
            // anchor per § 3.9; closure / deadline / auto-finalize NOT
            // installed in TB-4).
            let mut q_next = q.clone();
            let new_bal_micro = challenger_bal.micro_units() - challenge.stake.micro_units();
            q_next.economic_state_t.balances_t.0.insert(
                challenge.challenger_agent.clone(),
                crate::economy::money::MicroCoin::from_micro_units(new_bal_micro),
            );
            q_next.economic_state_t.challenge_cases_t.0.insert(
                challenge.tx_id.clone(),
                crate::state::q_state::ChallengeCase {
                    challenger: challenge.challenger_agent.clone(),
                    bond: challenge.stake.0,
                    opened_at_round: q.q_t.current_round, // ← § 3.9 anchor
                    target_work_tx: challenge.target_work_tx.clone(),
                },
            );
            // Step 7: monetary invariants (debit = credit; challenge_cases.bond
            // is the 5th holding term).
            assert_no_post_init_mint(tx, q)
                .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
            assert_total_ctf_conserved(
                &q.economic_state_t,
                &q_next.economic_state_t,
                &[],
            )
            .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
            // Step 8: state_root advance via CHALLENGE_ACCEPT_DOMAIN_V1.
            q_next.state_root_t = challenge_accept_state_root(&q.state_root_t, tx);

            Ok((q_next, SignalBundle::default()))
        }
        TypedTx::Reuse(_) => Err(TransitionError::NotYetImplemented),
        TypedTx::FinalizeReward(_) => Err(TransitionError::NotYetImplemented),
        TypedTx::TaskExpire(_) => Err(TransitionError::NotYetImplemented),
        TypedTx::TerminalSummary(_) => Err(TransitionError::NotYetImplemented),
        // ──────────────────────────────────────────────────────────────────
        // TB-3 Atom 4 — TaskOpen arm (charter § 4.3 + § 3.3 metadata-only).
        // Sponsor opens a task market entry; NO money movement; idempotent.
        // ──────────────────────────────────────────────────────────────────
        TypedTx::TaskOpen(open) => {
            // Step 1: parent-root match.
            if open.parent_state_root != q.state_root_t {
                return Err(TransitionError::StaleParent);
            }
            // Step 2: idempotency — reject second-open.
            if q.economic_state_t.task_markets_t.0.contains_key(&open.task_id) {
                return Err(TransitionError::TaskAlreadyOpen);
            }
            // Step 3: q_next — insert TaskMarketEntry; total_escrow=0.
            let mut q_next = q.clone();
            let entry = TaskMarketEntry {
                publisher: open.sponsor_agent.clone(),
                total_escrow: crate::economy::money::MicroCoin::zero(),
                escrow_lock_tx_ids: BTreeSet::new(),
                verifier_quorum: open.verifier_quorum,
                max_reuse_royalty_fraction_basis_points: open.max_reuse_royalty_fraction_basis_points,
                settlement_rule_hash: open.settlement_rule_hash,
            };
            q_next.economic_state_t.task_markets_t.0.insert(open.task_id.clone(), entry);

            // Step 4: monetary invariants. No money moved → trivially conserved.
            assert_no_post_init_mint(tx, q)
                .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
            assert_total_ctf_conserved(
                &q.economic_state_t,
                &q_next.economic_state_t,
                &[],
            )
            .map_err(|_| TransitionError::MonetaryInvariantViolation)?;

            // Step 5: state_root advance via TASK_OPEN_DOMAIN_V1.
            q_next.state_root_t = task_open_accept_state_root(&q.state_root_t, tx);

            Ok((q_next, SignalBundle::default()))
        }
        // ──────────────────────────────────────────────────────────────────
        // TB-3 Atom 5 — EscrowLock arm (charter § 4.3 + § 3.3 sole RSP-1
        // bounty funding path). Atomically debits balances, credits escrows,
        // updates the total_escrow cache. CTF-conserved (debit = credit).
        // ──────────────────────────────────────────────────────────────────
        TypedTx::EscrowLock(lock) => {
            // Step 1: parent-root match.
            if lock.parent_state_root != q.state_root_t {
                return Err(TransitionError::StaleParent);
            }
            // Step 2: target task must exist (no ghost liquidity — charter § 5 #12).
            if !q.economic_state_t.task_markets_t.0.contains_key(&lock.task_id) {
                return Err(TransitionError::TaskNotOpen);
            }
            // Step 3: sponsor solvency.
            let sponsor_bal = q.economic_state_t.balances_t.0
                .get(&lock.sponsor_agent)
                .copied()
                .unwrap_or(crate::economy::money::MicroCoin::zero());
            if sponsor_bal.micro_units() < lock.amount.micro_units() {
                return Err(TransitionError::InsufficientBalance);
            }
            // Step 4: q_next — atomic balance → escrow transfer + cache update.
            let mut q_next = q.clone();
            let new_bal_micro = sponsor_bal.micro_units() - lock.amount.micro_units();
            q_next.economic_state_t.balances_t.0.insert(
                lock.sponsor_agent.clone(),
                crate::economy::money::MicroCoin::from_micro_units(new_bal_micro),
            );
            q_next.economic_state_t.escrows_t.0.insert(
                lock.tx_id.clone(),
                EscrowEntry {
                    amount: lock.amount,
                    depositor: lock.sponsor_agent.clone(),
                    task_id: lock.task_id.clone(),
                },
            );
            // Cache update — total_escrow + escrow_lock_tx_ids.
            {
                let entry = q_next.economic_state_t.task_markets_t.0
                    .get_mut(&lock.task_id)
                    .expect("task verified to exist at step 2");
                let new_total = entry.total_escrow.micro_units() + lock.amount.micro_units();
                entry.total_escrow = crate::economy::money::MicroCoin::from_micro_units(new_total);
                entry.escrow_lock_tx_ids.insert(lock.tx_id.clone());
            }

            // Step 5: monetary invariants (debit = credit).
            assert_no_post_init_mint(tx, q)
                .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
            assert_total_ctf_conserved(
                &q.economic_state_t,
                &q_next.economic_state_t,
                &[],
            )
            .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
            // TB-3 charter § 3.2 cache=truth invariant.
            assert_task_market_total_escrow_matches_locks(
                &q_next.economic_state_t,
                &lock.task_id,
            )
            .map_err(|_| TransitionError::MonetaryInvariantViolation)?;

            // Step 6: state_root advance via ESCROW_LOCK_DOMAIN_V1.
            q_next.state_root_t = escrow_lock_accept_state_root(&q.state_root_t, tx);

            Ok((q_next, SignalBundle::default()))
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────
// CO1.7-extra D2: advance_head_t — post-commit head_t close (Art 0.4)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX § 5 — L4 sequencer post-commit head_t wiring (Art 0.4).
///
/// Closes the G-1 carry-forward: when `writer` surfaces a commit OID hex
/// (Git2LedgerWriter), advance `q.head_t = state::q_state::NodeId(hex)`;
/// when `writer` returns None (InMemoryLedgerWriter), leave `q.head_t`
/// unchanged (no-op preservation).
///
/// Called from `apply_one` stage 9 AFTER `writer.commit` succeeds. Pure
/// function (writer is `&dyn` so behavior depends only on writer's
/// `head_commit_oid_hex` return + q's prior state).
///
/// **Visibility** (CO1.7-extra round-3 B2): `pub` (NOT `pub(crate)`) so that
/// flat integration tests under `tests/co1_7_extra_*.rs` per round-2 MF5 can
/// call this helper directly.
///
/// **Atomicity** (CO1.7-extra round-2 MF9): in apply_one, called under the
/// `q_w` write lock immediately after `writer.commit` returns Ok. For Git2
/// (Some path), this is post-commit non-failing best-effort head binding —
/// `q.head_t`, `q.ledger_root_t`, and `next_logical_t` advance atomically.
/// For InMemory (None path), this is explicit no-op preservation —
/// `q.head_t` stays at the value `*q_w = q_next` left it (which equals the
/// prior value because pure transition bodies never mutate head_t per
/// CO1.7 K3 v1.2).
pub fn advance_head_t(q: &mut QState, writer: &dyn LedgerWriter) {
    if let Some(commit_oid_hex) = writer.head_commit_oid_hex() {
        q.head_t = crate::state::q_state::NodeId(commit_oid_hex);
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Submission types — K1 dual counter
// ────────────────────────────────────────────────────────────────────────────

/// Returned by `Sequencer::submit`. Carries `submit_id` (always assigned at
/// submit time) but **NOT** `logical_t` — logical_t is only assigned post-accept
/// per K1 (see spec § 3 + CO1.7 K1 closure).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SubmissionReceipt {
    pub submit_id: u64,
}

/// TRACE_MATRIX FC3-S3: L4 sequencer queue payload carrying both `submit_id`
/// and the typed tx through to `apply_one`.
///
/// Required by P1:6 / TB-2 charter §1: the L4.E rejection-evidence ledger
/// keys rejected submissions by `submit_id` (NOT `logical_t`), so the
/// sequencer driver loop must observe the same `submit_id` the caller received.
///
/// Pre-TB-2 the queue carried `TypedTx` only, stranding `submit_id` at
/// `submit()`. TB-2 preflight v3 §3.1 (P1-C r1: named struct over tuple).
#[derive(Debug)]
pub struct SubmissionEnvelope {
    pub submit_id: u64,
    pub tx: TypedTx,
}

#[derive(Debug)]
pub enum SubmitError {
    /// Bounded queue saturated (Q1/Q2 resolution: agent retries with backoff).
    QueueFull,
    /// Receiver dropped — sequencer no longer running.
    QueueClosed,
}

impl std::fmt::Display for SubmitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::QueueFull => write!(f, "submission queue saturated"),
            Self::QueueClosed => write!(f, "submission queue closed"),
        }
    }
}
impl std::error::Error for SubmitError {}

/// Errors that can occur during `apply_one`. Spec § 3 implicitly assumes
/// `Result<_, TransitionError>` but the actual `?`-propagated error chain
/// crosses CAS, keypair, and ledger-writer boundaries — wrapper enum captures
/// all of these explicitly. **Implementation note vs. spec**: spec § 3 line
/// 307 writes the apply_one signature as `Result<LedgerEntry, TransitionError>`;
/// this implementation widens to `Result<LedgerEntry, ApplyError>` to preserve
/// distinct error provenance (TransitionError keeps its closed taxonomy +
/// additive-only invariant per CO1.1.4-pre1 § 7.2).
#[derive(Debug)]
pub enum ApplyError {
    /// Pure transition function rejected the tx.
    Transition(TransitionError),
    /// CAS payload put failed.
    Cas(CasError),
    /// System keypair sign failed.
    Keypair(KeypairError),
    /// Ledger writer commit failed.
    LedgerCommit(LedgerWriterError),
    /// Internal: canonical encoding of typed-tx payload failed (should never
    /// happen for serde-derive types; surfaced for completeness).
    PayloadEncode(String),
    /// `q.read()` / `q.write()` lock poisoned by panicking thread.
    QStateLockPoisoned,
}

impl std::fmt::Display for ApplyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Transition(e) => write!(f, "transition rejected: {e}"),
            Self::Cas(e) => write!(f, "cas put failed: {e}"),
            Self::Keypair(e) => write!(f, "keypair sign failed: {e:?}"),
            Self::LedgerCommit(e) => write!(f, "ledger commit failed: {e}"),
            Self::PayloadEncode(s) => write!(f, "payload encode failed: {s}"),
            Self::QStateLockPoisoned => write!(f, "q-state lock poisoned"),
        }
    }
}
impl std::error::Error for ApplyError {}

impl From<TransitionError> for ApplyError {
    fn from(e: TransitionError) -> Self {
        Self::Transition(e)
    }
}
impl From<CasError> for ApplyError {
    fn from(e: CasError) -> Self {
        Self::Cas(e)
    }
}
impl From<KeypairError> for ApplyError {
    fn from(e: KeypairError) -> Self {
        Self::Keypair(e)
    }
}
impl From<LedgerWriterError> for ApplyError {
    fn from(e: LedgerWriterError) -> Self {
        Self::LedgerCommit(e)
    }
}

#[derive(Debug)]
pub enum SequencerError {
    /// `run()` was called when the receiver had already been consumed.
    ReceiverAlreadyTaken,
}

impl std::fmt::Display for SequencerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ReceiverAlreadyTaken => write!(f, "sequencer receiver already taken"),
        }
    }
}
impl std::error::Error for SequencerError {}

// ────────────────────────────────────────────────────────────────────────────
// Sequencer — single-writer per (runtime_repo, run_id)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX § 5.2.1 — L4 sequencer; single-writer per (runtime_repo, run_id).
///
/// **K1 dual counter**: `next_submit_id` advances at every `submit()` (used to
/// derive `SubmissionReceipt.submit_id`); `next_logical_t` advances ONLY at
/// commit time (rejected submissions never get a logical_t — preserves
/// `LedgerWriter`'s strict logical_t monotonicity invariant).
///
/// **K3 v1.2 + CO1.7-extra D2 (revised)**: the pure transition function does
/// NOT mutate `q.head_t` or `q.state_root_t`; it returns the new `QState`
/// and the sequencer accepts it as-is. `head_t` mutation now happens
/// post-commit via `advance_head_t()` (CO1.7-extra D2): when
/// `LedgerWriter::head_commit_oid_hex()` returns Some (Git2LedgerWriter),
/// the sequencer writes `q.head_t = NodeId(commit_oid_hex)`; when None
/// (InMemoryLedgerWriter), `head_t` is left unchanged (no-op preservation).
///
/// **C3 sign API**: signs through
/// `transition_ledger_emitter::sign_ledger_entry(keypair, digest_bytes)` —
/// the typed `CanonicalMessage::LedgerEntrySigning([u8;32])` extension closes
/// the C3 round-2 audit point.
/// **CO1.7-extra D3 (round-2 MF6)**: manual `Debug` impl below — `#[derive(Debug)]`
/// fails because `Arc<Ed25519Keypair>` field has no Debug derive (intentional;
/// `Ed25519Keypair` derives only `Zeroize, ZeroizeOnDrop` for secret-handling).
/// `finish_non_exhaustive()` leaks no keypair / QState / CAS contents and
/// satisfies Debug propagation through `Arc<Sequencer>` for `TuringBus.Debug`.
pub struct Sequencer {
    /// K1: assigned at submit; never appears in LedgerEntry.
    next_submit_id: AtomicU64,
    /// K1: advances ONLY on commit; first accepted entry gets logical_t=1.
    next_logical_t: AtomicU64,

    queue_tx: tokio::sync::mpsc::Sender<SubmissionEnvelope>,

    cas: Arc<RwLock<CasStore>>,
    keypair: Arc<Ed25519Keypair>,
    epoch: SystemEpoch,
    ledger_writer: Arc<RwLock<dyn LedgerWriter>>,
    /// TB-2 Atom 4: L4.E rejection-evidence writer. Mirrors `ledger_writer`'s
    /// `Arc<RwLock<...>>` shape (P0-1 r2: `append_rejected` is `&mut self`).
    /// Constructor-injected so integration tests can retain a clone of the
    /// `Arc` for L4.E observation (P0-5 r2).
    rejection_writer: Arc<RwLock<RejectionEvidenceWriter>>,

    predicate_registry: Arc<PredicateRegistry>,
    tool_registry: Arc<ToolRegistry>,

    q: RwLock<QState>,
}

/// CO1.7-extra D3 (round-2 MF6): manual Debug impl. Uses `finish_non_exhaustive()`
/// to satisfy the Debug trait without exposing keypair / QState / CAS internals.
impl std::fmt::Debug for Sequencer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Sequencer").finish_non_exhaustive()
    }
}

impl Sequencer {
    /// Construct. Returns the `Sequencer` plus the receiver half of the
    /// internal mpsc; pass the receiver to `run()` exactly once.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        cas: Arc<RwLock<CasStore>>,
        keypair: Arc<Ed25519Keypair>,
        epoch: SystemEpoch,
        ledger_writer: Arc<RwLock<dyn LedgerWriter>>,
        rejection_writer: Arc<RwLock<RejectionEvidenceWriter>>,
        predicate_registry: Arc<PredicateRegistry>,
        tool_registry: Arc<ToolRegistry>,
        initial_q: QState,
        queue_capacity: usize,
    ) -> (Self, tokio::sync::mpsc::Receiver<SubmissionEnvelope>) {
        let (queue_tx, queue_rx) = tokio::sync::mpsc::channel(queue_capacity);
        let seq = Self {
            next_submit_id: AtomicU64::new(1),
            next_logical_t: AtomicU64::new(0), // first accepted commit advances to 1
            queue_tx,
            cas,
            keypair,
            epoch,
            ledger_writer,
            rejection_writer,
            predicate_registry,
            tool_registry,
            q: RwLock::new(initial_q),
        };
        (seq, queue_rx)
    }

    /// Submit a typed transition. Returns immediately with a receipt carrying
    /// `submit_id` (NOT `logical_t`). Per Q2 (back-pressure resolution): on
    /// queue saturation returns `Err(SubmitError::QueueFull)` and the agent is
    /// expected to retry with deterministic exponential backoff.
    pub async fn submit(&self, tx: TypedTx) -> Result<SubmissionReceipt, SubmitError> {
        // TB-2 P1-D r1 concurrency contract: fetch_add precedes try_send, so
        // submit_id allocation order is NOT receiver arrival order under
        // multi-producer scheduling. submit_id is always burned (never reused)
        // even when try_send fails — locked by integration test I2.
        let submit_id = self.next_submit_id.fetch_add(1, Ordering::SeqCst);
        let envelope = SubmissionEnvelope { submit_id, tx };
        match self.queue_tx.try_send(envelope) {
            Ok(()) => Ok(SubmissionReceipt { submit_id }),
            Err(tokio::sync::mpsc::error::TrySendError::Full(_)) => Err(SubmitError::QueueFull),
            Err(tokio::sync::mpsc::error::TrySendError::Closed(_)) => Err(SubmitError::QueueClosed),
        }
    }

    /// Driver loop. Drains the queue and runs `apply_one` on each tx. Errors
    /// from individual `apply_one` calls are logged and skipped (per-tx
    /// rejection does NOT halt the sequencer). Returns when the queue is
    /// closed and drained.
    pub async fn run(
        &self,
        mut queue_rx: tokio::sync::mpsc::Receiver<SubmissionEnvelope>,
    ) -> Result<(), SequencerError> {
        while let Some(envelope) = queue_rx.recv().await {
            // Stub state: dispatch returns NotYetImplemented; apply_one
            // bubbles up. We log and continue per spec § 3 v1.2 ordering rule
            // (rejection does not consume a logical_t — see K1).
            if let Err(e) = self.apply_one(envelope) {
                log::debug!("sequencer apply_one rejected: {e}");
            }
        }
        Ok(())
    }

    /// TRACE_MATRIX FC3-S3: single-step driver companion to `run()` for tests.
    ///
    /// Drains at most one envelope from the queue and runs `apply_one` on it.
    /// Returns `None` if the queue is empty. Production code uses `run()`
    /// instead. Required by integration tests in `tests/tb_2_runtime_boundary.rs`
    /// (TB-2 Atom 4+) because `run()` loops until the receiver closes — there
    /// is no other single-poll API. TB-2 preflight v3 §3.1 (P1-3 r2).
    pub fn try_apply_one(
        &self,
        queue_rx: &mut tokio::sync::mpsc::Receiver<SubmissionEnvelope>,
    ) -> Option<Result<LedgerEntry, ApplyError>> {
        match queue_rx.try_recv() {
            Ok(envelope) => Some(self.apply_one(envelope)),
            Err(_) => None,
        }
    }

    /// TRACE_MATRIX FC3-S3: L4 sequencer per-tx critical section.
    ///
    /// Pure transition + CAS put + sign + commit + Q_t mutation. See spec § 3
    /// stages 1-9. TB-2 Atom 2 changes the input type from `TypedTx` to
    /// `SubmissionEnvelope` so `submit_id` travels in (charter §1 / P1:6);
    /// the apply pipeline itself is unchanged in Atom 2.
    ///
    /// **v1.1 C-2 closure (Codex bundle Q-B)**: `next_logical_t` advances
    /// **only on commit success** — the original spec § 3 stage-4
    /// `fetch_add(1)` happened BEFORE sign + writer.commit, so any infra
    /// failure (sign / commit) left `next_logical_t` advanced past a
    /// logical_t that was never written to the ledger. The next accepted
    /// tx would then be assigned a logical_t the writer rejects forever
    /// (writer enforces strict `len + 1`). Fixed by `load → use → store
    /// after commit succeeds`. Single-writer per spec § 5.2.1 makes the
    /// load+store atomic enough; if multi-writer ever lands the AtomicU64
    /// can be upgraded to a `compare_exchange` reservation pattern.
    pub(crate) fn apply_one(
        &self,
        envelope: SubmissionEnvelope,
    ) -> Result<LedgerEntry, ApplyError> {
        // TB-2 Atom 2: queue payload is SubmissionEnvelope so submit_id
        // travels with the tx through to apply_one. Atom 4: submit_id is
        // now actually used for the L4.E rejection-evidence path below.
        let SubmissionEnvelope { submit_id, tx } = envelope;

        // Stage 1: snapshot Q_t under read lock.
        let q_snapshot = {
            let g = self.q.read().map_err(|_| ApplyError::QStateLockPoisoned)?;
            g.clone()
        };

        // Stage 2: dispatch (pure). On reject, route to L4.E rejection-evidence
        // ledger and return early. K1: no logical_t consumed; Inv 7: no
        // state_root_t / ledger_root_t advance.
        let (q_next, _signals) = match dispatch_transition(
            &q_snapshot,
            &tx,
            &self.predicate_registry,
            &self.tool_registry,
        ) {
            Ok(ok) => ok,
            Err(transition_err) => {
                // TB-2 Atom 4 — rejection-writer path (preflight v3 §3.5).
                // CAS-put canonical-encoded tx payload + diagnostic, then
                // append_rejected to L4.E with submit_id keyed off the envelope.
                let payload_bytes = canonical_encode(&tx)
                    .map_err(|e| ApplyError::PayloadEncode(e.to_string()))?;
                let creator = format!("sequencer.rejection_path.epoch-{}", self.epoch.get());
                let rejection_logical_t = self.next_logical_t.load(Ordering::SeqCst);

                let tx_payload_cid = {
                    let mut cas_w = self
                        .cas
                        .write()
                        .map_err(|_| ApplyError::QStateLockPoisoned)?;
                    cas_w.put(
                        &payload_bytes,
                        ObjectType::ProposalPayload,
                        &creator,
                        rejection_logical_t,
                        Some("TypedTx.v1".to_string()),
                    )?
                };

                // raw_diagnostic_cid is structurally serde-shielded on
                // RejectedSubmissionRecord per TB-1 P0-3
                // (rejection_evidence.rs:108). I8 re-confirms at runtime.
                let diag_bytes = transition_err.to_string().into_bytes();
                let raw_diagnostic_cid = {
                    let mut cas_w = self
                        .cas
                        .write()
                        .map_err(|_| ApplyError::QStateLockPoisoned)?;
                    Some(cas_w.put(
                        &diag_bytes,
                        ObjectType::Generic,
                        &creator,
                        rejection_logical_t,
                        Some("TransitionError.display.v1".to_string()),
                    )?)
                };

                // P0-2 r2: HasSubmitter::submitter_id() returns Option<AgentId>.
                // RejectedSubmissionRecord.agent_id is AgentId (not Option).
                // Fall back to SYSTEM_AGENT_ID_STR for variants that return None.
                // WorkTx always returns Some so the unwrap_or_else arm is
                // theoretical for TB-2 but covers future system-emitted variants.
                let agent_id = tx
                    .submitter_id()
                    .unwrap_or_else(|| AgentId(SYSTEM_AGENT_ID_STR.to_string()));

                {
                    let mut writer_w = self
                        .rejection_writer
                        .write()
                        .map_err(|_| ApplyError::QStateLockPoisoned)?;
                    writer_w.append_rejected(
                        submit_id,
                        q_snapshot.state_root_t,
                        agent_id,
                        tx.tx_kind(),
                        tx_payload_cid,
                        rejection_class_for(&transition_err),
                        raw_diagnostic_cid,
                        public_summary_for(&transition_err),
                    );
                }

                // No logical_t advance, no state_root advance, no ledger_root
                // advance. Caller observes ApplyError::Transition.
                return Err(ApplyError::Transition(transition_err));
            }
        };

        // v1.1 C-2: TENTATIVE logical_t (do NOT fetch_add yet).
        let logical_t = self.next_logical_t.load(Ordering::SeqCst) + 1;

        // Stage 3: put payload to CAS. DIV-5 5-param put signature.
        let payload_bytes = canonical_encode(&tx)
            .map_err(|e| ApplyError::PayloadEncode(e.to_string()))?;
        let payload_cid = {
            let mut cas_w = self.cas.write().map_err(|_| ApplyError::QStateLockPoisoned)?;
            cas_w.put(
                &payload_bytes,
                ObjectType::ProposalPayload,
                &format!("sequencer-epoch-{}", self.epoch.get()),
                logical_t,
                Some("TypedTx.v1".to_string()),
            )?
        };

        // Stage 5: build LedgerEntrySigningPayload (v1.1 — stage 4 fetch_add
        // moved to AFTER stage 9 commit success).
        let signing_payload = LedgerEntrySigningPayload {
            logical_t,
            parent_state_root: q_snapshot.state_root_t,
            parent_ledger_root: q_snapshot.ledger_root_t,
            tx_kind: tx.tx_kind(),
            tx_payload_cid: payload_cid,
            resulting_state_root: q_next.state_root_t,
            timestamp_logical: logical_t,
            epoch: self.epoch,
            extensions: std::collections::BTreeMap::new(),
        };

        // Stage 6: C3 — sign via typed CanonicalMessage::LedgerEntrySigning.
        let signing_digest = signing_payload.canonical_digest();
        let system_signature = transition_ledger_emitter::sign_ledger_entry(
            &self.keypair,
            signing_digest.0,
        )?;

        // Stage 7: pure ledger-root fold (deterministic).
        let resulting_ledger_root = append(&q_snapshot.ledger_root_t, &signing_digest);

        // Stage 8: build LedgerEntry (the stored record).
        let entry = LedgerEntry {
            logical_t: signing_payload.logical_t,
            parent_state_root: signing_payload.parent_state_root,
            parent_ledger_root: signing_payload.parent_ledger_root,
            tx_kind: signing_payload.tx_kind,
            tx_payload_cid: signing_payload.tx_payload_cid,
            resulting_state_root: signing_payload.resulting_state_root,
            resulting_ledger_root,
            timestamp_logical: signing_payload.timestamp_logical,
            epoch: signing_payload.epoch,
            extensions: signing_payload.extensions,
            system_signature,
        };

        // Stage 9: commit + mutate Q_t under write lock.
        // v1.1 C-2: next_logical_t.store(logical_t) HAPPENS ONLY AFTER
        // writer.commit succeeds — preserves K1 under infra failure.
        // CO1.7-extra D2: q.head_t = NodeId(commit_oid_hex) via advance_head_t
        // when writer surfaces a commit OID (Git2 path); no-op preservation
        // for writers that return None (InMemory path). state_root_t comes
        // from q_next as-is per K3 v1.2.
        {
            let mut q_w = self.q.write().map_err(|_| ApplyError::QStateLockPoisoned)?;
            let mut writer_w = self
                .ledger_writer
                .write()
                .map_err(|_| ApplyError::QStateLockPoisoned)?;
            writer_w.commit(&entry)?; // ← may fail; if it does, fetch_add was NOT called
            // commit succeeded → safe to advance counter.
            self.next_logical_t.store(logical_t, Ordering::SeqCst);
            *q_w = q_next;
            q_w.ledger_root_t = entry.resulting_ledger_root;
            // CO1.7-extra D2: close G-1 head_t carry-forward (Art 0.4).
            advance_head_t(&mut *q_w, &*writer_w);
        }

        Ok(entry)
    }

    /// Read-only accessor (testing + CO1.7.5+ wiring).
    pub fn q_snapshot(&self) -> Result<QState, ApplyError> {
        self.q
            .read()
            .map(|g| g.clone())
            .map_err(|_| ApplyError::QStateLockPoisoned)
    }

    pub fn next_submit_id_peek(&self) -> u64 {
        self.next_submit_id.load(Ordering::SeqCst)
    }

    pub fn next_logical_t_peek(&self) -> u64 {
        self.next_logical_t.load(Ordering::SeqCst)
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Tests — stub-mode coverage (CO1.7.5 fills real-transition tests)
// ────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bottom_white::ledger::transition_ledger::InMemoryLedgerWriter;
    use crate::state::typed_tx::{
        AgentSignature, BoolWithProof, ChallengeTx, ClaimId, FinalizeRewardTx, PredicateId,
        PredicateResultsBundle, ReadKey, ReuseTx, RunId, RunOutcome, SafetyOrCreation,
        TaskExpireTx, TerminalSummaryTx, ToolId, VerifyTx, VerifyVerdict, WorkTx,
        WriteKey,
    };
    use crate::state::q_state::{AgentId, TaskId, TxId};
    use crate::economy::money::{MicroCoin, StakeMicroCoin};
    use crate::bottom_white::cas::schema::Cid;
    use crate::bottom_white::ledger::system_keypair::SystemSignature;
    use std::collections::{BTreeMap, BTreeSet};
    use tempfile::TempDir;

    fn fresh_sequencer() -> (
        TempDir,
        Sequencer,
        tokio::sync::mpsc::Receiver<SubmissionEnvelope>,
        Arc<RwLock<RejectionEvidenceWriter>>,
    ) {
        let tmp = TempDir::new().expect("tempdir");
        let cas = Arc::new(RwLock::new(CasStore::open(tmp.path()).expect("cas open")));
        let keypair = Arc::new(
            Ed25519Keypair::generate_with_secure_entropy().expect("keypair gen"),
        );
        let epoch = SystemEpoch::new(1);
        let writer: Arc<RwLock<dyn LedgerWriter>> =
            Arc::new(RwLock::new(InMemoryLedgerWriter::new()));
        let rejection_writer = Arc::new(RwLock::new(RejectionEvidenceWriter::default()));
        let preds = Arc::new(PredicateRegistry::new());
        let tools = Arc::new(ToolRegistry::new());
        let q = QState::genesis();
        let (seq, rx) = Sequencer::new(
            cas,
            keypair,
            epoch,
            writer,
            rejection_writer.clone(),
            preds,
            tools,
            q,
            16,
        );
        (tmp, seq, rx, rejection_writer)
    }

    fn fixture_work_tx() -> WorkTx {
        let mut acceptance = BTreeMap::new();
        acceptance.insert(
            PredicateId("acc1".into()),
            BoolWithProof {
                value: true,
                proof_cid: None,
            },
        );
        WorkTx {
            tx_id: TxId("worktx-seq-fixture".into()),
            task_id: TaskId("task-seq-fixture".into()),
            parent_state_root: Default::default(),
            agent_id: AgentId("alice".into()),
            read_set: [ReadKey("k.read.a".into())].into_iter().collect::<BTreeSet<_>>(),
            write_set: [WriteKey("k.write.a".into())].into_iter().collect::<BTreeSet<_>>(),
            proposal_cid: Default::default(),
            predicate_results: PredicateResultsBundle {
                acceptance,
                settlement: BTreeMap::new(),
                safety_class: SafetyOrCreation::Safety,
            },
            stake: StakeMicroCoin::from_micro_units(1_000_000),
            signature: AgentSignature::from_bytes([0x77u8; 64]),
            timestamp_logical: 1,
        }
    }

    // 1. dispatch_transition: NON-WORK / NON-RSP1 / NON-RSP2 variants
    //    return NotYetImplemented.
    //
    // TB-2 Atom 3 narrowed this from "all variants" to "non-Work variants".
    // TB-3 narrowed it further (Work + TaskOpen + EscrowLock are now real;
    // their own U/I tests cover them). TB-4 Atom 4-5 narrows further
    // (Verify + Challenge are now real; covered by U12-U21 + I31-I43).
    // Reuse / FinalizeReward / TaskExpire / TerminalSummary remain stubs
    // (RSP-3+ / RSP-4 territory).
    #[test]
    fn dispatch_transition_stubs_non_work_non_rsp1_non_rsp2_variants() {
        let q = QState::genesis();
        let preds = PredicateRegistry::new();
        let tools = ToolRegistry::new();

        let cases: Vec<TypedTx> = vec![
            TypedTx::Reuse(ReuseTx {
                tx_id: TxId("rt".into()),
                reusing_work_tx: TxId("wt".into()),
                reused_tool_id: ToolId("tool".into()),
                reused_tool_creator: AgentId("a".into()),
                timestamp_logical: 1,
            }),
            TypedTx::FinalizeReward(FinalizeRewardTx {
                tx_id: TxId("ft".into()),
                claim_id: ClaimId::new("cl"),
                task_id: TaskId("t".into()),
                solver: AgentId("s".into()),
                reward: MicroCoin::from_micro_units(1),
                parent_state_root: Default::default(),
                epoch: SystemEpoch::new(1),
                timestamp_logical: 1,
                system_signature: SystemSignature::from_bytes([0; 64]),
            }),
            TypedTx::TaskExpire(TaskExpireTx {
                tx_id: TxId("et".into()),
                task_id: TaskId("t".into()),
                parent_state_root: Default::default(),
                bounty_refunded: MicroCoin::from_micro_units(1),
                epoch: SystemEpoch::new(1),
                timestamp_logical: 1,
                system_signature: SystemSignature::from_bytes([0; 64]),
            }),
            TypedTx::TerminalSummary(TerminalSummaryTx {
                tx_id: TxId("ts".into()),
                task_id: TaskId("t".into()),
                run_id: RunId("r".into()),
                run_outcome: RunOutcome::OmegaAccepted,
                total_attempts: 0,
                failure_class_histogram: BTreeMap::new(),
                last_logical_t: 0,
                system_signature: SystemSignature::from_bytes([0; 64]),
            }),
        ];

        for tx in cases {
            let result = dispatch_transition(&q, &tx, &preds, &tools);
            assert!(matches!(result, Err(TransitionError::NotYetImplemented)));
        }
    }

    // 2. K1 dual counter: submit advances submit_id but NOT logical_t.
    #[tokio::test]
    async fn submit_advances_submit_id_only() {
        let (_tmp, seq, _rx, _rejection_writer) = fresh_sequencer();
        assert_eq!(seq.next_submit_id_peek(), 1);
        assert_eq!(seq.next_logical_t_peek(), 0);

        let r1 = seq.submit(TypedTx::Work(fixture_work_tx())).await.expect("submit 1");
        assert_eq!(r1.submit_id, 1);
        assert_eq!(seq.next_submit_id_peek(), 2);
        assert_eq!(seq.next_logical_t_peek(), 0, "logical_t MUST NOT advance at submit");

        let r2 = seq.submit(TypedTx::Work(fixture_work_tx())).await.expect("submit 2");
        assert_eq!(r2.submit_id, 2);
        assert_eq!(seq.next_logical_t_peek(), 0);
    }

    // 3. apply_one rejected: returns Transition(EscrowMissing) with the default
    //    fixture (no escrow seeded for task-seq-fixture); no logical_t consumed
    //    (K1 invariant: rejected submission never advances commit counter).
    //    TB-2 Atom 3: was NotYetImplemented pre-Atom-3; now WorkTx arm runs
    //    real validation and rejects on missing escrow.
    #[test]
    fn apply_one_stub_does_not_consume_logical_t() {
        let (_tmp, seq, _rx, _rejection_writer) = fresh_sequencer();
        let pre = seq.next_logical_t_peek();
        let envelope = SubmissionEnvelope {
            submit_id: 1,
            tx: TypedTx::Work(fixture_work_tx()),
        };
        let err = seq.apply_one(envelope).unwrap_err();
        assert!(matches!(
            err,
            ApplyError::Transition(TransitionError::EscrowMissing)
        ));
        let post = seq.next_logical_t_peek();
        assert_eq!(pre, post, "logical_t MUST NOT advance on rejected apply_one");
    }

    // TB-2 Atom 4 — U2: apply_one rejected path keys L4.E by envelope.submit_id.
    //
    // Drives apply_one with a known submit_id and a WorkTx that fails the
    // EscrowMissing gate (default fixture has no seeded escrow). Asserts the
    // resulting L4.E row has the same submit_id, mapped rejection_class, and
    // q_snapshot.state_root_t carried in. Locks P1:6 contract.
    #[test]
    fn apply_one_rejected_path_uses_envelope_submit_id() {
        let (_tmp, seq, _rx, rejection_writer) = fresh_sequencer();
        let pre = seq.q_snapshot().expect("q_snapshot").state_root_t;
        let envelope = SubmissionEnvelope {
            submit_id: 42,
            tx: TypedTx::Work(fixture_work_tx()),
        };
        let err = seq.apply_one(envelope).unwrap_err();
        assert!(matches!(
            err,
            ApplyError::Transition(TransitionError::EscrowMissing)
        ));

        let writer_g = rejection_writer.read().expect("writer read");
        let records = writer_g.records();
        assert_eq!(records.len(), 1, "exactly one L4.E row appended");
        let row = &records[0];
        assert_eq!(row.submit_id, 42, "L4.E row keyed by envelope.submit_id");
        assert_eq!(
            row.rejection_class,
            L4ERejectionClass::EscrowMissing,
            "TransitionError::EscrowMissing maps to RejectionClass::EscrowMissing"
        );
        assert_eq!(
            row.parent_state_root, pre,
            "L4.E row records pre-submit state_root_t (Inv 7)"
        );
        // L4.E never advances state; sequencer's state_root_t is unchanged.
        let post = seq.q_snapshot().expect("q_snapshot").state_root_t;
        assert_eq!(pre, post, "rejected WorkTx leaves state_root_t unchanged");
        assert_eq!(seq.next_logical_t_peek(), 0, "no logical_t consumed");
    }

    // TB-2 Atom 2 — U1: apply_one consumes SubmissionEnvelope.
    //
    // Signature-level proof that the queue payload type now carries submit_id
    // through to apply_one. Charter §8 Proof 1 will further verify that the
    // submit_id materializes in an L4.E row (Atom 4); Atom 2 only locks the
    // plumbing.
    #[test]
    fn apply_one_consumes_submission_envelope() {
        let (_tmp, seq, _rx, _rejection_writer) = fresh_sequencer();
        let envelope = SubmissionEnvelope {
            submit_id: 12345,
            tx: TypedTx::Work(fixture_work_tx()),
        };
        // Compile-time: apply_one(SubmissionEnvelope) is the canonical signature.
        // Runtime (post-Atom-3): default fixture has no seeded escrow so the
        // WorkTx arm rejects with EscrowMissing.
        let result = seq.apply_one(envelope);
        assert!(matches!(
            result,
            Err(ApplyError::Transition(TransitionError::EscrowMissing))
        ));
    }

    // TB-2 Atom 2 — try_apply_one driver helper (P1-3 r2).
    //
    // Drains at most one envelope from the queue; returns None on empty.
    // Required by integration tests in tests/tb_2_runtime_boundary.rs (Atom 4+)
    // because Sequencer::run loops until close — there is no single-poll API.
    #[tokio::test]
    async fn try_apply_one_drains_one_envelope() {
        let (_tmp, seq, mut rx, _rejection_writer) = fresh_sequencer();

        // Empty queue → None.
        assert!(seq.try_apply_one(&mut rx).is_none());

        // Submit one tx through the public path; try_apply_one should drain it.
        let receipt = seq
            .submit(TypedTx::Work(fixture_work_tx()))
            .await
            .expect("submit");
        let drained = seq.try_apply_one(&mut rx).expect("envelope was queued");
        // Default fixture lacks seeded escrow so apply_one rejects with
        // EscrowMissing. The contract proven here is "envelope was drained
        // from queue and apply_one ran".
        assert!(matches!(
            drained,
            Err(ApplyError::Transition(TransitionError::EscrowMissing))
        ));
        // Receipt's submit_id is still recoverable; concurrency contract (P1-D)
        // says it MAY have been allocated as 1, 2, etc. depending on prior
        // counter state; here pre-state is fresh so it is 1.
        assert_eq!(receipt.submit_id, 1);

        // After drain, queue is empty again.
        assert!(seq.try_apply_one(&mut rx).is_none());
    }

    // TB-2 Atom 3 — U3: dispatch_transition WorkTx returns the interim
    // domain-separated state_root_t on accept.
    //
    // Drives dispatch_transition directly (not apply_one — that's the in-crate
    // pub(crate) test surface) with a predicate-passing WorkTx + stake>0 +
    // seeded escrow. Asserts q_next.state_root_t equals exactly
    // sha256(WORKTX_ACCEPT_DOMAIN_V1 || q.state_root_t.0 || worktx_canonical_hash(tx).0).
    // Locks the interim hash so any future change is loud.
    #[test]
    fn dispatch_transition_worktx_returns_state_root_via_domain_v1() {
        let preds = PredicateRegistry::new();
        let tools = ToolRegistry::new();
        let work_tx = fixture_work_tx();
        let task_id = work_tx.task_id.clone();
        let agent_id = work_tx.agent_id.clone();

        // **TB-3 Atom 6 fixture migration**: The legacy synthetic-TxId-from-TaskId
        // escrow seed no longer satisfies the new admission gate
        // (task_markets_t[task_id].total_escrow > 0 + balances_t[agent] >= stake).
        // Build the QState by applying TaskOpen + EscrowLock through dispatch_transition,
        // and seed solver balance directly (genesis-equivalent for stake commitment).
        let mut q = QState::genesis();
        // Seed solver balance.
        q.economic_state_t.balances_t.0.insert(
            agent_id.clone(),
            MicroCoin::from_coin(10).unwrap(),
        );
        // Seed sponsor balance.
        q.economic_state_t.balances_t.0.insert(
            AgentId("treasury".into()),
            MicroCoin::from_coin(100).unwrap(),
        );
        // TaskOpen via formal surface.
        let open_tx = TypedTx::TaskOpen(crate::state::typed_tx::TaskOpenTx {
            tx_id: TxId(format!("seed-open-{}", task_id.0)),
            task_id: task_id.clone(),
            parent_state_root: q.state_root_t,
            sponsor_agent: AgentId("treasury".into()),
            verifier_quorum: 1,
            max_reuse_royalty_fraction_basis_points: 1000,
            settlement_rule_hash: Hash::ZERO,
            signature: AgentSignature::from_bytes([0u8; 64]),
            timestamp_logical: 0,
        });
        let (q_after_open, _) = dispatch_transition(&q, &open_tx, &preds, &tools)
            .expect("seed TaskOpen accepts");
        // EscrowLock via formal surface.
        let lock_tx = TypedTx::EscrowLock(crate::state::typed_tx::EscrowLockTx {
            tx_id: TxId(format!("seed-lock-{}", task_id.0)),
            task_id: task_id.clone(),
            parent_state_root: q_after_open.state_root_t,
            sponsor_agent: AgentId("treasury".into()),
            amount: MicroCoin::from_coin(50).unwrap(),
            signature: AgentSignature::from_bytes([0u8; 64]),
            timestamp_logical: 0,
        });
        let (q_funded, _) = dispatch_transition(&q_after_open, &lock_tx, &preds, &tools)
            .expect("seed EscrowLock accepts");

        // Now construct WorkTx with parent matching the funded state's state_root.
        let mut work_tx = work_tx;
        work_tx.parent_state_root = q_funded.state_root_t;
        let tx = TypedTx::Work(work_tx);
        let (q_next, _signals) = dispatch_transition(&q_funded, &tx, &preds, &tools)
            .expect("predicate-passing WorkTx with funded task + solvent solver must accept");

        // Expected state_root_t per the interim domain-separated hash.
        let expected = {
            let work_digest = worktx_canonical_hash(&tx);
            let mut h = Sha256::new();
            h.update(WORKTX_ACCEPT_DOMAIN_V1);
            h.update(q_funded.state_root_t.0);
            h.update(work_digest.0);
            let bytes: [u8; 32] = h.finalize().into();
            Hash::from_bytes(bytes)
        };

        assert_eq!(q_next.state_root_t, expected, "state_root_t must match WORKTX_ACCEPT_DOMAIN_V1 hash");
        assert_ne!(q_next.state_root_t, q_funded.state_root_t, "state_root_t must advance on accept");
        // **TB-3 Atom 6 charter § 3.4 lock-on-accept**: accepted WorkTx now
        // MUTATES economic_state_t (debits agent balance + credits stakes_t).
        // The TB-2 "unchanged" invariant is replaced by the lock-on-accept invariant.
        assert_ne!(q_next.economic_state_t, q_funded.economic_state_t,
            "TB-3: accepted WorkTx commits stake (debits balance + credits stakes_t)");
        let stake_entry = q_next.economic_state_t.stakes_t.0
            .get(&TxId("worktx-seq-fixture".into()))
            .expect("stakes_t entry by work_tx_id");
        assert_eq!(stake_entry.task_id, task_id, "stake binds to task_id (event-bound)");
    }

    // 4. Queue saturation: submit returns QueueFull (Q1/Q2 resolution).
    #[tokio::test]
    async fn submit_returns_queue_full_on_saturation() {
        // Capacity=2; receiver never drained.
        let tmp = TempDir::new().expect("tempdir");
        let cas = Arc::new(RwLock::new(CasStore::open(tmp.path()).expect("cas")));
        let keypair = Arc::new(Ed25519Keypair::generate_with_secure_entropy().expect("kp"));
        let writer: Arc<RwLock<dyn LedgerWriter>> =
            Arc::new(RwLock::new(InMemoryLedgerWriter::new()));
        let rejection_writer = Arc::new(RwLock::new(RejectionEvidenceWriter::default()));
        let preds = Arc::new(PredicateRegistry::new());
        let tools = Arc::new(ToolRegistry::new());
        let (seq, _rx) = Sequencer::new(
            cas,
            keypair,
            SystemEpoch::new(1),
            writer,
            rejection_writer,
            preds,
            tools,
            QState::genesis(),
            2,
        );
        // Fill capacity.
        seq.submit(TypedTx::Work(fixture_work_tx())).await.expect("1");
        seq.submit(TypedTx::Work(fixture_work_tx())).await.expect("2");
        // Saturated.
        let err = seq.submit(TypedTx::Work(fixture_work_tx())).await.unwrap_err();
        assert!(matches!(err, SubmitError::QueueFull));
    }

    // 5. submit returns QueueClosed when receiver dropped.
    #[tokio::test]
    async fn submit_returns_queue_closed_after_rx_drop() {
        let (_tmp, seq, rx, _rejection_writer) = fresh_sequencer();
        drop(rx);
        let err = seq.submit(TypedTx::Work(fixture_work_tx())).await.unwrap_err();
        assert!(matches!(err, SubmitError::QueueClosed));
    }

    // ──────────────────────────────────────────────────────────────────
    // TB-3 Atom 4 — TaskOpen dispatch arm tests (charter § 4.7 U4 + U5)
    // ──────────────────────────────────────────────────────────────────

    use crate::state::typed_tx::TaskOpenTx;

    fn fixture_task_open_tx_v(task: &str, sponsor: &str) -> TaskOpenTx {
        TaskOpenTx {
            tx_id: TxId(format!("taskopen-{task}")),
            task_id: TaskId(task.into()),
            parent_state_root: Hash::ZERO,
            sponsor_agent: AgentId(sponsor.into()),
            verifier_quorum: 1,
            max_reuse_royalty_fraction_basis_points: 1000,
            settlement_rule_hash: Hash::ZERO,
            signature: AgentSignature::from_bytes([0u8; 64]),
            timestamp_logical: 1,
        }
    }

    /// U4 — TaskOpen dispatch inserts TaskMarketEntry; balances unchanged;
    /// total_escrow=0; state_root advances via TASK_OPEN_DOMAIN_V1.
    #[test]
    fn dispatch_task_open_inserts_task_market_entry() {
        let preds = PredicateRegistry::new();
        let tools = ToolRegistry::new();
        let q = QState::genesis();
        let tx = TypedTx::TaskOpen(fixture_task_open_tx_v("task-u4", "sponsor-alice"));
        let (q_next, _signals) = dispatch_transition(&q, &tx, &preds, &tools)
            .expect("TaskOpen on genesis must accept");

        let entry = q_next.economic_state_t.task_markets_t.0
            .get(&TaskId("task-u4".into()))
            .expect("TaskMarketEntry inserted");
        assert_eq!(entry.publisher, AgentId("sponsor-alice".into()));
        assert_eq!(entry.total_escrow.micro_units(), 0);
        assert!(entry.escrow_lock_tx_ids.is_empty(),
            "TaskOpen does not lock any escrow yet (charter § 3.3 metadata-only)");
        assert_eq!(entry.verifier_quorum, 1);

        // No money moved — balances stay empty (genesis baseline).
        assert!(q_next.economic_state_t.balances_t.0.is_empty());
        assert!(q_next.economic_state_t.escrows_t.0.is_empty());

        // state_root advanced via TASK_OPEN_DOMAIN_V1.
        let expected = task_open_accept_state_root(&Hash::ZERO, &tx);
        assert_eq!(q_next.state_root_t, expected);
        assert_ne!(q_next.state_root_t, Hash::ZERO);
    }

    /// U5 — TaskOpen idempotency: second open for same task_id rejects with
    /// TaskAlreadyOpen.
    #[test]
    fn dispatch_task_open_rejects_when_already_open() {
        let preds = PredicateRegistry::new();
        let tools = ToolRegistry::new();
        let mut q = QState::genesis();
        // First open: q ← q_next (in test we manually compose).
        let first = TypedTx::TaskOpen(fixture_task_open_tx_v("task-u5", "sponsor"));
        let (q_after_first, _) = dispatch_transition(&q, &first, &preds, &tools).expect("first");
        q = q_after_first;

        // Second open for the SAME task_id but with refreshed parent_root.
        let mut second = fixture_task_open_tx_v("task-u5", "sponsor");
        second.tx_id = TxId("taskopen-task-u5-second".into());
        second.parent_state_root = q.state_root_t;
        let r = dispatch_transition(&q, &TypedTx::TaskOpen(second), &preds, &tools);
        assert!(
            matches!(r, Err(TransitionError::TaskAlreadyOpen)),
            "second open for same task_id must reject TaskAlreadyOpen; got {:?}", r
        );
    }

    // ──────────────────────────────────────────────────────────────────
    // TB-3 Atom 5 — EscrowLock dispatch arm tests (charter § 4.7 U6-U8)
    // ──────────────────────────────────────────────────────────────────

    use crate::state::typed_tx::EscrowLockTx;

    fn fixture_escrow_lock_tx_v(task: &str, sponsor: &str, amount_micro: i64, parent: Hash, suffix: &str) -> EscrowLockTx {
        EscrowLockTx {
            tx_id: TxId(format!("escrowlock-{task}-{suffix}")),
            task_id: TaskId(task.into()),
            parent_state_root: parent,
            sponsor_agent: AgentId(sponsor.into()),
            amount: MicroCoin::from_micro_units(amount_micro),
            signature: AgentSignature::from_bytes([0u8; 64]),
            timestamp_logical: 1,
        }
    }

    /// Helper: open task + seed sponsor balance, return q.
    fn q_with_open_task_and_balance(task: &str, sponsor: &str, balance_coin: i64) -> QState {
        let preds = PredicateRegistry::new();
        let tools = ToolRegistry::new();
        let mut q = QState::genesis();
        // Seed sponsor balance.
        q.economic_state_t.balances_t.0.insert(
            AgentId(sponsor.into()),
            MicroCoin::from_coin(balance_coin).unwrap(),
        );
        // Open task.
        let open = TypedTx::TaskOpen(fixture_task_open_tx_v(task, sponsor));
        let (q_next, _) = dispatch_transition(&q, &open, &preds, &tools)
            .expect("TaskOpen on seeded balance must accept");
        q_next
    }

    /// U6 — EscrowLock dispatch debits balance, credits escrow, updates total_escrow + escrow_lock_tx_ids.
    #[test]
    fn dispatch_escrow_lock_debits_balance_credits_escrow_updates_total() {
        let preds = PredicateRegistry::new();
        let tools = ToolRegistry::new();
        let q = q_with_open_task_and_balance("task-u6", "sponsor-u6", 100);
        let parent = q.state_root_t;
        let lock_amount_micro = 30_000_000; // 30 coin
        let lock = TypedTx::EscrowLock(fixture_escrow_lock_tx_v(
            "task-u6", "sponsor-u6", lock_amount_micro, parent, "u6",
        ));

        let (q_next, _signals) = dispatch_transition(&q, &lock, &preds, &tools)
            .expect("EscrowLock with sufficient balance must accept");

        // Balance debited.
        let new_bal = q_next.economic_state_t.balances_t.0
            .get(&AgentId("sponsor-u6".into())).expect("sponsor balance still present");
        assert_eq!(new_bal.micro_units(), 70_000_000, "30 coin debited from 100");

        // Escrow credited.
        let lock_tx_id = TxId("escrowlock-task-u6-u6".into());
        let escrow = q_next.economic_state_t.escrows_t.0.get(&lock_tx_id)
            .expect("escrow row keyed by escrow_lock_tx_id");
        assert_eq!(escrow.amount.micro_units(), lock_amount_micro);
        assert_eq!(escrow.depositor, AgentId("sponsor-u6".into()));
        assert_eq!(escrow.task_id, TaskId("task-u6".into()));

        // Cache updated: total_escrow + escrow_lock_tx_ids.
        let market = q_next.economic_state_t.task_markets_t.0
            .get(&TaskId("task-u6".into())).expect("market exists");
        assert_eq!(market.total_escrow.micro_units(), lock_amount_micro);
        assert!(market.escrow_lock_tx_ids.contains(&lock_tx_id));

        // state_root advanced via ESCROW_LOCK_DOMAIN_V1.
        let expected = escrow_lock_accept_state_root(&parent, &lock);
        assert_eq!(q_next.state_root_t, expected);
    }

    /// U7 — EscrowLock to a task that is NOT open rejects with TaskNotOpen.
    #[test]
    fn dispatch_escrow_lock_rejects_when_task_not_open() {
        let preds = PredicateRegistry::new();
        let tools = ToolRegistry::new();
        // Sponsor has balance but no TaskOpen has been submitted.
        let mut q = QState::genesis();
        q.economic_state_t.balances_t.0.insert(
            AgentId("sponsor-u7".into()),
            MicroCoin::from_coin(50).unwrap(),
        );
        let lock = TypedTx::EscrowLock(fixture_escrow_lock_tx_v(
            "task-not-opened", "sponsor-u7", 10_000_000, Hash::ZERO, "u7",
        ));
        let r = dispatch_transition(&q, &lock, &preds, &tools);
        assert!(matches!(r, Err(TransitionError::TaskNotOpen)),
            "EscrowLock to unknown task must reject TaskNotOpen; got {:?}", r);
    }

    /// U8 — EscrowLock with sponsor balance < amount rejects with InsufficientBalance.
    #[test]
    fn dispatch_escrow_lock_rejects_when_insufficient_balance() {
        let preds = PredicateRegistry::new();
        let tools = ToolRegistry::new();
        // Open task first, but sponsor has only 5 coin.
        let q = q_with_open_task_and_balance("task-u8", "sponsor-u8", 5);
        let parent = q.state_root_t;
        let lock = TypedTx::EscrowLock(fixture_escrow_lock_tx_v(
            "task-u8", "sponsor-u8", 100_000_000 /* 100 coin > 5 */, parent, "u8",
        ));
        let r = dispatch_transition(&q, &lock, &preds, &tools);
        assert!(matches!(r, Err(TransitionError::InsufficientBalance)),
            "EscrowLock amount > balance must reject InsufficientBalance; got {:?}", r);
    }

    // ──────────────────────────────────────────────────────────────────
    // TB-3 Atom 6 — WorkTx arm refactor tests (charter § 4.7 U9-U11)
    // ──────────────────────────────────────────────────────────────────

    /// Helper: open task + lock escrow + seed solver balance, return q.
    fn q_with_funded_task_and_solver_balance(
        task: &str,
        sponsor: &str,
        sponsor_balance_coin: i64,
        escrow_coin: i64,
        solver: &str,
        solver_balance_coin: i64,
    ) -> QState {
        let preds = PredicateRegistry::new();
        let tools = ToolRegistry::new();
        let mut q = q_with_open_task_and_balance(task, sponsor, sponsor_balance_coin);
        // Seed solver balance directly (genesis-equivalent; state_root != ZERO at this
        // point, but assert_no_post_init_mint is permissive at genesis since on_init_tx
        // is not yet implemented — this helper is test-only and doesn't violate Inv 4).
        // We modify q before any further dispatch_transition so the seed is "implicit".
        q.economic_state_t.balances_t.0.insert(
            AgentId(solver.into()),
            MicroCoin::from_coin(solver_balance_coin).unwrap(),
        );
        // Lock escrow.
        let parent = q.state_root_t;
        let lock = TypedTx::EscrowLock(fixture_escrow_lock_tx_v(
            task, sponsor, escrow_coin * 1_000_000, parent, "funded",
        ));
        let (q_next, _) = dispatch_transition(&q, &lock, &preds, &tools)
            .expect("EscrowLock seed must accept");
        q_next
    }

    fn fixture_worktx_v(task: &str, agent: &str, parent: Hash, stake_micro: i64, suffix: &str, predicate_passes: bool) -> WorkTx {
        let mut acceptance = BTreeMap::new();
        acceptance.insert(
            PredicateId("acc1".into()),
            BoolWithProof { value: predicate_passes, proof_cid: None },
        );
        WorkTx {
            tx_id: TxId(format!("worktx-{task}-{suffix}")),
            task_id: TaskId(task.into()),
            parent_state_root: parent,
            agent_id: AgentId(agent.into()),
            read_set: BTreeSet::new(),
            write_set: BTreeSet::new(),
            proposal_cid: Default::default(),
            predicate_results: PredicateResultsBundle {
                acceptance,
                settlement: BTreeMap::new(),
                safety_class: SafetyOrCreation::Safety,
            },
            stake: StakeMicroCoin::from_micro_units(stake_micro),
            signature: AgentSignature::from_bytes([0u8; 64]),
            timestamp_logical: 1,
        }
    }

    /// U9 — WorkTx admission via formal surface (no bridge): predicate-passing
    /// WorkTx after open + lock + balance setup is accepted; state_root advances.
    #[test]
    fn dispatch_worktx_admission_via_formal_surface_no_bridge() {
        let preds = PredicateRegistry::new();
        let tools = ToolRegistry::new();
        let q = q_with_funded_task_and_solver_balance(
            "task-u9", "sponsor-u9", 100, 30, "solver-u9", 10,
        );
        let parent = q.state_root_t;
        let work = TypedTx::Work(fixture_worktx_v(
            "task-u9", "solver-u9", parent, 1_000_000 /* 1 coin */, "u9", true,
        ));
        let result = dispatch_transition(&q, &work, &preds, &tools);
        assert!(result.is_ok(),
            "WorkTx with funded task + solvent solver must accept via formal surface; got {:?}", result);
        let (q_next, _) = result.unwrap();
        // state_root advanced via WORKTX_ACCEPT_DOMAIN_V1.
        let expected = worktx_accept_state_root(&parent, &work);
        assert_eq!(q_next.state_root_t, expected);
    }

    /// U10 — WorkTx admission rejects when solver balance < stake.
    #[test]
    fn dispatch_worktx_rejects_when_solver_balance_lt_stake() {
        let preds = PredicateRegistry::new();
        let tools = ToolRegistry::new();
        // Solver has only 0 coin (no balance entry — defaults to zero).
        let q = q_with_funded_task_and_solver_balance(
            "task-u10", "sponsor-u10", 100, 30, "solver-other", 0,
        );
        let parent = q.state_root_t;
        let work = TypedTx::Work(fixture_worktx_v(
            "task-u10", "solver-broke", parent, 5_000_000 /* 5 coin */, "u10", true,
        ));
        let result = dispatch_transition(&q, &work, &preds, &tools);
        assert!(matches!(result, Err(TransitionError::InsufficientBalance)),
            "solver lacks balance for stake → InsufficientBalance; got {:?}", result);
    }

    /// U11 — Accepted WorkTx debits balance + credits stakes_t with task_id binding.
    #[test]
    fn dispatch_worktx_accept_debits_balance_credits_stakes() {
        let preds = PredicateRegistry::new();
        let tools = ToolRegistry::new();
        let q = q_with_funded_task_and_solver_balance(
            "task-u11", "sponsor-u11", 100, 30, "solver-u11", 10,
        );
        let parent = q.state_root_t;
        let pre_solver_bal = q.economic_state_t.balances_t.0
            .get(&AgentId("solver-u11".into())).copied().unwrap();
        let work = TypedTx::Work(fixture_worktx_v(
            "task-u11", "solver-u11", parent, 3_000_000 /* 3 coin */, "u11", true,
        ));
        let (q_next, _) = dispatch_transition(&q, &work, &preds, &tools)
            .expect("accept");

        // Balance debited by stake.
        let post_solver_bal = q_next.economic_state_t.balances_t.0
            .get(&AgentId("solver-u11".into())).copied().unwrap();
        assert_eq!(
            post_solver_bal.micro_units(),
            pre_solver_bal.micro_units() - 3_000_000,
            "solver balance debited by stake amount (10 coin -> 7 coin)"
        );

        // stakes_t populated with task_id binding.
        let stake_entry = q_next.economic_state_t.stakes_t.0
            .get(&TxId("worktx-task-u11-u11".into()))
            .expect("stakes_t entry by work_tx_id");
        assert_eq!(stake_entry.amount.micro_units(), 3_000_000);
        assert_eq!(stake_entry.staker, AgentId("solver-u11".into()));
        assert_eq!(stake_entry.task_id, TaskId("task-u11".into()),
            "task_id binding (per WP § 18 Inv 5 event-bound risk right)");

        // CTF conserved: balance debit (-3 coin) + stakes credit (+3 coin) = 0 delta.
        let pre_total: i64 = q.economic_state_t.balances_t.0.values().map(|v| v.micro_units()).sum::<i64>()
            + q.economic_state_t.escrows_t.0.values().map(|e| e.amount.micro_units()).sum::<i64>()
            + q.economic_state_t.stakes_t.0.values().map(|e| e.amount.micro_units()).sum::<i64>();
        let post_total: i64 = q_next.economic_state_t.balances_t.0.values().map(|v| v.micro_units()).sum::<i64>()
            + q_next.economic_state_t.escrows_t.0.values().map(|e| e.amount.micro_units()).sum::<i64>()
            + q_next.economic_state_t.stakes_t.0.values().map(|e| e.amount.micro_units()).sum::<i64>();
        assert_eq!(pre_total, post_total, "CTF conserved across WorkTx accept");
    }

    // ── TB-4 Atom 4 — Verify dispatch arm tests (charter § 4.7 U12-U16) ──

    /// Helper: seed Q with one balance entry + one stakes_t entry (the
    /// "live target WorkTx"). For Verify/Challenge unit tests that only
    /// need target liveness, NOT the full TaskOpen+EscrowLock+WorkTx flow.
    /// Returns (q, work_tx_id, task_id) so callers can target the seeded
    /// WorkTx by tx_id.
    fn seed_q_with_live_target(verifier: &str, balance_coin: i64, target_work_tx_id: &str)
        -> (QState, TxId, TaskId)
    {
        let mut q = QState::genesis();
        q.economic_state_t.balances_t.0.insert(
            AgentId(verifier.into()),
            MicroCoin::from_coin(balance_coin).unwrap(),
        );
        let target_tx = TxId(target_work_tx_id.into());
        let task_id = TaskId(format!("task-of-{target_work_tx_id}"));
        q.economic_state_t.stakes_t.0.insert(
            target_tx.clone(),
            crate::state::q_state::StakeEntry {
                amount: MicroCoin::from_coin(5).unwrap(),
                staker: AgentId("solver-x".into()),
                task_id: task_id.clone(),
            },
        );
        (q, target_tx, task_id)
    }

    fn fixture_verify_tx_for_target(verify_tx_id: &str, target_work_tx_id: &str,
                                    verifier: &str, bond_coin: i64,
                                    parent_root: Hash) -> VerifyTx {
        VerifyTx {
            tx_id: TxId(verify_tx_id.into()),
            parent_state_root: parent_root,
            target_work_tx: TxId(target_work_tx_id.into()),
            verifier_agent: AgentId(verifier.into()),
            bond: StakeMicroCoin::from_micro_units(
                MicroCoin::from_coin(bond_coin).unwrap().micro_units()
            ),
            verdict: VerifyVerdict::Confirm,
            signature: AgentSignature::from_bytes([0u8; 64]),
            timestamp_logical: 1,
        }
    }

    /// U12 — Verify accept locks bond into stakes_t at verify.tx_id with
    /// task_id binding inherited from target's stakes_t entry.
    #[test]
    fn dispatch_verify_locks_bond_in_stakes_t_at_verify_tx_id() {
        let preds = PredicateRegistry::new();
        let tools = ToolRegistry::new();
        let (q, _target, task_id) = seed_q_with_live_target("verifier-bob", 10, "wt-u12");
        let verify_tx = fixture_verify_tx_for_target(
            "vt-u12", "wt-u12", "verifier-bob", 3, q.state_root_t
        );
        let tx = TypedTx::Verify(verify_tx);
        let (q_next, _) = dispatch_transition(&q, &tx, &preds, &tools)
            .expect("Verify with positive bond + live target + solvent verifier must accept");

        // bond locked into stakes_t at verify.tx_id
        let entry = q_next.economic_state_t.stakes_t.0
            .get(&TxId("vt-u12".into()))
            .expect("stakes_t entry at verify.tx_id");
        assert_eq!(entry.amount.micro_units(),
                   MicroCoin::from_coin(3).unwrap().micro_units());
        assert_eq!(entry.staker, AgentId("verifier-bob".into()));
        // task_id binding inherited from target's stakes_t entry (charter § 3.4).
        assert_eq!(entry.task_id, task_id, "Verify entry task_id inherits from target");

        // verifier balance debited.
        let new_bal = q_next.economic_state_t.balances_t.0
            .get(&AgentId("verifier-bob".into())).copied().unwrap();
        assert_eq!(new_bal.micro_units(),
                   MicroCoin::from_coin(7).unwrap().micro_units());

        // state_root advanced via VERIFY_ACCEPT_DOMAIN_V1.
        let expected = verify_accept_state_root(&q.state_root_t, &tx);
        assert_eq!(q_next.state_root_t, expected);
        assert_ne!(q_next.state_root_t, q.state_root_t);
    }

    /// U13 — VerifyTx with bond.micro_units() == 0 rejects with BondInsufficient.
    #[test]
    fn dispatch_verify_rejects_when_bond_zero() {
        let preds = PredicateRegistry::new();
        let tools = ToolRegistry::new();
        let (q, _target, _task) = seed_q_with_live_target("v", 10, "wt-u13");
        let mut verify_tx = fixture_verify_tx_for_target(
            "vt-u13", "wt-u13", "v", 5, q.state_root_t
        );
        verify_tx.bond = StakeMicroCoin::from_micro_units(0);
        let tx = TypedTx::Verify(verify_tx);
        let err = dispatch_transition(&q, &tx, &preds, &tools).unwrap_err();
        assert!(matches!(err, TransitionError::BondInsufficient));
    }

    /// U14 — VerifyTx with target_work_tx not in stakes_t rejects with
    /// TargetWorkInactive (charter § 3.8 + directive Q3).
    #[test]
    fn dispatch_verify_rejects_when_target_not_in_stakes_t() {
        let preds = PredicateRegistry::new();
        let tools = ToolRegistry::new();
        // Q has no stakes_t entries.
        let mut q = QState::genesis();
        q.economic_state_t.balances_t.0.insert(
            AgentId("v".into()), MicroCoin::from_coin(10).unwrap()
        );
        let verify_tx = fixture_verify_tx_for_target(
            "vt-u14", "wt-not-existent", "v", 3, q.state_root_t
        );
        let tx = TypedTx::Verify(verify_tx);
        let err = dispatch_transition(&q, &tx, &preds, &tools).unwrap_err();
        assert!(matches!(err, TransitionError::TargetWorkInactive),
                "expected TargetWorkInactive, got {err:?}");
    }

    /// U15 — VerifyTx with stale parent_state_root rejects with StaleParent.
    /// (Charter § 3.4 step 1.)
    #[test]
    fn dispatch_verify_rejects_when_parent_stale() {
        let preds = PredicateRegistry::new();
        let tools = ToolRegistry::new();
        let (q, _target, _task) = seed_q_with_live_target("v", 10, "wt-u15");
        let mut verify_tx = fixture_verify_tx_for_target(
            "vt-u15", "wt-u15", "v", 3, Hash::ZERO  // ZERO ≠ q.state_root_t
        );
        // ensure parent_state_root really differs.
        if verify_tx.parent_state_root == q.state_root_t {
            verify_tx.parent_state_root = Hash([0xFFu8; 32]);
        }
        let tx = TypedTx::Verify(verify_tx);
        let err = dispatch_transition(&q, &tx, &preds, &tools).unwrap_err();
        assert!(matches!(err, TransitionError::StaleParent));
    }

    /// U16 — VerifyTx with verifier balance < bond rejects with InsufficientBalance.
    #[test]
    fn dispatch_verify_rejects_when_verifier_balance_lt_bond() {
        let preds = PredicateRegistry::new();
        let tools = ToolRegistry::new();
        let (q, _target, _task) = seed_q_with_live_target("v", 1, "wt-u16");  // only 1 coin
        let verify_tx = fixture_verify_tx_for_target(
            "vt-u16", "wt-u16", "v", 5, q.state_root_t  // requires 5 coin
        );
        let tx = TypedTx::Verify(verify_tx);
        let err = dispatch_transition(&q, &tx, &preds, &tools).unwrap_err();
        assert!(matches!(err, TransitionError::InsufficientBalance));
    }

    // ── TB-4 Atom 5 — Challenge dispatch arm tests (charter § 4.7 U17-U21) ──

    fn fixture_challenge_tx_for_target(
        challenge_tx_id: &str, target_work_tx_id: &str,
        challenger: &str, stake_coin: i64, counterex_byte: u8,
        parent_root: Hash,
    ) -> ChallengeTx {
        ChallengeTx {
            tx_id: TxId(challenge_tx_id.into()),
            parent_state_root: parent_root,
            target_work_tx: TxId(target_work_tx_id.into()),
            challenger_agent: AgentId(challenger.into()),
            stake: StakeMicroCoin::from_micro_units(
                MicroCoin::from_coin(stake_coin).unwrap().micro_units()
            ),
            counterexample_cid: Cid([counterex_byte; 32]),
            signature: AgentSignature::from_bytes([0u8; 64]),
            timestamp_logical: 1,
        }
    }

    /// Seed Q with challenger balance + a live target stakes_t entry AND set
    /// q.q_t.current_round to a non-zero value so we can pinpoint the
    /// opened_at_round anchor (charter § 3.9).
    fn seed_q_for_challenge(
        challenger: &str, balance_coin: i64, target_work_tx_id: &str, current_round: u64,
    ) -> (QState, TxId, TaskId) {
        let mut q = QState::genesis();
        q.q_t.current_round = current_round;
        q.economic_state_t.balances_t.0.insert(
            AgentId(challenger.into()),
            MicroCoin::from_coin(balance_coin).unwrap(),
        );
        let target_tx = TxId(target_work_tx_id.into());
        let task_id = TaskId(format!("task-of-{target_work_tx_id}"));
        q.economic_state_t.stakes_t.0.insert(
            target_tx.clone(),
            crate::state::q_state::StakeEntry {
                amount: MicroCoin::from_coin(5).unwrap(),
                staker: AgentId("solver-x".into()),
                task_id: task_id.clone(),
            },
        );
        (q, target_tx, task_id)
    }

    /// U17 — Challenge accept opens a ChallengeCase with the target back-ref
    /// and `opened_at_round = q.logical_t` anchor (charter § 3.5 + § 3.9).
    #[test]
    fn dispatch_challenge_opens_case_with_target_back_ref_and_logical_t_anchor() {
        let preds = PredicateRegistry::new();
        let tools = ToolRegistry::new();
        let (q, _target, _task) =
            seed_q_for_challenge("challenger-u17", 10, "wt-u17", 42);
        let chal_tx = fixture_challenge_tx_for_target(
            "ct-u17", "wt-u17", "challenger-u17", 4, 0xAB, q.state_root_t,
        );
        let tx = TypedTx::Challenge(chal_tx);
        let (q_next, _) = dispatch_transition(&q, &tx, &preds, &tools)
            .expect("Challenge with positive stake + live target + solvent challenger + non-zero counterex must accept");

        // ChallengeCase opened at challenge.tx_id with target back-ref + logical_t anchor.
        let case = q_next.economic_state_t.challenge_cases_t.0
            .get(&TxId("ct-u17".into()))
            .expect("ChallengeCase at challenge.tx_id");
        assert_eq!(case.bond.micro_units(),
                   MicroCoin::from_coin(4).unwrap().micro_units());
        assert_eq!(case.challenger, AgentId("challenger-u17".into()));
        assert_eq!(case.target_work_tx, TxId("wt-u17".into()),
                   "TB-4 target_work_tx back-ref (charter § 3.3)");
        assert_eq!(case.opened_at_round, 42,
                   "TB-4 § 3.9 anchor: opened_at_round = q.logical_t at accept");

        // Challenger balance debited.
        let new_bal = q_next.economic_state_t.balances_t.0
            .get(&AgentId("challenger-u17".into())).copied().unwrap();
        assert_eq!(new_bal.micro_units(),
                   MicroCoin::from_coin(6).unwrap().micro_units());

        // state_root advanced via CHALLENGE_ACCEPT_DOMAIN_V1.
        let expected = challenge_accept_state_root(&q.state_root_t, &tx);
        assert_eq!(q_next.state_root_t, expected);
    }

    /// U18 — ChallengeTx with stake.micro_units() == 0 rejects with StakeInsufficient.
    #[test]
    fn dispatch_challenge_rejects_when_stake_zero() {
        let preds = PredicateRegistry::new();
        let tools = ToolRegistry::new();
        let (q, _t, _task) = seed_q_for_challenge("c", 10, "wt-u18", 0);
        let mut chal_tx = fixture_challenge_tx_for_target(
            "ct-u18", "wt-u18", "c", 5, 0x01, q.state_root_t,
        );
        chal_tx.stake = StakeMicroCoin::from_micro_units(0);
        let tx = TypedTx::Challenge(chal_tx);
        let err = dispatch_transition(&q, &tx, &preds, &tools).unwrap_err();
        assert!(matches!(err, TransitionError::StakeInsufficient));
    }

    /// U19 — ChallengeTx with target_work_tx not in stakes_t rejects with
    /// TargetWorkInactive (charter § 3.5 step 3).
    #[test]
    fn dispatch_challenge_rejects_when_target_not_in_stakes_t() {
        let preds = PredicateRegistry::new();
        let tools = ToolRegistry::new();
        let mut q = QState::genesis();
        q.economic_state_t.balances_t.0.insert(
            AgentId("c".into()), MicroCoin::from_coin(10).unwrap(),
        );
        let chal_tx = fixture_challenge_tx_for_target(
            "ct-u19", "wt-not-existent", "c", 5, 0x01, q.state_root_t,
        );
        let tx = TypedTx::Challenge(chal_tx);
        let err = dispatch_transition(&q, &tx, &preds, &tools).unwrap_err();
        assert!(matches!(err, TransitionError::TargetWorkInactive),
                "expected TargetWorkInactive, got {err:?}");
    }

    /// U20 — ChallengeTx with counterexample_cid == Cid::ZERO rejects with
    /// EmptyCounterexample (charter § 3.5 step 6 + directive Q7).
    #[test]
    fn dispatch_challenge_rejects_when_counterexample_cid_zero() {
        let preds = PredicateRegistry::new();
        let tools = ToolRegistry::new();
        let (q, _t, _task) = seed_q_for_challenge("c", 10, "wt-u20", 0);
        let chal_tx = fixture_challenge_tx_for_target(
            "ct-u20", "wt-u20", "c", 5, 0x00, q.state_root_t,  // ZERO counterex
        );
        let tx = TypedTx::Challenge(chal_tx);
        let err = dispatch_transition(&q, &tx, &preds, &tools).unwrap_err();
        assert!(matches!(err, TransitionError::EmptyCounterexample));
    }

    /// U21 — ChallengeTx with challenger balance < stake rejects with
    /// InsufficientBalance.
    #[test]
    fn dispatch_challenge_rejects_when_challenger_balance_lt_stake() {
        let preds = PredicateRegistry::new();
        let tools = ToolRegistry::new();
        let (q, _t, _task) = seed_q_for_challenge("c", 1, "wt-u21", 0);  // only 1 coin
        let chal_tx = fixture_challenge_tx_for_target(
            "ct-u21", "wt-u21", "c", 5, 0xCC, q.state_root_t,  // requires 5 coin
        );
        let tx = TypedTx::Challenge(chal_tx);
        let err = dispatch_transition(&q, &tx, &preds, &tools).unwrap_err();
        assert!(matches!(err, TransitionError::InsufficientBalance));
    }
}
