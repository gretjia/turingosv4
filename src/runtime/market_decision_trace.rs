//! TB-N3 A2 (architect ruling 2026-05-11 §8.1 + §8.2 + §8.6) — agent invest
//! decision audit trail.
//!
//! Every agent emission of `{"tool":"invest", ...}` produces a
//! `MarketDecisionTrace` CAS object recording: agent identity, the prompt
//! `=== Market ===` block hash that informed the decision, the visible
//! candidate node ids, the chosen node + direction + amount + quoted price
//! (if any), and a `TraceOutcome` that distinguishes:
//!
//! - `Submitted{tx_id}` — the invest payload was constructed into a
//!   `BuyWithCoinRouterTx` and submitted; downstream `tx_id` is the L4 / L4.E
//!   anchor.
//! - `NoTrade{reason}` — pre-submit classification rejected the payload
//!   (zero amount, missing pool, balance shortfall, malformed node, etc.);
//!   no on-chain tx is constructed; trace lives in CAS only.
//! - `Declined` — agent emitted invest with `amount = 0` AS A SIGNAL of
//!   declination (vs. NoTrade::ZeroAmount which is parser-induced); kept
//!   distinct so the no-trade-reason aggregate report can separate
//!   "agent chose not to trade" from "system rejected agent attempt".
//!
//! Constitutional binding:
//! - Architect §8.6 "Failed invest 也算有意义 tape activity" — failed invests
//!   that actually constructed a tx anchor in L4.E; pre-submit failures
//!   anchor in CAS via this trace.
//! - Architect §8.1 "MarketDecisionTrace" — externalized market-decision
//!   audit (NOT private CoT; the agent's market-side decision is a
//!   first-class L4.E-or-trace-bearing action, not an internal monologue).
//! - Architect §8.2 "No-trade reason report" — `NoTradeReason` enum is the
//!   canonical taxonomy aggregated in `audit_dashboard --run-report` §F.
//!
//! Replay determinism: the trace is constructed from inputs the
//! evaluator already has (snapshot Q + agent action + candidate list).
//! No clock or randomness in the trace itself; storage layer (CasStore)
//! provides content-addressing.

use serde::{Deserialize, Serialize};

use crate::state::q_state::{AgentId, TxId};
use crate::state::typed_tx::BuyDirection;

/// TB-N3 A2 §8.2 verbatim taxonomy — every distinct cause for an invest
/// emission to NOT result in a submitted `BuyWithCoinRouterTx`.
///
/// Stable insertion order matches the `audit_dashboard --run-report` §F
/// breakdown columns.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum NoTradeReason {
    /// Agent prompt did not advertise the invest tool (e.g.
    /// `TURINGOS_DISABLE_MARKET_TOOLS=1`).
    NoPromptTool,
    /// Agent emitted invest but the payload failed parser shape check
    /// (missing node / missing direction / non-numeric amount).
    NoParsedInvest,
    /// Agent's `node` value did not refer to an L4-accepted WorkTx
    /// or contained illegal characters.
    MalformedNode,
    /// Agent emitted invest with `amount = 0` (neither submission nor
    /// SignalDeclined — bare zero).
    ZeroAmount,
    /// Agent's amount exceeded current balance.
    AmountExceedsBalance,
    /// Snapshot showed no Active CPMM pool for the resolved EventId
    /// (TB-N3 A3 auto-emit hasn't fired for this WorkTx yet, OR
    /// MarketMakerBudget was exhausted).
    NoPool,
    /// Router admission rejected (covers `RouterPoolNotActive`,
    /// `RouterInsufficientCoinBalance`, `RouterSwapInsufficientPoolOutput`,
    /// `RouterSlippageExceeded`, `RouterZeroPay` per Stage C P-M6
    /// `TransitionError` taxonomy).
    RouterRejected,
    /// Agent explicitly emitted invest with `amount = 0` as a "decline"
    /// signal (distinct from `ZeroAmount` parser output — `Declined` is
    /// runtime-classified when the prompt block exposed pools but agent
    /// chose to pass).
    AgentDeclined,
    /// Problem solved in 1-tx OMEGA before any peer agent had a window
    /// to invest (P01/P02-class fast-solve).
    TooFastSolve,
    /// Pool exists but quoted output for this `pay_coin` would be 0
    /// (slippage / depth too thin).
    SlippageOutZero,
    /// Catch-all for unanticipated paths; tagged for forward-investigation.
    Unknown,
}

