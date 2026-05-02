# Part B — First Architectural Plan (verbatim, SUPERSEDED)

**Source**: user message 2026-05-02. The user explicitly stated "以最后的为准" — Part C supersedes this plan.
**Status**: VERBATIM PRESERVATION (per `feedback_kolmogorov_compression`). Preserved as superseded text for audit.
**Supersession note**: Part C (`__part_C_updated_final_ruling.md`) is the canonical ruling. Part B is preserved because:
1. Part B's TB-7R audit, Flowchart-1/2/3 derivations, and v1.0 ship-gate definition are still useful reference material — Part C builds on them rather than rejecting them.
2. The Kolmogorov-lossless principle requires preserving full context, not selective omission.

The user introduced Part B with the sandbox download link:
> [下载：TuringOS_Official_Launch_Plan_After_TB7R_2026-05-02.md](sandbox:/mnt/data/TuringOS_Official_Launch_Plan_After_TB7R_2026-05-02.md)

The transition from Part B to Part C in the user's message was the Chinese semicolon:
> ；我读完你补充的工作笔记和 Gemini DeepThink 的评价后

---

## Verbatim text begins

# 1. 总裁决

**这份更细致的"无损宪法整编版"确实给当前整体方案提供了更多可执行细节，而且应当修正后续路线。**

它不是简单"补充哲学"。它提供了三个更强约束：

```text
1. 三张 flowchart 现在应成为开发验收标准，而不只是说明图。
2. logs archive as ground truth + 马科夫规则，应成为 Go Meta / Information Loom 的工程约束。
3. Boot / InitAI / JudgeAI / ArchitectAI 的三层元架构，要求我们不要过早进入 MetaTape，而要先把 runtime、boot、logs、dashboard、payout 打实。
```

无损宪法整编版明确说，它不是普通摘要，而是"宪法主文 + 注释层 + 完整原文归档 + 视觉校验资产"，并且三张 flowchart 原样嵌入并给出 SHA256 校验。 这意味着：**以后不能只引用自然语言版宪法，还要把 flowchart 本身纳入 TRACE_MATRIX / TB gates。**

---

# 2. 当前 TB-7R 状态审计

根据当前 repo handover，TB-7R 已经 shipped，并且不是普通 shipped，而是 Class 3 双审通过：

```text
TB-7R SHIPPED
Constitution-Aligned Frame B Repair
Codex round-1 VETO -> surgical remediation -> Codex round-2 PASS
Gemini strategic-tier PASS
cargo test --workspace = 712 passed / 0 failed / 150 ignored
```

当前 handover 还写明，TB-7R 的 4-clause acceptance 与 7-condition ship gate 全部 green，包括：

```text
every externalized proposal -> L4/L4.E
predicate evidence resolves from CAS
failed proposals shielded but auditable
dashboard regeneratable from ChainTape + CAS alone
```

这些已经落入 handover 最新状态。

所以我的状态裁决是：

```text
TB-7R = 通过。
P2 Frame B = 可视为完成。
下一阶段不应再纠结"真实 proposal 是否进 ChainTape"这个大问题。
```

但有 open follow-ups：

```text
OBS-1 coverage denominator:
  PartialOk -> Complete proof-prefix dependency

OBS-R022 TRACE_MATRIX orphans:
  chaintape_mode_gate.rs + genesis_report.rs 仍需未来 TRACE_MATRIX 行

CHECKPOINT_TB7R_2 #1:
  旧 evidence README annotation 被 editor hook revert
```

这些不是 TB-7R blocker，但会影响后续正式上线的严谨性。

---

# 3. 新宪法给当前方案带来的新增修正

## 3.1 Flowchart 1：基础运行循环必须成为验收主轴

无损宪法中 Flowchart 1 被定义为基础运行循环，并以第 8–9 页跨页原图保留。 它对应：

```text
Q_t
-> read tool
-> Agent δ
-> output
-> predicates
-> write tool
-> Q_{t+1}
```

这带来一个硬约束：

> **所有外部化 Agent proposal 必须进入 L4 或 L4.E；dashboard 只能是 materialized view；legacy append 不得再成为 authoritative path。**

TB-7R 已经大体满足这一点。
下一阶段要继续保持：

```text
L4 accepted = predicate-passed state transition
L4.E = predicate-failed rejection evidence
dashboard = view, not source of truth
```

这也解释了为什么我们上一轮坚持：不能为了让 DAG 好看，把 Lean-failed proposal 强行放入 L4 accepted。

---

## 3.2 Flowchart 2：Boot 与完整循环要求所有初始化进入 ChainTape

