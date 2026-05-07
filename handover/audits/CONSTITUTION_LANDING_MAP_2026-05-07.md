# 宪法落地地图 (TuringOS Constitution Landing Map) — 2026-05-07

> **生成方式**: 综合 Pass 1 (clause enumeration + witness map, 73 行) + Pass 2 (per-gap closure approach, 30 gap) 数据,按宪法章节重组。每条宪法子句标 ✅/⚠️/❌/🛑/🟡/⚪ 状态 + 未落地原因 + 解锁动作。
>
> **HEAD**: `38bb57d` (Pass 1+2 commit base; current session 已新增 untracked 文件,内容稳定)
> **Constitution**: `constitution.md` 886 行
> **Authoritative source**: Pass 1 + Pass 2 audit deliverables (cross-referenced per row)
>
> **Status legend**:
> - ✅ **LANDED** — 强 witness;违反则 test FAIL
> - ⚠️ **PARTIAL** — witness 存在但弱(synthetic-only / happy-path-only / sub-bullet 未覆盖)
> - ❌ **NOT-LANDED** — 无 executable witness,Pass 3 无依赖可立即开工
> - 🛑 **BLOCKED-DECISION** — gap 存在 AND 闭合需要 user/architect 决策
> - 🟡 **DEFERRED-FORWARD** — gap 存在 AND 闭合 deferred to forward TB charter
> - ⚪ **N/A** — Type-5 definitional/aspirational;无 test 义务

---

## §0 仪表板 (Dashboard)

### 总览

| 状态 | 计数 | 占比 (含 N/A) | 占比 (排除 N/A) |
|------|------|---------------|-----------------|
| ✅ LANDED | 28 | 38.4% | 43.8% |
| ⚠️ PARTIAL | 13 | 17.8% | 20.3% |
| ❌ NOT-LANDED | 14 | 19.2% | 21.9% |
| 🛑 BLOCKED-DECISION | 7 | 9.6% | 10.9% |
| 🟡 DEFERRED-FORWARD | 2 | 2.7% | 3.1% |
| ⚪ N/A | 9 | 12.3% | — |
| **总计** | **73** | 100% | 100% |

(64 testable + 9 Type-5 N/A = 73 行,与 Pass 1 §2 内部一致。30 gap 中 7 BLOCKED-DECISION + 2 DEFERRED-FORWARD + 21 NOT-LANDED;PARTIAL 13 行中部分也有 G-ID,见 §2 全表。)

### 按章节落地率

| 章节 | LANDED | PARTIAL | NOT-LANDED + BLOCKED + DEFERRED | LANDED 单独落地率 | LANDED+PARTIAL 落地率 |
|------|--------|---------|---------------------------------|-------------------|----------------------|
| Art. 0 图灵机原教旨 (含 Laws) | 6 | 2 | 9 | 35% | 47% |
| Art. I 信号量化 | 4 | 3 | 4 | 36% | 64% |
| Art. II 选择性广播 | 1 | 2 | 3 | 17% | 50% |
| Art. III 选择性屏蔽 | 0 | 1 | 4 | 0% | 20% |
| Art. IV Boot | 2 | 3 | 2 | 29% | 71% |
| Art. V Go Meta | 4 | 2 | 6 | 33% | 50% |
| FC Composite invariants | 5 | 1 | 0 | 83% | 100% |
| **整体 (排除 N/A)** | **28** | **13** | **23** | **44%** | **64%** |

### Top-3 load-bearing 未落地项 (导航锚)

1. 🛑 **G-009 (Art. 0.4 HEAD_t)** — Q_t 版本控制三元组整个空缺;矩阵 §A AMBER (Codex §9.4 2026-05-07 正式 downgrade);runtime grep `git2|libgit2|Command git` = 0 hits。**Unblock**: 架构师 path A/B/C 决策 (semver / git substrate / hybrid)。
2. 🛑 **G-012 (Art. I.1.1 PCP soundness)** — Π_p 稳健性侧零 adversarial benchmark;completeness 已测 (MiniF2F omega solves) 但 soundness 未统计测。**Unblock**: 用户对 corpus 选取方法决策 (miniF2F-v2 misalignment list vs 合成 mutation corpus)。
3. 🛑 **G-016/G-019/G-021/G-028 (Art. III prompt persistence, 4 gap 共枝)** — agent prompt 不上 tape;`agent_audit_trail.jsonl` 是 tx-level 不是 prompt-level。**Unblock**: 架构师决策 prompt persistence 是 Class-3 还是 Class-4 schema 工作。

