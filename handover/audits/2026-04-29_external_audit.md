# External Audit — 2026-04-29

**Source**: external auditor / user-provided (delivered via chat).
**Date received**: 2026-04-29 session-3, post `d29218e` (TB-1 re-charter).
**Audit method (per auditor)**: read-only GitHub connector sampled audit + cross-reference with whitepaper / constitution / manifest materials. NOT a full code-security audit, NOT a CI run, NOT a `cargo test` execution. Path-level + roadmap-coherence audit only.
**Author tooling note (per auditor)**: "当前工具暴露了 GitHub 读取/搜索/PR/CI 查询能力，但没有 create/update/commit 文件接口。因此本报告写入本地 `/mnt/data`，未直接修改 GitHub 仓库。"
**Repo state at audit time**: `main @ d29218e` (5-commit audit-ingestion wave from architect directive 2026-04-29).
**Sampled paths** (per auditor § 1.1): `CLAUDE.md`, `constitution.md`, `handover/architect-insights/ROADMAP_9_PHASE_2026-04-29.md`, `handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md`, `handover/tracer_bullets/TB_LOG.tsv`, `handover/tracer_bullets/TB-1_recharter_2026-04-29.md`, `genesis_payload.toml`, `docs/economics.md`, `src/sdk/tools/wallet.rs`, `src/kernel.rs`, `src/ledger.rs`, `src/state/q_state.rs`, `src/state/typed_tx.rs`, `src/bottom_white/ledger/transition_ledger.rs`, `src/state/sequencer.rs`.
**Verbatim**: yes. Body below is the auditor's report unedited except for the metadata header above.

---

# TuringOS v4 路线图与当前实现审计报告

**审计对象**：`gretjia/turingosv4` 当前 `main` 分支及用户提供的 External Auditor Pack。
**审计日期**：2026-04-29。
**审计方式**：只读 GitHub 连接器抽样审计 + 用户上传 manifest/宪法/白皮书材料交叉比对。
**写入状态**：当前工具暴露了 GitHub 读取/搜索/PR/CI 查询能力，但没有 create/update/commit 文件接口。因此本报告写入本地 `/mnt/data`，未直接修改 GitHub 仓库。

---

## 0. 一句话结论

你的项目路线图治理层已经明显优于普通 research repo：你已经有 canonical P0–P9 roadmap、TB 方法论、phase/exit/kill 绑定、Trust Root、TRACE\_MATRIX 和双外审纪律。真正的风险不在“有没有路线图”，而在以下三点：

1. **P6 MiniF2F / PPUT 研究线推进得比 P1/P3 基础设施线更快**，虽然已经被标记为 product-line anchor，但仍容易在执行中重新压过 GitTape/RSP 主轴。
2. **P1 Transition Ledger 的“accepted-only logical\_t”设计与路线图中“rejected tx 也 append”的验收描述存在语义冲突**。这必须拆成 accepted transition ledger 与 rejected evidence ledger 两条账本，否则会在实现时互相打架。
3. **P3 经济层仍有历史文档与旧代码语义漂移**：`docs/economics.md` 中的 Law 3、每个新节点自动注入 YES/NO 做市、ghost liquidity 等表述，和当前宪法的 `on_init` 唯一铸币点、RSP escrow-first 奖金结算方向存在冲突。

我的建议：**不要再扩大 TB-1 的目标；立刻把未来 3 个 TB 收缩到 P1 kill criteria 与 P3 RSP-0/RSP-1。** 只要这三件事跑通，TuringOS 就从“研究系统”变成真正的“反奥利奥经济操作系统内核”。

---

## 1. 审计范围与依据

### 1.1 已读取/抽样文件

主要依据：

- `CLAUDE.md`
- `constitution.md`
- `handover/architect-insights/ROADMAP_9_PHASE_2026-04-29.md`
- `handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md`
- `handover/tracer_bullets/TB_LOG.tsv`
- `handover/tracer_bullets/TB-1_recharter_2026-04-29.md`
- `genesis_payload.toml`
- `docs/economics.md`
- `src/sdk/tools/wallet.rs`
- `src/kernel.rs`
- `src/ledger.rs`
- `src/state/q_state.rs`
- `src/state/typed_tx.rs`
- `src/bottom_white/ledger/transition_ledger.rs`
- `src/state/sequencer.rs`

### 1.2 未完成事项

本次没有：

- clone 完整仓库；
- 运行 `cargo test`；
- 运行 CI；
- 直接修改 GitHub；
- 穷尽审计所有 cases/rules/tests。

因此，本报告是**路线图与架构一致性审计**，不是完整代码安全审计。

---

## 2. 当前总体评分

