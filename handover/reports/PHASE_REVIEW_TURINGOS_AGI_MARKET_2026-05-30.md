# TuringOS v4 — 阶段性回顾报告:宪法 agent 市场 benchmark

> 2026-05-30 · 撰写：Claude (实现+评估) · 分支 `claude/swebench-agi-benchmark` · PR #216(MERGED)+ #217
> 证据原则：本报告中每一条"已证明事实"都附**代码 seam（file:line）**或**真跑数据**，可独立复核。所有跑均为真实执行 + `verify_chaintape` replay 验证（非 review / 非模拟）。
> 用途：供架构师研究下一步测试方向。

---

## 0. 一句话摘要

我们把 TuringOS 宪法里"**架在工作 DAG 上的预测市场**"从设计落成可运行的真实系统,并用真 DeepSeek agent 跑出了**首个达到 OMEGA 的 live 多 agent 市场**——证明了机制活着、合规、可复现 ζ。但同时**暴露了一个决定性事实:在弱/宽松验证器下,市场的集体智能优势不显现(PPUT 不随规模升)。TuringOS 的价值是验证器强度的函数;它的真正主场是"强、廉价验证器 + 大搜索空间"的领域**(形式证明、编译器、测试)。它现在不是能力领先者(底层是 deepseek-chat),而是一个**可审计、防篡改、价格路由集体推理的 AGI 底物**——这才是它的差异化。

---

## 1. 已证明的事实(附代码 + 数据证据)

### 1.1 事实:宪法市场机器是真实的、已建好的(非愿景)

经只读核实(probe wgwsczqzd),v4 生产代码里**市场原语已落地**,整数货币、无 f64:

| 原语 | 代码 seam | 证据 |
|---|---|---|
| 经济态(16 整数子字段) | `src/state/q_state.rs:171` `EconomicState` | balances/escrow/stakes/claims/reputations/task_markets/**node_positions_t**/**conditional_collateral_t**/**conditional_share_balances_t**/**cpmm_pools_t**/lp_shares… |
| 真 CPMM AMM(k=yes×no) | `src/state/sequencer.rs:4010` + `cpmm_pools_t` `q_state.rs:749` | sequencer 在每个 CpmmSwap/BuyWithCoinRouter 上维护常积,整数 |
| 每节点价格(纯派生) | `src/state/price_index.rs:164` `compute_price_index` | `price_yes = long/(long+short)`,RationalPrice{num,den} 整数 |
| 价格驱动父节点选择 | `src/sdk/actor.rs:46` `boltzmann_select_parent_v2` | argmax price_yes + ε-greedy,所有未屏蔽历史节点可选 |
| Bull/Bear 角色强制 | `src/runtime/real5_roles.rs:619` `route_role_action` | Bull 限 BuyYes、Bear 限 BuyNo |
| OMEGA 完成判别 | `src/state/typed_tx.rs:243` `RunOutcome::OmegaAccepted` | + VerifyVerdict::Confirm（sequencer.rs:2165 "IS the OMEGA verdict"）|
| 真 git ChainTape(Path B) | `src/git_tape_ledger.rs`（git2 crate,Atom 20-22）| libgit2 commit semantics（v3 是 f64+无 git）|

> **注**：宪法文本 Art.0.4 仍写 "git substrate 未决"（constitution.md:149,"runtime 0 git hits"），但代码已实现 Path B——**这是文本陈旧、代码超前**的真漂移,独立 Class 4 修宪事项。

### 1.2 事实:G0 市场激活 11/11,真跑 + replay 验证

`src/bin/g0_market_activation_current_kernel.rs`,run `g0run5`,确定性 agent(机制证明,非能力):
```
agents=12 roles=4(Bull/Bear/Solver/Challenger)  CPMM YES+NO 交易移动池价
priced 4-node DAG: edges node1→node0, node2→node0(branching=2), node3→node1(非最新parent)
每节点 price_yes 来自真 compute_price_index(WorkTx-Long 1000 + ChallengeTx-Short 500 → 1000/1500)
sealed settlement: emit_system_tx(EventResolve(No)) → 市场 Bankrupt
c1-11 = [1111111T111]  exit 0  0 拒绝
```
`verify_chaintape` replay(宪法合规硬证据):
```
L4=25  L4E=0  ledger_root_verified=true  system_signatures_verified=true
agent_signatures_verified=true  state_reconstructed=true
economic_state_reconstructed=true(Art.0.2 tape canonicity)  replay_failure=null
```
宪法 gate:`run_constitution_gates.sh` → `[k-1-5] total=164 failed=0`;liveness 12/12;matrix_drift 3/3。

