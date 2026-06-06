# OBL-005 CI Source-Tree Reachability Repair

Date: 2026-06-06
Risk class: Class 2 accounting / witness liveness gate
FC trace: FC2 replay/current-source receipt binding; FC3 audit feedback / derived handover

## Trigger

After PR #310 merged, GitHub CI failed
`constitution_true_suite_evidence_reconciliation::final_closure_source_tree_commits_must_exist_in_git_history`
because final-closure evidence cited source-tree commit
`165f1878aeeecd5c427244843a77e4e2dff6cdc6`, which existed only in the local
object store and was not reachable from GitHub `main`.

## Repair

The gate now requires source-tree commits to be reachable from current `HEAD`
with `git merge-base --is-ancestor`, not merely present in the local object
database via `git cat-file -e`.

The reconciliation manifest was rebound to fresh evidence generated from
GitHub-reachable `e81c8a968c9b00723c2b4cd015368e3eb70dc58c`:

- `obl005_ci_replay_cas_20260606T142221Z`
- `obl005_ci_generate_20260606T142742Z`
- `obl005_ci_market_ab_20260606T143144Z`
- `obl005_ci_mind2web_20260606T143633Z`
- `obl005_ci_toolbench_20260606T143903Z`

Each new root was packaged with `scripts/package_true_suite_evidence.sh` and
restored with `scripts/restore_true_suite_chain_evidence.sh`.

## Boundaries

- No historical evidence root was edited or migrated.
- No `src/` runtime, sequencer, typed transaction schema, CAS schema, wallet,
  signing payload, or trust-root authority surface was edited.
- A03 runtime remains unimplemented and outside this repair.
- Market A/B, Mind2Web, and ToolBench capability/domain failures remain
  explicit `domain_receipt_final_closure_false` /
  `benchmark_capability_not_solved` annotations.

## Local Verification

Red reproduction:

```text
cargo test --test constitution_true_suite_evidence_reconciliation final_closure_source_tree_commits_must_exist_in_git_history -- --exact
FAILED: generate_artifact_chain_fresh bound 165f1878... not reachable from HEAD
```

Green after repair:

```text
cargo test --test constitution_true_suite_evidence_reconciliation final_closure_source_tree_commits_must_exist_in_git_history -- --exact
1 passed

cargo test --test constitution_true_suite_evidence_reconciliation --no-fail-fast
16 passed
```

Verdict: `OBL005-CI-REACHABILITY-REPAIR-READY`
