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
    assert_claim_amount_backed_by_escrow, assert_no_post_init_mint, assert_read_is_free,
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

/// TRACE_MATRIX TB-5 charter v2 § 4.6 + preflight § 7.1 —
/// ChallengeResolve-accept state-root domain.
pub(crate) const CHALLENGE_RESOLVE_DOMAIN_V1: &[u8] =
    b"turingosv4.challenge_resolve.accept.v1";

/// TRACE_MATRIX TB-5 charter v2 § 4.6 + preflight § 7.1 — interim state-root
/// mutator on `ChallengeResolveTx` accept (Released or UpheldDeferred).
/// Mirror of `challenge_accept_state_root`. Public single-item surface for
/// integration tests to recompute the expected post-accept hash.
pub fn challenge_resolve_accept_state_root(prev: &Hash, tx: &TypedTx) -> Hash {
    let mut h = Sha256::new();
    h.update(CHALLENGE_RESOLVE_DOMAIN_V1);
    h.update(prev.0);
    h.update(canonical_encode(tx).expect("TypedTx is canonical-encodable"));
    let digest: [u8; 32] = h.finalize().into();
    Hash::from_bytes(digest)
}

/// TRACE_MATRIX TB-8 charter §3 Atom 3 — FinalizeReward-accept state-root
/// domain. Mirror of `challenge_resolve_accept_state_root`.
pub(crate) const FINALIZE_REWARD_DOMAIN_V1: &[u8] =
    b"turingosv4.finalize_reward.accept.v1";

/// TRACE_MATRIX TB-8 charter §3 Atom 3 — interim state-root mutator on
/// `FinalizeRewardTx` accept (single-solver MVP; debit escrow + credit
/// balance + flip claim status to Finalized). Public single-item surface
/// for integration tests.
pub fn finalize_reward_accept_state_root(prev: &Hash, tx: &TypedTx) -> Hash {
    let mut h = Sha256::new();
    h.update(FINALIZE_REWARD_DOMAIN_V1);
    h.update(prev.0);
    h.update(canonical_encode(tx).expect("TypedTx is canonical-encodable"));
    let digest: [u8; 32] = h.finalize().into();
    Hash::from_bytes(digest)
}

/// TRACE_MATRIX TB-11 Atom 2 (architect §6.2 ruling 2026-05-02): TaskExpire
/// state-root domain. Mirror of `finalize_reward_accept_state_root`.
pub(crate) const TASK_EXPIRE_DOMAIN_V1: &[u8] = b"turingosv4.task_expire.accept.v1";

/// TRACE_MATRIX TB-11 Atom 2: state-root mutator on `TaskExpireTx` accept
/// (refund escrow → balances_t[sponsor]; CTF preserved bit-for-bit).
pub fn task_expire_accept_state_root(prev: &Hash, tx: &TypedTx) -> Hash {
    let mut h = Sha256::new();
    h.update(TASK_EXPIRE_DOMAIN_V1);
    h.update(prev.0);
    h.update(canonical_encode(tx).expect("TypedTx is canonical-encodable"));
    let digest: [u8; 32] = h.finalize().into();
    Hash::from_bytes(digest)
}

/// TRACE_MATRIX TB-11 Atom 2 (architect §6.2): TerminalSummary state-root
/// domain. Mirror of `finalize_reward_accept_state_root`.
pub(crate) const TERMINAL_SUMMARY_DOMAIN_V1: &[u8] =
    b"turingosv4.terminal_summary.accept.v1";

/// TRACE_MATRIX TB-11 Atom 2: state-root mutator on `TerminalSummaryTx`
/// accept (writes RunsIndex entry; no money movement).
pub fn terminal_summary_accept_state_root(prev: &Hash, tx: &TypedTx) -> Hash {
    let mut h = Sha256::new();
    h.update(TERMINAL_SUMMARY_DOMAIN_V1);
    h.update(prev.0);
    h.update(canonical_encode(tx).expect("TypedTx is canonical-encodable"));
    let digest: [u8; 32] = h.finalize().into();
    Hash::from_bytes(digest)
}

/// TRACE_MATRIX TB-11 Atom 2 (architect §6.2): TaskBankruptcy state-root
/// domain. Mirror of `finalize_reward_accept_state_root`.
pub(crate) const TASK_BANKRUPTCY_DOMAIN_V1: &[u8] =
    b"turingosv4.task_bankruptcy.accept.v1";

/// TRACE_MATRIX TB-11 Atom 2: state-root mutator on `TaskBankruptcyTx`
/// accept (mutates task_markets_t[task_id].state = Bankrupt + sets
/// bankruptcy_at_logical_t; no money movement).
pub fn task_bankruptcy_accept_state_root(prev: &Hash, tx: &TypedTx) -> Hash {
    let mut h = Sha256::new();
    h.update(TASK_BANKRUPTCY_DOMAIN_V1);
    h.update(prev.0);
    h.update(canonical_encode(tx).expect("TypedTx is canonical-encodable"));
    let digest: [u8; 32] = h.finalize().into();
    Hash::from_bytes(digest)
}

/// TRACE_MATRIX TB-13 Atom 2 (architect 2026-05-03 post-TB-12 ruling Part A
/// §4.3): CompleteSetMint-accept state-root domain.
pub(crate) const COMPLETE_SET_MINT_DOMAIN_V1: &[u8] =
    b"turingosv4.complete_set_mint.accept.v1";

/// TRACE_MATRIX TB-13 Atom 2: state-root mutator on `CompleteSetMintTx`
/// accept. Mirror of `task_open_accept_state_root`.
pub fn complete_set_mint_accept_state_root(prev: &Hash, tx: &TypedTx) -> Hash {
    let mut h = Sha256::new();
    h.update(COMPLETE_SET_MINT_DOMAIN_V1);
    h.update(prev.0);
    h.update(canonical_encode(tx).expect("TypedTx is canonical-encodable"));
    let digest: [u8; 32] = h.finalize().into();
    Hash::from_bytes(digest)
}

/// TRACE_MATRIX TB-13 Atom 2 (architect §4.3): CompleteSetRedeem-accept
/// state-root domain.
pub(crate) const COMPLETE_SET_REDEEM_DOMAIN_V1: &[u8] =
    b"turingosv4.complete_set_redeem.accept.v1";

/// TRACE_MATRIX TB-13 Atom 2: state-root mutator on `CompleteSetRedeemTx`
/// accept. Mirror of `complete_set_mint_accept_state_root`.
pub fn complete_set_redeem_accept_state_root(prev: &Hash, tx: &TypedTx) -> Hash {
    let mut h = Sha256::new();
    h.update(COMPLETE_SET_REDEEM_DOMAIN_V1);
    h.update(prev.0);
    h.update(canonical_encode(tx).expect("TypedTx is canonical-encodable"));
    let digest: [u8; 32] = h.finalize().into();
    Hash::from_bytes(digest)
}

/// TRACE_MATRIX TB-13 Atom 2 (architect §4.3): MarketSeed-accept state-root
/// domain.
pub(crate) const MARKET_SEED_DOMAIN_V1: &[u8] =
    b"turingosv4.market_seed.accept.v1";

/// TRACE_MATRIX TB-13 Atom 2: state-root mutator on `MarketSeedTx` accept.
/// Mirror of `complete_set_mint_accept_state_root`.
pub fn market_seed_accept_state_root(prev: &Hash, tx: &TypedTx) -> Hash {
    let mut h = Sha256::new();
    h.update(MARKET_SEED_DOMAIN_V1);
    h.update(prev.0);
    h.update(canonical_encode(tx).expect("TypedTx is canonical-encodable"));
    let digest: [u8; 32] = h.finalize().into();
    Hash::from_bytes(digest)
}

/// TRACE_MATRIX Stage C P-M2 / Phase F.1 (architect §7.3): CompleteSetMerge-
/// accept state-root domain.
pub(crate) const COMPLETE_SET_MERGE_DOMAIN_V1: &[u8] =
    b"turingosv4.complete_set_merge.accept.v1";

/// TRACE_MATRIX Stage C P-M2 / Phase F.1: state-root mutator on
/// `CompleteSetMergeTx` accept. Mirror of `complete_set_mint_accept_state_root`.
pub fn complete_set_merge_accept_state_root(prev: &Hash, tx: &TypedTx) -> Hash {
    let mut h = Sha256::new();
    h.update(COMPLETE_SET_MERGE_DOMAIN_V1);
    h.update(prev.0);
    h.update(canonical_encode(tx).expect("TypedTx is canonical-encodable"));
    let digest: [u8; 32] = h.finalize().into();
    Hash::from_bytes(digest)
}

/// TRACE_MATRIX FC1-Append Stage C P-M4 / Phase F.3 (architect manual §7.5;
/// remediation directive §1.C row 3): CpmmPool-accept state-root domain.
/// Mirrors sibling `MARKET_SEED_DOMAIN_V1` / `COMPLETE_SET_MERGE_DOMAIN_V1`
/// naming convention (`turingosv4.<purpose>.accept.v1`).
pub(crate) const CPMM_POOL_DOMAIN_V1: &[u8] =
    b"turingosv4.cpmm_pool.accept.v1";

/// TRACE_MATRIX FC1-Append Stage C P-M4 / Phase F.3: state-root mutator on
/// `CpmmPoolTx` accept. Mirror of `complete_set_merge_accept_state_root`
/// (sha256(domain || prev || canonical_encode(tx))).
pub fn cpmm_pool_accept_state_root(prev: &Hash, tx: &TypedTx) -> Hash {
    let mut h = Sha256::new();
    h.update(CPMM_POOL_DOMAIN_V1);
    h.update(prev.0);
    h.update(canonical_encode(tx).expect("TypedTx is canonical-encodable"));
    let digest: [u8; 32] = h.finalize().into();
    Hash::from_bytes(digest)
}

/// TRACE_MATRIX FC1-Append Stage C P-M5 / Phase F.4 (architect manual §7.6;
/// remediation directive §1.C row 4): CpmmSwap-accept state-root domain.
/// Mirrors sibling `CPMM_POOL_DOMAIN_V1` naming convention
/// (`turingosv4.<purpose>.accept.v1`).
pub(crate) const CPMM_SWAP_DOMAIN_V1: &[u8] =
    b"turingosv4.cpmm_swap.accept.v1";

/// TRACE_MATRIX FC1-Append Stage C P-M5 / Phase F.4: state-root mutator on
/// `CpmmSwapTx` accept. Mirror of `cpmm_pool_accept_state_root`
/// (sha256(domain || prev || canonical_encode(tx))).
pub fn cpmm_swap_accept_state_root(prev: &Hash, tx: &TypedTx) -> Hash {
    let mut h = Sha256::new();
    h.update(CPMM_SWAP_DOMAIN_V1);
    h.update(prev.0);
    h.update(canonical_encode(tx).expect("TypedTx is canonical-encodable"));
    let digest: [u8; 32] = h.finalize().into();
    Hash::from_bytes(digest)
}

/// TRACE_MATRIX FC1-Append Stage C P-M6 / Phase F.5 (architect manual §7.7;
/// remediation directive §1.C row 5): BuyWithCoinRouter-accept state-root
/// domain. Mirrors sibling `CPMM_POOL_DOMAIN_V1` / `CPMM_SWAP_DOMAIN_V1`
/// naming convention (`turingosv4.<purpose>.accept.v1`).
pub(crate) const BUY_WITH_COIN_ROUTER_DOMAIN_V1: &[u8] =
    b"turingosv4.buy_with_coin_router.accept.v1";

/// TRACE_MATRIX FC1-Append Stage C P-M6 / Phase F.5: state-root mutator on
/// `BuyWithCoinRouterTx` accept. Mirror of `cpmm_swap_accept_state_root`
/// (sha256(domain || prev || canonical_encode(tx))). The single
/// state_root advance at end of the 9-step composite admission arm is
/// the atomic commit point per architect §7.7 + Codex G2 audit
/// 2026-05-09 defect 2 atomicity requirement.
pub fn buy_with_coin_router_accept_state_root(prev: &Hash, tx: &TypedTx) -> Hash {
    let mut h = Sha256::new();
    h.update(BUY_WITH_COIN_ROUTER_DOMAIN_V1);
    h.update(prev.0);
    h.update(canonical_encode(tx).expect("TypedTx is canonical-encodable"));
    let digest: [u8; 32] = h.finalize().into();
    Hash::from_bytes(digest)
}

/// TRACE_MATRIX FC1-Append Stage C P-M6 / Phase F.5 (E.2 atomic-rollback
/// witness gate; Codex G2 audit 2026-05-09 defect 2):
/// debug-only failure-injection hook for the 9-step composite router
/// admission arm. Reads `TURINGOS_TEST_ROUTER_FAIL_AT_STEP` env var; if
/// set to a step number in 1..=9 matching `current_step`, returns
/// `TestForcedFailure` so the test suite can witness the rollback path.
///
/// **Gate choice — `cfg(debug_assertions)` not `cfg(test)`**: integration
/// tests in `tests/*.rs` link against the *non-test* lib crate (cfg(test)
/// applies only within the crate currently being test-built; the
/// integration-test target builds the lib WITHOUT cfg(test)). Switching
/// to `cfg(debug_assertions)` makes the injection reachable from
/// integration tests AND from `cargo test --lib` AND from dev builds —
/// while still being compiled OUT in `--release` builds (production
/// replay determinism preserved; environment variable cannot influence a
/// release-mode chain).
///
/// Production builds (`cfg(not(debug_assertions))`) compile out the
/// entire body to `Ok(())` — zero runtime cost; the env var is unreadable
/// by design.
///
/// Per E.2 binding pattern (`tests/constitution_class4_atomic_rollback_
/// witness.rs::router_atomic_rollback_on_failure`): tests invoke
/// `std::env::set_var("TURINGOS_TEST_ROUTER_FAIL_AT_STEP", "<n>")` before
/// dispatching the router tx; assert `result.is_err()` AND
/// `q.state_root` UNCHANGED post-failure (atomic rollback witnessed).
#[cfg(debug_assertions)]
fn check_router_test_failure_injection(
    current_step: u8,
) -> Result<(), TransitionError> {
    if let Ok(target) = std::env::var("TURINGOS_TEST_ROUTER_FAIL_AT_STEP") {
        if let Ok(target_step) = target.parse::<u8>() {
            if target_step == current_step {
                return Err(TransitionError::TestForcedFailure);
            }
        }
    }
    Ok(())
}

#[cfg(not(debug_assertions))]
#[inline(always)]
fn check_router_test_failure_injection(
    _current_step: u8,
) -> Result<(), TransitionError> {
    Ok(())
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
        // TB-5 RSP-3.0/3.1 (charter v2 § 4.5):
        TE::SystemTxForbiddenOnAgentIngress => RC::PolicyViolation,
        TE::InvalidSystemSignatureLive => RC::PolicyViolation,
        TE::ChallengeNotFound => RC::PolicyViolation,
        TE::AlreadyResolved => RC::PolicyViolation,
        // TB-8 Atom 3 (charter § 4.5):
        TE::ClaimAlreadyFinalized => RC::PolicyViolation,
        TE::ChallengeWindowStillOpen => RC::PolicyViolation,
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
        // TB-5 RSP-3.0/3.1.
        TransitionError::SystemTxForbiddenOnAgentIngress => Some("system_tx_forbidden_on_agent_ingress".into()),
        TransitionError::InvalidSystemSignatureLive => Some("invalid_system_signature_live".into()),
        TransitionError::ChallengeNotFound => Some("challenge_not_found".into()),
        TransitionError::AlreadyResolved => Some("already_resolved".into()),
        // TB-8 Atom 3.
        TransitionError::ClaimAlreadyFinalized => Some("claim_already_finalized".into()),
        TransitionError::ChallengeWindowStillOpen => Some("challenge_window_still_open".into()),
        _ => Some("policy_violation".into()),
    }
}

