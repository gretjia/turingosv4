//! TB-18R R1 — `AttemptTelemetry` + `LeanResult` + `TerminalAbortRecord` CAS object schemas.
//!
//! Per **TB-18R charter v2** (`handover/tracer_bullets/TB-18R_charter_2026-05-06.md`):
//! every externalized LLM-Lean cycle in evaluator.rs produces a CAS-resident
//! `AttemptTelemetry` object. Predicate-pass attempts route to L4 accepted
//! `WorkTx`; predicate-fail attempts route to L4.E rejection evidence (see R3).
//!
//! TB-18R closes the **failure-path asymmetry** documented in the 2026-05-06
//! external-audit VETO at
//! `handover/architect-insights/TB18_TAPE_NON_EXTERNALIZATION_VETO_2026-05-06.md`:
//! the omega_wtool success path was already externalized via
//! `bus.submit_typed_tx`, but `step_reject` / `parse_fail` / `llm_err` /
//! `step_partial_ok` failure paths leaked only to evaluator stdout / kernel.tape
//! shadow without L4 / L4.E entries. R1 ratifies the schema; R2 wires the
//! evaluator hot path; R3 extends sequencer L4.E admission.
//!
//! Distinction from `ProposalTelemetry` (TB-7 Atom 1.5): `ProposalTelemetry`
//! records WorkTx-level proposal metadata (token counts + tool calls). The
//! `WorkTx.proposal_cid` post-TB-18R points at an `AttemptTelemetry` CAS
//! object; the AttemptTelemetry's `proposal_telemetry_cid: Option<Cid>` field
//! references the existing ProposalTelemetry record so per-attempt records
//! and per-WorkTx records share the same evidentiary base.
//!
//! TRACE_MATRIX **FC1-N41** (NEW; charter v2 §FC1): per-LLM-Lean-cycle CAS
//! object — every externalized attempt produces a durable record with parsed
//! candidate payload + Lean verdict + outcome class.
//!
//! TRACE_MATRIX **FC1-N42** (NEW; charter v2 §FC1): runtime-path
//! attempt-to-L4-or-L4.E routing (target lives in R3 sequencer).
//!
//! TRACE_MATRIX **FC1-N43** (NEW; charter v2 §FC1): chain_derived_run_facts
//! exact-accounting equation (target lives in R4).
//!
//! ## Privacy invariant (CR-18R.4 v2; Codex Q3 ratified)
//!
//! `AttemptTelemetry.candidate_payload_cid` MUST point to **parsed external
//! candidate bytes**: the proof prefix / tactic body / Lean source actually
//! sent to Lean or used as the next external proof state. **NEVER** the raw
//! LLM response containing private chain-of-thought.
//!
//! Forbidden contents (mirroring `ProposalTelemetry::ToolCallRecord` precedent
//! at proposal_telemetry.rs:80-88):
//! - raw model deliberation
//! - raw tool transcripts
//! - internal reasoning
//! - raw prompt/completion strings
//! - hidden / structured-thinking blocks emitted by the model SDK
//!
//! `prompt_context_hash` is a 32-byte SHA-256 hash of the prompt context
//! supplied to the LLM; the prompt itself is NEVER stored. Raw prompt /
//! completion transcripts, if retained at all (e.g. for OBS-only audit-time
//! forensics), live in a separate AuditOnly CAS object distinct from
//! AttemptTelemetry.
//!
//! Tested by `tests/tb_18r_no_raw_response_in_attempt_payload.rs` (structural
//! fence) + R2-side enforcement at evaluator parse time.
//!
//! ## Storage shape
//!
//! - canonical-encoded (bincode v2 BE + fixed-int) for byte-stable CID
//! - put via `CasStore::put` with `ObjectType::AttemptTelemetry` /
//!   `ObjectType::LeanResult` / `ObjectType::TerminalAbortRecord` (R1 NEW)
//! - schema IDs: `turingosv4.attempt_telemetry.v1` /
//!   `turingosv4.lean_result.v1` / `turingosv4.terminal_abort_record.v1`
//! - retrievable via `read_attempt_telemetry_from_cas` etc. for replay /
//!   `verify_chaintape` extension / `audit_tape` sampler (R5)

use serde::{Deserialize, Serialize};

use crate::bottom_white::cas::schema::{Cid, ObjectType};
use crate::bottom_white::cas::store::{CasError, CasStore};
use crate::bottom_white::ledger::transition_ledger::{
    canonical_decode, canonical_encode, CanonicalCodecError,
};
use crate::runtime::proposal_telemetry::TokenCounts;
use crate::state::q_state::{AgentId, Hash, TxId};

// ── Schema IDs (charter v2 §6 binding) ──────────────────────────────────────

/// TRACE_MATRIX FC1-N41: schema id for `AttemptTelemetry` CAS objects.
pub const ATTEMPT_TELEMETRY_SCHEMA_ID: &str = "turingosv4.attempt_telemetry.v3";

/// TRACE_MATRIX FC1-N41: schema id for `LeanResult` CAS objects.
///
/// **Phase 2 (TB-18R G2 round-2 ruling 2026-05-06)**: bumped from `v1` to `v2`.
/// v2 adds the typed `verdict_kind: LeanVerdictKind` field as required, replacing
/// the v1 inferred-by-context multiplexing of `(verified, error_class, exit_code)`
/// onto three semantic states. v1 records (R6/R7 evidence) are byte-incompatible
/// with v2 readers; the architect-grandfathered evidence is preserved as-is per
/// `feedback_no_retroactive_evidence_rewrite` and is not decoded by v2 builds.
/// Phase 3 evidence is fresh on the v2 substrate.
pub const LEAN_RESULT_SCHEMA_ID: &str = "turingosv4.lean_result.v2";

/// TRACE_MATRIX FC1-N42: schema id for `TerminalAbortRecord` CAS objects.
/// Per FR-18R.3 v2 + Codex Q4 remediation: aborted attempts (externally
/// killed / mid-Lean SIGKILL / per-call budget halt) get an explicit
/// per-aborted-attempt record so the chain-derived equation
/// `evaluator_reported_completed_llm_calls == l4 + l4e` holds exactly
/// after the sequencer drain barrier.
pub const TERMINAL_ABORT_RECORD_SCHEMA_ID: &str = "turingosv4.terminal_abort_record.v1";

/// TRACE_MATRIX FC1-N41: schema version constant for `AttemptTelemetry`.
/// v2 = G4.2 actual model identity tail-add.
/// v3 = REAL-5 PromptCapsuleV2 CID linkage.
/// Historical v1/v2 bincode bytes are grandfathered by
/// `decode_attempt_telemetry_compat`.
pub const ATTEMPT_TELEMETRY_SCHEMA_VERSION: u32 = 3;

// ── AttemptKind (Codex Q5: separate from outcome; tail-extensible) ──────────

/// TRACE_MATRIX FC1-N41: kind discriminator for an Attempt.
///
/// Per `feedback_chaintape_externalized_proposal` (memory): "1 LLM call → 1
/// compound payload = 1 Attempt Node, NOT N tactic-level nodes". TB-18R uses
/// only `ExternalizedLlmCycle`. Tactic-level decomposition is reserved for
/// TB-8+; the `Tactic` and `ExternalToolCall` variants exist now to avoid a
/// later schema rewrite (per Codex Q5 ratified).
///
/// Stable byte-encoding via `#[repr(u8)]` so the discriminator rides into
/// the canonical hash deterministically across compiler versions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum AttemptKind {
    /// TB-18R primary: one externalized LLM call producing one parsed
    /// candidate payload sent to Lean (or used as the next external proof
    /// state). The TB-18R "1 LLM call = 1 Attempt Node" semantic.
    ExternalizedLlmCycle = 0,
    /// **Reserved for TB-8+**: per-tactic decomposition when the system
    /// actively makes per-tactic external tool calls. Not used in TB-18R.
    Tactic = 1,
    /// **Reserved for TB-8+**: external tool calls beyond Lean (e.g.,
    /// theorem search service, HTTP RAG endpoint, external SAT solver).
    /// Not used in TB-18R.
    ExternalToolCall = 2,
}

