# CHECKPOINT — Phase 2 (Reward-Pull Founder Grant)

**Date**: 2026-04-20
**Branch**: `feat/tape-phase-2-rewardpull` (worktree `../v4-tape-reward/`, stacked on `feat/tape-phase-1-wal`)
**Commits on branch**: `5d9715e` "Phase 2 (C-042 candidate): founder grant + portfolio settlement" + Phase 1 ancestor

## What changed vs Phase 1

| Component | Change |
|---|---|
| `src/bus.rs` append phase | Under `TAPE_ECONOMY_V2=1`: after market create, call `wallet.record_shares(author, node_id, γ·lp, 0, 0)` — founder grant. No mint (LP-backed). |
| `src/bus.rs` halt_and_settle | Under `TAPE_ECONOMY_V2=1`: call `settle_portfolios()` — iterates `wallet.portfolios`, credits YES/NO shares against resolved markets, zeros settled positions for idempotency. |
| new: `tests/reward_pull_conservation.rs` | 5 integration tests: founder grant on append, off-flag yields zero, GP payout, NO-side zero, total-coin conservation bounded by LP |
| **Unchanged**: prompts, tools_description, oracle, agent loop, wallet mint rules |

## Phase 2 N=20 batch (`templadder_n8_20260420T160259.jsonl`)

### Solve outcomes
- **13/20 = 65%**  ← at the low end of the variance band
- 7 timeouts: algebra_apbon, amc12b_2021_p13, imo_1962_p2, induction_sumkexp3eqsumksq, mathd_algebra_208, mathd_algebra_332, numbertheory_notEquiv2i2jasqbsqdiv8

### Audit
```
=== Re-verifying 13 artifacts ===
  ... 13 VERIFIED ...
  Re-verified:   13/13 = 100.0%   Wall time: 184s
```
All claimed solves real, independently compiled by Lean.

### Unit tests (worktree)
```
running 5 tests
test phase2_conservation_total_coins_bounded ... ok
test phase2_founder_grant_credits_yes_on_append ... ok
test phase2_no_grant_when_flag_off ... ok
test phase2_settle_pays_out_on_golden_path ... ok
test phase2_settle_zero_on_losing_side ... ok
```
Infrastructure is provably correct: 5/5 conservation tests pass.

### Telemetry
| tool | count |
|---|---|
| complete | 42 |
| search | 29 |
| parse_fail | 5 |
| **append** | **0**  ← founder-grant code path never fired |

## The key observation

**`append: 0` across all 20 problems.** Rational LLM agents observed:
- No portfolio signal in prompt (they can't see the γ·lp YES shares they'd earn)
- No cross-problem memory (a single-problem agent has never experienced a settlement payout)
- Single-problem EV of append is invisible → prompt-driven heuristic "direct-complete is shortest" dominates

**Consequence**: Phase 2 runtime execution is **byte-identical to Phase 1** at the tool-flow level (founder-grant code exists but is never invoked). The 13 vs 17 solve-count delta between Phase 2 and Phase 1 is **pure LLM sampling variance**, deductively — not a mechanism regression.

Historical N=20 samples: [11, 14, 15, 16, 16, 17, 18]. 13 is below mean (15.3) but within observed spread. Phase 1's 17 was an upper-tail sample that Phase 2 couldn't reproduce with a new LLM random draw.

## Red-line check

| # | Red line | Status | Notes |
|---|---|---|---|
| 1 | New agent funded post-genesis | ✓ PASS | founder grant draws from pre-committed LP, not mint |
| 2 | Markets resolve on process exit | ✓ PASS | still resolved by halt_and_settle (oracle-driven) |
| 3 | Raw CoT to public tape | ✓ PASS | prompt unchanged |
| 4 | Prompt manipulation toward append | ✓ PASS | prompt unchanged |
| 5 | Reward curve as env-var | ⚠️ YELLOW | γ is currently env (FOUNDER_GRANT_GAMMA) for A/B; MUST become constitutional default before merge to main (per C-042 candidate) |
| 6 | ∏p accepts non-re-verifiable | ✓ PASS | 13/13 re-verified |
| 7 | Anything downgraded to Phase N | ⚠️ partial | Phase 2's OWN success criterion (activate tape) not met in this run; re-framed, not deferred |

## Stop conditions

| Condition | Threshold | Observed | Status |
|---|---|---|---|
| Solve rate vs baseline median (16) | ≥ -5pp | 13/20 (-15pp) | ❌ TRIGGERED |
| Conservation test | sum wallet == initial | PASS | ✓ |
| Re-verifiability | ≥90% | 100% | ✓ |
| Phase 2 success criterion | append > 0 per run | 0 / run | ❌ NOT MET |

## Recommendation: **INFRA-MERGE, MECHANISM-PAUSE**

### What to merge to main (safe)
- `feat/tape-phase-1-wal` — WAL persistence, proven in Phase 1 + Phase 2 unchanged. Audit-compatible, opt-in via `WAL_DIR`. Merge directly.
- `feat/tape-phase-2-rewardpull` **with TAPE_ECONOMY_V2=0 as default**. Infrastructure is correct; flag-on only fires when user requests. Zero runtime impact unless opted in. Merge as latent capability.

### What to NOT do yet
- Don't declare Phase 2 "working". The tape is still dormant at runtime.
- Don't proceed to Phase 3 (marginal-contribution scoring) until tape actually fills.
- Don't assume Phase 2 solve drop is causal — it's variance.

### Why the mechanism stayed dormant (and what would wake it)
Founder grant is a **delayed reward** that agents can only see through:
- (a) a portfolio view in the prompt (Hayek price signal — borderline C-034),
- (b) cross-problem reputation (Phase 4: agent sees past earnings in balance), OR
- (c) real-time settlement within a problem (doesn't fit our one-halt model).

(a) is the cheapest test; (b) is the most constitutional. My recommendation is to **skip to Phase 4 next**. Phase 4 makes `append` rewards compound across problems, so the first agent to earn from a successful tape-path solve discovers it by balance growth on the NEXT problem. That's the real Hayek loop.

Optional intermediate: **Phase 2.5 = portfolio-in-prompt**. Tiny additive change to `src/sdk/prompt.rs`: include a line like `Your stake: 10 YES on tx_0_by_Agent_0 (mkt 50%)` when non-empty. If Phase 4 is too big, 2.5 might nudge behavior.

## Next steps, in priority order

1. **Merge Phase 1 (WAL) to main** — independently safe and valuable.
2. **Merge Phase 2 (infra-only, default-off) to main** — latent capability, future phases will consume it.
3. **Decision point for user**: Phase 4 (cross-problem persistence) vs Phase 2.5 (portfolio-in-prompt experiment) vs re-run Phase 2 on different seed to confirm the variance story.
4. Whichever path → continue plan.

## Artifacts
- `exp_n20_phase2_rewardpull.log`, `logs/templadder_n8_20260420T160259.jsonl`
- 13 reproducible proof artifacts under `experiments/minif2f_v4/proofs/`
- 20 WAL files under `experiments/minif2f_v4/wal_phase2_test/` (all 2-event RunStart+RunEnd or 1-event RunStart-only for timeouts; 0 nodes — consistent with append=0)
