# Architect Directive: OS Qualification Freeze Until M07 Green

Date: 2026-06-07
Status: active execution directive, Class 0
Obligation: OBL-016 (PR #314 后续 M07 收敛计划) — Phase 0 deliverable 1/3
Branch: `claude/m07-pr314-followup-prep` (base `fc839ae7` = PR #314)
Parent plan: `handover/directives/2026-06-05_AGENTIC_OS_PIVOT_EXECUTION_PLAN_DRAFT.md`
Layer directive: `handover/directives/OS_PIVOT_AGENTIC_OS_SUBSTRATE_2026-06-05.md`
FC anchors: FC1 runtime loop, FC2 boot/predicate admission, FC3 governance trail

Document role: Class-0 freeze directive layered on top of the Agentic OS pivot.
This is NOT a new master plan and does NOT supersede the A00–A14 atom queue. It
narrows what may merge until the M07 single-admission predicate gate is green.

## Ruling

From now until M07 is green, the repository is under an **OS Qualification
Freeze**. The kernel can currently advance `verified_head` without any
predicate-admission receipt (`src/memory_kernel.rs:171-188`), and zero-root
admission trusts self-reported booleans instead of re-executing the oracle
(`src/state/sequencer.rs:1231`). While that bypass stands, any "Agentic OS"
qualification claim is unstable, so forward expansion is frozen.

`M07 is green` is defined narrowly: the kill-condition conjunction below
flips from red to green on a real run.

```text
M07_GREEN :=
  (G1 kernel-predicate gate green
     AND G2 single-admission gate green
     AND G3 zero-root-not-oracle gate green)
  AND no regression in the existing constitution gate suite
  AND the fix landed under the user's §8 token(s)

# G4 budget-ceiling and G5 FC3-meta-loop are STANDING (await separate §8
# rulings) and are NOT part of the M07_GREEN conjunction. They remain red and
# pending after M07 closes; see the pending audit for their standing tokens.
```

The five kill-condition gates and their standing tokens are defined in
`handover/audits/PENDING_AGENTIC_OS_KILL_CONDITIONS_2026-06-07.md`
(`tests/pending/constitution_kernel_predicate_gate.rs`,
`constitution_kernel_sequencer_single_admission.rs`,
`constitution_predicate_zero_root_is_not_oracle.rs`,
`constitution_budget_ceiling_enforced.rs`,
`constitution_fc3_meta_loop_closure.rs`), runnable via
`scripts/run_pending_agentic_os_kill_conditions.sh`.

## What is frozen

Until `M07_GREEN`, the following PR classes are FROZEN (do not draft, do not
merge):

```text
frozen = benchmark / solve-rate PRs          # priority_4 work, see pivot directive
frozen = market-causal / price-beats-control claims
frozen = FC3 runtime-engine activation        # proposer / synthesis / canary / re-init engine
frozen = interop / external-call / A2A expansion
frozen = large-evidence / new-CAS-surface PRs
frozen = monolithic snapshot merge            # already forbidden by pivot directive
```

## What is allowed

Only these four lanes may proceed during the freeze:

```text
allowed_A = M07 predicate-admission repair      # single-admission gate; touches src/state/sequencer.rs + src/memory_kernel.rs under §8
allowed_B = zero-root oracle fix                # zero registry root must re-execute, not trust booleans
allowed_C = OS qualification gate / audit suite # promote pending gates → constitution gates; clean-context audit
allowed_D = Class-0 docs                        # directives, reports, handover, OBLIGATIONS reconciliation
```

Lanes A and B touch Class-4 admission-topology surfaces
(`src/state/sequencer.rs`, `src/memory_kernel.rs`) and are BLOCKED until the
user supplies the §8 token(s). The pending §8 packet requests
`APPROVE-M07-A4-SINGLE-ADMISSION-PREDICATE-GATE`
(`handover/section8/APPROVE_M07_SINGLE_ADMISSION_PREDICATE_GATE_2026-06-07.md`),
with separate legs for the zero-root quarantine, the single-admission
invariant, and the FC3 irreversible-commit path.

## Claim boundary (no strong claim until green)

Until the `M07_GREEN` conjunction above is satisfied on a real run, NONE of the
following claims may appear in any PR title, PR body, report, dashboard,
README, or `LATEST.md`:

```text
forbidden_claim = "Agentic OS qualified"
forbidden_claim = "M07 closed" / "M07 green" (before the conjunction is met)
forbidden_claim = "predicate-gated kernel"
forbidden_claim = "single admission authority"
forbidden_claim = "OS v0 exists"
```

Supported by this directive:

```text
The repository has a freeze discipline and a narrow allowed-work list until M07 is green.
The kernel-vs-sequencer admission bypass is demonstrated red by five pending kill-condition gates.
The fixes are Class-4 and BLOCKED pending the user's §8 token(s).
```

Unsupported by this directive:

```text
No claim that the kernel is predicate-gated yet.
No claim that M07 is closed.
No claim that the Agentic OS is qualified.
No claim that FC3 governance closes (G5 standing).
No claim that budget ceilings are enforced (G4 standing).
```

## References

- Pending kill-condition audit + per-gate detail + standing tokens:
  `handover/audits/PENDING_AGENTIC_OS_KILL_CONDITIONS_2026-06-07.md`
- §8 decision packet (token requested, not yet consumed):
  `handover/section8/APPROVE_M07_SINGLE_ADMISSION_PREDICATE_GATE_2026-06-07.md`
- Pivot master plan / atom queue:
  `handover/directives/2026-06-05_AGENTIC_OS_PIVOT_EXECUTION_PLAN_DRAFT.md`
- OS Layer Contract (L0–L9):
  `handover/directives/OS_PIVOT_AGENTIC_OS_SUBSTRATE_2026-06-05.md`
- A08 predicate-admission preflight (hard-blocker precedent):
  `handover/directives/2026-06-05_A08_PREDICATE_RECEIPT_LEAN_JUDGE_PREFLIGHT.md`
- Obligation ledger: `OBLIGATIONS.md` OBL-016.
