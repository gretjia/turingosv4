# TISR Reference Inputs

**本文件为 forward-bound research 提案；任何派生 TB 须独立 §8 ratification；本文档不构成已批准方案。**

本文件归档 TISR 项目启动时用户提供的所有 reference 输入材料，作为 Phase 1-5 工作的事实基础。

---

## 1. 用户上传报告: 《TuringOS 生成式 HTML 核心交互层研究报告》

**提交时间**: 2026-05-17 (TISR 项目启动同日)
**报告形态**: 用户在 plan-mode 启动 prompt 中直接粘贴的完整文本
**报告主旨** (Executive Summary verbatim 摘录):

> 我需要为 TuringOS 设计一个 **Generative UI**，并把 **生成式 HTML** 提升为首要用户入口，用它替代传统文字化 CLI，同时保留表格、图表、mindmap、可编辑表单、可拖拽节点、多模态内容、工具调用、审计与回放等能力。

### 1.1 报告核心论点

1. **不允许模型直接输出任意 HTML/JS**: 应建立 `Turing UI IR` (中间表示)，由受控 UI Materializer 物化
2. **四个长期维护核心**: Turing UI IR / Event Bridge / Policy Engine / Audit Store
3. **HTML-first Workspace 替代 CLI**: CLI 降级为专家回退入口
4. **React 主壳 + Web Components 边界层 + 组件白名单 + 受控事件桥**
5. **三类原型对比**:
   - HTML-first 工作台 (3 个月 MVP, **首选主线**)
   - Canvas / Flow 工作台 (6 个月扩展)
   - Artifact Studio (6-12 个月)
6. **安全四层**: 内容层 (IR 白名单) / 渲染层 (CSP + Trusted Types) / 动作层 (Event Bridge + Typed Action ABI) / 执行层 (Policy + Approval Gate)
7. **协作演进**: MVP WebSocket → 6-12 月引入 Yjs/CRDT
8. **模型路由**: 主规划 / 低成本 / 私有化三层

### 1.2 报告引用的关键外部论文

- **Magentic-UI: Towards Human-in-the-loop Agentic Systems** — co-planning / co-tasking / action guards / multi-tasking / long-term memory
- **Design2Code: Benchmarking Multimodal Code Generation** — 自由生成整页代码仍不稳
- **AutoGen / AutoGen Studio** — 多代理编排
- **WebGPT** — 搜索-引用-回答证据绑定工作流
- **MCP at First Glance** — MCP 安全研究
- **MCP Safety Audit** — tool poisoning / prompt injection / resource injection

### 1.3 报告引用的关键产品/事件

- Karpathy: Sequoia Ascent 2026 / 2025 LLM Year in Review
- Anthropic: Building Effective Agents / Effective Context Engineering / Writing Effective Tools / MCP / Artifacts
- OpenAI: Apps in ChatGPT / Apps SDK
- Replit: Agent 4
- GitHub: Copilot Cloud Agent

### 1.4 报告的方法论 (我的判断)

- **优点**: 把"生成式 UI"框定为"协议优先而非组件优先"，与 TuringOS 物化思维契合
- **优点**: 安全四层是 production-ready 的，不是 demo-grade
- **优点**: 明确区分 workflow vs agent (Anthropic framing)
- **盲点 1 (auditor 已揭示)**: 几乎没有触及 **agent-to-agent 自治通信**（侧重单人-单 agent / 单人-多 agent，缺失多 agent-多 agent）
- **盲点 2**: 没有 TuringOS 经济模型集成 (CompleteSet / 价格信号 / role-scoped view)
- **盲点 3**: 没有 ChainTape / CAS 集成路径（提到 audit store 但未连到 L4/L4.E）
- **盲点 4**: "人 = agent 特例" 命题与 TuringOS 现状架构不兼容 (auditor 已论证)
- **盲点 5**: 多模态 agent / embodied AI / 长期 memory / world model 几乎未涉
- **盲点 6**: zkML / 可验证 AI / 去中心化 agent 基础设施未涉