impl Default for AttemptKind {
    fn default() -> Self {
        Self::ExternalizedLlmCycle
    }
}

// ── AttemptOutcome (Codex Q5: separate from kind) ───────────────────────────

/// TRACE_MATRIX FC1-N41: outcome class for an Attempt.
///
/// Mirrors evaluator-side `tool_dist` increment paths at evaluator.rs lines
/// 2317 / 2861 (omega_wtool → LeanPass), 3263 (step_reject → LeanFail), 3275
/// (parse_fail → ParseFail), 3289 (llm_err → LlmErr), 3236 (step_partial_ok
/// → LeanPass with `proof_artifact_cid = None` if intermediate). `SorryBlock`
/// matches the existing `tb11_sorry_block_count` evaluator counter.
/// `Aborted` is for terminal-abort cases per FR-18R.3 v2 (Codex Q4
/// remediation): externally killed / mid-Lean SIGKILL / per-call budget halt.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum AttemptOutcome {
    /// Lean accepted the candidate (final composite proof OR intermediate
    /// partial-accept). Routes to L4 accepted WorkTx (R3 admission expansion).
    LeanPass = 0,
    /// Lean rejected the candidate (tactic failed, type error, unification
    /// failure, etc.). Routes to L4.E with `RejectionClass::LeanFailed=6`
    /// (R3 admission expansion).
    LeanFail = 1,
    /// Evaluator could not parse the model's external candidate from the
    /// LLM response. Routes to L4.E with `RejectionClass::ParseFailed=7`.
    ParseFail = 2,
    /// Candidate contained a forbidden `sorry` (or equivalent unsafe
    /// payload). Routes to L4.E with `RejectionClass::SorryBlocked=8`.
    SorryBlock = 3,
    /// LLM API call itself failed (network / rate-limit / provider error).
    /// Routes to L4.E with `RejectionClass::LlmError=9`.
    LlmErr = 4,
    /// Externally killed / mid-Lean SIGKILL / per-call budget halt /
    /// WallClockCap reached during this attempt's Lean check. Counted in
    /// `attempt_aborted_count` per FR-18R.3 v2; gets a separate
    /// `TerminalAbortRecord` CAS object. NOT counted in the
    /// `evaluator_reported_completed_llm_calls` invariant numerator.
    Aborted = 5,
    /// **TB-18R Phase 2 (2026-05-06; tail-additive)**: `step_partial_ok`
    /// intermediate Lean-accepted progress that is NOT omega-complete.
    /// Replaces the v1 misuse of `LeanPass` for `step_partial_ok` (FC-first
    /// analysis 2026-05-06 §2.5). Routes to CAS only per R3 §1.3 amended;
    /// no L4 and no L4.E entry. Mirrors `LeanVerdictKind::PartialAccepted`
    /// on the paired LeanResult.
    PartialAccepted = 6,
}

impl Default for AttemptOutcome {
    fn default() -> Self {
        Self::LeanPass
    }
}

// ── LeanErrorClass (mirror of R3 RejectionClass tail-append values) ─────────

/// TRACE_MATRIX FC1-N41: fine-grained Lean error classification.
///
/// Mirrors the values that R3 will tail-append to
/// `src/bottom_white/ledger/rejection_evidence.rs::RejectionClass` (existing
/// pre-TB-18R variants 0..5 unchanged: PredicateFailed=0, PolicyViolation=1,
/// EscrowMissing=2, InvariantViolation=3, MalformedPayload=4,
/// InsufficientBalance=5; per Codex Q8 source-grounded). R1 defines the
/// evaluator-side projection here; R3 wires `From<LeanErrorClass>` to
/// `RejectionClass` at the sequencer admission boundary.
///
/// Stable byte-encoding via `#[repr(u8)]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum LeanErrorClass {
    /// Lean tactic returned a failure verdict; type error / unification
    /// failure / undefined symbol / etc. R3 RejectionClass::LeanFailed=6.
    LeanFailed = 6,
    /// Evaluator could not parse a candidate from the model output (e.g.
    /// no recognizable lean code block, malformed wrapper). R3
    /// RejectionClass::ParseFailed=7.
    ParseFailed = 7,
    /// Candidate uses `sorry` or another forbidden incomplete-proof token.
    /// R3 RejectionClass::SorryBlocked=8.
    SorryBlocked = 8,
    /// LLM API itself errored (HTTP non-200, timeout, rate-limit, JSON
    /// parse fail on the LLM client side). R3 RejectionClass::LlmError=9.
    LlmError = 9,
}

// ── LeanVerdictKind (TB-18R Phase 2; typed PartialAccepted state) ───────────

/// TRACE_MATRIX FC1-N41 Phase 2 typed verdict classification.
///
/// Introduced 2026-05-06 by TB-18R G2 round-2 architect ruling §4 Q-P2 + §5
/// Phase 2. Replaces the v1 inferred-by-context multiplexing of
/// `(verified: bool, error_class: Option<LeanErrorClass>, exit_code: i32)` onto
/// three semantic states (Verified / Failed / Partial-or-Sorry). Phase 2 makes
/// the discriminator explicit so `assert_45` can typed-check each state arm
/// without falling back to error_class=None inference (which round-2 round-1
/// VETO surfaced as a semantic hole).
///
/// Stable byte-encoding via `#[repr(u8)]`. Tail-additive only (mirrors R3
/// `RejectionClass` pattern). Discriminant assignments are byte-stable for
/// canonical-hash purposes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum LeanVerdictKind {
    /// `exit_code == 0` AND `verified == true` AND `error_class == None`.
    /// Clean omega proof; `proof_artifact_cid` SHOULD be `Some(_)`.
    Verified = 0,
    /// `exit_code != 0` AND `verified == false` AND `error_class.is_some()`.
    /// Real Lean failure with a classified `LeanErrorClass`.
    Failed = 1,
    /// `exit_code == 0` AND `verified == false` AND `error_class == None`.
    /// Intermediate `step_partial_ok` Lean-accepted progress that is NOT
    /// omega-complete. `proof_artifact_cid` SHOULD be `None`. Per R3 §1.3
    /// amended, this state stays CAS-only (no L4 / no L4.E entry).
    PartialAccepted = 2,
    /// `exit_code == 0` AND `verified == false` AND
    /// `error_class == Some(LeanErrorClass::SorryBlocked)`.
    /// `sorry` / forbidden-payload classified.
    SorryBlocked = 3,
}

impl Default for LeanVerdictKind {
    fn default() -> Self {
        // Failed is the safest fallback: false-positive on a partial-accept
        // record will FAIL assert_45 (visible defect), false-negative on a
        // real failure would silently swallow a defect. The default should
        // never fire in practice — every emitter sets the kind explicitly.
        Self::Failed
    }
}

// ── AttemptTelemetry (the primary CAS object) ───────────────────────────────

