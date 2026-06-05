# TC-104 LLM Call Fact

Status: ready
Owner lane: gateway
Risk class: Class 2 security-sensitive telemetry
FC nodes: FC1 external input, FC3 replay
Dependencies: TC-101
Active obligations: OBL-014(open) -> this atom

Allowed write paths:

- `src/runtime/external_call.rs`
- `src/runtime/tc_tape_canonical.rs`
- `tests/tc_tape_canonical_repairs.rs`
- `tests/tc_external_call_records.rs`

Forbidden paths: raw prompt stores, API key exposure, direct provider secrets.

Task:

Represent LLM calls as redacted request/result hashes linked to tape.

Test first:

`llm_call_fact_has_redacted_request_cid_and_no_raw_prompt`.

Assertions:

- request body is represented by `redacted_request_cid` and hash.
- result body is represented by hash/CID and usage metadata.
- raw prompt and raw provider response are not serialized in public fact.
- credential header, bearer-token, and API-key strings are rejected.

Ship gate:

```bash
cargo test --test tc_tape_canonical_repairs --test tc_external_call_records --no-fail-fast
(git diff --name-only origin/main...HEAD; git ls-files -o --exclude-standard) | sort -u | grep -E '^(src/runtime|src/drivers|tests)/' | xargs grep -nE 'api[_-]?ke[y]|Authori[z]ation|Bear[e]r'
```

Expected: cargo tests exit 0; grep has no output.

Audit: Security Auditor.
Verdict: `NO-VIOLATION` or `VIOLATION-FOUND secret-leak <file>:<line>`.