这些盲点正是 TISR Phase 2-3 的研究方向。

### 1.5 报告全文存档

报告原文已在 TISR 启动 prompt 中完整保留 (~17000 字)。本文件不复制全文以避免冗余；Phase 4 gap_analysis.md 将直接引用报告章节做对照分析。

---

## 2. 用户原始诉求 (verbatim, TISR 启动 prompt)

```
in PLAN MODE: 设置新 WorkTree, 沿着我，这是一个大型的研究项目，
需要你组织和设计一个大型的研究计划，利用 multi agents。
首先要沿着我提供给你的这些思路，以及本项目 TuringOS v4 的实际的架构，
深入进行可落地的代码层面的研究和计划。
同时要组织单独的 agents，在一些前沿的相关领域进行横向拓展，
研究一下我是否在之前的计划中有遗漏的，或者是不足够优秀的思想。
我的目的是一个适合于未来 AGI 时代的，一个是与人类互动，
更加互动强的 Software 3.0 的这么一个交互体验，
同时要满足我的 TunOS 的真正内核，是一个让 AI agents 可以自由的在里面
交通、交汇、沟通，而不需要人类参与的一个全方位的一个计划方案。
```

**关键短语解析**:
- "可落地的代码层面" → Phase 2 五个 track 必须给具体 file:line 引用
- "横向拓展" → Phase 3 五个 track 必须找现有报告盲点
- "遗漏的，或者是不足够优秀的思想" → Phase 4 gap_analysis 是核心 deliverable
- "AGI 时代" → 不是 demo-grade 项目；要为 long-term 留余地
- "Software 3.0 ... 互动体验" → 轴 A
- "AI agents ... 交通、交汇、沟通，而不需要人类参与" → 轴 B
- 双轴并重，**不是单轴**

---

## 3. 用户 Plan-Mode 答复 (verbatim, 2026-05-17)

### 3.1 第一轮三个澄清问题答复

**Q1: 项目 scope 边界?**
> 研究 + 长期持有（未来阶段种子）

**Q2: 与 G-Phase 关系?**
> 独立平行轨道

**Q3: 交付后立即做什么?**
> 仅交付计划，等审批

### 3.2 Auditor CHALLENGE 后的修订答复

**Q1: 架构师 verbatim 冲突怎么处理?**
> 现阶段不需要架构师参与了，反正是在独立 worktree, 等 PR 时再让架构师审计。
> 我补充一件事，可以先研究一下是否先设计一个 cli，可以先使用上。
> 要把整体 turingos 的架构都完整连接上，从用户角度。

**重要含义**:
- worktree 物理隔离作为 G-Phase carve-out 的承诺机制
- **新需求**: CLI 先行设计（TISR Phase 1 新增）
- "从用户角度" → CLI 必须覆盖用户视角的完整工作流

**Q2: 轴 A "人 = agent 特例" 命题?**
> 现阶段"人"要批准 spec 和启动 init 模块，以后的 polymarket 环节可以让 Agent 自己接入，
> 但现阶段大概率还是人来接入，这个更符合现在阶段，但是 ai 时代可能来的比我们预期更快

**重要含义**:
- v2 命题应为**阶段化**: 现阶段人=spec 批准+init 启动；未来留 agent 自接入扩展
- "AI 时代可能来的比我们预期更快" → 设计要留 future-proof hooks，但不假设其立即成立

**Q3: 是否再派 audit agent?**
> 不需要，本轮审计足够

---

## 4. Auditor 第一轮审计输出 (TISR v1 plan)

**Verdict**: CHALLENGE / Confidence: High

**6 个 must-fix 项**:
1. 架构师 verbatim 冲突 (2026-05-14 line 147)
2. "人 = agent 特例" 命题不兼容
3. Class 4 候选风险预警缺失
4. Phase 1/2 与已存在 surface 重叠
5. Kill condition 不够具体
6. 0 real-problem witness 风险

**8 个弱项建议**:
- Track A scope 缩小
- Track C 明确 from-scratch web layer
- 工时上调 15-20h，并行批次 ≤3 agent
- WebSearch 预算 explicit 授权
- Track H/J 范围聚焦
- Phase 0 增加关系矩阵
- Phase 3 context 压缩策略

