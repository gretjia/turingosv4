//! TRACE_MATRIX FC1a-output_edge: TDMA-Bounded state-first prefix parser.
//!
//! The worker model emits a structured header (a JSON object matching `tdma-state-update/v1`)
//! followed by a free-form body. The parser scans ONLY the first `B_HEADER_SCAN` tokens of the
//! output, extracts the first balanced JSON object, and routes the kernel based on the parsed
//! header. The `---BODY---` marker is a human-readability hint, NOT a parser dependency
//! (directive §4.1, §8). The legacy `<STATE_UPDATE>...</STATE_UPDATE>` closing-tag pattern is
//! explicitly FORBIDDEN (KILL-tdma-4).
//!
//! On-disk §8: handover/directives/2026-05-22_TDMA_BOUNDED_RC1_DIRECTIVE_AND_§8.md

use serde::{Deserialize, Serialize};

// ── Schema ───────────────────────────────────────────────────────

/// Status of the worker's reasoning step (directive §4.1).
/// TRACE_MATRIX FC1a-output_edge: Discriminates the kernel routing decision.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum StateStatus {
    /// Step accepted; kernel should advance verified_head.
    Proceed,
    /// Step rejected; kernel should commit AgentProposal verified=false and retry.
    Retry,
    /// Header itself malformed; kernel should retry the header-invalid path.
    Invalid,
    /// Terminal failure or user-requested halt; kernel should escalate.
    Halt,
}

/// State-first header (directive §4.1). The first JSON object in the worker's output.
/// schema_version pinned to "tdma-state-update/v1".
/// TRACE_MATRIX FC1a-output_edge: The structured signal the kernel parses out of
/// the LLM response before consuming the body.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StateUpdate {
    pub schema_version: String,
    pub status: StateStatus,
    pub task_id: String,
    pub action: String,
    pub failed_predicate: Option<String>,
    pub reject_class: Option<String>,
    pub next_action_hint: Option<String>,
    pub evidence_hash: Option<String>,
}

// ── Errors ───────────────────────────────────────────────────────

/// Header parse outcomes (directive §8 six-case matrix).
/// TRACE_MATRIX FC1a-output_edge: Each error variant maps to one routing case;
/// none of them advance verified_head.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HeaderParseError {
    MissingJsonObject,
    MalformedJson(String),
    SchemaInvalid(String),
    HeaderTooLong(usize),
}

impl std::fmt::Display for HeaderParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HeaderParseError::MissingJsonObject => {
                write!(f, "Missing JSON object in scan-budget prefix")
            }
            HeaderParseError::MalformedJson(e) => write!(f, "Malformed JSON: {}", e),
            HeaderParseError::SchemaInvalid(e) => write!(f, "Schema invalid: {}", e),
            HeaderParseError::HeaderTooLong(n) => write!(f, "Header too long: {} tokens", n),
        }
    }
}

impl std::error::Error for HeaderParseError {}

// ── Streaming JSON extractor ────────────────────────────────────

/// Extract the first balanced JSON object from a prefix string.
/// Returns the substring spanning the object (inclusive of braces).
/// TRACE_MATRIX FC1a-output_edge: Decouples header location from any closing-tag
/// marker (KILL-tdma-4); the parser tolerates body truncation because it only
/// reads up to the first balanced `{...}`.
pub fn streaming_extract_first_json_object(prefix: &str) -> Option<String> {
    let bytes = prefix.as_bytes();
    let mut depth: i32 = 0;
    let mut start: Option<usize> = None;
    let mut in_string = false;
    let mut escape = false;

    for (i, &b) in bytes.iter().enumerate() {
        if in_string {
            if escape {
                escape = false;
            } else if b == b'\\' {
                escape = true;
            } else if b == b'"' {
                in_string = false;
            }
            continue;
        }
        match b {
            b'"' => in_string = true,
            b'{' => {
                if depth == 0 {
                    start = Some(i);
                }
                depth += 1;
            }
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    if let Some(s) = start {
                        // Safe slice on UTF-8 ASCII brace bounds.
                        return Some(prefix[s..=i].to_string());
                    }
                }
            }
            _ => {}
        }
    }
    None
}

// ── Schema validation ────────────────────────────────────────────

