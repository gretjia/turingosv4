# Gemini CO1.1.4-pre1 Typed Tx ABI Round-1 Audit Run
- Model: gemini-2.5-pro
- Packet chars: 328684
- Started: 2026-04-28T11:22:05+00:00

---

# Gemini CO1.1.4-pre1 Round-1 Audit

## Q1 Constitutional alignment

**Verdict: PASS.** The ABI demonstrates strong alignment with the constitution, with the critical D-1 divergence (TxStatus elision) being sound under Art 0.2.

-   **Art 0.1 四要素 (Tape / Input-Tape / Q / State):** `TypedTx` is the correct primary input-tape unit. WP § 5.L4 defines L4 as the "typed transition tape," and `TypedTx` is the concrete schema for entries on this tape. This correctly maps the "pencil" (wtool) writing a structured unit to the "paper" (tape).

-   **Art 0.2 Tape Canonical 公理:** The D-1 divergence, which elides `TxStatus` from the wire format, is **CONSISTENT WITH** Art 0.2. The axiom requires that all signals be reconstructible from the tape, not that all runtime state be explicitly stored on the tape. The status of a transaction (`Accepted`, `Rejected`, `Finalized`) is a *derived property* of the state machine (`Q_t`) after replaying the sequence of accepted transactions from the tape. Since `dispatch_transition` is a pure function that computes the next state (including `ClaimsIndex` and `per_agent.last_accepted_tx`), the tape of `TypedTx` payloads is sufficient for full state reconstruction. This is a sophisticated and correct interpretation of the axiom, avoiding the anti-pattern of storing redundant, derivable state on the canonical tape.

-   **Art 0.4 Q_t version-controlled state:** The structure is correct. `LedgerEntry` (CO1.7) is the version-control envelope, akin to a git commit object, containing metadata and a pointer (`tx_payload_cid`) to the content. `TypedTx` is the content itself, akin to a git blob. This cleanly separates the transport/envelope layer from the payload/content layer, a robust architectural pattern.

-   **Anti-Oreo 三层:** The location in `src/state/` is semantically correct. Per Constitution Art 0.4, `Q_t` is the state, and `src/state/` is its implementation anchor. `TypedTx` is the input to the state transition function; it is part of the state *machinery*. It is not a bottom-white tool (like a CAS store or a ledger writer) nor a top-white predicate. It is the object upon which the top-white predicates and bottom-white tools operate to mutate the state.

## Q2 Inv 3 escrow conservation interaction

**Verdict: PASS.** The type system provides valuable safeguards but does not, and is not expected to, replace runtime enforcement of Inv 3. The current two-way distinction is sufficient.

-   The `StakeMicroCoin(MicroCoin)` newtype is a strong, compile-time safeguard against accidentally mixing staked funds with liquid balances in function signatures. This is a good use of the type system to prevent entire classes of bugs. The implementation in `WorkTx.stake`, `VerifyTx.bond`, and `ChallengeTx.stake` is correct.

-   The use of bare `MicroCoin` for `FinalizeRewardTx.reward` is also correct, as this represents a credit to a liquid balance, not a new stake.

-   The type system does **not** enforce Inv 3 at the wire level. It prevents type confusion in code, but the actual conservation logic (e.g., `tx.stake <= q.balances_t[tx.agent_id]`) is, and must be, enforced at runtime by the transition functions (per `STATE_TRANSITION_SPEC § 3 stage 3`). The ABI supports this but does not replace it.

-   A third `EscrowMicroCoin` newtype is unnecessary at this stage. Escrow is an accounting state within `Q_t.economic_state_t.escrows_t`, not a distinct kind of money an agent can hold. The critical distinction is between liquid (`MicroCoin`) and at-risk (`StakeMicroCoin`) funds from an agent's perspective. The current two-type system correctly models this.

## Q3 v4 / v4.1 boundary preservation

**Verdict: CHALLENGE.** The ABI is additive-compatible for `TypedTx` variants, but this property is fragile and not sufficiently specified. The lack of explicit discriminants on `TransitionError` is a direct threat to forward compatibility.

-   Adding `TypedTx::MetaTx(MetaTx)` in v4.1 is bincode-additive. However, the spec (§12.3) only briefly mentions "additive variants." It **must explicitly commit** to a rule: "New variants may only be appended. Existing variants must never be reordered or removed." This is a constitutional-level commitment for a wire format.

-   The addition of a new variant will break any digest fixtures that serialize the `TypedTx` enum, as the variant index is part of the encoding. This is an unavoidable cost of evolution and is acceptable, provided the rule above is followed.

-   **CRITICAL FLAW:** The `TransitionError` enum in `typed_tx.rs` does **not** have explicit discriminants (e.g., `ClaimNotFound = 0, ...`). If a v4.1 `MetaTx` requires a new error variant, inserting it into the middle of the enum would silently break the wire format for all subsequent variants. This violates the principle of additive-only changes. All enums that are part of the canonical ABI must use `#[repr(u8)]` and have explicit, stable discriminants, just as `TxKind` does.

