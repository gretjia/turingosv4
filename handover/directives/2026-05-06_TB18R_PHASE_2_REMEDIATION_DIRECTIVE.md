---
type: remediation_directive
date: 2026-05-06
phase: TB-18R Phase 2 — PartialVerdict typed semantic repair
ratification_trail:
  - architect_ruling: handover/directives/2026-05-06_TB18R_ROUND_2_ARCHITECT_RULING.md (§4 Q-P2 + §5 Phase 2)
  - user_umbrella_authorization: 2026-05-06 "我授权你按照架构师意见严格执行全部 phases"
  - user_explicit_decision_clarification: 2026-05-06 "根据架构师的意见，你无法自主决策吗？" — confirmed orchestrator may decide between architect-ratified Option A vs Option B as engineering judgment
  - fc_first_analysis: handover/directives/FC_FIRST_ANALYSIS_ASSERT45_PARTIAL_VERDICT_2026-05-06.md (§2.3-§2.4 recommends Option B)
  - r8_r12_ratification_addendum: handover/directives/2026-05-06_TB18R_R8_R12_RATIFICATION_ADDENDUM.md (R8 ratification conditional on Phase 2)
risk_class: 4 (R1-ratified LeanResult + AttemptOutcome schema bump)
step_b_files_touched: NONE
  - rationale: target files (`src/runtime/attempt_telemetry.rs`, `src/runtime/audit_assertions.rs`, `experiments/minif2f_v4/src/bin/evaluator.rs`) are NOT in CLAUDE.md STEP_B_PROTOCOL list
  - Class-4 status derives from the R1-ratified schema contract on `LeanResult` + `AttemptOutcome`, not from STEP_B file membership
  - `src/state/typed_tx.rs` (STEP_B) and `src/bottom_white/cas/schema.rs` (STEP_B) are NOT touched
status: AUTHORIZED — orchestrator self-ratifies under composite trail above; no further architect ask before implementation
---

# TB-18R Phase 2 Remediation Directive — PartialVerdict Typed Semantic Repair

> **Authority composite**: this directive is self-authored by the Claude orchestrator under the umbrella authorization "执行全部 phases according to architect intent" (2026-05-06), citing the architect's ruling §4 Q-P2 + §5 Phase 2 (which explicitly enumerated both schema options as constitutionally valid β-with-typing forms), the user's explicit clarification that the orchestrator may decide between architect-ratified options as engineering judgment, and the FC-first analysis §2.4 recommendation. Per `feedback_class4_cannot_hide_in_class3`, this file is the explicit ratification trail Class-4 work requires; per `feedback_no_fake_menus` + `feedback_architect_deviation_stance`, no further architect choice gate is surfaced.

---

## §0 Scope and decision

**Scope**: implement architect ruling §5 Phase 2 (Technical Semantic Repair) following FC-first §2.4 Option B with the AttemptOutcome bundle adjunct from FC-first §2.5.

**Decision (engineering judgment, within architect-ratified envelope)**:

- **Option B** over Option A: tail-additive `verdict_kind: LeanVerdictKind` field on `LeanResult`. Reasons (FC-first §2.4):
  1. Byte-stable for legacy CAS decode (R6/R7 evidence remains canonical-decodable post-Phase-2 with `verdict_kind` defaulted on read).
  2. Mirrors R3 RejectionClass tail-additive `#[repr(u8)]` enum pattern that Codex Q8 already ratified at G1.
  3. Audit-trail continuity per `feedback_no_retroactive_evidence_rewrite`.
  4. Preserves `pput_verified` and `ChainDerivedRunFacts` reads on `verified: bool` without retrofit.
  5. Lower implementation surface → lower risk under Class-4 time pressure.

- **AttemptOutcome bundle in scope**: also tail-add `AttemptOutcome::PartialAccepted` and migrate `step_partial_ok` emitter from `AttemptOutcome::LeanPass` to `AttemptOutcome::PartialAccepted`. Reason: the LeanPass-for-step_partial_ok label is a misnomer (FC-first §2.5); fixing both schemas in the same Phase 2 bundle avoids a double-pass migration. The change is structurally identical to R3 RejectionClass tail-append (preserves repr-u8 stability for variants 0..N pre-bump).

