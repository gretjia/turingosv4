// Tier 2: Minimal prompt template — state-only, no examples
// Constitutional basis: Art. III.2 (encapsulate details, progressive disclosure)
// V3L-40: no example value anchoring. V3L-17: don't truncate reasoning.

/// Session #34 2026-05-10 — prompt-variant experiment harness.
///
/// Reads `TURINGOS_PROMPT_VARIANT` env var (default = `v0` = unchanged
/// baseline). Each variant injects extra prompt sections to test
/// hypotheses about tactic-search-strategy collapse observed in M0
/// 2026-05-10 batch evidence.
///
/// Variants:
/// - `v0` (default) — control; current v4 prompt unchanged.
/// - `v1` — drop unused tools (`invest`/`search`/`post`) from schema; M0
///   evidence shows agent never calls them at N=1 with deepseek-chat.
/// - `v2` — `=== Tactic Search Guidance ===` section nudging structurally
///   different tactic families on reject.
/// - `v3` — v3-style explicit-LAWS block adapted for v4 reality (LAW 1/2/3
///   re budget + reject + diversity; "what makes a step worth submitting"
///   criteria). Per user "你可以根据...Turing OS V3的Prompt尝试...".
/// - `v4` — `v2` + dynamic `=== Last Rejected Tactics (DO NOT REPEAT) ===`
///   echo of the last 3 entries from `recent_errors` (recency-targeted).
///
/// Plan: `handover/alignment/PROMPT_VARIANT_EXPERIMENT_PLAN_2026-05-10.md`.
/// All variants are additive + opt-in; default behavior is bit-identical
/// to the pre-session-#34 prompt.
fn current_prompt_variant() -> String {
    std::env::var("TURINGOS_PROMPT_VARIANT")
        .ok()
        .map(|s| s.to_lowercase())
        .filter(|s| matches!(s.as_str(), "v0" | "v1" | "v2" | "v3" | "v4"))
        .unwrap_or_else(|| "v0".into())
}

/// Build the agent prompt from pure state.
///
/// Philosophy: "Gravity doesn't explain itself to apples."
/// - No rules explanation (V3L-39: LLMs follow incentives, not explanations)
/// - No example values (V3L-40: examples become anchors)
/// - State only: what exists, what's available, what's your balance
/// TB-N1-AGENT-ECONOMY A2 (session #35 2026-05-10): replaced
/// `balance: f64` with `econ_position: &str`. Caller (typically
/// `evaluator.rs`) renders the position block via
/// `crate::sdk::econ_position::render_econ_position(&q, &agent_id)`
/// from canonical `EconomicState`. Empty string suppresses the
/// `=== Your Economic Position ===` block (back-compat for tests + any
/// caller without sequencer access).
pub fn build_agent_prompt(
    chain_so_far: &str,
    skill: &str,
    market_ticker: &str,
    recent_errors: &[String],
    recent_search_hits: &[String],
    econ_position: &str,
    tools_description: &str,
    team_board: &str,
) -> String {
    let variant = current_prompt_variant();
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

    // TB-N1-AGENT-ECONOMY A2: agent's full economic position block,
    // rendered by caller from canonical EconomicState (balances_t /
    // stakes_t / claims_t / reputations_t). Closes the
    // perception gap from session #35 smoke evidence (n=1 economy
    // landed structurally but invisible to agent at prompt layer).
    // Pre-A2 a single `Balance: N Coins` line was rendered here; the
    // legacy contract is preserved for callers that pass an empty
    // string (block is suppressed entirely in that case so unit tests
    // and minimal callers stay simple).
    if !econ_position.is_empty() {
        prompt.push_str("=== Your Economic Position ===\n");
        prompt.push_str(econ_position);
        if !econ_position.ends_with('\n') { prompt.push('\n'); }
        prompt.push('\n');
    }

    // Available tools
    prompt.push_str("=== Tools ===\n");
    prompt.push_str(tools_description);
    prompt.push_str("\n\n");

    // ── Session #34 prompt-variant injection (pre-Output sections) ──
    // V2/V3/V4 inject tactic-search guidance BEFORE the Output schema so the
    // model reads the guidance with the same recency as the schema. V4 also
    // echoes the most-recent rejected tactics for direct don't-repeat targeting.
    inject_variant_pre_output(&mut prompt, &variant, recent_errors);

    // Output format (C-009: explicit schema; V3L-40: no value anchors, only field shape)
    prompt.push_str("=== Output ===\n");
    prompt.push_str("Respond with exactly one <action>{JSON}</action> block. No prose outside.\n");
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
    // Session #34 V1 (tool clean) drops unused tools from the schema. M0
    // 2026-05-10 evidence: agent never invokes search/invest/post at N=1.
    if variant != "v1" {
        prompt.push_str("  {\"tool\":\"search\",\"query\":\"<keyword>\"}\n");
        prompt.push_str("  {\"tool\":\"invest\",\"node\":\"<node-id>\",\"amount\":<number>,\"direction\":\"long|short\"}\n");
        prompt.push_str("    Bet on a tape node's quality (Art. II.2 price signal).\n");
        prompt.push_str("    direction=\"long\" buys YES shares (this node is on the winning path);\n");
        prompt.push_str("    direction=\"short\" buys NO shares (this node is a dead end).\n");
        prompt.push_str("    Use short to price-signal dissent — silence != disagreement.\n");
        prompt.push_str("    amount is coins deducted from your balance.\n");
        prompt.push_str("  {\"tool\":\"post\",\"payload\":\"<short message to team board>\"}\n");
    }

    prompt
}

