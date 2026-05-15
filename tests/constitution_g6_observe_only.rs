//! TB-G G6 — price observe-only gates.

use std::path::Path;

use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::runtime::agent_scheduler::{
    build_observe_only_scheduler_trace, read_scheduler_decision_trace_from_cas,
    render_scheduler_trace_section, scheduler_decision_trace_cids,
    write_scheduler_decision_trace_to_cas, SchedulerPnlSignal, SCHEDULER_DECISION_TRACE_SCHEMA_ID,
};
use turingosv4::runtime::real5_roles::{AgentRole, PriceSignal};
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

#[test]
fn sg_6d_1_and_6d_2_scheduler_trace_carries_price_and_pnl_observe_only() {
    let trace = build_observe_only_scheduler_trace(
        "head-42".to_string(),
        vec![AgentId("Agent_0".into()), AgentId("Agent_1".into())],
        vec![TxId("worktx-Agent_0-1".into())],
        vec![PriceSignal {
            event_id: "task_outcome:task-1".into(),
            price: "3/5".into(),
            depth: Some(250_000),
        }],
        vec![SchedulerPnlSignal {
            agent_id: AgentId("Agent_1".into()),
            realized_pnl: -10_000,
            unrealized_pnl: 25_000,
            available_micro: 990_000,
            risk_cap_micro: 100_000,
        }],
        Some(AgentId("Agent_1".into())),
        Some(AgentRole::Trader),
        Some("consider_task_outcome_market".into()),
    );

    assert!(trace.observe_only, "SG-6D.2 observe_only=true");
    assert_eq!(trace.head_t, "head-42");
    assert_eq!(trace.price_signals.len(), 1, "SG-6D.1 price signal");
    assert_eq!(trace.pnl_signals.len(), 1, "SG-6D.1 PnL signal");
    assert_eq!(
        trace.recommended_agent.as_ref().map(|a| a.0.as_str()),
        Some("Agent_1")
    );
    assert_eq!(trace.recommended_role, Some(AgentRole::Trader));
    assert_eq!(
        trace.recommended_action.as_deref(),
        Some("consider_task_outcome_market")
    );
}

#[test]
fn sg_6d_5_dashboard_scheduler_section_is_non_binding_materialized_view() {
    let trace = build_observe_only_scheduler_trace(
        "head-7".to_string(),
        vec![AgentId("Agent_0".into())],
        vec![TxId("node-1".into())],
        vec![PriceSignal {
            event_id: "event-1".into(),
            price: "1/2".into(),
            depth: Some(1_000),
        }],
        vec![SchedulerPnlSignal {
            agent_id: AgentId("Agent_0".into()),
            realized_pnl: 0,
            unrealized_pnl: 0,
            available_micro: 1_000_000,
            risk_cap_micro: 100_000,
        }],
        Some(AgentId("Agent_0".into())),
        Some(AgentRole::Trader),
        Some("observe_market".into()),
    );
    let rendered = render_scheduler_trace_section(&trace);

    assert!(rendered.contains("Opportunity Scheduler"));
    assert!(rendered.contains("observe_only: true"));
    assert!(rendered.contains("non-binding"));
    assert!(rendered.contains("price is signal, not truth"));
    assert!(
        !rendered.contains("model ranking") && !rendered.contains("admission override"),
        "SG-6D.5 dashboard must not present scheduler recommendation as authority"
    );
}