// ────────────────────────────────────────────────────────────────────────────
// TB-18R R3 — fine-grained rejection-class refinement via AttemptTelemetry
// (preflight `handover/ai-direct/TB-18R_R3_STEP_B_admission.md` §1.2 + §3.1
// + §3.6)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC1-N42 (R3): refine a base `L4ERejectionClass` (from
/// `rejection_class_for(err)`) into a fine-grained class derived from the
/// `WorkTx.proposal_cid` if that CID resolves to an `AttemptTelemetry` CAS
/// object with a failure-path outcome.
///
/// **Design D** (preflight §3.1): pure-additive on the sequencer; legacy
/// chains replay byte-identical because legacy `proposal_cid` resolves to a
/// `ProposalTelemetry` CAS object, NOT an `AttemptTelemetry`, so
/// `read_attempt_telemetry_from_cas` returns `Err(Codec(...))` and the
/// helper falls back to `base_class` (`PredicateFailed`).
///
/// Refinement only occurs on:
/// - `tx` is a `WorkTx` (other variants don't carry `proposal_cid`);
/// - `base_class == PredicateFailed` (other rejection classes — e.g.
///   `EscrowMissing`, `InsufficientBalance` — keep their existing semantics
///   unchanged; predicate-failure is the only arm where R3 disambiguates).
///
/// **Failure handling** (preflight §3.6): in `cfg(debug_assertions)`,
/// inconsistent state (e.g. `outcome=LeanPass` reaching this helper)
/// panics for early detection. In release builds, log warn + fall back —
/// chain continues.
pub fn refine_rejection_class_via_attempt_telemetry(
    cas: &Arc<RwLock<CasStore>>,
    tx: &TypedTx,
    base_class: L4ERejectionClass,
) -> L4ERejectionClass {
    if base_class != L4ERejectionClass::PredicateFailed {
        return base_class;
    }
    let proposal_cid = match tx {
        TypedTx::Work(w) => w.proposal_cid.clone(),
        _ => return base_class,
    };
    use crate::runtime::attempt_telemetry::{
        read_attempt_telemetry_from_cas, AttemptOutcome,
    };
    // R3.fix (preflight handover/ai-direct/TB-18R_R3FIX_STEP_B_cas_reload.md
    // §3.2 + §3.3): the long-lived sequencer.cas handle has a stale in-memory
    // index relative to evaluator-side handles that wrote AttemptTelemetry on
    // the same disk path AFTER sequencer startup. First read may miss; on miss
    // we promote to a write lock, reload sidecar, drop, retry read once. On
    // second miss we fall back to base_class (legacy ProposalTelemetry CID
    // path; documented behavior preserved).
    let initial = {
        let cas_g = match cas.read() {
            Ok(g) => g,
            Err(_) => return base_class,
        };
        read_attempt_telemetry_from_cas(&cas_g, &proposal_cid)
    };
    let attempt = match initial {
        Ok(a) => a,
        Err(_) => {
            // Reload sidecar to pick up writes from other CasStore handles.
            if let Ok(mut cas_w) = cas.write() {
                let _ = cas_w.reload_index_from_sidecar();
            }
            // Retry once. If still miss → legitimate fallback (legacy
            // ProposalTelemetry CID, corrupt CID, or wrong object type).
            let retry = {
                let cas_g = match cas.read() {
                    Ok(g) => g,
                    Err(_) => return base_class,
                };
                read_attempt_telemetry_from_cas(&cas_g, &proposal_cid)
            };
            match retry {
                Ok(a) => a,
                Err(_) => return base_class,
            }
        }
    };
    match attempt.outcome {
        AttemptOutcome::LeanFail => L4ERejectionClass::LeanFailed,
        AttemptOutcome::ParseFail => L4ERejectionClass::ParseFailed,
        AttemptOutcome::SorryBlock => L4ERejectionClass::SorryBlocked,
        AttemptOutcome::LlmErr => L4ERejectionClass::LlmError,
        AttemptOutcome::LeanPass => {
            #[cfg(debug_assertions)]
            {
                panic!(
                    "TB-18R R3 invariant violation: AttemptTelemetry.outcome=LeanPass \
                     reached predicate-failure rejection arm; proposal_cid is supposed \
                     to point at a *failed* attempt"
                );
            }
            #[cfg(not(debug_assertions))]
            {
                log::warn!(
                    "[tb18r-r3] AttemptTelemetry.outcome=LeanPass on rejection arm; \
                     falling back to PredicateFailed"
                );
                base_class
            }
        }
        AttemptOutcome::Aborted => base_class,
        // TB-18R Phase 2 (2026-05-06): PartialAccepted is the typed
        // step_partial_ok outcome (replaces LeanPass-misnomer). Per R3 §1.3
        // amended, step_partial_ok stays CAS-only; reaching the rejection arm
        // here would be an invariant violation (no L4.E entry expected).
        AttemptOutcome::PartialAccepted => {
            #[cfg(debug_assertions)]
            {
                panic!(
                    "TB-18R R3 invariant violation: AttemptTelemetry.outcome=PartialAccepted \
                     reached predicate-failure rejection arm; step_partial_ok is supposed \
                     to be CAS-only (no L4.E entry per R3 §1.3 amended)"
                );
            }
            #[cfg(not(debug_assertions))]
            {
                log::warn!(
                    "[tb18r-r3] AttemptTelemetry.outcome=PartialAccepted on rejection arm; \
                     falling back to PredicateFailed"
                );
                base_class
            }
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────
// TB-5 Atom 4 — apply_one Stage 1.5 helpers (preflight § 4.5)
//
// `system_message_for_verification`: exhaustively matches the 4 system-emitted
// TypedTx variants and returns the `CanonicalMessage` whose digest the
// system_signature should bind to. Agent variants return `None`. The
// exhaustive match is the contract: any future system variant added to
// `TypedTx` causes a non-exhaustive compile error here, forcing explicit
// handling at the apply-side verification boundary.
//
// `system_signature_of` / `system_epoch_of`: extract the signature + epoch
// from a system-emitted TypedTx variant. Agent variants → `None`.
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX TB-5 charter v2 § 4.5 + preflight § 4.5: exhaustively project
/// a system-emitted `TypedTx` to its `CanonicalMessage` for live signature
/// verification at apply_one stage 1.5. Returns `None` for agent variants
/// (their signatures are agent-domain `AgentSignature`, verified separately
/// at predicate-runner / admission gates).
fn system_message_for_verification(
    tx: &TypedTx,
) -> Option<crate::bottom_white::ledger::system_keypair::CanonicalMessage> {
    use crate::bottom_white::ledger::system_keypair::CanonicalMessage;
    match tx {
        TypedTx::FinalizeReward(t) => {
            let digest = t.to_signing_payload().canonical_digest();
            Some(CanonicalMessage::FinalizeRewardSigning(digest))
        }
        TypedTx::TaskExpire(t) => {
            let digest = t.to_signing_payload().canonical_digest();
            Some(CanonicalMessage::TaskExpireSigning(digest))
        }
        TypedTx::TerminalSummary(t) => {
            let digest = t.to_signing_payload().canonical_digest();
            Some(CanonicalMessage::TerminalSummarySigning(digest))
        }
        TypedTx::ChallengeResolve(t) => {
            let digest = t.to_signing_payload().canonical_digest();
            Some(CanonicalMessage::ChallengeResolveSigning(digest))
        }
        // TB-11 Atom 1: TaskBankruptcyTx is system-emitted; verify against
        // its signing payload digest under the TaskBankruptcySigning canonical
        // message domain.
        TypedTx::TaskBankruptcy(t) => {
            let digest = t.to_signing_payload().canonical_digest();
            Some(CanonicalMessage::TaskBankruptcySigning(digest))
        }
        // Agent-submitted variants: stage 1.5 is system-only. TB-13
        // CompleteSetMint / CompleteSetRedeem / MarketSeed are agent-signed
        // (verified separately at admission via the agent-signature path).
        TypedTx::Work(_)
        | TypedTx::Verify(_)
        | TypedTx::Challenge(_)
        | TypedTx::Reuse(_)
        | TypedTx::TaskOpen(_)
        | TypedTx::EscrowLock(_)
        | TypedTx::CompleteSetMint(_)
        | TypedTx::CompleteSetRedeem(_)
        | TypedTx::MarketSeed(_)
        | TypedTx::CompleteSetMerge(_)
        // Stage C P-M4 / Phase F.3 — agent-signed (provider AgentSignature);
        // verified separately at admission, not at system stage 1.5.
        | TypedTx::CpmmPool(_)
        // Stage C P-M5 / Phase F.4 — agent-signed (trader AgentSignature);
        // verified separately at admission, not at system stage 1.5.
        | TypedTx::CpmmSwap(_)
        // Stage C P-M6 / Phase F.5 — agent-signed (buyer AgentSignature);
        // verified separately at admission, not at system stage 1.5.
        | TypedTx::BuyWithCoinRouter(_) => None,
    }
}

/// TRACE_MATRIX TB-5 Atom 4: extract `&SystemSignature` from a system-emitted
/// TypedTx variant. Agent variants → `None`.
fn system_signature_of(
    tx: &TypedTx,
) -> Option<&crate::bottom_white::ledger::system_keypair::SystemSignature> {
    match tx {
        TypedTx::FinalizeReward(t) => Some(&t.system_signature),
        TypedTx::TaskExpire(t) => Some(&t.system_signature),
        TypedTx::TerminalSummary(t) => Some(&t.system_signature),
        TypedTx::ChallengeResolve(t) => Some(&t.system_signature),
        TypedTx::TaskBankruptcy(t) => Some(&t.system_signature),
        TypedTx::Work(_)
        | TypedTx::Verify(_)
        | TypedTx::Challenge(_)
        | TypedTx::Reuse(_)
        | TypedTx::TaskOpen(_)
        | TypedTx::EscrowLock(_)
        | TypedTx::CompleteSetMint(_)
        | TypedTx::CompleteSetRedeem(_)
        | TypedTx::MarketSeed(_)
        | TypedTx::CompleteSetMerge(_)
        // Stage C P-M4 / Phase F.3 — agent-signed.
        | TypedTx::CpmmPool(_)
        // Stage C P-M5 / Phase F.4 — agent-signed.
        | TypedTx::CpmmSwap(_)
        // Stage C P-M6 / Phase F.5 — agent-signed.
        | TypedTx::BuyWithCoinRouter(_) => None,
    }
}

/// TRACE_MATRIX TB-5 Atom 4: extract `SystemEpoch` from a system-emitted
/// TypedTx variant for pinned-pubkey lookup. Agent variants → `None`.
fn system_epoch_of(tx: &TypedTx) -> Option<SystemEpoch> {
    match tx {
        TypedTx::FinalizeReward(t) => Some(t.epoch),
        TypedTx::TaskExpire(t) => Some(t.epoch),
        // TerminalSummaryTx is signed via opaque digest only (no epoch field
        // in struct per STATE § 1.5 8-field schema). Verification still uses
        // the signing keypair's epoch — but since live verification needs
        // the pinned pubkey for *some* epoch, we fall back to the signing
        // keypair's currently-active epoch. Today TerminalSummary is emitted
        // by the sequencer's runtime keypair under self.epoch; if cross-epoch
        // replay is added the verifier will need to scan all pinned epochs.
        TypedTx::TerminalSummary(_) => None,
        TypedTx::ChallengeResolve(t) => Some(t.epoch),
        TypedTx::TaskBankruptcy(t) => Some(t.epoch),
        TypedTx::Work(_)
        | TypedTx::Verify(_)
        | TypedTx::Challenge(_)
        | TypedTx::Reuse(_)
        | TypedTx::TaskOpen(_)
        | TypedTx::EscrowLock(_)
        | TypedTx::CompleteSetMint(_)
        | TypedTx::CompleteSetRedeem(_)
        | TypedTx::MarketSeed(_)
        | TypedTx::CompleteSetMerge(_)
        // Stage C P-M4 / Phase F.3 — agent-signed (no system epoch).
        | TypedTx::CpmmPool(_)
        // Stage C P-M5 / Phase F.4 — agent-signed (no system epoch).
        | TypedTx::CpmmSwap(_)
        // Stage C P-M6 / Phase F.5 — agent-signed (no system epoch).
        | TypedTx::BuyWithCoinRouter(_) => None,
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
            // ──────────────────────────────────────────────────────────────
            // TB-12 Atom 2 (architect 2026-05-03 ruling §3 + §8 Atom 2):
            // accepted WorkTx with stake > 0 derives a `FirstLong`
            // NodePosition exposure record. Pure additive index write —
            // **no money mutation**, **no change** to balances_t / stakes_t
            // / total_supply (those are handled above by TB-3 economic
            // logic). NodePosition.amount is **NOT a Coin holding** per
            // CR-12.1 + CR-12.2; the 5-holding CTF sum stays unchanged.
            // FR-12.1 + FR-12.4: kind = FirstLong; node_id = work.tx_id;
            // position_id = source_tx = work.tx_id (one-source-tx-one-position
            // invariant for TB-12 per architect §4 last paragraph).
            // ──────────────────────────────────────────────────────────────
            if work.stake.micro_units() > 0 {
                let position = crate::state::typed_tx::NodePosition {
                    position_id: work.tx_id.clone(),
                    node_id: work.tx_id.clone(),
                    task_id: work.task_id.clone(),
                    owner: work.agent_id.clone(),
                    side: crate::state::typed_tx::PositionSide::Long,
                    kind: crate::state::typed_tx::PositionKind::FirstLong,
                    amount: work.stake.0,
                    source_tx: work.tx_id.clone(),
                    opened_at_round: work.timestamp_logical,
                };
                q_next
                    .economic_state_t
                    .node_positions_t
                    .0
                    .insert(work.tx_id.clone(), position);
            }
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
            // ──────────────────────────────────────────────────────────────
            // TB-8 Atom 1 — claims_t writer (charter §3 Atom 1 +
            // ratification §1 Q1/Q3/Q5 + §2.1/§2.2).
            //
            // OMEGA-Confirm path: when verify.verdict == Confirm AND the
            // target_work_tx is in stakes_t, create a ClaimEntry. Per
            // ratification §2.1: VerifyVerdict::Confirm IS the OMEGA verdict
            // (no separate `Omega` variant exists).
            //
            // Single-solver MVP: claim.amount = task_market_entry.total_escrow
            // (ratification §1 Q5 — no fee, no factor). Multi-solver royalty
            // splits are charter §4 forbidden #6.
            //
            // claim_id derivation: `claim-<verify.tx_id>` per ratification
            // §2.2 (deterministic, replay-safe, collision-free).
            //
            // Idempotency: a re-OMEGA on the same target_work_tx is
            // structurally impossible upstream — every accepted VerifyTx has
            // a unique tx_id (sequencer enforces), so the derived claim_id is
            // unique per VerifyTx. A second VerifyTx targeting the same
            // WorkTx would create a SECOND claim entry (its own claim_id),
            // not collide. In single-verifier MVP this case does not arise;
            // multi-verifier (RSP-2) handling is post-TB-8 scope.
            //
            // Claim is created only if (a) verdict == Confirm, (b) the
            // task_market entry exists (must — WorkTx admission already
            // verified total_escrow > 0), and (c) the escrow_lock_tx_id can
            // be resolved (single-solver MVP: pick the first lock_tx in the
            // task_market's escrow_lock_tx_ids set; multi-lock task funding
            // is post-TB-8). If (c) fails, the claim is NOT created — the
            // VerifyTx still accepts (no economic regression).
            // ──────────────────────────────────────────────────────────────
            if verify.verdict == crate::state::typed_tx::VerifyVerdict::Confirm {
                let task_id = target_stake.task_id.clone();
                // TB-8 Atom 1 round-2 (Codex VETO RQ4 fix): one-claim-per-
                // work_tx_id idempotency. A second Confirm VerifyTx targeting
                // the same WorkTx must NOT create a second claim row — that
                // would let aggregate Open claims exceed the backing escrow,
                // making finalize unbackable post-mutation. The Verify itself
                // still accepts (its bond locks; verdict rides L4); only the
                // claim creation is suppressed.
                let already_claimed = q
                    .economic_state_t
                    .claims_t
                    .0
                    .values()
                    .any(|c| c.work_tx_id == verify.target_work_tx);
                if !already_claimed {
                if let Some(task_market) =
                    q.economic_state_t.task_markets_t.0.get(&task_id)
                {
                    if let Some(escrow_lock_tx_id) =
                        task_market.escrow_lock_tx_ids.iter().next().cloned()
                    {
                        let claim_id = crate::state::typed_tx::ClaimId(
                            crate::state::q_state::TxId(format!(
                                "claim-{}",
                                verify.tx_id.0
                            )),
                        );
                        q_next.economic_state_t.claims_t.0.insert(
                            claim_id.0.clone(),
                            crate::state::q_state::ClaimEntry {
                                amount: task_market.total_escrow,
                                claimant: target_stake.staker.clone(),
                                task_id: task_id.clone(),
                                escrow_lock_tx_id,
                                work_tx_id: verify.target_work_tx.clone(),
                                verify_tx_id: verify.tx_id.clone(),
                                status: crate::state::q_state::ClaimStatus::Open,
                                // Zero-window MVP per ratification §1 Q3:
                                // value 0 is the structural marker
                                // "window-closed-immediately" — finalize is
                                // legal as soon as the claim exists. A
                                // future TB introducing a real challenge
                                // window will set this to a non-zero value
                                // (sequencer's logical_t at accept-time + N
                                // blocks). The Atom-3 gate fires only when
                                // this field is > 0 AND fr.timestamp_logical
                                // ≤ this field. agent-supplied
                                // verify.timestamp_logical is intentionally
                                // NOT used here — agent and sequencer
                                // logical_t live in different namespaces.
                                challenge_window_close_logical_t: 0,
                            },
                        );
                    }
                }
                } // end: !already_claimed
            }
            // Step 6: monetary invariants (debit = credit; claim creation is
            // a metadata write — no money moves at claim creation).
            assert_no_post_init_mint(tx, q)
                .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
            assert_total_ctf_conserved(
                &q.economic_state_t,
                &q_next.economic_state_t,
                &[],
            )
            .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
            // TB-8 Atom 1 — intent-vs-backing invariant: any new claim row
            // must have claim.amount ≤ escrow_lock_tx_id's current escrow row.
            assert_claim_amount_backed_by_escrow(&q_next.economic_state_t)
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
                    status: crate::state::q_state::ChallengeStatus::Open, // TB-5 ABI default
                },
            );
            // ──────────────────────────────────────────────────────────────
            // TB-12 Atom 2 (architect 2026-05-03 ruling §3 + §8 Atom 2):
            // accepted ChallengeTx with stake > 0 derives a `ChallengeShort`
            // NodePosition exposure record. Pure additive index write —
            // **no money mutation**, **no change** to balances_t /
            // challenge_cases_t / total_supply (those are handled above by
            // TB-4 economic logic). NodePosition.amount is **NOT a Coin
            // holding** per CR-12.1 + CR-12.2; the 5-holding CTF sum stays
            // unchanged. FR-12.2 + FR-12.5: kind = ChallengeShort; node_id
            // = challenge.target_work_tx; position_id = source_tx =
            // challenge.tx_id. task_id derived via stakes_t[target_work_tx]
            // (the target's stake row holds the task_id backref).
            // ──────────────────────────────────────────────────────────────
            if challenge.stake.micro_units() > 0 {
                // Q-derive task_id from the target WorkTx's stake row.
                let task_id_for_position = q
                    .economic_state_t
                    .stakes_t
                    .0
                    .get(&challenge.target_work_tx)
                    .map(|s| s.task_id.clone())
                    .unwrap_or_default();
                let position = crate::state::typed_tx::NodePosition {
                    position_id: challenge.tx_id.clone(),
                    node_id: challenge.target_work_tx.clone(),
                    task_id: task_id_for_position,
                    owner: challenge.challenger_agent.clone(),
                    side: crate::state::typed_tx::PositionSide::Short,
                    kind: crate::state::typed_tx::PositionKind::ChallengeShort,
                    amount: challenge.stake.0,
                    source_tx: challenge.tx_id.clone(),
                    opened_at_round: challenge.timestamp_logical,
                };
                q_next
                    .economic_state_t
                    .node_positions_t
                    .0
                    .insert(challenge.tx_id.clone(), position);
            }
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
        // ──────────────────────────────────────────────────────────────────
        // TB-8 Atom 3 — FinalizeReward dispatch arm (charter §3 Atom 3 +
        // ratification §1 Q2/Q3/Q4/Q5).
        //
        // Single-solver MVP: debit escrows_t[claim.escrow_lock_tx_id].amount
        // by `reward`, credit balances_t[claim.claimant] by `reward`, flip
        // claims_t[claim_id].status to Finalized. CTF conserved (escrow →
        // balance is a balanced transfer; both are in the post-TB-8 4
        // holdings sum).
        //
        // Q-derived authority: per typed_tx.rs:300-304, task_id / solver /
        // reward on the wire are LEDGER SUMMARY only; the authoritative
        // values are claims_t[claim_id]. Step 4 below cross-verifies wire
        // vs. Q-derived to catch any forgery that bypassed emit_system_tx
        // (defense-in-depth atop the constructive guarantee).
        //
        // Charter forbidden lines respected:
        // - #6 multi-solver royalty splits — single-solver MVP only.
        // - #7 DAG-aware payout splits — claim sweeps total_escrow.
        // - #11 RSP-4 SettlementEngine generalization — settlement_rule_hash
        //   stays opaque; trivial settlement amount = total_escrow.
        // - #12 Slash execution — UpheldDeferred → finalize blocked at
        //   step 3; turning UpheldDeferred into money mutation is post-v1.0.
        // ──────────────────────────────────────────────────────────────────
        TypedTx::FinalizeReward(fr) => {
            // Step 0: parent-root match (Anti-Oreo: every accepted tx
            // commits to a frozen state root).
            if fr.parent_state_root != q.state_root_t {
                return Err(TransitionError::StaleParent);
            }
            // Step 1: lookup claim. ClaimNotFound is a structural error
            // (caller passed an unknown claim_id; emit_system_tx would have
            // caught this at construction time, but defense-in-depth here).
            let claim = match q
                .economic_state_t
                .claims_t
                .0
                .get(fr.claim_id.as_tx_id())
            {
                Some(c) => c.clone(),
                None => return Err(TransitionError::ClaimNotFound),
            };
            // Step 2: idempotency gate.
            match claim.status {
                crate::state::q_state::ClaimStatus::Open => { /* proceed */ }
                crate::state::q_state::ClaimStatus::Finalized => {
                    return Err(TransitionError::ClaimAlreadyFinalized);
                }
                crate::state::q_state::ClaimStatus::Slashed => {
                    return Err(TransitionError::AlreadySlashed);
                }
            }
            // Step 3: ChallengeWindow gate. Zero-window MVP per ratification
            // §1 Q3 + Atom-1 writer: claim.challenge_window_close_logical_t
            // is set to 0 at claim-creation as the "window-closed-immediately"
            // structural marker. The gate fires ONLY when window > 0
            // (forward-compat with future non-zero windows). For TB-8 MVP,
            // the gate is a no-op; the Open→Finalized transition itself is
            // the structural ordering guarantee.
            if claim.challenge_window_close_logical_t > 0
                && fr.timestamp_logical <= claim.challenge_window_close_logical_t
            {
                return Err(TransitionError::ChallengeWindowStillOpen);
            }
            // Step 4: UpheldDeferred challenge gate. If any challenge_cases_t
            // entry targets this claim's verify_tx_id with status =
            // UpheldDeferred, finalize is blocked. (TB-5 RSP-3.1 records
            // UpheldDeferred markers; this gate reads them.)
            //
            // **Conservatism**: a challenge targets the WORK_TX, not the
            // VERIFY_TX. The claim references work_tx_id, so we check
            // challenge_cases_t for any UpheldDeferred entry against the
            // work tx that produced this claim.
            let upheld_blocking = q
                .economic_state_t
                .challenge_cases_t
                .0
                .values()
                .any(|cc| {
                    cc.target_work_tx == claim.work_tx_id
                        && cc.status
                            == crate::state::q_state::ChallengeStatus::UpheldDeferred
                });
            if upheld_blocking {
                return Err(TransitionError::SettlementPredicateFailed(
                    crate::state::typed_tx::PredicateId(
                        "challenge_window_closed_with_no_upheld_challenge".into(),
                    ),
                ));
            }
            // Step 5: Q-derived reward consistency check (anti-forgery).
            // The wire `fr.reward` field is summary-only; the authoritative
            // value is `claim.amount`. A mismatch indicates either a forged
            // tx that bypassed emit_system_tx OR a desync between the
            // claim row at emit-time and the claim row at apply-time
            // (impossible in single-threaded sequencer; defense-in-depth).
            if fr.reward != claim.amount {
                return Err(TransitionError::SettlementPredicateFailed(
                    crate::state::typed_tx::PredicateId(
                        "reward_matches_q_derived".into(),
                    ),
                ));
            }
            if fr.solver != claim.claimant {
                return Err(TransitionError::SettlementPredicateFailed(
                    crate::state::typed_tx::PredicateId(
                        "solver_matches_q_derived".into(),
                    ),
                ));
            }
            if fr.task_id != claim.task_id {
                return Err(TransitionError::SettlementPredicateFailed(
                    crate::state::typed_tx::PredicateId(
                        "task_id_matches_q_derived".into(),
                    ),
                ));
            }
            // Step 6: escrow row exists + has sufficient balance.
            let escrow = match q
                .economic_state_t
                .escrows_t
                .0
                .get(&claim.escrow_lock_tx_id)
            {
                Some(e) => e.clone(),
                None => {
                    return Err(TransitionError::SettlementPredicateFailed(
                        crate::state::typed_tx::PredicateId(
                            "escrow_lock_resolves".into(),
                        ),
                    ));
                }
            };
            if escrow.amount.micro_units() < claim.amount.micro_units() {
                return Err(TransitionError::SettlementPredicateFailed(
                    crate::state::typed_tx::PredicateId(
                        "escrow_sufficient_for_reward".into(),
                    ),
                ));
            }
            // Step 7: atomic mutation — q_next.
            let mut q_next = q.clone();
            // 7a. Debit escrows_t[lock_id] by reward.
            let new_escrow_micro = escrow.amount.micro_units() - claim.amount.micro_units();
            q_next.economic_state_t.escrows_t.0.insert(
                claim.escrow_lock_tx_id.clone(),
                crate::state::q_state::EscrowEntry {
                    amount: crate::economy::money::MicroCoin::from_micro_units(
                        new_escrow_micro,
                    ),
                    depositor: escrow.depositor.clone(),
                    task_id: escrow.task_id.clone(),
                },
            );
            // 7b. Credit balances_t[solver].
            let cur_solver_bal = q
                .economic_state_t
                .balances_t
                .0
                .get(&claim.claimant)
                .copied()
                .unwrap_or_else(crate::economy::money::MicroCoin::zero);
            let new_solver_micro =
                cur_solver_bal.micro_units() + claim.amount.micro_units();
            q_next.economic_state_t.balances_t.0.insert(
                claim.claimant.clone(),
                crate::economy::money::MicroCoin::from_micro_units(new_solver_micro),
            );
            // 7c. Flip claim status to Finalized (entry preserved as
            // historical record per `feedback_no_retroactive_evidence_rewrite`).
            let entry = q_next
                .economic_state_t
                .claims_t
                .0
                .get_mut(fr.claim_id.as_tx_id())
                .expect("verified at step 1");
            entry.status = crate::state::q_state::ClaimStatus::Finalized;
            // 7d. Update task_markets_t cache (total_escrow -= reward).
            // Cache=truth per TB-3 charter §3.2; the escrow debit above
            // changes the derived sum, so the cache must follow.
            if let Some(tm) = q_next
                .economic_state_t
                .task_markets_t
                .0
                .get_mut(&claim.task_id)
            {
                let new_total = tm.total_escrow.micro_units() - claim.amount.micro_units();
                tm.total_escrow =
                    crate::economy::money::MicroCoin::from_micro_units(new_total);
            }
            // Step 8: monetary invariants.
            assert_no_post_init_mint(tx, q)
                .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
            // CTF conserved: escrow -reward + balance +reward = 0 delta on
            // the holding sum. claims_t.status flip is metadata. No
            // exemption needed (ratification §1 Q4 + STEP_B preflight §3).
            assert_total_ctf_conserved(
                &q.economic_state_t,
                &q_next.economic_state_t,
                &[],
            )
            .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
            // TB-3 cache=truth invariant on the affected task.
            assert_task_market_total_escrow_matches_locks(
                &q_next.economic_state_t,
                &claim.task_id,
            )
            .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
            // TB-8 intent-vs-backing invariant: any remaining Open claims
            // must still be backed (this finalize doesn't touch them; the
            // check is fast and catches concurrent dispatch bugs).
            assert_claim_amount_backed_by_escrow(&q_next.economic_state_t)
                .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
            // Step 9: state_root advance via FINALIZE_REWARD_DOMAIN_V1.
            q_next.state_root_t = finalize_reward_accept_state_root(&q.state_root_t, tx);

            Ok((
                q_next,
                SignalBundle::finalize(fr.claim_id.clone(), claim.amount),
            ))
        }
        // ──────────────────────────────────────────────────────────────────
        // TB-11 Atom 2 (architect §6.2 ruling 2026-05-02) — TaskExpire
        // dispatch arm. Refunds escrow → sponsor balance for tasks that
        // exceeded the expiry policy without a Finalized claim.
        //
        // Architect §7.4: "escrow 不能无限期无状态锁死" — every escrow has
        // a deadline; if no FinalizeReward by deadline, system MUST refund.
        //
        // CTF invariant: pure transfer (escrow -amount + balance +amount =
        // 0 delta on holding sum). No mint, no burn.
        //
        // Anti-Oreo: arm fires only when system_signature verified at
        // apply_one stage 1.5; reaches dispatch only via emit_system_tx.
        // ──────────────────────────────────────────────────────────────────
        TypedTx::TaskExpire(expire) => {
            // Step 0: parent-root match.
            if expire.parent_state_root != q.state_root_t {
                return Err(TransitionError::StaleParent);
            }
            // Step 1: task exists.
            let task_entry = match q.economic_state_t.task_markets_t.0.get(&expire.task_id) {
                Some(e) => e.clone(),
                None => return Err(TransitionError::TaskNotFound),
            };
            // Step 2: task lifecycle gate. Already-expired or already-bankrupt-and-refunded
            // tasks reject; finalized tasks reject (cannot refund a paid task).
            // Open and Bankrupt (architect §7.3) ARE expirable — Bankrupt + Expire
            // is the post-bankruptcy capital release path (`reason: BankruptcyTriggered`).
            match task_entry.state {
                crate::state::q_state::TaskMarketState::Open
                | crate::state::q_state::TaskMarketState::Bankrupt => { /* proceed */ }
                crate::state::q_state::TaskMarketState::Expired => {
                    return Err(TransitionError::TaskAlreadyOpen);
                    // Reusing TaskAlreadyOpen for "already-expired" idempotency
                    // gate — same conceptual class (lifecycle terminal-marker
                    // already set). A future refinement can split out a
                    // TaskAlreadyExpired variant.
                }
                crate::state::q_state::TaskMarketState::Finalized => {
                    return Err(TransitionError::ClaimAlreadyFinalized);
                }
            }
            // Step 3: no Finalized claim against this task. Defense-in-depth
            // for replay determinism; emit_system_tx already gates this but
            // dispatch enforces it irrespective of emit-time state.
            let task_has_finalized_claim = q.economic_state_t.claims_t.0.values().any(|c| {
                c.task_id == expire.task_id
                    && c.status == crate::state::q_state::ClaimStatus::Finalized
            });
            if task_has_finalized_claim {
                return Err(TransitionError::ClaimAlreadyFinalized);
            }
            // Step 4: no Open challenge_cases targeting this task's WorkTxs
            // (cannot refund while a dispute is pending). TB-5 carry — open
            // challenge holds the bond, which is a separate holding from
            // escrow but the policy is "wait until challenge resolves".
            let task_has_open_challenge = q.economic_state_t.challenge_cases_t.0.values().any(|cc| {
                let work_for_this_task = q
                    .economic_state_t
                    .stakes_t
                    .0
                    .get(&cc.target_work_tx)
                    .map(|s| s.task_id == expire.task_id)
                    .unwrap_or(false);
                work_for_this_task
                    && cc.status == crate::state::q_state::ChallengeStatus::Open
            });
            if task_has_open_challenge {
                return Err(TransitionError::ChallengeWindowStillOpen);
            }
            // Step 5: escrow row exists + matches sponsor + matches task.
            let escrow = match q.economic_state_t.escrows_t.0.get(&expire.escrow_tx_id) {
                Some(e) => e.clone(),
                None => return Err(TransitionError::EscrowMissing),
            };
            if escrow.task_id != expire.task_id {
                return Err(TransitionError::EscrowMissing);
            }
            if escrow.depositor != expire.sponsor_agent {
                return Err(TransitionError::EscrowMissing);
            }
            // Step 6: Q-derived bounty_refunded consistency. Wire field is
            // ledger summary; Q is authoritative.
            if expire.bounty_refunded != escrow.amount {
                return Err(TransitionError::SettlementPredicateFailed(
                    crate::state::typed_tx::PredicateId(
                        "bounty_refunded_matches_q_derived".into(),
                    ),
                ));
            }
            // Step 7: atomic mutation — q_next.
            let mut q_next = q.clone();
            // 7a. Remove escrow row (refund consumes it; replay-deterministic).
            q_next.economic_state_t.escrows_t.0.remove(&expire.escrow_tx_id);
            // 7b. Credit balances_t[sponsor].
            let cur_sponsor_bal = q
                .economic_state_t
                .balances_t
                .0
                .get(&expire.sponsor_agent)
                .copied()
                .unwrap_or_else(crate::economy::money::MicroCoin::zero);
            let new_sponsor_micro = cur_sponsor_bal.micro_units() + escrow.amount.micro_units();
            q_next.economic_state_t.balances_t.0.insert(
                expire.sponsor_agent.clone(),
                crate::economy::money::MicroCoin::from_micro_units(new_sponsor_micro),
            );
            // 7c. Update task_markets_t cache (total_escrow -= refunded amount;
            // remove escrow_tx_id from set; flip state to Expired).
            if let Some(tm) = q_next.economic_state_t.task_markets_t.0.get_mut(&expire.task_id) {
                let new_total = tm.total_escrow.micro_units() - escrow.amount.micro_units();
                tm.total_escrow =
                    crate::economy::money::MicroCoin::from_micro_units(new_total);
                tm.escrow_lock_tx_ids.remove(&expire.escrow_tx_id);
                tm.state = crate::state::q_state::TaskMarketState::Expired;
            }
            // Step 8: monetary invariants.
            assert_no_post_init_mint(tx, q)
                .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
            assert_total_ctf_conserved(&q.economic_state_t, &q_next.economic_state_t, &[])
                .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
            assert_task_market_total_escrow_matches_locks(&q_next.economic_state_t, &expire.task_id)
                .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
            assert_claim_amount_backed_by_escrow(&q_next.economic_state_t)
                .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
            // Step 9: state_root advance.
            q_next.state_root_t = task_expire_accept_state_root(&q.state_root_t, tx);
            Ok((
                q_next,
                SignalBundle::task_expired(expire.task_id.clone(), escrow.amount),
            ))
        }
        // ──────────────────────────────────────────────────────────────────
        // TB-11 Atom 2 (architect §6.2 ruling 2026-05-02) — TerminalSummary
        // dispatch arm. Anchors a run-level outcome (architect's
        // RunExhaustedTx) on L4 with `evidence_capsule_cid` carrying
        // forward to CAS for O(N) auditability.
        //
        // No money movement; CTF preserved trivially.
        //
        // Idempotency on `run_id` — duplicate emissions for the same run
        // are rejected.
        // ──────────────────────────────────────────────────────────────────
        TypedTx::TerminalSummary(ts) => {
            // Step 0: parent-root match.
            if ts.parent_state_root != q.state_root_t {
                return Err(TransitionError::StaleParent);
            }
            // Step 1: idempotency. RunsIndex key by run_id.
            if q.economic_state_t.runs_t.0.contains_key(&ts.run_id) {
                return Err(TransitionError::TerminalSummaryNotApplicable);
            }
            // Step 2: q_next — write RunSummaryEntry.
            let mut q_next = q.clone();
            let entry = crate::state::q_state::RunSummaryEntry {
                task_id: ts.task_id.clone(),
                run_outcome: ts.run_outcome,
                attempt_count: ts.total_attempts as u64,
                evidence_capsule_cid: ts.evidence_capsule_cid,
                solver_agent: ts.solver_agent.clone(),
                last_logical_t: ts.last_logical_t,
            };
            q_next.economic_state_t.runs_t.0.insert(ts.run_id.clone(), entry);
            // Step 3: monetary invariants. No money moved.
            assert_no_post_init_mint(tx, q)
                .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
            assert_total_ctf_conserved(&q.economic_state_t, &q_next.economic_state_t, &[])
                .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
            // Step 4: state_root advance.
            q_next.state_root_t = terminal_summary_accept_state_root(&q.state_root_t, tx);
            Ok((q_next, SignalBundle::terminal_summary(ts.run_id.clone(), ts.run_outcome)))
        }
        // ──────────────────────────────────────────────────────────────────
        // TB-11 Atom 2 (architect §6.2 ruling 2026-05-02) — TaskBankruptcy
        // dispatch arm. Marks a task as Bankrupt — chain-resident "death
        // certificate" that future TB-12 NodeMarket Short / NO settlement
        // can reference as resolution anchor.
        //
        // No money movement; CTF preserved trivially. (Refund of any
        // remaining escrow is a separate post-bankruptcy TaskExpireTx with
        // `reason: BankruptcyTriggered`.)
        //
        // Idempotency: rejects if task already Bankrupt or Finalized.
        // ──────────────────────────────────────────────────────────────────
        TypedTx::TaskBankruptcy(bk) => {
            // Step 0: parent-root match.
            if bk.parent_state_root != q.state_root_t {
                return Err(TransitionError::StaleParent);
            }
            // Step 1: task exists.
            let task_entry = match q.economic_state_t.task_markets_t.0.get(&bk.task_id) {
                Some(e) => e.clone(),
                None => return Err(TransitionError::TaskNotFound),
            };
            // Step 2: idempotency + lifecycle gate.
            match task_entry.state {
                crate::state::q_state::TaskMarketState::Open
                | crate::state::q_state::TaskMarketState::Expired => { /* proceed */ }
                crate::state::q_state::TaskMarketState::Bankrupt => {
                    return Err(TransitionError::TaskAlreadyOpen);
                    // Idempotent re-bankruptcy refused; reuse TaskAlreadyOpen
                    // pending dedicated TaskAlreadyBankrupt variant.
                }
                crate::state::q_state::TaskMarketState::Finalized => {
                    return Err(TransitionError::ClaimAlreadyFinalized);
                }
            }
            // Step 3: q_next — flip state to Bankrupt + record bankruptcy_at_logical_t.
            let mut q_next = q.clone();
            if let Some(tm) = q_next.economic_state_t.task_markets_t.0.get_mut(&bk.task_id) {
                tm.state = crate::state::q_state::TaskMarketState::Bankrupt;
                tm.bankruptcy_at_logical_t = Some(bk.timestamp_logical);
            }
            // Step 3.5 — TB-15 Atom 3 (architect §6.2) + R2 closure
            // (Gemini R1 VETO Q12 activation gate for replay-determinism):
            // emit deterministic AgentAutopsyCapsule Cids into
            // agent_autopsies_t for each staker losing on the bankrupted
            // task IFF the bankruptcy timestamp_logical is at or past the
            // TB-15 activation cutoff. PURE: no CAS write here —
            // apply_one's post-dispatch hook (Stage 3.5) writes the bytes
            // using the same `derive_autopsies_for_bankruptcy` helper
            // (replay-safe identical Cids). CR-15.1 + halt-trigger #1:
            // Cids are NOT projected to AgentVisibleProjection.
            // `is_autopsy_active_at` defaults true for fresh chains
            // (TB15_AUTOPSY_ACTIVATION_LOGICAL_T=0); pre-TB-15 chain
            // migration would override the constant to skip pre-cutoff
            // rows per `feedback_no_retroactive_evidence_rewrite`.
            if crate::runtime::autopsy_capsule::is_autopsy_active_at(bk.timestamp_logical) {
                let derived =
                    crate::runtime::autopsy_capsule::derive_autopsies_for_bankruptcy(
                        &q.economic_state_t,
                        bk,
                        q.q_t.current_round,
                        bk.timestamp_logical,
                    );
                if !derived.is_empty() {
                    let event_id = crate::state::typed_tx::EventId(bk.task_id.clone());
                    let entry = q_next
                        .economic_state_t
                        .agent_autopsies_t
                        .0
                        .entry(event_id)
                        .or_default();
                    for d in &derived {
                        entry.push(d.capsule.capsule_id);
                    }
                }
            }
            // Step 4: monetary invariants. No money moved.
            assert_no_post_init_mint(tx, q)
                .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
            assert_total_ctf_conserved(&q.economic_state_t, &q_next.economic_state_t, &[])
                .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
            // Step 5: state_root advance.
            q_next.state_root_t = task_bankruptcy_accept_state_root(&q.state_root_t, tx);
            Ok((q_next, SignalBundle::empty()))
        }
        // ──────────────────────────────────────────────────────────────────
        // TB-5 Atom 5+6 — ChallengeResolve arm (charter v2 § 4.6 +
        // preflight § 7.2). Two paths:
        //   Released:        refund challenger bond + flip status to Released
        //                    (entry stays; bond field becomes 0 per directive § 7 Q6)
        //   UpheldDeferred:  marker-only flip to UpheldDeferred; bond preserved
        //                    for TB-6 RSP-3.2 slash routing (no money movement)
        // The 5-holding CTF invariant is preserved: Released's bond-refund is
        // a balanced transfer between holding 5 (challenge_cases.bond) and
        // holding 1 (balances_t); UpheldDeferred touches no holding term.
        // ──────────────────────────────────────────────────────────────────
        TypedTx::ChallengeResolve(resolve) => {
            // Step 1: parent-root match.
            if resolve.parent_state_root != q.state_root_t {
                return Err(TransitionError::StaleParent);
            }
            // Step 2: target ChallengeCase exists.
            let case = match q
                .economic_state_t
                .challenge_cases_t
                .0
                .get(&resolve.target_challenge_tx_id)
            {
                Some(c) => c.clone(),
                None => return Err(TransitionError::ChallengeNotFound),
            };
            // Step 3: idempotency — case must be Open at resolve time.
            if case.status != crate::state::q_state::ChallengeStatus::Open {
                return Err(TransitionError::AlreadyResolved);
            }
            // Step 4: build q_next.
            let mut q_next = q.clone();
            match resolve.resolution {
                crate::state::typed_tx::ChallengeResolution::Released => {
                    // Step 4a: refund challenger.
                    let cur = q
                        .economic_state_t
                        .balances_t
                        .0
                        .get(&case.challenger)
                        .copied()
                        .unwrap_or_else(crate::economy::money::MicroCoin::zero);
                    let new_bal = cur.micro_units() + case.bond.micro_units();
                    q_next.economic_state_t.balances_t.0.insert(
                        case.challenger.clone(),
                        crate::economy::money::MicroCoin::from_micro_units(new_bal),
                    );
                    // Step 4b: zero bond + flip status (entry preserved per
                    // directive § 7 Q6 — audit trail kept).
                    let entry = q_next
                        .economic_state_t
                        .challenge_cases_t
                        .0
                        .get_mut(&resolve.target_challenge_tx_id)
                        .expect("verified at step 2");
                    entry.bond = crate::economy::money::MicroCoin::zero();
                    entry.status = crate::state::q_state::ChallengeStatus::Released;
                }
                crate::state::typed_tx::ChallengeResolution::UpheldDeferred => {
                    // Step 4c: marker only — flip status; bond preserved for
                    // TB-6 RSP-3.2 slash routing. challenger / opened_at_round
                    // / target_work_tx untouched.
                    let entry = q_next
                        .economic_state_t
                        .challenge_cases_t
                        .0
                        .get_mut(&resolve.target_challenge_tx_id)
                        .expect("verified at step 2");
                    entry.status = crate::state::q_state::ChallengeStatus::UpheldDeferred;
                    // bond stays > 0; intentional.
                }
            }
            // Step 5: monetary invariants.
            assert_no_post_init_mint(tx, q)
                .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
            assert_total_ctf_conserved(
                &q.economic_state_t,
                &q_next.economic_state_t,
                &[],
            )
            .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
            // Step 6: state_root advance via CHALLENGE_RESOLVE_DOMAIN_V1.
            q_next.state_root_t = challenge_resolve_accept_state_root(&q.state_root_t, tx);

            Ok((q_next, SignalBundle::default()))
        }
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
                // TB-11 (architect §6.2 ruling 2026-05-02): default lifecycle
                // state Open at TaskOpen dispatch; bankruptcy_at_logical_t
                // None; opened_at_logical_t captures the open-time stamp for
                // tb11_emit_expire_for_eligible deadline policy in Atom 2.
                state: crate::state::q_state::TaskMarketState::Open,
                bankruptcy_at_logical_t: None,
                opened_at_logical_t: open.timestamp_logical,
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
        // ──────────────────────────────────────────────────────────────────
        // TB-13 Atom 2 — CompleteSetMintTx accept arm (architect 2026-05-03
        // post-TB-12 ruling Part A §4.3 + §4.4 FR-13.1..3 + CR-13.1..6).
        //
        //   1 locked Coin → 1 YES_E + 1 NO_E.
        //
        // Debits balances_t[owner] by amount; credits
        // conditional_collateral_t[event_id] by amount; credits BOTH
        // conditional_share_balances_t[owner][event][Yes] and [No] by
        // amount.units. CTF preserved (balance debit = collateral credit;
        // shares are claims, not Coin per CR-13.3 + SG-13.2).
        // ──────────────────────────────────────────────────────────────────
        TypedTx::CompleteSetMint(mint) => {
            // Step 1: parent-root match.
            if mint.parent_state_root != q.state_root_t {
                return Err(TransitionError::StaleParent);
            }
            // Step 2: amount > 0 strictly. `MicroCoin` is i64-backed and
            // permits negative values at the type level (see
            // `src/economy/money.rs`); `<= 0` rejects both zero (no-op
            // mint) and negative (would credit balance + write negative
            // collateral + cast to huge u128 shares). Codex round-1 VETO
            // TB13-V1 remediation (2026-05-03).
            if mint.amount.micro_units() <= 0 {
                return Err(TransitionError::InsufficientBalanceForMint);
            }
            // Step 2.5: event state gate (Gemini round-2 CHALLENGE Q13
            // remediation 2026-05-03). Reject mint against an event
            // whose task_markets_t state is anything but Open. Closes a
            // griefing surface where an agent could mint shares against
            // a Finalized/Bankrupt event, immediately redeem the
            // winning side for full refund, and leave noise on-chain.
            // Missing task_markets_t entry is also rejected (mint
            // requires a task to exist; EventId is 1:1 with TaskId in
            // TB-13 per architect §4.3).
            let market_state = q
                .economic_state_t
                .task_markets_t
                .0
                .get(&mint.event_id.0)
                .map(|m| m.state)
                .ok_or(TransitionError::TaskNotOpen)?;
            if market_state != crate::state::q_state::TaskMarketState::Open {
                return Err(TransitionError::EventNotOpen);
            }
            // Step 3: owner solvency.
            let owner_bal = q
                .economic_state_t
                .balances_t
                .0
                .get(&mint.owner)
                .copied()
                .unwrap_or(crate::economy::money::MicroCoin::zero());
            if owner_bal.micro_units() < mint.amount.micro_units() {
                return Err(TransitionError::InsufficientBalanceForMint);
            }
            // Step 4: build q_next — atomic balance → collateral migration +
            // equal YES_E + NO_E share mint. The 6-holding sum (Atom 3
            // monetary_invariant extension) treats conditional_collateral_t
            // as a Coin holding, so total_supply_micro is preserved
            // bit-for-bit across mint.
            let mut q_next = q.clone();
            let new_bal_micro = owner_bal.micro_units() - mint.amount.micro_units();
            q_next.economic_state_t.balances_t.0.insert(
                mint.owner.clone(),
                crate::economy::money::MicroCoin::from_micro_units(new_bal_micro),
            );
            let collateral_entry = q_next
                .economic_state_t
                .conditional_collateral_t
                .0
                .entry(mint.event_id.clone())
                .or_insert(crate::economy::money::MicroCoin::zero());
            *collateral_entry = crate::economy::money::MicroCoin::from_micro_units(
                collateral_entry.micro_units() + mint.amount.micro_units(),
            );
            let owner_shares = q_next
                .economic_state_t
                .conditional_share_balances_t
                .0
                .entry(mint.owner.clone())
                .or_insert_with(std::collections::BTreeMap::new);
            let pair = owner_shares
                .entry(mint.event_id.clone())
                .or_insert(crate::state::q_state::ShareSidePair::default());
            pair.yes = crate::state::typed_tx::ShareAmount::from_units(
                pair.yes.units + mint.amount.micro_units() as u128,
            );
            pair.no = crate::state::typed_tx::ShareAmount::from_units(
                pair.no.units + mint.amount.micro_units() as u128,
            );

            // Step 5: monetary invariants.
            assert_no_post_init_mint(tx, q)
                .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
            assert_total_ctf_conserved(
                &q.economic_state_t,
                &q_next.economic_state_t,
                &[],
            )
            .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
            // Codex round-2 CHALLENGE remediation 2026-05-03: call
            // assert_complete_set_balanced from dispatch arm (was test-
            // only). This ensures the 1 Coin → 1 YES_E + 1 NO_E identity
            // is enforced live, not just in test fixtures.
            crate::economy::monetary_invariant::assert_complete_set_balanced(
                &q_next.economic_state_t,
            )
            .map_err(|_| TransitionError::MonetaryInvariantViolation)?;

            // Step 6: state_root advance.
            q_next.state_root_t = complete_set_mint_accept_state_root(&q.state_root_t, tx);

            Ok((q_next, SignalBundle::default()))
        }
        // ──────────────────────────────────────────────────────────────────
        // TB-13 Atom 2 — CompleteSetRedeemTx accept arm (architect §4.3 +
        // FR-13.4..5 + SG-13.5..6).
        //
        // Validation:
        //   - task_markets_t[event_id.0].state must be Finalized (Yes) or
        //     Bankrupt (No); else RedeemBeforeResolution.
        //   - redeem.outcome must match the state; else InvalidResolutionRef.
        //   - owner's winning-side share balance must cover share_amount;
        //     else RedeemMoreThanOwned.
        //   - event collateral must cover share_amount; else
        //     InsufficientCollateral.
        //
        // Effect: 1 share → 1 MicroCoin (architect §4.3: "after YES outcome
        // pays YES shares"). Debit shares + collateral; credit balance.
        // ──────────────────────────────────────────────────────────────────
        TypedTx::CompleteSetRedeem(redeem) => {
            if redeem.parent_state_root != q.state_root_t {
                return Err(TransitionError::StaleParent);
            }
            // Step 1: lookup task_markets_t state.
            let market_state = q
                .economic_state_t
                .task_markets_t
                .0
                .get(&redeem.event_id.0)
                .map(|m| m.state)
                .ok_or(TransitionError::RedeemBeforeResolution)?;
            match (market_state, redeem.outcome) {
                (crate::state::q_state::TaskMarketState::Finalized,
                 crate::state::typed_tx::OutcomeSide::Yes) => { /* ok — YES wins */ }
                (crate::state::q_state::TaskMarketState::Bankrupt,
                 crate::state::typed_tx::OutcomeSide::No) => { /* ok — NO wins */ }
                (crate::state::q_state::TaskMarketState::Finalized, _)
                | (crate::state::q_state::TaskMarketState::Bankrupt, _) => {
                    return Err(TransitionError::InvalidResolutionRef);
                }
                (crate::state::q_state::TaskMarketState::Open, _)
                | (crate::state::q_state::TaskMarketState::Expired, _) => {
                    return Err(TransitionError::RedeemBeforeResolution);
                }
            }
            // Step 2: owner's share balance for the winning side.
            let pair = q
                .economic_state_t
                .conditional_share_balances_t
                .0
                .get(&redeem.owner)
                .and_then(|m| m.get(&redeem.event_id))
                .copied()
                .unwrap_or_default();
            let owned_units = match redeem.outcome {
                crate::state::typed_tx::OutcomeSide::Yes => pair.yes.units,
                crate::state::typed_tx::OutcomeSide::No => pair.no.units,
            };
            if owned_units < redeem.share_amount.units {
                return Err(TransitionError::RedeemMoreThanOwned);
            }
            // Step 3: collateral coverage (defensive; should hold if
            // assert_complete_set_balanced is preserved).
            let event_collateral = q
                .economic_state_t
                .conditional_collateral_t
                .0
                .get(&redeem.event_id)
                .copied()
                .unwrap_or(crate::economy::money::MicroCoin::zero());
            if (event_collateral.micro_units() as u128) < redeem.share_amount.units {
                return Err(TransitionError::InsufficientCollateral);
            }

            // Step 4: build q_next.
            let mut q_next = q.clone();
            // 4a: debit the winning side from owner's share balance.
            {
                let owner_shares = q_next
                    .economic_state_t
                    .conditional_share_balances_t
                    .0
                    .entry(redeem.owner.clone())
                    .or_insert_with(std::collections::BTreeMap::new);
                let pair = owner_shares
                    .entry(redeem.event_id.clone())
                    .or_insert(crate::state::q_state::ShareSidePair::default());
                match redeem.outcome {
                    crate::state::typed_tx::OutcomeSide::Yes => {
                        pair.yes = crate::state::typed_tx::ShareAmount::from_units(
                            pair.yes.units - redeem.share_amount.units,
                        );
                    }
                    crate::state::typed_tx::OutcomeSide::No => {
                        pair.no = crate::state::typed_tx::ShareAmount::from_units(
                            pair.no.units - redeem.share_amount.units,
                        );
                    }
                }
            }
            // 4b: debit collateral.
            {
                let collateral_entry = q_next
                    .economic_state_t
                    .conditional_collateral_t
                    .0
                    .entry(redeem.event_id.clone())
                    .or_insert(crate::economy::money::MicroCoin::zero());
                *collateral_entry = crate::economy::money::MicroCoin::from_micro_units(
                    collateral_entry.micro_units() - redeem.share_amount.units as i64,
                );
            }
            // 4c: credit owner's balance 1:1 (1 winning share = 1 MicroCoin).
            let owner_bal = q_next
                .economic_state_t
                .balances_t
                .0
                .get(&redeem.owner)
                .copied()
                .unwrap_or(crate::economy::money::MicroCoin::zero());
            q_next.economic_state_t.balances_t.0.insert(
                redeem.owner.clone(),
                crate::economy::money::MicroCoin::from_micro_units(
                    owner_bal.micro_units() + redeem.share_amount.units as i64,
                ),
            );

            // Step 5: monetary invariants.
            assert_no_post_init_mint(tx, q)
                .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
            assert_total_ctf_conserved(
                &q.economic_state_t,
                &q_next.economic_state_t,
                &[],
            )
            .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
            crate::economy::monetary_invariant::assert_complete_set_balanced(
                &q_next.economic_state_t,
            )
            .map_err(|_| TransitionError::MonetaryInvariantViolation)?;

            // Step 6: state_root advance.
            q_next.state_root_t = complete_set_redeem_accept_state_root(&q.state_root_t, tx);

            Ok((q_next, SignalBundle::default()))
        }
        // ──────────────────────────────────────────────────────────────────
        // TB-13 Atom 2 — MarketSeedTx accept arm (architect §4.3 + FR-13.6..7 +
        // SG-13.3..4). Provider explicitly funds collateral + receives BOTH
        // YES + NO share inventory. **No trading. No quoting. No pricing.**
        // ──────────────────────────────────────────────────────────────────
        TypedTx::MarketSeed(seed) => {
            if seed.parent_state_root != q.state_root_t {
                return Err(TransitionError::StaleParent);
            }
            // Step 1: collateral_amount > 0 strictly (architect SG-13.4).
            // `<= 0` rejects both zero (architect direct mandate) and
            // negative (would mirror the V1 attack on CompleteSetMint —
            // negative collateral + huge u128 shares). Codex round-1
            // VETO TB13-V1 remediation (2026-05-03).
            if seed.collateral_amount.micro_units() <= 0 {
                return Err(TransitionError::InsufficientCollateral);
            }
            // Step 1.5: event state gate (Gemini round-2 CHALLENGE Q13
            // remediation 2026-05-03). Same rationale as CompleteSetMint
            // step 2.5: reject seeding into a closed/missing event.
            let seed_market_state = q
                .economic_state_t
                .task_markets_t
                .0
                .get(&seed.event_id.0)
                .map(|m| m.state)
                .ok_or(TransitionError::TaskNotOpen)?;
            if seed_market_state != crate::state::q_state::TaskMarketState::Open {
                return Err(TransitionError::EventNotOpen);
            }
            // Step 2: provider solvency (architect SG-13.3).
            let provider_bal = q
                .economic_state_t
                .balances_t
                .0
                .get(&seed.provider)
                .copied()
                .unwrap_or(crate::economy::money::MicroCoin::zero());
            if provider_bal.micro_units() < seed.collateral_amount.micro_units() {
                return Err(TransitionError::InsufficientBalanceForMint);
            }
            // Step 3: build q_next — provider balance → collateral + provider
            // receives BOTH YES + NO share inventory.
            let mut q_next = q.clone();
            let new_bal_micro =
                provider_bal.micro_units() - seed.collateral_amount.micro_units();
            q_next.economic_state_t.balances_t.0.insert(
                seed.provider.clone(),
                crate::economy::money::MicroCoin::from_micro_units(new_bal_micro),
            );
            let collateral_entry = q_next
                .economic_state_t
                .conditional_collateral_t
                .0
                .entry(seed.event_id.clone())
                .or_insert(crate::economy::money::MicroCoin::zero());
            *collateral_entry = crate::economy::money::MicroCoin::from_micro_units(
                collateral_entry.micro_units() + seed.collateral_amount.micro_units(),
            );
            let provider_shares = q_next
                .economic_state_t
                .conditional_share_balances_t
                .0
                .entry(seed.provider.clone())
                .or_insert_with(std::collections::BTreeMap::new);
            let pair = provider_shares
                .entry(seed.event_id.clone())
                .or_insert(crate::state::q_state::ShareSidePair::default());
            pair.yes = crate::state::typed_tx::ShareAmount::from_units(
                pair.yes.units + seed.collateral_amount.micro_units() as u128,
            );
            pair.no = crate::state::typed_tx::ShareAmount::from_units(
                pair.no.units + seed.collateral_amount.micro_units() as u128,
            );

            // Step 4: monetary invariants.
            assert_no_post_init_mint(tx, q)
                .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
            assert_total_ctf_conserved(
                &q.economic_state_t,
                &q_next.economic_state_t,
                &[],
            )
            .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
            crate::economy::monetary_invariant::assert_complete_set_balanced(
                &q_next.economic_state_t,
            )
            .map_err(|_| TransitionError::MonetaryInvariantViolation)?;

            // Step 5: state_root advance.
            q_next.state_root_t = market_seed_accept_state_root(&q.state_root_t, tx);

            Ok((q_next, SignalBundle::default()))
        }
        // ──────────────────────────────────────────────────────────────────
        // Stage C P-M2 / Phase F.1 — CompleteSetMergeTx accept arm
        // (architect manual §7.3 verbatim).
        //
        //   1 YES + 1 NO -> 1 Coin (pre-resolution exit).
        //
        // Inverse of CompleteSetMint: burns equal YES + NO from owner; debits
        // conditional_collateral_t[event_id] by amount; credits balances_t
        // [owner] by amount Coin. CTF preserved (collateral debit equals
        // balance credit; YES + NO claim retired symmetrically).
        //
        // Architect §7.3 semantics block contains NO event-state gate — merge
        // remains available post-resolution as long as the owner still holds
        // matching YES + NO inventory (test
        // `merge_unavailable_after_final_redeem_if_shares_exhausted` formalises
        // this: merge is share-balance bounded, not state-bounded).
        // ──────────────────────────────────────────────────────────────────
        TypedTx::CompleteSetMerge(merge) => {
            // Step 1: parent-root match.
            if merge.parent_state_root != q.state_root_t {
                return Err(TransitionError::StaleParent);
            }
            // Step 2: owner share balance — both YES + NO must cover amount
            // (architect §7.3 verbatim "require owner YES >= amount" + "require
            // owner NO >= amount"). Distinct error class from RedeemMoreThanOwned
            // because merge is a non-resolution exit, not a winning-side payout.
            //
            // Zero-amount merge is admitted (strict §7.3 verbatim per
            // `feedback_no_workarounds_strict_constitution`): both preconditions
            // trivially hold for amount=0 (u128 >= 0); steps 4a/4b/4c become
            // `-= 0` / `+= 0` no-ops; assert_complete_set_balanced still holds.
            // Codex R1 CHALLENGE Q3 remediation 2026-05-09 (architect spec
            // does not exclude zero; prior `amount.units == 0` early-reject
            // was extra policy not ratified by §7.3).
            let pair = q
                .economic_state_t
                .conditional_share_balances_t
                .0
                .get(&merge.owner)
                .and_then(|m| m.get(&merge.event_id))
                .copied()
                .unwrap_or_default();
            if pair.yes.units < merge.amount.units {
                return Err(TransitionError::InsufficientSharesForMerge);
            }
            if pair.no.units < merge.amount.units {
                return Err(TransitionError::InsufficientSharesForMerge);
            }
            // Step 3: defensive collateral coverage (should hold under
            // assert_complete_set_balanced; mirror CompleteSetRedeem step 3).
            let event_collateral = q
                .economic_state_t
                .conditional_collateral_t
                .0
                .get(&merge.event_id)
                .copied()
                .unwrap_or(crate::economy::money::MicroCoin::zero());
            if (event_collateral.micro_units() as u128) < merge.amount.units {
                return Err(TransitionError::InsufficientCollateral);
            }

            // Step 4: build q_next — atomic dual-side share burn + collateral
            // debit + balance credit. 1 share-unit = 1 micro-Coin (the same
            // equivalence set at CompleteSetMint time).
            let mut q_next = q.clone();
            // 4a: debit YES + NO each by amount.units.
            {
                let owner_shares = q_next
                    .economic_state_t
                    .conditional_share_balances_t
                    .0
                    .entry(merge.owner.clone())
                    .or_insert_with(std::collections::BTreeMap::new);
                let pair_mut = owner_shares
                    .entry(merge.event_id.clone())
                    .or_insert(crate::state::q_state::ShareSidePair::default());
                pair_mut.yes = crate::state::typed_tx::ShareAmount::from_units(
                    pair_mut.yes.units - merge.amount.units,
                );
                pair_mut.no = crate::state::typed_tx::ShareAmount::from_units(
                    pair_mut.no.units - merge.amount.units,
                );
            }
            // 4b: debit collateral by amount.units (cast to i64 micro-Coin).
            {
                let collateral_entry = q_next
                    .economic_state_t
                    .conditional_collateral_t
                    .0
                    .entry(merge.event_id.clone())
                    .or_insert(crate::economy::money::MicroCoin::zero());
                *collateral_entry = crate::economy::money::MicroCoin::from_micro_units(
                    collateral_entry.micro_units() - merge.amount.units as i64,
                );
            }
            // 4c: credit owner balance 1:1.
            let owner_bal = q_next
                .economic_state_t
                .balances_t
                .0
                .get(&merge.owner)
                .copied()
                .unwrap_or(crate::economy::money::MicroCoin::zero());
            q_next.economic_state_t.balances_t.0.insert(
                merge.owner.clone(),
                crate::economy::money::MicroCoin::from_micro_units(
                    owner_bal.micro_units() + merge.amount.units as i64,
                ),
            );

            // Step 5: monetary invariants — exact mirror of CompleteSetMint /
            // CompleteSetRedeem; merge MUST satisfy
            // assert_complete_set_balanced because it is the bit-for-bit
            // inverse of CompleteSetMint.
            assert_no_post_init_mint(tx, q)
                .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
            assert_total_ctf_conserved(
                &q.economic_state_t,
                &q_next.economic_state_t,
                &[],
            )
            .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
            crate::economy::monetary_invariant::assert_complete_set_balanced(
                &q_next.economic_state_t,
            )
            .map_err(|_| TransitionError::MonetaryInvariantViolation)?;

            // Step 6: state_root advance.
            q_next.state_root_t = complete_set_merge_accept_state_root(&q.state_root_t, tx);

            Ok((q_next, SignalBundle::default()))
        }

        // ──────────────────────────────────────────────────────────────────
        // Stage C P-M4 / Phase F.3 (architect manual §7.5 verbatim 5-field
        // CpmmPool state struct; remediation directive §1.C row 3 Class-4
        // STEP_B rebuild post-VETO).
        //
        // Architect §7.5 specifies STATE struct only; tx schema is
        // implementation-defined (7-field minimal, NO `timestamp_logical`).
        //
        // 5-stage admission preconditions:
        //   1. parent-root match
        //   2. seed_yes > 0 && seed_no > 0          (no degenerate pools)
        //   3. seed_yes == seed_no                  (symmetric init invariant)
        //   4. provider has YES + NO inventory      (collateralized-shares pre)
        //   5. no existing pool for event_id        (one-per-event)
        //
        // Atomic state transitions:
        //   - debit provider's conditional_share_balances_t (yes + no)
        //   - create cpmm_pools_t[event_id] (Active, lp_total = seed_yes)
        //   - credit lp_share_balances_t[(provider, event_id)] += seed_yes
        //
        // Coin-conservation invariant UNCHANGED. conditional_collateral_t
        // UNCHANGED (collateral was already locked at MarketSeed time;
        // pool creation only moves YES + NO claims). Pool reserves and LP
        // shares are NOT Coin per architect §7.5 rules 2 + 3 →
        // `assert_total_ctf_conserved` passes with empty exempt-list.
        // ──────────────────────────────────────────────────────────────────
        TypedTx::CpmmPool(pool) => {
            // Step 1: parent-root match (Inv 5).
            if pool.parent_state_root != q.state_root_t {
                return Err(TransitionError::StaleParent);
            }
            // Step 2: non-zero reserves on both sides (architect §7.5 rule
            // `k = pool_yes * pool_no` requires k > 0).
            if pool.seed_yes.units == 0 || pool.seed_no.units == 0 {
                return Err(TransitionError::InvalidPoolSeed);
            }
            // Step 3: symmetric init invariant. v4 simplification — supports
            // clean `lp_total_shares = seed_yes.units` formula. Asymmetric
            // (geometric-mean init) deferred to a future TB; surfaced as
            // `UnbalancedPoolSeed` so the rejection class is distinct from
            // amount-zero / inventory-shortage.
            if pool.seed_yes.units != pool.seed_no.units {
                return Err(TransitionError::UnbalancedPoolSeed);
            }
            // Step 4: provider must hold seed_yes YES + seed_no NO. Architect
            // §7.5 test `pool_cannot_exist_without_collateralized_shares`
            // exercises this rejection path (provider with empty
            // `conditional_share_balances_t`).
            let pair = q
                .economic_state_t
                .conditional_share_balances_t
                .0
                .get(&pool.provider)
                .and_then(|m| m.get(&pool.event_id))
                .copied()
                .unwrap_or_default();
            if pair.yes.units < pool.seed_yes.units {
                return Err(TransitionError::InsufficientSharesForPool);
            }
            if pair.no.units < pool.seed_no.units {
                return Err(TransitionError::InsufficientSharesForPool);
            }
            // Step 5: one-pool-per-event (architect §7.5 implies `cpmm_pools_t`
            // keyed by `EventId`; double-create rejected idempotently).
            if q
                .economic_state_t
                .cpmm_pools_t
                .0
                .contains_key(&pool.event_id)
            {
                return Err(TransitionError::PoolAlreadyExists);
            }

            // Step 6: build q_next — atomic share-debit + pool-create + LP
            // credit. 1 share-unit moves 1:1 from provider's
            // conditional_share_balances_t into pool's pool_yes/pool_no.
            let mut q_next = q.clone();
            // 6a: debit provider YES + NO inventory.
            {
                let provider_shares = q_next
                    .economic_state_t
                    .conditional_share_balances_t
                    .0
                    .entry(pool.provider.clone())
                    .or_insert_with(std::collections::BTreeMap::new);
                let pair_mut = provider_shares
                    .entry(pool.event_id.clone())
                    .or_insert(crate::state::q_state::ShareSidePair::default());
                pair_mut.yes = crate::state::typed_tx::ShareAmount::from_units(
                    pair_mut.yes.units - pool.seed_yes.units,
                );
                pair_mut.no = crate::state::typed_tx::ShareAmount::from_units(
                    pair_mut.no.units - pool.seed_no.units,
                );
            }
            // 6b: create pool entry (status = Active; lp_total_shares =
            // seed_yes.units per symmetric-init formula).
            q_next.economic_state_t.cpmm_pools_t.0.insert(
                pool.event_id.clone(),
                crate::state::q_state::CpmmPool {
                    event_id: pool.event_id.clone(),
                    pool_yes: pool.seed_yes,
                    pool_no: pool.seed_no,
                    lp_total_shares:
                        crate::state::q_state::LpShareAmount::from_units(
                            pool.seed_yes.units,
                        ),
                    status: crate::state::q_state::PoolStatus::Active,
                },
            );
            // 6c: credit provider with LP shares (1:1 with seed_yes per
            // symmetric-init formula). Tuple-keyed BTreeMap — provider's
            // first LP receipt for this event creates the entry; subsequent
            // pool-creates would fail at step 5 anyway.
            q_next.economic_state_t.lp_share_balances_t.0.insert(
                (pool.provider.clone(), pool.event_id.clone()),
                crate::state::q_state::LpShareAmount::from_units(
                    pool.seed_yes.units,
                ),
            );

            // Step 7: monetary invariants. Coin-side untouched (pool reserves
            // and LP shares are NOT Coin per architect §7.5 rules 2 + 3);
            // assert_total_ctf_conserved with empty exempt-list MUST pass
            // because balances_t / conditional_collateral_t / escrows_t /
            // stakes_t / claims_t / runs_t are all bit-identical. Conditional-
            // share total moves from provider individual to pool reserves —
            // since neither side is in `total_supply_micro`, conservation
            // holds trivially. assert_complete_set_balanced verifies the
            // collateral-vs-shares balance is preserved (pool reserves carry
            // forward the YES + NO claim counts that the provider previously
            // held; collateral lock unchanged).
            assert_no_post_init_mint(tx, q)
                .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
            assert_total_ctf_conserved(
                &q.economic_state_t,
                &q_next.economic_state_t,
                &[],
            )
            .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
            crate::economy::monetary_invariant::assert_complete_set_balanced(
                &q_next.economic_state_t,
            )
            .map_err(|_| TransitionError::MonetaryInvariantViolation)?;

            // Step 8: state_root advance.
            q_next.state_root_t = cpmm_pool_accept_state_root(&q.state_root_t, tx);

            Ok((q_next, SignalBundle::default()))
        }

        // ──────────────────────────────────────────────────────────────────
        // Stage C P-M5 / Phase F.4 (architect manual §7.6 verbatim "Buy YES
        // with NO" / "Symmetric Buy NO with YES"; remediation directive
        // §1.C row 4 verbatim "P-M5 CpmmSwap (re-apply); Class 3").
        //
        // Architect §7.6 specifies BEHAVIOR + 6 mandated tests (no STATE
        // struct; mutates existing CpmmPool fields + ShareSidePair); tx
        // schema is implementation-defined.
        //
        // 6-stage admission preconditions (per `direction`):
        //   1. parent-root match
        //   2. amount_in.units > 0                      (architect dN/dY > 0)
        //   3. pool exists at event_id AND status == Active
        //   4. trader holds amount_in of input side
        //   5. compute out = floor(amount_in * pool_other / (pool_input +
        //      amount_in)); out > 0                     (no dust extractor)
        //   6. out >= min_out                           (slippage budget)
        //
        // Atomic state transitions (3-step):
        //   - debit trader's conditional_share_balances_t (input side)
        //   - update pool reserves (pool_input += amount_in;
        //                           pool_other -= out)
        //   - credit trader's conditional_share_balances_t (output side)
        //
        // Coin invariant UNCHANGED (pure share rotation; no balances_t /
        // conditional_collateral_t mutation). LP shares UNCHANGED.
        //
        // Constant-product invariant `pool_yes1 * pool_no1 >= pool_yes *
        // pool_no` (architect §7.6 verbatim `>=` because integer floor
        // leaves dust in pool — `swap_uses_integer_math_no_f64` test
        // exercises this). Holds by construction: floor rounds the trader's
        // output DOWN (i.e., in the pool's favor), so the pool retains at
        // least the multiplicative invariant.
        //
        // assert_complete_set_balanced (extended in P-M4 to count pool
        // reserves) holds because share totals across (traders + pool)
        // are preserved on each side: pool_input gain == trader_input
        // loss; pool_other loss == trader_other gain.
        // ──────────────────────────────────────────────────────────────────
        TypedTx::CpmmSwap(swap) => {
            use crate::state::typed_tx::SwapDirection;

            // Step 1: parent-root match (Inv 5).
            if swap.parent_state_root != q.state_root_t {
                return Err(TransitionError::StaleParent);
            }
            // Step 2: non-zero input (architect §7.6 verbatim dN > 0 / dY > 0).
            if swap.amount_in.units == 0 {
                return Err(TransitionError::SwapZeroInput);
            }
            // Step 3: pool exists AND is Active. Resolved / Closed pools
            // forward-bound to future TB redemption / unwind path.
            let (pool_input_units, pool_other_units) = {
                let pool = match q.economic_state_t.cpmm_pools_t.0.get(&swap.event_id) {
                    Some(p) => p,
                    None => return Err(TransitionError::PoolNotActive),
                };
                if pool.status != crate::state::q_state::PoolStatus::Active {
                    return Err(TransitionError::PoolNotActive);
                }
                // Project pool reserves onto (input_side, other_side) per
                // direction. `BuyYesWithNo` → input side is NO, other is YES.
                // `BuyNoWithYes` → input side is YES, other is NO.
                match swap.direction {
                    SwapDirection::BuyYesWithNo => {
                        (pool.pool_no.units, pool.pool_yes.units)
                    }
                    SwapDirection::BuyNoWithYes => {
                        (pool.pool_yes.units, pool.pool_no.units)
                    }
                }
            };

            // Step 4: trader holds amount_in of input side.
            let trader_pair = q
                .economic_state_t
                .conditional_share_balances_t
                .0
                .get(&swap.trader)
                .and_then(|m| m.get(&swap.event_id))
                .copied()
                .unwrap_or_default();
            let trader_input_units = match swap.direction {
                SwapDirection::BuyYesWithNo => trader_pair.no.units,
                SwapDirection::BuyNoWithYes => trader_pair.yes.units,
            };
            if trader_input_units < swap.amount_in.units {
                return Err(TransitionError::InsufficientSharesForSwap);
            }

            // Step 5: compute out = floor(amount_in * pool_other /
            // (pool_input + amount_in)). Integer math only — `u128` widens
            // intermediate product to avoid overflow; `swap_uses_integer_
            // math_no_f64` source-grep gate enforces no `f64` / `f32` /
            // `as f..` in this arm.
            let denom = pool_input_units
                .checked_add(swap.amount_in.units)
                .ok_or(TransitionError::MonetaryInvariantViolation)?;
            // denom == 0 is impossible here: amount_in.units > 0 (step 2);
            // overflow already trapped above.
            let numer = swap
                .amount_in
                .units
                .checked_mul(pool_other_units)
                .ok_or(TransitionError::MonetaryInvariantViolation)?;
            let out_units: u128 = numer / denom;

            // Step 6: out > 0 else SwapInsufficientPoolOutput (input too
            // small relative to pool ratio; floor returns zero).
            if out_units == 0 {
                return Err(TransitionError::SwapInsufficientPoolOutput);
            }
            // Step 7: out >= min_out else SwapSlippageExceeded.
            if out_units < swap.min_out.units {
                return Err(TransitionError::SwapSlippageExceeded);
            }

            // Step 8: build q_next — atomic 3-step mutation.
            let mut q_next = q.clone();
            // 8a: debit trader input side; 8c: credit trader output side
            // (combined under one entry mutation).
            {
                let trader_shares = q_next
                    .economic_state_t
                    .conditional_share_balances_t
                    .0
                    .entry(swap.trader.clone())
                    .or_insert_with(std::collections::BTreeMap::new);
                let pair_mut = trader_shares
                    .entry(swap.event_id.clone())
                    .or_insert(crate::state::q_state::ShareSidePair::default());
                match swap.direction {
                    SwapDirection::BuyYesWithNo => {
                        pair_mut.no = crate::state::typed_tx::ShareAmount::from_units(
                            pair_mut.no.units - swap.amount_in.units,
                        );
                        pair_mut.yes = crate::state::typed_tx::ShareAmount::from_units(
                            pair_mut.yes.units + out_units,
                        );
                    }
                    SwapDirection::BuyNoWithYes => {
                        pair_mut.yes = crate::state::typed_tx::ShareAmount::from_units(
                            pair_mut.yes.units - swap.amount_in.units,
                        );
                        pair_mut.no = crate::state::typed_tx::ShareAmount::from_units(
                            pair_mut.no.units + out_units,
                        );
                    }
                }
            }
            // 8b: update pool reserves. Pool input grows by amount_in;
            // pool other shrinks by out.
            {
                let pool_mut = q_next
                    .economic_state_t
                    .cpmm_pools_t
                    .0
                    .get_mut(&swap.event_id)
                    .expect("pool existence checked at Step 3");
                match swap.direction {
                    SwapDirection::BuyYesWithNo => {
                        // Input side = NO; other = YES.
                        pool_mut.pool_no =
                            crate::state::typed_tx::ShareAmount::from_units(
                                pool_mut.pool_no.units + swap.amount_in.units,
                            );
                        pool_mut.pool_yes =
                            crate::state::typed_tx::ShareAmount::from_units(
                                pool_mut.pool_yes.units - out_units,
                            );
                    }
                    SwapDirection::BuyNoWithYes => {
                        // Input side = YES; other = NO.
                        pool_mut.pool_yes =
                            crate::state::typed_tx::ShareAmount::from_units(
                                pool_mut.pool_yes.units + swap.amount_in.units,
                            );
                        pool_mut.pool_no =
                            crate::state::typed_tx::ShareAmount::from_units(
                                pool_mut.pool_no.units - out_units,
                            );
                    }
                }
            }

            // Step 9: monetary invariants. Coin-side untouched; LP shares
            // untouched. assert_total_ctf_conserved with empty exempt-list
            // MUST pass (balances_t / conditional_collateral_t / escrows_t /
            // stakes_t / claims_t / runs_t bit-identical). Conditional-share
            // total per side (traders + pool) preserved bit-for-bit:
            //   sum YES post = sum YES pre  (pool_yes - out moved to trader's yes)
            //   sum NO  post = sum NO  pre  (pool_no  + amount_in came from trader's no)
            // → assert_complete_set_balanced (extended in P-M4 to count pool
            // reserves alongside conditional_share_balances_t) holds.
            assert_no_post_init_mint(tx, q)
                .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
            assert_total_ctf_conserved(
                &q.economic_state_t,
                &q_next.economic_state_t,
                &[],
            )
            .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
            crate::economy::monetary_invariant::assert_complete_set_balanced(
                &q_next.economic_state_t,
            )
            .map_err(|_| TransitionError::MonetaryInvariantViolation)?;

            // Step 10: state_root advance.
            q_next.state_root_t = cpmm_swap_accept_state_root(&q.state_root_t, tx);

            Ok((q_next, SignalBundle::default()))
        }

        // ──────────────────────────────────────────────────────────────────
        // Stage C P-M6 / Phase F.5 (architect manual §7.7 verbatim
        // "BuyYesWithCoinRouter" / "BuyNoWithCoinRouter" 9-step composite;
        // remediation directive §1.C row 5 verbatim "P-M6 BuyWithCoinRouter
        // (rebuild); Class 4 STEP_B; per-atom §8 + PRE-§8 dual audit
        // mandatory"; Codex G2 audit 2026-05-09 defect 1 + defect 2 patches).
        //
        // Defect-1 patch: monetary invariants enforce STRICT equality
        // (`assert_complete_set_balanced` symmetric branch is strict-equality
        // post-Phase E.3 refactor; the router post-state must hit the
        // symmetric branch since Coin->collateral mint is balanced 1:1).
        //
        // Defect-2 patch: cfg(test) failure-injection hook
        // (`check_router_test_failure_injection`) called between EACH of the
        // 9 architect steps so the rollback test can witness the atomic
        // failure path. Rust's `q_next = q.clone()` + final state_root
        // commit pattern provides structural atomicity — failure mid-arm
        // drops `q_next` without persisting; the original `q.state_root_t`
        // is unchanged.
        //
        // Architect §7.7 verbatim 9-step composite (BuyYesWithCoinRouter):
        //   1. Debit buyer Coin by payC.        (balances_t -=)
        //   2. Lock payC collateral.            (conditional_collateral_t +=)
        //   3. Mint payC YES + payC NO to router book-keeping.
        //   4. Transfer payC YES to buyer.      (conditional_share_balances_t.yes +=)
        //   5. Swap payC NO into CPMM pool.     (pool.pool_no += payC.micro)
        //   6. Pool receives dN = payC NO.      (synthetic NO consumed by pool)
        //   7. Router receives outY YES:
        //        outY = floor(payC.micro * pool.pool_yes /
        //                     (pool.pool_no + payC.micro))
        //      (pool.pool_yes -= outY)
        //   8. Transfer outY YES to buyer.      (conditional_share_balances_t.yes += outY)
        //   9. buyer receives getY = payC + outY (cumulative ledger effect).
        //
        // Symmetric `BuyNo` mirrors steps 4/5/7/8 with YES↔NO swap (architect
        // §7.7 "BuyNoWithCoinRouter").
        //
        // Per architect §7.7 integer invariant:
        //   pool_yes1 * pool_no1 >= pool_yes * pool_no    (>= because floor)
        //
        // Coin conservation: balances_t -1 payC; conditional_collateral_t +1
        // payC; net delta = 0. assert_total_ctf_conserved with empty
        // exempt-list MUST pass.
        // ──────────────────────────────────────────────────────────────────
        TypedTx::BuyWithCoinRouter(router) => {
            use crate::state::typed_tx::BuyDirection;

            // === Pre-step admission preconditions (steps 1-prep gate) ===

            // Pre-1: parent-root match (Inv 5).
            if router.parent_state_root != q.state_root_t {
                return Err(TransitionError::StaleParent);
            }
            // Pre-2: pay_coin > 0 (architect §7.7 implies payC > 0).
            if router.pay_coin.micro_units() <= 0 {
                return Err(TransitionError::RouterZeroPay);
            }
            // Pre-3: pool exists AND status == Active.
            let (pool_input_units_pre, pool_other_units_pre) = {
                let pool =
                    match q.economic_state_t.cpmm_pools_t.0.get(&router.event_id) {
                        Some(p) => p,
                        None => return Err(TransitionError::RouterPoolNotActive),
                    };
                if pool.status != crate::state::q_state::PoolStatus::Active {
                    return Err(TransitionError::RouterPoolNotActive);
                }
                // Per direction: BuyYes → input side (pool_no) takes payC,
                // other side (pool_yes) gives outY. BuyNo: symmetric.
                match router.direction {
                    BuyDirection::BuyYes => (pool.pool_no.units, pool.pool_yes.units),
                    BuyDirection::BuyNo => (pool.pool_yes.units, pool.pool_no.units),
                }
            };
            // Pre-4: buyer Coin balance >= payC (architect §7.7 step 1).
            let buyer_balance_pre = q
                .economic_state_t
                .balances_t
                .0
                .get(&router.buyer)
                .copied()
                .unwrap_or_default();
            if buyer_balance_pre.micro_units() < router.pay_coin.micro_units() {
                return Err(TransitionError::RouterInsufficientCoinBalance);
            }
            // Pre-5: compute out_shares = floor(payC * pool_other /
            // (pool_input + payC)). Integer math only.
            let pay_coin_units: u128 = router.pay_coin.micro_units() as u128;
            let denom = pool_input_units_pre
                .checked_add(pay_coin_units)
                .ok_or(TransitionError::MonetaryInvariantViolation)?;
            let numer = pay_coin_units
                .checked_mul(pool_other_units_pre)
                .ok_or(TransitionError::MonetaryInvariantViolation)?;
            let out_shares: u128 = numer / denom;
            // Pre-6: out > 0 else RouterSwapInsufficientPoolOutput.
            if out_shares == 0 {
                return Err(TransitionError::RouterSwapInsufficientPoolOutput);
            }
            // Pre-7: out >= min_out_shares else RouterSlippageExceeded.
            if out_shares < router.min_out_shares.units {
                return Err(TransitionError::RouterSlippageExceeded);
            }

            // === Build q_next: 9 architect steps applied atomically ===
            //
            // Atomicity model: q_next is a FRESH clone of q. All 9 mutations
            // touch q_next ONLY. If any cfg(test) failure-injection fires
            // mid-arm, we early-return Err and Rust drops q_next — the
            // original q is unchanged. The state_root advance at the end is
            // the single atomic commit point.
            let mut q_next = q.clone();
            let pay_coin_micro_i64 = router.pay_coin.micro_units();

            // Step 1 — architect §7.7 verbatim: "Debit buyer Coin by payC."
            check_router_test_failure_injection(1)?;
            {
                let buyer_bal = q_next
                    .economic_state_t
                    .balances_t
                    .0
                    .entry(router.buyer.clone())
                    .or_insert(crate::economy::money::MicroCoin::zero());
                *buyer_bal = buyer_bal
                    .checked_sub(router.pay_coin)
                    .ok_or(TransitionError::MonetaryInvariantViolation)?;
            }

            // Step 2 — architect §7.7 verbatim: "Lock payC collateral."
            check_router_test_failure_injection(2)?;
            {
                let coll = q_next
                    .economic_state_t
                    .conditional_collateral_t
                    .0
                    .entry(router.event_id.clone())
                    .or_insert(crate::economy::money::MicroCoin::zero());
                *coll = coll
                    .checked_add(router.pay_coin)
                    .ok_or(TransitionError::MonetaryInvariantViolation)?;
            }

            // Step 3 — architect §7.7 verbatim: "Mint payC YES + payC NO to
            // router." (book-keeping; in-place via steps 4 + 5 below — the
            // synthetic intermediate router holding never persists in state.
            // This step is the architect's logical conservation explanation:
            // the locked collateral logically represents one complete set,
            // which the router immediately decomposes into the buyer's
            // retained side + the pool's swap side.)
            check_router_test_failure_injection(3)?;

            // Step 4 — architect §7.7 verbatim: "Transfer payC YES to buyer."
            // (BuyYes: trader's YES += payC.micro; BuyNo: trader's NO += payC.micro.)
            check_router_test_failure_injection(4)?;
            {
                let buyer_shares = q_next
                    .economic_state_t
                    .conditional_share_balances_t
                    .0
                    .entry(router.buyer.clone())
                    .or_insert_with(std::collections::BTreeMap::new);
                let pair_mut = buyer_shares
                    .entry(router.event_id.clone())
                    .or_insert(crate::state::q_state::ShareSidePair::default());
                match router.direction {
                    BuyDirection::BuyYes => {
                        pair_mut.yes = crate::state::typed_tx::ShareAmount::from_units(
                            pair_mut.yes.units + pay_coin_units,
                        );
                    }
                    BuyDirection::BuyNo => {
                        pair_mut.no = crate::state::typed_tx::ShareAmount::from_units(
                            pair_mut.no.units + pay_coin_units,
                        );
                    }
                }
            }

            // Step 5 — architect §7.7 verbatim: "Swap payC NO into CPMM pool."
            // (BuyYes: pool.pool_no += payC.micro; BuyNo: pool.pool_yes += payC.micro.)
            check_router_test_failure_injection(5)?;
            // Step 6 — architect §7.7 verbatim: "Pool receives dN = payC NO."
            // (combined with Step 5 — single mutation: pool input side gets payC.)
            check_router_test_failure_injection(6)?;
            // Step 7 — architect §7.7 verbatim: "Router receives outY YES:
            //   outY = floor(payC * poolY / (poolN + payC))"
            // (pool other side decreases by out_shares.)
            check_router_test_failure_injection(7)?;
            {
                let pool_mut = q_next
                    .economic_state_t
                    .cpmm_pools_t
                    .0
                    .get_mut(&router.event_id)
                    .expect("pool existence checked at Pre-3");
                match router.direction {
                    BuyDirection::BuyYes => {
                        // BuyYes: pool_no += payC; pool_yes -= out_shares.
                        pool_mut.pool_no =
                            crate::state::typed_tx::ShareAmount::from_units(
                                pool_mut.pool_no.units + pay_coin_units,
                            );
                        pool_mut.pool_yes =
                            crate::state::typed_tx::ShareAmount::from_units(
                                pool_mut.pool_yes.units - out_shares,
                            );
                    }
                    BuyDirection::BuyNo => {
                        // BuyNo: pool_yes += payC; pool_no -= out_shares.
                        pool_mut.pool_yes =
                            crate::state::typed_tx::ShareAmount::from_units(
                                pool_mut.pool_yes.units + pay_coin_units,
                            );
                        pool_mut.pool_no =
                            crate::state::typed_tx::ShareAmount::from_units(
                                pool_mut.pool_no.units - out_shares,
                            );
                    }
                }
            }

            // Step 8 — architect §7.7 verbatim: "Transfer outY YES to buyer."
            // (BuyYes: buyer's YES += out_shares; BuyNo: buyer's NO += out_shares.)
            check_router_test_failure_injection(8)?;
            {
                let buyer_shares = q_next
                    .economic_state_t
                    .conditional_share_balances_t
                    .0
                    .get_mut(&router.buyer)
                    .expect("buyer entry created at Step 4");
                let pair_mut = buyer_shares
                    .get_mut(&router.event_id)
                    .expect("buyer event entry created at Step 4");
                match router.direction {
                    BuyDirection::BuyYes => {
                        pair_mut.yes = crate::state::typed_tx::ShareAmount::from_units(
                            pair_mut.yes.units + out_shares,
                        );
                    }
                    BuyDirection::BuyNo => {
                        pair_mut.no = crate::state::typed_tx::ShareAmount::from_units(
                            pair_mut.no.units + out_shares,
                        );
                    }
                }
            }

            // Step 9 — architect §7.7 verbatim: "buyer receives getY = payC +
            // outY." (cumulative ledger statement; combined effect of steps
            // 4 + 8. No additional mutation.)
            check_router_test_failure_injection(9)?;

            // === Post-step monetary invariants (Defect-1 patch + CTF) ===
            //
            // assert_no_post_init_mint(tx, q): TypedTx::BuyWithCoinRouter is
            // in the allow-list. Net effect on `total_supply_micro`: ZERO
            // (balances_t -1 payC, conditional_collateral_t +1 payC; both
            // are Coin holdings; symmetric movement).
            //
            // assert_total_ctf_conserved(empty exempt): Coin sum bit-identical
            // pre/post (debit + credit cancel within the 6-holding sum).
            //
            // assert_complete_set_balanced (P-M4 extended to count pool
            // reserves; Phase E.3 refactored to STRICT symmetric-equality):
            // post-state must hit symmetric branch with strict
            // sum_yes == sum_no == collateral. Trace:
            //   pre: sum_yes_traders + pool.pool_yes  ;  sum_no_traders + pool.pool_no  =  collateral
            //   For BuyYes:
            //     post sum_yes = sum_yes_traders + payC + out_shares + (pool.pool_yes - out_shares) =
            //                    sum_yes_traders + pool.pool_yes + payC = pre + payC
            //     post sum_no  = sum_no_traders + (pool.pool_no + payC) = pre + payC
            //     post collateral = pre + payC
            //   sum_yes' == sum_no' == collateral' ✓ (symmetric branch holds).
            //   BuyNo: symmetric, same conclusion.
            assert_no_post_init_mint(tx, q)
                .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
            assert_total_ctf_conserved(
                &q.economic_state_t,
                &q_next.economic_state_t,
                &[],
            )
            .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
            crate::economy::monetary_invariant::assert_complete_set_balanced(
                &q_next.economic_state_t,
            )
            .map_err(|_| TransitionError::MonetaryInvariantViolation)?;

            // === Atomic commit: state_root advance ===
            //
            // Single state-root mutation = atomic commit point. If any of
            // the 9 step injection-checks above returned Err, q_next was
            // dropped before reaching here and q.state_root_t remains
            // unchanged (witnessed by E.2 atomic-rollback gate). The fact
            // that we successfully reach this line implies all 9 steps +
            // 3 monetary invariants passed.
            //
            // pay_coin_micro_i64 binding suppresses unused-var warning on
            // builds where downstream comments reference it.
            let _ = pay_coin_micro_i64;
            q_next.state_root_t =
                buy_with_coin_router_accept_state_root(&q.state_root_t, tx);

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
    /// TB-5.0 Atom 2: agent attempted to submit a system-emitted variant
    /// (FinalizeReward / TaskExpire / TerminalSummary; ChallengeResolve
    /// added in Atom 3) through the agent ingress path. Rejected pre-queue
    /// per Anti-Oreo agent-ingress barrier (charter v2 § 4.9 + preflight
    /// § 3.2; constitutional Art V.1.3 + WP § 12.4: agent ≠ direct state
    /// writer; system-emitted variants must come through `emit_system_tx`).
    SystemTxForbiddenOnAgentIngress,
    /// TB-13 Atom 6 round-3 (Codex VETO TB13-AUTH remediation 2026-05-03):
    /// agent signature verification failed at submit-time admission for a
    /// TB-13 variant (CompleteSetMint / CompleteSetRedeem / MarketSeed)
    /// when the optional `agent_pubkeys` manifest is set. Either the
    /// owner/provider is not registered in the manifest, or the signature
    /// does not match the agent's pinned pubkey for the canonical
    /// signing-payload digest. Rejected pre-queue per Class 3
    /// money/collateral admission control.
    AgentSignatureInvalid,
}

impl std::fmt::Display for SubmitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::QueueFull => write!(f, "submission queue saturated"),
            Self::QueueClosed => write!(f, "submission queue closed"),
            Self::AgentSignatureInvalid => write!(
                f,
                "agent signature verification failed for TB-13 variant; \
                 owner/provider unregistered or signature does not match"
            ),
            Self::SystemTxForbiddenOnAgentIngress => write!(
                f,
                "system-emitted tx variant forbidden on agent ingress; \
                 use Sequencer::emit_system_tx (TB-5.0 Anti-Oreo barrier)"
            ),
        }
    }
}
impl std::error::Error for SubmitError {}

