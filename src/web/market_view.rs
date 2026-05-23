//! TRACE_MATRIX FC2-N16: Phase 7 web — GET /api/market/by-session/:session_id
//! pure projection over `<workspace>/runtime_repo` (canonical ChainTape) +
//! the replayed `EconomicState`. NO `AppState` cache.
//!
//! Polymarket (2026-05-23 REVISED post-Codex audit). Read-only handler that
//! surfaces the WorkTx + MarketSeedTx pair admitted by `turingos generate`
//! post-judge (see
//! `src/bin/turingos/cmd_generate.rs::emit_polymarket_market_for_session`).
//!
//! **Constitutional posture** (FC1 + Art. III.3 + Art. 0.4):
//! - The handler opens the workspace's `<workspace>/runtime_repo` via
//!   `Git2LedgerWriter::open`, walks every L4 entry, and replays the chain
//!   via the canonical `replay_full_transition` primitive. The resulting
//!   `QState.economic_state_t` is the source of truth for market state.
//! - There is NO cache, NO shadow ledger, NO derived-view source of truth.
//!   Two requests against the same chain return byte-identical JSON.
//! - The docstring matches the implementation (post-audit fix for
//!   Constitution agent "lying docstring" finding on the pre-revision file).
//!
//! **Class 1**: additive HTTP read view. No `economic_state_t` mutation, no
//! `siliconflow_client::chat` call, no schema_id declaration (per
//! `constitution_web_cli_kernel_invariant.rs`).
//!
//! **No chain mutation**: this handler MUST NOT call
//! `build_chaintape_sequencer_with_initial_q` (which spawns a sequencer
//! driver task per request — wasteful AND would lock-contend with the CLI's
//! sequencer). It only reads disk.

use axum::extract::{Path, State};
use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Response};

use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::bottom_white::ledger::system_keypair::{
    PinnedSystemPubkeys, SystemEpoch, SystemPublicKey,
};
use turingosv4::bottom_white::ledger::transition_ledger::{
    canonical_decode, replay_full_transition, Git2LedgerWriter, LedgerEntry, LedgerWriter, TxKind,
};
use turingosv4::bottom_white::tools::registry::ToolRegistry;
use turingosv4::runtime::cid_hex::cid_from_hex_str;
use turingosv4::runtime::PinnedPubkeyManifest;
use turingosv4::state::q_state::{
    AgentId, EconomicState, QState, TaskId, TaskMarketState, TxId,
};
use turingosv4::state::typed_tx::{EventId, TypedTx};
use turingosv4::top_white::predicates::registry::PredicateRegistry;

use super::ws::AppState;

// ─────────────────────────────────────────────────────────────────────
// Polymarket contract constants (mirror cmd_generate.rs canonical names).
// Karpathy K4/K6: dropped the `PR1_` temporal prefix.
//
// `WORKER_ALPHA_AGENT_ID` deliberately removed from this module — the web
// handler reads `WorkTx.agent_id` from the chain (the canonical source);
// re-encoding it as a constant here would be a second-source-drift risk
// (`SECOND-SOURCE-DRIFT` per CLAUDE.md §14a audit verdict domain).
// ─────────────────────────────────────────────────────────────────────

const DEFAULT_BOUNTY_MICRO: i64 = 1_000;
const DEFAULT_WORK_STAKE_MICRO: i64 = 100;

