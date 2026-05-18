//! REAL-14 — independent E2 candidate verifier.
//!
//! This helper recomputes live agent economic action from ChainTape/CAS by
//! exact tx-id join. It intentionally does not read dashboard text.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::bottom_white::cas::schema::Cid;
use crate::bottom_white::cas::store::CasStore;
use crate::bottom_white::ledger::transition_ledger::{
    canonical_decode, Git2LedgerWriter, LedgerWriter, TxKind,
};
use crate::economy::money::MicroCoin;
use crate::runtime::ev_decision_trace::{
    ev_decision_trace_cids, read_ev_decision_trace_from_cas, EVAction, EVDecisionTrace,
};
use crate::runtime::librarian_broadcast::{
    assert_no_forbidden_broadcast_material, librarian_digest_cids, read_librarian_digest_from_cas,
    RoleNotificationView, LIBRARIAN_ROLE_CROP_SCHEMA_ID,
};
use crate::runtime::market_decision_provenance_link::{
    list_market_decision_provenance_links_from_cas, MarketDecisionProvenanceLink,
};
use crate::runtime::market_decision_trace::{MarketDecisionTrace, TraceOutcome};
use crate::runtime::market_opportunity_trace::{
    market_opportunity_trace_cids, MarketOpportunityTrace,
};
use crate::runtime::policy_trader_trace::PolicyTraderTraceSummary;
use crate::runtime::prompt_capsule::read_prompt_capsule_v2_from_cas;
use crate::runtime::real5_roles::{
    is_trader_like, read_role_turn_trace_from_cas, role_turn_trace_cids, RoleTurnTrace,
};
use crate::runtime::real6_attempt_prediction::attempt_prediction_fixture_cids;
use crate::state::q_state::{AgentId, TxId};
use crate::state::typed_tx::{BuyDirection, EventId, TypedTx};

/// TRACE_MATRIX FC1/FC3: verifier runtime options for exact-join evidence
/// replay; controls expected count checks without changing ChainTape/CAS.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct E2CandidateVerifierOptions {
    pub expected_exact_join_count: Option<u64>,
    pub require_matched_tx_provenance: bool,
    pub require_direct_prompt_capsule_provenance: bool,
}

impl Default for E2CandidateVerifierOptions {
    fn default() -> Self {
        Self {
            expected_exact_join_count: None,
            require_matched_tx_provenance: true,
            require_direct_prompt_capsule_provenance: false,
        }
    }
}

/// TRACE_MATRIX FC3: clean verifier outcome for research evidence labeling;
/// does not authorize E2 achieved claims.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum E2CandidateVerifierVerdict {
    Proceed,
    Veto,
}

/// TRACE_MATRIX FC1/FC3: materialized verifier report derived from ChainTape
/// and CAS exact joins, not dashboard text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct E2CandidateVerifierReport {
    pub schema_version: String,
    pub l4_router_tx_count: u64,
    pub submitted_trace_tx_count: u64,
    pub exact_join_count: u64,
    pub matched_tx_ids: Vec<String>,
    pub unmatched_l4_router_tx_ids: Vec<String>,
    pub unmatched_submitted_trace_tx_ids: Vec<String>,
    pub duplicate_l4_router_tx_id_count: u64,
    pub duplicate_submitted_trace_tx_id_count: u64,
    pub scripted_fixture_tx_count: u64,
    pub policy_counts_for_e2: bool,
    pub direct_prompt_capsule_provenance_count: u64,
    pub indirect_prompt_capsule_provenance_count: u64,
    pub missing_direct_prompt_capsule_provenance_count: u64,
    pub matched_tx_provenance: Vec<MatchedTxProvenanceReport>,
    pub bcast_shielding: BcastShieldingReport,
    pub verdict: E2CandidateVerifierVerdict,
    pub failure_reasons: Vec<String>,
}

impl E2CandidateVerifierReport {
    /// TRACE_MATRIX FC3: schema id for REAL-14 exact-join verifier reports.
    pub const SCHEMA_VERSION: &'static str = "real14.e2_candidate_verifier.v1";

