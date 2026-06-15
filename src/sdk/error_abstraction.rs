// Error abstraction for Art. II.1 broadcast.
//
// Constitutional basis: Art. II.1 requires "将这类典型错误 **抽象** 出来"
// and "绝不能把具体报错日志群发给所有人 — 那会造成灾难性的上下文污染" (C-022).
//
// This module maps raw Lean/parse error strings into a CLOSED enum with
// a FIXED label set. TopK broadcast in bus.rs emits label+count summaries
// only — never raw strings.
//
// C-012 freeze contract:
//   - CLASSIFIER_VERSION constant stamped on every artifact
//   - Enum variants are load-bearing; any change ⇒ version bump
//   - Fixture tests in #[cfg(test)] must stay green

/// Frozen classifier version. Bumped on every taxonomy change.
/// Stamped in PputResult.classifier_version per C-012.
/// `v2_2026-06-15-a`: de-Lean label migration — same 12 variants / same
/// classify match order, but `label()` now emits generic `err:tool_*` /
/// domain-agnostic strings (legacy `err:tactic_*` etc. recognized on read).
/// Forward-only; no historical artifact rewritten.
pub const CLASSIFIER_VERSION: &str = "v2_2026-06-15-a";

/// CLOSED taxonomy of Lean/parse error classes. No String carriers.
/// Art. II.1 mandate: bounded label space for broadcast.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OracleErrClass {
    // Tactic-family: bounded known-tactic list. Unknown tactic → TacticOther.
    TacticLinarith,
    TacticSimpNoProgress,
    TacticRingFailed,
    TacticNormNumFailed,
    TacticOther,
    // Structural
    UnknownConstant,
    UnsolvedGoals,
    UnexpectedToken,
    TypeMismatch,
    RewriteNoMatch,
    HeartbeatExceeded,
    // Catchall — NEVER carries String. All unmatched patterns collapse here.
    Other,
}

impl OracleErrClass {
    /// Stable terse label. Bounded set; safe for agent-visible prompt.
    ///
    /// De-Lean migration (2026-06-15): labels emit generic `err:tool_*` /
    /// domain-agnostic names going FORWARD. The Lean-specific tactic proper
    /// nouns (`linarith`/`simp`/`ring`/`norm_num`) and the Lean-ism
    /// `heartbeat` are dropped from the wire vocabulary. Legacy `err:tactic_*`
    /// / `err:heartbeat` / `err:unknown_const` / `err:unsolved_goals` strings
    /// are still RECOGNIZED on read (see `bus.rs::bus_classify`), so historical
    /// tape rows that carry the old `error_class` String still pass the
    /// finite-set shield. `error_class` rides the CAS as an opaque String, so
    /// no historical object is rewritten — this is a forward-only label swap.
    pub fn label(&self) -> &'static str {
        match self {
            OracleErrClass::TacticLinarith => "err:tool_method_arith",
            OracleErrClass::TacticSimpNoProgress => "err:tool_method_no_progress",
            OracleErrClass::TacticRingFailed => "err:tool_method_algebra",
            OracleErrClass::TacticNormNumFailed => "err:tool_method_numeric",
            OracleErrClass::TacticOther => "err:tool_method_other",
            OracleErrClass::UnknownConstant => "err:unknown_symbol",
            OracleErrClass::UnsolvedGoals => "err:unsolved_obligations",
            OracleErrClass::UnexpectedToken => "err:unexpected_token",
            OracleErrClass::TypeMismatch => "err:type_mismatch",
            OracleErrClass::RewriteNoMatch => "err:rewrite_no_match",
            OracleErrClass::HeartbeatExceeded => "err:resource_exceeded",
            OracleErrClass::Other => "err:other",
        }
    }

    /// Legacy `err:*` labels emitted by this classifier BEFORE the 2026-06-15
    /// de-Lean migration. Kept so any read/parse path (e.g. `bus.rs`'s
    /// finite-set shield) recognizes historical tape rows instead of collapsing
    /// them to `err:other`. NEVER emitted going forward; recognition-on-read
    /// only. Index-aligned with `label()` variant order is NOT required — this
    /// is a membership set, not a positional encoding.
    pub fn legacy_labels() -> &'static [&'static str] {
        &[
            "err:tactic_linarith",
            "err:tactic_simp_noprog",
            "err:tactic_ring",
            "err:tactic_norm_num",
            "err:tactic_other",
            "err:unknown_const",
            "err:unsolved_goals",
            "err:heartbeat",
        ]
    }

    /// Ordinal for tiebreak in TopK sort (lower = first).
    pub fn ordinal(&self) -> u8 {
        match self {
            OracleErrClass::TacticLinarith => 0,
            OracleErrClass::TacticSimpNoProgress => 1,
            OracleErrClass::TacticRingFailed => 2,
            OracleErrClass::TacticNormNumFailed => 3,
            OracleErrClass::TacticOther => 4,
            OracleErrClass::UnknownConstant => 5,
            OracleErrClass::UnsolvedGoals => 6,
            OracleErrClass::UnexpectedToken => 7,
            OracleErrClass::TypeMismatch => 8,
            OracleErrClass::RewriteNoMatch => 9,
            OracleErrClass::HeartbeatExceeded => 10,
            OracleErrClass::Other => 11,
        }
    }
}

