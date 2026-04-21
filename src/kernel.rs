// Tier 1: Pure topology (DAG) + zero-profit treasury + settlement
// Constitutional basis: Law 1 (zero domain knowledge)
// V3L-45: no domain strings. V3L-23: no hardcoded params.
//
// CRITICAL: This module must NEVER contain domain-specific terms.
// R-001 enforced by judge.sh — any edit is scanned.

use crate::ledger::{Node, NodeId, Tape, TapeError};
use crate::prediction_market::{BinaryMarket, MarketError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ── Core types ──────────────────────────────────────────────────

/// The pure topology manager.
/// It knows about nodes, edges (citations), and markets.
/// It does NOT know what the nodes contain or what domain they belong to.
#[derive(Debug, Serialize, Deserialize)]
pub struct Kernel {
    pub tape: Tape,
    pub markets: HashMap<NodeId, BinaryMarket>,
    /// Phase 3A (Hayek): bounty market opened at run start, seeded with
    /// pre-committed LP from the same ghost-liquidity pool as per-node markets.
    /// Liquid from tx 0 → gives agents a price signal BEFORE any behaviour.
    /// Resolves YES if golden path exists; pool distributed to GP-node authors.
    #[serde(default)]
    pub bounty_market: Option<BinaryMarket>,
    /// Seed LP committed to the bounty market at open time (separate from
    /// BinaryMarket's internal CPMM book). Used for payout distribution.
    #[serde(default)]
    pub bounty_lp_seed: f64,
}

/// Result of an append operation.
#[derive(Debug)]
pub struct AppendResult {
    pub node_id: NodeId,
}

/// Result of a resolution operation.
#[derive(Debug)]
pub struct ResolutionResult {
    pub golden_path: Vec<NodeId>,
    pub markets_resolved: usize,
}

// ── Implementation ──────────────────────────────────────────────

impl Kernel {
    pub fn new() -> Self {
        Kernel {
            tape: Tape::new(),
            markets: HashMap::new(),
            bounty_market: None,
            bounty_lp_seed: 0.0,
        }
    }

    /// Phase 3A (Hayek): open a run-level bounty market seeded with `lp_coins`.
    /// Agents see its YES price from tx 0; price pre-exists behaviour,
    /// breaking the Phase 2.5 bootstrap deadlock where no signal existed until
    /// some agent had already acted.
    pub fn open_bounty_market(&mut self, lp_coins: f64) -> Result<(), KernelError> {
        if self.bounty_market.is_some() {
            return Err(KernelError::MarketExists("__bounty__".to_string()));
        }
        let market = BinaryMarket::create("__bounty__".to_string(), lp_coins)
            .map_err(KernelError::Market)?;
        self.bounty_market = Some(market);
        self.bounty_lp_seed = lp_coins;
        Ok(())
    }

    pub fn bounty_yes_price(&self) -> Option<f64> {
        self.bounty_market.as_ref().map(|m| m.yes_price())
    }

    /// Resolve the bounty market. `gp_authors` lists the author of each node
    /// on the golden path (duplicates allowed — occurrences proxy contribution
    /// count). Empty list → YES loses, seed returned to ghost pool, no payout.
    /// Non-empty → YES wins, LP distributed equally across entries (so an
    /// author with 2 GP nodes gets twice the share of one with 1).
    pub fn resolve_bounty(&mut self, gp_authors: &[String]) -> HashMap<String, f64> {
        let mut payouts: HashMap<String, f64> = HashMap::new();
        let market = match self.bounty_market.as_mut() {
            Some(m) => m,
            None => return payouts,
        };
        if market.resolved.is_some() {
            return payouts;
        }
        let yes_wins = !gp_authors.is_empty();
        let _ = market.resolve(yes_wins);
        if !yes_wins {
            return payouts;
        }
        let lp = self.bounty_lp_seed;
        let n = gp_authors.len() as f64;
        for a in gp_authors {
            *payouts.entry(a.clone()).or_insert(0.0) += lp / n;
        }
        payouts
    }

    /// Append a node to the tape.
    /// Only checks structural validity (topology).
    /// Content validation is NOT this module's job (engine separation, C-003).
    pub fn append(&mut self, node: Node) -> Result<AppendResult, KernelError> {
        let node_id = node.id.clone();
        self.tape.append(node).map_err(KernelError::Tape)?;
        Ok(AppendResult { node_id })
    }

    /// Create a prediction market for a node.
    pub fn create_market(&mut self, node_id: &str, lp_coins: f64) -> Result<(), KernelError> {
        if !self.tape.nodes().contains_key(node_id) {
            return Err(KernelError::NodeNotFound(node_id.to_string()));
        }
        if self.markets.contains_key(node_id) {
            return Err(KernelError::MarketExists(node_id.to_string()));
        }
        let market = BinaryMarket::create(node_id.to_string(), lp_coins)
            .map_err(KernelError::Market)?;
        self.markets.insert(node_id.to_string(), market);
        Ok(())
    }

    /// Buy YES shares on a node's market.
    pub fn buy_yes(&mut self, node_id: &str, coins: f64) -> Result<f64, KernelError> {
        let market = self.markets.get_mut(node_id)
            .ok_or_else(|| KernelError::MarketNotFound(node_id.to_string()))?;
        let outcome = market.buy_yes(coins).map_err(KernelError::Market)?;
        Ok(outcome.shares_received)
    }

    /// Buy NO shares on a node's market.
    pub fn buy_no(&mut self, node_id: &str, coins: f64) -> Result<f64, KernelError> {
        let market = self.markets.get_mut(node_id)
            .ok_or_else(|| KernelError::MarketNotFound(node_id.to_string()))?;
        let outcome = market.buy_no(coins).map_err(KernelError::Market)?;
        Ok(outcome.shares_received)
    }

    /// Trace ancestors from a terminal node back to root(s).
    /// Pure topology — path validity is determined externally.
    pub fn trace_golden_path(&self, terminal_id: &str) -> Result<Vec<NodeId>, KernelError> {
        if !self.tape.nodes().contains_key(terminal_id) {
            return Err(KernelError::NodeNotFound(terminal_id.to_string()));
        }
        Ok(self.tape.trace_ancestors(terminal_id))
    }

    /// Resolve all markets after external settlement.
    /// `golden_path_ids`: nodes on the verified path (YES wins).
    /// All other markets resolve NO.
    pub fn resolve_all(
        &mut self,
        golden_path_ids: &[NodeId],
    ) -> Result<ResolutionResult, KernelError> {
        let gp_set: std::collections::HashSet<&str> =
            golden_path_ids.iter().map(|s| s.as_str()).collect();

        let mut resolved_count = 0;

        for (node_id, market) in self.markets.iter_mut() {
            if market.resolved.is_some() {
                continue;
            }
            let yes_wins = gp_set.contains(node_id.as_str());
            market.resolve(yes_wins).map_err(KernelError::Market)?;
            resolved_count += 1;
        }

        Ok(ResolutionResult {
            golden_path: golden_path_ids.to_vec(),
            markets_resolved: resolved_count,
        })
    }

    /// Get the current YES price for a node's market.
    pub fn yes_price(&self, node_id: &str) -> Option<f64> {
        self.markets.get(node_id).map(|m| m.yes_price())
    }

    /// Get top N nodes by YES price (highest first). Unresolved markets only.
    pub fn market_ticker(&self, top_n: usize) -> Vec<(NodeId, f64)> {
        let mut prices: Vec<(NodeId, f64)> = self.markets.iter()
            .filter(|(_, m)| m.resolved.is_none())
            .map(|(id, m)| (id.clone(), m.yes_price()))
            .collect();
        prices.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        prices.truncate(top_n);
        prices
    }
}

impl Default for Kernel {
    fn default() -> Self {
        Self::new()
    }
}

// ── Errors ──────────────────────────────────────────────────────

#[derive(Debug)]
pub enum KernelError {
    Tape(TapeError),
    Market(MarketError),
    NodeNotFound(String),
    MarketNotFound(String),
    MarketExists(String),
}

impl std::fmt::Display for KernelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KernelError::Tape(e) => write!(f, "Tape error: {}", e),
            KernelError::Market(e) => write!(f, "Market error: {}", e),
            KernelError::NodeNotFound(id) => write!(f, "Node not found: {}", id),
            KernelError::MarketNotFound(id) => write!(f, "Market not found for node: {}", id),
            KernelError::MarketExists(id) => write!(f, "Market already exists for node: {}", id),
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
        assert!(k.append(make_node("n1", "A0", "step 1", vec!["ghost"])).is_err());
    }

    #[test]
    fn test_market_lifecycle() {
        let mut k = Kernel::new();
        k.append(make_node("n1", "A0", "step 1", vec![])).unwrap();
        k.create_market("n1", 2000.0).unwrap();

        let shares = k.buy_yes("n1", 100.0).unwrap();
        assert!(shares > 0.0);

        let price = k.yes_price("n1").unwrap();
        assert!(price > 0.5);
    }

    #[test]
    fn test_no_market_for_nonexistent_node() {
        let mut k = Kernel::new();
        assert!(k.create_market("ghost", 2000.0).is_err());
    }

    #[test]
    fn test_no_duplicate_market() {
        let mut k = Kernel::new();
        k.append(make_node("n1", "A0", "step", vec![])).unwrap();
        k.create_market("n1", 2000.0).unwrap();
        assert!(k.create_market("n1", 2000.0).is_err());
    }

    #[test]
    fn test_golden_path_trace() {
        let mut k = Kernel::new();
        k.append(make_node("root", "A0", "root", vec![])).unwrap();
        k.append(make_node("mid", "A1", "mid", vec!["root"])).unwrap();
        k.append(make_node("leaf", "A0", "leaf", vec!["mid"])).unwrap();

        let path = k.trace_golden_path("leaf").unwrap();
        assert_eq!(path, vec!["root", "mid", "leaf"]);
    }

    #[test]
    fn test_resolve_all_markets() {
        let mut k = Kernel::new();
        k.append(make_node("n1", "A0", "good", vec![])).unwrap();
        k.append(make_node("n2", "A1", "bad", vec![])).unwrap();
        k.create_market("n1", 2000.0).unwrap();
        k.create_market("n2", 2000.0).unwrap();

        let result = k.resolve_all(&["n1".to_string()]).unwrap();
        assert_eq!(result.markets_resolved, 2);
        assert_eq!(k.markets["n1"].resolved, Some(true));
        assert_eq!(k.markets["n2"].resolved, Some(false));
    }

    #[test]
    fn test_market_ticker() {
        let mut k = Kernel::new();
        k.append(make_node("n1", "A0", "a", vec![])).unwrap();
        k.append(make_node("n2", "A1", "b", vec![])).unwrap();
        k.create_market("n1", 2000.0).unwrap();
        k.create_market("n2", 2000.0).unwrap();
        k.buy_yes("n1", 100.0).unwrap();

        let ticker = k.market_ticker(10);
        assert_eq!(ticker.len(), 2);
        assert_eq!(ticker[0].0, "n1");
    }
}
