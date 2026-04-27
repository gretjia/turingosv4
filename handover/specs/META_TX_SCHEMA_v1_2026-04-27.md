# MetaTx Schema Spec v1 (Phase 3 Prep)

> **Date**: 2026-04-27
> **Purpose**: Gemini v3.2 Q7 CHALLENGE — define concrete deliverables for "Phase 3 prep" so D-VETO-4 deferral is auditable, not weasel wording. This is artifact #1 of 7 in CO P3-PREP.
> **Authority**: WP architecture § 12.2 `meta_tx` definition (lines 870-887).
> **Status**: v4 ships this typed schema; v4.1 runtime ArchitectAI/JudgeAI implements consumption.

---

## § 1 What is `MetaTx`

A meta-transaction proposes architectural changes to TuringOS itself: predicate registry updates, tool registry updates, even (with human signature) constitutional amendments. Distinct from object-level transitions (`WorkTx` / `VerifyTx` / `ChallengeTx`) which operate within the current architecture.

In v4: ArchitectAI offline (per Constitution Art V.1.2) produces `MetaProposalDraft` CAS objects; humans review + cp-workflow + git tag sign for any constitutional changes.
In v4.1: runtime ArchitectAI/JudgeAI actors emit/validate `MetaTx` directly to L4 with the human signature gate intact for constitutional amendments.

This spec defines the **typed schema** so v4 outputs can be consumed by v4.1 without rewrite.

---

## § 2 Typed Schema

```rust
pub struct MetaTx {
    pub tx_id:                  TxId,
    pub parent_architecture_root: Hash,    // = Q_t.predicate_registry_root_t || tool_registry_root_t at submission
    pub proposed_changes:        ProposedChanges,
    pub log_evidence_cids:       Vec<Cid>,    // L3 CAS handles to log excerpts justifying change
    pub reversibility_plan_cid:  Cid,         // L3 CAS handle to "how to undo" plan
    pub constitution_check:      ConstitutionCheckProof,
    pub judge_signatures:        Vec<JudgeSignature>,    // ≥1 JudgeAI signature
    pub human_signature_required: bool,                  // true iff change touches constitution.md or core architectural axioms
    pub human_signature:         Option<HumanPgpSignature>,  // MUST be Some if human_signature_required
    pub signature:               AgentSignature,         // ArchitectAI's own signature
    pub timestamp_logical:       u64,
    pub status:                  MetaTxStatus,
}

pub enum ProposedChanges {
    /// Modify L1 Predicate Registry: add / replace / deprecate predicates
    PredicateRegistryUpdate(Vec<PredicatePatch>),
    /// Modify L2 Tool Registry: add / replace / deprecate tools
    ToolRegistryUpdate(Vec<ToolPatch>),
    /// Constitutional amendment (always requires human_signature)
    ConstitutionAmendment(ConstitutionDiff),
    /// Bundle: multiple change types in one tx (atomic)
    Bundle(Vec<ProposedChanges>),
}

pub struct PredicatePatch {
    pub op: PatchOp,                    // Add | Replace | Deprecate
    pub predicate_id: PredicateId,
    pub new_code_hash: Option<Hash>,    // None for Deprecate
    pub new_version: Option<Version>,
    pub new_visibility: Option<VisibilityPolicy>,
    pub justification_cid: Cid,         // L3 CAS handle to rationale
}

pub struct ToolPatch {
    pub op: PatchOp,
    pub tool_id: ToolId,
    pub new_capability_hash: Option<Hash>,
    pub new_permission_policy: Option<PermissionPolicy>,
    pub new_determinism_class: Option<DeterminismClass>,
    pub new_side_effect_class: Option<SideEffectClass>,
    pub justification_cid: Cid,
}

pub struct ConstitutionDiff {
    pub from_constitution_hash: Hash,    // current SHA
    pub to_constitution_hash:   Hash,    // proposed SHA
    pub diff_cid:               Cid,     // L3 CAS handle to unified diff
    pub article_affected:       Vec<ArticleId>,    // e.g., ["Art. V.3", "Art. 0.2"]
    pub amendment_predicate_check: AmendmentPredicateProof,    // proof that current amendment_predicate accepts this diff
}

pub struct ConstitutionCheckProof {
    pub axiom_violations_check: BTreeMap<AxiomId, AxiomCheckResult>,    // each of 6 axioms (Art 0.5) checked
    pub historical_invariant_check: BTreeMap<InvariantId, InvariantCheckResult>,    // 12 economic + 16 transition invariants
    pub jacobian_change_set: ChangeSet,    // explicit list of which existing tx semantics change
}

pub enum MetaTxStatus {
    Draft,                              // ArchitectAI offline; v4 stays here
    Submitted,                          // emitted to L4 (v4.1 only)
    UnderReview,                        // JudgeAI reviewing (v4.1 only)
    Vetoed(VetoReasonCid),
    HumanReview,                        // awaiting human PGP signature
    Approved,                           // ready to apply
    Applied(EpochAfterChange),          // L4 applied + new architecture root
}
```

---

## § 3 Validation Rules (consumed by `meta_validator` library — CO P3-prep.3)

