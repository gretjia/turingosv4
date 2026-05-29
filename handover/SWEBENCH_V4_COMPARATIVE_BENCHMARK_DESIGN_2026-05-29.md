# TuringOS × SWE-bench Verified 对比基准设计文档（修订版 v3）
### 全系统消融实验（genesis → kernel → externalized-CoT loop）vs 裸 flash 模型 + 市场坐标 (n=50, DeepSeek V4 Flash worker)

> 文档状态：DESIGN ONLY（设计稿，无实现代码，仅给出 seam 引用）
> 日期：2026-05-29 · 仓库：`/Users/zephryj/work/turingosv4` · 基线 HEAD：`1f00012d` (main)
> 运行分支：**本 campaign 必须在 BRANCH 上运行**（架构指令：definitely branch；PR-only，禁 merge main）
> 撰写：benchmark-design architect · 路线已决（见下），本文不再重新论证路线选择
> 修订说明：本 v3 依四条架构指令（D1 措辞订正 / D2 worker=flash / D3 meta 模型 / D4 从 init 全流程进入）重写，并据已核实 findings（kernelTape / rolesModel / initLifecycle / v4Flash / dsmlFormat / externalizedCotPrecedent）重新界定科学主张归因边界。所有 file:line 引用已对照 findings JSON + 在 `1f00012d` 抽样复核。

---

## 0. 路线锚定（不再重新论证）

本设计在以下已决定的路线上展开，任何后续 session 不得重开此辩论：

- **主张 = 全系统消融（PRIMARY，归因边界已重界定，见 §1）**：完整 TuringOS 生命周期（`init`/genesis → 内核 → externalized-CoT 循环）vs **同一裸 flash worker 模型**（never enters kernel）、**同一脚手架前处理**、**同一组 gold-gated 实例** → resolve-rate 差值 + 95% CI + 配对检验。
- **关键归因订正（D4 的诚实后果）**：因为 loop 臂现在从 genesis 完整进入（不再走 `tdma run` 快捷方式），Δ 现在归因于 **整个 TuringOS 架构（genesis + 内核 + 外化 CoT 循环 + tape 富度）对裸 worker 模型** 的增量，**不是**狭义地归因于「retry 循环」单独一项。能分解哪个子组件驱动 Δ 需要进一步消融（见 §1.4）。
- **市场坐标 = 受混淆的次要参照（SECONDARY）**：OpenHands / Amazon Q / Anthropic 等平台**已发布的排行榜数字**，仅作为「市场坐标」引用，明确标注 model+scaffold 双重混淆，**不构成严格头对头主张**。OpenClaw / Hermes **非** Verified 选手（§2.2）。
- **不做的事**：我们**不**搭起 OpenHands / SWE-agent 去跑 DeepSeek V4（重型基础设施，已否决）。

### 0.1 四条架构指令在本 v3 的落地映射（导航）

| 指令 | 核心订正 | 主要落地章节 |
|---|---|---|
| **D1** 无「假 git tape」 | git-tape/ChainTape 是每次内核运行的**宪法法律**（Art. 0.2 + Art. 0.4）。loop 臂 = 进入内核（依法发 ChainTape + git tape）；control 臂 = 裸模型，**从不进入内核**（无 tape）。无 tape 的内核运行**违宪**。 | §1.1、§2.1、§3.2、§5.4 |
| **D2** worker = flash | worker 永远是 flash / 非 thinking 模型（现 DeepSeek V4 Flash）。ChainTape/git-tape **就是**显式外化的 chain-of-thought——worker 的推理活在 tape + loop 里，不在内部 thinking。 | §1.3、§2.1、§3.5、§6.1（cost headline）、铁律(§11) |
| **D3** meta 模型 | meta 可 thinking-on **若有理由**；但 swebench loop 里 meta 的角色是轻量/确定性谓词路由 + patch 生成——强 flash 足矣（findings 实证）。thinking 须 per-role 可配（已核实**未**硬编码）。 | §3.4、§2.1、§9 |
| **D4** 从 init 全流程 | 基准**必须**从 `init`/genesis 完整生命周期进入，**不**走 `tdma run` 快捷方式（后者假设预配置状态，流程不完整，无法测真实端到端 path-to-AGI 能力）。 | §1.4、§3.3、§4.5、§7(Atom 0/3)、§8(G0) |

---

## 1. 目标与科学主张

### 1.1 主张（PRIMARY，可辩护 —— 归因边界已据 D4 重界定）

> **在硬编码、答案无关验证（真实 SWE-bench 隐藏测试在 Docker 内执行）的编码修复任务上，完整的 TuringOS 架构（从 genesis 进入内核 + 外化 CoT 循环）是否优于同一 flash worker 模型的裸单发？**

精确形式：

- **被测变量**：是否经过**完整 TuringOS 生命周期**——`init`/genesis 引导（`build_chaintape_sequencer_with_initial_q`，runtime/mod.rs:706）→ 内核 verify-retry 循环 → **依宪法强制发出的 ChainTape + git tape**（Art. 0.2 + Art. 0.4，见 §1.2）。loop 臂**进入内核**；control 臂是**裸 flash 模型，从不进入内核**（无 tape）。
- **保持恒定**：worker 模型（`deepseek-v4-flash`，**thinking 显式 disabled**，见 §1.3/§3.5）、采样参数、对模型暴露的前处理上下文（消融校正后，见 §10）、judge（官方 swebench harness，100% 确定性、无 LLM，见 §3.4）、实例集（gold-gated）。
- **配对设计**：同一组实例同时跑两臂，按实例配对。
- **结论形式**：`Δresolve = p_loop − p_bare`，附 McNemar 检验 + 配对 bootstrap，并报告两臂各自的 Wilson 95% CI。

**我们将主张**：在 n（gold-gated hermetic 实例）上，loop 臂（完整 TuringOS）resolve 率为 `p_loop%`（Wilson 95% CI），裸 flash 臂为 `p_bare%`；配对差值 Δ 在统计上显著/不显著（McNemar χ², p；discordant b/c）。

### 1.2 归因边界（必须随主张明示 —— 这是诚实伙伴关系，不是合规辞令）

因为 D4 要求从 genesis 完整进入，科学主张的**归因边界发生改变**，必须平白陈述：

- ✅ **可以主张**：「完整的 TuringOS 架构（genesis → 内核 → 外化 CoT 循环）在同一裸 flash worker 之上增加了 X（Δresolve，配对 CI）」。
- ❌ **不能（仅凭本设计）主张**：哪个子组件——genesis 引导 / 内核循环 / tape 富度——驱动了 Δ。**分解需要进一步消融**（见 §1.4 的未来消融阶梯）。把 Δ 狭义归因于「retry 循环」是**越界**，clean-context 审计的 SECOND-SOURCE-DRIFT 域会标记这种僭越。
- ⚠️ **n=50 功效警告（保留）**：n=50 对小配对差欠功效；不显著时写「在 n=50 上未检出显著差异（功效有限）」，**不**写「无差异」（见 §4.2）。

> **为什么这是诚实的而非偷懒的**：D4 的指令本身改变了被测系统的边界。`tdma run` 快捷方式只测一个**已预配置**的内核子系统（无 genesis、无 Sequencer、无签名 typed-tx、无 CAS），那是一个不完整的、手工配置出来的子集，**测不出 TuringOS 真实的端到端 path-to-AGI 能力**（D4 原文）。一旦我们诚实地从 genesis 进入，Δ 的归因主体就**必然**是整个系统，而不能再假装只是「循环」一项。把窄主张写成宽系统其实是**降低**而非提高科学严谨性——反过来，把整个系统的 Δ 假装成某单一子组件（如循环）的功劳，同样是越界。

### 1.3 worker 模型恒为 flash（D2 —— externalized CoT 论题的核心）

- **worker = flash，推理外化到 tape**：worker 模型**永远**是 flash / 非 thinking 模型（现 `deepseek-v4-flash`，thinking 显式 disabled；将来可能换其他 flash 模型）。**理由**：ChainTape/git-tape **就是**显式外化的 chain-of-thought——worker 的推理不活在它的内部 thinking token 里，而活在 **tape + loop** 里。这是 TuringOS 的核心架构论题，与 Art. 0.2「所有信号必须可从 tape 重建」（constitution.md:54）直接呼应：若推理必须可重建，那它就必须落在 tape 上，而非藏在模型内部不可观测的 thinking。
- **不再有「thinking-on」worker 铁律**：旧版本曾把 worker 设为 `deepseek-v4-pro, thinking=on`。本 v3 据 D2 **废除**该设定。新铁律：**worker = flash，reasoning externalized to tape**（见 §11 铁律 1）。
- **flash 默认 thinking-ON 的陷阱（findings 核实，承重）**：DeepSeek V4 Flash 在 `thinking`/`reasoning_effort` 未显式设置时**默认开 thinking**（LiteLLM #27453 三条件对照测试证实；AnswerOverflow 线程佐证）。「Flash」≠「out-of-the-box 非 thinking」——它是一个 284B total / 13B active 的独立 MoE 小模型，不是 Google 意义上的「速度档位」。**必须显式传 `{"thinking":{"type":"disabled"}}`**——仅省略 `reasoning_effort` 或设 `"none"` **不足以**关闭（仍产 `reasoning_content`，触发多轮 400，LiteLLM #27453 Test 3 vs Test 1 对照）。本 campaign 的 worker 必须显式 disabled，否则「tape = 外化 CoT」的论题被模型内部 thinking 污染、且多轮 `reasoning_content` round-trip 会中断 trajectory（dsmlFormat findings）。
- **flash 的代价/能力前沿（市场坐标的新框架）**：flash 比 pro 便宜约 3x（输入 list rate $0.14/M cache-miss vs Pro $0.435/M cache-miss；cache hit $0.0028/M ≈ 比 cache-miss 便宜 50x；输出 $0.28/M vs Pro $0.87/M，详 §6.1）。因此 **cost-per-resolved 成为 headline 强项**：论证不再是「我们达到 X%」，而是「**flash + TuringOS 在某 cost/capability 前沿上，相对 thinking-models-alone 给出更优的每解美元成本**」。这是市场坐标的新框架（§2.2、§3.5）。

### 1.4 未来消融阶梯（明示「现在不能分解什么」）

为诚实标注 §1.2 的归因边界，列出**本设计不做、但分解 Δ 所需**的后续消融（供后续 campaign，不在本 n=50 范围）：

| 后续消融 | 隔离的子组件 | 设计草图 |
|---|---|---|
| A. genesis-on vs genesis-off（同 flash + 同内核循环） | genesis 引导（Sequencer/CAS/typed-tx/经济态）对纯内核循环的增量 | full-flow binary vs `tdma run` 快捷路径，同实例配对 |
| B. tape-rich vs tape-thin（同 genesis + 同循环） | tape 携带的推理 substrate 富度（§3.5 的 enrichment） | retry 上下文携带「失败测试名 only」vs「+ 先前 rationale/apply-error/attempt 史」（在屏蔽规则内）。⚠️ findings 核实：**该 ablation（tape-with-rationale vs tape-without，同数据集）在文献中尚不存在**（截至 2026-05），故 B 既是我们的科学贡献机会，也意味着无现成基线可借 |
| C. loop(k=N) vs single-shot（同 genesis） | 重试循环本身 vs 单发 | 匹配期望成本下比较（§6.1 pass@k 口径） |

→ **本 campaign 只产 §1.1 的全系统 Δ**。A/B/C 是「为什么有效」的后续科学，**明确超出本设计**。

### 1.5 坐标（SECONDARY，受混淆）

> **TuringOS + flash worker 在 SWE-bench Verified 上的绝对量级 + 每解成本，落在哪个已发布平台的邻域？**

**我们将主张**：「TuringOS + DeepSeek V4 Flash 达到 X%、每解 $Y，与 [平台] 用 [不同模型] 达到的 Z% 处于同一量级」——仅作方位参照，并以 cost-per-resolved 标出 flash 的前沿位置（§3.5）。

### 1.6 我们明确**不**主张

1. **不**主张「TuringOS 击败 OpenHands/Amazon Q/Anthropic」——这些数字是 (model × scaffold) 的乘积，模型不同则不可比。
2. **不**用「flash 在 TuringOS 上得 X%，而竞品用 Claude 只得 Y%」来证明 TuringOS 脚手架更优——模型差异本身就能解释大部分方差（同一 Claude 3.7 Sonnet 原味 62.3% / 自定义脚手架 70.3%，+8pp 全归因于脚手架；externalizedCotPrecedent findings）。
3. **不**用 DeepSeek V4 Flash 厂商自报的 79.0% 或 V4 Pro 的 80.6% **外推**出我们应得的分数——前者未独立复现且很可能是 thinking-enabled 口径（非我们用的 disabled，AA Intelligence Index non-thinking 36 vs Think-Max 47 暗示 disabled 明显更低，无公开直测值），后者是不同模型。
4. **不**把 Δ 分解到 TuringOS 的任一子组件（genesis vs 循环 vs tape 富度）——见 §1.2、§1.4，那需要进一步消融。
5. **不**在没有「git-history 访问已禁用/审计」声明的情况下，对任何 >70% 的分数（含他人的）赋予可信度——git log 泄漏 gold patch 是已知的作弊路径（SWE-bench Issue #465 类问题）。

---

## 2. 对比矩阵

### 2.1 消融臂（PRIMARY — 这是科学主张所在）

| 维度 | TuringOS loop 臂（完整生命周期，**进入内核**） | 裸 flash control 臂（**从不进入内核**，无 tape） |
|---|---|---|
| 入口 | **`init`/genesis 全流程**：full-flow binary `swebench_live_coding_repair_current_kernel`（src/bin/，调 `build_chaintape_sequencer_with_initial_q`，runtime/mod.rs:706）→ 签名 typed-tx → 内核循环 → **ChainTape + git tape（依宪法强制，见 §1.2/§5.4）** | 单次 API 调用，`scripts/probe_bare_v4_swebench.py`，无 genesis、无 Sequencer、无内核、**无任何 tape** |
| worker 模型 | `deepseek-v4-flash`，**thinking 显式 disabled**（D2；`{"thinking":{"type":"disabled"}}`） | `deepseek-v4-flash`，**thinking 显式 disabled**（同一） |
| meta 模型（仅 loop 臂有 meta 槽） | 见 §3.4：loop 内 meta 角色为轻量/确定性谓词路由 + patch 生成 → **强 flash 足矣**；若实测 patch 生成受益于 CoT 才升 thinking（per-role 可配，cmd_tdma.rs:445） | 不适用（control 臂无内核、无角色） |
| 采样 | temperature/top_p 锁定一致（见 §10 校正）⚠️ 注意 findings：thinking 模式下 temperature/top_p/penalty 被忽略——本 campaign thinking=disabled，采样参数生效 | 同一 |
| 对模型暴露的上下文 | `SWEBENCH_SYSTEM_PROMPT` (cmd_tdma.rs:252) — 含 `{"schema_version":"tdma-state-update/v1",...}` 首行 header + `---BODY---` 契约（承重，见 §9 坑 2）；`make_swebench_user_prompt` (cmd_tdma.rs:259) | **消融校正后**与 loop 臂 sha256 一致（当前不一致，见 §10） |
| tape（externalized CoT substrate） | **强制**：每次 attempt 无条件 commit `TapeNode`（`step_forward`，memory_kernel.rs:162-207；成功 `StateAccepted`，拒绝 `AgentProposal`+`RetryBeliefState`，handle_rejection memory_kernel.rs:211）；full-flow binary 经 Sequencer 的 `Git2LedgerWriter` L4 ledger 写**宪法 ChainTape 本体**（runtime/mod.rs genesis 路径），即依 Art. 0.4 Phase E 强制的 Path B git substrate | **无 tape**（裸模型从不进入内核；无 ChainTape、无 git tape） |
| 重试反馈（externalized CoT 回流） | `assemble_o1_prompt`（memory_kernel.rs:377-422）：CharterCore + `[OUTPUT CONTRACT]` + `[AUTHORITATIVE SESSION DIGEST]`(SessionDigest) + **`[RETRY BELIEF STATE]`**（RetryBeliefState：failure_signature = reject_class + failed_predicate + zero_gain_streak，累积 constraints "avoid X:Y at attempt N"）+ **`[EVIDENCE POINTERS]` 仅两个 hash**（`raw_failure_evidence_node` / `belief_state_node`；raw stderr/candidate body 永不入 prompt，KILL-tdma-1）+ `[CURRENT TASK]`。⚠️ **当前 tape 回流对 SWE-bench 实质仅携带失败测试名级别信号**（failed_predicate = 失败测试名 ≤200 char 一行；见 §3.5：D2 论题需富化） | 无（单发） |
| 网络路径 | 经 `chat_complete_blocking` (cmd_tdma.rs:586)，`thinking_for_llm` 透传 (cmd_tdma.rs:583/591)，由 `TURINGOS_SILICONFLOW_ENDPOINT` 覆写指向代理（probe_swebench_loop.sh:16-17）；**须含 DSML-leak 防御 + thinking=disabled 强制**（§5.5） | Python urllib 直连代理；同样须 DSML-leak 防御 + thinking=disabled |
| Verifier | 官方 `swebench.harness.run_evaluation`，Docker，HF-offline，**100% 确定性无 LLM**（swebench_test_judge.rs:159，读 report.json `resolved`，:288）（同一） | 同一 |
| 实例集 | gold-gated 子集（同一） | 同一 |

