# Mainline Baseline Snapshot - 2026-06-06

## Classification

- Risk class: Class 0 docs / derived handover sync.
- Trigger: after OBL-005 closure and PR #311 merge, the user agreed to continue
  with the recommended mainline baseline freeze / current truth map.
- Scope: record the current GitHub `main` baseline and sync the derived
  handover pointer. No source code, runtime, schema, ChainTape, CAS, evidence
  object, sequencer, typed transaction, trust-root authority, or A03 runtime
  implementation is changed.

## Baseline

- Worktree:
  `/home/zephryj/projects/turingosv4-mainline-baseline-snapshot`
- Branch: `codex/mainline-baseline-snapshot`
- Base: `origin/main`
- Source baseline commit:
  `4af83627ef013b65a4764b4b9c4fffb93ea0a8ae`
- Merged source: PR #311, `Class 2: repair OBL005 CI source reachability`
- PR #311 merge time: `2026-06-06T15:17:02Z`
- PR #311 merge commit:
  `4af83627ef013b65a4764b4b9c4fffb93ea0a8ae`
- Snapshot sync PR: PR #312, `Class 0: record mainline baseline snapshot`
- PR #312 merge time: `2026-06-06T23:27:10Z`
- PR #312 merge commit:
  `f599be295ceffd24c5e9fba03eccbdb10bc78d0e`

## Truth-Tier Reading

- Tier 1 axioms: `constitution.md` and canonical flowchart hashes are not
  changed by this snapshot.
- Tier 2 facts: ChainTape, CAS, deterministic replay, and OBL-005 evidence
  roots remain the authority. This snapshot does not rewrite historical
  evidence or create new evidence roots.
- Tier 3 derived pointers: `OBLIGATIONS.md` is already closed with an exact
  `OBL-ALL-CLOSED` status, and PR #312 updated `handover/ai-direct/LATEST.md`
  from the stale PR #310 pointer to the PR #311 GitHub-main baseline.

## Current State

- OBL ledger: `OBL-ALL-CLOSED`.
- OBL-005 scoped status token: `OBL005_FINAL_CLOSURE_VERIFIED`.
- Closure scope: OBL-005 is closed only for
  no-zombie/no-drift/no-unconstitutional-retained-substrate proof, preserving
  benchmark and domain failures as capability-pending facts.
- A03 runtime: not authorized and not implemented. Future A03 runtime work
  still requires one exact Section-8 phrase from the A03 preflight packet:
  `APPROVE-A03-SECTION8-KEEP-SRC-BOOT`,
  `APPROVE-A03-SECTION8-WRAPPER-MODULE`,
  `APPROVE-A03-SECTION8-DEFER-TO-TC002`, or
  `REJECT-A03-RUNTIME-FOR-NOW`.
- Open PR check: two audit-only `DO NOT MERGE` PRs existed when this baseline
  was prepared. They do not block this unique Class 0 snapshot path.

## Verification

```text
$ git status --short
<clean>

$ git rev-parse HEAD
4af83627ef013b65a4764b4b9c4fffb93ea0a8ae

$ git rev-parse origin/main
4af83627ef013b65a4764b4b9c4fffb93ea0a8ae

$ git merge-base --is-ancestor HEAD origin/main && echo head-is-origin-main
head-is-origin-main
```

```text
$ gh pr view 311 --repo gretjia/turingosv4 --json state,mergedAt,mergeCommit,url
state=MERGED
mergedAt=2026-06-06T15:17:02Z
mergeCommit=4af83627ef013b65a4764b4b9c4fffb93ea0a8ae
url=https://github.com/gretjia/turingosv4/pull/311
```

```text
$ gh pr checks 311 --repo gretjia/turingosv4
Constitution gate suite: pass
Feature freeze check: pass
validate PR has no sidecar contamination: pass
```

```text
$ cargo test --test constitution_true_suite_evidence_reconciliation \
  --test constitution_obl005_final_closure_witness \
  --test constitution_production_module_liveness \
  --test constitution_matrix_drift \
  --no-fail-fast

constitution_matrix_drift: 3 passed
constitution_obl005_final_closure_witness: 10 passed
constitution_production_module_liveness: 22 passed
constitution_true_suite_evidence_reconciliation: 16 passed
```

```text
$ bash scripts/run_constitution_gates.sh
[k-1-5] total=167 failed=0
```

## Next-Agent Guardrails

- Do not cite `e81c8a96` / PR #310 as the latest synchronized base after this
  snapshot lands; use `4af83627` / PR #311 unless GitHub `main` advances again.
- Do not start A03 runtime work from "continue", "go", "fix", or similar
  generic wording. Require one exact A03 Section-8 phrase.
- Do not convert benchmark/domain capability-pending rows into closure success
  without a separate capability/domain witness.
- Treat this file and `handover/ai-direct/LATEST.md` as derived views only;
  ChainTape/CAS/replay and constitution gates win on conflict.
