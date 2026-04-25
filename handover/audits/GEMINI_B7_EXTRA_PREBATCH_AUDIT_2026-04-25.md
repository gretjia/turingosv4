# Gemini PPUT-CCL Phase B B7-extra Pre-Batch Audit
**Date**: 2026-04-25
**Targets**: rollback_sim.rs + boot.rs + genesis_payload.toml + run_p0_calibration.sh + compute_p0.py + alignment + findings
**Test baseline**: 187/187 PASS + 20 ignored
**Elapsed**: 63.8s
**Prompt size**: 81,240 chars

---

An independent audit of the Phase B B7 + B7-extra pre-batch gate follows.

---

### Q1 — Constitutional anchor of synthetic-veto-at-tx-50

-   **(Q1.a) Soundness of equivalence claim**: **CHALLENGE**. The claim of observational equivalence is a rationalization that holds only for the narrow purpose of the `p_0` calculation, not in general. The module header for `rollback_sim.rs` states the short-circuit is "observably equivalent: identical exit state, identical cost accumulator...". This is incorrect.
    -   **Cost**: The cost accumulator is *not* identical. A true 150-tx vetoed loop would involve LLM calls for each proposal, accumulating cost. The short-circuit explicitly prevents this. This is acknowledged in Finding (B) (`B7_EXTRA_ABSTRACTION_DEPTH_FINDINGS...`) and the `synthetic_short_circuit` doc comment (`evaluator.rs:149-151`), which directly contradicts the `rollback_sim.rs` header's claim.
    -   **Bus Path**: The short-circuit bypasses the bus's `evaluate_predicates` path for tx 50-199. A true vetoed loop would exercise this path. This is the subject of Finding (A).
    -   **Conclusion**: The equivalence is a convenience that is valid *only because* `compute_p0.py` ignores cost and intra-run state, joining only on the final `SOLVED`/`UNSOLVED` status. The constitutional anchor is weak; it achieves an outcome analogous to FC2-N22 HALT, but it does not *traverse* the constitutional path of FC1-E18 (repeatedly). The documentation in `rollback_sim.rs` should be amended to state this is a *functionally equivalent outcome for p_0 estimation* rather than a generally "observably equivalent" process.

-   **(Q1.b) Warning level**: **CHALLENGE**. The `info!` log at `evaluator.rs:509` is insufficient for the semantic gravity of the cost-asymmetry warning. A log reader can easily miss an `info!` message. Given that this asymmetry could lead to incorrect conclusions if the data is ever re-used, this should be a `warn!`. This provides a more durable and visible warning in the execution trace itself, complementing the code comment.

-   **(Q1.c) Serialization of `synthetic_short_circuit`**: **PASS**. The implementation is correct. The field `synthetic_short_circuit` in `PputResult` (`evaluator.rs:142`) is an `Option<bool>` with `#[serde(skip_serializing_if = "Option::is_none")]`. The default construction path in `make_pput` (`evaluator.rs:1252`) sets it to `None`, ensuring it is omitted for control runs. The treatment short-circuit path (`evaluator.rs:517`) explicitly sets it to `Some(true)`. This matches the smoke test verification.

### Q2 — Trust Root manifest sufficiency

-   **(Q2.a) `src/boot.rs` omission**: **PASS**. The rationale in `TRACE_MATRIX_v1 § 4` is sound within the stated threat model (passive tamper vs. malicious recompile). An attacker who can recompile the verifier can bypass any check. Hashing the verifier's source does not prevent this. The choice is defensible.

-   **(Q2.b) `src/main.rs` omission**: **VETO**. This is a critical omission. `TRACE_MATRIX_v1 § 4` defends omitting `boot.rs`, but `main.rs` is the *caller* that enforces the check. If `main.rs:12` is commented out, the entire Trust Root verification is silently bypassed. An attacker with passive file system access (the stated threat model) could make this one-line change without recompiling (if source is deployed alongside binary) or as part of a malicious recompile. The call to `verify_trust_root` is the lynchpin of the entire security model; its call site must be immutable. `src/main.rs` must be added to the Trust Root manifest.

-   **(Q2.c) `experiments/minif2f_v4/src/lib.rs` omission**: **PASS**. This file only contains `pub mod` declarations. Tampering that removes a module would cause a compile-time error if that module is used in `evaluator.rs`. Since `evaluator.rs` *is* in the Trust Root, its hash would change, and the tampering would be detected transitively. This is sufficient.

-   **(Q2.d) `cases/MANIFEST.sha256` depth**: **PASS**. Hashing a manifest of file hashes is a standard and sufficient integrity mechanism (cf. git tree objects).