**被测变量 = 「是否经过完整 TuringOS 生命周期（genesis → 内核 → tape 循环）」整体一项。** 其余对模型暴露面必须强制相等（§10 列出当前必须修掉的不对称）。

### 2.2 市场坐标表（SECONDARY — 受混淆，仅作方位）

> ⚠️ **混淆声明（必须随表展示）**：下表每一个分数都是 **(模型 × 脚手架)** 的乘积，不是脚手架质量或模型能力的单独度量。这两个变量在任何已发布数字里都不可分离。本表仅用于建立量级方位，不构成能力排名。所有分数为各自 pass@1 口径，时间点不同（排行榜逐周变动，<5pp 差异普遍落在 CI 重叠区）。
> ⚠️ **flash 口径附注**：我们的 worker 是 **flash + thinking 显式 disabled**；下表 V4 数字几乎都是 thinking-enabled（默认）口径，**口径不同，不可直接对标**（见 §3.5）。

| 平台 / 脚手架 | Verified 分数 | 模型 | 脚手架 | 来源（一级优先） | 混淆标记 |
|---|---|---|---|---|---|
| **DeepSeek V4-Pro（厂商自报）** | **80.6%**（很可能 Think-High/Max 口径） | DeepSeek-V4-Pro（1.6T total / 49B active） | DeepSeek 内部 harness（未公开命名，非 OpenHands/SWE-agent） | V4 preview release news260424；BenchLM 镜像 | 🔴🔴 模型+脚手架；**且未经独立复现** |
| **DeepSeek V4-Flash（厂商自报）** | **79.0%**（很可能 thinking-enabled 默认口径） | DeepSeek-V4-Flash（284B total / 13B active） | DeepSeek 内部 harness | V4 preview release；vals.ai / benchlm.ai 镜像 | 🔴🔴 模型+脚手架；**未独立复现；且非 disabled 口径**（我们用 disabled，AA Index 36 vs Think-Max 47，预期 non-thinking 明显更低，无公开直测值） |
| **Claude Opus 4.7（厂商自报）** | ~80.8%（参照系顶部） | Claude Opus 4.7 | Anthropic 内部 | vals.ai 镜像 | 🔴🔴 模型+脚手架（量级参照） |
| **GPT-5.5（厂商自报）** | ~74.9% | GPT-5.5 | OpenAI 内部 | vals.ai 镜像 | 🔴🔴 模型+脚手架 |
| **OpenHands** | 66.4%（5-trajectory + critic）；单发 60.6% | Claude 4（出厂 70.4%）；critic=Qwen2.5-Coder-32B | CodeAct + 推理时扩展 | OpenHands 官方博客 2025-04-17 | 🔴🔴 模型+脚手架 |
| **Amazon Q Developer** | 66%（2025-04-21）；38.8%（2024-09） | AWS 专有（未披露） | AWS 专有 agent | AWS What's New 2025-04-21 | 🔴🔴 模型+脚手架 |
| **Anthropic（Claude Code 脚手架）** | 3.7 Sonnet 63.7% 原味 / 70.3% high-compute；Opus 4.x 更高 | Claude 3.7 Sonnet / Opus 4.x | Bash + Edit + 规划 prompt | Anthropic Engineering 2025-02-24 等 | 🔴🔴 模型+脚手架 |
| **CodeMonkeys（学术参照，flash+loop 先例）** | 57.4%（coverage 69.8%）；vs o3 71.7%，差 14.3pp | Claude 3.5 Sonnet（**non-thinking**）+ 外部迭代 test+edit 循环 | 测试执行 + 并行采样 + 选择 | Stanford 2025-01；arXiv:2501.14723 | 🟡 **最贴近本设计的「flash+外化循环 vs thinking 模型」公开数据点**（§3.5 引用） |
| **SWE-agent 1.0** | SoTA 声明（2025-02-25），**精确 % 一级源未给出** | Claude 3.7 Sonnet | ACI（专用 shell 接口） | GitHub (Princeton NLP) | 🔴🔴 模型+脚手架；**精确数低置信，勿引用具体值** |
| **Devin / Cognition** | 原版 13.86%（full，非 Verified）；Devin 2.0 ~45.8%（**仅第三方**） | 专有（未披露） | 专有单 agent | Cognition 2024-03 技术报告（仅覆盖原版） | 🔴🔴 模型+脚手架；2.0 数无一级源 |

**关于「OpenClaw / Hermes」的诚实更正**（任务要求点名澄清，PRESERVE）：

- **OpenClaw 不是编码 agent 平台**。它是本地优先通用自动化 agent 框架（连 WhatsApp/Telegram/Discord 到任意 LLM），2026-02 破 10 万 GitHub star。它**没有平台级 SWE-bench Verified 分数**；它自己的基准是 **PinchBench**（自动化任务，与 Verified 不可比）。坊间流传的「OpenClaw 69.6% Verified」其实是**跑在它上面的某模型自己的分**，不是平台分。→ **本表不收录 OpenClaw 的 Verified 行**；若需要「最接近的真实开源编码平台」，替代物是 **OpenHands** 或 **SWE-agent**（已入表）。
- **Hermes（Nous Research）也不是 Verified 选手**。两个同名物：(1) Hermes 系列微调 LLM 权重；(2) 2026-02 发布的 Hermes Agent 框架（用 GEPA 自改进，基准是 Terminal-Bench 2.0 / TBLite / YC-Bench，**均非 SWE-bench**）。Nous 明确表示「不为基准优化」。坊间「Hermes ... Verified」来自第三方测评博客，**官方排行榜无此条目**。→ **本表不收录 Hermes 行**。

**置信度标注**：OpenHands 66.4%、Amazon Q 66%、Anthropic 系列、CodeMonkeys 57.4%、Devin 原版 13.86% — 高（一级源）。SWE-agent 精确 % — 中（SoTA 声明确认但数不可提取）。Devin 2.0 45.8% — 中低（仅第三方）。V4 Pro 80.6% / Flash 79.0% / Opus 4.7 ~80.8% / GPT-5.5 ~74.9% — 中高（量级可信，**独立复现状态低置信，V4 数非 disabled 口径**）。任何来自 llm-stats.com / benchlm.ai / awesomeagents.ai 的聚合数字 — 低，不作一级引用。

---

## 3. 为什么这样设计

### 3.1 model+scaffold 双重混淆（为何不能直接对标排行榜）

每一个已发布 Verified 分数都同时绑定一个特定模型和一个特定脚手架，二者在任何公开数字里不可分离（脚手架维度包括：控制架构、工具集、上下文压缩策略、迭代/成本上限、prompt、默认模型/温度、记忆机制——每一维都独立影响分数）。实证（externalizedCotPrecedent findings）：同一 Claude 3.7 Sonnet 原味 62.3% vs 自定义脚手架 70.3%（+8pp 纯脚手架差）。因此若 TuringOS + flash 得 X%，**既不能**把 X 归功于 flash（脚手架有贡献），**也不能**把 X 当作 TuringOS 脚手架质量证明（flash 原始能力有贡献）。

→ **结论**：直接头对头是假科学。唯一干净的设计是**在内部把 worker 模型钉死为同一 flash，只动「是否经过完整 TuringOS 生命周期」这一根轴**（=全系统消融，归因边界见 §1.2），把外部平台数字降级为「坐标」。

### 3.2 没有「假 git tape」——git-tape 是每次内核运行的宪法法律（D1）

**这是必须订正的措辞**。TuringOS 内核路径里**不存在**任何「假 / mock / 降级」git tape。loop-vs-control 的区分**不是**「真 tape vs 假 tape」，而是：

- **loop 臂 = 进入 TuringOS 内核**——内核**依法**发出 ChainTape（`TapeNode` commit 序列，经 `ImmutableTapeLedger::commit`）；full-flow binary 经 Sequencer 的 `Git2LedgerWriter` 写持久 git substrate（宪法 ChainTape 本体，Path B）。
- **control 臂 = 裸 flash 模型，从不进入内核**——因此**完全没有 tape**（无 ChainTape、无 git tape）。

**宪法法律引用（已核实，对照 constitution.md 抽样复核）**：

- **Art. 0.2 Tape Canonical 公理**（constitution.md:54）：**「所有信号必须可从 tape 重建。」** 原文续：「这是 Turing 1948 定义的直接逻辑后承：如果 paper 不携带产生当前状态所需的全部信号，那 'a person with paper' 就不能复算到那个状态——universal machine 性质破产。」配套不变量 `derive_latest_belief_state_from_tape` **必须**是纯函数、只读 tape（ledger.rs:564，Gate 5 invariant，Art. 0.2 tape canonicity）——「so replay can reconstruct BBS from tape alone」。
- **Art. 0.4**（constitution.md:149）：**「Phase E gate 强制 B」**（libgit2 真实 git substrate / Path B），「除非 Phase E 之前用户 sudo 修宪降低 fidelity 要求」。Phase E 下用 `--tape-backend=memory` 而非 git，会**违反 Art. 0.4 Path B gate**（即便 in-memory 的 `MemoryTapeLedger` commit 也是真实的——区别是**持久化** git bare-repo vs in-process memory，而非 tape 完整性）。

**关于 `MemoryTapeLedger` 的精确订正（findings，避免再生「假 tape」误解）**：`run_proof()`（无 ledger 形参，tdma_runner.rs:495–503）默认用 `MemoryTapeLedger`，这**不是**假 tape——它是 `ImmutableTapeLedger` 的全功能 in-memory 实现，commit 完全相同的 `TapeNode` 结构，只是不持久化到 git bare-repo。`"mock-model"` 仅出现在 `#[cfg(test)]` 单元测试（tdma_runner.rs:896/937），**非生产旁路**。

**「无 tape 的内核运行」在架构上不可能、且违宪**：`step_forward` commit 是无条件的（tdma_runner.rs:644 主循环体；`step_forward_with_workspace` memory_kernel.rs:162–207 每 attempt 至少 commit 一个 `TapeNode`，pass 发 `StateAccepted`，fail 发 `AgentProposal`+`RetryBeliefState`，handle_rejection memory_kernel.rs:211）。一次产出零 tape commit 的内核运行**给定当前代码不可能发生**，且会违反 Art. 0.2。

→ **设计含义**：本设计 loop 臂的 tape 不是「证据装饰」，而是**externalized chain-of-thought 的物理载体**（D2 论题的依托）。control 臂的「无 tape」正是它「无外化推理」的物理体现。两臂差 = 整个「进入内核 + 外化 CoT」机制（§1.2 归因边界）。

### 3.3 为何必须从 init/genesis 全流程进入，而非 `tdma run` 快捷方式（D4）

**`tdma run` 完全旁路 genesis/init**（findings 核实，cmd_tdma.rs:603-624）：它**不**调 `verify_trust_root`、**不**调 `build_chaintape_sequencer_with_initial_q`、**不**初始化 Sequencer、**不**写 ChainTape L4/L4.E、**不**开 CasStore、**不**生成 `pinned_pubkeys.json` / `initial_q_state.json`。它只 boot 一个 `MemoryKernel`（tdma_runner.rs:541）——一个 TDMA-bounded in-memory 状态机，**不是**宪法 Sequencer（不用 Ed25519 签名、不写 `Git2LedgerWriter`、不写 `CasStore`、不开 `rejections.jsonl`、不产 `genesis_report.json`）。

**注意厘清两类 git tape**：即便 `tdma run --tape-backend=git` 也只产 `<workspace>/tdma_tape.git` 的 `GitTapeLedger`，那是 TDMA 框架自己的 tape，**不是** Sequencer 经 `Git2LedgerWriter` 写的宪法 ChainTape L4 本体。本 campaign loop 臂走 full-flow binary，其 tape 是后者（宪法本体）。

**`tdma run` 快捷方式跳过了什么（findings 核实）**：

1. **Trust Root 完整性校验**（`verify_trust_root`，boot.rs:97；43 文件 SHA-256 manifest + `[constitution_root]` 8 键 + `{cases,rules}/MANIFEST.sha256` 递归）——注意此校验仅在 `turingosv4` library 主 binary（src/main.rs:14）跑，CLI 与所有 sub-binary 均不调（见 §3.3.1 诚实附注）。
2. **加密 keypair 生成 + `pinned_pubkeys.json`**（`Ed25519Keypair::generate_with_secure_entropy`，SystemEpoch(1)，runtime/mod.rs:782/786）。
3. **QState 初始化 + `initial_q_state.json` 持久化**（`QState::genesis()` = `QState::default()` 全零空态，q_state.rs:958；写盘 runtime/mod.rs:831）。
4. **宪法 Sequencer**（typed-tx admission、predicate registry、tool registry、L4 ledger、L4.E rejection ledger；`activate_predicate_binding_for_boot` + `activate_map_reduce_tick_for_boot`，runtime/mod.rs:872–876）+ `RejectionEvidenceWriter::open_jsonl`（rejections.jsonl，fail-closed，:809）+ spawn `run_chaintape_driver` Tokio task（:883）。
5. **全部 typed transactions**（`TaskOpenTx`、`EscrowLockTx`、`WorkTx`，均带 Ed25519 签名，经 `submit_agent_tx` → `apply_one` → `Git2LedgerWriter.append`，await state_root 推进）。
6. **CasStore**（issue capsule、patch claim capsule、evaluation capsule、ProposalTelemetry——WorkTx 须引用 proposal_telemetry_cid）。
7. **`genesis_report.json`**（constitution_hash、system_pubkey_hash、initial_balances、task_id）。
8. **多方经济态**（agent balances、escrow、stakes，QState.economic_state_t）。

→ **D4 的核心论点**：手工配置 init 使流程**不完整**，测不出 TuringOS 真实的端到端 path-to-AGI 能力。**全流程入口是 `swebench_live_coding_repair_current_kernel` binary**（src/bin/；已核实存在，30KB，调 `build_chaintape_sequencer_with_initial_q`（:452）、`resume_existing_chain: false`（:450）、提交签名 `TaskOpenTx`（:478-484）/`EscrowLockTx`（:495-501）/`WorkTx`（:514-525）并 await state_root 推进，写 `genesis_report.json`（:564））。本设计 loop 臂**必须经此 binary 进入**，不得用 `tdma run --judge swebench`。

**完整生命周期 stage（findings 核实，供 Atom 0/3 落地）**：

| Stage | 内容 | seam |
|---|---|---|
| 0 | Trust Root 校验（仅 library binary，见 §3.3.1） | boot.rs:97（src/main.rs:14） |
| 1 | workspace/sample 准备 → 验证 sample JSON → 写 SweBenchIssueCapsule 到 CAS | swebench_live binary main → parse_args → run() |
| 2 | LLM proposal（经 `ResilientLLMClient::generate`，drivers/llm_http.rs；算 provider_response_sha256） | — |
| 3 | patch 解析 + 结构评估（rationale_guard / has_unified_diff / target_file_overlap）→ 写 PatchClaim/Evaluation capsule | parse_patch_claim() |
| 4 | ProposalTelemetry 写 CAS（返回 proposal_telemetry_cid） | runtime::proposal_telemetry |
| 5 | **QState genesis + ChainTape bootstrap** | **build_chaintape_sequencer_with_initial_q，runtime/mod.rs:706** |
| 6 | agent keypair 注册 → seq.set_agent_pubkeys | AgentKeypairRegistry::open() |
| 7 | ChainTape 上 TaskOpenTx（L4，await state_root） | make_real_task_open_signed_by → submit_agent_tx |
| 8 | ChainTape 上 EscrowLockTx（L4） | make_real_escrow_lock_signed_by |
| 9 | ChainTape 上 WorkTx（L4，引用 proposal_telemetry_cid） | make_real_worktx_signed_by |
| 10 | shutdown（drain queue）+ GenesisReport | bundle.shutdown() → GenesisReport::write_to_runtime_repo() |
| 11 | evidence manifest 写出（failure_taxonomy + SweBenchEvidenceManifest，含 CIDs + tx_id + landed） | SweBenchEvidenceManifest |

