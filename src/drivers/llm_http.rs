// Tier 5: Resilient HTTP client — local proxy only, never direct HTTPS
// Constitutional basis: Art. IV (Boot infrastructure)
// V3L-25: never direct HTTPS from Rust (TLS deadlock on certain endpoints)
// V3L-26: ThreadingMixIn on proxy side (single-thread = 502)
// V3L-27: rate limit handling (retry with backoff)

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::Path;
use std::path::PathBuf;
use std::time::Duration;

use crate::runtime::external_call::{
    ExternalCallIntent, ExternalCallOutbox, ExternalCallRecord, ExternalCallTerminal, Usage,
};

// ── Core types ──────────────────────────────────────────────────

/// LLM generation request.
#[derive(Debug, Serialize)]
pub struct GenerateRequest {
    pub model: String,
    pub messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

/// LLM generation response.
#[derive(Debug, Clone, Deserialize)]
pub struct GenerateResponse {
    pub content: String,
    pub completion_tokens: u32,
    /// API-reported prompt tokens. Falls back to 0 if `usage.prompt_tokens` is
    /// absent in the proxy response (older proxies). Surfaced for PPUT-CCL
    /// Phase B C_i accounting (post-hoc, not estimation — plan B2 default).
    pub prompt_tokens: u32,
    pub model: String,
}

/// Driver errors. V3L-09: explicit, never silent.
#[derive(Debug)]
pub enum DriverError {
    NetworkError(String),
    Timeout,
    RateLimited,
    ParseError(String),
    BackendError(String),
}

impl std::fmt::Display for DriverError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DriverError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            DriverError::Timeout => write!(f, "Request timeout"),
            DriverError::RateLimited => write!(f, "Rate limited (429)"),
            DriverError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            DriverError::BackendError(msg) => write!(f, "Backend error: {}", msg),
        }
    }
}

impl std::error::Error for DriverError {}

/// Resilient HTTP client that connects to a LOCAL proxy only.
/// V3L-25: NEVER connect directly to cloud HTTPS endpoints from Rust.
/// The proxy (llm_proxy.py) handles TLS, rate limits, and provider routing.
pub struct ResilientLLMClient {
    proxy_url: String,
    timeout: Duration,
    max_retries: u32,
}

pub struct RecordedLlmClient {
    inner: ResilientLLMClient,
    outbox_path: PathBuf,
    run_id: String,
    provider: String,
}

pub struct MockLlmTransport {
    response: Result<GenerateResponse, DriverError>,
    pub send_count: usize,
    pub send_seen_intent: bool,
}

impl MockLlmTransport {
    pub fn new(response: GenerateResponse) -> Self {
        Self {
            response: Ok(response),
            send_count: 0,
            send_seen_intent: false,
        }
    }

    pub fn failing(error: DriverError) -> Self {
        Self {
            response: Err(error),
            send_count: 0,
            send_seen_intent: false,
        }
    }

    fn send(
        &mut self,
        outbox_path: &Path,
        intent_id: &str,
        _request: &GenerateRequest,
    ) -> Result<GenerateResponse, DriverError> {
        let reopened = ExternalCallOutbox::open(outbox_path).map_err(DriverError::BackendError)?;
        self.send_seen_intent = reopened.ledger().has_pending_intent(intent_id);
        if !self.send_seen_intent {
            return Err(DriverError::BackendError(format!(
                "send attempted before durable intent {intent_id}"
            )));
        }
        self.send_count += 1;
        match &self.response {
            Ok(response) => Ok(response.clone()),
            Err(error) => Err(error.clone_for_recording()),
        }
    }
}

impl ResilientLLMClient {
    /// Create a client pointing to a LOCAL HTTP proxy.
    /// `proxy_url` must be http://localhost:PORT or http://127.0.0.1:PORT.
    pub fn new(proxy_url: &str, timeout_secs: u64, max_retries: u32) -> Self {
        ResilientLLMClient {
            proxy_url: proxy_url.to_string(),
            timeout: Duration::from_secs(timeout_secs),
            max_retries,
        }
    }

