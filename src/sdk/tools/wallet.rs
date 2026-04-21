// Tier 2: WalletTool — balance + YES/NO/LP portfolios
// Constitutional basis: Law 2 (only investment costs money, CTF conservation)
// V3L-22: Falsifier cannot buy YES (Goodhart defense at tool level)
// V3L-39: constraints encoded to tool, not prompt

use crate::sdk::tool::{ToolSignal, TuringTool};
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::collections::HashMap;

// ── Core types ──────────────────────────────────────────────────

/// Per-node portfolio: (YES shares, NO shares, LP shares)
pub type Portfolio = HashMap<String, (f64, f64, f64)>;

/// Wallet tool manages all agent balances and portfolios.
///
/// Law 2 invariants:
/// - GENESIS (on_init) is the ONLY legal coin injection
/// - No abolished v3 injection methods (C-001, C-002)
/// - 1 Coin = 1 YES + 1 NO (CTF conservation)
/// - Append is FREE (Law 1) — only investment costs money
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletTool {
    pub balances: HashMap<String, f64>,
    pub portfolios: HashMap<String, Portfolio>,
    pub genesis_done: bool,
    genesis_coins: f64,
}

impl WalletTool {
    pub fn new(genesis_coins: f64) -> Self {
        WalletTool {
            balances: HashMap::new(),
            portfolios: HashMap::new(),
            genesis_done: false,
            genesis_coins,
        }
    }

    pub fn balance(&self, agent: &str) -> f64 {
        self.balances.get(agent).copied().unwrap_or(0.0)
    }

    pub fn deduct(&mut self, agent: &str, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            return Err(format!("Deduct amount must be positive, got {}", amount));
        }
        let bal = self.balances.get_mut(agent)
            .ok_or_else(|| format!("Agent {} not found", agent))?;
        if *bal < amount {
            return Err(format!("Insufficient balance: {} < {}", bal, amount));
        }
        *bal -= amount;
        Ok(())
    }

    /// Credit coins back to an agent. Only for refunds/settlement — NOT for minting.
    /// This is pub(crate) to prevent external callers from injecting coins.
    pub(crate) fn credit(&mut self, agent: &str, amount: f64) {
        if amount <= 0.0 { return; }
        *self.balances.entry(agent.to_string()).or_insert(0.0) += amount;
    }

    /// Record shares in agent's portfolio.
    pub fn record_shares(&mut self, agent: &str, node_id: &str,
                         yes_delta: f64, no_delta: f64, lp_delta: f64) {
        let portfolio = self.portfolios
            .entry(agent.to_string())
            .or_default();
        let entry = portfolio
            .entry(node_id.to_string())
            .or_insert((0.0, 0.0, 0.0));
        entry.0 += yes_delta;
        entry.1 += no_delta;
        entry.2 += lp_delta;
    }

    /// Phase 4 (C-041 candidate): persist wallet + portfolio state to disk for
    /// cross-problem continuity. Law 2 preserved: `genesis_done` is serialised
    /// too, so a load round-trip carries the original genesis flag and prevents
    /// any second mint (`on_init` skips when `genesis_done == true`).
    pub fn save_to_disk(&self, path: &std::path::Path) -> std::io::Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        std::fs::write(path, json)
    }

    /// Load a previously-persisted wallet. Returns None if missing/corrupt —
    /// caller falls back to fresh genesis.
    pub fn load_from_disk(path: &std::path::Path) -> Option<Self> {
        let raw = std::fs::read_to_string(path).ok()?;
        serde_json::from_str(&raw).ok()
    }

    /// Phase 4 helper: initialise any agents that aren't yet in the wallet
    /// WITHOUT re-minting existing ones. Only mints for never-seen agents IF
    /// genesis has not yet run (first-ever boot). Post-genesis, new agents
    /// enter at zero balance (C-001 / C-038: no post-genesis mint).
    pub fn ensure_agents(&mut self, agent_ids: &[String]) {
        for agent in agent_ids {
            if !self.balances.contains_key(agent) {
                let bal = if self.genesis_done { 0.0 } else { self.genesis_coins };
                self.balances.insert(agent.clone(), bal);
                self.portfolios.insert(agent.clone(), HashMap::new());
            }
        }
    }
}

impl TuringTool for WalletTool {
    fn manifest(&self) -> &str {
        "wallet"
    }

    /// GENESIS: the ONLY legal coin injection point.
    /// Law 2: No post-genesis injection. Period.
    fn on_init(&mut self, agent_ids: &[String]) {
        if self.genesis_done {
            // V3L-41/42/43: absolutely no second injection
            return;
        }
        for agent in agent_ids {
            self.balances.insert(agent.clone(), self.genesis_coins);
            self.portfolios.insert(agent.clone(), HashMap::new());
        }
        self.genesis_done = true;
    }

