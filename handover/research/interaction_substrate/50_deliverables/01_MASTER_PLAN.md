# TISR Deliverable 01 — Master Plan

**本文档为 forward-bound research 提案；任何派生 TB 须独立 §8 ratification；本文档不构成已批准方案。**

**Phase**: TISR Final Deliverable 01 | **Status**: TISR Phase 5 总纲领

---

## 1. 项目身份与愿景

**项目名**: TuringOS Interaction Substrate Research (TISR)
**Phase ID**: TISR-001
**Charter Date**: 2026-05-17
**Architect Carve-out**: Worktree 物理隔离; 不消耗 G-Phase 收口工时; PR 时 surface 给架构师

### 1.1 双轴愿景 (v2.1 阶段化版本)

> **轴 A (Software 3.0 HCI)**: 现阶段人 = constitution-level 立法者 + read-view 一等公民 + spec 批准 + init 启动; 未来留 agent 自接入扩展面
>
> **轴 B (A2A 自治通信基底)**: agents 在 tape 上自由通信; 价格信号作 Hayek 通信原语; REAL-5/12/BCAST-1 角色分化 + 大规模集群涌现
>
> **交汇命题 v2.1**: 人代理 agent (Agent_user_0) + agent (自治执行) 通过同一条 tape + 同一套谓词通信; Signature 二元 (system/agent), ingress 三模式 (system_emitted / agent_submitted via Agent_user_0 / agent_submitted via 其他 agent); 人的特权在 constitution-level 而非 ingress-level

### 1.2 与用户上传 Generative HTML 报告的关系

报告 = 优秀的轴 A 设计起点 (8 大命题被验证). 但严重低估了 TuringOS 独有的**轴 B + 经济模型 + 监管对齐**价值 (12 弱项补充). TISR 在保留报告 4 核心架构 (UI IR / Event Bridge / Policy / Audit Store) 基础上, 主动定位 TuringOS 为 **AGI 时代可验证 A2A substrate**, 不是 generic UI 项目.

详见: `40_synthesis/gap_analysis.md`

---

## 2. TuringOS 长期定位

### 2.1 三大独特价值 (vs 商业 agent platforms)

1. **可验证 substrate** (vs LangChain/AutoGen/Replit Agent 4)
   - ChainTape + CAS + replay → "agent 失败可被独立第三方追责" 协议级保证
   - 监管对齐风口: EU AI Act §12 record-keeping (2024-2025) 几乎 spec 上要求 ChainTape 等价物

2. **constitutional runtime** (vs Anthropic CAI training-time)
   - 运行时强制宪法; 与 Anthropic CAI 哲学**正交互补**
   - Anthropic 2025-2026 Constitutional Classifiers 转向 runtime, 与 TuringOS 收敛 ✅

3. **内生经济市场** (vs Bittensor token subsidy)
   - 1 Coin = 1 YES + 1 NO 守恒; 无 inflation (closed-economy arena)
   - 价格 = Hayek 通信原语 (signal not truth, Art. II.2)

### 2.2 长期角色

- **现阶段 (2026)**: Lean proof market + Polymarket smoke; user + agent 共生 substrate
- **中期 (2027-2028)**: 通用 task market; 多模态扩展; 跨实例 agent 联邦
- **长期 (2029+)**: AGI 时代可验证 A2A substrate; 监管合规默认; 人立法 + agent 自治市场

---

## 3. TISR 项目结构

### 3.1 5 Phase 研究 + 4 Phase 实施

```
TISR 研究 (Class 0, 2026-05-17):
  Phase 0: 基础设施 + Existing Surface Inventory   ✅ 完成
  Phase 1: Unified CLI Design (4 子 track)          ✅ 完成
  Phase 2: 并行架构映射 (5 tracks)                  ✅ 完成
  Phase 3: 并行前沿调研 (5 tracks)                  ✅ 完成
  Phase 4: 综合 + 差距分析                          ✅ 完成
  Phase 5: 6 份最终交付 (本文档系列)                ✅ 完成

TISR 后续实施 (separate charter, 待 G-Phase 完成):
  Phase 6: CLI MVP (Class 1-2, ~6-8 周)
  Phase 7: Web UI MVP (Class 1-2 + cas/schema 扩展, ~10-12 周)
  Phase 8: A2A 深化 + 协作 + 联邦 (Class 2-3, ~10-12 周)
  Phase 9+: 评估 / 长期 (Yjs / zkML / 跨实例)
```

### 3.2 总 LOC + Class 估算

- Phase 6 MVP: ~5900 LOC (Class 1-2; 0 Class 4)
- Phase 7 Web: ~10000+ LOC (Class 1-3; cas/schema.rs +5 variants 是 Class 4 候选)
- Phase 8 A2A 深化: ~5000+ LOC (Class 2-3; 部分 typed_tx 扩展 Class 4 候选)
- Phase 9+: 视长期评估