---

## §1 按章节落地状态

### Art. 0 — 图灵机原教旨

#### Art. 0 preamble
- ⚪ **Art. 0 preamble** (TuringOS 是真的通用机器): N/A — framing axiom

#### Art. 0.1 — 四要素映射
- ✅ **0.1(a) Paper→tape_t**: LANDED — `src/ledger.rs::Tape`; `tests/four_element_mapping.rs`
- ✅ **0.1(b) Pencil→wtool**: LANDED — `src/bus.rs::append_internal`
- ✅ **0.1(c) Rubber→append-only / failed proposal verified=false / Q_{t+1}=Q_t if Π_p=0**: LANDED — `tests/constitution_predicate_gate.rs::predicate_failure_cannot_enter_l4` + `predicate_pass_required_for_l4`; FC1-N15 chain witness (`mathd_numbertheory_1124` 12 rejections)
- ⚠️ **0.1(d) Strict discipline → predicates Π_p + Veto-AI**: PARTIAL — predicate 侧强 (`src/sdk/predicate.rs`); Veto-AI 侧 weak (见 Art. V.1.3 row, G-026)

#### Art. 0.2 — Tape Canonical 公理
- ⚠️ **0.2 axiom (信号必须可从 tape 重建)**: PARTIAL — `tests/constitution_no_parallel_ledger.rs` + `constitution_tape_canonical_gate.rs`;24 历史违规 sub-bullet 未逐项测
- ❌ **0.2 §1 cost/time/provenance 派生测试** [G-001]
  - **原因**: `RunCostAccumulator` / `WalletTool` / `search_cache` / `LibrarianTool` / FC trace 等 9 派生视图缺 `assert_eq!(view, derive_from_tape(tape))` 守恒测试
  - **Unblock**: Pass 3 Wave 3 (offline 派生 library;TB-C0 P08 evidence 充分;~5-7 days)
- ❌ **0.2 §2 parallel-ledger 守恒** [G-002]
  - **原因**: 不存在性已 enforce,`assert_eq` 守恒侧仅 ~2/6 视图覆盖
  - **Unblock**: Pass 3 Wave 3 (与 G-001 共 derivation library)
- ❌ **0.2 §3 PputResult 字段全可重建** [G-003]
  - **原因**: 无 PputResult-字段-vs-tape derivability 测试;9 problems × ~7 fields 待验证
  - **Unblock**: Pass 3 Wave 3 (扩展 matrix §M `chain_derived_facts_not_evaluator_stdout`)
- 🟡 **0.2 §4 Phase D ArchitectAI 成本归因** [G-004]
  - **原因**: forward-bound;Phase D 未上 roadmap exit
  - **Unblock**: 等 Phase D TB charter (probe 现可 ship,full coverage 等 Phase D)
- ✅ **0.2 §5 failure→tape verified=false / reject_class**: LANDED — `tests/tb_18r_attempt_routes_to_l4_or_l4e.rs`;FC1-N15 chain
- ❌ **0.2 §6 WAL per-line SHA-256 hash chain unbroken** [G-005]
  - **原因**: WAL 存在,逐行 hash chain 不可篡改未 chain-resident 测;只在 `audit_tape_tamper` 安全探针里间接验
  - **Unblock**: Pass 3 Wave 3 (~S effort;P07 50 lines 充分)
- ❌ **0.2 24 历史违规 rolling closure** [G-006]
  - **原因**: 闭合分散在 TB-13/14/15/16/17/18R; 无 violation→closure-test 滚动 map
  - **Unblock**: Pass 3 Wave 2 (parser test + 24 violation 先 ID 化)
- ❌ **0.2 10-commit 原子计划行** [G-007]
  - **原因**: 无 row→commit/test 绑定测试
  - **Unblock**: Pass 3 Wave 2 (与 G-006 共 substrate)

#### Art. 0.3 — 区块链化保留
- ❌ **0.3 hash:[u8;32] 语义槽** [G-008]
  - **原因**: Phase E 承诺;`Node` 字段命名 + `bus.append` 签名扩展空间预留未测
  - **Unblock**: Pass 3 Wave 1 (Type-4 静态形状 + 平行 compile-test);**注意**: 若架构师选 Path B,此 slot moot,Pass 3 user input 需先确认
