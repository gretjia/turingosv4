//! TRACE_MATRIX FC1-N5: Phase 7 web — GET /api/dag/by-session/:session_id
//!
//! Read-only **citation DAG** projection over the canonical ChainTape + CAS.
//! Reconstructs the multi-agent proof/work DAG (ζ-Sum style): each WorkTx node
//! cites a parent (`ProposalTelemetry.parent_tx`), forming a multi-root tree;
//! per-node trading (YES/NO positions → BULL/BEAR dominance, whale, price),
//! agent role activity, and the Golden Path to OMEGA (oracle-verified chain).
//!
//! **Truth boundary** (mirrors `market_view.rs`): pure projection over the
//! replayed `EconomicState` + `LedgerEntry` rows + their CAS `TypedTx` payloads.
//! Never a second truth source; never read by economic/winner logic. Two
//! requests over the same chain return byte-identical JSON (no `AppState`
//! cache). The parent_tx walk + golden-path reconstruction mirror the proven
//! `src/bin/audit_dashboard.rs` oracle (NOT a kernel import).

use std::collections::BTreeMap;

use axum::extract::{Path, State};
use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Response};

use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::bottom_white::ledger::transition_ledger::{
    canonical_decode, replay_full_transition_with_predicate_binding, Git2LedgerWriter,
    LedgerEntry, LedgerWriter, TxKind,
};
use turingosv4::bottom_white::tools::registry::ToolRegistry;
use turingosv4::runtime::agent_role_classifier::{classify_agent_role, RoleActivity};
use turingosv4::runtime::predicate_registry_loader;
use turingosv4::runtime::proposal_telemetry::read_from_cas as read_proposal_telemetry;
use turingosv4::runtime::verification_result::read_from_cas as read_verification_result;
use turingosv4::state::q_state::{EconomicState, QState};
use turingosv4::state::typed_tx::{EventId, TypedTx};

use super::market_view::{derive_yes_signal_bp, read_initial_q_state, read_pinned_pubkeys};
use super::ws::AppState;

/// Whale threshold: total position units on a node above this = ⚠W.
const WHALE_UNITS: u64 = 500;

fn is_safe_session_id(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= 128
        && s.chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}

/// GET /api/dag/by-session/:session_id — citation DAG projection.
pub async fn dag_view_handler(
    Path(session_id): Path<String>,
    State(_state): State<AppState>,
) -> Response {
    if !is_safe_session_id(&session_id) {
        return (
            StatusCode::BAD_REQUEST,
            [(header::CONTENT_TYPE, "application/json")],
            r#"{"error":"invalid session_id"}"#.to_string(),
        )
            .into_response();
    }
    let workspace = super::welcome::resolve_workspace_path();
    match build_dag_view(&workspace, &session_id) {
        Ok(Some(body)) => (
            StatusCode::OK,
            [(header::CONTENT_TYPE, "application/json")],
            body,
        )
            .into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            [(header::CONTENT_TYPE, "application/json")],
            format!(
                r#"{{"error":"no chain for session","session_id":"{}"}}"#,
                session_id
            ),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            [(header::CONTENT_TYPE, "application/json")],
            format!(r#"{{"error":"dag view build failed: {}"}}"#, e),
        )
            .into_response(),
    }
}

/// One reconstructed DAG node (a WorkTx).
struct DagNode {
    tx_id: String,
    parent_tx: Option<String>,
    agent: String,
    tactic: String,
    oracle_verified: bool,
    on_golden_path: bool,
    // trading (derived from the node's event market)
    yes_units: u64,
    no_units: u64,
    bet_count: u64,
    price_bp: u32,
    traded: bool,
}

/// Aggregate trading for one event from the replayed EconomicState.
/// `yes/no_units` = total conditional shares held across all agents on the
/// event (the YES/NO bets); `bet_count` = #agents with a nonzero position.
fn node_trading(econ: &EconomicState, event_id: &EventId) -> (u64, u64, u64, u32, bool) {
    let mut yes: u64 = 0;
    let mut no: u64 = 0;
    let mut bettors: u64 = 0;
    for by_event in econ.conditional_share_balances_t.0.values() {
        if let Some(pair) = by_event.get(event_id) {
            let y = pair.yes.units as u64;
            let n = pair.no.units as u64;
            if y > 0 || n > 0 {
                bettors += 1;
                yes = yes.saturating_add(y);
                no = no.saturating_add(n);
            }
        }
    }
    let has_pool = econ.cpmm_pools_t.0.contains_key(event_id);
    let traded = has_pool || bettors > 0;
    let price_bp = derive_yes_signal_bp(econ, event_id);
    (yes, no, bettors, price_bp, traded)
}

fn price_marker(traded: bool, price_bp: u32) -> &'static str {
    if !traded {
        "never"
    } else if price_bp >= 5000 {
        "P1"
    } else {
        "P0"
    }
}

fn dominance(yes: u64, no: u64) -> Option<&'static str> {
    if yes == 0 && no == 0 {
        None
    } else if yes >= no {
        Some("BULL")
    } else {
        Some("BEAR")
    }
}