### 1.3 事实:G1 — live 多 agent 市场达到 OMEGA(首次 v4 能力评估)

`src/bin/g1_market_live_agent.rs`,真 DeepSeek(`deepseek-chat` via proxy :8123,无脚本)。每 agent:读屏蔽市场视图 → LLM 提议证明步(带 confidence)→ 确定性 ζ 判官 verdict → Pass 则建 per-task 节点(stake=f(confidence)→价格发现)+ ChallengeTx 定价 → OMEGA 结算。

**run g1run2(4 agent × 4 轮)** — 5 个真 LLM 步骤构成连贯 ζ 推导,达 OMEGA,replay 全绿:
```
node Agent_0 conf95% : Define S(N)=Σ_{m≥0} m·exp(-m/N)cos(m/N)
node Agent_1 conf95% : Evaluate analytically S(N)=Re[Σ m·exp(-(1+i)m/N)]
node Agent_2 conf95% : Series: x=exp(-(1+i)/N), Σ m·x^m = x/(1-x)²
node Agent_3 conf95% : Compute real part explicitly
node Agent_0 conf100% ★OMEGA : Thus lim_{N→∞} S(N) = -1/12  [COMPLETE]
```
(对比 v3 ζ:golden path 步骤是"同一句重复14遍"——v4 这条**质量明显更高**。)

**strict 真验证器 run（g2_strict）** — 即使要求证明链经过 def_S → 级数 → 渐近三个真推导里程碑才允许 OMEGA:
```
8 agents  llm_calls=7 pass=6  omega=TRUE  golden_path=3  PPUT=127.5  wall=17.7s
```
→ OMEGA 不是"嘴上说 -1/12",证明真经过了阶段。

**rich DAG run（g1run3,8 agent,continue-past-omega）** — ζ 式涌现树,replay 验证（L4=119, L4E=27, 全签名验, econ 可重建, fail=null）:
```
ROOT (35 nodes, max_branching=21, 6 distinct prices, multiple OMEGA)
└── ○ n0 [Agent_0 p=3810/4310] ✓GP
    ├── ○ n1 → n4 → ● n20 ★OMEGA
    ├── ○ n2
    └── ○ n3 [Agent_4] ✓GP
        ├── ○ n6 [Agent_7] ✓GP ★OMEGA      ← golden path 终点(首个 OMEGA)
        └── ● n7 [Agent_0]                  ← 价格驱动收敛:21 个子节点!
            ├── ● n9  ● n10★  ● n12★  ... (22 descendants)
```

### 1.4 事实:G2 规模曲线 + 成本前沿(PPUT 按架构师定义 = golden-path tokens / wall-time,无完成则 0)

全 live + replay 验证,stop@OMEGA:

| agents | nodes | distinct prices | golden-path tokens | wall(s) | **PPUT** | cost(USD) |
|---|---|---|---|---|---|---|
| 4 | 6 | 5 | 3107 | 25.7 | **121.0** | $0.0009 |
| 8 | 9 | 5 | 3177 | 73.7 | **43.1**† | $0.0022 |
| 16 | 7 | 3 | 2600 | 19.1 | **136.4** | $0.0014 |
| 30 | 6 | 3 | 1819 | 15.8 | **115.1** | $0.0008 |
| 8 STRICT | 6 | 3 | 2262 | 17.7 | **127.5** | $0.0011 |

†N=8 离群:24 次 LLM 调用中 15 次返回坏 JSON → 慢 → 低 PPUT(模型 flakiness,非规模效应)。

**两条硬结论:**
1. **PPUT 大致平(~115-136),不随 agent 数升。** 因为此任务在确定性 ζ 判官下太易,几个好步骤就到 OMEGA;加 agent 只加并行噪声。**ζ 的 90-agent 优势只在 HARD 任务的深度集体搜索才显现。**
2. **成本前沿 < $0.003/OMEGA**(deepseek-chat flash;4k-10k tokens)。极廉的集体推理。