- ⚪ **0.3 Path A vs Path B 决策**: N/A (待 Art. 0.4 path 决策;非 testable)

#### Art. 0.4 — Q_t version-controlled (最 load-bearing)
- 🛑 **0.4 Q_t = ⟨q_t, HEAD_t, tape_t⟩ git-style 版本控制** [G-009] **(LOAD-BEARING)**
  - **原因**: `HEAD_t` 完全未实现 — runtime grep `git2::|libgit2|Command git` = 0 hits;矩阵 §A AMBER (Codex §9.4 2026-05-07 正式 downgrade);`q_state_reconstruct.rs` 仅覆盖 `q_t`;rtool/wtool 三元组签名未 enforce
  - **Unblock**: **架构师决策 path A (libgit2 in-process,~3 周) / path B (子进程 git CLI,6-8 周) / path C (混合)**;Phase E gate 强制 B unless human sudo 修宪;决策落地后 Pass 3 Wave 6 才能开工
- 🛑 **0.4 path A/B/C 决策 landing 测试** [G-010]
  - **原因**: 决策本身未做;扩展 `constitution_art_v3_amendment_log.rs` 的 forcing-function gate (今天 RED-by-design)
  - **Unblock**: 同 G-009;test 可先 wire 即使 RED

#### Art. 0 Laws (基本法)
- ✅ **Law 1 Information is Free** (search/view 零成本): LANDED — `tests/constitution_economy_gate.rs::economy_read_is_free`
- ✅ **Law 2 Investment Costs Money / 1 Coin = 1 YES + 1 NO / on_init unique mint**: LANDED — `economy_complete_set_yes_no_not_coin` + `economy_no_post_init_mint` + `economy_total_coin_conserved` + `fc2_on_init_only_mint`

### Art. I — 信号的量化

#### Art. I preface
- ⚪ **Art. I preface** (量化 = 有损压缩到确定性低维标量): N/A

#### Art. I.1 — Boolean signal predicate
- ✅ **I.1 谓词 = 0 或 1** (binary): LANDED — `tests/constitution_predicate_gate.rs::predicate_result_is_binary`
- ❌ **I.1 hard-vs-soft constraint exclusivity** [G-011]
  - **原因**: 无 test 枚举所有 predicate kind 并断言每种都有 code-side enforcement (非 LLM-judge)
  - **Unblock**: Pass 3 Wave 1 (Type-4 enumeration + grep `reqwest|http|llm_client|chat_completion`;~S effort)
- 🛑 **I.1.1 PCP soundness statistical floor (extreme high prob reject)** [G-012] **(LOAD-BEARING)**
  - **原因**: Completeness 侧覆盖 (MiniF2F omega solves);**soundness 侧需要 adversarial false-proof injection benchmark**,12 gates 都不测;无现成 Lean 公开 false-proof corpus
  - **Unblock**: **用户决策 corpus 方法**: (a) miniF2F-v2 misalignment list (arXiv:2511.03108) — 已存在但需 license 确认; (b) 合成 mutation corpus (mCoq Coq port to Lean — 自建,需用户 approval); decision 后 Pass 3 Wave 5
- ✅ **I.1 Ground Truth as code** (predicate 必须显式存在): LANDED — `src/sdk/predicate.rs` (structural)

#### Art. I.2 — Statistical signal
- ⚠️ **I.2 PPUT / consensus / reputation 主指标 pipe**: PARTIAL — pipe exists; CLAUDE.md Report Standard 强制但 [G-013] 无 executable enforcement
- ❌ **I.2 Report standard executable enforcement (ΣPPUT + Mean PPUT + 95% CI Wilson)** [G-013]
  - **原因**: CLAUDE.md 强制但无 parser test fire 当 TB 报告漏字段
  - **Unblock**: Pass 3 Wave 2 (parser over `handover/tracer_bullets/` + `handover/audits/`;cutoff 2026-05-06 grandfather)
- ⚠️ **I.2 consensus = mode/median (not LLM-judged)** [G-014]
  - **原因**: 现 code 路径未必 compute consensus signal (n=5 是 admission 不是 consensus);若 implemented,无测断言提取函数是 mode/median 而非 LLM
  - **Unblock**: Pass 3 Wave 3 (与 G-001 共 derivation library;若 forward-bound 则降为 gardener-style requirement)
