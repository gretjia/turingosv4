//! A06 external call outbox substrate.
//!
//! The module records external call attempts as a two-event tape contract:
//! `PendingIntent` before the provider boundary, then exactly one
//! `TerminalExternalCall` after success, failure, timeout, or boot recovery.
//! Provider execution is injected by trait, so replay and boot repair remain
//! offline and deterministic.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::bottom_white::cas::{CasError, CasStore, Cid, ObjectType};
use crate::bottom_white::ledger::transition_ledger::{canonical_encode, CanonicalCodecError};
use crate::runtime::tape_event::{TapeEventEnvelope, TapeEventError, TapeEventKind, TapeEventRef};

/// TRACE_MATRIX Art.0.2 + FC1-N7 + FC1-N13 + FC2-N22: CAS schema id for A06 external call Intent payloads.
pub const EXTERNAL_CALL_INTENT_SCHEMA_ID: &str = "turingos.external_call.intent.v1";

/// TRACE_MATRIX Art.0.2 + FC1-N7 + FC1-N13 + FC2-N22: CAS schema id for A06 external call Terminal payloads.
pub const EXTERNAL_CALL_TERMINAL_SCHEMA_ID: &str = "turingos.external_call.terminal.v1";

/// TRACE_MATRIX Art.0.2 + FC1-N7 + FC1-N13 + FC2-N22: CAS schema id for provider response bytes captured by the outbox.
pub const EXTERNAL_CALL_RESPONSE_SCHEMA_ID: &str = "turingos.external_call.response.v1";

/// TRACE_MATRIX Art.0.2 + FC1-N7 + FC1-N13: request descriptor recorded before an external provider boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalCallRequest {
    pub call_id: String,
    pub provider: String,
    pub operation: String,
    pub request_cid: Cid,
    pub idempotency_key: String,
    pub provider_supports_idempotency: bool,
}

/// TRACE_MATRIX Art.0.2 + FC1-N7 + FC1-N13: tape-visible external call Intent payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExternalCallIntent {
    pub call_id: String,
    pub logical_t: u64,
    pub provider: String,
    pub operation: String,
    pub request_cid: Cid,
    pub idempotency_key: String,
    pub provider_supports_idempotency: bool,
}

/// TRACE_MATRIX Art.0.2 + FC1-N7 + FC1-N13: terminal status for a tape-visible external call.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExternalCallStatus {
    Succeeded,
    Failed,
    TimedOut,
    Abandoned,
}

/// TRACE_MATRIX Art.0.2 + FC1-N7 + FC1-N13: tape-visible terminal payload closing one external call Intent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExternalCallTerminal {
    pub call_id: String,
    pub intent_logical_t: u64,
    pub terminal_logical_t: u64,
    pub status: ExternalCallStatus,
    pub response_cid: Option<Cid>,
    pub error_class: Option<String>,
    pub may_have_spent: bool,
}

/// TRACE_MATRIX Art.0.2 + FC1-N7 + FC1-N13: provider result normalized before writing the Terminal event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExternalCallProviderResult {
    Success {
        response_bytes: Vec<u8>,
    },
    Failed {
        error_class: String,
        may_have_spent: bool,
    },
    TimedOut {
        error_class: String,
        may_have_spent: bool,
    },
}

/// TRACE_MATRIX Art.0.2 + FC1-N7 + FC1-N13: evidence made available to a provider after Intent is on tape.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalCallProviderContext {
    pub intent_cid: Cid,
    pub intent_event: TapeEventEnvelope,
    pub record_count_before_provider: usize,
}

/// TRACE_MATRIX Art.0.2 + FC1-N7 + FC1-N13: injected provider boundary so replay code never imports network clients.
pub trait ExternalCallProvider {
    fn call(
        &mut self,
        intent: &ExternalCallIntent,
        context: &ExternalCallProviderContext,
    ) -> ExternalCallProviderResult;
}

/// TRACE_MATRIX Art.0.2 + FC1-N7 + FC1-N13: typed payload carried beside a TapeEvent envelope in tests and derived projections.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExternalCallPayload {
    Intent(ExternalCallIntent),
    Terminal(ExternalCallTerminal),
}

