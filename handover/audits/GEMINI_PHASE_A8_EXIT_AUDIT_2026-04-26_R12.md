# Gemini Phase A → B Exit Audit (PPUT-CCL arc)
**Round**: R12
**Date**: 2026-04-26
**Reference**: see `handover/audits/A8_AUDIT_HISTORY_2026-04-26.md` for full chronology including atom commit chain + per-round verdicts/fixes.
**Test baseline**: 265 PASS + 29 ignored + 0 failed (Rust); 16/16 PASS (Python proxy tests)
**Trust Root**: 37-entry manifest verifies clean
**Elapsed**: 75.9s
**Prompt size**: 325,739 chars

---

This audit has reviewed the Phase A → B Exit Audit Packet (A8) for the PPUT-CCL arc. The packet, its companion history document, and the appended source files are of high quality. The extensive, multi-round audit history has clearly resulted in a significant hardening of the codebase, documentation, and governance processes. The structural split between the stable-state packet and the append-only history (A8e7) is a notable process improvement that has successfully eliminated the recurring documentary-staleness challenges seen in rounds 2–6.

All specific questions posed to the auditor (Q1–Q5) have been verified against the provided materials and are found to be satisfactory. The fixes for the 13 substantive findings from prior rounds, including critical issues in routing logic, statistical framing, and gate machinery, have been confirmed as correctly implemented and holding in the current state.

However, an independent review of the core `run_swarm` logic has identified one new substantive finding.

### Finding 1: Stale Search Cache After Hitting `SEARCH_CAP`

-   **Atom**: General harness (`run_swarm` logic)
-   **File/Line**: `experiments/minif2f_v4/src/bin/evaluator.rs:749-753`
-   **Description**: The `run_swarm` function maintains a `search_count` and a `search_cache` per agent. When an agent's `search_count` exceeds the `SEARCH_CAP` (default 20), the `search` tool is no longer offered in its prompt. However, the `search_cache` containing the results of its last successful search is not cleared. The prompt builder at line 753 (`build_agent_prompt`) continues to inject these same, now-stale, search results (`hits_ref`) into every subsequent prompt for that agent for the remainder of the run. This could lead to the agent becoming stuck in unproductive loops, reasoning from outdated information, and represents a potential confound in experiments measuring agent performance. This behavior is not documented in the packet's "Risks and known limitations" (§ 5).
-   **Impact**: While this logic flaw does not directly compromise the PPUT accounting mechanisms being gated for Phase B, it represents a potential performance bug and an uncontrolled experimental variable. Given the rigor required for the PPUT-CCL arc, this state-management issue should be addressed before proceeding with large-scale experiments in Phase B.

---

-   **VERDICT**: CHALLENGE
-   **Conviction**: high
-   **Recommendation**: FIX-THEN-PROCEED