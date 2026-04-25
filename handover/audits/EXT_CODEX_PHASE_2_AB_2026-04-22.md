# EXT_CODEX_PHASE_2_AB_2026-04-22

## Meta
- auditor: codex
- date: 2026-04-22
- scope: Phase 2 A/B verdict + Phase 9.M Hayek pivot soundness
- artifacts_reviewed:
  - `handover/ai-direct/EXT_AUDIT_PHASE_2_AB_2026-04-22/brief.md`
  - `handover/ai-direct/EXT_AUDIT_PHASE_2_AB_2026-04-22/per_problem.tsv`
  - `experiments/minif2f_v4/logs/phase8_baseline_main_oneshot_20260422T122117.jsonl`
  - `.claude/worktrees/phase-8a-snapshot/experiments/minif2f_v4/logs/phase8_experiment_oneshot_20260422T122119.jsonl`
  - `handover/audits/PPUT_RAW_DATA_2026-04-22.md`
  - `handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md`
  - `handover/ai-direct/REGISTRATION_PHASE_9_2026-04-22.md`
  - `constitution.md`
  - `handover/audits/EXT_CODEX_PHASE_8_R1ALPHA_2026-04-22.md`
  - `handover/audits/EXT_GEMINI_PHASE_8_R1ALPHA_2026-04-22.md`

## Q1 Phase 2 A/B verdict
- choice: b
- one_line_reason: Re-run one independent seed; this seed mechanically fails the registered gate, but the raw paired evidence is too outlier-dominated and underpowered to justify either a hard regression story or a direct pivot.
- evidence:
  - Raw-jsonl recompute matches `per_problem.tsv` exactly: baseline `8/20`, experiment `7/20`, `ΣPPUT 9.163` vs `6.917`, overlap `7 both / 1 main-only / 0 exp-only / 12 neither`, and paired mean `ΔPPUT = -0.112` with an approximate t95 interval `[-0.313, +0.088]`, so the sign is negative but not resolved at `N=20`.
  - The loss is concentrated in `per_problem.tsv` rows `mathd_algebra_359` (`2.942 -> 1.083`) and `mathd_algebra_160` (`2.285 -> 1.892`): together they contribute `-2.251` PPUT while the full net gap is `-2.246`, so the brief's `83%` attribution is unverified from the raw rows; both jsonl files also share `condition=oneshot` and `model=deepseek-reasoner`, so there is no observed asymmetric config confound, while the brief's same-machine parallel-risk note is unverified from the logs.

## Q2 Hayek pivot soundness
### Q2.1 C-049 root-cause hypothesis
- verdict: challenge
- rationale: `AUTO_RESEARCH_NOTEPAD.md:57-64` confirms C-049 (`bus.snapshot()` exposed empty balances), but `AUTO_RESEARCH_NOTEPAD.md:191-200` separately records tape-economy v2 as `append: 0`, `complete_cold_fee: 54`, and "economic cold fee alone cannot activate tape", so treating F-20-04 as primarily an IO-bug failure remains a brief-side hypothesis (`brief.md:59`), not an observed root cause.

### Q2.2 M1 / M4 / M7 / M8 operationalizability
Cross-check: `REGISTRATION_PHASE_9_2026-04-22.md` does not enumerate `M1/M4/M7/M8`; that menu appears in `brief.md:65-69`, so the verdicts below are conservative.

- M1: defensible — `AUTO_RESEARCH_NOTEPAD.md:35` says the Phase 3A Hayek Problem Bounty Market is implemented, and `REGISTRATION_PHASE_9_2026-04-22.md:103-105` already assumes wallet/market payout plumbing, so a dynamic founder-grant variant looks like an incremental mechanism change rather than a greenfield subsystem.
- M4: challenge — `AUTO_RESEARCH_NOTEPAD.md:36` says Phase 3B Satoshi Citation Rebate is only queued and depends on depth ancestry, and the current registration defines no citation metric or acceptance rule for it, so the idea is plausible but not execution-ready.
- M7: challenge — `REGISTRATION_PHASE_9_2026-04-22.md:103-105` proves wallet/refund machinery exists, but no required artifact specifies append-stake escrow, refund, or slashing semantics, so this is still a design sketch rather than a ready bake-off arm.
- M8: challenge — `REGISTRATION_PHASE_9_2026-04-22.md:104-105` mentions `market.lp_reserves` and `settle_portfolios`, so LP primitives exist, but neither the registration nor the notepad defines a bonding-curve market maker or agent-LP policy, which makes the current timeline aspirational.

### Q2.3 Paper 1 thesis upgrade
- verdict: over-claim
- rationale: `constitution.md` Art. I.2 and Art. II.2/II.2.1 justify price signals conceptually, but `AUTO_RESEARCH_NOTEPAD.md:137-143`, `191-200`, and `558-564` still show tape/markets dormant in practice, while `REGISTRATION_PHASE_9_2026-04-22.md:46-50` frames Phase 9 as a post-Phase-8 statistical baseline rather than evidence that market mechanisms already drive faster proof discovery.

## Overall
- recommendation: ITERATE
- one_line_reason: Re-run one new Phase 2 seed and, if the market pivot is still desired, register it as a new Phase 9 revision because the current registration requires a PASS first (`REGISTRATION_PHASE_9_2026-04-22.md:48-50`) and does not itself specify `M1/M4/M7/M8`; prior Phase 8 audits also disagreed on swarm-readiness details (`EXT_CODEX_PHASE_8_R1ALPHA_2026-04-22.md:60-62` vs `EXT_GEMINI_PHASE_8_R1ALPHA_2026-04-22.md:35-41`), so the conservative call is iteration, not direct proceed.

## Return-to-caller block (for Claude to paste back)
- Q1: b — re-run one seed; the strict fail is real on this seed, but the raw paired result is still too concentrated and inconclusive to close as a regression or skip the gate.
- Q2: challenge — C-049 is real, but the pivot menu is only partially grounded in the registered artifacts and the Paper 1 market-thesis upgrade is currently an over-claim.
- Batch: ITERATE
