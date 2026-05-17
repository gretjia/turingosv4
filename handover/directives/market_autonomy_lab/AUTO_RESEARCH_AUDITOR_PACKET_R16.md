# Market Autonomy Lab R16 Auditor Packet

Prepared: 2026-05-16
Worktree: `/home/zephryj/projects/turingosv4-market-autonomy-lab`
Branch: `codex/market-autonomy-lab-20260516`

## Auditor Request

Please audit whether R16 can be treated as:

```text
E2 candidate pending audit
```

Do not audit it as an E2/E3/E4 achieved claim. This is Constitutional Research
Mode evidence only. It is not a main-branch merge authorization, not ship
evidence, and not a final TuringOS market-emergence claim.

Requested verdict format:

```text
PROCEED | CHALLENGE | VETO
```

Lead with findings and cite file/line evidence. Distinguish production defects,
test-scaffold/reporting defects, evidence gaps, and residual research risks.

## Claim Boundary

The narrow claim under review is:

```text
R16 contains live, non-scripted, agent-generated BuyWithCoinRouterTx evidence
with ChainTape/CAS/audit-dashboard provenance, and therefore may be labeled
"E2 candidate pending audit" if the auditor agrees the exact-join and shielding
evidence are sufficient.
```

The following are not claimed:

- E2 achieved.
- E3 achieved.
- E4 achieved.
- Market mechanism shipped.
- PolicyTrader or scripted fixture action counted as E2.
- Price used as Lean truth or predicate truth.
- Forced trade.
- Ghost liquidity.
- Off-tape WAL truth.
- Raw CoT, raw prompt, raw completion, or raw log broadcast.

Primary truth order for this packet remains ChainTape + CAS + executable gates.
Dashboard/report text is treated as a materialized view and must not override
ChainTape/CAS.

## Scope And Constitutional Boundary

Research envelope:
`/home/zephryj/projects/turingosv4-market-autonomy-lab/handover/directives/market_autonomy_lab/RESEARCH_ENVELOPE_V2.md`

Relevant envelope constraints:

- The envelope defines the Grand Objective and forbidden mechanisms, including
  no forced trade, no scripted/PolicyTrader baseline counted as E2,
  no price-as-truth, no ghost liquidity, and no f64/f32 money/probability in
  market paths
  (`RESEARCH_ENVELOPE_V2.md:22-27`).
- It explicitly permits Atom 3, the PolicyTrader deterministic baseline, but
  only as a baseline
  (`RESEARCH_ENVELOPE_V2.md:54`).
- It permits runner/report scaffolds for `scripts/run_real12_task_market_probe.sh`
  and `scripts/run_real13_market_pressure_probe.sh`, while forbidding those
  scripts from enabling forced trade, scripted buys, live REAL-6B,
  price-as-truth, or PolicyTrader/scripted actions counted as E2
  (`RESEARCH_ENVELOPE_V2.md:81-98`).
- It permits `genesis_payload.toml` only for Trust Root rehash of touched
  pinned files
  (`RESEARCH_ENVELOPE_V2.md:88-91`, `RESEARCH_ENVELOPE_V2.md:153`).
- It restates E2 candidate requirements: no forced trade, no price-as-truth,
  no ghost liquidity, not PolicyTrader, not scripted
  (`RESEARCH_ENVELOPE_V2.md:264-267`).

Touched flowchart surface, by function:

- FC1 runtime loop: role turns, market decisions, typed tx emission, predicates,
  and tape append.
- FC2 boot/evidence invariants: Trust Root verification and pinned hard10
  problem set.
- FC3 meta/audit loop: CAS-derived Librarian digest, dashboard materialization,
  clean-context audit, and next-hypothesis reporting.

Risk class:

- Class 3 for market/economic state, CAS/ChainTape evidence, and audit paths.
- Class 4 research-envelope behavior for touched Trust-Root-pinned surfaces and
  `genesis_payload.toml` rehash, limited to the preauthorized envelope.

## Active Evidence Package

Active candidate run:

```text
R16
evidence_dir=/home/zephryj/projects/turingosv4-market-autonomy-lab/handover/evidence/market_autonomy_lab_hard10_bcast_exact_join_R16_20260516T202248Z
run_tag=market_autonomy_lab_hard10_bcast_exact_join_R16_20260516T202248Z
problem_set=/home/zephryj/projects/turingosv4-market-autonomy-lab/handover/preregistration/sample_E1v2_hard10_S20260423.txt
problem_set_sha256=138f75bc3a776df11df45fed77c01565e128662ca03e3d863e68dcf8ee22d8fc
```