---

## 2. 已暴露的约束/限制(附代码证据)

| 约束 | 代码证据 | 影响 |
|---|---|---|
| **WorkTx 绑定 event YES 条件代币 stake** | `sequencer.rs:1959/1969` + `assert_total_ctf_conserved` `sequencer.rs:2016` | 同一 event 多 WorkTx 触 monetary_invariant → 用 **per-task 节点模型**绕过(每节点自己的 task)。dag_view 按 task_id 过滤,故用 CanonicalNodeGraph(task-agnostic)接边 |
| **challenge at scale 撞 monetary_invariant** | 真跑 g1run3:27 L4.E（~半数 challenge 失败,非致命）| 失败的节点无 Short → price_yes 默认 1.0 → 价格区分受限 |
| **判官宽松度决定 OMEGA 含金量** | `src/judges/math_step_judge.rs` OfflineHeuristicJudge | 宽松 → OMEGA 太易 → PPUT 的 progress=100% 含金量低 |
| **LLM JSON malformation at scale** | g2_n8:15/24 坏 JSON | 多 agent 并发下解析失败累积 → 拖慢 |
| **模型过度自信** | g1run3:confidence 全 85-100% | stake 聚集 → 价格博弈浅(无 ζ 的剧烈 contested) |

---

## 3. 优势分析（我的判断）

1. **可审计、防篡改的底物是真护城河。** 整数货币 CPMM、Ed25519 签名 typed-tx、ChainTape L4、**确定性 replay 全态重建**(每次跑都验证)。这在多 agent AGI 里罕见——大多数 agent 框架是"stdout + 信任"。TuringOS 是"tape-canonical + 可重放审计"。**这是"无虚假确定性"的工程实现。**
2. **价格路由的集体推理是真实存在的机制。** boltzmann 让一个节点吸引 21 个子节点(g1run3)——价格真在路由集体注意力。这是 OpenHands/裸模型/Hermes 都没有的结构。
3. **弱模型 + 市场 → 达 OMEGA 是真的、且极廉。** $0.001/OMEGA。"把便宜模型变成集体"在机制上成立。
4. **宪法/治理层是差异化叙事。** §6 受限面、§8 逐原子批准、clean-context 审计、liveness no-zombie gate——一套"合规即架构"的纪律。这对"可信 AGI"有独特价值。
5. **证明质量比 v3 高**(连贯推导 vs 重复句),在更严底物上。

## 4. 劣势分析（我的判断）

1. **底层能力不是 TuringOS 的(是 deepseek-chat)。** TuringOS 是 scaffold/substrate,不是模型。它的"能力"上限受底层模型限制。它**不是能力领先者**。
2. **集体市场 > 单 agent 这个核心论题,尚未在硬任务上证明。** 当前 PPUT 规模平 → 在易任务上,市场是纯开销。**"市场会深度思考"还是个开放问题。** 这是决定 TuringOS 是"真 AGI 方向"还是"优雅基础设施"的关键。
3. **价格博弈浅。** 模型过自信 + challenge at scale 失败 → 没有 ζ 的剧烈价格发现/whale/做空涌现。市场的"信息聚合"效果未充分体现。
4. **判官是阿喀琉斯之踵。** 宽松判官 → OMEGA 廉价 → 度量失真。严判官 → 可能不可达 → PPUT=0。**整个能力评估悬于验证器质量。**
5. **规模化有工程债**(challenge monetary_invariant、LLM JSON flakiness、>10 agent preseed)。

## 5. 市场位置（诚实,低置信）

- **不能与 OpenHands/Amazon Q 的 SWE-bench 编码分数同台比**:不同任务(数学 vs 编码)、不同模型、宽松判官。我的 G1 是"$0.001 的 ζ 证明市场",不是 leaderboard 坐标。
- **TuringOS 不是"又一个更强的 coding scaffold"**(那条路它输给 model×scaffold 强的玩家)。它是**一个新的坐标轴:可审计的、价格路由的集体推理底物**。
- 真正的对标对象不是 OpenHands,而是:**多 agent 编排框架(AutoGen/CrewAR/Swarm)在"可审计性 + 价格机制"上的缺失**,以及**形式化验证 + LLM(如 AlphaProof/Lean-agent)在"市场化集体搜索"上的缺失**。TuringOS 站在这两者的交叉点,且暂无直接竞品。
- **结论**:TuringOS 当前是"**机制独特、能力待证**"。差异化清晰(宪法市场 + 可审计),但"集体智能涌现"的杀手级证据还没拿到。

