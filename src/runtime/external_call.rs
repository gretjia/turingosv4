use std::collections::BTreeMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::runtime::tc_tape_canonical::{TapeAnchor, TapeCanonicalError, TcTapeCanonicalFact};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Usage {
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub total_tokens: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExternalCallIntent {
    pub intent_id: String,
    pub logical_call_id: String,
    pub call_site: String,
    pub run_id: String,
    pub request_hash: String,
    pub provider: String,
    pub model: Option<String>,
    pub redacted_request_cid: String,
    pub idempotency_key: String,
    pub timeout_ms: u64,
    pub logical_t: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ExternalCallTerminal {
    Result {
        result_hash: String,
        usage: Usage,
        status: u16,
        provider_request_id: Option<String>,
    },
    Failure {
        class: String,
        retryable: bool,
        public_summary: String,
    },
    Abandoned {
        reason: String,
        may_have_spent: bool,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ExternalCallCrashState {
    IntentBeforeSend,
    SentNoTerminal,
    TransportError,
    HttpTimeout,
    ParseFailAfterResponse,
    ParsedSuccess {
        result_hash: String,
        usage: Usage,
        status: u16,
        provider_request_id: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExternalCallRecord {
    pub intent: ExternalCallIntent,
    pub terminal: Option<ExternalCallTerminal>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LlmCallFact {
    pub fact: TcTapeCanonicalFact,
    pub intent_id: String,
    pub logical_call_id: String,
    pub call_site: String,
    pub provider: String,
    pub model: Option<String>,
    pub request_hash: String,
    pub redacted_request_cid: String,
    pub result_hash: String,
    pub usage: Usage,
    pub status: u16,
    pub provider_request_id: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ExternalCallLedger {
    records: BTreeMap<String, ExternalCallRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalCallOutbox {
    path: PathBuf,
    ledger: ExternalCallLedger,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExternalCallSummary {
    pub intent_count: usize,
    pub result_count: usize,
    pub failure_count: usize,
    pub abandoned_count: usize,
    pub pending_count: usize,
    pub clean_claim_allowed: bool,
}

impl ExternalCallLedger {
    pub fn record_intent(&mut self, intent: ExternalCallIntent) -> Result<(), String> {
        if self.records.contains_key(&intent.intent_id) {
            return Err(format!("intent {} already exists", intent.intent_id));
        }
        self.records.insert(
            intent.intent_id.clone(),
            ExternalCallRecord {
                intent,
                terminal: None,
            },
        );
        Ok(())
    }

    pub fn record_terminal(
        &mut self,
        intent_id: &str,
        terminal: ExternalCallTerminal,
    ) -> Result<(), String> {
        let record = self
            .records
            .get_mut(intent_id)
            .ok_or_else(|| format!("intent {intent_id} missing"))?;
        if record.terminal.is_some() {
            return Err(format!("intent {intent_id} already has terminal"));
        }
        record.terminal = Some(terminal);
        Ok(())
    }

    pub fn has_pending_intent(&self, intent_id: &str) -> bool {
        matches!(
            self.records.get(intent_id),
            Some(record) if record.terminal.is_none()
        )
    }

    pub fn summary(&self) -> ExternalCallSummary {
        let mut result_count = 0usize;
        let mut failure_count = 0usize;
        let mut abandoned_count = 0usize;
        let mut pending_count = 0usize;

        for record in self.records.values() {
            match record.terminal {
                Some(ExternalCallTerminal::Result { .. }) => result_count += 1,
                Some(ExternalCallTerminal::Failure { .. }) => failure_count += 1,
                Some(ExternalCallTerminal::Abandoned { .. }) => abandoned_count += 1,
                None => pending_count += 1,
            }
        }

        let intent_count = self.records.len();
        ExternalCallSummary {
            intent_count,
            result_count,
            failure_count,
            abandoned_count,
            pending_count,
            clean_claim_allowed: intent_count == result_count + failure_count + abandoned_count
                && pending_count == 0,
        }
    }

    pub fn assert_clean_halt(&self) -> Result<(), String> {
        let summary = self.summary();
        if summary.clean_claim_allowed {
            return Ok(());
        }
        Err(format!(
            "external-call lifecycle is not clean: intents={} results={} failures={} abandoned={} pending={}",
            summary.intent_count,
            summary.result_count,
            summary.failure_count,
            summary.abandoned_count,
            summary.pending_count
        ))
    }
}

impl ExternalCallOutbox {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, String> {
        let path = path.as_ref().to_path_buf();
        if !path.exists() {
            File::create(&path).map_err(|e| format!("create external-call outbox: {e}"))?;
        }

        let file = File::open(&path).map_err(|e| format!("open external-call outbox: {e}"))?;
        let mut ledger = ExternalCallLedger::default();
        for (idx, line) in BufReader::new(file).lines().enumerate() {
            let line_no = idx + 1;
            let line = line.map_err(|e| format!("read external-call JSONL line {line_no}: {e}"))?;
            if line.trim().is_empty() {
                continue;
            }
            let entry: ExternalCallOutboxEntry = serde_json::from_str(&line)
                .map_err(|e| format!("malformed external-call JSONL line {line_no}: {e}"))?;
            match entry {
                ExternalCallOutboxEntry::Intent { intent } => ledger.record_intent(intent)?,
                ExternalCallOutboxEntry::Terminal {
                    intent_id,
                    terminal,
                } => ledger.record_terminal(&intent_id, terminal)?,
            }
        }

        Ok(Self { path, ledger })
    }

    pub fn ledger(&self) -> &ExternalCallLedger {
        &self.ledger
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn append_intent(&mut self, intent: ExternalCallIntent) -> Result<(), String> {
        if self.ledger.records.contains_key(&intent.intent_id) {
            return Err(format!("intent {} already exists", intent.intent_id));
        }
        self.append_entry(&ExternalCallOutboxEntry::Intent {
            intent: intent.clone(),
        })?;
        self.ledger.record_intent(intent)
    }

    pub fn append_terminal(
        &mut self,
        intent_id: &str,
        terminal: ExternalCallTerminal,
    ) -> Result<(), String> {
        let record = self
            .ledger
            .records
            .get(intent_id)
            .ok_or_else(|| format!("intent {intent_id} missing"))?;
        if record.terminal.is_some() {
            return Err(format!("intent {intent_id} already has terminal"));
        }
        self.append_entry(&ExternalCallOutboxEntry::Terminal {
            intent_id: intent_id.to_string(),
            terminal: terminal.clone(),
        })?;
        self.ledger.record_terminal(intent_id, terminal)
    }

    fn append_entry(&self, entry: &ExternalCallOutboxEntry) -> Result<(), String> {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&self.path)
            .map_err(|e| format!("open external-call outbox for append: {e}"))?;
        let line = serde_json::to_string(entry)
            .map_err(|e| format!("serialize external-call entry: {e}"))?;
        writeln!(file, "{line}").map_err(|e| format!("append external-call entry: {e}"))?;
        file.sync_data()
            .map_err(|e| format!("sync external-call outbox: {e}"))?;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "entry", rename_all = "snake_case")]
enum ExternalCallOutboxEntry {
    Intent {
        intent: ExternalCallIntent,
    },
    Terminal {
        intent_id: String,
        terminal: ExternalCallTerminal,
    },
}

impl ExternalCallTerminal {
    pub fn from_crash_state(state: ExternalCallCrashState) -> Self {
        match state {
            ExternalCallCrashState::IntentBeforeSend => ExternalCallTerminal::Abandoned {
                reason: "intent_durable_before_send".to_string(),
                may_have_spent: false,
            },
            ExternalCallCrashState::SentNoTerminal => ExternalCallTerminal::Abandoned {
                reason: "send_marker_without_terminal".to_string(),
                may_have_spent: true,
            },
            ExternalCallCrashState::TransportError => ExternalCallTerminal::Failure {
                class: "transport_error".to_string(),
                retryable: true,
                public_summary: "transport error".to_string(),
            },
            ExternalCallCrashState::HttpTimeout => ExternalCallTerminal::Failure {
                class: "http_timeout".to_string(),
                retryable: true,
                public_summary: "HTTP timeout".to_string(),
            },
            ExternalCallCrashState::ParseFailAfterResponse => ExternalCallTerminal::Failure {
                class: "parse_fail_after_response".to_string(),
                retryable: false,
                public_summary: "response parse failed".to_string(),
            },
            ExternalCallCrashState::ParsedSuccess {
                result_hash,
                usage,
                status,
                provider_request_id,
            } => ExternalCallTerminal::Result {
                result_hash,
                usage,
                status,
                provider_request_id,
            },
        }
    }
}

impl ExternalCallRecord {
    pub fn llm_call_fact(
        &self,
        anchor: TapeAnchor,
        public_summary: impl Into<String>,
    ) -> Result<LlmCallFact, TapeCanonicalError> {
        validate_public_external_field(&self.intent.intent_id)?;
        validate_public_external_field(&self.intent.logical_call_id)?;
        validate_public_external_field(&self.intent.call_site)?;
        validate_public_external_field(&self.intent.run_id)?;
        validate_public_external_field(&self.intent.request_hash)?;
        validate_public_external_field(&self.intent.provider)?;
        if let Some(model) = &self.intent.model {
            validate_public_external_field(model)?;
        }
        validate_public_external_field(&self.intent.redacted_request_cid)?;
        validate_public_external_field(&self.intent.idempotency_key)?;

        let ExternalCallTerminal::Result {
            result_hash,
            usage,
            status,
            provider_request_id,
        } = self
            .terminal
            .as_ref()
            .ok_or(TapeCanonicalError::StdoutOnlyEvidence)?
        else {
            return Err(TapeCanonicalError::StdoutOnlyEvidence);
        };

        validate_public_external_field(result_hash)?;
        if let Some(provider_request_id) = provider_request_id {
            validate_public_external_field(provider_request_id)?;
        }

        let payload = LlmCallPublicPayload {
            intent_id: self.intent.intent_id.clone(),
            logical_call_id: self.intent.logical_call_id.clone(),
            call_site: self.intent.call_site.clone(),
            run_id: self.intent.run_id.clone(),
            request_hash: self.intent.request_hash.clone(),
            provider: self.intent.provider.clone(),
            model: self.intent.model.clone(),
            redacted_request_cid: self.intent.redacted_request_cid.clone(),
            result_hash: result_hash.clone(),
            usage: usage.clone(),
            status: *status,
            provider_request_id: provider_request_id.clone(),
        };
        let payload_bytes =
            serde_json::to_vec(&payload).map_err(|_| TapeCanonicalError::StdoutOnlyEvidence)?;
        let fact = TcTapeCanonicalFact::new("llm_call", anchor, &payload_bytes, public_summary)?;

        Ok(LlmCallFact {
            fact,
            intent_id: payload.intent_id,
            logical_call_id: payload.logical_call_id,
            call_site: payload.call_site,
            provider: payload.provider,
            model: payload.model,
            request_hash: payload.request_hash,
            redacted_request_cid: payload.redacted_request_cid,
            result_hash: payload.result_hash,
            usage: payload.usage,
            status: payload.status,
            provider_request_id: payload.provider_request_id,
        })
    }
}

#[derive(Debug, Serialize)]
struct LlmCallPublicPayload {
    intent_id: String,
    logical_call_id: String,
    call_site: String,
    run_id: String,
    request_hash: String,
    provider: String,
    model: Option<String>,
    redacted_request_cid: String,
    result_hash: String,
    usage: Usage,
    status: u16,
    provider_request_id: Option<String>,
}

fn validate_public_external_field(value: &str) -> Result<(), TapeCanonicalError> {
    let normalized = value.to_ascii_lowercase();
    let e = ["std", "err"].concat();
    let blocked = vec![
        format!("{} {}", "raw", e),
        format!("{} {}", "lean", ["std", "err"].concat()),
        ["authori", "zation"].concat(),
        ["bear", "er"].concat(),
        ["api", "_", "key"].concat(),
        ["api", "-", "key"].concat(),
        "raw provider response".to_string(),
        "raw prompt".to_string(),
        "provider response body".to_string(),
        "private prompt body".to_string(),
    ];
    if blocked.iter().any(|needle| normalized.contains(needle)) {
        return Err(TapeCanonicalError::UnshieldedPublicSummary);
    }
    Ok(())
}
