// Tier 4: TSP Event Bus — SKILL lifecycle serial reactor
// Constitutional basis: Art. II (selective broadcast), Art. III (selective shielding)
// V3L-11: serial reactor for causal ordering (no concurrent pricing oscillation)
// V3L-21: one-step-per-node payload limits
// V3L-31: supervisor loop, never silent exit
// V3L-32: cascade failure protection

use crate::kernel::{Kernel, KernelError};
use crate::ledger::{EventType, Ledger, Node, NodeId, TapeError};
use crate::sdk::tool::{BetDirection, ToolSignal, TuringTool};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ── Configuration ───────────────────────────────────────────────

/// Bus configuration. V3L-23: no hardcoded values, all configurable.
pub struct BusConfig {
    pub max_payload_chars: usize,
    pub max_payload_lines: usize,
    pub system_lp_amount: f64,
    pub forbidden_patterns: Vec<String>,
}

impl Default for BusConfig {
    fn default() -> Self {
        BusConfig {
            max_payload_chars: 1600,
            max_payload_lines: 24,
            system_lp_amount: 200.0,
            forbidden_patterns: Vec::new(),
        }
    }
}

// ── Core Bus ────────────────────────────────────────────────────

/// The serial event reactor.
/// V3L-11: ALL state mutations go through this single-threaded reactor.
/// No concurrent access to kernel/markets — causal ordering guaranteed.
pub struct TuringBus {
    pub kernel: Kernel,
    pub ledger: Ledger,
    pub tools: Vec<Box<dyn TuringTool>>,
    pub config: BusConfig,
    pub clock: u64,
    pub tx_count: u64,
    pub generation: u32,
    graveyard: HashMap<String, Vec<String>>,
}

/// Scope for recent_rejections query.
/// Step-B v3 Art. II.1 fix: enables global abstract-broadcast without violating C-022.
#[derive(Debug, Clone, Copy)]
pub enum RejectionScope {
    /// Legacy: per-author graveyard (before-fix behavior).
    PerAuthor,
    /// Flattened across all authors, chronological (may leak raw content — use with caution).
    Global,
    /// Art. II.1 compliant: counted + top-k class labels. Requires callers to record class labels.
    TopKClasses(usize),
}

/// Result of a bus append operation.
#[derive(Debug)]
pub enum BusResult {
    Appended { node_id: NodeId },
    Invested { node_id: NodeId, shares: f64 },
    Vetoed { reason: String },
}

impl TuringBus {
    pub fn new(kernel: Kernel, config: BusConfig) -> Self {
        TuringBus {
            kernel,
            ledger: Ledger::new(),
            tools: Vec::new(),
            config,
            clock: 0,
            tx_count: 0,
            generation: 0,
            graveyard: HashMap::new(),
        }
    }

    /// Mount a tool into the bus. Tools execute in mount order.
    pub fn mount_tool(&mut self, tool: Box<dyn TuringTool>) {
        self.tools.push(tool);
    }

    /// Boot all tools.
    pub fn boot(&mut self) {
        for tool in &mut self.tools {
            tool.on_boot();
        }
    }

    /// Initialize all tools with agent list. Triggers GENESIS.
    pub fn init(&mut self, agent_ids: &[String]) {
        for tool in &mut self.tools {
            tool.on_init(agent_ids);
        }
        self.ledger.append(EventType::RunStart, None, None, None).ok();
    }

