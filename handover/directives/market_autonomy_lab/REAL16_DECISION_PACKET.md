# REAL-16 Decision Packet

REAL-17 main-CAS integration note: this packet is preserved as historical
market-autonomy context from the older pre-CAS-repair worktree. The evidence
directories cited here were intentionally not migrated into the new
main-based REAL-17 worktree and are not forward claim-bearing on the updated
CAS Git commit-chain baseline. REAL-17 must regenerate any claim-bearing E4
evidence on the repaired CAS baseline.

decision: `E4 candidate pending audit`

This packet remains inside Constitutional Research Mode under
`MARKET_AUTONOMY_LAB_RESEARCH_ENVELOPE_V2`. It is not Ship Mode.

## Baseline

- REAL-14G and REAL-14H already established replicated E2 candidate evidence.
- REAL-15 established `E3 candidate pending audit`.
- REAL-16 tests whether market pressure has a candidate performance signal.

## Evidence Used

Use:

- A/B/C from
  `handover/evidence/market_autonomy_lab_real16_hard10_ab_sf_oppfix_20260517T105111Z`
- D from
  `handover/evidence/market_autonomy_lab_real16D_hard10_sf_oppfix_recovery_20260517T135314Z`
- Combined verifier packet:
  `handover/evidence/market_autonomy_lab_real16_abcd_oppfix_recovery_combined_20260517T150000Z`

Do not use the full-run D arm from
`market_autonomy_lab_real16_hard10_ab_sf_oppfix_20260517T105111Z/arm_D`; it is
contaminated by ENOSPC and contains zero-byte verifier/aggregate files.

## Verification

Executed checks:

```text
cargo fmt --all -- --check
cargo test --test constitution_real16_market_performance -- --test-threads=1
cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo -- --test-threads=1
TURINGOS_RESEARCH_ENVELOPE=MARKET_AUTONOMY_LAB_RESEARCH_ENVELOPE_V2 bash scripts/run_market_autonomy_research_preflight.sh
```

Observed status:

```text
fmt: PASS
REAL-16 tests: PASS, 13/13
Trust Root unit: PASS
research preflight: Level0 Continue
```

## Verifier Changes

The REAL-16 verifier was corrected so that an invalid router match in the
control arm is not counted as claim-bearing agent action and does not cause a
global E4 VETO. Market-pressure arms still require E2 verifier `PROCEED`.

The load-bearing files remain Trust Root pinned after the envelope-local rehash.

## Result Table

| arm | source run | audit_tape | E2 verifier | exact_join | solved | verified_pput | wasted | failed branches |
| --- | --- | --- | --- | ---: | ---: | ---: | ---: | ---: |
| A | full A/B/C | PROCEED | VETO control no-candidate | 0 | 0 | 0 | 173 | 182 |
| B | full A/B/C | PROCEED | PROCEED | 9 | 0 | 0 | 180 | 188 |
| C | full A/B/C | PROCEED | PROCEED | 3 | 0 | 0 | 166 | 178 |
| D | recovery | PROCEED | PROCEED | 16 | 0 | 0 | 149 | 158 |

Combined verifier result:

```text
verdict: Proceed
e4_candidate: true
best_arm_id: D
improved_metrics:
  - wasted_attempts
  - failed_branch_count
  - ev_to_action_conversion
failure_reasons: []
```

## Decision

REAL-16 may be presented to clean-context audit as:

```text
E4 candidate pending audit
```

It may not be presented as:

```text
E4 achieved
market emergence proven
market mechanism shipped
```

## Next Step

If clean-context audit returns `PROCEED`, open the market emergence final-audit
packet over:

- REAL-14G/14H E2 replicated candidate evidence,
- REAL-14H two-sided candidate evidence,
- REAL-15 E3 candidate evidence,
- REAL-16 E4 candidate evidence.

If audit returns `CHALLENGE`, fix within the research envelope and rerun the
relevant verifier. If it returns `VETO`, downgrade REAL-16 to clean-negative and
continue with a new performance hypothesis.
