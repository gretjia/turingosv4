# §8 RATIFICATION PACKET — Market Activation Class 3/4 Atoms (M0 / M2 / M3)

> 类型：Section-8 per-atom 架构师批准请求（AGENTS.md §5 / §6 / §14）
> 状态：**SIGNED 2026-05-29 — 架构师(gretjia)逐原子授权 M0 + M2 + M3 全部实现(见 §7)。** 下游门(audit/predicate-green/ship-confirm/PR-only)仍适用。
> 日期：2026-05-29 · 仓库：`/Users/zephryj/work/turingosv4` · 基线 HEAD：`1f00012d`
> 分支：`claude/swebench-agi-benchmark` · 设计来源：`handover/SWEBENCH_MARKET_ACTIVATION_BENCHMARK_v4_2026-05-29.md`
> 涉及 atom：**M0**(N-agent 编排,Class 3 + Class-4 trip-wire)、**M2**(真验证器结算谓词,Class 4)、**M3**(payout/redeem arm,Class 4)

---

## 0. 这份 packet 是什么 / 不是什么

- **是**:动手实现前的 §8-IMPL 授权请求。把每个 atom 的精确范围、所触 §6 受限面、FC 节点/不变量、护栏、Class-4 trip-wire、验收谓词配方摆清,供你逐原子批准。
- **不是**:不是 ship 授权。即便你签了本 packet,每个 atom 仍须走:分支实现 → 真证据 → **clean-context 审计(Class 4 = PRE-ship,AGENTS.md §14)** → predicate-GREEN + FC1 不变量 → 你最终 ship-confirm → orchestrator merge PR。
- **不是**:不授权任何超出下列精确范围的改动。**Class 4 不得藏在 Class 3 伞下**(§6)。范围外的发现 → 停 → 回来重签。

**批准协议(AGENTS.md §5,承重)**:逐原子、显式、**非一词**。`fix`/`go`/`ok`/`continue`/`可以` 不构成 Class 4 签署。用 §6 的精确 trigger 句逐条签。

---

## 1. 三个 atom 共同的护栏(所有签署一律适用)

1. **整数货币**:money/market 路径只用整数(MicroCoin=i64),严禁 f64/f32(CLAUDE.md §4)。
2. **守恒**:任何 money 流转必须保 `total_supply_micro` 守恒;以 `src/economy/monetary_invariant.rs` 为守恒检查源。
3. **Tape canonicity(Art. 0.2)**:price/wallet/routing/cost/verdict/payout 全部从 tape/CAS 可重建;报告仅派生;严禁 dashboard/sidecar 作真相源(SECOND-SOURCE-DRIFT)。
4. **屏蔽(Art. III)**:hidden tests / gold_patch / test_patch / FAIL_TO_PASS·PASS_TO_PASS 名称 / raw judge stderr / autopsy 永不进任何 agent prompt 或市场搜索;真验证器只在 sealed settlement 跑、其结果不回流搜索。
5. **additive-only on wire**:严禁修改既有 typed_tx 判别式(discriminant)、既有 acceptance 谓词、既有 sequencer admission 语义;只允许**新增**。任何对既有判别式/admission 的改动 = 触发 trip-wire,停并重签。
6. **apply 串行不变**:sequencer 的 `apply_one` 保持串行(bus.rs:169 "never concurrently");并发只在 agent/LLM 层,不在 sequencer apply 层。
7. **FC1 不变量**:`evaluator_reported_completed_llm_calls = step + parse_fail + llm_err`(LHS 不得用 tx_count);失败即 HALT(CLAUDE.md §4)。
8. **PR-only**:分支实现,禁 merge main;不提交 `handover/evidence/**` sidecar;新模块/脚本进 liveness、干净 worktree 验证。
9. **真跑无虚假确定性**:flake 绝不当 resolved;不伪造 genesis report;不回溯改写 evidence。

---

## 2. M0 — N-agent 并发编排器

