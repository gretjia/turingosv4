# Gemini TB-15 Ship Audit — Lamarckian Autopsy + Markov EvidenceCapsule (Class 2 retroactive dual audit)
**Round**: R3
**Date**: 2026-05-04
**Test baseline**: cargo test --workspace = 878 PASS / 0 FAILED / 150 ignored (TB-15 ship commit 2337381)
**Halt-trigger battery**: 6/6 GREEN (tests/tb_15_halt_triggers.rs)
**Trust Root**: GREEN (6 rehashes propagated correctly)
**Original audit envelope**: Class 2 self-audit per charter §4 (no Codex/Gemini at ship)
**Retroactive dual audit**: requested by user 2026-05-04 to verify Class 2 envelope held
**Elapsed**: 42.5s
**Prompt size**: 817,975 chars
**Audit mode**: architectural strategic (Codex covers impl-paranoid in parallel)

---

# Gemini TB-15 R3 Ship Audit — POST R2-VETO-CLOSURE

## Executive Summary

I have reviewed the R3 remediations applied to address the Codex R2 VETO. The core finding of the VETO — a critical bug where the on-CAS content hash did not match the published `capsule_id`, breaking CAS resolvability and violating SG-15.3 — has been comprehensively fixed. The new writer pattern correctly ensures that `capsule_id == sha256(stored_bytes)`. The addition of symmetric restore helpers and explicit round-trip tests provides strong evidence of closure.

My primary architectural concern with the R3 changes is the asymmetry between the on-CAS byte representation (with zeroed identity fields) and the in-memory struct representation (with populated identity fields). This pattern, while correct, introduces a "footgun" for future developers who might naively `canonical_decode` CAS bytes without using the new, mandatory `restore_*` helpers, leading to silent failures from default/zeroed `capsule_id` fields.

Despite this concern, the R2 VETO is structurally closed. The immediate production-blocking defect is resolved. In accordance with `feedback_audit_loop_roi_flip`, the remaining architectural challenge is best addressed as a high-priority observation rather than blocking the ship for an R4. The TB-11 cross-cut finding is similarly deferred as a carry-forward OBS to maintain TB-15 scope.

The R3 changes are architecturally sound in their primary goal and sufficient for ship.

---

### 1. Architectural soundness of the writer pattern

**Finding**: The new writer pattern ("store the zeroed-identity bytes; compute capsule_id from them; populate in-memory struct after") is architecturally sound and **correctly closes the R2 VETO**. It guarantees that the `capsule_id` published is the content-address of the bytes stored in CAS, making `cas.get(&capsule.capsule_id)` resolvable. This holds for `write_markov_capsule` and the refactored `write_autopsy_capsule` / `derive_autopsies_for_bankruptcy` flow.

**Architectural Concern (Footgun)**: The pattern introduces a significant asymmetry between the on-CAS representation and the in-memory representation.
-   **On-CAS bytes**: `canonical_encode(Capsule { capsule_id: Cid::default(), ... })`
-   **In-memory struct**: `Capsule { capsule_id: <derived_from_cas_bytes>, ... }`

This asymmetry is a footgun. A future developer, unaware of this specific implementation detail, might fetch bytes from CAS and call `canonical_decode` directly. They would receive a syntactically valid struct but with a semantically invalid state (`capsule_id: Cid::default()`). This would not cause an immediate crash but would lead to silent, hard-to-debug failures downstream (e.g., failed lookups, incorrect indexing, broken chain validation). The system now relies on a **convention** (`must use restore_* helper`) rather than a structural guarantee at the type level.

**Alternative**: A cleaner architectural alternative would be to use `#[serde(skip)]` on the `capsule_id` and `sha256` fields. The on-CAS bytes would then naturally not contain these fields. The `restore_*` helpers would still be necessary to re-derive and populate these fields post-deserialization, but the intent would be more explicit in the struct definition itself, reducing the likelihood of a developer misinterpreting the on-CAS format. The current implementation is effectively a manual, less-discoverable `serde-skip`.

**Conclusion**: The pattern is correct but introduces a developer-experience hazard. This does not warrant a VETO as the production bug is fixed, but it should be addressed.

### 2. Restore helpers correctness

**Finding**: The new helpers, `restore_markov_capsule_from_cas_bytes` and `restore_autopsy_capsule_from_cas_bytes`, are correct. They symmetrically reverse the write operation by performing a `canonical_decode` and then re-deriving the `capsule_id` and `sha256` fields from the content hash of the input bytes. The new round-trip tests (`write_markov_capsule_cas_resolvable_by_capsule_id`, etc.) correctly assert this round-trip integrity.

