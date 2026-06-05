//! A05 generic ChainTape projection contract.
//!
//! Projection code consumes `TapeEventEnvelope` slices. It does not read
//! dashboards, stdout, reports, benchmark manifests, or TDMA-only tape state.

use serde::{Deserialize, Serialize};

use crate::bottom_white::ledger::transition_ledger::{LedgerWriter, LedgerWriterError};
use crate::runtime::tape_event::{TapeEventEnvelope, TapeEventError, TapeEventKind, TapeEventRef};

/// TRACE_MATRIX Art.0.2 + FC1-N13: generic projection trait over ChainTape-derived event envelopes.
pub trait Projection {
    type Output;

    fn projection_id() -> &'static str;
    fn derive_from_tape(events: &[TapeEventEnvelope]) -> Result<Self::Output, ProjectionError>;
}

/// TRACE_MATRIX Art.0.2: optional metadata wrapper for materialized projection values.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectionOutput<T> {
    pub projection_id: String,
    pub source_head_oid: String,
    pub value: T,
}

/// TRACE_MATRIX Art.0.2 + FC1-N13: typed projection failure domain; derived-view inputs fail closed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProjectionError {
    EmptyTape,
    MissingGitHead,
    ReadLedger {
        logical_t: u64,
        reason: String,
    },
    TapeEvent(TapeEventError),
    UnexpectedEventKind {
        projection_id: &'static str,
        logical_t: u64,
        kind: TapeEventKind,
    },
}

impl From<TapeEventError> for ProjectionError {
    fn from(value: TapeEventError) -> Self {
        Self::TapeEvent(value)
    }
}

impl From<LedgerWriterError> for ProjectionError {
    fn from(value: LedgerWriterError) -> Self {
        Self::ReadLedger {
            logical_t: 0,
            reason: value.to_string(),
        }
    }
}

impl std::fmt::Display for ProjectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyTape => write!(f, "empty ChainTape event stream"),
            Self::MissingGitHead => write!(f, "git-backed ChainTape head OID missing"),
            Self::ReadLedger { logical_t, reason } => {
                write!(f, "read L4 ledger at logical_t={logical_t}: {reason}")
            }
            Self::TapeEvent(e) => write!(f, "invalid tape event: {e}"),
            Self::UnexpectedEventKind {
                projection_id,
                logical_t,
                kind,
            } => write!(
                f,
                "projection {projection_id} cannot consume {kind:?} at logical_t={logical_t}"
            ),
        }
    }
}

impl std::error::Error for ProjectionError {}

/// TRACE_MATRIX Art.0.2 + FC1-N13: derive generic accepted-transition events from an L4 LedgerWriter, never from manifests or stdout.
pub fn derive_l4_events_from_writer<W: LedgerWriter>(
    writer: &W,
) -> Result<Vec<TapeEventEnvelope>, ProjectionError> {
    let head_oid = writer
        .head_commit_oid_hex()
        .ok_or(ProjectionError::MissingGitHead)?;
    let mut events = Vec::with_capacity(writer.len() as usize);
    for logical_t in 1..=writer.len() {
        let entry = writer
            .read_at(logical_t)
            .map_err(|e| ProjectionError::ReadLedger {
                logical_t,
                reason: e.to_string(),
            })?;
        let event = TapeEventEnvelope {
            logical_t: entry.logical_t,
            tape_ref: TapeEventRef::L4Accepted {
                head_oid: head_oid.clone(),
            },
            kind: TapeEventKind::AcceptedTransition,
            payload_cid: Some(entry.tx_payload_cid),
            source_tx_kind: Some(entry.tx_kind),
        };
        event.validate()?;
        events.push(event);
    }
    Ok(events)
}
