# TRACE_MATRIX_v3 — Bidirectional Mapping with N/M/D Classification

> **ARCHIVAL DERIVED VIEW — not current authority.** Retained only as historical evidence. Current flowchart topology authority is `constitution.md` FC1 `455-509`, FC2 `571-660`, FC3 `826-870`, plus the active matrices that explicitly derive from those blocks.

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

### § E.1 Row coverage (matrix rows by classification)

| Source | Total rows | [N] | [M] | [D] |
|---|---|---|---|---|
| Constitution Articles + sub-articles | 27 | 24 | 1 (Art V.2) | 2 (Art 0.3 partial Path A; Art 0.5 future) |
| WP architecture §§ | 21 (incl 0/1/2 plus subsections 5.L0-L6, 7.2, 9.1-4, 12.2, 17 phases) | 17 (full) + 4 (partial / phase-conditional) | 4 (Abstract, § 1, § 14, § 18) | embedded in partial rows |
| WP economic §§ | 8 (numbered 0/2/7/15/18/19/20/21) | 7 | 1 (cross-ref table) | embedded in partial rows |
| RSP appendix | 4 sub-§ | 3 | 1 | — |

**Total Normative coverage**: ~51 rows. Each Normative row has at least 1 conformance test path (existing or planned in Plan v3.2 atoms).

**Test count from this matrix**: ~60-70 distinct conformance tests (some rows share tests; e.g., Goodhart shield).

### § E.2 Code-side backlink coverage (CO1.13.1 measured 2026-04-29 @ HEAD `ad61798`)

| Metric | Count | Notes |
|---|---|---|
| `pub fn / struct / enum / trait / const` items in `src/` | 354 | grep on `pub fn\|pub struct\|pub enum\|pub trait\|pub const` |
| `/// TRACE_MATRIX` doc-comment lines (raw grep) | 154 | includes a few `//! /// TRACE_MATRIX` typos and `/// /// TRACE_MATRIX` doubled-prefix lines |
| `/// TRACE_MATRIX` clean doc-comment lines (parsed) | 135 | strict 1-`///`-prefix item-level doc-comment lines; basis for § F manual population |
| `pub` items with TRACE_MATRIX backlink (immediately preceding doc-block) | 87 | per CO1.13 spec § 9 S7 baseline (semantic block walk per Codex r1 P0-4 measurement at 86%) |
| Files in `src/` containing TRACE_MATRIX reference | 22 / 42 | `grep -rln TRACE_MATRIX src/` ÷ all `src/**/*.rs` |
| **Forward-coverage ratio** (pub items w/ backlink ÷ total pub items) | **24.6 %** (87 / 354) | quantified legacy gap; **CO1.13-extra** closes ≥75 % gap before Phase D per spec § 0.5 (Gemini r1 Q7) |

### § E.3 Shipped atoms covered by § F initial population

| Atom | Source path(s) | Backlinks parsed (clean) |
|---|---|---|
| **CO1.0 / CO1.0a** Trust Anchor + MicroCoin | `src/boot.rs`, `src/economy/money.rs` | 4 (boot.rs); module-level `//!` for money.rs |
| **CO1.1.4-pre1** Typed-tx ABI | `src/state/typed_tx.rs` | 27 |
| **CO1.2** Q_t struct | `src/state/q_state.rs` | 30 |
| **CO1.4 / CO1.4-extra** CAS + sidecar JSONL | `src/bottom_white/cas/{mod,schema,store}.rs` | module-level `//!` only (auto-refresh via CO1.13.3) |
| **CO1.7 / CO1.7-impl A1-A4** transition_ledger + Sequencer | `src/bottom_white/ledger/transition_ledger.rs`, `src/state/sequencer.rs`, `src/state/mod.rs` | 12 (6 + 3 + 3) |
| **CO1.7.0a** system keypair | `src/bottom_white/ledger/system_keypair.rs` | 56 |
| **CO1.7-extra** TuringBus head_t close + Sequencer entry | `src/bus.rs` | 3 |
| Crate-root scaffolding | `src/bottom_white/mod.rs`, `src/bottom_white/ledger/mod.rs` | 3 |

**Forbidden state**: any Normative row with empty "code symbol" or empty "conformance test" column. Pre-commit hook R-022 (added per Plan v3.2 **CO1.13.2** — corrects stale "CO P0.8" attribution) enforces.

---

## § F — Bidirectional Reverse: Code Symbol → Source

This section is the reverse-map: every `/// TRACE_MATRIX <id>: <role>` doc-comment in `src/` paired with the symbol (or variant/field) immediately following. It enables substance-side review (the form-vs-substance two-layer model per CO1.13 spec § 2.5): **R-022** at commit-time enforces *form* (every NEW `pub` symbol carries a backlink); § F enables eventual-consistency *substance* review (every shipped backlink → its claimed alignment row → human/AI verifier checks layer correctness).

**Generation**:
- Initial population done manually by **CO1.13.1** (this section, snapshot at HEAD `ad61798`, 2026-04-29) for shipped atoms (CO1.0 / CO1.0a / CO1.1.4-pre1 / CO1.2 / CO1.4 / CO1.4-extra / CO1.7 / CO1.7-impl A1-A4 / CO1.7.0a / CO1.7-extra).
- Subsequent refresh is auto-generated idempotently by `scripts/update_trace_matrix_reverse_map.py` (lands in **CO1.13.3**), which shares its parser module with `scripts/check_trace_matrix.py` (lands in **CO1.13.2**) per CO1.13 spec § 1.2 + § 1.3.
- **Pre-commit hook R-022** (lands in **CO1.13.2**, NOT in legacy CO P0.8 attribution that v3 doc previously claimed) enforces: every NEW `pub fn / struct / enum / trait / const / mod / type / static` (and `pub(crate)` variants) under `src/` MUST have a `/// TRACE_MATRIX <id>: <role>` doc-comment in the **immediately preceding contiguous doc/attribute/comment/blank-line block** per CO1.13 spec § 2.1 (semantic block walk; Codex r1 P0-4 empirical 86 % accuracy vs 69.4 % under raw 5-line heuristic), OR be filed in § J orphan extensions, OR cite a `[R-022-skip: …]` token in the commit message with `cases/Cxxx | PREREG-§n.m | OBS_R022_*.md` justification per CO1.13 spec § 2.2. Commit aborts (exit 2) if missing.

