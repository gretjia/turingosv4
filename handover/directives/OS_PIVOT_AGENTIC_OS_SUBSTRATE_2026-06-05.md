# Architect Directive: TuringOS Agentic OS Substrate Pivot

Date: 2026-06-05
Status: active execution directive, Class 0
Parent plan: `handover/directives/2026-06-05_AGENTIC_OS_PIVOT_EXECUTION_PLAN_DRAFT.md`
FC anchors: FC1 runtime loop, FC2 boot/replay, FC3 governance trail

Document role: child artifact for atom A00. This is not the master execution
plan. The canonical master plan is
`handover/directives/2026-06-05_AGENTIC_OS_PIVOT_EXECUTION_PLAN_DRAFT.md`.

## Ruling

TuringOS pivots toward a real Agentic OS substrate before benchmark
solve-rate claims.

Priority order:

```text
priority_1 = Agentic OS substrate
priority_2 = tape-canonical economy service
priority_3 = workload adapters
priority_4 = benchmark solve-rate
forbidden = monolithic snapshot merge
```

## Snapshot Freeze

PR #280 remains audit-only quarry.

```text
title = AUDIT ONLY: TuringOS-TC operationalization snapshot - DO NOT MERGE
head = codex/tc-operationalization-audit-snapshot-20260605
base = main
production_merge = forbidden
```

PR #283 remains audit-only quarry.

```text
title = AUDIT ONLY: TuringOS P1 real-value market experiment ... DO NOT MERGE
head = claude/p1-realvalue-audit-snapshot-20260605
production_merge = forbidden
```

Every production branch must be cut from current `origin/main`. Useful code or
ideas from #280/#283 must be cherry-picked, reimplemented, or rewritten as
small atoms with exact allowed paths and fresh verification.

Required ancestry check:

```bash
for n in 280 283; do
  oid=$(gh pr view "$n" --json headRefOid --jq .headRefOid)
  git merge-base --is-ancestor "$oid" HEAD && exit 1 || true
done
```

Expected: exit 0.

## OS Layer Contract

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

GitTape/ChainTape is the only physical state substrate for `Q_t`. Economy,
wallet, price, search, scheduler boards, reports, dashboards, and `LATEST.md`
are derived views unless a future Class 4 ratified atom changes the
constitution itself.

## Immediate Atom Queue

```text
A00 Freeze and Pivot Package
A01 Path B ADR and Layer Contract
A02 Claim Integrity Docs and Generic Gate Plan
A03 Boot Trust Root Manifest Gate
A04 GitTape Physical Ledger and Single Writer
A05 Tape Event Envelope and Projection Trait
A06 ExternalCall Outbox, Crash Matrix, and Orphan Sweeper
A07 Agent Process Model and View Shielding
A08 PredicateReceipt and LeanJudge Axiom Gate
A09 Economy Service v0
A10 Projection Cache With GitOid Watermark
A11 Scheduler and Search Policies
A12 Universal Machine Witnesses
A13 Agentic OS v0 E2E CLI
A14 Workload Adapters, Market Research, and Benchmark Boundary
```

## Claim Boundary

Supported by this directive:

```text
The repository has an execution priority and merge discipline for the pivot.
#280/#283 are not production merge branches.
Market economy remains a first-class derived OS service.
```

Unsupported by this directive:

```text
No claim that OS v0 exists yet.
No claim that FC3 gaps are closed.
No claim that TuringOS solves any benchmark.
No claim that market price beats controls.
```