| 维度                  | 评分     | 判断                                                                                                                       |
| ------------------- | -----: | ------------------------------------------------------------------------------------------------------------------------ |
| 宪法根与治理纪律            | 8.5/10 | Trust Root、R-018、TRACE\_MATRIX、双外审纪律很强；但 `creator_signature` 与 self boot attestation 仍是 pending 语义。                      |
| 路线图完整性              | 9/10   | P0–P9 + 6-field contract + kill criteria + 90-day plan 已经非常清晰。                                                           |
| P1 GitTape Kernel   | 5.5/10 | CAS、TypedTx、QState、Transition Ledger skeleton 很强；但 `dispatch_transition` 仍为 NotYetImplemented，accepted/rejected 账本语义需拆分。 |
| P2 Agent Runtime    | 4/10   | MiniF2F swarm 有实验基础，但 Solver/Verifier/Challenger/Planner 的运行时角色还没有作为 TuringOS runtime 闭环落地。                              |
| P3 RSP Economy      | 3.5/10 | Wallet 与 MicroCoin/TypedTx 已有基础，但 escrow、YES/NO stake、challenge window、Contribution DAG、settlement engine 仍未真正闭环。        |
| P4 Information Loom | 3.5/10 | prompt\_guard/fc\_trace/librarian 有局部基础；error clusterer、read-view shielding、price/risk index 未成系统。                       |
| P5 MetaTape         | 1/10   | 文档设计强，但实现未启动；正确地被后置。                                                                                                     |
| P6 Epistemic Lab    | 7/10   | MiniF2F/Lean oracle/PPUT 实验线非常成熟，但必须保持“product-line anchor”身份，不能替代 P1/P3 基础设施。                                           |
| 区块链/ChainTape 现代化   | 5/10   | Git2 transition ledger 与 CAS 方向正确；permissioned/public settlement 还只是远期设计。                                                |

---

## 3. 最重要的正向发现

### F1. Canonical Roadmap 已经落地，不只是口头路线图

`ROADMAP_9_PHASE_2026-04-29.md` 已经把路线图写成可执行治理机制：每个 phase 都必须有 Goal、Build、Transactions、Predicates、Exit Criteria、Forbidden 六字段；phase 只有在每个 exit criterion 都有测试或证据时才 green；任一 kill criterion 被证明成立就进入 DEAD 并停止。这是非常正确的。

这比普通“项目路线图”强很多，因为它不是按功能列表推进，而是按**宪法不变量的可证伪验收**推进。

### F2. TB 方法论已经接入路线图

`AUTO_RESEARCH_NOTEPAD.md` 已经规定每个 TB 必须声明：

```text
phase_id
roadmap_exit_criteria_addressed
kill_criteria_tested
capability surface
ship surface
```

并且规定优先选择最低编号、仍有 RED kill criterion 或未覆盖 exit criterion 的 phase。这一点非常重要，因为它能防止项目继续被 P6 实验指标牵引而越过 P1/P3。

### F3. TB-1 Re-charter 方向正确

原 TB-1 把 P1 ledger、P3 economy、P5 capability compilation、P6 metric 打包进 7 天，是过度耦合。新的 re-charter 已经把 AT-5（winning tactic 注入 prompt context）移出 TB-1，归入未来 P5 MetaTape。这是正确修正。

### F4. QState / TypedTx / TransitionLedger 已经有很强的 ABI 基础

`QState` 已经是 9 字段结构，且包含 `EconomicState` 9 个子字段；`TypedTx` 已经有 Work/Verify/Challenge/Reuse/FinalizeReward/TaskExpire/TerminalSummary；`transition_ledger.rs` 已经有 CAS、canonical encoding、ledger\_root fold、Git2LedgerWriter、replay\_chain\_integrity 与 replay\_full\_transition 的结构。这说明系统不是停留在白皮书，而是真的在构造可重放状态机。

### F5. Trust Root 与 audit discipline 优于当前阶段需求

`genesis_payload.toml` 的 trust\_root manifest 和 boot verification 机制，是 P0/P1 走向严肃工程的关键基础。虽然还有 pending 字段，但整体方向正确：把 load-bearing files 哈希锁定，启动时 fail-closed。

---

## 4. Critical Findings：需要立即修正的路线图/实现风险

## CF-1：Accepted Transition Ledger 与 Rejected Evidence Ledger 混淆

### 现象

路线图 P1 Exit 写道：

```text
rejected tx 的原始错误不进入其他 Agent 的 read view
```

TB-1 Day 3 又写得更具体：

```text
simulate a tx that fails predicate;
assert state_root unchanged;
assert ledger entry IS appended (with status=rejected) but tx is not applied.
```

但当前 `src/state/sequencer.rs` 与 `src/bottom_white/ledger/transition_ledger.rs` 的设计是：

```text
logical_t only assigned post-accept
rejected submissions never get logical_t
apply_one 在 dispatch_transition rejected 时 early return
run() 只 log debug and continue
```

这两者冲突。

### 为什么严重

这不是小命名问题，而是 ChainTape 的语义边界问题：

- 如果 rejected tx 进入同一个 `L4 Transition Ledger` 并占用 `logical_t`，则“每个 logical\_t 是 accepted transition”的 replay 语义被破坏。
- 如果 rejected tx 完全不记录，则 P1/P4 的错误屏蔽、挑战、reputation、error clustering 没有证据来源。

### 宪法正确解

拆成两条账本：

```text
L4 Transition Ledger
  只记录 accepted transition
  有 logical_t
  推进 ledger_root_t / state_root_t
  可用于 replay_full_transition

L4.E Rejection Evidence Ledger
  记录 rejected submission evidence
  有 submit_id，不占 logical_t
  hash-chained / CAS-backed
  不推进 state_root_t
  raw diagnostic 默认 private / shielded
  可供 ErrorClusterer、ChallengeCourt、ReputationIndex 使用
```

### 立即修改建议

新增文件：

