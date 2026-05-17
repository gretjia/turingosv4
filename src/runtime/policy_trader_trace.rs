//! REAL-13 — deterministic PolicyTrader baseline sidecar.
//!
//! PolicyTraderTrace records a counterfactual policy comparison against an
//! EVDecisionTrace. It is Generic CAS evidence only and never contributes to
//! live E2 market-action counts.

use serde::{Deserialize, Serialize};

use crate::bottom_white::cas::schema::{Cid, ObjectType};
use crate::bottom_white::cas::store::{CasError, CasStore};

/// TRACE_MATRIX FC1/FC3: schema id for PolicyTrader baseline CAS records.
pub const POLICY_TRADER_TRACE_SCHEMA_ID: &str = "real13.policy_trader_trace.v1";

/// TRACE_MATRIX FC3: comparison label between deterministic policy and LLM
/// market behavior.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum PolicyTraderComparison {
    PolicyPositiveEV_LLMAbstained,
    PolicyNoPositiveEV,
    BothBuy,
    LLMBuyPolicyNoBuy,
    GatewayBlocked,
    InsufficientPublicEVBasis,
}

impl PolicyTraderComparison {
    /// TRACE_MATRIX FC3: exhaustive PolicyTrader-vs-LLM comparison taxonomy
    /// used by audit summaries without counting policy actions as E2.
    pub const fn all() -> [Self; 6] {
        [
            Self::PolicyPositiveEV_LLMAbstained,
            Self::PolicyNoPositiveEV,
            Self::BothBuy,
            Self::LLMBuyPolicyNoBuy,
            Self::GatewayBlocked,
            Self::InsufficientPublicEVBasis,
        ]
    }
}

/// TRACE_MATRIX FC1/FC3: counterfactual policy trace linked to source EV,
/// prompt capsule, and market snapshot CIDs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyTraderTrace {
    pub schema_version: String,
    pub source_ev_decision_trace_cid: Cid,
    pub prompt_capsule_cid: Cid,
    pub market_snapshot_cid: Cid,
    #[serde(default)]
    pub policy_probability_bps: Option<i64>,
    #[serde(default)]
    pub implied_probability_bps: Option<i64>,
    #[serde(default)]
    pub policy_edge_bps: Option<i64>,
    #[serde(default)]
    pub policy_expected_value_micro: Option<i128>,
    pub counterfactual_only: bool,
    pub counts_for_e2: bool,
    pub comparison: PolicyTraderComparison,
    pub gateway_blocked: bool,
    pub policy_public_summary: String,
}

impl PolicyTraderTrace {
    /// TRACE_MATRIX FC1/FC3: deterministic integer EV helper for callers
    /// that already derived policy probability, implied probability, and
    /// stake in micro-Coin.
    pub fn expected_value_micro(policy_edge_bps: i64, stake_micro: i64) -> i128 {
        (policy_edge_bps as i128 * stake_micro as i128) / 10_000
    }
}

/// TRACE_MATRIX FC1/FC3: validate shielding, integer-bps bounds, and the
/// counterfactual-only E2 exclusion before CAS persistence.
pub fn validate_policy_trader_trace(trace: &PolicyTraderTrace) -> Result<(), String> {
    if trace.schema_version != POLICY_TRADER_TRACE_SCHEMA_ID {
        return Err(format!(
            "unexpected PolicyTraderTrace schema_version={}",
            trace.schema_version
        ));
    }
    if let Some(value) = trace.policy_probability_bps {
        validate_bps("policy_probability_bps", value)?;
    }
    if let Some(value) = trace.implied_probability_bps {
        validate_bps("implied_probability_bps", value)?;
    }
    if let (Some(policy), Some(implied), Some(edge)) = (
        trace.policy_probability_bps,
        trace.implied_probability_bps,
        trace.policy_edge_bps,
    ) {
        let expected_edge_bps = policy - implied;
        if edge != expected_edge_bps {
            return Err("policy_edge_bps must equal policy minus implied bps".into());
        }
    }
    if !trace.counterfactual_only {
        return Err("PolicyTraderTrace must be counterfactual_only".into());
    }
    if trace.counts_for_e2 {
        return Err("PolicyTraderTrace must set counts_for_e2: false".into());
    }
    if trace.policy_public_summary.trim().is_empty() {
        return Err("PolicyTraderTrace policy_public_summary must be non-empty".into());
    }
    if contains_forbidden_private_material(&trace.policy_public_summary) {
        return Err("PolicyTraderTrace policy_public_summary contains private/raw material".into());
    }
    Ok(())
}

fn validate_bps(field: &str, value: i64) -> Result<(), String> {
    if value < 0 || value > 10_000 {
        return Err(format!("{field} must be integer bps in range"));
    }
    Ok(())
}

fn contains_forbidden_private_material(summary: &str) -> bool {
    let lower = summary.to_ascii_lowercase();
    [
        "raw_prompt",
        "raw prompt",
        "raw_completion",
        "raw completion",
        "private cot",
        "chain of thought",
        "raw_log",
        "raw log",
    ]
    .iter()
    .any(|needle| lower.contains(needle))
}

