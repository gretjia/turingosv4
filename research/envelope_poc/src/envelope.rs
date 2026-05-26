//! Envelope validator — adapter-side structure gate.
//!
//! Single public entry point: [`validate`]. Returns:
//!   Ok(ValidationOk) — envelope parsed; predicate gate may now run.
//!   Err(EnvelopeValidationSubclass) — short-circuit; predicate NOT invoked.
//!
//! Surrogate enums (`AttemptOutcomeSurrogate`, `RejectionClassSurrogate`)
//! mirror the main crate's `runtime::attempt_telemetry::AttemptOutcome` and
//! `bottom_white::ledger::rejection_evidence::RejectionClass` by variant
//! order. The mapping `EnvelopeValidationSubclass -> {AttemptOutcome,
//! RejectionClass}` is the contract proven by `tests/decoupling.rs`.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

// ── Surrogate enums (mirror main crate; do NOT drift) ───────────────────────

/// Mirror of `src/runtime/attempt_telemetry.rs::AttemptOutcome` (commit
/// 03a84470). 7 variants, exhaustive.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum AttemptOutcomeSurrogate {
    LeanPass = 0,
    LeanFail = 1,
    ParseFail = 2,
    SorryBlock = 3,
    LlmErr = 4,
    Aborted = 5,
    PartialAccepted = 6,
}

/// Mirror of `src/bottom_white/ledger/rejection_evidence.rs::RejectionClass`
/// (variants 0..5 are the pre-TB-18R set; 6..9 are the R3 tail-append per
/// `src/runtime/mod.rs:70` comment).
///
/// **Surrogate self-audit note (2026-05-26)**: variants 2-5 were initially
/// drafted from generic-pattern guess (BudgetExceeded/StateRootStale/
/// SignatureInvalid/NonceReplay) and corrected after grep against the real
/// `rejection_evidence.rs:204-241`. The PoC's mapping table never uses
/// variants 2-5 (only 1 and 7), so behavior was unaffected; but the names
/// must match the main crate for the surrogate claim to hold. This
/// correction is preserved as a research finding — see
/// `handover/research/SELF_AUDIT_2026-05-26.md §8 Round 2 drift`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum RejectionClassSurrogate {
    PredicateFailed = 0,
    PolicyViolation = 1,
    EscrowMissing = 2,
    InvariantViolation = 3,
    MalformedPayload = 4,
    InsufficientBalance = 5,
    LeanFailed = 6,
    ParseFailed = 7,
    SorryBlocked = 8,
    LlmError = 9,
}

// ── EnvelopeValidationSubclass — the new adapter-internal taxonomy ──────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum EnvelopeValidationSubclass {
    EnvelopeNotJson = 0,
    EnvelopeMalformed = 1,
    EnvelopeUnknownVariant = 2,
    EnvelopePayloadMalformed = 3,
    EnvelopeFieldTooLarge = 4,
    EnvelopeAgentIdentityMismatch = 5,
    EnvelopeStageOutOfSet = 6,
}

impl EnvelopeValidationSubclass {
    pub fn to_attempt_outcome(self) -> AttemptOutcomeSurrogate {
        AttemptOutcomeSurrogate::ParseFail
    }

    pub fn to_rejection_class(self) -> RejectionClassSurrogate {
        match self {
            EnvelopeValidationSubclass::EnvelopeAgentIdentityMismatch => {
                RejectionClassSurrogate::PolicyViolation
            }
            _ => RejectionClassSurrogate::ParseFailed,
        }
    }

    pub fn dotted_label(self) -> &'static str {
        match self {
            EnvelopeValidationSubclass::EnvelopeNotJson => "parse_fail.envelope_not_json",
            EnvelopeValidationSubclass::EnvelopeMalformed => "parse_fail.envelope_malformed",
            EnvelopeValidationSubclass::EnvelopeUnknownVariant => {
                "parse_fail.envelope_unknown_variant"
            }
            EnvelopeValidationSubclass::EnvelopePayloadMalformed => {
                "parse_fail.envelope_payload_malformed"
            }
            EnvelopeValidationSubclass::EnvelopeFieldTooLarge => "parse_fail.envelope_field_too_large",
            EnvelopeValidationSubclass::EnvelopeAgentIdentityMismatch => {
                "parse_fail.envelope_identity_mismatch"
            }
            EnvelopeValidationSubclass::EnvelopeStageOutOfSet => "parse_fail.envelope_stage_out_of_set",
        }
    }
}

