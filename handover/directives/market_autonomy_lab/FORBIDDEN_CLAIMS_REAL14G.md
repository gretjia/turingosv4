# REAL-14G Forbidden Claims Scan

scope: non-historical REAL-14G deliverables plus REAL-14G evidence packet
decision_authority: none

## Commands

```bash
rg -n "E2 achieved|market emergence proven|market mechanism shipped|E3 achieved|E4 achieved" \
  handover/directives/market_autonomy_lab \
  handover/evidence/market_autonomy_lab_hard10_real14G_action_conversion_20260517T022457Z
```

```bash
rg -n "forced trade|price-as-truth|ghost liquidity|PolicyTrader.*E2|scripted.*E2|raw CoT|raw prompt|raw completion|raw log" \
  handover/directives/market_autonomy_lab \
  handover/evidence/market_autonomy_lab_hard10_real14G_action_conversion_20260517T022457Z
```

## Result

The scan finds many historical, quoted, negated, or forbidden-list references
in prior REAL-14/REAL-14F files. Those files are not rewritten because they are
historical or already annotate the wording as forbidden.

No active REAL-14G deliverable claims any of the following:

```text
E2 achieved
market emergence proven
market mechanism shipped
E3 achieved
E4 achieved
```

## Active REAL-14G Wording Table

| phrase | status | replacement |
| --- | --- | --- |
| `E2 achieved` | forbidden; only appears in REAL-14G as negated wording | `E2 candidate pending audit` |
| `market emergence proven` | forbidden; only appears in REAL-14G as negated wording | `market emergence candidate pending final audit` only after full ladder |
| `market mechanism shipped` | forbidden; only appears in REAL-14G as negated wording | `research evidence only` |
| `E3 achieved` | forbidden; only appears in REAL-14G as negated wording | `E3 candidate pending audit` only after role-differentiation evidence |
| `E4 achieved` | forbidden; only appears in REAL-14G as negated wording | `E4 candidate pending audit` only after pinned A/B evidence |

## Notes

Historical files with forbidden strings are annotated, not edited. REAL-14G
uses the narrow candidate label and explicitly records side and replication
limits.
