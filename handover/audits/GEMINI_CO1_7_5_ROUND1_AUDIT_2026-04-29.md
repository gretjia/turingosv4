# Gemini CO1.7.5 transition_bodies_and_runtime_wiring Round-1 Audit Run
- Model: gemini-2.5-pro
- Packet chars: 462259
- Started: 2026-04-29T00:58:27+00:00
- HEAD: 334111af8866d84977957163be3ce6517808b195

---

# Gemini CO1.7.5 Round-1 Audit

## Q1 Constitutional alignment

**Verdict**: PASS

CO1.7.5 demonstrates strong constitutional alignment.

-   **Art 0.1 四要素映射 (Tape / Input / Q / State)**: The design correctly maps the four elements. The transition bodies are the `Strict discipline` operating on `Q_t` (the `Paper`). The Sequencer's `apply_one` machinery acts as the `Pencil`, appending an accepted `LedgerEntry` to the `Tape`. The "rubber" function is correctly implemented by the principle that rejected transactions do not advance `Q_t` (`Q_{t+1} = Q_t`), as specified in `Sequencer::apply_one`'s early return on `Err`.

-   **Art 0.2 Tape Canonical 公理**: The design upholds this axiom. The `Sequencer::apply_one` pseudocode in the inherited `CO1.7` spec (§ 3) and the shipped `sequencer.rs` (lines 310-315) show that the pure transition function is called *before* any commit action. If it returns an `Err`, the function returns early, and no `LedgerEntry` is committed. This ensures rejected transactions do not advance the ledger state, which holds for all 7 transition bodies as they are all routed through `dispatch_transition`. The `LedgerEntry.system_signature` attests to the sequencer-stamped semantics of the *accepted* transition, fulfilling the attestation requirement.

-   **Art 0.4 Q_t version-controlled**: The design correctly implements the Art 0.4 binding. The spec's D2 deliverable (§ 1 D2) explicitly sets `q.head_t = state::q_state::NodeId(commit_oid_hex)`. This is the direct implementation of Art 0.4's `HEAD_t = git commit SHA` principle. The spec correctly notes in § 0.3.1 that this supersedes the `STATE v1.4 § 3` `NodeId::from_state_root(...)` suggestion, citing the higher authority of `CO1.7 K3 v1.2 § 5`. This demonstrates correct interpretation of the constitutional hierarchy: the specific, audited implementation detail in a later spec (CO1.7) refines the general principle from an earlier one (STATE v1.4), and both align with the highest law (Constitution Art 0.4).

-   **Anti-Oreo 三层**: The architecture is correctly layered. The 7 transition bodies are pure functions that perform state mutation logic, fitting squarely in the `middle-black` layer. The `LedgerWriter` trait and its `Git2LedgerWriter` implementation, which persist the ledger, are in the `bottom-white` layer (`src/bottom_white/ledger/`). The Sequencer acts as the orchestrator between these layers.

## Q2 Authority chain on STATE supersessions

**Verdict**: CHALLENGE

The spec correctly identifies and carries forward two necessary supersessions from prior, audited specs (`CO1.7-K3-v1.2` and `CO1.1.4-pre1`). The legitimacy of a downstream, more specific, audited spec superseding an upstream, more general one is a valid and necessary principle for architectural evolution. The resolutions themselves are sound.

However, the spec's posture on this institutional process is too passive. § 0.3 states, "...whether STATE_TRANSITION_SPEC needs a v1.5 housekeeping commit is a separate decision for the STATE spec curator." This is ducking responsibility.

A downstream spec that creates a divergence has an institutional obligation to formally notify the upstream curator and trigger the reconciliation process. Simply documenting the divergence and delegating the fix creates institutional drift and documentation debt. The "conservative-wins" principle dictates that when downstream and upstream disagree, the more recently audited (downstream) spec dominates, but it also incurs a responsibility to resolve the discrepancy, not just note it.

**This constitutes a process-level challenge**: The spec must be amended to state that as part of the CO1.7.5 atom's closure, a formal change request or issue will be filed against the `STATE_TRANSITION_SPEC` to align its pseudocode with the as-implemented reality of `head_t` mutation and `SignalKind` shape.

## Q3 Wave 6 #1 closure correctness

**Verdict**: PASS

CO1.7.5 correctly closes Wave 6 #1.

