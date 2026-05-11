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

/// TRACE_MATRIX § 3 orphan (see
/// `handover/alignment/OBS_R022_TRACE_MATRIX_TB7R_ORPHANS_2026-05-02.md`):
/// ChainTape-mode pre-routing predicate gate that fail-closes any
/// CONDITION known to bypass `bus.submit_typed_tx` authoritative
/// routing. Per architect verdict 2026-05-01 §5.6 / B3 — TB-7R
/// Deliverable B. `FC-trace: Art.I.1 + Art.III.4 + WP-§5.L3 + WP-§5.L4`.
pub mod chaintape_mode_gate;

/// TB-18 Atom A (architect 2026-05-05 ruling §3 + OBS_M0_DEEPSEEK_DRIFT §5.1):
/// per-LLM-call budget enforcement (token-floor + consecutive-trivial cap +
/// aggregate per-run wall-clock cap) producing `RunOutcome::DegradedLLM`
/// halts. Wired into evaluator's LLM call sites (run_oneshot + run_swarm)
/// so drift episodes halt cleanly with EvidenceCapsule emission rather than
/// spinning until external `timeout`.
pub mod per_call_budget;

/// TB-18 Atom A (architect §3 + PRE-17.6 §6.A): re-entrant per-task driver
/// API surface. Atom A.1 ratifies the public signature; Atom B will be the
/// first caller passing a shared chain to drive multiple tasks against ONE
/// runtime_repo + ONE CAS + ONE Sequencer (architect §2.8).
pub mod drive_task;

/// TB-18 Atom B Phase 1 (architect §2.8 + §3 Atom B): `SharedChain` —
/// Kernel + BusConfig + ChaintapeBundle + AgentKeypairRegistry + TuringBus
/// initialization lifted out of `evaluator.rs::run_swarm`. Phase 1 is a
/// pure mechanical extraction (byte-identical single-task semantics);
/// Phase 2 will lift one-time chain bootstrap; Phase 3 adds `shutdown(self)`
/// and parameterizes per-task entry; Phase 4 substantive `comprehensive_arena.rs`.
pub mod chain_runtime;

/// TRACE_MATRIX FC2-Boot adjacent (TB-G G1.2-3 2026-05-11; Option B+
/// orchestration ruling): `batch_orchestrator` — binding glue between
/// G1.2-1 ResumePreflight, G1.2-2 ChainTapeLease, and the existing
/// `evaluator` binary. Provides `BatchSpec` / `TaskOutcome` /
/// `prepare_task_boundary` / `build_subprocess_env` /
/// `verify_chain_continuity` / `write_manifest_skeleton`. Does NOT
/// execute LLM-Lean cycles itself — those happen inside spawned
/// `evaluator` subprocesses. Sequential-batch use today; concurrent
/// expansion forward. Constitutional Justification:
/// `handover/directives/2026-05-11_TB_G_G1_2_OPTION_B_PLUS_RULING.md`
/// §1 + §3.1 + §3.2 + §3.3 + §3.5.
pub mod batch_orchestrator;