- ✅ **I.2 reputation accumulation (调用计数器)**: LANDED — `src/economy/reputation.rs`; `economic_state_reconstruct.rs`
- ⚠️ **I.2 utility scoring (期望/方差)** [G-015]
  - **原因**: PPUT mean 已计算;σ(PPUT) / variance 未作为强制报告字段
  - **Unblock**: Pass 3 Wave 2 (扩展 G-013 parser regex;~S effort)

### Art. II — 信号的选择性广播

#### Art. II preface
- ⚪ **Art. II preface** (选择性广播): N/A

#### Art. II.1 — Broadcast typical errors
- 🛑 **II.1 NOT raw stderr to all (chain-resident no-leak)** [G-016] **(LOAD-BEARING; prompt persistence 共枝)**
  - **原因**: source-grep `raw_lean_stderr_not_in_agent_read_view` 只 catch 静态路径;无 chain-resident test 在真 n≥2 batch 验证 agent prompt 无 stderr 污染。**关键发现**: TB-C0 `agent_audit_trail.jsonl` 是 tx-level 不是 prompt-level — agent prompts 不上 tape
  - **Unblock**: **架构师决策 prompt persistence 是 Class-3 wire-up 还是 Class-4 schema 工作**;G-016/G-019/G-021/G-028 共此 unblocker
- ❌ **II.1 typical-error → globalized rule pipeline** [G-017]
  - **原因**: CLAUDE.md `feedback_*` 是 documented globalization 机制;无 test 断言每 TB ship 至少 1 个 lesson→mechanism trace per `feedback_norm_needs_mechanism`
  - **Unblock**: Pass 3 Wave 2 (parser TB ship × feedback_*.md 创建史)

#### Art. II.2 — Broadcast price signals
- ✅ **II.2 高权重标价驱动注意力**: LANDED — `tests/tb_14_price_index.rs`; `price_never_overrides_predicate`

#### Art. II.2.1 — Exploration / exploitation
- ⚠️ **II.2.1 exploration / exploitation 平衡** [G-018]
  - **原因**: `axiom_2_payload_diversity` 强制 floor;**无 real-load benchmark fire 两个极端** (over-exploit + over-explore) 验证 system 实际会 warn
  - **Unblock**: Pass 3 Wave 4 (BOLTZMANN_TEMPERATURE 双极 mini-run;6 problem-runs;~L effort + LLM budget)

### Art. III — 信号的选择性屏蔽

#### Art. III preface
- ⚪ **Art. III preface** (选择性屏蔽): N/A

#### Art. III.1 — Shield errors
- 🛑 **III.1 in-context learning of bad pattern** [G-019] **(prompt persistence 共枝)**
  - **原因**: source-grep 不模拟 multi-LLM-call cycle;P07 50 attempts agent_audit_trail 是 tx-level
  - **Unblock**: 同 G-016 (架构师 prompt persistence 决策);decision 后 Pass 3 Wave 4
- 🟡 **III.1 background gardener Agent (GC of stale code/docs)** [G-020]
  - **原因**: NO gardener Agent role 实现;宪法 "必须" 但 src/runtime/ 不存在;test 今天 = stub asserting absent feature
  - **Unblock**: **forward TB charter (TB-Gardener-1)**;FREEZE LIFTED 2026-05-07 后用户授权;Pass 3 不能 ship test

#### Art. III.2 — Encapsulation / progressive disclosure
- 🛑 **III.2 progressive disclosure / agent prompt size budget** [G-021] **(prompt persistence 共枝)**
  - **原因**: capsule-shielding 测 audit-only routing;无 test 断言 agent prompt size 守 token budget on real run。**取决于** prompt persistence
  - **Unblock**: 同 G-016

#### Art. III.3 — Shield correlation
- ⚠️ **III.3 horizontal context-isolation on real n≥2** [G-022]
  - **原因**: `pairwise_payload_diversity_mean` floor 已查;无 test 断言 agents 不共享 live context (only state-via-tape)
  - **Unblock**: Pass 3 Wave 3 (TB-C0 n=5 substrate; source-grep `static mut` + `Arc<Mutex>` for state ledgers + offline diversity floor)

