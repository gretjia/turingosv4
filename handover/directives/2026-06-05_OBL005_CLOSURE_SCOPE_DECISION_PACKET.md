# OBL-005 Closure Scope Decision Packet

Date: 2026-06-05
Risk class: Class 0 (decision packet / derived handover only)
Touched FC nodes: FC1 full-system receipts, FC2 replay/boot receipts, FC3 meta-role receipts, Market Coin/YES/NO receipts

## Current Evidence State

Authoritative current-state reads on `origin/main` `0d704f4a`:

- `tests/fixtures/liveness/true_suite_evidence_reconciliation.toml` has 21 bound rows.
- `final_closure_claimed = false`.
- `fresh_final_closure_witness_missing = 21`.
- `domain_receipt_final_closure_false = 14`.
- `benchmark_capability_not_solved = 10`.
- `source_receipt_final_closure_false = 0`.
- `source_tree_fingerprint_missing = 0`.
- `domain_receipt_final_closure_missing = 0`.
- `market_no_or_short_side_missing = 0`.
- `tests/fixtures/liveness/production_module_liveness.toml` has 15 retained groups, all `historical_real_world_candidate`, with no `legacy_quarantined` or `smoke_only` group.

This means current-source full-system receipts now exist for the retained
surface inventory, and the market rows have two-sided YES/NO evidence. It does
not mean OBL-005 can close: the manifest is intentionally fail-closed and still
records domain/capability blockers.

## Scope Fork

OBL-005 source text asks for a coverage harness proving:

- all three constitution flowcharts participate,
- no zombie modules remain,
- no flowchart node is missing code/test binding.

The current reconciliation fixture also tracks broad benchmark/domain rows
whose domain manifests remain non-closing. Those blockers are honest and must
not be hidden. The scope question is whether those benchmark/domain wins are
part of OBL-005 final closure, or whether they are separate capability work
once no-zombie/no-drift/current-source liveness is proven.

### Option A: Benchmark-Win Scope

OBL-005 remains open until all 14 `domain_receipt_final_closure_false` rows and
10 `benchmark_capability_not_solved` rows close.

Consequence: closure depends on model capability and broad benchmark suite
success, not only on TuringOS substrate liveness. Current single-sample GPQA
and Math successes remain insufficient after PR #275 because a domain closure
needs either a `domain_closure_witness` or `suite_sample_count > 1`, and any
failed benchmark sample must remain visible.

### Option B: No-Zombie / No-Drift Scope

OBL-005 final closure is scoped to proving no retained production/script/module
substrate is zombie, missing from FC1/FC2/FC3 production paths, or
unconstitutional. Benchmark/domain rows remain honest capability-pending rows
and do not become final-closure blockers for the no-zombie obligation.

Consequence: a later Class 2 final-closure witness may close OBL-005 only if it
proves the retained substrate inventory from ChainTape/CAS/current-source
receipts, verifies no evidence rewrite, and leaves non-closing benchmark/domain
rows explicitly annotated as capability-pending. It must not flip benchmark
domain manifests or erase blockers.

Recommendation: Option B. It matches the user obligation text and the 2026-06-04
reopen reason, which was retained diagnostic/zombie accounting, not a demand
that the current model solve every benchmark family.

### Option C: Market/DAG Reward Settlement Scope

OBL-005 also requires multi-node priced-DAG reward settlement closure.

Consequence: this is not a G0 witness. Current kernel settlement is single
rewardable WorkTx per task escrow at the claim/finalize layer: multiple
WorkTx/node stakes can exist, but reward claim and FinalizeReward are still
coupled to a single claim sweeping task escrow. Closing this requires a
separate Class 4 M2/M3 settlement redesign or multi-task-node model.

## Required Ratification Before Closure Witness

Before any future PR changes `final_closure_claimed` or authors a fresh
current-tree final-closure witness, the architect/user must ratify the closure
scope explicitly. One-word messages such as `ok`, `go`, or `continue` are not
sufficient.

Recommended ratification phrase:

`APPROVED-OBL005-NO-ZOMBIE-SCOPE`

This phrase means:

- close OBL-005 only against no-zombie/no-drift/no-unconstitutional-retained-substrate proof,
- keep broad benchmark/domain failures as capability-pending facts,
- do not rewrite historical evidence,
- do not touch Class 4 settlement/kernel surfaces without a separate Section 8 packet.

## Forbidden Moves

- Do not turn `final_closure_claimed` true while blocker-bearing bindings remain unresolved under the chosen scope.
- Do not edit old ChainTape/CAS evidence to satisfy new rules.
- Do not relabel single-sample benchmark success as domain closure without a suite/domain witness.
- Do not treat current Market A/B G0 evidence as multi-node priced-DAG reward settlement closure.
- Do not edit `src/state/sequencer.rs`, `src/state/typed_tx.rs`, or signing/admission surfaces for this Class 0 packet.
