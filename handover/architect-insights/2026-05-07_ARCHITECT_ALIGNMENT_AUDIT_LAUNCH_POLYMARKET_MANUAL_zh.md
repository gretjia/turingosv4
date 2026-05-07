<!--
  Archive header — added by /architect-ingest 2026-05-07.
  ===================================================================
  Source           : architect message, 2026-05-07
  Companion file   : 2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_en.md
  Status           : ARCHIVED — top-tier architect alignment document.
  Authorization    : NOT YET AUTHORIZED for execution. 接收指令 ≠ 授权执行.
                     Action items inside (TB-18R Final closure, AMBER-row
                     closure, HEAD_t C2 design, CompleteSetMergeTx, CPMM
                     LiquidityPool, Mint-and-Swap Router, restricted beta,
                     etc.) require explicit Class-3 / Class-4 ratification
                     per CLAUDE.md §10 before any TB charter executes them.
  Layer-1 impact   : None detected.
                     - kernel.rs zero-domain-knowledge: not amended
                     - Append-Only DAG: not amended
                     - Economic conservation: reaffirmed, not amended
                     - Constitution clauses: no new clauses; restates
                       existing FC1/FC2/FC3, Art. III shielding, CTF
                       invariant `1 Coin = 1 YES + 1 NO`, no-f64-money,
                       no-ghost-liquidity, predicate-as-truth.
  Operating-mode   : Reaffirms Constitutional Harness Engineering as
                     Prime Operating Mode (CLAUDE.md §2). No deviation.
  Freeze-status    : Document instructs continued freeze on public
                     benchmark claim, real-world readiness, public chain,
                     real-money market, NodeMarket trading, AMM/CPMM
                     router, formal H-VPPU claim — until critical gates
                     are green. This is consistent with current
                     `MEMORY.md` / TB-C0 freeze-lift carve-outs.
  Intended use     : Reference roadmap & Polymarket/CPMM AI-coder spec.
                     Future TB charters that touch market mechanics
                     should cite this document (and its English twin)
                     in their `roadmap_exit_criteria_addressed` field.
  Companion records: companion English doc is the canonical engineering
                     spec for AI coders; this Chinese doc is the
                     architect's narrative-style alignment view.
                     Both are kept verbatim per Kolmogorov-compression
                     rule (`feedback_kolmogorov_compression`).
  ===================================================================
-->

# 1. 当前工作审计结论

## 1.1 目前状态比上一阶段明显健康

我接受以下当前状态判断：

```text
workspace tests: 1181 / 0 / 151
constitution gates: 97 / 0 / 1
matrix RED rows: 0
matrix AMBER rows: 13
```

你提供的最新现状文档也记录了这个 snapshot，并明确当前分析边界是从 commit `feec129` 后的 constitution landing first / Wave-3 binding 状态出发。

这说明项目已经不是之前那种"宪法主要靠审计口头对齐"的状态。现在至少有一套可执行 constitutional harness，而且 gate 数量正在增长。

我也认可目前 `CLAUDE.md` 的方向：它已经把 **Constitutional Harness Engineering** 设为 Prime Operating Mode，要求：

```text
1. Constitutional harness as executable tests
2. Minimal real run that exercises tape
3. External audit only after evidence passes
4. Documentation packages proof, never substitutes for tape
```

这正是我们前面讨论的策略重置。

---

## 1.2 但是：宪法还没有 100% 物理落地

虽然状态已经明显改善，但还不能说"宪法完全落地"。

原因：

```text
1. 仍有 13 个 AMBER。
2. C2 Git refs 还未完成。
3. Polymarket / CompleteSet / CPMM 仍未完全进入动态闭环。
4. Art. III shielding 虽有进展，但还需要更强的直接 evidence binding。
5. 真实大规模 benchmark 还没有完全成为正式发布级证据。
```

尤其是 HEAD_t 现在是 **C1 完成**，不是全部完成。你给出的最新核对说：底层确实已接入 git2-rs / libgit2，L4 账本通过 `Git2LedgerWriter` 写真实 Git commit 到 `refs/transitions/main`，`advance_head_t()` 会用真实 commit OID 更新 `q.head_t`。这说明 C1 是真实落地的。但 C2 仍然要把 L4、L4.E、CAS 统一为 `refs/chaintape/{l4,l4e,cas}` 这一套多 ref 生产形态。

