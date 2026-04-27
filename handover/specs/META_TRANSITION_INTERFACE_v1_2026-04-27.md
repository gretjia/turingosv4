# MetaTransitionInterface Trait Spec v1 (CO P3-prep.5)

> **Date**: 2026-04-27
> **Purpose**: Plan v3.2 CO P3-prep.5 — define the Rust trait that v4.1 runtime ArchitectAI/JudgeAI actors will implement; v4 ships the trait + zero implementors. Doc-only design ensures v4.1 can plug in without re-architecting.
> **Authority**: WP architecture § 12 + § 17; Constitution Art V.1.2 + V.1.3.
> **Status**: v4 deliverable; v4.1 implements.

---

## § 1 Why a Trait

D-VETO-4 ratified resolution: v4 defers runtime MetaTape to v4.1, but ships **Phase 3 prep artifacts** that lock the contract. A Rust trait is the strongest contract form in the v4 type system: any v4.1 actor that implements `MetaTransitionInterface` is automatically wire-compatible with v4 substrate.

Without a trait: v4.1 might invent its own actor signatures, requiring v4.0 → v4.1 bridge code.
With a trait: v4.1 actors plug into v4.0's `governance::*` registration points; no bridge.

---

## § 2 Trait Definition

```rust
// src/governance/meta_transition_interface.rs (NEW per CO P3-prep.5; v4 ships zero implementors)
//
// /// TRACE_MATRIX WP-arch-§12 + Const-Art-V.1.2/3: MetaTransitionInterface
//
// This trait defines the API surface that v4.1+ runtime ArchitectAI and
// JudgeAI actors MUST implement. v4 produces this trait file with no impls;
// v4 governance flows go through `meta_validator::validate_meta_proposal`
// (offline library, called via cp workflow). v4.1 spawns runtime actors
// that wrap meta_validator + emit MetaTx to L4.

use crate::state::q_state::QState;
use crate::governance::meta_tx::{MetaTx, MetaProposalDraft, ValidatorVerdict};

/// Object-level Constitution Art V.1.2 ArchitectAI: proposes architectural changes.
///
/// v4: ArchitectAI is a HUMAN+CLAUDE workflow that produces MetaProposalDraft
/// CAS objects. No runtime impl; the human runs `cargo run --bin architect_propose`
/// to validate + persist drafts.
///
/// v4.1: ArchitectAI becomes a runtime actor that observes Q_t, proposes when
/// thresholds are crossed, and emits MetaTx via L4.
pub trait ArchitectActor: Send + Sync {
    /// Observe current Q_t and decide whether to propose.
    /// Returns None if no proposal needed.
    /// v4: returns None (offline only).
    /// v4.1: implements observability + proposal logic.
    fn observe_and_decide(&mut self, q: &QState) -> Option<MetaProposalDraft>;

    /// Validate a proposal against current Q_t before submission.
    /// Wraps `meta_validator::validate_meta_proposal`.
    fn validate_before_submit(&self, draft: &MetaProposalDraft, q: &QState) -> ValidatorVerdict;

    /// Persist proposal as L3 CAS object (v4) or emit MetaTx to L4 (v4.1).
    fn submit(&mut self, draft: MetaProposalDraft, q: &QState) -> Result<MetaTx, ProposalError>;

    /// Identity: return ArchitectAI's identifier (signing key, etc.).
    fn architect_id(&self) -> ArchitectId;

    /// Sign a proposal with ArchitectAI's authority.
    fn sign_proposal(&self, draft: &MetaProposalDraft) -> ArchitectSignature;
}

/// Object-level Constitution Art V.1.3 Veto-AI / JudgeAI: vetoes architectural changes.
///
/// v4: JudgeAI is the dual external audit (Codex + Gemini per Protocol).
/// v4.1: JudgeAI becomes a runtime actor with its own keypair that emits Approved/Vetoed L4 entries.
pub trait JudgeActor: Send + Sync {
    /// Review a submitted MetaTx against constitution + current Q_t.
    /// Returns PASS / VETO / SKIP (judge declines this particular case).
    fn review(&mut self, meta_tx: &MetaTx, q: &QState) -> JudgeVerdict;

    /// Identity.
    fn judge_id(&self) -> JudgeId;

    /// Sign verdict with JudgeAI's authority.
    fn sign_verdict(&self, meta_tx: &MetaTx, verdict: &JudgeVerdict) -> JudgeSignature;
}

pub enum JudgeVerdict {
    Pass,
    Veto(VetoReason),
    Skip,    // judge declines (e.g., conflict of interest); other judges still review
}

/// Coordinator that orchestrates ArchitectActor + JudgeActor + human signature flow.
///
/// v4: zero implementations.
/// v4.1: implements `RuntimeMetaCoordinator` that runs ArchitectAI proposal loops,
/// distributes proposals to N JudgeActor instances, applies on M-of-N approvals,
/// + human signature gate for constitutional changes.
pub trait MetaCoordinator: Send + Sync {
    /// Single tick: observe Q_t, run ArchitectAI, run all JudgeAIs, decide outcome.
    fn tick(&mut self, q: &QState) -> Result<MetaTickOutcome, CoordinatorError>;

    /// Apply approved MetaTx to Q_t (mutates predicate registry / tool registry).
    fn apply_approved(&mut self, meta_tx: &MetaTx, q: &mut QState) -> Result<(), ApplyError>;

    /// Configuration: list registered ArchitectActor + JudgeActor instances.
    fn registered_actors(&self) -> &MetaActorRegistry;
}

pub enum MetaTickOutcome {
    NoProposal,                                  // ArchitectAI didn't propose
    ProposalPending(MetaTx),                     // proposal submitted, awaiting judges
    ApprovedAndApplied(MetaTx, EpochAfterChange),
    Vetoed(MetaTx, Vec<VetoReason>),
    HumanReviewRequired(MetaTx),                 // constitutional change needs human PGP
}
```