-   **Exhaustiveness**: The four deliverables (D1-D4) comprehensively cover the remaining work for the L4 transition ledger family. D1 implements the core logic, D2 wires the final constitutional anchor (`head_t`), D3 integrates it into the runtime, and D4 enables the end-to-end verification tests. There are no apparent gaps or silently deferred items that belong in L4. The spec correctly identifies L5 (materializer) and L6 (signal indices) as subsequent waves.

-   **Forward Affordances**: The design provides sufficient affordances for Wave 6 #2 and #3.
    -   **CO1.8 (L5 materializer)**: The spec (§ 2, item 6) confirms that transition bodies will use the existing `q_next.economic_state_t.derive_state_root()` accessor. This provides a clean, single point of substitution for CO1.8's real merkleized materializer, avoiding hard-coding.
    -   **CO1.9 (L6 signal indices)**: The spec (§ 0.3.2) correctly uses the minimal 4-variant `SignalKind` frozen by `CO1.1.4-pre1`. As established in `CO1.1.4-pre1 § 7.2`, adding new variants to a Rust enum is a non-breaking change to the wire format of existing variants. This provides a clear and safe extension path for CO1.9 without requiring an ABI migration.

## Q4 Combined STEP_B ceremony

**Verdict**: PASS

The proposal for a combined STEP_B ceremony is strategically sound and defensible.

-   **Tradeoff Defensibility**: The spec's justification (§ 1 D3) is compelling: "the bus.rs forwarder cannot be reasoned about without the kernel.rs field it forwards through." Performing separate ceremonies would test two meaningless, incomplete changes. Combining them ensures the A/B test is performed on a single, coherent, "minimum sufficient" unit of functionality. The increased blast radius is acceptable given the small, additive nature of the changes.

-   **`STEP_B_PROTOCOL.md` Phase 0 Interpretation**: The spec's invocation of the "minimum sufficient version" criterion is correct and binding. It applies the principle not just to the necessity of the change, but to the *atomicity* of the change being tested. This is a mature interpretation of the protocol's intent.

-   **Sequencer Ownership & Abstraction**: Placing the `Sequencer` instance in `Kernel` is the correct architectural choice. The Kernel is the heart of the runtime's state machine, already holding `Tape` and `NodeId` from the legacy system. The Sequencer is the modern equivalent of this core machinery. The state itself (`Q_t`) is not stored *in* the Kernel but is managed *by* the Sequencer, which is owned by the Kernel. This respects the Anti-Oreo layering by keeping the state-transition *driver* in a core runtime component, distinct from the pure state-mutation logic (middle-black) and the ledger persistence (bottom-white).

## Q5 SignalKind minimization forward-compat

**Verdict**: PASS

The v1 minimization of `SignalKind` is a safe deferral, not a deferred hazard.

-   **Art 0.2 Reconstructibility**: Emitting `SignalKind::Empty` for 4 of 7 transition bodies does **not** cause observable-state loss that breaks reconstructibility. Art 0.2 requires that all signals be reconstructible from the `tape`. The tape contains the full `LedgerEntry`, which includes the `tx_payload_cid`, pointing to the complete `TypedTx`. The logic for deriving richer signals (like reputation or price deltas) is deterministic and based on the contents of the `TypedTx`. Therefore, L5/L6 can re-run this deterministic logic during replay to reconstruct the full signal stream. The `SignalKind` on the `LedgerEntry` is a summary, not the sole source of truth.

-   **ABI Lock**: As analyzed in Q3, adding new variants to the `SignalKind` enum in CO1.9 will not break the `CO1.1.4-pre1` ABI lock. The wire format of the existing four variants remains stable. It will be a source-level breaking change for consumers with non-exhaustive `match` statements, which is a desirable forcing function to ensure new signals are handled.

-   **Safe Deferral**: This is a classic example of incremental delivery. It ships a functional L4 now, with a clear and safe path for adding L6 signal richness later. This reduces the complexity and risk of the CO1.7.5 atom.

## Q6 Hygiene OBS quality

**Verdict**: PASS

The handling of the path drift and the resulting OBS is appropriate and demonstrates good institutional hygiene.