```text
src/bottom_white/ledger/rejection_evidence.rs
```

建议结构：

```rust
pub struct RejectedSubmissionRecord {
    pub submit_id: u64,
    pub parent_state_root: Hash,
    pub agent_id: AgentId,
    pub tx_kind: TxKind,
    pub tx_payload_cid: Cid,
    pub rejection_class: RejectionClass,
    pub raw_diagnostic_cid: Option<Cid>,
    pub public_summary: Option<String>,
    pub timestamp_logical_submit: u64,
    pub prev_rejection_hash: Hash,
    pub rejection_hash: Hash,
}
```

修改 `Sequencer::run()`：

```rust
while let Some(tx) = queue_rx.recv().await {
    let submit_id = ... // already assigned by submit receipt or carried in envelope
    match self.apply_one(tx.clone()) {
        Ok(entry) => { /* accepted transition ledger */ }
        Err(ApplyError::Transition(inner)) => {
            rejection_writer.append_rejected(submit_id, tx, inner, q_snapshot.state_root_t)?;
            // MUST NOT mutate q.state_root_t or q.ledger_root_t
        }
        Err(e) => { /* infra failure: separate class; may halt or evidence-log depending policy */ }
    }
}
```

新增测试：

```text
test_p1_rejected_submission_goes_to_evidence_log_not_transition_ledger
test_p1_transition_ledger_logical_t_accepted_only
test_p1_rejected_tx_no_state_root_advance
test_p1_rejected_raw_log_hidden_from_other_agent_view
test_p1_rejection_evidence_hash_chain_breaks_on_row_deletion
```

### 推荐路线图文本修订

把 P1 Exit 6/9 改成：

```text
6. tx 被拒绝后，state_root 不变；accepted transition ledger 不增加 logical_t；rejection evidence ledger 增加 submit_id 记录。
9. rejected evidence 的 raw diagnostic 不进入其他 Agent read view；只允许 public_summary 或 aggregate counter 出现在 materialized view。
```

---

## CF-2：P3 经济文档与当前宪法/RSP 存在漂移

### 现象

`docs/economics.md` 仍然写着：

```text
Law 3: Digital Property Rights
每个新节点系统自动注入 1000 YES + 1000 NO 做市
APMM Mint-and-Swap router
```

同时 `src/kernel.rs` 中仍存在 run-level bounty market、ghost-liquidity pool、bounty\_lp\_seed 等遗留语义。

### 为什么严重

当前宪法基本法只有：

```text
Law 1: Information is Free
Law 2: Only Investment Costs Money — 1 Coin = 1 YES + 1 NO; on_init 是唯一合法铸币点
```

如果“每个新节点系统自动注入 YES/NO 做市”仍被当作经济文档里的合法机制，会直接破坏 P3：

- 自动注入看起来像 post-init mint；
- ghost liquidity 无法解释资金来源；
- bounty market 与 task escrow 容易混同；
- Agent Economy 会从 RSP 问责协议退化成旧式 prediction market。

### 立即修改建议

把 `docs/economics.md` 重写为 RSP 版本。建议最小替换：

```markdown
# TuringOS v4 Economic Engine — RSP-0/RSP-1

## Constitutional Laws

### Law 1: Information is Free
rtool, search, read-view construction, local thinking, private draft, and sandbox simulation do not charge core Coin. Physical compute may be quota-limited, but read/thinking cannot debit `economic_state_t.balances_t`.

### Law 2: Only Investment Costs Money
Any action that changes or attempts to change global state must lock stake or escrow first:
- task_open_tx requires escrow_lock_tx;
- work_tx requires YES stake;
- verify_tx requires verifier bond;
- challenge_tx requires NO stake;
- settlement_tx requires escrow_sufficient + payout_sum_valid.

### CTF Conservation
For event E:
1 locked Coin = 1 YES_E + 1 NO_E.
YES/NO are event-bound claims, not new money.

### Unique Mint
on_init is the only legal base Coin injection. Post-init mint is invalid.
```

并删除或降级：

```text
Law 3
per-node automatic injection
ghost liquidity
APMM mint-and-swap as a current invariant
```

如果仍需要 price oracle / liquidity，则改成：

```text
Liquidity must be funded from an explicit treasury or sponsor escrow account. Any market-making transfer must pass monetary_invariant and be visible in economic_state_t.
```

新增测试：

```text
test_no_per_node_auto_injection_after_on_init
test_market_liquidity_requires_treasury_debit
test_ghost_liquidity_forbidden_or_accounted
test_yes_no_split_requires_locked_coin
```

---

## CF-3：WalletTool 仍是 f64 legacy economy，RSP 应迁移到 MicroCoin + EconomicState

### 现象

`src/sdk/tools/wallet.rs` 使用：

```rust
HashMap<String, f64>
pub type Portfolio = HashMap<String, (f64, f64, f64)>;
```

它已经有 `genesis_done`、no double genesis、negative deduct rejected 等测试，这很好。但新 `QState::EconomicState` 使用的是 `MicroCoin`、`EscrowsIndex`、`StakesIndex`、`ClaimsIndex` 等 typed structures。

### 为什么严重

P3 RSP 不能建立在 f64 上：

- f64 不适合货币守恒；
- old wallet 与 new economic\_state\_t 会形成双账本；
- `credit(pub(crate))` 虽然限制外部调用，但仍可能绕过 settlement engine；
- portfolio YES/NO/LP 与 event-bound YES\_E/NO\_E 语义不同。

