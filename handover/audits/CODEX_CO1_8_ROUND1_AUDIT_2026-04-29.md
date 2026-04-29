# Codex CO1.8 Round-1 Audit
**Date**: 2026-04-29
**Target**: spec v1 (greenfield)
**HEAD**: e2752e83d40df448622ae05947ac694a1e0cea0e
**Prompt size**: 140471 chars

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
session id: 019dd90e-9890-7090-a27a-68172596f3b2
--------
user
# Codex Adversarial Audit — CO1.8 v1 L5 Materializer (Round 1; greenfield)

**Role**: skeptical adversarial implementer-reviewer. Independent of Gemini round-1 (parallel).

**Mandate**: round-1 dual external audit on CO1.8 v1 — a GREENFIELD spec for the L5 Materialized State + Agent Read View atom. Per CLAUDE.md "Audit Standard": Generator ≠ Evaluator. Per memory `feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS.

**Background**: CO1.7-extra (final L4 atom; STEP_B closed at `4a978f0`) just shipped. CO1.8 is the determinate next atom per LATEST.md "Wave 6 #2" framing + SPRINT_DEPENDENCY_GRAPH line 109 (Materialized State, 8 atoms). This is a greenfield draft from primary sources; no prior rounds.

## What you're reviewing

1. **Spec doc**: `handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md` (~300 lines)
2. **Frozen prior specs (consumed surfaces)**:
   - CO1.1.4-pre1 v1.2.2 — 7-variant TypedTx ABI (`src/state/typed_tx.rs`)
   - CO1.7 v1.2 + CO1.7-impl bundle — `LedgerEntry` + `Sequencer::apply_one` 9-stage
   - CO1.7-extra v1.2.2 — `q.head_t` post-commit binding + `LedgerWriter::head_commit_oid_hex`
   - STATE_TRANSITION_SPEC v1.4 — 7 invocation sites of `materializer::apply` (lines 399/466/560/624/700/758/852)
   - WP § 5.L5 (lines 392-413) — primary source for L5 module structure
3. **Frozen Q_t types**: `src/state/q_state.rs` (Hash sha256-32 + state_root_t field + QState)
4. **L3 CAS surface (snapshot storage)**: `src/bottom_white/cas/store.rs`

## Round 1 audit questions (7)

**Q1. Greenfield scope soundness**: spec § 0.1 declares CO1.8 "the unique unblocked next L-layer atom" post-CO1.7-extra. Verify:
- Is this true? Sprint graph line 109 shows `[CO1.8] blockedBy: CO1.7` — CO1.7 family is closed, but does CO1.8 also (transitively) need CO1.7.5 transition bodies to exist to be testable?
- Spec § 0.4 #1 explicitly defers "live wiring with Sequencer stage 4-7" to a future atom (gated on CO1.7.5). Is the v1 atom (pure-function materializer + tests against fixtures) genuinely standalone, or does the test plan (§ 4) hide a transition-body dependency?
- Is "ship the materializer as a standalone library" a valid Anti-Oreo three-layer decomposition, or does it create a dead-code period (materializer compiled but unused) that violates the "no half-finished implementation" CLAUDE.md "Doing tasks" guidance?

**Q2. The 5 open questions** (spec § 5):
- **Q1 state_root semantics** (sha256-of-snapshot vs literal git-tree object id): is the author's lean architecturally sound? q_state.rs:27 says "generic 32-byte hash (sha256)". STATE spec line 78 says "git tree root in Path B". These are reconcilable IFF state_root = sha256(serialize(snapshot)) AND the snapshot bytes are stored as a git blob whose tree-root is computed separately. Is this the right reading? Are there other readings?
- **Q2 PriorRootNotFound failure mode**: v1 ships failure semantics; lazy reconstruction deferred to CO1.8-extra. Will this break Sequencer apply_one stage 4-7 invocations after a process restart (BTreeMap cache empty post-restart, all subsequent applies fail)? Should v1 ship eager-fill-from-L4-replay instead?
- **Q3 single backing vs separate BTreeMaps**: author lean is single backing with namespaced keys. Does this introduce a write-amp problem (every index update rewrites a single map)? Path B git-tree migration consideration: would namespaced keys map to git-tree paths cleanly, or do separate BTreeMaps map better to separate git refs?
- **Q4 bincode v2 serialization**: re-uses CO1.7-impl precedent. But CO1.7-impl serializes individual `LedgerEntry`s; CO1.8 serializes the FULL state snapshot. Is bincode v2 deterministic across compilation flags / target archs / future versions? Should CO1.8 specify a frozen format-version field?
- **Q5 agent_view stub**: v1 ships no-op filter. Does this leak evaluator internals (Inv 10 Goodhart shield breach) in the interim? Should v1 instead ship a deny-by-default stub that returns minimal hardcoded fields until CO1.5 ships?

**Q3. Sub-atom decomposition (spec § 0.3 8-atom table)**:
- Sprint graph says "Materialized State (8 atoms)" but does NOT enumerate. Author's mapping uses WP § 5.L5's 7 named modules + apply() = 8. Is this the right decomposition?
- Alternative decomposition: WP § 5.L5 lines 397-402 enumerate 6 modules (current_state_db, task_index, agent_reputation_index, error_taxonomy_index, price_signal_index, permission_view) + `read_tool` (line 408) + `apply()` = 8. The author folded "permission_view" into CO1.8.7 `agent_view::project_for_agent`. Is this correct, or is permission_view a SEPARATE module from agent_view (e.g., the access-control matrix vs the projected view)?
- LoC estimates per sub-atom (§ 0.3 column 3): are these grounded? CO1.8.2 apply.rs at ~120 LoC dispatching on TxKind + 7 variants seems low (~17 LoC per variant including doc + test stub).

**Q4. The apply() interface contract** (spec § 2):
- I-DET (determinism): is `apply(r, tx) == apply(r, tx)` actually achievable with bincode v2 serialization of complex types like `BTreeMap<String, Value>`? Map iteration order is deterministic for BTreeMap but not for HashMap. Does the spec mandate BTreeMap throughout state representation?
- I-PURE (no I/O): the BTreeMap-backed state_db is process-state. If multiple apply() calls share the same state_db (CO1.8.3), is apply() reading shared mutable state? If yes, "pure function" is a misnomer — it's a function over (cache, prior_root, tx) with implicit cache parameter.
- Atomicity (§ 2.3): "no atomicity concerns at the materializer layer". But if the state_db is shared mutable (point above), parallel Sequencer instances would race. Spec assumes single-Sequencer; should this be made explicit as a v1 invariant?

**Q5. STATE spec invocation surface match**:
- All 7 STATE spec invocation sites use signature `materializer::apply(&q.state_root_t, tx)`. CO1.8 § 2.1 signature is `apply(prior_root: &Hash, tx: &TypedTx) -> Result<Hash, MaterializerError>`. The 7 STATE spec sites discard the result (no `?` or unwrap shown). Is the STATE spec stale on this point? Should CO1.8 introduce a panic-or-error helper?
- STATE spec line 852 has `materializer::apply(&q.state_root_t, &summary)` — `summary` is a `RejectedAttemptSummary`, NOT a `TypedTx`. Spec § 2.1 signature only accepts `&TypedTx`. Mismatch: either CO1.8 needs an overloaded `apply_summary` variant, or STATE spec line 852 is wrong.

**Q6. Test plan adequacy** (spec § 4, 5 tests):
- `apply_determinism` (4.1): adequate. But: does it cover bincode-version-skew, or only single-process determinism?
- `apply_genesis_to_first_state` (4.2): the assertion `assert_ne!(h, Hash::ZERO)` is necessary but not sufficient; a bug that hashes "v1.0\n" → some_constant for ANY tx would also pass. Should the test check `h ==` a specific golden hex?
- `agent_reputation_increments` (4.3): assumes `materializer::indices::agent_reputation::reputation_for(&new_root, "A0")` exists. But § 0.3 puts CO1.8.5 at ~70 LoC; this implies the public accessor signature. Is the accessor signature spec'd anywhere?
- `agent_view_filters_internals` (4.4): asserts `!view.contains_evaluator_internal_field("oracle_seed")`. But `oracle_seed` is not a field defined in v1 (no PredicateRegistry visibility tags). Is the test compilable in v1, or does it assume CO1.5 surfaces?
- `state_root_reproducibility` (4.5): genuinely substrate-independent.
- Missing tests: NO test exercises `MaterializerError::PriorRootNotFound` path (Q2 above). NO test exercises sub-index cross-references (e.g., does TaskMarket task creation affect agent reputation index?).

**Q7. Strategic risks not yet flagged**:
- Per memory `project_thesis`: "Frozen 5-step compile loop: Proposal → Ground-Truth Feedback → Logging → Capability Compilation → ↑H-VPPUT". Does CO1.8 advance this loop, or is it pure infrastructure that doesn't directly affect H-VPPUT measurability?
- CO1.8.7 `agent_view` is the KEY surface for the "minimal sufficient context" property (WP § 9.2). Stubbing it as no-op might be acceptable for v1 compile, but is there a HARD GATE somewhere downstream (PPUT-CCL Phase D? Phase C unfreeze?) that requires the real visibility filter to be in place?
- The 8 sub-atoms ship as a single compile unit but spec § 3 says "MAY get its own STEP_B-non-restricted commit during impl phase if size warrants". Does this disclaimer leave room for a half-finished interim state where some sub-indices are wired but others aren't?

## Verdict format

Section A: Verdict (PASS/CHALLENGE/VETO) with conviction (LOW/MED/HIGH).
Section B: P0 blockers (must-fix before round-2).
Section C: Open questions raised (architectural).
Section D: Suggested patches (specific spec line/section edits).
Section E: Forward-sustainability notes.

Be concrete. Cite spec § + line where possible.



---

# XREF: spec — handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md

# CO1.8: L5 Materialized State + Agent Read View v1 ⏳ pre-audit (round-1 pending)

**Status**: v1 (2026-04-29; **PENDING round-1 dual external audit** per CLAUDE.md "Audit Standard"). Greenfield atom — `src/bottom_white/materializer/` does not yet exist (verified). Wave 6 #2 per LATEST.md. Determinate next atom post-CO1.7-extra closure (4a978f0).

**Author**: ArchitectAI (Claude); session 2026-04-29.

**Companion specs (frozen, read first)**:
- `CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md` v1.2 — round-3 PASS/PASS; freezes `LedgerWriter` trait + Sequencer 9-stage `apply_one`.
- `CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md` v1.2.2 — round-4 PASS/PASS + Branch B closed; freezes `q.head_t` post-commit binding via `advance_head_t` + `LedgerWriter::head_commit_oid_hex`.
- `CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md` — frozen 7-variant TypedTx ABI; CO1.8 consumes `TypedTx` exclusively (no transition-body internals).
- `STATE_TRANSITION_SPEC_v1_2026-04-27.md` v1.4 — references `materializer::apply(&q.state_root_t, tx)` at 7 invocation sites (lines 399, 466, 560, 624, 700, 758, 852); CO1.8 ships the function those sites call.
- `TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md` § 5.L5 (lines 392-413) — primary source: enumerates 6 sub-modules + `read_tool` minimal-context.

**Single sentence**: ship the L5 `bottom_white::materializer` module as a substrate-independent SHA-256 state-projection layer — deterministic `apply(state_root, tx) → new_state_root` + 6 sub-indices (per WP § 5.L5) + visibility-filtered `agent_view::project_for_agent` — leaving sub-index *content* schema to per-sub-atom audit and leaving runtime integration with Sequencer stage 4-7 to a later wiring atom that depends on CO1.7.5 transition bodies.

---

## § 0 Scope decision

### 0.1 Why this atom exists

Per `SPRINT_DEPENDENCY_GRAPH_v1_2026-04-27.md` line 109-111: `[CO1.8] Materialized State (8 atoms) | blockedBy: CO1.7 | blocks: CO1.9`. CO1.7 family (transition_ledger spec + impl + extra) is ✅ closed (Branch A `5ce01b1` + Branch B closure 4a978f0). CO1.8 is therefore the unique unblocked next L-layer atom in the ChainTape vertical. Per LATEST.md line 156: "**Then** Wave 6 #2/#3 unblocks (CO1.8 L5 materializer + CO1.9 L6 signal indices)".

Per WP § 5.L5 (lines 392-413), L5 is the **agent read surface**: `read_tool(agent_i, task_j, Q_t) → minimal sufficient context`. Agents do NOT read L4 transition ledger directly; they read L5 indices. L5 is therefore the gate between ground-truth ledger (L4) and agent-facing context (top management δ in the constitution mermaid).

### 0.2 What this atom inherits (frozen)

| Frozen by | Surface CO1.8 consumes |
|---|---|
| CO1.1.4-pre1 | 7-variant `TypedTx` ABI; CO1.8 dispatches on `TxKind` discriminant only — no per-variant body interpretation |
| CO1.7-impl | `LedgerEntry` 9-field signing surface; CO1.8 reads `tx_payload_cid` to fetch tx bodies via L3 CAS |
| CO1.7-extra | `q.head_t` post-commit binding via `advance_head_t` — CO1.8 reads `head_t` for replay-from-genesis tests |
| q_state.rs | `pub struct Hash(pub [u8; 32])` SHA-256 generic root + `Hash::ZERO` genesis sentinel + `q.state_root_t: Hash` field |
| L3 CAS (CO1.4 + CO1.4-extra) | `CasStore::put`/`get` round-trip + sidecar JSONL persistence — CO1.8 stores serialized state snapshots in CAS, indexed by state_root |

### 0.3 What this atom delivers (new)

8 sub-atoms (decomposed from whitepaper § 5.L5's 7 named modules + the `apply()` function):

| Sub-atom | Module / fn | LoC est | Substrate-independent? |
|---|---|---|---|
| CO1.8.1 | `bottom_white::materializer::mod.rs` skeleton + `pub use` re-exports + `State` / `StateDelta` types + `MaterializerError` enum | ~80 | ✅ |
| CO1.8.2 | `materializer::apply(prior_root: &Hash, tx: &TypedTx) -> Result<Hash, MaterializerError>` — pure, deterministic, dispatches on `TxKind` | ~120 | ✅ |
| CO1.8.3 | `materializer::state_db::CurrentStateDb` — per-key state cells (BTreeMap-backed in v1; Path B git-tree migration deferred to CO1.8-extra) | ~100 | ✅ |
| CO1.8.4 | `materializer::indices::task_index` — TaskMarket task cells (CO P2.1 surface placeholder; v1 ships read API only) | ~80 | ✅ |
| CO1.8.5 | `materializer::indices::agent_reputation_index` — per-agent reputation counter (`AgentId → u64`); incremented by Verify/Challenge txs (per WP § 5.L5 line 399) | ~70 | ✅ |
| CO1.8.6 | `materializer::indices::{error_taxonomy_index, price_signal_index}` — rejection-class histogram (`ErrorClass → u64`) + market-price-signal cache (`MarketId → PriceSignal`) | ~90 | ✅ |
| CO1.8.7 | `materializer::agent_view::project_for_agent(agent_id: AgentId, q: &QState) -> AgentReadView` — visibility-filtered minimal-context; consumes L1 PredicateRegistry visibility tags (CO1.5 surface) | ~120 | ⚠️ depends on CO1.5 PredicateRegistry visibility tags being shipped (currently § 3.2 unblocked but not started; v1 ships interface + stub returning full view if PredicateRegistry not present) |
| CO1.8.8 | substrate-independent test plan — 5 tests: `apply_determinism` + `apply_genesis_to_first_state` + `agent_reputation_increments_on_verify_tx` + `agent_view_filters_internals` + `state_root_reproducibility_across_restart` | ~180 | ✅ |

**Total v1 estimate**: ~840 LoC across 7 production files + 1 test file (or 5 test files flat-named per CO1.7-extra MF5 convention).

### 0.4 Out of scope (deferred per Anti-Oreo three-layer boundary)

1. **Per-variant transition body integration** — Sequencer stage 4-7 currently call `materializer::apply` against `Err(NotYetImplemented)` transition stubs. Wiring CO1.8 into the live transition path is gated on **future CO1.7.5** (per CO1.7-extra spec § 0.1; gated on CO P2.x substrate). v1 ships the materializer as standalone library; integration is a separate atom.
2. **Path B git-tree state_db backend** — v1 uses BTreeMap-backed in-memory state_db. Migration to git-tree-as-storage substrate is a follow-up CO1.8-extra (likely after TFR S2-S3 git substrate work). This preserves the "Hash = sha256 of serialized state" semantics; only the storage layer changes.
3. **L6 derivable indices** — `reputation_counters` (windowed delta), `price_signals` (market microstructure compression), `failure_histogram` belong to CO1.9 L6 per WP § 5.L5 line 427. CO1.8 ships only L5 absolute-state indices; L6 derived statistics are a separate atom.
4. **CO1.5 PredicateRegistry visibility tags** — CO1.8.7 `project_for_agent` interface lands; tag-driven filtering is no-op (returns full view) until CO1.5 ships visibility tags. Documented as known gap; not blocking.
5. **Materializer integration with WAL** — current `Bus.wal` is the legacy in-memory ledger surface. Materializer state recovery on cold-start uses L4 transition ledger replay (`Git2LedgerWriter`) only; no WAL coupling.

---

## § 1 Module structure

```
src/bottom_white/materializer/
├── mod.rs                    # CO1.8.1 — pub use re-exports + State, StateDelta, MaterializerError
├── apply.rs                  # CO1.8.2 — apply(prior_root, tx) -> new_root
├── state_db.rs               # CO1.8.3 — CurrentStateDb (BTreeMap-backed v1)
├── indices/
│   ├── mod.rs                # re-export
│   ├── task_index.rs         # CO1.8.4 — TaskMarket task cells
│   ├── agent_reputation.rs   # CO1.8.5 — per-agent reputation counter
│   ├── error_taxonomy.rs     # CO1.8.6a — ErrorClass histogram
│   └── price_signal.rs       # CO1.8.6b — MarketId → PriceSignal cache
└── agent_view.rs             # CO1.8.7 — project_for_agent(agent_id, q_t) -> AgentReadView
```

`pub use` chain at `mod.rs` flattens to `crate::bottom_white::materializer::{apply, State, StateDelta, MaterializerError, project_for_agent}` — agent_view is the only sub-module needing a top-level re-export per WP § 5.L5 line 408 read_tool signature.

---

## § 2 `apply()` interface contract

### 2.1 Signature

```rust
/// TRACE_MATRIX § 5.L5 — L5 materialize step (called by Sequencer stage 4-7
/// of all transition bodies per STATE_TRANSITION_SPEC v1.4 lines 399, 466,
/// 560, 624, 700, 758, 852).
///
/// Pure function: deterministic mapping `(prior_root, tx) → new_root`.
/// No I/O; no side effects on outer state. The materialized snapshot is
/// reconstructed in-memory from the prior_root + tx delta and re-hashed
/// to produce the new root.
///
/// Returns `Err(MaterializerError::PriorRootNotFound)` if `prior_root` is
/// not in the state_db cache (v1 pre-Path-B; CO1.8-extra git-tree migration
/// removes this failure mode). For ZERO root (genesis), returns Ok(...).
pub fn apply(
    prior_root: &Hash,
    tx: &TypedTx,
) -> Result<Hash, MaterializerError>;
```

### 2.2 Invariants (audited at sub-atom level)

| Invariant | Statement | Test |
|---|---|---|
| **I-DET** | `apply(r, tx) == apply(r, tx)` for all r, tx | CO1.8.8 `apply_determinism` |
| **I-GEN** | `apply(Hash::ZERO, tx) == h` for some h ≠ ZERO when tx has non-empty effect | CO1.8.8 `apply_genesis_to_first_state` |
| **I-PURE** | apply has no observable side effects: no logs, no mutations to global state, no I/O | static review (no `&mut self`, no `pub static`) |
| **I-CACHE** | repeat apply(r, tx) does not consume O(t) memory; state_db caches by root | static review (BTreeMap caching) |

### 2.3 Atomicity

Materializer apply is invoked from Sequencer `apply_one` AFTER `writer_w.commit(&entry)?` returns Ok and BEFORE `*q_w = q_next` — i.e., between L4 ledger commit and Q_t state advance. v1 ships materializer as a pure function so no atomicity concerns at the materializer layer; the Sequencer's existing q_w lock guarantees serial invocation.

---

## § 3 Sub-atom decomposition (8 atoms — see § 0.3 table)

Sub-atoms are NOT independently shippable in v1 — they form a single compile unit (mod.rs re-exports cascade). Decomposition exists for audit-readability and future-refactor predictability. Each sub-atom MAY get its own STEP_B-non-restricted commit during impl phase if size warrants (>300 LoC delta).

---

## § 4 Test plan (substrate-independent; round-2 MF5 flat-naming convention)

5 tests, flat-named in `tests/`:

### 4.1 `tests/co1_8_apply_determinism.rs`

```rust
#[test]
fn apply_is_deterministic_across_calls() {
    let prior = Hash::ZERO;
    let tx = TypedTx::Work(canonical_work_tx_fixture());
    let h1 = materializer::apply(&prior, &tx).expect("apply 1");
    let h2 = materializer::apply(&prior, &tx).expect("apply 2");
    assert_eq!(h1, h2);
}
```

### 4.2 `tests/co1_8_apply_genesis_to_first_state.rs`

```rust
#[test]
fn apply_advances_state_root_from_zero_on_first_tx() {
    let tx = TypedTx::Work(canonical_work_tx_fixture());
    let h = materializer::apply(&Hash::ZERO, &tx).expect("apply");
    assert_ne!(h, Hash::ZERO, "first tx must advance state root from genesis");
}
```

### 4.3 `tests/co1_8_agent_reputation_increments.rs`

```rust
#[test]
fn verify_tx_increments_target_agent_reputation() {
    let mut q = QState::genesis();
    let verify_tx = canonical_verify_tx_fixture(/* target = */ "A0");
    let new_root = materializer::apply(&q.state_root_t, &verify_tx).expect("apply");
    let view = materializer::indices::agent_reputation::reputation_for(&new_root, "A0");
    assert!(view.unwrap_or(0) > 0, "A0 reputation must increment after VerifyTx");
}
```

### 4.4 `tests/co1_8_agent_view_filters_internals.rs`

```rust
#[test]
fn project_for_agent_returns_minimal_context_only() {
    let q = canonical_q_state_with_internals();
    let view = materializer::project_for_agent("A0", &q);
    assert!(view.contains_task_id("task_0"));
    assert!(!view.contains_evaluator_internal_field("oracle_seed"),
        "agent_view MUST NOT leak evaluator internals (Inv 10 Goodhart shield)");
}
```

### 4.5 `tests/co1_8_state_root_reproducibility.rs`

```rust
#[test]
fn state_root_is_reproducible_across_cold_restart() {
    let txs = vec![/* 5 deterministic TypedTx fixtures */];
    let h_first = txs.iter().fold(Hash::ZERO, |r, tx|
        materializer::apply(&r, tx).expect("apply"));
    // Drop in-memory state; reconstruct from txs alone.
    let h_second = txs.iter().fold(Hash::ZERO, |r, tx|
        materializer::apply(&r, tx).expect("apply (cold)"));
    assert_eq!(h_first, h_second, "state_root reproducibility from L4 ledger replay");
}
```

---

## § 5 Open questions (audit-resolved)

| Q | Statement | Author lean |
|---|---|---|
| Q1 | Does `state_root` literally equal a git tree object id (Path B), or is it sha256 of a serialized state snapshot whose storage *backend* is git tree? | Sha256 of canonical-serialized snapshot. q_state.rs:27 explicitly says "generic 32-byte hash (sha256)". The "git tree root" comment in STATE_TRANSITION_SPEC line 78 refers to where the snapshot LIVES (git tree object), not what state_root EQUALS. v1 ships sha256-of-snapshot semantics. |
| Q2 | Is materializer::apply allowed to fail on `PriorRootNotFound`, or must v1 always succeed (e.g., by lazy snapshot reconstruction from L4 replay)? | v1 ships PriorRootNotFound failure mode (BTreeMap cache lookup). Lazy reconstruction is a CO1.8-extra concern (would require Sequencer reference for L4 replay; out of scope for pure-function v1). |
| Q3 | Should sub-indices share a single state_db backing (one BTreeMap<Key, Value> with namespaced keys) or live in separate BTreeMaps with cross-references? | Single backing with namespaced keys: `("task", task_id)` / `("rep", agent_id)` / etc. Simpler audit; trivial migration to git-tree path-keying in CO1.8-extra. |
| Q4 | What is the canonical serialization format for state snapshots fed into sha256? bincode (CO1.7-impl precedent)? Custom flat-buffer? | bincode v2 `BorrowDecode` per CO1.7-impl A1 precedent. Re-uses existing dependency; deterministic-encoding-by-construction. |
| Q5 | Does `agent_view::project_for_agent` consume CO1.5 PredicateRegistry visibility tags, or does v1 ship a no-op filter (returns full view)? | v1 ships no-op filter with explicit TODO `pending CO1.5`. Documented as known gap in § 0.4 #4. Not blocking for Wave 6 #2. |

---

## § 6 Audit gates

| Round | Codex | Gemini | Conservative | Action |
|---|---|---|---|---|
| 1 (this v1) | ⏳ pending | ⏳ pending | TBD | round-1 dual external audit on CO1.8 v1 |
| 2+ | … | … | … | iterate to PASS/PASS |

**Pre-implementation gate**: spec must reach PASS/PASS before any code in `src/bottom_white/materializer/**` is written. Per CLAUDE.md "Audit Standard". No STEP_B-restricted files touched (kernel.rs / bus.rs / wallet.rs UNTOUCHED by v1; only new files under `src/bottom_white/materializer/`).

---

## § 7 Estimated scope

- **Spec rounds**: round-1 expected CHALLENGE/CHALLENGE (open Q1-Q5 absorb both audits); round-2 PASS-or-CHALLENGE; round-3 PASS/PASS likely. Round budget ~$10-15.
- **Implementation scope** (post-PASS/PASS):
  - 8 sub-atoms × ~80-180 LoC each = ~840 LoC production + ~180 LoC tests.
  - Largest single file: `apply.rs` (~120 LoC; dispatches on TxKind discriminant).
- **Total atom budget**: ~1020 LoC; **estimated calendar time**: 2-4 days impl + 1-2 days audit cycles. Cumulative project audit spend after CO1.8 PASS/PASS: ~$210-330 / $890 mid-budget.

---

## § 8 Honest acknowledgements

1. **Greenfield atom**: `src/bottom_white/materializer/` does not exist. Verified via `ls src/bottom_white/` showing only `cas`, `ledger`, `mod.rs`, `tools`. v1 creates the module from primary sources (whitepaper § 5.L5 + STATE spec invocation surface + sprint graph dependency note).
2. **8 sub-atoms is author-decomposed**, not pre-existing in any document. SPRINT_DEPENDENCY_GRAPH line 109 says "Materialized State (8 atoms)" but does not enumerate. The decomposition in § 0.3 is the author's best mapping of WP § 5.L5's 7 named modules + the `apply()` function (= 8). Audit may suggest re-decomposition.
3. **CO1.5 PredicateRegistry visibility-tag dependency is acknowledged but stubbed** (§ 0.4 #4). Without CO1.5 visibility tags, `project_for_agent` is a no-op filter. This means CO1.8.7 ships an interface, not a fully-functional Inv-10 Goodhart shield. The gap is documented; CO1.5 + a follow-up CO1.8-extra closes the loop.
4. **state_root semantics interpretation lean** (Q1) is author's reading of q_state.rs:27 ("generic 32-byte hash (sha256)") taking precedence over STATE_TRANSITION_SPEC line 78's "git tree root in Path B" gloss. If audit finds the gloss authoritative, v1 § 2 signature must change to integrate with git2-rs tree-builder API.
5. **Path B git-tree backend deferred** to CO1.8-extra (§ 0.4 #2). v1 ships in-memory BTreeMap to keep the substrate-independence invariant (no git2-rs touch in v1). This means v1 alone does NOT cold-restart from disk; cold-restart-via-L4-replay test (CO1.8.8) only verifies determinism, not durability. Durability lands with CO1.8-extra.
6. **No STEP_B-restricted file touches**. Kernel + bus + wallet untouched. v1 is pure-additive at `src/bottom_white/materializer/**`. No STEP_B parallel-branch ceremony required.
7. **FC-trace requirements**: every new pub symbol in CO1.8 implementation must carry `/// TRACE_MATRIX § 5.L5: <role>` doc-comment per CLAUDE.md "Alignment Standard". Set: `apply` + `State` + `StateDelta` + `MaterializerError` + `CurrentStateDb` + `task_index` accessors + `agent_reputation::reputation_for` + `error_taxonomy` + `price_signal` + `project_for_agent`.

---

## § 9 Pre-audit smoke test plan

Per memory `feedback_smoke_before_batch`. Smoke run before round-1 audit launch, at the v1 commit HEAD.

| # | Claim | Smoke command | Pass criterion |
|---|---|---|---|
| S1 | `Hash` is `pub struct Hash(pub [u8; 32])` (sha256-sized) | `grep -A1 'pub struct Hash' src/state/q_state.rs` | matches |
| S2 | `Hash::ZERO` exists as genesis sentinel | `grep -n 'pub const ZERO: Hash' src/state/q_state.rs` | one hit |
| S3 | `q.state_root_t: Hash` field present | `grep -n 'pub state_root_t' src/state/q_state.rs` | one hit |
| S4 | `materializer::apply` is invoked at 7 sites in STATE spec | `grep -c 'materializer::apply' handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md` | 7 |
| S5 | `bottom_white::materializer` does NOT exist (greenfield) | `ls src/bottom_white/materializer/ 2>&1` | "No such file or directory" |
| S6 | `TypedTx` 7-variant ABI frozen and present | `grep -n 'pub enum TypedTx' src/state/typed_tx.rs && grep -c 'TxKind::' src/state/typed_tx.rs` | one TypedTx hit; ≥7 TxKind:: hits |
| S7 | CO1.5 PredicateRegistry status | check § 3.2 of decision map for CO1.5 line | confirm "🟢 ready but not started" or equivalent (justifies § 0.4 #4 stub) |
| S8 | CAS surface available for snapshot storage | `grep -n 'pub fn put\|pub fn get' src/bottom_white/cas/store.rs` | both present |
| S9 | bincode v2 dependency available | `grep -n 'bincode' Cargo.toml` | dep present |
| S10 | cargo baseline | `cargo check --workspace && cargo test --workspace --lib` | clean compile + 239/0/1 (matches post-CO1.7-extra-impl baseline at HEAD `4a978f0`) |

---

**END v1 DRAFT body.**

## Pre-audit smoke results

### Round-1 smoke (HEAD `4a978f0`; v1)

| # | Claim | Result | Status |
|---|---|---|---|
| S1 | Hash type | `pub struct Hash(pub [u8; 32])` (q_state.rs:29) | ✅ PASS |
| S2 | Hash::ZERO | `pub const ZERO: Hash = Hash([0u8; 32])` (q_state.rs:33) | ✅ PASS |
| S3 | q.state_root_t | `pub state_root_t: Hash` (q_state.rs:313) | ✅ PASS |
| S4 | materializer::apply invocations in STATE spec | 7 hits (lines 399/466/560/624/700/758/852) | ✅ PASS |
| S5 | greenfield | `ls: cannot access 'src/bottom_white/materializer/': No such file or directory` | ✅ PASS |
| S6 | TypedTx ABI | `pub enum TypedTx` at line 608; 14 TxKind:: hits | ✅ PASS |
| S7 | CO1.5 status | Wave 2 sub-choice B; "CO P2.1 uses CO1.5 visibility" — CO1.5 still in-flight per § 3.2 | ✅ PASS (justifies § 0.4 #4 stub) |
| S8 | CAS surface | `pub fn put` line 163; `pub fn get` line 199 | ✅ PASS |
| S9 | bincode v2 | `bincode = { version = "2", features = ["serde"] }` (Cargo.toml:9) | ✅ PASS |
| S10 | cargo baseline | check clean (warnings pre-existing); test 239/0/1 ignored (sequencer_serial_replay_byte_identity, deferred to future CO1.7.5) | ✅ PASS |

**Smoke gate v1**: 10/10 PASS at HEAD `4a978f0`. Spec v1 ready for round-1 dual external audit.

## Patch log

**v1 (2026-04-29; greenfield draft)** — initial spec draft from primary sources:
- Whitepaper § 5.L5 lines 392-413 (6 sub-modules + read_tool minimal-context)
- STATE_TRANSITION_SPEC v1.4 lines 399/466/560/624/700/758/852 (7 `materializer::apply` invocation sites)
- SPRINT_DEPENDENCY_GRAPH v1 line 109 ("Materialized State (8 atoms)")
- TRACE_MATRIX_v3 row § 5.L5 (module path `bottom_white::materializer::{state_db, indices, agent_view}`)
- q_state.rs:27-49 (Hash type + state_root_t field)
- CO1.7-extra v1.2.2 + CO1.7-impl bundle (frozen interfaces consumed)

8 sub-atoms (CO1.8.1-CO1.8.8) decomposed by author from WP § 5.L5's 7 named modules + the `apply()` function. 5 substrate-independent tests (flat-named per CO1.7-extra MF5 convention). 5 open questions for round-1 audit (Q1 state_root semantics being the most consequential).

### Awaiting

1. ⏳ pre-audit smoke run at v1 commit HEAD (S1-S10 from § 9)
2. ⏳ round-1 dual external audit (Codex + Gemini per CLAUDE.md "Audit Standard"; conservative VETO>CHALLENGE>PASS per memory `feedback_dual_audit_conflict`)
3. ⏳ iterate v1.x patches per audit findings until PASS/PASS
4. ⏳ implementation start gated on PASS/PASS spec


---

# XREF: WP § 5.L5 (whitepaper primary source)

```
timestamp
status
```

这层可以是：

- local hash chain
- append-only JSONL
- Git commit graph
- permissioned blockchain
- public-chain settlement proof

#### Layer 5：Materialized State and Agent Read View

物化状态层。

```
current_state_db
task_index
agent_reputation_index
error_taxonomy_index
price_signal_index
permission_view
```

Agent 实际读取的是这一层，而不是底层完整账本。

```
read_tool(agent_i, task_j, Q_t)
  -> minimal sufficient context
```

这正是选择性屏蔽与渐进式披露。

#### Layer 6：Signal Indices

信号索引层。

```
boolean pass/fail history
typical error clusters
price signals
reputation counters
resource scarcity indicators
exploration/exploitation statistics
```

> **L5 vs L6 区分 (Art 0.2 重建性合规)**: Layer 5 是**当前状态的物化视图**（agent 直接 read_tool 读到的最小充分上下文，含 `agent_reputation_index` / `price_signal_index`）。Layer 6 是**统计聚合 / 趋势压缩**（`reputation counters` 是窗口内增量统计，`price signals` 是市场微观结构压缩）。**L6 严格 derivable from L4 + L5**：删掉 L6 不破坏 Art 0.2 重建性，因为 L4 (transition ledger) 是 ground truth，L5 是 L4 物化，L6 是 L5 统计变换。L6 存在的唯一目的是**降低顶层白盒访问成本**（从 O(账本扫描) 降到 O(预聚合查询)）。

这层让顶层白盒能够持续进行量化、广播、屏蔽。

---

## 6. 状态转移协议

一次标准计算循环如下。

### 6.1 Read

```
input_i = rtool(Q_t, agent_i, task_j)

```


---

# XREF: STATE_TRANSITION_SPEC v1.4 (7 materializer::apply invocation sites)

```
397-
398-    // L5 materialize
399:    let new_state_root = materializer::apply(&q.state_root_t, tx);
400-    q_next.state_root_t = new_state_root;
401-
--
464-    // STAGE 5: append + materialize + signals
465-    q_next.ledger_root_t = ledger::append(&q.ledger_root_t, tx);
466:    q_next.state_root_t  = materializer::apply(&q.state_root_t, tx);
467-    q_next.head_t        = NodeId::from_state_root(q_next.state_root_t);
468-
--
558-    // STAGE 6: append + materialize + signals
559-    q_next.ledger_root_t = ledger::append(&q.ledger_root_t, tx);
560:    q_next.state_root_t  = materializer::apply(&q.state_root_t, tx);
561-    q_next.head_t        = NodeId::from_state_root(q_next.state_root_t);
562-
--
622-    // STAGE 4: append + materialize (no signals; royalty paid at finalize_reward time)
623-    q_next.ledger_root_t = ledger::append(&q.ledger_root_t, tx);
624:    q_next.state_root_t  = materializer::apply(&q.state_root_t, tx);
625-    q_next.head_t        = NodeId::from_state_root(q_next.state_root_t);
626-
--
698-    // STAGE 4: emit terminal signals
699-    q_next.ledger_root_t = ledger::append(&q.ledger_root_t, &FinalizeTx::from(claim_id, reward));
700:    q_next.state_root_t  = materializer::apply(&q.state_root_t, &FinalizeTx::from(claim_id, reward));
701-    q_next.head_t        = NodeId::from_state_root(q_next.state_root_t);
702-
--
756-    // STAGE 7: append + materialize + signal (purely on tx, q)
757-    q_next.ledger_root_t = ledger::append(&q.ledger_root_t, tx);
758:    q_next.state_root_t  = materializer::apply(&q.state_root_t, tx);
759-    q_next.head_t        = NodeId::from_state_root(q_next.state_root_t);
760-
--
850-    let mut q_next = q.clone();
851-    q_next.ledger_root_t = ledger::append(&q.ledger_root_t, &summary);
852:    q_next.state_root_t  = materializer::apply(&q.state_root_t, &summary);
853-    q_next.head_t        = NodeId::from_state_root(q_next.state_root_t);
854-

```


---

# XREF: src/state/q_state.rs (Hash + state_root_t field)

```rust
//! Q_t — system state vector per `STATE_TRANSITION_SPEC v1.4 § 1.1`.
//!
//! TRACE_MATRIX Art 0.1 — 四要素映射: `QState` provides the tape/control mapping.
//! TRACE_MATRIX Art 0.4 — Q_t version-controlled: `head_t` = git commit SHA in Path B substrate.
//! TRACE_MATRIX Art IV — Boot: `QState::genesis` is the starting state of every runtime.
//! TRACE_MATRIX WP § 0 axiom 1 — state monotonicity: Q_t evolves only via accepted transitions.
//! TRACE_MATRIX WP § 4 — 9-component system state.
//! TRACE_MATRIX WP § 2 economic — `EconomicState` 9 sub-fields (CO1.2.2).
//!
//! **BTreeMap, not HashMap, everywhere** (Inv determinism;
//! Codex flagged `kernel.rs:187-204` HashMap nondeterminism in round-2).
//!
//! Sub-types whose entry shapes are scoped to later atoms (CO P2.x economic engine,
//! CO1.7 transition_ledger) are intentionally minimal here — full schemas land per atom,
//! but the *index typing* (BTreeMap newtype shells) freezes here so Q_t is total.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::economy::money::MicroCoin;

// ────────────────────────────────────────────────────────────────────────────
// Newtype primitives — minimal, deterministic, serde-ready.
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX § 1.1 — generic 32-byte hash (sha256). State / ledger / registry roots.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Hash(pub [u8; 32]);

impl Hash {
    /// TRACE_MATRIX § 1.1 — additive identity (genesis state-root, ledger-root, etc.).
    pub const ZERO: Hash = Hash([0u8; 32]);

    /// TRACE_MATRIX § 1.1 — construct from a 32-byte digest (sha256 output).
    pub fn from_bytes(b: [u8; 32]) -> Self {
        Hash(b)
    }
}

impl Default for Hash {
    fn default() -> Self {
        Hash::ZERO
    }
}

/// TRACE_MATRIX Art 0.4 — `head_t` = git commit SHA in Path B substrate (40 hex chars).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct NodeId(pub String);

impl NodeId {
    /// TRACE_MATRIX § 3 — pseudocode `NodeId::from_state_root(state_root)` constructor.
    /// Concrete derivation (commit-tree-of-state-root) lands in CO1.7 transition_ledger.
    pub fn from_state_root(state_root: Hash) -> Self {
        let mut s = String::with_capacity(64);
        for byte in state_root.0.iter() {
            s.push_str(&format!("{:02x}", byte));
        }
        NodeId(s)
    }
}

/// TRACE_MATRIX § 1.1 — agent identity (string, opaque to Q_t).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct AgentId(pub String);

/// TRACE_MATRIX § 1.1 — accepted-transaction id (string, opaque to Q_t).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct TxId(pub String);

/// TRACE_MATRIX § 1.1 — reputation snapshot. Signed i64 to permit negative reputation
/// (e.g. post-slash); ledger-of-record lives in `ReputationsIndex` (CO P2.9).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct Reputation(pub i64);

// ────────────────────────────────────────────────────────────────────────────
// AgentSwarmState + PerAgentState — spec § 1.1 verbatim.
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX § 1.1 — agent swarm sub-state.
/// MUST be reconstructible from L4 transition ledger replay.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AgentSwarmState {
    pub agents: BTreeMap<AgentId, PerAgentState>,
    pub current_round: u64,
}

/// TRACE_MATRIX § 1.1 — per-agent runtime state.
/// `retry_counter_for_current_task` resets on accept; persists across rejections.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PerAgentState {
    pub reputation_snapshot: Reputation,
    pub last_accepted_tx: Option<TxId>,
    pub retry_counter_for_current_task: u32,
}

// ────────────────────────────────────────────────────────────────────────────
// AgentVisibleProjection — Inv 10 Goodhart shield (CO P2.7 visibility runtime).
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX § 1.1 — agent-visible projection of tape filtered by per-agent
/// visibility policy (Inv 10 Goodhart shield; `top_white::predicates::visibility`).
///
/// `views`: per-agent filtered head pointer; full filtering machinery lands in CO P2.7.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AgentVisibleProjection {
    pub views: BTreeMap<AgentId, NodeId>,
}

// ────────────────────────────────────────────────────────────────────────────
// BudgetSnapshot — global compute / cost / wall-clock budget.
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX § 1.1 — global budget snapshot:
/// cost ceiling (MicroCoin), wall clock remaining (ms), compute cap remaining.
/// Exhaustion → halt_reason ∈ {WallClockCap, ComputeCapViolated, MaxTxExhausted}.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BudgetSnapshot {
    pub cost_ceiling_microcoin: MicroCoin,
    pub wall_clock_remaining_ms: u64,
    pub compute_cap_remaining: u64,
}

impl Default for BudgetSnapshot {
    fn default() -> Self {
        Self {
            cost_ceiling_microcoin: MicroCoin::zero(),
            wall_clock_remaining_ms: 0,
            compute_cap_remaining: 0,
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────
// EconomicState — WP § 2 economic, 9 sub-fields. Atom CO1.2.2.
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX WP § 2 economic — 9-sub-field economic state. Each sub-index
/// is a BTreeMap newtype; entry shapes (Escrow / Stake / Claim / TaskMarket /
/// RoyaltyEdge / ChallengeCase) are minimal-but-typed here and fully fleshed
/// in the owning atoms (CO P2.1-2.6).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct EconomicState {
    pub balances_t: BalancesIndex,
    pub escrows_t: EscrowsIndex,
    pub stakes_t: StakesIndex,
    pub claims_t: ClaimsIndex,
    pub reputations_t: ReputationsIndex,
    pub task_markets_t: TaskMarketsIndex,
    pub royalty_graph_t: RoyaltyGraph,
    pub challenge_cases_t: ChallengeCasesIndex,
    pub price_index_t: PriceIndex,
}

/// TRACE_MATRIX WP § 2 — agent → balance ledger. Concrete entry: `MicroCoin` (CO1.0a).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct BalancesIndex(pub BTreeMap<AgentId, MicroCoin>);

/// TRACE_MATRIX WP § 2 — tx → escrow entry. Full schema lands CO P2.2 EscrowVault.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct EscrowsIndex(pub BTreeMap<TxId, EscrowEntry>);

/// TRACE_MATRIX WP § 2 — escrow entry shape (stub). Full fields land CO P2.2.
/// `#[serde(default)]` on each field gives forward-compat: future atoms can add
/// fields without breaking deserialization of historical ledger rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EscrowEntry {
    #[serde(default = "MicroCoin::zero")]
    pub amount: MicroCoin,
    #[serde(default)]
    pub depositor: AgentId,
}

impl Default for EscrowEntry {
    fn default() -> Self {
        Self { amount: MicroCoin::zero(), depositor: AgentId::default() }
    }
}

/// TRACE_MATRIX WP § 2 — tx → stake entry. Full schema lands CO P2.5 ChallengeCourt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct StakesIndex(pub BTreeMap<TxId, StakeEntry>);

/// TRACE_MATRIX WP § 2 — stake entry shape (stub). Full fields land CO P2.5.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StakeEntry {
    #[serde(default = "MicroCoin::zero")]
    pub amount: MicroCoin,
    #[serde(default)]
    pub staker: AgentId,
}

impl Default for StakeEntry {
    fn default() -> Self {
        Self { amount: MicroCoin::zero(), staker: AgentId::default() }
    }
}

/// TRACE_MATRIX WP § 2 — tx → reward claim. Full schema lands CO P2.6 SettlementEngine.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ClaimsIndex(pub BTreeMap<TxId, ClaimEntry>);

/// TRACE_MATRIX WP § 2 — claim entry shape (stub). Full fields land CO P2.6.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClaimEntry {
    #[serde(default = "MicroCoin::zero")]
    pub amount: MicroCoin,
    #[serde(default)]
    pub claimant: AgentId,
}

impl Default for ClaimEntry {
    fn default() -> Self {
        Self { amount: MicroCoin::zero(), claimant: AgentId::default() }
    }
}

/// TRACE_MATRIX WP § 2 — agent → reputation ledger.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ReputationsIndex(pub BTreeMap<AgentId, Reputation>);

/// TRACE_MATRIX WP § 2 — tx → task market. Full schema lands CO P2.1.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TaskMarketsIndex(pub BTreeMap<TxId, TaskMarketEntry>);

/// TRACE_MATRIX WP § 2 — task market entry shape (stub). Full fields land CO P2.1.
/// Default values (verifier_quorum=1, max_reuse_royalty_fraction=0.10) match the
/// PROJECT_DECISION_MAP § 2.3 spec gap defaults.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskMarketEntry {
    #[serde(default)]
    pub publisher: AgentId,
    #[serde(default = "MicroCoin::zero")]
    pub bounty: MicroCoin,
    #[serde(default = "task_market_default_quorum")]
    pub verifier_quorum: u32,
    #[serde(default = "task_market_default_royalty_bp")]
    pub max_reuse_royalty_fraction_basis_points: u16,
}

fn task_market_default_quorum() -> u32 {
    1
}
fn task_market_default_royalty_bp() -> u16 {
    1000
}

impl Default for TaskMarketEntry {
    fn default() -> Self {
        Self {
            publisher: AgentId::default(),
            bounty: MicroCoin::zero(),
            verifier_quorum: 1,
            max_reuse_royalty_fraction_basis_points: 1000, // 0.10 per spec gap default
        }
    }
}

/// TRACE_MATRIX WP § 2 — directed royalty edges (reuse depth attribution).
/// Full attribution algebra lands CO P2.4.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RoyaltyGraph(pub BTreeMap<TxId, Vec<RoyaltyEdge>>);

/// TRACE_MATRIX WP § 2 — single royalty edge (ancestor → reuse weight). Stub; CO P2.4.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RoyaltyEdge {
    #[serde(default)]
    pub ancestor: TxId,
    #[serde(default)]
    pub fraction_basis_points: u16,
}

/// TRACE_MATRIX WP § 2 — tx → challenge case. Full schema lands CO P2.5.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ChallengeCasesIndex(pub BTreeMap<TxId, ChallengeCase>);

/// TRACE_MATRIX WP § 2 — challenge case shape (stub). Full fields land CO P2.5.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChallengeCase {
    #[serde(default)]
    pub challenger: AgentId,
    #[serde(default = "MicroCoin::zero")]
    pub bond: MicroCoin,
    #[serde(default)]
    pub opened_at_round: u64,
}

impl Default for ChallengeCase {
    fn default() -> Self {
        Self { challenger: AgentId::default(), bond: MicroCoin::zero(), opened_at_round: 0 }
    }
}

/// TRACE_MATRIX WP § 2 — tx → posted price (last accepted price index).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PriceIndex(pub BTreeMap<TxId, MicroCoin>);

// ────────────────────────────────────────────────────────────────────────────
// QState — § 1.1 verbatim, 9 fields.
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX § 1.1 — system state Q_t. 9 fields per WP § 4 + economic § 2 amendment.
///
/// Reconstructibility: every field is derivable from L4 transition ledger replay
/// (Art IV Boot 公理).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct QState {
    /// Agent swarm sub-state (tape head per agent + per-agent reputation snapshots).
    pub q_t: AgentSwarmState,
    /// Current ChainTape head pointer = git commit SHA in Path B substrate.
    pub head_t: NodeId,
    /// Materialized state Merkle root (git tree root in Path B).
    pub state_root_t: Hash,
    /// Agent-visible projection of tape filtered by per-agent visibility policy.
    pub tape_view_t: AgentVisibleProjection,
    /// L4 Transition Ledger root (Merkle root of all accepted tx so far).
    pub ledger_root_t: Hash,
    /// L1 Predicate Registry root.
    pub predicate_registry_root_t: Hash,
    /// L2 Tool Registry root.
    pub tool_registry_root_t: Hash,
    /// Economic state (WP § 2 amendment, 9 sub-fields).
    pub economic_state_t: EconomicState,
    /// Global budget snapshot.
    pub budget_state_t: BudgetSnapshot,
}

impl QState {
    /// TRACE_MATRIX Art IV Boot — genesis Q_t. All zero / empty;
    /// roots populated by `boot::verify_trust_root` and the `state_root_t` published
    /// in `genesis_payload.toml [constitution_root]`.
    pub fn genesis() -> Self {
        QState::default()
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Inline determinism tests (round-trip + insertion-order independence).
// Conformance tests proper live in tests/{four_element_mapping, q_state_reconstruct,
// economic_state_reconstruct, six_axioms_alignment}.rs per TRACE_MATRIX_v3.
// ────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn genesis_q_state_is_total_and_default() {
        let g = QState::genesis();
        assert_eq!(g, QState::default());
        assert_eq!(g.q_t.current_round, 0);
        assert!(g.q_t.agents.is_empty());
        assert_eq!(g.head_t, NodeId::default());
        assert_eq!(g.state_root_t, Hash::ZERO);
    }

    #[test]
    fn nine_field_count_via_serde_json() {
        // Sanity that QState has exactly 9 top-level fields.
        let s = serde_json::to_value(QState::genesis()).unwrap();
        let obj = s.as_object().expect("object");
        assert_eq!(
            obj.len(),
            9,
            "QState must have exactly 9 fields per WP § 4; got {}",
            obj.len()
        );
        for k in &[
            "q_t",
            "head_t",
            "state_root_t",
            "tape_view_t",
            "ledger_root_t",
            "predicate_registry_root_t",
            "tool_registry_root_t",
            "economic_state_t",
            "budget_state_t",
        ] {
            assert!(obj.contains_key(*k), "QState missing field {}", k);
        }
    }

    #[test]
    fn economic_state_has_nine_sub_fields() {
        let e = EconomicState::default();
        let s = serde_json::to_value(&e).unwrap();
        let obj = s.as_object().unwrap();
        assert_eq!(
            obj.len(),
            9,
            "EconomicState must have 9 sub-fields per WP § 2; got {}",
            obj.len()
        );
    }

    #[test]
    fn btreemap_insertion_order_independent_serialization() {
        // Insertion-order independence (Inv determinism).
        let mut a = BalancesIndex::default();
        a.0.insert(AgentId("alice".into()), MicroCoin::from_coin(10).unwrap());
        a.0.insert(AgentId("bob".into()), MicroCoin::from_coin(20).unwrap());

        let mut b = BalancesIndex::default();
        b.0.insert(AgentId("bob".into()), MicroCoin::from_coin(20).unwrap());
        b.0.insert(AgentId("alice".into()), MicroCoin::from_coin(10).unwrap());

        let sa = serde_json::to_string(&a).unwrap();
        let sb = serde_json::to_string(&b).unwrap();
        assert_eq!(sa, sb, "BTreeMap must yield identical bytes regardless of insertion order");
    }

    #[test]
    fn node_id_from_state_root_is_deterministic() {
        let r = Hash::from_bytes([0xAB; 32]);
        let n1 = NodeId::from_state_root(r);
        let n2 = NodeId::from_state_root(r);
        assert_eq!(n1, n2);
        assert_eq!(n1.0.len(), 64, "40-byte git SHA hex form would be 40; we use full 32-byte sha256 hex = 64");
    }
}

```


---

# XREF: src/state/typed_tx.rs (TypedTx 7-variant ABI)

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
/// `WorkSigningPayload::canonical_digest()` — i.e. the projection produced by
/// `WorkTx::to_signing_payload()` (excludes the signature field itself; per
/// v1.1 P1 the digest pre-image carries the `b"turingosv4.agent_sig.work.v1"`
/// domain prefix).
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
/// pattern as `LedgerEntrySigning` — so the digest is computed here via
/// `TerminalSummaryTx::to_signing_payload().canonical_digest()` (with the
/// `b"turingosv4.system_sig.terminal_summary.v1"` domain prefix per v1.1 P1)
/// and `system_keypair` stays oblivious to the typed-tx schema (no circular
/// `bottom_white ↔ state` dependency).
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
/// 7 variants (K5 closed: NO `Slash`). All variants are defined in this
/// module (`state::typed_tx`); v1.1 P3 migrated `TerminalSummaryTx` here
/// from a 3-field placeholder previously in `system_keypair.rs`.
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

# XREF: src/bottom_white/cas/store.rs (snapshot storage surface)

```rust
//! CAS store backed by git2-rs blob layer.
//!
//! Each runtime_repo (per spec § 5.2.2 cell isolation) has its own CasStore.
//! Objects are content-addressed by `Cid` (sha256 of content); git's sha-1
//! OID is recorded but not canonical.
//!
//! **CO1.4-extra (this atom)** adds index persistence: the `Cid → metadata`
//! map is durably persisted to a sidecar JSONL file at
//! `<repo_path>/.turingos_cas_index.jsonl`. On `CasStore::open()` the sidecar
//! is replayed into an in-memory BTreeMap; on `CasStore::put()` (new entries
//! only) one JSONL line is appended + flushed. This closes the Art 0.2
//! tape-canonicality cold-replay gate that CO1.7 spec § 0 + CO1.1.4-pre1
//! v1.1 § 0.1 declared a hard prerequisite for `replay_full_transition`
//! (CO1.7-impl A4).
//!
//! **Design choice (sidecar JSONL)**: chosen over (b) git-tag manifest /
//! (c) bincode index + WAL because (a) is the simplest deterministic
//! append-only artifact, replayable from scratch, easy to audit by reading.
//! Per "压缩即智能" — pick simplest correct shape; upgrade later if profiling
//! shows O(N)-on-restart cost is real.
//!
//! /// TRACE_MATRIX WP-arch-§5.L3 + spec-§5.2.2 (cell isolation): CAS store
//! /// TRACE_MATRIX CO1.7 spec § 0 + CO1.1.4-pre1 § 0.1 cross-atom ordering:
//! /// CAS index persistence — required by `replay_full_transition` cold-restart.

use git2::{ObjectType as Git2ObjectType, Repository};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use super::schema::{CasObjectMetadata, Cid, ObjectType};

const CAS_INDEX_FILENAME: &str = ".turingos_cas_index.jsonl";

#[derive(Debug)]
pub enum CasError {
    /// git2-rs underlying error.
    Git2(git2::Error),
    /// Cid not found in this CasStore's metadata index.
    CidNotFound(Cid),
    /// Content stored at git OID but Cid metadata absent (corrupted index).
    MetadataMissing(Cid),
    /// Content's sha256 doesn't match the asserted Cid (corruption).
    CidMismatch { expected: Cid, computed: Cid },
    /// I/O error reading or writing the CO1.4-extra sidecar index file.
    IoError(io::Error),
    /// JSON-deserialization error on a sidecar index line. Includes 1-based
    /// line number for diagnostics.
    IndexParse { line: usize, error: String },
}

impl std::fmt::Display for CasError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Git2(e) => write!(f, "git2 backend error: {e}"),
            Self::CidNotFound(c) => write!(f, "{c} not found in CAS index"),
            Self::MetadataMissing(c) => write!(f, "{c} metadata missing (index corrupted)"),
            Self::CidMismatch { expected, computed } => write!(
                f,
                "CAS content corruption: expected {expected}, computed {computed}"
            ),
            Self::IoError(e) => write!(f, "cas index I/O error: {e}"),
            Self::IndexParse { line, error } => {
                write!(f, "cas index parse error at line {line}: {error}")
            }
        }
    }
}

impl std::error::Error for CasError {}

impl From<git2::Error> for CasError {
    fn from(e: git2::Error) -> Self {
        Self::Git2(e)
    }
}

impl From<io::Error> for CasError {
    fn from(e: io::Error) -> Self {
        Self::IoError(e)
    }
}

fn cas_index_path(repo_path: &Path) -> PathBuf {
    repo_path.join(CAS_INDEX_FILENAME)
}

/// CO1.4-extra: read the sidecar JSONL into an in-memory index.
/// Strict mode — any malformed line aborts the load (per Art 0.2: a
/// corrupted index means the tape is non-canonical; abort + diagnose
/// is more honest than skip-and-warn).
fn load_index_from_sidecar(repo_path: &Path) -> Result<BTreeMap<Cid, CasObjectMetadata>, CasError> {
    let path = cas_index_path(repo_path);
    let mut index = BTreeMap::new();
    if !path.exists() {
        return Ok(index);
    }
    let content = std::fs::read_to_string(&path)?;
    for (i, line) in content.lines().enumerate() {
        if line.is_empty() {
            continue;
        }
        let meta: CasObjectMetadata =
            serde_json::from_str(line).map_err(|e| CasError::IndexParse {
                line: i + 1,
                error: e.to_string(),
            })?;
        index.insert(meta.cid, meta);
    }
    Ok(index)
}

/// CO1.4-extra: append a single JSONL line for a newly-created CAS object.
/// Followed by `sync_data` for durability (single-writer per cell per spec
/// § 5.2.2 — concurrent-writer atomicity is out of scope).
fn append_to_sidecar(repo_path: &Path, meta: &CasObjectMetadata) -> Result<(), CasError> {
    let path = cas_index_path(repo_path);
    let serialized = serde_json::to_string(meta).map_err(|e| CasError::IndexParse {
        line: 0,
        error: format!("serialize: {e}"),
    })?;
    let mut f = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)?;
    f.write_all(serialized.as_bytes())?;
    f.write_all(b"\n")?;
    f.sync_data()?;
    Ok(())
}

/// Content-addressable store backed by git's blob object database.
#[derive(Debug)]
pub struct CasStore {
    repo_path: PathBuf,
    /// Cid → metadata index. BTreeMap per spec § 2 I-BTREE.
    index: BTreeMap<Cid, CasObjectMetadata>,
}

impl CasStore {
    /// Open or initialize a CAS store at the given runtime_repo path.
    /// Creates the git repo if it doesn't exist. **CO1.4-extra**: replays
    /// the sidecar `.turingos_cas_index.jsonl` (if any) into the in-memory
    /// index, restoring all metadata that was durably appended in prior
    /// sessions.
    pub fn open(repo_path: &Path) -> Result<Self, CasError> {
        let repo_path = repo_path.to_path_buf();
        let _repo = match Repository::open(&repo_path) {
            Ok(r) => r,
            Err(_) => Repository::init(&repo_path)?,
        };
        let index = load_index_from_sidecar(&repo_path)?;
        Ok(Self { repo_path, index })
    }

    fn open_repo(&self) -> Result<Repository, CasError> {
        Repository::open(&self.repo_path).map_err(CasError::from)
    }

    /// Store content; returns its Cid. Idempotent — same content → same Cid.
    pub fn put(
        &mut self,
        content: &[u8],
        object_type: ObjectType,
        creator: &str,
        created_at_logical_t: u64,
        schema_id: Option<String>,
    ) -> Result<Cid, CasError> {
        let cid = Cid::from_content(content);
        let repo = self.open_repo()?;
        let git_oid = repo.blob(content)?;

        // If already in index, idempotent: just return Cid (content addressing
        // guarantees same content → same Cid → already present)
        if self.index.contains_key(&cid) {
            return Ok(cid);
        }

        let metadata = CasObjectMetadata {
            cid,
            backend_oid_hex: git_oid.to_string(),
            object_type,
            creator: creator.to_string(),
            created_at_logical_t,
            schema_id,
            size_bytes: content.len() as u64,
        };
        // CO1.4-extra: durably append BEFORE inserting into in-memory index
        // (so a crash mid-write leaves the runtime in a consistent state —
        // either the entry is durably recorded AND in-memory, or neither).
        append_to_sidecar(&self.repo_path, &metadata)?;
        self.index.insert(cid, metadata);
        Ok(cid)
    }

    /// Retrieve content by Cid. Verifies content sha256 matches Cid (corruption check).
    pub fn get(&self, cid: &Cid) -> Result<Vec<u8>, CasError> {
        let metadata = self
            .index
            .get(cid)
            .ok_or(CasError::CidNotFound(*cid))?;
        let repo = self.open_repo()?;
        let git_oid = git2::Oid::from_str(&metadata.backend_oid_hex)
            .map_err(CasError::Git2)?;
        let blob = repo.find_blob(git_oid)?;
        let content = blob.content().to_vec();

        // Verify content sha256 matches Cid (defense against corruption).
        let mut h = Sha256::new();
        h.update(&content);
        let computed = Cid(h.finalize().into());
        if &computed != cid {
            return Err(CasError::CidMismatch {
                expected: *cid,
                computed,
            });
        }

        Ok(content)
    }

    /// Get metadata only (no content fetch).
    pub fn metadata(&self, cid: &Cid) -> Option<&CasObjectMetadata> {
        self.index.get(cid)
    }

    pub fn len(&self) -> usize {
        self.index.len()
    }

    pub fn is_empty(&self) -> bool {
        self.index.is_empty()
    }

    /// Merkle root over all CAS object metadata; deterministic per BTreeMap order.
    pub fn merkle_root(&self) -> [u8; 32] {
        let mut h = Sha256::new();
        for (_cid, meta) in &self.index {
            h.update(meta.canonical_hash());
        }
        h.finalize().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn fresh_store() -> (TempDir, CasStore) {
        let tmp = TempDir::new().unwrap();
        let store = CasStore::open(tmp.path()).unwrap();
        (tmp, store)
    }

    #[test]
    fn put_get_round_trip_small() {
        let (_tmp, mut s) = fresh_store();
        let cid = s.put(b"hello world", ObjectType::ProposalPayload, "alice", 100, None).unwrap();
        let content = s.get(&cid).unwrap();
        assert_eq!(content, b"hello world");
    }

    #[test]
    fn put_get_round_trip_large() {
        let (_tmp, mut s) = fresh_store();
        let big = vec![0xab; 65536];
        let cid = s.put(&big, ObjectType::PredicateBytecode, "system", 0, Some("wasm".into())).unwrap();
        let content = s.get(&cid).unwrap();
        assert_eq!(content, big);
    }

    #[test]
    fn put_idempotent_same_content() {
        let (_tmp, mut s) = fresh_store();
        let cid_a = s.put(b"x", ObjectType::Generic, "alice", 1, None).unwrap();
        let cid_b = s.put(b"x", ObjectType::Generic, "bob", 2, None).unwrap();
        assert_eq!(cid_a, cid_b, "same content → same Cid");
        // Index size = 1 (idempotent)
        assert_eq!(s.len(), 1);
    }

    #[test]
    fn cid_is_content_address() {
        let (_tmp, mut s) = fresh_store();
        let cid = s.put(b"specific content", ObjectType::Generic, "system", 0, None).unwrap();
        // Cid is sha256 of content; verifiable independently
        let expected = Cid::from_content(b"specific content");
        assert_eq!(cid, expected);
    }

    #[test]
    fn get_nonexistent_returns_error() {
        let (_tmp, s) = fresh_store();
        let bogus = Cid([0u8; 32]);
        match s.get(&bogus) {
            Err(CasError::CidNotFound(c)) => assert_eq!(c, bogus),
            other => panic!("expected CidNotFound, got {other:?}"),
        }
    }

    #[test]
    fn metadata_recorded() {
        let (_tmp, mut s) = fresh_store();
        let cid = s.put(b"meta test", ObjectType::CounterexamplePayload, "carol", 250, Some("v1".into())).unwrap();
        let meta = s.metadata(&cid).unwrap();
        assert_eq!(meta.cid, cid);
        assert_eq!(meta.object_type, ObjectType::CounterexamplePayload);
        assert_eq!(meta.creator, "carol");
        assert_eq!(meta.created_at_logical_t, 250);
        assert_eq!(meta.schema_id.as_deref(), Some("v1"));
        assert_eq!(meta.size_bytes, 9);
    }

    #[test]
    fn merkle_root_deterministic_two_runs() {
        let (_tmp1, mut s1) = fresh_store();
        let (_tmp2, mut s2) = fresh_store();
        for content in [b"a".as_slice(), b"b".as_slice(), b"c".as_slice()] {
            s1.put(content, ObjectType::Generic, "system", 0, None).unwrap();
        }
        // Different insertion order
        for content in [b"c".as_slice(), b"b".as_slice(), b"a".as_slice()] {
            s2.put(content, ObjectType::Generic, "system", 0, None).unwrap();
        }
        assert_eq!(s1.merkle_root(), s2.merkle_root(),
            "BTreeMap-ordered: insertion order independent (I-DET)");
    }

    #[test]
    fn empty_store_root() {
        let (_tmp, s) = fresh_store();
        let r = s.merkle_root();
        let expected: [u8; 32] = Sha256::new().finalize().into();
        assert_eq!(r, expected, "empty store root = sha256(empty)");
    }

    #[test]
    fn cell_isolation_disjoint_cas() {
        // Per spec § 5.2.2 cross-cell isolation: separate runtime_repo paths
        // → completely disjoint CasStore instances.
        let (_tmp_a, mut store_a) = fresh_store();
        let (_tmp_b, mut store_b) = fresh_store();

        let cid_a = store_a.put(b"only in a", ObjectType::Generic, "agent_a", 100, None).unwrap();
        let cid_b = store_b.put(b"only in b", ObjectType::Generic, "agent_b", 100, None).unwrap();

        // Each store has its own object only
        assert!(store_a.get(&cid_a).is_ok(), "store_a has cid_a");
        assert!(store_a.get(&cid_b).is_err(), "store_a lacks cid_b (isolated)");
        assert!(store_b.get(&cid_b).is_ok(), "store_b has cid_b");
        assert!(store_b.get(&cid_a).is_err(), "store_b lacks cid_a (isolated)");
    }

    #[test]
    fn put_many_then_iterate_count() {
        let (_tmp, mut s) = fresh_store();
        for i in 0..50 {
            s.put(
                format!("content {i}").as_bytes(),
                ObjectType::ProposalPayload,
                "system",
                i as u64,
                None,
            )
            .unwrap();
        }
        assert_eq!(s.len(), 50);
        assert!(!s.is_empty());
    }

    // ── CO1.4-extra: sidecar JSONL persistence tests ─────────────────────────

    /// Cold-restart: reopen recovers all metadata; get() works post-reopen
    /// (closes the Art 0.2 cold-replay gate that CO1.7-impl A4 needs).
    #[test]
    fn reopen_recovers_index_and_get_works() {
        let tmp = TempDir::new().expect("tempdir");
        let cid_a;
        let cid_b;
        {
            let mut s = CasStore::open(tmp.path()).expect("open");
            cid_a = s
                .put(b"alpha", ObjectType::ProposalPayload, "alice", 1, None)
                .unwrap();
            cid_b = s
                .put(b"beta", ObjectType::CounterexamplePayload, "bob", 2, Some("s.v1".into()))
                .unwrap();
        }
        // Reopen: in-memory store is fresh; sidecar replay is the ONLY way
        // metadata survives.
        let s2 = CasStore::open(tmp.path()).expect("reopen");
        assert_eq!(s2.len(), 2);
        assert_eq!(s2.get(&cid_a).expect("get a"), b"alpha");
        assert_eq!(s2.get(&cid_b).expect("get b"), b"beta");

        let meta_b = s2.metadata(&cid_b).expect("metadata b");
        assert_eq!(meta_b.creator, "bob");
        assert_eq!(meta_b.created_at_logical_t, 2);
        assert_eq!(meta_b.schema_id.as_deref(), Some("s.v1"));
        assert_eq!(meta_b.object_type, ObjectType::CounterexamplePayload);
    }

    /// Idempotent put: same content twice → same Cid → only ONE sidecar line.
    #[test]
    fn idempotent_put_does_not_duplicate_sidecar_line() {
        let tmp = TempDir::new().expect("tempdir");
        let mut s = CasStore::open(tmp.path()).expect("open");
        let _ = s
            .put(b"content", ObjectType::Generic, "alice", 1, None)
            .unwrap();
        let _ = s
            .put(b"content", ObjectType::Generic, "alice", 1, None)
            .unwrap();
        let path = cas_index_path(tmp.path());
        let lines: Vec<&str> = std::fs::read_to_string(&path)
            .unwrap()
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| {
                // own the str via leak — cheap for test
                Box::leak(l.to_string().into_boxed_str()) as &str
            })
            .collect();
        assert_eq!(lines.len(), 1, "idempotent put should produce 1 sidecar line, got {}", lines.len());
    }

    /// Append-only: each NEW put adds exactly ONE line.
    #[test]
    fn each_new_put_appends_one_line() {
        let tmp = TempDir::new().expect("tempdir");
        let mut s = CasStore::open(tmp.path()).expect("open");
        for i in 0..5 {
            s.put(
                format!("c{i}").as_bytes(),
                ObjectType::Generic,
                "system",
                i,
                None,
            )
            .unwrap();
        }
        let path = cas_index_path(tmp.path());
        let line_count = std::fs::read_to_string(&path)
            .unwrap()
            .lines()
            .filter(|l| !l.is_empty())
            .count();
        assert_eq!(line_count, 5);
    }

    /// Corrupted JSONL → strict parse error with line number (not silent skip).
    #[test]
    fn corrupted_sidecar_line_returns_parse_error() {
        let tmp = TempDir::new().expect("tempdir");
        // Init repo + ONE valid put to get a known-good first line.
        {
            let mut s = CasStore::open(tmp.path()).expect("open");
            s.put(b"hello", ObjectType::Generic, "alice", 1, None).unwrap();
        }
        // Corrupt: append a malformed line.
        let path = cas_index_path(tmp.path());
        let mut f = OpenOptions::new().append(true).open(&path).unwrap();
        f.write_all(b"this is not valid json\n").unwrap();
        f.sync_data().unwrap();

        // Reopen MUST fail with a typed IndexParse error citing the line number.
        let err = CasStore::open(tmp.path()).unwrap_err();
        match err {
            CasError::IndexParse { line, .. } => {
                assert_eq!(line, 2, "expected line 2 to be flagged");
            }
            other => panic!("expected IndexParse, got {other:?}"),
        }
    }

    /// Empty / non-existent sidecar → opens fresh with empty index.
    #[test]
    fn missing_sidecar_opens_fresh() {
        let tmp = TempDir::new().expect("tempdir");
        let s = CasStore::open(tmp.path()).expect("open");
        assert_eq!(s.len(), 0);
        assert!(s.is_empty());
    }
}

```


---

Now give your INDEPENDENT round-1 audit. Cite spec § + line where possible.

codex
I’ll ground this against the repo files rather than only the pasted excerpts, mainly to verify line numbers and any local guidance that affects the audit standard.
exec
/bin/bash -lc 'git rev-parse --show-toplevel && git rev-parse --short HEAD && git status --short' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
/home/zephryj/projects/turingosv4
e2752e8
?? handover/audits/CODEX_CO1_8_ROUND1_AUDIT_2026-04-29.md

exec
/bin/bash -lc 'find .. -name AGENTS.md -print' in /home/zephryj/projects/turingosv4
exec
/bin/bash -lc "rg --files -g 'AGENTS.md' -g 'CLAUDE.md' -g 'LATEST.md' -g 'SPRINT_DEPENDENCY_GRAPH*' -g 'CO1_8_MATERIALIZER_v1_2026-04-29.md' -g 'STATE_TRANSITION_SPEC_v1_2026-04-27.md' -g 'TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
CLAUDE.md
handover/ai-direct/LATEST.md
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md
handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md
handover/whitepapers/TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md
handover/architect-insights/SPRINT_DEPENDENCY_GRAPH_v1_2026-04-27.md

 succeeded in 882ms:
../turingos/AGENTS.md
../omega_kepler_discovery/AGENTS.md
../omega/AGENTS.md
../turingosv2/AGENTS.md
../turingclaw/AGENTS.md

codex
I found no in-scope `AGENTS.md` under `turingosv4`; the audit should be governed by the prompt plus `CLAUDE.md`. There is already an untracked Codex audit file in `handover/audits/`, so I’ll avoid touching it and just return the audit here.
exec
/bin/bash -lc "nl -ba CLAUDE.md | sed -n '1,220p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	# TuringOS v4
     2	
     3	## What
     4	Silicon-Native Microkernel for LLM Formal Verification Swarm.
     5	Rust 2021, tokio, serde_json. Mission: MiniF2F Lean 4.
     6	
     7	## Why
     8	- 唯一对齐文档: `constitution.md` (反奥利奥架构)
     9	- 压缩即智能: 抽象原则进宪法 / 具体情境进 `cases/`
    10	- 机制 > 参数 > 提示 (Art. V + C-021/C-031/C-034/C-043)
    11	
    12	## Code Standard (Art. I.1 + C-004 + C-027)
    13	- `cargo check` / `cargo test` 必过；`.env` 永不 commit
    14	- `src/{kernel,bus}.rs` + `src/sdk/tools/wallet.rs` 改动走 STEP_B_PROTOCOL（不直接编辑 main）
    15	- 任何影响行为的参数必须 env/config 可覆盖，不可硬编码
    16	
    17	## Audit Standard (Art. V.1 + C-010 + C-023 + C-035)
    18	- Generator ≠ Evaluator：代码作者不可是唯一审计者
    19	- 所有 merge / phase 决策双外审（Codex + Gemini）；VETO > CHALLENGE > PASS
    20	- 宪法违规立即 BLOCKER，不可延期、不可"可接受"
    21	
    22	## Report Standard (Art. I.2 + Art. II.2.1 + Art. IV 强制, C-052 + C-053 + C-057 + C-059 + C-061)
    23	- **主指标**（每报必填）: ΣPPUT + Mean PPUT (solved) + 95% CI (Wilson)
    24	- Art. I.2 三大统计信号不可缺: **信誉** (reputation_distribution p50/p90/max) + 效用 (PPUT) + 共识 (如适用)
    25	- Art. IV 终态区分: `halt_reason_distribution` {OmegaAccepted, MaxTxExhausted, WallClockCap, ComputeCapViolated, ErrorHalt}
    26	- 多 agent (n≥2) 专用: `parent_selection_entropy` + `pairwise_payload_diversity_mean`；任一 < 0.25 = Art. II.2.1 告警
    27	- solve count 不可独立陈述，必须配对 PPUT；以 solve count 起头 = 违宪
    28	
    29	## Reproducibility Standard (Art. I + C-012/C-016/C-032/C-039)
    30	- OMEGA accept 必留 self-contained artifact (`proofs/*.lean` + `gp_payload`)
    31	- 度量工具上线即冻结；Oracle 参数冻结；实验禁混 Oracle 模式
    32	- 中间件若修改数学内容 → 是 ArchitectAI 贡献，不是 swarm 涌现（C-023）
    33	
    34	## Alignment Standard (Art. IV + C-069)
    35	- 权威对齐文件: `handover/alignment/TRACE_MATRIX_v0_2026-04-22.md`
    36	  (后续 rev: `TRACE_MATRIX_vN.md`)
    37	- 每个 src/ pub 符号必须映射到宪法 flowchart 元素、标 orphan+justification、
    38	  或 BLOCK merge。doc-comment backlink 格式: `/// TRACE_MATRIX <FC-id>: <role>`
    39	- Conformance tests: `tests/fc_alignment_conformance.rs` — 每个 ✅ 行 ≥1
    40	  witness test；`#[ignore]` stub 覆盖 📅 deferred rows
    41	- 宪法 flowchart 修改仅 human architect 可触发，需重跑 Phase Z′ 6-stage
    42	- constitution.md hygiene 观察登记到 `handover/alignment/OBS_*.md`，不改宪法
    43	
    44	## Common Law (宪法 + 判例)
    45	宪法高度压缩，具体裁决查 `cases/C-xxx.yaml` (facts → ruling → precedent)
    46	- 按条款查: `grep -l "Art. I.1" cases/*.yaml`
    47	- 映射：`cases/V3_LESSONS.md` (50 v3 教训 → 现行判例)
    48	- 编号跳号：C-038 / C-042 为 reserved（见 C-041/C-043 预引用）
    49	
    50	## Docs (按需加载)
    51	| 文档 | 何时加载 |
    52	|------|---------|
    53	| `docs/architecture.md` | 修改 src/ 核心模块时 |
    54	| `docs/economics.md` | 修改经济引擎 (wallet/market) 时 |
    55	| `docs/hardware.md` | SSH/部署/远程操作时 |
    56	| `docs/experiments.md` | 创建或运行实验时 |
    57	| `docs/rules.md` | 触发规则或修改规则时 |
    58	
    59	## User
    60	独狼研究员, 零编程基础 vibe coder. 中文为主, 技术术语英文可.

exec
/bin/bash -lc "nl -ba handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md | sed -n '1,380p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	# CO1.8: L5 Materialized State + Agent Read View v1 ⏳ pre-audit (round-1 pending)
     2	
     3	**Status**: v1 (2026-04-29; **PENDING round-1 dual external audit** per CLAUDE.md "Audit Standard"). Greenfield atom — `src/bottom_white/materializer/` does not yet exist (verified). Wave 6 #2 per LATEST.md. Determinate next atom post-CO1.7-extra closure (4a978f0).
     4	
     5	**Author**: ArchitectAI (Claude); session 2026-04-29.
     6	
     7	**Companion specs (frozen, read first)**:
     8	- `CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md` v1.2 — round-3 PASS/PASS; freezes `LedgerWriter` trait + Sequencer 9-stage `apply_one`.
     9	- `CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md` v1.2.2 — round-4 PASS/PASS + Branch B closed; freezes `q.head_t` post-commit binding via `advance_head_t` + `LedgerWriter::head_commit_oid_hex`.
    10	- `CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md` — frozen 7-variant TypedTx ABI; CO1.8 consumes `TypedTx` exclusively (no transition-body internals).
    11	- `STATE_TRANSITION_SPEC_v1_2026-04-27.md` v1.4 — references `materializer::apply(&q.state_root_t, tx)` at 7 invocation sites (lines 399, 466, 560, 624, 700, 758, 852); CO1.8 ships the function those sites call.
    12	- `TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md` § 5.L5 (lines 392-413) — primary source: enumerates 6 sub-modules + `read_tool` minimal-context.
    13	
    14	**Single sentence**: ship the L5 `bottom_white::materializer` module as a substrate-independent SHA-256 state-projection layer — deterministic `apply(state_root, tx) → new_state_root` + 6 sub-indices (per WP § 5.L5) + visibility-filtered `agent_view::project_for_agent` — leaving sub-index *content* schema to per-sub-atom audit and leaving runtime integration with Sequencer stage 4-7 to a later wiring atom that depends on CO1.7.5 transition bodies.
    15	
    16	---
    17	
    18	## § 0 Scope decision
    19	
    20	### 0.1 Why this atom exists
    21	
    22	Per `SPRINT_DEPENDENCY_GRAPH_v1_2026-04-27.md` line 109-111: `[CO1.8] Materialized State (8 atoms) | blockedBy: CO1.7 | blocks: CO1.9`. CO1.7 family (transition_ledger spec + impl + extra) is ✅ closed (Branch A `5ce01b1` + Branch B closure 4a978f0). CO1.8 is therefore the unique unblocked next L-layer atom in the ChainTape vertical. Per LATEST.md line 156: "**Then** Wave 6 #2/#3 unblocks (CO1.8 L5 materializer + CO1.9 L6 signal indices)".
    23	
    24	Per WP § 5.L5 (lines 392-413), L5 is the **agent read surface**: `read_tool(agent_i, task_j, Q_t) → minimal sufficient context`. Agents do NOT read L4 transition ledger directly; they read L5 indices. L5 is therefore the gate between ground-truth ledger (L4) and agent-facing context (top management δ in the constitution mermaid).
    25	
    26	### 0.2 What this atom inherits (frozen)
    27	
    28	| Frozen by | Surface CO1.8 consumes |
    29	|---|---|
    30	| CO1.1.4-pre1 | 7-variant `TypedTx` ABI; CO1.8 dispatches on `TxKind` discriminant only — no per-variant body interpretation |
    31	| CO1.7-impl | `LedgerEntry` 9-field signing surface; CO1.8 reads `tx_payload_cid` to fetch tx bodies via L3 CAS |
    32	| CO1.7-extra | `q.head_t` post-commit binding via `advance_head_t` — CO1.8 reads `head_t` for replay-from-genesis tests |
    33	| q_state.rs | `pub struct Hash(pub [u8; 32])` SHA-256 generic root + `Hash::ZERO` genesis sentinel + `q.state_root_t: Hash` field |
    34	| L3 CAS (CO1.4 + CO1.4-extra) | `CasStore::put`/`get` round-trip + sidecar JSONL persistence — CO1.8 stores serialized state snapshots in CAS, indexed by state_root |
    35	
    36	### 0.3 What this atom delivers (new)
    37	
    38	8 sub-atoms (decomposed from whitepaper § 5.L5's 7 named modules + the `apply()` function):
    39	
    40	| Sub-atom | Module / fn | LoC est | Substrate-independent? |
    41	|---|---|---|---|
    42	| CO1.8.1 | `bottom_white::materializer::mod.rs` skeleton + `pub use` re-exports + `State` / `StateDelta` types + `MaterializerError` enum | ~80 | ✅ |
    43	| CO1.8.2 | `materializer::apply(prior_root: &Hash, tx: &TypedTx) -> Result<Hash, MaterializerError>` — pure, deterministic, dispatches on `TxKind` | ~120 | ✅ |
    44	| CO1.8.3 | `materializer::state_db::CurrentStateDb` — per-key state cells (BTreeMap-backed in v1; Path B git-tree migration deferred to CO1.8-extra) | ~100 | ✅ |
    45	| CO1.8.4 | `materializer::indices::task_index` — TaskMarket task cells (CO P2.1 surface placeholder; v1 ships read API only) | ~80 | ✅ |
    46	| CO1.8.5 | `materializer::indices::agent_reputation_index` — per-agent reputation counter (`AgentId → u64`); incremented by Verify/Challenge txs (per WP § 5.L5 line 399) | ~70 | ✅ |
    47	| CO1.8.6 | `materializer::indices::{error_taxonomy_index, price_signal_index}` — rejection-class histogram (`ErrorClass → u64`) + market-price-signal cache (`MarketId → PriceSignal`) | ~90 | ✅ |
    48	| CO1.8.7 | `materializer::agent_view::project_for_agent(agent_id: AgentId, q: &QState) -> AgentReadView` — visibility-filtered minimal-context; consumes L1 PredicateRegistry visibility tags (CO1.5 surface) | ~120 | ⚠️ depends on CO1.5 PredicateRegistry visibility tags being shipped (currently § 3.2 unblocked but not started; v1 ships interface + stub returning full view if PredicateRegistry not present) |
    49	| CO1.8.8 | substrate-independent test plan — 5 tests: `apply_determinism` + `apply_genesis_to_first_state` + `agent_reputation_increments_on_verify_tx` + `agent_view_filters_internals` + `state_root_reproducibility_across_restart` | ~180 | ✅ |
    50	
    51	**Total v1 estimate**: ~840 LoC across 7 production files + 1 test file (or 5 test files flat-named per CO1.7-extra MF5 convention).
    52	
    53	### 0.4 Out of scope (deferred per Anti-Oreo three-layer boundary)
    54	
    55	1. **Per-variant transition body integration** — Sequencer stage 4-7 currently call `materializer::apply` against `Err(NotYetImplemented)` transition stubs. Wiring CO1.8 into the live transition path is gated on **future CO1.7.5** (per CO1.7-extra spec § 0.1; gated on CO P2.x substrate). v1 ships the materializer as standalone library; integration is a separate atom.
    56	2. **Path B git-tree state_db backend** — v1 uses BTreeMap-backed in-memory state_db. Migration to git-tree-as-storage substrate is a follow-up CO1.8-extra (likely after TFR S2-S3 git substrate work). This preserves the "Hash = sha256 of serialized state" semantics; only the storage layer changes.
    57	3. **L6 derivable indices** — `reputation_counters` (windowed delta), `price_signals` (market microstructure compression), `failure_histogram` belong to CO1.9 L6 per WP § 5.L5 line 427. CO1.8 ships only L5 absolute-state indices; L6 derived statistics are a separate atom.
    58	4. **CO1.5 PredicateRegistry visibility tags** — CO1.8.7 `project_for_agent` interface lands; tag-driven filtering is no-op (returns full view) until CO1.5 ships visibility tags. Documented as known gap; not blocking.
    59	5. **Materializer integration with WAL** — current `Bus.wal` is the legacy in-memory ledger surface. Materializer state recovery on cold-start uses L4 transition ledger replay (`Git2LedgerWriter`) only; no WAL coupling.
    60	
    61	---
    62	
    63	## § 1 Module structure
    64	
    65	```
    66	src/bottom_white/materializer/
    67	├── mod.rs                    # CO1.8.1 — pub use re-exports + State, StateDelta, MaterializerError
    68	├── apply.rs                  # CO1.8.2 — apply(prior_root, tx) -> new_root
    69	├── state_db.rs               # CO1.8.3 — CurrentStateDb (BTreeMap-backed v1)
    70	├── indices/
    71	│   ├── mod.rs                # re-export
    72	│   ├── task_index.rs         # CO1.8.4 — TaskMarket task cells
    73	│   ├── agent_reputation.rs   # CO1.8.5 — per-agent reputation counter
    74	│   ├── error_taxonomy.rs     # CO1.8.6a — ErrorClass histogram
    75	│   └── price_signal.rs       # CO1.8.6b — MarketId → PriceSignal cache
    76	└── agent_view.rs             # CO1.8.7 — project_for_agent(agent_id, q_t) -> AgentReadView
    77	```
    78	
    79	`pub use` chain at `mod.rs` flattens to `crate::bottom_white::materializer::{apply, State, StateDelta, MaterializerError, project_for_agent}` — agent_view is the only sub-module needing a top-level re-export per WP § 5.L5 line 408 read_tool signature.
    80	
    81	---
    82	
    83	## § 2 `apply()` interface contract
    84	
    85	### 2.1 Signature
    86	
    87	```rust
    88	/// TRACE_MATRIX § 5.L5 — L5 materialize step (called by Sequencer stage 4-7
    89	/// of all transition bodies per STATE_TRANSITION_SPEC v1.4 lines 399, 466,
    90	/// 560, 624, 700, 758, 852).
    91	///
    92	/// Pure function: deterministic mapping `(prior_root, tx) → new_root`.
    93	/// No I/O; no side effects on outer state. The materialized snapshot is
    94	/// reconstructed in-memory from the prior_root + tx delta and re-hashed
    95	/// to produce the new root.
    96	///
    97	/// Returns `Err(MaterializerError::PriorRootNotFound)` if `prior_root` is
    98	/// not in the state_db cache (v1 pre-Path-B; CO1.8-extra git-tree migration
    99	/// removes this failure mode). For ZERO root (genesis), returns Ok(...).
   100	pub fn apply(
   101	    prior_root: &Hash,
   102	    tx: &TypedTx,
   103	) -> Result<Hash, MaterializerError>;
   104	```
   105	
   106	### 2.2 Invariants (audited at sub-atom level)
   107	
   108	| Invariant | Statement | Test |
   109	|---|---|---|
   110	| **I-DET** | `apply(r, tx) == apply(r, tx)` for all r, tx | CO1.8.8 `apply_determinism` |
   111	| **I-GEN** | `apply(Hash::ZERO, tx) == h` for some h ≠ ZERO when tx has non-empty effect | CO1.8.8 `apply_genesis_to_first_state` |
   112	| **I-PURE** | apply has no observable side effects: no logs, no mutations to global state, no I/O | static review (no `&mut self`, no `pub static`) |
   113	| **I-CACHE** | repeat apply(r, tx) does not consume O(t) memory; state_db caches by root | static review (BTreeMap caching) |
   114	
   115	### 2.3 Atomicity
   116	
   117	Materializer apply is invoked from Sequencer `apply_one` AFTER `writer_w.commit(&entry)?` returns Ok and BEFORE `*q_w = q_next` — i.e., between L4 ledger commit and Q_t state advance. v1 ships materializer as a pure function so no atomicity concerns at the materializer layer; the Sequencer's existing q_w lock guarantees serial invocation.
   118	
   119	---
   120	
   121	## § 3 Sub-atom decomposition (8 atoms — see § 0.3 table)
   122	
   123	Sub-atoms are NOT independently shippable in v1 — they form a single compile unit (mod.rs re-exports cascade). Decomposition exists for audit-readability and future-refactor predictability. Each sub-atom MAY get its own STEP_B-non-restricted commit during impl phase if size warrants (>300 LoC delta).
   124	
   125	---
   126	
   127	## § 4 Test plan (substrate-independent; round-2 MF5 flat-naming convention)
   128	
   129	5 tests, flat-named in `tests/`:
   130	
   131	### 4.1 `tests/co1_8_apply_determinism.rs`
   132	
   133	```rust
   134	#[test]
   135	fn apply_is_deterministic_across_calls() {
   136	    let prior = Hash::ZERO;
   137	    let tx = TypedTx::Work(canonical_work_tx_fixture());
   138	    let h1 = materializer::apply(&prior, &tx).expect("apply 1");
   139	    let h2 = materializer::apply(&prior, &tx).expect("apply 2");
   140	    assert_eq!(h1, h2);
   141	}
   142	```
   143	
   144	### 4.2 `tests/co1_8_apply_genesis_to_first_state.rs`
   145	
   146	```rust
   147	#[test]
   148	fn apply_advances_state_root_from_zero_on_first_tx() {
   149	    let tx = TypedTx::Work(canonical_work_tx_fixture());
   150	    let h = materializer::apply(&Hash::ZERO, &tx).expect("apply");
   151	    assert_ne!(h, Hash::ZERO, "first tx must advance state root from genesis");
   152	}
   153	```
   154	
   155	### 4.3 `tests/co1_8_agent_reputation_increments.rs`
   156	
   157	```rust
   158	#[test]
   159	fn verify_tx_increments_target_agent_reputation() {
   160	    let mut q = QState::genesis();
   161	    let verify_tx = canonical_verify_tx_fixture(/* target = */ "A0");
   162	    let new_root = materializer::apply(&q.state_root_t, &verify_tx).expect("apply");
   163	    let view = materializer::indices::agent_reputation::reputation_for(&new_root, "A0");
   164	    assert!(view.unwrap_or(0) > 0, "A0 reputation must increment after VerifyTx");
   165	}
   166	```
   167	
   168	### 4.4 `tests/co1_8_agent_view_filters_internals.rs`
   169	
   170	```rust
   171	#[test]
   172	fn project_for_agent_returns_minimal_context_only() {
   173	    let q = canonical_q_state_with_internals();
   174	    let view = materializer::project_for_agent("A0", &q);
   175	    assert!(view.contains_task_id("task_0"));
   176	    assert!(!view.contains_evaluator_internal_field("oracle_seed"),
   177	        "agent_view MUST NOT leak evaluator internals (Inv 10 Goodhart shield)");
   178	}
   179	```
   180	
   181	### 4.5 `tests/co1_8_state_root_reproducibility.rs`
   182	
   183	```rust
   184	#[test]
   185	fn state_root_is_reproducible_across_cold_restart() {
   186	    let txs = vec![/* 5 deterministic TypedTx fixtures */];
   187	    let h_first = txs.iter().fold(Hash::ZERO, |r, tx|
   188	        materializer::apply(&r, tx).expect("apply"));
   189	    // Drop in-memory state; reconstruct from txs alone.
   190	    let h_second = txs.iter().fold(Hash::ZERO, |r, tx|
   191	        materializer::apply(&r, tx).expect("apply (cold)"));
   192	    assert_eq!(h_first, h_second, "state_root reproducibility from L4 ledger replay");
   193	}
   194	```
   195	
   196	---
   197	
   198	## § 5 Open questions (audit-resolved)
   199	
   200	| Q | Statement | Author lean |
   201	|---|---|---|
   202	| Q1 | Does `state_root` literally equal a git tree object id (Path B), or is it sha256 of a serialized state snapshot whose storage *backend* is git tree? | Sha256 of canonical-serialized snapshot. q_state.rs:27 explicitly says "generic 32-byte hash (sha256)". The "git tree root" comment in STATE_TRANSITION_SPEC line 78 refers to where the snapshot LIVES (git tree object), not what state_root EQUALS. v1 ships sha256-of-snapshot semantics. |
   203	| Q2 | Is materializer::apply allowed to fail on `PriorRootNotFound`, or must v1 always succeed (e.g., by lazy snapshot reconstruction from L4 replay)? | v1 ships PriorRootNotFound failure mode (BTreeMap cache lookup). Lazy reconstruction is a CO1.8-extra concern (would require Sequencer reference for L4 replay; out of scope for pure-function v1). |
   204	| Q3 | Should sub-indices share a single state_db backing (one BTreeMap<Key, Value> with namespaced keys) or live in separate BTreeMaps with cross-references? | Single backing with namespaced keys: `("task", task_id)` / `("rep", agent_id)` / etc. Simpler audit; trivial migration to git-tree path-keying in CO1.8-extra. |
   205	| Q4 | What is the canonical serialization format for state snapshots fed into sha256? bincode (CO1.7-impl precedent)? Custom flat-buffer? | bincode v2 `BorrowDecode` per CO1.7-impl A1 precedent. Re-uses existing dependency; deterministic-encoding-by-construction. |
   206	| Q5 | Does `agent_view::project_for_agent` consume CO1.5 PredicateRegistry visibility tags, or does v1 ship a no-op filter (returns full view)? | v1 ships no-op filter with explicit TODO `pending CO1.5`. Documented as known gap in § 0.4 #4. Not blocking for Wave 6 #2. |
   207	
   208	---
   209	
   210	## § 6 Audit gates
   211	
   212	| Round | Codex | Gemini | Conservative | Action |
   213	|---|---|---|---|---|
   214	| 1 (this v1) | ⏳ pending | ⏳ pending | TBD | round-1 dual external audit on CO1.8 v1 |
   215	| 2+ | … | … | … | iterate to PASS/PASS |
   216	
   217	**Pre-implementation gate**: spec must reach PASS/PASS before any code in `src/bottom_white/materializer/**` is written. Per CLAUDE.md "Audit Standard". No STEP_B-restricted files touched (kernel.rs / bus.rs / wallet.rs UNTOUCHED by v1; only new files under `src/bottom_white/materializer/`).
   218	
   219	---
   220	
   221	## § 7 Estimated scope
   222	
   223	- **Spec rounds**: round-1 expected CHALLENGE/CHALLENGE (open Q1-Q5 absorb both audits); round-2 PASS-or-CHALLENGE; round-3 PASS/PASS likely. Round budget ~$10-15.
   224	- **Implementation scope** (post-PASS/PASS):
   225	  - 8 sub-atoms × ~80-180 LoC each = ~840 LoC production + ~180 LoC tests.
   226	  - Largest single file: `apply.rs` (~120 LoC; dispatches on TxKind discriminant).
   227	- **Total atom budget**: ~1020 LoC; **estimated calendar time**: 2-4 days impl + 1-2 days audit cycles. Cumulative project audit spend after CO1.8 PASS/PASS: ~$210-330 / $890 mid-budget.
   228	
   229	---
   230	
   231	## § 8 Honest acknowledgements
   232	
   233	1. **Greenfield atom**: `src/bottom_white/materializer/` does not exist. Verified via `ls src/bottom_white/` showing only `cas`, `ledger`, `mod.rs`, `tools`. v1 creates the module from primary sources (whitepaper § 5.L5 + STATE spec invocation surface + sprint graph dependency note).
   234	2. **8 sub-atoms is author-decomposed**, not pre-existing in any document. SPRINT_DEPENDENCY_GRAPH line 109 says "Materialized State (8 atoms)" but does not enumerate. The decomposition in § 0.3 is the author's best mapping of WP § 5.L5's 7 named modules + the `apply()` function (= 8). Audit may suggest re-decomposition.
   235	3. **CO1.5 PredicateRegistry visibility-tag dependency is acknowledged but stubbed** (§ 0.4 #4). Without CO1.5 visibility tags, `project_for_agent` is a no-op filter. This means CO1.8.7 ships an interface, not a fully-functional Inv-10 Goodhart shield. The gap is documented; CO1.5 + a follow-up CO1.8-extra closes the loop.
   236	4. **state_root semantics interpretation lean** (Q1) is author's reading of q_state.rs:27 ("generic 32-byte hash (sha256)") taking precedence over STATE_TRANSITION_SPEC line 78's "git tree root in Path B" gloss. If audit finds the gloss authoritative, v1 § 2 signature must change to integrate with git2-rs tree-builder API.
   237	5. **Path B git-tree backend deferred** to CO1.8-extra (§ 0.4 #2). v1 ships in-memory BTreeMap to keep the substrate-independence invariant (no git2-rs touch in v1). This means v1 alone does NOT cold-restart from disk; cold-restart-via-L4-replay test (CO1.8.8) only verifies determinism, not durability. Durability lands with CO1.8-extra.
   238	6. **No STEP_B-restricted file touches**. Kernel + bus + wallet untouched. v1 is pure-additive at `src/bottom_white/materializer/**`. No STEP_B parallel-branch ceremony required.
   239	7. **FC-trace requirements**: every new pub symbol in CO1.8 implementation must carry `/// TRACE_MATRIX § 5.L5: <role>` doc-comment per CLAUDE.md "Alignment Standard". Set: `apply` + `State` + `StateDelta` + `MaterializerError` + `CurrentStateDb` + `task_index` accessors + `agent_reputation::reputation_for` + `error_taxonomy` + `price_signal` + `project_for_agent`.
   240	
   241	---
   242	
   243	## § 9 Pre-audit smoke test plan
   244	
   245	Per memory `feedback_smoke_before_batch`. Smoke run before round-1 audit launch, at the v1 commit HEAD.
   246	
   247	| # | Claim | Smoke command | Pass criterion |
   248	|---|---|---|---|
   249	| S1 | `Hash` is `pub struct Hash(pub [u8; 32])` (sha256-sized) | `grep -A1 'pub struct Hash' src/state/q_state.rs` | matches |
   250	| S2 | `Hash::ZERO` exists as genesis sentinel | `grep -n 'pub const ZERO: Hash' src/state/q_state.rs` | one hit |
   251	| S3 | `q.state_root_t: Hash` field present | `grep -n 'pub state_root_t' src/state/q_state.rs` | one hit |
   252	| S4 | `materializer::apply` is invoked at 7 sites in STATE spec | `grep -c 'materializer::apply' handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md` | 7 |
   253	| S5 | `bottom_white::materializer` does NOT exist (greenfield) | `ls src/bottom_white/materializer/ 2>&1` | "No such file or directory" |
   254	| S6 | `TypedTx` 7-variant ABI frozen and present | `grep -n 'pub enum TypedTx' src/state/typed_tx.rs && grep -c 'TxKind::' src/state/typed_tx.rs` | one TypedTx hit; ≥7 TxKind:: hits |
   255	| S7 | CO1.5 PredicateRegistry status | check § 3.2 of decision map for CO1.5 line | confirm "🟢 ready but not started" or equivalent (justifies § 0.4 #4 stub) |
   256	| S8 | CAS surface available for snapshot storage | `grep -n 'pub fn put\|pub fn get' src/bottom_white/cas/store.rs` | both present |
   257	| S9 | bincode v2 dependency available | `grep -n 'bincode' Cargo.toml` | dep present |
   258	| S10 | cargo baseline | `cargo check --workspace && cargo test --workspace --lib` | clean compile + 239/0/1 (matches post-CO1.7-extra-impl baseline at HEAD `4a978f0`) |
   259	
   260	---
   261	
   262	**END v1 DRAFT body.**
   263	
   264	## Pre-audit smoke results
   265	
   266	### Round-1 smoke (HEAD `4a978f0`; v1)
   267	
   268	| # | Claim | Result | Status |
   269	|---|---|---|---|
   270	| S1 | Hash type | `pub struct Hash(pub [u8; 32])` (q_state.rs:29) | ✅ PASS |
   271	| S2 | Hash::ZERO | `pub const ZERO: Hash = Hash([0u8; 32])` (q_state.rs:33) | ✅ PASS |
   272	| S3 | q.state_root_t | `pub state_root_t: Hash` (q_state.rs:313) | ✅ PASS |
   273	| S4 | materializer::apply invocations in STATE spec | 7 hits (lines 399/466/560/624/700/758/852) | ✅ PASS |
   274	| S5 | greenfield | `ls: cannot access 'src/bottom_white/materializer/': No such file or directory` | ✅ PASS |
   275	| S6 | TypedTx ABI | `pub enum TypedTx` at line 608; 14 TxKind:: hits | ✅ PASS |
   276	| S7 | CO1.5 status | Wave 2 sub-choice B; "CO P2.1 uses CO1.5 visibility" — CO1.5 still in-flight per § 3.2 | ✅ PASS (justifies § 0.4 #4 stub) |
   277	| S8 | CAS surface | `pub fn put` line 163; `pub fn get` line 199 | ✅ PASS |
   278	| S9 | bincode v2 | `bincode = { version = "2", features = ["serde"] }` (Cargo.toml:9) | ✅ PASS |
   279	| S10 | cargo baseline | check clean (warnings pre-existing); test 239/0/1 ignored (sequencer_serial_replay_byte_identity, deferred to future CO1.7.5) | ✅ PASS |
   280	
   281	**Smoke gate v1**: 10/10 PASS at HEAD `4a978f0`. Spec v1 ready for round-1 dual external audit.
   282	
   283	## Patch log
   284	
   285	**v1 (2026-04-29; greenfield draft)** — initial spec draft from primary sources:
   286	- Whitepaper § 5.L5 lines 392-413 (6 sub-modules + read_tool minimal-context)
   287	- STATE_TRANSITION_SPEC v1.4 lines 399/466/560/624/700/758/852 (7 `materializer::apply` invocation sites)
   288	- SPRINT_DEPENDENCY_GRAPH v1 line 109 ("Materialized State (8 atoms)")
   289	- TRACE_MATRIX_v3 row § 5.L5 (module path `bottom_white::materializer::{state_db, indices, agent_view}`)
   290	- q_state.rs:27-49 (Hash type + state_root_t field)
   291	- CO1.7-extra v1.2.2 + CO1.7-impl bundle (frozen interfaces consumed)
   292	
   293	8 sub-atoms (CO1.8.1-CO1.8.8) decomposed by author from WP § 5.L5's 7 named modules + the `apply()` function. 5 substrate-independent tests (flat-named per CO1.7-extra MF5 convention). 5 open questions for round-1 audit (Q1 state_root semantics being the most consequential).
   294	
   295	### Awaiting
   296	
   297	1. ⏳ pre-audit smoke run at v1 commit HEAD (S1-S10 from § 9)
   298	2. ⏳ round-1 dual external audit (Codex + Gemini per CLAUDE.md "Audit Standard"; conservative VETO>CHALLENGE>PASS per memory `feedback_dual_audit_conflict`)
   299	3. ⏳ iterate v1.x patches per audit findings until PASS/PASS
   300	4. ⏳ implementation start gated on PASS/PASS spec

exec
/bin/bash -lc "nl -ba handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md | sed -n '60,90p;385,410p;455,475p;548,568p;614,632p;690,706p;748,765p;842,858p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
    60	- AttributionEngine DAG construction algorithm — deferred to CO2.4.0 spike (Inv 8 design)
    61	- Full predicate visibility air-gap proof — deferred to CO P1.5 (Goodhart shield design)
    62	
    63	---
    64	
    65	## § 1 Typed Schemas
    66	
    67	### 1.1 QState (white paper § 4 + economic § 2 amendment, 9 fields)
    68	
    69	```rust
    70	pub struct QState {
    71	    /// Agent swarm sub-state: tape head per agent, per-agent reputation snapshots, etc.
    72	    /// MUST be reconstructible from L4 transition ledger replay.
    73	    pub q_t: AgentSwarmState,
    74	
    75	    /// Current ChainTape head pointer = git commit SHA in Path B substrate.
    76	    pub head_t: NodeId,
    77	
    78	    /// Materialized state Merkle root (git tree root in Path B).
    79	    pub state_root_t: Hash,
    80	
    81	    /// Agent-visible projection of tape filtered by per-agent visibility policy
    82	    /// (Inv 10 Goodhart shield). Derived from L1 PredicateRegistry visibility tags.
    83	    pub tape_view_t: AgentVisibleProjection,
    84	
    85	    /// L4 Transition Ledger root (Merkle root of all accepted tx so far).
    86	    pub ledger_root_t: Hash,
    87	
    88	    /// L1 Predicate Registry root.
    89	    pub predicate_registry_root_t: Hash,
    90	
   385	    let mut q_next = q.clone();
   386	    q_next.economic_state_t.claims_t.insert(claim, provisional_reward);
   387	    q_next.economic_state_t.stakes_t.lock(tx.agent_id, tx.task_id, tx.stake);
   388	    q_next.economic_state_t.balances_t.debit(tx.agent_id, tx.stake);
   389	    q_next.q_t.update_per_agent(tx.agent_id, |s| {
   390	        s.last_accepted_tx = Some(tx.tx_id);
   391	        s.retry_counter_for_current_task = 0;  // reset on accept
   392	    });
   393	
   394	    // L4 append
   395	    let new_ledger_root = ledger::append(&q.ledger_root_t, tx);
   396	    q_next.ledger_root_t = new_ledger_root;
   397	
   398	    // L5 materialize
   399	    let new_state_root = materializer::apply(&q.state_root_t, tx);
   400	    q_next.state_root_t = new_state_root;
   401	
   402	    // L6 signal emit (broadcast price + reputation; NOT evaluator internals — Inv 10)
   403	    let signals = SignalBundle {
   404	        boolean: vec![Signal::Boolean(BoolSignal::AcceptedAt(tx.tx_id))],
   405	        statistical: vec![
   406	            Signal::Statistical(StatSignal::PriceUpdate(price_for(tx.task_id, q_next.economic_state_t.price_index_t))),
   407	            Signal::Statistical(StatSignal::ReputationDelta(tx.agent_id, +reputation_delta(tx))),
   408	        ],
   409	    };
   410	
   455	        return Err(TransitionError::VerificationPredicateFailed(verify_results));
   456	    }
   457	
   458	    // STAGE 4: state transition
   459	    let mut q_next = q.clone();
   460	    q_next.economic_state_t.balances_t.debit(tx.verifier_agent, tx.bond);
   461	    q_next.economic_state_t.stakes_t.lock_verifier_bond(tx.verifier_agent, tx.target_work_tx, tx.bond);
   462	    q_next.economic_state_t.claims_t.add_verification(tx.target_work_tx, tx.verifier_agent, tx.verdict);
   463	
   464	    // STAGE 5: append + materialize + signals
   465	    q_next.ledger_root_t = ledger::append(&q.ledger_root_t, tx);
   466	    q_next.state_root_t  = materializer::apply(&q.state_root_t, tx);
   467	    q_next.head_t        = NodeId::from_state_root(q_next.state_root_t);
   468	
   469	    let signals = SignalBundle {
   470	        boolean: vec![Signal::Boolean(BoolSignal::VerifiedAt(tx.tx_id))],
   471	        statistical: vec![Signal::Statistical(StatSignal::ReputationDelta(tx.verifier_agent, +verify_reputation_delta(tx, target)))],
   472	    };
   473	
   474	    Ok((q_next, signals))
   475	}
   548	                q_next.economic_state_t.balances_t.credit(tx.challenger_agent, bond);
   549	                q_next.economic_state_t.stakes_t.slash_verifier_bond(verifier, tx.target_work_tx);
   550	                q_next.economic_state_t.reputations_t.adjust(verifier, -verifier_slash_delta());
   551	            }
   552	        }
   553	    }
   554	
   555	    // STAGE 5: close challenge window
   556	    q_next.economic_state_t.challenge_cases_t.close(tx.target_work_tx, ChallengeOutcome::Slashed(tx.tx_id));
   557	
   558	    // STAGE 6: append + materialize + signals
   559	    q_next.ledger_root_t = ledger::append(&q.ledger_root_t, tx);
   560	    q_next.state_root_t  = materializer::apply(&q.state_root_t, tx);
   561	    q_next.head_t        = NodeId::from_state_root(q_next.state_root_t);
   562	
   563	    let signals = SignalBundle {
   564	        boolean: vec![Signal::Boolean(BoolSignal::ChallengeUpheld(tx.tx_id))],
   565	        statistical: vec![
   566	            Signal::Statistical(StatSignal::ReputationDelta(target.solver, -slash_reputation_delta())),
   567	            Signal::Statistical(StatSignal::ReputationDelta(tx.challenger_agent, +challenge_reputation_delta())),
   568	        ],
   614	    let mut q_next = q.clone();
   615	    q_next.economic_state_t.royalty_graph_t.add_edge(
   616	        from: tx.reusing_work_tx,
   617	        to:   tx.reused_tool_id,
   618	        creator: tx.reused_tool_creator,
   619	        weight: bounded_weight,    // clamped per gap 11.3
   620	    );
   621	
   622	    // STAGE 4: append + materialize (no signals; royalty paid at finalize_reward time)
   623	    q_next.ledger_root_t = ledger::append(&q.ledger_root_t, tx);
   624	    q_next.state_root_t  = materializer::apply(&q.state_root_t, tx);
   625	    q_next.head_t        = NodeId::from_state_root(q_next.state_root_t);
   626	
   627	    Ok((q_next, SignalBundle::empty()))
   628	}
   629	```
   630	
   631	### 3.4 finalize_reward (challenge window expiry)
   632	
   690	            / 1_000_000;    // integer floor; deterministic across platforms
   691	        let royalty = MicroCoin::from_micro_units(royalty_micro);
   692	        q_next.economic_state_t.balances_t.credit(edge.creator, royalty);
   693	        q_next.economic_state_t.balances_t.debit(target.solver, royalty);  // royalty comes from solver's reward, not extra mint (Inv 4)
   694	    }
   695	    // Note: integer floor means total royalty payments may be < `reward × Σ weights` by up to `n` micro-units (1 per edge);
   696	    // the dust remains in solver's balance. This is intentional and consistent with Bitcoin satoshi rounding.
   697	
   698	    // STAGE 4: emit terminal signals
   699	    q_next.ledger_root_t = ledger::append(&q.ledger_root_t, &FinalizeTx::from(claim_id, reward));
   700	    q_next.state_root_t  = materializer::apply(&q.state_root_t, &FinalizeTx::from(claim_id, reward));
   701	    q_next.head_t        = NodeId::from_state_root(q_next.state_root_t);
   702	
   703	    Ok((q_next, SignalBundle::finalize(claim_id, reward)))
   704	}
   705	```
   706	
   748	    for (agent, locked_stake) in q.economic_state_t.stakes_t.all_locked_for_task(tx.task_id) {
   749	        q_next.economic_state_t.stakes_t.unlock(agent, tx.task_id);
   750	        q_next.economic_state_t.balances_t.credit(agent, locked_stake);
   751	    }
   752	
   753	    // STAGE 6: remove task from active markets
   754	    q_next.economic_state_t.task_markets_t.remove(tx.task_id);
   755	
   756	    // STAGE 7: append + materialize + signal (purely on tx, q)
   757	    q_next.ledger_root_t = ledger::append(&q.ledger_root_t, tx);
   758	    q_next.state_root_t  = materializer::apply(&q.state_root_t, tx);
   759	    q_next.head_t        = NodeId::from_state_root(q_next.state_root_t);
   760	
   761	    let signals = SignalBundle::task_expired(tx.task_id, bounty);
   762	
   763	    Ok((q_next, signals))
   764	}
   765	
   842	        run_outcome: run.outcome(),
   843	        total_attempts: run.attempt_counter(),
   844	        failure_class_histogram: run.failure_histogram(),
   845	        last_logical_t: run.last_logical_t(),
   846	        system_signature: runtime.system_keypair().sign(canonical_digest_terminal(run)),
   847	    };
   848	
   849	    // STAGE: append; materialize; emit failure-class signals to L6
   850	    let mut q_next = q.clone();
   851	    q_next.ledger_root_t = ledger::append(&q.ledger_root_t, &summary);
   852	    q_next.state_root_t  = materializer::apply(&q.state_root_t, &summary);
   853	    q_next.head_t        = NodeId::from_state_root(q_next.state_root_t);
   854	
   855	    let signals = SignalBundle::terminal_summary(&summary);
   856	
   857	    Ok((q_next, signals))
   858	}

exec
/bin/bash -lc "nl -ba handover/whitepapers/TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md | sed -n '388,416p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   388	- Git commit graph
   389	- permissioned blockchain
   390	- public-chain settlement proof
   391	
   392	#### Layer 5：Materialized State and Agent Read View
   393	
   394	物化状态层。
   395	
   396	```
   397	current_state_db
   398	task_index
   399	agent_reputation_index
   400	error_taxonomy_index
   401	price_signal_index
   402	permission_view
   403	```
   404	
   405	Agent 实际读取的是这一层，而不是底层完整账本。
   406	
   407	```
   408	read_tool(agent_i, task_j, Q_t)
   409	  -> minimal sufficient context
   410	```
   411	
   412	这正是选择性屏蔽与渐进式披露。
   413	
   414	#### Layer 6：Signal Indices
   415	
   416	信号索引层。

codex
The first concrete issue is already visible: the spec’s claimed `Result<Hash, MaterializerError>` interface does not match the STATE pseudocode assignment sites, and the terminal summary call is not a `TypedTx` in the source spec. I’m checking the surrounding frozen ABI and dependency graph before deciding whether that is a challenge or a veto.
exec
/bin/bash -lc "nl -ba handover/architect-insights/SPRINT_DEPENDENCY_GRAPH_v1_2026-04-27.md | sed -n '100,116p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   100	[CO1.1.6]     layer-leak conformance test
   101	    └─ blockedBy: CO1.1.4, CO1.1.5
   102	    └─ blocks: nothing
   103	[CO1.2]       Q_t struct (3 atoms)
   104	    └─ blockedBy: CO1.1.5 (kernel split provides typed home)
   105	    └─ blocks: CO1.7.5 (step_transition uses QState)
   106	[CO1.7.5]     step_transition fn impl (THE main transition fn)
   107	    └─ blockedBy: CO1.7.1 (TransitionTx schema), CO1.5, CO1.6, CO1.4, CO1.2, CO1.7.0c (sign API)
   108	    └─ blocks: CO1.8, CO1.9, all CO P2
   109	[CO1.8]       Materialized State (8 atoms)
   110	    └─ blockedBy: CO1.7
   111	    └─ blocks: CO1.9 (signal indices need materialized view)
   112	[CO1.9]       Signal Indices (7 atoms)
   113	    └─ blockedBy: CO1.8
   114	    └─ blocks: CO1.10 (boolean vs statistical signals)
   115	[CO1.9.5]     derive_l6_from_tape (failure histogram)
   116	    └─ blockedBy: CO1.7 (retry metadata schema)

exec
/bin/bash -lc "nl -ba handover/ai-direct/LATEST.md | sed -n '145,165p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   145	User explicit instruction 2026-04-29 session-2:
   146	> "我要求你在遵守宪法、白皮书和我们刚才讨论的elon-mode下自动执行..."
   147	
   148	Operationalized priority order:
   149	1. Constitution
   150	2. Whitepaper v2
   151	3. Elon-mode (round cap=2, OBS threshold=3, cap-exception via auto-execute on determinate-best surgical patch)
   152	4. Standing memories (dual-audit, smoke-before-batch, no-fake-menus, FC-first, NEW lake-packages-vendored)
   153	
   154	When facing decision: 1→2→3→4 order; if no resolution → state determinate-best + execute (no fake menus). Per-phase drift review at atom-complete boundary. When lacking data: run real tests, don't speculate.
   155	
   156	---
   157	
   158	## 🌊 2026-04-29 Session-2 — CO1.7-extra Branch B closure + CO1.13 spec PASS-with-cap-exception (Elon-mode launch)
   159	
   160	**Updated**: 2026-04-29 (session-2)
   161	**Status**: spec phase **DONE** (CO1.7-extra ceremony closed + CO1.13 cleared for impl); implementation phase **READY TO START** in fresh session.
   162	
   163	### 🚀 Next-session entry point
   164	
   165	**Pick up at CO1.13 implementation phase per spec § 0.3 v1.1.1**. Three sub-atoms in dependency order:

exec
/bin/bash -lc 'rg -n "canonical_encode|bincode|standard|legacy|config" src/bottom_white/ledger src/state Cargo.toml' in /home/zephryj/projects/turingosv4
 succeeded in 210ms:
Cargo.toml:9:bincode = { version = "2", features = ["serde"] }
src/bottom_white/ledger/transition_ledger.rs:121:    /// Canonical SHA-256 digest. Stable wire format (NOT bincode/serde dependent).
src/bottom_white/ledger/transition_ledger.rs:542:// § 2.5 Canonical serialization (bincode v2; STATE_TRANSITION_SPEC § 2.5)
src/bottom_white/ledger/transition_ledger.rs:545:/// `bincode::config` used for the canonical `LedgerEntry` wire format.
src/bottom_white/ledger/transition_ledger.rs:550:/// - **`BTreeMap` keys**: bincode iterates the map in serde-supplied order; we
src/bottom_white/ledger/transition_ledger.rs:553:fn bincode_canonical_config() -> impl bincode::config::Config {
src/bottom_white/ledger/transition_ledger.rs:554:    bincode::config::standard()
src/bottom_white/ledger/transition_ledger.rs:562:pub fn canonical_encode<T: Serialize>(value: &T) -> Result<Vec<u8>, CanonicalCodecError> {
src/bottom_white/ledger/transition_ledger.rs:563:    bincode::serde::encode_to_vec(value, bincode_canonical_config())
src/bottom_white/ledger/transition_ledger.rs:567:/// Canonical decode the inverse of `canonical_encode`. Returns the value plus
src/bottom_white/ledger/transition_ledger.rs:573:        bincode::serde::decode_from_slice::<T, _>(bytes, bincode_canonical_config())
src/bottom_white/ledger/transition_ledger.rs:615:///     - `entry_canonical` = bincode v2 BE + fixed-int encoding of the full
src/bottom_white/ledger/transition_ledger.rs:757:        let canonical = canonical_encode(entry).map_err(|e| {
src/bottom_white/ledger/transition_ledger.rs:758:            LedgerWriterError::BackendCorruption(format!("canonical_encode: {e}"))
src/bottom_white/ledger/transition_ledger.rs:1206:        let bytes = canonical_encode(typed_tx).expect("encode");
src/bottom_white/ledger/transition_ledger.rs:1432:        let bad_bytes = b"\xff\xff this is not a valid bincode TypedTx";
src/bottom_white/ledger/transition_ledger.rs:1491:    // 14. canonical_encode/decode round-trip for LedgerEntry (foundation of read_at).
src/bottom_white/ledger/transition_ledger.rs:1495:        let bytes = canonical_encode(&e1).expect("encode");
src/bottom_white/ledger/transition_ledger.rs:1500:        let bytes_again = canonical_encode(&e1).expect("encode again");
src/bottom_white/ledger/system_keypair.rs:86:/// arrays > 32). With `bincode` + `fixed_int_encoding` this writes 64 raw bytes —
src/bottom_white/ledger/system_keypair.rs:98:/// (deterministic under bincode `fixed_int_encoding` → 64 raw bytes; no length prefix
src/state/sequencer.rs:28:    append, canonical_encode, LedgerEntry, LedgerEntrySigningPayload, LedgerWriter,
src/state/sequencer.rs:352:        let payload_bytes = canonical_encode(&tx)
src/state/typed_tx.rs:365:// prefix** before the bincode-canonical body bytes. This implements:
src/state/typed_tx.rs:367://   sig_input = sha256(b"turingosv4.<actor>.<purpose>.v1" || canonical_encode(payload))
src/state/typed_tx.rs:369:// Property: even if two distinct payload TYPES happen to bincode-encode to
src/state/typed_tx.rs:395:    use crate::bottom_white::ledger::transition_ledger::canonical_encode;
src/state/typed_tx.rs:396:    let body = canonical_encode(value).expect("canonical_encode of signing payload");
src/state/typed_tx.rs:895:    use crate::bottom_white::ledger::transition_ledger::{canonical_decode, canonical_encode};
src/state/typed_tx.rs:909:        let bytes = canonical_encode(value).expect("encode");
src/state/typed_tx.rs:1054:            let bytes = canonical_encode(&tx).expect("encode");
src/state/typed_tx.rs:1064:        let bytes_a = canonical_encode(&tx).expect("encode a");
src/state/typed_tx.rs:1065:        let bytes_b = canonical_encode(&tx).expect("encode b");
src/state/typed_tx.rs:1077:            let bytes = canonical_encode(&outer).expect("encode");
src/state/typed_tx.rs:1141:    /// (Different field shapes + bincode variant tags → ANY collision is a bincode
src/state/typed_tx.rs:1179:    /// this test locks that bincode honors the iteration order — defensive against
src/state/typed_tx.rs:1195:        let bytes_a = canonical_encode(&tx_a).expect("encode a");
src/state/typed_tx.rs:1196:        let bytes_b = canonical_encode(&tx_b).expect("encode b");
src/state/typed_tx.rs:1197:        let bytes_c = canonical_encode(&tx_c).expect("encode c");
src/state/typed_tx.rs:1216:            let bytes = canonical_encode(&tx).expect("encode default");
src/state/typed_tx.rs:1226:    /// even if their bincode bodies COULD overlap, the SHA-256 inputs differ.
src/state/typed_tx.rs:1409:        let bytes_a = canonical_encode(&tx_a).expect("encode a");
src/state/typed_tx.rs:1410:        let bytes_b = canonical_encode(&tx_b).expect("encode b");
src/state/typed_tx.rs:1411:        let bytes_c = canonical_encode(&tx_c).expect("encode c");

exec
/bin/bash -lc "rg -n \"CO1\\.7\\.5|transition bodies|NotYetImplemented|materializer|PriorRootNotFound|agent_view|permission_view|Goodhart|PredicateRegistry\" -S handover src | head -n 200" in /home/zephryj/projects/turingosv4
 succeeded in 171ms:
src/top_white/predicates/visibility.rs:1://! Predicate visibility policy (Goodhart shield per Const Art III.4 + WP § 9.4).
src/top_white/predicates/visibility.rs:8://! /// TRACE_MATRIX Const-Art-III.4 + Inv-10 + WP-spec-§1.5: Goodhart visibility
src/top_white/predicates/visibility.rs:17:    /// Used for hidden benchmarks that prevent Goodhart attacks.
src/top_white/predicates/registry.rs:6://! - Const Art III.4: Goodhart shield via three visibility classes
src/top_white/predicates/registry.rs:15://! /// TRACE_MATRIX WP-arch-§5.L1 + Inv-6 + Inv-10: PredicateRegistry
src/top_white/predicates/registry.rs:46:    /// Goodhart visibility class.
src/top_white/predicates/registry.rs:77:/// L1 PredicateRegistry — a deterministic ordered store of predicate metadata.
src/top_white/predicates/registry.rs:81:pub struct PredicateRegistry {
src/top_white/predicates/registry.rs:93:impl PredicateRegistry {
src/top_white/predicates/registry.rs:138:    /// Agent-visible projection of the registry (Goodhart shield per Inv 10).
src/top_white/predicates/registry.rs:172:        let mut reg = PredicateRegistry::new();
src/top_white/predicates/registry.rs:180:        let mut reg = PredicateRegistry::new();
src/top_white/predicates/registry.rs:191:        let mut reg = PredicateRegistry::new();
src/top_white/predicates/registry.rs:201:        let mut reg1 = PredicateRegistry::new();
src/top_white/predicates/registry.rs:202:        let mut reg2 = PredicateRegistry::new();
src/top_white/predicates/registry.rs:219:        let mut reg = PredicateRegistry::new();
src/top_white/predicates/registry.rs:228:        let mut reg = PredicateRegistry::new();
src/top_white/predicates/registry.rs:252:        let reg = PredicateRegistry::new();
src/bottom_white/mod.rs:3://! Deterministic, append-only substrate. tape, CAS, ledger, sandbox, materializer, tools.
src/bottom_white/ledger/transition_ledger.rs:10://! deferred to CO1.7.5 (NotYetImplemented stubs in `src/state/sequencer.rs`).
src/bottom_white/ledger/transition_ledger.rs:13://! - C1: two-mode replay enum (ChainOnly v1; FullTransition CO1.7.5+); skeleton now
src/bottom_white/ledger/transition_ledger.rs:16://!   sequencer code (deferred to CO1.7.5).
src/bottom_white/ledger/transition_ledger.rs:26://!   ride a `CanonicalMessage::LedgerEntrySigning(_)` variant when CO1.7.5+ extends
src/bottom_white/ledger/transition_ledger.rs:184:/// Production impl is `Git2LedgerWriter` (CO1.7.5+; refs/transitions/main commit chain).
src/bottom_white/ledger/transition_ledger.rs:188:/// deferred to CO1.7.5+ (only used by FullTransition replay; not v1 deliverable).
src/bottom_white/ledger/transition_ledger.rs:281:/// - `FullTransition`: CO1.7.5+ stage; verifies signatures + re-fetches payloads
src/bottom_white/ledger/transition_ledger.rs:296:    // FullTransition-mode-only (CO1.7.5+):
src/bottom_white/ledger/transition_ledger.rs:301:    /// (CO1.7.5 not yet shipped), this fires on every replay step with
src/bottom_white/ledger/transition_ledger.rs:302:    /// `inner = NotYetImplemented`.
src/bottom_white/ledger/transition_ledger.rs:390:/// **Stub-state caveat (CO1.7.5 unblocks)**: while `dispatch_transition`
src/bottom_white/ledger/transition_ledger.rs:391:/// returns `NotYetImplemented` for every variant, replay errors at stage 7
src/bottom_white/ledger/transition_ledger.rs:394:/// gates on CO1.7.5.
src/bottom_white/ledger/transition_ledger.rs:400:    predicate_registry: &crate::top_white::predicates::registry::PredicateRegistry,
src/bottom_white/ledger/transition_ledger.rs:501:/// - system_signature (CO1.7.5+: requires CanonicalMessage extension wired through keypair)
src/bottom_white/ledger/transition_ledger.rs:502:/// - resulting_state_root (CO1.7.5+: requires dispatch_transition + CO1.4-extra CAS persistence)
src/bottom_white/ledger/transition_ledger.rs:627:/// need it (CO1.7.5+ `head_t` wiring), but the `LedgerWriter::commit` trait
src/bottom_white/ledger/transition_ledger.rs:694:    /// CO1.7.5+ `head_t` wiring uses this to surface commit_sha alongside Hash.
src/bottom_white/ledger/transition_ledger.rs:1166:    use crate::top_white::predicates::registry::PredicateRegistry;
src/bottom_white/ledger/transition_ledger.rs:1246:        PredicateRegistry,
src/bottom_white/ledger/transition_ledger.rs:1255:        let preds = PredicateRegistry::new();
src/bottom_white/ledger/transition_ledger.rs:1260:    /// 15. CO1.7.5-stage: in stub mode, dispatch errors with NotYetImplemented;
src/bottom_white/ledger/transition_ledger.rs:1261:    ///     replay correctly bubbles up `Transition { at: 0, inner: NotYetImplemented }`.
src/bottom_white/ledger/transition_ledger.rs:1263:    ///     leaving stage 7 (dispatch) as the only gate. CO1.7.5 fills it.
src/bottom_white/ledger/transition_ledger.rs:1287:            matches!(err, ReplayError::Transition { at: 0, inner: crate::state::typed_tx::TransitionError::NotYetImplemented }),
src/bottom_white/ledger/transition_ledger.rs:1288:            "expected Transition(NotYetImplemented at 0); got {err:?}"
src/bottom_white/ledger/transition_ledger.rs:1330:        let preds = PredicateRegistry::new();
src/bottom_white/ledger/transition_ledger.rs:1480:    ///     until CO1.7.5 fills dispatch bodies. The skeleton of the
src/bottom_white/ledger/transition_ledger.rs:1481:    ///     test is here so CO1.7.5 just removes the #[ignore].
src/bottom_white/ledger/transition_ledger.rs:1483:    #[ignore = "CO1.7.5: requires real per-kind transition bodies"]
src/bottom_white/ledger/transition_ledger.rs:1485:        // CO1.7.5 plan: submit N tx through Sequencer + collect entries from
src/bottom_white/ledger/transition_ledger.rs:1488:        // (q_next, _signals) — currently all NotYetImplemented.
src/state/q_state.rs:98:// AgentVisibleProjection — Inv 10 Goodhart shield (CO P2.7 visibility runtime).
src/state/q_state.rs:102:/// visibility policy (Inv 10 Goodhart shield; `top_white::predicates::visibility`).
src/state/sequencer.rs:12://! `TransitionError::NotYetImplemented`; CO1.7.5 (downstream atom) fills the
src/state/sequencer.rs:34:use crate::top_white::predicates::registry::PredicateRegistry;
src/state/sequencer.rs:43:/// `TransitionError::NotYetImplemented`. CO1.7.5 fills each arm with the real
src/state/sequencer.rs:50:    _predicate_registry: &PredicateRegistry,
src/state/sequencer.rs:54:        TypedTx::Work(_) => Err(TransitionError::NotYetImplemented),
src/state/sequencer.rs:55:        TypedTx::Verify(_) => Err(TransitionError::NotYetImplemented),
src/state/sequencer.rs:56:        TypedTx::Challenge(_) => Err(TransitionError::NotYetImplemented),
src/state/sequencer.rs:57:        TypedTx::Reuse(_) => Err(TransitionError::NotYetImplemented),
src/state/sequencer.rs:58:        TypedTx::FinalizeReward(_) => Err(TransitionError::NotYetImplemented),
src/state/sequencer.rs:59:        TypedTx::TaskExpire(_) => Err(TransitionError::NotYetImplemented),
src/state/sequencer.rs:60:        TypedTx::TerminalSummary(_) => Err(TransitionError::NotYetImplemented),
src/state/sequencer.rs:89:/// prior value because pure transition bodies never mutate head_t per
src/state/sequencer.rs:243:    predicate_registry: Arc<PredicateRegistry>,
src/state/sequencer.rs:266:        predicate_registry: Arc<PredicateRegistry>,
src/state/sequencer.rs:309:            // Stub state: dispatch returns NotYetImplemented; apply_one
src/state/sequencer.rs:339:        // Stage 2: dispatch (pure). On reject (incl. NotYetImplemented stub),
src/state/sequencer.rs:429:    /// Read-only accessor (testing + CO1.7.5+ wiring).
src/state/sequencer.rs:447:// Tests — stub-mode coverage (CO1.7.5 fills real-transition tests)
src/state/sequencer.rs:480:        let preds = Arc::new(PredicateRegistry::new());
src/state/sequencer.rs:515:    // 1. dispatch_transition: every variant returns NotYetImplemented (stub state).
src/state/sequencer.rs:519:        let preds = PredicateRegistry::new();
src/state/sequencer.rs:583:            assert!(matches!(result, Err(TransitionError::NotYetImplemented)));
src/state/sequencer.rs:604:    // 3. apply_one in stub mode: returns Transition(NotYetImplemented); no
src/state/sequencer.rs:611:        assert!(matches!(err, ApplyError::Transition(TransitionError::NotYetImplemented)));
src/state/sequencer.rs:625:        let preds = Arc::new(PredicateRegistry::new());
src/state/typed_tx.rs:217:/// `step_transition` (CO1.7.5 body atom). The `signature` is over
src/state/typed_tx.rs:703:// note: full per-stage enum proliferation is CO1.7.5)
src/state/typed_tx.rs:708:/// `NotYetImplemented` for CO1.7.5 stub bodies (per Codex Q-G CHALLENGE).
src/state/typed_tx.rs:783:    // ── Stub sentinel (CO1.7.5 fills) ──────────────────────────────────────
src/state/typed_tx.rs:784:    /// Stub return value used by CO1.7.5 unimplemented bodies — preserves
src/state/typed_tx.rs:787:    NotYetImplemented,
src/state/typed_tx.rs:814:            Self::NotYetImplemented => write!(f, "transition body not yet implemented (CO1.7.5)"),
src/state/typed_tx.rs:821:// SignalBundle — minimal v1 typed shape (CO1.7.5 + CO1.9 enrich it later)
src/state/typed_tx.rs:828:/// compile and for CO1.7.5 transition bodies to construct each variant.
src/sdk/tools/wallet.rs:3:// V3L-22: Falsifier cannot buy YES (Goodhart defense at tool level)
handover/audits/GEMINI_CO1_7_EXTRA_ROUND3_AUDIT_2026-04-29.md:31:    -   Per Whitepaper v2 § 3.3, the bottom-white layer contains tools like the `ledger` and `state materializer`. The `Sequencer` is a state-mutation-driver, fitting this "tool" or "driver" role.
handover/audits/GEMINI_CO1_7_EXTRA_ROUND3_AUDIT_2026-04-29.md:60:1.  **"Later spec supersedes" Principle**: This principle remains coherent and is a cornerstone of the project's atomic, incremental development model. The v1.1 spec correctly applies this principle by acknowledging that CO1.7-extra (a later, more specific spec) enacts the `head_t` supersession that was previously deferred by CO1.7 v1.2. The table in § 0.4 now accurately reflects the division of labor: `head_t` is handled here, while `SignalKind` migrates to the future CO1.7.5.
handover/audits/GEMINI_CO1_7_EXTRA_ROUND3_AUDIT_2026-04-29.md:74:1.  **CO1.8 (L5 materializer)** and **CO1.9 (L6 signal indices)**: These future components are, like the Sequencer, runtime drivers. The `TuringBus.sequencer` pattern establishes a clean and correct precedent for where they will live: as peers to the Sequencer, owned directly by `TuringBus`. This avoids Kernel bloat and creates a clear "driver" layer, which is a highly sustainable pattern. No higher-level "Runtime" abstraction is immediately required; `TuringBus` is fulfilling that role correctly.
handover/audits/GEMINI_CO1_7_EXTRA_ROUND3_AUDIT_2026-04-29.md:76:2.  **CO1.7.5 (transition bodies)**: This atom's dependency is on the CO P2.x substrate, which provides schemas and logic for the transition functions. The Sequencer's placement is orthogonal to this. The `Sequencer` calls `dispatch_transition`, which will use the substrate. The entry-point (`TuringBus.submit_typed_tx`) is unaffected. The clean separation of the entry-point (TuringBus) from the execution logic (Sequencer/dispatch) makes the integration with the future substrate *cleaner*, not more complex.
handover/audits/GEMINI_WHITEPAPER_V2_AUDIT_2026-04-27_R2.md:43:**Reasoning**: The Public/Private/Commit-Reveal trinity is a direct and robust engineering implementation of the Goodhart shielding principle required by `constitution.md Art III.4`. It provides a concrete mechanism to prevent agents from gaming the evaluation metrics. The `OBS_WHITEPAPER_V2_PREDICATE_VISIBILITY_TRINITY.md` notes that the `Visibility` enum is already partly implemented (CO1.5), confirming this is a practical and aligned design.
handover/audits/GEMINI_CO1_7_ROUND1_AUDIT_2026-04-28.md:45:-   **A partial Art 0.2 implementation that should NOT be called "I-DETHASH witness" until full?**: **CHALLENGE**. Correct. Calling the current chain-integrity check a full "I-DETHASH witness" is an overstatement. It is a witness for *chain integrity*, not for *deterministic state transition*. The name should be qualified, for example, as an "I-DETHASH chain-integrity witness," until the full replay (spec § 4 `replay` function, including re-running pure transitions) is implemented in CO1.7.5+.
handover/audits/GEMINI_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:28:### Q1 — C_i full-cost honesty (anti-Goodhart #8)
handover/audits/GEMINI_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:38:### Q2 — T_i wall-clock seal (anti-Goodhart Soft Law escape)
handover/audits/GEMINI_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:47:*   **(Q2.c) Soft Law compatibility**: **PASS**. The seal is preserved. `mark_final_accept` is called *after* the Lean verification step. When Phase C introduces a Soft Law mode that fakes a runtime accept *before* a post-hoc verification, the current bracketing will correctly include the post-hoc verification time in `T_i`, preventing a Goodhart attack where the agent fakes success and stops the clock early.
handover/audits/GEMINI_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:90:The architecture does **not** force the Phase C implementer to do the right thing. It relies on them being disciplined enough to refactor the internals of `make_pput`. This is an unacceptable risk for a foundational anti-Goodhart mechanism.
handover/audits/CO1_7_EXTRA_DUAL_AUDIT_VERDICT_R3_2026-04-29.md:93:2. **CO1.7.5 (transition bodies)**: future atom (unchanged from r1+r2 verdicts)
handover/audits/GEMINI_CO1_7_5_ROUND1_AUDIT_2026-04-29.md:1:# Gemini CO1.7.5 transition_bodies_and_runtime_wiring Round-1 Audit Run
handover/audits/GEMINI_CO1_7_5_ROUND1_AUDIT_2026-04-29.md:9:# Gemini CO1.7.5 Round-1 Audit
handover/audits/GEMINI_CO1_7_5_ROUND1_AUDIT_2026-04-29.md:15:CO1.7.5 demonstrates strong constitutional alignment.
handover/audits/GEMINI_CO1_7_5_ROUND1_AUDIT_2026-04-29.md:17:-   **Art 0.1 四要素映射 (Tape / Input / Q / State)**: The design correctly maps the four elements. The transition bodies are the `Strict discipline` operating on `Q_t` (the `Paper`). The Sequencer's `apply_one` machinery acts as the `Pencil`, appending an accepted `LedgerEntry` to the `Tape`. The "rubber" function is correctly implemented by the principle that rejected transactions do not advance `Q_t` (`Q_{t+1} = Q_t`), as specified in `Sequencer::apply_one`'s early return on `Err`.
handover/audits/GEMINI_CO1_7_5_ROUND1_AUDIT_2026-04-29.md:19:-   **Art 0.2 Tape Canonical 公理**: The design upholds this axiom. The `Sequencer::apply_one` pseudocode in the inherited `CO1.7` spec (§ 3) and the shipped `sequencer.rs` (lines 310-315) show that the pure transition function is called *before* any commit action. If it returns an `Err`, the function returns early, and no `LedgerEntry` is committed. This ensures rejected transactions do not advance the ledger state, which holds for all 7 transition bodies as they are all routed through `dispatch_transition`. The `LedgerEntry.system_signature` attests to the sequencer-stamped semantics of the *accepted* transition, fulfilling the attestation requirement.
handover/audits/GEMINI_CO1_7_5_ROUND1_AUDIT_2026-04-29.md:23:-   **Anti-Oreo 三层**: The architecture is correctly layered. The 7 transition bodies are pure functions that perform state mutation logic, fitting squarely in the `middle-black` layer. The `LedgerWriter` trait and its `Git2LedgerWriter` implementation, which persist the ledger, are in the `bottom-white` layer (`src/bottom_white/ledger/`). The Sequencer acts as the orchestrator between these layers.
handover/audits/GEMINI_CO1_7_5_ROUND1_AUDIT_2026-04-29.md:35:**This constitutes a process-level challenge**: The spec must be amended to state that as part of the CO1.7.5 atom's closure, a formal change request or issue will be filed against the `STATE_TRANSITION_SPEC` to align its pseudocode with the as-implemented reality of `head_t` mutation and `SignalKind` shape.
handover/audits/GEMINI_CO1_7_5_ROUND1_AUDIT_2026-04-29.md:41:CO1.7.5 correctly closes Wave 6 #1.
handover/audits/GEMINI_CO1_7_5_ROUND1_AUDIT_2026-04-29.md:43:-   **Exhaustiveness**: The four deliverables (D1-D4) comprehensively cover the remaining work for the L4 transition ledger family. D1 implements the core logic, D2 wires the final constitutional anchor (`head_t`), D3 integrates it into the runtime, and D4 enables the end-to-end verification tests. There are no apparent gaps or silently deferred items that belong in L4. The spec correctly identifies L5 (materializer) and L6 (signal indices) as subsequent waves.
handover/audits/GEMINI_CO1_7_5_ROUND1_AUDIT_2026-04-29.md:46:    -   **CO1.8 (L5 materializer)**: The spec (§ 2, item 6) confirms that transition bodies will use the existing `q_next.economic_state_t.derive_state_root()` accessor. This provides a clean, single point of substitution for CO1.8's real merkleized materializer, avoiding hard-coding.
handover/audits/GEMINI_CO1_7_5_ROUND1_AUDIT_2026-04-29.md:67:-   **Art 0.2 Reconstructibility**: Emitting `SignalKind::Empty` for 4 of 7 transition bodies does **not** cause observable-state loss that breaks reconstructibility. Art 0.2 requires that all signals be reconstructible from the `tape`. The tape contains the full `LedgerEntry`, which includes the `tx_payload_cid`, pointing to the complete `TypedTx`. The logic for deriving richer signals (like reputation or price deltas) is deterministic and based on the contents of the `TypedTx`. Therefore, L5/L6 can re-run this deterministic logic during replay to reconstruct the full signal stream. The `SignalKind` on the `LedgerEntry` is a summary, not the sole source of truth.
handover/audits/GEMINI_CO1_7_5_ROUND1_AUDIT_2026-04-29.md:71:-   **Safe Deferral**: This is a classic example of incremental delivery. It ships a functional L4 now, with a clear and safe path for adding L6 signal richness later. This reduces the complexity and risk of the CO1.7.5 atom.
handover/audits/GEMINI_CO1_7_5_ROUND1_AUDIT_2026-04-29.md:89:-   **CO1.8 `materializer::apply` Substitution**: Yes, the spec reserves a clean substitution point. By relying on the existing `derive_state_root()` method, it avoids entangling the transition body logic with the details of state root computation, making the CO1.8 refactor a clean swap.
handover/audits/GEMINI_CO1_7_5_ROUND1_AUDIT_2026-04-29.md:93:-   **Avoidance of Hard-Coding**: The transition bodies are specified to be pure translations of the `STATE` spec pseudocode. They operate on the abstract `QState` and return a new `QState`. This insulates them from the specifics of downstream plumbing like materializers or signal indexers, which are handled by the Sequencer and future components.
handover/audits/GEMINI_CO1_7_5_ROUND1_AUDIT_2026-04-29.md:109:The CO1.7.5 spec is architecturally sound, constitutionally aligned, and demonstrates a mature understanding of the project's principles. The design correctly closes Wave 6 #1 while providing clear affordances for future waves.
handover/audits/GEMINI_CO1_7_5_ROUND1_AUDIT_2026-04-29.md:111:However, the verdict is a **CHALLENGE** based on two key points that require remediation before the spec can be considered complete. These are not flaws in the technical design of the transition bodies, but rather in the institutional process and conservative default principles that govern the system's long-term health.
handover/audits/CODEX_CO_P0_AUDIT_2026-04-26.md:32:| CO1.5 Predicate Registry + visibility | CHALLENGE | Moving `lean4_oracle.rs` from experiment to root requires dependency inversion: current file imports the root crate from the experiment crate (`/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/lean4_oracle.rs:7-10`). The Goodhart airgap test claims it will catch leaks via error/log/retry counts (`/home/zephryj/projects/turingosv4/handover/architect-insights/CO_MEGA_PLAN_v3.1_2026-04-26.md:111`), but retry-count/log-channel observability must be specified first. |
handover/audits/CODEX_CO_P0_AUDIT_2026-04-26.md:38:| CO1.11 Safety vs creation fail policy | CHALLENGE | Implementable after `PredicateRegistry` exists. Test can catch fail-open/fail-closed behavior, but only if predicate domain classification is part of registry schema (`/home/zephryj/projects/turingosv4/handover/architect-insights/CO_MEGA_PLAN_v3.1_2026-04-26.md:184-186`). |
handover/audits/CODEX_CO_P0_AUDIT_2026-04-26.md:75:| §6 transition protocol (`:422-515`) | No seed row | CO1.7.5 `src/transition/mod.rs` (`/home/zephryj/projects/turingosv4/handover/architect-insights/CO_MEGA_PLAN_v3.1_2026-04-26.md:136`) | ORPHAN-IN-TRACE, not orphan-in-plan. Add Blueprint §6 row and test for reject path preserving `Q_t` while recording rejected tx (`:505-514`). |
handover/audits/CODEX_CO_P0_AUDIT_2026-04-26.md:78:| §9 selective shielding (`:671-721`) | Only §9.4 Goodhart row (`/home/zephryj/projects/turingosv4/handover/whitepapers/TURINGOS_v4_FINAL_BLUEPRINT_2026-04-26.md:362`) | CO1.5.2/1.5.7, CO1.8.6/1.8.7 | CHALLENGE: §9.1-9.3 are not in Blueprint §6 seed; add rows for error hiding, minimal context, and correlation shielding. |
handover/audits/CODEX_CO_P0_AUDIT_2026-04-26.md:102:| Architecture relationship (`:139-149`) | No | Cross-reference only | PASS as explanatory section, but use it to add missing rows for Goodhart/economic Laws. |
handover/audits/CODEX_CO_P0_AUDIT_2026-04-26.md:226:- Whether the conformance suite design catches semantic violations, not just file existence. Highest-priority examples: Inv8 DAG construction, Goodhart private-predicate leak channels, L6 deterministic tie ordering, and reputation-not-predicate-substitute.
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:43:| R2-K3 | head_t mutation explicitly deferred to CO1.7.5+ when `Git2LedgerWriter` exists. v1.x ledger owns `ledger_root_t` only. `LedgerWriter::commit` keeps `Hash` return. Spec § 0/§ 3/§ 5 updated; "CO1.7 owns head_t" claim removed. |
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:60:- § 5 storage backend says head_t deferred to CO1.7.5+
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:101:**Status**: v1.2 — round-2 returned PASS (Gemini, high) + CHALLENGE (Codex, high; 3 narrow patch blockers). Conservative merged CHALLENGE. v1.2 closes the 3 v1.1→v1.2 patches: (a) C3 actually wired in code (`CanonicalMessage::LedgerEntrySigning([u8;32])` + `transition_ledger_emitter::sign_ledger_entry`); (b) K3 head_t mutation explicitly deferred to CO1.7.5+ (no longer claimed in v1.x); (c) `ObjectType::Transition` replaced with shipped `ObjectType::ProposalPayload`. Plus typo fix and 1 new test. Awaiting round-3.
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:122:| **R2-C3** | Spec claimed "C3 CLOSED" but `system_keypair.rs` had no LedgerEntry path; skeleton itself said "deferred to CO1.7.5+" | Wave 4-B additive extension shipped: `CanonicalMessage::LedgerEntrySigning([u8;32])` (opaque digest variant; avoids transition_ledger ↔ system_keypair circular dep) + `canonical_digest` match arm + new `pub(crate) mod transition_ledger_emitter` with `sign_ledger_entry(keypair, digest)`. Skeleton test 9 (`signature_round_trip_and_transplant_defense`) now exercises the real roundtrip + K2 + D1 defenses. | Codex round-2 must-fix #1 |
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:123:| **R2-K3** | Spec § 3 / § 5 said "CO1.7 owns head_t = NodeId(commit_sha)" but `LedgerWriter::commit` returns `Hash` not commit SHA; v1.1 InMemoryLedgerWriter has no commit_sha to return at all → contradiction | head_t mutation explicitly **deferred to CO1.7.5+** (when Git2LedgerWriter exists and can return both Hash + commit SHA). v1.x ledger owns `ledger_root_t` only; `head_t` continues to be set elsewhere (currently QState placeholder; CO1.7.5 wiring concern). Spec § 0 / § 3 / § 5 updated. | Codex round-2 must-fix #2 |
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:135:| C1 | replay was single-mode; called "I-DETHASH witness" but skeleton only did chain check | Two-mode `ReplayMode::ChainOnly` (skeleton-stage) vs `ReplayMode::FullTransition` (CO1.7.5+; I-DETHASH witness only in this mode) | Codex Q-D + Gemini Q3 |
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:141:| K4 | spec `LedgerWriter::commit(&self) → NodeId` + `iter_from` did not match skeleton `commit(&mut self) → Hash` | Spec aligned to skeleton: `&mut self` + `Hash` return; `iter_from` deferred to CO1.7.5+ when needed for cold-replay | Codex Q-H |
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:144:| K7 | spec promised 8 conformance tests; skeleton has 6 | Explicit list of 8 tests with skeleton-stage vs CO1.7.5-stage marker; unimplemented stubs now stage-marked | Codex Q-H |
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:160:- **replay (two-mode)**: `ChainOnly` (chain integrity; skeleton-stage; v1) vs `FullTransition` (rerun pure transitions + verify state_root + verify signatures; CO1.7.5+; THE I-DETHASH witness).
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:161:- **Storage backend**: git2-rs commit chain (built on CO1.4 CAS); each LedgerEntry = one git commit on `refs/transitions/main`. **R2-K3**: head_t mutation deferred to CO1.7.5+ — v1.x ledger does NOT mutate `Q_t.head_t` directly. Once `Git2LedgerWriter::commit` exists and returns commit_sha alongside Hash, CO1.7.5 wiring will set `head_t = NodeId(commit_sha)` outside the L4 sequencer apply path.
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:168:- L5 materializer (state_root computation) — deferred to **CO1.8**. **K3 boundary (revised v1.2)**: CO1.7 owns `ledger_root_t` only; CO1.8 owns `state_root_t`; **head_t mutation is deferred to CO1.7.5+ wiring** (when `Git2LedgerWriter` exists). Sequencer does NOT mutate `state_root_t` or `head_t` directly; it accepts `q_next.state_root_t` as returned by the transition function and persists `ledger_root_t`.
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:353:└── sequencer.rs                 (NEW; deferred to CO1.7.5; pre-audit type stub may land in v1.1 if useful)
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:384:    /// Storage backend (in CO1.7.5+; skeleton uses InMemoryLedgerWriter).
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:388:    predicate_registry: Arc<PredicateRegistry>,
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:476:        // `ledger_root_t` only. head_t mutation is **deferred to CO1.7.5+ wiring**
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:488:**Why no head_t mutation in apply_one (K3, revised v1.2)**: v1.x CO1.7 owns `ledger_root_t` only. CO1.8 owns `state_root_t`. **head_t mutation deferred to CO1.7.5+** when `Git2LedgerWriter` provides a commit_sha return alongside Hash; the InMemoryLedgerWriter used by the v1 skeleton has no commit_sha to expose, so the trait keeps a single `Hash` return and head_t wiring is a separate downstream concern. Sequencer never calls `NodeId::from_state_root(...)`.
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:513:    /// CO1.7.5+ stage: full re-execution.
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:528:/// CO1.7.5+ stage entry point (v1.1 spec only; impl deferred).
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:559:- **K3 (v1.2)**: `head_t = NodeId(commit_sha)` is the canonical convention WHEN head_t is wired (CO1.7.5+). v1.x sequencer does NOT mutate head_t — `Git2LedgerWriter` is needed to surface commit_sha. `NodeId::from_state_root(...)` is NOT used by L4 in any version.
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:577:    // K4: iter_from() deferred — used only by FullTransition replay; CO1.7.5+ adds it.
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:581:**Implementation (CO1.7.5+)**: `Git2LedgerWriter` (built on existing CO1.4 CAS); skeleton `InMemoryLedgerWriter` for v1 testing.
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:616:| `tests/replay_full_transition_state_root` | CO1.7.5+ (post-CO1.4-extra) | FullTransition replay re-runs dispatch_transition; asserts state_root match (I-DETHASH witness) |
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:617:| `tests/system_signature_verifies_via_canonical_message` | CO1.7.5+ | LedgerEntry.system_signature verifies through `verify_system_signature(&CanonicalMessage::LedgerEntrySigning(...), epoch, pinned_pubkeys)` |
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:618:| `tests/cas_payload_round_trip` | CO1.7.5+ (after CO1.4-extra) | put→get round trip; CID stability across runs |
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:619:| `tests/sequencer_serial_replay_byte_identity` | CO1.7.5+ | submit 100 tx; replay → byte-identical state_root |
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:621:**v1 stage (skeleton)**: 8 tests (6 already in skeleton + 2 NEW K2/Q9). **CO1.7.5+ stage**: 4 more.
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:631:    predicate_registry: &PredicateRegistry,
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:660:Future runtime wiring (CO1.7.5+) into `bus.rs`/`kernel.rs` WILL need STEP_B — that's a separate atom. The retirement of `src/ledger.rs` (legacy top-level) is in CO1.1.5 per `STATE_TRANSITION_SPEC § 5.3`.
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:686:| Q6 replay error mode | open | **Reject on first error** (current). Diagnostic-collection mode is a v4.x extension; first-error simplicity matches CO1.7.5 implementation budget. |
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:691:| Q11 NEW open Qs | — | Codex round-1 listed: parent_ledger_root binding (now K2 / done), rejected-submission logical time (now K1 / done), CAS persistence (now C2 → CO1.4-extra), canonical fixtures (deferred to CO1.7.5+ test stubs), L4/L5 head_t ownership (now K3 / done). All addressed. |
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:705:**Pre-implementation gate**: CO1.7 must reach `PASS/PASS` before implementing CO1.7.5 (transition function bodies) + CO1.4-extra (CAS persistence). Sedimented per CLAUDE.md "Audit Standard" (Generator ≠ Evaluator) + memory `feedback_dual_audit`.
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:713:  - CO1.7-impl proper: ~600-900 LoC + 8 conformance tests (4 skeleton-stage + 4 CO1.7.5-stage)
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:715:  - CO1.7.5 (transition function bodies): separate downstream atom
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:764:| K7 conformance test gap | Explicit 8 tests (4 skeleton + 4 CO1.7.5+ stage) | § 7 |
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:787://! persistence are stubbed; full-mode replay is deferred to CO1.7.5+.
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:790://! - C1: two-mode replay enum (ChainOnly v1; FullTransition CO1.7.5+); skeleton now
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:793://!   sequencer code (deferred to CO1.7.5).
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:803://!   ride a `CanonicalMessage::LedgerEntrySigning(_)` variant when CO1.7.5+ extends
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:958:/// Production impl is `Git2LedgerWriter` (CO1.7.5+; refs/transitions/main commit chain).
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:962:/// deferred to CO1.7.5+ (only used by FullTransition replay; not v1 deliverable).
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:1034:/// - `FullTransition`: CO1.7.5+ stage; verifies signatures + re-fetches payloads
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:1049:    // FullTransition-mode-only (CO1.7.5+):
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:1081:/// - system_signature (CO1.7.5+: requires CanonicalMessage extension wired through keypair)
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:1082:/// - resulting_state_root (CO1.7.5+: requires dispatch_transition + CO1.4-extra CAS persistence)
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:2279:| C1 | replay single-mode | Two-mode `ReplayMode::ChainOnly` (skeleton) vs `FullTransition` (CO1.7.5+); I-DETHASH bound to Full only |
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:2288:| K7 | spec promised 8 tests, skeleton had 6 | 8 tests with stage marker (4 skeleton + 4 CO1.7.5+) |
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:2317:**Q-A7** (K4 trait): commit(&mut self) -> Hash matches skeleton. iter_from deferred. Does this leave a hole for FullTransition replay? (it does — but flagged as CO1.7.5+ stage, OK?)
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:2323:**Q-A10** (K7 conformance): 8 tests now (4 skeleton + 4 CO1.7.5+). Verify the 4 skeleton-stage tests are actually present in skeleton (not the spec only). The CO1.7.5+ stage tests are deferred — is that OK, or should v1.1 ship at least stubbed unimplemented!() test functions?
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:2363:If PASS: explicit GO for CO1.7 implementation start (CO1.7.5 etc.).
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:2391:| C1 | replay was single-mode; called "I-DETHASH witness" but skeleton only did chain check | Two-mode `ReplayMode::ChainOnly` (skeleton-stage) vs `ReplayMode::FullTransition` (CO1.7.5+; I-DETHASH witness only in this mode) | Codex Q-D + Gemini Q3 |
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:2397:| K4 | spec `LedgerWriter::commit(&self) → NodeId` + `iter_from` did not match skeleton `commit(&mut self) → Hash` | Spec aligned to skeleton: `&mut self` + `Hash` return; `iter_from` deferred to CO1.7.5+ when needed for cold-replay | Codex Q-H |
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:2400:| K7 | spec promised 8 conformance tests; skeleton has 6 | Explicit list of 8 tests with skeleton-stage vs CO1.7.5-stage marker; unimplemented stubs now stage-marked | Codex Q-H |
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:2416:- **replay (two-mode)**: `ChainOnly` (chain integrity; skeleton-stage; v1) vs `FullTransition` (rerun pure transitions + verify state_root + verify signatures; CO1.7.5+; THE I-DETHASH witness).
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:2424:- L5 materializer (state_root computation) — deferred to **CO1.8**. **K3 boundary**: CO1.7 owns `ledger_root_t` + `head_t`; CO1.8 owns `state_root_t`. Sequencer does NOT mutate `state_root_t` directly; it accepts `q_next.state_root_t` as returned by the transition function.
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:2599:└── sequencer.rs                 (NEW; deferred to CO1.7.5; pre-audit type stub may land in v1.1 if useful)
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:2630:    /// Storage backend (in CO1.7.5+; skeleton uses InMemoryLedgerWriter).
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:2634:    predicate_registry: Arc<PredicateRegistry>,
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:2720:        // path is CO1.7.5+ wiring concern.)
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:2729:**Why no head_t mutation in apply_one (K3)**: CO1.7 owns `ledger_root_t` and the commit-chain `head_t`; CO1.8 (L5 materializer) owns `state_root_t` mutation. Sequencer accepts `q_next.state_root_t` as the transition function returns it; sequencer does NOT call `NodeId::from_state_root(...)`.
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:2754:    /// CO1.7.5+ stage: full re-execution.
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:2769:/// CO1.7.5+ stage entry point (v1.1 spec only; impl deferred).
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:2818:    // K4: iter_from() deferred — used only by FullTransition replay; CO1.7.5+ adds it.
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:2822:**Implementation (CO1.7.5+)**: `Git2LedgerWriter` (built on existing CO1.4 CAS); skeleton `InMemoryLedgerWriter` for v1 testing.
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:2857:| `tests/replay_full_transition_state_root` | CO1.7.5+ (post-CO1.4-extra) | FullTransition replay re-runs dispatch_transition; asserts state_root match (I-DETHASH witness) |
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:2858:| `tests/system_signature_verifies_via_canonical_message` | CO1.7.5+ | LedgerEntry.system_signature verifies through `verify_system_signature(&CanonicalMessage::LedgerEntrySigning(...), epoch, pinned_pubkeys)` |
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:2859:| `tests/cas_payload_round_trip` | CO1.7.5+ (after CO1.4-extra) | put→get round trip; CID stability across runs |
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:2860:| `tests/sequencer_serial_replay_byte_identity` | CO1.7.5+ | submit 100 tx; replay → byte-identical state_root |
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:2862:**v1 stage (skeleton)**: 8 tests (6 already in skeleton + 2 NEW K2/Q9). **CO1.7.5+ stage**: 4 more.
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:2872:    predicate_registry: &PredicateRegistry,
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:2901:Future runtime wiring (CO1.7.5+) into `bus.rs`/`kernel.rs` WILL need STEP_B — that's a separate atom. The retirement of `src/ledger.rs` (legacy top-level) is in CO1.1.5 per `STATE_TRANSITION_SPEC § 5.3`.
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:2927:| Q6 replay error mode | open | **Reject on first error** (current). Diagnostic-collection mode is a v4.x extension; first-error simplicity matches CO1.7.5 implementation budget. |
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:2932:| Q11 NEW open Qs | — | Codex round-1 listed: parent_ledger_root binding (now K2 / done), rejected-submission logical time (now K1 / done), CAS persistence (now C2 → CO1.4-extra), canonical fixtures (deferred to CO1.7.5+ test stubs), L4/L5 head_t ownership (now K3 / done). All addressed. |
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:2946:**Pre-implementation gate**: CO1.7 must reach `PASS/PASS` before implementing CO1.7.5 (transition function bodies) + CO1.4-extra (CAS persistence). Sedimented per CLAUDE.md "Audit Standard" (Generator ≠ Evaluator) + memory `feedback_dual_audit`.
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:2954:  - CO1.7-impl proper: ~600-900 LoC + 8 conformance tests (4 skeleton-stage + 4 CO1.7.5-stage)
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:2956:  - CO1.7.5 (transition function bodies): separate downstream atom
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:3005:| K7 conformance test gap | Explicit 8 tests (4 skeleton + 4 CO1.7.5+ stage) | § 7 |
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:3028://! persistence are stubbed; full-mode replay is deferred to CO1.7.5+.
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:3031://! - C1: two-mode replay enum (ChainOnly v1; FullTransition CO1.7.5+); skeleton now
handover/audits/CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md:3034://!   sequencer code (deferred to CO1.7.5).

exec
/bin/bash -lc "rg --files handover/specs handover/audits | rg 'CO1_7_5|CO1\\.7\\.5|1_7_5'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
handover/audits/GEMINI_CO1_7_5_ROUND1_AUDIT_2026-04-29.md
handover/audits/CO1_7_5_DUAL_AUDIT_VERDICT_R1_2026-04-29.md
handover/audits/run_gemini_co1_7_5_round1_audit.py
handover/audits/run_codex_co1_7_5_round1_audit.sh
handover/audits/CODEX_CO1_7_5_ROUND1_AUDIT_2026-04-29.md

exec
/bin/bash -lc "nl -ba src/bottom_white/ledger/transition_ledger.rs | sed -n '540,580p;380,405p;1480,1492p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   380	/// 6.5 (v1.1 C-3): decoded_typed_tx.tx_kind() MUST equal entry.tx_kind
   381	/// 7. dispatch_transition re-run produces (q_next, _signals)
   382	/// 8. q_next.state_root_t matches entry.resulting_state_root
   383	/// 9. resulting_ledger_root recomputed via append() matches stored
   384	///
   385	/// **v1.1 C-1 closure**: takes a full `genesis: &QState` (was `genesis_state_root`
   386	/// + `genesis_ledger_root` only). Caller provides the complete genesis state
   387	/// so dispatch_transition can read budget / registries / balances / task markets
   388	/// — fabricating `QState::genesis()` was dropping these fields.
   389	///
   390	/// **Stub-state caveat (CO1.7.5 unblocks)**: while `dispatch_transition`
   391	/// returns `NotYetImplemented` for every variant, replay errors at stage 7
   392	/// for any non-empty chain. Conformance tests exercising stages 1-6.5
   393	/// independently are `#[test]`-runnable now; full state_root reconstruction
   394	/// gates on CO1.7.5.
   395	pub fn replay_full_transition(
   396	    genesis: &crate::state::q_state::QState,
   397	    entries: &[LedgerEntry],
   398	    cas: &dyn LedgerCasView,
   399	    pinned_pubkeys: &crate::bottom_white::ledger::system_keypair::PinnedSystemPubkeys,
   400	    predicate_registry: &crate::top_white::predicates::registry::PredicateRegistry,
   401	    tool_registry: &crate::bottom_white::tools::registry::ToolRegistry,
   402	) -> Result<crate::state::q_state::QState, ReplayError> {
   403	    use crate::bottom_white::ledger::system_keypair::{
   404	        verify_system_signature, CanonicalMessage,
   405	    };
   540	
   541	// ────────────────────────────────────────────────────────────────────────────
   542	// § 2.5 Canonical serialization (bincode v2; STATE_TRANSITION_SPEC § 2.5)
   543	// ────────────────────────────────────────────────────────────────────────────
   544	
   545	/// `bincode::config` used for the canonical `LedgerEntry` wire format.
   546	///
   547	/// **Frozen choices** (per STATE_TRANSITION_SPEC § 2.5):
   548	/// - **Big-endian** byte order (network order; deterministic across platforms).
   549	/// - **Fixed-int encoding** (no varint; fixed-width for byte-stable round-trip).
   550	/// - **`BTreeMap` keys**: bincode iterates the map in serde-supplied order; we
   551	///   only ever encode `BTreeMap` (sorted by construction) so key order is lex.
   552	/// - **No padding, no implicit alignment.**
   553	fn bincode_canonical_config() -> impl bincode::config::Config {
   554	    bincode::config::standard()
   555	        .with_big_endian()
   556	        .with_fixed_int_encoding()
   557	}
   558	
   559	/// Canonical encode any serde-Serialize value to bytes (CO1.7 wire format).
   560	/// Used by `Git2LedgerWriter` for commit-message bodies and by future callers
   561	/// needing byte-stable signatures over typed payloads.
   562	pub fn canonical_encode<T: Serialize>(value: &T) -> Result<Vec<u8>, CanonicalCodecError> {
   563	    bincode::serde::encode_to_vec(value, bincode_canonical_config())
   564	        .map_err(|e| CanonicalCodecError::Encode(e.to_string()))
   565	}
   566	
   567	/// Canonical decode the inverse of `canonical_encode`. Returns the value plus
   568	/// the number of bytes consumed (entire input must be consumed for a clean decode).
   569	pub fn canonical_decode<T: serde::de::DeserializeOwned>(
   570	    bytes: &[u8],
   571	) -> Result<T, CanonicalCodecError> {
   572	    let (value, consumed) =
   573	        bincode::serde::decode_from_slice::<T, _>(bytes, bincode_canonical_config())
   574	            .map_err(|e| CanonicalCodecError::Decode(e.to_string()))?;
   575	    if consumed != bytes.len() {
   576	        return Err(CanonicalCodecError::TrailingBytes {
   577	            consumed,
   578	            total: bytes.len(),
   579	        });
   580	    }
  1480	    ///     until CO1.7.5 fills dispatch bodies. The skeleton of the
  1481	    ///     test is here so CO1.7.5 just removes the #[ignore].
  1482	    #[test]
  1483	    #[ignore = "CO1.7.5: requires real per-kind transition bodies"]
  1484	    fn sequencer_serial_replay_byte_identity() {
  1485	        // CO1.7.5 plan: submit N tx through Sequencer + collect entries from
  1486	        // ledger_writer + replay_full_transition(...) → assert final state_root
  1487	        // matches sequencer's q.state_root_t. Dispatch must produce real
  1488	        // (q_next, _signals) — currently all NotYetImplemented.
  1489	    }
  1490	
  1491	    // 14. canonical_encode/decode round-trip for LedgerEntry (foundation of read_at).
  1492	    #[test]

exec
/bin/bash -lc "nl -ba src/state/sequencer.rs | sed -n '1,70p;300,365p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	//! L4 Sequencer + dispatch_transition (CO1.7-impl A2 + A3).
     2	//!
     3	//! Spec authority:
     4	//! - `handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md` § 3 (Sequencer
     5	//!   pseudocode, K1 dual-counter, K3 head_t deferred, C3 sign API)
     6	//! - `handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md` § 8
     7	//!   (dispatch_transition exhaustive enum match; K5 Slash dropped)
     8	//!
     9	//! Single-writer per (runtime_repo, run_id). Per spec § 5.2.1.
    10	//!
    11	//! **Stub state (this atom)**: every per-kind transition returns
    12	//! `TransitionError::NotYetImplemented`; CO1.7.5 (downstream atom) fills the
    13	//! bodies. The structural correctness of the apply path (snapshot → dispatch →
    14	//! CAS put → sign → root fold → commit → Q_t mutation) is locked by the
    15	//! impl + tests here; what's left is per-kind transition logic.
    16	//!
    17	//! /// TRACE_MATRIX § 5.2.1 + § 8 — L4 sequencer single-writer + dispatch.
    18	
    19	use std::sync::atomic::{AtomicU64, Ordering};
    20	use std::sync::{Arc, RwLock};
    21	
    22	use crate::bottom_white::cas::schema::ObjectType;
    23	use crate::bottom_white::cas::store::{CasError, CasStore};
    24	use crate::bottom_white::ledger::system_keypair::{
    25	    transition_ledger_emitter, Ed25519Keypair, KeypairError, SystemEpoch,
    26	};
    27	use crate::bottom_white::ledger::transition_ledger::{
    28	    append, canonical_encode, LedgerEntry, LedgerEntrySigningPayload, LedgerWriter,
    29	    LedgerWriterError,
    30	};
    31	use crate::bottom_white::tools::registry::ToolRegistry;
    32	use crate::state::q_state::QState;
    33	use crate::state::typed_tx::{SignalBundle, TransitionError, TypedTx};
    34	use crate::top_white::predicates::registry::PredicateRegistry;
    35	
    36	// ────────────────────────────────────────────────────────────────────────────
    37	// § 8 dispatch_transition — exhaustive enum match (K5: NO Slash)
    38	// ────────────────────────────────────────────────────────────────────────────
    39	
    40	/// TRACE_MATRIX § 8 — exhaustive dispatch over `TypedTx` variants.
    41	///
    42	/// **Stub state (CO1.7-impl A3)**: every variant returns
    43	/// `TransitionError::NotYetImplemented`. CO1.7.5 fills each arm with the real
    44	/// transition body per `STATE_TRANSITION_SPEC § 3.1-3.7`. The exhaustive match
    45	/// itself is the contract: any future TypedTx variant addition triggers a
    46	/// non-exhaustive-match compile error here, forcing explicit handling.
    47	pub(crate) fn dispatch_transition(
    48	    _q: &QState,
    49	    tx: &TypedTx,
    50	    _predicate_registry: &PredicateRegistry,
    51	    _tool_registry: &ToolRegistry,
    52	) -> Result<(QState, SignalBundle), TransitionError> {
    53	    match tx {
    54	        TypedTx::Work(_) => Err(TransitionError::NotYetImplemented),
    55	        TypedTx::Verify(_) => Err(TransitionError::NotYetImplemented),
    56	        TypedTx::Challenge(_) => Err(TransitionError::NotYetImplemented),
    57	        TypedTx::Reuse(_) => Err(TransitionError::NotYetImplemented),
    58	        TypedTx::FinalizeReward(_) => Err(TransitionError::NotYetImplemented),
    59	        TypedTx::TaskExpire(_) => Err(TransitionError::NotYetImplemented),
    60	        TypedTx::TerminalSummary(_) => Err(TransitionError::NotYetImplemented),
    61	    }
    62	}
    63	
    64	// ────────────────────────────────────────────────────────────────────────────
    65	// CO1.7-extra D2: advance_head_t — post-commit head_t close (Art 0.4)
    66	// ────────────────────────────────────────────────────────────────────────────
    67	
    68	/// TRACE_MATRIX § 5 — L4 sequencer post-commit head_t wiring (Art 0.4).
    69	///
    70	/// Closes the G-1 carry-forward: when `writer` surfaces a commit OID hex
   300	    /// Driver loop. Drains the queue and runs `apply_one` on each tx. Errors
   301	    /// from individual `apply_one` calls are logged and skipped (per-tx
   302	    /// rejection does NOT halt the sequencer). Returns when the queue is
   303	    /// closed and drained.
   304	    pub async fn run(
   305	        &self,
   306	        mut queue_rx: tokio::sync::mpsc::Receiver<TypedTx>,
   307	    ) -> Result<(), SequencerError> {
   308	        while let Some(tx) = queue_rx.recv().await {
   309	            // Stub state: dispatch returns NotYetImplemented; apply_one
   310	            // bubbles up. We log and continue per spec § 3 v1.2 ordering rule
   311	            // (rejection does not consume a logical_t — see K1).
   312	            if let Err(e) = self.apply_one(tx) {
   313	                log::debug!("sequencer apply_one rejected: {e}");
   314	            }
   315	        }
   316	        Ok(())
   317	    }
   318	
   319	    /// Per-tx critical section. Pure transition + CAS put + sign + commit +
   320	    /// Q_t mutation. See spec § 3 stages 1-9.
   321	    ///
   322	    /// **v1.1 C-2 closure (Codex bundle Q-B)**: `next_logical_t` advances
   323	    /// **only on commit success** — the original spec § 3 stage-4
   324	    /// `fetch_add(1)` happened BEFORE sign + writer.commit, so any infra
   325	    /// failure (sign / commit) left `next_logical_t` advanced past a
   326	    /// logical_t that was never written to the ledger. The next accepted
   327	    /// tx would then be assigned a logical_t the writer rejects forever
   328	    /// (writer enforces strict `len + 1`). Fixed by `load → use → store
   329	    /// after commit succeeds`. Single-writer per spec § 5.2.1 makes the
   330	    /// load+store atomic enough; if multi-writer ever lands the AtomicU64
   331	    /// can be upgraded to a `compare_exchange` reservation pattern.
   332	    pub(crate) fn apply_one(&self, tx: TypedTx) -> Result<LedgerEntry, ApplyError> {
   333	        // Stage 1: snapshot Q_t under read lock.
   334	        let q_snapshot = {
   335	            let g = self.q.read().map_err(|_| ApplyError::QStateLockPoisoned)?;
   336	            g.clone()
   337	        };
   338	
   339	        // Stage 2: dispatch (pure). On reject (incl. NotYetImplemented stub),
   340	        // EARLY RETURN. K1: no logical_t consumed.
   341	        let (q_next, _signals) = dispatch_transition(
   342	            &q_snapshot,
   343	            &tx,
   344	            &self.predicate_registry,
   345	            &self.tool_registry,
   346	        )?;
   347	
   348	        // v1.1 C-2: TENTATIVE logical_t (do NOT fetch_add yet).
   349	        let logical_t = self.next_logical_t.load(Ordering::SeqCst) + 1;
   350	
   351	        // Stage 3: put payload to CAS. DIV-5 5-param put signature.
   352	        let payload_bytes = canonical_encode(&tx)
   353	            .map_err(|e| ApplyError::PayloadEncode(e.to_string()))?;
   354	        let payload_cid = {
   355	            let mut cas_w = self.cas.write().map_err(|_| ApplyError::QStateLockPoisoned)?;
   356	            cas_w.put(
   357	                &payload_bytes,
   358	                ObjectType::ProposalPayload,
   359	                &format!("sequencer-epoch-{}", self.epoch.get()),
   360	                logical_t,
   361	                Some("TypedTx.v1".to_string()),
   362	            )?
   363	        };
   364	
   365	        // Stage 5: build LedgerEntrySigningPayload (v1.1 — stage 4 fetch_add

exec
/bin/bash -lc "nl -ba src/top_white/predicates/registry.rs | sed -n '1,160p'; nl -ba src/top_white/predicates/visibility.rs | sed -n '1,80p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	//! L1 Predicate Registry — typed metadata store per WP § 5.L1 + spec v1.4 § 1.5.
     2	//!
     3	//! Constitution authority:
     4	//! - Inv 6 (predicate-gated transition): un-passed work_tx does NOT advance state
     5	//! - Inv 10 (signal vs evaluator): private/commit-reveal predicates SHIELDED from agent view
     6	//! - Const Art III.4: Goodhart shield via three visibility classes
     7	//!
     8	//! Spec authority:
     9	//! - STATE_TRANSITION_SPEC v1.4 § 4 invariants I-PRED-GATE + I-NORANDOM bound to this registry
    10	//! - § 2 hidden inputs: BTreeMap (not HashMap) for deterministic iteration order
    11	//!
    12	//! v4 first iteration: typed metadata + register/lookup + Merkle root computation.
    13	//! Predicate EXECUTION (running the actual predicate code on a work_tx) lives in `runner` (future atom CO1.5.6).
    14	//!
    15	//! /// TRACE_MATRIX WP-arch-§5.L1 + Inv-6 + Inv-10: PredicateRegistry
    16	
    17	use serde::{Deserialize, Serialize};
    18	use sha2::{Digest, Sha256};
    19	use std::collections::BTreeMap;
    20	
    21	use super::visibility::Visibility;
    22	
    23	/// Whether a predicate is fail-closed (Safety) or fail-open-with-signal (Creation).
    24	/// Per WP § 7.2.
    25	#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    26	pub enum SafetyOrCreation {
    27	    /// Fail-closed: rejected work_tx does NOT advance state_root.
    28	    Safety,
    29	    /// Fail-open-with-signal: rejected work_tx still produces a signal but does not advance state.
    30	    /// (In v4, both behave identically at the state-transition level; difference matters at signal layer.)
    31	    Creation,
    32	}
    33	
    34	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    35	pub struct PredicateMetadata {
    36	    /// Stable identifier; e.g., "lean4_oracle".
    37	    pub predicate_id: String,
    38	    /// Schema/code-hash version.
    39	    pub version: u32,
    40	    /// SHA-256 of compiled bytecode or canonical source.
    41	    pub code_hash: [u8; 32],
    42	    /// JSON Schema (or type ID) describing input shape.
    43	    pub input_schema: String,
    44	    /// JSON Schema describing output shape.
    45	    pub output_schema: String,
    46	    /// Goodhart visibility class.
    47	    pub visibility: Visibility,
    48	    /// Owner (agent_id or "system").
    49	    pub owner: String,
    50	    /// SHA-256 of conformance test suite committed alongside.
    51	    pub test_suite_hash: [u8; 32],
    52	    /// Fail-closed (Safety) or fail-open-with-signal (Creation).
    53	    pub safety_class: SafetyOrCreation,
    54	}
    55	
    56	impl PredicateMetadata {
    57	    /// Canonical hash of this metadata for Merkle tree inclusion.
    58	    /// Bincode-style; mirrors STATE_TRANSITION_SPEC § 2.5 canonical serialization rule
    59	    /// (BTreeMap key order is irrelevant here since fields are fixed-order in struct).
    60	    pub fn canonical_hash(&self) -> [u8; 32] {
    61	        // Manual canonical serialization for v1; matches spec § 2.5 deterministic format.
    62	        // (Avoiding bincode dep in lib for now; upgrade later if v1.4 conformance test demands.)
    63	        let mut h = Sha256::new();
    64	        h.update(self.predicate_id.as_bytes());
    65	        h.update(self.version.to_be_bytes());
    66	        h.update(self.code_hash);
    67	        h.update(self.input_schema.as_bytes());
    68	        h.update(self.output_schema.as_bytes());
    69	        h.update(serde_json::to_vec(&self.visibility).expect("visibility serialize"));
    70	        h.update(self.owner.as_bytes());
    71	        h.update(self.test_suite_hash);
    72	        h.update(serde_json::to_vec(&self.safety_class).expect("safety_class serialize"));
    73	        h.finalize().into()
    74	    }
    75	}
    76	
    77	/// L1 PredicateRegistry — a deterministic ordered store of predicate metadata.
    78	///
    79	/// Uses BTreeMap (not HashMap) per spec § 2 I-BTREE invariant.
    80	#[derive(Debug, Clone, Default, Serialize, Deserialize)]
    81	pub struct PredicateRegistry {
    82	    predicates: BTreeMap<String, PredicateMetadata>,
    83	}
    84	
    85	#[derive(Debug, PartialEq, Eq)]
    86	pub enum RegisterError {
    87	    /// A predicate with this id already registered (use `replace` for explicit replacement).
    88	    DuplicateId(String),
    89	    /// Empty / malformed predicate_id.
    90	    InvalidId(String),
    91	}
    92	
    93	impl PredicateRegistry {
    94	    pub fn new() -> Self {
    95	        Self {
    96	            predicates: BTreeMap::new(),
    97	        }
    98	    }
    99	
   100	    /// Register a NEW predicate. Returns DuplicateId if id exists.
   101	    pub fn register(&mut self, meta: PredicateMetadata) -> Result<(), RegisterError> {
   102	        if meta.predicate_id.is_empty() {
   103	            return Err(RegisterError::InvalidId(meta.predicate_id));
   104	        }
   105	        if self.predicates.contains_key(&meta.predicate_id) {
   106	            return Err(RegisterError::DuplicateId(meta.predicate_id));
   107	        }
   108	        self.predicates.insert(meta.predicate_id.clone(), meta);
   109	        Ok(())
   110	    }
   111	
   112	    /// Lookup by predicate_id.
   113	    pub fn get(&self, id: &str) -> Option<&PredicateMetadata> {
   114	        self.predicates.get(id)
   115	    }
   116	
   117	    /// Total count of registered predicates.
   118	    pub fn len(&self) -> usize {
   119	        self.predicates.len()
   120	    }
   121	
   122	    /// Whether the registry is empty.
   123	    pub fn is_empty(&self) -> bool {
   124	        self.predicates.is_empty()
   125	    }
   126	
   127	    /// Compute Merkle-style root over all registered predicates' canonical hashes.
   128	    /// Returns sha256 of empty bytes if registry is empty (matches spec § 5.L1 EMPTY_TREE_ROOT).
   129	    pub fn merkle_root(&self) -> [u8; 32] {
   130	        let mut h = Sha256::new();
   131	        // BTreeMap iterates in lexicographic key order — deterministic.
   132	        for (_id, meta) in &self.predicates {
   133	            h.update(meta.canonical_hash());
   134	        }
   135	        h.finalize().into()
   136	    }
   137	
   138	    /// Agent-visible projection of the registry (Goodhart shield per Inv 10).
   139	    /// Returns a NEW registry containing only Public predicates + commit-reveal that have reveal-time passed.
   140	    pub fn agent_visible_view(&self, now: u64) -> Self {
   141	        Self {
   142	            predicates: self
   143	                .predicates
   144	                .iter()
   145	                .filter(|(_, m)| m.visibility.content_visible_to_agent(now))
   146	                .map(|(k, v)| (k.clone(), v.clone()))
   147	                .collect(),
   148	        }
   149	    }
   150	}
   151	
   152	#[cfg(test)]
   153	mod tests {
   154	    use super::*;
   155	
   156	    fn sample_meta(id: &str, vis: Visibility) -> PredicateMetadata {
   157	        PredicateMetadata {
   158	            predicate_id: id.to_string(),
   159	            version: 1,
   160	            code_hash: [0xab; 32],
     1	//! Predicate visibility policy (Goodhart shield per Const Art III.4 + WP § 9.4).
     2	//!
     3	//! Three classes:
     4	//! - `Public`: schema + permission + basic tests visible to agent
     5	//! - `Private`: hidden benchmarks; agent CANNOT read source/results beyond pass/fail
     6	//! - `CommitReveal`: hash committed first; sample revealed at fixed logical_t
     7	//!
     8	//! /// TRACE_MATRIX Const-Art-III.4 + Inv-10 + WP-spec-§1.5: Goodhart visibility
     9	
    10	use serde::{Deserialize, Serialize};
    11	
    12	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    13	pub enum Visibility {
    14	    /// Agent sees full predicate metadata + schema + can read public test cases.
    15	    Public,
    16	    /// Agent sees only pass/fail outcome. Predicate source + test corpus private.
    17	    /// Used for hidden benchmarks that prevent Goodhart attacks.
    18	    Private,
    19	    /// Predicate hash is published; sample revealed at `reveal_at_logical_t`.
    20	    /// Until reveal, agents cannot test against it; after reveal, becomes Public.
    21	    CommitReveal { reveal_at_logical_t: u64, predicate_hash: [u8; 32] },
    22	}
    23	
    24	impl Visibility {
    25	    /// Whether the predicate's content (source / tests / inputs) is visible to agents NOW.
    26	    /// `now` is the current logical_t.
    27	    pub fn content_visible_to_agent(&self, now: u64) -> bool {
    28	        match self {
    29	            Self::Public => true,
    30	            Self::Private => false,
    31	            Self::CommitReveal { reveal_at_logical_t, .. } => now >= *reveal_at_logical_t,
    32	        }
    33	    }
    34	
    35	    /// Whether the predicate's commit hash (only) is visible.
    36	    /// All visibility classes expose at least the hash.
    37	    pub fn hash_visible_to_agent(&self) -> bool {
    38	        true
    39	    }
    40	}
    41	
    42	#[cfg(test)]
    43	mod tests {
    44	    use super::*;
    45	
    46	    #[test]
    47	    fn public_always_visible() {
    48	        let v = Visibility::Public;
    49	        assert!(v.content_visible_to_agent(0));
    50	        assert!(v.content_visible_to_agent(u64::MAX));
    51	        assert!(v.hash_visible_to_agent());
    52	    }
    53	
    54	    #[test]
    55	    fn private_never_content_visible() {
    56	        let v = Visibility::Private;
    57	        assert!(!v.content_visible_to_agent(0));
    58	        assert!(!v.content_visible_to_agent(u64::MAX));
    59	        // hash still visible
    60	        assert!(v.hash_visible_to_agent());
    61	    }
    62	
    63	    #[test]
    64	    fn commit_reveal_pre_reveal() {
    65	        let v = Visibility::CommitReveal {
    66	            reveal_at_logical_t: 1000,
    67	            predicate_hash: [0u8; 32],
    68	        };
    69	        assert!(!v.content_visible_to_agent(999));
    70	        assert!(v.content_visible_to_agent(1000));
    71	        assert!(v.content_visible_to_agent(1001));
    72	    }
    73	
    74	    #[test]
    75	    fn serde_round_trip_public() {
    76	        let v = Visibility::Public;
    77	        let s = serde_json::to_string(&v).unwrap();
    78	        let v2: Visibility = serde_json::from_str(&s).unwrap();
    79	        assert_eq!(v, v2);
    80	    }

exec
/bin/bash -lc "nl -ba src/state/q_state.rs | sed -n '24,38p;96,106p;306,318p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
    24	// Newtype primitives — minimal, deterministic, serde-ready.
    25	// ────────────────────────────────────────────────────────────────────────────
    26	
    27	/// TRACE_MATRIX § 1.1 — generic 32-byte hash (sha256). State / ledger / registry roots.
    28	#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
    29	pub struct Hash(pub [u8; 32]);
    30	
    31	impl Hash {
    32	    /// TRACE_MATRIX § 1.1 — additive identity (genesis state-root, ledger-root, etc.).
    33	    pub const ZERO: Hash = Hash([0u8; 32]);
    34	
    35	    /// TRACE_MATRIX § 1.1 — construct from a 32-byte digest (sha256 output).
    36	    pub fn from_bytes(b: [u8; 32]) -> Self {
    37	        Hash(b)
    38	    }
    96	
    97	// ────────────────────────────────────────────────────────────────────────────
    98	// AgentVisibleProjection — Inv 10 Goodhart shield (CO P2.7 visibility runtime).
    99	// ────────────────────────────────────────────────────────────────────────────
   100	
   101	/// TRACE_MATRIX § 1.1 — agent-visible projection of tape filtered by per-agent
   102	/// visibility policy (Inv 10 Goodhart shield; `top_white::predicates::visibility`).
   103	///
   104	/// `views`: per-agent filtered head pointer; full filtering machinery lands in CO P2.7.
   105	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   106	pub struct AgentVisibleProjection {
   306	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   307	pub struct QState {
   308	    /// Agent swarm sub-state (tape head per agent + per-agent reputation snapshots).
   309	    pub q_t: AgentSwarmState,
   310	    /// Current ChainTape head pointer = git commit SHA in Path B substrate.
   311	    pub head_t: NodeId,
   312	    /// Materialized state Merkle root (git tree root in Path B).
   313	    pub state_root_t: Hash,
   314	    /// Agent-visible projection of tape filtered by per-agent visibility policy.
   315	    pub tape_view_t: AgentVisibleProjection,
   316	    /// L4 Transition Ledger root (Merkle root of all accepted tx so far).
   317	    pub ledger_root_t: Hash,
   318	    /// L1 Predicate Registry root.

exec
/bin/bash -lc "nl -ba src/state/typed_tx.rs | sed -n '600,630p;835,850p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   600	// § 6 TypedTx outer enum
   601	// ────────────────────────────────────────────────────────────────────────────
   602	
   603	/// TRACE_MATRIX § 8 dispatch_transition — typed-tx outer enum.
   604	/// 7 variants (K5 closed: NO `Slash`). All variants are defined in this
   605	/// module (`state::typed_tx`); v1.1 P3 migrated `TerminalSummaryTx` here
   606	/// from a 3-field placeholder previously in `system_keypair.rs`.
   607	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
   608	pub enum TypedTx {
   609	    Work(WorkTx),
   610	    Verify(VerifyTx),
   611	    Challenge(ChallengeTx),
   612	    Reuse(ReuseTx),
   613	    FinalizeReward(FinalizeRewardTx),
   614	    TaskExpire(TaskExpireTx),
   615	    TerminalSummary(TerminalSummaryTx),
   616	}
   617	
   618	impl TypedTx {
   619	    /// Project to the [`TxKind`] discriminator stored in `LedgerEntry.tx_kind`.
   620	    pub fn tx_kind(&self) -> crate::bottom_white::ledger::transition_ledger::TxKind {
   621	        use crate::bottom_white::ledger::transition_ledger::TxKind;
   622	        match self {
   623	            Self::Work(_) => TxKind::Work,
   624	            Self::Verify(_) => TxKind::Verify,
   625	            Self::Challenge(_) => TxKind::Challenge,
   626	            Self::Reuse(_) => TxKind::Reuse,
   627	            Self::FinalizeReward(_) => TxKind::FinalizeReward,
   628	            Self::TaskExpire(_) => TxKind::TaskExpire,
   629	            Self::TerminalSummary(_) => TxKind::TerminalSummary,
   630	        }
   835	///
   836	/// **v1.2 round-2 closure (R2-1)**: `Finalize.claim_id` is `ClaimId` (was `TxId`
   837	/// in v1.1; round-2 caught the missed call site that leaked the old type
   838	/// through `SignalBundle::finalize`).
   839	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
   840	pub enum SignalKind {
   841	    Empty,
   842	    Finalize {
   843	        claim_id: ClaimId,
   844	        reward: MicroCoin,
   845	    },
   846	    TaskExpired {
   847	        task_id: TaskId,
   848	        bounty_refunded: MicroCoin,
   849	    },
   850	    TerminalSummary {

exec
/bin/bash -lc "nl -ba src/bottom_white/cas/schema.rs | sed -n '1,120p'; nl -ba src/bottom_white/cas/store.rs | sed -n '150,210p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	//! CAS object schema per WP architecture § 5.L3.
     2	//!
     3	//! /// TRACE_MATRIX WP-arch-§5.L3: CAS object schema
     4	
     5	use serde::{Deserialize, Serialize};
     6	use sha2::{Digest, Sha256};
     7	
     8	/// Content-addressed identifier — sha256 of payload bytes.
     9	///
    10	/// Distinct from git's SHA-1 OID (which is an internal storage detail of
    11	/// the git2-rs backend). `Cid` is the v4-canonical identifier; spec § 1.2
    12	/// `WorkTx.proposal_cid: Cid` references this.
    13	#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
    14	pub struct Cid(pub [u8; 32]);
    15	
    16	impl Cid {
    17	    /// Compute Cid from content bytes.
    18	    pub fn from_content(content: &[u8]) -> Self {
    19	        let mut h = Sha256::new();
    20	        h.update(content);
    21	        Self(h.finalize().into())
    22	    }
    23	
    24	    /// Hex-encoded representation (lowercase; 64 chars).
    25	    pub fn hex(&self) -> String {
    26	        let mut s = String::with_capacity(64);
    27	        for b in self.0 {
    28	            s.push_str(&format!("{:02x}", b));
    29	        }
    30	        s
    31	    }
    32	}
    33	
    34	impl std::fmt::Display for Cid {
    35	    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    36	        write!(f, "cid:{}", self.hex())
    37	    }
    38	}
    39	
    40	/// Type tag for CAS objects (replaces inline string-typed kind).
    41	#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    42	pub enum ObjectType {
    43	    /// Agent's work_tx proposal payload (Lean proof, code patch, etc.).
    44	    ProposalPayload,
    45	    /// Challenger's counterexample for a slashed claim.
    46	    CounterexamplePayload,
    47	    /// Predicate bytecode (Lean tactic, WASM module, Rust source bytes).
    48	    PredicateBytecode,
    49	    /// Tool bytecode.
    50	    ToolBytecode,
    51	    /// Constitution diff (for amendment proposals).
    52	    AmendmentDiff,
    53	    /// Reversibility plan attached to a meta_tx.
    54	    ReversibilityPlan,
    55	    /// Generic / unclassified blob.
    56	    Generic,
    57	}
    58	
    59	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    60	pub struct CasObjectMetadata {
    61	    /// Content-addressed identifier (sha256 of content).
    62	    pub cid: Cid,
    63	    /// Backend-specific OID (git sha-1 for git2-rs backend); informational only.
    64	    /// Different backends may have different OID schemes; Cid is canonical.
    65	    pub backend_oid_hex: String,
    66	    pub object_type: ObjectType,
    67	    /// Submitter / author. Use "system" for runtime-emitted objects.
    68	    pub creator: String,
    69	    /// Logical time at insertion (assigned by sequencer; not wall clock).
    70	    pub created_at_logical_t: u64,
    71	    /// Optional schema identifier (JSON Schema URI, type tag, etc.).
    72	    pub schema_id: Option<String>,
    73	    /// Size of content in bytes (informational; not part of canonical hash).
    74	    pub size_bytes: u64,
    75	}
    76	
    77	impl CasObjectMetadata {
    78	    /// Canonical hash of metadata for Merkle tree inclusion.
    79	    pub fn canonical_hash(&self) -> [u8; 32] {
    80	        let mut h = Sha256::new();
    81	        h.update(self.cid.0);
    82	        h.update(self.backend_oid_hex.as_bytes());
    83	        h.update(serde_json::to_vec(&self.object_type).expect("object_type serialize"));
    84	        h.update(self.creator.as_bytes());
    85	        h.update(self.created_at_logical_t.to_be_bytes());
    86	        if let Some(s) = &self.schema_id {
    87	            h.update(b"\x01");
    88	            h.update(s.as_bytes());
    89	        } else {
    90	            h.update(b"\x00");
    91	        }
    92	        h.update(self.size_bytes.to_be_bytes());
    93	        h.finalize().into()
    94	    }
    95	}
    96	
    97	#[cfg(test)]
    98	mod tests {
    99	    use super::*;
   100	
   101	    #[test]
   102	    fn cid_from_empty_content() {
   103	        let cid = Cid::from_content(b"");
   104	        // SHA-256 of empty input = e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
   105	        assert_eq!(
   106	            cid.hex(),
   107	            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
   108	        );
   109	    }
   110	
   111	    #[test]
   112	    fn cid_deterministic() {
   113	        let cid_a = Cid::from_content(b"hello");
   114	        let cid_b = Cid::from_content(b"hello");
   115	        assert_eq!(cid_a, cid_b);
   116	    }
   117	
   118	    #[test]
   119	    fn cid_differs_on_content() {
   120	        let cid_a = Cid::from_content(b"hello");
   150	        let _repo = match Repository::open(&repo_path) {
   151	            Ok(r) => r,
   152	            Err(_) => Repository::init(&repo_path)?,
   153	        };
   154	        let index = load_index_from_sidecar(&repo_path)?;
   155	        Ok(Self { repo_path, index })
   156	    }
   157	
   158	    fn open_repo(&self) -> Result<Repository, CasError> {
   159	        Repository::open(&self.repo_path).map_err(CasError::from)
   160	    }
   161	
   162	    /// Store content; returns its Cid. Idempotent — same content → same Cid.
   163	    pub fn put(
   164	        &mut self,
   165	        content: &[u8],
   166	        object_type: ObjectType,
   167	        creator: &str,
   168	        created_at_logical_t: u64,
   169	        schema_id: Option<String>,
   170	    ) -> Result<Cid, CasError> {
   171	        let cid = Cid::from_content(content);
   172	        let repo = self.open_repo()?;
   173	        let git_oid = repo.blob(content)?;
   174	
   175	        // If already in index, idempotent: just return Cid (content addressing
   176	        // guarantees same content → same Cid → already present)
   177	        if self.index.contains_key(&cid) {
   178	            return Ok(cid);
   179	        }
   180	
   181	        let metadata = CasObjectMetadata {
   182	            cid,
   183	            backend_oid_hex: git_oid.to_string(),
   184	            object_type,
   185	            creator: creator.to_string(),
   186	            created_at_logical_t,
   187	            schema_id,
   188	            size_bytes: content.len() as u64,
   189	        };
   190	        // CO1.4-extra: durably append BEFORE inserting into in-memory index
   191	        // (so a crash mid-write leaves the runtime in a consistent state —
   192	        // either the entry is durably recorded AND in-memory, or neither).
   193	        append_to_sidecar(&self.repo_path, &metadata)?;
   194	        self.index.insert(cid, metadata);
   195	        Ok(cid)
   196	    }
   197	
   198	    /// Retrieve content by Cid. Verifies content sha256 matches Cid (corruption check).
   199	    pub fn get(&self, cid: &Cid) -> Result<Vec<u8>, CasError> {
   200	        let metadata = self
   201	            .index
   202	            .get(cid)
   203	            .ok_or(CasError::CidNotFound(*cid))?;
   204	        let repo = self.open_repo()?;
   205	        let git_oid = git2::Oid::from_str(&metadata.backend_oid_hex)
   206	            .map_err(CasError::Git2)?;
   207	        let blob = repo.find_blob(git_oid)?;
   208	        let content = blob.content().to_vec();
   209	
   210	        // Verify content sha256 matches Cid (defense against corruption).

exec
/bin/bash -lc "nl -ba src/bottom_white/mod.rs | sed -n '1,80p'; find src/bottom_white -maxdepth 2 -type d | sort" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	//! Bottom White layer (per Anti-Oreo § 3 / Constitution Art I.1).
     2	//!
     3	//! Deterministic, append-only substrate. tape, CAS, ledger, sandbox, materializer, tools.
     4	//! v4 first iteration: tools module only (rest land in subsequent atoms).
     5	//!
     6	//! /// TRACE_MATRIX Const-Art-I.1 + WP-arch-§3 + WP-arch-§5.L0-L6: Bottom White layer root
     7	
     8	pub mod cas;
     9	/// TRACE_MATRIX FC1-Sig+FC3-Sig: Bottom White ledger crypto modules.
    10	pub mod ledger;
    11	pub mod tools;
src/bottom_white
src/bottom_white/cas
src/bottom_white/ledger
src/bottom_white/tools

exec
/bin/bash -lc "nl -ba handover/audits/CO1_7_5_DUAL_AUDIT_VERDICT_R1_2026-04-29.md | sed -n '1,140p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	# CO1.7.5 Dual External Audit — Round-1 Merged Verdict
     2	
     3	**Date**: 2026-04-29
     4	**Target**: `CO1_7_5_TRANSITION_BODIES_AND_RUNTIME_WIRING_v1_2026-04-29.md` at HEAD `334111a`
     5	**Audits**: Codex r1 (`CODEX_CO1_7_5_ROUND1_AUDIT_2026-04-29.md`) + Gemini r1 (`GEMINI_CO1_7_5_ROUND1_AUDIT_2026-04-29.md`)
     6	
     7	## Verdict matrix
     8	
     9	| Audit | Verdict | Conviction | Q breakdown |
    10	|---|---|---|---|
    11	| Codex | **CHALLENGE** | High | Q-A pass-with-v1.1-ask, Q-B pass-with-overclaim-fix, Q-C CHALLENGE (compile defects), Q-D **CHALLENGE (purity violations)**, Q-E **CHALLENGE (mapping table overclaims)**, Q-F PASS, Q-G CHALLENGE (smoke/patch staleness), Q-H **CHALLENGE (substrate missing)**, Q-I CHALLENGE (impl gating) |
    12	| Gemini | **CHALLENGE** | High | Q1 PASS, Q2 **CHALLENGE (process passive)**, Q3 PASS, Q4 PASS, Q5 PASS, Q6 PASS, Q7 PASS, Q8 vote=`unimplemented!()` |
    13	
    14	**Conservative-merged verdict** (per memory `feedback_dual_audit_conflict`, VETO > CHALLENGE > PASS): **CHALLENGE / High**. No VETO from either audit ("no foundational design flaw" — both explicit).
    15	
    16	## Where the audits agree
    17	
    18	1. § 0.3 STATE supersession framing must take **active** reconciliation responsibility (not delegate to STATE curator). Gemini MF1+MF3 + Codex Q-A v1.1 ask converge.
    19	2. SignalKind 4-variant minimization is a **safe deferral**, not a hazard (Gemini Q5 PASS + Codex Q-F PASS).
    20	3. Combined STEP_B ceremony (one A/B unit covering bus.rs + kernel.rs) is **strategically sound** (Gemini Q4 PASS); Codex agrees the combination is permissible but flags overclaim about Phase 0 binding-vs-advisory (Codex Q-C).
    21	4. Hygiene OBS handling appropriate — inline fix on CLAUDE.md is correct since it's project instructions, not constitution.md (Gemini Q6 PASS).
    22	
    23	## Where the audits disagree
    24	
    25	**Q1 (`head_commit_oid_hex` default impl)**:
    26	- Gemini: `unimplemented!()` (silent stagnation worse — head_t is constitutional anchor)
    27	- Codex: `default { None }` + mandate Git2 override + Git2-backed test (panic-after-commit-success worse)
    28	
    29	**Synthesis** (preserves both safety arguments): default `None` (Codex no-panic) + spec mandates every shipped LedgerWriter override head_commit_oid_hex + add a test that fails if Git2LedgerWriter returns None at commit time (Gemini silent-stagnation defense). The default is then dead code in production, never reached.
    30	
    31	## Where Codex went deeper than Gemini (substantive must-fix)
    32	
    33	Codex performed source-level verification at depth Gemini did not (Codex's prompt was implementer-paranoid; Gemini's was strategic). Findings unique to Codex:
    34	
    35	### M1 (Q-D + Q-H + Q-I — the heavyweight finding): substrate missing
    36	
    37	Codex verified each STATE § 3.x call site against shipped APIs and found the spec assumes infrastructure that **does not exist**:
    38	
    39	| STATE pseudocode reference | Shipped reality | Gap |
    40	|---|---|---|
    41	| `q.economic_state_t.claims_t.get(&tx.target_work_tx).status.allows_verification()` | `ClaimsIndex` = `BTreeMap<TxId, ClaimEntry>` with only `amount` + `claimant` | No `status`, `solver`, `task_id` fields |
    42	| `q.economic_state_t.task_markets_t.get(target.task_id).config.verifier_bond_on_slash` | `TaskMarketEntry` has no `deadline` / `creator` / `config` fields | No config substrate |
    43	| `window.is_open(tx.timestamp_logical)` | `ChallengeCase` lacks `duration` / `outcome` field + `is_open` method | No challenge-window machinery |
    44	| `registry.run_acceptance(tx, q)?` / `run_verification` / `run_counterexample_check` | `PredicateRegistry` exposes only `register/get/root/view` | No execution methods |
    45	| `q_next.economic_state_t.derive_state_root()` | Method does not exist on EconomicState | No state-root derivation |
    46	
    47	These are FC1 (top-white predicate execution) + FC2 (middle-black state-mutation schemas) responsibilities. Putting them inside an FC3 (bottom-white L4 ledger) atom violates Anti-Oreo 三层 separation.
    48	
    49	Per PROJECT_DECISION_MAP § 3.4, the prerequisite substrate is the planned **CO P2.x family** (currently in "Pending CO P2 (after CO P1 exit)"):
    50	- CO P2.1 TaskMarket
    51	- CO P2.2 EscrowVault
    52	- CO P2.3 ContributionLedger
    53	- CO P2.5 ChallengeCourt (challenge-window machinery)
    54	- CO P2.6 SettlementEngine (`issue_provisional`, settlement formula)
    55	- CO P2.7 Agent roles
    56	- CO P2.9 ReputationIndex (`reputations_t.adjust`)
    57	- CO1.11 Safety vs Creation (uses PredicateRegistry — likely supplies execution methods)
    58	
    59	### M2 (Q-D — purity boundary violations)
    60	
    61	Spec § 1 D1 promises 4-arg signature `(&QState, &TxVariant, &PredicateRegistry, &ToolRegistry)` + "no I/O". STATE pseudocode violates this:
    62	- `challenge_transition` reads CAS inside transition (`cas::get(&tx.counterexample_cid)?`) — needs CAS arg
    63	- `emit_terminal_summary_transition` takes `&Runtime`, reads run state, signs inside transition — needs runtime + keypair args
    64	- System signature verification needs `PinnedSystemPubkeys` — not in 4-arg sig
    65	
    66	### M3 (Q-C — D3 compile defects)
    67	
    68	- Bus type is **`TuringBus`** (`src/bus.rs:53`), not `Bus` — spec全文写错
    69	- Kernel derives `Debug, Serialize, Deserialize` (`src/kernel.rs:18`); adding `Option<Arc<Sequencer>>` requires `serde(skip)` + Debug handling; Sequencer has no derives
    70	- Kernel docs as "pure topology" (`src/kernel.rs:15-17`) — Sequencer placement needs stronger justification or move to a runtime layer
    71	
    72	### M6 (Q-E — TransitionError mapping table overclaims)
    73	
    74	Spec Q5 mapping table missed:
    75	- CAS lookup failure in `challenge_transition` (no mapped variant)
    76	- `SettlementEngine::issue_provisional` failure in Work (no mapped variant)
    77	- Runtime / system-signature validation paths for FinalizeRewardTx + TerminalSummaryTx
    78	- Some stale-parent checks for system tx
    79	
    80	### M7 (Q-E — RejectedAttemptSummary side channel not real)
    81	
    82	Spec asserts a side channel for rich rejection context. Codex finds:
    83	- A type at `src/bottom_white/ledger/system_keypair.rs:151-158` exists, but does NOT match STATE shape (`STATE:192-214`)
    84	- Sequencer rejection currently only logs and skips (`src/state/sequencer.rs:252-266`); no rejected-summary stamping path is wired
    85	
    86	### M8 (Q-G — smoke/patch staleness)
    87	
    88	- Footer says smoke ran at `2f5093a` — should be current HEAD `334111a` (smoke ran pre-commit; the spec was committed after, became HEAD)
    89	- Spec § 1 D4 cites `transition_ledger.rs:1451` for the `#[ignore]`; actual location is line `1455` (`1451` is the doc-comment)
    90	- S8 says "18 warnings"; full workspace also emits 1 `gix_capability_spike` warning → "19"
    91	- P3 references "§ 6 ack #8" but § 6 has only 6 items after self-audit dropped duplicates
    92	
    93	## Occam-driven scope decision (executed without further audit input)
    94	
    95	The audit findings reveal the v1 spec was **mis-scoped** by my session: D1 transition bodies were bundled with D2+D3+D4 wiring, but D1 has heavyweight cross-layer substrate dependencies that D2+D3 do not.
    96	
    97	**Decision** (per "无损压缩即智能" + Anti-Oreo + Occam, applied by ArchitectAI without further audit input):
    98	
    99	Split the atom by dependency profile, using existing `CO1.4-extra` pattern as precedent:
   100	
   101	| Atom | Scope | Substrate dependency | Ships when |
   102	|---|---|---|---|
   103	| **CO1.7-extra** (NEW; bridge atom) | D2 head_t close + D3 Sequencer entry-point + 1 D4 test (`cas_payload_round_trip`, substrate-independent) | None — uses only frozen LedgerWriter trait + Sequencer machinery + existing CasStore | Now (small atom; v1.1 fixes M3-M8) |
   104	| **CO1.7.5** (reverts to CO1.7 § 13 original meaning) | D1 transition bodies + 3 D4 tests + un-ignore `sequencer_serial_replay_byte_identity` | CO P2.1 / 2.2 / 2.3 / 2.5 / 2.6 / 2.7 / 2.9 + CO1.11 + (new) PredicateRegistry execution-methods atom | After substrate atoms PASS/PASS |
   105	
   106	### Why this beats the 3 user-presented options under Occam
   107	
   108	| Option | Description-length cost | Anti-Oreo | WP § 5.L4 | Verdict |
   109	|---|---|---|---|---|
   110	| A: CO1.7.5 owns substrate | NEW concept "L4 atom owns FC1/FC2 schemas" | ❌ violates 三层 | ❌ exceeds L4 boundary | NO |
   111	| B (raw): declare blocker; spec stays bundled | NEW concept "all-or-nothing implementation gate" | OK | OK but inefficient | suboptimal |
   112	| C: atom-internal phasing D5 | NEW concept "atom-internal heterogeneous phases" | ❌ if D5 cross-layer | ❌ same | NO |
   113	| **B2 (executed)**: split by dep profile | **0 new concepts** (CO1.4-extra precedent + Anti-Oreo + atom-decomposition) | ✅ each atom in its layer | ✅ L4 atom contains only L4 work | **YES** |
   114	
   115	### What this reveals about LATEST.md
   116	
   117	LATEST.md (commit `2f5093a`) claims "Wave 6 #1 80% complete; CO1.7.5 single critical path". This is **false-precision**. True state:
   118	- L4 wiring (D2+D3): **shipping now via CO1.7-extra v1.1** post round-2 PASS/PASS (~80% → ~85%)
   119	- L4 transition bodies (CO1.7.5 per CO1.7 § 13): **gated on 7+ substrate atoms** in the CO P2.x family
   120	- Wave 6 #1 actual closure: requires CO1.7-extra + CO1.7.5 + CO P2.x family → far from "single critical path"
   121	
   122	LATEST.md should be patched in the same session-cluster to reflect this audit-derived reality.
   123	
   124	## v1.1 patch plan (rolled into CO1.7-extra v1, applied this session)
   125	
   126	| ID | Source | Fix |
   127	|---|---|---|
   128	| **M1** scope | Codex Q-D/H/I + Occam | Atom rescoped to D2+D3 + 1 substrate-independent D4 test. D1 + 3 D4 tests + un-ignore moved to future CO1.7.5 atom (gated). |
   129	| **M2** purity | Codex Q-D | Now N/A for CO1.7-extra (no transition bodies in scope). Will be addressed by future CO1.7.5 spec. |
   130	| **M3a** TuringBus | Codex Q-C | Spec body uses `TuringBus` per `src/bus.rs:53`. |
   131	| **M3b** Kernel derives | Codex Q-C | Spec specifies `#[serde(skip)]` on Kernel.sequencer field + manual Debug; Sequencer.rs adds `#[derive(Debug)]` minimal. |
   132	| **M3c** Sequencer placement | Codex Q-C | v1.1 keeps Sequencer in Kernel; clarifies "pure topology" doc to acknowledge Sequencer as the typed-tx topology element (matching legacy Tape/NodeId pattern). |
   133	| **M4** § 0.3 active reconciliation | Gemini MF1+MF3 + Codex Q-A | § 0.3 commits to filing STATE_TRANSITION_SPEC v1.5 housekeeping issue as part of CO1.7-extra atom closure; asserts downstream-spec supersession principle explicitly. |
   134	| **M5** Q1 synthesis | Both | Default `None` + spec mandates every shipped LedgerWriter override + add test asserting Git2LedgerWriter returns Some at commit time. |
   135	| **M6** mapping table | Codex Q-E | Now N/A (TransitionError mapping is a transition-bodies concern; future CO1.7.5 owns it). |
   136	| **M7** RejectedAttemptSummary claim | Codex Q-E | Now N/A (transition-bodies concern). |
   137	| **M8a** smoke commit cite | Codex Q-G | Footer updated to current HEAD (TBD post-rewrite). |
   138	| **M8b** ignore line cite | Codex Q-G | `1451` → `1455` (doc-comment vs actual `#[ignore]`). |
   139	| **M8c** warning count | Codex Q-G | "18" → "19" (gix_capability_spike spike adds one warning at workspace level). |
   140	| **M8d** P3 stale wording | Codex Q-G | "§ 6 ack #8" → "§ 6 ack" (no #8 after self-audit drop). |

exec
/bin/bash -lc "nl -ba handover/audits/CODEX_CO1_7_5_ROUND1_AUDIT_2026-04-29.md | sed -n '1,150p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	# Codex CO1.7.5 transition_bodies_and_runtime_wiring Round-1 Audit
     2	**Date**: 2026-04-29
     3	**Target**: spec v1 DRAFT (self-audited; pre round-1)
     4	**HEAD**: 334111af8866d84977957163be3ce6517808b195
     5	**Prompt size**: 418334 chars
     6	
     7	---
     8	
     9	Reading prompt from stdin...
    10	OpenAI Codex v0.125.0 (research preview)
    11	--------
    12	workdir: /home/zephryj/projects/turingosv4
    13	model: gpt-5.5
    14	provider: openai
    15	approval: never
    16	sandbox: danger-full-access
    17	reasoning effort: xhigh
    18	reasoning summaries: none
    19	session id: 019dd6be-941e-7c42-b4fe-b4bebee3580d
    20	--------
    21	user
    22	# Codex Adversarial Audit — CO1.7.5 spec v1 DRAFT (Round 1)
    23	
    24	**Role**: skeptical adversarial implementer-reviewer. Independent of Gemini round-1 (running in parallel).
    25	
    26	**Mandate**: round 1 dual external audit on CO1.7.5 spec v1 (self-audited; pre round-1). Per CLAUDE.md "Audit Standard": Generator ≠ Evaluator (Claude generated; you review). Per memory `feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS.
    27	
    28	**Scope clarification**: this is a **spec-only** audit (no skeleton has been written yet — the implementation is gated post-PASS/PASS per CLAUDE.md "Audit Standard"). The smoke verification at § 7 + footer + the 12 patches in the patch log are part of the audit surface. Verify every smoke-cited line/byte against the actual repo at HEAD `334111a`.
    29	
    30	## What you're reviewing
    31	
    32	1. **Spec doc**: `handover/specs/CO1_7_5_TRANSITION_BODIES_AND_RUNTIME_WIRING_v1_2026-04-29.md` (372 lines)
    33	2. **Frozen prior specs (carry-forward authority)**: CO1.7 v1.2 (round-3 PASS/PASS), CO1.1.4-pre1 (PASS/PASS, ABI lock), STATE_TRANSITION_SPEC v1.4 (round-4 PASS/PASS)
    34	3. **Shipped code (target of D1-D4)**: src/state/sequencer.rs (apply_one + dispatch_transition stubs), src/state/typed_tx.rs (frozen TypedTx + 22 TransitionError + 4 SignalKind), src/bottom_white/ledger/transition_ledger.rs (Git2LedgerWriter + InMemoryLedgerWriter), src/state/q_state.rs (Q_t with NodeId tuple struct)
    35	4. **STEP_B-restricted touch surface**: src/bus.rs + src/kernel.rs (currently use legacy crate::ledger; CO1.7.5 D3 adds Sequencer wiring). Hygiene OBS sedimented at `handover/alignment/OBS_STEP_B_RESTRICTED_FILE_LIST_DRIFT_2026-04-29.md`.
    36	
    37	## What is at stake
    38	
    39	- **PASS** unblocks CO1.7.5-impl (final L4 atom; Wave 6 #1 100% closure)
    40	- **CHALLENGE** triggers spec v1.1 patch round
    41	- **VETO** blocks CO1.7.5 implementation entirely until major rework
    42	
    43	## Round 1 audit questions
    44	
    45	**Q-A. Two-supersession framework (§ 0.3)**: spec declares STATE v1.4 § 3 line 412 (NodeId::from_state_root) and STATE v1.4 § 3 lines 403-409 (BoolSignal/StatSignal richness) are "carried forward as superseded by CO1.7 K3 v1.2 + CO1.1.4-pre1". Is this:
    46	- Authority-chain valid? CO1.7 v1.2 is round-3 PASS/PASS (downstream of STATE v1.4 round-4 PASS/PASS). Can a downstream spec's resolution *legitimately* supersede an upstream PASS/PASS line, OR does this require a Phase Z' check / STATE re-audit?
    47	- Code-consistent? `src/state/typed_tx.rs:830-854` ships 4-variant SignalKind (Empty/Finalize/TaskExpired/TerminalSummary) — does the spec's emit table (§ 0.3.2) actually align with the shipped enum?
    48	- Forward-compatible? CO1.9 will extend SignalKind. Does CO1.7.5 carry-forward block / harden / stay-neutral about CO1.9 extension?
    49	
    50	**Q-B. D2 head_t close (§ 1 D2)**: spec proposes `q_w.head_t = state::q_state::NodeId(commit_oid_hex)` after `writer_w.commit(&entry)?` in apply_one stage 9. Verify:
    51	- Atomicity proof real? Spec claims "no failure point between commit success and head_t store under acquired lock". Walk through every line; could the AtomicU64::store, the `*q_w = q_next` move, or the field assignments fail in any way?
    52	- NodeId disambiguation correct? `src/ledger.rs:13` has `pub type NodeId = String` (legacy; imported by bus.rs+kernel.rs); `src/state/q_state.rs:49` has `pub struct NodeId(pub String)` (new tuple struct). q.head_t is the new variant per `q_state.rs:311 pub head_t: NodeId`. Does the `state::q_state::NodeId(commit_oid_hex)` constructor call type-check?
    53	- Trait method `LedgerWriter::head_commit_oid_hex(&self) -> Option<String>` default-None — Q1 OPEN: should the default be `unimplemented!()` instead, forcing every impl to declare? What's the case for / against each?
    54	
    55	**Q-C. D3 combined STEP_B ceremony (§ 1 D3)**: spec rejects per-file STEP_B (Q3 closed) in favor of one A/B unit covering kernel.rs + bus.rs together. Justification: "Bus forwarder is meaningless without Kernel field; STEP_B Phase 0 minimum-sufficient version".
    56	- Sound? Per `STEP_B_PROTOCOL.md` Phase 0, is "minimum sufficient" a binding criterion for ceremony scoping, or is it advisory?
    57	- Architecture: Sequencer field lives in Kernel only; Bus forwards via `self.kernel.sequencer`. Bus stays struct-shape-compatible. Is this the right ownership boundary, or should Sequencer live above Kernel (e.g., in a runtime layer that owns Bus)?
    58	- Risk: combined ceremony fails one A/B byte-identity check (either bus.rs or kernel.rs diverges) — does the spec specify what to do? (Restart whole ceremony, or split-and-redo?)
    59	
    60	**Q-D. D1 transition body purity (§ 1 D1)**: spec § 1 D1 promises every transition body
    61	- takes `(&QState, &TxVariant, &PredicateRegistry, &ToolRegistry)` and returns `Result<(QState, SignalBundle), TransitionError>`
    62	- "no I/O, no env reads, no clock reads, no HashMap iteration, no f64 arithmetic on monetary values"
    63	- mutates `q_next` cloned from `q`; returns byte-identically deterministic across processes
    64	
    65	Verify against STATE § 3 Rust pseudocode: are there hidden non-deterministic deps? E.g., HashMap iteration in `q.economic_state_t.task_markets_t.get(...).map(|tm| tm.config.verifier_bond_on_slash)` (STATE § 3.2 line 537)? f64 in `prediction_market.rs:21-27,87-133`? STATE § 3.3 royalty rounding? Specific calls + lines.
    66	
    67	**Q-E. Q5 mapping table completeness (§ 3.1 closure)**: spec Q5 closure table maps STATE § 3.1-3.7 rejection paths to 22 shipped TransitionError variants. Audit each:
    68	- Are there rejection paths in STATE § 3 / § 3.1 / § 3.2 / § 3.3 / § 3.4 / § 3.6 / § 3.7 that the table missed?
    69	- Are there shipped variants that are NEVER triggered in CO1.7.5? (Dead-variant audit.)
    70	- Minimal-payload pattern: spec asserts "rich context flows via RejectedAttemptSummary side channel". Where IS RejectedAttemptSummary in the codebase? Verify the side-channel is real, not aspirational.
    71	
    72	**Q-F. Q4 SignalKind closure (§ 0.3.2 emit table)**: 4 of 7 transition bodies emit `SignalKind::Empty`. Is "Empty" semantically correct for Work/Verify/Challenge/Reuse, or does this hide observable-state loss (e.g., reputation deltas in STATE § 3.2 line 530-531 silently dropped)?
    73	- If observable-state loss: is CO1.9 the right home for the missing variants, or should CO1.7.5 add 1-2 Bool/Stat variants (forcing a CO1.1.4-pre1 micro-amendment)?
    74	- Determinism: Empty signal vs full signal — both byte-identically deterministic, but the full signal exposes more API surface to L6 indices (CO1.9). Is the v1 minimization safe?
    75	
    76	**Q-G. Spec ↔ smoke / patch consistency**: 12 patches applied (P1-P4 smoke, P5-P12 self-audit). Verify each P5-P12 patch is real in the v1 spec body (no spec ↔ patch-log drift), and verify smoke S1-S8 + footer claims hold against repo HEAD `334111a`.
    77	
    78	**Q-H. New defects**: independent of audit prompt structure, what does the spec still get wrong?
    79	- Spec claims CO1.7-impl is "PASS/PASS-equivalent" at commit `2461fe6`; does this hold? (Pre-implementation gate from CO1.7 § 12 — verify.)
    80	- Spec claims `Git2LedgerWriter::head_commit_oid()` is exposed; verify line cited (`transition_ledger.rs:674`).
    81	- Spec § 5 LoC estimate (1,000-1,560 LoC) — defensible given STATE § 3.x complexity?
    82	
    83	**Q-I. Implementation gating**: assuming all your CHALLENGEs are addressed in v1.1, is the spec implementable end-to-end (i.e., CO1.7.5-impl can be written, audited, merged without scope explosion)? Specific blockers to call out — any v4-blocking deps that aren't in CO1.4-extra / CO1.7-impl / CO1.1.4-pre1 inheritance?
    84	
    85	## Output format
    86	
    87	# Codex CO1.7.5 Round-1 Audit
    88	## Q-A Two-supersession framework
    89	## Q-B D2 head_t close
    90	## Q-C D3 combined STEP_B
    91	## Q-D D1 transition body purity
    92	## Q-E Q5 mapping table completeness
    93	## Q-F Q4 SignalKind closure
    94	## Q-G Spec ↔ smoke/patch consistency
    95	## Q-H New defects
    96	## Q-I Implementation gating
    97	## **VERDICT**: PASS / CHALLENGE / VETO
    98	## Top 3 must-fix (if CHALLENGE)
    99	## Conviction (low/med/high)
   100	
   101	Be rigorous. Cite spec line numbers + source file line numbers when calling defects. Do NOT pass on principle; do NOT veto on principle. Real defects = CHALLENGE; foundational design flaw = VETO; clean spec = PASS.
   102	
   103	---
   104	
   105	
   106	# CO1.7.5 spec v1 (target of audit)
   107	
   108	# CO1.7.5 Transition Bodies + Runtime Wiring v1 — DRAFT (self-audited; pre round-1)
   109	
   110	**Status**: v1 DRAFT (2026-04-29). Smoke 8/8 PASS. Self-audit pass complete (8 patches; § 0.3 / § 1 D2-D3 / § 3 / § 5 / § 6 / § 0.2 rewritten — see footer patch log). Awaiting round-1 dual external audit.
   111	**Author**: ArchitectAI (Claude); session 2026-04-29.
   112	**Supersedes**: nothing (NEW atom; predicted by `CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md` § 13).
   113	**Pre-implementation gate**: this spec must reach **PASS/PASS** dual external audit before any code lands. Sedimented per CLAUDE.md "Audit Standard".
   114	
   115	**Companion specs (frozen, read first)**:
   116	- `CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md` v1.2 — L4 transition ledger + Sequencer + dispatch_transition skeleton (round-3 PASS/PASS); freezes ABI + apply_one machinery.
   117	- `CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md` — frozen 7-variant TypedTx + 13 locked golden hex + 22-variant TransitionError.
   118	- `STATE_TRANSITION_SPEC_v1_2026-04-27.md` v1.4 — pure transition pseudocode for 7 sub-sections § 3 / § 3.1 / § 3.2 / § 3.3 / § 3.4 / § 3.6 / § 3.7 (round-4 PASS/PASS).
   119	- `META_TRANSITION_INTERFACE_v1_2026-04-27.md` — trait pattern (deferred runtime to v4.1).
   120	
   121	**Single sentence**: implement the 7 per-kind transition function bodies (currently `Err(NotYetImplemented)` stubs), close the G-1 carry-forward `q.head_t = NodeId(commit_oid_hex)` wiring after `Git2LedgerWriter.commit`, perform STEP_B parallel-branch ceremony for the bus.rs / kernel.rs Sequencer entry-point, and un-ignore `sequencer_serial_replay_byte_identity` so the byte-identity I-DETHASH witness fires end-to-end.
   122	
   123	---
   124	
   125	## § 0 Status + dependency map
   126	
   127	### 0.1 What this atom inherits (frozen)
   128	| Frozen by | Surface | Why CO1.7.5 cannot change it |
   129	|---|---|---|
   130	| CO1.1.4-pre1 (commit `c1226e2`) | 7-variant `TypedTx` + 22-variant `TransitionError` (incl. `NotYetImplemented`) + 13 locked golden hex | ABI-locked; behavior change = re-audit + golden invalidation |
   131	| CO1.7-impl A1 (commit `2461fe6`) | `LedgerEntry` / `LedgerEntrySigningPayload` 9-field signing surface; `Git2LedgerWriter` + `InMemoryLedgerWriter`; `head_commit_oid()` accessor; `transition_ledger_emitter::sign_ledger_entry` | C3 wired in code; head_commit_oid already exposed |
   132	| CO1.7-impl A2+A3 (commit `2461fe6`) | `Sequencer` 9-stage `apply_one` + `dispatch_transition` exhaustive match | structural correctness locked; only per-variant arms change |
   133	| CO1.7-impl A4 (commit `2461fe6`) | `replay_full_transition` 9-stage I-DETHASH witness | replay fixture path locked |
   134	| CO1.4-extra (commit `b6b7574`) | CAS sidecar JSONL index persistence | cold-restart full-replay path unblocked |
   135	
   136	### 0.2 What this atom delivers (new)
   137	1. **D1 — 7 per-kind transition function bodies** translating `STATE_TRANSITION_SPEC § 3 / § 3.1-3.4 / § 3.6 / § 3.7` pseudocode into deterministic pure Rust, with two CO1.7-K3-v1.2 / CO1.1.4-pre1 supersessions carried forward (§ 0.3).
   138	2. **D2 — G-1 head_t close** at Sequencer post-commit: `q.head_t = state::q_state::NodeId(commit_oid_hex)` after `writer.commit(&entry)` returns Ok. Requires one additive `LedgerWriter` trait method (`head_commit_oid_hex`).
   139	3. **D3 — Combined STEP_B ceremony** for `src/kernel.rs` (Sequencer field) + `src/bus.rs` (forwarder method) as one A/B unit. Sequencer instance lives in Kernel only; Bus forwards via `self.kernel.sequencer`. Coexists with legacy `crate::ledger`; full retirement is CO1.1.5.
   140	4. **D4 — Un-ignore `sequencer_serial_replay_byte_identity`** + 3 NEW CO1.7.5+ stage tests already declared in `CO1_7 § 7` table.
   141	
   142	### 0.3 Two STATE_TRANSITION_SPEC § 3 supersessions adopted by CO1.7.5
   143	
   144	CO1.7.5 inherits two prior minimization decisions that diverge from STATE § 3 v1.4 pseudocode. Both were ratified by downstream specs reaching dual-audit PASS/PASS; CO1.7.5 carries them forward unchanged. Re-audit gating for the STATE spec itself is **not** decided here — CO1.7.5 spec only documents the carry-forward; whether STATE_TRANSITION_SPEC needs a v1.5 housekeeping commit is a separate decision for the STATE spec curator.
   145	
   146	#### 0.3.1 head_t mutation site
   147	| Source | Says | Authority |
   148	|---|---|---|
   149	| STATE § 3 line 412 (and parallel lines in § 3.1 line 467, § 3.2 line 561) | `q_next.head_t = NodeId::from_state_root(new_state_root)` inside the pure transition body | STATE v1.4 round-4 PASS/PASS |
   150	| CO1.7 K3 v1.2 § 5 | "`NodeId::from_state_root(...)` is NOT used by L4 in any version"; head_t mutation deferred to CO1.7.5+ Sequencer post-commit | CO1.7 v1.2 round-3 PASS/PASS |

exec
/bin/bash -lc "rg -n \"CO1\\.8|CO1.8|Materialized State|Wave 6|Next-session|CO1\\.13|CO1.13\" handover/ai-direct/LATEST.md handover/architect-insights/SPRINT_DEPENDENCY_GRAPH_v1_2026-04-27.md handover/audits/CO1_7_5_DUAL_AUDIT_VERDICT_R1_2026-04-29.md" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
handover/audits/CO1_7_5_DUAL_AUDIT_VERDICT_R1_2026-04-29.md:117:LATEST.md (commit `2f5093a`) claims "Wave 6 #1 80% complete; CO1.7.5 single critical path". This is **false-precision**. True state:
handover/audits/CO1_7_5_DUAL_AUDIT_VERDICT_R1_2026-04-29.md:120:- Wave 6 #1 actual closure: requires CO1.7-extra + CO1.7.5 + CO P2.x family → far from "single critical path"
handover/audits/CO1_7_5_DUAL_AUDIT_VERDICT_R1_2026-04-29.md:161:3. **LATEST.md**: should be patched to reflect audit-derived true state of Wave 6 #1 (~30-40%, not 80%)
handover/architect-insights/SPRINT_DEPENDENCY_GRAPH_v1_2026-04-27.md:96:    └─ blocks: CO1.8 (materializer)
handover/architect-insights/SPRINT_DEPENDENCY_GRAPH_v1_2026-04-27.md:108:    └─ blocks: CO1.8, CO1.9, all CO P2
handover/architect-insights/SPRINT_DEPENDENCY_GRAPH_v1_2026-04-27.md:109:[CO1.8]       Materialized State (8 atoms)
handover/architect-insights/SPRINT_DEPENDENCY_GRAPH_v1_2026-04-27.md:113:    └─ blockedBy: CO1.8
handover/architect-insights/SPRINT_DEPENDENCY_GRAPH_v1_2026-04-27.md:129:[CO1.13]      TRACE_MATRIX_v3 implementation (3 atoms incl R-022 hook)
handover/architect-insights/SPRINT_DEPENDENCY_GRAPH_v1_2026-04-27.md:152:  [CO1.8.*]   Materializer 8 atoms (each is an independent index)
handover/architect-insights/SPRINT_DEPENDENCY_GRAPH_v1_2026-04-27.md:159:  [CO1.13]    TRACE_MATRIX implementation (continuous)
handover/architect-insights/SPRINT_DEPENDENCY_GRAPH_v1_2026-04-27.md:187:[CO1.8 materializer + CO1.9 signal index + CO1.10 + CO1.11] (parallel; ~3 wk)
handover/architect-insights/SPRINT_DEPENDENCY_GRAPH_v1_2026-04-27.md:189:[CO1.13 TRACE_MATRIX impl] (1 wk; can start earlier in parallel)
handover/architect-insights/SPRINT_DEPENDENCY_GRAPH_v1_2026-04-27.md:335:    P1_PARA[CO1.8 + 1.9 + 1.10 + 1.11<br/>parallel; ~3 wk]:::wide
handover/architect-insights/SPRINT_DEPENDENCY_GRAPH_v1_2026-04-27.md:336:    P1_TRACE[CO1.13 TRACE_MATRIX impl]:::wide
handover/ai-direct/LATEST.md:18:**ChainTape Directive**: 项目全面向区块链前进 = ChainTape vertical (**Trust Anchor Layer 0 + ChainTape Layers 1–6**) becomes primary engineering thrust for Wave 6+. NOT "blockchain becomes body" (would invalidate v2 § 公理 5).
handover/ai-direct/LATEST.md:29:### Wave 6 priorities re-ordered under ChainTape lens
handover/ai-direct/LATEST.md:59:**Updated**: 2026-04-28 — **Wave 6 #1 CO1.7 spec PASS/PASS gate cleared** (`a946820` v1.2). Three rounds of dual external audit converged: R1 CHALLENGE/CHALLENGE → R2 PASS/CHALLENGE → R3 PASS/PASS. Spec + skeleton + system_keypair extension all audit-cleared; CO1.7 implementation start now unblocked.
handover/ai-direct/LATEST.md:63:**Next-session entry**: 🚀 **CO1.7 implementation** (now unblocked per `handover/audits/CO1_7_DUAL_AUDIT_VERDICT_R3_2026-04-28.md` PASS/PASS). Per spec § 13: 3 downstream atoms estimated 5-9 days total for Wave 6 #1 closure:
handover/ai-direct/LATEST.md:72:## 🎯 2026-04-29 Session-2 CLOSURE — CO1.13 atom bundle COMPLETE ✅
handover/ai-direct/LATEST.md:74:**Status**: CO1.13.1 + CO1.13.2 + CO1.13.3 all shipped + drift review = NO MATERIAL DRIFT. Wave 6 #2 PRE-CO1.8 alignment factory now LIVE.
handover/ai-direct/LATEST.md:75:**HEAD commit**: `1a5849f` (CO1.13 phase drift review + --half factory upgrade).
handover/ai-direct/LATEST.md:78:### 🚀 Next-session entry point
handover/ai-direct/LATEST.md:82:1. **CO1.8 spec round-1 audit launch** — spec drafted at `6cc5cc9`; launchers exist at `handover/audits/run_{codex,gemini}_co1_8_round1_audit.sh|py`; not yet run. CO1.13 factory is now LIVE so audits will benefit from R-022 + § F.2 auto-refresh + § J orphan registry + the `--half` Phase C regression check.
handover/ai-direct/LATEST.md:83:2. **CO1.13-extra** (legacy backlink closure; ~10-15 hr; ~250 missing backlinks) — MUST schedule before Phase D per spec § 0.5 Gemini r1 Q7. With R-022 LIVE, every NEW pub symbol since `e9c6a2b` is enforced; legacy gap is the remaining substantive debt.
handover/ai-direct/LATEST.md:85:### Three commits this CO1.13 closure arc
handover/ai-direct/LATEST.md:89:| 1 | `9be22b4` | CO1.13.1 — TRACE_MATRIX_v3 doc completion (§ E.2/E.3 measured stats; § F.2 manual snapshot 135 backlinks; § J Orphan Extensions schema; cross-ref reconciliation). +283 / -14 doc delta. Trust Root rehash for TRACE_MATRIX_v3. |
handover/ai-direct/LATEST.md:90:| 2 | `e9c6a2b` | CO1.13.2 + CO1.13.3 — R-022 hook (rules YAML + custom_commit_hook check_trace_matrix.py 421 LoC + tracked pre-commit shim + install_hooks.sh + .github/workflows/co1_13_r022_ci.yml + 5-line engine.py patch + 9 shell integration tests + Rust orchestrator) + auto-refreshing § F.2 reverse-map (update_trace_matrix_reverse_map.py 134 LoC; shares parser with R-022 check). +1011 / -31. Trust Root rehash for engine.py + TRACE_MATRIX_v3. |
handover/ai-direct/LATEST.md:91:| 3 | `1a5849f` | CO1.13 phase drift review (`handover/architect-insights/CO1_13_PHASE_DRIFT_REVIEW_2026-04-29.md` 215 LoC) + `--half` factory upgrade to `run_c2_phase_c_ablation.sh` (3 problems × 5 modes × 1 seed × MAX_TX=20; lives between cheap `--smoke` and full Phase C batch). Trust Root rehash for runner script. |
handover/ai-direct/LATEST.md:93:### CO1.13 final spec compliance (vs v1.1.1 § 0.3)
handover/ai-direct/LATEST.md:97:| CO1.13.1 | ~200 | +283 / -14 | ACCEPTABLE (table content + § J schema; quality spending) |
handover/ai-direct/LATEST.md:98:| CO1.13.2 | ~335 | ~676 (script 421 + yaml 20 + shim 13 + installer 31 + ci 24 + 5-line engine.py + tests 297) | ACCEPTABLE (test-isolation hardening forced by real pollution incident) |
handover/ai-direct/LATEST.md:99:| CO1.13.3 | ~100 | 134 | ACCEPTABLE (--check / --dry-run modes added) |
handover/ai-direct/LATEST.md:106:3. **CO1.13.3 idempotency** — `python3 scripts/update_trace_matrix_reverse_map.py --check` exits 0 immediately after first run.
handover/ai-direct/LATEST.md:107:4. **Phase C smoke 5/5 PASS in 95s** post-CO1.13 (consistent with 97s baseline at `8d88f2d`); soft_law H2 fake-accept signature preserved. Per user 2026-04-29 challenge: `--smoke` is pipeline-liveness only — for CO1.13 (0 lines of `src/` changed) it confirms only that Trust Root rehashes didn't break evaluator boot.
handover/ai-direct/LATEST.md:116:1. **CO1.8 spec round-1 audit launch** — drafted at `6cc5cc9`; ready under new factory regime
handover/ai-direct/LATEST.md:118:3. **CO1.13-extra** (legacy backlink closure; ~10-15 hr; ~250 backlinks; MUST before Phase D per Gemini r1 Q7)
handover/ai-direct/LATEST.md:119:4. **CO1.13-devtools-mathlib-mirror** (new follow-up sub-atom; this session): file-mirror endpoint on linux1 hosting Mathlib v4.24.0 `.lake/packages` tarball; omega-vm hydration script; Trust Root sha256 registration. Constitutionally clean (Lean stays local). Estimated ~1-2 day work; collapses future Mathlib re-fetch from 10-30 min to ~5 min internal-network rsync. Defer to between CO1.8 and CO1.9 atoms.
handover/ai-direct/LATEST.md:120:5. **CO1.13-devtools** (scaffold scripts + Trust Root rehash automation; per spec § 0.4) — non-spec; lands as separate commit
handover/ai-direct/LATEST.md:124:### New Constitutionally-clean Mathlib mirror architecture (CO1.13-devtools-mathlib-mirror; this session candidate spec)
handover/ai-direct/LATEST.md:137:### Cumulative project audit spend after CO1.13 closure
handover/ai-direct/LATEST.md:139:- This session's CO1.13 r1+r2 dual audits + cap-exception: ~$16-24 (per drift review § 7)
handover/ai-direct/LATEST.md:158:## 🌊 2026-04-29 Session-2 — CO1.7-extra Branch B closure + CO1.13 spec PASS-with-cap-exception (Elon-mode launch)
handover/ai-direct/LATEST.md:161:**Status**: spec phase **DONE** (CO1.7-extra ceremony closed + CO1.13 cleared for impl); implementation phase **READY TO START** in fresh session.
handover/ai-direct/LATEST.md:163:### 🚀 Next-session entry point
handover/ai-direct/LATEST.md:165:**Pick up at CO1.13 implementation phase per spec § 0.3 v1.1.1**. Three sub-atoms in dependency order:
handover/ai-direct/LATEST.md:167:1. **CO1.13.1** TRACE_MATRIX_v3 doc completion (~200 LoC docs delta; 0.5 day target)
handover/ai-direct/LATEST.md:171:2. **CO1.13.2** R-022 commit-time hook (~335 LoC; 1.5 day target)
handover/ai-direct/LATEST.md:178:3. **CO1.13.3** reverse-map § F populator (~100 LoC Python; 0.5 day target)
handover/ai-direct/LATEST.md:179:   - `scripts/update_trace_matrix_reverse_map.py` shares parser with CO1.13.2 (per Codex r1 § D "one parser shared")
handover/ai-direct/LATEST.md:183:**Authoritative spec**: `handover/specs/CO1_13_TRACE_MATRIX_IMPL_v1_2026-04-29.md` v1.1.1 (commit `813414c`). Read § 0.3 + § 1.2 + § 1.3 + § 2.1 + § 3 first; § 8 acknowledgements before coding.
handover/ai-direct/LATEST.md:197:| 1 | `6cc5cc9` | CO1.8 L5 Materializer v1 spec drafted (300 lines, 10/10 smoke). **AUDIT DEFERRED** in favor of CO1.13 per Elon-mode ROI analysis (factory amortization 20-50x over 150+ remaining atoms). |
handover/ai-direct/LATEST.md:198:| 2 | `8d88f2d` → `1423b90` → `813414c` | CO1.13 v1 → v1.1 (r1 9 patches) → v1.1.1 (r2 cap-exception 4 patches; Codex CHALLENGE-ESCALATE / Gemini PASS; conservative CHALLENGE-ESCALATE → cap-exception per Codex r2 § E recommendation). Spec at 420 lines; PASS-with-cap-exception. |
handover/ai-direct/LATEST.md:202:The user authorized "Elon-mode" framing for project management (factory > scope; cycle-time > round-count; constitution + whitepaper line-by-line preserved as scope, but PROCESS streamlined). Round-1 audit on CO1.13 v1 forced the policy to be CONCRETE rather than aspirational. v1.1.1 codified:
handover/ai-direct/LATEST.md:233:| CO1.13 spec-cycle wall-clock | ~2.5 hr (vs 14-day median pre-Elon-mode = ~134x compression on spec phase) | first real-test of Elon-mode "factory IS product" hypothesis; spec phase validated; impl phase pending |
handover/ai-direct/LATEST.md:234:| Backlink coverage baseline | 87/354 = 24.6% | 75% legacy gap quantified; CO1.13-extra (gap closure) MUST schedule before Phase D per Gemini r1 Q7 |
handover/ai-direct/LATEST.md:236:### Cumulative project audit spend after CO1.13 v1.1.1
handover/ai-direct/LATEST.md:240:- Per atom going forward (post-CO1.13 factory deployed): expected $5-10 (single round + targeted patches; CO1.13's R-022 + scaffold devtools amortize spec-cycle prep cost)
handover/ai-direct/LATEST.md:244:1. **CO1.13 implementation** (next-session entry; this is THE priority)
handover/ai-direct/LATEST.md:245:2. **CO1.8 spec round-1 audit** (deferred this session; spec drafted at `6cc5cc9` ready to launch; launchers exist at `handover/audits/run_{codex,gemini}_co1_8_round1_audit.sh|py` but were NOT run)
handover/ai-direct/LATEST.md:246:3. **CO1.13-extra** (legacy backlink closure; ~10-15 hr; ~250 missing backlinks; MUST schedule before Phase D per Gemini r1 Q7)
handover/ai-direct/LATEST.md:247:4. **CO1.13-devtools** (scaffold scripts + Trust Root rehash automation; non-spec follow-up; lands after CO1.13 PASS impl)
handover/ai-direct/LATEST.md:262:## 🌊 2026-04-29 Session-1 — Wave 6 #1 RECALIBRATION (CO1.7.5 split → CO1.7-extra; Branch A landed)
handover/ai-direct/LATEST.md:272:### Wave 6 #1 actual progress: ~30-40% (NOT 80%)
handover/ai-direct/LATEST.md:282:ChainTape vertical: L4 ~50-55% (storage + ABI + machinery + head_t close + Sequencer entry-point; transition bodies still pending). Estimate "Wave 6 #1 fully closed" = **after CO P2.x substrate ships** (multiple atoms; weeks-to-months out).
handover/ai-direct/LATEST.md:311:4. **Wave 6 #2 next-atom selection** — Wave 6 #1 (CO1.7 family) ceremony-closed; § 3.2 menu of unblocked atoms includes CO1.8 L5 materializer / CO1.9 L6 signal indices / CO1.10 signal dichotomy / CO1.11 safety vs creation / CO1.13 TRACE_MATRIX impl. Pending user direction on which Wave 6 #2 atom to spec next.
handover/ai-direct/LATEST.md:315:- **Q1 (sequencing)**: with Wave 6 #1 substrate now exposed as critical path, should the project reorder to ship CO P2.1/2.2/2.3/2.5/2.6/2.7/2.9 + CO1.11 before resuming CO1.7.5? Or continue Wave 6 #2/#3 affordances (CO1.8/CO1.9) in parallel?
handover/ai-direct/LATEST.md:320:## 🌊 2026-04-28 Session-2 Final — Wave 6 #1 IMPLEMENTATION PHASE COMPLETE ✅
handover/ai-direct/LATEST.md:327:**Wave 6 #1 (L4 Transition Ledger family) — 80% complete**:
handover/ai-direct/LATEST.md:347:2. **Then** Wave 6 #2/#3 unblocks (CO1.8 L5 materializer + CO1.9 L6 signal indices)
handover/ai-direct/LATEST.md:352:- **Q1 (architectural drift)**: TFR_MASTER_PLAN_2026-04-26 uses old paths (`src/tape/`, `src/wal.rs`, `src/ledger.rs`); actual work is under `src/bottom_white/ledger/` + `src/state/` per Anti-Oreo restoration. Worth a one-line "SUPERSEDED by Wave 6 framing" header, or leave as historical artifact?
handover/ai-direct/LATEST.md:384:| **Git substrate** | **65%** | gix→git2-rs pivot 完成 / CO1.3.1 spike 8/8 PASS / CO1.4 CAS 实现 (561 LoC + 16 tests) | runtime_repo 实例化 + evaluator 接线 = CO1.7+CO1.8 之后 |
handover/ai-direct/LATEST.md:422:**Deferred to Wave 6**:
handover/ai-direct/LATEST.md:423:- 🔄 **INV8 spec v2 revision** (NEW Wave 6 priority — close 4 VETO + 5 CHALLENGE; re-audit dual external; both PASS required for CO P2.4.0 spike clearance; CO P2.4.1+ atoms remain BLOCKED until then)
handover/ai-direct/LATEST.md:768:## Next-session boot sequence (CO P0 night-shift complete; CO P1 awaiting GO)

exec
/bin/bash -lc "rg -n \"derive_state_root|state_root_t|state_root\" src/state src -g '*.rs' | head -n 200" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
src/top_white/predicates/registry.rs:27:    /// Fail-closed: rejected work_tx does NOT advance state_root.
src/state/q_state.rs:52:    /// TRACE_MATRIX § 3 — pseudocode `NodeId::from_state_root(state_root)` constructor.
src/state/q_state.rs:54:    pub fn from_state_root(state_root: Hash) -> Self {
src/state/q_state.rs:56:        for byte in state_root.0.iter() {
src/state/q_state.rs:313:    pub state_root_t: Hash,
src/state/q_state.rs:330:    /// roots populated by `boot::verify_trust_root` and the `state_root_t` published
src/state/q_state.rs:354:        assert_eq!(g.state_root_t, Hash::ZERO);
src/state/q_state.rs:371:            "state_root_t",
src/state/q_state.rs:413:    fn node_id_from_state_root_is_deterministic() {
src/state/q_state.rs:415:        let n1 = NodeId::from_state_root(r);
src/state/q_state.rs:416:        let n2 = NodeId::from_state_root(r);
src/state/sequencer.rs:214:/// NOT mutate `q.head_t` or `q.state_root_t`; it returns the new `QState`
src/state/sequencer.rs:369:            parent_state_root: q_snapshot.state_root_t,
src/state/sequencer.rs:373:            resulting_state_root: q_next.state_root_t,
src/state/sequencer.rs:392:            parent_state_root: signing_payload.parent_state_root,
src/state/sequencer.rs:396:            resulting_state_root: signing_payload.resulting_state_root,
src/state/sequencer.rs:409:        // for writers that return None (InMemory path). state_root_t comes
src/state/sequencer.rs:499:            parent_state_root: Default::default(),
src/state/sequencer.rs:555:                parent_state_root: Default::default(),
src/state/sequencer.rs:563:                parent_state_root: Default::default(),
src/state/typed_tx.rs:226:    pub parent_state_root: Hash,                      //  3
src/state/typed_tx.rs:307:    pub parent_state_root: Hash,           //  6
src/state/typed_tx.rs:320:    pub parent_state_root: Hash,           //  3
src/state/typed_tx.rs:408:    pub parent_state_root: Hash,
src/state/typed_tx.rs:466:    pub parent_state_root: Hash,
src/state/typed_tx.rs:482:    pub parent_state_root: Hash,
src/state/typed_tx.rs:519:            parent_state_root: self.parent_state_root,
src/state/typed_tx.rs:565:            parent_state_root: self.parent_state_root,
src/state/typed_tx.rs:577:            parent_state_root: self.parent_state_root,
src/state/typed_tx.rs:719:    /// `parent_state_root` does not match `q.state_root_t` (any agent tx).
src/state/typed_tx.rs:793:            Self::StaleParent => write!(f, "stale parent_state_root"),
src/state/typed_tx.rs:943:            parent_state_root: h(0x42),
src/state/typed_tx.rs:1002:            parent_state_root: h(0x43),
src/state/typed_tx.rs:1013:            parent_state_root: h(0x44),
src/bottom_white/ledger/transition_ledger.rs:20://!   CO1.8 owns state_root mutation. Skeleton reflects boundary (no state_root mutation).
src/bottom_white/ledger/transition_ledger.rs:73:    pub parent_state_root: Hash,                 //  2
src/bottom_white/ledger/transition_ledger.rs:80:    /// Resulting state_root post-transition (NOT mutated by L4 — accepted as
src/bottom_white/ledger/transition_ledger.rs:82:    pub resulting_state_root: Hash,              //  6
src/bottom_white/ledger/transition_ledger.rs:110:    pub parent_state_root: Hash,
src/bottom_white/ledger/transition_ledger.rs:114:    pub resulting_state_root: Hash,
src/bottom_white/ledger/transition_ledger.rs:126:        h.update(self.parent_state_root.0);
src/bottom_white/ledger/transition_ledger.rs:130:        h.update(self.resulting_state_root.0);
src/bottom_white/ledger/transition_ledger.rs:152:            parent_state_root: self.parent_state_root,
src/bottom_white/ledger/transition_ledger.rs:156:            resulting_state_root: self.resulting_state_root,
src/bottom_white/ledger/transition_ledger.rs:279:/// - `ChainOnly`: skeleton-stage; chain integrity only (parent_state_root +
src/bottom_white/ledger/transition_ledger.rs:282:///   from CAS + re-runs pure transitions + asserts state_root match. THE
src/bottom_white/ledger/transition_ledger.rs:330:            Self::ParentStateMismatch { at } => write!(f, "parent_state_root mismatch at index {at}"),
src/bottom_white/ledger/transition_ledger.rs:335:            Self::StateRootMismatch { at } => write!(f, "resulting_state_root divergence at index {at}"),
src/bottom_white/ledger/transition_ledger.rs:375:/// 2. parent_state_root chain
src/bottom_white/ledger/transition_ledger.rs:382:/// 8. q_next.state_root_t matches entry.resulting_state_root
src/bottom_white/ledger/transition_ledger.rs:385:/// **v1.1 C-1 closure**: takes a full `genesis: &QState` (was `genesis_state_root`
src/bottom_white/ledger/transition_ledger.rs:393:/// independently are `#[test]`-runnable now; full state_root reconstruction
src/bottom_white/ledger/transition_ledger.rs:421:        if entry.parent_state_root != q.state_root_t {
src/bottom_white/ledger/transition_ledger.rs:473:        // Stage 8: state_root match.
src/bottom_white/ledger/transition_ledger.rs:474:        if q_next.state_root_t != entry.resulting_state_root {
src/bottom_white/ledger/transition_ledger.rs:496:/// 2. parent_state_root chain
src/bottom_white/ledger/transition_ledger.rs:502:/// - resulting_state_root (CO1.7.5+: requires dispatch_transition + CO1.4-extra CAS persistence)
src/bottom_white/ledger/transition_ledger.rs:504:/// Returns final (state_root, ledger_root) on success.
src/bottom_white/ledger/transition_ledger.rs:506:    genesis_state_root: Hash,
src/bottom_white/ledger/transition_ledger.rs:510:    let mut prev_state_root = genesis_state_root;
src/bottom_white/ledger/transition_ledger.rs:522:        if entry.parent_state_root != prev_state_root {
src/bottom_white/ledger/transition_ledger.rs:534:        prev_state_root = entry.resulting_state_root;
src/bottom_white/ledger/transition_ledger.rs:538:    Ok((prev_state_root, prev_ledger_root))
src/bottom_white/ledger/transition_ledger.rs:843:        parent_state_root: Hash,
src/bottom_white/ledger/transition_ledger.rs:845:        resulting_state_root: Hash,
src/bottom_white/ledger/transition_ledger.rs:849:            parent_state_root,
src/bottom_white/ledger/transition_ledger.rs:853:            resulting_state_root,
src/bottom_white/ledger/transition_ledger.rs:862:            parent_state_root: signing.parent_state_root,
src/bottom_white/ledger/transition_ledger.rs:866:            resulting_state_root: signing.resulting_state_root,
src/bottom_white/ledger/transition_ledger.rs:890:            parent_state_root: Hash::ZERO,
src/bottom_white/ledger/transition_ledger.rs:894:            resulting_state_root: h(0xaa),
src/bottom_white/ledger/transition_ledger.rs:911:        let e_skip = entry_at(3, e1.resulting_state_root, e1.resulting_ledger_root, h(2));
src/bottom_white/ledger/transition_ledger.rs:920:        let e2 = entry_at(2, e1.resulting_state_root, e1.resulting_ledger_root, h(2));
src/bottom_white/ledger/transition_ledger.rs:921:        let e3 = entry_at(3, e2.resulting_state_root, e2.resulting_ledger_root, h(3));
src/bottom_white/ledger/transition_ledger.rs:925:        assert_eq!(final_state, e3.resulting_state_root);
src/bottom_white/ledger/transition_ledger.rs:929:    // 5. ChainOnly replay rejects parent_state_root tamper
src/bottom_white/ledger/transition_ledger.rs:933:        let mut e2 = entry_at(2, e1.resulting_state_root, e1.resulting_ledger_root, h(2));
src/bottom_white/ledger/transition_ledger.rs:934:        e2.parent_state_root = h(0xff);
src/bottom_white/ledger/transition_ledger.rs:943:        let mut e2 = entry_at(2, e1.resulting_state_root, e1.resulting_ledger_root, h(2));
src/bottom_white/ledger/transition_ledger.rs:955:        let mut e2 = entry_at(2, e1.resulting_state_root, e1.resulting_ledger_root, h(2));
src/bottom_white/ledger/transition_ledger.rs:1009:            parent_state_root: Hash::ZERO,
src/bottom_white/ledger/transition_ledger.rs:1013:            resulting_state_root: h(1),
src/bottom_white/ledger/transition_ledger.rs:1083:        let e2 = entry_at(2, e1.resulting_state_root, e1.resulting_ledger_root, h(2));
src/bottom_white/ledger/transition_ledger.rs:1084:        let e3 = entry_at(3, e2.resulting_state_root, e2.resulting_ledger_root, h(3));
src/bottom_white/ledger/transition_ledger.rs:1115:        let e_skip = entry_at(3, e1.resulting_state_root, e1.resulting_ledger_root, h(2));
src/bottom_white/ledger/transition_ledger.rs:1129:        let e2 = entry_at(2, e1.resulting_state_root, e1.resulting_ledger_root, h(2));
src/bottom_white/ledger/transition_ledger.rs:1146:        let e3 = entry_at(3, e2.resulting_state_root, e2.resulting_ledger_root, h(3));
src/bottom_white/ledger/transition_ledger.rs:1177:            parent_state_root: Hash::ZERO,
src/bottom_white/ledger/transition_ledger.rs:1198:        parent_state_root: Hash,
src/bottom_white/ledger/transition_ledger.rs:1200:        resulting_state_root: Hash,
src/bottom_white/ledger/transition_ledger.rs:1212:            parent_state_root,
src/bottom_white/ledger/transition_ledger.rs:1216:            resulting_state_root,
src/bottom_white/ledger/transition_ledger.rs:1227:            parent_state_root,
src/bottom_white/ledger/transition_ledger.rs:1231:            resulting_state_root,
src/bottom_white/ledger/transition_ledger.rs:1271:            h(1), // resulting state_root (won't be reached due to dispatch stub)
src/bottom_white/ledger/transition_ledger.rs:1383:            parent_state_root: entry.parent_state_root,
src/bottom_white/ledger/transition_ledger.rs:1387:            resulting_state_root: entry.resulting_state_root,
src/bottom_white/ledger/transition_ledger.rs:1438:            parent_state_root: Hash::ZERO,
src/bottom_white/ledger/transition_ledger.rs:1442:            resulting_state_root: h(1),
src/bottom_white/ledger/transition_ledger.rs:1452:            parent_state_root: Hash::ZERO,
src/bottom_white/ledger/transition_ledger.rs:1456:            resulting_state_root: h(1),
src/bottom_white/ledger/transition_ledger.rs:1486:        // ledger_writer + replay_full_transition(...) → assert final state_root
src/bottom_white/ledger/transition_ledger.rs:1487:        // matches sequencer's q.state_root_t. Dispatch must produce real
src/state/q_state.rs:52:    /// TRACE_MATRIX § 3 — pseudocode `NodeId::from_state_root(state_root)` constructor.
src/state/q_state.rs:54:    pub fn from_state_root(state_root: Hash) -> Self {
src/state/q_state.rs:56:        for byte in state_root.0.iter() {
src/state/q_state.rs:313:    pub state_root_t: Hash,
src/state/q_state.rs:330:    /// roots populated by `boot::verify_trust_root` and the `state_root_t` published
src/state/q_state.rs:354:        assert_eq!(g.state_root_t, Hash::ZERO);
src/state/q_state.rs:371:            "state_root_t",
src/state/q_state.rs:413:    fn node_id_from_state_root_is_deterministic() {
src/state/q_state.rs:415:        let n1 = NodeId::from_state_root(r);
src/state/q_state.rs:416:        let n2 = NodeId::from_state_root(r);
src/state/sequencer.rs:214:/// NOT mutate `q.head_t` or `q.state_root_t`; it returns the new `QState`
src/state/sequencer.rs:369:            parent_state_root: q_snapshot.state_root_t,
src/state/sequencer.rs:373:            resulting_state_root: q_next.state_root_t,
src/state/sequencer.rs:392:            parent_state_root: signing_payload.parent_state_root,
src/state/sequencer.rs:396:            resulting_state_root: signing_payload.resulting_state_root,
src/state/sequencer.rs:409:        // for writers that return None (InMemory path). state_root_t comes
src/state/sequencer.rs:499:            parent_state_root: Default::default(),
src/state/sequencer.rs:555:                parent_state_root: Default::default(),
src/state/sequencer.rs:563:                parent_state_root: Default::default(),
src/state/typed_tx.rs:226:    pub parent_state_root: Hash,                      //  3
src/state/typed_tx.rs:307:    pub parent_state_root: Hash,           //  6
src/state/typed_tx.rs:320:    pub parent_state_root: Hash,           //  3
src/state/typed_tx.rs:408:    pub parent_state_root: Hash,
src/state/typed_tx.rs:466:    pub parent_state_root: Hash,
src/state/typed_tx.rs:482:    pub parent_state_root: Hash,
src/state/typed_tx.rs:519:            parent_state_root: self.parent_state_root,
src/state/typed_tx.rs:565:            parent_state_root: self.parent_state_root,
src/state/typed_tx.rs:577:            parent_state_root: self.parent_state_root,
src/state/typed_tx.rs:719:    /// `parent_state_root` does not match `q.state_root_t` (any agent tx).
src/state/typed_tx.rs:793:            Self::StaleParent => write!(f, "stale parent_state_root"),
src/state/typed_tx.rs:943:            parent_state_root: h(0x42),
src/state/typed_tx.rs:1002:            parent_state_root: h(0x43),
src/state/typed_tx.rs:1013:            parent_state_root: h(0x44),

exec
/bin/bash -lc "nl -ba src/state/typed_tx.rs | sed -n '240,290p;292,325p;326,360p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   240	pub struct VerifyTx {
   241	    pub tx_id: TxId,                       //  1
   242	    pub target_work_tx: TxId,              //  2
   243	    pub verifier_agent: AgentId,           //  3
   244	    pub bond: StakeMicroCoin,              //  4
   245	    pub verdict: VerifyVerdict,            //  5
   246	    pub signature: AgentSignature,         //  6
   247	    pub timestamp_logical: u64,            //  7
   248	}
   249	
   250	impl Default for VerifyVerdict {
   251	    fn default() -> Self {
   252	        Self::Confirm
   253	    }
   254	}
   255	
   256	/// TRACE_MATRIX § 1.3 — challenge transaction (counter-example posted with
   257	/// stake at risk).
   258	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   259	pub struct ChallengeTx {
   260	    pub tx_id: TxId,                       //  1
   261	    pub target_work_tx: TxId,              //  2
   262	    pub challenger_agent: AgentId,         //  3
   263	    pub stake: StakeMicroCoin,             //  4
   264	    pub counterexample_cid: Cid,           //  5
   265	    pub signature: AgentSignature,         //  6
   266	    pub timestamp_logical: u64,            //  7
   267	}
   268	
   269	/// TRACE_MATRIX § 1.3 — fact-tx recording reuse of a tool created by a prior
   270	/// agent (royalty graph edge). No submitting agent (per § 3.6.5 v1.3).
   271	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   272	pub struct ReuseTx {
   273	    pub tx_id: TxId,                       //  1
   274	    pub reusing_work_tx: TxId,             //  2
   275	    pub reused_tool_id: ToolId,            //  3
   276	    pub reused_tool_creator: AgentId,      //  4
   277	    pub timestamp_logical: u64,            //  5
   278	}
   279	
   280	/// TRACE_MATRIX CO1.1.4-pre1 spec § 4 — derived schema (STATE spec § 3.4
   281	/// uses opaque `FinalizeTx::from(claim_id, reward)` constructor without an
   282	/// explicit struct definition).
   283	///
   284	/// **v1.1 round-1 audit closures**:
   285	/// - **C-3 (Codex Q-B)**: `claim_id` is now a typed `ClaimId` newtype (was
   286	///   bare `TxId`) — STATE § 4 I-FINALIZE-BATCH-ORDER speaks in claim_id;
   287	///   reusing TxId leaked QState implementation into the wire format.
   288	/// - **C-3 (Codex Q-B)**: `task_id` / `solver` / `reward` are documented as
   289	///   **Q-DERIVED at replay** — replay (CO1.7-impl A4) re-fetches them from
   290	///   ClaimsIndex by `claim_id`, NOT trusted from wire. Wire fields are kept
   292	///   semantics) but the AUTHORITATIVE values come from Q_t.
   293	/// - **C-3 / GM-2 followup**: `system_signature` is RETAINED for v1.1 — it
   294	///   binds the system-emitted FinalizeRewardTx to a specific runtime keypair
   295	///   epoch (auditability + cross-cell trust). The CO1.7 `LedgerEntry`
   296	///   wraps this struct via CAS reference + signs the `LedgerEntrySigningPayload`
   297	///   digest; the two sigs are NOT redundant: this one binds the tx-payload
   298	///   bytes; the L4 envelope sig binds the (logical_t, parent_ledger_root, tx_payload_cid)
   299	///   sequencer-stamped envelope. v1.1 spec § 4 makes the dual-sign rationale explicit.
   300	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   301	pub struct FinalizeRewardTx {
   302	    pub tx_id: TxId,                       //  1
   303	    pub claim_id: ClaimId,                 //  2 — typed (was TxId in v1)
   304	    pub task_id: TaskId,                   //  3 — Q-derived authoritative; wire = ledger summary
   305	    pub solver: AgentId,                   //  4 — Q-derived authoritative; wire = ledger summary
   306	    pub reward: MicroCoin,                 //  5 — Q-derived authoritative (SettlementEngine output); wire = ledger summary
   307	    pub parent_state_root: Hash,           //  6
   308	    pub epoch: SystemEpoch,                //  7
   309	    pub timestamp_logical: u64,            //  8
   310	    pub system_signature: SystemSignature, //  9 — see doc-comment on dual-sign rationale
   311	}
   312	
   313	/// TRACE_MATRIX STATE spec § 3.6 v1.3 — system-emitted task-expiry tx
   314	/// (refunds bounty + locked stakes when no claim finalized by deadline).
   315	/// Verbatim transcription.
   316	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   317	pub struct TaskExpireTx {
   318	    pub tx_id: TxId,                       //  1
   319	    pub task_id: TaskId,                   //  2
   320	    pub parent_state_root: Hash,           //  3
   321	    pub bounty_refunded: MicroCoin,        //  4 (computed by runtime; included for ledger summary)
   322	    pub epoch: SystemEpoch,                //  5
   323	    pub timestamp_logical: u64,            //  6
   324	    pub system_signature: SystemSignature, //  7
   325	}
   326	
   327	/// TRACE_MATRIX STATE spec § 1.5 — system-emitted no-accept-run handler.
   328	/// Emitted exactly once if a run terminates without any accepted work_tx, so
   329	/// L6 reconstructibility (failure-class signal) is preserved on the tape
   330	/// even when no work_tx ever passed.
   331	///
   332	/// **v1.1 round-1 audit closure (C-3 Codex Q-C must-fix-now)**: replaces the
   333	/// 3-field placeholder previously living in `system_keypair.rs`. Full
   334	/// 8-field schema per STATE § 1.5. The signer (`system_keypair`) now signs
   335	/// an opaque `TerminalSummarySigning([u8; 32])` digest — same opaque-digest
   336	/// pattern as `LedgerEntrySigning` — so the digest is computed here via
   337	/// `TerminalSummaryTx::to_signing_payload().canonical_digest()` (with the
   338	/// `b"turingosv4.system_sig.terminal_summary.v1"` domain prefix per v1.1 P1)
   339	/// and `system_keypair` stays oblivious to the typed-tx schema (no circular
   340	/// `bottom_white ↔ state` dependency).
   341	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   342	pub struct TerminalSummaryTx {
   343	    pub tx_id: TxId,                                          //  1
   344	    pub task_id: TaskId,                                      //  2
   345	    pub run_id: RunId,                                        //  3
   346	    pub run_outcome: RunOutcome,                              //  4
   347	    pub total_attempts: u32,                                  //  5
   348	    pub failure_class_histogram: BTreeMap<RejectionClass, u32>,// 6
   349	    pub last_logical_t: u64,                                  //  7
   350	    pub system_signature: SystemSignature,                    //  8
   351	}
   352	
   353	impl Default for RunOutcome {
   354	    fn default() -> Self {
   355	        Self::OmegaAccepted
   356	    }
   357	}
   358	
   359	// ────────────────────────────────────────────────────────────────────────────
   360	// § 7 Signing payloads (CO1.1.4-pre1 v1.1 round-1 closure C-1)

exec
/bin/bash -lc "nl -ba src/state/sequencer.rs | sed -n '200,220p;365,415p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   200	impl std::error::Error for SequencerError {}
   201	
   202	// ────────────────────────────────────────────────────────────────────────────
   203	// Sequencer — single-writer per (runtime_repo, run_id)
   204	// ────────────────────────────────────────────────────────────────────────────
   205	
   206	/// TRACE_MATRIX § 5.2.1 — L4 sequencer; single-writer per (runtime_repo, run_id).
   207	///
   208	/// **K1 dual counter**: `next_submit_id` advances at every `submit()` (used to
   209	/// derive `SubmissionReceipt.submit_id`); `next_logical_t` advances ONLY at
   210	/// commit time (rejected submissions never get a logical_t — preserves
   211	/// `LedgerWriter`'s strict logical_t monotonicity invariant).
   212	///
   213	/// **K3 v1.2 + CO1.7-extra D2 (revised)**: the pure transition function does
   214	/// NOT mutate `q.head_t` or `q.state_root_t`; it returns the new `QState`
   215	/// and the sequencer accepts it as-is. `head_t` mutation now happens
   216	/// post-commit via `advance_head_t()` (CO1.7-extra D2): when
   217	/// `LedgerWriter::head_commit_oid_hex()` returns Some (Git2LedgerWriter),
   218	/// the sequencer writes `q.head_t = NodeId(commit_oid_hex)`; when None
   219	/// (InMemoryLedgerWriter), `head_t` is left unchanged (no-op preservation).
   220	///
   365	        // Stage 5: build LedgerEntrySigningPayload (v1.1 — stage 4 fetch_add
   366	        // moved to AFTER stage 9 commit success).
   367	        let signing_payload = LedgerEntrySigningPayload {
   368	            logical_t,
   369	            parent_state_root: q_snapshot.state_root_t,
   370	            parent_ledger_root: q_snapshot.ledger_root_t,
   371	            tx_kind: tx.tx_kind(),
   372	            tx_payload_cid: payload_cid,
   373	            resulting_state_root: q_next.state_root_t,
   374	            timestamp_logical: logical_t,
   375	            epoch: self.epoch,
   376	            extensions: std::collections::BTreeMap::new(),
   377	        };
   378	
   379	        // Stage 6: C3 — sign via typed CanonicalMessage::LedgerEntrySigning.
   380	        let signing_digest = signing_payload.canonical_digest();
   381	        let system_signature = transition_ledger_emitter::sign_ledger_entry(
   382	            &self.keypair,
   383	            signing_digest.0,
   384	        )?;
   385	
   386	        // Stage 7: pure ledger-root fold (deterministic).
   387	        let resulting_ledger_root = append(&q_snapshot.ledger_root_t, &signing_digest);
   388	
   389	        // Stage 8: build LedgerEntry (the stored record).
   390	        let entry = LedgerEntry {
   391	            logical_t: signing_payload.logical_t,
   392	            parent_state_root: signing_payload.parent_state_root,
   393	            parent_ledger_root: signing_payload.parent_ledger_root,
   394	            tx_kind: signing_payload.tx_kind,
   395	            tx_payload_cid: signing_payload.tx_payload_cid,
   396	            resulting_state_root: signing_payload.resulting_state_root,
   397	            resulting_ledger_root,
   398	            timestamp_logical: signing_payload.timestamp_logical,
   399	            epoch: signing_payload.epoch,
   400	            extensions: signing_payload.extensions,
   401	            system_signature,
   402	        };
   403	
   404	        // Stage 9: commit + mutate Q_t under write lock.
   405	        // v1.1 C-2: next_logical_t.store(logical_t) HAPPENS ONLY AFTER
   406	        // writer.commit succeeds — preserves K1 under infra failure.
   407	        // CO1.7-extra D2: q.head_t = NodeId(commit_oid_hex) via advance_head_t
   408	        // when writer surfaces a commit OID (Git2 path); no-op preservation
   409	        // for writers that return None (InMemory path). state_root_t comes
   410	        // from q_next as-is per K3 v1.2.
   411	        {
   412	            let mut q_w = self.q.write().map_err(|_| ApplyError::QStateLockPoisoned)?;
   413	            let mut writer_w = self
   414	                .ledger_writer
   415	                .write()

exec
/bin/bash -lc "nl -ba handover/audits/CO1_7_5_DUAL_AUDIT_VERDICT_R1_2026-04-29.md | sed -n '140,175p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   140	| **M8d** P3 stale wording | Codex Q-G | "§ 6 ack #8" → "§ 6 ack" (no #8 after self-audit drop). |
   141	
   142	## Not addressed in v1.1 (out of scope for CO1.7-extra atom)
   143	
   144	- M2 (purity boundary) — gated to future CO1.7.5 spec
   145	- M6 (mapping table completeness) — gated to future CO1.7.5 spec
   146	- M7 (RejectedAttemptSummary substantiation) — gated to future CO1.7.5 spec
   147	
   148	These migrate with D1 to the future CO1.7.5 atom, where they belong.
   149	
   150	## Audit cost summary
   151	
   152	- Codex r1: 254,013 tokens used (size matches deep source-level review)
   153	- Gemini r1: prompt=144,441 / candidates=3,107 / total=150,915 tokens
   154	- Estimated round cost: ~$8-15 (single round both audits)
   155	- Cumulative project audit spend: ~$183-288 / $890 mid-budget (~21-32%)
   156	
   157	## Status going forward
   158	
   159	1. **CO1.7-extra v1.1**: spec rewritten in place this session; awaiting round-2 dual audit
   160	2. **CO1.7.5 (transition bodies)**: future atom; spec to be drafted **after** CO P2.x substrate atoms reach individual PASS/PASS
   161	3. **LATEST.md**: should be patched to reflect audit-derived true state of Wave 6 #1 (~30-40%, not 80%)
   162	4. **PROJECT_DECISION_MAP**: should track CO1.7-extra as new bridge atom; CO1.7.5 dependency declared

exec
/bin/bash -lc 'rg -n "CO1_8|CO1.8|materializer" handover/specs src tests | head -n 80' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
tests/conformance_stubs.rs:72:#[test] #[ignore] fn chain_tape_l5_materialized_state() { unimplemented!("CO1.8 — L5 materialized state DB + indices") }
tests/conformance_stubs.rs:115:#[test] #[ignore] fn agent_view_filters_internals() { unimplemented!("CO1.8.6 visibility filter") }
tests/conformance_stubs.rs:116:#[test] #[ignore] fn agent_view_minimal_context() { unimplemented!("CO1.8.7 prompt_builder reads only agent_view") }
handover/specs/CO1_13_TRACE_MATRIX_IMPL_v1_2026-04-29.md:3:**Status**: v1.1.1 (2026-04-29; round-1 = CHALLENGE/CHALLENGE → 9 patches → v1.1; round-2 = **SPLIT** [Codex CHALLENGE-ESCALATE/HIGH per `CODEX_CO1_13_ROUND2_AUDIT_2026-04-29.md`; Gemini PASS/HIGH per `GEMINI_CO1_13_ROUND2_AUDIT_2026-04-29.md`]; conservative merge per `feedback_dual_audit_conflict` = CHALLENGE-ESCALATE; **CAP EXCEPTION authorized via auto-execute mode** per Codex r2 § E own recommendation "approve one surgical final patch despite the 2-round cap". v1.1.1 applies 4 surgical fixes (3 mechanical + 1 substantive CI gate) closing all Codex r2 New-P0s. Per Elon-mode policy refinement: this v1.1.1 is the GENUINE FINAL spec; if any subsequent issue surfaces during impl, ship-with-OBS allowed only for bounded-edge-cases — NOT for R-022 enforcement). Wave 6 #2 PRE-CO1.8.
handover/specs/CO1_13_TRACE_MATRIX_IMPL_v1_2026-04-29.md:27:CO1.13 lands these closures. Each subsequent atom (CO1.8 / CO1.9 / ... / CO1.14 / CO P2.0 / ... / CO P2.12 = ~150 atoms) saves 30-60 min/atom on alignment hygiene under R-022 enforcement = **~75-150 hr amortization** over remaining sprint.
handover/specs/CO1_13_TRACE_MATRIX_IMPL_v1_2026-04-29.md:153:`scripts/update_trace_matrix_reverse_map.sh` walks `src/*.rs`, extracts every `/// TRACE_MATRIX <id>: <role>` doc-comment + the immediately-following pub line, formats as `| <pub_symbol> | <id> | <role> |`, writes to TRACE_MATRIX_v3.md § F (idempotent — replaces section content). First run populates from current HEAD; subsequent runs (e.g., post-CO1.8 land) refresh.
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:42:| K3 | L4/L5 head_t ownership inconsistent (spec line 194 vs 276 disagreed) | CO1.7 owns `ledger_root_t` + commit-chain `head_t = NodeId(commit_sha)` only; L5 (CO1.8) owns `state_root_t` mutation; sequencer drops `head_t = NodeId::from_state_root(...)` line | Codex Q-E |
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:70:- L5 materializer (state_root computation) — deferred to **CO1.8**. **K3 boundary (revised v1.2)**: CO1.7 owns `ledger_root_t` only; CO1.8 owns `state_root_t`; **head_t mutation is deferred to CO1.7.5+ wiring** (when `Git2LedgerWriter` exists). Sequencer does NOT mutate `state_root_t` or `head_t` directly; it accepts `q_next.state_root_t` as returned by the transition function and persists `ledger_root_t`.
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:390:**Why no head_t mutation in apply_one (K3, revised v1.2)**: v1.x CO1.7 owns `ledger_root_t` only. CO1.8 owns `state_root_t`. **head_t mutation deferred to CO1.7.5+** when `Git2LedgerWriter` provides a commit_sha return alongside Hash; the InMemoryLedgerWriter used by the v1 skeleton has no commit_sha to expose, so the trait keeps a single `Hash` return and head_t wiring is a separate downstream concern. Sequencer never calls `NodeId::from_state_root(...)`.
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md:662:| K3 L4/L5 head_t ownership | Boundary clarified: CO1.7 owns ledger_root + commit-chain head_t (NodeId(commit_sha)); CO1.8 owns state_root | § 0, § 3, § 5 |
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:399:    let new_state_root = materializer::apply(&q.state_root_t, tx);
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:466:    q_next.state_root_t  = materializer::apply(&q.state_root_t, tx);
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:560:    q_next.state_root_t  = materializer::apply(&q.state_root_t, tx);
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:624:    q_next.state_root_t  = materializer::apply(&q.state_root_t, tx);
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:700:    q_next.state_root_t  = materializer::apply(&q.state_root_t, &FinalizeTx::from(claim_id, reward));
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:758:    q_next.state_root_t  = materializer::apply(&q.state_root_t, tx);
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md:852:    q_next.state_root_t  = materializer::apply(&q.state_root_t, &summary);
src/bottom_white/mod.rs:3://! Deterministic, append-only substrate. tape, CAS, ledger, sandbox, materializer, tools.
src/bottom_white/ledger/transition_ledger.rs:20://!   CO1.8 owns state_root mutation. Skeleton reflects boundary (no state_root mutation).
handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md:1:# CO1.8: L5 Materialized State + Agent Read View v1 ⏳ pre-audit (round-1 pending)
handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md:3:**Status**: v1 (2026-04-29; **PENDING round-1 dual external audit** per CLAUDE.md "Audit Standard"). Greenfield atom — `src/bottom_white/materializer/` does not yet exist (verified). Wave 6 #2 per LATEST.md. Determinate next atom post-CO1.7-extra closure (4a978f0).
handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md:10:- `CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md` — frozen 7-variant TypedTx ABI; CO1.8 consumes `TypedTx` exclusively (no transition-body internals).
handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md:11:- `STATE_TRANSITION_SPEC_v1_2026-04-27.md` v1.4 — references `materializer::apply(&q.state_root_t, tx)` at 7 invocation sites (lines 399, 466, 560, 624, 700, 758, 852); CO1.8 ships the function those sites call.
handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md:14:**Single sentence**: ship the L5 `bottom_white::materializer` module as a substrate-independent SHA-256 state-projection layer — deterministic `apply(state_root, tx) → new_state_root` + 6 sub-indices (per WP § 5.L5) + visibility-filtered `agent_view::project_for_agent` — leaving sub-index *content* schema to per-sub-atom audit and leaving runtime integration with Sequencer stage 4-7 to a later wiring atom that depends on CO1.7.5 transition bodies.
handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md:22:Per `SPRINT_DEPENDENCY_GRAPH_v1_2026-04-27.md` line 109-111: `[CO1.8] Materialized State (8 atoms) | blockedBy: CO1.7 | blocks: CO1.9`. CO1.7 family (transition_ledger spec + impl + extra) is ✅ closed (Branch A `5ce01b1` + Branch B closure 4a978f0). CO1.8 is therefore the unique unblocked next L-layer atom in the ChainTape vertical. Per LATEST.md line 156: "**Then** Wave 6 #2/#3 unblocks (CO1.8 L5 materializer + CO1.9 L6 signal indices)".
handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md:28:| Frozen by | Surface CO1.8 consumes |
handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md:30:| CO1.1.4-pre1 | 7-variant `TypedTx` ABI; CO1.8 dispatches on `TxKind` discriminant only — no per-variant body interpretation |
handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md:31:| CO1.7-impl | `LedgerEntry` 9-field signing surface; CO1.8 reads `tx_payload_cid` to fetch tx bodies via L3 CAS |
handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md:32:| CO1.7-extra | `q.head_t` post-commit binding via `advance_head_t` — CO1.8 reads `head_t` for replay-from-genesis tests |
handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md:34:| L3 CAS (CO1.4 + CO1.4-extra) | `CasStore::put`/`get` round-trip + sidecar JSONL persistence — CO1.8 stores serialized state snapshots in CAS, indexed by state_root |
handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md:42:| CO1.8.1 | `bottom_white::materializer::mod.rs` skeleton + `pub use` re-exports + `State` / `StateDelta` types + `MaterializerError` enum | ~80 | ✅ |
handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md:43:| CO1.8.2 | `materializer::apply(prior_root: &Hash, tx: &TypedTx) -> Result<Hash, MaterializerError>` — pure, deterministic, dispatches on `TxKind` | ~120 | ✅ |
handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md:44:| CO1.8.3 | `materializer::state_db::CurrentStateDb` — per-key state cells (BTreeMap-backed in v1; Path B git-tree migration deferred to CO1.8-extra) | ~100 | ✅ |
handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md:45:| CO1.8.4 | `materializer::indices::task_index` — TaskMarket task cells (CO P2.1 surface placeholder; v1 ships read API only) | ~80 | ✅ |
handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md:46:| CO1.8.5 | `materializer::indices::agent_reputation_index` — per-agent reputation counter (`AgentId → u64`); incremented by Verify/Challenge txs (per WP § 5.L5 line 399) | ~70 | ✅ |
handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md:47:| CO1.8.6 | `materializer::indices::{error_taxonomy_index, price_signal_index}` — rejection-class histogram (`ErrorClass → u64`) + market-price-signal cache (`MarketId → PriceSignal`) | ~90 | ✅ |
handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md:48:| CO1.8.7 | `materializer::agent_view::project_for_agent(agent_id: AgentId, q: &QState) -> AgentReadView` — visibility-filtered minimal-context; consumes L1 PredicateRegistry visibility tags (CO1.5 surface) | ~120 | ⚠️ depends on CO1.5 PredicateRegistry visibility tags being shipped (currently § 3.2 unblocked but not started; v1 ships interface + stub returning full view if PredicateRegistry not present) |
handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md:49:| CO1.8.8 | substrate-independent test plan — 5 tests: `apply_determinism` + `apply_genesis_to_first_state` + `agent_reputation_increments_on_verify_tx` + `agent_view_filters_internals` + `state_root_reproducibility_across_restart` | ~180 | ✅ |
handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md:55:1. **Per-variant transition body integration** — Sequencer stage 4-7 currently call `materializer::apply` against `Err(NotYetImplemented)` transition stubs. Wiring CO1.8 into the live transition path is gated on **future CO1.7.5** (per CO1.7-extra spec § 0.1; gated on CO P2.x substrate). v1 ships the materializer as standalone library; integration is a separate atom.
handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md:56:2. **Path B git-tree state_db backend** — v1 uses BTreeMap-backed in-memory state_db. Migration to git-tree-as-storage substrate is a follow-up CO1.8-extra (likely after TFR S2-S3 git substrate work). This preserves the "Hash = sha256 of serialized state" semantics; only the storage layer changes.
handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md:57:3. **L6 derivable indices** — `reputation_counters` (windowed delta), `price_signals` (market microstructure compression), `failure_histogram` belong to CO1.9 L6 per WP § 5.L5 line 427. CO1.8 ships only L5 absolute-state indices; L6 derived statistics are a separate atom.
handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md:58:4. **CO1.5 PredicateRegistry visibility tags** — CO1.8.7 `project_for_agent` interface lands; tag-driven filtering is no-op (returns full view) until CO1.5 ships visibility tags. Documented as known gap; not blocking.
handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md:66:src/bottom_white/materializer/
handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md:67:├── mod.rs                    # CO1.8.1 — pub use re-exports + State, StateDelta, MaterializerError
handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md:68:├── apply.rs                  # CO1.8.2 — apply(prior_root, tx) -> new_root
handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md:69:├── state_db.rs               # CO1.8.3 — CurrentStateDb (BTreeMap-backed v1)
handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md:72:│   ├── task_index.rs         # CO1.8.4 — TaskMarket task cells
handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md:73:│   ├── agent_reputation.rs   # CO1.8.5 — per-agent reputation counter
handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md:74:│   ├── error_taxonomy.rs     # CO1.8.6a — ErrorClass histogram
handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md:75:│   └── price_signal.rs       # CO1.8.6b — MarketId → PriceSignal cache
handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md:76:└── agent_view.rs             # CO1.8.7 — project_for_agent(agent_id, q_t) -> AgentReadView
handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md:79:`pub use` chain at `mod.rs` flattens to `crate::bottom_white::materializer::{apply, State, StateDelta, MaterializerError, project_for_agent}` — agent_view is the only sub-module needing a top-level re-export per WP § 5.L5 line 408 read_tool signature.
handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md:98:/// not in the state_db cache (v1 pre-Path-B; CO1.8-extra git-tree migration
handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md:110:| **I-DET** | `apply(r, tx) == apply(r, tx)` for all r, tx | CO1.8.8 `apply_determinism` |
handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md:111:| **I-GEN** | `apply(Hash::ZERO, tx) == h` for some h ≠ ZERO when tx has non-empty effect | CO1.8.8 `apply_genesis_to_first_state` |
handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md:117:Materializer apply is invoked from Sequencer `apply_one` AFTER `writer_w.commit(&entry)?` returns Ok and BEFORE `*q_w = q_next` — i.e., between L4 ledger commit and Q_t state advance. v1 ships materializer as a pure function so no atomicity concerns at the materializer layer; the Sequencer's existing q_w lock guarantees serial invocation.
handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md:138:    let h1 = materializer::apply(&prior, &tx).expect("apply 1");
handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md:139:    let h2 = materializer::apply(&prior, &tx).expect("apply 2");
handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md:150:    let h = materializer::apply(&Hash::ZERO, &tx).expect("apply");
handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md:162:    let new_root = materializer::apply(&q.state_root_t, &verify_tx).expect("apply");
handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md:163:    let view = materializer::indices::agent_reputation::reputation_for(&new_root, "A0");
handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md:174:    let view = materializer::project_for_agent("A0", &q);
handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md:188:        materializer::apply(&r, tx).expect("apply"));
handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md:191:        materializer::apply(&r, tx).expect("apply (cold)"));
handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md:203:| Q2 | Is materializer::apply allowed to fail on `PriorRootNotFound`, or must v1 always succeed (e.g., by lazy snapshot reconstruction from L4 replay)? | v1 ships PriorRootNotFound failure mode (BTreeMap cache lookup). Lazy reconstruction is a CO1.8-extra concern (would require Sequencer reference for L4 replay; out of scope for pure-function v1). |
handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md:204:| Q3 | Should sub-indices share a single state_db backing (one BTreeMap<Key, Value> with namespaced keys) or live in separate BTreeMaps with cross-references? | Single backing with namespaced keys: `("task", task_id)` / `("rep", agent_id)` / etc. Simpler audit; trivial migration to git-tree path-keying in CO1.8-extra. |
handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md:214:| 1 (this v1) | ⏳ pending | ⏳ pending | TBD | round-1 dual external audit on CO1.8 v1 |
handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md:217:**Pre-implementation gate**: spec must reach PASS/PASS before any code in `src/bottom_white/materializer/**` is written. Per CLAUDE.md "Audit Standard". No STEP_B-restricted files touched (kernel.rs / bus.rs / wallet.rs UNTOUCHED by v1; only new files under `src/bottom_white/materializer/`).
handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md:227:- **Total atom budget**: ~1020 LoC; **estimated calendar time**: 2-4 days impl + 1-2 days audit cycles. Cumulative project audit spend after CO1.8 PASS/PASS: ~$210-330 / $890 mid-budget.
handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md:233:1. **Greenfield atom**: `src/bottom_white/materializer/` does not exist. Verified via `ls src/bottom_white/` showing only `cas`, `ledger`, `mod.rs`, `tools`. v1 creates the module from primary sources (whitepaper § 5.L5 + STATE spec invocation surface + sprint graph dependency note).
handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md:235:3. **CO1.5 PredicateRegistry visibility-tag dependency is acknowledged but stubbed** (§ 0.4 #4). Without CO1.5 visibility tags, `project_for_agent` is a no-op filter. This means CO1.8.7 ships an interface, not a fully-functional Inv-10 Goodhart shield. The gap is documented; CO1.5 + a follow-up CO1.8-extra closes the loop.
handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md:237:5. **Path B git-tree backend deferred** to CO1.8-extra (§ 0.4 #2). v1 ships in-memory BTreeMap to keep the substrate-independence invariant (no git2-rs touch in v1). This means v1 alone does NOT cold-restart from disk; cold-restart-via-L4-replay test (CO1.8.8) only verifies determinism, not durability. Durability lands with CO1.8-extra.
handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md:238:6. **No STEP_B-restricted file touches**. Kernel + bus + wallet untouched. v1 is pure-additive at `src/bottom_white/materializer/**`. No STEP_B parallel-branch ceremony required.
handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md:239:7. **FC-trace requirements**: every new pub symbol in CO1.8 implementation must carry `/// TRACE_MATRIX § 5.L5: <role>` doc-comment per CLAUDE.md "Alignment Standard". Set: `apply` + `State` + `StateDelta` + `MaterializerError` + `CurrentStateDb` + `task_index` accessors + `agent_reputation::reputation_for` + `error_taxonomy` + `price_signal` + `project_for_agent`.
handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md:252:| S4 | `materializer::apply` is invoked at 7 sites in STATE spec | `grep -c 'materializer::apply' handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md` | 7 |
handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md:253:| S5 | `bottom_white::materializer` does NOT exist (greenfield) | `ls src/bottom_white/materializer/ 2>&1` | "No such file or directory" |
handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md:273:| S4 | materializer::apply invocations in STATE spec | 7 hits (lines 399/466/560/624/700/758/852) | ✅ PASS |
handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md:274:| S5 | greenfield | `ls: cannot access 'src/bottom_white/materializer/': No such file or directory` | ✅ PASS |
handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md:287:- STATE_TRANSITION_SPEC v1.4 lines 399/466/560/624/700/758/852 (7 `materializer::apply` invocation sites)
handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md:289:- TRACE_MATRIX_v3 row § 5.L5 (module path `bottom_white::materializer::{state_db, indices, agent_view}`)

codex
**Section A: Verdict**

**VETO / HIGH.** The v1 spec has foundational contract contradictions, not just patchable wording issues. Main reason: `apply(&Hash, &TypedTx) -> Result<Hash, _>` is specified as pure/no-I/O while also depending on an implicit mutable state snapshot cache and CAS storage. That interface cannot reconstruct prior state, survive restart, or match the frozen STATE invocation surface as written.

**Section B: P0 Blockers**

1. **CO1.8 is not cleanly unblocked by CO1.7-extra alone.** Spec §0.1 line 22 cites SPRINT line 109, but the same sprint graph says `[CO1.7.5] ... blocks: CO1.8` at `SPRINT_DEPENDENCY_GRAPH`:106-108. The CO1.7.5 merged verdict also says transition bodies are future work gated on CO P2.x substrate, lines 101-104 and 157-160. So §0.1’s “unique unblocked next L-layer atom” overclaims.

2. **The materializer semantics contradict the “no transition-body internals” boundary.** Spec §0.2 line 30 says CO1.8 dispatches on `TxKind` only, with no per-variant body interpretation. But §0.3 line 46 and test §4.3 lines 159-164 require reputation changes from Verify/Challenge. `VerifyTx` only contains `target_work_tx` and `verifier_agent` (`typed_tx.rs`:240-247); incrementing the target solver/agent requires prior Work/Claim state, not just the discriminator.

3. **`apply()` is not a pure function as specified.** §2.1 lines 92-99 says no I/O and no side effects, but also says `PriorRootNotFound` comes from a `state_db` cache. §0.2 line 34 says snapshots are stored in CAS. §0.4 line 56 says v1 is in-memory BTreeMap. These cannot all be true. The real signature must expose state explicitly, e.g. `apply_to_state(&State, &TypedTx) -> Result<StateDelta/State, _>` plus a separate impure storage wrapper.

4. **STATE invocation surface mismatch is hard-blocking.** STATE assigns `materializer::apply(...)` directly into `Hash` fields at lines 399, 466, 560, 624, 700, 758, 852. Spec §2.1 lines 100-103 returns `Result<Hash, MaterializerError>`. Also STATE line 852 passes `&summary`, not `&TypedTx`; line 700 uses `FinalizeTx::from(...)`, while shipped ABI is `TypedTx::FinalizeReward(FinalizeRewardTx)` (`typed_tx.rs`:608-615). CO1.8 must explicitly supersede/patch these callsites or provide compatible wrappers.

5. **Agent view no-op full view is a constitutional leak.** §0.4 line 58 and §5 line 206 allow `project_for_agent` to return full view until CO1.5. But predicate visibility already exists in code (`registry.rs`:46-47, 138-146; `visibility.rs`:12-31). Shipping full-view fallback breaches the Goodhart shield. Minimum safe stub is deny-by-default/minimal empty view, not full view.

6. **Atomicity claim is wrong.** §2.3 line 117 says materializer runs after ledger commit and before Q advance. Current Sequencer computes `q_next` before commit and stores `resulting_state_root` into the ledger entry (`sequencer.rs`:339-374). If materialization can fail after commit, L4 can commit an entry Q never advances to. Materialization must happen before commit, or the spec must define an atomic commit+state-root persistence protocol.

**Section C: Open Architectural Questions**

- **State root semantics:** Author lean is mostly right: `Hash` is a 32-byte SHA-256 type (`q_state.rs`:27-33), while git backend OIDs are separate/informational in CAS (`cas/schema.rs`:8-12, 61-65). But §5 Q1 must define a dual-root model: `state_root_t = sha256(domain || version || canonical_snapshot_bytes)`, while git tree/blob OID is backend metadata.

- **Backing layout:** Single namespaced `BTreeMap` is acceptable for deterministic v1, but full-snapshot serialization creates write amplification. Namespaced keys map fine to git-tree paths later; separate BTreeMaps map better to Rust module ownership. The spec should choose one and define the serialized `StateSnapshot` shape.

- **Decomposition:** WP §5.L5 enumerates six modules including `permission_view` (`whitepaper`:397-402), then `read_tool` (`whitepaper`:408-409). Folding `permission_view` into `agent_view` is not clean. Better 8 atoms: `apply`, `current_state_db`, four indices, `permission_view`, `read_tool/project_for_agent`.

**Section D: Suggested Patches**

- §0.1 line 22: replace “unique unblocked next” with “valid spec-audit target; live integration remains gated by CO1.7.5/CO P2.x.”
- §0.2 line 30 / §0.3 lines 45-47: remove “TxKind only” or downgrade indices to schema/accessor-only until per-variant body semantics exist.
- §2.1 lines 92-103: split APIs:
  `apply_to_state(prior: &State, tx: &TypedTx) -> Result<State, _>` and
  `MaterializerStore::apply(prior_root, tx) -> Result<Hash, _>`.
- §2.3 line 117: move materialization before ledger commit, or define a two-phase commit invariant.
- §4.2 lines 146-152: assert a golden hex, not only `!= ZERO`.
- §4.4 lines 170-178: make the v1 test deny-by-default and compile against shipped `PredicateRegistry::agent_visible_view`, not fictional `oracle_seed`.
- §5 Q4 line 205: reuse `canonical_encode` config from `transition_ledger.rs`:545-557, add snapshot version/domain prefix and golden fixtures.

**Section E: Forward-Sustainability Notes**

CO1.8 is valuable infrastructure, but it only advances the PPUT/H-VPPUT loop once `agent_view/read_tool` gives minimal sufficient context. The conformance stubs already name this as a gate (`tests/conformance_stubs.rs`:115-116). Do not ship a public full-view stub and hope a later atom fixes it.

Also rerun smoke against current HEAD before round-2. The spec footer cites HEAD `4a978f0` (§9 lines 266-281), while the repo has moved. That is not the core veto, but stale smoke will confuse the next audit.
2026-04-29T11:47:58.312642Z ERROR codex_core::session: failed to record rollout items: thread 019dd90e-9890-7090-a27a-68172596f3b2 not found
tokens used
122,394
**Section A: Verdict**

**VETO / HIGH.** The v1 spec has foundational contract contradictions, not just patchable wording issues. Main reason: `apply(&Hash, &TypedTx) -> Result<Hash, _>` is specified as pure/no-I/O while also depending on an implicit mutable state snapshot cache and CAS storage. That interface cannot reconstruct prior state, survive restart, or match the frozen STATE invocation surface as written.

**Section B: P0 Blockers**

1. **CO1.8 is not cleanly unblocked by CO1.7-extra alone.** Spec §0.1 line 22 cites SPRINT line 109, but the same sprint graph says `[CO1.7.5] ... blocks: CO1.8` at `SPRINT_DEPENDENCY_GRAPH`:106-108. The CO1.7.5 merged verdict also says transition bodies are future work gated on CO P2.x substrate, lines 101-104 and 157-160. So §0.1’s “unique unblocked next L-layer atom” overclaims.

2. **The materializer semantics contradict the “no transition-body internals” boundary.** Spec §0.2 line 30 says CO1.8 dispatches on `TxKind` only, with no per-variant body interpretation. But §0.3 line 46 and test §4.3 lines 159-164 require reputation changes from Verify/Challenge. `VerifyTx` only contains `target_work_tx` and `verifier_agent` (`typed_tx.rs`:240-247); incrementing the target solver/agent requires prior Work/Claim state, not just the discriminator.

3. **`apply()` is not a pure function as specified.** §2.1 lines 92-99 says no I/O and no side effects, but also says `PriorRootNotFound` comes from a `state_db` cache. §0.2 line 34 says snapshots are stored in CAS. §0.4 line 56 says v1 is in-memory BTreeMap. These cannot all be true. The real signature must expose state explicitly, e.g. `apply_to_state(&State, &TypedTx) -> Result<StateDelta/State, _>` plus a separate impure storage wrapper.

4. **STATE invocation surface mismatch is hard-blocking.** STATE assigns `materializer::apply(...)` directly into `Hash` fields at lines 399, 466, 560, 624, 700, 758, 852. Spec §2.1 lines 100-103 returns `Result<Hash, MaterializerError>`. Also STATE line 852 passes `&summary`, not `&TypedTx`; line 700 uses `FinalizeTx::from(...)`, while shipped ABI is `TypedTx::FinalizeReward(FinalizeRewardTx)` (`typed_tx.rs`:608-615). CO1.8 must explicitly supersede/patch these callsites or provide compatible wrappers.

5. **Agent view no-op full view is a constitutional leak.** §0.4 line 58 and §5 line 206 allow `project_for_agent` to return full view until CO1.5. But predicate visibility already exists in code (`registry.rs`:46-47, 138-146; `visibility.rs`:12-31). Shipping full-view fallback breaches the Goodhart shield. Minimum safe stub is deny-by-default/minimal empty view, not full view.

6. **Atomicity claim is wrong.** §2.3 line 117 says materializer runs after ledger commit and before Q advance. Current Sequencer computes `q_next` before commit and stores `resulting_state_root` into the ledger entry (`sequencer.rs`:339-374). If materialization can fail after commit, L4 can commit an entry Q never advances to. Materialization must happen before commit, or the spec must define an atomic commit+state-root persistence protocol.

**Section C: Open Architectural Questions**

- **State root semantics:** Author lean is mostly right: `Hash` is a 32-byte SHA-256 type (`q_state.rs`:27-33), while git backend OIDs are separate/informational in CAS (`cas/schema.rs`:8-12, 61-65). But §5 Q1 must define a dual-root model: `state_root_t = sha256(domain || version || canonical_snapshot_bytes)`, while git tree/blob OID is backend metadata.

- **Backing layout:** Single namespaced `BTreeMap` is acceptable for deterministic v1, but full-snapshot serialization creates write amplification. Namespaced keys map fine to git-tree paths later; separate BTreeMaps map better to Rust module ownership. The spec should choose one and define the serialized `StateSnapshot` shape.

- **Decomposition:** WP §5.L5 enumerates six modules including `permission_view` (`whitepaper`:397-402), then `read_tool` (`whitepaper`:408-409). Folding `permission_view` into `agent_view` is not clean. Better 8 atoms: `apply`, `current_state_db`, four indices, `permission_view`, `read_tool/project_for_agent`.

**Section D: Suggested Patches**

- §0.1 line 22: replace “unique unblocked next” with “valid spec-audit target; live integration remains gated by CO1.7.5/CO P2.x.”
- §0.2 line 30 / §0.3 lines 45-47: remove “TxKind only” or downgrade indices to schema/accessor-only until per-variant body semantics exist.
- §2.1 lines 92-103: split APIs:
  `apply_to_state(prior: &State, tx: &TypedTx) -> Result<State, _>` and
  `MaterializerStore::apply(prior_root, tx) -> Result<Hash, _>`.
- §2.3 line 117: move materialization before ledger commit, or define a two-phase commit invariant.
- §4.2 lines 146-152: assert a golden hex, not only `!= ZERO`.
- §4.4 lines 170-178: make the v1 test deny-by-default and compile against shipped `PredicateRegistry::agent_visible_view`, not fictional `oracle_seed`.
- §5 Q4 line 205: reuse `canonical_encode` config from `transition_ledger.rs`:545-557, add snapshot version/domain prefix and golden fixtures.

**Section E: Forward-Sustainability Notes**

CO1.8 is valuable infrastructure, but it only advances the PPUT/H-VPPUT loop once `agent_view/read_tool` gives minimal sufficient context. The conformance stubs already name this as a gate (`tests/conformance_stubs.rs`:115-116). Do not ship a public full-view stub and hope a later atom fixes it.

Also rerun smoke against current HEAD before round-2. The spec footer cites HEAD `4a978f0` (§9 lines 266-281), while the repo has moved. That is not the core veto, but stale smoke will confuse the next audit.