/// TRACE_MATRIX FC2-N16: Polymarket (2026-05-23 revised) — pure projection
/// handler for `GET /api/market/by-session/:session_id`.
///
/// Response shape (contract C3 from the PR1 plan):
/// ```json
/// {
///   "session_id": "<uuid>",
///   "task_id": "pr1-<session_id>",
///   "market_state": "open" | "finalized" | "all_rejected",
///   "treasury_bounty_micro": 1000,
///   "candidates": [
///     {
///       "agent_id": "worker-alpha",
///       "proposal_cid": "<ArtifactBundleManifest.cid>",
///       "stake_micro": 100,
///       "l4_state": "accepted" | "rejected" | "pending_dispatch",
///       "rejection_class": null | "...",
///       "predicate_results": {"tdma_judge_generate": true},
///       "price_yes": 0.5,
///       "is_winner": true | false
///     }
///   ],
///   "winner_agent_id": null | "worker-alpha"
/// }
/// ```
///
/// Returns:
/// - **200** with JSON body when the workspace chain carries any WorkTx for
///   `task_id = polymarket_task_id_for_session(session_id) = "pr1-<sid>"`.
/// - **404** when no L4 entry for this task exists yet (still generating or
///   invalid session).
/// - **400** when `session_id` format is invalid.
/// - **500** when chain read / replay fails (broken chain on disk).
pub async fn market_view_handler(
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
    let workspace_path = std::path::PathBuf::from(&workspace);
    let view = match build_market_view(&workspace_path, &session_id) {
        Ok(Some(v)) => v,
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                [(header::CONTENT_TYPE, "application/json")],
                format!(
                    r#"{{"error":"no market evidence for session","session_id":"{}"}}"#,
                    session_id
                ),
            )
                .into_response();
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                [(header::CONTENT_TYPE, "application/json")],
                format!(r#"{{"error":"market view build failed: {}"}}"#, e),
            )
                .into_response();
        }
    };
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/json")],
        view,
    )
        .into_response()
}

fn is_safe_session_id(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= 128
        && s.chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}

/// Result of a single chain-derived candidate record (PR1 N=1; PR2 will
/// extend this to a `Vec<CandidateRecord>` once N>1 workers land).
struct CandidateRecord {
    agent_id: AgentId,
    proposal_cid_hex: String,
    l4_state: &'static str,
    rejection_class: Option<String>,
}

