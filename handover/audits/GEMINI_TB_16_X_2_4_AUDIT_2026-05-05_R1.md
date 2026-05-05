# Gemini TB-16.x.2.4 Audit — Multi-WorkTx + Boltzmann RUNTIME (Class 3 dual external audit)
**Round**: R1
**Date**: 2026-05-05
**Audit target**: commit b5118fd17b0f8666a25453239104e54406e9f80b (TB-16.x.2.4 pre-audit)
**Test baseline**: cargo test --workspace = 915 PASS / 0 FAILED / 150 ignored
**Trust Root**: GREEN
**Audit envelope**: Class 3 (per umbrella charter §0; high-impact V3L-14 anti-collapse mechanism)
**Elapsed**: 62.8s
**Prompt size**: 94,665 chars
**Audit mode**: architectural strategic (Codex covers impl-paranoid in parallel)

---

# Gemini TB-16.x.2.4 Audit Report

**Auditor**: Gemini (Architectural Strategic Reviewer)
**Role**: Skeptical Adversarial Reviewer (Independent of Codex)
**Commit**: `b5118fd17b0f8666a25453239104e54406e9f80b`
**Verdict Conviction**: High

---

## Overall Recommendation: VETO

This commit is a **VETO** due to a critical lack of unit testing for new, non-trivial logic (Q8), which violates fundamental engineering quality standards for a Class 3 change.

Even if the VETO were resolved, multiple **CHALLENGE**s would remain. The implementation deviates strategically from the charter's intent by failing to wire the Boltzmann selector into the sequencer's admission path (Q1), and the provided smoke evidence represents a degenerate, hollow exercise of the anti-star-topology mechanism it claims to test (Q2). The commit successfully builds the *plumbing* for Boltzmann runtime exercise but fails to demonstrate its *purpose* or meet basic testing standards.

---

## Detailed Audit Findings (Q1-Q12)

### Q1 (STEP_B deviation)

**Verdict**: **CHALLENGE**

The deviation is **not defensible** as load-bearing-equivalent. The charter's intent, "verify boltzmann in admission path" (`TB-16.x.2_charter...md` §2 Atom 2.4), implies sequencer-side awareness and potential enforcement. The current implementation in `evaluator.rs` (diff line ~1260) places the selection logic entirely on the proposal side. The `parent_tx` is recorded in `ProposalTelemetry`, a CAS object whose contents the sequencer treats as an opaque CID (`WorkTx.proposal_cid`).

This dodges the core security and policy concern: what prevents a malicious or buggy agent from generating `ProposalTelemetry` that lies about its chosen parent? The sequencer currently has no mechanism to verify that the claimed `parent_tx` matches the output of its own canonical Boltzmann v2 selector. The implementation achieves *observation* of a cooperative agent's choice but fails to build the infrastructure for *enforcement*, which is the spirit of an "admission path" check. This is a significant strategic gap.

### Q2 (entropy degeneracy)

**Verdict**: **CHALLENGE**

The smoke test is a **hollow drill** that satisfies the letter of the ship gate but not its intent. The parent distribution `{None: 1, "iter-0": 3}` (per `README.md` and `boltzmann_trace.txt`) generates its entropy (0.811 bits) solely from the distinction between the root proposal (`None`) and all subsequent proposals.

The core intent of mechanism 5 (V3L-14) is to prevent *star-topology collapse*, where all agents build on a single, dominant parent. This smoke test *actively creates and validates a star topology*, with `iter-0` as the hub. The fact that `v2_pick` was `Some(iter-0)` for iterations 1, 2, and 3 demonstrates a lack of diversity, not its presence. The test passes a numerical threshold due to a categorical artifact (root vs. non-root), not because it has demonstrated a healthy, non-degenerate parent graph. This is a weak and potentially misleading validation of the anti-collapse mechanism.

### Q3 (selector determinism)

**Verdict**: **PASS**

The hardcoded seed `0xB01_72A_4_u64` (`evaluator.rs` diff line ~1290) is **defensible** for this specific smoke test. A primary system invariant is byte-identical replay (Art.0.2), which requires deterministic execution. Using a fixed seed for the smoke run ensures this property is met and tested. While a configurable seed is superior for generalized fuzzing, for a canonical evidence-producing run, determinism is paramount.

### Q4 (Price index population at iter 0)

**Verdict**: **PASS**

The implementation correctly handles the initial empty state. At `iter=0`, `snap.price_index` is empty, so `boltzmann_select_parent_v2` correctly returns `None` (`sdk/actor.rs:91`). The fallback logic at `evaluator.rs` (diff line ~1320) then checks `produced_worktx_ids.last()`, which is also `None`. The resulting `parent_tx = None` is the correct semantic for a root proposal. This is not a bug; it is a correct and necessary handling of the bootstrap case.

