# Gemini PPUT-CCL Phase B B2/B3/B4 Mid-Term Audit
**Date**: 2026-04-25
**Targets**: cost_aggregator.rs + wall_clock.rs + post_hoc_verifier.rs + evaluator.rs wiring
**Test baseline**: 143/143 PASS (was 131; +12 from B2-B4 unit tests)
**Elapsed**: 62.3s
**Prompt size**: 67,672 chars

---

# Gemini Mid-Term Audit — Phase B B2/B3/B4 Implementation

**Verdict**: **CHALLENGE**

**Conviction**: High. The implementation is clean, well-documented, and correctly passes its unit and conformance tests. However, it introduces a critical architectural hazard that undermines the primary goal of B4: creating a safe, un-gameable foundation for the Phase C Soft Law ablation. A second, less severe schema-drift bug violates the B1 contract. These issues must be fixed before B5+ builds on this foundation.

---

### Top 3 P0 Issues

1.  **Architectural Hazard for Phase C (Q7, Q3.b)**: The B4 implementation is not Phase-C-safe by construction. The core logic for determining `post_hoc_verified` is hard-coded inside the `make_pput` function (`evaluator.rs:1026`) as `let post_hoc_verified = has_gp;`. A future implementer of Soft Law could easily (and incorrectly) provide a fake `has_gp=true` signal and neglect to insert the mandatory, expensive `verify_post_hoc` call, thus corrupting the North Star `pput_verified` metric. The current design relies on future discipline rather than being safe by construction.
2.  **Schema Drift from B1 Contract (Q5.c)**: The B1 `RunAggregate` v2 schema mandates a `progress: u8` field. The B4 implementation in `PputResult` emits `verified: Option<bool>` (`evaluator.rs:90`). While semantically similar, this is a direct violation of the frozen schema's type and name contract. Downstream tooling built against the B1 spec will break or require shims.
3.  **Misaligned Abstraction for Verification (Q3.b)**: The `make_pput` function (`evaluator.rs:1011`) is responsible for both PPUT *computation* and truth *determination*. It computes PPUT from `(c_i, t_i)` but also decides what `post_hoc_verified` means. This violates separation of concerns. The caller (the run loop) should determine the ground truth (`runtime_accepted`, `post_hoc_verified`) and pass those values to a pure computation function.

---

## Detailed Findings

### Q1 — C_i full-cost honesty (anti-Goodhart #8)

The B2 cost aggregator implementation is sound.

*   **(Q1.a) Unwired `Ok(response)` sites**: **PASS**. The two `client.generate(...)` call sites in `evaluator.rs` (`run_oneshot:232`, `run_swarm:583`) are both immediately followed by `acc.record_llm_call(...)`. No call paths appear to be un-metered.
*   **(Q1.b) Unwired proposal parse paths**: **PASS**. Both LLM call sites are also immediately followed by `acc.record_proposal(false)` (`evaluator.rs:234`, `evaluator.rs:587`). This correctly establishes the "fail-by-default" accounting that `flip_last_failed_to_accepted` relies on.
*   **(Q1.c) Tool stdout for bus mutations**: **PASS (Defensible)**. The implementation correctly counts agent-observable feedback (`search hits`, `reject error_detail`, `parse-fail label`) as `tool_tokens`. It does *not* count the state changes from `append`, `invest`, or `post`. This is defensible because the result of those mutations is the new proof state, which is fully embedded in the *next* prompt. Counting it as tool output and then again as prompt input would be double-counting. The current model correctly attributes the cost to the subsequent prompt.
*   **(Q1.d) `flip_last_failed_to_accepted` underflow**: **PASS**. The implementation at `cost_aggregator.rs:100-102` is `if self.failed_branch_count > 0 { self.failed_branch_count -= 1; }`. This is a saturating subtraction that cannot underflow or corrupt the count. It is robust against wiring bugs.
*   **(Q1.e) `chars/4` heuristic**: **PASS (Honest Enough)**. PREREG § 5 does not mandate a specific tokenizer for tool stdout. The `(chars().count() as u64 + 3) / 4` implementation (`cost_aggregator.rs:79`) is a reasonable, deterministic, and slightly conservative (due to ceiling) heuristic. For an accounting metric, this is sufficient and avoids a dependency on a specific tokenizer library for non-LLM output.

