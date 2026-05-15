# REAL-10 Evidence Contamination Note

Date: 2026-05-15

Status: do not use this directory as the final REAL-8X evidence source.

Cause:

```text
While recording Harness command evidence, `turingos_dev record-command` was
mistakenly used with the full REAL-8X benchmark command. `record-command`
executes the command instead of attaching a prior command transcript, so it
started a second benchmark run against this already-completed output path.
The duplicate run was killed, but not before it rewrote partial arm summary and
report material in this directory.
```

Evidence discipline:

```text
This note preserves the failure instead of deleting or rewriting the evidence
directory. The final REAL-10 decision gate and audits must use a fresh
REAL-8X output directory created after this note.
```

Invalidated path:

```text
handover/evidence/real8x_market_ab_20260515T134453Z/
```

Required recovery:

```text
Run REAL-8X again with the same pinned inputs and a new output path.
Do not cite this contaminated directory as final benchmark evidence.
```
