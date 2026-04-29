# CO1.8: L5 Materialized State + Agent Read View v1 🛑 DEFERRED (post-r1 audit; capability-first pivot)

**Status**: v1 **DEFERRED** 2026-04-29 (session-3). r1 dual external audit returned **Codex VETO/HIGH + Gemini CHALLENGE/HIGH** (conservative merge per `feedback_dual_audit_conflict` = VETO). Two real architectural P0s found (sprint-graph overclaim + apply() interface contradiction) + one Goodhart-shield P0. Per **2026-04-29 capability-first pivot** (LATEST.md session-3): **NO r2/r3 patch cycle**; spec deferred as-is, finding archived to `handover/alignment/OBS_CO1_8_V1_DEFERRED_2026-04-29.md`. Re-spec when CO1.7.5 transition bodies ship (precondition for valid apply() signature). Original v1 text preserved below as evidence of r1 finding.

**Author**: ArchitectAI (Claude); session 2026-04-29.

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
