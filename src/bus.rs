// Tier 4: TSP Event Bus — SKILL lifecycle serial reactor
// Constitutional basis: Art. II (selective broadcast), Art. III (selective shielding)
// V3L-11: serial reactor for causal ordering (no concurrent pricing oscillation)
// V3L-21: one-step-per-node payload limits
// V3L-31: supervisor loop, never silent exit
// V3L-32: cascade failure protection

use crate::kernel::{Kernel, KernelError};
use crate::ledger::{EventType, Ledger, Node, NodeId, TapeError};
use crate::sdk::tool::{ToolSignal, TuringTool};
use crate::state::sequencer::{Sequencer, SubmissionReceipt, SubmitError};
use crate::state::typed_tx::TypedTx;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

// ── Symbolic constants (V-01 ceremonial kill per D-VETO-7 ratified A) ──────────

/// TRACE_MATRIX FC1-Cost / FC3-Cost: placeholder until CO1.1.4 STEP_B propagates
/// real LLM completion tokens from `drivers::llm_http::LlmResponse` through to
/// `Node::completion_tokens`. CO1.1.4-pre1 ceremonial commit replaces the magic
/// literal `0` at `bus.rs:268` with this named constant; the value is unchanged
/// (still 0), but the literal is killed so the STEP_B refactor has a clear
/// rename target rather than an anonymous integer.
///
/// See `handover/architect-insights/PROJECT_DECISION_MAP_2026-04-27.md` § 2.2
/// D-VETO-7 for the ratified disposition.
pub(crate) const PENDING_COMPLETION_TOKENS_CO1_1_4: u32 = 0;

// ── Configuration ───────────────────────────────────────────────

/// Bus configuration. V3L-23: no hardcoded values, all configurable.
///
/// TB-14 Atom 6 (2026-05-03): `system_lp_amount: f64` was excised together
/// with `kernel.create_market` (legacy CPMM scaffolding). Pricing is now a
/// derived view over `EconomicState` via `state::compute_price_index`; no
/// LP injection at bus level.
pub struct BusConfig {
    pub max_payload_chars: usize,
    pub max_payload_lines: usize,
    pub forbidden_patterns: Vec<String>,
}

impl Default for BusConfig {
    fn default() -> Self {
        BusConfig {
            max_payload_chars: 1600,
            max_payload_lines: 24,
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
    /// CO1.7-extra D3: typed-tx Sequencer; `None` when bus runs in legacy
    /// ledger-only mode. Spec § 2.1 + D3 STEP_B Branch A. `#[serde(skip)]`
    /// is conditional on TuringBus having serde derives — it currently
    /// does not (per `pub struct TuringBus` declaration above), so the
    /// attribute is omitted at this landing. If a future atom adds serde
    /// to TuringBus, the skip MUST be added in the same patch.
    pub sequencer: Option<Arc<Sequencer>>,
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
            sequencer: None,
        }
    }

    /// CO1.7-extra D3: opt-in constructor wiring a typed-tx Sequencer
    /// alongside the legacy ledger. Spec § 2.1 + § 2.2 (Sequencer lives at
    /// TuringBus level, not nested through Kernel).
    ///
    /// TRACE_MATRIX § 5.2.1 — single-writer entry-point.
    pub fn with_sequencer(
        kernel: Kernel,
        config: BusConfig,
        sequencer: Arc<Sequencer>,
    ) -> Self {
        let mut bus = Self::new(kernel, config);
        bus.sequencer = Some(sequencer);
        bus
    }