/// Classify raw Lean oracle output (combined stdout+stderr) into a bounded class.
/// Match order is authoritative; first match wins. Falls back to `Other`.
pub fn classify_lean_error(combined: &str) -> OracleErrClass {
    // Normalize to lowercase for matching (patterns below use lowercase).
    let c = combined.to_lowercase();
    // Structural error patterns (higher-specificity first)
    if c.contains("unknown constant")
        || c.contains("unknownidentifier")
        || c.contains("unknown identifier")
    {
        return OracleErrClass::UnknownConstant;
    }
    if c.contains("unexpected token") {
        return OracleErrClass::UnexpectedToken;
    }
    if c.contains("type mismatch") {
        return OracleErrClass::TypeMismatch;
    }
    if c.contains("did not find an occurrence") {
        return OracleErrClass::RewriteNoMatch;
    }
    if c.contains("maxheartbeats") || c.contains("heartbeat") {
        return OracleErrClass::HeartbeatExceeded;
    }
    // Tactic-failure patterns (ordered, specific → generic)
    if c.contains("`linarith` failed") || c.contains("linarith failed") {
        return OracleErrClass::TacticLinarith;
    }
    if c.contains("`simp` made no progress") || c.contains("simp made no progress") {
        return OracleErrClass::TacticSimpNoProgress;
    }
    if c.contains("`ring`") && c.contains("failed") {
        return OracleErrClass::TacticRingFailed;
    }
    if c.contains("`norm_num`") || c.contains("norm_num failed") {
        return OracleErrClass::TacticNormNumFailed;
    }
    if c.contains("tactic") && c.contains("failed") {
        return OracleErrClass::TacticOther;
    }
    if c.contains("unsolved goals") {
        return OracleErrClass::UnsolvedGoals;
    }
    OracleErrClass::Other
}

/// Classify a parse-fail reason string.
pub fn classify_parse_error(_reason: &str) -> OracleErrClass {
    // Parse failures are themselves "unexpected token" class from agents' perspective.
    OracleErrClass::UnexpectedToken
}

