# TB-18R R4 STEP_B Preflight — Chain-Derived Attempt-Count Invariant

**Atom**: TB-18R R4
**Class**: **4** (G1-ratified canonical contract on `chain_derived_run_facts`
ship-gate equation; implementation populates ratified spec **without
alteration**). Per `feedback_class4_cannot_hide_in_class3`: the equation is
the canonical-payload-adjacent ship-gate per Codex Q1 remediation. Per
charter §0.A row Q1 + Q4: G1 ratified the equation **shape**; R4 implements
it byte-faithfully. **No equation alteration**, **no ±N tolerance**, **no
substitution of approximate accounting**.
**STEP_B_PROTOCOL**: **applies** (charter §0.A row R4 + §1.1 `STEP_B_PROTOCOL flag`).
Parallel-branch worktree A/B; no direct edit on `main`. Worktree branch:
`tb-18r-r4-invariant` (off `main` at HEAD `63fde2b`).
**Date**: 2026-05-06
**Author**: Claude orchestrator (post-R3.fix SHIPPED, pre-user "go" on R4).
**Predecessor**: TB-18R R3.fix SHIPPED 2026-05-06 (commit `2ca1aed` via
merge `f2e73f6`; main HEAD `63fde2b`; workspace 1022/1/150 with the lone
fail = pre-existing arena flake unrelated to TB-18R surface).
**Charter**: `handover/tracer_bullets/TB-18R_charter_2026-05-06.md` §2 atom
table row R4 + §0.A Q1 + Q4 + §1.2 FR-18R.3 + FR-18R.4 + §1.4 SG-18R.3 +
SG-18R.4.

---

## §1 Atom scope (binding per charter §2 R4 row + §0.A Q1+Q4)

R4 is the **ship-gate equation** atom: extends `chain_derived_run_facts.rs`
with the FR-18R.3 / FR-18R.4 fields + invariant helper, **populating the
G1-ratified equation without alteration** per charter §0.A Q1 remediation.

> "G1 ratifies the equation as canonical contract; R4-impl follows
>  ratified spec." — charter §0.A Q1 v2.

This preflight specifies the IMPLEMENTATION shape that satisfies the
ratified equation. Equation **shape** is frozen by Codex Gate 1; R4 only
populates fields and computes deterministic accounting.

### §1.1 G1-ratified equation (verbatim from charter §1.2 FR-18R.3 v2)

```text
evaluator_reported_completed_llm_calls
  == l4_work_attempt_count + l4e_work_attempt_count

(after a mandatory sequencer drain barrier — every submitted typed-tx
 has reached a terminal state in chain or L4.E before the equation is
 evaluated)
```

Plus the auxiliary equation for aborted attempts:

```text
evaluator_observed_llm_call_starts
  == evaluator_reported_completed_llm_calls + attempt_aborted_count
```

R4-impl populates these two equations exactly. **No deviation, no
tolerance, no alternative formulation.** Mismatch is a ship blocker per
charter §1.4 SG-18R.3.

### §1.2 FR-18R.4 v2 six-field exact accounting (verbatim, charter §1.2)

Per evidence run, `chain_derived_run_facts` emits these exact fields:

```text
expected_completed_attempts: u64
l4_work_attempt_count: u64
l4e_work_attempt_count: u64
attempt_aborted_count: u64
delta: i64
  (= l4_work_attempt_count + l4e_work_attempt_count
     - expected_completed_attempts)
terminal_halt_class: TerminalHaltClass
```

For clean halts (`OmegaAccepted` / `MaxTxExhausted`), `delta == 0` AND
`attempt_aborted_count == 0` are required. Nonzero `delta` or nonzero
`attempt_aborted_count` is admissible only under named terminal abort
states (`WallClockCap` / `ComputeCapViolated` / `ErrorHalt` / `DegradedLLM`)
WITH explicit `TerminalAbortRecord` per aborted attempt (R1 schema
shipped; see `src/runtime/attempt_telemetry.rs::TerminalAbortRecord`).

