# TB-N3-POLYMARKET-AGENT-BRIDGE — Charter (2026-05-11 session open; v3 post-architect-ruling)

> **v3 (this update, post architect ruling 2026-05-11)**: incorporates the
> verbatim architect verdict (`TuringOS_TB_N3_Polymarket_Agent_Bridge_Ruling_2026-05-11.md`).
> The ruling is **binding and supersedes v2** on the following surfaces:
> §0.6 (NEW; architect amendments), §3 (atom list — A6 excluded from core,
> A0 + A0.5 added), §6 (ship gates renumbered SG-N3.1..SG-N3.15 verbatim),
> §7 (forbidden list verbatim), §9 (architect Q1-Q8 verdicts). v2 baseline
> kept intact below for context; binding clauses are in §0.6 + §3 + §6 + §7 + §9.
>
> **Class 0 charter** drafted autonomously by Claude **and updated v2 with
> Phase 1 empirical batch evidence** (closure report at
> `handover/evidence/phase1_n5_batch_2026-05-11T03-46-20Z/PHASE_1_CLOSURE_REPORT.md`).
>
> Phase 1 evidence (9 real MiniF2F problems × n=5 multi-agent × deepseek-chat ×
> 33 min wall, HEAD `ced19363`) confirmed:
> - ✅ v4 multi-agent infrastructure complete — 5/9 SOLVED including 2 HARD
> - ✅ CPMM kernel complete (Stage C SHIPPED FINAL; §1+§2 verbatim verified)
> - ✅ TB-N2 B2 EventResolveTx **first witnessed firing** on real-LLM tape (P06)
> - ❌ Zero market activity across all 9 problems despite both sides ready:
>   `complete_set_mint=0, cpmm_pool=0, cpmm_swap=0, buy_with_coin_router=0`
>
> The gap is **wire**, not capability. This charter scopes the wire-up.
>
> User direction (2026-05-11): "v4 终极设计目的也是 multi-llm-agents，只不过
> 前期为了测试 tape，现在已经完全具备 multi-agents" — explicitly rejects the
> session #34 "(E) EMPIRICALLY CLOSED at N=1" interpretation as final
> architectural decision. The empirical closure was specific to N=1 prompt
> schema; the multi-agent + CPMM cross-wire is the next landing.
>
> Reference target: v3 `run6_90agents_6000tx_dag.md` (1748 tx, 853 BUY YES +
> 239 BUY NO, 16 roots, depth 18, OMEGA reached). v4 must produce structurally
> equivalent tape on real MiniF2F problems via real CpmmSwap / BuyWithCoinRouter
> agent-driven market dynamics.

---

## §0. Tags (mandatory per `feedback_tb_phase_tag_required`)

- **phase_id**: P3 — RSP Economy Core (Stage E post-Stage-C; agent-facing
  surface of the CPMM substrate landed in P-M0..P-M9).
- **roadmap_exit_criteria_addressed**:
  1. P3 Exit "agent-driven CPMM market dynamics on tape" — agents observe
     other agents' WorkTx and decide BUY YES / BUY NO via real
     `BuyWithCoinRouterTx`, producing whale-contested nodes + role-divergent
     trading signals (v3 run6 equivalence).
  2. P3 Exit "every proposal carries a market" — system auto-emits
     CompleteSetMintTx + CpmmPoolTx on each accepted WorkTx, so every node
     has a tradeable quality market.
  3. Stage C overall §8 §6 deferred clause closure — "agent-bridge for the
     CPMM substrate" explicitly forward-bound out of TB-N2 (§2 cuts list).
- **kill_criteria_tested**:
  1. If post-TB-N3 batch evidence on the same 9-problem set still shows
     `cpmm_pool == 0 OR buy_with_coin_router == 0 OR cpmm_swap == 0`,
     reject (clean-negative empirical reproduction).
  2. If any agent invest decision creates ghost liquidity (router accept
     without provider Coin debit OR pool reserves grow without `collateral`
     credit), reject — CLAUDE.md §13 economy law violation.
  3. If multi-agent run produces single-root single-step DAG instead of
     multi-root multi-depth citation tree, reject — emergent role
     differentiation must show on tape.
  4. If `assert_complete_set_balanced` ever fails under agent-driven swap
     load (Stage C E.3 strict-equality invariant), reject — Defect-1
     regression.
- **Authority basis**: user verbatim 2026-05-11 "我要从 tape 上看到类似
  [run6] 的完整经济学实现的证明" + "v4 终极设计目的也是 multi-llm-agents".
  Class-0 charter authority autonomous; Class-3+ atoms require fresh
  authorization; Class-4 atoms (A3 sequencer auto-emit) require per-atom
  architect §8 per `feedback_no_batch_class4_signoff`.

## §0.6. Architect ruling 2026-05-11 — binding amendments (v3)

Source: `TuringOS_TB_N3_Polymarket_Agent_Bridge_Ruling_2026-05-11.md` (architect
sandbox doc). Verdict: "TB-N3 可以继续，但必须先修 charter." Five binding
amendments + Q1-Q8 verdicts + SG-N3.1..15 + forbidden list. The amendments
override v2 §3 / §6 / §9 wherever they conflict.

### Amendment 1 — `event_id` MUST be `node_survive(work_tx.tx_id)`, NOT `task_id`

> 错误 (using `task_id`)，必须改 → 正确：`event_id = node_survive(work_tx.tx_id)`
> or `event_id = EventId("node_survive:<accepted_work_tx_id>")`. One accepted
> WorkTx → one node event → at most one CPMM pool. `task_id` only as metadata,
> not as node-market identity.

Implementation: `EventId(pub TaskId)` is the existing newtype at
`src/state/typed_tx.rs:1166`; `TaskId` is a `pub String`. Namespace by
encoding the `work_tx_id` inside the TaskId payload — no new enum variant
(architect-defended `event_id_kind` defect prevention preserved per Stage C
P-M4 §8 precedent). New pure constructor at `src/state/typed_tx.rs` after
EventId definition:

```rust
pub fn node_survive_event_id(work_tx_id: &TxId) -> EventId {
    EventId(crate::state::q_state::TaskId(format!("node_survive:{}", work_tx_id.0)))
}
```

### Amendment 2 — A2 deterministic-fixture only; live router smoke moves to A3-+

> A2 真正的 real-LLM market smoke 应放在 A3 之后. A2 exit改为：
> 1. deterministic fixture: parsed invest + existing pool → BuyWithCoinRouterTx accepted
> 2. negative fixture: parsed invest + missing pool → L4.E `RouterPoolNotActive`

A2 can NOT require `buy_with_coin_router >= 1` from a live LLM run because
no auto-pool exists yet. v2 §6 SG-N3.1 (live-LLM smoke ≥ 1) is replaced by
the v3 SG-N3.1 + SG-N3.2 fixture-only gates.