所以我的判断是：

```text
当前状态 = Constitution Landing substantially improved
但还不是 Constitution Fully Landed
```

---

# 2. 当前不能做什么

当前仍应冻结：

```text
正式 Polymarket trading
AMM / CPMM router
public chain
real-money market
real-world tasks
public benchmark claim
formal H-VPPU claim
```

原因不是这些方向错，而是它们依赖更底层的宪法 harness、benchmark harness 和 market accounting 完整性。

`CLAUDE.md` 已经明确：没有 tape evidence 不算测试，stdout / 私有日志 / dashboard / README 都不能替代 tape。

因此，当前允许做的是：

```text
1. TB-18R final sign-off / evidence completion
2. Constitution AMBER row closure
3. 20p / 50p diagnostic benchmark
4. CompleteSet 当前实现审计
5. Polymarket implementation planning / manual
6. C2 Git ref unification design
```

---

# 3. 从现在到上线的完整 PLAN

## Stage A — Constitution Landing Closure

### A1. TB-18R Final

目标：

```text
让 TB-18R 从 candidate remediation 变成 final shipped。
```

必要证据：

```text
P38
P49
M0 mini-batch
attempt equality report
audit_tape report
dashboard regeneration report
```

验收标准：

```text
SG-A1.1 P38 attempt equality green.
SG-A1.2 P49 attempt equality green.
SG-A1.3 M0 mini-batch green.
SG-A1.4 no fake accepted nodes.
SG-A1.5 every real Lean reject represented in L4.E or anchored EvidenceCapsule.
SG-A1.6 chain facts derived from ChainTape/CAS, not evaluator stdout.
SG-A1.7 final dual audit PASS under VETO > CHALLENGE > PASS.
```

---

### A2. Wave 1 / Wave 2 / AMBER closure

目标：

```text
关闭剩余 AMBER，尤其是 Art. 0 / Art. I.2 / Art. II / Art. III / Art. IV.boot。
```

验收标准：

```text
SG-A2.1 constitution gates >= current 97 and no regression.
SG-A2.2 all new gate files included in scripts/run_constitution_gates.sh.
SG-A2.3 every matrix promotion has a real witness.
SG-A2.4 no doc-only GREEN promotion.
```

---

### A3. HEAD_t C2

目标：

```text
从当前 C1 的真实 libgit2 L4 commit chain，升级为 C2 的多 ref ChainTape。
```

需要：

```text
refs/chaintape/l4
refs/chaintape/l4e
refs/chaintape/cas
```

验收标准：

```text
SG-A3.1 L4 head ref advances on accepted transition.
SG-A3.2 L4.E head ref advances on rejected evidence.
SG-A3.3 CAS root ref advances when CAS evidence added.
SG-A3.4 replay reconstructs HEAD_t from refs.
SG-A3.5 no hidden filesystem pointer.
```

---

## Stage B — Formal Benchmark Scale-Up

### B1. 20p diagnostic

```text
20 real MiniF2F / Lean problems
chain-backed
no public claim
no market
```

验收：

```text
all runs chain-backed
all failures have L4.E or EvidenceCapsule
dashboard regenerates
BenchmarkManifest exists
```

### B2. 50p controlled benchmark

```text
50 problems
n1 / n3
chain-backed
```

验收：

```text
50/50 chain_invariant reports exist
aggregate report not dependent on stdout
PPUT discipline green
no hidden excluded runs
```

### B3. 100p / M2 benchmark

只在 B1/B2 green 后进行。

验收：

```text
sampled full replay works
failure-heavy sample replay works
solved sample replay works
unsolved sample replay works
EvidencePackagingPolicy satisfied
no public claim unless audit-approved
```

---

## Stage C — Polymarket / RSP-M

这部分下面单独给详细说明书。

先后顺序：

