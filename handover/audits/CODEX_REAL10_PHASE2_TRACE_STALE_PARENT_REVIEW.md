# CODEX REAL-10 Phase 2 Trace / Stale-Parent Review

Reviewer: independent clean-context Codex audit worker
Date: 2026-05-15

Scope: REAL-10 Phase 2 Atom 1 / Atom 2 only.

Risk class: Class 4 package, because `TRACE_MATRIX_v3_2026-04-27.md` is a Trust Root-pinned traceability surface and Atom 2 exercises stale-parent Sequencer/QState behavior.

Touched FC / invariants:

- FC1 runtime/admission parent-root behavior: stale `VerifyTx.parent_state_root` must reject; refreshed `q_snapshot().state_root_t` must accept.
- FC3 traceability/materialized evidence: R-022 skip cleanup must register public surfaces in TRACE_MATRIX §J without weakening policy.
- TRACE_MATRIX R-022 public surface backlink discipline.
- Live REAL-6B AttemptPrediction remains gated and is not enabled by REAL-8/REAL-10 runner.

## Findings

No blocking findings.

I found no production defects in the Atom 1/2 review scope. I also found no remaining test-scaffold blocker for the stale-parent gap: the old source-grep check remains only as a secondary guard, while `real8_task_outcome_arm_refreshes_verify_parent_behaviorally` directly drives a Sequencer fixture through accepted state mutation, stale-parent rejection, and refreshed-parent acceptance.

## Requirement Checks

1. R-022 skip cleanup is not treated as policy relaxation.

   PASS. The cleanup report states this is a "one-time bulk-ship exception closure, not policy relaxation" and records "R-022 not treated as waiver" in SG-10.1.

   Evidence:
   - `handover/alignment/TRACE_MATRIX_BACKLINK_CLEANUP_REAL5S_REAL9.md:7`
   - `handover/alignment/TRACE_MATRIX_BACKLINK_CLEANUP_REAL5S_REAL9.md:10`
   - `handover/alignment/TRACE_MATRIX_BACKLINK_CLEANUP_REAL5S_REAL9.md:16`
   - `handover/alignment/TRACE_MATRIX_BACKLINK_CLEANUP_REAL5S_REAL9.md:18`

2. Every REAL-5S -> REAL-9 skipped public surface from `rules/enforcement.log` has TRACE_MATRIX §J registration coverage.

   PASS. `TRACE_MATRIX_v3_2026-04-27.md` defines §J as the R-022 fallback target, requires a justification ref, and registers the REAL-5S -> REAL-9 skipped surfaces in §J.2. The regression test parses `rules/enforcement.log`, filters `R-022-SKIP` rows for `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md`, normalizes symbols, excludes only `trace_removal`, and asserts there are no missing §J pairs.

   Evidence:
   - `handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:550`
   - `handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:552`
   - `handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:570`
   - `handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:574`
   - `handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:723`
   - `tests/constitution_real10_trace_cleanup.rs:28`
   - `tests/constitution_real10_trace_cleanup.rs:47`
   - `tests/constitution_real10_trace_cleanup.rs:82`
   - `tests/constitution_real10_trace_cleanup.rs:90`

3. `trace_removal` entries are legacy cleanup/audit trail, not open public API surfaces.

   PASS. `trace_removal` is excluded from §J.2 pair generation, and the matrix places the two entries in §J.3 as closed audit-trail rows only.

   Evidence:
   - `tests/constitution_real10_trace_cleanup.rs:47`
   - `tests/constitution_real10_trace_cleanup.rs:124`
   - `tests/constitution_real10_trace_cleanup.rs:140`
   - `handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:724`
   - `handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:728`
   - `handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:729`