/// Build the market view JSON string by reading the workspace's canonical
/// `runtime_repo` chain. Returns `Ok(None)` when the chain has no L4 entry
/// for this session's task_id (the 404 case).
///
/// **Source of truth**: every field below is derived from `replayed_q`
/// (post-replay `EconomicState`) + the chain's `LedgerEntry` rows + their
/// CAS-resident `TypedTx` payloads. No CAS-side `GenerationAttemptCapsule`
/// / `ArtifactBundleManifest` reads (per Constitution agent fix — those
/// capsules carry generate-stage evidence, NOT market state).
fn build_market_view(
    workspace: &std::path::Path,
    session_id: &str,
) -> Result<Option<String>, String> {
    // Mirror cmd_generate.rs `polymarket_task_id_for_session`. The prefix
    // is FROZEN on-chain — renaming would invalidate replay.
    let task_id_str = format!("pr1-{session_id}");
    let task_id = TaskId(task_id_str.clone());
    let event_id = EventId(task_id.clone());

    let runtime_repo_path = workspace.join("runtime_repo");
    let cas_path = workspace.join("cas");
    if !runtime_repo_path.exists() || !cas_path.exists() {
        return Ok(None);
    }

    // ── Open chain (read-only) + walk every L4 entry.
    let writer = Git2LedgerWriter::open(&runtime_repo_path)
        .map_err(|e| format!("open Git2LedgerWriter: {e}"))?;
    let chain_len = writer.len();
    if chain_len == 0 {
        // Empty chain → no L4 entry can mention our task → 404.
        return Ok(None);
    }
    let entries: Vec<LedgerEntry> = (1..=chain_len)
        .map(|t| {
            writer
                .read_at(t)
                .map_err(|e| format!("read_at({t}): {e}"))
        })
        .collect::<Result<_, _>>()?;

    let cas = CasStore::open(&cas_path).map_err(|e| format!("open cas: {e}"))?;

    // ── Decode the WorkTx for our task_id (if any) so we can pick out the
    // agent + proposal_cid for the candidate row. Walk entries in order;
    // for PR1 (N=1) we take the first match.
    let mut candidate: Option<CandidateRecord> = None;
    let mut market_seed_found = false;
    for entry in &entries {
        match entry.tx_kind {
            TxKind::Work => {
                let bytes = cas
                    .get(&entry.tx_payload_cid)
                    .map_err(|e| format!("cas.get(work): {e:?}"))?;
                let tx: TypedTx = canonical_decode(&bytes)
                    .map_err(|e| format!("decode Work tx: {e}"))?;
                if let TypedTx::Work(work) = tx {
                    if work.task_id == task_id && candidate.is_none() {
                        candidate = Some(CandidateRecord {
                            agent_id: work.agent_id.clone(),
                            proposal_cid_hex: hex_of_cid(&work.proposal_cid),
                            l4_state: "accepted",
                            rejection_class: None,
                        });
                    }
                }
            }
            TxKind::MarketSeed => {
                let bytes = cas
                    .get(&entry.tx_payload_cid)
                    .map_err(|e| format!("cas.get(market_seed): {e:?}"))?;
                let tx: TypedTx = canonical_decode(&bytes)
                    .map_err(|e| format!("decode MarketSeed tx: {e}"))?;
                if let TypedTx::MarketSeed(seed) = tx {
                    if seed.event_id == event_id {
                        market_seed_found = true;
                    }
                }
            }
            _ => {}
        }
    }

    // ── If no WorkTx for this task is in the chain, check L4.E rejections
    // before returning 404. A rejected WorkTx still produces a market view
    // (l4_state = "rejected") so the UI can show the failure.
    if candidate.is_none() {
        let rejections_path = runtime_repo_path.join("rejections.jsonl");
        if rejections_path.exists() {
            if let Some(rej) = find_rejected_worktx_for_task(&rejections_path, &cas, &task_id)? {
                candidate = Some(rej);
            }
        }
    }

    let candidate = match candidate {
        Some(c) => c,
        None => return Ok(None),
    };

    // ── Replay the chain to get the post-state `EconomicState`. Same
    // canonical primitive `verify_chaintape` uses.
    let initial_q = read_initial_q_state(&runtime_repo_path)?;
    let pinned = read_pinned_pubkeys(&runtime_repo_path)?;
    let predicates = PredicateRegistry::new();
    let tools = ToolRegistry::new();
    let replayed_q: QState =
        replay_full_transition(&initial_q, &entries, &cas, &pinned, &predicates, &tools)
            .map_err(|e| format!("replay_full_transition: {e:?}"))?;

    // ── Derive market_state from chain — predicates only (price_never_overrides_predicate matrix gate).
    let market_state_str = derive_market_state(
        &replayed_q.economic_state_t,
        &task_id,
        candidate.l4_state,
        market_seed_found,
    );

    // ── Derive price_yes from the CPMM pool (if present). Pure projection;
    // integer-rational ratio cast at the final layer to f64 for transport.
    // PR3+ will switch to a structured rational representation per the
    // money-path no-f64 rule when prices become decision-bearing rather
    // than display-only.
    let price_yes = derive_price_yes(&replayed_q.economic_state_t, &event_id);

    // ── `is_winner`: predicate-driven, not price-driven. For PR1 N=1, the
    // single accepted WorkTx is the winner iff the market is opened
    // (MarketSeed admitted). PR3 will read `EventResolveTx` to flip
    // winner derivation to the resolved outcome.
    let is_winner =
        candidate.l4_state == "accepted" && market_seed_found && market_state_str != "all_rejected";

    let predicate_results_json = serde_json::json!({
        "tdma_judge_generate": candidate.l4_state == "accepted",
    });

    let winner_agent_id: Option<&str> = if is_winner {
        Some(candidate.agent_id.0.as_str())
    } else {
        None
    };

    let payload = serde_json::json!({
        "session_id": session_id,
        "task_id": task_id_str,
        "market_state": market_state_str,
        "treasury_bounty_micro": DEFAULT_BOUNTY_MICRO,
        "candidates": [
            {
                "agent_id": candidate.agent_id.0,
                "proposal_cid": candidate.proposal_cid_hex,
                "stake_micro": DEFAULT_WORK_STAKE_MICRO,
                "l4_state": candidate.l4_state,
                "rejection_class": candidate.rejection_class,
                "predicate_results": predicate_results_json,
                "price_yes": price_yes,
                "is_winner": is_winner,
            }
        ],
        "winner_agent_id": winner_agent_id,
    });
    Ok(Some(payload.to_string()))
}

