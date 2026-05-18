# REAL-17 P6D — TaskOutcomeMarket Seed Poll-Budget Stabilization

Date: 2026-05-18
Mode: Constitutional Research Mode under `MARKET_AUTONOMY_LAB_RESEARCH_ENVELOPE_V2`
Risk: Class 3-adjacent, trust-root-pinned evaluator rehash inside envelope

## Claim Boundary

P6D is a runner-stability atom for REAL-17 side-balance evidence. It is not an
E2/E3/E4 claim and does not change the market mechanism.

Allowed forward label remains:

```text
market emergence candidate -- final audit PROCEED, hardening pending
```

Forbidden active claims remain forbidden:

```text
E2 achieved
E3 achieved
E4 achieved
market emergence proven
market mechanism shipped
```

## Evidence Trigger

P6c evidence:

```text
handover/evidence/market_autonomy_lab_real17P6c_tisr_main_hard10_bear_salience_clean_20260518T033801Z
```

P6c was launched from a clean worktree at commit `b4020fd3` with
`TURINGOS_REAL6A_POLL_BUDGET_MS=120000`. The run produced partial diagnostic
evidence only:

```text
batch_evaluator exit: non-zero
terminated_reason: subprocess for task 8 mathd_algebra_332 exited non-zero (3)
audit_tape verdict: PROCEED
exact_join_count: 11
direct PromptCapsule provenance: 11/11
BCAST shielding: PASS
BuyNo exact-join: 0
```

Failure root:

```text
[chaintape/real6a] TaskOutcomeMarket seed FAIL-CLOSED:
await REAL-6A MarketSeedTx commit: ()
```

The evaluator still used hard-coded `5000` millisecond waits in the
TaskOutcomeMarket seed path, so the configured 120s poll budget did not cover
the pre-seed EscrowLock wait or the MarketSeed/CpmmPool helper wait.

## FC / Constitution Mapping

Touched FC nodes:

- FC1 runtime loop: task-open market seeding before agent work.
- FC1 `wtool` / ChainTape commit observation: wait for canonical typed tx
  admission instead of treating stdout/dashboard as truth.

Untouched restricted surfaces:

- no sequencer admission change
- no typed tx schema/discriminant change
- no canonical signing payload change
- no wallet/kernel/bus authority change
- no CAS ObjectType schema change
- no market math or conservation change

## Patch

The patch routes the existing TaskOutcomeMarket seed waits through the existing
configured budget helper:

```text
real6a_poll_budget_ms()
```

Affected waits:

```text
EscrowLock commit before REAL-6A TaskOutcomeMarket seed
REAL-6A MarketSeedTx / CpmmPoolTx helper commit waits
```

This preserves fail-closed behavior. A missing commit still exits non-zero; it
just uses the configured evidence-run latency budget rather than a hard-coded
5s timeout.

## Gates

Red gate before implementation:

```bash
cargo test --test constitution_real6_task_outcome_market \
  sg_6a_task_outcome_seed_uses_configured_poll_budget -- --test-threads=1
```

Observed expected failure:

```text
TaskOutcomeMarket MarketSeed/CpmmPool helper must use
TURINGOS_REAL6A_POLL_BUDGET_MS
```

Green gates after implementation:

```bash
cargo test --test constitution_real6_task_outcome_market \
  sg_6a_task_outcome_seed_uses_configured_poll_budget -- --test-threads=1

cargo test --test constitution_real6_task_outcome_market -- --test-threads=1

rustfmt --edition 2021 --check \
  experiments/minif2f_v4/src/bin/evaluator.rs \
  tests/constitution_real6_task_outcome_market.rs

git diff --check

cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo \
  -- --test-threads=1
```

Observed result:

```text
targeted REAL-6A market tests: 23 passed / 0 failed
Trust Root unit gate: passed
```

## P6c Mechanism Result

P6c remains a partial diagnostic, not full hard10 evidence. It supports:

```text
direct-provenance exact-join still works on the partial run
BearTrader NO-side action conversion remains unresolved
TaskOutcomeMarket seed commit budget was a runner stability bottleneck
```

P6c does not support:

```text
stable two-sided market
full hard10 replication
E2/E3/E4 achieved
market emergence proven
```

## Next Step

After clean-context audit of P6D, rerun side-balance evidence from a clean
commit with the same constitution-preserving settings:

```text
REAL-17 P7 — Side-Balance hard10 after seed poll-budget stabilization
```

If P7 remains YES-only, the next mechanism hypothesis is BearTrader semantic
clarity: NO-side action may need to be framed as task-outcome NO / no valid
proof before deadline, not as mathematical theorem falsehood. Any such patch
must remain non-forcing and must preserve valid abstain.
