# REAL-14F Forbidden Claims Scan

Scope: REAL-14F forbidden-claims scan over `/home/zephryj/projects/turingosv4-market-autonomy-lab`.

Risk class: Class 0 documentation annotation. Touched FC nodes/invariants:
FC1 report/materialized-view claim boundary, FC2 evidence-not-dashboard truth
boundary, and FC3 shielding/claim-boundary discipline. No source code,
historical evidence, ChainTape/CAS evidence, sequencer admission, typed tx
schema, signing payload, CAS schema, constitution, or flowchart authority was
edited.

Decision authority: none. This file records scan annotations only.

Required read-only scan evidence:

```bash
rg -n "E2 achieved|market emergence proven|market mechanism shipped|E3 achieved|E4 achieved|spontaneous market emergence" handover/directives handover/reports
rg -n "forced trade|price-as-truth|ghost liquidity|PolicyTrader.*E2|scripted.*E2|raw CoT|raw prompt|raw completion|raw log" handover/directives handover/reports
```

Result: zero active forbidden overclaims found in non-historical REAL-14F docs.
Historical, quoted, negated, forbidden-list, stop-policy, and prior REAL-14
boundary wording is annotated here and not edited.

| forbidden_phrase | path | line | status | replacement | risk_id | risk_class | evidence |
| --- | --- | ---: | --- | --- | --- | ---: | --- |
| `E2 achieved` | `handover/directives/market_autonomy_lab/CLEAN_NEGATIVE_REPORT_REAL14_R17_R18.md` | 3 | REAL-14F context; negated candidate boundary | `R16 candidate only; R17/R18 clean-negative` | REAL14F-CLAIM-001 | 3 | File says `E2 candidate pending audit` applies only for R16; R17/R18 are clean-negative replication/ablation runs. |
| `forced trade` | `handover/directives/market_autonomy_lab/CLEAN_NEGATIVE_REPORT_REAL14_R17_R18.md` | 38 | REAL-14F follow-up guard; negated mechanism | `no forced trade` | REAL14F-CLAIM-005 | 4 | Follow-up says EV basis stabilization must not force action. |
| `price-as-truth` | `handover/directives/market_autonomy_lab/CLEAN_NEGATIVE_REPORT_REAL14_R17_R18.md` | 38 | REAL-14F follow-up guard; negated mechanism | `price as signal only` | REAL14F-CLAIM-006 | 4 | Follow-up forbids price-as-truth. |
| `ghost liquidity` | `handover/directives/market_autonomy_lab/CLEAN_NEGATIVE_REPORT_REAL14_R17_R18.md` | 39 | REAL-14F follow-up guard; negated mechanism | `budget-backed/on-tape liquidity only` | REAL14F-CLAIM-007 | 3 | Follow-up forbids ghost liquidity. |
| `raw prompt/completion/CoT/log` | `handover/directives/market_autonomy_lab/CLEAN_NEGATIVE_REPORT_REAL14_R17_R18.md` | 39 | REAL-14F follow-up guard; shielding boundary | `shielded summaries/CIDs only` | REAL14F-CLAIM-010 | 3 | Follow-up forbids raw prompt/completion/CoT/log broadcast. |
| `PolicyTrader/scripted action counted as E2` | `handover/directives/market_autonomy_lab/CLEAN_NEGATIVE_REPORT_REAL14_R17_R18.md` | 40 | REAL-14F follow-up guard; negated numerator | `PolicyTrader/scripted excluded from E2` | REAL14F-CLAIM-008 | 3 | Follow-up says PolicyTrader/scripted action must not count as E2. |
| `PolicyTrader.*E2` | `handover/directives/market_autonomy_lab/CLEAN_NEGATIVE_REPORT_REAL14_R17_R18.md` | 35 | REAL-14F clean-negative guard; negated numerator | `PolicyTrader baseline excluded from E2` | REAL14F-CLAIM-008 | 3 | Report says PolicyTrader remains counterfactual and excluded from E2. |
| `E2 achieved` | `handover/directives/market_autonomy_lab/FORBIDDEN_CLAIMS.md` | 9 | quoted forbidden phrase | `E2 candidate pending audit` | REAL14F-CLAIM-001 | 3 | Existing forbidden-claims table says never in this lab without later audit and separate approval. |
| `E2 achieved` | `handover/directives/market_autonomy_lab/FORBIDDEN_CLAIMS.md` | 36 | archived-source exception | `E2 candidate pending audit` | REAL14F-CLAIM-001 | 3 | Source archive exception says quoted architect-source strings are not current lab claims. |
| `E2 achieved` | `handover/directives/market_autonomy_lab/REAL14_DECISION_PACKET.md` | 5 | negated forbidden-claims list | `E2 candidate pending audit` | REAL14F-CLAIM-001 | 3 | Decision packet lists no E2 achieved, no market emergence proven, no market mechanism shipped, no E3/E4 achieved. |
| `market emergence proven` | `handover/directives/market_autonomy_lab/REAL14_DECISION_PACKET.md` | 5 | negated forbidden-claims list | `E2 candidate pending audit after audit/ratification only` | REAL14F-CLAIM-004 | 3 | Same line forbids market-emergence proof wording. |
| `market mechanism shipped` | `handover/directives/market_autonomy_lab/REAL14_DECISION_PACKET.md` | 5 | negated forbidden-claims list | `research evidence only` | REAL14F-CLAIM-003 | 3 | Same line forbids shipped-mechanism wording. |
| `E3/E4 achieved` | `handover/directives/market_autonomy_lab/REAL14_DECISION_PACKET.md` | 5 | negated forbidden-claims list | `no E3/E4 claim` | REAL14F-CLAIM-002 | 3 | Same line forbids E3/E4 achieved wording. |
| `E2 achieved` | `handover/directives/market_autonomy_lab/REAL14_DECISION_PACKET.md` | 50 | negated audit boundary | `E2 candidate pending audit` | REAL14F-CLAIM-001 | 3 | Audit row says verdict permits only candidate wording, not achieved wording. |
| `ghost liquidity` | `handover/directives/market_autonomy_lab/REAL14_DECISION_PACKET.md` | 141 | negated design boundary | `collateral-backed market parameters` | REAL14F-CLAIM-007 | 3 | Line says collateral-backed parameters without ghost liquidity or price-as-truth. |
| `price-as-truth` | `handover/directives/market_autonomy_lab/REAL14_DECISION_PACKET.md` | 141 | negated design boundary | `price as signal only` | REAL14F-CLAIM-006 | 4 | Line says collateral-backed parameters without ghost liquidity or price-as-truth. |
| `forced trade` | `handover/directives/market_autonomy_lab/ARH_V2_STOP_POLICY.md` | 98 | hard-stop trigger, not authorization | `stop before forced trade` | REAL14F-CLAIM-005 | 4 | Stop policy lists forced trade requirement as Level 3 hard stop. |
| `forced/scripted/PolicyTrader action would be counted as E2` | `handover/directives/market_autonomy_lab/ARH_V2_STOP_POLICY.md` | 99 | hard-stop trigger, not authorization | `scripted/PolicyTrader excluded from E2` | REAL14F-CLAIM-008 | 3 | Stop policy makes this a hard stop. |
| `ghost liquidity` | `handover/directives/market_autonomy_lab/ARH_V2_STOP_POLICY.md` | 101 | hard-stop trigger, not authorization | `budget-backed/on-tape liquidity only` | REAL14F-CLAIM-007 | 3 | Stop policy makes ghost liquidity requirement a hard stop. |
| `raw prompt/completion/CoT/log` | `handover/directives/market_autonomy_lab/ARH_V2_STOP_POLICY.md` | 104 | hard-stop trigger; shielding boundary | `shielded summaries/CIDs only` | REAL14F-CLAIM-010 | 3 | Stop policy makes raw broadcast a hard stop. |
| `E2 achieved` | `handover/directives/market_autonomy_lab/AUTO_RESEARCH_AUDITOR_PACKET_R16.md` | 15 | prior R16 audit boundary; historical/current quoted non-claim | `E2 candidate pending audit` | REAL14F-CLAIM-001 | 3 | Auditor packet says do not audit as achieved. |
| `E2 achieved` | `handover/directives/market_autonomy_lab/AUTO_RESEARCH_AUDITOR_PACKET_R16.md` | 41 | non-claimed list | `E2 candidate pending audit` | REAL14F-CLAIM-001 | 3 | Packet lists E2 achieved as not claimed. |
| `E3 achieved` | `handover/directives/market_autonomy_lab/AUTO_RESEARCH_AUDITOR_PACKET_R16.md` | 42 | non-claimed list | `no E3 claim` | REAL14F-CLAIM-002 | 3 | Packet lists E3 achieved as not claimed. |
| `E4 achieved` | `handover/directives/market_autonomy_lab/AUTO_RESEARCH_AUDITOR_PACKET_R16.md` | 43 | non-claimed list | `no E4 claim` | REAL14F-CLAIM-002 | 3 | Packet lists E4 achieved as not claimed. |
| `Market mechanism shipped` | `handover/directives/market_autonomy_lab/AUTO_RESEARCH_AUDITOR_PACKET_R16.md` | 44 | non-claimed list | `research evidence only` | REAL14F-CLAIM-003 | 3 | Packet lists market mechanism shipped as not claimed. |
| `PolicyTrader or scripted fixture action counted as E2` | `handover/directives/market_autonomy_lab/AUTO_RESEARCH_AUDITOR_PACKET_R16.md` | 45 | non-claimed list | `PolicyTrader/scripted excluded from E2` | REAL14F-CLAIM-008 | 3 | Packet lists this as not claimed. |
| `Raw CoT, raw prompt, raw completion, or raw log broadcast` | `handover/directives/market_autonomy_lab/AUTO_RESEARCH_AUDITOR_PACKET_R16.md` | 50 | non-claimed list; shielding boundary | `shielded summaries/CIDs only` | REAL14F-CLAIM-010 | 3 | Packet lists raw broadcast as not claimed. |
| `E2 achieved` | `handover/directives/market_autonomy_lab/REAL14F_DECISION_PACKET.md` | 5 | negated forbidden-claims list | `E2 candidate pending audit` | REAL14F-CLAIM-001 | 3 | REAL-14F packet explicitly forbids achieved wording. |
| `market emergence proven` | `handover/directives/market_autonomy_lab/REAL14F_DECISION_PACKET.md` | 5 | negated forbidden-claims list | `E2 candidate pending audit` | REAL14F-CLAIM-004 | 3 | REAL-14F packet explicitly forbids market-emergence proof wording. |
| `market mechanism shipped` | `handover/directives/market_autonomy_lab/REAL14F_DECISION_PACKET.md` | 6 | negated forbidden-claims list | `research evidence only` | REAL14F-CLAIM-003 | 3 | REAL-14F packet explicitly forbids shipped-mechanism wording. |
| `E3 achieved` / `E4 achieved` | `handover/directives/market_autonomy_lab/REAL14F_DECISION_PACKET.md` | 6 | negated forbidden-claims list | `no E3/E4 claim` | REAL14F-CLAIM-002 | 3 | REAL-14F packet explicitly forbids E3/E4 achieved wording. |
| `E2 achieved` / `market emergence proven` / `market mechanism shipped` / `E3/E4 achieved` | `handover/directives/market_autonomy_lab/REAL14F_E2_CANDIDATE_REPORT.md` | 5 | negated claim-boundary list | `E2 candidate pending audit` | REAL14F-CLAIM-001 | 3 | Candidate report says candidate only and forbids achieved/proven/shipped wording. |
| `E2 achieved` / `market emergence proven` / `market mechanism shipped` / `E3/E4 achieved` | `handover/directives/market_autonomy_lab/REAL14F_METRICS_REAL14F.json` | 42 | quoted forbidden list field | `claim_boundary=E2 candidate pending audit` | REAL14F-CLAIM-001 | 3 | JSON records forbidden strings as forbidden-claims metadata, not active claims. |
| `E2 achieved` / `market emergence proven` / `market mechanism shipped` / `E3/E4 achieved` | `handover/directives/market_autonomy_lab/REAL14F_CLEAN_CONTEXT_AUDIT.md` | 64 | negated claim-boundary list | `E2 candidate pending audit` | REAL14F-CLAIM-001 | 3 | Clean-context audit records these strings as forbidden, not as active claims. |
| `PolicyTrader.*E2` | `handover/directives/market_autonomy_lab/REAL14F_DECISION_PACKET.md` | 93 | negated numerator guard | `PolicyTrader excluded from E2` | REAL14F-CLAIM-008 | 3 | Next recommendation says do not count PolicyTrader as E2. |
| `spontaneous market emergence` | `handover/reports/REAL_BCAST_1_AND_HARD10_STRESS_REPORT.md` | 311 | historical/current report boundary phrase; not REAL-14F active proof | `E2 candidate pending audit after audit/ratification only` | REAL14F-CLAIM-004 | 3 | Search hit is preserved as report wording; not edited. |
| `price-as-truth` | `handover/reports/REAL_BCAST_1_AND_HARD10_STRESS_REPORT.md` | 316 | historical/current report boundary phrase; not authorization | `price as signal only` | REAL14F-CLAIM-006 | 4 | Search hit is preserved as report wording; not edited. |
| old REAL-10/11/12/13 `E2 achieved` / `scripted.*E2` / forbidden-mechanism wording | `handover/directives`, `handover/reports` | multiple | historical or pre-REAL-14 context; annotated, not edited | `do not use as REAL-14F active claim` | REAL14F-CLAIM-011 | 0 | Required scans found older directive/report hits; user explicitly prohibited deleting or rewriting historical evidence. |
| existing REAL-14 scan/register self-hits | `handover/directives/market_autonomy_lab/FORBIDDEN_CLAIMS_REAL14.md`, `handover/directives/market_autonomy_lab/CONSTITUTIONAL_RISK_REGISTER_REAL14.md` | multiple | quoted scan/register rows | keep as historical scan annotations | REAL14F-CLAIM-012 | 0 | Prior scan ledgers contain forbidden phrases as table keys and evidence annotations, not active claims. |

## Active Violations

None found in non-historical REAL-14F docs from the required scans.