#### 3.3.1 诚实附注：Trust Root 校验的真实触发面（不可夸大）

findings 核实：`verify_trust_root` **只在** `turingosv4` library 主 binary（src/main.rs:14）启动时跑，**CLI（src/bin/turingos.rs）与所有 sub-binary（含 `swebench_live_coding_repair_current_kernel`）都不调用它**。因此「全流程入口」严格而言提供的是 **stage 1-11（genesis + Sequencer + typed-tx + CAS + GenesisReport）**，但**不**自动包含 stage 0 的 trust-root 校验。

→ **诚实结论**：相对 `tdma run`，full-flow binary 真正补回的是「宪法 genesis + 签名 typed-tx + CAS 经济态」这一**端到端内核生命周期**，这正是 D4 要测的 path-to-AGI 主体。若要把 stage 0 也纳入，需在 runner 前显式调用 `verify_trust_root`（Atom 0 可选守卫，§7）。**本设计不夸大**：不声称 full-flow binary「自带 trust-root 校验」，只声称它走完 genesis→typed-tx→CAS 的真实内核路径。

#### 3.3.2 验证器组合缺口（本设计最关键的落地点 —— 必须显式弥合）

⚠️ **核实发现的硬缺口**：两个现存入口**没有一个**同时满足「完整 genesis」+「真 Docker 隐藏测试」：

| 入口 | 完整 genesis | 真 SWE-bench Docker 隐藏测试 |
|---|---|---|
| `turingos tdma run --judge swebench` | ❌ 旁路（只 MemoryKernel） | ✅ 真跑（`SwebenchTestJudge::verdict`，swebench_test_judge.rs:159 → `run_evaluation` → report.json `resolved` :288）|
| `swebench_live_coding_repair_current_kernel`（full-flow binary） | ✅ 完整（§3.3） | ❌ **仅结构性 verdict**：`patch_structurally_plausible`（binary:358-365）、`benchmark_verdict="repair_patch_structurally_plausible"`；binary 头注释自述「**The structural patch verdict is a capability signal only; the liveness proof is the replayable ChainTape/CAS path**」（binary:7-8）|

**后果（若不弥合）**：照 D4 让 loop 臂走 full-flow binary，则 loop 臂被「patch 结构上是否像样」打分，裸臂被真 Docker 隐藏测试打分——**两臂打分口径不同，对比直接作废**。这是本设计**头号效度威胁**，比 prompt 不对称（§10）更严重。

**必须弥合**：把真 `SwebenchTestJudge`（Docker 真测）接进 full-flow binary，使**两臂都由同一 Docker `resolved` 口径打分**（scoring parity）。full-flow binary 原本是 **liveness 证明**（宪法机器端到端活着），benchmark 需要它**同时**产出**真 capability verdict**（Docker resolve）。落地见 **Atom 0.5（§7）**。

⚠️ **分类问题（必须先裁定，关乎 Class 2 vs Class 4）**：full-flow binary 把 WorkTx 提交**条件在 `patch_structurally_plausible`**（binary:522 `let after_work = if patch_structurally_plausible`）。若把此门改成「真 Docker resolved」：
- 若改动**只在 binary 本地逻辑**（Sequencer 仍只校验签名+well-formed）→ **Class 2**（benchmark harness wire-up）。
- 若 swebench-resolve 进入 **Sequencer 的 admission predicate**（`apply_one` 经 predicate registry，V2 核实 Sequencer 确有 registry）→ 触及 **sequencer admission（§6 受限面）= Class 4**，须 per-atom §8 架构师批准。

→ **最保守安全的弥合**：**不动** WorkTx 的结构性门（保住 binary liveness 语义），而在 binary 跑完 genesis+typed-tx 后**额外**调一次 `SwebenchTestJudge::verdict` 对同一 patch 做真 Docker 打分，`resolved` 写进 evidence manifest 作 benchmark 口径——既保宪法机器 liveness，又拿到与裸臂同口径的真 resolve，且大概率落 Class 2。**本设计不预判分类**，Atom 0.5 第一步先核实门在哪。

### 3.4 meta 模型的具体建议（D3 —— 据 meta 真实角色）

**findings 核实的 meta 真实角色（关键）**：

- TDMA loop 里**只有一个 agent**：`--role`（默认 `meta`，cmd_tdma.rs:306）选中模型槽，驱动单一 `llm_call` 闭包（cmd_tdma.rs:445）。**META 不是独立 overseer**，它**就是**生成 patch/step 的那个 agent；每 attempt 仅一次 LLM 调用（`llm_call(&system_prompt, &user_prompt)`，tdma_runner.rs:594）。
- swebench predicate **100% 确定性、无任何 LLM 调用**：`SwebenchTestJudge::verdict`（swebench_test_judge.rs:159）shell 出官方 Docker harness `python -m swebench.harness.run_evaluation`，读 `report.json` 的 `resolved`（:288）。fail reason 仅携带测试名 + patch-apply 错误（屏蔽）。
- **所有 retry/rejection 逻辑是确定性 Rust**：`step_forward` 决定 Proceed/Retry/Escalate；distiller 压缩 BBS constraints（`compress_belief_state`，Jaccard-dedup）；下一轮 prompt 由 Rust 字符串格式化（tdma_runner.rs:582-593）。**没有**第二个 meta-level LLM 调用做分解、retry 决策或反馈压缩。
- 默认模型常量：`DEFAULT_META_MODEL = deepseek-ai/DeepSeek-V3.2`（chat_client.rs:25）；`DEFAULT_BLACKBOX_MODEL = Qwen/Qwen3-Coder-30B-A3B-Instruct`（chat_client.rs:29）。架构意图上 `blackbox` 被描述为「fast/codegen」worker 槽。

→ **D3 具体建议**：在 swebench loop 里，meta 的「角色」仅是**轻量/确定性谓词路由 + patch 生成**——没有任何 meta-level 的分解/reward-shaping/loop-control 推理需要 thinking（这些全是确定性 Rust）。**因此强 flash 模型对 meta 足矣**（thinking-off）。thinking 只在 patch 生成任务本身受益于 extended CoT 时才有真实价值（如复杂多文件 diff）；对简单 bug 或在意 latency/cost 时，flash thinking-off 架构上充分。

- **本设计默认**：loop 臂 meta 槽也用 **flash thinking-disabled**（与 worker 同），保持「全系统 = flash + 外化 CoT」论题纯净。
- **保留升级口**：若 G1 门上实测某类复杂实例的 patch 生成明显受益于 CoT，可**仅对 meta** 升 thinking-on（per-role 可配，见下）——但这会引入「meta 内部 thinking」混淆，需在报告显式声明并视作 §1.4-B 的变体消融，不混入主 Δ；且届时须落实 `reasoning_content` round-trip 防御（§5.5）。

**thinking per-role 可配，且已核实未硬编码（D3 要求 flag）**：

- ✅ **当前 HEAD `1f00012d` thinking 未硬编码**（findings 核实，**订正前轮「`thinking: None` 硬编码」探针结论**——该结论已被超越）。cmd_tdma.rs:445 按 `--role` 读 workspace TOML 的 thinking（`read_meta_thinking` cmd_llm.rs:508 / `read_blackbox_thinking` cmd_llm.rs:527），经 `thinking_for_llm` 透传到 `chat_complete_blocking`（cmd_tdma.rs:583/591）。
- `read_*_thinking` 仅在 TOML key 缺失时返 `None`（默认 off）；当 `llm.*.thinking = "off"` 时发显式 `ThinkingConfig{kind:"disabled"}` 防 provider 端默认重新开启（cmd_llm.rs:516-518）。src/ 内唯一的 `thinking: None` 是 chat_client.rs:43 的字段语义注释，**非硬编码代码**。
- meta 与 worker 可用**不同模型 + 不同 thinking**（各有 `llm.meta.*` / `llm.blackbox.*` stanza，含独立 `api_key_env`，cmd_llm.rs:426-435）。FULL_HELP 的 DEEPSEEK DUAL-KEY 示例（cmd_llm.rs:146-153）明示 `--meta-model deepseek-v4-pro --blackbox-model deepseek-v4-flash --meta-thinking on --blackbox-thinking off`——本设计据 D2/D3 取 **两者皆 flash + disabled** 为默认。

### 3.5 externalized-CoT 论题的张力与必要订正（D2 的诚实后果）

D2 把 worker 设为 flash 后，findings 揭示两处**承重张力**，必须明示并据以订正设计：

**张力 1 —— flash worker 在 diff/format 算术上更弱 → Atom 2 更承重，而非更轻**。

- findings（v4Flash.nativeEditFormat，置信中）：**无任何一级源**记录 V4 Flash 可靠产出的 diff 格式。已记录的是：non-thinking 模式 AA Intelligence Index 仅 36（vs Think-Max 47）；长视野 agentic 任务 Flash 明显低于 Pro；non-thinking 移除处理多步 hunk 算术的内部 CoT。unified diff 行号算术对 off-by-one 敏感，thinking 模式倾向自纠。**informed inference**：无 thinking 时 flash **更可能**退化成 search/replace 块或整文件重写，而非正确 unified diff。**apply-gate 风险真实存在**。
- → **设计订正**：**Atom 2（确定性 diff 合成——把 format 从模型手里拿走）变得更加承重（more load-bearing），不是更轻**。把行号算术彻底从 flash 手里拿走（search/replace → 确定性 diff 合成）正是对冲 flash format 弱点的核心机制。同时**保留 raw unified-diff 直通作一等回退**，Go/No-Go 经验定 V4 Flash 上哪条 apply 率更高（§3.6、Atom 2）。findings 进一步给出根治选项：把编辑包成**结构化工具调用**（`str_replace_editor` / `write_file`），让 apply 由结构化 tool call 中介而非解析模型生成的 diff 文本——这绕开 malformed-diff 失败。

**张力 2 —— 「tape = 外化 CoT」要在 flash worker 上成立，tape 必须携带真实推理 substrate，而非只有失败测试名**。

- findings（externalizedCotPrecedent，置信中高）：外化循环对 flash **有效但有条件**——结构化可验证任务（HumanEval/LiveCodeBench/MBPP）闭合 60-80% 差距（S\*：GPT-4o-mini 40.9%→61.3%，超 o1-preview；Reflexion：GPT-4 80.1%→91.0%）；**但真实 SWE-bench 仍有约 14pp 持久差**（CodeMonkeys Claude 3.5 Sonnet 57.4% vs o3 71.7%）。**关键 nuance（findings 原文）**：tape 作为**纯 action log**（「测试 X FAILED」/「ran test, got error」）**不**替代内部 thinking；tape 作为**结构化 deliberation 史**（hypothesis → evidence → revision → new hypothesis）**部分**替代。trajectory 分析：thought-action misalignment（成功轨迹 ~1% vs 失败 3.5%）强预测失败；「test X failed because 边界条件在 line 47 未处理空输入」才使 repair 可能。
- ⚠️ **findings 核实当前 tape 回流的实情**：retry 上下文（`assemble_o1_prompt`，memory_kernel.rs:377-422）携带 `RetryBeliefState`（failure_signature = reject_class + **failed_predicate** + zero_gain_streak + 累积 constraints "avoid X:Y at attempt N"），**raw stderr 与 candidate body 永不入 prompt**（仅 `[EVIDENCE POINTERS]` 两个 hash 指针，KILL-tdma-1）。即对 SWE-bench，failed_predicate 实质是**失败测试名（≤200 char 一行，经 `swebench_failed_predicate`→`make_judge_stderr`→`deterministic_trace_slicer`）**——worker **看不到自己先前的 raw rationale/body**，只看到从中抽取的 failed_predicate + 累积 constraint 规则。先前 body 虽 commit 入 tape（`AgentProposal.payload.raw_output`），但**仅其 hash 指针**会进下一轮 prompt。
- → **设计订正（D2 论题成立的必要条件）**：为使「tape = 外化 CoT」在 flash worker 上**真实成立**，retry/loop 上下文必须在屏蔽规则内携带**真实推理 substrate**（先前 rationale、apply error、attempt 史），**而非仅失败测试名**。这是一项**设计 refinement**，记为 **Atom 5（tape 富化，在屏蔽红线内）**（§7）。**诚实标注**：当前 tape 偏 thin（失败测试名级），D2 论题在富化前只能**部分**成立；富化是让论题为真的前置。富化必须严守屏蔽红线（§5.3）：可加「模型自己的先前 rationale 摘要 / 模型自己的 apply error / attempt 序号与 zero-gain streak」，**绝不可**加 gold/test diff、expected-vs-actual、隐藏测试源。
- ⚠️ **contextual drag 风险（findings）**：把失败的先前尝试喂回弱模型可能**主动恶化**性能（10-20% 跌幅）；自精炼可「坍缩成 self-deterioration」。同时 findings：**76–95% 的外化循环收益落在前 2 轮**，第 3 轮起边际极小（gain function 凹且紧）——不能靠加迭代线性补偿弱模型。→ Atom 5 富化须受 distiller 的 Jaccard-dedup 压缩约束（`compress_belief_state`，distiller.rs）、并保 O(1) token 预算，**避免**把原始失败正文堆进上下文。富化的是**结构化 rationale**，不是 raw 失败堆叠。
- ⚠️ **localization 是循环前瓶颈（findings，承重诚实项）**：弱模型若起手定位错文件，**任何测试反馈都纠不回**（测试只确认「补丁应用在对的位置」，不确认「找对了位置」）。Agentless 多类型文件定位 F1 仅 0.142。这是 tape 无法修复的**pre-loop**问题——意味着即便 Atom 5 富化到位，flash worker 在需多文件定位的实例上仍可能受限。这是我们对 Δ 量级保持谦逊的实证理由。

### 3.6 DeepSeek V4 Flash 的真实工程坑（findings 核实，承重）

前轮 n=3 实验在 flask 三元组两臂全 0/3，主导失败是 **patch-apply 阶段**。findings 给出 flash 特有的多个根因，**0% 几乎确定是脚手架/集成问题，不是模型差**：

1. **DSML 泄漏（高置信，多独立一级源）**：V4 原生工具格式是 DSML（DeepSeek Markup Language），内部生成以 `｜DSML｜` 特殊 token 分隔的 XML 块：`<｜DSML｜tool_calls><｜DSML｜invoke name="..."><｜DSML｜parameter name="..." string="true|false">...</｜DSML｜parameter></｜DSML｜invoke></｜DSML｜tool_calls>`（`string="true|false"` 属性区分 raw string 与 JSON 编码参数，V4 引入以消除 V3 的 JSON 转义失败）。OpenAI 兼容包装层**确证会失败**：(a) 间歇全泄漏（DSML markup 进 `content`，`tool_calls: null`，`finish_reason: "stop"`；deepseek-ai/DeepSeek-V3#1244、openclaw#85918、NVIDIA forum）；(b) partial marker 泄漏（缺 `｜DSML｜` wrapper，留裸 `<function_c...>`；sglang#14695）。第三方 wrapper（NVIDIA NIM / Azure Foundry）可能**100% 失败**（非 ~11-21% 间歇），足以单独造成 0% apply。NIM 还伴随**严重虚高 token 计数**副作用。
2. **flash 默认 thinking-ON + reasoning_content 多轮 400（高置信）**：见 §1.3。未显式 disabled → flash 默认 thinking → 产 `reasoning_content` → 多数 OpenAI 兼容 SDK/proxy 剥离该字段 → 下一轮 400「The reasoning_content in the thinking mode must be passed back to the API」→ **每条多轮 trajectory 在产 patch 前夭折**（n8n#29661、opencode#24124/24130、KiloCode#9501、claude-code-router#1378）。`reasoning_effort:"none"` 单独**不足**——只有 `{"thinking":{"type":"disabled"}}` 可靠消除（LiteLLM#27453 Test 3）。这是 flash 用作 worker 时 0% apply 的又一可信主因。
3. **DeepSeek 原生偏好 unified diff、抗拒 search/replace**：被强制进 search/replace 会退化整文件重写，严格 apply gate 命中 0 SEARCH 块。
4. **malformed diff（中高置信）**：non-thinking flash 的 hunk 行号算术 off-by-one（§3.5 张力 1）。
5. **finish_reason=length 中途截断**：未设显式 `max_tokens` → mid-JSON 截断产 malformed 参数，外观类似 DSML-leak 失败。

