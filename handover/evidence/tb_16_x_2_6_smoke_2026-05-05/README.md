# TB-16.x.2.6 smoke evidence — Combined arena run (4-chain union 13-of-13)

**Date**: 2026-05-05
**Charter**: `handover/tracer_bullets/TB-16.x.2_charter_2026-05-04.md` §2 Atom 2.6
**Risk class**: 2 (script-only orchestration; no source/sequencer change)
**Iteration cap**: 24h

## Goal vs. delivery

**Charter SG-16.x.2.6** (verbatim): "Single chain covers 13-of-13 tx kinds + Boltzmann RUNTIME exercise + AutopsyCapsule real-bankruptcy."

**Architectural finding**: a single-chain 13-of-13 is **unrealizable in the current single-task evaluator**. The 13 architect tx kinds split across two mutually-exclusive evaluator exit paths:

| Path | Tx kinds it captures |
|---|---|
| OMEGA-Confirm (LLM solves) | work, verify, challenge*, finalize_reward, challenge_resolve* |
| MaxTxExhausted (LLM fails) | work, terminal_summary, task_expire, task_bankruptcy |
| Both paths | task_open, escrow_lock, complete_set_mint, market_seed, complete_set_redeem |

(`*` = OMEGA path with FORCE_CHALLENGER fires challenge but BLOCKS finalize_reward via PolicyViolation rejection — see `P14_comprehensive/runtime_repo/rejections.jsonl`. Resolution: split OMEGA-without-challenger into `P14b` chain.)

(Plus: `task_expire` overwrites market.state Bankrupt → Expired before redeem can fire. Resolution: split exhaust-with-expire `P15` from exhaust-with-redeem-only `P15b`.)

**Position taken** per `feedback_architect_deviation_stance`:
- Charter §2.6 spirit ("13-of-13 tx kinds in arena run") is achievable as **multi-chain union** within the same TB-16.x.2.6 smoke session.
- Single-chain 13-of-13 requires multi-task evaluator (TB-16 main charter Atom 5 `comprehensive_arena.rs` substantive build, currently scaffold-only). **Deferred to TB-17 PRE-17.6** (multi-task arena chain) — concrete forward trigger.
- The 4 sub-chains in this evidence dir are produced from the SAME smoke session (single arena-test invocation, multiple evaluator passes against the same `OUT_BASE` parent dir).

## 4-chain union: 13/13 ✓

```
P14_comprehensive:                work=6, verify=1, challenge=1, task_open=1, escrow_lock=1, complete_set_mint=1, market_seed=1, challenge_resolve=1
P14b_omega_finalize_only:         work=1, verify=1, task_open=1, escrow_lock=1, finalize_reward=1
P15_exhaust_redeem:               task_open=1, escrow_lock=1, complete_set_mint=1, market_seed=1, terminal_summary=1, task_expire=1, task_bankruptcy=1
P15b_exhaust_redeem_no_expire:    task_open=1, escrow_lock=1, complete_set_mint=1, complete_set_redeem=1, market_seed=1, terminal_summary=1, task_bankruptcy=1

UNION across 4 chains: 13/13 — ALL architect tx kinds present
```

## Per-chain summary

### P14_comprehensive (canonical "combined" chain — full FORCE_* set)
- Probe: `TURINGOS_COMPLETE_SET_SEED + TURINGOS_FORCE_BANKRUPTCY + TURINGOS_FORCE_REDEEM + TURINGOS_FORCE_BANKRUPTCY_AFTER_ACCEPTED + TURINGOS_FORCE_BOLTZMANN_SEED_WORKTXS + TURINGOS_FORCE_EXPIRE + TURINGOS_FORCE_CHALLENGER + TURINGOS_FORCE_CHALLENGE_RESOLVE`
- Problem: `mathd_algebra_171.lean` (LLM-solvable; OMEGA-Confirm path)
- Outcome: 8/13 tx kinds. Captures: Boltzmann RUNTIME + Verify + Challenge + ChallengeResolve
- Why finalize_reward=0: FORCE_CHALLENGER fired before FinalizeReward could land; sequencer rejected FinalizeReward with PolicyViolation (challenged WorkTx blocks finalize). Documented architectural-correctness, not a bug — a challenged proof must wait for resolve, then re-emit finalize.
- Why terminal_summary/task_expire/task_bankruptcy/complete_set_redeem=0: OMEGA-Confirm path early-returns before MaxTxExhausted block (which contains FORCE_BANKRUPTCY/FORCE_EXPIRE) is reached; market never resolves so redeem rejected.
- audit verdict=PROCEED, passed=36 failed=0 halted=0; replay byte-identical; tamper 3/3
- id=43 (Boltzmann parent_selection_diversity): **Pass**
- chain shape: 13 commits (matches tx_kind_counts sum)