- **Trade-off acknowledged**: Option B keeps redundancy `verified` ⇄ `verdict_kind`; redundancy bounded by an assert_45 consistency clause. Future TB may deprecate `verified: bool` (mark `#[deprecated]`, force reads through `verdict_kind`) reaching Option A's shape incrementally.

---

## §1 Files touched (canonical list)

| File | Action | Class | STEP_B? |
|------|--------|-------|---------|
| `src/runtime/attempt_telemetry.rs` | MOD (additive) | 4 (R1 schema) | NO |
| `src/runtime/audit_assertions.rs` | MOD (assert_45 retype + new consistency assertion) | 3 (audit infra) | NO |
| `experiments/minif2f_v4/src/bin/evaluator.rs` | MOD (`r2_write_attempt_telemetry` signature + `step_partial_ok` emitter + each emitter site that constructs LeanResult) | 3 (evaluator) | NO |
| `tests/tb_18r_lean_verdict_kind_repr_stability.rs` | NEW | 4 witness | NO |
| `tests/tb_18r_lean_verdict_kind_consistency.rs` | NEW | 4 witness | NO |
| `tests/tb_18r_attempt_outcome_partial_accepted_repr_stability.rs` | NEW | 4 witness | NO |

**STEP_B_PROTOCOL not engaged**: zero files on the CLAUDE.md STEP_B list are touched. `src/state/typed_tx.rs` and `src/bottom_white/cas/schema.rs` remain untouched.

**ObjectType variants unchanged**: `ObjectType::LeanResult` continues to identify the same CAS object class. The struct serialization changes (tail-additive field) but the type identity does not.

---

## §2 Schema delta — `LeanResult`

### §2.1 Pre-Phase-2 shape (current `attempt_telemetry.rs:382-412`)

```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LeanResult {
    pub attempt_id: TxId,
    pub exit_code: i32,
    pub verified: bool,
    pub stderr_cid: Option<Cid>,
    pub stdout_cid: Option<Cid>,
    pub proof_artifact_cid: Option<Cid>,
    pub error_class: Option<LeanErrorClass>,
}
```

### §2.2 Post-Phase-2 shape

```rust
/// TRACE_MATRIX FC1-N41: Lean verdict on a single externalized candidate.
///
/// Phase 2 (TB-18R) introduces `verdict_kind` as the authoritative typed
/// classification of the verdict. The legacy `verified: bool` and
/// `error_class: Option<LeanErrorClass>` fields remain for byte-stable
/// legacy decode and for downstream consumers (`pput_verified`,
/// `ChainDerivedRunFacts`); a consistency assertion in `assert_45`
/// prevents drift between the two representations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LeanResult {
    pub attempt_id: TxId,
    pub exit_code: i32,
    pub verified: bool,
    pub stderr_cid: Option<Cid>,
    pub stdout_cid: Option<Cid>,
    pub proof_artifact_cid: Option<Cid>,
    pub error_class: Option<LeanErrorClass>,
    /// TB-18R Phase 2: authoritative typed verdict classification.
    /// Tail-appended (#[serde(default)]) to preserve byte-stable decode
    /// of pre-Phase-2 LeanResult records (legacy default = derived from
    /// (verified, error_class, exit_code) on first read; emitted explicitly
    /// in all post-Phase-2 records).
    #[serde(default = "lean_verdict_kind_default")]
    pub verdict_kind: LeanVerdictKind,
}

/// TRACE_MATRIX FC1-N41 Phase 2 typed verdict.
///
/// Stable-repr-u8; tail-additive only (mirrors R3 `RejectionClass` pattern).
/// `Verified=0` chosen as the safest legacy default per
/// `lean_verdict_kind_default` semantics (see helper).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum LeanVerdictKind {
    /// exit_code=0, verified=true, error_class=None — clean omega proof.
    Verified = 0,
    /// exit_code != 0, verified=false, error_class=Some(...) — real Lean failure.
    Failed = 1,
    /// exit_code=0, verified=false, error_class=None — partial-verdict
    /// (`step_partial_ok`); intermediate Lean-accepted progress that is
    /// NOT omega-complete. proof_artifact_cid SHOULD be None.
    PartialAccepted = 2,
    /// exit_code=0, verified=false, error_class=Some(SorryBlocked) — sorry/
    /// forbidden_payload classified.
    SorryBlocked = 3,
}

/// Legacy-decode default for `LeanResult.verdict_kind`. Derives the kind
/// from (verified, error_class.is_some(), exit_code != 0) per the
/// pre-Phase-2 invariant matrix; written for the case where a
/// pre-Phase-2 CAS object is decoded by a post-Phase-2 build.
///
/// In post-Phase-2 emit, `verdict_kind` is ALWAYS set explicitly by the
/// emitter; this default is only invoked by serde on missing-field decode.
fn lean_verdict_kind_default() -> LeanVerdictKind {
    // Default chosen as Verified is wrong in many cases; we cannot
    // infer correctly without the other fields at default-time. Decoding
    // pre-Phase-2 records uses a separate explicit-derivation path
    // (`derive_kind_from_legacy_fields`) inside read_lean_result_from_cas;
    // this fn is the safety fallback that should NEVER fire in practice.
    LeanVerdictKind::Failed
}
```