### 推荐策略

不要立即删除 WalletTool。把它降级为 legacy adapter：

```text
WalletTool = legacy query / compatibility only
EconomicState + SettlementEngine = canonical source of truth
```

新增：

```text
src/economy/monetary_invariant.rs
src/economy/escrow_vault.rs
src/economy/stake_manager.rs
src/economy/settlement_engine.rs
src/economy/ctf.rs
```

所有 P3 新逻辑使用：

```rust
MicroCoin
StakeMicroCoin
EconomicState
BalancesIndex
EscrowsIndex
StakesIndex
ClaimsIndex
```

新增 invariant：

```rust
pub fn total_coin(e: &EconomicState) -> MicroCoin {
    balances + escrows + stakes + pending_claims + settlement_locks
}
```

测试：

```text
test_wallettool_credit_cannot_change_canonical_economic_state
test_canonical_total_coin_ignores_legacy_wallet_float
test_all_rsp_transitions_use_microcoin_not_f64
```

---

## CF-4：`dispatch_transition` 仍为 NotYetImplemented，P1 还不是闭环

### 现象

`src/state/sequencer.rs` 中 `dispatch_transition` 对每个 TypedTx 都返回 `TransitionError::NotYetImplemented`。

这意味着当前已实现的是：

```text
submission queue
CAS put/sign/root fold skeleton
TransitionLedger schema
replay skeleton
```

但尚未实现：

```text
WorkTx 如何验证/接受/拒绝
VerifyTx 如何更新验证状态
ChallengeTx 如何打开挑战案
FinalizeRewardTx 如何释放奖金
```

### 判断

P1 不能标为 green，只能标为 structural partial。路线图中已经承认 P1 partial，这是准确的。

### 推荐最小实现顺序

不要一次实现所有 tx。P1 只实现 WorkTx 的 accept/reject skeleton：

```rust
fn dispatch_work_tx(q, work, predicate_registry, tool_registry) -> Result<(QState, SignalBundle), TransitionError> {
    verify_parent_state_root(q, work.parent_state_root)?;
    verify_stake_present_or_if_p1_allow_stub()?; // P3 gated
    verify_read_set_authorized(q, work.read_set)?;
    verify_write_set_authorized(q, work.write_set)?;
    verify_predicate_results(work.predicate_results)?;

    if all_acceptance_predicates_true {
        let q_next = apply_work_state_delta(q, work)?;
        Ok((q_next, SignalBundle::accepted(...)))
    } else {
        Err(TransitionError::PredicateRejected(...))
    }
}
```

P3 之前可以允许 `stake` stub，但必须明确：

```text
P1-only mode = no payout, no settlement, no challenge payout
P3 mode = stake/escrow required
```

---

## CF-5：TB-1 仍然偏宽，建议拆成 TB-1A/TB-1B/TB-1C 或降低 Day 5 集成要求

### 现象

Re-charter 已经比原 TB-1 好很多，但仍然同时要求：

```text
P1 Exit 5,6,7,8,9
P3 Exit 1,2,5,6,8
P6 h_vppu non-null
original AT-1 n3 solve regression
ledger rows per tx
econ_balance_delta non-zero
```

### 风险

7 天内同时推进 P1 ledger semantics、P3 RSP-0、P6 h\_vppu、MiniF2F solve，很容易导致：

- 为了通过 test 而写 glue code，不建立正确模型；
- P6 evaluator hot path 被迫背负 P1/P3 责任；
- 经济层为了 AT-4 临时改 wallet，而不是接入 canonical EconomicState；
- 再次混淆 product-line evidence 与 infrastructure evidence。

### 推荐处理

如果 TB-1 已经执行中，不一定要推翻。但 Day 5 验收建议降级为：

```text
TB-1 ship = P1 kill tests + P3 RSP-0 conservation tests green
P6 h_vppu = non-blocking metric artifact
MiniF2F solve regression = non-blocking smoke, not phase-gate
```

更理想的拆法：

```text
TB-1A: P1 Rejection Evidence + accepted-only transition ledger
TB-1B: P3 RSP-0 monetary invariant + no_post_init_mint + read_is_free
TB-1C: P1 read-view shielding for rejected evidence
TB-2:  P3 RSP-1 task escrow + work_tx YES stake
```

---

## CF-6：RootBox/P0 还缺最终人类签名仪式语义

### 现象

`genesis_payload.toml [constitution_root]` 中仍有：

```text
creator_signature = "PENDING_USER_PGP_SSH_SIGNATURE_v4_FIRST_ENACTMENT"
boot_attestation_hash = "PENDING_v4_X_SELF_REFERENTIAL_COMPUTE"
```

### 判断

这不阻塞 P1/P3 开发，但应避免对外宣称 P0 完全 finished。更准确：

```text
P0 engineering gate mostly green;
P0 RootBox ceremony pending.
```

### 建议

新增一个明确的 P0 sub-phase：

```text
P0.R RootBox Ratification Ceremony
```

Exit：

```text
1. creator_signature 为真实 human-root signature。
2. boot_attestation_hash 可重算。
3. root_sudo.sig 轮换/撤销流程文档化。
4. 修改 constitution.md 无签名必 fail-closed。
```