```text
C1 Polymarket Manual / DECISION records
C2 CompleteSet audit hardening
C3 CompleteSetMergeTx
C4 ShareBalances export
C5 MarketSeed sanity + collateral accounting
C6 CPMM LiquidityPool + Mint-and-Swap Router
C7 controlled market smoke
C8 restricted beta market
```

---

## Stage D — Real-world readiness

真实世界任务仍然不能马上开始。

必须先完成：

```text
REAL_WORLD_READINESS_REPORT
DOMAIN_SELECTION_CRITERIA
ORACLE_REQUIREMENTS
CHALLENGE_COURT_REQUIREMENTS
SAFETY_BOUNDARY
IRREVERSIBLE_ACTION_POLICY
```

并且必须保证：

```text
no real-world domain without oracle
no subjective task without predicate plan
no irreversible external action
no settlement before challenge window
no price-as-truth
human escalation for high-risk domains
```

---

# 4. 每个关键位置的验收标准

## 4.1 Evaluator

必须做到：

```text
每个 externalized LLM-Lean cycle 都产生 AttemptTelemetry。
每次 Lean check 都产生 LeanResult。
Pass / Fail 必须路由到 L4 / L4.E 或被 EvidenceCapsule anchor。
ChainDerivedRunFacts 来自 ChainTape + CAS，而不是 stdout。
```

Halt：

```text
N attempts collapse into 1 WorkTx
Lean reject only in stdout
dashboard needs stdout for core facts
```

---

## 4.2 Sequencer

必须做到：

```text
system-only tx 不可由 agent submit。
predicate pass 才能 L4。
predicate fail 进入 L4.E。
HEAD_t 正确推进。
经济守恒。
```

Halt：

```text
system tx accepted from agent ingress
post-init mint
predicate failure enters L4
```

---

## 4.3 CAS

必须存储：

```text
PromptCapsule
AttemptTelemetry
LeanResult
EvidenceCapsule
MarkovEvidenceCapsule
proof artifacts
raw diagnostic audit-only data
```

Halt：

```text
CID exists but payload missing
CAS race
unresolvable evidence used as proof
```

---

## 4.4 Dashboard

Dashboard 必须是：

```text
materialized view only
```

必须可删除重建：

```text
delete dashboard
regenerate from ChainTape + CAS
same result
```

Halt：

```text
dashboard is source-of-truth
dashboard requires evaluator stdout
dashboard leaks raw private diagnostic
```

---

## 4.5 Market / Polymarket

必须满足：

```text
integer / rational math only
no f64 money path
no ghost liquidity
no automatic YES/NO injection
shares are claims, not Coin
collateral is Coin holding
price cannot override predicate
```

Halt：

```text
liquidity created without collateral
YES/NO shares counted as Coin
price-as-truth
f64 in market path
```

---

# 5. Polymarket / CPMM 给 AI coder 的完整说明书

下面这部分是 AI coder 可以直接照着执行的实现手册。

---

## 5.1 核心原则

Polymarket / CTF 核心：

```text
1 locked Coin = 1 YES_E + 1 NO_E
```

含义：

```text
YES/NO 是 claim，不是 Coin。
locked collateral 是 Coin holding。
YES/NO shares 不进入 total_coin_supply。
```

TuringOS 中的映射：

```text
WorkTx.stake      -> FirstLong exposure
ChallengeTx.stake -> Short / NO exposure
VerifyTx.bond     -> responsibility bond, not market position
```

---

## 5.2 先隔离 legacy f64 CPMM

在任何新 market 代码前，必须确保：

```text
src/prediction_market.rs legacy f64 CPMM 不被新代码 import。
```

测试：

```text
legacy_cpm_api_not_imported_by_new_market
no_f64_in_market_modules
```

这是因为你给的最新核对已经指出，legacy f64 CPMM 已经作为 OBS 被跟踪，不能让它污染新 Polymarket 路径。

---

## 5.3 CompleteSet hardening

当前核对显示 `CompleteSetMintTx` / `CompleteSetRedeemTx` 已经落地。
但需要 hardening。

### Mint

逻辑：

```text
balances_t[owner] -= amount
conditional_collateral_t[event_id] += amount
share_balance[(owner,event,YES)] += amount
share_balance[(owner,event,NO)] += amount
```

