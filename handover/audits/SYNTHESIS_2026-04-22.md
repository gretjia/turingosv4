# 三路外部审计 Synthesis — 2026-04-22

> **2026-04-22 PPUT Amendment (二次修订, C-066)**：本 synthesis § 2 / § 4 以 solve count 做 baseline（"9/20 vs 17/20 输 40pp"）违 C-052。
>
> **一次修订声称"Phase 2.1b ΣPPUT=83.32 是历史峰值"— 该陈述来自 Agent A 污染数据，经用户质疑 + Claude 独立核查推翻**（见 `handover/audits/PPUT_HISTORICAL_AUDIT_2026-04-22.md` tombstone）。
>
> **真·baseline（`PPUT_RAW_DATA_2026-04-22.md` § 4）**：
> - ΣPPUT 绝对峰值 = 170.13（20260419T221252 dual-path baseline，N=43），非跨 run 可比
> - Mean PPUT (solved-only) top 3：6.158 / 5.561 / **5.354 (Phase 7)** — Phase 7 排第 3
> - Depth≥10 出现史：20260419T181411（1 个 depth-12，PPUT=0.11）+ Phase 7（3 个 depth 17/20/23，PPUT=0.65）
> - Phase 7 独特贡献：首次 `per_tactic` gp_path + 首次 depth≥10 solves ≥ 2
>
> 原 HOLD 裁决仍成立（4 条 BLOCKER 在）；"架构胜负"以 Mean PPUT + Σdepth≥10 PPUT 陈述。

---

**审计对象**：TuringOS v4 commit `e0a75ec`（Phase 7 Turing per-tactic δ-step 合并后）
**外部审计模型**：Codex (GPT-5.4) / Gemini 2.5 Pro / DeepSeek-Reasoner
**裁决规则**：VETO > CHALLENGE > PASS（保守胜出，`feedback_dual_audit_conflict`）
**源文件**：
- `handover/audits/EXT_CODEX_2026-04-22.md`
- `handover/audits/EXT_GEMINI_2026-04-21.md`
- `handover/audits/EXT_DEEPSEEK_2026-04-21.md`
- Claude 内部审计基线：`handover/ai-direct/EXT_AUDIT_2026-04-21/claude_internal_findings.md`

---

## § 1. 最终裁决：**HOLD**

**不能合并直接上线**。有 2 条 Codex VETO + 3 条 Codex 发现的新漏洞（Claude 未提及）+ 2 条 Gemini 统计性 CHALLENGE。下一阶段必须先过这 7 关。

---

## § 2. 外部 VETO（即停项，不可绕过）

### V-1. Codex VETO：F-20-05 封堵**不完整**，`append_oracle_accepted` 是 public unguarded 的第 4 层漏洞
- **证据**：`src/bus.rs:174-176` `append_oracle_accepted` 是 public API，盲目把调用者参数转成 `oracle_blessed = true`
- **现状靠的是"调用点纪律"**：evaluator `complete` 路径在 `verify_omega_detailed` 返回 true 后才调用（`evaluator.rs:585-624`）、`step` 路径在 `PartialVerdict::Complete/PartialOk` 后才调（`evaluator.rs:796-842`）
- **缺陷**：bus 本身没有 capability/proof token 强制验证"oracle 真的接受了这个 payload"。任何新接入的外部 agent 拿到 bus handle 就能绕过 forbidden_patterns
- **Claude 原声称**：F-20-05 已**三层**封堵 → 实际上是**四层**防御但**最关键的一层（bus 层门控）是召集约定、不是强制**
- **修复方向**：`append_oracle_accepted(payload, oracle_token)` 签名化，或把 oracle 的 verify 结果打成 signed receipt 让 bus 校验

### V-2. Codex VETO：Art. V 三权分立**非零-state 但实质未实现**
- **证据**：`routines/daily_drift.yaml:1-17` 的确启用了 JudgeAI-advisory 外部 routine；`handover/audits/` 在审计期间已非空（本次审计刚刚填入 3 份报告）；`handover/directives/` 仍为空
- **但是**：Rust runtime 和 CI 都没有 SOP enforcement；`.claude/agents/{proposer,auditor}.md` 只是 prompt 定义；`ROADMAP_LAUNCH_2026-04-21.md:22,82-84,107` 明确承认 P1-4 仍是"未来工作"
- **Claude 原声称**："pure prompt / 全空壳" → 措辞过强，但精神正确
- **DeepSeek 给出最小可实现方案**：InitAI + ArchitectAI + JudgeAI 三进程 + 只读宪法（chmod 444 + GPG 签名 + dm-verity） + 消息队列；至少 ArchitectAI 与 JudgeAI 必须不同模型；与 P0-6 争议通道整合（详见 EXT_DEEPSEEK Q1 架构图）

