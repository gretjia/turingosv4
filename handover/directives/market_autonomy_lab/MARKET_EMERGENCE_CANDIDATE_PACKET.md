# Market Emergence Candidate Packet

REAL-17 main-CAS integration note: this packet is preserved as historical
market-autonomy context from the older pre-CAS-repair worktree. The candidate
evidence paths cited below were intentionally not migrated into the new
main-based REAL-17 worktree and are not forward claim-bearing on the updated
CAS Git commit-chain baseline. REAL-17 must regenerate any stronger
market-emergence packet from repaired-baseline ChainTape/CAS evidence.

claim_boundary: `market emergence candidate pending final audit`

This packet is candidate-only. It does not claim `market emergence proven`,
`market mechanism shipped`, `E2 achieved`, `E3 achieved`, or `E4 achieved`.

## Scope

This packet composes prior audited candidate evidence:

1. replicated live non-scripted agent-generated economic action,
2. voluntary YES-side and NO/short-side behavior,
3. stable role differentiation,
4. candidate performance-pressure signal,
5. no forbidden mechanism in the accepted evidence boundary.

## E2 Replicated Candidate

Evidence:

- REAL-14G:
  `handover/evidence/market_autonomy_lab_hard10_real14G_action_conversion_20260517T022457Z/REAL14G_E2_CANDIDATE_VERIFIER_EXPECT8.json`
- REAL-14H:
  `handover/evidence/market_autonomy_lab_hard10_real14H_frozen_real14G_replication_20260517T025400Z/REAL14H_E2_CANDIDATE_VERIFIER_EXPECT13.json`

Structured verifier facts:

| run | verifier | exact_join | scripted fixtures | PolicyTrader counted | BCAST shielding |
| --- | --- | ---: | ---: | --- | --- |
| REAL-14G | PROCEED | 8 | 0 | false | PASS |
| REAL-14H | PROCEED | 13 | 0 | false | PASS |

Claim boundary:

```text
E2 replicated candidate
```

## Two-Sided Market Candidate

REAL-14H contains voluntary YES and NO-side action. The NO-side rows are
BearTrader `BuyNo` actions with EV `PositiveEV`, MarketOpportunityTrace,
EVDecisionTrace, and indirect PromptCapsule linkage:

```text
router-task-outcome-Agent_1-task-n5_amc12_2000_p12_1778986645493-Agent_1-6
router-task-outcome-Agent_1-task-n5_mathd_algebra_208_1778986920491-Agent_1-1
```

Claim boundary:

```text
Two-sided market candidate
```

## E3 Candidate

Evidence:

- `handover/directives/market_autonomy_lab/REAL15_ROLE_DIFFERENTIATION_REAL14G_REAL14H.json`
- `handover/directives/market_autonomy_lab/REAL15_CLEAN_CONTEXT_AUDIT.md`

Structured verifier facts:

```text
verdict = PROCEED
e3_candidate = true
run_count = 2
audit_tape_proceed_count = 2
persistent_active_role_count = 2
distinct_action_signature_count = 2
```

Role evidence:

```text
BullTrader: active_run_count=2, exact_join_market_action_count=19, buy_yes_count=19
Solver: active_run_count=2, work_count=34
BearTrader: buy_no_count=2, active_run_count=1, residual side-balance evidence
Verifier: verify_count=2, active_run_count=1
```

Claim boundary:

```text
E3 candidate pending audit
```

## E4 Candidate

Evidence:

- `handover/evidence/market_autonomy_lab_real16_abcd_oppfix_recovery_combined_20260517T150000Z/REAL16_MARKET_PERFORMANCE_REPORT.json`
- `handover/directives/market_autonomy_lab/REAL16_CLEAN_CONTEXT_AUDIT.md`

Structured verifier facts:

```text
verdict = Proceed
claim_boundary = E4 candidate pending audit
best_arm_id = D
improved_metrics =
  - wasted_attempts
  - failed_branch_count
  - ev_to_action_conversion
```

Arm summary:

| arm | E2 verifier | exact_join | solved | verified PPUT | wasted | failed branches |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| A | VETO control no-candidate | 0 | 0 | 0 | 173 | 182 |
| B | PROCEED | 9 | 0 | 0 | 180 | 188 |
| C | PROCEED | 3 | 0 | 0 | 166 | 178 |
| D | PROCEED | 16 | 0 | 0 | 149 | 158 |

Residual limit:

```text
solve_rate did not improve
verified PPUT did not improve
all arms solved_count = 0
```

Claim boundary:

```text
E4 candidate pending audit
```

## Constitutional Boundary

The accepted evidence boundary excludes:

- forced trade,
- price-as-truth,
- price-driven Lean accept/reject,
- ghost liquidity,
- off-tape truth,
- raw CoT/raw prompt/raw completion/raw log broadcast,
- f64/f32 market money path,
- scripted or PolicyTrader action counted as E2/E4,
- live REAL-6B unless separately ratified.

## Residual Risks

- REAL-16 D is a recovery arm because the original full-run D was contaminated
  by ENOSPC. The packet discloses and separates this.
- REAL-16 E4 candidate is based on reduced wasted attempts, fewer failed
  branches, and EV-to-action conversion, not solve-rate or verified PPUT.
- PromptCapsule linkage for E2 rows is indirect through EVDecisionTrace where
  MarketDecisionTrace lacks a direct PromptCapsule field.
- BearTrader two-sided evidence exists, but side balance is not yet as stable
  as YES-side behavior.

## Requested Final Audit Question

```text
Can this packet be labeled market emergence candidate pending final audit?
```

Allowed final wording if audit returns `PROCEED`:

```text
market emergence candidate pending final audit
```

Forbidden wording:

```text
market emergence proven
market mechanism shipped
E2 achieved
E3 achieved
E4 achieved
```