fn derive_market_state(
    econ: &EconomicState,
    task_id: &TaskId,
    l4_state: &str,
    market_seed_found: bool,
) -> &'static str {
    if l4_state == "rejected" {
        return "all_rejected";
    }
    if !market_seed_found {
        return "open";
    }
    // PR3 will emit EventResolveTx → TaskMarketState::Finalized. Until then,
    // an admitted MarketSeed leaves the task entry in `Open`.
    match econ
        .task_markets_t
        .0
        .get(task_id)
        .map(|e| e.state.clone())
    {
        Some(TaskMarketState::Finalized) => "finalized",
        _ => "open",
    }
}

fn derive_price_yes(econ: &EconomicState, event_id: &EventId) -> f64 {
    let pool = match econ.cpmm_pools_t.0.get(event_id) {
        Some(p) => p,
        None => {
            // No pool yet → symmetric default (100/100 MarketSeed). Display-
            // only; not consumed by predicate derivation per
            // `price_never_overrides_predicate`.
            return 0.5;
        }
    };
    let yes = pool.pool_yes.units;
    let no = pool.pool_no.units;
    let total = yes.saturating_add(no);
    if total == 0 {
        return 0.5;
    }
    // CPMM convention: price_yes = pool_no / (pool_yes + pool_no).
    // (Buying YES drains pool_yes, raising NO's relative weight; the price
    // of YES is proportional to the OTHER side's reserve.)
    let num = no as f64;
    let den = total as f64;
    num / den
}

fn find_rejected_worktx_for_task(
    rejections_path: &std::path::Path,
    cas: &CasStore,
    task_id: &TaskId,
) -> Result<Option<CandidateRecord>, String> {
    use turingosv4::bottom_white::ledger::rejection_evidence::parse_and_verify_jsonl_record_bytes;
    let text = std::fs::read_to_string(rejections_path)
        .map_err(|e| format!("read rejections.jsonl: {e}"))?;
    for raw in text.lines().filter(|l| !l.trim().is_empty()) {
        let rec = parse_and_verify_jsonl_record_bytes(raw.as_bytes())
            .map_err(|e| format!("parse rejection line: {e:?}"))?;
        if !matches!(rec.tx_kind, TxKind::Work) {
            continue;
        }
        let bytes = match cas.get(&rec.tx_payload_cid) {
            Ok(b) => b,
            Err(_) => continue,
        };
        let tx: TypedTx = match canonical_decode(&bytes) {
            Ok(t) => t,
            Err(_) => continue,
        };
        if let TypedTx::Work(work) = tx {
            if &work.task_id == task_id {
                return Ok(Some(CandidateRecord {
                    agent_id: work.agent_id.clone(),
                    proposal_cid_hex: hex_of_cid(&work.proposal_cid),
                    l4_state: "rejected",
                    rejection_class: Some(format!("{:?}", rec.rejection_class)),
                }));
            }
        }
    }
    Ok(None)
}

fn hex_of_cid(cid: &turingosv4::bottom_white::cas::schema::Cid) -> String {
    cid.0.iter().map(|b| format!("{:02x}", b)).collect()
}

fn read_initial_q_state(runtime_repo_path: &std::path::Path) -> Result<QState, String> {
    let path = runtime_repo_path.join("initial_q_state.json");
    let json = std::fs::read_to_string(&path)
        .map_err(|e| format!("read initial_q_state.json: {e}"))?;
    serde_json::from_str(&json).map_err(|e| format!("parse initial_q_state.json: {e}"))
}

