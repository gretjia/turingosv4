# TuringOS v4 — Handover State
**Updated**: 2026-04-21
**Session Summary**: Full-plan execution complete through Phase 4. All constitutional capabilities landed on main. First honest N=20 = 17/20 = 85%, 100% externally re-verifiable. Discovered and fixed F-2026-04-20-05 (native_decide bypass that had inflated all prior claims).

## Current State

### Honest headline
| Run | Solves / 20 | Rate | Audit |
|---|---|---|---|
| Phase 2.1c (first honest) | 17/20 | **85%** | 17/17 re-verified, 0 taint |
| Main validation post-merge | 17/20 | 85% | (pending re-audit) |
| Phase 4 cross-persist | 17/20 | 85% | 17/17 re-verified |
| **N=50 honest, in flight** | TBD | — | launched 2026-04-21 |

### Capabilities landed on main (all commits)
- **Phase 0** (`c0d76d2`): `gp_payload` field + `proofs/*.lean` standalone artifacts + `audit_proof.py` offline verifier. Closes the audit gap (C-039).
- **F-2026-04-20-05 fix** (`f72166e`): `verify_omega_detailed` now enforces `check_payload` pre-Lean. Closes `native_decide` bypass that inflated 17 historical solves across 5 runs.
- **Phase 1 WAL** (merged via `f63f0cb`): `src/wal.rs` + `bus.with_wal_path`. Q_t persists across process restart; crash-resume integration test passes (C-037).
- **Phase 2 reward-pull** (same merge): founder grant γ·lp YES shares on every append; halt settles portfolio → author wallet. 5/5 conservation unit tests pass.
- **Phase 2.1 mandatory wtool** (same merge): every OMEGA-accept writes a tape node via `bus.append_oracle_accepted` → founder grant fires → balance += γ·lp. Art. IV `∏p=1 ⟹ wtool` now architecturally enforced, not optional.
- **Phase 4 cross-problem** (`7781958`): wallet + portfolio save/load via `WALLET_STATE` env. Reputation accumulates across problems. Law 2 holds (genesis_done persists, no re-mint).

### What's still OPEN (not blocking)

1. **`append: 0` behaviourally**. Agents never explicitly call the `append` tool — they always `complete`. The mandatory-wtool path auto-writes anyway, so tape evolves and economics work, but the "agent DECIDES to build tape" Hayek behaviour is dormant.
   - Root: single-session LLM can't infer "balance grew because I did X" across transactions.
   - Unlock candidates: portfolio-in-prompt (C-034 borderline), permissionless multi-session (Phase 5), or stronger model.
2. **3 persistent failures** across multiple runs: `amc12b_2021_p13`, `induction_sumkexp3eqsumksq`, `mathd_algebra_332`. Proof-hard problems, not mechanism issues.
3. **γ hardcoded via env** (`FOUNDER_GRANT_GAMMA=0.05`, yellow on C-042). Should become a constitutional default in a follow-up.

### Active experiments
- **N=50 honest final** (PID in exp_n50_final_honest.log): fair single-run headline with all capabilities + F-20-05 filter. ETA ~3-4h.

## Next Steps (priority order, awaiting user direction)

1. **Wait for N=50 honest** → will be the first defensible headline number for external publication.
2. **C-037/C-038/C-039/C-041/C-042/C-043 precedents**: draft yaml files so the landed mechanisms are canonicalized (should not be skipped).
3. **Phase 5 (permissionless/signed) OR a model upgrade pass** — two very different directions:
   - Phase 5: ed25519 keys, external-agent registration, signed tape. ~1 week work.
   - Model upgrade: swap to a stronger LLM; try Opus or GPT-5; behaviour might change dramatically.
4. **Phase 2.5 (optional nudge)**: portfolio-in-prompt. 30-min tweak, might activate explicit-append. Low risk.

## Retroactive honest scoreboard after F-20-05

| Prior claim | Honest after filter |
|---|---|
| N=20 Phase 0 baseline 15/20 | 11/20 |
| N=20 Phase 1 WAL 17/20 | 13/20 |
| N=20 Phase 2 reward 13/20 | 10/20 |
| N=20 Phase 2.1 wtool 16/20 | 13/20 |
| N=20 Phase 2.1b oracle-accepted 17/20 | 14/20 |
| **N=20 Phase 2.1c (post-fix) 17/20** | **17/20 clean** |
| N=50 dual-path 43/50 (86%) | unknowable (no payload saved pre-Phase-0) |

## Reference
- Plan: `~/.claude/plans/replicated-enchanting-sundae.md`
- Notepad: `handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md` — F-2026-04-18/19/20 findings, including F-20-05 detail
- Checkpoints: `CHECKPOINT_PHASE_0/1/2/2_1c/4_*.md`
- Design doc: `TAPE_ECONOMY_v1_2026-04-20.md` (informative; v1 push mechanism was refuted; current implementation is v2 pull + mandatory wtool)
- Worktree branches preserved for archival: `feat/tape-phase-1-wal`, `feat/tape-phase-2-rewardpull`, `feat/tape-phase-2.1-mandatory-wtool`, `feat/tape-phase-4-cross-problem`. All effectively subsumed by main; can be removed at your discretion.
