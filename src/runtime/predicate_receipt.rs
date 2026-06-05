//! A08 PredicateReceipt derived receipt.
//!
//! This module is deliberately non-authoritative: it records a replayable
//! receipt over a canonical `TapeEventEnvelope` plus CAS CIDs. It does not
//! change predicate admission, typed transaction wire shape, sequencer logic,
//! or CAS `ObjectType`.

use serde::{Deserialize, Serialize};

use crate::bottom_white::cas::schema::{Cid, ObjectType};
use crate::bottom_white::cas::store::{CasError, CasStore};
use crate::bottom_white::ledger::transition_ledger::{canonical_decode, canonical_encode};
use crate::runtime::tape_event::{TapeEventEnvelope, TapeEventError};
use crate::state::q_state::{Hash, TxId};
use crate::state::typed_tx::PredicateId;

/// TRACE_MATRIX FC1-N11 + FC1-N12 + FC1-N14 + FC1-N15: A08 PredicateReceipt Generic-CAS schema id.
pub const PREDICATE_RECEIPT_SCHEMA_ID: &str = "turingosv4.predicate_receipt.v1";

/// TRACE_MATRIX FC1-N11 + FC1-N12 + FC1-N14 + FC1-N15: non-authoritative predicate replay receipt derived from tape/CAS.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PredicateReceipt {
    pub predicate_id: PredicateId,
    pub subject_tx_id: TxId,
    pub tape_event_id: Option<String>,
    pub tape_head_oid: String,
    pub logical_t: u64,
    pub input_cid: Cid,
    pub verdict_cid: Cid,
    pub registry_root: Hash,
    pub result: bool,
}

/// TRACE_MATRIX FC1-N11 + FC1-N12 + FC1-N14 + FC1-N15: derive a receipt only from canonical ChainTape event envelopes.
pub fn derive_predicate_receipt(
    event: &TapeEventEnvelope,
    predicate_id: PredicateId,
    subject_tx_id: TxId,
    input_cid: Cid,
    verdict_cid: Cid,
    registry_root: Hash,
    result: bool,
) -> Result<PredicateReceipt, PredicateReceiptError> {
    event.validate()?;
    let payload_cid = event
        .payload_cid
        .ok_or(PredicateReceiptError::MissingVerdictPayload {
            logical_t: event.logical_t,
        })?;
    if payload_cid != verdict_cid {
        return Err(PredicateReceiptError::VerdictCidMismatch {
            logical_t: event.logical_t,
            event_payload_cid: payload_cid,
            verdict_cid,
        });
    }
    let tape_head_oid = event.tape_ref.head_oid_hex().to_string();
    Ok(PredicateReceipt {
        predicate_id,
        subject_tx_id,
        tape_event_id: Some(format!("{}:{}", tape_head_oid, event.logical_t)),
        tape_head_oid,
        logical_t: event.logical_t,
        input_cid,
        verdict_cid,
        registry_root,
        result,
    })
}

/// TRACE_MATRIX FC1-N11 + FC1-N12 + FC2-N34: write PredicateReceipt as Generic CAS + schema_id.
pub fn write_to_cas(
    cas: &mut CasStore,
    receipt: &PredicateReceipt,
    creator: &str,
) -> Result<Cid, PredicateReceiptError> {
    let bytes =
        canonical_encode(receipt).map_err(|e| PredicateReceiptError::Codec(e.to_string()))?;
    Ok(cas.put(
        &bytes,
        ObjectType::Generic,
        creator,
        receipt.logical_t,
        Some(PREDICATE_RECEIPT_SCHEMA_ID.to_string()),
    )?)
}

/// TRACE_MATRIX FC1-N11 + FC1-N12 + FC2-N34: read PredicateReceipt from CAS and canonical-decode.
pub fn read_from_cas(cas: &CasStore, cid: &Cid) -> Result<PredicateReceipt, PredicateReceiptError> {
    let bytes = cas.get(cid)?;
    canonical_decode(&bytes).map_err(|e| PredicateReceiptError::Codec(e.to_string()))
}

/// TRACE_MATRIX FC1-N11 + FC1-N12 + FC2-N34: PredicateReceipt failure domain.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PredicateReceiptError {
    TapeEvent(TapeEventError),
    MissingVerdictPayload {
        logical_t: u64,
    },
    VerdictCidMismatch {
        logical_t: u64,
        event_payload_cid: Cid,
        verdict_cid: Cid,
    },
    Cas(String),
    Codec(String),
}

impl From<TapeEventError> for PredicateReceiptError {
    fn from(value: TapeEventError) -> Self {
        Self::TapeEvent(value)
    }
}

impl From<CasError> for PredicateReceiptError {
    fn from(value: CasError) -> Self {
        Self::Cas(value.to_string())
    }
}

impl std::fmt::Display for PredicateReceiptError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TapeEvent(e) => write!(f, "invalid tape event: {e}"),
            Self::MissingVerdictPayload { logical_t } => {
                write!(f, "predicate receipt at logical_t={logical_t} missing verdict payload")
            }
            Self::VerdictCidMismatch {
                logical_t,
                event_payload_cid,
                verdict_cid,
            } => write!(
                f,
                "predicate receipt at logical_t={logical_t} verdict cid mismatch: event={event_payload_cid}, receipt={verdict_cid}"
            ),
            Self::Cas(e) => write!(f, "cas error: {e}"),
            Self::Codec(e) => write!(f, "codec error: {e}"),
        }
    }
}

impl std::error::Error for PredicateReceiptError {}