/// Validate the parsed header against schema_version + required-field rules.
/// TRACE_MATRIX FC1a-output_edge + KILL-tdma-3: Enforces schema_version pin
/// so the kernel cannot accept stale-schema headers.
pub fn validate_state_update_schema(h: &StateUpdate) -> Result<(), HeaderParseError> {
    if h.schema_version != "tdma-state-update/v1" {
        return Err(HeaderParseError::SchemaInvalid(format!(
            "expected schema_version 'tdma-state-update/v1', got '{}'",
            h.schema_version
        )));
    }
    if h.task_id.is_empty() {
        return Err(HeaderParseError::SchemaInvalid("task_id is empty".into()));
    }
    if h.action.is_empty() {
        return Err(HeaderParseError::SchemaInvalid("action is empty".into()));
    }
    Ok(())
}

// ── Token-aware prefix scan ──────────────────────────────────────

/// Conservative token estimator used when no full tokenizer is wired (Atom 3
/// provides the real `Tokenizer`). 4 chars ~= 1 token is the GPT-family
/// rule-of-thumb. This estimator is sufficient for the scan-budget gate.
/// TRACE_MATRIX FC1a-output_edge: Bounded-prefix policy enforcement.
fn estimate_tokens(s: &str) -> usize {
    (s.chars().count() + 3) / 4
}

/// Parse the state-first header from a worker's raw output.
/// Scans only the first `scan_budget` *tokens* of the prefix; extracts the first
/// balanced JSON object; validates against the schema; enforces `header_budget`
/// on the extracted object length.
/// TRACE_MATRIX FC1a-output_edge: Single entry-point used by memory_kernel
/// `step_forward` for the routing decision.
pub fn parse_prefix_json(
    raw_output: &str,
    scan_budget_tokens: usize,
    header_budget_tokens: usize,
) -> Result<StateUpdate, HeaderParseError> {
    // Cap the prefix scan to scan_budget tokens (estimated).
    let scan_budget_chars = scan_budget_tokens.saturating_mul(4);
    let prefix: String = raw_output.chars().take(scan_budget_chars).collect();

    let first_obj = streaming_extract_first_json_object(&prefix)
        .ok_or(HeaderParseError::MissingJsonObject)?;

    let header: StateUpdate = serde_json::from_str(&first_obj)
        .map_err(|e| HeaderParseError::MalformedJson(e.to_string()))?;

    validate_state_update_schema(&header)?;

    let header_tokens = estimate_tokens(&first_obj);
    if header_tokens > header_budget_tokens {
        return Err(HeaderParseError::HeaderTooLong(header_tokens));
    }

    Ok(header)
}

