# REAL-14G Constitutional Risk Register

scope: REAL-14G PositiveEVIgnored / action-conversion stabilization
risk_class: Class 3 research evidence under `MARKET_AUTONOMY_LAB_RESEARCH_ENVELOPE_V2`

| risk_id | risk | class | evidence | mitigation | status |
| --- | --- | --- | --- | --- | --- |
| REAL14G-RISK-001 | Premature E2/E3/E4 or emergence overclaim | 3 | REAL-14G exact_join_count is 8, but buy_no=0 and E3/E4 not tested | Use only `E2 candidate pending audit`; forbid achieved/proven/shipped wording | controlled |
| REAL14G-RISK-002 | YES-side-only behavior | 3 | `buy_yes_count=8`, `buy_no_count=0` | Open REAL-14H side-balance/BuyNo probe; do not claim two-sided market | open |
| REAL14G-RISK-003 | PositiveEVIgnored remains high | 3 | 17 ignored positive-EV rows, all `ModelAbstentionDespiteClearBasis` | Continue action-conversion diagnostics; do not force trades | open |
| REAL14G-RISK-004 | Indirect PromptCapsule provenance | 3 | verifier reports `indirect_via_ev_decision_trace` | Keep residual risk explicit; future atom may add direct field if within envelope | accepted residual |
| REAL14G-RISK-005 | Duplicate matching EV rows for first two tx ids | 3 | clean-context audit observed two matching EVDecisionTrace rows for first two tx ids | Exact tx_id join disambiguates count; keep as residual risk | accepted residual |
| REAL14G-RISK-006 | Dashboard/report ambiguity | 2 | G7 guard rows are false in N/A context where `g7_guard_cas_count=0` | Treat dashboard as materialized view only; verifier remains source of gate truth | accepted residual |
| REAL14G-RISK-007 | Resource pressure | 2 | runner warned low disk free space before hard10 | Monitor before hard20/hard36; resource exhaustion remains hard-stop clause | open |
| REAL14G-RISK-008 | PolicyTrader accidentally counted as E2 | 3 | verifier JSON has `policy_counts_for_e2=false` | Keep exact-join verifier as required gate for all candidate claims | controlled |
| REAL14G-RISK-009 | Forced-trade contamination | 3 | TraderView wording is optional; no must-buy/must-short command | Continue forbidden wording scan; VETO contaminated evidence if forced trade appears | controlled |
| REAL14G-RISK-010 | Price-as-truth contamination | 3 | `price_index_is_view_only` passed; dashboard labels price as signal only | Keep price observe-only; no predicate or Lean acceptance dependency | controlled |

## Hard Stop Review

No Level 3 Constitutional Hard Stop was reached in REAL-14G.

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
