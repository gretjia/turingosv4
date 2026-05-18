//! REAL-BCAST-1 — CAS-backed Librarian broadcast loop.
//!
//! This module derives sanitized, role-scoped notices from existing
//! ChainTape/CAS evidence. It does not introduce TypedTx, sequencer
//! admission rules, signing payloads, or CAS ObjectType variants.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::bottom_white::cas::schema::{Cid, ObjectType};
use crate::bottom_white::cas::store::{CasError, CasStore};
use crate::runtime::attempt_telemetry::{
    read_attempt_telemetry_shared_slot_from_cas, read_lean_result_from_cas, LeanVerdictKind,
};
use crate::runtime::economic_judgment::{economic_judgment_cids, read_economic_judgment_from_cas};
use crate::runtime::ev_decision_trace::{ev_decision_trace_cids, read_ev_decision_trace_from_cas};
use crate::runtime::market_decision_trace::{MarketDecisionTrace, TraceOutcome};
use crate::runtime::market_review::{
    market_review_summary_cids, validate_market_review_summary, MarketReviewSummary,
};
use crate::runtime::prompt_capsule::PromptCapsuleV2;
use crate::runtime::real5_roles::{AgentRole, HeadT};

/// TRACE_MATRIX FC3: CAS Generic schema tag for materialized LibrarianDigest views.
pub const LIBRARIAN_DIGEST_SCHEMA_ID: &str = "turingosv4.librarian_digest.v1";
/// TRACE_MATRIX FC3: CAS Generic schema tag for role-cropped broadcast views.
pub const LIBRARIAN_ROLE_CROP_SCHEMA_ID: &str = "turingosv4.librarian_role_crop.v1";
/// TRACE_MATRIX FC1/FC3: replay epoch tag for barriered broadcast prompts.
pub const LIBRARIAN_BROADCAST_EPOCH_SCHEMA_ID: &str = "turingosv4.librarian_broadcast_epoch.v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// TRACE_MATRIX FC3: explicit CAS source scope for digest derivation.
pub struct LibrarianSourceScope {
    pub current_run_cas_root: Cid,
    pub prior_capsule_cids: Vec<Cid>,
    pub max_prior_batches: u32,
    pub task_tags: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
/// TRACE_MATRIX FC3: typed evidence classes allowed into Librarian selection.
pub enum LibrarianEvidenceKind {
    LeanError,
    PartialProgress,
    MarketReason,
    EVReason,
    EconomicJudgment,
    AttemptOutcome,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// TRACE_MATRIX FC3: sanitized evidence atom selected from known CAS schemas.
pub struct LibrarianEvidenceEvent {
    pub cid: Cid,
    pub kind: LibrarianEvidenceKind,
    pub class_label: String,
    pub task_id: Option<String>,
    pub public_summary: String,
    pub head_t: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
/// TRACE_MATRIX FC3: deterministic trend label for clustered evidence.
pub enum Trend {
    Up,
    Stable,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
/// TRACE_MATRIX FC3: digest staleness label derived from tape/CAS head distance.
pub enum Staleness {
    Current,
    Recent,
    Stale,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// TRACE_MATRIX FC3: raw-log-redacted repeated-error broadcast cluster.
pub struct TypicalErrorCluster {
    pub cluster_id: String,
    pub error_class: String,
    pub count: u32,
    pub trend: Trend,
    pub task_tags: Vec<String>,
    pub role_hints: Vec<AgentRole>,
    pub public_summary: String,
    pub action_hint: Option<String>,
    pub provenance_cids: Vec<Cid>,
    pub staleness: Staleness,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// TRACE_MATRIX FC3: public category for externalized proof progress summaries.
pub enum ProgressKind {
    PartialAccepted,
    Verified,
    FailedClassified,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// TRACE_MATRIX Art.III/FC3: role/task scope for broadcast visibility.
pub enum VisibilityScope {
    SameTask,
    SameTag,
    AuditOnly,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// TRACE_MATRIX FC3: sanitized summary of externalized AttemptTelemetry/LeanResult progress.
pub struct PartialProgressSummary {
    pub summary_id: String,
    pub task_id: String,
    pub source_attempt_cid: Cid,
    pub lean_result_cid: Cid,
    pub progress_kind: ProgressKind,
    pub tactic_class: Option<String>,
    pub public_summary: String,
    pub visibility_scope: VisibilityScope,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// TRACE_MATRIX FC3: clustered market or EV reason broadcast payload.
pub struct ReasonCluster {
    pub reason: String,
    pub count: u32,
    pub role_hints: Vec<AgentRole>,
    pub public_summary: String,
    pub provenance_cids: Vec<Cid>,
    pub staleness: Staleness,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// TRACE_MATRIX FC3: CAS-backed materialized broadcast digest, not source of truth.
pub struct LibrarianDigest {
    pub schema_version: String,
    pub digest_id: Cid,
    pub source_scope: LibrarianSourceScope,
    pub generated_at_head_t: HeadT,
    pub typical_error_clusters: Vec<TypicalErrorCluster>,
    pub partial_progress_summaries: Vec<PartialProgressSummary>,
    pub market_reason_clusters: Vec<ReasonCluster>,
    pub ev_reason_clusters: Vec<ReasonCluster>,
    pub provenance_root: Cid,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// TRACE_MATRIX FC1/FC3: deterministic broadcast epoch for half-async replay.
pub struct BroadcastEpoch {
    pub epoch_id: String,
    pub source_head_t: u64,
    pub digest_cid: Cid,
    pub valid_from: u64,
    pub valid_until: u64,
    pub task_tags: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// TRACE_MATRIX Art.III/FC3: role-scoped, redacted prompt notification view.
pub struct RoleNotificationView {
    pub source_digest_cid: Cid,
    pub target_role: AgentRole,
    pub rendered_notice: String,
    pub redacted_fields: Vec<String>,
}

/// TRACE_MATRIX FC3: fail-closed source scope validator forbidding global pointers.
pub fn validate_librarian_source_scope(
    scope: &LibrarianSourceScope,
    cas: &CasStore,
) -> Result<(), String> {
    if scope.current_run_cas_root == Cid::default() {
        return Err("missing current_run_cas_root".into());
    }
    for tag in &scope.task_tags {
        let lower = tag.to_ascii_lowercase();
        if lower.contains("latest") || lower.contains("pointer") || lower.contains(".txt") {
            return Err("source scope contains forbidden global pointer marker".into());
        }
    }
    for cid in &scope.prior_capsule_cids {
        if cas.metadata(cid).is_none() {
            return Err(format!("unresolved prior_capsule_cid {cid}"));
        }
    }
    Ok(())
}

/// TRACE_MATRIX Art.0.2 + Art.II: derive a run-local CAS-root surrogate from
/// explicit CAS index CIDs. This is not a global pointer and is only used as
/// source-scope provenance for a materialized digest.
pub fn derive_current_run_cas_root(cas: &CasStore) -> Cid {
    let mut cids = cas.list_all_cids();
    cids.sort();
    let bytes = serde_json::to_vec(&cids).unwrap_or_default();
    Cid::from_content(&bytes)
}

/// TRACE_MATRIX Art.III/FC3: shielding gate for broadcast digest and prompt bytes.
pub fn assert_no_forbidden_broadcast_material(text: &str) -> Result<(), String> {
    let lower = text.to_ascii_lowercase();
    let scan_lower = lower
        .replace("raw logs redacted", "")
        .replace("raw log redacted", "");
    let forbidden = [
        "raw lean stderr",
        "raw_prompt",
        "raw prompt",
        "raw_completion",
        "raw completion",
        "raw_log",
        "raw logs",
        "raw log",
        "private cot",
        "chain of thought",
        "raw diagnostics",
        "untriaged historical",
    ];
    if let Some(hit) = forbidden
        .iter()
        .find(|needle| scan_lower.contains(**needle))
    {
        return Err(format!("forbidden broadcast material: {hit}"));
    }
    Ok(())
}

fn decode_librarian_candidate_events(
    cas: &CasStore,
    cid: &Cid,
) -> Result<Vec<LibrarianEvidenceEvent>, String> {
    let Some(meta) = cas.metadata(cid) else {
        return Err(format!("missing CAS metadata for {cid}"));
    };

    if meta.schema_id.as_deref()
        == Some(crate::runtime::market_review::MARKET_REVIEW_SUMMARY_SCHEMA_ID)
    {
        return market_review_summary_events(cas, cid, meta.created_at_logical_t);
    }

    if meta.schema_id.is_none() && meta.object_type == ObjectType::AttemptTelemetry {
        if let Some(trace) = read_market_decision_trace_from_shared_slot(cas, cid)? {
            return Ok(market_decision_trace_events(
                *cid,
                meta.created_at_logical_t,
                trace,
            ));
        }
    }

    Ok(decode_librarian_candidate(cas, cid)?.into_iter().collect())
}

fn read_market_decision_trace_from_shared_slot(
    cas: &CasStore,
    cid: &Cid,
) -> Result<Option<MarketDecisionTrace>, String> {
    let bytes = cas.get(cid).map_err(|e| e.to_string())?;
    let first = bytes.iter().copied().find(|b| !b.is_ascii_whitespace());
    if !matches!(first, Some(b'{') | Some(b'[')) {
        return Ok(None);
    }

    let value: serde_json::Value = serde_json::from_slice(&bytes)
        .map_err(|e| format!("unknown JSON in AttemptTelemetry slot: invalid JSON: {e}"))?;
    let schema_version = value.get("schema_version").and_then(|v| v.as_str());
    if schema_version != Some(MarketDecisionTrace::SCHEMA_VERSION) {
        return Ok(None);
    }

    let trace: MarketDecisionTrace =
        serde_json::from_value(value).map_err(|e| format!("MarketDecisionTrace decode: {e}"))?;
    if trace.schema_version != MarketDecisionTrace::SCHEMA_VERSION {
        return Err(format!(
            "unexpected MarketDecisionTrace schema {}",
            trace.schema_version
        ));
    }
    Ok(Some(trace))
}

fn market_decision_trace_events(
    cid: Cid,
    head_t: u64,
    trace: MarketDecisionTrace,
) -> Vec<LibrarianEvidenceEvent> {
    let TraceOutcome::NoTrade { reason, .. } = trace.outcome else {
        return Vec::new();
    };

    vec![LibrarianEvidenceEvent {
        cid,
        kind: LibrarianEvidenceKind::MarketReason,
        class_label: format!("market_no_trade:{}", reason.label()),
        task_id: trace.chosen_node_id.map(|id| id.0),
        public_summary: trace.reason_summary_public,
        head_t,
    }]
}

fn market_review_summary_events(
    cas: &CasStore,
    cid: &Cid,
    head_t: u64,
) -> Result<Vec<LibrarianEvidenceEvent>, String> {
    let bytes = cas.get(cid).map_err(|e| e.to_string())?;
    let summary: MarketReviewSummary =
        serde_json::from_slice(&bytes).map_err(|e| format!("MarketReviewSummary decode: {e}"))?;
    validate_market_review_summary(&summary)
        .map_err(|e| format!("MarketReviewSummary invalid: {e}"))?;

    let mut events = Vec::new();
    let crate::state::typed_tx::EventId(task_id_wrapper) = summary.event_id.clone();
    let task_id = task_id_wrapper.0;
    if summary.abstain_count > 0 {
        events.push(LibrarianEvidenceEvent {
            cid: *cid,
            kind: LibrarianEvidenceKind::MarketReason,
            class_label: "market_review:abstain".into(),
            task_id: Some(task_id.clone()),
            public_summary: format!(
                "Market review window recorded {} abstain responses",
                summary.abstain_count
            ),
            head_t,
        });
    }
    if summary.missing_count > 0 {
        events.push(LibrarianEvidenceEvent {
            cid: *cid,
            kind: LibrarianEvidenceKind::MarketReason,
            class_label: "market_review:missing".into(),
            task_id: Some(task_id),
            public_summary: format!(
                "Market review window recorded {} missing responses",
                summary.missing_count
            ),
            head_t,
        });
    }
    Ok(events)
}

fn market_opportunity_trace_event(
    cas: &CasStore,
    cid: &Cid,
    head_t: u64,
) -> Result<LibrarianEvidenceEvent, String> {
    let bytes = cas.get(cid).map_err(|e| e.to_string())?;
    let trace: crate::runtime::market_opportunity_trace::MarketOpportunityTrace =
        serde_json::from_slice(&bytes)
            .map_err(|e| format!("MarketOpportunityTrace decode: {e}"))?;
    if trace.schema_version
        != crate::runtime::market_opportunity_trace::MARKET_OPPORTUNITY_TRACE_SCHEMA_VERSION
    {
        return Err(format!(
            "unexpected MarketOpportunityTrace schema {}",
            trace.schema_version
        ));
    }
    let label = trace
        .reason_if_no_actionable_market
        .map(|reason| reason.label().to_string())
        .unwrap_or_else(|| "actionable".to_string());
    Ok(LibrarianEvidenceEvent {
        cid: *cid,
        kind: LibrarianEvidenceKind::MarketReason,
        class_label: format!("market_opportunity:{label}"),
        task_id: Some(trace.task_id.0),
        public_summary: format!(
            "Market opportunity trace recorded {} visible and {} actionable markets",
            trace.visible_markets.len(),
            trace.actionable_markets.len()
        ),
        head_t,
    })
}

/// TRACE_MATRIX FC3: typed CAS decoder for allowed Librarian evidence schemas.
pub fn decode_librarian_candidate(
    cas: &CasStore,
    cid: &Cid,
) -> Result<Option<LibrarianEvidenceEvent>, String> {
    let Some(meta) = cas.metadata(cid) else {
        return Err(format!("missing CAS metadata for {cid}"));
    };
    match meta.schema_id.as_deref() {
        Some(crate::runtime::ev_decision_trace::EV_DECISION_TRACE_SCHEMA_ID) => {
            let trace = read_ev_decision_trace_from_cas(cas, cid).map_err(|e| e.to_string())?;
            Ok(Some(LibrarianEvidenceEvent {
                cid: *cid,
                kind: LibrarianEvidenceKind::EVReason,
                class_label: format!("ev:{:?}", trace.reason),
                task_id: Some(trace.task_id.0),
                public_summary: trace.public_summary,
                head_t: meta.created_at_logical_t,
            }))
        }
        Some(crate::runtime::economic_judgment::ECONOMIC_JUDGMENT_SCHEMA_ID) => {
            let judgment = read_economic_judgment_from_cas(cas, cid).map_err(|e| e.to_string())?;
            Ok(Some(LibrarianEvidenceEvent {
                cid: *cid,
                kind: LibrarianEvidenceKind::EconomicJudgment,
                class_label: format!("economic:{:?}", judgment.reason),
                task_id: Some(judgment.task_id.0),
                public_summary: judgment.public_summary,
                head_t: meta.created_at_logical_t,
            }))
        }
        Some(crate::runtime::market_opportunity_trace::MARKET_OPPORTUNITY_TRACE_SCHEMA_VERSION) => {
            market_opportunity_trace_event(cas, cid, meta.created_at_logical_t).map(Some)
        }
        Some(schema) if schema == crate::runtime::attempt_telemetry::LEAN_RESULT_SCHEMA_ID => {
            let result = read_lean_result_from_cas(cas, cid).map_err(|e| e.to_string())?;
            let (kind, class_label, progress_kind) = match result.verdict_kind {
                LeanVerdictKind::PartialAccepted => (
                    LibrarianEvidenceKind::PartialProgress,
                    "lean:PartialAccepted".to_string(),
                    Some(ProgressKind::PartialAccepted),
                ),
                LeanVerdictKind::Verified => (
                    LibrarianEvidenceKind::PartialProgress,
                    "lean:Verified".to_string(),
                    Some(ProgressKind::Verified),
                ),
                LeanVerdictKind::Failed | LeanVerdictKind::SorryBlocked => (
                    LibrarianEvidenceKind::LeanError,
                    format!(
                        "err:{}",
                        result
                            .error_class
                            .map(|c| format!("{c:?}"))
                            .unwrap_or_else(|| format!("{:?}", result.verdict_kind))
                    ),
                    Some(ProgressKind::FailedClassified),
                ),
            };
            let summary = match progress_kind {
                Some(ProgressKind::PartialAccepted) => {
                    "Externalized Lean attempt made partial progress".to_string()
                }
                Some(ProgressKind::Verified) => "Externalized Lean attempt verified".to_string(),
                _ => "Externalized Lean attempt failed with classified error".to_string(),
            };
            Ok(Some(LibrarianEvidenceEvent {
                cid: *cid,
                kind,
                class_label,
                task_id: None,
                public_summary: summary,
                head_t: meta.created_at_logical_t,
            }))
        }
        Some(schema)
            if schema == crate::runtime::attempt_telemetry::ATTEMPT_TELEMETRY_SCHEMA_ID
                || schema == "AttemptTelemetry" =>
        {
            let Some(attempt) =
                read_attempt_telemetry_shared_slot_from_cas(cas, cid).map_err(|e| e.to_string())?
            else {
                return Ok(None);
            };
            Ok(Some(LibrarianEvidenceEvent {
                cid: *cid,
                kind: LibrarianEvidenceKind::AttemptOutcome,
                class_label: format!("attempt:{:?}", attempt.outcome),
                task_id: Some(attempt.task_id),
                public_summary: format!("Externalized attempt outcome {:?}", attempt.outcome),
                head_t: meta.created_at_logical_t,
            }))
        }
        Some(schema)
            if schema.starts_with("TypedTx.")
                || schema == crate::runtime::prompt_capsule::PROMPT_CAPSULE_V2_SCHEMA_ID
                || schema == crate::runtime::real5_roles::ROLE_TURN_TRACE_SCHEMA_ID
                || schema == "real5.prompt.visible_context.v1"
                || schema == "real5.derived_view.v1"
                || schema == crate::runtime::market_review::MARKET_REVIEW_WINDOW_SCHEMA_ID
                || schema == crate::runtime::market_review::MARKET_REVIEW_RESPONSE_SCHEMA_ID
                || schema == crate::runtime::market_review::MARKET_REVIEW_SUMMARY_SCHEMA_ID
                || schema
                    == crate::runtime::market_decision_provenance_link::MARKET_DECISION_PROVENANCE_LINK_SCHEMA_ID
                || schema == crate::runtime::policy_trader_trace::POLICY_TRADER_TRACE_SCHEMA_ID
                || schema == LIBRARIAN_DIGEST_SCHEMA_ID
                || schema == LIBRARIAN_ROLE_CROP_SCHEMA_ID
                || schema == LIBRARIAN_BROADCAST_EPOCH_SCHEMA_ID
                || schema == "TransitionError.display.v1"
                || schema == crate::runtime::real5_roles::ROLE_ASSIGNMENT_MANIFEST_SCHEMA_ID
                || schema
                    == crate::runtime::genesis_report::MODEL_ASSIGNMENT_MANIFEST_SCHEMA_ID
                || schema == "turingosv4.agent_proposal_record.v1"
                || schema == "turingosv4.verification_result.v1"
                || schema == "turingosv4.proposal_telemetry.v1" =>
        {
            Ok(None)
        }
        Some(schema) => Err(format!("unknown librarian evidence schema: {schema}")),
        None => match meta.object_type {
            ObjectType::LeanResult => {
                let result = read_lean_result_from_cas(cas, cid).map_err(|e| e.to_string())?;
                let class_label = match result.verdict_kind {
                    LeanVerdictKind::PartialAccepted => "lean:PartialAccepted".to_string(),
                    LeanVerdictKind::Verified => "lean:Verified".to_string(),
                    LeanVerdictKind::Failed | LeanVerdictKind::SorryBlocked => format!(
                        "err:{}",
                        result
                            .error_class
                            .map(|c| format!("{c:?}"))
                            .unwrap_or_else(|| format!("{:?}", result.verdict_kind))
                    ),
                };
                Ok(Some(LibrarianEvidenceEvent {
                    cid: *cid,
                    kind: if matches!(
                        result.verdict_kind,
                        LeanVerdictKind::PartialAccepted | LeanVerdictKind::Verified
                    ) {
                        LibrarianEvidenceKind::PartialProgress
                    } else {
                        LibrarianEvidenceKind::LeanError
                    },
                    class_label,
                    task_id: None,
                    public_summary: "Externalized Lean attempt has classified verdict".into(),
                    head_t: meta.created_at_logical_t,
                }))
            }
            ObjectType::AttemptTelemetry => {
                let Some(attempt) = read_attempt_telemetry_shared_slot_from_cas(cas, cid)
                    .map_err(|e| e.to_string())?
                else {
                    return Ok(None);
                };
                Ok(Some(LibrarianEvidenceEvent {
                    cid: *cid,
                    kind: LibrarianEvidenceKind::AttemptOutcome,
                    class_label: format!("attempt:{:?}", attempt.outcome),
                    task_id: Some(attempt.task_id),
                    public_summary: format!("Externalized attempt outcome {:?}", attempt.outcome),
                    head_t: meta.created_at_logical_t,
                }))
            }
            _ => Ok(None),
        },
    }
}

/// TRACE_MATRIX FC3: deterministic selector over known CAS evidence, no raw scans.
pub fn select_librarian_events(cas: &CasStore) -> Result<Vec<LibrarianEvidenceEvent>, String> {
    let mut cids = Vec::new();
    cids.extend(ev_decision_trace_cids(cas));
    cids.extend(economic_judgment_cids(cas));
    cids.extend(market_review_summary_cids(cas));
    cids.extend(cas.list_cids_by_object_type(ObjectType::Generic));
    cids.extend(cas.list_cids_by_object_type(ObjectType::LeanResult));
    cids.extend(cas.list_cids_by_object_type(ObjectType::AttemptTelemetry));
    cids.sort();
    cids.dedup();

    let mut events = Vec::new();
    for cid in cids {
        for event in decode_librarian_candidate_events(cas, &cid)? {
            assert_no_forbidden_broadcast_material(&event.public_summary)?;
            events.push(event);
        }
    }
    events.sort_by(|a, b| {
        a.class_label
            .cmp(&b.class_label)
            .then_with(|| a.head_t.cmp(&b.head_t))
            .then_with(|| a.cid.cmp(&b.cid))
    });
    Ok(events)
}

/// TRACE_MATRIX FC3: deterministic digest builder from selected ChainTape/CAS evidence.
pub fn build_librarian_digest(
    source_scope: LibrarianSourceScope,
    generated_at_head_t: u64,
    mut events: Vec<LibrarianEvidenceEvent>,
) -> Result<LibrarianDigest, String> {
    events.sort_by(|a, b| {
        a.class_label
            .cmp(&b.class_label)
            .then_with(|| a.head_t.cmp(&b.head_t))
            .then_with(|| a.cid.cmp(&b.cid))
    });
    for event in &events {
        assert_no_forbidden_broadcast_material(&event.public_summary)?;
    }

    let mut grouped: BTreeMap<String, Vec<LibrarianEvidenceEvent>> = BTreeMap::new();
    for event in events {
        grouped
            .entry(event.class_label.clone())
            .or_default()
            .push(event);
    }

    let mut typical_error_clusters = Vec::new();
    let mut partial_progress_summaries = Vec::new();
    let mut market_reason_clusters = Vec::new();
    let mut ev_reason_clusters = Vec::new();
    let mut provenance = Vec::new();

    for (class, items) in grouped {
        provenance.extend(items.iter().map(|e| e.cid));
        let staleness = staleness_for(&items, generated_at_head_t);
        let count = items.len() as u32;
        let task_tags = task_tags_for(&items);
        match items[0].kind {
            LibrarianEvidenceKind::LeanError => {
                if count >= 2 {
                    typical_error_clusters.push(TypicalErrorCluster {
                        cluster_id: format!("cluster:{class}"),
                        error_class: class.clone(),
                        count,
                        trend: Trend::Stable,
                        task_tags,
                        role_hints: vec![AgentRole::Solver, AgentRole::Verifier],
                        public_summary: format!("{class} repeated {count} times"),
                        action_hint: Some(
                            "Check the goal/type shape before repeating the tactic".into(),
                        ),
                        provenance_cids: items.iter().map(|e| e.cid).collect(),
                        staleness,
                    });
                }
            }
            LibrarianEvidenceKind::PartialProgress => {
                for event in items {
                    let progress_kind = if event.class_label == "lean:Verified" {
                        ProgressKind::Verified
                    } else {
                        ProgressKind::PartialAccepted
                    };
                    partial_progress_summaries.push(PartialProgressSummary {
                        summary_id: format!("partial:{}", event.cid.hex()),
                        task_id: event.task_id.unwrap_or_else(|| "unknown-task".into()),
                        source_attempt_cid: event.cid,
                        lean_result_cid: event.cid,
                        progress_kind,
                        tactic_class: Some(class.clone()),
                        public_summary: event.public_summary,
                        visibility_scope: VisibilityScope::SameTask,
                    });
                }
            }
            LibrarianEvidenceKind::MarketReason => {
                market_reason_clusters.push(ReasonCluster {
                    reason: class.clone(),
                    count,
                    role_hints: vec![
                        AgentRole::Trader,
                        AgentRole::BullTrader,
                        AgentRole::BearTrader,
                    ],
                    public_summary: format!("{class} observed {count} times"),
                    provenance_cids: items.iter().map(|e| e.cid).collect(),
                    staleness,
                });
            }
            LibrarianEvidenceKind::EVReason | LibrarianEvidenceKind::EconomicJudgment => {
                ev_reason_clusters.push(ReasonCluster {
                    reason: class.clone(),
                    count,
                    role_hints: vec![
                        AgentRole::Trader,
                        AgentRole::BullTrader,
                        AgentRole::BearTrader,
                    ],
                    public_summary: format!("{class} observed {count} times"),
                    provenance_cids: items.iter().map(|e| e.cid).collect(),
                    staleness,
                });
            }
            LibrarianEvidenceKind::AttemptOutcome => {}
        }
    }
    provenance.sort();
    provenance.dedup();
    let provenance_root = Cid::from_content(
        &serde_json::to_vec(&provenance).map_err(|e| format!("provenance encode: {e}"))?,
    );

    let mut digest = LibrarianDigest {
        schema_version: LIBRARIAN_DIGEST_SCHEMA_ID.to_string(),
        digest_id: Cid::default(),
        source_scope,
        generated_at_head_t: generated_at_head_t.to_string(),
        typical_error_clusters,
        partial_progress_summaries,
        market_reason_clusters,
        ev_reason_clusters,
        provenance_root,
    };
    digest.digest_id = compute_digest_id(&digest)?;
    validate_librarian_digest(&digest)?;
    Ok(digest)
}

fn task_tags_for(items: &[LibrarianEvidenceEvent]) -> Vec<String> {
    let mut tags: Vec<String> = items.iter().filter_map(|e| e.task_id.clone()).collect();
    tags.sort();
    tags.dedup();
    tags
}

fn staleness_for(items: &[LibrarianEvidenceEvent], generated_at_head_t: u64) -> Staleness {
    let latest = items.iter().map(|e| e.head_t).max().unwrap_or(0);
    match generated_at_head_t.saturating_sub(latest) {
        0..=5 => Staleness::Current,
        6..=50 => Staleness::Recent,
        _ => Staleness::Stale,
    }
}

fn compute_digest_id(digest: &LibrarianDigest) -> Result<Cid, String> {
    #[derive(Serialize)]
    struct DigestIdInput<'a> {
        schema_version: &'a str,
        source_scope: &'a LibrarianSourceScope,
        generated_at_head_t: &'a HeadT,
        typical_error_clusters: &'a [TypicalErrorCluster],
        partial_progress_summaries: &'a [PartialProgressSummary],
        market_reason_clusters: &'a [ReasonCluster],
        ev_reason_clusters: &'a [ReasonCluster],
        provenance_root: Cid,
    }
    let input = DigestIdInput {
        schema_version: &digest.schema_version,
        source_scope: &digest.source_scope,
        generated_at_head_t: &digest.generated_at_head_t,
        typical_error_clusters: &digest.typical_error_clusters,
        partial_progress_summaries: &digest.partial_progress_summaries,
        market_reason_clusters: &digest.market_reason_clusters,
        ev_reason_clusters: &digest.ev_reason_clusters,
        provenance_root: digest.provenance_root,
    };
    let bytes = serde_json::to_vec(&input).map_err(|e| format!("digest id encode: {e}"))?;
    Ok(Cid::from_content(&bytes))
}

/// TRACE_MATRIX FC3: fail-closed digest validator for provenance and no-leak gates.
pub fn validate_librarian_digest(digest: &LibrarianDigest) -> Result<(), String> {
    if digest.schema_version != LIBRARIAN_DIGEST_SCHEMA_ID {
        return Err(format!(
            "unexpected LibrarianDigest schema {}",
            digest.schema_version
        ));
    }
    if digest.digest_id == Cid::default() {
        return Err("LibrarianDigest digest_id must be non-zero".into());
    }
    if digest.provenance_root == Cid::default() {
        return Err("LibrarianDigest provenance_root must be non-zero".into());
    }
    for cluster in &digest.typical_error_clusters {
        assert_no_forbidden_broadcast_material(&cluster.public_summary)?;
        if let Some(hint) = &cluster.action_hint {
            assert_no_forbidden_broadcast_material(hint)?;
        }
        if cluster.count < 2 {
            return Err("TypicalErrorCluster requires >=2 evidence events".into());
        }
        if cluster.provenance_cids.is_empty() {
            return Err("TypicalErrorCluster requires provenance_cids".into());
        }
    }
    for summary in &digest.partial_progress_summaries {
        assert_no_forbidden_broadcast_material(&summary.public_summary)?;
        if summary.source_attempt_cid == Cid::default() || summary.lean_result_cid == Cid::default()
        {
            return Err(
                "PartialProgressSummary requires source AttemptTelemetry/LeanResult CIDs".into(),
            );
        }
    }
    for cluster in digest
        .market_reason_clusters
        .iter()
        .chain(digest.ev_reason_clusters.iter())
    {
        assert_no_forbidden_broadcast_material(&cluster.public_summary)?;
        if cluster.provenance_cids.is_empty() {
            return Err("ReasonCluster requires provenance_cids".into());
        }
    }
    Ok(())
}

/// TRACE_MATRIX FC3: writes LibrarianDigest as CAS Generic materialized view.
pub fn write_librarian_digest_to_cas(
    cas: &mut CasStore,
    digest: &LibrarianDigest,
    suffix: &str,
    logical_t: u64,
) -> Result<Cid, CasError> {
    validate_librarian_digest(digest)
        .map_err(|e| CasError::BackendCorruption(format!("LibrarianDigest invalid: {e}")))?;
    let bytes = serde_json::to_vec(digest)
        .map_err(|e| CasError::BackendCorruption(format!("LibrarianDigest encode: {e}")))?;
    cas.put(
        &bytes,
        ObjectType::Generic,
        &format!("real-bcast-librarian-digest-{suffix}"),
        logical_t,
        Some(LIBRARIAN_DIGEST_SCHEMA_ID.to_string()),
    )
}

/// TRACE_MATRIX FC3: reads and validates CAS-backed LibrarianDigest views.
pub fn read_librarian_digest_from_cas(
    cas: &CasStore,
    cid: &Cid,
) -> Result<LibrarianDigest, CasError> {
    let bytes = cas.get(cid)?;
    let digest: LibrarianDigest = serde_json::from_slice(&bytes)
        .map_err(|e| CasError::BackendCorruption(format!("LibrarianDigest decode: {e}")))?;
    validate_librarian_digest(&digest)
        .map_err(|e| CasError::BackendCorruption(format!("LibrarianDigest invalid: {e}")))?;
    Ok(digest)
}

/// TRACE_MATRIX FC3: lists digest CIDs by schema tag from CAS metadata.
pub fn librarian_digest_cids(cas: &CasStore) -> Vec<Cid> {
    cas.list_all_cids()
        .into_iter()
        .filter(|cid| {
            cas.metadata(cid).and_then(|meta| meta.schema_id.as_deref())
                == Some(LIBRARIAN_DIGEST_SCHEMA_ID)
        })
        .collect()
}

/// TRACE_MATRIX FC1/FC3: rejects future/stale broadcast epochs for replay safety.
pub fn validate_broadcast_epoch(epoch: &BroadcastEpoch, current_head_t: u64) -> Result<(), String> {
    if epoch.epoch_id.trim().is_empty() {
        return Err("BroadcastEpoch epoch_id must be non-empty".into());
    }
    if epoch.digest_cid == Cid::default() {
        return Err("BroadcastEpoch digest_cid must be non-zero".into());
    }
    if epoch.valid_from < epoch.source_head_t {
        return Err("BroadcastEpoch valid_from cannot precede source_head_t".into());
    }
    if current_head_t < epoch.valid_from {
        return Err("future digest epoch used before valid_from".into());
    }
    if current_head_t > epoch.valid_until {
        return Err("expired broadcast epoch".into());
    }
    Ok(())
}

/// TRACE_MATRIX Art.III/FC3: projects one digest into role-scoped notices.
pub fn project_role_notifications(
    digest: &LibrarianDigest,
    role: AgentRole,
    max_items: usize,
) -> Result<RoleNotificationView, String> {
    let mut notices = Vec::new();
    match role {
        AgentRole::Solver => {
            notices.extend(
                digest
                    .typical_error_clusters
                    .iter()
                    .take(max_items)
                    .map(|c| {
                        format!(
                            "{} count={} trend={:?} hint={}",
                            c.error_class,
                            c.count,
                            c.trend,
                            c.action_hint.clone().unwrap_or_default()
                        )
                    }),
            );
            notices.extend(
                digest
                    .partial_progress_summaries
                    .iter()
                    .take(max_items.saturating_sub(notices.len()))
                    .map(|s| format!("partial:{} {}", s.summary_id, s.public_summary)),
            );
        }
        AgentRole::Trader | AgentRole::BullTrader | AgentRole::BearTrader => {
            notices.extend(digest.ev_reason_clusters.iter().take(max_items).map(|c| {
                format!(
                    "{} count={} price anomaly check: {}",
                    c.reason, c.count, c.public_summary
                )
            }));
            notices.extend(
                digest
                    .market_reason_clusters
                    .iter()
                    .take(max_items.saturating_sub(notices.len()))
                    .map(|c| format!("{} count={} {}", c.reason, c.count, c.public_summary)),
            );
        }
        AgentRole::Verifier => {
            notices
                .push("proof-risk: inspect common false-progress patterns before verifying".into());
            notices.extend(
                digest
                    .typical_error_clusters
                    .iter()
                    .take(max_items)
                    .map(|c| format!("proof-risk {} count={}", c.error_class, c.count)),
            );
        }
        AgentRole::Challenger => {
            notices.push("suspicious-node summary: repeated failures may mark weak claims".into());
            notices.extend(
                digest
                    .typical_error_clusters
                    .iter()
                    .take(max_items)
                    .map(|c| format!("weakness {} count={}", c.error_class, c.count)),
            );
        }
        AgentRole::Architect => {
            notices.push(format!(
                "aggregate trends: errors={} ev_reasons={}",
                digest.typical_error_clusters.len(),
                digest.ev_reason_clusters.len()
            ));
        }
        _ => {}
    }
    if notices.is_empty() {
        notices.push("No librarian notices for this role at current scope".into());
    }
    let rendered_notice =
        render_librarian_notices_section(&digest.digest_id.to_string(), &notices, max_items)?;
    Ok(RoleNotificationView {
        source_digest_cid: digest.digest_id,
        target_role: role,
        rendered_notice,
        redacted_fields: vec![
            "lean_stderr_redacted".into(),
            "prompt_body_redacted".into(),
            "completion_body_redacted".into(),
            "private_cot_redacted".into(),
            "diagnostics_redacted".into(),
        ],
    })
}

/// TRACE_MATRIX FC3: writes role-cropped notices as CAS Generic materialized views.
pub fn write_role_notification_view_to_cas(
    cas: &mut CasStore,
    view: &RoleNotificationView,
    suffix: &str,
    logical_t: u64,
) -> Result<Cid, CasError> {
    assert_no_forbidden_broadcast_material(&view.rendered_notice)
        .map_err(|e| CasError::BackendCorruption(format!("RoleNotificationView invalid: {e}")))?;
    let bytes = serde_json::to_vec(view)
        .map_err(|e| CasError::BackendCorruption(format!("RoleNotificationView encode: {e}")))?;
    cas.put(
        &bytes,
        ObjectType::Generic,
        &format!("real-bcast-role-notification-{suffix}"),
        logical_t,
        Some(LIBRARIAN_ROLE_CROP_SCHEMA_ID.to_string()),
    )
}

/// TRACE_MATRIX FC1/FC3: renders bounded prompt section from role-cropped notices.
pub fn render_librarian_notices_section(
    digest_label: &str,
    notices: &[String],
    max_items: usize,
) -> Result<String, String> {
    let mut out = String::new();
    out.push_str("=== Librarian Notices ===\n");
    out.push_str("source: CAS/ChainTape-derived, role-scoped, raw logs redacted\n");
    out.push_str(&format!("digest: {digest_label}\n"));
    for notice in notices.iter().take(max_items) {
        assert_no_forbidden_broadcast_material(notice)?;
        out.push_str("- ");
        out.push_str(notice);
        out.push('\n');
    }
    out.push('\n');
    Ok(out)
}

/// TRACE_MATRIX FC1/FC2/FC3: verifies PromptCapsule read_set and visible-context binding.
pub fn validate_prompt_capsule_librarian_binding(
    capsule: &PromptCapsuleV2,
    visible_context_bytes: &[u8],
    digest_cid: Cid,
    role_crop_cid: Cid,
) -> Result<(), String> {
    if !capsule.read_set.contains(&digest_cid) {
        return Err("PromptCapsuleV2 read_set missing LibrarianDigest CID".into());
    }
    if !capsule.read_set.contains(&role_crop_cid) {
        return Err("PromptCapsuleV2 read_set missing role crop CID".into());
    }
    if capsule.visible_context_cid != Cid::from_content(visible_context_bytes) {
        return Err("PromptCapsuleV2 visible_context_cid does not match visible bytes".into());
    }
    if crate::state::q_state::Hash(Cid::from_content(visible_context_bytes).0)
        != capsule.prompt_context_hash
    {
        return Err("PromptCapsuleV2 prompt_context_hash does not match visible bytes".into());
    }
    let visible = String::from_utf8_lossy(visible_context_bytes);
    if !visible.contains("=== Librarian Notices ===") {
        return Err("visible context missing Librarian Notices section".into());
    }
    assert_no_forbidden_broadcast_material(&visible)?;
    Ok(())
}