---

## § 3 v4 vs v4.1 Implementation Status

### v4 (current scope)

```rust
// src/governance/meta_transition_interface.rs — TRAIT ONLY, zero implementations.
// Visible APIs:
//   pub trait ArchitectActor { ... }
//   pub trait JudgeActor { ... }
//   pub trait MetaCoordinator { ... }
//
// All three traits compile but have no impl blocks in v4.
// Use sites: `meta_validator::validate_meta_proposal` is callable directly without an ArchitectActor instance.
//
// Conformance test (v4 only):
//   tests/meta_transition_interface_compiles.rs  // verifies traits are well-formed Rust
//   tests/meta_validator_correctness.rs         // exercises meta_validator (non-trait path)
```

### v4.1 (future scope)

```rust
// src/governance/runtime_architect.rs — RuntimeArchitectActor implements ArchitectActor
// src/governance/runtime_judge.rs       — RuntimeJudgeActor implements JudgeActor
// src/governance/runtime_meta_coordinator.rs — RuntimeMetaCoordinator implements MetaCoordinator
//
// New tests (v4.1):
//   tests/runtime_architect_proposal_loop.rs
//   tests/runtime_judge_quorum.rs
//   tests/runtime_meta_coordinator_tick.rs
//   tests/runtime_meta_full_lifecycle.rs    // end-to-end: proposal → judges → apply
//   tests/runtime_meta_constitutional_human_sign.rs
```

The v4 trait file becomes the **plug-in surface** for v4.1; no rewrite needed.

---

## § 4 Identity Types

```rust
// src/governance/identity.rs (NEW per CO P3-prep.5)

pub struct ArchitectId(pub PublicKey);
pub struct JudgeId(pub PublicKey);

pub struct ArchitectSignature {
    pub architect_id: ArchitectId,
    pub signature_bytes: [u8; 64],
}

pub struct JudgeSignature {
    pub judge_id: JudgeId,
    pub signature_bytes: [u8; 64],
}

pub struct MetaActorRegistry {
    pub architects: BTreeMap<ArchitectId, ArchitectMetadata>,
    pub judges: BTreeMap<JudgeId, JudgeMetadata>,
    pub quorum_required: usize,    // e.g., M-of-N approvals
}

pub struct ArchitectMetadata {
    pub registered_at: TxId,        // L4 entry registering this actor
    pub revoked_at: Option<TxId>,
    pub specialization: Option<String>,    // e.g., "L1 predicates", "L4 ledger"
}

pub struct JudgeMetadata {
    pub registered_at: TxId,
    pub revoked_at: Option<TxId>,
    pub specialization: Option<String>,
    pub safety_or_creation: SafetyOrCreation,    // judge focus
}
```