    /// Pre-append validation.
    /// Law 1: Append is FREE — if no wallet tag, return Pass.
    /// Law 2: Only investment costs money.
    fn on_pre_append(&mut self, author: &str, _payload: &str) -> ToolSignal {
        // Law 1: topology is free. Any agent can append without paying.
        // Investment routing is handled by the bus, not here.
        // We just verify the agent exists.
        if !self.balances.contains_key(author) {
            return ToolSignal::Veto(format!("Unknown agent: {}", author));
        }
        ToolSignal::Pass
    }

    fn on_halt(&mut self, _golden_path: &[String]) {
        // Settlement handled by bus calling kernel.resolve_all
    }

    fn query_state(&self, key: &str) -> Option<String> {
        if let Some(agent) = key.strip_prefix("balance_") {
            Some(format!("{:.2}", self.balance(agent)))
        } else {
            None
        }
    }

    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}

// ── Tests ───────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_genesis_allocation() {
        let mut wallet = WalletTool::new(10000.0);
        wallet.on_init(&["A0".into(), "A1".into(), "A2".into()]);
        assert_eq!(wallet.balance("A0"), 10000.0);
        assert_eq!(wallet.balance("A1"), 10000.0);
        assert_eq!(wallet.balance("A2"), 10000.0);
    }

    #[test]
    fn test_no_double_genesis() {
        // V3L-41/42/43: no post-genesis injection
        let mut wallet = WalletTool::new(10000.0);
        wallet.on_init(&["A0".into()]);
        wallet.deduct("A0", 5000.0).unwrap();

        // Second init should be ignored
        wallet.on_init(&["A0".into()]);
        assert_eq!(wallet.balance("A0"), 5000.0, "Balance not reset by second init");
    }

    #[test]
    fn test_deduct_and_credit() {
        let mut wallet = WalletTool::new(10000.0);
        wallet.on_init(&["A0".into()]);

        wallet.deduct("A0", 3000.0).unwrap();
        assert_eq!(wallet.balance("A0"), 7000.0);

        wallet.credit("A0", 1000.0);
        assert_eq!(wallet.balance("A0"), 8000.0);
    }

    #[test]
    fn test_insufficient_balance_rejected() {
        let mut wallet = WalletTool::new(100.0);
        wallet.on_init(&["A0".into()]);
        assert!(wallet.deduct("A0", 200.0).is_err());
    }

    #[test]
    fn test_append_is_free() {
        // Law 1: append costs nothing
        let mut wallet = WalletTool::new(10000.0);
        wallet.on_init(&["A0".into()]);
        let signal = wallet.on_pre_append("A0", "any payload");
        assert!(matches!(signal, ToolSignal::Pass));
    }

    #[test]
    fn test_unknown_agent_vetoed() {
        let mut wallet = WalletTool::new(10000.0);
        wallet.on_init(&["A0".into()]);
        let signal = wallet.on_pre_append("unknown", "payload");
        assert!(matches!(signal, ToolSignal::Veto(_)));
    }

    #[test]
    fn test_portfolio_tracking() {
        let mut wallet = WalletTool::new(10000.0);
        wallet.on_init(&["A0".into()]);
        wallet.record_shares("A0", "n1", 150.0, 0.0, 0.0);
        wallet.record_shares("A0", "n1", 50.0, 0.0, 0.0);

        let portfolio = wallet.portfolios.get("A0").unwrap();
        let (yes, no, lp) = portfolio.get("n1").unwrap();
        assert_eq!(*yes, 200.0);
        assert_eq!(*no, 0.0);
        assert_eq!(*lp, 0.0);
    }

    #[test]
    fn test_query_balance() {
        let mut wallet = WalletTool::new(5000.0);
        wallet.on_init(&["A0".into()]);
        let result = wallet.query_state("balance_A0");
        assert_eq!(result, Some("5000.00".to_string()));
    }

    #[test]
    fn test_query_unknown_key() {
        let wallet = WalletTool::new(5000.0);
        assert!(wallet.query_state("unknown_key").is_none());
    }

    #[test]
    fn test_negative_deduct_rejected() {
        // Codex finding: negative deduct = hidden credit. Must reject.
        let mut wallet = WalletTool::new(10000.0);
        wallet.on_init(&["A0".into()]);
        assert!(wallet.deduct("A0", -100.0).is_err(), "Negative deduct must be rejected");
        assert_eq!(wallet.balance("A0"), 10000.0, "Balance must not change");
    }

    #[test]
    fn test_zero_deduct_rejected() {
        let mut wallet = WalletTool::new(10000.0);
        wallet.on_init(&["A0".into()]);
        assert!(wallet.deduct("A0", 0.0).is_err(), "Zero deduct must be rejected");
    }
}
