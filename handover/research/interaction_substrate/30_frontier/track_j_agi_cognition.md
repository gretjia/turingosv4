# Phase 3 / Track J — AGI 认知架构与涌现智能

## 0. forward-bound 警告 + scope (聚焦 "TuringOS 缺什么")

**全 forward-bound**。本文档不修改 ChainTape/CAS, 不写入证据, 不触动宪法落地条目, 不产生 §8 决策。所有结论是**"Phase 4-5 设计提示"**, 等同 OBS 注记, 须经架构师/用户 §8 才能进入 TB。

**Scope 限制 (避免 AGI 综述)**:
- **不做** "通用 AGI 现状回顾" / "LLM 能力增长曲线" / "AGI safety 综述"。
- **只问** 一件事: TuringOS = 外部 tape + 谓词 + 经济驱动 + 黑盒 LLM agent, 这种"外部基底"假设缺了哪些"agent 内部认知结构"? 是否需要补?
- **结论形式**: 每个问题给出 "需要 / 不需要 / 视 TISR 轴 B 子目标而定" 三档判定 + 1-2 句理由。

时间窗口: 2025-01 至 2026-05。所有引用 markdown 链接。

***

## 1. 任务 scope + WebSearch 清单

**已调研的 7 条主线**:

1. JEPA / V-JEPA / V-JEPA2 / LeWorldModel (LeCun 路线 world model)
2. NVIDIA Cosmos / Dreamer V3 (其他 world model 范式)
3. Active Inference / 自由能原理 (Friston 2025 多 agent 工作)
4. Reflexion / Meta-Policy Reflexion / Multi-Agent Reflexion (元认知)
5. Stigmergy / 涌现集体记忆 / LLM swarm (集体智能)
6. HTN / MCTS for LLM / Tree-of-Options (长时序规划)
7. Multi-agent scaling laws / collaborative emergence (涌现阈值)

每条主线挑出 2025-2026 最新代表作, 不做全文综述, **只摘"对 TuringOS 是否补"有判定价值的事实**。

***

## 2. 总览 (含外部链接)

**核心 frontier 共识 (2025-2026)**:

- **World model 不再等于 token-level LLM**。LeCun 力推 [V-JEPA2 (Meta 2025-06)](https://ai.meta.com/blog/v-jepa-yann-lecun-ai-model-video-joint-embedding-predictive-architecture/) 做隐式 latent prediction; [Dreamer V3 (Nature 2025)](https://danijar.com/project/dreamerv3/) 把 RSSM latent state 当成 actor-critic 想象 rollout 基底; [NVIDIA Cosmos (2025-03)](https://www.nvidia.com/en-us/ai/cosmos/) 走 video foundation model + Physical AI 路线。三派对"world model 应该是 latent prediction, 不是 token autoregression"高度一致。
- **Active Inference 成为多 agent 通信底层候选**。[VERSES + Friston 2025-07 mobile manipulation 论文](https://deniseholt.us/verses-ai-leads-active-inference-breakthrough-in-robotics/) 演示了多 active-inference agent 在单 robot body 内通过共享 belief state + Free Energy 互通; [Nature Communications 2025-12 DR-FREE](https://www.nature.com/articles/s41467-025-67348-6) 引入 distributional robustness。这条线和 Hayek 价格信号本质不同 (后述)。
- **元认知 (Reflexion 家族) 成为标配能力, 非可选**。[Multi-Agent Reflexion MAR (2025-12)](https://arxiv.org/html/2512.20845) 把单 agent reflection 升级为多 agent 互评; [Meta-Policy Reflexion (2025-09)](https://arxiv.org/abs/2509.03990) 把反思变成可复用 rule。生产部署 (Meta REA / Cognition Devin) 把反思层视为基本组件。
- **大规模 agent 涌现阈值出乎意料地低**。[Towards a Science of Scaling Agent Systems (2025-12)](https://arxiv.org/abs/2512.08296) 报告 N≈16-32 即触发集体涌现拐点 γ (vs 神经网络需 10¹⁸-10²⁴ 参数)。**结论**: 涌现不必靠"更大模型", 而靠 "更好通信拓扑 + 更小 N"。
- **Stigmergy + 外部 trace 在 N ≥ 500 时超过 individual memory**。[Emergent Collective Memory (2025-12)](https://arxiv.org/abs/2512.10166) 在 30x30+ 网格、密度 ρ ≥ 0.20 下, 环境 trace + 个体 memory 联合方案比纯 memory 强 36-41%; **环境 trace 单独无效**。
- **长时序规划层 (planning) 与 LLM token 推理仍是两层**。[Tree of Options (ICLR 2025)](https://openreview.net/forum?id=uZFpfHSgN0) 指出"LLM 作 low-level world model 时, 长时序误差累计崩盘", 必须升到 temporally-extended option 层。ToT/GoT/MCTS-for-LLM 是补丁性方案。

***

## 3. Q1: World model 必要性

**问题**: TuringOS agent (黑盒 LLM) 是否需要**显式** world model? 还是 ChainTape 已是隐式 world model?

**判定**: **视 TISR 轴 B 子目标而定。轴 B 的"大规模 agent 集群涌现"不强需要显式 latent world model; ChainTape + CAS 已经是"外部可重放的世界状态"。但**单个 agent 的长时序决策质量**会因为缺显式 latent 预测而瓶颈在 token-level 自回归错误累积上**。

**依据**:
- V-JEPA2 / Dreamer V3 / Cosmos 三家共同点: **world model 是 latent prediction + rollout**, 目的是让 actor 在"想象空间"里搜索, 而不是真实世界试错。
- TuringOS ChainTape 是**离散事件 ledger**, 不是"可微 latent rollout 空间"。它能事后审计、回放, 但 agent 在 t 时刻**无法用 ChainTape "想象"** 接下来 100 步会发生什么 (除非把它喂给另一个 LLM 当 context)。
- 替代方案: 不需要给每个 agent 装 V-JEPA, 但**可以让 LibrarianAgent 维护一个"近期 ChainTape 的语义摘要 capsule" (类似 LeWorldModel 的 latent state cache)**, 作 agent rollout 的隐式基底。这与 TB-N3 / G-Phase 的 capsule 方向兼容。

**TuringOS 缺什么**: 缺**"latent state cache" capsule** — 当前 PromptCapsule 只记录单次 prompt context, 不记录"未来若干步的预测分布"。Phase 4-5 可考虑 ForecastCapsule。

***

## 4. Q2: Active Inference vs Hayek 价格信号

**问题**: Active Inference / 自由能原理是否适合作 TISR 轴 B 的 agent 通信驱动? (vs Hayek 价格信号)

**判定**: **不替代, 互补**。Hayek 价格信号 (TuringOS 当前已有的 Polymarket CPMM 路线) 是**事后竞价 + 风险定价机制**; Active Inference 是**事前预测 + 信念更新机制**。两者作用层不同。

**依据**:
- VERSES 2025-07 mobile manipulation 论文里, **多 Active Inference agent 通过 Free Energy 互通** = 每个 agent 把"我对世界的 belief"广播给其他 agent, 其他 agent 用 prediction error 更新自己 belief。这是**信念-空间通信协议**, 不需要"价格"。
- 但 Active Inference 假定**所有 agent 共享 generative model**, 这对 TuringOS 黑盒 LLM 不成立 (Claude / GPT / Gemini 内部 generative model 不可共享、不可知)。
- Hayek 价格信号 (CPMM YES/NO + LMSR-like 流动性) 是**model-agnostic** 的: 不管 agent 内部怎么想, 只要它愿意以 X 价格买 YES, 价格就反映"它认为为真的概率"。这正是 TuringOS 设计的精髓。
- **结论**: 不把 Active Inference 作主通信协议 (会破坏 model-agnostic 原则); 但**可以作"角色 agent 内部决策算法"** — 例如 [REAL-12 角色经济 agent](../../../tracer_bullets/) 内部用 Free Energy 选 action, 对外仍发 BuyYes/BuyNo Tx。

**TuringOS 缺什么**: 不缺通信协议 (CPMM 价格信号已够); 缺**"agent 内部决策算法的标准化建议"** — 当前每个角色 agent 决策逻辑各异, 可考虑用 Active Inference 作"参考实现"。

***

## 5. Q3: Meta-cognition 标准能力

**问题**: Meta-cognition (Reflexion / SelfRefine / 自我评议) 是否应作为 TuringOS agent 标准能力? 与 REAL-5 角色分化兼容?

**判定**: **应作为标准能力, 且与 REAL-5 角色分化完全兼容, 甚至天然契合**。

**依据**:
- MAR (2025-12) 实测: **多 agent 互评 > 单 agent 自评 > 无 reflection**, 显著优于 GPT-3.5 baseline。
- REAL-5 已分化 Librarian / Maker / Critic 等角色 — **CriticAgent 本质就是 "Reflexion 中的 evaluator"**。Meta-Policy Reflexion (2025-09) 提出"把反思变成可复用 rule"完全可以映射为 TuringOS 的 EvidenceCapsule + PolicyCapsule。
- 警惕: Reflexion 论文里"反思 → memory → 下一轮 prompt"的反馈环, 在 TuringOS 必须经 ChainTape 锚定 (PromptCapsule + AttemptTelemetry 已支持), **不能走 in-memory 内部 buffer** (违反 FC1 hard invariant)。

**TuringOS 缺什么**: 缺**"标准化的 reflection-as-Tx" 模式** — 当前 CriticAgent 反思是否走 L4 (作 ReviewTx)、是否走 L4.E (作 RejectTx)、是否走 CAS (作 ReviewCapsule), 三种路径并存, 缺少 canonical mapping。Phase 4-5 可考虑 ReflectionTx + ReflectionCapsule schema 标准化。

***

## 6. Q4: 集体智能涌现条件

**问题**: 大规模 agent 集群涌现条件 (N agent → swarm behavior)?

**判定**: **三条硬条件**: (a) N ≥ 16-32 (涌现拐点); (b) 通信拓扑匹配任务结构 (不是 all-to-all); (c) 高密度 (ρ ≥ 0.20) 下必须有**外部 trace** (stigmergy), 否则纯 individual memory 退化。

**依据**:
- Scaling Agent Systems (2025-12): 涌现拐点 γ 出现在 N≈16-32, 不需要百万 agent。**关键变量是 coordination topology, 不是 agent 数**。
- Emergent Collective Memory (2025-12): 在 30x30+ 网格、500+ agent 时, **环境 trace + memory** 联合方案比纯 memory 强 36-41%; **纯环境 trace 单独无效** (要配合 individual memory)。
- 对 TuringOS 的映射: ChainTape **就是** stigmergy 环境 trace — agent 写 Tx 到 tape, 其他 agent 通过读 tape 间接通信。TuringOS 已经具备 stigmergy 基底, 但**当前 agent 数 N 太小** (REAL-12 角色经济只有 4-5 个角色 agent)。
- 警惕: stigmergy 在小 N (< 16) 下**反而拖累** — 论文实测 sparse density 下 individual memory 更强。这意味着 TuringOS 在 REAL-12 阶段不应过度依赖 ChainTape 跨 agent 通信, 应让每个角色 agent 维持自己的内部 memory (PromptCapsule)。

**TuringOS 缺什么**: 缺**"agent 数 → 涌现门槛"的设计目标** — 当前没有 "至少跑到 N=16 才能验证 swarm 假设" 的实验设计。Phase 4-5 应明确把 REAL-N (≥ 16 agent) 作为涌现验证里程碑。

***

## 7. Q5: 长时序规划必要性

**问题**: TuringOS 当前 agent 一步式行动 (proposal → predicate → WorkTx), 是否需要规划层 (HTN / MCTS / Tree-of-Options)?

**判定**: **需要, 但不应在 kernel 层加; 应在 agent 层加 (Class-1 additive)**。

**依据**:
- Tree of Options (ICLR 2025) 明确: **LLM 直接做 low-level world model 在长时序任务上误差累积崩盘**, 必须升到 "temporally-extended option" 层。
- TuringOS 当前 agent 一步式 = 每次 proposal 只能想一步, 长证明 (MiniF2F P49 等) 实测就是这样掉链子的 (TB-18 M1 tape VETO 根因之一)。
- HTN + MCTS-for-LLM (LLM-MCTS / MC-DML / Tree of Options) 三派都是 **agent-side scaffold**, 不动 kernel。映射到 TuringOS: 在 LibrarianAgent / MakerAgent 内部加 "tactic tree search + UCB rollout", 对外仍发单个 WorkTx, 不破坏 FC1。
- 关键警告: 规划层产生的"中间 thought tree" **不能作 private CoT 隐藏掉** — 一旦它影响未来 prompt context 或 final proof, 它就是 externalized attempt, 必须走 PromptCapsule + AttemptTelemetry + L4/L4.E (Class-3 PromptCapsule + L4 anchor 路线, CLAUDE.md §4.3)。

**TuringOS 缺什么**: 缺**"规划层 thought tree 的 capsule 标准"** — 当前 PromptCapsule 只记录单次 prompt, 不记录"我探索了 10 个 tactic 分支, 选了第 3 个"。Phase 4-5 可考虑 PlanCapsule (记录 thought tree 结构 + UCB 评分) + 单一 WorkTx 引用该 capsule_cid。

***

## 8. TuringOS 缺什么清单 (Top 5)

按"对 TISR 轴 B 价值 × Phase 4-5 落地成本"排序:

| # | 缺什么 | 价值 | 成本 | 建议优先级 |
|---|---|---|---|---|
| 1 | **PlanCapsule + thought tree CAS schema** (Q5) — 让 agent 内部规划成为可审计的 CAS 对象 | 高: 直接解决长证明误差累积 | 中: Class-1 additive + CAS schema 扩展 | **Phase 4 优先** |
| 2 | **ReflectionTx + ReflectionCapsule** (Q3) — 标准化 critic 反思的 Tx 形态 | 高: 与 REAL-5 角色分化完美契合 | 中: 需新 TypedTx variant (Class-3/4) | **Phase 4-5** |
| 3 | **N ≥ 16 涌现验证里程碑 REAL-N** (Q4) — 没这个目标, swarm 假设无法证伪 | 高: TISR 轴 B 核心命题 | 高: 需 ≥ 16 LLM concurrent, 成本估算待 G-Phase 评估 | **Phase 5 (G-Phase 后)** |
| 4 | **ForecastCapsule "latent state cache"** (Q1) — Librarian 维护未来若干步的预测分布 | 中: 改善单 agent 决策, 不是轴 B 核心 | 中: Class-1 additive | **Phase 5** |
| 5 | **Active Inference 作角色 agent 参考实现** (Q2) — REAL-12 经济角色 agent 内部决策算法标准化 | 中: 增加角色 agent 一致性, 不影响通信协议 | 低: 纯 agent-side, 不动 kernel | **Phase 5 可选** |

**显式 NOT 在清单内**:
- **显式 latent world model (V-JEPA 类)** — TuringOS 不需要给每个 agent 装。ChainTape 替代足够。
- **Active Inference 作主通信协议** — 破坏 model-agnostic 原则, 替换 Hayek 价格信号弊大于利。

***

## 9. 给 Phase 4-5 建议

**Phase 4 (近期)**:
1. 把 PlanCapsule + ReflectionTx 作为下一个 substrate TB 候选 (排在 G-Phase G0..G7 持久化之后)。
2. 修改 PromptCapsule schema 预留 `plan_capsule_cid` + `reflection_capsule_cid` 引用 slot (forward-compatible 扩展, Class-2)。
3. 在 REAL-12 角色经济 agent 代码里**显式标注**"这一步可以加 MCTS rollout, 但当前不加" — 留下 hook 点。

**Phase 5 (中期, G-Phase 之后)**:
1. 启动 REAL-N (N ≥ 16) 涌现验证 TB, 验收标准: **若 N=16 时通信拓扑不匹配, 性能不应超过 N=4**; **若拓扑匹配, 性能应在 N=16 出现非线性拐点** (Scaling Agent Systems 2025-12 命题的本地复现)。
2. 给 Librarian 加 ForecastCapsule, 但**先做"近期 ChainTape 摘要"版本**, 不做完整 latent rollout (避免 V-JEPA 级训练成本)。
3. Active Inference 作 REAL-12 经济角色 agent 内部决策的"参考实现 PR" (可选; 用户验收前不上 main)。

**Phase 4-5 都不做**:
- 不做 V-JEPA / Cosmos 级 world model 训练 (成本不匹配 TISR 目标)。
- 不替换 CPMM 为 Active Inference 通信 (破坏 model-agnostic)。
- 不把 reflection / planning 当成 private CoT 隐藏 (违反 FC1)。

***

## 10. References

- [I-JEPA: The first AI model based on Yann LeCun's vision for more human-like AI (Meta AI)](https://ai.meta.com/blog/yann-lecun-ai-model-i-jepa/)
- [V-JEPA: The next step toward advanced machine intelligence (Meta AI)](https://ai.meta.com/blog/v-jepa-yann-lecun-ai-model-video-joint-embedding-predictive-architecture/)
- [LeWorldModel: Stable End-to-End JEPA from Pixels (arxiv 2603.19312, 2026)](https://arxiv.org/abs/2603.19312)
- [World Models: JEPA and VL-JEPA (Themesis 2026-01)](https://themesis.com/2026/01/21/world-models-jepa-and-vl-jepa/)
- [DreamerV3: Mastering Diverse Domains through World Models (Nature 2025)](https://danijar.com/project/dreamerv3/)
- [Multimodal Dreaming: Global Workspace + DreamerV3 (arxiv 2502.21142)](https://arxiv.org/html/2502.21142v2)
- [NVIDIA Cosmos World Foundation Models](https://www.nvidia.com/en-us/ai/cosmos/)
- [NVIDIA Cosmos major release (2025-03)](https://nvidianews.nvidia.com/news/nvidia-announces-major-release-of-cosmos-world-foundation-models-and-physical-ai-data-tools)
- [World Simulation with Video Foundation Models for Physical AI (arxiv 2511.00062)](https://arxiv.org/abs/2511.00062)
- [VERSES + Friston Active Inference robotics breakthrough (2025-07)](https://deniseholt.us/verses-ai-leads-active-inference-breakthrough-in-robotics/)
- [As One and Many: Group-Level Generative Models in Active Inference (MDPI 2025)](https://www.mdpi.com/1099-4300/27/2/143)
- [Distributionally Robust Free Energy Principle DR-FREE (Nature Comm 2025-12)](https://www.nature.com/articles/s41467-025-67348-6)
- [From AI to Active Inference: 6G World Brain (Optica 2026)](https://opg.optica.org/jocn/abstract.cfm?uri=jocn-18-1-A28)
- [FEP Active Inference Papers Repository](https://github.com/BerenMillidge/FEP_Active_Inference_Papers)
- [Multi-Agent Reflexion MAR (arxiv 2512.20845, 2025-12)](https://arxiv.org/html/2512.20845)
- [Meta-Policy Reflexion: Reusable Reflective Memory (arxiv 2509.03990, 2025-09)](https://arxiv.org/abs/2509.03990)
- [Self-Improving AI Agents: The 2026 Guide (o-mega)](https://o-mega.ai/articles/self-improving-ai-agents-the-2026-guide)
- [Meta-cognitive Reflection for Efficient Self-Improvement (arxiv 2601.11974)](https://arxiv.org/html/2601.11974v1)
- [Emergent Collective Memory in Decentralized Multi-Agent AI (arxiv 2512.10166, 2025-12)](https://arxiv.org/abs/2512.10166)
- [Multi-Agent Systems Powered by LLMs: Swarm Intelligence (Frontiers AI 2025)](https://www.frontiersin.org/journals/artificial-intelligence/articles/10.3389/frai.2025.1593017/full)
- [Collective Stigmergic Optimization (Medium 2025)](https://medium.com/@jsmith0475/collective-stigmergic-optimization-leveraging-ant-colony-emergent-properties-for-multi-agent-ai-55fa5e80456a)
- [Towards a Science of Scaling Agent Systems (arxiv 2512.08296, 2025-12)](https://arxiv.org/abs/2512.08296)
- [Random Scaling of Emergent Capabilities (arxiv 2502.17356)](https://arxiv.org/abs/2502.17356)
- [Tree of Options: Temporally Extended World Modeling with LLMs (ICLR 2025)](https://openreview.net/forum?id=uZFpfHSgN0)
- [Tree of Thoughts (NeurIPS 2023)](https://openreview.net/forum?id=5Xc1ecxO1h)
- [LLM-MCTS](https://llm-mcts.github.io/)
- [Monte Carlo Planning with LLM for Text-Based Games (arxiv 2504.16855)](https://arxiv.org/pdf/2504.16855)
- [LLMs Can Plan Only If We Tell Them (ICLR 2025)](https://proceedings.iclr.cc/paper_files/paper/2025/file/c1e67cde895c3c91edb43569ad0df260-Paper-Conference.pdf)
- [Externalization in LLM Agents: Unified Review (arxiv 2604.08224)](https://arxiv.org/pdf/2604.08224)
- [Embodied AI from LLMs to World Models (IEEE CAS Q4 2025)](https://mn.cs.tsinghua.edu.cn/xinwang/PDF/papers/2025_Embodied%20AI%20from%20LLMs%20to%20World%20Models.pdf)
