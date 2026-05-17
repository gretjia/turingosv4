# REAL-15 Constitutional Risk Register

scope: REAL-15 Persistent Role Differentiation / E3 Candidate Study
risk_class: Class 3 research evidence under `MARKET_AUTONOMY_LAB_RESEARCH_ENVELOPE_V2`

| risk_id | risk | class | evidence | mitigation | status |
| --- | --- | --- | --- | --- | --- |
| REAL15-RISK-001 | Premature E3 overclaim | 3 | REAL-15 supports candidate-only label | Use `E3 candidate pending audit`; forbid `E3 achieved` | controlled |
| REAL15-RISK-002 | Dashboard as source of truth | 3 | E3 verifier must derive from CAS role traces plus exact-join verifier JSON | Verifier and tests reject dashboard-only source | controlled |
| REAL15-RISK-003 | Solver activity overstatement | 3 | Solver `work_count=34` is CAS `RoleTurnOutcome::SubmitProof`, not accepted WorkTx production | Report as role activity only; do not claim accepted solver production | accepted residual |
| REAL15-RISK-004 | BearTrader/Verifier not persistent across both runs | 3 | BearTrader and Verifier each active in one run | E3 threshold uses BullTrader + Solver; Bear/Verifier treated as supporting context | accepted residual |
| REAL15-RISK-005 | Upstream exact-join provenance residuals | 3 | REAL-14G/H verifier residual warnings are surfaced in REAL-15 JSON | Keep candidate-only label; future provenance tightening may reduce warnings | accepted residual |
| REAL15-RISK-006 | PolicyTrader/scripted action counted as role market action | 3 | Input exact-join reports have `scripted_fixture_tx_count=0` and `policy_counts_for_e2=false` | REAL-15 VETOes scripted/PolicyTrader input reports | controlled |
| REAL15-RISK-007 | Trust Root drift from additive module export | 4-adjacent | `src/runtime/mod.rs` changed to export `role_differentiation` | Allowed envelope rehash in `genesis_payload.toml`; Trust Root rerun passed | controlled |
| REAL15-RISK-008 | Broad gate runner resource failure | 2 | Two G3 gates failed during global script with linker Bus error | Both affected tests passed sequentially with `CARGO_BUILD_JOBS=1`; record as resource checkpoint | controlled |
| REAL15-RISK-009 | E4 not established | 3 | No pinned A/B performance benchmark in REAL-15 | Open REAL-16 E4 candidate benchmark | open |

## Hard Stop Review

No Level 3 Constitutional Hard Stop was reached in REAL-15.

No evidence indicates:

```text
forced trade counted as E2/E3
price-as-truth
ghost liquidity
off-tape truth
raw CoT/raw prompt/raw completion/raw log broadcast
f64/f32 market money path
PolicyTrader/scripted action counted as E2/E3
```
