# TISR Phase 0 — Foundation Read 摘要集

**本文件为 forward-bound research 提案；任何派生 TB 须独立 §8 ratification；本文档不构成已批准方案。**

**目的**: TISR Phase 0 第 7 步 Foundation Read。读 7 个 TuringOS v4 关键文档，每个产出 300-500 字摘要，作为 Phase 1-5 设计依据。

**生成方式**: Phase 0 期间由 Explore subagent 完成，由主 agent 写入。

---

## 摘要 1: Constitution.md (Art. II + III + V)

**来源**: `constitution.md`

**核心论点**:
TuringOS 宪法确立了顶层白盒的三大信号工程职责：量化、广播、屏蔽。系统通过选择性广播价格信号引导 agent 群体演化，同时通过信息屏蔽防止错误传播和 Goodhart 问题，最终在三权分立（Constitution/ArchitectAI/VetoAI）的制度框架内实现自我进化。

**与 TISR 双轴关系**:
- **轴 A (Software 3.0 HCI)**: Art. II.2 价格信号的选择性广播（line 332）直接对应 "人=spec 批准者" 的角色。宪法把价格作为统计信号（非真理），广播给特定角色的 read-view，保证人类的初始设计获得机械强制执行。
- **轴 B (A2A 自治通信)**: Art. III 的信号屏蔽机制（line 360-512）与 Hayek 的分散知识通信原语对齐：价格广播给不同角色，每个角色只看到自己角色需要的信息，形成分层信息流。Art. V.1 三权分立（line 692-787）则奠定了 agent 自治学习的制度基础。

**关键 file:line 引用**:
- `constitution.md:299-358` (Art. II 信号广播，含 line 332 价格信号广播核心论述)
- `constitution.md:360-512` (Art. III 信号屏蔽，含 III.4 Goodhart 防护 line 413)
- `constitution.md:664-787` (Art. V 元架构三权分立，line 704 Constitution 地位、line 719 ArchitectAI 权限)
- `constitution.md:46` (Art. 0.1 图灵四要素映射，Strict discipline 对应 Π_p + Veto-AI)

**TISR 启示**:
宪法的三权分立框架（人立法→AI 提议→AI 否决）为 TISR Phase 0-5 的逐层权力分离奠定基础；价格信号的角色专属广播机制预示了后续 REAL-5/REAL-12 中角色视图的制度要求。

---

## 摘要 2: G-Phase Generative Arena Directive (2026-05-11)

**来源**: `handover/directives/2026-05-11_G_PHASE_GENERATIVE_ARENA_DIRECTIVE.md`

**核心论点**:
TB-N3 substrate 成功后，系统进入生成性竞技场阶段。问题不是宪法太严，而是 agent 没有跨问题持续身份、没有看到价格信号、没有持续经济后果，导致市场机制未涌现。G-Phase 通过建立跨问题持久化、市场决策可观测性、持续 PnL 反馈，把宪法从防御盾牌转化为竞技场边界。

**与 TISR 双轴关系**:
- **轴 A**: G0 Charter（line 169）和 G1-G7 七层结构将 "人的初始批准" 扩展为 "每层输出的明确验收标准"。人类设计的 ship gates（SG-G1.1 - SG-G7）成为每个阶段的 ground truth。
- **轴 B**: G1 Cross-Problem Persistence（line 190-230）要求 agent balance/positions/reputation 跨问题存续，直接映射到 Nakamoto 持续激励原则；G2/G3 的 MarketDecisionTrace（line 232-284）和 PnL 反馈（line 264-284）是 "价格信号自由流动" 的制度实现。

**关键 file:line 引用**:
- `:19-31` (宪法转竞技场核心命题)
- `:111-133` (四思想家视角：Hayek/Nakamoto/Turing/Drucker)
- `:147` ("现在不要再开新方向。先把 G-Phase 作为一个整体收口" — TISR carve-out 关键引用)
- `:190-230` (G1 Cross-Problem Persistence 最高优先级)
- `:334-349` (G6 价格信号 observe-only，"Price is signal, not truth")