/// TRACE_MATRIX Art.0.2 + FC1-N7 + FC1-N13: one in-memory tape event record over a CAS payload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalCallTapeRecord {
    pub envelope: TapeEventEnvelope,
    pub payload_cid: Cid,
    pub payload: ExternalCallPayload,
}

/// TRACE_MATRIX Art.0.2 + FC1-N7 + FC1-N13: return value for an Intent write.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordedExternalCallIntent {
    pub intent: ExternalCallIntent,
    pub intent_cid: Cid,
    pub intent_event: TapeEventEnvelope,
}

/// TRACE_MATRIX Art.0.2 + FC1-N7 + FC1-N13: return value for a full external call outbox execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalCallExecution {
    pub intent_cid: Cid,
    pub terminal_cid: Cid,
    pub response_cid: Option<Cid>,
    pub terminal: ExternalCallTerminal,
}

/// TRACE_MATRIX Art.0.2 + FC1-N7 + FC1-N13: single-writer test recorder for A06 outbox events over one ChainTape head.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalCallRecorder {
    head_oid: String,
    next_logical_t: u64,
    records: Vec<ExternalCallTapeRecord>,
}

impl ExternalCallRecorder {
    /// TRACE_MATRIX Art.0.2 + FC1-N7 + FC1-N13: create a recorder bound to the current ChainTape head OID.
    pub fn new(head_oid: String) -> Self {
        Self {
            head_oid,
            next_logical_t: 1,
            records: Vec::new(),
        }
    }

    /// TRACE_MATRIX Art.0.2 + FC1-N7 + FC1-N13: expose the append-only event slice for deterministic derivation.
    pub fn records(&self) -> &[ExternalCallTapeRecord] {
        &self.records
    }

    /// TRACE_MATRIX Art.0.2 + FC1-N7 + FC1-N13: expose the next event logical time for boot repair code.
    pub fn next_logical_t(&self) -> u64 {
        self.next_logical_t
    }

    /// TRACE_MATRIX Art.0.2 + FC1-N7 + FC1-N13: record the PendingIntent event before a provider call can run.
    pub fn record_intent(
        &mut self,
        cas: &mut CasStore,
        request: &ExternalCallRequest,
        creator: &str,
    ) -> Result<RecordedExternalCallIntent, ExternalCallError> {
        let logical_t = self.next_logical_t;
        let intent = ExternalCallIntent {
            call_id: request.call_id.clone(),
            logical_t,
            provider: request.provider.clone(),
            operation: request.operation.clone(),
            request_cid: request.request_cid,
            idempotency_key: request.idempotency_key.clone(),
            provider_supports_idempotency: request.provider_supports_idempotency,
        };
        let payload = ExternalCallPayload::Intent(intent.clone());
        let payload_cid = write_external_call_payload(
            cas,
            &payload,
            EXTERNAL_CALL_INTENT_SCHEMA_ID,
            creator,
            logical_t,
        )?;
        let event = TapeEventEnvelope {
            logical_t,
            tape_ref: TapeEventRef::Pending {
                head_oid: self.head_oid.clone(),
            },
            kind: TapeEventKind::PendingIntent,
            payload_cid: Some(payload_cid),
            source_tx_kind: None,
        };
        event.validate()?;
        self.records.push(ExternalCallTapeRecord {
            envelope: event.clone(),
            payload_cid,
            payload,
        });
        self.next_logical_t += 1;
        Ok(RecordedExternalCallIntent {
            intent,
            intent_cid: payload_cid,
            intent_event: event,
        })
    }

    /// TRACE_MATRIX Art.0.2 + FC1-N7 + FC1-N13 + FC2-N22: append an Abandoned Terminal for crash recovery.
    pub fn record_abandoned_terminal(
        &mut self,
        cas: &mut CasStore,
        intent: &ExternalCallIntent,
        reason: &str,
        may_have_spent: bool,
        creator: &str,
    ) -> Result<Cid, ExternalCallError> {
        let terminal = ExternalCallTerminal {
            call_id: intent.call_id.clone(),
            intent_logical_t: intent.logical_t,
            terminal_logical_t: self.next_logical_t,
            status: ExternalCallStatus::Abandoned,
            response_cid: None,
            error_class: Some(reason.to_string()),
            may_have_spent,
        };
        self.record_terminal(cas, terminal, creator)
    }