## 6. 可能的 AGI 主攻方向（我的建议,按可辩护性排序）

**方向 A(最可辩护)——强验证器领域的可审计集体推理。**
TuringOS 的价值 = f(验证器强度)。它的**自然主场是"强、廉价、确定性验证器 + 大搜索空间 + 可分叉工作"**:
- 形式化数学(Lean kernel:每步机器验证,搜索空间巨大,可分叉证明树)——这正是 ζ 想做的,但要换真 Lean 判官。
- 编译/类型可判定任务、定理证明、程序合成(编译器/测试做验证器)。
在这些域,市场的价格路由集体搜索 + 严格结算应该真正发力(多 agent 帮助探索 + 严判官保证质量)。**这是把"市场会思考"从未证变已证的路径。**

**方向 B——可审计 AGI 底物(治理/信任层)。**
不主打"最强能力",而主打"**最可信、可重放、无虚假确定性的多 agent AGI OS**"。在 AGI 安全/治理叙事下,TuringOS 的 tape-canonical + 宪法 + clean-context 审计是独特资产。定位为"AGI 的可审计操作系统",而非"最强 scaffold"。

**方向 C——成本前沿的弱模型放大。**
$0.001/OMEGA 的集体推理。若能在硬任务上证明"市场化集体 flash > 单发 thinking 模型(同成本)",这是一个真实的成本/能力前沿卖点。需要方向 A 的硬任务证据支撑。

> 我的倾向:**A 优先**(它同时验证核心论题"集体市场 > 单 agent",并天然产生 B 的可审计性 + C 的成本优势)。B/C 是 A 成功后的叙事/商业化延伸。

## 7. 下一步测试方向建议（供你研究决策）

按"先回答核心科学问题"排序:

1. **【核心实验】换强验证器 + 硬任务,测"集体市场是否真 > 单 agent"。**
   - 数学:接 Lean kernel 做判官(每步真验证),用一个**真正难的定理**(单 agent 难解),跑 market(N agent)vs 单 agent(同 token 预算),比 resolve 率 + PPUT。
   - 这是决定 TuringOS 是不是真 AGI 方向的**关键 go/no-go 实验**。若市场显著 > 单 agent → 核心论题成立;若不 → TuringOS 是优雅基础设施,需重新定位。
2. **【规模相变】在硬任务上重跑规模曲线**(N=4/16/30/90,ζ 规则 tx≥agents×20),看 PPUT 是否随规模出现相变(易任务上是平的;硬任务上应该升)。
3. **【价格博弈深化】修 challenge monetary_invariant + 诱导校准 confidence/加显式 bet** → 看是否产生 ζ 式剧烈价格发现(contested 节点、whale、做空)。这验证"价格发现是否真在聚合信息"。
4. **【编码域】装 swebench venv + 真 Docker judge**,跑 SWE-bench 硬实例的 market run → 首个可与行业(低置信)对标的编码坐标。注意 OBL-009 的 apply-gate 教训(M7 diff materializer 已备)。
5. **【可审计性压测】**对 G1 run 做 clean-context 审计 + tamper 测试(audit_tape),验证 tape-canonical 的防篡改性——支撑方向 B 叙事。

---

## 附:本阶段交付物索引
- PR #216(MERGED):G0 激活(11/11)+ M7 diff materializer + v4 设计 + §8 治理。
- PR #217:G1/G2 live 能力层。
- `src/bin/g0_market_activation_current_kernel.rs`(G0 确定性市场)。
- `src/bin/g1_market_live_agent.rs`(G1 live-LLM 市场,`--strict` 真验证器)。
- `src/judges/shared_output_adapter.rs`(M7 git2 diff materializer)。
- 报告:`handover/reports/G1_G2_LIVE_MARKET_PPUT_REPORT_2026-05-30.md`(G1/G2 细节)+ 本文。
- 设计:`handover/SWEBENCH_MARKET_ACTIVATION_BENCHMARK_v4_2026-05-29.md`。
- OBL-010 / OBL-011:均 satisfied。