impl NoTradeReason {
    /// Stable string label for `tool_dist` counter keys + run-report
    /// aggregate columns. Lower-snake-case so it composes with the
    /// existing evaluator `tool_dist` convention.
    pub const fn label(&self) -> &'static str {
        match self {
            NoTradeReason::NoPromptTool => "no_prompt_tool",
            NoTradeReason::NoParsedInvest => "no_parsed_invest",
            NoTradeReason::MalformedNode => "malformed_node",
            NoTradeReason::ZeroAmount => "zero_amount",
            NoTradeReason::AmountExceedsBalance => "amount_exceeds_balance",
            NoTradeReason::NoPool => "no_pool",
            NoTradeReason::RouterRejected => "router_rejected",
            NoTradeReason::AgentDeclined => "agent_declined",
            NoTradeReason::TooFastSolve => "too_fast_solve",
            NoTradeReason::SlippageOutZero => "slippage_out_zero",
            NoTradeReason::Unknown => "unknown",
        }
    }
}

/// TB-N3 A2 §8.1 — outcome of a single invest decision pipeline.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TraceOutcome {
    /// Invest payload was successfully constructed into a
    /// `BuyWithCoinRouterTx` and submitted to the sequencer. The L4 / L4.E
    /// outcome is the on-chain anchor; this trace cross-references the
    /// `tx_id` for forensic correlation.
    Submitted { tx_id: TxId },
    /// Pre-submit classification or post-submit admission classified the
    /// attempt as a no-trade. `tx_id` is `None` if no tx was constructed
    /// (pre-submit) or `Some(_)` for L4.E-anchored router rejections
    /// where construction succeeded but admission failed.
    NoTrade {
        reason: NoTradeReason,
        tx_id: Option<TxId>,
    },
    /// Agent explicitly chose to pass (saw the market block, emitted
    /// invest with amount=0 as a no-trade signal). Distinct from
    /// `NoTrade { reason: ZeroAmount }` which is parser-induced.
    Declined,
}

/// TB-N3 A2 §8.1 — single agent invest decision audit record. Anchored as
/// CAS object with content-addressed CID; not embedded in canonical L4 tape
/// (per CLAUDE.md §6 the L4 / L4.E anchor is the BuyWithCoinRouterTx; this
/// trace is auxiliary CAS evidence used for run-report aggregation).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketDecisionTrace {
    /// Schema version for forward-compat parsing of the trace JSON.
    pub schema_version: String,
    /// Agent emitting the invest action.
    pub agent_id: AgentId,
    /// SHA-256 of the `=== Market ===` block bytes the agent saw at LLM
    /// call time (links the decision to the exact prompt context). Empty
    /// string if the prompt did not include a market block.
    pub prompt_market_block_sha256_hex: String,
    /// Candidate node ids visible in the prompt's `=== Market ===` block
    /// (top-K per `TURINGOS_TB_N3_MARKET_CONTEXT_K`). Empty list if no
    /// market block.
    pub seen_node_ids: Vec<TxId>,
    /// Node the agent chose to act on (None = no parseable node).
    pub chosen_node_id: Option<TxId>,
    /// Direction (None = parser failed before direction extraction).
    pub direction: Option<BuyDirection>,
    /// Amount in micro-Coin (None = parser failed before amount extraction).
    pub amount_micro: Option<i64>,
    /// Quoted price for the chosen node, if pool snapshot was available
    /// at decision time. Format: `numerator/denominator` integer-rational
    /// (NEVER decimal — architect "price is signal, not truth").
    pub quoted_price_yes: Option<String>,
    pub quoted_price_no: Option<String>,
    /// Final outcome after pre-submit + admission pipeline.
    pub outcome: TraceOutcome,
    /// Agent-public reason summary (≤ 120 chars). Surfaces in
    /// `audit_dashboard --run-report` §F.
    pub reason_summary_public: String,
}

impl MarketDecisionTrace {
    /// TB-N3 A2 schema version sentinel.
    pub const SCHEMA_VERSION: &'static str = "tb_n3.market_decision_trace.v1";

    /// Construct a NoTrade trace for a pre-submit classification failure.
    /// Caller passes the agent + reason + a short public summary (≤ 120 chars).
    pub fn no_trade(
        agent_id: AgentId,
        chosen_node_id: Option<TxId>,
        direction: Option<BuyDirection>,
        amount_micro: Option<i64>,
        reason: NoTradeReason,
        reason_summary_public: impl Into<String>,
    ) -> Self {
        let summary: String = reason_summary_public.into();
        let summary = if summary.len() > 120 {
            summary[..120].to_string()
        } else {
            summary
        };
        Self {
            schema_version: Self::SCHEMA_VERSION.to_string(),
            agent_id,
            prompt_market_block_sha256_hex: String::new(),
            seen_node_ids: Vec::new(),
            chosen_node_id,
            direction,
            amount_micro,
            quoted_price_yes: None,
            quoted_price_no: None,
            outcome: TraceOutcome::NoTrade { reason, tx_id: None },
            reason_summary_public: summary,
        }
    }

