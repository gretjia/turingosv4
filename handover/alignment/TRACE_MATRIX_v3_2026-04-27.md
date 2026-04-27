# TRACE_MATRIX_v3 — Bidirectional Mapping with N/M/D Classification

> **Date**: 2026-04-27
> **Purpose**: D-VETO-5 final form; Codex CO P0.7 §2 demanded full coverage beyond seed; Gemini v3.2 Q1 PASS pending complete trace.
> **Authority**: Constitution + WP architecture (21 §) + WP economic (8 §, numbered 0/2/7/15/18/19/20/21) + RSP appendix.
> **Classification**:
> - **[N]ormative** = MUST map to ≥1 code symbol AND ≥1 conformance test
> - **[M]otivational** = explanatory text; no code mapping required
> - **[D]eferred** = out-of-v4 scope; lists target version + reason
>
> **Scope rule**: every WP § + every Constitution Article = one row in this matrix. If a § contains multiple normative claims, sub-rows allowed.

---

## § A — Constitution → Code Symbol Map

| Article | Class | Code symbol | Conformance test | Plan v3.2 atom |
|---|---|---|---|---|
| Art 0 — 图灵机原教旨 | N | `bottom_white::tape::chain_tape::ChainTape` + `tools::wtool::*` + `wal::Wal` + `top_white::predicates::registry::PredicateRegistry` (4-element mapping) | `tests/turing_fundamentalism.rs` | CO1.0 / CO1.5 / CO1.6 / CO1.7 |
| Art 0.1 — 四要素映射 | N | (same as Art 0) + `state::q_state::QState` for tape/control mapping | `tests/four_element_mapping.rs` | CO1.2 |
| Art 0.2 — Tape Canonical 公理 | N | `bottom_white::tape::tape_canonical_check::*` + 24 V-violation tests | `tests/tape_canonical_V01..V24.rs` (24 tests) | CO1.5-1.9 |
| Art 0.2 item 5 — failure-on-tape interpretation | N | `bottom_white::ledger::retry_metadata::{RejectedAttemptSummary, TerminalSummaryTx}` (Reading Y per Art 0.2 reinterpretation) | `tests/l6_reconstructibility.rs` + `tests/failure_histogram_reconstruct.rs` | CO1.7.0 + CO1.9.5 |
| Art 0.3 — 区块链化保留 | D (Path A semantic) / N (Path B git substrate) | `bottom_white::tape::git_substrate::*` (Path B chosen) | `tests/git_substrate_runtime_repo.rs` | CO1.3 |
| Art 0.4 — Q_t version-controlled | N | `state::q_state::QState` + `bottom_white::tape::git_substrate::on_cell_start` | `tests/q_state_reconstruct.rs` | CO1.2 + CO1.3 |
| Laws (基本法 1: Coin 守恒) | N | `economy::escrow_vault::*` + `economy::settlement_engine::*` | `tests/economic_invariant_INV3_escrow_only.rs` | CO P2.2 + CO P2.6 |
| Laws (基本法 2: founder grant) | N | `economy::escrow_vault::founder_grant_at_task_create` | `tests/economic_audit_E04_founder_grant_law2.rs` | CO P2.10 |
| Art I — 信号的量化 (top-level) | N | `top_white::signals::{boolean,statistical}` | `tests/signal_dichotomy.rs` | CO1.10 |
| Art I.1 — 布尔信号 | N | `top_white::signals::boolean::*` + `top_white::predicates::runner::run_acceptance` | `tests/boolean_signal_pass_fail.rs` | CO1.10 + CO1.5 |
| Art I.1.1 — PCP 谓词疑罪从无 | N | `top_white::predicates::registry::SafetyOrCreation` enum | `tests/safety_creation_dichotomy.rs` | CO1.11 |
| Art I.2 — 统计信号 | N | `top_white::signals::statistical::*` + `bottom_white::signal_index::stat_index` + `economy::reputation_index::*` + PPUT report | `tests/statistical_signals_complete.rs` | CO1.10 + CO1.9 |
| Art I.2 — PPUT/H-VPPUT/CI 报告强制项 | N | `experiments/.../bin/evaluator.rs::emit_summary` | `tests/report_standard_pput_ci_required.rs` | (existing; preserve through CO1.1.5 split) |
| Art II — 选择性广播 (top-level) | N | `top_white::signals::price_broadcast::emit` + L6 indices | `tests/broadcast_emits_to_l6.rs` | CO1.9 |
| Art II.1 — 广播典型错误 | N | `bottom_white::signal_index::failure_histogram` (system-derived, NOT agent self-report) | `tests/failure_histogram_reconstruct.rs` | CO1.9.5 |
| Art II.2 — 广播价格信号 | N | `top_white::signals::price_broadcast::emit_price` | `tests/price_broadcast_l6.rs` | CO1.9 |
| Art II.2.1 — 探索/利用 + parent_selection_entropy + payload_diversity | N | `experiments/.../bin/evaluator.rs::compute_entropy_and_diversity` | `tests/entropy_diversity_thresholds.rs` (per CLAUDE.md alert at < 0.25) | (existing; preserve) |
| Art III — 选择性屏蔽 (top-level) | N | `top_white::predicates::visibility::*` + `bottom_white::materializer::agent_view` | `tests/visibility_filter.rs` | CO1.5 + CO1.8 |
| Art III.1 — 屏蔽错误 | N | `top_white::predicates::visibility::Visibility::Private` for error contents | `tests/private_predicate_error_no_leak.rs` | CO1.5.7 |
| Art III.2 — 封装细节 | N | `bottom_white::materializer::agent_view::project_for_agent` | `tests/agent_view_filters_internals.rs` | CO1.8.6 |
| Art III.3 — 屏蔽相关性 | N | `economy::price_index::aggregation_filter` (top-K only; no fine-grain) | `tests/price_aggregation_correlation_shield.rs` | CO P2.1 (TaskMarket price publish) |
| Art III.4 — 屏蔽 Goodhart | N | `top_white::predicates::visibility::Visibility::{Public,Private,CommitReveal}` | `tests/goodhart_shield.rs` + `tests/economic_invariant_INV10_signal_vs_evaluator.rs` | CO1.5.2 + CO1.5.7 |
| Art IV — Boot (Bootstrap 公理) | N | `boot::verify_trust_root` + `boot::verify_constitution_root` (NEW per genesis spec) + `state::q_state::QState::genesis` | `tests/boot_genesis_minimal_with_anchor.rs` + 5 new genesis tests | CO1.0 |
| Art IV — terminal categorization (halt_reason 5种) | N | `experiments/.../bin/evaluator.rs::HaltReason` enum + summary | `tests/halt_reason_distribution.rs` | (existing; preserve) |
| Art V — Go Meta (top-level) | N (offline path) / D-v4.1 (runtime path) | `governance::meta_validator::validate_meta_proposal` (offline) / runtime ArchitectAI deferred to v4.1 | `tests/meta_validator_correctness.rs` (CO P3-prep) + v4.1 runtime tests | CO P3-PREP 1-7 |
| Art V.1.1 — Constitution 唯一基准 | N | `genesis_payload::constitution_root::constitution_hash` + `boot::verify_constitution_root` | `tests/genesis_constitution_root_verify.rs` | CO1.0.4 |
| Art V.1.2 — ArchitectAI 提出者 | N (offline v4) | `governance::amendment_predicate::evaluate` + cp workflow | `tests/architect_proposal_offline.rs` | CO P3-prep.4 |
| Art V.1.3 — Veto-AI 验证者 | N | dual external audit (Codex + Gemini) per `TRI_MODEL_ORCHESTRATION_PROTOCOL` | `tests/dual_audit_protocol_existence.rs` (meta-test) | (existing) |
| Art V.2 — 宪法界限与示例 | M | (no code mapping; explanatory) | n/a | n/a |
| Art V.3 — 宪法修订日志 | N | `handover/architect-insights/RATIFICATION_*.md` chain + signed git tags | `tests/ratification_chain_verifies.rs` | (governance gate; existing per B-1) |