---

## 4. 双轴战略

### 4.1 轴 A (Software 3.0 HCI) 路线图

| Phase | 目标 | 交付 |
|---|---|---|
| 6 (CLI MVP) | lean_market → turingos 演化; 37 subcommand; 整体 TuringOS 用户视角接通 | `00_UNIFIED_CLI_SPEC.md` |
| 7 (Web MVP) | from-scratch HTML Workspace; 4 层信息架构; Turing UI IR + Event Bridge + Policy + Audit Store | `03_CODE_INTEGRATION_SPEC.md` |
| 8 (Collaboration) | Task-granular lease; optimistic update; WebSocket real-time sync | (separate charter) |
| 9+ (扩展) | Yjs/CRDT 评估; voice-first UX; VR/spatial 接口 (forward-bound) | (long-term) |

### 4.2 轴 B (A2A 自治通信) 路线图

| Phase | 目标 | 交付 |
|---|---|---|
| 7 (Substrate) | A2AMessageCapsule CAS schema; PeerVerificationVote (optional); 价格 3 层协议 | `04_A2A_PROTOCOL_DESIGN.md` |
| 8 (Maturity) | DID 桥 (`did:turingos:`); MCP server (read-only tape view); ChainTape 周期性外锚 | (separate charter) |
| 9 (涌现验证) | REAL-N (N≥16) 涌现验证里程碑; Active Inference 角色 agent 参考实现 | (Track J Q4) |
| 10+ (AGI) | DirectSwapTx (Class 4); AgentProposedTaskOpen (Class 4); 跨实例联邦 | (long-term) |

---

## 5. 现阶段优先级矩阵

### 5.1 P0 (G-Phase 收口后立即启动)

1. **Phase 6 CLI MVP** (~6-8 周) — TuringOS 用户视角完整接通
2. **真问题 witness 累积**: lean_market → turingos happy path 跑通

### 5.2 P1 (Phase 6 ship 后)

1. **Phase 7 Web MVP** (~10-12 周) — HTML-first Workspace
2. **cas/schema.rs ObjectType 扩展 §8 ratification** (架构师独立批准)
3. **UIContentPredicate / A2AMessageSignatureVerifier 注册** (新 predicate)

### 5.3 P2 (Phase 7 ship 后)

1. **DID 桥** (`did:turingos:` URI)
2. **MCP server** (read-only tape view; AGI agent 互操作)
3. **ForecastCapsule + ReflectionTx schema** (Track J 优先 capsules)
4. **REAL-N (N≥16) 涌现验证里程碑**

### 5.4 P3 (长期评估)

1. **Yjs/CRDT 评估** (仅 read-side concurrent audit)
2. **zkML proof in EvidenceCapsule** (Track I Q1)
3. **跨 TuringOS 实例联邦** (Track I Q5)
4. **Voice-first / VR / 具身 AI** (Track G; 长期可选)

---

## 6. 资源 + 工时估算

### 6.1 TISR 研究阶段 (已完成)
- Phase 0-5: 1 session, ~13-16h Claude 工时, 19+ 份文档, ~10000+ 行

### 6.2 TISR 后续实施阶段 (待启动)

| Phase | 工时 | LOC | 团队规模 (建议) |
|---|---|---:|---|
| Phase 6 CLI | 6-8 周 | ~5900 | 1-2 Rust 工程师 |
| Phase 7 Web | 10-12 周 | ~10000+ | 1-2 Rust + 1-2 frontend (React) |
| Phase 8 A2A | 10-12 周 | ~5000+ | 1-2 Rust |
| Phase 9+ | 长期 | (视项目) | (视项目) |

**总 (Phase 6-8)**: ~26-32 周; 2-3 工程师并行; 待 G-Phase 收口后启动.

---

## 7. 关键里程碑

| 里程碑 | 验收标准 | Phase |
|---|---|---|
| **TISR 研究 ship** | 6 份 deliverable + 0 Class 4 修改 + Charter §5 8 kill condition 不触发 | Phase 5 (本文) ✅ |
| **Phase 6 CLI ship** | Happy path 12 步; evidence bundle PROCEED; ProvenanceCapsule 区分 human/agent | Phase 6 |
| **Phase 7 Web ship** | 3+ 用户独立 Web form 提交; ≥1 VetoAI Reject; 多模态 artifact ≥3 type | Phase 7 |
| **A2A 协议 ship** | A2AMessageCapsule + LibrarianBroadcast 扩展; 价格三层协议 enforce | Phase 7-8 |
| **DID 桥 ship** | `did:turingos:` 跨实例可读; W3C DID v1.1 兼容 | Phase 8 |
| **N≥16 涌现验证** | 16-32 agent batch 涌现现象 + ChainTape 完整记录 | Phase 9 |

