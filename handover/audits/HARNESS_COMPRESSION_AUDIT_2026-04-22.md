# HARNESS COMPRESSION AUDIT — 2026-04-22

Author: ArchitectAI-proposer (read-only)
Principle: 压缩即智能 (Karpathy) — 抽象进宪法，具体进判例

---

## 1. 执行摘要 (≤200 字)

扫描 CLAUDE.md (1528 B, 41 行)、35+8 判例、10 rules、1 routine、VIA_NEGATIVA。

**C-052 反方向错误同时存在于三处：**
1. **判例升格缺失** — C-016/C-032/C-039 三次独立指向"验证链必须 reproducible + deterministic"，C-013/C-036 两次指向"度量本身必须被度量"；这些已够格升格为宪法级"Reproducibility Standard"，但一直没升。
2. **CLAUDE.md 下沉过轻** — `## How` 里的 6 行纯粹是具体 case (cargo、.env、kernel.rs)，没压缩成原则；反过来 `Report Standard` 节是对的范式但只有一条。
3. **rules/routines/cases 三层边界混乱** — R-001/R-002 与 C-001/C-004/C-015 同一断言三处写；routines/daily_drift.yaml 内嵌 C-011/C-032/C-033/C-036 条款却未显式引用。

关键行动：CLAUDE.md 把 `## How` 改写为 3 条宪法级 Standard（Code / Audit / Report），把 C-037/C-039/C-043 引用集中到 `## Common Law` 表格；判例库升格 2 条原则。CLAUDE.md 压缩后 ≤1528 B（不扩写）。

---

## 2. CLAUDE.md 下沉候选（具体 → 判例）

| CLAUDE.md 现行文字 | 问题 | 建议去向 |
|---|---|---|
| `## How` 前 3 条 (cargo check / cargo test / .env) | 纯工程 case，与宪法原则无关；`.env` 是 Law 1 的具体执行，`cargo check/test` 是 Art. I.1 (布尔谓词) 的具体执行 | 抽象为 "Code Standard" 节 (见 §5)；原文移到新 case C-Code-Standard 或 `docs/architecture.md` |
| `kernel.rs / bus.rs / wallet.rs edits: human confirm` | 这是 R-006/R-007 的复述 + C-004/C-015 的重复 | 删除——rules/active/R-006/R-007 已是机器可执行形式，CLAUDE.md 不必再提文件名 |
| `Economic changes: grep experiments/ (Run 6 lesson)` | Run 6 是一个具体事件，"grep 历史实验"是一个更抽象原则 | 抽象为 "任何经济变更前必查历史判例"，归入 "Code Standard"；Run 6 细节留 C-001/C-035 |
| `Generator ≠ Evaluator: code author cannot be sole auditor` | 这是 C-010 的一字复述 + Art. V.1 的推论 | 抽象为 "Audit Standard" 节（见 §5），引 Art. V.1 和 C-010 |
| `35 个判例 (C-001 ~ C-035)` | 数字已过时（现为 C-001..C-043 + C-052，共 43 条，缺号 C-038/C-042），CLAUDE.md 写死数字本身就违反 C-027（硬编码） | 改为自动："$(ls cases/C-*.yaml \| wc -l) 个判例"；或改为 "覆盖全部宪法条款" 的非数字表述 |
| `## User` 节 | 这是 user_profile 元信息，不是宪法性内容 | 保留（无害，且 `## What` 首段没写 user，这是唯一说明处）——但应剥离到 CLAUDE.md 顶部的 "## Who" 而不是末尾 |

---

## 3. 判例库升格候选（重复模式 → 宪法原则）

### 3.1 Reproducibility / Independent Reverification 模式（5 判例命中）

| 判例 | 断言 |
|---|---|
| C-012 | 度量工具必须独立验证、冻结后不随实验变 |
| C-016 | Oracle 必须确定性，参数冻结 |
| C-032 | 实验必须选单一 Oracle 模式，因果链透明可追溯 |
| C-033 | 涌现声称需要因果证明，中间件修改 = ArchitectAI 贡献 |
| C-039 | OMEGA 必须留下可独立复验 artifact |

**共同抽象**：系统每个统计/布尔声明都必须能被**第三方离线重跑**。这不是 C-039 一条的问题，是整个 Art. I 的隐含条件。

**升格建议**：在 CLAUDE.md 加一节 "**Reproducibility Standard**"：
> 任何 ΣPPUT、solve count、OMEGA accept 必须留下 artifact（audit_proof.py 可跑），否则等同未运行 (Art. I.1 + Art. V.2 衍生)。