### Q2 — T_i wall-clock seal (anti-Goodhart Soft Law escape)

The B3 wall-clock implementation is sound and forward-compatible with Phase C.

*   **(Q2.a) Lean call paths bypassing `mark_final_accept`**: **PASS**. All Lean call paths are correctly bracketed:
    *   `run_oneshot`: `verify_omega` is followed by `mark_final_accept` (`evaluator.rs:241`).
    *   `run_swarm` (OMEGA): `verify_omega_detailed` paths are followed by `mark_final_accept` (`evaluator.rs:681`).
    *   `run_swarm` (step): `verify_partial` is followed by `mark_final_accept` (`evaluator.rs:900`).
*   **(Q2.b) End-of-loop no-OMEGA path**: **PASS**. The max-transaction exit path correctly calls `mark_final_accept` (`evaluator.rs:993`), ensuring that failed runs have an accurate total time.
*   **(Q2.c) Soft Law compatibility**: **PASS**. The seal is preserved. `mark_final_accept` is called *after* the Lean verification step. When Phase C introduces a Soft Law mode that fakes a runtime accept *before* a post-hoc verification, the current bracketing will correctly include the post-hoc verification time in `T_i`, preventing a Goodhart attack where the agent fakes success and stops the clock early.
*   **(Q2.d) `from_instants` leak**: **PASS**. The function is correctly gated by `#[cfg(test)]` at `wall_clock.rs:88` and will not be present in production builds.
*   **(Q2.e) `elapsed_ms` semantic for no-OMEGA exit**: **PASS**. The implementation at `wall_clock.rs:79` correctly returns `start.elapsed()` if `final_accept` is not set. This accurately captures the wall-clock time for runs that terminate due to timeout or transaction limits.

### Q3 — Dual-PPUT divergence as H1 detection

The B4 implementation is logically correct but architecturally fragile.

*   **(Q3.a) `compute_progress_verified` AND logic**: **PASS**. The logic `if runtime_accepted && post_hoc_verified` (`post_hoc_verifier.rs:60`) is correct. Per PREREG, progress requires ground-truth acceptance. A verified proof that was not accepted by the runtime is a system anomaly, not progress, and should correctly result in `progress=0`.
*   **(Q3.b) `post_hoc_verified = has_gp` call site**: **CHALLENGE**. This is a critical design flaw. The logic is buried inside `make_pput` (`evaluator.rs:1026`). This is the wrong level of abstraction. The `evaluator`'s run loop is the component that knows *why* a run is considered accepted (e.g., real Lean call vs. Soft Law heuristic). The run loop should determine `runtime_accepted` and `post_hoc_verified` and pass them as arguments to `make_pput`. The current design invites a Phase C implementer to make a subtle but catastrophic error.
*   **(Q3.c) `verify_post_hoc` dead code**: **PASS (Deferred)**. The function is correctly defined (`post_hoc_verifier.rs:48`) and its non-use in Phase B is intentional, as documented. The comments are clear enough to guide the Phase C implementation.
*   **(Q3.d) `pput_m_verified` precision**: **PASS**. The `1e6` multiplier is applied to a standard `f64`. Given the expected range of `C_i` and `T_i`, this will not cause precision issues and is a standard practice for improving the readability of small-valued metrics.

### Q4 — Backward compat with legacy jsonl

The implementation correctly handles backward compatibility.

*   **(Q4.a) Round-trip preserved**: **PASS**. All new fields in `PputResult` are `Option<T>` and use `#[serde(skip_serializing_if = "Option::is_none")]` (`evaluator.rs:82-93`). This is the canonical and correct way to ensure that deserializing old data and re-serializing it produces identical output.
*   **(Q4.b) Downstream tooling**: **PASS**. The implementation correctly anticipates that downstream tools must handle `Option` types. The field names align with the B1 `RunAggregate` schema (see Q5.b), which is the correct contract.
*   **(Q4.c) `..r` field-spread**: **PASS (Inferred)**. While the `Hybrid_v1` code is not in the provided diff, the standard behavior of Rust's struct update syntax (`..r`) ensures that all fields of the struct, including the newly added ones, are propagated. This is not a risk.

