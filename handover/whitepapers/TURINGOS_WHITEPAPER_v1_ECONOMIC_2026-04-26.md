# TuringOS Whitepaper v1 — Economic Mechanism Chapter

> **Source**: User-authored 2026-04-26 ultrathink supplementation to `TURINGOS_WHITEPAPER_v1_2026-04-26.md`
> **Status**: Authoritative spec for economic subsystem; canonical source for RSP design
> **Position**: Sub-architecture under 反奥利奥 (Anti-Oreo); economic_state_t is part of system state Q_t, not external module
>
> Full 21-section text preserved verbatim from user message 2026-04-26 ultrathink turn. See implementation atomization in `TURINGOS_v4_FINAL_BLUEPRINT_2026-04-26.md` and `CO_MEGA_PLAN_v3.1_2026-04-26.md`.

## 核心校准 (§ 0)

> **TuringOS 的经济机制不是发币叙事，而是反奥利奥架构中的信号工程、问责机制与状态转移结算协议。**
> 区块链只承担 escrow、结算、挑战、状态根锚定与跨主体信任；不承担 AGI 推理本身。

经济机制本质：

```text
经济 = 信号管理 + 风险抵押 + 贡献归因 + 状态结算
```

NOT:

```text
经济 = 发币 + 炒作 + 链上跑模型
```

## Q_t 扩展 (§ 2 — 核心结构性 amendment)

economic_state_t 进入 Q_t：

```text
Q_t = <
  state_root_t,
  HEAD_t,
  tape_view_t,
  ledger_root_t,
  economic_state_t,            ← 经济章节核心扩展
  predicate_registry_root_t,
  tool_registry_root_t
>

economic_state_t = <
  balances_t,
  escrows_t,
  stakes_t,
  claims_t,
  reputations_t,
  task_markets_t,
  royalty_graph_t,
  challenge_cases_t,
  price_index_t
>
```

经济不是外部附属模块，而是 TuringOS 状态机的一部分。每一笔奖金、质押、挑战、slash、信誉变化，都必须成为 ChainTape 上的可验证状态转移。

## 12 Economic Invariants (§ 18)

完整复制于此作为 conformance test 直接来源：

```text
Invariant 1:  Agent 不因思考获得奖励，只因被接受的状态转移获得奖励。
Invariant 2:  Agent 不直接领取奖金，只提交 claim；Settlement Engine 决定。
Invariant 3:  所有奖金必须来自预锁定 escrow 或合法 treasury，不得事后增发。
Invariant 4:  on_init 之后不得铸造新的基础 Coin。
Invariant 5:  YES/NO 是事件绑定的风险权利，不是无抵押新币。
Invariant 6:  没有通过谓词的 work_tx 不得改变 world state。
Invariant 7:  通过谓词只产生 provisional reward；挑战期结束后产生 final reward。
Invariant 8:  贡献归因来自 Contribution DAG + 统计信号，不来自 Agent 自我声明。
Invariant 9:  信誉不可转让，不可替代谓词。
Invariant 10: 价格信号可以广播，完整评分器必须屏蔽。
Invariant 11: 链上记录承诺 + 状态根 + 结算结果；链下执行推理 + 测试 + 长上下文。
Invariant 12: 共识只能证明记录被接受，不能证明现实事实为真。
```

每条 → 至少 1 conformance test (`tests/economic_invariant_INV{N}.rs`)。

> **§ 18 修订 2026-04-27 cross-ref**: 上述 12 经济不变式 + `STATE_TRANSITION_SPEC_v1.1 § 4` 的 22 个 transition invariants（共 34 条）共同构成 v4 ship gate 的 conformance test 强制项。具体 invariant ID（I-DET / I-NOSIDE / I-PARENT / ... / I-VBOND-RELEASE / I-ROYALTY-CAP）见 STATE_TRANSITION_SPEC。

## RSP-1 模块 (§ 19)

```text
TaskMarket          发布任务、广播价格、锁定奖金
EscrowVault         保存 bounty + stake + deferred + royalty pool
ContributionLedger  记录 work_tx / verify_tx / challenge_tx / reuse_tx
PredicateRunner     执行验收 + 结算 + 货币守恒谓词
AttributionEngine   Contribution DAG → 贡献权重
ChallengeCourt      挑战期 + 反例 + 冻结 + 回滚 + slash
SettlementEngine    生成 settlement_tx + 释放奖金
ReputationIndex     维护非转让信誉
PriceIndex          广播任务价格 + 风险价格 + 资源稀缺信号
```

