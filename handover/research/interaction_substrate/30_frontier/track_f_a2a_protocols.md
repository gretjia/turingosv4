# Phase 3 / Track F — A2A 协议前沿调研 (2025-2026)

**Status**: forward-bound research; **not** a Class-3/4 implementation proposal.
**Author**: TISR Phase 3 Track F researcher agent
**Date**: 2026-05-17
**Cwd**: `tisr-2026-05-17` worktree, `handover/research/interaction_substrate/30_frontier/`

---

## 0. Disclaimer (forward-bound)

本文档是 forward-bound research 提案, 用于 TISR 双轴愿景中**轴 B (A2A 自治通信基底)** 的外部前沿映射. **不构成已批准方案, 不修改任何 src/, 不提议 Class-4 实施**. 所有引用按 markdown 链接呈现; 一切落地仍走 constitution gate + per-atom 架构师 §8 签字路径. 提到 TuringOS 内部接口处, 仅作差异对比, 不绑死设计.

---

## 1. 任务 scope + 调研方法

### 1.1 Scope

本 track 调研 **2025-2026 年 agent-to-agent (A2A) 协议前沿**, 重点是 MCP 之外的设计空间. 不重复盘点 TuringOS 已有的 REAL-5 角色分化 / REAL-12 EconomicJudgment CAS schema / REAL-BCAST-1 LibrarianBroadcast / TISR Phase 2 Track A "A2A messages 走 CAS 不走 typed_tx" 结论.

### 1.2 调研方法

**7 个方向 + 每方向 ≥2 次 WebSearch + 关键源 WebFetch**:

| 方向 | WebSearch 关键词 | WebFetch 深读 |
|---|---|---|
| Google A2A spec | "Agent2Agent protocol specification 2025", "AgentCard Task data structure 2025" | `a2a-protocol.org/latest/specification/` |
| OpenAI Swarm | "OpenAI Swarm multi-agent orchestration handoff 2025" | — (GitHub README 已通过 WebSearch summary 覆盖) |
| LangGraph | "LangGraph stateful multi-agent graph supervisor swarm 2025" | — |
| AutoGen Magentic-One | "Microsoft AutoGen Magentic-One multi-agent framework 2025" | — |
| Mechanism design / 博弈 | "mechanism design LLM agents auction game theory 2025 paper" | — (arXiv 2412.00495 / 2502.09053 摘要充足) |
| BFT / consensus | "Byzantine fault tolerance multi-agent LLM consensus 2025" | `arxiv.org/html/2511.10400v2` |
| 自组织 / stigmergy | "stigmergy swarm intelligence LLM emergent coordination 2025" | — |
| 对话即协议 vs RPC | "Karpathy chat protocol agent natural language vs JSON RPC" | `agoraprotocol.org` |
| 协议综述 | "agent protocol 2025 survey MCP A2A ANP comparison" | — (arXiv 2505.02279 / 2504.16736) |
| Gossip / 分布式 | "gossip protocol distributed agent LLM federated 2025" | `arxiv.org/pdf/2508.01531` |
| Hayek / 价格信号 | "price signal agent communication Hayek information markets 2025" | — |
| ANP / DID 身份 | "agent identity decentralized DID JSON-LD ANP 2025" | — |

**WebSearch 实际查询次数**: 13 (覆盖 7 个主线方向 + 5 个辅助方向).
**WebFetch 深读次数**: 4 (官方 spec + BFT 论文 + Agora 主页 + gossip 论文).

---

## 2. 调研发现总览

### 2.1 协议谱系 (2026 年 5 月时点)

可分四带, 按"中心化 → 去中心化"递增:

| 带 | 协议 | 主导方 | 数据模型 | 传输 | 身份层 |
|---|---|---|---|---|---|
| MCP | Model Context Protocol | Anthropic → 开源 | JSON-RPC tool call | HTTP+SSE | tool-server 静态 |
| ACP | Agent Communication Protocol | IBM/BeeAI | REST multi-part | REST+streaming | API key |
| **A2A** | Agent2Agent | Google → Linux Foundation | **AgentCard + Task + Message + Part + Artifact** | HTTP+SSE+JSON-RPC+gRPC | OpenAPI auth schemes |
| ANP | Agent Network Protocol | 开源中文社区 | JSON-LD + ADP | HTTPS | **W3C DID (`did:wba`) + Verifiable Credentials** |
| Agora | meta-protocol | 学术 (Marro et al.) | **Protocol Document + SHA1 hash + 自适应 NL/code** | HTTPS+JSON wrap | 协议级 (不绑身份) |

### 2.2 外部资源 markdown 索引

