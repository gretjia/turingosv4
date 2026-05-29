# TuringOS × SWE-bench — Constitutional Market Activation Benchmark (v4)
### Constitutional priced-DAG market search vs bare-flash / same-budget baselines

> 文档状态：DESIGN ONLY（设计稿，无实现代码，仅给 seam 引用 + Class 标注 + 门条件）
> 日期：2026-05-29 · 仓库：`/Users/zephryj/work/turingosv4` · 基线 HEAD：`1f00012d` (main)
> 运行分支：`claude/swebench-agi-benchmark`（PR-only，禁 merge main，AGENTS.md §14a）
> 取代：`SWEBENCH_V4_COMPARATIVE_BENCHMARK_DESIGN_2026-05-29.md`（v3，已废弃；仅作审计轨迹 + 子部件来源）
> 撰写依据：架构师 2026-05-29 拍板（A′ 市场激活路线）+ 已核实的市场机器地图（probe wgwsczqzd）+ kernel/role/init/flash/dsml findings。所有 file:line 已对照 `1f00012d`。

---

## 0. 核心订正（本 v4 的存在理由）

> **TuringOS loop = priced DAG search under constitutional market dynamics.**
> **不是** `worker → judge → retry → judge → retry`。

v3 把 TuringOS 降维成"单 flash worker + verify-retry 循环"——那是普通 agent scaffold,**测不到宪法里的 TuringOS**。宪法真正的循环是:

```
genesis
→ 初始化经济态 / 钱包 / agent roster / 任务市场 / AMM
→ N 个角色分化 agent 读取【价格信号 + 有限上下文】
→ 按价格选择历史 DAG 节点作为 parent（可分叉、回溯、非线性）
→ 生成新节点 或 下注 YES/NO
→ 价格更新 → 价格广播
→ 其他 agent 据价格继续分叉 / 修复 / 做空 / 延展
→ 到预算 或 候选 OMEGA
→ sealed true-verifier settlement（真 Docker/Lean,只此一次）
→ payout / attribution / golden-path 抽取（全部可从 tape 重建）
```

**关键事实(probe 已核实):现版本里市场原语已大量建好,缺的是"激活层"。** 见 §3、§8。

| 已建好(生产 src/,wired) | 缺失(激活层) |
|---|---|
| `EconomicState` 16 整数子字段(q_state.rs:171);真 CPMM AMM(`cpmm_pools_t` q_state.rs:749,k=yes×no sequencer.rs:4010+);stake-ratio price_index(price_index.rs:164);YES/NO 份额(`conditional_share_balances_t` q_state.rs:637);Bull/Bear 角色强制(real5_roles.rs:619);全节点价格广播(`UniverseSnapshot.price_index` snapshot.rs:58);价格驱动父节点选择(`boltzmann_select_parent_v2` actor.rs:46);引用 DAG(`ProposalTelemetry.parent_tx` proposal_telemetry.rs:135);golden path(kernel.rs:55);OMEGA(`RunOutcome::OmegaAccepted` typed_tx.rs:243) | **① N-agent 并发编排器**(现内核单 agent:MemoryKernel memory_kernel.rs:82,TuringBus 串行 bus.rs:169;batch_orchestrator 顺序、n_agents 恒 1);**② 真验证器结算 OMEGA**(现为自我宣称 VerifyTx + 内容静态谓词,无 Docker);**③ payout/redeem arm**(EventResolveTx 已定义 typed_tx.rs:1616,但赎回赢家份额→Coin 的 sequencer arm 缺);**④ 角色装配进运行循环**(`agent_role_assignment=vec![]` cmd_generate.rs:2397);**⑤ roster 扩到 N**(polymarket 硬编码 1-3 worker,genesis preseed 5 钱包) |

所以本 campaign 的本质是**"造激活层,把已建好的宪法市场跑起来,并用真验证器结算"**,而非"把 SWE-bench 塞进 loop"。这必然触 §6 受限面(sequencer / typed_tx / predicate registry)= **Class 3/4,逐原子 §8 批准**。

---

## 1. 科学主张（claims）

PRIMARY 从 v3 的"loop vs bare Δresolve"**降级**,新主张是市场的**规模/成本曲线**:

> **resolve_rate, candidate_quality = f(num_agents, tx_budget, role_mix, verifier_strength, cost)**

