# REAL-17 P4c Clean-Context Audit

Date: 2026-05-17

Reviewer:

```text
clean-context Codex GPT-5.5 high
```

Audit question:

```text
Can P4c be used as REAL-17 direct PromptCapsule provenance hardening evidence
for the candidate-only market-autonomy line?
```

Required boundary:

```text
candidate-only hardening evidence; no achieved/proven/shipped market label
```

## Findings

No blocker found.

1. P4c can be cited as candidate-only direct PromptCapsule provenance
   hardening evidence. `REAL17P4C_E2_VERIFIER_STRICT_DIRECT.json` reports:

```text
verdict                                      PROCEED
l4_router_tx_count                           13
submitted_trace_tx_count                     13
exact_join_count                             13
scripted_fixture_tx_count                    0
policy_counts_for_e2                         false
direct_prompt_capsule_provenance_count       13
missing_direct_prompt_capsule_provenance     0
```

2. The tape/audit layer supports the evidence package. `aggregate_verdict.json`
   reports:

```text
verdict  PROCEED
passed   41
failed   0
halted   0
skipped  11
```

The audit covers ChainTape/CAS assertions including CID resolution,
replay/economic state, conservation, price-view-only, and shielding.

3. Shielding and exclusion of non-live-action counters are adequately supported
   for this candidate claim. The strict verifier shows:

```text
prompt_capsule_linkage        direct_via_market_decision_provenance_link
actor_is_policy_trader        false
actor_is_live_agent_role      true
bcast_shielding.verdict       PASS
```

4. P4c does not support an A/B performance claim. The parent
   `REAL16_MARKET_PERFORMANCE_REPORT.json` reports:

```text
claim_boundary  clean-negative; no E4 candidate
arm_count       1
e4_candidate    false
verdict         Veto
failure         fewer_than_two_ab_arms
```

5. Persistence is sufficient for a hardening-evidence citation, but not for a
   broad emergence/performance citation. `PERSISTENCE_BINDING_REPORT.json`
   reports:

```text
is_passing  true
n_witnessed 3
```

Balances, PnL, and model identity are witnessed; positions, reputation, and
autopsy are explicitly empty.

## Evidence Defects

None found for the requested narrow claim:

```text
hard10 D-arm
exact-join live agent economic action candidate
direct PromptCapsule provenance 13/13
audit_tape PROCEED
```

## Reporting Gaps

The verifier file names and report title still contain E2 candidate wording.
Downstream summaries must preserve candidate language.

Some matched rows carry residual risk text:

```text
multiple EVDecisionTrace rows match agent/event/action;
tx_id is disambiguated only by exact router join
```

This is acceptable for exact-join hardening evidence, but must not be inflated
into a stronger behavioral or causal claim.

## Residual Risks

Do not cite P4c as:

```text
an E-level achieved label
a market-emergence proof
a shipped mechanism
an A/B performance result
```

Use it only for candidate direct-provenance hardening.

## Verdict

```text
PROCEED
```