/// TRACE_MATRIX FC1-N41: per-externalized-LLM-Lean-cycle telemetry record.
///
/// **Privacy invariant (CR-18R.4 v2)**: `candidate_payload_cid` points at
/// parsed external candidate bytes only; raw LLM response is NEVER stored.
/// See module-level doc-comment for the full FORBIDDEN list. R2 evaluator
/// hot path is responsible for parse-then-store; R5 audit_tape extension
/// includes a structural fence test to catch raw-response-shaped payloads.
///
/// **Field-set ordering (binding for canonical encode)**: do NOT reorder
/// existing fields without bumping `schema_version`. Tail-additive fields
/// (with `#[serde(default)]`) are forward-compat at v1.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttemptTelemetry {
    /// Schema version. Bump when adding non-defaulted fields or changing
    /// serialization shape. v1 = TB-18R initial.
    pub schema_version: u32,
    /// Stable Tx identifier for this attempt. Same TxId used on the
    /// `WorkTx.tx_id` if the attempt routes to L4 accepted, or
    /// `RejectedSubmissionRecord.submit_id` if it routes to L4.E.
    pub attempt_id: TxId,
    /// Run identifier (from `RuntimeChaintapeConfig::run_id`).
    pub run_id: String,
    /// Task identifier — the MiniF2F problem / Lean task being attempted.
    pub task_id: String,
    /// Agent that produced this attempt. Single-agent runs use a sandbox
    /// prefix (per TB-16 §6.1 sandbox-prefixed agents).
    pub agent_id: AgentId,
    /// Branch label within the agent's attempt tree (e.g. `"n1.b0"`).
    pub branch_id: String,
    /// `TxId` of the parent attempt on the same branch. `None` for the
    /// root attempt of a (run, agent, branch) tuple.
    pub parent_attempt_tx: Option<TxId>,
    /// Monotonic per-attempt index within (run, agent, branch). Starts at 0.
    pub attempt_index: u64,
    /// 32-byte SHA-256 of the prompt context. The prompt itself is NEVER
    /// stored here; this hash is the canonical reference. Forbidden
    /// contents per module-level FORBIDDEN list.
    pub prompt_context_hash: Hash,
    /// CID of the parsed external candidate payload. Privacy-invariant:
    /// parsed bytes only, NEVER raw LLM response. R2 enforces; R5 fences.
    pub candidate_payload_cid: Cid,
    /// CID of the `LeanResult` CAS object capturing Lean's verdict on
    /// this candidate. `None` if Lean was not invoked (parse fail / LLM
    /// error before Lean) or attempt was aborted before Lean returned.
    pub lean_result_cid: Option<Cid>,
    /// Kind discriminator. TB-18R uses `ExternalizedLlmCycle` exclusively.
    pub attempt_kind: AttemptKind,
    /// Outcome class. Separate from `attempt_kind` per Codex Q5 ratified.
    pub outcome: AttemptOutcome,
    /// Token usage for this attempt (mirrors `ProposalTelemetry::token_counts`).
    pub token_counts: TokenCounts,
    /// Tool name used to drive this attempt (e.g. `"omega_wtool"`,
    /// `"step"`, `"step_partial_ok"`, `"step_reject"`, `"parse_fail"`,
    /// `"llm_err"`). Mirrors evaluator `tool_dist` keys at evaluator.rs.
    pub tool_name: String,
    /// CID of the matching `ProposalTelemetry` record (TB-7 Atom 1.5
    /// schema). Set when this attempt's WorkTx also has a ProposalTelemetry
    /// reference; allows per-attempt + per-WorkTx records to share the
    /// same evidentiary base. `None` for attempts that bypass the
    /// ProposalTelemetry path (e.g. parse_fail with no submitted WorkTx).
    #[serde(default)]
    pub proposal_telemetry_cid: Option<Cid>,
    /// CID of the matching `VerificationResult` record (TB-7.7 D4 schema).
    /// Set when Lean was invoked and the verdict was recorded as a separate
    /// VerificationResult CAS object. `None` if attempt was aborted before
    /// VerificationResult emission. Distinct from `lean_result_cid` only
    /// in that VerificationResult is the TB-7.7 schema and LeanResult is
    /// the TB-18R schema; both may coexist during R2 transition.
    #[serde(default)]
    pub verification_result_cid: Option<Cid>,
    /// **Some only on the final composite proof attempt** (the OMEGA-accept
    /// terminal attempt). Merkle root over the constituent attempt_ids
    /// that contributed to the composite proof. `None` for all
    /// intermediate / failed / aborted attempts. Per Codex Q8 ratified:
    /// `attempt_chain_root` payload schema lives on `AttemptTelemetry`,
    /// NOT on `WorkTx` — preserves WorkTx canonical wire bytes (see
    /// preflight `handover/ai-direct/TB-18R_R1_STEP_B_schema.md` §3
    /// Design B).
    #[serde(default)]
    pub attempt_chain_root: Option<Hash>,
    /// G4.2 actual model identity observed for this LLM attempt.
    #[serde(default)]
    pub model_name: Option<String>,
    #[serde(default)]
    pub model_family: Option<String>,
    #[serde(default)]
    pub model_provider: Option<String>,
    #[serde(default)]
    pub model_version: Option<String>,
    #[serde(default)]
    pub temperature_milli: Option<i64>,
    /// REAL-5 PromptCapsuleV2 linkage for role-scoped, replayable views.
    /// Historical v1/v2 records decode with `None`; new REAL-5 externalized
    /// attempts set this to the CAS CID of the role/view PromptCapsule.
    #[serde(default)]
    pub prompt_capsule_cid: Option<Cid>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct AttemptTelemetryV1Wire {
    schema_version: u32,
    attempt_id: TxId,
    run_id: String,
    task_id: String,
    agent_id: AgentId,
    branch_id: String,
    parent_attempt_tx: Option<TxId>,
    attempt_index: u64,
    prompt_context_hash: Hash,
    candidate_payload_cid: Cid,
    lean_result_cid: Option<Cid>,
    attempt_kind: AttemptKind,
    outcome: AttemptOutcome,
    token_counts: TokenCounts,
    tool_name: String,
    #[serde(default)]
    proposal_telemetry_cid: Option<Cid>,
    #[serde(default)]
    verification_result_cid: Option<Cid>,
    #[serde(default)]
    attempt_chain_root: Option<Hash>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct AttemptTelemetryV2Wire {
    schema_version: u32,
    attempt_id: TxId,
    run_id: String,
    task_id: String,
    agent_id: AgentId,
    branch_id: String,
    parent_attempt_tx: Option<TxId>,
    attempt_index: u64,
    prompt_context_hash: Hash,
    candidate_payload_cid: Cid,
    lean_result_cid: Option<Cid>,
    attempt_kind: AttemptKind,
    outcome: AttemptOutcome,
    token_counts: TokenCounts,
    tool_name: String,
    #[serde(default)]
    proposal_telemetry_cid: Option<Cid>,
    #[serde(default)]
    verification_result_cid: Option<Cid>,
    #[serde(default)]
    attempt_chain_root: Option<Hash>,
    #[serde(default)]
    model_name: Option<String>,
    #[serde(default)]
    model_family: Option<String>,
    #[serde(default)]
    model_provider: Option<String>,
    #[serde(default)]
    model_version: Option<String>,
    #[serde(default)]
    temperature_milli: Option<i64>,
}

impl From<AttemptTelemetryV1Wire> for AttemptTelemetry {
    fn from(v1: AttemptTelemetryV1Wire) -> Self {
        Self {
            schema_version: v1.schema_version,
            attempt_id: v1.attempt_id,
            run_id: v1.run_id,
            task_id: v1.task_id,
            agent_id: v1.agent_id,
            branch_id: v1.branch_id,
            parent_attempt_tx: v1.parent_attempt_tx,
            attempt_index: v1.attempt_index,
            prompt_context_hash: v1.prompt_context_hash,
            candidate_payload_cid: v1.candidate_payload_cid,
            lean_result_cid: v1.lean_result_cid,
            attempt_kind: v1.attempt_kind,
            outcome: v1.outcome,
            token_counts: v1.token_counts,
            tool_name: v1.tool_name,
            proposal_telemetry_cid: v1.proposal_telemetry_cid,
            verification_result_cid: v1.verification_result_cid,
            attempt_chain_root: v1.attempt_chain_root,
            model_name: None,
            model_family: None,
            model_provider: None,
            model_version: None,
            temperature_milli: None,
            prompt_capsule_cid: None,
        }
    }
}

impl From<AttemptTelemetryV2Wire> for AttemptTelemetry {
    fn from(v2: AttemptTelemetryV2Wire) -> Self {
        Self {
            schema_version: v2.schema_version,
            attempt_id: v2.attempt_id,
            run_id: v2.run_id,
            task_id: v2.task_id,
            agent_id: v2.agent_id,
            branch_id: v2.branch_id,
            parent_attempt_tx: v2.parent_attempt_tx,
            attempt_index: v2.attempt_index,
            prompt_context_hash: v2.prompt_context_hash,
            candidate_payload_cid: v2.candidate_payload_cid,
            lean_result_cid: v2.lean_result_cid,
            attempt_kind: v2.attempt_kind,
            outcome: v2.outcome,
            token_counts: v2.token_counts,
            tool_name: v2.tool_name,
            proposal_telemetry_cid: v2.proposal_telemetry_cid,
            verification_result_cid: v2.verification_result_cid,
            attempt_chain_root: v2.attempt_chain_root,
            model_name: v2.model_name,
            model_family: v2.model_family,
            model_provider: v2.model_provider,
            model_version: v2.model_version,
            temperature_milli: v2.temperature_milli,
            prompt_capsule_cid: None,
        }
    }
}

impl AttemptTelemetry {
    /// TRACE_MATRIX FC1-N41: convenience constructor for the common case
    /// where the attempt has no parent (root attempt of (run, agent, branch)).
    /// Used by R2 evaluator hooks at the per-iteration loop boundary.
    #[allow(clippy::too_many_arguments)]
    pub fn new_root(
        attempt_id: TxId,
        run_id: String,
        task_id: String,
        agent_id: AgentId,
        branch_id: String,
        prompt_context_hash: Hash,
        candidate_payload_cid: Cid,
        attempt_kind: AttemptKind,
        outcome: AttemptOutcome,
        token_counts: TokenCounts,
        tool_name: String,
    ) -> Self {
        Self {
            schema_version: ATTEMPT_TELEMETRY_SCHEMA_VERSION,
            attempt_id,
            run_id,
            task_id,
            agent_id,
            branch_id,
            parent_attempt_tx: None,
            attempt_index: 0,
            prompt_context_hash,
            candidate_payload_cid,
            lean_result_cid: None,
            attempt_kind,
            outcome,
            token_counts,
            tool_name,
            proposal_telemetry_cid: None,
            verification_result_cid: None,
            attempt_chain_root: None,
            model_name: None,
            model_family: None,
            model_provider: None,
            model_version: None,
            temperature_milli: None,
            prompt_capsule_cid: None,
        }
    }

    /// TRACE_MATRIX FC1-N41: terminal-attempt constructor — the OMEGA-accept
    /// final composite proof. Carries `attempt_chain_root: Some(merkle_root)`
    /// over the constituent attempt_ids that contributed to the composite.
    /// Per Codex Q8 ratified.
    #[allow(clippy::too_many_arguments)]
    pub fn new_terminal_composite(
        attempt_id: TxId,
        run_id: String,
        task_id: String,
        agent_id: AgentId,
        branch_id: String,
        parent_attempt_tx: Option<TxId>,
        attempt_index: u64,
        prompt_context_hash: Hash,
        candidate_payload_cid: Cid,
        token_counts: TokenCounts,
        tool_name: String,
        attempt_chain_root: Hash,
    ) -> Self {
        Self {
            schema_version: ATTEMPT_TELEMETRY_SCHEMA_VERSION,
            attempt_id,
            run_id,
            task_id,
            agent_id,
            branch_id,
            parent_attempt_tx,
            attempt_index,
            prompt_context_hash,
            candidate_payload_cid,
            lean_result_cid: None,
            attempt_kind: AttemptKind::ExternalizedLlmCycle,
            outcome: AttemptOutcome::LeanPass,
            token_counts,
            tool_name,
            proposal_telemetry_cid: None,
            verification_result_cid: None,
            attempt_chain_root: Some(attempt_chain_root),
            model_name: None,
            model_family: None,
            model_provider: None,
            model_version: None,
            temperature_milli: None,
            prompt_capsule_cid: None,
        }
    }

    /// TRACE_MATRIX FC1 AttemptTelemetry: attach provider-reported actual model identity to an attempt.
    pub fn with_actual_model_identity(
        mut self,
        model_name: impl Into<String>,
        model_family: impl Into<String>,
        model_provider: impl Into<String>,
        model_version: Option<String>,
        temperature_milli: i64,
    ) -> Self {
        self.model_name = Some(model_name.into());
        self.model_family = Some(model_family.into());
        self.model_provider = Some(model_provider.into());
        self.model_version = model_version;
        self.temperature_milli = Some(temperature_milli);
        self
    }

    /// TRACE_MATRIX FC1 + REAL-5 Atom 3: attach the CAS CID of the
    /// role/view PromptCapsule used to construct this externalized attempt.
    pub fn with_prompt_capsule_cid(mut self, prompt_capsule_cid: Cid) -> Self {
        self.prompt_capsule_cid = Some(prompt_capsule_cid);
        self
    }
}

// ── LeanResult (CAS-resident Lean verdict) ──────────────────────────────────

/// TRACE_MATRIX FC1-N41: Lean verdict on a single externalized candidate.
///
/// **Schema version**: `v2` post-TB-18R Phase 2 (2026-05-06). v1 records
/// (R6/R7 grandfathered evidence) are byte-incompatible with v2 readers per
/// `feedback_no_retroactive_evidence_rewrite`; v2 chains carry `verdict_kind`
/// as a required field that types the verdict classification explicitly.
///
/// Privacy: `stderr_cid` and `stdout_cid` are CIDs to AuditOnly CAS objects
/// (per Art.III.1 + TB-15 `CapsulePrivacyPolicy`); raw stderr / stdout bytes
/// stay shielded behind those CIDs and are NOT broadcast in the public
/// `attempt_kind` / `outcome` fields. The `error_class` field carries the
/// low-pollution rejection-class label for `public_summary` use.
///
/// **Phase 2 typed invariant** (enforced by `assert_45`):
/// the `verdict_kind` discriminates the four legitimate (exit_code, verified,
/// error_class) shapes; `assert_45` performs an exact 4-arm match. The
/// legacy `verified: bool` and `error_class: Option<LeanErrorClass>` fields
/// remain for downstream consumers (`pput_verified`,
/// `ChainDerivedRunFacts`) and an `assert_45` consistency clause prevents
/// drift between the typed kind and the redundant legacy fields.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LeanResult {
    /// Reference back to the `AttemptTelemetry.attempt_id` this verdict
    /// belongs to.
    pub attempt_id: TxId,
    /// Lean process exit code. 0 means tactic succeeded; non-zero means
    /// failure (specific class in `error_class`).
    pub exit_code: i32,
    /// True iff Lean fully verified the candidate (no errors, no `sorry`).
    /// False if exit_code != 0 OR sorry was used OR partial verdict.
    /// **Phase 2**: redundant with `verdict_kind`; downstream consumers
    /// (`pput_verified`, `ChainDerivedRunFacts`) read this field. The
    /// `assert_45` consistency clause prevents drift.
    pub verified: bool,
    /// CID of the AuditOnly CAS object holding raw Lean stderr bytes.
    /// `None` if Lean produced no stderr (rare; clean omega_wtool path).
    pub stderr_cid: Option<Cid>,
    /// CID of the AuditOnly CAS object holding raw Lean stdout bytes.
    /// `None` if Lean produced no stdout.
    pub stdout_cid: Option<Cid>,
    /// CID of the proof artifact (Lean source produced by the candidate)
    /// when verified successfully. `None` for failed / aborted attempts /
    /// partial-accepted attempts.
    pub proof_artifact_cid: Option<Cid>,
    /// Fine-grained error class. Phase 2: redundant with `verdict_kind`;
    /// kept for backward compat with downstream rejection-class consumers.
    /// Canonical shapes (matched by `assert_45` consistency clause):
    /// - `Verified`        → `error_class == None`
    /// - `Failed`          → `error_class.is_some()`
    /// - `PartialAccepted` → `error_class == None`
    /// - `SorryBlocked`    → `error_class == Some(SorryBlocked)`
    pub error_class: Option<LeanErrorClass>,
    /// **TB-18R Phase 2 (2026-05-06; LeanResult schema v2)**: typed verdict
    /// classification. REQUIRED field; v2 byte-format includes this
    /// discriminator at the canonical-encoded tail. Pre-v2 records do not
    /// include this byte; they will fail decode under v2 builds (acceptable
    /// per architect-grandfathered evidence policy — R6/R7 evidence is not
    /// re-decoded by v2; Phase 3 evidence is fresh on the v2 substrate).
    pub verdict_kind: LeanVerdictKind,
}