/// TRACE_MATRIX TB-5 charter v2 § 4.2 + preflight § 3.4: high-level command
/// for `Sequencer::emit_system_tx`. Inputs that emit_system_tx accepts; the
/// typed tx struct is constructed + signed inside emit_system_tx, never by
/// the caller. This is the structural Anti-Oreo guarantee — callers cannot
/// pass a forged signature because they don't construct the typed tx.
#[derive(Debug, Clone)]
pub enum SystemEmitCommand {
    /// Resolve a posted ChallengeCase (TB-5.1 Atom 5+6 dispatch path).
    /// `target_challenge_tx_id` keys the existing `challenge_cases_t` row.
    /// `resolution` selects Released (refund + status flip) vs UpheldDeferred
    /// (marker only; slash deferred to TB-6 RSP-3.2).
    ChallengeResolve {
        target_challenge_tx_id: crate::state::q_state::TxId,
        resolution: crate::state::typed_tx::ChallengeResolution,
    },
    /// TB-8 Atom 2 — finalize a reward claim (RSP-4 MVP settlement-spine).
    ///
    /// Caller passes ONLY `claim_id`. The runtime constructs the
    /// `FinalizeRewardTx` by Q-deriving `task_id` / `solver` / `reward` from
    /// `claims_t[claim_id]` (per typed_tx.rs:300-304 anti-forgery doc-comment).
    /// The wire fields exist as a ledger summary; the authoritative values
    /// always come from Q_t.
    ///
    /// Mirrors `ChallengeResolve` shape (single typed key + system-derived
    /// payload). NOT agent-submittable — `submit_agent_tx` rejects every
    /// system-emitted variant per TB-5.0 Atom 2 Anti-Oreo barrier.
    FinalizeReward {
        claim_id: crate::state::typed_tx::ClaimId,
    },
    /// TB-11 Atom 2 (architect §6.2 ruling 2026-05-02) — refund a locked
    /// escrow that exceeded its expiry policy without a Finalized claim.
    ///
    /// Caller passes `task_id` + `escrow_tx_id`; the runtime Q-derives
    /// `sponsor_agent` (from `escrows_t[escrow_tx_id].depositor`),
    /// `bounty_refunded` (from `escrows_t[escrow_tx_id].amount`), and
    /// fills `parent_state_root` + `epoch` + `timestamp_logical` from
    /// the sequencer's current state. `reason` is caller-specified
    /// (Deadline / MaxRunCountReached / ManualSponsorRequest /
    /// BankruptcyTriggered) so the on-chain record names the policy
    /// that triggered expiry.
    TaskExpire {
        task_id: crate::state::q_state::TaskId,
        escrow_tx_id: crate::state::q_state::TxId,
        reason: crate::state::typed_tx::ExpireReason,
    },
    /// TB-11 Atom 2 (architect §6.2 ruling 2026-05-02; ≡ RunExhaustedTx) —
    /// anchor a run-level outcome (typically failure: MaxTxExhausted /
    /// WallClockCap / ComputeCap / ErrorHalt) on L4 with
    /// `evidence_capsule_cid` carrying forward to CAS.
    ///
    /// Caller passes the run-summary fields directly; emit_system_tx fills
    /// `parent_state_root` from current Q + `tx_id` deterministically.
    /// No money movement; idempotent on `run_id`.
    TerminalSummary {
        run_id: crate::state::typed_tx::RunId,
        task_id: crate::state::q_state::TaskId,
        run_outcome: crate::state::typed_tx::RunOutcome,
        total_attempts: u32,
        failure_class_histogram:
            std::collections::BTreeMap<crate::state::typed_tx::RejectionClass, u32>,
        last_logical_t: u64,
        solver_agent: Option<crate::state::q_state::AgentId>,
        evidence_capsule_cid: Option<crate::bottom_white::cas::schema::Cid>,
    },
    /// TB-11 Atom 2 (architect §6.2 ruling 2026-05-02) — task-level death
    /// certificate. Anchors a TaskBankruptcyTx on L4 referencing the
    /// rolled-up evidence capsule.
    ///
    /// Caller passes `task_id` + `evidence_capsule_cid` + `bankruptcy_reason`
    /// + `failed_run_count`; runtime fills parent_state_root + epoch +
    /// timestamp_logical. No money movement (refund is a separate
    /// post-bankruptcy TaskExpire with `reason: BankruptcyTriggered`).
    TaskBankruptcy {
        task_id: crate::state::q_state::TaskId,
        evidence_capsule_cid: crate::bottom_white::cas::schema::Cid,
        bankruptcy_reason: crate::state::typed_tx::BankruptcyReason,
        failed_run_count: u32,
    },
    // Future RSP-3.2 additions (NOT in TB-11 scope):
    //   SlashTx        { ... }   (RSP-3.2)
}

