# TC-007c Production LLM Wrapper

Status: ready
Owner lane: gateway
Risk class: Class 2
FC nodes: FC1 external input, FC3 replay
Dependencies: TC-007b
Active obligations: OBL-014(open) -> this atom

Allowed write paths:

- `src/drivers/llm_http.rs`
- `src/runtime/external_call.rs`
- `tests/tc_external_call_records.rs`

Forbidden paths: direct provider secrets, network in tests, kernel/bus.

Task:

Wrap LLM HTTP calls so durable intent is written before send and terminal after
response. Tests must use mock transport only.

Test first:

`recorded_llm_client_writes_intent_before_mock_send_and_terminal_after`.

Assertions:

- send cannot occur before durable intent.
- terminal is written after mock response.
- request hash and redacted CID are present.
- raw request body is not public.

Ship gate:

```bash
cargo test --test tc_external_call_records --no-fail-fast
```

Expected: command exits 0 without network.

Audit: Security Auditor.
Verdict: `NO-VIOLATION` or `VIOLATION-FOUND secret-leak <file>:<line>`.