/// Reconstruct the citation DAG for the workspace's canonical chain.
/// Mirrors `market_view::build_market_view` (chain open + replay) and
/// `audit_dashboard.rs` (parent_tx lineage + golden path).
fn build_dag_view(workspace: &std::path::Path, session_id: &str) -> Result<Option<String>, String> {
    let runtime_repo_path = workspace.join("runtime_repo");
    let cas_path = workspace.join("cas");
    if !runtime_repo_path.exists() || !cas_path.exists() {
        return Ok(None);
    }
    let writer = Git2LedgerWriter::open(&runtime_repo_path)
        .map_err(|e| format!("open Git2LedgerWriter: {e}"))?;
    let chain_len = writer.len();
    if chain_len == 0 {
        return Ok(None);
    }
    let entries: Vec<LedgerEntry> = (1..=chain_len)
        .map(|t| writer.read_at(t).map_err(|e| format!("read_at({t}): {e}")))
        .collect::<Result<_, _>>()?;
    let cas = CasStore::open(&cas_path).map_err(|e| format!("open cas: {e}"))?;

    // ── Replay once → post-state EconomicState (same primitive as market_view).
    let initial_q = read_initial_q_state(&runtime_repo_path)?;
    let pinned = read_pinned_pubkeys(&runtime_repo_path)?;
    let predicates = predicate_registry_loader::load_replay_registry();
    let tools = ToolRegistry::new();
    let replayed_q: QState = replay_full_transition_with_predicate_binding(
        &initial_q, &entries, &cas, &cas, &pinned, &predicates, &tools,
    )
    .map_err(|e| format!("replay_full_transition: {e:?}"))?;
    let econ = &replayed_q.economic_state_t;

    // ── Walk L4 entries: build nodes + parent edges + role activity +
    // oracle-verified set (mirror audit_dashboard.rs:505-553).
    let mut per_agent: BTreeMap<String, RoleActivity> = BTreeMap::new();
    let mut nodes: Vec<DagNode> = Vec::new();
    let mut work_parent_by_tx_id: BTreeMap<String, Option<String>> = BTreeMap::new();
    // tx_id -> EventId(task) used for per-node trading lookup.
    let mut node_event: BTreeMap<String, EventId> = BTreeMap::new();
    // first oracle-verified WorkTx (deterministic = chain order) → golden path root.
    let mut first_verified: Option<String> = None;

    for entry in &entries {
        let bytes = match cas.get(&entry.tx_payload_cid) {
            Ok(b) => b,
            Err(_) => continue,
        };
        let tx: TypedTx = match canonical_decode(&bytes) {
            Ok(t) => t,
            Err(_) => continue,
        };
        // Role-activity counters (public accepted-tx counts only).
        match entry.tx_kind {
            TxKind::Verify => {
                if let TypedTx::Verify(v) = &tx {
                    per_agent
                        .entry(v.verifier_agent.0.clone())
                        .or_default()
                        .verify_tx_accepted += 1;
                }
            }
            TxKind::Challenge => {
                if let TypedTx::Challenge(c) = &tx {
                    per_agent
                        .entry(c.challenger_agent.0.clone())
                        .or_default()
                        .challenge_tx_accepted += 1;
                }
            }
            TxKind::CpmmSwap | TxKind::BuyWithCoinRouter => {
                // Trading activity — credit the signer if exposed; best-effort.
                // (Role classifier only needs approximate invest counts.)
            }
            TxKind::Work => {
                if let TypedTx::Work(work) = &tx {
                    per_agent
                        .entry(work.agent_id.0.clone())
                        .or_default()
                        .work_tx_accepted += 1;

                    let mut parent_tx: Option<String> = None;
                    let mut tactic = String::new();
                    let mut oracle_verified = false;
                    if work.proposal_cid.0 != [0u8; 32] {
                        if let Ok(tel) = read_proposal_telemetry(&cas, &work.proposal_cid) {
                            tactic = tel.candidate_tactic.clone();
                            parent_tx = tel.parent_tx.as_ref().map(|t| t.0.clone());
                            if let Some(vr_cid) = tel.verification_result_cid.as_ref() {
                                if let Ok(vr) = read_verification_result(&cas, vr_cid) {
                                    oracle_verified = vr.verified;
                                }
                            }
                        }
                    }
                    let tx_id = work.tx_id.0.clone();
                    work_parent_by_tx_id.insert(tx_id.clone(), parent_tx.clone());
                    node_event.insert(tx_id.clone(), EventId(work.task_id.clone()));
                    if oracle_verified && first_verified.is_none() {
                        first_verified = Some(tx_id.clone());
                    }
                    nodes.push(DagNode {
                        tx_id,
                        parent_tx,
                        agent: work.agent_id.0.clone(),
                        tactic,
                        oracle_verified,
                        on_golden_path: false,
                        yes_units: 0,
                        no_units: 0,
                        bet_count: 0,
                        price_bp: 5000,
                        traded: false,
                    });
                }
            }
            _ => {}
        }
    }

    if nodes.is_empty() {
        return Ok(None);
    }

    // ── Per-node trading from replayed EconomicState.
    for node in &mut nodes {
        if let Some(eid) = node_event.get(&node.tx_id) {
            let (yes, no, bets, price_bp, traded) = node_trading(econ, eid);
            node.yes_units = yes;
            node.no_units = no;
            node.bet_count = bets;
            node.price_bp = price_bp;
            node.traded = traded;
        }
    }

    // ── Golden path: walk parent_tx upward from first oracle-verified node,
    // reverse → root→winner (mirror audit_dashboard.rs:869-925).
    let mut golden_path: Vec<String> = Vec::new();
    if let Some(winner) = first_verified.clone() {
        let mut chain = vec![winner.clone()];
        let mut cursor = work_parent_by_tx_id.get(&winner).cloned().flatten();
        let mut safety = 0;
        while let Some(parent) = cursor {
            safety += 1;
            if safety > 1000 {
                break;
            }
            chain.push(parent.clone());
            cursor = work_parent_by_tx_id.get(&parent).cloned().flatten();
        }
        chain.reverse();
        let on_gp: std::collections::BTreeSet<String> = chain.iter().cloned().collect();
        for node in &mut nodes {
            if on_gp.contains(&node.tx_id) {
                node.on_golden_path = true;
            }
        }
        golden_path.push("ROOT".to_string());
        golden_path.extend(chain);
        golden_path.push("OMEGA".to_string());
    }

    // ── children index for the frontend tree.
    let mut children: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for node in &nodes {
        if let Some(p) = &node.parent_tx {
            children.entry(p.clone()).or_default().push(node.tx_id.clone());
        }
    }

    // ── depth via BFS from roots.
    let mut depth: BTreeMap<String, u32> = BTreeMap::new();
    let roots: Vec<String> = nodes
        .iter()
        .filter(|n| n.parent_tx.is_none())
        .map(|n| n.tx_id.clone())
        .collect();
    let mut queue: std::collections::VecDeque<(String, u32)> =
        roots.iter().map(|r| (r.clone(), 0u32)).collect();
    while let Some((tx, d)) = queue.pop_front() {
        depth.insert(tx.clone(), d);
        if let Some(kids) = children.get(&tx) {
            for k in kids {
                if !depth.contains_key(k) {
                    queue.push_back((k.clone(), d + 1));
                }
            }
        }
    }

    // ── Assemble JSON.
    let traded_count = nodes.iter().filter(|n| n.traded).count();
    let nodes_json: Vec<serde_json::Value> = nodes
        .iter()
        .map(|n| {
            serde_json::json!({
                "tx_id": n.tx_id,
                "parent_tx": n.parent_tx,
                "agent": n.agent,
                "role": classify_agent_role(per_agent.get(&n.agent).unwrap_or(&RoleActivity::default())).label(),
                "tactic": n.tactic,
                "yes_units": n.yes_units,
                "no_units": n.no_units,
                "bet_count": n.bet_count,
                "whale": n.yes_units.saturating_add(n.no_units) > WHALE_UNITS,
                "price_bp": n.price_bp,
                "price_marker": price_marker(n.traded, n.price_bp),
                "dominance": dominance(n.yes_units, n.no_units),
                "oracle_verified": n.oracle_verified,
                "on_golden_path": n.on_golden_path,
                "depth": depth.get(&n.tx_id).copied().unwrap_or(0),
                "children": children.get(&n.tx_id).cloned().unwrap_or_default(),
            })
        })
        .collect();

    // Summary: role activity, top contested (by bet_count), whales.
    let role_activity: Vec<serde_json::Value> = per_agent
        .iter()
        .map(|(agent, act)| {
            serde_json::json!({
                "agent": agent,
                "role": classify_agent_role(act).label(),
                "work": act.work_tx_accepted,
                "verify": act.verify_tx_accepted,
                "challenge": act.challenge_tx_accepted,
            })
        })
        .collect();
    let mut contested: Vec<&DagNode> = nodes.iter().filter(|n| n.bet_count > 0).collect();
    contested.sort_by(|a, b| b.bet_count.cmp(&a.bet_count));
    let top_contested: Vec<serde_json::Value> = contested
        .iter()
        .take(10)
        .map(|n| {
            serde_json::json!({
                "tx_id": n.tx_id, "yes": n.yes_units, "no": n.no_units, "bets": n.bet_count,
            })
        })
        .collect();
    let whales: Vec<serde_json::Value> = nodes
        .iter()
        .filter(|n| n.yes_units.saturating_add(n.no_units) > WHALE_UNITS)
        .map(|n| {
            serde_json::json!({
                "tx_id": n.tx_id, "agent": n.agent,
                "total": n.yes_units.saturating_add(n.no_units),
            })
        })
        .collect();

    let payload = serde_json::json!({
        "session_id": session_id,
        "root": {
            "node_count": nodes.len(),
            "traded": traded_count,
            "untraded": nodes.len() - traded_count,
            "roots": roots.len(),
        },
        "nodes": nodes_json,
        "golden_path": golden_path,
        "summary": {
            "role_activity": role_activity,
            "top_contested": top_contested,
            "whales": whales,
        },
    });
    Ok(Some(payload.to_string()))
}
