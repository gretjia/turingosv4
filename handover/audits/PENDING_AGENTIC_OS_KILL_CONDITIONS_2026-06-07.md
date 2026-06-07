# PENDING Agentic-OS Kill-Condition Gates — Phase 1 (M07 PRE-§8 prep)

**Date**: 2026-06-07
**Obligation**: OBL-016 (PR #314 后续 M07 收敛计划) — Phase 1 deliverable
**Branch**: `claude/m07-pr314-followup-prep` (base `fc839ae7` = PR #314)
**Risk class**: Class 1 (additive pending test files + dev-only runner) +
Class 0 (this doc). **No `src/` change. No admission behavior change.**
**§8 token requested (NOT yet consumed)**:
`APPROVE-M07-A4-SINGLE-ADMISSION-PREDICATE-GATE`
(plus separate §8 legs for zero-root quarantine, single-admission invariant, and
the FC3 irreversible-commit path).

> **This is PRE-§8 prep only.** These gates are kill-conditions that *demonstrate
> on a real run* the Class-4 admission-topology gaps M07 must close. They are
> DELIBERATELY EXCLUDED from default CI and EXPECTED TO FAIL (red). The fixes
> that turn them green touch `src/state/sequencer.rs`, `src/memory_kernel.rs`,
> and the FC3 runtime — all Class-4 surfaces BLOCKED until the user supplies the
> §8 token(s). Nothing here changes admission behavior.

---

## 1. Why these gates exist (FC trace)

| Gate | FC node | The bypass it demonstrates | Source line |
|------|---------|----------------------------|-------------|
| G1 kernel-predicate | FC1 runtime loop (`Q_t → … → wtool → Q_{t+1}`) | `MemoryKernel` advances `verified_head` purely on `success + Proceed`, predicate-blind | `src/memory_kernel.rs:171-188` |
| G2 single-admission | FC1 ⨯ FC2 admission | TWO admission authorities (kernel vs sequencer) reach DIFFERENT predicate verdicts for the same claim | `src/memory_kernel.rs:171-188` vs `src/state/sequencer.rs:1225` |
| G3 zero-root-not-oracle | FC2 predicate/verifier (L1) | zero registry root trusts self-reported booleans instead of re-executing the oracle | `src/state/sequencer.rs:1231` |
| G4 budget ceiling | FC2 boot budget (Art. V.2) | no admission gate compares `budget_state_t` against any ceiling; over-budget runs admit | `src/state/q_state.rs:153-160`; `constitution.md:796-797` |
| G5 FC3 meta-loop | FC3 meta-architecture | substrate live but runtime engine missing: proposer inert, loop dead-ends at sandbox canary, never re-inits | `src/runtime/real5_roles.rs:464-469,1077-1091`; `fc3_governance_reinit_current_kernel.rs` |

These are the M07 facts already grep-verified by OBL-016; this Phase 1 turns each
into a *runnable, red, public-API* demonstration on the current source.

---

## 2. Per-gate detail

Each gate compiles against the real public API (`turingosv4::memory_kernel`,
`::ledger`, `::state::sequencer`, `::state::typed_tx`, `::state::q_state`,
`::top_white::predicates::registry`, `::runtime::real5_roles`,
`::charter_core`, `::tokenizer`) — no private symbols. Each first ASSERTS the
current (broken) precondition holds, then asserts the DESIRED post-fix
invariant, which fails red today. So the red is a genuine demonstration of the
gap, not a harness/precondition failure.

### G1 — `tests/pending/constitution_kernel_predicate_gate.rs`
**Test**: `m07_kernel_must_not_advance_verified_head_without_predicate_admission_receipt`
- **Proves**: the kernel commits `NodeKind::StateAccepted` and calls
  `set_verified_head()` on a bare worker `Proceed` (+ `env_result.success`),
  with no `verify_work_predicates` call, no `WorkTx`, no `PredicateRegistry`,
  and NO predicate-admission receipt on tape.
- **Current red**: the post-fix invariant (head advance must be backed by a
  tape-recorded predicate-admission PASS receipt) fails — no such receipt
  exists (`NodeKind` has no admission-receipt variant; the `StateAccepted`
  payload is only `{state_update, output_summary}`).
- **Promotes when**: the single-admission predicate gate lands and the kernel's
  head advance is gated on a tape-recorded predicate-admission PASS. §8 token
  `APPROVE-M07-A4-SINGLE-ADMISSION-PREDICATE-GATE`.
- **Standing token**: `M07_EXPECTED_RED`.

### G2 — `tests/pending/constitution_kernel_sequencer_single_admission.rs`
**Test**: `m07_kernel_and_sequencer_must_share_one_predicate_admission_contract`
- **Proves**: feeds one logical "failing-predicate" claim to BOTH authorities.
  The kernel ADMITS (advances `verified_head`) on a bare `Proceed`; the
  sequencer REJECTS the equivalent `WorkTx` whose acceptance predicate is
  `false` (zero-root branch, `sequencer.rs:1231-1235`). `kernel_admitted=true`,
  `sequencer_admitted=false`.
- **Current red**: the invariant `kernel_admitted == sequencer_admitted` fails
  (`true != false`) — two admission authorities, two verdicts.
- **Promotes when**: both paths route through ONE shared predicate-admission
  contract. §8 token `APPROVE-M07-A4-SINGLE-ADMISSION-PREDICATE-GATE`.
- **Standing token**: `SINGLE_ADMISSION_EXPECTED_RED`.

### G3 — `tests/pending/constitution_predicate_zero_root_is_not_oracle.rs`
**Test**: `m07_os_qualified_run_must_not_admit_under_zero_predicate_registry_root`
- **Proves**: a `WorkTx` carrying a SELF-ASSERTED acceptance predicate
  (`value=true, proof_cid=None`) is ADMITTED under
  `predicate_registry_root_t == Hash::ZERO` with no oracle re-execution
  (`sequencer.rs:1231`). Paired positive control: under a NON-ZERO bound
  registry root the oracle branch (line ~1245) REJECTS the same unexpected
  self-asserted key (`AcceptancePredicateUnexpected`) — no blind boolean trust.
- **Current red**: the desired invariant (an OS-qualified run must NOT
  admit/oracle-replay at a zero registry root) fails because zero-root
  admission succeeds.
- **Promotes when**: OS-qualified admission requires a non-zero bound registry
  root → real re-execution. §8 token
  `APPROVE-M07-A4-SINGLE-ADMISSION-PREDICATE-GATE` (zero-root quarantine leg).
- **Standing token**: `ZERO_ROOT_EXPECTED_RED`.

### G4 — `tests/pending/constitution_budget_ceiling_enforced.rs` (STANDING)
**Test**: `m07_over_budget_run_must_be_rejected_at_admission`
- **Proves**: with `budget_state_t` drained to zero remaining compute cap, zero
  wall-clock, and zero cost-ceiling headroom, a normal `WorkTx` is still
  ADMITTED — no sequencer admission gate compares `budget_state_t` against any
  Art. V.2 ceiling.
- **Current red**: the desired invariant (over-budget run rejected at
  admission) fails because the WorkTx admits (`Ok(LedgerEntry)`).
- **STANDING — promotes only on a USER §8 RULING**: (1) whether the Art. V.2
  numbers (`constitution.md:796-797`, headed "下面给出一些可能的宪法级约束" —
  *possible* constraints) are HARD admission ceilings or illustrative examples,
  AND (2) the requirement that the concrete ceiling values come from
  genesis/manifest, never hardcoded (`CLAUDE.md` forbids hardcoded behavior
  parameters; integer-only on the money/compute path). This is not a
  "fix-coming" red — it awaits a scoping decision.
- **Standing token**: `BUDGET_CEILING_STANDING_PENDING`.

### G5 — `tests/pending/constitution_fc3_meta_loop_closure.rs` (STANDING)
**Test**: `m07_fc3_meta_loop_must_close_with_tape_visible_reinit`
- **Proves**: (A) the live role-path proposal payload is
  `ToolProposalPayload::default()` (`proposal_id == None`) — the ArchitectAI
  proposer carries no real spec; (B) the terminal status of an ACCEPTED proposal
  is `proposal_activation_status(..) == "sandbox:canary_only"`
  (`real5_roles.rs:1089`) — a dead-end, never a re-init / trust-root recompute /
  constitution-bound hash advance. (The FC3 governance binary stamps the SAME
  `constitution_source_hash()` at every stage, so the "re-init" never mutates
  the constitution-bound state.)
- **Current red**: assertion (A) fails first — proposer inert. The substrate
  (typed-tx variants + sequencer arms + deterministic Veto-AI checks) IS live;
  the RUNTIME ENGINE is missing.
- **STANDING — promotes only on §8 CLASS-4 RATIFICATION** of the FC3
  irreversible-commit path. Closing FC3 means an ArchitectAI proposal, after a
  Veto-AI PASS, drives a tape-visible re-init that recomputes the boot Trust
  Root — that touches RootBox / boot trust-root authority and the
  constitution-amendment boundary (Art. V.1.1), so it is Class-4 and needs
  per-atom §8 sign-off.
- **Standing token**: `FC3_META_LOOP_STANDING_PENDING`.

---

## 3. Exclusion mechanism — why main CI stays green by design

The gates dodge all THREE constitution-gate discovery surfaces while staying
compile-checked and runnable-red:

1. **`cargo test --workspace`** does NOT build them. Cargo only auto-targets
   FLAT `tests/*.rs`; files in the `tests/pending/` SUBDIRECTORY are never
   integration targets. Proven empirically by the long-standing
   `tests/pending_probe/zzz_probe_should_not_autocompile.rs` (deliberately
   invalid Rust that never breaks CI). **No `Cargo.toml` edit was made** — on
   this worktree `Cargo.toml` is pinned in the Trust Root
   (`genesis_payload.toml`), so any edit would trip
   `src/boot.rs::verify_trust_root` (`TRUST_ROOT_TAMPERED`, Class-4) — forbidden
   PRE-§8. The RECON-1 `[[test]] test=false` mechanism is therefore UNUSABLE
   here; we use the standalone-`rustc` fallback instead.
2. **`scripts/run_constitution_gates.sh`** does NOT discover them. Its glob is
   the flat, non-recursive `ls tests/constitution_*.rs`; it never recurses into
   `tests/pending/`. The files are also not in
   `scripts/constitution_gates.manifest.toml`, so its bidirectional
   discovered⨯manifest cross-check stays empty on both sides.
3. **`tests/constitution_matrix_drift.rs`** does NOT see them. That gate is
   manifest-driven (`manifest_gates()` parses `name = "..."` lines); since no
   pending gate is registered in the manifest, the subset-of-matrix assertion
   is unaffected and the allowlist size cap is untouched.

**None of these gates are registered in the constitution manifest.** This is
deliberate: they are kill-conditions standing pending §8, not constitution
gates. Promotion (post-§8) is the documented triple-coupling move: rename to
`tests/constitution_*.rs` AND add a matching manifest entry AND add a
matrix/allowlist reference, in one atomic change.

### Runner
`scripts/run_pending_agentic_os_kill_conditions.sh` (dev-only; classified in
`tests/fixtures/liveness/script_liveness_inventory.toml` as
`pending_agentic_os_kill_condition_runner`, `classification=dev_harness`,
`counts_for_obl005_script_closure=false`). It:
- runs `cargo build --tests` once to materialize the `turingosv4` rlib + dep
  rlibs in `target/debug/deps`,
- compiles each pending gate standalone via
  `rustc --edition 2021 --test --extern turingosv4=<rlib> [--extern tokio/tempfile/serde_json/serde] -L dependency=target/debug/deps`,
- runs each binary, treats NON-ZERO exit (`test result: FAILED`) as the EXPECTED
  red state and prints the gate's standing token,
- errors LOUDLY (exit 1) if any gate UNEXPECTEDLY PASSES (premature wire-up or a
  vacuous assertion that must be fixed before §8) or fails to COMPILE (real API
  drift, not the intended assertion-red),
- exits 0 iff every gate is in its expected (compiles + asserts-red) state.

It is explicitly NOT a constitution gate, does NOT run inside
`run_constitution_gates.sh`, and does NOT block default CI.

---

## 4. Verification (this turn, verbatim)

```text
# Pending runner — all 5 red as expected, exit 0
$ bash scripts/run_pending_agentic_os_kill_conditions.sh
  ...
  M07_EXPECTED_RED                 (gate red as expected — standing pending §8)
  SINGLE_ADMISSION_EXPECTED_RED    (gate red as expected — standing pending §8)
  ZERO_ROOT_EXPECTED_RED           (gate red as expected — standing pending §8)
  BUDGET_CEILING_STANDING_PENDING  (gate red as expected — standing pending §8)
  FC3_META_LOOP_STANDING_PENDING   (gate red as expected — standing pending §8)
  === SUMMARY ===  expected-red: 5  unexpected-pass: 0  compile-break: 0
  RESULT: ALL-PENDING-RED-AS-EXPECTED (standing pending §8 token)   # exit 0

# Drift gate — green
$ cargo test --test constitution_matrix_drift
  test result: ok. 3 passed; 0 failed; ...

# Constitution gates runner, with OBLIGATIONS.md at its committed state
# (isolates this turn's deliverables from the parallel OBL-016 headline edit)
$ bash scripts/run_constitution_gates.sh
  [k-1-5] total=168 failed=0   # exit 0
```

**Pre-existing dirty-tree note**: on the live worktree, `OBLIGATIONS.md` carries
an uncommitted edit from a parallel OBL-016 session (headline changed from
`OBL-ALL-CLOSED` to `OBL-001..015 CLOSED / OBL-016 BLOCKED-ON-§8`). That edit
alone red-flags two obligation gates
(`constitution_obl005_final_closure_witness`,
`constitution_obligation_repair_reconciliation`) which assert the literal
`OBL-ALL-CLOSED` string. With `OBLIGATIONS.md` restored to its committed state,
both gates pass (10 + 4 = 14 passed / 0 failed) and the full gate runner is
`total=168 failed=0`. Per AGENTS.md §8 the parallel-session edit was NOT
reverted. **This turn's deliverables (the 5 `tests/pending/` gates, the runner,
the fixture classification) introduce ZERO new test failures and ZERO new
manifest gates.**

---

## 5. Constraint compliance

- No `src/` file touched. No `constitution.md`, `genesis_payload.toml`,
  `build.rs`, `Cargo.toml`, `Cargo.lock`, sequencer, memory_kernel, typed_tx,
  or `scripts/constitution_gates.manifest.toml` change.
- No admission behavior changed. All deliverables are Class 0 (this doc) /
  Class 1 (additive pending tests + dev-only runner + one test-fixture
  classification line).
- Class-4 admission-topology work (the fixes that turn these gates green) is
  BLOCKED awaiting the user's §8 token(s). This document and the gates only
  *describe and demonstrate* the gaps; they do not close them.