### § F.1 Coverage caveats (manual-population edition)

Two caveats while CO1.13.3 has not yet shipped:

1. **Module-level `//!` backlinks not enumerated below**. The `src/bottom_white/cas/{mod,schema,store}.rs`, `src/bottom_white/tools/{mod,registry}.rs`, `src/top_white/{mod,predicates/{mod,registry,visibility}}.rs`, `src/economy/{mod,money}.rs`, `src/main.rs` files carry module-level `//! /// TRACE_MATRIX …` lines. CO1.13.3 will pick these up via a separate query (`//!`-prefixed lines whose body matches the same `TRACE_MATRIX <id>: <role>` shape) and merge into § F. v3 manual snapshot covers item-level `///` backlinks only.
2. **Variant/field rows present but R-022 enforcement is at enclosing-symbol granularity** per CO1.13 spec § 1.3 R-022 Scope Table. The variant/field rows below carry their own backlinks (legacy practice; preserved); R-022 itself fires on the parent `pub enum` / `pub struct`. Variant/field backlinks pass form check via the parent's preceding doc-block.

### § F.2 Reverse-map snapshot (HEAD `9be22b4`, auto-refreshed by CO1.13.3)

#### `src/boot.rs`

| Line | Symbol | TRACE_MATRIX backlink |
|---:|---|---|
| 30 | `pub enum TrustRootError` | FC3-N34: failure variants of the readonly-guard verification |
| 62 | `pub fn verify_trust_root` | FC3-N34: implementation of the constitutional `readonly` |
| 246 | `fn` | FC3-N34 + case C-075: child-manifest recursion |
| 281 | `pub fn parse_trust_root_section` | FC3-N34: helper for `verify_trust_root` — exposed because |

#### `src/bottom_white/ledger/mod.rs`

| Line | Symbol | TRACE_MATRIX backlink |
|---:|---|---|
| 5 | `pub mod system_keypair` | FC1-Sig+FC3-Sig: system runtime signature key lifecycle |
| 8 | `pub mod transition_ledger` | FC2-Append + WP § 5.L4: L4 transition ledger (CO1.7 type skeleton) |

#### `src/bottom_white/ledger/system_keypair.rs`

