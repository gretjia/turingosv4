# CHECKPOINT — Phase 2.1c (Honest Art. IV compliance)

**Date**: 2026-04-21
**Branch**: `feat/tape-phase-2.1-mandatory-wtool` (worktree `../v4-tape-wtool/`)
**Stack**: Phase 0 (audit) ← Phase 1 (WAL) ← Phase 2 (reward-pull) ← Phase 2.1 (mandatory wtool) ← Phase 2.1b (oracle-accepted append) ← Phase 2.1c (F-20-05 oracle fix)

## The milestone

**17/20 = 85% solved. 17/17 audit re-verified. 0 `native_decide`-tainted. 17 tape nodes in WAL = 1 per solve. `omega_wtool: 17 = solved: 17`.**

This is the first single-run in the project's history that is simultaneously:
- The project's highest solve rate (tied with prior inflated 17/20 from Phase 1)
- Fully externally re-verifiable (audit_proof.py passes 17/17 with forbidden-pattern check enabled)
- Fully Art. IV-compliant (every ∏p = 1 triggers wtool → a new tape node)
- Fully C-011-compliant (no brute-force `native_decide` in any solve)
- Fully Law-2-conserving (founder grant at γ·lp draws from LP, no mint — proven by 5/5 conservation unit tests)

## Phase-by-phase journey

| Phase | Claimed | After F-20-05 filter | Why it changed |
|---|---|---|---|
| Phase 0 (audit fix only) | 15/20 | 11/20 real | 4 solves were `native_decide` |
| Phase 1 (WAL) | 17/20 | 13/20 real | 4 tainted |
| Phase 2 (reward-pull infra) | 13/20 | 10/20 real | 3 tainted |
| Phase 2.1 (mandatory wtool) | 16/20 | 13/20 real | 3 tainted; bus re-rejected omega-using legit proofs |
| Phase 2.1b (oracle-accepted append) | 17/20 | 14/20 real | 3 still `native_decide` because oracle itself didn't check |
| **Phase 2.1c (this)** | **17/20** | **17/20 honest** | oracle now enforces FORBIDDEN_PATTERNS before Lean; zero taint |

## F-2026-04-20-05 (the bug that was hiding everything)

- Root cause: evaluator's `complete` handler called `Lean4Oracle::verify_omega_detailed` directly, bypassing the `on_pre_append → check_payload` gate where forbidden-pattern checks lived. `native_decide` (C-011 brute-force bytecode) was always silently accepted.
- Fix: `verify_omega_detailed` now calls `check_payload` at entry (inline pre-Lean). `audit_proof.py` mirrors the same FORBIDDEN_PATTERNS list.
- Historical impact: 17 solves across 5 post-Phase-0 batches retroactively invalidated. Pre-Phase-0 runs cannot be re-checked because no `gp_payload` was saved (exactly the gap Phase 0 was built to close).

## What's in this branch vs main

- **Main has**: Phase 0 audit fix (`c0d76d2`) + F-20-05 oracle fix (`f72166e`). The honest-baseline-capable evaluator for future work.
- **Branch adds**:
  - `src/wal.rs` + `bus.with_wal_path` → persistent Q_t
  - `bus.append` → founder grant (γ·lp YES to author) under `TAPE_ECONOMY_V2=1`
  - `bus.halt_and_settle` → `settle_portfolios` (resolved-side redemption)
  - `bus.append_oracle_accepted` → ∏p-blessed tape write
  - evaluator `complete` accept → mandatory wtool via oracle-accepted path
  - larger payload caps (max_payload_chars=8000, lines=200) for real proofs
  - `tests/wal_resume.rs` (crash-recovery) + `tests/reward_pull_conservation.rs` (founder grant + settlement)
- **Behavioural difference**: branch is an almost drop-in improvement. Tape WAL is opt-in via `WAL_DIR`. Founder grant is opt-in via `TAPE_ECONOMY_V2=1`. Mandatory wtool is always-on when `complete` succeeds (cheap; falls back gracefully if bus vetos for any reason).

## Red-line check

| # | Red line | Status |
|---|---|---|
| 1 | New agent funded post-genesis | ✓ PASS (founder grant from LP, not mint) |
| 2 | Markets resolve on process exit | ✓ PASS (oracle-driven) |
| 3 | Raw CoT to public tape | ✓ PASS (only final canonical payload) |
| 4 | Prompt manipulation toward append | ✓ PASS (prompt unchanged) |
| 5 | Reward curve as env-var | ⚠️ YELLOW (γ via FOUNDER_GRANT_GAMMA; needs to become constitutional default at merge — file an issue, not a blocker) |
| 6 | ∏p accepts non-re-verifiable | ✓ PASS (100% audit) |
| 7 | Anything downgraded to Phase N | ✓ PASS |

## Stop conditions

| Condition | Threshold | Observed | Status |
|---|---|---|---|
| Solve rate vs honest median | ≥ -5pp | 17/20 (above honest median 13) | ✓ well above |
| Conservation test | pass | 5/5 | ✓ |
| Re-verifiability | ≥90% | 100% | ✓ |
| Red lines | none | 1 yellow | ✓ (non-blocking) |
| Lean preflight | OK | all 17 compile independently | ✓ |
| Phase 2.1 success criterion | `omega_wtool ≥ solved` per run | 17=17 | ✓ |

## Recommendation: **MERGE to main**

Fast-forward `feat/tape-phase-2.1-mandatory-wtool` onto `main`. The stack is coherent, validated, and opt-in where it could be controversial. Main retains WAL+reward-pull+mandatory-wtool as standard capability; TAPE_ECONOMY_V2 flag can go default-on in a follow-up once Phase 3 (marginal contribution) refines the reward curve.

After merge, proceed to Phase 3 (marginal contribution scoring per Shapley-lift) OR Phase 4 (cross-problem reputation) per original plan §三. Both compound on the now-persistent tape.

## Artifacts
- `logs/templadder_n8_20260420T225140.jsonl` (17 solves, all with gp_payload)
- `experiments/minif2f_v4/proofs/*.lean` (17 standalone re-runnable proof files)
- `experiments/minif2f_v4/wal_phase2_1c_test/*.jsonl` (17 WALs with node records + lifecycle events)
- Worktree `../v4-tape-wtool/` preserved for archival; can be removed after merge.
