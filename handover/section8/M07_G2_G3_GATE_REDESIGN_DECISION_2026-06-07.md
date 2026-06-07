# §8 Decision Packet — M07 G2/G3 Gate Redesign & `M07_GREEN` Correction

- **Date:** 2026-06-07
- **Branch:** `claude/m07-pr314-followup-prep` (worktree `turingosv4-m07-prep`)
- **Risk class:** This packet is **Class 0** (a decision record). It *requests*
  one **Class 4** ratification (the G3 run-level field — see §6).
- **Predecessor / dependency:** `handover/section8/APPROVE_M07_SINGLE_ADMISSION_PREDICATE_GATE_2026-06-07.md`
  (route-A authorization, token `APPROVE-M07-A4-SINGLE-ADMISSION-PREDICATE-GATE`).
  This packet does **not** supersede it; it corrects the `M07_GREEN` closure
  predicate that packet defined and resolves its two unreachable legs (G2, G3).
- **Spec:** `handover/design/M07_SINGLE_ADMISSION_IMPLEMENTATION_SPEC_2026-06-07.md`
- **FC trace:** FC1 (runtime loop admission) × FC2 (boot/L5 predicate registry) —
  same nodes as the route-A change.

---

## §0. TL;DR (the decision in three lines)

1. **The route-A change CANNOT make G2 and G3 green.** It already landed and
   correctly closes **G1**. G2-as-written is **logically self-contradictory**
   (no source can satisfy it). G3 is **structurally unreachable** under the
   `os_qualified = (registry_root != ZERO)` derivation that route-A shipped.
2. **`M07_GREEN` as currently written is unsatisfiable** and must be amended:
   "G1 ∧ G2 ∧ G3 flip together under one token" is **false**. The three gates
   have three different fix-classes (§3).
3. **G2** → retire the self-contradictory behavioral assertion; keep the
   structural anti-duplication gate already added; **recommended:** add a
   *correct* behavioral gate (§5). **G3** → requires a **new run-level
   `os_qualified` field** (QState schema, trust-root pinned) under a **new
   Class-4 §8** (§6). It is explicitly **out of scope** for the existing
   `APPROVE-M07-A4-…` token (which scoped itself to "no schema surface").

---

## §1. Verified current state (read from the working tree, not the spec)

The route-A src change is present (uncommitted) in this worktree:
`src/predicate_admission.rs` (new), `src/memory_kernel.rs`, `src/state/sequencer.rs`,
`src/lib.rs`, `genesis_payload.toml` (pins rehashed), `src/tdma_runner.rs`.

A **concurrent promotion session** is also active in this worktree (file mtimes
2026-06-07 06:37). It has already:

- **Promoted G1** → `tests/pending/constitution_kernel_predicate_gate.rs` renamed
  to `tests/constitution_kernel_predicate_gate.rs`, registered in
  `scripts/constitution_gates.manifest.toml`.
- **Added a structural gate** `tests/constitution_single_admission_contract.rs`
  (greps `src/` for the anti-duplication invariant: `decide_admission` has one
  home, both legs call it, kernel decides before advancing head).
- **Rewritten the pending runner** so G1 is promoted out and G2/G3/G4/G5 remain
  the residual set, with the in-source comment:
  *"G2 as written is self-contradictory; G3 awaits an architect ruling on the
  os_qualified source."*

This packet **agrees with and formalizes** that diagnosis (reached
independently from source) and supplies the ruling the runner comment defers.

> Note on evidence: a live runner pass/fail was **not** used as evidence — the
> tree is being mutated by the concurrent session (build artifacts and the G1
> source file changed mid-run). The findings below come from reading the
> committed/landed source, which is stable regardless of tree churn.

---

## §2. Why G2-as-written can never be green (logical contradiction)

`tests/pending/constitution_kernel_sequencer_single_admission.rs:282-311` asserts
**three** facts about the same run:

```
assert!( kernel_admitted );             // line 287-291  → kernel_admitted == true
assert!( !sequencer_admitted );         // line 292-296  → sequencer_admitted == false
assert_eq!(kernel_admitted, sequencer_admitted);  // line 298-310 → true == false
```

These are mutually unsatisfiable. The two legs feed **different logical claims**:

- **Kernel leg** (`kernel_admits_on_worker_proceed`, lines 108-129) drives a bare
  `Proceed` through the **3-arg** `MemoryKernel::step_forward` shim
  (`src/memory_kernel.rs:210`). That shim supplies
  `PredicateClaimSet::default()` (empty) → `decide_admission(zero, [], false)` →
  `Pass` (`src/predicate_admission.rs:124`) → head advances → `kernel_admitted = true`.
