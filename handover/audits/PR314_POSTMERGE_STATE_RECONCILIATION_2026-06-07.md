# PR #314 Post-Merge State Reconciliation — Agentic-OS Qualification

Date: 2026-06-07

Workspace:
`/home/zephryj/projects/turingosv4-m07-prep`

Branch / base:
`claude/m07-pr314-followup-prep`, base `origin/main` `fc839ae7`
("Class 3: land A03 keep-src-boot trust-root gate (#314)").

Risk class of THIS document:
Class 0 (handover state correction). No `src/`, `constitution.md`,
`genesis_payload.toml`, `build.rs`, sequencer admission, `memory_kernel`, or
typed-tx schema is touched by this document. This is PRE-§8 reconciliation
prep only.

Obligation context:
`OBLIGATIONS.md` OBL-016 (`Status: blocked`). This is the Phase-0 document
2/3. Phase 3+ (kernel predicate gate / zero-root quarantine /
single-admission invariant / FC3 runtime engine) is Class-4 admission-topology
work and is BLOCKED awaiting the user's §8 token
`APPROVE-M07-A4-SINGLE-ADMISSION-PREDICATE-GATE`.

Purpose:
Reconcile the Agentic-OS pivot audit/plan (the A00–A14 master queue in
`handover/directives/2026-06-05_AGENTIC_OS_PIVOT_EXECUTION_PLAN_DRAFT.md`)
against the actual state of `main` AFTER #314 merged. Every conclusion below
was grep-verified against the worktree at base `fc839ae7`; each cites
`file:line`. Production defects are distinguished from test-scaffold gaps.

---

## Method

Read-only grep + `Read` over the worktree at HEAD `fc839ae7`. No file was
mutated to produce this reconciliation. Cited line ranges were opened directly
and the quoted control flow confirmed (not inferred from the plan text).

---

## Conclusion 1 — #314 closed A03 KEEP-SRC-BOOT (OBL-014) ONLY; it did NOT close M07

**Verdict: confirmed. #314 is an A03 boot-Trust-Root gate, orthogonal to M07
kernel/sequencer admission.**

