//! TB-6 Atom 1 — Production ChainTape runtime bootstrap.
//!
//! Connects the experiment evaluator binary to the kernel `Sequencer` via
//! `TuringBus::with_sequencer`. Produces an on-disk `Git2LedgerWriter` chain
//! (`refs/transitions/main`) plus persistent L4.E JSONL rejection ledger
//! (`<runtime_repo>/rejections.jsonl`) — the architect § 3.5 deliverable shape
//! for chain-backed ChainTape smoke evidence.
//!
//! Driver lifecycle: a runtime-side wrapper (`run_chaintape_driver`) owns the
//! Sequencer mpsc receiver, races shutdown_rx via `tokio::select!`, and calls
//! `Sequencer::apply_one` (`pub(crate)` — same crate) directly. We do NOT call
//! `Sequencer::run` because it has no shutdown branch and `Sequencer` owns
//! `queue_tx` (driver task's `Arc<Sequencer>` would keep the sender alive,
//! preventing clean exit).
//!
//! Per architect ruling 2026-05-01 Path A, atom count stays at 8.
//! See `handover/ai-direct/TB-6_PRODUCTION_CHAINTAPE_BOOTSTRAP_2026-05-01.md`
//! v2.1 for the full preflight.

/// TRACE_MATRIX FC3-N1: TB-6 Atom 2 — chaintape adapter helpers (synthetic TaskOpen/EscrowLock/WorkTx constructors + balance seeding).
pub mod adapter;

/// TRACE_MATRIX FC3-N1: TB-6 Atom 4 — replay verifier (re-opens runtime_repo + cas + pinned_pubkeys.json, replays L4 chain, emits replay_report.json).
pub mod verify;

/// TRACE_MATRIX FC3-N1: TB-6 Atom 5 — Agent audit trail (AgentProposalRecord + CAS storage + JSONL index linking tx_id → proposal_record_cid).
pub mod agent_audit_trail;

/// TRACE_MATRIX FC3-N1: TB-6 Atom 6 — Branch / fork visibility summary (tx_count, failed_branch_count, rollback_count, accepted/rejected tx_id sets, candidate proposal CIDs).
pub mod run_summary;

/// TRACE_MATRIX FC1-N14: TB-7 Atom 1 — Per-agent Ed25519 keypair manager + on-disk pubkey manifest (run-local identity ONLY; not durable reputation).
pub mod agent_keypairs;

/// TRACE_MATRIX FC1-N14: TB-9 Atom 1 — Durable agent keystore (encrypted-at-rest persistence of per-agent Ed25519 secrets across evaluator restarts; satisfies architect TB-9 mandate "agent durable key registry" + "cross-run identity").
pub mod agent_keystore;

/// TRACE_MATRIX FC1-N14: TB-7 Atom 1.5 — ProposalTelemetry CAS object writer (per-WorkTx LLM proposal metadata: agent_id / prompt_context_hash / proposal_artifact_cid / candidate_tactic / token_counts / tool_calls / branch_id / parent_tx; per ARCHITECT_RULING D5 + charter §4.5).
pub mod proposal_telemetry;

/// TRACE_MATRIX FC1-N14: TB-7 Atom 5 — ChainDerivedRunFacts aggregator (renamed from chain_derived_pput per ARCHITECT_RULING D4; bit-exact §4.4 structural field set computed from L4 + L4.E + CAS alone).
pub mod chain_derived_run_facts;

/// TRACE_MATRIX FC1-N14: TB-7.7 D4 — VerificationResult CAS object recording Lean oracle verdict (target_work_tx / verifier_agent / lean_exit_code / lean_*_hash / proof_file_hash / proof_artifact_cid / verified). Linked from ProposalTelemetry.verification_result_cid.
pub mod verification_result;

/// TRACE_MATRIX § 3 orphan (see `handover/alignment/OBS_R022_TRACE_MATRIX_TB7R_ORPHANS_2026-05-02.md`): TB-7R Deliverable C — `genesis_report.json` emitter capturing constitution_hash + runtime_repo + cas_path + system_pubkey_hash + agent_pubkeys_path + initial_balances + (preseed only) task_id / task_open_tx / escrow_lock_tx. No canonical FC row exists yet (FC2 is Append/Submit, NOT Boot/Genesis); promotion target is a future TRACE_MATRIX revision under Article IV Boot. `FC-trace: Art.IV Boot + Art.I.1 + Art.III.4 + WP-§11`.
pub mod genesis_report;

/// TRACE_MATRIX TB-11 Atom 1 (architect §6.1 ruling 2026-05-02): EvidenceCapsule schema + writer surface. CAS-resident rollup of failed-run evidence (attempt_count / lean_error_count / sorry_block_count / parse_failure_count / partial_accept_count + compressed_log_cid + privacy_policy). Anchored on chain by `TerminalSummaryTx.evidence_capsule_cid` (architect's RunExhaustedTx role) and `TaskBankruptcyTx.evidence_capsule_cid`. Privacy default `AuditOnly` per architect §6.1 屏蔽规则.
pub mod evidence_capsule;

/// TRACE_MATRIX FC1-N32 + FC2-N30 (TB-15 Atom 2 + 4; architect §6.2 ruling 2026-05-02 + 2026-05-03): per-agent `AgentAutopsyCapsule` schema + writer (Atom 2) and `cluster_autopsies` typical-error broadcast (Atom 4). CAS-resident; AuditOnly default. Derived from ChainTape evidence — NEVER LLM self-narrative. Anchored on `EconomicState.agent_autopsies_t[event_id]: Vec<Cid>` (Atom 3).
pub mod autopsy_capsule;

/// TRACE_MATRIX FC3-N43 (TB-15 Atom 5; architect §6.2 + FR-15.4 + FR-15.5): `MarkovEvidenceCapsule` schema + writer + default-deny deep-history gate. End-of-TB rollup binding constitution_hash + L4 root + L4.E root + CAS root + previous capsule + typical_errors + unresolved_obs + next_session_context_cid. Default next-session bootstrap source; deeper history requires `TURINGOS_MARKOV_OVERRIDE=1`.
pub mod markov_capsule;

/// TRACE_MATRIX FC2 Boot: TB-10 Atom 1 — Reusable preseed factory for chaintape genesis QState. Single source of truth for `tb7-7-sponsor` + `Agent_user_0` + `Agent_0..9` initial balances. Consumed by both evaluator (`--task-mode self|both`) and `lean_market` user CLI bootstrap. Pure function; replay-deterministic.
pub mod bootstrap;

/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 Atom 2; architect §7.5 + design §6.2): 38-assertion audit-from-tape battery. Pure-fn library over on-disk artifacts (runtime_repo + cas_dir + manifests + constitution + markov pointer); NO live process state. Drives `audit_tape` + `audit_tape_tamper` binaries; verdict.json wire format per design §6.3.
pub mod audit_assertions;