    fn record_terminal(
        &mut self,
        cas: &mut CasStore,
        terminal: ExternalCallTerminal,
        creator: &str,
    ) -> Result<Cid, ExternalCallError> {
        if terminal.terminal_logical_t != self.next_logical_t {
            return Err(ExternalCallError::TerminalLogicalTMismatch {
                call_id: terminal.call_id,
            });
        }
        let logical_t = terminal.terminal_logical_t;
        let payload = ExternalCallPayload::Terminal(terminal);
        let payload_cid = write_external_call_payload(
            cas,
            &payload,
            EXTERNAL_CALL_TERMINAL_SCHEMA_ID,
            creator,
            logical_t,
        )?;
        let event = TapeEventEnvelope {
            logical_t,
            tape_ref: TapeEventRef::Terminal {
                head_oid: self.head_oid.clone(),
            },
            kind: TapeEventKind::TerminalExternalCall,
            payload_cid: Some(payload_cid),
            source_tx_kind: None,
        };
        event.validate()?;
        self.records.push(ExternalCallTapeRecord {
            envelope: event,
            payload_cid,
            payload,
        });
        self.next_logical_t += 1;
        Ok(payload_cid)
    }
}

/// TRACE_MATRIX Art.0.2 + FC1-N7 + FC1-N13: execute one provider call only after the Intent is durable in CAS/tape.
pub fn execute_external_call<P: ExternalCallProvider>(
    cas: &mut CasStore,
    recorder: &mut ExternalCallRecorder,
    request: &ExternalCallRequest,
    creator: &str,
    provider: &mut P,
) -> Result<ExternalCallExecution, ExternalCallError> {
    let recorded = recorder.record_intent(cas, request, creator)?;
    let context = ExternalCallProviderContext {
        intent_cid: recorded.intent_cid,
        intent_event: recorded.intent_event.clone(),
        record_count_before_provider: recorder.records.len(),
    };
    let result = provider.call(&recorded.intent, &context);
    let terminal_logical_t = recorder.next_logical_t();
    let (status, response_cid, error_class, may_have_spent) = match result {
        ExternalCallProviderResult::Success { response_bytes } => {
            let response_cid = cas.put(
                &response_bytes,
                ObjectType::Generic,
                creator,
                terminal_logical_t,
                Some(EXTERNAL_CALL_RESPONSE_SCHEMA_ID.to_string()),
            )?;
            (
                ExternalCallStatus::Succeeded,
                Some(response_cid),
                None,
                true,
            )
        }
        ExternalCallProviderResult::Failed {
            error_class,
            may_have_spent,
        } => (
            ExternalCallStatus::Failed,
            None,
            Some(error_class),
            may_have_spent,
        ),
        ExternalCallProviderResult::TimedOut {
            error_class,
            may_have_spent,
        } => (
            ExternalCallStatus::TimedOut,
            None,
            Some(error_class),
            may_have_spent,
        ),
    };
    let terminal = ExternalCallTerminal {
        call_id: recorded.intent.call_id.clone(),
        intent_logical_t: recorded.intent.logical_t,
        terminal_logical_t,
        status,
        response_cid,
        error_class,
        may_have_spent,
    };
    let terminal_cid = recorder.record_terminal(cas, terminal.clone(), creator)?;
    Ok(ExternalCallExecution {
        intent_cid: recorded.intent_cid,
        terminal_cid,
        response_cid,
        terminal,
    })
}

/// TRACE_MATRIX Art.0.2 + FC1-N7 + FC1-N13: reconstruct a retry request without minting a new logical call identity.
pub fn retry_request_from_intent(
    intent: &ExternalCallIntent,
) -> Result<ExternalCallRequest, ExternalCallError> {
    if !intent.provider_supports_idempotency {
        return Err(ExternalCallError::ProviderNotIdempotent {
            call_id: intent.call_id.clone(),
        });
    }
    Ok(ExternalCallRequest {
        call_id: intent.call_id.clone(),
        provider: intent.provider.clone(),
        operation: intent.operation.clone(),
        request_cid: intent.request_cid,
        idempotency_key: intent.idempotency_key.clone(),
        provider_supports_idempotency: intent.provider_supports_idempotency,
    })
}

