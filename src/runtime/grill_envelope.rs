//! TRACE_MATRIX FC1-N5 + FC1-N7: grill turn-payload schema, parser, and validator.
//!
//! Phase 6.3.x Software 3.0 LLM-driven grill. Each LLM turn returns a JSON
//! envelope conforming to `TurnPayload`. This module parses and structurally
//! validates that envelope; semantic validation (P3 vocab, P4 monotonicity,
//! etc.) is in `crate::grill_predicates`.
//!
//! R2.1 path canon: this module lives in the library crate (not bin) so
//! `grill_predicates` (also library-crate) can import `TurnPayload` directly.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Canonical 8 interview slot identifiers. Order matters for tests but not
/// semantics. P3 (slots_in_vocab predicate) checks every emitted slot id is
/// in this set.
pub const CANONICAL_SLOTS: &[&str] = &[
    "job",
    "anchor",
    "memory",
    "first_run",
    "robustness",
    "scope",
    "acceptance",
    "mirror",
];

/// Required slot subset (7 of 8; excludes "mirror"). Termination predicate
/// requires `covered_slots ⊇ REQUIRED_SLOTS` before allowing LLM done=true to
/// terminate the session.
pub const REQUIRED_SLOTS: &[&str] = &[
    "job",
    "anchor",
    "memory",
    "first_run",
    "robustness",
    "scope",
    "acceptance",
];

/// Parsed turn payload from one LLM response. Matches the OUTPUT CONTRACT in
/// `assets/prompts/grill_meta_v1.md` (the meta-prompt). Field names match the
/// LLM's emitted JSON keys exactly.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TurnPayload {
    pub turn: u32,
    pub question: Option<String>,
    pub covered_slots: Vec<String>,
    pub open_slots: Vec<String>,
    pub confidence: f64,
    pub done: bool,
    pub rationale: String,
    /// Present only when done=true; the 7-row "fridge note" mirror.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub playback: Option<String>,
}

/// Errors from parsing/validating the turn-payload envelope. Structural
/// errors only; semantic predicate failures (P3 vocab, P4 monotonicity) are
/// in `crate::grill_predicates`.
#[derive(Debug, Clone, PartialEq)]
pub enum EnvelopeParseError {
    InvalidJson(String),
    TurnOutOfRange(u32),
    ConfidenceOutOfRange(f64),
    EmptyQuestionWhenNotDone,
    MissingPlaybackOnDone,
}

impl fmt::Display for EnvelopeParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidJson(s) => write!(f, "invalid JSON: {}", s),
            Self::TurnOutOfRange(n) => write!(f, "turn must be 1..=15, got {}", n),
            Self::ConfidenceOutOfRange(c) => {
                write!(f, "confidence must be in [0.0, 1.0], got {}", c)
            }
            Self::EmptyQuestionWhenNotDone => {
                write!(f, "if done=false, question must be non-empty string")
            }
            Self::MissingPlaybackOnDone => {
                write!(f, "if done=true, playback must be present and non-empty")
            }
        }
    }
}

impl std::error::Error for EnvelopeParseError {}

/// Parse raw LLM response string into TurnPayload. Pure deserialize; does NOT
/// apply semantic checks (range / well-formedness). Use `parse_and_validate`
/// for the full check.
///
/// Returns `EnvelopeParseError::InvalidJson` on serde_json errors.
pub fn parse_turn_payload(raw: &str) -> Result<TurnPayload, EnvelopeParseError> {
    serde_json::from_str::<TurnPayload>(raw)
        .map_err(|e| EnvelopeParseError::InvalidJson(e.to_string()))
}

/// Parse + structurally validate. Applies:
/// - JSON parse (delegated to `parse_turn_payload`)
/// - turn ∈ [1, 15]
/// - confidence ∈ [0.0, 1.0]
/// - if done=false then question is Some(non-empty)
/// - if done=true then playback is Some(non-empty)
///
/// Does NOT validate slot vocabulary (P3), monotonicity (P4), language (P6).
/// Those are predicate concerns handled by `crate::grill_predicates`.
pub fn parse_and_validate(raw: &str) -> Result<TurnPayload, EnvelopeParseError> {
    let payload = parse_turn_payload(raw)?;

    if payload.turn == 0 || payload.turn > 15 {
        return Err(EnvelopeParseError::TurnOutOfRange(payload.turn));
    }

    if !(0.0..=1.0).contains(&payload.confidence) {
        return Err(EnvelopeParseError::ConfidenceOutOfRange(payload.confidence));
    }

    if !payload.done {
        match &payload.question {
            Some(q) if !q.is_empty() => {}
            _ => return Err(EnvelopeParseError::EmptyQuestionWhenNotDone),
        }
    } else {
        match &payload.playback {
            Some(p) if !p.is_empty() => {}
            _ => return Err(EnvelopeParseError::MissingPlaybackOnDone),
        }
    }

    Ok(payload)
}