| Line | Symbol | TRACE_MATRIX backlink |
|---:|---|---|
| 40 | `pub struct SystemEpoch` | FC1-Sig+FC3-Sig: system signature epoch identifier |
| 45 | `pub const fn` | FC1-Sig+FC3-Sig: construct a system signature epoch |
| 50 | `pub const fn` | FC1-Sig+FC3-Sig: expose the numeric epoch for canonical encoding |
| 62 | `pub struct SystemPublicKey` | FC1-Sig+FC3-Sig: ed25519 public key pinned by epoch |
| 67 | `pub const fn` | FC1-Sig+FC3-Sig: construct a system public key from raw ed25519 bytes |
| 72 | `pub const fn` | FC1-Sig+FC3-Sig: expose raw public key bytes for pinning and verification |
| 77 | `pub fn fingerprint_sha256` | FC3-Sig: stable SHA-256 fingerprint for audit logs and rotation records |
| 83 | `pub struct SystemSignature` | FC1-Sig+FC3-Sig: ed25519 detached signature over a canonical system message digest |
| 140 | `pub const fn` | FC1-Sig+FC3-Sig: construct a detached system signature from raw ed25519 bytes |
| 145 | `pub const fn` | FC1-Sig+FC3-Sig: expose raw signature bytes for tape serialization |
| 151 | `pub struct RejectedAttemptSummary` | FC1-Sig: typed rejection summary stamped by the predicate runner |
| 161 | `pub fn new` | FC1-Sig: construct a typed rejected-attempt summary, never a free-form sign blob |
| 184 | `pub struct EpochRotationProof` | FC3-Sig: typed continuity statement for system key rotation |
| 195 | `pub const fn` | FC3-Sig: construct a typed epoch-rotation continuity proof |
| 212 | `pub const fn` | FC3-Sig: old signing epoch certified by the rotation proof |
| 217 | `pub const fn` | FC3-Sig: new signing epoch certified by the rotation proof |
| 223 | `pub enum CanonicalMessage` | FC1-Sig+FC3-Sig: only typed runtime messages may enter signature verification |
| 226 | `RejectedAttemptSummary` | FC1-Sig: predicate-runner rejection summary |
| 228 | `TerminalSummarySigning` | FC1-Sig+FC3-Sig (CO1.1.4-pre1 v1.1 closure C-3): terminal |
| 236 | `FinalizeRewardSigning` | FC1-Sig+FC3-Sig (CO1.1.4-pre1 v1.2 closure R2-2): finalize |
| 240 | `TaskExpireSigning` | FC1-Sig+FC3-Sig (CO1.1.4-pre1 v1.2 closure R2-2): task |
| 244 | `EpochRotationProof` | FC3-Sig: system key epoch continuity proof |
| 246 | `LedgerEntrySigning` | FC2-Append (CO1.7 v1.2 round-2 closure C3): L4 transition_ledger |
| 255 | `pub struct PinnedSystemPubkeys` | FC1-Sig+FC3-Sig: epoch-indexed public keys pinned by genesis and rotation history |
| 262 | `pub fn new` | FC1-Sig+FC3-Sig: create an empty pinned system-key map |
| 267 | `pub fn insert` | FC1-Sig+FC3-Sig: pin a public key for a system epoch |
| 276 | `pub fn get` | FC1-Sig+FC3-Sig: fetch the public key pinned for a system epoch |
| 282 | `pub struct Ed25519Keypair` | FC1-Sig+FC3-Sig: in-memory ed25519 system keypair with zeroized private key on drop |
| 291 | `pub fn generate_with_secure_entropy` | FC1-Sig+FC3-Sig: generate ed25519 key material from `getrandom(2)` entropy |
| 306 | `pub const fn` | FC1-Sig+FC3-Sig: return the public half of the system keypair |
| 367 | `pub enum KeypairError` | FC1-Sig+FC3-Sig: system keypair lifecycle and crypto error taxonomy |
| 370 | `Io` | FC1-Sig+FC3-Sig: filesystem operation failed |
| 372 | `Entropy` | FC1-Sig+FC3-Sig: secure operating-system entropy failed |
| 374 | `KdfParam` | FC1-Sig+FC3-Sig: KDF environment parameter was absent or invalid |
| 376 | `Kdf` | FC1-Sig+FC3-Sig: Argon2id key derivation failed |
| 378 | `Crypto` | FC1-Sig+FC3-Sig: ChaCha20-Poly1305 encryption or authentication failed |
| 380 | `InvalidFormat` | FC1-Sig+FC3-Sig: encrypted keystore format was malformed |
| 382 | `HomeUnavailable` | FC1-Sig+FC3-Sig: default keystore path could not be resolved |
| 410 | `pub fn default_system_keystore_path` | FC1-Sig+FC3-Sig: resolve `~/.turingos/keystore/system_keypair_v{epoch}.enc` |
| 425 | `pub fn generate_or_load_system_keypair` | FC1-Sig+FC3-Sig: first-boot generate-or-second-boot decrypt lifecycle entrypoint |
| 440 | `pub fn load_existing_keypair` | FC1-Sig+FC3-Sig: decrypt an existing encrypted system keypair keystore |
| 460 | `pub fn canonical_digest` | FC1-Sig+FC3-Sig: canonical SHA-256 digest for typed system messages |
| 500 | `pub fn verify_system_signature` | FC1-Sig+FC3-Sig: public system signature verification against pinned epoch keys |
| 519 | `pub fn verify_epoch_rotation_proof` | FC3-Sig: verify old and new signatures over a rotation continuity proof |
| 531 | `pub fn verify_system_pubkeys` | FC3-Sig: boot extension stub for genesis `[system_pubkeys]` verification |
| 541 | `pub mod predicate_runner` | FC1-Sig: crate-only signing surface for the predicate runner |
| 548 | `pub fn sign_rejected_attempt_summary` | FC1-Sig: sign only typed rejected-attempt summaries from the predicate runner |
| 559 | `pub fn sign_system_message` | FC1-Sig: sign only typed canonical messages within the predicate-runner scope |
| 568 | `pub mod terminal_summary_emitter` | FC1-Sig+FC3-Sig: crate-only signing surface for system-emitted |
| 584 | `pub fn sign_terminal_summary` | FC1-Sig+FC3-Sig: sign an opaque 32-byte digest of a |
| 593 | `pub fn sign_finalize_reward` | FC1-Sig+FC3-Sig (CO1.1.4-pre1 v1.2 closure R2-2): sign an |
| 604 | `pub fn sign_task_expire` | FC1-Sig+FC3-Sig (CO1.1.4-pre1 v1.2 closure R2-2): sign an |
| 615 | `pub fn sign_epoch_rotation_proof` | FC3-Sig: sign only typed epoch rotation proofs |
| 626 | `pub fn sign_system_message` | FC1-Sig+FC3-Sig: sign only typed canonical messages within terminal-summary scope |
| 635 | `pub mod transition_ledger_emitter` | FC2-Append + FC1-Sig: crate-only signing surface for the L4 |
| 646 | `pub fn sign_ledger_entry` | FC2-Append: sign only the canonical-digest of a |

#### `src/bottom_white/ledger/transition_ledger.rs`

| Line | Symbol | TRACE_MATRIX backlink |
|---:|---|---|
| 48 | `pub enum TxKind` | FC2-Append: discriminator for the typed payload behind a CAS Cid |
| 63 | `pub struct LedgerEntry` | FC2-Append + WP § 5.L4: stored LedgerEntry record (11 fields) |
| 99 | `pub struct LedgerEntrySigningPayload` | FC2-Append C3: the bytes the system keypair actually signs |
| 168 | `pub fn append` | FC2-Append + spec § 4: pure ledger-root fold over signed digests |
| 183 | `pub trait LedgerWriter` | FC2-Append: storage abstraction for L4 |
| 194 | `fn` | § 5 — L4 sequencer post-commit head_t wiring (Art 0.4) |

#### `src/bottom_white/mod.rs`

| Line | Symbol | TRACE_MATRIX backlink |
|---:|---|---|
| 9 | `pub mod ledger` | FC1-Sig+FC3-Sig: Bottom White ledger crypto modules |

#### `src/bus.rs`