/// Session #34 prompt-variant injection helper. V0/V1 are no-ops here
/// (V0 = default; V1 only affects the schema list inside `build_agent_prompt`).
/// V2/V3/V4 inject extra prompt sections immediately before the Output block.
fn inject_variant_pre_output(prompt: &mut String, variant: &str, recent_errors: &[String]) {
    match variant {
        "v2" => {
            prompt.push_str("=== Tactic Search Guidance ===\n");
            prompt.push_str(
                "If your previous tactic was rejected, try a STRUCTURALLY DIFFERENT\n\
                 tactic family. Do not repeat near-identical failed tactics — the\n\
                 budget shrinks on every submission whether accepted or rejected.\n\
                 Examples of structurally distinct families:\n\
                 \u{0020}\u{0020}arithmetic decision: omega / linarith / nlinarith / polyrith\n\
                 \u{0020}\u{0020}algebraic rewrite:   ring / field_simp / norm_num / push_cast\n\
                 \u{0020}\u{0020}simplification:      simp / aesop / decide\n\
                 \u{0020}\u{0020}decomposition:       have ... := by ...; cases ...; induction ...\n\n",
            );
        }
        "v3" => {
            prompt.push_str("=== Operating Laws ===\n");
            prompt.push_str(
                "LAW 1: Each `step` submission consumes 1 of 200 budgeted attempts,\n\
                 \u{0020}\u{0020}\u{0020}\u{0020}whether accepted or rejected.\n\
                 LAW 2: A REJECTED step does not advance the proof; it only burns budget.\n\
                 LAW 3: If two consecutive steps were rejected, switch to a structurally\n\
                 \u{0020}\u{0020}\u{0020}\u{0020}different tactic family (do not repeat the same approach).\n\n\
                 === What makes a step worth submitting ===\n\
                 \u{0020}\u{0020}\u{2713} Logically follows from the proof state in === Current Chain ===\n\
                 \u{0020}\u{0020}\u{2713} Uses a tactic family appropriate for the goal type\n\
                 \u{0020}\u{0020}\u{2713} Is atomic — one tactic, not a chain of `<;>` composites\n\
                 \u{0020}\u{0020}\u{2717} Repeats a tactic that already rejected\n\
                 \u{0020}\u{0020}\u{2717} Hand-waving (`sorry`, `admit`, `???`) is forbidden by the oracle\n\n",
            );
        }
        "v4" => {
            // V4 = V2 base + dynamic recent-rejects echo.
            prompt.push_str("=== Tactic Search Guidance ===\n");
            prompt.push_str(
                "If your previous tactic was rejected, try a STRUCTURALLY DIFFERENT\n\
                 tactic family. Do not repeat near-identical failed tactics — the\n\
                 budget shrinks on every submission whether accepted or rejected.\n\
                 Examples of structurally distinct families:\n\
                 \u{0020}\u{0020}arithmetic decision: omega / linarith / nlinarith / polyrith\n\
                 \u{0020}\u{0020}algebraic rewrite:   ring / field_simp / norm_num / push_cast\n\
                 \u{0020}\u{0020}simplification:      simp / aesop / decide\n\
                 \u{0020}\u{0020}decomposition:       have ... := by ...; cases ...; induction ...\n\n",
            );
            if !recent_errors.is_empty() {
                prompt.push_str("=== Last Rejected Tactics (DO NOT REPEAT) ===\n");
                for err in recent_errors.iter().take(3) {
                    prompt.push_str("- ");
                    prompt.push_str(err);
                    prompt.push('\n');
                }
                prompt.push('\n');
            }
        }
        _ => {} // v0 / v1 / unknown → no extra section here
    }
}

