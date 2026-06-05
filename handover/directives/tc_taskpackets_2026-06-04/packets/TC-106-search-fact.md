# TC-106 Search Fact

Status: ready
Owner lane: gateway
Risk class: Class 2 shielding
FC nodes: FC1 external input, FC3 replay
Dependencies: TC-101
Active obligations: OBL-014(open) -> this atom

Allowed write paths:

- `src/runtime/tc_tape_canonical.rs`
- `tests/tc_tape_canonical_repairs.rs`

Forbidden paths: raw private search dump stores, network during replay.

Task:

Represent search/librarian activity as query hash, result hash, and tape anchor.

Test first:

`search_fact_records_query_hash_result_hash_and_anchor`.

Assertions:

- query body is hashed or redacted.
- result body is hashed or CID referenced.
- replay of the fact never calls network/search provider.

Ship gate:

```bash
cargo test --test tc_tape_canonical_repairs --no-fail-fast
```

Expected: command exits 0.

Audit: Security Auditor.
Verdict: `NO-VIOLATION` or `RECONSTRUCTION-FAILURE <artifact>`.
