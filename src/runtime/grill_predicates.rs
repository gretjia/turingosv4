//! TRACE_MATRIX FC1-N9: per-turn predicates + session-aggregate termination
//! predicate for the Software 3.0 LLM-driven spec grill.
//!
//! Phase 6.3.x. R2 §A8 typed verdict enum. R2 §A3 termination predicate is
//! FC1-N9 session-aggregate variant (Class 3, not Class 4 oracle).
//!
//! Predicates are pure functions over `TurnPayload`. No I/O, no async.

use serde::{Deserialize, Serialize};

use crate::runtime::grill_envelope::{TurnPayload, CANONICAL_SLOTS, REQUIRED_SLOTS};

/// Language selector for P6 (question_non_empty_lang). Mirrors `cmd_spec::Lang`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Lang {
    Zh,
    En,
}

/// Typed predicate verdict (R2 §A8). Pass / Fail with typed failure class.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PredicateVerdict {
    Pass,
    Fail(PredicateFailureClass),
}

impl PredicateVerdict {
    pub fn is_pass(&self) -> bool {
        matches!(self, PredicateVerdict::Pass)
    }

    pub fn failure_class(&self) -> Option<PredicateFailureClass> {
        match self {
            PredicateVerdict::Pass => None,
            PredicateVerdict::Fail(c) => Some(*c),
        }
    }
}

/// Byte-stable typed failure class (discriminants pinned; tail-additive).
/// Mirrors `LeanErrorClass` / `RejectionClass` typed-error discipline per
/// CLAUDE.md §14. Per R2 §A8.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum PredicateFailureClass {
    /// P1: envelope did not parse (handled at parser layer; included here
    /// for trace symmetry).
    SchemaParseError = 0,
    /// P2 sub: top-level kind mismatch (e.g. done=true + null playback at
    /// predicate layer, even though parse_and_validate also catches it).
    KindMismatch = 1,
    /// P3: covered_slots or open_slots contains a slot id not in
    /// `CANONICAL_SLOTS`.
    UnknownSlot = 2,
    /// P4: covered_slots[N+1] is NOT a superset of covered_slots[N].
    NonMonotonic = 3,
    /// P5: turn_index > 15.
    TurnOutOfRange = 4,
    /// P6: language predicate — question lang doesn't match --lang param.
    LanguageMismatch = 5,
    /// P6 sub: question is shorter than 8 chars (when done=false).
    QuestionTooShort = 6,
    /// P2 sub: done=false but question is None or empty.
    QuestionMissing = 7,
    /// P2 sub: done=true but playback is None or empty.
    PlaybackMissing = 8,
    /// Envelope confidence out of [0.0, 1.0] (parse layer catches; here for
    /// symmetry).
    ConfidenceOutOfRange = 9,
}

/// Bundled per-turn predicate results. P1 is parser-level (implicit pass if
/// we got a `TurnPayload`; kept here for trace symmetry).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredicateBundle {
    pub p1_schema_parse_ok: PredicateVerdict,
    pub p2_kind_ok: PredicateVerdict,
    pub p3_slots_in_vocab: PredicateVerdict,
    pub p4_monotonic: PredicateVerdict,
    pub p5_turn_bounded: PredicateVerdict,
    pub p6_question_nonempty_lang: PredicateVerdict,
}

impl PredicateBundle {
    pub fn all_pass(&self) -> bool {
        self.p1_schema_parse_ok.is_pass()
            && self.p2_kind_ok.is_pass()
            && self.p3_slots_in_vocab.is_pass()
            && self.p4_monotonic.is_pass()
            && self.p5_turn_bounded.is_pass()
            && self.p6_question_nonempty_lang.is_pass()
    }

    /// Returns the first failed predicate's class, if any.
    pub fn first_failure(&self) -> Option<PredicateFailureClass> {
        for v in [
            &self.p1_schema_parse_ok,
            &self.p2_kind_ok,
            &self.p3_slots_in_vocab,
            &self.p4_monotonic,
            &self.p5_turn_bounded,
            &self.p6_question_nonempty_lang,
        ] {
            if let Some(c) = v.failure_class() {
                return Some(c);
            }
        }
        None
    }
}

/// P2: kind discriminator. Done flag must be consistent with question/playback presence.
/// (Some overlap with parse_and_validate — both layers MUST agree.)
pub fn p2_kind_ok(payload: &TurnPayload) -> PredicateVerdict {
    if !payload.done {
        match &payload.question {
            Some(q) if !q.is_empty() => PredicateVerdict::Pass,
            _ => PredicateVerdict::Fail(PredicateFailureClass::QuestionMissing),
        }
    } else {
        match &payload.playback {
            Some(p) if !p.is_empty() => PredicateVerdict::Pass,
            _ => PredicateVerdict::Fail(PredicateFailureClass::PlaybackMissing),
        }
    }
}

/// P3: every slot id in covered_slots and open_slots must be in CANONICAL_SLOTS.
pub fn p3_slots_in_vocab(payload: &TurnPayload) -> PredicateVerdict {
    for slot in payload
        .covered_slots
        .iter()
        .chain(payload.open_slots.iter())
    {
        if !CANONICAL_SLOTS.contains(&slot.as_str()) {
            return PredicateVerdict::Fail(PredicateFailureClass::UnknownSlot);
        }
    }
    PredicateVerdict::Pass
}