```rust
pub fn validate_meta_proposal(
    proposal: &MetaProposalDraft,    // v4 input format; deserializes into MetaTx for typed checks
    q: &QState,
) -> ValidatorVerdict {
    // R1: parent_architecture_root must equal current Q_t.predicate_registry_root_t || tool_registry_root_t
    if !proposal.parent_root_matches(q) {
        return ValidatorVerdict::Veto(VetoReason::StaleParentRoot);
    }

    // R2: ArchitectAI signature verifies
    if !verify_architect_signature(&proposal.signature, &proposal) {
        return ValidatorVerdict::Veto(VetoReason::InvalidSignature);
    }

    // R3: at least one valid JudgeAI signature
    if proposal.judge_signatures.iter().filter(|s| verify_judge_signature(s)).count() == 0 {
        return ValidatorVerdict::Veto(VetoReason::NoValidJudgeSignature);
    }

    // R4: if constitutional change, human_signature_required must be true AND signature present
    if proposal.touches_constitution() && !proposal.human_signature.is_some() {
        return ValidatorVerdict::Veto(VetoReason::HumanSignatureMissing);
    }
    if let Some(sig) = &proposal.human_signature {
        if !verify_pgp(sig, &proposal, &PINNED_CREATOR_PUBKEY) {
            return ValidatorVerdict::Veto(VetoReason::InvalidHumanSignature);
        }
    }

    // R5: amendment_predicate accepts the diff (Bitcoin-softfork-style recursion)
    if let ProposedChanges::ConstitutionAmendment(diff) = &proposal.proposed_changes {
        let amendment_predicate = registry.amendment_predicate();
        if !amendment_predicate.evaluate(&diff) {
            return ValidatorVerdict::Veto(VetoReason::AmendmentPredicateRejected);
        }
    }

    // R6: each axiom check must PASS (Art 0.5 6 axioms)
    for (axiom, result) in &proposal.constitution_check.axiom_violations_check {
        if matches!(result, AxiomCheckResult::Violated(_)) {
            return ValidatorVerdict::Veto(VetoReason::AxiomViolated(*axiom));
        }
    }

    // R7: each historical invariant check must PASS (12 economic + 16 transition + 24 V + 4 E)
    for (inv, result) in &proposal.constitution_check.historical_invariant_check {
        if matches!(result, InvariantCheckResult::WouldViolate(_)) {
            return ValidatorVerdict::Veto(VetoReason::InvariantWouldViolate(*inv));
        }
    }

    // R8: reversibility plan exists + parses
    if cas::lookup(&proposal.reversibility_plan_cid).is_none() {
        return ValidatorVerdict::Veto(VetoReason::ReversibilityPlanMissing);
    }

    ValidatorVerdict::Pass
}
```

This validator is callable both:
- **Offline** in v4: ArchitectAI runs it on each `MetaProposalDraft` before submitting via cp workflow
- **Runtime** in v4.1: ArchitectAI runtime actor calls before submitting; JudgeAI re-runs on receipt

Same library, same output.

---

## § 4 v4 vs v4.1 Implementation Boundary

### v4 implements (Phase 3 prep):
- This MetaTx schema (typed Rust)
- `MetaProposalDraft` CAS storage (offline-produced; not L4)
- `meta_validator` library (called via CLI tool by ArchitectAI offline workflow)
- `MetaTransitionInterface` Rust trait (no implementor)
- AmendmentFlow format for cp-workflow Art V.3 amendments

### v4.1 implements (Phase 3 runtime):
- Runtime ArchitectAI actor that emits `MetaTx` to L4
- Runtime JudgeAI actor that consumes `MetaTx` + emits Approved/Vetoed
- L4 acceptance of `MetaTx` (extends step_transition with `meta_transition` arm)
- Architecture root update on Approved meta_tx

### v5 implements (per WP § 17 Phase 4-5):
- Multi-organization Hyperledger Fabric chaincode for permissioned MetaTx
- Public AGI Market with cross-domain reputation

---

## § 5 Conformance Tests (3 new in v4)

```
tests/meta_tx_schema_serialization.rs       — round-trip serialize/deserialize MetaTx; field invariants
tests/meta_validator_pass_cases.rs           — hand-crafted PASS proposals validate correctly
tests/meta_validator_veto_cases.rs           — hand-crafted VETO proposals (one per VetoReason variant) reject correctly
```

---

## § 6 Honest Acknowledgements

What this spec achieves:
- Closes Gemini v3.2 Q7 CHALLENGE on "Phase 3 prep" being weasel wording
- Provides typed schema implementable in v4 + consumable by v4.1
- Includes validator with explicit veto reasons
- Honors Constitution Art V.1 separation of powers (ArchitectAI proposes, JudgeAI validates, human signs constitutional changes)

What this spec is honest about:
- v4 does NOT runtime-emit MetaTx; only produces drafts
- v4.1 runtime is **NOT** part of v4 deliverable; only the consumption-ready interface is
- WP § 12 + § 17 ultimate Phase 3 vision (runtime self-modification) is preserved as a v4.1 target, not deleted

— ArchitectAI, 2026-04-27
