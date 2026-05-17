# Phase 3 / Track H — Software 3.0 / Agentic 工程

## 0. forward-bound 警告 + scope (聚焦差异, 不综述)

**Forward-bound**: 全部资料均为 2025-2026 公开发布 (Karpathy YC AI Startup School 2025-06, Anthropic Engineering 2025-09/2025-11, Letta V1 2025, Replit Agent 4 2025-12, Cognition Windsurf 2.0 2026-04)。结论形式为"TuringOS 命题 vs commercial pattern 的差异点", 不重写 Karpathy / Anthropic 框架综述。

**Scope 边界**:
- **In**: Software 3.0 / agentic engineering 工程模式、长期记忆架构、agent CI/CD、commercial agent platform 对比。
- **Out**: 多 agent 通信协议 (Track B 范畴)、Software 3.0 综述 (用户上传报告已覆盖)、Karpathy 哲学 long-form 解读。
- **目标**: 识别 TuringOS tape-first / constitutional / market-economy substrate 在 2025-2026 frontier 中**未覆盖的工程实践层级 gap** + **TuringOS 对 Karpathy 命题的超越/退守判定**。

---

## 1. 任务 scope + WebSearch 清单

5 个核心问题 (Q1-Q5) + 6 个 WebSearch + 2 个 WebFetch 锁定原文。来源域:

- karpathy.bearblog.dev (2025 LLM Year in Review)
- ycombinator.com/library, latent.space (Karpathy YC talk 一手转述)
- anthropic.com/engineering (Effective Context Engineering 2025-09, Effective Harnesses 2025-11, Managed Agents, Harness Design for Long-Running Apps)
- letta.com, github.com/letta-ai/letta (MemGPT → Letta V1 架构)
- mindstudio.ai, thenewstack.io (vibe coding vs agentic engineering 评论)
- braintrust.dev, langwatch.ai, latitude.so (agent observability 2026 工具景观)
- techcrunch.com, devops.com, cognition.ai (Devin / Windsurf 2.0 / Replit Agent 4)

---

## 2. Karpathy 2025-2026 关键演讲摘要 (含 URL)

### 2.1 YC AI Startup School 2025-06 "Software Is Changing (Again)"

主张 (核心命题):

1. **三时代分层**: Software 1.0 = 显式规则代码; Software 2.0 = 学习权重; Software 3.0 = **prompt 即程序**, LLM 是解释器。
2. **LLM as OS**: 模型权重 = CPU; **context window = RAM**; tool/MCP = 系统调用; 个体 LLM 实例像跑在 mainframe 上的 batch job。
3. **Decade of Agents**: "2025 isn't the year of AI agents, it's the decade" — 反对 imminent-AGI 叙事; 当前 LLM 缺失 continual learning, 弱多模态, 不会用电脑; AGI 宏观冲击会渐扩散为 ~2% GDP 增长, 不是单点跳跃。
4. **Partial Autonomy Sliders**: 反对 full-autonomy agent; 主张 cursor tab → cursor agent → 长任务的**滑块式控制**, "AI on tight leash" + "the faster the loop the better"。
5. **Demo-Product Gap**: "Demo is `works.any()`, product is `works.all()`" — 演示偶尔工作, 产品要求全场景可靠; 这是当前 agent 工程的核心 gap。
6. **Agent 缺失**: anterograde amnesia (长期记忆缺失), "system prompt learning" 这条新范式还未存在 (策略级学习应可显式编码, 不只靠 weight update); 文档/工具是为人类写的 (建议 `llms.txt` for machines)。

### 2.2 Karpathy 2025 LLM Year in Review (karpathy.bearblog.dev)

- December 2025 inflection point: 模型代码质量稳定到 "I can't remember the last time I corrected the model"。
- Cursor 兴起标志 LLM 应用层 (specialized orchestrator)。
- Claude Code 是第一个 functional LLM agent (tool use + reasoning 长链)。
- "vibe coding" → "agentic engineering" 转变: 价值从语法/实现转移到判断、品味、监督; 单回合协作 → 多步自治工作流。

### 2.3 Karpathy Sequoia Ascent 2025 5 大预测 (mindstudio.ai 转述)