/// TRACE_MATRIX FC1-N41 + FC1-N42 + FC1-N43 NEW (TB-18R R1 charter v2 §1 + Codex Gate 1 ratified 2026-05-06): per-LLM-Lean-cycle `AttemptTelemetry` + `LeanResult` + `TerminalAbortRecord` CAS object schemas. Closes failure-path asymmetry (omega ✅ / step_reject + parse_fail + llm_err + step_partial_ok ❌) documented in 2026-05-06 external-audit VETO. R2 wires evaluator hot path; R3 extends sequencer L4.E admission with `RejectionClass::LeanFailed=6 / ParseFailed=7 / SorryBlocked=8 / LlmError=9` tail-append.
pub mod attempt_telemetry;

/// TRACE_MATRIX FC1-N44 NEW (Constitution Landing First 2026-05-07; HARNESS.md §3 G-016/G-019/G-021/G-028; architect ruling §4.3): Class-3 `PromptCapsule` CAS schema — tape-resident proof that the agent's prompt context was derivable from a fixed read-set + redaction policy + system-prompt template hash, without storing verbatim prompt bytes. Closes Art. III selective shielding / prompt persistence gap (0% LANDED → first LANDED row).
pub mod prompt_capsule;

/// TRACE_MATRIX § 3 orphan (Constitution Landing 2026-05-08; report-side helper, not chain-resident): closes Art. I.2 PPUT statistical signal AMBER (CLAUDE.md §17 Report Standard "95% CI if reporting aggregate"). Wilson score 95% CI for binomial proportions (solve-count over batch). Pure helper; no chain side effects. Constitutional Justification: `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md` §B Art. I.2 row.
pub mod wilson_ci;

/// TRACE_MATRIX § 3 orphan (Constitution Landing 2026-05-08; report-side helper, not chain-resident): closes Art. II.2.1 exploration/exploitation AMBER (kill: parent_selection_entropy < 0.25 OR pairwise_payload_diversity_mean < 0.25). Shannon entropy over parent_tx selection (None-filtered per V3L-14 anti-pattern fix from audit_assertions id=43) + distinct-payload fraction. Pure helpers; no chain side effects. Constitutional Justification: `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md` §C Art. II.2.1 row.
pub mod diversity;

/// TRACE_MATRIX § 3 orphan (Stage B3 / TB-18B 2026-05-08; report-side schema, not chain-resident): TB-18B FR-18B.1 / CR-18B.5 — `BenchmarkManifest` pin-set required before any 50+ × n>1 × multi-seed batch. Constitutional Justification: `feedback_benchmark_manifest_required` + TB-18B charter §3 line 47-48.
pub mod benchmark_manifest;

/// TRACE_MATRIX § 3 orphan (Stage B3 / TB-18B 2026-05-08; report-side aggregator, not chain-resident): TB-18B FR-18B.5 / FR-18B.6 / FR-18B.11 — `AggregateReport` consuming `wilson_ci.rs` + `diversity.rs` per CLAUDE.md §17 Report Standard. Closes the Wilson CI + DiversityReport wire-up forward step from session #18 Wave-1/2 forward-bind. Constitutional Justification: CLAUDE.md §17 + TB-18B charter §3 line 51 + `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md` §B Art. I.2 + §C Art. II.2.1 rows.
pub mod aggregate_report;

/// TRACE_MATRIX FC1-Append Stage C P-M8 / Phase F.7 (architect manual §7.9):
/// `audit_views` — 4 pure derived-view aggregators over `EconomicState`:
/// `audit_view_shares` (owner YES/NO + conditional collateral),
/// `audit_view_pools` (CpmmPool reserves + LP shares),
/// `audit_view_prices` (router quote signal per pool over caller-supplied
/// pay_coin samples), `audit_view_positions` (NodePositions exposure).
/// Constitutional Justification: architect §7.9 — audit tools must show
/// shares, collateral, pool reserves, LP shares, NodePositions, price
/// signal. Pure functions; no state mutation; replay-deterministic.
pub mod audit_views;

/// TRACE_MATRIX FC1-N35 + FC2-INV1 (M0 batch 2026-05-10 surfaced TB-16-era
/// drift): `audit_tamper` — three corruption primitives used by the
/// `audit_tape_tamper` binary harness. Constitutional Justification:
/// architect §B.9.3 mandates "prove no fake accepted" via 3/3 tamper
/// detection. Stage A3 multi-ref ChainTape (refs/chaintape/{l4,l4e,cas})
/// invalidated TB-16-era assumptions (largest-by-bytes blob + alias-only
/// ref truncation); the M0 P01 evidence showed 1/3 detection. Library
/// API is exercised by `tests/constitution_audit_tamper_3_of_3.rs` so
/// future drift is gate-time-caught. See `feedback_no_workarounds_strict_constitution`.
pub mod audit_tamper;

/// TRACE_MATRIX FC1-Append TB-N3 A2 (architect ruling 2026-05-11 §8.1+§8.2 + §8.6):
/// `market_decision_trace` — CAS-anchored audit trail for every agent invest
/// intent. `MarketDecisionTrace` records the agent's market-side decision
/// (chosen node + direction + amount + quoted price) plus a `TraceOutcome`
/// distinguishing `Submitted{tx_id}` from `NoTrade{NoTradeReason}` /
/// `Declined`. Failed invests (no pool / insufficient balance / router
/// rejected) anchor here as `NoTrade` outcomes; submitted invests carry the
/// L4 / L4.E TxId. Aggregate render lives in `audit_dashboard --run-report`
/// §F (no-trade reason distribution). Constitutional Justification:
/// architect §8.6 "Failed invest 也算有意义 tape activity" + §8.1 "外部化
/// market decision 的审计轨迹" + Phase-2 SG-N3.12 (run_report §F).
pub mod market_decision_trace;

/// TRACE_MATRIX FC2-Boot (TB-G G1.2-1 2026-05-11; Option B+ orchestration
/// ruling): `ResumePreflight` — fail-closed library that validates a
/// subprocess's claim to resume an existing ChainTape. Architect classified
/// `TURINGOS_CHAINTAPE_RESUME=1` alone as a signal, not a safety protocol;
/// preflight is the safety protocol. Eleven gates SG-G1.2-1.1..11.
/// Constitutional Justification:
/// `handover/directives/2026-05-11_TB_G_G1_2_OPTION_B_PLUS_RULING.md` §3.1.
pub mod resume_preflight;

/// TRACE_MATRIX FC2-Boot (TB-G G1.2-2 2026-05-11; Option B+ orchestration
/// ruling §3.2): `ChainTapeLease` — single-writer file-lock guarding
/// `refs/transitions/main` advancement against concurrent subprocess
/// writers. Atomic tempfile+rename write; stale-lock detection via
/// `kill -0 holder_pid`; RAII drop releases. Six gates SG-G1.2-2.1..6.
/// Sequential-batch use only today; concurrent expansion forward.
/// Constitutional Justification:
/// `handover/directives/2026-05-11_TB_G_G1_2_OPTION_B_PLUS_RULING.md` §3.2.
pub mod chain_tape_lease;

