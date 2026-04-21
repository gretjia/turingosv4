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
    // Phase 1 (C-037 candidate): durable Q_t. None = legacy in-memory mode.
    wal: Option<crate::wal::Wal>,
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
            wal: None,
        }
    }

    /// Phase 1: open with WAL persistence. If the path exists, replay it to
    /// rebuild tape + ledger state (resume mode). If not, start fresh and append
    /// to the WAL going forward (durable mode). Either way, the Wal handle is
    /// retained and every successful tape.append / ledger.append persists.
    pub fn with_wal_path(
        kernel: Kernel,
        config: BusConfig,
        wal_path: impl Into<std::path::PathBuf>,
    ) -> Result<Self, std::io::Error> {
        let wal_path = wal_path.into();
        let mut bus = Self::new(kernel, config);
        // Replay first (if file exists), then open in append mode.
        let (nodes, events) = crate::wal::Wal::replay(&wal_path)?;
        let resumed_nodes = nodes.len();
        let resumed_events = events.len();
        for n in nodes {
            // Replay errors are tolerable — duplicates and dangling cites can
            // happen if the WAL was concurrently appended at a stale point. We
            // log and skip; the surviving prefix is canonical Q_t.
            if let Err(e) = bus.kernel.append(n.clone()) {
                eprintln!("[wal/replay] skip node {}: {}", n.id, e);
            }
        }
        for e in events {
            // Re-append events through the ledger so hash chain is recomputed
            // from this process's perspective. Original hashes are discarded.
            bus.ledger.append(e.event_type, e.node_id, e.agent, e.detail).ok();
        }
        if resumed_nodes > 0 || resumed_events > 0 {
            eprintln!("[wal/replay] resumed {} nodes, {} events from {:?}",
                      resumed_nodes, resumed_events, wal_path);
        }
        bus.wal = Some(crate::wal::Wal::open(&wal_path)?);
        Ok(bus)
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
        // Phase 3A (Hayek): open the bounty market at genesis if the feature
        // is enabled. Seed from ghost-liquidity pool (same exemption as per-
        // node markets — pre-committed LP, not a mint). BOUNTY_LP env tunable
        // for experimentation; constitutional default lands later.
        if std::env::var("HAYEK_BOUNTY").ok().as_deref() == Some("1") {
            let lp: f64 = std::env::var("BOUNTY_LP")
                .ok().and_then(|s| s.parse().ok())
                .unwrap_or(self.config.system_lp_amount);
            let _ = self.kernel.open_bounty_market(lp);
        }
        if let Ok(evt) = self.ledger.append(EventType::RunStart, None, None, None) {
            let evt_clone = evt.clone();
            if let Some(w) = self.wal.as_mut() {
                let _ = w.write_event(&evt_clone);
            }
        }
    }

    /// The main append pipeline — 6 phases.
    /// V3L-11: this runs serially, never concurrently.
    pub fn append(&mut self, author: &str, payload: &str,
                  parent_id: Option<&str>) -> Result<BusResult, String> {
        self.append_internal(author, payload, parent_id, /*oracle_blessed*/ false)
    }

    /// Phase 2.1 (C-043 candidate): bypass agent-facing gates for ∏p-blessed payloads.
    /// The forbidden_patterns list (C-011) exists to prevent agents from appending
    /// brute-force tactics (e.g. bare `decide`, `omega`, `native_decide`) as scratch
    /// work. Once the Lean oracle has accepted a full proof, those same tactics are
    /// by construction legitimate — re-rejecting at bus level would block the
    /// wtool write that Art. IV mandates. Only oracle-accepted payloads should
    /// take this path. Payload-size caps are also relaxed (proofs are longer than
    /// agent scratch steps).
    pub fn append_oracle_accepted(&mut self, author: &str, payload: &str,
                                   parent_id: Option<&str>) -> Result<BusResult, String> {
        self.append_internal(author, payload, parent_id, /*oracle_blessed*/ true)
    }

    fn append_internal(&mut self, author: &str, payload: &str,
                       parent_id: Option<&str>, oracle_blessed: bool) -> Result<BusResult, String> {
        // Phase 0: Forbidden pattern check — skipped for oracle-accepted payloads.
        if !oracle_blessed {
            for pattern in &self.config.forbidden_patterns {
                if payload.contains(pattern.as_str()) {
                    let reason = format!("Forbidden pattern: {}", pattern);
                    self.record_rejection(author, &reason);
                    return Ok(BusResult::Vetoed { reason });
                }
            }
        }

        // Phase 0b: Payload size limits (V3L-21). Skipped for oracle-accepted since
        // real proofs can legitimately exceed the per-step scratch budget.
        if !oracle_blessed {
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

            if let Ok(evt) = self.ledger.append(EventType::Invest, Some(target_node.clone()),
                               Some(author.to_string()), None) {
                let evt_clone = evt.clone();
                if let Some(w) = self.wal.as_mut() {
                    let _ = w.write_event(&evt_clone);
                }
            }
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

        self.kernel.append(node.clone()).map_err(|e| e.to_string())?;

        // Phase 1 WAL: persist node AFTER successful in-memory append, BEFORE
        // any downstream effects. At-most-one-loss-on-crash semantics: if the
        // process dies between in-memory insert and this write, the node is
        // lost on replay but every prior node survives. Log+continue on I/O
        // error rather than aborting the run (Q_t durability is best-effort
        // when disk is the failing component).
        if let Some(w) = self.wal.as_mut() {
            if let Err(e) = w.write_node(&node) {
                log::warn!("[wal] write_node({}) failed: {}", node.id, e);
            }
        }

        // Phase 4: System Market Maker (Magna Carta Rule #19 exemption)
        // System MM injects liquidity from a dedicated pool, NOT from agent wallets.
        // This is the only constitutional exception to Law 2 — MM impermanent loss
        // is an expected physical cost, not minting. See C-001/C-002 for history.
        self.kernel.create_market(&node_id, self.config.system_lp_amount)
            .ok(); // Market creation failure is non-fatal

        // Phase 2 (C-042 candidate): founder grant — the author of this tape
        // node auto-receives γ·system_lp YES shares. No Coin is minted: the
        // market's redeem math pays out at most `lp_coins` on the winning side,
        // so these shares draw from pre-committed ghost liquidity (same as how
        // agent `invest` does). Gated by TAPE_ECONOMY_V2=1; γ via
        // FOUNDER_GRANT_GAMMA env (experimental) → constitutional default at merge.
        if std::env::var("TAPE_ECONOMY_V2").ok().as_deref() == Some("1") {
            let gamma: f64 = std::env::var("FOUNDER_GRANT_GAMMA")
                .ok().and_then(|s| s.parse().ok()).unwrap_or(0.05);
            let grant_shares = gamma * self.config.system_lp_amount;
            for tool in &mut self.tools {
                if tool.manifest() == "wallet" {
                    if let Some(wallet) = tool.as_any_mut()
                        .downcast_mut::<crate::sdk::tools::wallet::WalletTool>()
                    {
                        wallet.record_shares(author, &node_id, grant_shares, 0.0, 0.0);
                        break;
                    }
                }
            }
        }

        // Phase 5: Tool post-append hooks
        for tool in &mut self.tools {
            tool.on_post_append(author, &node_id);
        }

        if let Ok(evt) = self.ledger.append(EventType::Append, Some(node_id.clone()),
                                             Some(author.to_string()), None) {
            // Phase 1 WAL: persist ledger event for full hash-chain recovery.
            if let Some(w) = self.wal.as_mut() {
                let evt_clone = evt.clone();
                if let Err(e) = w.write_event(&evt_clone) {
                    log::warn!("[wal] write_event(Append) failed: {}", e);
                }
            }
        }
        self.tx_count += 1;
        self.clock += 1;

        Ok(BusResult::Appended { node_id })
    }

    /// Halt and settle — triggered by Oracle verification.
    pub fn halt_and_settle(&mut self, golden_path: &[NodeId]) -> Result<(), String> {
        // Resolve all markets
        self.kernel.resolve_all(golden_path).map_err(|e| e.to_string())?;

        // Phase 2: pay out every agent's YES/NO positions against resolved markets.
        // Shares redeem 1:1 against the winning side; losing shares redeem to 0.
        // Conservation: LP that backed the market flows to winners; total Coin
        // across the system is preserved (LP-side only). Gated behind the same
        // TAPE_ECONOMY_V2 toggle so baseline runs keep historical behaviour.
        if std::env::var("TAPE_ECONOMY_V2").ok().as_deref() == Some("1") {
            self.settle_portfolios();
        }

        // Phase 3A (Hayek): resolve the bounty market and distribute its
        // committed LP to GP-node authors by occurrence count. This creates
        // the cross-agent reward that makes appending a lemma EV-positive
        // independently of whether the lemma-author also closes the proof.
        if std::env::var("HAYEK_BOUNTY").ok().as_deref() == Some("1") {
            let gp_authors: Vec<String> = golden_path.iter()
                .filter_map(|nid| self.kernel.tape.get(nid).map(|n| n.author.clone()))
                .collect();
            let payouts = self.kernel.resolve_bounty(&gp_authors);
            for (agent, amount) in payouts {
                self.credit_wallet(&agent, amount);
            }
        }

        // Tool halt hooks
        let gp: Vec<String> = golden_path.to_vec();
        for tool in &mut self.tools {
            tool.on_halt(&gp);
        }

        if let Ok(evt) = self.ledger.append(EventType::RunEnd, None, None, None) {
            let evt_clone = evt.clone();
            if let Some(w) = self.wal.as_mut() {
                let _ = w.write_event(&evt_clone);
            }
        }
        Ok(())
    }

    /// Phase 2: redeem every agent's portfolio against resolved markets.
    /// Walks wallet.portfolios, finds matching resolved market, credits wallet
    /// with share count on the winning side (0 on the losing side). Resolved
    /// positions are zeroed to prevent double-redemption on a second call.
    /// Conservation: pays only from LP already committed at market creation.
    fn settle_portfolios(&mut self) {
        use crate::sdk::tools::wallet::WalletTool;
        // Snapshot resolved outcomes so we can borrow kernel + wallet disjointly.
        let outcomes: HashMap<String, bool> = self.kernel.markets.iter()
            .filter_map(|(id, m)| m.resolved.map(|w| (id.clone(), w)))
            .collect();
        let wallet: &mut WalletTool = match self.tools.iter_mut()
            .find_map(|t| t.as_any_mut().downcast_mut::<WalletTool>())
        {
            Some(w) => w,
            None => return,
        };
        let mut credits: Vec<(String, f64)> = Vec::new();
        for (agent, portfolio) in wallet.portfolios.iter_mut() {
            for (node_id, entry) in portfolio.iter_mut() {
                let (yes, no, _lp) = *entry;
                if let Some(yes_wins) = outcomes.get(node_id) {
                    let payout = if *yes_wins { yes } else { no };
                    if payout > 0.0 {
                        credits.push((agent.clone(), payout));
                    }
                    // Zero out settled positions to make settle_portfolios idempotent.
                    entry.0 = 0.0;
                    entry.1 = 0.0;
                }
            }
        }
        for (agent, amount) in credits {
            wallet.credit(&agent, amount);
        }
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

    /// Record a rejection in the graveyard.
    /// Step-B v3: ALL stored entries are bounded class labels (C-022 shield enforced at write).
    /// If `reason` is already a valid class label (starts with "err:"), stored as-is.
    /// Otherwise normalized to a bus-level class via `bus_classify`.
    /// Exposed publicly so evaluator.rs can populate from OMEGA-reject and parse-fail.
    pub fn record_rejection(&mut self, author: &str, reason: &str) {
        let label = Self::bus_classify(reason);
        self.graveyard
            .entry(author.to_string())
            .or_default()
            .push(label.to_string());
    }

    /// Bus-level classifier: coerces any rejection reason to a bounded label.
    /// This is the write-side shield that enforces Art. II.1 end-to-end.
    /// The finite label set is the union of:
    ///   - "err:" prefixed labels from sdk::error_abstraction (caller-classified)
    ///   - "veto:forbidden", "veto:size", "veto:lines", "veto:wallet", "veto:tool_other"
    ///     (bus-internal veto classes)
    ///   - "err:other" catchall
    pub fn bus_classify(reason: &str) -> &'static str {
        // If caller already produced an "err:..." class label, trust it.
        // Validate prefix; the length is bounded because the enum of labels is finite.
        if reason.starts_with("err:") {
            // Accept as-is but intern to static slice where possible.
            // For simplicity we allocate a leaked &'static; safer: fixed mapping of known labels.
            // Here we collapse unknown "err:*" to err:other to preserve finite-set invariant.
            return match reason {
                "err:tactic_linarith" => "err:tactic_linarith",
                "err:tactic_simp_noprog" => "err:tactic_simp_noprog",
                "err:tactic_ring" => "err:tactic_ring",
                "err:tactic_norm_num" => "err:tactic_norm_num",
                "err:tactic_other" => "err:tactic_other",
                "err:unknown_const" => "err:unknown_const",
                "err:unsolved_goals" => "err:unsolved_goals",
                "err:unexpected_token" => "err:unexpected_token",
                "err:type_mismatch" => "err:type_mismatch",
                "err:rewrite_no_match" => "err:rewrite_no_match",
                "err:heartbeat" => "err:heartbeat",
                "err:other" => "err:other",
                _ => "err:other",
            };
        }
        // Bus internal veto reasons get their own bounded classes.
        if reason.starts_with("Forbidden") { return "veto:forbidden"; }
        if reason.starts_with("Payload too long") { return "veto:size"; }
        if reason.starts_with("Too many lines") { return "veto:lines"; }
        if reason.contains("wallet") || reason.contains("balance") { return "veto:wallet"; }
        if reason.starts_with("Tool") || reason.contains("tool") { return "veto:tool_other"; }
        "err:other"
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
        let mut ticker_lines: Vec<String> = ticker.iter()
            .map(|(id, price)| format!("{}: {:.1}%", id, price * 100.0))
            .collect();
        // Phase 3A: surface the bounty price first so agents see the pre-
        // existing signal. No prose, no rule — just price-as-state (Hayek).
        if let Some(bp) = self.kernel.bounty_yes_price() {
            ticker_lines.insert(0,
                format!("__bounty__: {:.1}% (LP={:.0})", bp * 100.0,
                        self.kernel.bounty_lp_seed));
        }
        let ticker_str = ticker_lines.join(", ");

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
        // Step-B v3: default recent_rejections() returns TopKClasses-abstracted labels.
        // Raw "Forbidden pattern: ..." strings are normalized to "veto:forbidden" via bus_classify.
        let mut bus = make_bus();
        bus.append("A0", "this is FORBIDDEN content", None).unwrap();
        // TopKClasses default: returns "label(count)"
        let rejections = bus.recent_rejections("A0", 5);
        assert_eq!(rejections.len(), 1);
        assert!(
            rejections[0].contains("veto:forbidden"),
            "expected abstracted class label, got: {:?}", rejections[0],
        );
        // Per-author scope still returns raw labels without count
        let per_author = bus.recent_rejections_scoped(
            "A0", 5, crate::bus::RejectionScope::PerAuthor,
        );
        assert_eq!(per_author, vec!["veto:forbidden".to_string()]);
    }

    #[test]
    fn test_bus_classify_bounded() {
        // Invariant: bus_classify never returns unbounded text.
        assert_eq!(TuringBus::bus_classify("Forbidden pattern: decide"), "veto:forbidden");
        assert_eq!(TuringBus::bus_classify("Payload too long: 9999 > 1000"), "veto:size");
        assert_eq!(TuringBus::bus_classify("Too many lines: 50 > 18"), "veto:lines");
        assert_eq!(TuringBus::bus_classify("err:tactic_linarith"), "err:tactic_linarith");
        assert_eq!(TuringBus::bus_classify("err:unknown_variant_we_dont_track"), "err:other");
        assert_eq!(TuringBus::bus_classify("some unprecedented garbage"), "err:other");
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