/// TRACE_MATRIX FC1/FC3: write validated PolicyTraderTrace to CAS using
/// ObjectType::Generic and a pinned schema id.
pub fn write_policy_trader_trace_to_cas(
    cas: &mut CasStore,
    trace: &PolicyTraderTrace,
    suffix: &str,
    logical_t: u64,
) -> Result<Cid, CasError> {
    validate_policy_trader_trace(trace)
        .map_err(|e| CasError::BackendCorruption(format!("PolicyTraderTrace invalid: {e}")))?;
    let bytes = serde_json::to_vec(trace)
        .map_err(|e| CasError::BackendCorruption(format!("PolicyTraderTrace encode: {e}")))?;
    cas.put(
        &bytes,
        ObjectType::Generic,
        &format!("real13-policy-trader-trace-{suffix}"),
        logical_t,
        Some(POLICY_TRADER_TRACE_SCHEMA_ID.to_string()),
    )
}

/// TRACE_MATRIX FC3: read and validate one PolicyTraderTrace from CAS.
pub fn read_policy_trader_trace_from_cas(
    cas: &CasStore,
    cid: &Cid,
) -> Result<PolicyTraderTrace, CasError> {
    let bytes = cas.get(cid)?;
    let trace: PolicyTraderTrace = serde_json::from_slice(&bytes)
        .map_err(|e| CasError::BackendCorruption(format!("PolicyTraderTrace decode: {e}")))?;
    validate_policy_trader_trace(&trace)
        .map_err(|e| CasError::BackendCorruption(format!("PolicyTraderTrace invalid: {e}")))?;
    Ok(trace)
}

/// TRACE_MATRIX FC3: list PolicyTraderTrace CIDs by CAS metadata schema id.
pub fn policy_trader_trace_cids(cas: &CasStore) -> Vec<Cid> {
    cas.list_all_cids()
        .into_iter()
        .filter(|cid| {
            cas.metadata(cid).and_then(|meta| meta.schema_id.as_deref())
                == Some(POLICY_TRADER_TRACE_SCHEMA_ID)
        })
        .collect()
}

/// TRACE_MATRIX FC3: read all PolicyTraderTrace objects from CAS in metadata
/// order for materialized views.
pub fn list_policy_trader_traces_from_cas(
    cas: &CasStore,
) -> Result<Vec<PolicyTraderTrace>, CasError> {
    policy_trader_trace_cids(cas)
        .into_iter()
        .map(|cid| read_policy_trader_trace_from_cas(cas, &cid))
        .collect()
}

/// TRACE_MATRIX FC3: CAS-derived summary for dashboard/report callers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyTraderTraceSummary {
    pub policy_trader_trace_total_cas: u64,
    pub policy_positive_ev_count: u64,
    pub policy_positive_ev_llm_abstained_count: u64,
    pub policy_no_positive_ev_count: u64,
    pub policy_insufficient_public_basis_count: u64,
    pub both_buy_count: u64,
    pub llm_buy_policy_no_buy_count: u64,
    pub gateway_blocked_count: u64,
    pub policy_counts_for_e2: bool,
}

impl PolicyTraderTraceSummary {
    /// TRACE_MATRIX FC3: fold Generic CAS traces into counts without reading
    /// stdout or live router counters. Renderers should include
    /// policy_counts_for_e2=false.
    pub fn from_cas(cas: &CasStore) -> Result<Self, CasError> {
        let mut summary = Self {
            policy_trader_trace_total_cas: 0,
            policy_positive_ev_count: 0,
            policy_positive_ev_llm_abstained_count: 0,
            policy_no_positive_ev_count: 0,
            policy_insufficient_public_basis_count: 0,
            both_buy_count: 0,
            llm_buy_policy_no_buy_count: 0,
            gateway_blocked_count: 0,
            policy_counts_for_e2: false,
        };
        for trace in list_policy_trader_traces_from_cas(cas)? {
            summary.policy_trader_trace_total_cas += 1;
            if trace.policy_edge_bps.map(|edge| edge > 0).unwrap_or(false) {
                summary.policy_positive_ev_count += 1;
            }
            let mut comparison_gateway_blocked = false;
            match trace.comparison {
                PolicyTraderComparison::PolicyPositiveEV_LLMAbstained => {
                    summary.policy_positive_ev_llm_abstained_count += 1;
                }
                PolicyTraderComparison::PolicyNoPositiveEV => {
                    summary.policy_no_positive_ev_count += 1;
                }
                PolicyTraderComparison::BothBuy => summary.both_buy_count += 1,
                PolicyTraderComparison::LLMBuyPolicyNoBuy => {
                    summary.llm_buy_policy_no_buy_count += 1;
                }
                PolicyTraderComparison::GatewayBlocked => {
                    comparison_gateway_blocked = true;
                }
                PolicyTraderComparison::InsufficientPublicEVBasis => {
                    summary.policy_insufficient_public_basis_count += 1;
                    comparison_gateway_blocked = true;
                }
            }
            if trace.gateway_blocked || comparison_gateway_blocked {
                summary.gateway_blocked_count += 1;
            }
        }
        Ok(summary)
    }
}