    /// TRACE_MATRIX FC3: render bounded human report text from verifier facts
    /// while preserving zero-join clean-negative claim boundaries.
    pub fn render_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("# REAL-14 E2 Candidate Verifier Report\n\n");
        out.push_str(&format!(
            "claim_boundary: {}\n\n",
            self.markdown_claim_boundary()
        ));
        out.push_str(&format!("verdict: {:?}\n", self.verdict));
        out.push_str(&format!(
            "l4_router_tx_count: {}\n",
            self.l4_router_tx_count
        ));
        out.push_str(&format!(
            "submitted_trace_tx_count: {}\n",
            self.submitted_trace_tx_count
        ));
        out.push_str(&format!("exact_join_count: {}\n", self.exact_join_count));
        out.push_str(&format!(
            "duplicate_l4_router_tx_id_count: {}\n",
            self.duplicate_l4_router_tx_id_count
        ));
        out.push_str(&format!(
            "duplicate_submitted_trace_tx_id_count: {}\n",
            self.duplicate_submitted_trace_tx_id_count
        ));
        out.push_str(&format!(
            "scripted_fixture_tx_count: {}\n",
            self.scripted_fixture_tx_count
        ));
        out.push_str(&format!(
            "policy_counts_for_e2: {}\n",
            self.policy_counts_for_e2
        ));
        out.push_str(&format!(
            "direct_prompt_capsule_provenance_count: {}\n",
            self.direct_prompt_capsule_provenance_count
        ));
        out.push_str(&format!(
            "indirect_prompt_capsule_provenance_count: {}\n",
            self.indirect_prompt_capsule_provenance_count
        ));
        out.push_str(&format!(
            "missing_direct_prompt_capsule_provenance_count: {}\n",
            self.missing_direct_prompt_capsule_provenance_count
        ));
        out.push_str("\nmatched_tx_ids:\n");
        for tx_id in &self.matched_tx_ids {
            out.push_str(&format!("- {tx_id}\n"));
        }
        out.push_str("\nmatched_tx_provenance:\n");
        for row in &self.matched_tx_provenance {
            out.push_str(&format!(
                "- tx_id={} actor={} role={:?} ev={} opportunity={} prompt_link={} role_turn={} residual_risks={}\n",
                row.tx_id,
                row.market_decision_agent_id
                    .as_deref()
                    .unwrap_or("<missing>"),
                row.live_agent_role,
                row.ev_decision_trace_count,
                row.market_opportunity_trace_count,
                row.prompt_capsule_linkage,
                row.role_turn_trace_count,
                row.residual_risks.len()
            ));
        }
        out.push_str("\nbcast_shielding:\n");
        out.push_str(&format!(
            "  verdict: {}\n  digest_count: {}\n  role_crop_count: {}\n  visible_context_count: {}\n  failure_count: {}\n",
            self.bcast_shielding.verdict,
            self.bcast_shielding.librarian_digest_count,
            self.bcast_shielding.librarian_role_crop_count,
            self.bcast_shielding.visible_context_count,
            self.bcast_shielding.failure_reasons.len()
        ));
        if !self.failure_reasons.is_empty() {
            out.push_str("\nfailure_reasons:\n");
            for reason in &self.failure_reasons {
                out.push_str(&format!("- {reason}\n"));
            }
        }
        out
    }

    fn markdown_claim_boundary(&self) -> &'static str {
        match (self.verdict, self.exact_join_count) {
            (E2CandidateVerifierVerdict::Proceed, 1..) => "E2 candidate pending audit",
            (E2CandidateVerifierVerdict::Proceed, 0) => {
                "clean-negative: no E2 candidate in this run"
            }
            (E2CandidateVerifierVerdict::Veto, _) => "not an E2 candidate",
        }
    }
}

/// TRACE_MATRIX FC1/FC3: per-matched-router provenance row joining L4 router,
/// submitted market decision, EV, market opportunity, prompt, and role traces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatchedTxProvenanceReport {
    pub tx_id: String,
    pub l4_buyer: String,
    pub l4_event_id: String,
    pub l4_direction: String,
    pub l4_pay_coin_micro: i64,
    pub l4_min_out_shares: u128,
    pub market_decision_trace_count: u64,
    pub market_decision_trace_cids: Vec<String>,
    pub market_decision_provenance_link_cids: Vec<String>,
    pub market_decision_agent_id: Option<String>,
    pub market_decision_chosen_node_id: Option<String>,
    pub market_decision_direction: Option<String>,
    pub market_decision_amount_micro: Option<i64>,
    pub ev_decision_trace_count: u64,
    pub ev_decision_trace_cids: Vec<String>,
    pub ev_actions: Vec<String>,
    pub ev_reasons: Vec<String>,
    pub market_opportunity_trace_count: u64,
    pub market_opportunity_trace_cids: Vec<String>,
    pub prompt_capsule_linkage: String,
    pub prompt_capsule_cids: Vec<String>,
    pub role_turn_trace_count: u64,
    pub role_turn_trace_cids: Vec<String>,
    pub live_agent_role: Option<String>,
    pub actor_is_policy_trader: bool,
    pub actor_is_live_agent_role: bool,
    pub residual_risks: Vec<String>,
}