- **Sequencer leg** (`sequencer_admits_failing_predicate_worktx`, lines 243-271)
  submits a `WorkTx` whose acceptance predicate is **`false`** → rejected →
  `sequencer_admitted = false`.

No `src/` change can make "admit an empty/no-predicate `Proceed`" equal "reject a
`false` predicate" while *also* asserting the first is `true` and the second is
`false`. **The gate compares apples to oranges; it is a broken test, not a
falsifiable invariant.** Per `AGENTS.md §7`, a test that cannot pass is
documentation, not a gate.

---

## §3. Why G3 can never be green under route-A (structural / dead code)

G3 (`tests/pending/constitution_predicate_zero_root_is_not_oracle.rs:247-280`)
requires the sequencer to **refuse** a zero-root self-asserted-`true` `WorkTx`.

Route-A shipped the spec's *recommended* derivation in **both** legs:

```
src/state/sequencer.rs:1243   let os_qualified = q.predicate_registry_root_t != Hash::ZERO;
src/memory_kernel.rs:247      let os_qualified = self.predicate_registry_root_t != Hash::ZERO;
```

In the sequencer this line lives **inside** the `q.predicate_registry_root_t ==
Hash::ZERO` branch (opened at `sequencer.rs:1231`). Inside that branch the root
**is** zero, so `os_qualified` is **always `false`**. Therefore the refuse path

```
src/predicate_admission.rs:97-103
  if is_zero_root && os_qualified { return Fail(ZeroRootRefusedForOsQualifiedRun) }
```

is **unreachable from the sequencer** — the source itself admits this:

```
src/state/sequencer.rs:1343-1345
  // Not reachable from the sequencer's zero-root branch, where os_qualified is
  // always false; mapped for totality.
```

So a zero-root `WorkTx` always falls through to the legacy boolean scan
(`predicate_admission.rs:105-127`) and is **admitted**. G3 stays RED.

### The spec is internally inconsistent here

The implementation spec simultaneously claims, of the **same** derivation:

- §6 / §12 Open-Question-1: *"derive `os_qualified = (registry_root != ZERO)`
  (recommended, no schema surface)"*, and
- §9: *"to turn G3 green, an OS-qualified sequencer run must pass
  `os_qualified=true`."*

Both cannot hold: under that derivation a **zero-root** run is **never**
`os_qualified`, so it is **never** refused. **G3 green requires `os_qualified` to
be a signal that is independent of `registry_root`.**

### Why you cannot just force `os_qualified = true` for zero-root runs

`registry_root == Hash::ZERO` is shared by **both** a malicious G3 run **and**
every legitimate genesis test run. `QState::genesis()` = `QState::default()`
(`src/state/q_state.rs:958`) → `predicate_registry_root_t == Hash::ZERO`. The
following suites all admit a zero-root `WorkTx` carrying `true` predicates and
would **break** if zero-root admission were refused wholesale:

- `tests/tb_8_minimal_payout.rs::apply_task_funded_with_accepted_worktx` (line 185)
  and its **8+** call sites (lines 242, 315, 358, 407, 454, 504, 908, 1002),
- `tests/constitution_fc1_runtime_loop.rs`, `tests/tb_1_acceptance.rs`,
  `tests/tb_2_runtime_boundary.rs`, `tests/tb_3_rsp1_formal_surface.rs`,
  `tests/tb_6_*`, `tests/tb_7r_parent_tx_conformance.rs`,
  `tests/constitution_n1_*`, `tests/constitution_n2_event_resolve.rs`, … (15+).

`registry_root` therefore **cannot** discriminate. A new run-level signal is
required — this is the "new explicit run-level field" branch of the spec's own
Open Question 1.

---

## §4. The `M07_GREEN` correction (the "flip together" expectation)

`APPROVE_M07_SINGLE_ADMISSION_PREDICATE_GATE_2026-06-07.md §1` defines:

```
M07_GREEN := G1 green AND G2 green AND G3 green AND <no regression> AND <under §8>
```

Given §2 and §3, **this predicate is unsatisfiable by route-A** and must be
amended. The three gates do **not** flip together:

| Gate | Fix class | Status / path |
|------|-----------|---------------|
| **G1** kernel-predicate gate | route-A src (done) | **GREEN**, promoted to `tests/constitution_kernel_predicate_gate.rs`. |
| **G2** single-admission | **test redesign** (test-only; src already supports it) | Self-contradictory as written → retire the assertion. Structural witness `tests/constitution_single_admission_contract.rs` already added. **Recommended:** add the behavioral gate in §5. |
| **G3** zero-root-not-oracle | **NEW Class-4 §8** (new QState field) | Cannot land under `APPROVE-M07-A4-…`; needs the ruling in §6. |

### Proposed `M07_GREEN` v2

```
M07_GREEN_v2 :=
      G1 kernel-predicate gate green                                  (DONE)
  AND single-admission invariant proven by:
        constitution_single_admission_contract  (structural, DONE)
        AND constitution_single_admission_behavioral (behavioral, §5 — recommended)
  AND <no regression: cargo test --workspace --no-fail-fast,
        run_constitution_gates.sh, constitution_matrix_drift all exit 0>
  AND landed under APPROVE-M07-A4-SINGLE-ADMISSION-PREDICATE-GATE

# G3 zero-root-not-oracle is REMOVED from M07_GREEN and re-scoped to its own
# §8 (§6 below). G4/G5 remain standing pending (separate §8), unchanged.
```

Rationale: G3 is not a "single-admission" property at all — it is an
*oracle-vs-verdict-trust* property of an OS-qualified run. Conflating it into the
single-admission token was the original scope error. Keeping it inside
`M07_GREEN` makes M07 permanently unclosable.

---

## §5. G2 ruling — retire the contradiction, add a correct behavioral gate

**Decision:** the self-contradictory pending file
`tests/pending/constitution_kernel_sequencer_single_admission.rs` is **retired**
(it stays in `tests/pending/` documented as a permanently-red broken assertion,
or is deleted). The single-admission invariant is proven by **two** gates:

1. **Structural** (already added): `tests/constitution_single_admission_contract.rs`
   — one `decide_admission` home, both legs call it, kernel decides before
   advancing head, no inline duplication.