---

## § B — WP Architecture → Code Symbol Map

| § | Title | Class | Code symbol | Conformance test | Plan atom |
|---|---|---|---|---|---|
| Abstract | (TuringOS = …) | M | n/a | n/a | n/a |
| § 0 设计公理 | 6 axioms | N (bridge to Const Art 0.5 + 6 公理) | `state::q_state::QState` (axiom 1) + `top_white::predicates::*` (axiom 2) + `economy::*` (axiom 3) etc. | `tests/six_axioms_alignment.rs` | CO0.8 + CO1.* |
| § 1 问题 | why agents crash | M | n/a | n/a | n/a |
| § 2 图灵机隐喻 | paper/pencil/rubber | N (mirrors Const Art 0) | (same as Const Art 0) | (same) | CO1.0/1.6/1.7 |
| § 3 反奥利奥三层 | top/middle/bottom white | N | `src/{top_white,middle_black,bottom_white,economy}/*` directory structure | `tests/anti_oreo_layer_audit.rs` | CO1.1.* |
| § 4 系统状态 Q_t | 8 components | N | `state::q_state::QState` (9 fields incl economic_state_t) | `tests/q_state_reconstruct.rs` + `tests/economic_state_reconstruct.rs` | CO1.2 |
| § 5.L0 Constitution Root | hash + sig + sudo + amendment_rules + attestation | N | `genesis_payload::constitution_root::*` (8 fields per `GENESIS_MINIMAL_WITH_ANCHOR_v1`) | `tests/genesis_constitution_root_*.rs` (5) | CO1.0.* |
| § 5.L1 Predicate Registry | id + version + code_hash + schema + visibility + owner + test_suite | N | `top_white::predicates::registry::PredicateRegistry` | `tests/chain_tape_L1_predicate_registry.rs` | CO1.5 |
| § 5.L2 Tool Registry | id + capability + permission + determinism + side_effect | N | `bottom_white::tools::registry::ToolRegistry` | `tests/chain_tape_L2_tool_registry.rs` | CO1.6 |
| § 5.L3 CAS | cid + hash + type + creator + visibility | N | `bottom_white::cas::store::*` | `tests/chain_tape_L3_cas.rs` | CO1.4 |
| § 5.L4 Transition Ledger | 12 fields | N | `bottom_white::ledger::transition::TransitionTx` (12 fields incl task_id) | `tests/chain_tape_L4_transition_ledger.rs` + `tests/transition_tx_12_fields.rs` | CO1.7 |
| § 5.L5 Materialized State + Agent View | indices + permission_view | N | `bottom_white::materializer::{state_db, indices, agent_view}` | `tests/chain_tape_L5_materialized_state.rs` | CO1.8 |
| § 5.L6 Signal Indices | boolean + price + reputation + scarcity + explore/exploit | N | `bottom_white::signal_index::*` + `top_white::signals::*` | `tests/chain_tape_L6_signal_indices.rs` | CO1.9 |
| § 6 状态转移协议 | step_transition 7 stages | N | `transition::step_transition` + verify/challenge/reuse/finalize per `STATE_TRANSITION_SPEC_v1` | 20 invariants → 20 tests `tests/transition_*.rs` | CO1.SPEC.0 + CO1.7.5 |
| § 7 信号的量化 | boolean vs statistical dichotomy | N | (same as Const Art I) | `tests/signal_dichotomy.rs` | CO1.10 |
| § 7.2 安全 vs 创造 fail-policy | safety fail-closed; creation fail-open-with-signal | N | `top_white::predicates::registry::SafetyOrCreation` | `tests/safety_creation_dichotomy.rs` | CO1.11 |
| § 8 选择性广播 | broadcast price + boolean signal aggregates | N | `top_white::signals::price_broadcast::*` | `tests/price_broadcast_l6.rs` | CO1.9 + CO1.10 |
| § 9.1 屏蔽错误 (per Codex demand) | error hiding | N | `top_white::predicates::visibility::Visibility::Private` error filter | `tests/private_predicate_error_no_leak.rs` | CO1.5.7 |
| § 9.2 最小上下文 | minimal agent context window | N | `bottom_white::materializer::agent_view::project_for_agent` (visibility-filtered) | `tests/agent_view_minimal_context.rs` | CO1.8.6 + CO1.8.7 |
| § 9.3 屏蔽相关性 | correlation shielding | N | `economy::price_index::aggregation_filter` | `tests/price_aggregation_correlation_shield.rs` | CO P2.1 |
| § 9.4 Goodhart 屏蔽 (public/private/commit-reveal) | three visibility classes | N | `top_white::predicates::visibility::Visibility` enum | `tests/goodhart_shield.rs` + `tests/economic_invariant_INV10_signal_vs_evaluator.rs` | CO1.5.2 |
| § 10 Laws of Money | monetary discipline → economic chapter elaborates | N | (links to economic chapter Inv 1-12) | (12 INV tests) | CO P2.* |
| § 11 Boot — 创世状态 | genesis block fields | N | `genesis_payload::*` (8 fields per GENESIS_MINIMAL_WITH_ANCHOR_v1) | `tests/genesis_*.rs` (5) | CO1.0 |
| § 12 Go Meta | meta_tx semantics | N (offline) / D-v4.1 (runtime) | `META_TX_SCHEMA_v1` typed schema + `governance::meta_validator::*` (offline); runtime ArchitectAI/JudgeAI deferred to v4.1 | `tests/meta_tx_schema_serialization.rs` + `tests/meta_validator_*.rs` | CO P3-PREP.1, .3, .5, .6 |
| § 12.2 meta_tx schema | parent_root + patches + evidence + reversibility + check + sigs + human_sig | N (schema) / D-v4.1 (L4 acceptance) | `META_TX_SCHEMA_v1` § 2 typed schema | `tests/meta_tx_schema_serialization.rs` | CO P3-prep.1 |
| § 13 区块链位置 | local→permissioned→rollup→public | partial: N (local hashchain/git → v4); D-v4.1+ (Hyperledger / rollup / public) | `bottom_white::tape::git_substrate` (local Path B); permissioned/rollup deferred | `tests/git_substrate_*.rs` | CO1.3 |
| § 14 数据结构示例 | illustrative TOML/Rust snippets | M | n/a | n/a | n/a |
| § 15 MVP | minimum viable phase | N | (links to § 17 Phase 1+2; v4 scope) | (per phase exit gates) | CO P0/P1/P2 exits |
| § 16 安全边界与失败模式 | threat model, failure classes | N | `SYSTEM_KEYPAIR_SECURITY_v1` § 2 threat model + `top_white::predicates::*` failure classification | `tests/system_keypair_*.rs` (5) + per-failure-class tests | CO1.7.0a + CO1.5 |
| § 17 实施路线 5-Phase | Phase 1+2 (v4) + Phase 3 prep + Phase 4-5 deferred | N (v4 Phase 1+2 + Phase 3 prep) / D-v4.1+ (Phase 4-5) | Plan v3.2 atoms CO P0+P1+P2 + CO P3-PREP track | `tests/phase_1_2_complete.rs` (synthetic) | CO P0-P2 + CO P3-PREP |
| § 18 结论 | summary | M | n/a | n/a | n/a |
| RSP § 1-16 (appendix) | RSP details, mostly redundant with economic chapter | N (redundant; map via economic chapter rows) | (see § C below) | (see § C) | CO P2 |

