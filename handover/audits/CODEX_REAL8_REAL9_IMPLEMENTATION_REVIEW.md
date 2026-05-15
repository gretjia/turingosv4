# CODEX REAL-8 / REAL-9 Implementation Review

Reviewer: clean-context Codex (Mendel, xhigh)
Date: 2026-05-15
Verdict: PROCEED

## Findings

No production defects found in the reviewed REAL-8/REAL-9 surfaces.

The C/D stale-parent fix looks sound: `real6_verify_parent_root_after_optional_market` refreshes from current `q_snapshot().state_root_t` before `VerifyTx` construction (`experiments/minif2f_v4/src/bin/evaluator.rs:681`), and both OMEGA paths call it after optional node-market emission and before signing/submitting `VerifyTx` (`experiments/minif2f_v4/src/bin/evaluator.rs:4718`, `experiments/minif2f_v4/src/bin/evaluator.rs:5849`). The fix does not bypass sequencer admission: the Verify arm still rejects stale parents at Step 1 (`src/state/sequencer.rs:1079`), and the `VerifyTx` canonical signing payload includes `parent_state_root` (`src/state/typed_tx.rs:976`).

REAL-8 input pinning and claim boundaries are present: the runner copies/hashes one shared problem/model/budget set (`scripts/run_real8_market_ab_benchmark.sh:72`), exports the same model/budget settings before arm-specific toggles (`scripts/run_real8_market_ab_benchmark.sh:230`), and limits conclusions to descriptive evidence/negative results (`scripts/run_real8_market_ab_benchmark.sh:329`). Final evidence matches: all four arms are `exit=0`, `audit=PROCEED`, 3 tasks; market tx counts are A=0, B=4, C=10, D=10.

REAL-9 docs satisfy the architect boundary: v4 does not copy v3, price is signal not truth, markets are role-specific institutions, and forbidden claims are explicitly listed (`handover/whitepapers/TURINGOS_GENERATIVE_ECONOMY_WHITEPAPER_UPDATE_REAL9.md:7`, `handover/whitepapers/TURINGOS_MARKET_DEVELOPER_MANUAL_REAL9.md:7`).

Non-blocking test-scaffold gap: `real8_task_outcome_arm_refreshes_verify_parent_after_auto_market` is source-grep/count based rather than a direct behavioral unit test (`tests/constitution_real8_market_ab_benchmark.rs:115`). The final C/D real chain evidence covers the behavioral path, so I do not consider this ship-blocking.

I reran the Trust Root single test locally; it passed. I also inspected the final REAL-8 evidence and dev harness artifacts showing targeted REAL-8/9 tests, REAL-8 benchmark run, constitution gates, and workspace tests passing.

PROCEED