**TISR 启示**:
G-Phase 七层递进架构明确了 TISR 各阶段的 HCI 输出点（每层 Charter + ship gates）和 A2A 通信基础（跨问题身份、价格广播、role 分化测量）；是 TISR 双轴从宪法到涌现的具体路线图。

---

## 摘要 3: REAL-5 Role-Based Generative Scaffolding (2026-05-14)

**来源**: `handover/directives/2026-05-14_REAL5_ROLE_BASED_GENERATIVE_SCAFFOLDING_ARCHITECT_ORIGINAL.md`

**核心论点**:
REAL-5 把 "角色" 从 prompt 标签升级为宪法制度：每个角色（Solver/Verifier/Challenger/Trader/MarketMaker/Architect/Veto/Observer）都有明确的生成义务、角色专属视图、成本预算、谓词门槛、ChainTape 化石记录。这才是反奥利奥的生成引擎——不强迫交易，但强迫 "在制度脚手架内可观测地做出角色判断"。

**与 TISR 双轴关系**:
- **轴 A**: Atom 1-3（line 69-190）定义了 AgentRoleAssignment、DerivedView、PromptCapsule，把 "人批准的角色设计" 编码为机器可执行的白盒结构。Atom 0 Charter（line 45-67）强制显式声明 "不改宪法、不强迫交易、price 不成 truth"。
- **轴 B**: Atom 2 Role-Scoped rtool（line 107-167）实现了信息屏蔽的制度化——Trader 看见价格但 Solver 不看，Verifier 看证明但不看交易动机。Atom 6（line 257-285）的 Trader 角色强制输出 MarketDecisionTrace or NoTradeReason，标志着 agent 经济判断的首次可观测化。

**关键 file:line 引用**:
- `:3-28` (REAL-5 目标陈述：role-based generative scaffolding)
- `:69-167` (Atom 1-2：AgentRoleAssignment + DerivedView)
- `:257-285` (Atom 6 Trader activation + NoTradeReason)
- `:436-454` (硬约束清单：no price-as-truth, no forced trade, every action L4-anchored)

**TISR 启示**:
REAL-5 是 "轴 A 的人类设计" 到 "轴 B 的 agent 自治" 的过渡层：通过角色专属视图和强制经济判断化石化，使得后续的 REAL-12 BullTrader/BearTrader 分化有了清晰的 "可观测" 基础。

---

## 摘要 4: REAL-12 Role-Specialized Economic Agents (2026-05-16)

**来源**: `handover/directives/2026-05-16_REAL12_ROLE_SPECIALIZED_ECONOMIC_AGENTS_ARCHITECT_ORIGINAL.md`

**核心论点**:
REAL-12 承认当前瓶颈不是市场结构或宪法缺陷，而是 "agent 没有 live economic motive"。对标 v3 的短反馈回路，系统强制分配 BullTrader/BearTrader 角色，每个角色有专属视图、强制经济判断（EconomicJudgment 必填），允许 Abstain 但需结构化理由。关键是：强制角色分化，不强制交易行为，但强制判断可观测化。

**与 TISR 双轴关系**:
- **轴 A**: Atom 0（line 325-345）在 REAL-11 诊断基础上，由人类架构师明确裁决 "E2 未达成但路径清楚"。后续 Atom 1-5（line 347-497）都由人类批准的 ship gates 控制，确保每步输出都可验收。
- **轴 B**: Atom 2 BullTraderView/BearTraderView（line 392-429）把 Hayek 的 "价格作分散知识通信协议" 具体化——Bull 只看多头机会，Bear 只看空头信号，两个 trader agent 视图互补。Atom 3 EconomicJudgment（line 431-459）强制将经济判断论述化，成为后续 A2A 协商的结构化语言。

**关键 file:line 引用**:
- `:165-187` (瓶颈判断：live economic motive 缺失)
- `:285-311` (角色强制 vs. 行为强制的宪法边界)
- `:392-429` (BullTraderView/BearTraderView 角色视图设计)
- `:431-459` (EconomicJudgment schema + Abstain 必须有理由)

**TISR 启示**:
REAL-12 通过强制角色、强制判断化石化、允许自主选择，实现了 TISR 轴 B 中 "agent 自由通信但在人类设计的制度框架内" 的原则；BullTrader vs BearTrader 的对抗结构预示后续 REAL-13+ 的市场涌现。

