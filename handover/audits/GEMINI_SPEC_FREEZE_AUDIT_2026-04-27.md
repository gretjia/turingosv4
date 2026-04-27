# Gemini Spec Freeze Audit Run
- Model: gemini-2.5-pro
- Packet chars: 226991
- Started: 2026-04-27T07:55:51+00:00

---

# Gemini Spec Freeze Audit (CO1.SPEC.0.5)

## Q1. Spec completeness for ship gate
**Verdict: [PASS]**

The v1.1 spec substantively addresses my prior CHALLENGE regarding incomplete pseudocode. The addition of `VerifyTx`, `ChallengeTx`, `ReuseTx`, `finalize_reward`, and `terminal_summary` transition functions provides the necessary logical scaffolding. The explicit classification of hidden inputs (§ 2) and the promotion of environment variables to typed `QState` fields is a significant improvement in rigor and directly responds to Codex's concerns. The spec now presents a coherent, if not yet exhaustive, model for the core work lifecycle. It is sufficiently complete to serve as the binding contract for the STEP_B implementation comparison. Remaining gaps are primarily in lifecycle edge cases (see Q2/Q3), not the core transitions themselves.

## Q2. 22 invariants vs whole-system requirements
**Verdict: [CHALLENGE]**

The 22 specified invariants show excellent coverage of the direct state transition mechanism. However, when cross-referenced against the whole-system requirements, critical gaps emerge.

-   **12 economic invariants (WP § 18):** Covered. The walk-through document convincingly demonstrates how the transition invariants map to and enforce the economic invariants for the scenarios covered. `I-PROV` (provisional reward) directly implements `Inv 7`, and `I-ROYALTY-CAP` helps bound `Inv 8` attribution.
-   **24 V tape canonical violations (Const Art 0.2):** Covered, via reinterpretation. The spec's approach—using `RejectedAttemptSummary` and `TerminalSummaryTx` instead of storing raw failed proposals—is a valid and arguably superior implementation of Art 0.2's intent ("failure-on-tape"). `I-NOSIDECAR`, `I-RETRY`, and `I-TERMINAL` combine to enforce this.
-   **Constitution Art I-V philosophical principles:** Covered. The spec's structure aligns well: Art I (Quantization) is embodied in the `SignalBundle`; Art III (Shielding) in `tape_view_t`; Art V (Go Meta) in the `MetaTx` stub.
-   **Phase 3 prep deliverables (META_TX_SCHEMA / meta_validator):** Covered. The spec correctly stubs `MetaTx` and the cross-referenced documents (`META_TX_SCHEMA_v1`, `MetaTransitionInterface`) provide a robust, auditable deferral path to v4.1.

**Missing Invariants:**
The primary gap is in lifecycle management beyond the "happy path" or "challenge path".
1.  **I-STAKE-RETURN:** An invariant is needed to guarantee that a Solver's stake is returned upon successful, unchallenged finalization of their work. The `finalize_reward` pseudocode credits the reward but does not explicitly show the stake being unlocked and returned.
2.  **I-BOUNTY-REFUND:** There is no invariant or mechanism for returning a TaskMarket's bounty from escrow if the task expires without any accepted solution. This is a critical economic conservation law.
3.  **I-AGENT-INIT:** There is no invariant governing the initial state of an agent (e.g., initial reputation). The system assumes agents appear with pre-existing state.
4.  **I-PREDICATE-BOOTSTRAP:** While `MetaTx` is deferred, an invariant is needed to describe how the *very first* set of predicates enters the (initially empty) registry in v4. The current model implies a v4.1 dependency to make v4 functional, which is a bootstrap paradox. The offline `cp` workflow needs to be acknowledged as the canonical v4 path.

## Q3. 5 transition functions sufficiency
**Verdict: [CHALLENGE]**

The set of 5 core transition functions (`work`, `verify`, `challenge`, `reuse`, `finalize_reward`) plus the terminal summary handler is a strong foundation but is insufficient for the full v4 scope. It omits key economic state changes:

-   **Stake withdrawal / refund:** As noted in Q2, the mechanism for returning a Solver's stake after the challenge window closes successfully is absent from the `finalize_reward` pseudocode. This is a critical omission.
-   **Bounty refund:** The lifecycle of a task itself is incomplete. A function is needed to handle task expiry and refund the bounty from escrow to the task creator.
-   **Agent reputation explicit init:** The system lacks a transition for onboarding a new agent and setting their initial reputation. This is currently an unhandled genesis event for each agent.
-   **Predicate registry update:** This is explicitly deferred via `MetaTx` to v4.1 runtime, which is an acceptable and well-documented architectural choice. The v4 offline `meta_validator` library provides the necessary stopgap.

The set is sufficient for the *work submission and review* lifecycle, but not the *task and agent* lifecycle.

## Q4. Cross-spec consistency
**Verdict: [PASS]**

The references to external specs are coherent and demonstrate strong architectural integrity.
-   `GENESIS_MINIMAL_WITH_ANCHOR_v1`: The `initial_predicate_registry_root` in genesis maps directly to the `predicate_registry_root_t` in `QState`. The anchoring of the `amendment_predicate` provides the necessary trust root for the deferred `MetaTx` system.
-   `SYSTEM_KEYPAIR_SECURITY_v1`: The `system_signature` on `TerminalSummaryTx` is fully specified by this security document, closing a critical trust boundary VETO'd in the v3.2 review.
-   `META_TX_SCHEMA_v1` / `MetaTransitionInterface`: The deferral of runtime meta-transitions is handled cleanly. The main spec defines the stub, the schema spec defines the type, and the interface spec defines the v4.1 plug-in contract. This is an exemplary handling of a phased implementation.

A mental type-check of key fields (`Hash`, `TxId`, `AgentId`) across the specs reveals no inconsistencies.

## Q5. Determinism guarantees
**Verdict: [PASS]**

The spec's commitment to determinism is robust and well-articulated. The promise of "byte-identical output" is backed by concrete, machine-checkable rules:
-   Elimination of `SystemTime::now()`, `std::env::var`, and other side effects is correctly identified as paramount (`I-NOSIDE`, `I-NOENV`).
-   Mandating `BTreeMap` over `HashMap` (`I-BTREE`) correctly addresses iteration order non-determinism.
-   Using a `MicroCoin(i64)` newtype (`I-MICROCOIN`) correctly avoids IEEE 754 floating-point non-determinism for all economic calculations.

The remaining sleeper risks (e.g., non-determinism within a WASM-sandboxed predicate, toolchain differences) are implementation-level concerns that fall under Codex's mandate. The spec itself has taken all reasonable strategic measures.

## Q6. TaskMarket config defaults
**Verdict: [PASS]**

The proposed defaults are game-theoretically sound for the initial v4 launch and the planned Phase C experiments.
-   `false_challenge=0`: A neutral default that encourages challenges without penalizing failed-but-honest attempts. This maximizes information discovery in an early-stage market.
-   `verifier_bond=Return`: Correctly incentivizes verification. Slashing good-faith verifiers would create a chilling effect and starve the system of a crucial validation layer.
-   `royalty_cap=0.10`: A reasonable cap that rewards tool builders while preserving the primary incentive for solvers. It prevents parasitic or extractive strategies.
-   `quorum=1`: A pragmatic choice for v4, simplifying the logic while deferring the complexity of M-of-N aggregation rules.

These defaults prioritize participation, safety, and simplicity, which is the correct posture for a new system.

## Q7. Walk-through gap closure
**Verdict: [PASS]**

v1.1 successfully closes all 4 gaps identified in `SPEC_WALKTHROUGH_v1`. The resolution method—instituting a machine-checkable default while making the parameter configurable per `TaskMarket`—is the ideal architectural pattern. It resolves the spec's ambiguity without over-constraining the system's future experimental flexibility. The spec does not merely defer the problem; it provides the mechanism (`TaskMarket.config`) for addressing it at runtime.

## Q8. STEP_B feasibility prediction
**Verdict: [PASS]**

The prediction of surfacing 5-10 spec ambiguities during the parallel STEP_B implementation is realistic and demonstrates mature foresight. No spec of this complexity is perfect. The provided pseudocode, while much improved, still abstracts away details in functions like `materializer::apply` and `SettlementEngine::finalize`. These are the most likely sources of ambiguity.

An audit and resolution cost of 5-20 person-days is a reasonable budget for this risk. The spec is sufficiently detailed to make STEP_B a valuable and feasible process. The cost of resolving these ambiguities at the spec/implementation stage is orders of magnitude lower than fixing them post-launch. The plan is sound.

