# Codex CO1.1.4-pre1 Round-3 Audit
**Date**: 2026-04-28
**Target**: spec v1.2 + impl v1.2 + 20 tests joint artifact (closure check)
**Prompt size**: 147570 chars

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
session id: 019dd3fb-56f6-7941-8463-f04bfce6a019
--------
user
# Codex Round-3 Audit — CO1.1.4-pre1 v1.2 (narrow closure check)

**Role**: skeptical adversarial reviewer (round-3 narrow-scope closure check). Independent of Gemini round-3.

**Mandate**: round-3 closure verification on v1.2 (commit `f4649a9`). Round-2 returned CHALLENGE/PASS → conservative CHALLENGE. v1.2 claims to close 4 must-fix + 1 secondary + 3 Gemini recommendations.

Per CLAUDE.md "Audit Standard" + memory `feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS.

## What changed v1.1 → v1.2

| ID | v1.1 issue (round-2 finding) | v1.2 fix claim | Cite line/section to verify |
|---|---|---|---|
| **P11** | `SignalKind::Finalize.claim_id: TxId` leaked old type (P2 missed call site) | claim_id: ClaimId; SignalBundle::finalize signature updated | typed_tx.rs SignalKind::Finalize + SignalBundle::finalize impl |
| **P12** | No FinalizeRewardSigning / TaskExpireSigning CanonicalMessage variants → dual-sign path not executable for 2 of 3 system txs | NEW CanonicalMessage::FinalizeRewardSigning([u8;32]) + TaskExpireSigning([u8;32]) + canonical_digest match arms + sign_finalize_reward + sign_task_expire emitter fns | system_keypair.rs CanonicalMessage enum + canonical_digest match + terminal_summary_emitter mod |
| **P13** | Spec drift: § 0/§ 6/§ 9 still referenced TerminalSummaryTx in system_keypair after P3 migration | § 0 lists state::typed_tx; § 6 inline comment updated; § 9 D-3 row REMOVED (HTML comment marker) | spec § 0 / § 6 / § 9 |
| **P14** | Domain-prefix tests not load-bearing (round-1 used different bodies) | NEW signing_payload_domain_prefix_is_load_bearing (identical body, 6 distinct digests) + extended signing_payload_excludes_signature to all 6 + NEW signing_payload_golden_digests with 6 locked hex | typed_tx.rs tests |
| **P15** | BTreeMap permutation only covered BTreeSet | NEW typed_tx_btreemap_permutation_independence using PredicateResultsBundle.acceptance | typed_tx.rs tests |
| **GR-1** | MetaTx domain not reserved | NEW DOMAIN_AGENT_META_PROPOSAL constant (#[allow(dead_code)]) | typed_tx.rs |
| **GR-2** | TransitionError additive-only commitment absent | spec § 7.2 NEW additive-only commitment for all ABI enums | spec § 7.2 |
| **GR-3** | Domain rotation process undocumented | spec § 7.3 NEW rotation process | spec § 7.3 |

## Round-3 narrow closure questions

**Q1. P11 closure**: SignalKind::Finalize.claim_id is now ClaimId? SignalBundle::finalize takes ClaimId? Any other site grep would catch (e.g. test fixtures, doc-comments) still using TxId for claim references?

**Q2. P12 closure**: CanonicalMessage has both new variants? canonical_digest match exhaustive (no compile warning on new variants)? sign_finalize_reward / sign_task_expire actually invokable + symmetric to sign_terminal_summary? Any other code path that would need updating for the new variants (e.g. a verify-side counterpart)?

**Q3. P13 closure**: § 0 line 47 fixed? § 6 line 210 fixed? § 9 D-3 row actually removed (HTML comment present, no D-3 row visible)? Any OTHER stale TerminalSummaryTx-in-system_keypair reference that grep would catch?

**Q4. P14 closure**: signing_payload_domain_prefix_is_load_bearing test — does it ACTUALLY use identical body bytes (verify by reading the test)? signing_payload_excludes_signature now covers all 6 signed tx kinds? signing_payload_golden_digests has 6 locked EXPECTED_SIGNING_HEX_* constants matching the runtime computation?

**Q5. P15 closure**: typed_tx_btreemap_permutation_independence test uses PredicateResultsBundle.acceptance? Three different insertion orders produce byte-identical encoding?

**Q6. GR-1/2/3 closures**: DOMAIN_AGENT_META_PROPOSAL constant present? Spec § 7.2 + § 7.3 sections present and clear?

**Q7. NEW defects introduced by v1.2**: anything broken or unintentionally regressed? cargo test --lib still 224/0?

**Q8. PASS gate**: does v1.2 close all R2 findings cleanly enough for CO1.7-impl A2 unblock?

## Output format

# Codex CO1.1.4-pre1 Round-3 Audit
## Q1 P11 closure
## Q2 P12 closure
## Q3 P13 closure
## Q4 P14 closure
## Q5 P15 closure
## Q6 GR-1/2/3 closures
## Q7 New defects
## Q8 PASS gate
## **VERDICT**: PASS / CHALLENGE / VETO
## Top must-fix (if CHALLENGE)
## Conviction (low/med/high)

Be terse. Cite line numbers. PASS = all R2 must-fix closed; CHALLENGE = at least one specific patch-mechanical defect remaining; VETO = foundational design flaw (unlikely at this stage).

---


# CO1.1.4-pre1 spec v1.2 (target)
# CO1.1.4-pre1 — Typed Tx ABI Surface (v1.2)

**Status**: v1.2 — round-2 returned PASS (Gemini, high) + CHALLENGE (Codex, high; 4 patch-mechanical defects). Conservative merged CHALLENGE. v1.2 closes the 4 must-fix items (P11-P14 below) + 1 secondary (P15) + 3 Gemini recommendations (GR-1/2/3). Awaiting round-3.
**Status (v1.1)**: round-1 dual audit returned CHALLENGE/CHALLENGE; v1.1 closed 10 patches (P1-P10).
**Status (v1)**: v1 DRAFT, post-CO1.7 PASS/PASS gate (2026-04-28).
**Author**: ArchitectAI (Claude); session 2026-04-28 (continued).
**Round-1 verdicts**: `handover/audits/CODEX_CO1_1_4_PRE1_ROUND1_AUDIT_2026-04-28.md` (CHALLENGE/high) + `handover/audits/GEMINI_CO1_1_4_PRE1_ROUND1_AUDIT_2026-04-28.md` (CHALLENGE/high); merged in `handover/audits/CO1_1_4_PRE1_DUAL_AUDIT_VERDICT_R1_2026-04-28.md`.
**Round-2 verdicts**: `handover/audits/CODEX_CO1_1_4_PRE1_ROUND2_AUDIT_2026-04-28.md` (CHALLENGE/high) + `handover/audits/GEMINI_CO1_1_4_PRE1_ROUND2_AUDIT_2026-04-28.md` (PASS/high); merged in `handover/audits/CO1_1_4_PRE1_DUAL_AUDIT_VERDICT_R2_2026-04-28.md`.

## v1.2 patch log (vs. v1.1) — round-2 closure

| ID | v1.1 issue | v1.2 fix | Source |
|---|---|---|---|
| **P11** | `SignalKind::Finalize` and `SignalBundle::finalize` still used `TxId` (v1.1 P2 missed call site) | `SignalKind::Finalize.claim_id: ClaimId` + `SignalBundle::finalize(claim_id: ClaimId, ...)` | R2-1 (Codex Q-2) |
| **P12** | `FinalizeRewardTx.system_signature` and `TaskExpireTx.system_signature` retained but no `CanonicalMessage::FinalizeRewardSigning` / `::TaskExpireSigning` variants + no emitter fns. Dual-sign rationale (§ 4.2) not executable for 2 of 3 system txs | NEW `CanonicalMessage::FinalizeRewardSigning([u8; 32])` + `TaskExpireSigning([u8; 32])` variants (system_keypair.rs) + canonical_digest match arms + `transition_emitter::sign_finalize_reward` + `sign_task_expire` symmetric emitter fns | R2-2 (Codex Q-7) |
| **P13** | Spec drift: § 0 line 47 still said "TerminalSummaryTx in system_keypair.rs"; § 6 line 210 said "imported from system_keypair"; § 9 D-3 row still present despite v1.1 P7 claiming removal | All 3 stale references cleaned: § 0 lists TerminalSummaryTx in `state::typed_tx`; § 6 inline comment updated; § 9 D-3 row REMOVED (HTML comment marker placed in its slot) | R2-3 (Codex Q-3) |
| **P14** | Signing-payload tests not load-bearing: round-1 `signing_payload_domains_are_distinct` used different bodies (would pass even without domain prefix); `signing_payload_excludes_signature` only tested for WorkTx; no signing-payload golden hex | NEW `signing_payload_domain_prefix_is_load_bearing` test (identical 64-byte body across 6 domains → 6 pairwise-distinct digests; would FAIL without domain prefix). `signing_payload_excludes_signature` extended to all 6 signed tx kinds. NEW `signing_payload_golden_digests` test with locked SHA-256 hex per signing payload (6 EXPECTED_SIGNING_HEX_* constants). | R2-4 (Codex Q-9) |
| **P15** | BTreeMap permutation only covered BTreeSet (read_set); BTreeMap fields (predicate_results, failure_class_histogram) untested for permutation independence | NEW `typed_tx_btreemap_permutation_independence` test using `predicate_results.acceptance` (BTreeMap<PredicateId, BoolWithProof>) | Codex round-2 secondary (Q-5 caveat) |
| **GR-1** (Gemini PASS recommendation) | MetaTx domain prefix not reserved; v4.1 namespace might force domain rotation later | NEW `DOMAIN_AGENT_META_PROPOSAL: &[u8] = b"turingosv4.agent_sig.meta_proposal.v1"` constant (typed_tx.rs); marked `#[allow(dead_code)]` until v4.1 wires MetaTx | Gemini Q9 / GR-1 |
| **GR-2** (Gemini recommendation) | TransitionError additive-only commitment not stated | spec § 7.2 NEW: TransitionError variants in v4 are additive-only; never reorder; new variants append at the end | Gemini Q9 / GR-2 |
| **GR-3** (Gemini recommendation) | Domain-string rotation process not documented | spec § 7.3 NEW: domain rotation = new constant (`*.v2`) added in parallel; old `.v1` retained until all-replay window passes; bumping major version triggers a v2 spec round | Gemini Q9 / GR-3 |

5 must-fix patches + 1 secondary + 3 Gemini recommendations = **9 closures** integrated below.



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
8. **Typed-tx payload structs** (all in `state::typed_tx`): `WorkTx`, `VerifyTx`, `ChallengeTx`, `ReuseTx`, `FinalizeRewardTx`, `TaskExpireTx`, `TerminalSummaryTx` (8-field STATE § 1.5 schema; v1.1 P3 migrated from a 3-field placeholder previously in `system_keypair.rs`).
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
    TerminalSummary(TerminalSummaryTx),  // 8-field schema in state::typed_tx (v1.1 P3)
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

### § 7.2 TransitionError additive-only commitment (v1.2 GR-2 per Gemini round-2)

`TransitionError` variants in **v4 are additive-only**:
- New variants MUST be APPENDED to the existing list (no insertion that would shift downstream variant indices).
- Existing variants MUST NOT be reordered (bincode emits variant-index-as-u32-BE; reordering changes the wire format and invalidates locked golden fixtures).
- Variant removal is NOT permitted within v4; deprecated error classes get a doc-comment "deprecated; replaced by X" but the variant stays.
- Bumping the major version (v4 → v5) is the only path that allows non-additive changes; that triggers an ABI rotation cycle (re-audit + re-fixture + canonical re-encoding migration).

This rule applies symmetrically to other ABI enums frozen in this atom: `TxStatus` / `RejectionClass` / `VerifyVerdict` / `RunOutcome` / `SafetyOrCreation` / `SignalKind` / `CanonicalMessage` / `TxKind` (transition_ledger.rs).

### § 7.3 Domain-string rotation process (v1.2 GR-3 per Gemini round-2)

If a future audit finds a security defect in a domain prefix (`b"turingosv4.<actor>.<purpose>.v1"`), rotation follows this discipline:

1. **Add NEW constant** (`*.v2`) **in parallel**; do NOT delete the old `*.v1` constant.
2. **Old `*.v1` MUST remain reachable in code** until the runtime can prove no in-flight tx still uses it (replay-window quiescence).
3. **New transitions emit only `*.v2` digests**; the runtime accepts both digests during the rotation window.
4. **Bump the v4 spec minor version** with a "domain rotation" entry in the patch log.
5. **Lock new golden hex** for v2-domain digests; v1-domain digests stay locked too (so historical replay still verifies).

The `.v1` suffix on every current domain constant is the affordance that makes this protocol possible without ambiguity.

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
<!-- v1.2 (R2-3 closure): D-3 row removed. Migration is complete; no divergence remains. -->
| **D-4** (v1.1 NEW per Codex Q-J / CX-3) | QState `task_markets_t` / `escrows_t` / `stakes_t` keyed by `TxId` (q_state.rs:201/161/182) but typed_tx schemas use `TaskId` for the same task references | **NOT retrofit in this atom**. Migration owned by **CO P2.1 (TaskMarket atom)** which will rekey the QState indices to `TaskId`. CO1.1.4-pre1 documents the cross-atom debt; no wire-format consequence (the typed-tx schemas already use `TaskId` correctly per STATE § 1.2). |

---

## § 10 Audit gates

| Round | Codex | Gemini | Conservative | Action |
|---|---|---|---|---|
| 1 | CHALLENGE (high) | CHALLENGE (high) | **CHALLENGE** | v1.1 patch round (P1-P10) |
| 2 | CHALLENGE (high) | PASS (high) | **CHALLENGE** | v1.2 patch round (P11-P15 + GR-1/2/3) — this version |
| 3 | ⏳ pending | ⏳ pending | TBD | re-audit on v1.2; expected PASS or 1-issue CHALLENGE |
| 4+ | … | … | … | iterate to PASS/PASS |

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

# Implementation v1.2: src/state/typed_tx.rs

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

