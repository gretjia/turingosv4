# Gemini PPUT-CCL Phase B B7-extra RE-AUDIT (post-VETO-fixes)
**Date**: 2026-04-25
**Predecessor**: GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md (verdict VETO)
**Fixes commits**: 15b87fb (Phase A+B+C) + 438a648 (code-simplifier) + c061450 (constitution) + df77b0a (audit packet record)
**Test baseline**: 187/187 PASS + 20 ignored
**Smoke**: control SOLVED tx=15, treatment UNSOLVED tx=50 with synthetic_short_circuit=true
**Negative test**: tamper Trust Root → evaluator panic at boot ✓
**Elapsed**: 59.2s
**Prompt size**: 83,325 chars

---

An independent re-audit of the fixes follows.

---

### RQ1 — Q2.b/e fix completeness (the two VETO items)

-   **(RQ1.a) Is the manifest now COMPLETE for the threat model you previously framed (passive tamper without recompile)?**
    -   The manifest has expanded from 16 to 20 entries. The additions are `src/main.rs`, `Cargo.lock`, `run_p0_calibration.sh`, and `compute_p0.py`.
    -   These additions directly address the VETO findings (call site, supply chain) and a key CHALLENGE (runner/estimator as qualified tools). Hashing the runner script (`run_p0_calibration.sh`) is a significant security improvement, as it freezes the entire execution protocol, including timeout values, environment variable setup, and the logic for handling crashes.
    -   The manifest appears complete for the stated threat model. It covers the source code for the core logic, the data inputs (splits, prereg), the constitutional ground truth, and now the execution/analysis tooling. The only notable exclusion is `src/boot.rs` itself, which is explicitly justified in `TRACE_MATRIX_v1` § 4 as a conscious trade-off against an attacker who can recompile, a threat outside the current model. This is a reasonable and documented decision.
    -   **Finding**: The manifest is now sufficiently complete for the specified threat model.

-   **(RQ1.b) `genesis_payload.toml` itself is not self-hashed. Is this acceptable?**
    -   Yes, this is acceptable. The chicken-and-egg problem is real. The defense is now multi-layered:
        1.  The code that *parses* the manifest (`src/boot.rs`) is in the Trust Root via the hash of `evaluator.rs` which contains it.
        2.  The test suite (`trust_root_immutability.rs:91`) asserts that the manifest contains a specific, hard-coded list of 20 required files. This provides a strong CI-time guarantee against accidental or malicious removal of an entry from the committed `genesis_payload.toml`.
        3.  The call site (`evaluator.rs:155`) is hashed.
    -   This combination of runtime verification of the *verifier's logic* and CI-time verification of the *manifest's completeness* is a robust mitigation for the lack of self-hashing.

-   **(RQ1.c) Does evaluator.rs's call to `verify_trust_root` happen BEFORE any other state-modifying action?**
    -   Walking the `main` function in `evaluator.rs`:
        1.  `env_logger::init();` (line 148)
        2.  `verify_trust_root(&repo_root)` block (lines 155-160)
        3.  `env::set_var("CLASSIFIER_VERSION", ...)` (line 164)
        4.  The main `run_swarm` logic, including all LLM calls and file I/O.
    -   The verification happens *after* logger initialization but *before* any other meaningful action, especially before setting environment variables that affect output or calling the core PPUT logic. While initializing the logger is technically a state-modifying action, it's a low-risk one. The critical, high-risk operations are correctly guarded. This order is acceptable.

**Conclusion for RQ1**: The VETO items are fully resolved. The Trust Root mechanism is now robustly implemented at the correct entry point (`evaluator.rs`) with a sufficiently complete manifest.

### RQ2 — Q7.b fix (timeout sampling bias)

-   **(RQ2.a) Confirm the synthetic row's schema is correct and `compute_p0.py` reads it as UNSOLVED.**
    -   The `emit_synthetic_unsolved` function in `run_p0_calibration.sh` correctly sets `"solved": False`, `"verified": False`, and `"progress": 0`.
    -   The `solved()` function in `compute_p0.py` (line 45) reads the `progress` field. Since the synthetic row has `progress: 0`, `solved()` will correctly return `False`.
    -   The fix is correct and effective. The schema includes the `synthetic_timeout_or_crash: true` flag, which is crucial for downstream analysis to distinguish these cases from natural UNSOLVED runs.

-   **(RQ2.b) Are there OTHER failure modes that still cause data loss?**
    -   The runner script (`run_p0_calibration.sh`) now uses `set -euo pipefail`. This is critical.
    -   The core execution block (lines 240-279) is structured as `OUTPUT=$(...) || EXIT=$?`. This correctly captures the exit code from the `timeout` command without the script exiting immediately.
    -   The subsequent `if [ -n "$PPUT_JSON" ] && [ "$EXIT" -eq 0 ]` block handles the success case.
    -   The `else` block handles *all other cases*: non-zero exit code (crash, timeout), or empty `PPUT_JSON` (e.g., the `grep` or `sed` failed to find the result line). In all these failure scenarios, `emit_synthetic_unsolved` is called.
    -   The logic appears robust. The combination of `set -euo pipefail` and the explicit `if/else` on the captured exit code and output prevents silent data loss.

