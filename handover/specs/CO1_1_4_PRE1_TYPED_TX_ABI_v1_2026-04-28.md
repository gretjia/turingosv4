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
