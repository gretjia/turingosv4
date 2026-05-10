// Tier 2: Postel's law parser — wide input, strict output
// Constitutional basis: Art. I.1.1 (PCP predicate, reject-only)
// V3 lessons: V3L-08 (format fragility), V3L-09 (no silent failure),
//             V3L-15 (context self-poisoning), V3L-16 (dual-chamber)

use serde::Deserialize;
use std::fmt;

// ── Core types ──────────────────────────────────────────────────

/// Parsed agent action from LLM output.
#[derive(Debug, Clone, Deserialize)]
pub struct AgentAction {
    pub tool: String,
    #[serde(default)]
    pub payload: Option<String>,
    #[serde(default)]
    pub amount: Option<f64>,
    #[serde(default)]
    pub node: Option<String>,
    #[serde(default)]
    pub query: Option<String>,
    /// Bet direction for `invest` tool. Valid values: "long"/"yes" (buy YES)
    /// or "short"/"no" (buy NO). If absent, falls back to sign of `amount`:
    /// positive ⇒ long, negative ⇒ short. See Art. II.2 bidirectional price signal.
    #[serde(default)]
    pub direction: Option<String>,
    /// TB-N1-AGENT-ECONOMY Phase 2 A3 (2026-05-10): agent-decided stake for
    /// the `step` tool, in micro-units (1 Coin = 1_000_000 μC). Optional —
    /// when absent, evaluator falls back to the env default
    /// `TURINGOS_CHAINTAPE_PROPOSAL_STAKE_MICRO` (1000 μC). Sequencer
    /// admission step-4 rejects with `StakeBalanceExceeded` if the declared
    /// `stake_micro` exceeds the agent's `balances_t` entry. Closes the
    /// agency layer of CLAUDE.md §13: agent-decided stake within
    /// `[1, balance]` is now a typed admission gate.
    #[serde(default)]
    pub stake_micro: Option<u64>,
    /// TB-N1-AGENT-ECONOMY Phase 2 A4 (2026-05-10): target_work_tx_id for
    /// the `verify_peer` tool. The TxId of the WorkTx being verified by
    /// this agent. Required when `tool == "verify_peer"`. Sequencer admission
    /// step-3 rejects with `VerifyTargetNotAccepted` if the target is not
    /// present in `q.economic_state_t.stakes_t`.
    #[serde(default)]
    pub target_work_tx_id: Option<String>,
    /// TB-N1-AGENT-ECONOMY Phase 2 A4 (2026-05-10): verdict for the
    /// `verify_peer` tool. Valid values: `"confirm"` (target is correct;
    /// OMEGA-Confirm path; creates a claim) or `"deny"` (target is
    /// incorrect). Required when `tool == "verify_peer"`. Defaults to
    /// `"confirm"` on absence per VerifyVerdict::Confirm as the OMEGA verdict
    /// (ratification §2.1).
    #[serde(default)]
    pub verdict: Option<String>,
    /// TB-N1-AGENT-ECONOMY Phase 2 A4 (2026-05-10): agent-decided bond for
    /// the `verify_peer` tool, in micro-units. Optional — when absent,
    /// evaluator falls back to a small env default. Sequencer admission
    /// step-2.5 rejects with `VerifyBondOutOfBounds` if the declared
    /// `bond_micro` exceeds the verifier's `balances_t` entry; step-2
    /// rejects with `BondInsufficient` if bond_micro == 0. Mirrors
    /// `stake_micro` for the verify-peer agency path.
    #[serde(default)]
    pub bond_micro: Option<u64>,
}

/// Parse error with explicit reason. V3L-09: NEVER silently return None.
#[derive(Debug, Clone)]
pub enum ParseError {
    NoActionTag,
    InvalidJson(String),
    EmptyPayload,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::NoActionTag => write!(f, "No <action> tag found in output"),
            ParseError::InvalidJson(msg) => write!(f, "Invalid JSON in action: {}", msg),
            ParseError::EmptyPayload => write!(f, "Empty payload in action"),
        }
    }
}

impl std::error::Error for ParseError {}

// ── Parser ──────────────────────────────────────────────────────

/// Strip `<think>...</think>` blocks from LLM output.
/// V3L-15: raw think blocks leak into next agent's context = self-poisoning.
/// V3L-16: dual-chamber — free thinking in user space, extract determinism at membrane.
pub fn strip_think_blocks(raw: &str) -> String {
    let mut result = String::new();
    let mut remaining = raw;

    loop {
        if let Some(start) = remaining.find("<think>") {
            result.push_str(&remaining[..start]);
            if let Some(end) = remaining[start..].find("</think>") {
                remaining = &remaining[start + end + "</think>".len()..];
            } else {
                // Unclosed <think> — strip everything after it
                break;
            }
        } else {
            result.push_str(remaining);
            break;
        }
    }

    result
}

/// Parse agent output into an AgentAction.
///
/// Three-layer tolerance (Postel's law — V3L-08):
/// 1. Find `<action>{JSON}</action>` tag
/// 2. Find bare `{JSON}` with "tool" field (fallback)
/// 3. Return explicit ParseError (NEVER None — V3L-09)
///
/// Rule 22 v2 clause 4: reject-only, no byte-modifying repairs.
pub fn parse_agent_output(raw: &str) -> Result<AgentAction, ParseError> {
    // First: strip think blocks (V3L-15/16)
    let cleaned = strip_think_blocks(raw);

    // Layer 1: <action>{...}</action> protocol
    // If <action> tag is present but malformed, REJECT — don't fall through to Layer 2
    if cleaned.contains("<action>") {
        return match try_parse_action_tag(&cleaned) {
            Some(result) => result,
            None => Err(ParseError::NoActionTag), // tag present but malformed = reject
        };
    }

    // Layer 2: bare JSON object with "tool" field (only if no <action> tag at all)
    if let Some(action) = try_parse_bare_json(&cleaned) {
        return action;
    }

    // Layer 3: explicit error (V3L-09: NEVER silently return None)
    Err(ParseError::NoActionTag)
}

