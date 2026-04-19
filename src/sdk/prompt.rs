// Tier 2: Minimal prompt template — state-only, no examples
// Constitutional basis: Art. III.2 (encapsulate details, progressive disclosure)
// V3L-40: no example value anchoring. V3L-17: don't truncate reasoning.

/// Build the agent prompt from pure state.
///
/// Philosophy: "Gravity doesn't explain itself to apples."
/// - No rules explanation (V3L-39: LLMs follow incentives, not explanations)
/// - No example values (V3L-40: examples become anchors)
/// - State only: what exists, what's available, what's your balance
pub fn build_agent_prompt(
    chain_so_far: &str,
    skill: &str,
    market_ticker: &str,
    recent_errors: &[String],
    recent_search_hits: &[String],
    balance: f64,
    tools_description: &str,
) -> String {
    let mut prompt = String::new();

    // Current state (what the agent sees)
    if !chain_so_far.is_empty() {
        prompt.push_str("=== Current Chain ===\n");
        prompt.push_str(chain_so_far);
        prompt.push_str("\n\n");
    }

    // Agent's skill/role (Librarian-compressed DNA)
    if !skill.is_empty() {
        prompt.push_str("=== Your Skill ===\n");
        prompt.push_str(skill);
        prompt.push_str("\n\n");
    }

    // Market prices (Art. II.2: broadcast price signals)
    if !market_ticker.is_empty() {
        prompt.push_str("=== Market ===\n");
        prompt.push_str(market_ticker);
        prompt.push_str("\n\n");
    }

    // Recent errors (Art. II.1: broadcast typical errors)
    if !recent_errors.is_empty() {
        prompt.push_str("=== Recent Errors ===\n");
        for err in recent_errors.iter().take(3) {
            prompt.push_str("- ");
            prompt.push_str(err);
            prompt.push('\n');
        }
        prompt.push('\n');
    }

    // Art. III.2 progressive disclosure: surface recent search hits so the
    // search tool is not a write-only sink (F-2026-04-19-02).
    if !recent_search_hits.is_empty() {
        prompt.push_str("=== Recent Search Hits ===\n");
        for h in recent_search_hits.iter().take(5) {
            prompt.push_str("- ");
            prompt.push_str(h);
            prompt.push('\n');
        }
        prompt.push('\n');
    }

    // Balance (agent's resource awareness)
    prompt.push_str(&format!("Balance: {:.0} Coins\n\n", balance));

    // Available tools
    prompt.push_str("=== Tools ===\n");
    prompt.push_str(tools_description);
    prompt.push_str("\n\n");

    // Output format (C-009: explicit schema; V3L-40: no value anchors, only field shape)
    prompt.push_str("=== Output ===\n");
    prompt.push_str("Respond with exactly one <action>{JSON}</action> block. No prose outside.\n");
    prompt.push_str("Schemas by tool:\n");
    prompt.push_str("  {\"tool\":\"append\",\"payload\":\"<proof-step-text>\",\"node\":\"<optional-parent-id>\"}\n");
    prompt.push_str("    Optional scratch space (tape Q_t). Use only if you cannot one-shot the proof.\n");
    prompt.push_str("  {\"tool\":\"complete\",\"payload\":\"<tactics-only>\"}\n");
    prompt.push_str("    Verified as payload alone first. If rejected and tape has nodes, we retry\n");
    prompt.push_str("    with (tape joined by \\n) + payload. Either path counts as success.\n");
    prompt.push_str("    payload = tactics after `:= by`. MUST NOT re-declare the goal.\n");
    prompt.push_str("    Indent as body of `by`; multi-line allowed.\n");
    prompt.push_str("  {\"tool\":\"search\",\"query\":\"<keyword>\"}\n");
    prompt.push_str("  {\"tool\":\"invest\",\"node\":\"<node-id>\",\"amount\":<number>}\n");

    prompt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prompt_contains_no_example_values() {
        // V3L-40: no hardcoded example amounts that become anchors
        let prompt = build_agent_prompt("", "", "", &[], &[], 10000.0, "append, invest, search");
        assert!(!prompt.contains("50.0"), "No example amounts in prompt");
        assert!(!prompt.contains("100.0"), "No example amounts in prompt");
    }

    #[test]
    fn test_prompt_includes_balance() {
        let prompt = build_agent_prompt("", "", "", &[], &[], 5000.0, "");
        assert!(prompt.contains("5000"));
    }

    #[test]
    fn test_prompt_truncates_errors_to_3() {
        let errors: Vec<String> = (0..10).map(|i| format!("error {}", i)).collect();
        let prompt = build_agent_prompt("", "", "", &errors, &[], 0.0, "");
        assert!(prompt.contains("error 0"));
        assert!(prompt.contains("error 2"));
        assert!(!prompt.contains("error 3"));
    }

    #[test]
    fn test_prompt_surfaces_search_hits() {
        let hits: Vec<String> = vec!["thm_a.lean".into(), "thm_b.lean".into()];
        let prompt = build_agent_prompt("", "", "", &[], &hits, 0.0, "");
        assert!(prompt.contains("Recent Search Hits"));
        assert!(prompt.contains("thm_a.lean"));
    }
}
