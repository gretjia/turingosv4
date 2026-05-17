# REAL-14G E2 Candidate Report

claim_boundary: `E2 candidate pending audit`
evidence_dir: `handover/evidence/market_autonomy_lab_hard10_real14G_action_conversion_20260517T022457Z`
audit_tape verdict: `PROCEED`
config_hash: `35b03c8ed3d49979390e11235657baa057b7b93f7608d99994037bc8d5043d42`

REAL-14G is a constitutional research result, not a ship result. It does not
claim `E2 achieved`, `E3/E4 achieved`, `market emergence proven`, or
`market mechanism shipped`.

## Candidate Evidence

The independent verifier recomputed the candidate count from ChainTape/CAS
inputs, not from dashboard text:

```text
L4 BuyWithCoinRouter tx ids
INTERSECT
submitted MarketDecisionTrace router tx ids
```

Verifier output:

```text
verdict: PROCEED
l4_router_tx_count: 8
submitted_trace_tx_count: 8
exact_join_count: 8
duplicate_l4_router_tx_id_count: 0
duplicate_submitted_trace_tx_id_count: 0
scripted_fixture_tx_count: 0
policy_counts_for_e2: false
bcast_shielding_verdict: PASS
```

Matched tx ids:

```text
router-task-outcome-Agent_0-task-n5_amc12_2000_p12_1778984985337-Agent_0-0
router-task-outcome-Agent_0-task-n5_amc12_2000_p12_1778984985337-Agent_0-5
router-task-outcome-Agent_0-task-n5_amc12_2000_p6_1778985024025-Agent_0-0
router-task-outcome-Agent_0-task-n5_mathd_algebra_208_1778985137183-Agent_0-0
router-task-outcome-Agent_0-task-n5_mathd_algebra_246_1778985171418-Agent_0-0
router-task-outcome-Agent_0-task-n5_mathd_algebra_270_1778985187029-Agent_0-0
router-task-outcome-Agent_0-task-n5_mathd_algebra_332_1778985237848-Agent_0-5
router-task-outcome-Agent_0-task-n5_numbertheory_2pownm1prime_nprime_1778985313104-Agent_0-0
```

## Action Conversion

REAL-14F baseline showed public EV basis delivery was no longer the dominant
bottleneck. REAL-14G improved the action-conversion result while preserving
voluntary action:

```text
public EV basis delivery: 38/38
PolicyTrader positive EV count: 25
executed exact-join positive action count: 8
PositiveEVIgnored: 17
action_conversion_rate_bps: 3200
```

The remaining ignored opportunities are all classified by the new CAS-derived
summary as:

```text
ModelAbstentionDespiteClearBasis: 17
Unknown: 0
```

This means the next mechanism bottleneck is no longer missing public EV basis.
It is still voluntary action conversion, plus side balance.

## Side Boundary

REAL-14G is YES-side only:

```text
buy_yes_count: 8
buy_no_count: 0
```

The report therefore does not claim a two-sided market.

## Provenance And Shielding

The matched candidates have live-agent role provenance:

```text
actor: Agent_0
role: BullTrader
PromptCapsule linkage: indirect_via_ev_decision_trace
MarketOpportunityTrace: present
EVDecisionTrace: present
RoleTurnTrace: present
```

BCAST shielding was independently reproduced:

```text
librarian_digest_count: 91
librarian_role_crop_count: 91
visible_context_count: 91
verdict: PASS
failure_count: 0
```

No raw prompt, completion, private CoT, or raw log broadcast is used as
provenance evidence.

## Residual Risks

1. Prompt provenance is indirect through EVDecisionTrace rather than a direct
   MarketDecisionTrace field.
2. The first two matched tx ids each have two matching EVDecisionTrace rows;
   exact router tx_id join disambiguates candidate counting.
3. No voluntary buy_no or short-equivalent appeared.
4. PositiveEVIgnored remains high at 17.
5. This is still research evidence under the envelope, not ship evidence.

## Decision

REAL-14G may be labeled:

```text
E2 candidate pending audit
```

It must not be labeled:

```text
E2 achieved
market emergence proven
market mechanism shipped
E3 achieved
E4 achieved
```