Flowchart 2 是 Boot 与完整循环。无损宪法的 Boot 段落写得很清楚：

```text
Boot = 初始化顶层白盒规则 + 初始世界状态
```

它的本质是把人类 spec 编译成 predicates，并写入系统信任根，从而允许黑盒 Agent 在验证约束下持续演化世界状态。

这对当前方案的修正是：

```text
不再允许 memory-only preseed 作为正式上线证据。
TaskOpenTx / EscrowLockTx / on_init / initial balances 必须可 replay。
每个正式 run 必须有 genesis_report。
```

TB-7R 已经在这方面推进：`genesis_report.json`、on-chain TaskOpen/EscrowLock verification 已落地到 TB-7R commit chain 中，当前 handover 也记录为 green。

但正式上线前还必须完成：

```text
P0.R RootBox ceremony
production no pending placeholders
durable boot attestation
```

这不是 beta demo 的 blocker，但会是 v1.0 正式生产的 blocker。

---

## 3.3 Flowchart 3：元架构要求日志归档 + Markov 规则

这是新宪法对当前路线最有价值的升级。

Flowchart 3 把：

```text
constitution as ground truth
logs archive as ground truth
JudgeAI
ArchitectAI
boot
log
archive
feedback
re-init
```

放入元架构循环。无损整编版还明确解释：`constitution as ground truth` 与 `logs archive as ground truth` 是元架构顶层白盒，`JudgeAI / ArchitectAI` 是元架构中层黑盒，tools/log/Q/anti-oreo runtime 是元架构底层白盒。

更重要的是"马科夫规则"：系统日志会指数膨胀，InitAI 不应每次从大爆炸日志开始读，而应默认只读上一轮最终错误日志与当前宪法；历史日志保留但不默认进入上下文。

这对当前方案的新增要求是：

```text
1. 每个 TB / run 结束后必须生成 EvidenceCapsule。
2. logs archive 是 ground truth，但 Agent / InitAI 不能读全历史。
3. InitAI 默认只读 latest capsule + current constitution。
4. 只有顽疾问题才允许 Markov override 深读历史。
5. Dashboard / Audit report 应成为 logs archive 的 materialized view。
```

所以 **TB-8 之后必须加 Information Loom / Markov Log Loom**，否则 Go Meta 会在长周期内被日志淹没。

---

# 4. 目前整体路线是否需要改？

需要，但不是推翻。

我建议把原先"继续经济复杂化 / NodeMarket / Slash"的路线，调整成上线导向：

```text
TB-8   Minimal payout / FinalizeRewardTx
TB-9   Durable AgentRegistry + Wallet projection
TB-10  Lean Proof Task Market MVP
TB-11  Markov Log Loom / Information Loom v0
TB-12  Challenge slash / RSP-3.2
TB-13  NodeMarket v0
TB-14  Beta release
TB-15  Official v1.0 launch
```

这个顺序比"先 NodeMarket / Slash / P6 研究扩展"更接近正式上线。

---

# 5. 正式上线定义

我建议把 "TuringOS 正式上线" 定义为一个窄而完整的产品：

```text
TuringOS v1.0 = Lean Proof Task Market on ChainTape
```

不是完整 AGI 自治经济体。

v1.0 必须支持：

```text
1. 用户发布 Lean proof task。
2. 用户 escrow bounty。
3. Agent 读取 scoped view。
4. Agent 生成 proposal。
5. Proposal 进入 CAS。
6. WorkTx 进入 L4 或 L4.E。
7. Lean predicate / VerificationResult 进入 CAS。
8. Accepted proof 可 replay。
9. Challenge window 简化存在。
10. FinalizeRewardTx 支付给 solver。
11. Dashboard 可展示完整事件。
12. 运行可从 ChainTape + CAS 重建。
```

不属于 v1.0 blocker：

```text
NodeMarket trading
AMM
public chain
MetaTape
multi-org
royalty
long-term reuse market
per-tactic DAG
full Polymarket-style trading
```

---

# 6. 从现在到正式上线的完整路径

## TB-8：Minimal payout / FinalizeRewardTx

这是当前 handover 里也已经建议的下一步：TB-8 做 minimal payout / FinalizeRewardTx，并且明确禁止 NodeMarket trading、AMM、multi-solver royalty、DAG-aware payout splits、public-chain anchoring、MetaTape、multi-org、full RSP-4 settlement、P6 expansion。

目标：

```text
accepted proof -> system-emitted FinalizeRewardTx
escrow -> solver balance
CTF conservation
economic_state replay
```

范围：

```text
single solver
single verifier
no royalty
no ContributionDAG
no NodeMarket
no public chain
```