impl LeanResult {
    /// TRACE_MATRIX FC1-N41 Phase 2: derive the canonical `LeanVerdictKind`
    /// from the legacy fields `(verified, error_class.is_some(),
    /// exit_code != 0)`. Used by `assert_45` consistency clause and by
    /// emitter callsites that haven't yet been migrated to pass an explicit
    /// `verdict_kind`.
    ///
    /// The four canonical shapes:
    /// | exit_code | verified | error_class           | derived kind   |
    /// |-----------|----------|-----------------------|----------------|
    /// | 0         | true     | None                  | Verified       |
    /// | ≠0        | false    | Some(_)               | Failed         |
    /// | 0         | false    | None                  | PartialAccepted|
    /// | 0         | false    | Some(SorryBlocked)    | SorryBlocked   |
    ///
    /// Out-of-canonical shapes return `None` (the caller MUST treat this
    /// as a defect and fail the assertion). This function does NOT silently
    /// repair drift; it identifies it.
    pub fn derive_verdict_kind_from_legacy_fields(
        exit_code: i32,
        verified: bool,
        error_class: Option<LeanErrorClass>,
    ) -> Option<LeanVerdictKind> {
        match (exit_code, verified, error_class) {
            (0, true, None) => Some(LeanVerdictKind::Verified),
            (ec, false, Some(_)) if ec != 0 => Some(LeanVerdictKind::Failed),
            (0, false, None) => Some(LeanVerdictKind::PartialAccepted),
            (0, false, Some(LeanErrorClass::SorryBlocked)) => Some(LeanVerdictKind::SorryBlocked),
            _ => None,
        }
    }

