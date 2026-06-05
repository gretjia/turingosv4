//! A05 TapeEvent envelope.
//!
//! Generic event shape shared by later projection atoms. This module does not
//! define economy, scheduler, provider, or market policy.

use serde::{Deserialize, Serialize};

use crate::bottom_white::cas::Cid;
use crate::bottom_white::ledger::transition_ledger::TxKind;

/// TRACE_MATRIX Art.0.2 + FC1-N13 + FC1-N14 + FC1-N15: canonical tape ref carried by a generic TapeEvent envelope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TapeEventRef {
    L4Accepted { head_oid: String },
    L4Rejected { head_oid: String },
    Pending { head_oid: String },
    Terminal { head_oid: String },
    DerivedView { source: String },
}

impl TapeEventRef {
    /// TRACE_MATRIX Art.0.2 + FC1-N13: expose the canonical git OID hex for tape-backed refs.
    pub fn head_oid_hex(&self) -> &str {
        match self {
            Self::L4Accepted { head_oid }
            | Self::L4Rejected { head_oid }
            | Self::Pending { head_oid }
            | Self::Terminal { head_oid } => head_oid,
            Self::DerivedView { source } => source,
        }
    }

    fn validate(&self) -> Result<(), TapeEventError> {
        match self {
            Self::L4Accepted { head_oid }
            | Self::L4Rejected { head_oid }
            | Self::Pending { head_oid }
            | Self::Terminal { head_oid } => validate_git_oid(head_oid),
            Self::DerivedView { source } => Err(TapeEventError::NonCanonicalTapeRef {
                source: source.clone(),
            }),
        }
    }
}

/// TRACE_MATRIX Art.0.2 + FC1-N13 + FC1-N14 + FC1-N15: generic tape event class, not policy-specific mechanism state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TapeEventKind {
    AcceptedTransition,
    RejectedTransition,
    PendingIntent,
    TerminalExternalCall,
}

impl TapeEventKind {
    /// TRACE_MATRIX Art.0.2: enumerate generic event kinds for structural tests without policy-specific variants.
    pub const fn all() -> &'static [Self] {
        &[
            Self::AcceptedTransition,
            Self::RejectedTransition,
            Self::PendingIntent,
            Self::TerminalExternalCall,
        ]
    }
}

/// TRACE_MATRIX Art.0.2 + FC1-N13 + FC1-N14 + FC1-N15: generic envelope projected from ChainTape/L4-family refs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TapeEventEnvelope {
    pub logical_t: u64,
    pub tape_ref: TapeEventRef,
    pub kind: TapeEventKind,
    pub payload_cid: Option<Cid>,
    pub source_tx_kind: Option<TxKind>,
}

impl TapeEventEnvelope {
    /// TRACE_MATRIX Art.0.2 + FC1-N13: fail-closed validation for canonical tape-derived event envelopes.
    pub fn validate(&self) -> Result<(), TapeEventError> {
        if self.logical_t == 0 {
            return Err(TapeEventError::InvalidLogicalT { got: 0 });
        }
        self.tape_ref.validate()?;
        if self.kind == TapeEventKind::AcceptedTransition
            && (self.payload_cid.is_none() || self.source_tx_kind.is_none())
        {
            return Err(TapeEventError::MissingAcceptedPayload {
                logical_t: self.logical_t,
            });
        }
        Ok(())
    }
}

/// TRACE_MATRIX Art.0.2 + FC1-N13: typed validation errors for bad or non-canonical TapeEvent envelopes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TapeEventError {
    InvalidLogicalT { got: u64 },
    InvalidGitOid { value: String },
    MissingAcceptedPayload { logical_t: u64 },
    NonCanonicalTapeRef { source: String },
}

impl std::fmt::Display for TapeEventError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidLogicalT { got } => write!(f, "invalid logical_t: {got}"),
            Self::InvalidGitOid { value } => write!(f, "invalid git oid: {value}"),
            Self::MissingAcceptedPayload { logical_t } => {
                write!(
                    f,
                    "accepted event at logical_t={logical_t} missing payload or tx kind"
                )
            }
            Self::NonCanonicalTapeRef { source } => {
                write!(f, "non-canonical tape ref from derived view: {source}")
            }
        }
    }
}

impl std::error::Error for TapeEventError {}

fn validate_git_oid(value: &str) -> Result<(), TapeEventError> {
    if value.len() == 40
        && value
            .bytes()
            .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
    {
        Ok(())
    } else {
        Err(TapeEventError::InvalidGitOid {
            value: value.to_string(),
        })
    }
}
