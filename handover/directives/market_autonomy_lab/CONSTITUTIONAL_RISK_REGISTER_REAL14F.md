# REAL-14F Constitutional Risk Register

Scope: annotation-only register for REAL-14F forbidden overclaim and
forbidden-mechanism scan results.

Risk class: Class 0 documentation annotation. Touched FC nodes/invariants:
FC1 report/materialized-view claim boundary, FC2 evidence-not-dashboard truth
boundary, and FC3 shielding/claim-boundary discipline. No source code,
historical evidence, ChainTape/CAS evidence, sequencer admission, typed tx
schema, signing payload, CAS schema, constitution, or flowchart authority was
edited.

Decision authority: none. This register does not approve, reject, ship, or
upgrade any REAL-14F claim.

| forbidden_phrase | path | line | status | replacement | risk_id | risk_class | evidence |
| --- | --- | ---: | --- | --- | --- | ---: | --- |
| `E2 achieved` | non-historical REAL-14F docs | multiple | controlled wording risk; no active overclaim found | `E2 candidate pending audit` only after audit/ratification | REAL14F-RISK-001 | 3 | Required scan found negated, quoted, forbidden-list, prior-scan, or historical wording only. |
| `E3 achieved` / `E4 achieved` | non-historical REAL-14F docs | multiple | controlled wording risk; no active overclaim found | `no E3/E4 claim` | REAL14F-RISK-002 | 3 | Current packets list E3/E4 achieved as not claimed or not approved. |
| `market emergence proven` / `spontaneous market emergence` | `handover/directives`, `handover/reports` | multiple | historical/report-boundary wording; no REAL-14F proof claim found | `E2 candidate pending audit` until clean-context audit and separate approval | REAL14F-RISK-003 | 3 | Hits are older directives/reports, negations, or preserved report boundary wording; no edit made. |
| `market mechanism shipped` | non-historical REAL-14F docs | multiple | controlled wording risk; no ship claim found | `research evidence only` | REAL14F-RISK-004 | 3 | Decision/auditor packets explicitly deny ship-mechanism wording. |
| `forced trade` | non-historical REAL-14F docs | multiple | hard-stop/guard wording; no active mechanism authorization found | `voluntary live non-scripted agent action only` | REAL14F-RISK-005 | 4 | ARH-v2 stop policy and REAL-14F follow-up text forbid forced trade. |
| `price-as-truth` | non-historical REAL-14F docs | multiple | hard-stop/guard wording; no predicate-truth authorization found | `price as signal only` | REAL14F-RISK-006 | 4 | Hits forbid price-as-truth or state collateral-backed parameters without price-as-truth. |
| `ghost liquidity` | non-historical REAL-14F docs | multiple | hard-stop/guard wording; no unbacked-liquidity authorization found | `budget-backed/on-tape settled liquidity only` | REAL14F-RISK-007 | 3 | Hits forbid ghost liquidity or require collateral-backed parameters. |
| `PolicyTrader.*E2` | non-historical REAL-14F docs | multiple | numerator contamination guard; no active overclaim found | `PolicyTrader baseline excluded from E2` | REAL14F-RISK-008 | 3 | R18 verifier has `policy_counts_for_e2=false`; packets and stop policy forbid counting PolicyTrader as E2. |
| `scripted.*E2` | non-historical REAL-14F docs | multiple | numerator contamination guard; no active overclaim found | `scripted fixture excluded from E2` | REAL14F-RISK-009 | 3 | Packets and stop policy forbid scripted action counted as E2. |
| `raw CoT` / `raw prompt` / `raw completion` / `raw log` | non-historical REAL-14F docs | multiple | shielding guard; no broadcast authorization found | `shielded summaries/CIDs only` | REAL14F-RISK-010 | 3 | Hits are forbidden lists, hard stops, auditor checks, or shielding boundaries. |
| old REAL-10/11/12/13 wording | `handover/directives`, `handover/reports` | multiple | historical / out of REAL-14F active-claim scope | annotate only; do not edit historical evidence | REAL14F-RISK-011 | 0 | Required scans hit older directives/reports; historical evidence was preserved. |
| prior REAL-14 scan/register self-hits | `handover/directives/market_autonomy_lab/FORBIDDEN_CLAIMS_REAL14.md`, `handover/directives/market_autonomy_lab/CONSTITUTIONAL_RISK_REGISTER_REAL14.md` | multiple | quoted scan/register rows | keep as prior scan annotation | REAL14F-RISK-012 | 0 | Prior scan documents contain forbidden phrases as table keys/evidence, not active claims. |
| new REAL-14F packet forbidden-list hits | `REAL14F_DECISION_PACKET.md`, `REAL14F_E2_CANDIDATE_REPORT.md`, `REAL14F_METRICS_REAL14F.json`, `REAL14F_CLEAN_CONTEXT_AUDIT.md` | multiple | negated or metadata-only forbidden-claims lists | keep candidate-only boundary | REAL14F-RISK-013 | 3 | Required scan hits are explicit denials or JSON forbidden-claims metadata; no active overclaim found. |

## Active Violations

Zero active forbidden overclaims remain in non-historical REAL-14F docs based on
the required scans. Historical/quoted/negated wording was annotated and not
edited.