---

## § C — WP Economic → Code Symbol Map

| § | Title | Class | Code symbol | Conformance test | Plan atom |
|---|---|---|---|---|---|
| § 0 核心校准 | "经济不是发币" negative invariant | N | `economy::*` (no `mint_post_init` API surface; Inv 4 + cargo-deny) + negative test | `tests/economic_audit_E03_naming.rs` (no token-issuance APIs) + `tests/no_post_init_mint.rs` | CO P2.0 + CO P2.10 |
| § 2 Q_t 扩展 | economic_state_t 9 sub-fields | N | `state::q_state::EconomicState` 9 sub-fields | `tests/economic_state_reconstruct.rs` | CO1.2.2 |
| § 7 Agent 5 经济角色 | Solver/Verifier/Challenger/Builder/ArchitectAI/JudgeAI (6 roles, "5 + Judge meta" interpretation) | N | `experiments/.../agents/{solver,verifier,challenger,builder,architect_ai,judge_ai}.rs` (6 files) | `tests/agent_role_economic.rs` (6 roles dispatch) | CO P2.7 |
| § 15 区块链技术定位 | local/permissioned/rollup/ZK/oracle | partial: N (local) / D-v4.1+ (rest) | (see arch § 13 row) | (same) | CO1.3 |
| § 18 12 Economic Invariants | Inv 1-12 | N (each invariant is its own conformance test) | `economy::invariants::inv01..inv12` | `tests/economic_invariant_INV1..12.rs` (12 tests) | CO P2.* |
| § 19 RSP-1 modules (9) | TaskMarket / EscrowVault / ContributionLedger / PredicateRunner / AttributionEngine / ChallengeCourt / SettlementEngine / ReputationIndex / PriceIndex | N | `economy::{task_market, escrow_vault, contribution_ledger, attribution_engine, challenge_court, settlement_engine, reputation_index, price_index}::*` (8 dirs; PredicateRunner lives in `top_white::predicates::runner`) | `tests/rsp1_modules_smoke.rs` + per-module tests | CO P2.1-2.9 |
| § 20 5-Phase 部署 | Phase 1 (Local Ledger) / Phase 2 (Internal Task Market) / Phase 3-5 deferred | N (v4 Phase 1+2) / D-v4.x (Phase 3-5) | (Plan v3.2 atoms) | (per phase gates) | CO P0-P2 |
| § 21 最终公式 | reward_i = Finalize(Escrow × Accept × Attribution × Survival × Utility × Constitution) | N | `economy::settlement_engine::finalize_reward` (per `STATE_TRANSITION_SPEC_v1` § 3.4) | `tests/final_reward_formula.rs` | CO P2.6.4 |
| (cross-ref to architecture) | mapping table | M | n/a | n/a | n/a |