/// TRACE_MATRIX TB-5 charter v2 § 4.2: receipt for `emit_system_tx`.
/// Carries `emit_id` (parallel to agent-side `submit_id`); never `logical_t`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SystemEmitReceipt {
    pub emit_id: u64,
}

/// TRACE_MATRIX TB-5 charter v2 § 4.2 + preflight § 3.5: errors specific to
/// `emit_system_tx`. Distinct from `SubmitError` so tests can match on
/// system-vs-agent ingress failure modes precisely.
#[derive(Debug)]
pub enum EmitSystemError {
    /// Bounded queue saturated.
    QueueFull,
    /// Receiver dropped — sequencer no longer running.
    QueueClosed,
    /// Signing the constructed tx with the system keypair failed.
    SignatureConstruction(KeypairError),
    /// Verification of the just-signed signature failed against pinned
    /// pubkeys for the current epoch. Should not happen in production
    /// (tests pin the runtime keypair's pubkey by-construction); defensive
    /// check that catches keypair/pinned-pubkey desync.
    InvalidSystemSignatureLive,
    /// CAS or other internal lock poisoned during emit.
    InternalLockPoisoned,
    /// TB-8 Atom 2: `SystemEmitCommand::FinalizeReward { claim_id }` referenced
    /// a `claim_id` not present in `claims_t`. Caller-side error (caller
    /// asked for a finalize on a non-existent claim); never reachable from
    /// the production evaluator path because the OMEGA caller derives
    /// `claim_id` from the just-accepted VerifyTx.
    ClaimNotFound,
}