#### Art. III.4 — Shield Goodhart
- ⚠️ **III.4 scoring-formula leakage (stronger than current pollution test)** [G-023]
  - **原因**: pollution-floor 不 catch coefficient leakage (e.g. PPUT formula constants 进 prompt)
  - **Unblock**: Pass 3 Wave 1 (grep PPUT 系数 vs prompt builder;~S effort;与 G-016 可合并)

### Art. IV — Boot

#### Art. IV preface
- ⚪ **Art. IV preface** (Boot = 元程序拉起 + 谓词写入信任根): N/A

#### Art. IV — Boot 主体
- ✅ **IV Boot — InitAI compiles spec → predicates**: LANDED — 8 dedicated tests (`fc2_genesis_report_exists` + `fc2_on_init_only_mint` + `fc2_no_post_init_mint` + `fc2_taskopen_escrowlock_are_chain_events` + `fc2_no_memory_only_preseed` + `fc2_run_replayable_from_genesis_tape_cas` + `fc2_system_pubkeys_verify` + `fc2_agent_registry_resolves`);chain witness on every MiniF2F run
- ⚠️ **IV Q_t 三元组** (mermaid lines 540-610): PARTIAL — subsumed under Art. 0.4 (G-009)
- ⚠️ **IV rtool/wtool 三元组 signatures**: PARTIAL — code 用不同 signature;subsumed under Art. 0.4 (G-009)
- ✅ **IV HALT terminal anchor**: LANDED — `src/ledger.rs::HaltReason`; matrix §E GREEN
- ⚠️ **IV clock + map-reduce tick** [G-025]
  - **原因**: 取决于 run length;FC2-N20 AMBER for runs < TICK_INTERVAL;P07/P08 (tx_count=50) 可能已观察到但需确认 mr-tick event 签名
  - **Unblock**: Pass 3 Wave 4 (检查 `src/bus.rs::clock` 锁定 marker;P07/P08 evidence 大概率充分;若不够 PutnamBench mini-run)
- ❌ **IV initialization is one-shot** [G-024]
  - **原因**: `fc2_on_init_only_mint` proxies;无 test 断言 NO post-boot mutation 路径 for predicates / tools / Q_0 schema (broader claim)
  - **Unblock**: Pass 3 Wave 1 (静态 enumeration over PredicateRegistry / ToolRegistry / Q_0 / AgentRegistry;~S effort)

### Art. V — Go Meta

#### Art. V preface
- ⚪ **Art. V preface** (元架构 = 架构的架构): N/A

#### Art. V.1 — 三权分立
- ✅ **V.1 机制/突变/选择 三角**: LANDED — structural (`feedback_dual_audit.md`, directives, audits)

#### Art. V.1.1 — sudo 范围 + 三段守护
- ✅ **V.1.1 sudo 仅作用于 constitution.md** (2026-04-25 amendment): LANDED — `fc3_constitution_hash_pinned` + `constitution_hash_matches_trust_root_manifest`
- ❌ **V.1.1 三段守护 (sudo + Veto-AI + Boot manifest) — Veto-AI middle layer executable check** [G-026]
  - **原因**: layer 1 (sudo) 和 layer 3 (Boot manifest) 已测;layer 2 (Veto-AI role-narrowness on real audit decision) 仅 structural-only marker
  - **Unblock**: Pass 3 Wave 2 (parser over `handover/audits/CODEX_*.md` + `GEMINI_*.md` + `DUAL_AUDIT_*VERDICT*`;每 verdict 验证 output domain ⊂ {PASS, CHALLENGE, VETO})

#### Art. V.1.2 — ArchitectAI commit authority
- ❌ **V.1.2 ArchitectAI commit authority — executable enforcement** [G-027]
  - **原因**: 仅 structural-only `fc3_architectai_proposal_not_direct_write` (AMBER);无 executable check 验证 architecture commit 由 Veto-AI PASS 先行
  - **Unblock**: Pass 3 Wave 2 (git2-rs + audit-file index;每 architecture commit 验证 ≥1 PASS/non-VETO Codex + Gemini verdict 先行;~L effort)

#### Art. V.1.3 — Veto-AI 范围
- 🛑 **V.1.3 Veto-AI output domain {PASS, VETO, CHALLENGE} (no quality/perf judgments)** [G-028] **(prompt persistence 共枝 - 部分)**
  - **原因**: structural-only `fc3_judgeai_veto_only`;无 test 解析真 Codex/Gemini audit reports 验证不含 quality/perf-only verdicts (无 clause citation)
  - **Unblock**: 与 G-026 共 scaffolding;额外断言 non-PASS verdict 必引宪法 clause;30-line proximity heuristic 待用户确认 (semantic § boundary or 数值)
