# A06 External Call Outbox Preflight

Date: 2026-06-05

Parent plan:
`handover/directives/2026-06-05_AGENTIC_OS_PIVOT_EXECUTION_PLAN_DRAFT.md`

Atom: A06. ExternalCall Outbox, Crash Matrix, and Orphan Sweeper

Document role: Class 0 preflight. This document does not authorize production
LLM/network path changes by itself.

## Decision

A06 should not directly wrap the production LLM/network driver until A04 and
A05 establish the physical ChainTape authority and generic event envelope.
Without those predecessors, A06 would have to invent its own Intent/Terminal
store, which would violate the plan's single-source-of-truth constraint.

Safe work now:

- docs-only preflight
- crash-matrix design
- static no-network replay gate design

Blocked until A04/A05 predecessor contracts are ratified:

- changing `src/drivers/llm_http.rs` behavior
- adding production provider wrappers
- claiming clean halt from memory-only pending intent counters
- adding an orphan sweeper that does not append a tape-visible terminal event

## Hard Blockers

- A06-HB1: Production wrapping of LLM/network calls is blocked until A04
  ChainTape-L4 authority and A05 TapeEvent envelope exist.
- A06-HB2: No clean halt may rely on memory-only pending intent counters; every
  Intent must have exactly one tape-visible Terminal.
- A06-HB3: The boot orphan sweeper must append an
  `ExternalCallTerminal::Abandoned { reason: OS_CRASH_RECOVERY,
  may_have_spent: true }`-equivalent event for stale orphan intents.
- A06-HB4: Wrapping only `src/drivers/llm_http.rs` cannot claim A06 completion
  unless the CLI/Web `chat_client`, `cmd_llm`, `cmd_generate`, `cmd_spec`, and
  web shellout inventory is explicitly covered or left as a failing gate.
- A06-HB5: Recovery from `after_provider_before_terminal` must not reissue the
  physical provider call unless the original `logical_call_id` and
  `idempotency_key` are reused against a provider-supported idempotent endpoint.
  Otherwise the sweeper appends Abandoned with `may_have_spent=true`.

## Current-State Facts

Parent-plan A06 allowed paths as originally written:

```text
src/runtime/external_call.rs
src/runtime/tc_tape_canonical.rs
src/drivers/llm_http.rs
src/runtime/orphan_intent_sweeper.rs
tests/tc_external_call_records.rs
tests/tc_tape_canonical_repairs.rs
tests/external_call_orphan_sweeper.rs
```

Corrected implementation path inventory:

```text
src/runtime/external_call.rs
src/runtime/tc_tape_canonical.rs
src/runtime/orphan_intent_sweeper.rs
src/runtime/mod.rs
src/bin/turingos/chat_client.rs
src/bin/turingos/cmd_llm.rs
src/bin/turingos/cmd_generate.rs
src/bin/turingos/cmd_spec.rs
src/drivers/llm_http.rs
src/web/spec.rs
src/web/generate.rs
tests/tc_external_call_records.rs
tests/tc_tape_canonical_repairs.rs
tests/external_call_orphan_sweeper.rs
tests/offline_replay_no_llm_dependency_static_check.rs
```

Existence check:

```text
MISSING src/runtime/external_call.rs
MISSING src/runtime/tc_tape_canonical.rs
EXISTS src/drivers/llm_http.rs
MISSING src/runtime/orphan_intent_sweeper.rs
MISSING tests/tc_external_call_records.rs
MISSING tests/tc_tape_canonical_repairs.rs
MISSING tests/external_call_orphan_sweeper.rs
```

Dirty-path check for A06 allowed paths:

```text
no current dirty diff in the originally listed A06 paths.

pre-existing dirty paths in the corrected inventory include:
  src/runtime/mod.rs
  src/bin/turingos/cmd_spec.rs

Implementation must read and preserve those edits. Do not overwrite them as
part of A06 wiring.
```

Existing LLM/network call surfaces:

```text
src/bin/turingos/chat_client.rs:20
  SiliconFlow endpoint default
src/bin/turingos/chat_client.rs:296
  chat_complete(...)
src/bin/turingos/chat_client.rs:313
  reqwest POST/send for CLI production path
src/bin/turingos/cmd_llm.rs
  complete / triage / prompt-eval callers use chat_complete
src/bin/turingos/cmd_generate.rs
  generate path calls chat_complete_blocking
src/bin/turingos/cmd_spec.rs
  spec path calls chat_complete_blocking and shells out to turingos llm
src/web/spec.rs
  web path shells out to turingos llm complete
src/web/generate.rs
  web path shells out to turingos generate
src/drivers/llm_http.rs:65   ResilientLLMClient
src/drivers/llm_http.rs:88   generate(...)
src/drivers/llm_http.rs:106  HTTP POST to local proxy
src/drivers/llm_http.rs:139  returns GenerateResponse
src/drivers/llm_http.rs:157  returns last DriverError
```

Do not claim A06 completion by wrapping only `src/drivers/llm_http.rs`.
The CLI/Web production path currently routes through `chat_client.rs` and its
callers.

Existing evidence/canonicality neighbors:

```text
src/runtime/attempt_telemetry.rs:1
  AttemptTelemetry + LeanResult + TerminalAbortRecord CAS object schemas
src/runtime/attempt_telemetry.rs:36
  AttemptTelemetry stores parsed external candidate bytes, never raw LLM response
src/runtime/attempt_telemetry.rs:94
  TerminalAbortRecord schema id
tests/tb_18r_attempt_telemetry_per_llm_call.rs:1
  per-LLM-call AttemptTelemetry path-shape tests
tests/offline_replay_no_llm_dependency_static_check.rs:1
  offline replay must not import LLM/network clients
tests/constitution_tape_canonical_gate.rs:146
  all externalized attempts have CAS payload
src/bottom_white/cas/schema.rs:134
  ObjectType::Generic exists
src/bottom_white/cas/schema.rs:150
  CasObjectMetadata.schema_id exists
```