测试：

```text
mint_one_coin_creates_one_yes_one_no
mint_conserves_total_coin
shares_not_counted_as_coin
```

### Redeem

逻辑：

```text
requires resolved outcome

if outcome = YES:
  burn YES shares
  owner balance += share_amount
  collateral -= share_amount
  NO receives 0

if outcome = NO:
  burn NO shares
  owner balance += share_amount
  collateral -= share_amount
  YES receives 0
```

测试：

```text
redeem_unavailable_before_resolution
redeem_yes_after_yes_pays_yes_not_no
redeem_no_after_no_pays_no_not_yes
redeem_cannot_exceed_share_balance
redeem_debits_collateral
```

---

## 5.4 CompleteSetMergeTx

这是目前仍缺失、但完整市场需要的模块。

目标：

```text
1 YES + 1 NO -> 1 Coin
```

用途：

```text
允许未结算前撤回成对风险敞口。
```

结构：

```rust
pub struct CompleteSetMergeTx {
    pub tx_id: TxId,
    pub parent_state_root: Hash,
    pub event_id: EventId,
    pub owner: AgentId,
    pub amount: ShareAmount,
    pub signature: AgentSignature,
}
```

语义：

```text
require owner YES >= amount
require owner NO >= amount

burn amount YES
burn amount NO
conditional_collateral_t[event] -= amount
balances_t[owner] += amount Coin
```

测试：

```text
merge_yes_no_returns_coin
merge_requires_both_sides
merge_conserves_total_coin
merge_reduces_collateral
```

---

## 5.5 MarketSeedTx

`MarketSeedTx` 必须显式抵押，不能自动注入。

语义：

```text
provider deposits seedC Coin
CompleteSetMintTx-like operation creates seedC YES + seedC NO
YES/NO shares go to pool inventory
collateral locks seedC
```

测试：

```text
market_seed_debits_provider
market_seed_creates_yes_no_inventory
market_seed_fails_insufficient_balance
market_seed_no_ghost_liquidity
market_seed_conserves_total_coin
```

禁止：

```text
每个新 node 自动凭空获得 100 YES + 100 NO
```

如果要"每个 node 初始做市"，只能这样：

```text
on_init 预设 MarketMakerBudget
MarketSeedTx 每次从 budget debit
budget 不足则 seed fail
```

---

## 5.6 LiquidityPool / CPMM

新增池状态：

```rust
pub struct CpmmPool {
    pub event_id: EventId,
    pub pool_yes: ShareAmount,
    pub pool_no: ShareAmount,
    pub lp_total_shares: LpShareAmount,
    pub status: PoolStatus,
}
```

规则：

```text
pool_yes / pool_no 是 pool 持有的 YES/NO shares。
pool reserves 不是 Coin。
lp shares 不是 Coin。
k = pool_yes * pool_no。
```

测试：

```text
pool_created_from_seed_inventory
pool_reserves_not_counted_as_coin
lp_shares_not_counted_as_coin
pool_cannot_exist_without_collateralized_shares
```

---

## 5.7 YES/NO share swap

先实现 share-only swap，再做 Coin router。

### 用 NO 买 YES

输入：

```text
dN > 0
```

公式：

```text
outY = floor(dN * poolY / (poolN + dN))
poolN1 = poolN + dN
poolY1 = poolY - outY
```

整数不变量：

```text
poolY1 * poolN1 >= poolY * poolN
```

因为 floor rounding 会把 dust 留在池子里。

### 用 YES 买 NO

```text
outN = floor(dY * poolN / (poolY + dY))
poolY1 = poolY + dY
poolN1 = poolN - outN
```

测试：

```text
swap_no_for_yes_constant_product_non_decreasing
swap_yes_for_no_constant_product_non_decreasing
swap_fails_zero_input
swap_fails_insufficient_pool_output
swap_respects_min_out_slippage
swap_uses_integer_math_no_f64
```

---

## 5.8 Mint-and-Swap Router

这是你给的架构师公式的正式工程化版本。

### BuyYesWithCoinRouter

用户支付：

```text
payC Coin
```

原子步骤：