    /// Construct a Submitted trace for a successful pre-submit
    /// classification. Caller fills the `quoted_price_*` and prompt-context
    /// fields when known.
    pub fn submitted(
        agent_id: AgentId,
        chosen_node_id: TxId,
        direction: BuyDirection,
        amount_micro: i64,
        tx_id: TxId,
        reason_summary_public: impl Into<String>,
    ) -> Self {
        let summary: String = reason_summary_public.into();
        let summary = if summary.len() > 120 {
            summary[..120].to_string()
        } else {
            summary
        };
        Self {
            schema_version: Self::SCHEMA_VERSION.to_string(),
            agent_id,
            prompt_market_block_sha256_hex: String::new(),
            seen_node_ids: Vec::new(),
            chosen_node_id: Some(chosen_node_id),
            direction: Some(direction),
            amount_micro: Some(amount_micro),
            quoted_price_yes: None,
            quoted_price_no: None,
            outcome: TraceOutcome::Submitted { tx_id },
            reason_summary_public: summary,
        }
    }

    /// Construct a Declined trace (agent saw market, emitted invest with
    /// amount=0 as deliberate pass).
    pub fn declined(
        agent_id: AgentId,
        chosen_node_id: Option<TxId>,
        reason_summary_public: impl Into<String>,
    ) -> Self {
        let summary: String = reason_summary_public.into();
        let summary = if summary.len() > 120 {
            summary[..120].to_string()
        } else {
            summary
        };
        Self {
            schema_version: Self::SCHEMA_VERSION.to_string(),
            agent_id,
            prompt_market_block_sha256_hex: String::new(),
            seen_node_ids: Vec::new(),
            chosen_node_id,
            direction: None,
            amount_micro: Some(0),
            quoted_price_yes: None,
            quoted_price_no: None,
            outcome: TraceOutcome::Declined,
            reason_summary_public: summary,
        }
    }

    /// Attach prompt context (sha256 of market block + visible candidate ids).
    /// Builder-style; trace is mutable until anchored to CAS.
    pub fn with_prompt_context(
        mut self,
        prompt_market_block_sha256_hex: impl Into<String>,
        seen_node_ids: Vec<TxId>,
    ) -> Self {
        self.prompt_market_block_sha256_hex = prompt_market_block_sha256_hex.into();
        self.seen_node_ids = seen_node_ids;
        self
    }

    /// Attach quoted prices (integer-rational `n/d` format).
    pub fn with_quoted_prices(
        mut self,
        quoted_price_yes: Option<String>,
        quoted_price_no: Option<String>,
    ) -> Self {
        self.quoted_price_yes = quoted_price_yes;
        self.quoted_price_no = quoted_price_no;
        self
    }
}

