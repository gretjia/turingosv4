# REAL-14 Forbidden Claims Scan

Scope: current non-historical REAL-14 / `market_autonomy_lab` directives and
current market-autonomy reports under `handover/directives` and
`handover/reports`.

Risk class: Class 0 documentation annotation. FC nodes touched: FC1 materialized
claim/report views and FC3 shielding claim boundaries only. No source, tests,
evidence, historical evidence, architecture, sequencer admission, typed tx
schema, signing payload, CAS schema, or constitution surfaces are changed.

Read-only scan evidence:

```bash
rg -n "E2 achieved|market emergence proven|market mechanism shipped|E3 achieved|E4 achieved|ship evidence|spontaneous market emergence" handover/directives handover/reports
rg -n "forced trade|price-as-truth|ghost liquidity|PolicyTrader.*E2|scripted.*E2|raw CoT|raw prompt|raw completion|raw log" handover/directives handover/reports
```

Result: zero active forbidden overclaims found in current non-historical
REAL-14 / `market_autonomy_lab` docs. Historical or older REAL-10/11/12/13
documents are not edited here; old wording is treated as historical/archive
context or already-negated boundary language.

| forbidden_phrase | path | line | status | replacement | risk_id | risk_class | evidence |
| --- | --- | ---: | --- | --- | --- | ---: | --- |
| `E2 achieved` | `handover/directives/market_autonomy_lab/FORBIDDEN_CLAIMS.md` | 9 | allowed quoted forbidden phrase | `E2 candidate pending audit` | REAL14-CLAIM-001 | 3 | Row says never in this lab without later clean-context audit and separate approval. |
| `E2 achieved` | `handover/directives/market_autonomy_lab/FORBIDDEN_CLAIMS.md` | 36 | archived-source exception | `E2 candidate pending audit` | REAL14-CLAIM-001 | 3 | Source Archive Exception says quoted architect-source strings are not current lab claims. |
| `E2 achieved` | `handover/directives/market_autonomy_lab/CODEX_GOAL_PROMPT.md` | 10 | negated boundary | `E2 candidate pending audit` | REAL14-CLAIM-001 | 3 | Line says success is a candidate pending audit, not immediate E2 achieved. |
| `E2/E3/E4 achieved` | `handover/directives/market_autonomy_lab/CODEX_GOAL_PROMPT.md` | 38 | negated boundary | `E2 candidate pending audit`; no E3/E4 claim | REAL14-CLAIM-002 | 3 | Line forbids claiming E2/E3/E4 achieved. |
| `E2 achieved` | `handover/directives/market_autonomy_lab/CODEX_GOAL_PROMPT.md` | 113 | forbidden wording list | `E2 candidate pending audit` | REAL14-CLAIM-001 | 3 | Line says never write this phrase from the lab. |
| `E2/E3/E4 achieved` | `handover/directives/market_autonomy_lab/RESEARCH_ENVELOPE_V2.md` | 10 | negated boundary | `E2 candidate pending audit`; no E3/E4 claim | REAL14-CLAIM-002 | 3 | Line says the envelope is not permission to claim achieved status. |
| `E2 achieved` | `handover/directives/market_autonomy_lab/RESEARCH_ENVELOPE_V2.md` | 271 | forbidden wording list | `E2 candidate pending audit` | REAL14-CLAIM-001 | 3 | Line says never write this phrase from the research envelope. |
| `E2/E3/E4 achieved` | `handover/directives/market_autonomy_lab/STATUS_SYNC_OR_OBS.md` | 40 | negated boundary | `E2 candidate pending audit`; no E3/E4 claim | REAL14-CLAIM-002 | 3 | Status says not authorization to claim achieved status. |
| `E2 achieved` | `handover/directives/market_autonomy_lab/STATUS_SYNC_OR_OBS.md` | 159 | negated boundary | `E2 candidate pending audit` | REAL14-CLAIM-001 | 3 | Line says not E2 achieved. |
| `ship evidence` | `handover/directives/market_autonomy_lab/STATUS_SYNC_OR_OBS.md` | 160 | negated boundary | `research evidence only` | REAL14-CLAIM-003 | 3 | Line says not ship evidence. |
| `E2 achieved` | `handover/directives/market_autonomy_lab/STATUS_SYNC_OR_OBS.md` | 191 | negated boundary | `E2 candidate pending audit` | REAL14-CLAIM-001 | 3 | Line says not E2 achieved. |
| `ship evidence` | `handover/directives/market_autonomy_lab/STATUS_SYNC_OR_OBS.md` | 192 | negated boundary | `research evidence only` | REAL14-CLAIM-003 | 3 | Line says not ship evidence. |
| `E2 achievement` | `handover/directives/market_autonomy_lab/STATUS_SYNC_OR_OBS.md` | 274 | negative conclusion | `no E2 achievement` | REAL14-CLAIM-001 | 3 | Conclusion says zero live non-scripted router tx and no E2 achievement. |
| `E2 achieved` | `handover/directives/market_autonomy_lab/TEST_RESULTS_SUMMARY.md` | 158 | negative conclusion | `does not support E2 achieved` | REAL14-CLAIM-001 | 3 | Line says R14 does not support E2 achieved. |
| `E2 achieved` | `handover/directives/market_autonomy_lab/TEST_RESULTS_SUMMARY.md` | 244 | negative conclusion | `does not support E2 achieved` | REAL14-CLAIM-001 | 3 | Line says the evidence does not support the claim. |
| `E2 achieved` | `handover/directives/market_autonomy_lab/NEXT_STEP_RECOMMENDATION.md` | 129 | negated boundary | `E2 candidate pending audit` | REAL14-CLAIM-001 | 3 | Line says use candidate wording and do not claim E2 achieved. |
| `E2 achieved` | `handover/directives/market_autonomy_lab/E2_CANDIDATE_REPORT.md` | 9 | negated boundary | `E2 candidate pending audit` | REAL14-CLAIM-001 | 3 | Line says this is not an E2 achieved claim. |
| `E2 achieved` | `handover/directives/market_autonomy_lab/E2_CANDIDATE_REPORT.md` | 191 | negated boundary | `E2 candidate pending audit` | REAL14-CLAIM-001 | 3 | Line says the report does not authorize E2 achieved. |
| `E2 achieved` | `handover/directives/market_autonomy_lab/E2_CANDIDATE_REPORT.md` | 205 | negated boundary | `research evidence only` | REAL14-CLAIM-003 | 3 | Line says research evidence must not become E2 achieved or ship evidence. |
| `E2 achieved` | `handover/directives/market_autonomy_lab/E2_CANDIDATE_REPORT.md` | 219 | forbidden wording list | `E2 candidate pending audit` | REAL14-CLAIM-001 | 3 | The surrounding section labels this as forbidden wording. |
| `ship evidence` | `handover/directives/market_autonomy_lab/E2_CANDIDATE_REPORT.md` | 221 | forbidden wording list | `research evidence only` | REAL14-CLAIM-003 | 3 | The surrounding section labels this as forbidden wording. |
| `E2 achieved` | `handover/directives/market_autonomy_lab/AUTO_RESEARCH_AUDITOR_PACKET_R16.md` | 41 | non-claimed list | `E2 candidate pending audit` | REAL14-CLAIM-001 | 3 | The packet says these are not claimed. |
| `E3 achieved` | `handover/directives/market_autonomy_lab/AUTO_RESEARCH_AUDITOR_PACKET_R16.md` | 42 | non-claimed list | `no E3 claim` | REAL14-CLAIM-002 | 3 | The packet says these are not claimed. |
| `E4 achieved` | `handover/directives/market_autonomy_lab/AUTO_RESEARCH_AUDITOR_PACKET_R16.md` | 43 | non-claimed list | `no E4 claim` | REAL14-CLAIM-002 | 3 | The packet says these are not claimed. |
| `Market mechanism shipped` | `handover/directives/market_autonomy_lab/AUTO_RESEARCH_AUDITOR_PACKET_R16.md` | 44 | non-claimed list | `research candidate pending audit` | REAL14-CLAIM-003 | 3 | The packet says this is not claimed. |
| `E2 achieved` | `handover/directives/market_autonomy_lab/AUTO_RESEARCH_AUDITOR_PACKET_R16.md` | 537 | negated boundary | `E2 candidate pending audit` | REAL14-CLAIM-001 | 3 | Line says R16 is still only a candidate pending audit, not E2 achieved. |
| `E2 achieved`, `E3/E4 claims`, `ship` | `handover/directives/market_autonomy_lab/AUTO_RESEARCH_AUDITOR_PACKET_R16.md` | 556 | negated boundary | `E2 candidate pending audit`; no E3/E4/ship claim | REAL14-CLAIM-002 | 3 | Verdict scope explicitly does not approve these claims. |
| `spontaneous market emergence` | `handover/reports/REAL_BCAST_1_AND_HARD10_STRESS_REPORT.md` | 311 | current report forbidden/negative boundary | `E2 candidate pending audit` | REAL14-CLAIM-004 | 3 | Search hit is current stress-report boundary language, not an achieved claim. |

## Active Violations

None found in current non-historical REAL-14 / `market_autonomy_lab` docs.

