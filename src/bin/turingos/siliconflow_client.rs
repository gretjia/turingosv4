//! TRACE_MATRIX FC2-N16: SiliconFlow HTTP client (Phase 6.3 real-LLM wire)
//!
//! Thin async client for the SiliconFlow OpenAI-compatible Chat Completions
//! API at https://api.siliconflow.cn/v1/chat/completions. Used by
//! `turingos spec` (Meta AI / reasoning) and `turingos generate`
//! (Blackbox AI / fast codegen).
//!
//! Architecture posture: SiliconFlow is one provider — the wire surface is
//! intentionally written so that swapping in another OpenAI-compatible
//! provider later requires changing only the endpoint constant and the
//! model_id strings, not the message-shape code.
//!
//! Phase 6.3 scope: chat completions (non-streaming). Streaming and tool-use
//! deferred to a future atom.

use std::time::Duration;

use serde::{Deserialize, Serialize};

/// TRACE_MATRIX FC2-N16: SiliconFlow default API endpoint (mainland China region).
pub(crate) const SILICONFLOW_ENDPOINT: &str = "https://api.siliconflow.cn/v1/chat/completions";

/// TRACE_MATRIX FC2-N16: Default reasoning model (Meta AI role).
/// Research note: DeepSeek-V3.2 — ¥2/M in, ¥3/M out, 160K ctx, Chinese-first.
pub(crate) const DEFAULT_META_MODEL: &str = "deepseek-ai/DeepSeek-V3.2";

/// TRACE_MATRIX FC2-N16: Default fast / codegen model (Blackbox AI role).
/// Research note: Qwen3-Coder-30B-A3B-Instruct — code MoE, 256K ctx.
pub(crate) const DEFAULT_BLACKBOX_MODEL: &str = "Qwen/Qwen3-Coder-30B-A3B-Instruct";

/// TRACE_MATRIX FC2-N16: Default API base — overridable via
/// TURINGOS_SILICONFLOW_ENDPOINT env var (e.g. for the international mirror
/// api.siliconflow.com or for a test localhost stub).
pub(crate) fn endpoint() -> String {
    std::env::var("TURINGOS_SILICONFLOW_ENDPOINT")
        .unwrap_or_else(|_| SILICONFLOW_ENDPOINT.to_string())
}

/// TRACE_MATRIX FC2-N16: Chat message envelope (OpenAI-compatible role+content).
#[derive(Debug, Clone, Serialize)]
pub(crate) struct ChatMessage {
    pub role: String,
    pub content: String,
}

impl ChatMessage {
    /// TRACE_MATRIX FC2-N16: build a "system" role message
    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: "system".into(),
            content: content.into(),
        }
    }
    /// TRACE_MATRIX FC2-N16: build a "user" role message
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: "user".into(),
            content: content.into(),
        }
    }
    /// TRACE_MATRIX FC2-N16: build an "assistant" role message
    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: "assistant".into(),
            content: content.into(),
        }
    }
}

#[derive(Debug, Serialize)]
struct ChatRequest<'a> {
    model: &'a str,
    messages: &'a [ChatMessage],
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
}

#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<ChatChoice>,
    #[serde(default)]
    usage: Option<Usage>,
}