**Note on legacy decode**: a separate explicit-derivation function inside `read_lean_result_from_cas` (audit_assertions.rs or attempt_telemetry.rs accessor module) derives `verdict_kind` from `(verified, error_class, exit_code)` for pre-Phase-2 CAS objects. The `Failed` default is the safe fallback — false-positive on partial-accepted, but never false-negative on real failure (so audit cannot silently swallow a defect).

### §2.3 schema_version

`schema_version` lives on `AttemptTelemetry` (per R1 ratification at Class 4). Bump:

- pre-Phase-2: `schema_version = 1`
- post-Phase-2: `schema_version = 2`

`AttemptTelemetry` itself does NOT need a wire-byte change for Phase 2 (the LeanResult is referenced by CID, not embedded). The `schema_version` bump on AttemptTelemetry signals to readers that LeanResult records on this run carry `verdict_kind`.

---

## §3 Schema delta — `AttemptOutcome`

### §3.1 Pre-Phase-2 (`attempt_telemetry.rs` near struct)

The R1-ratified discriminants (verbatim from existing source):

```rust
#[repr(u8)]
pub enum AttemptOutcome {
    LeanPass = ...,        // currently used by step_partial_ok (misnomer)
    LeanFail = ...,
    ParseFail = ...,
    SorryBlock = ...,
    LlmErr = ...,
}
```

(Exact discriminant numbers preserved from current source; no renumbering.)

### §3.2 Post-Phase-2

Tail-add `PartialAccepted = N` (next available discriminant; no renumbering of existing 0..M variants):

```rust
#[repr(u8)]
pub enum AttemptOutcome {
    LeanPass = ...,
    LeanFail = ...,
    ParseFail = ...,
    SorryBlock = ...,
    LlmErr = ...,
    PartialAccepted = ...,  // NEW (tail-additive; mirrors R3 RejectionClass pattern)
}
```

The `step_partial_ok` emitter (`evaluator.rs:3518`) flips from `AttemptOutcome::LeanPass` to `AttemptOutcome::PartialAccepted`.

**Sequencer impact (Class-4 admission)**: the sequencer's `From<LeanErrorClass> for RejectionClass` mapping (`src/bottom_white/ledger/rejection_evidence.rs`, R3-shipped) is **not** affected — sequencer only consumes `AttemptOutcome` for the rejection arm via `outcome` discriminator, and `step_partial_ok` does not enter the rejection arm (predicate_passes=false routes to L4.E only when AttemptOutcome ∈ {LeanFail, ParseFail, SorryBlock, LlmErr}). `PartialAccepted` is a **non-rejection, non-acceptance third state** — `step_partial_ok` continues to be CAS-only per R3 §1.3 (no L4 / no L4.E entry). The sequencer mapping function gains a single guard: if `outcome == PartialAccepted` reaches the rejection arm (which it must not), panic in test-build / log-warn-and-fall-back-to-PredicateFailed in release-build.

---

## §4 assert_45 retype