---

## 5. 路线图级审计：是否符合我之前设计的 P0–P9？

### 5.1 顺序总体正确

当前 canonical ordering 是：

```text
P0 -> P1 -> P2 -> P3 -> P4 -> P5 -> P6 -> P7 -> P8 -> P9
```

但 roadmap 文档里又把 P3 放在 P2 后，TB 实际则 P1+P3+P6 混合。我的建议略微调整执行优先级：

```text
P0 baseline
P1 minimal kernel
P3 RSP-0/RSP-1 immediately after P1 kill tests
P2 Agent roles after RSP-1
P4 Signal layer after rejected evidence exists
P5 MetaTape after RSP-3
P6 product line may continue, but cannot define infrastructure green
```

原因：P2 Agent Runtime 如果没有 P3 stake/escrow，会变成无成本刷 proposal；P4 Information Loom 如果没有 rejected evidence ledger，会没有真实数据源。

### 5.2 建议更新 phase dependency graph

当前：

```text
P0 -> P1 -> P2 -> P3 -> P4 -> P5 -> P6 -> P7 -> P8
```

建议执行依赖：

```text
P0
 └─ P1 kernel kill tests
     ├─ P3 RSP-0/RSP-1
     │   ├─ P2 Agent Runtime roles
     │   ├─ P4 Information Loom
     │   └─ P3 RSP-2/RSP-3
     │       └─ P5 MetaTape v1
     └─ P6 Epistemic Lab product-line anchor (allowed out-of-order, non-blocking)
```

也就是：**P2/P4/P5 都依赖 P1/P3 的最低闭环。**

---

## 6. 逐 phase 审计与落地建议

## P0 Constitution-to-Code

### 当前状态

强。`genesis_payload.toml` 的 trust\_root manifest、boot verification、R-018 sudo scope、TRACE\_MATRIX discipline 都说明 P0 的工程土壤已经建立。

### 缺口

1. `forbidden_actions.json` 与 `invariant_registry.json` 似乎在 roadmap 中要求，但 manifest 中未见明确实现。
2. RootBox human signature 与 boot attestation 仍 pending。
3. `no_context_cross_contamination` 目前更多是规范/测试目标，而非 runtime predicate。

### 立即任务

```text
P0.1 invariant_registry.json
P0.2 forbidden_actions.json
P0.3 rootbox_signature_ceremony.md
P0.4 no_context_cross_contamination predicate test
```

---

## P1 GitTape Kernel

### 当前状态

结构强、闭环弱。

已具备：

```text
CAS store
QState
TypedTx
TransitionLedger
Git2LedgerWriter
replay_chain_integrity
replay_full_transition skeleton
Sequencer queue/apply skeleton
```

未具备：

```text
dispatch_transition bodies
accepted WorkTx state mutation
rejected evidence logging
state.db reconstruction from ledger as end-to-end test
read-view isolation test
```

### 下一个最小交付

不要做所有 tx。只做：

```text
WorkTx accepted/rejected skeleton
RejectedEvidenceLedger
P1 kill tests
```

### P1 关键文件修改

```text
src/state/sequencer.rs
src/state/typed_tx.rs
src/bottom_white/ledger/transition_ledger.rs
src/bottom_white/ledger/rejection_evidence.rs  # new
src/state/q_state.rs
src/bottom_white/tools/registry.rs
```

### P1 测试清单

```text
test_p1_no_wtool_bypass
test_p1_work_tx_accept_advances_state_root
test_p1_work_tx_reject_keeps_state_root
test_p1_reject_not_in_transition_ledger_logical_t
test_p1_reject_in_rejection_evidence_log
test_p1_rejection_raw_diagnostic_not_in_other_agent_view
test_p1_transition_ledger_replay_chain_integrity
test_p1_state_reconstruct_from_accepted_ledger
```

---

## P2 Agent Runtime

### 当前状态

实验 Agent 与 MiniF2F evaluator 运行成熟，但 TuringOS 角色分离尚不足。

### 风险

Solver-only swarm 会把 P2 伪装成“多 Agent”，但无法证明 Verifier/Challenger/Planner 的经济与安全职责。

### 建议

等 P3 RSP-1 完成后再启动 P2 v0：

```text
Solver submits WorkTx with YES stake
Verifier submits VerifyTx with bond
Challenger submits ChallengeTx with NO stake but no payout yet
Planner submits PlanTx but cannot write state
```

### 测试

```text
test_solver_cannot_verify_own_work_unless_policy_allows
test_verifier_cannot_modify_artifact
test_challenger_cannot_revert_directly
test_solver_read_view_is_isolated_from_other_solver_scratch
test_all_agent_outputs_go_to_cas_before_tx
```

---

## P3 RSP Economy Core

### 当前状态

这是最大缺口。

已有：

```text
WalletTool with genesis_done
MicroCoin type likely exists
QState EconomicState stubs
TypedTx Work/Verify/Challenge/FinalizeReward
```

缺失：

```text
EscrowVault
YES/NO event-bound stake
ChallengeCourt
SettlementEngine
ContributionDAG
deferred impact bonus
reuse royalty accounting
canonical monetary_invariant tests
```

### 立即实现顺序

#### RSP-0

```text
monetary_invariant.rs
on_init unique mint
read_is_free
no_post_init_mint
canonical total_coin(economic_state)
```

