# Harness 缺陷审计 + 加固提案 v1.2 — 2026-05-24

**触发**: 用户报告 7 项宪法/FC 违宪聚集在 visualization/diagram 派生工件中；要求 harness 侧根因诊断，**非代码修复**。

**任务分类**: 调研 + 提案文档（Risk Class 0）。根因为 Class-4 系统性结构缺陷。

**Orchestrate trail** (per `skills/orchestrate/SKILL.md`):
- Phase 0 triage → DELIBERATIVE; Verdict 域 `{READY-AS-PROPOSAL | NEEDS-MORE-DEPTH | STRUCTURAL-GAP-MISSING | OVER-INSTRUMENTATION-RISK}`
- Phase 1 — R1 catalog (167 mechanisms / 8 categories) + R2 forensics (verdict: SUPPORTED) + R3 gap taxonomy (dominant cause: δ)
- Phase 2 — contract lock
- Phase 3 — synthesis (v1)
- Phase 5 — 3 Adversarial-Critic (C1 Constitution + C2 Karpathy + C3 Practical-Deployment) ✅ 完成
- Phase 6 — triage: 4 项 MUST-FIX 全部吸收；8 项 → 6 项 ship-ready (v1.2)
- Phase 7 — pending: Shipping-Witness verdict
- Output: this file → architect review → individual H0/H1/H2/H3/H4/H5/H8 走各自 charter 流程

---

## §0 摘要 (TL;DR — v1.2 post-triage)

- **诚实诊断**: 用户假设 (「这些违宪是 harness 架构长期累积缺陷的产物，不是个别人错误」) **SUPPORTED**（R2 取证）。
- **主导根因**: δ **DERIVATIVE-ARTIFACT-BLIND-SPOT** —— harness 全部 133 个 `tests/constitution_*.rs` gate 都在 `src/` + tape + CAS 上守护，**没有任何 gate 检验 derivative-artifact（图、可视化、dashboard）的宪法/FC 完整性**；CI path filter (`constitution_gates.yml`) 显式排除 handover/research/, docs/, `*.html`, `*.svg`, `*.mmd`。
- **次要根因**: γ instruction-vs-mechanism gap（57 个 feedback_*.md 中 ~45 个 instruction-only）+ Cat 8 cross-PR cumulative drift = 0 mechanism + **OBS-archive escalation 缺口**（第 10 个 gap，由验证 R2 cite 意外发现：55 个 `OBS_*.md` 累积，OBS_CONSTITUTION_MERMAID_FENCE_2026-04-22 32 天未修）。
- **不能成立的归因**: 不是个别人疏忽；不是 FC3 测试缺失（`constitution_fc3_meta` 等存在并在 manifest）；不是审计制度缺失（§14 cadence 表完整）。问题在 cadence 把派生工件归 Class 0 (no audit)，而 §1 又警告派生工件可能变成 alternative source of truth。
- **核心提案 (v1.2)**: **6 项 harness 加固 ship-ready (H0/H1/H2/H3/H4/H5/H8)**，从 v1 的 8 项压缩 — Phase 5 三家 Critic 一致 MUST-FIX 了 v1 的 H6 (premature skill 泛化) 与 H7 (premature cron daemon)，已 DROP (见 §7 critic log)。
- **顺序**:
  - **H0 独立 ship** (Class 4 sudo per Art. V.1.1 — constitution.md 修订)
  - **Phase A 单天** (H1 + H2 + H8) — 3 个 Class 0/1
  - **Phase B 单 charter** (H3 — Class 2 cadence exception, **不是新 class 0.5**)
  - **Phase C 单 charter** (H4 + H5) — 2 个 Class 1 test gate
- **总成本**: 2 charter + 1 Class 4 hotfix。**Class 4 work = 1 (H0)**；其余 6 项全 Class 0/1。

---

## §1 诚实诊断

### 1.1 用户假设取证结论 (R2)

用户假设 **SUPPORTED**。证据：

1. **Failure mode 分布 (n=7)**: GATE-MISSING=1 (①) / GATE-INSTRUCTION-ONLY=2 (②④) / GATE-SCOPE-MISMATCH=3 (③⑤⑥) / GATE-MISCLASSIFIED-RISK=1 (★)。GATE-RAN-BUT-PASSED 和 AUDIT-DEFERRED 都是 0 —— **gates 如果跑在正确工件上就抓得到**，问题是没跑在那里。
2. **6/7 违宪居住在派生 visualization 工件**（freshly generated, untracked），被 CI path filter 完全排除（`constitution_gates.yml` 只触发 `src/**/*.rs`, `tests/**/*.rs`, `constitution.md`, 矩阵 .md）。
3. **4/7 违宪（②④⑤⑥）有对应 memory norm 但 instruction-only**（`feedback_chaintape_externalized_proposal` / `_rejection_evidence_separate` / `_tape_first_real_tests` / `_class4_cannot_hide_in_class3`）。
4. **★ = dual git substrate**: PR #110→#111→#112→#113 在 ~36 小时累积部署，每个自报 Class 2 (additive skeleton)，累计是 Class-4 候选第二 git substrate。**Cat 8 cumulative drift detection = NO MECHANISM**。