    /// Generate a completion via the local proxy.
    /// Retries on transient errors with exponential backoff.
    /// V3L-27: handles 429 rate limits gracefully.
    pub async fn generate(
        &self,
        request: &GenerateRequest,
    ) -> Result<GenerateResponse, DriverError> {
        let client = reqwest::Client::builder()
            .timeout(self.timeout)
            .build()
            .map_err(|e| DriverError::NetworkError(e.to_string()))?;

        let mut last_error = DriverError::NetworkError("No attempts made".into());

        for attempt in 0..=self.max_retries {
            if attempt > 0 {
                // Exponential backoff: 1s, 2s, 4s...
                let delay = Duration::from_secs(1 << (attempt - 1).min(4));
                tokio::time::sleep(delay).await;
            }

            match client
                .post(&format!("{}/v1/chat/completions", self.proxy_url))
                .json(request)
                .send()
                .await
            {
                Ok(response) => {
                    let status = response.status();
                    if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
                        last_error = DriverError::RateLimited;
                        continue;
                    }
                    if !status.is_success() {
                        let body = response.text().await.unwrap_or_default();
                        last_error =
                            DriverError::BackendError(format!("HTTP {}: {}", status, body));
                        continue;
                    }

                    // Parse OpenAI-compatible response
                    let body: serde_json::Value = response
                        .json()
                        .await
                        .map_err(|e| DriverError::ParseError(e.to_string()))?;

                    let content = body["choices"][0]["message"]["content"]
                        .as_str()
                        .unwrap_or("")
                        .to_string();
                    let tokens = body["usage"]["completion_tokens"].as_u64().unwrap_or(0) as u32;
                    let prompt_tokens = body["usage"]["prompt_tokens"].as_u64().unwrap_or(0) as u32;
                    let model = body["model"].as_str().unwrap_or(&request.model).to_string();

                    return Ok(GenerateResponse {
                        content,
                        completion_tokens: tokens,
                        prompt_tokens,
                        model,
                    });
                }
                Err(e) => {
                    if e.is_timeout() {
                        last_error = DriverError::Timeout;
                    } else {
                        last_error = DriverError::NetworkError(e.to_string());
                    }
                    continue;
                }
            }
        }

        Err(last_error)
    }
}

impl RecordedLlmClient {
    /// Create a recorded client. The default outbox is per-process to avoid
    /// parallel test collisions; production runners can pin it with
    /// TURINGOS_EXTERNAL_CALL_OUTBOX and TURINGOS_EXTERNAL_CALL_RUN_ID.
    pub fn new(proxy_url: &str, timeout_secs: u64, max_retries: u32) -> Self {
        Self {
            inner: ResilientLLMClient::new(proxy_url, timeout_secs, max_retries),
            outbox_path: default_external_call_outbox_path(),
            run_id: default_external_call_run_id(),
            provider: "local-proxy".to_string(),
        }
    }

    pub fn with_outbox(
        proxy_url: &str,
        timeout_secs: u64,
        max_retries: u32,
        outbox_path: impl Into<PathBuf>,
        run_id: impl Into<String>,
        provider: impl Into<String>,
    ) -> Self {
        Self {
            inner: ResilientLLMClient::new(proxy_url, timeout_secs, max_retries),
            outbox_path: outbox_path.into(),
            run_id: run_id.into(),
            provider: provider.into(),
        }
    }

    pub fn outbox_path(&self) -> &Path {
        &self.outbox_path
    }

    #[track_caller]
    pub fn generate_recorded<'a>(
        &'a self,
        request: &'a GenerateRequest,
    ) -> impl std::future::Future<Output = Result<GenerateResponse, DriverError>> + 'a {
        let caller = std::panic::Location::caller();
        let call_site = format!("{}:{}", caller.file(), caller.line());
        async move { self.generate_recorded_at(&call_site, request).await }
    }

    pub async fn generate_recorded_at(
        &self,
        call_site: &str,
        request: &GenerateRequest,
    ) -> Result<GenerateResponse, DriverError> {
        let mut outbox =
            ExternalCallOutbox::open(&self.outbox_path).map_err(DriverError::BackendError)?;
        let intent = build_recording_intent(
            &outbox,
            &self.run_id,
            call_site,
            &self.provider,
            request,
            self.inner.timeout.as_millis() as u64,
        )?;
        let intent_id = intent.intent_id.clone();
        outbox
            .append_intent(intent)
            .map_err(DriverError::BackendError)?;

        match self.inner.generate(request).await {
            Ok(response) => {
                let terminal = result_terminal_from_response(&response);
                outbox
                    .append_terminal(&intent_id, terminal)
                    .map_err(DriverError::BackendError)?;
                Ok(response)
            }
            Err(error) => {
                let terminal = terminal_from_driver_error(&error);
                outbox
                    .append_terminal(&intent_id, terminal)
                    .map_err(DriverError::BackendError)?;
                Err(error)
            }
        }
    }
}

pub fn recorded_generate_with_transport(
    outbox: &mut ExternalCallOutbox,
    transport: &mut MockLlmTransport,
    run_id: &str,
    call_site: &str,
    provider: &str,
    request: &GenerateRequest,
) -> Result<ExternalCallRecord, DriverError> {
    let intent = build_recording_intent(outbox, run_id, call_site, provider, request, 30_000)?;
    let intent_id = intent.intent_id.clone();

    outbox
        .append_intent(intent.clone())
        .map_err(DriverError::BackendError)?;
    let response = match transport.send(outbox.path(), &intent_id, request) {
        Ok(response) => response,
        Err(error) => {
            let terminal = terminal_from_driver_error(&error);
            outbox
                .append_terminal(&intent_id, terminal)
                .map_err(DriverError::BackendError)?;
            return Err(error);
        }
    };
    let terminal = result_terminal_from_response(&response);
    outbox
        .append_terminal(&intent_id, terminal.clone())
        .map_err(DriverError::BackendError)?;
    Ok(ExternalCallRecord {
        intent,
        terminal: Some(terminal),
    })
}