#### RSP-1

```text
task_open_tx
escrow_lock_tx
work_tx requires YES stake
no escrow -> no official task market
```

#### RSP-2

```text
verify_tx bond
challenge_tx NO stake
challenge failure slash challenger
challenge success slash solver/bad verifier
```

#### RSP-3

```text
provisional_accept
challenge_window
settlement only after window close
```

#### RSP-4

```text
ContributionDAG
payout_sum <= escrow_pool
agent self-claim ignored
```

### P3 file plan

```text
src/economy/mod.rs
src/economy/money.rs
src/economy/monetary_invariant.rs      # new
src/economy/ctf.rs                     # new
src/economy/escrow_vault.rs            # new
src/economy/stake_manager.rs           # new
src/economy/task_market.rs             # new
src/economy/challenge_court.rs         # new later
src/economy/settlement_engine.rs       # new later
src/economy/contribution_dag.rs        # new later
src/economy/reputation_index.rs        # new later
```

### P3 kill tests

```text
post_init_mint_rejected
agent_stakeless_work_tx_rejected
settlement_over_escrow_rejected
agent_self_claim_contribution_ignored
provisional_accept_not_full_payout
```

---

## P4 Information Loom

### 当前状态

概念成熟，实装不足。

### 前置依赖

必须等 P1 rejected evidence ledger 与 P3 reputation/stake events 存在，否则 P4 没数据。

### 最小版本

```text
ErrorClusterer reads RejectionEvidenceLedger
SignalRouter emits public_summary only
ReadViewCompiler filters raw_diagnostic_cid
PriceIndex reads TaskMarket + YES/NO prices
GoodhartShield masks private predicates
```

### 测试

```text
test_raw_rejection_log_private
test_error_cluster_has_evidence_cid
test_public_broadcast_has_no_private_predicate_id
test_price_signal_cannot_override_predicate_failure
test_agent_view_differs_by_agent_policy
```

---

## P5 MetaTape

### 当前状态

未开始，且正确地未开始。

### 风险

不要让 P6 PPUT-CCL 的 “Capability Compilation” 提前偷跑成 P5。当前 notepad 已经明确 Phase D shadow CCL deferred，正确。

### P5 启动条件

必须满足：

```text
P1 accepted/rejected ledgers green
P3 RSP-3 green
P4 ErrorClusterer + GoodhartShield green
```

### P5 最小版本

```text
ArchitectAI reads error_cluster summaries, not raw private logs by default
ArchitectAI proposes predicate_patch_tx
JudgeAI only PASS/VETO constitution_compliant
meta_sandbox runs patch test suite
canary deploy requires rollback_plan
```

---

## P6 Epistemic Lab

### 当前状态

成熟度最高，但顺序风险最大。

### 正确定位

P6 MiniF2F/Lean oracle/PPUT 是“产品线 anchor evidence”，不是 primary infrastructure。

### 建议

继续保留，但每次 P6 TB 必须写明：

```text
This does NOT make P1/P3 greener.
This is product-line evidence only.
```

不要让 `h_vppu`、PPUT、MiniF2F solve rate 成为 P1/P3 的验收替代品。

---

## P7/P8/P9

当前不用投入代码。保持设计文档即可。

禁止在 P3 之前进入：

```text
public settlement
bridged assets
rollup fraud proof
autonomous market
ArchitectAI self-upgrade
```

---

## 7. 未来 30/60/90 天执行计划

## Days 1–10：P1/P3 骨架杀死标准

目标：把最危险的语义冲突消掉。

### Day 1–3

```text
新增 RejectionEvidenceLedger
修改 TB-1 P1 wording: rejected does not enter accepted transition ledger
新增 accepted-only logical_t test
新增 rejected evidence hash-chain test
```

### Day 4–6

```text
RSP-0 monetary_invariant
MicroCoin total_coin
no_post_init_mint
read_is_free
WalletTool legacy adapter note
```

### Day 7–10

```text
WorkTx minimal dispatch body
P1 accepted/rejected state_root tests
read-view raw rejection shielding test
```

Kill criteria：

```text
如果 rejected tx 仍然进入 accepted logical_t ledger -> STOP
如果 post-init mint 仍可能发生 -> STOP
如果 raw diagnostic 能进入其他 Agent view -> STOP
```

---

## Days 11–30：RSP-1 / RSP-2

目标：Agent 写入必须先有 escrow/stake。

```text
TaskMarketEntry full schema
EscrowVault
StakeManager
YES/NO event-bound claims
work_tx requires YES stake
challenge_tx requires NO stake
verify_tx requires bond
```

测试：

```text
no_escrow_no_task_market
work_tx_without_yes_stake_rejected
challenge_tx_without_no_stake_rejected
escrow_over_payout_rejected
```

---

## Days 31–60：Challenge Window + Settlement

目标：通过谓词不等于全额付款。

```text
ProvisionalAcceptTx
ChallengeCase
ChallengeWindow
ChallengeResolveTx
SettlementEngine
FinalizeRewardTx dispatch body
```

测试：

```text
accepted_work_only_provisional
settlement_before_window_close_rejected
valid_challenge_slashes_solver
invalid_challenge_slashes_challenger
final_payout_sum_le_escrow
```

---

## Days 61–90：Contribution DAG + Information Loom v0

