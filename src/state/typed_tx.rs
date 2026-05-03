//! Typed transaction ABI surface — CO1.1.4-pre1.
//!
//! Spec authority:
//! - `handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md` — this atom
//! - `STATE_TRANSITION_SPEC_v1_2026-04-27.md` § 1 (typed schemas), § 2.5
//!   (canonical serialization), § 3 (transition pseudocode used to derive
//!   FinalizeRewardTx schema in spec § 4)
//!
//! Why this module exists: when CO1.7-impl A1 (Git2LedgerWriter) shipped, the
//! downstream A2 (Sequencer + `dispatch_transition`) needed a `TypedTx` enum
//! whose variants carry per-kind tx structs. Those structs and ~20 supporting
//! types (identifiers, signatures, predicate-result types, status enums) were
//! "frozen on paper" in STATE_TRANSITION_SPEC § 1 but had no Rust definition.
//! CO1.1.4-pre1 lands them in isolation under its own dual-audit gate,
//! per the project's per-atom audit principle (CLAUDE.md "Audit Standard").
//!
//! /// TRACE_MATRIX FC2-Submit + § 1 typed schemas: typed-tx ABI surface.

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

use sha2::{Digest, Sha256};

use crate::bottom_white::cas::schema::Cid;
use crate::bottom_white::ledger::system_keypair::{serde_bytes_64, SystemEpoch, SystemSignature};
use crate::economy::money::{MicroCoin, StakeMicroCoin};
use crate::state::q_state::{AgentId, Hash, TaskId, TxId};

// ────────────────────────────────────────────────────────────────────────────
// § 2 Identifier newtypes (all opaque strings to Q_t)
// ────────────────────────────────────────────────────────────────────────────

// `TaskId` previously lived here; moved to `state::q_state` in TB-3 (2026-04-30)
// to eliminate the q_state↔typed_tx circular-dependency that would have arisen
// when q_state needs `TaskId` as the `TaskMarketsIndex` key. See q_state.rs.

/// TRACE_MATRIX § 1.5 — runtime run id (one run per `Sequencer` driver lifecycle).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct RunId(pub String);

/// TRACE_MATRIX STATE § 3.4 + § 4 I-FINALIZE-BATCH-ORDER — typed claim id used
/// in `FinalizeRewardTx.claim_id` and `ClaimsIndex` keying. Wraps `TxId`
/// (the underlying claim is recorded against the work_tx's TxId in
/// ClaimsIndex per current QState shape) but **prevents accidental mixing
/// of claim references with arbitrary transaction references** at the type
/// level (Codex round-1 Q-B CHALLENGE).
///
/// `#[serde(transparent)]` — wire-identical to TxId, so adoption is
/// non-breaking for canonical encoding.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
#[serde(transparent)]
pub struct ClaimId(pub TxId);

impl ClaimId {
    pub fn new(s: impl Into<String>) -> Self {
        Self(TxId(s.into()))
    }
    pub fn as_tx_id(&self) -> &TxId {
        &self.0
    }
}

/// TRACE_MATRIX § 1.3 ReuseTx + L2 Tool Registry — opaque tool identifier.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct ToolId(pub String);

/// TRACE_MATRIX § 1.2 PredicateResultsBundle + L1 Predicate Registry — opaque predicate id.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct PredicateId(pub String);

/// TRACE_MATRIX § 1.2 WorkTx field 5 — read-set key (DAG attribution / replay).
/// Kept as opaque string in v1; stricter typing (path / tape-coordinate) lands
/// in CO P2.4.0 attribution-engine spike.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct ReadKey(pub String);

/// TRACE_MATRIX § 1.2 WorkTx field 6 — write-set key (DAG attribution / replay).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct WriteKey(pub String);

// ────────────────────────────────────────────────────────────────────────────
// § 3 AgentSignature (Ed25519 [u8;64], type-distinct from SystemSignature)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX § 1.2 WorkTx field 10 + I-SIG: agent-side detached Ed25519
/// signature over the per-tx canonical_digest. Distinct type from
/// `SystemSignature` to prevent accidental confusion at API boundaries
/// (Codex sec-arg: agent-vs-system signature mixing is a real hazard).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentSignature(#[serde(with = "serde_bytes_64")] [u8; 64]);

impl AgentSignature {
    pub const fn from_bytes(bytes: [u8; 64]) -> Self {
        Self(bytes)
    }
    pub const fn as_bytes(&self) -> &[u8; 64] {
        &self.0
    }
}

impl Default for AgentSignature {
    fn default() -> Self {
        Self([0u8; 64])
    }
}

// ────────────────────────────────────────────────────────────────────────────
// § 3 SlashEvidenceCid (newtype; type-distinct slash-evidence reference)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX § 1.2 TxStatus::FinalizedSlash — typed reference to the
/// counter-example payload that justified the slash (lives in L3 CAS).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
#[serde(transparent)]
pub struct SlashEvidenceCid(pub Cid);

// ────────────────────────────────────────────────────────────────────────────
// § 4 Predicate result types
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX § 1.2 PredicateResultsBundle — boolean predicate verdict
/// optionally accompanied by an L3 CAS reference to the proof object
/// (e.g. Lean witness, ZK proof bytes).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct BoolWithProof {
    pub value: bool,
    pub proof_cid: Option<Cid>,
}

/// TRACE_MATRIX § 1.2 PredicateResultsBundle — safety-class discriminator.
/// Determines fail-closed (Safety) vs fail-open-with-signal (Creation) behavior
/// when a predicate's evaluation errors. Frozen STATE spec § 1.2.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum SafetyOrCreation {
    Safety = 0,
    Creation = 1,
}

impl Default for SafetyOrCreation {
    fn default() -> Self {
        // Safety bias by default: fail-closed if no class declared.
        Self::Safety
    }
}

/// TRACE_MATRIX § 1.2 WorkTx field 8 — runner-stamped predicate results
/// (acceptance + settlement gates) with explicit safety-class discriminator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PredicateResultsBundle {
    pub acceptance: BTreeMap<PredicateId, BoolWithProof>,
    pub settlement: BTreeMap<PredicateId, BoolWithProof>,
    pub safety_class: SafetyOrCreation,
}

// ────────────────────────────────────────────────────────────────────────────
// § 5 Status / class enums (RejectionClass, VerifyVerdict, RunOutcome,
//                          and the runtime-only TxStatus per D-1)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX § 1.4 — classification of a rejected attempt.
/// Public predicates are classified concretely; private predicates surface as
/// `Opaque` (no information leakage to attacker).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum RejectionClass {
    AcceptancePredicateFail(PredicateId),
    SettlementPredicateFail(PredicateId),
    StakeInsufficient,
    SignatureInvalid,
    StaleParentRoot,
    Opaque,
    BudgetExceeded,
}

/// TRACE_MATRIX § 1.3 VerifyTx field 5 — verifier verdict.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum VerifyVerdict {
    Confirm = 0,
    Doubt = 1,
}

/// TRACE_MATRIX § 1.5 TerminalSummaryTx field 4 + Art. IV halt-reason taxonomy.
/// Five-way partition over how a run terminates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum RunOutcome {
    OmegaAccepted = 0,
    MaxTxExhausted = 1,
    WallClockCap = 2,
    ComputeCap = 3,
    ErrorHalt = 4,
}

/// TRACE_MATRIX § 1.2 TxStatus — **runtime book-keeping only** (D-1 divergence
/// from STATE spec): never serialized into a TypedTx variant's wire bytes.
/// Tracked in `q_t.q_t.agents[id].last_accepted_tx` + `ClaimsIndex`. Exposed
/// here as a public type for the runtime API surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TxStatus {
    Pending,
    Accepted,
    Rejected(RejectionClass),
    FinalizedReward(MicroCoin),
    FinalizedSlash(SlashEvidenceCid),
}

// ────────────────────────────────────────────────────────────────────────────
// § 5 (cont'd) — Typed tx structs (STATE spec § 1.2-1.6 + § 3.6)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX § 1.2 — agent-submitted work transaction (12-field schema;
/// **D-1 divergence**: field 12 `status: TxStatus` is excluded from canonical
/// wire bytes — TxStatus is runner book-keeping per CO1.1.4-pre1 spec § 5).
///
/// This is the per-tx struct that the CO1.7 sequencer hands to
/// `step_transition` (CO1.7.5 body atom). The `signature` is over
/// `WorkSigningPayload::canonical_digest()` — i.e. the projection produced by
/// `WorkTx::to_signing_payload()` (excludes the signature field itself; per
/// v1.1 P1 the digest pre-image carries the `b"turingosv4.agent_sig.work.v1"`
/// domain prefix).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct WorkTx {
    pub tx_id: TxId,                                  //  1
    pub task_id: TaskId,                              //  2
    pub parent_state_root: Hash,                      //  3
    pub agent_id: AgentId,                            //  4
    pub read_set: BTreeSet<ReadKey>,                  //  5
    pub write_set: BTreeSet<WriteKey>,                //  6
    pub proposal_cid: Cid,                            //  7
    pub predicate_results: PredicateResultsBundle,    //  8 (runner-stamped)
    pub stake: StakeMicroCoin,                        //  9
    pub signature: AgentSignature,                    // 10
    pub timestamp_logical: u64,                       // 11
    // 12: TxStatus — D-1 elision; runtime-only.
}

/// TRACE_MATRIX § 1.3 — verifier verdict transaction.
///
/// **TB-4 (2026-04-30) schema bump**: `parent_state_root: Hash` added as
/// field #2 (per TB-4 charter § 4.1 + directive Q2). Constitutional shape
/// — every accepted-tx variant must carry an explicit parent_state_root
/// for the StaleParent gate. Pre-TB-4 has no production-accepted VerifyTx
/// rows (dispatch arm was `NotYetImplemented`), so the wire bump is harmless.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct VerifyTx {
    pub tx_id: TxId,                       //  1
    pub parent_state_root: Hash,           //  2  (TB-4 NEW)
    pub target_work_tx: TxId,              //  3
    pub verifier_agent: AgentId,           //  4
    pub bond: StakeMicroCoin,              //  5
    pub verdict: VerifyVerdict,            //  6
    pub signature: AgentSignature,         //  7
    pub timestamp_logical: u64,            //  8
}

impl Default for VerifyVerdict {
    fn default() -> Self {
        Self::Confirm
    }
}

/// TRACE_MATRIX § 1.3 — challenge transaction (counter-example posted with
/// stake at risk).
///
/// **TB-4 (2026-04-30) schema bump**: `parent_state_root: Hash` added as
/// field #2 (per TB-4 charter § 4.1 + directive Q2). Same justification as
/// VerifyTx (constitutional shape; pre-TB-4 has no production-accepted rows).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ChallengeTx {
    pub tx_id: TxId,                       //  1
    pub parent_state_root: Hash,           //  2  (TB-4 NEW)
    pub target_work_tx: TxId,              //  3
    pub challenger_agent: AgentId,         //  4
    pub stake: StakeMicroCoin,             //  5
    pub counterexample_cid: Cid,           //  6
    pub signature: AgentSignature,         //  7
    pub timestamp_logical: u64,            //  8
}

/// TRACE_MATRIX § 1.3 — fact-tx recording reuse of a tool created by a prior
/// agent (royalty graph edge). No submitting agent (per § 3.6.5 v1.3).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ReuseTx {
    pub tx_id: TxId,                       //  1
    pub reusing_work_tx: TxId,             //  2
    pub reused_tool_id: ToolId,            //  3
    pub reused_tool_creator: AgentId,      //  4
    pub timestamp_logical: u64,            //  5
}

/// TRACE_MATRIX CO1.1.4-pre1 spec § 4 — derived schema (STATE spec § 3.4
/// uses opaque `FinalizeTx::from(claim_id, reward)` constructor without an
/// explicit struct definition).
///
/// **v1.1 round-1 audit closures**:
/// - **C-3 (Codex Q-B)**: `claim_id` is now a typed `ClaimId` newtype (was
///   bare `TxId`) — STATE § 4 I-FINALIZE-BATCH-ORDER speaks in claim_id;
///   reusing TxId leaked QState implementation into the wire format.
/// - **C-3 (Codex Q-B)**: `task_id` / `solver` / `reward` are documented as
///   **Q-DERIVED at replay** — replay (CO1.7-impl A4) re-fetches them from
///   ClaimsIndex by `claim_id`, NOT trusted from wire. Wire fields are kept
///   as a ledger summary (so a human reading L4 can see the finalize event
///   semantics) but the AUTHORITATIVE values come from Q_t.
/// - **C-3 / GM-2 followup**: `system_signature` is RETAINED for v1.1 — it
///   binds the system-emitted FinalizeRewardTx to a specific runtime keypair
///   epoch (auditability + cross-cell trust). The CO1.7 `LedgerEntry`
///   wraps this struct via CAS reference + signs the `LedgerEntrySigningPayload`
///   digest; the two sigs are NOT redundant: this one binds the tx-payload
///   bytes; the L4 envelope sig binds the (logical_t, parent_ledger_root, tx_payload_cid)
///   sequencer-stamped envelope. v1.1 spec § 4 makes the dual-sign rationale explicit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct FinalizeRewardTx {
    pub tx_id: TxId,                       //  1
    pub claim_id: ClaimId,                 //  2 — typed (was TxId in v1)
    pub task_id: TaskId,                   //  3 — Q-derived authoritative; wire = ledger summary
    pub solver: AgentId,                   //  4 — Q-derived authoritative; wire = ledger summary
    pub reward: MicroCoin,                 //  5 — Q-derived authoritative (SettlementEngine output); wire = ledger summary
    pub parent_state_root: Hash,           //  6
    pub epoch: SystemEpoch,                //  7
    pub timestamp_logical: u64,            //  8
    pub system_signature: SystemSignature, //  9 — see doc-comment on dual-sign rationale
}

/// TRACE_MATRIX STATE spec § 3.6 v1.3 — system-emitted task-expiry tx
/// (refunds bounty + locked stakes when no claim finalized by deadline).
/// TRACE_MATRIX FC1-N1: TB-11 (2026-05-02 architect ruling §6.2 Epistemic
/// Exhaust & Capital Liberation) — additive bump of the wire schema to
/// carry the architect-mandated `sponsor_agent` + `escrow_tx_id` +
/// `reason` fields needed by the dispatch arm to enact the refund.
///
/// **System-emitted only**: agent ingress (`Sequencer::submit_agent_tx`)
/// rejects this variant pre-queue per Anti-Oreo (Art V.1.3); construction
/// goes through `Sequencer::emit_system_tx`.
///
/// **TB-11 additive bump** (no production rows pre-TB-11; dispatch arm was
/// `NotYetImplemented`; safe per `feedback_no_retroactive_evidence_rewrite`):
/// adds `sponsor_agent` (depositor of the escrow being refunded) +
/// `escrow_tx_id` (which `escrows_t` row to refund) + `reason` (taxonomy
/// discriminator). Field 8/9/10 inserted **before** `system_signature` so
/// the signing payload sees them in canonical position; this rotates the
/// golden digest fixtures (TB-11 charter §6 G9 + golden-digest rotation
/// protocol documented in this file's tests module).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TaskExpireTx {
    pub tx_id: TxId,                       //  1
    pub task_id: TaskId,                   //  2
    pub parent_state_root: Hash,           //  3
    pub bounty_refunded: MicroCoin,        //  4 (computed by runtime; included for ledger summary; equals escrows_t[escrow_tx_id].amount at emit time)
    pub epoch: SystemEpoch,                //  5
    pub timestamp_logical: u64,            //  6
    /// TB-11 NEW: depositor of `escrows_t[escrow_tx_id]`. Q-derived at
    /// `emit_system_tx` time; wire field is ledger summary (Q is authoritative).
    pub sponsor_agent: AgentId,            //  7  TB-11 NEW
    /// TB-11 NEW: which `escrows_t` row to refund. Required because a single
    /// task may have multiple `EscrowLockTx`s contributing to its
    /// `task_markets_t.total_escrow`; the refund pathway must be per-escrow
    /// (1 TaskExpireTx per escrow) to preserve replay-deterministic
    /// CTF accounting.
    pub escrow_tx_id: TxId,                //  8  TB-11 NEW
    /// TB-11 NEW: discriminator over the policy that triggered expiry.
    pub reason: ExpireReason,              //  9  TB-11 NEW
    pub system_signature: SystemSignature, // 10  (was field 7 pre-TB-11)
}

/// TRACE_MATRIX STATE spec § 1.5 — system-emitted no-accept-run handler.
/// TRACE_MATRIX FC1-N1: TB-11 (2026-05-02 architect ruling §6.2): this
/// struct serves as the canonical anchor for the architect's
/// `RunExhaustedTx` role in the failure path (≡ semantically equivalent;
/// see `pub type RunExhaustedTx = TerminalSummaryTx` alias below).
///
/// Emitted exactly once if a run terminates without any accepted work_tx, so
/// L6 reconstructibility (failure-class signal) is preserved on the tape
/// even when no work_tx ever passed. **TB-11 architect-vocabulary alias**:
/// the architect's `RunExhaustedTx` (per
/// `handover/directives/2026-05-02_TB11_EPISTEMIC_EXHAUST_ARCHITECT_RULING.md` §6.2)
/// is **semantically equivalent to this struct** — both anchor a run-level
/// outcome on L4 with a system_signature.
///
/// **v1.1 round-1 audit closure (C-3 Codex Q-C must-fix-now)**: replaces the
/// 3-field placeholder previously living in `system_keypair.rs`. Full
/// 8-field schema per STATE § 1.5.
///
/// **TB-11 additive bump** (no production rows pre-TB-11; dispatch arm was
/// `NotYetImplemented`; safe per `feedback_no_retroactive_evidence_rewrite`):
/// adds `parent_state_root` (constitutional shape, mirrors VerifyTx /
/// ChallengeTx TB-4 bumps) + `solver_agent: Option<AgentId>` (which agent
/// owned the failed run, if any) + `evidence_capsule_cid: Option<Cid>`
/// (architect §6.2 — references the `EvidenceCapsule` CAS bytes for O(N)
/// auditability with O(1) chain cost). `None` for OmegaAccepted; `Some` for
/// failure outcomes (MaxTxExhausted / WallClockCap / ComputeCap / ErrorHalt).
/// Fields inserted **before** `system_signature` so the signing payload sees
/// them in canonical position; this rotates the golden digest fixtures.
///
/// The signer (`system_keypair`) signs an opaque `TerminalSummarySigning([u8; 32])`
/// digest — same opaque-digest pattern as `LedgerEntrySigning`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TerminalSummaryTx {
    pub tx_id: TxId,                                          //  1
    pub task_id: TaskId,                                      //  2
    pub run_id: RunId,                                        //  3
    pub run_outcome: RunOutcome,                              //  4
    pub total_attempts: u32,                                  //  5
    pub failure_class_histogram: BTreeMap<RejectionClass, u32>,// 6
    pub last_logical_t: u64,                                  //  7
    /// TB-11 NEW: constitutional StaleParent gate (mirrors VerifyTx + ChallengeTx
    /// TB-4 schema bump; every accepted-tx variant carries explicit
    /// parent_state_root).
    pub parent_state_root: Hash,                              //  8 TB-11 NEW
    /// TB-11 NEW: which agent owned the run (None if no solver was assigned
    /// before the run terminated, e.g. evaluator never picked up the task).
    pub solver_agent: Option<AgentId>,                        //  9 TB-11 NEW
    /// TB-11 NEW: architect §6.2 — references the EvidenceCapsule CAS bytes.
    /// `None` for `RunOutcome::OmegaAccepted` (success path needs no failure
    /// evidence). `Some` for the 4 failure outcomes (MaxTxExhausted /
    /// WallClockCap / ComputeCap / ErrorHalt).
    pub evidence_capsule_cid: Option<Cid>,                    // 10 TB-11 NEW
    pub system_signature: SystemSignature,                    // 11 (was field 8 pre-TB-11)
}

