# REAL-14G Status Sync / OBS

REAL-17 main-CAS integration note: this document is preserved as pre-CAS-repair
historical context only. It is not forward claim-bearing on
`codex/real17-emergence-hardening-20260517`; REAL-17 must regenerate
ChainTape/CAS/verifier evidence on the updated CAS Git commit-chain baseline
before any forward claim can rely on it.

date_utc: 2026-05-17
run_id: `market_autonomy_lab_hard10_real14G_action_conversion_20260517T022457Z`
status: `E2 candidate pending audit`

## Current State

REAL-14G completed the PositiveEVIgnored/action-conversion cycle under the
research envelope.

Key results:

```text
hard10 batch exit: 0
audit_tape: PROCEED
exact_join_count: 8
public EV basis delivery: 38/38
PolicyTrader positive EV: 25
PositiveEVIgnored: 17
action_conversion_rate_bps: 3200
buy_yes: 8
buy_no: 0
BCAST shielding: PASS
clean-context audit: PROCEED
```

## Interpretation

The current dominant bottleneck is no longer missing public EV basis. It is:

```text
voluntary action conversion plus YES-side-only behavior
```

The next research move should not force shorting or buying. It should improve
BearTrader NO-side visibility and affordance, then replicate the frozen
REAL-14G configuration.

## Claim Boundary

Allowed:

```text
E2 candidate pending audit
```

Not allowed:

```text
E2 achieved
E2 replicated candidate
Two-sided market candidate
E3 candidate
E4 candidate
market emergence candidate
ship evidence
```

## Next

Open:

```text
REAL-14H -- Side-Balance / BuyNo Probe + Frozen REAL-14G Replication
```