### §1.3 Drain barrier semantics (charter FR-18R.3 v2 + Codex Q4)

The G1 equation is evaluated POST-drain. A "drain" means every submitted
typed-tx has reached a terminal state in chain (L4) or L4.E. The runtime
already provides this primitive: `ChaintapeBundle::shutdown(self).await`
sends a oneshot signal, the driver wrapper closes the queue receiver,
drains remaining envelopes, and the join handle resolves only after
all `apply_one` calls complete. This IS the "or equivalent" drain barrier
per charter §1.2 FR-18R.3 v2.

R4 does **not** add a redundant `Sequencer::drain_until_quiescent()`
method — the existing `ChaintapeBundle::shutdown()` discharges the
contract. R4 instead:

  - Adds a quiescence-check helper `verify_chain_quiescent_post_drain` in
    `chain_derived_run_facts.rs` that asserts
    `next_submit_id - 1 == l4_count + l4e_count`. This serves as a
    **defense-in-depth witness** that the drain happened, callable
    post-shutdown for ship-gate evidence.
  - Documents the call-site contract on `compute_attempt_count_invariant_facts`
    so callers cannot accidentally invoke pre-drain.

This satisfies "drain barrier or equivalent" per FR-18R.3 v2 without a
redundant Class-4 sequencer touch (sequencer is a STEP_B-restricted file;
adding new public methods would require its own ratification justification
when an existing primitive already discharges the contract).

## §2 Out of scope for R4 (forwarded to R5+ / R6 / R7)

- `audit_tape` sampler extension to AttemptTelemetry / LeanResult — R5.
- `attempt_chain_root` Merkle root computation on final composite — R5.
- Capturing actual Lean stderr/stdout bytes into CAS (still
  `LeanResult.stderr_cid = None`) — R5+.
- P23/P38/P49 + M0 reruns — R6/R7.
- Modification to historical M1 evidence (per
  `feedback_no_retroactive_evidence_rewrite`).
- Adding new `Sequencer` public methods (the existing `ChaintapeBundle::shutdown`
  + `next_submit_id_peek` / `next_logical_t_peek` primitives suffice; no
  Class-4 sequencer touch).
- Touching Atom-3 fixture origin-tag (CR-18R.5; pre-R3 L4.E fixtures stay
  as-is; runtime-path L4.E from R3 already distinguished by
  `synthetic_rejection_for_l4e_gate=false`).

## §3 Design decisions

### §3.1 Field placement: extend `ChainDerivedRunFacts` vs new struct

**Constraint**: charter §1.2 FR-18R.4 says "chain_derived_run_facts emits
these exact fields". TB-7 ChainDerivedRunFacts already serializes;
R4-impl must extend it without breaking the existing schema (TB-7 audit
+ adapter consumers).

**Candidates**:

| | (A) Extend ChainDerivedRunFacts inline | (B) New AttemptCountInvariantFacts wrapper struct | (C) Sidecar struct on disk only |
|---|---|---|---|
| Charter §1.2 wording match | ✅ "chain_derived_run_facts emits" | ⚠️ separate name | ⚠️ requires extra serialization layer |
| Backward compat | ✅ `#[serde(default)]` on new fields | ✅ orthogonal | ✅ |
| Test plumbing | ✅ single struct | ❌ two structs to thread | ❌ extra disk hop |
| Adapter / dashboard surface | ✅ existing serializers handle | ❌ new serializer needed | ❌ |

**Decision: (A)** — extend `ChainDerivedRunFacts` with 6 new fields, all
`#[serde(default)]` for byte-stable deserialization of pre-R4 JSON.
Pre-R4 evidence runs (R6/R7 not yet run) deserialize cleanly with
default-zero values; new R4+ runs populate them.

### §3.2 `terminal_halt_class` enum

Per charter §1.2 FR-18R.4 list: `terminal_halt_class: TerminalHaltClass`.
The constitutional `RunOutcome` enum at `src/state/typed_tx.rs:191-204`
already enumerates the six halt states:

