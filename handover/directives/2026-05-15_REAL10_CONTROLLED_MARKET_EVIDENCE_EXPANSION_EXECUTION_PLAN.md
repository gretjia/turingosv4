# REAL-10 Controlled Market Evidence Expansion — Orchestrated Execution Plan

## Summary

This plan implements the architect's two 2026-05-15 replies as a single
Class-4 package. The claim boundary is deliberately narrow:

```text
REAL-5S -> REAL-9 is ratified as:
  chain-backed role scaffold
  lawful market-pressure substrate
  structural pressure smoke
  descriptive A/B benchmark
  launch synthesis

REAL-5S -> REAL-9 is NOT ratified as:
  spontaneous market emergence
  causal performance improvement
  autonomous secondary market already alive
  model ranking
  price-as-truth
  unconstrained DeFi behavior
  real-world readiness
  live REAL-6B real-LLM AttemptPrediction approval
```

REAL-10 does not add a new market mechanism. It expands controlled evidence:
first close traceability and test-scaffold gaps, then run a larger pinned REAL-8
A/B benchmark, then classify outcomes with explicit E1/E2/E3/E4 metrics.

## Agent Topology

All agents use GPT-5.5; only reasoning depth changes.

| Role | Model / effort | Responsibility | Edit authority |
|---|---:|---|---|
| Orchestrator | GPT-5.5 high, xhigh when needed | truth order, Class-4 boundary, task split, integration, evidence interpretation, final claim boundary | main workspace |
| Docs Worker | GPT-5.5 low | archive architect originals, approved plan, narrow ratification, forbidden claim list | templated docs only |
| Trace Worker | GPT-5.5 medium | R-022 skip parsing, TRACE/§J cleanup design, regression test shape | reviewed before merge |
| Behavior Test Worker | GPT-5.5 high | stale-parent behavioral test design over Sequencer/QState/VerifyTx | reviewed before merge |
| Benchmark Worker | GPT-5.5 medium | REAL-8X runner/report extension, arm manifests, metric extraction | reviewed before merge |
| Analysis Worker | GPT-5.5 high | Wilson CI, PPUT, waste metrics, NoTradeReason, PnL dispersion, decision gate | reports only |
| Audit Worker | GPT-5.5 xhigh | plan / gap / evidence / ship audits with `PROCEED | CHALLENGE | VETO` | no implementation |

Audit is not per-atom. It happens at:

1. plan alignment before implementation;
2. after TRACE/R-022 + stale-parent closure;
3. after REAL-8X evidence and claim-boundary report;
4. final clean-context implementation review.

## Harness Contract

Open a `turingos_dev` run:

```text
module: REAL-10 Controlled Market Evidence Expansion
risk: 4
unit: atom
fc_nodes:
  FC1 market/action evidence
  FC2 replay/pinning
  FC3 report/materialized views
  Art. III shielding
  market/economic gates
```

Edit-allowed paths:

```text
handover/directives/2026-05-15_REAL10_CONTROLLED_MARKET_EVIDENCE_EXPANSION_ARCHITECT_ORIGINAL.md
handover/directives/2026-05-15_REAL10_CONTROLLED_MARKET_EVIDENCE_EXPANSION_EXECUTION_PLAN.md
handover/directives/2026-05-15_REAL5S_REAL9_NARROW_RATIFICATION.md
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md
handover/alignment/TRACE_MATRIX_BACKLINK_CLEANUP_REAL5S_REAL9.md
handover/alignment/EMERGENCE_METRICS_E1_E2_E3_E4.md
handover/audits/CODEX_REAL10_EXECUTION_PLAN_ALIGNMENT_REVIEW.md
handover/audits/CODEX_REAL10_TRACE_STALE_PARENT_REVIEW.md
handover/audits/CODEX_REAL10_EVIDENCE_CLAIM_REVIEW.md
handover/audits/CODEX_REAL10_IMPLEMENTATION_REVIEW.md
handover/reports/REAL10_DECISION_GATE_REPORT.md
handover/reports/REAL10_OVERCLAIM_BOUNDARY_REPORT.md
tests/constitution_real8_market_ab_benchmark.rs
tests/constitution_real10_trace_cleanup.rs
tests/constitution_real10_emergence_metrics.rs
scripts/run_real8_market_ab_benchmark.sh
handover/evidence/real10_*
handover/evidence/dev_self_hosting/*
```