```text
1. debit buyer Coin by payC
2. lock payC collateral
3. mint payC YES + payC NO to router
4. transfer payC YES to buyer
5. swap payC NO into pool
6. pool receives dN = payC NO
7. router receives outY YES
8. transfer outY YES to buyer
9. buyer total getY = payC + outY
```

公式：

```text
outY = floor(payC * poolY / (poolN + payC))
getY = payC + outY
priceY = payC / getY
poolY1 = poolY - outY
poolN1 = poolN + payC
```

整数不变量：

```text
poolY1 * poolN1 >= poolY * poolN
```

### BuyNoWithCoinRouter

对称：

```text
outN = floor(payC * poolN / (poolY + payC))
getN = payC + outN
poolY1 = poolY + payC
poolN1 = poolN - outN
```

测试：

```text
buy_yes_with_coin_matches_formula
buy_no_with_coin_matches_symmetric_formula
buy_yes_debits_coin_locks_collateral
buy_yes_mints_complete_set
buy_yes_transfers_retained_yes_plus_swap_yes
buy_yes_respects_min_yes_out
buy_yes_no_f64
buy_yes_no_ghost_liquidity
router_atomic_rollback_on_failure
```

---

## 5.9 PriceIndex

Price 是 signal，不是 truth。

```text
price_yes_effective = quote_payC / quote_getY
price_no_effective  = quote_payC / quote_getN
```

测试：

```text
price_quote_does_not_change_state
price_signal_not_predicate
price_does_not_make_failed_node_accepted
low_liquidity_warning
```

---

## 5.10 Audit tools

必须增加：

```text
audit_tape view-shares
audit_tape view-pools
audit_tape view-prices
audit_tape view-positions
```

显示：

```text
owner YES/NO shares
conditional collateral
pool reserves
LP shares
NodePositions
price signal
```

测试：

```text
audit_view_shares_matches_state
audit_view_pools_matches_state
dashboard_regenerates_market_view
```

---

# 6. Polymarket 禁止清单

AI coder 必须牢记：

```text
No automatic per-node 100 YES + 100 NO without collateral.
No Treasury magic seed without debit.
No f64 money math.
No DPMM / pro-rata payout inside CTF track.
No price-based settlement.
No agent-submitted MarketResolveTx.
No agent-submitted system resolution.
No AMM before CompleteSet.
No trading before audit tools.
No public chain before sandbox.
No real money before readiness gate.
```

---

# 7. 给 AI coder 的总指令

可以直接发：

```text
Current project state:
Constitutional harness is now primary.
Do not resume old Atomic Agentic Engineering.

Immediate priorities:
1. Finish TB-18R final sign-off with current-head evidence.
2. Close remaining constitution AMBER rows.
3. Keep FC1/FC2/FC3 gates green.
4. Do not start executable Polymarket features until constitution gates and diagnostic benchmarks are stable.

When starting Polymarket:
1. Quarantine legacy f64 CPMM.
2. Harden CompleteSet Mint/Redeem.
3. Implement CompleteSetMergeTx.
4. Harden MarketSeedTx.
5. Implement integer CpmmPool.
6. Implement share-only swap.
7. Implement Mint-and-Swap router exactly as specified.
8. Add audit views.
9. Run controlled market smoke.

At every step:
- no f64,
- no ghost liquidity,
- no price-as-truth,
- no dashboard source-of-truth,
- no real funds,
- no public chain.
```

---

# 8. 最终判断

目前项目比之前健康很多：
你已经把开发哲学从 audit ceremony 拉回了 harness-first，且当前 gates 已经进入可执行层。

但项目还没有到"完整上线"状态。

当前正确路线是：

```text
Constitution Landing closure
-> Formal benchmark diagnostics
-> CompleteSet / Polymarket hardening
-> Controlled market smoke
-> restricted beta
-> real-world readiness
```

Polymarket 的设计可以吸收你给的 CPMM / Mint-and-Swap 公式，但必须严格工程化为：

```text
collateral-backed
integer math
no ghost liquidity
price-as-signal
predicate-first truth
```

这样既保留架构师的市场直觉，又不会破坏 TuringOS 宪法。