impl std::fmt::Display for EmitSystemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::QueueFull => write!(f, "system-emit queue saturated"),
            Self::QueueClosed => write!(f, "system-emit queue closed"),
            Self::SignatureConstruction(e) => {
                write!(f, "system-tx signature construction failed: {e:?}")
            }
            Self::InvalidSystemSignatureLive => write!(
                f,
                "system_signature failed live verification against pinned pubkeys at emit time"
            ),
            Self::InternalLockPoisoned => write!(f, "internal lock poisoned during emit"),
            Self::ClaimNotFound => write!(
                f,
                "SystemEmitCommand::FinalizeReward referenced a claim_id not present in claims_t"
            ),
        }
    }
}
impl std::error::Error for EmitSystemError {}

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
    /// TB-5 Atom 4 (charter v2 § 4.2 + preflight § 3.5): emit_id is assigned
    /// at `emit_system_tx` ingress; parallel namespace to `next_submit_id`
    /// so agent + system ingress paths advance independently. Both push
    /// to the shared queue via `SubmissionEnvelope`.
    next_emit_id: AtomicU64,

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

    /// TB-5 Atom 4 (charter v2 § 4.3 + preflight § 4.2): pinned system-key
    /// public-key map. Used by apply_one stage 1.5 to verify
    /// `system_signature` on system-emitted variants (defense-in-depth atop
    /// the constructive guarantee from `emit_system_tx`). Tests pin
    /// `self.keypair`'s pubkey under `epoch` for by-construction verification;
    /// production sources from `genesis_payload.toml [system_pubkeys]`.
    pinned_pubkeys: Arc<crate::bottom_white::ledger::system_keypair::PinnedSystemPubkeys>,

    /// TRACE_MATRIX TB-13 Atom 6 round-3 (Codex VETO TB13-AUTH remediation
    /// 2026-05-03): opt-in agent pubkey manifest for submit-time
    /// signature verification of the 3 TB-13 conditional-share variants
    /// (CompleteSetMint / CompleteSetRedeem / MarketSeed).
    ///
    /// **Default state**: empty (`OnceLock::new()`) — preserves
    /// backward-compat with all TB-3..TB-12 callers + test fixtures
    /// using placeholder `[0u8; 64]` signatures (the codebase-wide
    /// agent-sig admission gap is OBS-tracked at
    /// `OBS_AGENT_SIG_REPLAY_GAP_2026-05-03.md` and remains future scope
    /// for the broader codebase).
    ///
    /// **TB-13 enforcement**: when set via [`Sequencer::set_agent_pubkeys`],
    /// `submit_agent_tx` verifies TB-13 variants' signatures against the
    /// pinned pubkeys; failed verification → `SubmitError::AgentSignatureInvalid`.
    /// Closes Codex round-2 VETO TB13-AUTH for Class 3
    /// (money/collateral) admission control.
    agent_pubkeys: std::sync::OnceLock<Arc<crate::runtime::agent_keypairs::AgentPubkeyManifest>>,

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
    ///
    /// **TB-5 Atom 4 signature change** (charter v2 § 4.2 + preflight § 4.2):
    /// added `pinned_pubkeys` parameter. Existing callers (7 src + tests
    /// per Codex round-2 cascade) updated to pass an `Arc<PinnedSystemPubkeys>`
    /// derived from the same keypair (test fixtures) or genesis-pinned
    /// (production). Tests typically pin `keypair.public_key()` under
    /// `epoch` for by-construction signature-verification correctness.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        cas: Arc<RwLock<CasStore>>,
        keypair: Arc<Ed25519Keypair>,
        epoch: SystemEpoch,
        ledger_writer: Arc<RwLock<dyn LedgerWriter>>,
        rejection_writer: Arc<RwLock<RejectionEvidenceWriter>>,
        predicate_registry: Arc<PredicateRegistry>,
        tool_registry: Arc<ToolRegistry>,
        pinned_pubkeys: Arc<crate::bottom_white::ledger::system_keypair::PinnedSystemPubkeys>,
        initial_q: QState,
        queue_capacity: usize,
    ) -> (Self, tokio::sync::mpsc::Receiver<SubmissionEnvelope>) {
        let (queue_tx, queue_rx) = tokio::sync::mpsc::channel(queue_capacity);
        let seq = Self {
            next_submit_id: AtomicU64::new(1),
            next_logical_t: AtomicU64::new(0), // first accepted commit advances to 1
            next_emit_id: AtomicU64::new(1),    // TB-5 Atom 4: parallel system-emit counter
            queue_tx,
            cas,
            keypair,
            epoch,
            ledger_writer,
            rejection_writer,
            predicate_registry,
            tool_registry,
            pinned_pubkeys,
            agent_pubkeys: std::sync::OnceLock::new(),
            q: RwLock::new(initial_q),
        };
        (seq, queue_rx)
    }

    /// TRACE_MATRIX TB-13 Atom 6 round-3 (Codex VETO TB13-AUTH 2026-05-03):
    /// install the agent pubkey manifest for submit-time signature
    /// verification of TB-13 variants. Called once per Sequencer
    /// lifetime (post-construction, pre-first-submit). Returns the
    /// manifest back as `Err` if already set.
    ///
    /// Production binaries plumb this from
    /// `<runtime_repo>/agent_pubkeys.json` after agent registration.
    /// Tests may opt in by constructing an `AgentPubkeyManifest` from
    /// real keypairs.
    pub fn set_agent_pubkeys(
        &self,
        manifest: Arc<crate::runtime::agent_keypairs::AgentPubkeyManifest>,
    ) -> Result<(), Arc<crate::runtime::agent_keypairs::AgentPubkeyManifest>> {
        self.agent_pubkeys.set(manifest)
    }

    /// TRACE_MATRIX TB-5 charter v2 § 4.2: peek pinned_pubkeys (for tests +
    /// observability; production callers should not depend on this).
    #[cfg(test)]
    pub fn pinned_pubkeys(&self) -> &crate::bottom_white::ledger::system_keypair::PinnedSystemPubkeys {
        &self.pinned_pubkeys
    }

    /// TRACE_MATRIX TB-5 charter v2 § 4.2: peek next_emit_id (parallel to
    /// `next_submit_id_peek` for K1-style observability).
    pub fn next_emit_id_peek(&self) -> u64 {
        self.next_emit_id.load(Ordering::SeqCst)
    }

    /// TRACE_MATRIX FC2-Submit + § 5.2.1: TB-5.0 Atom 2 agent-only ingress
    /// barrier (charter v2 § 4.2 + § 4.9 + preflight § 3.2; Anti-Oreo Art V.1.3).
    ///
    /// Accepts ONLY agent-submitted variants. System-emitted variants
    /// (FinalizeReward / TaskExpire / TerminalSummary; ChallengeResolve added
    /// in Atom 3) are rejected pre-queue with
    /// `SubmitError::SystemTxForbiddenOnAgentIngress`. This is the
    /// constitutional Anti-Oreo "agent ≠ direct state writer" boundary,
    /// structurally enforced (was a documented norm without live enforcement
    /// through TB-3 + TB-4; TB-5.0 retires that debt for system-tx).
    ///
    /// **WP-canonical reconciliation**: ChallengeResolveTx (TB-5 Atom 3) +
    /// SlashTx / SettlementTx / ProvisionalAcceptTx / ReputationUpdateTx
    /// (RSP-3.2+ / RSP-4 territory) will be added to the rejection match
    /// at their respective TB landings — each new system variant extends
    /// this list, never bypasses it.
    pub async fn submit_agent_tx(&self, tx: TypedTx) -> Result<SubmissionReceipt, SubmitError> {
        // TB-5.0 ingress barrier: reject 4 system-emitted variants
        // (FinalizeReward / TaskExpire / TerminalSummary added in Atom 2;
        // ChallengeResolve added in Atom 3 when its TypedTx variant landed).
        match &tx {
            TypedTx::FinalizeReward(_)
            | TypedTx::TaskExpire(_)
            | TypedTx::TerminalSummary(_)
            | TypedTx::ChallengeResolve(_)
            // TB-11 Atom 1 (architect §6.2 ruling 2026-05-02): TaskBankruptcyTx
            // is system-emitted only; agent ingress must reject pre-queue per
            // Anti-Oreo (Art V.1.3). Construction goes through emit_system_tx.
            | TypedTx::TaskBankruptcy(_) => {
                return Err(SubmitError::SystemTxForbiddenOnAgentIngress);
            }
            // Agent-submitted variants — proceed to queue. TB-13 conditional-
            // share variants (CompleteSetMint / CompleteSetRedeem / MarketSeed)
            // are agent-signed and admit through the same ingress path.
            TypedTx::Work(_)
            | TypedTx::Verify(_)
            | TypedTx::Challenge(_)
            | TypedTx::Reuse(_)
            | TypedTx::TaskOpen(_)
            | TypedTx::EscrowLock(_)
            | TypedTx::CompleteSetMint(_)
            | TypedTx::CompleteSetRedeem(_)
            | TypedTx::MarketSeed(_)
            // Stage C P-M2 / Phase F.1 — agent-signed; admits through agent
            // ingress path identical to TB-13 conditional-share variants.
            | TypedTx::CompleteSetMerge(_)
            // Stage C P-M4 / Phase F.3 — agent-signed (provider); admits
            // through identical agent ingress path. Pool creation is an
            // economic mutator (debits provider's YES + NO inventory; credits
            // pool reserves + provider LP shares) and is therefore subject
            // to all the same admission gates as TB-13 / P-M2.
            | TypedTx::CpmmPool(_)
            // Stage C P-M5 / Phase F.4 — agent-signed (trader); admits
            // through identical agent ingress path. Pure share rotation
            // between trader and pool reserves; no Coin movement; subject
            // to the same admission gates as P-M4 (manifest-when-set
            // signature gate; replay-time Gate 4 fallback).
            | TypedTx::CpmmSwap(_)
            // Stage C P-M6 / Phase F.5 — agent-signed (buyer); admits
            // through identical agent ingress path. 9-step composite
            // Mint-and-Swap router: Coin payment → collateral lock + YES/NO
            // mint → swap retains buyer's preferred side; subject to the
            // same admission gates as sibling agent-signed variants.
            | TypedTx::BuyWithCoinRouter(_) => {}
        }
        // TRACE_MATRIX TB-13 Atom 6 round-3 (Codex VETO TB13-AUTH 2026-05-03):
        // submit-time agent-signature verification for the 3 TB-13
        // conditional-share variants. Opt-in via `set_agent_pubkeys` —
        // when the manifest is set, forged or unregistered signatures
        // are rejected pre-queue with `SubmitError::AgentSignatureInvalid`.
        // When the manifest is absent (default), this gate is bypassed
        // and replay-time `verify.rs` Gate 4 is the only line of defense
        // (see OBS_AGENT_SIG_REPLAY_GAP_2026-05-03.md).
        if let Some(manifest) = self.agent_pubkeys.get() {
            use crate::runtime::agent_keypairs::verify_agent_signature;
            match &tx {
                TypedTx::CompleteSetMint(mint) => {
                    let pubkey = manifest
                        .get(&mint.owner)
                        .ok_or(SubmitError::AgentSignatureInvalid)?;
                    let digest = mint.to_signing_payload().canonical_digest();
                    if verify_agent_signature(&mint.signature, &digest, &pubkey).is_err() {
                        return Err(SubmitError::AgentSignatureInvalid);
                    }
                }
                TypedTx::CompleteSetRedeem(redeem) => {
                    let pubkey = manifest
                        .get(&redeem.owner)
                        .ok_or(SubmitError::AgentSignatureInvalid)?;
                    let digest = redeem.to_signing_payload().canonical_digest();
                    if verify_agent_signature(&redeem.signature, &digest, &pubkey).is_err() {
                        return Err(SubmitError::AgentSignatureInvalid);
                    }
                }
                TypedTx::MarketSeed(seed) => {
                    let pubkey = manifest
                        .get(&seed.provider)
                        .ok_or(SubmitError::AgentSignatureInvalid)?;
                    let digest = seed.to_signing_payload().canonical_digest();
                    if verify_agent_signature(&seed.signature, &digest, &pubkey).is_err() {
                        return Err(SubmitError::AgentSignatureInvalid);
                    }
                }
                // Stage C P-M2 / Phase F.1 (architect §7.3): agent-signature
                // gate parallel to CompleteSetMint / CompleteSetRedeem /
                // MarketSeed. Owner is the signer; pubkey lookup mirrors
                // CompleteSetMint admission.
                TypedTx::CompleteSetMerge(merge) => {
                    let pubkey = manifest
                        .get(&merge.owner)
                        .ok_or(SubmitError::AgentSignatureInvalid)?;
                    let digest = merge.to_signing_payload().canonical_digest();
                    if verify_agent_signature(&merge.signature, &digest, &pubkey).is_err() {
                        return Err(SubmitError::AgentSignatureInvalid);
                    }
                }
                // Stage C P-M4 / Phase F.3 (architect §7.5): agent-signature
                // gate parallel to MarketSeed (provider is the signer). Pool
                // creation is a Class-3 economic mutator → manifest-when-set
                // gating consistent with sibling P-M3 / P-M2 admission.
                TypedTx::CpmmPool(pool) => {
                    let pubkey = manifest
                        .get(&pool.provider)
                        .ok_or(SubmitError::AgentSignatureInvalid)?;
                    let digest = pool.to_signing_payload().canonical_digest();
                    if verify_agent_signature(&pool.signature, &digest, &pubkey).is_err() {
                        return Err(SubmitError::AgentSignatureInvalid);
                    }
                }
                // Stage C P-M5 / Phase F.4 (architect §7.6): agent-signature
                // gate parallel to CpmmPool (trader is the signer). Swap is a
                // pure share-rotation Class-3 economic mutator → manifest-
                // when-set gating consistent with sibling P-M4 admission.
                TypedTx::CpmmSwap(swap) => {
                    let pubkey = manifest
                        .get(&swap.trader)
                        .ok_or(SubmitError::AgentSignatureInvalid)?;
                    let digest = swap.to_signing_payload().canonical_digest();
                    if verify_agent_signature(&swap.signature, &digest, &pubkey).is_err() {
                        return Err(SubmitError::AgentSignatureInvalid);
                    }
                }
                // Stage C P-M6 / Phase F.5 (architect §7.7): agent-signature
                // gate parallel to CpmmSwap (buyer is the signer). Router is
                // a Class-4 STEP_B economic mutator (9-step composite atomic
                // tx) → manifest-when-set gating consistent with sibling
                // P-M5 admission.
                TypedTx::BuyWithCoinRouter(router) => {
                    let pubkey = manifest
                        .get(&router.buyer)
                        .ok_or(SubmitError::AgentSignatureInvalid)?;
                    let digest = router.to_signing_payload().canonical_digest();
                    if verify_agent_signature(&router.signature, &digest, &pubkey).is_err() {
                        return Err(SubmitError::AgentSignatureInvalid);
                    }
                }
                // Other agent variants are not gated here — codebase-wide
                // forward-dep per OBS_AGENT_SIG_REPLAY_GAP.
                _ => {}
            }
        }
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

    /// TRACE_MATRIX TB-5 Atom 4 (charter v2 § 4.2 + preflight § 3.3): system-only
    /// ingress for system-emitted variants. Constructs the typed tx + signs
    /// internally with the runtime's `Ed25519Keypair` + verifies via
    /// `PinnedSystemPubkeys` (defense-in-depth) before pushing to the queue.
    /// Cannot be invoked with a forged signature because the signature is
    /// constructed from the runtime's own keypair — this is the structural
    /// Anti-Oreo guarantee that complements the agent-side submit_agent_tx
    /// barrier.
    pub async fn emit_system_tx(
        &self,
        command: SystemEmitCommand,
    ) -> Result<SystemEmitReceipt, EmitSystemError> {
        // Step 1: Build the typed tx struct from the command + sign internally.
        let tx = self.build_signed_system_tx(command)?;
        // Step 2: Defense-in-depth — verify the just-signed signature against
        // pinned pubkeys for the current epoch. Tests pin runtime keypair's
        // public key under epoch, so this MUST pass under normal operation.
        // Catches keypair/pinned-pubkey desync.
        self.verify_emitted_system_tx_signature(&tx)?;
        // Step 3: Allocate emit_id (parallel to submit_id; separate counter
        // namespace per charter v2 § 4.2 + preflight § 3.6).
        let emit_id = self.next_emit_id.fetch_add(1, Ordering::SeqCst);
        let envelope = SubmissionEnvelope { submit_id: emit_id, tx };
        // Step 4: Push to shared queue (single queue for both ingress paths;
        // dispatch_transition discriminates by variant TYPE per preflight § 3.6).
        match self.queue_tx.try_send(envelope) {
            Ok(()) => Ok(SystemEmitReceipt { emit_id }),
            Err(tokio::sync::mpsc::error::TrySendError::Full(_)) => Err(EmitSystemError::QueueFull),
            Err(tokio::sync::mpsc::error::TrySendError::Closed(_)) => {
                Err(EmitSystemError::QueueClosed)
            }
        }
    }

    /// TRACE_MATRIX TB-5 Atom 4 (preflight § 4.4): construct + sign a system
    /// tx from a high-level `SystemEmitCommand`. Internal-only; called by
    /// `emit_system_tx`. Each command variant constructs its corresponding
    /// typed tx struct, computes the signing-payload digest, signs with the
    /// runtime's system keypair, and returns the signed `TypedTx`.
    fn build_signed_system_tx(
        &self,
        command: SystemEmitCommand,
    ) -> Result<TypedTx, EmitSystemError> {
        use crate::bottom_white::ledger::system_keypair::terminal_summary_emitter::{
            sign_challenge_resolve, sign_finalize_reward,
        };
        use crate::bottom_white::ledger::system_keypair::SystemSignature;
        use crate::state::typed_tx::{ChallengeResolveTx, FinalizeRewardTx};
        match command {
            SystemEmitCommand::ChallengeResolve { target_challenge_tx_id, resolution } => {
                let q_snap = self
                    .q
                    .read()
                    .map_err(|_| EmitSystemError::InternalLockPoisoned)?;
                let logical_t_for_id = self.next_logical_t.load(Ordering::SeqCst) + 1;
                let mut tx = ChallengeResolveTx {
                    tx_id: crate::state::q_state::TxId(format!(
                        "system-challenge-resolve-{}-{}",
                        self.epoch.get(),
                        logical_t_for_id
                    )),
                    parent_state_root: q_snap.state_root_t,
                    target_challenge_tx_id,
                    resolution,
                    epoch: self.epoch,
                    timestamp_logical: logical_t_for_id,
                    system_signature: SystemSignature::from_bytes([0u8; 64]),  // placeholder
                };
                drop(q_snap);
                let payload = tx.to_signing_payload();
                let digest = payload.canonical_digest();
                let sig = sign_challenge_resolve(&self.keypair, digest)
                    .map_err(EmitSystemError::SignatureConstruction)?;
                tx.system_signature = sig;
                Ok(TypedTx::ChallengeResolve(tx))
            }
            // ──────────────────────────────────────────────────────────────
            // TB-8 Atom 2 — FinalizeReward construction.
            //
            // Caller passes claim_id only. task_id / solver / reward are
            // Q-derived from claims_t[claim_id] (anti-forgery per
            // typed_tx.rs:300-304). Wire fields = ledger summary; Q is
            // authoritative.
            //
            // Idempotency / window / upheld-challenge gates are enforced at
            // the dispatch arm (Atom 3), NOT here. emit_system_tx is the
            // construction layer; dispatch is the validation layer. This
            // separation matches the ChallengeResolve precedent.
            // ──────────────────────────────────────────────────────────────
            SystemEmitCommand::FinalizeReward { claim_id } => {
                let q_snap = self
                    .q
                    .read()
                    .map_err(|_| EmitSystemError::InternalLockPoisoned)?;
                let claim = q_snap
                    .economic_state_t
                    .claims_t
                    .0
                    .get(claim_id.as_tx_id())
                    .ok_or(EmitSystemError::ClaimNotFound)?
                    .clone();
                let logical_t_for_id = self.next_logical_t.load(Ordering::SeqCst) + 1;
                let mut tx = FinalizeRewardTx {
                    tx_id: crate::state::q_state::TxId(format!(
                        "system-finalize-reward-{}-{}",
                        self.epoch.get(),
                        logical_t_for_id
                    )),
                    claim_id,
                    task_id: claim.task_id.clone(),
                    solver: claim.claimant.clone(),
                    reward: claim.amount,
                    parent_state_root: q_snap.state_root_t,
                    epoch: self.epoch,
                    timestamp_logical: logical_t_for_id,
                    system_signature: SystemSignature::from_bytes([0u8; 64]),  // placeholder
                };
                drop(q_snap);
                let payload = tx.to_signing_payload();
                let digest = payload.canonical_digest();
                let sig = sign_finalize_reward(&self.keypair, digest)
                    .map_err(EmitSystemError::SignatureConstruction)?;
                tx.system_signature = sig;
                Ok(TypedTx::FinalizeReward(tx))
            }
            // ─────────────────────────────────────────────────────────────
            // TB-11 Atom 2 (architect §6.2 ruling 2026-05-02) — TaskExpire
            // construction. Caller passes task_id + escrow_tx_id + reason;
            // runtime Q-derives sponsor_agent + bounty_refunded.
            // ─────────────────────────────────────────────────────────────
            SystemEmitCommand::TaskExpire { task_id, escrow_tx_id, reason } => {
                use crate::bottom_white::ledger::system_keypair::terminal_summary_emitter::sign_task_expire;
                use crate::state::typed_tx::TaskExpireTx;
                let q_snap = self
                    .q
                    .read()
                    .map_err(|_| EmitSystemError::InternalLockPoisoned)?;
                let escrow = q_snap
                    .economic_state_t
                    .escrows_t
                    .0
                    .get(&escrow_tx_id)
                    .ok_or(EmitSystemError::ClaimNotFound)?
                    .clone();
                let logical_t_for_id = self.next_logical_t.load(Ordering::SeqCst) + 1;
                let mut tx = TaskExpireTx {
                    tx_id: crate::state::q_state::TxId(format!(
                        "system-task-expire-{}-{}",
                        self.epoch.get(),
                        logical_t_for_id
                    )),
                    task_id,
                    parent_state_root: q_snap.state_root_t,
                    bounty_refunded: escrow.amount,
                    epoch: self.epoch,
                    timestamp_logical: logical_t_for_id,
                    sponsor_agent: escrow.depositor.clone(),
                    escrow_tx_id,
                    reason,
                    system_signature: SystemSignature::from_bytes([0u8; 64]), // placeholder
                };
                drop(q_snap);
                let payload = tx.to_signing_payload();
                let digest = payload.canonical_digest();
                let sig = sign_task_expire(&self.keypair, digest)
                    .map_err(EmitSystemError::SignatureConstruction)?;
                tx.system_signature = sig;
                Ok(TypedTx::TaskExpire(tx))
            }
            // ─────────────────────────────────────────────────────────────
            // TB-11 Atom 2 (architect §6.2 ruling 2026-05-02) —
            // TerminalSummary construction. Caller passes the run-summary
            // fields directly; runtime fills tx_id + parent_state_root +
            // epoch + timestamp_logical.
            // ─────────────────────────────────────────────────────────────
            SystemEmitCommand::TerminalSummary {
                run_id,
                task_id,
                run_outcome,
                total_attempts,
                failure_class_histogram,
                last_logical_t,
                solver_agent,
                evidence_capsule_cid,
            } => {
                use crate::bottom_white::ledger::system_keypair::terminal_summary_emitter::sign_terminal_summary;
                use crate::state::typed_tx::TerminalSummaryTx;
                let q_snap = self
                    .q
                    .read()
                    .map_err(|_| EmitSystemError::InternalLockPoisoned)?;
                let logical_t_for_id = self.next_logical_t.load(Ordering::SeqCst) + 1;
                let mut tx = TerminalSummaryTx {
                    tx_id: crate::state::q_state::TxId(format!(
                        "system-terminal-summary-{}-{}",
                        self.epoch.get(),
                        logical_t_for_id
                    )),
                    task_id,
                    run_id,
                    run_outcome,
                    total_attempts,
                    failure_class_histogram,
                    last_logical_t,
                    parent_state_root: q_snap.state_root_t,
                    solver_agent,
                    evidence_capsule_cid,
                    system_signature: SystemSignature::from_bytes([0u8; 64]), // placeholder
                };
                drop(q_snap);
                let payload = tx.to_signing_payload();
                let digest = payload.canonical_digest();
                let sig = sign_terminal_summary(&self.keypair, digest)
                    .map_err(EmitSystemError::SignatureConstruction)?;
                tx.system_signature = sig;
                Ok(TypedTx::TerminalSummary(tx))
            }
            // ─────────────────────────────────────────────────────────────
            // TB-11 Atom 2 (architect §6.2 ruling 2026-05-02) —
            // TaskBankruptcy construction. Caller passes task_id +
            // evidence_capsule_cid + bankruptcy_reason + failed_run_count;
            // runtime fills tx_id + parent_state_root + epoch + ts_logical.
            // ─────────────────────────────────────────────────────────────
            SystemEmitCommand::TaskBankruptcy {
                task_id,
                evidence_capsule_cid,
                bankruptcy_reason,
                failed_run_count,
            } => {
                use crate::bottom_white::ledger::system_keypair::terminal_summary_emitter::sign_task_bankruptcy;
                use crate::state::typed_tx::TaskBankruptcyTx;
                let q_snap = self
                    .q
                    .read()
                    .map_err(|_| EmitSystemError::InternalLockPoisoned)?;
                let logical_t_for_id = self.next_logical_t.load(Ordering::SeqCst) + 1;
                let mut tx = TaskBankruptcyTx {
                    tx_id: crate::state::q_state::TxId(format!(
                        "system-task-bankruptcy-{}-{}",
                        self.epoch.get(),
                        logical_t_for_id
                    )),
                    parent_state_root: q_snap.state_root_t,
                    task_id,
                    evidence_capsule_cid,
                    bankruptcy_reason,
                    failed_run_count,
                    epoch: self.epoch,
                    timestamp_logical: logical_t_for_id,
                    system_signature: SystemSignature::from_bytes([0u8; 64]), // placeholder
                };
                drop(q_snap);
                let payload = tx.to_signing_payload();
                let digest = payload.canonical_digest();
                let sig = sign_task_bankruptcy(&self.keypair, digest)
                    .map_err(EmitSystemError::SignatureConstruction)?;
                tx.system_signature = sig;
                Ok(TypedTx::TaskBankruptcy(tx))
            }
        }
    }

    /// TRACE_MATRIX TB-5 Atom 4 (preflight § 4.5): defense-in-depth signature
    /// verification at emit time. Verifies the just-signed signature against
    /// pinned pubkeys for the current epoch.
    fn verify_emitted_system_tx_signature(&self, tx: &TypedTx) -> Result<(), EmitSystemError> {
        use crate::bottom_white::ledger::system_keypair::{verify_system_signature, CanonicalMessage};
        match tx {
            TypedTx::ChallengeResolve(t) => {
                let digest = t.to_signing_payload().canonical_digest();
                let msg = CanonicalMessage::ChallengeResolveSigning(digest);
                if !verify_system_signature(&t.system_signature, &msg, t.epoch, &self.pinned_pubkeys) {
                    return Err(EmitSystemError::InvalidSystemSignatureLive);
                }
                Ok(())
            }
            // TB-8 Atom 2 — FinalizeReward defense-in-depth verify.
            TypedTx::FinalizeReward(t) => {
                let digest = t.to_signing_payload().canonical_digest();
                let msg = CanonicalMessage::FinalizeRewardSigning(digest);
                if !verify_system_signature(&t.system_signature, &msg, t.epoch, &self.pinned_pubkeys) {
                    return Err(EmitSystemError::InvalidSystemSignatureLive);
                }
                Ok(())
            }
            // TB-11 Atom 2 — TaskExpire defense-in-depth verify.
            TypedTx::TaskExpire(t) => {
                let digest = t.to_signing_payload().canonical_digest();
                let msg = CanonicalMessage::TaskExpireSigning(digest);
                if !verify_system_signature(&t.system_signature, &msg, t.epoch, &self.pinned_pubkeys) {
                    return Err(EmitSystemError::InvalidSystemSignatureLive);
                }
                Ok(())
            }
            // TB-11 Atom 2 — TerminalSummary defense-in-depth verify.
            // TerminalSummaryTx has no `epoch` field on the wire; verify
            // against the sequencer's current epoch (mirrors apply_one
            // stage 1.5 behavior at system_epoch_of(TerminalSummary) -> None
            // → falls back to current epoch).
            TypedTx::TerminalSummary(t) => {
                let digest = t.to_signing_payload().canonical_digest();
                let msg = CanonicalMessage::TerminalSummarySigning(digest);
                if !verify_system_signature(&t.system_signature, &msg, self.epoch, &self.pinned_pubkeys) {
                    return Err(EmitSystemError::InvalidSystemSignatureLive);
                }
                Ok(())
            }
            // TB-11 Atom 2 — TaskBankruptcy defense-in-depth verify.
            TypedTx::TaskBankruptcy(t) => {
                let digest = t.to_signing_payload().canonical_digest();
                let msg = CanonicalMessage::TaskBankruptcySigning(digest);
                if !verify_system_signature(&t.system_signature, &msg, t.epoch, &self.pinned_pubkeys) {
                    return Err(EmitSystemError::InvalidSystemSignatureLive);
                }
                Ok(())
            }
            // emit_system_tx is system-only — agent variants are unreachable here.
            _ => Ok(()),
        }
    }

    /// TRACE_MATRIX FC2-Submit + § 5.2.1: legacy public submit alias.
    ///
    /// Submit a typed transition (legacy alias; delegates to `submit_agent_tx`
    /// post-TB-5 Atom 2). Returns immediately with a receipt carrying
    /// `submit_id` (NOT `logical_t`). Per Q2 (back-pressure resolution): on
    /// queue saturation returns `Err(SubmitError::QueueFull)` and the agent is
    /// expected to retry with deterministic exponential backoff.
    ///
    /// **TB-5.0 Atom 2 narrowing** (charter v2 § 4.2): this method now
    /// inherits `submit_agent_tx`'s system-variant rejection. Existing
    /// callers (e.g., bus.rs:135-141 `TuringBus::submit_typed_tx`) keep
    /// working unchanged for agent variants. Test fixtures retain backward
    /// compatibility — the only behavioral change is that bare `submit(tx)`
    /// of a system-emitted variant now rejects with
    /// `SubmitError::SystemTxForbiddenOnAgentIngress` instead of silently
    /// queueing for dispatch.
    pub async fn submit(&self, tx: TypedTx) -> Result<SubmissionReceipt, SubmitError> {
        self.submit_agent_tx(tx).await
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

    /// TRACE_MATRIX FC3-S3 (TB-5 Atom 4 preflight § 4.5): factor the L4.E
    /// rejection-writer arm out of `apply_one` so it can be invoked from
    /// BOTH dispatch failures (stage 2) AND signature-verification failures
    /// (stage 1.5). Behavior preserved exactly per the existing TB-2 Atom 4
    /// rejection-writer semantics: no logical_t / state_root / ledger_root
    /// advance. Records:
    /// - tx_payload_cid (canonical-encoded TypedTx)
    /// - raw_diagnostic_cid (TransitionError display, structurally
    ///   serde-shielded on RejectedSubmissionRecord per TB-1 P0-3)
    /// - rejection_class via `rejection_class_for(err)`
    /// - public_summary via `public_summary_for(err)`
    /// - agent_id via `tx.submitter_id().unwrap_or(SYSTEM_AGENT_ID)`
    fn record_rejection(
        &self,
        submit_id: u64,
        tx: &TypedTx,
        q_snapshot: &QState,
        err: &TransitionError,
    ) -> Result<(), ApplyError> {
        let payload_bytes = canonical_encode(tx)
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

        let diag_bytes = err.to_string().into_bytes();
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

        let agent_id = tx
            .submitter_id()
            .unwrap_or_else(|| AgentId(SYSTEM_AGENT_ID_STR.to_string()));

        // TB-18R R3 (preflight §1.2 + §3.1 Design D): refine the base
        // rejection class via AttemptTelemetry when the WorkTx.proposal_cid
        // resolves to one. Pure-additive; legacy proposal_cid → fall-back to
        // base class (PredicateFailed); other rejection arms unchanged.
        let base_class = rejection_class_for(err);
        let refined_class =
            refine_rejection_class_via_attempt_telemetry(&self.cas, tx, base_class);

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
                refined_class,
                raw_diagnostic_cid,
                public_summary_for(err),
            );
        }
        Ok(())
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

        // TB-5 Atom 4 (preflight § 4.5): Stage 1.5 — defense-in-depth signature
        // verification for system-emitted variants. Even though emit_system_tx
        // signs the message before queueing, apply_one re-verifies against
        // pinned_pubkeys here so that any future bypass of emit_system_tx
        // (or stale signature in a replay) is rejected at the apply boundary.
        // On verification failure, route to L4.E with InvalidSystemSignatureLive
        // exactly like a dispatch reject — no logical_t consumed, no state_root
        // advance.
        if let Some(msg) = system_message_for_verification(&tx) {
            use crate::bottom_white::ledger::system_keypair::verify_system_signature;
            let sig = system_signature_of(&tx)
                .expect("system_message_for_verification implies system_signature present");
            // TerminalSummaryTx carries no epoch field (STATE § 1.5 8-field
            // schema is digest-only); fall back to the apply-time sequencer
            // epoch. Other system variants carry epoch on the wire.
            let tx_epoch = system_epoch_of(&tx).unwrap_or(self.epoch);
            if !verify_system_signature(sig, &msg, tx_epoch, &self.pinned_pubkeys) {
                let err = TransitionError::InvalidSystemSignatureLive;
                self.record_rejection(submit_id, &tx, &q_snapshot, &err)?;
                return Err(ApplyError::Transition(err));
            }
        }

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
                self.record_rejection(submit_id, &tx, &q_snapshot, &transition_err)?;
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

        // Stage 3.5 — TB-15 Atom 3 (architect §6.2): post-dispatch autopsy
        // CAS-write hook. For accepted TaskBankruptcyTx, derive the same
        // capsules the dispatch arm pushed Cids for + write their bytes
        // (capsule + private_detail) to CAS. Idempotent: identical bytes
        // → identical Cids → CAS dedupe. Replay-safe: re-running this
        // produces the same CAS state. Failure here is a hard error
        // (ApplyError) — autopsy bytes MUST be retrievable for SG-15.6
        // dashboard regenerability.
        // R2 closure (Gemini R1 VETO Q12): activation-gate the CAS write
        // identically to the dispatch arm. Both gates pin on the same
        // constant TB15_AUTOPSY_ACTIVATION_LOGICAL_T → dispatch and
        // apply_one stay agreement-locked: pre-cutoff rows write nothing
        // to CAS AND populate no agent_autopsies_t Cids.
        if let TypedTx::TaskBankruptcy(bk) = &tx {
            if crate::runtime::autopsy_capsule::is_autopsy_active_at(bk.timestamp_logical) {
                let _ = crate::runtime::autopsy_capsule::write_bankruptcy_autopsies_to_cas(
                    &self.cas,
                    &q_snapshot.economic_state_t,
                    bk,
                    q_snapshot.q_t.current_round,
                    bk.timestamp_logical,
                    &format!("sequencer-epoch-{}", self.epoch.get()),
                )
                .map_err(|e| match e {
                    crate::runtime::autopsy_capsule::AutopsyWriteError::Cas(c) => {
                        ApplyError::Cas(c)
                    }
                    crate::runtime::autopsy_capsule::AutopsyWriteError::Encode(s) => {
                        ApplyError::PayloadEncode(s)
                    }
                    crate::runtime::autopsy_capsule::AutopsyWriteError::InternalLockPoisoned => {
                        ApplyError::QStateLockPoisoned
                    }
                })?;
            }
        }

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

    /// TRACE_MATRIX TB-14 Atom 6 B′ step 4 (FC2-N28; architect ruling
    /// 2026-05-03 §3+§4): build a canonical-keyed parent → children edge
    /// map by walking the L4 chain and reading
    /// `ProposalTelemetry.parent_tx` for each accepted WorkTx via its
    /// `proposal_cid`. Replaces the legacy shadow `kernel.tape`
    /// consumption at the bus snapshot's mask-set derivation site
    /// (canonical-graph rewire closes Codex R1 ship audit VETO; full
    /// detail in `handover/directives/2026-05-03_TB14_ATOM6_VETO_RULING.md`).
    ///
    /// **Replay determinism** (Art.0.2): the L4 chain + CAS payloads are
    /// both replay-deterministic per TB-13 chaintape evidence. Walking
    /// L4 in `read_at` order + reading ProposalTelemetry from CAS
    /// produces a byte-equal `BTreeMap<TxId, BTreeSet<TxId>>` across
    /// live vs replay.
    ///
    /// **Empty fallback**: failures at any layer (lock poisoned, CAS
    /// missing payload, canonical_decode error, ProposalTelemetry
    /// decode error, no parent_tx in telemetry) are silently skipped
    /// rather than propagated — bus.snapshot must NEVER crash because
    /// of an edge-map build failure (consumers handle empty as "no
    /// canonical edges yet"). The L4 chain itself is the canonical
    /// source of truth; this is a derived view.
    ///
    /// **Cost**: O(N + N·CAS_read) per call where N = L4 length. Bus
    /// snapshot frequency is bounded by the evaluator iteration cap;
    /// for a 50-iteration run this is ~50²/2 = 1250 CAS reads total.
    /// A future optimization can cache by writer.len() but is premature
    /// at B′ step 4.
    ///
    /// **TB-9 zero-CID synthetic seed**: legacy synthetic-seed WorkTx
    /// (proposal_cid = `[0u8; 32]`) has no telemetry record; skipped
    /// silently (mirrors `chain_derived_run_facts` line 340 discipline).
    pub fn compute_canonical_edges_at_head(
        &self,
    ) -> std::collections::BTreeMap<crate::state::TxId, std::collections::BTreeSet<crate::state::TxId>>
    {
        use crate::bottom_white::ledger::transition_ledger::canonical_decode;
        use crate::runtime::proposal_telemetry::read_from_cas as read_proposal_telemetry;

        let mut edges: std::collections::BTreeMap<
            crate::state::TxId,
            std::collections::BTreeSet<crate::state::TxId>,
        > = std::collections::BTreeMap::new();

        let writer_r = match self.ledger_writer.read() {
            Ok(g) => g,
            Err(_) => return edges,
        };
        let cas_r = match self.cas.read() {
            Ok(g) => g,
            Err(_) => return edges,
        };

        let n = writer_r.len();
        for t in 1..=n {
            let entry = match writer_r.read_at(t) {
                Ok(e) => e,
                Err(_) => continue,
            };
            // Only Work entries carry parent_tx via ProposalTelemetry.
            if entry.tx_kind != crate::bottom_white::ledger::transition_ledger::TxKind::Work {
                continue;
            }
            let payload = match cas_r.get(&entry.tx_payload_cid) {
                Ok(b) => b,
                Err(_) => continue,
            };
            let typed_tx: TypedTx = match canonical_decode(&payload) {
                Ok(tx) => tx,
                Err(_) => continue,
            };
            let work = match typed_tx {
                TypedTx::Work(w) => w,
                _ => continue,
            };
            // Skip TB-9 zero-CID synthetic seed (no ProposalTelemetry).
            if work.proposal_cid.0 == [0u8; 32] {
                continue;
            }
            let tel = match read_proposal_telemetry(&cas_r, &work.proposal_cid) {
                Ok(t) => t,
                Err(_) => continue,
            };
            if let Some(parent_tx) = tel.parent_tx {
                edges
                    .entry(parent_tx)
                    .or_insert_with(std::collections::BTreeSet::new)
                    .insert(work.tx_id);
            }
        }

        edges
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
        // TB-5 Atom 4: tests pin keypair's own pubkey under the test epoch
        // (preflight § 4.2). emit_system_tx signs with self.keypair, so
        // verification by-construction succeeds when the pinned pubkey for
        // `epoch` matches keypair.public_key().
        let mut pinned = crate::bottom_white::ledger::system_keypair::PinnedSystemPubkeys::new();
        pinned.insert(epoch, keypair.public_key());
        let pinned_pubkeys = Arc::new(pinned);
        let (seq, rx) = Sequencer::new(
            cas,
            keypair,
            epoch,
            writer,
            rejection_writer.clone(),
            preds,
            tools,
            pinned_pubkeys,
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

    // 1. dispatch_transition: NON-WORK / NON-RSP1 / NON-RSP2 / NON-RSP4
    //    variants return NotYetImplemented.
    //
    // TB-2 Atom 3 narrowed this from "all variants" to "non-Work variants".
    // TB-3 narrowed it further (Work + TaskOpen + EscrowLock are now real;
    // their own U/I tests cover them). TB-4 Atom 4-5 narrows further
    // (Verify + Challenge are now real; covered by U12-U21 + I31-I43).
    // **TB-8 Atom 3 narrows further (2026-05-02)**: FinalizeReward is now
    // real (covered by tests/tb_8_minimal_payout.rs I110-I121).
    // **TB-11 Atom 2 narrows further (2026-05-02 architect §6.2 ruling)**:
    // TaskExpire + TerminalSummary + TaskBankruptcy now have real
    // dispatch bodies (covered by tests/tb_11_epistemic_exhaust.rs);
    // ONLY Reuse remains stubbed (post-v1.0 RSP-5 territory).
    #[test]
    fn dispatch_transition_stubs_reuse_only() {
        let q = QState::genesis();
        let preds = PredicateRegistry::new();
        let tools = ToolRegistry::new();

        // Reuse remains the only NotYetImplemented dispatch arm (RSP-5).
        let tx = TypedTx::Reuse(ReuseTx {
            tx_id: TxId("rt".into()),
            reusing_work_tx: TxId("wt".into()),
            reused_tool_id: ToolId("tool".into()),
            reused_tool_creator: AgentId("a".into()),
            timestamp_logical: 1,
        });
        let result = dispatch_transition(&q, &tx, &preds, &tools);
        assert!(matches!(result, Err(TransitionError::NotYetImplemented)));
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
        let epoch = SystemEpoch::new(1);
        // TB-5 Atom 4: pin keypair pubkey under epoch (preflight § 4.2).
        let mut pinned = crate::bottom_white::ledger::system_keypair::PinnedSystemPubkeys::new();
        pinned.insert(epoch, keypair.public_key());
        let pinned_pubkeys = Arc::new(pinned);
        let (seq, _rx) = Sequencer::new(
            cas,
            keypair,
            epoch,
            writer,
            rejection_writer,
            preds,
            tools,
            pinned_pubkeys,
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

    // ── TB-5.0 Atom 2 — agent-ingress barrier tests (charter v2 § 5.3 U22-U26) ──
    //
    // Note: U22 (rejects ChallengeResolveTx) is DEFERRED to Atom 3 because
    // ChallengeResolveTx variant doesn't exist in TypedTx until Atom 3.
    // Atom 2 covers U23-U26: the three system variants that exist at HEAD
    // (FinalizeReward, TaskExpire, TerminalSummary) + accept-path for the
    // 6 agent variants (Work, Verify, Challenge, Reuse, TaskOpen, EscrowLock).
    //
    // When Atom 3 lands ChallengeResolveTx, it extends the submit_agent_tx
    // rejection match to include ChallengeResolve and adds U22 alongside.

    /// U23 — submit_agent_tx rejects FinalizeRewardTx pre-queue with
    /// SystemTxForbiddenOnAgentIngress. submit_id is NOT advanced
    /// (rejection happens before fetch_add).
    #[tokio::test]
    async fn submit_agent_tx_rejects_finalize_reward_pre_queue() {
        let (_tmp, seq, _rx, _rejection_writer) = fresh_sequencer();
        let pre_submit_id = seq.next_submit_id_peek();
        let tx = TypedTx::FinalizeReward(FinalizeRewardTx {
            tx_id: TxId("ft-u23".into()),
            claim_id: ClaimId::new("cl-u23"),
            task_id: TaskId("t-u23".into()),
            solver: AgentId("s-u23".into()),
            reward: MicroCoin::from_micro_units(1),
            parent_state_root: Hash::ZERO,
            epoch: SystemEpoch::new(1),
            timestamp_logical: 1,
            system_signature: SystemSignature::from_bytes([0u8; 64]),
        });
        let err = seq.submit_agent_tx(tx).await.unwrap_err();
        assert!(matches!(err, SubmitError::SystemTxForbiddenOnAgentIngress));
        // submit_id NOT advanced (rejection is pre-queue, before fetch_add).
        assert_eq!(seq.next_submit_id_peek(), pre_submit_id,
            "submit_id must not advance on system-tx ingress rejection");
    }

    /// U24 — submit_agent_tx rejects TaskExpireTx pre-queue.
    #[tokio::test]
    async fn submit_agent_tx_rejects_task_expire_pre_queue() {
        let (_tmp, seq, _rx, _rejection_writer) = fresh_sequencer();
        let pre_submit_id = seq.next_submit_id_peek();
        let tx = TypedTx::TaskExpire(TaskExpireTx {
            tx_id: TxId("et-u24".into()),
            task_id: TaskId("t-u24".into()),
            parent_state_root: Hash::ZERO,
            bounty_refunded: MicroCoin::from_micro_units(1),
            epoch: SystemEpoch::new(1),
            timestamp_logical: 1,
            sponsor_agent: AgentId("sp-u24".into()),                                 // TB-11
            escrow_tx_id: TxId("e-u24".into()),                                      // TB-11
            reason: crate::state::typed_tx::ExpireReason::Deadline,                  // TB-11
            system_signature: SystemSignature::from_bytes([0u8; 64]),
        });
        let err = seq.submit_agent_tx(tx).await.unwrap_err();
        assert!(matches!(err, SubmitError::SystemTxForbiddenOnAgentIngress));
        assert_eq!(seq.next_submit_id_peek(), pre_submit_id);
    }

    /// U25 — submit_agent_tx rejects TerminalSummaryTx pre-queue.
    #[tokio::test]
    async fn submit_agent_tx_rejects_terminal_summary_pre_queue() {
        let (_tmp, seq, _rx, _rejection_writer) = fresh_sequencer();
        let pre_submit_id = seq.next_submit_id_peek();
        let tx = TypedTx::TerminalSummary(TerminalSummaryTx {
            tx_id: TxId("ts-u25".into()),
            task_id: TaskId("t-u25".into()),
            run_id: RunId("r-u25".into()),
            run_outcome: RunOutcome::OmegaAccepted,
            total_attempts: 0,
            failure_class_histogram: BTreeMap::new(),
            last_logical_t: 0,
            parent_state_root: Hash::ZERO,                                           // TB-11
            solver_agent: None,                                                       // TB-11
            evidence_capsule_cid: None,                                               // TB-11
            system_signature: SystemSignature::from_bytes([0u8; 64]),
        });
        let err = seq.submit_agent_tx(tx).await.unwrap_err();
        assert!(matches!(err, SubmitError::SystemTxForbiddenOnAgentIngress));
        assert_eq!(seq.next_submit_id_peek(), pre_submit_id);
    }

    /// U22 — submit_agent_tx rejects ChallengeResolveTx pre-queue
    /// (TB-5 Atom 3 added the variant; previously was DEFERRED in Atom 2).
    /// charter v2 § 4.9 + § 5.3 binding.
    #[tokio::test]
    async fn submit_agent_tx_rejects_challenge_resolve_pre_queue() {
        use crate::state::typed_tx::{ChallengeResolveTx, ChallengeResolution};
        let (_tmp, seq, _rx, _rejection_writer) = fresh_sequencer();
        let pre_submit_id = seq.next_submit_id_peek();
        let tx = TypedTx::ChallengeResolve(ChallengeResolveTx {
            tx_id: TxId("crt-u22".into()),
            parent_state_root: Hash::ZERO,
            target_challenge_tx_id: TxId("ct-target".into()),
            resolution: ChallengeResolution::Released,
            epoch: SystemEpoch::new(1),
            timestamp_logical: 1,
            system_signature: SystemSignature::from_bytes([0u8; 64]),
        });
        let err = seq.submit_agent_tx(tx).await.unwrap_err();
        assert!(matches!(err, SubmitError::SystemTxForbiddenOnAgentIngress),
            "ChallengeResolveTx must reject on agent ingress per TB-5.0 substrate");
        // submit_id NOT advanced (pre-queue rejection per Anti-Oreo guarantee).
        assert_eq!(seq.next_submit_id_peek(), pre_submit_id);
    }

    /// U26 — submit_agent_tx accepts all 6 agent-submitted variants
    /// (Work, Verify, Challenge, Reuse, TaskOpen, EscrowLock) — submit_id
    /// advances; envelope queued.
    #[tokio::test]
    async fn submit_agent_tx_accepts_work_verify_challenge_taskopen_escrowlock_reuse() {
        let (_tmp, seq, _rx, _rejection_writer) = fresh_sequencer();

        // Work (existing fixture).
        let r = seq.submit_agent_tx(TypedTx::Work(fixture_work_tx())).await;
        assert!(r.is_ok(), "Work agent variant accepted; got {r:?}");

        // Verify.
        let r = seq.submit_agent_tx(TypedTx::Verify(VerifyTx {
            tx_id: TxId("vt-u26".into()),
            parent_state_root: Hash::ZERO,
            target_work_tx: TxId("wt-u26".into()),
            verifier_agent: AgentId("v".into()),
            bond: StakeMicroCoin::from_micro_units(1),
            verdict: VerifyVerdict::Confirm,
            signature: AgentSignature::from_bytes([0; 64]),
            timestamp_logical: 1,
        })).await;
        assert!(r.is_ok(), "Verify agent variant accepted; got {r:?}");

        // Challenge.
        let r = seq.submit_agent_tx(TypedTx::Challenge(ChallengeTx {
            tx_id: TxId("ct-u26".into()),
            parent_state_root: Hash::ZERO,
            target_work_tx: TxId("wt-u26".into()),
            challenger_agent: AgentId("c".into()),
            stake: StakeMicroCoin::from_micro_units(1),
            counterexample_cid: Cid([1; 32]),
            signature: AgentSignature::from_bytes([0; 64]),
            timestamp_logical: 1,
        })).await;
        assert!(r.is_ok(), "Challenge agent variant accepted; got {r:?}");

        // Reuse.
        let r = seq.submit_agent_tx(TypedTx::Reuse(ReuseTx {
            tx_id: TxId("rt-u26".into()),
            reusing_work_tx: TxId("wt-u26".into()),
            reused_tool_id: ToolId("tool".into()),
            reused_tool_creator: AgentId("a".into()),
            timestamp_logical: 1,
        })).await;
        assert!(r.is_ok(), "Reuse agent variant accepted; got {r:?}");

        // TaskOpen.
        use crate::state::typed_tx::TaskOpenTx;
        let r = seq.submit_agent_tx(TypedTx::TaskOpen(TaskOpenTx {
            tx_id: TxId("ot-u26".into()),
            task_id: TaskId("t-u26".into()),
            parent_state_root: Hash::ZERO,
            sponsor_agent: AgentId("sponsor".into()),
            verifier_quorum: 1,
            max_reuse_royalty_fraction_basis_points: 1000,
            settlement_rule_hash: Hash::ZERO,
            signature: AgentSignature::from_bytes([0u8; 64]),
            timestamp_logical: 1,
        })).await;
        assert!(r.is_ok(), "TaskOpen agent variant accepted; got {r:?}");

        // EscrowLock.
        use crate::state::typed_tx::EscrowLockTx;
        let r = seq.submit_agent_tx(TypedTx::EscrowLock(EscrowLockTx {
            tx_id: TxId("lt-u26".into()),
            task_id: TaskId("t-u26".into()),
            parent_state_root: Hash::ZERO,
            sponsor_agent: AgentId("sponsor".into()),
            amount: MicroCoin::from_micro_units(1),
            signature: AgentSignature::from_bytes([0u8; 64]),
            timestamp_logical: 1,
        })).await;
        assert!(r.is_ok(), "EscrowLock agent variant accepted; got {r:?}");

        // 6 successful submissions → submit_id advanced 6 times (started at 1).
        assert_eq!(seq.next_submit_id_peek(), 7,
            "next_submit_id should be 1 + 6 successful agent-submissions");
    }

    // ────────────────────────────────────────────────────────────────────────
    // TB-5 Atom 4 — apply_one stage 1.5 unit-tests (preflight § 8.4)
    //
    // U27/U28 + I66/I66.a/b/c: forged signatures on system-emitted variants
    // are rejected with TransitionError::InvalidSystemSignatureLive at
    // apply_one stage 1.5 BEFORE dispatch_transition is invoked. Each rejection
    // writes 1 L4.E row (record_rejection helper, factored out of the
    // dispatch-reject path so both reject paths share semantics). Counter
    // invariants (next_logical_t, state_root_t) MUST NOT advance.
    // ────────────────────────────────────────────────────────────────────────

    use crate::state::typed_tx::ChallengeResolveTx;

    /// Helper: forge a ChallengeResolveTx with all-zero signature.
    fn forged_challenge_resolve() -> TypedTx {
        TypedTx::ChallengeResolve(ChallengeResolveTx {
            tx_id: TxId("crt-stage15-forged".into()),
            parent_state_root: Hash::ZERO,
            target_challenge_tx_id: TxId("ct-target".into()),
            resolution: crate::state::typed_tx::ChallengeResolution::Released,
            epoch: SystemEpoch::new(1),
            timestamp_logical: 1,
            system_signature: SystemSignature::from_bytes([0u8; 64]),
        })
    }

    fn forged_finalize_reward() -> TypedTx {
        TypedTx::FinalizeReward(FinalizeRewardTx {
            tx_id: TxId("ft-stage15-forged".into()),
            claim_id: ClaimId::new("cl-fwd"),
            task_id: TaskId("t-fwd".into()),
            solver: AgentId("solver".into()),
            reward: MicroCoin::from_micro_units(100),
            parent_state_root: Hash::ZERO,
            epoch: SystemEpoch::new(1),
            timestamp_logical: 1,
            system_signature: SystemSignature::from_bytes([0u8; 64]),
        })
    }

    fn forged_task_expire() -> TypedTx {
        TypedTx::TaskExpire(TaskExpireTx {
            tx_id: TxId("et-stage15-forged".into()),
            task_id: TaskId("t-exp".into()),
            parent_state_root: Hash::ZERO,
            bounty_refunded: MicroCoin::from_micro_units(1),
            epoch: SystemEpoch::new(1),
            timestamp_logical: 1,
            sponsor_agent: AgentId("sp-fwd".into()),                                 // TB-11
            escrow_tx_id: TxId("e-fwd".into()),                                      // TB-11
            reason: crate::state::typed_tx::ExpireReason::Deadline,                  // TB-11
            system_signature: SystemSignature::from_bytes([0u8; 64]),
        })
    }

    fn forged_terminal_summary() -> TypedTx {
        TypedTx::TerminalSummary(TerminalSummaryTx {
            tx_id: TxId("ts-stage15-forged".into()),
            task_id: TaskId("t-ts".into()),
            run_id: RunId("r-ts".into()),
            run_outcome: RunOutcome::OmegaAccepted,
            total_attempts: 0,
            failure_class_histogram: BTreeMap::new(),
            last_logical_t: 0,
            parent_state_root: Hash::ZERO,                                           // TB-11
            solver_agent: None,                                                       // TB-11
            evidence_capsule_cid: None,                                               // TB-11
            system_signature: SystemSignature::from_bytes([0u8; 64]),
        })
    }

    /// I66: stage-1.5 rejects forged ChallengeResolve sig + writes L4.E.
    #[test]
    fn stage_1_5_rejects_forged_challenge_resolve_signature() {
        let (_tmp, seq, _rx, rejection_writer) = fresh_sequencer();
        let pre_l4e = rejection_writer.read().expect("read").records().len();
        let pre_logical = seq.next_logical_t_peek();

        let envelope = SubmissionEnvelope { submit_id: 4242, tx: forged_challenge_resolve() };
        let err = seq.apply_one(envelope).expect_err("forged sig must reject");
        match err {
            ApplyError::Transition(TransitionError::InvalidSystemSignatureLive) => {}
            other => panic!("expected InvalidSystemSignatureLive, got {other:?}"),
        }

        let post_l4e = rejection_writer.read().expect("read").records().len();
        assert_eq!(post_l4e, pre_l4e + 1, "stage 1.5 reject writes 1 L4.E row");
        assert_eq!(seq.next_logical_t_peek(), pre_logical,
            "K1: stage 1.5 reject MUST NOT advance logical_t");
    }

    /// I66.a: stage-1.5 rejects forged FinalizeReward sig.
    #[test]
    fn stage_1_5_rejects_forged_finalize_reward_signature() {
        let (_tmp, seq, _rx, rejection_writer) = fresh_sequencer();
        let pre_l4e = rejection_writer.read().expect("read").records().len();
        let envelope = SubmissionEnvelope { submit_id: 4243, tx: forged_finalize_reward() };
        let err = seq.apply_one(envelope).expect_err("forged sig must reject");
        match err {
            ApplyError::Transition(TransitionError::InvalidSystemSignatureLive) => {}
            other => panic!("expected InvalidSystemSignatureLive, got {other:?}"),
        }
        let post_l4e = rejection_writer.read().expect("read").records().len();
        assert_eq!(post_l4e, pre_l4e + 1);
    }

    /// I66.b: stage-1.5 rejects forged TaskExpire sig.
    #[test]
    fn stage_1_5_rejects_forged_task_expire_signature() {
        let (_tmp, seq, _rx, rejection_writer) = fresh_sequencer();
        let pre_l4e = rejection_writer.read().expect("read").records().len();
        let envelope = SubmissionEnvelope { submit_id: 4244, tx: forged_task_expire() };
        let err = seq.apply_one(envelope).expect_err("forged sig must reject");
        match err {
            ApplyError::Transition(TransitionError::InvalidSystemSignatureLive) => {}
            other => panic!("expected InvalidSystemSignatureLive, got {other:?}"),
        }
        let post_l4e = rejection_writer.read().expect("read").records().len();
        assert_eq!(post_l4e, pre_l4e + 1);
    }

    /// I66.c: stage-1.5 rejects forged TerminalSummary sig.
    #[test]
    fn stage_1_5_rejects_forged_terminal_summary_signature() {
        let (_tmp, seq, _rx, rejection_writer) = fresh_sequencer();
        let pre_l4e = rejection_writer.read().expect("read").records().len();
        let envelope = SubmissionEnvelope { submit_id: 4245, tx: forged_terminal_summary() };
        let err = seq.apply_one(envelope).expect_err("forged sig must reject");
        match err {
            ApplyError::Transition(TransitionError::InvalidSystemSignatureLive) => {}
            other => panic!("expected InvalidSystemSignatureLive, got {other:?}"),
        }
        let post_l4e = rejection_writer.read().expect("read").records().len();
        assert_eq!(post_l4e, pre_l4e + 1);
    }

    /// U27: emit_system_tx round-trip — emitted ChallengeResolve survives
    /// apply_one stage 1.5 verification (constructive correctness; pinned
    /// pubkey matches the runtime keypair's pubkey under epoch). Atom 5
    /// updated this expectation: dispatch is now real, so a target that
    /// does not exist in challenge_cases_t surfaces as ChallengeNotFound
    /// (NOT NotYetImplemented). The contract the test enforces is
    /// "stage 1.5 must NOT reject self-signed emit txns" — we assert the
    /// observed error is downstream of stage 1.5 (any Transition variant
    /// other than InvalidSystemSignatureLive).
    #[tokio::test]
    async fn stage_1_5_accepts_emit_system_tx_self_signed_challenge_resolve() {
        let (_tmp, seq, mut rx, _rejection) = fresh_sequencer();

        let _receipt = seq
            .emit_system_tx(SystemEmitCommand::ChallengeResolve {
                target_challenge_tx_id: TxId("ct-u27".into()),
                resolution: crate::state::typed_tx::ChallengeResolution::Released,
            })
            .await
            .expect("emit ok");

        let envelope = rx.try_recv().expect("envelope queued");
        let err = seq.apply_one(envelope).expect_err("target absent → reject");
        match err {
            ApplyError::Transition(TransitionError::InvalidSystemSignatureLive) => {
                panic!("Self-signed emit_system_tx MUST PASS stage 1.5 verification");
            }
            ApplyError::Transition(TransitionError::ChallengeNotFound) => {
                // Expected post-Atom-5: stage 1.5 passed; dispatch's real
                // arm rejected with ChallengeNotFound (target_challenge_tx_id
                // not present in challenge_cases_t — no fixture seeded).
            }
            other => {
                eprintln!("non-ChallengeNotFound observed: {other:?} — stage 1.5 still passed");
            }
        }
    }

    /// U28: stage 1.5 path is bypassed for agent variants — no spurious
    /// "missing system_signature" errors when an agent variant is applied.
    #[test]
    fn stage_1_5_skipped_for_agent_variants() {
        // Build a WorkTx fixture and submit through apply_one directly.
        // We don't care that dispatch_transition succeeds — we only assert
        // we don't observe InvalidSystemSignatureLive (which would be a
        // false positive at the verifier).
        let (_tmp, seq, _rx, _rejection) = fresh_sequencer();
        let work = TypedTx::Work(super::tests::fixture_work_tx());
        let envelope = SubmissionEnvelope { submit_id: 1, tx: work };
        let result = seq.apply_one(envelope);
        match result {
            Err(ApplyError::Transition(TransitionError::InvalidSystemSignatureLive)) => {
                panic!("stage 1.5 must NOT trip on agent variants");
            }
            _ => {}
        }
    }

    // ────────────────────────────────────────────────────────────────────────
    // TB-5 Atom 5+6 — ChallengeResolve dispatch unit-tests (preflight § 8.2)
    //
    // U29-U33: dispatch_transition direct invocation; isolates the dispatch
    // arm body from the apply_one + queue + signature pipeline.
    // ────────────────────────────────────────────────────────────────────────

    use crate::state::q_state::ChallengeStatus;
    use crate::state::typed_tx::ChallengeResolution;

    /// Seed Q with a single Open ChallengeCase so dispatch can resolve it.
    /// Returns (q, target_challenge_tx_id, challenger_id, bond_amount).
    fn seed_q_with_open_challenge_case(
        challenger: &str,
        challenger_starting_balance_micro: i64,
        challenge_tx_id: &str,
        bond_micro: i64,
        target_work_tx_id: &str,
    ) -> (QState, TxId, AgentId, MicroCoin) {
        let mut q = QState::genesis();
        let challenger_id = AgentId(challenger.into());
        if challenger_starting_balance_micro > 0 {
            q.economic_state_t.balances_t.0.insert(
                challenger_id.clone(),
                MicroCoin::from_micro_units(challenger_starting_balance_micro),
            );
        }
        let challenge_id = TxId(challenge_tx_id.into());
        let bond = MicroCoin::from_micro_units(bond_micro);
        q.economic_state_t.challenge_cases_t.0.insert(
            challenge_id.clone(),
            crate::state::q_state::ChallengeCase {
                challenger: challenger_id.clone(),
                bond,
                opened_at_round: 7,
                target_work_tx: TxId(target_work_tx_id.into()),
                status: ChallengeStatus::Open,
            },
        );
        (q, challenge_id, challenger_id, bond)
    }

    fn make_resolve_tx(
        target: &TxId,
        resolution: ChallengeResolution,
        parent_root: Hash,
    ) -> TypedTx {
        TypedTx::ChallengeResolve(crate::state::typed_tx::ChallengeResolveTx {
            tx_id: TxId(format!("crt-disp-{}", target.0)),
            parent_state_root: parent_root,
            target_challenge_tx_id: target.clone(),
            resolution,
            epoch: SystemEpoch::new(1),
            timestamp_logical: 1,
            system_signature: SystemSignature::from_bytes([0u8; 64]),
        })
    }

    /// U29 + U30: dispatch_challenge_resolve_released_zeros_bond_and_sets_status
    /// + dispatch_challenge_resolve_released_refunds_balance.
    #[test]
    fn dispatch_challenge_resolve_released_zeros_bond_refunds_balance_and_sets_status() {
        let preds = PredicateRegistry::new();
        let tools = ToolRegistry::new();
        // Pre: challenger had 100 micro pre-challenge; on challenge accept
        // (in TB-4) bond was already debited from balances_t and credited
        // to challenge_cases_t. Here we model the post-challenge state:
        // challenger balance = 100 - 4 = 96; case.bond = 4.
        let (q, target_id, challenger, bond) = seed_q_with_open_challenge_case(
            "challenger-u29", 96, "ct-u29", 4, "wt-u29");
        let pre_balance = q
            .economic_state_t
            .balances_t
            .0
            .get(&challenger)
            .copied()
            .unwrap();

        let tx = make_resolve_tx(&target_id, ChallengeResolution::Released, q.state_root_t);
        let (q_next, _) = dispatch_transition(&q, &tx, &preds, &tools)
            .expect("Released path with valid Open case must accept");

        // Refund: challenger balance += bond.
        let post_balance = q_next
            .economic_state_t
            .balances_t
            .0
            .get(&challenger)
            .copied()
            .unwrap();
        assert_eq!(
            post_balance.micro_units(),
            pre_balance.micro_units() + bond.micro_units(),
            "Released must refund bond to challenger"
        );

        // Bond zeroed; status flipped; entry preserved (audit trail).
        let entry = q_next
            .economic_state_t
            .challenge_cases_t
            .0
            .get(&target_id)
            .expect("entry preserved per directive § 7 Q6");
        assert_eq!(entry.bond.micro_units(), 0, "bond must be zeroed on Released");
        assert_eq!(entry.status, ChallengeStatus::Released, "status flipped");
        assert_eq!(entry.challenger, challenger, "challenger preserved");
        assert_eq!(entry.target_work_tx, TxId("wt-u29".into()), "target preserved");

        // state_root advanced via CHALLENGE_RESOLVE_DOMAIN_V1.
        let expected = challenge_resolve_accept_state_root(&q.state_root_t, &tx);
        assert_eq!(q_next.state_root_t, expected,
            "state_root must match CHALLENGE_RESOLVE_DOMAIN_V1 hash");
    }

    /// U31: AlreadyResolved gate — second resolve with same target rejects.
    #[test]
    fn dispatch_challenge_resolve_released_cannot_run_twice() {
        let preds = PredicateRegistry::new();
        let tools = ToolRegistry::new();
        let (q, target_id, _challenger, _bond) = seed_q_with_open_challenge_case(
            "challenger-u31", 96, "ct-u31", 4, "wt-u31");

        // First resolve succeeds.
        let tx1 = make_resolve_tx(&target_id, ChallengeResolution::Released, q.state_root_t);
        let (q1, _) = dispatch_transition(&q, &tx1, &preds, &tools)
            .expect("first Released accepts");

        // Second resolve on the same case (now status=Released) MUST reject
        // with AlreadyResolved.
        let tx2 = make_resolve_tx(&target_id, ChallengeResolution::Released, q1.state_root_t);
        let err = dispatch_transition(&q1, &tx2, &preds, &tools)
            .expect_err("second resolve must reject");
        match err {
            TransitionError::AlreadyResolved => {}
            other => panic!("expected AlreadyResolved, got {other:?}"),
        }
    }

    /// U32: ChallengeNotFound — target not in challenge_cases_t.
    #[test]
    fn dispatch_challenge_resolve_unknown_target_rejects() {
        let preds = PredicateRegistry::new();
        let tools = ToolRegistry::new();
        let q = QState::genesis(); // empty challenge_cases_t.
        let tx = make_resolve_tx(
            &TxId("ct-u32-nonexistent".into()),
            ChallengeResolution::Released,
            q.state_root_t,
        );
        let err = dispatch_transition(&q, &tx, &preds, &tools)
            .expect_err("unknown target must reject");
        match err {
            TransitionError::ChallengeNotFound => {}
            other => panic!("expected ChallengeNotFound, got {other:?}"),
        }
    }

    /// U33: UpheldDeferred — marker only; bond preserved.
    #[test]
    fn dispatch_challenge_resolve_upheld_deferred_marker_only() {
        let preds = PredicateRegistry::new();
        let tools = ToolRegistry::new();
        let (q, target_id, challenger, bond) = seed_q_with_open_challenge_case(
            "challenger-u33", 96, "ct-u33", 4, "wt-u33");
        let pre_balance = q
            .economic_state_t
            .balances_t
            .0
            .get(&challenger)
            .copied()
            .unwrap();

        let tx = make_resolve_tx(&target_id, ChallengeResolution::UpheldDeferred, q.state_root_t);
        let (q_next, _) = dispatch_transition(&q, &tx, &preds, &tools)
            .expect("UpheldDeferred path with valid Open case must accept");

        // No balance mutation.
        let post_balance = q_next
            .economic_state_t
            .balances_t
            .0
            .get(&challenger)
            .copied()
            .unwrap();
        assert_eq!(
            post_balance.micro_units(),
            pre_balance.micro_units(),
            "UpheldDeferred must NOT mutate challenger balance"
        );

        // Bond preserved; status flipped to UpheldDeferred.
        let entry = q_next
            .economic_state_t
            .challenge_cases_t
            .0
            .get(&target_id)
            .expect("entry preserved");
        assert_eq!(
            entry.bond.micro_units(),
            bond.micro_units(),
            "UpheldDeferred MUST preserve bond (TB-6 RSP-3.2 slash routing target)"
        );
        assert_eq!(entry.status, ChallengeStatus::UpheldDeferred);

        // state_root advanced via CHALLENGE_RESOLVE_DOMAIN_V1.
        let expected = challenge_resolve_accept_state_root(&q.state_root_t, &tx);
        assert_eq!(q_next.state_root_t, expected);
    }

    /// U34: StaleParent — parent_state_root mismatch.
    #[test]
    fn dispatch_challenge_resolve_rejects_stale_parent() {
        let preds = PredicateRegistry::new();
        let tools = ToolRegistry::new();
        let (q, target_id, _challenger, _bond) = seed_q_with_open_challenge_case(
            "challenger-u34", 96, "ct-u34", 4, "wt-u34");
        // Forge a wrong parent root.
        let stale_root = Hash::from_bytes([0xde; 32]);
        let tx = make_resolve_tx(&target_id, ChallengeResolution::Released, stale_root);
        let err = dispatch_transition(&q, &tx, &preds, &tools)
            .expect_err("stale parent must reject");
        match err {
            TransitionError::StaleParent => {}
            other => panic!("expected StaleParent, got {other:?}"),
        }
    }
}
