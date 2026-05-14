//! TB-N3 A4 (architect ruling 2026-05-11 amendment 5 + Q7 + §8.4) — same-task
//! market context renderer for the agent prompt's `=== Market ===` block.
//!
//! Renders the top-K (default 10) `node_survive:*`-namespaced active CPMM
//! pools whose underlying WorkTx maps to the prompt's `task_id`. Each row
//! shows pool depth + integer-rational price (NEVER decimal — architect
//! "no price as truth"). Suffix banner `"price is signal, not truth"` per
//! CLAUDE.md §17 + audit_dashboard §14.
//!
//! Output is consumed by `evaluator.rs::build_agent_prompt(market_ticker)`
//! arg — caller passes the rendered string verbatim. Empty string → no
//! `=== Market ===` block in the prompt.

use crate::economy::money::MicroCoin;
use crate::state::q_state::{AgentId, ChallengeStatus, EconomicState, QState, TaskId, TxId};

/// TB-N3 A4 default K per architect Q7 ("K = 10 default; K = 5 if context
/// budget tight"). Caller-supplied via `TURINGOS_TB_N3_MARKET_CONTEXT_K` env.
pub const DEFAULT_MARKET_CONTEXT_K: usize = 10;

/// TRACE_MATRIX FC1-N7 + Art.I predicate boundary: observe-only market trace
/// hint rendered in prompts/reports; it has no predicate or sequencer authority.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct MarketTraceHint {
    pub submitted_count: u64,
    pub no_trade_count: u64,
}

/// TB-N3 A4 default top-K node-market render. Returns the empty string when
/// no eligible pool exists for `task_id` — this suppresses the
/// `=== Market ===` block in `build_agent_prompt`.
///
/// Eligibility per architect amendment 5 + Q7:
/// - `event_id.0.0.starts_with("node_survive:")` (TB-N3 A3 auto-emitted node markets)
/// - underlying `work_tx_id` is L4-accepted in `accepted_work_tx_for_task`
/// - pool status is `Active`
///
/// Sort order (architect Q7):
/// 1. liquidity depth (`pool_yes + pool_no`) descending
/// 2. recency descending (caller's `accepted_work_tx_for_task` order is
///    already insertion-order-preserving from the ledger walk; we reverse it
///    to put recent WorkTx first within the same depth bucket)
///
/// Args:
/// - `q` — current QState snapshot
/// - `task_id` — the prompt's task (filter by underlying WorkTx.task_id)
/// - `accepted_work_tx_for_task` — caller-derived list of WorkTx ids that
///   are L4-accepted under this task. Caller pre-filters from ledger walk
///   to keep this module pure (no ledger reader dependency).
/// - `k` — top-K cap (default `DEFAULT_MARKET_CONTEXT_K`)
/// - `_viewer` — agent id of the prompt recipient. Unused at MVP (no
///   per-viewer redaction); reserved for future shielding.
pub fn render_market_context(
    q: &QState,
    task_id: &TaskId,
    accepted_work_tx_for_task: &[TxId],
    k: usize,
    viewer: &AgentId,
) -> String {
    render_market_context_with_trace_hints(q, task_id, accepted_work_tx_for_task, k, viewer, &[])
}