/// TRACE_MATRIX Art.0.2 + FC1-N7 + FC1-N13 + FC2-N22: derived state from A06 outbox tape records.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalCallState {
    intents: BTreeMap<String, ExternalCallIntent>,
    terminals: BTreeMap<String, ExternalCallTerminal>,
}

impl ExternalCallState {
    /// TRACE_MATRIX Art.0.2 + FC1-N7 + FC1-N13 + FC2-N22: derive pending/terminal state from tape records only.
    pub fn derive_from_tape(records: &[ExternalCallTapeRecord]) -> Result<Self, ExternalCallError> {
        let mut intents = BTreeMap::new();
        let mut terminals = BTreeMap::new();

        for record in records {
            validate_record(record)?;
            match &record.payload {
                ExternalCallPayload::Intent(intent) => {
                    if intents
                        .insert(intent.call_id.clone(), intent.clone())
                        .is_some()
                    {
                        return Err(ExternalCallError::DuplicateIntent {
                            call_id: intent.call_id.clone(),
                        });
                    }
                }
                ExternalCallPayload::Terminal(terminal) => {
                    let intent = intents.get(&terminal.call_id).ok_or_else(|| {
                        ExternalCallError::TerminalWithoutIntent {
                            call_id: terminal.call_id.clone(),
                        }
                    })?;
                    if intent.logical_t != terminal.intent_logical_t {
                        return Err(ExternalCallError::TerminalIntentLogicalTMismatch {
                            call_id: terminal.call_id.clone(),
                        });
                    }
                    if terminals
                        .insert(terminal.call_id.clone(), terminal.clone())
                        .is_some()
                    {
                        return Err(ExternalCallError::DuplicateTerminal {
                            call_id: terminal.call_id.clone(),
                        });
                    }
                }
            }
        }

        Ok(Self { intents, terminals })
    }

    /// TRACE_MATRIX Art.0.2 + FC1-N7 + FC1-N13 + FC2-N22: true only when every Intent has exactly one Terminal.
    pub fn clean_halt_allowed(&self) -> bool {
        self.pending_call_ids().is_empty()
    }

    /// TRACE_MATRIX Art.0.2 + FC1-N7 + FC1-N13 + FC2-N22: fail-closed clean-halt predicate for callers.
    pub fn require_clean_halt(&self) -> Result<(), ExternalCallError> {
        if self.clean_halt_allowed() {
            Ok(())
        } else {
            Err(ExternalCallError::PendingIntents)
        }
    }

    /// TRACE_MATRIX Art.0.2 + FC1-N7 + FC1-N13 + FC2-N22: sorted call IDs still missing a Terminal.
    pub fn pending_call_ids(&self) -> Vec<String> {
        self.intents
            .keys()
            .filter(|call_id| !self.terminals.contains_key(*call_id))
            .cloned()
            .collect()
    }

    /// TRACE_MATRIX Art.0.2 + FC1-N7 + FC1-N13 + FC2-N22: lookup the Terminal payload for one call id.
    pub fn terminal_for(&self, call_id: &str) -> Option<&ExternalCallTerminal> {
        self.terminals.get(call_id)
    }

    /// TRACE_MATRIX Art.0.2 + FC1-N7 + FC1-N13 + FC2-N22: sorted Intents still missing Terminal payloads.
    pub fn pending_intents(&self) -> Vec<&ExternalCallIntent> {
        self.intents
            .iter()
            .filter(|(call_id, _)| !self.terminals.contains_key(*call_id))
            .map(|(_, intent)| intent)
            .collect()
    }
}

/// TRACE_MATRIX Art.0.2 + FC1-N7 + FC1-N13 + FC2-N22: typed errors for fail-closed outbox projection and repair.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExternalCallError {
    Cas(String),
    Codec(String),
    Tape(String),
    MissingPayloadCid { logical_t: u64 },
    PayloadCidMismatch { logical_t: u64 },
    PayloadKindMismatch { logical_t: u64 },
    IntentLogicalTMismatch { call_id: String },
    TerminalLogicalTMismatch { call_id: String },
    TerminalIntentLogicalTMismatch { call_id: String },
    DuplicateIntent { call_id: String },
    DuplicateTerminal { call_id: String },
    TerminalWithoutIntent { call_id: String },
    PendingIntents,
    ProviderNotIdempotent { call_id: String },
}