→ **设计含义（Atom 2 + §5.5 集成防御）**：好脚手架必须 (a) 接受 unified diff 并用标准 patch 应用，或把编辑包成结构化工具调用绕开 diff 文本解析；(b) **client-side DSML-leak 检测 + 恢复**（扫 `finish_reason=="stop"` 且 `tool_calls` null 时 `content` 内的 `<｜DSML｜tool_calls>` / `<｜DSML｜invoke`，手动从 `name` 属性重建 tool call，参照 openclaw `openai-transport-stream.ts`；并防 partial-marker 裸 `<function_c` 形）；(c) **强制 `{"thinking":{"type":"disabled"}}`**（worker，D2）；(d) strict-mode tool schema（`"strict": true` + `additionalProperties:false` + 全属性入 `required`）+ 显式 `max_tokens` 防截断。本设计主路径采 **search/replace → 确定性 diff 合成**（Atom 2，把行号从模型手里拿走），**保留 raw unified-diff 直通作回退**，Go/No-Go 门在 flask 三元组上经验校准哪条在 V4 Flash 上 apply 率更高。**这是需实测分离的开放问题（findings：无 agent log 不能定论 DSML-leak vs malformed-diff 谁主导，二者可能复合），不是先验定论。**

### 3.7 为何「自证 + 坐标」胜过假头对头

- **自证（全系统消融）是可辩护的**：钉死 flash worker → Δresolve 干净归因于「完整 TuringOS 架构 vs 裸 flash」（§1.2 边界）→ 配对检验给出统计可信度。
- **坐标是诚实的方位**：给出绝对量级 + cost-per-resolved 前沿（§3.5），不僭称等价。
- **假头对头会被审计否决**：把不同模型的分数差当脚手架优劣，违反 §1.6，clean-context 审计的 SECOND-SOURCE-DRIFT 域会标记。

---

## 4. 数据集与实例选择

### 4.1 数据集

- **`princeton-nlp/SWE-bench_Verified`**（500 实例，OpenAI 人工校验子集）。
- **Python-only**：明确声明结论不外推到 JS/Go/Rust/TS。

### 4.2 确定性 n=50 选择

1. **限定 hermetic 仓库白名单**（§4.4），从中取实例。
2. **确定性抽样**：按 `instance_id` 字典序排序后取定步长 / 固定种子，使任何 session 重跑得到**同一 50 个 id**。50 写死并记录在 manifest（无静默截断）。
3. 在 manifest 记录抽样算法、种子、最终 50 id 列表，供审计复现。

**统计现实检查**：n=50 的 Wilson 半宽在 p=0.5 时约 ±13.6pp（单臂），偏宽。但 PRIMARY 主张是**配对**的，由 discordant 对数 b+c 驱动，不由 n 驱动。McNemar 在 10pp 真差、n≈113 时才有 80% power；**n=50 对 10pp 配对差是欠功效的**。因此：
- n=50 是**第一阶段**（成本可控、验证管线、产出诚实初步 Δ + 宽 CI）。
- 文档**明确承认 n=50 对小差欠功效**；若 Δ 不显著，结论写「在 n=50 上未检出显著差异（功效有限）」，**不**写「无差异」。
- 提供向 hermetic 全集 n≈390+（django+sympy+pylint+sphinx+pytest+xarray，gold-gate 后）扩展的路径（Atom 3 设计为可续跑批处理，直接支持后续放大）。

### 4.3 Gold-gating 程序

**定义**：跑任何 agent 前，先把官方 gold patch 作为 `--predictions_path gold` 喂进本地 offline Docker harness，**永久丢弃 gold 不能 RESOLVED 的实例**。只有存活实例构成诚实的 n。

程序：
1. `python -m swebench.harness.run_evaluation --predictions_path gold --dataset_name princeton-nlp/SWE-bench_Verified --split test`（sanitized+offline 环境，复用前轮 `fixtest_offline.log` 路径）。
2. 记录 `status != RESOLVED` 的 instance_id。
3. 生成排除这些 id 的过滤列表，**带日志**（无静默截断）。
4. 后续两臂只在过滤列表上跑。
5. 报告 `n_gold_valid / 50` 作为透明度数字。

**已知 gold-broken 实例**（必须预期并排除）：astropy-7606/8707/8872（NumPy 1.24+ 移除 np.int 等别名）、matplotlib-20488、django-10097；psf/requests 全家（httpbin 外部依赖，offline 503）。前轮已有 goldsmoke 证据：flask-5063 (~19s)、flask-4045、flask-4992 均 gold-resolve（`handover/evidence/swebench_loop_20260528/logs/goldsmoke_flask{5063,4045,4992}.log`）。

### 4.4 Hermetic 仓库白名单

| 仓库 | Verified 实例数 | hermetic 性 |
|---|---|---|
| django/django | 231 | ✅ 完全（SQLite in-process，无网络）— 最可靠 |
| sympy/sympy | 75 | ✅ 完全（纯 Python CAS）|
| sphinx-doc/sphinx | 44 | ✅ 完全（本地文件 fixture）|
| pytest-dev/pytest | 19 | ✅ 完全 |
| pydata/xarray | 22 | ✅ 完全 |
| pylint-dev/pylint | 10 | ✅ 完全 |
| pallets/flask | 1 | ✅（前轮三元组取自 Lite，非 Verified；Verified 仅 1 个）|
| matplotlib | 34 | ⚠️ 像素比对可能 flaky（无网络）|
| scikit-learn | 32 | ⚠️ hermetic 但慢 |
| astropy | 22 | ⚠️ 3 个 gold-broken，余下校正后 hermetic |
| **psf/requests** | 8 | ❌ **非 hermetic**（httpbin → 503 offline），**必须排除并记录** |
| mwaskom/seaborn | 2 | ❌ 下载数据集，排除 |

**本设计 n=50 抽样仅从 ✅ 列（django+sympy+sphinx+pytest+xarray+pylint = 401 实例）抽取**，gold-gate 后得诚实 n。前轮确认 hermetic 家族：flask/django/sympy/pylint。Go/No-Go 门仍用 **flask 三元组**（与前轮一致，复用已有 Docker 镜像 + goldsmoke 证据），因为它是最快的「管线活着」探针。

### 4.5 genesis 成本模型：genesis-per-task vs init-once-then-many（D4）

D4 要求从 genesis 进入，必须回答「每 task 一次 genesis 还是一次 init 喂多 task」的成本权衡（findings 核实）：

**genesis-per-task（当前 full-flow binary 的天然模型，本设计采用）**：

- full-flow binary 每次调用一次 `build_chaintape_sequencer_with_initial_q`（`resume_existing_chain: false`，:450），`BootstrapError::NonEmptyRuntimeRepo` fail-closed 门（runtime/mod.rs:731）强制每次拿全新 runtime_repo（若 runtime_repo 已有 refs/transitions/main 且 resume 关 → 拒）。
- **每 genesis 开销（findings 核实）**：写 4 个小文件（pinned_pubkeys.json / initial_q_state.json / rejections.jsonl / genesis_report.json）；一次 Ed25519 keypair（OS 熵，微秒级）；一次 `Git2LedgerWriter::open` 空 bare repo（git init 当量 ~1-5ms）；spawn 一个 Tokio 驱动 task（可忽略）。
- **50 实例 × 2 臂 = 100 的 I/O 上界估算**（注：control 臂裸模型**不**进 genesis，故实际 genesis 次数 = loop 臂实例数；这里取 findings 的上界 100 估算 I/O）：~100 git init + ~400 文件写 + ~100 keypair ≈ **总计 <1 秒 I/O 开销**。存储主成本是 CAS payload 磁盘（capsule 典型 1-10KB）。
- **结论**：genesis 开销被 LLM latency（每 task 秒到分钟级）**完全淹没**，相比之下**实际为零**。genesis-per-task 是当前生产 pattern，**FEASIBLE**。

**init-once-then-feed-many（G1.1 RESUME 模式，本设计不采用，仅备注）**：

- `RuntimeChaintapeConfig.resume_existing_chain = true`（须 env `TURINGOS_CHAINTAPE_RESUME=1`，严格字符串等于，runtime/mod.rs:470-471）→ 读现有 pinned_pubkeys.json、`replay_full_transition` 全链重放、为新 epoch 生成新 keypair 追加 manifest、从重放重建 QState（`bootstrap_resume_state`，runtime/mod.rs:920）。
- **优**：跨 task 持久化 agent balances/reputation/PnL（BatchContinuationManifest 路径，runtime/mod.rs:165）。**劣**：tamper 检测每次须从 genesis 重放（O(chain_length)）；mid-batch 失败难隔离。50 task/臂时链长 150 L4 entry（每 task TaskOpen+EscrowLock+WorkTx），每次 resume 重放 ~毫秒级。
- **结论**：RESUME 适合需跨 task 状态持久的多 task 运行，但需 `TURINGOS_CHAINTAPE_RESUME=1` + BatchContinuationManifest 机制，**非当前 swebench binary 实现**。**本 campaign 取 genesis-per-task**（成本可忽略 + mid-batch 失败易隔离 + 与 full-flow binary 现状一致）。

→ **设计决策**：**genesis-per-task**。每个 loop 臂实例 = 一次完整 genesis（全新 runtime_repo + CAS）。这与 §1.2 的归因边界自洽：每实例都走完整生命周期，Δ 归因于「完整 TuringOS vs 裸 flash」。

---

## 5. 评测协议与屏蔽红线

### 5.1 官方 Docker 评测

- 三层 Docker 镜像（base → environment → instance），每任务全隔离容器。
- 执行经 `SwebenchTestJudge::verdict` (swebench_test_judge.rs:159) → `run_sanitized` (judge:236) shell 出 `python -m swebench.harness.run_evaluation` (judge:200)。**100% 确定性、无 LLM**（§3.4）。
- **HF 必须 offline**：`HF_HUB_OFFLINE=1` + `HF_DATASETS_OFFLINE=1`（judge:228-229，已硬编码），因为 sanitized runner 剥离代理变量。镜像须由 goldsmoke 预构建（构建需网络，hermetic env 阻断网络）。
- sanitized 环境白名单仅 `PATH, HOME, USER, LANG, DOCKER_HOST, TMPDIR`（judge:226）。

### 5.2 FAIL_TO_PASS / PASS_TO_PASS 与 resolve 定义

- **FAIL_TO_PASS**：base 快照上失败、patch 后须通过的测试（PR 中形式化修复的测试，推理时对模型隐藏）。
- **PASS_TO_PASS**：base 上通过、patch 后须仍通过的回归守卫。
- **RESOLVED（精确，grading.py "FULL"）= 全部 FAIL_TO_PASS 通过 AND 全部 PASS_TO_PASS 仍通过**。排行榜用二值 RESOLVED。
- **apply 三级回退**（harness 内部）：`git apply --verbose` → `git apply --verbose --reject` → `patch --batch --fuzz=5 -p1`；三者皆败 → EMPTY_PATCH/ERROR，无 report.json。
- **当前 judge 的已知缺口**：`verdict` 只读 `report.json` 的 `resolved` 字段（judge:288-289）判 Pass，失败时只抽 `FAIL_TO_PASS.failure` 测试名（judge:292-311）。它**不**单独校验 PASS_TO_PASS——但 `resolved` 字段本身已包含 PASS_TO_PASS 检查，故唯一缺口是 retry 反馈永不向模型暴露被破坏的 PASS_TO_PASS（仅暴露 FAIL_TO_PASS 名）。与官方口径一致，记录为已知设计选择。

### 5.3 屏蔽红线（哪个 struct/field 强制 —— 全部 PRESERVE）

| 红线 | 强制点（精确 seam）|
|---|---|
| `gold_patch` / `test_patch` / 隐藏测试修复后正文**永不**进任何 prompt | **`SwebenchSampleInput` (cmd_tdma.rs:37)** 按构造仅反序列化 6 字段：`instance_id, repo, base_commit, problem_statement, hints_text(Option,default), fail_to_pass(Vec,default)`。serde 忽略多余字段——磁盘 JSON 即便带 gold/test，prompt-building struct 也看不见。**禁止给 SwebenchSampleInput 加 gold/test 字段。** |
| 用户 prompt 仅暴露 repo/base_commit/problem/hints/失败测试名 | `make_swebench_user_prompt` (cmd_tdma.rs:259) 只读上述字段 |
| 重试反馈仅携带：失败测试**名**（FAIL_TO_PASS，已公开）或模型自身 apply error | judge 失败分支抽 `FAIL_TO_PASS.failure` 名（judge:292-311）；apply 失败时 `harness_failure_reason` (judge:321) 从 `run_instance.log` 抽模型自己的 patch 错误（关于模型自己的 patch，非 gold diff）；经 `swebench_failed_predicate` (tdma_runner.rs:439) 路由；`assemble_o1_prompt` (memory_kernel.rs:374) **RAW STDERR 永不入 prompt，仅 evidence_hash/bbs_hash 指针** |
| 永不携带 expected vs actual 的 diff，永不携带隐藏测试源 | 反馈构造在 `make_judge_stderr` (tdma_runner.rs:444/448)；full blob（含 candidate body）入 tape 为 `AgentProposal.payload.raw_output` 但 distiller 压缩后才回流 prompt（仅 hash 指针入下一轮） |
| **Atom 5 tape 富化的屏蔽护栏（新增，承重）** | 富化**只可**加：模型自己的先前 rationale 摘要 / 模型自己的 apply error / attempt 序号 + zero_gain_streak。**绝不可**加 gold/test diff、expected-vs-actual、隐藏测试源、raw 隐藏测试 stderr。富化经 distiller Jaccard-dedup（`compress_belief_state`，distiller.rs）压缩、O(1) 预算 |
| 所有 evidence/probe manifest 须记 `leak_in_any_prompt = false` | manifest 写入点 tdma_runner.rs:819 |
| 测试须断言 Atom 1 的 `target_files` 永不含 gold/test-patch 内容 | 新增测试（§7 Atom 1 validation）|
| 允许 pre-fix base_commit 源（真实工程师读的 buggy 代码）；不允许解 patch | Atom 1 抓取仅读 base_commit 内容 |

### 5.4 tape 完整性红线（D1 —— 新增，承重）

| 红线 | 强制点 |
|---|---|
| loop 臂每次 attempt **必须**产 ChainTape commit（无 tape 内核运行违宪，Art. 0.2 constitution.md:54） | `step_forward` 无条件 commit（tdma_runner.rs:644；memory_kernel.rs:162-207）；`handle_rejection` 无条件 commit AgentProposal+RetryBeliefState（memory_kernel.rs:211） |
| loop 臂 Phase E 须用 git tape（Path B），**禁** `--tape-backend=memory`（违 Art. 0.4 constitution.md:149） | 生产默认 `tape_backend="git"`（cmd_tdma.rs:321）；但**注意**：本 campaign loop 臂走 full-flow binary（非 `tdma run`），其 tape 经 Sequencer 的 `Git2LedgerWriter` L4 ledger（runtime/mod.rs genesis 路径），即宪法 ChainTape 本体，非 `tdma_tape.git` GitTapeLedger |
| `derive_latest_belief_state_from_tape` 必须纯函数只读 tape（Gate 5，Art. 0.2） | ledger.rs:564 |
| RetryBeliefState 只活在 `TapeNode.payload`（不入 memory cache） | ledger.rs:548 |
| evidence 不得僭越 ChainTape/CAS 为真相源 | §9.5；审计 SECOND-SOURCE-DRIFT 域 |

### 5.5 DeepSeek V4 Flash 集成防御红线（D2/findings —— 新增，承重）

两臂的 LLM 调用**都**必须实现以下防御，否则 0% apply 风险（§3.6）：

| 防御 | 要求 | 来源 |
|---|---|---|
| **thinking=disabled 强制** | worker 每次请求显式 `{"thinking":{"type":"disabled"}}`；**不**靠省略 `reasoning_effort` 或设 `"none"`（flash 默认 thinking-ON，且 `"none"` 仍产 reasoning_content） | dsmlFormat / v4Flash findings；LiteLLM#27453 |
| **DSML-leak 检测+恢复** | 响应后查 `finish_reason=="stop"` 且 `tool_calls` null → 扫 `content` 的 `<｜DSML｜tool_calls>`/`<｜DSML｜invoke`/裸 `<function_c` → 手动从 `name`/`parameter` 属性解析重建 | openclaw#85918（`openai-transport-stream.ts`）；sglang#14695；NVIDIA forum |
| **reasoning_content round-trip**（仅若 meta 升 thinking-on 时） | 若任何角色 thinking-on，assistant 消息的 `reasoning_content` 须保留并回传（标准 SDK 会剥离未知字段，须显式复制），否则多轮 400 | DeepSeek Thinking Mode docs；LiteLLM#27453 |
| **strict-mode tool schema + max_tokens** | `"strict": true` + `additionalProperties:false` + 全属性入 `required` + 显式 `max_tokens` 防 `finish_reason:length` 截断 | DeepSeek tool_calls docs |
| **第三方 wrapper 警惕** | 经代理/wrapper（NIM/Foundry）时 DSML 恢复为强制防御层；并与 cost 交叉核验 token（NIM 虚高） | NVIDIA forum；openclaw#85918 |