/// Reserved for v4.1 MetaTx (Gemini round-2 GR-1 recommendation).
/// Not used in v4 — namespace placeholder so v4.1 can introduce
/// `MetaSigningPayload` without re-rotating sibling domains. Marked
/// `#[allow(dead_code)]` because no v4 consumer references it.
#[allow(dead_code)]
const DOMAIN_AGENT_META_PROPOSAL: &[u8] = b"turingosv4.agent_sig.meta_proposal.v1";

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
///
/// **v1.2 round-2 closure (R2-1)**: `Finalize.claim_id` is `ClaimId` (was `TxId`
/// in v1.1; round-2 caught the missed call site that leaked the old type
/// through `SignalBundle::finalize`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SignalKind {
    Empty,
    Finalize {
        claim_id: ClaimId,
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
    pub fn finalize(claim_id: ClaimId, reward: MicroCoin) -> Self {
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
        // WorkTx (agent-signed)
        let tx_clean = fixture_work_tx();
        let d_clean = tx_clean.to_signing_payload().canonical_digest();
        let mut tx_mut = tx_clean.clone();
        tx_mut.signature = AgentSignature::from_bytes([0xff; 64]);
        let d_mut_sig = tx_mut.to_signing_payload().canonical_digest();
        assert_eq!(d_clean, d_mut_sig, "Work: mutating signature must NOT affect digest");

        // VerifyTx (agent-signed)
        let v_clean = fixture_verify_tx();
        let dv_clean = v_clean.to_signing_payload().canonical_digest();
        let mut v_mut = v_clean.clone();
        v_mut.signature = AgentSignature::from_bytes([0xee; 64]);
        assert_eq!(
            dv_clean,
            v_mut.to_signing_payload().canonical_digest(),
            "Verify: mutating signature must NOT affect digest"
        );

        // ChallengeTx (agent-signed)
        let c_clean = fixture_challenge_tx();
        let dc_clean = c_clean.to_signing_payload().canonical_digest();
        let mut c_mut = c_clean.clone();
        c_mut.signature = AgentSignature::from_bytes([0xdd; 64]);
        assert_eq!(
            dc_clean,
            c_mut.to_signing_payload().canonical_digest(),
            "Challenge: mutating signature must NOT affect digest"
        );

        // FinalizeRewardTx / TaskExpireTx / TerminalSummaryTx (system-signed)
        let f_clean = fixture_finalize_reward_tx();
        let df_clean = f_clean.to_signing_payload().canonical_digest();
        let mut f_mut = f_clean.clone();
        f_mut.system_signature = SystemSignature::from_bytes([0x11; 64]);
        assert_eq!(
            df_clean,
            f_mut.to_signing_payload().canonical_digest(),
            "FinalizeReward: mutating signature must NOT affect digest"
        );
        let t_clean = fixture_task_expire_tx();
        let dt_clean = t_clean.to_signing_payload().canonical_digest();
        let mut t_mut = t_clean.clone();
        t_mut.system_signature = SystemSignature::from_bytes([0x22; 64]);
        assert_eq!(
            dt_clean,
            t_mut.to_signing_payload().canonical_digest(),
            "TaskExpire: mutating signature must NOT affect digest"
        );
        let ts_clean = fixture_terminal_summary_tx();
        let dts_clean = ts_clean.to_signing_payload().canonical_digest();
        let mut ts_mut = ts_clean.clone();
        ts_mut.system_signature = SystemSignature::from_bytes([0x33; 64]);
        assert_eq!(
            dts_clean,
            ts_mut.to_signing_payload().canonical_digest(),
            "TerminalSummary: mutating signature must NOT affect digest"
        );

        // Sanity: mutating a SIGNED field DOES change digest.
        let mut tx_signed_change = tx_clean.clone();
        tx_signed_change.timestamp_logical = 9999;
        let d_signed = tx_signed_change.to_signing_payload().canonical_digest();
        assert_ne!(d_clean, d_signed);
    }

    // ── v1.2 NEW (R2-4 Codex round-2): LOAD-BEARING domain test ─────────────

    /// Hash the SAME body bytes with each of the 6 domain prefixes; assert all
    /// 6 results are pairwise distinct. Without the domain prefix, this test
    /// would FAIL — proving the prefix is load-bearing (the round-1 test
    /// `signing_payload_domains_are_distinct` used different bodies and
    /// would have passed even without domains).
    #[test]
    fn signing_payload_domain_prefix_is_load_bearing() {
        // Identical 64-byte body across all domains; the only thing that varies
        // is which domain prefix gets prepended before SHA-256.
        let body: Vec<u8> = (0..64u8).collect();
        let domains: &[&[u8]] = &[
            DOMAIN_AGENT_WORK,
            DOMAIN_AGENT_VERIFY,
            DOMAIN_AGENT_CHALLENGE,
            DOMAIN_SYSTEM_FINALIZE_REWARD,
            DOMAIN_SYSTEM_TASK_EXPIRE,
            DOMAIN_SYSTEM_TERMINAL_SUMMARY,
        ];
        let digests: Vec<[u8; 32]> = domains
            .iter()
            .map(|d| {
                let mut h = Sha256::new();
                h.update(d);
                h.update(&body);
                h.finalize().into()
            })
            .collect();
        for i in 0..digests.len() {
            for j in (i + 1)..digests.len() {
                assert_ne!(
                    digests[i], digests[j],
                    "domains {} and {} produced identical digests on identical body",
                    String::from_utf8_lossy(domains[i]),
                    String::from_utf8_lossy(domains[j])
                );
            }
        }
    }

    // ── v1.2 NEW (P15 Codex round-2 secondary): BTreeMap permutation ───────

    /// PredicateResultsBundle's `acceptance: BTreeMap<PredicateId, BoolWithProof>`
    /// must encode identically regardless of insertion order (matches the BTreeSet
    /// permutation test for read_set; closes round-2 caveat that BTreeMap
    /// fields weren't covered).
    #[test]
    fn typed_tx_btreemap_permutation_independence() {
        let make_work_tx = |insertion_order: &[(&str, bool)]| -> WorkTx {
            let mut tx = fixture_work_tx();
            tx.predicate_results.acceptance = BTreeMap::new();
            for (k, v) in insertion_order {
                tx.predicate_results.acceptance.insert(
                    PredicateId((*k).into()),
                    BoolWithProof {
                        value: *v,
                        proof_cid: None,
                    },
                );
            }
            tx
        };
        let tx_a = make_work_tx(&[("p_a", true), ("p_b", false), ("p_c", true)]);
        let tx_b = make_work_tx(&[("p_c", true), ("p_a", true), ("p_b", false)]);
        let tx_c = make_work_tx(&[("p_b", false), ("p_c", true), ("p_a", true)]);
        let bytes_a = canonical_encode(&tx_a).expect("encode a");
        let bytes_b = canonical_encode(&tx_b).expect("encode b");
        let bytes_c = canonical_encode(&tx_c).expect("encode c");
        assert_eq!(bytes_a, bytes_b);
        assert_eq!(bytes_a, bytes_c);
    }

    // ── v1.2 NEW (R2-4): signing-payload golden hex ────────────────────────

    fn signing_digest_hex(bytes: &[u8; 32]) -> String {
        hex_lower(bytes)
    }

    /// Lock SHA-256 hex of each signing-payload's canonical_digest. Any
    /// future codec / domain / projection change diffs one of these hex strings.
    /// Locked values captured 2026-04-28.
    #[test]
    fn signing_payload_golden_digests() {
        let tests: &[(&str, [u8; 32], &str)] = &[
            (
                "Work",
                fixture_work_tx().to_signing_payload().canonical_digest(),
                EXPECTED_SIGNING_HEX_WORK,
            ),
            (
                "Verify",
                fixture_verify_tx().to_signing_payload().canonical_digest(),
                EXPECTED_SIGNING_HEX_VERIFY,
            ),
            (
                "Challenge",
                fixture_challenge_tx().to_signing_payload().canonical_digest(),
                EXPECTED_SIGNING_HEX_CHALLENGE,
            ),
            (
                "FinalizeReward",
                fixture_finalize_reward_tx()
                    .to_signing_payload()
                    .canonical_digest(),
                EXPECTED_SIGNING_HEX_FINALIZE_REWARD,
            ),
            (
                "TaskExpire",
                fixture_task_expire_tx().to_signing_payload().canonical_digest(),
                EXPECTED_SIGNING_HEX_TASK_EXPIRE,
            ),
            (
                "TerminalSummary",
                fixture_terminal_summary_tx()
                    .to_signing_payload()
                    .canonical_digest(),
                EXPECTED_SIGNING_HEX_TERMINAL_SUMMARY,
            ),
        ];
        // Collect all mismatches before panicking — useful for capturing fresh
        // hex on first run (otherwise only the first failure prints).
        let mut mismatches: Vec<String> = Vec::new();
        for (name, actual, expected) in tests {
            let actual_hex = signing_digest_hex(actual);
            if &actual_hex != expected {
                mismatches.push(format!("{name}: actual={actual_hex} expected={expected}"));
            }
        }
        assert!(
            mismatches.is_empty(),
            "signing-payload digest mismatches:\n  {}",
            mismatches.join("\n  ")
        );
    }

    const EXPECTED_SIGNING_HEX_WORK: &str =
        "534d3cf26b7419a2741fa4eb2930b37095f982cc09c75ba2ee34396675a3d685";
    const EXPECTED_SIGNING_HEX_VERIFY: &str =
        "7c0f5ff4423bf204d39ff17c5f4d8d65a19861140ed15c59f304b2eda167fb95";
    const EXPECTED_SIGNING_HEX_CHALLENGE: &str =
        "64d190a2576ba0e4a1055a0d98a7763c35f817d914ce9eb2a3a49f614b704aa4";
    const EXPECTED_SIGNING_HEX_FINALIZE_REWARD: &str =
        "74fd6bfb730b9d3e9828e4ebf8c3edb24aabb755813a058583949f08fbf5654b";
    const EXPECTED_SIGNING_HEX_TASK_EXPIRE: &str =
        "d30fcf5fd45e32975e5547e266bcc4ef16353284205009d3feb4189e8b248def";
    const EXPECTED_SIGNING_HEX_TERMINAL_SUMMARY: &str =
        "71143e56cbd0fc3bdc4d8b764af9572564f8d66b2f4062d57d3678d4a311ac12";

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

# Supporting v1.2: src/bottom_white/ledger/system_keypair.rs (P12 target)

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
    /// TRACE_MATRIX FC1-Sig+FC3-Sig (CO1.1.4-pre1 v1.2 closure R2-2): finalize
    /// reward signing-payload digest. Opaque `[u8; 32]` produced by
    /// `state::typed_tx::FinalizeRewardSigningPayload::canonical_digest()`.
    FinalizeRewardSigning([u8; 32]),
    /// TRACE_MATRIX FC1-Sig+FC3-Sig (CO1.1.4-pre1 v1.2 closure R2-2): task
    /// expire signing-payload digest. Opaque `[u8; 32]` produced by
    /// `state::typed_tx::TaskExpireSigningPayload::canonical_digest()`.
    TaskExpireSigning([u8; 32]),
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
        CanonicalMessage::FinalizeRewardSigning(digest) => {
            h.update(b"FinalizeRewardSigning");
            h.update(digest);
        }
        CanonicalMessage::TaskExpireSigning(digest) => {
            h.update(b"TaskExpireSigning");
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

    /// TRACE_MATRIX FC1-Sig+FC3-Sig (CO1.1.4-pre1 v1.2 closure R2-2): sign an
    /// opaque 32-byte digest of a `FinalizeRewardSigningPayload` (computed by
    /// state::typed_tx). Symmetric to `sign_terminal_summary` and
    /// `sign_task_expire`.
    pub(crate) fn sign_finalize_reward(
        keypair: &Ed25519Keypair,
        digest: [u8; 32],
    ) -> Result<SystemSignature, KeypairError> {
        sign_system_message_inner(keypair, &CanonicalMessage::FinalizeRewardSigning(digest))
    }

    /// TRACE_MATRIX FC1-Sig+FC3-Sig (CO1.1.4-pre1 v1.2 closure R2-2): sign an
    /// opaque 32-byte digest of a `TaskExpireSigningPayload` (computed by
    /// state::typed_tx). Symmetric to `sign_terminal_summary` and
    /// `sign_finalize_reward`.
    pub(crate) fn sign_task_expire(
        keypair: &Ed25519Keypair,
        digest: [u8; 32],
    ) -> Result<SystemSignature, KeypairError> {
        sign_system_message_inner(keypair, &CanonicalMessage::TaskExpireSigning(digest))
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

# Round-2 merged verdict (closure check reference)

# CO1.1.4-pre1 Round-2 Dual External Audit — Merged Verdict

**Date**: 2026-04-28
**Target**: spec v1.1 + impl v1.1 + 17 tests joint artifact (commit `e0e4565`)
**Auditors**: Codex (gpt-5-codex; 165,930 tokens) + Gemini 2.5 Pro (114,610 tokens)
**Conservative rule** (memory `feedback_dual_audit_conflict`): VETO > CHALLENGE > PASS.

---

## § 1 Verdicts

| Auditor | Verdict | Conviction |
|---|---|---|
| **Codex** | **CHALLENGE** | High |
| **Gemini** | **PASS** | High |
| **Conservative merged** | **CHALLENGE** | High |

**Convergence pattern**: design quality (Gemini PASS) + implementation discipline (Codex CHALLENGE) — same pattern as CO1.7 R1/R2 per memory `feedback_dual_audit_conflict`. Codex caught 4 concrete patch-mechanical defects the v1.1 patch round missed.

---

## § 2 Codex must-fix items (round-2)

| ID | Item | Codex citation | Severity |
|---|---|---|---|
| **R2-1** | `SignalKind::Finalize` and `SignalBundle::finalize` still use `TxId`, NOT `ClaimId` (P2 missed call site) | typed_tx.rs:826-854 | Direct ABI leak |
| **R2-2** | `FinalizeRewardTx.system_signature` and `TaskExpireTx.system_signature` retained but no `CanonicalMessage::FinalizeRewardSigning` / `::TaskExpireSigning` variants + no emitter fns. Dual-sign rationale (§ 4.2) not executable for 2 of 3 system txs | system_keypair.rs (no variants); typed_tx.rs:266+316 (sigs retained) | Authorized signing path missing |
| **R2-3** | Spec drift: § 0 line 47 still says TerminalSummaryTx lives in system_keypair.rs; § 6 line 210 says "imported from system_keypair"; § 9 D-3 row still present despite P7 claiming removal | spec § 0/§ 6/§ 9 | Self-contradictory spec |
| **R2-4** | Signing-payload tests not load-bearing: `signing_payload_domains_are_distinct` uses different bodies (would pass even if domains were removed); `signing_payload_excludes_signature` only tested for WorkTx; no golden hex for signing-payload digests | typed_tx.rs domain test + signing-payload tests | Test discipline gap |

## § 3 Codex secondary findings (caveats, not must-fix)

- BTreeMap permutation test only covers BTreeSet (read_set); predicate_results + failure_class_histogram BTreeMap fields untested for permutation independence
- Missing dedicated `FinalizeRewardSummaryMismatch` error variant for Q-derived wire-vs-Q discipline rejection (P8 § 4.1) — could be added in CO1.7-impl A4

## § 4 Gemini recommendations (PASS verdict; non-blocking)

- **GR-1**: reserve MetaTx domain prefix in typed_tx.rs (placeholder constant for v4.1 namespace)
- **GR-2**: spec note committing to additive-only TransitionError changes within v4 major version
- **GR-3**: brief domain-string rotation process documented in spec

---

## § 5 v1.2 patch plan (round-2 closure)

| Patch | Maps to | Touches |
|---|---|---|
| **P11**: SignalKind::Finalize { claim_id: ClaimId, ... } + SignalBundle::finalize signature update | R2-1 | typed_tx.rs |
| **P12**: NEW `CanonicalMessage::FinalizeRewardSigning([u8; 32])` + `TaskExpireSigning([u8; 32])` variants + canonical_digest match arms + `transition_emitter::sign_finalize_reward` + `sign_task_expire` emitter fns | R2-2 | system_keypair.rs |
| **P13**: spec drift cleanup — § 0 line 47 says state::typed_tx now; § 6 line 210 update; § 9 D-3 row REMOVED (still present) | R2-3 | spec |
| **P14**: same-body-different-domain load-bearing test (build identical bincode body bytes, hash with each domain prefix, assert all 6 distinct) + lock 6 signing-payload golden hex constants | R2-4 | typed_tx.rs tests |
| **P15** (Codex secondary): BTreeMap permutation independence test using predicate_results | secondary | typed_tx.rs tests |
| **GR-1** (Gemini recommendation): reserve `DOMAIN_AGENT_META: &[u8] = b"turingosv4.agent_sig.meta_proposal.v1"` constant — placeholder; never used in v4 | low-risk recommendation | typed_tx.rs |
| **GR-2** (Gemini recommendation): spec § 7.2 NEW additive-only TransitionError commitment | low-risk recommendation | spec |
| **GR-3** (Gemini recommendation): spec § 7.3 NEW domain-string rotation process | low-risk recommendation | spec |

**Estimated scope**: ~150-250 LoC code + 30-50 LoC spec. ~0.3 day. Round-3 audit cost: ~$8-15.

---

## § 6 Round structure forward

| Round | Codex | Gemini | Conservative | Action |
|---|---|---|---|---|
| 1 | CHALLENGE/high | CHALLENGE/high | CHALLENGE | v1.1 patch round (10 patches; commit `e0e4565`) |
| 2 | **CHALLENGE/high** | **PASS/high** | **CHALLENGE** | v1.2 patch round (5 patches + 3 Gemini recommendations) |
| 3 | ⏳ | ⏳ | TBD | round-3 closure check; expected PASS/PASS |

---

## § 7 Cumulative cost

| Round | Codex tokens | Gemini tokens | Estimated $ |
|---|---|---|---|
| 1 | 199,200 | 113,295 | ~$8-15 |
| 2 | 165,930 | 114,610 | ~$7-13 |
| **CO1.1.4-pre1 r1+r2 total** | **365,130** | **227,905** | **~$15-28** |

Cumulative project audit spend: ~$150-230 / $890 mid-budget (~17-26%).

---

## § 8 Sedimented lessons (this round)

1. **Single-call-site type-update is insufficient**: P2 added ClaimId newtype but missed that SignalBundle::Finalize still used TxId. Sedimented: when changing a tx-payload field type, grep for ALL consumers (incl. SignalBundle / runtime APIs / fixtures), don't just update the struct definition.

2. **Symmetric-API completion**: P3 added `TerminalSummarySigning` variant but did NOT add corresponding `FinalizeRewardSigning` / `TaskExpireSigning` (the other two system-emitted txs). Sedimented: when introducing a typed signing primitive for one variant, confirm symmetric coverage for ALL variants in the same family — partial migration creates execution-blocked dual-sign rationale.

3. **Domain-prefix tests must use IDENTICAL bodies**: a non-collision test using DIFFERENT bodies passes trivially even without domain prefix. Sedimented: load-bearing domain-separation tests must construct identical bincode body bytes, hash with each domain, and assert distinct results — otherwise the test is testing struct-shape uniqueness, not domain prefix.

4. **Spec drift after structural migration**: P3/P7 claimed D-3 row removal but only updated the row content; the row was still in § 9. Sedimented: when claiming "row removed", verify with grep on the spec doc — claim-vs-doc parity has been an audit finding before (CO1.7 round-2 R2-C3 was the same pattern: spec said "C3 CLOSED" while code had no LedgerEntry path).

5. **Codex implementation discipline + Gemini design quality is a stable axis decomposition**: matches CO1.7 round-1/2 + this audit. PASS/PASS only when both axes clean. Project pattern.

— ArchitectAI synthesis, 2026-04-28; Round-2 closure 2026-04-28; v1.2 patch round opens.


---

Now give your INDEPENDENT round-3 narrow-scope audit. Cite line numbers.

codex
I’ll verify against the actual working tree rather than relying on the pasted snippets, then run the narrow grep/test checks and report only closure-relevant findings with line citations.
exec
/bin/bash -lc "pwd && rg --files -g 'AGENTS.md' -g 'CLAUDE.md'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
/home/zephryj/projects/turingosv4
CLAUDE.md

exec
/bin/bash -lc 'git status --short --branch && git rev-parse HEAD' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
## main...origin/main
?? handover/audits/CODEX_CO1_1_4_PRE1_ROUND3_AUDIT_2026-04-28.md
?? handover/audits/GEMINI_CO1_1_4_PRE1_ROUND3_AUDIT_2026-04-28.md
?? handover/audits/run_codex_co1_1_4_pre1_round3_audit.sh
?? handover/audits/run_gemini_co1_1_4_pre1_round3_audit.py
f4649a9582b0408efcf81fce56374796d82b5e54

codex
The repo is already on `f4649a9`; there are untracked audit artifacts only. I’m reading the project instructions next, then I’ll inspect the exact code/spec lines for each closure question.
exec
/bin/bash -lc "sed -n '1,220p' CLAUDE.md" in /home/zephryj/projects/turingosv4
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
/bin/bash -lc 'rg -n "SignalKind|SignalBundle|ClaimId|TerminalSummaryTx|DOMAIN_AGENT_META_PROPOSAL|signing_payload_domain_prefix_is_load_bearing|signing_payload_excludes_signature|signing_payload_golden_digests|typed_tx_btreemap_permutation_independence|EXPECTED_SIGNING_HEX|FinalizeRewardSigning|TaskExpireSigning|sign_finalize_reward|sign_task_expire" src/state/typed_tx.rs src/bottom_white/ledger/system_keypair.rs' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
src/bottom_white/ledger/system_keypair.rs:178:// the typed `TerminalSummaryTx` struct (8-field per STATE § 1.5) now lives in
src/bottom_white/ledger/system_keypair.rs:230:    /// of the 8-field `state::typed_tx::TerminalSummaryTx` is computed in
src/bottom_white/ledger/system_keypair.rs:237:    /// `state::typed_tx::FinalizeRewardSigningPayload::canonical_digest()`.
src/bottom_white/ledger/system_keypair.rs:238:    FinalizeRewardSigning([u8; 32]),
src/bottom_white/ledger/system_keypair.rs:241:    /// `state::typed_tx::TaskExpireSigningPayload::canonical_digest()`.
src/bottom_white/ledger/system_keypair.rs:242:    TaskExpireSigning([u8; 32]),
src/bottom_white/ledger/system_keypair.rs:475:        CanonicalMessage::FinalizeRewardSigning(digest) => {
src/bottom_white/ledger/system_keypair.rs:476:            h.update(b"FinalizeRewardSigning");
src/bottom_white/ledger/system_keypair.rs:479:        CanonicalMessage::TaskExpireSigning(digest) => {
src/bottom_white/ledger/system_keypair.rs:480:            h.update(b"TaskExpireSigning");
src/bottom_white/ledger/system_keypair.rs:570:/// digest produced by `state::typed_tx::TerminalSummaryTx::canonical_digest()`
src/bottom_white/ledger/system_keypair.rs:590:    /// opaque 32-byte digest of a `FinalizeRewardSigningPayload` (computed by
src/bottom_white/ledger/system_keypair.rs:592:    /// `sign_task_expire`.
src/bottom_white/ledger/system_keypair.rs:593:    pub(crate) fn sign_finalize_reward(
src/bottom_white/ledger/system_keypair.rs:597:        sign_system_message_inner(keypair, &CanonicalMessage::FinalizeRewardSigning(digest))
src/bottom_white/ledger/system_keypair.rs:601:    /// opaque 32-byte digest of a `TaskExpireSigningPayload` (computed by
src/bottom_white/ledger/system_keypair.rs:603:    /// `sign_finalize_reward`.
src/bottom_white/ledger/system_keypair.rs:604:    pub(crate) fn sign_task_expire(
src/bottom_white/ledger/system_keypair.rs:608:        sign_system_message_inner(keypair, &CanonicalMessage::TaskExpireSigning(digest))
src/state/typed_tx.rs:52:pub struct ClaimId(pub TxId);
src/state/typed_tx.rs:54:impl ClaimId {
src/state/typed_tx.rs:183:/// TRACE_MATRIX § 1.5 TerminalSummaryTx field 4 + Art. IV halt-reason taxonomy.
src/state/typed_tx.rs:283:/// - **C-3 (Codex Q-B)**: `claim_id` is now a typed `ClaimId` newtype (was
src/state/typed_tx.rs:301:    pub claim_id: ClaimId,                 //  2 — typed (was TxId in v1)
src/state/typed_tx.rs:338:pub struct TerminalSummaryTx {
src/state/typed_tx.rs:388:const DOMAIN_AGENT_META_PROPOSAL: &[u8] = b"turingosv4.agent_sig.meta_proposal.v1";
src/state/typed_tx.rs:456:pub struct FinalizeRewardSigningPayload {
src/state/typed_tx.rs:458:    pub claim_id: ClaimId,
src/state/typed_tx.rs:467:impl FinalizeRewardSigningPayload {
src/state/typed_tx.rs:475:pub struct TaskExpireSigningPayload {
src/state/typed_tx.rs:484:impl TaskExpireSigningPayload {
src/state/typed_tx.rs:490:/// System signing payload for `TerminalSummaryTx` (8 fields → 7 fields).
src/state/typed_tx.rs:554:    pub fn to_signing_payload(&self) -> FinalizeRewardSigningPayload {
src/state/typed_tx.rs:555:        FinalizeRewardSigningPayload {
src/state/typed_tx.rs:569:    pub fn to_signing_payload(&self) -> TaskExpireSigningPayload {
src/state/typed_tx.rs:570:        TaskExpireSigningPayload {
src/state/typed_tx.rs:581:impl TerminalSummaryTx {
src/state/typed_tx.rs:601:/// `TerminalSummaryTx` is imported from `system_keypair.rs` (already shipped).
src/state/typed_tx.rs:610:    TerminalSummary(TerminalSummaryTx),
src/state/typed_tx.rs:676:impl HasSubmitter for TerminalSummaryTx {
src/state/typed_tx.rs:816:// SignalBundle — minimal v1 typed shape (CO1.7.5 + CO1.9 enrich it later)
src/state/typed_tx.rs:825:pub struct SignalBundle {
src/state/typed_tx.rs:826:    pub kind: SignalKind,
src/state/typed_tx.rs:829:/// Discriminator over the spec § 3 pseudocode's `SignalBundle::*` constructors.
src/state/typed_tx.rs:831:/// **v1.2 round-2 closure (R2-1)**: `Finalize.claim_id` is `ClaimId` (was `TxId`
src/state/typed_tx.rs:833:/// through `SignalBundle::finalize`).
src/state/typed_tx.rs:835:pub enum SignalKind {
src/state/typed_tx.rs:838:        claim_id: ClaimId,
src/state/typed_tx.rs:851:impl Default for SignalKind {
src/state/typed_tx.rs:857:impl SignalBundle {
src/state/typed_tx.rs:860:            kind: SignalKind::Empty,
src/state/typed_tx.rs:863:    pub fn finalize(claim_id: ClaimId, reward: MicroCoin) -> Self {
src/state/typed_tx.rs:865:            kind: SignalKind::Finalize { claim_id, reward },
src/state/typed_tx.rs:870:            kind: SignalKind::TaskExpired {
src/state/typed_tx.rs:878:            kind: SignalKind::TerminalSummary { run_id, outcome },
src/state/typed_tx.rs:993:            claim_id: ClaimId::new("claim-001"),
src/state/typed_tx.rs:1016:    fn fixture_terminal_summary_tx() -> TerminalSummaryTx {
src/state/typed_tx.rs:1024:        TerminalSummaryTx {
src/state/typed_tx.rs:1208:            TypedTx::TerminalSummary(TerminalSummaryTx::default()),
src/state/typed_tx.rs:1271:    fn signing_payload_excludes_signature() {
src/state/typed_tx.rs:1302:        // FinalizeRewardTx / TaskExpireTx / TerminalSummaryTx (system-signed)
src/state/typed_tx.rs:1346:    fn signing_payload_domain_prefix_is_load_bearing() {
src/state/typed_tx.rs:1386:    fn typed_tx_btreemap_permutation_independence() {
src/state/typed_tx.rs:1421:    fn signing_payload_golden_digests() {
src/state/typed_tx.rs:1426:                EXPECTED_SIGNING_HEX_WORK,
src/state/typed_tx.rs:1431:                EXPECTED_SIGNING_HEX_VERIFY,
src/state/typed_tx.rs:1436:                EXPECTED_SIGNING_HEX_CHALLENGE,
src/state/typed_tx.rs:1443:                EXPECTED_SIGNING_HEX_FINALIZE_REWARD,
src/state/typed_tx.rs:1448:                EXPECTED_SIGNING_HEX_TASK_EXPIRE,
src/state/typed_tx.rs:1455:                EXPECTED_SIGNING_HEX_TERMINAL_SUMMARY,
src/state/typed_tx.rs:1474:    const EXPECTED_SIGNING_HEX_WORK: &str =
src/state/typed_tx.rs:1476:    const EXPECTED_SIGNING_HEX_VERIFY: &str =
src/state/typed_tx.rs:1478:    const EXPECTED_SIGNING_HEX_CHALLENGE: &str =
src/state/typed_tx.rs:1480:    const EXPECTED_SIGNING_HEX_FINALIZE_REWARD: &str =
src/state/typed_tx.rs:1482:    const EXPECTED_SIGNING_HEX_TASK_EXPIRE: &str =
src/state/typed_tx.rs:1484:    const EXPECTED_SIGNING_HEX_TERMINAL_SUMMARY: &str =

exec
/bin/bash -lc "rg -n \"TerminalSummaryTx|D-3|7\\.2|7\\.3|additive-only|Domain-string|domain rotation|state::typed_tx|system_keypair\" handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
15:| **P12** | `FinalizeRewardTx.system_signature` and `TaskExpireTx.system_signature` retained but no `CanonicalMessage::FinalizeRewardSigning` / `::TaskExpireSigning` variants + no emitter fns. Dual-sign rationale (§ 4.2) not executable for 2 of 3 system txs | NEW `CanonicalMessage::FinalizeRewardSigning([u8; 32])` + `TaskExpireSigning([u8; 32])` variants (system_keypair.rs) + canonical_digest match arms + `transition_emitter::sign_finalize_reward` + `sign_task_expire` symmetric emitter fns | R2-2 (Codex Q-7) |
16:| **P13** | Spec drift: § 0 line 47 still said "TerminalSummaryTx in system_keypair.rs"; § 6 line 210 said "imported from system_keypair"; § 9 D-3 row still present despite v1.1 P7 claiming removal | All 3 stale references cleaned: § 0 lists TerminalSummaryTx in `state::typed_tx`; § 6 inline comment updated; § 9 D-3 row REMOVED (HTML comment marker placed in its slot) | R2-3 (Codex Q-3) |
19:| **GR-1** (Gemini PASS recommendation) | MetaTx domain prefix not reserved; v4.1 namespace might force domain rotation later | NEW `DOMAIN_AGENT_META_PROPOSAL: &[u8] = b"turingosv4.agent_sig.meta_proposal.v1"` constant (typed_tx.rs); marked `#[allow(dead_code)]` until v4.1 wires MetaTx | Gemini Q9 / GR-1 |
20:| **GR-2** (Gemini recommendation) | TransitionError additive-only commitment not stated | spec § 7.2 NEW: TransitionError variants in v4 are additive-only; never reorder; new variants append at the end | Gemini Q9 / GR-2 |
21:| **GR-3** (Gemini recommendation) | Domain-string rotation process not documented | spec § 7.3 NEW: domain rotation = new constant (`*.v2`) added in parallel; old `.v1` retained until all-replay window passes; bumping major version triggers a v2 spec round | Gemini Q9 / GR-3 |
33:| **P3** | `TerminalSummaryTx` was 3-field placeholder living in `system_keypair.rs` (versus STATE § 1.5 8-field schema); locking the wrong shape into ABI | Migrated to `state::typed_tx::TerminalSummaryTx` with full 8-field STATE schema (tx_id / task_id / run_id / run_outcome / total_attempts / failure_class_histogram / last_logical_t / system_signature). `system_keypair` now signs an opaque `TerminalSummarySigning([u8; 32])` digest (same opaque-digest pattern as `LedgerEntrySigning`) — no `bottom_white ↔ state` circular dep. | C-3 (Codex Q-C must-fix-now) |
37:| **P7** | D-3 TerminalSummaryTx field-set divergence | RESOLVED (P3 migrated to full schema). § 9 D-3 row removed. | C-3 followup |
61:3. **Agent signature**: `AgentSignature([u8; 64])` — Ed25519 detached signature, distinct from `SystemSignature` (system_keypair.rs).
66:8. **Typed-tx payload structs** (all in `state::typed_tx`): `WorkTx`, `VerifyTx`, `ChallengeTx`, `ReuseTx`, `FinalizeRewardTx`, `TaskExpireTx`, `TerminalSummaryTx` (8-field STATE § 1.5 schema; v1.1 P3 migrated from a 3-field placeholder previously in `system_keypair.rs`).
112:└── system_keypair.rs            (existing; serde_bytes_64 helper promoted to pub(crate)
116:**Crate boundary**: `state::typed_tx` consumes (a) `state::q_state` types (Hash, AgentId, TxId, NodeId), (b) `economy::money::MicroCoin` + `StakeMicroCoin`, (c) `bottom_white::cas::schema::Cid`, (d) `bottom_white::ledger::system_keypair::{SystemEpoch, SystemSignature}`. No new outward dependencies; no circular dep risk.
153:pub struct AgentSignature(#[serde(with = "system_keypair::serde_bytes_64")] [u8; 64]);
229:    TerminalSummary(TerminalSummaryTx),  // 8-field schema in state::typed_tx (v1.1 P3)
277:### § 7.2 TransitionError additive-only commitment (v1.2 GR-2 per Gemini round-2)
279:`TransitionError` variants in **v4 are additive-only**:
287:### § 7.3 Domain-string rotation process (v1.2 GR-3 per Gemini round-2)
294:4. **Bump the v4 spec minor version** with a "domain rotation" entry in the patch log.
312:// FinalizeRewardTx, TaskExpireTx, TerminalSummaryTx: system-emitted; submitter_id() = None
325:<!-- v1.2 (R2-3 closure): D-3 row removed. Migration is complete; no divergence remains. -->

exec
/bin/bash -lc "rg -n \"Finalize\\.claim_id|claim_id: TxId|claim_id.*TxId|SignalBundle::finalize|ClaimId\" src handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:14:| **P11** | `SignalKind::Finalize` and `SignalBundle::finalize` still used `TxId` (v1.1 P2 missed call site) | `SignalKind::Finalize.claim_id: ClaimId` + `SignalBundle::finalize(claim_id: ClaimId, ...)` | R2-1 (Codex Q-2) |
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:32:| **P2** | `FinalizeRewardTx.claim_id: TxId` reused TxId, leaking ClaimsIndex impl into wire format | New `ClaimId(pub TxId)` newtype with `#[serde(transparent)]` (wire-identical to TxId; non-breaking); `FinalizeRewardTx.claim_id: ClaimId` now | C-3 (Codex Q-B) |
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:179:    pub claim_id: ClaimId,                 //  2  TYPED newtype (v1.1 P2)
src/state/mod.rs:23:    AgentSignature, BoolWithProof, ChallengeSigningPayload, ChallengeTx, ClaimId,
src/state/typed_tx.rs:42:/// in `FinalizeRewardTx.claim_id` and `ClaimsIndex` keying. Wraps `TxId`
src/state/typed_tx.rs:52:pub struct ClaimId(pub TxId);
src/state/typed_tx.rs:54:impl ClaimId {
src/state/typed_tx.rs:283:/// - **C-3 (Codex Q-B)**: `claim_id` is now a typed `ClaimId` newtype (was
src/state/typed_tx.rs:301:    pub claim_id: ClaimId,                 //  2 — typed (was TxId in v1)
src/state/typed_tx.rs:458:    pub claim_id: ClaimId,
src/state/typed_tx.rs:831:/// **v1.2 round-2 closure (R2-1)**: `Finalize.claim_id` is `ClaimId` (was `TxId`
src/state/typed_tx.rs:833:/// through `SignalBundle::finalize`).
src/state/typed_tx.rs:838:        claim_id: ClaimId,
src/state/typed_tx.rs:863:    pub fn finalize(claim_id: ClaimId, reward: MicroCoin) -> Self {
src/state/typed_tx.rs:993:            claim_id: ClaimId::new("claim-001"),

codex
One stale implementation comment already surfaced: `TypedTx::TerminalSummary` still says it is imported from `system_keypair.rs`, despite the spec copy being fixed. I’m checking whether that is the only mechanical residue and whether tests still pass.
exec
/bin/bash -lc "nl -ba src/state/typed_tx.rs | sed -n '36,65p;292,306p;380,392p;452,488p;596,612p;825,866p;1268,1328p;1340,1408p;1418,1486p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
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
   380	const DOMAIN_SYSTEM_TASK_EXPIRE: &[u8] = b"turingosv4.system_sig.task_expire.v1";
   381	const DOMAIN_SYSTEM_TERMINAL_SUMMARY: &[u8] = b"turingosv4.system_sig.terminal_summary.v1";
   382	
   383	/// Reserved for v4.1 MetaTx (Gemini round-2 GR-1 recommendation).
   384	/// Not used in v4 — namespace placeholder so v4.1 can introduce
   385	/// `MetaSigningPayload` without re-rotating sibling domains. Marked
   386	/// `#[allow(dead_code)]` because no v4 consumer references it.
   387	#[allow(dead_code)]
   388	const DOMAIN_AGENT_META_PROPOSAL: &[u8] = b"turingosv4.agent_sig.meta_proposal.v1";
   389	
   390	fn domain_prefixed_digest<T: Serialize>(domain: &[u8], value: &T) -> [u8; 32] {
   391	    use crate::bottom_white::ledger::transition_ledger::canonical_encode;
   392	    let body = canonical_encode(value).expect("canonical_encode of signing payload");
   452	}
   453	
   454	/// System signing payload for `FinalizeRewardTx` (9 fields → 8 fields).
   455	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   456	pub struct FinalizeRewardSigningPayload {
   457	    pub tx_id: TxId,
   458	    pub claim_id: ClaimId,
   459	    pub task_id: TaskId,
   460	    pub solver: AgentId,
   461	    pub reward: MicroCoin,
   462	    pub parent_state_root: Hash,
   463	    pub epoch: SystemEpoch,
   464	    pub timestamp_logical: u64,
   465	}
   466	
   467	impl FinalizeRewardSigningPayload {
   468	    pub fn canonical_digest(&self) -> [u8; 32] {
   469	        domain_prefixed_digest(DOMAIN_SYSTEM_FINALIZE_REWARD, self)
   470	    }
   471	}
   472	
   473	/// System signing payload for `TaskExpireTx` (7 fields → 6 fields).
   474	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   475	pub struct TaskExpireSigningPayload {
   476	    pub tx_id: TxId,
   477	    pub task_id: TaskId,
   478	    pub parent_state_root: Hash,
   479	    pub bounty_refunded: MicroCoin,
   480	    pub epoch: SystemEpoch,
   481	    pub timestamp_logical: u64,
   482	}
   483	
   484	impl TaskExpireSigningPayload {
   485	    pub fn canonical_digest(&self) -> [u8; 32] {
   486	        domain_prefixed_digest(DOMAIN_SYSTEM_TASK_EXPIRE, self)
   487	    }
   488	}
   596	// § 6 TypedTx outer enum
   597	// ────────────────────────────────────────────────────────────────────────────
   598	
   599	/// TRACE_MATRIX § 8 dispatch_transition — typed-tx outer enum.
   600	/// 7 variants (K5 closed: NO `Slash`).
   601	/// `TerminalSummaryTx` is imported from `system_keypair.rs` (already shipped).
   602	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
   603	pub enum TypedTx {
   604	    Work(WorkTx),
   605	    Verify(VerifyTx),
   606	    Challenge(ChallengeTx),
   607	    Reuse(ReuseTx),
   608	    FinalizeReward(FinalizeRewardTx),
   609	    TaskExpire(TaskExpireTx),
   610	    TerminalSummary(TerminalSummaryTx),
   611	}
   612	
   825	pub struct SignalBundle {
   826	    pub kind: SignalKind,
   827	}
   828	
   829	/// Discriminator over the spec § 3 pseudocode's `SignalBundle::*` constructors.
   830	///
   831	/// **v1.2 round-2 closure (R2-1)**: `Finalize.claim_id` is `ClaimId` (was `TxId`
   832	/// in v1.1; round-2 caught the missed call site that leaked the old type
   833	/// through `SignalBundle::finalize`).
   834	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
   835	pub enum SignalKind {
   836	    Empty,
   837	    Finalize {
   838	        claim_id: ClaimId,
   839	        reward: MicroCoin,
   840	    },
   841	    TaskExpired {
   842	        task_id: TaskId,
   843	        bounty_refunded: MicroCoin,
   844	    },
   845	    TerminalSummary {
   846	        run_id: RunId,
   847	        outcome: RunOutcome,
   848	    },
   849	}
   850	
   851	impl Default for SignalKind {
   852	    fn default() -> Self {
   853	        Self::Empty
   854	    }
   855	}
   856	
   857	impl SignalBundle {
   858	    pub fn empty() -> Self {
   859	        Self {
   860	            kind: SignalKind::Empty,
   861	        }
   862	    }
   863	    pub fn finalize(claim_id: ClaimId, reward: MicroCoin) -> Self {
   864	        Self {
   865	            kind: SignalKind::Finalize { claim_id, reward },
   866	        }
  1268	    /// signing-payload digest (the signature is its own input — a canonical
  1269	    /// digest cycle prevention property).
  1270	    #[test]
  1271	    fn signing_payload_excludes_signature() {
  1272	        // WorkTx (agent-signed)
  1273	        let tx_clean = fixture_work_tx();
  1274	        let d_clean = tx_clean.to_signing_payload().canonical_digest();
  1275	        let mut tx_mut = tx_clean.clone();
  1276	        tx_mut.signature = AgentSignature::from_bytes([0xff; 64]);
  1277	        let d_mut_sig = tx_mut.to_signing_payload().canonical_digest();
  1278	        assert_eq!(d_clean, d_mut_sig, "Work: mutating signature must NOT affect digest");
  1279	
  1280	        // VerifyTx (agent-signed)
  1281	        let v_clean = fixture_verify_tx();
  1282	        let dv_clean = v_clean.to_signing_payload().canonical_digest();
  1283	        let mut v_mut = v_clean.clone();
  1284	        v_mut.signature = AgentSignature::from_bytes([0xee; 64]);
  1285	        assert_eq!(
  1286	            dv_clean,
  1287	            v_mut.to_signing_payload().canonical_digest(),
  1288	            "Verify: mutating signature must NOT affect digest"
  1289	        );
  1290	
  1291	        // ChallengeTx (agent-signed)
  1292	        let c_clean = fixture_challenge_tx();
  1293	        let dc_clean = c_clean.to_signing_payload().canonical_digest();
  1294	        let mut c_mut = c_clean.clone();
  1295	        c_mut.signature = AgentSignature::from_bytes([0xdd; 64]);
  1296	        assert_eq!(
  1297	            dc_clean,
  1298	            c_mut.to_signing_payload().canonical_digest(),
  1299	            "Challenge: mutating signature must NOT affect digest"
  1300	        );
  1301	
  1302	        // FinalizeRewardTx / TaskExpireTx / TerminalSummaryTx (system-signed)
  1303	        let f_clean = fixture_finalize_reward_tx();
  1304	        let df_clean = f_clean.to_signing_payload().canonical_digest();
  1305	        let mut f_mut = f_clean.clone();
  1306	        f_mut.system_signature = SystemSignature::from_bytes([0x11; 64]);
  1307	        assert_eq!(
  1308	            df_clean,
  1309	            f_mut.to_signing_payload().canonical_digest(),
  1310	            "FinalizeReward: mutating signature must NOT affect digest"
  1311	        );
  1312	        let t_clean = fixture_task_expire_tx();
  1313	        let dt_clean = t_clean.to_signing_payload().canonical_digest();
  1314	        let mut t_mut = t_clean.clone();
  1315	        t_mut.system_signature = SystemSignature::from_bytes([0x22; 64]);
  1316	        assert_eq!(
  1317	            dt_clean,
  1318	            t_mut.to_signing_payload().canonical_digest(),
  1319	            "TaskExpire: mutating signature must NOT affect digest"
  1320	        );
  1321	        let ts_clean = fixture_terminal_summary_tx();
  1322	        let dts_clean = ts_clean.to_signing_payload().canonical_digest();
  1323	        let mut ts_mut = ts_clean.clone();
  1324	        ts_mut.system_signature = SystemSignature::from_bytes([0x33; 64]);
  1325	        assert_eq!(
  1326	            dts_clean,
  1327	            ts_mut.to_signing_payload().canonical_digest(),
  1328	            "TerminalSummary: mutating signature must NOT affect digest"
  1340	    /// Hash the SAME body bytes with each of the 6 domain prefixes; assert all
  1341	    /// 6 results are pairwise distinct. Without the domain prefix, this test
  1342	    /// would FAIL — proving the prefix is load-bearing (the round-1 test
  1343	    /// `signing_payload_domains_are_distinct` used different bodies and
  1344	    /// would have passed even without domains).
  1345	    #[test]
  1346	    fn signing_payload_domain_prefix_is_load_bearing() {
  1347	        // Identical 64-byte body across all domains; the only thing that varies
  1348	        // is which domain prefix gets prepended before SHA-256.
  1349	        let body: Vec<u8> = (0..64u8).collect();
  1350	        let domains: &[&[u8]] = &[
  1351	            DOMAIN_AGENT_WORK,
  1352	            DOMAIN_AGENT_VERIFY,
  1353	            DOMAIN_AGENT_CHALLENGE,
  1354	            DOMAIN_SYSTEM_FINALIZE_REWARD,
  1355	            DOMAIN_SYSTEM_TASK_EXPIRE,
  1356	            DOMAIN_SYSTEM_TERMINAL_SUMMARY,
  1357	        ];
  1358	        let digests: Vec<[u8; 32]> = domains
  1359	            .iter()
  1360	            .map(|d| {
  1361	                let mut h = Sha256::new();
  1362	                h.update(d);
  1363	                h.update(&body);
  1364	                h.finalize().into()
  1365	            })
  1366	            .collect();
  1367	        for i in 0..digests.len() {
  1368	            for j in (i + 1)..digests.len() {
  1369	                assert_ne!(
  1370	                    digests[i], digests[j],
  1371	                    "domains {} and {} produced identical digests on identical body",
  1372	                    String::from_utf8_lossy(domains[i]),
  1373	                    String::from_utf8_lossy(domains[j])
  1374	                );
  1375	            }
  1376	        }
  1377	    }
  1378	
  1379	    // ── v1.2 NEW (P15 Codex round-2 secondary): BTreeMap permutation ───────
  1380	
  1381	    /// PredicateResultsBundle's `acceptance: BTreeMap<PredicateId, BoolWithProof>`
  1382	    /// must encode identically regardless of insertion order (matches the BTreeSet
  1383	    /// permutation test for read_set; closes round-2 caveat that BTreeMap
  1384	    /// fields weren't covered).
  1385	    #[test]
  1386	    fn typed_tx_btreemap_permutation_independence() {
  1387	        let make_work_tx = |insertion_order: &[(&str, bool)]| -> WorkTx {
  1388	            let mut tx = fixture_work_tx();
  1389	            tx.predicate_results.acceptance = BTreeMap::new();
  1390	            for (k, v) in insertion_order {
  1391	                tx.predicate_results.acceptance.insert(
  1392	                    PredicateId((*k).into()),
  1393	                    BoolWithProof {
  1394	                        value: *v,
  1395	                        proof_cid: None,
  1396	                    },
  1397	                );
  1398	            }
  1399	            tx
  1400	        };
  1401	        let tx_a = make_work_tx(&[("p_a", true), ("p_b", false), ("p_c", true)]);
  1402	        let tx_b = make_work_tx(&[("p_c", true), ("p_a", true), ("p_b", false)]);
  1403	        let tx_c = make_work_tx(&[("p_b", false), ("p_c", true), ("p_a", true)]);
  1404	        let bytes_a = canonical_encode(&tx_a).expect("encode a");
  1405	        let bytes_b = canonical_encode(&tx_b).expect("encode b");
  1406	        let bytes_c = canonical_encode(&tx_c).expect("encode c");
  1407	        assert_eq!(bytes_a, bytes_b);
  1408	        assert_eq!(bytes_a, bytes_c);
  1418	    /// future codec / domain / projection change diffs one of these hex strings.
  1419	    /// Locked values captured 2026-04-28.
  1420	    #[test]
  1421	    fn signing_payload_golden_digests() {
  1422	        let tests: &[(&str, [u8; 32], &str)] = &[
  1423	            (
  1424	                "Work",
  1425	                fixture_work_tx().to_signing_payload().canonical_digest(),
  1426	                EXPECTED_SIGNING_HEX_WORK,
  1427	            ),
  1428	            (
  1429	                "Verify",
  1430	                fixture_verify_tx().to_signing_payload().canonical_digest(),
  1431	                EXPECTED_SIGNING_HEX_VERIFY,
  1432	            ),
  1433	            (
  1434	                "Challenge",
  1435	                fixture_challenge_tx().to_signing_payload().canonical_digest(),
  1436	                EXPECTED_SIGNING_HEX_CHALLENGE,
  1437	            ),
  1438	            (
  1439	                "FinalizeReward",
  1440	                fixture_finalize_reward_tx()
  1441	                    .to_signing_payload()
  1442	                    .canonical_digest(),
  1443	                EXPECTED_SIGNING_HEX_FINALIZE_REWARD,
  1444	            ),
  1445	            (
  1446	                "TaskExpire",
  1447	                fixture_task_expire_tx().to_signing_payload().canonical_digest(),
  1448	                EXPECTED_SIGNING_HEX_TASK_EXPIRE,
  1449	            ),
  1450	            (
  1451	                "TerminalSummary",
  1452	                fixture_terminal_summary_tx()
  1453	                    .to_signing_payload()
  1454	                    .canonical_digest(),
  1455	                EXPECTED_SIGNING_HEX_TERMINAL_SUMMARY,
  1456	            ),
  1457	        ];
  1458	        // Collect all mismatches before panicking — useful for capturing fresh
  1459	        // hex on first run (otherwise only the first failure prints).
  1460	        let mut mismatches: Vec<String> = Vec::new();
  1461	        for (name, actual, expected) in tests {
  1462	            let actual_hex = signing_digest_hex(actual);
  1463	            if &actual_hex != expected {
  1464	                mismatches.push(format!("{name}: actual={actual_hex} expected={expected}"));
  1465	            }
  1466	        }
  1467	        assert!(
  1468	            mismatches.is_empty(),
  1469	            "signing-payload digest mismatches:\n  {}",
  1470	            mismatches.join("\n  ")
  1471	        );
  1472	    }
  1473	
  1474	    const EXPECTED_SIGNING_HEX_WORK: &str =
  1475	        "534d3cf26b7419a2741fa4eb2930b37095f982cc09c75ba2ee34396675a3d685";
  1476	    const EXPECTED_SIGNING_HEX_VERIFY: &str =
  1477	        "7c0f5ff4423bf204d39ff17c5f4d8d65a19861140ed15c59f304b2eda167fb95";
  1478	    const EXPECTED_SIGNING_HEX_CHALLENGE: &str =
  1479	        "64d190a2576ba0e4a1055a0d98a7763c35f817d914ce9eb2a3a49f614b704aa4";
  1480	    const EXPECTED_SIGNING_HEX_FINALIZE_REWARD: &str =
  1481	        "74fd6bfb730b9d3e9828e4ebf8c3edb24aabb755813a058583949f08fbf5654b";
  1482	    const EXPECTED_SIGNING_HEX_TASK_EXPIRE: &str =
  1483	        "d30fcf5fd45e32975e5547e266bcc4ef16353284205009d3feb4189e8b248def";
  1484	    const EXPECTED_SIGNING_HEX_TERMINAL_SUMMARY: &str =
  1485	        "71143e56cbd0fc3bdc4d8b764af9572564f8d66b2f4062d57d3678d4a311ac12";
  1486	

exec
/bin/bash -lc "nl -ba src/bottom_white/ledger/system_keypair.rs | sed -n '222,244p;460,486p;564,610p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
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
   235	    /// TRACE_MATRIX FC1-Sig+FC3-Sig (CO1.1.4-pre1 v1.2 closure R2-2): finalize
   236	    /// reward signing-payload digest. Opaque `[u8; 32]` produced by
   237	    /// `state::typed_tx::FinalizeRewardSigningPayload::canonical_digest()`.
   238	    FinalizeRewardSigning([u8; 32]),
   239	    /// TRACE_MATRIX FC1-Sig+FC3-Sig (CO1.1.4-pre1 v1.2 closure R2-2): task
   240	    /// expire signing-payload digest. Opaque `[u8; 32]` produced by
   241	    /// `state::typed_tx::TaskExpireSigningPayload::canonical_digest()`.
   242	    TaskExpireSigning([u8; 32]),
   243	    /// TRACE_MATRIX FC3-Sig: system key epoch continuity proof.
   244	    EpochRotationProof(EpochRotationProof),
   460	pub fn canonical_digest(message: &CanonicalMessage) -> [u8; 32] {
   461	    let mut h = Sha256::new();
   462	    h.update(b"turingosv4.system_keypair.v1");
   463	    match message {
   464	        CanonicalMessage::RejectedAttemptSummary(summary) => {
   465	            h.update(b"RejectedAttemptSummary");
   466	            update_len_prefixed(&mut h, summary.run_id.as_bytes());
   467	            update_len_prefixed(&mut h, summary.attempt_id.as_bytes());
   468	            update_len_prefixed(&mut h, summary.failure_class.as_bytes());
   469	            h.update(summary.summary_hash);
   470	        }
   471	        CanonicalMessage::TerminalSummarySigning(digest) => {
   472	            h.update(b"TerminalSummarySigning");
   473	            h.update(digest);
   474	        }
   475	        CanonicalMessage::FinalizeRewardSigning(digest) => {
   476	            h.update(b"FinalizeRewardSigning");
   477	            h.update(digest);
   478	        }
   479	        CanonicalMessage::TaskExpireSigning(digest) => {
   480	            h.update(b"TaskExpireSigning");
   481	            h.update(digest);
   482	        }
   483	        CanonicalMessage::EpochRotationProof(proof) => {
   484	            h.update(b"EpochRotationProof");
   485	            h.update(proof.old_epoch.get().to_be_bytes());
   486	            h.update(proof.new_epoch.get().to_be_bytes());
   564	    }
   565	}
   566	
   567	/// TRACE_MATRIX FC1-Sig+FC3-Sig: crate-only signing surface for terminal summary emission.
   568	///
   569	/// **CO1.1.4-pre1 v1.1 round-1 closure (C-3)**: signs an opaque `[u8; 32]`
   570	/// digest produced by `state::typed_tx::TerminalSummaryTx::canonical_digest()`
   571	/// (same opaque-digest pattern as `transition_ledger_emitter::sign_ledger_entry`)
   572	/// rather than the typed struct directly — keeps `system_keypair` oblivious
   573	/// to the typed-tx schema, no `bottom_white ↔ state` circular dep.
   574	pub(crate) mod terminal_summary_emitter {
   575	    use super::{
   576	        sign_system_message_inner, CanonicalMessage, Ed25519Keypair, EpochRotationProof,
   577	        KeypairError, SystemSignature,
   578	    };
   579	
   580	    /// TRACE_MATRIX FC1-Sig+FC3-Sig: sign an opaque 32-byte digest of a
   581	    /// terminal-summary signing payload (computed by typed_tx).
   582	    pub(crate) fn sign_terminal_summary(
   583	        keypair: &Ed25519Keypair,
   584	        digest: [u8; 32],
   585	    ) -> Result<SystemSignature, KeypairError> {
   586	        sign_system_message_inner(keypair, &CanonicalMessage::TerminalSummarySigning(digest))
   587	    }
   588	
   589	    /// TRACE_MATRIX FC1-Sig+FC3-Sig (CO1.1.4-pre1 v1.2 closure R2-2): sign an
   590	    /// opaque 32-byte digest of a `FinalizeRewardSigningPayload` (computed by
   591	    /// state::typed_tx). Symmetric to `sign_terminal_summary` and
   592	    /// `sign_task_expire`.
   593	    pub(crate) fn sign_finalize_reward(
   594	        keypair: &Ed25519Keypair,
   595	        digest: [u8; 32],
   596	    ) -> Result<SystemSignature, KeypairError> {
   597	        sign_system_message_inner(keypair, &CanonicalMessage::FinalizeRewardSigning(digest))
   598	    }
   599	
   600	    /// TRACE_MATRIX FC1-Sig+FC3-Sig (CO1.1.4-pre1 v1.2 closure R2-2): sign an
   601	    /// opaque 32-byte digest of a `TaskExpireSigningPayload` (computed by
   602	    /// state::typed_tx). Symmetric to `sign_terminal_summary` and
   603	    /// `sign_finalize_reward`.
   604	    pub(crate) fn sign_task_expire(
   605	        keypair: &Ed25519Keypair,
   606	        digest: [u8; 32],
   607	    ) -> Result<SystemSignature, KeypairError> {
   608	        sign_system_message_inner(keypair, &CanonicalMessage::TaskExpireSigning(digest))
   609	    }
   610	

exec
/bin/bash -lc "nl -ba handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md | sed -n '58,70p;224,232p;277,296p;320,328p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
    58	
    59	1. **Identifier newtypes**: `TaskId`, `RunId`, `ToolId`, `PredicateId` (each opaque `String`).
    60	2. **Read/Write set keys**: `ReadKey(String)`, `WriteKey(String)`.
    61	3. **Agent signature**: `AgentSignature([u8; 64])` — Ed25519 detached signature, distinct from `SystemSignature` (system_keypair.rs).
    62	4. **Predicate result types**: `BoolWithProof`, `PredicateResultsBundle`, `SafetyOrCreation`.
    63	5. **Status / class enums**: `TxStatus`, `RejectionClass`, `VerifyVerdict`, `RunOutcome`.
    64	6. **Slash evidence reference**: `SlashEvidenceCid(Cid)` newtype.
    65	7. **Money newtype**: `StakeMicroCoin(MicroCoin)` (non-negative invariant enforced at business layer; type-level newtype prevents accidental mix with general `MicroCoin`).
    66	8. **Typed-tx payload structs** (all in `state::typed_tx`): `WorkTx`, `VerifyTx`, `ChallengeTx`, `ReuseTx`, `FinalizeRewardTx`, `TaskExpireTx`, `TerminalSummaryTx` (8-field STATE § 1.5 schema; v1.1 P3 migrated from a 3-field placeholder previously in `system_keypair.rs`).
    67	9. **Outer enum**: `pub enum TypedTx` with the 7 variants.
    68	10. **Trait**: `pub trait HasSubmitter` per STATE spec § 3.6.5 v1.3.
    69	11. **Conformance tests**: 1 golden fixture per main tx kind (input → known SHA-256 of canonical bytes) + 100-input round-trip + cross-call byte stability.
    70	
   224	    Verify(VerifyTx),
   225	    Challenge(ChallengeTx),
   226	    Reuse(ReuseTx),
   227	    FinalizeReward(FinalizeRewardTx),
   228	    TaskExpire(TaskExpireTx),
   229	    TerminalSummary(TerminalSummaryTx),  // 8-field schema in state::typed_tx (v1.1 P3)
   230	}
   231	
   232	impl TypedTx {
   277	### § 7.2 TransitionError additive-only commitment (v1.2 GR-2 per Gemini round-2)
   278	
   279	`TransitionError` variants in **v4 are additive-only**:
   280	- New variants MUST be APPENDED to the existing list (no insertion that would shift downstream variant indices).
   281	- Existing variants MUST NOT be reordered (bincode emits variant-index-as-u32-BE; reordering changes the wire format and invalidates locked golden fixtures).
   282	- Variant removal is NOT permitted within v4; deprecated error classes get a doc-comment "deprecated; replaced by X" but the variant stays.
   283	- Bumping the major version (v4 → v5) is the only path that allows non-additive changes; that triggers an ABI rotation cycle (re-audit + re-fixture + canonical re-encoding migration).
   284	
   285	This rule applies symmetrically to other ABI enums frozen in this atom: `TxStatus` / `RejectionClass` / `VerifyVerdict` / `RunOutcome` / `SafetyOrCreation` / `SignalKind` / `CanonicalMessage` / `TxKind` (transition_ledger.rs).
   286	
   287	### § 7.3 Domain-string rotation process (v1.2 GR-3 per Gemini round-2)
   288	
   289	If a future audit finds a security defect in a domain prefix (`b"turingosv4.<actor>.<purpose>.v1"`), rotation follows this discipline:
   290	
   291	1. **Add NEW constant** (`*.v2`) **in parallel**; do NOT delete the old `*.v1` constant.
   292	2. **Old `*.v1` MUST remain reachable in code** until the runtime can prove no in-flight tx still uses it (replay-window quiescence).
   293	3. **New transitions emit only `*.v2` digests**; the runtime accepts both digests during the rotation window.
   294	4. **Bump the v4 spec minor version** with a "domain rotation" entry in the patch log.
   295	5. **Lock new golden hex** for v2-domain digests; v1-domain digests stay locked too (so historical replay still verifies).
   296	
   320	
   321	| ID | STATE spec | CO1.1.4-pre1 v1.1 | Reason |
   322	|---|---|---|---|
   323	| **D-1** | § 1.2 WorkTx field 12 = `status: TxStatus` | **dropped from wire** (Codex round-1 PASS with patch note) | TxStatus is runner book-keeping, not canonical wire data. STATE § 3 transition fns do NOT read `tx.status` from received tx; status is derived from accepted-tx history + ClaimsIndex. Codex Q-A round-1: PASS. |
   324	| **D-2** | § 3.4 `FinalizeTx::from(claim_id, reward)` opaque constructor | **explicit `FinalizeRewardTx` struct** with Q-derived field discipline (§ 4.1) + dual-sign rationale (§ 4.2) | spec gap; derived schema. |
   325	<!-- v1.2 (R2-3 closure): D-3 row removed. Migration is complete; no divergence remains. -->
   326	| **D-4** (v1.1 NEW per Codex Q-J / CX-3) | QState `task_markets_t` / `escrows_t` / `stakes_t` keyed by `TxId` (q_state.rs:201/161/182) but typed_tx schemas use `TaskId` for the same task references | **NOT retrofit in this atom**. Migration owned by **CO P2.1 (TaskMarket atom)** which will rekey the QState indices to `TaskId`. CO1.1.4-pre1 documents the cross-atom debt; no wire-format consequence (the typed-tx schemas already use `TaskId` correctly per STATE § 1.2). |
   327	
   328	---

exec
/bin/bash -lc "rg -n \"claim_id:\\s*TxId|SignalBundle::finalize\\(["'^)]*TxId|Finalize'"\\s*\\{\\s*claim_id:\\s*TxId|claim_id.*bare "'`TxId`|claim_id.*TxId" src handover/specs -g '"'"'!target'"'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
/bin/bash: line 1: TxId: command not found
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:32:| **P2** | `FinalizeRewardTx.claim_id: TxId` reused TxId, leaking ClaimsIndex impl into wire format | New `ClaimId(pub TxId)` newtype with `#[serde(transparent)]` (wire-identical to TxId; non-breaking); `FinalizeRewardTx.claim_id: ClaimId` now | C-3 (Codex Q-B) |
src/state/typed_tx.rs:42:/// in `FinalizeRewardTx.claim_id` and `ClaimsIndex` keying. Wraps `TxId`
src/state/typed_tx.rs:301:    pub claim_id: ClaimId,                 //  2 — typed (was TxId in v1)
src/state/typed_tx.rs:831:/// **v1.2 round-2 closure (R2-1)**: `Finalize.claim_id` is `ClaimId` (was `TxId`

exec
/bin/bash -lc "rg -n \"TerminalSummaryTx.*system_keypair|system_keypair\\.rs.*TerminalSummaryTx|imported from "'`system_keypair|TerminalSummaryTx` was|TerminalSummaryTx in `system_keypair|TerminalSummaryTx.*lives in `system_keypair" src handover/specs handover/audits -g '"'"'!target'"'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
/bin/bash: line 1: system_keypair: command not found
/bin/bash: line 1: TerminalSummaryTx: command not found
/bin/bash: line 1: system_keypair: command not found
/bin/bash: line 1: TerminalSummaryTx.*lives: command not found
handover/audits/run_codex_co1_1_4_pre1_round3_audit.sh:25:| **P13** | Spec drift: § 0/§ 6/§ 9 still referenced TerminalSummaryTx in system_keypair after P3 migration | § 0 lists state::typed_tx; § 6 inline comment updated; § 9 D-3 row REMOVED (HTML comment marker) | spec § 0 / § 6 / § 9 |
handover/audits/run_codex_co1_1_4_pre1_round3_audit.sh:38:**Q3. P13 closure**: § 0 line 47 fixed? § 6 line 210 fixed? § 9 D-3 row actually removed (HTML comment present, no D-3 row visible)? Any OTHER stale TerminalSummaryTx-in-system_keypair reference that grep would catch?
handover/audits/CODEX_CO1_1_4_PRE1_ROUND2_AUDIT_2026-04-28.md:35:| **P3** (C-3 Codex Q-C) | TerminalSummaryTx 3-field placeholder | Migrated to 8-field STATE § 1.5 schema in state::typed_tx; system_keypair signs opaque [u8;32] via NEW CanonicalMessage::TerminalSummarySigning variant | Full schema present? bottom_white ↔ state circular dep eliminated? Old struct fully removed? Sig API correct? |
handover/audits/CODEX_CO1_1_4_PRE1_ROUND2_AUDIT_2026-04-28.md:59:**Q-3. P3 closure (TerminalSummaryTx migration)**: Old 3-field struct fully removed from system_keypair.rs (not just commented out)? New 8-field struct in typed_tx.rs has correct serde derives? CanonicalMessage::TerminalSummarySigning([u8;32]) variant added correctly + canonical_digest match arm updated? terminal_summary_emitter::sign_terminal_summary takes [u8;32] (not the struct)? bottom_white ↔ state circular dep verifiably absent (grep)?
handover/audits/CODEX_CO1_1_4_PRE1_ROUND2_AUDIT_2026-04-28.md:117:| **P3** | `TerminalSummaryTx` was 3-field placeholder living in `system_keypair.rs` (versus STATE § 1.5 8-field schema); locking the wrong shape into ABI | Migrated to `state::typed_tx::TerminalSummaryTx` with full 8-field STATE schema (tx_id / task_id / run_id / run_outcome / total_attempts / failure_class_histogram / last_logical_t / system_signature). `system_keypair` now signs an opaque `TerminalSummarySigning([u8; 32])` digest (same opaque-digest pattern as `LedgerEntrySigning`) — no `bottom_white ↔ state` circular dep. | C-3 (Codex Q-C must-fix-now) |
handover/audits/CODEX_CO1_1_4_PRE1_ROUND2_AUDIT_2026-04-28.md:150:8. **Typed-tx payload structs**: `WorkTx`, `VerifyTx`, `ChallengeTx`, `ReuseTx`, `FinalizeRewardTx`, `TaskExpireTx`. (`TerminalSummaryTx` already exists in `system_keypair.rs`.)
handover/audits/CODEX_CO1_1_4_PRE1_ROUND2_AUDIT_2026-04-28.md:313:    TerminalSummary(TerminalSummaryTx),  // imported from system_keypair
handover/audits/CODEX_CO1_1_4_PRE1_ROUND2_AUDIT_2026-04-28.md:387:| **D-3** | ~~§ 1.5 `TerminalSummaryTx` 3-field placeholder~~ | **RESOLVED v1.1 P3**: migrated to full 8-field STATE § 1.5 schema in `state::typed_tx`; system_keypair signs opaque `TerminalSummarySigning([u8;32])` digest. |
handover/audits/CODEX_CO1_1_4_PRE1_ROUND2_AUDIT_2026-04-28.md:1025:/// `TerminalSummaryTx` is imported from `system_keypair.rs` (already shipped).
handover/audits/CODEX_CO1_1_4_PRE1_ROUND2_AUDIT_2026-04-28.md:2821:| **P3**: migrate `TerminalSummaryTx` from system_keypair.rs (3-field) → state::typed_tx (8-field per STATE § 1.5: tx_id / task_id / run_id / run_outcome / total_attempts / failure_class_histogram / last_logical_t / system_signature); drop the `system_keypair::TerminalSummaryTx`; system_keypair::sign_terminal_summary_tx accepts opaque digest | C-3 main | typed_tx.rs + system_keypair.rs (deletion + reroute) |
handover/audits/CODEX_CO1_1_4_PRE1_ROUND2_AUDIT_2026-04-28.md:2858:1. **Single-source-of-truth for tx schemas matters**: TerminalSummaryTx living in `system_keypair.rs` (signer module) leaked into a "frozen" location and made the ABI atom imports dependent on the placeholder. Sedimented: per-typed-tx struct should live in `state::typed_tx`; signers consume opaque digests.
handover/audits/CODEX_CO1_1_4_PRE1_ROUND2_AUDIT_2026-04-28.md:5042:src/state/typed_tx.rs:594:/// `TerminalSummaryTx` is imported from `system_keypair.rs` (already shipped).
handover/audits/CODEX_CO1_1_4_PRE1_ROUND2_AUDIT_2026-04-28.md:5054:src/bottom_white/ledger/system_keypair.rs:178:// the typed `TerminalSummaryTx` struct (8-field per STATE § 1.5) now lives in
handover/audits/CODEX_CO1_1_4_PRE1_ROUND2_AUDIT_2026-04-28.md:5055:src/bottom_white/ledger/system_keypair.rs:230:    /// of the 8-field `state::typed_tx::TerminalSummaryTx` is computed in
handover/audits/CODEX_CO1_1_4_PRE1_ROUND2_AUDIT_2026-04-28.md:5056:src/bottom_white/ledger/system_keypair.rs:554:/// digest produced by `state::typed_tx::TerminalSummaryTx::canonical_digest()`
handover/audits/CODEX_CO1_1_4_PRE1_ROUND2_AUDIT_2026-04-28.md:5072:handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:14:| **P3** | `TerminalSummaryTx` was 3-field placeholder living in `system_keypair.rs` (versus STATE § 1.5 8-field schema); locking the wrong shape into ABI | Migrated to `state::typed_tx::TerminalSummaryTx` with full 8-field STATE schema (tx_id / task_id / run_id / run_outcome / total_attempts / failure_class_histogram / last_logical_t / system_signature). `system_keypair` now signs an opaque `TerminalSummarySigning([u8; 32])` digest (same opaque-digest pattern as `LedgerEntrySigning`) — no `bottom_white ↔ state` circular dep. | C-3 (Codex Q-C must-fix-now) |
handover/audits/CODEX_CO1_1_4_PRE1_ROUND2_AUDIT_2026-04-28.md:5077:handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:47:8. **Typed-tx payload structs**: `WorkTx`, `VerifyTx`, `ChallengeTx`, `ReuseTx`, `FinalizeRewardTx`, `TaskExpireTx`. (`TerminalSummaryTx` already exists in `system_keypair.rs`.)
handover/audits/CODEX_CO1_1_4_PRE1_ROUND2_AUDIT_2026-04-28.md:5083:handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:210:    TerminalSummary(TerminalSummaryTx),  // imported from system_keypair
handover/audits/CODEX_CO1_1_4_PRE1_ROUND2_AUDIT_2026-04-28.md:5087:handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:284:| **D-3** | ~~§ 1.5 `TerminalSummaryTx` 3-field placeholder~~ | **RESOLVED v1.1 P3**: migrated to full 8-field STATE § 1.5 schema in `state::typed_tx`; system_keypair signs opaque `TerminalSummarySigning([u8;32])` digest. |
handover/audits/CODEX_CO1_1_4_PRE1_ROUND2_AUDIT_2026-04-28.md:5782:   594	/// `TerminalSummaryTx` is imported from `system_keypair.rs` (already shipped).
handover/audits/CODEX_CO1_1_4_PRE1_ROUND2_AUDIT_2026-04-28.md:7474:    14	| **P3** | `TerminalSummaryTx` was 3-field placeholder living in `system_keypair.rs` (versus STATE § 1.5 8-field schema); locking the wrong shape into ABI | Migrated to `state::typed_tx::TerminalSummaryTx` with full 8-field STATE schema (tx_id / task_id / run_id / run_outcome / total_attempts / failure_class_histogram / last_logical_t / system_signature). `system_keypair` now signs an opaque `TerminalSummarySigning([u8; 32])` digest (same opaque-digest pattern as `LedgerEntrySigning`) — no `bottom_white ↔ state` circular dep. | C-3 (Codex Q-C must-fix-now) |
handover/audits/CODEX_CO1_1_4_PRE1_ROUND2_AUDIT_2026-04-28.md:7507:    47	8. **Typed-tx payload structs**: `WorkTx`, `VerifyTx`, `ChallengeTx`, `ReuseTx`, `FinalizeRewardTx`, `TaskExpireTx`. (`TerminalSummaryTx` already exists in `system_keypair.rs`.)
handover/audits/CODEX_CO1_1_4_PRE1_ROUND2_AUDIT_2026-04-28.md:7679:   284	| **D-3** | ~~§ 1.5 `TerminalSummaryTx` 3-field placeholder~~ | **RESOLVED v1.1 P3**: migrated to full 8-field STATE § 1.5 schema in `state::typed_tx`; system_keypair signs opaque `TerminalSummarySigning([u8;32])` digest. |
handover/audits/CODEX_CO1_1_4_PRE1_ROUND2_AUDIT_2026-04-28.md:7873:The main v1.1 code shape is present. I’ve already found a spec hygiene miss: §9 still contains a D-3 row marked “RESOLVED,” despite the patch log saying it was removed, and §0 scope still says TerminalSummaryTx exists in `system_keypair.rs`. I’m checking whether there are behavioral blockers too.
handover/audits/CODEX_CO1_1_4_PRE1_ROUND2_AUDIT_2026-04-28.md:7907:handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:47:8. **Typed-tx payload structs**: `WorkTx`, `VerifyTx`, `ChallengeTx`, `ReuseTx`, `FinalizeRewardTx`, `TaskExpireTx`. (`TerminalSummaryTx` already exists in `system_keypair.rs`.)
handover/audits/CODEX_CO1_1_4_PRE1_ROUND2_AUDIT_2026-04-28.md:8420:/bin/bash -lc 'rg -n "TerminalSummaryTx" handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md src/state/typed_tx.rs src/bottom_white/ledger/system_keypair.rs' in /home/zephryj/projects/turingosv4
handover/audits/CODEX_CO1_1_4_PRE1_ROUND2_AUDIT_2026-04-28.md:8422:src/bottom_white/ledger/system_keypair.rs:178:// the typed `TerminalSummaryTx` struct (8-field per STATE § 1.5) now lives in
handover/audits/CODEX_CO1_1_4_PRE1_ROUND2_AUDIT_2026-04-28.md:8423:src/bottom_white/ledger/system_keypair.rs:230:    /// of the 8-field `state::typed_tx::TerminalSummaryTx` is computed in
handover/audits/CODEX_CO1_1_4_PRE1_ROUND2_AUDIT_2026-04-28.md:8424:src/bottom_white/ledger/system_keypair.rs:554:/// digest produced by `state::typed_tx::TerminalSummaryTx::canonical_digest()`
handover/audits/CODEX_CO1_1_4_PRE1_ROUND2_AUDIT_2026-04-28.md:8429:src/state/typed_tx.rs:594:/// `TerminalSummaryTx` is imported from `system_keypair.rs` (already shipped).
handover/audits/CODEX_CO1_1_4_PRE1_ROUND2_AUDIT_2026-04-28.md:8435:handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:14:| **P3** | `TerminalSummaryTx` was 3-field placeholder living in `system_keypair.rs` (versus STATE § 1.5 8-field schema); locking the wrong shape into ABI | Migrated to `state::typed_tx::TerminalSummaryTx` with full 8-field STATE schema (tx_id / task_id / run_id / run_outcome / total_attempts / failure_class_histogram / last_logical_t / system_signature). `system_keypair` now signs an opaque `TerminalSummarySigning([u8; 32])` digest (same opaque-digest pattern as `LedgerEntrySigning`) — no `bottom_white ↔ state` circular dep. | C-3 (Codex Q-C must-fix-now) |
handover/audits/CODEX_CO1_1_4_PRE1_ROUND2_AUDIT_2026-04-28.md:8437:handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:47:8. **Typed-tx payload structs**: `WorkTx`, `VerifyTx`, `ChallengeTx`, `ReuseTx`, `FinalizeRewardTx`, `TaskExpireTx`. (`TerminalSummaryTx` already exists in `system_keypair.rs`.)
handover/audits/CODEX_CO1_1_4_PRE1_ROUND2_AUDIT_2026-04-28.md:8438:handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:210:    TerminalSummary(TerminalSummaryTx),  // imported from system_keypair
handover/audits/CODEX_CO1_1_4_PRE1_ROUND2_AUDIT_2026-04-28.md:8440:handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:284:| **D-3** | ~~§ 1.5 `TerminalSummaryTx` 3-field placeholder~~ | **RESOLVED v1.1 P3**: migrated to full 8-field STATE § 1.5 schema in `state::typed_tx`; system_keypair signs opaque `TerminalSummarySigning([u8;32])` digest. |
handover/audits/CODEX_CO1_1_4_PRE1_ROUND2_AUDIT_2026-04-28.md:9038:src/bottom_white/ledger/system_keypair.rs:178:// the typed `TerminalSummaryTx` struct (8-field per STATE § 1.5) now lives in
handover/audits/CODEX_CO1_1_4_PRE1_ROUND2_AUDIT_2026-04-28.md:9040:src/bottom_white/ledger/system_keypair.rs:230:    /// of the 8-field `state::typed_tx::TerminalSummaryTx` is computed in
handover/audits/CODEX_CO1_1_4_PRE1_ROUND2_AUDIT_2026-04-28.md:9041:src/bottom_white/ledger/system_keypair.rs:554:/// digest produced by `state::typed_tx::TerminalSummaryTx::canonical_digest()`
handover/audits/CODEX_CO1_1_4_PRE1_ROUND2_AUDIT_2026-04-28.md:9166:Implementation closure: yes. New 8-field `TerminalSummaryTx` matches STATE §1.5 and is in [typed_tx.rs:337-347](/home/zephryj/projects/turingosv4/src/state/typed_tx.rs:337). `system_keypair` has `CanonicalMessage::TerminalSummarySigning([u8; 32])` at [223-234](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/system_keypair.rs:223), digest arm at [463-466](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/system_keypair.rs:463), and `sign_terminal_summary(..., digest: [u8; 32])` at [566-570](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/system_keypair.rs:566). Grep shows no old struct in `system_keypair.rs`.
handover/audits/CODEX_CO1_1_4_PRE1_ROUND2_AUDIT_2026-04-28.md:9168:Spec/doc hygiene is not closed: CO1.1.4 §0 still says `TerminalSummaryTx` exists in `system_keypair.rs` at spec line 47, §6 says “imported from system_keypair” at line 210, and §9 still contains D-3 at line 284 despite P7 claiming the row was removed.
handover/audits/CODEX_CO1_1_4_PRE1_ROUND2_AUDIT_2026-04-28.md:9212:3. Clean spec drift: remove D-3 from §9 or stop claiming removal; fix stale “TerminalSummaryTx in/imported from system_keypair” references.
handover/audits/CODEX_CO1_1_4_PRE1_ROUND2_AUDIT_2026-04-28.md:9233:Implementation closure: yes. New 8-field `TerminalSummaryTx` matches STATE §1.5 and is in [typed_tx.rs:337-347](/home/zephryj/projects/turingosv4/src/state/typed_tx.rs:337). `system_keypair` has `CanonicalMessage::TerminalSummarySigning([u8; 32])` at [223-234](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/system_keypair.rs:223), digest arm at [463-466](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/system_keypair.rs:463), and `sign_terminal_summary(..., digest: [u8; 32])` at [566-570](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/system_keypair.rs:566). Grep shows no old struct in `system_keypair.rs`.
handover/audits/CODEX_CO1_1_4_PRE1_ROUND2_AUDIT_2026-04-28.md:9235:Spec/doc hygiene is not closed: CO1.1.4 §0 still says `TerminalSummaryTx` exists in `system_keypair.rs` at spec line 47, §6 says “imported from system_keypair” at line 210, and §9 still contains D-3 at line 284 despite P7 claiming the row was removed.
handover/audits/CODEX_CO1_1_4_PRE1_ROUND2_AUDIT_2026-04-28.md:9279:3. Clean spec drift: remove D-3 from §9 or stop claiming removal; fix stale “TerminalSummaryTx in/imported from system_keypair” references.
handover/audits/CLAUDE_AUDITOR_CO1_7_0AF_KEYPAIR_2026-04-27.md:21:| Q8 | TRACE_MATRIX coverage | PASS | All public symbols carry `/// TRACE_MATRIX FC1-Sig` and/or `FC3-Sig` doc-comments — module header, `SystemEpoch`, `SystemPublicKey`, `SystemSignature`, `RejectedAttemptSummary`, `TerminalSummaryTx`, `EpochRotationProof`, `CanonicalMessage` + each variant, `PinnedSystemPubkeys` + methods, `Ed25519Keypair` + methods, all `KeypairError` variants, `default_system_keystore_path`, `generate_or_load_system_keypair`, `load_existing_keypair`, `verify_system_signature`, `verify_epoch_rotation_proof`, `verify_system_pubkeys`, `canonical_digest`. WP § 16 backlink format honored. |
handover/audits/run_codex_co1_1_4_pre1_round2_audit.sh:26:| **P3** (C-3 Codex Q-C) | TerminalSummaryTx 3-field placeholder | Migrated to 8-field STATE § 1.5 schema in state::typed_tx; system_keypair signs opaque [u8;32] via NEW CanonicalMessage::TerminalSummarySigning variant | Full schema present? bottom_white ↔ state circular dep eliminated? Old struct fully removed? Sig API correct? |
handover/audits/run_codex_co1_1_4_pre1_round2_audit.sh:50:**Q-3. P3 closure (TerminalSummaryTx migration)**: Old 3-field struct fully removed from system_keypair.rs (not just commented out)? New 8-field struct in typed_tx.rs has correct serde derives? CanonicalMessage::TerminalSummarySigning([u8;32]) variant added correctly + canonical_digest match arm updated? terminal_summary_emitter::sign_terminal_summary takes [u8;32] (not the struct)? bottom_white ↔ state circular dep verifiably absent (grep)?
handover/audits/run_gemini_co1_1_4_pre1_round2_audit.py:43:| **P3** | TerminalSummaryTx 3-field placeholder | Migrated to 8-field STATE § 1.5 schema in state::typed_tx; system_keypair signs opaque [u8;32] via NEW CanonicalMessage::TerminalSummarySigning variant | Constitutional: is the migration HONEST (full schema present + old removed)? bottom_white ↔ state circular-dep risk eliminated? |
handover/audits/CODEX_CO1_1_4_PRE1_ROUND3_AUDIT_2026-04-28.md:35:| **P13** | Spec drift: § 0/§ 6/§ 9 still referenced TerminalSummaryTx in system_keypair after P3 migration | § 0 lists state::typed_tx; § 6 inline comment updated; § 9 D-3 row REMOVED (HTML comment marker) | spec § 0 / § 6 / § 9 |
handover/audits/CODEX_CO1_1_4_PRE1_ROUND3_AUDIT_2026-04-28.md:48:**Q3. P13 closure**: § 0 line 47 fixed? § 6 line 210 fixed? § 9 D-3 row actually removed (HTML comment present, no D-3 row visible)? Any OTHER stale TerminalSummaryTx-in-system_keypair reference that grep would catch?
handover/audits/CODEX_CO1_1_4_PRE1_ROUND3_AUDIT_2026-04-28.md:96:| **P13** | Spec drift: § 0 line 47 still said "TerminalSummaryTx in system_keypair.rs"; § 6 line 210 said "imported from system_keypair"; § 9 D-3 row still present despite v1.1 P7 claiming removal | All 3 stale references cleaned: § 0 lists TerminalSummaryTx in `state::typed_tx`; § 6 inline comment updated; § 9 D-3 row REMOVED (HTML comment marker placed in its slot) | R2-3 (Codex Q-3) |
handover/audits/CODEX_CO1_1_4_PRE1_ROUND3_AUDIT_2026-04-28.md:113:| **P3** | `TerminalSummaryTx` was 3-field placeholder living in `system_keypair.rs` (versus STATE § 1.5 8-field schema); locking the wrong shape into ABI | Migrated to `state::typed_tx::TerminalSummaryTx` with full 8-field STATE schema (tx_id / task_id / run_id / run_outcome / total_attempts / failure_class_histogram / last_logical_t / system_signature). `system_keypair` now signs an opaque `TerminalSummarySigning([u8; 32])` digest (same opaque-digest pattern as `LedgerEntrySigning`) — no `bottom_white ↔ state` circular dep. | C-3 (Codex Q-C must-fix-now) |
handover/audits/CODEX_CO1_1_4_PRE1_ROUND3_AUDIT_2026-04-28.md:146:8. **Typed-tx payload structs** (all in `state::typed_tx`): `WorkTx`, `VerifyTx`, `ChallengeTx`, `ReuseTx`, `FinalizeRewardTx`, `TaskExpireTx`, `TerminalSummaryTx` (8-field STATE § 1.5 schema; v1.1 P3 migrated from a 3-field placeholder previously in `system_keypair.rs`).
handover/audits/CODEX_CO1_1_4_PRE1_ROUND3_AUDIT_2026-04-28.md:1051:/// `TerminalSummaryTx` is imported from `system_keypair.rs` (already shipped).
handover/audits/CODEX_CO1_1_4_PRE1_ROUND3_AUDIT_2026-04-28.md:3014:| **R2-3** | Spec drift: § 0 line 47 still says TerminalSummaryTx lives in system_keypair.rs; § 6 line 210 says "imported from system_keypair"; § 9 D-3 row still present despite P7 claiming removal | spec § 0/§ 6/§ 9 | Self-contradictory spec |
handover/audits/CODEX_CO1_1_4_PRE1_ROUND3_AUDIT_2026-04-28.md:3173:/bin/bash -lc 'rg -n "SignalKind|SignalBundle|ClaimId|TerminalSummaryTx|DOMAIN_AGENT_META_PROPOSAL|signing_payload_domain_prefix_is_load_bearing|signing_payload_excludes_signature|signing_payload_golden_digests|typed_tx_btreemap_permutation_independence|EXPECTED_SIGNING_HEX|FinalizeRewardSigning|TaskExpireSigning|sign_finalize_reward|sign_task_expire" src/state/typed_tx.rs src/bottom_white/ledger/system_keypair.rs' in /home/zephryj/projects/turingosv4
handover/audits/CODEX_CO1_1_4_PRE1_ROUND3_AUDIT_2026-04-28.md:3175:src/bottom_white/ledger/system_keypair.rs:178:// the typed `TerminalSummaryTx` struct (8-field per STATE § 1.5) now lives in
handover/audits/CODEX_CO1_1_4_PRE1_ROUND3_AUDIT_2026-04-28.md:3176:src/bottom_white/ledger/system_keypair.rs:230:    /// of the 8-field `state::typed_tx::TerminalSummaryTx` is computed in
handover/audits/CODEX_CO1_1_4_PRE1_ROUND3_AUDIT_2026-04-28.md:3185:src/bottom_white/ledger/system_keypair.rs:570:/// digest produced by `state::typed_tx::TerminalSummaryTx::canonical_digest()`
handover/audits/CODEX_CO1_1_4_PRE1_ROUND3_AUDIT_2026-04-28.md:3212:src/state/typed_tx.rs:601:/// `TerminalSummaryTx` is imported from `system_keypair.rs` (already shipped).
handover/audits/CODEX_CO1_1_4_PRE1_ROUND3_AUDIT_2026-04-28.md:3253:/bin/bash -lc "rg -n \"TerminalSummaryTx|D-3|7\\.2|7\\.3|additive-only|Domain-string|domain rotation|state::typed_tx|system_keypair\" handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md" in /home/zephryj/projects/turingosv4
handover/audits/CODEX_CO1_1_4_PRE1_ROUND3_AUDIT_2026-04-28.md:3256:16:| **P13** | Spec drift: § 0 line 47 still said "TerminalSummaryTx in system_keypair.rs"; § 6 line 210 said "imported from system_keypair"; § 9 D-3 row still present despite v1.1 P7 claiming removal | All 3 stale references cleaned: § 0 lists TerminalSummaryTx in `state::typed_tx`; § 6 inline comment updated; § 9 D-3 row REMOVED (HTML comment marker placed in its slot) | R2-3 (Codex Q-3) |
handover/audits/CODEX_CO1_1_4_PRE1_ROUND3_AUDIT_2026-04-28.md:3260:33:| **P3** | `TerminalSummaryTx` was 3-field placeholder living in `system_keypair.rs` (versus STATE § 1.5 8-field schema); locking the wrong shape into ABI | Migrated to `state::typed_tx::TerminalSummaryTx` with full 8-field STATE schema (tx_id / task_id / run_id / run_outcome / total_attempts / failure_class_histogram / last_logical_t / system_signature). `system_keypair` now signs an opaque `TerminalSummarySigning([u8; 32])` digest (same opaque-digest pattern as `LedgerEntrySigning`) — no `bottom_white ↔ state` circular dep. | C-3 (Codex Q-C must-fix-now) |
handover/audits/CODEX_CO1_1_4_PRE1_ROUND3_AUDIT_2026-04-28.md:3263:66:8. **Typed-tx payload structs** (all in `state::typed_tx`): `WorkTx`, `VerifyTx`, `ChallengeTx`, `ReuseTx`, `FinalizeRewardTx`, `TaskExpireTx`, `TerminalSummaryTx` (8-field STATE § 1.5 schema; v1.1 P3 migrated from a 3-field placeholder previously in `system_keypair.rs`).
handover/audits/CODEX_CO1_1_4_PRE1_ROUND3_AUDIT_2026-04-28.md:3399:   601	/// `TerminalSummaryTx` is imported from `system_keypair.rs` (already shipped).
handover/audits/CODEX_CO1_1_4_PRE1_ROUND3_AUDIT_2026-04-28.md:3765:    66	8. **Typed-tx payload structs** (all in `state::typed_tx`): `WorkTx`, `VerifyTx`, `ChallengeTx`, `ReuseTx`, `FinalizeRewardTx`, `TaskExpireTx`, `TerminalSummaryTx` (8-field STATE § 1.5 schema; v1.1 P3 migrated from a 3-field placeholder previously in `system_keypair.rs`).
handover/audits/GEMINI_CO1_1_4_PRE1_ROUND2_AUDIT_2026-04-28.md:18:PASS. The migration of `TerminalSummaryTx` from `bottom_white/ledger/system_keypair.rs` to `state/typed_tx.rs` **improves** constitutional alignment with the Anti-Oreo three-layer architecture.
handover/audits/run_codex_co1_1_4_pre1_round1_audit.sh:29:   - `src/bottom_white/ledger/system_keypair.rs` (+Default on SystemEpoch + SystemSignature; serde_bytes_64 → pub(crate); +Serialize/Deserialize/Default on TerminalSummaryTx)
handover/audits/run_codex_co1_1_4_pre1_round1_audit.sh:52:**Q-C. D-3 (TerminalSummaryTx field-set divergence)**: shipped `system_keypair.rs::TerminalSummaryTx` has 3 fields (run_id / terminal_state_root / rejected_attempt_count); STATE spec § 1.5 defines an 8-field schema (tx_id / task_id / run_id / run_outcome / total_attempts / failure_class_histogram / last_logical_t / system_signature). CO1.1.4-pre1 imports the 3-field version unchanged.
handover/audits/run_codex_co1_1_4_pre1_round1_audit.sh:54:- Module-placement question: should TerminalSummaryTx move from `system_keypair.rs` to `state/typed_tx.rs` so all TypedTx variants live in one module?
handover/audits/run_codex_co1_1_4_pre1_round1_audit.sh:124:printf '\n```\n\n---\n\n# Supporting: src/bottom_white/ledger/system_keypair.rs (+Default + serde_bytes_64 pub(crate) + TerminalSummaryTx serde derives)\n\n```rust\n' >> "$TMP_PROMPT"
handover/audits/CO1_1_4_PRE1_DUAL_AUDIT_VERDICT_R1_2026-04-28.md:65:| **P3**: migrate `TerminalSummaryTx` from system_keypair.rs (3-field) → state::typed_tx (8-field per STATE § 1.5: tx_id / task_id / run_id / run_outcome / total_attempts / failure_class_histogram / last_logical_t / system_signature); drop the `system_keypair::TerminalSummaryTx`; system_keypair::sign_terminal_summary_tx accepts opaque digest | C-3 main | typed_tx.rs + system_keypair.rs (deletion + reroute) |
handover/audits/CO1_1_4_PRE1_DUAL_AUDIT_VERDICT_R1_2026-04-28.md:102:1. **Single-source-of-truth for tx schemas matters**: TerminalSummaryTx living in `system_keypair.rs` (signer module) leaked into a "frozen" location and made the ABI atom imports dependent on the placeholder. Sedimented: per-typed-tx struct should live in `state::typed_tx`; signers consume opaque digests.
handover/audits/CODEX_CO1_1_4_PRE1_ROUND1_AUDIT_2026-04-28.md:36:   - `src/bottom_white/ledger/system_keypair.rs` (+Default on SystemEpoch + SystemSignature; serde_bytes_64 → pub(crate); +Serialize/Deserialize/Default on TerminalSummaryTx)
handover/audits/CODEX_CO1_1_4_PRE1_ROUND1_AUDIT_2026-04-28.md:59:**Q-C. D-3 (TerminalSummaryTx field-set divergence)**: shipped `system_keypair.rs::TerminalSummaryTx` has 3 fields (run_id / terminal_state_root / rejected_attempt_count); STATE spec § 1.5 defines an 8-field schema (tx_id / task_id / run_id / run_outcome / total_attempts / failure_class_histogram / last_logical_t / system_signature). CO1.1.4-pre1 imports the 3-field version unchanged.
handover/audits/CODEX_CO1_1_4_PRE1_ROUND1_AUDIT_2026-04-28.md:61:- Module-placement question: should TerminalSummaryTx move from `system_keypair.rs` to `state/typed_tx.rs` so all TypedTx variants live in one module?
handover/audits/CODEX_CO1_1_4_PRE1_ROUND1_AUDIT_2026-04-28.md:143:8. **Typed-tx payload structs**: `WorkTx`, `VerifyTx`, `ChallengeTx`, `ReuseTx`, `FinalizeRewardTx`, `TaskExpireTx`. (`TerminalSummaryTx` already exists in `system_keypair.rs`.)
handover/audits/CODEX_CO1_1_4_PRE1_ROUND1_AUDIT_2026-04-28.md:278:    TerminalSummary(TerminalSummaryTx),  // imported from system_keypair
handover/audits/CODEX_CO1_1_4_PRE1_ROUND1_AUDIT_2026-04-28.md:337:| **D-3** | § 1.5 `TerminalSummaryTx` | **NOT redefined** here | already shipped in `system_keypair.rs`; CO1.1.4-pre1 imports + reuses; module placement migration (move to typed_tx.rs?) deferred to v1.1 if auditors flag. |
handover/audits/CODEX_CO1_1_4_PRE1_ROUND1_AUDIT_2026-04-28.md:677:/// `TerminalSummaryTx` is imported from `system_keypair.rs` (already shipped).
handover/audits/CODEX_CO1_1_4_PRE1_ROUND1_AUDIT_2026-04-28.md:1716:# Supporting: src/bottom_white/ledger/system_keypair.rs (+Default + serde_bytes_64 pub(crate) + TerminalSummaryTx serde derives)
handover/audits/CODEX_CO1_1_4_PRE1_ROUND1_AUDIT_2026-04-28.md:5509:handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:24:8. **Typed-tx payload structs**: `WorkTx`, `VerifyTx`, `ChallengeTx`, `ReuseTx`, `FinalizeRewardTx`, `TaskExpireTx`. (`TerminalSummaryTx` already exists in `system_keypair.rs`.)
handover/audits/CODEX_CO1_1_4_PRE1_ROUND1_AUDIT_2026-04-28.md:5526:handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:159:    TerminalSummary(TerminalSummaryTx),  // imported from system_keypair
handover/audits/CODEX_CO1_1_4_PRE1_ROUND1_AUDIT_2026-04-28.md:5539:handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:218:| **D-3** | § 1.5 `TerminalSummaryTx` | **NOT redefined** here | already shipped in `system_keypair.rs`; CO1.1.4-pre1 imports + reuses; module placement migration (move to typed_tx.rs?) deferred to v1.1 if auditors flag. |
handover/audits/CODEX_CO1_1_4_PRE1_ROUND1_AUDIT_2026-04-28.md:5644:/bin/bash -lc 'rg -n "''^(pub struct|pub enum|impl|pub trait|fn |    fn|#'"\\[test\\])|TxStatus|FinalizeRewardTx|TerminalSummaryTx|SignalBundle|TransitionError|HasSubmitter|golden_|serde_bytes_64|bincode_canonical_config|canonical_encode|canonical_decode|StakeMicroCoin|MicroCoin|pub struct Cid|RunOutcome|RejectionClass|TypedTx|AgentSignature|SystemSignature|CanonicalMessage|canonical_digest\" src/state/typed_tx.rs src/economy/money.rs src/bottom_white/cas/schema.rs src/bottom_white/ledger/system_keypair.rs src/bottom_white/ledger/transition_ledger.rs src/state/mod.rs" in /home/zephryj/projects/turingosv4
handover/audits/CODEX_CO1_1_4_PRE1_ROUND1_AUDIT_2026-04-28.md:5666:src/bottom_white/ledger/system_keypair.rs:186:pub struct TerminalSummaryTx {
handover/audits/CODEX_CO1_1_4_PRE1_ROUND1_AUDIT_2026-04-28.md:5667:src/bottom_white/ledger/system_keypair.rs:192:impl TerminalSummaryTx {
handover/audits/CODEX_CO1_1_4_PRE1_ROUND1_AUDIT_2026-04-28.md:5671:src/bottom_white/ledger/system_keypair.rs:252:    TerminalSummaryTx(TerminalSummaryTx),
handover/audits/CODEX_CO1_1_4_PRE1_ROUND1_AUDIT_2026-04-28.md:5692:src/bottom_white/ledger/system_keypair.rs:481:        CanonicalMessage::TerminalSummaryTx(tx) => {
handover/audits/CODEX_CO1_1_4_PRE1_ROUND1_AUDIT_2026-04-28.md:5693:src/bottom_white/ledger/system_keypair.rs:482:            h.update(b"TerminalSummaryTx");
handover/audits/CODEX_CO1_1_4_PRE1_ROUND1_AUDIT_2026-04-28.md:5709:src/bottom_white/ledger/system_keypair.rs:575:        KeypairError, SystemSignature, TerminalSummaryTx,
handover/audits/CODEX_CO1_1_4_PRE1_ROUND1_AUDIT_2026-04-28.md:5710:src/bottom_white/ledger/system_keypair.rs:581:        tx: &TerminalSummaryTx,
handover/audits/CODEX_CO1_1_4_PRE1_ROUND1_AUDIT_2026-04-28.md:5712:src/bottom_white/ledger/system_keypair.rs:583:        sign_system_message_inner(keypair, &CanonicalMessage::TerminalSummaryTx(tx.clone()))
handover/audits/CODEX_CO1_1_4_PRE1_ROUND1_AUDIT_2026-04-28.md:5990:src/state/typed_tx.rs:298:/// `TerminalSummaryTx` is imported from `system_keypair.rs` (already shipped).
handover/audits/CODEX_CO1_1_4_PRE1_ROUND1_AUDIT_2026-04-28.md:6411:   298	/// `TerminalSummaryTx` is imported from `system_keypair.rs` (already shipped).
handover/audits/CODEX_CO1_1_4_PRE1_ROUND1_AUDIT_2026-04-28.md:7115:    24	8. **Typed-tx payload structs**: `WorkTx`, `VerifyTx`, `ChallengeTx`, `ReuseTx`, `FinalizeRewardTx`, `TaskExpireTx`. (`TerminalSummaryTx` already exists in `system_keypair.rs`.)
handover/audits/CODEX_CO1_1_4_PRE1_ROUND1_AUDIT_2026-04-28.md:7250:   159	    TerminalSummary(TerminalSummaryTx),  // imported from system_keypair
handover/audits/CODEX_CO1_1_4_PRE1_ROUND1_AUDIT_2026-04-28.md:7309:   218	| **D-3** | § 1.5 `TerminalSummaryTx` | **NOT redefined** here | already shipped in `system_keypair.rs`; CO1.1.4-pre1 imports + reuses; module placement migration (move to typed_tx.rs?) deferred to v1.1 if auditors flag. |
handover/audits/CO1_1_4_PRE1_DUAL_AUDIT_VERDICT_R2_2026-04-28.md:28:| **R2-3** | Spec drift: § 0 line 47 still says TerminalSummaryTx lives in system_keypair.rs; § 6 line 210 says "imported from system_keypair"; § 9 D-3 row still present despite P7 claiming removal | spec § 0/§ 6/§ 9 | Self-contradictory spec |
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:16:| **P13** | Spec drift: § 0 line 47 still said "TerminalSummaryTx in system_keypair.rs"; § 6 line 210 said "imported from system_keypair"; § 9 D-3 row still present despite v1.1 P7 claiming removal | All 3 stale references cleaned: § 0 lists TerminalSummaryTx in `state::typed_tx`; § 6 inline comment updated; § 9 D-3 row REMOVED (HTML comment marker placed in its slot) | R2-3 (Codex Q-3) |
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:33:| **P3** | `TerminalSummaryTx` was 3-field placeholder living in `system_keypair.rs` (versus STATE § 1.5 8-field schema); locking the wrong shape into ABI | Migrated to `state::typed_tx::TerminalSummaryTx` with full 8-field STATE schema (tx_id / task_id / run_id / run_outcome / total_attempts / failure_class_histogram / last_logical_t / system_signature). `system_keypair` now signs an opaque `TerminalSummarySigning([u8; 32])` digest (same opaque-digest pattern as `LedgerEntrySigning`) — no `bottom_white ↔ state` circular dep. | C-3 (Codex Q-C must-fix-now) |
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:66:8. **Typed-tx payload structs** (all in `state::typed_tx`): `WorkTx`, `VerifyTx`, `ChallengeTx`, `ReuseTx`, `FinalizeRewardTx`, `TaskExpireTx`, `TerminalSummaryTx` (8-field STATE § 1.5 schema; v1.1 P3 migrated from a 3-field placeholder previously in `system_keypair.rs`).
src/state/typed_tx.rs:601:/// `TerminalSummaryTx` is imported from `system_keypair.rs` (already shipped).

exec
/bin/bash -lc "rg -n \"CanonicalMessage::|verify_system_signature|sign_finalize_reward|sign_task_expire|FinalizeRewardSigning|TaskExpireSigning\" src -g '"'!target'"'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
src/state/mod.rs:24:    FinalizeRewardSigningPayload, FinalizeRewardTx, HasSubmitter, PredicateId,
src/state/mod.rs:26:    SafetyOrCreation, SignalBundle, SignalKind, SlashEvidenceCid, TaskExpireSigningPayload,
src/bottom_white/ledger/system_keypair.rs:180:// `CanonicalMessage::TerminalSummarySigning([u8; 32])` variant — same
src/bottom_white/ledger/system_keypair.rs:237:    /// `state::typed_tx::FinalizeRewardSigningPayload::canonical_digest()`.
src/bottom_white/ledger/system_keypair.rs:238:    FinalizeRewardSigning([u8; 32]),
src/bottom_white/ledger/system_keypair.rs:241:    /// `state::typed_tx::TaskExpireSigningPayload::canonical_digest()`.
src/bottom_white/ledger/system_keypair.rs:242:    TaskExpireSigning([u8; 32]),
src/bottom_white/ledger/system_keypair.rs:464:        CanonicalMessage::RejectedAttemptSummary(summary) => {
src/bottom_white/ledger/system_keypair.rs:471:        CanonicalMessage::TerminalSummarySigning(digest) => {
src/bottom_white/ledger/system_keypair.rs:475:        CanonicalMessage::FinalizeRewardSigning(digest) => {
src/bottom_white/ledger/system_keypair.rs:476:            h.update(b"FinalizeRewardSigning");
src/bottom_white/ledger/system_keypair.rs:479:        CanonicalMessage::TaskExpireSigning(digest) => {
src/bottom_white/ledger/system_keypair.rs:480:            h.update(b"TaskExpireSigning");
src/bottom_white/ledger/system_keypair.rs:483:        CanonicalMessage::EpochRotationProof(proof) => {
src/bottom_white/ledger/system_keypair.rs:491:        CanonicalMessage::LedgerEntrySigning(digest) => {
src/bottom_white/ledger/system_keypair.rs:500:pub fn verify_system_signature(
src/bottom_white/ledger/system_keypair.rs:525:    let message = CanonicalMessage::EpochRotationProof(proof.clone());
src/bottom_white/ledger/system_keypair.rs:526:    verify_system_signature(old_signature, &message, proof.old_epoch(), pinned_pubkeys)
src/bottom_white/ledger/system_keypair.rs:527:        && verify_system_signature(new_signature, &message, proof.new_epoch(), pinned_pubkeys)
src/bottom_white/ledger/system_keypair.rs:554:            &CanonicalMessage::RejectedAttemptSummary(summary.clone()),
src/bottom_white/ledger/system_keypair.rs:586:        sign_system_message_inner(keypair, &CanonicalMessage::TerminalSummarySigning(digest))
src/bottom_white/ledger/system_keypair.rs:590:    /// opaque 32-byte digest of a `FinalizeRewardSigningPayload` (computed by
src/bottom_white/ledger/system_keypair.rs:592:    /// `sign_task_expire`.
src/bottom_white/ledger/system_keypair.rs:593:    pub(crate) fn sign_finalize_reward(
src/bottom_white/ledger/system_keypair.rs:597:        sign_system_message_inner(keypair, &CanonicalMessage::FinalizeRewardSigning(digest))
src/bottom_white/ledger/system_keypair.rs:601:    /// opaque 32-byte digest of a `TaskExpireSigningPayload` (computed by
src/bottom_white/ledger/system_keypair.rs:603:    /// `sign_finalize_reward`.
src/bottom_white/ledger/system_keypair.rs:604:    pub(crate) fn sign_task_expire(
src/bottom_white/ledger/system_keypair.rs:608:        sign_system_message_inner(keypair, &CanonicalMessage::TaskExpireSigning(digest))
src/bottom_white/ledger/system_keypair.rs:618:            &CanonicalMessage::EpochRotationProof(proof.clone()),
src/bottom_white/ledger/system_keypair.rs:651:            &CanonicalMessage::LedgerEntrySigning(signing_payload_digest),
src/bottom_white/ledger/system_keypair.rs:936:        assert!(verify_system_signature(
src/bottom_white/ledger/system_keypair.rs:938:            &CanonicalMessage::RejectedAttemptSummary(summary),
src/bottom_white/ledger/transition_ledger.rs:26://!   ride a `CanonicalMessage::LedgerEntrySigning(_)` variant when CO1.7.5+ extends
src/bottom_white/ledger/transition_ledger.rs:791:    // Verifies: (a) typed sign API works; (b) signature verifies via CanonicalMessage::LedgerEntrySigning;
src/bottom_white/ledger/transition_ledger.rs:797:            SystemEpoch, verify_system_signature,
src/bottom_white/ledger/transition_ledger.rs:824:        let msg_clean = CanonicalMessage::LedgerEntrySigning(digest.0);
src/bottom_white/ledger/transition_ledger.rs:826:            verify_system_signature(&sig, &msg_clean, epoch, &pinned),
src/bottom_white/ledger/transition_ledger.rs:834:        let msg_tamper = CanonicalMessage::LedgerEntrySigning(digest_tamper.0);
src/bottom_white/ledger/transition_ledger.rs:836:            !verify_system_signature(&sig, &msg_tamper, epoch, &pinned),
src/bottom_white/ledger/transition_ledger.rs:849:        let msg_other_epoch = CanonicalMessage::LedgerEntrySigning(digest_other_epoch.0);
src/bottom_white/ledger/transition_ledger.rs:851:            !verify_system_signature(&sig, &msg_other_epoch, epoch, &pinned),
src/state/typed_tx.rs:456:pub struct FinalizeRewardSigningPayload {
src/state/typed_tx.rs:467:impl FinalizeRewardSigningPayload {
src/state/typed_tx.rs:475:pub struct TaskExpireSigningPayload {
src/state/typed_tx.rs:484:impl TaskExpireSigningPayload {
src/state/typed_tx.rs:554:    pub fn to_signing_payload(&self) -> FinalizeRewardSigningPayload {
src/state/typed_tx.rs:555:        FinalizeRewardSigningPayload {
src/state/typed_tx.rs:569:    pub fn to_signing_payload(&self) -> TaskExpireSigningPayload {
src/state/typed_tx.rs:570:        TaskExpireSigningPayload {

exec
/bin/bash -lc 'cargo test --lib' in /home/zephryj/projects/turingosv4
 succeeded in 1479ms:
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
   --> src/bottom_white/ledger/system_keypair.rs:559:19
    |
559 |     pub(crate) fn sign_system_message(
    |                   ^^^^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: function `sign_terminal_summary` is never used
   --> src/bottom_white/ledger/system_keypair.rs:582:19
    |
582 |     pub(crate) fn sign_terminal_summary(
    |                   ^^^^^^^^^^^^^^^^^^^^^

warning: function `sign_finalize_reward` is never used
   --> src/bottom_white/ledger/system_keypair.rs:593:19
    |
593 |     pub(crate) fn sign_finalize_reward(
    |                   ^^^^^^^^^^^^^^^^^^^^

warning: function `sign_task_expire` is never used
   --> src/bottom_white/ledger/system_keypair.rs:604:19
    |
604 |     pub(crate) fn sign_task_expire(
    |                   ^^^^^^^^^^^^^^^^

warning: function `sign_system_message` is never used
   --> src/bottom_white/ledger/system_keypair.rs:623:19
    |
623 |     pub(crate) fn sign_system_message(
    |                   ^^^^^^^^^^^^^^^^^^^

warning: `turingosv4` (lib test) generated 16 warnings (run `cargo fix --lib -p turingosv4 --tests` to apply 11 suggestions)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.22s
     Running unittests src/lib.rs (target/debug/deps/turingosv4-337b37021c2192ab)

running 224 tests
test boot::tests::parse_errors_when_section_missing ... ok
test boot::tests::parse_strips_inline_comment_and_blanks ... ok
test boot::tests::parse_errors_on_unquoted_key ... ok
test boot::tests::verify_trust_root_detects_child_manifest_tamper ... ok
test boot::tests::verify_trust_root_detects_tamper_in_tempdir ... ok
test bottom_white::cas::schema::tests::cid_deterministic ... ok
test boot::tests::verify_trust_root_passes_with_matching_child_manifest ... ok
test bottom_white::cas::schema::tests::cid_display_format ... ok
test boot::tests::verify_trust_root_passes_when_hash_matches_in_tempdir ... ok
test bottom_white::cas::schema::tests::metadata_canonical_hash_deterministic ... ok
test bottom_white::cas::schema::tests::cid_differs_on_content ... ok
test bottom_white::cas::schema::tests::cid_from_empty_content ... ok
test bottom_white::cas::schema::tests::metadata_canonical_hash_differs_on_object_type ... ok
test bottom_white::cas::store::tests::cell_isolation_disjoint_cas ... ok
test bottom_white::cas::store::tests::cid_is_content_address ... ok
test bottom_white::cas::store::tests::empty_store_root ... ok
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
test bottom_white::ledger::transition_ledger::tests::git2_writer_append_and_read_back ... ok
test bottom_white::ledger::transition_ledger::tests::git2_writer_empty_chain ... ok
test bottom_white::cas::store::tests::put_many_then_iterate_count ... ok
test bottom_white::ledger::system_keypair::tests::terminal_scope_rotation_signing_round_trip ... ok
test bottom_white::ledger::transition_ledger::tests::in_memory_writer_enforces_logical_t ... ok
test bottom_white::ledger::transition_ledger::tests::replay_chain_integrity_clean ... ok
test bottom_white::ledger::transition_ledger::tests::git2_writer_rejects_logical_t_gap ... ok
test bottom_white::ledger::transition_ledger::tests::replay_rejects_ledger_root_tamper ... ok
test bottom_white::ledger::transition_ledger::tests::replay_rejects_parent_ledger_tamper ... ok
test bottom_white::ledger::transition_ledger::tests::replay_rejects_parent_state_tamper ... ok
test bottom_white::tools::registry::tests::duplicate_id_rejected ... ok
test bottom_white::tools::registry::tests::empty_id_rejected ... ok
test bottom_white::tools::registry::tests::empty_registry ... ok
test bottom_white::tools::registry::tests::find_by_capability_replaces_magic_string ... ok
test bottom_white::tools::registry::tests::merkle_root_deterministic ... ok
test bottom_white::tools::registry::tests::non_idempotent_rejected ... ok
test bottom_white::tools::registry::tests::register_and_get_round_trip ... ok
test bus::tests::test_bus_basic_append ... ok
test bus::tests::test_bus_classify_bounded ... ok
test bus::tests::test_bus_creates_market_on_append ... ok
test bottom_white::ledger::transition_ledger::tests::git2_writer_reopen_recovers_chain ... ok
test bus::tests::test_bus_forbidden_pattern_veto ... ok
test bus::tests::test_bus_graveyard_feedback ... ok
test bus::tests::test_bus_ledger_integrity ... ok
test bus::tests::test_bus_halt_and_settle ... ok
test bus::tests::test_bus_payload_too_long ... ok
test bus::tests::test_bus_snapshot ... ok
test bus::tests::test_bus_too_many_lines ... ok
test bus::tests::test_bus_unknown_agent_vetoed ... ok
test drivers::llm_http::tests::test_client_creation ... ok
test bus::tests::test_bus_serial_ordering ... ok
test drivers::llm_http::tests::test_driver_error_display ... ok
test economy::money::tests::checked_add_normal ... ok
test economy::money::tests::checked_add_overflow_returns_none ... ok
test drivers::llm_http::tests::test_generate_request_serialization ... ok
test economy::money::tests::conservation_law_basic ... ok
test economy::money::tests::display_positive ... ok
test economy::money::tests::checked_sub_normal ... ok
test economy::money::tests::from_coin_overflow_returns_none ... ok
test economy::money::tests::display_zero ... ok
test economy::money::tests::from_coin_round_trip ... ok
test economy::money::tests::from_micro_units_zero ... ok
test economy::money::tests::ordering_for_btreemap ... ok
test economy::money::tests::royalty_10_percent_rounds_down ... ok
test economy::money::tests::royalty_floor_dust ... ok
test economy::money::tests::royalty_rejects_negative ... ok
test economy::money::tests::serde_round_trip_json ... ok
test economy::money::tests::royalty_rejects_weight_above_1 ... ok
test kernel::tests::test_append_and_retrieve ... ok
test economy::money::tests::serde_transparent_format ... ok
test kernel::tests::test_golden_path_trace ... ok
test kernel::tests::test_market_lifecycle ... ok
test kernel::tests::test_market_ticker ... ok
test kernel::tests::test_no_market_for_nonexistent_node ... ok
test kernel::tests::test_no_duplicate_market ... ok
test kernel::tests::test_reject_duplicate ... ok
test kernel::tests::test_reject_dangling_citation ... ok
test kernel::tests::test_resolve_all_markets ... ok
test ledger::tests::test_ledger_hash_chain_integrity ... ok
test ledger::tests::test_ledger_append_and_verify ... ok
test ledger::tests::test_ledger_omega_vocabulary ... ok
test ledger::tests::test_ledger_sequence_monotonic ... ok
test ledger::tests::test_tape_append_root_node ... ok
test ledger::tests::test_ledger_tamper_detection ... ok
test ledger::tests::test_tape_append_with_valid_citation ... ok
test ledger::tests::test_tape_dag_branching ... ok
test ledger::tests::test_tape_reject_dangling_citation ... ok
test ledger::tests::test_tape_empty ... ok
test ledger::tests::test_tape_reject_duplicate_id ... ok
test ledger::tests::test_tape_time_arrow_ordering ... ok
test prediction_market::tests::test_assassin_profit ... ok
test prediction_market::tests::test_buy_no_increases_no_price ... ok
test ledger::tests::test_tape_trace_ancestors ... ok
test prediction_market::tests::test_buy_yes_increases_yes_price ... ok
test prediction_market::tests::test_constant_product_invariant ... ok
test prediction_market::tests::test_create_market ... ok
test prediction_market::tests::test_ctf_conservation_1_coin_1_yes_1_no ... ok
test prediction_market::tests::test_initial_price_is_50_50 ... ok
test prediction_market::tests::test_no_double_resolution ... ok
test prediction_market::tests::test_multiple_traders_price_discovery ... ok
test prediction_market::tests::test_no_trading_after_resolution ... ok
test prediction_market::tests::test_pioneer_profit ... ok
test prediction_market::tests::test_prices_sum_to_one ... ok
test prediction_market::tests::test_redeem_requires_resolution ... ok
test prediction_market::tests::test_reject_zero_or_negative_amounts ... ok
test sdk::actor::tests::test_boltzmann_never_returns_none_with_nodes ... ok
test sdk::actor::tests::test_boltzmann_returns_none_empty_tape ... ok
test sdk::actor::tests::test_frontier_detection_leaf ... ok
test sdk::actor::tests::test_frontier_detection_parent_with_child ... ok
test sdk::actor::tests::test_lineage_score_increases_with_depth ... ok
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
test sdk::prompt::tests::test_prompt_surfaces_team_board ... ok
test sdk::prompt::tests::test_prompt_truncates_errors_to_3 ... ok
test sdk::prompt_guard::tests::test_case_insensitive_match - should panic ... ok
test sdk::prompt_guard::tests::test_clean_prompt_passes ... ok
test sdk::prompt_guard::tests::test_empty_prompt_passes ... ok
test sdk::prompt_guard::tests::test_h_vpput_caught - should panic ... ok
test sdk::prompt_guard::tests::test_pput_assignment_pattern_caught - should panic ... ok
test sdk::prompt_guard::tests::test_pput_m_verified_caught - should panic ... ok
test sdk::prompt_guard::tests::test_pput_runtime_caught - should panic ... ok
test sdk::prompt_guard::tests::test_pput_substring_in_larger_text - should panic ... ok
test sdk::prompt_guard::tests::test_pput_verified_caught - should panic ... ok
test sdk::prompt_guard::tests::test_wbcg_caught - should panic ... ok
test sdk::protocol::tests::test_deduct_negative_amount_rejected ... ok
test sdk::protocol::tests::test_malformed_action_tag_rejected_not_fallback ... ok
test sdk::protocol::tests::test_no_byte_repair_on_invalid_escape ... ok
test sdk::actor::tests::test_boltzmann_diversity_not_deterministic ... ok
test sdk::protocol::tests::test_parse_action_tag_valid ... ok
test sdk::protocol::tests::test_parse_action_tag_with_think_block ... ok
test sdk::protocol::tests::test_parse_bare_json_fallback ... ok
test sdk::protocol::tests::test_parse_no_action_returns_error ... ok
test sdk::protocol::tests::test_parse_invalid_json_returns_error ... ok
test sdk::protocol::tests::test_strip_multiple_think_blocks ... ok
test sdk::protocol::tests::test_parse_with_invest_action ... ok
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
test sdk::sandbox::tests::test_sandbox_nonzero_exit ... ok
test state::q_state::tests::btreemap_insertion_order_independent_serialization ... ok
test state::q_state::tests::genesis_q_state_is_total_and_default ... ok
test state::q_state::tests::economic_state_has_nine_sub_fields ... ok
test state::q_state::tests::node_id_from_state_root_is_deterministic ... ok
test state::q_state::tests::nine_field_count_via_serde_json ... ok
test state::typed_tx::tests::golden_challenge_tx_digest ... ok
test state::typed_tx::tests::golden_finalize_reward_tx_digest ... ok
test state::typed_tx::tests::golden_reuse_tx_digest ... ok
test state::typed_tx::tests::golden_task_expire_tx_digest ... ok
test state::typed_tx::tests::golden_verify_tx_digest ... ok
test state::typed_tx::tests::golden_terminal_summary_tx_digest ... ok
test state::typed_tx::tests::has_submitter_partitioning ... ok
test state::typed_tx::tests::golden_work_tx_digest ... ok
test state::typed_tx::tests::signing_payload_domain_prefix_is_load_bearing ... ok
test state::typed_tx::tests::signing_payload_domains_are_distinct ... ok
test state::typed_tx::tests::signing_payload_golden_digests ... ok
test state::typed_tx::tests::typed_tx_btree_permutation_independence ... ok
test state::typed_tx::tests::signing_payload_excludes_signature ... ok
test state::typed_tx::tests::typed_tx_btreemap_permutation_independence ... ok
test state::typed_tx::tests::typed_tx_byte_stability_across_calls ... ok
test state::typed_tx::tests::typed_tx_cross_variant_non_collision ... ok
test state::typed_tx::tests::typed_tx_default_round_trip ... ok
test state::typed_tx::tests::typed_tx_kind_projection ... ok
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
test wal::tests::test_wal_roundtrip_mixed ... ok
test wal::tests::test_wal_skip_malformed_line ... ok
test wal::tests::test_wal_roundtrip_nodes_only ... ok
test boot::tests::verify_trust_root_passes_on_intact_repo ... ok
test sdk::sandbox::tests::test_sandbox_timeout_kills_process ... ok

test result: ok. 224 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.20s


exec
/bin/bash -lc "rg -n 'pub struct TerminalSummaryTx|CanonicalMessage::TerminalSummaryTx|sign_terminal_summary_tx|TerminalSummaryTx\\(' src/bottom_white/ledger/system_keypair.rs src/state/typed_tx.rs" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
src/state/typed_tx.rs:338:pub struct TerminalSummaryTx {

exec
/bin/bash -lc "rg -n 'claim_id: TxId|Finalize \\{[[:space:]]*claim_id: TxId|fn finalize\\(["'^'"\\n]*TxId|ClaimId::new|claim_id: ClaimId' src/state/typed_tx.rs src/state/mod.rs handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:14:| **P11** | `SignalKind::Finalize` and `SignalBundle::finalize` still used `TxId` (v1.1 P2 missed call site) | `SignalKind::Finalize.claim_id: ClaimId` + `SignalBundle::finalize(claim_id: ClaimId, ...)` | R2-1 (Codex Q-2) |
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:32:| **P2** | `FinalizeRewardTx.claim_id: TxId` reused TxId, leaking ClaimsIndex impl into wire format | New `ClaimId(pub TxId)` newtype with `#[serde(transparent)]` (wire-identical to TxId; non-breaking); `FinalizeRewardTx.claim_id: ClaimId` now | C-3 (Codex Q-B) |
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:179:    pub claim_id: ClaimId,                 //  2  TYPED newtype (v1.1 P2)
src/state/typed_tx.rs:301:    pub claim_id: ClaimId,                 //  2 — typed (was TxId in v1)
src/state/typed_tx.rs:458:    pub claim_id: ClaimId,
src/state/typed_tx.rs:838:        claim_id: ClaimId,
src/state/typed_tx.rs:863:    pub fn finalize(claim_id: ClaimId, reward: MicroCoin) -> Self {
src/state/typed_tx.rs:993:            claim_id: ClaimId::new("claim-001"),

exec
/bin/bash -lc "nl -ba src/state/mod.rs | sed -n '20,30p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
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

exec
/bin/bash -lc "rg -n 'TerminalSummaryTx::canonical_digest|imported from "'`system_keypair'"\\.rs"'`|TerminalSummaryTx.*system_keypair'"\\.rs' src handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:16:| **P13** | Spec drift: § 0 line 47 still said "TerminalSummaryTx in system_keypair.rs"; § 6 line 210 said "imported from system_keypair"; § 9 D-3 row still present despite v1.1 P7 claiming removal | All 3 stale references cleaned: § 0 lists TerminalSummaryTx in `state::typed_tx`; § 6 inline comment updated; § 9 D-3 row REMOVED (HTML comment marker placed in its slot) | R2-3 (Codex Q-3) |
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:33:| **P3** | `TerminalSummaryTx` was 3-field placeholder living in `system_keypair.rs` (versus STATE § 1.5 8-field schema); locking the wrong shape into ABI | Migrated to `state::typed_tx::TerminalSummaryTx` with full 8-field STATE schema (tx_id / task_id / run_id / run_outcome / total_attempts / failure_class_histogram / last_logical_t / system_signature). `system_keypair` now signs an opaque `TerminalSummarySigning([u8; 32])` digest (same opaque-digest pattern as `LedgerEntrySigning`) — no `bottom_white ↔ state` circular dep. | C-3 (Codex Q-C must-fix-now) |
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:66:8. **Typed-tx payload structs** (all in `state::typed_tx`): `WorkTx`, `VerifyTx`, `ChallengeTx`, `ReuseTx`, `FinalizeRewardTx`, `TaskExpireTx`, `TerminalSummaryTx` (8-field STATE § 1.5 schema; v1.1 P3 migrated from a 3-field placeholder previously in `system_keypair.rs`).
src/state/typed_tx.rs:601:/// `TerminalSummaryTx` is imported from `system_keypair.rs` (already shipped).
src/bottom_white/ledger/system_keypair.rs:570:/// digest produced by `state::typed_tx::TerminalSummaryTx::canonical_digest()`

exec
/bin/bash -lc "nl -ba src/state/typed_tx.rs | sed -n '581,594p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   581	impl TerminalSummaryTx {
   582	    pub fn to_signing_payload(&self) -> TerminalSummarySigningPayload {
   583	        TerminalSummarySigningPayload {
   584	            tx_id: self.tx_id.clone(),
   585	            task_id: self.task_id.clone(),
   586	            run_id: self.run_id.clone(),
   587	            run_outcome: self.run_outcome,
   588	            total_attempts: self.total_attempts,
   589	            failure_class_histogram: self.failure_class_histogram.clone(),
   590	            last_logical_t: self.last_logical_t,
   591	        }
   592	    }
   593	}
   594	

exec
/bin/bash -lc "nl -ba src/state/typed_tx.rs | sed -n '490,504p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   490	/// System signing payload for `TerminalSummaryTx` (8 fields → 7 fields).
   491	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   492	pub struct TerminalSummarySigningPayload {
   493	    pub tx_id: TxId,
   494	    pub task_id: TaskId,
   495	    pub run_id: RunId,
   496	    pub run_outcome: RunOutcome,
   497	    pub total_attempts: u32,
   498	    pub failure_class_histogram: BTreeMap<RejectionClass, u32>,
   499	    pub last_logical_t: u64,
   500	}
   501	
   502	impl TerminalSummarySigningPayload {
   503	    pub fn canonical_digest(&self) -> [u8; 32] {
   504	        domain_prefixed_digest(DOMAIN_SYSTEM_TERMINAL_SUMMARY, self)

exec
/bin/bash -lc "nl -ba src/bottom_white/ledger/system_keypair.rs | sed -n '500,516p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   500	pub fn verify_system_signature(
   501	    sig: &SystemSignature,
   502	    message: &CanonicalMessage,
   503	    epoch: SystemEpoch,
   504	    pinned_pubkeys: &PinnedSystemPubkeys,
   505	) -> bool {
   506	    let Some(pk) = pinned_pubkeys.get(epoch) else {
   507	        return false;
   508	    };
   509	    let Ok(verifying_key) = VerifyingKey::from_bytes(pk.as_bytes()) else {
   510	        return false;
   511	    };
   512	    let signature = Signature::from_bytes(sig.as_bytes());
   513	    verifying_key
   514	        .verify(&canonical_digest(message), &signature)
   515	        .is_ok()
   516	}

codex
# Codex CO1.1.4-pre1 Round-3 Audit

## Q1 P11 closure
PASS. `SignalKind::Finalize.claim_id` is `ClaimId` at `src/state/typed_tx.rs:837-839`; `SignalBundle::finalize` takes `ClaimId` at `src/state/typed_tx.rs:863`. `FinalizeRewardTx.claim_id` and signing payload also use `ClaimId` at `src/state/typed_tx.rs:301` and `src/state/typed_tx.rs:458`. No live `claim_id: TxId` code found.

## Q2 P12 closure
PASS. `CanonicalMessage::{FinalizeRewardSigning, TaskExpireSigning}` present at `src/bottom_white/ledger/system_keypair.rs:235-242`; digest arms present at `src/bottom_white/ledger/system_keypair.rs:475-481`; emitter fns present and symmetric at `src/bottom_white/ledger/system_keypair.rs:593-608`. Verify side is generic over `CanonicalMessage` at `src/bottom_white/ledger/system_keypair.rs:500-515`, so no per-variant verifier needed. Caveat: new emitter fns compile but are unused, warned as dead code.

## Q3 P13 closure
CHALLENGE. Spec targets are fixed: §0 lists all typed tx structs in `state::typed_tx` at `handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:66`; §6 says `state::typed_tx` at line `229`; §9 has the D-3 removal marker and no visible D-3 row at line `325`.

But grep still catches stale implementation doc: `src/state/typed_tx.rs:601` says ``TerminalSummaryTx` is imported from `system_keypair.rs``. Also `src/bottom_white/ledger/system_keypair.rs:570` says `TerminalSummaryTx::canonical_digest()`, but the actual path is `to_signing_payload()` at `src/state/typed_tx.rs:581-593` plus `TerminalSummarySigningPayload::canonical_digest()` at `src/state/typed_tx.rs:502-504`.

## Q4 P14 closure
PASS. Load-bearing test uses one identical 64-byte body at `src/state/typed_tx.rs:1346-1364` across six domains at `1350-1357`. Signature-exclusion test covers Work, Verify, Challenge, FinalizeReward, TaskExpire, TerminalSummary at `src/state/typed_tx.rs:1271-1328`. Golden signing digests cover all six at `src/state/typed_tx.rs:1421-1455`, with six locked constants at `1474-1485`.

## Q5 P15 closure
PASS. `typed_tx_btreemap_permutation_independence` mutates `PredicateResultsBundle.acceptance` at `src/state/typed_tx.rs:1386-1399`; three insertion orders are encoded and byte-compared at `1401-1408`.

## Q6 GR-1/2/3 closures
PASS. `DOMAIN_AGENT_META_PROPOSAL` exists at `src/state/typed_tx.rs:383-388`. Spec §7.2 additive-only ABI enum rule is present at `handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:277-285`; §7.3 domain rotation process is present at `287-295`.

## Q7 New defects
`cargo test --lib` passes: 224 passed, 0 failed, 0 ignored. New/remaining defect is doc hygiene only: stale `TerminalSummaryTx` placement/method comments at `src/state/typed_tx.rs:601` and `src/bottom_white/ledger/system_keypair.rs:570`.

## Q8 PASS gate
No. Behavioral/code closure is effectively clean, but the stated round-3 gate asks whether grep catches any stale `TerminalSummaryTx-in-system_keypair` residue; it does.

## **VERDICT**: CHALLENGE

## Top must-fix
1. Fix `src/state/typed_tx.rs:601` to say `TerminalSummaryTx` is defined in `state::typed_tx`.
2. Fix `src/bottom_white/ledger/system_keypair.rs:570` to reference `TerminalSummarySigningPayload::canonical_digest()` or `TerminalSummaryTx::to_signing_payload().canonical_digest()`.

## Conviction
High.
2026-04-28T12:08:21.926714Z ERROR codex_core::session: failed to record rollout items: thread 019dd3fb-56f6-7941-8463-f04bfce6a019 not found
tokens used
91,030
# Codex CO1.1.4-pre1 Round-3 Audit

## Q1 P11 closure
PASS. `SignalKind::Finalize.claim_id` is `ClaimId` at `src/state/typed_tx.rs:837-839`; `SignalBundle::finalize` takes `ClaimId` at `src/state/typed_tx.rs:863`. `FinalizeRewardTx.claim_id` and signing payload also use `ClaimId` at `src/state/typed_tx.rs:301` and `src/state/typed_tx.rs:458`. No live `claim_id: TxId` code found.

## Q2 P12 closure
PASS. `CanonicalMessage::{FinalizeRewardSigning, TaskExpireSigning}` present at `src/bottom_white/ledger/system_keypair.rs:235-242`; digest arms present at `src/bottom_white/ledger/system_keypair.rs:475-481`; emitter fns present and symmetric at `src/bottom_white/ledger/system_keypair.rs:593-608`. Verify side is generic over `CanonicalMessage` at `src/bottom_white/ledger/system_keypair.rs:500-515`, so no per-variant verifier needed. Caveat: new emitter fns compile but are unused, warned as dead code.

## Q3 P13 closure
CHALLENGE. Spec targets are fixed: §0 lists all typed tx structs in `state::typed_tx` at `handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:66`; §6 says `state::typed_tx` at line `229`; §9 has the D-3 removal marker and no visible D-3 row at line `325`.

But grep still catches stale implementation doc: `src/state/typed_tx.rs:601` says ``TerminalSummaryTx` is imported from `system_keypair.rs``. Also `src/bottom_white/ledger/system_keypair.rs:570` says `TerminalSummaryTx::canonical_digest()`, but the actual path is `to_signing_payload()` at `src/state/typed_tx.rs:581-593` plus `TerminalSummarySigningPayload::canonical_digest()` at `src/state/typed_tx.rs:502-504`.

## Q4 P14 closure
PASS. Load-bearing test uses one identical 64-byte body at `src/state/typed_tx.rs:1346-1364` across six domains at `1350-1357`. Signature-exclusion test covers Work, Verify, Challenge, FinalizeReward, TaskExpire, TerminalSummary at `src/state/typed_tx.rs:1271-1328`. Golden signing digests cover all six at `src/state/typed_tx.rs:1421-1455`, with six locked constants at `1474-1485`.

## Q5 P15 closure
PASS. `typed_tx_btreemap_permutation_independence` mutates `PredicateResultsBundle.acceptance` at `src/state/typed_tx.rs:1386-1399`; three insertion orders are encoded and byte-compared at `1401-1408`.

## Q6 GR-1/2/3 closures
PASS. `DOMAIN_AGENT_META_PROPOSAL` exists at `src/state/typed_tx.rs:383-388`. Spec §7.2 additive-only ABI enum rule is present at `handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md:277-285`; §7.3 domain rotation process is present at `287-295`.

## Q7 New defects
`cargo test --lib` passes: 224 passed, 0 failed, 0 ignored. New/remaining defect is doc hygiene only: stale `TerminalSummaryTx` placement/method comments at `src/state/typed_tx.rs:601` and `src/bottom_white/ledger/system_keypair.rs:570`.

## Q8 PASS gate
No. Behavioral/code closure is effectively clean, but the stated round-3 gate asks whether grep catches any stale `TerminalSummaryTx-in-system_keypair` residue; it does.

## **VERDICT**: CHALLENGE

## Top must-fix
1. Fix `src/state/typed_tx.rs:601` to say `TerminalSummaryTx` is defined in `state::typed_tx`.
2. Fix `src/bottom_white/ledger/system_keypair.rs:570` to reference `TerminalSummarySigningPayload::canonical_digest()` or `TerminalSummaryTx::to_signing_payload().canonical_digest()`.

## Conviction
High.
