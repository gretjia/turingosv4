# Gemini CO1.8 Round-1 Audit
**Date**: 2026-04-29
**Target**: spec v1 (greenfield)
**HEAD**: e2752e83d40df448622ae05947ac694a1e0cea0e
**Prompt size**: 131342 chars
**API latency**: 40.3s

---

This is a **round-1 dual external audit** from the **Gemini DeepThink** perspective. My review focuses on strategic alignment, constitutional invariants, and architectural sustainability. 

The greenfield draft is exceptionally well-reasoned. The author’s decomposition of WP § 5.L5 into 8 sub-atoms is logical, and the substrate-independence boundary is correctly drawn. However, there is one architectural hazard regarding the Inv 10 Goodhart shield stubbing that must be corrected before implementation.

Here is the formal verdict.

### Section A: Verdict

**CHALLENGE** with **HIGH** conviction.

The spec is architecturally sound in its handling of state roots, layer boundaries, and Anti-Oreo separation. The CHALLENGE is strictly scoped to **Q4 (Inv 10 Goodhart shield stubbing)**. Shipping a default-allow (full passthrough) visibility filter, even as a temporary stub, creates a structural vulnerability and risks downstream test coupling to leaked internals.

### Section B: P0 Blockers (Must-fix for round-2)

**B.1. Default-Deny / Restrictive Stubbing for `project_for_agent` (Ref: § 0.4 #4 & § 4.4)**
*   **The Flaw:** Spec § 0.4 #4 states that `project_for_agent` will ship as a "no-op filter (returns full view)" until CO1.5 ships. This violates Inv 10 (Goodhart shield) by default. If early agents or tests are built against this stub, they will accidentally couple to evaluator internals, making the eventual CO1.5 integration a massive breaking change.
*   **The Fix:** The v1 stub MUST be restrictive (deny-by-default or hardcoded-minimal). It should return *only* the agent's own `PerAgentState`, public `TaskMarketEntry` fields, and global budget/round data. It must explicitly strip or omit any `evaluator_internal` fields. 
*   **Action:** Rewrite § 0.4 #4 to mandate a restrictive stub. Update the test in § 4.4 to assert that the stub actively drops known internal fields (e.g., `oracle_seed`), proving the Inv 10 shield is structurally active even without dynamic CO1.5 tags.

### Section C: Resolution of Strategic Questions (Q1-Q7)

**C.1 (Q1) Constitutional alignment of L5:** I strongly affirm the author's lean. `state_root_t` MUST be the SHA-256 of the canonical-serialized snapshot. If we equate `state_root_t` to a git tree OID, we permanently leak the storage substrate (Path B) into the constitutional state definition, violating Anti-Oreo. The STATE spec comment ("git tree root in Path B") should be interpreted as "the snapshot is *stored* at this git tree root", not "the hash *is* the git tree root". PASS author's lean.

**C.2 (Q2) Anti-Oreo separation:** PASS. `materializer::apply` correctly operates at FC3 (bottom-white). It dispatches on `TxKind` and applies the delta. It does *not* evaluate predicates (FC1). `read_tool` is correctly positioned as a bottom-white data projection function consumed by top-white agents.

**C.3 (Q3) L5 vs L6 boundary:** PASS. The absolute vs. derived cut is mathematically and architecturally correct. L5 holds the absolute materialized state (fold over L4). L6 holds statistical/windowed derivations over L5. 

**C.4 (Q5) WP § 5.L5 fidelity:** PASS. Folding `permission_view` and `read_tool` into `agent_view` is pragmatic and aligns with standard Rust module design. The 8-atom decomposition is approved.

**C.5 (Q6) Sub-atom shippability:** Clarification: A PASS/PASS on this v1 spec authorizes all 8 sub-atoms. They do not need individual dual-audits if they conform to this spec. They should be implemented in separate, reviewable commits (STEP_B non-restricted) that incrementally build the `mod.rs` re-export tree.

**C.6 (Q7) Forward sustainability:** PASS author's implicit interface. Do *not* ship a `pub trait L5Surface` yet. Premature abstraction is an anti-pattern. L5 should expose concrete Rust getters (`pub fn reputation_for(...)`). When CO1.9 (L6) is designed, it will consume these concrete getters.

### Section D: Suggested Patches

**Patch 1: Update § 0.4 #4 (Restrictive Stub)**
```markdown
- 4. **CO1.5 PredicateRegistry visibility tags** — CO1.8.7 `project_for_agent` interface lands; tag-driven filtering is no-op (returns full view) until CO1.5 ships visibility tags. Documented as known gap; not blocking.
+ 4. **CO1.5 PredicateRegistry visibility tags** — CO1.8.7 `project_for_agent` interface lands. Until CO1.5 ships, this acts as a **hardcoded restrictive stub** (returns only the agent's own state, public task fields, and global budget). It MUST NOT return a full view, preserving the Inv 10 Goodhart shield structurally and preventing downstream test coupling to internals.
```

**Patch 2: Update § 5 Open Questions Table (Q1 & Q5)**
*   Update Q1 to reflect that the SHA-256 semantics are constitutionally normative and approved.
*   Update Q5 to reflect the restrictive stub mandate.

### Section E: Forward-Sustainability Notes

1.  **Path B Git-Tree Migration (CO1.8-extra):** By locking in SHA-256 snapshot semantics now, the future Path B migration becomes purely a storage-layer swap. The materializer will serialize the snapshot, hash it (SHA-256) for the `state_root_t`, and then hand the bytes to `git2-rs` to store as a blob/tree. This ensures the constitutional tape remains verifiable without requiring a git binary.
2.  **BTreeMap Caching:** The v1 `CurrentStateDb` uses an in-memory `BTreeMap`. Ensure that the `apply` function's signature and error handling (`PriorRootNotFound`) are designed such that when CO1.8-extra introduces lazy loading from the git substrate, the public API of `apply` does not need to change (i.e., keep it returning `Result<Hash, MaterializerError>`).