#[derive(Debug, Deserialize)]
struct ChatChoice {
    message: ChatMessageOwned,
    #[serde(default)]
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ChatMessageOwned {
    #[allow(dead_code)]
    role: String,
    content: String,
}

/// TRACE_MATRIX FC2-N16: OpenAI-compatible token-usage record returned by chat-complete.
#[derive(Debug, Deserialize, Default, Clone)]
pub(crate) struct Usage {
    #[serde(default)]
    pub prompt_tokens: u64,
    #[serde(default)]
    pub completion_tokens: u64,
    #[serde(default)]
    pub total_tokens: u64,
}

/// TRACE_MATRIX FC2-N16: full chat-completion result handed back to CLI callers.
#[derive(Debug)]
pub(crate) struct ChatResult {
    pub content: String,
    pub usage: Usage,
    pub finish_reason: Option<String>,
}

/// TRACE_MATRIX FC2-N16: SiliconFlow client error taxonomy (missing key, HTTP error, transport, decode).
#[derive(Debug)]
pub(crate) enum LlmError {
    MissingApiKey { env_var: String },
    HttpStatus { status: u16, body: String },
    Transport(String),
    DecodeError(String),
    NoChoices,
}

impl std::fmt::Display for LlmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingApiKey { env_var } => write!(
                f,
                "missing API key: set env var {env_var} to your SiliconFlow API key"
            ),
            Self::HttpStatus { status, body } => {
                write!(f, "HTTP {status} from SiliconFlow: {body}")
            }
            Self::Transport(e) => write!(f, "HTTP transport error: {e}"),
            Self::DecodeError(e) => write!(f, "response decode error: {e}"),
            Self::NoChoices => write!(f, "SiliconFlow returned 0 choices"),
        }
    }
}

impl std::error::Error for LlmError {}

/// TRACE_MATRIX FC2-N16: async chat-completion against SiliconFlow.
///
/// Call a SiliconFlow chat-completion. Returns the assistant message content +
/// usage tokens. Blocking-friendly: callers wrap with a tokio current-thread
/// runtime (see cmd_spec.rs / cmd_generate.rs).
pub(crate) async fn chat_complete(
    api_key: &str,
    model: &str,
    messages: &[ChatMessage],
    max_tokens: Option<u32>,
    temperature: Option<f32>,
) -> Result<ChatResult, LlmError> {
    let url = endpoint();
    let body = ChatRequest {
        model,
        messages,
        max_tokens,
        temperature,
    };

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(180))
        .build()
        .map_err(|e| LlmError::Transport(e.to_string()))?;

    let resp = client
        .post(&url)
        .bearer_auth(api_key)
        .json(&body)
        .send()
        .await
        .map_err(|e| LlmError::Transport(e.to_string()))?;

    let status = resp.status();
    let text = resp
        .text()
        .await
        .map_err(|e| LlmError::Transport(e.to_string()))?;

    if !status.is_success() {
        return Err(LlmError::HttpStatus {
            status: status.as_u16(),
            body: text,
        });
    }

    let parsed: ChatResponse =
        serde_json::from_str(&text).map_err(|e| LlmError::DecodeError(e.to_string()))?;

    let first = parsed
        .choices
        .into_iter()
        .next()
        .ok_or(LlmError::NoChoices)?;
    Ok(ChatResult {
        content: first.message.content,
        usage: parsed.usage.unwrap_or_default(),
        finish_reason: first.finish_reason,
    })
}

/// TRACE_MATRIX FC2-N16: sync wrapper over chat_complete (spins a current-thread tokio runtime).
///
/// Block on chat_complete from a sync context. Spins up a current-thread
/// tokio runtime per call — fine for CLI usage (one or two calls per command).
#[allow(dead_code)]
pub(crate) fn chat_complete_blocking(
    api_key: &str,
    model: &str,
    messages: &[ChatMessage],
    max_tokens: Option<u32>,
    temperature: Option<f32>,
) -> Result<ChatResult, LlmError> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|e| LlmError::Transport(format!("tokio runtime: {e}")))?;
    rt.block_on(chat_complete(
        api_key,
        model,
        messages,
        max_tokens,
        temperature,
    ))
}

/// TRACE_MATRIX FC2-N16: env-var-only API-key read (NEVER persists to disk).
///
/// Read API key from the configured env var. Returns MissingApiKey if unset
/// or empty (so callers can print a clear setup hint).
pub(crate) fn require_api_key(env_var: &str) -> Result<String, LlmError> {
    match std::env::var(env_var) {
        Ok(v) if !v.is_empty() => Ok(v),
        _ => Err(LlmError::MissingApiKey {
            env_var: env_var.to_string(),
        }),
    }
}