Stop and re-ratify if implementation needs:

```text
src/state/sequencer.rs
src/state/typed_tx.rs
canonical signing payloads
src/kernel.rs
src/bus.rs
src/sdk/tools/wallet.rs
src/bottom_white/cas/schema.rs
genesis_payload.toml
constitution / flowchart authority documents
live REAL-6B real-LLM AttemptPrediction
```

## Phase 0 — Provenance And Plan Audit

Artifacts:

```text
handover/directives/2026-05-15_REAL10_CONTROLLED_MARKET_EVIDENCE_EXPANSION_ARCHITECT_ORIGINAL.md
handover/directives/2026-05-15_REAL10_CONTROLLED_MARKET_EVIDENCE_EXPANSION_EXECUTION_PLAN.md
handover/audits/CODEX_REAL10_EXECUTION_PLAN_ALIGNMENT_REVIEW.md
```

Gate:

```text
Plan alignment audit verdict = PROCEED
```

## Atom 0 — Narrow REAL-5S -> REAL-9 Ratification

Create:

```text
handover/directives/2026-05-15_REAL5S_REAL9_NARROW_RATIFICATION.md
```

Must include current REAL-8 empirical facts:

```text
Arms: A/B/C/D
tasks per arm: 3
exit: all 0
audit: all PROCEED
market_tx_count: A=0, B=4, C=10, D=10
solve_rate: 2/3 in every arm
interpretation: market activity increased; no solve-rate separation
```

SG-10.0:

```text
SG-10.0.1 Ratification file exists.
SG-10.0.2 Claim boundary matches REAL-9 docs.
SG-10.0.3 No E2/E3 overclaim.
SG-10.0.4 No price-as-truth claim.
SG-10.0.5 No live REAL-6B approval.
```

## Atom 1 — TRACE_MATRIX / R-022 Backlink Cleanup

Create:

```text
handover/alignment/TRACE_MATRIX_BACKLINK_CLEANUP_REAL5S_REAL9.md
```

Implementation:

1. Parse `rules/enforcement.log` for `R-022-SKIP` rows referencing
   `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md`.
2. Normalize skipped symbols:
   - `pub_fn_foo` -> `foo`
   - `pub_struct_Foo` -> `Foo`
   - `pub_enum_Foo` -> `Foo`
   - `pub_mod_foo` -> `foo`
   - `trace_removal` -> cleanup-document entry, not a symbol row.
3. Prefer docs-first §J registration in `TRACE_MATRIX_v3_2026-04-27.md` instead
   of editing restricted Rust surfaces.
4. Add a regression test proving every REAL-5S -> REAL-9 skipped public surface
   has TRACE/§J coverage.
5. Record that R-022 skip was a one-time cleanup exception, not a policy waiver.

SG-10.1:

```text
SG-10.1.1 No new public surface lacks TRACE/§J backlink.
SG-10.1.2 R-022 not treated as waiver.
SG-10.1.3 Future hook would not require skip for same class of change.
```

## Atom 2 — Stale-parent Direct Behavioral Test

Update:

```text
tests/constitution_real8_market_ab_benchmark.rs
```

Add:

```text
real8_task_outcome_arm_refreshes_verify_parent_behaviorally
```

Test logic:

1. Create a Sequencer fixture.
2. Accept a WorkTx and capture `post_work_root`.
3. Apply a legitimate state mutation representing optional market emission.
4. Build VerifyTx with stale `post_work_root`; assert sequencer rejects it.
5. Build VerifyTx with refreshed `q_snapshot().state_root_t`; assert sequencer
   accepts it.