// ── TaskKind + envelope shapes ──────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskKind {
    LeanStep,
    Math500,
    Gpqa,
    MarketSignal,
    Fc3Directive,
}

impl TaskKind {
    pub fn wire_label(self) -> &'static str {
        match self {
            TaskKind::LeanStep => "lean_step",
            TaskKind::Math500 => "math500",
            TaskKind::Gpqa => "gpqa",
            TaskKind::MarketSignal => "market_signal",
            TaskKind::Fc3Directive => "fc3_directive",
        }
    }

    pub fn from_wire(s: &str) -> Option<TaskKind> {
        Some(match s {
            "lean_step" => TaskKind::LeanStep,
            "math500" => TaskKind::Math500,
            "gpqa" => TaskKind::Gpqa,
            "market_signal" => TaskKind::MarketSignal,
            "fc3_directive" => TaskKind::Fc3Directive,
            _ => return None,
        })
    }
}

const MAX_FIELD_BYTES: usize = 2 * 1024;
const MAX_NARRATION_BYTES: usize = 512;
const ENVELOPE_VERSION: &str = "v1";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSelfReport {
    pub agent_label: String,
    pub stage_label: String,
    #[serde(default)]
    pub model_provider_hint: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentOutputEnvelope {
    pub envelope_version: String,
    pub task_kind: String,
    pub task_id: String,
    pub attempt_branch_id: String,
    pub agent_self_report: AgentSelfReport,
    pub payload: serde_json::Value,
}

// ── Typed payload candidates (post-validation) ──────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MarketSide {
    Yes,
    No,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DirectiveKind {
    Propose,
    Veto,
    Ratify,
}

#[derive(Debug, Clone)]
pub enum PayloadCandidate {
    LeanStep {
        lean_tactic_block: String,
        narration: String,
        claims_omega_complete: bool,
    },
    Math500 {
        final_answer_boxed: String,
        working: String,
    },
    Gpqa {
        final_answer_letter: char,
        working: String,
        confidence_milli: Option<i64>,
    },
    MarketSignal {
        event_id: String,
        side: MarketSide,
        size_lots: i64,
        rationale: String,
        claimed_evidence_cids: Vec<String>,
    },
    Fc3Directive {
        directive_kind: DirectiveKind,
        target_fc_node: String,
        target_predicate_id: Option<String>,
        rationale: String,
        constitution_section_ref: String,
    },
}

#[derive(Debug, Clone)]
pub struct ValidationOk {
    pub envelope: AgentOutputEnvelope,
    pub payload: PayloadCandidate,
}

// ── EnvelopeRejectionPayload — what goes into CAS on failure ────────────────

/// The minimal CAS-resident object written when validation fails.
/// CR-18R.4 v2 privacy invariant: NEVER contains raw LLM response bytes —
/// only a hash prefix for cross-record correlation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvelopeRejectionPayload {
    pub envelope_validation_subclass: EnvelopeValidationSubclass,
    pub first_error_path: String,
    pub first_error_message: String,
    pub raw_body_sha256_prefix_8_hex: String,
    pub task_kind_attempted: String,
}

impl EnvelopeRejectionPayload {
    pub fn from(
        subclass: EnvelopeValidationSubclass,
        path: impl Into<String>,
        msg: impl Into<String>,
        raw_body: &str,
        task_kind_attempted: TaskKind,
    ) -> Self {
        let mut h = Sha256::new();
        h.update(raw_body.as_bytes());
        let full = format!("{:x}", h.finalize());
        Self {
            envelope_validation_subclass: subclass,
            first_error_path: path.into(),
            first_error_message: msg.into(),
            raw_body_sha256_prefix_8_hex: full[..8].to_string(),
            task_kind_attempted: task_kind_attempted.wire_label().to_string(),
        }
    }
}