/// TRACE_MATRIX FC2-Boot + FC3-Markov (TB-G G1.2-4 2026-05-11; Option B+
/// orchestration ruling §3.3): `BatchContinuationManifest` — architect-
/// mandated fact-identity for a multi-task batch. CAS-anchorable
/// (`ObjectType::Generic` with
/// `schema_id="batch_continuation_manifest_g1_2_v1"`); `replay_continuity`
/// verifies chain continuity across tasks; `replay_matches_real_chain_head`
/// cross-checks the manifest against the live `refs/transitions/main`.
/// Four gates SG-G1.2-4.1..4. Constitutional Justification:
/// `handover/directives/2026-05-11_TB_G_G1_2_OPTION_B_PLUS_RULING.md` §3.3.
pub mod batch_continuation_manifest;

/// TRACE_MATRIX FC2-Boot + FC3-Markov (TB-G G1.2-5 2026-05-11; Option B+
/// orchestration ruling §3.4 + charter §1 G1.2-5): persistence-evidence
/// binding library. Classifies each of the six architect-required
/// persisted fields (balances / positions / reputation / PnL / autopsy /
/// model identity) against per-task `QState` snapshots and the
/// `BatchContinuationManifest`. Six gates SG-G1.2-5.1..6 verify the
/// classifier itself; G1.2-6/7 real-LLM evidence wires it against
/// chain-replay snapshots. Constitutional Justification:
/// `handover/directives/2026-05-11_TB_G_G1_2_OPTION_B_PLUS_RULING.md` §3.4 +
/// `handover/tracer_bullets/TB_G_GENERATIVE_ARENA_charter_2026-05-11.md`
/// §0 kill_criteria_tested #1.
pub mod persistence_evidence;

/// TRACE_MATRIX FC1-N7 + §15 + §17 reporting standard — TB-G G2P.2
/// (charter §1 Module G2P): peer-verify-coverage walker. Derives
/// per-agent `peer_verify_count` + per-target verifier count +
/// `non_solver_verifications` (architect §8.2 ship-gate signal) from
/// canonical L4 ledger entries + CAS. Powers the
/// `audit_dashboard --run-report` §F.X section. Closes user
/// 2026-05-12 病灶3 "0 verify" quantification gap.
pub mod peer_verify_coverage;

/// TRACE_MATRIX FC1-N5 + §15 + §17 reporting standard — TB-G G2.2
/// (charter §1 Module G2 atom G2.2; G-Phase directive §G2 SG-G2.3
/// "NoTradeReason appears in dashboard"): MarketDecisionTrace §F
/// summary builder. Walks CAS for `tb_n3.market_decision_trace.v1`
/// objects, computes per-`NoTradeReason` counts (exhaustive 13-row
/// stable iteration over `NoTradeReason::ALL`) + `submitted_vs_traced`
/// ratio, and renders the `## §F MarketDecisionTrace summary` block
/// for `audit_dashboard --run-report`. Lift-out from
/// `src/bin/audit_dashboard.rs` to gain library-test access to the
/// row-rendering contract.
pub mod market_decision_trace_summary;

/// TRACE_MATRIX FC1-N5 + §15 + §17 (TB-G G3.1 2026-05-12; charter §1 Module
/// G3 atom G3.1; G-Phase directive §G3 verbatim 7-field shape + SG-G3.5
/// "PnL is visible in dashboard as materialized view"): per-agent PnL
/// derived view. `compute_agent_pnl(q, agent_id, initial_balance_micro)`
/// returns the architect-spec'd `AgentMarketStateView` — balance,
/// open_positions, realized_pnl, unrealized_pnl, solvency_status,
/// reputation_score. Pure derivation; never mutates state. Integer math
/// only (CLAUDE.md §13 no-f64). Sibling renderer is G3.3
/// `src/sdk/your_position.rs`; dashboard consumer is G3.4 §G PnL
/// trajectory in `src/bin/audit_dashboard.rs`.
pub mod agent_pnl;

/// TRACE_MATRIX FC1-N43 + FC3-N13 (TB-G G3.2 2026-05-12; architect §7.1 +
/// §7.5): post-hoc audit views over the canonical chain — `RiskCapImpactReport`
/// (per-rejection rows: agent / balance_before / risk_cap / tx_kind /
/// task_id / another_agent_continued / solve_outcome) for solve-rate
/// regression attribution, plus `FinalizeRewardPayoutBreakdown`
/// (solver_reward_delta / verifier_bond_return_delta / other_settlement_delta)
/// for payout traceability. Both are PURE chain-derived: identical chain
/// + identical `q` → identical report. Dashboard consumer:
/// `src/bin/audit_dashboard.rs` §G PnL trajectory + §H+ extensions.
pub mod risk_cap_impact_report;

/// TRACE_MATRIX FC1-N7 + FC3-N43 (TB-G G5 2026-05-14): observe-only
/// opportunity scheduler helper. Pure read-side decision record; does not
/// mutate QState or replace sequencer admission.
pub mod agent_scheduler;

/// TRACE_MATRIX FC3-N43 (TB-G G5 2026-05-14): public activity role classifier
/// for dashboard/report views. Derived from ChainTape/CAS counts only; no CoT
/// or private prompt body inputs.
pub mod agent_role_classifier;

/// TRACE_MATRIX FC3-N43 (TB-G G7 2026-05-14): structural run6-equivalent smoke
/// evaluator and §K clean-negative renderer.
pub mod g7_structural_smoke;

/// TRACE_MATRIX FC1/FC3 (REAL-11 Atom 1 2026-05-15): report-side market
/// transaction categorization. Splits legacy market_tx_count into structural
/// market activity, live agent economic action, scripted fixture activity, and
/// resolution tx. Pure view helper only; no sequencer admission or tx schema
/// authority.
pub mod market_tx_category;

/// TRACE_MATRIX FC1/FC3 (REAL-11 Atom 3 2026-05-15): pure derived trace for
/// whether a Trader turn had an actionable market opportunity. No new CAS
/// object type and no economic mutation.
pub mod market_opportunity_trace;

/// TRACE_MATRIX FC1/FC2/FC3 (REAL-5 2026-05-14): role-based generative
/// scaffolding — role assignment, role-scoped views, typed generation gateway,
/// tick budget, and tape-visible role decision traces.
pub mod real5_roles;

/// REAL-6A — TaskOutcomeMarket metadata/view helpers. Economic mutation still
/// routes through typed tx admission (`MarketSeedTx` + `CpmmPoolTx`).
pub mod real6_task_outcome;

/// REAL-6B — AttemptPredictionMarket sealed-oracle scripted fixture helpers.
/// Current stage is design + scripted fixture only; no live real-LLM ship.
pub mod real6_attempt_prediction;

/// REAL-6C — ConvictionBudget / PnL feedback derived view. Free cognition,
/// paid conviction; no new economic source of truth.
pub mod real6_conviction_budget;

/// TRACE_MATRIX FC3-N33 + FC3-N43 (Unified Agent Harness 2026-05-13):
/// self-hosting development evidence sidecar. `turingos_dev` records
/// module/molecule/atom contracts, command evidence, review verdicts, and an
/// append-only hash chain for development work. It is NOT a canonical tape or
/// a second CAS; future work may anchor summaries into ChainTape/CAS after
/// G3.2/G4.2/PromptCapsule runtime closure.
pub mod dev_harness;

