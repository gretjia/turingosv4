//! TB-G G6 — price observe-only gates.

use std::path::Path;

use turingosv4::sdk::market_context::{
    render_market_context_with_trace_hints, MarketTraceHint, DEFAULT_MARKET_CONTEXT_K,
};
use turingosv4::state::q_state::{AgentId, QState, TaskId, TxId};

fn insert_active_pool(q: &mut QState, work_tx_id: &str, py: u128, pn: u128) {
    let event_id = turingosv4::state::typed_tx::node_survive_event_id(&TxId(work_tx_id.into()));
    q.economic_state_t.cpmm_pools_t.0.insert(
        event_id.clone(),
        turingosv4::state::q_state::CpmmPool {
            event_id,
            pool_yes: turingosv4::state::typed_tx::ShareAmount::from_units(py),
            pool_no: turingosv4::state::typed_tx::ShareAmount::from_units(pn),
            lp_total_shares: turingosv4::state::q_state::LpShareAmount::from_units(py),
            status: turingosv4::state::q_state::PoolStatus::Active,
        },
    );
}

#[test]
fn sg_g6_1_market_context_renders_trace_hints_as_observe_only_signal() {
    let mut q = QState::default();
    insert_active_pool(&mut q, "worktx-Agent_0-1", 4_000_000, 6_000_000);
    let out = render_market_context_with_trace_hints(
        &q,
        &TaskId("task-1".into()),
        &[TxId("worktx-Agent_0-1".into())],
        DEFAULT_MARKET_CONTEXT_K,
        &AgentId("Agent_2".into()),
        &[(
            TxId("worktx-Agent_0-1".into()),
            MarketTraceHint {
                submitted_count: 2,
                no_trade_count: 3,
            },
        )],
    );
    assert!(out.contains("trace_submitted=2"));
    assert!(out.contains("trace_no_trade=3"));
    assert!(out.contains("price is signal, not truth"));
    assert!(
        !out.contains("0."),
        "prices and rates must not render as decimals"
    );
}

#[test]
fn sg_g6_4_predicates_do_not_read_market_price_or_trace() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut offenders = Vec::new();
    let mut stack = vec![root.join("src/top_white/predicates")];
    while let Some(path) = stack.pop() {
        for entry in std::fs::read_dir(&path).expect("read predicate dir") {
            let entry = entry.expect("dir entry");
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                let text = std::fs::read_to_string(&path).expect("read predicate source");
                for forbidden in ["cpmm_pools_t", "MarketDecisionTrace", "price_index"] {
                    if text.contains(forbidden) {
                        offenders.push(format!("{} contains {forbidden}", path.display()));
                    }
                }
            }
        }
    }
    assert!(
        offenders.is_empty(),
        "G6 price is observe-only; predicates must not read price/market/trace: {offenders:?}"
    );
}