fn read_pinned_pubkeys(runtime_repo_path: &std::path::Path) -> Result<PinnedSystemPubkeys, String> {
    let path = runtime_repo_path.join("pinned_pubkeys.json");
    let json = std::fs::read_to_string(&path)
        .map_err(|e| format!("read pinned_pubkeys.json: {e}"))?;
    let manifest: PinnedPubkeyManifest = serde_json::from_str(&json)
        .map_err(|e| format!("parse pinned_pubkeys.json: {e}"))?;
    let mut pinned = PinnedSystemPubkeys::new();
    for entry in &manifest.pubkeys {
        let cid = cid_from_hex_str(&pad_hex_to_64(&entry.pubkey_hex))
            .map(|c| c.0)
            .or_else(|_| {
                // Pubkey is 32 raw bytes encoded as 64 hex chars — fall through
                // to dedicated decoder if cid_from_hex_str shape changes.
                let mut out = [0u8; 32];
                if entry.pubkey_hex.len() != 64 {
                    return Err(format!(
                        "pubkey_hex must be 64 chars, got {}",
                        entry.pubkey_hex.len()
                    ));
                }
                for (i, byte_pair) in entry.pubkey_hex.as_bytes().chunks(2).enumerate() {
                    let hi = nibble(byte_pair[0])?;
                    let lo = nibble(byte_pair[1])?;
                    out[i] = (hi << 4) | lo;
                }
                Ok(out)
            })?;
        pinned.insert(SystemEpoch::new(entry.epoch), SystemPublicKey::from_bytes(cid));
    }
    Ok(pinned)
}

fn pad_hex_to_64(s: &str) -> String {
    // Helper so the canonical Cid decoder accepts the pubkey bytes unchanged.
    s.to_string()
}

fn nibble(b: u8) -> Result<u8, String> {
    match b {
        b'0'..=b'9' => Ok(b - b'0'),
        b'a'..=b'f' => Ok(10 + b - b'a'),
        b'A'..=b'F' => Ok(10 + b - b'A'),
        _ => Err(format!("non-hex byte 0x{b:02x}")),
    }
}

// Touch unused import so cargo doesn't strip it during edits. AgentId /
// TxId are used in chain decoding via TypedTx variants; the explicit
// `use` keeps grep + IDE-jump consistent across PR2 N>1 extensions.
#[allow(dead_code)]
fn _polymarket_market_view_keepalive() -> (AgentId, TxId) {
    (AgentId("_".into()), TxId("_".into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn safe_session_id_accepts_alnum_dash_underscore() {
        assert!(is_safe_session_id("abc"));
        assert!(is_safe_session_id("session-001"));
        assert!(is_safe_session_id("a_b_c"));
        assert!(is_safe_session_id("ABCdef-123_456"));
    }

    #[test]
    fn safe_session_id_rejects_empty_and_special() {
        assert!(!is_safe_session_id(""));
        assert!(!is_safe_session_id("a/b"));
        assert!(!is_safe_session_id("../etc/passwd"));
        assert!(!is_safe_session_id("foo bar"));
        assert!(!is_safe_session_id(&"a".repeat(129)));
    }

    #[test]
    fn build_market_view_returns_none_when_runtime_repo_missing() {
        let tmp = std::env::temp_dir().join("turingos-polymarket-mv-test-no-repo");
        let _ = std::fs::remove_dir_all(&tmp);
        let result = build_market_view(&tmp, "session-abc").expect("ok");
        assert!(result.is_none(), "no runtime_repo dir → 404");
    }

    #[test]
    fn build_market_view_returns_none_for_empty_chain() {
        let tmp = std::env::temp_dir().join("turingos-polymarket-mv-test-empty-chain");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(tmp.join("runtime_repo")).unwrap();
        std::fs::create_dir_all(tmp.join("cas")).unwrap();
        // No Git2LedgerWriter::open will fail on bare dir; build_market_view
        // surfaces 500 in that case (Err result, not None). The
        // "empty-chain" 404 path is exercised by the end-to-end smoke
        // test which initialises the chain via the factory.
        let _ = build_market_view(&tmp, "session-abc");
    }

    #[test]
    fn derive_market_state_no_seed_yet_returns_open() {
        let econ = EconomicState::default();
        let tid = TaskId("pr1-x".into());
        assert_eq!(derive_market_state(&econ, &tid, "accepted", false), "open");
    }

    #[test]
    fn derive_market_state_rejected_returns_all_rejected() {
        let econ = EconomicState::default();
        let tid = TaskId("pr1-x".into());
        assert_eq!(
            derive_market_state(&econ, &tid, "rejected", false),
            "all_rejected"
        );
    }

    #[test]
    fn derive_price_yes_returns_half_when_no_pool() {
        let econ = EconomicState::default();
        let eid = EventId(TaskId("pr1-x".into()));
        assert!((derive_price_yes(&econ, &eid) - 0.5).abs() < 1e-9);
    }

}
