// Tier 1: Pure topology (DAG)
// Constitutional basis: Law 1 (zero domain knowledge)
// V3L-45: no domain strings. V3L-23: no hardcoded params.
//
// CRITICAL: This module must NEVER contain domain-specific terms.
// R-001 enforced by judge.sh — any edit is scanned.
//
// TB-14 Atom 6 (2026-05-03 closing OBS_TB_12_LEGACY_CPMM_QUARANTINE):
// All decimal-float CPMM scaffolding (`markets`, `bounty_market`,
// `bounty_lp_seed`, `create_market`, `buy_yes`, `buy_no`, `yes_price`,
// `market_ticker`, `market_ticker_full`, `open_bounty_market`,
// `bounty_yes_price`, `resolve_bounty`, `resolve_all`) was excised
// together with `src/prediction_market.rs`. Pricing now lives entirely
// in the derived view `state::compute_price_index`; YES/NO claims live
// in TB-13 `ConditionalShareBalances`. The kernel is pure topology
// (V3L-45 docstring contract restored).

use crate::ledger::{Node, NodeId, Tape, TapeError};
use serde::{Deserialize, Serialize};

// ── Core types ──────────────────────────────────────────────────

/// The pure topology manager.
/// It knows about nodes and edges (citations).
/// It does NOT know what the nodes contain or what domain they belong to.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Kernel {
    pub tape: Tape,
}

/// Result of an append operation.
#[derive(Debug)]
pub struct AppendResult {
    pub node_id: NodeId,
}

// ── Implementation ──────────────────────────────────────────────

impl Kernel {
    pub fn new() -> Self {
        Kernel { tape: Tape::new() }
    }

    /// Append a node to the tape.
    /// Only checks structural validity (topology).
    /// Content validation is NOT this module's job (engine separation, C-003).
    pub fn append(&mut self, node: Node) -> Result<AppendResult, KernelError> {
        let node_id = node.id.clone();
        self.tape.append(node).map_err(KernelError::Tape)?;
        Ok(AppendResult { node_id })
    }

    /// Trace ancestors from a terminal node back to root(s).
    /// Pure topology — path validity is determined externally.
    pub fn trace_golden_path(&self, terminal_id: &str) -> Result<Vec<NodeId>, KernelError> {
        if !self.tape.nodes().contains_key(terminal_id) {
            return Err(KernelError::NodeNotFound(terminal_id.to_string()));
        }
        Ok(self.tape.trace_ancestors(terminal_id))
    }
}

// ── Errors ──────────────────────────────────────────────────────

#[derive(Debug)]
pub enum KernelError {
    Tape(TapeError),
    NodeNotFound(String),
}

impl std::fmt::Display for KernelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KernelError::Tape(e) => write!(f, "Tape error: {}", e),
            KernelError::NodeNotFound(id) => write!(f, "Node not found: {}", id),
        }
    }
}

impl std::error::Error for KernelError {}

// ── Tests ───────────────────────────────────────────────────────
// NOTE: Domain-purity test lives in tests/kernel_purity.rs (outside this file)
// because R-001 forbids domain terms even as test strings in kernel.rs.

#[cfg(test)]
mod tests {
    use super::*;

    fn make_node(id: &str, author: &str, payload: &str, citations: Vec<&str>) -> Node {
        Node {
            id: id.to_string(),
            author: author.to_string(),
            payload: payload.to_string(),
            citations: citations.into_iter().map(|s| s.to_string()).collect(),
            created_at: 0,
            completion_tokens: 0,
        }
    }

    #[test]
    fn test_append_and_retrieve() {
        let mut k = Kernel::new();
        k.append(make_node("n1", "A0", "step 1", vec![])).unwrap();
        assert!(k.tape.get("n1").is_some());
    }

    #[test]
    fn test_reject_duplicate() {
        let mut k = Kernel::new();
        k.append(make_node("n1", "A0", "step 1", vec![])).unwrap();
        assert!(k.append(make_node("n1", "A1", "step 2", vec![])).is_err());
    }

    #[test]
    fn test_reject_dangling_citation() {
        let mut k = Kernel::new();
        assert!(k
            .append(make_node("n1", "A0", "step 1", vec!["ghost"]))
            .is_err());
    }

    #[test]
    fn test_golden_path_trace() {
        let mut k = Kernel::new();
        k.append(make_node("root", "A0", "root", vec![])).unwrap();
        k.append(make_node("mid", "A1", "mid", vec!["root"]))
            .unwrap();
        k.append(make_node("leaf", "A0", "leaf", vec!["mid"]))
            .unwrap();

        let path = k.trace_golden_path("leaf").unwrap();
        assert_eq!(path, vec!["root", "mid", "leaf"]);
    }

    #[test]
    fn test_trace_golden_path_unknown_node() {
        let k = Kernel::new();
        assert!(matches!(
            k.trace_golden_path("ghost"),
            Err(KernelError::NodeNotFound(_))
        ));
    }
}