// ── validate — single public entry point ────────────────────────────────────

pub struct ValidateContext<'a> {
    pub expected_task_kind: TaskKind,
    pub expected_task_id: &'a str,
    pub expected_agent_id: &'a str,
    pub legal_stages: &'a [&'a str],
    pub known_fc_nodes: &'a [&'a str],
}

pub fn validate(
    body: &str,
    ctx: &ValidateContext,
) -> Result<ValidationOk, (EnvelopeValidationSubclass, String, String)> {
    // PRIVACY RULE (round-4 hardening, 2026-05-26): error messages MUST
    // carry only static category text + system-defined enum labels. NEVER
    // embed agent-supplied field values (raw body content). Path info goes
    // into err.1 — values do NOT belong in err.2. Tested by
    // tests/privacy_fence.rs::error_message_disjoint_from_raw_body.

    // 1. JSON lexical parse — drop serde's error string; it may quote body fragments.
    let v: serde_json::Value =
        serde_json::from_str(body).map_err(|_e| {
            (
                EnvelopeValidationSubclass::EnvelopeNotJson,
                "$".to_string(),
                "body is not valid JSON".to_string(),
            )
        })?;

    // 2. envelope shape — drop serde's error string (may quote field values).
    let envelope: AgentOutputEnvelope = serde_json::from_value(v).map_err(|_e| {
        (
            EnvelopeValidationSubclass::EnvelopeMalformed,
            "$".to_string(),
            "envelope shape mismatch (missing or wrong-type required field)".to_string(),
        )
    })?;

    // 3. envelope_version
    if envelope.envelope_version != ENVELOPE_VERSION {
        return Err((
            EnvelopeValidationSubclass::EnvelopeUnknownVariant,
            "$.envelope_version".to_string(),
            format!("envelope_version must equal {}", ENVELOPE_VERSION),
        ));
    }

    // 4. task_kind known + matches expected — system enum labels only.
    let parsed_kind = TaskKind::from_wire(&envelope.task_kind).ok_or((
        EnvelopeValidationSubclass::EnvelopeUnknownVariant,
        "$.task_kind".to_string(),
        "task_kind not in {lean_step,math500,gpqa,market_signal,fc3_directive}"
            .to_string(),
    ))?;
    if parsed_kind != ctx.expected_task_kind {
        return Err((
            EnvelopeValidationSubclass::EnvelopeMalformed,
            "$.task_kind".to_string(),
            format!(
                "task_kind mismatch (expected {})",
                ctx.expected_task_kind.wire_label()
            ),
        ));
    }

    // 5. task_id matches
    if envelope.task_id != ctx.expected_task_id {
        return Err((
            EnvelopeValidationSubclass::EnvelopeMalformed,
            "$.task_id".to_string(),
            "task_id mismatch (see $.task_id path)".to_string(),
        ));
    }

    // 6. agent identity
    if envelope.agent_self_report.agent_label != ctx.expected_agent_id {
        return Err((
            EnvelopeValidationSubclass::EnvelopeAgentIdentityMismatch,
            "$.agent_self_report.agent_label".to_string(),
            "agent_label mismatch (see path)".to_string(),
        ));
    }

    // 7. stage in legal set — only system-defined legal_stages set is named.
    if !ctx
        .legal_stages
        .iter()
        .any(|&s| s == envelope.agent_self_report.stage_label)
    {
        return Err((
            EnvelopeValidationSubclass::EnvelopeStageOutOfSet,
            "$.agent_self_report.stage_label".to_string(),
            format!("stage_label not in legal set {:?}", ctx.legal_stages),
        ));
    }

    // 8. typed payload per task_kind
    let payload = parse_payload(&envelope, parsed_kind, ctx)?;

    Ok(ValidationOk { envelope, payload })
}