```rust
pub enum RunOutcome {
    OmegaAccepted = 0,
    MaxTxExhausted = 1,
    WallClockCap = 2,
    ComputeCap = 3,
    ErrorHalt = 4,
    DegradedLLM = 5,
}
```

R4 reuses `RunOutcome` directly as `terminal_halt_class`. **No new enum.**
This avoids enum proliferation per `feedback_no_workarounds_strict_constitution`
(strict alignment to existing constitutional types). Charter §1.2 FR-18R.3
v2 uses "RunOutcome::WallClockCap / ComputeCapViolated / ErrorHalt"
consistent with this reuse (`ComputeCapViolated` is the
`AbortCause::ComputeCapViolated` from R1; `RunOutcome::ComputeCap` is the
run-level halt; both refer to the same constitutional class at different
granularities).

The R1-shipped `TerminalAbortRecord.cause: AbortCause` enum stays the
per-attempt granularity; `ChainDerivedRunFacts.terminal_halt_class:
RunOutcome` is the run-level granularity. Both can populate from the
same evaluator-side terminal halt determination.

### §3.3 Counting `l4_work_attempt_count` and `l4e_work_attempt_count`

The existing `compute_run_facts_from_chain` already walks L4 and L4.E
and tracks `proposal_count`. R4 splits the count along the L4 / L4.E
axis:

  - `l4_work_attempt_count` = number of L4 LedgerEntry whose decoded
    `TypedTx` is `TypedTx::Work`.
  - `l4e_work_attempt_count` = number of L4.E `RejectedSubmissionRecord`
    with `tx_kind == TxKind::Work`.

Both definitions are pure-functional over chain bytes; no CAS dependency
(unlike `proposal_count` for R3 omega/failure path mix, the count just
tallies entries). This intentionally counts ALL Work entries:
  - Atom-3 fixture WorkTx on L4.E (pre-R3) — these are synthetic
    fixtures and **not real attempts**. R6/R7 evidence runs use clean
    runtime_repo dirs (no Atom-3 fixtures); production count is correct.
    Test fixtures that pre-seed synthetic L4.E records are out-of-scope
    for the invariant — the invariant evaluates production-shape runs,
    not synthetic test inputs.
  - Synthetic-rejection L4.E from R3 with origin-tag — same as above
    (test-only).

**For R6/R7 production runs**: `l4_work_attempt_count + l4e_work_attempt_count`
counts exactly one entry per LLM-Lean cycle (omega cycle → 1 L4 Work;
failure cycle → 1 L4.E Work). The invariant equation holds.

**For R4 unit tests**: tests construct fully-runtime-shape chains (no
Atom-3 fixtures injected) so the count semantics are unambiguous.

### §3.4 Counting `attempt_aborted_count`

`attempt_aborted_count` = number of `TerminalAbortRecord` CAS objects
written under the run's CAS root. The existing CAS index already tracks
`ObjectType::TerminalAbortRecord` (R1 schema shipped). R4 walks the CAS
sidecar `.turingos_cas_index.jsonl`, filters entries by `object_type ==
"TerminalAbortRecord"`, and counts.

R3.fix-shipped `CasStore::reload_index_from_sidecar` is the correct
read-path (not relevant here because R4 reads CAS at evidence-aggregation
time, NOT in-run; the CAS handle opened by `compute_run_facts_from_chain`
already loads the full sidecar at construction).

### §3.5 `delta` semantics

Per charter §1.2 FR-18R.4: `delta = l4 + l4e - expected`.

  - `delta == 0` for clean halts (`OmegaAccepted` / `MaxTxExhausted`)
    AND `attempt_aborted_count == 0` is the **only** acceptable shape
    for ship-gate per SG-18R.4.
  - Nonzero `delta` admissible only under terminal abort states with
    explicit `TerminalAbortRecord` per aborted attempt.
  - `delta < 0` would mean the chain has fewer attempts than the
    evaluator reported completing — strictly forbidden (means an
    attempt vanished pre-chain). R4 reports the negative delta but
    `attempt_count_invariant()` returns Err.
  - `delta > 0` would mean the chain has more attempts than reported
    completed — admissible only if `attempt_aborted_count == delta` and
    halt class is one of the abort states. R4 reports + `attempt_count_invariant()`
    returns Ok in that specific configuration.