目标：奖金归因与错误屏蔽形成闭环。

```text
ContributionDAG
DAGWeight scorer
Verifier reward
Challenger reward
basic reuse royalty edge
ErrorClusterer
ReadViewCompiler shielding
PriceIndex
```

测试：

```text
agent_self_claim_ignored
reward_explained_by_dag
raw_failure_log_hidden
typical_error_broadcast_has_evidence_not_raw_log
price_signal_does_not_override_predicate
```

---

## 8. 推荐直接提交的文档修订

## 8.1 `docs/economics.md` 建议替换版

```markdown
# TuringOS v4 Economic Engine — Reward Settlement Protocol

## Constitutional Laws

### Law 1: Information is Free
Agents may read authorized views, search, inspect public task signals, and think without debiting core Coin. Physical compute may be quota-limited, but read/thinking cannot mutate `economic_state_t`.

### Law 2: Only Investment Costs Money
Any operation that attempts to change global state or economic state must lock stake or escrow first:
- `task_open_tx` requires escrow;
- `work_tx` requires event-bound YES stake;
- `verify_tx` requires verifier bond;
- `challenge_tx` requires event-bound NO stake;
- `settlement_tx` requires `escrow_sufficient`, `payout_sum_valid`, and `monetary_invariant`.

### CTF Conservation
For a specific event E:

`1 locked Coin = 1 YES_E + 1 NO_E`

YES/NO are event-bound claims over locked Coin. They are not new money.

### Unique Mint
`on_init` is the only legal base Coin injection. Post-init mint is invalid. Any liquidity, bounty, reward, or settlement must come from an explicit balance, escrow, slash pool, or sponsor deposit.

## RSP State Machine

OPEN -> SUBMITTED -> VERIFIED -> PROVISIONAL_ACCEPTED -> CHALLENGE_WINDOW -> FINALIZED -> PAID
fail: SUBMITTED -> REJECTED -> STAKE_SLASHED
challenge-ok: PROVISIONAL_ACCEPTED -> CHALLENGED -> REVERTED/COMPENSATED -> SLASHED -> CHALLENGER_REWARDED

## Forbidden Legacy Semantics

- no per-node automatic YES/NO injection after genesis;
- no ghost liquidity without treasury debit;
- no reward by token count, runtime, or self-reported contribution;
- no full payout before challenge window closes;
- no price signal can override predicate or constitution failure.
```

---

## 8.2 `ROADMAP_9_PHASE_2026-04-29.md` 建议补丁

在 P1 Exit Criteria 中修改：

```diff
- 6. tx 被拒绝后，state_root 不变。
+ 6. tx 被拒绝后，state_root 不变；accepted transition ledger 不增加 logical_t；rejected evidence ledger 增加 submit_id 记录。

- 9. rejected tx 的原始错误不进入其他 Agent 的 read view。
+ 9. rejected evidence 的 raw diagnostic 不进入其他 Agent 的 read view；只允许 aggregate counter / public_summary 出现在 materialized view。
```

在 P1 Build 增加：

```text
/bottom_white/ledger/rejection_evidence.rs
```

在 P3 Forbidden 增加：

```text
per-node automatic liquidity injection / ghost liquidity without explicit treasury debit
```

---

## 8.3 `TB-1_recharter_2026-04-29.md` 建议调整

把 Day 3 的这句：

```text
assert ledger entry IS appended (with status=rejected) but tx is not applied
```

改成：

```text
assert accepted transition ledger does NOT append logical_t;
assert rejection evidence ledger appends a submit_id-scoped rejected record;
assert tx is not applied and state_root remains unchanged.
```

把 Day 5 AT-4：

```text
PputResult.econ_balance_delta non-zero
```

降级为：

```text
P3 RSP-0 evidence: economic_state total_coin invariant holds; escrow/provisional structures exist. Actual econ_balance_delta may remain zero until RSP-1/RSP-3.
```

原因：RSP-0 不应被迫产生真实经济支付；真实余额变化应从 RSP-1/RSP-3 开始。

---

## 9. 推荐代码骨架

## 9.1 `src/economy/monetary_invariant.rs`

```rust
use crate::state::q_state::EconomicState;
use crate::economy::money::MicroCoin;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MonetaryError {
    PostInitMint,
    TotalCoinChanged { before: MicroCoin, after: MicroCoin },
    ReadCharged { fee: MicroCoin },
    PayoutExceedsEscrow { payout: MicroCoin, escrow: MicroCoin },
    StakeRequired,
}

pub fn total_coin(e: &EconomicState) -> MicroCoin {
    // TODO: implement checked addition over all canonical indexes:
    // balances_t + escrows_t + stakes_t + claims_t + pending settlements.
    // Do NOT include legacy WalletTool f64 balances.
    MicroCoin::zero()
}

pub fn assert_total_conserved(before: &EconomicState, after: &EconomicState) -> Result<(), MonetaryError> {
    let b = total_coin(before);
    let a = total_coin(after);
    if b != a {
        return Err(MonetaryError::TotalCoinChanged { before: b, after: a });
    }
    Ok(())
}

pub fn assert_read_is_free(fee: MicroCoin) -> Result<(), MonetaryError> {
    if fee != MicroCoin::zero() {
        return Err(MonetaryError::ReadCharged { fee });
    }
    Ok(())
}
```