- 模型质量将稳定到不需要人为修正; 瓶颈转移到**测试基础设施 + 反馈循环**。
- "Agentic engineering 的最大区分点是 testing relentlessly"。
- 工程师价值从"写代码"转为"verify-correct loop 的设计者"。

---

## 3. Anthropic Effective Agents framework (差异层级)

### 3.1 Context Engineering (2025-09)

核心命题: 把信息当成 "finite resource with diminishing marginal returns", 目标是 "smallest possible set of high-signal tokens"。原语:

- **Just-in-Time Loading**: 不预加载, 持有 lightweight identifiers (file paths, queries, links), 运行时按需取。
- **Memory Tool**: 文件系统外置存储, 跨 session 累积知识。
- **Structured Note-Taking**: NOTES.md / progress.txt 等持久化笔记。
- **Sub-Agent Architectures**: clean-context 子 agent 处理 focused 任务, 返回压缩 summary 给协调主 agent。
- **Compaction**: 长任务总结历史并重新初始化, 保留架构决策, 丢弃冗余输出。

### 3.2 Effective Harnesses for Long-Running Agents (2025-11)

核心问题: long-running agent **每次 session 都从零开始**, 没有记忆衔接; compaction 不足以传完整指令。Anthropic 解法:

- **Initializer agent + coding agent 二段式 harness**: initializer 设置 feature list / git repo / progress 跟踪文件; coding agent 增量推进, 维护干净代码状态。
- **claude-progress.txt + git history** 是 fresh-context 重启的恢复手柄。
- Browser automation (Puppeteer 等) 作为 verification loop。

### 3.3 Building Effective AI Agents (workflow vs agent)

- **Workflow**: 预定义的多 LLM 调用图 (orchestrator-worker, evaluator-optimizer, routing, prompt chaining), 路径固定。
- **Agent**: LLM 在 tool-use loop 中**自主决定下一步**, 路径开放, 终止条件由 agent 判断。
- 建议: 大多数生产场景 workflow 已足够; agent 仅在**开放任务空间 + 多步推理 + 工具调用不可预先排序**时才必要。

---

## 4. Q1-Q5 (核心比较与差异)

### Q1: Karpathy Software 3.0 vs TuringOS — 哪些对齐, 哪些更激进/保守?

**对齐**:
- LLM as OS / context = RAM: TuringOS 的 ChainTape + CAS + HEAD_t 是**比 Karpathy 概念图更显式的 OS 抽象** (有真正的 paging、commit、replay)。
- Partial autonomy: TuringOS 的 predicate gate + L4.E rejection 就是 Karpathy 描述的 "tight leash" 在 sequencer 层的物化。
- Demo-Product Gap: TuringOS 的 `works.all()` 标准 = 宪法 21 条 + FC1 attempt 等式 + 122+ constitution gates。

**TuringOS 更激进**:
- Karpathy 留在**编辑器 UI 层 "autonomy slider"**; TuringOS 已把 slider 下沉到**协议层** (typed_tx + sequencer admission + RootBox)。
- Karpathy "system prompt learning" 仍是空范畴; TuringOS 的 `PromptCapsule` + `AttemptTelemetry` + L4 anchor 已是**显式可审计的策略记账机制**。
- Karpathy 没有谈 economic conservation; TuringOS 的 1 Coin = 1 YES + 1 NO + escrow + on_init-only mint 是**用经济不变量替代规范性话语**的工程实现。

**TuringOS 更保守 (退守点)**:
- Karpathy 的 LLM-as-OS 包含**应用层 UI 演化** (Cursor → Replit → Devin 这一脉); TuringOS 还没有面向最终用户的"打开就能用"应用层, `lean_market` CLI 仍是研究者工具。
- Karpathy 谈"渐扩散 2% GDP 增长"暗含**部署到主流开发流**; TuringOS 集中在 MiniF2F / Lean 形式证明, 主流软件工程域未覆盖。

### Q2: Anthropic Effective Agents (workflow vs agent) vs TuringOS REAL-5 role-based scaffolding

**Anthropic workflow/agent 二分** (路径预定 vs 自主决定下一步) 在 TuringOS 表现:

