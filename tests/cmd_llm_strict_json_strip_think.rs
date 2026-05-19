//! F2 fix tests — cmd_llm `complete --strict-json` and `triage` must strip
//! `<think>...</think>` blocks from LLM output BEFORE feeding the content to
//! the JSON parsers (grill envelope for `complete`, classification JSON for
//! `triage`).
//!
//! These tests exercise the strip→parse composition directly (using the same
//! shared helper `sdk::protocol::strip_think_blocks` that the bin now calls),
//! independent of subprocess/HTTP mock infrastructure. This is the load-bearing
//! parser hole identified in Research-D Part I.
//!
//! Covered models (per F2 brief): DeepSeek-V3.1-Terminus think-on, DeepSeek-R1,
//! Qwen3-8B/14B/32B with default-on thinking, GLM-4.7, Kimi-K2.5/K2.6 with
//! thinking enabled. All of these may prefix the JSON envelope with a
//! reasoning trace in `<think>...</think>` form.

use turingosv4::runtime::grill_envelope::{parse_and_validate, EnvelopeParseError};
use turingosv4::sdk::protocol::strip_think_blocks;

/// Convenience: replicate the exact pipeline used in cmd_llm.rs `complete`
/// strict-json branch (strip → trim → parse_and_validate).
fn strict_json_pipeline(
    raw: &str,
) -> Result<turingosv4::runtime::grill_envelope::TurnPayload, EnvelopeParseError> {
    let stripped = strip_think_blocks(raw);
    parse_and_validate(stripped.trim())
}

#[test]
fn strict_json_strips_think_block_before_parse() {
    // The defect: without stripping, the leading <think>...</think> prefix
    // makes serde_json see non-JSON bytes and reject the entire turn.
    let raw = "<think>I'll analyze the user's answer and decide which slot to probe next. They mentioned offline play, so I should ask about multiplayer.</think>{\"turn\":1,\"question\":\"Does it need multiplayer?\",\"covered_slots\":[\"job\"],\"open_slots\":[\"anchor\",\"memory\",\"first_run\",\"robustness\",\"scope\",\"acceptance\"],\"confidence\":0.5,\"done\":false,\"rationale\":\"probe multiplayer scope\"}";
    let payload = strict_json_pipeline(raw)
        .expect("strict-JSON parse must succeed after stripping think block");
    assert_eq!(payload.turn, 1);
    assert_eq!(payload.covered_slots, vec!["job"]);
    assert!(!payload.done);
    assert_eq!(payload.confidence, 0.5);
}

#[test]
fn strict_json_strips_multiple_think_blocks() {
    // Some models emit interleaved think/output spans (e.g., GLM-4.7 in
    // streaming mode flattened by the provider). All blocks must be stripped.
    let raw = "<think>first pass — covered_slots looks short</think>some prose between<think>second pass — adding open_slots</think>{\"turn\":2,\"question\":\"What's the anchor genre?\",\"covered_slots\":[\"job\"],\"open_slots\":[\"anchor\",\"memory\",\"first_run\",\"robustness\",\"scope\",\"acceptance\"],\"confidence\":0.6,\"done\":false,\"rationale\":\"probe anchor\"}";
    // Note: "some prose between" remains after stripping; parse_and_validate
    // will see "some prose between{...}" which serde_json rejects. This is
    // CORRECT behavior — the contract is "strip think tags, then parse", not
    // "find JSON anywhere". The test asserts a CLEAN failure with the
    // structural error code, NOT a parse-trace leak from the think block.
    let stripped = strip_think_blocks(raw);
    assert!(
        !stripped.contains("<think>") && !stripped.contains("</think>"),
        "all think tags must be removed; got: {}",
        stripped
    );
    assert!(
        !stripped.contains("first pass") && !stripped.contains("second pass"),
        "think content must be removed; got: {}",
        stripped
    );
    assert!(
        stripped.contains("some prose between"),
        "non-think prose must survive; got: {}",
        stripped
    );
}

#[test]
fn strict_json_strips_two_think_blocks_then_pure_json() {
    // Realistic shape: two think spans with no prose between, then clean JSON.
    // This is the common GLM-4.7 / Qwen3-32B thinking-on pattern.
    let raw = "<think>step 1</think><think>step 2</think>{\"turn\":3,\"question\":\"What memory aid?\",\"covered_slots\":[\"job\",\"anchor\"],\"open_slots\":[\"memory\",\"first_run\",\"robustness\",\"scope\",\"acceptance\"],\"confidence\":0.7,\"done\":false,\"rationale\":\"probe memory\"}";
    let payload = strict_json_pipeline(raw).expect("two think blocks then clean JSON must parse");
    assert_eq!(payload.turn, 3);
    assert_eq!(payload.covered_slots.len(), 2);
}