| 编号 | 主张 | 验证方式 |
|---|---|---|
| **C0** | **市场可激活**:从 genesis 进入能跑起一个真·priced-DAG 多 agent 市场(多 agent 独立建节点、YES/NO 双边交易、价格变动、价格驱动非线性 parent 选择、sealed 真验证器结算、全程 tape 可重建) | G0(§7),单实例,resolve 可为 0 |
| **C1** | **市场 > 同预算无市场**:`A3 − A1` = 宪法市场相对同 token/tx 预算的裸独立采样的增益 | 配对,Wilson CI + McNemar |
| **C2** | **规模相变**:resolve/candidate-quality 随 `agents × tx_budget` 出现相变(ζ 先验 `tx ≥ agents×20`) | scale grid(§4),曲线形状 |
| **C3** | **价格发现本身的贡献**:`A3 − A2` = 价格驱动 vs 多 agent 但随机/round-robin parent(隔离"多 agent"与"市场"两件事) | 配对 |
| **C4** | **验证器强度差**:`A4 − A3` = 隐藏测试低维反馈进市场 vs 不进。回答"TuringOS 受限于市场机制,还是受限于 SWE-bench 合法验证器太弱" | 跨协议 GAP |
| **C5** | **成本前沿**:cost-per-resolved(拆 cache-hit/miss/output),论证"把便宜弱模型变成集体" | 经济指标,全部 tape 可重建 |

**明确不主张**:① 不与 OpenHands/Hermes/Amazon Q 做严格头对头(model×scaffold 混淆,见 v3 §2.2 坐标表,仅作低置信方位)。② 不把 SWE-bench 分数当 TuringOS 能力上限(SWE-bench 是逆境主场,§7-定位)。③ 不在隐藏测试进搜索的口径下声称 leaderboard 可比(那是 A4 诊断模式)。

---

## 2. 协议（protocols）

| 协议 | 隐藏测试 | settlement | 用途 | leaderboard 可比 |
|---|---|---|---|---|
| **Standard-clean(主)** | 不进 prompt / retry / 市场搜索 | sealed,只在最终对单一候选跑一次真 Docker | 行业方位 + 主科学结论 | ✅(勉强,带污染 caveat) |
| **Oracle-feedback(诊断)** | 隐藏 judge 低维失败信号可进市场 | 同上 | 测天花板 / 验证器强度差 C4 | ❌ |
| **Strong-verifier(自然主场)** | N/A(合法验证器本就强) | Lean kernel / 编译器 / 类型检查 | 测 TuringOS 自然优势域 | 另算(非 SWE-bench) |

**价格广播 vs 上下文屏蔽(架构师补的边界,承重)**——agent 能看到所有节点**价格**,**不等于**能看到所有节点**完整上下文**(否则群体智慧退化成单黑盒智慧,违 Art. III 相关性屏蔽):

```
全量广播：node_id, price_yes, price_no, liquidity, volume, depth, author_role, parent, summary_hash
选择披露：agent 决定 follow/inspect 某节点时,再读该节点摘要/局部内容
严格屏蔽：hidden tests, gold_patch, test_patch, FAIL_TO_PASS/PASS_TO_PASS 名称, 私有 judge 结果, raw autopsy
```

现版本已部分实现:`UniverseSnapshot.price_index`(snapshot.rs:58)广播价格;`mask_set`(BoltzmannMaskPolicy)抑制被支配父节点的读视图;prompt "=== Market ===" 块(prompt.rs:91-95,market_context.rs:121-153)。**待补:summary_hash 级广播 + follow-then-inspect 的选择披露分层**(M5)。

---

## 3. 系统架构（genesis → settlement）

完整生命周期 stage(已核实,供实现锚定)。★ = 现已 wired;◇ = 激活层待建。