/// TRACE_MATRIX FC1-N1: TB-11 (2026-05-02 architect ruling §6.2) —
/// architect-vocabulary alias for `TerminalSummaryTx` in the failure path.
/// The struct itself is unchanged in identity; this alias makes the
/// architect's naming visible at API boundaries without rotating the wire
/// format. Use `RunExhaustedTx` in new code that emphasizes the
/// failure-anchor role; `TerminalSummaryTx` remains the primary schema
/// name for backward-compatibility with pre-TB-11 references.
pub type RunExhaustedTx = TerminalSummaryTx;

/// TRACE_MATRIX TB-11 (2026-05-02 architect ruling §6.2) —
/// system-emitted task-level failure marker. **NEW in TB-11**.
///
/// Anchors a "death certificate" on L4 for a task that has accumulated
/// enough failed runs (or other architect-policy triggers) to be considered
/// non-resolvable. Future TB-12 NodeMarket Short / NO settlement uses this
/// as the canonical resolution anchor: a NO position references a
/// TaskBankruptcyTx as its on-chain death proof.
///
/// **System-emitted only**: agent ingress rejects pre-queue with
/// `SubmitError::SystemTxForbiddenOnAgentIngress`; construction goes through
/// `Sequencer::emit_system_tx`.
///
/// **No money movement**: TaskBankruptcyTx records a state mutation on
/// `task_markets_t[task_id].state = Bankrupt` but does NOT debit/credit
/// any balance. Refund (if any) is a separate TaskExpireTx fired
/// post-bankruptcy by the runtime tick.
///
/// **Constitutional preservation**: `evidence_capsule_cid` carries the
/// rolled-up failure evidence (architect §7.1 — O(1) chain cost, O(N)
/// auditability). `bankruptcy_reason` discriminates the policy that
/// triggered bankruptcy (max failed run count, deadline exceeded, etc.).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TaskBankruptcyTx {
    pub tx_id: TxId,                       //  1
    pub parent_state_root: Hash,           //  2
    pub task_id: TaskId,                   //  3
    /// Architect §6.2: rollup CAS object referencing all per-run capsules
    /// for this task (or a single dominant capsule if only one failed run
    /// triggered the policy threshold).
    pub evidence_capsule_cid: Cid,         //  4
    pub bankruptcy_reason: BankruptcyReason, //  5
    /// Number of failed runs observed at bankruptcy time (anti-frivolous
    /// emission; emit_system_tx checks this against
    /// `TASK_BANKRUPTCY_FAILED_RUN_COUNT_MIN`).
    pub failed_run_count: u32,             //  6
    pub epoch: SystemEpoch,                //  7
    pub timestamp_logical: u64,            //  8
    pub system_signature: SystemSignature, //  9
}

/// TRACE_MATRIX FC1-N1: TB-11 (architect §6.2) — taxonomy of why a
/// `TaskExpireTx` was emitted. Discriminator on the policy that
/// triggered expiry (deadline / max-run-count / sponsor-request /
/// post-bankruptcy refund).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum ExpireReason {
    /// Task open-time + TASK_EXPIRY_LOGICAL_T_DELTA exceeded without Finalized claim.
    Deadline = 0,
    /// Task accumulated >= MAX_RUN_COUNT_BEFORE_REFUND failed runs.
    MaxRunCountReached = 1,
    /// Sponsor explicitly requested expiry (privileged operator path; defer to TB-12+).
    ManualSponsorRequest = 2,
    /// Task was already TaskBankruptcy-marked; expiry is the post-bankruptcy
    /// refund step.
    BankruptcyTriggered = 3,
}

impl Default for ExpireReason {
    fn default() -> Self {
        Self::Deadline
    }
}

/// TRACE_MATRIX FC1-N1: TB-11 (architect §6.2) — taxonomy of why a
/// `TaskBankruptcyTx` was emitted. Discriminator on the policy that
/// triggered bankruptcy (max failed run count / deadline exceeded /
/// architect-future evidence-converged failure).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum BankruptcyReason {
    /// Task accumulated >= TASK_BANKRUPTCY_FAILED_RUN_COUNT_MIN RunExhausted events.
    MaxFailedRunCount = 0,
    /// Task open-time + TASK_BANKRUPTCY_DEADLINE_LOGICAL_T_DELTA exceeded
    /// without Finalized claim.
    DeadlineExceeded = 1,
    /// Architect-future hook: EvidenceCapsule rollup convergence indicates
    /// task is fundamentally unsolvable. Reserved for TB-15+ Markov Loom
    /// policy.
    EvidenceConvergedFailure = 2,
}

impl Default for BankruptcyReason {
    fn default() -> Self {
        Self::MaxFailedRunCount
    }
}

/// TRACE_MATRIX FC1-N1: TB-11 (architect §6.1) — taxonomy of why an
/// evaluator run reached terminal exhaustion. Maps 1:1 to `RunOutcome`
/// failure variants (Art. IV halt_reason taxonomy); 5 variants vs
/// RunOutcome's 4 because architect §6.1 distinguishes ProtocolCollapse
/// from SolverGiveUp at the capsule level (both fold into RunOutcome::ErrorHalt
/// at the chain level).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum ExhaustionReason {
    MaxTxExhausted = 0,
    WallClockCap = 1,
    ComputeCap = 2,
    ProtocolCollapse = 3,
    SolverGiveUp = 4,
}

impl Default for ExhaustionReason {
    fn default() -> Self {
        Self::MaxTxExhausted
    }
}

impl ExhaustionReason {
    /// TRACE_MATRIX Art.IV halt_reason taxonomy: project `ExhaustionReason`
    /// to the canonical `RunOutcome` discriminator stored on
    /// `TerminalSummaryTx.run_outcome`. ProtocolCollapse / SolverGiveUp
    /// both map to `ErrorHalt` since `RunOutcome` is the constitutional
    /// taxonomy and is intentionally narrower.
    pub fn to_run_outcome(self) -> RunOutcome {
        match self {
            Self::MaxTxExhausted => RunOutcome::MaxTxExhausted,
            Self::WallClockCap => RunOutcome::WallClockCap,
            Self::ComputeCap => RunOutcome::ComputeCap,
            Self::ProtocolCollapse | Self::SolverGiveUp => RunOutcome::ErrorHalt,
        }
    }
}

/// TRACE_MATRIX FC1-N1: TB-11 (architect §6.1 屏蔽规则) — privacy policy
/// for a CAS-resident `EvidenceCapsule`. Default `AuditOnly` —
/// public_summary may be surfaced to dashboard / read view, raw
/// compressed evidence requires authorized audit-role access.
/// Constitutional: 顶层白盒 quantize/broadcast/shield (Art. II.2.1) means
/// raw failure logs cannot pollute future Agent context — only the
/// public_summary surface broadcasts; capsule's compressed_log is shielded
/// behind audit role.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum CapsulePrivacyPolicy {
    /// Default — only `public_summary` field surfaces to non-audit views;
    /// raw compressed log requires direct CAS read.
    AuditOnly = 0,
    /// public_summary may also enter Librarian message_board for next-iteration
    /// agents (TB-15 Markov Loom prep).
    PublicSummaryBroadcast = 1,
    /// Full evidence visible to a designated audit-role (TB-17+ ChallengeCourt prep).
    AuthorizedCAS = 2,
}

impl Default for CapsulePrivacyPolicy {
    fn default() -> Self {
        Self::AuditOnly
    }
}

impl Default for RunOutcome {
    fn default() -> Self {
        Self::OmegaAccepted
    }
}

// ────────────────────────────────────────────────────────────────────────────
// § 5a-TB-12 — NodePosition exposure record (architect 2026-05-03 ruling)
//
// TB-12 = Node exposure index, NOT trading market. Architect §10 critical
// insight: NodePosition is **immutable exposure record**, not active position
// balance. TB-12 forbids close / settle / transfer / mark-to-market —
// those land in TB-13 (CompleteSet) + TB-14 (PriceIndex) + TB-16
// (controlled-arena P&L).
//
// FORBIDDEN in TB-12 (architect §9.4):
//   No NodeMarketEntry as canonical EconomicState field (TB-14 derived view).
//   No MarketBuy / MarketSell PositionKind variants (TB-13+ trading layer).
//   No price_yes / price_no calculation (TB-14).
//   No CompleteSet / MarketSeedTx / AMM / CPMM (TB-13/14 territory).
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX TB-12 Atom 1 (architect 2026-05-03 ruling §3 + §8 Atom 1):
/// position side discriminator. TB-12 only uses Long / Short. Long is
/// derived from accepted `WorkTx.stake`; Short is from accepted
/// `ChallengeTx.stake`. Per FR-12.3 + CR-12.8: VerifyTx.bond is NEITHER
/// (responsibility bond, not market side).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum PositionSide {
    Long = 0,
    Short = 1,
}

impl Default for PositionSide {
    fn default() -> Self {
        Self::Long
    }
}

/// TRACE_MATRIX TB-12 Atom 1 (architect 2026-05-03 ruling §8 Atom 1):
/// position kind. **TB-12 only ships FirstLong + ChallengeShort.**
/// `MarketBuy` / `MarketSell` are explicitly forbidden (architect §9.4 +
/// §10) — they belong to the future TB-13+ trading layer. Adding them
/// now would prematurely commit to a trading semantics not yet
/// architected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum PositionKind {
    /// Position derived from accepted `WorkTx.stake` (work-side commitment
    /// to a node). FirstLong.node_id == own work_tx_id (architect §4 +
    /// FR-12.4).
    FirstLong = 0,
    /// Position derived from accepted `ChallengeTx.stake` (challenge-side
    /// commitment). ChallengeShort.node_id == challenge.target_work_tx
    /// (architect §4 + FR-12.5).
    ChallengeShort = 1,
}

impl Default for PositionKind {
    fn default() -> Self {
        Self::FirstLong
    }
}

/// TRACE_MATRIX TB-12 Atom 1 (architect 2026-05-03 §3 + §4 + §10 critical
/// insight): **IMMUTABLE EXPOSURE RECORD**, NOT a Coin holding.
///
/// **What this struct IS** (architect §10):
/// - A frozen record that "this Agent took Long/Short directional risk on
///   this node at the moment of accepting their source_tx".
/// - Q-derived from typed-tx fields at accept time (replay-deterministic).
/// - Read-only after creation in TB-12.
///
/// **What this struct IS NOT** (architect §10 + §9.4 forbidden list):
/// - NOT a Coin holding (CR-12.1; NodePosition.amount is NOT in
///   `total_supply_micro`; CR-12.2).
/// - NOT a tradable share balance.
/// - NOT a YES/NO claim (TB-13 CompleteSet territory).
/// - NOT an LP share or market order.
/// - NOT closeable / settleable / transferable / mark-to-marketable in
///   TB-12 (those operations land in TB-13+ / TB-16).
///
/// **TB-12 invariants**:
/// - `position_id == source_tx` (one accepted source-tx ↔ one position;
///   architect §4 last paragraph). Future MarketBuyTx may break this 1:1
///   when one trade produces multiple lots; that's TB-13+ territory.
/// - `FirstLong`: `node_id == source_tx == work.tx_id`,
///   `owner == work.agent_id`, `amount == work.stake.into()`,
///   `side == Long`, `kind == FirstLong`.
/// - `ChallengeShort`: `node_id == challenge.target_work_tx`,
///   `source_tx == position_id == challenge.tx_id`,
///   `owner == challenge.challenger_agent`,
///   `amount == challenge.stake.into()`, `side == Short`,
///   `kind == ChallengeShort`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct NodePosition {
    pub position_id: TxId,
    pub node_id: TxId,
    pub task_id: TaskId,
    pub owner: AgentId,
    pub side: PositionSide,
    pub kind: PositionKind,
    pub amount: MicroCoin,
    pub source_tx: TxId,
    pub opened_at_round: u64,
}

// ────────────────────────────────────────────────────────────────────────────
// § 5b TB-3 RSP-1 formal-tx-surface — TaskOpenTx + EscrowLockTx
//
// Per TB-3 charter v2 (`handover/tracer_bullets/TB-3_charter_2026-04-30.md`):
// - § 3.1 WP-canonical: only TWO new TypedTx variants are introduced
//   (TaskOpenTx + EscrowLockTx). NO YesStakeTx variant; YES stake stays
//   inline in `WorkTx.stake` per WP § 14.1 + § 18 Inv 5.
// - § 3.3 TaskOpen / EscrowLock semantics: TaskOpen is metadata-only (no
//   money); EscrowLock is the sole RSP-1 bounty funding path (atomic
//   balances → escrow transfer + total_escrow cache update).
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX TB-3 charter § 4.1 + WP § 19 RSP-1 — task-open transaction.
///
/// Sponsor opens a task market entry; **does not move money** (per charter
/// § 3.3: TaskOpen is metadata-only). Idempotency: a `TaskOpenTx` for an
/// already-open `task_id` is rejected with `TransitionError::TaskAlreadyOpen`.
/// Funding flows through the separate `EscrowLockTx` admission gate; an
/// opened-but-unfunded task carries `total_escrow == 0` which fails
/// `WorkTx` admission step 2 (TB-3 charter § 3.1).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TaskOpenTx {
    pub tx_id: TxId,                           //  1
    pub task_id: TaskId,                       //  2 — TaskMarketsIndex key
    pub parent_state_root: Hash,               //  3
    pub sponsor_agent: AgentId,                //  4 — becomes TaskMarketEntry.publisher
    pub verifier_quorum: u32,                  //  5 — RSP-2+ field; default 1
    pub max_reuse_royalty_fraction_basis_points: u16, //  6 — RSP-5+ cap; default 1000 (10%)
    pub settlement_rule_hash: Hash,            //  7 — RSP-3/4 hook; opaque hash for TB-3
    pub signature: AgentSignature,             //  8
    pub timestamp_logical: u64,                //  9
}

/// TRACE_MATRIX TB-3 charter § 4.1 + WP § 19 RSP-1 — escrow-lock transaction.
///
/// **The sole RSP-1 bounty funding path**. Atomically debits
/// `balances_t[sponsor]`, credits `escrows_t[tx_id]` with the new
/// `EscrowEntry { amount, depositor, task_id }`, and updates the
/// `task_markets_t[task_id]` cache (`total_escrow += amount`,
/// `escrow_lock_tx_ids.insert(tx_id)`). Per charter § 3.2 the cache is
/// derived; the source of truth is `escrows_t.amount`. Per § 3 P3 Forbidden
/// CF-2 ("no ghost liquidity"): every `total_escrow` increase MUST be a
/// single `EscrowLockTx` with paired balance debit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct EscrowLockTx {
    pub tx_id: TxId,                           //  1 — EscrowsIndex key
    pub task_id: TaskId,                       //  2 — must reference an open task
    pub parent_state_root: Hash,               //  3
    pub sponsor_agent: AgentId,                //  4 — depositor (publisher OR third-party top-up)
    pub amount: MicroCoin,                     //  5 — debited from balances_t[sponsor]
    pub signature: AgentSignature,             //  6
    pub timestamp_logical: u64,                //  7
}

// ────────────────────────────────────────────────────────────────────────────
// § 5c TB-5 RSP-3.0/3.1 system-emitted resolution surface — ChallengeResolveTx
//
// Per TB-5 charter v2 § 4.1 + § 4.5 + preflight v2 § 5.1:
//   - First-class allowed-named system-only TypedTx variant (per WP § 19
//     RSP-1 ChallengeCourt module + ROADMAP § 3 P3 transactions list).
//   - System-emitted ONLY: agent ingress (`Sequencer::submit_agent_tx`) rejects
//     this variant pre-queue with `SubmitError::SystemTxForbiddenOnAgentIngress`
//     (TB-5.0 Atom 2). System ingress (`Sequencer::emit_system_tx`, TB-5 Atom 4)
//     constructs + signs internally with the runtime's system_keypair.
//   - Released path (TB-5.1 Atom 5) refunds challenger bond + flips
//     ChallengeCase.status to Released (no removal; audit trail preserved).
//   - UpheldDeferred path (TB-5.1 Atom 6) is a marker only — slash is
//     RSP-3.2 / TB-6 territory.
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX TB-5 charter v2 § 4.5 — system-emitted challenge resolution.
/// Cannot enter Q via agent ingress (charter § 4.9 + § 5.0 substrate barrier);
/// must come through Sequencer::emit_system_tx which signs internally.
/// Released → challenger bond returns, case.status = Released.
/// UpheldDeferred → marker only; ChallengeCase preserved for TB-6 slash.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ChallengeResolveTx {
    pub tx_id: TxId,                                //  1
    pub parent_state_root: Hash,                    //  2
    pub target_challenge_tx_id: TxId,               //  3 — keys challenge_cases_t lookup
    pub resolution: ChallengeResolution,            //  4
    pub epoch: SystemEpoch,                         //  5
    pub timestamp_logical: u64,                     //  6
    pub system_signature: SystemSignature,          //  7
}

/// TRACE_MATRIX TB-5 charter v2 § 4.5 — resolution outcome (per directive Q4).
/// Released = TB-5.1 active path (CTF round-trip; bond refunded).
/// UpheldDeferred = TB-5.1 marker-only path (slash deferred to TB-6).
/// Lives in typed_tx.rs alongside ChallengeResolveTx; ChallengeStatus
/// (Open/Released/UpheldDeferred for case-state tracking) lives in
/// q_state.rs per Codex round-2 + round-3 Q4 single-source-of-truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum ChallengeResolution {
    Released = 0,
    UpheldDeferred = 1,
}

impl Default for ChallengeResolution {
    fn default() -> Self {
        Self::Released
    }
}

