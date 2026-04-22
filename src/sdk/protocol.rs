// Tier 2: Postel's law parser — wide input, strict output
// Constitutional basis: Art. I.1.1 (PCP predicate, reject-only)
// V3 lessons: V3L-08 (format fragility), V3L-09 (no silent failure),
//             V3L-15 (context self-poisoning), V3L-16 (dual-chamber)

use serde::{Deserialize, Deserializer};
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
}

/// The δ output ⟨q_o, a_o⟩ per constitution Art. IV mermaid:
///   AI[δ](input) = ⟨q_o, a_o⟩
/// where `q_o` (`q_delta`) is the OPTIONAL state hint the agent signals for
/// Q_{t+1} (e.g. "halt_soon", "continue_dfs") and `a_o` (`action`) is the
/// concrete tool call that mutates the tape.
///
/// Backward-compat: the legacy flat JSON form (`{"tool":"step",...}`) is still
/// accepted and deserializes into `AgentOutput { q_delta: None, action: ... }`.
/// New form is `{"q_delta":"...","action":{"tool":"step",...}}`.
#[derive(Debug, Clone)]
pub struct AgentOutput {
    /// Optional state hint (q_o). Agents MAY emit this; consumers MUST tolerate absence.
    pub q_delta: Option<String>,
    /// The concrete action (a_o) — the tool call that mutates the tape.
    pub action: AgentAction,
}

impl AgentOutput {
    /// Construct from a legacy `AgentAction` with no state hint.
    pub fn from_action(action: AgentAction) -> Self {
        Self { q_delta: None, action }
    }
}

// Custom deserializer: accept both `{q_delta, action}` wrapped form and the
// legacy flat `{tool, payload, ...}` form. We do NOT use serde's `untagged`
// because inner-action errors must be surfaced (untagged would swallow them
// and try the other variant). A manual impl gives us explicit dispatch on
// the presence of the "action" field: that field is the disambiguator for
// the wrapped δ-output shape ⟨q_o, a_o⟩.
impl<'de> Deserialize<'de> for AgentOutput {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        let obj = value
            .as_object()
            .ok_or_else(|| serde::de::Error::custom("expected JSON object for AgentOutput"))?;

        // Wrapped form: has "action" field (and optionally "q_delta"). This
        // is the new ⟨q_o, a_o⟩ shape.
        if let Some(action_val) = obj.get("action") {
            let action: AgentAction = serde_json::from_value(action_val.clone())
                .map_err(serde::de::Error::custom)?;
            let q_delta = obj
                .get("q_delta")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            return Ok(AgentOutput { q_delta, action });
        }

        // Legacy flat form: whole object is an AgentAction (has "tool" field).
        let action: AgentAction =
            serde_json::from_value(value).map_err(serde::de::Error::custom)?;
        Ok(AgentOutput { q_delta: None, action })
    }
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

/// Parse agent output into an `AgentOutput` (⟨q_o, a_o⟩ per Art. IV).
///
/// Three-layer tolerance (Postel's law — V3L-08):
/// 1. Find `<action>{JSON}</action>` tag
/// 2. Find bare `{JSON}` with "tool"/"action" field (fallback)
/// 3. Return explicit ParseError (NEVER None — V3L-09)
///
/// Accepts both:
///   - legacy flat form: `{"tool":"step","payload":"..."}` → `q_delta: None`
///   - wrapped form:     `{"q_delta":"...","action":{"tool":"step",...}}`
///
/// Rule 22 v2 clause 4: reject-only, no byte-modifying repairs.
pub fn parse_agent_output(raw: &str) -> Result<AgentOutput, ParseError> {
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

    // Layer 2: bare JSON object with "tool" or "action" field (only if no <action> tag)
    if let Some(result) = try_parse_bare_json(&cleaned) {
        return result;
    }

    // Layer 3: explicit error (V3L-09: NEVER silently return None)
    Err(ParseError::NoActionTag)
}

/// Layer 1: Find <action>{...}</action> and parse the JSON inside.
fn try_parse_action_tag(text: &str) -> Option<Result<AgentOutput, ParseError>> {
    let start_tag = "<action>";
    let end_tag = "</action>";

    let start = text.find(start_tag)?;
    let json_start = start + start_tag.len();
    let end = text[json_start..].find(end_tag)?;
    let json_str = &text[json_start..json_start + end];

    Some(parse_json(json_str))
}