### Amendment 3 — A3 uses `MarketSeedTx` via canonical signing path

> A3 应优先用 `MarketSeedTx` (architect §3.3). Even if internal kernel surfaces
> need `CompleteSetMintTx + CpmmPoolTx`, must go through canonical admission;
> never hand-write treasury debit / pool reserve.

Implementation: A3 calls `make_real_market_seed_signed_by(provider="MarketMakerBudget")`
followed by `make_real_cpmm_pool_signed_by` (new helper). Both signed by
the genesis-preseeded `MarketMakerBudget` agent — not synthesized.
`MarketMakerBudget` MUST exist at genesis (closure: amendment 5 below).

### Amendment 4 — A3 fires only on accepted `WorkTx`, never on L4.E

> 只对 accepted WorkTx 触发. 不对 L4.E rejection / parse_fail / Lean fail /
> step_reject / sorry block / raw externalized attempt 触发. accepted state
> node 才是可交易 node. 失败 attempt 是 evidence，不是 node market.

Implementation: A3 emit helper inspects `transition_ledger.entries[..].tx_id`
and only proceeds when `work_tx_id` is L4-accepted. L4.E and CAS-only attempts
return `NodeMarketEmitOutcome::WorkNotAccepted`.

### Amendment 5 — Same-task-only market context in prompt; A6 excluded from N3 core

> Prompt 默认只展示 same task accepted WorkTx + active pool. cross-task market
> 会增加上下文污染和误导.
>
> A6 (sandbox finalize) 不属于 TB-N3 core. 如果做，sandbox synthesis only;
> 不要让 A6 阻塞 TB-N3 market bridge.

