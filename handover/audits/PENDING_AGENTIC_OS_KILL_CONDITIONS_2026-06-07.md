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

> ## POST-§8 UPDATE (2026-06-07) — route-A landed; G1 promoted, G2/G3 stay red
>
> Under §8 token `APPROVE-M07-A4-SINGLE-ADMISSION-PREDICATE-GATE` the route-A
> single-admission src change landed (shared `src/predicate_admission.rs`
> contract; kernel head advance gated on `decide_admission` + additive
> `predicate_admission` PASS receipt). Outcome on the kill-conditions:
>
> - **G1 — PROMOTED + GREEN.** Moved to
>   `tests/constitution_kernel_predicate_gate.rs`, registered in
>   `scripts/constitution_gates.manifest.toml` and the execution matrix. The
>   advancing `StateAccepted` node now carries a tape-recorded
>   `predicate_admission` PASS receipt, so the bypass is closed and enforced.
> - **G6 (NEW) — GREEN.** A structural anti-duplication witness
>   `tests/constitution_single_admission_contract.rs` (spec §6) was added and
>   triple-coupled. It proves the verdict-trusting zero-root scan has exactly one
>   home (`decide_admission`) called by both authorities.
> - **G2 — STILL RED (not promotable as written).** G2 asserts THREE mutually
>   exclusive facts simultaneously (`kernel_admitted==true` ∧
>   `sequencer_admitted==false` ∧ `kernel_admitted==sequencer_admitted`). Its
>   kernel leg drives the bare 3-arg `step_forward` (empty claims → zero-root
>   PASS → admits=true) while its sequencer leg submits a FALSE-predicate WorkTx
>   (rejects → false). No route-A src change can make `true==false` hold; the
>   gate would have to be REWRITTEN to feed the kernel a failing claim set
>   (`step_forward_with_claims` with a false acceptance claim) before it can be a
>   green single-admission witness. That rewrite is out of scope of this §8
>   token (it changes what the gate asserts). G2 remains an EXPECTED-RED
>   kill-condition in the pending runner.
> - **G3 — STILL RED (needs architect ruling).** G3 requires the sequencer to
>   REFUSE a zero-root self-asserted-true WorkTx. The spec's recommended
>   `os_qualified = (registry_root != ZERO)` makes genesis zero-root runs
>   non-qualified → admitted, so G3 stays red. Forcing it green requires
>   `os_qualified = true` for zero-root runs, which REGRESSES 15+ existing
>   workspace tests that legitimately admit zero-root WorkTx with true predicates
>   (e.g. `tests/tb_8_minimal_payout.rs:223` `.expect("work ok")` under a genesis
>   zero root; also `constitution_fc1_runtime_loop`, `tb_2/3/4`). The HARD
>   CONSTRAINT "behavior-preserving for the sequencer zero-root branch" forbids
>   that regression. G3 is the spec's Open Question #1 (os_qualified source):
>   distinguishing a G3 zero-root run from a legitimate tb_8 zero-root run needs
>   a NEW run-level OS-qualified field the registry root alone cannot supply.
>   G3 remains an EXPECTED-RED kill-condition pending that architect decision.
> - **G4 / G5 — STANDING pending** a separate §8 (budget hard-ceiling ruling;
>   FC3 irreversible-commit Class-4 ratification). Unchanged.
>
> Net pending runner after this turn: G2/G3/G4/G5 EXPECTED-RED, exit 0
> (`expected-red=4, unexpected-pass=0, compile-break=0`). G1 no longer appears
> in the pending runner — it is a live constitution gate.

