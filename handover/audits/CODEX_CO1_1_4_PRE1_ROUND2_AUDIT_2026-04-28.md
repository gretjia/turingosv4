# Codex CO1.1.4-pre1 Round-2 Audit
**Date**: 2026-04-28
**Target**: spec v1.1 + impl v1.1 + 17 tests joint artifact (closure verification)
**Prompt size**: 252084 chars

---

Reading prompt from stdin...
OpenAI Codex v0.125.0 (research preview)
--------
workdir: /home/zephryj/projects/turingosv4
model: gpt-5.5
provider: openai
approval: never
sandbox: danger-full-access
reasoning effort: xhigh
reasoning summaries: none
session id: 019dd3f0-80ea-7763-a21e-6bb5666e3411
--------
user
# Codex Round-2 Audit — CO1.1.4-pre1 Typed Tx ABI v1.1 (post round-1 CHALLENGE)

**Role**: skeptical adversarial implementer-reviewer. Independent of Gemini round-2 (running in parallel).

**Mandate**: round-2 closure-verification audit on v1.1 joint artifact (spec + impl + 17 tests). Round-1 returned CHALLENGE/CHALLENGE (verdict at `handover/audits/CO1_1_4_PRE1_DUAL_AUDIT_VERDICT_R1_2026-04-28.md`); v1.1 (commit `e0e4565`) claims to close 10 patches (P1-P10).

Per CLAUDE.md "Audit Standard": Generator ≠ Evaluator. Per memory `feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS.

## What changed v1 → v1.1 (per patch log)

| ID | v1 issue (round-1 finding) | v1.1 fix claim | What to verify |
|---|---|---|---|
| **P1** (C-1) | Agent-sig domain separation missing | NEW signing-payload structs + b"turingosv4.<actor>.<purpose>.v1" domain prefixes; to_signing_payload() projection on each tx | Cross-domain digests truly distinct? Domain prefix actually included in SHA-256 input? Signature excluded from signed digest? |
| **P2** (C-3 Codex Q-B) | claim_id: TxId leaked impl | NEW ClaimId(pub TxId) #[serde(transparent)] newtype | Wire-identical to TxId? Type-distinction at API surface? |
| **P3** (C-3 Codex Q-C) | TerminalSummaryTx 3-field placeholder | Migrated to 8-field STATE § 1.5 schema in state::typed_tx; system_keypair signs opaque [u8;32] via NEW CanonicalMessage::TerminalSummarySigning variant | Full schema present? bottom_white ↔ state circular dep eliminated? Old struct fully removed? Sig API correct? |
| **P4** (CX-1 Codex Q-G) | TransitionError 10 variants insufficient | Expanded to 22 variants per STATE § 3 pseudocode | All variants STATE § 3 invokes covered? NotYetImplemented retained as stub sentinel only? |
| **P5** (C-2 both auditors) | Golden fixtures unlocked + TerminalSummary missing | Hardcoded SHA-256 hex for all 7 variants; +6 new tests (cross-variant non-collision, BTreeSet permutation, default round-trip, signing-payload domain distinctness, signing-payload-excludes-signature, TerminalSummary in round-trip+kind) | Hex actually locked (not just length=64)? All 7 variants covered? Permutation test catches HashMap-style hazard? |
| **P6** (CX-2 Codex Q-D) | STATE § 2.5 wording wrong | spec § 7.1 codec wording fixed (u32 BE variants, u64 BE lengths); codec unchanged | Wording accurate now? Cross-references valid? Decision to keep u32/u64 (not force u8) defensible? |
| **P7** (C-3 followup) | D-3 divergence | RESOLVED via P3 | D-3 row fully removed from § 9 (not just edited)? |
| **P8** (C-3 + GM-2) | FinalizeRewardTx Q-derived discipline + dual-sign rationale | NEW § 4.1 Q-derived (task_id/solver/reward authoritative from Q at replay; royalty NOT on wire); NEW § 4.2 dual-sign (this struct's sig binds payload bytes; envelope sig binds sequencer-stamped bytes; both needed) | Q-derived discipline clearly stated? Replay rule (CO1.7-impl A4) committed? Dual-sign rationale convincing or still redundant? |
| **P9** (GM-1 Gemini Q4) | Cold-replay Art 0.2 | NEW § 0.1 cross-atom ordering gate: v1.1 PASS contingent on CO1.4-extra shipping BEFORE CO1.7-impl A4 | Ordering gate explicit + binding? Constitutional commitment strong enough? |
| **P10** (CX-3 Codex Q-J) | TaskId vs TxId QState mismatch | NEW § 9 D-4: cross-atom debt assigned to CO P2.1; no retrofit in this atom | D-4 sufficiently honest? Future-migration plan clear? Wire-format consequence correctly assessed (none)? |

## Your previous (round-1) verdict

You returned **CHALLENGE** with high conviction. Top 3 must-fix:
1. Replace TerminalSummaryTx with 8-field STATE schema + add tests/fixtures
2. Fix canonical serialization documentation/tests: hardcode golden hex, include all variants, document actual bincode u32 enum/u64 length behavior
3. Add ClaimId, complete TransitionError, define signature-domain signing payloads excluding signature fields

Per-question PASS items: Q-A (D-1 TxStatus elision PASS w/ patch note); Q-H (HasSubmitter correct); Q-I (atom scope creep PASS w/ caveat).

## Round-2 closure verification questions

**Q-1. P1 closure (agent-sig domain separation)**: Cite typed_tx.rs lines for the 6 SigningPayload structs + canonical_digest impls. Is `domain_prefixed_digest` correct (sha256(domain || canonical_encode(self)))? Does signing_payload_excludes_signature test prove what it should? Cross-domain non-collision (signing_payload_domains_are_distinct) sufficient or trivial-pass? Any new defect?

**Q-2. P2 closure (ClaimId newtype)**: `#[serde(transparent)]` actually wire-identical to TxId? FinalizeRewardTx.claim_id field updated? Fixture updated to use ClaimId::new()? Any other missed call site?

**Q-3. P3 closure (TerminalSummaryTx migration)**: Old 3-field struct fully removed from system_keypair.rs (not just commented out)? New 8-field struct in typed_tx.rs has correct serde derives? CanonicalMessage::TerminalSummarySigning([u8;32]) variant added correctly + canonical_digest match arm updated? terminal_summary_emitter::sign_terminal_summary takes [u8;32] (not the struct)? bottom_white ↔ state circular dep verifiably absent (grep)?

**Q-4. P4 closure (TransitionError taxonomy)**: 22 variants enough? Each STATE § 3.1-3.7 pseudocode error invocation has a corresponding variant? Are payloads (PredicateId for *PredicateFailed) right or should they be richer (PredicateResultsBundle)?

**Q-5. P5 closure (golden fixtures + new tests)**: Hex constants actually load-bearing (assertion fails on diff)? cargo test --lib confirms 17/17 PASS? TerminalSummary in round_trip_all_variants + tx_kind_projection + golden? Cross-variant non-collision pairwise (7×7) or just smoke? BTreeSet permutation test would catch a HashMap-style codec bug?

**Q-6. P6 closure (codec wording)**: spec § 7.1 wording now matches bincode-2.0.1 source citations? Decision to keep u32/u64 (not force u8) — is that the right call given >256 variant ceiling never likely to be hit? Any cascaded impact on shipping CO1.7 LedgerEntrySigningPayload digest stability? (CO1.7 commit a03cc52 used same canonical_encode helper.)

**Q-7. P8 + P9 closure (Q-derived + dual-sign + cold-replay gate)**: Spec § 4.1 commits replay (CO1.7-impl A4) to Q-derived discipline — is this binding enough? Spec § 0.1 cross-atom ordering gate phrased as "MUST NOT ship before CO1.4-extra" — strong constitutional commitment or hedged language?

**Q-8. P10 closure (D-4 TaskId/TxId)**: Adequate honesty about the cross-atom debt? Any wire-format consequence I'm missing? CO P2.1 the right atom owner?

**Q-9. New defects in v1.1 (independent of P1-P10)**:
- Test gaps: any test class still missing (e.g. signing-payload round-trip, signing-payload golden hex)?
- Type errors that cargo check missed?
- Spec ↔ code parity drift introduced by v1.1 patches?
- Imports: does `use sha2::{Digest, Sha256}` collision-free with existing usage?
- Missing Default impls causing panics in #[derive(Default)] expansion?

**Q-10. Implementation gating (overall)**: with v1.1 closure, is CO1.7-impl A2 (TypedTx + Sequencer + dispatch_transition) implementable end-to-end against this ABI surface? Specific blockers to call out.

## Output format

# Codex CO1.1.4-pre1 Round-2 Audit
## Q-1 P1 closure
## Q-2 P2 closure
## Q-3 P3 closure
## Q-4 P4 closure
## Q-5 P5 closure
## Q-6 P6 closure
## Q-7 P8+P9 closure
## Q-8 P10 closure
## Q-9 New defects
## Q-10 Implementation gating
## **VERDICT**: PASS / CHALLENGE / VETO
## Top must-fix (if CHALLENGE) — be specific
## Conviction (low/med/high)

Be rigorous. Cite spec § + code line numbers. Per memory `feedback_dual_audit_conflict`: do NOT pass on principle; do NOT veto on principle. Real defects = CHALLENGE; foundational design flaw = VETO; clean closure = PASS.

---


# CO1.1.4-pre1 spec v1.1 (target of audit)

# CO1.1.4-pre1 — Typed Tx ABI Surface (v1.1)

**Status**: v1.1 — round-1 dual audit returned CHALLENGE/CHALLENGE; this version closes 10 patches (P1-P10) per the merged verdict (`handover/audits/CO1_1_4_PRE1_DUAL_AUDIT_VERDICT_R1_2026-04-28.md`). Awaiting round-2.
**Status (v1)**: v1 DRAFT, post-CO1.7 PASS/PASS gate (2026-04-28).
**Author**: ArchitectAI (Claude); session 2026-04-28 (continued).
**Round-1 verdicts**: `handover/audits/CODEX_CO1_1_4_PRE1_ROUND1_AUDIT_2026-04-28.md` (CHALLENGE/high) + `handover/audits/GEMINI_CO1_1_4_PRE1_ROUND1_AUDIT_2026-04-28.md` (CHALLENGE/high); merged in `handover/audits/CO1_1_4_PRE1_DUAL_AUDIT_VERDICT_R1_2026-04-28.md`.

## v1.1 patch log (vs. v1) — round-1 closure

| ID | v1 issue | v1.1 fix | Source |
|---|---|---|---|
| **P1** | AgentSignature reused 64-byte adapter without domain separation; comments implied "exclude signature" digest with no signing payload | NEW signing-payload structs (`WorkSigningPayload` / `VerifySigningPayload` / `ChallengeSigningPayload` / `FinalizeRewardSigningPayload` / `TaskExpireSigningPayload` / `TerminalSummarySigningPayload`) — each has explicit domain prefix (`b"turingosv4.<actor>.<purpose>.v1"`) prepended to bincode body bytes in `canonical_digest()`. Plus `to_signing_payload()` projection on each tx. | C-1 (Codex Q-E + Gemini Q7) |
| **P2** | `FinalizeRewardTx.claim_id: TxId` reused TxId, leaking ClaimsIndex impl into wire format | New `ClaimId(pub TxId)` newtype with `#[serde(transparent)]` (wire-identical to TxId; non-breaking); `FinalizeRewardTx.claim_id: ClaimId` now | C-3 (Codex Q-B) |
| **P3** | `TerminalSummaryTx` was 3-field placeholder living in `system_keypair.rs` (versus STATE § 1.5 8-field schema); locking the wrong shape into ABI | Migrated to `state::typed_tx::TerminalSummaryTx` with full 8-field STATE schema (tx_id / task_id / run_id / run_outcome / total_attempts / failure_class_histogram / last_logical_t / system_signature). `system_keypair` now signs an opaque `TerminalSummarySigning([u8; 32])` digest (same opaque-digest pattern as `LedgerEntrySigning`) — no `bottom_white ↔ state` circular dep. | C-3 (Codex Q-C must-fix-now) |
| **P4** | `TransitionError` had only 10 variants; STATE § 3 pseudocode invokes ~22 | Expanded to 22 variants: SignatureInvalid / StakeInsufficient / TargetWorkTxNotFound / TargetWorkTxNotVerifiable / ParentNotAcceptedYet / AcceptancePredicateFailed(PredicateId) / VerificationPredicateFailed(PredicateId) / SettlementPredicateFailed(PredicateId) / ChallengeWindowClosed / CounterexampleInsufficient / ToolNotInRegistry / ToolCreatorMismatch + 10 prior. Plus `NotYetImplemented` retained as explicit stub sentinel. | CX-1 (Codex Q-G) |
| **P5** | "Phase 1 record-only" golden fixture tests asserted only length=64 + self-stability, did NOT lock SHA-256 hex; `TerminalSummary` excluded from round-trip / kind / golden tests | Hardcoded SHA-256 hex constants for all 7 TypedTx fixture digests (Work / Verify / Challenge / Reuse / FinalizeReward / TaskExpire / TerminalSummary). NEW tests: cross-variant non-collision (7×7 pairwise distinct), BTreeSet permutation independence, default round-trip, signing-payload domain non-collision (6 distinct domain digests), signing-payload-excludes-signature (mutating tx.signature must NOT affect digest). All variants now in round-trip + kind-projection. Total typed_tx tests: 11 → 17. | C-2 (Codex Q-J + Gemini Q9) |
| **P6** | STATE § 2.5 wording wrong vs actual codec — claimed `#[repr(u8)]`-controlled enum discriminants; bincode-2 actually emits u32 BE for variants and u64 BE for lengths | This v1.1 spec § 2.5-bis explicitly documents the actual codec behavior + cross-references bincode-2 source (`bincode 2.0.1 src/features/serde/ser.rs:186`, `enc/impls.rs:68 + :128`). `#[repr(u8)]` is a Rust language attribute that does NOT control serde wire format. Recommendation accepted: keep u32 variants + u64 lengths (no codec change; spec language fixed). | CX-2 (Codex Q-D) |
| **P7** | D-3 TerminalSummaryTx field-set divergence | RESOLVED (P3 migrated to full schema). § 9 D-3 row removed. | C-3 followup |
| **P8** | FinalizeRewardTx had ambiguous {task_id, solver, reward, royalty} provenance + redundant system_signature unclear | This spec § 4 explicitly states {task_id, solver, reward} are **Q-DERIVED at replay** (re-fetched from ClaimsIndex by claim_id; wire fields are ledger summary, NOT trusted from wire); `system_signature` is RETAINED with explicit dual-sign rationale (this sig binds the tx-payload bytes; the L4 `LedgerEntrySigningPayload` sig binds the sequencer-stamped envelope; both are needed). | C-3 + GM-2 |
| **P9** | Cold-replay → Art 0.2 violation if CAS index not persisted | This spec § 0 NEW "Cross-Atom Ordering Gate": v1.1 PASS is contingent on CO1.4-extra (CAS index persistence) shipping BEFORE CO1.7-impl A4 (replay_full_transition). CO1.7-impl A2 (Sequencer apply path) and A3 (dispatch_transition stubs) may proceed; A4 BLOCKED on CO1.4-extra. | GM-1 (Gemini Q4) |
| **P10** | TaskId-vs-TxId QState index mismatch (typed_tx uses TaskId; QState `task_markets_t` / `escrows_t` / `stakes_t` keyed by TxId) | This spec § 9 NEW D-4 documents the forward-migration plan: CO P2.1 (TaskMarket atom) owns the QState retrofit; v1.1 records the migration debt + cross-atom dependency note. Does NOT perform the retrofit (out of CO1.1.4-pre1 scope; would touch q_state.rs which is its own atom). | CX-3 (Codex Q-J) |

10 patches integrated below.

---


**Why this atom exists**: spec § 2.5 of `STATE_TRANSITION_SPEC_v1_2026-04-27.md` explicitly deferred "full ABI surface for QState/SignalBundle/TransitionError" to CO1.7. CO1.7 spec § 0 places the per-kind tx schemas in `STATE_TRANSITION_SPEC § 1` ("frozen on paper, not yet in code"). When CO1.7-impl A1 (Git2LedgerWriter, commit `a03cc52`) shipped, downstream A2 (TypedTx + dispatch_transition) discovered ~30 supporting schema types are required but **none of them exist in code** — only `MicroCoin` is defined. This atom defines that ABI surface in isolation under its own dual-audit gate, per the project's per-atom audit principle (CLAUDE.md "Audit Standard").

**Companion**: `STATE_TRANSITION_SPEC_v1_2026-04-27.md` § 1 (typed schemas), § 2.5 (canonical serialization), § 3 (transition pseudocode — informs FinalizeRewardTx schema, see § 4 below).

**Single sentence**: define every supporting type + the 7 typed-tx variant payload structs + the `TypedTx` enum, with `Serialize/Deserialize` derives over the spec § 2.5 canonical encoding (bincode v2 BE + fixed_int), so that CO1.7-impl A2-A4 (Sequencer + dispatch_transition + replay_full_transition) can be implemented against a stable type surface.

---

## § 0 Scope

### In scope

1. **Identifier newtypes**: `TaskId`, `RunId`, `ToolId`, `PredicateId` (each opaque `String`).
2. **Read/Write set keys**: `ReadKey(String)`, `WriteKey(String)`.
3. **Agent signature**: `AgentSignature([u8; 64])` — Ed25519 detached signature, distinct from `SystemSignature` (system_keypair.rs).
4. **Predicate result types**: `BoolWithProof`, `PredicateResultsBundle`, `SafetyOrCreation`.
5. **Status / class enums**: `TxStatus`, `RejectionClass`, `VerifyVerdict`, `RunOutcome`.
6. **Slash evidence reference**: `SlashEvidenceCid(Cid)` newtype.
7. **Money newtype**: `StakeMicroCoin(MicroCoin)` (non-negative invariant enforced at business layer; type-level newtype prevents accidental mix with general `MicroCoin`).
8. **Typed-tx payload structs**: `WorkTx`, `VerifyTx`, `ChallengeTx`, `ReuseTx`, `FinalizeRewardTx`, `TaskExpireTx`. (`TerminalSummaryTx` already exists in `system_keypair.rs`.)
9. **Outer enum**: `pub enum TypedTx` with the 7 variants.
10. **Trait**: `pub trait HasSubmitter` per STATE spec § 3.6.5 v1.3.
11. **Conformance tests**: 1 golden fixture per main tx kind (input → known SHA-256 of canonical bytes) + 100-input round-trip + cross-call byte stability.

### Out of scope (explicit deferral)

- **MetaTx + ancillaries** (`PredicatePatch`, `ToolPatch`, `JudgeSignature`, `HumanSignature`, `ConstitutionCheckProof`, `ReversibilityPlan`) — STATE spec § 1.6 declares MetaTx is **v4.1 only**; v4 emits `MetaProposalDraft` to L3 CAS, not L4. ⏭ deferred.
- **Slash transition** — already deferred to CO P2.5 ChallengeCourt per CO1.7 spec K5.
- **Per-kind transition function bodies** (`step_transition`, `verify_transition`, `challenge_transition`, `reuse_transition`, `finalize_reward_transition`, `task_expire_transition`, `emit_terminal_summary_transition`) — these consume the ABI defined here; they belong to **CO1.7.5** (the body atom).
- **Sequencer + dispatch_transition + replay_full_transition** — these consume the ABI; they belong to CO1.7-impl **A2-A4** (post this atom).
- **`SignalBundle` typed shape** — STATE spec uses `SignalBundle::empty()` / `::finalize(...)` / `::task_expired(...)` / `::terminal_summary(...)` constructors. v1 of this atom emits a minimal typed `SignalBundle` (single enum-like discriminator + payload) sufficient for CO1.7-impl to compile; full event-stream design lands in CO1.9 L6 signal indices.
- **TransitionError full taxonomy** — v1 emits a minimal enum covering the variants invoked in spec § 3 pseudocode (`ClaimNotFound`, `ChallengeWindowStillOpen`, `AlreadySlashed`, `TaskNotFound`, `InvalidSystemSignature`, `StaleParent`, `TaskNotExpired`, `TaskHasOpenClaim`, `TerminalSummaryNotApplicable`, `NotYetImplemented`); per-stage enum proliferation is a CO1.7.5 concern.

### What this atom is NOT replacing

- `src/state/q_state.rs` (existing): keeps its existing types verbatim. CO1.1.4-pre1 only adds new types in `src/state/typed_tx.rs`.
- `src/economy/money.rs` (existing): unchanged. `StakeMicroCoin` is a **newtype on `MicroCoin`** living in `src/economy/money.rs` (additive).

### § 0.1 Cross-atom ordering gate (v1.1 NEW per Gemini Q4 round-1)

**Constitutional concern**: CO1.7 LedgerEntry stores typed-tx payloads in L3 CAS via `tx_payload_cid: Cid`. The current shipped `CasStore::open()` initializes an empty in-memory index (CO1.4 store.rs:67); after process restart the CAS bytes are unrecoverable until the index is repopulated. This means **cold-replay of L4 cannot reconstruct typed payloads** — a direct Art. 0.2 (tape canonicality) violation if uncorrected.

**Mitigation**: CAS index persistence is its own atom — **CO1.4-extra** — already named in CO1.7 spec § 0. CO1.4-extra adds index persistence (likely a sidecar JSONL or git-tag manifest) so cold-replay can recover payloads via `CasStore::get`.

**Hard ordering for v1.1 PASS**:
- CO1.7-impl A2 (Sequencer apply path) + A3 (dispatch_transition skeleton) may proceed against CO1.1.4-pre1 v1.1 PASS independently.
- **CO1.7-impl A4 (replay_full_transition) MUST NOT ship before CO1.4-extra**. Until then, FullTransition replay errors with `CasMissing` after process restart (already documented in CO1.7 spec § 4 / `ReplayError::CasMissing`).
- CO1.4-extra has its own dual-audit gate.

This ordering is a **necessary condition for CO1.1.4-pre1 PASS** per round-1 Gemini Q4; documented here so future audits cannot reinterpret silence as approval.

---

## § 1 Module layout

```
src/state/
├── mod.rs                       (existing; +pub mod typed_tx + re-exports)
├── q_state.rs                   (existing; unchanged)
└── typed_tx.rs                  (NEW; ~600-900 LoC; the ABI surface)

src/economy/
└── money.rs                     (existing; +pub struct StakeMicroCoin newtype + minimal impls)

src/bottom_white/ledger/
└── system_keypair.rs            (existing; serde_bytes_64 helper promoted to pub(crate)
                                  so AgentSignature can re-use the [u8; 64] adapter)
```

**Crate boundary**: `state::typed_tx` consumes (a) `state::q_state` types (Hash, AgentId, TxId, NodeId), (b) `economy::money::MicroCoin` + `StakeMicroCoin`, (c) `bottom_white::cas::schema::Cid`, (d) `bottom_white::ledger::system_keypair::{SystemEpoch, SystemSignature}`. No new outward dependencies; no circular dep risk.

---

## § 2 Identifier newtypes

```rust
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct TaskId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct RunId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct ToolId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct PredicateId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct ReadKey(pub String);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct WriteKey(pub String);
```

All identifiers are opaque strings to Q_t (per existing `AgentId` / `TxId` pattern in q_state.rs). Concrete derivation rules (e.g. `TxId::derive(run_id, "terminal")` per STATE § 3.7) live at the call sites, not in the type.

---

## § 3 AgentSignature, StakeMicroCoin, SlashEvidenceCid

```rust
/// Detached Ed25519 signature over a per-tx canonical_digest.
/// Distinct from SystemSignature (system-keypair signatures) at type level —
/// agent-vs-system signature confusion would be a security hazard.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentSignature(#[serde(with = "system_keypair::serde_bytes_64")] [u8; 64]);

/// Newtype on MicroCoin for stake fields. Non-negative is a runtime invariant
/// (not a type invariant) per Inv 3.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
#[serde(transparent)]
pub struct StakeMicroCoin(pub MicroCoin);

/// L3 CAS handle to slash evidence. Kept as a newtype (not a bare Cid) so the
/// FinalizedSlash variant of TxStatus can't accidentally accept arbitrary CIDs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
#[serde(transparent)]
pub struct SlashEvidenceCid(pub Cid);
```

---

## § 4 FinalizeRewardTx — derived schema

**Spec gap**: STATE_TRANSITION_SPEC § 3.4 uses `FinalizeTx::from(claim_id, reward)` constructor pattern but provides no explicit struct. CO1.7 spec § 1 lists `TxKind::FinalizeReward = 4` but defers the struct to "frozen in STATE_TRANSITION_SPEC § 1" — which the STATE spec doesn't actually contain.

**v1 derivation** (from § 3.4 call sites + the TaskExpireTx pattern in § 3.6, system-emitted):

```rust
pub struct FinalizeRewardTx {
    pub tx_id: TxId,                       //  1
    pub claim_id: ClaimId,                 //  2  TYPED newtype (v1.1 P2)
    pub task_id: TaskId,                   //  3  Q-DERIVED at replay; wire = ledger summary
    pub solver: AgentId,                   //  4  Q-DERIVED at replay; wire = ledger summary
    pub reward: MicroCoin,                 //  5  Q-DERIVED at replay (SettlementEngine output); wire = ledger summary
    pub parent_state_root: Hash,           //  6  must equal q.state_root_t at submission
    pub epoch: SystemEpoch,                //  7  which keypair signed
    pub timestamp_logical: u64,            //  8  monotonic
    pub system_signature: SystemSignature, //  9  system-emitted, see § 4.1 dual-sign rationale
}
```

### § 4.1 Q-derived vs wire-only fields (v1.1 NEW per Codex Q-B + Gemini Q6)

For `FinalizeRewardTx`, fields {`task_id`, `solver`, `reward`} are recorded on the wire as a **ledger summary** (so a human reading L4 can see the finalize event semantics + downstream tools without Q_t access can render the event). At replay, however, **the AUTHORITATIVE values come from `Q_t` lookups by `claim_id`**:
- `task_id` = `q.economic_state_t.claims_t[claim_id].task_id` (or equivalent ClaimEntry field)
- `solver` = `q.economic_state_t.claims_t[claim_id].solver` (or claimant)
- `reward` = `SettlementEngine::finalize(claim, escrow, attribution, ...)` — recomputed from Q_t

If wire-stored values diverge from Q-derived values at replay, **replay rejects with `TransitionError::ClaimNotFound` or a stricter mismatch error** (CO1.7-impl A4 enforces this; CO1.7.5 transition body owns the comparison rule).

**Royalty edges**: NOT on wire. Replay walks `q.economic_state_t.royalty_graph_t.edges_from(claim.target_work_tx)` per STATE § 3.4 stage 3c. Eliminates wire-format bloat + prevents stale royalty snapshots from being trusted post-amendment.

### § 4.2 Dual-sign rationale (v1.1 NEW per Gemini Q6)

`FinalizeRewardTx.system_signature` is **NOT redundant** with the L4 envelope signature. They sign different bytes:
- `FinalizeRewardTx.system_signature` signs the **payload bytes** (`FinalizeRewardSigningPayload.canonical_digest()` via `b"turingosv4.system_sig.finalize_reward.v1"` domain prefix). Audit-relevant for: "this finalize event was emitted by a runtime keypair epoch X" (cross-cell trust + post-hoc forensics).
- L4 `LedgerEntry.system_signature` signs the **sequencer-stamped envelope** (`LedgerEntrySigningPayload.canonical_digest()` via `b"turingosv4.ledger_entry_signing.v1"` — CO1.7 spec § 1.2). Audit-relevant for: "this `(logical_t, parent_ledger_root, tx_payload_cid)` was committed by the sequencer".

A successful replay verifies BOTH: payload sig (this struct) confirms typed bytes integrity; envelope sig confirms sequencer commitment ordering.

---

## § 5 Other typed tx schemas (transcribed from STATE spec)

`WorkTx` (§ 1.2 — 12 fields), `VerifyTx` / `ChallengeTx` / `ReuseTx` (§ 1.3), `TaskExpireTx` (§ 3.6 v1.3 schema). Verbatim transcription; minor adjustments documented inline.

`TxStatus` includes a `Pending` variant (per STATE § 1.2) but in this v4 codebase `TxStatus` is **set BY the runner**, never serialized into the canonical transaction wire format. Therefore: `TxStatus` is **NOT a field of any TypedTx variant**; it is a runtime book-keeping enum exposed on the public API surface but not part of the canonical encoding. (CO1.7 spec § 1.2 puts `status: TxStatus` on WorkTx field 12; this atom **diverges**: status is tracked in `q_t.q_t.agents[id].last_accepted_tx` + ClaimsIndex, NOT on the wire. **Audit input**: confirm or push back.)

---

## § 6 TypedTx enum

```rust
pub enum TypedTx {
    Work(WorkTx),
    Verify(VerifyTx),
    Challenge(ChallengeTx),
    Reuse(ReuseTx),
    FinalizeReward(FinalizeRewardTx),
    TaskExpire(TaskExpireTx),
    TerminalSummary(TerminalSummaryTx),  // imported from system_keypair
}

impl TypedTx {
    pub fn tx_kind(&self) -> TxKind {
        match self {
            Self::Work(_)            => TxKind::Work,
            Self::Verify(_)          => TxKind::Verify,
            Self::Challenge(_)       => TxKind::Challenge,
            Self::Reuse(_)           => TxKind::Reuse,
            Self::FinalizeReward(_)  => TxKind::FinalizeReward,
            Self::TaskExpire(_)      => TxKind::TaskExpire,
            Self::TerminalSummary(_) => TxKind::TerminalSummary,
        }
    }
}
```

The `TxKind` enum already exists in `transition_ledger.rs` with `#[repr(u8)]` and explicit discriminants. `TypedTx::tx_kind()` is the projection used by CO1.7 sequencer apply_one stage 5 (`tx_kind: TxKind::from_typed(&tx)` → renamed `TypedTx::tx_kind(&tx)` for ergonomics).

---

## § 7 Canonical serialization invariants

`canonical_encode` / `canonical_decode` (already shipped in `transition_ledger.rs` per CO1.7-impl A1) are reused as the wire codec:

- **I-CANON-A**: `canonical_encode(typed_tx)` returns deterministic bytes (BE + fixed_int + BTreeMap/BTreeSet lex order).
- **I-CANON-B**: `decode(encode(x)) == x` byte-identically for ALL variants (incl. zero-default).
- **I-CANON-C**: 2 independent encode calls on the same value produce identical bytes.
- **I-CANON-D**: per-variant golden fixture: every TypedTx variant (7 / 7) has a known SHA-256 of canonical bytes, hard-coded in tests (`EXPECTED_HEX_*`). Future serde-derive / codec change → fixture diff → audit-required (rotation commit).
- **I-CANON-E** (v1.1 NEW): cross-variant non-collision — pairwise digests over all 7 fixture variants are distinct.
- **I-CANON-F** (v1.1 NEW): BTreeMap / BTreeSet permutation independence — building the same struct via different insertion orders produces byte-identical bytes.
- **I-CANON-G** (v1.1 NEW per C-1): each agent-signed and system-emitted typed-tx has a paired `*SigningPayload` struct + `canonical_digest()` with explicit domain prefix `b"turingosv4.<actor>.<purpose>.v1"`. Domain prefix bytes are part of the SHA-256 input. 6 distinct domains (work / verify / challenge agent + finalize_reward / task_expire / terminal_summary system) yield pairwise-distinct digests.

### § 7.1 Codec wording fix (v1.1 P6 per Codex Q-D round-1)

STATE_TRANSITION_SPEC § 2.5 v1.4 wording is **inaccurate** for the actual codec; this v1.1 spec corrects:

| What § 2.5 said | What bincode-2 actually does |
|---|---|
| `Enum discriminant: u8 (variant index in declaration order)` | **u32 BE** ([bincode 2.0.1 src/features/serde/ser.rs:186](https://docs.rs/bincode/2.0.1/src/bincode/features/serde/ser.rs.html), [src/enc/impls.rs:68](https://docs.rs/bincode/2.0.1/src/bincode/enc/impls.rs.html)) under `with_fixed_int_encoding`. The variant index is encoded as `u32::to_be_bytes()`. |
| `Strings serialized as UTF-8 with explicit length prefix u32-BE` | **u64 BE** length prefix (bincode encodes `usize` as u64 under `with_fixed_int_encoding`; [src/enc/impls.rs:128](https://docs.rs/bincode/2.0.1/src/bincode/enc/impls.rs.html)). The same applies to BTreeMap / BTreeSet / Vec lengths. |
| `#[repr(u8)]` controls discriminant | **No** — `#[repr(u8)]` is a Rust language attribute affecting in-memory layout + raw cast (`as u8`) but does NOT control serde wire format. Codex caught this; spec language fixed. |

**v1.1 decision**: keep u32 variants + u64 lengths; do NOT introduce a custom serde adapter to force u8 discriminants (which would force re-encoding of all existing fixtures + complicate forward-compat for >256 variants). The locked golden fixtures in `EXPECTED_HEX_*` reflect the actual u32/u64 codec.

This wording fix is a **spec-only patch**; no code change required (the codec was already correct; only the description was wrong).

---

## § 8 HasSubmitter trait

```rust
pub trait HasSubmitter {
    fn submitter_id(&self) -> Option<AgentId>;
}

impl HasSubmitter for WorkTx       { fn submitter_id(&self) -> Option<AgentId> { Some(self.agent_id.clone()) } }
impl HasSubmitter for VerifyTx     { fn submitter_id(&self) -> Option<AgentId> { Some(self.verifier_agent.clone()) } }
impl HasSubmitter for ChallengeTx  { fn submitter_id(&self) -> Option<AgentId> { Some(self.challenger_agent.clone()) } }
impl HasSubmitter for ReuseTx      { fn submitter_id(&self) -> Option<AgentId> { None } }
// FinalizeRewardTx, TaskExpireTx, TerminalSummaryTx: system-emitted; submitter_id() = None
```

Implements STATE spec § 3.6.5 v1.3 directive verbatim.

---

## § 9 Acknowledged divergences from STATE_TRANSITION_SPEC

| ID | STATE spec | CO1.1.4-pre1 v1.1 | Reason |
|---|---|---|---|
| **D-1** | § 1.2 WorkTx field 12 = `status: TxStatus` | **dropped from wire** (Codex round-1 PASS with patch note) | TxStatus is runner book-keeping, not canonical wire data. STATE § 3 transition fns do NOT read `tx.status` from received tx; status is derived from accepted-tx history + ClaimsIndex. Codex Q-A round-1: PASS. |
| **D-2** | § 3.4 `FinalizeTx::from(claim_id, reward)` opaque constructor | **explicit `FinalizeRewardTx` struct** with Q-derived field discipline (§ 4.1) + dual-sign rationale (§ 4.2) | spec gap; derived schema. |
| **D-3** | ~~§ 1.5 `TerminalSummaryTx` 3-field placeholder~~ | **RESOLVED v1.1 P3**: migrated to full 8-field STATE § 1.5 schema in `state::typed_tx`; system_keypair signs opaque `TerminalSummarySigning([u8;32])` digest. |
| **D-4** (v1.1 NEW per Codex Q-J / CX-3) | QState `task_markets_t` / `escrows_t` / `stakes_t` keyed by `TxId` (q_state.rs:201/161/182) but typed_tx schemas use `TaskId` for the same task references | **NOT retrofit in this atom**. Migration owned by **CO P2.1 (TaskMarket atom)** which will rekey the QState indices to `TaskId`. CO1.1.4-pre1 documents the cross-atom debt; no wire-format consequence (the typed-tx schemas already use `TaskId` correctly per STATE § 1.2). |

---

## § 10 Audit gates

| Round | Codex | Gemini | Conservative | Action |
|---|---|---|---|---|
| 1 | CHALLENGE (high) | CHALLENGE (high) | **CHALLENGE** | v1.1 patch round (P1-P10 above) — this version |
| 2 | ⏳ pending | ⏳ pending | TBD | re-audit on v1.1; expected PASS or 1-issue CHALLENGE |
| 3+ | … | … | … | iterate to PASS/PASS |

**Pre-implementation gate** (for CO1.7-impl A2-A4): CO1.1.4-pre1 must reach `PASS/PASS` before A2 starts.

**Audit cost estimate**: ~$15-25 (smaller surface than CO1.7 spec @ $25-42; mostly type definitions + 2 plausibly-derived schemas).

---

## § 11 Estimated scope

- **Spec rounds**: 1-2 expected. The bulk is mechanical transcription; § 4 (FinalizeRewardTx derivation) + § 5 D-1 (TxStatus elision) are the only design decisions auditors are likely to test.
- **Implementation**: ~600-900 LoC (types) + ~150-250 LoC (golden fixture + round-trip tests). All in `src/state/typed_tx.rs` + minimal `src/economy/money.rs` extension.
- **Wall-clock**: 1-2 days.
- **Total atom budget**: ~1.5-2.5 days from spec draft to PASS/PASS.

---

## § 12 What this spec does NOT specify

1. **Field-level meaning beyond identifier types**: e.g. what `read_set` MUST contain for replay attribution to work — that's a CO1.7.5 + CO P2.4.0 concern.
2. **Encryption**: no field is encrypted. Predicate visibility is a Q_t projection (Inv 10), not a schema concern.
3. **Versioning**: `extensions: BTreeMap<String, Vec<u8>>` is on `LedgerEntry` (CO1.7); per-tx forward compat is via additive variants on `TypedTx` (e.g. `TypedTx::MetaTx(...)` lands in v4.1). No per-struct `version` field.
4. **CAS persistence of payloads**: `tx_payload_cid: Cid` is the CAS handle; the bytes lookup is L3 CAS (CO1.4). CAS index persistence is **CO1.4-extra** (separate atom).

---

— ArchitectAI synthesis, 2026-04-28; awaiting round-1 dual external audit.


---

# Implementation v1.1: src/state/typed_tx.rs (target of audit)

```rust
//! Typed transaction ABI surface — CO1.1.4-pre1.
//!
//! Spec authority:
//! - `handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md` — this atom
//! - `STATE_TRANSITION_SPEC_v1_2026-04-27.md` § 1 (typed schemas), § 2.5
//!   (canonical serialization), § 3 (transition pseudocode used to derive
//!   FinalizeRewardTx schema in spec § 4)
//!
//! Why this module exists: when CO1.7-impl A1 (Git2LedgerWriter) shipped, the
//! downstream A2 (Sequencer + `dispatch_transition`) needed a `TypedTx` enum
//! whose variants carry per-kind tx structs. Those structs and ~20 supporting
//! types (identifiers, signatures, predicate-result types, status enums) were
//! "frozen on paper" in STATE_TRANSITION_SPEC § 1 but had no Rust definition.
//! CO1.1.4-pre1 lands them in isolation under its own dual-audit gate,
//! per the project's per-atom audit principle (CLAUDE.md "Audit Standard").
//!
//! /// TRACE_MATRIX FC2-Submit + § 1 typed schemas: typed-tx ABI surface.

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

use sha2::{Digest, Sha256};

use crate::bottom_white::cas::schema::Cid;
use crate::bottom_white::ledger::system_keypair::{serde_bytes_64, SystemEpoch, SystemSignature};
use crate::economy::money::{MicroCoin, StakeMicroCoin};
use crate::state::q_state::{AgentId, Hash, TxId};

// ────────────────────────────────────────────────────────────────────────────
// § 2 Identifier newtypes (all opaque strings to Q_t)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX § 1.2 — task-market entry id; opaque string.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct TaskId(pub String);

/// TRACE_MATRIX § 1.5 — runtime run id (one run per `Sequencer` driver lifecycle).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct RunId(pub String);

/// TRACE_MATRIX STATE § 3.4 + § 4 I-FINALIZE-BATCH-ORDER — typed claim id used
/// in `FinalizeRewardTx.claim_id` and `ClaimsIndex` keying. Wraps `TxId`
/// (the underlying claim is recorded against the work_tx's TxId in
/// ClaimsIndex per current QState shape) but **prevents accidental mixing
/// of claim references with arbitrary transaction references** at the type
/// level (Codex round-1 Q-B CHALLENGE).
///
/// `#[serde(transparent)]` — wire-identical to TxId, so adoption is
/// non-breaking for canonical encoding.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
#[serde(transparent)]
pub struct ClaimId(pub TxId);

impl ClaimId {
    pub fn new(s: impl Into<String>) -> Self {
        Self(TxId(s.into()))
    }
    pub fn as_tx_id(&self) -> &TxId {
        &self.0
    }
}

/// TRACE_MATRIX § 1.3 ReuseTx + L2 Tool Registry — opaque tool identifier.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct ToolId(pub String);

/// TRACE_MATRIX § 1.2 PredicateResultsBundle + L1 Predicate Registry — opaque predicate id.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct PredicateId(pub String);

/// TRACE_MATRIX § 1.2 WorkTx field 5 — read-set key (DAG attribution / replay).
/// Kept as opaque string in v1; stricter typing (path / tape-coordinate) lands
/// in CO P2.4.0 attribution-engine spike.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct ReadKey(pub String);

/// TRACE_MATRIX § 1.2 WorkTx field 6 — write-set key (DAG attribution / replay).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct WriteKey(pub String);

// ────────────────────────────────────────────────────────────────────────────
// § 3 AgentSignature (Ed25519 [u8;64], type-distinct from SystemSignature)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX § 1.2 WorkTx field 10 + I-SIG: agent-side detached Ed25519
/// signature over the per-tx canonical_digest. Distinct type from
/// `SystemSignature` to prevent accidental confusion at API boundaries
/// (Codex sec-arg: agent-vs-system signature mixing is a real hazard).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentSignature(#[serde(with = "serde_bytes_64")] [u8; 64]);

impl AgentSignature {
    pub const fn from_bytes(bytes: [u8; 64]) -> Self {
        Self(bytes)
    }
    pub const fn as_bytes(&self) -> &[u8; 64] {
        &self.0
    }
}

impl Default for AgentSignature {
    fn default() -> Self {
        Self([0u8; 64])
    }
}

// ────────────────────────────────────────────────────────────────────────────
// § 3 SlashEvidenceCid (newtype; type-distinct slash-evidence reference)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX § 1.2 TxStatus::FinalizedSlash — typed reference to the
/// counter-example payload that justified the slash (lives in L3 CAS).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
#[serde(transparent)]
pub struct SlashEvidenceCid(pub Cid);

// ────────────────────────────────────────────────────────────────────────────
// § 4 Predicate result types
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX § 1.2 PredicateResultsBundle — boolean predicate verdict
/// optionally accompanied by an L3 CAS reference to the proof object
/// (e.g. Lean witness, ZK proof bytes).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct BoolWithProof {
    pub value: bool,
    pub proof_cid: Option<Cid>,
}

/// TRACE_MATRIX § 1.2 PredicateResultsBundle — safety-class discriminator.
/// Determines fail-closed (Safety) vs fail-open-with-signal (Creation) behavior
/// when a predicate's evaluation errors. Frozen STATE spec § 1.2.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum SafetyOrCreation {
    Safety = 0,
    Creation = 1,
}

impl Default for SafetyOrCreation {
    fn default() -> Self {
        // Safety bias by default: fail-closed if no class declared.
        Self::Safety
    }
}

/// TRACE_MATRIX § 1.2 WorkTx field 8 — runner-stamped predicate results
/// (acceptance + settlement gates) with explicit safety-class discriminator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PredicateResultsBundle {
    pub acceptance: BTreeMap<PredicateId, BoolWithProof>,
    pub settlement: BTreeMap<PredicateId, BoolWithProof>,
    pub safety_class: SafetyOrCreation,
}

// ────────────────────────────────────────────────────────────────────────────
// § 5 Status / class enums (RejectionClass, VerifyVerdict, RunOutcome,
//                          and the runtime-only TxStatus per D-1)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX § 1.4 — classification of a rejected attempt.
/// Public predicates are classified concretely; private predicates surface as
/// `Opaque` (no information leakage to attacker).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum RejectionClass {
    AcceptancePredicateFail(PredicateId),
    SettlementPredicateFail(PredicateId),
    StakeInsufficient,
    SignatureInvalid,
    StaleParentRoot,
    Opaque,
    BudgetExceeded,
}

/// TRACE_MATRIX § 1.3 VerifyTx field 5 — verifier verdict.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum VerifyVerdict {
    Confirm = 0,
    Doubt = 1,
}

/// TRACE_MATRIX § 1.5 TerminalSummaryTx field 4 + Art. IV halt-reason taxonomy.
/// Five-way partition over how a run terminates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum RunOutcome {
    OmegaAccepted = 0,
    MaxTxExhausted = 1,
    WallClockCap = 2,
    ComputeCap = 3,
    ErrorHalt = 4,
}

/// TRACE_MATRIX § 1.2 TxStatus — **runtime book-keeping only** (D-1 divergence
/// from STATE spec): never serialized into a TypedTx variant's wire bytes.
/// Tracked in `q_t.q_t.agents[id].last_accepted_tx` + `ClaimsIndex`. Exposed
/// here as a public type for the runtime API surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TxStatus {
    Pending,
    Accepted,
    Rejected(RejectionClass),
    FinalizedReward(MicroCoin),
    FinalizedSlash(SlashEvidenceCid),
}

// ────────────────────────────────────────────────────────────────────────────
// § 5 (cont'd) — Typed tx structs (STATE spec § 1.2-1.6 + § 3.6)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX § 1.2 — agent-submitted work transaction (12-field schema;
/// **D-1 divergence**: field 12 `status: TxStatus` is excluded from canonical
/// wire bytes — TxStatus is runner book-keeping per CO1.1.4-pre1 spec § 5).
///
/// This is the per-tx struct that the CO1.7 sequencer hands to
/// `step_transition` (CO1.7.5 body atom). The `signature` is over
/// `canonical_digest(&work_tx)` where the digest pre-image excludes the
/// signature itself (its own input).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct WorkTx {
    pub tx_id: TxId,                                  //  1
    pub task_id: TaskId,                              //  2
    pub parent_state_root: Hash,                      //  3
    pub agent_id: AgentId,                            //  4
    pub read_set: BTreeSet<ReadKey>,                  //  5
    pub write_set: BTreeSet<WriteKey>,                //  6
    pub proposal_cid: Cid,                            //  7
    pub predicate_results: PredicateResultsBundle,    //  8 (runner-stamped)
    pub stake: StakeMicroCoin,                        //  9
    pub signature: AgentSignature,                    // 10
    pub timestamp_logical: u64,                       // 11
    // 12: TxStatus — D-1 elision; runtime-only.
}

/// TRACE_MATRIX § 1.3 — verifier verdict transaction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct VerifyTx {
    pub tx_id: TxId,                       //  1
    pub target_work_tx: TxId,              //  2
    pub verifier_agent: AgentId,           //  3
    pub bond: StakeMicroCoin,              //  4
    pub verdict: VerifyVerdict,            //  5
    pub signature: AgentSignature,         //  6
    pub timestamp_logical: u64,            //  7
}

impl Default for VerifyVerdict {
    fn default() -> Self {
        Self::Confirm
    }
}

/// TRACE_MATRIX § 1.3 — challenge transaction (counter-example posted with
/// stake at risk).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ChallengeTx {
    pub tx_id: TxId,                       //  1
    pub target_work_tx: TxId,              //  2
    pub challenger_agent: AgentId,         //  3
    pub stake: StakeMicroCoin,             //  4
    pub counterexample_cid: Cid,           //  5
    pub signature: AgentSignature,         //  6
    pub timestamp_logical: u64,            //  7
}

/// TRACE_MATRIX § 1.3 — fact-tx recording reuse of a tool created by a prior
/// agent (royalty graph edge). No submitting agent (per § 3.6.5 v1.3).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ReuseTx {
    pub tx_id: TxId,                       //  1
    pub reusing_work_tx: TxId,             //  2
    pub reused_tool_id: ToolId,            //  3
    pub reused_tool_creator: AgentId,      //  4
    pub timestamp_logical: u64,            //  5
}

/// TRACE_MATRIX CO1.1.4-pre1 spec § 4 — derived schema (STATE spec § 3.4
/// uses opaque `FinalizeTx::from(claim_id, reward)` constructor without an
/// explicit struct definition).
///
/// **v1.1 round-1 audit closures**:
/// - **C-3 (Codex Q-B)**: `claim_id` is now a typed `ClaimId` newtype (was
///   bare `TxId`) — STATE § 4 I-FINALIZE-BATCH-ORDER speaks in claim_id;
///   reusing TxId leaked QState implementation into the wire format.
/// - **C-3 (Codex Q-B)**: `task_id` / `solver` / `reward` are documented as
///   **Q-DERIVED at replay** — replay (CO1.7-impl A4) re-fetches them from
///   ClaimsIndex by `claim_id`, NOT trusted from wire. Wire fields are kept
///   as a ledger summary (so a human reading L4 can see the finalize event
///   semantics) but the AUTHORITATIVE values come from Q_t.
/// - **C-3 / GM-2 followup**: `system_signature` is RETAINED for v1.1 — it
///   binds the system-emitted FinalizeRewardTx to a specific runtime keypair
///   epoch (auditability + cross-cell trust). The CO1.7 `LedgerEntry`
///   wraps this struct via CAS reference + signs the `LedgerEntrySigningPayload`
///   digest; the two sigs are NOT redundant: this one binds the tx-payload
///   bytes; the L4 envelope sig binds the (logical_t, parent_ledger_root, tx_payload_cid)
///   sequencer-stamped envelope. v1.1 spec § 4 makes the dual-sign rationale explicit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct FinalizeRewardTx {
    pub tx_id: TxId,                       //  1
    pub claim_id: ClaimId,                 //  2 — typed (was TxId in v1)
    pub task_id: TaskId,                   //  3 — Q-derived authoritative; wire = ledger summary
    pub solver: AgentId,                   //  4 — Q-derived authoritative; wire = ledger summary
    pub reward: MicroCoin,                 //  5 — Q-derived authoritative (SettlementEngine output); wire = ledger summary
    pub parent_state_root: Hash,           //  6
    pub epoch: SystemEpoch,                //  7
    pub timestamp_logical: u64,            //  8
    pub system_signature: SystemSignature, //  9 — see doc-comment on dual-sign rationale
}

/// TRACE_MATRIX STATE spec § 3.6 v1.3 — system-emitted task-expiry tx
/// (refunds bounty + locked stakes when no claim finalized by deadline).
/// Verbatim transcription.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TaskExpireTx {
    pub tx_id: TxId,                       //  1
    pub task_id: TaskId,                   //  2
    pub parent_state_root: Hash,           //  3
    pub bounty_refunded: MicroCoin,        //  4 (computed by runtime; included for ledger summary)
    pub epoch: SystemEpoch,                //  5
    pub timestamp_logical: u64,            //  6
    pub system_signature: SystemSignature, //  7
}

/// TRACE_MATRIX STATE spec § 1.5 — system-emitted no-accept-run handler.
/// Emitted exactly once if a run terminates without any accepted work_tx, so
/// L6 reconstructibility (failure-class signal) is preserved on the tape
/// even when no work_tx ever passed.
///
/// **v1.1 round-1 audit closure (C-3 Codex Q-C must-fix-now)**: replaces the
/// 3-field placeholder previously living in `system_keypair.rs`. Full
/// 8-field schema per STATE § 1.5. The signer (`system_keypair`) now signs
/// an opaque `TerminalSummarySigning([u8; 32])` digest — same opaque-digest
/// pattern as `LedgerEntrySigning` — so the canonical_digest is computed
/// here and `system_keypair` stays oblivious to the typed-tx schema (no
/// circular `bottom_white ↔ state` dependency).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TerminalSummaryTx {
    pub tx_id: TxId,                                          //  1
    pub task_id: TaskId,                                      //  2
    pub run_id: RunId,                                        //  3
    pub run_outcome: RunOutcome,                              //  4
    pub total_attempts: u32,                                  //  5
    pub failure_class_histogram: BTreeMap<RejectionClass, u32>,// 6
    pub last_logical_t: u64,                                  //  7
    pub system_signature: SystemSignature,                    //  8
}

impl Default for RunOutcome {
    fn default() -> Self {
        Self::OmegaAccepted
    }
}

// ────────────────────────────────────────────────────────────────────────────
// § 7 Signing payloads (CO1.1.4-pre1 v1.1 round-1 closure C-1)
//
// Each agent-signed and system-emitted typed-tx has a paired `*SigningPayload`
// struct (subset of fields, EXCLUDES the signature itself) with a
// `canonical_digest()` method that **prepends a stable domain-separation
// prefix** before the bincode-canonical body bytes. This implements:
//
//   sig_input = sha256(b"turingosv4.<actor>.<purpose>.v1" || canonical_encode(payload))
//
// Property: even if two distinct payload TYPES happen to bincode-encode to
// identical bytes (extremely unlikely given distinct field shapes, but
// defensively guaranteed), the domain prefix ensures the SHA-256 inputs
// differ. Closes Codex Q-E + Gemini Q7: type-level distinction is necessary
// but not sufficient as a security boundary.
//
// **Forward dependency**: actual `verify_agent_signature(sig, payload, agent_pubkey)`
// + agent-pubkey-registry lookup is CO P2.x AgentRegistry territory; this
// atom only freezes the canonical_digest pre-image.
// ────────────────────────────────────────────────────────────────────────────

const DOMAIN_AGENT_WORK: &[u8] = b"turingosv4.agent_sig.work.v1";
const DOMAIN_AGENT_VERIFY: &[u8] = b"turingosv4.agent_sig.verify.v1";
const DOMAIN_AGENT_CHALLENGE: &[u8] = b"turingosv4.agent_sig.challenge.v1";
const DOMAIN_SYSTEM_FINALIZE_REWARD: &[u8] = b"turingosv4.system_sig.finalize_reward.v1";
const DOMAIN_SYSTEM_TASK_EXPIRE: &[u8] = b"turingosv4.system_sig.task_expire.v1";
const DOMAIN_SYSTEM_TERMINAL_SUMMARY: &[u8] = b"turingosv4.system_sig.terminal_summary.v1";

fn domain_prefixed_digest<T: Serialize>(domain: &[u8], value: &T) -> [u8; 32] {
    use crate::bottom_white::ledger::transition_ledger::canonical_encode;
    let body = canonical_encode(value).expect("canonical_encode of signing payload");
    let mut h = Sha256::new();
    h.update(domain);
    h.update(&body);
    h.finalize().into()
}

/// Agent signing payload for `WorkTx` (12 fields → 11 fields; signature excluded).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct WorkSigningPayload {
    pub tx_id: TxId,
    pub task_id: TaskId,
    pub parent_state_root: Hash,
    pub agent_id: AgentId,
    pub read_set: BTreeSet<ReadKey>,
    pub write_set: BTreeSet<WriteKey>,
    pub proposal_cid: Cid,
    pub predicate_results: PredicateResultsBundle,
    pub stake: StakeMicroCoin,
    pub timestamp_logical: u64,
}

impl WorkSigningPayload {
    pub fn canonical_digest(&self) -> [u8; 32] {
        domain_prefixed_digest(DOMAIN_AGENT_WORK, self)
    }
}

/// Agent signing payload for `VerifyTx` (7 fields → 6 fields).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct VerifySigningPayload {
    pub tx_id: TxId,
    pub target_work_tx: TxId,
    pub verifier_agent: AgentId,
    pub bond: StakeMicroCoin,
    pub verdict: VerifyVerdict,
    pub timestamp_logical: u64,
}

impl VerifySigningPayload {
    pub fn canonical_digest(&self) -> [u8; 32] {
        domain_prefixed_digest(DOMAIN_AGENT_VERIFY, self)
    }
}

/// Agent signing payload for `ChallengeTx` (7 fields → 6 fields).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ChallengeSigningPayload {
    pub tx_id: TxId,
    pub target_work_tx: TxId,
    pub challenger_agent: AgentId,
    pub stake: StakeMicroCoin,
    pub counterexample_cid: Cid,
    pub timestamp_logical: u64,
}

impl ChallengeSigningPayload {
    pub fn canonical_digest(&self) -> [u8; 32] {
        domain_prefixed_digest(DOMAIN_AGENT_CHALLENGE, self)
    }
}

/// System signing payload for `FinalizeRewardTx` (9 fields → 8 fields).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct FinalizeRewardSigningPayload {
    pub tx_id: TxId,
    pub claim_id: ClaimId,
    pub task_id: TaskId,
    pub solver: AgentId,
    pub reward: MicroCoin,
    pub parent_state_root: Hash,
    pub epoch: SystemEpoch,
    pub timestamp_logical: u64,
}

impl FinalizeRewardSigningPayload {
    pub fn canonical_digest(&self) -> [u8; 32] {
        domain_prefixed_digest(DOMAIN_SYSTEM_FINALIZE_REWARD, self)
    }
}

/// System signing payload for `TaskExpireTx` (7 fields → 6 fields).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TaskExpireSigningPayload {
    pub tx_id: TxId,
    pub task_id: TaskId,
    pub parent_state_root: Hash,
    pub bounty_refunded: MicroCoin,
    pub epoch: SystemEpoch,
    pub timestamp_logical: u64,
}

impl TaskExpireSigningPayload {
    pub fn canonical_digest(&self) -> [u8; 32] {
        domain_prefixed_digest(DOMAIN_SYSTEM_TASK_EXPIRE, self)
    }
}

/// System signing payload for `TerminalSummaryTx` (8 fields → 7 fields).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TerminalSummarySigningPayload {
    pub tx_id: TxId,
    pub task_id: TaskId,
    pub run_id: RunId,
    pub run_outcome: RunOutcome,
    pub total_attempts: u32,
    pub failure_class_histogram: BTreeMap<RejectionClass, u32>,
    pub last_logical_t: u64,
}

impl TerminalSummarySigningPayload {
    pub fn canonical_digest(&self) -> [u8; 32] {
        domain_prefixed_digest(DOMAIN_SYSTEM_TERMINAL_SUMMARY, self)
    }
}

// ── Projections: tx → signing payload ────────────────────────────────────

impl WorkTx {
    pub fn to_signing_payload(&self) -> WorkSigningPayload {
        WorkSigningPayload {
            tx_id: self.tx_id.clone(),
            task_id: self.task_id.clone(),
            parent_state_root: self.parent_state_root,
            agent_id: self.agent_id.clone(),
            read_set: self.read_set.clone(),
            write_set: self.write_set.clone(),
            proposal_cid: self.proposal_cid,
            predicate_results: self.predicate_results.clone(),
            stake: self.stake,
            timestamp_logical: self.timestamp_logical,
        }
    }
}

impl VerifyTx {
    pub fn to_signing_payload(&self) -> VerifySigningPayload {
        VerifySigningPayload {
            tx_id: self.tx_id.clone(),
            target_work_tx: self.target_work_tx.clone(),
            verifier_agent: self.verifier_agent.clone(),
            bond: self.bond,
            verdict: self.verdict,
            timestamp_logical: self.timestamp_logical,
        }
    }
}

impl ChallengeTx {
    pub fn to_signing_payload(&self) -> ChallengeSigningPayload {
        ChallengeSigningPayload {
            tx_id: self.tx_id.clone(),
            target_work_tx: self.target_work_tx.clone(),
            challenger_agent: self.challenger_agent.clone(),
            stake: self.stake,
            counterexample_cid: self.counterexample_cid,
            timestamp_logical: self.timestamp_logical,
        }
    }
}

impl FinalizeRewardTx {
    pub fn to_signing_payload(&self) -> FinalizeRewardSigningPayload {
        FinalizeRewardSigningPayload {
            tx_id: self.tx_id.clone(),
            claim_id: self.claim_id.clone(),
            task_id: self.task_id.clone(),
            solver: self.solver.clone(),
            reward: self.reward,
            parent_state_root: self.parent_state_root,
            epoch: self.epoch,
            timestamp_logical: self.timestamp_logical,
        }
    }
}

impl TaskExpireTx {
    pub fn to_signing_payload(&self) -> TaskExpireSigningPayload {
        TaskExpireSigningPayload {
            tx_id: self.tx_id.clone(),
            task_id: self.task_id.clone(),
            parent_state_root: self.parent_state_root,
            bounty_refunded: self.bounty_refunded,
            epoch: self.epoch,
            timestamp_logical: self.timestamp_logical,
        }
    }
}

impl TerminalSummaryTx {
    pub fn to_signing_payload(&self) -> TerminalSummarySigningPayload {
        TerminalSummarySigningPayload {
            tx_id: self.tx_id.clone(),
            task_id: self.task_id.clone(),
            run_id: self.run_id.clone(),
            run_outcome: self.run_outcome,
            total_attempts: self.total_attempts,
            failure_class_histogram: self.failure_class_histogram.clone(),
            last_logical_t: self.last_logical_t,
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────
// § 6 TypedTx outer enum
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX § 8 dispatch_transition — typed-tx outer enum.
/// 7 variants (K5 closed: NO `Slash`).
/// `TerminalSummaryTx` is imported from `system_keypair.rs` (already shipped).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TypedTx {
    Work(WorkTx),
    Verify(VerifyTx),
    Challenge(ChallengeTx),
    Reuse(ReuseTx),
    FinalizeReward(FinalizeRewardTx),
    TaskExpire(TaskExpireTx),
    TerminalSummary(TerminalSummaryTx),
}

impl TypedTx {
    /// Project to the [`TxKind`] discriminator stored in `LedgerEntry.tx_kind`.
    pub fn tx_kind(&self) -> crate::bottom_white::ledger::transition_ledger::TxKind {
        use crate::bottom_white::ledger::transition_ledger::TxKind;
        match self {
            Self::Work(_) => TxKind::Work,
            Self::Verify(_) => TxKind::Verify,
            Self::Challenge(_) => TxKind::Challenge,
            Self::Reuse(_) => TxKind::Reuse,
            Self::FinalizeReward(_) => TxKind::FinalizeReward,
            Self::TaskExpire(_) => TxKind::TaskExpire,
            Self::TerminalSummary(_) => TxKind::TerminalSummary,
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────
// § 8 HasSubmitter trait (STATE spec § 3.6.5 v1.3)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX STATE spec § 3.6.5 v1.3 — submitter resolution trait used
/// by the implicit-init step in agent-submitted transitions. System-emitted
/// transitions return `None` (no agent to init).
pub trait HasSubmitter {
    fn submitter_id(&self) -> Option<AgentId>;
}

impl HasSubmitter for WorkTx {
    fn submitter_id(&self) -> Option<AgentId> {
        Some(self.agent_id.clone())
    }
}

impl HasSubmitter for VerifyTx {
    fn submitter_id(&self) -> Option<AgentId> {
        Some(self.verifier_agent.clone())
    }
}

impl HasSubmitter for ChallengeTx {
    fn submitter_id(&self) -> Option<AgentId> {
        Some(self.challenger_agent.clone())
    }
}

impl HasSubmitter for ReuseTx {
    fn submitter_id(&self) -> Option<AgentId> {
        None
    }
}

impl HasSubmitter for FinalizeRewardTx {
    fn submitter_id(&self) -> Option<AgentId> {
        None
    }
}

impl HasSubmitter for TaskExpireTx {
    fn submitter_id(&self) -> Option<AgentId> {
        None
    }
}

impl HasSubmitter for TerminalSummaryTx {
    fn submitter_id(&self) -> Option<AgentId> {
        None
    }
}

impl HasSubmitter for TypedTx {
    fn submitter_id(&self) -> Option<AgentId> {
        match self {
            Self::Work(t) => t.submitter_id(),
            Self::Verify(t) => t.submitter_id(),
            Self::Challenge(t) => t.submitter_id(),
            Self::Reuse(t) => t.submitter_id(),
            Self::FinalizeReward(t) => t.submitter_id(),
            Self::TaskExpire(t) => t.submitter_id(),
            Self::TerminalSummary(t) => t.submitter_id(),
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────
// TransitionError — minimal v1 taxonomy (CO1.1.4-pre1 spec § 0 out-of-scope
// note: full per-stage enum proliferation is CO1.7.5)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX STATE § 3 — transition-function error taxonomy. v1.1 covers
/// every variant invoked in STATE_TRANSITION_SPEC § 3.1-3.7 pseudocode +
/// `NotYetImplemented` for CO1.7.5 stub bodies (per Codex Q-G CHALLENGE).
///
/// **Why payloads are minimal**: the failed `PredicateId` (etc.) is a string
/// reference; richer context (PredicateResultsBundle, Cid of failed proof)
/// is attached by the runtime via separate book-keeping channels (rejected
/// summary stamping, bus rejection log). Keeping TransitionError serializable
/// with primitive payloads avoids forcing PredicateResultsBundle through
/// every error site.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransitionError {
    // ── Stale-parent & signature ───────────────────────────────────────────
    /// `parent_state_root` does not match `q.state_root_t` (any agent tx).
    StaleParent,
    /// Agent signature verify failed (work / verify / challenge tx).
    SignatureInvalid,
    /// System-keypair signature verify failed (system-emitted tx).
    InvalidSystemSignature,

    // ── Economy ────────────────────────────────────────────────────────────
    /// Submitter's available balance is below the declared stake / bond.
    /// Payload-rich variant (available + required) is intentionally elided
    /// in v1.1 to keep this enum primitive-payloads-only; runtime attaches
    /// context via the rejection log (per STATE § 1.4 RejectedAttemptSummary).
    StakeInsufficient,

    // ── Target lookup ──────────────────────────────────────────────────────
    /// VerifyTx / ChallengeTx / ReuseTx target work_tx not found in L4.
    TargetWorkTxNotFound,
    /// VerifyTx target is not in a verifiable status (e.g. already finalized).
    TargetWorkTxNotVerifiable,
    /// ReuseTx target work_tx exists but is not yet Accepted (parent must accept first).
    ParentNotAcceptedYet,

    // ── Predicate failures ─────────────────────────────────────────────────
    /// step_transition stage 4 — acceptance predicate denied. `PredicateId`
    /// is the public predicate that failed; private predicates surface as
    /// `RejectionClass::Opaque` in book-keeping (NOT here).
    AcceptancePredicateFailed(PredicateId),
    /// verify_transition stage 4 — verification predicate denied.
    VerificationPredicateFailed(PredicateId),
    /// finalize_reward / step_transition stage 5 — settlement predicate denied.
    SettlementPredicateFailed(PredicateId),

    // ── Challenge ──────────────────────────────────────────────────────────
    /// challenge_transition stage 1 — challenge filed after window closed.
    ChallengeWindowClosed,
    /// finalize_reward stage 1 — challenge window still open; cannot finalize.
    ChallengeWindowStillOpen,
    /// finalize_reward stage 1 — claim already slashed; cannot also reward.
    AlreadySlashed,
    /// challenge_transition stage 4 — counterexample failed predicate check.
    CounterexampleInsufficient,

    // ── Reuse ──────────────────────────────────────────────────────────────
    /// reuse_transition stage 1 — referenced tool not in L2 ToolRegistry.
    ToolNotInRegistry,
    /// reuse_transition stage 1 — declared tool creator does not match registry.
    ToolCreatorMismatch,

    // ── Finalize ───────────────────────────────────────────────────────────
    /// finalize_reward — no claim entry for the given claim_id.
    ClaimNotFound,

    // ── Task expire ────────────────────────────────────────────────────────
    /// task_expire — referenced TaskMarket entry not found.
    TaskNotFound,
    /// task_expire — deadline not yet reached.
    TaskNotExpired,
    /// task_expire — at least one open claim exists; cannot refund bounty.
    TaskHasOpenClaim,

    // ── Terminal summary ───────────────────────────────────────────────────
    /// emit_terminal_summary — run already has an accepted work_tx.
    TerminalSummaryNotApplicable,

    // ── Stub sentinel (CO1.7.5 fills) ──────────────────────────────────────
    /// Stub return value used by CO1.7.5 unimplemented bodies — preserves
    /// sequencer + dispatch correctness without forcing transition logic
    /// into this atom. Audit input: this is intentional, not a code smell.
    NotYetImplemented,
}

impl std::fmt::Display for TransitionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StaleParent => write!(f, "stale parent_state_root"),
            Self::SignatureInvalid => write!(f, "agent signature invalid"),
            Self::InvalidSystemSignature => write!(f, "invalid system signature"),
            Self::StakeInsufficient => write!(f, "stake / bond insufficient"),
            Self::TargetWorkTxNotFound => write!(f, "target work_tx not found"),
            Self::TargetWorkTxNotVerifiable => write!(f, "target work_tx not in a verifiable state"),
            Self::ParentNotAcceptedYet => write!(f, "parent work_tx not yet accepted"),
            Self::AcceptancePredicateFailed(p) => write!(f, "acceptance predicate failed: {p:?}"),
            Self::VerificationPredicateFailed(p) => write!(f, "verification predicate failed: {p:?}"),
            Self::SettlementPredicateFailed(p) => write!(f, "settlement predicate failed: {p:?}"),
            Self::ChallengeWindowClosed => write!(f, "challenge window closed"),
            Self::ChallengeWindowStillOpen => write!(f, "challenge window still open"),
            Self::AlreadySlashed => write!(f, "already slashed"),
            Self::CounterexampleInsufficient => write!(f, "counterexample insufficient"),
            Self::ToolNotInRegistry => write!(f, "reuse tool not in registry"),
            Self::ToolCreatorMismatch => write!(f, "reuse tool creator mismatch"),
            Self::ClaimNotFound => write!(f, "claim not found"),
            Self::TaskNotFound => write!(f, "task not found"),
            Self::TaskNotExpired => write!(f, "task deadline not yet reached"),
            Self::TaskHasOpenClaim => write!(f, "task has at least one open claim"),
            Self::TerminalSummaryNotApplicable => write!(f, "terminal summary not applicable"),
            Self::NotYetImplemented => write!(f, "transition body not yet implemented (CO1.7.5)"),
        }
    }
}
impl std::error::Error for TransitionError {}

// ────────────────────────────────────────────────────────────────────────────
// SignalBundle — minimal v1 typed shape (CO1.7.5 + CO1.9 enrich it later)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX STATE § 3 — tape-emitted signal bundle. v1 minimal: a single
/// enum variant per spec call site in § 3 pseudocode (`empty` /
/// `finalize` / `task_expired` / `terminal_summary`). Full L6 signal-stream
/// design is CO1.9. CO1.1.4-pre1 ships just enough shape for CO1.7-impl to
/// compile and for CO1.7.5 transition bodies to construct each variant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SignalBundle {
    pub kind: SignalKind,
}

/// Discriminator over the spec § 3 pseudocode's `SignalBundle::*` constructors.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SignalKind {
    Empty,
    Finalize {
        claim_id: TxId,
        reward: MicroCoin,
    },
    TaskExpired {
        task_id: TaskId,
        bounty_refunded: MicroCoin,
    },
    TerminalSummary {
        run_id: RunId,
        outcome: RunOutcome,
    },
}

impl Default for SignalKind {
    fn default() -> Self {
        Self::Empty
    }
}

impl SignalBundle {
    pub fn empty() -> Self {
        Self {
            kind: SignalKind::Empty,
        }
    }
    pub fn finalize(claim_id: TxId, reward: MicroCoin) -> Self {
        Self {
            kind: SignalKind::Finalize { claim_id, reward },
        }
    }
    pub fn task_expired(task_id: TaskId, bounty_refunded: MicroCoin) -> Self {
        Self {
            kind: SignalKind::TaskExpired {
                task_id,
                bounty_refunded,
            },
        }
    }
    pub fn terminal_summary(run_id: RunId, outcome: RunOutcome) -> Self {
        Self {
            kind: SignalKind::TerminalSummary { run_id, outcome },
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Tests — round-trip (I-CANON-A/B/C) + golden fixtures (I-CANON-D)
// ────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bottom_white::ledger::transition_ledger::{canonical_decode, canonical_encode};
    use sha2::{Digest, Sha256};

    fn h(byte: u8) -> Hash {
        Hash([byte; 32])
    }
    fn cid(byte: u8) -> Cid {
        Cid([byte; 32])
    }

    /// Helper: canonical bytes → SHA-256 hex string. Used to lock golden
    /// fixtures: any future change to the wire format causes the digest hex
    /// to diverge → audit-required.
    fn digest_hex<T: Serialize>(value: &T) -> String {
        let bytes = canonical_encode(value).expect("encode");
        let hash = Sha256::digest(&bytes);
        hex_lower(&hash)
    }
    fn hex_lower(bytes: &[u8]) -> String {
        let mut s = String::with_capacity(bytes.len() * 2);
        for b in bytes {
            s.push_str(&format!("{:02x}", b));
        }
        s
    }

    // ── I-CANON-A/B/C — round-trip + byte-stability ──────────────────────────

    fn fixture_work_tx() -> WorkTx {
        let mut acceptance = BTreeMap::new();
        acceptance.insert(
            PredicateId("acc1".into()),
            BoolWithProof {
                value: true,
                proof_cid: Some(cid(0x11)),
            },
        );
        let mut settlement = BTreeMap::new();
        settlement.insert(
            PredicateId("set1".into()),
            BoolWithProof {
                value: true,
                proof_cid: None,
            },
        );
        WorkTx {
            tx_id: TxId("worktx-fixture-01".into()),
            task_id: TaskId("task-fixture-01".into()),
            parent_state_root: h(0x42),
            agent_id: AgentId("alice".into()),
            read_set: [ReadKey("k.read.a".into()), ReadKey("k.read.b".into())]
                .into_iter()
                .collect(),
            write_set: [WriteKey("k.write.a".into())].into_iter().collect(),
            proposal_cid: cid(0x13),
            predicate_results: PredicateResultsBundle {
                acceptance,
                settlement,
                safety_class: SafetyOrCreation::Safety,
            },
            stake: StakeMicroCoin::from_micro_units(1_000_000),
            signature: AgentSignature::from_bytes([0x77u8; 64]),
            timestamp_logical: 7,
        }
    }

    fn fixture_verify_tx() -> VerifyTx {
        VerifyTx {
            tx_id: TxId("verifytx-fixture-01".into()),
            target_work_tx: TxId("worktx-fixture-01".into()),
            verifier_agent: AgentId("bob".into()),
            bond: StakeMicroCoin::from_micro_units(500_000),
            verdict: VerifyVerdict::Confirm,
            signature: AgentSignature::from_bytes([0x55u8; 64]),
            timestamp_logical: 8,
        }
    }

    fn fixture_challenge_tx() -> ChallengeTx {
        ChallengeTx {
            tx_id: TxId("challengetx-fixture-01".into()),
            target_work_tx: TxId("worktx-fixture-01".into()),
            challenger_agent: AgentId("carol".into()),
            stake: StakeMicroCoin::from_micro_units(2_000_000),
            counterexample_cid: cid(0x21),
            signature: AgentSignature::from_bytes([0x33u8; 64]),
            timestamp_logical: 9,
        }
    }

    fn fixture_reuse_tx() -> ReuseTx {
        ReuseTx {
            tx_id: TxId("reusetx-fixture-01".into()),
            reusing_work_tx: TxId("worktx-fixture-02".into()),
            reused_tool_id: ToolId("tool-001".into()),
            reused_tool_creator: AgentId("alice".into()),
            timestamp_logical: 10,
        }
    }

    fn fixture_finalize_reward_tx() -> FinalizeRewardTx {
        FinalizeRewardTx {
            tx_id: TxId("finalizetx-fixture-01".into()),
            claim_id: ClaimId::new("claim-001"),
            task_id: TaskId("task-fixture-01".into()),
            solver: AgentId("alice".into()),
            reward: MicroCoin::from_micro_units(5_000_000),
            parent_state_root: h(0x43),
            epoch: SystemEpoch::new(1),
            timestamp_logical: 11,
            system_signature: SystemSignature::from_bytes([0xaau8; 64]),
        }
    }

    fn fixture_task_expire_tx() -> TaskExpireTx {
        TaskExpireTx {
            tx_id: TxId("expiretx-fixture-01".into()),
            task_id: TaskId("task-fixture-02".into()),
            parent_state_root: h(0x44),
            bounty_refunded: MicroCoin::from_micro_units(3_000_000),
            epoch: SystemEpoch::new(1),
            timestamp_logical: 12,
            system_signature: SystemSignature::from_bytes([0xbbu8; 64]),
        }
    }

    fn fixture_terminal_summary_tx() -> TerminalSummaryTx {
        let mut hist = BTreeMap::new();
        hist.insert(RejectionClass::SignatureInvalid, 2);
        hist.insert(RejectionClass::StakeInsufficient, 1);
        hist.insert(
            RejectionClass::AcceptancePredicateFail(PredicateId("acc1".into())),
            5,
        );
        TerminalSummaryTx {
            tx_id: TxId("terminalsummary-fixture-01".into()),
            task_id: TaskId("task-fixture-03".into()),
            run_id: RunId("run-001".into()),
            run_outcome: RunOutcome::MaxTxExhausted,
            total_attempts: 8,
            failure_class_histogram: hist,
            last_logical_t: 13,
            system_signature: SystemSignature::from_bytes([0xccu8; 64]),
        }
    }

    /// Round-trip for every typed-tx variant.
    #[test]
    fn typed_tx_round_trip_all_variants() {
        let cases: Vec<TypedTx> = vec![
            TypedTx::Work(fixture_work_tx()),
            TypedTx::Verify(fixture_verify_tx()),
            TypedTx::Challenge(fixture_challenge_tx()),
            TypedTx::Reuse(fixture_reuse_tx()),
            TypedTx::FinalizeReward(fixture_finalize_reward_tx()),
            TypedTx::TaskExpire(fixture_task_expire_tx()),
            TypedTx::TerminalSummary(fixture_terminal_summary_tx()),
        ];
        for tx in cases {
            let bytes = canonical_encode(&tx).expect("encode");
            let decoded: TypedTx = canonical_decode(&bytes).expect("decode");
            assert_eq!(tx, decoded, "round-trip mismatch on {:?}", tx.tx_kind());
        }
    }

    /// Two encodes of the same value produce byte-identical bytes.
    #[test]
    fn typed_tx_byte_stability_across_calls() {
        let tx = TypedTx::Work(fixture_work_tx());
        let bytes_a = canonical_encode(&tx).expect("encode a");
        let bytes_b = canonical_encode(&tx).expect("encode b");
        assert_eq!(bytes_a, bytes_b);
    }

    /// 100-input round-trip: random-ish AgentSignature bytes + variant choice.
    #[test]
    fn typed_tx_round_trip_100_inputs() {
        let mut tx = fixture_work_tx();
        for i in 0u32..100 {
            tx.timestamp_logical = i as u64;
            tx.signature = AgentSignature::from_bytes([(i % 256) as u8; 64]);
            let outer = TypedTx::Work(tx.clone());
            let bytes = canonical_encode(&outer).expect("encode");
            let back: TypedTx = canonical_decode(&bytes).expect("decode");
            assert_eq!(outer, back);
        }
    }

    /// HasSubmitter — agent-submitted vs system-emitted partitioning.
    #[test]
    fn has_submitter_partitioning() {
        let alice = AgentId("alice".into());
        assert_eq!(
            TypedTx::Work(fixture_work_tx()).submitter_id(),
            Some(alice.clone())
        );
        assert_eq!(
            TypedTx::Verify(fixture_verify_tx()).submitter_id(),
            Some(AgentId("bob".into()))
        );
        assert_eq!(
            TypedTx::Challenge(fixture_challenge_tx()).submitter_id(),
            Some(AgentId("carol".into()))
        );
        assert_eq!(TypedTx::Reuse(fixture_reuse_tx()).submitter_id(), None);
        assert_eq!(
            TypedTx::FinalizeReward(fixture_finalize_reward_tx()).submitter_id(),
            None
        );
        assert_eq!(
            TypedTx::TaskExpire(fixture_task_expire_tx()).submitter_id(),
            None
        );
    }

    /// tx_kind matches the LedgerEntry TxKind enum variant.
    #[test]
    fn typed_tx_kind_projection() {
        use crate::bottom_white::ledger::transition_ledger::TxKind;
        assert_eq!(TypedTx::Work(fixture_work_tx()).tx_kind(), TxKind::Work);
        assert_eq!(
            TypedTx::Verify(fixture_verify_tx()).tx_kind(),
            TxKind::Verify
        );
        assert_eq!(
            TypedTx::Challenge(fixture_challenge_tx()).tx_kind(),
            TxKind::Challenge
        );
        assert_eq!(TypedTx::Reuse(fixture_reuse_tx()).tx_kind(), TxKind::Reuse);
        assert_eq!(
            TypedTx::FinalizeReward(fixture_finalize_reward_tx()).tx_kind(),
            TxKind::FinalizeReward
        );
        assert_eq!(
            TypedTx::TaskExpire(fixture_task_expire_tx()).tx_kind(),
            TxKind::TaskExpire
        );
        assert_eq!(
            TypedTx::TerminalSummary(fixture_terminal_summary_tx()).tx_kind(),
            TxKind::TerminalSummary,
        );
    }

    // ── v1.1 NEW: cross-variant non-collision (C-2 / Codex Q-J) ──────────────

    /// All 7 TypedTx variant fixtures encode to pairwise-distinct canonical bytes.
    /// (Different field shapes + bincode variant tags → ANY collision is a bincode
    /// regression that this test catches.)
    #[test]
    fn typed_tx_cross_variant_non_collision() {
        let variants: Vec<(&str, TypedTx)> = vec![
            ("Work", TypedTx::Work(fixture_work_tx())),
            ("Verify", TypedTx::Verify(fixture_verify_tx())),
            ("Challenge", TypedTx::Challenge(fixture_challenge_tx())),
            ("Reuse", TypedTx::Reuse(fixture_reuse_tx())),
            (
                "FinalizeReward",
                TypedTx::FinalizeReward(fixture_finalize_reward_tx()),
            ),
            ("TaskExpire", TypedTx::TaskExpire(fixture_task_expire_tx())),
            (
                "TerminalSummary",
                TypedTx::TerminalSummary(fixture_terminal_summary_tx()),
            ),
        ];
        let digests: Vec<(&str, String)> = variants
            .iter()
            .map(|(name, tx)| (*name, digest_hex(tx)))
            .collect();
        for i in 0..digests.len() {
            for j in (i + 1)..digests.len() {
                assert_ne!(
                    digests[i].1, digests[j].1,
                    "{} and {} have colliding canonical digests",
                    digests[i].0, digests[j].0
                );
            }
        }
    }

    // ── v1.1 NEW: BTreeMap / BTreeSet permutation independence (C-2 / Gemini Q9) ─

    /// Building the same WorkTx via different `BTreeSet` insertion orders produces
    /// byte-identical canonical bytes. (BTreeSet iterates in sorted order, but
    /// this test locks that bincode honors the iteration order — defensive against
    /// a future codec choice that uses HashMap-style hash-randomized iteration.)
    #[test]
    fn typed_tx_btree_permutation_independence() {
        let make_work_tx = |read_keys_in_order: &[&str]| -> WorkTx {
            let mut tx = fixture_work_tx();
            tx.read_set = BTreeSet::new();
            for k in read_keys_in_order {
                tx.read_set.insert(ReadKey((*k).into()));
            }
            tx
        };
        // Insert keys in different orders.
        let tx_a = make_work_tx(&["k.read.a", "k.read.b", "k.read.c"]);
        let tx_b = make_work_tx(&["k.read.c", "k.read.a", "k.read.b"]);
        let tx_c = make_work_tx(&["k.read.b", "k.read.c", "k.read.a"]);
        let bytes_a = canonical_encode(&tx_a).expect("encode a");
        let bytes_b = canonical_encode(&tx_b).expect("encode b");
        let bytes_c = canonical_encode(&tx_c).expect("encode c");
        assert_eq!(bytes_a, bytes_b);
        assert_eq!(bytes_a, bytes_c);
    }

    // ── v1.1 NEW: zero-default round-trip per main tx kind (Gemini Q9) ──────

    #[test]
    fn typed_tx_default_round_trip() {
        let cases: Vec<TypedTx> = vec![
            TypedTx::Work(WorkTx::default()),
            TypedTx::Verify(VerifyTx::default()),
            TypedTx::Challenge(ChallengeTx::default()),
            TypedTx::Reuse(ReuseTx::default()),
            TypedTx::FinalizeReward(FinalizeRewardTx::default()),
            TypedTx::TaskExpire(TaskExpireTx::default()),
            TypedTx::TerminalSummary(TerminalSummaryTx::default()),
        ];
        for tx in cases {
            let bytes = canonical_encode(&tx).expect("encode default");
            let back: TypedTx = canonical_decode(&bytes).expect("decode default");
            assert_eq!(tx, back, "default round-trip mismatch on {:?}", tx.tx_kind());
        }
    }

    // ── v1.1 NEW: signing-payload domain-prefix non-collision (C-1) ─────────

    /// 6 signing-payload digests (Work / Verify / Challenge agent + Finalize /
    /// TaskExpire / TerminalSummary system) all have distinct domain prefixes;
    /// even if their bincode bodies COULD overlap, the SHA-256 inputs differ.
    /// We don't construct bodies that overlap (different fields); the assertion
    /// is simply that all 6 distinct domain-prefixed digests are pairwise distinct
    /// — which is the property auditors flagged as essential.
    #[test]
    fn signing_payload_domains_are_distinct() {
        let digests: Vec<(&str, [u8; 32])> = vec![
            ("Work", fixture_work_tx().to_signing_payload().canonical_digest()),
            (
                "Verify",
                fixture_verify_tx().to_signing_payload().canonical_digest(),
            ),
            (
                "Challenge",
                fixture_challenge_tx().to_signing_payload().canonical_digest(),
            ),
            (
                "FinalizeReward",
                fixture_finalize_reward_tx()
                    .to_signing_payload()
                    .canonical_digest(),
            ),
            (
                "TaskExpire",
                fixture_task_expire_tx()
                    .to_signing_payload()
                    .canonical_digest(),
            ),
            (
                "TerminalSummary",
                fixture_terminal_summary_tx()
                    .to_signing_payload()
                    .canonical_digest(),
            ),
        ];
        for i in 0..digests.len() {
            for j in (i + 1)..digests.len() {
                assert_ne!(
                    digests[i].1, digests[j].1,
                    "{} and {} signing-payload digests collide",
                    digests[i].0, digests[j].0
                );
            }
        }
    }

    /// Excluding the signature: mutating `tx.signature` must NOT change the
    /// signing-payload digest (the signature is its own input — a canonical
    /// digest cycle prevention property).
    #[test]
    fn signing_payload_excludes_signature() {
        let tx_clean = fixture_work_tx();
        let d_clean = tx_clean.to_signing_payload().canonical_digest();

        let mut tx_mut = tx_clean.clone();
        tx_mut.signature = AgentSignature::from_bytes([0xff; 64]);
        let d_mut_sig = tx_mut.to_signing_payload().canonical_digest();
        assert_eq!(d_clean, d_mut_sig, "mutating signature must NOT affect digest");

        // Sanity: mutating a SIGNED field DOES change digest.
        let mut tx_signed_change = tx_clean.clone();
        tx_signed_change.timestamp_logical = 9999;
        let d_signed = tx_signed_change.to_signing_payload().canonical_digest();
        assert_ne!(d_clean, d_signed);
    }

    // ── I-CANON-D — golden fixtures (locked SHA-256 of canonical bytes) ──────
    //
    // **v1.1 round-1 closure (C-2 / Codex Q-J / Gemini Q9)**: hex values are
    // hardcoded — any future codec / schema change causes the assertion to
    // fail, forcing a deliberate "ABI golden fixture rotation" commit with
    // re-audit. To rotate:
    //   1. Run `cargo test --lib state::typed_tx::tests::golden_` with current code
    //   2. The assertion failure messages report the new hex in the `actual` slot
    //   3. Update each `EXPECTED_HEX` constant + cite the rotation rationale in commit message

    const EXPECTED_HEX_WORK: &str =
        "6ec94fa4910ef4cc108ca8f36c202647d2cf60426d13ca0bccf777efb07b4fef";
    const EXPECTED_HEX_VERIFY: &str =
        "425b9bd7e99c427b3b7934d45a00dee3d66fc346deed72ec307de01bb3f1db99";
    const EXPECTED_HEX_CHALLENGE: &str =
        "c90be7617e9aba5a70dc8d625e654c1c712403aaf47e7734497fc0e909e8f788";
    const EXPECTED_HEX_REUSE: &str =
        "8bb33232b7c20a63a206f505179b0f64fa50acb41061aaa471ba8e4435593aed";
    const EXPECTED_HEX_FINALIZE_REWARD: &str =
        "0f5e213ec919f8e61dc998b13a4dcd49ff6e81e473850725f2ca1f27c1d65a2d";
    const EXPECTED_HEX_TASK_EXPIRE: &str =
        "835cdec950a7fd09531e03b1ab2f571ccc9a7c05b3a3e04905f0dc77078c2d60";
    const EXPECTED_HEX_TERMINAL_SUMMARY: &str =
        "f05983df19cb2af951d79216d71a64aae6b1ae960d036022f90f28039b059208";

    #[test]
    fn golden_work_tx_digest() {
        let actual = digest_hex(&TypedTx::Work(fixture_work_tx()));
        assert_eq!(actual.len(), 64);
        assert_eq!(actual, EXPECTED_HEX_WORK, "Work canonical digest changed");
    }

    #[test]
    fn golden_verify_tx_digest() {
        let actual = digest_hex(&TypedTx::Verify(fixture_verify_tx()));
        assert_eq!(actual, EXPECTED_HEX_VERIFY);
    }

    #[test]
    fn golden_challenge_tx_digest() {
        let actual = digest_hex(&TypedTx::Challenge(fixture_challenge_tx()));
        assert_eq!(actual, EXPECTED_HEX_CHALLENGE);
    }

    #[test]
    fn golden_reuse_tx_digest() {
        let actual = digest_hex(&TypedTx::Reuse(fixture_reuse_tx()));
        assert_eq!(actual, EXPECTED_HEX_REUSE);
    }

    #[test]
    fn golden_finalize_reward_tx_digest() {
        let actual = digest_hex(&TypedTx::FinalizeReward(fixture_finalize_reward_tx()));
        assert_eq!(actual, EXPECTED_HEX_FINALIZE_REWARD);
    }

    #[test]
    fn golden_task_expire_tx_digest() {
        let actual = digest_hex(&TypedTx::TaskExpire(fixture_task_expire_tx()));
        assert_eq!(actual, EXPECTED_HEX_TASK_EXPIRE);
    }

    #[test]
    fn golden_terminal_summary_tx_digest() {
        let actual = digest_hex(&TypedTx::TerminalSummary(fixture_terminal_summary_tx()));
        assert_eq!(actual, EXPECTED_HEX_TERMINAL_SUMMARY);
    }
}

```

---

# Supporting v1.1: src/state/mod.rs (re-exports)

```rust
//! TRACE_MATRIX Art 0.1: 四要素映射 (Tape / Input-Tape / Q / State).
//! TRACE_MATRIX Art 0.4: Q_t version-controlled state vector.
//! TRACE_MATRIX WP § 4: 9-component system state Q_t.
//! TRACE_MATRIX WP § 0 axiom 1: state monotonicity.
//!
//! Atom: CO1.2 (Q_t struct) — implements `STATE_TRANSITION_SPEC v1.4 § 1.1`.
//! All public re-exports below are surface for the same TRACE_MATRIX rows.

/// TRACE_MATRIX Art 0.4 / WP § 4 — Q_t module: implements all 9 system state fields.
pub mod q_state;

/// TRACE_MATRIX FC2-Submit / CO1.1.4-pre1 — typed-tx ABI surface (TypedTx + per-kind structs).
pub mod typed_tx;

pub use q_state::{
    AgentId, AgentSwarmState, AgentVisibleProjection, BalancesIndex, BudgetSnapshot,
    ChallengeCase, ChallengeCasesIndex, ClaimEntry, ClaimsIndex, EconomicState, EscrowEntry,
    EscrowsIndex, Hash, NodeId, PerAgentState, PriceIndex, QState, Reputation, ReputationsIndex,
    RoyaltyEdge, RoyaltyGraph, StakeEntry, StakesIndex, TaskMarketEntry, TaskMarketsIndex, TxId,
};

pub use typed_tx::{
    AgentSignature, BoolWithProof, ChallengeSigningPayload, ChallengeTx, ClaimId,
    FinalizeRewardSigningPayload, FinalizeRewardTx, HasSubmitter, PredicateId,
    PredicateResultsBundle, ReadKey, RejectionClass, ReuseTx, RunId, RunOutcome,
    SafetyOrCreation, SignalBundle, SignalKind, SlashEvidenceCid, TaskExpireSigningPayload,
    TaskExpireTx, TaskId, TerminalSummarySigningPayload, TerminalSummaryTx, ToolId,
    TransitionError, TxStatus, TypedTx, VerifySigningPayload, VerifyTx, VerifyVerdict,
    WorkSigningPayload, WorkTx, WriteKey,
};

```

---

# Supporting v1.1: src/bottom_white/ledger/system_keypair.rs (TerminalSummary migration target)

```rust
//! Runtime system keypair lifecycle per
//! `handover/specs/SYSTEM_KEYPAIR_SECURITY_v1_2026-04-27.md`.
//!
//! The private key is generated from OS entropy via `getrandom(2)`, encrypted
//! at rest with ChaCha20-Poly1305, and protected by Argon2id using RFC 9106 /
//! OWASP-class defaults: m=64 MiB, t=3, p=4. The KDF parameters are read from
//! environment variables so deployments can ratchet cost without code churn.
//!
//! /// TRACE_MATRIX FC1-Sig+FC3-Sig: runtime attribution signature primitive

use crate::boot::TrustRootError;
use argon2::{Algorithm, Argon2, Params, Version};
use chacha20poly1305::aead::{Aead, KeyInit};
use chacha20poly1305::{ChaCha20Poly1305, Nonce};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::env;
use std::fmt;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use zeroize::{Zeroize, ZeroizeOnDrop};

const DEFAULT_KDF_MEMORY_KIB: u32 = 65_536;
const DEFAULT_KDF_ITER: u32 = 3;
const DEFAULT_KDF_LANES: u32 = 4;
const DERIVED_KEY_LEN: usize = 32;
const SECRET_KEY_LEN: usize = 32;
const PUBLIC_KEY_LEN: usize = 32;
const SIGNATURE_LEN: usize = 64;
const SALT_LEN: usize = 16;
const NONCE_LEN: usize = 12;
const PLAINTEXT_LEN: usize = SECRET_KEY_LEN + PUBLIC_KEY_LEN;
const FORMAT_MAGIC: &[u8; 11] = b"TOS4SYSKEY1";
const FORMAT_VERSION: u8 = 1;

/// TRACE_MATRIX FC1-Sig+FC3-Sig: system signature epoch identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct SystemEpoch(u64);

impl SystemEpoch {
    /// TRACE_MATRIX FC1-Sig+FC3-Sig: construct a system signature epoch.
    pub const fn new(epoch: u64) -> Self {
        Self(epoch)
    }

    /// TRACE_MATRIX FC1-Sig+FC3-Sig: expose the numeric epoch for canonical encoding.
    pub const fn get(self) -> u64 {
        self.0
    }
}

impl From<u64> for SystemEpoch {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

/// TRACE_MATRIX FC1-Sig+FC3-Sig: ed25519 public key pinned by epoch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SystemPublicKey([u8; PUBLIC_KEY_LEN]);

impl SystemPublicKey {
    /// TRACE_MATRIX FC1-Sig+FC3-Sig: construct a system public key from raw ed25519 bytes.
    pub const fn from_bytes(bytes: [u8; PUBLIC_KEY_LEN]) -> Self {
        Self(bytes)
    }

    /// TRACE_MATRIX FC1-Sig+FC3-Sig: expose raw public key bytes for pinning and verification.
    pub const fn as_bytes(&self) -> &[u8; PUBLIC_KEY_LEN] {
        &self.0
    }

    /// TRACE_MATRIX FC3-Sig: stable SHA-256 fingerprint for audit logs and rotation records.
    pub fn fingerprint_sha256(&self) -> [u8; 32] {
        Sha256::digest(self.0).into()
    }
}

/// TRACE_MATRIX FC1-Sig+FC3-Sig: ed25519 detached signature over a canonical system message digest.
///
/// `[u8; 64]` serde via `serde_bytes_64` (serde-derive default doesn't support
/// arrays > 32). With `bincode` + `fixed_int_encoding` this writes 64 raw bytes —
/// deterministic, platform-stable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemSignature(#[serde(with = "serde_bytes_64")] [u8; SIGNATURE_LEN]);

impl Default for SystemSignature {
    fn default() -> Self {
        Self([0u8; SIGNATURE_LEN])
    }
}

/// Serde adapter for `[u8; 64]`: serializes as a length-64 byte sequence
/// (deterministic under bincode `fixed_int_encoding` → 64 raw bytes; no length prefix
/// because the ARRAY type encodes its length statically).
///
/// `pub(crate)` so other in-crate types with `[u8; 64]` fields (e.g.
/// `state::typed_tx::AgentSignature`) can reuse the same adapter — keeps the
/// serde wire format byte-identical across all 64-byte signature types.
pub(crate) mod serde_bytes_64 {
    use serde::de::{SeqAccess, Visitor};
    use serde::ser::SerializeTuple;
    use serde::{Deserializer, Serializer};
    use std::fmt;

    pub fn serialize<S: Serializer>(bytes: &[u8; 64], s: S) -> Result<S::Ok, S::Error> {
        let mut tup = s.serialize_tuple(64)?;
        for b in bytes {
            tup.serialize_element(b)?;
        }
        tup.end()
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<[u8; 64], D::Error> {
        struct ArrVisitor;
        impl<'de> Visitor<'de> for ArrVisitor {
            type Value = [u8; 64];
            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "byte array of length 64")
            }
            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                let mut out = [0u8; 64];
                for (i, slot) in out.iter_mut().enumerate() {
                    *slot = seq
                        .next_element()?
                        .ok_or_else(|| serde::de::Error::invalid_length(i, &self))?;
                }
                Ok(out)
            }
        }
        d.deserialize_tuple(64, ArrVisitor)
    }
}

impl SystemSignature {
    /// TRACE_MATRIX FC1-Sig+FC3-Sig: construct a detached system signature from raw ed25519 bytes.
    pub const fn from_bytes(bytes: [u8; SIGNATURE_LEN]) -> Self {
        Self(bytes)
    }

    /// TRACE_MATRIX FC1-Sig+FC3-Sig: expose raw signature bytes for tape serialization.
    pub const fn as_bytes(&self) -> &[u8; SIGNATURE_LEN] {
        &self.0
    }
}

/// TRACE_MATRIX FC1-Sig: typed rejection summary stamped by the predicate runner.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RejectedAttemptSummary {
    run_id: String,
    attempt_id: String,
    failure_class: String,
    summary_hash: [u8; 32],
}

impl RejectedAttemptSummary {
    /// TRACE_MATRIX FC1-Sig: construct a typed rejected-attempt summary, never a free-form sign blob.
    pub fn new(
        run_id: impl Into<String>,
        attempt_id: impl Into<String>,
        failure_class: impl Into<String>,
        summary_hash: [u8; 32],
    ) -> Self {
        Self {
            run_id: run_id.into(),
            attempt_id: attempt_id.into(),
            failure_class: failure_class.into(),
            summary_hash,
        }
    }
}

// TRACE_MATRIX CO1.1.4-pre1 v1.1 round-1 closure (C-3 / Codex Q-C):
// the typed `TerminalSummaryTx` struct (8-field per STATE § 1.5) now lives in
// `state::typed_tx`. system_keypair signs an opaque digest via the
// `CanonicalMessage::TerminalSummarySigning([u8; 32])` variant — same
// opaque-digest pattern as `LedgerEntrySigning`, avoiding `bottom_white ↔ state`
// circular dependency.

/// TRACE_MATRIX FC3-Sig: typed continuity statement for system key rotation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EpochRotationProof {
    old_epoch: SystemEpoch,
    new_epoch: SystemEpoch,
    old_pubkey: SystemPublicKey,
    new_pubkey: SystemPublicKey,
    signed_at_unix: u64,
}

impl EpochRotationProof {
    /// TRACE_MATRIX FC3-Sig: construct a typed epoch-rotation continuity proof.
    pub const fn new(
        old_epoch: SystemEpoch,
        new_epoch: SystemEpoch,
        old_pubkey: SystemPublicKey,
        new_pubkey: SystemPublicKey,
        signed_at_unix: u64,
    ) -> Self {
        Self {
            old_epoch,
            new_epoch,
            old_pubkey,
            new_pubkey,
            signed_at_unix,
        }
    }

    /// TRACE_MATRIX FC3-Sig: old signing epoch certified by the rotation proof.
    pub const fn old_epoch(&self) -> SystemEpoch {
        self.old_epoch
    }

    /// TRACE_MATRIX FC3-Sig: new signing epoch certified by the rotation proof.
    pub const fn new_epoch(&self) -> SystemEpoch {
        self.new_epoch
    }
}

/// TRACE_MATRIX FC1-Sig+FC3-Sig: only typed runtime messages may enter signature verification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CanonicalMessage {
    /// TRACE_MATRIX FC1-Sig: predicate-runner rejection summary.
    RejectedAttemptSummary(RejectedAttemptSummary),
    /// TRACE_MATRIX FC1-Sig+FC3-Sig (CO1.1.4-pre1 v1.1 closure C-3): terminal
    /// summary signing-payload digest. Opaque `[u8; 32]` — full canonical_digest
    /// of the 8-field `state::typed_tx::TerminalSummaryTx` is computed in
    /// typed_tx; this variant only carries the 32-byte digest into the typed
    /// sign API. Same opaque-digest pattern as `LedgerEntrySigning`; avoids a
    /// circular `system_keypair ↔ state` module dependency.
    TerminalSummarySigning([u8; 32]),
    /// TRACE_MATRIX FC3-Sig: system key epoch continuity proof.
    EpochRotationProof(EpochRotationProof),
    /// TRACE_MATRIX FC2-Append (CO1.7 v1.2 round-2 closure C3): L4 transition_ledger
    /// signing payload digest. Opaque [u8; 32] — full canonical_digest of
    /// `LedgerEntrySigningPayload` is computed in `transition_ledger`; this variant
    /// only carries the 32-byte digest into the typed sign API. Avoids a circular
    /// `system_keypair ↔ transition_ledger` module dependency while preserving the
    /// "all sign goes through CanonicalMessage" invariant.
    LedgerEntrySigning([u8; 32]),
}

/// TRACE_MATRIX FC1-Sig+FC3-Sig: epoch-indexed public keys pinned by genesis and rotation history.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PinnedSystemPubkeys {
    keys: BTreeMap<SystemEpoch, SystemPublicKey>,
}

impl PinnedSystemPubkeys {
    /// TRACE_MATRIX FC1-Sig+FC3-Sig: create an empty pinned system-key map.
    pub fn new() -> Self {
        Self::default()
    }

    /// TRACE_MATRIX FC1-Sig+FC3-Sig: pin a public key for a system epoch.
    pub fn insert(
        &mut self,
        epoch: SystemEpoch,
        public_key: SystemPublicKey,
    ) -> Option<SystemPublicKey> {
        self.keys.insert(epoch, public_key)
    }

    /// TRACE_MATRIX FC1-Sig+FC3-Sig: fetch the public key pinned for a system epoch.
    pub fn get(&self, epoch: SystemEpoch) -> Option<&SystemPublicKey> {
        self.keys.get(&epoch)
    }
}

/// TRACE_MATRIX FC1-Sig+FC3-Sig: in-memory ed25519 system keypair with zeroized private key on drop.
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct Ed25519Keypair {
    secret_key: Box<[u8]>,
    #[zeroize(skip)]
    public_key: SystemPublicKey,
}

impl Ed25519Keypair {
    /// TRACE_MATRIX FC1-Sig+FC3-Sig: generate ed25519 key material from `getrandom(2)` entropy.
    pub fn generate_with_secure_entropy() -> Result<Self, KeypairError> {
        let mut seed = [0u8; SECRET_KEY_LEN];
        getrandom::getrandom(&mut seed).map_err(KeypairError::Entropy)?;
        let signing_key = SigningKey::from_bytes(&seed);
        let public_key = SystemPublicKey::from_bytes(signing_key.verifying_key().to_bytes());
        let mut keypair = Self {
            secret_key: Vec::from(seed).into_boxed_slice(),
            public_key,
        };
        seed.zeroize();
        keypair.mlock_private_key_best_effort();
        Ok(keypair)
    }

    /// TRACE_MATRIX FC1-Sig+FC3-Sig: return the public half of the system keypair.
    pub const fn public_key(&self) -> SystemPublicKey {
        self.public_key
    }

    fn from_plaintext(plaintext: &[u8]) -> Result<Self, KeypairError> {
        if plaintext.len() != PLAINTEXT_LEN {
            return Err(KeypairError::InvalidFormat("bad plaintext length"));
        }
        let mut secret = [0u8; SECRET_KEY_LEN];
        secret.copy_from_slice(&plaintext[..SECRET_KEY_LEN]);
        let mut public = [0u8; PUBLIC_KEY_LEN];
        public.copy_from_slice(&plaintext[SECRET_KEY_LEN..]);

        let signing_key = SigningKey::from_bytes(&secret);
        if signing_key.verifying_key().to_bytes() != public {
            secret.zeroize();
            return Err(KeypairError::InvalidFormat(
                "public key does not match private key",
            ));
        }

        let mut keypair = Self {
            secret_key: Vec::from(secret).into_boxed_slice(),
            public_key: SystemPublicKey::from_bytes(public),
        };
        secret.zeroize();
        keypair.mlock_private_key_best_effort();
        Ok(keypair)
    }

    fn to_plaintext(&self) -> Result<[u8; PLAINTEXT_LEN], KeypairError> {
        let secret = self.secret_slice()?;
        let mut plaintext = [0u8; PLAINTEXT_LEN];
        plaintext[..SECRET_KEY_LEN].copy_from_slice(secret);
        plaintext[SECRET_KEY_LEN..].copy_from_slice(self.public_key.as_bytes());
        Ok(plaintext)
    }

    fn sign_digest(&self, digest: [u8; 32]) -> Result<SystemSignature, KeypairError> {
        let mut secret = [0u8; SECRET_KEY_LEN];
        secret.copy_from_slice(self.secret_slice()?);
        let signing_key = SigningKey::from_bytes(&secret);
        let signature = signing_key.sign(&digest);
        secret.zeroize();
        Ok(SystemSignature::from_bytes(signature.to_bytes()))
    }

    fn secret_slice(&self) -> Result<&[u8], KeypairError> {
        if self.secret_key.len() == SECRET_KEY_LEN {
            Ok(&self.secret_key)
        } else {
            Err(KeypairError::InvalidFormat("bad in-memory secret length"))
        }
    }

    fn mlock_private_key_best_effort(&mut self) -> bool {
        mlock_best_effort(self.secret_key.as_ptr(), self.secret_key.len())
    }
}

/// TRACE_MATRIX FC1-Sig+FC3-Sig: system keypair lifecycle and crypto error taxonomy.
#[derive(Debug)]
pub enum KeypairError {
    /// TRACE_MATRIX FC1-Sig+FC3-Sig: filesystem operation failed.
    Io(std::io::Error),
    /// TRACE_MATRIX FC1-Sig+FC3-Sig: secure operating-system entropy failed.
    Entropy(getrandom::Error),
    /// TRACE_MATRIX FC1-Sig+FC3-Sig: KDF environment parameter was absent or invalid.
    KdfParam(String),
    /// TRACE_MATRIX FC1-Sig+FC3-Sig: Argon2id key derivation failed.
    Kdf(argon2::Error),
    /// TRACE_MATRIX FC1-Sig+FC3-Sig: ChaCha20-Poly1305 encryption or authentication failed.
    Crypto(&'static str),
    /// TRACE_MATRIX FC1-Sig+FC3-Sig: encrypted keystore format was malformed.
    InvalidFormat(&'static str),
    /// TRACE_MATRIX FC1-Sig+FC3-Sig: default keystore path could not be resolved.
    HomeUnavailable,
}

impl fmt::Display for KeypairError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(err) => write!(f, "system keypair I/O failed: {err}"),
            Self::Entropy(err) => write!(f, "system keypair entropy failed: {err}"),
            Self::KdfParam(msg) => write!(f, "system keypair KDF parameter invalid: {msg}"),
            Self::Kdf(err) => write!(f, "system keypair KDF failed: {err}"),
            Self::Crypto(msg) => write!(f, "system keypair crypto failed: {msg}"),
            Self::InvalidFormat(msg) => write!(f, "system keypair keystore invalid: {msg}"),
            Self::HomeUnavailable => {
                write!(f, "system keypair default keystore path requires HOME")
            }
        }
    }
}

impl std::error::Error for KeypairError {}

impl From<std::io::Error> for KeypairError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

/// TRACE_MATRIX FC1-Sig+FC3-Sig: resolve `~/.turingos/keystore/system_keypair_v{epoch}.enc`.
///
/// `TURINGOS_KEYSTORE_PATH` overrides the default path. The default never
/// points into the repository, CAS, or ledger directories.
pub fn default_system_keystore_path(epoch: SystemEpoch) -> Result<PathBuf, KeypairError> {
    if let Ok(path) = env::var("TURINGOS_KEYSTORE_PATH") {
        return Ok(PathBuf::from(path));
    }
    let home = env::var("HOME").map_err(|_| KeypairError::HomeUnavailable)?;
    Ok(PathBuf::from(home)
        .join(".turingos")
        .join("keystore")
        .join(format!("system_keypair_v{}.enc", epoch.get())))
}

/// TRACE_MATRIX FC1-Sig+FC3-Sig: first-boot generate-or-second-boot decrypt lifecycle entrypoint.
pub fn generate_or_load_system_keypair(
    keystore_path: &Path,
    user_kdf_password: &SecretString,
) -> Result<Ed25519Keypair, KeypairError> {
    if keystore_path.exists() {
        return load_existing_keypair(keystore_path, user_kdf_password);
    }

    let keypair = Ed25519Keypair::generate_with_secure_entropy()?;
    let encrypted = encrypt_at_rest(&keypair, user_kdf_password)?;
    write_keystore_0600(keystore_path, &encrypted)?;
    Ok(keypair)
}

/// TRACE_MATRIX FC1-Sig+FC3-Sig: decrypt an existing encrypted system keypair keystore.
pub fn load_existing_keypair(
    keystore_path: &Path,
    user_kdf_password: &SecretString,
) -> Result<Ed25519Keypair, KeypairError> {
    let bytes = fs::read(keystore_path)?;
    let encoded = EncryptedKeypair::decode(&bytes)?;
    let mut key = derive_key(user_kdf_password, &encoded.salt, encoded.kdf)?;
    let cipher = ChaCha20Poly1305::new_from_slice(&key)
        .map_err(|_| KeypairError::Crypto("bad cipher key"))?;
    let plaintext = cipher
        .decrypt(
            Nonce::from_slice(&encoded.nonce),
            encoded.ciphertext.as_ref(),
        )
        .map_err(|_| KeypairError::Crypto("keystore authentication failed"))?;
    key.zeroize();
    Ed25519Keypair::from_plaintext(&plaintext)
}

/// TRACE_MATRIX FC1-Sig+FC3-Sig: canonical SHA-256 digest for typed system messages.
pub fn canonical_digest(message: &CanonicalMessage) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(b"turingosv4.system_keypair.v1");
    match message {
        CanonicalMessage::RejectedAttemptSummary(summary) => {
            h.update(b"RejectedAttemptSummary");
            update_len_prefixed(&mut h, summary.run_id.as_bytes());
            update_len_prefixed(&mut h, summary.attempt_id.as_bytes());
            update_len_prefixed(&mut h, summary.failure_class.as_bytes());
            h.update(summary.summary_hash);
        }
        CanonicalMessage::TerminalSummarySigning(digest) => {
            h.update(b"TerminalSummarySigning");
            h.update(digest);
        }
        CanonicalMessage::EpochRotationProof(proof) => {
            h.update(b"EpochRotationProof");
            h.update(proof.old_epoch.get().to_be_bytes());
            h.update(proof.new_epoch.get().to_be_bytes());
            h.update(proof.old_pubkey.as_bytes());
            h.update(proof.new_pubkey.as_bytes());
            h.update(proof.signed_at_unix.to_be_bytes());
        }
        CanonicalMessage::LedgerEntrySigning(digest) => {
            h.update(b"LedgerEntrySigning");
            h.update(digest);
        }
    }
    h.finalize().into()
}

/// TRACE_MATRIX FC1-Sig+FC3-Sig: public system signature verification against pinned epoch keys.
pub fn verify_system_signature(
    sig: &SystemSignature,
    message: &CanonicalMessage,
    epoch: SystemEpoch,
    pinned_pubkeys: &PinnedSystemPubkeys,
) -> bool {
    let Some(pk) = pinned_pubkeys.get(epoch) else {
        return false;
    };
    let Ok(verifying_key) = VerifyingKey::from_bytes(pk.as_bytes()) else {
        return false;
    };
    let signature = Signature::from_bytes(sig.as_bytes());
    verifying_key
        .verify(&canonical_digest(message), &signature)
        .is_ok()
}

/// TRACE_MATRIX FC3-Sig: verify old and new signatures over a rotation continuity proof.
pub fn verify_epoch_rotation_proof(
    proof: &EpochRotationProof,
    old_signature: &SystemSignature,
    new_signature: &SystemSignature,
    pinned_pubkeys: &PinnedSystemPubkeys,
) -> bool {
    let message = CanonicalMessage::EpochRotationProof(proof.clone());
    verify_system_signature(old_signature, &message, proof.old_epoch(), pinned_pubkeys)
        && verify_system_signature(new_signature, &message, proof.new_epoch(), pinned_pubkeys)
}

/// TRACE_MATRIX FC3-Sig: boot extension stub for genesis `[system_pubkeys]` verification.
pub fn verify_system_pubkeys(genesis_payload_toml: &str) -> Result<(), TrustRootError> {
    if !has_toml_section(genesis_payload_toml, "system_pubkeys") {
        return Ok(());
    }
    // TODO(CO1.7): parse genesis_payload.toml [system_pubkeys] entries and
    // verify creator PGP signatures against the pinned creator public key.
    Ok(())
}

/// TRACE_MATRIX FC1-Sig: crate-only signing surface for the predicate runner.
pub(crate) mod predicate_runner {
    use super::{
        sign_system_message_inner, CanonicalMessage, Ed25519Keypair, KeypairError,
        RejectedAttemptSummary, SystemSignature,
    };

    /// TRACE_MATRIX FC1-Sig: sign only typed rejected-attempt summaries from the predicate runner.
    pub(crate) fn sign_rejected_attempt_summary(
        keypair: &Ed25519Keypair,
        summary: &RejectedAttemptSummary,
    ) -> Result<SystemSignature, KeypairError> {
        sign_system_message_inner(
            keypair,
            &CanonicalMessage::RejectedAttemptSummary(summary.clone()),
        )
    }

    /// TRACE_MATRIX FC1-Sig: sign only typed canonical messages within the predicate-runner scope.
    pub(crate) fn sign_system_message(
        keypair: &Ed25519Keypair,
        message: &CanonicalMessage,
    ) -> Result<SystemSignature, KeypairError> {
        sign_system_message_inner(keypair, message)
    }
}

/// TRACE_MATRIX FC1-Sig+FC3-Sig: crate-only signing surface for terminal summary emission.
///
/// **CO1.1.4-pre1 v1.1 round-1 closure (C-3)**: signs an opaque `[u8; 32]`
/// digest produced by `state::typed_tx::TerminalSummaryTx::canonical_digest()`
/// (same opaque-digest pattern as `transition_ledger_emitter::sign_ledger_entry`)
/// rather than the typed struct directly — keeps `system_keypair` oblivious
/// to the typed-tx schema, no `bottom_white ↔ state` circular dep.
pub(crate) mod terminal_summary_emitter {
    use super::{
        sign_system_message_inner, CanonicalMessage, Ed25519Keypair, EpochRotationProof,
        KeypairError, SystemSignature,
    };

    /// TRACE_MATRIX FC1-Sig+FC3-Sig: sign an opaque 32-byte digest of a
    /// terminal-summary signing payload (computed by typed_tx).
    pub(crate) fn sign_terminal_summary(
        keypair: &Ed25519Keypair,
        digest: [u8; 32],
    ) -> Result<SystemSignature, KeypairError> {
        sign_system_message_inner(keypair, &CanonicalMessage::TerminalSummarySigning(digest))
    }

    /// TRACE_MATRIX FC3-Sig: sign only typed epoch rotation proofs.
    pub(crate) fn sign_epoch_rotation_proof(
        keypair: &Ed25519Keypair,
        proof: &EpochRotationProof,
    ) -> Result<SystemSignature, KeypairError> {
        sign_system_message_inner(
            keypair,
            &CanonicalMessage::EpochRotationProof(proof.clone()),
        )
    }

    /// TRACE_MATRIX FC1-Sig+FC3-Sig: sign only typed canonical messages within terminal-summary scope.
    pub(crate) fn sign_system_message(
        keypair: &Ed25519Keypair,
        message: &CanonicalMessage,
    ) -> Result<SystemSignature, KeypairError> {
        sign_system_message_inner(keypair, message)
    }
}

/// TRACE_MATRIX FC2-Append + FC1-Sig: crate-only signing surface for the L4
/// transition ledger sequencer (CO1.7 v1.2). Authorized emitter pattern per
/// round-1 audit Q-G recommendation: the ledger sequencer calls
/// `sign_ledger_entry` with the canonical digest of `LedgerEntrySigningPayload`
/// and gets back a `SystemSignature` bound through `CanonicalMessage`. No raw
/// digest signer escapes this module.
pub(crate) mod transition_ledger_emitter {
    use super::{
        sign_system_message_inner, CanonicalMessage, Ed25519Keypair, KeypairError, SystemSignature,
    };

    /// TRACE_MATRIX FC2-Append: sign only the canonical-digest of a
    /// `LedgerEntrySigningPayload`. Caller (sequencer in CO1.7) is responsible
    /// for computing the digest; this fn only wraps in the typed enum.
    pub(crate) fn sign_ledger_entry(
        keypair: &Ed25519Keypair,
        signing_payload_digest: [u8; 32],
    ) -> Result<SystemSignature, KeypairError> {
        sign_system_message_inner(
            keypair,
            &CanonicalMessage::LedgerEntrySigning(signing_payload_digest),
        )
    }
}

fn sign_system_message_inner(
    keypair: &Ed25519Keypair,
    message: &CanonicalMessage,
) -> Result<SystemSignature, KeypairError> {
    keypair.sign_digest(canonical_digest(message))
}

fn encrypt_at_rest(
    keypair: &Ed25519Keypair,
    user_kdf_password: &SecretString,
) -> Result<Vec<u8>, KeypairError> {
    let kdf = KdfParams::from_env()?;
    let mut salt = [0u8; SALT_LEN];
    let mut nonce = [0u8; NONCE_LEN];
    getrandom::getrandom(&mut salt).map_err(KeypairError::Entropy)?;
    getrandom::getrandom(&mut nonce).map_err(KeypairError::Entropy)?;

    let mut key = derive_key(user_kdf_password, &salt, kdf)?;
    let cipher = ChaCha20Poly1305::new_from_slice(&key)
        .map_err(|_| KeypairError::Crypto("bad cipher key"))?;
    let mut plaintext = keypair.to_plaintext()?;
    let ciphertext = cipher
        .encrypt(Nonce::from_slice(&nonce), plaintext.as_ref())
        .map_err(|_| KeypairError::Crypto("keystore encryption failed"))?;
    plaintext.zeroize();
    key.zeroize();

    EncryptedKeypair {
        kdf,
        salt,
        nonce,
        ciphertext,
    }
    .encode()
}

fn derive_key(
    user_kdf_password: &SecretString,
    salt: &[u8; SALT_LEN],
    kdf: KdfParams,
) -> Result<[u8; DERIVED_KEY_LEN], KeypairError> {
    let params = Params::new(
        kdf.memory_kib,
        kdf.iterations,
        kdf.lanes,
        Some(DERIVED_KEY_LEN),
    )
    .map_err(|err| KeypairError::KdfParam(err.to_string()))?;
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
    let mut key = [0u8; DERIVED_KEY_LEN];
    argon2
        .hash_password_into(user_kdf_password.expose_secret().as_bytes(), salt, &mut key)
        .map_err(KeypairError::Kdf)?;
    Ok(key)
}

fn write_keystore_0600(path: &Path, bytes: &[u8]) -> Result<(), KeypairError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    set_open_options_mode_0600(&mut options);
    let mut file = options.open(path)?;
    file.write_all(bytes)?;
    file.sync_all()?;
    set_file_permissions_0600(path)?;
    Ok(())
}

#[cfg(unix)]
fn set_open_options_mode_0600(options: &mut OpenOptions) {
    use std::os::unix::fs::OpenOptionsExt;
    options.mode(0o600);
}

#[cfg(not(unix))]
fn set_open_options_mode_0600(_options: &mut OpenOptions) {}

#[cfg(unix)]
fn set_file_permissions_0600(path: &Path) -> Result<(), KeypairError> {
    use std::os::unix::fs::PermissionsExt;
    fs::set_permissions(path, fs::Permissions::from_mode(0o600)).map_err(KeypairError::Io)
}

#[cfg(not(unix))]
fn set_file_permissions_0600(_path: &Path) -> Result<(), KeypairError> {
    Ok(())
}

#[derive(Debug, Clone, Copy)]
struct KdfParams {
    memory_kib: u32,
    iterations: u32,
    lanes: u32,
}

impl KdfParams {
    fn from_env() -> Result<Self, KeypairError> {
        Ok(Self {
            memory_kib: read_env_u32("TURINGOS_KDF_MEMORY_KIB", DEFAULT_KDF_MEMORY_KIB)?,
            iterations: read_env_u32("TURINGOS_KDF_ITER", DEFAULT_KDF_ITER)?,
            lanes: read_env_u32("TURINGOS_KDF_LANES", DEFAULT_KDF_LANES)?,
        })
    }
}

fn read_env_u32(name: &str, default: u32) -> Result<u32, KeypairError> {
    match env::var(name) {
        Ok(value) => {
            let parsed = value
                .parse::<u32>()
                .map_err(|_| KeypairError::KdfParam(format!("{name} must be u32")))?;
            if parsed == 0 {
                return Err(KeypairError::KdfParam(format!("{name} must be non-zero")));
            }
            Ok(parsed)
        }
        Err(env::VarError::NotPresent) => Ok(default),
        Err(env::VarError::NotUnicode(_)) => {
            Err(KeypairError::KdfParam(format!("{name} is not unicode")))
        }
    }
}

struct EncryptedKeypair {
    kdf: KdfParams,
    salt: [u8; SALT_LEN],
    nonce: [u8; NONCE_LEN],
    ciphertext: Vec<u8>,
}

impl EncryptedKeypair {
    fn encode(self) -> Result<Vec<u8>, KeypairError> {
        let ciphertext_len = u32::try_from(self.ciphertext.len())
            .map_err(|_| KeypairError::InvalidFormat("ciphertext too large"))?;
        let mut out = Vec::with_capacity(
            FORMAT_MAGIC.len() + 1 + 4 + 4 + 4 + SALT_LEN + NONCE_LEN + 4 + self.ciphertext.len(),
        );
        out.extend_from_slice(FORMAT_MAGIC);
        out.push(FORMAT_VERSION);
        out.extend_from_slice(&self.kdf.memory_kib.to_be_bytes());
        out.extend_from_slice(&self.kdf.iterations.to_be_bytes());
        out.extend_from_slice(&self.kdf.lanes.to_be_bytes());
        out.extend_from_slice(&self.salt);
        out.extend_from_slice(&self.nonce);
        out.extend_from_slice(&ciphertext_len.to_be_bytes());
        out.extend_from_slice(&self.ciphertext);
        Ok(out)
    }

    fn decode(bytes: &[u8]) -> Result<Self, KeypairError> {
        let mut cursor = Cursor::new(bytes);
        if cursor.read(FORMAT_MAGIC.len())? != FORMAT_MAGIC {
            return Err(KeypairError::InvalidFormat("bad magic"));
        }
        if cursor.read_u8()? != FORMAT_VERSION {
            return Err(KeypairError::InvalidFormat("bad version"));
        }
        let kdf = KdfParams {
            memory_kib: cursor.read_u32()?,
            iterations: cursor.read_u32()?,
            lanes: cursor.read_u32()?,
        };
        let mut salt = [0u8; SALT_LEN];
        salt.copy_from_slice(cursor.read(SALT_LEN)?);
        let mut nonce = [0u8; NONCE_LEN];
        nonce.copy_from_slice(cursor.read(NONCE_LEN)?);
        let ciphertext_len = cursor.read_u32()? as usize;
        let ciphertext = cursor.read(ciphertext_len)?.to_vec();
        if !cursor.is_finished() {
            return Err(KeypairError::InvalidFormat("trailing bytes"));
        }
        Ok(Self {
            kdf,
            salt,
            nonce,
            ciphertext,
        })
    }
}

struct Cursor<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> Cursor<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, pos: 0 }
    }

    fn read(&mut self, len: usize) -> Result<&'a [u8], KeypairError> {
        let end = self
            .pos
            .checked_add(len)
            .ok_or(KeypairError::InvalidFormat("offset overflow"))?;
        if end > self.bytes.len() {
            return Err(KeypairError::InvalidFormat("truncated keystore"));
        }
        let slice = &self.bytes[self.pos..end];
        self.pos = end;
        Ok(slice)
    }

    fn read_u8(&mut self) -> Result<u8, KeypairError> {
        Ok(self.read(1)?[0])
    }

    fn read_u32(&mut self) -> Result<u32, KeypairError> {
        let mut out = [0u8; 4];
        out.copy_from_slice(self.read(4)?);
        Ok(u32::from_be_bytes(out))
    }

    fn is_finished(&self) -> bool {
        self.pos == self.bytes.len()
    }
}

fn update_len_prefixed(h: &mut Sha256, bytes: &[u8]) {
    h.update((bytes.len() as u64).to_be_bytes());
    h.update(bytes);
}

fn has_toml_section(text: &str, name: &str) -> bool {
    text.lines().any(|raw| {
        let line = strip_comment(raw).trim();
        line.strip_prefix('[')
            .and_then(|s| s.strip_suffix(']'))
            .map(|header| header.trim() == name)
            .unwrap_or(false)
    })
}

fn strip_comment(line: &str) -> &str {
    let mut in_string = false;
    for (i, c) in line.char_indices() {
        match c {
            '"' => in_string = !in_string,
            '#' if !in_string => return &line[..i],
            _ => {}
        }
    }
    line
}

fn mlock_best_effort(ptr: *const u8, len: usize) -> bool {
    if ptr.is_null() || len == 0 {
        return false;
    }
    mlock_os_best_effort(ptr, len)
}

#[cfg(unix)]
fn mlock_os_best_effort(ptr: *const u8, len: usize) -> bool {
    // SAFETY: `ptr` and `len` come from a live boxed private-key byte slice in
    // `Ed25519Keypair`; mlock does not take ownership and failure is non-fatal.
    unsafe { libc::mlock(ptr.cast(), len) == 0 }
}

#[cfg(not(unix))]
fn mlock_os_best_effort(_ptr: *const u8, _len: usize) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn authorized_scope_signing_round_trip() {
        let keypair = Ed25519Keypair::generate_with_secure_entropy().expect("keypair");
        let summary = RejectedAttemptSummary::new("run", "attempt", "predicate_reject", [7u8; 32]);
        let sig =
            predicate_runner::sign_rejected_attempt_summary(&keypair, &summary).expect("sign");

        let mut pinned = PinnedSystemPubkeys::new();
        pinned.insert(SystemEpoch::new(1), keypair.public_key());
        assert!(verify_system_signature(
            &sig,
            &CanonicalMessage::RejectedAttemptSummary(summary),
            SystemEpoch::new(1),
            &pinned
        ));
    }

    #[test]
    fn terminal_scope_rotation_signing_round_trip() {
        let old = Ed25519Keypair::generate_with_secure_entropy().expect("old");
        let new = Ed25519Keypair::generate_with_secure_entropy().expect("new");
        let proof = EpochRotationProof::new(
            SystemEpoch::new(1),
            SystemEpoch::new(2),
            old.public_key(),
            new.public_key(),
            1_776_000_000,
        );
        let old_sig =
            terminal_summary_emitter::sign_epoch_rotation_proof(&old, &proof).expect("old sign");
        let new_sig =
            terminal_summary_emitter::sign_epoch_rotation_proof(&new, &proof).expect("new sign");

        let mut pinned = PinnedSystemPubkeys::new();
        pinned.insert(SystemEpoch::new(1), old.public_key());
        pinned.insert(SystemEpoch::new(2), new.public_key());
        assert!(verify_epoch_rotation_proof(
            &proof, &old_sig, &new_sig, &pinned
        ));
    }
}

```

---

# Round-1 merged verdict (closure check reference)

# CO1.1.4-pre1 Round-1 Dual External Audit — Merged Verdict

**Date**: 2026-04-28
**Target**: spec v1 (`handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md`) + impl (`src/state/typed_tx.rs` + supporting derive additions in `economy/money.rs`, `cas/schema.rs`, `system_keypair.rs`) — committed `227de72`.
**Auditors**: Codex (gpt-5-codex; 199K tokens) + Gemini 2.5 Pro (113K tokens).
**Conservative rule** (memory `feedback_dual_audit_conflict`): VETO > CHALLENGE > PASS.

---

## § 1 Verdicts

| Auditor | Verdict | Conviction | Top must-fix count |
|---|---|---|---|
| **Codex** | **CHALLENGE** | High | 3 (sched-merged below; many sub-items per § Q-A..Q-J) |
| **Gemini** | **CHALLENGE** | High | 3 |
| **Conservative merged** | **CHALLENGE** | High | — |

**Pre-implementation gate**: NOT cleared. v1.1 patch round required before CO1.7-impl A2 unblocks.

---

## § 2 Convergent must-fix items (both auditors flagged)

| ID | Item | Codex frame | Gemini frame |
|---|---|---|---|
| **C-1** | **Agent-signature domain separation** | Q-E: `AgentSignature` has no signed-payload struct + no domain prefix; comments imply an "exclude signature" digest without a signing payload — impossible | Q7: type-level distinction insufficient; agent-signed payloads MUST add unique domain separators (`v4.agent_sig.work` vs `v4.agent_sig.verify` vs `v4.agent_sig.challenge`) to prevent type confusion attacks |
| **C-2** | **Lock golden fixture hex + expand test coverage** | Q-J: golden tests only assert length=64 + self-stability; don't lock hex; `TypedTx::TerminalSummary` excluded from round-trip / kind / golden tests | Q9: "phase 1 record-only" insufficient for an ABI-defining atom; must hardcode SHA-256 hex; add cross-variant non-collision + BTreeMap permutation independence + zero-value default tests |
| **C-3** | **Schema parity (TerminalSummary 8-field + claim-id newtype + signing-payload structs)** | Q-C must-fix-now: STATE § 1.5 has 8 fields; shipped 3-field placeholder is exactly the ABI being frozen; future additions are decode-breaking. Move tx schema OUT of `system_keypair.rs` into `state::typed_tx`; system_keypair signs opaque digest. Q-B: `claim_id: TxId` should be `claim_id: ClaimId` newtype (STATE speaks in ClaimId; finalization order is by claim_id). | Q6: FinalizeRewardTx system_signature might be REDUNDANT given LedgerEntrySigningPayload signs envelope; spec must explicitly distinguish "agent-to-runtime sign" from "runtime-to-ledger sign" or DROP the field; claim_id should be typed ClaimId not reused TxId |

---

## § 3 Codex-only must-fix items

| ID | Item | Codex citation |
|---|---|---|
| **CX-1** | **TransitionError taxonomy incomplete** | Q-G: missing `SignatureInvalid`, `StakeInsufficient`, predicate failures, target-not-found/not-verifiable, challenge-window-closed, counterexample-insufficient, tool errors, parent-not-accepted etc. STATE § 3 pseudocode raises these; code v1 lists only 10 stub variants. `NotYetImplemented` is acceptable as transition-stage sentinel but not as production error. |
| **CX-2** | **Bincode spec/code parity** | Q-D: STATE § 2.5 wording is wrong vs actual codec — bincode-2 serde encodes enum variant indices as **u32 BE** (not u8 / not `#[repr(u8)]`-controlled), and lengths as **u64 BE** (`usize` materialization). `#[repr(u8)]` does not control serde wire format. `#[serde(transparent)]` newtypes ARE wire-identical to inner values (confirmed). **Sub-implication**: STATE § 2.5 spec must be patched OR the codec settings must change to match (u8 variant indices via custom serde adapter). |
| **CX-3** | **TaskId vs TxId QState index mismatch** | Q-J: typed_tx.rs uses `TaskId` for task references, but current `QState.economic_state_t.{task_markets_t, escrows_t, stakes_t}` are keyed by `TxId` (q_state.rs:201, 161, 182). Future CO P2.x atoms WILL hit this divergence. Must either retrofit QState now or document migration plan. |

## § 4 Gemini-only must-fix items

| ID | Item | Gemini citation |
|---|---|---|
| **GM-1** | **Art 0.2 cold-replay constitutional commitment** | Q4: payload data lives in CAS (referenced via `tx_payload_cid: Cid`); CO1.7 spec § 0 already deferred CAS index persistence to **CO1.4-extra**. Until that ships, cold-restart loses payload data → tape is non-canonical → Art 0.2 violation. v1.1 of CO1.1.4-pre1 must explicitly gate PASS on a concrete commitment to ship CO1.4-extra alongside or before CO1.7-impl A4 (replay_full_transition). Strategic risk: shipping a constitutionally-non-compliant tape layer, even temporarily, is too high. |

---

## § 5 PASS items (both auditors)

- **Codex Q-A (D-1 TxStatus elision)**: PASS with patch note. STATE `step_transition` does NOT read `tx.status`; status is derived from accepted-tx history + ClaimsIndex + per-agent state. Constitutionally sound. Migration path documented.
- **Codex Q-H (HasSubmitter correctness)**: PASS. Work/Verify/Challenge delegate to actual submitter; Reuse returns None correctly (creator is royalty recipient, not submitter). Outer delegation correct.
- **Codex Q-I (atom scope creep)**: PASS with caveat (TerminalSummary placement — but that caveat already covered by C-3).
- **Gemini Q1-Q3, Q5, Q8**: implicit PASS (not in must-fix list). Constitutional alignment, Inv 3 interaction, v4/v4.1 boundary preservation, reconstructibility-via-Q-derivation, forward sustainability all OK.

---

## § 6 v1.1 patch plan

**Estimated scope**: ~300-500 LoC code + 60-100 LoC spec patches + 1 new module (`agent_signing_payloads`) + golden fixture rotation. ~0.5-1 day. Round-2 audit cost: ~$10-20.

| Patch | Maps to | Touches |
|---|---|---|
| **P1**: introduce `WorkSigningPayload` / `VerifySigningPayload` / `ChallengeSigningPayload` structs (subset of each tx, EXCLUDES signature; canonical_digest with domain prefix `b"v4.agent_sig.work.v1"` / etc.); add `verify_agent_signature(sig, payload, agent_pubkey)` API surface | C-1 | typed_tx.rs +~100 LoC; spec § 7 + new § 7.1 |
| **P2**: add `ClaimId(pub TxId)` newtype (spec § 4 + typed_tx.rs); update FinalizeRewardTx.claim_id; specify in spec § 4 that {task_id, solver, reward, royalty edges} are Q-derived at replay (NOT trusted from wire) | C-3 partial (claim-id) | typed_tx.rs + spec § 4 |
| **P3**: migrate `TerminalSummaryTx` from system_keypair.rs (3-field) → state::typed_tx (8-field per STATE § 1.5: tx_id / task_id / run_id / run_outcome / total_attempts / failure_class_histogram / last_logical_t / system_signature); drop the `system_keypair::TerminalSummaryTx`; system_keypair::sign_terminal_summary_tx accepts opaque digest | C-3 main | typed_tx.rs + system_keypair.rs (deletion + reroute) |
| **P4**: complete `TransitionError` taxonomy with all variants invoked in STATE § 3 pseudocode (~12-15 additional variants); keep `NotYetImplemented` as explicit stub sentinel | CX-1 | typed_tx.rs |
| **P5**: lock golden fixture hex (SHA-256 of canonical bytes for each TypedTx variant fixture); add cross-variant non-collision test; add BTreeMap permutation independence test; add zero-value default round-trip; include `TypedTx::TerminalSummary` in all test classes | C-2 | typed_tx.rs tests |
| **P6**: spec patch — STATE § 2.5 wording drift (variant index = u32 BE, lengths = u64 BE; `#[repr(u8)]` does NOT control wire format) — either fix the spec OR add a custom serde adapter forcing u8 discriminants. **Decision required**: cheap (spec patch) vs expensive (codec change forces re-encode of all existing fixtures). Given v1 is brand-new, recommend spec patch + accept u32/u64 sizing. | CX-2 | spec § 2.5 OR new serde adapter |
| **P7**: spec § 9 D-3 → resolved (no longer a divergence; full schema migrated). Remove D-3 row. | C-3 followup | spec § 9 |
| **P8**: spec § 5 add explicit Q-derived note for FinalizeRewardTx fields {task_id, solver, reward, royalty}; spec § 4 commit to making LedgerEntrySigningPayload the SOLE signing point for FinalizeRewardTx (drop FinalizeRewardTx.system_signature OR clarify dual-sign rationale) | C-3 + GM-2 | spec § 4 + § 5 |
| **P9**: spec patch — § 0 add explicit "Art 0.2 cold-replay gate": v1.1 PASS contingent on CO1.4-extra commitment (explicit cross-atom dependency; CO1.7-impl A4 must NOT ship before CO1.4-extra) | GM-1 | spec § 0 |
| **P10**: spec patch — § 9 add D-4 TaskId-vs-TxId-keyed-QState forward-migration plan (CO P2.1 TaskMarket atom owns the QState retrofit; v1.1 documents but does not perform) | CX-3 | spec § 9 + new appendix |

---

## § 7 Round structure forward

| Round | Codex | Gemini | Conservative | Action |
|---|---|---|---|---|
| 1 | CHALLENGE (high) | CHALLENGE (high) | **CHALLENGE** | v1.1 patch round (P1-P10 above) |
| 2 | ⏳ | ⏳ | TBD | re-audit on v1.1; expected PASS or 1-issue CHALLENGE |
| 3+ | … | … | … | iterate to PASS/PASS |

**Pre-implementation gate** (for CO1.7-impl A2-A4): CO1.1.4-pre1 must reach `PASS/PASS` before A2 starts.

---

## § 8 Cumulative cost (this round)

| Auditor | Tokens | Estimated $ |
|---|---|---|
| Codex r1 | 199,200 | ~$5-10 |
| Gemini r1 | 113,295 | ~$3-5 |
| **Round 1 total** | **312,495** | **~$8-15** |

Tracks lower than CO1.7 spec round-1 ($7-12) because spec is shorter. Cumulative project audit spend: ~$143-217 / $890 mid-budget (~16-24%).

---

## § 9 Sedimented lessons (this round)

1. **Single-source-of-truth for tx schemas matters**: TerminalSummaryTx living in `system_keypair.rs` (signer module) leaked into a "frozen" location and made the ABI atom imports dependent on the placeholder. Sedimented: per-typed-tx struct should live in `state::typed_tx`; signers consume opaque digests.

2. **Spec § 2.5 wording drift**: STATE_TRANSITION_SPEC § 2.5 claimed bincode would emit `#[repr(u8)]` discriminants, which is FALSE — that attribute does not control serde wire format. Sedimented: when freezing a wire format, the spec must include the actual codec library version + verified-by-test byte layout, not the LANGUAGE-level repr.

3. **"Record-only" golden fixtures are not golden**: a self-comparison ("two encodes match") is round-trip stability, not ABI freezing. ABI freeze requires hardcoded hex against which any future encode is compared. Sedimented: every ABI-freeze atom MUST lock SHA-256 hex in v1; "phase 1 record-only" deferral is a CHALLENGE smell.

4. **Domain separation is non-negotiable when a 64-byte signature is reused for distinct semantic roles** (agent vs system; work vs verify vs challenge). Type-level distinction at the API surface is necessary but not sufficient; the canonical_digest pre-image must encode the role. Sedimented: when introducing a new signature primitive, the canonical_digest spec must include a stable role-prefix byte string (`b"v4.<actor>.<purpose>.v1"`).

5. **Cold-replay → Art 0.2 commitment is a real cross-atom gate**: declaring "CO1.4-extra is a separate atom" is not enough; the deferred dependency creates a window where the tape is non-canonical. Sedimented: v1.1 must explicitly state the cross-atom ordering constraint (CO1.4-extra MUST ship before CO1.7-impl A4 replay_full_transition) and PASS is contingent on that ordering being honored.

— ArchitectAI synthesis, 2026-04-28; Round-1 closure 2026-04-28; v1.1 patch round opens.


---

# XREF: STATE_TRANSITION_SPEC v1.4 (frozen)

# State Transition Specification v1.4

> **Date**: 2026-04-27 (v1.4 closes 4 cosmetic Codex round-3 PARTIAL items)
>
> **Patch v1.3 → v1.4 changes** (per Codex round-3 re-audit at `handover/audits/CODEX_SPEC_V13_REAUDIT_2026-04-27.md`):
> - **§ 5.3 grep list cleanup** (Q1.1): patch log no longer claims `TaskMarketPublishTx` is RETIRED; it's a NEW transition deferred to CO P2.1. Conformance test grep includes only actually-retired symbols.
> - **§ 3.2 challenge_transition + § 3.4 finalize_reward** (Q2.4): both now invoke `ChallengeWindow::is_open(now)` method (defined § 5.2.5 NEW); pseudocode no longer hand-codes the inequality.
> - **§ 5.2.1 sequencer tie-break** (Q6): `next_logical_t()` is atomic; assigned `logical_t` IS the canonical tie-break for concurrent submissions; explicit prose added.
> - **§ 2.5 + § 7 fixture corpus defer-ack** (Q5/NEW-5): canonical serialization RULE frozen v1.4; full golden fixture corpus + differential fuzz seed lands in CO1.1.4-pre1 + CO1.7 atoms (not v1.x spec scope).
>
> **Patch v1.2 → v1.3 changes** (per Codex re-audit verdict CHALLENGE/NO-GO at `handover/audits/CODEX_SPEC_V12_REAUDIT_2026-04-27.md`):
> - **§ 3.6 task_expire_transition refactored** — removed runtime side effects from pure transition; runtime constructs+signs `TaskExpireTx` BEFORE pure entry; restores § 2 + § 3 pure-boundary discipline (Codex new-issue #1 fix)
> - **§ 3.6 stage 3 expiry guard broadened** — refund only if NO claim of ANY status exists for task; prevents race with Pending/Provisional claims (Codex new-issue #2 fix)
> - **§ 3.6.5 agent_implicit_init refactored** — introduce `HasSubmitter` trait with per-tx `submitter_id()` methods; resolves WorkTx vs VerifyTx vs ChallengeTx vs ReuseTx field-name divergence; `ReuseTx` returns None (intentional; reuse facts have no submitter) (Codex new-issue #3 fix)
> - **I-FINALIZE-BATCH-ORDER + § 5.2.3 + test all use `claim_id`** consistently (was 3-way contradiction with `target_work_tx`) (Codex new-issue #4 fix)
> - **I-CHALLENGE-WINDOW-EDGE binding** — `is_open(now)` defined as `now < opens_at + duration_ticks`; both challenge_transition AND finalize_reward MUST use same `is_open()` rule (Codex Q2.4 fix)
> - **§ 5.1 false-challenge prose cleanup** — removed "User can override any default" generality where 11.1 is in fact NOT overridable in v4
> - **§ 6 (NEW) Legacy economic tx disposition** — InvestTx / TaskMarketPublishTx / MarketCreateTx / MarketResolveTx explicitly retired in CO1.1.4 atom (Codex Q1.1 NOT-CLOSED fix)
>
> **Patch v1.1 → v1.2 changes** (per Codex+Gemini CO1.SPEC.0.5 dual audit, 2026-04-27):
> - **§ 2 hidden-input table EXPANDED** — added HAYEK_BOUNTY, BOUNTY_LP, Boltzmann params, BOLTZMANN_SEED, async ordering boundary, WAL/git commit boundary, full HashMap scope, f64 royalty math
> - **§ 2.5 (NEW) canonical serialization** — defines byte-level format for all signed tx + state roots
> - **§ 3.4 finalize_reward** — added stage 3a (solver stake unlock + return); royalty math now uses integer floor rule
> - **§ 3.6 (NEW) task_expire_transition** — handles unsolved task bounty refund
> - **§ 3.7 (NEW) agent_register implicit-init** — first appearance in L4 = default reputation 0
> - **§ 5.1 false-challenge resolution** — fixed to "v4 default 0, NOT configurable" (resolves prose-vs-pseudocode contradiction)
> - **§ 4 invariants** — 22 → 27 (added I-STAKE-RETURN, I-BOUNTY-REFUND, I-FINALIZE-BATCH-ORDER, I-CHALLENGE-WINDOW-EDGE, I-AGENT-INIT)
> - **§ 6.1 (NEW) concurrency rule** — L4 sequencer per (runtime_repo, run_id); deterministic ordering key
> - **§ 8 count fix** — "16 invariants" → "27 invariants"
>
> **Patch v1 → v1.1 changes** (per SPEC_WALKTHROUGH gap fixes, 2026-04-27):
> - § 3.2 (challenge_transition) stage 4e ADDED: verifier_bond release policy (default = return to verifier; configurable)
> - § 3.3 (reuse_transition) stage 3 AMENDED: edge weight bounded by `MAX_REUSE_ROYALTY_FRACTION` config (default = 0.10)
> - § 3.2 (challenge_transition) stage 4d AMENDED: false-challenge reputation penalty (v1.3 update: **fixed to 0 in v4; NOT configurable**; previous v1.1 patch log saying "configurable" is OBSOLETE)
> - § 3.1 (verify_transition) note ADDED: quorum-aggregation rule placeholder (default = 1; configurable)
> - § 4 invariants ADDED: I-VBOND-RELEASE / I-ROYALTY-CAP
> - § 11 (Found Inconsistencies) — promoted from SPEC_WALKTHROUGH § 11
>
> All 4 walk-through gaps now have either (a) machine-checkable default applied, or (b) explicit deferral with target atom.
>
> **Purpose**: D-VETO-1 binding form. Defines `step_transition: (Q_t, tx_i) → (Q_{t+1}, signals_t)` with typed schemas, deterministic pseudocode, named invariants, conformance test list. Gates CO1.1.4/CO1.1.5 bus.rs/kernel.rs split (per Plan v3.2 atom CO1.SPEC.0).
>
> **Authority**: Constitution Art. 0–0.4 + white paper architecture § 3-7 + economic § 2/§ 6/§ 18-21. Where this spec disagrees with white paper, **white paper wins** and this spec must be amended.
>
> **Audit**: Codex CO P0.7 T+S review (2026-04-27) demanded binding spec form before refactor. This document is the response.

---

## § 0 Scope

**In scope**:
- The single-step state transition function `step_transition` for object-level work_tx
- Typed `QState`, `WorkTx`, `VerifyTx`, `ChallengeTx`, `RejectedAttemptSummary`, `TerminalSummaryTx` schemas
- Hidden-input classification: which existing `bus.rs`/`kernel.rs` inputs are `Q_t`, which are `tx_i`, which are illegal side effects
- Named invariants enforceable mechanically
- Conformance test list generated from the spec

**Out of scope** (handled separately):
- `MetaTx` schema for runtime meta-transitions — defined as **stub only** here; full schema deferred to v4.1 per D-VETO-4 = B (defer, not abandon)
- AttributionEngine DAG construction algorithm — deferred to CO2.4.0 spike (Inv 8 design)
- Full predicate visibility air-gap proof — deferred to CO P1.5 (Goodhart shield design)

---

## § 1 Typed Schemas

### 1.1 QState (white paper § 4 + economic § 2 amendment, 9 fields)

```rust
pub struct QState {
    /// Agent swarm sub-state: tape head per agent, per-agent reputation snapshots, etc.
    /// MUST be reconstructible from L4 transition ledger replay.
    pub q_t: AgentSwarmState,

    /// Current ChainTape head pointer = git commit SHA in Path B substrate.
    pub head_t: NodeId,

    /// Materialized state Merkle root (git tree root in Path B).
    pub state_root_t: Hash,

    /// Agent-visible projection of tape filtered by per-agent visibility policy
    /// (Inv 10 Goodhart shield). Derived from L1 PredicateRegistry visibility tags.
    pub tape_view_t: AgentVisibleProjection,

    /// L4 Transition Ledger root (Merkle root of all accepted tx so far).
    pub ledger_root_t: Hash,

    /// L1 Predicate Registry root.
    pub predicate_registry_root_t: Hash,

    /// L2 Tool Registry root.
    pub tool_registry_root_t: Hash,

    /// Economic state (economic § 2 amendment, 9 sub-fields).
    pub economic_state_t: EconomicState,

    /// Global budget snapshot: cost ceiling, wall clock, compute cap.
    pub budget_state_t: BudgetSnapshot,
}

pub struct AgentSwarmState {
    pub agents: BTreeMap<AgentId, PerAgentState>,
    pub current_round: u64,
}

pub struct PerAgentState {
    pub reputation_snapshot: Reputation,
    pub last_accepted_tx: Option<TxId>,
    pub retry_counter_for_current_task: u32,  // resets on accept; persists across rejections
}

pub struct EconomicState {
    pub balances_t:       BalancesIndex,
    pub escrows_t:        EscrowsIndex,
    pub stakes_t:         StakesIndex,
    pub claims_t:         ClaimsIndex,
    pub reputations_t:    ReputationsIndex,
    pub task_markets_t:   TaskMarketsIndex,
    pub royalty_graph_t:  RoyaltyGraph,
    pub challenge_cases_t: ChallengeCasesIndex,
    pub price_index_t:    PriceIndex,
}
```

**BTreeMap, not HashMap, everywhere**: deterministic iteration order for replay byte-identity (Codex flagged kernel.rs:187-204 HashMap nondeterminism).

### 1.2 WorkTx (12 fields per WP § 5.L4)

```rust
pub struct WorkTx {
    pub tx_id: TxId,                              //  1
    pub task_id: TaskId,                          //  2  links to TaskMarket entry
    pub parent_state_root: Hash,                  //  3  must equal Q_t.state_root_t at submission
    pub agent_id: AgentId,                        //  4
    pub read_set: BTreeSet<ReadKey>,              //  5  agent MUST declare read deps (DAG attribution)
    pub write_set: BTreeSet<WriteKey>,            //  6  agent MUST declare write targets
    pub proposal_cid: Cid,                        //  7  L3 CAS handle to payload (not raw payload)
    pub predicate_results: PredicateResultsBundle,//  8  filled BY runner, not by agent
    pub stake: StakeMicroCoin,                    //  9  YES_E stake, i64 micro-coin units
    pub signature: AgentSignature,                // 10
    pub timestamp_logical: u64,                   // 11  monotonic counter from runtime, NOT wall clock
    pub status: TxStatus,                         // 12  Pending | Accepted | Rejected(class) | Finalized
}

pub enum TxStatus {
    Pending,
    Accepted,
    Rejected(RejectionClass),
    FinalizedReward(MicroCoin),
    FinalizedSlash(SlashEvidenceCid),
}

pub struct PredicateResultsBundle {
    pub acceptance: BTreeMap<PredicateId, BoolWithProof>,
    pub settlement: BTreeMap<PredicateId, BoolWithProof>,
    pub safety_class: SafetyOrCreation,  // determines fail-closed vs fail-open-with-signal
}
```

### 1.3 VerifyTx, ChallengeTx, ReuseTx (economic § 13)

```rust
pub struct VerifyTx {
    pub tx_id: TxId,
    pub target_work_tx: TxId,         // the work_tx being verified
    pub verifier_agent: AgentId,
    pub bond: StakeMicroCoin,         // verifier reputation/bond stake
    pub verdict: VerifyVerdict,       // Confirm | Doubt
    pub signature: AgentSignature,
    pub timestamp_logical: u64,
}

pub struct ChallengeTx {
    pub tx_id: TxId,
    pub target_work_tx: TxId,
    pub challenger_agent: AgentId,
    pub stake: StakeMicroCoin,        // NO_E stake, i64 micro-coin
    pub counterexample_cid: Cid,      // L3 CAS handle to counterexample
    pub signature: AgentSignature,
    pub timestamp_logical: u64,
}

pub struct ReuseTx {
    pub tx_id: TxId,
    pub reusing_work_tx: TxId,        // the work_tx that triggered the reuse
    pub reused_tool_id: ToolId,       // L2 Tool Registry handle
    pub reused_tool_creator: AgentId, // royalty recipient
    pub timestamp_logical: u64,
}
```

### 1.4 RejectedAttemptSummary (D-VETO-6 system-stamped, NOT agent self-report)

```rust
pub struct RejectedAttemptSummary {
    pub failed_attempts_since_last_accept: u32,           // bounded, capped at u32::MAX
    pub failure_class_histogram: BTreeMap<RejectionClass, u32>,  // counts only, no payloads
    pub first_failure_logical_t: Option<u64>,             // for time-to-first-fail signal
    pub last_failure_logical_t: Option<u64>,              // for recency signal
    // NO raw error strings, NO agent payload contents, NO predicate internal traces
}

pub enum RejectionClass {
    AcceptancePredicateFail(PredicateId),     // public predicates only; private predicates → Opaque
    SettlementPredicateFail(PredicateId),
    StakeInsufficient,
    SignatureInvalid,
    StaleParentRoot,                          // Q_t advanced; agent's view stale
    Opaque,                                   // private predicate failure; classification withheld
    BudgetExceeded,
}
```

`RejectedAttemptSummary` is stamped **by the white-box predicate runner** onto the next accepted `WorkTx`. Trust boundary: the runner generates this summary; the agent does NOT self-report. Verified at conformance test level.

### 1.5 TerminalSummaryTx (no-accept run handler)

```rust
pub struct TerminalSummaryTx {
    pub tx_id: TxId,
    pub task_id: TaskId,
    pub run_id: RunId,
    pub run_outcome: RunOutcome,           // OmegaAccepted | MaxTxExhausted | WallClockCap | ComputeCap | ErrorHalt
    pub total_attempts: u32,
    pub failure_class_histogram: BTreeMap<RejectionClass, u32>,
    pub last_logical_t: u64,
    pub system_signature: SystemSignature,  // signed by runtime keypair, not by any agent
}
```

If a run terminates without any accepted work_tx, the runtime emits exactly one `TerminalSummaryTx` to L4. This preserves L6 reconstructibility: error class signal is derivable from tape even if no work_tx ever passed.

### 1.6 MetaTx (stub for v4.1; v4 only emits `MetaProposalDraft` to L3 CAS, not L4)

```rust
pub struct MetaTx {
    pub tx_id: TxId,
    pub parent_architecture_root: Hash,
    pub proposed_predicate_patches: Vec<PredicatePatch>,
    pub proposed_tool_patches:      Vec<ToolPatch>,
    pub log_evidence_cids:           Vec<Cid>,
    pub reversibility_plan_cid:      Cid,
    pub constitution_check:          ConstitutionCheckProof,
    pub judge_signatures:            Vec<JudgeSignature>,
    pub human_signature_required:    bool,
    pub human_signature:             Option<HumanSignature>,
}
```

**v4 status**: MetaTx schema reserved; runtime ArchitectAI/JudgeAI **NOT implemented**. v4 produces `MetaProposalDraft` (a CAS object) only, written when ArchitectAI proposes architecture amendments via the cp workflow. v4.1 implements the runtime actor + L4 acceptance.

This is the D-VETO-4 = B (defer, not abandon) implementation.

---

## § 2 Hidden-Input Classification (Codex § C demanded)

The current `src/bus.rs` and `src/kernel.rs` mix four categories of inputs. The spec must classify each:

| Input | Current source | T+S classification | New home in step_transition |
|---|---|---|---|
| `created_at` (wall clock seconds) | `bus.rs:264-268` `SystemTime::now()` | **ILLEGAL hidden side effect** | retire; replace with `timestamp_logical: u64` from runtime monotonic counter |
| `completion_tokens: 0` literal | `bus.rs:268` | **ILLEGAL hidden zero** | kill in CO1.1.4-pre1; read real value from LLM `usage.completion_tokens` |
| `TAPE_ECONOMY_V2` env var | `bus.rs:298, 345` | **`Q_t.budget_state_t.feature_flags`** | promote to typed field; tx must reference flag value at parent_state_root |
| `FOUNDER_GRANT_GAMMA` env var | `bus.rs:307` | **`Q_t.economic_state_t.task_markets_t.config.founder_grant_gamma`** | promote to typed field; bound at task creation, not env at runtime |
| `self.config.system_lp_amount` | `bus.rs:340` | **`Q_t.economic_state_t.task_markets_t.config.system_lp_amount`** | promote |
| `self.clock` counter | `bus.rs:42` | **`Q_t.q_t.current_round` derived** | derive from L4 ledger length; not separately tracked |
| `self.tx_count` | `bus.rs:42` | **`Q_t.q_t.current_round` derived** | derive |
| `self.generation` | `bus.rs:42` | **`Q_t.q_t.generation` typed field** | promote |
| `self.graveyard: HashMap<String, Vec<String>>` | `bus.rs:48` | **ILLEGAL sidecar** (Art. 0.2 explicitly anti-patterned) | retire; replace with `RejectedAttemptSummary` stamped on next accepted tx + `TerminalSummaryTx` |
| Tool list iteration order | `bus.rs:312-319` Vec | **`Q_t.tool_registry_root_t` derived** | runner queries L2 in deterministic order |
| Wallet "magic search" | `bus.rs:312-319` `manifest() == "wallet"` | **EXPLICIT capability lookup** | runner queries L2 by `Capability::EconomicWallet` tag, not by string match |
| `HAYEK_BOUNTY` env var (v1.2 added per Codex Q3) | `src/bus.rs:141-150` (init), `src/bus.rs:349-360` (settle) | **`Q_t.economic_state_t.task_markets_t.config.hayek_bounty_enabled`** | promote to typed task config; bound at task creation |
| `BOUNTY_LP` env var (v1.2 added per Codex Q3) | `src/bus.rs:141-150`, `src/bus.rs:349-360` | **`Q_t.economic_state_t.task_markets_t.config.bounty_lp_seed: MicroCoin`** | promote to typed task config |
| `BOLTZMANN_TEMP` / `FRONTIER_CAP` / `DEPTH_WEIGHT` / `PRICE_GATE_ALPHA` / `BOLTZMANN_SEED` env (v1.2 added per Codex Q3) | `src/sdk/actor.rs:22-39` (params), `experiments/.../bin/evaluator.rs:693-697` (seed) | **OFF-TAPE proposal-generation only**; NOT part of `Q_t`; routing seed visible in `proposal_cid` payload (CAS); transition pseudocode does NOT consume these | classified as "agent-side proposal entropy"; the SAMPLED outcome is on tape via proposal_cid; the sampling RNG state is NOT |
| HashMap iteration order broadly (v1.2 added per Codex Q3) | `src/kernel.rs:19-21` (markets), `src/kernel.rs:165-204` (resolve + ticker), any new code | **BANNED in any module reachable from `step_transition` call tree** | runtime test grep extends to ALL `src/` files reachable transitively; not just modules containing "q_state" or "transition" |
| Async tokio task completion ordering (v1.2 added per Codex Q3 + Q6) | `experiments/.../bin/evaluator.rs:192-193` (#[tokio::main]) | **L4 sequencer (§ 6.1) defines deterministic ordering key (logical_t, tx_id)**; async completion order is NOT used | sequencer enforces serialization point per (runtime_repo, run_id); see § 6.1 |
| WAL / git commit filesystem effects (v1.2 added per Codex Q3) | `src/bus.rs:279-282` (WAL Node), `src/bus.rs:319-327` (WAL event) | **explicit boundary: pure `step_transition(q, tx)` returns `(q', signals)` PURELY; runtime layer commits side effects to WAL/git AFTER pure result** | step_transition is pure function of (q, tx); commit is runtime concern; § 6.1 specifies commit point |
| `f64` arithmetic in monetary / royalty math (v1.2 added per Codex Q3 + Q10) | `src/prediction_market.rs:21-27,87-133` (reserves, trades) + spec § 3.3 royalty `reward * edge.weight` | **i64 MicroCoin only; royalty rounding rule = integer floor (`micro_reward * weight_micro / 1_000_000`)** | promote `prediction_market.rs` to MicroCoin; spec § 3.3 stage 3b adds explicit rounding |
| Future tokio::spawn introduction (v1.2 hypothesis per Codex Q3) | (none currently) | **BANNED in `src/transition/*` and `src/economy/*` call trees** | cargo-deny rule + transitive grep |

After this classification, every step_transition input is either part of `Q_t`, part of `tx_i`, or part of the runtime config bound at genesis (which is itself in `Q_t`).

**Conformance test for § 2** (`tests/no_hidden_inputs.rs`):
- grep src/ for `SystemTime::now()` → must return 0 hits in non-runtime-bootstrap code
- grep src/ for `std::env::var(` → must return 0 hits in step_transition path **AND** in any module transitively reachable from `transition::*`, `economy::*`, `top_white::predicates::*` (v1.2 expanded scope per Codex Q3)
- grep src/ for `HashMap` → must return 0 hits in **ALL modules reachable from `step_transition` call tree** (v1.2 expanded scope; was: only "q_state" or "transition" modules; new scope: full transitive reach)
- assert all monetary fields are typed `MicroCoin` (a newtype around `i64`), no `f64` — **including `src/prediction_market.rs` and any RSP module**
- grep src/ for `tokio::spawn` → must return 0 hits in `src/{transition,economy,top_white::predicates}/*` (v1.2 added per Codex Q3 hypothesis)

## § 2.5 Canonical Serialization (v1.2 NEW per Codex Q5)

> **Required because**: `tx.canonical_digest()` is called in spec § 3 stages 2 of WorkTx / VerifyTx / ChallengeTx, but byte-level format is undefined. STEP_B branch A vs branch B may pick different serialization (JSON sorted keys vs bincode vs Rust derive order) → cross-branch signature verification fails. Mandatory canonical format closes this.

**Format**: **bincode v2** (`bincode::serde`) with the following constraints:
- **Big-endian byte order** for all multi-byte integers (network order; deterministic across platforms)
- **`BTreeMap` keys serialized in lexicographic byte order** (this is bincode default; verified by test)
- **Strings serialized as UTF-8 with explicit length prefix u32-BE**
- **Optional fields: `0x00` prefix for `None`, `0x01` + value for `Some`**
- **Enum discriminant: u8 (variant index in declaration order)**
- **No padding bytes; no implicit alignment**

**Application**:
```rust
pub fn canonical_digest<T: Serialize>(value: &T) -> [u8; 32] {
    let bytes = bincode::serde::encode_to_vec(value, bincode_canonical_config()).expect("serialize");
    sha256(&bytes)
}

fn bincode_canonical_config() -> bincode::config::Configuration {
    bincode::config::standard()
        .with_big_endian()
        .with_fixed_int_encoding()    // no varint; fixed-width for determinism
}
```

**Conformance**: `tests/canonical_serialization.rs` MUST verify:
- 1 golden tx fixture per tx type (WorkTx / VerifyTx / ChallengeTx / ReuseTx / TerminalSummaryTx); each has known input → known SHA-256 output
- Round-trip: `decode(encode(x)) == x` byte-identical for 100 random inputs
- Stability: 2 independent runs on same input → same bytes

**STEP_B implication**: branches A and B both use this exact `bincode_canonical_config`; signature verification works cross-branch by construction.

**Out of scope for v1.x spec** (deferred per Codex Q5/NEW-5 round-3 PARTIAL acknowledgment): full golden fixture corpus + differential fuzzing seed + complete runner ABI for QState/SignalBundle/TransitionError. v1.4 freezes the SERIALIZATION RULE (bincode v2 big-endian + BTreeMap lex); fixtures + ABI land in **CO1.1.4-pre1** (canonical fixture corpus) + **CO1.7** (full ABI surface). This is an **explicit deferral** — not unresolved spec ambiguity. STEP_B branch A and branch B both implement the SAME bincode rule; per-tx digest matching is mechanical from v1.4. Full corpus generation is a downstream code task, not spec scope.

---

## § 3 step_transition (Deterministic Pseudocode)

```rust
/// Pure function. Same (Q_t, tx_i) → byte-identical (Q_{t+1}, signals_t).
/// No I/O. No env reads. No clock reads. No randomness without seed in tx_i.
pub fn step_transition(
    q: &QState,
    tx: &WorkTx,
    registry: &PredicateRegistry,
    tool_registry: &ToolRegistry,
) -> Result<(QState, SignalBundle), TransitionError> {

    // STAGE 1: parent_state_root match (stale view rejection)
    if tx.parent_state_root != q.state_root_t {
        return Err(TransitionError::StaleParent {
            expected: q.state_root_t,
            got:      tx.parent_state_root,
        });
        // NB: rejection here does NOT change Q_t; runner stamps RejectedAttemptSummary
        // onto the NEXT accepted tx (or onto TerminalSummaryTx if run ends without accept)
    }

    // STAGE 2: signature verification
    if !verify_signature(&tx.signature, tx.canonical_digest()) {
        return Err(TransitionError::SignatureInvalid);
    }

    // STAGE 3: stake availability (Inv 5 — YES_E event-bound)
    let agent_balance = q.economic_state_t.balances_t.get(&tx.agent_id);
    if agent_balance < tx.stake {
        return Err(TransitionError::StakeInsufficient { available: agent_balance, required: tx.stake });
    }

    // STAGE 4: predicate gate (Inv 6 — predicate-gated transition)
    let acceptance_results = registry.run_acceptance(tx, q)?;
    let safety_class = registry.classify(tx);
    match (safety_class, acceptance_results.all_passed()) {
        (SafetyOrCreation::Safety, false) => {
            return Err(TransitionError::AcceptancePredicateFailed(acceptance_results));
            // fail-closed for Safety (WP § 7.2)
        }
        (SafetyOrCreation::Creation, false) => {
            // fail-open-with-signal: still reject, but emit informational signal (no Q_t change)
            return Err(TransitionError::AcceptancePredicateFailed(acceptance_results));
        }
        _ => {}  // passed; continue
    }

    // STAGE 5: provisional reward issue (Inv 7 — provisional then final)
    let claim = ClaimId::derive(tx.tx_id);
    let provisional_reward = SettlementEngine::issue_provisional(
        claim,
        &q.economic_state_t.escrows_t,
        tx.task_id,
    )?;

    // STAGE 6: state transition apply (deterministic)
    let mut q_next = q.clone();
    q_next.economic_state_t.claims_t.insert(claim, provisional_reward);
    q_next.economic_state_t.stakes_t.lock(tx.agent_id, tx.task_id, tx.stake);
    q_next.economic_state_t.balances_t.debit(tx.agent_id, tx.stake);
    q_next.q_t.update_per_agent(tx.agent_id, |s| {
        s.last_accepted_tx = Some(tx.tx_id);
        s.retry_counter_for_current_task = 0;  // reset on accept
    });

    // L4 append
    let new_ledger_root = ledger::append(&q.ledger_root_t, tx);
    q_next.ledger_root_t = new_ledger_root;

    // L5 materialize
    let new_state_root = materializer::apply(&q.state_root_t, tx);
    q_next.state_root_t = new_state_root;

    // L6 signal emit (broadcast price + reputation; NOT evaluator internals — Inv 10)
    let signals = SignalBundle {
        boolean: vec![Signal::Boolean(BoolSignal::AcceptedAt(tx.tx_id))],
        statistical: vec![
            Signal::Statistical(StatSignal::PriceUpdate(price_for(tx.task_id, q_next.economic_state_t.price_index_t))),
            Signal::Statistical(StatSignal::ReputationDelta(tx.agent_id, +reputation_delta(tx))),
        ],
    };

    // STAGE 7: head advance
    q_next.head_t = NodeId::from_state_root(new_state_root);

    // STAGE 8: challenge window open (Inv 7 — finalization is deferred)
    q_next.economic_state_t.challenge_cases_t.open(claim, tx.timestamp_logical, CHALLENGE_WINDOW_TICKS);

    Ok((q_next, signals))
}
```

**No wall-clock, no env-var, no HashMap iteration**. Every input is either `q`, `tx`, or registries (themselves in `q.predicate_registry_root_t` / `q.tool_registry_root_t`).

### 3.1 verify_transition (VerifyTx)

Per Gemini v3.2 review Q10 VETO — extending pseudocode to all state-mutating tx types.

> **v1.1 note (gap 11.4)**: this pseudocode handles ONE verifier per tx. Multi-verifier quorum aggregation is a TaskMarket config (`verifier_quorum_required: usize` default = 1). When N>1 verifiers each submit verify_tx for the same target_work_tx, claim transitions to `Pending → ApprovedByVerifiers` only after `verifier_quorum_required` distinct verifiers have submitted `Confirm`. Aggregation rule deferred to CO P2.7 atom (Verifier role detail). For v4 default (quorum=1), each verify_tx independently advances claim to ApprovedByVerifiers.

```rust
pub fn verify_transition(
    q: &QState,
    tx: &VerifyTx,
    registry: &PredicateRegistry,
) -> Result<(QState, SignalBundle), TransitionError> {

    // STAGE 1: target work_tx must exist + be in Pending or Provisional state
    let target = q.economic_state_t.claims_t.get(&tx.target_work_tx)
        .ok_or(TransitionError::TargetWorkTxNotFound)?;
    if !target.status.allows_verification() {
        return Err(TransitionError::TargetWorkTxNotVerifiable);
    }

    // STAGE 2: signature + bond
    if !verify_signature(&tx.signature, tx.canonical_digest()) {
        return Err(TransitionError::SignatureInvalid);
    }
    let verifier_balance = q.economic_state_t.balances_t.get(&tx.verifier_agent);
    if verifier_balance < tx.bond {
        return Err(TransitionError::StakeInsufficient);
    }

    // STAGE 3: predicate gate (verifier predicate, NOT same as work_tx acceptance)
    let verify_results = registry.run_verification(tx, target, q)?;
    if !verify_results.all_passed() {
        return Err(TransitionError::VerificationPredicateFailed(verify_results));
    }

    // STAGE 4: state transition
    let mut q_next = q.clone();
    q_next.economic_state_t.balances_t.debit(tx.verifier_agent, tx.bond);
    q_next.economic_state_t.stakes_t.lock_verifier_bond(tx.verifier_agent, tx.target_work_tx, tx.bond);
    q_next.economic_state_t.claims_t.add_verification(tx.target_work_tx, tx.verifier_agent, tx.verdict);

    // STAGE 5: append + materialize + signals
    q_next.ledger_root_t = ledger::append(&q.ledger_root_t, tx);
    q_next.state_root_t  = materializer::apply(&q.state_root_t, tx);
    q_next.head_t        = NodeId::from_state_root(q_next.state_root_t);

    let signals = SignalBundle {
        boolean: vec![Signal::Boolean(BoolSignal::VerifiedAt(tx.tx_id))],
        statistical: vec![Signal::Statistical(StatSignal::ReputationDelta(tx.verifier_agent, +verify_reputation_delta(tx, target)))],
    };

    Ok((q_next, signals))
}
```

### 3.2 challenge_transition (ChallengeTx)

```rust
pub fn challenge_transition(
    q: &QState,
    tx: &ChallengeTx,
    registry: &PredicateRegistry,
) -> Result<(QState, SignalBundle), TransitionError> {

    // STAGE 1: target work_tx must exist + still in challenge window
    let target = q.economic_state_t.claims_t.get(&tx.target_work_tx)
        .ok_or(TransitionError::TargetWorkTxNotFound)?;
    let window = q.economic_state_t.challenge_cases_t.get(tx.target_work_tx)
        .ok_or(TransitionError::ChallengeWindowClosed)?;
    // v1.4: use ChallengeWindow::is_open(now) per § 5.2.5; same rule used by finalize_reward
    if !window.is_open(tx.timestamp_logical) {
        return Err(TransitionError::ChallengeWindowClosed);
    }

    // STAGE 2: signature + NO_E stake
    if !verify_signature(&tx.signature, tx.canonical_digest()) {
        return Err(TransitionError::SignatureInvalid);
    }
    let challenger_balance = q.economic_state_t.balances_t.get(&tx.challenger_agent);
    if challenger_balance < tx.stake {
        return Err(TransitionError::StakeInsufficient);
    }

    // STAGE 3: counterexample acceptance predicate (the BURDEN OF PROOF predicate, Inv 7)
    let counterexample = cas::get(&tx.counterexample_cid)?;
    let counter_check = registry.run_counterexample_check(target, &counterexample, q)?;
    if !counter_check.proves_violation() {
        return Err(TransitionError::CounterexampleInsufficient(counter_check));
    }

    // STAGE 4: state transition — ROLLBACK target work_tx + slash original solver + reward challenger
    let mut q_next = q.clone();
    q_next.economic_state_t.balances_t.debit(tx.challenger_agent, tx.stake);

    // 4a: rollback target's provisional reward
    let rollback_amount = q.economic_state_t.claims_t.provisional_amount(tx.target_work_tx);
    q_next.economic_state_t.claims_t.mark_slashed(tx.target_work_tx, tx.tx_id);

    // 4b: slash original solver's stake → reward pool for challenger
    let solver_stake = q.economic_state_t.stakes_t.get(target.solver, target.task_id);
    q_next.economic_state_t.stakes_t.slash(target.solver, target.task_id);
    q_next.economic_state_t.escrows_t.deposit_from_slash(tx.challenger_agent, solver_stake);

    // 4c: challenger gets back NO_E stake + slashed solver stake
    q_next.economic_state_t.balances_t.credit(tx.challenger_agent, tx.stake + solver_stake);

    // 4d: solver reputation -= delta; challenger reputation += delta (Inv 9 immutable but we update via formula not transfer)
    q_next.economic_state_t.reputations_t.adjust(target.solver, -slash_reputation_delta());
    q_next.economic_state_t.reputations_t.adjust(tx.challenger_agent, +challenge_reputation_delta());

    // 4e: verifier_bond release per task config (gap 11.2 fix; default = return to good-faith verifier)
    //   Rationale: when Carol slashes Alice via challenge, Bob (the verifier) was duped but acted in good faith.
    //   Slashing Bob's bond would discourage future verification. Configurable per TaskMarket.
    //   Applies to ALL verifiers who voted Confirm on the slashed work_tx.
    let bond_release_policy = q.economic_state_t.task_markets_t
        .get(target.task_id)
        .map(|tm| tm.config.verifier_bond_on_slash)
        .unwrap_or(VerifierBondPolicy::ReturnToVerifier);
    for (verifier, bond) in q.economic_state_t.stakes_t.verifier_bonds_for(tx.target_work_tx) {
        match bond_release_policy {
            VerifierBondPolicy::ReturnToVerifier => {
                q_next.economic_state_t.balances_t.credit(verifier, bond);
                q_next.economic_state_t.stakes_t.release_verifier_bond(verifier, tx.target_work_tx);
            }
            VerifierBondPolicy::SlashedToChallenger => {
                q_next.economic_state_t.balances_t.credit(tx.challenger_agent, bond);
                q_next.economic_state_t.stakes_t.slash_verifier_bond(verifier, tx.target_work_tx);
                q_next.economic_state_t.reputations_t.adjust(verifier, -verifier_slash_delta());
            }
        }
    }

    // STAGE 5: close challenge window
    q_next.economic_state_t.challenge_cases_t.close(tx.target_work_tx, ChallengeOutcome::Slashed(tx.tx_id));

    // STAGE 6: append + materialize + signals
    q_next.ledger_root_t = ledger::append(&q.ledger_root_t, tx);
    q_next.state_root_t  = materializer::apply(&q.state_root_t, tx);
    q_next.head_t        = NodeId::from_state_root(q_next.state_root_t);

    let signals = SignalBundle {
        boolean: vec![Signal::Boolean(BoolSignal::ChallengeUpheld(tx.tx_id))],
        statistical: vec![
            Signal::Statistical(StatSignal::ReputationDelta(target.solver, -slash_reputation_delta())),
            Signal::Statistical(StatSignal::ReputationDelta(tx.challenger_agent, +challenge_reputation_delta())),
        ],
    };

    Ok((q_next, signals))
}
```

### 3.3 reuse_transition (ReuseTx)

```rust
pub fn reuse_transition(
    q: &QState,
    tx: &ReuseTx,
    tool_registry: &ToolRegistry,
) -> Result<(QState, SignalBundle), TransitionError> {
    // STAGE 1: tool must be registered + still active in L2
    let tool = tool_registry.get(tx.reused_tool_id)
        .ok_or(TransitionError::ToolNotInRegistry)?;
    if tool.creator != tx.reused_tool_creator {
        return Err(TransitionError::ToolCreatorMismatch);
    }

    // STAGE 2: parent reusing_work_tx must exist + Accepted
    let parent = q.economic_state_t.claims_t.get(&tx.reusing_work_tx)
        .ok_or(TransitionError::TargetWorkTxNotFound)?;
    if !parent.status.is_accepted_or_finalized() {
        return Err(TransitionError::ParentNotAcceptedYet);
    }

    // STAGE 3: state transition — add edge to royalty graph
    //   gap 11.3 fix: weight bounded by MAX_REUSE_ROYALTY_FRACTION = 0.10 default
    //   Rationale: 10% upper bound protects solver's primary reward. Builders earn via creating
    //   widely-reusable tools, not via single high-percentage extractions. Configurable per TaskMarket
    //   for cases where user wants to override (e.g., creator-economy experiments).
    let max_royalty = q.economic_state_t.task_markets_t
        .get(parent.task_id)
        .and_then(|tm| tm.config.max_reuse_royalty_fraction)
        .unwrap_or(MAX_REUSE_ROYALTY_FRACTION_DEFAULT);  // = 0.10 in micro-coin fractional repr (10000 / 100000)
    let bounded_weight = tool.reuse_royalty_share.min(max_royalty);
    if tool.reuse_royalty_share > max_royalty {
        log::warn!(
            "reuse_tx {}: tool {} declared royalty {} > max {}; clamping to {}",
            tx.tx_id, tx.reused_tool_id, tool.reuse_royalty_share, max_royalty, bounded_weight
        );
    }

    let mut q_next = q.clone();
    q_next.economic_state_t.royalty_graph_t.add_edge(
        from: tx.reusing_work_tx,
        to:   tx.reused_tool_id,
        creator: tx.reused_tool_creator,
        weight: bounded_weight,    // clamped per gap 11.3
    );

    // STAGE 4: append + materialize (no signals; royalty paid at finalize_reward time)
    q_next.ledger_root_t = ledger::append(&q.ledger_root_t, tx);
    q_next.state_root_t  = materializer::apply(&q.state_root_t, tx);
    q_next.head_t        = NodeId::from_state_root(q_next.state_root_t);

    Ok((q_next, SignalBundle::empty()))
}
```

### 3.4 finalize_reward (challenge window expiry)

Triggered by tick (no agent submits this; runtime emits when challenge window expires for any provisional claim).

```rust
pub fn finalize_reward_transition(
    q: &QState,
    claim_id: ClaimId,
    settlement_engine: &SettlementEngine,
) -> Result<(QState, SignalBundle), TransitionError> {
    let claim = q.economic_state_t.claims_t.get(&claim_id)
        .ok_or(TransitionError::ClaimNotFound)?;
    let window = q.economic_state_t.challenge_cases_t.get(claim.target_work_tx);

    // STAGE 1: window must be expired AND no open slash
    // v1.4: invoke ChallengeWindow::is_open(now) per § 5.2.5 with explicit `now` arg;
    // same rule as challenge_transition stage 1
    if let Some(w) = window {
        if w.is_open(q.q_t.current_round) {
            return Err(TransitionError::ChallengeWindowStillOpen);
        }
        if w.outcome == Some(ChallengeOutcome::Slashed(_)) {
            return Err(TransitionError::AlreadySlashed);  // never finalize a slashed claim
        }
    }

    // STAGE 2: compute reward per Economic § 21 final formula
    let reward = settlement_engine.finalize(
        claim,
        Escrow::lookup(q, claim.task_id),
        Attribution::lookup(q, claim.target_work_tx),
        Survival::full,  // window expired without slash
        Utility::lookup(q, claim.target_work_tx),
        Constitution::check(q),
    )?;

    // STAGE 3: state transition
    let mut q_next = q.clone();
    let target = claim.target_work_tx_data;

    // 3a (v1.2 NEW; gap 11.A per Gemini + Codex Q2): unlock + return solver's stake
    // Without this, every successful solver permanently loses their stake → Inv 3 violation.
    let solver_stake_locked = q.economic_state_t.stakes_t.get(target.solver, target.task_id);
    q_next.economic_state_t.stakes_t.unlock(target.solver, target.task_id);
    q_next.economic_state_t.balances_t.credit(target.solver, solver_stake_locked);

    // 3b: credit reward + finalize claim + debit escrow
    q_next.economic_state_t.balances_t.credit(target.solver, reward);
    q_next.economic_state_t.claims_t.finalize(claim_id, reward);
    q_next.economic_state_t.escrows_t.debit(claim.task_id, reward);

    // 3c: pay royalties along royalty_graph_t edges (v1.2 explicit rounding rule per Codex Q3 + Q10)
    // Royalty math uses i64 micro-coin throughout; rounding = integer floor (round-down) to preserve Inv 3.
    // No f64; no implicit casts. weight stored as MicroFraction (i64 in 1_000_000 units representing 0.0..1.0).
    let reward_micro = reward.to_micro_units();    // i64
    for edge in q.economic_state_t.royalty_graph_t.edges_from(claim.target_work_tx) {
        let royalty_micro = reward_micro
            .checked_mul(edge.weight.micro_units())
            .expect("overflow")
            / 1_000_000;    // integer floor; deterministic across platforms
        let royalty = MicroCoin::from_micro_units(royalty_micro);
        q_next.economic_state_t.balances_t.credit(edge.creator, royalty);
        q_next.economic_state_t.balances_t.debit(target.solver, royalty);  // royalty comes from solver's reward, not extra mint (Inv 4)
    }
    // Note: integer floor means total royalty payments may be < `reward × Σ weights` by up to `n` micro-units (1 per edge);
    // the dust remains in solver's balance. This is intentional and consistent with Bitcoin satoshi rounding.

    // STAGE 4: emit terminal signals
    q_next.ledger_root_t = ledger::append(&q.ledger_root_t, &FinalizeTx::from(claim_id, reward));
    q_next.state_root_t  = materializer::apply(&q.state_root_t, &FinalizeTx::from(claim_id, reward));
    q_next.head_t        = NodeId::from_state_root(q_next.state_root_t);

    Ok((q_next, SignalBundle::finalize(claim_id, reward)))
}
```

### 3.6 task_expire_transition (v1.3 refactored: pure boundary preserved per Codex re-audit)

**Why**: a TaskMarket entry has a deadline; if no work_tx is accepted by deadline, the bounty MUST refund to task creator (otherwise Inv 3 monetary conservation broken: bounty trapped in escrow forever).

**v1.3 fix**: split runtime side effects (signing, logical time assignment) from pure transition. Runtime constructs `TaskExpireTx` BEFORE entering pure transition; pure `task_expire_transition` takes already-signed tx as argument. This restores § 2 + § 3 pure-boundary discipline (Codex Q1.3 + new-issue #1 fix).

```rust
// PURE transition (used by both branch A and branch B in STEP_B)
pub fn task_expire_transition(
    q: &QState,
    tx: &TaskExpireTx,    // v1.3: already-signed by runtime BEFORE entry
) -> Result<(QState, SignalBundle), TransitionError> {
    let task = q.economic_state_t.task_markets_t.get(tx.task_id)
        .ok_or(TransitionError::TaskNotFound)?;

    // STAGE 1: signature verification (system signature; not agent)
    if !verify_system_signature(&tx.system_signature, &tx, q.system_pubkey_at_epoch(tx.epoch)) {
        return Err(TransitionError::InvalidSystemSignature);
    }

    // STAGE 2: parent_state_root match (stale view rejection)
    if tx.parent_state_root != q.state_root_t {
        return Err(TransitionError::StaleParent);
    }

    // STAGE 3: expiry check — task must be expired AND have NO Pending OR Provisional OR Finalized claim
    // v1.3 fix (Codex new-issue #2): broaden race-protection from "Finalized only" to all claim statuses
    if task.deadline_logical_t > q.q_t.current_round {
        return Err(TransitionError::TaskNotExpired);
    }
    if q.economic_state_t.claims_t.any_claim_for_task(tx.task_id) {
        return Err(TransitionError::TaskHasOpenClaim);    // refund only if NO claim exists at all
    }

    // STAGE 4: refund bounty from escrow to task creator
    let mut q_next = q.clone();
    let bounty = q.economic_state_t.escrows_t.get(tx.task_id);
    q_next.economic_state_t.escrows_t.refund(tx.task_id);
    q_next.economic_state_t.balances_t.credit(task.creator, bounty);

    // STAGE 5: refund any solver stakes still locked on expired task
    for (agent, locked_stake) in q.economic_state_t.stakes_t.all_locked_for_task(tx.task_id) {
        q_next.economic_state_t.stakes_t.unlock(agent, tx.task_id);
        q_next.economic_state_t.balances_t.credit(agent, locked_stake);
    }

    // STAGE 6: remove task from active markets
    q_next.economic_state_t.task_markets_t.remove(tx.task_id);

    // STAGE 7: append + materialize + signal (purely on tx, q)
    q_next.ledger_root_t = ledger::append(&q.ledger_root_t, tx);
    q_next.state_root_t  = materializer::apply(&q.state_root_t, tx);
    q_next.head_t        = NodeId::from_state_root(q_next.state_root_t);

    let signals = SignalBundle::task_expired(tx.task_id, bounty);

    Ok((q_next, signals))
}

// TaskExpireTx schema (v1.3 NEW typed schema):
pub struct TaskExpireTx {
    pub tx_id: TxId,
    pub task_id: TaskId,
    pub parent_state_root: Hash,
    pub bounty_refunded: MicroCoin,         // for ledger summary; runtime computes from q
    pub epoch: SystemEpoch,                  // which keypair signed
    pub timestamp_logical: u64,              // assigned by runtime BEFORE pure transition
    pub system_signature: SystemSignature,   // computed by runtime BEFORE pure transition
}
```

**Trigger**: runtime tick scans for expired tasks; for each, runtime:
1. Calls `runtime.next_logical_t()` to get next logical_t
2. Constructs `TaskExpireTx` with current `q.state_root_t` as parent
3. Signs `TaskExpireTx` via `runtime.system_keypair().sign(canonical_digest(&tx))`
4. Submits signed tx to L4 sequencer (§ 5.2.1)
5. Sequencer calls pure `task_expire_transition(q, &tx)`

This split is identical to how `WorkTx` is constructed by agent BEFORE submitting to pure `step_transition`. Agents construct + sign; runtime constructs + sign for system tx. Pure transition fn is `(q, tx) → (q', signals)` in BOTH cases.

### 3.6.5 Agent Implicit Init (v1.3 fixed: trait-based submitter resolution per Codex re-audit Q1.4)

**Where**: applies to ALL agent-submitted transitions (work_transition / verify_transition / challenge_transition / reuse_transition). Inline at stage 4 of each, before user-state mutations.

**v1.3 fix**: WorkTx has `agent_id`; VerifyTx has `verifier_agent`; ChallengeTx has `challenger_agent`; ReuseTx has no submitting-agent field (it's a fact-tx). Introduce a `Tx::submitter_id() -> Option<AgentId>` trait method that each tx implements explicitly:

```rust
pub trait HasSubmitter {
    fn submitter_id(&self) -> Option<AgentId>;
}

impl HasSubmitter for WorkTx       { fn submitter_id(&self) -> Option<AgentId> { Some(self.agent_id.clone()) } }
impl HasSubmitter for VerifyTx     { fn submitter_id(&self) -> Option<AgentId> { Some(self.verifier_agent.clone()) } }
impl HasSubmitter for ChallengeTx  { fn submitter_id(&self) -> Option<AgentId> { Some(self.challenger_agent.clone()) } }
impl HasSubmitter for ReuseTx      { fn submitter_id(&self) -> Option<AgentId> { None }    // ReuseTx has no submitting agent; reuse facts derive from L4 read_set }

// In each agent-submitted transition's stage 4, INLINE this snippet:
fn implicit_init_agent_if_new(q_next: &mut QState, tx: &impl HasSubmitter) {
    if let Some(submitter) = tx.submitter_id() {
        if !q_next.q_t.agents.contains_key(&submitter) {
            q_next.q_t.agents.insert(submitter, PerAgentState {
                reputation_snapshot: Reputation::default_initial(),    // = 0
                last_accepted_tx: None,
                retry_counter_for_current_task: 0,
            });
        }
    }
}
```

**Rule**: each transition function MUST call `implicit_init_agent_if_new(&mut q_next, tx)` as the FIRST statement of stage 4 (after stage 3 predicate gate, before any user-state mutation). For `ReuseTx`, `submitter_id()` returns None; no init happens; that's intentional (ReuseTx has no submitting agent to init).

**Why implicit (not explicit `register_agent_transition`)**:
- Satoshi parallel: Bitcoin addresses are implicitly created at first use; no separate register step
- Avoids gatekeeping: any agent submitting a valid signed tx joins the system
- v4 single-user friendly: gretjia + Codex/Gemini auto-discoverable
- v4.1+: if needed, can add explicit `agent_register_tx` later WITHOUT breaking implicit-init (new tx is purely additive)

### 3.7 emit_terminal_summary (run-end without acceptance)

```rust
pub fn emit_terminal_summary_transition(
    q: &QState,
    run_id: RunId,
    runtime: &Runtime,
) -> Result<(QState, SignalBundle), TransitionError> {
    let run = runtime.run_state(run_id)?;
    if run.has_accepted_work_tx() {
        return Err(TransitionError::TerminalSummaryNotApplicable);  // only emitted for no-accept runs
    }

    let summary = TerminalSummaryTx {
        tx_id: TxId::derive(run_id, "terminal"),
        task_id: run.task_id,
        run_id,
        run_outcome: run.outcome(),
        total_attempts: run.attempt_counter(),
        failure_class_histogram: run.failure_histogram(),
        last_logical_t: run.last_logical_t(),
        system_signature: runtime.system_keypair().sign(canonical_digest_terminal(run)),
    };

    // STAGE: append; materialize; emit failure-class signals to L6
    let mut q_next = q.clone();
    q_next.ledger_root_t = ledger::append(&q.ledger_root_t, &summary);
    q_next.state_root_t  = materializer::apply(&q.state_root_t, &summary);
    q_next.head_t        = NodeId::from_state_root(q_next.state_root_t);

    let signals = SignalBundle::terminal_summary(&summary);

    Ok((q_next, signals))
}
```

---

## § 4 Named Invariants (machine-checkable)

| ID | Invariant | Enforced at | Conformance test |
|---|---|---|---|
| I-DET | Same (Q_t, tx) → byte-identical (Q_{t+1}, signals) | step_transition stage 6-8 | `tests/transition_determinism.rs` |
| I-DETHASH | `state_root_t` after replay from genesis matches authoritative state | replay test | `tests/q_state_reconstruct.rs` |
| I-NOSIDE | step_transition reads only (q, tx, registries); no I/O | static analysis grep + cargo-deny | `tests/no_hidden_inputs.rs` |
| I-PARENT | tx.parent_state_root must equal q.state_root_t | stage 1 | `tests/stale_parent_rejection.rs` |
| I-SIG | tx.signature verifies against tx.canonical_digest() | stage 2 | `tests/signature_verification.rs` |
| I-STAKE | tx.stake ≤ q.balances_t[tx.agent_id]; debit atomic | stage 3, 6 | `tests/stake_atomicity.rs` |
| I-PRED-GATE | rejected work_tx does NOT advance state_root_t | stage 4 | `tests/economic_invariant_INV6_predicate_gated.rs` |
| I-PROV | accepted work_tx → provisional claim, NOT finalized reward | stage 5 | `tests/economic_invariant_INV7_provisional_then_final.rs` |
| I-LOGTIME | timestamp_logical strictly monotonic per-tx; no wall clock | stage 6 | `tests/no_wall_clock_in_tx.rs` |
| I-MICROCOIN | all monetary fields are MicroCoin (i64 newtype) | type system | compile-time + `tests/no_f64_money.rs` |
| I-BTREE | Q_t indices use BTreeMap, not HashMap (deterministic order) | type system | `tests/q_state_uses_btree.rs` |
| I-NOSIDECAR | no Vec/HashMap "graveyard"-like sidecar (Art. 0.2) | static analysis | `tests/no_rejection_sidecar.rs` |
| I-RETRY | RejectedAttemptSummary stamped by runner, not agent | stamp call site | `tests/retry_summary_runner_signed.rs` |
| I-TERMINAL | every run terminates with at least one of: accepted work_tx OR TerminalSummaryTx | run finalize hook | `tests/run_terminal_invariant.rs` |
| I-NOENV | step_transition dependency tree contains no `std::env` access | cargo-deny + grep | `tests/no_env_in_transition.rs` |
| I-FREEZE-CONFIG | TAPE_ECONOMY_V2 + FOUNDER_GRANT_GAMMA + system_lp_amount frozen at task creation, not at tx submission | TaskMarket::publish | `tests/task_config_frozen_at_publish.rs` |
| **I-NORANDOM** (added per Gemini v3.2 review Q1) | Any tx that consumes randomness MUST seed PRNG from `(tx.tx_id, q.state_root_t)`; no system entropy in step_transition path | step_transition stages 1-7 | `tests/no_runtime_entropy.rs` |
| **I-VERIFY-LIVE** (added per Gemini v3.2 review Q10) | VerifyTx targets MUST be in Pending or Provisional state; cannot verify Accepted-and-finalized or Slashed | verify_transition stage 1 | `tests/verify_target_liveness.rs` |
| **I-CHAL-WINDOW** (added per Gemini v3.2 review Q10) | ChallengeTx must be received within target's challenge_cases_t window; no challenges after window close | challenge_transition stage 1 | `tests/challenge_window_enforced.rs` |
| **I-FINALIZE-EXCLUSIVE** (added) | FinalizeRewardTx and SlashTx are mutually exclusive per claim_id; system runtime serializes | finalize_reward_transition stage 2 | `tests/finalize_or_slash_exclusive.rs` |
| **I-VBOND-RELEASE** (v1.1, gap 11.2 fix) | Verifier bond release on slashed work_tx follows TaskMarket.config.verifier_bond_on_slash policy; default = `ReturnToVerifier`; verifier reputation NOT adjusted under default policy | challenge_transition stage 4e | `tests/verifier_bond_release.rs` |
| **I-ROYALTY-CAP** (v1.1, gap 11.3 fix) | reuse_tx edge weight ≤ TaskMarket.config.max_reuse_royalty_fraction (default 0.10); excess clamped + warning logged | reuse_transition stage 3 | `tests/royalty_cap_enforced.rs` |
| **I-STAKE-RETURN** (v1.2 NEW per Gemini Q2 + Codex Q2) | Successful unchallenged finalize_reward returns + unlocks solver's locked stake exactly once (in addition to reward credit). Test attempts double-claim. | finalize_reward_transition stage 3a | `tests/stake_return_on_finalize.rs` |
| **I-BOUNTY-REFUND** (v1.2 NEW per Gemini Q2 + Codex Q2) | task_expire_transition refunds full bounty to creator + refunds any locked solver stakes when no claim finalized by deadline | task_expire_transition stages 2-3 | `tests/bounty_refund_on_expire.rs` |
| **I-FINALIZE-BATCH-ORDER** (v1.3 corrected: single key throughout) | When N claims become finalizable at the same logical_t, finalize_tx emit order is `(expires_at_logical ASC, claim_id ASC)` — `claim_id` (NOT `target_work_tx`) used everywhere: invariant + § 5.2.3 + conformance test all consistent. | runtime finalize loop + § 5.2.3 | `tests/finalize_batch_order.rs` |
| **I-CHALLENGE-WINDOW-EDGE** (v1.3 finalize binding fixed) | Challenge window is `[opens_at, opens_at + duration_ticks)` — left-inclusive, right-exclusive. `is_open(now)` defined as `now < opens_at + duration_ticks`. **Both** challenge_transition stage 1 AND finalize_reward stage 1 MUST use `is_open(q.q_t.current_round)` (NOT a different rule). | challenge_transition + finalize_reward_transition | `tests/challenge_window_edge.rs` |
| **I-AGENT-INIT** (v1.2 NEW per Gemini Q2) | First appearance of agent in L4 transition tx implicitly initializes q_t.agents[id] with reputation=0; subsequent appearances do not re-initialize | work/verify/challenge/reuse_transition stage 4 | `tests/agent_implicit_init.rs` |

**Total: 27 invariants → 27 tests** (was 22 in v1.1; +5 in v1.2). Every transition test must pass before CO1.1.4 (bus.rs split) starts. STEP_B implementation comparison is "branch X conforms to spec" / "branch Y conforms to spec", not "branch X looks like branch Y".

---

## § 5 Optional TLA+ Skeleton (deferred to spec-gate audit)

For ordering + replay invariants (I-DET, I-DETHASH, I-LOGTIME), Codex suggested TLA+/PlusCal. ArchitectAI agrees with the suggestion but does NOT include the full model in v1 of this spec — it would balloon the doc. Skeleton:

```tla
EXTENDS Naturals, Sequences

VARIABLES q, ledger, signals

Init == /\ q = GenesisQState
        /\ ledger = <<>>
        /\ signals = <<>>

Step(tx) == /\ ValidParent(tx, q)
            /\ ValidSignature(tx)
            /\ StakeAvailable(tx, q)
            /\ AcceptancePredicates(tx, q)
            /\ q' = Apply(q, tx)
            /\ ledger' = Append(ledger, tx)
            /\ signals' = EmitSignals(q, tx, q')

Spec == Init /\ [][\E tx \in WorkTx : Step(tx)]_<<q, ledger, signals>>

\* Determinism: same input sequence → same final state
DeterminismProperty == \A seq1, seq2 \in Seq(WorkTx) :
    (seq1 = seq2) => (Replay(seq1) = Replay(seq2))
```

If CO P1 audit demands stronger guarantees, the TLA+ model is upgraded to a full PlusCal program with TLC model checking. For v4 scope, the type-level + conformance-test combination is deemed sufficient by Codex.

---

## § 5.1 v1.1 Walk-Through Gap Resolutions

Per `SPEC_WALKTHROUGH_v1_2026-04-27.md` § 11, four spec gaps were found. Resolution status:

| Gap | Issue | v1.2 Resolution | User-overridable |
|---|---|---|---|
| 11.1 | False-challenge reputation penalty undefined | **v1.2 (Codex Q10 fix)**: false_challenge_reputation_penalty is **fixed to 0 in v4** (NOT configurable). Pseudocode resolves contradiction: challenge_transition stage 3 returns `Err(CounterexampleInsufficient)` BEFORE any state mutation; no executable path for nonzero penalty → "configurable" prose retired. v4.1+ MAY add explicit `failed_challenge_penalty_transition` if needed. | NO (v4) |
| 11.2 | Verifier bond release policy on slashed claim | spec § 3.2 stage 4e ADDED with `VerifierBondPolicy::ReturnToVerifier` default | yes — `verifier_bond_on_slash` config |
| 11.3 | Royalty edge weight bound | spec § 3.3 stage 3 ADDED with `MAX_REUSE_ROYALTY_FRACTION_DEFAULT = 0.10` | yes — `max_reuse_royalty_fraction` config |
| 11.4 | Multi-verifier quorum aggregation | spec § 3.1 note ADDED with `verifier_quorum_required: usize = 1` default; full multi-verifier impl deferred to CO P2.7 | yes — set per TaskMarket |

All 4 gaps now have machine-checkable defaults. User can override 11.2/11.3/11.4 defaults via TaskMarket.config when creating tasks; the default applies if config field is missing. **11.1 (false-challenge penalty) is NOT user-overridable in v4** (fixed to 0; v4.1+ may introduce a separate `failed_challenge_penalty_transition` if needed).

---

## § 5.2 Concurrency Rules (v1.2 NEW per Codex Q6)

**Why**: spec § 3 pseudocode is single-threaded; CO P1 may parallelize Phase C 5 modes × N seeds. Without explicit serialization rule, two work_tx can race on same parent_state_root, both pass `I-PARENT`, but produce different (logical_t, tx_id) sequences across STEP_B branches → state_root divergence.

### 5.2.1 L4 Sequencer

**Per (runtime_repo, run_id)** there is exactly ONE L4 sequencer instance. The sequencer:
1. **Receives** tx submissions in any order (concurrent-safe queue)
2. **Assigns** monotonic `(logical_t, tx_id)` ordering key:
   - `logical_t = sequencer.next_logical_t()` (atomic counter; starts at 1 per genesis)
   - `tx_id = TxId::derive(logical_t, agent_id, payload_hash)` (deterministic from above)
3. **Serializes** transition execution: takes 1 tx at a time from queue in submission order; calls pure `step_transition`
4. **Commits** result to L4 (WAL write + git commit) BEFORE accepting next tx

**Async completion order is NEVER an ordering source**. Even if async tasks finish out-of-order, sequencer enforces submission-order ingestion.

### 5.2.2 Cross-Cell Isolation

**Phase C 5-mode × 10-problem × N-seed cells** (per `CO1_3_1_GIX_SPIKE_PREFLIGHT § 1` C4) MUST use:
- **Disjoint `runtime_repo`** (different filesystem path; no shared state)
- **Disjoint `QState`** (each cell has its own genesis_payload + Q_t replay)
- **No shared L4 sequencer** (each cell has its own)

If a future deployment shares runtime_repo across cells (e.g., multi-tenant): MUST add **ref locks** (gix branch refs serve as atomic guards) + **deterministic retry semantics** (failed lock → wait 100ms × n_attempts; deterministic seed from `(run_id, tx_id)`).

### 5.2.3 Finalize Batch Order

When N claims expire at the same `logical_t`:
- Order = `(claim.expires_at_logical ASC, claim.claim_id ASC)` (stable, deterministic) — v1.3 fix: uses `claim_id` consistently (NOT `target_work_tx`) to align with `I-FINALIZE-BATCH-ORDER` invariant + conformance test
- Sequencer emits `finalize_reward_transition` ONE AT A TIME in this order
- Each finalize advances state_root before next finalize starts

### 5.2.4 Conformance Tests

- `tests/l4_sequencer_serialization.rs` — concurrent submit; assert single-threaded execution by sequencer; same input order → same state_root
- `tests/cross_cell_isolation.rs` — 5 cells run; assert disjoint state_roots; no cross-contamination
- `tests/finalize_batch_order.rs` — 3 claims expire same tick; assert ordering by (expires_at, claim_id); 2 runs byte-identical

### 5.2.5 ChallengeWindow::is_open (v1.4 NEW per Codex Q2.4)

```rust
impl ChallengeWindow {
    /// Half-open interval `[opens_at, opens_at + duration_ticks)`.
    /// Both challenge_transition stage 1 AND finalize_reward stage 1 MUST invoke this method
    /// (NOT hand-code the inequality) to guarantee consistent edge semantics.
    pub fn is_open(&self, now: u64) -> bool {
        now >= self.opens_at && now < self.opens_at + self.duration_ticks
    }
}
```

**Invariant binding**: `I-CHALLENGE-WINDOW-EDGE` enforces that BOTH transition functions call `is_open(now)` rather than hand-coding the boundary check. STEP_B branch A vs branch B both implement the same `is_open()`; cross-branch comparison verifies identical results for all (opens_at, duration_ticks, now) triples.

### 5.2.6 Sequencer Tie-Break (v1.4 NEW per Codex Q6)

When multiple agent threads concurrently call sequencer's `submit(tx)`, the sequencer's atomic `next_logical_t()` (§ 5.2.1 step 2) provides the **canonical tie-breaker**:

- `logical_t` assignments are produced by atomic increment (e.g., `AtomicU64::fetch_add`)
- The order in which threads receive their `logical_t` values IS the canonical ordering
- "Submission order" = the order of `logical_t` assignment, NOT wall-clock arrival order
- For two `tx` arriving at the same nanosecond on different threads, whichever thread wins the atomic gets the lower `logical_t`; the other gets the next higher

This means: STEP_B branch A and branch B may serialize threads differently (depending on OS scheduler), but as long as both branches use atomic logical_t assignment + replay from the SAME logical_t sequence, they produce byte-identical state_roots.

**Conformance test addition** (extends `tests/l4_sequencer_serialization.rs`): submit 100 tx concurrently from 8 threads; assert `(logical_t, tx_id_hash)` is a strict total order; replay produces deterministic state_root regardless of thread interleaving.

### 5.2.7 What This Does NOT Specify

- Async runtime choice (tokio vs std::thread): runtime concern, not spec; spec only requires sequencer property
- Sequencer implementation: lock-free queue, mutex, channel — implementation detail
- Cross-cell sharing pattern (post-v4): future v4.x extension

## § 5.3 Legacy Economic Tx Disposition (v1.3 NEW per Codex Q1.1)

The current pre-CO-P1 codebase contains economic mutation surfaces in `src/bus.rs` and `src/kernel.rs` that have NO direct equivalent in v1.x typed transitions:

| Legacy mutation | Current location | v4 disposition |
|---|---|---|
| `Invest` event (agent stakes Coin to YES/NO market position) | `src/bus.rs:229-252,285-290` `handle_invest_only` + market interactions | **RETIRED in CO1.1.4** — agent staking now goes through `WorkTx.stake` (YES_E) or `ChallengeTx.stake` (NO_E); no separate InvestTx. |
| `TaskMarketPublish` (task creator publishes new task) | implicit in current code; tasks hardcoded | **NEW v1 transition (deferred to CO P2.1)** — `TaskMarketPublishTx` lands in CO P2.1 atom; v1.x spec stubs the schema only |
| `MarketCreate` (per-node market on each tape append) | `src/bus.rs:285-290` + `src/kernel.rs:114-126` `Kernel::create_market` | **RETIRED in CO1.1.5** — per-node markets are an artifact of the Phase A "every node = market" pattern; CO P2.1 TaskMarket replaces with per-task markets only |
| `MarketResolve` (settle markets at OMEGA accept) | `src/kernel.rs:156-206` `Kernel::resolve_all` | **RETIRED in CO1.1.5** — market resolution becomes part of `finalize_reward_transition` (per-task, per-claim); no separate market-resolve event |
| `RunEnd` / `halt_and_settle` (run-level settlement) | `src/bus.rs:355-375` `TuringBus::halt_and_settle` | **RETIRED in CO1.1.4** — run-end becomes implicit via `TerminalSummaryTx` (§ 3.7) for no-accept runs OR `finalize_reward_transition` for accepted runs |
| WAL append side effect | `src/bus.rs:273-282` + `:319-327` | **MOVED to runtime layer**, not transition: spec § 5.2.1 sequencer commits L4 entries AFTER pure `step_transition` returns |
| Tool post-append hook | `src/bus.rs:312-318` `tool.on_post_append()` | **RETIRED**: tool hooks become explicit ToolInvocation field in `WorkTx.write_set` (read by predicate runner); no separate hook |

**Conformance test**: `tests/legacy_economic_tx_retired.rs` greps post-CO1.1.4/CO1.1.5 codebase for: `Invest` event variant, `Kernel::create_market`, `Kernel::resolve_all`, `halt_and_settle`, `tool.on_post_append`. Each must return 0 hits in the new `src/{top_white,middle_black,bottom_white,economy,state,transition}/*` dirs (matches in old `src/{bus,kernel}.rs` ARE expected if those files still exist as legacy markers; CO1.1.4 atom retires them).

**Why retired-not-renamed**: each legacy operation is either (a) absorbed into a v1.x typed transition (Invest → WorkTx.stake; Resolve → finalize_reward) OR (b) moved to runtime layer (WAL append; tool hook). Direct rename would preserve the old monolithic semantics.

## § 6 What This Spec DOES NOT Specify

Listed for honesty:

1. **MetaTx full schema** — only stub here; v4.1 atom defines.
2. **AttributionEngine deterministic DAG construction** — CO2.4.0 spike (separate doc).
3. **Predicate visibility leak channels** — covered at CO P1.5 design (Goodhart shield); this spec only declares `BoolWithProof.proof_visibility_class`, not the leak-proof proof format.
4. **gix Path B substrate-specific operations** — CO1.3.1 spike validates; this spec is substrate-agnostic.
5. **Retry metadata bound on `failed_attempts_since_last_accept`** — must be finite for tape size containment, but exact bound (e.g., u32::MAX vs cap-at-1000) is CO P1.7 design choice.
6. **Verifier verdict aggregation rule** — when N verifiers vote, how to combine? CO2.7 design.
7. **Challenge window length** — `CHALLENGE_WINDOW_TICKS` is a TaskMarket config bound at publish, but the default value + bounds are CO2.5 design.

These deferrals are **explicit and named**. Future atoms reference this list to resolve them.

---

## § 7 Pre-CO P1 Gate Procedure

1. ArchitectAI commits this spec v1
2. Codex independent review: confirm that every WP § 4-7 + economic § 2/§ 6 / § 18-21 concept maps to a typed field or invariant here
3. Gemini cross-review: confirm spec respects ENTIRE white paper (not just cited §)
4. Both PASS → spec frozen as v1 (any change requires re-audit)
5. **Then** Plan v3.2 atom CO1.SPEC.0 marked complete; CO1.0 / CO1.1.* / CO1.2.* atoms cleared to start
6. STEP_B implementation: Claude implements branch A against spec; Codex implements branch B against spec; comparison metric = "spec conformance", not "code similarity"

---

## § 8 Honest Acknowledgements

What this spec is:
- A typed, deterministic, side-effect-free state transition definition
- A binding contract for STEP_B branch A/B comparison
- A list of **27 named invariants** (was 16 in v1; 22 in v1.1; +5 in v1.2: I-STAKE-RETURN / I-BOUNTY-REFUND / I-FINALIZE-BATCH-ORDER / I-CHALLENGE-WINDOW-EDGE / I-AGENT-INIT) each backed by a conformance test path

What this spec is NOT:
- A full formal proof (no Lean/Coq)
- A complete TLA+ model (skeleton only)
- A substitute for code review (still required per Protocol Hard rule 1+2)
- A guarantee that branches A/B will produce identical Rust code (only spec-equivalent code)

What this spec does NOT yet include and the user must decide:
- Whether to run full TLA+ TLC model check (~3-5 day effort) or stop at type+test level (Codex suggested optional)
- Whether `RejectionClass::Opaque` aggregation respects Goodhart shield in practice (deferred to CO P1.5)
- Whether to embed Art 0.2 mini-amendment (see `ART_0_2_REINTERPRETATION_2026-04-27.md`) BEFORE running this spec, or AFTER (depends on rejection-on-tape constitutional reading)

— ArchitectAI, 2026-04-27


---

# XREF: shipped src/bottom_white/ledger/transition_ledger.rs (consumes typed_tx)

```rust
//! L4 Transition Ledger (CO1.7) — type skeleton + pure helpers.
//!
//! TRACE_MATRIX FC2-Append: canonical envelope appended to L4 once a transition is accepted.
//! TRACE_MATRIX WP § 5.L4: ChainTape Layer 4 spine; one LedgerEntry per accepted transition.
//! TRACE_MATRIX § 1 (CO1_7_TRANSITION_LEDGER_v1_2026-04-28 v1.1): schema + append() + replay_chain_integrity() pseudocode.
//!
//! **Status**: v1.1 type skeleton — round-1 dual audit returned CHALLENGE/CHALLENGE; this
//! version closes 11 must-fix items (C1/C2/C3 + K1-K7 + G1 + D1). Awaiting round-2.
//! All bodies that depend on yet-to-implement transition functions or CAS index
//! persistence are stubbed; full-mode replay is deferred to CO1.7.5+.
//!
//! v1 → v1.1 changes (smoke for round-2 dual audit):
//! - C1: two-mode replay enum (ChainOnly v1; FullTransition CO1.7.5+); skeleton now
//!   exposes `replay_chain_integrity` only (renamed for honesty).
//! - K1: sequencer dual-counter design — documented in spec § 3; skeleton has no
//!   sequencer code (deferred to CO1.7.5).
//! - K2: `parent_ledger_root: Hash` field added + bound in signing payload (transplant
//!   defense); new test asserts replay rejects parent_ledger_root tamper.
//! - K3: L4/L5 boundary clarified — CO1.7 owns ledger_root + commit-chain head_t;
//!   CO1.8 owns state_root mutation. Skeleton reflects boundary (no state_root mutation).
//! - K5: `TxKind::Slash` DROPPED for v4 (deferred to CO P2.5).
//! - K6: `#[repr(u8)]` + explicit discriminants on TxKind.
//! - K7: +2 conformance tests (parent_ledger_root tamper, digest exclusion).
//! - G1: `extensions: BTreeMap<String, Vec<u8>>` forward-compat field (empty in v1).
//! - C3 / Q8: signing target is `LedgerEntrySigningPayload` (separate struct) ready to
//!   ride a `CanonicalMessage::LedgerEntrySigning(_)` variant when CO1.7.5+ extends
//!   `system_keypair` (Wave 4-B additive extension). Skeleton has the payload struct
//!   + canonical_digest method; the actual CanonicalMessage extension is deferred.
//! - Q9: canonical_digest now lives on LedgerEntrySigningPayload, not LedgerEntry —
//!   structurally enforces "derivatives excluded".
//! - D1: epoch is bound in signing payload (Codex security wins over Gemini orthogonality).

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use git2::{ObjectType as Git2ObjectType, Repository, Signature as GitSignature};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::bottom_white::cas::schema::Cid;
use crate::bottom_white::ledger::system_keypair::{SystemEpoch, SystemSignature};
use crate::state::q_state::Hash;

// ────────────────────────────────────────────────────────────────────────────
// § 1 LedgerEntry — the stored record (11 fields per v1.1)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC2-Append: discriminator for the typed payload behind a CAS Cid.
/// **K6**: `#[repr(u8)]` + explicit discriminants for stable cast in canonical digest.
/// **K5**: NO `Slash` variant — ChallengeCourt slash event deferred to CO P2.5 atom.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum TxKind {
    Work            = 0,
    Verify          = 1,
    Challenge       = 2,
    Reuse           = 3,
    FinalizeReward  = 4,
    TaskExpire      = 5,
    TerminalSummary = 6,
}

/// TRACE_MATRIX FC2-Append + WP § 5.L4: stored LedgerEntry record (11 fields).
///
/// Distinct from `LedgerEntrySigningPayload`: this is the FULL stored record
/// (includes derivatives + signature); the signing payload is the subset that
/// the system keypair attests.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LedgerEntry {
    /// **K1**: assigned ONLY at commit (sequencer dual-counter design); rejected
    /// submissions never get a logical_t.
    pub logical_t: u64,                          //  1
    pub parent_state_root: Hash,                 //  2
    /// **K2 NEW**: parent_ledger_root before fold; bound in signed payload to
    /// prevent transplant attacks.
    pub parent_ledger_root: Hash,                //  3
    pub tx_kind: TxKind,                         //  4
    /// CAS handle (CO1.4) to canonical-serialized payload (DIV-5 5-param put).
    pub tx_payload_cid: Cid,                     //  5
    /// Resulting state_root post-transition (NOT mutated by L4 — accepted as
    /// returned by transition function per K3 boundary).
    pub resulting_state_root: Hash,              //  6
    /// Resulting ledger_root after fold. Derivative; NOT in signed digest.
    pub resulting_ledger_root: Hash,             //  7
    pub timestamp_logical: u64,                  //  8
    /// **D1 / Q10**: epoch bound in signed payload (Codex security wins).
    pub epoch: SystemEpoch,                      //  9
    /// **G1 NEW**: forward-compat extension map. Empty in v1; reserved for v4.x.
    /// Bound in signed payload (G1 cannot bypass signature).
    pub extensions: BTreeMap<String, Vec<u8>>,   // 10
    /// Detached system signature over `LedgerEntrySigningPayload.canonical_digest()`.
    pub system_signature: SystemSignature,       // 11
}

// ────────────────────────────────────────────────────────────────────────────
// § 1.1 LedgerEntrySigningPayload — the signed bytes (NEW per C3 / Q9)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC2-Append C3: the bytes the system keypair actually signs.
///
/// **Excludes** (Q9 cycle prevention):
/// - `resulting_ledger_root` (derivative; including → cycle)
/// - `system_signature` (its own input)
///
/// **Includes** (9 non-derivative bound fields). Domain-separation prefix is
/// part of the digest to prevent cross-namespace collision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LedgerEntrySigningPayload {
    pub logical_t: u64,
    pub parent_state_root: Hash,
    pub parent_ledger_root: Hash,                  // K2
    pub tx_kind: TxKind,
    pub tx_payload_cid: Cid,
    pub resulting_state_root: Hash,
    pub timestamp_logical: u64,
    pub epoch: SystemEpoch,                        // D1
    pub extensions: BTreeMap<String, Vec<u8>>,     // G1
}

impl LedgerEntrySigningPayload {
    /// Canonical SHA-256 digest. Stable wire format (NOT bincode/serde dependent).
    pub fn canonical_digest(&self) -> Hash {
        let mut h = Sha256::new();
        h.update(b"turingosv4.ledger_entry_signing.v1");
        h.update(self.logical_t.to_be_bytes());
        h.update(self.parent_state_root.0);
        h.update(self.parent_ledger_root.0);
        h.update((self.tx_kind as u8).to_be_bytes()); // K6 #[repr(u8)] makes cast stable
        h.update(self.tx_payload_cid.0);
        h.update(self.resulting_state_root.0);
        h.update(self.timestamp_logical.to_be_bytes());
        h.update(self.epoch.get().to_be_bytes());
        // Extensions: BTreeMap iterates in lex key order (deterministic);
        // length-prefix every field to prevent ambiguity attacks.
        h.update((self.extensions.len() as u64).to_be_bytes());
        for (k, v) in &self.extensions {
            h.update((k.len() as u64).to_be_bytes());
            h.update(k.as_bytes());
            h.update((v.len() as u64).to_be_bytes());
            h.update(v);
        }
        Hash(h.finalize().into())
    }
}

impl LedgerEntry {
    /// Project the LedgerEntry's signed-fields-subset back into a signing payload.
    /// Used by replay to recompute `signing_digest` and re-verify chain integrity.
    pub fn to_signing_payload(&self) -> LedgerEntrySigningPayload {
        LedgerEntrySigningPayload {
            logical_t: self.logical_t,
            parent_state_root: self.parent_state_root,
            parent_ledger_root: self.parent_ledger_root,
            tx_kind: self.tx_kind,
            tx_payload_cid: self.tx_payload_cid,
            resulting_state_root: self.resulting_state_root,
            timestamp_logical: self.timestamp_logical,
            epoch: self.epoch,
            extensions: self.extensions.clone(),
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────
// § 4 append() — pure ledger-root fold
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC2-Append + spec § 4: pure ledger-root fold over signed digests.
/// Same `(parent_root, signing_digest)` → byte-identical `new_root`.
/// No I/O, no clock, no env. Witness for I-DET ledger axis.
pub fn append(parent_root: &Hash, signing_digest: &Hash) -> Hash {
    let mut h = Sha256::new();
    h.update(b"turingosv4.ledger_root.v1");
    h.update(parent_root.0);
    h.update(signing_digest.0);
    Hash(h.finalize().into())
}

// ────────────────────────────────────────────────────────────────────────────
// LedgerWriter trait (K4 reconciled to skeleton signature)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC2-Append: storage abstraction for L4.
/// Production impl is `Git2LedgerWriter` (CO1.7.5+; refs/transitions/main commit chain).
/// Test/skeleton impl is `InMemoryLedgerWriter` below.
///
/// **K4**: signature `commit(&mut self) → Hash` (NOT `&self → NodeId`); `iter_from`
/// deferred to CO1.7.5+ (only used by FullTransition replay; not v1 deliverable).
pub trait LedgerWriter: Send + Sync {
    fn commit(&mut self, entry: &LedgerEntry) -> Result<Hash, LedgerWriterError>;
    fn read_at(&self, logical_t: u64) -> Result<LedgerEntry, LedgerWriterError>;
    fn len(&self) -> u64;
}

#[derive(Debug)]
pub enum LedgerWriterError {
    LogicalTGap { expected: u64, got: u64 },
    NotFound { logical_t: u64 },
    BackendCorruption(String),
}

impl std::fmt::Display for LedgerWriterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LogicalTGap { expected, got } => {
                write!(f, "logical_t gap: expected {expected}, got {got}")
            }
            Self::NotFound { logical_t } => write!(f, "no entry at logical_t={logical_t}"),
            Self::BackendCorruption(msg) => write!(f, "backend corruption: {msg}"),
        }
    }
}
impl std::error::Error for LedgerWriterError {}

/// In-memory test/skeleton writer; Vec backing strict logical_t enforced at commit.
#[derive(Debug, Default)]
pub struct InMemoryLedgerWriter {
    entries: Vec<LedgerEntry>,
}

impl InMemoryLedgerWriter {
    pub fn new() -> Self {
        Self::default()
    }
}

impl LedgerWriter for InMemoryLedgerWriter {
    fn commit(&mut self, entry: &LedgerEntry) -> Result<Hash, LedgerWriterError> {
        let expected = (self.entries.len() as u64) + 1;
        if entry.logical_t != expected {
            return Err(LedgerWriterError::LogicalTGap {
                expected,
                got: entry.logical_t,
            });
        }
        let root = entry.resulting_ledger_root;
        self.entries.push(entry.clone());
        Ok(root)
    }

    fn read_at(&self, logical_t: u64) -> Result<LedgerEntry, LedgerWriterError> {
        if logical_t == 0 || logical_t > self.entries.len() as u64 {
            return Err(LedgerWriterError::NotFound { logical_t });
        }
        Ok(self.entries[(logical_t - 1) as usize].clone())
    }

    fn len(&self) -> u64 {
        self.entries.len() as u64
    }
}

// ────────────────────────────────────────────────────────────────────────────
// § 4 replay — TWO-MODE per C1
// ────────────────────────────────────────────────────────────────────────────

/// **C1 NEW**: replay mode discriminator.
/// - `ChainOnly`: skeleton-stage; chain integrity only (parent_state_root +
///   parent_ledger_root + ledger_root chain). NOT the I-DETHASH witness.
/// - `FullTransition`: CO1.7.5+ stage; verifies signatures + re-fetches payloads
///   from CAS + re-runs pure transitions + asserts state_root match. THE
///   I-DETHASH witness; requires CO1.4-extra (CAS index persistence).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplayMode {
    ChainOnly,
    FullTransition,
}

#[derive(Debug)]
pub enum ReplayError {
    LogicalTGap { at: usize, expected: u64, got: u64 },
    ParentStateMismatch { at: usize },
    ParentLedgerMismatch { at: usize }, // K2 NEW
    LedgerRootMismatch { at: usize },
    // FullTransition-mode-only (CO1.7.5+):
    BadSignature { at: usize },
    CasMissing { at: usize },
    StateRootMismatch { at: usize },
}

impl std::fmt::Display for ReplayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LogicalTGap { at, expected, got } => {
                write!(f, "logical_t gap at index {at}: expected {expected}, got {got}")
            }
            Self::ParentStateMismatch { at } => write!(f, "parent_state_root mismatch at index {at}"),
            Self::ParentLedgerMismatch { at } => write!(f, "parent_ledger_root mismatch at index {at}"),
            Self::LedgerRootMismatch { at } => write!(f, "ledger_root mismatch at index {at}"),
            Self::BadSignature { at } => write!(f, "system_signature verify failed at index {at}"),
            Self::CasMissing { at } => write!(f, "CAS payload not retrievable at index {at}"),
            Self::StateRootMismatch { at } => write!(f, "resulting_state_root divergence at index {at}"),
        }
    }
}
impl std::error::Error for ReplayError {}

/// Skeleton-stage entry point (v1.1).
///
/// Validates:
/// 1. logical_t monotonicity (no gaps, no duplicates)
/// 2. parent_state_root chain
/// 3. parent_ledger_root chain (K2 transplant defense)
/// 4. resulting_ledger_root recomputed via append(prev_ledger_root, signing_digest)
///
/// Does NOT verify:
/// - system_signature (CO1.7.5+: requires CanonicalMessage extension wired through keypair)
/// - resulting_state_root (CO1.7.5+: requires dispatch_transition + CO1.4-extra CAS persistence)
///
/// Returns final (state_root, ledger_root) on success.
pub fn replay_chain_integrity(
    genesis_state_root: Hash,
    genesis_ledger_root: Hash,
    entries: &[LedgerEntry],
) -> Result<(Hash, Hash), ReplayError> {
    let mut prev_state_root = genesis_state_root;
    let mut prev_ledger_root = genesis_ledger_root;

    for (i, entry) in entries.iter().enumerate() {
        let expected_logical_t = (i as u64) + 1;
        if entry.logical_t != expected_logical_t {
            return Err(ReplayError::LogicalTGap {
                at: i,
                expected: expected_logical_t,
                got: entry.logical_t,
            });
        }
        if entry.parent_state_root != prev_state_root {
            return Err(ReplayError::ParentStateMismatch { at: i });
        }
        // K2 NEW: parent_ledger_root chain check
        if entry.parent_ledger_root != prev_ledger_root {
            return Err(ReplayError::ParentLedgerMismatch { at: i });
        }
        let signing_digest = entry.to_signing_payload().canonical_digest();
        let recomputed = append(&prev_ledger_root, &signing_digest);
        if recomputed != entry.resulting_ledger_root {
            return Err(ReplayError::LedgerRootMismatch { at: i });
        }
        prev_state_root = entry.resulting_state_root;
        prev_ledger_root = entry.resulting_ledger_root;
    }

    Ok((prev_state_root, prev_ledger_root))
}

// ────────────────────────────────────────────────────────────────────────────
// § 2.5 Canonical serialization (bincode v2; STATE_TRANSITION_SPEC § 2.5)
// ────────────────────────────────────────────────────────────────────────────

/// `bincode::config` used for the canonical `LedgerEntry` wire format.
///
/// **Frozen choices** (per STATE_TRANSITION_SPEC § 2.5):
/// - **Big-endian** byte order (network order; deterministic across platforms).
/// - **Fixed-int encoding** (no varint; fixed-width for byte-stable round-trip).
/// - **`BTreeMap` keys**: bincode iterates the map in serde-supplied order; we
///   only ever encode `BTreeMap` (sorted by construction) so key order is lex.
/// - **No padding, no implicit alignment.**
fn bincode_canonical_config() -> impl bincode::config::Config {
    bincode::config::standard()
        .with_big_endian()
        .with_fixed_int_encoding()
}

/// Canonical encode any serde-Serialize value to bytes (CO1.7 wire format).
/// Used by `Git2LedgerWriter` for commit-message bodies and by future callers
/// needing byte-stable signatures over typed payloads.
pub fn canonical_encode<T: Serialize>(value: &T) -> Result<Vec<u8>, CanonicalCodecError> {
    bincode::serde::encode_to_vec(value, bincode_canonical_config())
        .map_err(|e| CanonicalCodecError::Encode(e.to_string()))
}

/// Canonical decode the inverse of `canonical_encode`. Returns the value plus
/// the number of bytes consumed (entire input must be consumed for a clean decode).
pub fn canonical_decode<T: serde::de::DeserializeOwned>(
    bytes: &[u8],
) -> Result<T, CanonicalCodecError> {
    let (value, consumed) =
        bincode::serde::decode_from_slice::<T, _>(bytes, bincode_canonical_config())
            .map_err(|e| CanonicalCodecError::Decode(e.to_string()))?;
    if consumed != bytes.len() {
        return Err(CanonicalCodecError::TrailingBytes {
            consumed,
            total: bytes.len(),
        });
    }
    Ok(value)
}

#[derive(Debug)]
pub enum CanonicalCodecError {
    Encode(String),
    Decode(String),
    TrailingBytes { consumed: usize, total: usize },
}

impl std::fmt::Display for CanonicalCodecError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Encode(s) => write!(f, "canonical encode failed: {s}"),
            Self::Decode(s) => write!(f, "canonical decode failed: {s}"),
            Self::TrailingBytes { consumed, total } => {
                write!(f, "trailing bytes after decode: consumed {consumed} of {total}")
            }
        }
    }
}
impl std::error::Error for CanonicalCodecError {}

// ────────────────────────────────────────────────────────────────────────────
// § 5 Git2LedgerWriter — git2-rs commit chain on `refs/transitions/main`
// ────────────────────────────────────────────────────────────────────────────

/// Spec § 5 production storage backend.
///
/// **Mapping**:
/// - One `LedgerEntry` = one git commit on `refs/transitions/main`.
/// - **Commit tree** = three named blobs:
///     - `payload_cid`     = entry.tx_payload_cid.0 (32 bytes)
///     - `signature`       = entry.system_signature.as_bytes() (64 bytes)
///     - `entry_canonical` = bincode v2 BE + fixed-int encoding of the full
///       `LedgerEntry` (deterministic, byte-stable; this blob IS the
///       canonical record — `read_at` decodes it directly).
/// - **Commit message** = human-readable `"transition logical_t=<N>\n"` (the
///   canonical record lives in the tree blob, not the message — git
///   normalizes message bytes in ways that break round-trip).
/// - **Parent**: `head_t-1` commit (or none at genesis).
/// - **Author/committer identity**: fixed `("turingosv4 sequencer", "system@turingos")`
///   with `time = (logical_t as i64, 0)` to keep commit OIDs deterministic. NO
///   wall-clock leakage (`I-NOENV` + `I-LOGTIME`).
///
/// **K3 (revised v1.2)**: this writer surfaces `commit_oid` for callers that
/// need it (CO1.7.5+ `head_t` wiring), but the `LedgerWriter::commit` trait
/// returns only `Hash` (entry.resulting_ledger_root). Callers requesting the
/// commit OID use [`Git2LedgerWriter::head_commit_oid`] post-commit.
pub struct Git2LedgerWriter {
    repo_path: PathBuf,
    /// Last commit OID on `refs/transitions/main`; `None` at empty-chain genesis.
    head_oid: Option<git2::Oid>,
    /// Number of entries committed = highest assigned `logical_t` (0 at genesis).
    len: u64,
}

const TRANSITIONS_REF: &str = "refs/transitions/main";
const TREE_BLOB_PAYLOAD_CID: &str = "payload_cid";
const TREE_BLOB_SIGNATURE: &str = "signature";
const TREE_BLOB_ENTRY_CANONICAL: &str = "entry_canonical";

impl Git2LedgerWriter {
    /// Open or initialize a `Git2LedgerWriter` rooted at `repo_path`.
    /// Creates the underlying git repo if it doesn't exist; resolves the
    /// existing `refs/transitions/main` if present and seeds `head_oid` + `len`.
    pub fn open(repo_path: &Path) -> Result<Self, LedgerWriterError> {
        let repo_path = repo_path.to_path_buf();
        let repo = match Repository::open(&repo_path) {
            Ok(r) => r,
            Err(_) => Repository::init(&repo_path).map_err(|e| {
                LedgerWriterError::BackendCorruption(format!("repo init: {e}"))
            })?,
        };

        // Resolve refs/transitions/main if it exists.
        let (head_oid, len) = match repo.find_reference(TRANSITIONS_REF) {
            Ok(reference) => {
                let oid = reference
                    .target()
                    .ok_or_else(|| {
                        LedgerWriterError::BackendCorruption(format!(
                            "{TRANSITIONS_REF} has no direct target"
                        ))
                    })?;
                // Walk parents to count chain length.
                let mut n: u64 = 0;
                let mut cursor = Some(oid);
                while let Some(c) = cursor {
                    n += 1;
                    let commit = repo.find_commit(c).map_err(|e| {
                        LedgerWriterError::BackendCorruption(format!("walk parent: {e}"))
                    })?;
                    cursor = commit.parent(0).ok().map(|p| p.id());
                }
                (Some(oid), n)
            }
            Err(_) => (None, 0),
        };

        Ok(Self {
            repo_path,
            head_oid,
            len,
        })
    }

    fn open_repo(&self) -> Result<Repository, LedgerWriterError> {
        Repository::open(&self.repo_path)
            .map_err(|e| LedgerWriterError::BackendCorruption(format!("repo open: {e}")))
    }

    /// Commit OID of the most recent appended entry (None if chain is empty).
    /// CO1.7.5+ `head_t` wiring uses this to surface commit_sha alongside Hash.
    pub fn head_commit_oid(&self) -> Option<git2::Oid> {
        self.head_oid
    }

    /// Read raw canonical-encoded `LedgerEntry` bytes (the `entry_canonical`
    /// tree blob) for the entry at `logical_t`. `logical_t` is 1-indexed.
    fn read_canonical_bytes(&self, logical_t: u64) -> Result<Vec<u8>, LedgerWriterError> {
        if logical_t == 0 || logical_t > self.len {
            return Err(LedgerWriterError::NotFound { logical_t });
        }
        let repo = self.open_repo()?;
        // Walk back (len - logical_t) parents from head.
        let mut cursor = self.head_oid.ok_or(LedgerWriterError::NotFound { logical_t })?;
        let mut steps_back = self.len - logical_t;
        while steps_back > 0 {
            let commit = repo.find_commit(cursor).map_err(|e| {
                LedgerWriterError::BackendCorruption(format!("find_commit: {e}"))
            })?;
            cursor = commit
                .parent(0)
                .map_err(|e| LedgerWriterError::BackendCorruption(format!("parent: {e}")))?
                .id();
            steps_back -= 1;
        }
        let commit = repo
            .find_commit(cursor)
            .map_err(|e| LedgerWriterError::BackendCorruption(format!("find_commit: {e}")))?;
        let tree = commit
            .tree()
            .map_err(|e| LedgerWriterError::BackendCorruption(format!("tree: {e}")))?;
        let entry_obj = tree
            .get_name(TREE_BLOB_ENTRY_CANONICAL)
            .ok_or_else(|| {
                LedgerWriterError::BackendCorruption(format!(
                    "missing {TREE_BLOB_ENTRY_CANONICAL} blob at logical_t={logical_t}"
                ))
            })?;
        let blob = repo
            .find_blob(entry_obj.id())
            .map_err(|e| LedgerWriterError::BackendCorruption(format!("find_blob: {e}")))?;
        Ok(blob.content().to_vec())
    }
}

impl LedgerWriter for Git2LedgerWriter {
    fn commit(&mut self, entry: &LedgerEntry) -> Result<Hash, LedgerWriterError> {
        let expected = self.len + 1;
        if entry.logical_t != expected {
            return Err(LedgerWriterError::LogicalTGap {
                expected,
                got: entry.logical_t,
            });
        }

        let repo = self.open_repo()?;
        let canonical = canonical_encode(entry).map_err(|e| {
            LedgerWriterError::BackendCorruption(format!("canonical_encode: {e}"))
        })?;

        let mut tb = repo
            .treebuilder(None)
            .map_err(|e| LedgerWriterError::BackendCorruption(format!("treebuilder: {e}")))?;
        let cid_blob = repo
            .blob(&entry.tx_payload_cid.0)
            .map_err(|e| LedgerWriterError::BackendCorruption(format!("cid blob: {e}")))?;
        tb.insert(TREE_BLOB_PAYLOAD_CID, cid_blob, 0o100644)
            .map_err(|e| LedgerWriterError::BackendCorruption(format!("tree insert cid: {e}")))?;
        let sig_blob = repo
            .blob(entry.system_signature.as_bytes())
            .map_err(|e| LedgerWriterError::BackendCorruption(format!("sig blob: {e}")))?;
        tb.insert(TREE_BLOB_SIGNATURE, sig_blob, 0o100644)
            .map_err(|e| LedgerWriterError::BackendCorruption(format!("tree insert sig: {e}")))?;
        let entry_blob = repo
            .blob(&canonical)
            .map_err(|e| LedgerWriterError::BackendCorruption(format!("entry blob: {e}")))?;
        tb.insert(TREE_BLOB_ENTRY_CANONICAL, entry_blob, 0o100644)
            .map_err(|e| LedgerWriterError::BackendCorruption(format!("tree insert entry: {e}")))?;
        let tree_oid = tb
            .write()
            .map_err(|e| LedgerWriterError::BackendCorruption(format!("tree write: {e}")))?;
        let tree = repo
            .find_tree(tree_oid)
            .map_err(|e| LedgerWriterError::BackendCorruption(format!("find_tree: {e}")))?;

        // Determinism: time = (logical_t, 0). NO wall clock.
        let time = git2::Time::new(entry.logical_t as i64, 0);
        let author = GitSignature::new("turingosv4 sequencer", "system@turingos", &time)
            .map_err(|e| LedgerWriterError::BackendCorruption(format!("git sig: {e}")))?;
        let committer = author.clone();

        let parents: Vec<git2::Commit<'_>> = match self.head_oid {
            Some(oid) => vec![repo.find_commit(oid).map_err(|e| {
                LedgerWriterError::BackendCorruption(format!("parent commit: {e}"))
            })?],
            None => Vec::new(),
        };
        let parent_refs: Vec<&git2::Commit<'_>> = parents.iter().collect();
        let message = format!("transition logical_t={}\n", entry.logical_t);
        let new_oid = repo
            .commit(
                Some(TRANSITIONS_REF),
                &author,
                &committer,
                &message,
                &tree,
                &parent_refs,
            )
            .map_err(|e| LedgerWriterError::BackendCorruption(format!("commit: {e}")))?;

        self.head_oid = Some(new_oid);
        self.len += 1;
        Ok(entry.resulting_ledger_root)
    }

    fn read_at(&self, logical_t: u64) -> Result<LedgerEntry, LedgerWriterError> {
        let bytes = self.read_canonical_bytes(logical_t)?;
        canonical_decode::<LedgerEntry>(&bytes).map_err(|e| {
            LedgerWriterError::BackendCorruption(format!("canonical_decode at {logical_t}: {e}"))
        })
    }

    fn len(&self) -> u64 {
        self.len
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Tests — 8 conformance items (4 NEW vs v1 skeleton: K2 / Q9 / repr(u8) / extensions)
// ────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn h(byte: u8) -> Hash {
        Hash([byte; 32])
    }

    /// Build an entry that satisfies all chain invariants given the previous state.
    fn entry_at(
        logical_t: u64,
        parent_state_root: Hash,
        parent_ledger_root: Hash,
        resulting_state_root: Hash,
    ) -> LedgerEntry {
        let signing = LedgerEntrySigningPayload {
            logical_t,
            parent_state_root,
            parent_ledger_root,
            tx_kind: TxKind::Work,
            tx_payload_cid: Cid([0u8; 32]),
            resulting_state_root,
            timestamp_logical: logical_t,
            epoch: SystemEpoch::new(1),
            extensions: BTreeMap::new(),
        };
        let signing_digest = signing.canonical_digest();
        let resulting_ledger_root = append(&parent_ledger_root, &signing_digest);
        LedgerEntry {
            logical_t: signing.logical_t,
            parent_state_root: signing.parent_state_root,
            parent_ledger_root: signing.parent_ledger_root,
            tx_kind: signing.tx_kind,
            tx_payload_cid: signing.tx_payload_cid,
            resulting_state_root: signing.resulting_state_root,
            resulting_ledger_root,
            timestamp_logical: signing.timestamp_logical,
            epoch: signing.epoch,
            extensions: signing.extensions,
            system_signature: SystemSignature::from_bytes([0u8; 64]),
        }
    }

    // 1. append byte-stable (I-DET ledger axis)
    #[test]
    fn append_is_pure_and_byte_stable() {
        let a = append(&Hash::ZERO, &h(1));
        let b = append(&Hash::ZERO, &h(1));
        assert_eq!(a, b);
        let c = append(&Hash::ZERO, &h(2));
        assert_ne!(a, c);
    }

    // 2. canonical_digest stable (#[repr(u8)] discriminant stable)
    #[test]
    fn canonical_digest_stable_across_clones() {
        let p = LedgerEntrySigningPayload {
            logical_t: 1,
            parent_state_root: Hash::ZERO,
            parent_ledger_root: Hash::ZERO,
            tx_kind: TxKind::Work,
            tx_payload_cid: Cid([7u8; 32]),
            resulting_state_root: h(0xaa),
            timestamp_logical: 1,
            epoch: SystemEpoch::new(2),
            extensions: BTreeMap::new(),
        };
        let d1 = p.canonical_digest();
        let d2 = p.clone().canonical_digest();
        assert_eq!(d1, d2);
    }

    // 3. InMemoryWriter enforces logical_t monotonic
    #[test]
    fn in_memory_writer_enforces_logical_t() {
        let mut w = InMemoryLedgerWriter::new();
        let e1 = entry_at(1, Hash::ZERO, Hash::ZERO, h(1));
        assert!(w.commit(&e1).is_ok());

        let e_skip = entry_at(3, e1.resulting_state_root, e1.resulting_ledger_root, h(2));
        let err = w.commit(&e_skip).unwrap_err();
        assert!(matches!(err, LedgerWriterError::LogicalTGap { expected: 2, got: 3 }));
    }

    // 4. ChainOnly replay validates clean chain
    #[test]
    fn replay_chain_integrity_clean() {
        let e1 = entry_at(1, Hash::ZERO, Hash::ZERO, h(1));
        let e2 = entry_at(2, e1.resulting_state_root, e1.resulting_ledger_root, h(2));
        let e3 = entry_at(3, e2.resulting_state_root, e2.resulting_ledger_root, h(3));
        let (final_state, final_ledger) =
            replay_chain_integrity(Hash::ZERO, Hash::ZERO, &[e1.clone(), e2.clone(), e3.clone()])
                .expect("clean chain replays");
        assert_eq!(final_state, e3.resulting_state_root);
        assert_eq!(final_ledger, e3.resulting_ledger_root);
    }

    // 5. ChainOnly replay rejects parent_state_root tamper
    #[test]
    fn replay_rejects_parent_state_tamper() {
        let e1 = entry_at(1, Hash::ZERO, Hash::ZERO, h(1));
        let mut e2 = entry_at(2, e1.resulting_state_root, e1.resulting_ledger_root, h(2));
        e2.parent_state_root = h(0xff);
        let err = replay_chain_integrity(Hash::ZERO, Hash::ZERO, &[e1, e2]).unwrap_err();
        assert!(matches!(err, ReplayError::ParentStateMismatch { at: 1 }));
    }

    // 6. K2 NEW: ChainOnly replay rejects parent_ledger_root tamper (transplant defense)
    #[test]
    fn replay_rejects_parent_ledger_tamper() {
        let e1 = entry_at(1, Hash::ZERO, Hash::ZERO, h(1));
        let mut e2 = entry_at(2, e1.resulting_state_root, e1.resulting_ledger_root, h(2));
        // Tamper with parent_ledger_root WITHOUT recomputing resulting_ledger_root —
        // simulates an attacker transplanting an entry from a different ledger history.
        e2.parent_ledger_root = h(0xff);
        let err = replay_chain_integrity(Hash::ZERO, Hash::ZERO, &[e1, e2]).unwrap_err();
        assert!(matches!(err, ReplayError::ParentLedgerMismatch { at: 1 }));
    }

    // 7. ChainOnly replay rejects ledger_root tamper
    #[test]
    fn replay_rejects_ledger_root_tamper() {
        let e1 = entry_at(1, Hash::ZERO, Hash::ZERO, h(1));
        let mut e2 = entry_at(2, e1.resulting_state_root, e1.resulting_ledger_root, h(2));
        e2.resulting_ledger_root = h(0xee);
        let err = replay_chain_integrity(Hash::ZERO, Hash::ZERO, &[e1, e2]).unwrap_err();
        assert!(matches!(err, ReplayError::LedgerRootMismatch { at: 1 }));
    }

    // 8. Q9 NEW: canonical_digest excludes derivatives
    // Mutating `resulting_ledger_root` or `system_signature` of LedgerEntry must NOT
    // change the signing payload digest (because they're not in LedgerEntrySigningPayload).
    #[test]
    fn canonical_digest_excludes_derivatives() {
        let e_clean = entry_at(1, Hash::ZERO, Hash::ZERO, h(1));
        let digest_clean = e_clean.to_signing_payload().canonical_digest();

        // Mutate resulting_ledger_root (a derivative; should NOT affect signing digest)
        let mut e_tamper = e_clean.clone();
        e_tamper.resulting_ledger_root = h(0xff);
        let digest_after_root_tamper = e_tamper.to_signing_payload().canonical_digest();
        assert_eq!(
            digest_clean, digest_after_root_tamper,
            "resulting_ledger_root MUST NOT affect signing digest (Q9 cycle prevention)"
        );

        // Mutate system_signature (signature is its own input; should NOT affect signing digest)
        let mut e_tamper2 = e_clean.clone();
        e_tamper2.system_signature = SystemSignature::from_bytes([0xffu8; 64]);
        let digest_after_sig_tamper = e_tamper2.to_signing_payload().canonical_digest();
        assert_eq!(digest_clean, digest_after_sig_tamper);

        // Sanity: mutating a SIGNED field DOES change digest
        let mut e_signed_change = e_clean.clone();
        e_signed_change.epoch = SystemEpoch::new(99);
        let digest_after_signed = e_signed_change.to_signing_payload().canonical_digest();
        assert_ne!(digest_clean, digest_after_signed);
    }

    // 9. C3 closure (round-2): real signature roundtrip via system_keypair extension.
    // Verifies: (a) typed sign API works; (b) signature verifies via CanonicalMessage::LedgerEntrySigning;
    // (c) signature does NOT verify after mutating a signed field (parent_ledger_root — K2 transplant defense).
    #[test]
    fn signature_round_trip_and_transplant_defense() {
        use crate::bottom_white::ledger::system_keypair::{
            transition_ledger_emitter, CanonicalMessage, Ed25519Keypair, PinnedSystemPubkeys,
            SystemEpoch, verify_system_signature,
        };

        let keypair = Ed25519Keypair::generate_with_secure_entropy().expect("keypair gen");
        let epoch = SystemEpoch::new(1);
        let mut pinned = PinnedSystemPubkeys::new();
        pinned.insert(epoch, keypair.public_key());

        // Build a clean signing payload (e1's worth)
        let payload = LedgerEntrySigningPayload {
            logical_t: 1,
            parent_state_root: Hash::ZERO,
            parent_ledger_root: Hash::ZERO,
            tx_kind: TxKind::Work,
            tx_payload_cid: Cid([42u8; 32]),
            resulting_state_root: h(1),
            timestamp_logical: 1,
            epoch,
            extensions: BTreeMap::new(),
        };
        let digest = payload.canonical_digest();

        // Real sign through the typed CanonicalMessage extension
        let sig = transition_ledger_emitter::sign_ledger_entry(&keypair, digest.0)
            .expect("sign_ledger_entry");

        // Verify (clean) — must succeed
        let msg_clean = CanonicalMessage::LedgerEntrySigning(digest.0);
        assert!(
            verify_system_signature(&sig, &msg_clean, epoch, &pinned),
            "clean signature must verify"
        );

        // Verify (tamper parent_ledger_root) — K2 transplant defense
        let mut payload_tamper = payload.clone();
        payload_tamper.parent_ledger_root = h(0xff);
        let digest_tamper = payload_tamper.canonical_digest();
        let msg_tamper = CanonicalMessage::LedgerEntrySigning(digest_tamper.0);
        assert!(
            !verify_system_signature(&sig, &msg_tamper, epoch, &pinned),
            "transplanted parent_ledger_root MUST fail signature verify (K2)"
        );

        // Verify (cross-epoch transplant) — D1 defense via epoch IN payload digest.
        // Attacker scenario: sig was made for payload with epoch=1; attacker forges a
        // NEW payload claiming epoch=2 reusing the old sig. Since epoch is in the
        // canonical digest, digest_v2 ≠ digest_v1, so the sig on digest_v1 cannot
        // verify against digest_v2.
        let mut payload_other_epoch = payload.clone();
        payload_other_epoch.epoch = SystemEpoch::new(2);
        let digest_other_epoch = payload_other_epoch.canonical_digest();
        assert_ne!(digest, digest_other_epoch, "epoch is bound in canonical digest");
        let msg_other_epoch = CanonicalMessage::LedgerEntrySigning(digest_other_epoch.0);
        assert!(
            !verify_system_signature(&sig, &msg_other_epoch, epoch, &pinned),
            "cross-epoch transplant MUST fail signature verify (D1 epoch binding)"
        );
    }

    // ──────────────────────────────────────────────────────────────────────
    // 10–13. Git2LedgerWriter — git2-rs commit chain backend (§ 5)
    // ──────────────────────────────────────────────────────────────────────

    use tempfile::TempDir;

    fn fresh_git_writer() -> (TempDir, Git2LedgerWriter) {
        let tmp = TempDir::new().expect("tempdir");
        let w = Git2LedgerWriter::open(tmp.path()).expect("open");
        (tmp, w)
    }

    // 10. Empty repo: len()=0, head_commit_oid=None.
    #[test]
    fn git2_writer_empty_chain() {
        let (_tmp, w) = fresh_git_writer();
        assert_eq!(w.len(), 0);
        assert!(w.head_commit_oid().is_none());
    }

    // 11. Append three entries; len + head_commit_oid advance per commit;
    //     read_at recovers each entry byte-identically (canonical encode/decode round-trip).
    #[test]
    fn git2_writer_append_and_read_back() {
        let (_tmp, mut w) = fresh_git_writer();
        let e1 = entry_at(1, Hash::ZERO, Hash::ZERO, h(1));
        let e2 = entry_at(2, e1.resulting_state_root, e1.resulting_ledger_root, h(2));
        let e3 = entry_at(3, e2.resulting_state_root, e2.resulting_ledger_root, h(3));

        let r1 = w.commit(&e1).expect("commit 1");
        assert_eq!(r1, e1.resulting_ledger_root);
        assert_eq!(w.len(), 1);
        let oid_1 = w.head_commit_oid().expect("head after 1");

        let r2 = w.commit(&e2).expect("commit 2");
        assert_eq!(r2, e2.resulting_ledger_root);
        assert_eq!(w.len(), 2);
        let oid_2 = w.head_commit_oid().expect("head after 2");
        assert_ne!(oid_1, oid_2, "head must advance after second commit");

        w.commit(&e3).expect("commit 3");
        assert_eq!(w.len(), 3);

        // read_at returns each entry byte-identically.
        assert_eq!(w.read_at(1).expect("read 1"), e1);
        assert_eq!(w.read_at(2).expect("read 2"), e2);
        assert_eq!(w.read_at(3).expect("read 3"), e3);
    }

    // 12. Skipping a logical_t triggers LogicalTGap; chain state is unchanged.
    #[test]
    fn git2_writer_rejects_logical_t_gap() {
        let (_tmp, mut w) = fresh_git_writer();
        let e1 = entry_at(1, Hash::ZERO, Hash::ZERO, h(1));
        w.commit(&e1).expect("commit 1");
        let pre_oid = w.head_commit_oid();

        // Try to commit a logical_t=3 entry (gap: expected 2)
        let e_skip = entry_at(3, e1.resulting_state_root, e1.resulting_ledger_root, h(2));
        let err = w.commit(&e_skip).unwrap_err();
        assert!(matches!(err, LedgerWriterError::LogicalTGap { expected: 2, got: 3 }));
        // Chain unchanged.
        assert_eq!(w.len(), 1);
        assert_eq!(w.head_commit_oid(), pre_oid);
    }

    // 13. Reopening the same repo path resurrects the chain (head + len recovered
    //     from refs/transitions/main). Crucial for runtime cold-restart.
    #[test]
    fn git2_writer_reopen_recovers_chain() {
        let tmp = TempDir::new().expect("tempdir");
        let e1 = entry_at(1, Hash::ZERO, Hash::ZERO, h(1));
        let e2 = entry_at(2, e1.resulting_state_root, e1.resulting_ledger_root, h(2));
        let oid_after_two;
        {
            let mut w = Git2LedgerWriter::open(tmp.path()).expect("open");
            w.commit(&e1).expect("commit 1");
            w.commit(&e2).expect("commit 2");
            oid_after_two = w.head_commit_oid().expect("head");
        }
        // Reopen — fresh struct, same on-disk repo.
        let w2 = Git2LedgerWriter::open(tmp.path()).expect("reopen");
        assert_eq!(w2.len(), 2);
        assert_eq!(w2.head_commit_oid(), Some(oid_after_two));
        assert_eq!(w2.read_at(1).expect("read 1"), e1);
        assert_eq!(w2.read_at(2).expect("read 2"), e2);

        // Continue chain after reopen.
        let mut w3 = Git2LedgerWriter::open(tmp.path()).expect("reopen 2");
        let e3 = entry_at(3, e2.resulting_state_root, e2.resulting_ledger_root, h(3));
        w3.commit(&e3).expect("commit 3");
        assert_eq!(w3.len(), 3);
    }

    // 14. canonical_encode/decode round-trip for LedgerEntry (foundation of read_at).
    #[test]
    fn canonical_codec_round_trip() {
        let e1 = entry_at(7, h(0xaa), h(0xbb), h(0xcc));
        let bytes = canonical_encode(&e1).expect("encode");
        let e1_back: LedgerEntry = canonical_decode(&bytes).expect("decode");
        assert_eq!(e1, e1_back);

        // Two encodes of the same value must produce byte-identical bytes (I-DET).
        let bytes_again = canonical_encode(&e1).expect("encode again");
        assert_eq!(bytes, bytes_again);

        // Trailing garbage rejected.
        let mut bytes_extra = bytes.clone();
        bytes_extra.push(0xff);
        let err = canonical_decode::<LedgerEntry>(&bytes_extra).unwrap_err();
        assert!(matches!(err, CanonicalCodecError::TrailingBytes { .. }));
    }
}

```

---

Now give your INDEPENDENT round-2 closure-verification audit. Cite spec § + code line where possible.

codex
I’ll verify against the actual workspace rather than relying only on the pasted artifact: first the repo guidance, then the touched files, then targeted greps and the typed_tx test set.
exec
/bin/bash -lc "pwd && rg --files -g 'AGENTS.md' -g 'CLAUDE.md' -g 'Cargo.toml'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
/home/zephryj/projects/turingosv4
experiments/minif2f_v4/Cargo.toml
CLAUDE.md
Cargo.toml
spike/gix_capability/Cargo.toml

exec
/bin/bash -lc 'git status --short && git rev-parse HEAD && git log --oneline -5' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
?? handover/audits/CODEX_CO1_1_4_PRE1_ROUND2_AUDIT_2026-04-28.md
?? handover/audits/GEMINI_CO1_1_4_PRE1_ROUND2_AUDIT_2026-04-28.md
?? handover/audits/run_codex_co1_1_4_pre1_round2_audit.sh
?? handover/audits/run_gemini_co1_1_4_pre1_round2_audit.py
e0e456563c3cb921a06fa6adee48c211919659e7
e0e4565 CO1.1.4-pre1 v1.1: round-1 audit closure (10 patches integrated)
df548c5 CO1.1.4-pre1 round-1 dual audit returned: CHALLENGE/CHALLENGE → merged CHALLENGE
227de72 CO1.1.4-pre1 v1: Typed Tx ABI surface (spec + impl + tests)
a03cc52 CO1.7-impl A1: Git2LedgerWriter + canonical bincode codec
c3b00d0 CO1.7 PASS/PASS gate cleared — Wave 6 #1 spec phase COMPLETE ✅

codex
The workspace is at the claimed commit `e0e4565`. I see existing untracked round-2 audit files, so I’ll treat the tree as dirty and avoid touching anything.
exec
/bin/bash -lc "sed -n '1,240p' CLAUDE.md" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
# TuringOS v4

## What
Silicon-Native Microkernel for LLM Formal Verification Swarm.
Rust 2021, tokio, serde_json. Mission: MiniF2F Lean 4.

## Why
- 唯一对齐文档: `constitution.md` (反奥利奥架构)
- 压缩即智能: 抽象原则进宪法 / 具体情境进 `cases/`
- 机制 > 参数 > 提示 (Art. V + C-021/C-031/C-034/C-043)

## Code Standard (Art. I.1 + C-004 + C-027)
- `cargo check` / `cargo test` 必过；`.env` 永不 commit
- `src/{kernel,bus,wallet}.rs` 改动走 STEP_B_PROTOCOL（不直接编辑 main）
- 任何影响行为的参数必须 env/config 可覆盖，不可硬编码

## Audit Standard (Art. V.1 + C-010 + C-023 + C-035)
- Generator ≠ Evaluator：代码作者不可是唯一审计者
- 所有 merge / phase 决策双外审（Codex + Gemini）；VETO > CHALLENGE > PASS
- 宪法违规立即 BLOCKER，不可延期、不可"可接受"

## Report Standard (Art. I.2 + Art. II.2.1 + Art. IV 强制, C-052 + C-053 + C-057 + C-059 + C-061)
- **主指标**（每报必填）: ΣPPUT + Mean PPUT (solved) + 95% CI (Wilson)
- Art. I.2 三大统计信号不可缺: **信誉** (reputation_distribution p50/p90/max) + 效用 (PPUT) + 共识 (如适用)
- Art. IV 终态区分: `halt_reason_distribution` {OmegaAccepted, MaxTxExhausted, WallClockCap, ComputeCapViolated, ErrorHalt}
- 多 agent (n≥2) 专用: `parent_selection_entropy` + `pairwise_payload_diversity_mean`；任一 < 0.25 = Art. II.2.1 告警
- solve count 不可独立陈述，必须配对 PPUT；以 solve count 起头 = 违宪

## Reproducibility Standard (Art. I + C-012/C-016/C-032/C-039)
- OMEGA accept 必留 self-contained artifact (`proofs/*.lean` + `gp_payload`)
- 度量工具上线即冻结；Oracle 参数冻结；实验禁混 Oracle 模式
- 中间件若修改数学内容 → 是 ArchitectAI 贡献，不是 swarm 涌现（C-023）

## Alignment Standard (Art. IV + C-069)
- 权威对齐文件: `handover/alignment/TRACE_MATRIX_v0_2026-04-22.md`
  (后续 rev: `TRACE_MATRIX_vN.md`)
- 每个 src/ pub 符号必须映射到宪法 flowchart 元素、标 orphan+justification、
  或 BLOCK merge。doc-comment backlink 格式: `/// TRACE_MATRIX <FC-id>: <role>`
- Conformance tests: `tests/fc_alignment_conformance.rs` — 每个 ✅ 行 ≥1
  witness test；`#[ignore]` stub 覆盖 📅 deferred rows
- 宪法 flowchart 修改仅 human architect 可触发，需重跑 Phase Z′ 6-stage
- constitution.md hygiene 观察登记到 `handover/alignment/OBS_*.md`，不改宪法

## Common Law (宪法 + 判例)
宪法高度压缩，具体裁决查 `cases/C-xxx.yaml` (facts → ruling → precedent)
- 按条款查: `grep -l "Art. I.1" cases/*.yaml`
- 映射：`cases/V3_LESSONS.md` (50 v3 教训 → 现行判例)
- 编号跳号：C-038 / C-042 为 reserved（见 C-041/C-043 预引用）

## Docs (按需加载)
| 文档 | 何时加载 |
|------|---------|
| `docs/architecture.md` | 修改 src/ 核心模块时 |
| `docs/economics.md` | 修改经济引擎 (wallet/market) 时 |
| `docs/hardware.md` | SSH/部署/远程操作时 |
| `docs/experiments.md` | 创建或运行实验时 |
| `docs/rules.md` | 触发规则或修改规则时 |

## User
独狼研究员, 零编程基础 vibe coder. 中文为主, 技术术语英文可.

exec
/bin/bash -lc 'rg -n "CO1_1_4_PRE1|Typed Tx ABI|P1|Cross-atom ordering|Codec wording|D-4|Q-derived|Dual-sign|TerminalSummaryTx|TransitionError" handover/specs src/state src/bottom_white/ledger src/economy Cargo.toml' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
src/state/mod.rs:27:    TaskExpireTx, TaskId, TerminalSummarySigningPayload, TerminalSummaryTx, ToolId,
src/state/mod.rs:28:    TransitionError, TxStatus, TypedTx, VerifySigningPayload, VerifyTx, VerifyVerdict,
src/state/typed_tx.rs:4://! - `handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md` — this atom
src/state/typed_tx.rs:183:/// TRACE_MATRIX § 1.5 TerminalSummaryTx field 4 + Art. IV halt-reason taxonomy.
src/state/typed_tx.rs:302:    pub task_id: TaskId,                   //  3 — Q-derived authoritative; wire = ledger summary
src/state/typed_tx.rs:303:    pub solver: AgentId,                   //  4 — Q-derived authoritative; wire = ledger summary
src/state/typed_tx.rs:304:    pub reward: MicroCoin,                 //  5 — Q-derived authoritative (SettlementEngine output); wire = ledger summary
src/state/typed_tx.rs:338:pub struct TerminalSummaryTx {
src/state/typed_tx.rs:483:/// System signing payload for `TerminalSummaryTx` (8 fields → 7 fields).
src/state/typed_tx.rs:574:impl TerminalSummaryTx {
src/state/typed_tx.rs:594:/// `TerminalSummaryTx` is imported from `system_keypair.rs` (already shipped).
src/state/typed_tx.rs:603:    TerminalSummary(TerminalSummaryTx),
src/state/typed_tx.rs:669:impl HasSubmitter for TerminalSummaryTx {
src/state/typed_tx.rs:690:// TransitionError — minimal v1 taxonomy (CO1.1.4-pre1 spec § 0 out-of-scope
src/state/typed_tx.rs:701:/// summary stamping, bus rejection log). Keeping TransitionError serializable
src/state/typed_tx.rs:705:pub enum TransitionError {
src/state/typed_tx.rs:778:impl std::fmt::Display for TransitionError {
src/state/typed_tx.rs:806:impl std::error::Error for TransitionError {}
src/state/typed_tx.rs:1005:    fn fixture_terminal_summary_tx() -> TerminalSummaryTx {
src/state/typed_tx.rs:1013:        TerminalSummaryTx {
src/state/typed_tx.rs:1197:            TypedTx::TerminalSummary(TerminalSummaryTx::default()),
src/economy/mod.rs:4://! § 19. v4 ship target = 9 modules; v1 lands `money` (CO1.0a P1 prerequisite per Plan v3.2-fix3).
src/bottom_white/ledger/system_keypair.rs:178:// the typed `TerminalSummaryTx` struct (8-field per STATE § 1.5) now lives in
src/bottom_white/ledger/system_keypair.rs:230:    /// of the 8-field `state::typed_tx::TerminalSummaryTx` is computed in
src/bottom_white/ledger/system_keypair.rs:554:/// digest produced by `state::typed_tx::TerminalSummaryTx::canonical_digest()`
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:57:- **LedgerEntry schema**: canonical envelope wrapping each typed transition (WorkTx / VerifyTx / ChallengeTx / ReuseTx / FinalizeRewardTx / TaskExpireTx / TerminalSummaryTx) before append to L4. **Note**: `Slash` is NOT in v4 (deferred to CO P2.5 ChallengeCourt atom — K5).
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:67:- WorkTx / VerifyTx / ChallengeTx / ReuseTx / FinalizeRewardTx / TaskExpireTx / TerminalSummaryTx schemas — frozen in `STATE_TRANSITION_SPEC § 1`.
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:152:    TerminalSummary = 6,   // TerminalSummaryTx (STATE spec § 1.5 + § 3.7)
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:213:    TerminalSummaryTx(TerminalSummaryTx),            // existing
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:307:    fn apply_one(&self, tx: TypedTx) -> Result<LedgerEntry, TransitionError> {
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:449:- `TransitionError { at, inner }` (NEW; wraps dispatch_transition errors)
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:535:) -> Result<(QState, SignalBundle), TransitionError> {
handover/specs/SYSTEM_KEYPAIR_SECURITY_v1_2026-04-27.md:4:> **Purpose**: Gemini v3.2 review Q9 VETO — runtime system keypair (signs `RejectedAttemptSummary` + `TerminalSummaryTx`) lifecycle was unspecified; this doc defines generation, storage, rotation, threat model, and audit gates.
handover/specs/SYSTEM_KEYPAIR_SECURITY_v1_2026-04-27.md:12:D-VETO-6 / Codex CHALLENGE: failure metadata cannot be agent self-reported (wrong trust boundary). The runtime's white-box predicate runner stamps `RejectedAttemptSummary` onto next accepted `WorkTx`, and emits `TerminalSummaryTx` on no-accept runs. These stamps must be **cryptographically bound to the runtime instance**, not forgeable by any agent.
handover/specs/SYSTEM_KEYPAIR_SECURITY_v1_2026-04-27.md:102:`CanonicalMessage` is a typed enum — `RejectedAttemptSummary | TerminalSummaryTx | EpochRotationProof`. No free-form message signing exposed.
handover/specs/GENESIS_MINIMAL_WITH_ANCHOR_v1_2026-04-27.md:243:- Empty initial registries means v4 genesis has zero predicates / tools registered. Each subsequent atom (CO P1.5, CO P1.6) populates them via L4 transitions.
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:1:# CO1.1.4-pre1 — Typed Tx ABI Surface (v1.1)
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:3:**Status**: v1.1 — round-1 dual audit returned CHALLENGE/CHALLENGE; this version closes 10 patches (P1-P10) per the merged verdict (`handover/audits/CO1_1_4_PRE1_DUAL_AUDIT_VERDICT_R1_2026-04-28.md`). Awaiting round-2.
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:6:**Round-1 verdicts**: `handover/audits/CODEX_CO1_1_4_PRE1_ROUND1_AUDIT_2026-04-28.md` (CHALLENGE/high) + `handover/audits/GEMINI_CO1_1_4_PRE1_ROUND1_AUDIT_2026-04-28.md` (CHALLENGE/high); merged in `handover/audits/CO1_1_4_PRE1_DUAL_AUDIT_VERDICT_R1_2026-04-28.md`.
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:12:| **P1** | AgentSignature reused 64-byte adapter without domain separation; comments implied "exclude signature" digest with no signing payload | NEW signing-payload structs (`WorkSigningPayload` / `VerifySigningPayload` / `ChallengeSigningPayload` / `FinalizeRewardSigningPayload` / `TaskExpireSigningPayload` / `TerminalSummarySigningPayload`) — each has explicit domain prefix (`b"turingosv4.<actor>.<purpose>.v1"`) prepended to bincode body bytes in `canonical_digest()`. Plus `to_signing_payload()` projection on each tx. | C-1 (Codex Q-E + Gemini Q7) |
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:14:| **P3** | `TerminalSummaryTx` was 3-field placeholder living in `system_keypair.rs` (versus STATE § 1.5 8-field schema); locking the wrong shape into ABI | Migrated to `state::typed_tx::TerminalSummaryTx` with full 8-field STATE schema (tx_id / task_id / run_id / run_outcome / total_attempts / failure_class_histogram / last_logical_t / system_signature). `system_keypair` now signs an opaque `TerminalSummarySigning([u8; 32])` digest (same opaque-digest pattern as `LedgerEntrySigning`) — no `bottom_white ↔ state` circular dep. | C-3 (Codex Q-C must-fix-now) |
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:15:| **P4** | `TransitionError` had only 10 variants; STATE § 3 pseudocode invokes ~22 | Expanded to 22 variants: SignatureInvalid / StakeInsufficient / TargetWorkTxNotFound / TargetWorkTxNotVerifiable / ParentNotAcceptedYet / AcceptancePredicateFailed(PredicateId) / VerificationPredicateFailed(PredicateId) / SettlementPredicateFailed(PredicateId) / ChallengeWindowClosed / CounterexampleInsufficient / ToolNotInRegistry / ToolCreatorMismatch + 10 prior. Plus `NotYetImplemented` retained as explicit stub sentinel. | CX-1 (Codex Q-G) |
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:18:| **P7** | D-3 TerminalSummaryTx field-set divergence | RESOLVED (P3 migrated to full schema). § 9 D-3 row removed. | C-3 followup |
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:21:| **P10** | TaskId-vs-TxId QState index mismatch (typed_tx uses TaskId; QState `task_markets_t` / `escrows_t` / `stakes_t` keyed by TxId) | This spec § 9 NEW D-4 documents the forward-migration plan: CO P2.1 (TaskMarket atom) owns the QState retrofit; v1.1 records the migration debt + cross-atom dependency note. Does NOT perform the retrofit (out of CO1.1.4-pre1 scope; would touch q_state.rs which is its own atom). | CX-3 (Codex Q-J) |
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:28:**Why this atom exists**: spec § 2.5 of `STATE_TRANSITION_SPEC_v1_2026-04-27.md` explicitly deferred "full ABI surface for QState/SignalBundle/TransitionError" to CO1.7. CO1.7 spec § 0 places the per-kind tx schemas in `STATE_TRANSITION_SPEC § 1` ("frozen on paper, not yet in code"). When CO1.7-impl A1 (Git2LedgerWriter, commit `a03cc52`) shipped, downstream A2 (TypedTx + dispatch_transition) discovered ~30 supporting schema types are required but **none of them exist in code** — only `MicroCoin` is defined. This atom defines that ABI surface in isolation under its own dual-audit gate, per the project's per-atom audit principle (CLAUDE.md "Audit Standard").
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:47:8. **Typed-tx payload structs**: `WorkTx`, `VerifyTx`, `ChallengeTx`, `ReuseTx`, `FinalizeRewardTx`, `TaskExpireTx`. (`TerminalSummaryTx` already exists in `system_keypair.rs`.)
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:59:- **TransitionError full taxonomy** — v1 emits a minimal enum covering the variants invoked in spec § 3 pseudocode (`ClaimNotFound`, `ChallengeWindowStillOpen`, `AlreadySlashed`, `TaskNotFound`, `InvalidSystemSignature`, `StaleParent`, `TaskNotExpired`, `TaskHasOpenClaim`, `TerminalSummaryNotApplicable`, `NotYetImplemented`); per-stage enum proliferation is a CO1.7.5 concern.
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:66:### § 0.1 Cross-atom ordering gate (v1.1 NEW per Gemini Q4 round-1)
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:171:### § 4.1 Q-derived vs wire-only fields (v1.1 NEW per Codex Q-B + Gemini Q6)
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:178:If wire-stored values diverge from Q-derived values at replay, **replay rejects with `TransitionError::ClaimNotFound` or a stricter mismatch error** (CO1.7-impl A4 enforces this; CO1.7.5 transition body owns the comparison rule).
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:182:### § 4.2 Dual-sign rationale (v1.1 NEW per Gemini Q6)
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:210:    TerminalSummary(TerminalSummaryTx),  // imported from system_keypair
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:244:### § 7.1 Codec wording fix (v1.1 P6 per Codex Q-D round-1)
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:271:// FinalizeRewardTx, TaskExpireTx, TerminalSummaryTx: system-emitted; submitter_id() = None
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:283:| **D-2** | § 3.4 `FinalizeTx::from(claim_id, reward)` opaque constructor | **explicit `FinalizeRewardTx` struct** with Q-derived field discipline (§ 4.1) + dual-sign rationale (§ 4.2) | spec gap; derived schema. |
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:284:| **D-3** | ~~§ 1.5 `TerminalSummaryTx` 3-field placeholder~~ | **RESOLVED v1.1 P3**: migrated to full 8-field STATE § 1.5 schema in `state::typed_tx`; system_keypair signs opaque `TerminalSummarySigning([u8;32])` digest. |
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:285:| **D-4** (v1.1 NEW per Codex Q-J / CX-3) | QState `task_markets_t` / `escrows_t` / `stakes_t` keyed by `TxId` (q_state.rs:201/161/182) but typed_tx schemas use `TaskId` for the same task references | **NOT retrofit in this atom**. Migration owned by **CO P2.1 (TaskMarket atom)** which will rekey the QState indices to `TaskId`. CO1.1.4-pre1 documents the cross-atom debt; no wire-format consequence (the typed-tx schemas already use `TaskId` correctly per STATE § 1.2). |
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:293:| 1 | CHALLENGE (high) | CHALLENGE (high) | **CHALLENGE** | v1.1 patch round (P1-P10 above) — this version |
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:53:- Typed `QState`, `WorkTx`, `VerifyTx`, `ChallengeTx`, `RejectedAttemptSummary`, `TerminalSummaryTx` schemas
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:61:- Full predicate visibility air-gap proof — deferred to CO P1.5 (Goodhart shield design)
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:216:### 1.5 TerminalSummaryTx (no-accept run handler)
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:219:pub struct TerminalSummaryTx {
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:231:If a run terminates without any accepted work_tx, the runtime emits exactly one `TerminalSummaryTx` to L4. This preserves L6 reconstructibility: error class signal is derivable from tape even if no work_tx ever passed.
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:270:| `self.graveyard: HashMap<String, Vec<String>>` | `bus.rs:48` | **ILLEGAL sidecar** (Art. 0.2 explicitly anti-patterned) | retire; replace with `RejectedAttemptSummary` stamped on next accepted tx + `TerminalSummaryTx` |
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:318:- 1 golden tx fixture per tx type (WorkTx / VerifyTx / ChallengeTx / ReuseTx / TerminalSummaryTx); each has known input → known SHA-256 output
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:324:**Out of scope for v1.x spec** (deferred per Codex Q5/NEW-5 round-3 PARTIAL acknowledgment): full golden fixture corpus + differential fuzzing seed + complete runner ABI for QState/SignalBundle/TransitionError. v1.4 freezes the SERIALIZATION RULE (bincode v2 big-endian + BTreeMap lex); fixtures + ABI land in **CO1.1.4-pre1** (canonical fixture corpus) + **CO1.7** (full ABI surface). This is an **explicit deferral** — not unresolved spec ambiguity. STEP_B branch A and branch B both implement the SAME bincode rule; per-tx digest matching is mechanical from v1.4. Full corpus generation is a downstream code task, not spec scope.
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:338:) -> Result<(QState, SignalBundle), TransitionError> {
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:342:        return Err(TransitionError::StaleParent {
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:347:        // onto the NEXT accepted tx (or onto TerminalSummaryTx if run ends without accept)
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:352:        return Err(TransitionError::SignatureInvalid);
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:358:        return Err(TransitionError::StakeInsufficient { available: agent_balance, required: tx.stake });
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:366:            return Err(TransitionError::AcceptancePredicateFailed(acceptance_results));
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:371:            return Err(TransitionError::AcceptancePredicateFailed(acceptance_results));
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:434:) -> Result<(QState, SignalBundle), TransitionError> {
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:438:        .ok_or(TransitionError::TargetWorkTxNotFound)?;
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:440:        return Err(TransitionError::TargetWorkTxNotVerifiable);
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:445:        return Err(TransitionError::SignatureInvalid);
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:449:        return Err(TransitionError::StakeInsufficient);
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:455:        return Err(TransitionError::VerificationPredicateFailed(verify_results));
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:485:) -> Result<(QState, SignalBundle), TransitionError> {
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:489:        .ok_or(TransitionError::TargetWorkTxNotFound)?;
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:491:        .ok_or(TransitionError::ChallengeWindowClosed)?;
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:494:        return Err(TransitionError::ChallengeWindowClosed);
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:499:        return Err(TransitionError::SignatureInvalid);
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:503:        return Err(TransitionError::StakeInsufficient);
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:510:        return Err(TransitionError::CounterexampleInsufficient(counter_check));
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:582:) -> Result<(QState, SignalBundle), TransitionError> {
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:585:        .ok_or(TransitionError::ToolNotInRegistry)?;
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:587:        return Err(TransitionError::ToolCreatorMismatch);
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:592:        .ok_or(TransitionError::TargetWorkTxNotFound)?;
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:594:        return Err(TransitionError::ParentNotAcceptedYet);
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:640:) -> Result<(QState, SignalBundle), TransitionError> {
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:642:        .ok_or(TransitionError::ClaimNotFound)?;
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:650:            return Err(TransitionError::ChallengeWindowStillOpen);
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:653:            return Err(TransitionError::AlreadySlashed);  // never finalize a slashed claim
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:718:) -> Result<(QState, SignalBundle), TransitionError> {
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:720:        .ok_or(TransitionError::TaskNotFound)?;
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:724:        return Err(TransitionError::InvalidSystemSignature);
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:729:        return Err(TransitionError::StaleParent);
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:735:        return Err(TransitionError::TaskNotExpired);
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:738:        return Err(TransitionError::TaskHasOpenClaim);    // refund only if NO claim exists at all
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:832:) -> Result<(QState, SignalBundle), TransitionError> {
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:835:        return Err(TransitionError::TerminalSummaryNotApplicable);  // only emitted for no-accept runs
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:838:    let summary = TerminalSummaryTx {
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:880:| I-TERMINAL | every run terminates with at least one of: accepted work_tx OR TerminalSummaryTx | run finalize hook | `tests/run_terminal_invariant.rs` |
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:927:If CO P1 audit demands stronger guarantees, the TLA+ model is upgraded to a full PlusCal program with TLC model checking. For v4 scope, the type-level + conformance-test combination is deemed sufficient by Codex.
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:948:**Why**: spec § 3 pseudocode is single-threaded; CO P1 may parallelize Phase C 5 modes × N seeds. Without explicit serialization rule, two work_tx can race on same parent_state_root, both pass `I-PARENT`, but produce different (logical_t, tx_id) sequences across STEP_B branches → state_root divergence.
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:1020:The current pre-CO-P1 codebase contains economic mutation surfaces in `src/bus.rs` and `src/kernel.rs` that have NO direct equivalent in v1.x typed transitions:
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:1028:| `RunEnd` / `halt_and_settle` (run-level settlement) | `src/bus.rs:355-375` `TuringBus::halt_and_settle` | **RETIRED in CO1.1.4** — run-end becomes implicit via `TerminalSummaryTx` (§ 3.7) for no-accept runs OR `finalize_reward_transition` for accepted runs |
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:1042:3. **Predicate visibility leak channels** — covered at CO P1.5 design (Goodhart shield); this spec only declares `BoolWithProof.proof_visibility_class`, not the leak-proof proof format.
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:1044:5. **Retry metadata bound on `failed_attempts_since_last_accept`** — must be finite for tape size containment, but exact bound (e.g., u32::MAX vs cap-at-1000) is CO P1.7 design choice.
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:1052:## § 7 Pre-CO P1 Gate Procedure
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:1078:- Whether `RejectionClass::Opaque` aggregation respects Goodhart shield in practice (deferred to CO P1.5)
handover/specs/CO1_3_1_GIX_SPIKE_PREFLIGHT_v1_2026-04-27.md:5:> **Authority**: Constitution Art 0.4 (Path B = real git substrate). Plan v3.2 CO1.3.1 (FIRST CO P1 atom).
handover/specs/CO1_3_1_GIX_SPIKE_PREFLIGHT_v1_2026-04-27.md:121:Gemini audit prompt: "Strategic check: does the spike result demonstrate gix is fit for purpose for the *entire* CO P1 timeline? Are there capabilities outside C1-C8 that v4 will need but the spike didn't test (e.g., remotes, gc, refs/notes for trace_matrix)?"
handover/specs/PRE_COMMIT_HOOKS_R022_R023_v1_2026-04-27.md:94:        echo "  This is permitted in CO P0/P1.* atoms (matrix populates incrementally)" >&2
handover/specs/PRE_COMMIT_HOOKS_R022_R023_v1_2026-04-27.md:95:        echo "  CO P1.13 atom enforces strict update; until then this is a warning only" >&2
handover/specs/ART_0_2_REINTERPRETATION_2026-04-27.md:61:- All downstream code (CO1.7+CO1.9 retry metadata + TerminalSummaryTx) implements Reading Y
handover/specs/ART_0_2_REINTERPRETATION_2026-04-27.md:69:> 5. 失败信号必须可从 tape 重建（"失败也是状态"）。具体实现可以是：每个 reject 单独成 Node，或在下一条接受的 work_tx 上 stamp 系统签名的 bounded `RejectedAttemptSummary`（系统不是 agent 自报），或在零 accept 的 run 末尾 emit `TerminalSummaryTx`。原 `bus.graveyard: HashMap` 设计是 anti-pattern（不能从 tape 重建）；上述任一系统签名机制均合规。Phase B 之前所有平行账本同样违规。
handover/specs/ART_0_2_REINTERPRETATION_2026-04-27.md:102:2. `TerminalSummaryTx` (per spec § 1.5) handles no-accept runs
handover/specs/STATE_TRANSITION_SPEC_TLA_2026-04-27.tla:9:\*         + run TLC if any CO P1 audit demands stronger guarantees beyond the
handover/specs/STATE_TRANSITION_SPEC_TLA_2026-04-27.tla:227:\* To run TLC model check (when CO P1 audit demands it):
handover/specs/STATE_TRANSITION_SPEC_TLA_2026-04-27.tla:247:\* - VerifyTx / ChallengeTx / ReuseTx / FinalizeRewardTx / TerminalSummaryTx
handover/specs/STATE_TRANSITION_SPEC_TLA_2026-04-27.tla:253:\* - CO P1 audit explicitly demands stronger ordering proofs
handover/specs/STATE_TRANSITION_SPEC_TLA_2026-04-27.tla:255:\* - A specific bug in CO P1 implementation exposes a non-obvious ordering issue
handover/specs/SPEC_WALKTHROUGH_v1_2026-04-27.md:235:→ `Err(TransitionError::CounterexampleInsufficient)` returned
handover/specs/SPEC_WALKTHROUGH_v1_2026-04-27.md:367:## § 9 Step 8 — TerminalSummaryTx: scenario where Alice never gets accepted
handover/specs/SPEC_WALKTHROUGH_v1_2026-04-27.md:375:Runtime detects: no accepted work_tx for `run_id`. Emits `TerminalSummaryTx`:
handover/specs/SPEC_WALKTHROUGH_v1_2026-04-27.md:378:terminal_summary_tx = TerminalSummaryTx {
handover/specs/SPEC_WALKTHROUGH_v1_2026-04-27.md:402:| I-TERMINAL | run terminated with TerminalSummaryTx ✓ |
handover/specs/SPEC_WALKTHROUGH_v1_2026-04-27.md:428:| I-RETRY | system-stamped retry summary in TerminalSummaryTx | ✓ |
handover/specs/SPEC_WALKTHROUGH_v1_2026-04-27.md:429:| I-TERMINAL | step 9 emitted TerminalSummaryTx for no-accept run | ✓ |
handover/specs/SPEC_WALKTHROUGH_v1_2026-04-27.md:462:**Action**: spec § 3.2 needs an extra stage 4e bullet explicitly handling verifier bonds; defer fix to CO P1.SPEC.0 spec revision before code.
handover/specs/SPEC_WALKTHROUGH_v1_2026-04-27.md:468:**Action**: spec § 3.3 needs explicit bound; add to CO P1.SPEC.0 revision.
handover/specs/SPEC_WALKTHROUGH_v1_2026-04-27.md:493:These are deferred; CO P1+P2 sprint should generate at least one walk-through per atom delivered.
handover/specs/SPEC_WALKTHROUGH_v1_2026-04-27.md:508:- Found gaps trigger spec revisions (CO P1.SPEC.0); they are not fatal but must be closed before code

exec
/bin/bash -lc 'find .. -name AGENTS.md -print' in /home/zephryj/projects/turingosv4
 succeeded in 1257ms:
../turingos/AGENTS.md
../omega_kepler_discovery/AGENTS.md
../omega/AGENTS.md
../turingosv2/AGENTS.md
../turingclaw/AGENTS.md

exec
/bin/bash -lc 'for f in /AGENTS.md /home/AGENTS.md /home/zephryj/AGENTS.md /home/zephryj/projects/AGENTS.md /home/zephryj/projects/turingosv4/AGENTS.md; do if [ -f "$f" ]; then printf '"'%s\\n' \""'$f"; fi; done' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
exec
/bin/bash -lc "nl -ba src/state/typed_tx.rs | sed -n '1,260p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	//! Typed transaction ABI surface — CO1.1.4-pre1.
     2	//!
     3	//! Spec authority:
     4	//! - `handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md` — this atom
     5	//! - `STATE_TRANSITION_SPEC_v1_2026-04-27.md` § 1 (typed schemas), § 2.5
     6	//!   (canonical serialization), § 3 (transition pseudocode used to derive
     7	//!   FinalizeRewardTx schema in spec § 4)
     8	//!
     9	//! Why this module exists: when CO1.7-impl A1 (Git2LedgerWriter) shipped, the
    10	//! downstream A2 (Sequencer + `dispatch_transition`) needed a `TypedTx` enum
    11	//! whose variants carry per-kind tx structs. Those structs and ~20 supporting
    12	//! types (identifiers, signatures, predicate-result types, status enums) were
    13	//! "frozen on paper" in STATE_TRANSITION_SPEC § 1 but had no Rust definition.
    14	//! CO1.1.4-pre1 lands them in isolation under its own dual-audit gate,
    15	//! per the project's per-atom audit principle (CLAUDE.md "Audit Standard").
    16	//!
    17	//! /// TRACE_MATRIX FC2-Submit + § 1 typed schemas: typed-tx ABI surface.
    18	
    19	use serde::{Deserialize, Serialize};
    20	use std::collections::{BTreeMap, BTreeSet};
    21	
    22	use sha2::{Digest, Sha256};
    23	
    24	use crate::bottom_white::cas::schema::Cid;
    25	use crate::bottom_white::ledger::system_keypair::{serde_bytes_64, SystemEpoch, SystemSignature};
    26	use crate::economy::money::{MicroCoin, StakeMicroCoin};
    27	use crate::state::q_state::{AgentId, Hash, TxId};
    28	
    29	// ────────────────────────────────────────────────────────────────────────────
    30	// § 2 Identifier newtypes (all opaque strings to Q_t)
    31	// ────────────────────────────────────────────────────────────────────────────
    32	
    33	/// TRACE_MATRIX § 1.2 — task-market entry id; opaque string.
    34	#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
    35	pub struct TaskId(pub String);
    36	
    37	/// TRACE_MATRIX § 1.5 — runtime run id (one run per `Sequencer` driver lifecycle).
    38	#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
    39	pub struct RunId(pub String);
    40	
    41	/// TRACE_MATRIX STATE § 3.4 + § 4 I-FINALIZE-BATCH-ORDER — typed claim id used
    42	/// in `FinalizeRewardTx.claim_id` and `ClaimsIndex` keying. Wraps `TxId`
    43	/// (the underlying claim is recorded against the work_tx's TxId in
    44	/// ClaimsIndex per current QState shape) but **prevents accidental mixing
    45	/// of claim references with arbitrary transaction references** at the type
    46	/// level (Codex round-1 Q-B CHALLENGE).
    47	///
    48	/// `#[serde(transparent)]` — wire-identical to TxId, so adoption is
    49	/// non-breaking for canonical encoding.
    50	#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
    51	#[serde(transparent)]
    52	pub struct ClaimId(pub TxId);
    53	
    54	impl ClaimId {
    55	    pub fn new(s: impl Into<String>) -> Self {
    56	        Self(TxId(s.into()))
    57	    }
    58	    pub fn as_tx_id(&self) -> &TxId {
    59	        &self.0
    60	    }
    61	}
    62	
    63	/// TRACE_MATRIX § 1.3 ReuseTx + L2 Tool Registry — opaque tool identifier.
    64	#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
    65	pub struct ToolId(pub String);
    66	
    67	/// TRACE_MATRIX § 1.2 PredicateResultsBundle + L1 Predicate Registry — opaque predicate id.
    68	#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
    69	pub struct PredicateId(pub String);
    70	
    71	/// TRACE_MATRIX § 1.2 WorkTx field 5 — read-set key (DAG attribution / replay).
    72	/// Kept as opaque string in v1; stricter typing (path / tape-coordinate) lands
    73	/// in CO P2.4.0 attribution-engine spike.
    74	#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
    75	pub struct ReadKey(pub String);
    76	
    77	/// TRACE_MATRIX § 1.2 WorkTx field 6 — write-set key (DAG attribution / replay).
    78	#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
    79	pub struct WriteKey(pub String);
    80	
    81	// ────────────────────────────────────────────────────────────────────────────
    82	// § 3 AgentSignature (Ed25519 [u8;64], type-distinct from SystemSignature)
    83	// ────────────────────────────────────────────────────────────────────────────
    84	
    85	/// TRACE_MATRIX § 1.2 WorkTx field 10 + I-SIG: agent-side detached Ed25519
    86	/// signature over the per-tx canonical_digest. Distinct type from
    87	/// `SystemSignature` to prevent accidental confusion at API boundaries
    88	/// (Codex sec-arg: agent-vs-system signature mixing is a real hazard).
    89	#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    90	pub struct AgentSignature(#[serde(with = "serde_bytes_64")] [u8; 64]);
    91	
    92	impl AgentSignature {
    93	    pub const fn from_bytes(bytes: [u8; 64]) -> Self {
    94	        Self(bytes)
    95	    }
    96	    pub const fn as_bytes(&self) -> &[u8; 64] {
    97	        &self.0
    98	    }
    99	}
   100	
   101	impl Default for AgentSignature {
   102	    fn default() -> Self {
   103	        Self([0u8; 64])
   104	    }
   105	}
   106	
   107	// ────────────────────────────────────────────────────────────────────────────
   108	// § 3 SlashEvidenceCid (newtype; type-distinct slash-evidence reference)
   109	// ────────────────────────────────────────────────────────────────────────────
   110	
   111	/// TRACE_MATRIX § 1.2 TxStatus::FinalizedSlash — typed reference to the
   112	/// counter-example payload that justified the slash (lives in L3 CAS).
   113	#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
   114	#[serde(transparent)]
   115	pub struct SlashEvidenceCid(pub Cid);
   116	
   117	// ────────────────────────────────────────────────────────────────────────────
   118	// § 4 Predicate result types
   119	// ────────────────────────────────────────────────────────────────────────────
   120	
   121	/// TRACE_MATRIX § 1.2 PredicateResultsBundle — boolean predicate verdict
   122	/// optionally accompanied by an L3 CAS reference to the proof object
   123	/// (e.g. Lean witness, ZK proof bytes).
   124	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   125	pub struct BoolWithProof {
   126	    pub value: bool,
   127	    pub proof_cid: Option<Cid>,
   128	}
   129	
   130	/// TRACE_MATRIX § 1.2 PredicateResultsBundle — safety-class discriminator.
   131	/// Determines fail-closed (Safety) vs fail-open-with-signal (Creation) behavior
   132	/// when a predicate's evaluation errors. Frozen STATE spec § 1.2.
   133	#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
   134	#[repr(u8)]
   135	pub enum SafetyOrCreation {
   136	    Safety = 0,
   137	    Creation = 1,
   138	}
   139	
   140	impl Default for SafetyOrCreation {
   141	    fn default() -> Self {
   142	        // Safety bias by default: fail-closed if no class declared.
   143	        Self::Safety
   144	    }
   145	}
   146	
   147	/// TRACE_MATRIX § 1.2 WorkTx field 8 — runner-stamped predicate results
   148	/// (acceptance + settlement gates) with explicit safety-class discriminator.
   149	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   150	pub struct PredicateResultsBundle {
   151	    pub acceptance: BTreeMap<PredicateId, BoolWithProof>,
   152	    pub settlement: BTreeMap<PredicateId, BoolWithProof>,
   153	    pub safety_class: SafetyOrCreation,
   154	}
   155	
   156	// ────────────────────────────────────────────────────────────────────────────
   157	// § 5 Status / class enums (RejectionClass, VerifyVerdict, RunOutcome,
   158	//                          and the runtime-only TxStatus per D-1)
   159	// ────────────────────────────────────────────────────────────────────────────
   160	
   161	/// TRACE_MATRIX § 1.4 — classification of a rejected attempt.
   162	/// Public predicates are classified concretely; private predicates surface as
   163	/// `Opaque` (no information leakage to attacker).
   164	#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
   165	pub enum RejectionClass {
   166	    AcceptancePredicateFail(PredicateId),
   167	    SettlementPredicateFail(PredicateId),
   168	    StakeInsufficient,
   169	    SignatureInvalid,
   170	    StaleParentRoot,
   171	    Opaque,
   172	    BudgetExceeded,
   173	}
   174	
   175	/// TRACE_MATRIX § 1.3 VerifyTx field 5 — verifier verdict.
   176	#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
   177	#[repr(u8)]
   178	pub enum VerifyVerdict {
   179	    Confirm = 0,
   180	    Doubt = 1,
   181	}
   182	
   183	/// TRACE_MATRIX § 1.5 TerminalSummaryTx field 4 + Art. IV halt-reason taxonomy.
   184	/// Five-way partition over how a run terminates.
   185	#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
   186	#[repr(u8)]
   187	pub enum RunOutcome {
   188	    OmegaAccepted = 0,
   189	    MaxTxExhausted = 1,
   190	    WallClockCap = 2,
   191	    ComputeCap = 3,
   192	    ErrorHalt = 4,
   193	}
   194	
   195	/// TRACE_MATRIX § 1.2 TxStatus — **runtime book-keeping only** (D-1 divergence
   196	/// from STATE spec): never serialized into a TypedTx variant's wire bytes.
   197	/// Tracked in `q_t.q_t.agents[id].last_accepted_tx` + `ClaimsIndex`. Exposed
   198	/// here as a public type for the runtime API surface.
   199	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
   200	pub enum TxStatus {
   201	    Pending,
   202	    Accepted,
   203	    Rejected(RejectionClass),
   204	    FinalizedReward(MicroCoin),
   205	    FinalizedSlash(SlashEvidenceCid),
   206	}
   207	
   208	// ────────────────────────────────────────────────────────────────────────────
   209	// § 5 (cont'd) — Typed tx structs (STATE spec § 1.2-1.6 + § 3.6)
   210	// ────────────────────────────────────────────────────────────────────────────
   211	
   212	/// TRACE_MATRIX § 1.2 — agent-submitted work transaction (12-field schema;
   213	/// **D-1 divergence**: field 12 `status: TxStatus` is excluded from canonical
   214	/// wire bytes — TxStatus is runner book-keeping per CO1.1.4-pre1 spec § 5).
   215	///
   216	/// This is the per-tx struct that the CO1.7 sequencer hands to
   217	/// `step_transition` (CO1.7.5 body atom). The `signature` is over
   218	/// `canonical_digest(&work_tx)` where the digest pre-image excludes the
   219	/// signature itself (its own input).
   220	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   221	pub struct WorkTx {
   222	    pub tx_id: TxId,                                  //  1
   223	    pub task_id: TaskId,                              //  2
   224	    pub parent_state_root: Hash,                      //  3
   225	    pub agent_id: AgentId,                            //  4
   226	    pub read_set: BTreeSet<ReadKey>,                  //  5
   227	    pub write_set: BTreeSet<WriteKey>,                //  6
   228	    pub proposal_cid: Cid,                            //  7
   229	    pub predicate_results: PredicateResultsBundle,    //  8 (runner-stamped)
   230	    pub stake: StakeMicroCoin,                        //  9
   231	    pub signature: AgentSignature,                    // 10
   232	    pub timestamp_logical: u64,                       // 11
   233	    // 12: TxStatus — D-1 elision; runtime-only.
   234	}
   235	
   236	/// TRACE_MATRIX § 1.3 — verifier verdict transaction.
   237	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   238	pub struct VerifyTx {
   239	    pub tx_id: TxId,                       //  1
   240	    pub target_work_tx: TxId,              //  2
   241	    pub verifier_agent: AgentId,           //  3
   242	    pub bond: StakeMicroCoin,              //  4
   243	    pub verdict: VerifyVerdict,            //  5
   244	    pub signature: AgentSignature,         //  6
   245	    pub timestamp_logical: u64,            //  7
   246	}
   247	
   248	impl Default for VerifyVerdict {
   249	    fn default() -> Self {
   250	        Self::Confirm
   251	    }
   252	}
   253	
   254	/// TRACE_MATRIX § 1.3 — challenge transaction (counter-example posted with
   255	/// stake at risk).
   256	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   257	pub struct ChallengeTx {
   258	    pub tx_id: TxId,                       //  1
   259	    pub target_work_tx: TxId,              //  2
   260	    pub challenger_agent: AgentId,         //  3

exec
/bin/bash -lc "nl -ba src/state/typed_tx.rs | sed -n '260,620p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   260	    pub challenger_agent: AgentId,         //  3
   261	    pub stake: StakeMicroCoin,             //  4
   262	    pub counterexample_cid: Cid,           //  5
   263	    pub signature: AgentSignature,         //  6
   264	    pub timestamp_logical: u64,            //  7
   265	}
   266	
   267	/// TRACE_MATRIX § 1.3 — fact-tx recording reuse of a tool created by a prior
   268	/// agent (royalty graph edge). No submitting agent (per § 3.6.5 v1.3).
   269	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   270	pub struct ReuseTx {
   271	    pub tx_id: TxId,                       //  1
   272	    pub reusing_work_tx: TxId,             //  2
   273	    pub reused_tool_id: ToolId,            //  3
   274	    pub reused_tool_creator: AgentId,      //  4
   275	    pub timestamp_logical: u64,            //  5
   276	}
   277	
   278	/// TRACE_MATRIX CO1.1.4-pre1 spec § 4 — derived schema (STATE spec § 3.4
   279	/// uses opaque `FinalizeTx::from(claim_id, reward)` constructor without an
   280	/// explicit struct definition).
   281	///
   282	/// **v1.1 round-1 audit closures**:
   283	/// - **C-3 (Codex Q-B)**: `claim_id` is now a typed `ClaimId` newtype (was
   284	///   bare `TxId`) — STATE § 4 I-FINALIZE-BATCH-ORDER speaks in claim_id;
   285	///   reusing TxId leaked QState implementation into the wire format.
   286	/// - **C-3 (Codex Q-B)**: `task_id` / `solver` / `reward` are documented as
   287	///   **Q-DERIVED at replay** — replay (CO1.7-impl A4) re-fetches them from
   288	///   ClaimsIndex by `claim_id`, NOT trusted from wire. Wire fields are kept
   289	///   as a ledger summary (so a human reading L4 can see the finalize event
   290	///   semantics) but the AUTHORITATIVE values come from Q_t.
   291	/// - **C-3 / GM-2 followup**: `system_signature` is RETAINED for v1.1 — it
   292	///   binds the system-emitted FinalizeRewardTx to a specific runtime keypair
   293	///   epoch (auditability + cross-cell trust). The CO1.7 `LedgerEntry`
   294	///   wraps this struct via CAS reference + signs the `LedgerEntrySigningPayload`
   295	///   digest; the two sigs are NOT redundant: this one binds the tx-payload
   296	///   bytes; the L4 envelope sig binds the (logical_t, parent_ledger_root, tx_payload_cid)
   297	///   sequencer-stamped envelope. v1.1 spec § 4 makes the dual-sign rationale explicit.
   298	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   299	pub struct FinalizeRewardTx {
   300	    pub tx_id: TxId,                       //  1
   301	    pub claim_id: ClaimId,                 //  2 — typed (was TxId in v1)
   302	    pub task_id: TaskId,                   //  3 — Q-derived authoritative; wire = ledger summary
   303	    pub solver: AgentId,                   //  4 — Q-derived authoritative; wire = ledger summary
   304	    pub reward: MicroCoin,                 //  5 — Q-derived authoritative (SettlementEngine output); wire = ledger summary
   305	    pub parent_state_root: Hash,           //  6
   306	    pub epoch: SystemEpoch,                //  7
   307	    pub timestamp_logical: u64,            //  8
   308	    pub system_signature: SystemSignature, //  9 — see doc-comment on dual-sign rationale
   309	}
   310	
   311	/// TRACE_MATRIX STATE spec § 3.6 v1.3 — system-emitted task-expiry tx
   312	/// (refunds bounty + locked stakes when no claim finalized by deadline).
   313	/// Verbatim transcription.
   314	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   315	pub struct TaskExpireTx {
   316	    pub tx_id: TxId,                       //  1
   317	    pub task_id: TaskId,                   //  2
   318	    pub parent_state_root: Hash,           //  3
   319	    pub bounty_refunded: MicroCoin,        //  4 (computed by runtime; included for ledger summary)
   320	    pub epoch: SystemEpoch,                //  5
   321	    pub timestamp_logical: u64,            //  6
   322	    pub system_signature: SystemSignature, //  7
   323	}
   324	
   325	/// TRACE_MATRIX STATE spec § 1.5 — system-emitted no-accept-run handler.
   326	/// Emitted exactly once if a run terminates without any accepted work_tx, so
   327	/// L6 reconstructibility (failure-class signal) is preserved on the tape
   328	/// even when no work_tx ever passed.
   329	///
   330	/// **v1.1 round-1 audit closure (C-3 Codex Q-C must-fix-now)**: replaces the
   331	/// 3-field placeholder previously living in `system_keypair.rs`. Full
   332	/// 8-field schema per STATE § 1.5. The signer (`system_keypair`) now signs
   333	/// an opaque `TerminalSummarySigning([u8; 32])` digest — same opaque-digest
   334	/// pattern as `LedgerEntrySigning` — so the canonical_digest is computed
   335	/// here and `system_keypair` stays oblivious to the typed-tx schema (no
   336	/// circular `bottom_white ↔ state` dependency).
   337	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   338	pub struct TerminalSummaryTx {
   339	    pub tx_id: TxId,                                          //  1
   340	    pub task_id: TaskId,                                      //  2
   341	    pub run_id: RunId,                                        //  3
   342	    pub run_outcome: RunOutcome,                              //  4
   343	    pub total_attempts: u32,                                  //  5
   344	    pub failure_class_histogram: BTreeMap<RejectionClass, u32>,// 6
   345	    pub last_logical_t: u64,                                  //  7
   346	    pub system_signature: SystemSignature,                    //  8
   347	}
   348	
   349	impl Default for RunOutcome {
   350	    fn default() -> Self {
   351	        Self::OmegaAccepted
   352	    }
   353	}
   354	
   355	// ────────────────────────────────────────────────────────────────────────────
   356	// § 7 Signing payloads (CO1.1.4-pre1 v1.1 round-1 closure C-1)
   357	//
   358	// Each agent-signed and system-emitted typed-tx has a paired `*SigningPayload`
   359	// struct (subset of fields, EXCLUDES the signature itself) with a
   360	// `canonical_digest()` method that **prepends a stable domain-separation
   361	// prefix** before the bincode-canonical body bytes. This implements:
   362	//
   363	//   sig_input = sha256(b"turingosv4.<actor>.<purpose>.v1" || canonical_encode(payload))
   364	//
   365	// Property: even if two distinct payload TYPES happen to bincode-encode to
   366	// identical bytes (extremely unlikely given distinct field shapes, but
   367	// defensively guaranteed), the domain prefix ensures the SHA-256 inputs
   368	// differ. Closes Codex Q-E + Gemini Q7: type-level distinction is necessary
   369	// but not sufficient as a security boundary.
   370	//
   371	// **Forward dependency**: actual `verify_agent_signature(sig, payload, agent_pubkey)`
   372	// + agent-pubkey-registry lookup is CO P2.x AgentRegistry territory; this
   373	// atom only freezes the canonical_digest pre-image.
   374	// ────────────────────────────────────────────────────────────────────────────
   375	
   376	const DOMAIN_AGENT_WORK: &[u8] = b"turingosv4.agent_sig.work.v1";
   377	const DOMAIN_AGENT_VERIFY: &[u8] = b"turingosv4.agent_sig.verify.v1";
   378	const DOMAIN_AGENT_CHALLENGE: &[u8] = b"turingosv4.agent_sig.challenge.v1";
   379	const DOMAIN_SYSTEM_FINALIZE_REWARD: &[u8] = b"turingosv4.system_sig.finalize_reward.v1";
   380	const DOMAIN_SYSTEM_TASK_EXPIRE: &[u8] = b"turingosv4.system_sig.task_expire.v1";
   381	const DOMAIN_SYSTEM_TERMINAL_SUMMARY: &[u8] = b"turingosv4.system_sig.terminal_summary.v1";
   382	
   383	fn domain_prefixed_digest<T: Serialize>(domain: &[u8], value: &T) -> [u8; 32] {
   384	    use crate::bottom_white::ledger::transition_ledger::canonical_encode;
   385	    let body = canonical_encode(value).expect("canonical_encode of signing payload");
   386	    let mut h = Sha256::new();
   387	    h.update(domain);
   388	    h.update(&body);
   389	    h.finalize().into()
   390	}
   391	
   392	/// Agent signing payload for `WorkTx` (12 fields → 11 fields; signature excluded).
   393	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   394	pub struct WorkSigningPayload {
   395	    pub tx_id: TxId,
   396	    pub task_id: TaskId,
   397	    pub parent_state_root: Hash,
   398	    pub agent_id: AgentId,
   399	    pub read_set: BTreeSet<ReadKey>,
   400	    pub write_set: BTreeSet<WriteKey>,
   401	    pub proposal_cid: Cid,
   402	    pub predicate_results: PredicateResultsBundle,
   403	    pub stake: StakeMicroCoin,
   404	    pub timestamp_logical: u64,
   405	}
   406	
   407	impl WorkSigningPayload {
   408	    pub fn canonical_digest(&self) -> [u8; 32] {
   409	        domain_prefixed_digest(DOMAIN_AGENT_WORK, self)
   410	    }
   411	}
   412	
   413	/// Agent signing payload for `VerifyTx` (7 fields → 6 fields).
   414	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   415	pub struct VerifySigningPayload {
   416	    pub tx_id: TxId,
   417	    pub target_work_tx: TxId,
   418	    pub verifier_agent: AgentId,
   419	    pub bond: StakeMicroCoin,
   420	    pub verdict: VerifyVerdict,
   421	    pub timestamp_logical: u64,
   422	}
   423	
   424	impl VerifySigningPayload {
   425	    pub fn canonical_digest(&self) -> [u8; 32] {
   426	        domain_prefixed_digest(DOMAIN_AGENT_VERIFY, self)
   427	    }
   428	}
   429	
   430	/// Agent signing payload for `ChallengeTx` (7 fields → 6 fields).
   431	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   432	pub struct ChallengeSigningPayload {
   433	    pub tx_id: TxId,
   434	    pub target_work_tx: TxId,
   435	    pub challenger_agent: AgentId,
   436	    pub stake: StakeMicroCoin,
   437	    pub counterexample_cid: Cid,
   438	    pub timestamp_logical: u64,
   439	}
   440	
   441	impl ChallengeSigningPayload {
   442	    pub fn canonical_digest(&self) -> [u8; 32] {
   443	        domain_prefixed_digest(DOMAIN_AGENT_CHALLENGE, self)
   444	    }
   445	}
   446	
   447	/// System signing payload for `FinalizeRewardTx` (9 fields → 8 fields).
   448	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   449	pub struct FinalizeRewardSigningPayload {
   450	    pub tx_id: TxId,
   451	    pub claim_id: ClaimId,
   452	    pub task_id: TaskId,
   453	    pub solver: AgentId,
   454	    pub reward: MicroCoin,
   455	    pub parent_state_root: Hash,
   456	    pub epoch: SystemEpoch,
   457	    pub timestamp_logical: u64,
   458	}
   459	
   460	impl FinalizeRewardSigningPayload {
   461	    pub fn canonical_digest(&self) -> [u8; 32] {
   462	        domain_prefixed_digest(DOMAIN_SYSTEM_FINALIZE_REWARD, self)
   463	    }
   464	}
   465	
   466	/// System signing payload for `TaskExpireTx` (7 fields → 6 fields).
   467	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   468	pub struct TaskExpireSigningPayload {
   469	    pub tx_id: TxId,
   470	    pub task_id: TaskId,
   471	    pub parent_state_root: Hash,
   472	    pub bounty_refunded: MicroCoin,
   473	    pub epoch: SystemEpoch,
   474	    pub timestamp_logical: u64,
   475	}
   476	
   477	impl TaskExpireSigningPayload {
   478	    pub fn canonical_digest(&self) -> [u8; 32] {
   479	        domain_prefixed_digest(DOMAIN_SYSTEM_TASK_EXPIRE, self)
   480	    }
   481	}
   482	
   483	/// System signing payload for `TerminalSummaryTx` (8 fields → 7 fields).
   484	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   485	pub struct TerminalSummarySigningPayload {
   486	    pub tx_id: TxId,
   487	    pub task_id: TaskId,
   488	    pub run_id: RunId,
   489	    pub run_outcome: RunOutcome,
   490	    pub total_attempts: u32,
   491	    pub failure_class_histogram: BTreeMap<RejectionClass, u32>,
   492	    pub last_logical_t: u64,
   493	}
   494	
   495	impl TerminalSummarySigningPayload {
   496	    pub fn canonical_digest(&self) -> [u8; 32] {
   497	        domain_prefixed_digest(DOMAIN_SYSTEM_TERMINAL_SUMMARY, self)
   498	    }
   499	}
   500	
   501	// ── Projections: tx → signing payload ────────────────────────────────────
   502	
   503	impl WorkTx {
   504	    pub fn to_signing_payload(&self) -> WorkSigningPayload {
   505	        WorkSigningPayload {
   506	            tx_id: self.tx_id.clone(),
   507	            task_id: self.task_id.clone(),
   508	            parent_state_root: self.parent_state_root,
   509	            agent_id: self.agent_id.clone(),
   510	            read_set: self.read_set.clone(),
   511	            write_set: self.write_set.clone(),
   512	            proposal_cid: self.proposal_cid,
   513	            predicate_results: self.predicate_results.clone(),
   514	            stake: self.stake,
   515	            timestamp_logical: self.timestamp_logical,
   516	        }
   517	    }
   518	}
   519	
   520	impl VerifyTx {
   521	    pub fn to_signing_payload(&self) -> VerifySigningPayload {
   522	        VerifySigningPayload {
   523	            tx_id: self.tx_id.clone(),
   524	            target_work_tx: self.target_work_tx.clone(),
   525	            verifier_agent: self.verifier_agent.clone(),
   526	            bond: self.bond,
   527	            verdict: self.verdict,
   528	            timestamp_logical: self.timestamp_logical,
   529	        }
   530	    }
   531	}
   532	
   533	impl ChallengeTx {
   534	    pub fn to_signing_payload(&self) -> ChallengeSigningPayload {
   535	        ChallengeSigningPayload {
   536	            tx_id: self.tx_id.clone(),
   537	            target_work_tx: self.target_work_tx.clone(),
   538	            challenger_agent: self.challenger_agent.clone(),
   539	            stake: self.stake,
   540	            counterexample_cid: self.counterexample_cid,
   541	            timestamp_logical: self.timestamp_logical,
   542	        }
   543	    }
   544	}
   545	
   546	impl FinalizeRewardTx {
   547	    pub fn to_signing_payload(&self) -> FinalizeRewardSigningPayload {
   548	        FinalizeRewardSigningPayload {
   549	            tx_id: self.tx_id.clone(),
   550	            claim_id: self.claim_id.clone(),
   551	            task_id: self.task_id.clone(),
   552	            solver: self.solver.clone(),
   553	            reward: self.reward,
   554	            parent_state_root: self.parent_state_root,
   555	            epoch: self.epoch,
   556	            timestamp_logical: self.timestamp_logical,
   557	        }
   558	    }
   559	}
   560	
   561	impl TaskExpireTx {
   562	    pub fn to_signing_payload(&self) -> TaskExpireSigningPayload {
   563	        TaskExpireSigningPayload {
   564	            tx_id: self.tx_id.clone(),
   565	            task_id: self.task_id.clone(),
   566	            parent_state_root: self.parent_state_root,
   567	            bounty_refunded: self.bounty_refunded,
   568	            epoch: self.epoch,
   569	            timestamp_logical: self.timestamp_logical,
   570	        }
   571	    }
   572	}
   573	
   574	impl TerminalSummaryTx {
   575	    pub fn to_signing_payload(&self) -> TerminalSummarySigningPayload {
   576	        TerminalSummarySigningPayload {
   577	            tx_id: self.tx_id.clone(),
   578	            task_id: self.task_id.clone(),
   579	            run_id: self.run_id.clone(),
   580	            run_outcome: self.run_outcome,
   581	            total_attempts: self.total_attempts,
   582	            failure_class_histogram: self.failure_class_histogram.clone(),
   583	            last_logical_t: self.last_logical_t,
   584	        }
   585	    }
   586	}
   587	
   588	// ────────────────────────────────────────────────────────────────────────────
   589	// § 6 TypedTx outer enum
   590	// ────────────────────────────────────────────────────────────────────────────
   591	
   592	/// TRACE_MATRIX § 8 dispatch_transition — typed-tx outer enum.
   593	/// 7 variants (K5 closed: NO `Slash`).
   594	/// `TerminalSummaryTx` is imported from `system_keypair.rs` (already shipped).
   595	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
   596	pub enum TypedTx {
   597	    Work(WorkTx),
   598	    Verify(VerifyTx),
   599	    Challenge(ChallengeTx),
   600	    Reuse(ReuseTx),
   601	    FinalizeReward(FinalizeRewardTx),
   602	    TaskExpire(TaskExpireTx),
   603	    TerminalSummary(TerminalSummaryTx),
   604	}
   605	
   606	impl TypedTx {
   607	    /// Project to the [`TxKind`] discriminator stored in `LedgerEntry.tx_kind`.
   608	    pub fn tx_kind(&self) -> crate::bottom_white::ledger::transition_ledger::TxKind {
   609	        use crate::bottom_white::ledger::transition_ledger::TxKind;
   610	        match self {
   611	            Self::Work(_) => TxKind::Work,
   612	            Self::Verify(_) => TxKind::Verify,
   613	            Self::Challenge(_) => TxKind::Challenge,
   614	            Self::Reuse(_) => TxKind::Reuse,
   615	            Self::FinalizeReward(_) => TxKind::FinalizeReward,
   616	            Self::TaskExpire(_) => TxKind::TaskExpire,
   617	            Self::TerminalSummary(_) => TxKind::TerminalSummary,
   618	        }
   619	    }
   620	}

exec
/bin/bash -lc "nl -ba src/state/typed_tx.rs | sed -n '620,880p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   620	}
   621	
   622	// ────────────────────────────────────────────────────────────────────────────
   623	// § 8 HasSubmitter trait (STATE spec § 3.6.5 v1.3)
   624	// ────────────────────────────────────────────────────────────────────────────
   625	
   626	/// TRACE_MATRIX STATE spec § 3.6.5 v1.3 — submitter resolution trait used
   627	/// by the implicit-init step in agent-submitted transitions. System-emitted
   628	/// transitions return `None` (no agent to init).
   629	pub trait HasSubmitter {
   630	    fn submitter_id(&self) -> Option<AgentId>;
   631	}
   632	
   633	impl HasSubmitter for WorkTx {
   634	    fn submitter_id(&self) -> Option<AgentId> {
   635	        Some(self.agent_id.clone())
   636	    }
   637	}
   638	
   639	impl HasSubmitter for VerifyTx {
   640	    fn submitter_id(&self) -> Option<AgentId> {
   641	        Some(self.verifier_agent.clone())
   642	    }
   643	}
   644	
   645	impl HasSubmitter for ChallengeTx {
   646	    fn submitter_id(&self) -> Option<AgentId> {
   647	        Some(self.challenger_agent.clone())
   648	    }
   649	}
   650	
   651	impl HasSubmitter for ReuseTx {
   652	    fn submitter_id(&self) -> Option<AgentId> {
   653	        None
   654	    }
   655	}
   656	
   657	impl HasSubmitter for FinalizeRewardTx {
   658	    fn submitter_id(&self) -> Option<AgentId> {
   659	        None
   660	    }
   661	}
   662	
   663	impl HasSubmitter for TaskExpireTx {
   664	    fn submitter_id(&self) -> Option<AgentId> {
   665	        None
   666	    }
   667	}
   668	
   669	impl HasSubmitter for TerminalSummaryTx {
   670	    fn submitter_id(&self) -> Option<AgentId> {
   671	        None
   672	    }
   673	}
   674	
   675	impl HasSubmitter for TypedTx {
   676	    fn submitter_id(&self) -> Option<AgentId> {
   677	        match self {
   678	            Self::Work(t) => t.submitter_id(),
   679	            Self::Verify(t) => t.submitter_id(),
   680	            Self::Challenge(t) => t.submitter_id(),
   681	            Self::Reuse(t) => t.submitter_id(),
   682	            Self::FinalizeReward(t) => t.submitter_id(),
   683	            Self::TaskExpire(t) => t.submitter_id(),
   684	            Self::TerminalSummary(t) => t.submitter_id(),
   685	        }
   686	    }
   687	}
   688	
   689	// ────────────────────────────────────────────────────────────────────────────
   690	// TransitionError — minimal v1 taxonomy (CO1.1.4-pre1 spec § 0 out-of-scope
   691	// note: full per-stage enum proliferation is CO1.7.5)
   692	// ────────────────────────────────────────────────────────────────────────────
   693	
   694	/// TRACE_MATRIX STATE § 3 — transition-function error taxonomy. v1.1 covers
   695	/// every variant invoked in STATE_TRANSITION_SPEC § 3.1-3.7 pseudocode +
   696	/// `NotYetImplemented` for CO1.7.5 stub bodies (per Codex Q-G CHALLENGE).
   697	///
   698	/// **Why payloads are minimal**: the failed `PredicateId` (etc.) is a string
   699	/// reference; richer context (PredicateResultsBundle, Cid of failed proof)
   700	/// is attached by the runtime via separate book-keeping channels (rejected
   701	/// summary stamping, bus rejection log). Keeping TransitionError serializable
   702	/// with primitive payloads avoids forcing PredicateResultsBundle through
   703	/// every error site.
   704	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
   705	pub enum TransitionError {
   706	    // ── Stale-parent & signature ───────────────────────────────────────────
   707	    /// `parent_state_root` does not match `q.state_root_t` (any agent tx).
   708	    StaleParent,
   709	    /// Agent signature verify failed (work / verify / challenge tx).
   710	    SignatureInvalid,
   711	    /// System-keypair signature verify failed (system-emitted tx).
   712	    InvalidSystemSignature,
   713	
   714	    // ── Economy ────────────────────────────────────────────────────────────
   715	    /// Submitter's available balance is below the declared stake / bond.
   716	    /// Payload-rich variant (available + required) is intentionally elided
   717	    /// in v1.1 to keep this enum primitive-payloads-only; runtime attaches
   718	    /// context via the rejection log (per STATE § 1.4 RejectedAttemptSummary).
   719	    StakeInsufficient,
   720	
   721	    // ── Target lookup ──────────────────────────────────────────────────────
   722	    /// VerifyTx / ChallengeTx / ReuseTx target work_tx not found in L4.
   723	    TargetWorkTxNotFound,
   724	    /// VerifyTx target is not in a verifiable status (e.g. already finalized).
   725	    TargetWorkTxNotVerifiable,
   726	    /// ReuseTx target work_tx exists but is not yet Accepted (parent must accept first).
   727	    ParentNotAcceptedYet,
   728	
   729	    // ── Predicate failures ─────────────────────────────────────────────────
   730	    /// step_transition stage 4 — acceptance predicate denied. `PredicateId`
   731	    /// is the public predicate that failed; private predicates surface as
   732	    /// `RejectionClass::Opaque` in book-keeping (NOT here).
   733	    AcceptancePredicateFailed(PredicateId),
   734	    /// verify_transition stage 4 — verification predicate denied.
   735	    VerificationPredicateFailed(PredicateId),
   736	    /// finalize_reward / step_transition stage 5 — settlement predicate denied.
   737	    SettlementPredicateFailed(PredicateId),
   738	
   739	    // ── Challenge ──────────────────────────────────────────────────────────
   740	    /// challenge_transition stage 1 — challenge filed after window closed.
   741	    ChallengeWindowClosed,
   742	    /// finalize_reward stage 1 — challenge window still open; cannot finalize.
   743	    ChallengeWindowStillOpen,
   744	    /// finalize_reward stage 1 — claim already slashed; cannot also reward.
   745	    AlreadySlashed,
   746	    /// challenge_transition stage 4 — counterexample failed predicate check.
   747	    CounterexampleInsufficient,
   748	
   749	    // ── Reuse ──────────────────────────────────────────────────────────────
   750	    /// reuse_transition stage 1 — referenced tool not in L2 ToolRegistry.
   751	    ToolNotInRegistry,
   752	    /// reuse_transition stage 1 — declared tool creator does not match registry.
   753	    ToolCreatorMismatch,
   754	
   755	    // ── Finalize ───────────────────────────────────────────────────────────
   756	    /// finalize_reward — no claim entry for the given claim_id.
   757	    ClaimNotFound,
   758	
   759	    // ── Task expire ────────────────────────────────────────────────────────
   760	    /// task_expire — referenced TaskMarket entry not found.
   761	    TaskNotFound,
   762	    /// task_expire — deadline not yet reached.
   763	    TaskNotExpired,
   764	    /// task_expire — at least one open claim exists; cannot refund bounty.
   765	    TaskHasOpenClaim,
   766	
   767	    // ── Terminal summary ───────────────────────────────────────────────────
   768	    /// emit_terminal_summary — run already has an accepted work_tx.
   769	    TerminalSummaryNotApplicable,
   770	
   771	    // ── Stub sentinel (CO1.7.5 fills) ──────────────────────────────────────
   772	    /// Stub return value used by CO1.7.5 unimplemented bodies — preserves
   773	    /// sequencer + dispatch correctness without forcing transition logic
   774	    /// into this atom. Audit input: this is intentional, not a code smell.
   775	    NotYetImplemented,
   776	}
   777	
   778	impl std::fmt::Display for TransitionError {
   779	    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
   780	        match self {
   781	            Self::StaleParent => write!(f, "stale parent_state_root"),
   782	            Self::SignatureInvalid => write!(f, "agent signature invalid"),
   783	            Self::InvalidSystemSignature => write!(f, "invalid system signature"),
   784	            Self::StakeInsufficient => write!(f, "stake / bond insufficient"),
   785	            Self::TargetWorkTxNotFound => write!(f, "target work_tx not found"),
   786	            Self::TargetWorkTxNotVerifiable => write!(f, "target work_tx not in a verifiable state"),
   787	            Self::ParentNotAcceptedYet => write!(f, "parent work_tx not yet accepted"),
   788	            Self::AcceptancePredicateFailed(p) => write!(f, "acceptance predicate failed: {p:?}"),
   789	            Self::VerificationPredicateFailed(p) => write!(f, "verification predicate failed: {p:?}"),
   790	            Self::SettlementPredicateFailed(p) => write!(f, "settlement predicate failed: {p:?}"),
   791	            Self::ChallengeWindowClosed => write!(f, "challenge window closed"),
   792	            Self::ChallengeWindowStillOpen => write!(f, "challenge window still open"),
   793	            Self::AlreadySlashed => write!(f, "already slashed"),
   794	            Self::CounterexampleInsufficient => write!(f, "counterexample insufficient"),
   795	            Self::ToolNotInRegistry => write!(f, "reuse tool not in registry"),
   796	            Self::ToolCreatorMismatch => write!(f, "reuse tool creator mismatch"),
   797	            Self::ClaimNotFound => write!(f, "claim not found"),
   798	            Self::TaskNotFound => write!(f, "task not found"),
   799	            Self::TaskNotExpired => write!(f, "task deadline not yet reached"),
   800	            Self::TaskHasOpenClaim => write!(f, "task has at least one open claim"),
   801	            Self::TerminalSummaryNotApplicable => write!(f, "terminal summary not applicable"),
   802	            Self::NotYetImplemented => write!(f, "transition body not yet implemented (CO1.7.5)"),
   803	        }
   804	    }
   805	}
   806	impl std::error::Error for TransitionError {}
   807	
   808	// ────────────────────────────────────────────────────────────────────────────
   809	// SignalBundle — minimal v1 typed shape (CO1.7.5 + CO1.9 enrich it later)
   810	// ────────────────────────────────────────────────────────────────────────────
   811	
   812	/// TRACE_MATRIX STATE § 3 — tape-emitted signal bundle. v1 minimal: a single
   813	/// enum variant per spec call site in § 3 pseudocode (`empty` /
   814	/// `finalize` / `task_expired` / `terminal_summary`). Full L6 signal-stream
   815	/// design is CO1.9. CO1.1.4-pre1 ships just enough shape for CO1.7-impl to
   816	/// compile and for CO1.7.5 transition bodies to construct each variant.
   817	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   818	pub struct SignalBundle {
   819	    pub kind: SignalKind,
   820	}
   821	
   822	/// Discriminator over the spec § 3 pseudocode's `SignalBundle::*` constructors.
   823	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
   824	pub enum SignalKind {
   825	    Empty,
   826	    Finalize {
   827	        claim_id: TxId,
   828	        reward: MicroCoin,
   829	    },
   830	    TaskExpired {
   831	        task_id: TaskId,
   832	        bounty_refunded: MicroCoin,
   833	    },
   834	    TerminalSummary {
   835	        run_id: RunId,
   836	        outcome: RunOutcome,
   837	    },
   838	}
   839	
   840	impl Default for SignalKind {
   841	    fn default() -> Self {
   842	        Self::Empty
   843	    }
   844	}
   845	
   846	impl SignalBundle {
   847	    pub fn empty() -> Self {
   848	        Self {
   849	            kind: SignalKind::Empty,
   850	        }
   851	    }
   852	    pub fn finalize(claim_id: TxId, reward: MicroCoin) -> Self {
   853	        Self {
   854	            kind: SignalKind::Finalize { claim_id, reward },
   855	        }
   856	    }
   857	    pub fn task_expired(task_id: TaskId, bounty_refunded: MicroCoin) -> Self {
   858	        Self {
   859	            kind: SignalKind::TaskExpired {
   860	                task_id,
   861	                bounty_refunded,
   862	            },
   863	        }
   864	    }
   865	    pub fn terminal_summary(run_id: RunId, outcome: RunOutcome) -> Self {
   866	        Self {
   867	            kind: SignalKind::TerminalSummary { run_id, outcome },
   868	        }
   869	    }
   870	}
   871	
   872	// ────────────────────────────────────────────────────────────────────────────
   873	// Tests — round-trip (I-CANON-A/B/C) + golden fixtures (I-CANON-D)
   874	// ────────────────────────────────────────────────────────────────────────────
   875	
   876	#[cfg(test)]
   877	mod tests {
   878	    use super::*;
   879	    use crate::bottom_white::ledger::transition_ledger::{canonical_decode, canonical_encode};
   880	    use sha2::{Digest, Sha256};

exec
/bin/bash -lc "nl -ba src/state/typed_tx.rs | sed -n '880,1380p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   880	    use sha2::{Digest, Sha256};
   881	
   882	    fn h(byte: u8) -> Hash {
   883	        Hash([byte; 32])
   884	    }
   885	    fn cid(byte: u8) -> Cid {
   886	        Cid([byte; 32])
   887	    }
   888	
   889	    /// Helper: canonical bytes → SHA-256 hex string. Used to lock golden
   890	    /// fixtures: any future change to the wire format causes the digest hex
   891	    /// to diverge → audit-required.
   892	    fn digest_hex<T: Serialize>(value: &T) -> String {
   893	        let bytes = canonical_encode(value).expect("encode");
   894	        let hash = Sha256::digest(&bytes);
   895	        hex_lower(&hash)
   896	    }
   897	    fn hex_lower(bytes: &[u8]) -> String {
   898	        let mut s = String::with_capacity(bytes.len() * 2);
   899	        for b in bytes {
   900	            s.push_str(&format!("{:02x}", b));
   901	        }
   902	        s
   903	    }
   904	
   905	    // ── I-CANON-A/B/C — round-trip + byte-stability ──────────────────────────
   906	
   907	    fn fixture_work_tx() -> WorkTx {
   908	        let mut acceptance = BTreeMap::new();
   909	        acceptance.insert(
   910	            PredicateId("acc1".into()),
   911	            BoolWithProof {
   912	                value: true,
   913	                proof_cid: Some(cid(0x11)),
   914	            },
   915	        );
   916	        let mut settlement = BTreeMap::new();
   917	        settlement.insert(
   918	            PredicateId("set1".into()),
   919	            BoolWithProof {
   920	                value: true,
   921	                proof_cid: None,
   922	            },
   923	        );
   924	        WorkTx {
   925	            tx_id: TxId("worktx-fixture-01".into()),
   926	            task_id: TaskId("task-fixture-01".into()),
   927	            parent_state_root: h(0x42),
   928	            agent_id: AgentId("alice".into()),
   929	            read_set: [ReadKey("k.read.a".into()), ReadKey("k.read.b".into())]
   930	                .into_iter()
   931	                .collect(),
   932	            write_set: [WriteKey("k.write.a".into())].into_iter().collect(),
   933	            proposal_cid: cid(0x13),
   934	            predicate_results: PredicateResultsBundle {
   935	                acceptance,
   936	                settlement,
   937	                safety_class: SafetyOrCreation::Safety,
   938	            },
   939	            stake: StakeMicroCoin::from_micro_units(1_000_000),
   940	            signature: AgentSignature::from_bytes([0x77u8; 64]),
   941	            timestamp_logical: 7,
   942	        }
   943	    }
   944	
   945	    fn fixture_verify_tx() -> VerifyTx {
   946	        VerifyTx {
   947	            tx_id: TxId("verifytx-fixture-01".into()),
   948	            target_work_tx: TxId("worktx-fixture-01".into()),
   949	            verifier_agent: AgentId("bob".into()),
   950	            bond: StakeMicroCoin::from_micro_units(500_000),
   951	            verdict: VerifyVerdict::Confirm,
   952	            signature: AgentSignature::from_bytes([0x55u8; 64]),
   953	            timestamp_logical: 8,
   954	        }
   955	    }
   956	
   957	    fn fixture_challenge_tx() -> ChallengeTx {
   958	        ChallengeTx {
   959	            tx_id: TxId("challengetx-fixture-01".into()),
   960	            target_work_tx: TxId("worktx-fixture-01".into()),
   961	            challenger_agent: AgentId("carol".into()),
   962	            stake: StakeMicroCoin::from_micro_units(2_000_000),
   963	            counterexample_cid: cid(0x21),
   964	            signature: AgentSignature::from_bytes([0x33u8; 64]),
   965	            timestamp_logical: 9,
   966	        }
   967	    }
   968	
   969	    fn fixture_reuse_tx() -> ReuseTx {
   970	        ReuseTx {
   971	            tx_id: TxId("reusetx-fixture-01".into()),
   972	            reusing_work_tx: TxId("worktx-fixture-02".into()),
   973	            reused_tool_id: ToolId("tool-001".into()),
   974	            reused_tool_creator: AgentId("alice".into()),
   975	            timestamp_logical: 10,
   976	        }
   977	    }
   978	
   979	    fn fixture_finalize_reward_tx() -> FinalizeRewardTx {
   980	        FinalizeRewardTx {
   981	            tx_id: TxId("finalizetx-fixture-01".into()),
   982	            claim_id: ClaimId::new("claim-001"),
   983	            task_id: TaskId("task-fixture-01".into()),
   984	            solver: AgentId("alice".into()),
   985	            reward: MicroCoin::from_micro_units(5_000_000),
   986	            parent_state_root: h(0x43),
   987	            epoch: SystemEpoch::new(1),
   988	            timestamp_logical: 11,
   989	            system_signature: SystemSignature::from_bytes([0xaau8; 64]),
   990	        }
   991	    }
   992	
   993	    fn fixture_task_expire_tx() -> TaskExpireTx {
   994	        TaskExpireTx {
   995	            tx_id: TxId("expiretx-fixture-01".into()),
   996	            task_id: TaskId("task-fixture-02".into()),
   997	            parent_state_root: h(0x44),
   998	            bounty_refunded: MicroCoin::from_micro_units(3_000_000),
   999	            epoch: SystemEpoch::new(1),
  1000	            timestamp_logical: 12,
  1001	            system_signature: SystemSignature::from_bytes([0xbbu8; 64]),
  1002	        }
  1003	    }
  1004	
  1005	    fn fixture_terminal_summary_tx() -> TerminalSummaryTx {
  1006	        let mut hist = BTreeMap::new();
  1007	        hist.insert(RejectionClass::SignatureInvalid, 2);
  1008	        hist.insert(RejectionClass::StakeInsufficient, 1);
  1009	        hist.insert(
  1010	            RejectionClass::AcceptancePredicateFail(PredicateId("acc1".into())),
  1011	            5,
  1012	        );
  1013	        TerminalSummaryTx {
  1014	            tx_id: TxId("terminalsummary-fixture-01".into()),
  1015	            task_id: TaskId("task-fixture-03".into()),
  1016	            run_id: RunId("run-001".into()),
  1017	            run_outcome: RunOutcome::MaxTxExhausted,
  1018	            total_attempts: 8,
  1019	            failure_class_histogram: hist,
  1020	            last_logical_t: 13,
  1021	            system_signature: SystemSignature::from_bytes([0xccu8; 64]),
  1022	        }
  1023	    }
  1024	
  1025	    /// Round-trip for every typed-tx variant.
  1026	    #[test]
  1027	    fn typed_tx_round_trip_all_variants() {
  1028	        let cases: Vec<TypedTx> = vec![
  1029	            TypedTx::Work(fixture_work_tx()),
  1030	            TypedTx::Verify(fixture_verify_tx()),
  1031	            TypedTx::Challenge(fixture_challenge_tx()),
  1032	            TypedTx::Reuse(fixture_reuse_tx()),
  1033	            TypedTx::FinalizeReward(fixture_finalize_reward_tx()),
  1034	            TypedTx::TaskExpire(fixture_task_expire_tx()),
  1035	            TypedTx::TerminalSummary(fixture_terminal_summary_tx()),
  1036	        ];
  1037	        for tx in cases {
  1038	            let bytes = canonical_encode(&tx).expect("encode");
  1039	            let decoded: TypedTx = canonical_decode(&bytes).expect("decode");
  1040	            assert_eq!(tx, decoded, "round-trip mismatch on {:?}", tx.tx_kind());
  1041	        }
  1042	    }
  1043	
  1044	    /// Two encodes of the same value produce byte-identical bytes.
  1045	    #[test]
  1046	    fn typed_tx_byte_stability_across_calls() {
  1047	        let tx = TypedTx::Work(fixture_work_tx());
  1048	        let bytes_a = canonical_encode(&tx).expect("encode a");
  1049	        let bytes_b = canonical_encode(&tx).expect("encode b");
  1050	        assert_eq!(bytes_a, bytes_b);
  1051	    }
  1052	
  1053	    /// 100-input round-trip: random-ish AgentSignature bytes + variant choice.
  1054	    #[test]
  1055	    fn typed_tx_round_trip_100_inputs() {
  1056	        let mut tx = fixture_work_tx();
  1057	        for i in 0u32..100 {
  1058	            tx.timestamp_logical = i as u64;
  1059	            tx.signature = AgentSignature::from_bytes([(i % 256) as u8; 64]);
  1060	            let outer = TypedTx::Work(tx.clone());
  1061	            let bytes = canonical_encode(&outer).expect("encode");
  1062	            let back: TypedTx = canonical_decode(&bytes).expect("decode");
  1063	            assert_eq!(outer, back);
  1064	        }
  1065	    }
  1066	
  1067	    /// HasSubmitter — agent-submitted vs system-emitted partitioning.
  1068	    #[test]
  1069	    fn has_submitter_partitioning() {
  1070	        let alice = AgentId("alice".into());
  1071	        assert_eq!(
  1072	            TypedTx::Work(fixture_work_tx()).submitter_id(),
  1073	            Some(alice.clone())
  1074	        );
  1075	        assert_eq!(
  1076	            TypedTx::Verify(fixture_verify_tx()).submitter_id(),
  1077	            Some(AgentId("bob".into()))
  1078	        );
  1079	        assert_eq!(
  1080	            TypedTx::Challenge(fixture_challenge_tx()).submitter_id(),
  1081	            Some(AgentId("carol".into()))
  1082	        );
  1083	        assert_eq!(TypedTx::Reuse(fixture_reuse_tx()).submitter_id(), None);
  1084	        assert_eq!(
  1085	            TypedTx::FinalizeReward(fixture_finalize_reward_tx()).submitter_id(),
  1086	            None
  1087	        );
  1088	        assert_eq!(
  1089	            TypedTx::TaskExpire(fixture_task_expire_tx()).submitter_id(),
  1090	            None
  1091	        );
  1092	    }
  1093	
  1094	    /// tx_kind matches the LedgerEntry TxKind enum variant.
  1095	    #[test]
  1096	    fn typed_tx_kind_projection() {
  1097	        use crate::bottom_white::ledger::transition_ledger::TxKind;
  1098	        assert_eq!(TypedTx::Work(fixture_work_tx()).tx_kind(), TxKind::Work);
  1099	        assert_eq!(
  1100	            TypedTx::Verify(fixture_verify_tx()).tx_kind(),
  1101	            TxKind::Verify
  1102	        );
  1103	        assert_eq!(
  1104	            TypedTx::Challenge(fixture_challenge_tx()).tx_kind(),
  1105	            TxKind::Challenge
  1106	        );
  1107	        assert_eq!(TypedTx::Reuse(fixture_reuse_tx()).tx_kind(), TxKind::Reuse);
  1108	        assert_eq!(
  1109	            TypedTx::FinalizeReward(fixture_finalize_reward_tx()).tx_kind(),
  1110	            TxKind::FinalizeReward
  1111	        );
  1112	        assert_eq!(
  1113	            TypedTx::TaskExpire(fixture_task_expire_tx()).tx_kind(),
  1114	            TxKind::TaskExpire
  1115	        );
  1116	        assert_eq!(
  1117	            TypedTx::TerminalSummary(fixture_terminal_summary_tx()).tx_kind(),
  1118	            TxKind::TerminalSummary,
  1119	        );
  1120	    }
  1121	
  1122	    // ── v1.1 NEW: cross-variant non-collision (C-2 / Codex Q-J) ──────────────
  1123	
  1124	    /// All 7 TypedTx variant fixtures encode to pairwise-distinct canonical bytes.
  1125	    /// (Different field shapes + bincode variant tags → ANY collision is a bincode
  1126	    /// regression that this test catches.)
  1127	    #[test]
  1128	    fn typed_tx_cross_variant_non_collision() {
  1129	        let variants: Vec<(&str, TypedTx)> = vec![
  1130	            ("Work", TypedTx::Work(fixture_work_tx())),
  1131	            ("Verify", TypedTx::Verify(fixture_verify_tx())),
  1132	            ("Challenge", TypedTx::Challenge(fixture_challenge_tx())),
  1133	            ("Reuse", TypedTx::Reuse(fixture_reuse_tx())),
  1134	            (
  1135	                "FinalizeReward",
  1136	                TypedTx::FinalizeReward(fixture_finalize_reward_tx()),
  1137	            ),
  1138	            ("TaskExpire", TypedTx::TaskExpire(fixture_task_expire_tx())),
  1139	            (
  1140	                "TerminalSummary",
  1141	                TypedTx::TerminalSummary(fixture_terminal_summary_tx()),
  1142	            ),
  1143	        ];
  1144	        let digests: Vec<(&str, String)> = variants
  1145	            .iter()
  1146	            .map(|(name, tx)| (*name, digest_hex(tx)))
  1147	            .collect();
  1148	        for i in 0..digests.len() {
  1149	            for j in (i + 1)..digests.len() {
  1150	                assert_ne!(
  1151	                    digests[i].1, digests[j].1,
  1152	                    "{} and {} have colliding canonical digests",
  1153	                    digests[i].0, digests[j].0
  1154	                );
  1155	            }
  1156	        }
  1157	    }
  1158	
  1159	    // ── v1.1 NEW: BTreeMap / BTreeSet permutation independence (C-2 / Gemini Q9) ─
  1160	
  1161	    /// Building the same WorkTx via different `BTreeSet` insertion orders produces
  1162	    /// byte-identical canonical bytes. (BTreeSet iterates in sorted order, but
  1163	    /// this test locks that bincode honors the iteration order — defensive against
  1164	    /// a future codec choice that uses HashMap-style hash-randomized iteration.)
  1165	    #[test]
  1166	    fn typed_tx_btree_permutation_independence() {
  1167	        let make_work_tx = |read_keys_in_order: &[&str]| -> WorkTx {
  1168	            let mut tx = fixture_work_tx();
  1169	            tx.read_set = BTreeSet::new();
  1170	            for k in read_keys_in_order {
  1171	                tx.read_set.insert(ReadKey((*k).into()));
  1172	            }
  1173	            tx
  1174	        };
  1175	        // Insert keys in different orders.
  1176	        let tx_a = make_work_tx(&["k.read.a", "k.read.b", "k.read.c"]);
  1177	        let tx_b = make_work_tx(&["k.read.c", "k.read.a", "k.read.b"]);
  1178	        let tx_c = make_work_tx(&["k.read.b", "k.read.c", "k.read.a"]);
  1179	        let bytes_a = canonical_encode(&tx_a).expect("encode a");
  1180	        let bytes_b = canonical_encode(&tx_b).expect("encode b");
  1181	        let bytes_c = canonical_encode(&tx_c).expect("encode c");
  1182	        assert_eq!(bytes_a, bytes_b);
  1183	        assert_eq!(bytes_a, bytes_c);
  1184	    }
  1185	
  1186	    // ── v1.1 NEW: zero-default round-trip per main tx kind (Gemini Q9) ──────
  1187	
  1188	    #[test]
  1189	    fn typed_tx_default_round_trip() {
  1190	        let cases: Vec<TypedTx> = vec![
  1191	            TypedTx::Work(WorkTx::default()),
  1192	            TypedTx::Verify(VerifyTx::default()),
  1193	            TypedTx::Challenge(ChallengeTx::default()),
  1194	            TypedTx::Reuse(ReuseTx::default()),
  1195	            TypedTx::FinalizeReward(FinalizeRewardTx::default()),
  1196	            TypedTx::TaskExpire(TaskExpireTx::default()),
  1197	            TypedTx::TerminalSummary(TerminalSummaryTx::default()),
  1198	        ];
  1199	        for tx in cases {
  1200	            let bytes = canonical_encode(&tx).expect("encode default");
  1201	            let back: TypedTx = canonical_decode(&bytes).expect("decode default");
  1202	            assert_eq!(tx, back, "default round-trip mismatch on {:?}", tx.tx_kind());
  1203	        }
  1204	    }
  1205	
  1206	    // ── v1.1 NEW: signing-payload domain-prefix non-collision (C-1) ─────────
  1207	
  1208	    /// 6 signing-payload digests (Work / Verify / Challenge agent + Finalize /
  1209	    /// TaskExpire / TerminalSummary system) all have distinct domain prefixes;
  1210	    /// even if their bincode bodies COULD overlap, the SHA-256 inputs differ.
  1211	    /// We don't construct bodies that overlap (different fields); the assertion
  1212	    /// is simply that all 6 distinct domain-prefixed digests are pairwise distinct
  1213	    /// — which is the property auditors flagged as essential.
  1214	    #[test]
  1215	    fn signing_payload_domains_are_distinct() {
  1216	        let digests: Vec<(&str, [u8; 32])> = vec![
  1217	            ("Work", fixture_work_tx().to_signing_payload().canonical_digest()),
  1218	            (
  1219	                "Verify",
  1220	                fixture_verify_tx().to_signing_payload().canonical_digest(),
  1221	            ),
  1222	            (
  1223	                "Challenge",
  1224	                fixture_challenge_tx().to_signing_payload().canonical_digest(),
  1225	            ),
  1226	            (
  1227	                "FinalizeReward",
  1228	                fixture_finalize_reward_tx()
  1229	                    .to_signing_payload()
  1230	                    .canonical_digest(),
  1231	            ),
  1232	            (
  1233	                "TaskExpire",
  1234	                fixture_task_expire_tx()
  1235	                    .to_signing_payload()
  1236	                    .canonical_digest(),
  1237	            ),
  1238	            (
  1239	                "TerminalSummary",
  1240	                fixture_terminal_summary_tx()
  1241	                    .to_signing_payload()
  1242	                    .canonical_digest(),
  1243	            ),
  1244	        ];
  1245	        for i in 0..digests.len() {
  1246	            for j in (i + 1)..digests.len() {
  1247	                assert_ne!(
  1248	                    digests[i].1, digests[j].1,
  1249	                    "{} and {} signing-payload digests collide",
  1250	                    digests[i].0, digests[j].0
  1251	                );
  1252	            }
  1253	        }
  1254	    }
  1255	
  1256	    /// Excluding the signature: mutating `tx.signature` must NOT change the
  1257	    /// signing-payload digest (the signature is its own input — a canonical
  1258	    /// digest cycle prevention property).
  1259	    #[test]
  1260	    fn signing_payload_excludes_signature() {
  1261	        let tx_clean = fixture_work_tx();
  1262	        let d_clean = tx_clean.to_signing_payload().canonical_digest();
  1263	
  1264	        let mut tx_mut = tx_clean.clone();
  1265	        tx_mut.signature = AgentSignature::from_bytes([0xff; 64]);
  1266	        let d_mut_sig = tx_mut.to_signing_payload().canonical_digest();
  1267	        assert_eq!(d_clean, d_mut_sig, "mutating signature must NOT affect digest");
  1268	
  1269	        // Sanity: mutating a SIGNED field DOES change digest.
  1270	        let mut tx_signed_change = tx_clean.clone();
  1271	        tx_signed_change.timestamp_logical = 9999;
  1272	        let d_signed = tx_signed_change.to_signing_payload().canonical_digest();
  1273	        assert_ne!(d_clean, d_signed);
  1274	    }
  1275	
  1276	    // ── I-CANON-D — golden fixtures (locked SHA-256 of canonical bytes) ──────
  1277	    //
  1278	    // **v1.1 round-1 closure (C-2 / Codex Q-J / Gemini Q9)**: hex values are
  1279	    // hardcoded — any future codec / schema change causes the assertion to
  1280	    // fail, forcing a deliberate "ABI golden fixture rotation" commit with
  1281	    // re-audit. To rotate:
  1282	    //   1. Run `cargo test --lib state::typed_tx::tests::golden_` with current code
  1283	    //   2. The assertion failure messages report the new hex in the `actual` slot
  1284	    //   3. Update each `EXPECTED_HEX` constant + cite the rotation rationale in commit message
  1285	
  1286	    const EXPECTED_HEX_WORK: &str =
  1287	        "6ec94fa4910ef4cc108ca8f36c202647d2cf60426d13ca0bccf777efb07b4fef";
  1288	    const EXPECTED_HEX_VERIFY: &str =
  1289	        "425b9bd7e99c427b3b7934d45a00dee3d66fc346deed72ec307de01bb3f1db99";
  1290	    const EXPECTED_HEX_CHALLENGE: &str =
  1291	        "c90be7617e9aba5a70dc8d625e654c1c712403aaf47e7734497fc0e909e8f788";
  1292	    const EXPECTED_HEX_REUSE: &str =
  1293	        "8bb33232b7c20a63a206f505179b0f64fa50acb41061aaa471ba8e4435593aed";
  1294	    const EXPECTED_HEX_FINALIZE_REWARD: &str =
  1295	        "0f5e213ec919f8e61dc998b13a4dcd49ff6e81e473850725f2ca1f27c1d65a2d";
  1296	    const EXPECTED_HEX_TASK_EXPIRE: &str =
  1297	        "835cdec950a7fd09531e03b1ab2f571ccc9a7c05b3a3e04905f0dc77078c2d60";
  1298	    const EXPECTED_HEX_TERMINAL_SUMMARY: &str =
  1299	        "f05983df19cb2af951d79216d71a64aae6b1ae960d036022f90f28039b059208";
  1300	
  1301	    #[test]
  1302	    fn golden_work_tx_digest() {
  1303	        let actual = digest_hex(&TypedTx::Work(fixture_work_tx()));
  1304	        assert_eq!(actual.len(), 64);
  1305	        assert_eq!(actual, EXPECTED_HEX_WORK, "Work canonical digest changed");
  1306	    }
  1307	
  1308	    #[test]
  1309	    fn golden_verify_tx_digest() {
  1310	        let actual = digest_hex(&TypedTx::Verify(fixture_verify_tx()));
  1311	        assert_eq!(actual, EXPECTED_HEX_VERIFY);
  1312	    }
  1313	
  1314	    #[test]
  1315	    fn golden_challenge_tx_digest() {
  1316	        let actual = digest_hex(&TypedTx::Challenge(fixture_challenge_tx()));
  1317	        assert_eq!(actual, EXPECTED_HEX_CHALLENGE);
  1318	    }
  1319	
  1320	    #[test]
  1321	    fn golden_reuse_tx_digest() {
  1322	        let actual = digest_hex(&TypedTx::Reuse(fixture_reuse_tx()));
  1323	        assert_eq!(actual, EXPECTED_HEX_REUSE);
  1324	    }
  1325	
  1326	    #[test]
  1327	    fn golden_finalize_reward_tx_digest() {
  1328	        let actual = digest_hex(&TypedTx::FinalizeReward(fixture_finalize_reward_tx()));
  1329	        assert_eq!(actual, EXPECTED_HEX_FINALIZE_REWARD);
  1330	    }
  1331	
  1332	    #[test]
  1333	    fn golden_task_expire_tx_digest() {
  1334	        let actual = digest_hex(&TypedTx::TaskExpire(fixture_task_expire_tx()));
  1335	        assert_eq!(actual, EXPECTED_HEX_TASK_EXPIRE);
  1336	    }
  1337	
  1338	    #[test]
  1339	    fn golden_terminal_summary_tx_digest() {
  1340	        let actual = digest_hex(&TypedTx::TerminalSummary(fixture_terminal_summary_tx()));
  1341	        assert_eq!(actual, EXPECTED_HEX_TERMINAL_SUMMARY);
  1342	    }
  1343	}

exec
/bin/bash -lc "nl -ba src/bottom_white/ledger/system_keypair.rs | sed -n '160,260p;520,590p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   160	impl RejectedAttemptSummary {
   161	    /// TRACE_MATRIX FC1-Sig: construct a typed rejected-attempt summary, never a free-form sign blob.
   162	    pub fn new(
   163	        run_id: impl Into<String>,
   164	        attempt_id: impl Into<String>,
   165	        failure_class: impl Into<String>,
   166	        summary_hash: [u8; 32],
   167	    ) -> Self {
   168	        Self {
   169	            run_id: run_id.into(),
   170	            attempt_id: attempt_id.into(),
   171	            failure_class: failure_class.into(),
   172	            summary_hash,
   173	        }
   174	    }
   175	}
   176	
   177	// TRACE_MATRIX CO1.1.4-pre1 v1.1 round-1 closure (C-3 / Codex Q-C):
   178	// the typed `TerminalSummaryTx` struct (8-field per STATE § 1.5) now lives in
   179	// `state::typed_tx`. system_keypair signs an opaque digest via the
   180	// `CanonicalMessage::TerminalSummarySigning([u8; 32])` variant — same
   181	// opaque-digest pattern as `LedgerEntrySigning`, avoiding `bottom_white ↔ state`
   182	// circular dependency.
   183	
   184	/// TRACE_MATRIX FC3-Sig: typed continuity statement for system key rotation.
   185	#[derive(Debug, Clone, PartialEq, Eq)]
   186	pub struct EpochRotationProof {
   187	    old_epoch: SystemEpoch,
   188	    new_epoch: SystemEpoch,
   189	    old_pubkey: SystemPublicKey,
   190	    new_pubkey: SystemPublicKey,
   191	    signed_at_unix: u64,
   192	}
   193	
   194	impl EpochRotationProof {
   195	    /// TRACE_MATRIX FC3-Sig: construct a typed epoch-rotation continuity proof.
   196	    pub const fn new(
   197	        old_epoch: SystemEpoch,
   198	        new_epoch: SystemEpoch,
   199	        old_pubkey: SystemPublicKey,
   200	        new_pubkey: SystemPublicKey,
   201	        signed_at_unix: u64,
   202	    ) -> Self {
   203	        Self {
   204	            old_epoch,
   205	            new_epoch,
   206	            old_pubkey,
   207	            new_pubkey,
   208	            signed_at_unix,
   209	        }
   210	    }
   211	
   212	    /// TRACE_MATRIX FC3-Sig: old signing epoch certified by the rotation proof.
   213	    pub const fn old_epoch(&self) -> SystemEpoch {
   214	        self.old_epoch
   215	    }
   216	
   217	    /// TRACE_MATRIX FC3-Sig: new signing epoch certified by the rotation proof.
   218	    pub const fn new_epoch(&self) -> SystemEpoch {
   219	        self.new_epoch
   220	    }
   221	}
   222	
   223	/// TRACE_MATRIX FC1-Sig+FC3-Sig: only typed runtime messages may enter signature verification.
   224	#[derive(Debug, Clone, PartialEq, Eq)]
   225	pub enum CanonicalMessage {
   226	    /// TRACE_MATRIX FC1-Sig: predicate-runner rejection summary.
   227	    RejectedAttemptSummary(RejectedAttemptSummary),
   228	    /// TRACE_MATRIX FC1-Sig+FC3-Sig (CO1.1.4-pre1 v1.1 closure C-3): terminal
   229	    /// summary signing-payload digest. Opaque `[u8; 32]` — full canonical_digest
   230	    /// of the 8-field `state::typed_tx::TerminalSummaryTx` is computed in
   231	    /// typed_tx; this variant only carries the 32-byte digest into the typed
   232	    /// sign API. Same opaque-digest pattern as `LedgerEntrySigning`; avoids a
   233	    /// circular `system_keypair ↔ state` module dependency.
   234	    TerminalSummarySigning([u8; 32]),
   235	    /// TRACE_MATRIX FC3-Sig: system key epoch continuity proof.
   236	    EpochRotationProof(EpochRotationProof),
   237	    /// TRACE_MATRIX FC2-Append (CO1.7 v1.2 round-2 closure C3): L4 transition_ledger
   238	    /// signing payload digest. Opaque [u8; 32] — full canonical_digest of
   239	    /// `LedgerEntrySigningPayload` is computed in `transition_ledger`; this variant
   240	    /// only carries the 32-byte digest into the typed sign API. Avoids a circular
   241	    /// `system_keypair ↔ transition_ledger` module dependency while preserving the
   242	    /// "all sign goes through CanonicalMessage" invariant.
   243	    LedgerEntrySigning([u8; 32]),
   244	}
   245	
   246	/// TRACE_MATRIX FC1-Sig+FC3-Sig: epoch-indexed public keys pinned by genesis and rotation history.
   247	#[derive(Debug, Clone, Default, PartialEq, Eq)]
   248	pub struct PinnedSystemPubkeys {
   249	    keys: BTreeMap<SystemEpoch, SystemPublicKey>,
   250	}
   251	
   252	impl PinnedSystemPubkeys {
   253	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: create an empty pinned system-key map.
   254	    pub fn new() -> Self {
   255	        Self::default()
   256	    }
   257	
   258	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: pin a public key for a system epoch.
   259	    pub fn insert(
   260	        &mut self,
   520	    // verify creator PGP signatures against the pinned creator public key.
   521	    Ok(())
   522	}
   523	
   524	/// TRACE_MATRIX FC1-Sig: crate-only signing surface for the predicate runner.
   525	pub(crate) mod predicate_runner {
   526	    use super::{
   527	        sign_system_message_inner, CanonicalMessage, Ed25519Keypair, KeypairError,
   528	        RejectedAttemptSummary, SystemSignature,
   529	    };
   530	
   531	    /// TRACE_MATRIX FC1-Sig: sign only typed rejected-attempt summaries from the predicate runner.
   532	    pub(crate) fn sign_rejected_attempt_summary(
   533	        keypair: &Ed25519Keypair,
   534	        summary: &RejectedAttemptSummary,
   535	    ) -> Result<SystemSignature, KeypairError> {
   536	        sign_system_message_inner(
   537	            keypair,
   538	            &CanonicalMessage::RejectedAttemptSummary(summary.clone()),
   539	        )
   540	    }
   541	
   542	    /// TRACE_MATRIX FC1-Sig: sign only typed canonical messages within the predicate-runner scope.
   543	    pub(crate) fn sign_system_message(
   544	        keypair: &Ed25519Keypair,
   545	        message: &CanonicalMessage,
   546	    ) -> Result<SystemSignature, KeypairError> {
   547	        sign_system_message_inner(keypair, message)
   548	    }
   549	}
   550	
   551	/// TRACE_MATRIX FC1-Sig+FC3-Sig: crate-only signing surface for terminal summary emission.
   552	///
   553	/// **CO1.1.4-pre1 v1.1 round-1 closure (C-3)**: signs an opaque `[u8; 32]`
   554	/// digest produced by `state::typed_tx::TerminalSummaryTx::canonical_digest()`
   555	/// (same opaque-digest pattern as `transition_ledger_emitter::sign_ledger_entry`)
   556	/// rather than the typed struct directly — keeps `system_keypair` oblivious
   557	/// to the typed-tx schema, no `bottom_white ↔ state` circular dep.
   558	pub(crate) mod terminal_summary_emitter {
   559	    use super::{
   560	        sign_system_message_inner, CanonicalMessage, Ed25519Keypair, EpochRotationProof,
   561	        KeypairError, SystemSignature,
   562	    };
   563	
   564	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: sign an opaque 32-byte digest of a
   565	    /// terminal-summary signing payload (computed by typed_tx).
   566	    pub(crate) fn sign_terminal_summary(
   567	        keypair: &Ed25519Keypair,
   568	        digest: [u8; 32],
   569	    ) -> Result<SystemSignature, KeypairError> {
   570	        sign_system_message_inner(keypair, &CanonicalMessage::TerminalSummarySigning(digest))
   571	    }
   572	
   573	    /// TRACE_MATRIX FC3-Sig: sign only typed epoch rotation proofs.
   574	    pub(crate) fn sign_epoch_rotation_proof(
   575	        keypair: &Ed25519Keypair,
   576	        proof: &EpochRotationProof,
   577	    ) -> Result<SystemSignature, KeypairError> {
   578	        sign_system_message_inner(
   579	            keypair,
   580	            &CanonicalMessage::EpochRotationProof(proof.clone()),
   581	        )
   582	    }
   583	
   584	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: sign only typed canonical messages within terminal-summary scope.
   585	    pub(crate) fn sign_system_message(
   586	        keypair: &Ed25519Keypair,
   587	        message: &CanonicalMessage,
   588	    ) -> Result<SystemSignature, KeypairError> {
   589	        sign_system_message_inner(keypair, message)
   590	    }

exec
/bin/bash -lc "nl -ba src/bottom_white/ledger/system_keypair.rs | sed -n '430,520p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   430	
   431	/// TRACE_MATRIX FC1-Sig+FC3-Sig: decrypt an existing encrypted system keypair keystore.
   432	pub fn load_existing_keypair(
   433	    keystore_path: &Path,
   434	    user_kdf_password: &SecretString,
   435	) -> Result<Ed25519Keypair, KeypairError> {
   436	    let bytes = fs::read(keystore_path)?;
   437	    let encoded = EncryptedKeypair::decode(&bytes)?;
   438	    let mut key = derive_key(user_kdf_password, &encoded.salt, encoded.kdf)?;
   439	    let cipher = ChaCha20Poly1305::new_from_slice(&key)
   440	        .map_err(|_| KeypairError::Crypto("bad cipher key"))?;
   441	    let plaintext = cipher
   442	        .decrypt(
   443	            Nonce::from_slice(&encoded.nonce),
   444	            encoded.ciphertext.as_ref(),
   445	        )
   446	        .map_err(|_| KeypairError::Crypto("keystore authentication failed"))?;
   447	    key.zeroize();
   448	    Ed25519Keypair::from_plaintext(&plaintext)
   449	}
   450	
   451	/// TRACE_MATRIX FC1-Sig+FC3-Sig: canonical SHA-256 digest for typed system messages.
   452	pub fn canonical_digest(message: &CanonicalMessage) -> [u8; 32] {
   453	    let mut h = Sha256::new();
   454	    h.update(b"turingosv4.system_keypair.v1");
   455	    match message {
   456	        CanonicalMessage::RejectedAttemptSummary(summary) => {
   457	            h.update(b"RejectedAttemptSummary");
   458	            update_len_prefixed(&mut h, summary.run_id.as_bytes());
   459	            update_len_prefixed(&mut h, summary.attempt_id.as_bytes());
   460	            update_len_prefixed(&mut h, summary.failure_class.as_bytes());
   461	            h.update(summary.summary_hash);
   462	        }
   463	        CanonicalMessage::TerminalSummarySigning(digest) => {
   464	            h.update(b"TerminalSummarySigning");
   465	            h.update(digest);
   466	        }
   467	        CanonicalMessage::EpochRotationProof(proof) => {
   468	            h.update(b"EpochRotationProof");
   469	            h.update(proof.old_epoch.get().to_be_bytes());
   470	            h.update(proof.new_epoch.get().to_be_bytes());
   471	            h.update(proof.old_pubkey.as_bytes());
   472	            h.update(proof.new_pubkey.as_bytes());
   473	            h.update(proof.signed_at_unix.to_be_bytes());
   474	        }
   475	        CanonicalMessage::LedgerEntrySigning(digest) => {
   476	            h.update(b"LedgerEntrySigning");
   477	            h.update(digest);
   478	        }
   479	    }
   480	    h.finalize().into()
   481	}
   482	
   483	/// TRACE_MATRIX FC1-Sig+FC3-Sig: public system signature verification against pinned epoch keys.
   484	pub fn verify_system_signature(
   485	    sig: &SystemSignature,
   486	    message: &CanonicalMessage,
   487	    epoch: SystemEpoch,
   488	    pinned_pubkeys: &PinnedSystemPubkeys,
   489	) -> bool {
   490	    let Some(pk) = pinned_pubkeys.get(epoch) else {
   491	        return false;
   492	    };
   493	    let Ok(verifying_key) = VerifyingKey::from_bytes(pk.as_bytes()) else {
   494	        return false;
   495	    };
   496	    let signature = Signature::from_bytes(sig.as_bytes());
   497	    verifying_key
   498	        .verify(&canonical_digest(message), &signature)
   499	        .is_ok()
   500	}
   501	
   502	/// TRACE_MATRIX FC3-Sig: verify old and new signatures over a rotation continuity proof.
   503	pub fn verify_epoch_rotation_proof(
   504	    proof: &EpochRotationProof,
   505	    old_signature: &SystemSignature,
   506	    new_signature: &SystemSignature,
   507	    pinned_pubkeys: &PinnedSystemPubkeys,
   508	) -> bool {
   509	    let message = CanonicalMessage::EpochRotationProof(proof.clone());
   510	    verify_system_signature(old_signature, &message, proof.old_epoch(), pinned_pubkeys)
   511	        && verify_system_signature(new_signature, &message, proof.new_epoch(), pinned_pubkeys)
   512	}
   513	
   514	/// TRACE_MATRIX FC3-Sig: boot extension stub for genesis `[system_pubkeys]` verification.
   515	pub fn verify_system_pubkeys(genesis_payload_toml: &str) -> Result<(), TrustRootError> {
   516	    if !has_toml_section(genesis_payload_toml, "system_pubkeys") {
   517	        return Ok(());
   518	    }
   519	    // TODO(CO1.7): parse genesis_payload.toml [system_pubkeys] entries and
   520	    // verify creator PGP signatures against the pinned creator public key.

exec
/bin/bash -lc "nl -ba src/state/mod.rs | sed -n '1,80p' && nl -ba src/economy/money.rs | sed -n '1,220p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	//! TRACE_MATRIX Art 0.1: 四要素映射 (Tape / Input-Tape / Q / State).
     2	//! TRACE_MATRIX Art 0.4: Q_t version-controlled state vector.
     3	//! TRACE_MATRIX WP § 4: 9-component system state Q_t.
     4	//! TRACE_MATRIX WP § 0 axiom 1: state monotonicity.
     5	//!
     6	//! Atom: CO1.2 (Q_t struct) — implements `STATE_TRANSITION_SPEC v1.4 § 1.1`.
     7	//! All public re-exports below are surface for the same TRACE_MATRIX rows.
     8	
     9	/// TRACE_MATRIX Art 0.4 / WP § 4 — Q_t module: implements all 9 system state fields.
    10	pub mod q_state;
    11	
    12	/// TRACE_MATRIX FC2-Submit / CO1.1.4-pre1 — typed-tx ABI surface (TypedTx + per-kind structs).
    13	pub mod typed_tx;
    14	
    15	pub use q_state::{
    16	    AgentId, AgentSwarmState, AgentVisibleProjection, BalancesIndex, BudgetSnapshot,
    17	    ChallengeCase, ChallengeCasesIndex, ClaimEntry, ClaimsIndex, EconomicState, EscrowEntry,
    18	    EscrowsIndex, Hash, NodeId, PerAgentState, PriceIndex, QState, Reputation, ReputationsIndex,
    19	    RoyaltyEdge, RoyaltyGraph, StakeEntry, StakesIndex, TaskMarketEntry, TaskMarketsIndex, TxId,
    20	};
    21	
    22	pub use typed_tx::{
    23	    AgentSignature, BoolWithProof, ChallengeSigningPayload, ChallengeTx, ClaimId,
    24	    FinalizeRewardSigningPayload, FinalizeRewardTx, HasSubmitter, PredicateId,
    25	    PredicateResultsBundle, ReadKey, RejectionClass, ReuseTx, RunId, RunOutcome,
    26	    SafetyOrCreation, SignalBundle, SignalKind, SlashEvidenceCid, TaskExpireSigningPayload,
    27	    TaskExpireTx, TaskId, TerminalSummarySigningPayload, TerminalSummaryTx, ToolId,
    28	    TransitionError, TxStatus, TypedTx, VerifySigningPayload, VerifyTx, VerifyVerdict,
    29	    WorkSigningPayload, WorkTx, WriteKey,
    30	};
     1	//! `MicroCoin(i64)` — v4 monetary unit per Plan v3.2-fix3 CO1.0a + STATE_TRANSITION_SPEC v1.3.
     2	//!
     3	//! Constitution authority:
     4	//! - Laws 基本法 1 (Coin 守恒): monetary conservation MUST be exact
     5	//! - Inv 3 (escrow only): payouts come from pre-locked escrow; integer arithmetic prevents drift
     6	//! - Inv 4 (no post-init mint): mint API guarded; only genesis sets initial supply
     7	//!
     8	//! Spec authority:
     9	//! - STATE_TRANSITION_SPEC v1.3 § 1 typed schemas — all monetary fields are MicroCoin
    10	//! - § 2 hidden-input table: f64 BANNED in `src/economy/`
    11	//! - § 3.4 finalize_reward stage 3c royalty math: `royalty_micro = reward_micro * weight_micro / 1_000_000` (integer floor)
    12	//!
    13	//! Unit: 1 MicroCoin = 10⁻⁶ base coin. Range: i64 = ±9.2 × 10¹⁸ micro = ±9.2 × 10¹² base coin.
    14	//!
    15	//! Design:
    16	//! - Newtype around i64 to prevent accidental mixing with u64/u32/f64
    17	//! - All arithmetic returns Option (checked); panics not allowed in production paths
    18	//! - Display formats as base.fraction (e.g., "12.345678 coin")
    19	//! - serde-compatible for L4 transition_tx serialization
    20	//! - Hash + Ord + Eq for use as BTreeMap key (per § 2 I-BTREE)
    21	//!
    22	//! /// TRACE_MATRIX I-MICROCOIN + Inv-3 + Inv-4: monetary type for v4
    23	
    24	use serde::{Deserialize, Serialize};
    25	use std::fmt;
    26	
    27	/// A monetary value in micro-coin (10⁻⁶ base coin) as a signed 64-bit integer.
    28	///
    29	/// Negative values are allowed at the type level (e.g., signed deltas in tests),
    30	/// but balance / escrow / stake fields enforce non-negative invariants at the
    31	/// business logic layer (not in this type).
    32	#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
    33	#[serde(transparent)]
    34	pub struct MicroCoin(i64);
    35	
    36	/// 1 base coin in micro-units (= 10⁶).
    37	pub const MICRO_PER_COIN: i64 = 1_000_000;
    38	
    39	impl MicroCoin {
    40	    /// Construct from raw micro-units (signed).
    41	    pub const fn from_micro_units(micro: i64) -> Self {
    42	        Self(micro)
    43	    }
    44	
    45	    /// Construct from whole base coin (multiplied by `MICRO_PER_COIN`); `None` on overflow.
    46	    pub fn from_coin(coin: i64) -> Option<Self> {
    47	        coin.checked_mul(MICRO_PER_COIN).map(Self)
    48	    }
    49	
    50	    /// Zero (additive identity).
    51	    pub const fn zero() -> Self {
    52	        Self(0)
    53	    }
    54	
    55	    /// Get raw micro-units (signed i64).
    56	    pub const fn micro_units(self) -> i64 {
    57	        self.0
    58	    }
    59	
    60	    /// Whole base-coin component (truncates toward zero).
    61	    pub const fn coin_component(self) -> i64 {
    62	        self.0 / MICRO_PER_COIN
    63	    }
    64	
    65	    /// Fractional micro component in `[-999_999, 999_999]` (sign matches whole).
    66	    pub const fn micro_fraction_component(self) -> i64 {
    67	        self.0 % MICRO_PER_COIN
    68	    }
    69	
    70	    /// Checked addition. `None` on overflow.
    71	    pub const fn checked_add(self, rhs: Self) -> Option<Self> {
    72	        match self.0.checked_add(rhs.0) {
    73	            Some(v) => Some(Self(v)),
    74	            None => None,
    75	        }
    76	    }
    77	
    78	    /// Checked subtraction. `None` on overflow.
    79	    pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
    80	        match self.0.checked_sub(rhs.0) {
    81	            Some(v) => Some(Self(v)),
    82	            None => None,
    83	        }
    84	    }
    85	
    86	    /// Royalty / weight multiplication with integer-floor rounding.
    87	    ///
    88	    /// Implements the spec § 3.4 stage 3c rule:
    89	    ///     royalty_micro = reward_micro × weight_micro_fraction / 1_000_000
    90	    ///
    91	    /// `weight_micro_fraction` is interpreted as a rational in `[0.0, 1.0]`
    92	    /// scaled to micro-units (`1_000_000` = 1.0). Returns `None` on overflow
    93	    /// at the intermediate `reward_micro × weight` product step.
    94	    ///
    95	    /// Determinism: integer floor (`a / b` rounds toward zero in Rust). For
    96	    /// non-negative inputs this is round-down (floor). Negative values are
    97	    /// rejected (returns `None`) to keep monetary math non-negative-by-default.
    98	    pub fn checked_mul_floor_micro(self, weight_micro_fraction: i64) -> Option<Self> {
    99	        if self.0 < 0 || weight_micro_fraction < 0 {
   100	            return None;
   101	        }
   102	        if weight_micro_fraction > MICRO_PER_COIN {
   103	            // weight > 1.0 not allowed at type level
   104	            return None;
   105	        }
   106	        let prod = self.0.checked_mul(weight_micro_fraction)?;
   107	        Some(Self(prod / MICRO_PER_COIN))
   108	    }
   109	
   110	    /// True if value is zero.
   111	    pub const fn is_zero(self) -> bool {
   112	        self.0 == 0
   113	    }
   114	
   115	    /// True if value is strictly positive.
   116	    pub const fn is_positive(self) -> bool {
   117	        self.0 > 0
   118	    }
   119	
   120	    /// True if value is strictly negative.
   121	    pub const fn is_negative(self) -> bool {
   122	        self.0 < 0
   123	    }
   124	}
   125	
   126	impl fmt::Display for MicroCoin {
   127	    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
   128	        let coin = self.0 / MICRO_PER_COIN;
   129	        let frac = (self.0 % MICRO_PER_COIN).abs();
   130	        if self.0 < 0 && coin == 0 {
   131	            write!(f, "-0.{:06} coin", frac)
   132	        } else {
   133	            write!(f, "{}.{:06} coin", coin, frac)
   134	        }
   135	    }
   136	}
   137	
   138	// ────────────────────────────────────────────────────────────────────────────
   139	// StakeMicroCoin — newtype for stake fields per CO1.1.4-pre1 § 3
   140	// ────────────────────────────────────────────────────────────────────────────
   141	
   142	/// Newtype on `MicroCoin` for `WorkTx::stake`, `VerifyTx::bond`, `ChallengeTx::stake`
   143	/// fields. Non-negative is a runtime invariant per Inv 3 (escrow only); the
   144	/// type-level newtype prevents accidental mixing with general-purpose
   145	/// `MicroCoin` (e.g. crediting a balance with a stake amount or vice versa).
   146	///
   147	/// `#[serde(transparent)]` — wire format identical to `MicroCoin`, so adding
   148	/// the newtype is non-breaking for canonical encoding.
   149	///
   150	/// /// TRACE_MATRIX I-MICROCOIN + I-STAKE: stake-typed monetary newtype.
   151	#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
   152	#[serde(transparent)]
   153	pub struct StakeMicroCoin(pub MicroCoin);
   154	
   155	impl Default for StakeMicroCoin {
   156	    fn default() -> Self {
   157	        Self::zero()
   158	    }
   159	}
   160	
   161	impl StakeMicroCoin {
   162	    pub const fn from_micro_units(micro: i64) -> Self {
   163	        Self(MicroCoin::from_micro_units(micro))
   164	    }
   165	    pub const fn zero() -> Self {
   166	        Self(MicroCoin::zero())
   167	    }
   168	    pub const fn micro_units(self) -> i64 {
   169	        self.0.micro_units()
   170	    }
   171	    pub const fn as_micro_coin(self) -> MicroCoin {
   172	        self.0
   173	    }
   174	}
   175	
   176	impl fmt::Display for StakeMicroCoin {
   177	    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
   178	        write!(f, "stake({})", self.0)
   179	    }
   180	}
   181	
   182	#[cfg(test)]
   183	mod tests {
   184	    use super::*;
   185	
   186	    #[test]
   187	    fn from_coin_round_trip() {
   188	        let m = MicroCoin::from_coin(5).unwrap();
   189	        assert_eq!(m.micro_units(), 5_000_000);
   190	        assert_eq!(m.coin_component(), 5);
   191	        assert_eq!(m.micro_fraction_component(), 0);
   192	    }
   193	
   194	    #[test]
   195	    fn from_micro_units_zero() {
   196	        let m = MicroCoin::from_micro_units(0);
   197	        assert!(m.is_zero());
   198	        assert!(!m.is_positive());
   199	        assert!(!m.is_negative());
   200	    }
   201	
   202	    #[test]
   203	    fn checked_add_normal() {
   204	        let a = MicroCoin::from_coin(10).unwrap();
   205	        let b = MicroCoin::from_coin(5).unwrap();
   206	        let sum = a.checked_add(b).unwrap();
   207	        assert_eq!(sum, MicroCoin::from_coin(15).unwrap());
   208	    }
   209	
   210	    #[test]
   211	    fn checked_sub_normal() {
   212	        let a = MicroCoin::from_coin(10).unwrap();
   213	        let b = MicroCoin::from_coin(3).unwrap();
   214	        let diff = a.checked_sub(b).unwrap();
   215	        assert_eq!(diff, MicroCoin::from_coin(7).unwrap());
   216	    }
   217	
   218	    #[test]
   219	    fn checked_add_overflow_returns_none() {
   220	        let a = MicroCoin::from_micro_units(i64::MAX);

exec
/bin/bash -lc "nl -ba src/bottom_white/ledger/transition_ledger.rs | sed -n '245,345p;560,635p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   245	        Ok(self.entries[(logical_t - 1) as usize].clone())
   246	    }
   247	
   248	    fn len(&self) -> u64 {
   249	        self.entries.len() as u64
   250	    }
   251	}
   252	
   253	// ────────────────────────────────────────────────────────────────────────────
   254	// § 4 replay — TWO-MODE per C1
   255	// ────────────────────────────────────────────────────────────────────────────
   256	
   257	/// **C1 NEW**: replay mode discriminator.
   258	/// - `ChainOnly`: skeleton-stage; chain integrity only (parent_state_root +
   259	///   parent_ledger_root + ledger_root chain). NOT the I-DETHASH witness.
   260	/// - `FullTransition`: CO1.7.5+ stage; verifies signatures + re-fetches payloads
   261	///   from CAS + re-runs pure transitions + asserts state_root match. THE
   262	///   I-DETHASH witness; requires CO1.4-extra (CAS index persistence).
   263	#[derive(Debug, Clone, Copy, PartialEq, Eq)]
   264	pub enum ReplayMode {
   265	    ChainOnly,
   266	    FullTransition,
   267	}
   268	
   269	#[derive(Debug)]
   270	pub enum ReplayError {
   271	    LogicalTGap { at: usize, expected: u64, got: u64 },
   272	    ParentStateMismatch { at: usize },
   273	    ParentLedgerMismatch { at: usize }, // K2 NEW
   274	    LedgerRootMismatch { at: usize },
   275	    // FullTransition-mode-only (CO1.7.5+):
   276	    BadSignature { at: usize },
   277	    CasMissing { at: usize },
   278	    StateRootMismatch { at: usize },
   279	}
   280	
   281	impl std::fmt::Display for ReplayError {
   282	    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
   283	        match self {
   284	            Self::LogicalTGap { at, expected, got } => {
   285	                write!(f, "logical_t gap at index {at}: expected {expected}, got {got}")
   286	            }
   287	            Self::ParentStateMismatch { at } => write!(f, "parent_state_root mismatch at index {at}"),
   288	            Self::ParentLedgerMismatch { at } => write!(f, "parent_ledger_root mismatch at index {at}"),
   289	            Self::LedgerRootMismatch { at } => write!(f, "ledger_root mismatch at index {at}"),
   290	            Self::BadSignature { at } => write!(f, "system_signature verify failed at index {at}"),
   291	            Self::CasMissing { at } => write!(f, "CAS payload not retrievable at index {at}"),
   292	            Self::StateRootMismatch { at } => write!(f, "resulting_state_root divergence at index {at}"),
   293	        }
   294	    }
   295	}
   296	impl std::error::Error for ReplayError {}
   297	
   298	/// Skeleton-stage entry point (v1.1).
   299	///
   300	/// Validates:
   301	/// 1. logical_t monotonicity (no gaps, no duplicates)
   302	/// 2. parent_state_root chain
   303	/// 3. parent_ledger_root chain (K2 transplant defense)
   304	/// 4. resulting_ledger_root recomputed via append(prev_ledger_root, signing_digest)
   305	///
   306	/// Does NOT verify:
   307	/// - system_signature (CO1.7.5+: requires CanonicalMessage extension wired through keypair)
   308	/// - resulting_state_root (CO1.7.5+: requires dispatch_transition + CO1.4-extra CAS persistence)
   309	///
   310	/// Returns final (state_root, ledger_root) on success.
   311	pub fn replay_chain_integrity(
   312	    genesis_state_root: Hash,
   313	    genesis_ledger_root: Hash,
   314	    entries: &[LedgerEntry],
   315	) -> Result<(Hash, Hash), ReplayError> {
   316	    let mut prev_state_root = genesis_state_root;
   317	    let mut prev_ledger_root = genesis_ledger_root;
   318	
   319	    for (i, entry) in entries.iter().enumerate() {
   320	        let expected_logical_t = (i as u64) + 1;
   321	        if entry.logical_t != expected_logical_t {
   322	            return Err(ReplayError::LogicalTGap {
   323	                at: i,
   324	                expected: expected_logical_t,
   325	                got: entry.logical_t,
   326	            });
   327	        }
   328	        if entry.parent_state_root != prev_state_root {
   329	            return Err(ReplayError::ParentStateMismatch { at: i });
   330	        }
   331	        // K2 NEW: parent_ledger_root chain check
   332	        if entry.parent_ledger_root != prev_ledger_root {
   333	            return Err(ReplayError::ParentLedgerMismatch { at: i });
   334	        }
   335	        let signing_digest = entry.to_signing_payload().canonical_digest();
   336	        let recomputed = append(&prev_ledger_root, &signing_digest);
   337	        if recomputed != entry.resulting_ledger_root {
   338	            return Err(ReplayError::LedgerRootMismatch { at: i });
   339	        }
   340	        prev_state_root = entry.resulting_state_root;
   341	        prev_ledger_root = entry.resulting_ledger_root;
   342	    }
   343	
   344	    Ok((prev_state_root, prev_ledger_root))
   345	}
   560	        let mut tb = repo
   561	            .treebuilder(None)
   562	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("treebuilder: {e}")))?;
   563	        let cid_blob = repo
   564	            .blob(&entry.tx_payload_cid.0)
   565	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("cid blob: {e}")))?;
   566	        tb.insert(TREE_BLOB_PAYLOAD_CID, cid_blob, 0o100644)
   567	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("tree insert cid: {e}")))?;
   568	        let sig_blob = repo
   569	            .blob(entry.system_signature.as_bytes())
   570	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("sig blob: {e}")))?;
   571	        tb.insert(TREE_BLOB_SIGNATURE, sig_blob, 0o100644)
   572	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("tree insert sig: {e}")))?;
   573	        let entry_blob = repo
   574	            .blob(&canonical)
   575	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("entry blob: {e}")))?;
   576	        tb.insert(TREE_BLOB_ENTRY_CANONICAL, entry_blob, 0o100644)
   577	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("tree insert entry: {e}")))?;
   578	        let tree_oid = tb
   579	            .write()
   580	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("tree write: {e}")))?;
   581	        let tree = repo
   582	            .find_tree(tree_oid)
   583	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("find_tree: {e}")))?;
   584	
   585	        // Determinism: time = (logical_t, 0). NO wall clock.
   586	        let time = git2::Time::new(entry.logical_t as i64, 0);
   587	        let author = GitSignature::new("turingosv4 sequencer", "system@turingos", &time)
   588	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("git sig: {e}")))?;
   589	        let committer = author.clone();
   590	
   591	        let parents: Vec<git2::Commit<'_>> = match self.head_oid {
   592	            Some(oid) => vec![repo.find_commit(oid).map_err(|e| {
   593	                LedgerWriterError::BackendCorruption(format!("parent commit: {e}"))
   594	            })?],
   595	            None => Vec::new(),
   596	        };
   597	        let parent_refs: Vec<&git2::Commit<'_>> = parents.iter().collect();
   598	        let message = format!("transition logical_t={}\n", entry.logical_t);
   599	        let new_oid = repo
   600	            .commit(
   601	                Some(TRANSITIONS_REF),
   602	                &author,
   603	                &committer,
   604	                &message,
   605	                &tree,
   606	                &parent_refs,
   607	            )
   608	            .map_err(|e| LedgerWriterError::BackendCorruption(format!("commit: {e}")))?;
   609	
   610	        self.head_oid = Some(new_oid);
   611	        self.len += 1;
   612	        Ok(entry.resulting_ledger_root)
   613	    }
   614	
   615	    fn read_at(&self, logical_t: u64) -> Result<LedgerEntry, LedgerWriterError> {
   616	        let bytes = self.read_canonical_bytes(logical_t)?;
   617	        canonical_decode::<LedgerEntry>(&bytes).map_err(|e| {
   618	            LedgerWriterError::BackendCorruption(format!("canonical_decode at {logical_t}: {e}"))
   619	        })
   620	    }
   621	
   622	    fn len(&self) -> u64 {
   623	        self.len
   624	    }
   625	}
   626	
   627	// ────────────────────────────────────────────────────────────────────────────
   628	// Tests — 8 conformance items (4 NEW vs v1 skeleton: K2 / Q9 / repr(u8) / extensions)
   629	// ────────────────────────────────────────────────────────────────────────────
   630	
   631	#[cfg(test)]
   632	mod tests {
   633	    use super::*;
   634	
   635	    fn h(byte: u8) -> Hash {

exec
/bin/bash -lc "nl -ba src/bottom_white/ledger/transition_ledger.rs | sed -n '1,160p;346,390p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	//! L4 Transition Ledger (CO1.7) — type skeleton + pure helpers.
     2	//!
     3	//! TRACE_MATRIX FC2-Append: canonical envelope appended to L4 once a transition is accepted.
     4	//! TRACE_MATRIX WP § 5.L4: ChainTape Layer 4 spine; one LedgerEntry per accepted transition.
     5	//! TRACE_MATRIX § 1 (CO1_7_TRANSITION_LEDGER_v1_2026-04-28 v1.1): schema + append() + replay_chain_integrity() pseudocode.
     6	//!
     7	//! **Status**: v1.1 type skeleton — round-1 dual audit returned CHALLENGE/CHALLENGE; this
     8	//! version closes 11 must-fix items (C1/C2/C3 + K1-K7 + G1 + D1). Awaiting round-2.
     9	//! All bodies that depend on yet-to-implement transition functions or CAS index
    10	//! persistence are stubbed; full-mode replay is deferred to CO1.7.5+.
    11	//!
    12	//! v1 → v1.1 changes (smoke for round-2 dual audit):
    13	//! - C1: two-mode replay enum (ChainOnly v1; FullTransition CO1.7.5+); skeleton now
    14	//!   exposes `replay_chain_integrity` only (renamed for honesty).
    15	//! - K1: sequencer dual-counter design — documented in spec § 3; skeleton has no
    16	//!   sequencer code (deferred to CO1.7.5).
    17	//! - K2: `parent_ledger_root: Hash` field added + bound in signing payload (transplant
    18	//!   defense); new test asserts replay rejects parent_ledger_root tamper.
    19	//! - K3: L4/L5 boundary clarified — CO1.7 owns ledger_root + commit-chain head_t;
    20	//!   CO1.8 owns state_root mutation. Skeleton reflects boundary (no state_root mutation).
    21	//! - K5: `TxKind::Slash` DROPPED for v4 (deferred to CO P2.5).
    22	//! - K6: `#[repr(u8)]` + explicit discriminants on TxKind.
    23	//! - K7: +2 conformance tests (parent_ledger_root tamper, digest exclusion).
    24	//! - G1: `extensions: BTreeMap<String, Vec<u8>>` forward-compat field (empty in v1).
    25	//! - C3 / Q8: signing target is `LedgerEntrySigningPayload` (separate struct) ready to
    26	//!   ride a `CanonicalMessage::LedgerEntrySigning(_)` variant when CO1.7.5+ extends
    27	//!   `system_keypair` (Wave 4-B additive extension). Skeleton has the payload struct
    28	//!   + canonical_digest method; the actual CanonicalMessage extension is deferred.
    29	//! - Q9: canonical_digest now lives on LedgerEntrySigningPayload, not LedgerEntry —
    30	//!   structurally enforces "derivatives excluded".
    31	//! - D1: epoch is bound in signing payload (Codex security wins over Gemini orthogonality).
    32	
    33	use std::collections::BTreeMap;
    34	use std::path::{Path, PathBuf};
    35	
    36	use git2::{ObjectType as Git2ObjectType, Repository, Signature as GitSignature};
    37	use serde::{Deserialize, Serialize};
    38	use sha2::{Digest, Sha256};
    39	
    40	use crate::bottom_white::cas::schema::Cid;
    41	use crate::bottom_white::ledger::system_keypair::{SystemEpoch, SystemSignature};
    42	use crate::state::q_state::Hash;
    43	
    44	// ────────────────────────────────────────────────────────────────────────────
    45	// § 1 LedgerEntry — the stored record (11 fields per v1.1)
    46	// ────────────────────────────────────────────────────────────────────────────
    47	
    48	/// TRACE_MATRIX FC2-Append: discriminator for the typed payload behind a CAS Cid.
    49	/// **K6**: `#[repr(u8)]` + explicit discriminants for stable cast in canonical digest.
    50	/// **K5**: NO `Slash` variant — ChallengeCourt slash event deferred to CO P2.5 atom.
    51	#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    52	#[repr(u8)]
    53	pub enum TxKind {
    54	    Work            = 0,
    55	    Verify          = 1,
    56	    Challenge       = 2,
    57	    Reuse           = 3,
    58	    FinalizeReward  = 4,
    59	    TaskExpire      = 5,
    60	    TerminalSummary = 6,
    61	}
    62	
    63	/// TRACE_MATRIX FC2-Append + WP § 5.L4: stored LedgerEntry record (11 fields).
    64	///
    65	/// Distinct from `LedgerEntrySigningPayload`: this is the FULL stored record
    66	/// (includes derivatives + signature); the signing payload is the subset that
    67	/// the system keypair attests.
    68	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    69	pub struct LedgerEntry {
    70	    /// **K1**: assigned ONLY at commit (sequencer dual-counter design); rejected
    71	    /// submissions never get a logical_t.
    72	    pub logical_t: u64,                          //  1
    73	    pub parent_state_root: Hash,                 //  2
    74	    /// **K2 NEW**: parent_ledger_root before fold; bound in signed payload to
    75	    /// prevent transplant attacks.
    76	    pub parent_ledger_root: Hash,                //  3
    77	    pub tx_kind: TxKind,                         //  4
    78	    /// CAS handle (CO1.4) to canonical-serialized payload (DIV-5 5-param put).
    79	    pub tx_payload_cid: Cid,                     //  5
    80	    /// Resulting state_root post-transition (NOT mutated by L4 — accepted as
    81	    /// returned by transition function per K3 boundary).
    82	    pub resulting_state_root: Hash,              //  6
    83	    /// Resulting ledger_root after fold. Derivative; NOT in signed digest.
    84	    pub resulting_ledger_root: Hash,             //  7
    85	    pub timestamp_logical: u64,                  //  8
    86	    /// **D1 / Q10**: epoch bound in signed payload (Codex security wins).
    87	    pub epoch: SystemEpoch,                      //  9
    88	    /// **G1 NEW**: forward-compat extension map. Empty in v1; reserved for v4.x.
    89	    /// Bound in signed payload (G1 cannot bypass signature).
    90	    pub extensions: BTreeMap<String, Vec<u8>>,   // 10
    91	    /// Detached system signature over `LedgerEntrySigningPayload.canonical_digest()`.
    92	    pub system_signature: SystemSignature,       // 11
    93	}
    94	
    95	// ────────────────────────────────────────────────────────────────────────────
    96	// § 1.1 LedgerEntrySigningPayload — the signed bytes (NEW per C3 / Q9)
    97	// ────────────────────────────────────────────────────────────────────────────
    98	
    99	/// TRACE_MATRIX FC2-Append C3: the bytes the system keypair actually signs.
   100	///
   101	/// **Excludes** (Q9 cycle prevention):
   102	/// - `resulting_ledger_root` (derivative; including → cycle)
   103	/// - `system_signature` (its own input)
   104	///
   105	/// **Includes** (9 non-derivative bound fields). Domain-separation prefix is
   106	/// part of the digest to prevent cross-namespace collision.
   107	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
   108	pub struct LedgerEntrySigningPayload {
   109	    pub logical_t: u64,
   110	    pub parent_state_root: Hash,
   111	    pub parent_ledger_root: Hash,                  // K2
   112	    pub tx_kind: TxKind,
   113	    pub tx_payload_cid: Cid,
   114	    pub resulting_state_root: Hash,
   115	    pub timestamp_logical: u64,
   116	    pub epoch: SystemEpoch,                        // D1
   117	    pub extensions: BTreeMap<String, Vec<u8>>,     // G1
   118	}
   119	
   120	impl LedgerEntrySigningPayload {
   121	    /// Canonical SHA-256 digest. Stable wire format (NOT bincode/serde dependent).
   122	    pub fn canonical_digest(&self) -> Hash {
   123	        let mut h = Sha256::new();
   124	        h.update(b"turingosv4.ledger_entry_signing.v1");
   125	        h.update(self.logical_t.to_be_bytes());
   126	        h.update(self.parent_state_root.0);
   127	        h.update(self.parent_ledger_root.0);
   128	        h.update((self.tx_kind as u8).to_be_bytes()); // K6 #[repr(u8)] makes cast stable
   129	        h.update(self.tx_payload_cid.0);
   130	        h.update(self.resulting_state_root.0);
   131	        h.update(self.timestamp_logical.to_be_bytes());
   132	        h.update(self.epoch.get().to_be_bytes());
   133	        // Extensions: BTreeMap iterates in lex key order (deterministic);
   134	        // length-prefix every field to prevent ambiguity attacks.
   135	        h.update((self.extensions.len() as u64).to_be_bytes());
   136	        for (k, v) in &self.extensions {
   137	            h.update((k.len() as u64).to_be_bytes());
   138	            h.update(k.as_bytes());
   139	            h.update((v.len() as u64).to_be_bytes());
   140	            h.update(v);
   141	        }
   142	        Hash(h.finalize().into())
   143	    }
   144	}
   145	
   146	impl LedgerEntry {
   147	    /// Project the LedgerEntry's signed-fields-subset back into a signing payload.
   148	    /// Used by replay to recompute `signing_digest` and re-verify chain integrity.
   149	    pub fn to_signing_payload(&self) -> LedgerEntrySigningPayload {
   150	        LedgerEntrySigningPayload {
   151	            logical_t: self.logical_t,
   152	            parent_state_root: self.parent_state_root,
   153	            parent_ledger_root: self.parent_ledger_root,
   154	            tx_kind: self.tx_kind,
   155	            tx_payload_cid: self.tx_payload_cid,
   156	            resulting_state_root: self.resulting_state_root,
   157	            timestamp_logical: self.timestamp_logical,
   158	            epoch: self.epoch,
   159	            extensions: self.extensions.clone(),
   160	        }
   346	
   347	// ────────────────────────────────────────────────────────────────────────────
   348	// § 2.5 Canonical serialization (bincode v2; STATE_TRANSITION_SPEC § 2.5)
   349	// ────────────────────────────────────────────────────────────────────────────
   350	
   351	/// `bincode::config` used for the canonical `LedgerEntry` wire format.
   352	///
   353	/// **Frozen choices** (per STATE_TRANSITION_SPEC § 2.5):
   354	/// - **Big-endian** byte order (network order; deterministic across platforms).
   355	/// - **Fixed-int encoding** (no varint; fixed-width for byte-stable round-trip).
   356	/// - **`BTreeMap` keys**: bincode iterates the map in serde-supplied order; we
   357	///   only ever encode `BTreeMap` (sorted by construction) so key order is lex.
   358	/// - **No padding, no implicit alignment.**
   359	fn bincode_canonical_config() -> impl bincode::config::Config {
   360	    bincode::config::standard()
   361	        .with_big_endian()
   362	        .with_fixed_int_encoding()
   363	}
   364	
   365	/// Canonical encode any serde-Serialize value to bytes (CO1.7 wire format).
   366	/// Used by `Git2LedgerWriter` for commit-message bodies and by future callers
   367	/// needing byte-stable signatures over typed payloads.
   368	pub fn canonical_encode<T: Serialize>(value: &T) -> Result<Vec<u8>, CanonicalCodecError> {
   369	    bincode::serde::encode_to_vec(value, bincode_canonical_config())
   370	        .map_err(|e| CanonicalCodecError::Encode(e.to_string()))
   371	}
   372	
   373	/// Canonical decode the inverse of `canonical_encode`. Returns the value plus
   374	/// the number of bytes consumed (entire input must be consumed for a clean decode).
   375	pub fn canonical_decode<T: serde::de::DeserializeOwned>(
   376	    bytes: &[u8],
   377	) -> Result<T, CanonicalCodecError> {
   378	    let (value, consumed) =
   379	        bincode::serde::decode_from_slice::<T, _>(bytes, bincode_canonical_config())
   380	            .map_err(|e| CanonicalCodecError::Decode(e.to_string()))?;
   381	    if consumed != bytes.len() {
   382	        return Err(CanonicalCodecError::TrailingBytes {
   383	            consumed,
   384	            total: bytes.len(),
   385	        });
   386	    }
   387	    Ok(value)
   388	}
   389	
   390	#[derive(Debug)]

exec
/bin/bash -lc "nl -ba handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md | sed -n '1,120p;155,200p;232,290p'" in /home/zephryj/projects/turingosv4
 succeeded in 1ms:
     1	# CO1.1.4-pre1 — Typed Tx ABI Surface (v1.1)
     2	
     3	**Status**: v1.1 — round-1 dual audit returned CHALLENGE/CHALLENGE; this version closes 10 patches (P1-P10) per the merged verdict (`handover/audits/CO1_1_4_PRE1_DUAL_AUDIT_VERDICT_R1_2026-04-28.md`). Awaiting round-2.
     4	**Status (v1)**: v1 DRAFT, post-CO1.7 PASS/PASS gate (2026-04-28).
     5	**Author**: ArchitectAI (Claude); session 2026-04-28 (continued).
     6	**Round-1 verdicts**: `handover/audits/CODEX_CO1_1_4_PRE1_ROUND1_AUDIT_2026-04-28.md` (CHALLENGE/high) + `handover/audits/GEMINI_CO1_1_4_PRE1_ROUND1_AUDIT_2026-04-28.md` (CHALLENGE/high); merged in `handover/audits/CO1_1_4_PRE1_DUAL_AUDIT_VERDICT_R1_2026-04-28.md`.
     7	
     8	## v1.1 patch log (vs. v1) — round-1 closure
     9	
    10	| ID | v1 issue | v1.1 fix | Source |
    11	|---|---|---|---|
    12	| **P1** | AgentSignature reused 64-byte adapter without domain separation; comments implied "exclude signature" digest with no signing payload | NEW signing-payload structs (`WorkSigningPayload` / `VerifySigningPayload` / `ChallengeSigningPayload` / `FinalizeRewardSigningPayload` / `TaskExpireSigningPayload` / `TerminalSummarySigningPayload`) — each has explicit domain prefix (`b"turingosv4.<actor>.<purpose>.v1"`) prepended to bincode body bytes in `canonical_digest()`. Plus `to_signing_payload()` projection on each tx. | C-1 (Codex Q-E + Gemini Q7) |
    13	| **P2** | `FinalizeRewardTx.claim_id: TxId` reused TxId, leaking ClaimsIndex impl into wire format | New `ClaimId(pub TxId)` newtype with `#[serde(transparent)]` (wire-identical to TxId; non-breaking); `FinalizeRewardTx.claim_id: ClaimId` now | C-3 (Codex Q-B) |
    14	| **P3** | `TerminalSummaryTx` was 3-field placeholder living in `system_keypair.rs` (versus STATE § 1.5 8-field schema); locking the wrong shape into ABI | Migrated to `state::typed_tx::TerminalSummaryTx` with full 8-field STATE schema (tx_id / task_id / run_id / run_outcome / total_attempts / failure_class_histogram / last_logical_t / system_signature). `system_keypair` now signs an opaque `TerminalSummarySigning([u8; 32])` digest (same opaque-digest pattern as `LedgerEntrySigning`) — no `bottom_white ↔ state` circular dep. | C-3 (Codex Q-C must-fix-now) |
    15	| **P4** | `TransitionError` had only 10 variants; STATE § 3 pseudocode invokes ~22 | Expanded to 22 variants: SignatureInvalid / StakeInsufficient / TargetWorkTxNotFound / TargetWorkTxNotVerifiable / ParentNotAcceptedYet / AcceptancePredicateFailed(PredicateId) / VerificationPredicateFailed(PredicateId) / SettlementPredicateFailed(PredicateId) / ChallengeWindowClosed / CounterexampleInsufficient / ToolNotInRegistry / ToolCreatorMismatch + 10 prior. Plus `NotYetImplemented` retained as explicit stub sentinel. | CX-1 (Codex Q-G) |
    16	| **P5** | "Phase 1 record-only" golden fixture tests asserted only length=64 + self-stability, did NOT lock SHA-256 hex; `TerminalSummary` excluded from round-trip / kind / golden tests | Hardcoded SHA-256 hex constants for all 7 TypedTx fixture digests (Work / Verify / Challenge / Reuse / FinalizeReward / TaskExpire / TerminalSummary). NEW tests: cross-variant non-collision (7×7 pairwise distinct), BTreeSet permutation independence, default round-trip, signing-payload domain non-collision (6 distinct domain digests), signing-payload-excludes-signature (mutating tx.signature must NOT affect digest). All variants now in round-trip + kind-projection. Total typed_tx tests: 11 → 17. | C-2 (Codex Q-J + Gemini Q9) |
    17	| **P6** | STATE § 2.5 wording wrong vs actual codec — claimed `#[repr(u8)]`-controlled enum discriminants; bincode-2 actually emits u32 BE for variants and u64 BE for lengths | This v1.1 spec § 2.5-bis explicitly documents the actual codec behavior + cross-references bincode-2 source (`bincode 2.0.1 src/features/serde/ser.rs:186`, `enc/impls.rs:68 + :128`). `#[repr(u8)]` is a Rust language attribute that does NOT control serde wire format. Recommendation accepted: keep u32 variants + u64 lengths (no codec change; spec language fixed). | CX-2 (Codex Q-D) |
    18	| **P7** | D-3 TerminalSummaryTx field-set divergence | RESOLVED (P3 migrated to full schema). § 9 D-3 row removed. | C-3 followup |
    19	| **P8** | FinalizeRewardTx had ambiguous {task_id, solver, reward, royalty} provenance + redundant system_signature unclear | This spec § 4 explicitly states {task_id, solver, reward} are **Q-DERIVED at replay** (re-fetched from ClaimsIndex by claim_id; wire fields are ledger summary, NOT trusted from wire); `system_signature` is RETAINED with explicit dual-sign rationale (this sig binds the tx-payload bytes; the L4 `LedgerEntrySigningPayload` sig binds the sequencer-stamped envelope; both are needed). | C-3 + GM-2 |
    20	| **P9** | Cold-replay → Art 0.2 violation if CAS index not persisted | This spec § 0 NEW "Cross-Atom Ordering Gate": v1.1 PASS is contingent on CO1.4-extra (CAS index persistence) shipping BEFORE CO1.7-impl A4 (replay_full_transition). CO1.7-impl A2 (Sequencer apply path) and A3 (dispatch_transition stubs) may proceed; A4 BLOCKED on CO1.4-extra. | GM-1 (Gemini Q4) |
    21	| **P10** | TaskId-vs-TxId QState index mismatch (typed_tx uses TaskId; QState `task_markets_t` / `escrows_t` / `stakes_t` keyed by TxId) | This spec § 9 NEW D-4 documents the forward-migration plan: CO P2.1 (TaskMarket atom) owns the QState retrofit; v1.1 records the migration debt + cross-atom dependency note. Does NOT perform the retrofit (out of CO1.1.4-pre1 scope; would touch q_state.rs which is its own atom). | CX-3 (Codex Q-J) |
    22	
    23	10 patches integrated below.
    24	
    25	---
    26	
    27	
    28	**Why this atom exists**: spec § 2.5 of `STATE_TRANSITION_SPEC_v1_2026-04-27.md` explicitly deferred "full ABI surface for QState/SignalBundle/TransitionError" to CO1.7. CO1.7 spec § 0 places the per-kind tx schemas in `STATE_TRANSITION_SPEC § 1` ("frozen on paper, not yet in code"). When CO1.7-impl A1 (Git2LedgerWriter, commit `a03cc52`) shipped, downstream A2 (TypedTx + dispatch_transition) discovered ~30 supporting schema types are required but **none of them exist in code** — only `MicroCoin` is defined. This atom defines that ABI surface in isolation under its own dual-audit gate, per the project's per-atom audit principle (CLAUDE.md "Audit Standard").
    29	
    30	**Companion**: `STATE_TRANSITION_SPEC_v1_2026-04-27.md` § 1 (typed schemas), § 2.5 (canonical serialization), § 3 (transition pseudocode — informs FinalizeRewardTx schema, see § 4 below).
    31	
    32	**Single sentence**: define every supporting type + the 7 typed-tx variant payload structs + the `TypedTx` enum, with `Serialize/Deserialize` derives over the spec § 2.5 canonical encoding (bincode v2 BE + fixed_int), so that CO1.7-impl A2-A4 (Sequencer + dispatch_transition + replay_full_transition) can be implemented against a stable type surface.
    33	
    34	---
    35	
    36	## § 0 Scope
    37	
    38	### In scope
    39	
    40	1. **Identifier newtypes**: `TaskId`, `RunId`, `ToolId`, `PredicateId` (each opaque `String`).
    41	2. **Read/Write set keys**: `ReadKey(String)`, `WriteKey(String)`.
    42	3. **Agent signature**: `AgentSignature([u8; 64])` — Ed25519 detached signature, distinct from `SystemSignature` (system_keypair.rs).
    43	4. **Predicate result types**: `BoolWithProof`, `PredicateResultsBundle`, `SafetyOrCreation`.
    44	5. **Status / class enums**: `TxStatus`, `RejectionClass`, `VerifyVerdict`, `RunOutcome`.
    45	6. **Slash evidence reference**: `SlashEvidenceCid(Cid)` newtype.
    46	7. **Money newtype**: `StakeMicroCoin(MicroCoin)` (non-negative invariant enforced at business layer; type-level newtype prevents accidental mix with general `MicroCoin`).
    47	8. **Typed-tx payload structs**: `WorkTx`, `VerifyTx`, `ChallengeTx`, `ReuseTx`, `FinalizeRewardTx`, `TaskExpireTx`. (`TerminalSummaryTx` already exists in `system_keypair.rs`.)
    48	9. **Outer enum**: `pub enum TypedTx` with the 7 variants.
    49	10. **Trait**: `pub trait HasSubmitter` per STATE spec § 3.6.5 v1.3.
    50	11. **Conformance tests**: 1 golden fixture per main tx kind (input → known SHA-256 of canonical bytes) + 100-input round-trip + cross-call byte stability.
    51	
    52	### Out of scope (explicit deferral)
    53	
    54	- **MetaTx + ancillaries** (`PredicatePatch`, `ToolPatch`, `JudgeSignature`, `HumanSignature`, `ConstitutionCheckProof`, `ReversibilityPlan`) — STATE spec § 1.6 declares MetaTx is **v4.1 only**; v4 emits `MetaProposalDraft` to L3 CAS, not L4. ⏭ deferred.
    55	- **Slash transition** — already deferred to CO P2.5 ChallengeCourt per CO1.7 spec K5.
    56	- **Per-kind transition function bodies** (`step_transition`, `verify_transition`, `challenge_transition`, `reuse_transition`, `finalize_reward_transition`, `task_expire_transition`, `emit_terminal_summary_transition`) — these consume the ABI defined here; they belong to **CO1.7.5** (the body atom).
    57	- **Sequencer + dispatch_transition + replay_full_transition** — these consume the ABI; they belong to CO1.7-impl **A2-A4** (post this atom).
    58	- **`SignalBundle` typed shape** — STATE spec uses `SignalBundle::empty()` / `::finalize(...)` / `::task_expired(...)` / `::terminal_summary(...)` constructors. v1 of this atom emits a minimal typed `SignalBundle` (single enum-like discriminator + payload) sufficient for CO1.7-impl to compile; full event-stream design lands in CO1.9 L6 signal indices.
    59	- **TransitionError full taxonomy** — v1 emits a minimal enum covering the variants invoked in spec § 3 pseudocode (`ClaimNotFound`, `ChallengeWindowStillOpen`, `AlreadySlashed`, `TaskNotFound`, `InvalidSystemSignature`, `StaleParent`, `TaskNotExpired`, `TaskHasOpenClaim`, `TerminalSummaryNotApplicable`, `NotYetImplemented`); per-stage enum proliferation is a CO1.7.5 concern.
    60	
    61	### What this atom is NOT replacing
    62	
    63	- `src/state/q_state.rs` (existing): keeps its existing types verbatim. CO1.1.4-pre1 only adds new types in `src/state/typed_tx.rs`.
    64	- `src/economy/money.rs` (existing): unchanged. `StakeMicroCoin` is a **newtype on `MicroCoin`** living in `src/economy/money.rs` (additive).
    65	
    66	### § 0.1 Cross-atom ordering gate (v1.1 NEW per Gemini Q4 round-1)
    67	
    68	**Constitutional concern**: CO1.7 LedgerEntry stores typed-tx payloads in L3 CAS via `tx_payload_cid: Cid`. The current shipped `CasStore::open()` initializes an empty in-memory index (CO1.4 store.rs:67); after process restart the CAS bytes are unrecoverable until the index is repopulated. This means **cold-replay of L4 cannot reconstruct typed payloads** — a direct Art. 0.2 (tape canonicality) violation if uncorrected.
    69	
    70	**Mitigation**: CAS index persistence is its own atom — **CO1.4-extra** — already named in CO1.7 spec § 0. CO1.4-extra adds index persistence (likely a sidecar JSONL or git-tag manifest) so cold-replay can recover payloads via `CasStore::get`.
    71	
    72	**Hard ordering for v1.1 PASS**:
    73	- CO1.7-impl A2 (Sequencer apply path) + A3 (dispatch_transition skeleton) may proceed against CO1.1.4-pre1 v1.1 PASS independently.
    74	- **CO1.7-impl A4 (replay_full_transition) MUST NOT ship before CO1.4-extra**. Until then, FullTransition replay errors with `CasMissing` after process restart (already documented in CO1.7 spec § 4 / `ReplayError::CasMissing`).
    75	- CO1.4-extra has its own dual-audit gate.
    76	
    77	This ordering is a **necessary condition for CO1.1.4-pre1 PASS** per round-1 Gemini Q4; documented here so future audits cannot reinterpret silence as approval.
    78	
    79	---
    80	
    81	## § 1 Module layout
    82	
    83	```
    84	src/state/
    85	├── mod.rs                       (existing; +pub mod typed_tx + re-exports)
    86	├── q_state.rs                   (existing; unchanged)
    87	└── typed_tx.rs                  (NEW; ~600-900 LoC; the ABI surface)
    88	
    89	src/economy/
    90	└── money.rs                     (existing; +pub struct StakeMicroCoin newtype + minimal impls)
    91	
    92	src/bottom_white/ledger/
    93	└── system_keypair.rs            (existing; serde_bytes_64 helper promoted to pub(crate)
    94	                                  so AgentSignature can re-use the [u8; 64] adapter)
    95	```
    96	
    97	**Crate boundary**: `state::typed_tx` consumes (a) `state::q_state` types (Hash, AgentId, TxId, NodeId), (b) `economy::money::MicroCoin` + `StakeMicroCoin`, (c) `bottom_white::cas::schema::Cid`, (d) `bottom_white::ledger::system_keypair::{SystemEpoch, SystemSignature}`. No new outward dependencies; no circular dep risk.
    98	
    99	---
   100	
   101	## § 2 Identifier newtypes
   102	
   103	```rust
   104	#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
   105	pub struct TaskId(pub String);
   106	
   107	#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
   108	pub struct RunId(pub String);
   109	
   110	#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
   111	pub struct ToolId(pub String);
   112	
   113	#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
   114	pub struct PredicateId(pub String);
   115	
   116	#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
   117	pub struct ReadKey(pub String);
   118	
   119	#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
   120	pub struct WriteKey(pub String);
   155	**v1 derivation** (from § 3.4 call sites + the TaskExpireTx pattern in § 3.6, system-emitted):
   156	
   157	```rust
   158	pub struct FinalizeRewardTx {
   159	    pub tx_id: TxId,                       //  1
   160	    pub claim_id: ClaimId,                 //  2  TYPED newtype (v1.1 P2)
   161	    pub task_id: TaskId,                   //  3  Q-DERIVED at replay; wire = ledger summary
   162	    pub solver: AgentId,                   //  4  Q-DERIVED at replay; wire = ledger summary
   163	    pub reward: MicroCoin,                 //  5  Q-DERIVED at replay (SettlementEngine output); wire = ledger summary
   164	    pub parent_state_root: Hash,           //  6  must equal q.state_root_t at submission
   165	    pub epoch: SystemEpoch,                //  7  which keypair signed
   166	    pub timestamp_logical: u64,            //  8  monotonic
   167	    pub system_signature: SystemSignature, //  9  system-emitted, see § 4.1 dual-sign rationale
   168	}
   169	```
   170	
   171	### § 4.1 Q-derived vs wire-only fields (v1.1 NEW per Codex Q-B + Gemini Q6)
   172	
   173	For `FinalizeRewardTx`, fields {`task_id`, `solver`, `reward`} are recorded on the wire as a **ledger summary** (so a human reading L4 can see the finalize event semantics + downstream tools without Q_t access can render the event). At replay, however, **the AUTHORITATIVE values come from `Q_t` lookups by `claim_id`**:
   174	- `task_id` = `q.economic_state_t.claims_t[claim_id].task_id` (or equivalent ClaimEntry field)
   175	- `solver` = `q.economic_state_t.claims_t[claim_id].solver` (or claimant)
   176	- `reward` = `SettlementEngine::finalize(claim, escrow, attribution, ...)` — recomputed from Q_t
   177	
   178	If wire-stored values diverge from Q-derived values at replay, **replay rejects with `TransitionError::ClaimNotFound` or a stricter mismatch error** (CO1.7-impl A4 enforces this; CO1.7.5 transition body owns the comparison rule).
   179	
   180	**Royalty edges**: NOT on wire. Replay walks `q.economic_state_t.royalty_graph_t.edges_from(claim.target_work_tx)` per STATE § 3.4 stage 3c. Eliminates wire-format bloat + prevents stale royalty snapshots from being trusted post-amendment.
   181	
   182	### § 4.2 Dual-sign rationale (v1.1 NEW per Gemini Q6)
   183	
   184	`FinalizeRewardTx.system_signature` is **NOT redundant** with the L4 envelope signature. They sign different bytes:
   185	- `FinalizeRewardTx.system_signature` signs the **payload bytes** (`FinalizeRewardSigningPayload.canonical_digest()` via `b"turingosv4.system_sig.finalize_reward.v1"` domain prefix). Audit-relevant for: "this finalize event was emitted by a runtime keypair epoch X" (cross-cell trust + post-hoc forensics).
   186	- L4 `LedgerEntry.system_signature` signs the **sequencer-stamped envelope** (`LedgerEntrySigningPayload.canonical_digest()` via `b"turingosv4.ledger_entry_signing.v1"` — CO1.7 spec § 1.2). Audit-relevant for: "this `(logical_t, parent_ledger_root, tx_payload_cid)` was committed by the sequencer".
   187	
   188	A successful replay verifies BOTH: payload sig (this struct) confirms typed bytes integrity; envelope sig confirms sequencer commitment ordering.
   189	
   190	---
   191	
   192	## § 5 Other typed tx schemas (transcribed from STATE spec)
   193	
   194	`WorkTx` (§ 1.2 — 12 fields), `VerifyTx` / `ChallengeTx` / `ReuseTx` (§ 1.3), `TaskExpireTx` (§ 3.6 v1.3 schema). Verbatim transcription; minor adjustments documented inline.
   195	
   196	`TxStatus` includes a `Pending` variant (per STATE § 1.2) but in this v4 codebase `TxStatus` is **set BY the runner**, never serialized into the canonical transaction wire format. Therefore: `TxStatus` is **NOT a field of any TypedTx variant**; it is a runtime book-keeping enum exposed on the public API surface but not part of the canonical encoding. (CO1.7 spec § 1.2 puts `status: TxStatus` on WorkTx field 12; this atom **diverges**: status is tracked in `q_t.q_t.agents[id].last_accepted_tx` + ClaimsIndex, NOT on the wire. **Audit input**: confirm or push back.)
   197	
   198	---
   199	
   200	## § 6 TypedTx enum
   232	## § 7 Canonical serialization invariants
   233	
   234	`canonical_encode` / `canonical_decode` (already shipped in `transition_ledger.rs` per CO1.7-impl A1) are reused as the wire codec:
   235	
   236	- **I-CANON-A**: `canonical_encode(typed_tx)` returns deterministic bytes (BE + fixed_int + BTreeMap/BTreeSet lex order).
   237	- **I-CANON-B**: `decode(encode(x)) == x` byte-identically for ALL variants (incl. zero-default).
   238	- **I-CANON-C**: 2 independent encode calls on the same value produce identical bytes.
   239	- **I-CANON-D**: per-variant golden fixture: every TypedTx variant (7 / 7) has a known SHA-256 of canonical bytes, hard-coded in tests (`EXPECTED_HEX_*`). Future serde-derive / codec change → fixture diff → audit-required (rotation commit).
   240	- **I-CANON-E** (v1.1 NEW): cross-variant non-collision — pairwise digests over all 7 fixture variants are distinct.
   241	- **I-CANON-F** (v1.1 NEW): BTreeMap / BTreeSet permutation independence — building the same struct via different insertion orders produces byte-identical bytes.
   242	- **I-CANON-G** (v1.1 NEW per C-1): each agent-signed and system-emitted typed-tx has a paired `*SigningPayload` struct + `canonical_digest()` with explicit domain prefix `b"turingosv4.<actor>.<purpose>.v1"`. Domain prefix bytes are part of the SHA-256 input. 6 distinct domains (work / verify / challenge agent + finalize_reward / task_expire / terminal_summary system) yield pairwise-distinct digests.
   243	
   244	### § 7.1 Codec wording fix (v1.1 P6 per Codex Q-D round-1)
   245	
   246	STATE_TRANSITION_SPEC § 2.5 v1.4 wording is **inaccurate** for the actual codec; this v1.1 spec corrects:
   247	
   248	| What § 2.5 said | What bincode-2 actually does |
   249	|---|---|
   250	| `Enum discriminant: u8 (variant index in declaration order)` | **u32 BE** ([bincode 2.0.1 src/features/serde/ser.rs:186](https://docs.rs/bincode/2.0.1/src/bincode/features/serde/ser.rs.html), [src/enc/impls.rs:68](https://docs.rs/bincode/2.0.1/src/bincode/enc/impls.rs.html)) under `with_fixed_int_encoding`. The variant index is encoded as `u32::to_be_bytes()`. |
   251	| `Strings serialized as UTF-8 with explicit length prefix u32-BE` | **u64 BE** length prefix (bincode encodes `usize` as u64 under `with_fixed_int_encoding`; [src/enc/impls.rs:128](https://docs.rs/bincode/2.0.1/src/bincode/enc/impls.rs.html)). The same applies to BTreeMap / BTreeSet / Vec lengths. |
   252	| `#[repr(u8)]` controls discriminant | **No** — `#[repr(u8)]` is a Rust language attribute affecting in-memory layout + raw cast (`as u8`) but does NOT control serde wire format. Codex caught this; spec language fixed. |
   253	
   254	**v1.1 decision**: keep u32 variants + u64 lengths; do NOT introduce a custom serde adapter to force u8 discriminants (which would force re-encoding of all existing fixtures + complicate forward-compat for >256 variants). The locked golden fixtures in `EXPECTED_HEX_*` reflect the actual u32/u64 codec.
   255	
   256	This wording fix is a **spec-only patch**; no code change required (the codec was already correct; only the description was wrong).
   257	
   258	---
   259	
   260	## § 8 HasSubmitter trait
   261	
   262	```rust
   263	pub trait HasSubmitter {
   264	    fn submitter_id(&self) -> Option<AgentId>;
   265	}
   266	
   267	impl HasSubmitter for WorkTx       { fn submitter_id(&self) -> Option<AgentId> { Some(self.agent_id.clone()) } }
   268	impl HasSubmitter for VerifyTx     { fn submitter_id(&self) -> Option<AgentId> { Some(self.verifier_agent.clone()) } }
   269	impl HasSubmitter for ChallengeTx  { fn submitter_id(&self) -> Option<AgentId> { Some(self.challenger_agent.clone()) } }
   270	impl HasSubmitter for ReuseTx      { fn submitter_id(&self) -> Option<AgentId> { None } }
   271	// FinalizeRewardTx, TaskExpireTx, TerminalSummaryTx: system-emitted; submitter_id() = None
   272	```
   273	
   274	Implements STATE spec § 3.6.5 v1.3 directive verbatim.
   275	
   276	---
   277	
   278	## § 9 Acknowledged divergences from STATE_TRANSITION_SPEC
   279	
   280	| ID | STATE spec | CO1.1.4-pre1 v1.1 | Reason |
   281	|---|---|---|---|
   282	| **D-1** | § 1.2 WorkTx field 12 = `status: TxStatus` | **dropped from wire** (Codex round-1 PASS with patch note) | TxStatus is runner book-keeping, not canonical wire data. STATE § 3 transition fns do NOT read `tx.status` from received tx; status is derived from accepted-tx history + ClaimsIndex. Codex Q-A round-1: PASS. |
   283	| **D-2** | § 3.4 `FinalizeTx::from(claim_id, reward)` opaque constructor | **explicit `FinalizeRewardTx` struct** with Q-derived field discipline (§ 4.1) + dual-sign rationale (§ 4.2) | spec gap; derived schema. |
   284	| **D-3** | ~~§ 1.5 `TerminalSummaryTx` 3-field placeholder~~ | **RESOLVED v1.1 P3**: migrated to full 8-field STATE § 1.5 schema in `state::typed_tx`; system_keypair signs opaque `TerminalSummarySigning([u8;32])` digest. |
   285	| **D-4** (v1.1 NEW per Codex Q-J / CX-3) | QState `task_markets_t` / `escrows_t` / `stakes_t` keyed by `TxId` (q_state.rs:201/161/182) but typed_tx schemas use `TaskId` for the same task references | **NOT retrofit in this atom**. Migration owned by **CO P2.1 (TaskMarket atom)** which will rekey the QState indices to `TaskId`. CO1.1.4-pre1 documents the cross-atom debt; no wire-format consequence (the typed-tx schemas already use `TaskId` correctly per STATE § 1.2). |
   286	
   287	---
   288	
   289	## § 10 Audit gates
   290	

exec
/bin/bash -lc "nl -ba handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md | sed -n '216,233p;310,326p;338,371p;434,455p;485,510p;582,594p;640,654p;718,738p;832,846p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   216	### 1.5 TerminalSummaryTx (no-accept run handler)
   217	
   218	```rust
   219	pub struct TerminalSummaryTx {
   220	    pub tx_id: TxId,
   221	    pub task_id: TaskId,
   222	    pub run_id: RunId,
   223	    pub run_outcome: RunOutcome,           // OmegaAccepted | MaxTxExhausted | WallClockCap | ComputeCap | ErrorHalt
   224	    pub total_attempts: u32,
   225	    pub failure_class_histogram: BTreeMap<RejectionClass, u32>,
   226	    pub last_logical_t: u64,
   227	    pub system_signature: SystemSignature,  // signed by runtime keypair, not by any agent
   228	}
   229	```
   230	
   231	If a run terminates without any accepted work_tx, the runtime emits exactly one `TerminalSummaryTx` to L4. This preserves L6 reconstructibility: error class signal is derivable from tape even if no work_tx ever passed.
   232	
   233	### 1.6 MetaTx (stub for v4.1; v4 only emits `MetaProposalDraft` to L3 CAS, not L4)
   310	fn bincode_canonical_config() -> bincode::config::Configuration {
   311	    bincode::config::standard()
   312	        .with_big_endian()
   313	        .with_fixed_int_encoding()    // no varint; fixed-width for determinism
   314	}
   315	```
   316	
   317	**Conformance**: `tests/canonical_serialization.rs` MUST verify:
   318	- 1 golden tx fixture per tx type (WorkTx / VerifyTx / ChallengeTx / ReuseTx / TerminalSummaryTx); each has known input → known SHA-256 output
   319	- Round-trip: `decode(encode(x)) == x` byte-identical for 100 random inputs
   320	- Stability: 2 independent runs on same input → same bytes
   321	
   322	**STEP_B implication**: branches A and B both use this exact `bincode_canonical_config`; signature verification works cross-branch by construction.
   323	
   324	**Out of scope for v1.x spec** (deferred per Codex Q5/NEW-5 round-3 PARTIAL acknowledgment): full golden fixture corpus + differential fuzzing seed + complete runner ABI for QState/SignalBundle/TransitionError. v1.4 freezes the SERIALIZATION RULE (bincode v2 big-endian + BTreeMap lex); fixtures + ABI land in **CO1.1.4-pre1** (canonical fixture corpus) + **CO1.7** (full ABI surface). This is an **explicit deferral** — not unresolved spec ambiguity. STEP_B branch A and branch B both implement the SAME bincode rule; per-tx digest matching is mechanical from v1.4. Full corpus generation is a downstream code task, not spec scope.
   325	
   326	---
   338	) -> Result<(QState, SignalBundle), TransitionError> {
   339	
   340	    // STAGE 1: parent_state_root match (stale view rejection)
   341	    if tx.parent_state_root != q.state_root_t {
   342	        return Err(TransitionError::StaleParent {
   343	            expected: q.state_root_t,
   344	            got:      tx.parent_state_root,
   345	        });
   346	        // NB: rejection here does NOT change Q_t; runner stamps RejectedAttemptSummary
   347	        // onto the NEXT accepted tx (or onto TerminalSummaryTx if run ends without accept)
   348	    }
   349	
   350	    // STAGE 2: signature verification
   351	    if !verify_signature(&tx.signature, tx.canonical_digest()) {
   352	        return Err(TransitionError::SignatureInvalid);
   353	    }
   354	
   355	    // STAGE 3: stake availability (Inv 5 — YES_E event-bound)
   356	    let agent_balance = q.economic_state_t.balances_t.get(&tx.agent_id);
   357	    if agent_balance < tx.stake {
   358	        return Err(TransitionError::StakeInsufficient { available: agent_balance, required: tx.stake });
   359	    }
   360	
   361	    // STAGE 4: predicate gate (Inv 6 — predicate-gated transition)
   362	    let acceptance_results = registry.run_acceptance(tx, q)?;
   363	    let safety_class = registry.classify(tx);
   364	    match (safety_class, acceptance_results.all_passed()) {
   365	        (SafetyOrCreation::Safety, false) => {
   366	            return Err(TransitionError::AcceptancePredicateFailed(acceptance_results));
   367	            // fail-closed for Safety (WP § 7.2)
   368	        }
   369	        (SafetyOrCreation::Creation, false) => {
   370	            // fail-open-with-signal: still reject, but emit informational signal (no Q_t change)
   371	            return Err(TransitionError::AcceptancePredicateFailed(acceptance_results));
   434	) -> Result<(QState, SignalBundle), TransitionError> {
   435	
   436	    // STAGE 1: target work_tx must exist + be in Pending or Provisional state
   437	    let target = q.economic_state_t.claims_t.get(&tx.target_work_tx)
   438	        .ok_or(TransitionError::TargetWorkTxNotFound)?;
   439	    if !target.status.allows_verification() {
   440	        return Err(TransitionError::TargetWorkTxNotVerifiable);
   441	    }
   442	
   443	    // STAGE 2: signature + bond
   444	    if !verify_signature(&tx.signature, tx.canonical_digest()) {
   445	        return Err(TransitionError::SignatureInvalid);
   446	    }
   447	    let verifier_balance = q.economic_state_t.balances_t.get(&tx.verifier_agent);
   448	    if verifier_balance < tx.bond {
   449	        return Err(TransitionError::StakeInsufficient);
   450	    }
   451	
   452	    // STAGE 3: predicate gate (verifier predicate, NOT same as work_tx acceptance)
   453	    let verify_results = registry.run_verification(tx, target, q)?;
   454	    if !verify_results.all_passed() {
   455	        return Err(TransitionError::VerificationPredicateFailed(verify_results));
   485	) -> Result<(QState, SignalBundle), TransitionError> {
   486	
   487	    // STAGE 1: target work_tx must exist + still in challenge window
   488	    let target = q.economic_state_t.claims_t.get(&tx.target_work_tx)
   489	        .ok_or(TransitionError::TargetWorkTxNotFound)?;
   490	    let window = q.economic_state_t.challenge_cases_t.get(tx.target_work_tx)
   491	        .ok_or(TransitionError::ChallengeWindowClosed)?;
   492	    // v1.4: use ChallengeWindow::is_open(now) per § 5.2.5; same rule used by finalize_reward
   493	    if !window.is_open(tx.timestamp_logical) {
   494	        return Err(TransitionError::ChallengeWindowClosed);
   495	    }
   496	
   497	    // STAGE 2: signature + NO_E stake
   498	    if !verify_signature(&tx.signature, tx.canonical_digest()) {
   499	        return Err(TransitionError::SignatureInvalid);
   500	    }
   501	    let challenger_balance = q.economic_state_t.balances_t.get(&tx.challenger_agent);
   502	    if challenger_balance < tx.stake {
   503	        return Err(TransitionError::StakeInsufficient);
   504	    }
   505	
   506	    // STAGE 3: counterexample acceptance predicate (the BURDEN OF PROOF predicate, Inv 7)
   507	    let counterexample = cas::get(&tx.counterexample_cid)?;
   508	    let counter_check = registry.run_counterexample_check(target, &counterexample, q)?;
   509	    if !counter_check.proves_violation() {
   510	        return Err(TransitionError::CounterexampleInsufficient(counter_check));
   582	) -> Result<(QState, SignalBundle), TransitionError> {
   583	    // STAGE 1: tool must be registered + still active in L2
   584	    let tool = tool_registry.get(tx.reused_tool_id)
   585	        .ok_or(TransitionError::ToolNotInRegistry)?;
   586	    if tool.creator != tx.reused_tool_creator {
   587	        return Err(TransitionError::ToolCreatorMismatch);
   588	    }
   589	
   590	    // STAGE 2: parent reusing_work_tx must exist + Accepted
   591	    let parent = q.economic_state_t.claims_t.get(&tx.reusing_work_tx)
   592	        .ok_or(TransitionError::TargetWorkTxNotFound)?;
   593	    if !parent.status.is_accepted_or_finalized() {
   594	        return Err(TransitionError::ParentNotAcceptedYet);
   640	) -> Result<(QState, SignalBundle), TransitionError> {
   641	    let claim = q.economic_state_t.claims_t.get(&claim_id)
   642	        .ok_or(TransitionError::ClaimNotFound)?;
   643	    let window = q.economic_state_t.challenge_cases_t.get(claim.target_work_tx);
   644	
   645	    // STAGE 1: window must be expired AND no open slash
   646	    // v1.4: invoke ChallengeWindow::is_open(now) per § 5.2.5 with explicit `now` arg;
   647	    // same rule as challenge_transition stage 1
   648	    if let Some(w) = window {
   649	        if w.is_open(q.q_t.current_round) {
   650	            return Err(TransitionError::ChallengeWindowStillOpen);
   651	        }
   652	        if w.outcome == Some(ChallengeOutcome::Slashed(_)) {
   653	            return Err(TransitionError::AlreadySlashed);  // never finalize a slashed claim
   654	        }
   718	) -> Result<(QState, SignalBundle), TransitionError> {
   719	    let task = q.economic_state_t.task_markets_t.get(tx.task_id)
   720	        .ok_or(TransitionError::TaskNotFound)?;
   721	
   722	    // STAGE 1: signature verification (system signature; not agent)
   723	    if !verify_system_signature(&tx.system_signature, &tx, q.system_pubkey_at_epoch(tx.epoch)) {
   724	        return Err(TransitionError::InvalidSystemSignature);
   725	    }
   726	
   727	    // STAGE 2: parent_state_root match (stale view rejection)
   728	    if tx.parent_state_root != q.state_root_t {
   729	        return Err(TransitionError::StaleParent);
   730	    }
   731	
   732	    // STAGE 3: expiry check — task must be expired AND have NO Pending OR Provisional OR Finalized claim
   733	    // v1.3 fix (Codex new-issue #2): broaden race-protection from "Finalized only" to all claim statuses
   734	    if task.deadline_logical_t > q.q_t.current_round {
   735	        return Err(TransitionError::TaskNotExpired);
   736	    }
   737	    if q.economic_state_t.claims_t.any_claim_for_task(tx.task_id) {
   738	        return Err(TransitionError::TaskHasOpenClaim);    // refund only if NO claim exists at all
   832	) -> Result<(QState, SignalBundle), TransitionError> {
   833	    let run = runtime.run_state(run_id)?;
   834	    if run.has_accepted_work_tx() {
   835	        return Err(TransitionError::TerminalSummaryNotApplicable);  // only emitted for no-accept runs
   836	    }
   837	
   838	    let summary = TerminalSummaryTx {
   839	        tx_id: TxId::derive(run_id, "terminal"),
   840	        task_id: run.task_id,
   841	        run_id,
   842	        run_outcome: run.outcome(),
   843	        total_attempts: run.attempt_counter(),
   844	        failure_class_histogram: run.failure_histogram(),
   845	        last_logical_t: run.last_logical_t(),
   846	        system_signature: runtime.system_keypair().sign(canonical_digest_terminal(run)),

codex
The main v1.1 code shape is present. I’ve already found a spec hygiene miss: §9 still contains a D-3 row marked “RESOLVED,” despite the patch log saying it was removed, and §0 scope still says TerminalSummaryTx exists in `system_keypair.rs`. I’m checking whether there are behavioral blockers too.
exec
/bin/bash -lc "rg -n \"struct TerminalSummaryTx|TerminalSummaryTx\\s*\\{|TerminalSummaryTx\\(|TerminalSummarySigning|terminal_summary_emitter|sign_terminal_summary|use crate::state|crate::state::|super::.*TerminalSummary\" src/bottom_white src/state src/economy" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
src/state/mod.rs:27:    TaskExpireTx, TaskId, TerminalSummarySigningPayload, TerminalSummaryTx, ToolId,
src/state/typed_tx.rs:27:use crate::state::q_state::{AgentId, Hash, TxId};
src/state/typed_tx.rs:333:/// an opaque `TerminalSummarySigning([u8; 32])` digest — same opaque-digest
src/state/typed_tx.rs:338:pub struct TerminalSummaryTx {
src/state/typed_tx.rs:485:pub struct TerminalSummarySigningPayload {
src/state/typed_tx.rs:495:impl TerminalSummarySigningPayload {
src/state/typed_tx.rs:574:impl TerminalSummaryTx {
src/state/typed_tx.rs:575:    pub fn to_signing_payload(&self) -> TerminalSummarySigningPayload {
src/state/typed_tx.rs:576:        TerminalSummarySigningPayload {
src/state/typed_tx.rs:669:impl HasSubmitter for TerminalSummaryTx {
src/state/typed_tx.rs:1005:    fn fixture_terminal_summary_tx() -> TerminalSummaryTx {
src/state/typed_tx.rs:1013:        TerminalSummaryTx {
src/bottom_white/ledger/transition_ledger.rs:42:use crate::state::q_state::Hash;
src/bottom_white/ledger/system_keypair.rs:180:// `CanonicalMessage::TerminalSummarySigning([u8; 32])` variant — same
src/bottom_white/ledger/system_keypair.rs:234:    TerminalSummarySigning([u8; 32]),
src/bottom_white/ledger/system_keypair.rs:463:        CanonicalMessage::TerminalSummarySigning(digest) => {
src/bottom_white/ledger/system_keypair.rs:464:            h.update(b"TerminalSummarySigning");
src/bottom_white/ledger/system_keypair.rs:558:pub(crate) mod terminal_summary_emitter {
src/bottom_white/ledger/system_keypair.rs:566:    pub(crate) fn sign_terminal_summary(
src/bottom_white/ledger/system_keypair.rs:570:        sign_system_message_inner(keypair, &CanonicalMessage::TerminalSummarySigning(digest))
src/bottom_white/ledger/system_keypair.rs:918:            terminal_summary_emitter::sign_epoch_rotation_proof(&old, &proof).expect("old sign");
src/bottom_white/ledger/system_keypair.rs:920:            terminal_summary_emitter::sign_epoch_rotation_proof(&new, &proof).expect("new sign");

exec
/bin/bash -lc 'rg -n "claim_id|ClaimId|FinalizeRewardTx|SignalKind::Finalize|SignalBundle::finalize|FinalizeRewardSigningPayload" src handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:12:| **P1** | AgentSignature reused 64-byte adapter without domain separation; comments implied "exclude signature" digest with no signing payload | NEW signing-payload structs (`WorkSigningPayload` / `VerifySigningPayload` / `ChallengeSigningPayload` / `FinalizeRewardSigningPayload` / `TaskExpireSigningPayload` / `TerminalSummarySigningPayload`) — each has explicit domain prefix (`b"turingosv4.<actor>.<purpose>.v1"`) prepended to bincode body bytes in `canonical_digest()`. Plus `to_signing_payload()` projection on each tx. | C-1 (Codex Q-E + Gemini Q7) |
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:13:| **P2** | `FinalizeRewardTx.claim_id: TxId` reused TxId, leaking ClaimsIndex impl into wire format | New `ClaimId(pub TxId)` newtype with `#[serde(transparent)]` (wire-identical to TxId; non-breaking); `FinalizeRewardTx.claim_id: ClaimId` now | C-3 (Codex Q-B) |
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:19:| **P8** | FinalizeRewardTx had ambiguous {task_id, solver, reward, royalty} provenance + redundant system_signature unclear | This spec § 4 explicitly states {task_id, solver, reward} are **Q-DERIVED at replay** (re-fetched from ClaimsIndex by claim_id; wire fields are ledger summary, NOT trusted from wire); `system_signature` is RETAINED with explicit dual-sign rationale (this sig binds the tx-payload bytes; the L4 `LedgerEntrySigningPayload` sig binds the sequencer-stamped envelope; both are needed). | C-3 + GM-2 |
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:30:**Companion**: `STATE_TRANSITION_SPEC_v1_2026-04-27.md` § 1 (typed schemas), § 2.5 (canonical serialization), § 3 (transition pseudocode — informs FinalizeRewardTx schema, see § 4 below).
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:47:8. **Typed-tx payload structs**: `WorkTx`, `VerifyTx`, `ChallengeTx`, `ReuseTx`, `FinalizeRewardTx`, `TaskExpireTx`. (`TerminalSummaryTx` already exists in `system_keypair.rs`.)
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:151:## § 4 FinalizeRewardTx — derived schema
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:153:**Spec gap**: STATE_TRANSITION_SPEC § 3.4 uses `FinalizeTx::from(claim_id, reward)` constructor pattern but provides no explicit struct. CO1.7 spec § 1 lists `TxKind::FinalizeReward = 4` but defers the struct to "frozen in STATE_TRANSITION_SPEC § 1" — which the STATE spec doesn't actually contain.
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:158:pub struct FinalizeRewardTx {
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:160:    pub claim_id: ClaimId,                 //  2  TYPED newtype (v1.1 P2)
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:173:For `FinalizeRewardTx`, fields {`task_id`, `solver`, `reward`} are recorded on the wire as a **ledger summary** (so a human reading L4 can see the finalize event semantics + downstream tools without Q_t access can render the event). At replay, however, **the AUTHORITATIVE values come from `Q_t` lookups by `claim_id`**:
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:174:- `task_id` = `q.economic_state_t.claims_t[claim_id].task_id` (or equivalent ClaimEntry field)
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:175:- `solver` = `q.economic_state_t.claims_t[claim_id].solver` (or claimant)
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:184:`FinalizeRewardTx.system_signature` is **NOT redundant** with the L4 envelope signature. They sign different bytes:
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:185:- `FinalizeRewardTx.system_signature` signs the **payload bytes** (`FinalizeRewardSigningPayload.canonical_digest()` via `b"turingosv4.system_sig.finalize_reward.v1"` domain prefix). Audit-relevant for: "this finalize event was emitted by a runtime keypair epoch X" (cross-cell trust + post-hoc forensics).
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:208:    FinalizeReward(FinalizeRewardTx),
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:271:// FinalizeRewardTx, TaskExpireTx, TerminalSummaryTx: system-emitted; submitter_id() = None
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:283:| **D-2** | § 3.4 `FinalizeTx::from(claim_id, reward)` opaque constructor | **explicit `FinalizeRewardTx` struct** with Q-derived field discipline (§ 4.1) + dual-sign rationale (§ 4.2) | spec gap; derived schema. |
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:305:- **Spec rounds**: 1-2 expected. The bulk is mechanical transcription; § 4 (FinalizeRewardTx derivation) + § 5 D-1 (TxStatus elision) are the only design decisions auditors are likely to test.
src/state/mod.rs:23:    AgentSignature, BoolWithProof, ChallengeSigningPayload, ChallengeTx, ClaimId,
src/state/mod.rs:24:    FinalizeRewardSigningPayload, FinalizeRewardTx, HasSubmitter, PredicateId,
src/state/typed_tx.rs:7://!   FinalizeRewardTx schema in spec § 4)
src/state/typed_tx.rs:42:/// in `FinalizeRewardTx.claim_id` and `ClaimsIndex` keying. Wraps `TxId`
src/state/typed_tx.rs:52:pub struct ClaimId(pub TxId);
src/state/typed_tx.rs:54:impl ClaimId {
src/state/typed_tx.rs:279:/// uses opaque `FinalizeTx::from(claim_id, reward)` constructor without an
src/state/typed_tx.rs:283:/// - **C-3 (Codex Q-B)**: `claim_id` is now a typed `ClaimId` newtype (was
src/state/typed_tx.rs:284:///   bare `TxId`) — STATE § 4 I-FINALIZE-BATCH-ORDER speaks in claim_id;
src/state/typed_tx.rs:288:///   ClaimsIndex by `claim_id`, NOT trusted from wire. Wire fields are kept
src/state/typed_tx.rs:292:///   binds the system-emitted FinalizeRewardTx to a specific runtime keypair
src/state/typed_tx.rs:299:pub struct FinalizeRewardTx {
src/state/typed_tx.rs:301:    pub claim_id: ClaimId,                 //  2 — typed (was TxId in v1)
src/state/typed_tx.rs:447:/// System signing payload for `FinalizeRewardTx` (9 fields → 8 fields).
src/state/typed_tx.rs:449:pub struct FinalizeRewardSigningPayload {
src/state/typed_tx.rs:451:    pub claim_id: ClaimId,
src/state/typed_tx.rs:460:impl FinalizeRewardSigningPayload {
src/state/typed_tx.rs:546:impl FinalizeRewardTx {
src/state/typed_tx.rs:547:    pub fn to_signing_payload(&self) -> FinalizeRewardSigningPayload {
src/state/typed_tx.rs:548:        FinalizeRewardSigningPayload {
src/state/typed_tx.rs:550:            claim_id: self.claim_id.clone(),
src/state/typed_tx.rs:601:    FinalizeReward(FinalizeRewardTx),
src/state/typed_tx.rs:657:impl HasSubmitter for FinalizeRewardTx {
src/state/typed_tx.rs:756:    /// finalize_reward — no claim entry for the given claim_id.
src/state/typed_tx.rs:827:        claim_id: TxId,
src/state/typed_tx.rs:852:    pub fn finalize(claim_id: TxId, reward: MicroCoin) -> Self {
src/state/typed_tx.rs:854:            kind: SignalKind::Finalize { claim_id, reward },
src/state/typed_tx.rs:979:    fn fixture_finalize_reward_tx() -> FinalizeRewardTx {
src/state/typed_tx.rs:980:        FinalizeRewardTx {
src/state/typed_tx.rs:982:            claim_id: ClaimId::new("claim-001"),
src/state/typed_tx.rs:1195:            TypedTx::FinalizeReward(FinalizeRewardTx::default()),

exec
/bin/bash -lc 'cargo test --lib state::typed_tx -- --nocapture' in /home/zephryj/projects/turingosv4
 succeeded in 372ms:
warning: unused import: `KernelError`
 --> src/bus.rs:8:29
  |
8 | use crate::kernel::{Kernel, KernelError};
  |                             ^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `TapeError`
 --> src/bus.rs:9:54
  |
9 | use crate::ledger::{EventType, Ledger, Node, NodeId, TapeError};
  |                                                      ^^^^^^^^^

warning: unused imports: `Deserialize` and `Serialize`
  --> src/bus.rs:11:13
   |
11 | use serde::{Deserialize, Serialize};
   |             ^^^^^^^^^^^  ^^^^^^^^^

warning: unused import: `crate::sdk::snapshot::UniverseSnapshot`
 --> src/sdk/actor.rs:7:5
  |
7 | use crate::sdk::snapshot::UniverseSnapshot;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `ToolSignal`
 --> src/sdk/tools/search.rs:4:24
  |
4 | use crate::sdk::tool::{ToolSignal, TuringTool};
  |                        ^^^^^^^^^^

warning: unused import: `std::path::Path`
 --> src/sdk/tools/search.rs:6:5
  |
6 | use std::path::Path;
  |     ^^^^^^^^^^^^^^^

warning: unused import: `Deserialize`
 --> src/sdk/tools/librarian.rs:6:13
  |
6 | use serde::{Deserialize, Serialize};
  |             ^^^^^^^^^^^

warning: unused import: `Path`
 --> src/sdk/tools/librarian.rs:9:17
  |
9 | use std::path::{Path, PathBuf};
  |                 ^^^^

warning: unused import: `ObjectType as Git2ObjectType`
 --> src/bottom_white/cas/store.rs:9:12
  |
9 | use git2::{ObjectType as Git2ObjectType, Repository};
  |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `ObjectType as Git2ObjectType`
  --> src/bottom_white/ledger/transition_ledger.rs:36:12
   |
36 | use git2::{ObjectType as Git2ObjectType, Repository, Signature as GitSignature};
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: variable does not need to be mutable
   --> src/top_white/predicates/registry.rs:255:17
    |
255 |             let mut h = Sha256::new();
    |                 ----^
    |                 |
    |                 help: remove this `mut`
    |
    = note: `#[warn(unused_mut)]` (part of `#[warn(unused)]`) on by default

warning: function `sign_system_message` is never used
   --> src/bottom_white/ledger/system_keypair.rs:543:19
    |
543 |     pub(crate) fn sign_system_message(
    |                   ^^^^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: function `sign_terminal_summary` is never used
   --> src/bottom_white/ledger/system_keypair.rs:566:19
    |
566 |     pub(crate) fn sign_terminal_summary(
    |                   ^^^^^^^^^^^^^^^^^^^^^

warning: function `sign_system_message` is never used
   --> src/bottom_white/ledger/system_keypair.rs:585:19
    |
585 |     pub(crate) fn sign_system_message(
    |                   ^^^^^^^^^^^^^^^^^^^

warning: `turingosv4` (lib test) generated 14 warnings (run `cargo fix --lib -p turingosv4 --tests` to apply 11 suggestions)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.28s
     Running unittests src/lib.rs (target/debug/deps/turingosv4-337b37021c2192ab)

running 17 tests
test state::typed_tx::tests::golden_task_expire_tx_digest ... ok
test state::typed_tx::tests::golden_reuse_tx_digest ... ok
test state::typed_tx::tests::golden_finalize_reward_tx_digest ... ok
test state::typed_tx::tests::golden_challenge_tx_digest ... ok
test state::typed_tx::tests::has_submitter_partitioning ... ok
test state::typed_tx::tests::golden_terminal_summary_tx_digest ... ok
test state::typed_tx::tests::golden_work_tx_digest ... ok
test state::typed_tx::tests::signing_payload_excludes_signature ... ok
test state::typed_tx::tests::golden_verify_tx_digest ... ok
test state::typed_tx::tests::typed_tx_btree_permutation_independence ... ok
test state::typed_tx::tests::signing_payload_domains_are_distinct ... ok
test state::typed_tx::tests::typed_tx_cross_variant_non_collision ... ok
test state::typed_tx::tests::typed_tx_kind_projection ... ok
test state::typed_tx::tests::typed_tx_byte_stability_across_calls ... ok
test state::typed_tx::tests::typed_tx_default_round_trip ... ok
test state::typed_tx::tests::typed_tx_round_trip_all_variants ... ok
test state::typed_tx::tests::typed_tx_round_trip_100_inputs ... ok

test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 204 filtered out; finished in 0.01s


codex
The 17 typed_tx tests pass. A more substantive miss showed up in the v1.1 surface: `ClaimId` was applied to `FinalizeRewardTx`, but `SignalKind::Finalize` and `SignalBundle::finalize` still take `TxId`, so P2 did not fully close the API-surface type distinction.
exec
/bin/bash -lc 'rg -n "TaskMarketsIndex|EscrowsIndex|StakesIndex|ClaimsIndex|BTreeMap<TxId|HashMap|struct QState|pub struct EconomicState" src/state/q_state.rs' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
10://! **BTreeMap, not HashMap, everywhere** (Inv determinism;
11://! Codex flagged `kernel.rs:187-204` HashMap nondeterminism in round-2).
143:pub struct EconomicState {
145:    pub escrows_t: EscrowsIndex,
146:    pub stakes_t: StakesIndex,
147:    pub claims_t: ClaimsIndex,
149:    pub task_markets_t: TaskMarketsIndex,
161:pub struct EscrowsIndex(pub BTreeMap<TxId, EscrowEntry>);
182:pub struct StakesIndex(pub BTreeMap<TxId, StakeEntry>);
201:pub struct ClaimsIndex(pub BTreeMap<TxId, ClaimEntry>);
224:pub struct TaskMarketsIndex(pub BTreeMap<TxId, TaskMarketEntry>);
262:pub struct RoyaltyGraph(pub BTreeMap<TxId, Vec<RoyaltyEdge>>);
275:pub struct ChallengeCasesIndex(pub BTreeMap<TxId, ChallengeCase>);
296:pub struct PriceIndex(pub BTreeMap<TxId, MicroCoin>);
307:pub struct QState {

exec
/bin/bash -lc 'rg -n "bincode =|sha2 =|hex =" Cargo.toml Cargo.lock | head -60' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
Cargo.toml:9:bincode = { version = "2", features = ["serde"] }
Cargo.toml:21:sha2 = "0.10"

exec
/bin/bash -lc 'cargo check' in /home/zephryj/projects/turingosv4
 succeeded in 2324ms:
    Checking turingosv4 v0.1.0 (/home/zephryj/projects/turingosv4)
warning: unused import: `KernelError`
 --> src/bus.rs:8:29
  |
8 | use crate::kernel::{Kernel, KernelError};
  |                             ^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `TapeError`
 --> src/bus.rs:9:54
  |
9 | use crate::ledger::{EventType, Ledger, Node, NodeId, TapeError};
  |                                                      ^^^^^^^^^

warning: unused imports: `Deserialize` and `Serialize`
  --> src/bus.rs:11:13
   |
11 | use serde::{Deserialize, Serialize};
   |             ^^^^^^^^^^^  ^^^^^^^^^

warning: unused import: `crate::sdk::snapshot::UniverseSnapshot`
 --> src/sdk/actor.rs:7:5
  |
7 | use crate::sdk::snapshot::UniverseSnapshot;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `std::collections::HashSet`
 --> src/sdk/actor.rs:9:5
  |
9 | use std::collections::HashSet;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `ToolSignal`
 --> src/sdk/tools/search.rs:4:24
  |
4 | use crate::sdk::tool::{ToolSignal, TuringTool};
  |                        ^^^^^^^^^^

warning: unused import: `std::path::Path`
 --> src/sdk/tools/search.rs:6:5
  |
6 | use std::path::Path;
  |     ^^^^^^^^^^^^^^^

warning: unused import: `Deserialize`
 --> src/sdk/tools/librarian.rs:6:13
  |
6 | use serde::{Deserialize, Serialize};
  |             ^^^^^^^^^^^

warning: unused import: `Path`
 --> src/sdk/tools/librarian.rs:9:17
  |
9 | use std::path::{Path, PathBuf};
  |                 ^^^^

warning: unused import: `ObjectType as Git2ObjectType`
 --> src/bottom_white/cas/store.rs:9:12
  |
9 | use git2::{ObjectType as Git2ObjectType, Repository};
  |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `ObjectType as Git2ObjectType`
  --> src/bottom_white/ledger/transition_ledger.rs:36:12
   |
36 | use git2::{ObjectType as Git2ObjectType, Repository, Signature as GitSignature};
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: method `sign_digest` is never used
   --> src/bottom_white/ledger/system_keypair.rs:336:8
    |
281 | impl Ed25519Keypair {
    | ------------------- method in this implementation
...
336 |     fn sign_digest(&self, digest: [u8; 32]) -> Result<SystemSignature, KeypairError> {
    |        ^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: function `sign_system_message_inner` is never used
   --> src/bottom_white/ledger/system_keypair.rs:618:4
    |
618 | fn sign_system_message_inner(
    |    ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `sign_rejected_attempt_summary` is never used
   --> src/bottom_white/ledger/system_keypair.rs:532:19
    |
532 |     pub(crate) fn sign_rejected_attempt_summary(
    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `sign_system_message` is never used
   --> src/bottom_white/ledger/system_keypair.rs:543:19
    |
543 |     pub(crate) fn sign_system_message(
    |                   ^^^^^^^^^^^^^^^^^^^

warning: function `sign_terminal_summary` is never used
   --> src/bottom_white/ledger/system_keypair.rs:566:19
    |
566 |     pub(crate) fn sign_terminal_summary(
    |                   ^^^^^^^^^^^^^^^^^^^^^

warning: function `sign_epoch_rotation_proof` is never used
   --> src/bottom_white/ledger/system_keypair.rs:574:19
    |
574 |     pub(crate) fn sign_epoch_rotation_proof(
    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `sign_system_message` is never used
   --> src/bottom_white/ledger/system_keypair.rs:585:19
    |
585 |     pub(crate) fn sign_system_message(
    |                   ^^^^^^^^^^^^^^^^^^^

warning: function `sign_ledger_entry` is never used
   --> src/bottom_white/ledger/system_keypair.rs:607:19
    |
607 |     pub(crate) fn sign_ledger_entry(
    |                   ^^^^^^^^^^^^^^^^^

warning: `turingosv4` (lib) generated 19 warnings (run `cargo fix --lib -p turingosv4` to apply 11 suggestions)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.25s

exec
/bin/bash -lc "rg -n '"'^name = "bincode"|''^version = "2'"\\.' Cargo.lock -A2" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
111:name = "bincode"
112:version = "2.0.1"
113-source = "registry+https://github.com/rust-lang/crates.io-index"
114-checksum = "36eaf5d7b090263e8150820482d5d93cd964a81e4019913c972f4edcc6edb740"
--
123:version = "2.0.1"
124-source = "registry+https://github.com/rust-lang/crates.io-index"
125-checksum = "bf95709a440f45e986983918d0e8a1f30a9b1df04918fc828670606804ac3c09"
--
132:version = "2.11.0"
133-source = "registry+https://github.com/rust-lang/crates.io-index"
134-checksum = "843867be96c8daad0d758b57df9392b6d8d271134fce549de6ce169ff98a92af"
--
318:version = "2.2.3"
319-source = "registry+https://github.com/rust-lang/crates.io-index"
320-checksum = "115531babc129696a58c64a4fef0a8bf9e9698629fb97e9e40767d235cfbcd53"
--
328:version = "2.2.0"
329-source = "registry+https://github.com/rust-lang/crates.io-index"
330-checksum = "70e796c081cee67dc755e1a36a0a172b897fab85fc3f6bc48307991f64e4eca9"
--
375:version = "2.4.1"
376-source = "registry+https://github.com/rust-lang/crates.io-index"
377-checksum = "9f1f227452a390804cdb637b74a86990f2a7d7ba4b7d5693aac9b4dd6defd8d6"
--
595:version = "2.2.0"
596-source = "registry+https://github.com/rust-lang/crates.io-index"
597-checksum = "2984d1cd16c883d7935b9e07e44071dca8d917fd52ecc02c04d5fa0b5a3f191c"
--
609:version = "2.2.0"
610-source = "registry+https://github.com/rust-lang/crates.io-index"
611-checksum = "92219b62b3e2b4d88ac5119f8904c10f8f61bf7e95b640d25ba3075e6cac2c29"
--
622:version = "2.2.0"
623-source = "registry+https://github.com/rust-lang/crates.io-index"
624-checksum = "c56e5ee99d6e3d33bd91c5d85458b6005a22140021cc324cea84dd0e72cff3b4"
--
636:version = "2.2.0"
637-source = "registry+https://github.com/rust-lang/crates.io-index"
638-checksum = "da3be0ae77ea334f4da67c12f149704f19f81d1adf7c51cf482943e84a2bad38"
--
642:version = "2.2.0"
643-source = "registry+https://github.com/rust-lang/crates.io-index"
644-checksum = "bee3b67d0ea5c2cca5003417989af8996f8604e34fb9ddf96208a033901e70de"
--
656:version = "2.2.0"
657-source = "registry+https://github.com/rust-lang/crates.io-index"
658-checksum = "8e2bbb201e0c04f7b4b3e14382af113e17ba4f63e2c9d2ee626b720cbce54a14"
--
662:version = "2.2.0"
663-source = "registry+https://github.com/rust-lang/crates.io-index"
664-checksum = "139c4cf31c8b5f33d7e199446eff9c1e02decfc2f0eec2c8d71f65befa45b421"
--
707:version = "2.12.0"
708-source = "registry+https://github.com/rust-lang/crates.io-index"
709-checksum = "d98f6fed1fde3f8c21bc40a1abb88dd75e67924f9cffc3ef95607bad8017f8e2"
--
844:version = "2.8.0"
845-source = "registry+https://github.com/rust-lang/crates.io-index"
846-checksum = "f8ca58f447f06ed17d5fc4043ce1b10dd205e060fb3ce5b979b8ed8e59ff3f79"
--
926:version = "2.3.2"
927-source = "registry+https://github.com/rust-lang/crates.io-index"
928-checksum = "9b4f627cb1b25917193a259e49bdad08f671f8d9708acfd5fe0a8c1455d87220"
--
1226:version = "2.1.1"
1227-source = "registry+https://github.com/rust-lang/crates.io-index"
1228-checksum = "357703d41365b4b27c590e3ed91eabb1b663f07c4c084095e60cbed4362dff0d"
--
1404:version = "2.2.0"
1405-source = "registry+https://github.com/rust-lang/crates.io-index"
1406-checksum = "77549399552de45a898a580c1b41d445bf730df867cc44e6c0233bbc4b8329de"
--
1451:version = "2.6.1"
1452-source = "registry+https://github.com/rust-lang/crates.io-index"
1453-checksum = "13c2bddecc57b384dee18652358fb23172facb8a2c51ccc10d74c157bdea3292"
--
1457:version = "2.0.117"
1458-source = "registry+https://github.com/rust-lang/crates.io-index"
1459-checksum = "e665b8803e7b1d2a727f4023456bbbbe74da67099c585258af0ad9c5013b9b99"
--
1501:version = "2.0.18"
1502-source = "registry+https://github.com/rust-lang/crates.io-index"
1503-checksum = "4288b5bcbc7920c07a1149a35cf9590a2aa808e0bc1eafaade0b80947865fbc4"
--
1510:version = "2.0.18"
1511-source = "registry+https://github.com/rust-lang/crates.io-index"
1512-checksum = "ebc4ee7f67670e9b64d05fa4253e753e016c6c95ff35b89b7941d6b856dec1d5"
--
1563:version = "2.7.0"
1564-source = "registry+https://github.com/rust-lang/crates.io-index"
1565-checksum = "385a6cb71ab9ab790c5fe8d67f1645e6c450a7ce006a33de03daa956cf70a496"
--
1734:version = "2.5.8"
1735-source = "registry+https://github.com/rust-lang/crates.io-index"
1736-checksum = "ff67a8a4397373c3ef660812acab3268222035010ab8680ec4215f38ba3d0eed"

exec
/bin/bash -lc "sed -n '108,118p' Cargo.lock" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
checksum = "2af50177e190e07a26ab74f8b1efbfe2ef87da2116221318cb1c2e82baf7de06"

[[package]]
name = "bincode"
version = "2.0.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "36eaf5d7b090263e8150820482d5d93cd964a81e4019913c972f4edcc6edb740"
dependencies = [
 "bincode_derive",
 "serde",
 "unty",

exec
/bin/bash -lc "rg -n \"TxId\\)|TxId,|TxId>|ClaimId|Finalize\\s*\\{\" src/state/typed_tx.rs src/state/mod.rs src/**/*.rs" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
src/state/typed_tx.rs:48:/// `#[serde(transparent)]` — wire-identical to TxId, so adoption is
src/state/typed_tx.rs:52:pub struct ClaimId(pub TxId);
src/state/typed_tx.rs:54:impl ClaimId {
src/state/typed_tx.rs:222:    pub tx_id: TxId,                                  //  1
src/state/typed_tx.rs:239:    pub tx_id: TxId,                       //  1
src/state/typed_tx.rs:240:    pub target_work_tx: TxId,              //  2
src/state/typed_tx.rs:258:    pub tx_id: TxId,                       //  1
src/state/typed_tx.rs:259:    pub target_work_tx: TxId,              //  2
src/state/typed_tx.rs:271:    pub tx_id: TxId,                       //  1
src/state/typed_tx.rs:272:    pub reusing_work_tx: TxId,             //  2
src/state/typed_tx.rs:283:/// - **C-3 (Codex Q-B)**: `claim_id` is now a typed `ClaimId` newtype (was
src/state/typed_tx.rs:300:    pub tx_id: TxId,                       //  1
src/state/typed_tx.rs:301:    pub claim_id: ClaimId,                 //  2 — typed (was TxId in v1)
src/state/typed_tx.rs:316:    pub tx_id: TxId,                       //  1
src/state/typed_tx.rs:339:    pub tx_id: TxId,                                          //  1
src/state/typed_tx.rs:395:    pub tx_id: TxId,
src/state/typed_tx.rs:416:    pub tx_id: TxId,
src/state/typed_tx.rs:417:    pub target_work_tx: TxId,
src/state/typed_tx.rs:433:    pub tx_id: TxId,
src/state/typed_tx.rs:434:    pub target_work_tx: TxId,
src/state/typed_tx.rs:450:    pub tx_id: TxId,
src/state/typed_tx.rs:451:    pub claim_id: ClaimId,
src/state/typed_tx.rs:469:    pub tx_id: TxId,
src/state/typed_tx.rs:486:    pub tx_id: TxId,
src/state/typed_tx.rs:826:    Finalize {
src/state/typed_tx.rs:827:        claim_id: TxId,
src/state/typed_tx.rs:852:    pub fn finalize(claim_id: TxId, reward: MicroCoin) -> Self {
src/state/typed_tx.rs:854:            kind: SignalKind::Finalize { claim_id, reward },
src/state/typed_tx.rs:982:            claim_id: ClaimId::new("claim-001"),
src/state/q_state.rs:93:    pub last_accepted_tx: Option<TxId>,
src/state/q_state.rs:161:pub struct EscrowsIndex(pub BTreeMap<TxId, EscrowEntry>);
src/state/q_state.rs:182:pub struct StakesIndex(pub BTreeMap<TxId, StakeEntry>);
src/state/q_state.rs:201:pub struct ClaimsIndex(pub BTreeMap<TxId, ClaimEntry>);
src/state/q_state.rs:224:pub struct TaskMarketsIndex(pub BTreeMap<TxId, TaskMarketEntry>);
src/state/q_state.rs:262:pub struct RoyaltyGraph(pub BTreeMap<TxId, Vec<RoyaltyEdge>>);
src/state/q_state.rs:268:    pub ancestor: TxId,
src/state/q_state.rs:275:pub struct ChallengeCasesIndex(pub BTreeMap<TxId, ChallengeCase>);
src/state/q_state.rs:296:pub struct PriceIndex(pub BTreeMap<TxId, MicroCoin>);
src/state/mod.rs:19:    RoyaltyEdge, RoyaltyGraph, StakeEntry, StakesIndex, TaskMarketEntry, TaskMarketsIndex, TxId,
src/state/mod.rs:23:    AgentSignature, BoolWithProof, ChallengeSigningPayload, ChallengeTx, ClaimId,
src/state/mod.rs:19:    RoyaltyEdge, RoyaltyGraph, StakeEntry, StakesIndex, TaskMarketEntry, TaskMarketsIndex, TxId,
src/state/mod.rs:23:    AgentSignature, BoolWithProof, ChallengeSigningPayload, ChallengeTx, ClaimId,
src/state/typed_tx.rs:48:/// `#[serde(transparent)]` — wire-identical to TxId, so adoption is
src/state/typed_tx.rs:52:pub struct ClaimId(pub TxId);
src/state/typed_tx.rs:54:impl ClaimId {
src/state/typed_tx.rs:222:    pub tx_id: TxId,                                  //  1
src/state/typed_tx.rs:239:    pub tx_id: TxId,                       //  1
src/state/typed_tx.rs:240:    pub target_work_tx: TxId,              //  2
src/state/typed_tx.rs:258:    pub tx_id: TxId,                       //  1
src/state/typed_tx.rs:259:    pub target_work_tx: TxId,              //  2
src/state/typed_tx.rs:271:    pub tx_id: TxId,                       //  1
src/state/typed_tx.rs:272:    pub reusing_work_tx: TxId,             //  2
src/state/typed_tx.rs:283:/// - **C-3 (Codex Q-B)**: `claim_id` is now a typed `ClaimId` newtype (was
src/state/typed_tx.rs:300:    pub tx_id: TxId,                       //  1
src/state/typed_tx.rs:301:    pub claim_id: ClaimId,                 //  2 — typed (was TxId in v1)
src/state/typed_tx.rs:316:    pub tx_id: TxId,                       //  1
src/state/typed_tx.rs:339:    pub tx_id: TxId,                                          //  1
src/state/typed_tx.rs:395:    pub tx_id: TxId,
src/state/typed_tx.rs:416:    pub tx_id: TxId,
src/state/typed_tx.rs:417:    pub target_work_tx: TxId,
src/state/typed_tx.rs:433:    pub tx_id: TxId,
src/state/typed_tx.rs:434:    pub target_work_tx: TxId,
src/state/typed_tx.rs:450:    pub tx_id: TxId,
src/state/typed_tx.rs:451:    pub claim_id: ClaimId,
src/state/typed_tx.rs:469:    pub tx_id: TxId,
src/state/typed_tx.rs:486:    pub tx_id: TxId,
src/state/typed_tx.rs:826:    Finalize {
src/state/typed_tx.rs:827:        claim_id: TxId,
src/state/typed_tx.rs:852:    pub fn finalize(claim_id: TxId, reward: MicroCoin) -> Self {
src/state/typed_tx.rs:854:            kind: SignalKind::Finalize { claim_id, reward },
src/state/typed_tx.rs:982:            claim_id: ClaimId::new("claim-001"),

exec
/bin/bash -lc 'rg -n "TerminalSummaryTx" handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md src/state/typed_tx.rs src/bottom_white/ledger/system_keypair.rs' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
src/bottom_white/ledger/system_keypair.rs:178:// the typed `TerminalSummaryTx` struct (8-field per STATE § 1.5) now lives in
src/bottom_white/ledger/system_keypair.rs:230:    /// of the 8-field `state::typed_tx::TerminalSummaryTx` is computed in
src/bottom_white/ledger/system_keypair.rs:554:/// digest produced by `state::typed_tx::TerminalSummaryTx::canonical_digest()`
src/state/typed_tx.rs:183:/// TRACE_MATRIX § 1.5 TerminalSummaryTx field 4 + Art. IV halt-reason taxonomy.
src/state/typed_tx.rs:338:pub struct TerminalSummaryTx {
src/state/typed_tx.rs:483:/// System signing payload for `TerminalSummaryTx` (8 fields → 7 fields).
src/state/typed_tx.rs:574:impl TerminalSummaryTx {
src/state/typed_tx.rs:594:/// `TerminalSummaryTx` is imported from `system_keypair.rs` (already shipped).
src/state/typed_tx.rs:603:    TerminalSummary(TerminalSummaryTx),
src/state/typed_tx.rs:669:impl HasSubmitter for TerminalSummaryTx {
src/state/typed_tx.rs:1005:    fn fixture_terminal_summary_tx() -> TerminalSummaryTx {
src/state/typed_tx.rs:1013:        TerminalSummaryTx {
src/state/typed_tx.rs:1197:            TypedTx::TerminalSummary(TerminalSummaryTx::default()),
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:14:| **P3** | `TerminalSummaryTx` was 3-field placeholder living in `system_keypair.rs` (versus STATE § 1.5 8-field schema); locking the wrong shape into ABI | Migrated to `state::typed_tx::TerminalSummaryTx` with full 8-field STATE schema (tx_id / task_id / run_id / run_outcome / total_attempts / failure_class_histogram / last_logical_t / system_signature). `system_keypair` now signs an opaque `TerminalSummarySigning([u8; 32])` digest (same opaque-digest pattern as `LedgerEntrySigning`) — no `bottom_white ↔ state` circular dep. | C-3 (Codex Q-C must-fix-now) |
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:18:| **P7** | D-3 TerminalSummaryTx field-set divergence | RESOLVED (P3 migrated to full schema). § 9 D-3 row removed. | C-3 followup |
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:47:8. **Typed-tx payload structs**: `WorkTx`, `VerifyTx`, `ChallengeTx`, `ReuseTx`, `FinalizeRewardTx`, `TaskExpireTx`. (`TerminalSummaryTx` already exists in `system_keypair.rs`.)
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:210:    TerminalSummary(TerminalSummaryTx),  // imported from system_keypair
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:271:// FinalizeRewardTx, TaskExpireTx, TerminalSummaryTx: system-emitted; submitter_id() = None
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:284:| **D-3** | ~~§ 1.5 `TerminalSummaryTx` 3-field placeholder~~ | **RESOLVED v1.1 P3**: migrated to full 8-field STATE § 1.5 schema in `state::typed_tx`; system_keypair signs opaque `TerminalSummarySigning([u8;32])` digest. |

exec
/bin/bash -lc "rg -n \"fn serialize_unit_variant|variant_index|encode_slice_len|usize\" ~/.cargo/registry/src -g 'ser.rs' -g 'impls.rs' | rg 'bincode-2.0.1|features/serde/ser.rs|enc/impls.rs' | head -80" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
/home/zephryj/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-2.0.1/src/enc/impls.rs:128:impl Encode for usize {
/home/zephryj/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-2.0.1/src/enc/impls.rs:132:                crate::varint::varint_encode_usize(encoder.writer(), E::C::ENDIAN, *self)
/home/zephryj/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-2.0.1/src/enc/impls.rs:301:        super::encode_slice_len(encoder, self.len())?;
/home/zephryj/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-2.0.1/src/enc/impls.rs:357:impl<T, const N: usize> Encode for [T; N]
/home/zephryj/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-2.0.1/src/features/serde/ser.rs:32:pub fn encode_into_slice<E, C>(val: E, dst: &mut [u8], config: C) -> Result<usize, EncodeError>
/home/zephryj/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-2.0.1/src/features/serde/ser.rs:70:) -> Result<usize, EncodeError> {
/home/zephryj/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-2.0.1/src/features/serde/ser.rs:186:    fn serialize_unit_variant(
/home/zephryj/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-2.0.1/src/features/serde/ser.rs:189:        variant_index: u32,
/home/zephryj/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-2.0.1/src/features/serde/ser.rs:192:        variant_index.encode(self.enc)
/home/zephryj/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-2.0.1/src/features/serde/ser.rs:209:        variant_index: u32,
/home/zephryj/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-2.0.1/src/features/serde/ser.rs:216:        variant_index.encode(&mut self.enc)?;
/home/zephryj/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-2.0.1/src/features/serde/ser.rs:220:    fn serialize_seq(mut self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
/home/zephryj/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-2.0.1/src/features/serde/ser.rs:226:    fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> {
/home/zephryj/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-2.0.1/src/features/serde/ser.rs:233:        _len: usize,
/home/zephryj/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-2.0.1/src/features/serde/ser.rs:241:        variant_index: u32,
/home/zephryj/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-2.0.1/src/features/serde/ser.rs:243:        _len: usize,
/home/zephryj/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-2.0.1/src/features/serde/ser.rs:245:        variant_index.encode(&mut self.enc)?;
/home/zephryj/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-2.0.1/src/features/serde/ser.rs:249:    fn serialize_map(mut self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
/home/zephryj/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-2.0.1/src/features/serde/ser.rs:258:        _len: usize,
/home/zephryj/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-2.0.1/src/features/serde/ser.rs:266:        variant_index: u32,
/home/zephryj/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-2.0.1/src/features/serde/ser.rs:268:        _len: usize,
/home/zephryj/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-2.0.1/src/features/serde/ser.rs:270:        variant_index.encode(&mut self.enc)?;
/home/zephryj/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-2.0.1/src/de/impls.rs:174:impl<Context> Decode<Context> for usize {
/home/zephryj/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-2.0.1/src/de/impls.rs:179:                crate::varint::varint_decode_usize(decoder.reader(), D::C::ENDIAN)
/home/zephryj/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-2.0.1/src/de/impls.rs:197:impl_borrow_decode!(usize);
/home/zephryj/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-2.0.1/src/de/impls.rs:201:        NonZeroUsize::new(usize::decode(decoder)?).ok_or(DecodeError::NonZeroTypeIsZero {
/home/zephryj/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-2.0.1/src/de/impls.rs:474:impl<Context, T, const N: usize> Decode<Context> for [T; N]
/home/zephryj/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-2.0.1/src/de/impls.rs:504:impl<'de, T, const N: usize, Context> BorrowDecode<'de, Context> for [T; N]
/home/zephryj/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-2.0.1/src/de/impls.rs:796:const fn utf8_char_width(b: u8) -> usize {
/home/zephryj/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-2.0.1/src/de/impls.rs:797:    UTF8_CHAR_WIDTH[b as usize] as usize

exec
/bin/bash -lc "nl -ba /home/zephryj/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-2.0.1/src/features/serde/ser.rs | sed -n '180,220p;236,272p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   180	    }
   181	
   182	    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
   183	        Ok(())
   184	    }
   185	
   186	    fn serialize_unit_variant(
   187	        self,
   188	        _name: &'static str,
   189	        variant_index: u32,
   190	        _variant: &'static str,
   191	    ) -> Result<Self::Ok, Self::Error> {
   192	        variant_index.encode(self.enc)
   193	    }
   194	
   195	    fn serialize_newtype_struct<T>(
   196	        self,
   197	        _name: &'static str,
   198	        value: &T,
   199	    ) -> Result<Self::Ok, Self::Error>
   200	    where
   201	        T: Serialize + ?Sized,
   202	    {
   203	        value.serialize(self)
   204	    }
   205	
   206	    fn serialize_newtype_variant<T>(
   207	        mut self,
   208	        _name: &'static str,
   209	        variant_index: u32,
   210	        _variant: &'static str,
   211	        value: &T,
   212	    ) -> Result<Self::Ok, Self::Error>
   213	    where
   214	        T: Serialize + ?Sized,
   215	    {
   216	        variant_index.encode(&mut self.enc)?;
   217	        value.serialize(self)
   218	    }
   219	
   220	    fn serialize_seq(mut self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
   236	    }
   237	
   238	    fn serialize_tuple_variant(
   239	        mut self,
   240	        _name: &'static str,
   241	        variant_index: u32,
   242	        _variant: &'static str,
   243	        _len: usize,
   244	    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
   245	        variant_index.encode(&mut self.enc)?;
   246	        Ok(Compound { enc: self.enc })
   247	    }
   248	
   249	    fn serialize_map(mut self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
   250	        let len = len.ok_or_else(|| SerdeEncodeError::SequenceMustHaveLength.into())?;
   251	        len.encode(&mut self.enc)?;
   252	        Ok(Compound { enc: self.enc })
   253	    }
   254	
   255	    fn serialize_struct(
   256	        self,
   257	        _name: &'static str,
   258	        _len: usize,
   259	    ) -> Result<Self::SerializeStruct, Self::Error> {
   260	        Ok(Compound { enc: self.enc })
   261	    }
   262	
   263	    fn serialize_struct_variant(
   264	        mut self,
   265	        _name: &'static str,
   266	        variant_index: u32,
   267	        _variant: &'static str,
   268	        _len: usize,
   269	    ) -> Result<Self::SerializeStructVariant, Self::Error> {
   270	        variant_index.encode(&mut self.enc)?;
   271	        Ok(Compound { enc: self.enc })
   272	    }

exec
/bin/bash -lc "nl -ba /home/zephryj/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bincode-2.0.1/src/enc/impls.rs | sed -n '60,76p;124,136p;292,306p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
    60	}
    61	
    62	impl Encode for NonZeroU16 {
    63	    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
    64	        self.get().encode(encoder)
    65	    }
    66	}
    67	
    68	impl Encode for u32 {
    69	    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
    70	        match E::C::INT_ENCODING {
    71	            IntEncoding::Variable => {
    72	                crate::varint::varint_encode_u32(encoder.writer(), E::C::ENDIAN, *self)
    73	            }
    74	            IntEncoding::Fixed => match E::C::ENDIAN {
    75	                Endianness::Big => encoder.writer().write(&self.to_be_bytes()),
    76	                Endianness::Little => encoder.writer().write(&self.to_le_bytes()),
   124	        self.get().encode(encoder)
   125	    }
   126	}
   127	
   128	impl Encode for usize {
   129	    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
   130	        match E::C::INT_ENCODING {
   131	            IntEncoding::Variable => {
   132	                crate::varint::varint_encode_usize(encoder.writer(), E::C::ENDIAN, *self)
   133	            }
   134	            IntEncoding::Fixed => match E::C::ENDIAN {
   135	                Endianness::Big => encoder.writer().write(&(*self as u64).to_be_bytes()),
   136	                Endianness::Little => encoder.writer().write(&(*self as u64).to_le_bytes()),
   292	        encode_utf8(encoder.writer(), *self)
   293	    }
   294	}
   295	
   296	impl<T> Encode for [T]
   297	where
   298	    T: Encode,
   299	{
   300	    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
   301	        super::encode_slice_len(encoder, self.len())?;
   302	
   303	        if unty::type_equal::<T, u8>() {
   304	            // Safety: T = u8
   305	            let t: &[u8] = unsafe { core::mem::transmute(self) };
   306	            encoder.writer().write(t)?;

exec
/bin/bash -lc "nl -ba src/state/q_state.rs | sed -n '136,230p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   136	// ────────────────────────────────────────────────────────────────────────────
   137	
   138	/// TRACE_MATRIX WP § 2 economic — 9-sub-field economic state. Each sub-index
   139	/// is a BTreeMap newtype; entry shapes (Escrow / Stake / Claim / TaskMarket /
   140	/// RoyaltyEdge / ChallengeCase) are minimal-but-typed here and fully fleshed
   141	/// in the owning atoms (CO P2.1-2.6).
   142	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   143	pub struct EconomicState {
   144	    pub balances_t: BalancesIndex,
   145	    pub escrows_t: EscrowsIndex,
   146	    pub stakes_t: StakesIndex,
   147	    pub claims_t: ClaimsIndex,
   148	    pub reputations_t: ReputationsIndex,
   149	    pub task_markets_t: TaskMarketsIndex,
   150	    pub royalty_graph_t: RoyaltyGraph,
   151	    pub challenge_cases_t: ChallengeCasesIndex,
   152	    pub price_index_t: PriceIndex,
   153	}
   154	
   155	/// TRACE_MATRIX WP § 2 — agent → balance ledger. Concrete entry: `MicroCoin` (CO1.0a).
   156	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   157	pub struct BalancesIndex(pub BTreeMap<AgentId, MicroCoin>);
   158	
   159	/// TRACE_MATRIX WP § 2 — tx → escrow entry. Full schema lands CO P2.2 EscrowVault.
   160	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   161	pub struct EscrowsIndex(pub BTreeMap<TxId, EscrowEntry>);
   162	
   163	/// TRACE_MATRIX WP § 2 — escrow entry shape (stub). Full fields land CO P2.2.
   164	/// `#[serde(default)]` on each field gives forward-compat: future atoms can add
   165	/// fields without breaking deserialization of historical ledger rows.
   166	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
   167	pub struct EscrowEntry {
   168	    #[serde(default = "MicroCoin::zero")]
   169	    pub amount: MicroCoin,
   170	    #[serde(default)]
   171	    pub depositor: AgentId,
   172	}
   173	
   174	impl Default for EscrowEntry {
   175	    fn default() -> Self {
   176	        Self { amount: MicroCoin::zero(), depositor: AgentId::default() }
   177	    }
   178	}
   179	
   180	/// TRACE_MATRIX WP § 2 — tx → stake entry. Full schema lands CO P2.5 ChallengeCourt.
   181	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   182	pub struct StakesIndex(pub BTreeMap<TxId, StakeEntry>);
   183	
   184	/// TRACE_MATRIX WP § 2 — stake entry shape (stub). Full fields land CO P2.5.
   185	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
   186	pub struct StakeEntry {
   187	    #[serde(default = "MicroCoin::zero")]
   188	    pub amount: MicroCoin,
   189	    #[serde(default)]
   190	    pub staker: AgentId,
   191	}
   192	
   193	impl Default for StakeEntry {
   194	    fn default() -> Self {
   195	        Self { amount: MicroCoin::zero(), staker: AgentId::default() }
   196	    }
   197	}
   198	
   199	/// TRACE_MATRIX WP § 2 — tx → reward claim. Full schema lands CO P2.6 SettlementEngine.
   200	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   201	pub struct ClaimsIndex(pub BTreeMap<TxId, ClaimEntry>);
   202	
   203	/// TRACE_MATRIX WP § 2 — claim entry shape (stub). Full fields land CO P2.6.
   204	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
   205	pub struct ClaimEntry {
   206	    #[serde(default = "MicroCoin::zero")]
   207	    pub amount: MicroCoin,
   208	    #[serde(default)]
   209	    pub claimant: AgentId,
   210	}
   211	
   212	impl Default for ClaimEntry {
   213	    fn default() -> Self {
   214	        Self { amount: MicroCoin::zero(), claimant: AgentId::default() }
   215	    }
   216	}
   217	
   218	/// TRACE_MATRIX WP § 2 — agent → reputation ledger.
   219	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   220	pub struct ReputationsIndex(pub BTreeMap<AgentId, Reputation>);
   221	
   222	/// TRACE_MATRIX WP § 2 — tx → task market. Full schema lands CO P2.1.
   223	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   224	pub struct TaskMarketsIndex(pub BTreeMap<TxId, TaskMarketEntry>);
   225	
   226	/// TRACE_MATRIX WP § 2 — task market entry shape (stub). Full fields land CO P2.1.
   227	/// Default values (verifier_quorum=1, max_reuse_royalty_fraction=0.10) match the
   228	/// PROJECT_DECISION_MAP § 2.3 spec gap defaults.
   229	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
   230	pub struct TaskMarketEntry {

exec
/bin/bash -lc 'cargo test --lib' in /home/zephryj/projects/turingosv4
 succeeded in 1432ms:
warning: unused import: `KernelError`
 --> src/bus.rs:8:29
  |
8 | use crate::kernel::{Kernel, KernelError};
  |                             ^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `TapeError`
 --> src/bus.rs:9:54
  |
9 | use crate::ledger::{EventType, Ledger, Node, NodeId, TapeError};
  |                                                      ^^^^^^^^^

warning: unused imports: `Deserialize` and `Serialize`
  --> src/bus.rs:11:13
   |
11 | use serde::{Deserialize, Serialize};
   |             ^^^^^^^^^^^  ^^^^^^^^^

warning: unused import: `crate::sdk::snapshot::UniverseSnapshot`
 --> src/sdk/actor.rs:7:5
  |
7 | use crate::sdk::snapshot::UniverseSnapshot;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `ToolSignal`
 --> src/sdk/tools/search.rs:4:24
  |
4 | use crate::sdk::tool::{ToolSignal, TuringTool};
  |                        ^^^^^^^^^^

warning: unused import: `std::path::Path`
 --> src/sdk/tools/search.rs:6:5
  |
6 | use std::path::Path;
  |     ^^^^^^^^^^^^^^^

warning: unused import: `Deserialize`
 --> src/sdk/tools/librarian.rs:6:13
  |
6 | use serde::{Deserialize, Serialize};
  |             ^^^^^^^^^^^

warning: unused import: `Path`
 --> src/sdk/tools/librarian.rs:9:17
  |
9 | use std::path::{Path, PathBuf};
  |                 ^^^^

warning: unused import: `ObjectType as Git2ObjectType`
 --> src/bottom_white/cas/store.rs:9:12
  |
9 | use git2::{ObjectType as Git2ObjectType, Repository};
  |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `ObjectType as Git2ObjectType`
  --> src/bottom_white/ledger/transition_ledger.rs:36:12
   |
36 | use git2::{ObjectType as Git2ObjectType, Repository, Signature as GitSignature};
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: variable does not need to be mutable
   --> src/top_white/predicates/registry.rs:255:17
    |
255 |             let mut h = Sha256::new();
    |                 ----^
    |                 |
    |                 help: remove this `mut`
    |
    = note: `#[warn(unused_mut)]` (part of `#[warn(unused)]`) on by default

warning: function `sign_system_message` is never used
   --> src/bottom_white/ledger/system_keypair.rs:543:19
    |
543 |     pub(crate) fn sign_system_message(
    |                   ^^^^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: function `sign_terminal_summary` is never used
   --> src/bottom_white/ledger/system_keypair.rs:566:19
    |
566 |     pub(crate) fn sign_terminal_summary(
    |                   ^^^^^^^^^^^^^^^^^^^^^

warning: function `sign_system_message` is never used
   --> src/bottom_white/ledger/system_keypair.rs:585:19
    |
585 |     pub(crate) fn sign_system_message(
    |                   ^^^^^^^^^^^^^^^^^^^

warning: `turingosv4` (lib test) generated 14 warnings (run `cargo fix --lib -p turingosv4 --tests` to apply 11 suggestions)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.18s
     Running unittests src/lib.rs (target/debug/deps/turingosv4-337b37021c2192ab)

running 221 tests
test boot::tests::parse_errors_when_section_missing ... ok
test boot::tests::parse_errors_on_unquoted_key ... ok
test boot::tests::parse_strips_inline_comment_and_blanks ... ok
test boot::tests::verify_trust_root_detects_tamper_in_tempdir ... ok
test boot::tests::verify_trust_root_detects_child_manifest_tamper ... ok
test boot::tests::verify_trust_root_passes_when_hash_matches_in_tempdir ... ok
test bottom_white::cas::schema::tests::cid_differs_on_content ... ok
test bottom_white::cas::schema::tests::cid_deterministic ... ok
test bottom_white::cas::schema::tests::cid_from_empty_content ... ok
test boot::tests::verify_trust_root_passes_with_matching_child_manifest ... ok
test bottom_white::cas::schema::tests::cid_display_format ... ok
test bottom_white::cas::schema::tests::metadata_canonical_hash_deterministic ... ok
test bottom_white::cas::schema::tests::metadata_canonical_hash_differs_on_object_type ... ok
test bottom_white::cas::store::tests::cid_is_content_address ... ok
test bottom_white::cas::store::tests::empty_store_root ... ok
test bottom_white::cas::store::tests::cell_isolation_disjoint_cas ... ok
test bottom_white::cas::store::tests::get_nonexistent_returns_error ... ok
test bottom_white::cas::store::tests::metadata_recorded ... ok
test bottom_white::cas::store::tests::put_get_round_trip_small ... ok
test bottom_white::cas::store::tests::merkle_root_deterministic_two_runs ... ok
test bottom_white::cas::store::tests::put_idempotent_same_content ... ok
test bottom_white::cas::store::tests::put_get_round_trip_large ... ok
test bottom_white::ledger::system_keypair::tests::authorized_scope_signing_round_trip ... ok
test bottom_white::ledger::transition_ledger::tests::append_is_pure_and_byte_stable ... ok
test bottom_white::ledger::transition_ledger::tests::canonical_codec_round_trip ... ok
test bottom_white::ledger::transition_ledger::tests::canonical_digest_excludes_derivatives ... ok
test bottom_white::ledger::transition_ledger::tests::canonical_digest_stable_across_clones ... ok
test bottom_white::cas::store::tests::put_many_then_iterate_count ... ok
test bottom_white::ledger::transition_ledger::tests::git2_writer_append_and_read_back ... ok
test bottom_white::ledger::system_keypair::tests::terminal_scope_rotation_signing_round_trip ... ok
test bottom_white::ledger::transition_ledger::tests::git2_writer_empty_chain ... ok
test bottom_white::ledger::transition_ledger::tests::git2_writer_rejects_logical_t_gap ... ok
test bottom_white::ledger::transition_ledger::tests::replay_chain_integrity_clean ... ok
test bottom_white::ledger::transition_ledger::tests::replay_rejects_ledger_root_tamper ... ok
test bottom_white::ledger::transition_ledger::tests::replay_rejects_parent_ledger_tamper ... ok
test bottom_white::ledger::transition_ledger::tests::in_memory_writer_enforces_logical_t ... ok
test bottom_white::ledger::transition_ledger::tests::replay_rejects_parent_state_tamper ... ok
test bottom_white::tools::registry::tests::duplicate_id_rejected ... ok
test bottom_white::ledger::transition_ledger::tests::git2_writer_reopen_recovers_chain ... ok
test bottom_white::tools::registry::tests::empty_id_rejected ... ok
test bottom_white::tools::registry::tests::empty_registry ... ok
test bottom_white::tools::registry::tests::merkle_root_deterministic ... ok
test bottom_white::tools::registry::tests::find_by_capability_replaces_magic_string ... ok
test bottom_white::tools::registry::tests::register_and_get_round_trip ... ok
test bottom_white::tools::registry::tests::non_idempotent_rejected ... ok
test bus::tests::test_bus_basic_append ... ok
test bus::tests::test_bus_classify_bounded ... ok
test bus::tests::test_bus_creates_market_on_append ... ok
test bus::tests::test_bus_graveyard_feedback ... ok
test bus::tests::test_bus_halt_and_settle ... ok
test bus::tests::test_bus_ledger_integrity ... ok
test bus::tests::test_bus_payload_too_long ... ok
test bus::tests::test_bus_serial_ordering ... ok
test bus::tests::test_bus_snapshot ... ok
test bus::tests::test_bus_too_many_lines ... ok
test bus::tests::test_bus_unknown_agent_vetoed ... ok
test drivers::llm_http::tests::test_client_creation ... ok
test drivers::llm_http::tests::test_driver_error_display ... ok
test drivers::llm_http::tests::test_generate_request_serialization ... ok
test economy::money::tests::checked_add_normal ... ok
test economy::money::tests::checked_add_overflow_returns_none ... ok
test economy::money::tests::checked_sub_normal ... ok
test economy::money::tests::conservation_law_basic ... ok
test bus::tests::test_bus_forbidden_pattern_veto ... ok
test economy::money::tests::display_positive ... ok
test economy::money::tests::display_zero ... ok
test economy::money::tests::from_coin_round_trip ... ok
test economy::money::tests::from_micro_units_zero ... ok
test economy::money::tests::from_coin_overflow_returns_none ... ok
test economy::money::tests::royalty_10_percent_rounds_down ... ok
test economy::money::tests::royalty_floor_dust ... ok
test economy::money::tests::royalty_rejects_negative ... ok
test economy::money::tests::ordering_for_btreemap ... ok
test economy::money::tests::serde_round_trip_json ... ok
test economy::money::tests::royalty_rejects_weight_above_1 ... ok
test economy::money::tests::serde_transparent_format ... ok
test kernel::tests::test_golden_path_trace ... ok
test kernel::tests::test_market_lifecycle ... ok
test kernel::tests::test_append_and_retrieve ... ok
test kernel::tests::test_no_duplicate_market ... ok
test kernel::tests::test_market_ticker ... ok
test kernel::tests::test_reject_dangling_citation ... ok
test kernel::tests::test_reject_duplicate ... ok
test kernel::tests::test_no_market_for_nonexistent_node ... ok
test ledger::tests::test_ledger_append_and_verify ... ok
test kernel::tests::test_resolve_all_markets ... ok
test ledger::tests::test_ledger_omega_vocabulary ... ok
test ledger::tests::test_ledger_hash_chain_integrity ... ok
test ledger::tests::test_ledger_sequence_monotonic ... ok
test ledger::tests::test_tape_append_root_node ... ok
test ledger::tests::test_ledger_tamper_detection ... ok
test ledger::tests::test_tape_dag_branching ... ok
test ledger::tests::test_tape_empty ... ok
test ledger::tests::test_tape_append_with_valid_citation ... ok
test ledger::tests::test_tape_reject_duplicate_id ... ok
test ledger::tests::test_tape_reject_dangling_citation ... ok
test ledger::tests::test_tape_trace_ancestors ... ok
test prediction_market::tests::test_assassin_profit ... ok
test ledger::tests::test_tape_time_arrow_ordering ... ok
test prediction_market::tests::test_buy_yes_increases_yes_price ... ok
test prediction_market::tests::test_buy_no_increases_no_price ... ok
test prediction_market::tests::test_constant_product_invariant ... ok
test prediction_market::tests::test_create_market ... ok
test prediction_market::tests::test_ctf_conservation_1_coin_1_yes_1_no ... ok
test prediction_market::tests::test_initial_price_is_50_50 ... ok
test prediction_market::tests::test_multiple_traders_price_discovery ... ok
test prediction_market::tests::test_no_trading_after_resolution ... ok
test prediction_market::tests::test_pioneer_profit ... ok
test prediction_market::tests::test_no_double_resolution ... ok
test prediction_market::tests::test_prices_sum_to_one ... ok
test prediction_market::tests::test_reject_zero_or_negative_amounts ... ok
test prediction_market::tests::test_redeem_requires_resolution ... ok
test sdk::actor::tests::test_boltzmann_diversity_not_deterministic ... ok
test sdk::actor::tests::test_boltzmann_never_returns_none_with_nodes ... ok
test sdk::actor::tests::test_boltzmann_returns_none_empty_tape ... ok
test sdk::actor::tests::test_frontier_detection_leaf ... ok
test sdk::actor::tests::test_frontier_detection_parent_with_child ... ok
test sdk::error_abstraction::tests::classifier_version_is_stamped ... ok
test sdk::error_abstraction::tests::fixture_linarith_failed ... ok
test sdk::error_abstraction::tests::fixture_other_catchall ... ok
test sdk::error_abstraction::tests::fixture_rewrite_no_match ... ok
test sdk::error_abstraction::tests::fixture_simp_no_progress ... ok
test sdk::error_abstraction::tests::fixture_type_mismatch ... ok
test sdk::error_abstraction::tests::fixture_unexpected_token ... ok
test sdk::error_abstraction::tests::fixture_unknown_constant ... ok
test sdk::error_abstraction::tests::fixture_unsolved_goals ... ok
test sdk::error_abstraction::tests::labels_are_unique_and_stable ... ok
test sdk::prompt::tests::test_prompt_contains_no_example_values ... ok
test sdk::prompt::tests::test_prompt_includes_balance ... ok
test sdk::prompt::tests::test_prompt_surfaces_search_hits ... ok
test sdk::actor::tests::test_lineage_score_increases_with_depth ... ok
test sdk::prompt::tests::test_prompt_surfaces_team_board ... ok
test sdk::prompt::tests::test_prompt_truncates_errors_to_3 ... ok
test sdk::prompt_guard::tests::test_case_insensitive_match - should panic ... ok
test sdk::prompt_guard::tests::test_clean_prompt_passes ... ok
test sdk::prompt_guard::tests::test_h_vpput_caught - should panic ... ok
test sdk::prompt_guard::tests::test_empty_prompt_passes ... ok
test sdk::prompt_guard::tests::test_pput_assignment_pattern_caught - should panic ... ok
test sdk::prompt_guard::tests::test_pput_runtime_caught - should panic ... ok
test sdk::prompt_guard::tests::test_pput_m_verified_caught - should panic ... ok
test sdk::prompt_guard::tests::test_pput_verified_caught - should panic ... ok
test sdk::prompt_guard::tests::test_pput_substring_in_larger_text - should panic ... ok
test sdk::prompt_guard::tests::test_wbcg_caught - should panic ... ok
test sdk::protocol::tests::test_deduct_negative_amount_rejected ... ok
test sdk::protocol::tests::test_malformed_action_tag_rejected_not_fallback ... ok
test sdk::protocol::tests::test_no_byte_repair_on_invalid_escape ... ok
test sdk::protocol::tests::test_parse_action_tag_valid ... ok
test sdk::protocol::tests::test_parse_action_tag_with_think_block ... ok
test sdk::protocol::tests::test_parse_invalid_json_returns_error ... ok
test sdk::protocol::tests::test_parse_bare_json_fallback ... ok
test sdk::protocol::tests::test_parse_with_invest_action ... ok
test sdk::protocol::tests::test_parse_no_action_returns_error ... ok
test sdk::protocol::tests::test_strip_multiple_think_blocks ... ok
test sdk::protocol::tests::test_strip_think_blocks ... ok
test sdk::protocol::tests::test_strip_unclosed_think_block ... ok
test bottom_white::ledger::transition_ledger::tests::signature_round_trip_and_transplant_defense ... ok
test sdk::sandbox::tests::test_sandbox_captures_stderr ... ok
test sdk::sandbox::tests::test_sandbox_echo_command ... ok
test sdk::snapshot::tests::test_snapshot_balance_query ... ok
test sdk::tools::librarian::tests::test_board_post_append ... ok
test sdk::tools::librarian::tests::test_board_write_read_roundtrip ... ok
test sdk::tools::librarian::tests::test_build_compression_prompt ... ok
test sdk::tools::librarian::tests::test_compress_interval ... ok
test sdk::tools::librarian::tests::test_zero_interval_never_compresses ... ok
test sdk::tools::search::tests::test_sanitize_query ... ok
test sdk::tools::search::tests::test_search_empty_query ... ok
test sdk::tools::search::tests::test_search_nonexistent_path ... ok
test sdk::tools::wallet::tests::test_append_is_free ... ok
test sdk::tools::wallet::tests::test_deduct_and_credit ... ok
test sdk::tools::wallet::tests::test_genesis_allocation ... ok
test sdk::tools::wallet::tests::test_insufficient_balance_rejected ... ok
test sdk::tools::wallet::tests::test_negative_deduct_rejected ... ok
test sdk::tools::wallet::tests::test_no_double_genesis ... ok
test sdk::tools::wallet::tests::test_portfolio_tracking ... ok
test sdk::tools::wallet::tests::test_query_balance ... ok
test sdk::tools::wallet::tests::test_query_unknown_key ... ok
test sdk::tools::wallet::tests::test_unknown_agent_vetoed ... ok
test sdk::tools::wallet::tests::test_zero_deduct_rejected ... ok
test state::q_state::tests::btreemap_insertion_order_independent_serialization ... ok
test state::q_state::tests::economic_state_has_nine_sub_fields ... ok
test state::q_state::tests::genesis_q_state_is_total_and_default ... ok
test state::q_state::tests::nine_field_count_via_serde_json ... ok
test state::q_state::tests::node_id_from_state_root_is_deterministic ... ok
test sdk::sandbox::tests::test_sandbox_nonzero_exit ... ok
test state::typed_tx::tests::golden_challenge_tx_digest ... ok
test state::typed_tx::tests::golden_finalize_reward_tx_digest ... ok
test state::typed_tx::tests::golden_task_expire_tx_digest ... ok
test state::typed_tx::tests::golden_reuse_tx_digest ... ok
test state::typed_tx::tests::golden_verify_tx_digest ... ok
test state::typed_tx::tests::golden_terminal_summary_tx_digest ... ok
test state::typed_tx::tests::has_submitter_partitioning ... ok
test state::typed_tx::tests::golden_work_tx_digest ... ok
test state::typed_tx::tests::signing_payload_excludes_signature ... ok
test state::typed_tx::tests::signing_payload_domains_are_distinct ... ok
test state::typed_tx::tests::typed_tx_byte_stability_across_calls ... ok
test state::typed_tx::tests::typed_tx_btree_permutation_independence ... ok
test state::typed_tx::tests::typed_tx_default_round_trip ... ok
test state::typed_tx::tests::typed_tx_kind_projection ... ok
test state::typed_tx::tests::typed_tx_cross_variant_non_collision ... ok
test state::typed_tx::tests::typed_tx_round_trip_all_variants ... ok
test top_white::predicates::registry::tests::agent_visible_view_filters_private ... ok
test top_white::predicates::registry::tests::duplicate_id_rejected ... ok
test top_white::predicates::registry::tests::empty_id_rejected ... ok
test top_white::predicates::registry::tests::empty_registry_root_is_sha256_empty ... ok
test top_white::predicates::registry::tests::merkle_root_changes_on_register ... ok
test top_white::predicates::registry::tests::merkle_root_deterministic_two_runs ... ok
test top_white::predicates::registry::tests::metadata_canonical_hash_deterministic ... ok
test top_white::predicates::registry::tests::metadata_canonical_hash_differs_on_visibility ... ok
test top_white::predicates::registry::tests::register_and_get_round_trip ... ok
test top_white::predicates::visibility::tests::commit_reveal_pre_reveal ... ok
test top_white::predicates::visibility::tests::private_never_content_visible ... ok
test top_white::predicates::visibility::tests::public_always_visible ... ok
test top_white::predicates::visibility::tests::serde_round_trip_commit_reveal ... ok
test top_white::predicates::visibility::tests::serde_round_trip_public ... ok
test wal::tests::test_wal_replay_missing_file_is_empty ... ok
test state::typed_tx::tests::typed_tx_round_trip_100_inputs ... ok
test wal::tests::test_wal_roundtrip_nodes_only ... ok
test wal::tests::test_wal_skip_malformed_line ... ok
test wal::tests::test_wal_roundtrip_mixed ... ok
test boot::tests::verify_trust_root_passes_on_intact_repo ... ok
test sdk::sandbox::tests::test_sandbox_timeout_kills_process ... ok

test result: ok. 221 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.20s


exec
/bin/bash -lc 'rg -n "crate::state|state::typed_tx|typed_tx::|TerminalSummaryTx|WorkTx|TypedTx|TaskExpireTx|FinalizeRewardTx" src/bottom_white src/economy' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
src/economy/money.rs:142:/// Newtype on `MicroCoin` for `WorkTx::stake`, `VerifyTx::bond`, `ChallengeTx::stake`
src/bottom_white/cas/schema.rs:12:/// `WorkTx.proposal_cid: Cid` references this.
src/bottom_white/ledger/transition_ledger.rs:42:use crate::state::q_state::Hash;
src/bottom_white/ledger/system_keypair.rs:102:/// `state::typed_tx::AgentSignature`) can reuse the same adapter — keeps the
src/bottom_white/ledger/system_keypair.rs:178:// the typed `TerminalSummaryTx` struct (8-field per STATE § 1.5) now lives in
src/bottom_white/ledger/system_keypair.rs:179:// `state::typed_tx`. system_keypair signs an opaque digest via the
src/bottom_white/ledger/system_keypair.rs:230:    /// of the 8-field `state::typed_tx::TerminalSummaryTx` is computed in
src/bottom_white/ledger/system_keypair.rs:554:/// digest produced by `state::typed_tx::TerminalSummaryTx::canonical_digest()`

exec
/bin/bash -lc "rg -n \"FinalizeRewardSigning|TaskExpireSigning|TerminalSummarySigning|CanonicalMessage::.*Signing|sign_.*finalize|sign_.*expire|verify_system_signature\\(\" src/bottom_white/ledger/system_keypair.rs src/state/typed_tx.rs src -g '*.rs'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
src/state/mod.rs:24:    FinalizeRewardSigningPayload, FinalizeRewardTx, HasSubmitter, PredicateId,
src/state/mod.rs:26:    SafetyOrCreation, SignalBundle, SignalKind, SlashEvidenceCid, TaskExpireSigningPayload,
src/state/mod.rs:27:    TaskExpireTx, TaskId, TerminalSummarySigningPayload, TerminalSummaryTx, ToolId,
src/state/typed_tx.rs:333:/// an opaque `TerminalSummarySigning([u8; 32])` digest — same opaque-digest
src/state/typed_tx.rs:449:pub struct FinalizeRewardSigningPayload {
src/state/typed_tx.rs:460:impl FinalizeRewardSigningPayload {
src/state/typed_tx.rs:468:pub struct TaskExpireSigningPayload {
src/state/typed_tx.rs:477:impl TaskExpireSigningPayload {
src/state/typed_tx.rs:485:pub struct TerminalSummarySigningPayload {
src/state/typed_tx.rs:495:impl TerminalSummarySigningPayload {
src/state/typed_tx.rs:547:    pub fn to_signing_payload(&self) -> FinalizeRewardSigningPayload {
src/state/typed_tx.rs:548:        FinalizeRewardSigningPayload {
src/state/typed_tx.rs:562:    pub fn to_signing_payload(&self) -> TaskExpireSigningPayload {
src/state/typed_tx.rs:563:        TaskExpireSigningPayload {
src/state/typed_tx.rs:575:    pub fn to_signing_payload(&self) -> TerminalSummarySigningPayload {
src/state/typed_tx.rs:576:        TerminalSummarySigningPayload {
src/bottom_white/ledger/transition_ledger.rs:26://!   ride a `CanonicalMessage::LedgerEntrySigning(_)` variant when CO1.7.5+ extends
src/bottom_white/ledger/transition_ledger.rs:791:    // Verifies: (a) typed sign API works; (b) signature verifies via CanonicalMessage::LedgerEntrySigning;
src/bottom_white/ledger/transition_ledger.rs:824:        let msg_clean = CanonicalMessage::LedgerEntrySigning(digest.0);
src/bottom_white/ledger/transition_ledger.rs:826:            verify_system_signature(&sig, &msg_clean, epoch, &pinned),
src/bottom_white/ledger/transition_ledger.rs:834:        let msg_tamper = CanonicalMessage::LedgerEntrySigning(digest_tamper.0);
src/bottom_white/ledger/transition_ledger.rs:836:            !verify_system_signature(&sig, &msg_tamper, epoch, &pinned),
src/bottom_white/ledger/transition_ledger.rs:849:        let msg_other_epoch = CanonicalMessage::LedgerEntrySigning(digest_other_epoch.0);
src/bottom_white/ledger/transition_ledger.rs:851:            !verify_system_signature(&sig, &msg_other_epoch, epoch, &pinned),
src/state/typed_tx.rs:333:/// an opaque `TerminalSummarySigning([u8; 32])` digest — same opaque-digest
src/state/typed_tx.rs:449:pub struct FinalizeRewardSigningPayload {
src/state/typed_tx.rs:460:impl FinalizeRewardSigningPayload {
src/state/typed_tx.rs:468:pub struct TaskExpireSigningPayload {
src/state/typed_tx.rs:477:impl TaskExpireSigningPayload {
src/state/typed_tx.rs:485:pub struct TerminalSummarySigningPayload {
src/state/typed_tx.rs:495:impl TerminalSummarySigningPayload {
src/state/typed_tx.rs:547:    pub fn to_signing_payload(&self) -> FinalizeRewardSigningPayload {
src/state/typed_tx.rs:548:        FinalizeRewardSigningPayload {
src/state/typed_tx.rs:562:    pub fn to_signing_payload(&self) -> TaskExpireSigningPayload {
src/state/typed_tx.rs:563:        TaskExpireSigningPayload {
src/state/typed_tx.rs:575:    pub fn to_signing_payload(&self) -> TerminalSummarySigningPayload {
src/state/typed_tx.rs:576:        TerminalSummarySigningPayload {
src/bottom_white/ledger/system_keypair.rs:180:// `CanonicalMessage::TerminalSummarySigning([u8; 32])` variant — same
src/bottom_white/ledger/system_keypair.rs:234:    TerminalSummarySigning([u8; 32]),
src/bottom_white/ledger/system_keypair.rs:463:        CanonicalMessage::TerminalSummarySigning(digest) => {
src/bottom_white/ledger/system_keypair.rs:464:            h.update(b"TerminalSummarySigning");
src/bottom_white/ledger/system_keypair.rs:475:        CanonicalMessage::LedgerEntrySigning(digest) => {
src/bottom_white/ledger/system_keypair.rs:484:pub fn verify_system_signature(
src/bottom_white/ledger/system_keypair.rs:510:    verify_system_signature(old_signature, &message, proof.old_epoch(), pinned_pubkeys)
src/bottom_white/ledger/system_keypair.rs:511:        && verify_system_signature(new_signature, &message, proof.new_epoch(), pinned_pubkeys)
src/bottom_white/ledger/system_keypair.rs:570:        sign_system_message_inner(keypair, &CanonicalMessage::TerminalSummarySigning(digest))
src/bottom_white/ledger/system_keypair.rs:613:            &CanonicalMessage::LedgerEntrySigning(signing_payload_digest),
src/bottom_white/ledger/system_keypair.rs:898:        assert!(verify_system_signature(
src/bottom_white/ledger/system_keypair.rs:180:// `CanonicalMessage::TerminalSummarySigning([u8; 32])` variant — same
src/bottom_white/ledger/system_keypair.rs:234:    TerminalSummarySigning([u8; 32]),
src/bottom_white/ledger/system_keypair.rs:463:        CanonicalMessage::TerminalSummarySigning(digest) => {
src/bottom_white/ledger/system_keypair.rs:464:            h.update(b"TerminalSummarySigning");
src/bottom_white/ledger/system_keypair.rs:475:        CanonicalMessage::LedgerEntrySigning(digest) => {
src/bottom_white/ledger/system_keypair.rs:484:pub fn verify_system_signature(
src/bottom_white/ledger/system_keypair.rs:510:    verify_system_signature(old_signature, &message, proof.old_epoch(), pinned_pubkeys)
src/bottom_white/ledger/system_keypair.rs:511:        && verify_system_signature(new_signature, &message, proof.new_epoch(), pinned_pubkeys)
src/bottom_white/ledger/system_keypair.rs:570:        sign_system_message_inner(keypair, &CanonicalMessage::TerminalSummarySigning(digest))
src/bottom_white/ledger/system_keypair.rs:613:            &CanonicalMessage::LedgerEntrySigning(signing_payload_digest),
src/bottom_white/ledger/system_keypair.rs:898:        assert!(verify_system_signature(

exec
/bin/bash -lc 'rg -n "EXPECTED_HEX|golden_|signing_payload|domains_are_distinct|excludes_signature|btree_permutation|cross_variant" src/state/typed_tx.rs' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
504:    pub fn to_signing_payload(&self) -> WorkSigningPayload {
521:    pub fn to_signing_payload(&self) -> VerifySigningPayload {
534:    pub fn to_signing_payload(&self) -> ChallengeSigningPayload {
547:    pub fn to_signing_payload(&self) -> FinalizeRewardSigningPayload {
562:    pub fn to_signing_payload(&self) -> TaskExpireSigningPayload {
575:    pub fn to_signing_payload(&self) -> TerminalSummarySigningPayload {
1128:    fn typed_tx_cross_variant_non_collision() {
1166:    fn typed_tx_btree_permutation_independence() {
1215:    fn signing_payload_domains_are_distinct() {
1217:            ("Work", fixture_work_tx().to_signing_payload().canonical_digest()),
1220:                fixture_verify_tx().to_signing_payload().canonical_digest(),
1224:                fixture_challenge_tx().to_signing_payload().canonical_digest(),
1229:                    .to_signing_payload()
1235:                    .to_signing_payload()
1241:                    .to_signing_payload()
1260:    fn signing_payload_excludes_signature() {
1262:        let d_clean = tx_clean.to_signing_payload().canonical_digest();
1266:        let d_mut_sig = tx_mut.to_signing_payload().canonical_digest();
1272:        let d_signed = tx_signed_change.to_signing_payload().canonical_digest();
1282:    //   1. Run `cargo test --lib state::typed_tx::tests::golden_` with current code
1284:    //   3. Update each `EXPECTED_HEX` constant + cite the rotation rationale in commit message
1286:    const EXPECTED_HEX_WORK: &str =
1288:    const EXPECTED_HEX_VERIFY: &str =
1290:    const EXPECTED_HEX_CHALLENGE: &str =
1292:    const EXPECTED_HEX_REUSE: &str =
1294:    const EXPECTED_HEX_FINALIZE_REWARD: &str =
1296:    const EXPECTED_HEX_TASK_EXPIRE: &str =
1298:    const EXPECTED_HEX_TERMINAL_SUMMARY: &str =
1302:    fn golden_work_tx_digest() {
1305:        assert_eq!(actual, EXPECTED_HEX_WORK, "Work canonical digest changed");
1309:    fn golden_verify_tx_digest() {
1311:        assert_eq!(actual, EXPECTED_HEX_VERIFY);
1315:    fn golden_challenge_tx_digest() {
1317:        assert_eq!(actual, EXPECTED_HEX_CHALLENGE);
1321:    fn golden_reuse_tx_digest() {
1323:        assert_eq!(actual, EXPECTED_HEX_REUSE);
1327:    fn golden_finalize_reward_tx_digest() {
1329:        assert_eq!(actual, EXPECTED_HEX_FINALIZE_REWARD);
1333:    fn golden_task_expire_tx_digest() {
1335:        assert_eq!(actual, EXPECTED_HEX_TASK_EXPIRE);
1339:    fn golden_terminal_summary_tx_digest() {
1341:        assert_eq!(actual, EXPECTED_HEX_TERMINAL_SUMMARY);

codex
# Codex CO1.1.4-pre1 Round-2 Audit

## Q-1 P1 closure
Mostly closed in code. The 6 payloads are in [typed_tx.rs](/home/zephryj/projects/turingosv4/src/state/typed_tx.rs:392), [413](/home/zephryj/projects/turingosv4/src/state/typed_tx.rs:413), [430](/home/zephryj/projects/turingosv4/src/state/typed_tx.rs:430), [447](/home/zephryj/projects/turingosv4/src/state/typed_tx.rs:447), [466](/home/zephryj/projects/turingosv4/src/state/typed_tx.rs:466), [483](/home/zephryj/projects/turingosv4/src/state/typed_tx.rs:483). `domain_prefixed_digest` does `sha256(domain || canonical_encode(value))` at [383-389](/home/zephryj/projects/turingosv4/src/state/typed_tx.rs:383).

Code is correct. Tests are weaker than the code: `signing_payload_domains_are_distinct` uses different bodies, so it would still pass if domains were removed. `signing_payload_excludes_signature` proves this only for `WorkTx`, though projections for all signed txs exclude signatures at [503-586](/home/zephryj/projects/turingosv4/src/state/typed_tx.rs:503).

## Q-2 P2 closure
Partially closed. `ClaimId(pub TxId)` is `#[serde(transparent)]` at [50-52](/home/zephryj/projects/turingosv4/src/state/typed_tx.rs:50), and `FinalizeRewardTx.claim_id` is updated at [299-308](/home/zephryj/projects/turingosv4/src/state/typed_tx.rs:299). Fixture uses `ClaimId::new()` at [982](/home/zephryj/projects/turingosv4/src/state/typed_tx.rs:982).

Missed call site: `SignalKind::Finalize { claim_id: TxId }` and `SignalBundle::finalize(claim_id: TxId, ...)` still leak the old type at [826-854](/home/zephryj/projects/turingosv4/src/state/typed_tx.rs:826). Because `SignalBundle` is part of this ABI surface, P2 is not fully closed.

## Q-3 P3 closure
Implementation closure: yes. New 8-field `TerminalSummaryTx` matches STATE §1.5 and is in [typed_tx.rs:337-347](/home/zephryj/projects/turingosv4/src/state/typed_tx.rs:337). `system_keypair` has `CanonicalMessage::TerminalSummarySigning([u8; 32])` at [223-234](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/system_keypair.rs:223), digest arm at [463-466](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/system_keypair.rs:463), and `sign_terminal_summary(..., digest: [u8; 32])` at [566-570](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/system_keypair.rs:566). Grep shows no old struct in `system_keypair.rs`.

Spec/doc hygiene is not closed: CO1.1.4 §0 still says `TerminalSummaryTx` exists in `system_keypair.rs` at spec line 47, §6 says “imported from system_keypair” at line 210, and §9 still contains D-3 at line 284 despite P7 claiming the row was removed.

## Q-4 P4 closure
STATE §3.1-3.7 invoked variants are covered by [TransitionError](/home/zephryj/projects/turingosv4/src/state/typed_tx.rs:705): stale parent, sigs, stake, target lookup, predicate failures, challenge window, tool errors, finalize, task-expire, terminal summary, plus `NotYetImplemented`.

Payloads are acceptable for an ABI skeleton if richer rejection context is logged elsewhere, but one rule should be made explicit later: when a predicate bundle has multiple failures, transition code must choose/report predicate IDs deterministically. Also, CO1.1.4 §4.1 introduces wire-vs-Q mismatch rejection, but there is no dedicated `FinalizeRewardSummaryMismatch`-style error.

## Q-5 P5 closure
Golden fixtures are load-bearing: hardcoded constants at [1286-1299](/home/zephryj/projects/turingosv4/src/state/typed_tx.rs:1286), assertions at [1301-1341](/home/zephryj/projects/turingosv4/src/state/typed_tx.rs:1301). All 7 variants are in round-trip, kind projection, cross-variant non-collision, default round-trip, and golden tests.

`cargo test --lib` passes: 221/221. The typed_tx subset passes 17/17.

Caveat: BTree permutation coverage only exercises `BTreeSet` `read_set`, not `BTreeMap` fields such as predicate results or terminal histogram.

## Q-6 P6 closure
Spec §7.1 now matches bincode 2.0.1 behavior: serde variant index is `u32` and fixed big-endian `u32` writes via bincode source; `usize` lengths encode as fixed `u64`. Local source confirms `serialize_*_variant(... variant_index: u32)` and `usize` fixed BE casts to `u64`.

Keeping u32/u64 is the right call. Forcing u8 would require custom serde and would rotate all TypedTx payload CIDs. CO1.7 `canonical_encode` uses the same bincode config at [359-370](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/transition_ledger.rs:359), so digest stability depends on not changing this.

## Q-7 P8+P9 closure
P8 is conceptually strong: Q-derived fields are explicit in spec §4.1 lines 173-180, and dual-sign rationale is clear in §4.2 lines 184-188.

But implementation support is incomplete: `system_keypair` has TerminalSummary and LedgerEntry signing variants, but no `FinalizeRewardSigning` or `TaskExpireSigning` `CanonicalMessage` variants/emitter functions. Since `FinalizeRewardTx.system_signature` and `TaskExpireTx.system_signature` are retained, the authorized signing/verification path is missing.

P9 is binding enough: §0.1 says A4 “MUST NOT ship before CO1.4-extra” and calls that a necessary condition for PASS at spec lines 72-77.

## Q-8 P10 closure
Adequate honesty. QState still has TxId-keyed `EscrowsIndex`, `StakesIndex`, `ClaimsIndex`, and `TaskMarketsIndex` at [q_state.rs:161](/home/zephryj/projects/turingosv4/src/state/q_state.rs:161), [182](/home/zephryj/projects/turingosv4/src/state/q_state.rs:182), [201](/home/zephryj/projects/turingosv4/src/state/q_state.rs:201), [224](/home/zephryj/projects/turingosv4/src/state/q_state.rs:224). Spec §9 D-4 correctly assigns the retrofit to CO P2.1. No typed-tx wire consequence, because the wire already uses `TaskId`.

## Q-9 New defects
1. `SignalBundle::Finalize` still uses `TxId`, not `ClaimId`.
2. FinalizeReward/TaskExpire payload signatures have digest structs but no system-keypair `CanonicalMessage` variants or authorized emitter APIs.
3. Spec/doc drift remains around TerminalSummary ownership and D-3 removal.
4. Signing-payload tests lack golden hex and a same-body domain-prefix test; current domain test is non-load-bearing.
5. `cargo check` passes; warnings are unrelated existing unused/dead-code warnings. No `sha2::{Digest, Sha256}` collision and no Default expansion failure.

## Q-10 Implementation gating
CO1.7 A2/A3 skeleton work can proceed after fixing the local ABI issues above. End-to-end system-emitted tx handling is blocked until FinalizeReward/TaskExpire system signing is wired. A4 remains correctly blocked on CO1.4-extra CAS index persistence.

## **VERDICT**: CHALLENGE

## Top must-fix
1. Change `SignalKind::Finalize` and `SignalBundle::finalize` from `TxId` to `ClaimId`; add a regression test/grep for claim-id API leakage.
2. Add typed `CanonicalMessage` variants + authorized emitter/verification tests for `FinalizeRewardSigningPayload` and `TaskExpireSigningPayload`, or remove/defer those `system_signature` fields. Current dual-sign story is not executable for two of three system txs.
3. Clean spec drift: remove D-3 from §9 or stop claiming removal; fix stale “TerminalSummaryTx in/imported from system_keypair” references.
4. Add signing-payload golden digests and a load-bearing domain test using identical body bytes with different domains.

## Conviction
High.
2026-04-28T11:58:21.240691Z ERROR codex_core::session: failed to record rollout items: thread 019dd3f0-80ea-7763-a21e-6bb5666e3411 not found
tokens used
165,930
# Codex CO1.1.4-pre1 Round-2 Audit

## Q-1 P1 closure
Mostly closed in code. The 6 payloads are in [typed_tx.rs](/home/zephryj/projects/turingosv4/src/state/typed_tx.rs:392), [413](/home/zephryj/projects/turingosv4/src/state/typed_tx.rs:413), [430](/home/zephryj/projects/turingosv4/src/state/typed_tx.rs:430), [447](/home/zephryj/projects/turingosv4/src/state/typed_tx.rs:447), [466](/home/zephryj/projects/turingosv4/src/state/typed_tx.rs:466), [483](/home/zephryj/projects/turingosv4/src/state/typed_tx.rs:483). `domain_prefixed_digest` does `sha256(domain || canonical_encode(value))` at [383-389](/home/zephryj/projects/turingosv4/src/state/typed_tx.rs:383).

Code is correct. Tests are weaker than the code: `signing_payload_domains_are_distinct` uses different bodies, so it would still pass if domains were removed. `signing_payload_excludes_signature` proves this only for `WorkTx`, though projections for all signed txs exclude signatures at [503-586](/home/zephryj/projects/turingosv4/src/state/typed_tx.rs:503).

## Q-2 P2 closure
Partially closed. `ClaimId(pub TxId)` is `#[serde(transparent)]` at [50-52](/home/zephryj/projects/turingosv4/src/state/typed_tx.rs:50), and `FinalizeRewardTx.claim_id` is updated at [299-308](/home/zephryj/projects/turingosv4/src/state/typed_tx.rs:299). Fixture uses `ClaimId::new()` at [982](/home/zephryj/projects/turingosv4/src/state/typed_tx.rs:982).

Missed call site: `SignalKind::Finalize { claim_id: TxId }` and `SignalBundle::finalize(claim_id: TxId, ...)` still leak the old type at [826-854](/home/zephryj/projects/turingosv4/src/state/typed_tx.rs:826). Because `SignalBundle` is part of this ABI surface, P2 is not fully closed.

## Q-3 P3 closure
Implementation closure: yes. New 8-field `TerminalSummaryTx` matches STATE §1.5 and is in [typed_tx.rs:337-347](/home/zephryj/projects/turingosv4/src/state/typed_tx.rs:337). `system_keypair` has `CanonicalMessage::TerminalSummarySigning([u8; 32])` at [223-234](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/system_keypair.rs:223), digest arm at [463-466](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/system_keypair.rs:463), and `sign_terminal_summary(..., digest: [u8; 32])` at [566-570](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/system_keypair.rs:566). Grep shows no old struct in `system_keypair.rs`.

Spec/doc hygiene is not closed: CO1.1.4 §0 still says `TerminalSummaryTx` exists in `system_keypair.rs` at spec line 47, §6 says “imported from system_keypair” at line 210, and §9 still contains D-3 at line 284 despite P7 claiming the row was removed.

## Q-4 P4 closure
STATE §3.1-3.7 invoked variants are covered by [TransitionError](/home/zephryj/projects/turingosv4/src/state/typed_tx.rs:705): stale parent, sigs, stake, target lookup, predicate failures, challenge window, tool errors, finalize, task-expire, terminal summary, plus `NotYetImplemented`.

Payloads are acceptable for an ABI skeleton if richer rejection context is logged elsewhere, but one rule should be made explicit later: when a predicate bundle has multiple failures, transition code must choose/report predicate IDs deterministically. Also, CO1.1.4 §4.1 introduces wire-vs-Q mismatch rejection, but there is no dedicated `FinalizeRewardSummaryMismatch`-style error.

## Q-5 P5 closure
Golden fixtures are load-bearing: hardcoded constants at [1286-1299](/home/zephryj/projects/turingosv4/src/state/typed_tx.rs:1286), assertions at [1301-1341](/home/zephryj/projects/turingosv4/src/state/typed_tx.rs:1301). All 7 variants are in round-trip, kind projection, cross-variant non-collision, default round-trip, and golden tests.

`cargo test --lib` passes: 221/221. The typed_tx subset passes 17/17.

Caveat: BTree permutation coverage only exercises `BTreeSet` `read_set`, not `BTreeMap` fields such as predicate results or terminal histogram.

## Q-6 P6 closure
Spec §7.1 now matches bincode 2.0.1 behavior: serde variant index is `u32` and fixed big-endian `u32` writes via bincode source; `usize` lengths encode as fixed `u64`. Local source confirms `serialize_*_variant(... variant_index: u32)` and `usize` fixed BE casts to `u64`.

Keeping u32/u64 is the right call. Forcing u8 would require custom serde and would rotate all TypedTx payload CIDs. CO1.7 `canonical_encode` uses the same bincode config at [359-370](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/transition_ledger.rs:359), so digest stability depends on not changing this.

## Q-7 P8+P9 closure
P8 is conceptually strong: Q-derived fields are explicit in spec §4.1 lines 173-180, and dual-sign rationale is clear in §4.2 lines 184-188.

But implementation support is incomplete: `system_keypair` has TerminalSummary and LedgerEntry signing variants, but no `FinalizeRewardSigning` or `TaskExpireSigning` `CanonicalMessage` variants/emitter functions. Since `FinalizeRewardTx.system_signature` and `TaskExpireTx.system_signature` are retained, the authorized signing/verification path is missing.

P9 is binding enough: §0.1 says A4 “MUST NOT ship before CO1.4-extra” and calls that a necessary condition for PASS at spec lines 72-77.

## Q-8 P10 closure
Adequate honesty. QState still has TxId-keyed `EscrowsIndex`, `StakesIndex`, `ClaimsIndex`, and `TaskMarketsIndex` at [q_state.rs:161](/home/zephryj/projects/turingosv4/src/state/q_state.rs:161), [182](/home/zephryj/projects/turingosv4/src/state/q_state.rs:182), [201](/home/zephryj/projects/turingosv4/src/state/q_state.rs:201), [224](/home/zephryj/projects/turingosv4/src/state/q_state.rs:224). Spec §9 D-4 correctly assigns the retrofit to CO P2.1. No typed-tx wire consequence, because the wire already uses `TaskId`.

## Q-9 New defects
1. `SignalBundle::Finalize` still uses `TxId`, not `ClaimId`.
2. FinalizeReward/TaskExpire payload signatures have digest structs but no system-keypair `CanonicalMessage` variants or authorized emitter APIs.
3. Spec/doc drift remains around TerminalSummary ownership and D-3 removal.
4. Signing-payload tests lack golden hex and a same-body domain-prefix test; current domain test is non-load-bearing.
5. `cargo check` passes; warnings are unrelated existing unused/dead-code warnings. No `sha2::{Digest, Sha256}` collision and no Default expansion failure.

## Q-10 Implementation gating
CO1.7 A2/A3 skeleton work can proceed after fixing the local ABI issues above. End-to-end system-emitted tx handling is blocked until FinalizeReward/TaskExpire system signing is wired. A4 remains correctly blocked on CO1.4-extra CAS index persistence.

## **VERDICT**: CHALLENGE

## Top must-fix
1. Change `SignalKind::Finalize` and `SignalBundle::finalize` from `TxId` to `ClaimId`; add a regression test/grep for claim-id API leakage.
2. Add typed `CanonicalMessage` variants + authorized emitter/verification tests for `FinalizeRewardSigningPayload` and `TaskExpireSigningPayload`, or remove/defer those `system_signature` fields. Current dual-sign story is not executable for two of three system txs.
3. Clean spec drift: remove D-3 from §9 or stop claiming removal; fix stale “TerminalSummaryTx in/imported from system_keypair” references.
4. Add signing-payload golden digests and a load-bearing domain test using identical body bytes with different domains.

## Conviction
High.