---

## § D — RSP Appendix (architecture WP § 1050-1066) → Code Symbol Map

The RSP appendix in architecture WP largely overlaps the economic chapter. Cross-references:

| Appendix § | Architecture WP line | Economic chapter equivalent | Class |
|---|---|---|---|
| RSP § 1-3 (intro) | line 1050-1066 | § 0-19 | M (intro) |
| RSP § 4-8 (mechanisms) | line 1067+ (in WP) | § 21 final formula | N (mapped via econ § 21) |
| RSP § 9-12 (economic state, escrow, settlement) | line 1100+ (in WP) | § 19 RSP-1 modules | N (mapped via econ § 19) |
| RSP § 13-16 (governance, monetary base, signals) | line 1180+ | § 18 invariants + § 21 formula | N (mapped via econ § 18/21) |

**Note**: per Codex CO P0.7 §2 row (RSP appendix), economic chapter § 19 lists 9 modules but architecture appendix lists 8. Discrepancy resolved: PriceIndex is the 9th module in economic chapter; architecture appendix groups PriceIndex under Signal Indices L6 (still mapped, just split across two layers in architecture WP). Both are normative; both implemented.

---

## § E — Coverage Statistics

| Source | Total rows | [N] | [M] | [D] |
|---|---|---|---|---|
| Constitution Articles + sub-articles | 27 | 24 | 1 (Art V.2) | 2 (Art 0.3 partial Path A; Art 0.5 future) |
| WP architecture §§ | 21 (incl 0/1/2 plus subsections 5.L0-L6, 7.2, 9.1-4, 12.2, 17 phases) | 17 (full) + 4 (partial / phase-conditional) | 4 (Abstract, § 1, § 14, § 18) | embedded in partial rows |
| WP economic §§ | 8 (numbered 0/2/7/15/18/19/20/21) | 7 | 1 (cross-ref table) | embedded in partial rows |
| RSP appendix | 4 sub-§ | 3 | 1 | — |