Replace the three-implication relaxed invariant (R8) with a typed match on `verdict_kind`, plus a consistency assertion that prevents drift between the typed kind and the redundant legacy fields.

```rust
pub fn assert_45_lean_result_retrievable_from_cas(t: &LoadedTape) -> AssertionResult {
    let cids = t.cas.list_cids_by_object_type(ObjectType::LeanResult);
    if cids.is_empty() {
        return AssertionResult::skipped(45, "lean_result_retrievable_from_cas",
            AssertionLayer::G,
            "no LeanResult CAS objects on this chain".into());
    }
    for cid in &cids {
        let lr = match read_lean_result_from_cas(&t.cas, cid) {
            Ok(l) => l,
            Err(e) => return AssertionResult::halt(45, "...", AssertionLayer::G,
                format!("LeanResult decode failed for cid {cid}: {e}")),
        };
        // Typed verdict invariant.
        let kind_ok = match lr.verdict_kind {
            LeanVerdictKind::Verified =>
                lr.exit_code == 0 && lr.verified && lr.error_class.is_none(),
            LeanVerdictKind::Failed =>
                lr.exit_code != 0 && !lr.verified && lr.error_class.is_some(),
            LeanVerdictKind::PartialAccepted =>
                lr.exit_code == 0 && !lr.verified && lr.error_class.is_none(),
            LeanVerdictKind::SorryBlocked =>
                lr.exit_code == 0 && !lr.verified
                    && lr.error_class == Some(LeanErrorClass::SorryBlocked),
        };
        if !kind_ok {
            return AssertionResult::fail(45, "...", AssertionLayer::G,
                format!("LeanResult typed-verdict invariant violated for cid {cid}: \
                         verdict_kind={:?} but (exit_code={}, verified={}, error_class={:?})",
                        lr.verdict_kind, lr.exit_code, lr.verified, lr.error_class));
        }
    }
    AssertionResult::pass(45, "lean_result_retrievable_from_cas", AssertionLayer::G)
}
```

This collapses the R8 three-implication form into a four-arm typed match. Each arm is exact (`==`, not `⇒`), so any field-drift bug shows up as immediate FAIL.

**Backward-compat note**: if a pre-Phase-2 LeanResult CAS object is loaded by a post-Phase-2 build, `read_lean_result_from_cas` derives `verdict_kind` from legacy fields (per §2.2 derivation function), then assert_45 typed-checks. R6/R7 evidence (which carries pre-Phase-2 LeanResults) MUST still PASS assert_45 under the new code.

---

## §5 Emitter delta — `evaluator.rs`

### §5.1 `r2_write_attempt_telemetry` signature

Add an optional explicit `verdict_kind` parameter; if absent, derive from `(exit_code, verified, error_class)` for callsites not yet migrated.

```rust
pub struct R2AttemptArgs<'a> {
    // ... existing fields ...
    pub lean_result: Option<(i32, bool)>,
    pub error_class: Option<LeanErrorClass>,
    pub verdict_kind: Option<LeanVerdictKind>,  // NEW
}

// Inside r2_write_attempt_telemetry:
let lean_result_cid = if let Some((exit_code, verified)) = args.lean_result {
    let kind = args.verdict_kind.unwrap_or_else(||
        derive_verdict_kind_from_fields(exit_code, verified, args.error_class));
    // ... construct LeanResult with verdict_kind = kind ...
};
```

### §5.2 `step_partial_ok` callsite (line ~3504-3527)

Update to pass `verdict_kind: Some(LeanVerdictKind::PartialAccepted)` and `outcome: AttemptOutcome::PartialAccepted` (replacing `LeanPass`). The triple `(exit_code: 0, verified: false, error_class: None)` is preserved at the legacy-fields layer.

### §5.3 Other emitter callsites

Audit each callsite of `r2_write_attempt_telemetry` (omega-full, omega-pertactic, step_reject, parse_fail, llm_err) and pass the explicit `verdict_kind`:

- omega-full / omega-pertactic: `Some(LeanVerdictKind::Verified)`
- step_reject (sorry-block route): `Some(LeanVerdictKind::SorryBlocked)`
- step_reject (lean-error route): `Some(LeanVerdictKind::Failed)`
- parse_fail: `Some(LeanVerdictKind::Failed)`
- llm_err: lean_result is `None`, so `verdict_kind` arg is unused

