//! TRACE_MATRIX FC1a-budget_gate: TDMA-Bounded tokenizer.
//!
//! For RC1 we use a conservative 4-chars-per-token heuristic (GPT-family rule of
//! thumb). This is sufficient for budget gating because:
//!   1. Token budgets are hard CEILINGS, not exact accounting.
//!   2. The heuristic over-estimates for short ASCII and is roughly correct for
//!      English mixed with code, so we err on the safe side of the budget.
//!   3. Replacing the heuristic with a real BPE/SentencePiece tokenizer (e.g.
//!      tiktoken-rs) is a single-module swap behind the `Tokenizer` API.
//!
//! KILL-tdma-3: payload byte length is NEVER substituted for token count in
//! TDMA modules — all token math goes through this module.
//!
//! On-disk §8: handover/directives/2026-05-22_TDMA_BOUNDED_RC1_DIRECTIVE_AND_§8.md

use serde::Serialize;

/// Token-budget tokenizer (directive §7).
/// TRACE_MATRIX FC1a-budget_gate: Single API for counting / slicing tokens.
/// Concrete impl behind a struct (not a trait) until a second backend is added.
#[derive(Debug, Default, Clone)]
pub struct Tokenizer;

impl Tokenizer {
    /// TRACE_MATRIX FC1a-budget_gate: Constructor.
    pub fn new() -> Self {
        Tokenizer
    }

    /// Estimate token count for arbitrary text.
    /// 4 chars per token, rounded up so empty text -> 0 and 1 char -> 1.
    /// TRACE_MATRIX FC1a-budget_gate + KILL-tdma-3: Replaces any inline
    /// byte-length proxy in TDMA modules.
    pub fn count_text(&self, s: &str) -> usize {
        let chars = s.chars().count();
        if chars == 0 {
            0
        } else {
            (chars + 3) / 4
        }
    }

    /// Estimate token count for a JSON-serializable value.
    /// TRACE_MATRIX FC1a-budget_gate: Schema-preserving objects count via
    /// canonical JSON serialization.
    pub fn count_json<T: Serialize>(&self, v: &T) -> usize {
        serde_json::to_string(v)
            .map(|s| self.count_text(&s))
            .unwrap_or(0)
    }

    /// Return the prefix of `s` containing approximately the first `n` tokens.
    /// Used by the state-first parser to scan only the first B_HEADER_SCAN tokens.
    /// TRACE_MATRIX FC1a-budget_gate: Bounded-prefix policy enforcement helper.
    pub fn first_tokens(&self, s: &str, n: usize) -> String {
        let max_chars = n.saturating_mul(4);
        s.chars().take(max_chars).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_text_empty() {
        assert_eq!(Tokenizer::new().count_text(""), 0);
    }

    #[test]
    fn count_text_short() {
        // "abc" -> 3 chars -> ceil(3/4) = 1 token
        assert_eq!(Tokenizer::new().count_text("abc"), 1);
    }

    #[test]
    fn count_text_8_chars_2_tokens() {
        assert_eq!(Tokenizer::new().count_text("abcdefgh"), 2);
    }

    #[test]
    fn count_json_struct() {
        #[derive(Serialize)]
        struct X {
            a: u32,
            b: String,
        }
        let x = X {
            a: 1,
            b: "hello".into(),
        };
        let t = Tokenizer::new();
        let n = t.count_json(&x);
        assert!(n > 0);
        // {"a":1,"b":"hello"} -> 19 chars -> 5 tokens
        assert_eq!(n, 5);
    }

    #[test]
    fn first_tokens_takes_prefix() {
        let s = "a".repeat(40); // 40 chars -> 10 tokens
        let t = Tokenizer::new();
        let prefix = t.first_tokens(&s, 5);
        // 5 tokens * 4 chars = 20 char prefix
        assert_eq!(prefix.chars().count(), 20);
    }
}
