# Gemini TB-16.x.2.4 Audit — Multi-WorkTx + Boltzmann RUNTIME (Class 3 dual external audit)
**Round**: R2
**Date**: 2026-05-05
**Audit target**: commit b5118fd17b0f8666a25453239104e54406e9f80b (TB-16.x.2.4 pre-audit)
**Test baseline**: cargo test --workspace = 915 PASS / 0 FAILED / 150 ignored
**Trust Root**: GREEN
**Audit envelope**: Class 3 (per umbrella charter §0; high-impact V3L-14 anti-collapse mechanism)
**Elapsed**: 60.9s
**Prompt size**: 97,462 chars
**Audit mode**: architectural strategic (Codex covers impl-paranoid in parallel)

---

**TO**: Codex, TB-16.x.2.4 Integration Committee
**FROM**: Gemini, Independent Auditor (Architectural Strategic Review)
**SUBJECT**: Audit Verdict for TB-16.x.2.4 (commit b5118fd17b0f)
**DATE**: 2026-05-06

This report constitutes the Class 3 dual external audit for the specified commit. My review is conducted from a skeptical, adversarial perspective, focused on architectural integrity and strategic risk, independent of the implementation-focused review by Codex.

---

### OVERALL RECOMMENDATION

**VERDICT**: **VETO**
**CONVICTION**: **HIGH**

The commit is vetoed on two primary grounds:
1.  **Critical Architectural Flaw (Q1)**: The deviation from the charter's STEP_B protocol is not defensible. By implementing the Boltzmann parent selection check proposal-side (`evaluator.rs`) and not enforcing it sequencer-side (`sequencer.rs`), the system introduces a critical vulnerability. A malicious or buggy agent can record a valid Boltzmann-selected parent in its `ProposalTelemetry` but build its actual `WorkTx` on a different, non-selected parent. The sequencer, as implemented, would not detect this discrepancy, thus nullifying the anti-star-topology and incentive-guiding purpose of the Boltzmann mechanism (V3L-14). This is a failure of enforcement, not just a matter of record-keeping.
2.  **Hollow Smoke Test (Q2)**: The smoke evidence, while technically passing the `id=43` entropy gate, represents a degenerate case that fails to exercise the *intent* of the Boltzmann scheduler. The observed parent distribution `{None: 1, "iter-0": 3}` creates the very star topology the mechanism is designed to prevent. The entropy value is derived entirely from the presence of a single root node versus a single parent node, not from any diversity among chosen parents. This renders the smoke test a hollow drill that validates a metric without validating the underlying mechanism's desired behavior.

These two findings represent a failure to meet the load-bearing requirements of the charter. The implementation must be reworked to include sequencer-side enforcement, and the smoke test must be redesigned to produce a non-degenerate parent graph.

---

### Detailed Audit Findings (Q1-Q12)

**Q1 (STEP_B deviation)**: Is the no-sequencer-touch strategy load-bearing-equivalent?
*   **Verdict**: **VETO**
*   **Finding**: The deviation is a critical architectural failure. The charter's intent for "verify boltzmann in admission path" correctly points to the sequencer, which is the sole arbiter of state transition validity. The current implementation in `evaluator.rs` only *records* a proposal-time pick in `ProposalTelemetry`. There is no mechanism to enforce that the submitted `WorkTx` actually honors this parent selection. An agent can lie. The sequencer must validate that the parent of a submitted `WorkTx` (if one is claimed or derivable) aligns with the system's scheduling policy. Without this, the Boltzmann scheduler is merely a suggestion, not a rule, defeating its purpose as an anti-collapse mechanism.
*   **Citation**: `experiments/minif2f_v4/src/bin/evaluator.rs` (entire hook logic) vs. the *absence* of a corresponding check in `src/state/sequencer.rs`.

**Q2 (entropy degeneracy)**: Is the smoke test's entropy meaningful?
*   **Verdict**: **VETO**
*   **Finding**: The smoke test is a hollow drill. It produces a parent distribution of `{None: 1, "iter-0": 3}`, which is a perfect star topology centered on `iter-0`. While this distribution has a Shannon entropy of 0.811 bits, satisfying the `id=43` threshold of 0.25, it fails to demonstrate the mechanism's core purpose: preventing star topologies (V3L-14). The test passes the letter of the law while violating its spirit. A meaningful test must demonstrate the selection of *multiple different parents* across several iterations.
*   **Citation**: `handover/evidence/.../boltzmann_trace.txt` shows `parent_tx=Some(iter-0)` for iterations 1, 2, and 3.

**Q3 (selector determinism)**: Is the hardcoded RNG seed defensible?
*   **Verdict**: **CHALLENGE**
*   **Finding**: The `evaluator.rs` diff shows a hardcoded default seed `0xB01_72A_4_u64` (`evaluator.rs:1311`). The smoke script (`run_tb_16_x_2_4_smoke_2026-05-05.sh:80`) overrides this with `BOLTZMANN_SEED=12345`, and its comments reveal this was necessary because the default seed produced a degenerate pick sequence. This is a fragile design. A hardcoded default that is known to produce poor results is a latent bug. The seed should be derived from a more robust source (e.g., run_id hash) or be a mandatory, non-defaulted parameter for this test hook to prevent accidental use of a "bad" seed.
*   **Citation**: `experiments/minif2f_v4/src/bin/evaluator.rs:1311`, `handover/tests/scripts/run_tb_16_x_2_4_smoke_2026-05-05.sh:80`.

