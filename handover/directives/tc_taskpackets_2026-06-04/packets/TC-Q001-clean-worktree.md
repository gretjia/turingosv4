# TC-Q001 Clean Worktree

Status: ready
Owner lane: substrate
Risk class: Class 0
FC nodes: FC2 boot provenance
Dependencies: TC-Q000
Active obligations: OBL-014(open) -> this atom

Allowed write paths:

- `handover/directives/tc_taskpackets_2026-06-04/packets/TC-Q001-clean-worktree.md`

Forbidden paths: source, tests, scripts, restricted surfaces.

Task:

Confirm TC implementation proceeds from the clean locked base, not from the
dirty evidence branch.

Worker steps:

1. Run `git merge-base HEAD origin/main`.
2. Confirm the clean base is or descends from
   `39233aa7c868f0e9b37a7a29eb426279f41cf032`.
3. Run `git status --short --branch`.
4. Confirm dirty files are expected TC branch edits, not imports from the dirty
   source branch.

Ship gate:

```bash
git merge-base HEAD origin/main
git status --short --branch
```

Expected: merge-base is compatible with locked base; branch is
`codex/turingos-tc-operationalization`.

Audit: Constitution Auditor.
Verdict: `NO-VIOLATION` or `SECOND-SOURCE-DRIFT <view>`.