> ## POST-§8 UPDATE 2 (2026-06-07) — G2 retired + replaced by a live behavioral gate
>
> Per §8 packet `handover/section8/M07_G2_G3_GATE_REDESIGN_DECISION_2026-06-07.md`
> §5 (under the existing `APPROVE-M07-A4-SINGLE-ADMISSION-PREDICATE-GATE` token,
> test-only): the pending G2 file
> `tests/pending/constitution_kernel_sequencer_single_admission.rs` was
> **logically self-contradictory** — it asserted `kernel_admitted == true` AND
> `sequencer_admitted == false` AND `kernel_admitted == sequencer_admitted`
> (i.e. `true == false`). That is a broken test, not a falsifiable kill-condition
> (AGENTS.md §7): its kernel leg fed the empty 3-arg `step_forward` (zero-root
> PASS → admit) while its sequencer leg submitted a false-predicate WorkTx
> (reject), so the two legs decided DIFFERENT claims and were then asserted to
> agree.
>
> - **G2 — RETIRED.** The self-contradictory pending file was DELETED and removed
>   from `scripts/run_pending_agentic_os_kill_conditions.sh`.
> - **G2 (behavioral) — NEW, LIVE, GREEN.** Replaced by a CORRECT behavioral gate
>   `tests/constitution_single_admission_behavioral.rs`: feed BOTH authorities the
>   SAME claim and assert they AGREE. A FALSE acceptance claim is rejected by both
>   (`!kernel_admitted && !seq_admitted && kernel_admitted == seq_admitted`); a
>   TRUE claim is admitted by both (positive control). The kernel leg now drives
>   `step_forward_with_claims` with a real failing claim → `decide_admission` →
>   `Fail` → `handle_rejection` (no head advance). Triple-coupled into
>   `scripts/constitution_gates.manifest.toml`
>   (`constitution_single_admission_behavioral`) and the execution matrix.
>
> Net pending runner after this turn: **G3/G4/G5** EXPECTED-RED, exit 0
> (`expected-red=3, unexpected-pass=0, compile-break=0`). G1 and G2 no longer
> appear in the pending runner — both are live constitution gates.

> ## POST-§8 UPDATE 3 (2026-06-07) — G3 promoted under a NEW Class-4 §8 token
>
> Per §8 packet `handover/section8/M07_G2_G3_GATE_REDESIGN_DECISION_2026-06-07.md`
> §6 and the user's **new Class-4** token
> `APPROVE-M07-G3-OS-QUALIFIED-RUN-FIELD` (separate from the route-A token, which
> scoped itself to "no schema surface"): the architect ruling on the
> `os_qualified` source landed as a **run-level QState field**.
>
> - **Field**: `QState::os_qualified_t: bool` (`src/state/q_state.rs`, a
>   trust-root-pinned surface — pin rehashed in the same commit). `false` at
>   genesis/default, preserving every legacy zero-root suite (tb_8 et al. still
>   admit zero-root WorkTx with true predicates). It is **independent of**
>   `predicate_registry_root_t`, so the previously-dead refuse-path is reachable;
>   it is folded into `state_root_t` and replayable from tape. Flipped
>   `false→true` by the system-only `PredicateBindingActivate` accept in
>   `src/state/sequencer.rs`.
> - **Rewire**: both admission legs now read the field, not `registry_root !=
>   ZERO` — `src/state/sequencer.rs` zero-root branch (`q.os_qualified_t`) and
>   `src/memory_kernel.rs` Proceed branch (kernel `os_qualified_t`). For an
>   OS-qualified run (`os_qualified_t == true`) a zero registry root →
>   `decide_admission` → `Fail(ZeroRootRefusedForOsQualifiedRun)` → REFUSED.
> - **G3 — PROMOTED + GREEN.** Moved to
>   `tests/constitution_predicate_zero_root_is_not_oracle.rs`, registered in
>   `scripts/constitution_gates.manifest.toml`
>   (`constitution_predicate_zero_root_is_not_oracle`) and the execution matrix.
>   The test marks the run OS-qualified (`q.os_qualified_t = true`) and asserts the
>   zero-root self-asserted WorkTx is refused. G3 no longer appears in the pending
>   runner.
>
> Net pending runner after this turn: **G4/G5** EXPECTED-RED, exit 0
> (`expected-red=2, unexpected-pass=0, compile-break=0`). G1, G2, and G3 are all
> live constitution gates.

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
| G2 single-admission (RETIRED → live `constitution_single_admission_behavioral`) | FC1 ⨯ FC2 admission | TWO admission authorities (kernel vs sequencer) must reach the SAME predicate verdict for the same claim | `src/memory_kernel.rs` Proceed branch vs `src/state/sequencer.rs` zero-root branch |
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