```
★ Stage A  genesis：build_chaintape_sequencer_with_initial_q(runtime/mod.rs:706)
            → Ed25519 keypair, pinned_pubkeys.json, QState::genesis(q_state.rs:958),
              CasStore, rejections.jsonl, Sequencer 起,run_chaintape_driver spawn;
              NonEmptyRuntimeRepo fail-closed(runtime/mod.rs:731)。genesis-per-task。
◇ Stage B  roster 装配：preseed N 钱包 + N keypair(AgentKeypairRegistry)+ 角色分配
            (Solver/Bull/Bear[/Mathematician]);现 agent_role_assignment=vec![](cmd_generate.rs:2397)→ 必接。
★ Stage C  任务市场：TaskOpenTx → EscrowLockTx(escrow bounty) → MarketSeedTx(CPMM 池 seed);
            polymarket 序列已 wired(cmd_generate.rs: emit_polymarket_market_for_session:2293)。
◇ Stage D  N-agent 市场循环(核心缺失,M0)：
            并发 N agent →（读价格+有限上下文 snapshot.rs:58 / prompt.rs:91）
            → 按价格选 parent(boltzmann_select_parent_v2 actor.rs:46,现单 agent)
            → 出 WorkTx(新节点,parent_tx 引用,proposal_telemetry.rs:227)
              或 invest(BuyWithCoinRouterTx typed_tx.rs:1605,Bull=BuyYes/Bear=BuyNo real5_roles.rs:619)
            → sequencer apply → 价格更新(CPMM k=yes×no sequencer.rs:4010)→ 广播
            → 直到 tx_budget 耗尽 或 出现候选 OMEGA。
◇ Stage E  sealed settlement(M2,真验证器)：对市场选出的候选 patch(最高 price_yes 路径终点)
            跑真 SwebenchTestJudge::verdict(swebench_test_judge.rs:159 → run_evaluation → resolved:288)。
            通过 → 这是真 OMEGA;不通过 → 该候选 short 获利。**只此一次,不进搜索。**
◇ Stage F  payout/resolution(M3)：EventResolveTx(typed_tx.rs:1616)→ 赎回赢家 YES/NO 份额→Coin
            (现缺 redeem arm)→ FinalizeRewardTx → reputation 更新 → claims_t 结算。
★ Stage G  golden path 抽取 + GenesisReport：trace_golden_path(kernel.rs:55);
            genesis_report.json;evidence manifest(全部 CID + tx_id + landed)。
```

**关键设计原则(宪法对齐)**:
- **价格不是 sidecar,是推理物理过程的一部分**(Art. 0.2):cost/time/provenance/market price/wallet/search history/Boltzmann routing 必须可从 tape/CAS 重建。`price_index_t` 已是纯派生视图(TB-14 移出 canonical,price_index.rs:8-11"price is signal, not truth")——这是对的,但**派生函数的全部输入(node_positions/cpmm_pools)必须在 tape 上**。
- **价格广播是群体涌现引擎**(Art. II.2):系统只广播价值信号,不告诉黑盒怎么做,让黑盒自发调整注意力。
- **市场找路,真验证器结算**:杜绝 ζ 老run"自我宣称 OMEGA、proof quality LOW"的病。

---

## 4. 实验臂（arms）

| Arm | 名称 | 配置 | 隔离的变量 |
|---|---|---|---|
| **A0** | bare flash@1 | 单发裸 deepseek-v4-flash(thinking disabled) | 产品直觉基线 |
| **A1** | bare flash@budget | 同总 tx/token 预算,多次独立采样,无市场、无 parent 选择 | "多抽几次" |
| **A2** | multi-agent no-market | N agent,parent 随机/round-robin,**不看价格、不下注** | 隔离"多 agent"本身 |
| **A3** | **TuringOS market-standard(主)** | N agent + 价格发现 + 角色分化 + DAG 路由,无 hidden feedback,sealed 真 Docker 结算 | **完整宪法市场** |
| **A4** | TuringOS market-oracle | 同 A3,但隐藏测试低维信号可进市场 | 天花板 / 验证器强度差 |
| **A5** | TuringOS market-strong-verifier | Lean/编译器任务域(非 SWE-bench) | TuringOS 自然优势域 |

**关键对比**:
```
A3 − A2 = 价格发现/市场机制本身的贡献   (C3)
A3 − A1 = 宪法市场 vs 同预算裸采样的贡献  (C1)
A4 − A3 = verifier strength gap          (C4)  ← 回答"市场弱还是 SWE-bench 验证器弱"
```