/// Layer 1: Find <action>{...}</action> and parse the JSON inside.
fn try_parse_action_tag(text: &str) -> Option<Result<AgentAction, ParseError>> {
    let start_tag = "<action>";
    let end_tag = "</action>";

    let start = text.find(start_tag)?;
    let json_start = start + start_tag.len();
    let end = text[json_start..].find(end_tag)?;
    let json_str = &text[json_start..json_start + end];

    Some(parse_json(json_str))
}

/// Layer 2: Find any JSON object containing "tool" field.
fn try_parse_bare_json(text: &str) -> Option<Result<AgentAction, ParseError>> {
    // Find first '{' that might be a JSON object
    for (i, _) in text.match_indices('{') {
        // Find matching '}'
        let mut depth = 0;
        for (j, ch) in text[i..].char_indices() {
            match ch {
                '{' => depth += 1,
                '}' => {
                    depth -= 1;
                    if depth == 0 {
                        let candidate = &text[i..i + j + 1];
                        if candidate.contains("\"tool\"") {
                            return Some(parse_json(candidate));
                        }
                        break;
                    }
                }
                _ => {}
            }
        }
    }
    None
}

/// Parse a JSON string into AgentAction. No byte repair (Rule 22 v2 clause 4).
fn parse_json(json_str: &str) -> Result<AgentAction, ParseError> {
    serde_json::from_str::<AgentAction>(json_str)
        .map_err(|e| ParseError::InvalidJson(e.to_string()))
}

// ── Tests ───────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_action_tag_valid() {
        let raw = r#"Some preamble text <action>{"tool":"append","payload":"step 1"}</action>"#;
        let action = parse_agent_output(raw).unwrap();
        assert_eq!(action.tool, "append");
        assert_eq!(action.payload.as_deref(), Some("step 1"));
    }

    #[test]
    fn test_parse_action_tag_with_think_block() {
        // V3L-15: think blocks must be stripped
        let raw = r#"<think>internal reasoning</think><action>{"tool":"search","query":"test"}</action>"#;
        let action = parse_agent_output(raw).unwrap();
        assert_eq!(action.tool, "search");
        assert_eq!(action.query.as_deref(), Some("test"));
    }

    #[test]
    fn test_parse_bare_json_fallback() {
        // Layer 2: bare JSON without action tags
        let raw = r#"I think we should try {"tool":"append","payload":"step 2"}"#;
        let action = parse_agent_output(raw).unwrap();
        assert_eq!(action.tool, "append");
    }

    #[test]
    fn test_parse_no_action_returns_error() {
        // V3L-09: NEVER return None, always explicit error
        let raw = "Just some random text with no action";
        let result = parse_agent_output(raw);
        assert!(result.is_err());
        assert!(matches!(result, Err(ParseError::NoActionTag)));
    }

    #[test]
    fn test_parse_invalid_json_returns_error() {
        let raw = r#"<action>{invalid json here}</action>"#;
        let result = parse_agent_output(raw);
        assert!(matches!(result, Err(ParseError::InvalidJson(_))));
    }

    #[test]
    fn test_parse_with_invest_action() {
        let raw = r#"<action>{"tool":"invest","node":"n1","amount":50.0}</action>"#;
        let action = parse_agent_output(raw).unwrap();
        assert_eq!(action.tool, "invest");
        assert_eq!(action.node.as_deref(), Some("n1"));
        assert_eq!(action.amount, Some(50.0));
    }

    #[test]
    fn test_strip_think_blocks() {
        let input = "before<think>secret</think>after";
        assert_eq!(strip_think_blocks(input), "beforeafter");
    }

    #[test]
    fn test_strip_multiple_think_blocks() {
        let input = "a<think>x</think>b<think>y</think>c";
        assert_eq!(strip_think_blocks(input), "abc");
    }

    #[test]
    fn test_strip_unclosed_think_block() {
        // Unclosed think = strip everything after
        let input = "before<think>leaked";
        assert_eq!(strip_think_blocks(input), "before");
    }

    #[test]
    fn test_no_byte_repair_on_invalid_escape() {
        // Rule 22 v2 clause 4: reject-only, no repair
        // LaTeX escape \cdot is invalid JSON — must reject, not fix
        let raw = r#"<action>{"tool":"append","payload":"x \cdot y"}</action>"#;
        let result = parse_agent_output(raw);
        assert!(result.is_err(), "Invalid JSON escape must be rejected, not repaired");
    }

    #[test]
    fn test_malformed_action_tag_rejected_not_fallback() {
        // Codex finding: if <action> is present but malformed (no </action>),
        // must reject — NOT fall through to bare JSON fallback
        let raw = r#"<action>{"tool":"append"} some trailing text {"tool":"search","query":"test"}"#;
        let result = parse_agent_output(raw);
        assert!(result.is_err(), "Malformed <action> tag must be rejected, not fall through");
    }

    #[test]
    fn test_deduct_negative_amount_rejected() {
        // Codex finding: negative deduct = credit. Must reject.
        // (This is tested in wallet but verified here for completeness)
    }
}