**Total Normative coverage**: ~51 rows. Each Normative row has at least 1 conformance test path (existing or planned in Plan v3.2 atoms).

**Test count from this matrix**: ~60-70 distinct conformance tests (some rows share tests; e.g., Goodhart shield).

**Forbidden state**: any Normative row with empty "code symbol" or empty "conformance test" column. Pre-commit hook R-022 (added per Plan v3.2 CO P0.8) enforces.

---

## § F — Bidirectional Reverse: Code Symbol → Source

This section is populated incrementally as code lands (currently empty for v4 since CO P1 has not started). Format:

```
src/path/to/symbol.rs::function_name
  ↓
  TRACE_MATRIX_v3 row: <Constitution Art X | WP arch § Y | WP econ § Z>
```

This reverse map is auto-generated by `scripts/check_trace_matrix_updated.sh` per Plan v3.2 CO1.13.2 atom. Pre-commit hook R-022 enforces "every `pub` symbol in src/{top_white,middle_black,bottom_white,economy,state,transition,governance}/*.rs MUST have a `/// TRACE_MATRIX <id>: <role>` doc-comment". Build fails if missing.

**Initial state at v4 ratification (2026-04-27)**: section is **empty by design** — code does not yet exist. v4 will populate it commit-by-commit during CO P1+P2.

