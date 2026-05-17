# REAL-14H Constitutional Risk Register

scope: REAL-14H frozen REAL-14G replication and side-balance evidence
risk_class: Class 3 research evidence under `MARKET_AUTONOMY_LAB_RESEARCH_ENVELOPE_V2`

| risk_id | risk | class | evidence | mitigation | status |
| --- | --- | --- | --- | --- | --- |
| REAL14H-RISK-001 | Premature achieved/proven/ship overclaim | 3 | REAL-14H supports candidate labels only | Use `E2 replicated candidate` and `Two-sided market candidate`; forbid achieved/proven/shipped wording | controlled |
| REAL14H-RISK-002 | Prompt provenance indirect | 3 | verifier reports `indirect_via_ev_decision_trace` for matched rows | Keep as residual risk; do not overstate direct PromptCapsule field | accepted residual |
| REAL14H-RISK-003 | Multiple EV rows on some BullTrader matches | 3 | verifier residual risks on several Bull rows | Exact router tx_id join disambiguates count; future provenance tightening may reduce ambiguity | accepted residual |
| REAL14H-RISK-004 | Dashboard G7 guard ambiguity | 2 | G7 false booleans in N/A context with `g7_guard_cas_count=0` | Treat dashboard as materialized view only; exact verifier/audit_tape remain source of gate truth | accepted residual |
| REAL14H-RISK-005 | E3 not established | 3 | REAL-14H shows market action, not stable multi-batch role distributions | Open REAL-15 role differentiation study | open |
| REAL14H-RISK-006 | E4 not established | 3 | No pinned A/B performance benchmark in REAL-14H | Defer E4 until after E3 candidate | open |
| REAL14H-RISK-007 | PolicyTrader counted as E2 | 3 | verifier `policy_counts_for_e2=false` | Keep verifier mandatory for all E2 candidate/replication claims | controlled |
| REAL14H-RISK-008 | Forced trade contamination | 3 | no scripted fixture, no PolicyTrader E2 count, no forced-trade evidence found | Continue scans; VETO contaminated evidence if forced trade appears | controlled |
| REAL14H-RISK-009 | Price-as-truth contamination | 3 | `price_index_is_view_only` passed | Keep price observe-only and non-predicate | controlled |

## Hard Stop Review

No Level 3 Constitutional Hard Stop was reached in REAL-14H.

No evidence indicates:

```text
forced trade counted as E2
price-as-truth
ghost liquidity
off-tape truth
raw CoT/raw prompt/raw completion/raw log broadcast
f64/f32 market money path
PolicyTrader/scripted action counted as E2
```