/// TRACE_MATRIX FC1-N7 + Art.I predicate boundary: render market context with
/// observe-only trace hints and unresolved-challenge filtering; price remains
/// signal, not truth.
pub fn render_market_context_with_trace_hints(
    q: &QState,
    task_id: &TaskId,
    accepted_work_tx_for_task: &[TxId],
    k: usize,
    _viewer: &AgentId,
    trace_hints: &[(TxId, MarketTraceHint)],
) -> String {
    use std::collections::{BTreeMap, BTreeSet};

    let _ = task_id; // task_id is implicit in the caller-supplied work-tx list.
    let econ: &EconomicState = &q.economic_state_t;
    let open_challenge_targets: BTreeSet<TxId> = econ
        .challenge_cases_t
        .0
        .values()
        .filter(|case| case.status == ChallengeStatus::Open)
        .map(|case| case.target_work_tx.clone())
        .collect();
    let trace_hints: BTreeMap<TxId, MarketTraceHint> = trace_hints.iter().cloned().collect();

    // Build (work_tx_id, pool_yes, pool_no, recency_idx) for each
    // accepted WorkTx that has a matching node_survive event with an
    // active pool.
    let mut rows: Vec<(TxId, u128, u128, usize)> = Vec::new();
    for (recency_idx, work_tx_id) in accepted_work_tx_for_task.iter().enumerate() {
        if open_challenge_targets.contains(work_tx_id) {
            continue;
        }
        let event_id = crate::state::typed_tx::node_survive_event_id(work_tx_id);
        if let Some(pool) = econ.cpmm_pools_t.0.get(&event_id) {
            // Only Active pools (no Drained / Frozen markets in the
            // signal block).
            if !matches!(pool.status, crate::state::q_state::PoolStatus::Active) {
                continue;
            }
            rows.push((
                work_tx_id.clone(),
                pool.pool_yes.units,
                pool.pool_no.units,
                recency_idx,
            ));
        }
    }
    if rows.is_empty() {
        return String::new();
    }

    // Sort: depth desc; then recency desc within same depth bucket.
    rows.sort_by(|a, b| {
        let depth_a: u128 = a.1.saturating_add(a.2);
        let depth_b: u128 = b.1.saturating_add(b.2);
        depth_b.cmp(&depth_a).then_with(|| b.3.cmp(&a.3))
    });

    let mut buf = String::new();
    let take = rows.iter().take(k.max(1));
    for (work_tx_id, py, pn, _) in take {
        // Integer-rational price renders. Sum is non-zero because pool
        // creation requires `seed_yes != 0 && seed_no != 0` (architect §7.5
        // InvalidPoolSeed gate).
        let sum = py.saturating_add(*pn);
        let (price_yes_n, price_yes_d) = if sum == 0 { (0, 1) } else { (*pn, sum) };
        let (price_no_n, price_no_d) = if sum == 0 { (0, 1) } else { (*py, sum) };
        // Display: pool depths as MicroCoin micro_units (raw u128) since
        // cpmm_pool reserves are share-units (NOT MicroCoin), but the
        // share unit count IS numerically equal to the seed_micro per
        // symmetric-init. Suppress f64 entirely.
        buf.push_str(&format!(
            "- node {nid}: pool_yes={py} pool_no={pn} price_yes={pyn}/{pyd} price_no={pnn}/{pnd}\n",
            nid = work_tx_id.0,
            py = py,
            pn = pn,
            pyn = price_yes_n,
            pyd = price_yes_d,
            pnn = price_no_n,
            pnd = price_no_d,
        ));
        if let Some(hint) = trace_hints.get(work_tx_id) {
            buf.push_str(&format!(
                "  trace_submitted={} trace_no_trade={} (observe-only)\n",
                hint.submitted_count, hint.no_trade_count
            ));
        }
    }
    buf.push_str("price is signal, not truth\n");
    let _ = MicroCoin::zero(); // ensure import is used (no behavior).
    buf
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    fn fresh_q() -> QState {
        // Build a minimal QState by leveraging the test factory used by
        // bootstrap_tests — but we don't have access to it cross-crate;
        // construct directly via Default + manual mutation.
        QState::default()
    }

    fn insert_active_pool(q: &mut QState, work_tx_id: &str, py: u128, pn: u128) {
        let event_id = crate::state::typed_tx::node_survive_event_id(&TxId(work_tx_id.into()));
        q.economic_state_t.cpmm_pools_t.0.insert(
            event_id.clone(),
            crate::state::q_state::CpmmPool {
                event_id,
                pool_yes: crate::state::typed_tx::ShareAmount::from_units(py),
                pool_no: crate::state::typed_tx::ShareAmount::from_units(pn),
                lp_total_shares: crate::state::q_state::LpShareAmount::from_units(py),
                status: crate::state::q_state::PoolStatus::Active,
            },
        );
    }

    /// TB-N3 A4 U1 — empty pools → empty output (suppresses `=== Market ===`
    /// block in caller).
    #[test]
    fn empty_pool_set_returns_empty_string() {
        let q = fresh_q();
        let out = render_market_context(
            &q,
            &TaskId("task-1".into()),
            &[TxId("worktx-Agent_0-1".into())],
            DEFAULT_MARKET_CONTEXT_K,
            &AgentId("Agent_3".into()),
        );
        assert!(out.is_empty(), "no eligible pool → empty render");
    }

    /// TB-N3 A4 U2 — single active pool renders 1 row + signal-not-truth
    /// banner.
    #[test]
    fn single_pool_renders_row_plus_banner() {
        let mut q = fresh_q();
        insert_active_pool(&mut q, "worktx-Agent_0-1", 4_000_000, 4_000_000);
        let out = render_market_context(
            &q,
            &TaskId("task-1".into()),
            &[TxId("worktx-Agent_0-1".into())],
            DEFAULT_MARKET_CONTEXT_K,
            &AgentId("Agent_3".into()),
        );
        assert!(
            out.contains("node worktx-Agent_0-1"),
            "render must mention work_tx_id"
        );
        assert!(out.contains("pool_yes=4000000"), "raw u128 reserves");
        assert!(
            out.contains("price_yes=4000000/8000000"),
            "integer-rational price"
        );
        assert!(
            out.contains("price is signal, not truth"),
            "architect signal-not-truth banner"
        );
    }

    /// TB-N3 A4 U3 — top-K cap.
    #[test]
    fn top_k_cap_limits_rows() {
        let mut q = fresh_q();
        let work_tx_ids: Vec<TxId> = (0..20)
            .map(|i| TxId(format!("worktx-Agent_0-{i}")))
            .collect();
        for w in &work_tx_ids {
            insert_active_pool(&mut q, &w.0, 1_000_000, 1_000_000);
        }
        let out = render_market_context(
            &q,
            &TaskId("task-1".into()),
            &work_tx_ids,
            5,
            &AgentId("Agent_X".into()),
        );
        // Each rendered row contains a `node ` prefix.
        let rows: Vec<&str> = out.lines().filter(|l| l.starts_with("- node ")).collect();
        assert_eq!(rows.len(), 5, "K=5 cap respected");
    }

    /// TB-N3 A4 U4 — depth-descending sort.
    #[test]
    fn deeper_pools_render_first() {
        let mut q = fresh_q();
        insert_active_pool(&mut q, "worktx-Agent_0-low", 1_000_000, 1_000_000);
        insert_active_pool(&mut q, "worktx-Agent_0-high", 9_000_000, 9_000_000);
        let work_tx_ids = vec![
            TxId("worktx-Agent_0-low".into()),
            TxId("worktx-Agent_0-high".into()),
        ];
        let out = render_market_context(
            &q,
            &TaskId("task-1".into()),
            &work_tx_ids,
            DEFAULT_MARKET_CONTEXT_K,
            &AgentId("Agent_X".into()),
        );
        let high_idx = out.find("worktx-Agent_0-high").expect("high node present");
        let low_idx = out.find("worktx-Agent_0-low").expect("low node present");
        assert!(
            high_idx < low_idx,
            "depth-desc: high-depth pool renders first"
        );
    }

    /// TB-N3 A4 U5 — non-Active pool excluded.
    #[test]
    fn closed_pool_excluded() {
        let mut q = fresh_q();
        let event_id =
            crate::state::typed_tx::node_survive_event_id(&TxId("worktx-Agent_0-1".into()));
        q.economic_state_t.cpmm_pools_t.0.insert(
            event_id.clone(),
            crate::state::q_state::CpmmPool {
                event_id,
                pool_yes: crate::state::typed_tx::ShareAmount::from_units(1_000_000),
                pool_no: crate::state::typed_tx::ShareAmount::from_units(1_000_000),
                lp_total_shares: crate::state::q_state::LpShareAmount::from_units(1_000_000),
                status: crate::state::q_state::PoolStatus::Closed,
            },
        );
        let out = render_market_context(
            &q,
            &TaskId("task-1".into()),
            &[TxId("worktx-Agent_0-1".into())],
            DEFAULT_MARKET_CONTEXT_K,
            &AgentId("Agent_X".into()),
        );
        assert!(out.is_empty(), "Closed pool excluded → empty");
    }

    /// TB-N3 A4 U6 — output never contains a decimal-formatted price
    /// (architect "no price as truth"; integer-rational only).
    #[test]
    fn render_contains_no_decimal_price() {
        let mut q = fresh_q();
        insert_active_pool(&mut q, "worktx-Agent_0-1", 3_000_000, 5_000_000);
        let out = render_market_context(
            &q,
            &TaskId("task-1".into()),
            &[TxId("worktx-Agent_0-1".into())],
            DEFAULT_MARKET_CONTEXT_K,
            &AgentId("Agent_X".into()),
        );
        // No `0.5` / `0.625` / `0.375` etc.
        assert!(!out.contains("0."), "no decimal prices");
        assert!(!out.contains(".5"), "no decimal prices");
        // sanity: the integer-rational price is present
        assert!(out.contains("price_yes=5000000/8000000"));
        assert!(out.contains("price_no=3000000/8000000"));
    }

    /// Suppresses unused-import warning.
    #[test]
    fn _btreemap_import_used() {
        let _: BTreeMap<TxId, u32> = BTreeMap::new();
    }
}