-   **Inline Fix Appropriateness**: `CLAUDE.md` and `STEP_B_PROTOCOL.md` are project instructions, not the constitution. The strict "OBS not in-place" rule from the Alignment Standard applies to constitutional documents to preserve their amendment history. For operational documents, keeping them accurate and avoiding confusion for developers and auditors is paramount. Fixing the path drift inline was the correct, pragmatic choice to prevent a guaranteed, low-value CHALLENGE.

-   **OBS Honesty**: The OBS is sufficiently honest. It clearly states the finding, the fix, and explicitly scopes what was **not** investigated (bisecting the commit). This is the correct level of detail, providing a clear record without wasting resources on non-critical historical forensics.

## Q7 Forward sustainability for Wave 6 #2/#3

**Verdict**: PASS

The spec is well-designed for forward sustainability. This question overlaps significantly with Q3 and Q5, and the verdict is consistent.

-   **CO1.8 `materializer::apply` Substitution**: Yes, the spec reserves a clean substitution point. By relying on the existing `derive_state_root()` method, it avoids entangling the transition body logic with the details of state root computation, making the CO1.8 refactor a clean swap.

-   **CO1.9 `SignalKind` Extension**: Yes, the design allows for additive, non-ABI-breaking extensions to `SignalKind`, providing a clear path for CO1.9.

-   **Avoidance of Hard-Coding**: The transition bodies are specified to be pure translations of the `STATE` spec pseudocode. They operate on the abstract `QState` and return a new `QState`. This insulates them from the specifics of downstream plumbing like materializers or signal indexers, which are handled by the Sequencer and future components.

## Q8 Q1 head_commit_oid_hex default impl recommendation

**Vote**: `unimplemented!()`

**Rationale**: The `head_t` field is a direct implementation of the constitutional anchor `HEAD_t` from Art 0.4. Its integrity and liveness are of paramount importance.

-   `default { None }` introduces a significant risk of silent failure. A developer implementing a new `LedgerWriter` or a test using a mock could forget to override the method. The result would be a silently stagnating `head_t`, a subtle but severe violation of the system's version-control model. Debugging this would be difficult as it's a failure of omission.

-   `default { unimplemented!() }` acts as a compile-time forcing function. It makes it impossible to implement the `LedgerWriter` trait without explicitly considering the `head_commit_oid_hex` method. The implementer is forced to make a conscious choice: either provide a real implementation (like `Git2LedgerWriter`) or explicitly return `None` (like `InMemoryLedgerWriter`).

Per the project's "conservative wins" principle, preventing a silent failure mode for a constitutional anchor field is the top priority. The minor ergonomic cost of forcing an explicit implementation is a small price to pay for this critical safety guarantee.

## Q9 **VERDICT**: CHALLENGE

The CO1.7.5 spec is architecturally sound, constitutionally aligned, and demonstrates a mature understanding of the project's principles. The design correctly closes Wave 6 #1 while providing clear affordances for future waves.

However, the verdict is a **CHALLENGE** based on two key points that require remediation before the spec can be considered complete. These are not flaws in the technical design of the transition bodies, but rather in the institutional process and conservative default principles that govern the system's long-term health.

## Top 3 must-fix / risks

1.  **Formalize Upstream Spec Reconciliation Process (from Q2)**: The spec must be amended to take responsibility for initiating the reconciliation of the upstream `STATE_TRANSITION_SPEC`. The passive delegation to the "STATE curator" is insufficient. The spec must commit to filing a formal change request or issue to align the `STATE` spec's pseudocode with the as-implemented reality as part of this atom's work.
2.  **Adopt Conservative Default for `head_commit_oid_hex` (from Q8)**: The default implementation for the new `LedgerWriter::head_commit_oid_hex` trait method must be changed from `default { None }` to `default { unimplemented!() }`. This is a critical safety measure to prevent silent stagnation of the constitutional `head_t` anchor.
3.  **Clarify Supersession Authority and Responsibility (from Q2)**: The language in § 0.3 should be strengthened. Instead of merely documenting a carry-forward, it should assert the principle that a later, more specific, audited spec (like CO1.7) legitimately supersedes an earlier one (like STATE v1.4), while also acknowledging the institutional debt to file a housekeeping patch against the superseded document. This provides clarity on the project's governance of its own specifications.

## Conviction

High

---
## Usage: prompt=144441 candidates=3107 total=150915 thoughts=3367
- Finished: 2026-04-29T00:59:24+00:00