---

## § 3. Codex 发现的新漏洞（Claude 内部审计全部漏掉）

### N-1. `oneshot` 路径绕过 C-043 mandatory wtool ← **Art. IV 违宪**
- **证据**：`evaluator.rs:108-143,196-215` 的 oneshot 条件直接 `oracle.verify_omega` 拿结果就返回 `PputResult`，**不经过任何 bus write 或 settlement**
- **违反**：宪法 `constitution.md:521-524` 的 `∏p=1 → wtool → Q_{t+1}` + 判例 `C-043_mandatory_wtool_on_omega.yaml:15-20,26-34`
- **含义**：LATEST.md 声称"all Art. IV topology mechanisms landed on main" **不成立**。oneshot 是 v4 保留的 v3 遗留通道，仍然是宪法外飞地

### N-2. `bus.snapshot()` 返回的 `balances/portfolios` 是空的 ← **Art. II.2 经济信号失真**
- **证据**：`src/bus.rs:567-570` `snapshot()` 把 `balances: HashMap::new()` 和 `portfolios: HashMap::new()` 硬编码成空
- **链路**：`evaluator.rs:492-495` 把 `snap.get_balance(agent_id)` 喂进 prompt；`src/sdk/prompt.rs:78-84` 显示成 `Balance: {:.0} Coins`
- **后果**：agent 永远看到自己余额为 0，即使钱包里有 Coin。这直接解释了为什么"agents 从不 invest" — **agent 压根没看到市场信号**。所有 TAPE_ECONOMY 实验失败、Art. II.2 bounty 无人响应，很可能主因就是这个 bug，而不是激励曲线或 cold fee 不对
- **优先级**：应该**在 dual-mode N=50 实验之前**修掉 — 否则所有经济实验结论都失真

### N-3. `decide` / `omega` 未进 forbidden_patterns ← **C-011 只部分执行**
- **证据**：`experiments/minif2f_v4/src/lean4_oracle.rs:13-20,57-88` 只 ban `native_decide`；测试 `lean4_oracle.rs:395-399` 显式 assert `decide` 被允许
- **冲突**：`cases/C-011_brute_force_formalization.yaml:16-20` 要求三者都禁
- **风险**：F-20-05 后的"已封堵" 是**部分封堵**。agent 有机会学到用 `decide` / `omega` 暴力走决策过程，重演 native_decide 历史剧

---

## § 4. CHALLENGE（证据方向对但强度不足）

### C-1. Karpathy TOP-10 性能收益**大幅高估**
两家外部同时 CHALLENGE：

| Claude 声明 | Codex 判决 | Gemini 独立数值 | 差距 |
|---|---|---|---|
| `bus.rs:244-246` InvestOnly 三重 clone ~10-20%/tx | 不是热路径（没有 tool 发射 `ToolSignal::InvestOnly`） | 每 invest tx ~200B，非百分比 | 热路径前提不成立 |
| `bus.rs:416-424` `Box<dyn>` downcast ~5× 加速 | 不是 per-tx，仅 bus-side invest/refund/settle 路径 | ~1.05-1.1× 加速 | **Claude 严重失真**（5× vs 1.1×） |
| `src/ledger.rs:115-135` `trace_ancestors` HashSet alloc | **方向对** — 被 Boltzmann `lineage_score` + `is_frontier` 调用（`actor.rs:47-95`） | — | 唯一站得住 |

**综合**：Karpathy TOP-10 聚合预估"20-40% 加速 + 30-50% heap 降低"**不可信**。实际 ROI 最多 5-15% 加速 + 10-20% heap（且只在 append 真正发生时生效，而当前 Phase 7 log 显示 append 使用率为 0）。

**修正建议**：把 TOP-10 降级为 TOP-3，只保留 `trace_ancestors` + `author/payload to_string` + `graveyard/TopKClasses 去重` 这三条。其他条目推迟到 agent 数量上到 100+ 再说。

### C-2. Phase 7 DAG "emergent" 在 N=20 上**统计不足**
- **Gemini 判决**：深度分布 `{1:5, 3:1, 17:1, 20:1, 23:1}` 的结构特征**能**排除"简单背诵"（因为 step-only + 每步 oracle 验证的机制不允许），但 N=20 随机性太大，不足以排除"lucky seed"
- **具体证据**：
  - `imo_1964_p2` 的 `have h12 : ... := by nlinarith` — LLM 可能猜测但需要 runtime feedback 选对代数形式
  - `mathd_algebra_332` 从简单失败策略（`eq_of_sqrt_eq_sqrt`）演进到复杂成功策略（`pow_inj`）— 需要 oracle reject 引导
  - `imo_1981_p6` 73% reject rate → 精确的 rewrite 链是试错产物