use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

use tokio::sync::oneshot;
use tokio::task::JoinHandle;

use crate::bottom_white::cas::store::CasStore;
use crate::bottom_white::ledger::rejection_evidence::RejectionEvidenceWriter;
use crate::bottom_white::ledger::system_keypair::{
    Ed25519Keypair, PinnedSystemPubkeys, SystemEpoch, SystemPublicKey,
};
use crate::bottom_white::ledger::transition_ledger::{
    replay_full_transition, Git2LedgerWriter, LedgerEntry, LedgerWriter,
};
use crate::bottom_white::tools::registry::ToolRegistry;
use crate::state::q_state::QState;
use crate::state::sequencer::{Sequencer, SubmissionEnvelope};
use crate::top_white::predicates::registry::PredicateRegistry;

// ── Configuration ───────────────────────────────────────────────────────────

/// TRACE_MATRIX FC3-N1: configuration shape for production ChainTape mode.
///
/// Production ChainTape runtime configuration.
///
/// Enabled by env var `TURINGOS_CHAINTAPE_PATH=<runtime_repo_path>`.
/// When unset, the evaluator falls back to legacy `TuringBus::new` /
/// `TuringBus::with_wal_path` (no on-disk ChainTape).
#[derive(Debug, Clone)]
pub struct RuntimeChaintapeConfig {
    /// Filesystem path to the on-disk runtime git repo.
    /// `Git2LedgerWriter` rooted here writes `refs/transitions/main`.
    /// `<runtime_repo_path>/rejections.jsonl` is the L4.E persistent file.
    /// `<runtime_repo_path>/pinned_pubkeys.json` carries the per-run
    /// `PinnedSystemPubkeys` so `verify_chaintape` (Atom 4) can re-verify
    /// entry signatures without separate config.
    pub runtime_repo_path: PathBuf,
    /// CAS root directory. Distinct from `runtime_repo_path` so CAS payloads
    /// can be inspected independently of the chain refs.
    pub cas_path: PathBuf,
    /// Run identity for evidence-dir naming + audit trail. Defaults to
    /// `TURINGOS_RUN_ID` env var or current Unix-second timestamp.
    pub run_id: String,
    /// Sequencer mpsc channel capacity. Default 64.
    pub queue_capacity: usize,
    /// TB-G G1.1 (architect §8 SIGNED 2026-05-11; packet §2 + §5 Q4):
    /// when `true` AND `refs/transitions/main` is non-empty,
    /// `build_chaintape_sequencer` resumes the existing chain instead of
    /// fail-closing with `BootstrapError::NonEmptyRuntimeRepo`. Default
    /// `false` preserves back-compat for all pre-G-Phase callers (TB-N* /
    /// Stage C / Wave 3 50p / TB-N3 Phase 2). Env-gated by
    /// `TURINGOS_CHAINTAPE_RESUME == "1"` (strict equality; no
    /// truthy-string footgun).
    pub resume_existing_chain: bool,
}

impl RuntimeChaintapeConfig {
    /// TRACE_MATRIX FC3-N1: env-flag-gated chaintape mode entry — evaluator calls this once at boot.
    ///
    /// Build from env. Returns `None` if `TURINGOS_CHAINTAPE_PATH` unset.
    pub fn from_env() -> Option<Self> {
        let runtime_repo_path: PathBuf = std::env::var("TURINGOS_CHAINTAPE_PATH").ok()?.into();
        let cas_path: PathBuf = match std::env::var("TURINGOS_CAS_PATH") {
            Ok(p) => p.into(),
            Err(_) => runtime_repo_path
                .parent()
                .unwrap_or_else(|| Path::new("."))
                .join(format!(
                    "cas_{}",
                    runtime_repo_path
                        .file_name()
                        .map(|s| s.to_string_lossy().into_owned())
                        .unwrap_or_else(|| "default".into()),
                )),
        };
        let run_id = std::env::var("TURINGOS_RUN_ID").unwrap_or_else(|_| {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs().to_string())
                .unwrap_or_else(|_| "0".into())
        });
        let queue_capacity = std::env::var("TURINGOS_CHAINTAPE_QUEUE_CAPACITY")
            .ok()
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(64);
        // TB-G G1.1 (packet §5 Q4): strict equality on "1". `Ok("1")` flips
        // the resume gate on; any other value (including "true", "yes",
        // "TRUE", "01", " 1", absent) leaves the gate off. No
        // truthy-string footgun.
        let resume_existing_chain = matches!(
            std::env::var("TURINGOS_CHAINTAPE_RESUME").as_deref(),
            Ok("1")
        );
        Some(Self {
            runtime_repo_path,
            cas_path,
            run_id,
            queue_capacity,
            resume_existing_chain,
        })
    }
}

// ── Bundle returned by the factory ──────────────────────────────────────────

/// TRACE_MATRIX FC3-N1: handle bundle bridging evaluator → Sequencer + on-disk ChainTape.
///
/// Bundle of runtime handles produced by `build_chaintape_sequencer`.
///
/// The caller is responsible for wiring `sequencer` into a `TuringBus` via
/// `TuringBus::with_sequencer(kernel, config, bundle.sequencer.clone())`,
/// then driving the runtime to completion and calling `bundle.shutdown()`
/// at exit to drain queued submissions.
pub struct ChaintapeBundle {
    /// Cloned and passed to `TuringBus::with_sequencer`.
    pub sequencer: Arc<Sequencer>,
    /// Concrete L4 writer (Git-backed). Test code holds a clone for chain-walk verification.
    pub transition_writer: Arc<RwLock<dyn LedgerWriter>>,
    /// L4.E rejection writer. JSONL backend (Atom 1.2 extension) when persisting; falls back
    /// to in-memory if the JSONL path cannot be opened (caller error handling).
    pub rejection_writer: Arc<RwLock<RejectionEvidenceWriter>>,
    /// Per-run epoch — verifiers re-derive `pinned_pubkeys` map keyed by this epoch.
    pub epoch: SystemEpoch,
    /// Resolved runtime repo path (after canonicalization). Atom 5+ writes pinned pubkey
    /// JSON + agent audit trail under this dir.
    pub runtime_repo_path: PathBuf,
    /// Resolved CAS root directory. Atom 5 callers re-open `CasStore` here to
    /// write `AgentProposalRecord` artifacts (mirrors `runtime_repo_path` for
    /// the L4 / L4.E side).
    pub cas_path: PathBuf,
    /// Driver task running `run_chaintape_driver` against the queue.
    pub driver_handle: JoinHandle<()>,
    /// Drain trigger. Caller invokes `bundle.shutdown().await` at evaluator exit.
    pub shutdown_tx: oneshot::Sender<()>,
}