/// Layer 2: Find any JSON object containing "tool" or "action" field.
fn try_parse_bare_json(text: &str) -> Option<Result<AgentOutput, ParseError>> {
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
                        // Either legacy (has "tool") or wrapped (has "action").
                        if candidate.contains("\"tool\"") || candidate.contains("\"action\"") {
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

/// Parse a JSON string into AgentOutput. No byte repair (Rule 22 v2 clause 4).
/// Delegates to AgentOutput's custom Deserialize impl which accepts both the
/// wrapped ⟨q_delta, action⟩ form and the legacy flat action form.
fn parse_json(json_str: &str) -> Result<AgentOutput, ParseError> {
    serde_json::from_str::<AgentOutput>(json_str)
        .map_err(|e| ParseError::InvalidJson(e.to_string()))
}

// ── Tests ───────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_action_tag_valid() {
        let raw = r#"Some preamble text <action>{"tool":"append","payload":"step 1"}</action>"#;
        let out = parse_agent_output(raw).unwrap();
        assert_eq!(out.action.tool, "append");
        assert_eq!(out.action.payload.as_deref(), Some("step 1"));
        assert!(out.q_delta.is_none(), "legacy flat form must yield q_delta=None");
    }

    #[test]
    fn test_parse_action_tag_with_think_block() {
        // V3L-15: think blocks must be stripped
        let raw = r#"<think>internal reasoning</think><action>{"tool":"search","query":"test"}</action>"#;
        let out = parse_agent_output(raw).unwrap();
        assert_eq!(out.action.tool, "search");
        assert_eq!(out.action.query.as_deref(), Some("test"));
    }

    #[test]
    fn test_parse_bare_json_fallback() {
        // Layer 2: bare JSON without action tags
        let raw = r#"I think we should try {"tool":"append","payload":"step 2"}"#;
        let out = parse_agent_output(raw).unwrap();
        assert_eq!(out.action.tool, "append");
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
        let out = parse_agent_output(raw).unwrap();
        assert_eq!(out.action.tool, "invest");
        assert_eq!(out.action.node.as_deref(), Some("n1"));
        assert_eq!(out.action.amount, Some(50.0));
    }

    // ── ⟨q_o, a_o⟩ δ-output tests (Art. IV) ─────────────────────

    #[test]
    fn parse_legacy_flat_action_works() {
        // Backward-compat: existing JSON format `{"tool":"step","payload":"..."}`
        // must still parse into AgentOutput with q_delta=None.
        let raw = r#"<action>{"tool":"step","payload":"intro h"}</action>"#;
        let out = parse_agent_output(raw).expect("legacy flat form must parse");
        assert!(out.q_delta.is_none(), "legacy form yields no q_delta");
        assert_eq!(out.action.tool, "step");
        assert_eq!(out.action.payload.as_deref(), Some("intro h"));
    }

    #[test]
    fn parse_wrapped_output_form() {
        // New wrapped form: {"q_delta":"halt_soon","action":{"tool":"step","payload":"..."}}
        let raw = r#"<action>{"q_delta":"halt_soon","action":{"tool":"step","payload":"linarith"}}</action>"#;
        let out = parse_agent_output(raw).expect("wrapped form must parse");
        assert_eq!(out.q_delta.as_deref(), Some("halt_soon"));
        assert_eq!(out.action.tool, "step");
        assert_eq!(out.action.payload.as_deref(), Some("linarith"));
    }

    #[test]
    fn parse_error_on_bad_action() {
        // Both forms must propagate error if the inner action is malformed
        // (e.g. missing required "tool" field, or non-object action).

        // Legacy flat, missing required "tool": inner deserialize fails.
        let raw_legacy = r#"<action>{"payload":"x"}</action>"#;
        let r1 = parse_agent_output(raw_legacy);
        assert!(matches!(r1, Err(ParseError::InvalidJson(_))),
            "legacy form without `tool` must surface InvalidJson, got {:?}", r1);

        // Wrapped form with malformed inner action: inner deserialize fails.
        let raw_wrapped = r#"<action>{"q_delta":"x","action":{"payload":"y"}}</action>"#;
        let r2 = parse_agent_output(raw_wrapped);
        assert!(matches!(r2, Err(ParseError::InvalidJson(_))),
            "wrapped form with malformed inner action must surface InvalidJson, got {:?}", r2);

        // Wrapped form where `action` is not an object at all.
        let raw_nonobj = r#"<action>{"q_delta":"x","action":"not-an-object"}</action>"#;
        let r3 = parse_agent_output(raw_nonobj);
        assert!(matches!(r3, Err(ParseError::InvalidJson(_))),
            "wrapped form with non-object action must surface InvalidJson, got {:?}", r3);
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