| Line | Symbol | TRACE_MATRIX backlink |
|---:|---|---|
| 19 | `pub const PENDING_COMPLETION_TOKENS_CO1_1_4` | FC1-Cost / FC3-Cost: placeholder until CO1.1.4 STEP_B propagates |
| 116 | `pub fn with_sequencer` | § 5.2.1 — single-writer entry-point |
| 134 | `pub fn submit_typed_tx` | § 5.2.1 — typed-tx submission entry |

#### `src/state/mod.rs`

| Line | Symbol | TRACE_MATRIX backlink |
|---:|---|---|
| 9 | `pub mod q_state` | Art 0.4 / WP § 4 — Q_t module: implements all 9 system state fields |
| 12 | `pub mod typed_tx` | FC2-Submit / CO1.1.4-pre1 — typed-tx ABI surface (TypedTx + per-kind structs) |
| 15 | `pub mod sequencer` | § 5.2.1 / CO1.7-impl A2+A3 — L4 sequencer + dispatch_transition |

#### `src/state/q_state.rs`

| Line | Symbol | TRACE_MATRIX backlink |
|---:|---|---|
| 27 | `pub struct Hash` | § 1.1 — generic 32-byte hash (sha256). State / ledger / registry roots |
| 32 | `pub const ZERO` | § 1.1 — additive identity (genesis state-root, ledger-root, etc.) |
| 35 | `pub fn from_bytes` | § 1.1 — construct from a 32-byte digest (sha256 output) |
| 47 | `pub struct NodeId` | Art 0.4 — `head_t` = git commit SHA in Path B substrate (40 hex chars) |
| 52 | `pub fn from_state_root` | § 3 — pseudocode `NodeId::from_state_root(state_root)` constructor |
| 63 | `pub struct AgentId` | § 1.1 — agent identity (string, opaque to Q_t) |
| 67 | `pub struct TxId` | § 1.1 — accepted-transaction id (string, opaque to Q_t) |
| 71 | `pub struct Reputation` | § 1.1 — reputation snapshot. Signed i64 to permit negative reputation |
| 80 | `pub struct AgentSwarmState` | § 1.1 — agent swarm sub-state |
| 88 | `pub struct PerAgentState` | § 1.1 — per-agent runtime state |
| 101 | `pub struct AgentVisibleProjection` | § 1.1 — agent-visible projection of tape filtered by per-agent |
| 114 | `pub struct BudgetSnapshot` | § 1.1 — global budget snapshot: |
| 138 | `pub struct EconomicState` | WP § 2 economic — 9-sub-field economic state. Each sub-index |
| 155 | `pub struct BalancesIndex` | WP § 2 — agent → balance ledger. Concrete entry: `MicroCoin` (CO1.0a) |
| 159 | `pub struct EscrowsIndex` | WP § 2 — tx → escrow entry. Full schema lands CO P2.2 EscrowVault |
| 163 | `pub struct EscrowEntry` | WP § 2 — escrow entry shape (stub). Full fields land CO P2.2 |
| 180 | `pub struct StakesIndex` | WP § 2 — tx → stake entry. Full schema lands CO P2.5 ChallengeCourt |
| 184 | `pub struct StakeEntry` | WP § 2 — stake entry shape (stub). Full fields land CO P2.5 |
| 199 | `pub struct ClaimsIndex` | WP § 2 — tx → reward claim. Full schema lands CO P2.6 SettlementEngine |
| 203 | `pub struct ClaimEntry` | WP § 2 — claim entry shape (stub). Full fields land CO P2.6 |
| 218 | `pub struct ReputationsIndex` | WP § 2 — agent → reputation ledger |
| 222 | `pub struct TaskMarketsIndex` | WP § 2 — tx → task market. Full schema lands CO P2.1 |
| 226 | `pub struct TaskMarketEntry` | WP § 2 — task market entry shape (stub). Full fields land CO P2.1 |
| 259 | `pub struct RoyaltyGraph` | WP § 2 — directed royalty edges (reuse depth attribution) |
| 264 | `pub struct RoyaltyEdge` | WP § 2 — single royalty edge (ancestor → reuse weight). Stub; CO P2.4 |
| 273 | `pub struct ChallengeCasesIndex` | WP § 2 — tx → challenge case. Full schema lands CO P2.5 |
| 277 | `pub struct ChallengeCase` | WP § 2 — challenge case shape (stub). Full fields land CO P2.5 |
| 294 | `pub struct PriceIndex` | WP § 2 — tx → posted price (last accepted price index) |
| 302 | `pub struct QState` | § 1.1 — system state Q_t. 9 fields per WP § 4 + economic § 2 amendment |
| 329 | `pub fn genesis` | Art IV Boot — genesis Q_t. All zero / empty; |

#### `src/state/sequencer.rs`

| Line | Symbol | TRACE_MATRIX backlink |
|---:|---|---|
| 40 | `pub fn dispatch_transition` | § 8 — exhaustive dispatch over `TypedTx` variants |
| 68 | `pub fn advance_head_t` | § 5 — L4 sequencer post-commit head_t wiring (Art 0.4) |
| 206 | `pub struct Sequencer` | § 5.2.1 — L4 sequencer; single-writer per (runtime_repo, run_id) |

#### `src/state/typed_tx.rs`

