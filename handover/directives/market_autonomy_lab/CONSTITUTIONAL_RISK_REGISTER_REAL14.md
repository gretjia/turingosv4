# REAL-14 Constitutional Risk Register

Scope: annotation-only register for forbidden overclaim and forbidden-mechanism
wording found by current REAL-14 / `market_autonomy_lab` scan.

Risk class: Class 0 documentation annotation. FC nodes touched: FC1 report/view
claim boundaries and FC3 shielding boundaries. This document makes no
architecture recommendation and changes no source, tests, evidence, historical
evidence, or authority surface.

| forbidden_phrase | path | line | status | replacement | risk_id | risk_class | evidence |
| --- | --- | ---: | --- | --- | --- | ---: | --- |
| `E2 achieved` | current REAL-14 lab docs | multiple | controlled wording risk | `E2 candidate pending audit` | REAL14-RISK-001 | 3 | Active docs use negated, forbidden-list, or candidate-pending-audit wording; no active achieved claim found. |
| `E3 achieved` / `E4 achieved` | current REAL-14 lab docs | multiple | controlled wording risk | `no E3/E4 claim` | REAL14-RISK-002 | 3 | Current packets say E3/E4 are not claimed and not approved by the candidate verdict. |
| `market mechanism shipped` / `ship evidence` | current REAL-14 lab docs | multiple | controlled wording risk | `research evidence only` | REAL14-RISK-003 | 3 | Current packets distinguish research evidence from ship evidence. |
| `spontaneous market emergence` | `handover/reports/REAL_BCAST_1_AND_HARD10_STRESS_REPORT.md` | 311 | boundary wording; not active proof claim | `E2 candidate pending audit` until clean-context audit and separate approval | REAL14-RISK-004 | 3 | Report hit is treated as forbidden/negative boundary language under the current scan. |
| `forced trade` | current REAL-14 lab docs | multiple | forbidden mechanism guarded | `no forced trade`; `voluntary live non-scripted agent action only` | REAL14-RISK-005 | 4 | Hits are stop triggers, explicit prohibitions, or sentinels; no active mechanism requirement found. |
| `price-as-truth` | current REAL-14 lab docs | multiple | forbidden mechanism guarded | `price as signal only` | REAL14-RISK-006 | 4 | Hits are stop triggers, explicit prohibitions, or sentinels; no predicate/Lean truth claim found in docs. |
| `ghost liquidity` | current REAL-14 lab docs | multiple | forbidden mechanism guarded | `budget-backed/on-tape settled liquidity only` | REAL14-RISK-007 | 3 | Hits are stop triggers, explicit prohibitions, or sentinels; no active unbacked-liquidity claim found in docs. |
| `PolicyTrader.*E2` | current REAL-14 lab docs | multiple | forbidden mechanism guarded | `PolicyTrader baseline excluded from E2` | REAL14-RISK-008 | 3 | Hits say PolicyTrader is baseline-only or not counted as E2. |
| `scripted.*E2` | current REAL-14 lab docs | multiple | forbidden mechanism guarded | `scripted fixture excluded from E2` | REAL14-RISK-009 | 3 | Hits say scripted/counterfactual action is not counted as E2. |
| `raw CoT` / `raw prompt` / `raw completion` / `raw log` | current REAL-14 lab docs | multiple | shielding mechanism guarded | `shielded scoped summaries and CIDs only` | REAL14-RISK-010 | 3 | Hits are shielding prohibitions, auditor questions, or non-claim lists; no active broadcast authorization found. |
| old REAL-10/11/12/13 wording | `handover/directives`, `handover/reports` | multiple | historical / out of REAL-14 scan scope | annotate only; do not edit historical evidence | REAL14-RISK-011 | 0 | User requested historical immutable evidence be annotated, not edited; this register records the boundary only. |

## Active Violations

Zero active forbidden overclaims remain in current non-historical REAL-14 docs
based on the required scans. Historical immutable evidence and older
REAL-10/11/12/13 wording were not edited.