硬验收：

```text
FinalizeRewardTx system-only
agent submit cannot emit FinalizeRewardTx
payout_sum <= escrow
challenge window satisfied or explicitly simplified
replay reconstructs economic_state
dashboard shows payout
```

这一步是正式上线的关键，因为它把"做题"变成"获得结算"的闭环。

---

## TB-9：Durable AgentRegistry + Wallet Projection

TB-7R 目前的 run-local identity 适合 smoke，不适合正式上线。正式用户支付需要 durable identity。

目标：

```text
persistent AgentRegistry
agent durable public key
wallet read-only projection
EconomicState is canonical
```

硬验收：

```text
WorkTx signature verifies against persistent registry
agent identity survives run restart
WalletTool cannot mutate balances
no f64 balance mutation in RSP path
payout recipient is durable agent ID
```

这一步要防止 `WalletTool` 复活为第二套经济账本。

---

## TB-10：Lean Proof Task Market MVP

目标：

```text
第一个可用的 Lean proof task market。
```

流程：

```text
TaskOpenTx
EscrowLockTx
Agent solve
WorkTx
VerifyTx
Challenge window simplified
FinalizeRewardTx
Dashboard
Replay
```

硬验收：

```text
用户可发布任务
Agent 可提交 proof
Lean 验证结果进入 CAS
accepted / rejected branches 可见
escrow 支付给 solver
run 可 replay
dashboard 可重建
```

这是第一个可以对外展示的 alpha 版本。

---

## TB-11：Markov Log Loom / Information Loom v0

目标：

```text
把 logs archive as ground truth 落地。
```

新增结构：

```text
EvidenceCapsule
RunSummaryCapsule
ErrorCluster
TypicalErrorBroadcast
MarkovOverrideRequest
```

运行规则：

```text
默认 InitAI 只读 latest capsule + constitution。
历史 logs archive 保留但不进入默认 context。
遇到顽疾问题才允许 Markov override。
```

硬验收：

```text
每个 run 生成 EvidenceCapsule
capsule 链接 L4 / L4.E / CAS / dashboard / replay report
raw diagnostic 不进入 Agent read view
public_summary 可广播
typical error 可聚类
Markov override 必须有理由和签名
```

这一步把 Flowchart 3 的 logs archive / feedback / re-init 落成系统，而不是只存在于论文里。

---

## TB-12：Challenge Slash / RSP-3.2

现在才做 slash。

目标：

```text
UpheldDeferred -> slash path
solver stake / verifier bond penalty
challenger reward
```

硬验收：

```text
ChallengeResolveTx system-only
Released returns bond
UpheldDeferred keeps challenge evidence
SlashTx or equivalent accepted state transition
all money movement conserved
no ghost liquidity
```

这一步必须在 TB-8 payout 和 TB-11 evidence capsule 后做，因为 slash 是高风险经济裁决。

---

## TB-13：NodeMarket v0

现在才把 Polymarket 机制落入代码。

目标：

```text
WorkTx.stake -> FirstLongPosition
ChallengeTx.stake -> ShortPosition
VerifyTx.bond remains responsibility bond
PriceIndex v0
```

不做交易。

硬验收：

```text
NodePosition is exposure index, not Coin holding
NodePosition.amount not counted in total_supply
price = statistical signal, not truth
no automatic liquidity
no ghost liquidity
node outcome still resolved by predicates / challenge / system tx
```

这与宪法中的"价格信号只是广播信号，不指挥黑盒具体怎么做"一致。宪法明确把市场价格视为统计信号，不是事实裁判。

---

## TB-14：Beta Release

目标：

```text
有限用户、有限 Lean tasks、真实 payout、真实 replay。
```

硬验收：

```text
10+ independent tasks replayable
all accepted proofs CAS-resolvable
no fake accepted nodes
payout completed on solved tasks
unsolved tasks show L4.E failures
public operator guide
```

这一步可以对外小范围开放。

---

## TB-15：Official v1.0 Launch

目标：

```text
稳定 Lean Proof Task Market。
```

上线标准：

```text
>=100 tasks replayable
all accepted proofs CAS-resolvable
no ghost liquidity
no agent-submitted system tx
every externalized proposal represented in L4/L4.E
dashboard regeneratable
durable agent identity
minimal payout stable
audit PASS
```

正式上线时不要求：

```text
public chain
AMM
full NodeMarket
MetaTape
multi-org
```

这些是 v1.1+ / v2。

---

# 7. 这份更细宪法对当前开发流程的升级

## 7.1 Flowchart 应成为 TRACE_MATRIX 的一级对象

无损宪法给三张 flowchart 加了 SHA256。