- ❌ **V.1.3 JudgeAI → Veto-AI rename — residual symbol grep** [G-029]
  - **原因**: 无 grep test 断言 src/ 无 `JudgeAI`;**实测 1 hit** in `src/runtime/autopsy_capsule.rs` doc-comment
  - **Unblock**: Pass 3 Wave 1 (grep test + 1-line edit;同原子 ship;~S effort;forcing-function gate)

#### Art. V.2 — 宪法界限
- ⚠️ **V.2 宪法界限示例** (compute cap, time cap, deterministic predicates, reversibility): PARTIAL — compute/time caps 已测 (`max_tx`, `WallClockCap`); deterministic predicates 已测 (`predicate_result_is_binary`); **reversibility 未测**
- ❌ **V.2 reversibility constraint (Q_{t-1} rollback)** [G-030]
  - **原因**: 无 test 断言 every state mutation 有 inverse 路径;replay-from-genesis 从 Q_0 到 Q_t 但不 step-by-step 验证 reversibility
  - **Unblock**: Pass 3 Wave 3 (枚举 14 个 TxKind 变体;每个声明 inverse kind 或 `IRREVERSIBLE_BY_CONSTITUTION` 注释;`TerminalSummaryTx` 等 canonical irreversible 待用户确认列表)

#### Art. V.3 — 宪法修订日志
- ✅ **V.3 修订留痕 (date/trigger/section/summary)**: LANDED — `tests/constitution_art_v3_amendment_log.rs` 6 tests (round-8 landed)

#### Closing
- ⚪ **Closing 老子引言** (损之又损以至于无为): N/A — quotation
- ⚪ **Art. VI Reference** ([1][2][3]): N/A — bibliography

### FC Composite Invariants (跨章节)

- ✅ **FC1-INV1** every externalized attempt tape-visible: LANDED — `fc1_every_externalized_attempt_is_tape_visible`;chain witness 9/9 GREEN post-fix
- ✅ **FC1-INV3** 3-term count equality (constitutional invariant): LANDED — `fc1_attempt_count_equals_tape_count` + `tb_18r_chain_attempt_invariant.rs`;9/9 GREEN
- ✅ **FC1-INV6** no fake nodes (CAS bytes match CIDs): LANDED — composite chain + `audit_tape_tamper`
- ⚠️ **FC2-INV5** replay from genesis + tape + CAS: PARTIAL — `fc2_run_replayable_from_genesis_tape_cas` AMBER;standalone smoke 待 MVP-4
- ✅ **FC3-INV1** capsule integrity regen-from-L4+CAS: LANDED — `constitution_fc3_inv1_capsule_integrity_regen.rs` (round-8) validates P05/P07/P08
- ✅ **FC3-INV2** no global Markov pointer: LANDED — `tests/constitution_no_parallel_ledger.rs::no_global_markov_pointer`

---

## §2 全部 30 gap 解锁路径汇总