These types compile in v4 + are used by `meta_validator` library in v4 (offline). v4.1 RuntimeArchitectActor / RuntimeJudgeActor instantiate them with actor-specific keypairs.

---

## § 5 Why Traits + Why Now

**Alternative considered + rejected**: ship v4 without trait, define types in v4.1.
- Risk: v4.1 invents incompatible signatures requiring bridge code
- Risk: v4 substrate has no anchor for "where ArchitectAI plugs in"
- Risk: v4 audit cannot validate v4.1 plugability claim

**Trait now achieves**:
- v4 audit can verify traits are well-formed Rust + cover Constitution Art V.1.2/3 separation
- v4.1 implementers have a **typed contract** to satisfy
- v4 governance flows (offline meta_validator) can reference trait types without instantiating

**Trait does NOT lock v4.1 into specific actor architectures**:
- Multiple ArchitectActor implementations are allowed (specialization by domain)
- Multiple JudgeActor implementations are allowed (M-of-N quorum)
- MetaCoordinator can be reimplemented with different scheduling strategies

---

## § 6 Conformance Test (v4 only)

```rust
// tests/meta_transition_interface_compiles.rs
//
// Asserts:
//   - The 3 traits are well-formed (object-safe where applicable; no method has Self return where forbidden)
//   - Identity types serialize/deserialize round-trip
//   - JudgeVerdict + MetaTickOutcome enums cover all spec'd cases (exhaustive match check)
//   - No v4 implementations exist (cargo-deny rule against `impl ArchitectActor for *`)
//   - meta_validator can be called without any trait impl (verifying offline-only path works)

#[test]
fn architect_actor_trait_object_safe() {
    fn _assert_object_safe<T: ArchitectActor + ?Sized>() {}
    _assert_object_safe::<dyn ArchitectActor>();
}

#[test]
fn judge_verdict_exhaustive() {
    let v = JudgeVerdict::Pass;
    match v {
        JudgeVerdict::Pass => {}
        JudgeVerdict::Veto(_) => {}
        JudgeVerdict::Skip => {}
    }
    // any new variant added would break this test (compile-time exhaustive check)
}

#[test]
fn no_v4_implementations() {
    // Compile-time: search src/ for `impl ArchitectActor for `; expect 0 results
    let src = std::fs::read_dir("src/").unwrap();
    let count = walkdir::WalkDir::new("src/")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().map_or(false, |x| x == "rs"))
        .filter(|e| {
            let s = std::fs::read_to_string(e.path()).unwrap_or_default();
            s.contains("impl ArchitectActor for") || s.contains("impl JudgeActor for") || s.contains("impl MetaCoordinator for")
        })
        .count();
    assert_eq!(count, 0, "v4 must have ZERO MetaTransitionInterface implementations");
}
```

---

## § 7 Honest Acknowledgements

What this trait spec achieves:
- Closes Plan v3.2 CO P3-prep.5 atom (concrete deliverable per Gemini Q7 demand)
- v4 audit can verify trait definition is correct + coverage of Const Art V.1.2/3
- v4.1 implementers have typed contract; no bridge code

What this spec is honest about:
- The trait is intentionally generic (multiple ArchitectActors, multiple JudgeActors); it does NOT prescribe v4.1's concurrency model
- `apply_approved` mutates `&mut QState` — in v4.1 this is the L4 acceptance flow; v4 does NOT call apply_approved (proposals stay as MetaProposalDraft CAS objects)
- "v4 has zero implementations" is a hard rule enforced by `tests/no_v4_implementations.rs`; if any v4 atom adds an impl, the test fails

What this spec does NOT do:
- Define how v4.1 actors are deployed (single process? multi-process? cloud?)
- Define how M-of-N judge quorum is computed (left to v4.1 RuntimeMetaCoordinator)
- Pin a specific signature algorithm beyond what `governance::identity::*` types declare

— ArchitectAI, 2026-04-27
