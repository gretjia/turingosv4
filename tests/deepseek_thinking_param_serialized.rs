/// TRACE_MATRIX FC2-N16: Unit tests for DeepSeek thinking parameter serialization (P3).
///
/// Verifies that:
///   1. A ChatRequest-shaped JSON with `thinking: {type: "enabled"}` round-trips correctly.
///   2. A ChatRequest-shaped JSON with `thinking` absent does NOT include the key.
///   3. A sample response JSON containing both `content` and `reasoning_content` is
///      deserialized with both fields populated.
///
/// No live API calls are made. Uses only `serde_json`.
///
/// Note: `chat_client` lives in the binary crate (`src/bin/turingos/`), not
/// in the library crate, so we test the JSON contract directly via `serde_json::Value`
/// round-trips and struct-level serialization using local mirror structs that match
/// the exact wire shape.

use serde::{Deserialize, Serialize};

/// Mirror of `ThinkingConfig` from chat_client — same serde contract.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ThinkingConfig {
    #[serde(rename = "type")]
    kind: String,
}

/// Mirror of `ChatRequest` from chat_client — same serde contract.
#[derive(Debug, Serialize)]
struct ChatRequest<'a> {
    model: &'a str,
    messages: &'a [serde_json::Value],
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thinking: Option<ThinkingConfig>,
}

#[test]
fn thinking_enabled_serialized_to_request_json() {
    let messages: Vec<serde_json::Value> = vec![serde_json::json!({"role": "user", "content": "hello"})];
    let req = ChatRequest {
        model: "deepseek-v4-pro",
        messages: &messages,
        max_tokens: Some(3000),
        temperature: Some(0.3),
        thinking: Some(ThinkingConfig {
            kind: "enabled".to_string(),
        }),
    };
    let json = serde_json::to_string(&req).expect("serialization must not fail");
    assert!(
        json.contains(r#""thinking":{"type":"enabled"}"#),
        "expected JSON to contain thinking field with type=enabled; got: {json}"
    );
}

#[test]
fn thinking_none_omitted_from_request_json() {
    let messages: Vec<serde_json::Value> = vec![serde_json::json!({"role": "user", "content": "hello"})];
    let req = ChatRequest {
        model: "deepseek-v4-flash",
        messages: &messages,
        max_tokens: Some(1000),
        temperature: Some(0.2),
        thinking: None,
    };
    let json = serde_json::to_string(&req).expect("serialization must not fail");
    assert!(
        !json.contains("thinking"),
        "expected JSON to NOT contain thinking key when None (skip_serializing_if must work); got: {json}"
    );
}

#[test]
fn reasoning_content_decoded_from_response_json() {
    // Simulate the response body a DeepSeek thinking-mode API call returns.
    // We verify both `content` and `reasoning_content` fields decode correctly
    // from the JSON shape the wire returns.
    let response_json = r#"{
        "choices": [
            {
                "message": {
                    "role": "assistant",
                    "content": "The answer is 42.",
                    "reasoning_content": "I thought carefully about this and determined that 42 is correct."
                },
                "finish_reason": "stop"
            }
        ],
        "usage": {
            "prompt_tokens": 10,
            "completion_tokens": 20,
            "total_tokens": 30
        }
    }"#;

    let val: serde_json::Value =
        serde_json::from_str(response_json).expect("sample response must parse");

    let content = val["choices"][0]["message"]["content"]
        .as_str()
        .expect("content field must be present and a string");
    assert_eq!(content, "The answer is 42.");

    let reasoning = val["choices"][0]["message"]["reasoning_content"]
        .as_str()
        .expect("reasoning_content field must be present and a string");
    assert!(
        reasoning.contains("42"),
        "reasoning_content should contain '42'; got: {reasoning}"
    );

    // Also verify the mirror struct can deserialize reasoning_content.
    #[derive(Debug, Deserialize)]
    struct MsgOwned {
        #[allow(dead_code)]
        role: String,
        content: String,
        #[serde(default)]
        reasoning_content: Option<String>,
    }
    #[derive(Debug, Deserialize)]
    struct Choice {
        message: MsgOwned,
        #[serde(default)]
        finish_reason: Option<String>,
    }
    #[derive(Debug, Deserialize)]
    struct Response {
        choices: Vec<Choice>,
    }

    let parsed: Response = serde_json::from_str(response_json)
        .expect("response must deserialize into mirror struct");

    let first = &parsed.choices[0];
    assert_eq!(first.message.content, "The answer is 42.");
    assert_eq!(
        first.message.reasoning_content.as_deref(),
        Some("I thought carefully about this and determined that 42 is correct.")
    );
    assert_eq!(first.finish_reason.as_deref(), Some("stop"));
}