    /// CO1.7-extra D3: typed-tx submission path. Returns receipt
    /// (`submit_id`) immediately; commit happens asynchronously in
    /// `Sequencer::run` driver loop.
    ///
    /// Returns `Err(SubmitError::QueueClosed)` when the bus runs in
    /// legacy-only mode (no Sequencer wired).
    ///
    /// TRACE_MATRIX § 5.2.1 — typed-tx submission entry.
    pub async fn submit_typed_tx(
        &self,
        tx: TypedTx,
    ) -> Result<SubmissionReceipt, SubmitError> {
        match self.sequencer.as_ref() {
            Some(seq) => seq.submit(tx).await,
            None => Err(SubmitError::QueueClosed),
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
    ///
    /// TB-14 Atom 6 (2026-05-03): legacy `HAYEK_BOUNTY` env-gated bounty
    /// market open was excised together with `kernel.open_bounty_market`.
    /// Capital signals now live entirely in `state::NodePositionsIndex`
    /// (TB-12) and surface via `compute_price_index` derived view (TB-14).
    pub fn init(&mut self, agent_ids: &[String]) {
        for tool in &mut self.tools {
            tool.on_init(agent_ids);
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
        // TB-9 collapse (2026-05-02): InvestOnly routing deleted along with the
        // bus-level f64 wallet mutators (debit_wallet/credit_wallet/settle_portfolios).
        // Per architect directive 2026-05-02 line 1574 ("no f64 mutation;
        // EconomicState canonical"), the v3 share-buy path is gone. Stake
        // commitment now lives in `state::typed_tx::WorkTx.stake` mutating
        // `EconomicState.stakes_t` via the canonical sequencer dispatch arm.
        // YieldReward signals continue to be observed but are not routed to a
        // f64 mutator — they live for downstream tool hooks only.
        for tool in &mut self.tools {
            match tool.on_pre_append(author, payload) {
                ToolSignal::Veto(reason) => {
                    self.record_rejection(author, &reason);
                    return Ok(BusResult::Vetoed { reason });
                }
                ToolSignal::InvestOnly { .. } => {
                    let reason = "veto:invest_disabled_tb9".to_string();
                    self.record_rejection(author, &reason);
                    return Ok(BusResult::Vetoed { reason });
                }
                ToolSignal::YieldReward { .. } | ToolSignal::Pass => {}
            }
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
            completion_tokens: PENDING_COMPLETION_TOKENS_CO1_1_4,
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

        // Phase 4: TB-14 Atom 6 (2026-05-03 closing OBS_TB_12_LEGACY_CPMM_QUARANTINE):
        // legacy `kernel.create_market(node_id, system_lp_amount)` per-append
        // CPMM market open was excised together with `prediction_market.rs`.
        // Pricing is now a derived view over canonical `EconomicState`
        // (`state::compute_price_index`) populated by typed-tx admission via
        // `Sequencer::dispatch_transition` — never by bus-level f64 LP grant.

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
    ///
    /// TB-14 Atom 6 (2026-05-03): legacy `kernel.resolve_all(golden_path)`
    /// CPMM market resolution was excised together with `prediction_market.rs`.
    /// Settlement lives entirely in canonical typed-tx dispatch arms
    /// (`FinalizeRewardTx` since TB-8) via `Sequencer::apply_one`; the bus
    /// only fires the run-end event and lets tool hooks observe the golden
    /// path.
    pub fn halt_and_settle(&mut self, golden_path: &[NodeId]) -> Result<(), String> {
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
    ///
    /// TRACE_MATRIX TB-14 Atom 6 (FC2-N28 + FC3-N42; architect §5.1 +
    /// charter §3 Atom 6): the snapshot now carries the integer-rational
    /// `price_index` + `mask_set` derived from canonical `EconomicState`
    /// via `state::compute_price_index` + `state::compute_mask_set`,
    /// replacing the legacy decimal-float `markets: HashMap<_, MarketSnapshot>`
    /// CPMM read-view excised together with `src/prediction_market.rs`.
    ///
    /// **Halt-trigger #2 spirit preserved**: bus.rs imports TB-14 types
    /// (this is the legitimate broadcast point per kickoff doc), but the
    /// L4/L4.E classification path in `Sequencer::dispatch_transition`
    /// remains free of TB-14 imports — verified by halt-trigger #2's
    /// `use`-statement scan over `src/state/sequencer.rs`. The price
    /// signal flows: `EconomicState (canonical)` →
    /// `compute_price_index (pure derive)` → snapshot read-view →
    /// scheduler / dashboard / agent prompt. It NEVER flows back into
    /// `dispatch_transition`.
    ///
    /// **Replay-deterministic** (Art.0.2): `compute_price_index` and
    /// `compute_mask_set` are pure over their inputs. The snapshot's
    /// `price_index` / `mask_set` are reproducible from any byte-equal
    /// `EconomicState` + `Tape` + `BoltzmannMaskPolicy` without re-running
    /// the run.
    ///
    /// **Sequencer-optional**: when the bus runs in legacy ledger-only
    /// mode (`sequencer == None`, e.g. in WAL-only smoke tests), the
    /// price_index + mask_set are empty `BTreeMap` / `BTreeSet`. Callers
    /// (evaluator, dashboard) treat empty as "no signal yet" — they MUST
    /// NOT crash on empty.
    pub fn snapshot(&self) -> crate::sdk::snapshot::UniverseSnapshot {
        let policy = crate::state::BoltzmannMaskPolicy::from_env();

        let (price_index, mask_set) = match self.sequencer.as_ref() {
            Some(seq) => match seq.q_snapshot() {
                Ok(q) => {
                    let pi = crate::state::compute_price_index(&q.economic_state_t);
                    let ms = crate::state::compute_mask_set(
                        &q.economic_state_t,
                        &self.kernel.tape,
                        &policy,
                        &pi,
                    );
                    (pi, ms)
                }
                Err(_) => (
                    std::collections::BTreeMap::new(),
                    std::collections::BTreeSet::new(),
                ),
            },
            None => (
                std::collections::BTreeMap::new(),
                std::collections::BTreeSet::new(),
            ),
        };

        crate::sdk::snapshot::UniverseSnapshot {
            tape: self.kernel.tape.clone(),
            price_index,
            mask_set,
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
            forbidden_patterns: vec!["FORBIDDEN".to_string()],
        };
        let mut bus = TuringBus::new(kernel, config);
        bus.mount_tool(Box::new(WalletTool::new()));
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

    /// TB-9 collapse (2026-05-02): pre-TB-9 the WalletTool's `on_pre_append`
    /// vetoed unknown agents because they had no f64 balance row. After the
    /// projection collapse (no f64 ledger, `on_pre_append` returns `Pass`
    /// unconditionally), the v3 bus append path is genuinely Law 1 free for
    /// any author — typed_tx admission gates own author/balance veto logic at
    /// the canonical layer. Test renamed + inverted to lock in the new
    /// invariant.
    #[test]
    fn test_bus_unknown_agent_appends_post_tb9_collapse() {
        let mut bus = make_bus();
        match bus.append("unknown", "step", None).unwrap() {
            BusResult::Appended { .. } => {}
            other => panic!("Expected Appended (post-TB-9 collapse), got {:?}", other),
        }
    }

    #[test]
    fn test_bus_halt_and_settle() {
        // TB-14 Atom 6: kernel.markets.resolved was excised with
        // prediction_market.rs. halt_and_settle now only fires RunEnd +
        // tool.on_halt hooks; settlement state lives in canonical typed-tx
        // dispatch (FinalizeRewardTx). Test verifies the call succeeds and
        // the run-end ledger event landed.
        let mut bus = make_bus();
        if let BusResult::Appended { node_id } = bus.append("A0", "step", None).unwrap() {
            let len_before = bus.ledger.len();
            bus.halt_and_settle(&[node_id]).unwrap();
            assert!(
                bus.ledger.len() > len_before,
                "halt_and_settle must append RunEnd event"
            );
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
        // TB-14 Atom 6: snapshot.markets HashMap was replaced by
        // price_index: BTreeMap<TxId, NodeMarketEntry> + mask_set: BTreeSet<TxId>.
        // Without a sequencer wired (legacy ledger-only mode), both are empty
        // — the bus snapshot is sequencer-optional per CR-14.x; consumers
        // (evaluator, dashboard) treat empty as "no signal yet".
        let mut bus = make_bus();
        bus.append("A0", "step 1", None).unwrap();
        let snap = bus.snapshot();
        assert_eq!(snap.tx_count, 1);
        assert!(snap.price_index.is_empty(), "no sequencer → empty price_index");
        assert!(snap.mask_set.is_empty(), "no sequencer → empty mask_set");
        assert!(snap.tape.get(&"tx_0_by_A0".to_string()).is_some(),
                "appended node is in tape regardless of price index state");
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