// ────────────────────────────────────────────────────────────────────────────
// § 7 Signing payloads (CO1.1.4-pre1 v1.1 round-1 closure C-1)
//
// Each agent-signed and system-emitted typed-tx has a paired `*SigningPayload`
// struct (subset of fields, EXCLUDES the signature itself) with a
// `canonical_digest()` method that **prepends a stable domain-separation
// prefix** before the bincode-canonical body bytes. This implements:
//
//   sig_input = sha256(b"turingosv4.<actor>.<purpose>.v1" || canonical_encode(payload))
//
// Property: even if two distinct payload TYPES happen to bincode-encode to
// identical bytes (extremely unlikely given distinct field shapes, but
// defensively guaranteed), the domain prefix ensures the SHA-256 inputs
// differ. Closes Codex Q-E + Gemini Q7: type-level distinction is necessary
// but not sufficient as a security boundary.
//
// **Forward dependency**: actual `verify_agent_signature(sig, payload, agent_pubkey)`
// + agent-pubkey-registry lookup is CO P2.x AgentRegistry territory; this
// atom only freezes the canonical_digest pre-image.
// ────────────────────────────────────────────────────────────────────────────

const DOMAIN_AGENT_WORK: &[u8] = b"turingosv4.agent_sig.work.v1";
const DOMAIN_AGENT_VERIFY: &[u8] = b"turingosv4.agent_sig.verify.v1";
const DOMAIN_AGENT_CHALLENGE: &[u8] = b"turingosv4.agent_sig.challenge.v1";
const DOMAIN_AGENT_TASK_OPEN: &[u8] = b"turingosv4.agent_sig.task_open.v1";       // TB-3 RSP-1
const DOMAIN_AGENT_ESCROW_LOCK: &[u8] = b"turingosv4.agent_sig.escrow_lock.v1";   // TB-3 RSP-1
const DOMAIN_SYSTEM_FINALIZE_REWARD: &[u8] = b"turingosv4.system_sig.finalize_reward.v1";
const DOMAIN_SYSTEM_TASK_EXPIRE: &[u8] = b"turingosv4.system_sig.task_expire.v1";
const DOMAIN_SYSTEM_TERMINAL_SUMMARY: &[u8] = b"turingosv4.system_sig.terminal_summary.v1";
const DOMAIN_SYSTEM_CHALLENGE_RESOLVE: &[u8] = b"turingosv4.system_sig.challenge_resolve.v1"; // TB-5 Atom 3
const DOMAIN_SYSTEM_TASK_BANKRUPTCY: &[u8] = b"turingosv4.system_sig.task_bankruptcy.v1";    // TB-11
// TB-13 — CompleteSet + MarketSeedTx (architect 2026-05-03 post-TB-12 ruling Part A §4.3).
// All three TB-13 typed-tx are AGENT-SIGNED (provider funds explicit; no
// auto-seed; redeem requires system-resolution-reference + outcome match,
// gated sequencer-side at admission). Domain prefixes mirror existing
// agent-domain naming conventions (`turingosv4.agent_sig.<purpose>.v1`).
const DOMAIN_AGENT_COMPLETE_SET_MINT: &[u8] = b"turingosv4.agent_sig.complete_set_mint.v1";
const DOMAIN_AGENT_COMPLETE_SET_REDEEM: &[u8] = b"turingosv4.agent_sig.complete_set_redeem.v1";
const DOMAIN_AGENT_MARKET_SEED: &[u8] = b"turingosv4.agent_sig.market_seed.v1";

/// Reserved for v4.1 MetaTx (Gemini round-2 GR-1 recommendation).
/// Not used in v4 — namespace placeholder so v4.1 can introduce
/// `MetaSigningPayload` without re-rotating sibling domains. Marked
/// `#[allow(dead_code)]` because no v4 consumer references it.
#[allow(dead_code)]
const DOMAIN_AGENT_META_PROPOSAL: &[u8] = b"turingosv4.agent_sig.meta_proposal.v1";

fn domain_prefixed_digest<T: Serialize>(domain: &[u8], value: &T) -> [u8; 32] {
    use crate::bottom_white::ledger::transition_ledger::canonical_encode;
    let body = canonical_encode(value).expect("canonical_encode of signing payload");
    let mut h = Sha256::new();
    h.update(domain);
    h.update(&body);
    h.finalize().into()
}

/// Agent signing payload for `WorkTx` (12 fields → 11 fields; signature excluded).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct WorkSigningPayload {
    pub tx_id: TxId,
    pub task_id: TaskId,
    pub parent_state_root: Hash,
    pub agent_id: AgentId,
    pub read_set: BTreeSet<ReadKey>,
    pub write_set: BTreeSet<WriteKey>,
    pub proposal_cid: Cid,
    pub predicate_results: PredicateResultsBundle,
    pub stake: StakeMicroCoin,
    pub timestamp_logical: u64,
}

impl WorkSigningPayload {
    pub fn canonical_digest(&self) -> [u8; 32] {
        domain_prefixed_digest(DOMAIN_AGENT_WORK, self)
    }
}

/// Agent signing payload for `VerifyTx` (8 fields → 7 fields; TB-4 bump).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct VerifySigningPayload {
    pub tx_id: TxId,
    pub parent_state_root: Hash,           // TB-4 NEW
    pub target_work_tx: TxId,
    pub verifier_agent: AgentId,
    pub bond: StakeMicroCoin,
    pub verdict: VerifyVerdict,
    pub timestamp_logical: u64,
}

impl VerifySigningPayload {
    pub fn canonical_digest(&self) -> [u8; 32] {
        domain_prefixed_digest(DOMAIN_AGENT_VERIFY, self)
    }
}

/// Agent signing payload for `ChallengeTx` (8 fields → 7 fields; TB-4 bump).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ChallengeSigningPayload {
    pub tx_id: TxId,
    pub parent_state_root: Hash,           // TB-4 NEW
    pub target_work_tx: TxId,
    pub challenger_agent: AgentId,
    pub stake: StakeMicroCoin,
    pub counterexample_cid: Cid,
    pub timestamp_logical: u64,
}

impl ChallengeSigningPayload {
    pub fn canonical_digest(&self) -> [u8; 32] {
        domain_prefixed_digest(DOMAIN_AGENT_CHALLENGE, self)
    }
}

/// TRACE_MATRIX TB-3 — agent signing payload for `TaskOpenTx` (9 fields → 8 fields; signature excluded).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TaskOpenSigningPayload {
    pub tx_id: TxId,
    pub task_id: TaskId,
    pub parent_state_root: Hash,
    pub sponsor_agent: AgentId,
    pub verifier_quorum: u32,
    pub max_reuse_royalty_fraction_basis_points: u16,
    pub settlement_rule_hash: Hash,
    pub timestamp_logical: u64,
}

impl TaskOpenSigningPayload {
    pub fn canonical_digest(&self) -> [u8; 32] {
        domain_prefixed_digest(DOMAIN_AGENT_TASK_OPEN, self)
    }
}

/// TRACE_MATRIX TB-3 — agent signing payload for `EscrowLockTx` (7 fields → 6 fields).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct EscrowLockSigningPayload {
    pub tx_id: TxId,
    pub task_id: TaskId,
    pub parent_state_root: Hash,
    pub sponsor_agent: AgentId,
    pub amount: MicroCoin,
    pub timestamp_logical: u64,
}

impl EscrowLockSigningPayload {
    pub fn canonical_digest(&self) -> [u8; 32] {
        domain_prefixed_digest(DOMAIN_AGENT_ESCROW_LOCK, self)
    }
}

/// System signing payload for `FinalizeRewardTx` (9 fields → 8 fields).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct FinalizeRewardSigningPayload {
    pub tx_id: TxId,
    pub claim_id: ClaimId,
    pub task_id: TaskId,
    pub solver: AgentId,
    pub reward: MicroCoin,
    pub parent_state_root: Hash,
    pub epoch: SystemEpoch,
    pub timestamp_logical: u64,
}

impl FinalizeRewardSigningPayload {
    pub fn canonical_digest(&self) -> [u8; 32] {
        domain_prefixed_digest(DOMAIN_SYSTEM_FINALIZE_REWARD, self)
    }
}

/// System signing payload for `TaskExpireTx` (TB-11 bump: 10 fields → 9 fields).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TaskExpireSigningPayload {
    pub tx_id: TxId,
    pub task_id: TaskId,
    pub parent_state_root: Hash,
    pub bounty_refunded: MicroCoin,
    pub epoch: SystemEpoch,
    pub timestamp_logical: u64,
    pub sponsor_agent: AgentId,           // TB-11 NEW
    pub escrow_tx_id: TxId,               // TB-11 NEW
    pub reason: ExpireReason,             // TB-11 NEW
}

impl TaskExpireSigningPayload {
    pub fn canonical_digest(&self) -> [u8; 32] {
        domain_prefixed_digest(DOMAIN_SYSTEM_TASK_EXPIRE, self)
    }
}

/// System signing payload for `TerminalSummaryTx` (TB-11 bump: 11 fields → 10 fields).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TerminalSummarySigningPayload {
    pub tx_id: TxId,
    pub task_id: TaskId,
    pub run_id: RunId,
    pub run_outcome: RunOutcome,
    pub total_attempts: u32,
    pub failure_class_histogram: BTreeMap<RejectionClass, u32>,
    pub last_logical_t: u64,
    pub parent_state_root: Hash,                  // TB-11 NEW
    pub solver_agent: Option<AgentId>,            // TB-11 NEW
    pub evidence_capsule_cid: Option<Cid>,        // TB-11 NEW
}

impl TerminalSummarySigningPayload {
    pub fn canonical_digest(&self) -> [u8; 32] {
        domain_prefixed_digest(DOMAIN_SYSTEM_TERMINAL_SUMMARY, self)
    }
}

/// TRACE_MATRIX FC1-Sig + FC3-Sig: TB-11 — System signing payload for
/// `TaskBankruptcyTx` (9 fields → 8 fields; system_signature excluded).
/// Signed via `CanonicalMessage::TaskBankruptcySigning([u8;32])` opaque
/// digest pattern (mirrors FinalizeRewardSigningPayload /
/// TaskExpireSigningPayload / TerminalSummarySigningPayload).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TaskBankruptcySigningPayload {
    pub tx_id: TxId,
    pub parent_state_root: Hash,
    pub task_id: TaskId,
    pub evidence_capsule_cid: Cid,
    pub bankruptcy_reason: BankruptcyReason,
    pub failed_run_count: u32,
    pub epoch: SystemEpoch,
    pub timestamp_logical: u64,
}

impl TaskBankruptcySigningPayload {
    /// TRACE_MATRIX FC1-Sig: domain-prefixed canonical digest for
    /// system-emitted TaskBankruptcyTx signing. Domain prefix
    /// `b"turingosv4.system_sig.task_bankruptcy.v1"` mirrors the existing
    /// 4 system-tx signing domains (TerminalSummary / FinalizeReward /
    /// TaskExpire / ChallengeResolve).
    pub fn canonical_digest(&self) -> [u8; 32] {
        domain_prefixed_digest(DOMAIN_SYSTEM_TASK_BANKRUPTCY, self)
    }
}

/// TRACE_MATRIX TB-5 charter v2 § 4.5 — system signing payload for
/// `ChallengeResolveTx` (7 fields → 6 fields; signature excluded).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ChallengeResolveSigningPayload {
    pub tx_id: TxId,
    pub parent_state_root: Hash,
    pub target_challenge_tx_id: TxId,
    pub resolution: ChallengeResolution,
    pub epoch: SystemEpoch,
    pub timestamp_logical: u64,
}

impl ChallengeResolveSigningPayload {
    /// TRACE_MATRIX TB-5 charter v2 § 4.5: domain-prefixed canonical digest
    /// for system-emitted ChallengeResolveTx signing. Domain prefix
    /// `b"turingosv4.system_sig.challenge_resolve.v1"` mirrors the existing
    /// 3 system-tx signing domains.
    pub fn canonical_digest(&self) -> [u8; 32] {
        domain_prefixed_digest(DOMAIN_SYSTEM_CHALLENGE_RESOLVE, self)
    }
}

// ────────────────────────────────────────────────────────────────────────────
// § 5c-TB-13 — CompleteSet + MarketSeedTx conditional shares
//
// TRACE_MATRIX TB-13 Atom 1 (architect 2026-05-03 post-TB-12 ruling Part A
// §4.3 + §4.4 FR-13.1..7 + §4.5 CR-13.1..6).
//
// **Mathematical core**: `1 locked Coin = 1 YES_E + 1 NO_E`.
// `CompleteSetMintTx` debits Coin balance, locks it as `conditional_collateral_t`,
// mints equal YES_E + NO_E shares to the same owner. `CompleteSetRedeemTx`
// requires a system-resolved outcome reference and pays the winning side
// 1:1 against `conditional_collateral_t`. `MarketSeedTx` requires explicit
// provider funds; no auto-seed, no quote, no trade, no price.
//
// **Forbidden in TB-13** (architect §4.7): AMM / CPMM / orderbook /
// MarketOrderTx / MarketTradeTx / PriceIndex / DPMM / pro-rata / automatic
// liquidity / ghost liquidity / NodeMarketEntry as canonical state / f64.
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX TB-13 Atom 1 (architect §4.3): event identifier for
/// conditional shares. TB-13 maps `EventId` 1:1 to `TaskId` (the event
/// being resolved is "this task got finalized YES via FinalizeRewardTx
/// vs. died NO via TaskBankruptcyTx"); future TB-14+ may decouple to
/// per-node events.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct EventId(pub TaskId);

/// TRACE_MATRIX TB-13 Atom 1 (architect §4.3): outcome-side discriminator
/// for conditional shares. Yes = "this event was finalized YES";
/// No = "this event went bankrupt / was rejected".
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum OutcomeSide {
    Yes = 0,
    No = 1,
}

impl Default for OutcomeSide {
    fn default() -> Self {
        Self::Yes
    }
}

/// TRACE_MATRIX TB-15 Atom 2 (architect §6.2): identifier for a
/// protocol-level risk rule (`max_position_size`, `max_drawdown`,
/// `max_slippage`, `max_leverage`, `kelly_cap`, ...). Carried by
/// `AgentAutopsyCapsule.violated_risk_rule` as `Option<RiskRuleId>` —
/// names the protocol invariant that triggered the loss event, when
/// applicable. Opaque newtype so the autopsy writer never depends on
/// the live risk-rule registry (CR-15.3 — autopsy may suggest, never
/// mutate).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct RiskRuleId(pub String);

/// TRACE_MATRIX TB-13 Atom 1 (architect §4.3): non-negative share count.
///
/// Architect spec uses `units: i128`; we tighten to `u128` because TB-13
/// shares can never be negative (mint creates positive, redeem decreases
/// positive, no debt model). Underflow at redeem time is a sequencer
/// `RedeemMoreThanOwned` rejection, not a representation concern.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct ShareAmount {
    pub units: u128,
}

impl ShareAmount {
    /// TRACE_MATRIX TB-13 Atom 1 (architect §4.3): zero share amount —
    /// default constructor for empty share balance lookups.
    pub const fn zero() -> Self {
        Self { units: 0 }
    }
    /// TRACE_MATRIX TB-13 Atom 1 (architect §4.3): build a `ShareAmount`
    /// from a raw `u128` units count. Used by sequencer mint/redeem arms
    /// (Atom 2) to project `MicroCoin::micro_units() as u128` into the
    /// share-claim domain.
    pub const fn from_units(units: u128) -> Self {
        Self { units }
    }
}

/// TRACE_MATRIX TB-13 Atom 1 (architect §4.3 + FR-13.1..3): mint conditional
/// shares against locked Coin collateral.
///
/// Sequencer arm (Atom 2):
/// 1. `balances_t[owner] >= amount` else `InsufficientBalanceForMint`.
/// 2. `balances_t[owner] -= amount`.
/// 3. `conditional_collateral_t[event_id] += amount`.
/// 4. `conditional_share_balances_t[(owner, event_id, Yes)] += amount.units`.
/// 5. `conditional_share_balances_t[(owner, event_id, No)]  += amount.units`.
///
/// CTF preserved: balance debit equals collateral credit; YES/NO shares
/// are claims (not Coin) per CR-13.3 / SG-13.2.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct CompleteSetMintTx {
    pub tx_id: TxId,                          //  1
    pub parent_state_root: Hash,              //  2
    pub event_id: EventId,                    //  3
    pub owner: AgentId,                       //  4
    pub amount: MicroCoin,                    //  5
    pub signature: AgentSignature,            //  6
    pub timestamp_logical: u64,               //  7
}

/// TRACE_MATRIX TB-13 Atom 1 (architect §4.3 + FR-13.4..5 + SG-13.5..6):
/// redeem winning conditional shares post-resolution.
///
/// **Resolution authority is the live `task_markets_t[event_id.0].state`**
/// (Finalized → Yes wins; Bankrupt → No wins). The redeem carries no
/// resolution-ref wrapper — `outcome` IS the claim and the sequencer
/// reconciles it against state.
///
/// Sequencer arm (Atom 2):
/// 1. Look up `task_markets_t[event_id.0].state`:
///    - If `Finalized`: `outcome` must be `Yes` else `InvalidResolutionRef`.
///    - If `Bankrupt`:  `outcome` must be `No`  else `InvalidResolutionRef`.
///    - If `Open` or `Expired`: `RedeemBeforeResolution`.
///    - If absent: `RedeemBeforeResolution`.
/// 2. `conditional_share_balances_t[(owner, event_id, outcome)] >= share_amount.units`
///    else `RedeemMoreThanOwned`.
/// 3. `conditional_collateral_t[event_id] >= share_amount.units` else
///    `InsufficientCollateral` (defensive; should never fire if
///    `assert_complete_set_balanced` holds).
/// 4. Debit shares; debit collateral; credit `balances_t[owner]` 1:1.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct CompleteSetRedeemTx {
    pub tx_id: TxId,                          //  1
    pub parent_state_root: Hash,              //  2
    pub event_id: EventId,                    //  3
    pub owner: AgentId,                       //  4
    pub outcome: OutcomeSide,                 //  5
    pub share_amount: ShareAmount,            //  6
    pub signature: AgentSignature,            //  7
    pub timestamp_logical: u64,               //  8
}