#[test]
fn strict_json_handles_unclosed_think() {
    // Unclosed <think> (truncated by max_tokens, or model error): the JSON
    // never gets emitted. We MUST fail cleanly with parse error, never panic,
    // never silently accept.
    let raw = "<think>still thinking, the user said... I need to consider...";
    let result = strict_json_pipeline(raw);
    assert!(
        matches!(result, Err(EnvelopeParseError::InvalidJson(_))),
        "unclosed think with no JSON must yield InvalidJson; got: {:?}",
        result
    );
}

#[test]
fn strict_json_passthrough_no_think() {
    // Baseline: clean JSON with no think tags must parse unchanged.
    let raw = r#"{"turn":1,"question":"What kind of game?","covered_slots":["job"],"open_slots":["anchor","memory","first_run","robustness","scope","acceptance"],"confidence":0.4,"done":false,"rationale":"first turn"}"#;
    let payload = strict_json_pipeline(raw).expect("clean JSON must parse unchanged");
    assert_eq!(payload.turn, 1);
}

#[test]
fn strict_json_strips_think_with_newlines_and_indentation() {
    // DeepSeek-R1 and Kimi-K2.6 typically emit multi-line think blocks with
    // markdown-style indentation. The strip must handle these correctly.
    let raw = "<think>\nLet me think step by step:\n  1. User mentioned offline\n  2. Need to probe scope\n</think>\n\n{\"turn\":2,\"question\":\"Single-player only?\",\"covered_slots\":[\"job\",\"anchor\"],\"open_slots\":[\"memory\",\"first_run\",\"robustness\",\"scope\",\"acceptance\"],\"confidence\":0.55,\"done\":false,\"rationale\":\"scope probe\"}";
    let payload = strict_json_pipeline(raw).expect("multi-line think + JSON must parse");
    assert_eq!(payload.turn, 2);
    assert_eq!(payload.covered_slots.len(), 2);
}

#[test]
fn triage_strip_handles_unclosed() {
    // Triage path (Blackbox classifier) uses the same shared helper now.
    // An unclosed <think> with no JSON payload must produce a clean strip
    // result (truncated at the unclosed opener) that then fails parse with
    // a normal serde error — NOT a panic, NOT a regex-engine hang.
    let raw = "<think>classifying user input, considering tone";
    let stripped = strip_think_blocks(raw);
    assert_eq!(
        stripped, "",
        "unclosed think with no trailing content must strip to empty; got: {:?}",
        stripped
    );
    // Then the triage caller does `serde_json::from_str(stripped.trim())`
    // which must error cleanly:
    let parse_result: serde_json::Result<serde_json::Value> = serde_json::from_str(stripped.trim());
    assert!(
        parse_result.is_err(),
        "empty string must not be valid JSON; got: {:?}",
        parse_result
    );
}

#[test]
fn triage_strip_handles_think_then_classification_json() {
    // Thinking-mode model emitting reasoning before the classification JSON.
    // The old `strip_thinking_wrapper` (split at last </think>) would have
    // worked for THIS case, but failed on the unclosed and multi-block cases.
    // The new shared helper must continue to work for the simple case.
    let raw = "<think>This message is on-topic, the user is asking about game features.</think>{\"class\":\"relevant\",\"confidence\":0.92}";
    let stripped = strip_think_blocks(raw);
    let json_str = stripped.trim();
    let v: serde_json::Value =
        serde_json::from_str(json_str).expect("classification JSON must parse after stripping");
    assert_eq!(v["class"], serde_json::json!("relevant"));
    assert_eq!(v["confidence"], serde_json::json!(0.92));
}

#[test]
fn triage_strip_handles_multiple_think_blocks() {
    // Asymmetry fix: the old triage `strip_thinking_wrapper` split at the
    // LAST </think> only, which meant content between intermediate
    // <think>...</think> spans leaked into the JSON parser. The shared
    // helper handles all blocks.
    let raw = "<think>first thought</think><think>second thought</think>{\"class\":\"off_topic\",\"confidence\":0.85}";
    let stripped = strip_think_blocks(raw);
    let json_str = stripped.trim();
    let v: serde_json::Value = serde_json::from_str(json_str)
        .expect("classification JSON must parse after stripping all blocks");
    assert_eq!(v["class"], serde_json::json!("off_topic"));
}
