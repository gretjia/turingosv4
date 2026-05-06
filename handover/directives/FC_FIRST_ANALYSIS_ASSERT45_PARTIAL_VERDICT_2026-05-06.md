---
type: fc_first_analysis
date: 2026-05-06
parent_directive: handover/directives/2026-05-06_TB18R_ROUND_2_ARCHITECT_RULING.md (Phase 1 #5 / Q-P5 / Q-P2)
parent_dispatch: handover/audits/G2_TB_18R_ROUND_2_DUAL_AUDIT_DISPATCH_2026-05-06.md (§3.2 Gap B + §3.5 Gap E)
status: ANALYSIS-ONLY — not authority for Phase 2 implementation
phase_2_authorization: REQUIRES separate explicit remediation directive AFTER architect review of this document
---

# FC-First Analysis — assert_45 / step_partial_ok / PartialVerdict — 2026-05-06

> **This document is the upstream analysis the `feedback_fc_first_problem_handling` rule requires before any fix to the partial-verdict semantic gap.** It was missing from R8 design (round-2 dispatch §3.5 Gap E). The parent ruling §4 Q-P5 ruled the absence as CHALLENGE-not-VETO, with this document as the corrective.
>
> **Per `feedback_no_workarounds_strict_constitution` ("我不要凑活")**: this analysis must reach a constitutional verdict before Phase 2 schema work begins.

---

## §0 Source-of-truth grounding (verbatim from current code)

### §0.1 LeanResult struct — primary site

`src/runtime/attempt_telemetry.rs:373-412`. **TRACE_MATRIX annotation (line :375)**: `FC1-N41`.

```rust
/// TRACE_MATRIX FC1-N41: Lean verdict on a single externalized candidate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LeanResult {
    pub attempt_id: TxId,
    pub exit_code: i32,
    /// True iff Lean fully verified the candidate (no errors, no `sorry`).
    /// False if exit_code != 0 OR sorry was used OR partial verdict.
    pub verified: bool,
    pub stderr_cid: Option<Cid>,
    pub stdout_cid: Option<Cid>,
    pub proof_artifact_cid: Option<Cid>,
    /// Fine-grained error class. Per the partial-verdict-aware invariant
    /// (TB-18R G2 round-2 R8; enforced by `assert_45`):
    /// - `verified == true` ⇒ `error_class == None`   (clean omega path)
    /// - `!verified && exit_code != 0` ⇒ `error_class.is_some()`
    /// - `!verified && exit_code == 0` ⇒ `error_class` may be either
    ///     `None` (partial-verdict / `step_partial_ok`) or
    ///     `Some(SorryBlocked)` (`sorry`-block).
    pub error_class: Option<LeanErrorClass>,
}
```

### §0.2 assert_45 — invariant site

`src/runtime/audit_assertions.rs:2580-2656`. **TRACE_MATRIX annotation (line :2580)**: `FC2-N34`.

The invariant body (post-R8) checks the three implications above; PASSes if all hold, FAILs otherwise. The pre-R8 wording was the stricter iff `verified ↔ exit_code == 0` which produced the round-1 Q13 VETO.

### §0.3 step_partial_ok emission site

`experiments/minif2f_v4/src/bin/evaluator.rs:3489-3527`. The relevant fragment (commented at :3491-3494):

> // step_partial_ok stays CAS-only in R3; a future TB will
> // design an audit-compatible L4 lane for intermediate
> // partial-accept progress (would require a per-tactic
> // ProposalTelemetry to satisfy TB-7 Gate 5).

The actual write call (:3504-3527):

```rust
r2_write_attempt_telemetry(&mut cas_store, R2AttemptArgs {
    ...
    tool_name: "step_partial_ok",
    path_label: "step_partial_ok",
    is_omega_success: false,
    outcome: AttemptOutcome::LeanPass,         // ← (!) misnomer: see §3 below
    error_class: None,
    lean_result: Some((0, false)),              // (exit_code=0, verified=false)
    ...
});
```

`r2_write_attempt_telemetry` (`evaluator.rs:124-142`) constructs the `LeanResult` from `(exit_code, verified)` with `proof_artifact_cid = if verified { Some(candidate_cid) } else { None }` and `error_class` passed through.

So step_partial_ok writes the LeanResult triple `(0, false, None)` and the AttemptOutcome `LeanPass`.

### §0.4 VETO archive context (`TB18_TAPE_NON_EXTERNALIZATION_VETO_2026-05-06.md:614`)

> Flowchart hashes (no FC element added; existing FC1-N31..N40 cover the runtime loop; TB-18R adds witnesses, not nodes).

This confirms FC1-N31..N40 covers the runtime/proposal loop (where partial-verdict arises). FC2 covers the audit layer (where assert_45 lives).

---

## §1 The four constitutional questions

### §1.1 Q1 — Does PartialVerdict belong to FC1 (proposal flow) or FC2 (audit flow)?

**Answer: FC1.**

Three independent sources agree:

1. **Code-level TRACE_MATRIX**: `LeanResult` carries `FC1-N41` (`attempt_telemetry.rs:375`). The schema lives on the proposal-flow flowchart, not the audit-flow flowchart.
2. **Causal origin**: PartialVerdict arises in `evaluator.rs::step_partial_ok` — the per-iteration LLM-Lean cycle in the runtime/proposal loop. The state is **born** in FC1.
3. **VETO archive (`:614`)**: FC1-N31..N40 cover the runtime loop; TB-18R adds witnesses to FC1, not new audit-flow nodes. The partial-verdict state lives among FC1 witnesses.

assert_45 (FC2-N34) is the **audit-time invariant over an FC1 object**. FC2 reads FC1's witness; it does not own the witness's semantics.

**Implication**: any fix that "improves" assert_45 to admit partial-verdict is treating a symptom on FC2 while the constitutional contract sits on FC1. The contract that says "what shape may a `LeanResult` take?" is an FC1 contract.

### §1.2 Q2 — Is LeanResult a predicate-evidence object or an audit artifact?

**Answer: Primarily predicate evidence (FC1). Secondarily a read-only witness for FC2 audit walks.**

Operational distinction:

- **Predicate evidence (FC1 role)**: `LeanResult` is the externalized record of a proposal-flow LLM-Lean cycle. It tells the chain "this candidate, when fed to Lean, produced this verdict." It is an **input** to the predicate `predicate_passes` that the sequencer evaluates at admission time.
- **Audit witness (FC2 role)**: `LeanResult` CAS objects are walked by assert_45 + assert_44 + (future) attempt_chain_root computation. FC2 reads them; FC2 does **not** mint or modify them.

The asymmetry matters: FC2 invariants are downstream of FC1 schema. If FC1 schema admits an ambiguous state (e.g., `error_class=None` could mean partial-verdict OR a hypothetical bug), FC2 invariants cannot fully discriminate it.

**Implication**: the right place to typify "partial-verdict is a legitimate third state" is the **FC1 schema**, not the FC2 invariant.

### §1.3 Q3 — Is `error_class = None` for partial-verdict legal?

**Answer: Conditionally legal as semantic intent. NOT typed-legal.**

`error_class = None` for partial-verdict says "no error class because no error occurred (the verdict is partial-but-clean, not a failure)." This intent is constitutionally defensible — partial-verdict is in fact a non-failure state on FC1.

But the same triple `(exit_code=0, verified=false, error_class=None)` matches **three distinct semantic states**:

| Semantic state | (exit_code, verified, error_class) | Where it arises |
|----------------|-----------------------------------|-----------------|
| partial-verdict | (0, false, None) | step_partial_ok (FC1, legitimate) |
| `forbidden_payload` / `sorry` not classified | (0, false, None) | hypothetical bug if classify_lean_error misses a sorry-shape (FC1, defect) |
| forward variant we haven't designed | (0, false, None) | future TB extension (no current arises) |

The R8-relaxed assert_45 admits **all three** uniformly. It cannot distinguish the legitimate partial-verdict from a defect-emitting same-shape triple. This is the "semantic hole" the parent ruling §4 Q-P2 flagged: "exit_code=0, verified=false, error_class=None 太模糊，会成为 semantic hole".

**Implication**: `error_class = None` is **legal as long as a typed verdict tag accompanies it**. Without typing, the legality is inferred-by-context, which the round-2 dispatch §3.2 Gap B correctly identified as workaround-class.

### §1.4 Q4 — Does R8 change FC2 audit invariant or FC1 proposal semantics?

**Answer: R8 as currently shipped changes only the FC2 audit invariant. The FC1 proposal semantics remain unchanged.**

What R8 changed:
- `src/runtime/audit_assertions.rs:2580-2656` (FC2-N34) — replaced iff with three implications. **FC2 only.**
- `src/runtime/attempt_telemetry.rs:402-411` (FC1-N41 doc-comment) — refreshed the doc-comment to describe the three legitimate states. **No code change to FC1 schema; only documentation edit on the FC1 contract surface.**

What R8 did **not** change:
- `evaluator.rs::step_partial_ok` still writes `(exit_code=0, verified=false, error_class=None, AttemptOutcome::LeanPass)`.
- `LeanResult` struct still has `verified: bool` + `error_class: Option<LeanErrorClass>` as its only verdict-discriminator fields. No typed `LeanVerdict` enum is introduced.

So R8 is an **FC2-only patch over an FC1 ambiguity**. It is the constitutionally minimal way to clear the round-1 Q13 VETO without a Class-4 schema bump — but it leaves the FC1 ambiguity in place.

**This is the workaround**: the FC1 contract still has three semantic states multiplexed onto one untyped triple; FC2 just stopped checking the multiplexing.

---

## §2 What R8 should be (constitutional verdict)

### §2.1 The architect's own framing (Phase 2 binary choice)

Parent ruling §5 Phase 2 offered:

- **(α) Strict alignment**: `step_partial_ok` should not emit a LeanResult at all, OR should emit `error_class = Some(LeanErrorClass::PartialVerdict)` (a new variant) / `Some(SorryBlocked)`. Revert R8's invariant relaxation.
- **(β) Invariant relaxation with typed PartialAccepted state**: Introduce `LeanVerdict::PartialAccepted` (or `verdict_kind: VerdictKind` field). Update schema, doc-comment, invariant. Tests prove three legal states are typed-distinguishable.

Architect leaning: **β with typing**. NOT plain β (R8-as-shipped is plain β; no typing).

### §2.2 My constitutional verdict

**β-with-typing is correct. Plain β (R8-as-shipped) is workaround-class. Plain α is wrong-direction.**

Reasoning:

- **Why not plain α**: forcing `step_partial_ok` to emit `error_class = Some(LeanErrorClass::PartialVerdict)` *or* not to emit a LeanResult at all conflates "non-failure intermediate progress" with "failure" or "no Lean activity". Both reduce the semantic richness of FC1 evidence. step_partial_ok IS a Lean event (Lean compiled successfully on the partial candidate); pretending it isn't would lose audit-trail signal.
- **Why not plain β**: as §1.3 establishes, `(0, false, None)` is a multiplexed triple; relaxing FC2 to admit it loses defect-detection on FC1.
- **Why β-with-typing**: introduces a typed third state on FC1 (where it constitutionally belongs per §1.1), then makes FC2 invariant typed-aware. This is the constitutionally-clean form.

### §2.3 Two concrete Phase 2 schema options

**Option A — replace `verified` + `error_class` with a single typed enum (biggest break)**:

```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LeanVerdict {
    /// exit_code=0, omega-complete proof; proof_artifact present.
    Verified,
    /// exit_code≠0; specific error class carried.
    Failed(LeanErrorClass),
    /// exit_code=0, intermediate non-failure Lean accept (step_partial_ok).
    /// NOT omega-complete; no proof_artifact emitted.
    PartialAccepted,
    /// exit_code=0, sorry / forbidden_payload classified.
    SorryBlocked,
}

pub struct LeanResult {
    pub attempt_id: TxId,
    pub exit_code: i32,
    pub verdict: LeanVerdict,                  // replaces verified + error_class
    pub stderr_cid: Option<Cid>,
    pub stdout_cid: Option<Cid>,
    pub proof_artifact_cid: Option<Cid>,
}
```

assert_45 becomes:

```rust
match lr.verdict {
    LeanVerdict::Verified => assert!(lr.exit_code == 0 && lr.proof_artifact_cid.is_some()),
    LeanVerdict::Failed(_) => assert!(lr.exit_code != 0),
    LeanVerdict::PartialAccepted => assert!(lr.exit_code == 0 && lr.proof_artifact_cid.is_none()),
    LeanVerdict::SorryBlocked => assert!(lr.exit_code == 0 && lr.proof_artifact_cid.is_none()),
}
```

**Cost**: byte-incompatible with all existing R6/R7 LeanResult CAS objects. Requires schema_version 1 → 2 + parallel decoder for legacy.

**Option B — tail-additive `verdict_kind` field, preserving existing fields (smallest break)**:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum LeanVerdictKind {
    Verified = 0,         // exit_code=0, verified=true, error_class=None
    Failed = 1,           // exit_code≠0, verified=false, error_class=Some(...)
    PartialAccepted = 2,  // exit_code=0, verified=false, error_class=None
    SorryBlocked = 3,     // exit_code=0, verified=false, error_class=Some(SorryBlocked)
}

pub struct LeanResult {
    pub attempt_id: TxId,
    pub exit_code: i32,
    pub verified: bool,                        // existing (now redundant w/ kind)
    pub stderr_cid: Option<Cid>,
    pub stdout_cid: Option<Cid>,
    pub proof_artifact_cid: Option<Cid>,
    pub error_class: Option<LeanErrorClass>,   // existing
    pub verdict_kind: LeanVerdictKind,         // NEW (tail-additive)
}
```

assert_45 becomes typed via `verdict_kind` and adds a *consistency* clause `verdict_kind ↔ (verified, error_class.is_some(), exit_code != 0)` so the redundant fields cannot drift.

**Cost**: schema_version bump but byte-compatible-with-default for missing-field decode. Mirrors the R3 `RejectionClass` tail-append pattern that was already ratified.

### §2.4 Recommendation

**Option B** is preferable on these grounds:

1. **Byte-stable for legacy CAS**: pre-Phase-2 LeanResult records can decode with `verdict_kind` defaulted (or schema_version-gated re-derivation from `(verified, error_class, exit_code)`).
2. **Mirrors ratified pattern**: tail-additive enum with `#[repr(u8)]` is exactly the R3 RejectionClass approach (lines 6/7/8/9), which Codex Q8 explicitly ratified.
3. **Audit-trail continuity**: R6/R7 evidence remains canonical-decodable post-Phase-2; no migration of pre-Phase-2 chain artifacts (consistent with `feedback_no_retroactive_evidence_rewrite`).
4. **Allows `verified` to remain the FC2 ship-gate metric** while `verdict_kind` becomes the typed discriminator. Existing `pput_verified` and ChainDerivedRunFacts code paths that read `verified: bool` continue to work without retrofit.
5. **Lower implementation surface** → lower risk of introducing new defects under STEP_B time pressure.

Option A is **cleaner long-term** but is a Class-4 schema rewrite; Option B achieves the constitutional goal (typed PartialAccepted state) with smaller blast radius.

Trade-off acknowledgment: Option B keeps the redundancy `verified` ⇄ `verdict_kind`. The redundancy is contained by an assert_45 consistency clause; over future TBs the `verified: bool` field can deprecate (mark `#[deprecated]` + reads forced through `verdict_kind`), reaching Option A's shape incrementally.

### §2.5 Adjunct cleanup (out of scope for Phase 2 alone but relevant)

The R2 `AttemptOutcome::LeanPass` label for `step_partial_ok` (`evaluator.rs:3518`) is a misnomer — step_partial_ok is NOT a Lean PASS in the omega-complete sense. The correct AttemptOutcome should be a new variant like `AttemptOutcome::PartialAccepted`. This is a tail-additive enum bump on AttemptOutcome (parallel to RejectionClass / LeanErrorClass).

If Phase 2 covers this, the FC1 schema gains:
- `LeanVerdictKind::PartialAccepted` on LeanResult
- `AttemptOutcome::PartialAccepted` on AttemptTelemetry

Both consistent; both tail-additive; both Class-4 schema bumps.

If Phase 2 does NOT cover the AttemptOutcome rename, that's a separate forward-bound OBS: the LeanPass-misnomer is a documented risk with no chain-state impact (sequencer mapping is via the rejection arm, which does not consume `outcome` directly for omega-success paths).

**Recommendation**: include the AttemptOutcome::PartialAccepted bump in Phase 2 to avoid double-pass. But this is the architect's call.

---

## §3 What R8 should remain (interim status)

R8-as-shipped is **interim**: the FC2 invariant relaxation is correct *as a temporary fix to the round-1 Q13 VETO*, conditional on Phase 2 typed-PartialAccepted landing. Without Phase 2, R8 leaves a semantic hole.

**Therefore**:
- R8 stays on `main` per parent ruling §3.1 (do-not-rollback).
- R8 ratification is **conditional**: passes only if Phase 2 (Option B or Option A) introduces typed PartialAccepted, and assert_45 is reworked to be `verdict_kind`-typed-aware.
- If Phase 2 is rejected by architect / never implemented, R8 must be **reverted** in favor of α (force step_partial_ok to emit `error_class = Some(LeanErrorClass::PartialVerdict)` or to not emit LeanResult).

---

## §4 FC3 (economic) cross-edge audit

Parent ruling §4 Q-P5 implicitly asks whether FC3 (economic flow) has any cross-edge depending on assert_45's strict iff.

**Survey** (grepped FC3-relevant invariant sites):

- `src/state/sequencer.rs` admission: predicate_passes is computed from runtime evaluator; sequencer does NOT call assert_45. assert_45 is post-hoc audit only. → **no FC3 dependency**.
- `src/sdk/tools/wallet.rs`: stake/reward derives from `predicate_passes` via WorkTx, not from `verified` on LeanResult. → **no FC3 dependency**.
- `src/runtime/chain_derived_run_facts.rs`: ship-gate equation counts L4 vs L4.E by chain identity, not by LeanResult.verified. → **no FC3 dependency**.
- `experiments/minif2f_v4/src/bin/evaluator.rs::compute_pput_m`: reads `verified` from a separate per-problem result struct (`B4Result`), not from `LeanResult.verified`. → **no FC3 dependency**.

**Conclusion**: assert_45 is FC2-only, with no FC3 cross-edge. R8's invariant relaxation does not perturb economic flow. This validates that R8 is FC2-scope, and that Phase 2 typed-PartialAccepted will likewise be FC1-FC2-only without economic-engine downstream.

---

## §5 What this analysis does NOT do

- **Does NOT begin Phase 2 implementation.** No LeanVerdictKind enum is added to source. No schema_version bump. No assert_45 retyping.
- **Does NOT authorize Class-4 STEP_B preflight.** Phase 2 source work requires a separate explicit remediation directive citing this analysis.
- **Does NOT re-run P38/P49/M0.** Phase 3 reruns happen *after* Phase 2 implementation lands and STEP_B audits pass.
- **Does NOT mark TB-18R as ship-eligible.** TB-18R remains in CANDIDATE REMEDIATION until Phase 1 → 2 → 3 → final dual audit completes.

---

## §6 Summary table for architect review

| Question | Verdict |
|----------|---------|
| Q1 (PartialVerdict in FC1 or FC2?) | **FC1** (LeanResult is FC1-N41; state born in evaluator proposal loop) |
| Q2 (LeanResult predicate-evidence or audit-artifact?) | **Primarily predicate-evidence; secondarily audit-witness** |
| Q3 (error_class=None for partial-verdict legal?) | **Conditionally legal as semantic intent; NOT typed-legal** — ambiguity is the workaround risk |
| Q4 (R8 changes FC2 invariant or FC1 semantics?) | **R8 changes FC2 invariant only** — FC1 semantics still ambiguous |
| FC3 cross-edge? | **None** — assert_45 is FC2-scope; no economic downstream |
| Constitutional repair form (α / β / β-with-typing)? | **β-with-typing** (architect's preferred). Plain β is workaround. Plain α is wrong-direction. |
| Recommended Phase 2 schema option? | **Option B** (tail-additive `verdict_kind: LeanVerdictKind` field) — mirrors R3 RejectionClass pattern; byte-stable for legacy decode |
| Should `AttemptOutcome::LeanPass` for step_partial_ok be renamed? | **Yes** — recommend `AttemptOutcome::PartialAccepted` in same Phase 2 bundle |
| R8 disposition? | **Interim; preserve on `main` per parent ruling §3.1; ratification conditional on Phase 2** |

---

## §7 Halt point

This analysis ends here. Per architect-aligned discipline (`feedback_class4_cannot_hide_in_class3`, `feedback_no_workarounds_strict_constitution`), Phase 2 begins **only after**:

1. Architect explicit review of this FC-first analysis.
2. Architect explicit choice between Option A and Option B (or rejection of both with a third-option directive).
3. Architect explicit ratification of the AttemptOutcome::PartialAccepted bump scope (in Phase 2 bundle, or deferred to a separate atom).
4. A new file `handover/directives/2026-05-06_TB18R_PHASE_2_REMEDIATION_DIRECTIVE.md` (or similar) authoring the Phase 2 STEP_B preflight under a fresh authorization, citing this analysis as upstream.

**Until then, no source-code change implementing PartialAccepted is authorized.**

---

## §8 Cross-references

- Parent ruling: `handover/directives/2026-05-06_TB18R_ROUND_2_ARCHITECT_RULING.md`
- R8 ratification addendum: `handover/directives/2026-05-06_TB18R_R8_R12_RATIFICATION_ADDENDUM.md`
- R3 supersession OBS: `handover/alignment/OBS_TB18R_R3_PREFLIGHT_SUPERSESSION_2026-05-06.md`
- Round-2 dispatch §3.2 Gap B + §3.5 Gap E: `handover/audits/G2_TB_18R_ROUND_2_DUAL_AUDIT_DISPATCH_2026-05-06.md`
- LeanResult schema: `src/runtime/attempt_telemetry.rs:373-412`
- assert_45: `src/runtime/audit_assertions.rs:2580-2656`
- step_partial_ok emission: `experiments/minif2f_v4/src/bin/evaluator.rs:3489-3527`
- VETO archive (FC1-N31..N40 mention): `handover/architect-insights/TB18_TAPE_NON_EXTERNALIZATION_VETO_2026-05-06.md:614`
- Memory rules cited:
  - `feedback_no_workarounds_strict_constitution`
  - `feedback_fc_first_problem_handling`
  - `feedback_class4_cannot_hide_in_class3`
  - `feedback_no_retroactive_evidence_rewrite`

---

**End of FC-first analysis. Awaits architect ratification before any Phase 2 source change.**