/// P4: covered_slots monotonically non-shrinking. Compare to prev turn's covered.
/// prev_covered is empty for turn 1.
pub fn p4_monotonic(payload: &TurnPayload, prev_covered: &[String]) -> PredicateVerdict {
    for prev_slot in prev_covered {
        if !payload.covered_slots.contains(prev_slot) {
            return PredicateVerdict::Fail(PredicateFailureClass::NonMonotonic);
        }
    }
    PredicateVerdict::Pass
}

/// P5: turn ∈ [1, 15]. Hard ceiling per R1 §7.
pub fn p5_turn_bounded(payload: &TurnPayload) -> PredicateVerdict {
    if payload.turn == 0 || payload.turn > 15 {
        PredicateVerdict::Fail(PredicateFailureClass::TurnOutOfRange)
    } else {
        PredicateVerdict::Pass
    }
}

/// P6: when done=false, question must be non-empty AND match the --lang param.
/// Lang::Zh: Han-script ratio ≥ 0.5 of non-whitespace, non-punct chars.
/// Lang::En: ASCII alphanumeric ratio ≥ 0.8 of non-whitespace, non-punct chars.
/// Additionally: question.len() ≥ 8 chars when done=false.
pub fn p6_question_nonempty_lang(payload: &TurnPayload, lang: Lang) -> PredicateVerdict {
    if payload.done {
        return PredicateVerdict::Pass; // P6 doesn't apply on terminal turn
    }
    let q = match &payload.question {
        Some(q) if !q.is_empty() => q,
        _ => return PredicateVerdict::Fail(PredicateFailureClass::QuestionMissing),
    };
    if q.chars().count() < 8 {
        return PredicateVerdict::Fail(PredicateFailureClass::QuestionTooShort);
    }
    let lang_ratio = match lang {
        Lang::Zh => han_script_ratio(q),
        Lang::En => ascii_alpha_ratio(q),
    };
    let threshold = match lang {
        Lang::Zh => 0.5,
        Lang::En => 0.8,
    };
    if lang_ratio < threshold {
        return PredicateVerdict::Fail(PredicateFailureClass::LanguageMismatch);
    }
    PredicateVerdict::Pass
}

/// Run all 6 per-turn predicates. P1 is parser-implicit (we got a TurnPayload).
/// `prev_covered` is empty for turn 1.
pub fn run_turn_predicates(
    payload: &TurnPayload,
    prev_covered: &[String],
    lang: Lang,
) -> PredicateBundle {
    PredicateBundle {
        p1_schema_parse_ok: PredicateVerdict::Pass, // implicit
        p2_kind_ok: p2_kind_ok(payload),
        p3_slots_in_vocab: p3_slots_in_vocab(payload),
        p4_monotonic: p4_monotonic(payload, prev_covered),
        p5_turn_bounded: p5_turn_bounded(payload),
        p6_question_nonempty_lang: p6_question_nonempty_lang(payload, lang),
    }
}

/// Session-aggregate termination predicate (R2 §A3, FC1-N9 session-aggregate variant).
///
/// Returns Pass IFF:
///   covered_slots ⊇ REQUIRED_SLOTS
///   AND confidence ≥ 0.8
///   AND turn ≥ 4
///
/// Predicate fail at termination loops the interview (NOT L4.E); kernel
/// injects "You declared done but slot X is missing" per Researcher A §5.2.
pub fn termination_predicate(payload: &TurnPayload) -> PredicateVerdict {
    if payload.turn < 4 {
        return PredicateVerdict::Fail(PredicateFailureClass::TurnOutOfRange);
    }
    if !(0.0..=1.0).contains(&payload.confidence) {
        return PredicateVerdict::Fail(PredicateFailureClass::ConfidenceOutOfRange);
    }
    if payload.confidence < 0.8 {
        return PredicateVerdict::Fail(PredicateFailureClass::ConfidenceOutOfRange);
    }
    for required in REQUIRED_SLOTS {
        if !payload.covered_slots.iter().any(|s| s == *required) {
            return PredicateVerdict::Fail(PredicateFailureClass::QuestionMissing);
            // (re-using QuestionMissing for "required slot missing" semantically; a
            // future tail-additive variant SlotRequiredMissing could be added.)
        }
    }
    PredicateVerdict::Pass
}

// ── helpers ─────────────────────────────────────────────────────────────────

/// Han-script ratio: chars in CJK Unified Ideographs (U+4E00..=U+9FFF) divided
/// by total non-whitespace, non-punct chars. Returns 0.0 for empty denominator.
fn han_script_ratio(s: &str) -> f64 {
    let mut han = 0usize;
    let mut denom = 0usize;
    for c in s.chars() {
        if c.is_whitespace() || c.is_ascii_punctuation() {
            continue;
        }
        denom += 1;
        let code = c as u32;
        if (0x4E00..=0x9FFF).contains(&code) {
            han += 1;
        }
    }
    if denom == 0 {
        return 0.0;
    }
    han as f64 / denom as f64
}

/// ASCII alphanumeric ratio: same denominator, but counts a-z A-Z 0-9.
fn ascii_alpha_ratio(s: &str) -> f64 {
    let mut alpha = 0usize;
    let mut denom = 0usize;
    for c in s.chars() {
        if c.is_whitespace() || c.is_ascii_punctuation() {
            continue;
        }
        denom += 1;
        if c.is_ascii_alphanumeric() {
            alpha += 1;
        }
    }
    if denom == 0 {
        return 0.0;
    }
    alpha as f64 / denom as f64
}