### Q5 — Schema → emit alignment

The implementation deviates from the B1 schema contract.

*   **(Q5.a) `PputResult` vs `RunAggregate`**: **CHALLENGE (Minor)**. The plan to switch to `RunAggregate` was deferred. Instead, fields were added to the legacy `PputResult`. This is defensible as an incremental step, but it creates technical debt and naming confusion. A plan should be made to unify these types.
*   **(Q5.b) Field name alignment**: **PASS**. A manual check of the new fields in `PputResult` (`evaluator.rs:82-93`) against the `RunAggregate` v2 schema shows they are correctly named and spelled.
*   **(Q5.c) `progress: u8` vs `verified: Option<bool>`**: **CHALLENGE (Major Bug)**. This is a clear deviation. The B1 `RunAggregate` schema specifies `progress: u8`. The B4 implementation adds `verified: Option<bool>` (`evaluator.rs:90`). The emitted JSONL will have a field named `verified` with a boolean value, not a field named `progress` with a numeric value. This breaks the B1 contract and will fail validation against a strict schema-checker.

### Q6 — Architectural / structural concerns

The implementation is structurally sound and follows process.

*   **(Q6.a) STEP_B_PROTOCOL violation**: **PASS**. The diff touches `src/drivers/llm_http.rs`. The restricted file list (`bus.rs`, `kernel.rs`, `wallet.rs`) does not include the `drivers/` directory. No process violation occurred.
*   **(Q6.b) Hidden coupling**: **PASS**. The new modules (`cost_aggregator`, `wall_clock`, `post_hoc_verifier`) are well-defined and loosely coupled. The `evaluator` uses their public APIs cleanly. This is a good design.
*   **(Q6.c) Trust Root**: **PASS (Finding for B7)**. The PREREG § 1.8 defines the Trust Root. The logic for cost, time, and verified progress accounting is fundamental to the integrity of the PPUT metric. Therefore, `cost_aggregator.rs`, `wall_clock.rs`, and `post_hoc_verifier.rs` **must** be added to the Trust Root manifest. This is a mandatory action item for the B7 implementation.

### Q7 — End-to-end stress test

The current design is **not** Phase-C-safe by construction.

As detailed in the P0 issues and Q3.b, the logic `let post_hoc_verified = has_gp;` (`evaluator.rs:1026`) creates a trap. A future implementer working on Soft Law will modify the `evaluator` to produce a `has_gp = true` signal without running Lean. They could easily fail to notice that this fake signal will propagate into `post_hoc_verified`, effectively laundering a fake acceptance into the ground-truth North Star metric.

The architecture does **not** force the Phase C implementer to do the right thing. It relies on them being disciplined enough to refactor the internals of `make_pput`. This is an unacceptable risk for a foundational anti-Goodhart mechanism.

---

## Recommendations for B5/B6/B7

1.  **Immediate B4 Fix (P0)**: Refactor `make_pput` before any further work.
    *   Change its signature to be purely computational: `make_pput(..., runtime_accepted: bool, post_hoc_verified: bool, ...)`.
    *   Remove the internal logic that sets `post_hoc_verified = has_gp`.
    *   Update all call sites in `evaluator.rs` to explicitly determine `runtime_accepted` and `post_hoc_verified` and pass them in. For the current Phase B, this will simply be passing `has_gp` for both arguments, but the contract will be explicit and safe for Phase C.

2.  **Immediate B4 Fix (P0)**: Align `PputResult` with the `RunAggregate` schema.
    *   Rename the `verified` field to `progress_verified`.
    *   Change its type from `Option<bool>` to `Option<u8>`.
    *   Update the `make_pput` logic (`evaluator.rs:1030-1031`) to store the `u8` progress value, not a boolean.

3.  **B5 Design Debt**: Schedule the removal of the redundant `PputResult` struct. The `evaluator` should construct and emit the canonical `jsonl_schema::RunAggregate` struct directly.

4.  **B7 Implementation Mandate**: Add `cost_aggregator.rs`, `wall_clock.rs`, and `post_hoc_verifier.rs` to the Trust Root manifest. This is non-negotiable for system integrity.