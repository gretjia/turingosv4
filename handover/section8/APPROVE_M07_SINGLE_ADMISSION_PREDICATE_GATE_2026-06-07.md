# §8 Decision Packet — M07 Single-Admission Predicate Gate

**Status**: AWAITING ARCHITECT RATIFICATION. No implementation happens until the
architect supplies the token(s) below. This document is **Class-0 documentation
only** — it describes a Class-4 admission-topology change, requests per-atom §8
ratification, and authorizes nothing by itself.

**Date**: 2026-06-07
**Obligation**: OBL-016 (PR #314 后续 M07 收敛计划) — Phase 2 deliverable (§8 decision packet)
**Branch**: `claude/m07-pr314-followup-prep` (base `fc839ae7` = PR #314)
**Risk class**: **Class 4** — touches the §6 restricted-surface admission
topology (`src/state/sequencer.rs`, `src/memory_kernel.rs`). Requires **per-atom
§8** architect ratification before any implementation or ship
(`AGENTS.md §5`, §6). Short replies (`go`, `ok`, `continue`, `can`, `完成`) do
**not** constitute Class-4 sign-off.

**Proposed §8 token** (the architect replies with this exact phrase to ratify
the target route):

```text
APPROVE-M07-A4-SINGLE-ADMISSION-PREDICATE-GATE
```

with three separable legs, each requiring its own explicit token (no batching —
`feedback_no_batch_class4_signoff`):

```text
APPROVE-M07-LEG-SINGLE-ADMISSION-INVARIANT   # B3: one shared predicate-admission contract
APPROVE-M07-LEG-ZERO-ROOT-QUARANTINE         # B2: zero registry root must not back an OS-qualified run
APPROVE-M07-LEG-FC3-IRREVERSIBLE-COMMIT       # B4 standing: FC3 re-init / trust-root recompute (separate, NOT in M07_GREEN)
```

**Authority chain**:
- Parent plan / atom queue: `handover/directives/2026-06-05_AGENTIC_OS_PIVOT_EXECUTION_PLAN_DRAFT.md` (A00–A14).
- OS Layer Contract (L0–L9) + priority order: `handover/directives/OS_PIVOT_AGENTIC_OS_SUBSTRATE_2026-06-05.md`.
- Freeze ruling + `M07_GREEN` definition + forbidden-claim list: `handover/directives/OS_QUALIFICATION_FREEZE_M07_2026-06-07.md`.
- Status (HAVE/MISSING + B1–B6 chain): `handover/reports/AGENTIC_OS_STATUS_AFTER_PR314_2026-06-07.md`.
- Pending kill-condition gates + per-gate detail + standing tokens: `handover/audits/PENDING_AGENTIC_OS_KILL_CONDITIONS_2026-06-07.md`.
- Predicate-admission hard-blocker precedent: `handover/directives/2026-06-05_A08_PREDICATE_RECEIPT_LEAN_JUDGE_PREFLIGHT.md`.
- Obligation ledger: `OBLIGATIONS.md` OBL-016.
- Constitution binding: `AGENTS.md §5–§6, §9, §14`; `CLAUDE.md §4`; `constitution.md` FC1/FC2/FC3.

---

## §1. Decision statement

**Mainline must converge to a SINGLE predicate-admission authority.**

No live kernel path may advance `verified_head` (or commit `NodeKind::StateAccepted`,
or any accepted-state write) **solely** from `env_result.success`, parser/header
success, or `StateStatus::Proceed`. An accepted-state advance must be backed by a
**tape-recorded predicate-admission PASS receipt** produced by the same admission
contract the sequencer enforces.

Concretely, the decision asserts all three:

1. **No predicate-blind head advance.** The kernel
   (`src/memory_kernel.rs:171-188`) currently commits `NodeKind::StateAccepted`
   and calls `self.tape.set_verified_head(accepted.hash)` after matching
   `(parsed_header, env_result.is_success())` with `header.status ==
   StateStatus::Proceed`. It never calls `verify_work_predicates`, builds no
   `WorkTx`, touches no `PredicateRegistry`. This predicate-blind advance must
   end.

2. **One admission verdict for one logical claim.** For the same logical claim,
   `kernel_admitted == sequencer_admitted` must hold. Today the kernel can ADMIT
   a bare `Proceed` while the sequencer would REJECT the equivalent `WorkTx`
   whose acceptance predicate is `false` — two authorities, two verdicts.

3. **Zero registry root must not back an OS-qualified run.** Under
   `predicate_registry_root_t == Hash::ZERO` (`src/state/sequencer.rs:1231`) the
   sequencer trusts self-reported `BoolWithProof.value` booleans instead of
   re-executing the oracle. An OS-qualified admission must run against a non-zero
   bound registry root with real oracle re-execution (the bound branch,
   `sequencer.rs` ~line 1245, `verify_predicate_claim` against CAS proofs).

This decision is the head of the `M07` blocker chain
(`handover/reports/AGENTIC_OS_STATUS_AFTER_PR314_2026-06-07.md §4`):

```text
B1 M07 dual-admission        (FC1 × FC2 / L5 × L4)  → pending gates G1, G2
B2 zero-root-not-oracle      (FC2 / L5)             → pending gate G3
B3 single-admission invariant(FC1 × FC2 / L5)       → closes M07_GREEN
— after M07_GREEN, still standing / still pending —
B4 FC3 runtime engine        (FC3 / L0)             → standing gate G5 (separate §8 leg)
B5 memory model              (FC1 / L4)
B6 taint · budget · capability (FC2 / L4 × L7)      → standing gate G4 (separate §8 ruling)
```

`M07_GREEN` is the falsifiable closure predicate (verbatim from the freeze
directive and status report):

```text
M07_GREEN :=
      G1 kernel-predicate gate green
  AND G2 single-admission gate green
  AND G3 zero-root-not-oracle gate green
  AND no regression in the existing constitution gate suite
        (cargo test --workspace --no-fail-fast exit 0,
         bash scripts/run_constitution_gates.sh exit 0,
         cargo test --test constitution_matrix_drift exit 0)
  AND the fix landed under the user's §8 token(s)

# STANDING — NOT part of M07_GREEN; remain red after M07 closes, await separate §8:
#   G4 budget-ceiling  (BUDGET_CEILING_STANDING_PENDING)
#   G5 FC3-meta-loop   (FC3_META_LOOP_STANDING_PENDING)
```

---

## §2. Allowed engineering actions (only under the §8 token)

The following are the **only** engineering moves authorized once the architect
supplies the matching token. Each touches Class-4 admission topology and is
BLOCKED until then.

- **A-ALLOW-1 — extract the shared admission contract.** Lift the predicate-
  admission logic that currently lives in the sequencer private fn
  `verify_work_predicates` (`src/state/sequencer.rs:1225`) into a **non-private
  module** (e.g. `src/state/predicate_admission.rs` or equivalent) so it is a
  single callable contract. The sequencer keeps calling it; the kernel calls the
  **same** contract. The extracted contract is the one source of "what admit
  means".
- **A-ALLOW-2 — bind both authorities to the one contract.** EITHER (a) the
  kernel calls the shared predicate-admission contract before any
  accepted-state advance, OR (b) demote the kernel to **proposal-only** and make
  the sequencer the **sole accepted-state writer** (the kernel emits a proposal;
  the sequencer's admission path is the only thing that may advance
  `verified_head` / commit `StateAccepted`). Either route must satisfy
  `kernel_admitted == sequencer_admitted` for the same logical claim (gate G2).
- **A-ALLOW-3 — require a non-zero predicate registry root for OS-qualified
  runs.** OS-qualified admission must route through the bound branch
  (`registry.merkle_root_hash() == q.predicate_registry_root_t`, non-zero) with
  real oracle re-execution. The zero-root branch is quarantined to legacy /
  non-OS-qualified smoke only (see A-ALLOW-4).
- **A-ALLOW-4 — quarantine zero-root boolean-trusting paths as legacy smoke
  only.** The `predicate_registry_root_t == Hash::ZERO` path
  (`sequencer.rs:1231`) that trusts self-reported `BoolWithProof.value` booleans
  may not back an OS-qualification claim. It is retained, explicitly labeled,
  and confined to legacy / smoke contexts that make no OS-qualified claim.
- **A-ALLOW-5 — emit a tape-recorded predicate-admission PASS receipt.** The
  accepted-state advance must be accompanied by a tape-visible admission receipt
  (so gate G1's "head advance must be backed by a tape-recorded predicate-
  admission PASS receipt" invariant is satisfiable on a real run, not by prose).

**Ceiling/value sourcing (binding on all of the above):** any concrete value
introduced (e.g. a required registry-root predicate, an admission receipt shape)
must come from genesis / manifest / typed-tx schema, never a hardcoded behavior
parameter (`CLAUDE.md §4`). Money/compute paths stay integer-only.

---

## §3. Forbidden (even under the token)

- **No copy-pasted verifier into the kernel.** The kernel must call the **shared
  extracted contract** (A-ALLOW-1). Duplicating `verify_work_predicates` logic
  into `memory_kernel.rs` creates a second authority that can drift — exactly the
  M07 defect. A mandatory anti-duplication gate enforces this (see §4 route A).
- **No convenience `pub` without changing the authority model.** Merely making
  `verify_work_predicates` or `dispatch_transition` `pub`/`pub(crate)` so the
  kernel can reach it, while leaving the kernel free to *not* call it, does not
  unify the authority. The change must make the single contract the **only** way
  to advance accepted state.
- **`env_result.success` is not a predicate pass.** Environment/tool success,
  parser success, and `StateStatus::Proceed` are inputs to a proposal, not an
  admission verdict. No accepted-state advance may treat any of them as a
  predicate PASS.
- **Zero-root must not support OS qualification.** No OS-qualified run may admit
  under `predicate_registry_root_t == Hash::ZERO`. Zero-root stays quarantined
  (A-ALLOW-4); it may not be re-labeled "oracle-backed".
- **No new global-latest pointer as canonical** (master-plan blocker B11), no
  dashboard/board-as-truth, no `build.rs`/`genesis_payload.toml` edit outside the
  dedicated trust-root atom (blocker B7), no audit before runnable evidence
  (blocker B14). `Cargo.toml` is pinned in the Trust Root on this worktree —
  editing it trips `src/boot.rs::verify_trust_root` (`TRUST_ROOT_TAMPERED`,
  Class-4) and is forbidden here.

---

## §4. The two implementation routes

Both routes converge on the SAME invariant (`kernel_admitted ==
sequencer_admitted`, head advance backed by a tape-recorded predicate-admission
PASS receipt). They differ in transition strategy.

### Route A — short-term bridge (transitional)

The kernel calls the shared extracted predicate-admission contract (A-ALLOW-1 +
A-ALLOW-2(a)) before any accepted-state advance, while the kernel retains its
current step shape. This is the smaller diff and the faster path to
`M07_GREEN`, but it leaves two call sites (kernel + sequencer) both invoking the
contract.

**Mandatory anti-duplication gate (binding if Route A is chosen):** a
constitution gate must prove there is exactly ONE predicate-admission contract
and that the kernel reaches accepted-state ONLY through it — e.g. a
source-structural grep asserting `verify_work_predicates`-equivalent logic
exists in exactly one module, plus a behavioral gate asserting the kernel cannot
advance `verified_head` without the shared contract returning PASS. Without this
gate, Route A risks silently regressing into two drifting authorities (the
original M07 defect).

### Route B — target single-admission (recommended target)

Demote the kernel to **proposal-only**; the sequencer's admission path becomes
the **sole accepted-state writer** (A-ALLOW-2(b)). The kernel emits a proposal
(`WorkTx`-shaped or proposal-shaped); only the sequencer's
`verify_work_predicates` → admit path may advance `verified_head` / commit
`StateAccepted`. This structurally removes the second authority rather than
disciplining it, so the anti-duplication property holds by construction.

### Recommendation

**Approve Route B as the target single-admission shape, with Route A permitted
as a transitional bridge — but only if Route A ships with the mandatory
anti-duplication gate above.** Route B is the constitutionally honest end state
(one authority by construction; matches the master-plan B3 "single-admission
invariant"). Route A is acceptable as an interim step toward `M07_GREEN`
provided it cannot decay into dual authority. If the architect prefers to skip
the bridge, ratify Route B directly.

This recommendation takes an explicit position (per
`feedback_architect_deviation_stance`): Route B is the target; Route A is a
gated transitional option, not a fence-sit between equals.

---

## §5. Risk classification & FC trace

**Risk class: Class 4.** The change touches admission topology on §6
restricted surfaces:

```text
src/state/sequencer.rs   — verify_work_predicates extraction; zero-root quarantine; bound-root requirement
src/memory_kernel.rs     — kernel head-advance gated on the shared contract OR demoted to proposal-only
```

Class-4 triggers present (`AGENTS.md §5–§6`):
- changes a sequencer admission rule (what "admit" means; zero-root vs bound-root),
- changes which authority may advance accepted state / `verified_head`,
- (FC3 leg, standing) touches RootBox / boot trust-root recompute on re-init.

Per `AGENTS.md §5`, Class-4 requires explicit per-atom §8 ratification before
implementation or ship. Class-4 cannot hide inside a Class-3 umbrella
(`feedback_class4_cannot_hide_in_class3`).

**FC trace:**
- **FC1** runtime loop (`Q_t → rtool → input → Agent δ → output → predicates →
  wtool → Q_{t+1}`): the kernel's `output → predicates → wtool` edge currently
  skips the predicate node; this change restores it.
- **FC2** boot / predicate admission: the single predicate/verifier authority
  (L5) and the zero-root vs bound-root admission decision.
- **FC3** governance trail (standing leg only): the FC3 irreversible-commit /
  re-init path is a SEPARATE token (`APPROVE-M07-LEG-FC3-IRREVERSIBLE-COMMIT`)
  and is NOT part of `M07_GREEN`.

**STEP_B protocol** (`feedback_step_b_protocol`): the sequencer + kernel changes
are restricted-file changes and must be developed on a parallel branch with the
gate suite GREEN before commit. If the change alters the canonical signing
payload or typed-tx schema, that is a separate Class-4 surface requiring its own
ratification (it is NOT assumed here).

---

## §6. Evidence the gaps are real (pre-§8, already landed)

These are demonstrated **red** today by the Phase-1 pending kill-condition gates
(`handover/audits/PENDING_AGENTIC_OS_KILL_CONDITIONS_2026-06-07.md`), runnable
via `scripts/run_pending_agentic_os_kill_conditions.sh` (dev-only, expected-red,
NOT a constitution gate, does not block CI):

| Gate | Demonstrates | Source line | Standing token | In M07_GREEN? |
|------|--------------|-------------|----------------|---------------|
| G1 kernel-predicate (`tests/pending/constitution_kernel_predicate_gate.rs`) | kernel advances `verified_head` predicate-blind, no admission receipt | `src/memory_kernel.rs:171-188` | `M07_EXPECTED_RED` | YES |
| G2 single-admission (`tests/pending/constitution_kernel_sequencer_single_admission.rs`) | kernel ADMITS while sequencer REJECTS same claim | `memory_kernel.rs:171-188` vs `sequencer.rs:1225` | `SINGLE_ADMISSION_EXPECTED_RED` | YES |
| G3 zero-root-not-oracle (`tests/pending/constitution_predicate_zero_root_is_not_oracle.rs`) | zero registry root trusts booleans, no oracle re-exec | `src/state/sequencer.rs:1231` | `ZERO_ROOT_EXPECTED_RED` | YES |
| G4 budget ceiling (`tests/pending/constitution_budget_ceiling_enforced.rs`) | over-budget run admits; no Art. V.2 ceiling gate | `src/state/q_state.rs:153-160`; `constitution.md:796-797` | `BUDGET_CEILING_STANDING_PENDING` | NO (standing; separate §8 ruling) |
| G5 FC3 meta-loop (`tests/pending/constitution_fc3_meta_loop_closure.rs`) | proposer inert; loop dead-ends at `sandbox:canary_only`, never re-inits | `src/runtime/real5_roles.rs:464-469,1077-1091` | `FC3_META_LOOP_STANDING_PENDING` | NO (standing; FC3 §8 leg) |

The runner today prints all five red and exits 0 (`ALL-PENDING-RED-AS-EXPECTED`).
If G1/G2/G3 turn green BEFORE the §8 fix lands, the runner exits 1 — the machine
alarm against a premature or vacuous qualification claim. Promotion (post-§8) is
the triple-coupling move: rename to `tests/constitution_*.rs` + add manifest
entry + add matrix/allowlist reference, atomically.

---

## §7. What this packet does NOT authorize

- It does NOT authorize any `src/` edit. Lanes A and B of the freeze directive
  (`OS_QUALIFICATION_FREEZE_M07_2026-06-07.md`) stay BLOCKED until the token.
- It does NOT close `M07`. `M07_GREEN` is met only when G1∧G2∧G3 are green on a
  real run with no gate-suite regression, under the token.
- It does NOT lift the forbidden-claim list. Until `M07_GREEN`, none of
  "Agentic OS qualified", "M07 closed/green", "predicate-gated kernel", "single
  admission authority", "OS v0 exists" may appear in any PR / report / dashboard
  / `LATEST.md` (freeze directive §"Claim boundary").
- It does NOT authorize the FC3 runtime engine (B4 / gate G5) or the budget
  ceiling (B6 / gate G4) — those are STANDING, each behind its own §8 leg/ruling
  and NOT part of `M07_GREEN`.

---

## §8. Architect ratification (to be filled at user verbatim)

**Status: AWAITING ARCHITECT RATIFICATION.** Reply with the exact token to
ratify the corresponding leg. No implementation begins until a token is supplied;
each leg is its own atomic Class-4 §8 cycle (no batching).

```text
Target route ratification (choose Route B target; Route A permitted only with anti-duplication gate):
  APPROVE-M07-A4-SINGLE-ADMISSION-PREDICATE-GATE

Separable legs (each its own token):
  APPROVE-M07-LEG-SINGLE-ADMISSION-INVARIANT     # B3 single-admission contract (kernel + sequencer)
  APPROVE-M07-LEG-ZERO-ROOT-QUARANTINE           # B2 non-zero bound root required for OS-qualified admission
  APPROVE-M07-LEG-FC3-IRREVERSIBLE-COMMIT        # B4 standing — FC3 re-init / trust-root recompute (NOT in M07_GREEN)

Reject / defer options:
  REJECT-M07-RUNTIME-FOR-NOW                      # keep admission topology frozen; continue Class-0 docs only
```

**Architect §8 sign-off (FILLED IN AT USER VERBATIM):**

- Verbatim quote: `<pending user verbatim §8 quote>`
- Token(s) consumed: `<pending>`
- Ratified route: `<Route A bridge + anti-duplication gate | Route B target | rejected>`
- Date: `<pending>`
- Branch at ratification: `claude/m07-pr314-followup-prep`
- Parent commit: `fc839ae7` (PR #314)
- Sign-off doc (created at user verbatim §8): `handover/section8/APPROVE_M07_SINGLE_ADMISSION_PREDICATE_GATE_§8_SIGN_OFF_2026-06-XX.md`

---

`FC-trace: FC1 output→predicates→wtool edge restored (kernel no longer skips the predicate node) + FC2 single predicate/verifier authority (L5) with bound-root oracle re-execution + zero-root quarantine + FC3 irreversible-commit as a separate standing §8 leg. Class-4 admission-topology change; per-atom §8 required; no implementation until token supplied.`

**End of M07 Single-Admission Predicate Gate §8 decision packet (PENDING ARCHITECT RATIFICATION; documentation only).**
