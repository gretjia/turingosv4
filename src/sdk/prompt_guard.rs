// PPUT-CCL Phase B B6 — runtime PPUT-context-leak gate.
//
// Spec:
//   handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md § B6
//   handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md § 3 #6
//     (test_no_pput_in_agent_prompt)
//
// Why this lives in a separate file from prompt.rs:
//   The B5 conformance test `test_no_pput_in_agent_prompt` scans
//   `src/sdk/prompt.rs` for PPUT-related substrings. The gate function
//   below LEGITIMATELY contains them (its job is to detect them). Hosting
//   it in a separate module keeps prompt.rs pure and makes the gate's
//   role explicit. This is the legitimate-exception path PREREG § 3 #6
//   notes ("Whitelist: dashboard / logging / aggregator code paths only").
//
// Defense-in-depth pairing:
//   - STATIC (B5 `test_no_pput_in_agent_prompt`): scans prompt.rs source
//     to ensure no PPUT scalars are baked into prompt template strings.
//   - RUNTIME (this file, `assert_no_metric_leak`): scans the FINAL
//     ASSEMBLED prompt at every LLM-call boundary, defending against any
//     state surface (tape contents, board posts, search hits, learned
//     memory) that could inject a PPUT value at runtime even when the
//     prompt builder source is clean.

/// Forbidden substrings — every variant of PPUT mention an attacker
/// (or honest mistake) might inject. Case-insensitive matching is
/// applied at the call site.
///
/// Order: most specific → least specific. The first match wins for
/// diagnostic clarity (specific names give better error messages than
/// generic "PPUT" matches).
const FORBIDDEN_SUBSTRINGS: &[&str] = &[
    "pput_m_verified",
    "pput_verified",
    "pput_runtime",
    "PPUT-M",
    "H-VPPUT",
    "WBCG_PPUT",
    "WBCG",
    "pput=",
];

/// Scan an assembled agent prompt for PPUT-related substrings just
/// before it crosses the LLM-call boundary. Panics with
/// `PPUT_CONTEXT_LEAK_DETECTED` on any match.
///
/// Per PREREG § 3 #6 / plan B6, this is a hard BLOCKER, not a warning —
/// continuing the LLM call after a leak would emit a poisoned
/// measurement. Aborting deterministically is the only honest response.
pub fn assert_no_metric_leak(prompt: &str) {
    let lower = prompt.to_lowercase();
    for needle in FORBIDDEN_SUBSTRINGS {
        let needle_lower = needle.to_lowercase();
        if lower.contains(&needle_lower) {
            let idx = lower.find(&needle_lower).unwrap();
            let start = idx.saturating_sub(40);
            let end = (idx + needle_lower.len() + 40).min(prompt.len());
            // Preview from ORIGINAL prompt so capitalization is preserved.
            let preview: String = prompt.chars().skip(start).take(end - start).collect();
            panic!(
                "PPUT_CONTEXT_LEAK_DETECTED: forbidden substring '{}' found \
                 in agent prompt at byte offset {}. PREREG § 3 #6 + plan B6: \
                 PPUT scalars MUST NOT enter agent context. Run aborted — \
                 emitting any LLM call after this point would poison the \
                 measurement. Span around match: ...{}...",
                needle, idx, preview
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_prompt_passes() {
        // A typical agent prompt (no PPUT references) must pass.
        let prompt = "=== Current Chain ===\n\
                      intro h\nlinarith\n\n\
                      === Tools ===\nstep, search, invest\n\n\
                      === Output ===\nRespond with one <action>{JSON}</action>";
        assert_no_metric_leak(prompt);
    }

    #[test]
    #[should_panic(expected = "PPUT_CONTEXT_LEAK_DETECTED")]
    fn test_pput_runtime_caught() {
        assert_no_metric_leak("Your goal is to maximize pput_runtime.");
    }

    #[test]
    #[should_panic(expected = "PPUT_CONTEXT_LEAK_DETECTED")]
    fn test_pput_verified_caught() {
        assert_no_metric_leak("Recent runs reported pput_verified = 0.0023");
    }

    #[test]
    #[should_panic(expected = "PPUT_CONTEXT_LEAK_DETECTED")]
    fn test_pput_m_verified_caught() {
        assert_no_metric_leak("display: pput_m_verified=2.3");
    }

    #[test]
    #[should_panic(expected = "PPUT_CONTEXT_LEAK_DETECTED")]
    fn test_h_vpput_caught() {
        assert_no_metric_leak("The North Star is H-VPPUT on heldout-54");
    }

    #[test]
    #[should_panic(expected = "PPUT_CONTEXT_LEAK_DETECTED")]
    fn test_wbcg_caught() {
        assert_no_metric_leak("WBCG must be > 0");
    }

    #[test]
    #[should_panic(expected = "PPUT_CONTEXT_LEAK_DETECTED")]
    fn test_pput_assignment_pattern_caught() {
        assert_no_metric_leak("current state: pput=0.0023, time=30s");
    }

    /// Case-insensitive match — defends against minor evasion via case shifts.
    #[test]
    #[should_panic(expected = "PPUT_CONTEXT_LEAK_DETECTED")]
    fn test_case_insensitive_match() {
        assert_no_metric_leak("previous run: PPut_VeRiFiEd was 0.5");
    }

    /// Fragment in middle of larger string still triggers.
    #[test]
    #[should_panic(expected = "PPUT_CONTEXT_LEAK_DETECTED")]
    fn test_pput_substring_in_larger_text() {
        let big = "lots of innocent text ".repeat(100)
            + "and somewhere here pput_verified appears "
            + &"more text ".repeat(100);
        assert_no_metric_leak(&big);
    }

    /// Empty prompt is trivially clean.
    #[test]
    fn test_empty_prompt_passes() {
        assert_no_metric_leak("");
    }
}
