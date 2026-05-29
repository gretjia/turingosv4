# §8 PROPOSAL — Priced multi-node DAG: decouple WorkTx-node from the CTF stake (G0 c4/c5)

> 类型：Class-4 §8 决定 packet（你勾选 A/B/C 之一即解锁;未勾选前不动 §6 受限面）
> 日期：2026-05-30 · 分支 `claude/swebench-agi-benchmark` · 阻塞:G0 条件 c4/c5
> 关联:OBL-010 · charter `handover/tracer_bullets/TB-MARKET-ACTIVATION-G0_charter_2026-05-29.md`

## 1. 现状:G0 已 9/11 真跑证明(无需本提案)

`g0_market_activation_current_kernel` 真跑 + `verify_chaintape` replay 全绿,**9/11 条件由真题测试证明**:c1 genesis+市场 / c2 ≥5 agent / c3 ≥3 角色 / c6 YES+NO 双边 / c7 价格变动 / c8 replay 可重建(`economic_state_reconstructed=true`) / c9 屏蔽 / **c10 sealed 结算 / c11 结算落 tape**。市场环闭合、宪法合规(replay `replay_failure=null`、12 L4、0 L4.E)。

## 2. 为何 c4/c5(多节点 priced DAG)需要 Class-4 —— 经验证据(非分析)

**三次独立真跑一致否决"同一 task 多 WorkTx":**
```
g0smoke2 / g0smoke4 / g0run4:  第二个 WorkTx(同 task)→ rejection_class=InvariantViolation, public_summary="monetary_invariant"
  · 与 escrow 大小无关(10k 与 1M 均触发)
  · 与中间是否插入 ChallengeTx 无关(g0run4 按"WorkTx+ChallengeTx 配方"仍触发)
  · 各 tx 均带当前 state_root(非 StaleParent)
```
> ⚠️ **方法学注**:一个静态分析 workflow 曾断言"多 WorkTx 可自主落地、无需 Class-4",但它**未真跑**;三次真题测试否决了它。**判据是真跑(架构师标准:真题测试 > review/审计)。**

源码定位:WorkTx-accept 臂(`sequencer.rs:1856-2020`)对每个 WorkTx 做 balance→stakes + 写 `node_positions_t`(FirstLong),末尾 `assert_total_ctf_conserved`(`:2016`,经 `assert_no_post_init_mint :1953` / `assert_read_is_free :1955`,均映射 `MonetaryInvariantViolation`)。**同一 event 上第二个 WorkTx 触发其一**。多节点 priced DAG(每节点经 `compute_price_index` 出价)= 需要把"DAG 节点"与"event 上的条件代币/stake 守恒"解耦 = **sequencer admission 重设计 = Class 4**。

**c4/c5 是你愿景的核心**("每节点有市场价格、agent 按价格选节点"),所以值得解锁;但不能用 workaround 绕过守恒不变量(违 "no workaround closures")。

## 3. 三个候选模型(请勾选一个)

| 选项 | 做法 | seam | 取舍 | Class |
|---|---|---|---|---|
| **A. per-node 子事件** | 每个 DAG 节点开自己的 sub-event(`node_survive_event_id` typed_tx.rs:1215 已存在,但 dag_view 用的是 `EventId(task_id)`);WorkTx 绑到 per-node event 而非 task-event | `node_survive_event_id`、`dag_view.rs:259`、WorkTx accept event 绑定 | 每节点独立价格池,最贴合"每节点有价格";改 dag_view + WorkTx event 绑定 | 4 |
| **B. collateral-matched 节点 stake** | 每个 WorkTx 节点配套 mint 等量 collateral(CompleteSetMint),使 `assert_total_ctf_conserved` 成立 | WorkTx accept 增 collateral 配对;`monetary_invariant.rs:213-258` | 改动相对集中;但每节点要预算 collateral,经济语义变重 | 4 |
| **C. 非-CTF 节点原语** | 新增 `ProposeNodeTx`(只记 DAG 边 + Long stake,不碰 event 条件代币);价格来自 stake/challenge 比 | 新 typed_tx 判别式(§6 wire schema)、sequencer admission 臂、price_index node 来源 | 最干净的语义分离;但新增 wire 判别式 = 最大改动面 | 4 |

**我的建议:A**(`node_survive_event_id` 已半成形,最贴合你"每节点价格"愿景,改动面比 C 小)。

## 4. 解锁后我会做(选定即执行,真 cargo gate + 真跑 + replay)
1. 按选项实现多节点 priced DAG(WorkTx/节点 + ChallengeTx Short → `compute_price_index` 出价),G0 binary 跑出 c4(分支>1)+ c5(非最新 parent,boltzmann 价格驱动选择)。
2. 真跑 + `verify_chaintape` replay 验证 → **11/11**。
3. clean-context 审计(Class 4 PRE-ship)。

## 5. 架构师勾选栏(逐选项;非一词,trigger 句)
```
[ ] A — per-node 子事件
    签署：__________  trigger：我授权 A:per-node 子事件 §8(Class 4,改 WorkTx event 绑定 + dag_view)
[ ] B — collateral-matched 节点 stake
    签署：__________  trigger：我授权 B:collateral-matched §8(Class 4)
[ ] C — 非-CTF 节点原语 ProposeNodeTx
    签署：__________  trigger：我授权 C:ProposeNodeTx §8(Class 4,新 typed_tx 判别式)
[ ] 暂不解锁 c4/c5（接受 G0 = 9/11 + 本提案存档）
    签署：__________
```
> 注:本提案是 c4/c5 专属。M0/M2/M3 的旧 §8(2026-05-29)不覆盖这个 DAG-节点解耦(那是新设计面),故需本次单独勾选。