/// TRACE_MATRIX FC3: BCAST shielding scan result over CAS-derived digests,
/// role crops, and visible contexts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BcastShieldingReport {
    pub librarian_digest_count: u64,
    pub librarian_role_crop_count: u64,
    pub visible_context_count: u64,
    pub verdict: String,
    pub failure_reasons: Vec<String>,
}

#[derive(Debug, Clone)]
struct L4RouterWitness {
    buyer: AgentId,
    event_id: EventId,
    direction: BuyDirection,
    pay_coin: MicroCoin,
    min_out_shares: u128,
}

#[derive(Debug, Clone)]
struct SubmittedMarketDecisionWitness {
    trace_cid: Cid,
    trace: MarketDecisionTrace,
}

#[derive(Debug, Clone)]
struct EVDecisionWitness {
    trace_cid: Cid,
    trace: EVDecisionTrace,
}

#[derive(Debug, Clone)]
struct MarketOpportunityWitness {
    trace_cid: Cid,
    trace: MarketOpportunityTrace,
}

#[derive(Debug, Clone)]
struct RoleTurnWitness {
    trace_cid: Cid,
    trace: RoleTurnTrace,
}

#[derive(Debug, Clone)]
struct MarketDecisionProvenanceLinkWitness {
    link_cid: Cid,
    link: MarketDecisionProvenanceLink,
}