所有 6 个 must-fix + 8 个弱项已在 v2 Charter + Philosophy 中修复。

**Auditor file:line 引用** (用作 Phase 0 inventory 起点):

- `handover/directives/2026-05-14_TB_G_G_PHASE_CLOSEOUT_ARCHITECT_UPDATE.md:147` — 架构师 verbatim "现在不要再开新方向"
- `handover/directives/2026-05-11_G_PHASE_GENERATIVE_ARENA_DIRECTIVE.md:1` — ARCHIVED, awaiting authorization
- `src/state/typed_tx.rs:2327` — TypedTx 19 variants
- `src/state/sequencer.rs:4055` — SystemTxForbiddenOnAgentIngress
- `src/runtime/real5_roles.rs:730` — REAL-5 roles
- `src/runtime/market_decision_trace.rs` — MarketDecisionTrace
- `src/runtime/librarian_broadcast.rs:44` — Librarian
- `handover/directives/2026-05-16_REAL_BCAST_1_LIBRARIAN_BROADCAST_LOOP_ARCHITECT_ORIGINAL.md:140` — REAL-BCAST-1
- `handover/ai-direct/LATEST.md:9` — session #52 close
- `CLAUDE.md:516` — Class 4 触发条件
- `constitution.md:332` — Art. II.2 价格信号广播
- `src/bin/audit_dashboard.rs:1281` — render_text
- `src/state/typed_tx.rs:1585` — BuyWithCoinRouterTx
- `tests/fc_alignment_conformance.rs` — 695 行 per-FC witness battery

---

## 5. TuringOS v4 当前状态快照 (TISR 启动时刻)

来源: `handover/ai-direct/LATEST.md` session #52 close (2026-05-16)

```
最近 SHIPPED:
  REAL-12 Role-Specialized Economic Agents (2026-05-16) — Bull/Bear + EconomicJudgment CAS
  REAL-11 Agent Economic Action Activation (2026-05-15) — Router scripted positive control
  REAL-10 ...

In Flight:
  REAL-13 market pressure loop
  REAL-BCAST-1 librarian broadcast loop

G-Phase 收口:
  G5/G6/G7/SG-G overall §8 packet 待完成
  E2 (live non-scripted agent economic action) 仍未达成

宪法状态:
  461 constitution gates passing / 0 failing / 1 ignored
  workspace tests passing
  主干 GREEN; AMBER 主要剩 G-Phase aggregate
```

---

## 6. 后续 Foundation Read 清单 (Phase 0 第二步)

按 Charter §4 Phase 0 步骤，主 agent 将按顺序读以下 7 份文档，每份产 ≤500 字摘要到 `90_references/`:

1. `constitution.md` Art. II + Art. III + Art. V (最相关章节)
2. `handover/directives/2026-05-11_G_PHASE_GENERATIVE_ARENA_DIRECTIVE.md`
3. `handover/directives/2026-05-14_REAL5_ROLE_BASED_GENERATIVE_SCAFFOLDING_ARCHITECT_ORIGINAL.md`
4. `handover/directives/2026-05-16_REAL12_*.md` (角色分化最新版)
5. `handover/whitepapers/TURINGOS_GENERATIVE_ECONOMY_WHITEPAPER_UPDATE_REAL9.md`
6. `src/state/typed_tx.rs` (high-level 结构, 重点 line 1585-2400)
7. `src/state/sequencer.rs` (high-level 结构, 重点 line 4000-4200 admission)

(用户上传的 Generative HTML 报告已在 §1 归档，不重复读)

---

## 7. 本文档完整性自检

- ✅ 用户上传报告核心要点已归档
- ✅ 用户 verbatim 诉求 + plan-mode 答复已归档
- ✅ Auditor 输出已归档 (含 file:line 引用)
- ✅ TuringOS v4 当前状态快照 (LATEST.md session #52)
- ✅ Foundation Read 清单已规划