fn parse_payload(
    envelope: &AgentOutputEnvelope,
    kind: TaskKind,
    ctx: &ValidateContext,
) -> Result<PayloadCandidate, (EnvelopeValidationSubclass, String, String)> {
    let p = &envelope.payload;
    match kind {
        TaskKind::LeanStep => {
            let block = require_str(p, "lean_tactic_block", MAX_FIELD_BYTES)?;
            let narration = require_str(p, "narration", MAX_NARRATION_BYTES)?;
            let claims = require_bool(p, "claims_omega_complete")?;
            Ok(PayloadCandidate::LeanStep {
                lean_tactic_block: block,
                narration,
                claims_omega_complete: claims,
            })
        }
        TaskKind::Math500 => {
            let boxed = require_str(p, "final_answer_boxed", MAX_FIELD_BYTES)?;
            if !boxed.starts_with("\\boxed{") || !boxed.ends_with('}') {
                return Err((
                    EnvelopeValidationSubclass::EnvelopePayloadMalformed,
                    "$.payload.final_answer_boxed".to_string(),
                    "expected string starting with \\boxed{ and ending with }".to_string(),
                ));
            }
            let working = require_str(p, "working", MAX_FIELD_BYTES)?;
            Ok(PayloadCandidate::Math500 {
                final_answer_boxed: boxed,
                working,
            })
        }
        TaskKind::Gpqa => {
            let letter_str = require_str(p, "final_answer_letter", MAX_FIELD_BYTES)?;
            let mut chars = letter_str.chars();
            let letter = match (chars.next(), chars.next()) {
                (Some(c @ ('A' | 'B' | 'C' | 'D')), None) => c,
                _ => {
                    return Err((
                        EnvelopeValidationSubclass::EnvelopePayloadMalformed,
                        "$.payload.final_answer_letter".to_string(),
                        "expected single character in {A,B,C,D}".to_string(),
                    ))
                }
            };
            let working = require_str(p, "working", MAX_FIELD_BYTES)?;
            let confidence_milli = optional_i64(p, "confidence_milli")?;
            Ok(PayloadCandidate::Gpqa {
                final_answer_letter: letter,
                working,
                confidence_milli,
            })
        }
        TaskKind::MarketSignal => {
            let event_id = require_str(p, "event_id", MAX_FIELD_BYTES)?;
            if event_id != ctx.expected_task_id {
                return Err((
                    EnvelopeValidationSubclass::EnvelopePayloadMalformed,
                    "$.payload.event_id".to_string(),
                    "event_id must equal task_id".to_string(),
                ));
            }
            let side_str = require_str(p, "side", MAX_FIELD_BYTES)?;
            let side = match side_str.as_str() {
                "YES" => MarketSide::Yes,
                "NO" => MarketSide::No,
                _ => {
                    return Err((
                        EnvelopeValidationSubclass::EnvelopeUnknownVariant,
                        "$.payload.side".to_string(),
                        "side not in {YES,NO}".to_string(),
                    ))
                }
            };
            let size_lots = require_i64(p, "size_lots")?;
            if size_lots <= 0 {
                return Err((
                    EnvelopeValidationSubclass::EnvelopePayloadMalformed,
                    "$.payload.size_lots".to_string(),
                    "size_lots must be positive integer".to_string(),
                ));
            }
            let rationale = require_str(p, "rationale", MAX_FIELD_BYTES)?;
            let cids = require_string_array(p, "claimed_evidence_cids")?;
            Ok(PayloadCandidate::MarketSignal {
                event_id,
                side,
                size_lots,
                rationale,
                claimed_evidence_cids: cids,
            })
        }
        TaskKind::Fc3Directive => {
            let kind_str = require_str(p, "directive_kind", MAX_FIELD_BYTES)?;
            let directive_kind = match kind_str.as_str() {
                "PROPOSE" => DirectiveKind::Propose,
                "VETO" => DirectiveKind::Veto,
                "RATIFY" => DirectiveKind::Ratify,
                _ => {
                    return Err((
                        EnvelopeValidationSubclass::EnvelopeUnknownVariant,
                        "$.payload.directive_kind".to_string(),
                        "directive_kind not in {PROPOSE,VETO,RATIFY}".to_string(),
                    ))
                }
            };
            let target_fc_node = require_str(p, "target_fc_node", MAX_FIELD_BYTES)?;
            if !ctx.known_fc_nodes.iter().any(|&n| n == target_fc_node) {
                return Err((
                    EnvelopeValidationSubclass::EnvelopePayloadMalformed,
                    "$.payload.target_fc_node".to_string(),
                    "target_fc_node not in known FC node set".to_string(),
                ));
            }
            let target_predicate_id = optional_str(p, "target_predicate_id", MAX_FIELD_BYTES)?;
            let rationale = require_str(p, "rationale", MAX_FIELD_BYTES)?;
            let constitution_section_ref =
                require_str(p, "constitution_section_ref", MAX_FIELD_BYTES)?;
            Ok(PayloadCandidate::Fc3Directive {
                directive_kind,
                target_fc_node,
                target_predicate_id,
                rationale,
                constitution_section_ref,
            })
        }
    }
}