    /// TRACE_MATRIX FC1-N41 + FC2-N34 Phase 2: typed-verdict consistency check.
    ///
    /// Returns `true` iff the typed `verdict_kind` matches the canonical shape
    /// of `(exit_code, verified, error_class)`. This is the predicate that
    /// `assert_45_lean_result_retrievable_from_cas` enforces; exposed publicly
    /// so Phase 2 witness tests can exercise the contract without building a
    /// full `LoadedTape`.
    ///
    /// The four canonical arms (Phase 2 directive §6 + FC-first §2.4):
    ///   - `Verified`        ↔ `exit_code == 0 && verified == true && error_class == None`
    ///   - `Failed`          ↔ `exit_code != 0 && verified == false && error_class.is_some()`
    ///   - `PartialAccepted` ↔ `exit_code == 0 && verified == false && error_class == None`
    ///   - `SorryBlocked`    ↔ `exit_code == 0 && verified == false && error_class == Some(SorryBlocked)`
    ///
    /// Any drift between the typed kind and the legacy fields returns `false`.
    pub fn is_verdict_kind_consistent(&self) -> bool {
        match self.verdict_kind {
            LeanVerdictKind::Verified => {
                self.exit_code == 0 && self.verified && self.error_class.is_none()
            }
            LeanVerdictKind::Failed => {
                self.exit_code != 0 && !self.verified && self.error_class.is_some()
            }
            LeanVerdictKind::PartialAccepted => {
                self.exit_code == 0 && !self.verified && self.error_class.is_none()
            }
            LeanVerdictKind::SorryBlocked => {
                self.exit_code == 0
                    && !self.verified
                    && self.error_class == Some(LeanErrorClass::SorryBlocked)
            }
        }
    }
}

// ── TerminalAbortRecord (FR-18R.3 v2; Codex Q4 remediation) ─────────────────

/// TRACE_MATRIX FC1-N42: explicit per-aborted-attempt record.
///
/// Per FR-18R.3 v2: aborted attempts (externally killed / mid-Lean SIGKILL /
/// per-call budget halt / WallClockCap reached during attempt's Lean check)
/// are excluded from `evaluator_reported_completed_llm_calls` and counted
/// in `attempt_aborted_count` instead. Each aborted attempt gets a
/// `TerminalAbortRecord` CAS object so the chain-derived equation
/// `evaluator_reported_completed_llm_calls == l4_work_attempt_count +
/// l4e_work_attempt_count` (after sequencer drain barrier) holds exactly.
///
/// The `terminal_halt_class` field mirrors `RunOutcome` discriminant for
/// the run as a whole at the time of abort; the per-attempt record records
/// at finer granularity than the run-level `EvidenceCapsule.outcome`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerminalAbortRecord {
    /// The attempt that was aborted. Matches an `AttemptTelemetry.attempt_id`.
    pub attempt_id: TxId,
    /// Run identifier.
    pub run_id: String,
    /// Cause of the abort. See `AbortCause` variants.
    pub cause: AbortCause,
    /// Logical-time at which the abort was detected (sequencer-side).
    pub aborted_at_logical_t: u64,
    /// Optional CID pointing to a partial `LeanResult` if Lean was invoked
    /// but did not return a verdict before SIGKILL / external kill.
    /// `None` for pre-Lean aborts.
    pub partial_lean_result_cid: Option<Cid>,
}

/// TRACE_MATRIX FC1-N42: cause of a terminal abort.
///
/// Stable byte-encoding via `#[repr(u8)]`. Tail-additive only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum AbortCause {
    /// Per-LLM-call wall-clock budget exceeded (OBS_M0 §5.1 budget cap).
    PerCallBudgetExceeded = 0,
    /// Run-level wall-clock cap reached during this attempt's Lean check.
    WallClockCapDuringLean = 1,
    /// Lean process killed externally (SIGKILL / OOM / external timeout).
    LeanKilledExternally = 2,
    /// Compute-cap (`ComputeCapViolated`) reached.
    ComputeCapViolated = 3,
    /// Generic error halt (catch-all; should be rare and investigated).
    ErrorHalt = 4,
}

