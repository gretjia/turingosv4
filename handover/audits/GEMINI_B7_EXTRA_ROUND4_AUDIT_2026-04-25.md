# Gemini PPUT-CCL Phase B B7-extra ROUND-4 RE-AUDIT (post-VETO-fixes)
**Date**: 2026-04-25
**Predecessor**: GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md (verdict VETO)
**Fixes commits**: 15b87fb (Phase A+B+C) + 438a648 (code-simplifier) + c061450 (constitution) + df77b0a (audit packet record)
**Test baseline**: 187/187 PASS + 20 ignored
**Smoke**: control SOLVED tx=15, treatment UNSOLVED tx=50 with synthetic_short_circuit=true
**Negative test**: tamper Trust Root → evaluator panic at boot ✓
**Elapsed**: 44.5s
**Prompt size**: 90,818 chars

---

**TO**: PPUT-CCL Oversight Committee
**FROM**: Gemini Audit (Independent Reviewer)
**DATE**: 2026-04-26
**SUBJECT**: Round-4 Re-Audit of Commit `d0d474e` (Phase B B7-extra)

This document constitutes the fourth and final audit pass on the Phase B B7-extra calibration tooling. The mandate was to scrutinize commit `d0d474e`, verifying that it resolved all Round-3 CHALLENGE findings without introducing new defects, and to confirm that the original VETO items from Round 1 remain resolved.

---

### RQ1 — Q2.b/e fix completeness (the two VETO items)

The original VETO was issued because `src/main.rs` and `Cargo.lock` were missing from the Trust Root manifest, creating a silent bypass vulnerability and a supply-chain risk, respectively.

-   **(RQ1.a) Manifest Completeness:** The manifest in `genesis_payload.toml` now contains 20 entries. This is verified by the conformance test `experiments/minif2f_v4/tests/trust_root_immutability.rs::test_trust_root_manifest_includes_b2_b4_files`, which asserts the presence of all 20 required paths. The set of files appears complete for the stated threat model (passive tamper without recompile), covering the core logic, accounting layers, calibration scripts, and pre-registration artifacts.
-   **(RQ1.b) `genesis_payload.toml` Self-Hash:** The chicken-and-egg problem of the manifest not hashing itself is acknowledged and acceptable. The defense is sound: the call sites (`src/main.rs`, `evaluator.rs`) that invoke the verification logic are themselves in the manifest. An attacker cannot modify `genesis_payload.toml` to weaken the checks without also modifying a hashed file, which would trigger a `TRUST_ROOT_TAMPERED` failure. This is documented in `TRACE_MATRIX_v1_2026-04-25.md` § 3.
-   **(RQ1.c) Order of Operations:** In `experiments/minif2f_v4/src/bin/evaluator.rs`, the call to `verify_trust_root` occurs at the top of `main`. It is preceded only by `env_logger::init()`. While technically a state-modifying action (global logger state), it poses no risk to the integrity of the run. The critical check happens before any other significant logic, including environment variable setting, argument parsing, or any I/O related to the experiment itself. The process will panic and abort correctly if the trust root is compromised.

**Conclusion**: The original VETO items (Q2.b, Q2.e) are fully and robustly resolved.

### RQ2 — Q7.b fix (timeout sampling bias)

The previous implementation dropped timed-out runs, creating a sampling bias. The fix was to emit a synthetic UNSOLVED row.

-   **(RQ2.a) Synthetic Row Correctness:** The `emit_synthetic_unsolved` function in `run_p0_calibration.sh` correctly generates a JSONL row with `"solved": false`, `"progress": 0`, and `"synthetic_timeout_or_crash": true`. The `solved()` function in `compute_p0.py` correctly interprets this as UNSOLVED by checking `int(row["progress"]) == 1`. The smoke test evidence confirms the treatment run produced a row with `solved: false` and `synthetic_short_circuit: true`, which is handled by the same logic.
-   **(RQ2.b) Other Data Loss Modes:** The runner script is now significantly more robust against silent data loss.
    -   An evaluator crash (`exit != 0` and `exit != 124`) now aborts the entire batch (`run_p0_calibration.sh`, line 406). This correctly prevents crashes from being silently absorbed as UNSOLVED data, a P0 defect caught by Codex in Round 2.
    -   A malformed run (`exit == 0` but no `PPUT_RESULT` line) is now explicitly caught and aborts the batch with `exit 5` (`run_p0_calibration.sh`, line 385). This resolves my Round-3 CHALLENGE.
    -   A crash during the Python-based enrichment step would be caught by `set -e`, aborting the script.
    The logic appears to correctly funnel all outcomes into either a valid data row or a loud, batch-terminating failure.

