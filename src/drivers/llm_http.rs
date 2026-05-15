// Tier 5: Resilient HTTP client — local proxy only, never direct HTTPS
// Constitutional basis: Art. IV (Boot infrastructure)
// V3L-25: never direct HTTPS from Rust (TLS deadlock on certain endpoints)
// V3L-26: ThreadingMixIn on proxy side (single-thread = 502)
// V3L-27: rate limit handling (retry with backoff)

use serde::{Deserialize, Serialize};
use std::time::Duration;

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
#[derive(Debug, Deserialize)]
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