- TuringOS 的 REAL-5 librarian / market-maker / lean-prover 角色其实是**workflow 模式** (Anthropic 定义) — 角色边界、tool 集合、admission 规则**预先编码**, 由 sequencer 强制执行。
- TuringOS **没有真正的 "agent 模式"** (Anthropic 定义): 因为 sequencer 不允许 agent 自主决定下一步状态转换, 所有 transition 都要过 predicate gate。
- **差异结论**: TuringOS 是 "**constitutional workflow with agent-shaped LLM nodes**" — 比 Anthropic 的 workflow 更刚 (有 RootBox / 签名 / replay), 比 agent 更窄 (路径不能由 LLM 决定)。

**REAL-5 specifically**: role 是 scaffold 而非 emergent specialization; Anthropic Sub-Agent 是动态分派, TuringOS 角色是静态登记 (`AgentRegistry`)。两者解决不同问题: Anthropic 解 context 隔离, TuringOS 解 capability 授权 + 经济责任归属。

### Q3: 长期记忆 (MemGPT / Letta V1) vs TuringOS PromptCapsule + AgentReputation

**Letta V1 (2025) 架构**:
- LLM-as-OS paradigm: 把 context 当 RAM, 自带 paging (memory_replace, memory_insert, archival_memory_*, conversation_search)。
- 跨 500+ 交互保持 task context (vs RAG baseline 50 步就 fragment)。
- 把 send_message / heartbeat 删了, 用 native reasoning + 直接 assistant message。
- 商业目标: stateful agent 持续 30 天的 long-horizon 任务。

**TuringOS PromptCapsule + AgentReputation 已有**:
- `prompt_context_hash + read_set + policy_version + visible_context_cid + agent_view_manifest_cid` — 输入侧记忆有据。
- `AttemptTelemetry` + `LeanResult` — 输出侧每次尝试有可重放记录。
- `AgentReputation` (REAL-12 经济专精) — 跨 session 的角色资产记账。

**TuringOS 缺什么**:
- **没有 archival_memory_* 等价**: PromptCapsule 是 **per-attempt 输入快照**, 不是 cross-session **可主动 query 的累积知识库**。Agent 不能"翻看上周做过什么类似题目"。
- **没有 self-editing memory tool**: Letta 让 agent 自己编辑 memory block (memory_replace 等); TuringOS 的 prompt 装配仍由 librarian role 集中分发, agent 不能自己保留笔记。
- **没有 Anthropic NOTES.md / progress.txt 等价**: 没有 agent-writable 持久化 workspace 让 long-running task 在 fresh context 时恢复。
- **结论**: TuringOS 的记忆是**审计向** (可证伪、可重放、可经济归属), Letta 的记忆是**能力向** (可主动 recall、可自编辑)。两者方向不同, TuringOS 在能力向严重不足。

### Q4: Replit Agent 4 / GitHub Copilot Cloud / Devin 2.0 / Windsurf 2.0 vs TuringOS verifiable substrate

**Commercial pattern 2025-2026 实情**:
- **Replit Agent 4** (2025-12): 自然语言 → 完整 app 工程; 30+ connectors + MCP server 支持几百工具; 平面 ideation/design/code/deploy。
- **GitHub Copilot Cloud Agent**: agent mode 2025 全量发布; Claude 3.5/3.7 + Gemini 2.0 Flash 多模型 + MCP 接 DevOps stack。
- **Devin 2.0 + Windsurf 2.0** (Cognition 2025-07 收购): SWE-bench 13.86% (Devin) / 40.08% (Windsurf SWE-1.5); 集成成 Windsurf 2.0 + Agent Command Center + Devin cloud agent (2026-04)。

**用户视角对比**:

| 维度 | Commercial (Replit/Copilot/Devin) | TuringOS |
|---|---|---|
| 上手时间 | 分钟级 (打开 URL) | 周级 (理解 ChainTape / constitution) |
| 任务面 | 一般软件工程 (web app, CRUD, 脚本) | MiniF2F 形式证明 / 受限实验 |
| 验证机制 | test suite + LLM-as-judge + 人 review | 宪法 gate + Lean predicate + L4.E + replay |
| 失败可审计性 | trace + log (vendor-managed) | ChainTape/CAS/replay (用户可独立验证) |
| 用户友好度 | **高** (vibe coding 用户可直接用) | **低** (要求宪法理解) |
| 责任归属 | vendor (Replit/GitHub/Cognition) | 协议 (sequencer admission + escrow) |