The hard10 problem set and hash are recorded in:

- `/home/zephryj/projects/turingosv4-market-autonomy-lab/handover/directives/market_autonomy_lab/E2_CANDIDATE_REPORT.md:18-19`
- `/home/zephryj/projects/turingosv4-market-autonomy-lab/handover/directives/market_autonomy_lab/TEST_RESULTS_SUMMARY.md:22-26`
- `/home/zephryj/projects/turingosv4-market-autonomy-lab/handover/directives/market_autonomy_lab/EXPERIMENT_MATRIX.md:8`

R16 run metadata:

- Batch id and run tag: `/home/zephryj/projects/turingosv4-market-autonomy-lab/handover/evidence/market_autonomy_lab_hard10_bcast_exact_join_R16_20260516T202248Z/run_log.txt:1-2`
- Problem count: 10 true MiniF2F/Lean problems
  (`run_log.txt:3`)
- Git head: `7b39499a6d416081d2eb5cae69cd9278a4fb72ed`
  (`run_log.txt:4`)
- Batch exit: 0
  (`run_log.txt:6`)
- Audit exit: 0
  (`run_log.txt:7`)
- Audit verdict: PROCEED
  (`run_log.txt:8`)
- Persistence passing: true, with 5 witnessed persistence checks
  (`run_log.txt:10-11`)

Aggregate audit verdict:

- `buy_with_coin_router`: 8
  (`/home/zephryj/projects/turingosv4-market-autonomy-lab/handover/evidence/market_autonomy_lab_hard10_bcast_exact_join_R16_20260516T202248Z/aggregate_verdict.json:24`)
- `market_seed`: 11
  (`aggregate_verdict.json:20`)
- `cpmm_pool`: 11
  (`aggregate_verdict.json:22`)
- `event_resolve`: 10
  (`aggregate_verdict.json:30`)
- Assertions: 41 passed, 0 failed
  (`aggregate_verdict.json:398-399`)
- Verdict: PROCEED
  (`aggregate_verdict.json:416`)

## Result Evidence From R16

### Market Tx Split And Candidate Action

The CAS-derived dashboard reports:

- `agent_economic_action_tx_count: 8`
  (`/home/zephryj/projects/turingosv4-market-autonomy-lab/handover/evidence/market_autonomy_lab_hard10_bcast_exact_join_R16_20260516T202248Z/audit_dashboard_run_report.txt:529`)
- `scripted_fixture_tx_count: 0`
  (`audit_dashboard_run_report.txt:531`)
- `matched_submitted_router_tx_id_count: 8`
  (`audit_dashboard_run_report.txt:535`)
- The E2 candidate rule requires live non-scripted router tx with
  MarketDecisionTrace submitted provenance and ChainTape/CAS audit, while
  scripted/unproven router tx is not E2
  (`audit_dashboard_run_report.txt:538`)

Interpretation for auditor:

The reported live economic action count is not a raw router count. It is the
exact intersection between L4 `BuyWithCoinRouter` tx ids and submitted
`MarketDecisionTrace` tx ids. This was introduced after R14 was challenged for
duplicate tx ids and overbroad dashboard counting.

### MarketDecisionTrace And EV Trace

R16 has:

- `MarketDecisionTrace` total 13, no_trade 5, submitted 8
  (`audit_dashboard_run_report.txt:552-556`)
- `persisted_market_opportunity_trace_cas_count: 112`
  (`audit_dashboard_run_report.txt:659`)
- `economic_judgment_total_cas: 112`
  (`audit_dashboard_run_report.txt:663`)
- `economic_judgment_buy_count_cas: 8`
  (`audit_dashboard_run_report.txt:667`)
- `ev_decision_trace_total_cas: 112`
  (`audit_dashboard_run_report.txt:670`)
- Bull/Bear EV trace split 56/56
  (`audit_dashboard_run_report.txt:671-672`)
- `ev_decision_trace_buy_yes_count_cas: 8`
  (`audit_dashboard_run_report.txt:673`)