---

## § G — Deferred Items Justification

Items classified [D]eferred MUST list target version + reason. Audit gate: every [D] tag is reviewable; no opaque "later".

| Item | Target version | Reason |
|---|---|---|
| Constitution Art 0.3 Path A semantic version | NEVER (Path B chosen instead) | Art 0.4 commit selected Path B (real git substrate); Path A description in Art 0.3 marked obsolete by Art 0.4 caveat (line 110) |
| Constitution Art 0.5 (white paper integration) | CO P0 enactment (post-ratification cp ceremony) | DRAFT exists; awaits user cp + signed tag |
| WP architecture § 13 permissioned/rollup phases | v4.x or v5 | per WP § 17 explicit roadmap |
| WP architecture § 17 Phase 4-5 (public chaincode/rollup) | v5 | scope decision; WP says "post-v4" |
| WP architecture § 12 runtime ArchitectAI/JudgeAI | v4.1 | D-VETO-4 ratified resolution; v4 ships Phase 3 prep (CO P3-PREP.1-7) |
| WP economic § 15 ZK/Validity Proof predicates | v4.x or v5 | requires substantive cryptographic infrastructure beyond v4 Path B |
| WP economic § 15 Oracle integration | v4.x | external fact input substrate; v4 is closed-system |

---

## § H — Conformance Test Master List (output for cargo test wiring)

Tests required to claim 100% Normative coverage (organized by domain):

