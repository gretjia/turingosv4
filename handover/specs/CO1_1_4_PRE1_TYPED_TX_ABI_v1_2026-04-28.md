# CO1.1.4-pre1 — Typed Tx ABI Surface (v1)

**Status**: v1 DRAFT, post-CO1.7 PASS/PASS gate (2026-04-28).
**Author**: ArchitectAI (Claude); session 2026-04-28 (continued).
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
    pub claim_id: TxId,                    //  2  identifies the ClaimsIndex entry being finalized
    pub task_id: TaskId,                   //  3
    pub solver: AgentId,                   //  4  reward recipient
    pub reward: MicroCoin,                 //  5  computed by SettlementEngine
    pub parent_state_root: Hash,           //  6  must equal q.state_root_t at submission
    pub epoch: SystemEpoch,                //  7  which keypair signed
    pub timestamp_logical: u64,            //  8  monotonic
    pub system_signature: SystemSignature, //  9  system-emitted, not agent-signed
}
```

**Audit input**: this is the spec gap most likely to attract a CHALLENGE. Auditors should verify the field set is sufficient for `finalize_reward_transition` § 3.4 stage 3 (unlock + return solver stake + credit reward + finalize claim + debit escrow + pay royalties along `royalty_graph_t`). If a field is missing (e.g. royalty edges to walk), this atom should add it before proceeding to A2.

**Honest acknowledgement**: this is the only schema in CO1.1.4-pre1 not directly transcribed from STATE_TRANSITION_SPEC § 1.

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

- I-CANON-A: `canonical_encode(typed_tx)` returns deterministic bytes (BE + fixed_int + BTreeMap lex order).
- I-CANON-B: `decode(encode(x)) == x` byte-identically for ALL variants.
- I-CANON-C: 2 independent encode calls on the same value produce identical bytes.
- I-CANON-D: per-variant golden fixture: 1 hand-crafted instance per tx kind has a known SHA-256 of canonical bytes, hard-coded in tests. Future serde-derive change → fixture diff → audit-required.

Per STATE spec § 2.5 "Conformance" requirements; § 7 lifts them to invariant status for CO1.1.4-pre1.

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

| ID | STATE spec | CO1.1.4-pre1 v1 | Reason |
|---|---|---|---|
| **D-1** | § 1.2 WorkTx field 12 = `status: TxStatus` | **dropped from wire** | TxStatus is runner book-keeping, not canonical wire data. Mixing it forces every encode to make a status decision, conflating wire-format determinism with runtime state machinery. (Audit input.) |
| **D-2** | § 3.4 `FinalizeTx::from(claim_id, reward)` opaque constructor | **explicit `FinalizeRewardTx` struct** | spec gap; derived schema in § 4 above. |
| **D-3** | § 1.5 `TerminalSummaryTx` | **NOT redefined** here | already shipped in `system_keypair.rs`; CO1.1.4-pre1 imports + reuses; module placement migration (move to typed_tx.rs?) deferred to v1.1 if auditors flag. |

---

## § 10 Audit gates

| Round | Codex | Gemini | Conservative | Action |
|---|---|---|---|---|
| 1 | ⏳ pending | ⏳ pending | TBD | initial v1 audit |
| 2+ | … | … | … | iterate to PASS/PASS |

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