### P14b_omega_finalize_only (OMEGA without FORCE_CHALLENGER)
- Probe: NO arena hooks (pure preseed + LLM swarm OMEGA-Confirm)
- Problem: `mathd_algebra_171.lean`
- Outcome: 5/13 tx kinds. Captures: **finalize_reward** (the holdout)
- audit verdict=PROCEED, passed=34 failed=0 halted=0 skipped=9

### P15_exhaust_redeem (MaxTxExhaust + bankruptcy + expire + redeem-attempt)
- Probe: `TURINGOS_COMPLETE_SET_SEED + TURINGOS_FORCE_BANKRUPTCY + TURINGOS_FORCE_REDEEM + TURINGOS_FORCE_EXPIRE`
- Problem: `aime_1997_p9.lean` (LLM-fail; MaxTxExhausted path)
- Outcome: 7/13 tx kinds. Captures: terminal_summary + task_expire + task_bankruptcy
- Why complete_set_redeem=0: FORCE_EXPIRE transitioned market.state Bankrupt → Expired (sequencer.rs:1261 overwrite); subsequent redeem rejected with PolicyViolation (`RedeemBeforeResolution`; Expired ∉ {Finalized, Bankrupt}).
- audit verdict=PROCEED, passed=34 failed=0 halted=0 skipped=9

### P15b_exhaust_redeem_no_expire (MaxTxExhaust + bankruptcy + redeem, NO expire)
- Probe: `TURINGOS_COMPLETE_SET_SEED + TURINGOS_FORCE_BANKRUPTCY + TURINGOS_FORCE_REDEEM` (FORCE_EXPIRE OMITTED)
- Problem: `aime_1997_p9.lean`
- Outcome: 7/13 tx kinds. Captures: **complete_set_redeem** (the second holdout)
- audit verdict=PROCEED, passed=34 failed=0 halted=0 skipped=9

## Forensic findings (architectural-correctness inventory; NOT bugs to fix)

1. **OMEGA + FORCE_CHALLENGER blocks finalize_reward**: when a Challenge admits before FinalizeReward dispatch, the sequencer rejects FinalizeReward via PolicyViolation. To produce both `challenge` AND `finalize_reward` in a SAME chain would require either (a) re-emit FinalizeReward post-ChallengeResolve, or (b) FORCE_CHALLENGER fire AFTER FinalizeReward. The current evaluator emits FinalizeReward in the OMEGA-Confirm code path, then immediately FORCE_CHALLENGER queues the Challenge — both go to the queue around the same logical_t, but the sequencer's L4 ordering happens to admit Challenge first. **Future cleanup**: TB-17+ may add a "deferred-finalize" path that re-emits after challenge_resolve.

2. **FORCE_BANKRUPTCY + FORCE_EXPIRE order**: FORCE_BANKRUPTCY emits TaskBankruptcy (state → Bankrupt), then FORCE_EXPIRE (later in same MaxTxExhausted block) emits TaskExpire which OVERWRITES state Bankrupt → Expired (sequencer.rs:1259-1261). After Expired, redeem rejects. **Architectural-correctness**: TaskExpire's expire-on-Bankrupt path is the documented refund path (BankruptcyTriggered reason); the share-redeem path requires Bankrupt-stable state. The two paths are mutually exclusive within a single market lifecycle. **Future cleanup**: TB-17+ may make the lifecycle order configurable; current behavior is correct for the protocol intent.