- **Gemini 补强要求**：**3-5 次独立 seed × N≥50**，如果多样化深度分布稳定复现，才能 95% 置信度说"真涌现"

### C-3. Dual-mode N=50 实验设计本身**统计功效不足**
- **Gemini power analysis**：
  - H0: `p_dual = p_mono = 0.85`；H1: `p_dual ≤ 0.75`
  - N=50, α=0.05 → 实际 power = **46%**（远低于 80%）
  - 需要 **4 × N=50 = 200 配对**才能有 80% power 检测 Δ=2 solves
- **反事实警告**：如果 dual-mode 跑出 `{35, 37, 33}/50`：均值 0.70，95% CI = (0.627, 0.773)，**不含 0.85** → **不能宣称"恢复到 monolithic 水平"**。反而能以 >95% 置信度说 dual-mode **显著低于** monolithic
- **LATEST.md Next Steps #1 需要改写**：不是"跑一次 dual-mode N=50"，而是"跑 4 seeds × N=50 配对"

---

## § 5. PASS（三家共识，Claude 内部审计成立）

- **Art. V.1.1 宪法文件可写**（mode 644，无 git hook，无 CI lock） — 3 家都 PASS，P0-4 经济常量固化必须同步做宪法文件保护
- **Art. III.1 园丁 GC 缺失** — 代码只有 append-only 储存，`ledger.rs:27-35` 明确注释"NEVER modified or removed"；修复方向：tombstone + 被引用计数 GC
- **`recent_rejections(author,…)` author 参数被吞** — Codex PASS，与 Art. II.1/C-022 意图一致但与 Art. III.3 有张力；**建议立 C-047 形式化这个张力**
- **Law 2 CTF 守恒**：Gemini 确认核心逻辑 OK，但 **invest refund** + **Hayek bounty payout** 两条路径未进 `tests/reward_pull_conservation.rs`；需补测试而非找到具体反例

---

## § 6. DeepSeek 战略层新洞见（Claude 未深挖）

### S-1. Art. III.4 Goodhart 在**开源场景下必然失效**
- 代码公开 → 攻击者可逆向度量逻辑
- 宪法级**承认**这一点，然后用经济成本（P0-2 reputation-weighted γ）替代保密
- 含义：Art. III.4 需要一条补充判例明确"开源下的 Goodhart 防御主要靠经济成本而非保密"

### S-2. 攻击者长期收益率的**稳态分析**
- 在 P0-1（签名）+ P0-2（PoW + reputation γ）+ P0-3（append 质押）+ P0-6（challenge）齐备时，攻击者长期收益可能为负
- **但前提是参数调优**：新手关成本 $0.01 太低 / 质押金额太低 / 信誉曲线太平 / 挑战激励不够 — 任一项参数失配都让收益翻正
- **上线前必须做攻击模拟**（红队演练），不能靠机制直觉

### S-3. Persistent-fail 主因是 **(c) per-tactic Lean 延迟**（infra），不是 (a) 架构 bug 或 (b) 模型上限
- 含义：应该优先做**async oracle + cached elaboration + parallel step verify**，而不是新机制或升级模型

### S-4. "TuringOS scaffold 价值主张"需要**重新定位**
- 原命题："scaffold + 弱模型 > 强模型 oneshot"
- DeepSeek 反问：如果 Opus oneshot 能 90%，scaffold 对于**单次 solve rate** 的边际价值趋 0
- 新命题（DeepSeek 建议）：scaffold 的价值是**协作 / 经济激励 / 可验证性**，而不是"补偿弱模型"
- **战略含义**：上线后的 pitch 不能是"我们用 DeepSeek 也能解题" — 那 Opus 生态会吃掉；应该是"我们提供多方协作的宪法级框架"

### S-5. 上线成功的 3 个量化指标 + 4 条下线红线
| 维度 | 成功指标 | 下线红线 |
|---|---|---|
| 外部 agent 接入 | 日增 agent 数 + 活跃外部 agent 占比 | 连续 7 天 0 接入 / 活跃外部 <10% |
| 解题效率 | 日均解题数 + 解决率 | 日均 <1 / 解决率 <50% |
| 经济健康度 | Coin 流通深度 + Gini 系数 | Gini >0.8（极度不平等）/ 流通深度 <0.1（僵化） |
| 安全性 | — | 女巫成功 / 单日通胀 >10% / 数据被篡改 |

---

## § 7. 更新后的 Launch Gating（ROADMAP P0 修订）

按"外部审计后"重新排序：

