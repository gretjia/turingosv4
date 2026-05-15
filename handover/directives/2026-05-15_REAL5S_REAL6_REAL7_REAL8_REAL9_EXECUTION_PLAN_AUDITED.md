# REAL-5S -> REAL-9 Execution Plan Draft

Status: AUDITED. R1 VETO was remediated, and R2 independent clean-context Codex review returned `PROCEED` in `handover/audits/CODEX_REAL5S_REAL9_EXECUTION_PLAN_ALIGNMENT_REVIEW_R2.md`. This file is ready for user approval before any `turingos_dev` implementation run.

Architect source of truth:

```text
handover/directives/2026-05-15_REAL5S_REAL6_REAL7_REAL8_REAL9_ARCHITECT_ORIGINAL.md
```

R1 audit being remediated:

```text
handover/audits/CODEX_REAL5S_REAL9_EXECUTION_PLAN_ALIGNMENT_REVIEW_R1.md
```

## 0. R1 VETO Closure

This draft directly closes the R1 VETO findings:

```text
VETO: 当前可见方案不是 REAL-5S -> REAL-9 方案，而是旧 REAL-5 / REAL-6 方案。
VETO: 独立审计没有作为当前 REAL-5S -> REAL-9 执行前置落地。
VETO: 未要求把当前架构师原文逐字落地到新的本地文件，也未要求把审计后的 REAL-5S -> REAL-9 方案落地。
```

Closure decisions:

- The new route uses only `REAL-5S / REAL-6 / REAL-7 / REAL-8 / REAL-9`.
- `Phase E0-E7` naming is forbidden.
- The architect original has been saved verbatim in the 2026-05-15 original file listed above.
- This draft must be independently audited before approval.
- After audit `PROCEED` and user approval, the approved plan will be copied to:

```text
handover/directives/2026-05-15_REAL5S_REAL6_REAL7_REAL8_REAL9_EXECUTION_PLAN_APPROVED.md
```

## 1. Harness Frame

Risk class: Class 4 package.

Reason:

- REAL-6A may require `EventResolveTx YES/NO` semantics and therefore touches restricted surfaces if current `EventResolveTx` is YES-only.
- REAL-6 changes market event timing.
- REAL-6C touches risk-gated role availability.
- REAL-6D introduces scheduler recommendation traces, explicitly observe-only.
- REAL-7/REAL-8 are ship-path evidence packages.

FC mapping:

```text
FC1: role/market action loop; every externalized action or abstain reason is ChainTape/CAS-visible.
FC2: task-open market and replay authority; TaskOutcomeMarket exists before first WorkTx.
FC3: dashboard, reports, scheduler trace, PnL, and benchmark outputs are materialized views.
Art. III: role-scoped prompts and PnL summaries must not expose private CoT or raw logs.
Economy: no ghost liquidity, no f64 economy, CTF conserved.
Predicate: price never affects Lean predicate or L4/L4.E truth.
```

Restricted surfaces:

```text
src/state/typed_tx.rs
src/state/sequencer.rs
src/bottom_white/ledger/system_keypair.rs
src/runtime/adapter.rs
genesis_payload.toml
```

If REAL-6A requires `EventResolveTx` wire-shape, signing payload, or sequencer admission changes, implementation must keep it inside the Class-4 atom, update Trust Root, and obtain clean-context implementation audit before ship.

## 2. REAL-5S — Scaffold Ratification / Clean-Negative Closure

Architect route:

```text
REAL-5S  Scaffold ratification / clean-negative closure
```

Goal:

```text
正式确认：

REAL-5 proves role scaffolding.
REAL-5 does not prove market emergence.
```

### Atom 5S-A — Scaffold Ratification Report

Produce:

```text
handover/evidence/real5_overnight_20260514/REAL5_SCAFFOLD_RATIFICATION_REPORT.md
```

The report must write:

```text
role gateway blocks Trader proof-style leakage
Verifier behavior observed
Trader buy=0
NoPool dominates
No E2/E3 claim
```

The report must cite existing REAL-5 evidence:

```text
handover/evidence/g_phase_real_5_trader_first_b8_rolegate_20260514T192523Z
handover/evidence/g_phase_real_5_core3_b8_rolegate_20260514T192958Z
handover/audits/CODEX_REAL5_IMPLEMENTATION_REVIEW_R3.md
handover/evidence/dev_self_hosting/dev_1778788069384_807750
```

### Atom 5S-B — Clean Negative Report

Produce:

```text
handover/evidence/real5_overnight_20260514/REAL5_CLEAN_NEGATIVE_NO_TRADE_REPORT.md
```

The report must answer:

```text
Why no trade?
  NoPool dominates.
  Post-accept node market timing too late.
  Prompt-only exhausted.
```

### REAL-5S Ship Gates

```text
SG-5S.1 REAL-5 scaffold claim narrowed.
SG-5S.2 No E2/E3 emergence claim.
SG-5S.3 Role gateway regression tests pass.
SG-5S.4 Clean-negative report filed.
SG-5S.5 Constitution gates no regression.
```

Targeted verification:

```bash
cargo test --test constitution_real5_typed_generation_gateway
cargo test --test constitution_real5_role_based_smoke
bash scripts/run_constitution_gates.sh
```

## 3. REAL-6 — Event Timing & Lawful Pressure

Architect ruling:

```text
REAL-6 是核心阶段。
```

Accepted diagnosis:

```text
NoPool means post-accept node market is too late.
Stop prompt-only variants.
```

### REAL-6A — TaskOutcomeMarket

Goal:

```text
在任务开始时创建市场：

Event:
  task will be solved within budget/deadline.
```

Why first:

```text
它比 NodeSurviveMarket 更早，比 AttemptPredictionMarket 更安全。
```

Required structure:

```rust
TaskOutcomeEvent {
    event_id,
    task_id,
    deadline_round,
    max_budget,
    created_by_task_open_tx,
}
```

Required execution:

```text
TaskOpenTx / EscrowLockTx
-> MarketSeedTx for task_outcome event
```

Implementation atoms:

1. Add `TaskOutcomeEvent` and a CAS provenance manifest for task-level market creation.
2. Wire task-open runtime so the TaskOutcomeMarket is seeded after accepted `TaskOpenTx` / `EscrowLockTx` and before first `WorkTx`.
3. Ensure TraderView includes the active TaskOutcomeMarket, pool depth, price, budget/deadline, and scoped PnL.
4. Add scripted trader fixture for Buy YES and Buy NO on the TaskOutcomeMarket.
5. Keep real LLM trader behavior non-forced: it may buy or emit `MarketDecisionTrace` / classified `NoTradeReason`.
6. Implement `EventResolveTx YES` if verified proof occurs before budget/deadline.
7. Implement `EventResolveTx NO` if exhausted/deadline occurs without verified proof.

Important Class-4 constraint:

```text
SG-6A.7 says EventResolveTx NO.
Do not silently replace this with TaskBankruptcyTx.
```

If current `EventResolveTx` is YES-only, add an explicit Class-4 `EventResolveTx` outcome design inside REAL-6A:

- preserve historical evidence through schema bump or dual-reader;
- update canonical signing payload tests;
- update sequencer admission/apply semantics;
- update agent-ingress rejection for system-only tx if needed;
- update Trust Root for modified pinned files;
- include regression tests proving old EventResolve evidence remains readable or grandfathered.

REAL-6A gates:

```text
SG-6A.1 TaskOutcomeMarket exists before first WorkTx.
SG-6A.2 TraderView contains active TaskOutcomeMarket.
SG-6A.3 NoPool no longer dominates when task market exists.
SG-6A.4 Scripted trader can Buy YES/NO on TaskOutcomeMarket.
SG-6A.5 Real LLM trader emits MarketDecisionTrace or classified NoTradeReason.
SG-6A.6 EventResolveTx YES if verified proof before budget/deadline.
SG-6A.7 EventResolveTx NO if exhausted/deadline without verified proof.
SG-6A.8 No ghost liquidity.
SG-6A.9 CTF conserved.
SG-6A.10 Price never affects Lean predicate.
```

### REAL-6B — AttemptPredictionMarket

Goal:

```text
让 candidate proof 在 Lean final resolution 前有一个短期预测市场。
```

Sealed Oracle flow:

```text
SubmitCandidateTx
-> AttemptPredictionMarket opens
-> exactly K logical tape ticks for Trader / Verifier / Challenger
-> MarketCloseTx
-> OracleResolveTx executes Lean result
```

Current-stage limit:

```text
REAL-6B = design + scripted fixture only.
No live real-LLM ship until explicit Class-4 ratification.
```

Implementation atoms:

1. Write design record for the sealed oracle flow.
2. Add scripted fixture structs/manifests without live real-LLM ship.
3. Use logical tape ticks only; no wall-clock sleep.
4. Prove `MarketCloseTx` happens before `OracleResolveTx` in fixture evidence.
5. Prove Lean oracle remains absolute truth and price does not affect verification.

REAL-6B gates:

```text
SG-6B.1 No sleep-based artificial blocking.
SG-6B.2 K logical tape ticks are deterministic and replayable.
SG-6B.3 Lean oracle remains absolute truth.
SG-6B.4 MarketCloseTx happens before OracleResolveTx.
SG-6B.5 Trader actions during window are ChainTape-visible.
SG-6B.6 Price does not affect verification.
SG-6B.7 No ghost liquidity.
```

### REAL-6C — Conviction Budget / PnL Feedback

Goal:

```text
恢复 v3 的经济压力，但保持 v4 合宪。
```

Principle:

```text
free cognition
paid conviction
```

Required structure:

```rust
ConvictionBudget {
    agent_id,
    available_micro,
    reserved_micro,
    realized_pnl,
    unrealized_pnl,
    risk_cap,
}
```

Implementation requirement:

```text
ChainTape fold / materialized view
not HashMap sidecar
```

Risk restriction:

```text
below risk cap:
  cannot Trader/Challenger high-risk action
  can still observe/read/abstain/solve/possibly verify
```

Implementation atoms:

1. Compute ConvictionBudget as a pure derived view from ChainTape/CAS/QState.
2. Expose scoped ConvictionBudget/PnL summary to the agent prompt.
3. Gate Trader/Challenger high-risk actions when below risk cap.
4. Preserve read/observe/abstain/solve/possibly verify for low-balance agents.
5. Emit AutopsyCapsule after significant loss.

REAL-6C gates:

```text
SG-6C.1 PnL derived from ChainTape/CAS.
SG-6C.2 No PnL HashMap sidecar source-of-truth.
SG-6C.3 Agent prompt sees scoped PnL summary.
SG-6C.4 Low-balance agent blocked from high-risk market actions.
SG-6C.5 Low-balance agent not erased / reset.
SG-6C.6 AutopsyCapsule generated after significant loss.
```

### REAL-6D — Opportunity Scheduler Observe-Only

Goal:

```text
价格 / PnL 进入调度观察，但不执行 admission change。
```

Required structure:

```rust
SchedulerDecisionTrace {
    head_t,
    visible_agents,
    visible_nodes,
    price_signals,
    pnl_signals,
    recommended_agent,
    recommended_role,
    recommended_action,
    observe_only: true,
}
```

Implementation atoms:

1. Emit SchedulerDecisionTrace to CAS/ChainTape evidence path.
2. Force `observe_only: true`.
3. Ensure recommendation does not affect sequencer admission, L4, or L4.E.
4. Render scheduler recommendation in dashboard as non-binding.

REAL-6D gates:

```text
SG-6D.1 Scheduler trace includes price/PnL signals.
SG-6D.2 observe_only=true.
SG-6D.3 Recommendation does not change sequencer admission.
SG-6D.4 Price does not affect L4/L4.E.
SG-6D.5 Dashboard shows scheduler recommendation as non-binding.
```

## 4. REAL-7 — V3-Equivalent Structural Smoke

Goal:

```text
不是复制 v3 数量，而是重建 v3 的结构压力。
```

Architect comparison:

```text
v3 evidence 显示某些 run 产生过非零市场活动，比如 OMEGA_v3chat_N3_50k 的 raw-backed 指标包括 tx_count=436、nodes=127、markets=127 等。
但 v3 的核心价值不是数字本身，而是短反馈环：LLM action -> wallet pressure -> market price -> prompt/scheduler visibility -> OMEGA settlement。
```

