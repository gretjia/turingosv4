# TC-021A Full Audit Packet Export

Status: ready
Owner lane: audit
Risk class: Class 2 ship-path evidence
FC nodes: FC2 replay, FC3 audit packet
Dependencies: all implementation packets
Active obligations: OBL-014(open) -> this atom

Allowed write paths:

- `scripts/export_tc_audit_packet.sh`
- `handover/reports/TC_FULL_AUDIT_PACKET_MANIFEST_2026-06-04.md`
- `tests/tc_audit_packet_export.rs`

Forbidden paths: historical evidence rewrite, metadata artifacts as reliability evidence.

Task:

Export a self-contained audit packet for outside auditors.

Required contents:

- source SHA
- worktree status
- constitution hash
- boot manifest hash
- Path B ref schema
- replay commands
- crash matrix results
- Minsky and Brainfuck witness traces
- G0 manifest and scheduler traces
- parity schema and tables
- clean-context audit verdicts
- obligation witness verdict
- durable dirty-tree preservation archive reference

Excluded from final reliability audit:

- `RUN_STATUS.json`
- `STAGE_A_POWER_GATE.json`
- `prereg.json`

Tests first:

- `audit_packet_manifest_lists_required_artifacts`
- `audit_packet_reliability_excludes_metadata_artifacts`

Ship gate:

```bash
cargo test --test tc_audit_packet_export --no-fail-fast
bash scripts/export_tc_audit_packet.sh --check
```

Expected: both commands exit 0.

Audit: Constitution Auditor.
Verdict: `NO-VIOLATION` or `RECONSTRUCTION-FAILURE <artifact>`.
