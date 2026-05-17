# REAL-16 Constitutional Risk Register

## Risk Table

| id | risk | status | mitigation | evidence |
| --- | --- | --- | --- | --- |
| REAL16-R1 | Dashboard/stdout becomes source of truth | controlled | REAL-16 verifier derives metrics from ChainTape/CAS/exact-join verifier inputs; tests forbid dashboard-only and stdout-only sources | `tests/constitution_real16_market_performance.rs` |
| REAL16-R2 | Control-arm invalid router match counted as agent action | controlled | control arm E2 verifier VETO is normalized to `exact_join_count=0`; market-pressure arms still require E2 verifier `PROCEED` | `src/runtime/market_performance_e4.rs` |
| REAL16-R3 | Full-run D evidence contamination after ENOSPC | mitigated with separation | full-run D excluded; recovery D used from separate evidence dir with same hashes; residual risk left for clean audit | `REAL16_DECISION_PACKET.md` |
| REAL16-R4 | Overclaim from E4 candidate to achieved/emergence | controlled | reports say candidate-only; forbidden wording scan required | `REAL16_E4_CANDIDATE_REPORT.md` |
| REAL16-R5 | Market-tx-count-only overclaim | controlled | verifier requires behavior metrics; test rejects market_tx_count-only improvement | `tests/constitution_real16_market_performance.rs` |
| REAL16-R6 | Trust Root drift | controlled | envelope-local rehash applied; Trust Root unit passed | `genesis_payload.toml` |
| REAL16-R7 | Solve-rate/PPUT non-improvement hidden by candidate wording | disclosed | report explicitly states all arms solved 0 and PPUT 0; candidate rests on wasted attempts, failed branches, and conversion only | `REAL16_METRICS.json` |

## Forbidden Mechanisms

No evidence in REAL-16 requires:

- forced trade,
- price-as-truth,
- ghost liquidity,
- off-tape truth,
- raw CoT/prompt/completion/log broadcast,
- f64/f32 market money path,
- scripted or PolicyTrader action counted as E2/E4.

If clean-context audit finds any of the above, REAL-16 must be downgraded.