---

## 摘要 5: TuringOS Generative Economy Whitepaper Update (REAL-9)

**来源**: `handover/whitepapers/TURINGOS_GENERATIVE_ECONOMY_WHITEPAPER_UPDATE_REAL9.md`

**核心论点**:
REAL-9 白皮书重新定义了 v4 与 v3 的关系：v3 教会我们 "市场压力" 的威力，v4 在宪法约束下重建这种压力。核心申明 "price = signal, not truth" 和 "market = role-specific institution, not prompt decoration"，否定了 v3 的宪法风险（f64、off-tape、强制投资），转向 "在 tape 上可审计、可结算、可竞争、可学习的智能市场"。

**与 TISR 双轴关系**:
- **轴 A**: 白皮书明确拒绝 v3 的自动化陷阱："price-as-truth" 和 "dashboard as source of truth" 违反宪法。人类的角色界定（role assignment 由人批准，ship gates 由人设立）被写入白皮书承诺，是 TISR 轴 A 的政治宣言。
- **轴 B**: "price = signal" 原则把价格还原为 Hayek 的通信协议角色；role-specific market institution 意味着 Trader 和 Solver 看到不同的价格视图，从而产生信息非对称下的自主协商。

**关键 file:line 引用**:
- `:17-31` (v3 taught pressure, v4 gives law. Next phase must build lawful pressure)
- `:5-12` (price = signal not truth; market = role-specific institution)

**TISR 启示**:
白皮书是 TISR 项目的价值观锚点：价格信号的 "通信" 角色、角色制度化的 "合宪" 要求、agent 自治的 "可观测" 约束，三者构成 TISR 双轴的哲学层基础。

---

## 摘要 6: src/state/typed_tx.rs (关键章节)

**来源**: `src/state/typed_tx.rs` (lines 2327-2406, 82, 365)

**核心论点**:
TypedTx enum（19 variants）是宪法中 "量化" 原则的代码实现：每个 variant 代表一个明确的经济行动或系统状态转移，不允许模糊或多义解释。AgentSignature vs SystemSignature 的类型区分（line 82）强制系统白盒和 agent 黑盒的权力边界，将宪法的 "agent ≠ direct state writer" 原则编码为编译时类型安全。

**与 TISR 双轴关系**:
- **轴 A**: BuyWithCoinRouter 和 EventResolve（line 2371, 2379）的分离体现了人类设计的市场结构（router 是宪法允许的工具，EventResolve 是系统专属的状态转移）。每个 variant 都附带 "何时执行、谁可执行" 的注释，是人类 "spec 批准" 在代码中的直接体现。
- **轴 B**: 角色分化后，不同的 agent 只能发出特定的 TypedTx variants（line 4054-4089 sequencer.rs 的 agent ingress 屏蔽）。例如 BullTrader 只能发 BuyWithCoinRouter，不能发 FinalizeReward；这种类型约束实现了宪法中 "屏蔽" 原则对 A2A 通信的制约。

**关键 file:line 引用**:
- `typed_tx.rs:2327-2380` (TypedTx 19 variants 完整枚举)
- `typed_tx.rs:82-90` (AgentSignature vs SystemSignature 类型区分)
- `typed_tx.rs:2371` (BuyWithCoinRouter agent-signed variant)
- `sequencer.rs:4054-4089` (agent ingress 屏蔽列表，SystemTxForbiddenOnAgentIngress 检查)

**TISR 启示**:
TypedTx 的类型系统和 sequencer 的 ingress barrier 把宪法的 "量化" 和 "屏蔽" 原则从文字法律转化为编译器可强制的机制；是轴 A 的 "人类设计" 如何变成轴 B 的 "系统强制" 的最小例证。

---

## 摘要 7: src/state/sequencer.rs (关键段落)

**来源**: `src/state/sequencer.rs` (lines 4000-4089)