## 9.2 `src/bottom_white/ledger/rejection_evidence.rs`

```rust
use serde::{Serialize, Deserialize};
use crate::bottom_white::cas::schema::Cid;
use crate::bottom_white::ledger::transition_ledger::TxKind;
use crate::state::q_state::{AgentId, Hash};
use crate::state::typed_tx::RejectionClass;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RejectedSubmissionRecord {
    pub submit_id: u64,
    pub parent_state_root: Hash,
    pub agent_id: AgentId,
    pub tx_kind: TxKind,
    pub tx_payload_cid: Cid,
    pub rejection_class: RejectionClass,
    pub raw_diagnostic_cid: Option<Cid>,
    pub public_summary: Option<String>,
    pub timestamp_logical_submit: u64,
    pub prev_hash: Hash,
    pub hash: Hash,
}

pub trait RejectionEvidenceWriter {
    fn append_rejected(&mut self, record: RejectedSubmissionRecord) -> Result<Hash, RejectionEvidenceError>;
    fn verify_chain(&self) -> Result<(), RejectionEvidenceError>;
}

#[derive(Debug)]
pub enum RejectionEvidenceError {
    HashMismatch { at: usize },
    ParentMismatch { at: usize },
    Backend(String),
}
```

## 9.3 `src/economy/escrow_vault.rs`

```rust
use crate::economy::money::MicroCoin;
use crate::state::q_state::{AgentId, TxId};
use crate::state::typed_tx::TaskId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EscrowReceipt {
    pub escrow_id: TxId,
    pub task_id: TaskId,
    pub sponsor: AgentId,
    pub amount: MicroCoin,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EscrowError {
    InsufficientBalance,
    OverPayout { payout: MicroCoin, escrow: MicroCoin },
    MissingEscrow,
}

pub fn lock_escrow(
    task_id: TaskId,
    sponsor: AgentId,
    amount: MicroCoin,
) -> Result<EscrowReceipt, EscrowError> {
    // debit sponsor balance, credit escrow index; total_coin conserved.
    todo!()
}

pub fn assert_payout_sum_le_escrow(
    payout_sum: MicroCoin,
    escrow_amount: MicroCoin,
) -> Result<(), EscrowError> {
    if payout_sum > escrow_amount {
        return Err(EscrowError::OverPayout { payout: payout_sum, escrow: escrow_amount });
    }
    Ok(())
}
```

---

## 10. 建议立即创建的 issue / PR 列表

### Issue 1 — P1 accepted-only transition ledger vs rejected evidence ledger

**Severity**: P0 architecture blocker.
**Goal**: Split rejected evidence from L4 accepted transition ledger.
**Files**: `ROADMAP_9_PHASE...`, `TB-1_recharter...`, `src/state/sequencer.rs`, new `rejection_evidence.rs`.
**Exit**: rejected tx no logical\_t; evidence hash chain records submit\_id; raw diagnostic shielded.

### Issue 2 — Replace stale economics doc with RSP-0/RSP-1 semantics

**Severity**: P0 economic drift.
**Goal**: Remove Law3/per-node injection/ghost liquidity or make treasury-accounted.
**Files**: `docs/economics.md`, `src/kernel.rs` comments, `src/economy/*`.
**Exit**: docs and tests enforce on\_init unique mint and explicit escrow.

### Issue 3 — RSP-0 canonical monetary invariant

**Severity**: P3 blocker.
**Goal**: Implement `monetary_invariant.rs` over canonical `EconomicState`.
**Exit**: no post-init mint; read free; total coin conserved.

### Issue 4 — WorkTx minimal transition body

**Severity**: P1 blocker.
**Goal**: Replace NotYetImplemented for WorkTx only.
**Exit**: accepted WorkTx advances state root; rejected WorkTx does not.

### Issue 5 — Read-view shielding test

**Severity**: P4 prerequisite.
**Goal**: prove raw rejected diagnostic cannot leak to another Agent.
**Exit**: materialized view contains aggregate/summary only.

### Issue 6 — WalletTool legacy migration note

**Severity**: P3 consistency.
**Goal**: make canonical economic source explicit.
**Exit**: no new RSP code uses f64 WalletTool balances as source of truth.

---

## 11. Final verdict

你的当前路线图不是“方向错”，而是“治理层已经很强，基础设施闭环还未跟上”。最重要的不是继续写更多愿景，而是把下面三条 kill criteria 做成真实代码：

```text
1. Agent 无法绕过 wtool 修改状态。
2. Rejected tx 不推进 accepted transition ledger，但进入 rejected evidence ledger 且不会污染其他 Agent read view。
3. on_init 之后没有任何路径能增发 Coin；所有奖金必须来自 escrow/stake/slash/treasury 的守恒重分配。
```

只要这三条变成可重复测试，TuringOS 就完成了从“AGI 未来哲学”到“可施工操作系统内核”的第一次跃迁。

我建议你下一步只做一个非常克制的 TB：

```text
TB-Next: P1/P3 boundary cleanup
Primary: P1
Secondary: P3 RSP-0
Goal: accepted-only transition ledger + rejected evidence ledger + monetary invariant skeleton
Forbidden: P6 metric improvement, MetaTape, public chain, new whitepaper prose
```

这会把项目从“研究成果驱动”拉回“宪法不变量驱动”。