**Conclusion**: The sampling bias vulnerability is resolved. The runner's error handling is now exhaustive for the known failure modes.

### RQ3 — Q3.d strict-completeness fix

`compute_p0.py` was updated to fail loudly on incomplete or malformed data sets.

-   **(RQ3.a) Exit Code Handling:** `compute_p0.py` exits with status 1 on data integrity failures (e.g., missing problems, duplicate rows). The runner script (`run_p0_calibration.sh`, lines 463-465) correctly distinguishes this from a successful run (exit 0) and a ceiling violation (exit 2), propagating the error code and printing a specific diagnostic.
-   **(RQ3.b) Parameter Bypass:** The `compute()` function in `compute_p0.py` uses hardcoded `PREREG_` constants as default arguments. The `main()` function calls `compute()` without overriding these defaults, and the script's `argparse` does not expose them as command-line flags. This prevents a caller from accidentally or maliciously bypassing the pre-registered constraints.

**Conclusion**: The strict-completeness fix is correctly implemented and its failure modes are properly handled by the runner.

### RQ4 — Q1.a equivalence claim

The header in `rollback_sim.rs` was updated to clarify the narrow scope of equivalence for the calibration treatment.

-   **(RQ4.a) Header Accuracy:** The header in `experiments/minif2f_v4/src/rollback_sim.rs` now correctly states that equivalence is limited to the `(problem_id, seed, solved)` tuple. The list of non-equivalent dimensions (cost, wall-clock, WAL, etc.) is accurate for the current system architecture. Dimensions like RNG state or agent reputation are not applicable as there is no state carried between runs.
-   **(RQ4.b) `compute_p0.py` Honoring Equivalence:** The claim that `compute_p0.py` honors this narrow equivalence is verified. Its `solved()` function depends only on the `progress` field, which directly maps to the `solved` status. It does not inspect cost, `tx_count`, or any other non-equivalent field.

**Conclusion**: The documentation is now accurate and the downstream tooling correctly respects the stated limitations.

### RQ5 — New surfaces introduced by code-simplifier (commit 438a648)

This commit performed minor refactoring.

-   **(RQ5.a) `Sha256::digest`:** This is a standard, idiomatic use of the `sha2` crate's one-shot API. It is semantically equivalent to the previous multi-step process and introduces no new risk.
-   **(RQ5.b) Test Helper Consolidation:** The `write_single_entry_repo` test helper in `src/boot.rs` is used by two separate tests that cover the tamper-detection failure path and the hash-match success path, respectively. The consolidation did not reduce test coverage.

**Conclusion**: The code simplification commit introduced no new defects.

### RQ6 — Constitutional alignment (post-amendment)

The process was reviewed against the newly amended constitution.

-   **(RQ6.a) File Modification Authority:** The fixes in commit `d0d474e` and its predecessors modified `.rs`, `.toml`, `.sh`, `.py`, and `.md` files. None of these was `constitution.md`. Per the amended Art. V.1.2, ArchitectAI has commit authority on these files, subject to Veto-AI review. The audit process itself serves as the Veto-AI function. The actions taken were consistent with the amended constitution.

**Conclusion**: The fix process was constitutionally sound.

---

### Final Verdict

All findings from the previous three rounds of audit have been addressed. The two original VETO items (`Q2.b`, `Q2.e`) are resolved and protected by conformance tests. The critical sampling bias vulnerability (`Q7.b`) is fixed, and the runner script's robustness has been significantly improved with comprehensive pre-flight checks and explicit handling of all known failure modes. The fixes in commit `d0d474e` are correct, complete, and introduce no new P0 defects.

-   **VERDICT**: **PASS**
-   **Conviction**: **High**
-   **Recommendation**: **PROCEED** with the 576-run batch as authorized. The system has demonstrated sufficient maturity and robustness through four rounds of adversarial review.