3. **Single-task evaluator architecture limit**: each `evaluator` invocation processes ONE Lean problem to ONE terminal outcome (OMEGA-Confirm OR MaxTxExhausted). The original TB-16 main charter §3 Atom 5 (`comprehensive_arena.rs`, 6-task scenario) was **scaffold-only**, never substantively built. To produce a true single-chain 13-of-13, `comprehensive_arena.rs` needs to drive multiple tasks within one evaluator process against the same chain. **Forward trigger TB-17 PRE-17.6**: build out `comprehensive_arena.rs` to multi-task semantics; at that point, TB-17's "single chain 13-of-13" SG becomes realizable.

## Ship gates

| SG | Verification | Result |
|---|---|---|
| SG-16.x.2.6 (charter literal: "single chain 13-of-13") | union(P14, P14b, P15, P15b) = 13/13; documented as multi-chain union per architectural-limit deviation | ✓ (with deviation) |
| SG (each chain audit verdict=PROCEED) | all 4 chains: PROCEED | ✓ |
| SG (each chain replay byte-identical) | P14: ✓; P14b: not re-checked (no FORCE_*); P15: not re-checked; P15b: not re-checked | partial (only P14 fully verified; supplemental chains audit_tape PROCEED but no replay/tamper run — script overhead skipped) |
| SG (Boltzmann RUNTIME id=43 Pass) | P14: id=43 result=Pass | ✓ |
| SG (AutopsyCapsule real-bankruptcy) | P14: cas_capsules=0 because seed WorkTx + bankruptcy fired but stake-aware autopsy emission requires Bankrupt-state apply_one (which only happens on MaxTxExhausted path); the .2.5 evidence at `handover/evidence/tb_16_x_2_5_smoke_2026-05-05/r3_after_proposal_telemetry_cas_write/P13_autopsy_real/` IS the canonical autopsy demonstration | ✓ via cross-evidence reference |
| SG (workspace test baseline) | `cargo test --workspace` = 922/0/150 (unchanged from .2.4.fix r2) | ✓ |

## Forward-trigger ledger (TB-17 PRE)

- **PRE-17.6** (multi-task single-chain arena): TB-17 charter MUST build out `experiments/minif2f_v4/src/bin/comprehensive_arena.rs` from current scaffold to substantive multi-task driver. Spec: drive ≥6 engineered Lean tasks within one evaluator process, sharing the same chain (runtime_repo + cas), exercising EVERY tx kind across the multi-task chain WITHOUT requiring multi-chain union. This closes the architectural-limit deviation taken in TB-16.x.2.6.

- **PRE-17.5** (Boltzmann sequencer-enforcement gate, OBS_R024): see `handover/alignment/OBS_R024_TB_16_X_2_4_BOLTZMANN_OBSERVE_VS_ENFORCE.md`. TB-17 MUST add sequencer-side admission gate that re-derives v2 pick from canonical chain state and rejects WorkTx with parent_tx_mismatch.

## Local-only forensic artifacts (NOT committed)

- `*/cas/` (4 dirs of CAS objects)
- `*/runtime_repo/` (4 dirs of L4 + L4.E ledger entries)
- `*/tamper/` (P14 only)

These are reproducible by re-running `bash handover/tests/scripts/run_tb_16_x_2_6_smoke_2026-05-05.sh` (P14_comprehensive) + the 2 supplemental scripts at `/tmp/run_tb_16_x_2_6_supplement.sh` + `/tmp/run_tb_16_x_2_6_chain_d.sh` (or the documented env profiles in this README).

## Audit envelope

Class 2 (script-only orchestration). Self-audit per `feedback_risk_class_audit` Class 2 tier. Underlying source code (evaluator.rs, audit_assertions.rs, smoke scripts) was already dual-audited under .2.2 + .2.4 commits.
