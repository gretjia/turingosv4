# REAL-14F Decision Packet

status: `E2 candidate pending audit`

forbidden claims: no `E2 achieved`, no `market emergence proven`, no
`market mechanism shipped`, no `E3 achieved`, no `E4 achieved`.

## Verdict

REAL-14F can be labeled:

```text
E2 candidate pending audit
```

The label is narrow. The run produced three live, non-scripted,
agent-generated BuyWithCoinRouterTx candidates by exact tx-id join, but all
three are YES-side BullTrader actions. This does not prove two-sided market
emergence, E3, E4, or ship readiness.

## Evidence Table

| Item | Value |
| --- | --- |
| run | `market_autonomy_lab_hard10_real14F_ev_basis_20260517T012509Z` |
| evidence_dir | `handover/evidence/market_autonomy_lab_hard10_real14F_ev_basis_20260517T012509Z` |
| problem_set_hash | `138f75bc3a776df11df45fed77c01565e128662ca03e3d863e68dcf8ee22d8fc` |
| config_hash | `ce7496650684a74fb9d81b10cbe19e3f2ebbd7595da97df31f97755cc0f0a141` |
| audit_tape | `PROCEED`, failed `0`, halted `0`, assertions `52` |
| exact_join verifier | `PROCEED` |
| exact_join_count | `3` |
| BCAST shielding | `PASS` |

## Atom Status

| Atom | Status | Evidence |
| --- | --- | --- |
| Atom 0 Context Freeze | PASS | REAL-14 artifacts present; R16 label remains candidate-only |
| Atom 1 Public EV Basis Red Gates | PASS | `constitution_real13a_ev_decision_trace` red/green gates |
| Atom 2 EV Basis Stabilization | PASS | public basis delivery `38/38`, missing `0` |
| Atom 3 PolicyTrader Baseline Recovery | PASS | `policy_positive_ev_count=23`, `policy_counts_for_e2=false` |
| Atom 4 Action Conversion | PARTIAL | exact-join live buys `3`; ignored positive EV remains `20` |
| Atom 5 Market Parameter Tuning | NOT NEEDED THIS RUN | positive EV reappeared without tuning |
| Atom 6 Hard10 True-Problem Run | PASS | batch_exit `0`, audit_tape `PROCEED` |
| Atom 7 Exact-Join / Provenance Audit | PASS with residual risk | verifier `PROCEED`; PromptCapsule link indirect via EVDecisionTrace |
| Atom 8 Clean-Context Audit | PROCEED | `REAL14F_CLEAN_CONTEXT_AUDIT.md`; candidate label only |

## Mechanism Diagnosis

R17/R18 bottleneck was missing public EV basis. REAL-14F corrected that for the
new hard10 run:

```text
ev_public_basis_available_count=38
ev_public_basis_missing_count=0
ev_public_basis_delivery_rate_bps=10000
policy_insufficient_public_basis_count=0
```

Positive EV reappeared:

```text
policy_positive_ev_count=23
policy_positive_ev_llm_abstained_count=20
exact_join_count=3
```

The next bottleneck is no longer public basis; it is voluntary action
conversion and side balance.

## Residual Risks

| Risk | Status | Boundary |
| --- | --- | --- |
| YES-side only | open | do not claim two-sided market |
| PromptCapsule direct field absent in MarketDecisionTrace | open | verifier and clean-context audit mark indirect linkage via EVDecisionTrace |
| PositiveEVIgnored remains high | open | 20 ignored positive-EV opportunities |
| Replication beyond this hard10 | open | run is a new candidate, not E2 achieved |
| Full E3/E4 evidence | absent | no role-differentiation/performance claim |

## Next Recommendation

Proceed in-envelope to REAL-14G:

```text
PositiveEVIgnored / action-conversion stabilization
```

Use the same hard10 configuration as a baseline, preserve voluntary trade, and
test only constitution-preserving mechanisms: clearer public EV summary,
ChainTape/CAS PnL scoreboard, Librarian digest of missed positive-EV patterns,
and role-specific Bull/Bear view improvements. Do not force buy/short and do
not count PolicyTrader as E2.