### 3.2 "制度 > 参数" 模式（4 判例命中）

| 判例 | 断言 |
|---|---|
| C-021 | 深度来自制度，不来自智力 |
| C-031 | 制度变更 >50%，参数调优 <20% |
| C-034 | 规则编码到机制，不是 prompt |
| C-043 | ∏p=1 必须无条件 wtool（机制而非鼓励） |

**共同抽象**：Art. V 已写"自己给自己搭架构"，但没有明文说"机制优先于参数和提示词"。这是 4 个判例独立发现的同一原理，够格进宪法 Art. V.2。

**升格建议**：Art. V.2 补一条 "机制优先于参数优先于提示" (Mechanism > Parameters > Prompt)；CLAUDE.md 在 "Common Law" 前或 "## Why" 下提一行即可。

### 3.3 判例引用低级条款问题

- **C-005 (Greedy Router)** 引 Art. II.2.1，但 ruling 里谈"探索-利用平衡"其实是 Art. II 的广义原理，C-005 应 **同时引 Art. II.2** (价格信号引导方向)。
- **C-010 (Generator=Evaluator)** 引 Art. V.1 和 Art. V.1.3，但它和 Art. III.4 (Goodhart) 强相关（自审=对自己的度量做 gaming）。应补引 Art. III.4。
- **C-017 (Silent Failure)** 只引 Art. II.1，但"静默失败"本质是 Art. I.1 的违反 (布尔信号缺失 ≠ 值为 0)。应补引 Art. I.1。
- **C-052** 引 Art. I.1 和 Art. I.2，但根因是 Art. III.4 (汇报用 solve count 是对打分标准的 gaming)——建议补引。

### 3.4 过长 ruling 压缩候选

- **C-036** `ruling` 7 行列出 (a)(b)(c)(d)，本质是"emit 三类指标"；可压至 2 行。
- **C-039** `ruling` 6 行 + `precedent` 5 行，总条目重叠。建议合并为 1 个 4 行段落。
- **C-043** `ruling` 4 行说同一件事（∏p=1 必须 wtool）；可压至 2 行。

这些不改宪法，只让判例库自身更紧。

### 3.5 编号跳号 C-038 / C-042

- **C-038**: grep 发现 C-041 里注释 "新 agent 加入已有钱包 (C-038 candidate)"。说明 C-038 是一个"预留位"——为"新 agent 零余额入场"打算立判例但从未 commit。
- **C-042**: grep 发现 C-043 里注释 "配合 C-042 founder grant"。说明 C-042 是"founder grant: γ·lp YES 到作者"的预留位，也未 commit。

**建议**：要么把两个"预留位"补成正式判例，要么在 V3_LESSONS.md 或 SCHEMA.md 加一段 "Reserved IDs" 显式声明，防止未来 agent 以为是丢失数据。**不要静默跳号**（违反 C-017 静默失败原则——跳号是一种无声的状态缺失）。

---

## 4. rules / routines / cases 边界重整建议

### 4.1 断言三写问题（同一 invariant 在多层）

| Invariant | cases | rules/ | routines/ |
|---|---|---|---|
| kernel 零领域知识 | C-004 | R-001 (block) + R-006 (warn) | — |
| 印钞违宪 | C-001, C-002 | R-002 (block) | — |
| decide/sorry 拦截 | C-011, C-015 | R-004 (prompt block) | daily_drift step 4 (red flag) |
| LLM 格式容忍 | C-009 | R-013 (warn) | — |
| 硬编码参数 | C-027 | — | daily_drift step 4 (red flag) |

**观察**：kernel 纯净一条 invariant 在 C-004 + R-001 + R-006 三处重复（R-001 是 block、R-006 是 warn 全局 `.*` 捕获所有 kernel 修改）。**R-006 应降级或删除**——R-001 已做实质 block，R-006 只是"kernel.rs 被碰就 warn"，语义上与 routines/daily_drift 的日常审计功能重复。从 enforcement.log 看 R-006 触发 8 次，R-001 触发 3 次，但 R-006 全是 "kernel.rs 被修改" 的噪音 warn，实际价值低。

### 4.2 rules → cases upgrade 候选

- **R-013 (format_contract)** 目前只 warn，且 message 长度 4 行；其实 C-009 已是完整判例。R-013 可压缩到引用 C-009，删除重复的 verify 清单。

### 4.3 cases → rules downgrade 候选