→ **诚实标注**：findings 对「0% 是 DSML-leak 还是 malformed-diff 主导」**无法在无 agent log 下定论；二者可能复合**。先在 flask 三元组实测分离（G1 门）。

---

## 6. 指标体系（可衡量的优缺点）

### 6.1 主指标 + 统计 + cost headline（D2）

- **Resolve 率（pass@1，T 固定）**：每臂二值，分数 + 百分比。
- **Wilson 95% CI（每臂）**：`WilsonCi::new_95(successes, trials)` (wilson_ci.rs:36)，`format_percent` (wilson_ci.rs:60)。
- **配对消融**：
  - **McNemar**：χ² = (|b−c|−1)²/(b+c)，1 df；b=loop 解出而裸未解，c=裸解出而 loop 未解。功效由 b+c 驱动。小 b+c 用精确二项 p = P(Bin(b+c, 0.5) ≥ max(b,c))。
  - **配对 bootstrap（BCa）**：B=10,000 重采样配对结果，per-resample Δ=mean(loop)−mean(bare)；仅当 BCa 区间整体 >0 **且** 符号翻转置换 p<0.05 才宣称显著。
  - **pass@k 提醒**：loop 内部重试 ≈ bare 的 k>1。正确框架是**匹配期望成本**下比较，而非天真 loop(k=1) vs bare(k=1)。报告声明此口径。
- **🎯 cost-per-resolved 成为 headline（D2 新增，flash 是 cheap 的核心论证）**：
  - flash 价（list，findings v4Flash.pricing）：$0.14/M in（cache miss）、$0.0028/M in（cache hit，~50x 便宜）、$0.28/M out。对比 V4-Pro $0.435/M in（当前 75% off，2026-05-31 到期）、$0.87/M out。
  - 主 headline 指标：**cost-per-resolved = (两臂总 token × flash 价) ÷ resolved 数**。论证框架（§3.5）：**「flash + TuringOS 在 cost/capability 前沿上，相对 thinking-models-alone 的每解美元成本」**——这是 flash worker 的核心市场论点，而非绝对 resolve 率。
  - loop 臂虽每实例多 attempt（token↑），但若 Δresolve 为正且 flash 单价极低（且 cache-hit 在重试中可能命中前缀），cost-per-resolved 仍可能**优于**裸 thinking 模型单发——这正是要量化的前沿命题。
  - **诚实 caveat（findings）**：cost 论点的对照系是「thinking 模型单发」的公开价，而我们没有在同一 harness 下跑 thinking 模型（那会重新引入 model×scaffold 混淆）。故 cost-per-resolved 的「前沿」论断是**相对公开价目的方位论断**，非严格头对头——报告须如此声明。

### 6.2 必须 per-instance 捕获的量（含当前管线缺口）

| 指标 | 当前状态（seam）| 缺口 / 待补 |
|---|---|---|
| prompt/completion tokens | ✅ Probe 字段 `prompt_tokens_from_llm` / `completion_tokens` (tdma_runner.rs:354/353)，manifest 聚合 (tdma_runner.rs:397) | 来自 LLM usage，**可能是估计值**（代理在云 API 不返回 usage 时标 `estimated=true`）；⚠️ DSML 泄漏会**严重虚高 token 计数**（NVIDIA forum），须与 cost 交叉核验 |
| wall-clock | ✅ loop 臂 `Probe.wall_clock_ms` (tdma_runner.rs:363) | ❌ **裸臂完全不捕获 wallclock** → **Atom 4 必补** |
| retry/turn 数 | ✅ loop 臂 per_stage_attempts + manifest total_attempts | 裸臂恒为 0（单发）|
| **成本（美元）** | ❌ **任何地方都不计算 per-instance 美元** | **Atom 4 必补**：用 **flash 价**（$0.14/M in、$0.28/M out、cache-hit $0.0028/M in）× token → cost/instance + **cost-per-resolved（headline，§6.1）** |
| apply 率 | ❌ 未显式计算 | **Atom 4 必补**：patch 成功应用占比。search/replace+确定性合成应 >99%；raw model diff 典型 85-95%；**flash non-thinking 可能更低**（§3.5 张力 1）→ apply 率是 flash worker 的关键诊断 |
| per-repo / per-difficulty | ❌ 未分组 | **Atom 4 必补**：django 占 Verified 大头，按仓库分解刻画真实强项；按难度分解验证 loop 机制（findings：easy 偏定位失败，hard 偏迭代/验证失败——loop 应主要帮 hard）|
| 失败模式分类 | ❌ 无 | **Atom 4 必补**：3-phase（A 定位 / B 修复 / C 迭代验证）；**flash 特有**：DSML-leak、reasoning_content-400、malformed-diff 须单列类别 |
| **tape commit 完整性**（新增，D1） | ✅ loop 臂每 attempt commit（§5.4） | **Atom 4 补**：per-instance 记 ChainTape node 数 + git tape 存在性 + genesis_report.json 存在性（证 loop 臂确进内核、确发 tape、确走 genesis，区别于 control 臂无 tape）|

**诚实指标警告**：resolve 率可被弱测试虚高（SWE-bench 对抗增强研究显示顶级 agent 经对抗测试可掉双位数 pp；部分通过 patch 跑全仓库测试失败）。本设计**不**做对抗增强，报告中声明 resolve 率是「原测试口径」上限。

---

## 7. 任务拆解（atoms）

> 演化前轮 atom（用已校正精确 seam），据四指令重排：**新增 Atom 0（full-flow genesis 入口）**、**新增 Atom 5（tape 富化，D2 论题前置）**，调整 Atom 2 为更承重（flash format 对冲）。每个 atom 注明 seam、改什么、验证、**先在 flask 三元组验证再放大**。

### Atom 0 —（新增，D4）full-flow genesis 入口 + flash worker 配置

- **改什么**：loop 臂**改走 full-flow binary** `swebench_live_coding_repair_current_kernel`（src/bin/，调 `build_chaintape_sequencer_with_initial_q` runtime/mod.rs:706），**取代** `tdma run --judge swebench` 快捷方式（§3.3）。每实例 genesis-per-task（`resume_existing_chain: false`，§4.5）。配置 worker = `deepseek-v4-flash` + thinking=disabled（D2），meta 槽默认同 flash+disabled（D3）。worker/meta 模型经 `turingos llm config --blackbox-model deepseek-v4-flash --blackbox-thinking off`（cmd_llm.rs:429）写入 turingos.toml。
- **诚实守卫**：full-flow binary **不自带** trust-root 校验（§3.3.1）；若需 stage 0，runner 前显式调 `verify_trust_root`（boot.rs:97）——**可选**，不夸大。
- **seam**：
  - full-flow 入口 — `swebench_live_coding_repair_current_kernel.rs`（genesis 调用 :452、`resume_existing_chain:false` :450、签名 TaskOpenTx :478 / EscrowLockTx :495 / WorkTx :514、GenesisReport 写出 :564）
  - genesis 工厂 — `build_chaintape_sequencer_with_initial_q` **runtime/mod.rs:706**；fail-closed `NonEmptyRuntimeRepo` **runtime/mod.rs:731**；QState::genesis **q_state.rs:958**
  - worker/meta 配置 — `read_blackbox_model` cmd_llm.rs:481 / `read_blackbox_thinking` cmd_llm.rs:527 / `read_meta_thinking` cmd_llm.rs:508；DUAL-KEY 示例 cmd_llm.rs:146；默认模型 chat_client.rs:25/29
  - thinking 透传 — cmd_tdma.rs:445（读）→ :583/:591（透传 chat_complete_blocking）
- **验证**：flask 三元组上确认 full-flow binary 完成 genesis（产 pinned_pubkeys.json / initial_q_state.json / genesis_report.json）、提交签名 typed-tx、ChainTape L4 推进；确认 worker 请求体含 `{"thinking":{"type":"disabled"}}`。
- **先在 flask 三元组验证再放大**。

### Atom 0.5 —（新增，最关键）把真 Docker SwebenchTestJudge 接进 full-flow binary（scoring parity）

- **为何是头号 atom**：见 §3.3.2。没有它，loop 臂（结构性 verdict）与裸臂（真 Docker resolve）打分口径不同，**对比作废**。这比所有 prompt 对称性问题更优先。
- **第一步（分类裁定，阻塞）**：核实 full-flow binary 的 WorkTx 门（binary:522 `patch_structurally_plausible`）究竟在 binary 本地逻辑、还是 Sequencer 的 predicate registry（`apply_one` / `activate_predicate_binding_for_boot` runtime/mod.rs:872）。据此定 **Class 2 / Class 4**（§3.3.2）。**Class 4 须先取 §8 架构师批准再动手。**
- **改什么（保守安全路径，预期 Class 2）**：**不改** WorkTx 的结构性门（保住 binary liveness 语义）；在 full-flow binary 跑完 genesis + 签名 typed-tx 后，对同一 model patch **额外调** `SwebenchTestJudge::verdict`（swebench_test_judge.rs:159）做真 Docker 评测，把 `resolved` 布尔 + FAIL_TO_PASS 结果写进 `SweBenchEvidenceManifest` 作 benchmark 口径。裸臂沿用同一 `SwebenchTestJudge` 路径 → **两臂 scoring parity**。
- **屏蔽守恒**：`SwebenchTestJudge` 已是屏蔽的（fail reason 仅测试名 + apply error，§5.3）；接入不得引入 gold/test 泄漏。
- **seam**：
  - full-flow binary 评测注入点 — `swebench_live_coding_repair_current_kernel.rs`（结构 verdict :358-365；WorkTx 门 :522；evidence manifest 写出 :607-636 附近）
  - 真 Docker judge — `SwebenchTestJudge::verdict` **swebench_test_judge.rs:159**（`resolved` 读 :288）
  - 分类定位 — binary:522 `patch_structurally_plausible`；Sequencer predicate registry（若涉及）`apply_one` / `activate_predicate_binding_for_boot`（runtime/mod.rs:872）
- **验证**：flask 三元组上确认 full-flow binary 既产 genesis_report.json + ChainTape（liveness），又产真 Docker `resolved`（capability）；**diff 两臂评测调用栈，证明 `resolved` 来自同一 `SwebenchTestJudge` 代码路径**（scoring parity 的机器证据）。
- **先在 flask 三元组验证再放大**。

### Atom 1 — 喂 target-file 上下文

- **改什么**：为每实例 shallow-clone 仓库 @ `base_commit`，启发式选候选源文件（problem_statement 点名路径 + 失败测试 import 的源模块），把 `target_files=[{path, content}]` 嵌入 sample JSON（上限 ~3 文件 / ~2k 行）。给 `SwebenchSampleInput` 加 `target_files: Vec<TargetFile>`（`#[serde(default)]`，optional），在 `make_swebench_user_prompt` 渲染。
- **屏蔽守卫**：仅读 base_commit（pre-fix）内容；**显式排除** test_patch / FAIL_TO_PASS 测试模块里的任何路径。
- **seam**：
  - `SwebenchSampleInput` 结构 — **cmd_tdma.rs:37**（扩展字段）
  - `make_swebench_user_prompt` — **cmd_tdma.rs:259**（渲染 target_files）
  - **抓取脚本**：⚠️ 前轮 handoff 假设的 `scripts/probe_swebench_fetch_sample.py` **不存在**（已核实 NOT FOUND）。现有脚本：`probe_swebench_goldsmoke.sh` / `probe_swebench_loop.sh` / `probe_swebench_expand.sh` / `probe_bare_v4_swebench.py` / `run_true_suite_swebench_current_kernel.sh`。Atom 1 须**新建** `scripts/probe_swebench_fetch_sample.py`（新脚本 → 必须进 script_liveness_inventory.toml 的 `[[script_group]]`，见 §9）。
- **验证**：单元测试断言 `target_files` 永不含 gold/test-patch 内容（红线 §5.3）；flask 三元组目检抓到文件是 pre-fix 正文。
- **先在 flask 三元组验证再放大**。

### Atom 2 — 鲁棒编辑格式 → 确定性 diff 合成（**因 flash worker 更承重**，D2）

- **改什么**：重写 `SWEBENCH_SYSTEM_PROMPT` 要求 search/replace 块（Aider 式：path header + `<<<<<<< SEARCH / ======= / >>>>>>> REPLACE`，置于 patch JSON 内），**保留 `tdma-state-update/v1` 首行 header + `---BODY---` 契约不变**。在 materialization 槽把 SEARCH/REPLACE 块应用到 Atom 1 的 target_files 原文，产编辑后内容，**确定性计算正确 unified diff**。扩展 `extract_patch` 解析 search/replace 块，**保留 raw-diff 作一等回退**。区分屏蔽反馈：「SEARCH block not found in file」vs「tests still failing」。**替代根治路径（findings 推荐，可选）**：把编辑包成结构化工具调用（`str_replace_editor`/`write_file`），让 apply 由 tool call 中介，彻底绕开 diff 文本解析。
- **为何因 flash 更承重（§3.5 张力 1）**：flash non-thinking 在 hunk 行号算术上更弱、更易退化整文件重写。把行号算术**彻底从 flash 手里拿走**正是 Atom 2 的核心价值——flash worker 下它**比 thinking-worker 版本更承重，不是更轻**。
- **DeepSeek 现实赌注（§3.6）**：DeepSeek 原生偏好 unified diff、抗拒 search/replace → **raw unified-diff 直通必须作一等回退**，Go/No-Go 门经验定 V4 Flash 上主路径。叠加 §5.5 的 DSML-leak 防御 + thinking=disabled。
- **diff 合成实现选项**：⚠️ `similar` crate **不在 Cargo.toml**（已核实）；`diff`/`patch` crate 亦不在。两路：(a) 加 `similar` crate；(b) 经 `std::process::Command` 调 `git diff --no-index` 于临时文件。加 crate 是 Cargo.toml 改动（非 trust-root，但需 §9 注意）。
- **seam**：
  - `SWEBENCH_SYSTEM_PROMPT` 重写 — **cmd_tdma.rs:252**
  - **materialization 槽（合成放这里）** — **swebench_test_judge.rs:161-193**（extract_patch 调用 :161，predictions 写 :176-193）。⚠️ 前轮 handoff「170-193」混三子区，已校正：apply-gate 失败分支 **:265-280** + `harness_failure_reason` **:321**；resolve→Pass **:289**（读 resolved **:288**）；FAIL_TO_PASS 抽取 **:292-311**。
  - `extract_patch` 扩展 — **swebench_test_judge.rs:78**
  - 区分反馈路由 — `swebench_failed_predicate` **tdma_runner.rs:439**
- **验证**：单元测试覆盖 search/replace 解析 + 确定性 diff 输出；flask 三元组确认合成 diff `@@` 偏移正确、apply 越过屏障；**确认 flash worker 在 thinking=disabled + DSML 防御下不再 0% apply**。
- **先在 flask 三元组验证再放大**（Go/No-Go 门核心，见 §8）。

### Atom 3 — n=50 Verified 批处理 + 评分 + 诚实报告（**经 full-flow binary**，D4）

- **改什么**：数据集切 `princeton-nlp/SWE-bench_Verified`。确定性选 50 实例（§4.2），逐个 gold-gate，非 hermetic 带日志丢弃。建可续跑批聚合器（扩展 `scripts/probe_swebench_expand.sh`）：循环 gold-gated 实例 × {loop 臂=**full-flow binary（经 Atom 0.5 接真 Docker judge）**, 裸 flash 臂}，跳过已有 report.json 的实例。loop 臂 genesis-per-task（每实例全新 runtime_repo，§4.5）。**两臂的 `resolved` 均来自同一 `SwebenchTestJudge` Docker 路径（loop 臂经 Atom 0.5 注入，裸臂经 `probe_bare_v4_swebench.py:148`）——scoring parity 是有效对比前提（§3.3.2）**。汇总算每臂 Wilson 95% CI。
- **seam**：
  - `WilsonCi::new_95` — **wilson_ci.rs:36**；`format_percent` — **wilson_ci.rs:60**
  - 批处理 — `scripts/probe_swebench_expand.sh`（扩为驱动 full-flow binary）
  - 裸臂 resolved parse — **probe_bare_v4_swebench.py:148**（`out["resolved"] = inst.get("resolved")`）