## Q9. Holistic verdict on v1.1 freeze
-   **STATE_TRANSITION_SPEC v1.1:** **[CHALLENGE]**
-   **Ready for CO P1 launch:** **[NEEDS-FIX]**
-   **Should v1.2 patches happen first?** **Yes.**

The spec is a massive step forward and is 90% of the way there. However, the lifecycle gaps identified in Q2 and Q3 (stake return, bounty refund) are not minor omissions. They represent fundamental holes in the economic conservation laws of the system. Freezing the spec without them would guarantee that the initial implementations of the economic modules (CO P2) will be built on an incomplete foundation, necessitating expensive refactors. A v1.2 patch is mandatory before the freeze.

## Q10. What you'd want Codex to check next
As the strategic reviewer, I defer to Codex for code-grounded validation. I specifically recommend they focus their next audit cycle on:
1.  **Canonical Digests:** The exact byte-level serialization format for all signed transaction types (`WorkTx`, `VerifyTx`, `ChallengeTx`, `TerminalSummaryTx`). This must be specified to the byte to prevent signature validation mismatches between the two STEP_B implementations.
2.  **Predicate Sandbox Implementation:** A deep audit of the WASM runtime or other sandboxing mechanism. Verify its resource limiting (CPU, memory, I/O), determinism guarantees, and the security of the host-guest boundary.
3.  **State Materialization Logic:** An audit of the `materializer::apply` logic. This is a complex, stateful component where bugs or non-determinism can silently corrupt `state_root_t`, breaking the entire system's integrity.
4.  **Dependency Tree Audit:** A recursive audit of all third-party dependencies for sources of non-determinism, insecure code, or undesirable side-effects (e.g., network access, filesystem access) that might bypass the spec's `I-NOSIDE` intent.

---

## Cross-cutting concerns

-   **Lifecycle Completeness:** The most significant cross-cutting issue is the incompleteness of the economic lifecycle. The spec excels at the work-submit-verify-challenge flow but neglects the agent-join and task-expire/timeout flows. This must be addressed.
-   **Excellent Deferral Strategy:** The handling of `MetaTx` is a model for how to manage phased rollouts. By defining the schema, the validator, and the interface trait, v4 provides a concrete, auditable "promise" to v4.1 without pulling the full implementation forward.
-   **Determinism as a First-Class Citizen:** The spec's rigorous focus on eliminating non-determinism at the architectural level is commendable and essential for a verifiable system.

## Holistic verdict: CHALLENGE

The `STATE_TRANSITION_SPEC v1.1` is a high-quality, rigorous document that is very close to readiness. However, the identified gaps in the economic lifecycle are too fundamental to approve a final freeze. The system must be able to correctly handle stake returns and bounty refunds to be considered economically sound.

## Top-3 must-fix before CO P1 launch

1.  **Add Stake/Bounty Refund Logic:** Amend the spec (likely creating v1.2) with new transition functions or extensions to existing ones (`finalize_reward`) to handle the return of a Solver's stake on success and the refund of a Task's bounty on expiry. Add corresponding invariants (`I-STAKE-RETURN`, `I-BOUNTY-REFUND`).
2.  **Specify Agent/Reputation Initialization:** Add a section or transition function that defines how a new agent joins the system and what their initial reputation is. This closes the agent lifecycle loop.
3.  **Clarify v4 Predicate Bootstrap Path:** Add a note to the spec explicitly stating that in v4, the initial predicate set is populated via the offline, human-in-the-loop `cp` workflow and `MetaProposalDraft` validation, as the runtime `MetaTx` path is a v4.1 feature. This resolves the bootstrap paradox.

## Recommendation: CO P1 NEEDS-FIX

The CO Phase 1 launch should be **contingent** on a `v1.2` patch to `STATE_TRANSITION_SPEC` that addresses the three "must-fix" items above. Once that patch is reviewed and accepted, the spec can be frozen, and the launch is a **GO**. Proceeding before these fixes are in place would introduce significant and unnecessary architectural debt and risk.

---
## Usage: prompt=78201 candidates=3010 total=85145 thoughts=3934
- Finished: 2026-04-27T07:57:12+00:00