/// TRACE_MATRIX FC1/FC3: recompute live agent economic-action candidates from
/// L4 BuyWithCoinRouterTx ids intersected with submitted MarketDecisionTrace
/// ids, failing closed on provenance, fixture, policy, or shielding defects.
pub fn verify_market_e2_candidate(
    repo_path: &Path,
    cas_path: &Path,
    options: E2CandidateVerifierOptions,
) -> Result<E2CandidateVerifierReport, String> {
    let cas = CasStore::open(cas_path).map_err(|e| format!("CAS open failed: {e}"))?;
    let router_l4_tx_id_witnesses = collect_l4_router_tx_id_witnesses(repo_path, &cas)?;
    let (submitted_market_decision_router_tx_ids, unknown_schema_failures) =
        collect_submitted_market_decision_router_tx_ids(&cas)?;

    let router_l4_tx_ids: BTreeSet<String> = router_l4_tx_id_witnesses.keys().cloned().collect();
    let submitted_market_decision_router_tx_id_set: BTreeSet<String> =
        submitted_market_decision_router_tx_ids
            .keys()
            .cloned()
            .collect();
    let matched_tx_ids: Vec<String> = router_l4_tx_ids
        .intersection(&submitted_market_decision_router_tx_id_set)
        .cloned()
        .collect();
    let unmatched_l4_router_tx_ids: Vec<String> = router_l4_tx_ids
        .difference(&submitted_market_decision_router_tx_id_set)
        .cloned()
        .collect();
    let unmatched_submitted_trace_tx_ids: Vec<String> = submitted_market_decision_router_tx_id_set
        .difference(&router_l4_tx_ids)
        .cloned()
        .collect();
    let duplicate_l4_router_tx_id_count = duplicate_count(&router_l4_tx_id_witnesses);
    let duplicate_submitted_trace_tx_id_count =
        duplicate_count(&submitted_market_decision_router_tx_ids);
    let scripted_fixture_tx_count = attempt_prediction_fixture_cids(&cas).len() as u64;
    let policy_counts_for_e2 = PolicyTraderTraceSummary::from_cas(&cas)
        .map_err(|e| format!("PolicyTraderTrace summary failed: {e}"))?
        .policy_counts_for_e2;
    let ev_traces = collect_ev_decision_witnesses(&cas)?;
    let opportunity_traces = collect_market_opportunity_witnesses(&cas)?;
    let role_turn_traces = collect_role_turn_witnesses(&cas)?;
    let provenance_links = collect_market_decision_provenance_link_witnesses(&cas)?;
    let matched_tx_provenance = build_matched_tx_provenance(
        &cas,
        &matched_tx_ids,
        &router_l4_tx_id_witnesses,
        &submitted_market_decision_router_tx_ids,
        &ev_traces,
        &opportunity_traces,
        &role_turn_traces,
        &provenance_links,
    )?;
    let bcast_shielding = verify_bcast_shielding(&cas)?;

    let exact_join_count = matched_tx_ids.len() as u64;
    let direct_prompt_capsule_provenance_count = matched_tx_provenance
        .iter()
        .filter(|row| row.prompt_capsule_linkage == "direct_via_market_decision_provenance_link")
        .count() as u64;
    let indirect_prompt_capsule_provenance_count = matched_tx_provenance
        .iter()
        .filter(|row| row.prompt_capsule_linkage == "indirect_via_ev_decision_trace")
        .count() as u64;
    let missing_direct_prompt_capsule_provenance_count =
        exact_join_count.saturating_sub(direct_prompt_capsule_provenance_count);
    let mut failure_reasons = unknown_schema_failures;
    if let Some(expected) = options.expected_exact_join_count {
        if exact_join_count != expected {
            failure_reasons.push(format!(
                "exact_join_count mismatch: expected {expected}, got {exact_join_count}"
            ));
        }
    }
    if duplicate_l4_router_tx_id_count > 0 {
        failure_reasons.push(format!(
            "duplicate L4 router tx_id count = {duplicate_l4_router_tx_id_count}"
        ));
    }
    if duplicate_submitted_trace_tx_id_count > 0 {
        failure_reasons.push(format!(
            "duplicate submitted MarketDecisionTrace tx_id count = {duplicate_submitted_trace_tx_id_count}"
        ));
    }
    if scripted_fixture_tx_count > 0 {
        failure_reasons.push(format!(
            "scripted fixture CAS records present = {scripted_fixture_tx_count}"
        ));
    }
    if policy_counts_for_e2 {
        failure_reasons.push("PolicyTrader summary unexpectedly counts for E2".to_string());
    }
    for row in &matched_tx_provenance {
        if row.market_decision_trace_count == 1 {
            if row
                .market_decision_agent_id
                .as_deref()
                .map(|agent_id| agent_id != row.l4_buyer)
                .unwrap_or(true)
            {
                failure_reasons.push(format!(
                    "matched tx {} actor mismatch: L4 buyer={} MarketDecisionTrace agent={}",
                    row.tx_id,
                    row.l4_buyer,
                    row.market_decision_agent_id
                        .as_deref()
                        .unwrap_or("<missing>")
                ));
            }
            if row
                .market_decision_direction
                .as_deref()
                .map(|direction| direction != row.l4_direction)
                .unwrap_or(true)
            {
                failure_reasons.push(format!(
                    "matched tx {} direction mismatch: L4 direction={} MarketDecisionTrace direction={}",
                    row.tx_id,
                    row.l4_direction,
                    row.market_decision_direction
                        .as_deref()
                        .unwrap_or("<missing>")
                ));
            }
            if row
                .market_decision_amount_micro
                .map(|amount| amount != row.l4_pay_coin_micro)
                .unwrap_or(true)
            {
                failure_reasons.push(format!(
                    "matched tx {} amount mismatch: L4 pay_coin_micro={} MarketDecisionTrace amount_micro={}",
                    row.tx_id,
                    row.l4_pay_coin_micro,
                    row.market_decision_amount_micro
                        .map(|amount| amount.to_string())
                        .unwrap_or_else(|| "<missing>".to_string())
                ));
            }
        }
    }
    if options.require_matched_tx_provenance {
        for row in &matched_tx_provenance {
            if row.market_decision_trace_count != 1 {
                failure_reasons.push(format!(
                    "matched tx {} has MarketDecisionTrace count {}",
                    row.tx_id, row.market_decision_trace_count
                ));
            }
            if row.ev_decision_trace_count == 0 {
                failure_reasons.push(format!(
                    "matched tx {} has no EVDecisionTrace/economic rationale",
                    row.tx_id
                ));
            }
            if row.market_opportunity_trace_count == 0 {
                failure_reasons.push(format!(
                    "matched tx {} has no MarketOpportunityTrace",
                    row.tx_id
                ));
            }
            if row.prompt_capsule_linkage == "missing" {
                failure_reasons.push(format!(
                    "matched tx {} has no PromptCapsule/visible-context linkage",
                    row.tx_id
                ));
            }
            if !row.actor_is_live_agent_role {
                failure_reasons.push(format!(
                    "matched tx {} actor is not a live trader-like agent role",
                    row.tx_id
                ));
            }
            if row.actor_is_policy_trader {
                failure_reasons.push(format!(
                    "matched tx {} actor is PolicyTrader, not live agent",
                    row.tx_id
                ));
            }
        }
    }
    if options.require_direct_prompt_capsule_provenance {
        for row in &matched_tx_provenance {
            if row.prompt_capsule_linkage != "direct_via_market_decision_provenance_link" {
                failure_reasons.push(format!(
                    "matched tx {} missing direct PromptCapsule provenance",
                    row.tx_id
                ));
            }
        }
    }
    failure_reasons.extend(
        bcast_shielding
            .failure_reasons
            .iter()
            .map(|reason| format!("BCAST shielding failure: {reason}")),
    );
    let verdict = if failure_reasons.is_empty() {
        E2CandidateVerifierVerdict::Proceed
    } else {
        E2CandidateVerifierVerdict::Veto
    };

    Ok(E2CandidateVerifierReport {
        schema_version: E2CandidateVerifierReport::SCHEMA_VERSION.to_string(),
        l4_router_tx_count: router_l4_tx_ids.len() as u64,
        submitted_trace_tx_count: submitted_market_decision_router_tx_id_set.len() as u64,
        exact_join_count,
        matched_tx_ids,
        unmatched_l4_router_tx_ids,
        unmatched_submitted_trace_tx_ids,
        duplicate_l4_router_tx_id_count,
        duplicate_submitted_trace_tx_id_count,
        scripted_fixture_tx_count,
        policy_counts_for_e2,
        direct_prompt_capsule_provenance_count,
        indirect_prompt_capsule_provenance_count,
        missing_direct_prompt_capsule_provenance_count,
        matched_tx_provenance,
        bcast_shielding,
        verdict,
        failure_reasons,
    })
}