// ── Tiny extractors that surface the right subclass on failure ──────────────

fn require_str(
    v: &serde_json::Value,
    key: &str,
    max_bytes: usize,
) -> Result<String, (EnvelopeValidationSubclass, String, String)> {
    let s = v
        .get(key)
        .and_then(|x| x.as_str())
        .ok_or_else(|| {
            (
                EnvelopeValidationSubclass::EnvelopePayloadMalformed,
                format!("$.payload.{}", key),
                format!("missing string field {:?}", key),
            )
        })?
        .to_string();
    if s.len() > max_bytes {
        return Err((
            EnvelopeValidationSubclass::EnvelopeFieldTooLarge,
            format!("$.payload.{}", key),
            format!("field exceeds {} bytes (got {})", max_bytes, s.len()),
        ));
    }
    Ok(s)
}

fn optional_str(
    v: &serde_json::Value,
    key: &str,
    max_bytes: usize,
) -> Result<Option<String>, (EnvelopeValidationSubclass, String, String)> {
    match v.get(key) {
        None | Some(serde_json::Value::Null) => Ok(None),
        Some(_) => Ok(Some(require_str(v, key, max_bytes)?)),
    }
}

fn require_bool(
    v: &serde_json::Value,
    key: &str,
) -> Result<bool, (EnvelopeValidationSubclass, String, String)> {
    v.get(key).and_then(|x| x.as_bool()).ok_or_else(|| {
        (
            EnvelopeValidationSubclass::EnvelopePayloadMalformed,
            format!("$.payload.{}", key),
            format!("missing bool field {:?}", key),
        )
    })
}

fn require_i64(
    v: &serde_json::Value,
    key: &str,
) -> Result<i64, (EnvelopeValidationSubclass, String, String)> {
    v.get(key).and_then(|x| x.as_i64()).ok_or_else(|| {
        (
            EnvelopeValidationSubclass::EnvelopePayloadMalformed,
            format!("$.payload.{}", key),
            format!("missing i64 field {:?}", key),
        )
    })
}

fn optional_i64(
    v: &serde_json::Value,
    key: &str,
) -> Result<Option<i64>, (EnvelopeValidationSubclass, String, String)> {
    match v.get(key) {
        None | Some(serde_json::Value::Null) => Ok(None),
        Some(_) => Ok(Some(require_i64(v, key)?)),
    }
}

fn require_string_array(
    v: &serde_json::Value,
    key: &str,
) -> Result<Vec<String>, (EnvelopeValidationSubclass, String, String)> {
    let arr = v.get(key).and_then(|x| x.as_array()).ok_or_else(|| {
        (
            EnvelopeValidationSubclass::EnvelopePayloadMalformed,
            format!("$.payload.{}", key),
            format!("missing string-array field {:?}", key),
        )
    })?;
    let mut out = Vec::with_capacity(arr.len());
    for (i, item) in arr.iter().enumerate() {
        match item.as_str() {
            Some(s) => out.push(s.to_string()),
            None => {
                return Err((
                    EnvelopeValidationSubclass::EnvelopePayloadMalformed,
                    format!("$.payload.{}[{}]", key, i),
                    "array element must be string".to_string(),
                ))
            }
        }
    }
    Ok(out)
}