6. Keep the old source-grep test only as a secondary guard.

SG-10.2:

```text
SG-10.2.1 Old path fails with stale parent.
SG-10.2.2 Refreshed path passes.
SG-10.2.3 Source-grep test is no longer primary evidence.
```

## Atom 3 — Emergence Metrics

Create:

```text
handover/alignment/EMERGENCE_METRICS_E1_E2_E3_E4.md
tests/constitution_real10_emergence_metrics.rs
```

Definitions:

```text
E1 — Market Visibility
  Agent sees market context.
  MarketDecisionTrace / NoTradeReason is tape-visible.

E2 — Spontaneous Market Action
  At least one live, non-scripted, agent-generated BuyWithCoinRouterTx
  or short-equivalent.
  Must be ChainTape/CAS visible.
  No forced trade.
  No scripted action.
  Audit PROCEED.

E3 — Persistent Role Differentiation
  At least two roles show persistent, distinct action distributions across tasks.
  Derived from ChainTape/CAS, not prompt labels.
  Must persist across at least two consecutive tasks or batches.

E4 — Causal Performance Signal
  Market-enabled condition shows statistically meaningful difference in PPUT,
  solve rate, cost, wasted attempts, or verification latency under pinned inputs.
```

SG-10.3:

```text
SG-10.3.1 Metrics doc exists.
SG-10.3.2 Reports use E1/E2/E3/E4 terminology.
SG-10.3.3 Scripted actions cannot satisfy E2.
SG-10.3.4 Role labels alone cannot satisfy E3.
SG-10.3.5 Small-n descriptive evidence cannot claim E4.
```

## Atom 4 — REAL-8X 15-task/arm Benchmark

Update:

```text
scripts/run_real8_market_ab_benchmark.sh
tests/constitution_real8_market_ab_benchmark.rs
```

Arms stay unchanged:

```text
A: market disabled
B: market visible, no TaskOutcomeMarket
C: TaskOutcomeMarket enabled
D: TaskOutcomeMarket + scripted AttemptPrediction fixture
```

Hard pinning:

```text
same problem set
same model assignment
same budgets
same timeout
same max_tx
same seed/config
only arm toggles differ
```

Add per-arm config manifests and a diff-allowlist test. The allowlist may only
permit arm toggles such as market enabled/disabled, TaskOutcomeMarket enabled,
scripted AttemptPrediction fixture enabled, and arm label/output paths.

Metric formulas and allowed sources:

| Metric | Formula / source |
|---|---|
| `solve_rate` | `solved_tasks / total_tasks`, derived from ChainTape/CAS run facts |
| `verified_pput` | `verified_proofs / total_prompt_tokens` |
| `mean_pput_solved` | mean PPUT over solved tasks only |
| `wilson_ci_solve_rate` | Wilson 95% interval over solved/total |
| `market_tx_count` | accepted market typed tx count from ChainTape |
| `no_trade_reason_distribution` | NoTradeReason traces from CAS/ChainTape references |
| `pnl_dispersion_micro` | dispersion over ChainTape-derived PnL view |
| `role_diversity_index` | role action distribution, not prompt labels alone |
| `audit_failure_rate` | failed audits / total tasks |
| `cost_time` | run manifests plus chain-backed task duration facts |
| `failed_branch_count` | chain/CAS-derived failed proof/action branch count |
| `verification_latency` | logical ticks between WorkTx and VerifyTx/EventResolve |
| `wasted_attempts` | rejected or non-verifying attempts per solved task |

SG-10.4:

```text
SG-10.4.1 15 tasks per arm.
SG-10.4.2 All arms audit PROCEED.
SG-10.4.3 Inputs pinned by hash.
SG-10.4.4 Arm config diff only contains allowlisted toggles.
SG-10.4.5 Report is descriptive unless CI supports stronger claim.
SG-10.4.6 No E2 claim unless live non-scripted router tx exists.
SG-10.4.7 No E4 claim without defined statistical support.
SG-10.4.8 Dashboard/report regenerated from ChainTape + CAS.
```