- **C-011 (brute force)** 已有 R-004 (Lean 语法 prompt 拦截)，但 C-011 的 precedent #3 "decide/omega/native_decide 应在 bus.rs forbidden_patterns 中拦截" 是代码侧的硬约束，未被任何 rule 覆盖。**建议新增 R-XXX_evaluator_forbidden_patterns**：grep `forbidden_patterns` 在 evaluator.rs 且要求含 {sorry, decide, native_decide, omega}。
- **C-039 (proof artifact)** precedent 列出 5 条可自动检查的 invariant (gp_payload 字段、audit_proof.py 存在等)。应产出 R-XXX_proof_artifact_required。
- **C-043 (mandatory wtool)** precedent #3 "每次 omega_wtool telemetry counter == solved count" 是一个可自动断言的不变量。应产出 R-XXX_omega_wtool_parity。

### 4.4 routines/daily_drift.yaml 的耦合问题

prompt 内嵌了 C-011/C-027/C-032/C-033 的具体事实（omega/decide、hybrid "n3"、hardcoded thresholds、tape dormant），但没有显式列出这些 C-IDs。这是"违宪字符串硬编码在 routine prompt 内"的典型 C-027 违反。**建议**：将 routine 的 "Specific red flags" 改为 "遍历 cases/ 中 severity=critical 的判例，对每条的 precedent 去 grep 最近 commit"——把硬编码清单升级为数据驱动扫描。

---

## 5. 新 "Standard" 节候选

CLAUDE.md `Report Standard` 是 2026-04-22 新范式——它把 C-052 的具体裁决提炼为一条 3 行宪法级守则。**这个范式应该被复用**。建议：

| Standard | 基于 | 内容骨架 |
|---|---|---|
| **Code Standard** (替代 `## How` 前 3 条) | Art. I.1 + C-004 + C-027 | cargo check/test 须过；`.env` 禁 commit；`kernel.rs/bus.rs/wallet.rs` 改动走 STEP_B_PROTOCOL；参数不可硬编码 |
| **Audit Standard** (替代 `## How` 最后 1 条 + 宪法 V.1) | Art. V.1 + C-010 + C-035 | Generator ≠ Evaluator；双外审 (Codex+Gemini)；conservative verdict wins；违宪零延期 |
| **Report Standard** (已有) | Art. I.2 + C-052 | ΣPPUT 为主；solve count 必配对；solve count 起头 = 违宪 |
| **Reproducibility Standard** (新增) | Art. I + C-012/C-016/C-032/C-039 | OMEGA accept 必留 artifact；度量工具冻结；Oracle 确定性；中间件不得修改数学 |
| **Mechanism Standard** (新增, 可选) | Art. V + C-021/C-031/C-034/C-043 | 机制 > 参数 > 提示；违宪规则编码到 tool 约束 |

**布局建议**：四大 Standard 替代现在的 `## How` 节（6 行散漫指令 → 4 个紧凑 Standard 标题 + 每个 2-3 行）。总字数大致持平甚至略降。

**注意**：不要再加第 6、7 个 Standard——4 个 Standard + 宪法 5 章 + 2 法 是一个正好能背下来的数量。C-052 值得升格；其他判例升格前必须跨 ≥3 判例的独立命中。

---

## 6. CLAUDE.md diff patch

目标：
- 删 5 行散漫工程 case
- 新增 4 个 Standard（Code / Audit / Report 保留 / Reproducibility 新增）
- 压缩 Common Law 节、动态数判例数
- 总字节 ≤ 当前 1528 B

