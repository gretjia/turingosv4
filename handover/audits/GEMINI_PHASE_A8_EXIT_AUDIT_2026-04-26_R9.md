# Gemini Phase A → B Exit Audit (PPUT-CCL arc)
**Round**: R9
**Date**: 2026-04-26
**Reference**: see `handover/audits/A8_AUDIT_HISTORY_2026-04-26.md` for full chronology including atom commit chain + per-round verdicts/fixes.
**Test baseline**: 265 PASS + 29 ignored + 0 failed (Rust); 16/16 PASS (Python proxy tests)
**Trust Root**: 35-entry manifest verifies clean
**Elapsed**: 50.2s
**Prompt size**: 307,351 chars

---

**Role**: skeptical adversarial reviewer. Independent of Codex. Conservative merge rule (memory `feedback_dual_audit_conflict`): VETO > CHALLENGE > PASS.

---

This audit verifies the current-state snapshot of the PPUT-CCL arc at the Phase A → B exit gate. The review is based on the provided packet, including the appended source files and the full chronological audit history. The precedent for rigor is the A0 exit audit and the subsequent eight rounds of the A8 audit itself, which yielded nine substantive bug fixes.

### Overall Assessment

The Phase A artifacts have undergone an exceptionally rigorous and lengthy audit cycle. The history document reveals a process that successfully identified and closed a VETO-level defect (A7 automated conformance), multiple correctness bugs (A7 routing logic), and several subtle but critical logical contradictions in the governing `PREREG_AMENDMENT` document.

The A8e7 structural rewrite, which split the audit packet into a stable current-state snapshot and an append-only chronological history, has proven effective. The current packet is clean, free of historical lineage, and accurately describes the state of the codebase and its governing documents. This structure has finally allowed the audit process to converge by eliminating the recurring documentary-staleness challenges seen in rounds 2–6.

The current state is well-defended by an expanded Trust Root manifest (35 entries), a comprehensive conformance test suite (`fc_alignment_conformance.rs`), and a new recurring CI gate for the Python LLM proxy's core logic (`llm_proxy_python_conformance.rs`). All prior findings from rounds 1–8 appear to be closed, with the fixes verified in subsequent rounds.

### Per-Question Verification

#### Q1 — Code correctness spot-checks

-   **(Q1.a) `run_id` unification**: **PASS**. `experiments/minif2f_v4/src/bin/evaluator.rs` shows `run_id` is minted once at the entry of both `run_swarm` and `run_oneshot` via `minif2f_v4::run_id::mint_run_id`. This `run_id` variable is then passed by reference to all `fc_trace::emit_event` calls and to the final `make_pput` call within each function. The A8e fix F1 has been correctly implemented and maintained, closing the round-1 ms-drift finding.
-   **(Q1.b) `llm_proxy.py` routing**: **PASS**. `src/drivers/llm_proxy.py::detect_provider` now correctly prioritizes slash-form identifiers (`if "/" in model: return "siliconflow"`) before checking for bare substrings like `"deepseek"`. This ensures `deepseek-ai/DeepSeek-R1-Distill-Qwen-7B` routes to `siliconflow`. This logic is pinned by `scripts/test_llm_proxy.py::RoutingMatrixTests::test_deepseek_slash_form_routes_to_siliconflow`, closing the round-6 correctness bug K2.
-   **(Q1.c) Fail-closed Python wrapper**: **PASS**. `experiments/minif2f_v4/tests/llm_proxy_python_conformance.rs` contains an explicit `assert!` on the existence of the `python3` interpreter before attempting to run the script. The panic message is informative and directs the user to either install Python or set an explicit opt-out environment variable. This correctly implements the fail-closed principle required by the round-3 fix H6.

#### Q2 — PREREG amendment soundness

-   **(Q2.a) `p_0 = 0.10` substitution wording**: **PASS**. `PREREG_AMENDMENT_p0_defer_2026-04-25.md` § 2 and § 8 are now internally consistent. The text correctly identifies `p_0 = 0.10` as the "LEAST strict admissible ceiling" and clarifies that the substitution is operationally permitted at any phase, including Phase E, without inflating Type-I error. This resolves the substantive logical contradictions caught in rounds 1, 7, and 8.
-   **(Q2.b) Re-calibration conditions**: **PASS**. `PREREG_AMENDMENT` § 3 correctly frames the five conditions as pre-requisites for calibration to *run*, not as guarantees of completion before any specific phase. This is reinforced by the explicit text in § 2 and § 8 stating that Phase E can proceed with the substitution if the conditions are not met. The framing is sound.
-   **(Q2.c) Amendment in Trust Root**: **PASS**. `genesis_payload.toml` includes an entry for the amendment. `experiments/minif2f_v4/tests/trust_root_immutability.rs` includes the amendment in its list of required files, and the packet reports that `boot::tests::verify_trust_root_passes_on_intact_repo` passes.

#### Q3 — Atomicity, FC-trace discipline, governance

-   **(Q3.a) Atomic commits**: **PASS**. The commit chain presented in § 1 of the packet is consistent with the project's FC-first and atomicity rules. The `judge.sh` hook (itself in Trust Root) is the enforcement mechanism for the `FC-trace:` and no-`constitution.md` rules.
-   **(Q3.b) Cases C-071..C-075**: **PASS**. The cases are cross-referenced from `TRACE_MATRIX_v2_2026-04-25.md` and appear to correctly sediment the governance decisions made during Phase A.
-   **(Q3.c) Trust Root manifest**: **PASS**. The 35 entries listed in `experiments/minif2f_v4/tests/trust_root_immutability.rs` are all load-bearing. They include not only core logic and constitutional documents but also the tooling that enforces the constitutional process (e.g., `judge.sh`, `engine.py`, conformance tests), which is a sign of a mature and self-defending system per C-075.

#### Q4 — Phase A → B exit decision

-   **(Q4.a-c) Spot-checks**: **PASS**. The metrics in § 2 are clean and all automated checks are reported as passing.
-   **(Q4.d) Open P0 defects**: **PASS**. The `A8_AUDIT_HISTORY` document confirms that all findings from all prior rounds have a corresponding fix bundle, and the final state of the code reflects these closures. There are no open P0 defects.
-   **(Q4.e) Blockers for Phase B**: **PASS**. There are no blockers. Phase A has successfully delivered the necessary schema extensions (`jsonl_schema.rs`), budget regime logic (`budget_regime.rs`), and decomposed metrics (`hit_max_tx`, etc.) that are prerequisites for the Phase B scope of kernel instrumentation and PPUT accounting.

#### Q5 — Packet/history split

-   **(Q5.a) Consistency**: **PASS**. The split follows the established project pattern for managing immutable artifacts and their amendments.
-   **(Q5.b) History append-only**: **PASS**. The history document is structured as a chronology with frozen, timestamped entries.
-   **(Q5.c) Packet purity**: **PASS**. The current-state packet is now free of historical lineage text ("post-A8e", "fix F4", etc.). It correctly describes the "what" and leaves the "how" to the history document. The A8e7 structural rewrite and the subsequent A8e8 fix M3 have been successfully implemented.

### Conclusion

The artifacts presented in this packet have satisfied all prerequisites for Phase B. The audit process, though lengthy, has proven its value by catching and correcting nine substantive defects before the start of kernel instrumentation. The codebase and its governing documents are now in a robust, verifiable, and internally consistent state.

---

-   **VERDICT**: PASS
-   **Conviction**: high
-   **Recommendation**: PROCEED to Phase B