- `ev_decision_trace_abstain_count_cas: 104`
  (`audit_dashboard_run_report.txt:675`)
- `ev_decision_reason_PositiveEV: 8`
  (`audit_dashboard_run_report.txt:676`)
- `ev_decision_reason_PositiveEVIgnored: 48`
  (`audit_dashboard_run_report.txt:685`)
- `ev_decision_reason_Unknown: 0`
  (`audit_dashboard_run_report.txt:689`)

Interpretation for auditor:

This is not a "buy count only" claim. R16 also preserves the clean-negative
mechanism signal: 104 abstains remain visible, including 48 PositiveEVIgnored
cases, which is important for next-hypothesis research and prevents the report
from hiding failures behind the 8 buys.

### PolicyTrader Baseline Exclusion

R16 reports:

- `policy_trader_trace_total_cas: 112`
  (`audit_dashboard_run_report.txt:692`)
- `policy_positive_ev_count: 56`
  (`audit_dashboard_run_report.txt:693`)
- `policy_positive_ev_llm_abstained_count: 48`
  (`audit_dashboard_run_report.txt:694`)
- `policy_no_positive_ev_count: 56`
  (`audit_dashboard_run_report.txt:695`)
- `policy_counts_for_e2=false`
  (`audit_dashboard_run_report.txt:697`)

Interpretation for auditor:

The deterministic PolicyTrader baseline is present, but it is explicitly
counterfactual and not counted as E2. It is diagnostic evidence for whether
actionable positive EV existed; it is not the source of the 8 agent economic
actions.

### Librarian Broadcast And Shielding

R16 had the BCAST arm enabled. Dashboard evidence:

- `librarian_digest_cas_count: 278`
  (`audit_dashboard_run_report.txt:702`)
- `librarian_role_crop_cas_count: 278`
  (`audit_dashboard_run_report.txt:703`)
- `librarian_market_reason_cluster_count: 831`
  (`audit_dashboard_run_report.txt:704`)
- `librarian_no_trade_reason_cluster_count: 554`
  (`audit_dashboard_run_report.txt:705`)
- `librarian_ev_reason_cluster_count: 1263`
  (`audit_dashboard_run_report.txt:706`)
- `librarian_shielding_verdict: PASS`
  (`audit_dashboard_run_report.txt:707`)

Interpretation for auditor:

The active candidate is R16, not R15, because R16 repeats the exact-join run
with Librarian broadcast on and shielding passing.

### CAS Schema Count Recheck

The CAS index at:

`/home/zephryj/projects/turingosv4-market-autonomy-lab/handover/evidence/market_autonomy_lab_hard10_bcast_exact_join_R16_20260516T202248Z/cas/.turingos_cas_index.jsonl`

was re-counted with:

```bash
jq -r '.schema_id // empty' handover/evidence/market_autonomy_lab_hard10_bcast_exact_join_R16_20260516T202248Z/cas/.turingos_cas_index.jsonl | sort | uniq -c | sort -nr
```

Relevant counts:

```text
278 v2/prompt_capsule_role_view
278 turingosv4.librarian_role_crop.v1
278 turingosv4.librarian_digest.v1
278 real5.prompt.visible_context.v1
277 real5.role_turn_trace.v1
242 TypedTx.v1
112 real13b.market_review_window.v1
112 real13b.market_review_summary.v1
112 real13b.market_review_response.v1
112 real13a.ev_decision_trace.v1
112 real13.policy_trader_trace.v1
112 real12.economic_judgment.v1
112 real11.market_opportunity_trace.v1
```

Interpretation for auditor:

PromptCapsule/visible-context/Librarian/MarketOpportunity/EV/PolicyTrader
objects are present in CAS. One residual audit question remains whether
PromptCapsule provenance is sufficient through role-turn and visible-context
links, rather than as a direct field on each submitted MarketDecisionTrace.

## Code Evidence

### Unique Router Tx Ids Include Task Identity

R14 was challenged partly because router tx ids could collide across tasks. The
evaluator now includes task identity in the router tx suffix:

- `/home/zephryj/projects/turingosv4-market-autonomy-lab/experiments/minif2f_v4/src/bin/evaluator.rs:6370`

The nearby router submission path captures the resulting `BuyWithCoinRouter`
tx id and writes the submitted market decision provenance:

- `/home/zephryj/projects/turingosv4-market-autonomy-lab/experiments/minif2f_v4/src/bin/evaluator.rs:6413-6417`
- `/home/zephryj/projects/turingosv4-market-autonomy-lab/experiments/minif2f_v4/src/bin/evaluator.rs:6435`
- `/home/zephryj/projects/turingosv4-market-autonomy-lab/experiments/minif2f_v4/src/bin/evaluator.rs:6448-6449`

Auditor check:

Confirm that this task-scoped tx-id construction is sufficient to close the R14
duplicate-router challenge and that no legacy unscoped router id path remains
active for this run mode.

### Dashboard Counts Agent Economic Action By Exact Join

The dashboard computes `MarketDecisionTraceSummary` from CAS:

- `/home/zephryj/projects/turingosv4-market-autonomy-lab/src/bin/audit_dashboard.rs:2480`

It collects submitted MarketDecisionTrace router tx ids:

- `/home/zephryj/projects/turingosv4-market-autonomy-lab/src/bin/audit_dashboard.rs:2500-2512`

It computes exact set intersection and duplicate counts:

- `/home/zephryj/projects/turingosv4-market-autonomy-lab/src/bin/audit_dashboard.rs:2519-2530`

It defines:

- `agent_economic_action_tx_count = matched_submitted_router_tx_id_count`
  (`/home/zephryj/projects/turingosv4-market-autonomy-lab/src/bin/audit_dashboard.rs:2534`)
- `scripted_or_unproven_router_tx_count = router_total - agent_economic_action`
  (`/home/zephryj/projects/turingosv4-market-autonomy-lab/src/bin/audit_dashboard.rs:2535-2536`)
- dashboard rendering for `agent_economic_action_tx_count`,
  `scripted_or_unproven_router_tx_count`, `scripted_fixture_tx_count`,
  submitted router tx ids, exact match count, and duplicate counts
  (`/home/zephryj/projects/turingosv4-market-autonomy-lab/src/bin/audit_dashboard.rs:2555-2578`)

Auditor check:

Confirm the dashboard no longer uses `min(router_total, submitted_count)` or a
constant scripted fixture count. The exact-join evidence is central to whether
R16 can be considered live agent action.

### EV Diagnostics And PositiveEVIgnored

The EV reason taxonomy includes:

- `PositiveEVIgnored`
  (`/home/zephryj/projects/turingosv4-market-autonomy-lab/src/runtime/ev_decision_trace.rs:49`)
- `ProbabilityUncalibrated`
  (`/home/zephryj/projects/turingosv4-market-autonomy-lab/src/runtime/ev_decision_trace.rs:51`)
- `Unknown`
  (`/home/zephryj/projects/turingosv4-market-autonomy-lab/src/runtime/ev_decision_trace.rs:53`)

The taxonomy is rendered exhaustively through the `all()` list:

- `/home/zephryj/projects/turingosv4-market-autonomy-lab/src/runtime/ev_decision_trace.rs:68-72`

Positive EV is derived through public integer constraints:

- `/home/zephryj/projects/turingosv4-market-autonomy-lab/src/runtime/ev_decision_trace.rs:143`

The validator rejects unstructured abstain:

- `/home/zephryj/projects/turingosv4-market-autonomy-lab/src/runtime/ev_decision_trace.rs:231-232`

Regression gates require the classifier and dashboard behavior:

- `/home/zephryj/projects/turingosv4-market-autonomy-lab/tests/constitution_real13a_ev_decision_trace.rs:187-198`
- `/home/zephryj/projects/turingosv4-market-autonomy-lab/tests/constitution_real13a_ev_decision_trace.rs:511-530`

Auditor check:

Confirm PositiveEVIgnored is computed from public actionable EV constraints,
not from a private rationale or declared EV sign alone.

### PolicyTrader Is Counterfactual And Integer-Only

PolicyTraderTrace is a dedicated runtime sidecar:

- `/home/zephryj/projects/turingosv4-market-autonomy-lab/src/runtime/policy_trader_trace.rs:1-12`

The trace carries `counterfactual_only` and `counts_for_e2` fields:

- `/home/zephryj/projects/turingosv4-market-autonomy-lab/src/runtime/policy_trader_trace.rs:57-59`

Validation rejects non-counterfactual PolicyTrader traces and traces counted for
E2:

- `/home/zephryj/projects/turingosv4-market-autonomy-lab/src/runtime/policy_trader_trace.rs:98-102`

The summary hard-codes `policy_counts_for_e2: false`:

- `/home/zephryj/projects/turingosv4-market-autonomy-lab/src/runtime/policy_trader_trace.rs:203-220`

Regression gates assert:

- dedicated module presence
  (`/home/zephryj/projects/turingosv4-market-autonomy-lab/tests/constitution_policy_trader_trace.rs:11-18`)
- integer-only policy path
  (`/home/zephryj/projects/turingosv4-market-autonomy-lab/tests/constitution_policy_trader_trace.rs:58-75`)
- PolicyTrader exclusion from E2
  (`/home/zephryj/projects/turingosv4-market-autonomy-lab/tests/constitution_policy_trader_trace.rs:81-99`)

Auditor check:

Confirm none of R16's 8 `agent_economic_action_tx_count` entries are
PolicyTrader or scripted fixture actions.

### Librarian Market/No-Trade Coverage And Shielding

The Librarian selector imports and handles MarketDecisionTrace and
MarketReviewSummary:

- `/home/zephryj/projects/turingosv4-market-autonomy-lab/src/runtime/librarian_broadcast.rs:18-20`
- `/home/zephryj/projects/turingosv4-market-autonomy-lab/src/runtime/librarian_broadcast.rs:259-287`
- `/home/zephryj/projects/turingosv4-market-autonomy-lab/src/runtime/librarian_broadcast.rs:309-312`

The shielding code removes/forbids raw prompt/completion/log material:

- `/home/zephryj/projects/turingosv4-market-autonomy-lab/src/runtime/librarian_broadcast.rs:204-214`
- `/home/zephryj/projects/turingosv4-market-autonomy-lab/src/runtime/librarian_broadcast.rs:744`
- `/home/zephryj/projects/turingosv4-market-autonomy-lab/src/runtime/librarian_broadcast.rs:980`

Regression tests cover:

- fail-closed unknown selector candidate
  (`/home/zephryj/projects/turingosv4-market-autonomy-lab/tests/constitution_librarian_selector.rs:71`)
- no raw prompt/completion in rendered broadcast
  (`/home/zephryj/projects/turingosv4-market-autonomy-lab/tests/constitution_librarian_selector.rs:98-99`)
- market/no-trade trace path
  (`/home/zephryj/projects/turingosv4-market-autonomy-lab/tests/constitution_librarian_market_no_trade.rs:13-18`)
- unknown schema fail-closed in CAS index
  (`/home/zephryj/projects/turingosv4-market-autonomy-lab/tests/constitution_librarian_market_no_trade.rs:175`)
- raw prompt/private CoT broadcast rejection
  (`/home/zephryj/projects/turingosv4-market-autonomy-lab/tests/constitution_librarian_market_no_trade.rs:231`)

Auditor check:

Confirm R16's Librarian digest path is CAS-derived, role-cropped, and does not
leak raw prompt, raw completion, raw CoT, raw Lean stderr, or raw logs into
ordinary agent prompt context.

## R14 To R16 Challenge Closure Timeline

R14 produced apparent live router rows but received a clean-context
`CHALLENGE`. The challenged issues were:

1. Duplicate router tx ids across tasks.
2. Dashboard counting based on broad router totals instead of exact submitted
   MarketDecisionTrace/L4 tx-id join.
3. Overclaiming wording in report scaffolds.
4. Stale G7 booleans not clearly annotated as non-sentinel for this research
   classification.

R15 then confirmed the exact-join repair with Librarian off:

- Evidence dir:
  `/home/zephryj/projects/turingosv4-market-autonomy-lab/handover/evidence/market_autonomy_lab_hard10_exact_join_R15_20260516T195846Z`
- Recorded in:
  `/home/zephryj/projects/turingosv4-market-autonomy-lab/handover/directives/market_autonomy_lab/E2_CANDIDATE_REPORT.md:155-159`

R16 repeated hard10 with Librarian on and became the active candidate:

- `/home/zephryj/projects/turingosv4-market-autonomy-lab/handover/directives/market_autonomy_lab/E2_CANDIDATE_REPORT.md:167`

The R16 challenge-closure audit is recorded as PROCEED:

- `/home/zephryj/projects/turingosv4-market-autonomy-lab/handover/directives/market_autonomy_lab/E2_CANDIDATE_REPORT.md:169-187`