> **§ 19 修订 2026-04-27 注**: 共 9 个 modules。其中 `PredicateRunner` 在代码组织上**位于 `top_white::predicates::runner`**（顶层白盒，处理验收 + 结算谓词）；列在 RSP-1 是因为它在经济结算流程中扮演关键角色。`PriceIndex` 既是 RSP-1 第 9 module 也是 ChainTape L6 statistical signal 的 entry（cross-layer reference，per 反奥利奥 § 3 layer 边界）。架构章 RSP appendix 已对齐显示 9 modules（修订前曾列 8 modules，缺 PriceIndex；本次同步修订）。

## Agent 经济角色（5 object-level + 1 meta = 共 6 类）(§ 7)

> **修订 2026-04-27**: 原标题"Agent 5 经济角色"与下面 6 个角色列表 inconsistent；本次修订统一为「5 object-level + 1 meta」分层。Plan v3.2 CO P2.7 atom 数量按 6 角色派发；JudgeAI 在 v4 = 离线 dual audit (Codex + Gemini)，v4.1 才 promote 为 runtime actor（详见架构章 § 12.4）。

**Object-level 5 角色**（直接参与每一笔 task 的 work_tx / verify_tx / challenge_tx / reuse_tx）：

```text
Solver Agent      提交 work_tx 押注 YES_E
Verifier Agent    提交 verify_tx 抵押信誉 / bond
Challenger Agent  提交 challenge_tx 押注 NO_E
Builder Agent     创建可复用工具 → 收 deferred bonus + reuse royalty
ArchitectAI       提案新架构 → 收 meta bounty
```

**Meta-level 1 角色**（不参与单笔 task；Constitution Art V.1.3 守门人）：

```text
JudgeAI           否决违宪 → 不奖励"批准量"，奖励"低误判+低漏判+长期稳定"
```

## 5-Phase 部署 (§ 20)

```text
Phase 1: Local Ledger Economy   (Path B: gix runtime_repo + Rust predicates per Const Art 0.4)
Phase 2: Internal Task Market   (YES/NO stake + verifier + challenger + DAG + bonus)
Phase 3: Permissioned Settlement (multi-org chaincode escrow)
Phase 4: Rollup Settlement       (batch + economic_state_root + fraud proof)
Phase 5: Public AGI Market       (public escrow + cross-domain reputation + oracle)
```

> **§ 20 修订 2026-04-27**: Phase 1 描述原文为 "ledger.jsonl + SQLite + Python predicates"，那是早期路径 A 设想；2026-04-26 用户 ratify Path B（真 git substrate per Constitution Art 0.4）后实施 substrate 改为 gix Rust，本次修订统一描述。Substrate 选择见 `handover/specs/CO1_3_1_GIX_SPIKE_PREFLIGHT_v1_2026-04-27.md`。

turingosv4 scope: **Phase 1 + Phase 2 + Phase 3 prep** (per CO_MEGA_PLAN_v3.2)。Phase 3 prep deliverables 7 项见架构章 § 17 修订注。Phase 3 runtime + Phase 4-5 post-v4。

## 现代区块链技术定位 (§ 15)

| 技术 | TuringOS 作用 | v4 阶段 |
|---|---|---|
| Local GitTape / LedgerTape | early MVP 底座 | v4 P1+P2 ✓ |
| Permissioned ChainTape (Hyperledger Fabric) | 多组织背书 | v4.x |
| State Channels | 高频微结算 | v4.x |
| Optimistic Rollup | 开放市场挑战式结算 | v4.x |
| ZK / Validity Proof | 可形式化谓词快速结算 | v4.x |
| Oracles | 外部事实输入 | v4.x |

## 最终公式 (§ 21)

```text
reward_i =
  Finalize(
    Escrow(task)
    × Accept(tx_i)
    × Attribution(tx_i, ContributionDAG)
    × Survival(challenge_window)
    × Utility(post_acceptance_metrics)
    × Constitution(Q_t)
  )
```

## 与 Architecture Whitepaper 关系

- Architecture whitepaper § 5 ChainTape 6 layers → **L4 Transition Ledger 容纳所有经济交易类型**
- Architecture whitepaper § 7 Boolean signals → **acceptance_predicates + settlement_predicates + monetary_invariant** 都是 boolean
- Architecture whitepaper § 7 Statistical signals → **price_index + reputation_index + downstream_reuse_count** 都是 statistical
- Architecture whitepaper § 8 Broadcast price signals → **§ 11 经济价格信号广播** 是其经济实例化
- Architecture whitepaper § 9.4 Goodhart shielding → **§ 16 经济层 Goodhart 屏蔽**(public/private/commit-reveal predicates)
- Architecture whitepaper § 10 Laws → **§ 3 货币基本法 + § 4 CTF + § 5 Information/Investment** 操作化
- Architecture whitepaper § 12 Go Meta → **§ 7.5 ArchitectAI/JudgeAI 经济激励**

经济章节是 architecture whitepaper 的**忠实子集**——所有概念都已在 architecture chapter 中预定义，本章只是把 economy 作为 architecture 的一个子系统具体化。