### 1.2 主导根因 (R3)

R3 在 5 个候选根因中选 **δ DERIVATIVE-ARTIFACT-BLIND-SPOT**（原文）：「the entire gate machinery defends the wrong axis: code/tape/CAS rather than the visualization artifacts where 6/7 of the actual violations occurred」。

反驳竞争: α (gates 缺) 反驳 (133 gate 存在); β (误分类) 只解释 1/7; γ (norm 未机械化) 解释 5/7 但是 δ 的下游; ε (FC3 invisible) 反驳 (`constitution_fc3_meta` 存在)。

### 1.3 必须诚实承认

- **harness 已做对**: 133 substrate gate + `/harness-reflect` + `/constitution-landing-check` + `/runner-preflight` 这套 advisory skill。
- **harness 必须修复**: 派生 visualization/diagram/dashboard/handover-alignment 工件类的「宪法可证伪性」机制。
- **harness 不应做**: 不要扩大 ceremony；不增 Manager/Engine/Framework；不要每个新文档都跑 Codex 审计（per 2026-05-24 architect ratification: Gemini 审计员 dropped, Codex 单 audit 是唯一 default — 见 AGENTS.md §9 / §14 + [feedback_dual_audit](file:///home/zephryj/.claude/projects/-home-zephryj-projects-turingosv4/memory/feedback_dual_audit.md)）。

---

## §2 7 项违宪的 harness 侧根因 (R2 mapping)

| # | 违宪 | failure mode | 应触发未触发的 gate | 根因 |
|---|---|---|---|---|
| ① | FC3 meta 层缺席 | GATE-MISSING | `constitution_fc3_meta` 守 ChainTape capsule，不守 diagram completeness | 派生工件类无完整性 gate |
| ② | Q_t 三元组压扁 | GATE-INSTRUCTION-ONLY | `feedback_chaintape_externalized_proposal` (instruction-only) | norm 未机械化 + 派生盲区 |
| ③ | 谓词被 Judge 收编 | GATE-SCOPE-MISMATCH | `constitution_predicate_gate` 守 runtime binary，不守 diagram iconography | gate 范围止于代码边界 |
| ④ | L4/L4.E 画成两账本 | GATE-INSTRUCTION-ONLY | `constitution_no_parallel_ledger` 守 runtime 不守 diagram 单 ChainTape 双 channel 渲染 + `feedback_rejection_evidence_separate` | norm 被误译（"分开"读为"两账本"）|
| ⑤ | TuringBus 缺席 | GATE-SCOPE-MISMATCH | §6 守 `src/bus.rs` 不被改，不守 diagram 不能省略 | 守"不能编辑"非"必须出现" |
| ⑥ | Art. 0.2 守恒回路未画 | GATE-SCOPE-MISMATCH | `dashboard_regen` + `chain_derived_facts` (runtime 守恒); audit 域有 `RECONSTRUCTION-FAILURE` 但是 reactive | 范围 + verdict 域均限于 runtime |
| ★ | 双 git substrate 未标 Class-4 | GATE-MISCLASSIFIED-RISK | `feedback_class4_cannot_hide_in_class3` (instruction-only) + Cat 8 = 0 | 单 PR 看是 Class 2，累计是 Class 4 |

### 2.1 单点观察 — FC2/FC3 mermaid fence 32 天未修 (VERIFIED)

`handover/alignment/OBS_CONSTITUTION_MERMAID_FENCE_2026-04-22.md` 确实存在，记录：L325 FC-1 opener ✓ / L379 FC-1 closer ✓ / L530 FC-2 closer (opener MISSING at L441 `flowchart TD`) / L714 FC-3 closer (opener MISSING at L670 `graph TB`)。

OBS 已带修复 recipe（L441 与 L670 之前各加 ` ```mermaid `），但 OBS 自身判 「FILED FOR HUMAN ARCHITECT (Claude does NOT modify constitution.md)」 —— **32 天无任何 mechanism 触发架构师 follow-up**。三层级联失效：检测 OK → 归档 OK → 后续 FAIL。这是 R3 9 gap 之外的第 10 个 gap = **OBS-archive escalation**，触发 H8。

---

## §3 Harness 结构性缺口分类 (R3)

R3 调查 10 个候选 gap → 9 confirmed + 1 non-gap：

| 编号 | 名称 | 宪法锚 | 覆盖 (of 7) | cost |
|---|---|---|---|---|
| (a) | Artifact-to-Spec Completeness | Art. 0.2 #2 | 6/7 | M |
| (b) | FC3 Meta Enforcement on Derivative | Art. V.1.3 | 1.5/7 | S-M |
| (d) | Class-4 Detection Sensitivity | §5+§6+Art. 0.4 | 1/7 (★) + 未来 | M |
| (e) | Cross-PR Cumulative Drift | Art. 0.2 #2 + §1 | 1/7 + 潜在 | L |
| (f) | Instruction-vs-Mechanism Closure (meta) | Art. I.1 L207-209 | 5/7 | S |
| (g) | Verdict Domain Completeness | §14 | 3/7 | S |
| (h) | Derivative-Artifact Classification | §1+§5 | 6/7 | M |
| (i) | Cold-Context Audit Coverage | §9+§14 | 6/7 (与 h) | M |
| (j) | Skill-Activation Faithfulness | §5+Art. I.1 | 7/7 | S |

**Non-gap (c)** Conservation-Predicate — `constitution_economy_*` / `walkthrough_inv3_conservation` / `constitution_head_t_witness` 已覆盖 substrate 守恒。

R3 Top-3 leverage: (j) 7/7 cost S; (a) 6/7 cost M; (f) 5/7 cost S。

---

## §4 Harness 加固提案 v1.2 (post Phase 5/6 triage)

每个 ship-ready 提案 = 1 charter 候选。本节不设计代码，只锚定 (gap, 宪法, 覆盖违宪, class, cost, 依赖, Karpathy 边界)。

### H0 — constitution.md FC2/FC3 mermaid fence 修复 (Class 4 sudo per Art. V.1.1)

| 维度 | 内容 |
|---|---|
| 触发 gap | 暴露 Cat 8 失效（fence 破损 32 天无 gate；OBS 归档 32 天未升级）|
| 内容 | 按 OBS recipe: (1) L441 `flowchart TD` 前加 ` ```mermaid ` opener; (2) L670 `graph TB` 前加 opener; (3) verify L826-870 amendment FC3 graph 的 fence (OBS 未 cover); (4) **Art. V.3 修订日志追加一行** (per L806 "每次修订必须留痕"); (5) `grep -c '```mermaid' constitution.md` ≥3 |
| 宪法锚 | **Art. V.1.1**（sudo 权限仅作用于 constitution.md，人类架构师 only）+ Art. V.3 + Art. 0.2 #2 |
| 覆盖违宪 | ① 直接 |
| Class | **4** (constitution.md mutation 是 Art. V.1.1 sudo event, 不是 Class 1 — per C1 Phase 5 finding) |
| Cadence | Per AGENTS.md §14 Class 4 (updated 2026-05-24 single-Codex doctrine): TB charter + per-atom §8 + **clean-context Codex audit PRE-§8 (single witness; Gemini dropped per §9)**。**架构师 ratification 2026-05-24**: 选 **Cadence A — 完整 PRE-§8 Codex witness**（B precedent [Art. 0.4 sudo + §8 即 ship, witness defer] 未被选用） |
| Cost | XS execution; M cadence ceremony |
| 依赖 | 无 |
| Karpathy 边界 | 修复本身 3 行字符 + 1 行 V.3 log — surgical。Cadence 是宪法刚性要求，非 ceremony |

### H1 — Verdict 域 +1 token `STRUCTURAL-INCOMPLETENESS` (含 downstream 同步)

| 维度 | 内容 |
|---|---|
| 触发 gap | (g) Verdict Domain Completeness |
| 内容 | AGENTS.md §14 verdict 域加新 token `STRUCTURAL-INCOMPLETENESS <fc-element-omitted> <artifact-path>`，专门用于 derivative artifact 省略 FC 元素的情况。**同 PR 同步更新** downstream pointers (per C1 NICE-TO-FIX + C3 NICE-TO-FIX): `CLAUDE.md` §6 verdict 域引用、`skills/orchestrate/SKILL.md` 4-token 引用、`skills/orchestrate/auditors.md`、任何 `skills/codex/*` 含 4-token 模板。避免 truth-order pointer drift |
| 宪法锚 | Art. V.1.3 white-list extension + Art. 0.2 #2 + AGENTS.md §15 (concise instructions) |
| 覆盖违宪 | ①③⑤ 直接；②④⑥ 间接 |
| Class | 0 |
| Cost | S |
| 依赖 | 无 |
| Karpathy 边界 | 不增 trait 不增 type；纯 verdict 域扩展 + 单次 grep + 同步 |

### H2 — `feedback_*.md` `mechanism:` lint (escape hatch tightened per C3)

| 维度 | 内容 |
|---|---|
| 触发 gap | (f) Instruction-vs-Mechanism Closure (recursive) |
| 内容 | 新 `tests/constitution_feedback_mechanism_pairing.rs`，frontmatter 必须含 `mechanism:` 字段，值取自封闭枚举: `{<file:path>, none-with-charter <charter-ref>, grandfathered}`。当前 57 个 feedback_*.md 中 ~45 个 instruction-only 标 `grandfathered` 入 baseline (45-entry allowlist; baseline 用 K-2.3 manifest drift `BASELINE_ALLOWLIST` 同 pattern; 0 future growth in `grandfathered` set without explicit `mechanism: none-with-charter` declaration)。Meta-norm `feedback_norm_needs_mechanism.md` 必须标 `mechanism: tests/constitution_feedback_mechanism_pairing.rs` (自指闭合)。**Per C3 tightening**: `none-with-charter` 值必须 cite existing charter file path; **不允许** 自由文本 `none (rationale: ...)` —— 否则 solo user 在压力下会 reflexively `none`，defeat 整个 gate。This aligns with `feedback_no_workarounds_strict_constitution.md` ("我不要凑活") |
| 宪法锚 | Art. I.1 L207-209 + `feedback_no_workarounds_strict_constitution.md` |
| 覆盖违宪 | ②④⑥ 直接；①③⑤ 间接 |
| Class | 1 |
| Cost | S |
| 依赖 | 无 |
| Karpathy 边界 | reuse `constitution_matrix_drift.rs` baseline-allowlist pattern (per C2 verified)；不引新 schema |

### H3 — Class 2 cadence exception for derivative artifacts (REVISED per C1+C2+C3)

| 维度 | 内容 |
|---|---|
| 触发 gap | (h) + (i) |
| 内容 (REVISED, **no new class number**) | AGENTS.md §5 + §14 修订：在现有 Class 2 行 ("production wire-up, ... dashboards, ... non-authoritative views") 增加 sub-bullet "**derivative artifact sub-cluster**: `handover/alignment/*` (FC 矩阵)、`constitution.md` mermaid 块、`handover/architect-insights/*.md` architecture diagrams、`docs/architecture.md` → cadence = clean-context Codex witness only; charter NOT required; §8 NOT required; matrix update yes"。**不引入 Class 0.5 / Class D 新 class number** (Phase 5 三家一致 MUST-FIX：fractional class 破坏 §5 整数 schema + vague future extensibility 反模式 + solo user 维护负担)。Cadence intent 与 v1 H3 一致，但用 Class 2 exception 表达，保持 §5 schema 完整。受影响 artifact 在 frontmatter 标 `class: 2-derivative` + `depicts: [FC1,FC2,FC3]` 子集 (hygiene pass ~80 文件) |
| 宪法锚 | AGENTS.md §1 + §5 (Class 2 定义) + Art. 0.2 #2 |
| 覆盖违宪 | ①②③④⑤⑥ 全部 (cadence 杠杆 — 一旦工件进 Codex 审计，6 项 diagram 违宪都进入审计员视线) |
| Class | 0 (AGENTS.md 文本修订) |
| Cost | S-M (AGENTS.md 段落 + ~80 文件 frontmatter hygiene) |
| 依赖 | 与 H4 共生 (H4 提供检查机制；H3 强制审计) |
| Karpathy 边界 | **零新 class 数**；reuse §5 整数 schema; 用 sub-bullet 表达 cadence variant |

### H4 — Artifact-to-Spec Completeness 测试 (REVISED: inline const + WIP exemption per C2+C3)

| 维度 | 内容 |
|---|---|
| 触发 gap | (a) + (b) |
| 内容 (REVISED) | 新 `tests/constitution_derivative_artifact_completeness.rs`，扫: `constitution.md` mermaid 块 / `handover/alignment/TRACE_FLOWCHART_MATRIX.md` / `docs/architecture.md` / `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md` / in-repo `*.mmd`。每个 artifact 在 frontmatter declare `depicts: [FC1,FC2,FC3]` 子集；测试 parse FC 矩阵 declared element set vs artifact mermaid node set → missing → fail。**Karpathy revision per C2**: per-artifact "depicts" 白名单用 **inline Rust `const &[(path, &[FC])]` 数组** (4-10 entries, single consumer)，**不引入 YAML 子物** (mirror `BASELINE_ALLOWLIST` in `constitution_matrix_drift.rs` / `READ_ONLY_FIXTURE_TESTS` in `constitution_no_evidence_drift_in_tests.rs`)。**Deployment revision per C3**: 加 WIP exemption path `handover/scratch/*` 排除，让 iterative drafting 不与 gate 冲突，避免 `--no-verify` reflex |
| 宪法锚 | Art. 0.2 #2 + FC1/2/3 canonical hash (TRACE_FLOWCHART_MATRIX.md) |
| 覆盖违宪 | ①②③⑤⑥ 直接；④ 经语义 (required edge "L4_L4E_single_chaintape") 间接 |
| Class | 1 |
| Cost | M |
| 依赖 | H0 (FC3 fence 先修, 才能 parse FC3 block); H3 (Class 2-derivative cadence 是 H4 fail 的接收 channel) |
| Karpathy 边界 | **inline const, no YAML**; reuse `constitution_no_evidence_drift_in_tests` pattern; reuse R-022 trace-matrix parser if applicable |

### H5 — Class-4 Trust-Root Surface Auto-Detection (REVISED: inline + tighter loophole per C1+C2)

| 维度 | 内容 |
|---|---|
| 触发 gap | (d) |
| 内容 (REVISED) | 新 `tests/constitution_trust_root_surface_classification.rs`，结构性 grep: (1) git status 新文件 match `src/**/{*ledger*,*sequencer*,*signing*,*substrate*,*tape*,*authority*,*trust*}.rs`; (2) 新 typed_tx 变体; (3) 新 RootBox 字段; (4) 新 git substrate (`git2::Repository::init` in `src/**/*ledger*.rs`)。**Karpathy revision per C2**: pattern list 用 **inline Rust `const &[&str]`** (mirror `LEGACY_GLOBAL_MARKOV_POINTER` in `constitution_no_parallel_ledger.rs`), **不引入** `scripts/trust_root_patterns.yaml` 派生物。**Loophole tightening per C1**: 命中后允许的 PR 描述只有: (a) `risk-class: 4` 显式 OR (b) `risk-class: <N> + 引用现存 per-atom §8 directive file path`。**不允许** `(justification: not a trust-root because ...)` 自由文本 downgrade —— 防止 institutionalize `feedback_class4_cannot_hide_in_class3.md` 漏洞 |
| 宪法锚 | AGENTS.md §5 + §6 + Art. 0.4 + `feedback_class4_cannot_hide_in_class3.md` |
| 覆盖违宪 | ★ 直接；future Class-4 滑入 Class-2 PR 题头预防 |
| Class | 1 |
| Cost | M |
| 依赖 | 无 |
| Karpathy 边界 | inline const + 单 test; pattern list 同步 §6 现状 (`set(patterns) ⊇ set(§6)` 自检 per C1); 季度 `/harness-reflect` review |

### H8 — OBS Aging Escalation (REVISED: baseline=55 + quarterly ratchet per C3)

| 维度 | 内容 |
|---|---|
| 触发 gap | OBS-archive escalation 缺口 (第 10 个 gap，由验证 R2 cite 暴露) |
| 内容 (REVISED) | 新 `tests/constitution_obs_aging.rs`，扫 `handover/alignment/OBS_*.md` mtime + frontmatter：任一 mtime > N=14 天 且 无 `resolved: <PR/charter/atom-ref>` 或 `deferred-until: <date>` → test fail。**Baseline correction per C3**: actual count = **55** (not v1.1's 31; orchestrator counted via `find ... wc -l`)。55 现存 OBS grandfather 入 baseline allowlist。**Per C3 ratchet**: baseline **非 permanent** —— 季度 `/harness-reflect` 必须 shrink baseline by ≥ 5 (resolve OR `wontfix-permanently:` with rationale); 否则 `/harness-reflect` 报 OBS-debt 累积 fail。**Per C3 escape-hatch tightening**: `deferred-until:` 必须 ≤ 90 天 future date (防止 `deferred-until: 2099-01-01` silly-defer) |
| 宪法锚 | Art. I.1 L207-209 + AGENTS.md §8 (handover discipline) + Art. 0.2 #2 (OBS 是派生工件 — 长期未 resolve 会成 alt-source-of-truth) |
| 覆盖违宪 | ① 间接 (如有 mechanism, OBS_FENCE 在 14d 前升级触发 fence 修复); 预防未来 OBS-rot 类违宪 |
| Class | 1 |
| Cost | S (test); S (ratchet hygiene per quarter) |
| 依赖 | 无 |
| Karpathy 边界 | 单 test + baseline + 季度 ratchet 政策；不引入新 OBS schema；现有 OBS 不强制改字段 (grandfather) |

### H6 / H7 — DROPPED per Phase 5 三家共识 (见 §7 critic log)

- ~~**H6** `/derivative-artifact-emit` skill 泛化~~ — C2 (premature generalization, `feedback_defer_abstraction_until_second_impl` violated) + C3 (7th advisory skill = skill 疲劳；user 不会可靠调用) 共识 DROP。**补偿覆盖**: H4 在 CI 层捕获 (failure 时点是 PR review 而非 pre-emission)，trade-off 接受。
- ~~**H7** Weekly Cron Drift Detector~~ — C2 (premature daemon, KARPATHY_ARCHITECT §2 反对) + C3 (solo user 不会维护 weekly review cadence；cron 变 noise) 共识 DROP。**补偿覆盖**: H5 在单 PR 检测 trust-root surface 新增；H4 每 PR 检测 artifact 完整性。Multi-PR-only 漏失留给季度 `/harness-reflect` 手动 review。**Evidence-driven 再考虑**：如 H4/H5 demonstrably 漏 multi-PR-only failure，再设计 H7 v2。

---

## §5 推荐 ship 序列 (v1.2)

```
H0 — standalone, Class 4 sudo (无 phase 归属，独立)
   修 constitution.md L441 + L670 + L826-870 verify + Art. V.3 log entry
   → 走 §V.1.1 sudo + §V.3 amendment 流程
   → **Cadence A (架构师选, 2026-05-24)**: 完整 PRE-§8 clean-context Codex witness (single, per §9 updated doctrine)
   → ~~Cadence B (Art. 0.4 precedent)~~: 架构师未选用

Phase A — 单天, 3 个 Class 0/1 文本/测试:
   H1 (verdict 域 +1 token, 同步 CLAUDE.md + skills/orchestrate + skills/codex downstream pointers)
   H2 (mechanism: lint test + 45-entry grandfathered baseline + tightened escape)
   H8 (OBS aging test + 55-entry baseline + quarterly ratchet 政策 + 90-day defer cap)

Phase B — 1 atom charter, Class 0:
   H3 (Class 2 derivative-artifact cadence exception — AGENTS.md §5+§14 修订
       + ~80 文件 frontmatter `class: 2-derivative` + `depicts: [...]` hygiene pass)

Phase C — 1 atom charter, 2 个 Class 1 test:
   H4 (artifact completeness test, inline const, WIP exemption `handover/scratch/*`)
   H5 (trust-root surface test, inline const, tighter loophole [no free-text downgrade])

—— DROPPED (Phase 5 三家共识) ——
   H6 (skill 泛化)
   H7 (weekly cron drift)
```

**总成本**: 2 atom charter + 1 Class 4 hotfix (H0)。**Class 4 work = 1 (H0)**；其余 6 项 Class 0/1。

**Phase A 独立 ship**: H1/H2/H8 无相互依赖，不动 src/，可单天打成 3 个 PR。

**H3 + H4 + H5 顺序**: H3 是 cadence exception (定义谁要被审)；H4 是 artifact completeness mechanism (检查 cadence 接收的输入)；H5 是 trust-root mechanism (检查 cadence 升级的 trigger)。Phase B 先 H3 给后续 PR 一个 cadence 接收 channel；Phase C 再上 H4+H5。

---

## §6 诚实的残余 — 这些提案不能 catch 什么

1. **生成时 LLM 直接幻觉**: H4 在 CI 层 catch；不在 pre-emission 拦 (因 H6 DROPPED)。Trade-off 接受 (solo user skill 疲劳成本高于 pre-emission 收益)。

2. **Constitution 修订 vs FC 矩阵漂移**: 架构师改 constitution.md 加新 FC 节点但忘改 `TRACE_FLOWCHART_MATRIX.md` → H4 fail (正确 fail)。建议 H4 引入时 freeze baseline allowlist (同 H2 grandfather)。

3. **过去 N 个 PR 已合并的 visualization 工件**: H7 DROPPED, 没自动回看；当前 in-flight 违宪 diagram (committed) 需手动 cleanup 一次。Untracked in-session 生成物在 H4 落地后 CI 抓。

4. **★ 之外的 dual-substrate 候选**: H5 pattern list 是当前已知 trust-root surface；未来若出现「dual signing/capability/schema/Q_t」不在 pattern，仍可能漏过。补偿：季度 `/harness-reflect` review pattern list。

5. **Verdict domain 扩展 (H1) 不 retrofit 历史 audit**: 已完成 Codex/Gemini 审计仍是 4-token 域；新 token 只覆盖 going-forward。

6. **H3 Class 2-derivative cadence 不 retrofit 历史**: 只对加了 `class: 2-derivative` frontmatter 的工件生效；现有 ~80 个 handover/alignment 文件需 hygiene pass (建议同 H3 charter 完成)。

7. **本提案不解决 spec-html-renderer 之外的可视化生成路径**: 例如外部工具 (draw.io / Excalidraw) import — 仍依赖 H4 CI 触发。合理边界。

8. **OBS escalation 之外的 "filed-but-forgotten" 类**: H8 只 cover `handover/alignment/OBS_*.md`。`handover/proposals/` stale 提案、`TB_LOG.tsv` 中 OBS 引用、PR 描述中 "follow-up TODO" 等未 cover。Future charter 扩展。

9. **Cross-PR cumulative drift case** (H7 DROPPED 后): H4 + H5 在单 PR 层全覆盖；multi-PR-only 失败模式留给季度 `/harness-reflect` 手动 review。**Evidence-driven 升级**：如 H4/H5 demonstrably 漏 multi-PR 失败案例 (新 incident)，再设计 H7 v2 (cron 或 PR-blocking)。

10. **Sub-agent 报告引用需独立验证 (orchestrator-side lesson)**: 本次 Phase 3 我一度怀疑 R2 引用 OBS 是虚构 (false negative on first grep)；后续验证证明 OBS 真实存在。教训：sub-agent file path 引用应 `find -iname` 模糊匹配验证，非单次 grep。Behavioral discipline，**值得在下次 `/orchestrate` skill 修订时加入** "critic dismiss sub-agent finding 前必须 find 验证 file existence" 条款。

---

## §7 Phase 5 Critic Findings + Phase 6 Triage (audit trail)

### 7.1 C1 — Constitution Adversarial-Critic

**Verdict summary**: 3 MUST-FIX / 6 NICE-TO-FIX / 1 WONTFIX-WITH-REASON。

**MUST-FIX 全部吸收**:
- C1-1 (H0 misclassification): v1 写 H0 Class 1，但 constitution.md mutation = Art. V.1.1 sudo event = Class 4 cadence per AGENTS.md §5+§14。**v1.2 修复**: H0 Class 改 4；§5 ship sequence 把 H0 拆出 Phase A 独立 ship 走 §V.1.1 sudo + §V.3 amendment；§0 "0 Class 4" 改 "Class 4 work = 1 (H0)"。
- C1-2 (H3 fractional class): "Class 0.5" 破坏 §5 整数 schema (mechanism 依赖 `PR title states risk class` integer grep)。**v1.2 修复**: H3 改写为 Class 2 cadence sub-bullet，**不引入 0.5 / D 新 class 数**。
- C1-CC1 (Phase A "0 Class 4" inconsistency): 由 C1-1 引出。**v1.2 修复**: §0 + §5 同步更新。

**NICE-TO-FIX 大部分吸收**:
- C1 H0-2 (Art. V.3 amendment log 必填): 吸收 — H0 内容明示 (5) Art. V.3 log entry。
- C1 H1-2 (downstream pointer drift): 吸收 — H1 内容明示同步 CLAUDE.md / skills/orchestrate / skills/codex。
- C1 H3-2 (Codex witness 不升 Veto): 吸收 (隐含于 H3 cadence-only 措辞)。
- C1 H5-1 (YAML 作 §6 派生视图风险): 吸收 — H5 改 inline + `set(patterns) ⊇ set(§6)` 自检要求。
- C1 H5-2 (justification 自由文本 downgrade 漏洞): 吸收 — H5 内容明示禁自由文本 downgrade。
- C1 H7-2 (retro-classification 应 OBS 不 retroactive ship-gate): 部分吸收 — H7 已 DROP，未来 v2 设计时纳入。
- C1 CC2 (AGENTS.md cumulative delta): 部分吸收 — §0 总成本声明 "2 charter + 1 hotfix" 限制总 ceremony。

**WONTFIX-WITH-REASON 1**: C1 H1-1 (verdict 域扩展 = Veto-AI 越界候选): 经 C1 自己 re-read OK — witness 域非 Veto 域 (Art. V.1.3 L765)，5th token 仍是 objective citation-bound。Verdict: 允许 ship。

### 7.2 C2 — Karpathy Adversarial-Critic

**Verdict summary**: 3 MUST-FIX / 3 NICE-TO-FIX / 0 WONTFIX。

**MUST-FIX 全部吸收**:
- C2-H3 (Class 0.5 vague future extensibility): 与 C1-2 收敛。**v1.2**: H3 改 Class 2 sub-bullet (named cadence variant, no fractional class)。
- C2-H6 (premature generalization): `feedback_defer_abstraction_until_second_impl.md` 违反；spec-html-renderer 与 derivative-artifact-emit 共享零实现 surface。**v1.2**: H6 DROP；H4 CI 层捕获是真正 enforcement layer。
- C2-H7 (premature daemon): KARPATHY_ARCHITECT §2 ("monolithic & flat by default; do not introduce background daemons before real bottleneck")。H4+H5 单 PR 已覆盖；H7 cost L 但 leverage hypothetical。**v1.2**: H7 DROP。

**NICE-TO-FIX 全部吸收**:
- C2-H2 (add micro-example at ship): 吸收 — H2 内容明示自指闭合 (meta-norm 必须 declare `mechanism: this-test`)。
- C2-H4 (inline whitelist instead of YAML): 吸收 — H4 改 inline Rust const，明示 mirror `BASELINE_ALLOWLIST` pattern。
- C2-H5 (inline patterns instead of YAML): 吸收 — H5 改 inline Rust const，明示 mirror `LEGACY_GLOBAL_MARKOV_POINTER`。

**C2 整体评价**: "5/8 surgical; 3/8 cross into ceremony — drop H3/H6/H7 and inline H4/H5 config, package becomes clean 5-proposal surgical hardening。" v1.2 全部吸收 (注: C2 推荐 drop H3 整体；C1 推荐保 H3 改名；用户角度 H3 cadence 收益高 — 折中: 保 H3 但折叠为 Class 2 sub-bullet 不新增 class 数)。

### 7.3 C3 — Practical-Deployment Adversarial-Critic

**Verdict summary**: 4 MUST-FIX / 3 NICE-TO-FIX / 0 WONTFIX。

**MUST-FIX 全部吸收**:
- C3-H3 (solo user 不会自标 Class 0.5 frontmatter): 与 C1-2 + C2-H3 三家收敛。**v1.2**: H3 改 Class 2 sub-bullet (frontmatter 仍要标但 class 数不增)。
- C3-H6 (7th advisory skill = skill 疲劳): 与 C2-H6 收敛。**v1.2**: H6 DROP。
- C3-H7 (solo user 不会维护 weekly review cadence; cron 变 noise): 与 C2-H7 收敛。**v1.2**: H7 DROP。
- C3-H8 (factual: 31 → 55 baseline): **v1.2**: H8 baseline 改 55 (orchestrator find 验证)。

**NICE-TO-FIX 全部吸收**:
- C3-H1 (audit-prompt files 同步): 吸收 — H1 内容明示 downstream pointer 同步。
- C3-H2 (escape hatch too cheap): 吸收 — H2 改 `none-with-charter <charter-ref>` 必填 charter 引用，禁自由文本。
- C3-H4 (WIP exemption): 吸收 — H4 加 `handover/scratch/*` 排除 path。

**C3 H8 ratchet 建议** (`/harness-reflect` 季度 shrink baseline by ≥ 5 + 90-day defer cap): 吸收 — H8 内容明示。

### 7.4 Cross-critic 收敛点

| 议题 | C1 | C2 | C3 | v1.2 决策 |
|---|---|---|---|---|
| H3 Class 0.5 | MUST-FIX | MUST-FIX | MUST-FIX | 改 Class 2 sub-bullet, no new class |
| H6 premature | — (no constitutional) | MUST-FIX | MUST-FIX | DROP |
| H7 premature | — (no constitutional) | MUST-FIX | MUST-FIX | DROP |
| H0 Class | MUST-FIX (Class 4 sudo) | — | — | 改 Class 4 sudo |
| H1 downstream sync | NICE-TO-FIX | — | NICE-TO-FIX | 吸收 |
| H4/H5 YAML | NICE-TO-FIX (H5) | NICE-TO-FIX (both) | NICE-TO-FIX (H4 frontmatter) | inline const |
| H8 baseline 数 | — | — | MUST-FIX (factual) | 改 55 |
| H8 ratchet | — | — | (raised) | 季度 ratchet + 90d cap |

**最强三家共识**: H6 + H7 DROP。**最强二家共识**: H3 fractional class 反对。**单家强约束**: C1 H0 Class 4 reclassification (宪法刚性要求)。

---

## §8 引用

**Sub-agent reports (this session, 2026-05-24)**:
- R1 Harness Mechanism Catalog (167 mechanisms / 8 categories)
- R2 Violation Forensics Report (verdict: SUPPORTED)
- R3 Structural Gap Taxonomy (dominant cause: δ)
- C1 Constitution Adversarial-Critic (Phase 5)
- C2 Karpathy Adversarial-Critic (Phase 5)
- C3 Practical-Deployment Adversarial-Critic (Phase 5)

**Source-of-truth files**:
- `AGENTS.md` §1 / §5 / §6 / §9 / §14 / §14a / §15
- `CLAUDE.md` §3 / §4 / §5 / §6 / §9
- `constitution.md` Art. 0.2 / Art. 0.4 (L114-122) / Art. I.1 (L163-199, L207-209) / Art. V.1.1 (L704-715) / Art. V.1.2 / Art. V.1.3 (L740-765) / Art. V.3 (L804-813) / FC3 graph (L826-870)
- `handover/alignment/TRACE_FLOWCHART_MATRIX.md` (FC1/2/3 hashes)
- `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md`
- `handover/alignment/OBS_CONSTITUTION_MERMAID_FENCE_2026-04-22.md` (H0 recipe)
- `scripts/constitution_gates.manifest.toml` (133 [[gate]] blocks)
- `scripts/run_constitution_gates.sh`

**Prior art**:
- `handover/architect-insights/K-1-6_HARNESS_SHAPE_AUDIT.md`
- `handover/architect-insights/K-2-2_TRUTH_TIER_GREP_RECEIPTS.md`
- `handover/architect-insights/K_HARDEN_PROPOSAL_2026-05-20.md`
- `handover/architect-insights/AMENDMENT_2026-04-26_art-0-turing-fundamentalism.md`

**Karpathy doctrine** (C2 anchor):
- `skills/KARPATHY_ARCHITECT.md`
- `skills/KARPATHY_SIMPLE_CODE.md`

**Memory norms (instruction-only, mechanization candidates per H2)**:
- `feedback_norm_needs_mechanism.md` (meta-norm; self-instruction-only)
- `feedback_class4_cannot_hide_in_class3.md`
- `feedback_chaintape_externalized_proposal.md`
- `feedback_rejection_evidence_separate.md`
- `feedback_tape_first_real_tests.md`
- `feedback_defer_abstraction_until_second_impl.md` (C2 anchor for H6 DROP)
- `feedback_no_workarounds_strict_constitution.md` (C3 anchor for H2 escape tightening)

---

**Status**: v1.2 — Phase 5 (3 Critic) 完成 + Phase 6 triage 吸收。Pending Phase 7 Shipping-Witness verdict before architect ratification.
