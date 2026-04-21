# TuringOS v4 — Handover State
**Updated**: 2026-04-21 (Phase 7 Turing δ-step merged; constitutional TuringOS complete)

## Constitutional TuringOS — all phases landed

Art. IV topology `Q_t → rtool → AI(δ) → output → ∏p → wtool → Q_{t+1}` is now fully executable at runtime, not only on paper.

| Phase | Landed | Purpose |
|---|---|---|
| 0 — Audit artifact | main `c0d76d2` | gp_payload + `proofs/*.lean` + audit_proof.py (C-039) |
| 1 — WAL | main `f63f0cb` | Q_t persistence across process restart (C-037) |
| 2 — Founder grant | main `f63f0cb` | γ·lp YES per append; settle at halt |
| 2.1 — Mandatory wtool | main `f63f0cb` | ∏p=1 ⇒ tape node written (C-043) |
| 2.1c — Oracle hardening | main `f72166e` | block native_decide bypass |
| 3A — Hayek bounty | main `7e6054b` | Problem-level market visible from tx 0 |
| 4 — Cross-problem wallet | main `c0518a5` | balance persists across runs (C-041) |
| 6-emergent — Librarian board | main `7e6054b` | Shared team board; agents self-select role |
| **7 — Turing δ-step** | **main (this merge)** | **per-tactic proof construction; depth-N DAGs** |

## Headline result — first Turing DAGs

Phase 7 N=20 with `TURING_STEP_ONLY=1`:

| Problem | GP depth | δ-writes | Audit |
|---|---|---|---|
| **imo_1964_p2** | **23** | 22 | ✓ |
| **mathd_algebra_332** | **20** | 19 | ✓ (persistent-fail cracked) |
| **imo_1981_p6** | **17** | 16 | ✓ |
| mathd_algebra_171 | 3 | 2 | ✓ |
| 5 easy | 1 | 0 | ✓ |
| **9/9 re-verified** | — | — | **100%** |

## Solve rates across configurations (N=20 honest)

| Config | solved | Golden-path distribution |
|---|---|---|
| Phase 2.1c baseline (prior economics + no step) | 17/20 | {1:17} — all one-shot |
| Phase 6-emergent (Hayek + board) | 15/20 | {1:14, 2:1} — one collaborative case |
| Phase 7 TURING_STEP_ONLY | 9/20 | {1:5, 3:1, 17:1, 20:1, 23:1} — real distribution |

Step-only loses 8 solves vs monolithic but produces the constitutional shape.

## Recommended production config (dual-mode)

`step` and `complete` both available in prompt (default, `TURING_STEP_ONLY=0`). Agents self-select:
- `complete` wins on easy problems (one-shot sampling)
- `step` wins on hard problems (depth-N construction)

Expected: solve rate recovers above 15/20, depth histogram remains diverse.

## Constitutional compliance

All seven red lines clean:
1. ✓ No post-genesis mint (Law 2 conservation exact across all phases)
2. ✓ Oracle-triggered settlement (not process-exit)
3. ✓ Only canonical payloads on public tape (not raw CoT)
4. ✓ No prompt manipulation toward append/step — tool availability IS mechanism
5. ⚠️ Reward curve in env vars (γ, β, θ, BOUNTY_LP) — lift to constitutional defaults before final release
6. ✓ Every accepted proof externally re-verifiable (audit_proof.py)
7. ✓ No deferrals

## What's next

- Dual-mode N=50 honest benchmark
- Lift env-var reward curve to constitutional defaults
- Phase 3B Satoshi rebate now meaningful (ancestry chains of depth 17-23 pay ancestors)
- Phase 5 cryptographic / permissionless onboarding for external agents
- Model upgrade (Opus / GPT-5) should boost step acceptance rate

## Reference
- Checkpoints: `handover/ai-direct/CHECKPOINT_PHASE_*.md` (0, 1, 2, 2_1c, 4, 2_5, 3A+6_emergent, 7)
- Plan: `~/.claude/plans/replicated-enchanting-sundae.md`
- All branches preserved: feat/tape-phase-1-wal, ...2-rewardpull, ...2.1, ...2.5, ...4, phase-3a-hayek, ...3b-satoshi, ...6-emergent, ...7-turing
