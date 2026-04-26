# Gemini Phase A0 Exit Audit (Harness Modernization)
**Date**: 2026-04-25
**Commits**: 2e7f75a / d8950ee / 2a65339 / e94e1b9
**Test baseline**: 204 PASS + 29 ignored
**Trust Root**: 24-entry manifest verifies clean
**Elapsed**: 47.3s
**Prompt size**: 56,868 chars

---

# Phase A0 Exit Audit Report

**To**: Codex Governance Board
**From**: Independent Reviewer
**Subject**: Verdict on Gemini Phase A0 Exit Audit — Harness Modernization

---

## 1. Executive Summary

**VERDICT**: **CHALLENGE**

**Conviction**: **High**

**Recommendation**: **FIX-THEN-PROCEED**. The design principles of A0 are sound and address critical gaps in governance. However, the implementation contains several exploitable loopholes and process-noise issues that undermine the stated goal of preventing "silent drift". These flaws must be fixed within the A0 cycle before Phase A engineering atoms can safely begin. A full redesign is not required.

## 2. Detailed Findings

This audit assesses the A0 deliverables against the mandate to create a robust harness. Findings are organized by the specific questions posed.

### Q1 — Rule completeness

The new rules are a significant step forward, but the implementation and scope contain critical gaps.

-   **(Q1.a) Other B7+ surfaces still uncovered?**
    **CHALLENGE**. The provided rules do not cover other known areas of potential drift mentioned in the prompt. There are no rules governing `synthetic_short_circuit` field discipline, sanity-checking `AGENT_MODELS` environment variables, or enforcing the value domain of `BUDGET_MODE`. These omissions represent known-unknowns that remain un-gated, contrary to the spirit of A0.

-   **(Q1.b) R-014 fires on EVERY *.rs edit. Acceptable noise or scope-narrow?**
    **CHALLENGE**. The rule in `rules/active/R-014_trust_root_manifest_drift.yaml` uses `file_glob: "*.rs"`. A review of `genesis_payload.toml` shows only a small subset of `.rs` files are in the Trust Root. This implementation will generate a high volume of false-positive warnings, leading to alert fatigue and increasing the probability that a genuine required update is missed. The rule's `check` logic should be narrowed to only fire if the edited file is actually listed in `genesis_payload.toml`.

-   **(Q1.c) R-018 BLOCK path-routing logic airtight?**
    **CHALLENGE (High Severity)**. The logic in `.claude/hooks/judge.sh:58` (`[ "$(basename "$FILE_PATH")" = "constitution.md" ]`) is **NOT airtight**. It is vulnerable to a symlink bypass. An agent could create a symlink (`ln -s constitution.md my_other_doc.md`) and then edit `my_other_doc.md`. The `basename` would be `my_other_doc.md`, which would then match the `*.md` skip-list pattern at `judge.sh:75`, silently bypassing the R-018 sudo gate. This is a critical vulnerability that directly contradicts the mandate of C-071 and Art. V.1.1. The script must resolve the file's real path before performing the check.

-   **(Q1.d) R-016 `git commit -F` bypass?**
    **CHALLENGE (High Severity)**. The check in `.claude/hooks/judge.sh:48-56` greps the `COMMAND` variable for `FC-trace:`. As correctly hypothesized, a command like `git commit -F /tmp/msg` would pass this check, as the `COMMAND` string itself does not contain the commit message body. This allows an agent to bypass the FC-first discipline mandated by C-074. The hook must be sophisticated enough to inspect the actual commit message content, not just the command-line arguments.

### Q2 — Conformance battery integrity

The battery is a crucial addition, but its implementation could be hardened.

-   **(Q2.a) Are witnesses TRULY witnesses?**
    **CHALLENGE**. The battery is inconsistent. While some tests like `fc1_n4_tape_constructible_with_time_arrow` are strong witnesses that call methods, others like `fc1_n1_q_state_carrier_present` use `std::any::type_name::<T>()`. This only proves the type *exists*, not that its constitutionally relevant API is intact. This is a weak form of witnessing that could allow for silent drift if, for example, a key method on `TuringBus` were removed but the type itself remained. The weaker witnesses should be strengthened to call the specific functions or methods cited as evidence in `TRACE_MATRIX_v2`.

-   **(Q2.b) Ignored stubs use `panic!()` body. Right pattern?**
    **PASS**. This is the correct and robust pattern for ignored tests. It ensures that if a test is accidentally un-ignored, it will fail loudly rather than silently passing.

-   **(Q2.c) Cross-crate symbols deferral acceptable?**
    **PASS**. Deferring the witness test for `Lean4Oracle` to a separate `fc_alignment_conformance.rs` within its own crate (`experiments/minif2f_v4/tests/`) is an acceptable modularization strategy. The gap is documented in `tests/fc_alignment_conformance.rs:248`.

