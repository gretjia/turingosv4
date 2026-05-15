//! REAL-11 market transaction category helpers.
//!
//! These helpers keep REAL-10/REAL-11 reports from collapsing structural
//! market activity into live agent economic action. They are report-side
//! classifiers only; ChainTape/CAS remains the source of truth.

use serde::{Deserialize, Serialize};

/// TRACE_MATRIX FC3-N43 (REAL-11): claim-boundary split for market-facing
/// activity in dashboard/report materialized views.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MarketTxCategory {
    /// System-created market infrastructure such as MarketSeed/CpmmPool.
    StructuralMarketTx,
    /// Live, non-scripted, non-forced agent action with trace/audit provenance.
    AgentEconomicActionTx,
    /// Scripted fixture or missing-provenance action. This can test wiring but
    /// cannot satisfy E2 spontaneous market action.
    ScriptedFixtureTx,
    /// Resolution/settlement activity, such as EventResolve.
    ResolutionTx,
}

/// TRACE_MATRIX FC3-N43 (REAL-11): provenance gate before a router tx may
/// count as live, non-scripted agent economic action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MarketTxProvenance {
    System,
    ScriptedFixture,
    Missing,
    LiveAgentNonScripted {
        forced: bool,
        prompt_or_trace_linked: bool,
        chaintape_anchor: bool,
        audit_proceed: bool,
    },
}

/// TRACE_MATRIX FC3-N43 (REAL-11): classify market-facing tx kinds under the
/// structural-vs-agent-action claim boundary.
pub fn classify_market_tx(kind: &str, provenance: MarketTxProvenance) -> MarketTxCategory {
    let normalized = kind.to_ascii_lowercase();
    if normalized.contains("eventresolve")
        || normalized.contains("event_resolve")
        || normalized.contains("marketclose")
        || normalized.contains("market_close")
        || normalized.contains("oracleresolve")
        || normalized.contains("oracle_resolve")
    {
        return MarketTxCategory::ResolutionTx;
    }

    if normalized.contains("buywithcoinrouter")
        || normalized.contains("buy_with_coin_router")
        || normalized.contains("short")
        || normalized.contains("sell")
    {
        return match provenance {
            MarketTxProvenance::LiveAgentNonScripted {
                forced: false,
                prompt_or_trace_linked: true,
                chaintape_anchor: true,
                audit_proceed: true,
            } => MarketTxCategory::AgentEconomicActionTx,
            _ => MarketTxCategory::ScriptedFixtureTx,
        };
    }

    if matches!(provenance, MarketTxProvenance::ScriptedFixture) {
        return MarketTxCategory::ScriptedFixtureTx;
    }

    MarketTxCategory::StructuralMarketTx
}

/// TRACE_MATRIX FC3-N43 (REAL-11): per-REAL-8 arm market category counts used
/// to re-render REAL-10 evidence without E2 overclaim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArmMarketTxCategoryCounts {
    pub arm: &'static str,
    pub condition: &'static str,
    pub structural_market_tx_count: u64,
    pub agent_economic_action_tx_count: u64,
    pub scripted_fixture_tx_count: u64,
    pub resolution_tx_count: u64,
    pub buy_with_coin_router_count: u64,
}

impl ArmMarketTxCategoryCounts {
    /// REAL-10's reported `market_tx_count` tracked market-structure txs plus
    /// live agent economic action. It intentionally excludes fixture sidecars
    /// and resolution txs so comparisons to the clean REAL-10 table stay stable.
    /// TRACE_MATRIX FC3-N43 (REAL-11): stable total matching REAL-10's
    /// structural market count definition.
    pub fn market_tx_total(&self) -> u64 {
        self.structural_market_tx_count + self.agent_economic_action_tx_count
    }
}

/// TRACE_MATRIX FC3-N43 (REAL-11): split category table for the clean REAL-10
/// A/B/C/D evidence bundle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Real10MarketTxCategoryCounts {
    pub evidence_dir: &'static str,
    pub arms: Vec<ArmMarketTxCategoryCounts>,
}

impl Real10MarketTxCategoryCounts {
    /// TRACE_MATRIX FC3-N43 (REAL-11): lookup one A/B/C/D arm by label.
    pub fn arm(&self, arm: &str) -> Option<&ArmMarketTxCategoryCounts> {
        self.arms.iter().find(|row| row.arm == arm)
    }
}

/// TRACE_MATRIX FC3-N43 (REAL-11): re-render of the clean REAL-10 evidence at
/// `handover/evidence/real8x_market_ab_clean_20260515T141331Z`.
///
/// Source facts:
/// - `real8_arm_summary.tsv`: market_tx_count A/B/C/D = 0/10/42/38.
/// - `aggregate_verdict.json`: buy_with_coin_router = 0 in all arms.
/// - `audit_dashboard_run_report.txt`: D scripted_attempt_prediction_market_count = 15.
pub fn real10_category_counts() -> Real10MarketTxCategoryCounts {
    Real10MarketTxCategoryCounts {
        evidence_dir: "handover/evidence/real8x_market_ab_clean_20260515T141331Z",
        arms: vec![
            ArmMarketTxCategoryCounts {
                arm: "A",
                condition: "market disabled",
                structural_market_tx_count: 0,
                agent_economic_action_tx_count: 0,
                scripted_fixture_tx_count: 0,
                resolution_tx_count: 3,
                buy_with_coin_router_count: 0,
            },
            ArmMarketTxCategoryCounts {
                arm: "B",
                condition: "market visible, no TaskOutcomeMarket",
                structural_market_tx_count: 10,
                agent_economic_action_tx_count: 0,
                scripted_fixture_tx_count: 0,
                resolution_tx_count: 3,
                buy_with_coin_router_count: 0,
            },
            ArmMarketTxCategoryCounts {
                arm: "C",
                condition: "TaskOutcomeMarket enabled",
                structural_market_tx_count: 42,
                agent_economic_action_tx_count: 0,
                scripted_fixture_tx_count: 0,
                resolution_tx_count: 12,
                buy_with_coin_router_count: 0,
            },
            ArmMarketTxCategoryCounts {
                arm: "D",
                condition: "TaskOutcomeMarket + scripted AttemptPrediction fixture",
                structural_market_tx_count: 38,
                agent_economic_action_tx_count: 0,
                scripted_fixture_tx_count: 15,
                resolution_tx_count: 13,
                buy_with_coin_router_count: 0,
            },
        ],
    }
}