**判定**: TuringOS 在**普通用户友好度上明显落后**, 但在 **"agent 失败可被独立第三方追责"**上 unique。Commercial agent 的 "verify" 主要依赖 vendor 内部 trace + 用户人工 review; TuringOS 的 verify 是**协议级 replay** (任意第三方拿 genesis + ChainTape + CAS 就能重现)。
当前 commercial pattern 的 Q3 2025 真实事故 (Amazon 90 天 reset on code deployment controls after AI-coding 高 blast-radius incidents — thenewstack.io 报道) 恰好是 TuringOS 命题想避免的失败模式。

### Q5: Agent CI/CD (testing/observability/deployment) vs TuringOS audit_tape + replay_full_transition

**2026 agent observability 工业标准**:
- 89% 组织已部署 agent observability; 62% 有 step-level tracing (来源: State of Agent Engineering)。
- Braintrust + Langfuse + Latitude + LangSmith 主导市场 ($2.69B → $9.26B 2030)。
- **CI/CD 模式**: GitHub Actions 跑评测 on every PR, 阻断质量降级。**Layered eval**: 先 deterministic check (JSON schema / regex) → heuristic score → LLM-as-judge。
- 离线评测用 curated golden cases, 同样定义在 local + CI + production 都跑。

**TuringOS 现状**:
- `bash scripts/run_constitution_gates.sh` + 122+ constitution test = **constitutional CI** (类似 LangSmith 的 golden cases 但更严: 用 Lean 真证 + replay 等式而非 LLM-as-judge)。
- `audit_tape` + `replay_full_transition` = **replay-able trace**, 比 Braintrust step-level tracing 更**协议级**。
- `AttemptTelemetry` 等式 (FC1 invariant `evaluator_completed = L4 + L4.E + capsule-anchored`) = **强于 OpenTelemetry 的 trace 完整性保证**。

**TuringOS 缺什么**:
- **没有 Braintrust 式 "eval-driven development" 工作流**: TuringOS gate 写在 `tests/constitution_*.rs`, 但**没有数据驱动的 eval-as-product** (eval 集本身是 versioned 资产, 可以 PR review, 可以 A/B 部署 prompt 版本)。
- **没有 LLM-as-judge 路径**: TuringOS predicate gate 全是 deterministic (Lean predicate / 经济不变量), **没有承认主观判断的层**。但 Karpathy 命题预言**测试基础设施会是新瓶颈**, 主观任务 (UX / code review / paper review) 需要 layered eval (deterministic → heuristic → LLM-judge)。TuringOS 没有 layered 中段。
- **没有 production deployment gate**: constitution gate 是 PR-time gate; **没有 production canary / shadow deploy / quality regression 实时告警**。
- **没有跨 run aggregate observability dashboard**: 单 run 有 ChainTape, 但**没有 dashboard 跨 100+ run 的趋势可视化** (用户必须 grep TB_LOG.tsv)。

---

## 5. TuringOS 独特性 vs commercial agent platforms

| 维度 | TuringOS 独特点 | 业内 SOTA 对应 |
|---|---|---|
| 协议级可审计 | ChainTape + CAS + replay_full_transition | Braintrust step trace (vendor-managed) |
| 经济责任归属 | 1 Coin = 1 YES + 1 NO + escrow + slashing | (无对应; commercial 全是 vendor SLA) |
| 宪法 gate | 122+ 可失败 test 锁宪法 clause | Anthropic harness (有指南, 无强制 gate) |
| Predicate-based 拒绝 | L4.E rejection evidence 是一等公民 | LLM-as-judge (主观, 不可重放) |
| Capability 显式登记 | AgentRegistry + 签名 + system-key pinning | OAuth/MCP (集中授权; 无第三方审计) |
| Tape-as-memory | PromptCapsule + AttemptTelemetry 输入审计 | Letta archival_memory (能力向, 不可审) |

**最大 unique 命题**: TuringOS 是**唯一把"agent 失败可被独立第三方追责"作为协议级保证**的 substrate。Commercial agent 的所有 verify / observability 都依赖 vendor 内部, 不允许用户自己重放协议来追责。

---

## 6. TuringOS 缺什么清单 (Top 5)

按优先级 (forward-bound; 不立即实施, 留给 Phase 4-5):