/// TRACE_MATRIX TB-13 Atom 1 (architect §4.3 + FR-13.6..7): explicit
/// provider-funded protocol-owned share inventory seed. **NO trading,
/// NO quoting, NO pricing.**
///
/// Sequencer arm (Atom 2):
/// 1. `collateral_amount > 0` else `InsufficientCollateral` (SG-13.4).
/// 2. `balances_t[provider] >= collateral_amount` else
///    `InsufficientBalanceForMint` (SG-13.3).
/// 3. `balances_t[provider] -= collateral_amount`.
/// 4. `conditional_collateral_t[event_id] += collateral_amount`.
/// 5. Provider receives BOTH sides of share inventory:
///    `conditional_share_balances_t[(provider, event_id, Yes)] += collateral_amount.units`
///    `conditional_share_balances_t[(provider, event_id, No)]  += collateral_amount.units`.
///
/// The shape is identical to `CompleteSetMintTx` post-effect; the
/// distinction is semantic ("mint" = claim against own bet vs "seed" =
/// protocol-owned inventory pre-resolution). Future tracer-bullets may
/// treat seeded liquidity differently — TB-13 itself records only the
/// fact of seeding, not any signal derived from it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct MarketSeedTx {
    pub tx_id: TxId,                          //  1
    pub parent_state_root: Hash,              //  2
    pub event_id: EventId,                    //  3
    pub provider: AgentId,                    //  4
    pub collateral_amount: MicroCoin,         //  5
    pub signature: AgentSignature,            //  6
    pub timestamp_logical: u64,               //  7
}

// ── TB-13 SigningPayloads ───────────────────────────────────────────────

/// TRACE_MATRIX TB-13 Atom 1 (architect §4.3): signing payload for
/// `CompleteSetMintTx` (7 fields → 6 fields; signature excluded).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct CompleteSetMintSigningPayload {
    pub tx_id: TxId,
    pub parent_state_root: Hash,
    pub event_id: EventId,
    pub owner: AgentId,
    pub amount: MicroCoin,
    pub timestamp_logical: u64,
}

impl CompleteSetMintSigningPayload {
    /// TRACE_MATRIX TB-13 Atom 1 (architect §4.3): domain-prefixed
    /// canonical digest for agent-signed CompleteSetMintTx. Domain
    /// prefix `b"turingosv4.agent_sig.complete_set_mint.v1"` mirrors
    /// agent-domain naming (Work / Verify / Challenge / TaskOpen /
    /// EscrowLock).
    pub fn canonical_digest(&self) -> [u8; 32] {
        domain_prefixed_digest(DOMAIN_AGENT_COMPLETE_SET_MINT, self)
    }
}

/// TRACE_MATRIX TB-13 Atom 1 (architect §4.3): signing payload for
/// `CompleteSetRedeemTx` (8 fields → 7 fields; signature excluded).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct CompleteSetRedeemSigningPayload {
    pub tx_id: TxId,
    pub parent_state_root: Hash,
    pub event_id: EventId,
    pub owner: AgentId,
    pub outcome: OutcomeSide,
    pub share_amount: ShareAmount,
    pub timestamp_logical: u64,
}

impl CompleteSetRedeemSigningPayload {
    /// TRACE_MATRIX TB-13 Atom 1 (architect §4.3): domain-prefixed
    /// canonical digest for agent-signed CompleteSetRedeemTx. Domain
    /// prefix `b"turingosv4.agent_sig.complete_set_redeem.v1"`.
    pub fn canonical_digest(&self) -> [u8; 32] {
        domain_prefixed_digest(DOMAIN_AGENT_COMPLETE_SET_REDEEM, self)
    }
}

/// TRACE_MATRIX TB-13 Atom 1 (architect §4.3): signing payload for
/// `MarketSeedTx` (7 fields → 6 fields; signature excluded).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct MarketSeedSigningPayload {
    pub tx_id: TxId,
    pub parent_state_root: Hash,
    pub event_id: EventId,
    pub provider: AgentId,
    pub collateral_amount: MicroCoin,
    pub timestamp_logical: u64,
}

impl MarketSeedSigningPayload {
    /// TRACE_MATRIX TB-13 Atom 1 (architect §4.3): domain-prefixed
    /// canonical digest for agent-signed MarketSeedTx. Domain prefix
    /// `b"turingosv4.agent_sig.market_seed.v1"`.
    pub fn canonical_digest(&self) -> [u8; 32] {
        domain_prefixed_digest(DOMAIN_AGENT_MARKET_SEED, self)
    }
}

// ── Projections: tx → signing payload ────────────────────────────────────

impl WorkTx {
    pub fn to_signing_payload(&self) -> WorkSigningPayload {
        WorkSigningPayload {
            tx_id: self.tx_id.clone(),
            task_id: self.task_id.clone(),
            parent_state_root: self.parent_state_root,
            agent_id: self.agent_id.clone(),
            read_set: self.read_set.clone(),
            write_set: self.write_set.clone(),
            proposal_cid: self.proposal_cid,
            predicate_results: self.predicate_results.clone(),
            stake: self.stake,
            timestamp_logical: self.timestamp_logical,
        }
    }
}

impl VerifyTx {
    pub fn to_signing_payload(&self) -> VerifySigningPayload {
        VerifySigningPayload {
            tx_id: self.tx_id.clone(),
            parent_state_root: self.parent_state_root,
            target_work_tx: self.target_work_tx.clone(),
            verifier_agent: self.verifier_agent.clone(),
            bond: self.bond,
            verdict: self.verdict,
            timestamp_logical: self.timestamp_logical,
        }
    }
}

impl ChallengeTx {
    pub fn to_signing_payload(&self) -> ChallengeSigningPayload {
        ChallengeSigningPayload {
            tx_id: self.tx_id.clone(),
            parent_state_root: self.parent_state_root,
            target_work_tx: self.target_work_tx.clone(),
            challenger_agent: self.challenger_agent.clone(),
            stake: self.stake,
            counterexample_cid: self.counterexample_cid,
            timestamp_logical: self.timestamp_logical,
        }
    }
}

impl FinalizeRewardTx {
    pub fn to_signing_payload(&self) -> FinalizeRewardSigningPayload {
        FinalizeRewardSigningPayload {
            tx_id: self.tx_id.clone(),
            claim_id: self.claim_id.clone(),
            task_id: self.task_id.clone(),
            solver: self.solver.clone(),
            reward: self.reward,
            parent_state_root: self.parent_state_root,
            epoch: self.epoch,
            timestamp_logical: self.timestamp_logical,
        }
    }
}

impl TaskExpireTx {
    pub fn to_signing_payload(&self) -> TaskExpireSigningPayload {
        TaskExpireSigningPayload {
            tx_id: self.tx_id.clone(),
            task_id: self.task_id.clone(),
            parent_state_root: self.parent_state_root,
            bounty_refunded: self.bounty_refunded,
            epoch: self.epoch,
            timestamp_logical: self.timestamp_logical,
            sponsor_agent: self.sponsor_agent.clone(),
            escrow_tx_id: self.escrow_tx_id.clone(),
            reason: self.reason,
        }
    }
}

impl TerminalSummaryTx {
    pub fn to_signing_payload(&self) -> TerminalSummarySigningPayload {
        TerminalSummarySigningPayload {
            tx_id: self.tx_id.clone(),
            task_id: self.task_id.clone(),
            run_id: self.run_id.clone(),
            run_outcome: self.run_outcome,
            total_attempts: self.total_attempts,
            failure_class_histogram: self.failure_class_histogram.clone(),
            last_logical_t: self.last_logical_t,
            parent_state_root: self.parent_state_root,
            solver_agent: self.solver_agent.clone(),
            evidence_capsule_cid: self.evidence_capsule_cid,
        }
    }
}

impl TaskBankruptcyTx {
    /// TRACE_MATRIX FC1-Sig + FC3-Sig: project the wire struct to the
    /// signing payload subset (excludes `system_signature` to prevent
    /// cycle-on-self).
    pub fn to_signing_payload(&self) -> TaskBankruptcySigningPayload {
        TaskBankruptcySigningPayload {
            tx_id: self.tx_id.clone(),
            parent_state_root: self.parent_state_root,
            task_id: self.task_id.clone(),
            evidence_capsule_cid: self.evidence_capsule_cid,
            bankruptcy_reason: self.bankruptcy_reason,
            failed_run_count: self.failed_run_count,
            epoch: self.epoch,
            timestamp_logical: self.timestamp_logical,
        }
    }
}

impl TaskOpenTx {
    pub fn to_signing_payload(&self) -> TaskOpenSigningPayload {
        TaskOpenSigningPayload {
            tx_id: self.tx_id.clone(),
            task_id: self.task_id.clone(),
            parent_state_root: self.parent_state_root,
            sponsor_agent: self.sponsor_agent.clone(),
            verifier_quorum: self.verifier_quorum,
            max_reuse_royalty_fraction_basis_points: self.max_reuse_royalty_fraction_basis_points,
            settlement_rule_hash: self.settlement_rule_hash,
            timestamp_logical: self.timestamp_logical,
        }
    }
}

impl EscrowLockTx {
    pub fn to_signing_payload(&self) -> EscrowLockSigningPayload {
        EscrowLockSigningPayload {
            tx_id: self.tx_id.clone(),
            task_id: self.task_id.clone(),
            parent_state_root: self.parent_state_root,
            sponsor_agent: self.sponsor_agent.clone(),
            amount: self.amount,
            timestamp_logical: self.timestamp_logical,
        }
    }
}

impl ChallengeResolveTx {
    /// TRACE_MATRIX TB-5 charter v2 § 4.5: tx → signing payload projection
    /// (excludes system_signature; 7 fields → 6 fields). Used by
    /// `Sequencer::emit_system_tx` (Atom 4) to compute the digest the
    /// system_keypair signs over.
    pub fn to_signing_payload(&self) -> ChallengeResolveSigningPayload {
        ChallengeResolveSigningPayload {
            tx_id: self.tx_id.clone(),
            parent_state_root: self.parent_state_root,
            target_challenge_tx_id: self.target_challenge_tx_id.clone(),
            resolution: self.resolution,
            epoch: self.epoch,
            timestamp_logical: self.timestamp_logical,
        }
    }
}

// TB-13 — projection impls.

impl CompleteSetMintTx {
    /// TRACE_MATRIX TB-13 Atom 1 (architect §4.3): wire → signing payload
    /// projection. Excludes `signature` to prevent cycle-on-self.
    pub fn to_signing_payload(&self) -> CompleteSetMintSigningPayload {
        CompleteSetMintSigningPayload {
            tx_id: self.tx_id.clone(),
            parent_state_root: self.parent_state_root,
            event_id: self.event_id.clone(),
            owner: self.owner.clone(),
            amount: self.amount,
            timestamp_logical: self.timestamp_logical,
        }
    }
}

impl CompleteSetRedeemTx {
    /// TRACE_MATRIX TB-13 Atom 1 (architect §4.3): wire → signing payload
    /// projection. Excludes `signature` to prevent cycle-on-self.
    pub fn to_signing_payload(&self) -> CompleteSetRedeemSigningPayload {
        CompleteSetRedeemSigningPayload {
            tx_id: self.tx_id.clone(),
            parent_state_root: self.parent_state_root,
            event_id: self.event_id.clone(),
            owner: self.owner.clone(),
            outcome: self.outcome,
            share_amount: self.share_amount,
            timestamp_logical: self.timestamp_logical,
        }
    }
}