**Scale grid(先测曲线形状,别一上来只跑 90)**:
```
agents:    1, 3, 10, 30, 90
tx/agent:  5, 20, 60        （ζ 先验 tx_budget ≥ agents×20 作起点,非定论）
roles:     Solver / Bull / Bear   （Mathematician 视 M1 是否补回）
```
真正要看的不是"90 agent 是否赢",而是:**市场是否随 agents×budget 出现相变? Bear 是否在高预算后开始有效做空? 高 price_yes 路径是否比随机路径更接近 resolved(价格能否提前识别 golden path)?**

---

## 5. 市场指标（全部 tape/CAS 可重建,报告仅派生视图）

| 指标 | 含义 | 来源 seam |
|---|---|---|
| resolve_rate + Wilson 95% CI | 主能力指标 | `WilsonCi::new_95` wilson_ci.rs:36;真 Docker resolved(M2) |
| candidate_quality | sealed 前市场选出的候选 patch 的真 resolve 率 | M2 settlement |
| price calibration | 高 price_yes 节点是否真的更可能在 golden path 上(Brier/AUC) | price_index + 最终 settlement |
| golden-path price trajectory | golden path 上各节点价格随时间轨迹(价格能否提前识别) | trace_golden_path kernel.rs:55 + price 历史 |
| branching_factor / depth / roots | DAG 形态(非线性证据) | dag_view.rs(BFS depth 334-351) |
| parent-selection entropy | parent 选择是否真由价格驱动(vs 线性) | boltzmann_select_parent_v2 actor.rs:46 |
| bear utility | Bear 做空的净收益 / 是否有效杀死死节点 | node_positions_t Short / claims_t |
| whale concentration | 资本集中度(现 Rust 端无 whale 逻辑,仅前端 stub citation-dag.ts:178)→ M6 补 | node_positions_t 聚合 |
| YES/NO volume, liquidity | 市场活跃度 | cpmm_pools_t / conditional_share_balances_t |
| cost-per-resolved | 拆 cache-hit($0.0028/M)/miss($0.14/M)/output($0.28/M) | ProposalTelemetry.token_counts(CAS,已 wired)+ pricing_table_sha256 |

**Art. 0.2 红线**:每次 LLM call 的 provider/model/request_id/prompt&completion&cache tokens/thinking_state/price_table_version 必须进 tape/CAS(full-flow 的 `ProposalTelemetry` 已写 token_counts 到 CAS,延用);报告含 `tape_root / cas_root / pricing_table_sha256`,只做派生。

---

## 6. SWE-bench 专属屏蔽与口径（沿用 v3 仍有效部分）