1. **Agent-writable 累积记忆层** (能力向, 非审计向): 等价于 Letta archival_memory / Anthropic NOTES.md。当前 PromptCapsule 是 read-only 输入快照, agent 不能"翻看上周做过什么类似题"。**Phase 4 不动**, 但要在 archive 里记一条 "TuringOS 是能力受限的 audit-first substrate"。
2. **Layered eval 中段 (LLM-as-judge 受控引入)**: deterministic Lean predicate + 经济不变量是底层; 但 Karpathy 命题预言主观任务 (UX, code review) 需要 layered eval。TuringOS 缺 deterministic 与人之间的 LLM-as-judge 受控层, 且这一层必须**带 PromptCapsule + replay** 才不破宪法。
3. **跨 run aggregate observability dashboard**: 当前 dashboard 是**单 run materialized view**; 缺**100+ run 趋势可视化** (类似 Braintrust 的 eval over time)。要求**仍从 ChainTape/CAS 重新生成**, 不能成为 source of truth (CLAUDE.md §17 守住)。
4. **面向最终用户的"打开即用"应用层**: lean_market CLI 仍是研究者工具; 缺类似 Replit Agent 4 的 "natural language → working artifact" 用户入口, 但 backed by ChainTape/CAS verifiability。Phase 5 deliverable 候选。
5. **Initializer / coding agent 二段式 harness (Anthropic 2025-11 模式)**: 当前 long-running task 在 fresh context 时**没有恢复手柄** (没有 claude-progress.txt 等价)。TuringOS 的 ChainTape 已是 ultimate progress file, 但**没有让 agent 主动消费 ChainTape 状态的 protocol** — `HEAD_t` 只给 sequencer 用, agent prompt 仍要由 librarian 装配。

---

## 7. 给 Phase 4-5 建议

**Phase 4 (synthesis)**:
- 对 Top 5 缺口, 各给一段"forward-bound 注脚": 当前不做, 但 Phase 5 deliverable 要明确**是否纳入路线图 + 优先级**。
- 用 Q4 表格 (commercial pattern 对比) 作为**用户视角差异定位**的主图: TuringOS 不是 Replit/Devin 的替代品, 是**互补 substrate** (commercial 提供易用性, TuringOS 提供协议级问责)。
- 用 Q5 (agent CI/CD) 作为 **lean_market CLI 设计的工程对照**: lean_market 必须从一开始就采纳 layered eval (deterministic → heuristic → LLM-judge), 不能只有 Lean predicate 一层。

**Phase 5 (deliverables)**:
- 强烈建议 deliverable 包含一份 "**TuringOS positioning relative to Karpathy Software 3.0**" 单页 — 因为本 track 已确认 TuringOS 命题**对齐 Karpathy 宪法话语但在工程实现上更深**, 这是一个可对外讲清楚的差异点 (不只对架构师有用, 对潜在合作者 / 资本方也有用)。
- lean_market user CLI 设计建议优先采纳: just-in-time loading (Anthropic 原语) + structured note-taking (NOTES.md 类) + partial-autonomy slider (Karpathy 命题), 这三个原语**不破 TuringOS 宪法**, 但能极大提升用户友好度。

---

## 8. References

