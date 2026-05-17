# Market Autonomy Lab E2 Candidate Report

## Status

```text
E2 candidate pending audit
```

This is not an `E2 achieved` claim, not a ship claim, and not authorization to
merge main. The candidate is R16, not R14. R14 was challenged; R15 repaired the
exact-join issue with Librarian off; R16 repeated hard10 with Librarian on.

## Evidence Boundary

```text
candidate_run: R16
evidence_dir: handover/evidence/market_autonomy_lab_hard10_bcast_exact_join_R16_20260516T202248Z
problem_set: handover/preregistration/sample_E1v2_hard10_S20260423.txt
problem_set_sha256: 138f75bc3a776df11df45fed77c01565e128662ca03e3d863e68dcf8ee22d8fc
run_tag: market_autonomy_lab_hard10_bcast_exact_join_R16_20260516T202248Z
batch_exit: 0
elapsed_s: 1405
audit_tape: PROCEED
audit_assertions: passed=41 failed=0 halted=0 skipped=11
persistence: is_passing=true n_witnessed=5
clean_context_audit_r1: CHALLENGE
clean_context_challenge_closure_audit: PROCEED
```

R16 uses the constitutional research arm with:

```text
EV scaffold: on
Librarian: on
PnL / role-specialized traders: on
BullTrader / BearTrader / Solver / Verifier / Challenger roles
forced trade: off
scripted buys: off
PolicyTrader counts_for_e2: false
live REAL-6B: off
```

## Candidate Basis

| Requirement | R16 Evidence | Status |
| --- | --- | --- |
| live non-scripted router / short-equivalent tx | `buy_with_coin_router=8`, `agent_economic_action_tx_count=8` | pending audit |
| exact submitted trace join | `matched_submitted_router_tx_id_count=8`, `submitted_market_decision_router_tx_ids=8` | pending audit |
| no duplicate tx-id overcount | `duplicate_router_tx_id_count=0`, `duplicate_submitted_router_tx_id_count=0` | pending audit |
| not scripted fixture | `scripted_fixture_tx_count=0`, `scripted_or_unproven_router_tx_count=0` | pending audit |
| not PolicyTrader counted as E2 | `policy_counts_for_e2=false` | pending audit |
| ChainTape/CAS evidence | `aggregate_verdict.json` has `buy_with_coin_router=8`; dashboard derives counts from ChainTape/CAS | pending audit |
| PromptCapsule provenance | CAS index has `v2/prompt_capsule_role_view=278`, visible context `278`, role-turn trace `277` | pending audit |
| MarketOpportunityTrace | `persisted_market_opportunity_trace_cas_count=112` | pending audit |
| EVDecisionTrace / rationale | `ev_decision_trace_total_cas=112`, `ev_decision_trace_buy_yes_count_cas=8`, `EVReason::PositiveEV=8` | pending audit |
| Librarian market/no-trade broadcast | `LibrarianDigest=278`, role crops `278`, shielding `PASS` | pending audit |
| audit_tape PROCEED | `aggregate_verdict.json` verdict `PROCEED` | satisfied |
| no forced trade / price-as-truth / ghost liquidity | sentinels in run report; no Coin minted post-init; §K absent guard annotated N/A | pending audit |

## Key Metrics

| Metric | Value |
| --- | ---: |
| `structural_market_tx_count` | 22 |
| `agent_economic_action_tx_count` | 8 |
| `scripted_or_unproven_router_tx_count` | 0 |
| `scripted_fixture_tx_count` | 0 |
| `resolution_tx_count` | 10 |
| `buy_with_coin_router_count` | 8 |
| `submitted_market_decision_router_tx_ids` | 8 |
| `matched_submitted_router_tx_id_count` | 8 |
| `duplicate_router_tx_id_count` | 0 |
| `duplicate_submitted_router_tx_id_count` | 0 |
| `router_buy_yes` | 8 |
| `router_buy_no` | 0 |
| `MarketOpportunityTrace` | 112 |
| `EconomicJudgment` | 112 |
| `Bull / Bear EconomicJudgment` | 56 / 56 |
| `EVDecisionTrace` | 112 |
| `MarketReviewSummary` | 112 |
| `PolicyTraderTrace` | 112 |
| `LibrarianDigest / RoleCrop` | 278 / 278 |

EV reason distribution:

```text
PositiveEV: 8
PositiveEVIgnored: 48
NegativeEV: 48
ProbabilityUncalibrated: 8
Unknown: 0
```