    /// The main append pipeline — 6 phases.
    /// V3L-11: this runs serially, never concurrently.
    pub fn append(&mut self, author: &str, payload: &str,
                  parent_id: Option<&str>) -> Result<BusResult, String> {
        // Phase 0: Forbidden pattern check
        for pattern in &self.config.forbidden_patterns {
            if payload.contains(pattern.as_str()) {
                let reason = format!("Forbidden pattern: {}", pattern);
                self.record_rejection(author, &reason);
                return Ok(BusResult::Vetoed { reason });
            }
        }

        // Phase 0b: Payload size limits (V3L-21: one step per node)
        if payload.len() > self.config.max_payload_chars {
            let reason = format!("Payload too long: {} > {} chars",
                                 payload.len(), self.config.max_payload_chars);
            self.record_rejection(author, &reason);
            return Ok(BusResult::Vetoed { reason });
        }
        let line_count = payload.lines().count();
        if line_count > self.config.max_payload_lines {
            let reason = format!("Too many lines: {} > {}",
                                 line_count, self.config.max_payload_lines);
            self.record_rejection(author, &reason);
            return Ok(BusResult::Vetoed { reason });
        }

        // Phase 1: Tool pre-append hooks
        let mut signal = ToolSignal::Pass;
        for tool in &mut self.tools {
            match tool.on_pre_append(author, payload) {
                ToolSignal::Veto(reason) => {
                    self.record_rejection(author, &reason);
                    return Ok(BusResult::Vetoed { reason });
                }
                ToolSignal::InvestOnly { target_node, amount, direction } => {
                    signal = ToolSignal::InvestOnly { target_node, amount, direction };
                    break;
                }
                ToolSignal::YieldReward { reward } => {
                    signal = ToolSignal::YieldReward { reward };
                }
                ToolSignal::Pass => {}
            }
        }

        // Phase 2: InvestOnly routing (skip append, buy shares)
        // Law 2: staking COSTS money — debit wallet before buying shares
        if let ToolSignal::InvestOnly { target_node, amount, direction } = signal {
            // Debit the agent's wallet BEFORE buying shares
            self.debit_wallet(author, amount)?;

            let shares = match direction {
                BetDirection::Long => self.kernel.buy_yes(&target_node, amount),
                BetDirection::Short => self.kernel.buy_no(&target_node, amount),
            }.map_err(|e| {
                // Refund on failure (Law 2: no silent burns)
                self.credit_wallet(author, amount);
                e.to_string()
            })?;

            self.ledger.append(EventType::Invest, Some(target_node.clone()),
                               Some(author.to_string()), None).ok();
            self.tx_count += 1;
            return Ok(BusResult::Invested { node_id: target_node, shares });
        }

        // Phase 3: Kernel append (topology validation)
        let node_id = format!("tx_{}_by_{}", self.tx_count, author);
        let citations = parent_id.map(|p| vec![p.to_string()]).unwrap_or_default();

        let node = Node {
            id: node_id.clone(),
            author: author.to_string(),
            payload: payload.to_string(),
            citations,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            completion_tokens: 0,
        };

        self.kernel.append(node).map_err(|e| e.to_string())?;

        // Phase 4: System Market Maker (Magna Carta Rule #19 exemption)
        // System MM injects liquidity from a dedicated pool, NOT from agent wallets.
        // This is the only constitutional exception to Law 2 — MM impermanent loss
        // is an expected physical cost, not minting. See C-001/C-002 for history.
        self.kernel.create_market(&node_id, self.config.system_lp_amount)
            .ok(); // Market creation failure is non-fatal

        // Phase 5: Tool post-append hooks
        for tool in &mut self.tools {
            tool.on_post_append(author, &node_id);
        }

        self.ledger.append(EventType::Append, Some(node_id.clone()),
                           Some(author.to_string()), None).ok();
        self.tx_count += 1;
        self.clock += 1;

        Ok(BusResult::Appended { node_id })
    }

    /// Halt and settle — triggered by Oracle verification.
    pub fn halt_and_settle(&mut self, golden_path: &[NodeId]) -> Result<(), String> {
        // Resolve all markets
        self.kernel.resolve_all(golden_path).map_err(|e| e.to_string())?;

        // Tool halt hooks
        let gp: Vec<String> = golden_path.to_vec();
        for tool in &mut self.tools {
            tool.on_halt(&gp);
        }

        self.ledger.append(EventType::RunEnd, None, None, None).ok();
        Ok(())
    }

    /// Debit an agent's wallet. Finds the WalletTool among mounted tools.
    fn debit_wallet(&mut self, agent: &str, amount: f64) -> Result<(), String> {
        for tool in &mut self.tools {
            if tool.manifest() == "wallet" {
                if let Some(wallet) = tool.as_any_mut().downcast_mut::<crate::sdk::tools::wallet::WalletTool>() {
                    return wallet.deduct(agent, amount);
                }
            }
        }
        Err("No wallet tool mounted".into())
    }