fn collect_l4_router_tx_id_witnesses(
    repo_path: &Path,
    cas: &CasStore,
) -> Result<BTreeMap<String, Vec<L4RouterWitness>>, String> {
    let writer = Git2LedgerWriter::open(repo_path).map_err(|e| format!("open L4 ledger: {e:?}"))?;
    let mut counts = BTreeMap::new();
    for logical_t in 1..=writer.len() {
        let entry = writer
            .read_at(logical_t)
            .map_err(|e| format!("read L4 logical_t={logical_t}: {e:?}"))?;
        if entry.tx_kind != TxKind::BuyWithCoinRouter {
            continue;
        }
        let payload_bytes = cas
            .get(&entry.tx_payload_cid)
            .map_err(|e| format!("read TypedTx payload from CAS at t={logical_t}: {e}"))?;
        let typed_tx: TypedTx = canonical_decode(&payload_bytes)
            .map_err(|e| format!("decode TypedTx at t={logical_t}: {e}"))?;
        match typed_tx {
            TypedTx::BuyWithCoinRouter(router) => {
                counts
                    .entry(router.tx_id.0)
                    .or_insert_with(Vec::new)
                    .push(L4RouterWitness {
                        buyer: router.buyer,
                        event_id: router.event_id,
                        direction: router.direction,
                        pay_coin: router.pay_coin,
                        min_out_shares: router.min_out_shares.units,
                    });
            }
            other => {
                return Err(format!(
                    "L4 entry t={logical_t} has TxKind::BuyWithCoinRouter but payload is {:?}",
                    other.tx_kind()
                ));
            }
        }
    }
    Ok(counts)
}

fn collect_submitted_market_decision_router_tx_ids(
    cas: &CasStore,
) -> Result<
    (
        BTreeMap<String, Vec<SubmittedMarketDecisionWitness>>,
        Vec<String>,
    ),
    String,
> {
    let mut counts = BTreeMap::new();
    let mut failures = Vec::new();
    for cid in cas.list_all_cids() {
        let bytes = match cas.get(&cid) {
            Ok(bytes) => bytes,
            Err(e) => return Err(format!("CAS read failed for market decision scan: {e}")),
        };
        let value: serde_json::Value = match serde_json::from_slice(&bytes) {
            Ok(value) => value,
            Err(_) => continue,
        };
        let Some(schema_version) = value.get("schema_version").and_then(|v| v.as_str()) else {
            continue;
        };
        if schema_version != MarketDecisionTrace::SCHEMA_VERSION {
            if schema_version.starts_with("tb_n3.market_decision_trace") {
                failures.push(format!(
                    "unknown MarketDecisionTrace schema: {schema_version}"
                ));
            }
            continue;
        }
        let trace: MarketDecisionTrace = serde_json::from_value(value)
            .map_err(|e| format!("MarketDecisionTrace decode failed: {e}"))?;
        if let TraceOutcome::Submitted { tx_id } = &trace.outcome {
            counts.entry(tx_id.0.clone()).or_insert_with(Vec::new).push(
                SubmittedMarketDecisionWitness {
                    trace_cid: cid,
                    trace,
                },
            );
        }
    }
    Ok((counts, failures))
}

fn collect_ev_decision_witnesses(cas: &CasStore) -> Result<Vec<EVDecisionWitness>, String> {
    let mut out = Vec::new();
    for cid in ev_decision_trace_cids(cas) {
        let trace = read_ev_decision_trace_from_cas(cas, &cid)
            .map_err(|e| format!("EVDecisionTrace read failed for {cid}: {e}"))?;
        out.push(EVDecisionWitness {
            trace_cid: cid,
            trace,
        });
    }
    Ok(out)
}