| Line | Symbol | TRACE_MATRIX backlink |
|---:|---|---|
| 33 | `pub struct TaskId` | § 1.2 — task-market entry id; opaque string |
| 37 | `pub struct RunId` | § 1.5 — runtime run id (one run per `Sequencer` driver lifecycle) |
| 41 | `pub struct ClaimId` | STATE § 3.4 + § 4 I-FINALIZE-BATCH-ORDER — typed claim id used |
| 63 | `pub struct ToolId` | § 1.3 ReuseTx + L2 Tool Registry — opaque tool identifier |
| 67 | `pub struct PredicateId` | § 1.2 PredicateResultsBundle + L1 Predicate Registry — opaque predicate id |
| 71 | `pub struct ReadKey` | § 1.2 WorkTx field 5 — read-set key (DAG attribution / replay) |
| 77 | `pub struct WriteKey` | § 1.2 WorkTx field 6 — write-set key (DAG attribution / replay) |
| 85 | `pub struct AgentSignature` | § 1.2 WorkTx field 10 + I-SIG: agent-side detached Ed25519 |
| 111 | `pub struct SlashEvidenceCid` | § 1.2 TxStatus::FinalizedSlash — typed reference to the |
| 121 | `pub struct BoolWithProof` | § 1.2 PredicateResultsBundle — boolean predicate verdict |
| 130 | `pub enum SafetyOrCreation` | § 1.2 PredicateResultsBundle — safety-class discriminator |
| 147 | `pub struct PredicateResultsBundle` | § 1.2 WorkTx field 8 — runner-stamped predicate results |
| 161 | `pub enum RejectionClass` | § 1.4 — classification of a rejected attempt |
| 175 | `pub enum VerifyVerdict` | § 1.3 VerifyTx field 5 — verifier verdict |
| 183 | `pub enum RunOutcome` | § 1.5 TerminalSummaryTx field 4 + Art. IV halt-reason taxonomy |
| 195 | `pub enum TxStatus` | § 1.2 TxStatus — **runtime book-keeping only** (D-1 divergence |
| 212 | `pub struct WorkTx` | § 1.2 — agent-submitted work transaction (12-field schema; |
| 238 | `pub struct VerifyTx` | § 1.3 — verifier verdict transaction |
| 256 | `pub struct ChallengeTx` | § 1.3 — challenge transaction (counter-example posted with |
| 269 | `pub struct ReuseTx` | § 1.3 — fact-tx recording reuse of a tool created by a prior |
| 280 | `pub struct FinalizeRewardTx` | CO1.1.4-pre1 spec § 4 — derived schema (STATE spec § 3.4 |
| 313 | `pub struct TaskExpireTx` | STATE spec § 3.6 v1.3 — system-emitted task-expiry tx |
| 327 | `pub struct TerminalSummaryTx` | STATE spec § 1.5 — system-emitted no-accept-run handler |
| 603 | `pub enum TypedTx` | § 8 dispatch_transition — typed-tx outer enum |
| 638 | `pub trait HasSubmitter` | STATE spec § 3.6.5 v1.3 — submitter resolution trait used |
| 706 | `pub enum TransitionError` | STATE § 3 — transition-function error taxonomy. v1.1 covers |
| 824 | `pub struct SignalBundle` | STATE § 3 — tape-emitted signal bundle. v1 minimal: a single |

**Total**: 135 `///`-doc-comment backlinks across 10 source files (HEAD `9be22b4`). Auto-refreshed by `scripts/update_trace_matrix_reverse_map.py` per CO1.13.3.

— end of § F manual snapshot.
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
- §F reverse map was empty at v4 ratification (2026-04-27) and is now manually populated for shipped atoms by **CO1.13.1** at HEAD `ad61798` (2026-04-29); subsequent refresh is auto-generated by CO1.13.3
- Some [N] rows currently fail conformance because corresponding code doesn't exist (this is BY DESIGN — tests are the spec)
- Coverage statistics in §E count rows, not invariants; some [N] rows share invariants

What this matrix does NOT do:
- Generate the conformance tests automatically (each test is a Plan v3.2 atom CO P1.* / CO P2.*)
- Validate that tests actually catch the violation they claim (Codex/Gemini per-atom audits handle that)
- Replace the per-atom doc-comment `/// TRACE_MATRIX <id>: <role>` in each `pub` symbol (R-022 hook enforces at commit time)

— ArchitectAI, 2026-04-27 (§ F populated + § J added 2026-04-29 per CO1.13.1)

---

## § J — Orphan Extensions

> **Added by CO1.13.1 (2026-04-29) per CO1_13_TRACE_MATRIX_IMPL_v1_2026-04-29.md spec § 0.3 v1.1.1 + § 1.1 (Codex r1 P0-3 + Codex r2 New-P0-2 § G→§ J rename).**
>
> **Purpose**: register `pub`-style symbols in `src/` whose constitutional alignment is real but does not fit the canonical § A/§ B/§ C/§ D row schema (e.g., a constitutional concept lives across multiple WP sections, or a symbol implements scaffolding/glue that supports the matrix but has no single canonical row). § J is the **non-row-bound fallback target** that R-022 falls through to in step 3 of its check (per spec § 2.1).
>
> **Authority**: § J is a constitutional-surface extension. Each row MUST cite a justification reference (one or more of `cases/Cxxx`, `PREREG-§n.m`, or a sedimented `OBS_R022_*.md`). The justification reference protects against the silent-bypass risk: any symbol that lands here without a justification reference is a violation; R-022 enforces this at commit time per spec § 2.1 step 3.
>
> **Lifecycle**: a row is **opened** when an atom ships a symbol that R-022 needs to skip (e.g., a meta-tooling helper that supports the alignment regime itself), and **closed** by either (a) graduating into a canonical § A/§ B/§ C/§ D row in a subsequent doc revision when the atom matures, or (b) being deleted alongside the symbol.
>
> **Audit cadence**: § J is reviewed during the quarterly R-022 enforcement-log audit (per spec § 2.2 "Quarterly audit reviews accumulated R-022-SKIP log entries"). Suspicious accumulation (≥10 entries new since prior quarter, or any row missing a valid justification reference) triggers a factory halt per Elon-mode OBS-threshold policy (CO1.13 spec § 0.5).

### § J.1 Schema

| Column | Required | Description |
|---|---|---|
| **File path** | yes | absolute path under `src/` to the file containing the symbol |
| **Symbol** | yes | the `pub` (or `pub(crate)`) symbol exempted from row-bound matrix mapping; for variants/fields, use `EnumName::VariantName` notation |
| **Class** | yes | one of: **scaffolding** (test/devtools surface), **cross-row** (legitimately spans ≥2 § A/§ B rows), **placeholder** (will graduate to canonical row when atom matures), **legacy** (pre-CO1.13 untraced, deferred to CO1.13-extra closure) |
| **Justification ref** | yes | `cases/C-xxx`, `PREREG-§n.m`, or `OBS_R022_<topic>_<date>.md`. Must exist in repo at commit time; R-022 will reject if reference missing |
| **Opened atom** | yes | the atom that introduced or registered this orphan (e.g., `CO1.13.1`) |
| **Graduation target** | optional | atom-id where this row is expected to graduate to canonical § A/§ B/§ C/§ D row; blank if no graduation planned |
| **Notes** | optional | any further detail (e.g., "graduates when CO P2.1 lands TaskMarket bindings") |

### § J.2 Open orphan rows

| File path | Symbol | Class | Justification ref | Opened atom | Graduation target | Notes |
|---|---|---|---|---|---|---|
| `src/bottom_white/ledger/transition_ledger.rs` | `canonical_decode` | cross-row | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 686. |
| `src/bus.rs` | `append` | cross-row | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 219. |
| `src/bus.rs` | `append_oracle_accepted` | cross-row | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 236. |
| `src/drivers/llm_http.rs` | `generate` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 88. |
| `src/ledger.rs` | `append` | cross-row | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 237. |
| `src/lib.rs` | `bottom_white` | cross-row | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 2. |
| `src/lib.rs` | `kernel` | cross-row | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 6. |
| `src/lib.rs` | `ledger` | cross-row | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 7. |
| `src/lib.rs` | `sdk` | cross-row | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 10. |
| `src/lib.rs` | `state` | cross-row | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 11. |
| `src/lib.rs` | `top_white` | cross-row | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 12. |
| `src/lib.rs` | `wal` | cross-row | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 13. |
| `src/runtime/adapter.rs` | `tb_real6a_emit_task_outcome_no_after_exhaustion` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 764. |
| `src/runtime/adapter.rs` | `tb_real6a_seed_task_outcome_market_after_escrow` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 1504. |
| `src/runtime/adapter.rs` | `tb_real6a_invest_task_outcome_to_router_tx` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 1589. |
| `src/runtime/agent_scheduler.rs` | `SCHEDULER_DECISION_TRACE_SCHEMA_ID` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 14. |
| `src/runtime/agent_scheduler.rs` | `SchedulerPnlSignal` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 37. |
| `src/runtime/agent_scheduler.rs` | `write_scheduler_decision_trace_to_cas` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 61. |
| `src/runtime/agent_scheduler.rs` | `read_scheduler_decision_trace_from_cas` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 78. |
| `src/runtime/agent_scheduler.rs` | `scheduler_decision_trace_cids` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 87. |
| `src/runtime/agent_scheduler.rs` | `build_observe_only_scheduler_trace` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 130. |
| `src/runtime/agent_scheduler.rs` | `render_scheduler_trace_section` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 153. |
| `src/runtime/attempt_telemetry.rs` | `decode_attempt_telemetry_shared_slot` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 897. |
| `src/runtime/g7_structural_smoke.rs` | `G7_STRUCTURAL_GUARD_SCHEMA_ID` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 9. |
| `src/runtime/g7_structural_smoke.rs` | `G7StructuralGuard` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 45. |
| `src/runtime/g7_structural_smoke.rs` | `structural_fixture` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 57. |
| `src/runtime/g7_structural_smoke.rs` | `G7StructuralGuardSummary` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 72. |
| `src/runtime/g7_structural_smoke.rs` | `write_g7_structural_guard_to_cas` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 81. |
| `src/runtime/g7_structural_smoke.rs` | `summarize_g7_structural_guards_from_cas` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 98. |
| `src/runtime/mod.rs` | `real6_task_outcome` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 226. |
| `src/runtime/mod.rs` | `real6_attempt_prediction` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 230. |
| `src/runtime/mod.rs` | `real6_conviction_budget` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 234. |
| `src/runtime/prompt_capsule.rs` | `PROMPT_CAPSULE_V2_SCHEMA_ID` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 64. |
| `src/runtime/prompt_capsule.rs` | `PromptCapsuleV2` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 115. |
| `src/runtime/prompt_capsule.rs` | `assert_matches_assignment` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 129. |
| `src/runtime/prompt_capsule.rs` | `read_set_resolves` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 154. |
| `src/runtime/prompt_capsule.rs` | `write_prompt_capsule_v2_to_cas` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 270. |
| `src/runtime/prompt_capsule.rs` | `read_prompt_capsule_v2_from_cas` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 289. |
| `src/runtime/real5_roles.rs` | `ROLE_ASSIGNMENT_MANIFEST_SCHEMA_ID` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 21. |
| `src/runtime/real5_roles.rs` | `ROLE_TURN_TRACE_SCHEMA_ID` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 22. |
| `src/runtime/real5_roles.rs` | `ToolName` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 24. |
| `src/runtime/real5_roles.rs` | `PolicyId` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 25. |
| `src/runtime/real5_roles.rs` | `HeadT` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 26. |
| `src/runtime/real5_roles.rs` | `AgentRole` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 29. |
| `src/runtime/real5_roles.rs` | `ALL` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 41. |
| `src/runtime/real5_roles.rs` | `fn` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; parser_ambiguity=pub_const_fn; checker-compatible key `fn`; actual functions: `label` at line 52 and `kind` at line 421. |
| `src/runtime/real5_roles.rs` | `AgentRoleAssignment` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 91. |
| `src/runtime/real5_roles.rs` | `RoleAssignmentManifest` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 101. |
| `src/runtime/real5_roles.rs` | `sorted_role_assignment` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 108. |
| `src/runtime/real5_roles.rs` | `default_allowed_tools` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 115. |
| `src/runtime/real5_roles.rs` | `role_assignment_from_csv` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 128. |
| `src/runtime/real5_roles.rs` | `write_role_assignment_manifest_to_cas` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 167. |
| `src/runtime/real5_roles.rs` | `read_role_assignment_manifest_from_cas` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 184. |
| `src/runtime/real5_roles.rs` | `detect_hidden_role_switch` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 193. |
| `src/runtime/real5_roles.rs` | `render_role_assignment_dashboard` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 220. |
| `src/runtime/real5_roles.rs` | `DerivedViewRequest` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 236. |
| `src/runtime/real5_roles.rs` | `PriceSignal` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 244. |
| `src/runtime/real5_roles.rs` | `PublicErrorSummary` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 251. |
| `src/runtime/real5_roles.rs` | `DerivedView` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 257. |
| `src/runtime/real5_roles.rs` | `DerivedViewInput` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 268. |
| `src/runtime/real5_roles.rs` | `fixture` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 275. |
| `src/runtime/real5_roles.rs` | `derive_role_view` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 287. |
| `src/runtime/real5_roles.rs` | `derive_role_view_with_context_bytes` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 294. |
| `src/runtime/real5_roles.rs` | `WorkTxPayload` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 367. |
| `src/runtime/real5_roles.rs` | `VerifyPeerPayload` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 372. |
| `src/runtime/real5_roles.rs` | `ChallengePayload` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 377. |
| `src/runtime/real5_roles.rs` | `MarketInvestPayload` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 382. |
| `src/runtime/real5_roles.rs` | `LiquidityPayload` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 388. |
| `src/runtime/real5_roles.rs` | `ToolProposalPayload` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 394. |
| `src/runtime/real5_roles.rs` | `VetoPayload` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 399. |
| `src/runtime/real5_roles.rs` | `AbstainPayload` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 404. |
| `src/runtime/real5_roles.rs` | `RoleAction` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 409. |
| `src/runtime/real5_roles.rs` | `RoleActionRejection` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 436. |
| `src/runtime/real5_roles.rs` | `RoleActionRoute` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 442. |
| `src/runtime/real5_roles.rs` | `parse_role_action_json` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 455. |
| `src/runtime/real5_roles.rs` | `legacy_tool_to_role_action` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 490. |
| `src/runtime/real5_roles.rs` | `route_role_action` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 513. |
| `src/runtime/real5_roles.rs` | `TickBudget` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 560. |
| `src/runtime/real5_roles.rs` | `TickEvent` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 568. |
| `src/runtime/real5_roles.rs` | `derive_tick_budget` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 575. |
| `src/runtime/real5_roles.rs` | `RationalPrice` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 599. |
| `src/runtime/real5_roles.rs` | `new` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 605,796. |
| `src/runtime/real5_roles.rs` | `NoTradeReasonTrace` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 618. |
| `src/runtime/real5_roles.rs` | `TraderTurnWitness` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 631. |
| `src/runtime/real5_roles.rs` | `verify_trader_turns` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 636. |
| `src/runtime/real5_roles.rs` | `ScriptedTradeRoute` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 654. |
| `src/runtime/real5_roles.rs` | `scripted_positive_edge_trade` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 659. |
| `src/runtime/real5_roles.rs` | `VerifyPeerFixture` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 674. |
| `src/runtime/real5_roles.rs` | `verify_peer_fixture` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 681. |
| `src/runtime/real5_roles.rs` | `apply_verifier_reputation_delta` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 694. |
| `src/runtime/real5_roles.rs` | `NoVerifyReason` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 703. |
| `src/runtime/real5_roles.rs` | `VerifierTurnWitness` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 709. |
| `src/runtime/real5_roles.rs` | `NoChallengeReason` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 715. |
| `src/runtime/real5_roles.rs` | `ChallengeDecisionTrace` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 721. |
| `src/runtime/real5_roles.rs` | `challenge_decision_trace` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 728. |
| `src/runtime/real5_roles.rs` | `RoleTurnOutcome` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 747. |
| `src/runtime/real5_roles.rs` | `RoleTurnTrace` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 785. |
| `src/runtime/real5_roles.rs` | `write_role_turn_trace_to_cas` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 816. |
| `src/runtime/real5_roles.rs` | `RoleTurnTraceSummary` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 834. |
| `src/runtime/real5_roles.rs` | `summarize_role_turn_traces_from_cas` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 846. |
| `src/runtime/real5_roles.rs` | `MetricEstimate` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 889. |
| `src/runtime/real5_roles.rs` | `ToolProposal` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 896. |
| `src/runtime/real5_roles.rs` | `VetoVerdict` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 904. |
| `src/runtime/real5_roles.rs` | `VetoReasonClass` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 910. |
| `src/runtime/real5_roles.rs` | `VetoDecision` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 917. |
| `src/runtime/real5_roles.rs` | `proposal_activation_status` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 924. |
| `src/runtime/real5_roles.rs` | `Real5SmokeInput` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 941. |
| `src/runtime/real5_roles.rs` | `Real5SmokeReport` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 959. |
| `src/runtime/real5_roles.rs` | `evaluate_real5_smoke` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 966. |
| `src/runtime/real6_attempt_prediction.rs` | `REAL6B_SCHEMA_ID` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 18. |
| `src/runtime/real6_attempt_prediction.rs` | `REAL6B_STAGE_LIMIT` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 19. |
| `src/runtime/real6_attempt_prediction.rs` | `LeanOracleResult` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 23. |
| `src/runtime/real6_attempt_prediction.rs` | `AttemptPredictionStepKind` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 29. |
| `src/runtime/real6_attempt_prediction.rs` | `AttemptPredictionStep` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 40. |
| `src/runtime/real6_attempt_prediction.rs` | `fn` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; parser_ambiguity=pub_const_fn; checker-compatible key `fn`; actual function: `is_role_window_tick` at line 53. |
| `src/runtime/real6_attempt_prediction.rs` | `AttemptPredictionFixture` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 64. |
| `src/runtime/real6_attempt_prediction.rs` | `first_logical_t` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 83. |
| `src/runtime/real6_attempt_prediction.rs` | `attempt_prediction_event_id` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 91. |
| `src/runtime/real6_attempt_prediction.rs` | `build_scripted_attempt_prediction_fixture` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 99. |
| `src/runtime/real6_attempt_prediction.rs` | `validate_attempt_prediction_fixture` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 229. |
| `src/runtime/real6_attempt_prediction.rs` | `write_attempt_prediction_fixture_to_cas` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 318. |
| `src/runtime/real6_attempt_prediction.rs` | `attempt_prediction_fixture_cids` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 338. |
| `src/runtime/real6_conviction_budget.rs` | `ConvictionBudget` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 26. |
| `src/runtime/real6_conviction_budget.rs` | `ConvictionAction` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 36. |
| `src/runtime/real6_conviction_budget.rs` | `ConvictionActionAvailability` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 47. |
| `src/runtime/real6_conviction_budget.rs` | `derive_conviction_budget` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 52. |
| `src/runtime/real6_conviction_budget.rs` | `conviction_action_allowed` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 88. |
| `src/runtime/real6_conviction_budget.rs` | `route_role_action_with_conviction_budget` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 117. |
| `src/runtime/real6_conviction_budget.rs` | `render_scoped_conviction_budget_summary` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 161. |
| `src/runtime/real6_conviction_budget.rs` | `write_significant_loss_autopsy_to_cas` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 183. |
| `src/runtime/real6_task_outcome.rs` | `TaskOutcomeMarketKind` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 16. |
| `src/runtime/real6_task_outcome.rs` | `TaskOutcomeEvent` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 22. |
| `src/runtime/real6_task_outcome.rs` | `task_outcome_event_for_task` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 31. |
| `src/runtime/real6_task_outcome.rs` | `task_outcome_price_signal` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 49. |
| `src/runtime/real6_task_outcome.rs` | `TaskOutcomeMarketSeedOutcome` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 62. |
| `src/sdk/mod.rs` | `actor` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 1. |
| `src/sdk/mod.rs` | `error_abstraction` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 10. |
| `src/sdk/mod.rs` | `prompt` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 18. |
| `src/sdk/mod.rs` | `prompt_guard` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 19. |
| `src/sdk/mod.rs` | `protocol` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 20. |
| `src/sdk/mod.rs` | `sandbox` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 21. |
| `src/sdk/mod.rs` | `snapshot` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 22. |
| `src/sdk/mod.rs` | `tool` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 23. |
| `src/sdk/mod.rs` | `tools` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 24. |
| `src/sdk/tools/mod.rs` | `search` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 2. |
| `src/sdk/tools/mod.rs` | `wallet` | scaffolding | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 3. |
| `src/state/typed_tx.rs` | `canonical_digest` | cross-row | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 2011. |
| `src/state/typed_tx.rs` | `to_legacy_signing_payload` | cross-row | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 2144. |
| `src/wal.rs` | `path` | cross-row | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 45. |
| `src/wal.rs` | `replay` | cross-row | `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` | REAL-10 Atom 1 | REAL-10 TRACE cleanup | REAL-5S->REAL-9 R-022 cleanup; skipped lines 69. |
### § J.3 Closed / graduated rows (audit trail)

| File path | Symbol | Reason closed | Closed atom | Closed date |
|---|---|---|---|---|
| `src/runtime/attempt_telemetry.rs` | `trace_removal` | Legacy TRACE_MATRIX backlink removal recorded by R-022 skip; audit-trail row only, not an open public surface. | REAL-10 Atom 1 | 2026-05-15 |
| `src/state/typed_tx.rs` | `trace_removal` | Legacy TRACE_MATRIX backlink removal recorded by R-022 skip; audit-trail row only, not an open public surface. | REAL-10 Atom 1 | 2026-05-15 |

### § J.4 R-022 fallback semantics (cross-reference)

When `scripts/check_trace_matrix.py --mode commit` (CO1.13.2) runs, fallback step 3 (per CO1.13 spec § 2.1) searches § J.2 for `<file_path>:<symbol_name>` matching the staged-diff NEW pub symbol. PASS criteria:

1. The exact `<file_path>:<symbol_name>` must appear in § J.2 (case-sensitive; no fuzzy match).
2. The row must have a non-empty `Justification ref` column.
3. The justification reference must exist in the repo at commit time (file must be reachable from repo root, e.g., `cases/C-075.yaml` exists).

If any of (1)/(2)/(3) fails, R-022 falls through to step 4 (commit-message `[R-022-skip: …]` token check). If both fail, R-022 BLOCKS the commit and appends a structured log entry per spec § 2.2.

— end of § J Orphan Extensions (CO1.13.1 initial schema; rows populated commit-by-commit via R-022 skip-token use).