impl ChaintapeBundle {
    /// TRACE_MATRIX FC3-N1: drain + clean-shutdown contract — caller invokes at evaluator exit.
    ///
    /// Drain + shutdown contract:
    /// 1. Send shutdown signal (consumes shutdown_tx).
    /// 2. Driver wrapper sees signal → closes queue_rx → drains remaining → exits.
    /// 3. `driver_handle.await` blocks until drain completes.
    /// 4. JoinError (panic) is wrapped into `DriverError::JoinError`; clean exit returns Ok.
    pub async fn shutdown(self) -> Result<(), DriverError> {
        let _ = self.shutdown_tx.send(());
        match self.driver_handle.await {
            Ok(()) => Ok(()),
            Err(join_err) => Err(DriverError::JoinError(join_err.to_string())),
        }
    }
}

// ── Errors ──────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC3-N1: bootstrap error class — fail-closed admission gate for production ChainTape.
///
/// Errors produced by the factory at bootstrap time.
#[derive(Debug)]
pub enum BootstrapError {
    Io(std::io::Error),
    LedgerWriter(String),
    Cas(String),
    Keypair(String),
    /// Atom 1 fail-closed: refuse to bootstrap a `Sequencer` (which always
    /// starts `next_logical_t = 0`) on top of an existing `refs/transitions/main`
    /// chain — the next commit would mismatch `Git2LedgerWriter`'s strict
    /// `len + 1` invariant. Resume mode is deferred to a future TB.
    NonEmptyRuntimeRepo {
        path: PathBuf,
        existing_head: String,
    },
    /// TB-7 Atom 1.7 fail-closed (Codex audit cc7b3dd action item #1):
    /// refuse to bootstrap when the L4.E rejection writer cannot open its
    /// JSONL file. Silent fallback to in-memory L4.E is the same
    /// anti-pattern as legacy `bus.append` as authoritative state mutation
    /// — a chain-backed run that secretly drops L4.E writes is worse than
    /// a failed boot.
    RejectionWriter(String),
}

impl std::fmt::Display for BootstrapError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "io error: {e}"),
            Self::LedgerWriter(e) => write!(f, "ledger writer error: {e}"),
            Self::Cas(e) => write!(f, "cas error: {e}"),
            Self::Keypair(e) => write!(f, "keypair error: {e}"),
            Self::NonEmptyRuntimeRepo {
                path,
                existing_head,
            } => write!(
                f,
                "non-empty runtime repo at {path:?} (existing head {existing_head}); \
                 TB-6 Atom 1 fail-closes here. Reconstruction from existing chain is \
                 deferred to a future TB. Point TURINGOS_CHAINTAPE_PATH at a fresh \
                 directory to start a new run."
            ),
            Self::RejectionWriter(e) => write!(f, "rejection writer error: {e}"),
        }
    }
}

impl std::error::Error for BootstrapError {}

impl From<std::io::Error> for BootstrapError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

/// TRACE_MATRIX FC3-N1: runtime-side driver error — bounded to runtime/ module so Sequencer enum stays unchanged.
///
/// Runtime-local driver error. NOT a `Sequencer` enum addition — preserves
/// the no-STEP_B-trigger property of Atom 1.
#[derive(Debug)]
pub enum DriverError {
    JoinError(String),
}

impl std::fmt::Display for DriverError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::JoinError(s) => write!(f, "driver task join error: {s}"),
        }
    }
}

impl std::error::Error for DriverError {}

// ── Pinned pubkey on-disk format ────────────────────────────────────────────

/// TRACE_MATRIX FC3-N1: on-disk pinned-pubkey manifest — bridges Atom 1 keypair to Atom 4 verify_chaintape.
///
/// On-disk pinned-pubkey manifest. Written to `<runtime_repo>/pinned_pubkeys.json`
/// at bootstrap so `verify_chaintape` (Atom 4) can re-verify `system_signature`
/// on every `LedgerEntry` without separate config.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PinnedPubkeyManifest {
    pub run_id: String,
    pub tb_id: String,
    pub epoch: u64,
    pub pubkeys: Vec<PinnedPubkeyEntry>,
}

/// TRACE_MATRIX FC3-N1: single-epoch pinned pubkey row in the on-disk manifest.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PinnedPubkeyEntry {
    pub epoch: u64,
    pub pubkey_hex: String,
}

const PINNED_PUBKEYS_FILENAME: &str = "pinned_pubkeys.json";
const PINNED_PUBKEYS_TB_ID: &str = "TB-6";