## Q4 WP § 5.L4 conformance + envelope/payload split

**Verdict: CHALLENGE.** The architecture is consistent with the white paper, but its reliance on a not-yet-implemented CAS persistence layer (CO1.4-extra) creates a direct, unresolved conflict with Art 0.2 (Tape Canonicality) for cold-replay scenarios.

-   The 2-step indirection (L4 `LedgerEntry` → L3 CAS → `TypedTx` payload) is consistent with WP § 5.L3 and § 5.L4. The L4 tape contains the *commitment* (`tx_payload_cid`) to the payload. This is not an architectural sin like a "rejection sidecar"; it is a standard and sound content-addressing pattern. The tape is canonical because it contains the immutable, verifiable reference to the exact payload that was processed.

-   **CRITICAL FLAW:** The spec for CO1.7 (§0) and this atom's context correctly identify that if CAS bytes are lost, the system enters a state of partial verifiability. The L4 `LedgerEntry` chain can be verified (signatures over CIDs are intact), but the state transitions cannot be replayed, as the payloads are gone. This **violates Art 0.2 (Tape Canonical 公理)**, which demands that "all signals must be reconstructible from the tape." A tape of pointers to missing data is not sufficient for reconstruction. While the fix (CAS index persistence) belongs to CO1.4-extra, this atom cannot PASS without a concrete, committed, and immediate plan to resolve this constitutional violation. The current state introduces a dangerous "trust mode" ambiguity.

## Q5 Art 0.2 reconstructibility + TxStatus elision

**Verdict: PASS.** The design is sound. The elision of `TxStatus` (D-1) is a correct architectural choice that upholds, rather than violates, Art 0.2.

-   The state components `q_t.q_t.agents[id].last_accepted_tx` and `ClaimsIndex` **can be fully derived** by replaying the tape of accepted `LedgerEntry` payloads through the `dispatch_transition` function. The tape contains the sequence of valid state changes; the `Q_t` state is the result of applying them. This is the essence of a state machine. The design correctly places derived state in `Q_t` and canonical inputs on the tape.

-   Rejection status is not directly observable on the L4 tape, which only contains accepted transactions. Per `STATE_TRANSITION_SPEC § 1.4`, rejection summaries are stamped onto the *next accepted* `WorkTx` or the final `TerminalSummaryTx`. Therefore, the L4 tape *does* contain sufficient information to reconstruct the history of rejections, albeit in a summarized, batched form. This is consistent with Art 0.2.

-   The progression from `Accepted` to `FinalizedSlash` is fully captured. The L4 tape would show an accepted `WorkTx` followed by a later accepted `ChallengeTx` targeting it. Replaying this sequence through the state machine correctly updates the `ClaimsIndex` to reflect the slash. The sequence of accepted typed-tx variants is sufficient.

## Q6 FinalizeRewardTx derivation

**Verdict: CHALLENGE.** The derived schema is plausible but contains a significant ambiguity (redundant signature) and a type-level impurity (reusing `TxId` for `claim_id`).

-   The schema should **not** include a `royalty_graph_t` snapshot. The transition function must operate on the `Q_t` passed to it, which contains the canonical `royalty_graph_t`. Including a snapshot would be redundant and violate the single-source-of-truth principle. The current field set is correct in this regard.

-   The reuse of `TxId` for `claim_id` is an implementation detail leak. While claims may be keyed by the `TxId` of the `WorkTx` they correspond to, the ABI should be abstract. A newtype `pub struct ClaimId(pub TxId)` or `pub struct ClaimId(pub String)` would be architecturally cleaner and prevent future confusion if the keying strategy changes.

-   **CRITICAL FLAW:** The `system_signature` field is highly suspect. The `LedgerEntry` (CO1.7) envelope is already signed by the system. This signature covers the `tx_payload_cid`, which is the digest of the `FinalizeRewardTx`. A second system signature inside the payload appears redundant. If it signs something different (e.g., a message from a separate SettlementEngine service), the spec **must** define the canonical pre-image for this signature. As it stands, this is a dangerous ambiguity. The field should likely be **DROPPED**.

## Q7 AgentSignature security model

**Verdict: CHALLENGE.** The use of identical serde adapters for `AgentSignature` and `SystemSignature` creates a type confusion vulnerability at the wire level. The domain separation in the CO1.7 envelope is necessary but not sufficient.

-   The byte-identical wire format for `AgentSignature` and `SystemSignature` is a security risk. An attacker could potentially take a system-signed message, strip the envelope, and replay the payload in a context that expects an agent signature, or vice-versa. While the type system prevents this in safe Rust code, it offers no protection against raw byte manipulation or bugs in deserialization logic.