- HEAD `fc839ae7` is titled "Class 3: land A03 keep-src-boot trust-root gate
  (#314)" (`git log --oneline`). It satisfies `OBLIGATIONS.md` OBL-014, whose
  scope is verbatim: "keep `src/boot.rs::verify_trust_root` as the single
  authoritative boot Trust Root verifier, use the existing A13 public
  `turingos boot --verify-manifest` CLI hook, and add a focused constitutional
  gate ... and no wrapper module or second authority is introduced."
- #314 added EXACTLY one test file, `tests/constitution_tc_boot_trust_root_manifest.rs`
  (8 passed / 0 failed). It did NOT add `src/runtime/boot_trust_root_manifest.rs`
  (intentionally absent; `test ! -e` passed per OBL-014 evidence), and it did
  NOT edit `src/boot.rs`, `src/main.rs`, `build.rs`, `genesis_payload.toml`,
  sequencer admission, typed-tx schema, or signing payloads (OBL-014 Risk-class
  note).
- The clean-context witness
  `handover/audits/A03_KEEP_SRC_BOOT_CLEAN_CONTEXT_AUDIT_2026-06-06.md`
  returned `NO-VIOLATION` and explicitly scoped its diff to
  "OBL/LATEST/matrix/directive/gate manifest plus
  tests/constitution_tc_boot_trust_root_manifest.rs; no restricted source diff."
- A03 lives at the FC2 boot Trust-Root node. M07 lives at the FC1 runtime
  admission loop (kernel `step_forward`) and the WorkTx dispatch path
  (sequencer `verify_work_predicates`). They are different flowchart nodes;
  #314 touches neither M07 surface. **No M07 obligation was discharged by
  #314.**

---

## Conclusion 2 — M07 remains a live BLOCKER: dual, predicate-blind kernel admission

**Verdict: confirmed PRODUCTION DEFECT (not a test-scaffold gap). The runtime
loop has two independent admission authorities, and the kernel one is
predicate-blind.**

- `src/memory_kernel.rs:171-188` (`step_forward_with_workspace`): the kernel
  routes on `match (parsed_header, env_result.is_success())` and, when
  `header.status == StateStatus::Proceed` AND `is_success()` is true
  (`:172`), it directly `self.tape.commit(... NodeKind::StateAccepted ...)`
  (`:174-186`) and `self.tape.set_verified_head(accepted.hash)` (`:188`),
  returning `KernelStep::Proceed`. There is no call to `verify_work_predicates`,
  no `WorkTx` is built, and no `PredicateRegistry` is consulted on this path.
  Acceptance is decided by an LLM self-reported header status plus a process
  success boolean — `verified_head` advances on the agent's own word.
- Predicate execution lives ONLY on the sequencer's WorkTx dispatch path:
  `fn verify_work_predicates(q, work, registry, predicate_cas)` at
  `src/state/sequencer.rs:1225`. The two admission surfaces (kernel-tape vs
  sequencer-WorkTx) are not unified — this is the "dual admission" / "single
  admission convergence" target named by M07.
- Why this is a production defect, not scaffolding: `step_forward` /
  `step_forward_with_workspace` are the public runtime entrypoints
  (`src/memory_kernel.rs:156, 162`) that real runs drive; the predicate-blind
  accept at `:188` is the live behavior, not a test fixture. The kill-condition
  gate for this defect is the Phase-1 pending file
  `tests/pending/constitution_kernel_sequencer_single_admission.rs`
  (expected-red until the §8 fix lands).

---

## Conclusion 3 — zero-root admission trusts recorded booleans; it is NOT oracle re-execution

**Verdict: confirmed. The zero-root branch is verdict-trusting, not a
re-derivation of predicate truth from the registry/oracle.**

- `src/state/sequencer.rs:1231`: `if q.predicate_registry_root_t == Hash::ZERO`
  the function iterates `work.predicate_results.acceptance` (`:1232-1236`) then
  `settlement` (`:1237-1241`) and returns `Err(...PredicateFailed)` only when a
  recorded `bwp.value` is `false`; otherwise it `return Ok(())` at `:1242`.
  In this branch the registry argument is never consulted and no proof is
  re-checked against CAS — admission trusts the boolean the submitter wrote
  into the bundle.
- Re-execution / oracle binding exists only on the NON-zero branch: at
  `:1245` `if registry.merkle_root_hash() != q.predicate_registry_root_t`
  returns `PredicateRegistryRootMismatch`, then `verify_predicate_key_set`
  (`:1249-1258`) enforces the required key set, and `verify_predicate_claim`
  (`:1266-1277`) re-checks each claim against the registry + CAS proof store
  (`PredicateContext { proof_store: predicate_cas }`, `:1260-1264`).
- Therefore "zero-root admission" is a trust-the-verdict shortcut, not an
  oracle. A genesis QState (`predicate_registry_root_t == Hash::ZERO`) routes
  entirely through the verdict-trusting branch. The kill-condition gate is the
  Phase-1 pending file
  `tests/pending/constitution_predicate_zero_root_is_not_oracle.rs`
  (expected-red until a non-zero registry root + real re-execution is bound).

---

## Conclusion 4 — M20 (FC3 runtime self-evolution) status corrected: "PARTIAL substrate live, runtime engine missing"

**Verdict: correcting the audit/plan claim of "zero closure". The FC3 typed
transition substrate IS live; the runtime self-evolution ENGINE is not.**

- Substrate LIVE (typed-tx schema + sequencer transition arms exist):
  `LogFeedbackArchiveTx`, `ArchitectProposalTx`, `VetoDecisionTx`,
  `ArchitectCommitTx`, `ReinitRequestTx`, `ReinitBootTx` are defined at
  `src/state/typed_tx.rs:1893, 1945, 1999, 2033, 2084, 2105` with system signing
  payloads at `:2391, 2489, 2515`. The sequencer carries deterministic Veto-AI
  verdict checks: `VetoVerdict` / `VetoDecisionTx` imports at
  `src/state/sequencer.rs:50`, the runtime Veto-AI accept-state domain +
  accepted-decision root at `:336-339`, and the verdict-mismatch /
  capsule-invalid / root-mismatch transition errors at `:649-653`
  (`VetoDecisionLogicalTMismatch`, `VetoDecisionProposalMissing`,
  `VetoDecisionCapsuleInvalid`, `VetoDecisionRootMismatch`,
  `VetoDecisionMismatch`).
- Engine MISSING (no runtime that DRIVES the loop): there is no live runtime
  pipeline of log → proposer → synthesis → canary → trust-root rewrite →
  reinit. The transition arms can ADMIT the FC3 transactions if something emits
  them, but nothing autonomously generates the proposal/synthesis/canary
  sequence or executes a trust-root rewrite + reinit boot.
- Correct M20 status string going forward:
  **"PARTIAL — FC3 typed-transition substrate live (LogFeedbackArchiveTx ..
  ReinitBootTx + deterministic Veto-AI verdict checks); runtime
  self-evolution engine (log→proposer→synthesis→canary→trust-root-rewrite→reinit)
  missing."** Do NOT carry the prior "zero closure" claim; it understates the
  live substrate and would mis-scope the FC3 atom. The kill-condition gate is
  the Phase-1 pending file
  `tests/pending/constitution_fc3_meta_loop_closure.rs` (expected-red until the
  runtime engine drives a full FC3 cycle end-to-end on tape).