// ── Tests ────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn h_proceed(task: &str) -> String {
        format!(
            r#"{{"schema_version":"tdma-state-update/v1","status":"Proceed","task_id":"{}","action":"PROCEED"}}"#,
            task
        )
    }

    fn h_retry(task: &str) -> String {
        format!(
            r#"{{"schema_version":"tdma-state-update/v1","status":"Retry","task_id":"{}","action":"RETRY","failed_predicate":"x.y","reject_class":"schema-fail"}}"#,
            task
        )
    }

    // ── state_first_prefix_parser ────────────────────────────────

    #[test]
    fn state_first_prefix_parser_proceed() {
        let raw = format!("{}\n---BODY---\nhello", h_proceed("t1"));
        let h = parse_prefix_json(&raw, 512, 256).expect("must parse");
        assert_eq!(h.status, StateStatus::Proceed);
        assert_eq!(h.task_id, "t1");
        assert_eq!(h.action, "PROCEED");
    }

    #[test]
    fn state_first_prefix_parser_retry_with_optional_fields() {
        let raw = h_retry("t2");
        let h = parse_prefix_json(&raw, 512, 256).expect("must parse");
        assert_eq!(h.status, StateStatus::Retry);
        assert_eq!(h.failed_predicate, Some("x.y".into()));
        assert_eq!(h.reject_class, Some("schema-fail".into()));
    }

    #[test]
    fn state_first_prefix_parser_no_body_marker_required() {
        // ---BODY--- marker is human hint, NOT parser dependency
        let raw = h_proceed("t3");
        assert!(parse_prefix_json(&raw, 512, 256).is_ok());
    }

    // ── header_malformation (6-case matrix per directive §8) ─────

    #[test]
    fn header_malformation_case1_valid_passes() {
        let raw = h_retry("t1");
        assert!(parse_prefix_json(&raw, 512, 256).is_ok());
    }

    #[test]
    fn header_malformation_case2_missing_json_object() {
        let raw = "no json here, just text, nothing balanced".to_string();
        let err = parse_prefix_json(&raw, 512, 256).unwrap_err();
        assert!(matches!(err, HeaderParseError::MissingJsonObject));
    }

    #[test]
    fn header_malformation_case3_malformed_json() {
        // Unbalanced JSON in prefix (open brace, never closed)
        // The streaming extractor returns None when it never sees a balanced close.
        let raw = "{\"schema_version\":\"tdma-state-update/v1\",\"unterminated".to_string();
        let err = parse_prefix_json(&raw, 512, 256).unwrap_err();
        assert!(matches!(err, HeaderParseError::MissingJsonObject));
    }

    #[test]
    fn header_malformation_case3b_malformed_json_balanced_braces() {
        // Balanced braces but invalid JSON content (trailing comma is invalid in strict JSON)
        let raw = "{\"schema_version\":\"tdma-state-update/v1\",}".to_string();
        let err = parse_prefix_json(&raw, 512, 256).unwrap_err();
        assert!(matches!(err, HeaderParseError::MalformedJson(_)));
    }

    #[test]
    fn header_malformation_case4_schema_invalid() {
        let raw = r#"{"schema_version":"WRONG/v9","status":"Retry","task_id":"t","action":"RETRY"}"#
            .to_string();
        let err = parse_prefix_json(&raw, 512, 256).unwrap_err();
        assert!(matches!(err, HeaderParseError::SchemaInvalid(_)));
    }

    #[test]
    fn header_malformation_case5_header_too_long() {
        // Build a header that exceeds B_HEADER (256 tokens estimated)
        let task = "x".repeat(2000); // ~500 tokens — exceeds B_HEADER
        let raw = h_retry(&task);
        let err = parse_prefix_json(&raw, 4096, 256).unwrap_err();
        assert!(matches!(err, HeaderParseError::HeaderTooLong(_)));
    }

    #[test]
    fn header_malformation_case6_truncated_header_before_close() {
        // Header truncated mid-object — no closing brace ever appears.
        let raw =
            r#"{"schema_version":"tdma-state-update/v1","status":"Retry","task_id":"t","actio"#
                .to_string();
        let err = parse_prefix_json(&raw, 512, 256).unwrap_err();
        assert!(matches!(err, HeaderParseError::MissingJsonObject));
    }

    // ── truncation_survival (valid header + body truncated) ──────

    #[test]
    fn truncation_survival_valid_header_short_body() {
        // Valid header, then body cut off mid-stream
        let raw = format!("{}\n---BODY---\npartial body cut", h_proceed("survive"));
        let h = parse_prefix_json(&raw, 512, 256).expect("header must parse");
        assert_eq!(h.task_id, "survive");
    }

    #[test]
    fn truncation_survival_header_within_scan_budget() {
        // Pad body with junk but header is still within first 512 tokens
        let header = h_retry("padded");
        let padding = "x".repeat(10_000); // far past scan budget
        let raw = format!("{}\n---BODY---\n{}", header, padding);
        let h = parse_prefix_json(&raw, 512, 256).expect("header within scan");
        assert_eq!(h.task_id, "padded");
    }

    // ── streaming_extract_first_json_object structural ───────────

    #[test]
    fn streaming_extractor_nested_braces() {
        let s = r#"prefix {"a": {"b": 1}, "c": [{"d": 2}]} suffix"#;
        let extracted = streaming_extract_first_json_object(s).unwrap();
        assert_eq!(extracted, r#"{"a": {"b": 1}, "c": [{"d": 2}]}"#);
    }

    #[test]
    fn streaming_extractor_braces_in_string_ignored() {
        let s = r#"prefix {"key": "value with } brace"} suffix"#;
        let extracted = streaming_extract_first_json_object(s).unwrap();
        assert_eq!(extracted, r#"{"key": "value with } brace"}"#);
    }
}