Implementation: `render_market_context(q, task_id, k, viewer)` filters to
events with `event_id.0.0` matching `node_survive:*` AND underlying WorkTx
having `task_id == task_id` (the prompt's task). A6 deferred to a forward TB.

### Amendment 6 — Insert atom A0.5 (MarketMakerBudget precondition)

> A3 不能开始，直到 MarketMakerBudget 已验证存在. 如果没有：新增 A0.5
> MarketMakerBudget precondition atom. 改 on_init / genesis schema = Class 4.

Implementation: A0.5 appends `(AgentId("MarketMakerBudget"), MicroCoin::from_micro_units(5_000_000))`
to `default_pput_preseed_pairs()`. Class-4 STEP_B per amendment 6.

### Architect Q1-Q8 verdicts (verbatim)

| Q | Verdict | Implementation binding |
|---|---------|------------------------|
| Q1 | DEFAULT_POOL_SEED = 100_000 microCoin; configurable via `TURINGOS_AUTO_MARKET_SEED_MICRO`; budget ≥ 5_000_000 microCoin (10× safety). | A3 reads env default 100_000; A0.5 budgets 5M (5×10⁶ micro = 5 Coin). |
| Q2 | A3 cannot start until MarketMakerBudget verified. Genesis change = Class 4. | A0.5 STEP_B; bootstrap.rs preseed list + tests + Trust Root manifest rehash. |
| Q3 | V0+V1 default both include `invest`. Opt-out via `TURINGOS_DISABLE_MARKET_TOOLS=1`. | A1 keeps invest in V0 (already), restores invest in V1. |
| Q4 | A6 non-core. If done, synthesis route (Class 2-3). NOT SystemEmitCommand variant. | A6 excluded from N3; deferred to forward TB. |
| Q5 | min_out_shares = 0 for MVP; parser may allow future field. Router-output-zero MUST L4.E. | A2 hard-codes 0; rejection class taxonomy preserved. |
| Q6 | A3 fires only on accepted WorkTx (not L4.E / parse_fail / lean_fail / step_reject / sorry). | Amendment 4 binding. |
| Q7 | K = 10 default; K = 5 if context tight; same-task only; sort: same-task → accepted → active pool → liquidity → recency. Prompt MUST say "price is signal, not truth". | A4 default 10 (env override `TURINGOS_TB_N3_MARKET_CONTEXT_K`). |
| Q8 | Same 9 problems first. solved < 4/9 → 3-seed diagnostic before VETO (stochastic vs market-distraction). | Phase 2 batch script clones tbc0 9-problem set; auto 3-seed fallback if < 4/9. |

### Architect §8 better suggestions (non-blocking; folded into atoms)

- §8.1 `MarketDecisionTrace` CAS object per agent invest intent — landed in
  A2 module `src/runtime/market_decision_trace.rs`.
- §8.2 No-trade reason classification — `NoTradeReason` enum within A2;
  aggregate render in A5 §F.
- §8.3 Don't hard-induce trading; prompt copy must be permissive
  ("you may invest if market signal provides an advantage").
- §8.4 Same-task isolation default — amendment 5 binding.
- §8.5 Market budget burn report — A5 §E (pools_created / market_seed_total /
  treasury_budget_start / treasury_budget_end / pools_skipped_budget).
- §8.6 Failed invest is meaningful tape activity (L4.E + run_report inclusion).

## §0.5. Phase 1 empirical baseline (2026-05-11; binding for kill criteria)

Source: `handover/evidence/phase1_n5_batch_2026-05-11T03-46-20Z/` — 9-problem
batch × n=5 × deepseek-chat × no src/ changes (HEAD `ced19363`).

### Solve performance baseline

| Problem | Diff | tx | Wall | Halt | Solved | step_partial_ok |
|---------|------|----|----|------|--------|-----------------|
| P01 mathd_algebra_107 | easy | 1 | 21s | OmegaAccepted | ✅ | 0 |
| P02 mathd_algebra_125 | easy | 1 | 21s | OmegaAccepted | ✅ | 0 |
| P03 mathd_algebra_141 | easy | 1 | 11s | OmegaAccepted | ✅ | 0 |
| P04 mathd_algebra_113 | medium | 20 | 184s | MaxTxExhausted | ❌ | 0 |
| P05 mathd_algebra_114 | medium | 20 | 217s | MaxTxExhausted | ❌ | 12 |
| P06 mathd_numbertheory_1124 | **hard** | 2 | 19s | OmegaAccepted | ✅ | 0 |
| P07 numbertheory_2pownm1prime_nprime | **hard** | 39 | 447s | OmegaAccepted | ✅ | 14 |
| P08 aime_1983_p1 | hard | 50 | 513s | MaxTxExhausted | ❌ | 2 |
| P09 aime_1984_p1 | hard | 50 | 498s | MaxTxExhausted | ❌ | 4 |

**Solve rate baseline: 5 / 9 = 56%** (must not degrade > 20% post-TB-N3 → ≥ 4/9 required).
**Wall budget baseline: ~33 min** (must not exceed 4hr post-TB-N3).

### Economic-tape baseline (aggregate across 9 problems)

| TxKind | Count | Per-problem average |
|--------|-------|---------------------|
| task_open | 9 | 1.0 |
| escrow_lock | 9 | 1.0 |
| **work** (L4-accepted) | 5 | 0.56 |
| **verify** | 2 | 0.22 |
| **finalize_reward** | 2 | 0.22 |
| **event_resolve** | **1** (P06) | 0.11 |
| complete_set_mint | 0 | 0.0 |
| cpmm_pool | 0 | 0.0 |
| cpmm_swap | 0 | 0.0 |
| buy_with_coin_router | 0 | 0.0 |
| L4 total | 32 | 3.56 |
| L4.E total | 169 | 18.78 |
| CAS objects | 711 | 79.0 |

### Witnesses worth highlighting to architect

1. **P06 mathd_numbertheory_1124 — FIRST full economic-cycle on real-LLM tape**:
   - `work=1` (agent OMEGA-claim step at 19s wall)
   - `verify=1` (peer n=5 agent triggered verify_peer)
   - `finalize_reward=1` (TB-N1 phase 2 payout fired)
   - `event_resolve=1` (TB-N2 B2 system-emit fired; first ever)
   - This validates `b61735b` R2 race fix end-to-end on real-LLM 19s window.

2. **P03 mathd_algebra_141 — partial full-cycle**:
   - `work=1, verify=1, finalize_reward=1, event_resolve=0`
   - The 11s wall was sufficient for FinalizeReward apply but EventResolve
     poll budget expired before claims_t showed Finalized status.

3. **P01/P02 — 1-tx fast-solve skips entire payout path**:
   - 21s wall but verify_tx_id remained None at OMEGA-exit (peer agents
     didn't fire verify_peer in time).
   - `if let Some(vid) = verify_tx_id` guard at `evaluator.rs:2814` + `:3509`
     skips both finalize_reward and event_resolve emit.
   - This is the design rationale for atom **A6 sandbox synthesis path**.

### FC invariant baseline

- FC1: 8/9 GREEN (delta=0). One delta=+1 on P07 — bash extractor measurement
  bug (`tool_dist.step` excludes `omega_wtool` boundary case), not a kernel
  invariant violation. Forward Class-1 patch tracked as `OBS_PHASE_1_FC1_OMEGA_BOUNDARY_2026-05-11`.
- FC2: 9/9 GREEN. Replay from genesis + chain + CAS holds across all 9.
- FC3: matrix-aggregate level 162/162 GREEN; per-problem 4/5 AMBER on
  capsule-derived (5 problems didn't generate capsule due to insufficient
  attempts), 9/9 AMBER on INV3/5/7/8 (structural batch-aggregate witnesses,
  expected behavior).

### Multi-agent operation witnessed

- Per problem: 5 distinct Ed25519 pubkeys (Agent_0..Agent_4) in `agent_pubkeys.json`.
- `boltzmann_select_parent_v2` invoked; `agent_idx = tx % n_agents` rotation
  visible in agent_audit_trail.
- Fast-solve (1-tx OMEGA) problems: ONLY Agent_0 contributes; peer agents
  never engage.
- Multi-tx problems (P04/P05/P07/P08/P09): visible step_partial_ok
  accumulation across agents (P07 = 14 partial successes from multiple agents
  before final OMEGA).

### Regression observation (forward investigation)

- **P09 aime_1984_p1**: SOLVED in `tb_c0_multi_agent_2026-05-06` (89s n=5)
  but NOT solved today (50 tx exhausted, 498s). Same model + same n=5 + same
  problem. Either stochastic LLM variance OR DeepSeek snapshot drift between
  2026-05-06 and 2026-05-11. Tracked as `OBS_PHASE_1_P09_REGRESSION_2026-05-11`;
  Phase 2 re-run on multiple seeds will validate.

## §1. Mission

Close the **multi-agent CPMM cross-wire gap** witnessed in Phase 1 evidence:

```
v3 run6 (90 agents × 6000tx × 50min):
  1748 tx · 853 BUY YES + 239 BUY NO · 16 roots · max depth 18
  · OMEGA reached · role-divergent net trading (Math/Bull/Bear)

v4 Phase 1 (5 agents × 9 problems × 33min):
  32 L4 + 169 L4.E + 711 CAS · 5/9 OMEGA reached
  · complete_set_mint=0 · cpmm_pool=0 · cpmm_swap=0 · buy_with_coin_router=0
  · 1× full TB-N2 B2 EventResolve cycle (P06) · 2× verify+finalize cycles
```

The CPMM kernel is fully landed (Stage C SHIPPED FINAL; per-atom §8) and the
multi-agent evaluator is fully landed (tb_c0 + Phase 1 evidence). What is
missing is the **agent-side surface**:

1. **No automatic market per node** — agent submits WorkTx, but no
   CompleteSetMintTx + CpmmPoolTx is auto-emitted, so there is nothing for
   other agents to trade on.
2. **Agent prompt does not expose invest tool with real semantics** —
   `invest` is in V0 schema but unconditionally drops to V1 strip on
   single-LLM-N=1 evidence; multi-agent agents currently never see it.
3. **Agent prompt does not show other agents' WorkTx + current price** —
   no context for trade decisions.
4. **No run-report renderer** — even if all the above land and tape shows
   market activity, there is no `audit_view_*` projection that produces
   v3-run6-style citation tree + role activity + whale list.

Without these, v4 cannot produce a run6-equivalent tape no matter how many
agents are configured. The gap is wire, not capability.

## §2. Out-of-scope (explicit cuts)

The following are NOT in TB-N3 scope:

- **CLOB orderbook surface (architect RSP-M4)** — same cut as TB-N2 §2.
  TB-N3 lands CPMM-only agent surface. CLOB → forward separate TB.
- **K.1-K.6 Stage D real-world readiness gates** — DEFERRED behind explicit
  architect ship gate. TB-N3 is sandbox / simulation tape only; no real funds.
- **Cross-event arbitrage** — agents can invest within a single event's pool
  but NOT across events. Multi-event AMM cross-wire is forward TB scope.
- **TB-N2 LP unwind (B4) interaction** — agent-callable `CpmmLpUnwindTx` is
  TB-N2 atom B4. If B4 is shipped before TB-N3 lands, atoms A2 + A4 should
  also expose unwind tool to agent prompt. Otherwise: defer.
- **Real-time chain price oracle** — `audit_view_prices` is a derived view,
  not a real-time price feed. Agents read pool snapshots at LLM-call time;
  no streaming price subscription. Forward TB scope if needed.

## §3. Atom decomposition (sequenced; per-atom §8 for Class-4)

Six atoms ordered by dependency. **No batch §8** per `feedback_no_batch_class4_signoff`.

### A1 — Agent prompt schema expose CPMM tools (Class 2)

**Goal**: V0+ prompt schema unconditionally includes:
- `{"tool":"invest","node":"<work_tx_id>","amount":<microCoin>,"direction":"long|short"}`
- (optional A6) `{"tool":"complete_set_mint","amount":<microCoin>}`
- (optional A6) `{"tool":"cpmm_pool_create","event_id":"<event_id>","seed":<microCoin>}`

**Code surface**: `src/sdk/prompt.rs::build_agent_prompt` — flip V1 schema to
**re-include** invest tool (revert session #34 strip). Add explicit
`turingosv4.agent_sig.<purpose>.v1` domain-prefix examples.

**Predicate**: parser at `src/sdk/protocol.rs::parse_agent_output` recognizes
`invest` shape; rejects malformed via existing parser error path. No new
TxKind added (existing BuyWithCoinRouter is the underlying tx).

**Test (Class 1 + Class 2)**:
- `tests/constitution_tb_n3_a1_prompt_schema.rs::v0_prompt_includes_invest_tool`
- `tests/constitution_tb_n3_a1_prompt_schema.rs::parsed_invest_long_routes_to_buyyes_router_intent`
- `tests/constitution_tb_n3_a1_prompt_schema.rs::parsed_invest_short_routes_to_buyno_router_intent`

**Exit**: V0 prompt regression test passes; existing V1 variant honored as
explicit opt-out for evaluator config (not default).

### A2 — Agent invest tool ingress routing (Class 2-3)

**Goal**: Sequencer agent ingress accepts parsed-invest payload → constructs
`BuyWithCoinRouterTx` with `buyer = parsed_agent_id`, `event_id = derived
from node`, `pay_coin = MicroCoin::from_micro_units(amount)`, `direction =
{long→BuyYes, short→BuyNo}`, `min_out_shares = 0` (slippage disabled in MVP).

**Code surface**: `experiments/minif2f_v4/src/bin/evaluator.rs` per-tactic loop
+ `src/runtime/adapter.rs` — add `parse_agent_invest_payload` → submit
`TypedTx::BuyWithCoinRouter` through `bus.submit_typed_tx`. Reuses
**ALL of P-M6 Stage C admission** (no kernel change; pure wiring).

**Predicate**: existing P-M6 router admission gates (event Open, buyer
balance, pool exists, out>0, slippage). Failing invest → routes to L4.E
(SubmissionRejected{RouterInsufficientCoinBalance / RouterSwapInsufficient
PoolOutput / etc.}) per existing rejection class taxonomy.

**Test (Class 2)**:
- `tests/constitution_tb_n3_a2_invest_ingress.rs::happy_path_buyyes_emits_buy_with_coin_router`
- `tests/constitution_tb_n3_a2_invest_ingress.rs::insufficient_balance_routes_to_l4e`
- `tests/constitution_tb_n3_a2_invest_ingress.rs::missing_pool_routes_to_l4e_router_pool_not_active`
- **Real-LLM smoke target (Phase 1 informed)**: P07 `numbertheory_2pownm1prime_nprime`
  is the IDEAL smoke target — 39 tx solved at n=5 means peer agents have
  multiple WorkTx-accept windows where invest decisions are meaningful.
  P01-P03/P06 are 1-2 tx fast-solve → no time window for invest. P04/P05
  (medium MaxTx) are NOT solved but have 20 tx of partial activity → also good
  smoke targets. Smoke acceptance: `buy_with_coin_router >= 1` on tape.

**Exit**: smoke shows `buy_with_coin_router >= 1` on real MiniF2F problem;
`assert_total_ctf_conserved` holds; `assert_complete_set_balanced` holds.

### A3 — System auto-emit market per accepted WorkTx (Class 4 STEP_B; per-atom §8)

**Goal**: Every accepted WorkTx triggers system emit of:
1. `CompleteSetMintTx{owner=Treasury, amount=DEFAULT_POOL_SEED}` to mint a
   pool seed against treasury collateral budget.
2. `CpmmPoolTx{provider=Treasury, event_id=work_tx.task_id, seed_yes=seed_no=DEFAULT_POOL_SEED}`
   to create the pool.

**Code surface (Class 4 STEP_B)**: `src/state/sequencer.rs` — `WorkTx`
admission arm post-mutate step emits two follow-on system tx via
`SystemEmitCommand::CompleteSetMint + CpmmPoolCreate`. May require new
`SystemEmitCommand` variants. **Trust Root rehash required** (sequencer +
typed_tx if new variants).

**Predicate**: must respect `MarketMakerBudget` allocated at `on_init` (NO
ghost liquidity); pool seed comes from Treasury debit. If budget insufficient,
emit fails-closed (WorkTx still accepted but no pool created — agents cannot
invest on this node).

**Test (Class 1 + Class 4)**:
- `tests/constitution_tb_n3_a3_auto_market.rs::work_tx_accept_emits_complete_set_mint_and_pool`
- `tests/constitution_tb_n3_a3_auto_market.rs::treasury_budget_insufficient_skips_pool_emit`
- `tests/constitution_tb_n3_a3_auto_market.rs::auto_emit_preserves_total_ctf_conserved`
- Stage C-style verbatim binding gate for new SystemEmitCommand variants.

**Exit**: real-LLM smoke shows `cpmm_pool >= work_tx_count` (every accepted
WorkTx gets a pool); monetary invariants hold.

**Class-4 §8 packet required**: Codex G2 + Gemini DeepThink PRE-§8 dual audit
per `feedback_dual_audit` Class-4 timing rule.

### A4 — Agent prompt UI: nearby WorkTx + CPMM prices (Class 2)

**Goal**: `build_agent_prompt` includes a new context block:

```
=== Other agents' recent proposals (with CPMM prices) ===
- work_tx_id=W001 by Agent_2 (depth 3): pool_yes=4_000_000 pool_no=4_000_000
    price_yes=0.5 price_no=0.5 (signal only; not truth)
- work_tx_id=W002 by Agent_4 (depth 2): pool_yes=2_500_000 pool_no=5_500_000
    price_yes=0.69 price_no=0.31 (BULL)
...
```

**Code surface**: `src/sdk/prompt.rs::build_agent_prompt` — add
`recent_work_tx_with_prices` parameter; render top-K (default K=10) WorkTx
sorted by recency / pool depth. Read from
`audit_views::audit_view_prices(econ, K)` (existing).

**Predicate**: `assert_no_metric_leak` extends to ensure private metrics
(reputation / classifier_version / golden_path) NOT in this view.

**Test (Class 1)**:
- `tests/constitution_tb_n3_a4_prompt_context.rs::prompt_includes_nearby_work_tx_with_prices`
- `tests/constitution_tb_n3_a4_prompt_context.rs::prompt_does_not_leak_private_metrics`
- `tests/constitution_tb_n3_a4_prompt_context.rs::prompt_orders_by_recency_and_pool_depth`

**Exit**: real-LLM smoke shows agent decisions reference prices in their
free-text reasoning (parser-side optional; tape-side mandatory).

### A5 — Run-report renderer: citation tree + role activity + whale list (Class 1)

**Goal**: New binary `src/bin/run_report.rs` reads
`runtime_repo/refs/chaintape/l4` + cas + audit_views, produces
v3-run6-style markdown:

```
# TuringOS v4 — Run X: K Agents × N tx (Model)

**M nodes | T tx | TR traded | TU untraded | OMEGA reached ✓/✗**
...
## Citation Tree
ROOT (M nodes, TR traded, TU untraded)
├── tx_1_by_2 (Agent_2/M) [BULL 5000Y B=3 ⚠W] ●
│   └── ...
## Golden Path (X steps → OMEGA)
...
## Role Activity Breakdown
| Role | Agents | Nodes Created | % |
...
## Top Contested Nodes
| Node | YES | NO | Bets | Winner |
...
## Whale Nodes (>1000C)
| Node | YES | NO | Bets | Total | Author |
```

**Code surface**: `src/bin/run_report.rs` + helpers in
`src/runtime/run_report.rs` (new module). Pure read-only over ChainTape +
CAS + audit_views. NO state mutation.

**Predicate**: dashboard-regeneratable invariant — running the binary twice
on the same evidence dir produces byte-identical output (after sorting
unstable structures).

**Test (Class 1)**:
- `tests/constitution_tb_n3_a5_run_report.rs::run_report_byte_deterministic`
- `tests/constitution_tb_n3_a5_run_report.rs::run_report_renders_citation_tree`
- `tests/constitution_tb_n3_a5_run_report.rs::run_report_detects_golden_path`

**Exit**: run on Phase 1 evidence (already produced) renders a meaningful
report (even if market section is empty — that's the baseline that A1+A2+A3
fill in).

### A6 — Optional: FinalizeReward auto-emit on OMEGA without verify (Class 3)

**Goal**: For sandbox runs (TURINGOS_SANDBOX_MODE=1), system auto-emits
FinalizeReward + EventResolve on OMEGA-Confirm even without a preceding
VerifyTx. Forensic from this session showed P01-P03 OMEGA-in-1-tx never
triggered finalize because `verify_tx_id == None`.

**Code surface**: `experiments/minif2f_v4/src/bin/evaluator.rs::2814` —
when `verify_tx_id == None` AND `TURINGOS_SANDBOX_MODE=1`, synthesize a
self-confirm `VerifyTx{verdict=Confirm, bond_micro=0}` from the OMEGA-claimant
agent OR add a new `SystemEmitCommand::FinalizeRewardSandbox{work_tx_id}`
that bypasses claims_t lookup (treats omega_wtool_id as the claim).

**Class judgment**: if synthesis route → Class 2-3 (no new TypedTx variant);
if SystemEmitCommand variant → Class 4 STEP_B (touches sequencer
admission semantics).

**Test**:
- `tests/constitution_tb_n3_a6_sandbox_finalize.rs::omega_in_1_tx_emits_finalize_reward_in_sandbox`
- `tests/constitution_tb_n3_a6_sandbox_finalize.rs::sandbox_off_no_synthesis`

**Exit**: real-LLM smoke on mathd_algebra_107 n=5 shows
`finalize_reward == 1` AND `event_resolve == 1` even on 1-tx OMEGA path.

**Class-4 §8 packet required IF sequencer variant route chosen**.

## §4. Sequencing + ship gates

```
Step 1: A1 (Class 2 prompt schema) ────────────────────► smoke 1-problem
Step 2: A2 (Class 2-3 ingress wire) ───────────────────► smoke 1-problem
   GATE: smoke shows buy_with_coin_router >= 1
Step 3: A3 (Class 4 STEP_B sequencer auto-emit) ───────► PRE-§8 dual audit
   GATE: Codex G2 PASS + Gemini PASS → architect §8 → ship
Step 4: A4 (Class 2 prompt UI) ────────────────────────► smoke 1-problem
Step 5: A5 (Class 1 renderer) ─────────────────────────► byte-deterministic test
Step 6: A6 (Class 2-3 OR Class 4 §8) ──────────────────► smoke + (if Class-4) PRE-§8
Step 7: BATCH PHASE 2 — re-run 9-problem batch ────────► COMPARISON tape
   verdict.json on each problem: cpmm_pool > 0, buy_with_coin_router > 0,
   cpmm_swap >= 0 (depends on agent dynamics), finalize_reward >= 1,
   event_resolve >= 1
Step 8: A5 renderer on Phase 2 evidence ───────────────► v3-run6-equivalent
   markdown produced; user verifies "完整经济学实现的证明"
Step 9: TB-N3 SHIPPED CANDIDATE → architect §8 sign-off
```

## §5. Functional Requirements (FR)

| ID | Requirement | Atom | Class |
|----|-------------|------|-------|
| FR-N3.1 | V0 agent prompt schema includes `invest` tool with `node/amount/direction` fields. | A1 | 2 |
| FR-N3.2 | Parsed invest payload routes to `BuyWithCoinRouterTx` via Sequencer agent ingress. | A2 | 2-3 |
| FR-N3.3 | Failed invest (insufficient balance / no pool / slippage) routes to L4.E with explicit `Router*` rejection class. | A2 | 2 |
| FR-N3.4 | Every accepted WorkTx triggers system auto-emit of CompleteSetMintTx + CpmmPoolTx against Treasury budget. | A3 | 4 |
| FR-N3.5 | Treasury budget exhaustion fails-closed (no ghost pool created). | A3 | 4 |
| FR-N3.6 | Agent prompt includes nearby WorkTx + CPMM prices context block. | A4 | 2 |
| FR-N3.7 | Prompt context does not leak private metrics (reputation, classifier_version, golden_path). | A4 | 2 |
| FR-N3.8 | `run_report` binary produces byte-deterministic v3-run6-style markdown from any evidence dir. | A5 | 1 |
| FR-N3.9 | Sandbox-mode FinalizeReward + EventResolve fire on 1-tx OMEGA path. | A6 | 3 or 4 |
| FR-N3.10 | Post-TB-N3 batch tape on same 9-problem set shows `buy_with_coin_router >= work_tx_accepted_count / 5` (loose minimum). | Phase 2 evidence | runner |

## §6. Ship Gates (SG) — VERBATIM from architect ruling 2026-05-11 (v3 binding)

The v3 SG-N3.1..15 set (this section) **supersedes** the v2 SG-N3.1..N3.9
table in §6-legacy below. v2 thresholds preserved in §6-legacy for tracking
the Phase 1 baseline → Phase 2 delta, but v3 gates are the binding ship
criteria.

| ID | Gate | Implementation locus |
|----|------|----------------------|
| SG-N3.1 | A1/A2 fixture shows invest intent → BuyWithCoinRouterTx with existing pool. | `tests/constitution_tb_n3_invest_routing.rs::fixture_pool_present_router_accepts` |
| SG-N3.2 | A2 missing-pool path routes to L4.E, not silent drop. | `tests/constitution_tb_n3_invest_routing.rs::missing_pool_routes_to_l4e` |
| SG-N3.3 | A3 Class-4 dual-pattern self-audit dossier exists before A3 ships. | `handover/audits/TB_N3_A3_SELF_AUDIT_<ts>.md` (Codex G2 + Gemini DT patterns) |
| SG-N3.4 | Accepted WorkTx creates MarketSeed / CompleteSet+Pool for **node** event, not task event. | `tests/constitution_tb_n3_event_id_namespace.rs::pool_event_id_starts_with_node_survive` |
| SG-N3.5 | Treasury / MarketMakerBudget is debited for every auto pool seed. | `tests/constitution_tb_n3_treasury_debit.rs::budget_debit_per_pool` |
| SG-N3.6 | Budget insufficient creates no pool and no ghost liquidity. | `tests/constitution_tb_n3_treasury_debit.rs::budget_exhausted_no_ghost_pool` |
| SG-N3.7 | Phase 2 same-9 batch has `cpmm_pool >= accepted_work_tx_count` when budget sufficient. | `TB_N3_BATCH_SUMMARY.json` aggregator |
| SG-N3.8 | Phase 2 same-9 batch has `buy_with_coin_router >= 1`. | `TB_N3_BATCH_SUMMARY.json` aggregator |
| SG-N3.9 | Solved rate ≥ 4/9 OR 3-seed diagnostic justifies stochastic non-regression. | Batch script auto-runs 3-seed if < 4/9 |
| SG-N3.10 | Wall time ≤ 4 hours (14_400 s). | `batch_runner.log` total wall |
| SG-N3.11 | P06 full economic cycle remains non-regressed (work + verify + finalize + event_resolve). | `tests/constitution_tb_n3_p06_no_regression.rs` (or summary assert) |
| SG-N3.12 | run_report renders citation tree + market section from ChainTape + CAS. | `target/release/audit_dashboard --run-report` on Phase 2 evidence |
| SG-N3.13 | Constitution gates no regression (TB-N3 gates added; baseline 288 → ≥ 288 + new). | `bash scripts/run_constitution_gates.sh` |
| SG-N3.14 | Workspace tests no regression (baseline ~1448 → ≥ 1448 + new). | `cargo test --workspace` |
| SG-N3.15 | No f64 / no ghost liquidity / no price-as-truth checks pass. | `tests/constitution_tb_n3_invariants.rs` (re-uses `assert_no_post_init_mint` + `assert_total_ctf_conserved` + `assert_complete_set_balanced` + source-grep f64-in-market-path) |

## §6-legacy. v2 phase-1-calibrated ship gates (superseded by v3 §6 above)

All thresholds calibrated against Phase 1 empirical baseline (§0.5). Phase 2 batch
re-runs the SAME 9-problem set to enable direct delta comparison.

| ID | Gate | Phase 1 baseline | Phase 2 threshold | Verified by |
|----|------|------|------|-------------|
| SG-N3.1 | A1+A2 smoke shows `buy_with_coin_router >= 1` on real-LLM tape | 0 | ≥ 1 on P07-class problem | real-LLM smoke evidence |
| SG-N3.2 | A3 architect §8 sign-off (Class-4 STEP_B) | — | sign-off file | architect ratification + Codex G2 + Gemini PASS |
| SG-N3.3 | A5 renderer byte-determinism + golden-path detection on Phase 1 evidence | — | gate PASS | `tests/constitution_tb_n3_a5_run_report.rs` |
| SG-N3.4a | Phase 2 batch `cpmm_pool >= work_tx_accepted_count` | 0 / 5 work | ≥ 5 (one per accepted WorkTx) | Phase 2 verdict.json sum |
| SG-N3.4b | Phase 2 batch `buy_with_coin_router >= 1` aggregate | 0 | ≥ 1 across batch (architect mandate); aspiration ≥ 3 (3+ multi-tx problems × 1 invest each) | Phase 2 verdict.json sum |
| SG-N3.4c | Phase 2 batch `cpmm_swap >= 0` (depends on agent dynamics) | 0 | ≥ 0 (no hard floor; depends on agents trading peer-to-peer in same event) | Phase 2 verdict.json sum |
| SG-N3.4d | Phase 2 batch SOLVED rate must NOT degrade > 20% | 5 / 9 (56%) | ≥ 4 / 9 (44%) — market activity must not distract LLM proof-search | Phase 2 batch summary |
| SG-N3.4e | Phase 2 batch wall time must NOT exceed 4hr (Phase 1 = 33 min) | 33 min | ≤ 240 min | batch_runner.log |
| SG-N3.4f | Phase 2 P06 must still produce full cycle (work + verify + finalize + event_resolve) | 1 | 1 (no regression on validated TB-N2 B2 path) | P06 verdict.json |
| SG-N3.4g | Phase 2 P01/P02 with A6 sandbox synthesis fire finalize_reward + event_resolve | 0 / 0 | 1 / 1 each | P01+P02 verdict.json |
| SG-N3.5 | Constitution gates no regression from 288 GREEN | 288/0/1 | ≥ 288/0/1 | `bash scripts/run_constitution_gates.sh` |
| SG-N3.6 | Workspace tests no regression | ~1390 PASS | ≥ 1390 PASS | `cargo test --workspace` |
| SG-N3.7 | Monetary invariants 0 violations under agent CPMM load | n/a (no market) | 0 violations | `assert_complete_set_balanced` + `assert_total_ctf_conserved` |
| SG-N3.8 | Trust Root rehash iff Class-4 STEP_B file touched (A3) | n/a | manifest updated | trust_root manifest diff |
| SG-N3.9 | FC1-INV1 + INV3 RED count must NOT increase from baseline (Phase 1 had 1 RED on P07 due to bash extractor bug; if extractor patched then 0 RED) | 1 RED (P07 extractor) | ≤ 1 RED OR root-cause patched first | `fc_witness_aggregate.json` |

## §7. Forbidden — VERBATIM from architect ruling 2026-05-11 (v3 binding)

```
No event_id = task_id for node markets.
No auto pool for L4.E rejection.
No pool without collateral.
No automatic free 100 YES + 100 NO.
No ghost liquidity.
No CLOB.
No cross-event arbitrage.
No public chain.
No real funds.
No price-as-truth.
No hidden predicate leak.
No dashboard source-of-truth.
No f64 money math.
No A6 production self-verify.
```

In addition, v2 §7 forbidden carry-overs (still binding):

- **No batch §8 for Class-4 atoms** — A3 (and A0.5) each need own per-atom
  §8 packet per `feedback_no_batch_class4_signoff`.
- **No retroactive evidence rewrite** — Phase 1 evidence is read-only baseline;
  Phase 2 (post-TB-N3) goes to fresh timestamp dir.
- **No agent-submitted system tx** — A3 auto-emit goes through
  agent-signed canonical admission paths (TaskOpen + MarketSeed + CpmmPool
  signed by `MarketMakerBudget` agent identity), NEVER through bypass routes.

## §8. Pre-existing forward-bound items (track but do not close in TB-N3)

These remain DEFERRED-FORWARD per `LATEST.md` "Open after Polymarket":

- **C.5 PromptCapsule evaluator runtime wire-up** — Class 3 / ~1-2 days;
  forward TB.
- **B.4 CAS strict-Merkle commit-chain redesign** — Class 3-4 / Stage A3.6
  enhancement TB; forward.
- **K.1-K.6 Stage D real-world readiness directive package** — architect
  Class-4; forward.

TB-N3 does NOT include these. They remain in matrix §8 as DEFERRED-FORWARD.

## §9. Open questions for architect (require resolution before A3)

Each carries a Claude recommendation informed by Phase 1 baseline.

1. **Default pool seed size (`DEFAULT_POOL_SEED`)** —
   *Recommend*: **100_000 microCoin (0.1 Coin)** per pool, configurable via
   `TURINGOS_AUTO_MARKET_SEED_MICRO` env.
   *Phase-1-informed reasoning*: Phase 1 produced 5 accepted WorkTx across 9
   problems → A3 would auto-create 5 pools. At 100k µC each, that's 500k µC
   (0.5 Coin) treasury draw per 9-problem batch. genesis `MarketMakerBudget`
   needs ≥ ~10× this for safety margin (~5 Coin / 5M µC). Architect to set
   exact budget per `RSP-3 amendment 2026-05-02`.
2. **Treasury budget allocation** — does `on_init` already allocate
   `MarketMakerBudget` (per RSP-3 amendment 2026-05-02)?
   *Verify before A3 starts*; if absent, A3 charter expands to include
   genesis amendment (Class 4 STEP_B).
3. **A1 schema variant policy** — should V1 prompt variant ALSO include
   invest (revert session #34 strip) OR keep V1 as explicit "no market" opt-out?
   *Recommend*: **revert V1 strip**. V0+V1 both include invest. Session #34
   "EMPIRICALLY CLOSED" was N=1-specific; user 2026-05-11 explicitly clarified
   v4 terminal architecture is multi-agent.
4. **A6 Class judgment** — synthesis route (Class 2-3) vs SystemEmitCommand
   variant (Class 4 STEP_B)?
   *Recommend*: **synthesis route for MVP** (Class 2-3); architect §8 not
   strictly required if no new TypedTx variant. Forensic from Phase 1: P01/P02
   fast-solve (1 tx) skips finalize because `verify_tx_id == None` at OMEGA exit;
   synthesizing a `VerifyTx{verdict=Confirm, bond_micro=0}` from the OMEGA-claimant
   agent is the lowest-friction fix and doesn't touch sequencer admission semantics.
   SystemEmitCommand variant deferred unless synthesis breaks invariants in testing.
5. **A2 slippage default** — `min_out_shares = 0` vs `min_out_shares = floor(out_shares * 0.95)`?
   *Recommend*: **0 for MVP**. Agents can specify explicitly when they want
   slippage protection. Phase 1 evidence shows no live pool yet so we have no
   data on real-world price drift; conservative default = 0 lets us measure.
6. **A3 trigger granularity (NEW question, Phase-1-informed)** —
   on which event does A3 fire?
   - (a) **Every accepted WorkTx** (5 pools per Phase 1 batch — sparse, principled)
   - (b) **Every L4-or-L4.E externalized step** (174 pools per Phase 1 batch — too noisy)
   - (c) **Every step_partial_ok node** (~30 pools per Phase 1 batch — middle ground)
   *Recommend*: **(a) every accepted WorkTx** — matches the v3 run6 mental
   model where each surviving proposal becomes a tradeable node. Failed steps
   (parse_fail / llm_err / step_reject) get NO market — they are L4.E rejections,
   not proposals worth speculating on.
7. **A4 prompt context K (NEW question, Phase-1-informed)** —
   how many nearby WorkTx to expose in agent prompt?
   *Recommend*: **K=10 default, K=5 if context budget tight**. Phase 1
   per-problem average is 3.56 L4 (mostly task_open + escrow_lock + 1
   work_tx); cross-problem context could include WorkTx from OTHER concurrent
   problems if same agent operates across them. Multi-tx problems (P07: 39
   tx) generate more candidates; K=10 reasonable upper bound.
8. **Phase 2 batch composition (NEW question, Phase-1-informed)** —
   should Phase 2 re-run the SAME 9 problems OR expand?
   *Recommend*: **SAME 9 first** for direct delta comparison; if SOLVED rate
   degrades by < 20% AND market tx > 0, expand to 20-problem M0 set per
   `feedback_minif2f_scaling_policy`. Re-running same set is the cleanest
   kill-criterion test (controls for problem difficulty distribution).

## §10. Estimate

- A1 + A2: ~1-2 days (Class 2-3, no kernel change)
- A3: ~2-3 days (Class 4 STEP_B + §8 dual audit cycle)
- A4: ~1 day (Class 2 prompt only)
- A5: ~1 day (Class 1 additive renderer; Phase 1 evidence already a real
  test corpus for byte-determinism check)
- A6: ~1 day if synthesis route (Class 2-3, recommended per §9-Q4); ~1-2
  days if SystemEmitCommand variant (Class 4 + §8)
- Phase 2 batch + verification: **~1 hour wall** + audit (Phase 1 ran 9
  problems in 33 min; Phase 2 adds A3 auto-emit + agent invest calls →
  expect ~45-90 min wall; well under 4hr SG-N3.4e cap)

**Total: ~7-10 days wall**, gated by Class-4 §8 cadence (1-2 architect
rounds per atom). Phase 1 evidence reduces uncertainty on Phase 2 wall
time + smoke-target problem selection.

## §11. Risk register (Phase-1-informed)

| Risk | Phase-1 evidence | Mitigation |
|------|------|-----------|
| A3 sequencer auto-emit introduces consensus rule that's hard to roll back | n/a (new) | STEP_B parallel-branch + §8; reversible via env-gate `TURINGOS_AUTO_MARKET=1` for staged rollout |
| Agent invest decisions degrade Lean proof solving (distraction) | baseline SOLVED = 5/9 | A1+A2 ship gate measures SOLVED count on same 9-problem set (SG-N3.4d); if drops > 20% → OBS file + ratification |
| Multi-agent + market introduces flaky tests (race / timing) | P03 EventResolve poll-budget edge observed (11s wall → finalize fired but event_resolve race expired) | Use `submit_and_apply` deterministic harness; no `tokio::time` in tests; A6 sandbox synthesis avoids the race for fast-solve |
| Phase 2 batch wall time grows due to extra system-tx | Phase 1 wall = 33 min; SG-N3.4e cap = 240 min | A3 budget defaults sized for ~10x current tx volume (5 → ~50 system-tx); if wall > 4hr for 9-problem batch → OBS file |
| Run-report renderer becomes source-of-truth instead of derived view | n/a (new) | Test `run_report_byte_deterministic` + dashboard-regeneratable gate; renderer MUST be deletable + regeneratable |
| **Stochastic LLM regression** | P09 aime_1984_p1: tb_c0 2026-05-06 SOLVED → Phase 1 today NOT solved (same model + n=5) | Phase 2 batch runs each problem N≥3 seeds; report Wilson 95% CI per problem; do NOT treat single Phase 2 run as the kill criterion |
| **DeepSeek model snapshot drift** | observed between 2026-05-06 → 2026-05-11 | pin model snapshot identifier in BenchmarkManifest; report `model_snapshot` field in every PPUT_RESULT |
| **TB-N1 / TB-N2 verify+finalize+event_resolve interaction with A3+A2** | Phase 1: 2 cycles fired (P03, P06); P06 alone fired all 4 | Ship gate SG-N3.4f preserves P06 full-cycle; SG-N3.4g extends to P01+P02 via A6 |

## §12. Linked authority

- Architect Part C §2.1 verbatim CPMM formula (already implemented as P-M6)
- Architect Stage C overall §8 §6 — "agent-bridge for the CPMM substrate" forward-bound
- User 2026-05-11 verbatim: "v4 终极设计目的也是 multi-llm-agents" + "我要从
  tape 上看到类似 [run6] 的完整经济学实现的证明"
- `feedback_constitutional_harness_engineering` — harness → real run → audit
- `feedback_no_batch_class4_signoff` — A3 (and A6 if Class-4) per-atom §8
- `feedback_dual_audit` — Class-4 PRE-§8 dual audit timing
- `feedback_real_problems_not_designed` — Phase 2 batch on same 9 MiniF2F problems
- `feedback_no_workarounds_strict_constitution` — "我不要凑活"; no
  prompt-level workaround when kernel can do it right
- `LATEST.md` "Open after Polymarket" block — C.5 / B.4 / K.* remain
  DEFERRED-FORWARD, NOT folded into TB-N3

## §13. Phase 1 batch evidence references (binding for architect review)

Source materials the architect can read to verify the empirical baseline:

| Path | Content |
|------|---------|
| `handover/evidence/phase1_n5_batch_2026-05-11T03-46-20Z/PHASE_1_CLOSURE_REPORT.md` | Full closure report (9-problem solve table + economic-tape aggregate + FC invariant status + cross-wire gap quantification) |
| `handover/evidence/phase1_n5_batch_2026-05-11T03-46-20Z/TBC0_BATCH_SUMMARY.json` | Per-problem PPUT + halt + duration JSON |
| `handover/evidence/phase1_n5_batch_2026-05-11T03-46-20Z/fc_witness_aggregate.json` | FC1/FC2/FC3 aggregate witness across 9 problems |
| `handover/evidence/phase1_n5_batch_2026-05-11T03-46-20Z/P0{1..9}_*/verdict.json` | Per-problem audit_tape verdict (tx_kind_counts + tape_root + 51 assertions) |
| `handover/evidence/phase1_n5_batch_2026-05-11T03-46-20Z/P06_mathd_numbertheory_1124/verdict.json` | **The single full-economic-cycle witness** (work=1, verify=1, finalize_reward=1, event_resolve=1) |
| `handover/evidence/phase1_n5_batch_2026-05-11T03-46-20Z/batch_runner.log` | Runner log including audit_tape verdicts per problem |
| `handover/evidence/phase1_smoke_n5_2026-05-11T03-43-46Z/` | Pre-batch smoke evidence (single-problem mathd_algebra_107 smoke that validated runner path) |

### Architect decision items summary

Before A3 (Class-4 STEP_B) can start, architect needs to rule on §9 questions
1, 2, and 6 (DEFAULT_POOL_SEED size + MarketMakerBudget allocation + A3 trigger
granularity). Items 3, 4, 5, 7, 8 carry Claude recommendations that can proceed
under existing autonomous-execution grant unless architect overrides.

### What this charter is NOT

- This charter is NOT itself a Class-4 atom. It is Class-0 documentation per
  `feedback_tb_phase_tag_required`.
- This charter does NOT modify constitution.md. constitution_hash unchanged.
- This charter does NOT touch src/. Phase 1 batch was a runner-only operation
  against existing HEAD `ced19363` binary.
- This charter does NOT bind the architect. It carries Claude's recommendations
  + Phase 1 evidence so the architect can rule from the same baseline.

### Changelog

- **v1 (initial draft, this session ~03:35Z)**: charter scaffold + 6 atoms +
  ship gates + open questions; written BEFORE Phase 1 batch completed.
- **v2 (this update, ~04:25Z)**: Phase 1 empirical baseline added (§0.5);
  ship gates calibrated with concrete thresholds (§6); open questions augmented
  with Phase-1-informed defaults + 3 new questions Q6-Q8 (§9); estimate +
  risk register tightened (§10-§11); evidence references added (§13).
- **v3 (this update, post architect ruling 2026-05-11)**: §0.6 NEW — architect
  ruling amendments + Q1-Q8 verdicts (binding). §3 amended — A0 + A0.5
  inserted; A6 excluded from N3 core. §6 superseded — SG-N3.1..SG-N3.15
  verbatim from ruling; v2 SG table moved to §6-legacy. §7 superseded —
  forbidden list verbatim from ruling. New surfaces: `src/state/typed_tx.rs::node_survive_event_id`
  pure constructor; `src/runtime/market_decision_trace.rs` Class-1 module;
  `src/sdk/market_context.rs` same-task K=10 renderer;
  `tb_n3_invest_to_router_tx` + `tb_n3_emit_node_market_after_work_accept`
  in `src/runtime/adapter.rs`. STEP_B branches for A0.5 + A3. PRE-§8 dual
  self-audit dossier at `handover/audits/TB_N3_A3_SELF_AUDIT_<ts>.md`.
