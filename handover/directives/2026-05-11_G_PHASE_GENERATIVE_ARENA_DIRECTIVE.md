# 2026-05-11 — G-Phase / Generative Arena Architect Directive

**Status**: ARCHIVED — awaiting authorization to execute. Archive only per `/architect-ingest`.

**Ingestion context**:
- Triggered by user invocation of `/ultraplan` with full architect verdict text
- Remote ultraplan session running at https://claude.ai/code/session_01QqSehGhpsts18AC5qExyAS?from=cli (plan pending user approval)
- User verbatim ask: "根据架构师意见做完整的开发计划、验证方案、对齐审计方案，落实到 phase, module, atom 三个层面"
- User verbatim: "本期任务的核心为让 agent 在同一条 tape 上持续生活、赚钱、亏损、学习、分化"

**Layer 1 invariant check**: PASS — no kernel.rs / append-only DAG / economic-conservation violation. Directive explicitly preserves constitutional gates (tape-first, no ghost liquidity, no price-as-truth, dashboard materialized view only, no real funds, no public chain). Directive recasts constitution from "防御闸门" to "竞技场边界" without modifying invariants.

**Risk class**: Class-4 candidate (new phase declaration; cross-cuts sequencer, scheduler, runtime_repo lifecycle, prompt-context surface, agent persistence model). Per-atom §8 will be required for each Class-4 atom landing under G1..G7.

---

## 0. Architect Verdict — Core Position

> **TuringOS 防御宪法阶段基本成功；现在必须进入生成性市场阶段。**
> 这不是放松宪法，而是把宪法变成竞技场边界。

```
Constitution is no longer the product.
Constitution is the arena boundary.
The product is persistent multi-agent market collaboration on tape.
```

Architect rejects framing "宪法太严应该放松". Correct understanding:

> **宪法是必要边界，但不是生成动力。**
> 现在要从 "constitution-defensive substrate" 进入 "constitution-gated generative arena".

---

## 1. Current State Audit (架构师视角)

### 1.1 事实层 — TB-N3 substrate 成功

```text
9 real MiniF2F problems × n=5 multi-agent × deepseek-chat
5/9 solved, including 2 hard
CPMM kernel complete
EventResolveTx first witnessed firing on real-LLM tape
but:
  complete_set_mint = 0
  cpmm_pool = 0
  cpmm_swap = 0
  buy_with_coin_router = 0
```

TB-N3 charter 诊断 verbatim: `gap is wire, not capability`.

Phase 2 现象:

```text
6/6 accepted WorkTx 都正确 auto-emit node-survive market
9/9 audit_tape PROCEED
解题率 5 -> 6 提升
但:
  0 invest emission
  0 verify_peer
  0 finalize_reward / event_resolve / cpmm_swap
```

Architect 接受诊断: **TB-N3 substrate landed，但 agent behavior 没有进入 market loop**.

### 1.2 宪法层 — defensive harness 成功

```text
workspace tests: 1181 / 0 / 151
constitution gates: 97 / 0 / 1
matrix RED rows: 0
matrix AMBER rows: 13
```

(注: snapshot 引用; 当前实际值见 `bash scripts/run_constitution_gates.sh` 与 TB_LOG.tsv)

```text
Defensive engineering: strong.
Generative economy: weak / not yet born.
```

### 1.3 真正瓶颈 — 没有持续博弈

> v4 的 economy 现在是 constructivist，不是 emergent.

类比:

```text
合规交易所
KYC 完整
审计完整
账本完整
订单撮合模块也有
但没有真正的市场参与者在持续博弈
```

机制缺失:

```text
持续身份 / 持续余额 / 持续仓位 / 持续损益
持续 reputation / 破产风险 / 跨问题学习 / 价格信号反馈
```

每个 problem 都 fresh runtime_repo + fresh genesis → "每一轮开局都把交易员洗白、清仓、重置记忆。那市场不会涌现."

---

## 2. 四思想家视角

### 2.1 Hayek — 价格是分散知识的通信协议
- PriceIndex 不能只是 dashboard item
- Price 必须进入 agent scoped read view
- Position / PnL 必须跨 problem 存续
- Agent 必须因为错误价格判断而变穷、变弱、失去调度优先级
- 否则价格不传递知识，只是漂亮的统计字段

### 2.2 Nakamoto — 没有持续激励就没有自治网络
- 账本 + 激励 + 竞争 + 成本 + 奖励
- 当前: 账本有，agent 无持续经济后果
- 必须: persistent agent balance / positions / reputation; loss / bankruptcy / autopsy; scheduler priority from performance; market PnL