Supporting budget finding (Art.V.2 enforcement gap, same FC1 admission family):
`BudgetSnapshot::default()` at `src/state/q_state.rs:153-159` sets
`cost_ceiling_microcoin = MicroCoin::zero()`, `wall_clock_remaining_ms = 0`,
`compute_cap_remaining = 0`, and no admission gate compares these against the
Art.V.2 10000-microcoin / 24h ceiling. These are lazy/unbound budget fields,
not an enforced cap. Kill-condition gate:
`tests/pending/constitution_budget_ceiling_enforced.rs` (expected-red).

---

## Conclusion 5 — NO Agentic-OS qualification claim is permitted at this state

**Verdict: qualification is BLOCKED. None of the three required admission
invariants is green.**

An Agentic-OS qualification claim requires, at minimum, ALL of:

1. **Kernel predicate gate green** — `src/memory_kernel.rs` advances
   `verified_head` only AFTER predicate verification, not from
   `env_result.success` + a self-reported `Proceed` header
   (current defect: `memory_kernel.rs:171-188`).
2. **Non-zero predicate registry root in real admission** — admission routes
   through the oracle re-execution branch (`sequencer.rs:1245-1277`), not the
   verdict-trusting zero-root branch (`sequencer.rs:1231-1242`).
3. **Single-admission invariant green** — exactly one admission authority
   decides `verified_head` advancement, unifying the kernel-tape and
   sequencer-WorkTx paths.

At base `fc839ae7`, (1) is a confirmed production defect, (2) is not bound
(genesis routes through zero-root), and (3) does not hold (dual admission).
Each of the three legs is touched by Class-4 admission topology and is
BLOCKED awaiting per-leg §8 ratification per AGENTS.md §5. Until those gates
are green on tape, this repository **must not** assert Agentic-OS
qualification, "single-admission", "predicate-gated kernel", or "FC3 closure"
in any PR title, report, dashboard, or README. The Phase-1 pending gates
encode each leg as an expected-red kill condition; they are run only by
`scripts/run_pending_agentic_os_kill_conditions.sh` and are deliberately
excluded from the default `cargo test --workspace`, the constitution-gate
manifest, and the matrix-drift gate so main CI stays GREEN while the gates
stand red.

---

## Cross-references

- Master plan: `handover/directives/2026-06-05_AGENTIC_OS_PIVOT_EXECUTION_PLAN_DRAFT.md`
- OS layer contract: `handover/directives/OS_PIVOT_AGENTIC_OS_SUBSTRATE_2026-06-05.md`
- A08 predicate-admission preflight precedent:
  `handover/directives/2026-06-05_A08_PREDICATE_RECEIPT_LEAN_JUDGE_PREFLIGHT.md`
- A03 landing + audit: `handover/directives/2026-06-05_A03_BOOT_TRUST_ROOT_MANIFEST_PREFLIGHT_AND_SECTION8_REQUEST.md`,
  `handover/audits/A03_KEEP_SRC_BOOT_CLEAN_CONTEXT_AUDIT_2026-06-06.md`
- Phase-0 sibling docs: `handover/directives/OS_QUALIFICATION_FREEZE_M07_2026-06-07.md`,
  `handover/directives/AGENTIC_OS_STATUS_AFTER_PR314_2026-06-07.md`
- Phase-1 pending gates + runner:
  `tests/pending/constitution_kernel_sequencer_single_admission.rs`,
  `tests/pending/constitution_predicate_zero_root_is_not_oracle.rs`,
  `tests/pending/constitution_budget_ceiling_enforced.rs`,
  `tests/pending/constitution_fc3_meta_loop_closure.rs`,
  `scripts/run_pending_agentic_os_kill_conditions.sh`,
  `handover/audits/PENDING_AGENTIC_OS_KILL_CONDITIONS_2026-06-07.md`
- Phase-2 §8 packet (pending user token):
  `handover/section8/APPROVE_M07_SINGLE_ADMISSION_PREDICATE_GATE_2026-06-07.md`
- Obligation: `OBLIGATIONS.md` OBL-016 (`Status: blocked`).