Important distinction:

```text
AttemptTelemetry / TerminalAbortRecord are existing evidence capsules.
They are not the same as the planned ExternalCallIntent /
ExternalCallTerminal outbox protocol. Do not rename them into A06 completion.
```

CAS schema decision:

```text
Safer non-schema option:
  store ExternalCall request/response payloads as ObjectType::Generic with a
  stable schema_id such as turingos.external_call.intent.v1 /
  turingos.external_call.terminal.v1.

Higher-risk option:
  add dedicated CAS ObjectType variants for ExternalCallIntent and
  ExternalCallTerminal.

The higher-risk option changes a typed CAS schema surface and must be
classified and ratified explicitly before implementation.
```

## Risk Classification

Risk floor: Class 2.

Promote to Class 3 if:

- `src/drivers/llm_http.rs` production behavior changes
- provider tokens, money, or retry semantics change
- network/LLM capability routing changes
- clean-halt conditions become production gates

Promote to Class 4 if:

- sequencer admission changes
- typed tx schema changes
- canonical signing payload changes
- trust-root / constitution / flowchart authority changes

## Recommended Contract

Minimum external call state machine after A05 exists:

```text
ExternalCallIntent {
  call_id: String,
  logical_t: u64,
  provider: String,
  operation: String,
  request_cid: Cid,
}

ExternalCallTerminal {
  call_id: String,
  intent_logical_t: u64,
  status: ExternalCallStatus,
  response_cid: Option<Cid>,
  error_class: Option<String>,
  may_have_spent: bool,
}
```

Required invariant:

```text
for every Intent there is exactly one Terminal.
clean halt requires pending_intents == 0.
replay reads Intent/Terminal events and never calls network or LLM.
after_provider_before_terminal recovery does not duplicate physical calls.
permitted retries reuse the same logical_call_id/idempotency_key pair.
uncertain provider completion is Abandoned may_have_spent=true.
```

Required crash matrix:

```text
before_intent
after_intent_before_provider
after_provider_before_terminal
provider_500
timeout
```

## Atomized A06 Tasks

### A06.0 Preflight Lock

Description:
Record missing files, predecessor dependency, and the distinction between
existing AttemptTelemetry evidence and the planned ExternalCall outbox.

Acceptance:

```bash
for f in \
  handover/directives/2026-06-05_A06_EXTERNAL_CALL_OUTBOX_PREFLIGHT.md \
  handover/directives/2026-06-05_AGENTIC_OS_PIVOT_EXECUTION_PLAN_DRAFT.md
do
  git diff --no-index --check /dev/null "$f" || true
done
```

Expected:

```text
no whitespace errors.
A06 preflight states A04/A05 are predecessors.
```

### A06.1 Outbox Records

Description:
After A05 exists, add tests proving provider calls write Intent before the
physical call when possible and exactly one Terminal after completion.

Acceptance:

```bash
cargo test --test tc_external_call_records --no-fail-fast -- --test-threads=1
cargo test --test offline_replay_no_llm_dependency_static_check --no-fail-fast
git diff --check
```

Expected:

```text
successful provider call emits Intent + Terminal.
provider_500 emits Intent + Terminal(error).
timeout emits Intent + Terminal(error).
restarted after_provider_before_terminal does not make a second physical call.
retry path reuses the same logical_call_id/idempotency_key when the provider is
idempotent.
replay does not import or call network/LLM code.
call-site inventory covers chat_client, cmd_llm, cmd_generate, cmd_spec,
web shellouts, and llm_http.
```

### A06.2 Orphan Sweeper

Description:
Add boot-time orphan sweeper only after Intent/Terminal events are tape-visible.

Acceptance:

```bash
cargo test --test external_call_orphan_sweeper --no-fail-fast -- --test-threads=1
cargo test --test tc_tape_canonical_repairs --no-fail-fast -- --test-threads=1
git diff --check
```

Expected:

```text
stale unclosed Intent appends Abandoned terminal.
reason == OS_CRASH_RECOVERY.
may_have_spent == true when provider may have received the request.
no memory-only cleanup is accepted as closure.
```

## Final Pre-Implementation Gate

A06 implementation may start only when all are true:

- A04 has selected ChainTape-L4 as physical authority
- A05 has landed or ratified the generic TapeEvent envelope
- the first code change is a failing test for Intent/Terminal pairing
- production `llm_http` changes are classified as Class 3
- CLI/Web `chat_client` call surfaces are in scope or explicitly deferred with
  a failing inventory test
- CAS schema choice is explicit: ObjectType::Generic + schema_id, or ratified
  dedicated ObjectType variants
- offline replay static checks remain network-off and model-off

Clean-context audit input for a future implementation PR:

```text
Task brief: A06 ExternalCall Outbox, Crash Matrix, and Orphan Sweeper.
Risk class: Class 2; promote to Class 3 if production LLM/network capability
paths are changed.
FC nodes: FC1-N7, FC1-N13, FC2-N22, FC2 boot/replay, FC3 logs archive.
Evidence: A04/A05 predecessor evidence, crash-matrix tests, offline replay
test, orphan sweeper test, call-site inventory, constitution gates.
Verdict domain: NO-VIOLATION | VIOLATION-FOUND | RECONSTRUCTION-FAILURE |
SECOND-SOURCE-DRIFT
```
