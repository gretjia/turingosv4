# REAL-16 E4 Candidate Report

claim_boundary: `E4 candidate pending audit`

This is candidate-only research evidence. It does not authorize `E4 achieved`,
`market emergence proven`, or `market mechanism shipped`.

## Evidence

- Combined packet:
  `handover/evidence/market_autonomy_lab_real16_abcd_oppfix_recovery_combined_20260517T150000Z`
- Full A/B/C source run:
  `handover/evidence/market_autonomy_lab_real16_hard10_ab_sf_oppfix_20260517T105111Z`
- Recovery D source run:
  `handover/evidence/market_autonomy_lab_real16D_hard10_sf_oppfix_recovery_20260517T135314Z`

The full-run D arm is excluded because ENOSPC left zero-byte verifier and
aggregate files. Recovery D is used only because it has the same pinned
problem/model/budget/runtime/prompt hashes as A/B/C and independent
`audit_tape`/E2 verifier `PROCEED`.

## Result

The REAL-16 verifier reports:

```text
verdict: Proceed
claim_boundary: E4 candidate pending audit
best_arm_id: D
improved_metrics:
  - wasted_attempts
  - failed_branch_count
  - ev_to_action_conversion
```

Arm summary:

| arm | source | audit | E2 verifier | exact_join | solved | PPUT | wasted | failed branches |
| --- | --- | --- | --- | ---: | ---: | ---: | ---: | ---: |
| A | full A/B/C | PROCEED | VETO control no-candidate | 0 | 0 | 0 | 173 | 182 |
| B | full A/B/C | PROCEED | PROCEED | 9 | 0 | 0 | 180 | 188 |
| C | full A/B/C | PROCEED | PROCEED | 3 | 0 | 0 | 166 | 178 |
| D | recovery | PROCEED | PROCEED | 16 | 0 | 0 | 149 | 158 |

## Interpretation

The candidate is narrow. It supports a performance-pressure signal only on
wasted attempts, failed branch count, and EV-to-action conversion. It does not
show solve-rate or verified PPUT improvement: all arms have `solved_count=0`
and `verified_pput_micro=0`.

The A arm has an invalid router match, but A is the no-market-pressure control.
The verifier normalizes that invalid control-arm match to `exact_join_count=0`
and does not count it as agent economic action. Market-pressure arms still
require E2 verifier `PROCEED`; B, C, and recovery D satisfy that gate.

## Claim Boundary

Allowed wording:

```text
REAL-16 is E4 candidate pending audit.
```

Forbidden wording:

```text
E4 achieved
market emergence proven
market mechanism shipped
```

## Residual Risks

- D is a recovery arm, not the uninterrupted full-run D; this should be
  explicitly audited.
- The E4 signal is not solve-rate/PPUT based.
- A/B/C/D share pinned hashes, but the recovery arm should remain visibly
  separated from the contaminated full-run D.
- The candidate remains subject to clean-context audit.