fn collect_market_opportunity_witnesses(
    cas: &CasStore,
) -> Result<Vec<MarketOpportunityWitness>, String> {
    let mut out = Vec::new();
    for cid in market_opportunity_trace_cids(cas) {
        let bytes = cas
            .get(&cid)
            .map_err(|e| format!("MarketOpportunityTrace read failed for {cid}: {e}"))?;
        let trace: MarketOpportunityTrace = serde_json::from_slice(&bytes)
            .map_err(|e| format!("MarketOpportunityTrace decode failed for {cid}: {e}"))?;
        out.push(MarketOpportunityWitness {
            trace_cid: cid,
            trace,
        });
    }
    Ok(out)
}

fn collect_role_turn_witnesses(cas: &CasStore) -> Result<Vec<RoleTurnWitness>, String> {
    let mut out = Vec::new();
    for cid in role_turn_trace_cids(cas) {
        let trace = read_role_turn_trace_from_cas(cas, &cid)
            .map_err(|e| format!("RoleTurnTrace read failed for {cid}: {e}"))?;
        out.push(RoleTurnWitness {
            trace_cid: cid,
            trace,
        });
    }
    Ok(out)
}

fn collect_market_decision_provenance_link_witnesses(
    cas: &CasStore,
) -> Result<BTreeMap<String, Vec<MarketDecisionProvenanceLinkWitness>>, String> {
    let mut out: BTreeMap<String, Vec<MarketDecisionProvenanceLinkWitness>> = BTreeMap::new();
    for (cid, link) in list_market_decision_provenance_links_from_cas(cas)
        .map_err(|e| format!("MarketDecisionProvenanceLink read failed: {e}"))?
    {
        out.entry(link.submitted_router_tx_id.0.clone())
            .or_default()
            .push(MarketDecisionProvenanceLinkWitness {
                link_cid: cid,
                link,
            });
    }
    Ok(out)
}

