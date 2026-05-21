//! B1: Unit tests for the DeepSeek model-name rejection actionable rewrite.
//!
//! TRACE_MATRIX FC2-N16: SiliconFlow client error path (UX hardening).
//! Risk class: 2 (additive, production wire-up).
//! No network calls — exercises the rewrite helper directly.

// The rewrite helper is in the binary crate; access it via the binary's
// integration test path.  We reference the function through the siliconflow
// module re-export below.
//
// Because this is an integration test file, it can only access public items
// from the library crate (turingosv4) or from the binary via `#[path]`.
// We use `#[path]` to pull in the binary module directly.

#[path = "../src/bin/turingos/siliconflow_client.rs"]
#[allow(dead_code)]
mod siliconflow_client;

use siliconflow_client::maybe_rewrite_deepseek_model_error;

/// The canonical DeepSeek direct-API error body that triggers B1.
const DEEPSEEK_400_BODY: &str = r#"{"error":{"message":"The supported API model names are deepseek-v4-pro or deepseek-v4-flash, but you passed deepseek-ai/DeepSeek-V3.2.","type":"invalid_request_error","param":null,"code":null}}"#;

#[test]
fn deepseek_model_rejection_produces_actionable_message() {
    let result = maybe_rewrite_deepseek_model_error(DEEPSEEK_400_BODY);
    assert!(
        result.is_some(),
        "should rewrite DeepSeek model-name rejection; got None"
    );
    let msg = result.unwrap();
    assert!(
        msg.contains("LLM provider rejected model"),
        "message should contain 'LLM provider rejected model'; got:\n{msg}"
    );
    assert!(
        msg.contains("deepseek-v4-pro"),
        "message should suggest deepseek-v4-pro as meta-model; got:\n{msg}"
    );
    assert!(
        msg.contains("deepseek-v4-flash"),
        "message should suggest deepseek-v4-flash as blackbox-model; got:\n{msg}"
    );
    assert!(
        msg.contains("turingos llm config"),
        "message should include the remediation command; got:\n{msg}"
    );
    assert!(
        msg.contains("TURINGOS_SILICONFLOW_ENDPOINT"),
        "message should mention the endpoint env var; got:\n{msg}"
    );
}

#[test]
fn deepseek_rejection_extracts_rejected_model_name() {
    let result = maybe_rewrite_deepseek_model_error(DEEPSEEK_400_BODY);
    let msg = result.unwrap();
    // The rejected model name should appear in the message.
    assert!(
        msg.contains("deepseek-ai/DeepSeek-V3.2"),
        "rejected model name should appear in message; got:\n{msg}"
    );
}

#[test]
fn non_matching_4xx_passes_through_as_none() {
    // A generic 4xx that does NOT mention DeepSeek model names.
    let generic_body = r#"{"error":{"message":"Invalid API key","type":"auth_error"}}"#;
    let result = maybe_rewrite_deepseek_model_error(generic_body);
    assert!(
        result.is_none(),
        "non-DeepSeek errors must NOT be rewritten; got Some({:?})",
        result
    );
}

#[test]
fn only_partial_match_passes_through() {
    // Has "supported API model names" but NOT "deepseek-v4-" prefix.
    let partial = r#"{"error":{"message":"The supported API model names are gpt-4o, but you passed gpt-3.5."}}"#;
    let result = maybe_rewrite_deepseek_model_error(partial);
    assert!(
        result.is_none(),
        "partial match (no deepseek-v4-) must not rewrite; got Some"
    );
}

#[test]
fn qwen_model_name_in_error_also_triggers_rewrite() {
    // Simulates a Qwen model being sent to DeepSeek direct API.
    let body = r#"{"error":{"message":"The supported API model names are deepseek-v4-pro or deepseek-v4-flash, but you passed Qwen/Qwen3-Coder-30B-A3B-Instruct.","type":"invalid_request_error"}}"#;
    let result = maybe_rewrite_deepseek_model_error(body);
    assert!(
        result.is_some(),
        "Qwen model sent to DeepSeek API should also trigger rewrite"
    );
    let msg = result.unwrap();
    assert!(
        msg.contains("Qwen/Qwen3-Coder-30B-A3B-Instruct"),
        "should extract Qwen model name; got:\n{msg}"
    );
}