// PPUT-CCL B6 runtime PPUT-context-leak gate lives in `prompt_guard.rs`
// (separate module). The B5 conformance test `test_no_pput_in_agent_prompt`
// scans this file specifically — keeping the gate elsewhere preserves
// prompt.rs purity while the runtime defense remains active.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prompt_contains_no_example_values() {
        let prompt = build_agent_prompt("", "", "", &[], &[], "Balance: 10000 \u{03BC}Coin", "append, invest, search", "");
        assert!(!prompt.contains("50.0"), "No example amounts in prompt");
        assert!(!prompt.contains("100.0"), "No example amounts in prompt");
    }

    #[test]
    fn test_prompt_includes_balance() {
        let prompt = build_agent_prompt("", "", "", &[], &[], "Balance: 5000 \u{03BC}Coin", "", "");
        assert!(prompt.contains("5000"));
    }

    /// TB-N1-AGENT-ECONOMY A2 (session #35): econ_position block renders
    /// under canonical heading `=== Your Economic Position ===` when
    /// non-empty, and is suppressed entirely when empty (back-compat for
    /// tests + minimal callers without sequencer access).
    #[test]
    fn test_econ_position_block_renders_under_heading() {
        let block = "Balance: 1000000 \u{03BC}Coin (1.00 Coins)\n\
                     Active stakes: 0 \u{03BC}Coin across 0 pending WorkTx\n\
                     Pending claims: 0 \u{03BC}Coin (earned, not yet settled)\n\
                     Reputation: 0\n";
        let prompt = build_agent_prompt("", "", "", &[], &[], block, "", "");
        assert!(
            prompt.contains("=== Your Economic Position ===\n"),
            "block must render under canonical heading"
        );
        assert!(
            prompt.contains("Balance: 1000000 \u{03BC}Coin (1.00 Coins)"),
            "block body must be embedded verbatim"
        );
        assert!(
            prompt.contains("Active stakes: 0 \u{03BC}Coin across 0 pending WorkTx"),
            "block must include active-stake line"
        );
    }

    #[test]
    fn test_empty_econ_position_suppresses_block() {
        let prompt = build_agent_prompt("", "", "", &[], &[], "", "", "");
        assert!(
            !prompt.contains("=== Your Economic Position ==="),
            "empty econ_position must suppress the block entirely"
        );
        assert!(
            !prompt.contains("Balance:"),
            "empty econ_position must not render any balance line"
        );
    }

    #[test]
    fn test_prompt_truncates_errors_to_3() {
        let errors: Vec<String> = (0..10).map(|i| format!("error {}", i)).collect();
        let prompt = build_agent_prompt("", "", "", &errors, &[], "", "", "");
        assert!(prompt.contains("error 0"));
        assert!(prompt.contains("error 2"));
        assert!(!prompt.contains("error 3"));
    }

    #[test]
    fn test_prompt_surfaces_search_hits() {
        let hits: Vec<String> = vec!["thm_a.lean".into(), "thm_b.lean".into()];
        let prompt = build_agent_prompt("", "", "", &[], &hits, "", "", "");
        assert!(prompt.contains("Recent Search Hits"));
        assert!(prompt.contains("thm_a.lean"));
    }

    #[test]
    fn test_prompt_surfaces_team_board() {
        let board = "Agent_0 balance=10040 (+40)\nAgent_3 balance=10030 (+30)\n";
        let prompt = build_agent_prompt("", "", "", &[], &[], "", "", board);
        assert!(prompt.contains("Team Board"));
        assert!(prompt.contains("Agent_0 balance=10040"));
    }

    // Session #34 prompt-variant tests. Each variant exercise uses
    // `std::env::set_var` + `remove_var` directly. NOT thread-safe under
    // `cargo test --workspace`; per `feedback_env_var_test_lock` we serialize
    // via a static mutex.
    mod variant_tests {
        use super::*;
        use std::sync::Mutex;

        static ENV_LOCK: Mutex<()> = Mutex::new(());

        fn with_variant<F: FnOnce()>(variant: Option<&str>, body: F) {
            let _guard = ENV_LOCK.lock().expect("env lock");
            match variant {
                Some(v) => std::env::set_var("TURINGOS_PROMPT_VARIANT", v),
                None => std::env::remove_var("TURINGOS_PROMPT_VARIANT"),
            }
            body();
            std::env::remove_var("TURINGOS_PROMPT_VARIANT");
        }

        #[test]
        fn v0_default_lists_legacy_tools() {
            with_variant(None, || {
                let p = build_agent_prompt("", "", "", &[], &[], "", "", "");
                assert!(p.contains("\"invest\""), "V0 default lists the invest schema");
                assert!(!p.contains("Tactic Search Guidance"));
                assert!(!p.contains("Operating Laws"));
                assert!(!p.contains("Last Rejected Tactics"));
            });
        }

        #[test]
        fn v1_drops_unused_tools_from_schema() {
            with_variant(Some("v1"), || {
                let p = build_agent_prompt("", "", "", &[], &[], "", "", "");
                assert!(!p.contains("\"invest\""), "V1 drops the invest schema entry");
                assert!(!p.contains("\"search\""), "V1 drops the search schema entry");
                assert!(!p.contains("\"post\""), "V1 drops the post schema entry");
                assert!(p.contains("\"step\""), "V1 keeps the step schema entry");
            });
        }

        #[test]
        fn v2_injects_tactic_search_guidance() {
            with_variant(Some("v2"), || {
                let p = build_agent_prompt("", "", "", &[], &[], "", "", "");
                assert!(p.contains("Tactic Search Guidance"));
                assert!(p.contains("STRUCTURALLY DIFFERENT"));
                assert!(p.contains("nlinarith"));
                assert!(!p.contains("Operating Laws"));
            });
        }

        #[test]
        fn v3_injects_v3_style_laws_and_criteria() {
            with_variant(Some("v3"), || {
                let p = build_agent_prompt("", "", "", &[], &[], "", "", "");
                assert!(p.contains("Operating Laws"));
                assert!(p.contains("LAW 1"));
                assert!(p.contains("LAW 2"));
                assert!(p.contains("LAW 3"));
                assert!(p.contains("worth submitting"));
            });
        }

        #[test]
        fn v4_injects_guidance_plus_recent_rejects() {
            with_variant(Some("v4"), || {
                let errs: Vec<String> =
                    vec!["nlinarith (rejected)".into(), "linarith (rejected)".into()];
                let p = build_agent_prompt("", "", "", &errs, &[], "", "", "");
                assert!(p.contains("Tactic Search Guidance"));
                assert!(p.contains("Last Rejected Tactics"));
                assert!(p.contains("nlinarith (rejected)"));
                assert!(p.contains("linarith (rejected)"));
            });
        }

        #[test]
        fn v4_omits_rejects_block_when_no_errors() {
            with_variant(Some("v4"), || {
                let p = build_agent_prompt("", "", "", &[], &[], "", "", "");
                assert!(p.contains("Tactic Search Guidance"));
                assert!(!p.contains("Last Rejected Tactics"));
            });
        }

        #[test]
        fn unknown_variant_falls_back_to_default() {
            with_variant(Some("vNINE"), || {
                let p = build_agent_prompt("", "", "", &[], &[], "", "", "");
                assert!(p.contains("\"invest\""), "unknown variant defaults to V0");
                assert!(!p.contains("Tactic Search Guidance"));
            });
        }
    }
}