fn build_matched_tx_provenance(
    cas: &CasStore,
    matched_tx_ids: &[String],
    router_l4_tx_id_witnesses: &BTreeMap<String, Vec<L4RouterWitness>>,
    submitted_market_decision_router_tx_ids: &BTreeMap<String, Vec<SubmittedMarketDecisionWitness>>,
    ev_traces: &[EVDecisionWitness],
    opportunity_traces: &[MarketOpportunityWitness],
    role_turn_traces: &[RoleTurnWitness],
    provenance_links: &BTreeMap<String, Vec<MarketDecisionProvenanceLinkWitness>>,
) -> Result<Vec<MatchedTxProvenanceReport>, String> {
    let mut rows = Vec::new();
    for tx_id in matched_tx_ids {
        let l4_witnesses = router_l4_tx_id_witnesses
            .get(tx_id)
            .ok_or_else(|| format!("matched tx {tx_id} missing L4 witness"))?;
        let decision_witnesses = submitted_market_decision_router_tx_ids
            .get(tx_id)
            .ok_or_else(|| format!("matched tx {tx_id} missing MarketDecisionTrace witness"))?;
        let l4 = &l4_witnesses[0];
        let decision = decision_witnesses.first();
        let decision_trace = decision.map(|witness| &witness.trace);
        let decision_agent_id = decision_trace.map(|trace| trace.agent_id.clone());
        let chosen_node_id = decision_trace
            .and_then(|trace| trace.chosen_node_id.clone())
            .or_else(|| Some(TxId(l4.event_id.0 .0.clone())));
        let direction = decision_trace.and_then(|trace| trace.direction);
        let amount_micro = decision_trace.and_then(|trace| trace.amount_micro);
        let direct_link_matches = provenance_links.get(tx_id).cloned().unwrap_or_default();
        let direct_prompt_capsule_cids: BTreeSet<Cid> = direct_link_matches
            .iter()
            .map(|witness| witness.link.prompt_capsule_cid)
            .collect();
        let ev_matches: Vec<&EVDecisionWitness> = ev_traces
            .iter()
            .filter(|witness| {
                decision_agent_id
                    .as_ref()
                    .map(|agent_id| witness.trace.agent_id == *agent_id)
                    .unwrap_or(false)
                    && chosen_node_id
                        .as_ref()
                        .map(|node| witness.trace.event_id.0 .0 == node.0)
                        .unwrap_or(false)
                    && ev_action_matches_direction(witness.trace.action, direction)
            })
            .collect();
        let prompt_capsule_cids: BTreeSet<Cid> = ev_matches
            .iter()
            .map(|witness| witness.trace.prompt_capsule_cid)
            .collect();
        let matched_prompt_capsule_cids: BTreeSet<Cid> = if direct_prompt_capsule_cids.is_empty() {
            prompt_capsule_cids.clone()
        } else {
            direct_prompt_capsule_cids.clone()
        };
        let opportunity_matches: Vec<&MarketOpportunityWitness> = opportunity_traces
            .iter()
            .filter(|witness| {
                decision_agent_id
                    .as_ref()
                    .map(|agent_id| witness.trace.agent_id == *agent_id)
                    .unwrap_or(false)
                    && chosen_node_id
                        .as_ref()
                        .map(|node| {
                            witness
                                .trace
                                .actionable_markets
                                .iter()
                                .any(|event_id| event_id.0 .0 == node.0)
                        })
                        .unwrap_or(false)
                    && matched_prompt_capsule_cids
                        .iter()
                        .any(|cid| witness.trace.prompt_capsule_cid == Some(*cid))
            })
            .collect();
        let role_turn_matches: Vec<&RoleTurnWitness> = role_turn_traces
            .iter()
            .filter(|witness| {
                decision_agent_id
                    .as_ref()
                    .map(|agent_id| witness.trace.agent_id == *agent_id)
                    .unwrap_or(false)
                    && matched_prompt_capsule_cids.contains(&witness.trace.prompt_capsule_cid)
            })
            .collect();
        let direct_prompt_capsule_cids_decodable = direct_prompt_capsule_cids
            .iter()
            .filter(|cid| read_prompt_capsule_v2_from_cas(cas, cid).is_ok())
            .count();
        let prompt_capsule_cids_decodable = prompt_capsule_cids
            .iter()
            .filter(|cid| read_prompt_capsule_v2_from_cas(cas, cid).is_ok())
            .count();
        let prompt_capsule_linkage = if !direct_prompt_capsule_cids.is_empty()
            && direct_prompt_capsule_cids_decodable == direct_prompt_capsule_cids.len()
        {
            "direct_via_market_decision_provenance_link"
        } else if !direct_prompt_capsule_cids.is_empty() {
            "direct_market_decision_provenance_link_decode_failed"
        } else if prompt_capsule_cids.is_empty() {
            "missing"
        } else if prompt_capsule_cids_decodable == prompt_capsule_cids.len() {
            "indirect_via_ev_decision_trace"
        } else {
            "cid_present_decode_failed"
        }
        .to_string();
        let live_agent_role = role_turn_matches
            .iter()
            .find_map(|witness| {
                if is_trader_like(witness.trace.role) {
                    Some(witness.trace.role.label().to_string())
                } else {
                    None
                }
            })
            .or_else(|| {
                ev_matches
                    .first()
                    .map(|witness| witness.trace.role.label().to_string())
            });
        let actor_is_live_agent_role = live_agent_role
            .as_deref()
            .map(|role| matches!(role, "Trader" | "BullTrader" | "BearTrader"))
            .unwrap_or(false);
        let actor_is_policy_trader = decision_agent_id
            .as_ref()
            .map(|agent_id| agent_id.0.to_ascii_lowercase().contains("policy"))
            .unwrap_or(false);
        let mut residual_risks = Vec::new();
        if direct_link_matches.len() > 1 {
            residual_risks.push(format!(
                "multiple MarketDecisionProvenanceLink sidecars match router tx_id (count={})",
                direct_link_matches.len()
            ));
        }
        if prompt_capsule_linkage == "direct_market_decision_provenance_link_decode_failed" {
            residual_risks.push(
                "MarketDecisionProvenanceLink prompt capsule CID did not decode as PromptCapsuleV2"
                    .to_string(),
            );
        }
        if prompt_capsule_linkage == "indirect_via_ev_decision_trace" {
            residual_risks.push(
                "MarketDecisionTrace has no direct PromptCapsule field; linkage is via matched EVDecisionTrace"
                    .to_string(),
            );
        }
        if ev_matches.len() > 1 {
            residual_risks.push(format!(
                "multiple EVDecisionTrace rows match agent/event/action; tx_id is disambiguated only by exact router join (count={})",
                ev_matches.len()
            ));
        }
        rows.push(MatchedTxProvenanceReport {
            tx_id: tx_id.clone(),
            l4_buyer: l4.buyer.0.clone(),
            l4_event_id: l4.event_id.0 .0.clone(),
            l4_direction: format!("{:?}", l4.direction),
            l4_pay_coin_micro: l4.pay_coin.micro_units(),
            l4_min_out_shares: l4.min_out_shares,
            market_decision_trace_count: decision_witnesses.len() as u64,
            market_decision_trace_cids: decision_witnesses
                .iter()
                .map(|witness| witness.trace_cid.to_string())
                .collect(),
            market_decision_provenance_link_cids: direct_link_matches
                .iter()
                .map(|witness| witness.link_cid.to_string())
                .collect(),
            market_decision_agent_id: decision_trace.map(|trace| trace.agent_id.0.clone()),
            market_decision_chosen_node_id: decision_trace
                .and_then(|trace| trace.chosen_node_id.as_ref().map(|tx_id| tx_id.0.clone())),
            market_decision_direction: decision_trace
                .and_then(|trace| trace.direction.map(|direction| format!("{direction:?}"))),
            market_decision_amount_micro: amount_micro,
            ev_decision_trace_count: ev_matches.len() as u64,
            ev_decision_trace_cids: ev_matches
                .iter()
                .map(|witness| witness.trace_cid.to_string())
                .collect(),
            ev_actions: ev_matches
                .iter()
                .map(|witness| format!("{:?}", witness.trace.action))
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect(),
            ev_reasons: ev_matches
                .iter()
                .map(|witness| format!("{:?}", witness.trace.reason))
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect(),
            market_opportunity_trace_count: opportunity_matches.len() as u64,
            market_opportunity_trace_cids: opportunity_matches
                .iter()
                .map(|witness| witness.trace_cid.to_string())
                .collect(),
            prompt_capsule_linkage,
            prompt_capsule_cids: matched_prompt_capsule_cids
                .iter()
                .map(|cid| cid.to_string())
                .collect(),
            role_turn_trace_count: role_turn_matches.len() as u64,
            role_turn_trace_cids: role_turn_matches
                .iter()
                .map(|witness| witness.trace_cid.to_string())
                .collect(),
            live_agent_role,
            actor_is_policy_trader,
            actor_is_live_agent_role,
            residual_risks,
        });
    }
    Ok(rows)
}

