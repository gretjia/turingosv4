# Agentic OS Status After PR #314 — Falsifiable To-Be-Proven State

Date: 2026-06-07
Status: report, Class 0 — derived view (ChainTape / CAS / gates win on conflict)
Obligation: OBL-016 (PR #314 后续 M07 收敛计划) — Phase 0 deliverable 3/3
Branch: `claude/m07-pr314-followup-prep` (base `fc839ae7` = PR #314)
FC anchors: FC1 runtime loop, FC2 boot/predicate admission, FC3 governance trail

Sibling Phase-0 docs:
- `handover/directives/OS_QUALIFICATION_FREEZE_M07_2026-06-07.md` (freeze ruling 1/3)
- `handover/directives/PR314_POSTMERGE_STATE_RECONCILIATION_2026-06-07.md` (state reconciliation 2/3)

> **Read first — this is a status, not a claim.** Every "HAVE" line below is a
> property the substrate *appears* to satisfy; every "MISSING" line is a gap.
> The single load-bearing claim of this document is the **negative** one: the
> repository is **NOT** "Agentic OS qualified" today, and the qualification
> conjunction (§5) is the falsifiable predicate that decides when it could be.
> The forbidden-claim list in
> `handover/directives/OS_QUALIFICATION_FREEZE_M07_2026-06-07.md` is in force.

---

## 1. Frame: what "Agentic OS status" means here

The Agentic OS pivot defines a ten-layer contract (L0–L9) in
`handover/directives/OS_PIVOT_AGENTIC_OS_SUBSTRATE_2026-06-05.md`:

```text
L0 Constitution / Human Sudo
L1 Boot Trust Root / Manifest
L2 GitTape / ChainTape World State
L3 External Call Outbox / Side-effect Gateway
L4 Agent Process Model / Agent View Shielding
L5 Predicate / Verifier Framework
L6 Economy Service: Coin / Wallet / Market / Price / Settlement
L7 Scheduler / Search / Allocation Policies
L8 Workload Adapters
L9 Evidence / Benchmark / Reports
```

Status is reported per layer as a tri-state: **HAVE** (property holds on a real
path), **PARTIAL** (substrate present, runtime/enforcement missing), **MISSING**.
No layer is claimed "qualified" — qualification is a *conjunction* over machine
gates (§5), not a per-layer subjective grade.

---

## 2. HAVE — substrate properties that hold today

These are the load-bearing things #314 leaves standing. Each is grep-anchored or
gate-anchored; none is asserted from a dashboard or a self-report.

| # | Property (layer) | Evidence anchor | Standing |
|---|------------------|-----------------|----------|
| H1 | **Tape-canonical world state** — GitTape/ChainTape is the only physical substrate for `Q_t`; economy/price/boards/`LATEST.md` are derived views | OS Layer Contract L2; `AGENTS.md §1` truth tiers (ChainTape/CAS over derived views) | HAVE |
| H2 | **Money conservation, integer-only** — money/economy paths use integer math (`MicroCoin`/`StakeMicroCoin`), no `f64` in the conservation path | `CLAUDE.md §4` forbidden-list; `src/economy/monetary_invariant.rs`; `src/economy/money.rs` (`MicroCoin`, `StakeMicroCoin`) | HAVE |
| H3 | **External oracle re-execution — BOUND-ROOT ONLY** — when `predicate_registry_root_t != Hash::ZERO`, the sequencer re-executes predicate claims against CAS proofs instead of trusting booleans | `src/state/sequencer.rs` bound branch (`registry.merkle_root_hash() != q.predicate_registry_root_t` mismatch check ~line 1245; per-claim `verify_predicate_claim`) | HAVE (bound-root only — see B-chain B2) |
| H4 | **Boot Trust Root manifest gate (A03)** — `src/boot.rs::verify_trust_root` is the single authoritative boot verifier; valid manifests pass, mismatch/missing/child-manifest/adversarial-bypass fail closed | `tests/constitution_tc_boot_trust_root_manifest.rs` (8 passed / 0 failed, #314); §8 token `APPROVE-A03-SECTION8-KEEP-SRC-BOOT` consumed; status token `A03_KEEP_SRC_BOOT_LANDED` | HAVE |
| H5 | **Clean-context audit witness role** — platform-agnostic single clean-context audit; verdict domain `{NO-VIOLATION, VIOLATION-FOUND, RECONSTRUCTION-FAILURE, SECOND-SOURCE-DRIFT}`; A03 audited NO-VIOLATION | `AGENTS.md §9, §14`; `handover/audits/A03_KEEP_SRC_BOOT_CLEAN_CONTEXT_AUDIT_2026-06-06.md` (`NO-VIOLATION`) | HAVE |
| H6 | **FC3 governance SUBSTRATE (typed-tx + sequencer arms)** — `LogFeedbackArchiveTx`, `ArchitectProposalTx`, `VetoDecisionTx`, `ArchitectCommitTx`, `ReinitRequestTx`, `ReinitBootTx` + capsule types exist with sequencer transition arms and deterministic Veto-AI verdict checks | typed-tx FC3 variants; `src/state/sequencer.rs` FC3 admission arms; `tests/...fc3_governance_reinit_current_kernel.rs` | PARTIAL — substrate live, runtime engine missing (see M-chain) |

**What H1–H6 jointly support (and only this):** a tape-first substrate with
integer money conservation, a bound-root oracle re-execution path, a fail-closed
boot trust root, an independent audit role, and the *typed-transaction skeleton*
of an FC3 governance trail.

**What they do NOT support:** that the kernel is predicate-gated, that admission
has a single authority, that zero-root runs are oracle-backed, that budgets are
enforced, or that the FC3 loop closes. Those are the gaps below.

---

## 3. The first BLOCKER — M07 dual-admission (kernel is predicate-blind)

The top blocker is **M07: two admission authorities that can disagree.**

- **Kernel path (FC1):** `src/memory_kernel.rs:171-188` — `step_forward` routes on
  `(parsed_header, env_result.is_success())` with `header.status ==
  StateStatus::Proceed`, commits `NodeKind::StateAccepted` and calls
  `self.tape.set_verified_head(accepted.hash)`. It **never** calls
  `verify_work_predicates`, builds **no** `WorkTx`, touches **no**
  `PredicateRegistry`. Kernel admission is **predicate-blind**.
- **Sequencer path (FC2):** predicate execution lives only in
  `src/state/sequencer.rs` `fn verify_work_predicates(...)` on the `WorkTx`
  dispatch path.

So the **same logical claim** can be ADMITTED by the kernel (bare `Proceed` +
`success`) while the sequencer would REJECT the equivalent `WorkTx` whose
acceptance predicate is `false`. Two authorities, two verdicts — there is no
single admission contract. This is demonstrated red by pending gates **G1**
(kernel-predicate) and **G2** (single-admission) in
`handover/audits/PENDING_AGENTIC_OS_KILL_CONDITIONS_2026-06-07.md`.

---

## 4. The chain of blockers (ordered; each gates the next)

M07 is the head of a dependency chain. Closing it does not finish the OS; it
unblocks the next gap. The chain, in required order:

```text
B1  M07 dual-admission           (FC1×FC2 / L5×L4) — kernel predicate-blind vs sequencer
        src/memory_kernel.rs:171-188  vs  src/state/sequencer.rs verify_work_predicates
        → pending gates G1, G2

B2  zero-root-not-oracle         (FC2 / L5) — zero registry root trusts self-reported
        booleans instead of re-executing the oracle
        src/state/sequencer.rs:1231  (root == Hash::ZERO ⇒ trust bwp.value)
        → pending gate G3

B3  single-admission invariant   (FC1×FC2 / L5) — ONE shared predicate-admission
        contract for kernel + sequencer (the fix that turns G1/G2/G3 green together)
        → closes M07_GREEN (§5)

— after M07_GREEN, still red / still pending —

B4  FC3 runtime engine           (FC3 / L0) — substrate live (H6) but engine missing:
        log → proposer → synthesis → canary → trust-root rewrite → re-init.
        Live role-path proposal payload is ToolProposalPayload::default()
        (proposal_id == None); accepted-proposal terminal status is
        "sandbox:canary_only" (dead-end, never re-init / trust-root recompute).
        src/runtime/real5_roles.rs (shell payloads ~464-469, 1077-1091)
        → STANDING pending gate G5 (Class-4 §8: touches RootBox / boot trust-root)

B5  memory model                 (FC1 / L4) — tape-canonical SkillCapsule / durable
        agent memory beyond per-run belief state

B6  taint · budget · capability  (FC2 / L4×L7) — three enforcement gaps:
        - budget: BudgetSnapshot fields (cost_ceiling_microcoin, wall_clock_remaining_ms,
          compute_cap_remaining) default to zero and NO admission gate compares them
          against any Art. V.2 ceiling — src/state/q_state.rs:156-158
          → STANDING pending gate G4 (awaits §8 ruling: are Art. V.2 numbers HARD
            ceilings or illustrative; ceiling values must come from genesis/manifest)
        - taint: agent-view shielding enforcement (L4) beyond current scoping
        - capability: per-agent capability admission (L4×L7)
```

**Why the order is load-bearing:** B2 (zero-root) and B3 (single-admission) are
the same Class-4 admission-topology surface as B1 — you cannot make the kernel
predicate-gated (B1/B3) without deciding the zero-root oracle question (B2),
because a shared admission contract must define what "admit" means at a zero
root. B4 (FC3 engine) is gated behind a stable single-admission contract: a
governance loop that rewrites the trust root is only meaningful once admission
has one authority to rewrite. B5/B6 are enforcement layers that ride on top of
the unified admission path.

---

## 5. Minimum qualification conjunction — falsifiable, machine-reproducible

No "Agentic OS qualified" claim is permitted until the following conjunction
flips from **red to green on a real run**. This is the falsifiable predicate;
it is tracked by the pending kill-condition gates, not by prose.

```text
M07_GREEN :=
      G1 kernel-predicate gate green          # kernel head advance backed by a
                                              #   tape-recorded predicate-admission PASS receipt
  AND G2 single-admission gate green          # kernel_admitted == sequencer_admitted
                                              #   for the same logical claim
  AND G3 zero-root-not-oracle gate green      # OS-qualified admission requires a non-zero
                                              #   bound registry root → real oracle re-execution
  AND no regression in the existing constitution gate suite
        (`cargo test --workspace --no-fail-fast` exit 0,
         `bash scripts/run_constitution_gates.sh` exit 0,
         `cargo test --test constitution_matrix_drift` exit 0)
  AND the fix landed under the user's §8 token(s)

# STANDING — NOT part of M07_GREEN; remain red after M07 closes, await separate §8:
#   G4 budget-ceiling   (BUDGET_CEILING_STANDING_PENDING)  — §8 ruling on Art. V.2 ceilings
#   G5 FC3-meta-loop    (FC3_META_LOOP_STANDING_PENDING)   — Class-4 §8 on FC3 irreversible commit
```

`M07_GREEN` is the *necessary* gate for any single-admission / predicate-gated-
kernel claim. It is **not sufficient** for a full "Agentic OS qualified" claim:
that additionally requires B4 (FC3 engine) and B5/B6 (memory, taint·budget·
capability) to land under their own §8 rulings, plus a clean-context audit
(`AGENTS.md §9`) returning a non-violation verdict on the landed admission
topology. The standing gates G4/G5 stay red and pending after M07 closes — their
red state is the live tracker that those layers are not yet enforced.

**Falsification rule.** The conjunction is reproducible by running
`scripts/run_pending_agentic_os_kill_conditions.sh`. Today it prints all five
gates red (`M07_EXPECTED_RED`, `SINGLE_ADMISSION_EXPECTED_RED`,
`ZERO_ROOT_EXPECTED_RED`, `BUDGET_CEILING_STANDING_PENDING`,
`FC3_META_LOOP_STANDING_PENDING`) and exits 0 (= "all pending red as expected").
If any of G1/G2/G3 unexpectedly turns green before the §8 fix lands, the runner
exits 1 (= premature wire-up or a vacuous assertion) — that is the machine alarm
against a false qualification claim. When G1∧G2∧G3 turn green *under* the §8
fix, `M07_GREEN` is met and the gates are promoted to real constitution gates
(the documented triple-coupling move: rename to `tests/constitution_*.rs` + add
manifest entry + add matrix/allowlist reference).

---

## 6. Current state — one-line falsifiable summary

```text
to_be_proven: TuringOS is NOT YET Agentic-OS-qualified.
  HAVE     : tape-canonical world state (H1), integer money conservation (H2),
             bound-root oracle re-execution (H3), boot trust-root gate A03 (H4),
             clean-context audit role (H5), FC3 governance SUBSTRATE (H6, partial).
  BLOCKER  : M07 dual-admission — kernel advances verified_head with no predicate
             receipt (memory_kernel.rs:171-188) while sequencer gates on predicates;
             zero-root trusts booleans (sequencer.rs:1231).
  CHAIN    : B1 M07 → B2 zero-root → B3 single-admission (=M07_GREEN)
             → B4 FC3 engine → B5 memory → B6 taint·budget·capability.
  QUALIFIED-WHEN: G1 ∧ G2 ∧ G3 green on a real run, no gate-suite regression,
             under §8 token(s); G4/G5 standing; full claim also needs B4–B6 + audit.
  STATUS   : FROZEN per OS_QUALIFICATION_FREEZE_M07_2026-06-07.md until M07_GREEN.
             Class-4 fixes BLOCKED awaiting §8 token
             APPROVE-M07-A4-SINGLE-ADMISSION-PREDICATE-GATE (+ zero-root /
             single-admission / FC3 legs).
```

---

## 7. References

- Pending kill-condition gates + per-gate detail + standing tokens:
  `handover/audits/PENDING_AGENTIC_OS_KILL_CONDITIONS_2026-06-07.md`
- Runner (dev-only, expected-red, NOT a constitution gate):
  `scripts/run_pending_agentic_os_kill_conditions.sh`
- Freeze ruling + M07_GREEN definition + forbidden-claim list:
  `handover/directives/OS_QUALIFICATION_FREEZE_M07_2026-06-07.md`
- Post-merge state reconciliation:
  `handover/directives/PR314_POSTMERGE_STATE_RECONCILIATION_2026-06-07.md`
- OS Layer Contract (L0–L9) + priority order:
  `handover/directives/OS_PIVOT_AGENTIC_OS_SUBSTRATE_2026-06-05.md`
- Pivot master plan / atom queue (A00–A14):
  `handover/directives/2026-06-05_AGENTIC_OS_PIVOT_EXECUTION_PLAN_DRAFT.md`
- §8 decision packet (token requested, not yet consumed):
  `handover/section8/APPROVE_M07_SINGLE_ADMISSION_PREDICATE_GATE_2026-06-07.md`
- Obligation ledger: `OBLIGATIONS.md` OBL-016.
- Claim discipline: `AGENTS.md §9, §14, §14b`; `skills/no-proven-checklist.md`.
</content>
</invoke>