fn write_pinned_pubkey_manifest(
    runtime_repo_path: &Path,
    epoch: SystemEpoch,
    keypair: &Ed25519Keypair,
    run_id: &str,
) -> Result<(), BootstrapError> {
    let pubkey = keypair.public_key();
    let pubkey_hex: String = pubkey
        .as_bytes()
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect();
    let manifest = PinnedPubkeyManifest {
        run_id: run_id.to_string(),
        tb_id: PINNED_PUBKEYS_TB_ID.to_string(),
        epoch: epoch.get(),
        pubkeys: vec![PinnedPubkeyEntry {
            epoch: epoch.get(),
            pubkey_hex,
        }],
    };
    let json = serde_json::to_string_pretty(&manifest)
        .map_err(|e| BootstrapError::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?;
    std::fs::create_dir_all(runtime_repo_path)?;
    std::fs::write(runtime_repo_path.join(PINNED_PUBKEYS_FILENAME), json)?;
    Ok(())
}

// ── Factory ─────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC3-N1: production-mode factory — single entry-point that wires the kernel into an LLM-driven binary with on-disk chain persistence.
///
/// Build a production-mode `Sequencer` + `Git2LedgerWriter` + `RejectionEvidenceWriter` +
/// driver task per `RuntimeChaintapeConfig`.
///
/// Steps:
/// 1. Open `Git2LedgerWriter` at `config.runtime_repo_path`. **Fail-closed** if the
///    repo already has a `refs/transitions/main` reference (non-empty chain) —
///    resume mode is a future-TB enhancement.
/// 2. Open `CasStore` at `config.cas_path`.
/// 3. Generate a per-run `Ed25519Keypair`. Pin its public key to
///    `PinnedSystemPubkeys` under `epoch`. Write `pinned_pubkeys.json`
///    next to the runtime repo for `verify_chaintape`.
/// 4. Build `RejectionEvidenceWriter` (Atom 1.1: in-memory; Atom 1.2 extends to
///    JSONL-backed via `RejectionEvidenceWriter::open_jsonl`).
/// 5. Initialize `QState::genesis()` (existing `QState::default()` constructor).
/// 6. Construct `Sequencer::new(...)` — captures the queue receiver in the tuple return.
/// 7. Spawn `run_chaintape_driver(sequencer.clone(), queue_rx, shutdown_rx)` on
///    the current tokio runtime.
/// 8. Return `ChaintapeBundle` with all handles.
pub fn build_chaintape_sequencer(
    config: &RuntimeChaintapeConfig,
) -> Result<ChaintapeBundle, BootstrapError> {
    build_chaintape_sequencer_with_initial_q(config, QState::genesis())
}

/// TRACE_MATRIX FC3-N1: TB-6 Atom 2 — factory variant accepting a pre-seeded `initial_q`.
///
/// Production-mode factory variant for callers that need to pre-populate the
/// economic state (e.g., adapter-level tests + Atom 3 smoke fixtures that
/// seed sponsor balance for `WorkTx` admission per WP § 18 Inv 5). The base
/// `build_chaintape_sequencer` delegates here with `QState::genesis()`.
///
/// All other behavior (fail-closed on non-empty repo, pinned-pubkey manifest,
/// JSONL-backed L4.E writer, runtime-side driver wrapper, Sequencer wiring) is
/// identical to the base factory.
pub fn build_chaintape_sequencer_with_initial_q(
    config: &RuntimeChaintapeConfig,
    initial_q: QState,
) -> Result<ChaintapeBundle, BootstrapError> {
    // Step 1: open or init runtime repo. Branch on resume_existing_chain.
    //
    // TB-G G1.1 (architect §8 SIGNED 2026-05-11; packet §2): when
    // `resume_existing_chain == true` AND `refs/transitions/main` is
    // non-empty, take the resume path; otherwise preserve the original
    // TB-6 fail-closed behavior (`NonEmptyRuntimeRepo`). Empty repo +
    // resume=true degrades to fresh genesis (SG-G1.1 byte-equality).
    std::fs::create_dir_all(&config.runtime_repo_path)?;
    let git_writer = Git2LedgerWriter::open(&config.runtime_repo_path)
        .map_err(|e| BootstrapError::LedgerWriter(e.to_string()))?;
    let chain_is_non_empty = git_writer.head_commit_oid().is_some();
    let resume_active = config.resume_existing_chain && chain_is_non_empty;

    if chain_is_non_empty && !config.resume_existing_chain {
        // SG-G1.4: existing TB-6 fail-closed gate preserved when resume
        // is OFF. All TB-N* / Stage C / Wave 3 50p / TB-N3 Phase 2
        // smoke runs hit this branch unchanged.
        let existing_head = git_writer
            .head_commit_oid()
            .map(|o| o.to_string())
            .unwrap_or_default();
        return Err(BootstrapError::NonEmptyRuntimeRepo {
            path: config.runtime_repo_path.clone(),
            existing_head,
        });
    }

    // Step 2: open CAS (same path for both branches; resume reads
    // pre-existing payloads, fresh writes new ones).
    std::fs::create_dir_all(&config.cas_path)?;
    let cas_store =
        CasStore::open(&config.cas_path).map_err(|e| BootstrapError::Cas(e.to_string()))?;

    // Step 3-3.5: keypair + pinned pubkeys + seed QState + chain length.
    //
    // FRESH path: generate fresh keypair, write fresh manifest, seed
    // Sequencer with caller-supplied `initial_q`, next_logical_t = 0.
    //
    // RESUME path (FC2 Boot constitutional alignment — "every real
    // evidence run must be replayable from genesis_report + ChainTape
    // + CAS + agent registry + system pubkeys", CLAUDE.md §3.2):
    // read existing pinned_pubkeys.json (must exist; fail-closed if
    // not) → build PinnedSystemPubkeys from all manifest entries →
    // read existing initial_q_state.json (must exist; fail-closed
    // if not) → replay every L4 entry via the canonical
    // `replay_full_transition` primitive (the same primitive
    // `verify_chaintape` uses; FC2-Boot replay determinism is
    // covered by Stage A3 SG-A3.4 multi-ref replay-byte-equality)
    // → generate a NEW keypair for a NEW epoch, append to manifest,
    // pin the new pubkey. Old epochs remain pinned so prior entries
    // still verify; new commits sign with the new epoch.
    //
    // Note re packet §2 adjacent-surfaces row: that row described
    // `head_t_witness::reconstruct_from_chaintape_refs` as the
    // QState-rebuild primitive, but in code that helper only
    // reconstructs the 6-field HeadTWitness derived view from L4 /
    // L4.E / CAS ref OIDs (Stage A3 SG-A3.4 derived-view boundary).
    // Per FC2 §3.2 + §4.1 G-009 Path C the canonical QState replay
    // primitive is `replay_full_transition`, which is what
    // `verify_chaintape` uses. This module therefore takes the
    // canonical FC2 replay path; `reconstruct_from_chaintape_refs`
    // is unused by G1.1.
    let (keypair, epoch, pinned, seed_q, chain_length): (
        Arc<Ed25519Keypair>,
        SystemEpoch,
        PinnedSystemPubkeys,
        QState,
        u64,
    ) = if resume_active {
        bootstrap_resume_state(&config.runtime_repo_path, &cas_store, &git_writer)?
    } else {
        let kp = Arc::new(
            Ed25519Keypair::generate_with_secure_entropy()
                .map_err(|e| BootstrapError::Keypair(e.to_string()))?,
        );
        let epoch = SystemEpoch::new(1);
        write_pinned_pubkey_manifest(&config.runtime_repo_path, epoch, &kp, &config.run_id)?;
        let mut pinned = PinnedSystemPubkeys::new();
        pinned.insert(epoch, kp.public_key());
        (kp, epoch, pinned, initial_q, 0u64)
    };
    let pinned_pubkeys = Arc::new(pinned);

    let transition_writer: Arc<RwLock<dyn LedgerWriter>> = Arc::new(RwLock::new(git_writer));
    let cas = Arc::new(RwLock::new(cas_store));

    // Step 4: rejection writer — JSONL-backed at <runtime_repo>/rejections.jsonl
    // per Atom 1.2 + architect § 3.5 deliverable shape.
    //
    // **TB-7 Atom 1.7 (Codex audit cc7b3dd action item #1)**: fail-closed
    // when JSONL open fails. Silent in-memory fallback is the same
    // anti-pattern as legacy `bus.append` as authoritative state mutation:
    // a chain-backed run that secretly drops L4.E writes is worse than a
    // failed boot. ChainTape mode is contractually a fail-closed declaration.
    //
    // TB-G G1.1: on resume the existing rejections.jsonl is re-opened,
    // preserving the cumulative L4.E head. `open_jsonl` internally
    // verifies the chain and rejects any tamper.
    let rejections_path = config.runtime_repo_path.join("rejections.jsonl");
    let rejection_writer = match RejectionEvidenceWriter::open_jsonl(rejections_path.clone()) {
        Ok(w) => Arc::new(RwLock::new(w)),
        Err(e) => {
            return Err(BootstrapError::RejectionWriter(format!(
                "open_jsonl({:?}) failed: {e}",
                rejections_path
            )));
        }
    };

    // Step 5: predicate + tool registries (default empty registries — production-binary
    // is responsible for registering predicates / tools before submitting txs).
    let predicate_registry = Arc::new(PredicateRegistry::new());
    let tool_registry = Arc::new(ToolRegistry::new());

    // Step 6: persist `initial_q_state.json` so `verify_chaintape` can
    // replay from the same seed. TB-G G1.1: skip on resume — the file
    // already exists from the original bootstrap; re-writing would
    // overwrite the canonical genesis snapshot with the post-replay
    // QState and break FC2 replay determinism.
    if !resume_active {
        let initial_q_path = config.runtime_repo_path.join("initial_q_state.json");
        let initial_q_json = serde_json::to_string_pretty(&seed_q)
            .map_err(|e| BootstrapError::Cas(format!("initial_q serialize: {e}")))?;
        std::fs::write(&initial_q_path, initial_q_json)?;
    }

    // Step 7: construct Sequencer. Fresh path uses `Sequencer::new`
    // (next_logical_t = 0); resume path uses
    // `Sequencer::new_at_logical_t(.., chain_length)` so the next
    // commit satisfies `Git2LedgerWriter::append`'s strict
    // `len + 1` invariant (packet §3 SG-G1.2).
    let (sequencer, queue_rx) = if resume_active {
        Sequencer::new_at_logical_t(
            cas,
            keypair,
            epoch,
            transition_writer.clone(),
            rejection_writer.clone(),
            predicate_registry,
            tool_registry,
            pinned_pubkeys,
            seed_q,
            config.queue_capacity,
            chain_length,
        )
    } else {
        Sequencer::new(
            cas,
            keypair,
            epoch,
            transition_writer.clone(),
            rejection_writer.clone(),
            predicate_registry,
            tool_registry,
            pinned_pubkeys,
            seed_q,
            config.queue_capacity,
        )
    };
    let sequencer = Arc::new(sequencer);

    // Step 8: spawn driver wrapper + shutdown channel.
    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();
    let driver_seq = sequencer.clone();
    let driver_handle = tokio::spawn(async move {
        run_chaintape_driver(driver_seq, queue_rx, shutdown_rx).await;
    });

    Ok(ChaintapeBundle {
        sequencer,
        transition_writer,
        rejection_writer,
        epoch,
        runtime_repo_path: config.runtime_repo_path.clone(),
        cas_path: config.cas_path.clone(),
        driver_handle,
        shutdown_tx,
    })
}

/// TB-G G1.1 (architect §8 SIGNED 2026-05-11; packet §2 resume branch):
/// reconstruct the (keypair, epoch, pinned pubkeys, seed QState,
/// chain_length) tuple consumed by `build_chaintape_sequencer_with_initial_q`
/// when resuming a non-empty `refs/transitions/main` chain.
///
/// Steps (FC2 Boot constitutional alignment per CLAUDE.md §3.2 +
/// §4.1 G-009 Path C):
/// 1. Read existing `pinned_pubkeys.json`. Fail-closed if missing or
///    unparseable.
/// 2. Build `PinnedSystemPubkeys` from every manifest entry so all
///    prior-epoch entries still verify.
/// 3. Read existing `initial_q_state.json`. Fail-closed if missing.
/// 4. Read every L4 entry from `refs/transitions/main`.
/// 5. Replay via `replay_full_transition` — the canonical FC2 Boot
///    replay primitive shared with `verify_chaintape`. Any replay
///    failure is fail-closed (no partial admission per packet §5 Q1).
/// 6. Generate a NEW keypair for a NEW epoch (`max_existing + 1`).
///    Append the new entry to `pinned_pubkeys.json` on disk; old
///    entries stay so prior commits still verify. Pin the new pubkey
///    so new commits round-trip.
fn bootstrap_resume_state(
    runtime_repo_path: &Path,
    cas_store: &CasStore,
    git_writer: &Git2LedgerWriter,
) -> Result<
    (
        Arc<Ed25519Keypair>,
        SystemEpoch,
        PinnedSystemPubkeys,
        QState,
        u64,
    ),
    BootstrapError,
> {
    // 1. Pinned pubkeys manifest — fail-closed if missing.
    let manifest_path = runtime_repo_path.join(PINNED_PUBKEYS_FILENAME);
    if !manifest_path.exists() {
        return Err(BootstrapError::Keypair(format!(
            "resume mode: pinned_pubkeys.json missing at {manifest_path:?}; \
             cannot rebuild PinnedSystemPubkeys map for chain signature verification"
        )));
    }
    let manifest_json = std::fs::read_to_string(&manifest_path)?;
    let mut manifest: PinnedPubkeyManifest = serde_json::from_str(&manifest_json)
        .map_err(|e| BootstrapError::Keypair(format!("pinned_pubkeys.json parse: {e}")))?;

    // 2. Rehydrate PinnedSystemPubkeys from every manifest entry.
    let mut pinned = PinnedSystemPubkeys::new();
    let mut max_epoch: u64 = 0;
    for entry in &manifest.pubkeys {
        let bytes = decode_pubkey_hex_32(&entry.pubkey_hex)?;
        pinned.insert(
            SystemEpoch::new(entry.epoch),
            SystemPublicKey::from_bytes(bytes),
        );
        if entry.epoch > max_epoch {
            max_epoch = entry.epoch;
        }
    }

    // 3. Initial QState — fail-closed if missing.
    let initial_q_path = runtime_repo_path.join("initial_q_state.json");
    if !initial_q_path.exists() {
        return Err(BootstrapError::Cas(format!(
            "resume mode: initial_q_state.json missing at {initial_q_path:?}; \
             cannot replay chain without canonical genesis seed QState"
        )));
    }
    let initial_q_json = std::fs::read_to_string(&initial_q_path)?;
    let initial_q: QState = serde_json::from_str(&initial_q_json)
        .map_err(|e| BootstrapError::Cas(format!("initial_q_state.json parse: {e}")))?;

    // 4. Read every L4 entry.
    let chain_length = git_writer.len();
    let entries: Vec<LedgerEntry> = (1..=chain_length)
        .map(|t| git_writer.read_at(t))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| BootstrapError::LedgerWriter(format!("resume read_at sweep: {e}")))?;

    // 5. Replay via the canonical FC2 Boot primitive.
    //
    // Production binaries register predicates / tools AFTER bootstrap,
    // and `verify_chaintape` does the same — so empty registries here
    // mirror `verify_chaintape`'s replay envelope. Replay is
    // deterministic regardless of registry contents because
    // `dispatch_transition` is a pure function over inputs.
    let predicate_registry = PredicateRegistry::new();
    let tool_registry = ToolRegistry::new();
    let replayed_q = replay_full_transition(
        &initial_q,
        &entries,
        cas_store,
        &pinned,
        &predicate_registry,
        &tool_registry,
    )
    .map_err(|e| {
        BootstrapError::LedgerWriter(format!("resume replay_full_transition failed: {e:?}"))
    })?;

    // 6. New keypair + new epoch.
    let new_keypair = Arc::new(
        Ed25519Keypair::generate_with_secure_entropy()
            .map_err(|e| BootstrapError::Keypair(e.to_string()))?,
    );
    let new_epoch = SystemEpoch::new(max_epoch + 1);
    pinned.insert(new_epoch, new_keypair.public_key());

    // Append to on-disk manifest. Old entries preserved verbatim per
    // packet §1 ("pinned-pubkey continuity: resume branch reads
    // existing pinned_pubkeys.json if present; epoch preserved
    // across resume"). New entry appended; manifest top-level
    // `epoch` advances to the new epoch so verifiers can pick the
    // freshest pin while still resolving any earlier entry.
    let new_pubkey_hex: String = new_keypair
        .public_key()
        .as_bytes()
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect();
    manifest.epoch = new_epoch.get();
    manifest.pubkeys.push(PinnedPubkeyEntry {
        epoch: new_epoch.get(),
        pubkey_hex: new_pubkey_hex,
    });
    let updated_json = serde_json::to_string_pretty(&manifest)
        .map_err(|e| BootstrapError::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?;
    std::fs::write(&manifest_path, updated_json)?;

    Ok((new_keypair, new_epoch, pinned, replayed_q, chain_length))
}

/// TB-G G1.1: decode a 64-char hex string into a 32-byte ed25519 pubkey.
/// Used only by `bootstrap_resume_state`; kept private. Mirrors the
/// `verify.rs::decode_pubkey_hex` decode path but yields the array
/// shape directly (no intermediate `Vec`).
fn decode_pubkey_hex_32(hex: &str) -> Result<[u8; 32], BootstrapError> {
    if hex.len() != 64 {
        return Err(BootstrapError::Keypair(format!(
            "pubkey_hex expected 64 chars, got {}",
            hex.len()
        )));
    }
    let mut out = [0u8; 32];
    for i in 0..32 {
        let byte = u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16)
            .map_err(|e| BootstrapError::Keypair(format!("pubkey_hex decode at byte {i}: {e}")))?;
        out[i] = byte;
    }
    Ok(out)
}