// ── AttemptEnvelope (evaluator-side bridge; not all fields persisted) ───────

/// TRACE_MATRIX FC1-N41: evaluator hot-path bridge between LLM call and
/// AttemptTelemetry CAS write.
///
/// R2 evaluator uses `AttemptEnvelope` to thread per-attempt context from
/// the LLM-call boundary through Lean invocation to the final
/// `AttemptTelemetry` construction. NOT all envelope fields end up in the
/// CAS object: `parsed_candidate_bytes` is hashed + stored as
/// `candidate_payload_cid`, but the bytes themselves are stored separately
/// as a CAS object of `ObjectType::ProposalPayload`. The envelope is a
/// transient helper; only the AttemptTelemetry is durable.
///
/// Privacy invariant: `parsed_candidate_bytes` MUST be the parsed external
/// candidate (post-extraction from any model wrapper), NEVER the raw LLM
/// response. R2 implements the parse step.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttemptEnvelope {
    pub attempt_id: TxId,
    pub run_id: String,
    pub task_id: String,
    pub agent_id: AgentId,
    pub branch_id: String,
    pub parent_attempt_tx: Option<TxId>,
    pub attempt_index: u64,
    pub prompt_context_hash: Hash,
    /// Parsed external candidate bytes. Hashed + stored as CAS object;
    /// `candidate_payload_cid` on the AttemptTelemetry references the
    /// stored CAS object. NEVER the raw LLM response.
    pub parsed_candidate_bytes: Vec<u8>,
    pub attempt_kind: AttemptKind,
    pub token_counts: TokenCounts,
    pub tool_name: String,
    pub emitted_at_logical_t: u64,
}

// ── Errors ──────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC1-N41: error class for AttemptTelemetry / LeanResult CAS ops.
#[derive(Debug)]
pub enum AttemptTelemetryError {
    Cas(CasError),
    Codec(String),
}

impl std::fmt::Display for AttemptTelemetryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cas(e) => write!(f, "cas error: {e}"),
            Self::Codec(s) => write!(f, "codec error: {s}"),
        }
    }
}

impl std::error::Error for AttemptTelemetryError {}

impl From<CasError> for AttemptTelemetryError {
    fn from(e: CasError) -> Self {
        Self::Cas(e)
    }
}

// ── CAS storage helpers ─────────────────────────────────────────────────────

/// TRACE_MATRIX FC1-N41: canonical-encode an AttemptTelemetry + CAS put.
/// Returns the content-addressed CID. Idempotent (same record → same CID).
pub fn write_attempt_telemetry_to_cas(
    cas: &mut CasStore,
    record: &AttemptTelemetry,
    creator: &str,
    logical_t: u64,
) -> Result<Cid, AttemptTelemetryError> {
    let bytes =
        canonical_encode(record).map_err(|e| AttemptTelemetryError::Codec(e.to_string()))?;
    let cid = cas.put(
        &bytes,
        ObjectType::AttemptTelemetry,
        creator,
        logical_t,
        Some(ATTEMPT_TELEMETRY_SCHEMA_ID.to_string()),
    )?;
    Ok(cid)
}

/// TRACE_MATRIX FC1-N41: CAS fetch + canonical-decode an AttemptTelemetry.
pub fn read_attempt_telemetry_from_cas(
    cas: &CasStore,
    cid: &Cid,
) -> Result<AttemptTelemetry, AttemptTelemetryError> {
    let bytes = cas.get(cid)?;
    decode_attempt_telemetry_compat(&bytes).map_err(|e| AttemptTelemetryError::Codec(e.to_string()))
}

/// TRACE_MATRIX FC1-N41 + REAL-2/REAL-6 shared-slot hardening:
/// `MarketDecisionTrace` currently shares `ObjectType::AttemptTelemetry`
/// as transitional compatibility. Bulk AttemptTelemetry walkers must
/// explicitly classify the known JSON schema and skip it, while unknown
/// JSON in this slot fails closed instead of being silently ignored or
/// fed into the canonical bincode reader.
pub fn read_attempt_telemetry_shared_slot_from_cas(
    cas: &CasStore,
    cid: &Cid,
) -> Result<Option<AttemptTelemetry>, AttemptTelemetryError> {
    let bytes = cas.get(cid)?;
    decode_attempt_telemetry_shared_slot(&bytes)
}

/// Decode a CAS object from the shared AttemptTelemetry slot.
pub fn decode_attempt_telemetry_shared_slot(
    bytes: &[u8],
) -> Result<Option<AttemptTelemetry>, AttemptTelemetryError> {
    let first = bytes.iter().copied().find(|b| !b.is_ascii_whitespace());
    if matches!(first, Some(b'{') | Some(b'[')) {
        let value: serde_json::Value = serde_json::from_slice(bytes).map_err(|e| {
            AttemptTelemetryError::Codec(format!(
                "unknown JSON in AttemptTelemetry slot: invalid JSON: {e}"
            ))
        })?;
        let schema_version = value.get("schema_version").and_then(|v| v.as_str());
        if schema_version
            == Some(crate::runtime::market_decision_trace::MarketDecisionTrace::SCHEMA_VERSION)
        {
            return Ok(None);
        }
        return Err(AttemptTelemetryError::Codec(format!(
            "unknown JSON in AttemptTelemetry slot: schema_version={schema_version:?}"
        )));
    }

    decode_attempt_telemetry_compat(bytes)
        .map(Some)
        .map_err(|e| AttemptTelemetryError::Codec(e.to_string()))
}

/// TRACE_MATRIX FC1 AttemptTelemetry: v3/v2/v1 dual-reader preserving historical CAS evidence.
pub fn decode_attempt_telemetry_compat(
    bytes: &[u8],
) -> Result<AttemptTelemetry, CanonicalCodecError> {
    match canonical_decode::<AttemptTelemetry>(bytes) {
        Ok(v3) => Ok(v3),
        Err(v3_err) => match canonical_decode::<AttemptTelemetryV2Wire>(bytes) {
            Ok(v2) => Ok(v2.into()),
            Err(v2_err) => match canonical_decode::<AttemptTelemetryV1Wire>(bytes) {
                Ok(v1) => Ok(v1.into()),
                Err(v1_err) => Err(CanonicalCodecError::Decode(format!(
                    "AttemptTelemetry v3 decode failed: {v3_err}; v2 fallback failed: {v2_err}; \
                     v1 fallback failed: {v1_err}"
                ))),
            },
        },
    }
}

/// TRACE_MATRIX FC1-N41: canonical-encode a LeanResult + CAS put.
pub fn write_lean_result_to_cas(
    cas: &mut CasStore,
    record: &LeanResult,
    creator: &str,
    logical_t: u64,
) -> Result<Cid, AttemptTelemetryError> {
    let bytes =
        canonical_encode(record).map_err(|e| AttemptTelemetryError::Codec(e.to_string()))?;
    let cid = cas.put(
        &bytes,
        ObjectType::LeanResult,
        creator,
        logical_t,
        Some(LEAN_RESULT_SCHEMA_ID.to_string()),
    )?;
    Ok(cid)
}

/// TRACE_MATRIX FC1-N41: CAS fetch + canonical-decode a LeanResult.
pub fn read_lean_result_from_cas(
    cas: &CasStore,
    cid: &Cid,
) -> Result<LeanResult, AttemptTelemetryError> {
    let bytes = cas.get(cid)?;
    canonical_decode::<LeanResult>(&bytes).map_err(|e| AttemptTelemetryError::Codec(e.to_string()))
}