/// TB-N3 A2 — write a `MarketDecisionTrace` to CAS as a content-addressed
/// JSON object. Returns the `Cid` for forensic correlation. ObjectType
/// reuses `AttemptTelemetry` since the trace shares the
/// "agent-emission audit record" semantics; future TB-N3 sub-task may add
/// a dedicated `MarketDecisionTrace` ObjectType once a stable schema is
/// pinned for V1.
pub fn write_market_decision_trace_to_cas(
    cas_store: &mut crate::bottom_white::cas::store::CasStore,
    trace: &MarketDecisionTrace,
    suffix: &str,
    logical_t: u64,
) -> Result<crate::bottom_white::cas::schema::Cid, String> {
    use crate::bottom_white::cas::schema::ObjectType;
    let bytes = serde_json::to_vec(trace)
        .map_err(|e| format!("MarketDecisionTrace serialize: {e}"))?;
    cas_store
        .put(
            &bytes,
            ObjectType::AttemptTelemetry,
            &format!("tb_n3-market-decision-trace-{suffix}"),
            logical_t,
            None,
        )
        .map_err(|e| format!("MarketDecisionTrace CAS put: {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TB-N3 A2 U1 — every NoTradeReason has a stable lower-snake-case
    /// label that the evaluator's `tool_dist` counter can use as a key.
    #[test]
    fn all_no_trade_reasons_have_stable_lower_snake_labels() {
        for reason in [
            NoTradeReason::NoPromptTool,
            NoTradeReason::NoParsedInvest,
            NoTradeReason::MalformedNode,
            NoTradeReason::ZeroAmount,
            NoTradeReason::AmountExceedsBalance,
            NoTradeReason::NoPool,
            NoTradeReason::RouterRejected,
            NoTradeReason::AgentDeclined,
            NoTradeReason::TooFastSolve,
            NoTradeReason::SlippageOutZero,
            NoTradeReason::Unknown,
        ] {
            let label = reason.label();
            assert!(!label.is_empty());
            assert_eq!(label, label.to_lowercase());
            assert!(
                label.chars().all(|c| c.is_ascii_lowercase() || c == '_'),
                "label {label:?} must be ascii lower snake"
            );
        }
    }

    /// TB-N3 A2 U2 — `no_trade` constructor truncates summary to 120 chars
    /// (architect §8.1 reason_summary_public bound).
    #[test]
    fn no_trade_summary_truncated_to_120_chars() {
        let long = "x".repeat(200);
        let trace = MarketDecisionTrace::no_trade(
            AgentId("Agent_test".into()),
            None,
            None,
            None,
            NoTradeReason::Unknown,
            long.clone(),
        );
        assert_eq!(trace.reason_summary_public.len(), 120);
        assert_eq!(trace.reason_summary_public, "x".repeat(120));
    }

    /// TB-N3 A2 U3 — submitted trace carries TxId and Submitted outcome.
    #[test]
    fn submitted_trace_carries_tx_id_outcome() {
        let trace = MarketDecisionTrace::submitted(
            AgentId("Agent_3".into()),
            TxId("worktx-Agent_2-evt-7".into()),
            BuyDirection::BuyYes,
            500_000,
            TxId("router-Agent_3-7".into()),
            "long 500k μC on Agent_2's W7",
        );
        match trace.outcome {
            TraceOutcome::Submitted { ref tx_id } => {
                assert_eq!(tx_id.0, "router-Agent_3-7");
            }
            other => panic!("expected Submitted outcome, got {other:?}"),
        }
        assert_eq!(trace.amount_micro, Some(500_000));
        assert_eq!(trace.direction, Some(BuyDirection::BuyYes));
    }

    /// TB-N3 A2 U4 — declined trace marks Declined outcome with amount=0.
    #[test]
    fn declined_trace_marks_outcome_declined() {
        let trace = MarketDecisionTrace::declined(
            AgentId("Agent_5".into()),
            Some(TxId("worktx-Agent_1-evt-3".into())),
            "agent saw market, emitted amount=0 as pass signal",
        );
        assert!(matches!(trace.outcome, TraceOutcome::Declined));
        assert_eq!(trace.amount_micro, Some(0));
    }

    /// TB-N3 A2 U5 — builder methods chain cleanly.
    #[test]
    fn builder_chain_attaches_prompt_context_and_prices() {
        let trace = MarketDecisionTrace::submitted(
            AgentId("Agent_4".into()),
            TxId("worktx-Agent_2-evt-3".into()),
            BuyDirection::BuyNo,
            100_000,
            TxId("router-Agent_4-3".into()),
            "short 100k μC on Agent_2's W3 (price=4/10)",
        )
        .with_prompt_context(
            "abc123",
            vec![
                TxId("worktx-Agent_2-evt-3".into()),
                TxId("worktx-Agent_1-evt-3".into()),
            ],
        )
        .with_quoted_prices(Some("4/10".into()), Some("6/10".into()));
        assert_eq!(trace.prompt_market_block_sha256_hex, "abc123");
        assert_eq!(trace.seen_node_ids.len(), 2);
        assert_eq!(trace.quoted_price_yes.as_deref(), Some("4/10"));
        assert_eq!(trace.quoted_price_no.as_deref(), Some("6/10"));
    }

    /// TB-N3 A2 U6 — schema version sentinel matches the documented
    /// "tb_n3.market_decision_trace.v1" — guards forward parser drift.
    #[test]
    fn schema_version_sentinel_pinned() {
        assert_eq!(MarketDecisionTrace::SCHEMA_VERSION, "tb_n3.market_decision_trace.v1");
        let trace = MarketDecisionTrace::no_trade(
            AgentId("a".into()),
            None,
            None,
            None,
            NoTradeReason::NoPool,
            "no pool yet for chosen node",
        );
        assert_eq!(trace.schema_version, "tb_n3.market_decision_trace.v1");
    }
}
