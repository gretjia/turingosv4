# Gemini PPUT-CCL Phase B B7-extra ROUND-3 RE-AUDIT (post-VETO-fixes)
**Date**: 2026-04-25
**Predecessor**: GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md (verdict VETO)
**Fixes commits**: 15b87fb (Phase A+B+C) + 438a648 (code-simplifier) + c061450 (constitution) + df77b0a (audit packet record)
**Test baseline**: 187/187 PASS + 20 ignored
**Smoke**: control SOLVED tx=15, treatment UNSOLVED tx=50 with synthetic_short_circuit=true
**Negative test**: tamper Trust Root → evaluator panic at boot ✓
**Elapsed**: 41.3s
**Prompt size**: 88,009 chars

---

**VERDICT**: CHALLENGE
**Conviction**: HIGH
**Recommendation**: FIX-THEN-PROCEED again

The Round-2 fix (`1df1f62`) is a substantial improvement and correctly resolves the critical VETO-level defect identified by Codex in the previous round. The new crash-discrimination logic, which aborts the batch on any non-timeout, non-zero exit, prevents the silent absorption of panics (like `TRUST_ROOT_TAMPERED`) into the calibration dataset. This restores the integrity of the Trust Root mechanism. The addition of a pre-flight boot check is a commendable "fail fast" enhancement.

However, a new, lower-severity data-loss vulnerability was identified during this re-audit. While not a P0 architectural flaw that renders the batch unsafe, it is a data integrity defect that could bias the `p_0` measurement and should be fixed before launching the full 576-run batch.

---

### Detailed Findings

#### Primary VETO Issue (Codex) — RESOLVED

The core concern from the Round-2 VETO is fully addressed.
- **Evidence**: `handover/preregistration/scripts/run_p0_calibration.sh` lines 309-333.
- **Analysis**: The script's main loop now correctly partitions evaluator outcomes:
    1.  `EXIT=0` with `PPUT_RESULT` → Success path.
    2.  `EXIT=124` (timeout) → Synthetic UNSOLVED row emitted. This correctly implements the fix for my original Q7.b finding.
    3.  `EXIT` is non-zero and not 124 → `CRASH` path, which aborts the entire batch with a specific exit code (3 for `TRUST_ROOT_TAMPERED`, 4 for others).
- **Conclusion**: This logic prevents panics from being silently converted into "valid" `UNSOLVED` data points, which was the critical flaw. The `TRUST_ROOT_TAMPERED` pre-flight check (lines 163-173) further hardens this by catching the most severe integrity failures before any API spend.

#### Original VETO Issues (Gemini Q2.b, Q2.e) — REMAIN RESOLVED

My original VETO items are still fixed and have been integrated into the test suite.
- **Evidence**: `genesis_payload.toml` now includes `src/main.rs` and `Cargo.lock`. The test `experiments/minif2f_v4/tests/trust_root_immutability.rs::test_trust_root_manifest_includes_b2_b4_files` (lines 100-120) explicitly asserts the presence of these and 18 other required files. The negative tamper test passed, confirming the mechanism is active.

---

### New CHALLENGE Finding

A silent data-loss path exists in the runner script, which could bias the `p_0` estimate.

- **Finding ID**: RQ2.b
- **Location**: `handover/preregistration/scripts/run_p0_calibration.sh`, lines 299-333.
- **Vulnerability**: If the `evaluator` process exits successfully (`EXIT=0`) but fails to print a line starting with `PPUT_RESULT:` to stdout, the run is silently dropped.
- **Trace**:
    1. The `evaluator` runs, `EXIT` is set to `0`.
    2. The `PPUT_JSON=$(echo "$OUTPUT" | grep ...)` command finds no match, so `$PPUT_JSON` is an empty string.
    3. The condition `if [ -n "$PPUT_JSON" ] && [ "$EXIT" -eq 0 ]` evaluates to `false` because `-n ""` is false.
    4. The next condition `elif [ "$EXIT" -eq 124 ]` is `false`.
    5. The final `else` block is only triggered for non-zero `EXIT` codes, so it is skipped.
    6. No row is written to the `.jsonl` file for this (problem, seed) pair, and the script continues to the next run without any warning or error.
- **Impact**: This constitutes a silent `MEASUREMENT_ERROR`. If this occurs, `compute_p0.py` will fail its strict-completeness check, aborting the analysis after the entire batch has run and spent the budget. Worse, if the completeness check were ever relaxed, it would bias the `p_0` estimate by selectively dropping runs (likely successful ones, as a crash would be caught).
- **Recommendation**: The `if/elif/else` structure should be made exhaustive for the `EXIT=0` case. An `else` branch should be added to the main `if` to catch the `[ "$EXIT" -eq 0 ] && [ -z "$PPUT_JSON" ]` condition, print a loud error, and abort the batch.

---

### Other Questions (PASS)

- **RQ1 (Manifest Completeness)**: PASS. The 20-entry manifest is comprehensive for the stated threat model. The exclusion of `boot.rs` is a documented and reasonable trade-off (`TRACE_MATRIX_v1 § 4`). The `verify_trust_root` call in `evaluator.rs` happens correctly before any other action.
- **RQ3 (Strict-Completeness)**: PASS. `compute_p0.py` correctly uses hardcoded PREREG constants and does not expose them as arguments, preventing bypass. The runner script correctly distinguishes the data integrity exit code (1) from the ceiling violation exit code (2).
- **RQ4 (Equivalence Claim)**: PASS. The header in `rollback_sim.rs` is accurate for the system's current feature set. `compute_p0.py` correctly honors the narrow equivalence by only inspecting the `progress` field.
- **RQ5 (Code Simplifier)**: PASS. The refactorings are standard and do not introduce new risks. Test coverage was maintained.
- **RQ6 (Constitutional Alignment)**: PASS. The fix commits were made in accordance with the newly amended constitution (Art. V.1.2).

**Final Recommendation**: The team has successfully addressed the critical VETO. The remaining issue is a standard bug, not an architectural flaw. A small patch to `run_p0_calibration.sh` to make the result-parsing logic exhaustive will resolve the CHALLENGE. After that fix is applied and verified, the batch can proceed with high confidence.