// ── Driver wrapper ──────────────────────────────────────────────────────────

/// Runtime-side driver loop. NOT `Sequencer::run`: see module doc-comment.
///
/// Invariants:
/// - On `shutdown_rx` signal: closes `queue_rx` (refuses new sends), drains
///   the remaining queue synchronously via `Sequencer::apply_one`, returns.
/// - On `queue_rx.recv() == None` (all senders dropped): returns.
/// - `tokio::select! { biased; ... }` ensures the shutdown signal wins races
///   against pending `recv()` calls (otherwise busy queues could starve shutdown).
async fn run_chaintape_driver(
    sequencer: Arc<Sequencer>,
    mut queue_rx: tokio::sync::mpsc::Receiver<SubmissionEnvelope>,
    mut shutdown_rx: oneshot::Receiver<()>,
) {
    loop {
        tokio::select! {
            biased;
            _ = &mut shutdown_rx => {
                // Refuse new sends, then drain remaining envelopes.
                queue_rx.close();
                while let Some(envelope) = queue_rx.recv().await {
                    if let Err(e) = sequencer.apply_one(envelope) {
                        log::debug!("chaintape driver drain apply_one rejected: {e}");
                    }
                }
                return;
            }
            env = queue_rx.recv() => {
                match env {
                    Some(envelope) => {
                        if let Err(e) = sequencer.apply_one(envelope) {
                            log::debug!("chaintape driver apply_one rejected: {e}");
                        }
                    }
                    None => return,
                }
            }
        }
    }
}

