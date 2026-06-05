# TC-101 Gateway Tape-Canonical Fact

Status: ready
Owner lane: gateway
Risk class: Class 1 additive helper
FC nodes: FC1 tape, FC3 replay
Dependencies: TC-005b
Active obligations: OBL-014(open) -> this atom

Allowed write paths:

- `src/runtime/tc_tape_canonical.rs`
- `src/runtime/mod.rs`
- `tests/tc_tape_canonical_repairs.rs`

Forbidden paths: typed-tx schema, CAS schema, sequencer, signing payloads.

Task:

Create the generic fact and anchor shape used by TC-101..TC-110 without adding
new canonical transaction or CAS object schemas.

Test first:

`gateway_fact_requires_tape_anchor_and_reconstructs`.

Required data shape:

- `TapeAnchor { run_id, logical_t: Option<u64>, submit_id: Option<String>, head_ref, head_oid }`
- `TcTapeCanonicalFact { kind, anchor, payload_hash, public_summary }`

Rules:

- At least one of `logical_t` or `submit_id` must be present.
- `payload_hash` is SHA-256 hex over already-redacted payload bytes.
- `public_summary` must not contain raw prompt, raw provider response, raw
  verifier stderr, API key, bearer token, or private diagnostic body.

Ship gate:

```bash
cargo test --test tc_tape_canonical_repairs --no-fail-fast
```

Expected: command exits 0.

Audit: Data-integrity Auditor.
Verdict: `NO-VIOLATION` or `RECONSTRUCTION-FAILURE <artifact>`.