Minimum structure:

```text
>= 5 agents
>= 3 roles active
>= 3 tasks
>= 1 TaskOutcomeMarket
>= 1 scripted AttemptPredictionMarket
>= 1 BuyYesWithCoinRouterTx
>= 1 BuyNoWithCoinRouterTx or Short equivalent
>= 1 VerifyTx
>= 1 ChallengeTx or NoChallengeReason
>= 1 EventResolveTx
>= 1 PnL delta
>= 1 AutopsyCapsule if loss occurs
```

Implementation atoms:

1. Run persistent runtime_repo with same CAS across at least three tasks.
2. Use at least five agents and at least three active roles.
3. Include TaskOutcomeMarket and scripted AttemptPrediction fixture.
4. Witness Buy YES and Buy NO/Short equivalent without claiming forced live emergence.
5. Witness VerifyTx, ChallengeTx or NoChallengeReason, EventResolveTx, PnL delta, and AutopsyCapsule when loss occurs.
6. Produce clean v3 comparison without claiming identical equivalence.

REAL-7 gates:

```text
SG-7.1 Structural pattern achieved.
SG-7.2 No forced investment.
SG-7.3 No price-as-truth.
SG-7.4 No ghost liquidity.
SG-7.5 All market actions ChainTape-visible.
SG-7.6 Dashboard regenerates from ChainTape + CAS.
SG-7.7 Clean comparison to v3 metrics without claiming identical equivalence.
```

## 5. REAL-8 — Formal Market A/B Benchmark

A/B conditions:

```text
A: market disabled
B: market visible, no TaskOutcomeMarket
C: TaskOutcomeMarket enabled
D: TaskOutcomeMarket + scripted AttemptPrediction fixture
```

Metrics:

```text
solve rate
verified PPUT
false accept rate
cost per verified proof
market tx count
NoTradeReason distribution
PnL dispersion
role diversity index
audit failure rate
```

Implementation atoms:

1. Pin same problem set across A/B/C/D.
2. Pin same model assignment across A/B/C/D.
3. Pin same budgets across A/B/C/D.
4. Ensure all runs are chain-backed.
5. Produce negative-result-valid report if no market effect appears.
6. Avoid causal overclaim; report benchmark evidence only.

REAL-8 gates:

```text
SG-8.1 Same problem set across arms.
SG-8.2 Same model assignment.
SG-8.3 Same budgets.
SG-8.4 All runs chain-backed.
SG-8.5 No overclaim of causality.
SG-8.6 Negative result is valid and documented.
```

## 6. REAL-9 — Whitepaper / Launch Synthesis

Write:

```text
TuringOS Generative Economy Whitepaper Update
TuringOS Market Developer Manual
```

Must explicitly state:

```text
v4 does not copy v3.
v4 rebuilds v3's economic pressure under constitution.
price = signal, not truth.
market = role-specific institution, not prompt decoration.
```

Final framing to preserve:

```text
v3 taught us pressure.
v4 gives us law.
Next phase must build lawful pressure.
```

## 7. Forbidden In All Ship Claims

```text
no forced trades
no price-as-truth
no ghost liquidity
no f64 economy
no off-tape WAL as truth
no private CoT recording
no raw-log broadcast
```

Additional local constraints:

- No Phase E0-E7 route naming.
- No prompt-only A07/A08/A09 variants.
- No dashboard/report/stdout as source of truth.
- No sequencer admission change in REAL-6D.
- No live REAL-6B real-LLM ship until explicit Class-4 ratification.

## 8. Harness And Audit Sequence

Execution sequence after this draft receives independent audit `PROCEED` and user approval:

```text
1. Copy audited draft to APPROVED path.
2. Open turingos_dev with risk=4.
3. Record original architect file, approved plan, and R2 plan audit.
4. Implement REAL-5S first.
5. Implement REAL-6A/B/C/D as bounded atoms.
6. Run targeted gates and real evidence.
7. Request clean-context Codex implementation audit.
8. Fix CHALLENGE/VETO or forward-defer only with explicit user approval.
9. turingos_dev validate.
10. turingos_dev close.
11. Only then update LATEST / matrix / TB_LOG.
```