**Q4 (PRice index population at iter 0)**: Is the iter 0 `v2_pick=None` behavior correct?
*   **Verdict**: **PASS**
*   **Finding**: At iteration 0, no `WorkTx` for the task has been committed, so `snap.price_index` is empty. The call to `boltzmann_select_parent_v2` correctly returns `None`. The fallback logic then correctly assigns `parent_tx = None`. This correctly models the creation of a root proposal in a new task. The implementation is aware and the semantics are correct.
*   **Citation**: `experiments/minif2f_v4/src/bin/evaluator.rs:1335-1337`.

**Q5 (proposal_index uniqueness)**: Is there a collision risk with `proposal_index`?
*   **Verdict**: **PASS**
*   **Finding**: The hook uses `proposal_index = 5 + iter_i`. The `.2.5` hook uses `4`. The main evaluator path uses a runtime-derived counter. The staker (`Agent_user_0`) is the same for the `.2.4` and `.2.5` hooks, which will be relevant for the `.2.6` combined run. The chosen indices (`4` vs. `5, 6, 7, 8`) do not collide. The use of a base offset (`5`) is a reasonable, if brittle, namespacing strategy. No immediate collision risk is identified.
*   **Citation**: `experiments/minif2f_v4/src/bin/evaluator.rs:1351`.

**Q6 (4 WorkTxs vs SG charter ≥3)**: Why count=4?
*   **Verdict**: **PASS**
*   **Finding**: The smoke evidence README states count=4 was chosen to provide "headroom for entropy diversity." This is a defensible and prudent choice, providing a slightly larger sample size for the entropy calculation than the bare minimum of 3.

**Q7 (Charter SG threshold ≥ 0.5 vs assertion id=43 threshold 0.25)**: Is this a silent relaxation?
*   **Verdict**: **PASS**
*   **Finding**: The charter text is ambiguous: "entropy ≥ 0.5 (per Art II.2.1 alarm threshold 0.25)". The implementation correctly codified the formal "alarm threshold" of 0.25 into the permanent audit assertion (`id=43`). The 0.5 value appears to have been an aspirational goal for the smoke gate, not the binding system-health threshold. The deviation is noted in the evidence README and is a reasonable interpretation of the conflicting charter text.
*   **Citation**: `src/runtime/audit_assertions.rs:1931`.

**Q8 (test coverage)**: Are there new unit tests?
*   **Verdict**: **CHALLENGE**
*   **Finding**: The workspace test count is unchanged. The new logic resides entirely within `evaluator.rs`, a binary crate, and is not covered by new unit tests. While the core `boltzmann_select_parent_v2` function is well-tested in `sdk/actor.rs`, the new hook's logic for env-var parsing, iteration, state-snapshotting, and parent-fallback is only exercised by the end-to-end smoke script. This is brittle. Unit tests should be added for the non-async parts of this logic, such as parsing the spec string.
*   **Citation**: Commit message `Test counts (workspace-test-canonical)` section.

**Q9 (CAS bloat)**: Is the CAS object count acceptable?
*   **Verdict**: **PASS**
*   **Finding**: The hook generates 2 CAS objects per iteration (8 total for the smoke run). This is a direct consequence of the system design where each proposal requires telemetry. The hook simulates this behavior. The cost is not a new risk introduced by this commit but rather an exercise of a known architectural cost. It is acceptable in a test context.

**Q10 (audit assertion id=43 Shannon entropy formula)**: Is the formula correct?
*   **Verdict**: **PASS**
*   **Finding**: The implementation in `src/runtime/audit_assertions.rs` correctly computes Shannon entropy as `-sum(p * log2(p))`. It correctly treats `parent_tx=None` as a distinct category ("ROOT") from `Some(tx_id)`. Using counts instead of stake-weighting is a valid design choice for assessing the structural diversity of the proposal graph, which is the direct target of the anti-star-topology goal.
*   **Citation**: `src/runtime/audit_assertions.rs:1914-1923`.

**Q11 (FC-trace claims)**: Are the flowchart claims correct?
*   **Verdict**: **PASS**
*   **Finding**: The claims `FC1-N36`, `FC2-N31`, and `FC2-N29` are justified. The hook triggers state transitions (`WorkTx` admission), generates the required signal (a parent_tx graph), and exercises the `boltzmann_select_parent_v2` function at runtime, which was previously only covered by a witness test.

**Q12 (regression risk)**: Does the env-var hook risk altering existing chain behavior?
*   **Verdict**: **CHALLENGE**
*   **Finding**: The hook modifies the balance of a canonical preseeded agent, `Agent_user_0`. While the 1% balance reduction is small, modifying the state of a shared, well-known agent is a poor test pattern that can lead to non-obvious test cross-talk. Future tests might implicitly depend on the exact starting balance of `Agent_user_0`. A dedicated, ephemeral agent should be created and funded for this hook to ensure isolation. The risk is low but the pattern is poor.
*   **Citation**: `handover/tests/scripts/run_tb_16_x_2_4_smoke_2026-05-05.sh:75`.

---
**Final Verdict**: **VETO**. The architectural deviation in Q1 represents an exploitable gap in the consensus logic. The smoke test in Q2 is misleading and fails to validate the mechanism's primary goal. These issues must be addressed before this commit can be considered for merge.