- **验证**：flask 三元组端到端跑通续跑逻辑（杀掉再续，确认不重跑已完成实例）；qemu flake 重试不被误记 resolved；确认每 loop 实例确产 genesis_report.json + ChainTape（§5.4）；每实例独立 runtime_repo（防 NonEmptyRuntimeRepo）。
- **先在 flask 三元组验证再放大**。

### Atom 4 — 指标捕获 + 对比表/坐标报告（cost-per-resolved headline）

- **改什么**：补齐 §6.2 全部缺口。
  1. **裸臂加 wallclock 捕获**（当前缺失）。
  2. **per-instance 美元成本（用 flash 价）** + **cost-per-resolved（headline，§6.1）**。
  3. **apply 率**：两臂分别统计（flash worker 的关键诊断）。
  4. **per-repo / per-difficulty 分解** + **失败模式分类**（3-phase + flash 特有类别：DSML-leak / reasoning_content-400 / malformed-diff；人工分类 inter-rater Cohen's Kappa ≥0.72）。
  5. **tape 完整性记录**（per-instance ChainTape node 数 + git tape 存在性 + genesis_report.json 存在性，证 loop 臂确进内核，§5.4/§6.2）。
  6. 写 `handover/reports/PROBE_SWEBENCH_VERIFIED50_<UTC>.md`：loop vs 裸 resolve 率 + Wilson CI + McNemar/bootstrap + **cost-per-resolved headline** + apply 率 + token/wall + per-repo/difficulty + 失败分类 + tape 完整性 + **§2.2 市场坐标表（带混淆声明 + flash 口径附注）** + **§1.2 归因边界声明（全系统 Δ，不分解子组件）** + 消融结论。
- **seam**：
  - 裸臂 metrics — `scripts/probe_bare_v4_swebench.py`（加 wallclock + **flash 价** cost）
  - loop 臂 metrics 大部在 Probe (tdma_runner.rs:348-364) / manifest (:397)；cost/apply 率/分组/tape 完整性是报告层新计算
  - 报告生成 — 扩展 `scripts/probe_swebench_expand.sh` 或新建报告脚本（新脚本 → §9 liveness）
- **验证**：flask 三元组确认报告含全部字段、坐标表带混淆声明 + flash 口径附注、cost 数与 token 自洽、归因边界声明在场。
- **先在 flask 三元组验证再放大**。

### Atom 5 —（新增，D2 论题前置）tape 富化（在屏蔽红线内）

- **改什么**：使 retry/loop 上下文携带**真实推理 substrate**（§3.5 张力 2），而非仅失败测试名，让「tape = 外化 CoT」论题在 flash worker 上真实成立。在 `RetryBeliefState` / `assemble_o1_prompt` 路径内，于屏蔽规则下加入：**模型自己的先前 rationale 摘要 + 模型自己的 apply error + attempt 序号 + zero_gain_streak**。经 distiller Jaccard-dedup（`compress_belief_state`，distiller.rs）压缩、保 O(1) token 预算。
- **屏蔽护栏（§5.3 新行，承重）**：**绝不可**加 gold/test diff、expected-vs-actual、隐藏测试源、raw 隐藏测试 stderr。富化的是**结构化 rationale**，不是 raw 失败堆叠。
- **contextual-drag 守卫（§3.5 findings）**：把失败先前尝试喂回弱 flash 可能恶化性能（10-20% 跌幅，可坍缩为 self-deterioration）→ 富化必须是**压缩后的结构化 rationale**（distiller 约束），不得堆原始失败正文。findings 强调富化要含「hypothesis → evidence → revision」结构（root-cause framing：feedback 应问「为何失败」而非仅「修这个失败」），纯 action log 不替代内部 thinking。
- **seam**：
  - retry prompt 组装 — `assemble_o1_prompt` **memory_kernel.rs:377-422**（RAW STDERR 永不入，:374）
  - RetryBeliefState 载荷 — `ledger.rs:548`（failure_signature + 累积 constraints + zero_gain_streak，仅 TapeNode.payload）
  - 压缩 — `compress_belief_state`（distiller.rs，Jaccard-dedup）
  - 反馈源 — `make_judge_stderr` tdma_runner.rs:444/448
- **验证**：单元测试断言富化后**仍** `leak_in_any_prompt=false`（无 gold/test/隐藏测试源泄漏）；flask 三元组对比 thin vs rich tape 的 apply/resolve（这是 §1.4-B 消融的本地预演，但**本 campaign 主 Δ 仍以选定的单一 tape 配置跑**，富化与否须在报告固定声明，不在主 Δ 内混两配置）。
- **诚实标注**：Atom 5 是让 D2 论题为真的**必要 refinement**；**但**它本身改变了 tape 富度这一变量。为保 §1.2 归因边界，**主 Δ 跑只用一个固定 tape 配置**（建议：富化后，使 D2 论题成立）；thin-vs-rich 对比留作 §1.4-B 后续消融，不混入主 Δ。findings 另注：tape-with-rationale vs tape-without 的同数据集 ablation 文献中尚不存在，故 §1.4-B 是潜在科学贡献而非已知量。
- **先在 flask 三元组验证再放大**。

---

## 8. Go/No-Go 门

**门 G0（新增，D4 —— full-flow genesis 活着）**：

- flask 三元组 loop 臂经 **full-flow binary** 跑通完整 genesis 生命周期：产 pinned_pubkeys.json / initial_q_state.json / genesis_report.json，提交签名 TaskOpenTx/EscrowLockTx/WorkTx，ChainTape L4 推进，每 attempt 有 tape commit（§5.4）。
- **若 genesis 路径失败**（如 `NonEmptyRuntimeRepo` 误触、Sequencer 未起、typed-tx 拒绝）：先修 Atom 0，**不**放大。
- 判据：≥1 个 flask 实例完整走完 stage 5-11（genesis → typed-tx → GenesisReport），ChainTape 可 replay。

**门 G0.5（新增，§3.3.2 —— scoring parity，承重）**：

- 确认 loop 臂（full-flow binary，经 Atom 0.5）与裸臂的 `resolved` 来自**同一 `SwebenchTestJudge` Docker 代码路径**，口径一致。
- 判据：flask 三元组上 diff 两臂评测调用栈，证明同口径；loop 臂的结构性 `patch_structurally_plausible` **不**被用作 benchmark 打分（仅作 binary 内 liveness 语义）。
- **若两臂口径不一致**：对比无效，先修 Atom 0.5，**不**放大。

**门 G1（apply 屏障已修 + flash 集成防御生效，花 50 实例算力前的硬门）**：

- 重跑 **flask 三元组**的 **loop 臂**（经 full-flow binary），patch 必须**到达 test 阶段（不再卡在 apply）** 对多数尝试成立。
- 判据：Atom 1+2（+5）实现后，多数（≥majority）尝试 patch 越过 apply 屏障、隐藏测试真实执行（无论 pass/fail）。**且确认 §5.5 防御生效**：worker 请求 thinking=disabled、无 DSML-leak 静默失败、无 reasoning_content 400。
- **若 apply 仍是墙**：先查是 DSML-leak / reasoning_content-400 / malformed-diff 哪类（§3.6 分离诊断），修 Atom 2/§5.5，**不**放大到 n=50。
- **诚实警告**：前轮一份报告曾过度宣称 flask-5063 attempt 3 单次随机越过 apply（测试跑了但逻辑失败），在最终配置下**未复现**。**先验证再写报告**——一次随机 apply 不是「apply 屏障已修」。

**门 G2（gold-gate）**：n=50 子集每实例必须 gold-resolve（§4.3），否则丢弃并记录。`n_gold_valid/50` 随报告披露。

**门 G3（FC1 不变量）**：`evaluator_reported_completed_llm_calls = tool_dist.step + tool_dist.parse_fail + tool_dist.llm_err`（LHS **不得**用 `evaluator_reported_tx_count`）。若等式失败 → **HALT**，不继续基准、不审计为 pass。

---

## 9. 宪法合规与已知坑

### 9.1 允许路径（本 campaign 触及）

- `scripts/**` — 新 runner/probe 脚本（新建 `probe_swebench_fetch_sample.py` + 可能的报告脚本）。**每个新文件必须进 `script_liveness_inventory.toml` 的 `[[script_group]]`**（id / classification ∈ {local_probe 等} / status / paths(须存在) / covered_by / counts_for_obl005_script_closure）。
- `src/judges/**` — 若 Atom 2 新建 judge 子模块，须在 `src/judges/mod.rs` 声明并加入相应 liveness group 的 `module_ids`（重复 module_id 跨 group 会 panic）。**本设计倾向就地扩展现有 `swebench_test_judge.rs`，避免新模块**。
- `src/bin/turingos/**` — `cmd_tdma.rs` 扩展（struct + prompt + 渲染 + per-role thinking 已存在）。已在 CLI orchestration liveness group，无需新 group。
- `src/bin/swebench_live_coding_repair_current_kernel.rs` — full-flow 入口（D4）。若需扩展，确认其 liveness group 归属（bin root 可达）。
- `src/memory_kernel.rs` — **⚠️ Atom 5 tape 富化触及**。`assemble_o1_prompt`/`handle_rejection` 在此。**注意**：memory_kernel.rs 是内核 TDMA 状态机，改动须谨慎判 Class（富化只改 prompt 组装的屏蔽内容拼接，不改 admission/typed-tx schema → 倾向 Class 2，但须 §9.2 复核不触受限面）。
- `tests/**` — 新断言（target_files 无泄漏、search/replace 解析、Atom 5 富化后无泄漏）。测试文件免 liveness 注册，但写 dated `handover/evidence/` 须带 `TURINGOS_TEST_REGENERATE_EVIDENCE` env gate。
- `Cargo.toml` — 若 Atom 2 加 `similar` crate（非 trust-root，但 §9.3 liveness 注意）。
- `handover/**` — Class 0 文档（reports/）。`handover/evidence/` 写入须 runner-preflight + env gate。

### 9.2 受限面（禁止触碰）

`src/kernel.rs`、`src/bus.rs`、`src/state/sequencer.rs`、`src/state/typed_tx.rs`、`src/bottom_white/cas/schema.rs`、`src/bottom_white/ledger/*`、`src/top_white/*`、`src/runtime/predicate_registry_loader.rs`、RootBox/签名载荷、`constitution.md` flowchart 块（Class 4，human sudo only）。

- ⚠️ **D4 重要边界**：full-flow binary 调用的 `build_chaintape_sequencer_with_initial_q`（runtime/mod.rs）与 Sequencer 是宪法核心。**本 campaign 仅作为 caller 调用 genesis 工厂，不修改 runtime/mod.rs 的 genesis 逻辑、不改 Sequencer admission、不改 typed-tx schema**。若 full-flow binary 需任何 genesis/sequencer 行为改动 → 立即升 Class 4，停下等 §8 ratification。
- ⚠️ **Atom 5 边界**：memory_kernel.rs 的 `assemble_o1_prompt` 改动**仅限**屏蔽内容拼接（加模型自己的 rationale/apply-error 摘要），**不得**改 `step_forward` 的 admission/Proceed/Retry 决策、不得改 tape commit 语义、不得改 RetryBeliefState 的 typed schema 判别。越此即 Class 4。

### 9.3 Liveness 注册（+ clean-worktree fs::read_dir 坑）

- **模块 liveness**（`tests/fixtures/liveness/production_module_liveness.toml`，gate `tests/constitution_production_module_liveness.rs`）：gate 用 **`fs::read_dir`** 遍历 `src/`（**非 git ls-files**），断言每个磁盘 `.rs` 可从 lib.rs/main.rs/bin root 抵达。**未追踪新文件写盘但未在模块树声明 → 扫描发现但 `declared_source_files()` 没有 → 测试红**。
- **脚本 liveness**（`script_liveness_inventory.toml`，gate `constitution_script_liveness_inventory.rs`）：`collect_files`（同 fs::read_dir）收集 `scripts/, tools/, rules/, .claude/hooks/, .github/workflows/`，断言磁盘集 == script_group 声明集。**未追踪新脚本同样触发**。
- **铁律**：**在干净 git worktree checkout 上验证 liveness**——in-place dirty worktree（新文件已写未提交）会污染扫描。本 campaign 新建脚本前必须先注册或在干净 checkout 验证。

### 9.4 PR-only + clean-context 见证 + 分支

- **本 campaign 在 BRANCH 上运行**（架构指令：definitely branch）。
- **PR-only / no-merge-main**（AGENTS.md §14a，三层强制：GitHub branch protection / `pre-push.harden` / `validate_git_push.sh`）。Agent 只能开 PR；merge 是 orchestrator 专责（`GIT_HARDEN_ALLOW_MAIN=1 git push origin main`）。
- **Class 2 clean-context 见证**（2026-05-29 批准的平台无关学说，已**取代**旧「Codex 见证」专属要求）：新 judge 模块 / 写 evidence 的 runner 脚本 / memory_kernel.rs tape 富化 = Class 2，需**一名无实现 transcript 的新 agent**（任意平台：Claude/Codex/Antigravity）审计。只给：任务 brief + 风险类 + 触及 FC 节点 + diff/commit + 源/文档 + evidence 路径 + 精确验证命令输出 + 裁决格式 {PROCEED | CHALLENGE | VETO}。审计法律域：{NO-VIOLATION, VIOLATION-FOUND, RECONSTRUCTION-FAILURE, SECOND-SOURCE-DRIFT}。主观风格/性能/架构意见 out-of-scope。
- **pre-charter 并行写检查**：起草任何触及 src/ 或 scripts/ 的 charter 前，跑 `gh pr list --state open --json number,headRefName,title,files` 查在途 PR 路径冲突。

### 9.5 No-sidecar evidence

- **禁止提交进 PR**：`handover/evidence/dev_self_hosting/dev_*/`（`pre-commit.r022` 阻断）、`.claude/worktrees/`；**禁止通配 git add**（`git add .`/`-A`/`-u` 被 `validate_git_add.sh` 阻断，**仅用显式路径**）。
- **evidence 不可篡改**（FC2-INV5）：写 dated evidence 的测试须含 `TURINGOS_TEST_REGENERATE_EVIDENCE`；`cargo test --workspace` 不得静默改 committed evidence。**永不回溯改写/伪造 evidence**。
- **evidence 不得僭越 ChainTape/CAS 为真相源**：stdout/dashboard/私有计数器/LLM 自报/无锚 JSON 不足为证；须可从 ChainTape/CAS 重建（Art. 0.2，§5.4）。审计 SECOND-SOURCE-DRIFT 标记僭越的派生视图。
- **本 campaign 只提交**：代码 + manifest + 最终报告。**不提交** `handover/evidence/**` sidecar（CI no-sidecar 检查）。

### 9.6 坑清单（runnable 必读）

1. **后台 Bash 把 cwd 重置到 `$HOME`**（/Users/zephryj）。永远 cd 进仓库或前台跑；从 home 跑 cargo → 「could not find Cargo.toml」。
2. **TDMA header 是承重的**：丢首行 `{"schema_version":"tdma-state-update/v1",...}` 或 `---BODY---` 分隔符 → 内核永不 Proceed（看似模型失败，实为 prompt-契约 bug）。Atom 2 重写 prompt 时**必须保留 header + marker**。
3. **sanitized runner 剥离代理变量** → HF dataset 抓取必须 offline（`HF_HUB_OFFLINE=1` + `HF_DATASETS_OFFLINE=1`，已在 judge:228-229），否则隐藏测试静默不跑。
4. **qemu 在 arm64 Mac 慢（~20-45s/eval）且偶发 SSL flake** → 批处理必须可续跑/可重跑；**flake 绝不当真 resolved**。
5. **诚实报告**：前轮曾过度宣称 flask-5063 单次随机 apply（未复现）。**先验证再写**。
6. **不提交 `handover/evidence/**` sidecar**（CI no-sidecar 检查）。只提交代码 + manifest + 最终报告。
7. **`similar` crate 不在 Cargo.toml**（已核实）——Atom 2 需它或用 `git diff --no-index`。
8. **`requests` 家族非 hermetic**（httpbin → 503 offline）。仅 flask/django/sympy/pylint（+ sphinx/pytest/xarray）确认 hermetic。
9. **`scripts/probe_swebench_fetch_sample.py` 不存在**（前轮 handoff 误认其存在）——Atom 1 须新建并注册 liveness。
10. **运行写 evidence 的 runner 前调 `/runner-preflight`**：干净/已理解树、二进制 vs HEAD 新鲜、evidence 不可篡改、风险类、FC trace。
11. **批跑中不碰 trust-root-pinned 文件**：若需中途改源，先 abort 批处理或接受废跑，再编辑。
12. **（D2）flash 默认 thinking-ON**：worker 必须显式 `{"thinking":{"type":"disabled"}}`，省略 `reasoning_effort` 或设 `"none"` 不足以关；否则产 `reasoning_content` → 多轮 400 → trajectory 夭折（§1.3/§3.6/§5.5）。
13. **（D2）DSML-leak**：V4 工具格式 DSML（`｜DSML｜` token + `string="true|false"` 属性）经兼容层会泄漏进 `content` 致 `tool_calls` null 静默失败；须 client-side 检测+恢复（§5.5）；DSML 还会虚高 token 计数（与 cost 交叉核验，§6.2）。第三方 wrapper（NIM/Foundry）可能 100% 泄漏。
14. **（D4）full-flow binary 不自带 trust-root 校验**（§3.3.1）：不夸大「自带 stage 0」；需要时 runner 前显式调 `verify_trust_root`。
15. **（D4）genesis-per-task fail-closed**：`NonEmptyRuntimeRepo`（runtime/mod.rs:731）要求每实例全新 runtime_repo；批跑须确保每实例独立 runtime_repo 目录，否则第二实例起 genesis 失败。
16. **（findings）外化循环收益前 2 轮占 76-95%**：不能靠加迭代线性补偿弱 flash；max_attempts 设置以此为据，过深迭代收益递减且增 contextual-drag 风险。