Policy baseline comparison:

```text
policy_positive_ev_count: 56
policy_positive_ev_llm_abstained_count: 48
policy_no_positive_ev_count: 56
policy_insufficient_public_basis_count: 0
policy_counts_for_e2: false
```

Librarian broadcast:

```text
librarian_digest_cas_count: 278
librarian_role_crop_cas_count: 278
librarian_market_reason_cluster_count: 831
librarian_no_trade_reason_cluster_count: 554
librarian_ev_reason_cluster_count: 1263
librarian_shielding_verdict: PASS
```

CAS schema counts:

```text
v2/prompt_capsule_role_view: 278
real5.prompt.visible_context.v1: 278
real5.role_turn_trace.v1: 277
TypedTx.v1: 242
real13a.ev_decision_trace.v1: 112
real13.policy_trader_trace.v1: 112
real12.economic_judgment.v1: 112
real11.market_opportunity_trace.v1: 112
```

Post-CHALLENGE regeneration check:

```text
command:
  cargo run --quiet --bin audit_dashboard -- --repo R16/runtime_repo --cas R16/cas --run-report > /tmp/r16_post_challenge_dashboard.txt
result:
  agent_economic_action_tx_count: 8
  scripted_fixture_tx_count: 0
  scripted_attempt_prediction_market_count: 0
  matched_submitted_router_tx_id_count: 8
  duplicate_router_tx_id_count: 0
  librarian_digest_cas_count: 278
```

## R14 Challenge Closure

R14 clean-context audit returned `CHALLENGE`. The closure path is:

```text
router suffix now includes task identity
dashboard action count uses exact L4 BuyWithCoinRouter / submitted MarketDecisionTrace tx-id join
duplicate router/submitted tx-id counters are reported
scripted_fixture_tx_count is no longer used as a proxy for unproven router tx
runner/report wording uses "E2 candidate pending audit" only
§K absent G7 structural guard is annotated as N/A / non-sentinel
```

R15 confirmed the exact-join fix with Librarian off:

```text
evidence_dir: handover/evidence/market_autonomy_lab_hard10_exact_join_R15_20260516T195846Z
audit_tape: PROCEED
agent_economic_action_tx_count: 13
matched_submitted_router_tx_id_count: 13
duplicate_router_tx_id_count: 0
duplicate_submitted_router_tx_id_count: 0
librarian_digest_cas_count: 0
```

R16 repeated the hard10 with Librarian on and is the active candidate.

R16 audit CHALLENGE closure status:

```text
P1 envelope/report-runner boundary:
  RESEARCH_ENVELOPE_V2 now lists the REAL-12/REAL-13 runner scripts and
  constrains h_vppu_history.json to non-authoritative H-VPPU side-effect use.
P2 scripted fixture count:
  dashboard now renders scripted_fixture_tx_count from CAS-derived
  scripted_attempt_prediction_market_count, not a hard-coded zero.
P3 report label drift:
  future REAL-12 reports use economic_judgment_reason_distribution for all
  EconomicJudgment reason rows.
  The immutable R16 evidence-local report still carries the old label; it is
  treated as an annotated report-scaffold gap, not as candidate evidence.
clean-context closure audit:
  PROCEED
```

The closure audit found no production defects for the R16 CHALLENGE-closure
scope and no evidence of forced trade, price-as-truth, ghost liquidity, live
REAL-6B, scripted/PolicyTrader action counted as E2, raw prompt/completion/CoT
or log broadcast, or off-tape truth. This remains research evidence only and
does not authorize `E2 achieved`.

## Audit Focus

The clean-context reviewer must decide whether any of these remain blockers:

1. Submitted `MarketDecisionTrace` objects prove submitted tx IDs; PromptCapsule
   provenance is linked through role-turn traces and visible-context CAS, not as
   a direct field on each submitted trace.
2. §K still prints historical G7 smoke booleans as false, but now annotates the
   absent G7 guard CAS as non-sentinel for this Market Autonomy classification.
3. R16 uses BullTrader YES-side action only; BearTrader activity remains
   abstain-only. This can be E2-candidate evidence for live voluntary action but
   not E3 role differentiation.
4. This is research evidence only. It must not become `E2 achieved` or ship
   evidence without later approval.

## Boundary

Allowed wording:

```text
E2 candidate pending audit
```

Forbidden wording:

```text
E2 achieved
E2 candidate achieved
ship evidence
production-ready autonomy
causal E4 performance gain
```