建议新增：

```text
TRACE_FLOWCHART_MATRIX.md
```

每个 TB 必须标注：

```text
Flowchart 1 runtime loop
Flowchart 2 boot
Flowchart 3 meta architecture
```

例如：

```text
TB-7R -> Flowchart 1
TB-8  -> Flowchart 1 + Flowchart 2
TB-11 -> Flowchart 3
```

这比单独引用"Art. I / Art. III"更直观。

---

## 7.2 每个 TB 的 evidence 必须分层

按照无损宪法的"四层保真"原则：

```text
主文层
注释层
原文归档层
视觉校验层
```

我们的 TB evidence 也应分为：

```text
Run artifact
ChainTape / CAS
Dashboard / materialized view
Audit report
Visual / human-readable replay
```

这样防止"dashboard 截图"被误当作 source of truth。

---

## 7.3 Markov 规则应进入 handover 机制

现在 handover 太容易越积越长。新宪法明确指出，InitAI 不能每次从大爆炸日志开始读，否则会被上下文窗口瓶颈淹没；默认只应读上一轮最终错误日志和当前宪法。

所以应新增：

```text
LATEST_CAPSULE.md
```

每个 session 先读：

```text
constitution
LATEST_CAPSULE
TB_LOG latest rows
current charter
```

而不是全量 notepad。

---

# 8. 我会如何以"正式上线"为目标独立推进

如果由我独立推进，我会砍掉所有非上线前置项，保留最小闭环。

我不会马上做：

```text
NodeMarket trading
AMM
MetaTape
public chain
multi-org
P6 expansion
h_vppu polish
royalty
per-tactic DAG
```

我会只做：

```text
ChainTape authoritative proposal path
Lean proof task market
escrow
stake
VerifyTx
FinalizeRewardTx
dashboard
replay
durable identity
EvidenceCapsule
```

原因很简单：正式上线的第一个用户不需要看到完整 AGI 经济体，他需要看到：

```text
我发了任务；
Agent 解了；
系统验证了；
我能审计过程；
钱按规则发了。
```

这就是上线 MVP。

---

# 9. 给 AI coder 的直接执行口令

可以直接发：

```text
Architect ruling after reading lossless constitution:

1. The new constitution gives stronger implementation guidance. The three flowcharts must become explicit TB validation axes.
2. TB-7R is accepted as shipped. P2 Frame B is closed.
3. Next TB remains TB-8 Minimal payout / FinalizeRewardTx.
4. Do not start NodeMarket, AMM, public chain, MetaTape, multi-org, or P6 expansion before minimal payout.
5. TB-8 must implement system-only FinalizeRewardTx:
   accepted proof -> escrow -> solver balance.
6. FinalizeRewardTx cannot be agent-submitted.
7. Payout must conserve total CTF and be replayable from ChainTape.
8. Dashboard must show payout as materialized view, not source of truth.
9. TB-9 should implement durable AgentRegistry + wallet projection.
10. TB-10 should become Lean Proof Task Market MVP.
11. TB-11 should implement Markov Log Loom / EvidenceCapsule:
    logs archive as ground truth,
    latest capsule as default context,
    Markov override only for persistent disease problems.
12. TB-12 can do challenge slash.
13. TB-13 can do NodeMarket v0:
    WorkTx.stake = FirstLongPosition,
    ChallengeTx.stake = ShortPosition,
    price = statistical signal, not truth,
    no ghost liquidity.
14. Add TRACE_FLOWCHART_MATRIX:
    Flowchart 1 runtime,
    Flowchart 2 boot,
    Flowchart 3 meta.
15. Do not modify constitution.md unless explicitly sudo-authorized.
```

---

# 10. 最终判断

这份更细致的宪法让我们看清一件事：

> **TuringOS 正式上线的关键不是继续增加经济复杂度，而是让三张 flowchart 全部在产品中落地。**

对应关系是：

```text
Flowchart 1:
  TB-7R 已基本落地真实 Agent proposal -> ChainTape。

Flowchart 2:
  TB-8/TB-10 必须落地 Boot -> task -> escrow -> payout。

Flowchart 3:
  TB-11 必须落地 logs archive -> Markov capsule -> future improvement。
```

因此，从现在到正式上线的最短路不是：

```text
NodeMarket -> AMM -> public chain -> MetaTape
```

而是：

```text
FinalizeRewardTx
-> durable identity
-> Lean Proof Task Market MVP
-> EvidenceCapsule / Markov Log Loom
-> basic slash
-> NodeMarket v0
-> beta
-> v1.0
```

这条路最符合宪法，也最接近可上线产品。