---

## 10. 风险与混淆控制

| 威胁（threat to validity）| 控制 |
|---|---|
| **两臂对模型暴露面不对称**（裸臂 SYS 在 `probe_bare_v4_swebench.py:30` 仅「Output ONLY strict JSON...」，**无 TDMA header / ---BODY--- marker**；注释还误指 cmd_tdma.rs:249，实际 const 在 :252）| **PRIMARY 消融的头号混淆。** 校正：裸臂 SYS 必须与 loop 臂 `SWEBENCH_SYSTEM_PROMPT` (cmd_tdma.rs:252) **sha256 byte-identical**（前轮 `_fix2` 曾达一致，须恢复/复核）。区别只剩「是否经完整 TuringOS 生命周期」一项。报告记两臂 SYS/USER prompt sha256。 |
| **归因边界（D4 新增首要诚实项）** | Δ 归因「完整 TuringOS（genesis+内核+外化 CoT）vs 裸 flash」整体，**不分解**子组件（§1.2）。报告须含显式归因边界声明；越界分解（如「Δ 证明 retry 循环有效」）违 §1.6.4，审计 SECOND-SOURCE-DRIFT 标记。子组件分解留 §1.4 后续消融。 |
| **worker thinking 污染外化 CoT 论题（D2 新增）** | worker 必须 flash + 显式 thinking=disabled（§1.3/§5.5）。若任何角色误用 thinking-on，内部 CoT 污染「tape = 外化 CoT」论题。报告须记每角色 thinking 状态；meta 若升 thinking-on，视作 §1.4-B 变体，不混主 Δ（§3.4）。 |
| **flash format 弱点致 apply 失败（D2 新增）** | §3.5 张力 1：Atom 2 确定性 diff 合成把行号从 flash 手里拿走（更承重）；raw-diff 回退；可选结构化 tool-call 中介；§5.5 DSML 防御 + thinking=disabled。G1 门实测 flask 三元组分离 DSML-leak/reasoning-400/malformed-diff（§3.6）。 |
| **tape 偏 thin 致外化 CoT 论题不成立（D2 新增）** | §3.5 张力 2：当前 tape 仅失败测试名级（已核实）；Atom 5 在屏蔽内富化（模型自己 rationale/apply-error）。诚实标注：富化前论题仅部分成立。主 Δ 用固定 tape 配置（§7 Atom 5 诚实标注）。 |
| **localization 是循环前瓶颈（findings 新增）** | §3.5：弱模型起手定位错文件，测试反馈纠不回（pre-loop 问题，Agentless 多类型 F1=0.142）。报告对需多文件定位的实例 Δ 保持谦逊；per-difficulty 分解可暴露此模式。 |
| **两臂网络路径不对称**（loop 臂经 Rust `chat_complete_blocking` + 生产重试/超时；裸臂 Python urllib 无重试；二者最终打同一代理 → 同一 DeepSeek）| 影响延迟测量。控制：wallclock 仅辅助不作主张；resolve 率不受网络路径影响（同模型同端点）。报告披露。 |
| **DeepSeek V4 Flash 数字不可得 / 不可外推** | §1.6 明令不外推 V4 Flash 79.0%（且非 disabled 口径）或 V4 Pro 80.6%。坐标表标「厂商自报、未独立复现、非 disabled 口径」。无 disabled 模式公开直测值（AA Index 36 vs 47 暗示明显更低）。 |
| **model+scaffold 混淆** | §2.2 坐标表每行带混淆标记 + 表头混淆声明 + flash 口径附注；§1.6 列明不主张。 |
| **cost 前沿非严格头对头（D2 新增）** | §6.1：cost-per-resolved 对照系是公开价目，非同 harness 跑 thinking 模型（避免重引混淆）。报告声明为「相对价目方位」，非头对头。 |
| **n=50 配对欠功效** | §4.2 明确承认对 10pp 配对差欠功效；不显著时写「未检出」非「无差异」；Atom 3 可续跑支持向 n≈390 放大。 |
| **gold-broken 实例污染 n** | §4.3 gold-gate 程序，已知 broken 列表（astropy/matplotlib/django-10097/requests），`n_gold_valid/50` 披露。 |
| **resolve 率被弱测试虚高** | §6.2 声明「原测试口径上限」；引对抗增强研究作 caveat。 |
| **git-history 泄漏作弊** | sanitized Docker 环境隔离；报告须含「git history 访问已禁用/审计」声明（任何 >70% 分数无此声明不可信）。 |
| **qemu flake 误记 resolved** | §9.6 坑 4：批处理可续跑，flake 重试，绝不当 resolved。 |
| **stale binary smoke** | §8 G0/G1 + runner-preflight：二进制必须 vs 当前源/HEAD 新鲜。 |
| **liveness clean-worktree 坑** | §9.3：干净 checkout 验证，新文件先注册。 |
| **genesis-per-task fail-closed 误触（D4 新增）** | §9.6 坑 15：每实例独立 runtime_repo，否则 `NonEmptyRuntimeRepo`（runtime/mod.rs:731）从第二实例起失败。批聚合器须为每实例建独立目录。 |
| **报告过度宣称**（前轮教训）| §8 + §9.6 坑 5：先验证再写；honest reporting（`model_task_failure=true` 如实记，不 relabel）。 |

---

## 11. 新 session 冷启动 prompt（paste-ready，已据四指令更新）

```
你是 TuringOS 的 benchmark 实现工程师。仓库 /Users/zephryj/work/turingosv4（基线 main @ 1f00012d）。
目标：实现「完整 TuringOS 架构(genesis→内核→外化CoT循环) vs 裸 flash 模型」SWE-bench Verified 全系统消融
campaign（同一 flash worker = DeepSeek V4 Flash, thinking 显式 disabled）。设计文档已定稿(本 prompt 即承接它)。
路线已决，不重开辩论。**本 campaign 在 BRANCH 上跑(PR-only,禁 merge main)。**

== 已决路线(勿改) ==
- PRIMARY = 全系统消融：同一 deepseek-v4-flash(thinking=disabled)、同一前处理、同一组 gold-gated 实例，
  loop 臂(经 full-flow binary swebench_live_coding_repair_current_kernel,从 genesis 进入) vs
  裸 flash 臂(probe_bare_v4_swebench.py,从不进内核)，Δresolve + Wilson 95% CI + McNemar/配对bootstrap。
- **归因边界(关键诚实项)**：Δ 归因「完整 TuringOS(genesis+内核+外化CoT循环) vs 裸 flash」整体,
  不能分解到子组件(genesis vs 循环 vs tape富度)——那需后续消融。不能把 Δ 窄说成「retry 循环有效」,
  也不能把整系统 Δ 假装成某单一子组件的功劳。
- SECONDARY = 市场坐标：V4-Pro 80.6%/Flash 79.0%(厂商自报,未复现,非disabled口径)/Opus4.7 ~80.8%/OpenHands 66.4%(Claude4)/
  CodeMonkeys 57.4%(Claude3.5 non-thinking+外化循环,最贴近本设计的公开数据点)等,仅作量级方位 + cost-per-resolved 前沿,
  带 model×scaffold 混淆声明,不头对头。OpenClaw/Hermes 非 Verified 选手,不入表。
- 不搭 OpenHands/SWE-agent 跑 V4。

== 铁律(前轮血泪 + 四指令) ==
1. **worker = flash, reasoning 外化到 tape**(D2)：worker 永远 deepseek-v4-flash + 显式 thinking=disabled。
   ChainTape/git-tape 就是显式外化的 chain-of-thought——worker 推理活在 tape+loop,不在内部 thinking
   (与 Art.0.2 "所有信号必须可从 tape 重建" 直接呼应)。(旧「thinking-on worker」铁律已废除。)
2. **没有「假 git tape」**(D1)：git-tape/ChainTape 是每次内核运行的宪法法律(Art.0.2 constitution.md:54 全信号可从tape重建;
   Art.0.4 :149 "Phase E gate 强制 B" Path B git substrate)。loop 臂=进内核(依法发 ChainTape+git tape);
   裸臂=裸模型从不进内核(无 tape)。无 tape 的内核运行违宪且架构不可能(step_forward 无条件 commit)。
   MemoryTapeLedger 不是假 tape(是全功能 in-memory 实现,commit 相同 TapeNode,仅不持久化 git);mock-model 仅在 #[cfg(test)]。
3. **从 init/genesis 全流程进入**(D4)：loop 臂必须经 full-flow binary(调 build_chaintape_sequencer_with_initial_q
   runtime/mod.rs:706 → 签名 TaskOpenTx/EscrowLockTx/WorkTx → ChainTape L4 → GenesisReport),
   不用 `tdma run --judge swebench` 快捷方式(后者旁路 genesis/Sequencer/CAS/typed-tx,只 boot MemoryKernel,流程不完整,测不出真实端到端能力)。
   genesis-per-task(每实例全新 runtime_repo;NonEmptyRuntimeRepo fail-closed runtime/mod.rs:731;成本<1s I/O,被 LLM latency 淹没)。
   诚实:full-flow binary 不自带 trust-root 校验(verify_trust_root 仅 src/main.rs:14 跑),需要时 runner 前显式调,不夸大「自带 stage 0」。
4. **meta 可 flash**(D3)：swebench loop 里只有一个 agent(--role 选槽,默认 meta cmd_tdma.rs:306),meta 角色=轻量/确定性谓词路由+patch生成
   (predicate 100%确定性无LLM,swebench_test_judge.rs:159 读 report.json resolved :288;所有 retry 逻辑确定性 Rust tdma_runner.rs:582,
   无第二个 meta-level LLM 调用)。强 flash 足矣 → meta 默认也 flash+disabled。thinking per-role 可配且已核实未硬编码
   (订正前轮「thinking:None 硬编码」探针;cmd_tdma.rs:445 读,:583/:591 透传;read_*_thinking cmd_llm.rs:508/527)。
   若实测某类复杂实例 patch 受益于 CoT 才仅对 meta 升 thinking-on,记作变体消融不混主 Δ,且届时须落实 reasoning_content round-trip。
5. gold_patch/test_patch/隐藏测试正文 永不进任何 prompt。屏蔽靠 SwebenchSampleInput(cmd_tdma.rs:37)
   只反序列化 6 字段(instance_id/repo/base_commit/problem_statement/hints_text/fail_to_pass)——禁止给它加 gold/test 字段。
   assemble_o1_prompt(memory_kernel.rs:374) RAW STDERR 永不入,仅 evidence_hash/bbs_hash 指针。
6. 重试反馈只携带 失败测试名(FAIL_TO_PASS,公开) 或 模型自身 apply error。
   **tape 富化(Atom 5,D2 论题前置)**：当前 tape 偏 thin(仅失败测试名级 failed_predicate,已核实);
   findings:tape 作纯 action log("测试X FAILED")不替代内部 thinking,作结构化 deliberation 史(hypothesis→evidence→revision)部分替代。
   为使「tape=外化CoT」在 flash 上真实成立,须在屏蔽内富化(加 模型自己的 rationale摘要/apply-error/attempt序号),经 distiller 压缩 O(1) 预算。
   绝不可加 gold/test diff/expected-vs-actual/隐藏测试源。富化是结构化 rationale,不是 raw 失败堆叠(防 contextual drag 10-20% 跌幅)。
   诚实:tape-with-rationale vs without 同数据集 ablation 文献中尚不存在(§1.4-B 是潜在贡献)。
7. PR-only,branch,不 merge main。Class 2 需 clean-context 审计(任意平台,无实现transcript,2026-05-29 平台无关学说)。
   memory_kernel.rs Atom5 改动仅限屏蔽内容拼接,不改 step_forward admission/tape commit 语义/typed schema,越此即 Class4。
   full-flow binary 仅作 caller 调 genesis 工厂,不改 runtime/mod.rs genesis/Sequencer/typed-tx,越此即 Class4 停等 §8 ratification。
8. 真实跑,不伪造确定性;flash thinking=disabled 必须真实生效;flake 绝不当 resolved。
9. hermetic 实例only(django/sympy/sphinx/pytest/xarray/pylint);requests 非 hermetic必排除。
10. **flash 集成防御(§5.5,承重)**：worker 显式{"thinking":{"type":"disabled"}}(省略 reasoning_effort 或设"none"不足以关,flash 默认 thinking-ON);
    DSML-leak client-side 检测+恢复(finish_reason=stop 且 tool_calls null 时扫 content 的 <｜DSML｜tool_calls>/<｜DSML｜invoke/裸<function_c,
    从 name/parameter 属性重建);strict-mode tool schema(strict:true+additionalProperties:false+全属性required)+显式 max_tokens;
    经 wrapper(NIM/Foundry)时 DSML 恢复为强制层(NIM 还虚高 token,与 cost 交叉核验)。

== 精确 seam(已核实 @ 1f00012d,已校正前轮 handoff 漂移) ==
[full-flow genesis 入口 — D4]
- full-flow binary: src/bin/swebench_live_coding_repair_current_kernel.rs(已核实存在;genesis调用:452,resume_existing_chain:false:450,
  签名 TaskOpenTx:478/EscrowLockTx:495/WorkTx:514,GenesisReport写出:564)
- genesis 工厂: build_chaintape_sequencer_with_initial_q runtime/mod.rs:706;fail-closed NonEmptyRuntimeRepo :731;
  Ed25519 keypair+pinned_pubkeys :782/:786;initial_q_state写盘 :831;rejections.jsonl :809;driver spawn :883;QState::genesis q_state.rs:958
- trust-root(仅 library binary): verify_trust_root boot.rs:97(src/main.rs:14);init 纯文件脚手架 cmd_init.rs:570(零sequencer/typed-tx/CAS/ChainTape)
- tdma run 快捷路径(本 campaign 不用): cmd_tdma.rs:603-624(MemoryKernel tdma_runner.rs:541,无 Sequencer/CAS/genesis)
- RESUME 模式(本 campaign 不用): TURINGOS_CHAINTAPE_RESUME=1 严格等于 runtime/mod.rs:470;bootstrap_resume_state :920
[屏蔽承重]
- SwebenchSampleInput: cmd_tdma.rs:37(6字段,屏蔽承重)
- SWEBENCH_SYSTEM_PROMPT: cmd_tdma.rs:252(含首行{"schema_version":"tdma-state-update/v1",...} header + ---BODY---,承重,重写须保留)
- make_swebench_user_prompt: cmd_tdma.rs:259
- assemble_o1_prompt: memory_kernel.rs:374(RAW STDERR 永不入,仅 evidence_hash/bbs_hash 指针);377-422 组装(CharterCore+OUTPUT CONTRACT+
  AUTHORITATIVE SESSION DIGEST+RETRY BELIEF STATE+EVIDENCE POINTERS+CURRENT TASK)
[role/thinking — D2/D3,已核实未硬编码]
- role+thinking 读取: cmd_tdma.rs:445(read_meta/blackbox_thinking);默认 role=meta :306;tape_backend 默认 git :321;透传 chat_complete_blocking :583/:591
- read_meta_thinking cmd_llm.rs:508;read_blackbox_thinking :527;read_blackbox_model :481;DUAL-KEY 示例 :146;config 写 :429
- 默认模型: DEFAULT_META_MODEL deepseek-ai/DeepSeek-V3.2 chat_client.rs:25;DEFAULT_BLACKBOX Qwen3-Coder-30B :29;thinking:None 注释(非硬编码):43
  (本 campaign 经 turingos llm config --blackbox-model deepseek-v4-flash --blackbox-thinking off 覆写;两槽皆 flash+disabled)
[tape/judge/确定性]
- step_forward 无条件 commit: tdma_runner.rs:644;memory_kernel.rs:162-207;handle_rejection :211
- ChainTape→chaintape.jsonl: tdma_runner.rs:740;derive_latest_belief_state_from_tape(纯函数只读 tape): ledger.rs:564
- RetryBeliefState 载荷(仅 TapeNode.payload): ledger.rs:548;compress_belief_state distiller.rs(Jaccard-dedup)
- judge verdict(100%确定性无LLM): swebench_test_judge.rs:159;resolved→Pass :289(读 resolved :288);extract_patch :78(调用:161)
  · predictions 写: :176-193 · run_sanitized :236(cmd:200) · HF offline :228-229 · env 白名单 :226
  · apply-gate 失败分支 :265-280 · harness_failure_reason :321 · FAIL_TO_PASS 抽取(屏蔽反馈) :292-311
  ⚠️ 前轮 handoff「170-193」混三子区,已校正如上。
- extract_body tdma_runner.rs:426 · swebench_failed_predicate :439 · make_judge_stderr :444/:448
  · run_proof_with_ledger :509 · run_proof(MemoryTapeLedger 默认,非假 tape) :495 · Probe(tokens/wall) :348-364 · manifest :397/:819/:354/:353/:363
- WilsonCi::new_95 wilson_ci.rs:36 · format_percent :60
- 裸臂 resolved parse: probe_bare_v4_swebench.py:148 · 裸臂 SYS(当前不对称,须对齐:252) :30
- loop 脚本 env 覆写: probe_swebench_loop.sh:16-17

== 6 个 atom(按序,每个先在 flask 三元组验证再放大) ==
A0(新增,D4) full-flow genesis 入口+flash 配置：loop 臂改走 full-flow binary(genesis→签名typed-tx→GenesisReport),
   取代 tdma run 快捷。worker=deepseek-v4-flash+thinking=disabled,meta 默认同。genesis-per-task。诚实:不自带 trust-root 校验。
A1 喂 target-file 上下文：clone @base_commit 选≤3文件嵌 target_files;扩 SwebenchSampleInput(:37)+渲染(:259);
   新建 scripts/probe_swebench_fetch_sample.py(⚠️前轮误认其存在,实不存在,须新建+注册liveness)。
   守卫:只读 base_commit,排除 test_patch/FAIL_TO_PASS 路径。测试断言 target_files 无 gold/test 泄漏。
A2 search/replace→确定性 diff 合成(因 flash 更承重,D2)：重写 SWEBENCH_SYSTEM_PROMPT(:252,保留 header+marker);
   materialization 在 judge:161-193;扩 extract_patch(:78);把行号算术彻底从 flash 手里拿走(对冲 non-thinking format 弱点);
   ⚠️DeepSeek 抗拒 search/replace、偏好 unified diff→raw-diff 直通保留作一等回退;可选结构化 tool-call 中介绕开 diff 文本;
   叠加 §5.5 DSML防御+thinking=disabled;similar crate 不在 Cargo.toml(须加或用 git diff --no-index)。
A3 n=50 Verified 批+评分(经 full-flow binary)：切 princeton-nlp/SWE-bench_Verified;确定性选50(字典序/固定种子);逐个 gold-gate(丢弃带日志);
   扩 probe_swebench_expand.sh 为可续跑(跳过已有 report.json)+驱动 full-flow binary;每实例独立 runtime_repo(防 NonEmptyRuntimeRepo);每臂 WilsonCi::new_95。
A4 指标捕获+报告(cost-per-resolved headline)：补缺口——裸臂加 wallclock;per-instance 美元成本(用 flash 价 $0.14/M in,$0.28/M out,cache-hit $0.0028/M);
   cost-per-resolved(headline,声明为相对价目方位非头对头);apply 率(flash 关键诊断);per-repo/difficulty;失败分类(+flash 特有:DSML-leak/reasoning-400/malformed-diff);
   tape 完整性(per-instance ChainTape node数+git tape 存在性+genesis_report 存在性,证 loop 确进内核);
   写 handover/reports/PROBE_SWEBENCH_VERIFIED50_<UTC>.md(双臂 resolve+CI+McNemar/bootstrap+cost-per-resolved+坐标表带混淆声明+flash口径附注+归因边界声明+结论)。
A5(新增,D2 论题前置) tape 富化(屏蔽内)：assemble_o1_prompt(memory_kernel.rs:377-422)在屏蔽下加 模型自己的 rationale摘要/apply-error/attempt序号,
   经 distiller 压缩 O(1) 预算;含 hypothesis→evidence→revision 结构;绝不加 gold/test/隐藏测试源;防 contextual drag(压缩结构化 rationale,非 raw 失败堆叠)。
   测试断言富化后仍 leak_in_any_prompt=false。诚实:主 Δ 用单一固定 tape 配置(建议富化后),thin-vs-rich 留后续消融(§1.4-B,文献无现成基线)。

== Go/No-Go 门 ==
G0(新增,D4): flask 三元组 loop 臂经 full-flow binary 跑通完整 genesis(产 pinned_pubkeys/initial_q_state/genesis_report,
   签名 typed-tx,ChainTape L4 推进,每 attempt 有 tape commit),否则先修 A0 不放大。
G1: 实现 A1+A2(+A5)后重跑 flask 三元组 loop 臂(full-flow binary),patch 须到达 test 阶段(不卡 apply)对多数尝试成立;
    且确认 §5.5 防御生效(thinking=disabled/无 DSML-leak 静默失败/无 reasoning_content 400);
    若仍卡 apply,先分离诊断 DSML-leak/reasoning-400/malformed-diff,修 A2/§5.5,不放大。
    ⚠️前轮曾过度宣称单次随机 apply(未复现)——先验证再写。
G2: n=50 每实例须 gold-resolve,n_gold_valid/50 披露。
G3: FC1 不变量 evaluator_reported_completed_llm_calls = step+parse_fail+llm_err,失败即 HALT。

== 坑 ==
后台 Bash cwd 重置到 $HOME(先 cd 仓库);TDMA header 丢则内核永不 Proceed;sanitized 剥代理→HF 必 offline;
qemu 慢+SSL flake→可续跑;不提交 handover/evidence sidecar(CI no-sidecar检查,只提交代码+manifest+报告);
liveness gate 用 fs::read_dir→干净 checkout 验证,新脚本/模块先注册;写 evidence 前调 /runner-preflight;
批跑中不碰 trust-root 文件;**flash 默认 thinking-ON(必显式 disabled,"none"不够)**;**DSML-leak 须 client-side 检测+恢复**;
**full-flow binary 不自带 trust-root 校验(不夸大)**;**genesis-per-task 须每实例独立 runtime_repo(NonEmptyRuntimeRepo fail-closed)**;
**外化循环收益前2轮占76-95%(勿靠加迭代补偿弱 flash)**。

第一步：跑 `gh pr list --state open --json number,headRefName,title,files` 查路径冲突,
然后**开 branch**,起 A0 的 charter,干净 worktree 上实现,到 G0/G1 门停下等 flask 三元组验证结果。
```