// ── Tests ───────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixture_unknown_constant() {
        let s = "<stdin>:20:13: error(lean.unknownIdentifier): Unknown constant `Int.abs_le.mp`";
        assert_eq!(classify_lean_error(s), OracleErrClass::UnknownConstant);
    }

    #[test]
    fn fixture_rewrite_no_match() {
        let s = "error: Tactic `rewrite` failed: Did not find an occurrence of the pattern";
        assert_eq!(classify_lean_error(s), OracleErrClass::RewriteNoMatch);
    }

    #[test]
    fn fixture_linarith_failed() {
        let s = "error: linarith failed to find a contradiction";
        assert_eq!(classify_lean_error(s), OracleErrClass::TacticLinarith);
    }

    #[test]
    fn fixture_simp_no_progress() {
        let s = "<stdin>:17:2: error: `simp` made no progress";
        assert_eq!(classify_lean_error(s), OracleErrClass::TacticSimpNoProgress);
    }

    #[test]
    fn fixture_type_mismatch() {
        let s = "error: Type mismatch: The argument ...";
        assert_eq!(classify_lean_error(s), OracleErrClass::TypeMismatch);
    }

    #[test]
    fn fixture_unsolved_goals() {
        let s = "error: unsolved goals\n⊢ True";
        assert_eq!(classify_lean_error(s), OracleErrClass::UnsolvedGoals);
    }

    #[test]
    fn fixture_unexpected_token() {
        let s = "<stdin>:11:19: error: unexpected token 'by'; expected '{' or tactic";
        assert_eq!(classify_lean_error(s), OracleErrClass::UnexpectedToken);
    }

    #[test]
    fn fixture_other_catchall() {
        let s = "<stdin>:5:5: error: some unprecedented never-before-seen error";
        assert_eq!(classify_lean_error(s), OracleErrClass::Other);
    }

    #[test]
    fn labels_are_unique_and_stable() {
        let all = [
            OracleErrClass::TacticLinarith,
            OracleErrClass::TacticSimpNoProgress,
            OracleErrClass::TacticRingFailed,
            OracleErrClass::TacticNormNumFailed,
            OracleErrClass::TacticOther,
            OracleErrClass::UnknownConstant,
            OracleErrClass::UnsolvedGoals,
            OracleErrClass::UnexpectedToken,
            OracleErrClass::TypeMismatch,
            OracleErrClass::RewriteNoMatch,
            OracleErrClass::HeartbeatExceeded,
            OracleErrClass::Other,
        ];
        let labels: std::collections::HashSet<&str> = all.iter().map(|c| c.label()).collect();
        assert_eq!(labels.len(), all.len(), "labels must be unique");
        let ordinals: std::collections::HashSet<u8> = all.iter().map(|c| c.ordinal()).collect();
        assert_eq!(ordinals.len(), all.len(), "ordinals must be unique");
    }

    #[test]
    fn classifier_version_is_stamped() {
        assert_eq!(CLASSIFIER_VERSION, "v2_2026-06-15-a");
    }

    #[test]
    fn labels_are_generic_no_lean_naming() {
        // De-Lean: go-forward labels must not carry Lean tactic proper nouns
        // or the Lean-ism "heartbeat".
        let all = [
            OracleErrClass::TacticLinarith,
            OracleErrClass::TacticSimpNoProgress,
            OracleErrClass::TacticRingFailed,
            OracleErrClass::TacticNormNumFailed,
            OracleErrClass::TacticOther,
            OracleErrClass::UnknownConstant,
            OracleErrClass::UnsolvedGoals,
            OracleErrClass::UnexpectedToken,
            OracleErrClass::TypeMismatch,
            OracleErrClass::RewriteNoMatch,
            OracleErrClass::HeartbeatExceeded,
            OracleErrClass::Other,
        ];
        for c in all {
            let l = c.label();
            assert!(!l.contains("tactic"), "label still Lean-named: {l}");
            assert!(!l.contains("linarith"), "label still Lean-named: {l}");
            assert!(!l.contains("simp"), "label still Lean-named: {l}");
            assert!(!l.contains("norm_num"), "label still Lean-named: {l}");
            assert!(!l.contains("heartbeat"), "label still Lean-named: {l}");
        }
    }

    #[test]
    fn legacy_labels_recognized_on_read() {
        // The legacy set is the historical wire vocabulary the read-path shield
        // must still accept (bus.rs::bus_classify mirrors this).
        let legacy = OracleErrClass::legacy_labels();
        assert!(legacy.contains(&"err:tactic_linarith"));
        assert!(legacy.contains(&"err:heartbeat"));
        assert!(legacy.contains(&"err:unknown_const"));
        // Legacy strings are NOT in the go-forward emit set.
        let forward: std::collections::HashSet<&str> = [
            OracleErrClass::TacticLinarith,
            OracleErrClass::HeartbeatExceeded,
            OracleErrClass::UnknownConstant,
        ]
        .iter()
        .map(|c| c.label())
        .collect();
        for l in legacy {
            assert!(!forward.contains(l), "legacy label leaked into emit set: {l}");
        }
    }
}