### §3.6 `attempt_count_invariant()` API

```rust
/// TB-18R FR-18R.3 v2 (G1-ratified canonical contract; Codex Q4
/// remediation): chain-derived ship-gate equation. Populates ratified
/// spec without alteration.
///
/// Returns `Ok(())` iff:
///   - For clean halts (`OmegaAccepted` / `MaxTxExhausted`):
///       delta == 0 AND attempt_aborted_count == 0
///   - For terminal abort states (`WallClockCap` / `ComputeCap` /
///     `ErrorHalt` / `DegradedLLM`):
///       evaluator_reported_completed_llm_calls
///         + attempt_aborted_count
///         == l4_work_attempt_count + l4e_work_attempt_count
///       (and a TerminalAbortRecord exists per aborted attempt — verified
///       by attempt_aborted_count == count of TerminalAbortRecord CAS objects)
///
/// Returns `Err(AttemptCountInvariantViolation)` otherwise.
///
/// **Drain barrier contract**: caller MUST have drained the sequencer
/// (via `ChaintapeBundle::shutdown().await` or equivalent) before
/// invoking. See `verify_chain_quiescent_post_drain` for a witness.
pub fn attempt_count_invariant(
    facts: &ChainDerivedRunFacts,
) -> Result<(), AttemptCountInvariantViolation> { … }
```

### §3.7 `verify_chain_quiescent_post_drain()` API

```rust
/// TB-18R FR-18R.3 v2 drain-barrier witness. Asserts
/// `seq.next_submit_id_peek() - 1 == l4_count + l4e_count` —
/// every issued submit_id has reached terminal state on chain or L4.E.
///
/// Should be called AFTER `ChaintapeBundle::shutdown().await`. Defense
/// in depth: shutdown's join already guarantees drain; this re-asserts
/// the count invariant for ship-gate evidence.
pub fn verify_chain_quiescent_post_drain(
    seq: &Sequencer,
    runtime_repo_path: &Path,
) -> Result<(), DrainBarrierViolation> { … }
```

### §3.8 `compute_attempt_count_invariant_facts` callable

`compute_run_facts_from_chain` already walks L4 + L4.E. R4 extends it to
**also** walk CAS index for TerminalAbortRecord count and split L4 vs
L4.E Work counts. The signature gains an optional input:

```rust
pub struct AttemptCountInvariantInputs {
    pub expected_completed_attempts: u64,
    pub terminal_halt_class: RunOutcome,
}

pub fn compute_run_facts_from_chain_with_invariant(
    runtime_repo_path: &Path,
    cas_path: &Path,
    invariant_inputs: AttemptCountInvariantInputs,
) -> Result<ChainDerivedRunFacts, ChainDerivedError> { … }
```

The original `compute_run_facts_from_chain(runtime_repo, cas)` stays for
TB-7 / TB-8 / adapter back-compat (it sets the new fields to defaults).
R6/R7 evidence runs MUST use the `_with_invariant` variant.

## §4 Files touched (canonical list)