---

## 附录：本 v3 相对前轮 handoff 与早期版本的关键校正

**A. 四指令带来的结构性订正**

1. **D1 措辞订正——删除「真 git tape」框架**：早期版本曾用「真实 git tape」「真 git tape」措辞，暗示存在「假 git tape」。**v3 全删**。订正后：git-tape/ChainTape 是每次内核运行的宪法法律（Art. 0.2 constitution.md:54「所有信号必须可从 tape 重建」；Art. 0.4 :149「Phase E gate 强制 B」）。loop-vs-control = 「进内核(依法发 tape)」vs「裸模型从不进内核(无 tape)」。无 tape 内核运行违宪且架构不可能（step_forward 无条件 commit，tdma_runner.rs:644 / memory_kernel.rs:162-207）。明确：`MemoryTapeLedger` 不是假 tape（全功能 in-memory 实现，commit 相同 TapeNode，仅不持久化 git）；`mock-model` 仅在 `#[cfg(test)]`（tdma_runner.rs:896/937）。见 §3.2、§5.4。
2. **D2 订正——worker 恒 flash，废 thinking-on worker 铁律**：早期 worker = `deepseek-v4-pro, thinking=on`。**v3 改 `deepseek-v4-flash, thinking 显式 disabled`**，因 ChainTape/git-tape 就是外化 CoT（与 Art. 0.2 呼应）。铁律改为「worker = flash, reasoning 外化到 tape」。cost-per-resolved 升为 headline（flash $0.14/M in vs Pro $0.435/M；cache-hit $0.0028/M ≈ 50x）。揭示三张力：Atom 2 更承重（flash format 弱）、tape 须富化（Atom 5，当前仅失败测试名级）、localization 是循环前瓶颈（pre-loop，tape 纠不回）。见 §1.3、§3.5、§6.1。
3. **D3 订正——meta 可 flash + thinking 未硬编码**：findings 核实 meta 在 swebench loop 仅轻量/确定性谓词路由 + patch 生成（loop 只有一个 agent，--role 选槽默认 meta cmd_tdma.rs:306；predicate 100% 确定性无 LLM，swebench_test_judge.rs:159；retry 逻辑确定性 Rust tdma_runner.rs:582，无第二个 meta-level LLM 调用）→ 强 flash 足矣，默认 meta 也 flash+disabled。**订正前轮「thinking: None 硬编码」探针**：当前 HEAD thinking per-role 可配（cmd_tdma.rs:445 读，:583/:591 透传；read_*_thinking cmd_llm.rs:508/527）。见 §3.4。
4. **D4 订正——从 genesis 全流程进入，归因边界改变**：早期 loop 臂用 `tdma run --judge swebench`（旁路 genesis/Sequencer/CAS/typed-tx，只 boot MemoryKernel tdma_runner.rs:541）。**v3 改走 full-flow binary** `swebench_live_coding_repair_current_kernel`（已核实存在；调 build_chaintape_sequencer_with_initial_q runtime/mod.rs:706:452，签名 typed-tx，GenesisReport :564）。**科学主张归因边界随之改变**：Δ 归因「完整 TuringOS vs 裸 flash」整体，不可分解子组件(§1.2，需后续消融 §1.4)。genesis-per-task 成本可忽略(<1s I/O，被 LLM latency 淹没，§4.5)。诚实:full-flow binary 不自带 trust-root 校验(§3.3.1)。见 §3.3、§4.5、§7 Atom 0/3、§8 G0。

**B. 前轮 handoff seam 漂移校正（PRESERVE，已复核仍成立 @ 1f00012d）**

5. **`scripts/probe_swebench_fetch_sample.py` 不存在**（前轮 handoff 及 Atom 1 假设其存在）。已核实 scripts/ 仅有 `probe_bare_v4_swebench.py`、`probe_swebench_expand.sh`、`probe_swebench_goldsmoke.sh`、`probe_swebench_loop.sh`、`run_true_suite_swebench_current_kernel.sh`。Atom 1 须新建并注册 script_liveness。
6. **judge 子区 seam 校正**（前轮「170-193」混三区）：extract_patch 调用 `:161` / predictions 写 `:176-193` / run_sanitized `:236`(cmd `:200`) / resolve→Pass `:289`(读 resolved `:288`) / FAIL_TO_PASS 抽取 `:292-311` / apply-gate 失败分支 `:265-280` + harness_failure_reason `:321`。
7. **裸臂 SYS prompt 注释漂移**：`probe_bare_v4_swebench.py` 注释指 `cmd_tdma.rs:249`，但 `SWEBENCH_SYSTEM_PROMPT` const 实际在 **:252**。裸臂 SYS 当前在 :30 且与 loop 臂不对称（§10 头号混淆）。
8. **Class 2 见证学说更新**：旧「Codex 见证」已被 2026-05-29 批准的**平台无关 clean-context 审计**取代（要求 = 干净上下文 + 无实现 transcript，非特定厂商）。
9. **OpenClaw / Hermes 澄清**：二者均非 SWE-bench Verified 平台选手，坐标表不收录其 Verified 行；最近真实开源编码平台替代物为 OpenHands / SWE-agent。

**C. DeepSeek V4 Flash 存在性与口径的诚实标注（findings 核实）**

10. **DeepSeek V4 Flash 确实存在**（高置信，多一级源）：2026-04-24 发布（与 V4 Pro 同期），model ID `deepseek-v4-flash`，284B total / 13B active MoE（vs Pro 1.6T / 49B），1M context / 384K max output，function calling 在 thinking 与 non-thinking 模式皆支持，HuggingFace `deepseek-ai/DeepSeek-V4-Flash` 开源权重。legacy 别名 `deepseek-chat`→flash non-thinking、`deepseek-reasoner`→flash thinking（2026-07-24 退役）。**与任务假设一致——此名下确有其物**。
11. **SWE-bench 口径诚实标注**（中高置信）：V4 Flash 厂商自报 79.0% Verified，**很可能 thinking-enabled 默认口径**；**无公开的 thinking=disabled 直测值**。AA Intelligence Index non-thinking 36 vs Think-Max 47 暗示 disabled 口径明显更低。我们用 disabled，故**不可**用 79.0% 设期望(§1.6.3、§2.2、§10)。
12. **flash 默认 thinking-ON 是非显然陷阱**（高置信，LiteLLM #27453 三条件对照）：「Flash」≠ out-of-the-box 非 thinking；必须显式 `{"thinking":{"type":"disabled"}}`（`reasoning_effort:"none"` 不够）。这是 v3 多处承重新坑(§1.3、§3.6、§5.5、§9.6 坑 12)。
13. **DSML 工具格式泄漏**（高置信，多独立一级源：HF model card / deepseek-ai#1244 / sglang#14695 / openclaw#85918 / NVIDIA forum）：V4 原生 DSML（`｜DSML｜` token + `string="true|false"` 属性）经 OpenAI 兼容层泄漏进 content 致 tool_calls null 静默失败，第三方 wrapper（NIM/Foundry）可能 100% 失败（非 11-21% 间歇）。前轮 flask 0% apply 的可信主因之一（与 malformed-diff 不互斥，可复合，无 agent log 不能定论谁主导）。须 client-side 检测+恢复(§5.5、§9.6 坑 13)。

**D. externalized-CoT 先例诚实标注（findings 核实，中高置信）**

14. **flash+外化循环逼近 thinking 模型——条件性成立**：结构化可验证任务（HumanEval/LiveCodeBench/MBPP）外化循环闭合 60-80% 差距（S\*、Reflexion、7-model self-repair 研究）；**但真实 SWE-bench 仍约 14pp 持久差**（CodeMonkeys Claude 3.5 non-thinking 57.4% vs o3 71.7%）。论题在「结构化可验证 + tape 携带真实 deliberation」时成立，在「多文件定位 / 逻辑错误 / 纯 action-log tape」时退化。这是对 §1.1 主张量级保持谦逊的实证基础。
15. **tape-with-rationale vs without 的同数据集 ablation 文献中不存在**（截至 2026-05）：故 §1.4-B（tape 富度消融）既是潜在科学贡献，也意味着无现成基线可借——本 campaign 主 Δ 用单一固定 tape 配置，不在主 Δ 内混两配置。

**E. 其余确认无漂移的 seam（PRESERVE）**：`SwebenchSampleInput` cmd_tdma.rs:37 ✅、`SWEBENCH_SYSTEM_PROMPT` :252（含 tdma-state-update/v1 header + ---BODY---）✅、`make_swebench_user_prompt` :259 ✅、`WilsonCi::new_95` wilson_ci.rs:36 ✅、`run_proof_with_ledger` tdma_runner.rs:509 ✅、`run_proof` :495 ✅、`extract_body` :426 ✅、HF offline judge:228-229 ✅、`similar` crate 缺席 ✅、`build_chaintape_sequencer_with_initial_q` runtime/mod.rs:706 ✅、`NonEmptyRuntimeRepo` :731 ✅、per-role thinking cmd_tdma.rs:445 / cmd_llm.rs:508/527 ✅、full-flow binary `swebench_live_coding_repair_current_kernel.rs` 存在（genesis:452 / resume:false:450 / GenesisReport:564）✅、当前分支 main @ 1f00012d ✅、Art. 0.2 constitution.md:54 ✅、Art. 0.4 constitution.md:149 ✅、cmd_init.rs:570「No sequencer call. No typed_tx. No CAS write. No ChainTape advance.」✅。