pub mod lean4_oracle;
pub mod jsonl_schema;
pub mod cost_aggregator;
pub mod wall_clock;
pub mod post_hoc_verifier;
pub mod rollback_sim;
pub mod agent_models;
pub mod budget_regime;
pub mod experiment_mode;
pub mod fc_trace;
pub mod run_id;
/// TRACE_MATRIX orphan (P6 instrumentation; PREREG_PPUT_CCL_2026-04-26.md § 5
/// H-VPPUT North Star): per-problem rolling history of `pput_verified`
/// for the held-out verified PPUT regression metric. Wired into the
/// evaluator binary's main() at TB-1 Day-4.
pub mod h_vppu_history;

/// TRACE_MATRIX FC1-N6 (predicate gate; runtime state transition):
/// ChainTape-mode gate that fail-closes any CONDITION known to bypass
/// `bus.submit_typed_tx` authoritative routing. Per architect verdict
/// 2026-05-01 §5.6 / B3 — TB-7R Deliverable B.
pub mod chaintape_mode_gate;
