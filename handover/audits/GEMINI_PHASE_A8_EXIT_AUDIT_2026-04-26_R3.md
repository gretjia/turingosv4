# Gemini Phase A → B Exit Audit (PPUT-CCL arc) — round 2
**Date**: 2026-04-26 (post A8e fixes)
**Round**: R3
**Commits**: 2e7f75a / d8950ee / 2a65339 / e94e1b9 / 62c4e14 / 6be6eb4 / 180a300 / 7f4bc0c / a5c78e4 / 30f2a14 / 89994c7 / 90953d6 / 60292dc / 5a56ff6
**Test baseline**: 264 PASS + 29 ignored + 0 failed (Rust); 15/15 PASS (Python proxy tests)
**Trust Root**: 33-entry manifest verifies clean
**Elapsed**: 41.5s
**Prompt size**: 276,141 chars

---

As an independent, skeptical adversarial reviewer, I have completed the Phase A → B Exit Audit for the PPUT-CCL arc. The audit focused on the cumulative state of atoms A0 through A7, including the round-3 fixes (commit `A8e2`). The conservative merge rule (VETO > CHALLENGE > PASS) and the precedent for rigor from the A0 audit have been applied.

The team has demonstrated diligence in addressing the round-1 VETO and round-2 CHALLENGE findings. The G1 fix, in particular, provides a robust, recurring conformance gate for the Python proxy's round-robin logic, satisfying the core requirement of the original VETO. The G2 and G3 fixes correctly address the documentary and procedural gaps identified in round 2.

However, a procedural gap remains in the audit packet itself, indicating a lack of final-pass rigor on documentation that serves as a contract for these reviews. While the code appears sound and ready for Phase B, the process must be held to the same standard.

---

### Finding 1 (CHALLENGE)

**Stale Audit Question Indicates Procedural Gap.** The audit packet in § 6 still presents question Q4.d as an open query for auditors, yet the issue it describes was explicitly closed by fix F1 in round 1.

-   **§/file:line**: `§ 6. Specific questions for auditors / Q4 — FC tracing coverage (A6)`
-   **Atom**: A8 (the audit packet itself)
-   **Details**: Question Q4.d asks:
    > `run_corr_id` format = `condition_problem_id_unix-ms`. `make_pput`'s `run_id` independently re-computes this with its own ts. The two will differ by milliseconds. Is the join semantics for Phase D consumers documented anywhere?

    This describes the millisecond-drift problem that was a key finding in the round-1 audit (Codex#2, Gemini Q4). The packet's own summary of round-1 fixes correctly states that fix **F1** closed this:
    > **F1** unified `run_id` (new `run_id.rs` module + threaded into `make_pput`); oneshot stops using `oneshot_{problem_file}` placeholder. Closes Codex#2 + Gemini Q4.

    My verification of the code confirms that `experiments/minif2f_v4/src/bin/evaluator.rs` now mints a single `run_id` at the start of `run_swarm`/`run_oneshot` and passes it to both `fc_trace::emit_event` and `make_pput`. The problem is solved. The persistence of Q4.d in the round-3 packet is a documentation error that demonstrates a gap in the pre-flight checklist for the audit packet itself. An audit packet is a constitutional artifact for a gate decision; its internal consistency is mandatory.

### Finding 2 (Observation, Non-blocking)

**Function Signature Complexity Risk.** The `make_pput` function signature has grown to 21 arguments, creating a maintainability and correctness risk.

-   **§/file:line**: `experiments/minif2f_v4/src/bin/evaluator.rs:1338` (`fn make_pput(...)`)
-   **Atom**: A4/A5/A8 (cumulative additions)
-   **Details**: Atoms A4, A5, and A8e (fix F1) have added multiple non-Optional parameters to `make_pput`. While all call sites have been correctly updated, the long, position-dependent argument list is a code smell. It increases the cognitive load for developers and raises the risk of future errors where arguments could be swapped, especially those of the same type (e.g., `distinct_proposals` and `total_proposals`). This is not a defect that blocks Phase B, but it should be addressed via refactoring (e.g., a builder pattern or a dedicated `PputResultBuilder` struct) before Phase C to improve long-term system health.

### Verification of Round-3 Fixes

-   **(G1) Python Conformance Test Integration**: **VERIFIED**. The new Rust test `experiments/minif2f_v4/tests/llm_proxy_python_conformance.rs` correctly shells out to the Python test suite. It is present in the Trust Root manifest (`genesis_payload.toml`) and the immutability test (`trust_root_immutability.rs:130`). This successfully elevates the Python test from a manual check to a recurring, automated conformance gate, satisfying the round-1 VETO.
-   **(G2) PREREG Amendment Wording**: **VERIFIED**. A full-text search of `handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md` confirms that all references to the `p_0=0.10` substitution are now consistent with it being the "least-strict admissible ceiling," not the "strictest." The correction note in § 2 and the re-worded audit requirement in § 8 are both clear and correct.
-   **(G3) Packet and TRACE_MATRIX Staleness**: **VERIFIED**. The test and Trust Root counts in the packet (§ 2, § 4, § 6 Q7) now correctly reflect the final A8e2 state (265 tests, 34 TR entries). The TRACE_MATRIX milestone progression (`TRACE_MATRIX_v2_2026-04-25.md:§ 6`) is accurate. The stale risk #5 has been removed from § 5.

---

-   **VERDICT**: **CHALLENGE**
-   **Conviction**: high
-   **Recommendation**: **FIX-THEN-PROCEED**

The substantive code changes for Phase A are sound and the critical VETO-level finding from round 1 has been robustly addressed. The system is ready for Phase B kernel instrumentation. However, the procedural error in the audit packet itself (Finding 1) must be corrected before the gate is officially passed. The fix is trivial—remove the stale question Q4.d from the packet—but the principle of documentary rigor at a constitutional gate is not.