```
# Anti-Oreo + Q_t + Tape Canonical (CO1.1, CO1.2, CO1.5-1.9)
tests/anti_oreo_layer_audit.rs
tests/q_state_reconstruct.rs
tests/economic_state_reconstruct.rs
tests/four_element_mapping.rs
tests/turing_fundamentalism.rs
tests/tape_canonical_V01..V24.rs                    (24 tests)

# ChainTape layers (CO1.0-1.9)
tests/chain_tape_L0_constitution_root.rs
tests/chain_tape_L1_predicate_registry.rs
tests/chain_tape_L2_tool_registry.rs
tests/chain_tape_L3_cas.rs
tests/chain_tape_L4_transition_ledger.rs
tests/chain_tape_L5_materialized_state.rs
tests/chain_tape_L6_signal_indices.rs

# State transition spec invariants I-1 through I-20 (CO1.SPEC.0)
tests/transition_determinism.rs                    (I-DET)
tests/no_hidden_inputs.rs                          (I-NOSIDE)
tests/stale_parent_rejection.rs                    (I-PARENT)
tests/signature_verification.rs                    (I-SIG)
tests/stake_atomicity.rs                           (I-STAKE)
tests/no_wall_clock_in_tx.rs                       (I-LOGTIME)
tests/no_f64_money.rs                              (I-MICROCOIN)
tests/q_state_uses_btree.rs                        (I-BTREE)
tests/no_rejection_sidecar.rs                      (I-NOSIDECAR)
tests/retry_summary_runner_signed.rs               (I-RETRY)
tests/run_terminal_invariant.rs                    (I-TERMINAL)
tests/no_env_in_transition.rs                      (I-NOENV)
tests/task_config_frozen_at_publish.rs             (I-FREEZE-CONFIG)
tests/no_runtime_entropy.rs                        (I-NORANDOM)
tests/verify_target_liveness.rs                    (I-VERIFY-LIVE)
tests/challenge_window_enforced.rs                 (I-CHAL-WINDOW)
tests/finalize_or_slash_exclusive.rs               (I-FINALIZE-EXCLUSIVE)

# Genesis (CO1.0)
tests/genesis_constitution_root_verify.rs
tests/genesis_amendment_predicate_resolves.rs
tests/genesis_initial_registry_empty.rs
tests/genesis_boot_attestation_self_referential.rs
tests/genesis_creator_signature_verifies.rs

# Predicates + Visibility (CO1.5, CO1.11)
tests/safety_creation_dichotomy.rs
tests/private_predicate_error_no_leak.rs
tests/agent_view_filters_internals.rs
tests/agent_view_minimal_context.rs
tests/goodhart_shield.rs

# Signals (CO1.9, CO1.10)
tests/signal_dichotomy.rs
tests/boolean_signal_pass_fail.rs
tests/statistical_signals_complete.rs
tests/price_broadcast_l6.rs
tests/price_aggregation_correlation_shield.rs

# Reports (CLAUDE.md Report Standard)
tests/report_standard_pput_ci_required.rs
tests/halt_reason_distribution.rs
tests/entropy_diversity_thresholds.rs

# Economic invariants (CO P2.*)
tests/economic_invariant_INV1_no_thinking_reward.rs
tests/economic_invariant_INV2_no_direct_collect.rs
tests/economic_invariant_INV3_escrow_only.rs
tests/economic_invariant_INV4_no_post_mint.rs
tests/economic_invariant_INV5_yes_no_event_bound.rs
tests/economic_invariant_INV6_predicate_gated.rs
tests/economic_invariant_INV7_provisional_then_final.rs
tests/economic_invariant_INV8_dag_attribution.rs
tests/economic_invariant_INV9_reputation_immutable.rs
tests/economic_invariant_INV10_signal_vs_evaluator.rs
tests/economic_invariant_INV11_chain_record_only.rs
tests/economic_invariant_INV12_consensus_not_truth.rs

# Economic audit (CO P2.10)
tests/economic_audit_E01_production_default_on.rs
tests/economic_audit_E02_jsonl_summary.rs
tests/economic_audit_E03_naming.rs
tests/economic_audit_E04_founder_grant_law2.rs
tests/no_post_init_mint.rs

# RSP modules + final formula (CO P2.*)
tests/rsp1_modules_smoke.rs
tests/agent_role_economic.rs
tests/final_reward_formula.rs
tests/ctf_stake_symmetry.rs
tests/attribution_engine_determinism.rs

# Retry metadata (CO1.7.0, CO1.9.5)
tests/l6_reconstructibility.rs
tests/failure_histogram_reconstruct.rs

# System keypair (CO1.7.0a-f)
tests/system_keypair_generation.rs
tests/system_keypair_load_and_decrypt.rs
tests/system_keypair_sign_only_from_runner.rs
tests/system_keypair_verify_correctness.rs
tests/system_keypair_rotation_proof.rs

# MetaTx schema (CO P3-prep)
tests/meta_tx_schema_serialization.rs
tests/meta_validator_pass_cases.rs
tests/meta_validator_veto_cases.rs
tests/meta_validator_correctness.rs
tests/amendment_flow_format_validate.rs

# Substrate (CO1.3)
tests/git_substrate_runtime_repo.rs

# Trace matrix self-conformance (CO1.13)
tests/trace_matrix_v3_bidirectional.rs
tests/six_axioms_alignment.rs

# Governance (B-1)
tests/ratification_chain_verifies.rs
tests/dual_audit_protocol_existence.rs

# Cross-domain
tests/architect_proposal_offline.rs
tests/transition_tx_12_fields.rs
tests/anti_oreo_layer_audit.rs
tests/safety_creation_dichotomy.rs (already listed)
```

**Total target test count**: ~80 distinct test files. Some are stubs at v4 ratification (test exists, tests `unimplemented!()`); each will be implemented at the corresponding atom. v4 ship gate: 100% non-stubbed.

---

## § I — Honest Acknowledgements

What this matrix achieves:
- Closes Codex CO P0.7 §2 demand for full Normative coverage
- Closes Gemini v3.2 Q1 PASS qualifier ("every § mapped" claim now actually verifiable)
- Provides ~80-test target for v4 ship + bidirectional code↔doc traceability

What this matrix is honest about:
- §B/§C "Code symbol" column references modules that DON'T YET EXIST in v4 (the matrix anchors future code, which is OK per DO-178C; the test column gives the verification target)
- §F reverse map is empty until CO P1 lands
- Some [N] rows currently fail conformance because corresponding code doesn't exist (this is BY DESIGN — tests are the spec)
- Coverage statistics in §E count rows, not invariants; some [N] rows share invariants

What this matrix does NOT do:
- Generate the conformance tests automatically (each test is a Plan v3.2 atom CO P1.* / CO P2.*)
- Validate that tests actually catch the violation they claim (Codex/Gemini per-atom audits handle that)
- Replace the per-atom doc-comment `/// TRACE_MATRIX <id>: <role>` in each `pub` symbol (R-022 hook enforces at commit time)

— ArchitectAI, 2026-04-27