### 2.3 Turing — Tape 是计算过程
- TB-18 N→1 collapse 教训: 最终 proof 上链 ≠ 图灵机 tape
- TB-N3 新版本: 市场结构在 tape 上，但 Agent 的经济行动没有进入 tape
- 新目标: not only proof attempts on tape, but also economic attempts on tape
- 包括: seen price / invest decision / no-trade reason / buy/sell tx / failed invest / position update / PnL / autopsy

### 2.4 Drucker — What gets measured and managed gets done
- 当前管理: solve proof / submit proof / pass Lean
- 不管理: allocate capital / interpret price / challenge wrong nodes / verify peers / short bad nodes / improve future EV
- 因此 agent 不投资是合理的；它在按你实际管理的目标行动

---

## 3. 优先级裁决 (架构师重排)

```text
1. Cross-Problem Persistence — 最高优先级
2. Market Decision Observability — 与 1 同时做
3. Multi-LLM mix — 中优先级，紧随其后
4. Incentive / Scorecard / PnL feedback — 必须跟 1 绑定
5. Epistemic pricing feedback — 暂时 observe-only，不进 sequencer enforce
```

> 不要先做更复杂的 market mechanics。先让 agent 在一个连续世界里活下去。

---

## 4. 新阶段命名

```text
G-Phase: Constitution-Gated Generative Arena
```

目标:

```text
在不破坏 L4/L4.E、CTF 守恒、选择性屏蔽、tape-first 的前提下，
让 agent 产生持续经济行为和角色分化。
```

---

## 5. 立即路线图

### G0 — Charter reset: 进入 Generative Arena

File: `handover/tracer_bullets/TB_G0_GENERATIVE_ARENA_CHARTER.md`

```text
Objective:
  discover whether persistent multi-agent market dynamics emerge
  under constitutional gates.

Not objective:
  more substrate compliance
  public benchmark
  real-world readiness
  DeFi expansion
```

验收:

```text
substrate remains green
market activity appears or no-trade reasons explain absence
```

### G1 — Cross-Problem Persistence (最重要)

目标:

```text
一个 batch 共用一个 runtime_repo + 一个 CAS + 一个 agent state.
```

结构:

```text
batch_runtime_repo
  task_1
  task_2
  ...
  task_9
```

必须持久化: agent balance / positions / reputation / PnL / autopsy / market history / proof performance.

Ship gates:

```text
SG-G1.1  9 problems share one runtime_repo and one CAS.
SG-G1.2  problem k+1 starts from problem k final HEAD_t.
SG-G1.3  agent balances persist across problems.
SG-G1.4  agent positions persist across problems.
SG-G1.5  agent reputation persists across problems.
SG-G1.6  No new per-problem 1 Coin genesis reset.
SG-G1.7  Run report can show agent PnL trajectory across all 9 problems.
SG-G1.8  At least one agent ends with different balance from start.
```

Halt if:

```text
per-problem isolation reappears
fresh genesis per problem
positions reset silently
balance reset silently
```

### G2 — MarketDecisionTrace + NoTradeReason

新对象:

```rust
MarketDecisionTrace {
    agent_id, task_id, prompt_context_hash,
    seen_nodes, seen_prices,
    chosen_node, direction, amount, quoted_price,
    reason_summary_public, outcome, tx_id_or_none,
}

enum NoTradeReason {
    NoPool, NoPromptTool, NoParsedInvest,
    InsufficientBalance, RouterRejected,
    AgentDeclined, TooFastSolve,
    NoPerceivedEdge, PromptBudgetExceeded,
}
```

Ship gates:

```text
SG-G2.1  Every agent turn with market context has either:
         MarketDecisionTrace with tx_id  OR  NoTradeReason.
SG-G2.2  Phase 2 no-trade batch can be classified without reading raw stdout.
SG-G2.3  NoTradeReason appears in dashboard and CAS.
SG-G2.4  Failed invest attempts enter L4.E.
```

Critical: otherwise `0 invest` remains mysterious.

### G3 — Persistent PnL / Solvency / Bankruptcy

```rust
AgentMarketState {
    agent_id, balance,
    open_positions,
    realized_pnl, unrealized_pnl,
    solvency_status,
    reputation_score,
}
```

Ship gates:

```text
SG-G3.1  agent balance changes persist across tasks.
SG-G3.2  market gain/loss changes scheduling metadata.
SG-G3.3  bankrupt / low-balance agent receives AutopsyCapsule.
SG-G3.4  bankrupt agent cannot continue unlimited risk-taking.
SG-G3.5  PnL is visible in dashboard as materialized view.
```

Without this, bull/bear roles cannot emerge.

### G4 — Multi-LLM Mix

配置:

```text
Agent_0 = claude-sonnet
Agent_1 = gpt-5
Agent_2 = qwen
Agent_3 = deepseek
Agent_4 = local / fallback
```

记录: model_name / model_version / provider / temperature / prompt template.

Ship gates:

```text
SG-G4.1  At least 3 distinct model families participate.
SG-G4.2  Model identity is persistent per agent across tasks.
SG-G4.3  Run report shows model-level strategy divergence.
SG-G4.4  At least one market action or no-trade reason differs by model family.
SG-G4.5  No hidden model switch without ChainTape/CAS record.
```

目标不是提高 solved rate，而是制造认知差异。没有认知差异，就没有价格发现。

### G5 — Role Differentiation Measurement

角色:

```text
Solver-heavy  / Bull  / Bear  / Verifier
Challenger    / Market-maker  / Observer
```

Ship gates:

```text
SG-G5.1  Role classifier runs on ChainTape + CAS.
SG-G5.2  At least 2 non-identical roles detected in persistent batch.
SG-G5.3  Role classification does not use private CoT.
SG-G5.4  Role classification appears in dashboard.
SG-G5.5  If only one role appears, report explains mechanism bottleneck.
```

### G6 — Epistemic Pricing Feedback, Observe-only

```text
observe-only — 太早 enforce 危险
```

Price 影响 read-view ranking / proposal parent suggestion，但不改 predicate / sequencer admission.

Ship gates:

```text
SG-G6.1  Price appears in prompt context.
SG-G6.2  Agent citations can be analyzed against price.
SG-G6.3  High-price node selection rate is measured.
SG-G6.4  Price never changes L4/L4.E predicate decision.
SG-G6.5  Unresolved-challenged node cannot be promoted as safe.
```

"Price is signal, not truth" 真正落地.

### G7 — Run6-equivalent Structural Smoke

不追求 1748 tx 精确等价；追求结构等价.

最小目标:

```text
one persistent batch
>= 9 tasks
>= 5 agents
>= 3 model families
>= 2 roles detected
>= 1 buy yes
>= 1 buy no
>= 1 challenge
>= 1 verify
>= 1 payout
>= 1 autopsy
>= 1 persistent PnL difference
```

中级目标:

```text
>= 100 market tx
>= 20 proof-related tx
>= 3 tasks with persistent positions
>= 1 agent bankrupt or severely drawdown
>= 1 role switch after autopsy
```

---

## 6. 进入 Generative Phase 的裁决

> 是，进入 generative phase. 但不是放松宪法，而是把宪法作为固定边界，转向持续博弈环境的设计.

新主命题:

```text
Constitution is no longer the product.
Constitution is the arena boundary.
The product is persistent multi-agent market collaboration on tape.
```

---

## 7. 为何 Cross-Problem Persistence 必须先于 Multi-LLM

如果没有 persistence，multi-LLM 也只是"一群模型轮流重置状态". 不会产生 wealth accumulation / loss aversion / reputation / position carryover / autopsy learning / strategic memory.

顺序:

```text
G1 Persistence
G2 Decision trace
G3 PnL / solvency
G4 Multi-LLM
G5 Role classifier
G6 Price feedback
```

---

## 8. AI coder 尚未发现的问题

### 8.1 Market opens after node accepted — first-long alpha 被 WorkTx.stake 承担

```text
FirstLong exposure   = WorkTx.stake, before acceptance / at submission
Post-accept market   = node survive / reuse / challenge market
```

不要期待 router buy 在所有问题里自然出现.
若任务马上 solved，Agent 没时间或理由 trade，是正常的.

解决: persistent batch + survival/reuse markets + challenge incentives. NOT 强迫当前 problem 里立刻 buy.

### 8.2 verify_peer=0 比 invest=0 更危险

Market 可以没有，verifier network 不能没有.
若无 peer verify → RSP economy remains single-agent solve → system oracle → payout. 距离多 agent 协作还很远.

G1/G2 后应加 parallel priority: **Peer Verification Bridge**.

Ship gate: at least one non-solver VerifyTx on another agent's WorkTx.

### 8.3 Round-robin rotation 是伪 multi-agent

```text
agent_idx = tx % n_agents
```

不是竞争，只是轮流发言.

真 multi-agent 至少需要: active set / scheduler chooses based on state / agents can skip / bid / choose task / invest without proposing proof / verify/challenge others.

G1 后改 scheduler: from round-robin → opportunity-driven scheduler.

最小版选择集: propose proof / verify peer / challenge node / invest long/short / abstain.

### 8.4 Invest prompt ≠ utility function

`you may invest` 不会自动产生交易.