**Invariant Violation Risk**: As identified in point #1, a consumer who calls `canonical_decode` on CAS bytes *without* calling the restore helper will get a struct with `capsule_id = Cid::default()`. This is an invariant violation that would propagate silently. If this default `Cid` is used as a key in a `BTreeMap` or for a subsequent CAS lookup, it will fail or cause data corruption.

**Recommendation**: To mitigate this footgun, I recommend two actions be taken as follow-up work (OBS):
1.  **Loud Failure**: Add an assertion or a debug-panic inside any public accessor method for `capsule_id` on both `MarkovEvidenceCapsule` and `AgentAutopsyCapsule`. This would turn the silent failure into a loud, immediate crash during development and testing if a non-restored capsule is used incorrectly. E.g., `pub fn capsule_id(&self) -> Cid { assert_ne!(self.capsule_id, Cid::default(), "Capsule not restored from CAS bytes; use restore_* helper"); self.capsule_id }`.
2.  **Documentation**: Add prominent doc-comments to both capsule structs and the `canonical_decode` function, warning developers that direct decoding of CAS-resident capsules is unsafe and that the `restore_*` helpers are mandatory.

### 3. BankruptcyAutopsyDerivation struct API

**Finding**: The new `BankruptcyAutopsyDerivation` struct is a significant architectural improvement over the previous `(AgentAutopsyCapsule, Vec<u8>)` tuple.
-   **Ergonomics**: The API is more ergonomic and self-documenting. Named fields (`capsule`, `private_bytes`, `stored_capsule_bytes`) are clearer than tuple indices.
-   **Correctness**: It correctly bundles the exact `stored_capsule_bytes` that must be written to CAS to satisfy the `capsule_id == sha256(bytes)` invariant. This eliminates the risk of the `apply_one` hook re-computing or misinterpreting the bytes to be stored, which was a contributing factor to the original bug.
-   **Idempotency**: The `derive_autopsies_for_bankruptcy` function remains pure. Since `CasStore::put` is idempotent on content, the overall `write_bankruptcy_autopsies_to_cas` operation remains idempotent and replay-safe. This is a solid design.

### 4. Cross-cut to TB-11

**Finding**: The R2 VETO root cause (storing bytes whose hash differs from the computed ID) is a pattern defect. It is highly probable that `src/runtime/evidence_capsule.rs::write_evidence_capsule` from TB-11 has the exact same bug. This represents a latent defect in a previously shipped component.

**Recommendation**: Per `feedback_audit_loop_roi_flip`, expanding the scope of TB-15 R3 to include a fix and re-verification for TB-11 would introduce undue delay. The correct action is to ship TB-15 with its targeted fix and carry forward the TB-11 finding as a high-priority observation.

-   **OBS-TB15-R3-1**: The `write_evidence_capsule` function in `src/runtime/evidence_capsule.rs` likely suffers from the same CAS-ID mismatch bug as TB-15 R2. It should be refactored to use the "store zeroed-identity, populate after" pattern, and a corresponding `restore_evidence_capsule_from_cas_bytes` helper should be added. A round-trip test asserting `cas.get(&evidence_capsule.capsule_id)` succeeds must be implemented. This should be addressed in the next available tracer bullet (e.g., TB-16).

### 5. New Findings

No new production-blocking findings were identified. The R3 remediations are surgically focused on the VETO closure. The primary new concern is the architectural footgun detailed in points #1 and #2, which is captured as an observation.

---

## VERDICT: PASS

**Conviction**: High.

The R3 remediation directly and correctly resolves the critical `capsule_id` mismatch bug that triggered the Codex R2 VETO. The new writer pattern, while introducing a developer-experience hazard, ensures the fundamental contract of the content-addressed store (`id == hash(content)`) is now met for both `MarkovEvidenceCapsule` and `AgentAutopsyCapsule`. The new round-trip tests provide the necessary mechanical proof of closure.

The architectural concerns regarding the "footgun" asymmetry and the latent bug in TB-11 are valid but do not block the ship of TB-15. They are captured as high-priority observations for follow-up work. The R2 VETO is closed.

**Recommendation**: PROCEED to SHIP. Address the footgun (API hardening, documentation) and the TB-11 bug as high-priority follow-up items in TB-16.