Known closure caveats are explicitly retained:

- PromptCapsule provenance is linked through role-turn traces and
  visible-context CAS, not as a direct field on each submitted trace
  (`E2_CANDIDATE_REPORT.md:197-199`).
- Historical G7 booleans remain printed as false, but are annotated as
  non-sentinel when the guard CAS is absent for this Market Autonomy
  classification
  (`E2_CANDIDATE_REPORT.md:200-201`).

## Verification Evidence

Post-closure verification summary is recorded in:

- `/home/zephryj/projects/turingosv4-market-autonomy-lab/handover/directives/market_autonomy_lab/PRE_FLIGHT_REPORT.md:97-107`
- `/home/zephryj/projects/turingosv4-market-autonomy-lab/handover/directives/market_autonomy_lab/TEST_RESULTS_SUMMARY.md:275-280`

Commands reported passing:

```text
cargo fmt --all -- --check: PASS
git diff --check: PASS
Trust Root unit gate: PASS
research preflight: Level2 allowed Trust Root rehash checkpoint
constitution gates: 461 passed, 0 failed, 1 ignored
```

Targeted atom/regression tests previously passed:

```text
cargo test --test constitution_market_autonomy_research_envelope -- --test-threads=1
cargo test --test constitution_real12_task_market_action -- --test-threads=1
cargo test --test constitution_real13h_market_pressure_probe -- --test-threads=1
```

The dashboard regression gate for deriving scripted fixture count from CAS
rather than hard-coding it is:

- `/home/zephryj/projects/turingosv4-market-autonomy-lab/tests/constitution_real13h_market_pressure_probe.rs:106`

The REAL12 runner wording regression gate for economic judgment reason
distribution is:

- `/home/zephryj/projects/turingosv4-market-autonomy-lab/tests/constitution_real12_task_market_action.rs`

## Auditor Checklist

Please explicitly answer:

1. Does R16 satisfy the narrow evidence threshold for `E2 candidate pending
   audit`?
2. Is the `agent_economic_action_tx_count: 8` exact-join rule sufficient to
   distinguish live agent action from scripted/unproven router tx?
3. Are all 8 counted actions backed by submitted MarketDecisionTrace tx ids and
   L4 `BuyWithCoinRouter` tx ids?
4. Are PolicyTrader and scripted fixtures excluded from E2 counts?
5. Does the evaluator still permit abstain, or did any prompt/tool path force
   trade?
6. Is price observe-only, with no price-as-truth or predicate override?
7. Does Librarian BCAST avoid raw prompt/completion/CoT/log leakage?
8. Is the PromptCapsule provenance link sufficient, given it is via role-turn
   and visible-context CAS rather than a direct submitted-trace field?
9. Are the R14 challenge fixes materially present in code and reflected in R16
   evidence?
10. Are the residual G7 guard booleans correctly classified as non-sentinel for
    this Market Autonomy candidate?

## Residual Risks And Non-Blocking Research Gaps

These are intentionally not hidden:

- R16 is still only a candidate pending audit, not E2 achieved.
- PromptCapsule provenance is indirect through role-turn/visible-context CAS.
- The 8 live agent actions are YES-side buys; BearTrader remains zero buy-no in
  R16. This is not an E3 role-differentiation claim.
- `PositiveEVIgnored` remains high at 48, meaning the system still has a major
  EV execution bottleneck even after getting live buys.
- The run is hard10 only. It meets the minimum claim-bearing set but does not
  replace hard20/hard36 soak.
- Dashboard/report files remain materialized views. Auditor should prefer
  ChainTape/CAS/audit_tape when conflicts arise.

## Suggested Verdict Framing

If the auditor finds no blocking issue, the recommended narrow wording is:

```text
PROCEED for labeling R16 as "E2 candidate pending audit" under Constitutional
Research Mode.

This verdict does not approve "E2 achieved", main-branch ship, E3/E4 claims, or
any use of PolicyTrader/scripted actions as E2.
```

If the auditor challenges, please classify the issue as one of:

```text
production defect
evidence incompleteness
reporting/wording defect
test-scaffold gap
constitutional violation
```

and specify whether the issue is fixable inside
`MARKET_AUTONOMY_LAB_RESEARCH_ENVELOPE_V2`.
