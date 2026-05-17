# REAL-16 Status Sync / OBS

status: `candidate packet awaiting clean-context audit`

## Summary

REAL-16 now has a combined A/B/C + recovery-D verifier packet with
`claim_boundary=E4 candidate pending audit`.

This is not an achieved claim. It is a candidate packet for clean-context audit.

## Important Boundaries

- Full-run D from `market_autonomy_lab_real16_hard10_ab_sf_oppfix_20260517T105111Z`
  is contaminated by ENOSPC and excluded.
- Recovery D from
  `market_autonomy_lab_real16D_hard10_sf_oppfix_recovery_20260517T135314Z`
  is used with matching hashes.
- A is a control arm; its invalid router match is normalized to zero
  claim-bearing exact joins.
- B/C/D have E2 verifier `PROCEED`.

## Model Access Note

SiliconFlow-backed non-thinking/instruct model diversity is authorized for
future true-problem runs. Current hashes already pin the model assignment used
by REAL-16; future cycles should preserve model-assignment hashes and avoid
thinking-mode unless explicitly needed.

## Next

Await clean-context audit verdict on:

```text
Can the combined REAL-16 ABC + recovery-D packet be labeled exactly
E4 candidate pending audit?
```

If `PROCEED`, assemble the market-emergence final-audit packet over E2
replication, two-sided action, E3 role differentiation, and E4 candidate
performance signal.