    /// Credit an agent's wallet (for refunds only — not new coins).
    fn credit_wallet(&mut self, agent: &str, amount: f64) {
        for tool in &mut self.tools {
            if tool.manifest() == "wallet" {
                if let Some(wallet) = tool.as_any_mut().downcast_mut::<crate::sdk::tools::wallet::WalletTool>() {
                    wallet.credit(agent, amount);
                    return;
                }
            }
        }
    }

    /// Record a rejection in the graveyard (for feedback to agents).
    /// Reason SHOULD be a bounded class label from `sdk::error_abstraction`
    /// (Art. II.1 abstraction mandate + C-022 context-poisoning shield).
    /// Exposed publicly so evaluator.rs can populate from OMEGA-reject and parse-fail.
    pub fn record_rejection(&mut self, author: &str, reason: &str) {
        self.graveyard
            .entry(author.to_string())
            .or_default()
            .push(reason.to_string());
    }

    /// Get recent rejections for an agent (Art. II.1: broadcast typical errors).
    /// v3 Step-B: default scope changed to TopKClasses(3) — globally abstract-and-broadcast.
    /// Call sites that explicitly want per-author scope use `recent_rejections_scoped`.
    pub fn recent_rejections(&self, author: &str, max: usize) -> Vec<String> {
        self.recent_rejections_scoped(author, max, RejectionScope::TopKClasses(3))
    }

    /// Scoped rejection query (Step-B v3 Art. II.1 fix).
    pub fn recent_rejections_scoped(
        &self,
        author: &str,
        max: usize,
        scope: RejectionScope,
    ) -> Vec<String> {
        match scope {
            RejectionScope::PerAuthor => {
                self.graveyard.get(author)
                    .map(|v| v.iter().rev().take(max).cloned().collect())
                    .unwrap_or_default()
            }
            RejectionScope::Global => {
                // Flatten all authors' recent; keep most recent `max` across swarm.
                let mut all: Vec<&String> = self.graveyard.values().flatten().collect();
                // Heuristic: assume push-order ~= time-order; take last `max` global entries.
                let start = all.len().saturating_sub(max);
                all.drain(..start);
                all.into_iter().cloned().collect()
            }
            RejectionScope::TopKClasses(k) => {
                // C-022 shield: broadcast abstracted CLASSES with COUNTS, not raw strings.
                // Expects reason strings to already be class labels (see error_abstraction).
                let mut counts: HashMap<String, u32> = HashMap::new();
                for v in self.graveyard.values() {
                    for r in v {
                        *counts.entry(r.clone()).or_insert(0) += 1;
                    }
                }
                let mut sorted: Vec<(String, u32)> = counts.into_iter().collect();
                // Sort: count DESC, then alphabetical (tiebreak stable).
                sorted.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
                sorted.truncate(k);
                // Emit as "label(count)" strings for prompt.
                sorted.into_iter()
                    .map(|(lbl, c)| format!("{}({})", lbl, c))
                    .take(max)
                    .collect()
            }
        }
    }

    /// Get a snapshot of the universe for agents to read.
    pub fn snapshot(&self) -> crate::sdk::snapshot::UniverseSnapshot {
        let markets: HashMap<NodeId, crate::sdk::snapshot::MarketSnapshot> =
            self.kernel.markets.iter()
                .map(|(id, m)| (id.clone(), crate::sdk::snapshot::MarketSnapshot {
                    yes_price: m.yes_price(),
                    no_price: m.no_price(),
                    yes_reserve: m.yes_reserve(),
                    no_reserve: m.no_reserve(),
                    resolved: m.resolved,
                }))
                .collect();

        let ticker = self.kernel.market_ticker(10);
        let ticker_str = ticker.iter()
            .map(|(id, price)| format!("{}: {:.1}%", id, price * 100.0))
            .collect::<Vec<_>>()
            .join(", ");

        crate::sdk::snapshot::UniverseSnapshot {
            tape: self.kernel.tape.clone(),
            balances: HashMap::new(), // filled by wallet tool query
            portfolios: HashMap::new(),
            markets,
            market_ticker: ticker_str,
            generation: self.generation,
            tx_count: self.tx_count,
        }
    }
}