### G2 — RETIRED (was `tests/pending/constitution_kernel_sequencer_single_admission.rs`)
> **RETIRED 2026-06-07** per §8 packet
> `handover/section8/M07_G2_G3_GATE_REDESIGN_DECISION_2026-06-07.md` §5 — see
> POST-§8 UPDATE 2 above. The as-written pending G2 was logically
> self-contradictory (asserted `kernel_admitted==true` ∧ `seq_admitted==false` ∧
> `kernel_admitted==seq_admitted`, i.e. `true==false`): its kernel leg fed the
> empty 3-arg `step_forward` (zero-root PASS) while the sequencer leg submitted a
> false-predicate WorkTx, so the legs decided DIFFERENT claims then asserted
> agreement. That is documentation, not a falsifiable gate (AGENTS.md §7). The
> file was DELETED. The single-admission invariant is now enforced LIVE by the
> behavioral gate `tests/constitution_single_admission_behavioral.rs` (both
> authorities fed the SAME claim must agree) plus the structural anti-duplication
> gate `tests/constitution_single_admission_contract.rs`.

The original (broken) intent and why it could not be promoted as written:
- **Intended to prove**: feeds one logical "failing-predicate" claim to BOTH
  authorities and asserts they agree.
- **Why broken**: the kernel leg drove the 3-arg `step_forward` with an EMPTY
  claim set (zero-root PASS → admit=true) instead of a failing claim, so it
  asserted `true == false`. No source can satisfy that.
- **Correct replacement**: `tests/constitution_single_admission_behavioral.rs`
  feeds the kernel a real failing claim via `step_forward_with_claims` → both
  authorities reject → `kernel_admitted == seq_admitted == false`. §8 token
  `APPROVE-M07-A4-SINGLE-ADMISSION-PREDICATE-GATE`.

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

### G5 — `tests/pending/constitution_fc3_meta_loop_closure.rs` (PROMOTED OUT 2026-06-08)
> **PROMOTED OUT 2026-06-08** under §8 Class-4 token
> `APPROVE-FC3-RUNTIME-VETO-AND-TRUSTROOT-REINIT`
> (`handover/section8/APPROVE_FC3_RUNTIME_VETO_AND_TRUSTROOT_REINIT_2026-06-07.md`).
> The FC3 irreversible leg landed: a deterministic runtime Veto-AI `{Accept,Reject}`
> clause-walker (`src/runtime/real5_roles/fc3_veto.rs`) gates a PASS-only
> ArchitectCommit + SANDBOX trust-root recompute + tape-visible re-init
> (`src/runtime/real5_roles/fc3_commit_reinit.rs`). Both observations now flip
> GREEN: (A) the live `fc3_proposer` carries a real `ArchitectProposalCapsule`
> spec, and (B) a Veto-AI PASS reaches the loop-closing `"reinit:committed"`
> terminal (`closes_fc3_loop == true`). The gate is now the LIVE top-level gate
> `tests/constitution_fc3_meta_loop_closure.rs`, triple-coupled in the manifest +
> matrix, and REMOVED from this pending runner's `PENDING_GATES` array. The
> recompute runs against a TEMP-DIR manifest only (never the real
> `genesis_payload.toml`); a concrete live boot-manifest re-pin carries its own
> signed `v4-ratify` tag (G-GUARD-4). The standing-pending description below is
> the PRE-promotion record (kept for provenance; no longer the current state).

**Test (PRE-promotion record)**: `m07_fc3_meta_loop_must_close_with_tape_visible_reinit`
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

> NOTE 2026-06-07 (POST-§8 UPDATE 2): the verbatim runner output below is the
> ORIGINAL Phase-1 capture (5 pending gates, all EXPECTED-RED). After G1 was
> promoted and G2 retired + replaced by a live behavioral gate, the residual
> pending set is **G3/G4/G5** and the runner reports `expected-red: 3,
> unexpected-pass: 0, compile-break: 0` (still exit 0). Historical capture
> preserved (no retroactive evidence rewrite, AGENTS.md §8).

```text
# Pending runner — all 5 red as expected, exit 0 (ORIGINAL Phase-1 capture)
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