impl MarketSeedTx {
    /// TRACE_MATRIX TB-13 Atom 1 (architect §4.3): wire → signing payload
    /// projection. Excludes `signature` to prevent cycle-on-self.
    pub fn to_signing_payload(&self) -> MarketSeedSigningPayload {
        MarketSeedSigningPayload {
            tx_id: self.tx_id.clone(),
            parent_state_root: self.parent_state_root,
            event_id: self.event_id.clone(),
            provider: self.provider.clone(),
            collateral_amount: self.collateral_amount,
            timestamp_logical: self.timestamp_logical,
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────
// § 6 TypedTx outer enum
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX § 8 dispatch_transition — typed-tx outer enum.
/// **10 variants pre-TB-11; 11 variants TB-11+** (K5 closed: NO `Slash`).
/// v1.1 P3 migrated `TerminalSummaryTx` here. **TB-3 (2026-04-30)**: added
/// `TaskOpen` + `EscrowLock` (RSP-1 formal surface; charter § 4.1). YES stake
/// stays inline in `WorkTx.stake` per WP § 14.1 + § 18 Inv 5; no separate
/// `YesStakeTx` variant. **TB-11 (2026-05-02)**: added `TaskBankruptcy`
/// (system-emitted task-level death certificate; architect §6.2; future
/// TB-12 NodeMarket Short / NO settlement anchor).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TypedTx {
    Work(WorkTx),
    Verify(VerifyTx),
    Challenge(ChallengeTx),
    Reuse(ReuseTx),
    FinalizeReward(FinalizeRewardTx),
    TaskExpire(TaskExpireTx),
    TerminalSummary(TerminalSummaryTx),
    TaskOpen(TaskOpenTx),         // TB-3 RSP-1 formal surface
    EscrowLock(EscrowLockTx),     // TB-3 RSP-1 formal surface
    ChallengeResolve(ChallengeResolveTx), // TB-5 RSP-3.0/3.1 system-emitted resolution
    TaskBankruptcy(TaskBankruptcyTx),     // TB-11 system-emitted task-level failure marker
    CompleteSetMint(CompleteSetMintTx),   // TB-13 agent-signed conditional-share mint
    CompleteSetRedeem(CompleteSetRedeemTx), // TB-13 agent-signed conditional-share redeem
    MarketSeed(MarketSeedTx),             // TB-13 agent-signed protocol-owned share seed
}

impl TypedTx {
    /// Project to the [`TxKind`] discriminator stored in `LedgerEntry.tx_kind`.
    pub fn tx_kind(&self) -> crate::bottom_white::ledger::transition_ledger::TxKind {
        use crate::bottom_white::ledger::transition_ledger::TxKind;
        match self {
            Self::Work(_) => TxKind::Work,
            Self::Verify(_) => TxKind::Verify,
            Self::Challenge(_) => TxKind::Challenge,
            Self::Reuse(_) => TxKind::Reuse,
            Self::FinalizeReward(_) => TxKind::FinalizeReward,
            Self::TaskExpire(_) => TxKind::TaskExpire,
            Self::TerminalSummary(_) => TxKind::TerminalSummary,
            Self::TaskOpen(_) => TxKind::TaskOpen,
            Self::EscrowLock(_) => TxKind::EscrowLock,
            Self::ChallengeResolve(_) => TxKind::ChallengeResolve,
            Self::TaskBankruptcy(_) => TxKind::TaskBankruptcy,
            Self::CompleteSetMint(_) => TxKind::CompleteSetMint,
            Self::CompleteSetRedeem(_) => TxKind::CompleteSetRedeem,
            Self::MarketSeed(_) => TxKind::MarketSeed,
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────
// § 8 HasSubmitter trait (STATE spec § 3.6.5 v1.3)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX STATE spec § 3.6.5 v1.3 — submitter resolution trait used
/// by the implicit-init step in agent-submitted transitions. System-emitted
/// transitions return `None` (no agent to init).
pub trait HasSubmitter {
    fn submitter_id(&self) -> Option<AgentId>;
}

impl HasSubmitter for WorkTx {
    fn submitter_id(&self) -> Option<AgentId> {
        Some(self.agent_id.clone())
    }
}

impl HasSubmitter for VerifyTx {
    fn submitter_id(&self) -> Option<AgentId> {
        Some(self.verifier_agent.clone())
    }
}

impl HasSubmitter for ChallengeTx {
    fn submitter_id(&self) -> Option<AgentId> {
        Some(self.challenger_agent.clone())
    }
}

impl HasSubmitter for ReuseTx {
    fn submitter_id(&self) -> Option<AgentId> {
        None
    }
}

impl HasSubmitter for FinalizeRewardTx {
    fn submitter_id(&self) -> Option<AgentId> {
        None
    }
}

impl HasSubmitter for TaskExpireTx {
    fn submitter_id(&self) -> Option<AgentId> {
        None
    }
}

impl HasSubmitter for TerminalSummaryTx {
    fn submitter_id(&self) -> Option<AgentId> {
        None
    }
}

impl HasSubmitter for TaskOpenTx {
    fn submitter_id(&self) -> Option<AgentId> {
        Some(self.sponsor_agent.clone())
    }
}

impl HasSubmitter for EscrowLockTx {
    fn submitter_id(&self) -> Option<AgentId> {
        Some(self.sponsor_agent.clone())
    }
}

impl HasSubmitter for ChallengeResolveTx {
    fn submitter_id(&self) -> Option<AgentId> {
        None  // system-emitted; mirror FinalizeRewardTx / TaskExpireTx / TerminalSummaryTx
    }
}

impl HasSubmitter for TaskBankruptcyTx {
    fn submitter_id(&self) -> Option<AgentId> {
        None  // TB-11 system-emitted; mirror FinalizeRewardTx / TaskExpireTx / TerminalSummaryTx / ChallengeResolveTx
    }
}

// TB-13 — agent-signed conditional-share variants. Submitter is the
// owner / provider on the wire (mirrors WorkTx → agent_id pattern).

impl HasSubmitter for CompleteSetMintTx {
    fn submitter_id(&self) -> Option<AgentId> {
        Some(self.owner.clone())
    }
}

impl HasSubmitter for CompleteSetRedeemTx {
    fn submitter_id(&self) -> Option<AgentId> {
        Some(self.owner.clone())
    }
}

impl HasSubmitter for MarketSeedTx {
    fn submitter_id(&self) -> Option<AgentId> {
        Some(self.provider.clone())
    }
}

impl HasSubmitter for TypedTx {
    fn submitter_id(&self) -> Option<AgentId> {
        match self {
            Self::Work(t) => t.submitter_id(),
            Self::Verify(t) => t.submitter_id(),
            Self::Challenge(t) => t.submitter_id(),
            Self::Reuse(t) => t.submitter_id(),
            Self::FinalizeReward(t) => t.submitter_id(),
            Self::TaskExpire(t) => t.submitter_id(),
            Self::TerminalSummary(t) => t.submitter_id(),
            Self::TaskOpen(t) => t.submitter_id(),
            Self::EscrowLock(t) => t.submitter_id(),
            Self::ChallengeResolve(t) => t.submitter_id(),
            Self::TaskBankruptcy(t) => t.submitter_id(),
            Self::CompleteSetMint(t) => t.submitter_id(),
            Self::CompleteSetRedeem(t) => t.submitter_id(),
            Self::MarketSeed(t) => t.submitter_id(),
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────
// TransitionError — minimal v1 taxonomy (CO1.1.4-pre1 spec § 0 out-of-scope
// note: full per-stage enum proliferation is CO1.7.5)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX STATE § 3 — transition-function error taxonomy. v1.1 covers
/// every variant invoked in STATE_TRANSITION_SPEC § 3.1-3.7 pseudocode +
/// `NotYetImplemented` for CO1.7.5 stub bodies (per Codex Q-G CHALLENGE).
///
/// **Why payloads are minimal**: the failed `PredicateId` (etc.) is a string
/// reference; richer context (PredicateResultsBundle, Cid of failed proof)
/// is attached by the runtime via separate book-keeping channels (rejected
/// summary stamping, bus rejection log). Keeping TransitionError serializable
/// with primitive payloads avoids forcing PredicateResultsBundle through
/// every error site.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransitionError {
    // ── Stale-parent & signature ───────────────────────────────────────────
    /// `parent_state_root` does not match `q.state_root_t` (any agent tx).
    StaleParent,
    /// Agent signature verify failed (work / verify / challenge tx).
    SignatureInvalid,
    /// System-keypair signature verify failed (system-emitted tx).
    InvalidSystemSignature,

    // ── Economy ────────────────────────────────────────────────────────────
    /// Submitter's available balance is below the declared stake / bond.
    /// Payload-rich variant (available + required) is intentionally elided
    /// in v1.1 to keep this enum primitive-payloads-only; runtime attaches
    /// context via the rejection log (per STATE § 1.4 RejectedAttemptSummary).
    StakeInsufficient,

    // ── Target lookup ──────────────────────────────────────────────────────
    /// VerifyTx / ChallengeTx / ReuseTx target work_tx not found in L4.
    TargetWorkTxNotFound,
    /// VerifyTx target is not in a verifiable status (e.g. already finalized).
    TargetWorkTxNotVerifiable,
    /// ReuseTx target work_tx exists but is not yet Accepted (parent must accept first).
    ParentNotAcceptedYet,

    // ── Predicate failures ─────────────────────────────────────────────────
    /// step_transition stage 4 — acceptance predicate denied. `PredicateId`
    /// is the public predicate that failed; private predicates surface as
    /// `RejectionClass::Opaque` in book-keeping (NOT here).
    AcceptancePredicateFailed(PredicateId),
    /// verify_transition stage 4 — verification predicate denied.
    VerificationPredicateFailed(PredicateId),
    /// finalize_reward / step_transition stage 5 — settlement predicate denied.
    SettlementPredicateFailed(PredicateId),

    // ── Challenge ──────────────────────────────────────────────────────────
    /// challenge_transition stage 1 — challenge filed after window closed.
    ChallengeWindowClosed,
    /// finalize_reward stage 1 — challenge window still open; cannot finalize.
    ChallengeWindowStillOpen,
    /// finalize_reward stage 1 — claim already slashed; cannot also reward.
    AlreadySlashed,
    /// challenge_transition stage 4 — counterexample failed predicate check.
    CounterexampleInsufficient,

    // ── Reuse ──────────────────────────────────────────────────────────────
    /// reuse_transition stage 1 — referenced tool not in L2 ToolRegistry.
    ToolNotInRegistry,
    /// reuse_transition stage 1 — declared tool creator does not match registry.
    ToolCreatorMismatch,

    // ── Finalize ───────────────────────────────────────────────────────────
    /// finalize_reward — no claim entry for the given claim_id.
    ClaimNotFound,
    /// TB-8 Atom 3 (charter §3 Atom 3 + ratification §1 Q2): finalize_reward
    /// idempotency — claim was already finalized by a prior accepted
    /// FinalizeRewardTx. Distinct from `AlreadySlashed` (which marks the
    /// adversarial-path terminal state); separate variants preserve the
    /// reward/slash discriminator that Phase 4 Information Loom needs. Maps
    /// to `L4ERejectionClass::PolicyViolation` per charter § 4.5.
    ClaimAlreadyFinalized,

    // ── Task expire ────────────────────────────────────────────────────────
    /// task_expire — referenced TaskMarket entry not found.
    TaskNotFound,
    /// task_expire — deadline not yet reached.
    TaskNotExpired,
    /// task_expire — at least one open claim exists; cannot refund bounty.
    TaskHasOpenClaim,

    // ── Terminal summary ───────────────────────────────────────────────────
    /// emit_terminal_summary — run already has an accepted work_tx.
    TerminalSummaryNotApplicable,

    // ── TB-2 RSP-1 admission (preflight v3 §3.7) ───────────────────────────
    /// WorkTx-arm escrow / task-market lookup miss. The bridged
    /// `TxId(tx.task_id.0.clone())` did not match any entry in
    /// `q.economic_state_t.escrows_t.0` or `task_markets_t.0`. Maps to
    /// `L4ERejectionClass::EscrowMissing` per the §3.7 mapping table.
    EscrowMissing,
    /// `monetary_invariant::assert_no_post_init_mint` or
    /// `assert_total_ctf_conserved` failed on the WorkTx arm. Maps to
    /// `L4ERejectionClass::InvariantViolation`.
    MonetaryInvariantViolation,

    // ── TB-3 RSP-1 formal-tx-surface (charter § 4.4) ───────────────────────
    /// `TaskOpenTx` admission idempotency: `task_markets_t` already
    /// contains an entry for this `task_id`. Maps to
    /// `L4ERejectionClass::PolicyViolation` per charter § 4.5.
    TaskAlreadyOpen,
    /// `EscrowLockTx` / `WorkTx` admission referenced a `task_id` not in
    /// `task_markets_t`. Maps to `L4ERejectionClass::EscrowMissing` per
    /// charter § 4.5 (semantic re-use: no open task = no funded admission).
    TaskNotOpen,
    /// `EscrowLockTx` sponsor or accepted-`WorkTx` solver lacks balance
    /// for the requested debit. Maps to `L4ERejectionClass::InsufficientBalance`
    /// (NEW class per charter § 4.5 — do NOT fold into `PolicyViolation`;
    /// P4 Information Loom needs this discriminator).
    InsufficientBalance,

    // ── TB-4 RSP-2 admission (charter § 3.8 + directive Q3) ────────────────
    /// `VerifyTx.bond` micro_units == 0. Distinct from `StakeInsufficient`
    /// (which is reused for ChallengeTx.stake==0 to keep WP economic § 7
    /// "Verifier 抵押 bond" naming honest). Maps to
    /// `L4ERejectionClass::PolicyViolation` per charter § 4.5.
    BondInsufficient,
    /// VerifyTx / ChallengeTx target_work_tx is not in `q.economic_state_t.
    /// stakes_t` — i.e., the target was never accepted as a live WorkTx,
    /// OR has been resolved/finalized in a future RSP-3 path. In TB-4
    /// minimum scope these two cases collapse since RSP-3 has not yet
    /// introduced finalize-removes-stakes_t logic. **Distinct from**
    /// `TargetWorkTxNotFound` (reserved for "tx_id has no L4 row at all"
    /// — unreachable in TB-4 since dispatch_transition reads Q_t only)
    /// and `TargetWorkTxNotVerifiable` (reserved for "target tx_id exists
    /// but is not a WorkTx type" — also unreachable in TB-4 since the
    /// stakes_t lookup keys by TxId without type checking; TB-3
    /// `lock-on-accept` only inserts stakes_t entries for accepted WorkTx).
    /// Maps to `L4ERejectionClass::PolicyViolation` per charter § 4.5.
    TargetWorkInactive,
    /// `ChallengeTx.counterexample_cid == Cid::ZERO`. Sanity gate against
    /// empty challenges — distinct from `MalformedPayload` (which would
    /// reject earlier at deserialize time) and from `PolicyViolation`
    /// catch-all. P4 Information Loom needs this discriminator per
    /// directive Q7. Maps to `L4ERejectionClass::PolicyViolation` per
    /// charter § 4.5.
    EmptyCounterexample,

    // ── TB-5.0 RSP-3.0 substrate (charter v2 § 4.9 + preflight § 3.5) ──────
    /// Agent attempted to submit a system-emitted variant
    /// (FinalizeRewardTx / TaskExpireTx / TerminalSummaryTx; ChallengeResolveTx
    /// added in TB-5 Atom 3) through the agent ingress path. The primary
    /// rejection happens at `Sequencer::submit_agent_tx` BEFORE dispatch
    /// (returns `SubmitError::SystemTxForbiddenOnAgentIngress` pre-queue).
    /// This `TransitionError` variant is the **defensive twin**: should
    /// any code path bypass the submit_agent_tx barrier and surface a
    /// system variant in `dispatch_transition`, this variant is the
    /// fail-closed dispatch response. Maps to
    /// `L4ERejectionClass::PolicyViolation` per charter § 4.5.
    /// Anti-Oreo enforcement of "agent ≠ direct state writer" at the
    /// constitutional level (Art V.1.3 + WP § 12.4).
    SystemTxForbiddenOnAgentIngress,
    /// TB-5 Atom 4 (charter v2 § 4.3 + preflight § 4.5): apply_one stage 1.5
    /// live signature verification failed. Fired when a system-emitted
    /// variant reaches apply_one with a `system_signature` that does NOT
    /// verify against the pinned PinnedSystemPubkeys for the current epoch.
    /// Defense-in-depth atop the constructive `Sequencer::emit_system_tx`
    /// guarantee — under normal operation this should be unreachable
    /// (emit_system_tx signs internally with the runtime's keypair, and
    /// pinned_pubkeys are derived from that same keypair). This variant
    /// fires only if some code path bypasses emit_system_tx and surfaces a
    /// forged-signature system variant in the queue. Maps to
    /// `L4ERejectionClass::PolicyViolation` per charter § 4.5.
    /// Per directive § 11.4: "system_signature 不能只是 schema 上的字段"
    /// — this dispatch-side guard ensures it is live-verified.
    InvalidSystemSignatureLive,
    /// TB-5 Atom 5 (charter v2 § 4.6 + preflight § 7.2): the resolution
    /// targets a `target_challenge_tx_id` that is NOT present in
    /// `economic_state_t.challenge_cases_t` at apply time. Maps to
    /// `L4ERejectionClass::PolicyViolation`.
    ChallengeNotFound,
    /// TB-5 Atom 5 (charter v2 § 4.6 + preflight § 7.2): the targeted
    /// `ChallengeCase` is already in a non-Open state (Released or
    /// UpheldDeferred). Idempotency gate — re-resolution of the same
    /// case is rejected. Maps to `L4ERejectionClass::PolicyViolation`.
    AlreadyResolved,

    // ── TB-13 Atom 2 (architect 2026-05-03 ruling Part A §4.4 FR-13.1..7) ──
    /// `CompleteSetMintTx` admission: `balances_t[owner] < amount`.
    /// Distinct from `InsufficientBalance` to give Information Loom a
    /// per-tx-class discriminator. Maps to `L4ERejectionClass::InsufficientBalance`.
    InsufficientBalanceForMint,
    /// `CompleteSetRedeemTx` admission: the referenced event is in
    /// `task_markets_t[event_id.0]` but its state is `Open` or `Expired`
    /// (i.e., neither `Finalized` for YES nor `Bankrupt` for NO). Architect
    /// FR-13.4 + SG-13.5: redeem unavailable before outcome resolution.
    /// Maps to `L4ERejectionClass::PolicyViolation`.
    RedeemBeforeResolution,
    /// `CompleteSetRedeemTx` admission: the owner's
    /// `conditional_share_balances_t[owner][event_id].{yes|no}` is less
    /// than the requested `share_amount.units`. Cannot redeem more than
    /// owned. Maps to `L4ERejectionClass::PolicyViolation`.
    RedeemMoreThanOwned,
    /// `MarketSeedTx` admission: `collateral_amount.micro_units() == 0`.
    /// Architect SG-13.4: market seed cannot create liquidity without
    /// collateral. Also fired defensively at `CompleteSetRedeemTx` time
    /// if `conditional_collateral_t[event_id]` lacks the redeemed amount
    /// (should never happen if `assert_complete_set_balanced` holds).
    /// Maps to `L4ERejectionClass::PolicyViolation`.
    InsufficientCollateral,
    /// `CompleteSetRedeemTx` admission: the redeem's `outcome` does not
    /// match the `task_markets_t[event_id.0]` state (e.g., outcome=Yes
    /// but state=Bankrupt, or outcome=No but state=Finalized). Architect
    /// §4.3 + FR-13.5: after-YES pays YES not NO. Maps to
    /// `L4ERejectionClass::PolicyViolation`.
    InvalidResolutionRef,
    /// TB-13 Atom 6 round-2 (Gemini CHALLENGE Q13 remediation 2026-05-03):
    /// `CompleteSetMintTx` / `MarketSeedTx` admission rejected because
    /// the target event's `task_markets_t[event_id.0].state` is not
    /// `Open` (Finalized / Bankrupt / Expired). Closes a griefing
    /// surface where an agent could mint shares against a closed
    /// event and immediately redeem winning side for full refund,
    /// leaving noise + stranded shares on-chain. Maps to
    /// `L4ERejectionClass::PolicyViolation`.
    EventNotOpen,

    // ── Stub sentinel (CO1.7.5 fills) ──────────────────────────────────────
    /// Stub return value used by CO1.7.5 unimplemented bodies — preserves
    /// sequencer + dispatch correctness without forcing transition logic
    /// into this atom. Audit input: this is intentional, not a code smell.
    NotYetImplemented,
}

impl std::fmt::Display for TransitionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StaleParent => write!(f, "stale parent_state_root"),
            Self::SignatureInvalid => write!(f, "agent signature invalid"),
            Self::InvalidSystemSignature => write!(f, "invalid system signature"),
            Self::StakeInsufficient => write!(f, "stake / bond insufficient"),
            Self::TargetWorkTxNotFound => write!(f, "target work_tx not found"),
            Self::TargetWorkTxNotVerifiable => write!(f, "target work_tx not in a verifiable state"),
            Self::ParentNotAcceptedYet => write!(f, "parent work_tx not yet accepted"),
            Self::AcceptancePredicateFailed(p) => write!(f, "acceptance predicate failed: {p:?}"),
            Self::VerificationPredicateFailed(p) => write!(f, "verification predicate failed: {p:?}"),
            Self::SettlementPredicateFailed(p) => write!(f, "settlement predicate failed: {p:?}"),
            Self::ChallengeWindowClosed => write!(f, "challenge window closed"),
            Self::ChallengeWindowStillOpen => write!(f, "challenge window still open"),
            Self::AlreadySlashed => write!(f, "already slashed"),
            Self::CounterexampleInsufficient => write!(f, "counterexample insufficient"),
            Self::ToolNotInRegistry => write!(f, "reuse tool not in registry"),
            Self::ToolCreatorMismatch => write!(f, "reuse tool creator mismatch"),
            Self::ClaimNotFound => write!(f, "claim not found"),
            Self::ClaimAlreadyFinalized => write!(
                f,
                "claim already finalized (idempotent re-finalize rejected)"
            ),
            Self::TaskNotFound => write!(f, "task not found"),
            Self::TaskNotExpired => write!(f, "task deadline not yet reached"),
            Self::TaskHasOpenClaim => write!(f, "task has at least one open claim"),
            Self::TerminalSummaryNotApplicable => write!(f, "terminal summary not applicable"),
            Self::EscrowMissing => write!(f, "escrow / task-market entry missing for task_id"),
            Self::MonetaryInvariantViolation => write!(f, "monetary invariant violation (post-init mint or ctf-conservation break)"),
            Self::TaskAlreadyOpen => write!(f, "task market already open for task_id"),
            Self::TaskNotOpen => write!(f, "no open task market for task_id"),
            Self::InsufficientBalance => write!(f, "balance below required debit amount"),
            Self::BondInsufficient => write!(f, "verifier bond insufficient"),
            Self::TargetWorkInactive => write!(f, "target work_tx not in stakes_t (never accepted live, or already resolved)"),
            Self::EmptyCounterexample => write!(f, "challenge counterexample_cid is empty / zero"),
            Self::SystemTxForbiddenOnAgentIngress => write!(
                f,
                "system-emitted tx variant forbidden on agent ingress \
                 (Anti-Oreo dispatch-side defensive guard; primary barrier \
                 is Sequencer::submit_agent_tx pre-queue)"
            ),
            Self::InvalidSystemSignatureLive => write!(
                f,
                "system_signature failed live verification against pinned \
                 PinnedSystemPubkeys for current epoch (apply_one stage 1.5 \
                 defense-in-depth; primary guarantee is emit_system_tx \
                 internal signing)"
            ),
            Self::ChallengeNotFound => write!(
                f,
                "ChallengeResolveTx target_challenge_tx_id not present in challenge_cases_t"
            ),
            Self::AlreadyResolved => write!(
                f,
                "ChallengeCase already resolved (status != Open); idempotent re-resolution rejected"
            ),
            Self::InsufficientBalanceForMint => write!(
                f,
                "CompleteSetMintTx: owner's balances_t entry is below the requested mint amount"
            ),
            Self::RedeemBeforeResolution => write!(
                f,
                "CompleteSetRedeemTx: event task_markets_t state is Open or Expired (no system-emitted resolution yet)"
            ),
            Self::RedeemMoreThanOwned => write!(
                f,
                "CompleteSetRedeemTx: owner's conditional share balance is below the requested redeem amount"
            ),
            Self::InsufficientCollateral => write!(
                f,
                "TB-13 collateral missing: MarketSeed with zero collateral, or Redeem against insufficient conditional_collateral_t"
            ),
            Self::InvalidResolutionRef => write!(
                f,
                "CompleteSetRedeemTx: outcome does not match task_markets_t[event_id.0] state"
            ),
            Self::EventNotOpen => write!(
                f,
                "TB-13 mint/seed: target event's task_markets_t state is not Open (Finalized/Bankrupt/Expired)"
            ),
            Self::NotYetImplemented => write!(f, "transition body not yet implemented (CO1.7.5)"),
        }
    }
}
impl std::error::Error for TransitionError {}

// ────────────────────────────────────────────────────────────────────────────
// SignalBundle — minimal v1 typed shape (CO1.7.5 + CO1.9 enrich it later)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX STATE § 3 — tape-emitted signal bundle. v1 minimal: a single
/// enum variant per spec call site in § 3 pseudocode (`empty` /
/// `finalize` / `task_expired` / `terminal_summary`). Full L6 signal-stream
/// design is CO1.9. CO1.1.4-pre1 ships just enough shape for CO1.7-impl to
/// compile and for CO1.7.5 transition bodies to construct each variant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SignalBundle {
    pub kind: SignalKind,
}

/// Discriminator over the spec § 3 pseudocode's `SignalBundle::*` constructors.
///
/// **v1.2 round-2 closure (R2-1)**: `Finalize.claim_id` is `ClaimId` (was `TxId`
/// in v1.1; round-2 caught the missed call site that leaked the old type
/// through `SignalBundle::finalize`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SignalKind {
    Empty,
    Finalize {
        claim_id: ClaimId,
        reward: MicroCoin,
    },
    TaskExpired {
        task_id: TaskId,
        bounty_refunded: MicroCoin,
    },
    TerminalSummary {
        run_id: RunId,
        outcome: RunOutcome,
    },
}

impl Default for SignalKind {
    fn default() -> Self {
        Self::Empty
    }
}

impl SignalBundle {
    pub fn empty() -> Self {
        Self {
            kind: SignalKind::Empty,
        }
    }
    pub fn finalize(claim_id: ClaimId, reward: MicroCoin) -> Self {
        Self {
            kind: SignalKind::Finalize { claim_id, reward },
        }
    }
    pub fn task_expired(task_id: TaskId, bounty_refunded: MicroCoin) -> Self {
        Self {
            kind: SignalKind::TaskExpired {
                task_id,
                bounty_refunded,
            },
        }
    }
    pub fn terminal_summary(run_id: RunId, outcome: RunOutcome) -> Self {
        Self {
            kind: SignalKind::TerminalSummary { run_id, outcome },
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Tests — round-trip (I-CANON-A/B/C) + golden fixtures (I-CANON-D)
// ────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bottom_white::ledger::transition_ledger::{canonical_decode, canonical_encode};
    use sha2::{Digest, Sha256};

    fn h(byte: u8) -> Hash {
        Hash([byte; 32])
    }
    fn cid(byte: u8) -> Cid {
        Cid([byte; 32])
    }

    /// Helper: canonical bytes → SHA-256 hex string. Used to lock golden
    /// fixtures: any future change to the wire format causes the digest hex
    /// to diverge → audit-required.
    fn digest_hex<T: Serialize>(value: &T) -> String {
        let bytes = canonical_encode(value).expect("encode");
        let hash = Sha256::digest(&bytes);
        hex_lower(&hash)
    }
    fn hex_lower(bytes: &[u8]) -> String {
        let mut s = String::with_capacity(bytes.len() * 2);
        for b in bytes {
            s.push_str(&format!("{:02x}", b));
        }
        s
    }

    // ── I-CANON-A/B/C — round-trip + byte-stability ──────────────────────────

    fn fixture_work_tx() -> WorkTx {
        let mut acceptance = BTreeMap::new();
        acceptance.insert(
            PredicateId("acc1".into()),
            BoolWithProof {
                value: true,
                proof_cid: Some(cid(0x11)),
            },
        );
        let mut settlement = BTreeMap::new();
        settlement.insert(
            PredicateId("set1".into()),
            BoolWithProof {
                value: true,
                proof_cid: None,
            },
        );
        WorkTx {
            tx_id: TxId("worktx-fixture-01".into()),
            task_id: TaskId("task-fixture-01".into()),
            parent_state_root: h(0x42),
            agent_id: AgentId("alice".into()),
            read_set: [ReadKey("k.read.a".into()), ReadKey("k.read.b".into())]
                .into_iter()
                .collect(),
            write_set: [WriteKey("k.write.a".into())].into_iter().collect(),
            proposal_cid: cid(0x13),
            predicate_results: PredicateResultsBundle {
                acceptance,
                settlement,
                safety_class: SafetyOrCreation::Safety,
            },
            stake: StakeMicroCoin::from_micro_units(1_000_000),
            signature: AgentSignature::from_bytes([0x77u8; 64]),
            timestamp_logical: 7,
        }
    }

    fn fixture_verify_tx() -> VerifyTx {
        VerifyTx {
            tx_id: TxId("verifytx-fixture-01".into()),
            parent_state_root: h(0x66), // TB-4 NEW
            target_work_tx: TxId("worktx-fixture-01".into()),
            verifier_agent: AgentId("bob".into()),
            bond: StakeMicroCoin::from_micro_units(500_000),
            verdict: VerifyVerdict::Confirm,
            signature: AgentSignature::from_bytes([0x55u8; 64]),
            timestamp_logical: 8,
        }
    }

    fn fixture_challenge_tx() -> ChallengeTx {
        ChallengeTx {
            tx_id: TxId("challengetx-fixture-01".into()),
            parent_state_root: h(0x77), // TB-4 NEW
            target_work_tx: TxId("worktx-fixture-01".into()),
            challenger_agent: AgentId("carol".into()),
            stake: StakeMicroCoin::from_micro_units(2_000_000),
            counterexample_cid: cid(0x21),
            signature: AgentSignature::from_bytes([0x33u8; 64]),
            timestamp_logical: 9,
        }
    }

    fn fixture_reuse_tx() -> ReuseTx {
        ReuseTx {
            tx_id: TxId("reusetx-fixture-01".into()),
            reusing_work_tx: TxId("worktx-fixture-02".into()),
            reused_tool_id: ToolId("tool-001".into()),
            reused_tool_creator: AgentId("alice".into()),
            timestamp_logical: 10,
        }
    }

    fn fixture_finalize_reward_tx() -> FinalizeRewardTx {
        FinalizeRewardTx {
            tx_id: TxId("finalizetx-fixture-01".into()),
            claim_id: ClaimId::new("claim-001"),
            task_id: TaskId("task-fixture-01".into()),
            solver: AgentId("alice".into()),
            reward: MicroCoin::from_micro_units(5_000_000),
            parent_state_root: h(0x43),
            epoch: SystemEpoch::new(1),
            timestamp_logical: 11,
            system_signature: SystemSignature::from_bytes([0xaau8; 64]),
        }
    }

    fn fixture_task_expire_tx() -> TaskExpireTx {
        // TB-11: extended with sponsor_agent + escrow_tx_id + reason
        TaskExpireTx {
            tx_id: TxId("expiretx-fixture-01".into()),
            task_id: TaskId("task-fixture-02".into()),
            parent_state_root: h(0x44),
            bounty_refunded: MicroCoin::from_micro_units(3_000_000),
            epoch: SystemEpoch::new(1),
            timestamp_logical: 12,
            sponsor_agent: AgentId("sponsor-tb11".into()),
            escrow_tx_id: TxId("escrowlock-fixture-tb11-01".into()),
            reason: ExpireReason::Deadline,
            system_signature: SystemSignature::from_bytes([0xbbu8; 64]),
        }
    }

    fn fixture_terminal_summary_tx() -> TerminalSummaryTx {
        // TB-11: extended with parent_state_root + solver_agent + evidence_capsule_cid
        let mut hist = BTreeMap::new();
        hist.insert(RejectionClass::SignatureInvalid, 2);
        hist.insert(RejectionClass::StakeInsufficient, 1);
        hist.insert(
            RejectionClass::AcceptancePredicateFail(PredicateId("acc1".into())),
            5,
        );
        TerminalSummaryTx {
            tx_id: TxId("terminalsummary-fixture-01".into()),
            task_id: TaskId("task-fixture-03".into()),
            run_id: RunId("run-001".into()),
            run_outcome: RunOutcome::MaxTxExhausted,
            total_attempts: 8,
            failure_class_histogram: hist,
            last_logical_t: 13,
            parent_state_root: h(0x55),
            solver_agent: Some(AgentId("Agent_solver_tb11".into())),
            evidence_capsule_cid: Some(cid(0x77)),
            system_signature: SystemSignature::from_bytes([0xccu8; 64]),
        }
    }

    fn fixture_task_bankruptcy_tx() -> TaskBankruptcyTx {
        // TB-11 NEW
        TaskBankruptcyTx {
            tx_id: TxId("bankruptcy-fixture-01".into()),
            parent_state_root: h(0x66),
            task_id: TaskId("task-fixture-04".into()),
            evidence_capsule_cid: cid(0x88),
            bankruptcy_reason: BankruptcyReason::MaxFailedRunCount,
            failed_run_count: 3,
            epoch: SystemEpoch::new(1),
            timestamp_logical: 14,
            system_signature: SystemSignature::from_bytes([0xddu8; 64]),
        }
    }

    /// Round-trip for every typed-tx variant.
    #[test]
    fn typed_tx_round_trip_all_variants() {
        let cases: Vec<TypedTx> = vec![
            TypedTx::Work(fixture_work_tx()),
            TypedTx::Verify(fixture_verify_tx()),
            TypedTx::Challenge(fixture_challenge_tx()),
            TypedTx::Reuse(fixture_reuse_tx()),
            TypedTx::FinalizeReward(fixture_finalize_reward_tx()),
            TypedTx::TaskExpire(fixture_task_expire_tx()),
            TypedTx::TerminalSummary(fixture_terminal_summary_tx()),
            // TB-11: TaskBankruptcy round-trip.
            TypedTx::TaskBankruptcy(fixture_task_bankruptcy_tx()),
        ];
        for tx in cases {
            let bytes = canonical_encode(&tx).expect("encode");
            let decoded: TypedTx = canonical_decode(&bytes).expect("decode");
            assert_eq!(tx, decoded, "round-trip mismatch on {:?}", tx.tx_kind());
        }
    }

    /// Two encodes of the same value produce byte-identical bytes.
    #[test]
    fn typed_tx_byte_stability_across_calls() {
        let tx = TypedTx::Work(fixture_work_tx());
        let bytes_a = canonical_encode(&tx).expect("encode a");
        let bytes_b = canonical_encode(&tx).expect("encode b");
        assert_eq!(bytes_a, bytes_b);
    }

    /// 100-input round-trip: random-ish AgentSignature bytes + variant choice.
    #[test]
    fn typed_tx_round_trip_100_inputs() {
        let mut tx = fixture_work_tx();
        for i in 0u32..100 {
            tx.timestamp_logical = i as u64;
            tx.signature = AgentSignature::from_bytes([(i % 256) as u8; 64]);
            let outer = TypedTx::Work(tx.clone());
            let bytes = canonical_encode(&outer).expect("encode");
            let back: TypedTx = canonical_decode(&bytes).expect("decode");
            assert_eq!(outer, back);
        }
    }

    /// HasSubmitter — agent-submitted vs system-emitted partitioning.
    #[test]
    fn has_submitter_partitioning() {
        let alice = AgentId("alice".into());
        assert_eq!(
            TypedTx::Work(fixture_work_tx()).submitter_id(),
            Some(alice.clone())
        );
        assert_eq!(
            TypedTx::Verify(fixture_verify_tx()).submitter_id(),
            Some(AgentId("bob".into()))
        );
        assert_eq!(
            TypedTx::Challenge(fixture_challenge_tx()).submitter_id(),
            Some(AgentId("carol".into()))
        );
        assert_eq!(TypedTx::Reuse(fixture_reuse_tx()).submitter_id(), None);
        assert_eq!(
            TypedTx::FinalizeReward(fixture_finalize_reward_tx()).submitter_id(),
            None
        );
        assert_eq!(
            TypedTx::TaskExpire(fixture_task_expire_tx()).submitter_id(),
            None
        );
        // TB-11: TaskBankruptcy is system-emitted; HasSubmitter → None.
        assert_eq!(
            TypedTx::TaskBankruptcy(fixture_task_bankruptcy_tx()).submitter_id(),
            None
        );
    }

    /// tx_kind matches the LedgerEntry TxKind enum variant.
    #[test]
    fn typed_tx_kind_projection() {
        use crate::bottom_white::ledger::transition_ledger::TxKind;
        assert_eq!(TypedTx::Work(fixture_work_tx()).tx_kind(), TxKind::Work);
        assert_eq!(
            TypedTx::Verify(fixture_verify_tx()).tx_kind(),
            TxKind::Verify
        );
        assert_eq!(
            TypedTx::Challenge(fixture_challenge_tx()).tx_kind(),
            TxKind::Challenge
        );
        assert_eq!(TypedTx::Reuse(fixture_reuse_tx()).tx_kind(), TxKind::Reuse);
        assert_eq!(
            TypedTx::FinalizeReward(fixture_finalize_reward_tx()).tx_kind(),
            TxKind::FinalizeReward
        );
        assert_eq!(
            TypedTx::TaskExpire(fixture_task_expire_tx()).tx_kind(),
            TxKind::TaskExpire
        );
        assert_eq!(
            TypedTx::TerminalSummary(fixture_terminal_summary_tx()).tx_kind(),
            TxKind::TerminalSummary,
        );
        // TB-11
        assert_eq!(
            TypedTx::TaskBankruptcy(fixture_task_bankruptcy_tx()).tx_kind(),
            TxKind::TaskBankruptcy,
        );
    }

    // ── v1.1 NEW: cross-variant non-collision (C-2 / Codex Q-J) ──────────────

    /// All 8 TypedTx variant fixtures encode to pairwise-distinct canonical bytes.
    /// (Different field shapes + bincode variant tags → ANY collision is a bincode
    /// regression that this test catches.)  TB-11 added TaskBankruptcy.
    #[test]
    fn typed_tx_cross_variant_non_collision() {
        let variants: Vec<(&str, TypedTx)> = vec![
            ("Work", TypedTx::Work(fixture_work_tx())),
            ("Verify", TypedTx::Verify(fixture_verify_tx())),
            ("Challenge", TypedTx::Challenge(fixture_challenge_tx())),
            ("Reuse", TypedTx::Reuse(fixture_reuse_tx())),
            (
                "FinalizeReward",
                TypedTx::FinalizeReward(fixture_finalize_reward_tx()),
            ),
            ("TaskExpire", TypedTx::TaskExpire(fixture_task_expire_tx())),
            (
                "TerminalSummary",
                TypedTx::TerminalSummary(fixture_terminal_summary_tx()),
            ),
            // TB-11 NEW
            (
                "TaskBankruptcy",
                TypedTx::TaskBankruptcy(fixture_task_bankruptcy_tx()),
            ),
        ];
        let digests: Vec<(&str, String)> = variants
            .iter()
            .map(|(name, tx)| (*name, digest_hex(tx)))
            .collect();
        for i in 0..digests.len() {
            for j in (i + 1)..digests.len() {
                assert_ne!(
                    digests[i].1, digests[j].1,
                    "{} and {} have colliding canonical digests",
                    digests[i].0, digests[j].0
                );
            }
        }
    }

    // ── v1.1 NEW: BTreeMap / BTreeSet permutation independence (C-2 / Gemini Q9) ─

    /// Building the same WorkTx via different `BTreeSet` insertion orders produces
    /// byte-identical canonical bytes. (BTreeSet iterates in sorted order, but
    /// this test locks that bincode honors the iteration order — defensive against
    /// a future codec choice that uses HashMap-style hash-randomized iteration.)
    #[test]
    fn typed_tx_btree_permutation_independence() {
        let make_work_tx = |read_keys_in_order: &[&str]| -> WorkTx {
            let mut tx = fixture_work_tx();
            tx.read_set = BTreeSet::new();
            for k in read_keys_in_order {
                tx.read_set.insert(ReadKey((*k).into()));
            }
            tx
        };
        // Insert keys in different orders.
        let tx_a = make_work_tx(&["k.read.a", "k.read.b", "k.read.c"]);
        let tx_b = make_work_tx(&["k.read.c", "k.read.a", "k.read.b"]);
        let tx_c = make_work_tx(&["k.read.b", "k.read.c", "k.read.a"]);
        let bytes_a = canonical_encode(&tx_a).expect("encode a");
        let bytes_b = canonical_encode(&tx_b).expect("encode b");
        let bytes_c = canonical_encode(&tx_c).expect("encode c");
        assert_eq!(bytes_a, bytes_b);
        assert_eq!(bytes_a, bytes_c);
    }

    // ── v1.1 NEW: zero-default round-trip per main tx kind (Gemini Q9) ──────

    #[test]
    fn typed_tx_default_round_trip() {
        let cases: Vec<TypedTx> = vec![
            TypedTx::Work(WorkTx::default()),
            TypedTx::Verify(VerifyTx::default()),
            TypedTx::Challenge(ChallengeTx::default()),
            TypedTx::Reuse(ReuseTx::default()),
            TypedTx::FinalizeReward(FinalizeRewardTx::default()),
            TypedTx::TaskExpire(TaskExpireTx::default()),
            TypedTx::TerminalSummary(TerminalSummaryTx::default()),
            // TB-11
            TypedTx::TaskBankruptcy(TaskBankruptcyTx::default()),
        ];
        for tx in cases {
            let bytes = canonical_encode(&tx).expect("encode default");
            let back: TypedTx = canonical_decode(&bytes).expect("decode default");
            assert_eq!(tx, back, "default round-trip mismatch on {:?}", tx.tx_kind());
        }
    }

    // ── v1.1 NEW: signing-payload domain-prefix non-collision (C-1) ─────────

    /// 6 signing-payload digests (Work / Verify / Challenge agent + Finalize /
    /// TaskExpire / TerminalSummary system) all have distinct domain prefixes;
    /// even if their bincode bodies COULD overlap, the SHA-256 inputs differ.
    /// We don't construct bodies that overlap (different fields); the assertion
    /// is simply that all 6 distinct domain-prefixed digests are pairwise distinct
    /// — which is the property auditors flagged as essential.
    #[test]
    fn signing_payload_domains_are_distinct() {
        let digests: Vec<(&str, [u8; 32])> = vec![
            ("Work", fixture_work_tx().to_signing_payload().canonical_digest()),
            (
                "Verify",
                fixture_verify_tx().to_signing_payload().canonical_digest(),
            ),
            (
                "Challenge",
                fixture_challenge_tx().to_signing_payload().canonical_digest(),
            ),
            (
                "FinalizeReward",
                fixture_finalize_reward_tx()
                    .to_signing_payload()
                    .canonical_digest(),
            ),
            (
                "TaskExpire",
                fixture_task_expire_tx()
                    .to_signing_payload()
                    .canonical_digest(),
            ),
            (
                "TerminalSummary",
                fixture_terminal_summary_tx()
                    .to_signing_payload()
                    .canonical_digest(),
            ),
        ];
        for i in 0..digests.len() {
            for j in (i + 1)..digests.len() {
                assert_ne!(
                    digests[i].1, digests[j].1,
                    "{} and {} signing-payload digests collide",
                    digests[i].0, digests[j].0
                );
            }
        }
    }

    /// Excluding the signature: mutating `tx.signature` must NOT change the
    /// signing-payload digest (the signature is its own input — a canonical
    /// digest cycle prevention property).
    #[test]
    fn signing_payload_excludes_signature() {
        // WorkTx (agent-signed)
        let tx_clean = fixture_work_tx();
        let d_clean = tx_clean.to_signing_payload().canonical_digest();
        let mut tx_mut = tx_clean.clone();
        tx_mut.signature = AgentSignature::from_bytes([0xff; 64]);
        let d_mut_sig = tx_mut.to_signing_payload().canonical_digest();
        assert_eq!(d_clean, d_mut_sig, "Work: mutating signature must NOT affect digest");

        // VerifyTx (agent-signed)
        let v_clean = fixture_verify_tx();
        let dv_clean = v_clean.to_signing_payload().canonical_digest();
        let mut v_mut = v_clean.clone();
        v_mut.signature = AgentSignature::from_bytes([0xee; 64]);
        assert_eq!(
            dv_clean,
            v_mut.to_signing_payload().canonical_digest(),
            "Verify: mutating signature must NOT affect digest"
        );

        // ChallengeTx (agent-signed)
        let c_clean = fixture_challenge_tx();
        let dc_clean = c_clean.to_signing_payload().canonical_digest();
        let mut c_mut = c_clean.clone();
        c_mut.signature = AgentSignature::from_bytes([0xdd; 64]);
        assert_eq!(
            dc_clean,
            c_mut.to_signing_payload().canonical_digest(),
            "Challenge: mutating signature must NOT affect digest"
        );

        // FinalizeRewardTx / TaskExpireTx / TerminalSummaryTx (system-signed)
        let f_clean = fixture_finalize_reward_tx();
        let df_clean = f_clean.to_signing_payload().canonical_digest();
        let mut f_mut = f_clean.clone();
        f_mut.system_signature = SystemSignature::from_bytes([0x11; 64]);
        assert_eq!(
            df_clean,
            f_mut.to_signing_payload().canonical_digest(),
            "FinalizeReward: mutating signature must NOT affect digest"
        );
        let t_clean = fixture_task_expire_tx();
        let dt_clean = t_clean.to_signing_payload().canonical_digest();
        let mut t_mut = t_clean.clone();
        t_mut.system_signature = SystemSignature::from_bytes([0x22; 64]);
        assert_eq!(
            dt_clean,
            t_mut.to_signing_payload().canonical_digest(),
            "TaskExpire: mutating signature must NOT affect digest"
        );
        let ts_clean = fixture_terminal_summary_tx();
        let dts_clean = ts_clean.to_signing_payload().canonical_digest();
        let mut ts_mut = ts_clean.clone();
        ts_mut.system_signature = SystemSignature::from_bytes([0x33; 64]);
        assert_eq!(
            dts_clean,
            ts_mut.to_signing_payload().canonical_digest(),
            "TerminalSummary: mutating signature must NOT affect digest"
        );
        // TB-11: TaskBankruptcyTx
        let bk_clean = fixture_task_bankruptcy_tx();
        let dbk_clean = bk_clean.to_signing_payload().canonical_digest();
        let mut bk_mut = bk_clean.clone();
        bk_mut.system_signature = SystemSignature::from_bytes([0x44; 64]);
        assert_eq!(
            dbk_clean,
            bk_mut.to_signing_payload().canonical_digest(),
            "TaskBankruptcy: mutating signature must NOT affect digest"
        );

        // Sanity: mutating a SIGNED field DOES change digest.
        let mut tx_signed_change = tx_clean.clone();
        tx_signed_change.timestamp_logical = 9999;
        let d_signed = tx_signed_change.to_signing_payload().canonical_digest();
        assert_ne!(d_clean, d_signed);
    }

    // ── TB-11 — TaskBankruptcy unit tests ────────────────────────────────

    /// TB-11 U1: TypedTx::TaskBankruptcy round-trips through canonical_encode.
    #[test]
    fn task_bankruptcy_round_trip() {
        let tx = TypedTx::TaskBankruptcy(fixture_task_bankruptcy_tx());
        let bytes = canonical_encode(&tx).expect("encode");
        let back: TypedTx = canonical_decode(&bytes).expect("decode");
        assert_eq!(tx, back);
    }

    /// TB-11 U2: TaskBankruptcySigningPayload digest is deterministic
    /// across calls + uses the domain-prefixed canonical hash.
    #[test]
    fn task_bankruptcy_canonical_digest_is_deterministic() {
        let a = fixture_task_bankruptcy_tx().to_signing_payload().canonical_digest();
        let b = fixture_task_bankruptcy_tx().to_signing_payload().canonical_digest();
        assert_eq!(a, b);
    }

    /// TB-11 U3: signing-payload field count = 8 (9 struct fields - 1 sig).
    #[test]
    fn task_bankruptcy_signing_payload_field_count_8() {
        let p = fixture_task_bankruptcy_tx().to_signing_payload();
        let v = serde_json::to_value(&p).unwrap();
        let obj = v.as_object().unwrap();
        assert_eq!(
            obj.len(),
            8,
            "TaskBankruptcySigningPayload must have 8 fields (system_signature excluded), got {}",
            obj.len()
        );
        assert!(!obj.contains_key("system_signature"));
        assert!(obj.contains_key("evidence_capsule_cid"));
        assert!(obj.contains_key("bankruptcy_reason"));
        assert!(obj.contains_key("failed_run_count"));
    }

    /// TB-11 U4: TerminalSummary additive bump preserves architect's
    /// `evidence_capsule_cid` field (architect §6.2 RunExhaustedTx schema).
    #[test]
    fn terminal_summary_carries_evidence_capsule_cid() {
        let ts = fixture_terminal_summary_tx();
        assert_eq!(ts.evidence_capsule_cid, Some(cid(0x77)));
        // Round-trip preserves Option<Cid> fidelity (Some <-> None discrimination).
        let mut ts_none = ts.clone();
        ts_none.evidence_capsule_cid = None;
        let d_some = ts.to_signing_payload().canonical_digest();
        let d_none = ts_none.to_signing_payload().canonical_digest();
        assert_ne!(d_some, d_none, "evidence_capsule_cid presence must affect canonical digest");
    }

    /// TB-11 U5: TaskExpire additive bump preserves architect's
    /// `sponsor_agent` + `escrow_tx_id` + `reason` fields (architect §6.2).
    #[test]
    fn task_expire_carries_sponsor_escrow_reason() {
        let te = fixture_task_expire_tx();
        assert_eq!(te.sponsor_agent, AgentId("sponsor-tb11".into()));
        assert_eq!(te.escrow_tx_id, TxId("escrowlock-fixture-tb11-01".into()));
        assert_eq!(te.reason, ExpireReason::Deadline);
        // Mutating reason MUST change canonical digest.
        let d_deadline = te.to_signing_payload().canonical_digest();
        let mut te_bk = te.clone();
        te_bk.reason = ExpireReason::BankruptcyTriggered;
        let d_bk = te_bk.to_signing_payload().canonical_digest();
        assert_ne!(d_deadline, d_bk);
    }

    /// TB-11 U6: ExhaustionReason → RunOutcome projection covers all 5 variants.
    #[test]
    fn exhaustion_reason_to_run_outcome() {
        assert_eq!(ExhaustionReason::MaxTxExhausted.to_run_outcome(), RunOutcome::MaxTxExhausted);
        assert_eq!(ExhaustionReason::WallClockCap.to_run_outcome(), RunOutcome::WallClockCap);
        assert_eq!(ExhaustionReason::ComputeCap.to_run_outcome(), RunOutcome::ComputeCap);
        assert_eq!(ExhaustionReason::ProtocolCollapse.to_run_outcome(), RunOutcome::ErrorHalt);
        assert_eq!(ExhaustionReason::SolverGiveUp.to_run_outcome(), RunOutcome::ErrorHalt);
    }

    // ── v1.2 NEW (R2-4 Codex round-2): LOAD-BEARING domain test ─────────────

    /// Hash the SAME body bytes with each of the 6 domain prefixes; assert all
    /// 6 results are pairwise distinct. Without the domain prefix, this test
    /// would FAIL — proving the prefix is load-bearing (the round-1 test
    /// `signing_payload_domains_are_distinct` used different bodies and
    /// would have passed even without domains).
    #[test]
    fn signing_payload_domain_prefix_is_load_bearing() {
        // Identical 64-byte body across all domains; the only thing that varies
        // is which domain prefix gets prepended before SHA-256.
        let body: Vec<u8> = (0..64u8).collect();
        let domains: &[&[u8]] = &[
            DOMAIN_AGENT_WORK,
            DOMAIN_AGENT_VERIFY,
            DOMAIN_AGENT_CHALLENGE,
            DOMAIN_SYSTEM_FINALIZE_REWARD,
            DOMAIN_SYSTEM_TASK_EXPIRE,
            DOMAIN_SYSTEM_TERMINAL_SUMMARY,
        ];
        let digests: Vec<[u8; 32]> = domains
            .iter()
            .map(|d| {
                let mut h = Sha256::new();
                h.update(d);
                h.update(&body);
                h.finalize().into()
            })
            .collect();
        for i in 0..digests.len() {
            for j in (i + 1)..digests.len() {
                assert_ne!(
                    digests[i], digests[j],
                    "domains {} and {} produced identical digests on identical body",
                    String::from_utf8_lossy(domains[i]),
                    String::from_utf8_lossy(domains[j])
                );
            }
        }
    }

    // ── v1.2 NEW (P15 Codex round-2 secondary): BTreeMap permutation ───────

    /// PredicateResultsBundle's `acceptance: BTreeMap<PredicateId, BoolWithProof>`
    /// must encode identically regardless of insertion order (matches the BTreeSet
    /// permutation test for read_set; closes round-2 caveat that BTreeMap
    /// fields weren't covered).
    #[test]
    fn typed_tx_btreemap_permutation_independence() {
        let make_work_tx = |insertion_order: &[(&str, bool)]| -> WorkTx {
            let mut tx = fixture_work_tx();
            tx.predicate_results.acceptance = BTreeMap::new();
            for (k, v) in insertion_order {
                tx.predicate_results.acceptance.insert(
                    PredicateId((*k).into()),
                    BoolWithProof {
                        value: *v,
                        proof_cid: None,
                    },
                );
            }
            tx
        };
        let tx_a = make_work_tx(&[("p_a", true), ("p_b", false), ("p_c", true)]);
        let tx_b = make_work_tx(&[("p_c", true), ("p_a", true), ("p_b", false)]);
        let tx_c = make_work_tx(&[("p_b", false), ("p_c", true), ("p_a", true)]);
        let bytes_a = canonical_encode(&tx_a).expect("encode a");
        let bytes_b = canonical_encode(&tx_b).expect("encode b");
        let bytes_c = canonical_encode(&tx_c).expect("encode c");
        assert_eq!(bytes_a, bytes_b);
        assert_eq!(bytes_a, bytes_c);
    }

    // ── v1.2 NEW (R2-4): signing-payload golden hex ────────────────────────

    fn signing_digest_hex(bytes: &[u8; 32]) -> String {
        hex_lower(bytes)
    }

    /// Lock SHA-256 hex of each signing-payload's canonical_digest. Any
    /// future codec / domain / projection change diffs one of these hex strings.
    /// Locked values captured 2026-04-28.
    #[test]
    fn signing_payload_golden_digests() {
        let tests: &[(&str, [u8; 32], &str)] = &[
            (
                "Work",
                fixture_work_tx().to_signing_payload().canonical_digest(),
                EXPECTED_SIGNING_HEX_WORK,
            ),
            (
                "Verify",
                fixture_verify_tx().to_signing_payload().canonical_digest(),
                EXPECTED_SIGNING_HEX_VERIFY,
            ),
            (
                "Challenge",
                fixture_challenge_tx().to_signing_payload().canonical_digest(),
                EXPECTED_SIGNING_HEX_CHALLENGE,
            ),
            (
                "FinalizeReward",
                fixture_finalize_reward_tx()
                    .to_signing_payload()
                    .canonical_digest(),
                EXPECTED_SIGNING_HEX_FINALIZE_REWARD,
            ),
            (
                "TaskExpire",
                fixture_task_expire_tx().to_signing_payload().canonical_digest(),
                EXPECTED_SIGNING_HEX_TASK_EXPIRE,
            ),
            (
                "TerminalSummary",
                fixture_terminal_summary_tx()
                    .to_signing_payload()
                    .canonical_digest(),
                EXPECTED_SIGNING_HEX_TERMINAL_SUMMARY,
            ),
        ];
        // Collect all mismatches before panicking — useful for capturing fresh
        // hex on first run (otherwise only the first failure prints).
        let mut mismatches: Vec<String> = Vec::new();
        for (name, actual, expected) in tests {
            let actual_hex = signing_digest_hex(actual);
            if &actual_hex != expected {
                mismatches.push(format!("{name}: actual={actual_hex} expected={expected}"));
            }
        }
        assert!(
            mismatches.is_empty(),
            "signing-payload digest mismatches:\n  {}",
            mismatches.join("\n  ")
        );
    }

    const EXPECTED_SIGNING_HEX_WORK: &str =
        "534d3cf26b7419a2741fa4eb2930b37095f982cc09c75ba2ee34396675a3d685";
    // TB-4 rotation: VerifyTx + ChallengeTx schema bump (parent_state_root
    // field#2; signing-payload field count 6→7).
    const EXPECTED_SIGNING_HEX_VERIFY: &str =
        "ac244cdbb9e26387df20c101718f40fc909b645b1b98c8627b472215ff5d8696";
    const EXPECTED_SIGNING_HEX_CHALLENGE: &str =
        "17c21ac8b6886e3d262925fa0942bc9a8e4e231a21e9767d0a25dd7c1ce2fbb5";
    const EXPECTED_SIGNING_HEX_FINALIZE_REWARD: &str =
        "74fd6bfb730b9d3e9828e4ebf8c3edb24aabb755813a058583949f08fbf5654b";
    /// TB-11 (architect §6.2 ruling 2026-05-02) — TaskExpireSigningPayload
    /// digest rotated due to additive schema bump: + sponsor_agent +
    /// escrow_tx_id + reason. Rotation protocol: golden-digest constant
    /// rotation requires explicit ABI commit + audit (TB-11 charter §6 G9).
    /// Pre-TB-11 hex was `d30fcf5fd45e32975e5547e266bcc4ef16353284205009d3feb4189e8b248def`.
    const EXPECTED_SIGNING_HEX_TASK_EXPIRE: &str =
        "05e47a7df499c7122ed18029304951ce7631123fbc39403264649c46b7615210";
    /// TB-11 (architect §6.2 ruling 2026-05-02) — TerminalSummarySigningPayload
    /// digest rotated due to additive schema bump: + parent_state_root +
    /// solver_agent + evidence_capsule_cid. Rotation protocol same as above.
    /// Pre-TB-11 hex was `71143e56cbd0fc3bdc4d8b764af9572564f8d66b2f4062d57d3678d4a311ac12`.
    const EXPECTED_SIGNING_HEX_TERMINAL_SUMMARY: &str =
        "ab9b0e82dbf007e76ddeb1312010df1f1fb0686b32f6f3098cb055e4d20617e7";

    // ── I-CANON-D — golden fixtures (locked SHA-256 of canonical bytes) ──────
    //
    // **v1.1 round-1 closure (C-2 / Codex Q-J / Gemini Q9)**: hex values are
    // hardcoded — any future codec / schema change causes the assertion to
    // fail, forcing a deliberate "ABI golden fixture rotation" commit with
    // re-audit. To rotate:
    //   1. Run `cargo test --lib state::typed_tx::tests::golden_` with current code
    //   2. The assertion failure messages report the new hex in the `actual` slot
    //   3. Update each `EXPECTED_HEX` constant + cite the rotation rationale in commit message

    const EXPECTED_HEX_WORK: &str =
        "6ec94fa4910ef4cc108ca8f36c202647d2cf60426d13ca0bccf777efb07b4fef";
    // TB-4 rotation: VerifyTx + ChallengeTx schema bump (parent_state_root
    // field#2; tx field count 7→8).
    const EXPECTED_HEX_VERIFY: &str =
        "287b3f501f99beaed77374f5ebc2c4df857f544500fdfa62e533d8bed4297b11";
    const EXPECTED_HEX_CHALLENGE: &str =
        "d91f933ca5703865bd6bc510615527710ad681d142ad57f681543217ffbbf596";
    const EXPECTED_HEX_REUSE: &str =
        "8bb33232b7c20a63a206f505179b0f64fa50acb41061aaa471ba8e4435593aed";
    const EXPECTED_HEX_FINALIZE_REWARD: &str =
        "0f5e213ec919f8e61dc998b13a4dcd49ff6e81e473850725f2ca1f27c1d65a2d";
    // TB-11 (architect §6.2 ruling 2026-05-02) — TaskExpireTx + TerminalSummaryTx
    // schema additive bumps; golden TypedTx digest rotation. Pre-TB-11 values:
    //   TaskExpire:      835cdec950a7fd09531e03b1ab2f571ccc9a7c05b3a3e04905f0dc77078c2d60
    //   TerminalSummary: f05983df19cb2af951d79216d71a64aae6b1ae960d036022f90f28039b059208
    const EXPECTED_HEX_TASK_EXPIRE: &str =
        "8d45f5dcff4e65c6dc680add961933a8fa99f07e02885e81b14ce8594b30b811";
    const EXPECTED_HEX_TERMINAL_SUMMARY: &str =
        "9e568384a5cf16268900e2ac66549dc11c9a16c1b37e2ac20ddba3e0a1794578";

    #[test]
    fn golden_work_tx_digest() {
        let actual = digest_hex(&TypedTx::Work(fixture_work_tx()));
        assert_eq!(actual.len(), 64);
        assert_eq!(actual, EXPECTED_HEX_WORK, "Work canonical digest changed");
    }

    #[test]
    fn golden_verify_tx_digest() {
        let actual = digest_hex(&TypedTx::Verify(fixture_verify_tx()));
        assert_eq!(actual, EXPECTED_HEX_VERIFY);
    }

    #[test]
    fn golden_challenge_tx_digest() {
        let actual = digest_hex(&TypedTx::Challenge(fixture_challenge_tx()));
        assert_eq!(actual, EXPECTED_HEX_CHALLENGE);
    }

    #[test]
    fn golden_reuse_tx_digest() {
        let actual = digest_hex(&TypedTx::Reuse(fixture_reuse_tx()));
        assert_eq!(actual, EXPECTED_HEX_REUSE);
    }

    #[test]
    fn golden_finalize_reward_tx_digest() {
        let actual = digest_hex(&TypedTx::FinalizeReward(fixture_finalize_reward_tx()));
        assert_eq!(actual, EXPECTED_HEX_FINALIZE_REWARD);
    }

    #[test]
    fn golden_task_expire_tx_digest() {
        let actual = digest_hex(&TypedTx::TaskExpire(fixture_task_expire_tx()));
        assert_eq!(actual, EXPECTED_HEX_TASK_EXPIRE);
    }

    #[test]
    fn golden_terminal_summary_tx_digest() {
        let actual = digest_hex(&TypedTx::TerminalSummary(fixture_terminal_summary_tx()));
        assert_eq!(actual, EXPECTED_HEX_TERMINAL_SUMMARY);
    }

    // ── TB-3 Atom 3 — TaskOpenTx + EscrowLockTx ABI surface tests ────────

    fn fixture_task_open_tx() -> TaskOpenTx {
        TaskOpenTx {
            tx_id: TxId("taskopen-fixture-01".into()),
            task_id: TaskId("task-fixture-01".into()),
            parent_state_root: h(0x33),
            sponsor_agent: AgentId("sponsor-alice".into()),
            verifier_quorum: 1,
            max_reuse_royalty_fraction_basis_points: 1000,
            settlement_rule_hash: h(0x44),
            signature: AgentSignature::from_bytes([0u8; 64]),
            timestamp_logical: 7,
        }
    }

    fn fixture_escrow_lock_tx() -> EscrowLockTx {
        EscrowLockTx {
            tx_id: TxId("escrowlock-fixture-01".into()),
            task_id: TaskId("task-fixture-01".into()),
            parent_state_root: h(0x55),
            sponsor_agent: AgentId("sponsor-alice".into()),
            amount: MicroCoin::from_coin(100).unwrap(),
            signature: AgentSignature::from_bytes([0u8; 64]),
            timestamp_logical: 8,
        }
    }

    /// T1 — TaskOpen canonical_digest is deterministic.
    #[test]
    fn task_open_tx_canonical_digest_is_deterministic() {
        let a = fixture_task_open_tx().to_signing_payload().canonical_digest();
        let b = fixture_task_open_tx().to_signing_payload().canonical_digest();
        assert_eq!(a, b, "canonical_digest must be deterministic");
    }

    /// T2 — EscrowLock canonical_digest is deterministic.
    #[test]
    fn escrow_lock_tx_canonical_digest_is_deterministic() {
        let a = fixture_escrow_lock_tx().to_signing_payload().canonical_digest();
        let b = fixture_escrow_lock_tx().to_signing_payload().canonical_digest();
        assert_eq!(a, b);
    }

    /// T3 — TaskOpenSigningPayload excludes the signature field.
    /// Verified by serde JSON shape: 9-field tx → 8-field payload.
    #[test]
    fn task_open_signing_payload_excludes_signature() {
        let p = fixture_task_open_tx().to_signing_payload();
        let v = serde_json::to_value(&p).unwrap();
        let obj = v.as_object().unwrap();
        assert_eq!(obj.len(), 8, "TaskOpenSigningPayload must have 8 fields (signature excluded), got {}", obj.len());
        assert!(!obj.contains_key("signature"));
    }

    /// T4 — EscrowLockSigningPayload excludes the signature field.
    /// 7-field tx → 6-field payload.
    #[test]
    fn escrow_lock_signing_payload_excludes_signature() {
        let p = fixture_escrow_lock_tx().to_signing_payload();
        let v = serde_json::to_value(&p).unwrap();
        let obj = v.as_object().unwrap();
        assert_eq!(obj.len(), 6, "EscrowLockSigningPayload must have 6 fields (signature excluded), got {}", obj.len());
        assert!(!obj.contains_key("signature"));
    }

    /// T5 — TransitionError Display covers the 3 new TB-3 variants with
    /// non-empty distinct strings (P4 Information Loom needs them
    /// human-readable + discriminable per charter § 4.4).
    #[test]
    fn transition_error_display_covers_3_new_variants() {
        let s_already = format!("{}", TransitionError::TaskAlreadyOpen);
        let s_not_open = format!("{}", TransitionError::TaskNotOpen);
        let s_no_balance = format!("{}", TransitionError::InsufficientBalance);
        assert!(!s_already.is_empty());
        assert!(!s_not_open.is_empty());
        assert!(!s_no_balance.is_empty());
        assert_ne!(s_already, s_not_open);
        assert_ne!(s_not_open, s_no_balance);
        assert_ne!(s_already, s_no_balance);
        assert!(s_already.contains("already"));
        assert!(s_not_open.contains("no open"));
        assert!(s_no_balance.contains("balance"));
    }

    // ── TB-4 Atom 2 — VerifyTx + ChallengeTx schema bump tests (T1-T4) ────

    /// T1 — VerifyTx canonical_digest includes parent_state_root.
    /// Two fixtures with different parent_state_root MUST produce different
    /// digests (proves the field is in the canonical-encoded bytes).
    #[test]
    fn verify_tx_canonical_digest_includes_parent_state_root() {
        let mut a = fixture_verify_tx();
        let mut b = fixture_verify_tx();
        a.parent_state_root = h(0xAA);
        b.parent_state_root = h(0xBB);
        let d_a = a.to_signing_payload().canonical_digest();
        let d_b = b.to_signing_payload().canonical_digest();
        assert_ne!(d_a, d_b, "parent_state_root must affect VerifyTx canonical digest");
    }

    /// T2 — ChallengeTx canonical_digest includes parent_state_root.
    #[test]
    fn challenge_tx_canonical_digest_includes_parent_state_root() {
        let mut a = fixture_challenge_tx();
        let mut b = fixture_challenge_tx();
        a.parent_state_root = h(0xCC);
        b.parent_state_root = h(0xDD);
        let d_a = a.to_signing_payload().canonical_digest();
        let d_b = b.to_signing_payload().canonical_digest();
        assert_ne!(d_a, d_b, "parent_state_root must affect ChallengeTx canonical digest");
    }

    /// T3 — VerifySigningPayload excludes the signature field.
    /// Verified by serde JSON shape: 8-field tx → 7-field payload.
    #[test]
    fn verify_signing_payload_excludes_signature_field_count_7() {
        let p = fixture_verify_tx().to_signing_payload();
        let v = serde_json::to_value(&p).unwrap();
        let obj = v.as_object().unwrap();
        assert_eq!(obj.len(), 7, "VerifySigningPayload must have 7 fields (signature excluded), got {}", obj.len());
        assert!(!obj.contains_key("signature"));
        assert!(obj.contains_key("parent_state_root"), "TB-4 parent_state_root field missing");
    }

    /// T4 — ChallengeSigningPayload excludes the signature field.
    /// 8-field tx → 7-field payload.
    #[test]
    fn challenge_signing_payload_excludes_signature_field_count_7() {
        let p = fixture_challenge_tx().to_signing_payload();
        let v = serde_json::to_value(&p).unwrap();
        let obj = v.as_object().unwrap();
        assert_eq!(obj.len(), 7, "ChallengeSigningPayload must have 7 fields (signature excluded), got {}", obj.len());
        assert!(!obj.contains_key("signature"));
        assert!(obj.contains_key("parent_state_root"), "TB-4 parent_state_root field missing");
    }

    /// T5 — TransitionError Display covers the 3 new TB-4 variants AND the
    /// 2 reserved-existing variants (TargetWorkTxNotFound +
    /// TargetWorkTxNotVerifiable) — establishing the directive's Q3 three-class
    /// taxonomy as fully addressable from Display strings (P4 Information
    /// Loom signal-quantization requirement per charter § 3.8).
    #[test]
    fn transition_error_display_covers_3_new_tb4_variants_plus_reserved() {
        let s_bond = format!("{}", TransitionError::BondInsufficient);
        let s_inactive = format!("{}", TransitionError::TargetWorkInactive);
        let s_empty = format!("{}", TransitionError::EmptyCounterexample);
        let s_not_found = format!("{}", TransitionError::TargetWorkTxNotFound);
        let s_not_verif = format!("{}", TransitionError::TargetWorkTxNotVerifiable);
        // Non-empty + distinct.
        assert!(!s_bond.is_empty());
        assert!(!s_inactive.is_empty());
        assert!(!s_empty.is_empty());
        assert!(!s_not_found.is_empty());
        assert!(!s_not_verif.is_empty());
        assert_ne!(s_bond, s_inactive);
        assert_ne!(s_inactive, s_empty);
        assert_ne!(s_bond, s_empty);
        // Three-class taxonomy (Q3 directive): TargetWorkInactive,
        // TargetWorkTxNotFound, TargetWorkTxNotVerifiable are distinct.
        assert_ne!(s_inactive, s_not_found);
        assert_ne!(s_inactive, s_not_verif);
        assert_ne!(s_not_found, s_not_verif);
        // Discriminable via keyword tokens.
        assert!(s_bond.contains("bond"));
        assert!(s_inactive.contains("stakes_t"));
        assert!(s_empty.contains("counterexample"));
        assert!(s_not_found.contains("not found"));
        assert!(s_not_verif.contains("verifiable"));
    }

    // ── TB-5 Atom 3 — ChallengeResolveTx ABI tests (T1-T4) ──────────────────

    fn fixture_challenge_resolve_tx() -> ChallengeResolveTx {
        ChallengeResolveTx {
            tx_id: TxId("crt-fixture-01".into()),
            parent_state_root: h(0x88),
            target_challenge_tx_id: TxId("challengetx-fixture-01".into()),
            resolution: ChallengeResolution::Released,
            epoch: SystemEpoch::new(1),
            timestamp_logical: 10,
            system_signature: SystemSignature::from_bytes([0x99u8; 64]),
        }
    }

    /// T1 — ChallengeResolveTx canonical_digest is deterministic.
    /// Two identical fixtures must produce the same digest.
    #[test]
    fn challenge_resolve_canonical_digest_is_deterministic() {
        let a = fixture_challenge_resolve_tx().to_signing_payload().canonical_digest();
        let b = fixture_challenge_resolve_tx().to_signing_payload().canonical_digest();
        assert_eq!(a, b, "canonical_digest must be deterministic");
    }

    /// T2 — ChallengeResolveSigningPayload excludes the signature field.
    /// 7-field tx → 6-field payload.
    #[test]
    fn challenge_resolve_signing_payload_excludes_signature_field_count_6() {
        let p = fixture_challenge_resolve_tx().to_signing_payload();
        let v = serde_json::to_value(&p).unwrap();
        let obj = v.as_object().unwrap();
        assert_eq!(obj.len(), 6,
            "ChallengeResolveSigningPayload must have 6 fields (signature excluded), got {}",
            obj.len());
        assert!(!obj.contains_key("system_signature"));
        assert!(obj.contains_key("target_challenge_tx_id"),
            "target_challenge_tx_id field must be in signing payload");
        assert!(obj.contains_key("resolution"),
            "resolution field must be in signing payload");
    }

    // TB-5 golden digest constants for ChallengeResolveTx (charter v2 § 4.5).
    // Computed first run; rotation rule: any future codec / domain / projection
    // change that affects these hex values requires explicit ABI golden fixture
    // rotation commit + re-audit (per typed_tx.rs:1684-1688 protocol).
    const EXPECTED_HEX_CHALLENGE_RESOLVE: &str =
        "f0372b8d767bd159c991f41919eb390331347758cba98a12ede064008d5027ae";
    const EXPECTED_SIGNING_HEX_CHALLENGE_RESOLVE: &str =
        "6e73496903a9e99effe6c2f1a1f540e83aa1c385135a61b680a5df01c878f04e";

    /// T3 — Golden TypedTx::ChallengeResolve digest hex is locked.
    /// Any future change to ChallengeResolveTx canonical-encoded bytes
    /// flips this hex → audit-required ABI golden fixture rotation.
    #[test]
    fn golden_challenge_resolve_tx_digest() {
        let actual = digest_hex(&TypedTx::ChallengeResolve(fixture_challenge_resolve_tx()));
        assert_eq!(actual.len(), 64);
        assert_eq!(actual, EXPECTED_HEX_CHALLENGE_RESOLVE,
            "ChallengeResolve TypedTx canonical digest changed");
    }

    /// T4 — Golden ChallengeResolveSigningPayload digest hex is locked.
    #[test]
    fn golden_challenge_resolve_signing_payload_digest() {
        let actual = signing_digest_hex(
            &fixture_challenge_resolve_tx().to_signing_payload().canonical_digest()
        );
        assert_eq!(actual, EXPECTED_SIGNING_HEX_CHALLENGE_RESOLVE);
    }

    // ──────────────────────────────────────────────────────────────────
    // TB-13 Atom 1 unit tests — CompleteSetMint / CompleteSetRedeem /
    // MarketSeed (architect 2026-05-03 post-TB-12 ruling Part A §4.3).
    // ──────────────────────────────────────────────────────────────────

    fn fixture_complete_set_mint_tx() -> CompleteSetMintTx {
        CompleteSetMintTx {
            tx_id: TxId("complete-set-mint-fixture-01".into()),
            parent_state_root: h(0x77),
            event_id: EventId(TaskId("task-fixture-tb13-mint".into())),
            owner: AgentId("agent-mint-fixture".into()),
            amount: MicroCoin::from_micro_units(7_000_000),
            signature: AgentSignature::from_bytes([0xddu8; 64]),
            timestamp_logical: 21,
        }
    }

    fn fixture_complete_set_redeem_tx() -> CompleteSetRedeemTx {
        CompleteSetRedeemTx {
            tx_id: TxId("complete-set-redeem-fixture-01".into()),
            parent_state_root: h(0x88),
            event_id: EventId(TaskId("task-fixture-tb13-redeem".into())),
            owner: AgentId("agent-redeem-fixture".into()),
            outcome: OutcomeSide::Yes,
            share_amount: ShareAmount::from_units(7_000_000),
            signature: AgentSignature::from_bytes([0xeeu8; 64]),
            timestamp_logical: 22,
        }
    }

    fn fixture_market_seed_tx() -> MarketSeedTx {
        MarketSeedTx {
            tx_id: TxId("market-seed-fixture-01".into()),
            parent_state_root: h(0x99),
            event_id: EventId(TaskId("task-fixture-tb13-seed".into())),
            provider: AgentId("agent-provider-fixture".into()),
            collateral_amount: MicroCoin::from_micro_units(2_500_000),
            signature: AgentSignature::from_bytes([0xffu8; 64]),
            timestamp_logical: 23,
        }
    }

    /// TB-13 U1: CompleteSetMintTx round-trips through canonical encode.
    #[test]
    fn tb_13_complete_set_mint_round_trips_canonical() {
        let tx = TypedTx::CompleteSetMint(fixture_complete_set_mint_tx());
        let bytes = canonical_encode(&tx).expect("encode");
        let decoded: TypedTx = canonical_decode(&bytes).expect("decode");
        assert_eq!(tx, decoded, "CompleteSetMintTx round-trip mismatch");
        assert_eq!(
            decoded.tx_kind(),
            crate::bottom_white::ledger::transition_ledger::TxKind::CompleteSetMint,
        );
    }

    /// TB-13 U2: CompleteSetRedeemTx round-trips through canonical encode.
    #[test]
    fn tb_13_complete_set_redeem_round_trips_canonical() {
        let tx = TypedTx::CompleteSetRedeem(fixture_complete_set_redeem_tx());
        let bytes = canonical_encode(&tx).expect("encode");
        let decoded: TypedTx = canonical_decode(&bytes).expect("decode");
        assert_eq!(tx, decoded, "CompleteSetRedeemTx round-trip mismatch");
        assert_eq!(
            decoded.tx_kind(),
            crate::bottom_white::ledger::transition_ledger::TxKind::CompleteSetRedeem,
        );
    }

    /// TB-13 U3: MarketSeedTx round-trips through canonical encode.
    #[test]
    fn tb_13_market_seed_round_trips_canonical() {
        let tx = TypedTx::MarketSeed(fixture_market_seed_tx());
        let bytes = canonical_encode(&tx).expect("encode");
        let decoded: TypedTx = canonical_decode(&bytes).expect("decode");
        assert_eq!(tx, decoded, "MarketSeedTx round-trip mismatch");
        assert_eq!(
            decoded.tx_kind(),
            crate::bottom_white::ledger::transition_ledger::TxKind::MarketSeed,
        );
    }

    /// TB-13 U4: OutcomeSide repr discriminants stable.
    #[test]
    fn tb_13_outcome_side_repr_u8_stable() {
        assert_eq!(OutcomeSide::Yes as u8, 0);
        assert_eq!(OutcomeSide::No as u8, 1);
    }

    /// TB-13 U5: ShareAmount default is zero.
    #[test]
    fn tb_13_share_amount_default_zero_units() {
        assert_eq!(ShareAmount::default(), ShareAmount::zero());
        assert_eq!(ShareAmount::default().units, 0u128);
    }

    /// TB-13 U6: deterministic canonical_digest — same payload twice yields
    /// the same digest. Architect §4.3 requires deterministic signing
    /// payloads (no environmental input).
    #[test]
    fn tb_13_signing_payloads_deterministic_digest() {
        let mint_a = fixture_complete_set_mint_tx().to_signing_payload().canonical_digest();
        let mint_b = fixture_complete_set_mint_tx().to_signing_payload().canonical_digest();
        assert_eq!(mint_a, mint_b, "CompleteSetMint digest must be deterministic");

        let redeem_a = fixture_complete_set_redeem_tx().to_signing_payload().canonical_digest();
        let redeem_b = fixture_complete_set_redeem_tx().to_signing_payload().canonical_digest();
        assert_eq!(redeem_a, redeem_b, "CompleteSetRedeem digest must be deterministic");

        let seed_a = fixture_market_seed_tx().to_signing_payload().canonical_digest();
        let seed_b = fixture_market_seed_tx().to_signing_payload().canonical_digest();
        assert_eq!(seed_a, seed_b, "MarketSeed digest must be deterministic");
    }

    /// TB-13 U7: signing payloads exclude the `signature` field — exact
    /// field count enforced (mint 6 / redeem 7 / seed 6).
    #[test]
    fn tb_13_signing_payloads_exclude_signature_field_counts() {
        let mint_p = fixture_complete_set_mint_tx().to_signing_payload();
        let mint_v = serde_json::to_value(&mint_p).unwrap();
        let mint_o = mint_v.as_object().unwrap();
        assert_eq!(mint_o.len(), 6, "CompleteSetMintSigningPayload must have 6 fields");
        assert!(!mint_o.contains_key("signature"));

        let redeem_p = fixture_complete_set_redeem_tx().to_signing_payload();
        let redeem_v = serde_json::to_value(&redeem_p).unwrap();
        let redeem_o = redeem_v.as_object().unwrap();
        assert_eq!(redeem_o.len(), 7, "CompleteSetRedeemSigningPayload must have 7 fields");
        assert!(!redeem_o.contains_key("signature"));

        let seed_p = fixture_market_seed_tx().to_signing_payload();
        let seed_v = serde_json::to_value(&seed_p).unwrap();
        let seed_o = seed_v.as_object().unwrap();
        assert_eq!(seed_o.len(), 6, "MarketSeedSigningPayload must have 6 fields");
        assert!(!seed_o.contains_key("signature"));
    }

    /// TB-13 U8: HasSubmitter projects to the wire owner / provider.
    #[test]
    fn tb_13_has_submitter_returns_owner_or_provider() {
        let mint = fixture_complete_set_mint_tx();
        assert_eq!(mint.submitter_id(), Some(mint.owner.clone()));

        let redeem = fixture_complete_set_redeem_tx();
        assert_eq!(redeem.submitter_id(), Some(redeem.owner.clone()));

        let seed = fixture_market_seed_tx();
        assert_eq!(seed.submitter_id(), Some(seed.provider.clone()));

        // TypedTx wrapper delegates to inner.
        assert_eq!(
            TypedTx::CompleteSetMint(fixture_complete_set_mint_tx()).submitter_id(),
            Some(AgentId("agent-mint-fixture".into())),
        );
        assert_eq!(
            TypedTx::MarketSeed(fixture_market_seed_tx()).submitter_id(),
            Some(AgentId("agent-provider-fixture".into())),
        );
    }
}