-   **(Q2.e) `Cargo.lock` omission**: **VETO**. This is a severe supply-chain vulnerability. The `Cargo.lock` file freezes the exact versions of all transitive dependencies. Without it in the Trust Root, an attacker could modify it to inject a malicious version of a dependency (e.g., `serde`, `sha2`, `tokio`). The build would succeed, but the resulting binary would be compromised. This completely undermines the integrity of the execution environment. `Cargo.lock` must be added to the Trust Root.

### Q3 — p_0 calibration semantics

-   **(Q3.a) Fairness of proxy**: **CHALLENGE**. The proxy "control takes ≥ 50 tx to solve" is a measure of problem difficulty relative to the current agent's capability. It is not a direct proxy for "how often does a Phase E artifact corrupt mid-run state". The latter is a hypothetical future event. The former is a measurable property of the present system. While this is a reasonable, pragmatic choice for a guardrail, the documentation (`PREREG § 5.5`) should be more precise about what is being measured: "mimicking the worst-case... Phase E artifacts could trigger" is an overstatement. It mimics a *HALT at tx 50*. The link to Phase E artifacts is an inference, not a direct simulation.

-   **(Q3.b) `max(over seeds)` framing**: **PASS**. `PREREG § 5.5` line 450 explicitly specifies `max over the 2 seeds`. The implementation in `compute_p0.py:84` correctly reflects this policy (`if regression > per_problem_regression[pid]: ...`). This is a consistent implementation of a pre-registered, "worst-case" policy choice.

-   **(Q3.c) `solved()` predicate**: **PASS**. The `solved()` function in `compute_p0.py:44-48` correctly prioritizes `progress_verified` over the legacy `has_golden_path`, which aligns with the B4 audit findings and the `RunAggregate::V2` schema.

-   **(Q3.d) Silent drop in `compute_p0.py`**: **CHALLENGE**. The script at `compute_p0.py:56-58` silently skips rows that are missing `calibration_problem_id` or `calibration_seed`. A failure in the runner script (`run_p0_calibration.sh:170-177`) to stamp a row would cause that data point to be silently dropped, biasing the `p_0` result without warning. The script should fail loudly if any row from either input file is missing the required keys.

-   **(Q3.e) Runner script seed tampering**: **CHALLENGE**. The seeds `[31415, 2718]` are frozen in `PREREG § 5.5` and hardcoded in `run_p0_calibration.sh:54`. However, the runner script itself is not in the Trust Root. An operator could modify the script to use different seeds, potentially gaming the `p_0` value, and this would not be detected by `boot::verify_trust_root`. While `PREREG.md` is hashed, this only allows for manual post-hoc detection of a discrepancy. This is a gap in automated verification.

### Q4 — Ground-truth feedback honesty (thesis v2)

-   **(Q4.a) Control runs**: **PASS**. Control runs follow the full loop where proposals are judged by the Lean oracle, satisfying the thesis.
-   **(Q4.b) Treatment runs bypass**: **PASS**. The short-circuit does bypass the predicate path. This is a deviation from the thesis loop.
-   **(Q4.c) Counter-argument (measurement artifact)**: **PASS**. This is the correct and necessary framing. The thesis describes the intended production behavior of the system. The calibration is a meta-process to measure a parameter *about* the system. It is acceptable for the measurement apparatus to not perfectly mirror the system under test, as long as the deviation is understood and accounted for. Finding (A) and the `synthetic_short_circuit` flag provide this accounting.
-   **(Q4.d) `verified = false` anchor**: **PASS**. For a short-circuited run, the ground truth is that no solution was found and verified. The caller asserting `verified = false` (`evaluator.rs:513`) is stamping a factually correct summary of the run's outcome. The anchor is the logic of the measurement protocol itself, which is sufficient.

### Q5 — Findings C+D pre-batch impact

-   **(Q5.a) Impact on `p_0` result**: **PASS**. Finding C (missing WAL Omega* events) and Finding D (mixed rejection labels) relate to the granularity of intra-run logging. The `p_0` calculation depends only on the final, run-level `verified` status. These findings have no effect on the numerical outcome of the calibration.
-   **(Q5.b) `verified` field dependency**: **PASS**. The `verified` field is populated by `post_hoc_verifier.rs`, which calls the Lean oracle. It is not dependent on bus events. The short-circuit path explicitly sets `verified = false`. In neither case do Findings C or D affect the `verified` flag.
-   **(Q5.c) Downstream interpretation**: **PASS**. The `synthetic_short_circuit` flag and its associated doc comment (`evaluator.rs:149-151`) are sufficient mitigation for Finding (B) (cost asymmetry). Any downstream tool that misinterprets the cost of these rows is ignoring the explicit warning provided in the data schema.

### Q6 — Sanity gate enforcement

