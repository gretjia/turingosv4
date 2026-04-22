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

    /// Art. IV explicit form: wtool(output | tape_t, HEAD_t, tools_other).
    ///
    /// The `tools_other` slice names the ancillary TuringTools (Wallet,
    /// Librarian, Search, …) whose on_pre_append hooks must run before
    /// the commit. Its semantic role is **contract clarity**: the Art. IV
    /// mermaid lists tools_other as an explicit input to wtool, but in
    /// the default implementation bus.append_internal already iterates
    /// over `bus.tools` so the parameter does not change behavior.
    ///
    /// API caveat: this method is **assertive, not selective** — it
    /// verifies every requested tool is currently mounted and returns
    /// `Err("…tool '<name>' not mounted")` if any are missing. It does
    /// NOT filter the bus's actual hook dispatch to only the named
    /// subset; selective hook dispatch would require a bus-level change
    /// and is out of scope for Phase Z. Callers who need selective
    /// dispatch should open a follow-up against bus.append_internal.
    ///
    /// Default impl delegates to `write` after the presence check, so
    /// existing WriteTool implementations get the contract for free.
    fn write_with_tools(
        &self,
        bus: &mut crate::bus::TuringBus,
        author: &str,
        payload: &str,
        parent: Option<&str>,
        receipt: Option<&OracleReceipt>,
        tools_other: &[&str],
    ) -> Result<BusResult, String> {
        for name in tools_other {
            if !bus.tools.iter().any(|t| t.manifest() == *name) {
                return Err(format!(
                    "write_with_tools: tool '{}' not mounted",
                    name
                ));
            }
        }
        self.write(bus, author, payload, parent, receipt)
    }
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
