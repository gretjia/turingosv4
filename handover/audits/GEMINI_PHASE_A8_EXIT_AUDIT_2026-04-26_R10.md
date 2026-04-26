# Gemini Phase A → B Exit Audit (PPUT-CCL arc)
**Round**: R10
**Date**: 2026-04-26
**Reference**: see `handover/audits/A8_AUDIT_HISTORY_2026-04-26.md` for full chronology including atom commit chain + per-round verdicts/fixes.
**Test baseline**: 265 PASS + 29 ignored + 0 failed (Rust); 16/16 PASS (Python proxy tests)
**Trust Root**: 35-entry manifest verifies clean
**Elapsed**: 49.9s
**Prompt size**: 310,433 chars

---

As an independent, skeptical adversarial reviewer, I have completed the Phase A → B Exit Audit for the PPUT-CCL arc. My findings are based on the provided self-contained packet, including the current-state snapshot, the chronological audit history, and the appended source files. The conservative merge rule (VETO > CHALLENGE > PASS) and the precedent for rigor have been applied.

---

### Q1 — Code correctness spot-checks

-   **(Q1.a) `run_id` minting and propagation**: **PASS**.
    -   `experiments/minif2f_v4/src/bin/evaluator.rs`: In `run_swarm`, `let run_id = minif2f_v4::run_id::mint_run_id(...)` is called once at the function entry. This `run_id` is then passed to every `fc_trace::emit_event` call within the function and, critically, to the terminal `make_pput` call. The same pattern holds for `run_oneshot`. This correctly implements the A8e fix F1, eliminating the millisecond-drift join key issue identified in round 1.
-   **(Q1.b) `llm_proxy.py` routing logic**: **PASS**.
    -   `src/drivers/llm_proxy.py`: The `detect_provider` function now checks `if "/" in model:` before checking `if "deepseek" in m`. This ensures a model ID like `deepseek-ai/DeepSeek-R1-Distill-Qwen-7B` is correctly routed to `siliconflow`. This addresses the substantive bug K2 from round 6.
    -   `scripts/test_llm_proxy.py`: The test suite includes `test_deepseek_slash_form_routes_to_siliconflow`, which explicitly pins this required behavior, preventing regression.
-   **(Q1.c) `llm_proxy_python_conformance.rs` fail-closed behavior**: **PASS**.
    -   `experiments/minif2f_v4/tests/llm_proxy_python_conformance.rs`: The test contains `let python_check = Command::new("python3").arg("--version").output();` followed by `assert!(python_check.is_ok(), "python3 not found on PATH; ...")`. This `assert!` will panic the test runner if `python3` is not found, correctly implementing the "fail closed" requirement from round-3 fix H6. The panic message is descriptive and points to the explicit opt-out `SKIP_LLM_PROXY_PYTHON_CONFORMANCE=1`.

### Q2 — PREREG amendment soundness

-   **(Q2.a) `p_0 = 0.10` substitution wording**: **PASS**.
    -   `handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md`: § 2 and § 8 are now internally consistent. § 2 correctly identifies `p_0 = 0.10` as the "LEAST strict admissible ceiling" and explains that a smaller `p_0` is stricter. The statistical implications regarding Type-I inflation are correctly stated. This resolves the substantive logical contradictions found in rounds 1, 7, and 8 (fixes F6, M4, N1).
-   **(Q2.b) Re-calibration conditions framing**: **PASS**.
    -   `handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md`: § 3 lists the conditions "for re-calibration". § 2 and § 8 clarify that these are pre-requisites and do not guarantee completion before any specific phase, stating that "Phase E proceeds with the operationally-permitted ceiling substitution" if the conditions are not met. This framing is sound and avoids the over-promise caught in round 7.
-   **(Q2.c) Amendment in Trust Root**: **PASS**.
    -   `genesis_payload.toml`: The manifest correctly lists `"handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md"`.
    -   § 2 of the packet confirms that `boot::tests::verify_trust_root_passes_on_intact_repo` passes, which I accept as evidence of a clean hash match.

