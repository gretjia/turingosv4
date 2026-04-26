# Gemini Phase A → B Exit Audit (PPUT-CCL arc) — round 2
**Date**: 2026-04-26 (post A8e fixes)
**Round**: R6
**Commits**: 2e7f75a / d8950ee / 2a65339 / e94e1b9 / 62c4e14 / 6be6eb4 / 180a300 / 7f4bc0c / a5c78e4 / 30f2a14 / 89994c7 / 90953d6 / 60292dc / 5a56ff6
**Test baseline**: 264 PASS + 29 ignored + 0 failed (Rust); 15/15 PASS (Python proxy tests)
**Trust Root**: 33-entry manifest verifies clean
**Elapsed**: 41.9s
**Prompt size**: 289,206 chars

---

This audit has been conducted on the state of the PPUT-CCL arc after the `A8e5` commit, which incorporates five rounds of prior audit feedback. The code base appears sound and the major procedural gaps identified in earlier rounds (e.g., lack of recurring conformance tests, inconsistent identifiers) have been closed. However, the mandate for a Phase A exit audit requires exacting rigor, particularly concerning the self-consistency and accuracy of the documentation packet itself. After six rounds, the packet and its supporting artifacts must be flawless. They are not.

The findings below are documentary. While they do not require source code changes, their existence after five previous rounds of fixes indicates a persistent lack of holistic review. The conservative merge rule and the precedent for rigor from the A0 audit compel a final CHALLENGE to ensure the artifacts entering Phase B are pristine.

---

### Finding 1: Stale Anchor Count in TRACE_MATRIX Summary

The summary description for atom A6 in the TRACE_MATRIX is internally inconsistent and stale. The top-line summary claims six anchor sites, while the detailed text in the same paragraph correctly states the post-A8e total is nine. This contradiction survived the A8e5 review cycle.

-   **File**: `handover/alignment/TRACE_MATRIX_v2_2026-04-25.md`
-   **Atom**: A6 (per-line FC tagging)
-   **Line**: The line beginning "Six anchor sites wired..."
-   **Defect**: The line states "Six anchor sites wired (5 in `run_swarm` + 1 in `run_oneshot`)..." This is immediately contradicted by the following sentence: "A8e fix F4 added 3 more in `run_swarm` ... bringing the post-A8e total to 9 production emit sites." The summary was not updated when the details were added.

### Finding 2: Incorrect Fix Cross-Reference in A8 Packet

The A8 Exit Packet incorrectly attributes the correction of the Trust Root manifest count to fix `H4`. Fix `H4` (from round 4) corrected a comment in `genesis_payload.toml`. The fix that corrected the per-atom delta counts within the A8 packet itself was `I2` (from round 5). This indicates a copy-paste or incomplete search-and-replace during the round-6 fixes.

-   **File**: `A8_EXIT_PACKET_2026-04-26.md` (the main packet document)
-   **Section**: § 3. Per-atom FC-trace map and acceptance evidence
-   **Atoms**: A5 (budget regime) and A6 (FC tracing)
-   **Defect**:
    -   The A5 description ends with: "(A8e3 fix **H4** corrected the prior 25→26 claim...)"
    -   The A6 description ends with: "(Same off-by-one correction as A5's delta...)"
    -   The correct fix that adjusted these numbers in the packet was **I2**. The cross-reference is incorrect, demonstrating a lack of self-consistency in the final packet.

---

-   **VERDICT**: CHALLENGE
-   **Conviction**: high
-   **Recommendation**: FIX-THEN-PROCEED

The project is ready for Phase B. The code is sound, the test harnesses are robust, and the major risks have been addressed. However, the constitutional process demands that the final exit packet and its supporting documentation be free of error. These two minor but unambiguous documentary defects must be corrected in a final `A8e6` commit before Phase B is authorized. No further review should be necessary if the fixes are limited to these two items.