### Q5 (proposal_index uniqueness)

**Verdict**: **CHALLENGE**

The use of hardcoded magic numbers for `proposal_index` (`5 + iter_i` at `evaluator.rs` diff line ~1336) is brittle and poses a collision risk. The .2.5 hook uses index `4`. While `4` does not collide with `5, 6, 7, 8`, this relies on an implicit, undocumented assumption about the execution order of env-var hooks. If the upcoming TB-16.x.2.6 combined run reorders or parallelizes these hooks, or if the main OMEGA swarm's `proposal_count` happens to fall in this range, it could lead to data corruption or replay failures. Downstream systems that assume `(run_id, agent_id, proposal_index)` is a unique key would be compromised. This approach is not robust.

### Q6 (4 WorkTxs vs SG charter ≥3)

**Verdict**: **PASS**

Using `count=4` when the ship gate requires `≥3` is defensible. The justification of "headroom for entropy diversity" (`README.md`) is plausible. It provides a slightly more populated dataset for the entropy calculation without appearing to hide any edge-case behavior that a count of 3 would have exposed.

### Q7 (Charter SG threshold ≥ 0.5 vs assertion id=43 threshold 0.25)

**Verdict**: **CHALLENGE**

This is a **silent relaxation of the ship gate requirement**. The charter (`TB-16.x.2_charter...md` §2 Atom 2.4) specifies `SG-16.x.2.4 = "...entropy ≥ 0.5"`. The implemented assertion (`audit_assertions.rs:1921`) uses a hardcoded threshold of `0.25`. While the smoke run's entropy of 0.811 passes both, the code enforces the weaker of the two values. This discrepancy must be resolved: either the assertion should be updated to enforce 0.5, or the charter must be formally amended to accept 0.25 as the ship gate. Shipping with this ambiguity sets a poor precedent.

### Q8 (test coverage)

**Verdict**: **VETO**

The commit adds over 100 lines of non-trivial logic to `evaluator.rs` (diff lines ~1240-1450) and a new, stateful assertion (`audit_assertions.rs:1842-1933`) **with zero new unit tests**. The workspace test count remains unchanged at 915/0/150.

The new `evaluator.rs` logic includes env-var parsing, looping, stateful fallback logic (`produced_worktx_ids`), multiple fallible calls to CAS and the sequencer, and construction of signed transactions. The new assertion `id=43` involves complex logic for grouping, counting, and floating-point entropy calculation. Relying solely on a single, coarse-grained smoke test for correctness is unacceptable for a Class 3 change. This is a critical failure of engineering discipline.

### Q9 (CAS bloat)

**Verdict**: **PASS**

The creation of 8 CAS objects (4 `proposal_artifact`, 4 `ProposalTelemetry`) for 4 `WorkTx`s is acceptable. The cost to create these objects is gated by the stake required for the `WorkTx` itself. This provides an economic brake on spam/bloat attacks. The telemetry data is valuable for audit and is a designed feature of the system. The overhead is not excessive.

### Q10 (audit assertion id=43 Shannon entropy formula)

**Verdict**: **PASS**

The implementation in `audit_assertions.rs` (lines ~1908-1916) is correct. It uses the standard Shannon entropy formula `H(X) = -Σ p(x) log₂(p(x))`. It correctly treats `None` (ROOT) as a distinct category from `Some(tx_id)`, which is essential for measuring topological diversity. Weighting by count (i.e., each proposal is one event) is the standard and appropriate method for this calculation, as opposed to weighting by stake, which would measure a different, economic property.

### Q11 (FC-trace claims)

**Verdict**: **PASS**

The flowchart trace claims in the commit message are justifiable.
-   **FC1-N36 (state-transition kernel)**: The hook drives the submission and admission of 4 `WorkTx`s, directly causing state transitions.
-   **FC2-N31 (signal)**: The resulting chain contains the `parent_tx` graph within `ProposalTelemetry` objects, which constitutes the intended signal.
-   **FC2-N29 (witness)**: The hook calls `boltzmann_select_parent_v2` (`evaluator.rs` diff line ~1312), providing a runtime exercise of the code covered by the witness test in `tests/fc_alignment_conformance.rs`. This aligns with the project's use of the "witness" tag.

### Q12 (regression risk)

**Verdict**: **CHALLENGE**

The hook consumes `100,000 μC` from `Agent_user_0`'s preseeded balance before the main LLM swarm begins. This creates a **fragile, implicit dependency** on test execution order and initial state. While this may not break the dedicated `.2.4` smoke run, it poses a significant risk to the planned `.2.6` combined run, where other hooks or tests might implicitly assume `Agent_user_0` has its full 10M μC preseed. Any test that depends on an exact balance or performs calculations based on it will fail. This pattern of mutating shared fixture state is a known source of test flakiness and should be avoided or explicitly managed.