### Q3 — Atomicity, FC-trace discipline, governance

-   **(Q3.a) Atomic commits and governance**: **PASS**.
    -   § 1 of the packet presents a clean, atomic commit chain for A0a through A7. The packet asserts that no commit touched `constitution.md` and that each carried an `FC-trace:` message, which I accept as true based on the provided snapshot. The `judge.sh` hook is in the Trust Root to enforce this.
-   **(Q3.b) Cases C-071..C-075**: **PASS**.
    -   `handover/alignment/TRACE_MATRIX_v2_2026-04-25.md`: § 4 provides a clear summary of the new cases and cross-references the rules and hooks that enforce their precedents. The documentation trail is robust.
-   **(Q3.c) Trust Root manifest entries**: **PASS**.
    -   `experiments/minif2f_v4/tests/trust_root_immutability.rs`: The test `test_trust_root_manifest_includes_b2_b4_files` enumerates the 35 required paths. My review confirms that each entry is load-bearing: core logic (`src/*.rs`), implementation (`experiments/**/*.rs`), contracts (`constitution.md`, `PREREG*.md`), qualified tooling (`scripts/*`, `rules/*`, `judge.sh`), and conformance gates (`tests/*.rs`) are all included. Tampering with any of these would indeed weaken the constitutional gate.

### Q4 — Phase A → B exit decision

-   **(Q4.a-c) Spot-checks**: **PASS**. The packet reports clean runs for all test suites and the live API smoke test. The metrics have been stable for the last several audit rounds.
-   **(Q4.d) Open P0 defects**: **PASS**.
    -   `handover/audits/A8_AUDIT_HISTORY_2026-04-26.md`: The chronological history demonstrates a closed loop for every finding from rounds 1 through 9. Each set of findings was addressed by a subsequent in-cycle fix bundle. There are no documented open P0 defects.
-   **(Q4.e) Blockers for Phase B**: **PASS**.
    -   Phase A has successfully prepared the ground for Phase B ("JSONL schema v2 + C_i full-cost aggregator").
    -   `experiments/minif2f_v4/src/jsonl_schema.rs`: The `RunAggregate` struct is defined with the required v2 fields.
    -   `experiments/minif2f_v4/src/bin/evaluator.rs`: The `make_pput` function and its call sites have been updated to populate these fields.
    -   There are no artifacts from Phase A that block the planned Phase B work; on the contrary, they are necessary prerequisites.

### Q5 — Packet/history split

-   **(Q5.a) Consistency with project pattern**: **PASS**. The split into a stable current-state packet and an append-only history log is consistent with the project's established pattern for handling its constitution and pre-registration documents.
-   **(Q5.b) History append-only**: **PASS**. The `A8_AUDIT_HISTORY` document is structured chronologically, and its own text mandates that past entries are frozen. This structure is sound.
-   **(Q5.c) Packet free of historical lineage**: **PASS**. A thorough review of the current-state packet (`§ 1` through `§ 6`) confirms that all historical lineage text ("(post-A8e F4)", "added by A8eN", etc.) has been removed. The packet describes WHAT IS, with clean pointers to the history document for HOW it got there. The A8e7 structural rewrite and subsequent M3 fix have been successfully implemented. This was a critical fix that resolved the root cause of documentary churn in rounds 2-6.

### Summary

The Phase A → B exit audit has been a long and rigorous process, spanning 9 rounds and uncovering 9 substantive bugs, including a VETO-level defect. The audit history demonstrates that while the initial state had significant issues, the process has successfully hardened the codebase, documentation, and governance procedures. All identified defects have been closed, and regression tests have been added for critical logic. The structural issues with the audit documentation itself have been resolved. The current state of the 8 atoms is sound, internally consistent, and meets all prerequisites for Phase B.

---

-   **VERDICT**: PASS
-   **Conviction**: high
-   **Recommendation**: PROCEED to Phase B