-   **(Q6.a) `exit 2` abort**: **CHALLENGE**. The runner script `run_p0_calibration.sh` does not use `set -e`. If `compute_p0.py` exits with code 2, the script will continue executing, printing the summary box. An automated runner could miss the failure. The script should use `set -e` or explicitly check the exit code of `compute_p0.py` and abort.
-   **(Q6.b) `p_0 > 0.10` meaning**: **PASS**. The interpretation is correct. This is the intended behavior of the guardrail: if the system is too brittle or problems are too hard (requiring >50 tx), the calibration fails, preventing a potentially flawed `p_0` from being frozen.
-   **(Q6.c) Borderline result**: **PASS**. `PREREG § 5.5` defines a hard ceiling, not a warning band. The implementation matches the specification.

### Q7 — Calibration run economics + failure mode resilience

-   **(Q7.a) API drift**: **CHALLENGE**. This is a significant, unmitigated risk to measurement validity. An API drift mid-batch could change agent behavior and invalidate the assumption of a stationary process. The protocol should at least require logging the precise model version (if available from headers) and the start/end timestamps of the batch to aid in any post-hoc analysis of potential drift.
-   **(Q7.b) Timeout impact on `p_0`**: **CHALLENGE**. A run that times out is logged as `MEASUREMENT_ERROR` (`run_p0_calibration.sh:195`). This means the (problem, seed) pair will be missing from one of the jsonl files. `compute_p0.py` will then exclude this pair from its analysis (`set(c.keys()) & set(t.keys())`). This introduces sampling bias. A timeout is a valid outcome (UNSOLVED) and should be treated as such, not as a data error. The runner script should be modified to emit a valid UNSOLVED jsonl row for timed-out runs.
-   **(Q7.c) No resume mode**: **PASS**. This is an operational inconvenience, not a validity threat. For an 8-hour batch, it is a high inconvenience, but acceptable for a one-off calibration.
-   **(Q7.d) Oracle preflight depth**: **PASS**. The preflight check is minimal but sufficient for its purpose: to detect a completely broken Lean environment before starting an 8-hour batch.
-   **(Q7.e) Cost overrun**: **PASS**. This is a financial risk, not a validity risk. The user's estimate appears optimistic if many runs approach the timeout, but the `timeout 2400` command provides a hard cap on per-run duration, limiting the worst-case scenario.

### Q8 — Constitutional flowchart compliance (FC1/FC2/FC3 trace)

-   **(Q8.a) "Verify" column check**:
    -   `rollback_sim`: **CHALLENGE**. The trace is incorrect. As established in Q1 and Finding (A), this is an evaluator-layer *bypass*, not an implementation that routes through the FC1-E18 + FC2-N22 bus-level paths. The status in `TRACE_MATRIX_v1 § 2` should be `⚠️ partial` with a note explaining the bypass.
    -   All others: **PASS**. The other traces are legitimate.
-   **(Q8.b) FC3-N34 promotion**: **PASS**. The implementation in `src/boot.rs` is a direct and robust implementation of the constitutional requirement. The promotion to ✅ is justified.
-   **(Q8.c) TRACE_MATRIX_v2 needed**: **PASS**. `TRACE_MATRIX_v1` was created specifically to cover these B7 changes and appears sufficient.

---

### **VERDICT**: VETO

**Conviction**: High.

The discovery of two critical omissions from the Trust Root manifest (`src/main.rs` and `Cargo.lock`) represents a fundamental failure of the system's integrity guarantee. These are not minor issues; they undermine the entire premise of the `boot::verify_trust_root` check. The batch cannot proceed until these are fixed, as the resulting `genesis_payload.toml` would be based on a compromised verification process.

Several CHALLENGE-level findings also represent significant threats to the validity and robustness of the measurement.

**Specific recommendation**: **REDESIGN** (of Trust Root manifest) then **FIX-THEN-PROCEED**.

The batch must be blocked. The following P0 fixes are mandatory before re-submitting for audit:

1.  **VETO Fix**: Add `src/main.rs` to the `[trust_root]` manifest in `genesis_payload.toml`. Its integrity is as important as the verifier it calls. (`Q2.b`)
2.  **VETO Fix**: Add `Cargo.lock` to the `[trust_root]` manifest. This is non-negotiable for supply-chain security and reproducible builds. (`Q2.e`)
3.  **CHALLENGE Fix**: Modify `compute_p0.py` to fail loudly if any row is missing the required `calibration_*` keys, preventing silent data loss and biased results. (`Q3.d`)
4.  **CHALLENGE Fix**: Modify `run_p0_calibration.sh` to treat a timeout as a valid `UNSOLVED` outcome and emit a corresponding JSONL row, instead of a `MEASUREMENT_ERROR` that leads to data exclusion. (`Q7.b`)

Proceeding with the batch in its current state would produce a `p_0` value and a Trust Root that are not defensible to an external observer. The integrity claims would be demonstrably false.