#[test]
fn sg_6d_1_scheduler_trace_is_chain_backed_cas_evidence() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let mut cas = CasStore::open(tmp.path()).expect("cas open");
    let trace = build_observe_only_scheduler_trace(
        "HEAD_t:{\"run_id\":\"real6d-test\"}".to_string(),
        vec![AgentId("Agent_0".into()), AgentId("Agent_1".into())],
        vec![TxId("task_outcome:task-1".into())],
        vec![PriceSignal {
            event_id: "task_outcome:task-1".into(),
            price: "1/2".into(),
            depth: Some(2_000_000),
        }],
        vec![SchedulerPnlSignal {
            agent_id: AgentId("Agent_1".into()),
            realized_pnl: 10_000,
            unrealized_pnl: 0,
            available_micro: 1_010_000,
            risk_cap_micro: 100_000,
        }],
        Some(AgentId("Agent_1".into())),
        Some(AgentRole::Trader),
        Some("observe_task_outcome_market".into()),
    );

    let cid = write_scheduler_decision_trace_to_cas(&mut cas, &trace, "sg-6d-1", 42)
        .expect("scheduler trace CAS write");
    let metadata = cas.metadata(&cid).expect("scheduler trace metadata");
    assert_eq!(
        metadata.schema_id.as_deref(),
        Some(SCHEDULER_DECISION_TRACE_SCHEMA_ID),
        "SG-6D.1 scheduler trace must carry a stable CAS schema id"
    );
    assert!(
        tmp.path().join(".git/refs/chaintape/cas").exists(),
        "SG-6D.1 CasStore::put must advance refs/chaintape/cas"
    );

    let decoded =
        read_scheduler_decision_trace_from_cas(&cas, &cid).expect("scheduler trace CAS read");
    assert_eq!(decoded, trace);
    assert_eq!(scheduler_decision_trace_cids(&cas), vec![cid]);
    assert!(decoded.observe_only);
    assert_eq!(decoded.price_signals.len(), 1);
    assert_eq!(decoded.pnl_signals.len(), 1);
}

#[test]
fn sg_6d_3_and_6d_4_scheduler_does_not_touch_admission_or_predicates() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let scheduler_src =
        std::fs::read_to_string(root.join("src/runtime/agent_scheduler.rs")).expect("scheduler");
    for forbidden in [
        "submit_agent_tx",
        "dispatch_transition",
        "predicate_passes",
        "Sequencer",
        "TypedTx",
    ] {
        assert!(
            !scheduler_src.contains(forbidden),
            "SG-6D.3/6D.4 scheduler observe-only helper must not touch {forbidden}"
        );
    }

    let dashboard_src =
        std::fs::read_to_string(root.join("src/bin/audit_dashboard.rs")).expect("dashboard");
    assert!(
        dashboard_src.contains("render_scheduler_trace_section"),
        "SG-6D.5 dashboard must render scheduler recommendation as non-binding view"
    );
    assert!(
        !dashboard_src.contains("format!(\"tx_count:{}\", report.run_facts.tx_count)"),
        "SG-6D.1 dashboard must not use tx_count as a pseudo HEAD_t witness"
    );
    assert!(
        dashboard_src.contains("scheduler_head_t"),
        "SG-6D.1 dashboard must build scheduler trace from an explicit head_t witness label"
    );
    assert!(
        dashboard_src.contains("scheduler_price_signals.push")
            && dashboard_src.contains("task_outcome"),
        "SG-6D.1 dashboard scheduler trace must include TaskOutcomeMarket price signals, not only node_survive pools"
    );
    assert!(
        dashboard_src.contains("scheduler_decision_trace_cids")
            && dashboard_src.contains("persisted_scheduler_trace_cas_count"),
        "SG-6D.1 dashboard must surface persisted SchedulerDecisionTrace CAS anchors"
    );
}

#[test]
fn sg_6d_1_runtime_emits_scheduler_trace_to_run_cas() {
    let evaluator_src =
        std::fs::read_to_string("experiments/minif2f_v4/src/bin/evaluator.rs").unwrap();
    assert!(
        evaluator_src.contains("TURINGOS_REAL6_SCHEDULER_OBSERVE_ONLY"),
        "REAL-6D runtime emission must be explicitly feature-gated"
    );
    assert!(
        evaluator_src.contains("write_scheduler_decision_trace_to_cas_or_exit")
            && evaluator_src.contains("write_scheduler_decision_trace_to_cas")
            && evaluator_src.contains("SchedulerDecisionTrace CAS write FAIL-CLOSED"),
        "SG-6D.1 scheduler trace must be emitted to the run CAS evidence path fail-closed"
    );
    assert!(
        evaluator_src.contains("real6d_scheduler_price_signals_from_q")
            && evaluator_src.contains("real6d_scheduler_pnl_signal_from_budget"),
        "SG-6D.1 runtime trace must derive both price and PnL signals from canonical run state"
    );
}