| 字段 | 内容 |
|---|---|
| **范围(scope)** | 新增一个编排层,并发驱动 N 个**角色分化**的 LLM agent,各自用**自己的 Ed25519 keypair**(AgentKeypairRegistry)对**共享 Sequencer** 提交 `WorkTx`(新节点,带 `parent_tx` 引用)/ `BuyWithCoinRouterTx`(invest,Bull=BuyYes / Bear=BuyNo)/ `VerifyTx`,经**既有** `submit_agent_tx` API。每 agent 读价格广播(`UniverseSnapshot.price_index`)+ 有限上下文,按价格选 parent(复用 `boltzmann_select_parent_v2`)。 |
| **Risk class** | **Class 3**(多 agent 生产接线,经 sequencer 队列触及 market/economic 状态)。 |
| **§6 受限面触及** | `src/state/sequencer.rs`:**仅调用** `submit_agent_tx` / 读异步队列结果;**不改** admission 规则、不改 `apply_one` 顺序/判别式。`src/bus.rs`:遵守串行 apply 不变量,不改。新编排代码应为**新增模块**(建议 `src/runtime/` 下),additive。 |
| **FC 节点/不变量** | FC1 runtime loop(N 个 agent 进入 Q_t→Δ→Q_{t+1});FC2 map-reduce tick(并发 agent = "map");**FC1 attempt-count 不变量**(N agent 下计数须正确聚合,见共同护栏 #7);tape canonicity(每 agent 动作落 typed tx 上 tape)。 |
| **精确改动** | (a) 新编排模块:agent 池、并发 LLM 调度、每 agent 独立 keypair/role、价格读取→parent 选择→出 tx;(b) roster 装配调用(与 M4 衔接);(c) tx_budget 计量 + 终止(候选 OMEGA 或预算耗尽)。 |
| **必须不做(guardrails)** | 不并行化 `apply_one`(串行不变);不改 admission/判别式;不引入 sequencer 外的共享可变状态(唯一真相是 canonical QState);不让任一 agent 看到 hidden tests。 |
| **Class-4 trip-wire(命中即停,重签)** | ① 若发现必须改 sequencer admission 规则 / `apply_one` 顺序 / typed_tx 判别式;② 若并发要求改 `bus.rs` 串行语义;③ 若须新增 typed_tx 种类。任一命中 → M0 升 Class 4,停止,回本 packet 加签。 |
| **宪法依据** | Art. II.2(价格广播驱动群体涌现);FC1/FC2。 |
| **验收谓词配方(→ GREEN)** | `[ ] ≥5 agent 各以独立 keypair 提交 ≥1 WorkTx,均经 submit_agent_tx 落 L4` · `[ ] ≥1 次 Bull BuyYes + ≥1 次 Bear BuyNo 经 BuyWithCoinRouterTx 落账` · `[ ] parent_tx 非全 None(出现非线性 DAG,branching_factor>1)` · `[ ] apply_one 仍串行(无并发 apply 证据)` · `[ ] FC1 不变量等式成立` · `[ ] sequencer.rs admission/判别式 git diff = 空` · `[ ] cargo test --workspace + run_constitution_gates.sh exit 0` |

---

## 3. M2 — 真验证器结算 OMEGA(settlement predicate)

| 字段 | 内容 |
|---|---|
| **范围(scope)** | 新增一个 **settlement-bundle 谓词**:在 sealed settlement 时对市场选出的候选 patch 跑**真** `SwebenchTestJudge::verdict`(Docker,swebench_test_judge.rs:159 → run_evaluation → `resolved` :288);把 Docker verdict + 其输入落 CAS(PredicateProofCapsule)。**取代**当前"自我宣称 VerifyTx + 内容静态谓词"作为 OMEGA 的结算权威。 |
| **Risk class** | **Class 4**(predicate registry = admission 规则;改变什么 gate settlement)。 |
| **§6 受限面触及** | `src/top_white/predicates/registry.rs`(v8_production :607 — **新增** BootPredicateKind,注册 `required_in = [Settlement]`);`runtime/predicate_registry_loader.rs`;`src/state/sequencer.rs`(`verify_work_predicates` :1225 / settlement 路径 — 接入新谓词的 verify_proof);可能 `src/state/typed_tx.rs`(若需 PredicateProofKind::ReExecute 等**新增**变体)。 |
| **FC 节点/不变量** | FC1 predicate gate(Q_t→predicates→wtool 的 predicates 节点);FC2 halt(OMEGA settlement = halt);Art. 0.2(Docker verdict 必须落 tape/CAS,可重建)。 |
| **精确改动** | (a) 新 `BootPredicateKind::SwebenchDocker`(及/或 Lean 对应)实现 `verify_proof` 调真 harness;(b) registry v8 → v9:**新增**该谓词、`required_in` 含 `Settlement`(**不动**既有 acc1/forbidden/sorry_free/size/lean 五谓词);(c) settlement 路径在 OMEGA-confirm 前调用该谓词;(d) verdict + inputs → CAS PredicateProofCapsule。 |
| **必须不做(guardrails)** | 不改既有 5 个 acceptance 谓词;不改既有 tx 判别式(只新增变体);Docker 是 OMEGA 的**唯一**结算权威,不留自我宣称旁路;谓词须确定性/可复现;hidden tests 只在此 settlement 跑,**绝不**回流搜索。 |
| **Class-4 trip-wire** | 本身即 Class 4。额外标红:若须**新增 typed_tx 判别式**(wire schema 变更),在签署栏显式确认你知悉这是 wire-schema 级改动。 |
| **宪法依据** | Art. 0.2(verdict 可重建);"市场找路、真验证器结算"原则;FC2 halt;杜绝 ζ 老run 自我宣称 OMEGA。 |
| **验收谓词配方(→ GREEN)** | `[ ] sealed settlement 对候选 patch 跑真 Docker,report.json resolved 读取正确` · `[ ] 无自我宣称旁路:OMEGA 仅当 Docker resolved=true` · `[ ] Docker verdict + inputs 落 CAS,可从 tape/CAS root 重建` · `[ ] 既有 5 谓词 git diff = 空(仅新增)` · `[ ] hidden tests 不在任何 search-phase prompt 中(grep 证据)` · `[ ] predicate_registry_root 经 activate_predicate_binding 锚定新 registry` · `[ ] cargo test --workspace + run_constitution_gates.sh + constitution_matrix_drift exit 0` |

---

## 4. M3 — payout / redeem arm

| 字段 | 内容 |
|---|---|
| **范围(scope)** | 补完 `EventResolveTx`(typed_tx.rs:1616)的结算:赎回**赢家** YES/NO 份额 → Coin(从 `conditional_collateral_t` q_state.rs:622);`FinalizeRewardTx` 发奖;`reputations_t` 更新;`claims_t` 结算。当前缺 redeem arm。 |
| **Risk class** | **Class 4**(新增 sequencer settlement arm + money/CAS 流转)。 |
| **§6 受限面触及** | `src/state/sequencer.rs`(**新增** EventResolve 赎回 arm);`src/state/typed_tx.rs`(EventResolveTx 已存在;若赎回需新 tx 种类则**新增**);economic 状态变更(`conditional_collateral_t` / `conditional_share_balances_t` / `claims_t` / `balances_t` / `reputations_t`)。 |
| **FC 节点/不变量** | FC1 wtool(Q_t→Q_{t+1} 含 money 流转);**money 守恒**(`total_supply_micro`,monetary_invariant.rs);Art. 0.2(payout 可重建)。 |
| **精确改动** | (a) sequencer 新增 EventResolve 处理 arm:对**已被 M2 真结算**的 event,赢家 side 份额按 collateral 赎回 Coin,输家 side 份额作废;(b) FinalizeReward debit/credit escrow→claims→balances;(c) reputation 增减;(d) 全部落 L4 + CAS。 |
| **必须不做(guardrails)** | 严格币守恒(赎回额 = 释放的 conditional_collateral,无 mint/burn 失衡);整数;**只对 M2 真结算的 event 赎回**(不可对自我宣称结算赎回);additive(不改既有 escrow/stake/claim arm);不改既有判别式。 |
| **Class-4 trip-wire** | 本身即 Class 4。若赎回需**新 typed_tx 判别式**,在签署栏显式确认 wire-schema 改动。 |
| **宪法依据** | money 守恒;Art. 0.2;整数货币;FC1 wtool。 |
| **验收谓词配方(→ GREEN)** | `[ ] 赎回前后 total_supply_micro 守恒(monetary_invariant 通过)` · `[ ] 仅 M2 真结算的 event 可赎回(无自我宣称旁路)` · `[ ] 赢家 side 得 Coin、输家 side 份额作废,账本自洽` · `[ ] 全程整数,无 f64` · `[ ] payout 落 L4+CAS,可重建` · `[ ] 既有 escrow/stake/claim arm + 既有判别式 git diff = 空(仅新增)` · `[ ] cargo test --workspace + run_constitution_gates.sh + constitution_matrix_drift exit 0` |

---

## 5. 不在本 packet 内(范围边界)

- **M1**(角色装配进运行循环):若仅填 `agent_role_assignment` + 复用既有 real5_roles 路由(real5_roles.rs:619)= Class 2,**无需 §8**。若发现须改基于角色的 admission = Class 3,届时单独补签。
- **M4/M5/M6/M7**(roster 扩 N / SWE-bench 适配+屏蔽 / 指标+报告 / diff materializer)= Class 1-2,无需 §8,走常规 clean-context 审计(Class 2)。
- 任何对 `kernel.rs` / `state/sequencer.rs` admission / `typed_tx.rs` 既有判别式 / canonical signing payload / `bottom_white/cas/schema.rs` / RootBox 的改动,**均不在本 packet 授权内**,须新 §8。

---

## 6. 签署后到 ship 的下游门(每个 atom)

```
本 §8 签署(授权实现)
→ 分支实现(干净 worktree,新模块/脚本先注册 liveness)
→ 真证据(G0/G1 真跑,tape/CAS 落地)
→ clean-context 审计(Class 4 = PRE-ship;无实现 transcript;输出 {NO-VIOLATION | VIOLATION-FOUND | RECONSTRUCTION-FAILURE | SECOND-SOURCE-DRIFT};AGENTS.md §9)
→ predicate-GREEN(本 atom 验收配方全绿)+ FC1 不变量成立
→ 你最终 ship-confirm
→ orchestrator merge PR(GIT_HARDEN_ALLOW_MAIN=1)
```

clean-context 审计 packet 只给:任务 brief + risk class + 触及 FC 节点 + diff/commit + 源/文档 + evidence 路径 + 精确验证命令输出 + 裁决格式。**不给实现 transcript。**

---

## 7. 架构师批准栏(逐原子;请用对应 trigger 句逐条签;非一词)

> 签法:在对应行写明授权 + 你的 trigger 句。留空 = 未授权 = 不得实现该 atom。
> 你可只签其中一部分(例如先签 M0,等市场跑起来确认 settlement 接口形状后再签 M2/M3)。

```
[x] M0(Class 3 + Class-4 trip-wire)
    架构师签署：gretjia · 2026-05-29
    trigger 句：「我授权 M0 按本 §8 范围实现(Class 3,含 trip-wire,命中即停重签)」
    备注/约束：trip-wire 命中(须改 admission/判别式/串行语义/新 tx 种类)即停,回本 packet 重签。

[x] M2(Class 4：新增 settlement predicate + 改 predicate registry;含 wire-schema 知悉)
    架构师签署：gretjia · 2026-05-29
    trigger 句：「我授权 M2 §8,知悉这是 predicate registry / settlement admission 级 Class 4 改动」
    备注/约束：仅新增 settlement 谓词,不动既有 5 acceptance 谓词;Docker 为 OMEGA 唯一结算权威。

[x] M3(Class 4：新增 payout/redeem sequencer arm;money 守恒;含 wire-schema 知悉)
    架构师签署：gretjia · 2026-05-29
    trigger 句：「我授权 M3 §8,知悉这是新增 sequencer 结算 arm 的 Class 4 money 改动」
    备注/约束：币守恒(monetary_invariant)硬门;仅对 M2 真结算的 event 赎回。
```

> 撤销/缩限:你可随时以 `撤销 M<n> §8` / `M<n> 改用…` 缩限或撤回任一授权。
