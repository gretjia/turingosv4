# Gemini Phase A → B Exit Audit (PPUT-CCL arc) — round 2
**Date**: 2026-04-26 (post A8e fixes)
**Round**: R7
**Commits**: 2e7f75a / d8950ee / 2a65339 / e94e1b9 / 62c4e14 / 6be6eb4 / 180a300 / 7f4bc0c / a5c78e4 / 30f2a14 / 89994c7 / 90953d6 / 60292dc / 5a56ff6
**Test baseline**: 264 PASS + 29 ignored + 0 failed (Rust); 15/15 PASS (Python proxy tests)
**Trust Root**: 33-entry manifest verifies clean
**Elapsed**: 52.4s
**Prompt size**: 294,208 chars

---

This audit finds that while the underlying technical work of Phase A appears sound and has been rigorously debugged through six prior rounds, the current packet fails on two key procedural and documentary grounds. The A8e7 structural rewrite, intended to create a clean separation between current-state and historical context, was not fully executed, leaving the current-state packet littered with historical cross-references. Additionally, the packet is not self-contained as claimed, omitting a key source file required for verification. These are correctable defects, but they represent a failure of the rigorous process mandated by the project's governance.

### Finding 1: A8e7 Structural Rewrite Incomplete (High-Severity Procedural)

The A8e7 rewrite was triggered by a "recurring documentary CHALLENGE pattern" where the packet conflated a stable snapshot with an append-only history. The stated fix was to split the two modes into two documents. However, the current-state packet violates this separation by including numerous historical references to the in-cycle fix bundles. The packet should describe the current state without explaining *how* it arrived at that state; the "how" belongs exclusively in the `A8_AUDIT_HISTORY` document.

-   **§ 1 / file:line**: `A8_EXIT_PACKET_2026-04-26.md` § 1, paragraph 1: "...plus the in-cycle fix bundles A8e..A8e6 produced during the multi-round audit." This is a historical statement.
-   **§ 3 A1 / file:line**: `A8_EXIT_PACKET_2026-04-26.md` § 3, Atom A1, "Statistical framing": "(post-A8e F6 + A8e2 G2 wording corrections)". This explicitly references historical fixes.
-   **§ 3 A6 / file:line**: `A8_EXIT_PACKET_2026-04-26.md` § 3, Atom A6, title: "(+ A8e fix F4)".
-   **§ 3 A6 / file:line**: `A8_EXIT_PACKET_2026-04-26.md` § 3, Atom A6, "9 wired anchor sites": "(post-A8e total ...)", "(added by A8e F4)".
-   **§ 3 A6 / file:line**: `A8_EXIT_PACKET_2026-04-26.md` § 3, Atom A6, "Per-run correlation": "(added by A8e F1)".
-   **§ 3 A6 / file:line**: `A8_EXIT_PACKET_2026-04-26.md` § 3, Atom A6, "Trust Root": "...(chain position 33 via A8e)".
-   **§ 3 A7 / file:line**: `A8_EXIT_PACKET_2026-04-26.md` § 3, Atom A7, title: "(+ A8e2 fix G1 + A8e6 fix K2)".
-   **§ 3 A7 / file:line**: `A8_EXIT_PACKET_2026-04-26.md` § 3, Atom A7, "detect_provider() routing": "(post-A8e F3 + A8e6 K2)".
-   **§ 3 A7 / file:line**: `A8_EXIT_PACKET_2026-04-26.md` § 3, Atom A7, `cargo test --workspace`: "(added by A8e2 G1)".
-   **§ 3 A7 / file:line**: `A8_EXIT_PACKET_2026-04-26.md` § 3, Atom A7, "Trust Root": "A8e adds ... A8e2 adds ...".

This pattern of historical reference is pervasive and undermines the stated goal of the A8e7 rewrite.

### Finding 2: Packet Not Self-Contained (Medium-Severity Procedural)

The packet's introductory paragraph claims it is "self-contained". However, it omits the source code for `experiments/minif2f_v4/tests/llm_proxy_python_conformance.rs`, which was added in fix G1 to close the original VETO finding from Round 1. Without this source file, it is impossible to independently verify the auditor question Q1.c regarding its fail-closed behavior.

-   **§ 6 Q1.c / file:line**: `A8_EXIT_PACKET_2026-04-26.md` § 6, Q1.c asks the auditor to verify the fail-closed logic of `experiments/minif2f_v4/tests/llm_proxy_python_conformance.rs`.
-   **Packet Contents / file:line**: The "Appended source files" section does not contain this file, making verification of Q1.c impossible from the packet alone.

### Other Observations (Non-blocking)

-   **Q1, Q2, Q3, Q4, Q5 (excluding findings above)**: All other specific auditor questions were checked and appear to be correctly addressed in the current state. The code logic for `run_id` minting (Q1.a) and proxy routing (Q1.b) is sound and well-tested. The PREREG amendment (Q2) is statistically correct. The Trust Root manifest (Q3.c) appears to contain only load-bearing files. The history of closures (Q4.d) is complete. The packet/history split *pattern* (Q5.a) is consistent with project precedent, even if its implementation was flawed (Finding 1).
-   **`make_pput` arity**: The known limitation regarding the 24-argument `make_pput` function is well-documented in §5, Risk #5. Deferral to Phase B is acceptable.

---

-   **VERDICT**: CHALLENGE
-   **Conviction**: high
-   **Recommendation**: FIX-THEN-PROCEED