// ── Tests ───────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sdk::tools::wallet::WalletTool;

    fn make_bus() -> TuringBus {
        let kernel = Kernel::new();
        let config = BusConfig {
            max_payload_chars: 200,
            max_payload_lines: 10,
            system_lp_amount: 200.0,
            forbidden_patterns: vec!["FORBIDDEN".to_string()],
        };
        let mut bus = TuringBus::new(kernel, config);
        bus.mount_tool(Box::new(WalletTool::new(10000.0)));
        bus.init(&["A0".into(), "A1".into()]);
        bus
    }

    #[test]
    fn test_bus_basic_append() {
        let mut bus = make_bus();
        match bus.append("A0", "step 1", None).unwrap() {
            BusResult::Appended { node_id } => {
                assert!(node_id.starts_with("tx_"));
                assert!(bus.kernel.tape.get(&node_id).is_some());
            }
            _ => panic!("Expected Appended"),
        }
    }

    #[test]
    fn test_bus_forbidden_pattern_veto() {
        let mut bus = make_bus();
        match bus.append("A0", "this is FORBIDDEN content", None).unwrap() {
            BusResult::Vetoed { reason } => {
                assert!(reason.contains("Forbidden"));
            }
            _ => panic!("Expected Vetoed"),
        }
    }

    #[test]
    fn test_bus_payload_too_long() {
        let mut bus = make_bus();
        let long_payload = "x".repeat(300);
        match bus.append("A0", &long_payload, None).unwrap() {
            BusResult::Vetoed { reason } => {
                assert!(reason.contains("too long"));
            }
            _ => panic!("Expected Vetoed"),
        }
    }

    #[test]
    fn test_bus_too_many_lines() {
        let mut bus = make_bus();
        let many_lines = (0..20).map(|i| format!("line {}", i)).collect::<Vec<_>>().join("\n");
        match bus.append("A0", &many_lines, None).unwrap() {
            BusResult::Vetoed { reason } => {
                assert!(reason.contains("many lines"));
            }
            _ => panic!("Expected Vetoed"),
        }
    }

    #[test]
    fn test_bus_unknown_agent_vetoed() {
        let mut bus = make_bus();
        match bus.append("unknown", "step", None).unwrap() {
            BusResult::Vetoed { reason } => {
                assert!(reason.contains("Unknown"));
            }
            _ => panic!("Expected Vetoed"),
        }
    }

    #[test]
    fn test_bus_creates_market_on_append() {
        let mut bus = make_bus();
        if let BusResult::Appended { node_id } = bus.append("A0", "step 1", None).unwrap() {
            assert!(bus.kernel.markets.contains_key(&node_id));
        }
    }

    #[test]
    fn test_bus_halt_and_settle() {
        let mut bus = make_bus();
        if let BusResult::Appended { node_id } = bus.append("A0", "step", None).unwrap() {
            bus.halt_and_settle(&[node_id.clone()]).unwrap();
            assert_eq!(bus.kernel.markets[&node_id].resolved, Some(true));
        }
    }

    #[test]
    fn test_bus_ledger_integrity() {
        let mut bus = make_bus();
        bus.append("A0", "step 1", None).unwrap();
        bus.append("A1", "step 2", None).unwrap();
        assert!(bus.ledger.verify().is_ok());
        assert!(bus.ledger.len() >= 3); // RunStart + 2 appends
    }

    #[test]
    fn test_bus_graveyard_feedback() {
        let mut bus = make_bus();
        bus.append("A0", "this is FORBIDDEN content", None).unwrap();
        let rejections = bus.recent_rejections("A0", 5);
        assert_eq!(rejections.len(), 1);
        assert!(rejections[0].contains("Forbidden"));
    }

    #[test]
    fn test_bus_snapshot() {
        let mut bus = make_bus();
        bus.append("A0", "step 1", None).unwrap();
        let snap = bus.snapshot();
        assert_eq!(snap.tx_count, 1);
        assert!(!snap.markets.is_empty());
    }

    #[test]
    fn test_bus_serial_ordering() {
        // V3L-11: tx_count must increment monotonically
        let mut bus = make_bus();
        for i in 0..5 {
            bus.append("A0", &format!("step {}", i), None).unwrap();
        }
        assert_eq!(bus.tx_count, 5);
        assert_eq!(bus.clock, 5);
    }
}