**核心论点**:
Sequencer::submit_agent_tx() 的 ingress barrier（line 4037-4090）是 Anti-Oreo 宪法原则的执行体：通过模式匹配，严格区分 agent-submitted 和 system-emitted tx variants，拒绝 agent 直接写入系统状态（FinalizeReward, TaskExpire, EventResolve 等）。这个 ~60 行的屏蔽机制体现了宪法 Art. V.1.3 的 "agent ≠ state writer" 权力边界。

**与 TISR 双轴关系**:
- **轴 A**: 屏蔽列表在 TB-5（line 4022 TRACE_MATRIX FC2-Submit）由人类架构师明确批准。每次新增系统 tx variant（如 TB-11 的 TaskBankruptcy，line 4046-4049），都要经过人类的显式 amendment，确保权力边界的清晰性。
- **轴 B**: agent 能发送的 tx 被严格限制，但同一次提交中可包含多个合法的 agent-signed tx（CompleteSetMint, BuyWithCoinRouter 等）。这种 "whitelist" 而非 "blacklist" 的机制确保了 agent 的有限自治权空间，避免了宪法 "漂移"。

**关键 file:line 引用**:
- `sequencer.rs:4037-4090` (submit_agent_tx ingress barrier，包含完整的屏蔽匹配)
- `sequencer.rs:4054-4056` (SystemTxForbiddenOnAgentIngress 错误类型定义)
- `sequencer.rs:4046-4053` (TB-11/TB-N2 新增系统 tx 的注释，说明每次修宪的审查过程)

**TISR 启示**:
sequencer 的 ingress barrier 是 TISR 轴 A-to-B 的权力交接口：人类在这里设置最后的 "yes/no" 门槛，agent 在这里被强制尊重宪法的权力边界；后续 agent 的一切行动都在这道光栅内展开，形成了自治与约束的张力平衡。

---

## 最终综合判断 (Foundation Read 阶段 synthesis)

TISR Phase 0 Foundation Read 的七份文档和源文件描绘了一个完整的系统架构：从**宪法的量化、广播、屏蔽**（摘要 1），经过**生成竞技场的阶段化设计**（摘要 2），到**角色制度化和经济判断化石化**（摘要 3-4），再到**代码层的类型系统和权力边界**（摘要 6-7），最后由**白皮书**（摘要 5）统一承诺价值观。

核心张力是：**人类通过宪法设置不可更改的游戏规则，agent 在规则内通过价格信号、角色分化、跨问题身份实现自治通信，系统通过 tape-first 和可观测性防止权力偏移**。这是 TISR 双轴从描述层（轴 A：人=spec 批准者）到运作层（轴 B：agent 自由通信在制度框架内）的完整闭环。

### Phase 1-5 设计参考要点

1. **Phase 1 CLI 设计** 应充分利用 sequencer ingress barrier 作为 "人=spec 批准+init 启动者" 的具体接口点；CLI 触发的所有 tx 都走 system_emitted 路径，不冲击 agent ingress whitelist。

2. **Phase 2 Track A (typed_tx)** scope 已经被 Foundation Read 校验：19 variants 已极其完备，TISR 不应轻易提议新 variant；任何新 variant 都触发 Class 4 STEP_B。

3. **Phase 2 Track D (经济学)** 设计必须以 REAL-9 白皮书的 "price = signal, not truth" 为底线；TISR 不能 promote price 到 truth。

4. **Phase 2 Track B (ChainTape/CAS/HEAD_t)** 设计应充分利用现有 PromptCapsule (Art. III) / EvidenceCapsule / MarkovCapsule 框架；新 capsule schema 是 Class 1-3，不冲击 Class 4。

5. **Phase 2 Track C (Materializer)** 必须**从 audit_dashboard.rs 完全独立**做 from-scratch web layer；audit_dashboard 是 text-only，不能 "扩展"。

6. **Phase 2 Track E (谓词)** 设计应沿 REAL-5/REAL-12 的 EconomicJudgment 制度化模型；新增 UIJudgment / A2AMessageJudgment 在 schema 层可行。

7. **Phase 3 横向调研** 应聚焦 "TuringOS 缺什么"：Hayek 已落地为价格广播；Nakamoto 持续身份已部分落地（G1）；Turing tape 已落地；**缺**: 多模态、可验证 AI、AGI 认知架构、MCP 之外的 A2A 协议。
