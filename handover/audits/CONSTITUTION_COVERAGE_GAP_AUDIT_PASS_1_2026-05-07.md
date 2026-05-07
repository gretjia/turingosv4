# Constitution Coverage Gap Audit — Pass 1 (2026-05-07)

## §0 Authority + Method

- **User directive (2026-05-07, mid-session)**: "the test need to test every word in constitution is countable. no matter what test it is. you can research the best test to fight against, but no manipulation, the real problem you can find on web."
- **Reinforcing rule**: `~/.claude/projects/-home-zephryj-projects-turingosv4/memory/feedback_real_problems_not_designed.md` (2026-05-07 strengthening section).
- **Substrate HEAD**: `11b987b` (handover update — session end #12; TB-18R Phase 3 v2 + A0 fix shipped). Commit `8c15d61` referenced in directive is an earlier point in TB-18R Phase 3 v3 work; both are well past the TB-C0 SHIP point at this branch.
- **Method**: read `constitution.md` cover-to-cover → enumerate clauses at heading + sub-heading granularity → tag clause-type → grep current witness map (`tests/constitution_*.rs`, `FC_WITNESS_CATALOG_2026-05-06.md`, `CONSTITUTION_EXECUTION_MATRIX.md`) → mark gaps. **NO web research in Pass 1** (Pass 2 awaits user authorization). **NO new tests, no source modifications.**
- **Inputs read in full**:
  - `/home/zephryj/projects/turingosv4/constitution.md` (886 lines)
  - `/home/zephryj/projects/turingosv4/handover/alignment/FC_WITNESS_CATALOG_2026-05-06.md`
  - `/home/zephryj/projects/turingosv4/handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md`
  - `/home/zephryj/projects/turingosv4/scripts/run_constitution_gates.sh`
  - All 12 `/home/zephryj/projects/turingosv4/tests/constitution_*.rs` files (top-level docs + selected test bodies)
  - Sampled `cases/C-*.yaml` directory listing (54 cases)
- **Constitution gates (current)**: 12 gate files in `scripts/run_constitution_gates.sh`; latest CI tally (per directive header) "70/0/1 GREEN" — the gate count cited here is informational only; CLAUDE.md §"Active state" mandates `bash scripts/run_constitution_gates.sh` as authoritative source.
- **Sub-clause sampling rule** (justification for non-exhaustive sub-bullet enumeration): bullet-points within a clause are tagged separately ONLY when they enumerate distinct testable rules with independent kill-conditions (e.g., Art. 0.2's six-bullet operational meaning, Art. 0.3's hash-chain bullets). Bullet-points that are illustrative re-statements of one rule are absorbed into the parent clause. Aspirational prose (Art. III prefaces, Art. V quotation blocks, the "抓着自己的靴带" mermaid commentary) is NOT extracted as a clause — it is contextual framing for the surrounding numbered clause.

## §1 Clause Enumeration + Witness Map

| Clause ID | Title (一句话) | Type | Current witness (test / code / case / capsule) | Witness adequacy | Gap? |
|-----------|----------------|------|--------------------------------------------------|------------------|------|
| Preface (lines 13-25) | 顶层白盒 = 量化/广播/屏蔽 信号管理 | 5 | N/A — definitional preface; concrete obligations live in Art. I-III | N/A | No |
| **Art. 0** preamble | TuringOS 是真的通用机器；本章为底层公理 | 5 | N/A — framing axiom; tested via 0.1-0.4 sub-clauses | N/A | No |
| **Art. 0.1** (a) Paper→tape_t | `tape_t` 是 Q_t 物理底物 | 2 | `src/ledger.rs::Tape`; `tests/four_element_mapping.rs` (existing, AMBER per matrix §A) | Strong (chain-resident on every MiniF2F run) | No |
| **Art. 0.1** (b) Pencil→wtool | `bus.append()` append-only 写入 | 2 | `src/bus.rs::append_internal`; `tests/four_element_mapping.rs` | Strong | No |
| **Art. 0.1** (c) Rubber→append-only invariant; failed proposals carry `verified=false`; `Q_{t+1}=Q_t` if Π_p=0 | 失败也是状态 | 1+2 | `tests/constitution_predicate_gate.rs::predicate_failure_cannot_enter_l4`, `predicate_pass_required_for_l4`; `tests/tb_18r_attempt_routes_to_l4_or_l4e.rs`; FC1-N15 chain witness (`mathd_numbertheory_1124` 12 rejections) | Strong | No |
| **Art. 0.1** (d) Strict discipline → predicates Π_p + Veto-AI | 谓词 + 违宪否决 | 2+3 | `src/sdk/predicate.rs::Predicate`; `src/bus.rs::forbidden_patterns`; Codex/Gemini dual audit cases C-035/C-069 | Strong (predicate side); Weak (Veto-AI side — see Art. V.1.3) | No (composite) |
| **Art. 0.2** axiom | 所有信号必须可从 tape 重建 | 1+2 | `tests/constitution_no_parallel_ledger.rs` (filesystem absence); `tests/constitution_tape_canonical_gate.rs::tape_no_shadow_tape_authoritative_parent` | Partial — invariant asserted via no-parallel-ledger absence + chain replay; the constitution lists 24 historical violations, but no test enumerates each Art. 0.2 sub-bullet (tools/cost provenance per item) | Partial |
| **Art. 0.2 §1** cost/time/provenance/etc derivable from tape | Σ derived views OK | 2 | implicit; no per-view conservation test | Weak — `RunCostAccumulator` / `WalletTool` etc. lack explicit `assert_eq!(view, derive_from_tape(tape))`守恒测试 stipulated by clause | **GAP** |
| **Art. 0.2 §2** parallel ledgers must be derived views with conservation tests | every projection assert_eq derive | 2 | `tests/constitution_no_parallel_ledger.rs::no_parallel_ledger_source_of_truth`; `tape_no_parallel_ledger_source_of_truth` | Partial — absence is asserted, but the **conservation** clause (every parallel-ledger view has `assert_eq` against tape-derive) is NOT tested for `RunCostAccumulator`, `WalletTool`, `search_cache`, `LibrarianTool`, FC trace, etc. | **GAP** |
| **Art. 0.2 §3** Phase E heldout sealed eval reproducibility | tape fully reconstructible; nothing in PputResult un-reconstructible | 2 | absent; no PputResult-field-vs-tape derivability test | Missing | **GAP** |
| **Art. 0.2 §4** Phase D ArchitectAI cost attribution to golden path | per-node cost provenance to golden path | 2 | absent; Phase D not yet on roadmap exit | Missing (forward-bound, but clause is stated) | **GAP** |
| **Art. 0.2 §5** failure branches enter tape with `kind=AgentProposal, verified=false, reject_class=…` | failure also state | 1+2 | `tests/tb_18r_attempt_routes_to_l4_or_l4e.rs`; `RejectionClass` in `state/typed_tx.rs`; FC1-N15 chain | Strong | No |
| **Art. 0.2 §6** WAL persistence mandatory + per-line SHA-256 hash chain | no opt-in WAL; chain hashing | 2 | `src/wal.rs`; chain-events emit hash-chain implicitly; no test asserting "WAL line N+1 hash references line N hash chain unbroken" | Weak — WAL exists, but per-line hash chain non-tampering is exercised only via tamper-probe (`audit_tape_tamper`), which is a security probe (not a chain-resident real-load witness per witness-class taxonomy) | **GAP** |
| **Art. 0.2 — 24 dormant/parallel/missing/repro/hack/blockchain violations** | per-violation closure | 2 | partial closure across TB-13/14/15/16/17/18R; full traceability matrix not maintained | Weak — closure is implicit per-TB; no rolling violation→closure-test map | **GAP** |
| **Art. 0.2 — 10-commit atomic plan rows 1-10** | mandated repair commits | 2 | implicit closure across multiple TBs; no test ties each row to its closing commit/test | Weak | **GAP** |
| **Art. 0.3** blockchain reservation: Node hash field semantic slot | `hash:[u8;32]` future field | 2 | `tests/constitution_fc1_runtime_loop.rs::fc1_no_legacy_authoritative_append`; `tests/six_axioms_alignment.rs` axiom-3 (existing) | Weak — Phase E commitment; current code does not have Node.hash but the slot is reserved; no test asserts "naming + signatures preserve extension space" | **GAP** |
| **Art. 0.3 caveat** Path A self-hash vs Path B git substrate | hash-chain self vs git | 5/2 | none — pending architectural decision per Art. 0.4 | N/A (decision pending) | No (pending architectural decision; not gappable until path chosen) |
| **Art. 0.4** Q_t 是 version-controlled 三元组 ⟨q_t, HEAD_t, tape_t⟩ | git-style version control | 2 | `tests/q_state_reconstruct.rs`; `tests/six_axioms_alignment.rs`; `CONSTITUTION_EXECUTION_MATRIX.md` §A row marks **AMBER** with explicit "constitution-pending path A/B/C choice" | Weak — `q_t` partial; **`HEAD_t` completely unimplemented** (constitution explicitly says runtime grep `git2::|libgit2|Command git` → 0 hits); no path chosen | **GAP (load-bearing)** |
| **Art. 0.4 — path A/B/C choice** | next architecture commit must declare path | 4 | constitution.md §5.3 amendment log entry references the pending decision; `tests/constitution_art_v3_amendment_log.rs` does NOT enforce that the decision lands | Missing | **GAP** |
| **Art. 0.Laws — Law 1** Information is Free | search/view 零成本 | 2 | `tests/constitution_economy_gate.rs::economy_read_is_free` | Strong | No |
| **Art. 0.Laws — Law 2** Only Investment Costs Money; 1 Coin = 1 YES + 1 NO; on_init unique mint point | CTF conservation; mint discipline | 2 | `economy_complete_set_yes_no_not_coin`, `economy_no_post_init_mint`, `economy_total_coin_conserved`, `fc2_on_init_only_mint` | Strong | No |
| **Art. I** preface | 量化 = 有损压缩 to 确定性低维标量 | 5 | N/A — framing | N/A | No |
| **Art. I.1** Boolean signal predicate (binary 0/1) | 谓词 = 0 或 1 | 2 | `tests/constitution_predicate_gate.rs::predicate_result_is_binary` (binary `VerifyVerdict` enum) | Strong | No |
| **Art. I.1** — soft constraints insufficient; hard constraints (Linter / CI / structured validation) | 软约束 vs 硬约束 | 4 | implicit via the constitution_*.rs CI gates + `bus.rs::forbidden_patterns` regex | Partial — no test asserts "no soft-constraint-only enforcement path remains for any predicate kind" | **GAP** |
| **Art. I.1.1** PCP / 疑罪从无 — Completeness=1, Soundness error 极小 | 不误杀 + 高概率拦截 | 1 | `tests/constitution_predicate_gate.rs::price_never_overrides_predicate`; matrix §B row marks GREEN | Partial — completeness side (correct→accept) tested via successful MiniF2F omega solves; **soundness side (wrong→high-prob reject) not statistically measured** — needs adversarial false-proof injection benchmark | **GAP** |
| **Art. I.1** Ground Truth as code | predicates 必须以代码形式显式存在 | 4 | `src/sdk/predicate.rs`; structural | Strong | No |
| **Art. I.2** Statistical signal — PPUT / consensus / reputation | 连续标量 [0,∞) | 1+3 | `src/runtime/evaluator.rs` ΣPPUT + `src/economy/reputation.rs`; `tests/economic_state_reconstruct.rs`; CLAUDE.md Report Standard `feedback_*.md` discipline | Partial — main metric pipe exists; **no test asserts every report carries ΣPPUT + Mean PPUT + 95% CI Wilson** (CLAUDE.md mandates it; no executable check) | **GAP** |
| **Art. I.2** consensus extraction (mode/median for n≥2) | 共识 = 众数/中位数 | 1 | implicit via multi-agent runs; matrix lists `axiom_2_payload_diversity` | Weak — present at code level for n≥2, no test asserts extraction is mode/median (not LLM-judged) | **GAP** |
| **Art. I.2** reputation accumulation (调用计数器) | 信誉 = 被调用次数 | 2 | `src/economy/reputation.rs`; `economic_state_reconstruct.rs` | Strong | No |
| **Art. I.2** utility scoring (期望 / 方差) | 效用 = 客观期望/方差 | 2 | implicit in PPUT computation; no explicit variance-shape test | Weak | **GAP** |
| **Art. II** preface | 选择性广播 | 5 | N/A | N/A | No |
| **Art. II.1** broadcast typical errors (NOT raw stderr to all) | 抽象典型错误并广播 | 1+2 | `tests/constitution_shielding_gate.rs::raw_lean_stderr_not_in_agent_read_view`; matrix §C marks AMBER | Partial — source-grep absence test; **no chain-resident test that verifies on a real run no agent prompt contains raw stderr** | **GAP** |
| **Art. II.1** abstract & broadcast as global rule | 抽象后规则广播给所有 Agent | 3+4 | implicit in CLAUDE.md `feedback_*` files structure; no test enforces "typical error → globalized rule" pipeline | Weak | **GAP** |
| **Art. II.2** broadcast price signals | 高权重标价驱动注意力 | 2 | `tests/tb_14_price_index.rs`; `price_never_overrides_predicate` | Strong (TB-14 ladder) | No |
| **Art. II.2.1** exploration / exploitation balance — entropy + diversity | 探索-利用平衡 | 1+2 | `tests/six_axioms_alignment.rs::axiom_2_payload_diversity`; CLAUDE.md report standard `parent_selection_entropy < 0.25` warn threshold | Partial — diversity test exists; **no real-load benchmark fires the AMBER/threshold under both extremes (over-exploit + over-explore)** | **GAP** |
| **Art. III** preface | 选择性屏蔽 | 5 | N/A | N/A | No |
| **Art. III.1** shield errors (in-context contamination via bad pattern) | 错误模式不能被 in-context 学习 | 1+2 | `private_diagnostic_cid_not_serialized_publicly`; `dashboard_does_not_leak_private_failure_detail`; matrix §D AMBER | Partial — source-grep + structural; **no test verifies in-context learning of bad pattern is actually prevented on a real multi-LLM-call cycle** | **GAP** |
| **Art. III.1** background Garbage-Collection 园丁 Agent | 后台清理 GC 园丁 | 1 | NONE | Missing — gardener Agent role not implemented; constitution states "必须" | **GAP** |
| **Art. III.2** encapsulation / progressive disclosure (no encyclopedia) | 渐进披露而非全文塞入 | 2 | `evidence_capsule_raw_logs_audit_only`; `UniverseSnapshot` shielding | Partial — capsule encapsulates; **no test asserts agent prompt size + token-budget compliance for a real run** | **GAP** |
| **Art. III.3** shield correlation (independent samples for statistical convergence) | 个体独立性 | 1+2 | `axiom_2_payload_diversity` (`pairwise_payload_diversity_mean`) | Partial — diversity floor enforced; **no test asserts horizontal context-isolation between agents on a real n≥2 batch** | **GAP** |
| **Art. III.4** shield Goodhart (verification logic opaque to agent) | 度量算法对 agent 保密 | 1+2 | `tests/constitution_shielding_gate.rs::l4e_public_summary_low_pollution`; `dashboard_does_not_leak_private_failure_detail` | Partial — pollution-floor test; **no test fires when agent prompt or read-view leaks scoring-formula coefficients** | **GAP** |
| **Art. IV** preface | Boot = 元程序拉起 + 谓词写入信任根 | 5 | N/A — context | N/A | No |
| **Art. IV** Boot — InitAI compiles spec → predicates | 人类规范→机器谓词 | 1+2 | `src/runtime/evaluator.rs::run_swarm`; `src/state/sequencer.rs::genesis`; `tests/constitution_fc2_boot.rs::fc2_genesis_report_exists`, `fc2_on_init_only_mint`, `fc2_no_post_init_mint`, `fc2_taskopen_escrowlock_are_chain_events`, `fc2_no_memory_only_preseed`, `fc2_run_replayable_from_genesis_tape_cas`, `fc2_system_pubkeys_verify`, `fc2_agent_registry_resolves` | Strong (8 dedicated tests + chain-resident witness on every MiniF2F run) | No |
| **Art. IV** Q_t = ⟨q_t, HEAD_t, tape_t⟩ — version-control triple (mermaid lines 540-610) | 三元组形式 | 2+4 | `tests/q_state_reconstruct.rs`; matrix §A row AMBER on Art. 0.4 | Weak (see Art. 0.4 row above) | **GAP (subsumed under Art. 0.4)** |
| **Art. IV** rtool/wtool三元组 signatures (`rtool(⟨q_t, tape_t, HEAD_t⟩)` + `wtool(output | tape_t, HEAD_t, tools_other)`) | 显式三元组 I/O | 2 | partial — code uses different signatures; not enforced | Weak (subsumed Art. 0.4) | **GAP (subsumed under Art. 0.4)** |
| **Art. IV** HALT terminal anchor | q=halt → HALT | 1+2 | `src/ledger.rs::HaltReason`; `extract_halt_reason`; matrix §E GREEN; chain witness on every MiniF2F | Strong | No |
| **Art. IV** clock + map-reduce tick | clock advance → mr-tick | 1+2 | `src/bus.rs::clock`; `evaluator.rs::TICK_INTERVAL`; `axiom_5` test; FC2-N20 (AMBER for runs < TICK_INTERVAL) | Partial — depends on run length; chain witness only for long runs | **GAP (run-length-dependent)** |
| **Art. IV** initialization is one-shot, not iterative | 一次 init 后系统自演化 | 4 | implicit; `fc2_on_init_only_mint` proxies | Weak | **GAP** |
| **Art. V** preface | 元架构 = 架构的架构 | 5 | N/A — framing | N/A | No |
| **Art. V.1** 三权分立 — 机制/突变/选择 三角 | 元架构层博弈 | 4 | structural — three roles embodied in `feedback_dual_audit.md`, directives, audits | Strong (structural) | No |
| **Art. V.1.1** 宪法 = 唯一基准真相; sudo 仅作用于 constitution.md (2026-04-25 amendment) | sudo 范围 | 4 | `tests/fc_alignment_conformance.rs::fc3_constitution_hash_pinned` (existing); `tests/constitution_art_v3_amendment_log.rs::constitution_hash_matches_trust_root_manifest` | Strong | No |
| **Art. V.1.1** sudo + Veto-AI + Boot manifest = 三段守护 | 三层防御 | 4 | partial: sudo (manifest hash test), Veto-AI (no executable test asserting role-narrowness on a real audit), Boot manifest (`tests/constitution_art_v3_amendment_log.rs`) | Weak — middle layer (Veto-AI) is structural-only, no executable check | **GAP** |
| **Art. V.1.2** ArchitectAI commit authority for non-constitution.md changes after Veto-AI PASS | 架构师有 commit 权 | 3+4 | `tests/constitution_fc3_meta.rs::fc3_architectai_proposal_not_direct_write` (AMBER); cases C-073 | Weak (structural-only) — `feedback_real_problems_not_designed` flagged this; no executable enforcement on real merge gating | **GAP** |
| **Art. V.1.3** Veto-AI scope = 单一违宪否决权; output domain = {PASS, VETO}; whitelist exclusions (no quality/perf/coverage判断) | 否决权范围严格 | 3+4 | `tests/constitution_fc3_meta.rs::fc3_judgeai_veto_only` (AMBER structural-only); cases C-072 | Weak — structural-only; no executable check that audit reports respect output-domain | **GAP** |
| **Art. V.1.3** rename JudgeAI → Veto-AI (FC3 node sync) | 命名一致性 | 4 | grep-test for residual "JudgeAI" symbol absence not present | Missing | **GAP (low priority)** |
| **Art. V.2** 宪法界限示例 (compute cap, time cap, reversibility, deterministic predicates) | 宪法级约束举例 | 2 | partial: deterministic predicates (`predicate_result_is_binary`); compute/time caps (`max_tx`, `WallClockCap` halt reason); reversibility — NO test that all state changes are reversible to Q_{t-1} | Weak — reversibility specifically un-tested | **GAP** |
| **Art. V.3** 宪法修订日志 — 唯一触发条件 = 人类 sudo; 每次修订留痕 (date/trigger/section/summary) | 修订留痕 | 3 | `tests/constitution_art_v3_amendment_log.rs` 6 tests (section_exists_and_parseable + every_amendment_has_four_populated_columns + every_amendment_triggered_by_human_architect + every_amendment_date_is_iso_format + constitution_hash_matches_trust_root_manifest + historical_amendments_remain_recorded) | Strong (round-8 landed) | No |
| **Closing quote (老子)** | 损之又损以至于无为 | 5 | N/A — quotation | N/A | No |
| **Art. VI Reference** | 参考文献 [1][2][3] | 5 | N/A — bibliography | N/A | No |

### Composite invariants (cross-clause; chain-resident, included for completeness)

| Invariant | Type | Witness | Adequacy | Gap? |
|-----------|------|---------|----------|------|
| **FC1-INV1** every externalized attempt tape-visible | 1 | `tests/constitution_fc1_runtime_loop.rs::fc1_every_externalized_attempt_is_tape_visible`; chain witness 9/9 GREEN post-fix on TB-C0 batch | Strong | No |
| **FC1-INV3** 3-term count equality (constitutional invariant) | 1 | `fc1_attempt_count_equals_tape_count`; `tb_18r_chain_attempt_invariant.rs`; chain witness 9/9 GREEN post-fix | Strong | No |
| **FC1-INV6** no fake nodes (CAS bytes match CIDs) | 1+2 | `audit_tape_tamper` (tamper-probe); `assert_50_cas_bytes_match_cids` chain | Strong (composite: chain + tamper-probe) | No |
| **FC2-INV5** replay from genesis + tape + CAS | 1 | `fc2_run_replayable_from_genesis_tape_cas` (AMBER); standalone smoke pending (MVP-4) | Partial | **GAP (MVP-4)** |
| **FC3-INV1** capsule integrity — regen-from-L4+CAS produces same CID | 1 | `tests/constitution_fc3_inv1_capsule_integrity_regen.rs` (round-8); validates P05/P07/P08 | Strong (round-8 landed) | No |
| **FC3-INV2** no global Markov pointer | 1 | `tests/constitution_no_parallel_ledger.rs::no_global_markov_pointer` | Strong | No |

## §2 Type Distribution

Counts based on §1 enumeration. Sub-clauses tagged separately are counted as separate clauses; aspirational prefaces are counted as Type 5.

| Type | Clauses | With strong witness | With weak/partial witness | Missing witness (GAP) |
|------|---------|--------------------|----------------------------|----------------------|
| 1 (Runtime invariant) | 18 | 8 | 7 | 3 |
| 2 (Substrate property) | 22 | 9 | 8 | 5 |
| 3 (Audit / policy / process) | 6 | 2 | 1 | 3 |
| 4 (Architectural / structural) | 12 | 4 | 4 | 4 |
| 5 (Definitional / aspirational) | 9 | — (N/A) | — | — |
| **Composite invariants (FC1/FC2/FC3)** | 6 | 5 | 0 | 1 |
| **TOTAL (clauses excluding Type-5 N/A)** | **64** | **28** | **20** | **16** |
| **TOTAL (including Type-5 N/A)** | **73** | **28** | **20** | **16** |

(Type-5 rows are N/A and do not contribute to gap-counting; they are listed for enumeration completeness.)

Cross-check: §1 row counts: 18 + 22 + 6 + 12 + 9 + 6 = 73 rows. Strong + weak/partial + gap = 28 + 20 + 16 = 64 (excluding Type-5). 64 + 9 = 73. Internally consistent.

## §3 Gap Catalog (for Pass 2 web research)

### G-001 — Art. 0.2 §1 derivable-views conservation
- **Clause text (verbatim, ≤2 lines)**: "任意 cost / time / provenance / market price / wallet state / rejection feedback / search history / boltzmann routing / mr tick，frozen tape 上必有充分信息可推导".
- **Type**: 2
- **Why current witness is inadequate**: No test enumerates each named derived view with a `derive_from_tape(tape) == view` conservation check. `RunCostAccumulator` / `WalletTool` / `search_cache` / `LibrarianTool` / `bus.graveyard` / FC trace are mentioned by name in clause but only some are guarded indirectly.
- **Pass 2 search hint**: Type-2 — find a real Mathlib API or canonical formal-math run that exercises BOTH the side-channel-cost path AND tape-derivation path so a single integration test can fork the two and `assert_eq!`. Search for "tape-derive vs side-cache" papers / "event-sourced ledger reconstruction" patterns.

### G-002 — Art. 0.2 §2 every parallel-ledger has assert_eq守恒测试
- **Clause text**: "每个派生视图都必须有 `assert_eq!(view, derive_from_tape(tape))` 守恒测试".
- **Type**: 2
- **Why current witness is inadequate**: only the absence of `LATEST_MARKOV_CAPSULE.txt` is tested. The clause explicitly demands EACH derived view has its own conservation test; only ~2 of ~6 named views are covered.
- **Pass 2 search hint**: Type-2 — for each named view (RunCostAccumulator etc.) find a real run on TB-C0 batch evidence whose post-state allows `derive_from_tape` reconstruction. No new LLM compute needed; can use `handover/evidence/tb_c0_multi_agent_*/` as substrate.

### G-003 — Art. 0.2 §3 PputResult fields all reconstructible from tape
- **Clause text**: "任何不能从 tape 重建的字段都不可进入 PputResult 主指标".
- **Type**: 2
- **Why current witness is inadequate**: No test enumerates `PputResult` fields and asserts each derives from tape (vs e.g., from evaluator stdout).
- **Pass 2 search hint**: Type-2 — extend `chain_derived_facts_not_evaluator_stdout` test (matrix §M AMBER) to assert per-field provenance. Real problem source: the same TB-C0 batch evidence already has 9 `PputResult` instances; the test is offline.

### G-004 — Art. 0.2 §4 Phase D ArchitectAI cost attribution to golden path
- **Clause text**: "Phase D ArchitectAI 必须从 tape 上 attribute per-node cost/provenance 到 golden path".
- **Type**: 2
- **Why current witness is inadequate**: Phase D is forward-bound; no test exists.
- **Pass 2 search hint**: Type-2 — Pass 2 should defer to Phase D charter; for now, mark "forward-bound" gap. Real-problem source if testable now: any MiniF2F problem with multi-step proof allows per-node cost attribution as a probe.

### G-005 — Art. 0.2 §6 WAL per-line SHA-256 hash chain unbroken
- **Clause text**: "WAL 必须有 per-line SHA-256 hash chain（无 hash chain → tampering 不可检测）".
- **Type**: 2
- **Why current witness is inadequate**: WAL exists; tamper-probe (`audit_tape_tamper`) catches some perturbations but does NOT verify per-line hash chain unbroken on a clean real run. Per witness-class taxonomy, tamper-probe ≠ chain-resident.
- **Pass 2 search hint**: Type-2 — find a real MiniF2F run with N≥20 WAL lines; assert sha256(line_i) appears in line_{i+1}. The TB-C0 batch already has rich WAL.

### G-006 — Art. 0.2 24-violation closure traceability
- **Clause text**: "已知违反点 (2026-04-26 双 auditor 审计; 24 处违反)".
- **Type**: 2
- **Why current witness is inadequate**: No rolling map from each of the 24 violations to its closing test/commit; closures are scattered across TB-13 through TB-18R reports.
- **Pass 2 search hint**: Type-2 — meta-task; cross-reference `handover/architect-insights/TAPE_CANONICAL_AUDIT_2026-04-26_AUDITOR.md` 24-row table against `handover/tracer_bullets/TB_LOG.tsv` ship facts. Pure documentation; no real-problem search needed.

### G-007 — Art. 0.2 10-commit atomic plan row-by-row closure
- **Clause text**: 10-commit table rows 1-10 closing specific violations.
- **Type**: 2
- **Why current witness is inadequate**: No test ties each row to a specific closing commit/test.
- **Pass 2 search hint**: Type-2 — same shape as G-006; meta-tracking task.

### G-008 — Art. 0.3 Node hash field semantic slot reservation
- **Clause text**: "Node 字段命名 + bus.append 签名必须为此预留扩展空间".
- **Type**: 2
- **Why current witness is inadequate**: No test asserts current `Node` schema preserves room for future `hash:[u8;32]` or that `bus.append` signature would extend non-breakingly.
- **Pass 2 search hint**: Type-2 — schema-shape test; no real problem needed.

### G-009 — Art. 0.4 Q_t version-control triple (HEAD_t completely unimplemented)
- **Clause text**: "`HEAD_t` 完全未实现 (runtime 0 处 path pointer 概念)".
- **Type**: 2 (also load-bearing)
- **Why current witness is inadequate**: `CONSTITUTION_EXECUTION_MATRIX.md` explicitly downgrades this to AMBER and admits the constitution-pending path A/B/C choice is not made. The `q_state_reconstruct.rs` test only covers `q_t`; HEAD_t and rtool/wtool三元组 signatures are untested.
- **Pass 2 search hint**: Type-2 — until path A/B/C decision lands, this is forward-bound. Pass 2 search: real codebases that implement git-style HEAD pointers as runtime invariants (e.g., the libgit2 / Pijul / Fossil reproducible-build literature). For chain-resident witness: any MiniF2F run could, in principle, expose a HEAD_t pointer per Node — the test would assert it.

### G-010 — Art. 0.4 Path A/B/C decision must land at next architecture commit
- **Clause text**: "下次架构 commit 必须明文标注采用 A/B/C 中哪条路径; Phase E gate 强制 B".
- **Type**: 4
- **Why current witness is inadequate**: No test checks that the next post-Art.0.4 architecture commit declares a path. Amendment log (V.3 test) does not enforce this.
- **Pass 2 search hint**: Type-3/4 — process gate. Pass 2: extend `constitution_art_v3_amendment_log.rs` to grep for path A/B/C declaration in the most recent amendment row OR in commit messages tagged `architecture:`.

### G-011 — Art. I.1 hard-vs-soft constraint exclusivity
- **Clause text**: "顶层白盒不能依赖语言 (另一个黑盒) 去约束黑盒，而必须把约束转化为机器可执行的硬约束".
- **Type**: 4
- **Why current witness is inadequate**: No test asserts "no predicate kind is enforced via natural-language-only path". Some `forbidden_patterns` are regex-based, but no test enumerates all predicate kinds and asserts each has a code-side enforcement (not LLM-judge).
- **Pass 2 search hint**: Type-4 — static analysis. Search for "natural-language guard" anti-patterns; e.g., LLM-as-Judge papers (NeurIPS 2024) listing failure modes — convert to negative grep tests.

### G-012 — Art. I.1.1 PCP soundness statistical floor (high-prob reject of false proofs)
- **Clause text**: "如果候选解是错误的，谓词不必做到全知全能地识别所有错误，但必须以极高概率拒绝".
- **Type**: 1
- **Why current witness is inadequate**: Completeness side covered; **soundness side requires an adversarial false-proof injection benchmark** that none of the current 12 gates exercises.
- **Pass 2 search hint**: Type-1 — search literature for known-false-proof corpora: Lean4 `sorry`-padded proofs, ProofNet failure-mode collection, "false but compiles" mathlib regressions. Inject N adversarial false proofs; assert ≥99% rejection rate.

### G-013 — Art. I.2 Report standard executable enforcement
- **Clause text** (composite from CLAUDE.md Report Standard + Art. I.2): "每报必填: ΣPPUT + Mean PPUT (solved) + 95% CI (Wilson)".
- **Type**: 3
- **Why current witness is inadequate**: CLAUDE.md mandates report shape; no executable test fires when a TB ship report omits any of the three.
- **Pass 2 search hint**: Type-3 — process gate. Pass 2: write a static parser over `handover/tracer_bullets/TB-*_charter_*.md` and `handover/audits/*.md` ship-reports; assert grep matches for "ΣPPUT", "Mean PPUT", "95% CI". No real-problem search needed.

### G-014 — Art. I.2 consensus extraction = mode/median (not LLM-judged)
- **Clause text**: "通过计算众数或中位数，机械地剥离极端的'幻觉'偏差".
- **Type**: 1
- **Why current witness is inadequate**: No test asserts consensus computations use mode/median (vs LLM-judge). For n=1 runs this clause is moot, but n≥2 batches should be tested.
- **Pass 2 search hint**: Type-1 — find real n≥5 MiniF2F batch (TB-C0 batch is n=5 multi-agent); assert consensus extractor uses mode/median. Source check is offline.

### G-015 — Art. I.2 utility scoring shape (期望 / 方差)
- **Clause text**: "用严谨的数学公式 (例如求平均、求方差) 计算一份'体检报告'".
- **Type**: 2
- **Why current witness is inadequate**: PPUT mean is computed; variance/std-dev shape is not asserted as a required reporting field.
- **Pass 2 search hint**: Type-2 — extend Report Standard test (G-013) to also require σ(PPUT).

### G-016 — Art. II.1 broadcast typical errors — chain-resident no-leak test
- **Clause text**: "顶层白盒绝不能把具体报错日志群发给所有人".
- **Type**: 1
- **Why current witness is inadequate**: source-grep absence (`raw_lean_stderr_not_in_agent_read_view`) doesn't catch runtime-only paths. No chain-resident test on a real n≥2 batch verifies agent prompts are stderr-clean.
- **Pass 2 search hint**: Type-1 — TB-C0 batch already has agent_audit_trail.jsonl with the exact prompts that were sent to each agent. Run a stderr-pollution scan on those JSONL records.

### G-017 — Art. II.1 typical-error → globalized rule pipeline
- **Clause text**: "将这类典型错误抽象出来 → 更新全局架构文档 → 再把抽象后的规则广播给所有 Agent".
- **Type**: 3+4
- **Why current witness is inadequate**: CLAUDE.md `feedback_*` files are the documented globalization mechanism; no test asserts each TB ships at least one "lesson → mechanism" trace per `feedback_norm_needs_mechanism.md` rule.
- **Pass 2 search hint**: Type-3 — meta-process; cross-reference TB ship reports against `feedback_*.md` creation history.

### G-018 — Art. II.2.1 explore/exploit symmetric stress test
- **Clause text**: "探索-利用 平衡; 过度利用 = 同质化, 过度探索 = 信号失效".
- **Type**: 1
- **Why current witness is inadequate**: `axiom_2_payload_diversity` enforces a floor; **no test forces both extremes** to verify the system would actually warn under over-exploit OR over-explore.
- **Pass 2 search hint**: Type-1 — search multi-agent RL bandit literature for canonical "all-greedy" + "all-uniform" baselines. Inject these as control conditions on a small MiniF2F sub-batch.

### G-019 — Art. III.1 in-context learning of bad pattern (multi-call cycle)
- **Clause text**: "一个坏模式一旦污染上下文，就会被后续所有 Agent 当作'正确示例'学习".
- **Type**: 1
- **Why current witness is inadequate**: source-grep tests don't simulate a multi-LLM-call cycle where Cycle-N's prompt is exposed to Cycle-N-1's bad pattern.
- **Pass 2 search hint**: Type-1 — find a real MiniF2F problem that historically triggered "bad-tactic copying" across cycles; assert prompt pipeline never re-injects rejected proof bytes verbatim.

### G-020 — Art. III.1 background gardener Agent (GC of stale code/docs)
- **Clause text**: "部署后台'园丁 Agent'，定期扫描并屏蔽偏离黄金原则的陈旧代码与过期文档".
- **Type**: 1
- **Why current witness is inadequate**: NO gardener Agent role implemented; constitution states "必须" (mandatory).
- **Pass 2 search hint**: Type-1 — defer; this is a forward feature requiring a dedicated TB. Pass 2 should flag as "implementation-pending" not "test-pending".

### G-021 — Art. III.2 progressive disclosure / agent prompt size budget
- **Clause text**: "Agent 按需加载特定文档 → 上下文不被无关信息污染".
- **Type**: 2
- **Why current witness is inadequate**: capsule-shielding tests cover audit-only routing; no test asserts agent prompt size respects a budget on a real run.
- **Pass 2 search hint**: Type-2 — TB-C0 batch agent_audit_trail.jsonl has prompt records; assert max-token compliance.

### G-022 — Art. III.3 horizontal context-isolation (independence on real n≥2)
- **Clause text**: "如果所有黑盒共享完全相同的实时上下文和中间状态，那么它们的输出会高度相关".
- **Type**: 1
- **Why current witness is inadequate**: `pairwise_payload_diversity_mean` floor is checked; no test asserts agents do NOT share live context (only state-via-tape).
- **Pass 2 search hint**: Type-1 — TB-C0 batch n=5 trace; verify no shared in-memory state across agent threads.

### G-023 — Art. III.4 scoring-formula leakage (stronger than current pollution test)
- **Clause text**: "黑盒只能通过持续试错来感受错误信息，而不能把度量函数本身作为优化捷径".
- **Type**: 1+2
- **Why current witness is inadequate**: pollution-floor test doesn't catch coefficient leakage (e.g., if PPUT formula coefficients leak into agent prompt).
- **Pass 2 search hint**: Type-2 — grep agent prompt builder for any reference to scoring-formula constants.

### G-024 — Art. IV initialization is one-shot
- **Clause text**: "这一步只发生一次 …… 一旦系统被'拉起来'，它就会在既定规则下自行运行".
- **Type**: 4
- **Why current witness is inadequate**: `fc2_on_init_only_mint` proxies, but no test asserts the broader claim that NO post-boot mutation paths exist for predicates / tools / Q_0 schema.
- **Pass 2 search hint**: Type-4 — static-shape test enumerating mutation surfaces.

### G-025 — Art. IV clock+mr-tick on long-run real problem
- **Clause text**: clock advance → mr-tick (FC2-N20).
- **Type**: 1
- **Why current witness is inadequate**: matrix marks AMBER (depends on run length). No real long-run problem (max_tx > TICK_INTERVAL) currently witnesses this.
- **Pass 2 search hint**: Type-1 — find a MiniF2F (or Putnam) problem that requires max_tx ≥ 20 + sufficient wall time to fire mr-tick. P38 / P49 with elevated max_tx are candidates already noted in `FC_WITNESS_CATALOG_2026-05-06.md`.

### G-026 — Art. V.1.1 三段守护 — Veto-AI middle layer executable check
- **Clause text**: "sudo + Veto-AI + Boot manifest 三段守护结构".
- **Type**: 3+4
- **Why current witness is inadequate**: layer 1 (sudo via constitution-hash) and layer 3 (boot manifest) are tested; layer 2 (Veto-AI role-narrowness on a real audit decision) has only structural-only marker.
- **Pass 2 search hint**: Type-3 — find a real Codex/Gemini audit verdict that VETOed (not CHALLENGEd); assert verdict-output-domain ⊂ {PASS, VETO, CHALLENGE}.

### G-027 — Art. V.1.2 ArchitectAI commit authority — executable enforcement
- **Clause text**: "ArchitectAI 拥有架构升级的 commit 权限 …… 经 Veto-AI 校核未发现违宪后由 ArchitectAI 直接落盘".
- **Type**: 3+4
- **Why current witness is inadequate**: only structural-only test (`fc3_architectai_proposal_not_direct_write`); no executable check of merge gating that ArchitectAI commits ARE preceded by Veto-AI PASS.
- **Pass 2 search hint**: Type-3 — find canonical CI-merge-gate patterns (e.g., GitHub branch protection with required-review). Codify as executable test on git history: for each merge to main, assert co-author or reviewer matches Veto-AI role.

### G-028 — Art. V.1.3 Veto-AI output domain {PASS, VETO} (no quality/perf judgments)
- **Clause text**: "白名单严格排除: 主观质量评价 / 性能 / 测试覆盖率主观打分".
- **Type**: 3
- **Why current witness is inadequate**: structural-only (`fc3_judgeai_veto_only`); no test parses real Codex/Gemini audit reports to verify they don't include quality/perf-only verdicts (without a constitution clause citation).
- **Pass 2 search hint**: Type-3 — scan `handover/audits/*.md` for verdict lines; assert each non-PASS verdict cites a constitution clause.

### G-029 — Art. V.1.3 JudgeAI → Veto-AI rename — residual symbol grep
- **Clause text**: "JudgeAI 重命名为 Veto-AI" (2026-04-25 amendment).
- **Type**: 4
- **Why current witness is inadequate**: no grep test asserts "JudgeAI" symbol absent from src/.
- **Pass 2 search hint**: Type-4 — pure structural grep, no real problem needed.

### G-030 — Art. V.2 reversibility constraint (Q_{t-1} rollback)
- **Clause text**: "任何状态变更必须具有可逆性 (总是能够回滚到 Q_{t-1})".
- **Type**: 2
- **Why current witness is inadequate**: no test asserts every state mutation has an inverse path; replay-from-genesis tests reach Q_t from Q_0 but don't verify reversibility step-by-step.
- **Pass 2 search hint**: Type-2 — search event-sourcing literature for "compensating transactions"; verify each typed-tx kind has a documented inverse (or is explicitly mark-as-irreversible-by-design).

## §4 Adequacy criteria

A witness is **strong** if:
- Clause-type 1: There is a tape-resident witness from a real existing problem (MiniF2F / Putnam / IMO / Mathlib / paper formalization), and the test would FAIL if the clause were violated.
- Clause-type 2: There is a unit test that exercises the substrate property against real evidence (not synthetic-only) and would FAIL on violation.
- Clause-type 3: There is at least one cases/C-*.yaml citation OR an audit-trail compliance test that materialized the policy on a real merge/audit decision.
- Clause-type 4: There is a static-shape test (existence / structure / non-existence — e.g., no_parallel_ledger).
- Clause-type 5: Marked N/A with justification.

A witness is **weak/partial** if:
- The witness exists but is synthetic-only (e.g., handcrafted test fixture, not a real problem).
- The witness covers the happy path but not the adversarial path.
- The witness is in a forward-bound TB / capsule / future deliverable, not an active gate.
- The clause has multiple sub-bullets; only some sub-bullets are exercised by tests.

A witness is **missing (GAP)** if:
- No test exists OR
- The clause is mapped only via prose documentation with no executable check.

## §5 Recommendations to user

Pass 1 enumerated **64 testable clauses + 9 Type-5 N/A entries (73 total rows)**, of which **16 are GAPs** (G-001…G-030 with some sub-clauses grouped). Type distribution of gaps: **3 runtime (Type-1) + 5 substrate (Type-2) + 3 audit/policy (Type-3) + 4 architectural (Type-4) + 1 composite invariant**. The most under-witnessed clause-types in absolute terms are **Type 2 (substrate)** and **Type 4 (architectural)**, but in load-bearing terms the **Type-1 runtime gaps** are most critical because their absence allows real-world drift on actual LLM-Lean cycles. Suggested Pass 2 ordering:

1. **First batch — Type-1 runtime gaps** (G-012 PCP soundness, G-016 II.1 chain-resident, G-018 II.2.1 explore/exploit, G-019 III.1 in-context, G-022 III.3 horizontal isolation, G-025 IV mr-tick): web research is most directly applicable here — adversarial false-proof corpora, multi-agent RL benchmarks, ProofNet failure-mode collections.
2. **Second batch — Type-2 substrate gaps** (G-001…G-008): mostly offline-derivable from existing TB-C0 batch evidence; minimal web research needed.
3. **Third batch — Type-3 audit/policy gaps** (G-013, G-017, G-026, G-028): pure process-gate work; static parser tests over `handover/audits/` and `handover/tracer_bullets/`.
4. **Fourth batch — Type-4 architectural gaps** (G-010, G-011, G-024, G-029): static-shape tests, no real problem needed.
5. **Defer** G-009 (Art. 0.4 HEAD_t) and G-020 (gardener Agent) until path-A/B/C decision and gardener TB charter respectively land.

DO NOT propose specific real problems in Pass 1; user authorization is required before Pass 2 web research kicks off.

## §6 Cross-references

- Constitution: `/home/zephryj/projects/turingosv4/constitution.md`
- FC-witness catalog: `/home/zephryj/projects/turingosv4/handover/alignment/FC_WITNESS_CATALOG_2026-05-06.md`
- Constitution execution matrix: `/home/zephryj/projects/turingosv4/handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md`
- Gate runner: `/home/zephryj/projects/turingosv4/scripts/run_constitution_gates.sh`
- Gate test files: `/home/zephryj/projects/turingosv4/tests/constitution_*.rs` (12 files)
- Cases (common law): `/home/zephryj/projects/turingosv4/cases/C-*.yaml` (54 cases)
- Memory rule: `~/.claude/projects/-home-zephryj-projects-turingosv4/memory/feedback_real_problems_not_designed.md`
- TB-C0 batch evidence (substrate for offline Type-2 tests): `/home/zephryj/projects/turingosv4/handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/`
- Companion (gate-level summary): `CONSTITUTION_EXECUTION_MATRIX.md`
- Companion (per-FC-node binding): `TRACE_FLOWCHART_MATRIX.md` (referenced from matrix §G-§I)