impl std::fmt::Display for ExternalCallError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cas(message) => write!(f, "cas error: {message}"),
            Self::Codec(message) => write!(f, "codec error: {message}"),
            Self::Tape(message) => write!(f, "tape event error: {message}"),
            Self::MissingPayloadCid { logical_t } => {
                write!(
                    f,
                    "external call event at logical_t={logical_t} missing payload cid"
                )
            }
            Self::PayloadCidMismatch { logical_t } => {
                write!(
                    f,
                    "external call event at logical_t={logical_t} payload cid mismatch"
                )
            }
            Self::PayloadKindMismatch { logical_t } => {
                write!(
                    f,
                    "external call event at logical_t={logical_t} payload kind mismatch"
                )
            }
            Self::IntentLogicalTMismatch { call_id } => {
                write!(f, "intent logical_t mismatch for {call_id}")
            }
            Self::TerminalLogicalTMismatch { call_id } => {
                write!(f, "terminal logical_t mismatch for {call_id}")
            }
            Self::TerminalIntentLogicalTMismatch { call_id } => {
                write!(f, "terminal intent logical_t mismatch for {call_id}")
            }
            Self::DuplicateIntent { call_id } => write!(f, "duplicate intent for {call_id}"),
            Self::DuplicateTerminal { call_id } => write!(f, "duplicate terminal for {call_id}"),
            Self::TerminalWithoutIntent { call_id } => {
                write!(f, "terminal without matching intent for {call_id}")
            }
            Self::PendingIntents => write!(f, "pending external call intents remain open"),
            Self::ProviderNotIdempotent { call_id } => {
                write!(
                    f,
                    "provider does not support idempotent retry for {call_id}"
                )
            }
        }
    }
}

impl std::error::Error for ExternalCallError {}

impl From<CasError> for ExternalCallError {
    fn from(value: CasError) -> Self {
        Self::Cas(value.to_string())
    }
}

impl From<CanonicalCodecError> for ExternalCallError {
    fn from(value: CanonicalCodecError) -> Self {
        Self::Codec(value.to_string())
    }
}

impl From<TapeEventError> for ExternalCallError {
    fn from(value: TapeEventError) -> Self {
        Self::Tape(value.to_string())
    }
}

fn write_external_call_payload(
    cas: &mut CasStore,
    payload: &ExternalCallPayload,
    schema_id: &str,
    creator: &str,
    logical_t: u64,
) -> Result<Cid, ExternalCallError> {
    let bytes = canonical_encode(payload)?;
    Ok(cas.put(
        &bytes,
        ObjectType::Generic,
        creator,
        logical_t,
        Some(schema_id.to_string()),
    )?)
}

fn validate_record(record: &ExternalCallTapeRecord) -> Result<(), ExternalCallError> {
    record.envelope.validate()?;
    if record.envelope.payload_cid.is_none() {
        return Err(ExternalCallError::MissingPayloadCid {
            logical_t: record.envelope.logical_t,
        });
    }
    if record.envelope.payload_cid != Some(record.payload_cid) {
        return Err(ExternalCallError::PayloadCidMismatch {
            logical_t: record.envelope.logical_t,
        });
    }
    match (&record.envelope.kind, &record.payload) {
        (TapeEventKind::PendingIntent, ExternalCallPayload::Intent(intent)) => {
            if intent.logical_t == record.envelope.logical_t {
                Ok(())
            } else {
                Err(ExternalCallError::IntentLogicalTMismatch {
                    call_id: intent.call_id.clone(),
                })
            }
        }
        (TapeEventKind::TerminalExternalCall, ExternalCallPayload::Terminal(terminal)) => {
            if terminal.terminal_logical_t == record.envelope.logical_t {
                Ok(())
            } else {
                Err(ExternalCallError::TerminalLogicalTMismatch {
                    call_id: terminal.call_id.clone(),
                })
            }
        }
        _ => Err(ExternalCallError::PayloadKindMismatch {
            logical_t: record.envelope.logical_t,
        }),
    }
}