**协议 spec / 实现**:
- [A2A Protocol Specification](https://a2a-protocol.org/latest/specification/)
- [A2A GitHub (a2aproject/A2A, Apache 2.0)](https://github.com/a2aproject/A2A)
- [Google Developers Blog: Announcing A2A](https://developers.googleblog.com/en/a2a-a-new-era-of-agent-interoperability/)
- [Agora Protocol homepage](https://agoraprotocol.org/) + [paper-demo repo](https://github.com/agora-protocol/paper-demo)
- [ANP Technical White Paper (arXiv 2508.00007)](https://arxiv.org/html/2508.00007v1)
- [OpenAI Swarm (deprecated → Agents SDK)](https://github.com/openai/swarm)
- [Microsoft Magentic-One](https://microsoft.github.io/autogen/stable//user-guide/agentchat-user-guide/magentic-one.html)
- [LangGraph Swarm reference](https://reference.langchain.com/python/langgraph-swarm)
- [LangGraph Supervisor reference](https://reference.langchain.com/python/langgraph-supervisor)

**学术综述 / 论文**:
- [Survey of AI Agent Protocols (arXiv 2504.16736)](https://arxiv.org/pdf/2504.16736)
- [Survey of Agent Interoperability Protocols: MCP/ACP/A2A/ANP (arXiv 2505.02279)](https://arxiv.org/abs/2505.02279)
- [Security Threat Modeling for MCP/A2A/Agora/ANP (arXiv 2602.11327)](https://arxiv.org/html/2602.11327v1)
- [BFT for Multi-Agent Systems (arXiv 2511.10400)](https://arxiv.org/abs/2511.10400)
- [Byzantine-Robust Decentralized Coordination (arXiv 2507.14928)](https://arxiv.org/pdf/2507.14928)
- [Mechanism Design for LLMs (Google Research, WWW 2024 Best Paper)](https://research.google/blog/mechanism-design-for-large-language-models/)
- [Game Theory + LLM Systematic Survey (arXiv 2502.09053)](https://arxiv.org/html/2502.09053v2)
- [ACM EC 2026 Workshop: Game Theory + Mechanism Design with LLMs](https://llm-incentives.com/)
- [Institutional AI: Governing LLM Collusion in Cournot Markets (arXiv 2601.11369)](https://arxiv.org/html/2601.11369v1)
- [Rethinking Strategic Mechanism Design for LLMs (arXiv 2412.00495)](https://arxiv.org/html/2412.00495v1)
- [Magentic Marketplace (Microsoft Research, 2025-10)](https://www.microsoft.com/en-us/research/wp-content/uploads/2025/10/multi-agent-marketplace.pdf)
- [SwarmSys (arXiv 2510.10047)](https://arxiv.org/html/2510.10047v1)
- [SwarmBench (arXiv 2505.04364)](https://arxiv.org/html/2505.04364v4)
- [Self-Organizing LLM Agents Drop Hierarchy (arXiv 2603.28990)](https://arxiv.org/html/2603.28990v1)
- [Pressure Fields + Temporal Decay (arXiv 2601.08129)](https://arxiv.org/html/2601.08129v3)
- [Gossip Protocols for Agentic AI (arXiv 2508.01531)](https://www.arxiv.org/pdf/2508.01531)
- [Gossip-Enhanced Communication Substrate (arXiv 2512.03285)](https://arxiv.org/html/2512.03285v1)
- [Towards Transparent Blockchain-Driven Decentralized LLM Multi-Agent (arXiv 2509.16736)](https://arxiv.org/html/2509.16736)
- [Karpathy 2025 LLM Year in Review](https://karpathy.bearblog.dev/year-in-review-2025/)
- [Mises Institute: When AI Agents Trade with AI Agents, Price Discovery Dies](https://mises.org/mises-wire/when-ai-agents-trade-ai-agents-price-discovery-dies)

---

## 3. Q1: Google A2A 协议 vs TuringOS CAS 对比

### 3.1 A2A 核心数据结构 (来自 [a2a-protocol.org spec](https://a2a-protocol.org/latest/specification/))

A2A 由 Google 2025 年 4 月发布, 2025 年 6 月捐赠给 Linux Foundation; 截至 2026 年初已有 150+ 组织 (Microsoft / AWS / Salesforce / SAP / ServiceNow / Workday / IBM) 加入 ([Stellagent 2025-04](https://stellagent.ai/insights/a2a-protocol-google-agent-to-agent)).

**5 个核心对象** (字段从 spec 抓取):

```
AgentCard       — /.well-known/agent.json, JSON, capabilities + skills + auth
Task            — id, contextId, status (TaskState enum), artifacts[], history[Message], metadata
Message         — messageId, contextId?, taskId?, role, parts[], extensions[], referenceTaskIds[]
Part            — OneOf{ text | raw(base64) | url | data(JSON) }, metadata?, filename?, mediaType?
Artifact        — artifactId, name?, description?, parts[], extensions[], metadata
```

**TaskState 枚举**: `UNSPECIFIED | SUBMITTED | WORKING | COMPLETED | FAILED | CANCELED | INPUT_REQUIRED | REJECTED | AUTH_REQUIRED`.

**11 个 JSON-RPC 方法**: `SendMessage / SendStreamingMessage / GetTask / ListTasks / CancelTask / SubscribeToTask / Create|Get|List|Delete PushNotificationConfig / GetExtendedAgentCard`.

### 3.2 与 TuringOS CAS schema 对比

| 维度 | A2A (Google) | TuringOS REAL-12 / REAL-BCAST-1 |
|---|---|---|
| 主对象 | `Task` (lifecycle 9 态) | `EconomicJudgment` (CAS) + `LibrarianDigest` (CAS, REAL-BCAST-1) |
| 状态语义 | client/server task lifecycle | tape 上的 L4 accepted / L4.E 拒绝证据 |
| 身份 | OpenAPI auth schemes (静态 API-key/OAuth) | Agent registry + system pubkeys (`genesis_report` 锚) |
| 持久化 | 服务端内存 + push notification | **ChainTape + CAS** (replay-reconstructable) |
| 可审计性 | 服务端 task history (不绑链) | `HEAD_t` witness + replay verifier |
| 失败语义 | `FAILED / REJECTED` enum | L4.E `RejectionEvidence` 强类型变体 |
| 消息载荷 | `Part` (text/raw/url/data) | CAS schema typed payload, content-addressed |

### 3.3 异同总结

**相同点**: 都是 task-oriented (非 RPC-call-oriented); 都支持 streaming/push; 都使用 content-addressed 引用 (A2A 通过 `Part.url`, TuringOS 通过 CAS CID); 都尝试**透明协调不可观察的 opaque agent** (A2A 显式: "without needing access to each other's internal state, memory, or tools" — [Google Blog](https://developers.googleblog.com/en/a2a-a-new-era-of-agent-interoperability/)).

**关键差异**:

1. **持久化语义**: A2A `Task.history` 默认存活于 server, 不强制全局可审计; TuringOS CAS + ChainTape **强制 replay-from-genesis** (FC2 boot gate).
2. **失败建模**: A2A 用 enum 字符串; TuringOS 用强类型 `RejectionEvidence` 变体, 每类失败有自己的 CAS schema.
3. **经济耦合**: A2A 完全不耦合经济 (任务付费走外部支付/合约); TuringOS REAL-12 EconomicJudgment **就是** A2A 消息的核心载荷, 内嵌 EscrowLockTx / FinalizeRewardTx 路径.
4. **身份层**: A2A 走 OpenAPI auth (中心化 API key); ANP 走 W3C DID `did:wba` ([ANP white paper](https://agent-network-protocol.com/specs/white-paper.html)); TuringOS 走 system-pubkey + agent-registry (`genesis_report` 锚的混合模型).

### 3.4 TISR Phase 4-5 建议

**不要"采纳 A2A 作为基底"**. A2A 的 `Task / Message / Part` 是有用的**信封建模参考**, 但 TuringOS CAS schema 已经覆盖且更严格. **可借鉴**: A2A 的 `AgentCard` (`/.well-known/agent.json` capability advertisement) 思路, 在 TuringOS agent registry 上加一个 **`AgentCapabilityManifest` CAS object**, 公开声明该 agent 可处理的 broadcast 主题 + judgment 类型. 这与 REAL-5 角色分化是天然契合的 (10 个 AgentRole 各自有不同 capability surface).

---

## 4. Q2: 拍卖/博弈机制最新进展

### 4.1 Token 拍卖 (Google Research, WWW 2024 Best Paper)

[Google Research](https://research.google/blog/mechanism-design-for-large-language-models/) 提出 **token-level auction**: 当 N 个 LLM 各有自己的 logits 分布, 一个外部聚合器以"广告拍卖"为模型聚合 N 个 LLM 的下一个 token 输出, 兼顾真实性 (truthfulness) 与个体理性 (IR). 这是首个把"LLM 输出聚合"形式化为机制设计问题的工作.

### 4.2 LLM Cournot 市场的隐性共谋

[Institutional AI (arXiv 2601.11369)](https://arxiv.org/html/2601.11369v1) 发现: 在 Cournot 市场仿真中, **LLM agents 在无显式串通指令下学会了 supra-competitive 市场分割策略**. 即"隐性勾结" (tacit collusion) 出现. 提出"public governance graph"作监管原语. [Mises 评论](https://mises.org/mises-wire/when-ai-agents-trade-ai-agents-price-discovery-dies)则警告: 当 AI agents 互相交易, 价格信号可能携带零经济信息.

### 4.3 ALYMPICS / 行为博弈

[ALYMPICS (COLING 2025)](https://aclanthology.org/2025.coling-main.193.pdf) 提供了 LLM agent 在博弈论场景下的 benchmark; [Game-Theoretic Lens 综述 (arXiv 2502.09053)](https://arxiv.org/html/2502.09053v2) 系统化覆盖 LLM 多智能体博弈, 关键观察: **简单投票/独立聚合常优于复杂迭代反馈**.

### 4.4 ACM EC 2026 Workshop

[Game Theory & Mechanism Design with LLMs](https://llm-incentives.com/) 是 ACM Economics & Computation 顶会的正式 workshop, 议程涵盖: 价格设定 LLM agents / LLM 协商代理 / 信息聚合 / 战略交互. 这是 academic 主线进入主流的标志.

### 4.5 应用到 TISR 轴 B 的可能性

TuringOS 已有的 Polymarket-style CPMM (Stage C P-M2..P-M9 已 SHIPPED) 是**价格预测市场 substrate**, 不是聚合多 LLM 输出的机制. 可考虑两层叠加:

- **L1 (已有)**: 事件预测市场, YES/NO share, CPMM swap. Agents 用价格信号读取群体信念.
- **L2 (待考)**: 在 L1 之上, 把"agent 间通信"建模为**信息拍卖**. 例如 Librarian broadcast 的 "what to broadcast / who pays attention" 用 token auction 聚合多 librarian 的 digest 输出. 这与 [Google token auction](https://research.google/blog/mechanism-design-for-large-language-models/) 高度同构.

**风险**: 隐性共谋. 必须配套 [Institutional AI](https://arxiv.org/html/2601.11369v1) 的 governance graph 监测原语, 否则 agents 会学到 supra-competitive 策略而非"诚实暴露私有信号".

---

## 5. Q3: BFT 对 LLM agent 必要性

### 5.1 CP-WBFT 结果 ([arXiv 2511.10400](https://arxiv.org/html/2511.10400v2))

经典 BFT 假设: f < n/3 才能达成共识. 该论文的反直觉发现: **LLM agents 的 BFT 阈值远高于经典模型**, 在 7 节点完整图下能容忍 6 个恶意节点 (85.7%) 仍保持 100% round-level 准确率 (复杂图topology). 关键机制是 **confidence-probe weighting**: 不靠加密签名验证, 而靠 LLM 自身的 reflective discriminative capability — 高置信邻居答案被采纳.

类似工作 [Byzantine-Robust Decentralized Coordination (arXiv 2507.14928)](https://arxiv.org/pdf/2507.14928) 显示 LLM 在 GSM8K 数学推理任务上, 即使 85.7% 节点为恶意仍能达成共识, 这是经典 BFT 节点 2-3 倍提升.

### 5.2 TuringOS 当前"诚实假设"

读源码 (在 worktree 顶层 CLAUDE.md + 现有 trust-root):

- **Sequencer admission**: typed_tx 必须签名验证 (`agent_sig` manifest) → **不假设 agent 诚实**, 但假设其私钥未泄露.
- **Predicate gate**: L4 接受需要 predicate pass; **predicate 自身被视为可信** (因为 predicate 是系统侧逻辑, 非 agent 输出).
- **PCP soundness corpus** (G-012 决定): 对 Lean 验证器输出有对抗性测试集, 确保**不存在 invalid proof 通过 predicate**.
- **Trust Root** (STEP_B): 7 个 kernel 文件 sha256 锚定, 任何修改需架构师 §8.

**诚实假设强弱**:
- **强假设**: predicate / sequencer / Lean / system pubkey 全可信.
- **弱假设**: agent (任何 AgentRole) 可能 byzantine — 用 sequencer admission + predicate + 经济激励 + L4.E rejection 保护.

### 5.3 BFT 对 TuringOS 是否必要?

**结论**: 部分必要, 但不在共识层, 在 **判断聚合层**.

- TuringOS 的 ChainTape 不是 PoW/PoS 分布式共识 — 它是**单一 sequencer + replay verifier** (G-009 Path C hybrid, `HEAD_t` witness). 不需要经典 BFT 解决"哪条链是规范"的问题.
- 但**多 agent 对一个 EconomicJudgment / LibrarianDigest 的内容产生分歧时**, BFT-style 加权聚合 ([CP-WBFT](https://arxiv.org/html/2511.10400v2)) 是直接可用的. REAL-12 已有 EconomicJudgment, 但目前是单 agent 输出. 未来若多 agent 对同一事件做经济判断, **confidence-weighted aggregation** 是机制候选.
- 风险: confidence probe 自身可被 sycophancy 污染 ([Game Theory + LLM Survey](https://arxiv.org/html/2502.09053v2) 指出); 需配套对抗性 corpus 校准.

---

## 6. Q4: 自组织 vs REAL-5 角色分化

### 6.1 自组织 / Stigmergy / Swarm 前沿

- **[SwarmSys (arXiv 2510.10047)](https://arxiv.org/html/2510.10047v1)**: Explorer / Worker / Validator 三角色, 通过 embedding matching + pheromone-inspired 强化做动态任务分配. **关键点**: 角色不是 hard-coded, 而是 agent 自己声明的 profile, 通过 stigmergic feedback 演化.
- **[SwarmBench (arXiv 2505.04364)](https://arxiv.org/html/2505.04364v4)**: 5 个去中心化任务 (Pursuit / Synchronization / Foraging / Flocking / Transport) 在 2D 网格上, agents 只有局部感知 + 局部通信.
- **[Drop the Hierarchy and Roles (arXiv 2603.28990)](https://arxiv.org/html/2603.28990v1)**: 反直觉结论 — **自组织 LLM agents 在多任务上超越人工设计角色的 hierarchy**. 最优协调来自"最小结构脚手架 (固定顺序) + 完全角色自治 (内生 specialization)".
- **[Pressure Fields (arXiv 2601.08129)](https://arxiv.org/html/2601.08129v3)**: 共享状态的 pressure field + temporal decay 实现隐式协调, 在会议室调度任务上 > 30% 提升, **不需要 coordinator / planner / 消息传递**.

### 6.2 与 REAL-5 角色分化对比

| 维度 | REAL-5 (TuringOS 已有) | 自组织 (SwarmSys 等) |
|---|---|---|
| 角色定义时机 | 构造时 hard-coded (10 个 AgentRole enum) | 运行时 emergent |
| 协调机制 | 显式 ToolRouter 路由 + 经济激励 | stigmergic pheromone + pressure field |
| 可审计性 | 强 (每个角色有明确职能边界) | 弱 (角色边界模糊) |
| 适应性 | 弱 (新角色需 Class-4 §8) | 强 (角色 profile 自演化) |
| 共谋风险 | 低 (角色规约约束) | 高 (无约束下学到 supra-competitive 策略) |

### 6.3 哪个更优? — 取决于优化目标

- 若目标是 **可审计 + 形式化保证 + 经济守恒** (TuringOS 当前): REAL-5 更优. Trust Root + STEP_B 机制需要静态可枚举的 attack surface, 自组织 emergent 角色无法被 sequencer admission 静态白名单约束.
- 若目标是 **解题广度 + zero-shot 适应**: 自组织更优. [Drop the Hierarchy](https://arxiv.org/html/2603.28990v1) 给的实验证据明确.

**混合候选** (forward-bound, 不构成提案):
- REAL-5 角色作为 **kernel hard scaffold** (固定顺序 / 静态 capability surface, 类似 [Drop the Hierarchy] "minimal structural scaffolding").
- 在 ToolRouter 边缘允许 agent 通过 **CapabilityManifest CAS object** 自我扩展 sub-role profile, 走 LibrarianBroadcast 而非 typed_tx 注入 (这与 Phase 2 Track A 结论一致).
- 自组织协调的"信号"用 **价格** (Hayek 原语) 而非 pheromone — 这是 TISR 轴 B 的天然原语.

---

## 7. Q5: 对话即协议 vs typed_tx

### 7.1 "对话即协议"在 2025-2026

**Karpathy 主线** ([Karpathy 2025 Year in Review](https://karpathy.bearblog.dev/year-in-review-2025/)): 没有直接讲 "agent-to-agent chat as protocol", 但他的 **"English is the new programming language"** + **Claude Code 是首个有说服力的 agent 演示**的论断, 隐含: agent 的 first-class interface 应是自然语言, 而不是结构化 RPC schema.

**Agora protocol** ([agoraprotocol.org](https://agoraprotocol.org/), [arXiv 2410.11905](https://arxiv.org/html/2410.11905v1)) 是这条思路的工程化实例:
- 引入 **Protocol Document (PD)**: plain-text 协议描述. 每个 PD 有一个 SHA1 hash.
- 三层混合: 高频通信走 standardized routines (token cost ↓); 低频走 NL (versatility ↑); 中间地带走 **LLM-written routines** (agent 自己写代码实现新协议).
- 声称 token 节省 98% 相比纯 NL.

**ANP 的对应做法** ([ANP comparative analysis](https://agent-network-protocol.com/blogs/posts/agent-communication-protocols-comparison.html)): meta-protocol 层允许 agents 用 NL 协商, 然后生成 application-layer 协议代码. 同样是 "NL 协商 + 结构化执行" 双模.

### 7.2 与 TuringOS typed_tx schema-based 协议对比

| 维度 | typed_tx (TuringOS) | Agora PD / ANP NL-negotiation |
|---|---|---|
| Schema 定义 | 构造时静态 (Class-4 STEP_B) | 运行时动态生成 |
| 演化路径 | 架构师 §8 + Trust Root rehash | agent 自主 (PD hash 即版本) |
| 字段语义 | 强类型, 编译期保证 | 自然语言 + LLM 解释 |
| 错误处理 | RejectionEvidence 强类型变体 | LLM 解析失败 → NL 错误消息 |
| 审计性 | 完整 (replay-deterministic) | 弱 (LLM 重解析非确定) |
| Token 成本 | 低 (序列化好的二进制) | 中 (PD 缓存后) → 高 (PD 首次协商) |

### 7.3 融合候选 (forward-bound)

**关键观察**: TuringOS 已经有 typed_tx 的 hard 边界 (sequencer admission), 这是必须的 — 经济守恒不能由 LLM 重解析. **但 A2A 层的非经济通信不需要走 typed_tx**. TISR Phase 2 Track A 已经确定 "A2A messages 走 CAS schema 不走 typed_tx", 这与 Agora PD 思路天然契合:

- **kernel-money path** (kernel layer): typed_tx, 严格. 不变.
- **agent-to-agent broadcast** (CAS schema layer): 可以是结构化 schema (REAL-12 / REAL-BCAST-1), **也可以**是 Agora-style PD 引用 — agent 在 CAS 中存一个 `ProtocolDocument` object (SHA256 → CAS CID), broadcast 时引用该 CID, 接收 agent 按 PD 解释 payload.
- **真正的 chat-as-protocol layer**: 留给 NL prompt body, **不强制 schema**, 但需要 PromptCapsule 锚定 (G-021 Class-3 默认).

这给出三层栈: typed_tx (硬) / CAS schema (软, 可选 PD) / NL (自由, PromptCapsule 锚定).

### 7.4 警告 (来自 [Security Threat Modeling for Agent Protocols](https://arxiv.org/html/2602.11327v1))

NL-negotiation 路径有重大威胁面: **协议注入 / PD 投毒 / 协商劫持**. 任何 NL-negotiation 实施必须有:
1. PD CID 严格签名验证 (类似 Agora SHA1 hash, 但用 SHA256 + 签名);
2. 系统侧 PD 白名单 (architect-approved PD CID 集合) + agent-proposed PD 走 challenge 期;
3. PromptCapsule + AttemptTelemetry 全程锚定 NL 协商过程 (FC1 hard invariant).

---

## 8. 现有报告盲点清单 (Top 5)

基于用户上传的 Generative HTML 报告中 MCP / A2UI / AG-UI / Magentic-UI 的覆盖范围, 以下 5 个方向在 TuringOS 现有架构 + 用户报告里**未深入或未覆盖**:

### 盲点 1: ANP DID 身份层 (W3C 标准)

- **何为**: [ANP `did:wba` ](https://agent-network-protocol.com/specs/white-paper.html) 提供 W3C DID + Verifiable Credentials + JSON-LD agent identity, 非区块链, 走 web 基础设施 (HTTPS+DNS).
- **TuringOS 现状**: agent registry + system pubkey, 中心化 anchor, 无 cross-organization 身份互认.
- **盲点价值**: 当 TISR 进入跨组织 A2A 时代 (轴 B 多 LLM-vendor), 没有 DID 身份层无法解决"对方 agent 是谁 / 是否可信"问题. **A2A 协议本身的身份层 (OpenAPI auth) 是 enterprise-grade 但中心化的**, ANP 是去中心化补丁.

### 盲点 2: BFT 置信加权聚合 (CP-WBFT)

- **何为**: [CP-WBFT](https://arxiv.org/html/2511.10400v2) 用 confidence-probe + topology-aware weighting 在 85.7% 恶意节点下保持共识.
- **TuringOS 现状**: 单 agent 输出 EconomicJudgment, 无多 agent 聚合.
- **盲点价值**: REAL-12 EconomicJudgment 未来多 agent 共同判断时, **confidence-weighted aggregation > 简单多数投票**, 抗 sycophancy. 这是 REAL-12 v2 候选机制.

### 盲点 3: Gossip 协议作为 LibrarianBroadcast 演化

- **何为**: [Gossip Protocols for Agentic AI (arXiv 2508.01531)](https://www.arxiv.org/pdf/2508.01531) 提议混合 LLM-gossip agent, 学**"share what / when / with whom"**.
- **TuringOS 现状**: REAL-BCAST-1 LibrarianBroadcast 走 hub-and-spoke (Librarian → 所有订阅 agent).
- **盲点价值**: hub-and-spoke 单点故障 + 不 scale. Gossip 允许 broadcast 邻居 → 邻居扩散, 也兼容 TuringOS tape (gossip event 仍可锚到 ChainTape 作为可审计原语). LibrarianBroadcast v2 候选拓扑.

### 盲点 4: 隐性共谋治理 (Institutional AI)

- **何为**: [Institutional AI (arXiv 2601.11369)](https://arxiv.org/html/2601.11369v1) 警告 LLM agents 在市场中**自发学到 supra-competitive 策略**, 提出 public governance graph 监管.
- **TuringOS 现状**: CPMM market + EconomicJudgment 有经济激励, 但**无显式反共谋监测**.
- **盲点价值**: 当 TISR Phase 4+ 多 LLM-vendor 共存 + 都用 EconomicJudgment 给市场喂判断, **共谋会出现**. 必须有 governance graph (CAS schema, 定期统计 agent 间策略相关性 + price discovery 质量) 作为预警.

### 盲点 5: Agora-style Protocol Document 演化

- **何为**: [Agora PD](https://agoraprotocol.org/) 让 agents 自己协商 + 生成协议代码; SHA1-hash 引用 + LLM-written routines.
- **TuringOS 现状**: typed_tx 是 Class-4 锁死的; CAS schema 演化也需架构师介入.
- **盲点价值**: REAL-5 角色分化是静态的, 新角色需 Class-4 §8. Agora-style PD 允许**在 CAS 层动态扩展子 capability** 而不动 kernel — 这是 TuringOS architectural rigidity 与 LLM agent 适应性之间的现实折中候选.

---

## 9. 给 TISR Phase 4-5 deliverable 的建议

下列 5 条均 **forward-bound**, 不构成已批准方案, 每条都需后续 charter + 架构师 §8:

### R1. AgentCapabilityManifest CAS schema (轻量)

引入 `AgentCapabilityManifest` CAS object 公开声明每个 agent 的可处理 broadcast 主题 + judgment 类型, 类似 A2A `AgentCard` 但走 CAS + content-addressed. **Class 2 候选** (production wire-up, 不动 sequencer admission, 不动 typed_tx).

### R2. EconomicJudgment v2 候选: confidence-weighted aggregation

当多 agent 对同一事件产 EconomicJudgment 时, 引入 [CP-WBFT-style](https://arxiv.org/html/2511.10400v2) confidence weighting. 需要先建 confidence calibration corpus (类似 PCP soundness corpus). **Class 3 候选** (经济判断聚合, 不动钱路径但影响价格信号生成).

### R3. LibrarianBroadcast v2 候选: gossip 扩散拓扑

当前 hub-and-spoke 演化为 gossip-style 邻居扩散, 每个 broadcast event 仍锚 ChainTape 但分发走 gossip. 需先有 SwarmBench-style benchmark 评估 LLM-gossip 在 TuringOS 上的可行性. **Class 2 候选**.

### R4. 共谋监测 governance graph (CAS schema)

引入 `GovernanceGraph` CAS schema, 定期 (每 N tape epoch) 统计 agent 间策略相关性 + price discovery 质量, 触发架构师告警. 不直接干预市场, 只观察 + 报告. **Class 2 候选 + 监测面板**.

### R5. 跨组织身份层调研 (DID/ANP)

启动 TISR Phase 4 Track G: 调研 `did:wba` 与 TuringOS agent registry 的桥接路径. **Class 0 (docs/research)** 先行.

---

## 10. References

### 协议规范 / 实现仓库

- [A2A Protocol Specification (latest)](https://a2a-protocol.org/latest/specification/)
- [A2A Protocol Specification v0.2.5](https://a2a-protocol.org/v0.2.5/specification/)
- [A2A GitHub (a2aproject/A2A)](https://github.com/a2aproject/A2A)
- [Google Developers Blog — Announcing A2A](https://developers.googleblog.com/en/a2a-a-new-era-of-agent-interoperability/)
- [Google Codelabs — Getting Started with A2A](https://codelabs.developers.google.com/intro-a2a-purchasing-concierge)
- [Agora Protocol homepage](https://agoraprotocol.org/)
- [Agora paper-demo repo](https://github.com/agora-protocol/paper-demo)
- [Agora ZeroXClem implementation](https://github.com/ZeroXClem/agora-protocol)
- [ANP Technical White Paper](https://agent-network-protocol.com/specs/white-paper.html)
- [ANP Communication Specification](https://agent-network-protocol.com/specs/communication.html)
- [ANP Comparative Analysis (vs MCP/A2A/Agora/agents.json/LMOS/AITP)](https://agent-network-protocol.com/blogs/posts/agent-communication-protocols-comparison.html)
- [OpenAI Swarm GitHub](https://github.com/openai/swarm)
- [OpenAI Cookbook — Orchestrating Agents](https://developers.openai.com/cookbook/examples/orchestrating_agents)
- [Microsoft Magentic-One](https://microsoft.github.io/autogen/stable//user-guide/agentchat-user-guide/magentic-one.html)
- [Microsoft Research — Magentic-One article](https://www.microsoft.com/en-us/research/articles/magentic-one-a-generalist-multi-agent-system-for-solving-complex-tasks/)
- [Microsoft Agent Framework Overview](https://learn.microsoft.com/en-us/agent-framework/overview/)
- [Microsoft AutoGen GitHub](https://github.com/microsoft/autogen)
- [LangGraph Swarm reference](https://reference.langchain.com/python/langgraph-swarm)
- [LangGraph Supervisor reference](https://reference.langchain.com/python/langgraph-supervisor)

### 学术 / 调研

- [Survey of AI Agent Protocols (arXiv 2504.16736)](https://arxiv.org/pdf/2504.16736)
- [Survey of Agent Interoperability Protocols MCP/ACP/A2A/ANP (arXiv 2505.02279)](https://arxiv.org/abs/2505.02279)
- [Security Threat Modeling for MCP/A2A/Agora/ANP (arXiv 2602.11327)](https://arxiv.org/html/2602.11327v1)
- [Agora — Scalable Communication Protocol (arXiv 2410.11905)](https://arxiv.org/html/2410.11905v1)
- [ANP Technical White Paper (arXiv 2508.00007)](https://arxiv.org/html/2508.00007v1)
- [AI Agents with DIDs and Verifiable Credentials (arXiv 2511.02841)](https://arxiv.org/html/2511.02841v2)
- [Rethinking the Reliability of MAS via BFT (arXiv 2511.10400)](https://arxiv.org/abs/2511.10400)
- [Byzantine-Robust Decentralized Coordination of LLM Agents (arXiv 2507.14928)](https://arxiv.org/pdf/2507.14928)
- [Weighted Byzantine Fault Tolerance Consensus (Dustdar et al., 2025)](https://dsg.tuwien.ac.at/team/sd/papers/Preprint_2025_S_Dustdar_A_Weighted.pdf)
- [BFT Approach towards AI Safety (arXiv 2504.14668)](https://arxiv.org/pdf/2504.14668)
- [Mechanism Design for LLMs (Google Research)](https://research.google/blog/mechanism-design-for-large-language-models/)
- [Rethinking Strategic Mechanism Design for LLMs (arXiv 2412.00495)](https://arxiv.org/html/2412.00495v1)
- [Game Theory Meets LLMs — Systematic Survey (arXiv 2502.09053)](https://arxiv.org/html/2502.09053v2)
- [Game-Theoretic Lens on LLM MAS (arXiv 2601.15047)](https://arxiv.org/html/2601.15047v1)
- [Understanding LLM Agent Behaviours via Game Theory (arXiv 2512.07462)](https://arxiv.org/abs/2512.07462)
- [Institutional AI — Governing LLM Collusion in Cournot Markets (arXiv 2601.11369)](https://arxiv.org/html/2601.11369v1)
- [ALYMPICS — LLM Agents Meet Game Theory (COLING 2025)](https://aclanthology.org/2025.coling-main.193.pdf)
- [ACM EC 2026 Workshop — Game Theory & Mechanism Design with LLMs](https://llm-incentives.com/)
- [Magentic Marketplace (Microsoft Research 2025-10)](https://www.microsoft.com/en-us/research/wp-content/uploads/2025/10/multi-agent-marketplace.pdf)
- [QuantAgent — Price-Driven Multi-Agent LLMs (arXiv 2509.09995)](https://arxiv.org/html/2509.09995v3)
- [AI Agents in Financial Markets (arXiv 2603.13942)](https://arxiv.org/html/2603.13942v1)
- [Revisiting Gossip Protocols (arXiv 2508.01531)](https://www.arxiv.org/pdf/2508.01531)
- [Gossip-Enhanced Communication Substrate (arXiv 2512.03285)](https://arxiv.org/html/2512.03285v1)
- [SwarmSys (arXiv 2510.10047)](https://arxiv.org/html/2510.10047v1)
- [SwarmBench — Benchmarking LLMs Swarm Intelligence (arXiv 2505.04364)](https://arxiv.org/html/2505.04364v4)
- [Drop the Hierarchy and Roles (arXiv 2603.28990)](https://arxiv.org/html/2603.28990v1)
- [Emergent Coordination via Pressure Fields (arXiv 2601.08129)](https://arxiv.org/html/2601.08129v3)
- [Towards Transparent Blockchain-Driven LLM Multi-Agent (arXiv 2509.16736)](https://arxiv.org/html/2509.16736)
- [Autonomous Agents on Blockchains (arXiv 2601.04583)](https://arxiv.org/pdf/2601.04583)

### 评论 / 行业

- [Mises Institute — When AI Agents Trade with AI Agents, Price Discovery Dies](https://mises.org/mises-wire/when-ai-agents-trade-ai-agents-price-discovery-dies)
- [Hayek on Decentralized Information in Markets](https://conversableeconomist.com/2025/01/28/hayek-on-decentralized-information-in-markets/)
- [Karpathy 2025 LLM Year in Review](https://karpathy.bearblog.dev/year-in-review-2025/)
- [Best of 2025 — Google Cloud Unveils A2A](https://platformengineering.com/editorial-calendar/best-of-2025/google-cloud-unveils-agent2agent-protocol-a-new-standard-for-ai-agent-interoperability-2/)
- [A2A Protocol — 150+ Organizations in One Year (Stellagent)](https://stellagent.ai/insights/a2a-protocol-google-agent-to-agent)
- [JSON-RPC and AI — Why the Simplest Protocol Powers the Smartest Agents](https://jemshit.com/blog/json-rpc-simplest-protocol-smartest-agents)
- [Microsoft Agent Framework — Production-Ready Convergence](https://cloudsummit.eu/blog/microsoft-agent-framework-production-ready-convergence-autogen-semantic-kernel)
- [IBM — What Are AI Agent Protocols?](https://www.ibm.com/think/topics/ai-agent-protocols)

---

**EOF** — Phase 3 / Track F (forward-bound research)