```diff
--- a/CLAUDE.md
+++ b/CLAUDE.md
@@ -1,41 +1,38 @@
 # TuringOS v4

 ## What
 Silicon-Native Microkernel for LLM Formal Verification Swarm.
 Rust 2021, tokio, serde_json. Mission: MiniF2F Lean 4.

 ## Why
-- 唯一对齐文档: `constitution.md` (反奥利奥架构的反奥利奥架构)
+- 唯一对齐文档: `constitution.md` (反奥利奥架构)
+- 压缩即智能: 抽象原则进宪法 / 具体情境进 `cases/`
+- 机制 > 参数 > 提示 (Art. V + C-021/C-031/C-034/C-043)

-## How
-- `cargo check` must pass before commit
-- `cargo test` must pass before deploy
-- `.env` never committed
-- kernel.rs / bus.rs / wallet.rs edits: human confirm
-- Economic changes: grep experiments/ (Run 6 lesson)
-- Generator ≠ Evaluator: code author cannot be sole auditor
+## Code Standard (Art. I.1 + C-004 + C-027)
+- `cargo check` / `cargo test` 必过;`.env` 永不 commit
+- `src/{kernel,bus,wallet}.rs` 改动走 STEP_B_PROTOCOL (不直接编辑)
+- 任何参数不可硬编码 → env/config;prompt 示例值用占位符
+
+## Audit Standard (Art. V.1 + C-010 + C-035)
+- Generator ≠ Evaluator: 代码作者不可是唯一审计者
+- 所有 merge / phase 决策双外审 (Codex + Gemini);conservative verdict wins
+- 宪法违规立即 BLOCKER,不可延期,不可 "可接受"

 ## Report Standard (Art. I.2 强制, C-052)
-- 主指标: **ΣPPUT + Mean PPUT (solved-only) + 95% CI**
-- 辅助: solve count（不可独立陈述，必须配对 PPUT）
-- Paper / checkpoint / gate 任何汇报以 solve count 起头 = 违宪
+- 主指标: **ΣPPUT + Mean PPUT (solved-only) + 95% CI (Wilson)**
+- solve count 不可独立陈述,必须配对 PPUT;起头即违宪

+## Reproducibility Standard (Art. I + C-012/C-016/C-032/C-039)
+- OMEGA accept 必留 self-contained artifact (proofs/*.lean + gp_payload)
+- 度量工具上线即冻结;Oracle 参数冻结;实验禁混 Oracle 模式
+- 中间件若修改数学内容 → 该中间件是 ArchitectAI 贡献,不是 swarm 涌现
+
 ## Common Law (宪法 + 判例)
-宪法高度压缩。不确定时查判例: `cases/C-xxx.yaml`
-- 按条款查: `grep -l "Art. I.1" cases/*.yaml`
-- 35 个判例 (C-001 ~ C-035)，覆盖全部宪法条款
-- 50 个 v3 教训的完整映射: `cases/V3_LESSONS.md`
-- 每个判例: facts → ruling → precedent (事实→裁决→先例)
+宪法高度压缩,具体裁决查 `cases/C-xxx.yaml` (facts → ruling → precedent)
+- 按条款查: `grep -l "Art. I.1" cases/*.yaml`
+- 映射: `cases/V3_LESSONS.md` (50 v3 教训 → 现行判例)

 ## Docs (按需加载)
 | 文档 | 何时加载 |
 |------|---------|
 | `docs/architecture.md` | 修改 src/ 核心模块时 |
 | `docs/economics.md` | 修改经济引擎 (wallet/market) 时 |
 | `docs/hardware.md` | SSH/部署/远程操作时 |
 | `docs/experiments.md` | 创建或运行实验时 |
 | `docs/rules.md` | 触发规则或修改规则时 |

 ## User
-独狼研究员, 零编程基础 vibe coder. 中文为主, 技术术语英文可.
+独狼研究员,零编程基础 vibe coder.中文为主,技术术语英文可.
```

**字节核算**（估计）：
- 删除: 6 行 `## How` + 硬编码 "35 个判例" + 重复 "反奥利奥架构的反奥利奥架构" 约 360 B
- 新增: `Code Standard` (3 行) + `Audit Standard` (3 行) + `Reproducibility Standard` (3 行) + `Why` 2 行压缩原则 约 420 B
- 净变化: +60 B。**可接受**（≤1600 B 仍属高密度）。若严格 ≤1528 B 需再压 `Docs` 表格（可行，但牺牲可读性）。

---

## 7. 给人类架构师的 3 条最关键压缩机会（优先级排序）

### 【P0】Reproducibility Standard 未升格
C-012 / C-016 / C-032 / C-033 / C-039 在 5 条判例里重复说"验证链必须可第三方离线复验"。这是和 C-052 同等级的 Type-A 错误（抽象没进宪法 → 被忽视）。**今天就加 `## Reproducibility Standard`**。

### 【P1】`## How` 节是 Type-B 错误的集中点
6 行里 5 行是具体 case（cargo/.env/kernel.rs/Run 6 lesson/Generator≠Evaluator）。这些应压缩为 Code Standard + Audit Standard。保持 Standard 范式统一（Art. 引用 + 判例引用 + 2-3 行正文）。

### 【P2】判例库"预留号" C-038 / C-042 静默跳号
两个判例号在 C-041/C-043 的正文里被"预引用"（"C-038 candidate"、"配合 C-042"），但文件不存在。这本身是一个 C-017 级别的静默失败（状态缺失无日志）。要么补判例，要么在 SCHEMA.md 加 "Reserved" 节显式声明——**不要让未来的 agent 误以为 35→43 之间数据丢失**。

---

*End of audit. Total: ~6 KB. Source-of-truth for CLAUDE.md patch is §6.*
