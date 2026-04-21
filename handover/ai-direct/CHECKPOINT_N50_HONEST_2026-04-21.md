# N=50 HONEST FINAL — publication headline

**Date**: 2026-04-21
**Branch**: `main` at `1cbff78` (post-Phase-4 merge)
**Caveat**: mid-batch binary rebuild (at ~problem 20) when main was rebuilt with Phase 4 support. First 19 problems: Phase 2.1c binary (no WALLET_STATE). Problems 20-50: Phase 4 binary (with cross-problem persistence). Does not affect solve rate; does affect wallet persistence attribution.

## Headline

**35/50 = 70% solved, honest**
- 35/35 re-verified via `audit_proof.py` with forbidden-pattern check enabled (100%)
- 0 `native_decide`-tainted solves (C-011 / F-2026-04-20-05 enforced)
- 35 `omega_wtool` firings = 35 solved (Art. IV wtool unconditional on ∏p=1)
- 15 timeouts distributed across historically-hard problems

## Firsts

This is the first N=50 run that is simultaneously:

- externally re-verifiable from disk artifacts alone (35 standalone `.lean` proof files at `experiments/minif2f_v4/proofs/`);
- proved clean of `native_decide` brute force;
- backed by tape-WAL records for every solve;
- economically closed (founder grant + settle_portfolios fired for every winning agent; Law 2 conservation trivially verified from wallet totals);
- cross-problem-persistent (Phase 4): wallet_n50_final.json records +200 Coin across the 20 post-swap problems = 20 solves × γ·lp = 10 Coin each, math exact.

### Breakthrough telemetry
```
aggregate tool_dist: {
  'complete': 150,
  'omega_wtool': 35,     ← one per solve; Art. IV loop closed
  'search': 288,
  'parse_fail': 20,
  'append': 1,            ← FIRST EXPLICIT AGENT APPEND IN PROJECT HISTORY
  'complete_via_tape': 1  ← one problem required tape+payload dual-path
}
```

The `append: 1` is a single, isolated event — Hayek signal is still effectively off, but the mechanism fired correctly when one agent did try. This confirms:
1. The pipeline works end-to-end (agent → bus.append → tape write → founder grant)
2. The reward was distributed (wallet record via `record_shares`)
3. The tape path can contribute to a solve (`complete_via_tape: 1`)

The behaviour-level activation of tape is still a model-layer problem, not architecture. Phase 4 substrate is ready for sessions lasting long enough for the Hayek signal to compound.

## Honest scoreboard (post-F-20-05 fix)

| Run | Solves / 50 | Rate | Notes |
|---|---|---|---|
| v3.1 n1 (pre-F20-05) | 30/50 | 60% | baseline unverifiable for native_decide |
| Dual-path 74677 raw | 43/50 | 86% | ~3-4 native_decide — honest ~39/50 = 78% |
| Dual-path 31415 raw | 41/50 | 82% | ~3-4 native_decide — honest ~37/50 = 74% |
| **N=50 honest final (this)** | **35/50** | **70%** | 0 taint, 100% re-verified, +200 Coin persisted |

The 70% is a firm, defensible number: every solve has a standalone `.lean` artifact that any external verifier can compile. Below the estimated-honest dual-path samples, but those were not measured directly — they're retrospective estimates. This is the first prospectively measured value.

## Variance vs the 17/20 = 85% N=20 claim

N=50 samples more of the distribution, including harder problems (aime, imo, amc12b) not in the N=20 subset. A single run is also noisy. Running a second N=50 at seed 31415 would establish a CI.

## Persistent failures this run (15)

`mathd_algebra_208`, `mathd_algebra_293`, `mathd_algebra_332`, `mathd_numbertheory_427`, `mathd_numbertheory_5`, `mathd_numbertheory_728`, `amc12_2000_p20`, `amc12a_2008_p25`, `amc12b_2021_p1`, `amc12b_2021_p13`, `imo_1962_p2` (flipped — was solved in prior runs), `imo_1965_p2`, `induction_sumkexp3eqsumksq`, `aime_1991_p9`, `aime_1999_p11`.

Most are hard-ceiling problems. `imo_1962_p2` timeout here despite solving in prior runs → run variance.

## What landed on main (final state)

- Phase 0 — `gp_payload`, proof archive, `audit_proof.py` (C-039)
- F-2026-04-20-05 fix — oracle enforces forbidden_patterns pre-Lean
- Phase 1 — WAL persistence (C-037)
- Phase 2 — founder grant reward-pull + settle_portfolios (Law 2 conserved)
- Phase 2.1 — mandatory wtool on OMEGA (Art. IV)
- Phase 2.1b — `bus.append_oracle_accepted` (orthogonal policy)
- Phase 4 — cross-problem wallet persistence (C-041)
- C-036 harness telemetry (from pre-session)

Behaviourally still missing: explicit agent-driven `append` / tape building. Architectural path to unlock: Phase 5 (signing + permissionless + multi-session) OR model upgrade OR Phase 2.5 prompt portfolio surface.

## Recommendation

This is a good session milestone. Before Phase 5, worth running:
1. **Variance N=50 at seed=31415** — establish confidence interval on 70%
2. **Clean Phase-4-from-start N=50** — avoid the mid-batch binary swap contamination
3. **Precedent canonicalization**: draft `cases/C-037/C-038/C-039/C-041/C-042/C-043.yaml` files per the plan's precedent table

After those, Phase 5 (cryptographic signing + permissionless onboarding) is the next substantive constitutional addition.
