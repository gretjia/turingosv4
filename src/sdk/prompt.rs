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
    team_board: &str,
) -> String {
    let mut prompt = String::new();

    // Phase 6-emergent (Drucker-revised per user 2026-04-21): shared team
    // message board. State display of per-agent cumulative facts + recent
    // posts. Agents self-select role by reading + posting; no centrally-
    // enforced allowlist (C-034 clean, Hayek-Drucker hybrid).
    if !team_board.is_empty() {
        prompt.push_str("=== Team Board ===\n");
        prompt.push_str(team_board);
        if !team_board.ends_with('\n') { prompt.push('\n'); }
        prompt.push('\n');
    }

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
    // Art. IV ⟨q_o, a_o⟩: δ produces a next-state hint (q_o) and an action (a_o).
    // OPTIONAL wrapped form lets you signal q_o explicitly:
    //   {"q_delta":"<hint>","action":{<tool-schema-below>}}
    // where q_delta is a short state hint (e.g. "halt_soon", "continue_dfs",
    // "need_search"). If omitted, emit the flat action JSON directly — both
    // forms are accepted (backward compat).
    prompt.push_str("Optional wrapped form: {\"q_delta\":\"<state-hint>\",\"action\":{...tool JSON...}}\n");
    prompt.push_str("  q_delta is an OPTIONAL next-state hint (e.g. \"halt_soon\", \"continue_dfs\").\n");
    prompt.push_str("  You may also emit the inner action JSON directly — both forms work.\n");
    prompt.push_str("Schemas by tool:\n");
    // Phase 7 (TURING_STEP_ONLY=1): only `step` is available as the proof-
    // progression tool. Art. IV strict: δ writes one square. No monolithic
    // complete; no free-form append. Agents emit one Lean tactic per call,
    // and the oracle classifies the accumulated prefix as Complete /
    // PartialOk / Reject. Unchanged: search, invest, post.
    let step_only = std::env::var("TURING_STEP_ONLY").ok().as_deref() == Some("1");
    if step_only {
        prompt.push_str("  {\"tool\":\"step\",\"payload\":\"<one Lean tactic>\"}\n");
        prompt.push_str("    THE proof-progression tool. Submit ONE tactic (e.g. `intro h`,\n");
        prompt.push_str("    `rw [h₀]`, `linarith`, `induction' n with m IH`). The oracle\n");
        prompt.push_str("    elaborates (problem_statement ++ accumulated_tape ++ this_tactic):\n");
        prompt.push_str("      - all goals solved → OMEGA, run halts\n");
        prompt.push_str("      - goals remain, no type errors → tactic joins tape as Q_{t+1}\n");
        prompt.push_str("      - Lean errors → tape unchanged, try a different tactic\n");
        prompt.push_str("    Build the proof incrementally. Cite prior tape nodes by reading the\n");
        prompt.push_str("    === Current Chain === section; your next tactic operates on the\n");
        prompt.push_str("    proof state that already follows from those steps.\n");
    } else {
        prompt.push_str("  {\"tool\":\"step\",\"payload\":\"<one Lean tactic>\"}\n");
        prompt.push_str("    Phase 7 Art. IV δ-step: the system appends payload to tape, then\n");
        prompt.push_str("    Lean elaborates (problem + accumulated_tape + this_tactic).\n");
        prompt.push_str("    Goals-solved → OMEGA; partial-ok → tape grows; error → reject.\n");
        prompt.push_str("  {\"tool\":\"append\",\"payload\":\"<proof-step-text>\",\"node\":\"<optional-parent-id>\"}\n");
        prompt.push_str("    Raw scratch write (no oracle check). Use `step` instead when possible.\n");
        prompt.push_str("  {\"tool\":\"complete\",\"payload\":\"<tactics-only>\"}\n");
        prompt.push_str("    Legacy one-shot: full proof. Payload alone, then tape+payload fallback.\n");
    }
    prompt.push_str("  {\"tool\":\"search\",\"query\":\"<keyword>\"}\n");
    prompt.push_str("  {\"tool\":\"invest\",\"node\":\"<node-id>\",\"amount\":<number>,\"direction\":\"long|short\"}\n");
    prompt.push_str("    Bet on a tape node's quality (Art. II.2 price signal).\n");
    prompt.push_str("    direction=\"long\" buys YES shares (this node is on the winning path);\n");
    prompt.push_str("    direction=\"short\" buys NO shares (this node is a dead end).\n");
    prompt.push_str("    Use short to price-signal dissent — silence != disagreement.\n");
    prompt.push_str("    amount is coins deducted from your balance.\n");
    prompt.push_str("  {\"tool\":\"post\",\"payload\":\"<short message to team board>\"}\n");

    prompt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prompt_contains_no_example_values() {
        let prompt = build_agent_prompt("", "", "", &[], &[], 10000.0, "append, invest, search", "");
        assert!(!prompt.contains("50.0"), "No example amounts in prompt");
        assert!(!prompt.contains("100.0"), "No example amounts in prompt");
    }

    #[test]
    fn test_prompt_includes_balance() {
        let prompt = build_agent_prompt("", "", "", &[], &[], 5000.0, "", "");
        assert!(prompt.contains("5000"));
    }

    #[test]
    fn test_prompt_truncates_errors_to_3() {
        let errors: Vec<String> = (0..10).map(|i| format!("error {}", i)).collect();
        let prompt = build_agent_prompt("", "", "", &errors, &[], 0.0, "", "");
        assert!(prompt.contains("error 0"));
        assert!(prompt.contains("error 2"));
        assert!(!prompt.contains("error 3"));
    }

    #[test]
    fn test_prompt_surfaces_search_hits() {
        let hits: Vec<String> = vec!["thm_a.lean".into(), "thm_b.lean".into()];
        let prompt = build_agent_prompt("", "", "", &[], &hits, 0.0, "", "");
        assert!(prompt.contains("Recent Search Hits"));
        assert!(prompt.contains("thm_a.lean"));
    }

    #[test]
    fn test_prompt_surfaces_team_board() {
        let board = "Agent_0 balance=10040 (+40)\nAgent_3 balance=10030 (+30)\n";
        let prompt = build_agent_prompt("", "", "", &[], &[], 0.0, "", board);
        assert!(prompt.contains("Team Board"));
        assert!(prompt.contains("Agent_0 balance=10040"));
    }
}