| File | Status | Surface | Diff intent |
|---|---|---|---|
| `src/runtime/chain_derived_run_facts.rs` | MOD (additive) | Class 4 STEP_B (G1-ratified equation) | Extend `ChainDerivedRunFacts` with 6 new fields (`expected_completed_attempts`, `l4_work_attempt_count`, `l4e_work_attempt_count`, `attempt_aborted_count`, `delta`, `terminal_halt_class`), all `#[serde(default)]`; add `AttemptCountInvariantInputs` + `AttemptCountInvariantViolation` + `DrainBarrierViolation` types; add `compute_run_facts_from_chain_with_invariant()` + `attempt_count_invariant()` + `verify_chain_quiescent_post_drain()` functions; extend the existing L4 + L4.E walks to split Work counts and read CAS for TerminalAbortRecord count |
| `genesis_payload.toml` | MOD (rehash) | TR pin | Update SHA pin for `src/runtime/chain_derived_run_facts.rs` (per existing pin at line 253; routine SHA refresh per R1/R3/R3.fix precedent) |
| `tests/tb_18r_chain_attempt_invariant.rs` | NEW | Class 4 witness | Asserts `attempt_count_invariant()` Ok/Err semantics across all halt classes + edge cases |
| `tests/tb_18r_chain_derived_facts_exact_accounting.rs` | NEW | Class 4 witness | Asserts `compute_run_facts_from_chain_with_invariant()` populates all 6 FR-18R.4 fields exactly per charter spec |
| `tests/tb_18r_drain_barrier_quiescence.rs` | NEW | Class 4 witness | Asserts `verify_chain_quiescent_post_drain()` PASS post-shutdown + FAIL when called pre-drain (synthetic non-quiescent state) |

Net delta target: **≥ +6 tests** (3 new test files × ≥2 fns each).
Workspace 1022 → ≥ 1025 (charter §1.4 SG-18R.11 binding minimum +3).

## §5 Test plan

