# CHECKPOINT — Phase 2.5 (Portfolio-in-Prompt — Hayek signal attempt)

**Date**: 2026-04-21
**Branch**: `feat/tape-phase-2.5-portfolio-prompt`
**Commit**: `fc3ca82`

## Hypothesis
Adding a `=== Your Positions ===` block to the prompt showing each agent's
open YES/NO shares + current market price would act as a Hayek-style price
signal: if an agent sees it holds 10 YES on its own node with 50% market
probability, it can reason "appending earns me 10 Coins expected" and start
exploring tape. Pure state display, not rule (C-034 clean).

## Result: Hypothesis REFUTED
- **Solve rate 17/20 = 85%** — same as Phase 2.1c baseline (no regression; good)
- **`append: 0, invest: 0, complete_via_tape: 0`** — zero behaviour change
- 17/17 audit re-verified, +170 Coin distributed via mandatory wtool

Agents literally never see a non-empty portfolio block, because they never
execute append to populate one. Even when mandatory wtool automatically gives
them 10 YES on their own winning node, the payout happens at halt → the
portfolio is zeroed before next problem starts (wallet.portfolios persists,
but settle_portfolios zeros entries). Bootstrap problem: the signal requires
behaviour the signal was supposed to cause.

## Why the mechanism worked at integration-test level but not in practice
- `test_prompt_surfaces_portfolio` passes — the prompt builder correctly
  emits the block when given data.
- Runtime telemetry shows the block fires for 0 prompts over the 20-problem
  batch because `portfolio.is_empty()` is always true (agents own nothing
  they haven't already redeemed).
- Under mandatory wtool, positions are created *and* cleared in the same
  `halt_and_settle` call — they never linger into a subsequent prompt.

## Red-line check

| # | Red line | Status |
|---|---|---|
| 1-7 | (all unchanged from Phase 2.1c) | ✓ |

## Stop conditions

| Condition | Threshold | Observed | Status |
|---|---|---|---|
| Solve rate | ≥ -5pp vs median (17 is median-ish) | 17/20 | ✓ |
| Re-verifiability | ≥90% | 100% | ✓ |
| Phase 2.5 hypothesis | `append > 0` or `invest > 0` | 0 | ❌ REFUTED |
| Red lines | none | none | ✓ |

## Interpretation

The bootstrap problem is now empirically proven: a reactive signal cannot
activate the behaviour that creates the signal. Three remaining unlock paths,
in increasing aggressiveness:

1. **Seed positions**: auto-issue every agent 1 YES share on a synthetic
   `root` node at genesis so portfolio is non-empty from tx 0. Quasi-nudge,
   C-001-risky (verify no mint).
2. **Structural gate (A)**: restrict each agent to ≤ 1 `complete` per
   problem; anything else must be `append`. Forces tree construction.
3. **Model upgrade**: deepseek-chat may fundamentally lack the meta-reasoning
   to notice portfolio state. Claude Opus or GPT-5 might.

## Recommendation: **CLOSE Phase 2.5 (keep infra, retire as refuted)**

Phase 2.5 code is constitutionally clean and the unit test proves the prompt
builder works — worth keeping for later when agents do have lingering
positions (e.g., Phase 5 with cross-session portfolios that haven't settled
yet). But as a standalone behaviour-change mechanism it does not bite.

**Do NOT merge `feat/tape-phase-2.5-portfolio-prompt` to main without
stacking it on a phase that produces non-empty portfolios.** Next experiment
should be the structural gate (Option A) — it is the only remaining
mechanism-level lever before model upgrade.