LLM 需要看到 persistent balance / future ability impact / current PnL / balance / positions. 必须把 PnL / solvency 加进 prompt context.

### 8.5 "空市场失败"作为科研结论

若 G1/G2/G3/G4 都做了，还是 0 trade — 这是重要科研结果:

```text
LLM proof-solving agents do not spontaneously engage in
prediction-market trading under this reward structure.
```

需要: specialized trader agents / explicit market-maker agents / different utility prompting / stronger reward coupling.

不要把 0 trade 当成失败；要把它变成可解释的实验结果.

---

## 9. 从现在到远景的完整路径

```text
Phase 0  Current substrate accepted (constitutional harness green, CPMM/NodeMarket/TB-N3 substrate landed)
Phase 1  Persistence Arena (single runtime_repo / persistent balances+positions+reputation / same agents across tasks / PnL dashboard)
Phase 2  Market Decision Observability (MarketDecisionTrace / NoTradeReason / failed-invest L4.E / market budget burn report)
Phase 3  Persistent Multi-LLM Swarm (heterogeneous agents / persistent role state / opportunity-driven scheduler / peer verification)
Phase 4  Emergent Market Arena (long/short positions / price feedback observe-only / role classifier / autopsy / market smoke)
Phase 5  Run6 structural equivalence (many tx / multiple roles / persistent market / buy/sell/challenge/verify / PnL changes / bankruptcy/autopsy)
Phase 6  Scaled Formal Benchmark (MiniF2F 50/100+ / market-enabled vs disabled A/B / solved rate × pput × market activity × role diversity × PnL × audit failures)
Phase 7  Real-world domain design (only after formal arena stable)
```

---

## 10. AI Coder 直接执行口令 (verbatim)

```text
Architect verdict:

TB-N3 substrate is accepted as landed, but Phase 2 proves
generative market activity is zero.

Do not add more CPMM mechanics now.
Do not force agents to invest by prompt command.

Start Generative Phase G.

G1 Cross-Problem Persistence:
- one batch = one runtime_repo + one CAS + one continuous HEAD_t
- no per-problem genesis reset
- agents persist balance, reputation, positions, PnL
- problem k+1 starts from problem k final state
- dashboard shows cross-problem PnL

G2 MarketDecisionTrace + NoTradeReason:
- every market-visible turn records invest tx or no-trade reason
- failed invest goes L4.E
- classify NO_POOL / NO_PROMPT_TOOL / NO_PARSED_INVEST / AGENT_DECLINED / etc.

G3 PnL / Solvency:
- wrong market decisions affect future agent state
- bankruptcy/autopsy possible
- PnL in prompt context

G4 Multi-LLM Mix:
- at least 3 model families
- persistent model-agent identity
- compare market behavior by model

G5 Role Differentiation:
- classify Solver / Bull / Bear / Verifier / Challenger / Observer
- role classifier from ChainTape + CAS only

G6 Price Feedback Observe-only:
- price enters prompt/read-view
- no sequencer enforcement
- price never changes predicate

G7 Run6-equivalent Structural Smoke:
- not exact 1748 tx target
- require structural evidence:
  buy yes, buy no, verify, challenge, payout, autopsy, PnL divergence, role differentiation.

Keep constitutional gates:
- tape-first
- no ghost liquidity
- no price-as-truth
- dashboard materialized view only
- no real funds
- no public chain
```

---

## 11. 最终判断 (架构师独立意见)

```text
TuringOS 的防御宪法阶段基本成功；
现在必须进入生成性市场阶段。
```

不是放松宪法，而是把宪法变成竞技场边界.

远景不是「合规但无人交易的账本」，而是「可审计、可结算、可竞争、可学习的智能市场」.

- Hayek: 价格必须传递分散知识
- Nakamoto: 激励必须长期绑定身份和资本
- Turing: 行动必须发生在 tape 上
- Drucker: 管理 PnL/角色/目标，不只管理 proof success

> 让 agent 在同一条 tape 上持续生活、赚钱、亏损、学习、分化.

---

## Authorization Status

**Archived, NOT yet executing.**

Per `/architect-ingest` step 4: present analysis, await explicit user approval before Class-4 atom landing.

Authorization scope when granted will need to name:
- which G atom (G0 / G1 / ... / G7) is authorized
- whether per-atom §8 is pre-committed or per-atom required
- whether dual audit (Codex G2 + Gemini DeepThink) timing is PRE-§8 or POST-§8
- ship gates for that atom
- rollback path

Authorization is **pending the remote ultraplan session** at
https://claude.ai/code/session_01QqSehGhpsts18AC5qExyAS?from=cli
which will surface the full development / validation / alignment audit plan
at phase / module / atom three-layer granularity.