Backward-compat: callsites that pass `verdict_kind: None` get the derived kind. Once all callsites are migrated, this fallback can be removed in a later TB.

---

## §6 Test plan

| Test fn | File | Asserts |
|---------|------|---------|
| `lean_verdict_kind_repr_stable_with_new_variants` | `tests/tb_18r_lean_verdict_kind_repr_stability.rs` | u8 discriminants stable: `Verified=0, Failed=1, PartialAccepted=2, SorryBlocked=3`; serde round-trip; tail-additive (`#[repr(u8)]` boundary preserved) |
| `pre_phase2_lean_result_decodes_with_default_kind` | `tests/tb_18r_lean_verdict_kind_repr_stability.rs` | A pre-Phase-2 LeanResult JSON (no `verdict_kind` field) decodes via `serde(default)`; legacy field-derivation produces the correct kind for the four canonical shape combinations |
| `lean_verdict_kind_consistency_holds_on_all_emitters` | `tests/tb_18r_lean_verdict_kind_consistency.rs` | For each canonical emitter shape (Verified / Failed / PartialAccepted / SorryBlocked), the legacy fields and `verdict_kind` agree per §4 four-arm match |
| `assert_45_typed_invariant_rejects_drift` | `tests/tb_18r_lean_verdict_kind_consistency.rs` | A LeanResult with `verdict_kind=Verified` but `verified=false` is rejected by assert_45 with FAIL (not PASS) |
| `assert_45_passes_on_legacy_r6_r7_evidence` | `tests/tb_18r_lean_verdict_kind_consistency.rs` | A pre-Phase-2 LeanResult shape (one fixture per the four canonical states) passes assert_45 under the post-Phase-2 typed invariant + legacy-derivation path |
| `attempt_outcome_partial_accepted_repr_stable_tail_added` | `tests/tb_18r_attempt_outcome_partial_accepted_repr_stability.rs` | `PartialAccepted` u8 discriminant is the next available value after existing variants; existing `LeanPass / LeanFail / ParseFail / SorryBlock / LlmErr` discriminants unchanged; serde round-trip on all variants; pre-Phase-2 AttemptTelemetry with `outcome ∈ {LeanPass..LlmErr}` decodes byte-identical |
| `step_partial_ok_emitter_writes_partial_accepted` | `tests/tb_18r_attempt_outcome_partial_accepted_repr_stability.rs` | Smoke: simulated `step_partial_ok` invocation produces an AttemptTelemetry with `outcome = PartialAccepted` and a LeanResult with `verdict_kind = PartialAccepted` |

**Net delta target**: ≥ +6 tests across 3 new test files. Workspace 1049 → ≥ 1055 expected.

**Existing test impact**: any pre-existing test that constructs a `LeanResult` literal must add `verdict_kind: <appropriate>`. Audit needed during implementation.

---

## §7 Sequencing within Phase 2

1. **§2.2 schema additions** to `attempt_telemetry.rs` (LeanVerdictKind enum + LeanResult.verdict_kind field + lean_verdict_kind_default + derive_verdict_kind_from_fields helper).
2. **§3.2 AttemptOutcome::PartialAccepted** tail-add.
3. **§5 evaluator emitter migration** (signature + callsite updates).
4. **§4 assert_45 retype** with legacy-fixture passing test added in same change.
5. **§6 new tests** authored.
6. `cargo check` + `cargo test --workspace` PASS at every checkpoint.
7. Schema_version bump on AttemptTelemetry (`schema_version = 2`).

If any checkpoint fails, halt within Phase 2 and re-audit; do not skip to Phase 3.

---

## §8 What this directive does NOT authorize