4. `pub_const_fn` parser ambiguity is explicit; normalized checker-compatible symbols are used.

   PASS. The cleanup report documents `pub_const_fn -> fn`; §J.2 uses `fn` for the affected rows and records parser ambiguity plus concrete function names/lines. The regression test rejects raw `pub_*` prefixes in registered §J.2 symbols and asserts the ambiguity text exists.

   Evidence:
   - `handover/alignment/TRACE_MATRIX_BACKLINK_CLEANUP_REAL5S_REAL9.md:45`
   - `handover/alignment/TRACE_MATRIX_BACKLINK_CLEANUP_REAL5S_REAL9.md:56`
   - `handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:619`
   - `handover/alignment/TRACE_MATRIX_v3_2026-04-27.md:688`
   - `tests/constitution_real10_trace_cleanup.rs:152`
   - `tests/constitution_real10_trace_cleanup.rs:174`

5. Stale-parent gap is covered by a direct behavioral test.

   PASS. The behavioral test accepts `TaskOpen`, `EscrowLock`, and `WorkTx`, captures `post_work_root`, applies a legitimate `CompleteSetMint` market-side state mutation, confirms `state_root_t` changes, asserts stale `VerifyTx(post_work_root)` rejects with `TransitionError::StaleParent`, confirms rejection does not advance state, then asserts a `VerifyTx` built from refreshed `q_snapshot().state_root_t` accepts.

   Evidence:
   - `tests/constitution_real8_market_ab_benchmark.rs:323`
   - `tests/constitution_real8_market_ab_benchmark.rs:328`
   - `tests/constitution_real8_market_ab_benchmark.rs:343`
   - `tests/constitution_real8_market_ab_benchmark.rs:348`
   - `tests/constitution_real8_market_ab_benchmark.rs:354`
   - `tests/constitution_real8_market_ab_benchmark.rs:358`
   - `tests/constitution_real8_market_ab_benchmark.rs:362`
   - `tests/constitution_real8_market_ab_benchmark.rs:367`

6. Live REAL-6B AttemptPrediction remains gated and is not enabled by REAL-8/REAL-10 runner.

   PASS. The runner fails closed when `TURINGOS_REAL6B_LIVE_ATTEMPT_PREDICTION=1`; Arm D only enables the scripted fixture. The test asserts the live REAL-6B boundary string and Arm D scripted fixture label.

   Evidence:
   - `scripts/run_real8_market_ab_benchmark.sh:88`
   - `scripts/run_real8_market_ab_benchmark.sh:89`
   - `scripts/run_real8_market_ab_benchmark.sh:358`
   - `scripts/run_real8_market_ab_benchmark.sh:361`
   - `tests/constitution_real8_market_ab_benchmark.rs:215`
   - `tests/constitution_real8_market_ab_benchmark.rs:220`

## Production Defects vs Test-Scaffold Gaps

Production defects: none found in the reviewed Atom 1/2 surfaces.

Test-scaffold gaps: none blocking. The prior stale-parent scaffold gap is now closed by direct Sequencer/QState behavior. The source-grep test at `tests/constitution_real8_market_ab_benchmark.rs:304` remains secondary and does not carry the main proof.

## Verification

Fresh commands run by this reviewer:

```text
bash -n scripts/run_real8_market_ab_benchmark.sh
exit 0

cargo test --test constitution_real10_trace_cleanup
6 passed; 0 failed

cargo test --test constitution_real8_market_ab_benchmark
8 passed; 0 failed

cargo test --test constitution_real10_emergence_metrics
4 passed; 0 failed

cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo
1 passed; 0 failed

git diff --check
exit 0
```

The Trust Root check is included because the TRACE_MATRIX file is pinned in `genesis_payload.toml`, and the current diff updates that hash. No production Rust restricted source file is modified in the reviewed Atom 1/2 diff; modified tracked files are `genesis_payload.toml`, `handover/alignment/TRACE_MATRIX_v3_2026-04-27.md`, `scripts/run_real8_market_ab_benchmark.sh`, and `tests/constitution_real8_market_ab_benchmark.rs`.

## Verdict

PROCEED