---

## 8. 风险矩阵

| 风险 | 缓解 |
|---|---|
| G-Phase 收口长期未完成 → TISR Phase 6+ 卡住 | TISR 研究阶段 (Phase 0-5) 已 ship; Phase 6+ 后续启动不阻塞当前 ChainTape |
| 架构师 PR 阶段 VETO TISR | Worktree 物理隔离; 无 main 影响; archived 重启即可 |
| cas/schema.rs ObjectType 扩展 Class 4 §8 失败 | Phase 7 拆分: Phase 7.0 完全无 cas/schema 修改 (UIEventCapsule 用 Generic ObjectType); Phase 7.1 申请 §8 后扩展 |
| Phase 7 Web LOC 超预算 | 严格 Turing UI IR + Materializer 边界; 不写自由 HTML |
| 真问题 witness 跟不上 → "design over evidence" | 每 Phase 出口强制 real-problem witness; 多模态延后 (Phase 9+) |

---

## 9. 杀手条件 (持续监控)

| Kill Condition | 触发 | 动作 |
|---|---|---|
| KC1 | 架构师明确 VETO TISR | archive 全部 |
| KC2 | G-Phase 期间新 directive 禁止并行 research | pause |
| KC3 | Phase 0-3 任 Track 命题被现 surface 覆盖 80%+ | archive 该 Track (Phase 0 验证 OK) |
| KC4 | Phase 6 CLI 实际覆盖 ≥ 90% | 缩为薄包装 |
| KC5 | 任 Phase 提议需 typed_tx schema 修改 | 标 Class 4 forward-bound |
| KC6 | Phase 3 调研发现用户报告 70%+ 覆盖 | 缩减或 archive |
| KC7 | 任 deliverable 暗藏 Class 4 surface | Charter 补丁 + 警告头部 |
| KC8 | G-Phase 完成时间表给出 | re-evaluate scheduling |

---

## 10. 与外部 AGI 研究的对照

| 研究方向 | TuringOS 立场 | 来源 |
|---|---|---|
| Karpathy Software 3.0 | 对齐 + 更激进 (协议层 vs UI 层) | Phase 3 Track H |
| Anthropic Constitutional AI | 哲学正交互补 (training-time vs runtime); 2025-2026 收敛 | Phase 3 Track I |
| Anthropic Effective Agents | TuringOS = "constitutional workflow with agent-shaped LLM nodes" | Phase 3 Track H |
| Google A2A 协议 | 借鉴 AgentCard / Task / Artifact 信封建模; 补充经济学 + verifiability | Phase 3 Track F |
| MCP (Model Context Protocol) | 工具协议 (LLM ↔ tool); 互补不替代 A2A | Phase 3 Track F |
| Bittensor / Fetch.ai | 经济模型不同 (TuringOS 内生 Coin vs token subsidy) | Phase 3 Track I |
| JEPA / V-JEPA 世界模型 | TuringOS tape 已是 explicit external world model; 不需 V-JEPA latent | Phase 3 Track J |
| Active Inference / Free Energy | 不替换 Hayek 价格信号; 可作角色 agent 内部参考实现 | Phase 3 Track J |

---

## 11. 结语

TISR 研究阶段 (Phase 0-5) 提供了 TuringOS 从"防御宪法基底" 走向"AGI 时代可验证 A2A substrate" 的完整 forward-bound 路线图. **0 个 Class 4 修改提议** (全 forward-bound 标记); 与 G-Phase 收口工作完全 worktree 物理隔离; 与 2025-2026 AGI 前沿研究系统对照.

**TuringOS 不是"再做一个 agent 框架";**
**TuringOS 是"AGI 时代第一个可验证 + 经济驱动 + 监管合规的 agent 操作系统底层基底"**.

Phase 5 6 份 deliverable 是后续 Phase 6-9 实施的 source-of-truth; 每 Phase 启动需独立 charter + 架构师 §8 ratification.

---

## 12. 6 份 Deliverable 引用

- `00_UNIFIED_CLI_SPEC.md` — Phase 6 CLI 源头规范
- `01_MASTER_PLAN.md` (本文)
- `02_CONSTITUTIONAL_ALIGNMENT.md` — 每特性 vs 宪法条款映射
- `03_CODE_INTEGRATION_SPEC.md` — 代码级 module 详细
- `04_A2A_PROTOCOL_DESIGN.md` — 轴 B 重点
- `05_ROADMAP_AND_KILL_CRITERIA.md` — 12 个月路线图

**TISR 完成**: 现阶段 ideation + research 已 ship. Phase 6+ 实施待 G-Phase 收口 + 架构师独立 ratification.