-   **(Q2.d) Binary-only symbols ignored. Defer refactor?**
    **PASS**. The harness has correctly identified an architectural limitation where symbols in `evaluator.rs` are not easily testable from the library. Documenting this in `tests/fc_alignment_conformance.rs:243` and deferring the refactoring to a later phase is a reasonable project management decision.

### Q3 — Case-law sediment

The new cases effectively formalize recent decisions into constitutional precedent.

-   **(Q3.a) TRUE precedents or session-summary docs?**
    **PASS**. The `ruling:` and `precedent:` sections in C-071 through C-075 use binding language (e.g., "frozen", "MUST", "NOT allowed"). They function as true precedents for future automated and human action.

-   **(Q3.b) C-073 overreach vs Art. V.1.2?**
    **PASS**. `cases/C-073_architect_ai_commit_authority.yaml` states ArchitectAI scope is "ALL files EXCEPT constitution.md". This is a direct and faithful interpretation of Art. V.1.2 ("ArchitectAI commit authority on non-constitution") and Art. V.1.1 ("sudo only constitution.md"). There is no overreach.

-   **(Q3.c) C-074 duplication?**
    **PASS**. The relationship between `cases/C-074_fc_first_problem_handling.yaml` and the `feedback_fc_first_problem_handling` memory is not duplication but *sedimentation*. The case formalizes an informal "memory" into binding case-law, which is a core function of this governance system.

-   **(Q3.d) Case cross-references form DAG. Circular reasoning hazard?**
    **PASS**. The cross-references between cases (e.g., C-071 ↔ C-073) represent a system of interdependent definitions of authority and process. This is analogous to legal frameworks and does not constitute fallacious circular reasoning. The system is coherent.

### Q4 — Trust Root manifest expansion 20 → 24

The expansion correctly identifies governance instrumentation as a constitutional artifact. The scoping decisions are sound.

-   **(Q4.a) Should `rules/SCHEMA.yaml` also be in Trust Root?**
    **PASS**. The rationale in `TRACE_MATRIX_v2_2026-04-25.md § 5` for excluding `SCHEMA.yaml` is sound: the engine does not currently use it for validation. Including it would be premature.

-   **(Q4.b) `build-check.sh` + `session-end.sh` excluded — gap?**
    **PASS**. `judge.sh` is the most critical pre-tool-use gate. Excluding the other, lower-priority hooks is a reasonable scoping decision for A0, as documented in `TRACE_MATRIX_v2 § 5`.

-   **(Q4.c) Should ALL tests be in Trust Root?**
    **PASS**. The decision to include only `tests/fc_alignment_conformance.rs` is correct. This file is unique in its role as a direct, mechanical witness for the constitution-to-code mapping. Including all tests would create excessive manifest churn for minimal gain in constitutional stability.

-   **(Q4.d) `TRACE_MATRIX_v?.md` docs not in Trust Root. Acceptable?**
    **PASS**. The rationale in `TRACE_MATRIX_v2 § 5` is correct. The document is descriptive, not prescriptive enforcement. The enforcement mechanism (`fc_alignment_conformance.rs`) *is* in the Trust Root. This avoids a problematic self-reference while securing the actual enforcement vector.

### Q5 — Constitutional alignment (meta-check)

The A0 changes themselves appear to follow the constitutional process they are designed to enforce, with one point requiring verification.

-   **(Q5.a) Did A0a-d touch `constitution.md`?**
    **PASS**. A review of the deliverable list confirms that the four A0 commits modified rules, tests, cases, and the trace matrix/manifest. None of them touched `constitution.md`, which would have required a separate `sudo` authorization.

-   **(Q5.b) Did each A0 commit message contain `FC-trace:`?**
    **CHALLENGE**. The R-016 rule was introduced in commit `2e7f75a`. The subsequent three commits (`d8950ee`, `2a65339`, `e94e1b9`) were made *after* this rule was notionally active. It is a procedural requirement that these commits contain a valid `FC-trace:`. Verification of the commit logs is required to close this challenge. This is a test of whether the new process was immediately adopted.

-   **(Q5.c) Regression test that asserts A0 itself can't be silently rolled back?**
    **PASS**. The combination of `boot::tests::verify_trust_root_passes_on_intact_repo` (run via `cargo test`) and the inclusion of the A0 artifacts (`judge.sh`, `rules/MANIFEST.sha256`, etc.) in `genesis_payload.toml` serves this function. A git rollback of A0 would cause a hash mismatch on boot, triggering a `TRUST_ROOT_TAMPERED` panic. The system correctly protects itself against this regression.