- **三层屏蔽**(§2):价格全广播 / 摘要选择披露 / 隐藏测试严格屏蔽。`SwebenchSampleInput`(cmd_tdma.rs:37)按构造只反序列化 6 字段,结构性屏蔽 gold/test——**禁止给它加 gold/test 字段**(沿用 v3 §5.3)。
- **隐藏测试只在 sealed settlement 跑一次**(M2),绝不进 prompt/retry/市场。FAIL_TO_PASS/PASS_TO_PASS 名称在 standard 协议下**不进任何 prompt**(订正 v3 的"测试名可回喂"——审计 P0.1/P0.4)。
- **output adapter parity**:所有 arm(A0-A4)共用同一 patch parser/materializer/fallback + 版本 hash(`output_adapter_version_sha256` 两臂必须一致),否则 Δ 可能是"Rust judge 比 Python parser 鲁棒"(审计 P1.7)。
- **gold-gating**(v3 §4.3,沿用 + 强化):独立 workspace 跑 gold;gold/test/gold-log 不进 agent workspace;预注册 replacement policy(防 silent cherry-pick,审计 P1.8);报告候选集大小 / 排除 ID / 原因 / 最终分布 / 与官方 500 的偏差。
- **hermetic 白名单 + 分层抽样**(v3 §4.4,沿用 + 强化):django/sympy/sphinx/pytest/xarray/pylint;requests 非 hermetic 必排除;stratified by repo+difficulty;标注"hermetic Python subset"(审计 P1.9)。
- **DeepSeek V4 Flash 集成防御**(v3 §5.5,沿用,承重):worker/agent 模型 = `deepseek-v4-flash`,**显式 `{"thinking":{"type":"disabled"}}`**(默认 thinking-ON,LiteLLM #27453);DSML-leak 检测+恢复;`reasoning_content` 多轮(flash disabled 下基本消失);patch 提交尽量包成工具调用绕开 diff 文本解析。thinking 已 per-role 可配、未硬编码(cmd_llm.rs:508/527)。

---

## 7. Go/No-Go 门（先证机制,再证能力）

**G0 — single-instance constitutional market activation(机制激活,不要求解题)**
选 1 个 hermetic SWE-bench 实例,5–10 agent,10–20 tx/agent。通过条件(全部满足):
```
1.  从 genesis 进入,初始化 task market / wallets / agent roster
2.  ≥5 agent 参与
3.  ≥3 角色:Solver / Bull / Bear
4.  非线性 DAG:branching_factor > 1
5.  ≥1 agent 选择非最新节点作为 parent
6.  ≥1 次 YES/NO 双边交易(至少各一笔)
7.  ≥1 节点价格发生显著变化
8.  market price / wallet / parent selection / boltzmann routing 均可从 tape 重建
9.  hidden tests 未进入任何 prompt
10. 最终只选一个 sealed candidate patch 跑真 Docker settlement
11. settlement 结果写入 tape/CAS,报告仅派生视图
```
**resolve=0 也算 G0 成功**——它证明现版本真·TuringOS 市场机制被激活了。

**G1 — market improves candidate selection**:扩到 10/30/90 agent,在 flask/django smoke 上观察价格发现是否提升候选质量(高 price_yes 路径是否更接近 resolved)。

**G2 — n=50 hermetic**:Verified hermetic 分层子集,跑 scale grid,出规模/成本曲线。

**G3 — Pro / public / heldout**:迁到 SWE-bench Pro 或 heldout 做行业坐标。

**横贯门(每阶段)**:FC1 不变量 `evaluator_reported_completed_llm_calls = step+parse_fail+llm_err`,失败即 HALT(CLAUDE.md §4)。

---

## 8. 实现拆解（atoms）与 Class 标注

> 这是"造激活层"工程,多数触 §6 受限面。**每个 Class 3/4 atom 须独立 §8 批准 + clean-context 审计**。建议顺序:M5/M7(低风险先行)→ M0 → M1 → M2 → M3 → M4 → M6。

| Atom | 内容 | 关键 seam | Class | §6 受限? |
|---|---|---|---|---|
| **M0** | **N-agent 并发编排器**:并发跑 N 个角色 agent,对共享 Sequencer 提交 WorkTx/invest/Verify。现内核单 agent(memory_kernel.rs:82,bus.rs:169 串行,batch_orchestrator n_agents=1 @ :418)。Sequencer 异步队列结构上支持(`submit_agent_tx`)。 | sequencer.rs(submit/apply),batch_orchestrator.rs,AgentKeypairRegistry | **3-4**(并发对 sequencer admission 的交互;若改 admission = 4) | ✅ |
| **M1** | **角色装配进运行循环**:把 Bull/Bear/Solver 角色分配接进 generate/run(现 `agent_role_assignment=vec![]` cmd_generate.rs:2397)。real5_roles 强制逻辑已在(real5_roles.rs:619)。是否补回 Mathematician 角色由你定。 | cmd_generate.rs:2397,real5_roles.rs | **2-3** | 部分 |
| **M2** | **真验证器结算 OMEGA**:新增 settlement 谓词,sealed 时跑真 SwebenchTestJudge(swebench_test_judge.rs:159);替掉自我宣称 VerifyTx + 内容静态谓词(registry.rs:607,Settlement bundle 现为空)。 | predicate registry registry.rs:607,verify_work_predicates sequencer.rs:1225,activate_predicate_binding runtime/mod.rs:872 | **4**(predicate registry + sequencer admission) | ✅ |
| **M3** | **payout/redeem arm**:补 EventResolveTx(typed_tx.rs:1616)的赎回赢家份额→Coin(从 conditional_collateral_t),现缺;FinalizeReward/reputation 结算。 | sequencer.rs(settlement arm),conditional_collateral_t q_state.rs:622 | **3-4**(money/CAS + 可能 admission) | ✅ |
| **M4** | **roster/preseed 扩到 N**:genesis preseed N 钱包 + N keypair + 调度 N agent(现硬编码 1-3 worker,5 钱包)。 | cmd_init genesis 模板,POLYMARKET_WORKER_IDS cmd_generate.rs:111,AgentKeypairRegistry | **2** | 否 |
| **M5** | **SWE-bench 任务适配 + 三层屏蔽 + target-file 上下文**:issue→task-market;价格全广播/摘要选择披露/隐藏严格屏蔽;target_files(v3 Atom1)。 | SwebenchSampleInput cmd_tdma.rs:37,make_swebench_user_prompt:259,新建 fetch 脚本 | **2** | 否 |
| **M6** | **市场指标 + cost/price 入 tape + scale-curve 报告 + 坐标 lockfile**:§5 全指标;ProposalTelemetry token→cost;market_coordinates_<UTC>.lock.json。 | ProposalTelemetry CAS,wilson_ci.rs:36,dag_view.rs | **2** | 否 |
| **M7** | **确定性 diff materializer(共享 output adapter)**:search/replace→确定性 diff(v3 Atom2),`similar` crate 不在 Cargo.toml(须加或 git diff --no-index);raw-diff 一等回退;两臂共用。 | swebench_test_judge.rs:78/161,SWEBENCH_SYSTEM_PROMPT cmd_tdma.rs:252 | **1-2** | 否 |

**已废弃(v3 单 agent 路线的 Atom 0/0.5/3 中的"单发"语义)**:不再用 `tdma run --judge swebench` 单 agent 路径作 PRIMARY;`tdma run` 单 agent verify-retry 保留为 **legacy control(可选,非主)**。

---

## 9. 宪法合规与风险登记

**Class 与审计**:M0/M2/M3 触 §6(sequencer admission / typed_tx / predicate registry)= Class 3-4,**逐原子 §8 批准 + PRE-§8 clean-context 审计**(AGENTS.md §9/§14)。M2 改 predicate registry 必为 Class 4。

**Art. 0.2 tape canonicity**:所有 price/wallet/routing/cost/search-history 信号从 tape/CAS 重建;报告派生;含 tape_root/cas_root/pricing_table_sha256。**禁** dashboard/sidecar 作真相源(否则 SECOND-SOURCE-DRIFT)。

**整数货币**:money/market 路径只用整数(MicroCoin=i64),无 f64(老版 f64 CPMM 已 excise,现 TB-13/14 整数 CPMM)。

**屏蔽**:hidden tests/gold/test_patch/raw autopsy 永不进 prompt(Art. III + AGENTS.md §12)。

**PR-only**:分支 `claude/swebench-agi-benchmark`,禁 merge main;不提交 `handover/evidence/**` sidecar(no-sidecar CI);liveness gate 用 fs::read_dir → 新模块/脚本先注册、干净 worktree 验证(#212 教训)。

**风险登记(显式标红,不阻塞但记录)**:
1. **Art. 0.4 漂移**:宪法文本(constitution.md:149)说 git substrate "未决/0 git hits",但代码已实现 Path B(`git_tape_ledger.rs` git2,Atom 20-22)。→ **本 campaign 不修宪**,报告显式写 drift + 给机器证据(git fsck / replay / tape_root / cas_root / state_root / price 派生);**独立 Class 4 task 修订 Art. 0.4 + amendment log**。
2. **市场两价信号未对齐**:CPMM 池价(给 agent 看)vs stake-ratio 价(scheduler 用)未 reconcile(probe gap#7)→ M6 需统一或显式声明用哪个。
3. **SWE-bench 是逆境主场**:合法验证器弱(修复前 repo 已有测试通常不暴露 issue;隐藏测试不能进搜索)。→ 结论写"TuringOS market 在弱合法 verifier 真实软工任务上的表现",**不写**"TuringOS 完整能力上限"。自然优势域 = 强合法验证器 + 大搜索空间 + 可分叉 DAG(Lean/编译器/形式化),见 A5。
4. **SWE-bench Verified 污染/饱和**(OpenAI 2026 已不再用它衡量 frontier):Verified 仅作内部消融 + 低置信坐标;行业定位迁 SWE-bench Pro(Scale,1865 任务/41 仓库)→ heldout。

**SWE-bench 定位四阶段**:
```
Stage 1  Verified-hermetic, n 小   →  机制激活 / 消融 / 工程稳定
Stage 2  Verified-hermetic 全子集  →  低成本内部曲线
Stage 3  SWE-bench Pro public      →  行业坐标
Stage 4  private / heldout / sealed →  真正可信定位
```

**行业定位结论(预期写法)**:TuringOS = **market-based swarm scaffold / cost-frontier weak-model amplifier**——能否把大量便宜弱模型,通过价格发现、角色博弈、真验证器结算,转化为单体没有的集体解题能力。**不是**"又一个单 agent coding scaffold 拿了 X%"。

---

## 附录 A — ζ Run 6 案例的教训（市场必须由真验证器结算）

ζ Run 6(90 agent × 6000tx,Qwen2.5-7B)价值不在"弱模型证了 ζ",而在证明 `弱模型 + 多 agent + 高 tx 预算 + 市场路由` 能产生深链与集体搜索;scaling 经验 `tx ≥ agents×20`。但它暴露:golden path 大量重复句、proof quality LOW、**OMEGA 近似自我宣称**。→ 现版本超越老版本的关键:`market finds path → TRUE verifier settles OMEGA`(SWE-bench=Docker;Lean=kernel;codegen=compiler/tests/type-checker 组合)。这是 TuringOS 从"市场叙事"升级为"可审计通用机器"的关键。

## 附录 B — 已废弃 v3 与本 v4 的关系

v3(`SWEBENCH_V4_COMPARATIVE_BENCHMARK_DESIGN_2026-05-29.md`)是单 agent verify-retry 心智模型,已废弃。v4 **复用** v3 中仍有效的子部件来源(不重复):gold-gating 程序、hermetic 白名单、DeepSeek V4 Flash 集成防御 §5.5、已核实 seam 校正、市场坐标混淆表 §2.2、统计方法(Wilson/McNemar/配对 bootstrap)。v3 的 D1-D4 订正(无假 git tape / worker=flash / meta=flash / 从 genesis 进)在 v4 里被市场框架吸收并深化。

## 附录 C — 新 session 冷启动 prompt（paste-ready）

```
你是 TuringOS 的实现工程师。仓库 /Users/zephryj/work/turingosv4(main @ 1f00012d),分支 claude/swebench-agi-benchmark。
冷启动读 AGENTS.md → HARNESS_PLAYBOOK.md → constitution.md → handover/ai-direct/LATEST.md。
目标:激活宪法市场(A′ 路线),用 SWE-bench 当任务域。设计定稿见
handover/SWEBENCH_MARKET_ACTIVATION_BENCHMARK_v4_2026-05-29.md(本 prompt 承接它)。路线已决,不重开辩论。

== 核心(勿降维)==
TuringOS loop = priced DAG market search,不是单 agent verify-retry。
现版本市场原语已建(EconomicState/CPMM/YES-NO/price_index/boltzmann parent select/Bull-Bear,见 v4 §0 表),
缺激活层:① N-agent 并发编排 ② 真验证器结算 OMEGA ③ payout arm ④ 角色装配 ⑤ roster 扩 N。

== 铁律 ==
1. 市场找路,真验证器结算 OMEGA(SWE-bench=真 Docker SwebenchTestJudge,只 sealed 跑一次)。
2. 价格全广播 / 摘要选择披露 / 隐藏测试(gold/test_patch/FAIL_TO_PASS名)严格屏蔽,绝不进 prompt/retry/搜索。
3. 所有 price/wallet/routing/cost 从 tape/CAS 重建(Art.0.2),报告仅派生,整数货币无 f64。
4. worker/agent = deepseek-v4-flash + 显式 thinking disabled;DSML-leak 防御。
5. PR-only 禁 merge main;Class 3/4 逐原子 §8 + clean-context 审计;M0/M2/M3 触 §6 受限面。
6. 先证机制(G0)再证能力;真跑无虚假确定性;flake 绝不当 resolved。

== 第一步 ==
跑 gh pr list 查路径冲突 → 起 G0 charter(单实例 5-10 agent 市场激活,11 条通过条件见 v4 §7)
→ 干净 worktree 实现 M5/M7(低风险)→ 然后 M0/M1 → 到 G0 门停下等真跑验证。
M2/M3 触 sequencer/predicate-registry = Class 4,动手前先取架构师 §8 批准。
```