| Test fn | File | Asserts |
|---|---|---|
| `clean_halt_omega_accepted_invariant_passes` | `tb_18r_chain_attempt_invariant.rs` | `OmegaAccepted` halt with `delta=0` + `attempt_aborted_count=0` → `Ok(())` |
| `clean_halt_max_tx_exhausted_invariant_passes` | `tb_18r_chain_attempt_invariant.rs` | `MaxTxExhausted` halt with `delta=0` + `attempt_aborted_count=0` → `Ok(())` |
| `clean_halt_with_nonzero_delta_invariant_fails` | `tb_18r_chain_attempt_invariant.rs` | `OmegaAccepted` halt with `delta != 0` → `Err(AttemptCountInvariantViolation)` |
| `clean_halt_with_nonzero_aborted_invariant_fails` | `tb_18r_chain_attempt_invariant.rs` | `MaxTxExhausted` halt with `attempt_aborted_count > 0` → `Err` |
| `wall_clock_cap_with_aborted_attempts_invariant_passes_when_balanced` | `tb_18r_chain_attempt_invariant.rs` | `WallClockCap` halt with `expected + aborted == l4 + l4e` → `Ok` |
| `wall_clock_cap_with_aborted_attempts_invariant_fails_when_unbalanced` | `tb_18r_chain_attempt_invariant.rs` | `WallClockCap` halt with `expected + aborted != l4 + l4e` → `Err` |
| `negative_delta_always_fails` | `tb_18r_chain_attempt_invariant.rs` | `delta < 0` → `Err` regardless of halt class |
| `compute_with_invariant_populates_all_six_fields` | `tb_18r_chain_derived_facts_exact_accounting.rs` | After bootstrap + submit + shutdown, all 6 FR-18R.4 fields are populated correctly |
| `l4_l4e_split_count_matches_chain_walk` | `tb_18r_chain_derived_facts_exact_accounting.rs` | `l4_work_attempt_count + l4e_work_attempt_count == proposal_count` (legacy proposal_count is the union per TB-7.5 fix #2) |
| `terminal_abort_record_count_from_cas_index` | `tb_18r_chain_derived_facts_exact_accounting.rs` | Pre-write N TerminalAbortRecord CAS objects → `attempt_aborted_count == N` |
| `quiescent_post_shutdown_passes` | `tb_18r_drain_barrier_quiescence.rs` | After `bundle.shutdown().await`, `verify_chain_quiescent_post_drain()` → `Ok` |
| `non_quiescent_pre_shutdown_fails` | `tb_18r_drain_barrier_quiescence.rs` | Synthetic state where `next_submit_id - 1 != l4 + l4e` → `Err(DrainBarrierViolation)` |

12 tests total. Charter SG-18R.11 binding minimum +3; this exceeds.

## §6 Risk matrix

| Risk | Probability | Mitigation |
|---|---|---|
| New `#[serde(default)]` fields on `ChainDerivedRunFacts` break TB-7 / TB-8 adapter / dashboard JSON deserialization | Low (default-zero values are a strict superset) | TB-7 + TB-8 + adapter tests rerun in `cargo test --workspace`; pre-R4 fixtures deserialize unchanged |
| Pre-R3 L4.E Atom-3 fixture WorkTx pollutes `l4e_work_attempt_count` for R4 unit tests | Low (R4 tests use fresh runtime_repo / cas dirs; no Atom-3 fixtures injected) | Test fixtures explicitly construct runtime-path WorkTx only |
| `verify_chain_quiescent_post_drain` reads stale `next_submit_id` after shutdown | Low — `next_submit_id` is `AtomicU64`; post-shutdown reads see all prior submits | Test `quiescent_post_shutdown_passes` exercises path |
| TerminalAbortRecord CAS-index walk hits split-brain (R3.fix pattern) | Low — R4 reads CAS post-shutdown via fresh handle; no in-run write/read race | `compute_run_facts_from_chain_with_invariant` opens CAS at function entry (existing pattern from `compute_run_facts_from_chain`); always sees fully-flushed sidecar |
| Equation drift on edge cases (genesis empty chain) | Low — empty chain: `l4=0`, `l4e=0`, `expected=0`, `aborted=0`, `delta=0`, `terminal_halt_class=OmegaAccepted` (or run never started, in which case the invariant is vacuously Ok by `expected=0`) | `compute_with_invariant_populates_all_six_fields` covers genesis |
| Trust Root rehash for chain_derived_run_facts.rs trips audit | Low — same precedent as TB-7R parent_tx pin update at line 253 | Routine pin refresh; comment-chain entry cites this preflight |
| New types (`AttemptCountInvariantInputs`, `AttemptCountInvariantViolation`, `DrainBarrierViolation`) inadvertently expose internal types | Mitigated — types are pure data + error enums; no `Sequencer` / `CasStore` internals leaked | Diff review confirms |
| RunOutcome import cycle (chain_derived_run_facts → state::typed_tx) | Low — module already imports `state::typed_tx::TypedTx`; adding `RunOutcome` is no-op | Compiler verifies |

## §7 STEP_B parallel-branch plan (binding)

Per CLAUDE.md STEP_B_PROTOCOL + R3 STEP_B precedent (`handover/ai-direct/TB-18R_R3_STEP_B_admission.md` §7):

1. **Phase 0 — necessity audit (this preflight)**: Design (A)+(B) populate-not-alter equation documented (§1.1, §3); user reviews preflight before worktree creation.
2. **Phase 1a — worktree spawn**:
   ```bash
   git worktree add .claude/worktrees/stepb-tb18r-r4-invariant -b tb-18r-r4-invariant main
   ```
   Off `main` at HEAD `63fde2b` (post-R3.fix ship + TB_LOG row).
3. **Phase 1b — implementation in isolation**:
   - Extend `ChainDerivedRunFacts` with 6 new fields (`#[serde(default)]`).
   - Add `AttemptCountInvariantInputs` / `AttemptCountInvariantViolation` / `DrainBarrierViolation` types.
   - Add `compute_run_facts_from_chain_with_invariant()` function.
   - Add `attempt_count_invariant()` function (G1-ratified equation populated verbatim).
   - Add `verify_chain_quiescent_post_drain()` function.
   - Add 3 new integration test files (12 tests total).
   - Update Trust Root pin for `chain_derived_run_facts.rs` (genesis_payload.toml line 253).
   - `cargo check --workspace` clean compile.
   - `cargo test --workspace --no-fail-fast` → 1022 → ≥ 1025; 0 R4-attributable failures.
4. **Phase 1c — implementation audit**:
   - Diff review (no statistical A/B; this is mechanism-edit not behavior-edit; per STEP_B_PROTOCOL.md "Phase 2 statistical A/B" applies to behavior changes; R4 is a derived-facts schema + pure-function addition validated by mechanism tests).
   - User reviews diff before merge.
5. **Phase 2 (skipped)**: no statistical A/B; R6/R7 evidence runs serve as the empirical test of the full invariant chain.
6. **Phase 3 — merge**:
   - On user approval: `git merge tb-18r-r4-invariant --no-ff` on main.
   - TB_LOG.tsv R4 ship row appended.
   - Worktree removed: `git worktree remove .claude/worktrees/stepb-tb18r-r4-invariant`.
7. **Phase 3b — abandonment** (if anything goes red): branch archived as `archive/tb-18r-r4-invariant_2026-05-06`; main remains at `63fde2b`.

**Iteration cap** (charter §0.A R4 row): **24h post-Gate-1**. Gate 1 closed
2026-05-06; deadline = 2026-05-07 + STEP_B preflight time. R4 launches when
user grants explicit "go" on this preflight.

## §8 Validation steps before R4 ship

1. `cargo check --workspace` clean compile.
2. `cargo test --workspace --no-fail-fast` → 1022 → ≥ 1025; 0 R4-attributable failures.
3. Diff review:
   - `ChainDerivedRunFacts` new fields all `#[serde(default)]`.
   - No `Sequencer` public-method addition (drain barrier honored via existing primitives).
   - No alteration to G1-ratified equation (literal text match: `evaluator_reported_completed_llm_calls == l4_work_attempt_count + l4e_work_attempt_count` and `evaluator_observed_llm_call_starts == evaluator_reported_completed_llm_calls + attempt_aborted_count`).
   - No ±N tolerance.
4. Trust Root rehash: `genesis_payload.toml:253` SHA matches new file content; comment-chain entry cites this preflight.
5. Commit on worktree branch; report deltas to user; user reviews + merges.
6. TB_LOG.tsv R4 ship row appended.

## §9 Forbidden in R4 (preflight binding)

- **Alter the G1-ratified equation** in any way (charter §0.A Q1+Q4 lock; preflight §1.1).
- Introduce ±N tolerance (Codex Q4 ratified deterministic + drain barrier; charter §0.A row Q4).
- Substitute approximate accounting (e.g. "within 1 of expected") in lieu of exact equality.
- Touch `WorkTx` canonical wire bytes (R1 Design B inherited; preflight §1.1; would re-trigger Class-4 ratification).
- Touch Atom-3 fixture origin-tag (CR-18R.5; pre-R3 L4.E fixtures stay as-is; runtime-path L4.E from R3 already has `synthetic_rejection_for_l4e_gate=false`).
- Add new `Sequencer` public methods (drain barrier discharged by existing `ChaintapeBundle::shutdown` + `next_submit_id_peek` / `next_logical_t_peek` primitives; new sequencer methods would be Class-4 sequencer surface needing separate ratification).
- Introduce a new enum for `terminal_halt_class` (reuses constitutional `RunOutcome`; preflight §3.2).
- Modify `compute_run_facts_from_chain` signature (back-compat invariant; new function `compute_run_facts_from_chain_with_invariant` is additive).
- Capture Lean stderr/stdout into CAS (still R5+ scope).
- Compute attempt_chain_root Merkle (R5 scope).
- Modify historical M1 evidence (per `feedback_no_retroactive_evidence_rewrite`; CR-18R.1).
- Wire P23/P38/P49 reruns (R6 scope).
- Wire M0 small batch (R7 scope).

## §10 Sign-off

This preflight grants implementation authority for R4 within the listed
scope **upon user "go"** per charter §1.1 Opening rule for Class-4 atoms
(Codex Gate 1 ratified the equation **shape**; user explicit "go"
authorizes the IMPLEMENTATION that populates ratified spec without
alteration per charter §0.A Q1).

  - Pre-impl: this doc filed under `handover/ai-direct/`; user reviews + grants "go".
  - Phase 1a: worktree + branch creation.
  - Phase 1b: implementation + tests in worktree.
  - Phase 1c: diff review (Claude self-pass + user review).
  - Phase 3: merge to main on user approval; TB_LOG row appended.

**End of R4 preflight. Awaits user "go" before Phase 1a worktree creation.**
