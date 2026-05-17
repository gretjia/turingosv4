# REAL-14 Decision Packet

status: `E2 candidate pending audit` for R16 only

forbidden claims: no `E2 achieved`, no `market emergence proven`, no `market mechanism shipped`, no `E3/E4 achieved`

## Verdict

R16 can remain labeled:

```text
E2 candidate pending audit
```

R16 cannot be upgraded beyond that label in REAL-14. R17 did not replicate the
candidate and R18 did not show BCAST-off action. The correct project state is:

```text
R16: single-run YES-side E2 candidate pending audit
R17: clean-negative replication, BCAST on
R18: clean-negative BCAST-off ablation
Replication status: not replicated
Two-sided status: not established
```

## Evidence Table

| Run | BCAST | Evidence dir | audit_tape | Verifier JSON | Exact join | Agent action | Claim boundary |
| --- | --- | --- | --- | --- | ---: | ---: | --- |
| R16 | on | `handover/evidence/market_autonomy_lab_hard10_bcast_exact_join_R16_20260516T202248Z` | PROCEED | `handover/directives/market_autonomy_lab/REAL14_R16_VERIFIER_REPORT.json` | 8 | 8 | single-run `E2 candidate pending audit` |
| R17 | on | `handover/evidence/market_autonomy_lab_hard10_real14_R17_20260516T234921Z` | PROCEED | `handover/directives/market_autonomy_lab/REAL14_R17_VERIFIER_REPORT.json` | 0 | 0 | clean-negative |
| R18 | off | `handover/evidence/market_autonomy_lab_hard10_real14_R18_bcast_off_20260517T000136Z` | PROCEED | `handover/directives/market_autonomy_lab/REAL14_R18_VERIFIER_REPORT.json` | 0 | 0 | clean-negative ablation |

All three runs use the hard10 problem set:

```text
handover/preregistration/sample_E1v2_hard10_S20260423.txt
sha256=138f75bc3a776df11df45fed77c01565e128662ca03e3d863e68dcf8ee22d8fc
```

## Atom Gate Status

| Atom | Gate status | Evidence |
| --- | --- | --- |
| Atom 0 — Freeze R16 evidence | PASS | `REAL14_R16_EVIDENCE_HASH_MANIFEST.md/json` |
| Atom 1 — exact-join verifier | PASS | `REAL14_R16_VERIFIER_REPORT.json`, `exact_join_count=8` |
| Atom 2 — matched provenance | PASS with residual risk | 8/8 matched tx have MarketDecisionTrace, EVDecisionTrace, MarketOpportunityTrace, PromptCapsule linkage, live BullTrader role; linkage is indirect via EVDecisionTrace |
| Atom 3 — economic invariant audit | PARTIAL | verifier confirms L4 typed router payload, real buyer, integer pay amount, no scripted fixture, PolicyTrader excluded; full balance/reserve replay remains auditor follow-up |
| Atom 4 — BCAST shielding | PASS | verifier scans digests, role crops, visible contexts; R16 PASS over 278/278/278 |
| Atom 5 — clean-context audit | PROCEED | `REAL14_CLEAN_CONTEXT_AUDIT.md`; auditor verdict permits only `E2 candidate pending audit`, not E2 achieved |
| Atom 6 — R17 replication | CLEAN-NEGATIVE | audit PROCEED, exact_join_count=0 |
| Atom 7 — R18 BCAST-off ablation | CLEAN-NEGATIVE | audit PROCEED, exact_join_count=0 |
| Atom 8 — short-side probe | NOT ESTABLISHED | no live buy_no/short-equivalent in R16/R17/R18 |

## R16 Exact-Join Evidence

The verifier computes:

```text
L4_buy_set = all L4 BuyWithCoinRouterTx tx_id
Trace_submitted_set = all CAS MarketDecisionTrace Submitted tx_id
Exact_join = L4_buy_set ∩ Trace_submitted_set
```

R16 result:

```text
l4_router_tx_count=8
submitted_trace_tx_count=8
exact_join_count=8
duplicate_l4_router_tx_id_count=0
duplicate_submitted_trace_tx_id_count=0
scripted_fixture_tx_count=0
policy_counts_for_e2=false
verdict=PROCEED
```

R16 matched actions are all YES-side BullTrader actions. This supports only a
YES-side candidate, not a two-sided market claim.

## Replication Interpretation

R17 and R18 both produced 40 EVDecisionTrace records, all abstain:

```text
R17 policy_positive_ev_count=0
R17 policy_insufficient_public_basis_count=40
R18 policy_positive_ev_count=0
R18 policy_insufficient_public_basis_count=40
```

So the current bottleneck is:

```text
public EV basis did not form reliably after R16
```

not:

```text
PositiveEVIgnored
```

## Residual Risks

| Risk | Status | Boundary |
| --- | --- | --- |
| R16 may be stochastic/single-run artifact | open | R17 did not replicate |
| R16 PromptCapsule linkage is indirect | open | Marked in verifier residual_risks; no upgrade beyond candidate |
| Full economic balance/reserve replay not yet executable as REAL-14 gate | open | Do not call E2 confirmed |
| Joined-field mismatch fixture not yet added | open | Auditor marked as non-blocking hardening; current R16 rows align buyer/event/direction/amount |
| Short side absent | open | Do not claim two-sided market |
| Dirty research worktree during R17/R18 | disclosed | Research mode only; not ship evidence |

## Final Verification

```text
cargo fmt --all -- --check: PASS
cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo -- --test-threads=1: PASS
TURINGOS_RESEARCH_ENVELOPE=MARKET_AUTONOMY_LAB_RESEARCH_ENVELOPE_V2 bash scripts/run_market_autonomy_research_preflight.sh: PASS, Level0 Continue
cargo test --test constitution_real14_e2_candidate_verifier -- --test-threads=1: PASS, 6/6
bash scripts/run_constitution_gates.sh: PASS, 461 passed / 0 failed / 1 ignored
git diff --check: PASS
jq empty REAL14 verifier/manifest/metrics JSON: PASS
real14_e2_candidate_verifier R16 --expect-count 8: PASS
clean-context Codex audit: PROCEED
```

## Next Recommendation

Proceed to a narrow in-envelope follow-up:

```text
REAL-14F — Public EV Basis Stabilization
```

Goal: make public EV basis fields reliably present for Bull/Bear market-review
turns without forcing trades. Re-run hard10 after the basis gate. If
PolicyTrader positive-EV opportunities reappear but LLM abstains, return to
PositiveEVIgnored/action-conversion work. If no positive EV appears, tune
collateral-backed market parameters without ghost liquidity or price-as-truth.