| Gap | 状态 | 章节 | 阻塞类型 | 解锁动作 | Pass 3 Wave |
|-----|------|------|----------|----------|-------------|
| G-001 | ❌ | 0.2§1 | 无 | offline derivation library (TB-C0 P08 substrate) | 3 |
| G-002 | ❌ | 0.2§2 | 无 | 共 G-001 derivation library | 3 |
| G-003 | ❌ | 0.2§3 | 无 (CLAUDE.md tx_count 注意点) | 扩展 chain_derived_facts_not_evaluator_stdout | 3 |
| G-004 | 🟡 | 0.2§4 | **forward Phase D charter** | probe 现可 ship; full coverage 等 Phase D | 6 (probe in 3) |
| G-005 | ❌ | 0.2§6 | 无 | offline parser + sha256 walk (P07 50 lines) | 3 |
| G-006 | ❌ | 0.2 24-violation | 无 (24 violation 先 ID 化) | parser TB_LOG.tsv × OBS_*.md | 2 |
| G-007 | ❌ | 0.2 10-commit | 无 (同 G-006 ID 化) | parser 10-row × git log | 2 |
| G-008 | ❌ | 0.3 hash slot | minor (Path B 选则 moot) | static-shape + parallel compile-test | 1 |
| G-009 | 🛑 | 0.4 HEAD_t | **架构师 path A/B/C 决策** | 写 decision framing 给架构师 | 6 (after) |
| G-010 | 🛑 | 0.4 path landing | **同 G-009** | 扩展 art_v3_amendment_log + RED-by-design wire | 6 (test 现在可 wire) |
| G-011 | ❌ | I.1 hard-vs-soft | 无 | predicate enum 枚举 + grep llm_client | 1 |
| G-012 | 🛑 | I.1.1 PCP soundness | **用户 corpus 方法决策** | miniF2F-v2 misalignment vs 合成 mutation;license 确认 | 5 (after) |
| G-013 | ❌ | I.2 Report standard | minor (cutoff date) | parser handover/tracer_bullets + audits | 2 |
| G-014 | ⚠️→❌ | I.2 consensus mode/median | minor (是否 implemented 待查) | 共 G-001 derivation;若 forward 标 gardener-style | 3 (or defer) |
| G-015 | ⚠️→❌ | I.2 utility variance | minor (σ 是否强制) | 扩展 G-013 regex | 2 |
| G-016 | 🛑 | II.1 chain-resident no-leak | **架构师 Class-3 vs Class-4 prompt persistence 决策** | 共 G-019/G-021/G-028 unblocker | 3 remainder + 4 |
| G-017 | ❌ | II.1 globalized rule pipeline | minor (lesson section schema) | parser TB ship × feedback_*.md | 2 |
| G-018 | ⚠️→❌ | II.2.1 explore/exploit | minor (LLM budget approval) | BOLTZMANN_TEMPERATURE 双极 mini-run | 4 |
| G-019 | 🛑 | III.1 in-context contamination | **同 G-016** | byte-substring scan once prompts persist | 4 (after) |
| G-020 | 🟡 | III.1 gardener Agent | **forward TB-Gardener-1 charter** | 用户授权 TB-Gardener-1 (FREEZE 已解) | 6 |
| G-021 | 🛑 | III.2 prompt size budget | **同 G-016** | tiktoken count 验证 once prompts persist | 4 (after) |
| G-022 | ⚠️→❌ | III.3 horizontal isolation | minor (0.25 floor 确认) | source-grep + offline diversity (TB-C0 n=5) | 3 |
| G-023 | ⚠️→❌ | III.4 scoring leakage | 无 (与 G-016 可合并) | grep PPUT 系数 vs prompt builder | 1 |
| G-024 | ❌ | IV one-shot init | minor (registries 列表) | 静态 enumeration mutation surfaces | 1 |
| G-025 | ⚠️→❌ | IV mr-tick | minor (mr-tick marker 签名) | 检查 src/bus.rs::clock + P07/P08 verdict | 4 |
| G-026 | ❌ | V.1.1 三段守护 (Veto-AI) | minor (grandfather cutoff) | parser CODEX_*.md + GEMINI_*.md | 2 |
| G-027 | ❌ | V.1.2 commit authority | minor (architecture commit 定义) | git2-rs + audit camp cross-ref | 2 |
| G-028 | 🛑 | V.1.3 output domain | **同 G-016 (部分,verdict 解析侧无依赖)** | 共 G-026 scaffolding + clause citation | 2 (verdict side) + 4 (rest) |
| G-029 | ❌ | V.1.3 JudgeAI rename | 无 | grep test + 1-line autopsy_capsule.rs edit | 1 |
| G-030 | ❌ | V.2 reversibility | minor (irreversible-by-design 列表) | 14 TxKind enumeration + inverse registry | 3 |

**计数核对**: 30 gap 全覆盖 (G-001..G-030)。状态分布: 🛑 7 (G-009/G-010/G-012/G-016/G-019/G-021/G-028) + 🟡 2 (G-004/G-020) + ❌ 21 (其余)。

---

## §3 解锁后能动多少 (impact 估算)

### 三个 strategic decision 解锁多少

- **架构师定 G-009 path A/B/C** → 解锁 G-009 + G-010 (2 gap;但 implementation 是 3-8 周后)
- **用户定 G-012 corpus 方法** → 解锁 G-012 (1 gap;Wave 5;5-10 days after approval)
- **架构师定 G-016 Class 边界 (prompt persistence)** → 解锁 G-016, G-019, G-021, G-028 (4 gap 共枝;Wave 3 remainder + Wave 4)
- **用户授权 TB-Gardener-1 charter** → 解锁 G-020 (1 gap;forward-bound 实现)
- **Phase D charter 上 roadmap** → 解锁 G-004 full coverage (probe 现可)