Open command shape:

```bash
turingos_dev open \
  --title "REAL-5S to REAL-9 lawful pressure route" \
  --module "REAL-5S/REAL-6/REAL-7/REAL-8/REAL-9" \
  --risk 4 \
  --fc "FC1 role/market action loop; FC2 task-open market/replay authority; FC3 dashboard/evidence materialized view; Art. III shielding; economy/predicate invariants" \
  --allowed "<approved path list>"
```

## 9. Verification Commands

Targeted gates to add and run:

```bash
cargo test --test constitution_real5s_scaffold_ratification
cargo test --test constitution_real6_task_outcome_market
cargo test --test constitution_real6_attempt_prediction_fixture
cargo test --test constitution_real6_conviction_budget
cargo test --test constitution_real6_scheduler_observe_only
cargo test --test constitution_real7_v3_structural_smoke
cargo test --test constitution_real8_market_ab_benchmark
```

Ship-level gates:

```bash
bash scripts/run_constitution_gates.sh
cargo test --workspace --no-fail-fast -- --test-threads=1
cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo
git diff --check
```

REAL-6A evidence command shape:

```bash
PHASE_D_HETERO_OK=1 \
TURINGOS_REAL5_ROLE_VIEWS=1 \
TURINGOS_REAL6_TASK_OUTCOME_MARKET=1 \
TURINGOS_REAL6_CONVICTION_BUDGET=1 \
TURINGOS_REAL6_SCHEDULER_OBSERVE_ONLY=1 \
TURINGOS_REAL5_ROLE_ASSIGNMENT="Solver,Trader,Verifier,Challenger,Observer" \
bash scripts/run_g_phase_batch.sh g_phase_real_6a_task_outcome_<UTC> mini
```

REAL-8 benchmark command shape:

```bash
bash scripts/run_real8_market_ab_benchmark.sh \
  --problems <same_problem_set_manifest> \
  --models <same_model_assignment_manifest> \
  --budgets <same_budget_manifest> \
  --arms A,B,C,D \
  --out handover/evidence/real8_market_ab_<UTC>
```

## 10. Plan R2 Audit Prompt

After this draft is saved, spawn independent clean-context Codex reviewer with:

```text
Read:
1. handover/directives/2026-05-15_REAL5S_REAL6_REAL7_REAL8_REAL9_ARCHITECT_ORIGINAL.md
2. handover/directives/2026-05-15_REAL5S_REAL6_REAL7_REAL8_REAL9_EXECUTION_PLAN_DRAFT.md
3. handover/audits/CODEX_REAL5S_REAL9_EXECUTION_PLAN_ALIGNMENT_REVIEW_R1.md

Review whether R1 VETO findings are closed and whether the draft preserves every architect requirement:
- route names REAL-5S / REAL-6 / REAL-7 / REAL-8 / REAL-9
- no Phase E0-E7
- REAL-5S scaffold + clean-negative only
- Atom 5S-A / Atom 5S-B products and required wording
- SG-5S.1 through SG-5S.5
- REAL-6A TaskOutcomeMarket and SG-6A.1 through SG-6A.10
- EventResolveTx YES and EventResolveTx NO wording preserved
- REAL-6B design + scripted fixture only and no live real-LLM ship until explicit Class-4 ratification
- REAL-6C ChainTape-derived ConvictionBudget/PnL, no HashMap sidecar
- REAL-6D observe_only scheduler, no admission change, no price-as-truth
- REAL-7 minimum structural targets and SG-7.1 through SG-7.7
- REAL-8 A/B conditions, metrics, SG-8.1 through SG-8.6
- REAL-9 whitepaper/manual requirements
- forbidden list exactly includes no forced trades, no price-as-truth, no ghost liquidity, no f64 economy, no off-tape WAL as truth, no private CoT recording, no raw-log broadcast
- Class-4 Harness and implementation audit flow preserved

Lead with findings. End with Verdict: PROCEED | CHALLENGE | VETO.
Do not edit files.
```