**Conclusion for RQ2**: The VETO-equivalent finding of sampling bias from timeouts/crashes is resolved. The runner's error handling is now robust and correctly transforms measurement failures into UNSOLVED data points, preserving the integrity of the `p_0` estimator.

### RQ3 — Q3.d strict-completeness fix

-   **(RQ3.a) Is exit 1 (data integrity failure) handled distinctly from exit 2 (ceiling violation)?**
    -   Yes. `compute_p0.py` exits with 1 on data integrity errors (e.g., missing pairs, seed mismatch) via `sys.exit("ERROR...")`. It exits with 2 specifically for a ceiling violation.
    -   `run_p0_calibration.sh` (lines 318-333) correctly distinguishes these:
        -   `P0_EXIT -eq 0` is PASS.
        -   `P0_EXIT -eq 2` is the explicit ceiling ABORT.
        -   The final `else` block catches all other non-zero codes (including 1) and reports a generic error, propagating the original exit code.
    -   This logic is correct and provides the necessary distinction for the operator.

-   **(RQ3.b) Could a future caller pass wrong values and bypass completeness checks?**
    -   The `compute` function in `compute_p0.py` takes `expected_n_problems` and `expected_seeds` as arguments with defaults pointing to the PREREG constants.
    -   However, the `main` function (line 160) calls `compute(control_rows, treatment_rows)` without overriding these defaults.
    -   Since `compute_p0.py` is now in the Trust Root manifest, its hash is fixed. Any execution of the calibration batch is guaranteed to run this specific version of the script, where the PREREG constants are effectively hard-coded into the `main` entry point. This prevents bypass.

**Conclusion for RQ3**: The strict-completeness fix is robust and correctly implemented. It prevents biased estimation from incomplete data and cannot be easily bypassed.

### RQ4 — Q1.a equivalence claim

-   **(RQ4.a) Is the new header text in `rollback_sim.rs` accurate?**
    -   The header now explicitly states "Equivalence is narrow" and lists non-equivalent dimensions: cost, wall-clock, WAL, bus predicate traversal, and `tx_count`.
    -   This is accurate and transparent. The smoke test results confirm this non-equivalence (e.g., `tx_count` is 15 for control, 50 for treatment). The list covers all major implemented metrics. It correctly sets expectations for any other tool that might consume this data.

-   **(RQ4.b) Does `compute_p0.py` honor this narrow equivalence?**
    -   Yes. As verified in RQ2.a, `compute_p0.py`'s `solved()` function relies only on the `progress` field. It does not inspect cost, time, or any other non-equivalent field. The code honors the contract described in the `rollback_sim.rs` header.

**Conclusion for RQ4**: The CHALLENGE regarding the over-broad equivalence claim is resolved. The documentation is now precise, and the consuming code adheres to the specified narrow interface.

### RQ5 — New surfaces introduced by code-simplifier

-   **(RQ5.a) `Sha256::digest` equivalence.**
    -   The use of the one-shot `Sha256::digest(&bytes)` function is idiomatic and correct. It is semantically equivalent to the multi-step `new/update/finalize` process for a single, complete data buffer. This is a safe simplification.

-   **(RQ5.b) Test helper consolidation in `boot.rs`.**
    -   The `write_single_entry_repo` helper in `boot.rs` (line 211) consolidates file I/O setup. The core logic of the tests—providing either a matching or a mismatching hash—remains within the respective test functions (`verify_trust_root_detects_tamper_in_tempdir` and `verify_trust_root_passes_when_hash_matches_in_tempdir`). No test coverage has been dropped. This is a safe and clean refactoring.

**Conclusion for RQ5**: The code-simplifier changes introduced no new defects.

### RQ6 — Constitutional alignment (post-amendment)

-   **(RQ6.a) Did any fix modify a file that should have required Veto-AI proposal review?**
    -   The constitutional amendments in `c061450` clarify that ArchitectAI has commit authority on all non-`constitution.md` files, pending Veto-AI review for constitutional violations. Human sudo is reserved for `constitution.md`.
    -   The 7 modified files in the fix commit (`15b87fb`) are all non-`constitution.md` files. They implement runtime checks, measurement protocols, and tooling. None of them alter the fundamental principles or articles of the constitution; rather, they are implementations under the existing constitutional framework.
    -   The process followed (ArchitectAI committing these changes) is fully aligned with the newly amended Art. V.1.2.

**Conclusion for RQ6**: The fix process was fully aligned with the amended constitution.

---

### Final Verdict

The two VETO findings (Q2.b, Q2.e) have been comprehensively resolved. The Trust Root manifest is now complete for its threat model, and the verification is correctly performed at the production entry point.

The VETO-equivalent finding on sampling bias (Q7.b) has been resolved with a robust error-handling mechanism in the runner script that prevents data loss.

All prior CHALLENGE items have been addressed through code changes or explicit documentation updates.

No new P0 defects were introduced by the fixes or the associated code refactoring. The entire process was conducted in alignment with the updated constitutional procedures.

-   **VERDICT**: **PASS**
-   **Conviction**: **high**
-   **Recommendation**: **PROCEED with 576-run batch**