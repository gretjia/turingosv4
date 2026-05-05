# TB-17 PRE-17.7 design proposal — In-tape Markov β-D pipeline (TerminalSummaryTx → EvidenceCapsule.markov_capsule_cid)

**Status**: **RATIFIED 2026-05-05 — DESIGN-ONLY SHIP; β-A feasibility verification + implementation co-located with TB-18 atom 8 substantive comprehensive_arena build.** (Class 0 doc; no source change at TB-17 ship.)
**Filed**: 2026-05-05.

**Ratification verdict (2026-05-05)**: **RATIFIED — DESIGN-ONLY SHIP; PRE-17.7 → TB-18.**
- Decided by: AI-coder under user-architect autonomous-execution authorization (verbatim: "由你负责执行，一直到TB-17 ship，有任何问题你无法决策，找到架构师意见做准则进行判断，严格执行").
- Standard applied: 2026-05-05 architect verdict §B.8 atom 9 verbatim ("先设计文档，再判断是否需要 signing-payload bump [...] 升级 Class 4"). Design committed (β-A provisional branch §3.5). β-A feasibility verification + resolver implementation + 5 tests + dual audit = multi-day Rust engineering, naturally co-located with TB-18 atom 8 substantive `comprehensive_arena.rs` build (per atom 8 deviation §6.A re-entrant evaluator API which is also the prerequisite for atom 9 emission-order verification §3.4 condition #3).
- §9 disposition: feasibility verification (1)/(2)/(3) deferred to TB-18 implementation phase; default fires (ships as DESIGN-ONLY in TB-17 atom 9; PRE-17.7 carries forward to TB-18 charter).
- TB-17 SG-17.16 ✅ — design-only deferral path satisfied.

**Authority**:
  - TB-17 charter §3 atom 9 + 2026-05-05 architect verdict §B.8 atom 9 verbatim ("先设计文档，再判断是否需要 signing-payload bump [...] 升级 Class 4").
  - MARKOV_INHERITANCE_POLICY.md §4 β long-term mode forward trigger.
  - 2026-05-04 architect OBS_R022 §A.5 + §B.6 (PRE-17.2 + PRE-17.3 mandate that inheritance must be tape-derived OR explicit prior-chain-runtime-repo).
  - Memory: `feedback_markov_inheritance_tape_derived` (per OBS_R022 ruling: tape-derived; no global pointers; per-runtime tolerable as α transitional, in-tape resolution is β long-term; provenance-sidecar rejected).

**Implementation status**: this doc determines whether implementation is **Class 3 additive** (proceed under TB-17 dual-audit envelope) OR **Class 4 signing-payload bump** (STOP for architect ratification per atom 7 pattern).

---

## §1 Problem statement

### §1.1 The α/β migration target

`MARKOV_INHERITANCE_POLICY.md` §4 declares two modes:

```
α (transitional, current):
  CLI arg --prior-chain-runtime-repo <path> + --prior-chain-cas-dir <path>
  Smoke runner provides paths from a deterministic source.

β (long-term canonical, per Art. 0.4 path B):
  Prior capsule resolved IN-TAPE — chain's L4 ledger contains a
  TerminalSummaryTx with an evidence_capsule_cid field pointing to
  a CAS object whose markov_capsule_cid field is the
  previous_capsule_cid for the next chain.
  No CLI argument needed; the chain itself is the authoritative source.
```

PRE-17.7 closes the α → β migration: by TB-17 ship, an audit_tape invocation on a chain MUST be able to derive the chain's `previous_capsule_cid` from the chain's own L4 + CAS, without external CLI hints.

### §1.2 Why this matters

Without β, run-to-run Markov inheritance still depends on **operator knowledge** (who knows the prior chain's filesystem path). Any forgotten / mistyped path → chain treats itself as genesis → silent context discontinuity.

Per `feedback_markov_inheritance_tape_derived` + Art. 0.2 (Tape Canonical), inheritance MUST be tape-derived. β closes this gap structurally rather than relying on procedural discipline.

---

## §2 Existing primitives (TB-15 + TB-16 substrate)

### §2.1 `MarkovEvidenceCapsule`

Defined in `src/runtime/markov_capsule.rs` (TB-15 commit `2337381`). Fields:

```rust
pub struct MarkovEvidenceCapsule {
    capsule_id              : Cid,
    previous_capsule_cid    : Option<Cid>,  // ← target of β resolution
    constitution_hash       : Hash,
    l4_root                 : Hash,
    l4e_root                : Hash,
    cas_root                : Hash,
    typical_errors          : Vec<TypicalErrorSummary>,
    unresolved_obs          : Vec<ObsId>,
    next_session_context_cid: Cid,
    sha256                  : Hash,
    created_at_logical_t    : LogicalT,
}
```

`previous_capsule_cid: Option<Cid>` is the field β fills.

### §2.2 `TerminalSummaryTx`

Defined in `src/state/typed_tx.rs` per TB-11 RunExhausted typed-tx. Current shape (per memory references; verify exact fields at implementation time):

```rust
pub struct TerminalSummaryTx {
    tx_id                  : TxId,
    parent_state_root      : Hash,
    run_outcome            : RunOutcome,  // OmegaAccepted / MaxTxExhausted / WallClockCap / ...
    evidence_capsule_cid   : Cid,         // ← already references EvidenceCapsule
    created_at_logical_t   : LogicalT,
    signature              : SystemSignature,  // system-emitted (architect ChainTape canonical)
}
```

### §2.3 `EvidenceCapsule` (TB-11)

Defined in `src/runtime/evidence_capsule.rs`:

```rust
pub struct EvidenceCapsule {
    capsule_id             : Cid,
    privacy_policy         : CapsulePrivacyPolicy,  // AuditOnly default
    run_outcome            : RunOutcome,
    accepted_count         : u32,
    rejected_count         : u32,
    cas_object_count       : u32,
    duration_seconds       : f64,
    created_at_wallclock   : u64,
    sha256                 : Hash,
    // ... possibly other fields per TB-11 commit
}
```

### §2.4 The pipeline gap

**β requires**: `TerminalSummaryTx.evidence_capsule_cid → EvidenceCapsule → EvidenceCapsule.markov_capsule_cid (NEW FIELD) → MarkovEvidenceCapsule.capsule_id`.

The chain bottoms out at `MarkovEvidenceCapsule` — that's the inherited capsule.

---

## §3 Design — minimal additive vs signing-payload-bump branches

This atom MUST decide: does the implementation require **only additive fields** (Class 3) or a **canonical signing-payload bump** (Class 4)?

### §3.1 Additive option (β-A): EvidenceCapsule gains `markov_capsule_cid: Option<Cid>` field

Proposed:

```rust
pub struct EvidenceCapsule {
    // ... existing TB-11 fields ...
    markov_capsule_cid: Option<Cid>,    // NEW
}
```

**Risk class**: depends on whether `EvidenceCapsule` has a canonical signing payload.

- EvidenceCapsule is **system-emitted** (TB-11 surface; not signed by an external agent).
- The capsule_id is `Cid::from_content` of the canonical-encoded bytes.
- Adding a field changes the canonical encoding → changes the capsule_id for new-shaped capsules.
- Existing TB-15 / TB-16 capsules retain their old (pre-bump) capsule_ids; new chains start with v2 EvidenceCapsule.

**System-signature impact**: if `EvidenceCapsule` is referenced from `TerminalSummaryTx.evidence_capsule_cid` and `TerminalSummaryTx` itself is system-signed → the signing payload changes only if `evidence_capsule_cid` is computed from a v2 EvidenceCapsule (different bytes). This is **NOT** a payload-format change; it's a content-hash drift, which is normal across schema versions.

**TerminalSummaryTx signing payload**: assuming current `TerminalSummaryTx` already includes `evidence_capsule_cid` in its signing payload (system-attested), no schema bump needed.

**Verdict**: if `EvidenceCapsule` and `TerminalSummaryTx` already carry the right hooks → **Class 3 additive**.

### §3.2 Signing-payload-bump option (β-B): TerminalSummaryTx gains `markov_capsule_cid` field directly

Alternative if EvidenceCapsule's structure is awkward to extend (e.g., EvidenceCapsule is a trait or has extension constraints):

```rust
pub struct TerminalSummaryTx {
    // ... existing fields ...
    markov_capsule_cid: Option<Cid>,    // NEW; promotes Markov inheritance to chain-level
}
```

This **WOULD** be a TerminalSummaryTx canonical signing-payload bump → **Class 4**.

### §3.3 Branch decision criteria

| Condition | Branch | Risk class |
|---|---|---|
| `EvidenceCapsule` is a struct with serde-stable canonical encoding AND `TerminalSummaryTx.evidence_capsule_cid` is in its signing payload | **β-A additive** | Class 3 |
| Otherwise (any structural reason additive doesn't cleanly apply) | **β-B signing payload bump** | Class 4 |

### §3.4 Implementation path resolution

At implementation time, the AI-coder MUST inspect `src/runtime/evidence_capsule.rs` + `src/state/typed_tx.rs::TerminalSummaryTx` at the freshest commit and confirm β-A is feasible. Specifically:

1. EvidenceCapsule must use `serde_json` canonical encoding (or BCS) where adding `markov_capsule_cid: Option<Cid>` is a non-breaking serde feature change (`#[serde(default)]`).
2. TerminalSummaryTx must already include `evidence_capsule_cid` in its signing payload.
3. The audit_tape resolver path must be able to derive: `chain.l4 → find TerminalSummaryTx → cas.get(evidence_capsule_cid) → decode EvidenceCapsule → read markov_capsule_cid → cas.get(markov_capsule_cid) → decode MarkovEvidenceCapsule → use as previous_capsule_cid for the NEXT chain's first capsule`.

If steps 1-2 hold → β-A; proceed to Class 3 implementation under TB-17.
If either fails → β-B; STOP for architect ratification.

### §3.5 BRANCH DECISION (filed by AI-coder at design time)

**This design provisionally selects β-A (Class 3 additive)** based on:

- TB-15 EvidenceCapsule already uses serde with extensible field set (TB-11 / TB-15 commits established the pattern).
- `Option<Cid>` with `#[serde(default)]` is non-breaking; old capsules deserialize as `None`; new capsules carry the field.
- `TerminalSummaryTx.evidence_capsule_cid` is already in signing payload (per TB-11 spec; verify at impl).

**If verification at implementation time reveals β-A is infeasible** (e.g., EvidenceCapsule canonical encoding does NOT support backward-compatible field add, or TerminalSummaryTx signing payload omits evidence_capsule_cid), AI-coder MUST:

1. STOP implementation immediately (per `feedback_no_workarounds_strict_constitution`).
2. Re-file design doc with branch flipped to β-B.
3. Request explicit architect ratification per atom 7 pattern.

---

## §4 Resolver design (β-A path)

### §4.1 New module: `src/runtime/markov_resolver.rs`

```rust
/// Resolve previous_capsule_cid for the given chain by walking
/// L4 (latest TerminalSummaryTx) → CAS (EvidenceCapsule) →
/// EvidenceCapsule.markov_capsule_cid.
///
/// Returns:
///   Ok(Some(cid))   if a TerminalSummaryTx exists in chain's L4 AND
///                    its EvidenceCapsule carries a non-None
///                    markov_capsule_cid.
///   Ok(None)        if chain has no TerminalSummaryTx (e.g., chain is
///                    still in progress; OmegaAccepted has fired but
///                    no terminal yet; or chain is genesis).
///   Err(Resolution) on:
///                    - TerminalSummaryTx exists but evidence_capsule_cid
///                      doesn't resolve in CAS
///                    - EvidenceCapsule decodes invalid
///                    - markov_capsule_cid present but doesn't resolve
///                    These are HALT conditions per
///                    MARKOV_INHERITANCE_POLICY §2.3 (Case C invalid).
pub fn resolve_previous_capsule_cid(
    chain: &Chain,
    cas: &CasStore,
) -> Result<Option<Cid>, MarkovResolverError>;
```

### §4.2 Integration with `audit_tape`

Current behavior (α + Option α post-OBS_R022):

- `--markov-pointer <path>` optional; absence → genesis (Layer G Skip).
- `--prior-chain-runtime-repo <path>` reserved (forward-compat) for explicit α inheritance.

Post-PRE-17.7 (β-A) behavior:

- New flag: `--in-tape-markov` (default ON for β chains; default OFF for α-legacy chains for backward-compat).
- When `--in-tape-markov` is ON, audit_tape calls `markov_resolver::resolve_previous_capsule_cid(chain, cas)`:
  - On `Ok(Some(cid))`: that's the Inherited case (MARKOV_INHERITANCE_POLICY §2.2).
  - On `Ok(None)`: Genesis case (§2.1).
  - On `Err(...)`: Invalid case (§2.3); BLOCK with detail string.
- When `--in-tape-markov` is OFF, fall through to α resolution (current behavior preserved).
- `--prior-chain-runtime-repo` flag remains available as a manual override even when `--in-tape-markov` is ON — useful for archeology / cross-chain bridging.

### §4.3 Integration with `generate_markov_capsule`

The capsule generator (TB-15 binary `src/bin/generate_markov_capsule.rs`) currently:
- Takes `--prior-markov-cid <cid>` or reads `LATEST_MARKOV_CAPSULE.txt` (DEPRECATED post-OBS_R022).

Post-PRE-17.7 (β-A) behavior:
- Takes `--in-tape-markov` flag (default ON for β chains).
- When ON: calls `markov_resolver::resolve_previous_capsule_cid` to derive `previous_capsule_cid` for the new capsule being generated.
- When OFF: falls through to existing `--prior-markov-cid` flag.

### §4.4 Wiring `markov_capsule_cid` into EvidenceCapsule emission

Where EvidenceCapsule is emitted today (TB-11 / TB-15 surfaces; sequencer calls into `EvidenceCapsule::write` on RunExhausted / TaskBankruptcy / etc.), the emit path needs to populate `markov_capsule_cid`:

- At the end of a RUN, the sequencer emits a `MarkovEvidenceCapsule` (TB-15 `generate_markov_capsule` runs, OR equivalent in-process).
- The MarkovEvidenceCapsule's capsule_id is then stored in the EvidenceCapsule that the TerminalSummaryTx references.
- Order: MarkovEvidenceCapsule → EvidenceCapsule (with markov_capsule_cid filled in) → TerminalSummaryTx (referencing EvidenceCapsule).

If the existing flow emits TerminalSummaryTx **before** the MarkovEvidenceCapsule generates, the order needs to flip — that's the trickiest implementation question. Atom 8 `comprehensive_arena.rs` substantive build is the natural exercise for this since it must drive multiple tasks → multiple TerminalSummaryTx → multiple EvidenceCapsule emissions in one chain.

---

## §5 Test plan (post-Class-3-confirmation)

### §5.1 `tests/tb_17_pre177_intape_markov_resolves.rs`
Two consecutive chain runs in a single comprehensive_arena evaluator process. Assert run 2's `previous_capsule_cid` equals run 1's `MarkovEvidenceCapsule.capsule_id`, derived from in-tape TerminalSummaryTx → EvidenceCapsule.markov_capsule_cid.

### §5.2 `tests/tb_17_pre177_alpha_path_still_works.rs`
Regression: α CLI sidecar (`--prior-chain-runtime-repo`) still produces correct `previous_capsule_cid` for a manually-bridged cross-chain audit.

### §5.3 `tests/tb_17_pre177_audit_assertions_hold.rs`
After β resolution, audit assertions id=32 (markov_constitution_hash_matches), id=33 (markov_typical_errors_recompute), id=34 (markov_unresolved_obs_recompute), id=35 (markov_next_session_context_resolves) all PASS on β-resolved chains.

### §5.4 `tests/tb_17_pre177_invalid_markov_blocks.rs`
- Construct a synthetic chain where TerminalSummaryTx.evidence_capsule_cid resolves but EvidenceCapsule.markov_capsule_cid points to a non-existent CAS object.
- Assert audit_tape with `--in-tape-markov` ON returns BLOCK with detail `MarkovCapsuleNotFound`.
- Aligns with MARKOV_INHERITANCE_POLICY §2.3 Case C.

### §5.5 `tests/tb_17_pre177_genesis_no_terminal.rs`
Run a chain that did NOT produce TerminalSummaryTx (e.g., was killed mid-execution). Assert β resolver returns `Ok(None)` (Genesis case), NOT an error.

---

## §6 Backward-compatibility

Per `feedback_no_retroactive_evidence_rewrite`:

- Existing chains: their EvidenceCapsules lack `markov_capsule_cid` (decode as `None` via serde-default); resolver returns `Ok(None)` for them — they look like Genesis to β audit_tape, which is **correct** because the inheritance information was simply not written yet.
- α CLI sidecar continues to work for explicit cross-chain bridging of legacy chains.
- Migration of in-flight chains: NONE NEEDED. The next emission will simply use β.

---

## §7 Audit envelope

If β-A holds (Class 3 additive):
- Full hybrid dual external audit (Codex + Gemini) per `feedback_dual_audit` Class 3 tier.
- Audit-loop discipline: round-cap = 2 per `feedback_audit_loop_roi_flip`.

If β-B fires (Class 4 signing-payload bump):
- STOP per atom 7 pattern; file architect ratification request; design doc (this) becomes the gating reference; implementation deferred.

---

## §8 Constitutional alignment

| Constitutional axiom | Impact |
|---|---|
| **Art. 0.1** (Append-Only DAG) | Preserved — additive field; existing capsules unaffected; new capsules append-only. |
| **Art. 0.2** (Tape Canonical) | **STRENGTHENED** — eliminates dependence on filesystem CLI sidecar for inheritance; inheritance becomes tape-derivable per `feedback_markov_inheritance_tape_derived`. |
| **Art. 0.3** (Replay Determinism) | Preserved — resolver is pure-fn over chain L4 + CAS; deterministic. |
| **Art. 0.4** (Q_t version-controlled, path B chain continuation) | **DELIVERED** — this atom is the path-B implementation per architect ruling §B.1.2. |
| **Art. III.1** (raw failure shielding) | Preserved — markov_capsule_cid is just a Cid pointer; no raw bytes leak. |
| **Art. V.1** (三权分立) | Unchanged. |

Net Layer 1 invariant impact: **POSITIVE** — closes a known constitutional gap (filesystem-sidecar dependency for cross-run inheritance).

---

## §9 Decision points for AI-coder at implementation time

The AI-coder, before starting code work, MUST verify at the latest commit:

1. **β-A feasibility**: confirm `EvidenceCapsule` canonical encoding supports backward-compatible `Option<Cid>` field add via `#[serde(default)]`.
2. **TerminalSummaryTx signing payload**: confirm `evidence_capsule_cid` is already in TerminalSummaryTx signing payload OR alternatively the TerminalSummaryTx is system-emitted and signature is derivable from canonical fields (in which case schema-level addition is OK).
3. **EvidenceCapsule emission order**: confirm MarkovEvidenceCapsule generation can occur BEFORE TerminalSummaryTx finalization (so the TerminalSummaryTx can reference an EvidenceCapsule with markov_capsule_cid populated).

If ALL three verify → proceed under Class 3, dual audit at ship.
If ANY fails → STOP, re-file design with β-B branch + architect ratification request.

**Default in absence of feasibility verification**: this design ships as DESIGN-ONLY in TB-17 atom 9; PRE-17.7 carries forward to TB-18 charter.

---

## §10 Cross-references

- TB-17 charter atom 9: `handover/tracer_bullets/TB-17_charter_2026-05-05.md` §3 atom 9
- 2026-05-05 architect verdict §B.8 atom 9: `handover/directives/2026-05-05_TB17_AUDIT_VERDICT_ARCHITECT_RULING.md`
- 2026-05-04 architect OBS_R022 ruling §A.5 + §B.6: `handover/directives/2026-05-04_TB16_OBS_R022_ARCHITECT_RULING.md`
- MARKOV_INHERITANCE_POLICY.md §4 β long-term mode: `handover/markov_capsules/MARKOV_INHERITANCE_POLICY.md`
- Memory: `feedback_markov_inheritance_tape_derived`, `feedback_class4_cannot_hide_in_class3`, `feedback_no_workarounds_strict_constitution`, `feedback_no_retroactive_evidence_rewrite`, `feedback_dual_audit`
- TB-15 MarkovEvidenceCapsule: `src/runtime/markov_capsule.rs`, `src/bin/generate_markov_capsule.rs`
- TB-11 EvidenceCapsule: `src/runtime/evidence_capsule.rs`
- TerminalSummaryTx surface: `src/state/typed_tx.rs`
- Constitution Art. 0.1 / 0.2 / 0.3 / 0.4 / III.1 / V.1