## Atom 5 — REAL-8X Decision Gate

Create:

```text
handover/reports/REAL10_DECISION_GATE_REPORT.md
```

Branch logic:

```text
If market activity increases but solve/PPUT does not separate:
  allowed conclusion = lawful market machinery active; no performance gain yet.

If market activity remains mainly scripted and no live E2 occurs:
  allowed conclusion = spontaneous market action not achieved.
  possible next = live REAL-6B Class-4 packet or stronger Trader utility/PnL visibility.

If live non-scripted router/short tx occurs:
  allowed conclusion = E2 achieved.
  possible next = study E3 persistent role differentiation.

If market-enabled arm regresses:
  allowed conclusion = market context may distract Solvers.
  possible next = strengthen role-scoped views.

If waste metrics improve but solve-rate does not:
  allowed conclusion = possible information-efficiency signal.
  forbidden conclusion = causal performance gain unless E4 statistical support exists.
```

## Atom 6 — Optional REAL-8Y 30-task/arm

Only after REAL-8X is audit-clean.

SG-10.6:

```text
SG-10.6.1 30 tasks per arm.
SG-10.6.2 EvidencePackagingPolicy satisfied.
SG-10.6.3 sampled full replay works.
SG-10.6.4 failure-heavy sample replay works.
SG-10.6.5 solved sample replay works.
SG-10.6.6 unsolved sample replay works.
SG-10.6.7 no hidden excluded runs.
```

## Atom 7 — Live REAL-6B Packet Is Deferred

REAL-10 must not run live real-LLM AttemptPrediction.

If later required, create a separate Class-4 packet containing:

```text
candidate timing
market close timing
oracle resolution order
settlement semantics
abort path
replay invariants
no price-as-truth proof
explicit Class-4 ratification
```

Forbidden:

```text
sleep-based timing
price affecting Lean verification
market price deciding L4/L4.E
```

## Verification

Targeted:

```bash
cargo test --test constitution_real8_market_ab_benchmark
cargo test --test constitution_real10_trace_cleanup
cargo test --test constitution_real10_emergence_metrics
```

Broad:

```bash
bash scripts/run_constitution_gates.sh
cargo test --workspace --no-fail-fast -- --test-threads=1
```

Evidence:

```bash
bash scripts/run_real8_market_ab_benchmark.sh \
  real8x_15_task_<UTC> \
  --tasks-per-arm 15 \
  --arms A,B,C,D \
  --out handover/evidence/real10_real8x_15_task_<UTC>
```

Overclaim tests must fail if any REAL-10 report claims:

```text
autonomous market emergence
causal performance improvement
model ranking
real-world readiness
live REAL-6B approval
price-as-truth
forced trade
ghost liquidity
off-tape WAL as truth
private CoT recording
raw-log broadcast
```

## Launch Boundary

Allowed internal milestone:

```text
TuringOS Research Alpha:
  ChainTape-backed Lean task economy
  role-based agent framework
  lawful market experiment harness
  no real money
  no public chain
  no spontaneous emergence claim
```

Not allowed:

```text
autonomous market emergence claim
real-money market
public chain settlement
real-world task execution
market improves solving claim
```

Research Alpha gates:

```text
constitution gates green
REAL-10 benchmark complete
TRACE_MATRIX debt closed
launch docs forbid overclaim
dashboard regenerates all reported facts from ChainTape/CAS
```

## Final Audit Rule

After implementation evidence exists, run clean-context Codex review with:

```text
task brief
risk class
touched FC nodes/invariants
current diff/commit
evidence paths
exact verification command output
required verdict: PROCEED | CHALLENGE | VETO
```

`VETO` blocks ship. `CHALLENGE` requires fix or explicit forward deferral.
`PROCEED` is necessary but not a substitute for gates/evidence.