- Any modification to `src/state/typed_tx.rs` (STEP_B; not needed for Phase 2 by §2-§3 design).
- Any modification to `src/bottom_white/cas/schema.rs` (STEP_B; ObjectType unchanged).
- Any modification to `src/state/sequencer.rs` (STEP_B; no admission logic change required since `step_partial_ok` remains CAS-only per R3 §1.3).
- Any modification to historical M1 / R6 / R7 evidence files (per `feedback_no_retroactive_evidence_rewrite`).
- Any benchmark scale-up, M2/M3 advance, or NodeMarket / Polymarket / TB-19 / public-chain / formal H-VPPU claim (all FROZEN per parent ruling §3 expanded FREEZE).
- Changes to the `verified: bool` field semantics or removal — that is a future TB.
- Changes to `pput_verified` / `ChainDerivedRunFacts` reads on `verified` — they continue to function unchanged.

If during implementation any of the above becomes load-bearing for Phase 2 success, **HALT and request explicit architect review** before proceeding.

---

## §9 Phase 3 sequencing

After Phase 2 PASSes `cargo test --workspace` and ships clean:

1. Author Phase 3 evidence runner (`handover/tests/scripts/run_tb_18r_phase_3_evidence.sh`) — single-problem rerun on the typed substrate for P38 + P49 + a M0 mini-batch (≤5 problems).
2. Run with `MAX_TRANSACTIONS=12` + `PER_PROBLEM_TIMEOUT_S=1800` (R9-derived budget).
3. Validate per-run:
   - `chain_attempt_count == evaluator_reported_tx_count` (R4 invariant equation evaluable + holds).
   - id44 / id45 / id46 all PASS on real evidence (assert_45 now typed; verdict_kind populated on every LeanResult).
   - LeanResult records carry the four canonical (kind ↔ legacy-fields) shapes correctly.
   - Dashboard substantive smoke still passes.
   - `verdict_kind = PartialAccepted` records on multi-iteration problems where step_partial_ok fired.
4. NO retroactive M1 evidence rewrite. Phase 3 evidence directory is fresh: `handover/evidence/tb_18r_phase_3_<timestamp>/`.

After Phase 3 evidence:

5. Author final dual-audit dispatch (round-3): `handover/audits/G2_TB_18R_ROUND_3_DUAL_AUDIT_DISPATCH_<date>.md`.
6. Conservative ranking VETO > CHALLENGE > PASS preserved.
7. After both auditors reply, compute merged verdict.
8. Architect explicit §8 sign-off on round-3 verdict + Phase 1+2+3 cumulative work — **this is the last gate**; user single-word inputs do NOT promote to §8.

---

## §10 Cross-references

- Architect ruling: `handover/directives/2026-05-06_TB18R_ROUND_2_ARCHITECT_RULING.md`
- R8–R12 ratification addendum: `handover/directives/2026-05-06_TB18R_R8_R12_RATIFICATION_ADDENDUM.md`
- FC-first analysis: `handover/directives/FC_FIRST_ANALYSIS_ASSERT45_PARTIAL_VERDICT_2026-05-06.md`
- R3 supersession OBS: `handover/alignment/OBS_TB18R_R3_PREFLIGHT_SUPERSESSION_2026-05-06.md`
- TB-18 delay post-mortem: `handover/post-mortems/ROOT_CAUSE_TB18_DELAY_2026-05-06.md`
- Round-2 dispatch: `handover/audits/G2_TB_18R_ROUND_2_DUAL_AUDIT_DISPATCH_2026-05-06.md`
- LeanResult source: `src/runtime/attempt_telemetry.rs:373-412`
- assert_45 source: `src/runtime/audit_assertions.rs:2580-2656`
- step_partial_ok emitter: `experiments/minif2f_v4/src/bin/evaluator.rs:3489-3527`
- Memory rules invoked:
  - `feedback_class4_cannot_hide_in_class3` (this directive is the explicit ratification)
  - `feedback_no_workarounds_strict_constitution` (Option B is typed, not workaround)
  - `feedback_fc_first_problem_handling` (FC-first analysis preceded this directive)
  - `feedback_no_fake_menus` (no choice menu surfaced; engineering judgment exercised within architect-ratified envelope)
  - `feedback_architect_deviation_stance` (explicit position taken on Option A vs B)
  - `feedback_no_retroactive_evidence_rewrite` (legacy CAS decode preserved; no retro rewrite)
  - `feedback_step_b_protocol` (no STEP_B file touched; STEP_B not engaged)

---

**End of Phase 2 remediation directive. Implementation authorized; orchestrator proceeds.**