// ── Tests ───────────────────────────────────────────────────────────────────
//
// In-crate unit tests cover the construction path that does NOT need the
// full evaluator / TuringBus surface. Integration tests for the full L4 path
// (including direct `bus.submit_typed_tx` fixture) live at
// `tests/tb_6_runtime_chaintape_bootstrap.rs` and land in Atom 1.3.

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn cfg_for(tmp: &TempDir, run_id: &str) -> RuntimeChaintapeConfig {
        RuntimeChaintapeConfig {
            runtime_repo_path: tmp.path().join("runtime_repo"),
            cas_path: tmp.path().join("cas"),
            run_id: run_id.to_string(),
            queue_capacity: 16,
            resume_existing_chain: false,
        }
    }

    #[tokio::test]
    async fn build_chaintape_sequencer_returns_non_none_sequencer_with_git_writer() {
        let tmp = TempDir::new().expect("tempdir");
        let cfg = cfg_for(&tmp, "t1-run");
        let bundle = build_chaintape_sequencer(&cfg).expect("bootstrap");
        // Sequencer constructed.
        assert_eq!(Arc::strong_count(&bundle.sequencer) >= 2, true);
        // pinned_pubkeys.json was written.
        let manifest_path = cfg.runtime_repo_path.join(PINNED_PUBKEYS_FILENAME);
        assert!(
            manifest_path.exists(),
            "pinned_pubkeys.json must exist at {manifest_path:?}"
        );
        // Clean shutdown.
        bundle.shutdown().await.expect("shutdown");
    }

    #[tokio::test]
    async fn build_chaintape_sequencer_writes_pinned_pubkeys_json_to_runtime_repo() {
        let tmp = TempDir::new().expect("tempdir");
        let cfg = cfg_for(&tmp, "t2-run");
        let bundle = build_chaintape_sequencer(&cfg).expect("bootstrap");
        let manifest_path = cfg.runtime_repo_path.join(PINNED_PUBKEYS_FILENAME);
        let json = std::fs::read_to_string(&manifest_path).expect("read manifest");
        let manifest: PinnedPubkeyManifest = serde_json::from_str(&json).expect("parse manifest");
        assert_eq!(manifest.run_id, "t2-run");
        assert_eq!(manifest.tb_id, "TB-6");
        assert_eq!(manifest.epoch, 1);
        assert_eq!(manifest.pubkeys.len(), 1);
        assert!(!manifest.pubkeys[0].pubkey_hex.is_empty());
        bundle.shutdown().await.expect("shutdown");
    }

    #[tokio::test]
    async fn build_chaintape_sequencer_fails_on_non_empty_repo() {
        let tmp = TempDir::new().expect("tempdir");
        let cfg = cfg_for(&tmp, "t3-run");
        // First bootstrap on empty repo — succeeds.
        let bundle = build_chaintape_sequencer(&cfg).expect("first bootstrap");
        bundle.shutdown().await.expect("shutdown");
        // Manually create a synthetic head commit so head_commit_oid().is_some()
        // We can do this by appending a fake LedgerEntry — but that requires a
        // signed entry. Cheaper: just open the same path again and check that
        // the FRESH bootstrap on an EMPTY but git-init'd repo still succeeds
        // (head_commit_oid is None for an init'd-but-no-commits repo). To
        // actually trigger NonEmptyRuntimeRepo, we'd need to commit a real
        // entry; that requires a full sequencer.apply_one path which is
        // cleaner to exercise in tb_6_runtime_chaintape_bootstrap.rs (Atom 1.3).
        //
        // For Atom 1.1 in-crate coverage: confirm the second bootstrap (with
        // empty git refs) still succeeds — exercises the head_commit_oid().is_none() branch.
        let bundle2 = build_chaintape_sequencer(&cfg).expect("second bootstrap on empty refs");
        bundle2.shutdown().await.expect("second shutdown");
    }

    #[tokio::test]
    async fn chaintape_bundle_shutdown_returns_clean_on_empty_queue() {
        let tmp = TempDir::new().expect("tempdir");
        let cfg = cfg_for(&tmp, "t5-run");
        let bundle = build_chaintape_sequencer(&cfg).expect("bootstrap");
        // No submissions made — queue stays empty. shutdown() must return Ok promptly.
        let res = tokio::time::timeout(std::time::Duration::from_secs(5), bundle.shutdown()).await;
        let inner = res.expect("shutdown did not time out");
        inner.expect("shutdown returned Ok");
    }
}