-   Art 0.2 requires canonical, unambiguous data. This implies that types which are semantically distinct should also be distinct at the wire level if possible, or at minimum, be protected by strong domain separation in the digests they sign over.

-   CO1.7's `LedgerEntrySigningPayload.canonical_digest()` correctly uses a domain separator (`b"turingosv4.ledger_entry_signing.v1"`). This protects the *envelope*. However, the `AgentSignature` in a `WorkTx` signs the canonical digest of the `WorkTx` payload itself. The spec and implementation **do not specify a domain separator** for the agent-signed payloads. This is a critical omission. Every signed payload type (`WorkTx`, `VerifyTx`, `ChallengeTx`) must have its own unique domain separator in its `canonical_digest` pre-image (e.g., `b"turingosv4.work_tx.v1"`).

## Q8 Forward sustainability

**Verdict: PASS.** The current strategy of using an `extensions` map on the envelope (`LedgerEntry`) and additive variants on the payload (`TypedTx`) is a reasonable and standard approach to forward compatibility.

-   The `LedgerEntry` having an `extensions: BTreeMap<String, Vec<u8>>` field is an excellent forward-compatibility hatch for metadata, proofs, or other envelope-level concerns.

-   The `TypedTx` enum relies on additive variants for evolution. This is a valid, albeit more rigid, strategy than an extensions map. It forces protocol upgrades to be explicit new transaction types, which can be desirable for clarity. Adding an `Extension(String, Vec<u8>)` variant to `TypedTx` itself could be a valuable, low-cost addition for future flexibility, allowing for protocol-level signaling without defining a full new transaction type. This is a recommendation, not a failure. The current design is sustainable.

## Q9 Test strategy completeness

**Verdict: CHALLENGE.** The test strategy is incomplete and fails to lock the ABI, which is the primary purpose of this atom.

-   **CRITICAL FLAW:** The golden fixture tests (`golden_*_tx_digest`) asserting only digest *stability* but not the *locked hex value* are unacceptable for a PASS. The entire point of I-CANON-D is to freeze the wire format. A test that passes on the first run with any output is not a test; it's a recording mechanism. For an atom whose purpose is to define a canonical ABI, the golden hex values **must be locked in v1**.

-   The test suite is missing key classes:
    -   **Cross-variant non-collision:** A test should ensure that default-initialized instances of different `TypedTx` variants do not serialize to the same byte string.
    -   **BTreeMap permutation independence:** While bincode should handle this, an explicit test that creates two `WorkTx` with identical data but different `BTreeSet`/`BTreeMap` insertion orders and asserts byte-identical serialization would be a valuable guarantee.
    -   **Zero-value defaults:** Testing the round-trip of `Default::default()` for each struct is a good practice to catch edge cases.

## Q10 **VERDICT**: CHALLENGE

The atom is architecturally sound in its core decisions (D-1 `TxStatus` elision, envelope/payload split) and correctly identifies its purpose. However, it suffers from critical flaws in security (signature domain separation), constitutional alignment (cold-replay violates Art 0.2), and implementation (unlocked golden fixtures, ambiguous fields). These issues prevent a PASS. A VETO is not warranted as the flaws are addressable without a fundamental redesign.

## Top 3 must-fix / risks

1.  **Signature Security & Ambiguity (Q6, Q7):** The `system_signature` in `FinalizeRewardTx` must be justified with a clearly specified canonical digest or be removed. Critically, all agent-signed payloads (`WorkTx`, `VerifyTx`, `ChallengeTx`) **must** have unique domain separators added to their canonical digest pre-images to prevent type confusion attacks. This is a non-negotiable security requirement.
2.  **Incomplete & Unlocked ABI Tests (Q9):** The golden fixture tests **must** be updated to assert against hardcoded, known-good SHA-256 hex strings. The "record-only" phase is insufficient for an ABI-defining atom. The test suite must be expanded to include cross-variant non-collision checks. An unlocked ABI is not an ABI.
3.  **Constitutional Violation on Cold Replay (Q4):** The ABI's reliance on a non-persistent CAS for payload storage creates a direct violation of Art 0.2 (Tape Canonicality) in any cold-restart scenario. While the fix is in CO1.4-extra, this atom's `PASS` must be gated on a concrete and immediate commitment to implementing CO1.4-extra. The strategic risk of shipping a constitutionally-non-compliant tape layer, even temporarily, is too high.

## Conviction

**High.** The identified issues are grounded in core constitutional principles (Art 0.2), fundamental security practices (domain separation), and the primary deliverable of the atom (a locked, tested ABI). The evidence is clear across the provided specifications and implementation.

---
## Usage: prompt=106230 candidates=3561 total=113295 thoughts=3504
- Finished: 2026-04-28T11:23:14+00:00