| # | 原 P0 / 新增 | 严重度 | 依据 |
|---|---|---|---|
| **P0-0** | **修复 `bus.snapshot()` 空 balances/portfolios** ← 新增 | **BLOCKER** | Codex N-2。在修这个之前，**所有 TAPE_ECONOMY / 经济实验结论全部不可信** |
| **P0-0b** | **`append_oracle_accepted` 加 capability token** ← 新增 | **BLOCKER** | Codex V-1。外部 agent 上线前必须封堵 |
| **P0-0c** | **`decide`/`omega` 进 forbidden_patterns** ← 新增 | **BLOCKER** | Codex N-3。C-011 完整执行 |
| **P0-0d** | **`oneshot` 路径走 C-043 mandatory wtool** ← 新增 | **BLOCKER** | Codex N-1。Art. IV 违宪必须修 |
| P0-1 | Ed25519 身份签名 | High | 原计划，仍 M |
| P0-2 | 女巫防御（Gemini sigmoid: γ=0.05/(1+exp(-0.146(n-25))), Sybil gain O(1)）| High | 原计划，数学设计已有 |
| P0-3 | Tape 污染 + 园丁 GC | High | 原计划 + 3 家 PASS |
| P0-4 | γ/β/θ 经济常量固化 + **宪法文件 chmod 444 + GPG hook** | High | 合并两条，原 P0-4 扩展 |
| P0-5 | Oracle DoS: fingerprint cache + 梯度定价（Gemini 验证 20-35% hit rate, 1.43×, 成本效益 > 2× workers） | Medium | 数学已验证 |
| **P0-5b** | **async oracle + cached elaboration + parallel step verify** ← 新增 | Medium | DeepSeek S-3。persistent-fail 主因 |
| P0-6 | 争议治理通道（整合 DeepSeek Art. V 三进程最小架构） | High | 扩展为 Art. V 完整实现 |
| P0-7 | Quickstart + 参考 agent 客户端 | Low | 原计划 |

**修订后的 T+1 week 任务**：P0-0 / P0-0b / P0-0c / P0-0d 四个 BLOCKER **优先于原 P0-1~P0-7 全部**。

---

## § 8. 实验设计修正

### E-1. dual-mode 实验必须改
- 原设计：`TURING_STEP_ONLY=0` N=50 一次
- **新设计**：**4 seeds × N=50**（Gemini power analysis 要求）
- **修 P0-0 之后再跑**（空 balances bug 不修，市场行为不可信）

### E-2. DAG 涌现宣称必须撑
- 原数据：N=20 一次深度分布 `{1:5,3:1,17:1,20:1,23:1}`
- **补跑**：3-5 独立 seed × N=50，验证深度分布稳定复现
- 否则不能在对外 pitch 里说 "emergent depth"

### E-3. Karpathy 优化 ROI 复核
- 不要相信"20-40% 加速"聚合估计
- 逐条 micro-benchmark：`trace_ancestors` / `author.to_string` / `graveyard 去重`
- 其他 7 条先删掉，不做

---

## § 9. 建议立新判例（4 家共识）

| 判例 | 依据 | 关键 file:line |
|---|---|---|
| **C-044** Meta 架构未实现 | Codex Q1 VETO + DeepSeek Q1 | `.claude/agents/{proposer,auditor}.md` + ROADMAP P1-4 |
| **C-045** 园丁 Agent 缺失（Art. III.1 违宪） | 3 家 PASS | `ledger.rs:27-35` append-only |
| **C-046** 宪法文件未物理保护（Art. V.1.1 违宪） | 3 家 PASS | `constitution.md` mode 644, no hooks |
| **C-047** author 广播 vs 横向解相关的张力 | Codex Q6 PASS + Claude | `bus.rs:494-537` |
| **C-048**（新增） **`oneshot` 路径绕过 mandatory wtool** | Codex N-1 | `evaluator.rs:108-143` 违反 C-043 |
| **C-049**（新增） **`bus.snapshot()` 喂错 balances** | Codex N-2 | `bus.rs:567-570` 违反 Art. II.2 |
| **C-050**（新增） **C-011 部分执行（`decide`/`omega` 未禁）** | Codex N-3 | `lean4_oracle.rs:13-20,395-399` |
| **C-051**（新增） **开源下 Art. III.4 Goodhart 防御退化为经济成本** | DeepSeek S-1 | — |

---

## § 10. 一句话总结

**Phase 7 不能直接 merge-forward 上线**。Claude 内部审计抓到了大多数宪法层问题但漏了 4 个关键代码层问题（oneshot 绕过 / 空 balances / decide 未禁 / blessed-write 召集约定），性能收益宣称偏乐观 3-5×，"emergent DAG" 统计不足。先修 4 条 BLOCKER → 4 seeds × N=50 验证 → 再谈上线。
