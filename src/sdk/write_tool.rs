//! WriteTool — the `wtool` node of Art. IV mermaid.
//!
//! Constitutional basis:
//! - Art. IV   `wtool(output | tape_t, HEAD_t, tools_other)` — the only
//!   legal path to mutate tape_t / HEAD_t. Gated by ∏p = 1.
//! - C-043     blessed write on OMEGA (requires OracleReceipt capability).
//! - C-067     Ed25519 R1-α capability token (unforgeable even by code
//!             holding `&mut Bus`).
//! - Art. V.1  separation of powers — WriteTool is the "court/treasury"
//!             that only acts when ∏p approves.
//!
//! Phase Z (2026-04-22): introduced as a trait so Paper 2+ can add
//! alternative write policies (e.g., shadow-writes for audit, delayed
//! settlement) without forking bus.append_oracle_accepted.

use crate::bus::BusResult;
use crate::ledger::NodeId;
use crate::sdk::oracle_receipt::OracleReceipt;

/// Policy for committing an accepted output to tape + HEAD + markets.
/// Callers MUST have already established ∏p = 1 (via
/// `TuringBus::evaluate_predicates` returning a non-Reject verdict)
/// before invoking `write`. The WriteTool does not re-verify predicates;
/// it handles capability checks (OracleReceipt) and durable state update.
pub trait WriteTool: Send + Sync {
    /// Commit a bless-tagged payload to tape. Capability (OracleReceipt)
    /// is required for Art. IV blessed-write path; `None` falls back to
    /// the unblessed `bus.append` path (Law 1: topology is free).
    fn write(
        &self,
        bus: &mut crate::bus::TuringBus,
        author: &str,
        payload: &str,
        parent: Option<&str>,
        receipt: Option<&OracleReceipt>,
    ) -> Result<BusResult, String>;
}

/// Default policy: delegates to existing bus methods.
/// - If `receipt` is Some → `append_oracle_accepted` (C-043 + C-067).
/// - If `receipt` is None → `bus.append` (Law 1 free topology).
pub struct DefaultWriteTool;

impl WriteTool for DefaultWriteTool {
    fn write(
        &self,
        bus: &mut crate::bus::TuringBus,
        author: &str,
        payload: &str,
        parent: Option<&str>,
        receipt: Option<&OracleReceipt>,
    ) -> Result<BusResult, String> {
        if let Some(r) = receipt {
            bus.append_oracle_accepted(author, payload, parent, r)
        } else {
            bus.append(author, payload, parent)
        }
    }
}

impl DefaultWriteTool {
    /// Helper: produce a concrete NodeId from a BusResult::Appended.
    pub fn extract_node_id(result: &BusResult) -> Option<NodeId> {
        match result {
            BusResult::Appended { node_id } => Some(node_id.clone()),
            _ => None,
        }
    }
}