/// TRACE_MATRIX FC1-N42: canonical-encode a TerminalAbortRecord + CAS put.
pub fn write_terminal_abort_record_to_cas(
    cas: &mut CasStore,
    record: &TerminalAbortRecord,
    creator: &str,
    logical_t: u64,
) -> Result<Cid, AttemptTelemetryError> {
    let bytes =
        canonical_encode(record).map_err(|e| AttemptTelemetryError::Codec(e.to_string()))?;
    let cid = cas.put(
        &bytes,
        ObjectType::TerminalAbortRecord,
        creator,
        logical_t,
        Some(TERMINAL_ABORT_RECORD_SCHEMA_ID.to_string()),
    )?;
    Ok(cid)
}

/// TRACE_MATRIX FC1-N42: CAS fetch + canonical-decode a TerminalAbortRecord.
pub fn read_terminal_abort_record_from_cas(
    cas: &CasStore,
    cid: &Cid,
) -> Result<TerminalAbortRecord, AttemptTelemetryError> {
    let bytes = cas.get(cid)?;
    canonical_decode::<TerminalAbortRecord>(&bytes)
        .map_err(|e| AttemptTelemetryError::Codec(e.to_string()))
}

// ── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use sha2::{Digest, Sha256};
    use tempfile::TempDir;

    fn fresh_cas() -> (TempDir, CasStore) {
        let dir = TempDir::new().expect("tempdir");
        let cas = CasStore::open(dir.path()).expect("open cas");
        (dir, cas)
    }

    fn fresh_hash(domain: &str) -> Hash {
        let mut h = Sha256::new();
        h.update(domain.as_bytes());
        Hash(h.finalize().into())
    }

    fn fresh_attempt(attempt_index: u64) -> AttemptTelemetry {
        AttemptTelemetry::new_root(
            TxId(format!("att-{attempt_index}")),
            "test-run".into(),
            "task-001".into(),
            AgentId("agent_0".into()),
            "n1.b0".into(),
            fresh_hash("ctx"),
            Cid::from_content(format!("candidate-{attempt_index}").as_bytes()),
            AttemptKind::ExternalizedLlmCycle,
            AttemptOutcome::LeanPass,
            TokenCounts::default(),
            "omega_wtool".into(),
        )
    }

    #[test]
    fn attempt_kind_repr_stable() {
        // Discriminator values are part of the canonical hash. Locked-in
        // for forward compat: ExternalizedLlmCycle=0, Tactic=1,
        // ExternalToolCall=2.
        assert_eq!(AttemptKind::ExternalizedLlmCycle as u8, 0);
        assert_eq!(AttemptKind::Tactic as u8, 1);
        assert_eq!(AttemptKind::ExternalToolCall as u8, 2);
    }

    #[test]
    fn attempt_outcome_repr_stable() {
        // Discriminator values locked: LeanPass=0, LeanFail=1, ParseFail=2,
        // SorryBlock=3, LlmErr=4, Aborted=5.
        // Phase 2 (TB-18R 2026-05-06; tail-additive): PartialAccepted=6.
        assert_eq!(AttemptOutcome::LeanPass as u8, 0);
        assert_eq!(AttemptOutcome::LeanFail as u8, 1);
        assert_eq!(AttemptOutcome::ParseFail as u8, 2);
        assert_eq!(AttemptOutcome::SorryBlock as u8, 3);
        assert_eq!(AttemptOutcome::LlmErr as u8, 4);
        assert_eq!(AttemptOutcome::Aborted as u8, 5);
        assert_eq!(AttemptOutcome::PartialAccepted as u8, 6);
    }

    #[test]
    fn lean_verdict_kind_repr_stable() {
        // Phase 2 (TB-18R 2026-05-06): typed verdict discriminators are
        // canonical-hash-bearing. Locked: Verified=0, Failed=1,
        // PartialAccepted=2, SorryBlocked=3.
        assert_eq!(LeanVerdictKind::Verified as u8, 0);
        assert_eq!(LeanVerdictKind::Failed as u8, 1);
        assert_eq!(LeanVerdictKind::PartialAccepted as u8, 2);
        assert_eq!(LeanVerdictKind::SorryBlocked as u8, 3);
    }

    #[test]
    fn lean_verdict_kind_legacy_field_derivation() {
        // The four canonical shapes derive their kinds correctly.
        assert_eq!(
            LeanResult::derive_verdict_kind_from_legacy_fields(0, true, None),
            Some(LeanVerdictKind::Verified)
        );
        assert_eq!(
            LeanResult::derive_verdict_kind_from_legacy_fields(
                1,
                false,
                Some(LeanErrorClass::LeanFailed)
            ),
            Some(LeanVerdictKind::Failed)
        );
        assert_eq!(
            LeanResult::derive_verdict_kind_from_legacy_fields(0, false, None),
            Some(LeanVerdictKind::PartialAccepted)
        );
        assert_eq!(
            LeanResult::derive_verdict_kind_from_legacy_fields(
                0,
                false,
                Some(LeanErrorClass::SorryBlocked)
            ),
            Some(LeanVerdictKind::SorryBlocked)
        );
        // Out-of-canonical shapes return None (caller must fail-close).
        assert_eq!(
            LeanResult::derive_verdict_kind_from_legacy_fields(
                0,
                true,
                Some(LeanErrorClass::LeanFailed)
            ),
            None
        );
        assert_eq!(
            LeanResult::derive_verdict_kind_from_legacy_fields(1, true, None),
            None
        );
        assert_eq!(
            LeanResult::derive_verdict_kind_from_legacy_fields(1, false, None),
            None
        );
    }

    #[test]
    fn lean_error_class_repr_mirrors_r3_rejection_class_tail_append() {
        // Per Codex Q8 source-grounded: pre-TB-18R RejectionClass has
        // 0..5 occupied; R3 tail-appends LeanFailed=6, ParseFailed=7,
        // SorryBlocked=8, LlmError=9. R1 LeanErrorClass mirrors the
        // target values so R3 can derive `From<LeanErrorClass>` for
        // RejectionClass without a renumbering hop.
        assert_eq!(LeanErrorClass::LeanFailed as u8, 6);
        assert_eq!(LeanErrorClass::ParseFailed as u8, 7);
        assert_eq!(LeanErrorClass::SorryBlocked as u8, 8);
        assert_eq!(LeanErrorClass::LlmError as u8, 9);
    }

    #[test]
    fn attempt_telemetry_canonical_encode_deterministic() {
        let a = fresh_attempt(0);
        let b = fresh_attempt(0);
        let bytes_a = canonical_encode(&a).expect("encode a");
        let bytes_b = canonical_encode(&b).expect("encode b");
        assert_eq!(
            bytes_a, bytes_b,
            "canonical_encode must be byte-deterministic for equal records"
        );
    }

    #[test]
    fn attempt_telemetry_canonical_round_trip() {
        let original = fresh_attempt(0);
        let bytes = canonical_encode(&original).expect("encode");
        let decoded: AttemptTelemetry = canonical_decode(&bytes).expect("decode");
        assert_eq!(original, decoded);
    }

    #[test]
    fn attempt_telemetry_cas_round_trip() {
        let (_dir, mut cas) = fresh_cas();
        let original = fresh_attempt(7);
        let cid = write_attempt_telemetry_to_cas(&mut cas, &original, "evaluator", 100)
            .expect("write to cas");
        let recovered = read_attempt_telemetry_from_cas(&cas, &cid).expect("read from cas");
        assert_eq!(original, recovered);
    }

    #[test]
    fn lean_result_canonical_round_trip() {
        let original = LeanResult {
            attempt_id: TxId("att-0".into()),
            exit_code: 1,
            verified: false,
            stderr_cid: Some(Cid::from_content(b"stderr-bytes")),
            stdout_cid: None,
            proof_artifact_cid: None,
            error_class: Some(LeanErrorClass::LeanFailed),
            verdict_kind: LeanVerdictKind::Failed,
        };
        let bytes = canonical_encode(&original).expect("encode");
        let decoded: LeanResult = canonical_decode(&bytes).expect("decode");
        assert_eq!(original, decoded);
    }

    #[test]
    fn lean_result_cas_round_trip() {
        let (_dir, mut cas) = fresh_cas();
        let original = LeanResult {
            attempt_id: TxId("att-1".into()),
            exit_code: 0,
            verified: true,
            stderr_cid: None,
            stdout_cid: None,
            proof_artifact_cid: Some(Cid::from_content(b"proof-bytes")),
            error_class: None,
            verdict_kind: LeanVerdictKind::Verified,
        };
        let cid = write_lean_result_to_cas(&mut cas, &original, "evaluator", 100).expect("write");
        let recovered = read_lean_result_from_cas(&cas, &cid).expect("read");
        assert_eq!(original, recovered);
    }

    #[test]
    fn terminal_abort_record_canonical_round_trip() {
        let original = TerminalAbortRecord {
            attempt_id: TxId("att-aborted".into()),
            run_id: "test-run".into(),
            cause: AbortCause::PerCallBudgetExceeded,
            aborted_at_logical_t: 42,
            partial_lean_result_cid: None,
        };
        let bytes = canonical_encode(&original).expect("encode");
        let decoded: TerminalAbortRecord = canonical_decode(&bytes).expect("decode");
        assert_eq!(original, decoded);
    }

    #[test]
    fn terminal_abort_record_cas_round_trip() {
        let (_dir, mut cas) = fresh_cas();
        let original = TerminalAbortRecord {
            attempt_id: TxId("att-aborted-2".into()),
            run_id: "test-run".into(),
            cause: AbortCause::WallClockCapDuringLean,
            aborted_at_logical_t: 100,
            partial_lean_result_cid: Some(Cid::from_content(b"partial-lean")),
        };
        let cid = write_terminal_abort_record_to_cas(&mut cas, &original, "sequencer", 100)
            .expect("write");
        let recovered = read_terminal_abort_record_from_cas(&cas, &cid).expect("read");
        assert_eq!(original, recovered);
    }

    #[test]
    fn attempt_chain_root_some_only_for_terminal_composite() {
        // Per Codex Q8 ratified: intermediate / failed / aborted attempts
        // carry attempt_chain_root = None; the OMEGA-accept terminal
        // composite carries Some(merkle_root).
        let intermediate = fresh_attempt(0);
        assert!(intermediate.attempt_chain_root.is_none());

        let merkle_root = fresh_hash("merkle-root-of-attempt-chain");
        let terminal = AttemptTelemetry::new_terminal_composite(
            TxId("att-terminal".into()),
            "test-run".into(),
            "task-001".into(),
            AgentId("agent_0".into()),
            "n1.b0".into(),
            Some(TxId("att-prev".into())),
            5,
            fresh_hash("ctx"),
            Cid::from_content(b"final-composite-payload"),
            TokenCounts::default(),
            "omega_wtool".into(),
            merkle_root,
        );
        assert_eq!(terminal.attempt_chain_root, Some(merkle_root));
        assert_eq!(terminal.outcome, AttemptOutcome::LeanPass);
        assert_eq!(terminal.attempt_kind, AttemptKind::ExternalizedLlmCycle);
    }

    #[test]
    fn schema_version_is_real5_v3() {
        // G4.2 bumped AttemptTelemetry for durable actual-model identity;
        // REAL-5 bumps again for PromptCapsuleV2 CID linkage while preserving
        // v1/v2 decode via `decode_attempt_telemetry_compat`.
        assert_eq!(ATTEMPT_TELEMETRY_SCHEMA_VERSION, 3);
        let attempt = fresh_attempt(0);
        assert_eq!(attempt.schema_version, 3);
    }

    #[test]
    fn historical_v2_model_identity_bytes_still_decode_after_real5_v3() {
        let legacy_v2 = AttemptTelemetryV2Wire {
            schema_version: 2,
            attempt_id: TxId("attempt-v2".into()),
            run_id: "run-v2".into(),
            task_id: "task-v2".into(),
            agent_id: AgentId("Agent_1".into()),
            branch_id: "n1.b0".into(),
            parent_attempt_tx: None,
            attempt_index: 0,
            prompt_context_hash: fresh_hash("ctx-v2"),
            candidate_payload_cid: Cid::from_content(b"candidate-v2"),
            lean_result_cid: None,
            attempt_kind: AttemptKind::ExternalizedLlmCycle,
            outcome: AttemptOutcome::LeanFail,
            token_counts: TokenCounts::default(),
            tool_name: "step".into(),
            proposal_telemetry_cid: None,
            verification_result_cid: None,
            attempt_chain_root: None,
            model_name: Some("deepseek-chat".into()),
            model_family: Some("deepseek".into()),
            model_provider: Some("deepseek".into()),
            model_version: None,
            temperature_milli: Some(700),
        };
        let bytes = canonical_encode(&legacy_v2).expect("legacy v2 canonical encode");
        let decoded =
            decode_attempt_telemetry_compat(&bytes).expect("v2 evidence remains parseable");
        assert_eq!(decoded.schema_version, 2);
        assert_eq!(decoded.model_family.as_deref(), Some("deepseek"));
        assert_eq!(decoded.prompt_capsule_cid, None);
    }

    #[test]
    fn lean_result_shielded_stderr_cid_is_cid_not_bytes() {
        // Privacy: stderr is not stored inline. The LeanResult struct
        // carries `stderr_cid: Option<Cid>`, never raw stderr bytes.
        // This is a structural guarantee: the type system prevents
        // raw bytes by construction (Cid is [u8; 32], not Vec<u8>).
        let r = LeanResult {
            attempt_id: TxId("att-0".into()),
            exit_code: 1,
            verified: false,
            stderr_cid: Some(Cid::from_content(b"some stderr text")),
            stdout_cid: None,
            proof_artifact_cid: None,
            error_class: Some(LeanErrorClass::LeanFailed),
            verdict_kind: LeanVerdictKind::Failed,
        };
        // Compile-time guarantee: stderr_cid is Option<Cid>, not Option<Vec<u8>>.
        // Runtime sanity: the Cid is exactly 32 bytes regardless of stderr length.
        let cid = r.stderr_cid.expect("stderr_cid set");
        assert_eq!(cid.0.len(), 32);
    }

    #[test]
    fn outcome_distinct_from_kind_in_canonical_encoding() {
        // Per Codex Q5 ratified: AttemptKind and AttemptOutcome are
        // separately encoded (not collapsed into a single enum). Verify
        // that two attempts with same kind but different outcome produce
        // distinct canonical bytes.
        let mut a = fresh_attempt(0);
        a.outcome = AttemptOutcome::LeanPass;
        let mut b = fresh_attempt(0);
        b.outcome = AttemptOutcome::LeanFail;
        let bytes_a = canonical_encode(&a).expect("encode a");
        let bytes_b = canonical_encode(&b).expect("encode b");
        assert_ne!(
            bytes_a, bytes_b,
            "different outcome must produce different canonical bytes"
        );
    }
}