fn ev_action_matches_direction(action: EVAction, direction: Option<BuyDirection>) -> bool {
    matches!(
        (action, direction),
        (EVAction::BuyYes, Some(BuyDirection::BuyYes))
            | (EVAction::BuyNo, Some(BuyDirection::BuyNo))
    )
}

fn verify_bcast_shielding(cas: &CasStore) -> Result<BcastShieldingReport, String> {
    let mut failures = Vec::new();
    let digest_cids = librarian_digest_cids(cas);
    for cid in &digest_cids {
        match read_librarian_digest_from_cas(cas, cid) {
            Ok(digest) => match serde_json::to_string(&digest) {
                Ok(text) => {
                    if let Err(e) = assert_no_forbidden_broadcast_material(&text) {
                        failures.push(format!("LibrarianDigest {cid}: {e}"));
                    }
                }
                Err(e) => failures.push(format!("LibrarianDigest {cid} JSON encode: {e}")),
            },
            Err(e) => failures.push(format!("LibrarianDigest {cid} read/validate: {e}")),
        }
    }
    let mut role_crop_count = 0u64;
    let mut visible_context_count = 0u64;
    for cid in cas.list_all_cids() {
        let Some(schema_id) = cas.metadata(&cid).and_then(|meta| meta.schema_id.clone()) else {
            continue;
        };
        if schema_id == LIBRARIAN_ROLE_CROP_SCHEMA_ID {
            role_crop_count += 1;
            let bytes = cas
                .get(&cid)
                .map_err(|e| format!("RoleNotificationView {cid} read failed: {e}"))?;
            match serde_json::from_slice::<RoleNotificationView>(&bytes) {
                Ok(view) => {
                    if let Err(e) = assert_no_forbidden_broadcast_material(&view.rendered_notice) {
                        failures.push(format!("RoleNotificationView {cid}: {e}"));
                    }
                }
                Err(e) => failures.push(format!("RoleNotificationView {cid} decode: {e}")),
            }
        } else if schema_id == "real5.prompt.visible_context.v1" {
            visible_context_count += 1;
            let bytes = cas
                .get(&cid)
                .map_err(|e| format!("visible context {cid} read failed: {e}"))?;
            let text = String::from_utf8_lossy(&bytes);
            if let Err(e) = assert_no_forbidden_broadcast_material(&text) {
                failures.push(format!("visible context {cid}: {e}"));
            }
        } else if schema_id.starts_with("turingosv4.librarian_")
            && schema_id != LIBRARIAN_ROLE_CROP_SCHEMA_ID
            && schema_id != crate::runtime::librarian_broadcast::LIBRARIAN_DIGEST_SCHEMA_ID
            && schema_id != crate::runtime::librarian_broadcast::LIBRARIAN_BROADCAST_EPOCH_SCHEMA_ID
        {
            failures.push(format!("unknown Librarian schema {schema_id} at {cid}"));
        }
    }
    Ok(BcastShieldingReport {
        librarian_digest_count: digest_cids.len() as u64,
        librarian_role_crop_count: role_crop_count,
        visible_context_count,
        verdict: if failures.is_empty() {
            "PASS".to_string()
        } else {
            "FAIL".to_string()
        },
        failure_reasons: failures,
    })
}

fn duplicate_count<T>(counts: &BTreeMap<String, Vec<T>>) -> u64 {
    counts
        .values()
        .map(|items| items.len().saturating_sub(1) as u64)
        .sum::<u64>()
}