2. **Behavioral (recommended, NEW):** prove the runtime invariant the structural
   grep cannot — that for the **same** logical claim both authorities reach the
   **same** verdict. This is the gate G2 was *meant* to be. It is test-only and
   is within the existing `APPROVE-M07-A4-…` token ("single-admission invariant
   gate"); the src already supports it via
   `MemoryKernel::step_forward_with_claims` (`src/memory_kernel.rs:218`).

   Sketch (both legs fed the **same** claim; assert agreement on **both** verdicts):

   ```rust
   // FAILING claim → BOTH reject.
   let failing = PredicateClaimSet {
       acceptance: vec![PredicateClaim { id: PredicateId("acc1".into()),
                                         value: false, proof_cid: None }],
       settlement: vec![],
   };
   // kernel: drive Proceed WITH the failing claim via step_forward_with_claims;
   // decide_admission(zero,[acc=false],false) -> Fail -> handle_rejection ->
   // head NOT advanced. (Drop the old `matches!(step, Proceed)` precondition;
   // the step is now a non-advancing rejection.)
   let kernel_admitted = kernel_admits_with_claims(failing.clone()); // == false
   let seq_admitted   = sequencer_admits_failing_predicate_worktx(); // == false
   assert!(!kernel_admitted);
   assert!(!seq_admitted);
   assert_eq!(kernel_admitted, seq_admitted);   // false == false  ✔

   // (Optional, stronger) PASSING claim → BOTH admit: assert_eq!(true, true).
   ```

   This is a real falsifiable invariant: if the kernel ever stops consulting the
   shared contract, `kernel_admitted` flips to `true` and the gate goes RED.

> Why keep both: the structural gate is a source grep (weaker — it can pass while
> the wired behavior is wrong); the behavioral gate is a runtime witness
> (tape-first house style). Together they cover "the contract exists" **and**
> "the contract is actually enforced at runtime."

---

## §6. G3 ruling — `os_qualified` is a run-level field (NEW Class-4 §8)

**Decision requested (Class 4 — requires a new user §8 token + signed tag):**
introduce an explicit run-level qualification signal, independent of
`registry_root`.

### Recommended design — `QState::os_qualified_t: bool`

- Add `pub os_qualified_t: bool` to `QState` (`src/state/q_state.rs`).
  `QState::default()`/`genesis()` ⇒ **`false`** (preserves every legacy/genesis
  zero-root suite in §3 unchanged).
- Both legs compute `os_qualified` from **this field**, not from the root:
  - `src/state/sequencer.rs:1243` → `let os_qualified = q.os_qualified_t;`
  - `src/memory_kernel.rs:247` → derive from the kernel's run qualification.
- Result:
  - **Legacy run** (`os_qualified_t == false`) + zero root + `true` predicates →
    admitted (tb_8 et al. stay green). ✔
  - **OS-qualified run** (`os_qualified_t == true`) + zero root → `decide_admission`
    returns `Fail(ZeroRootRefusedForOsQualifiedRun)` → **refused** (G3 green).
    The previously **dead** path at `predicate_admission.rs:97-103` and the
    sequencer mapping at `sequencer.rs:1341-1345` become **live**. ✔
  - An OS-qualified run must therefore bind a **non-zero** registry root → the
    sequencer oracle re-executes against CAS proofs (`sequencer.rs:1258-1292`). ✔
- **How `os_qualified_t` becomes `true`:** at boot, via a **system-only** tx
  (the existing `PredicateBindingActivate` boot path is the natural carrier —
  to be confirmed at implementation). Because it lives in `QState`, it is folded
  into `state_root_t` and is **reconstructable from tape** by replay — satisfying
  the constitutional "scoped, reconstructable" requirement. This is the decisive
  advantage over the alternatives below.

### Trust-root / ratification cost

- `src/state/q_state.rs` is pinned in `genesis_payload.toml`. Adding a field
  changes its hash → the pin **must be rehashed in the same commit** (else boot
  panics `TRUST_ROOT_TAMPERED`), and a **user-signed git tag** must be applied
  (`scripts/check_tr_ratification_chain.sh`, RATIFICATION_2026-04-27 §3).
- `QState` is a Class-4 restricted surface (`AGENTS.md §6`). This is **a separate
  ratification** from `APPROVE-M07-A4-…`, which explicitly scoped itself to
  "no schema surface" (spec §12 Q1) and "q_state.rs … NOT edited" (spec §8.5).
  **One-word approvals do not constitute Class-4 sign-off** (`AGENTS.md §5`).

### Alternatives considered (and why rejected)

- **Sequencer/kernel construction flag** (no wire schema): lighter, but **not
  reconstructable from tape** — an auditor replaying L4/CAS cannot tell the run
  was OS-qualified. Violates the reconstructability requirement. **Rejected.**
- **Field on `TaskOpenTx`/`WorkTx`** (`typed_tx.rs`, pinned): wrong granularity
  (OS-qualification is a property of the *run*, not a *tx*) and a heavier wire
  change. **Rejected.**
- **Descope G3 entirely** (accept zero-root verdict-trust as permanent OS
  behavior): drops the oracle-not-verdict-trust invariant the OS exists to
  enforce. Conflicts with the user's strict-constitution / no-凑活 stance.
  **Not recommended** — but it is the honest fallback if the architect declines
  the new field; in that case G3 must be deleted from `tests/pending/` and from
  every "flip together" expectation, and the spec annotated accordingly (no
  retroactive rewrite — add a note).

---

## §7. What this packet does NOT do

- It does **not** edit any `src/`, test, manifest, matrix, runner, or
  `OBLIGATIONS.md` file. It is a decision record only. (The concurrent session
  owns those edits for G1/G2-structural; collisions avoided by scope.)
- It does **not** implement the G3 field — that is gated on the new §8 below.
- It does **not** close M07. M07 closes under `M07_GREEN_v2` (§4); the G3 leg
  closes separately under the §6 ratification.
- It does **not** touch G4 (budget ceiling) / G5 (FC3 meta-loop) — they remain
  standing pending under their own separate §8, unchanged.

---

## §8. Architect ratification (to be filled at user verbatim)

Two independent decisions are requested. Each needs an explicit token.

**(A) `M07_GREEN` v2 + G2 behavioral gate** (test-only; within existing token
scope, recorded here for the matrix):
- [ ] Approve `M07_GREEN_v2` (§4): G3 removed from M07_GREEN; G2 proven by
      structural + behavioral gates.
- [ ] Approve adding `tests/constitution_single_admission_behavioral.rs` (§5)
      and retiring the self-contradictory pending G2.

`> Architect token / verbatim: ____________________`

**(B) G3 run-level `os_qualified` field — NEW Class-4 §8** (`src/state/q_state.rs`
schema + `genesis_payload.toml` rehash + signed tag):
- [ ] Approve the `QState::os_qualified_t` design (§6), OR
- [ ] Direct the descope fallback (§6, last bullet).

`> Architect token / verbatim (Class-4, e.g. APPROVE-M07-G3-OS-QUALIFIED-RUN-FIELD): ____________________`
`> Signed-tag applied (scripts/check_tr_ratification_chain.sh): ____________________`