### 无依赖,Pass 3 立即可启动

**Wave 1 (静态 shape, ~1 day)**: G-008 + G-024 + G-029 + G-011 + G-023 (5 gap)
**Wave 2 (parser docs, ~3-5 days)**: G-026 + G-028 verdict-parsing 部分 + G-013 + G-015 + G-006 + G-007 + G-017 + G-027 (8 gap)
**Wave 3 partial (offline derivation, ~5-7 days)**: G-005 + G-001 + G-002 + G-003 + G-022 + G-030 + G-014 (7 gap)

合并: **20 gap 路径明确,立即可启动**;另 4 gap (G-018 LLM budget / G-025 mr-tick / G-009 path / G-012 corpus 等) 需 minor user input;只有 G-009/G-010 (path) + G-016/19/21/28 部分 + G-012 + G-020 等 ~9 gap 真正 BLOCKED 待 strategic decision。

---

## §4 真实落地里程碑

```
M1 (now)        — 30 gap 中 20 个 path 明确,Pass 3 立即可启动 (Wave 1+2+部分3)
                  当前落地率 (LANDED 单独): 44% (28/64 testable)
                  当前 LANDED+PARTIAL: 64% (41/64)

M2 (~2 weeks)   — Wave 1+2+Wave 3 partial 闭合 → ~17-20 gap 关掉
                  落地率升至 ~71-76% (LANDED 单独)
                  关键: 12 gates → estimated ~24 gates

M3 (~3 weeks)   — 三个 strategic decision 落地 (path A/B/C + corpus 方法 + Class-3/4 prompt persistence)
                  Wave 3 remainder + Wave 4 启动
                  G-016/19/21/28 解锁

M4 (~5 weeks)   — Wave 3+4 闭合 + G-012 corpus 决策实施 → Wave 5 启动
                  落地率 ~85% (LANDED 单独)

M5 (~6-8 weeks) — Wave 5 闭合 (PCP soundness 真测) + G-009 实现完成 (path A=3wk / path B=6-8wk)
                  Wave 6 启动 (HEAD_t implementation tests)

M6 (~7-10 weeks)— 全部 30 gap 闭合 (除 G-004 full Phase D + G-020 gardener 的 forward-bound 余项) + v3 evidence → round-3 dispatch → §8 → TB-18R FINAL ship → 宪法真正落地 ✓
                  落地率: 95%+ (剩 forward-bound 是 Phase D + gardener,非 Pass 3 scope)
```

---

## §5 Cross-references

- Pass 1: `/home/zephryj/projects/turingosv4/handover/audits/CONSTITUTION_COVERAGE_GAP_AUDIT_PASS_1_2026-05-07.md`
- Pass 2: `/home/zephryj/projects/turingosv4/handover/audits/CONSTITUTION_COVERAGE_GAP_AUDIT_PASS_2_2026-05-07.md`
- Constitution: `/home/zephryj/projects/turingosv4/constitution.md` (886 lines)
- Execution matrix: `/home/zephryj/projects/turingosv4/handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md`
- FC-witness catalog: `/home/zephryj/projects/turingosv4/handover/alignment/FC_WITNESS_CATALOG_2026-05-06.md`
- 用户 directive 2026-05-07: `~/.claude/projects/-home-zephryj-projects-turingosv4/memory/feedback_real_problems_not_designed.md` (2026-05-07 strengthening)
- TB-C0 multi-agent batch evidence (Type-2 substrate): `/home/zephryj/projects/turingosv4/handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/`
- TB-18R Phase 3 v3 evidence (secondary substrate): `/home/zephryj/projects/turingosv4/handover/evidence/tb_18r_phase_3_2026-05-07T08-33-05Z/`
- 24-violation source (G-006): `/home/zephryj/projects/turingosv4/handover/architect-insights/TAPE_CANONICAL_AUDIT_2026-04-26_AUDITOR.md`
- TB ship log: `/home/zephryj/projects/turingosv4/handover/tracer_bullets/TB_LOG.tsv`
- LATEST.md (active state): `/home/zephryj/projects/turingosv4/handover/ai-direct/LATEST.md`