impl DriverError {
    fn clone_for_recording(&self) -> Self {
        match self {
            DriverError::NetworkError(msg) => DriverError::NetworkError(msg.clone()),
            DriverError::Timeout => DriverError::Timeout,
            DriverError::RateLimited => DriverError::RateLimited,
            DriverError::ParseError(msg) => DriverError::ParseError(msg.clone()),
            DriverError::BackendError(msg) => DriverError::BackendError(msg.clone()),
        }
    }
}

fn terminal_from_driver_error(error: &DriverError) -> ExternalCallTerminal {
    match error {
        DriverError::NetworkError(_) => ExternalCallTerminal::Failure {
            class: "transport_error".to_string(),
            retryable: true,
            public_summary: "transport error".to_string(),
        },
        DriverError::Timeout => ExternalCallTerminal::Failure {
            class: "http_timeout".to_string(),
            retryable: true,
            public_summary: "HTTP timeout".to_string(),
        },
        DriverError::RateLimited => ExternalCallTerminal::Failure {
            class: "http_429".to_string(),
            retryable: true,
            public_summary: "rate limited".to_string(),
        },
        DriverError::ParseError(_) => ExternalCallTerminal::Failure {
            class: "parse_error".to_string(),
            retryable: false,
            public_summary: "response parse failed".to_string(),
        },
        DriverError::BackendError(_) => ExternalCallTerminal::Failure {
            class: "backend_error".to_string(),
            retryable: false,
            public_summary: "backend error".to_string(),
        },
    }
}

fn build_recording_intent(
    outbox: &ExternalCallOutbox,
    run_id: &str,
    call_site: &str,
    provider: &str,
    request: &GenerateRequest,
    timeout_ms: u64,
) -> Result<ExternalCallIntent, DriverError> {
    let request_bytes =
        serde_json::to_vec(request).map_err(|e| DriverError::ParseError(e.to_string()))?;
    let request_hash = sha256_hex(&request_bytes);
    let logical_t = outbox.ledger().summary().intent_count as u64;
    let unique =
        sha256_hex(format!("{run_id}\0{call_site}\0{request_hash}\0{logical_t}").as_bytes());
    Ok(ExternalCallIntent {
        intent_id: format!("intent:{}", &unique[..16]),
        logical_call_id: format!("{run_id}:{call_site}:{logical_t}:{}", &request_hash[..12]),
        call_site: call_site.to_string(),
        run_id: run_id.to_string(),
        request_hash: request_hash.clone(),
        provider: provider.to_string(),
        model: Some(request.model.clone()),
        redacted_request_cid: format!("redacted-request:{}", &request_hash[..16]),
        idempotency_key: format!("{run_id}:{call_site}:{logical_t}:{request_hash}"),
        timeout_ms,
        logical_t,
    })
}

fn result_terminal_from_response(response: &GenerateResponse) -> ExternalCallTerminal {
    ExternalCallTerminal::Result {
        result_hash: sha256_hex(response.content.as_bytes()),
        usage: Usage {
            prompt_tokens: response.prompt_tokens as u64,
            completion_tokens: response.completion_tokens as u64,
            total_tokens: response.prompt_tokens as u64 + response.completion_tokens as u64,
        },
        status: 200,
        provider_request_id: None,
    }
}

fn default_external_call_outbox_path() -> PathBuf {
    if let Some(path) = std::env::var_os("TURINGOS_EXTERNAL_CALL_OUTBOX") {
        return PathBuf::from(path);
    }
    std::env::temp_dir().join(format!(
        "turingos_external_calls_{}.jsonl",
        std::process::id()
    ))
}

fn default_external_call_run_id() -> String {
    std::env::var("TURINGOS_EXTERNAL_CALL_RUN_ID")
        .unwrap_or_else(|_| format!("process-{}", std::process::id()))
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(bytes);
    h.finalize().iter().map(|b| format!("{b:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = ResilientLLMClient::new("http://localhost:8080", 120, 3);
        assert_eq!(client.proxy_url, "http://localhost:8080");
        assert_eq!(client.max_retries, 3);
    }

    #[test]
    fn test_generate_request_serialization() {
        let req = GenerateRequest {
            model: "deepseek-v3.2".into(),
            messages: vec![Message {
                role: "user".into(),
                content: "test".into(),
            }],
            temperature: Some(0.2),
            max_tokens: Some(8000),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("deepseek-v3.2"));
        assert!(json.contains("0.2"));
    }

    #[test]
    fn test_driver_error_display() {
        assert_eq!(
            format!("{}", DriverError::RateLimited),
            "Rate limited (429)"
        );
        assert_eq!(format!("{}", DriverError::Timeout), "Request timeout");
    }
}
