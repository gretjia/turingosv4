# TC-Q000 Dirty Quarantine

Status: ready
Owner lane: substrate
Risk class: Class 0
FC nodes: FC3 logs/archive, evidence provenance
Dependencies: none
Active obligations: OBL-014(open) -> this atom

Allowed write paths:

- `handover/reports/TC_Q_DIRTY_TREE_PRESERVATION_2026-06-04.yaml`
- `handover/directives/tc_taskpackets_2026-06-04/packets/TC-Q000-dirty-quarantine.md`

Forbidden paths: source, tests, scripts, restricted surfaces.

Context files:

- `handover/reports/TC_Q_DIRTY_TREE_PRESERVATION_2026-06-04.yaml`
- original dirty tree path `/Users/zephryj/work/turingosv4`

Task:

Verify the dirty tree was preserved and classified. The dirty tree remains an
evidence quarry only.

Worker steps:

1. Confirm manifest contains `snapshot_id`, `source_branch`, `source_head`,
   `origin_main`, `tracked_patch_sha256`, `ahead_patch_sha256`,
   `untracked_tgz_sha256`, `status_txt_sha256`, and `classifications`.
2. Confirm `origin_main` equals `39233aa7c868f0e9b37a7a29eb426279f41cf032`.
3. Confirm `rules/enforcement.log` is classified `DROP_JUNK`.
4. If the snapshot path is under `/tmp`, add a blocker note for TC-021A that
   final packet must re-hash or copy the archive into durable evidence.

Ship gate:

```bash
grep -n 'origin_main: "39233aa7c868f0e9b37a7a29eb426279f41cf032"' handover/reports/TC_Q_DIRTY_TREE_PRESERVATION_2026-06-04.yaml
grep -n 'rules/enforcement.log' handover/reports/TC_Q_DIRTY_TREE_PRESERVATION_2026-06-04.yaml
```

Expected: both commands print matching lines.

Audit: Dirty-tree steward.
Verdict: `DIRTY-PRESERVED` or `DIRTY-PRESERVATION-FAILURE <artifact>`.
