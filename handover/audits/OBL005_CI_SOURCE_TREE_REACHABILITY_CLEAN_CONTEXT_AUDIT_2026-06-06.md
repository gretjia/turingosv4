# OBL-005 CI Source-Tree Reachability Clean-Context Audit

Date: 2026-06-06
Reviewer: clean-context Claude CLI (`--no-session-persistence`)
Risk class: Class 2
FC trace: FC2 replay/current-source receipt binding; FC3 audit feedback/derived handover
Verdict: `NO-VIOLATION`

## Scope

This witness reviewed the post-PR-310 repair that hardens OBL-005 final-closure
source-tree evidence from local object existence to `HEAD` reachability, and
rebinds the affected final-closure-eligible evidence rows to fresh current-main
runs rooted at source commit `e81c8a968c9b00723c2b4cd015368e3eb70dc58c`.

The witness was not given the implementation transcript. It was given the task
brief, current diff scope, evidence roots, verification commands, and the
required constitutional verdict domain.

## Checked

- `tests/constitution_true_suite_evidence_reconciliation.rs` now checks source
  commits with `git merge-base --is-ancestor <commit> HEAD` after confirming
  the object exists.
- All final-closure-bound source-tree commits reachable from `HEAD`, including
  the rebinding commit `e81c8a968c9b00723c2b4cd015368e3eb70dc58c`.
- `domain_receipt_final_closure_false` and `benchmark_capability_not_solved`
  blockers remain present for Market A/B, Mind2Web, and ToolBench.
- Diff scope does not touch A03 runtime, `src/`, sequencer admission, typed tx
  schema, canonical signing payload, TrustRoot authority, wallet, kernel, bus,
  or CAS schema.
- Old evidence roots are not rewritten; new timestamped evidence roots are
  added and registered.
- `handover/ai-direct/LATEST.md` and `OBLIGATIONS.md` remain derived views.

## Witness Finding

The witness returned `NO-VIOLATION`, with one high-severity operational
finding: the five newly referenced `obl005_ci_*` evidence roots were still
untracked at witness time, so committing only tracked files would reproduce the
same fresh-clone reconstruction failure.

Resolution: after the witness completed, the five evidence roots were
explicitly staged. `git ls-files` over those roots returned 162 tracked files.

## Raw Verdict JSON

```json
{
  "verdict": "NO-VIOLATION",
  "findings": [
    {
      "severity": "high-must-resolve-before-push",
      "title": "Five new evidence roots are untracked (git status ??) but fixtures now depend on them",
      "detail": "tests/fixtures/liveness/true_suite_evidence_reconciliation.toml and production_module_liveness.toml were repointed to obl005_ci_replay_cas_20260606T142221Z, obl005_ci_generate_20260606T142742Z, obl005_ci_market_ab_20260606T143144Z, obl005_ci_mind2web_20260606T143633Z, obl005_ci_toolbench_20260606T143903Z, but all five dirs are untracked in the working tree. constitution_true_suite_evidence_reconciliation.rs:1056 (read_full_system_report) and sibling liveness/lit-evidence tests read these on-disk JSONs. If the PR commits only the tracked diff, a fresh GitHub clone fails with file-not-found -- the identical local-green/CI-red class this PR repairs. MUST git-add all five evidence roots before push and confirm with a clean re-clone or git ls-files."
    }
  ],
  "checked": [
    "rebind commit e81c8a968c9b00723c2b4cd015368e3eb70dc58c == HEAD and reachable",
    "guard hardened object-existence -> merge-base --is-ancestor HEAD",
    "all 16 distinct source_tree.commit values across 21 bindings verified reachable from HEAD",
    "benchmark/domain failures preserved as capability-pending blockers",
    "diff scope limited to tests/fixtures/OBLIGATIONS.md/LATEST.md and evidence/audit records",
    "no historical evidence rewrite"
  ],
  "notes": "The constitutional design of the repair is correct. Operationally, the five obl005_ci_* evidence roots must be tracked before push; this was resolved by explicit staging after the witness."
}
```
