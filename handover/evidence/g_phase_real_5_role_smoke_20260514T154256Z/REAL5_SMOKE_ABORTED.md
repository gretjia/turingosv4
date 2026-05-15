# REAL-5 Role Smoke Abort Note

UTC: 2026-05-14T15:56:41Z

Command:

```bash
TURINGOS_G_PHASE_DIRTY_OK=1 \
PHASE_D_HETERO_OK=1 \
TURINGOS_MARKET_ARENA_PROMPT=1 \
TURINGOS_TB_N3_AUTO_MARKET=1 \
TURINGOS_REAL5_ROLE_VIEWS=1 \
TURINGOS_REAL5_ROLE_ASSIGNMENT="Solver,Trader,Verifier,Challenger,Observer" \
TURINGOS_G_PHASE_N_AGENTS=5 \
PER_PROBLEM_TIMEOUT_S=240 \
bash scripts/run_g_phase_batch.sh g_phase_real_5_role_smoke_20260514T154256Z mini
```

Observed:

- `G_PHASE_BATCH_MANIFEST.json` was written with `real5_role_assignment = "Solver,Trader,Verifier,Challenger,Observer"` and `real5_role_views_enabled = "1"`.
- Task 0 reached `BoundaryPrep::FreshGenesis`.
- Task 1 reached `ChainTapeLease ACQUIRED` and `ResumePreflight::Ok`, with `chain_length=6`.
- Task 1 evaluator artifacts existed, but both stdout and stderr were empty:

```text
0 P001_mathd_algebra_125/evaluator.stdout
0 P001_mathd_algebra_125/evaluator.stderr
```

Reason for abort:

The run exceeded the configured `PER_PROBLEM_TIMEOUT_S=240` window by a wide margin while task 1 produced no evaluator stdout/stderr. Codex terminated the smoke processes to avoid an unbounded run. This is failed smoke evidence, not REAL-5 ship evidence.

Follow-up:

Before REAL-5 can claim the script-level smoke gate, the batch timeout/evaluator hang path should be made bounded or the smoke should be rerun on a known-responsive proxy/profile. The unit and constitution gates still cover the role scaffold; this aborted run only blocks the live `run_g_phase_batch.sh` smoke claim.
