# TC-020A Preregistration and Parity Schema

Status: ready
Owner lane: audit
Risk class: Class 1 manifest, Class 2 if runner integration changes
FC nodes: FC3 audit/statistics
Dependencies: TC-019A
Active obligations: OBL-014(open) -> this atom

Allowed write paths:

- `handover/directives/tc_prereg_2026-06-04/PARITY_SCHEMA.yaml`
- `tests/tc_prereg_parity.rs`

Forbidden paths: benchmark result rewriting, strong claim reports.

Task:

Define preregistered finite-budget Claim C parity schema.

Required metrics:

- proposal LLM calls
- route LLM calls
- challenge/bear LLM calls
- prompt tokens
- completion tokens
- total model tokens
- verifier calls
- scheduler ticks
- enumerator ticks
- accepted commits
- rejected commits
- wall-clock time

Tests first:

- `parity_schema_requires_all_compute_axes`
- `claim_c_report_is_descriptive_when_parity_fails`

Rule:

If token, verifier, scheduler, enumerator, or wall-clock parity fails, report
must be descriptive only and contain no cause-effect headline.

Ship gate:

```bash
cargo test --test tc_prereg_parity --no-fail-fast
```

Expected: command exits 0.

Audit: Statistics Auditor.
Verdict: `NO-VIOLATION` or `VIOLATION-FOUND parity-claim <file>:<line>`.