### Karpathy 2025-2026
- [Andrej Karpathy: Software Is Changing (Again) — YC AI Startup School 2025-06](https://www.ycombinator.com/library/MW-andrej-karpathy-software-is-changing-again)
- [Andrej Karpathy on Software 3.0 — Latent Space 深度转述](https://www.latent.space/p/s3)
- [Karpathy 2025 LLM Year in Review](https://karpathy.bearblog.dev/year-in-review-2025/)
- [Karpathy's Sequoia Talk: 5 Predictions About Agentic Engineering — MindStudio](https://www.mindstudio.ai/blog/karpathy-sequoia-talk-5-predictions-agentic-engineering)
- [Software 3.0 Explained: Why Karpathy Says the Context Window Is Your New RAM — MindStudio](https://www.mindstudio.ai/blog/software-3-0-explained-karpathy-context-window-ram-model-weights-cpu)
- [Vibe Coding vs Agentic Engineering — Karpathy's Framework — MindStudio](https://www.mindstudio.ai/blog/vibe-coding-vs-agentic-engineering-karpathy-framework)
- [From vibes to engineering: How AI agents outgrew their own terminology — TheNewStack](https://thenewstack.io/vibe-coding-agentic-engineering/)

### Anthropic Engineering 2025
- [Effective Context Engineering for AI Agents (2025-09)](https://www.anthropic.com/engineering/effective-context-engineering-for-ai-agents)
- [Effective Harnesses for Long-Running Agents (2025-11)](https://www.anthropic.com/engineering/effective-harnesses-for-long-running-agents)
- [Harness Design for Long-Running Application Development](https://www.anthropic.com/engineering/harness-design-long-running-apps)
- [Building Effective AI Agents (resources)](https://resources.anthropic.com/building-effective-ai-agents)
- [Scaling Managed Agents: Decoupling the Brain](https://www.anthropic.com/engineering/managed-agents)
- [Claude Cookbook: Context Engineering — memory, compaction, tool clearing](https://platform.claude.com/cookbook/tool-use-context-engineering-context-engineering-tools)

### Long-term agent memory
- [MemGPT is now part of Letta — Letta Blog](https://www.letta.com/blog/memgpt-and-letta)
- [Rearchitecting Letta's Agent Loop: Lessons from ReAct, MemGPT, & Claude Code — Letta V1](https://www.letta.com/blog/letta-v1-agent)
- [Letta Concepts — MemGPT — Letta Docs](https://docs.letta.com/concepts/memgpt/)
- [Mem0 vs Letta vs MemGPT 2026: Agent Memory Layer Comparison — TokenMix](https://tokenmix.ai/blog/ai-agent-memory-mem0-vs-letta-vs-memgpt-2026)
- [Agent Memory & Knowledge Systems Compared (2026 Guide)](https://fountaincity.tech/resources/blog/agent-memory-knowledge-systems-compared/)

### Commercial Agent Platforms 2025-2026
- [What Is Replit Agent 4? — MindStudio](https://www.mindstudio.ai/blog/what-is-replit-agent-4)
- [Replit Agent 4: Replit Just Changed Everything (2026-03)](https://atalupadhyay.wordpress.com/2026/03/19/replit-agent-4-replit-just-changed-everything/)
- [Best of 2025: GitHub Copilot Evolves: Agent Mode and Multi-Model Support — DevOps.com](https://devops.com/github-copilot-evolves-agent-mode-and-multi-model-support-transform-devops-workflows-2/)
- [Cognition, maker of Devin, acquires Windsurf — TechCrunch (2025-07)](https://techcrunch.com/2025/07/14/cognition-maker-of-the-ai-coding-agent-devin-acquires-windsurf/)
- [Cognition AI Launches Windsurf 2.0 with Agent Command Center and Built-in Devin Cloud Agent (2026-04)](https://www.kucoin.com/news/flash/cognition-ai-launches-windsurf-2-0-with-agent-command-center-and-built-in-devin-cloud-agent)
- [Devin in Windsurf — Cognition](https://cognition.ai/blog/devin-in-windsurf)
- [Best AI Coding Agents in 2026: Ranked and Compared — Codegen Blog](https://codegen.com/blog/best-ai-coding-agents/)

### Agent CI/CD & Observability 2026
- [Agent Observability: The Complete Guide for 2026 — Braintrust](https://www.braintrust.dev/articles/agent-observability-complete-guide-2026)
- [4 Best Tools for Monitoring LLM & Agent Applications in 2026 — Langwatch](https://langwatch.ai/blog/4-best-tools-for-monitoring-llm-agentapplications-in-2026)
- [LLM Testing Tools and Frameworks in 2026 — ContextQA](https://contextqa.com/blog/llm-testing-tools-frameworks-2026/)
- [Best AI Agent Observability Tools in 2026 — Latitude](https://latitude.so/blog/best-ai-agent-observability-tools-2026-comparison)
- [Ultimate Guide to CI/CD for LLM Evaluation — Latitude](https://latitude.so/blog/ultimate-ci-cd-llm-evaluation-guide)
- [Agent Evaluation Readiness Checklist — LangChain](https://blog.langchain.com/agent-evaluation-readiness-checklist/)
- [AI Agent Observability: Tracing, Testing, and Improving Agents — LangChain](https://www.langchain.com/articles/agent-observability)
