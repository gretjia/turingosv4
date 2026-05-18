# REAL-17 P6D Clean-Context Audit

Date: 2026-05-18
Reviewer: clean-context GPT-5.5 high
Verdict: PROCEED

## Audit Question

Can the narrow P6D patch proceed as constitutional research evidence support?

## Scope

Risk class:

```text
Class 3-adjacent / trust-root-pinned evaluator rehash inside
MARKET_AUTONOMY_LAB_RESEARCH_ENVELOPE_V2
```

Touched files:

```text
experiments/minif2f_v4/src/bin/evaluator.rs
tests/constitution_real6_task_outcome_market.rs
genesis_payload.toml
```

Touched FC / invariants:

```text
FC1 runtime loop commit/wtool evidence timing for REAL-6A TaskOutcomeMarket seed
ChainTape/CAS remains source of truth
```

Untouched restricted surfaces:

```text
sequencer admission
TypedTx schema/discriminant
canonical signing payload
wallet
kernel
bus
CAS ObjectType schema
```

## Findings

No production defects were found in the narrow P6D diff.

The production changes replace hard-coded `5000` commit-await budgets with the
existing `real6a_poll_budget_ms()` helper at the REAL-6A TaskOutcomeMarket
seed path. The helper keeps its existing environment/default behavior.

The adapter helper already consumes the passed budget for both MarketSeedTx and
CpmmPoolTx awaits, so the patch addresses the P6c failure mode without
altering market semantics.

The reviewer noted one non-blocking reporting gap before this file was written:
`genesis_payload.toml` had the correct evaluator hash but stale inline
provenance text from the prior REAL-17 Direct PromptCapsule rehash. The forward
P6D provenance text is now present on the evaluator pin line.

## Forbidden Mechanism Check

The diff introduces none of the following:

```text
forced trade
price-as-truth
ghost liquidity
f64/f32 money/probability market path
off-tape truth
raw prompt/completion/CoT/log broadcast
scripted or PolicyTrader action counted as E2
```

No E2/E3/E4 achieved claim or market-emergence-proven claim is introduced.

## Genesis Rehash

The evaluator SHA in `genesis_payload.toml` matches the current
`experiments/minif2f_v4/src/bin/evaluator.rs`, and the Trust Root unit gate
passes. The rehash is acceptable under the research envelope for this touched
pinned file.

## Verification

Commands run before the audit:

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

Observed:

```text
single red/green gate: passed after patch
constitution_real6_task_outcome_market: 23 passed / 0 failed
rustfmt check: passed
git diff --check: passed
Trust Root unit gate: passed
```

After updating the forward P6D provenance note in `genesis_payload.toml`, the
Trust Root unit gate was rerun and passed again.

## Verdict

```text
PROCEED
```
