# A3 — v4 codebase refactor cost analysis for multi-provider LLM

| Field | Value |
|-------|-------|
| Date | 2026-05-21 |
| Phase | A (research) — dispatch 3 of 3 |
| Agent | Explore (read-only) |
| Source | `/home/zephryj/projects/turingosv4` working tree at HEAD `dbb3c485` |
| Word count | ~1700 |

## TL;DR

**10 files changed** (siliconflow_client.rs + cmd_spec.rs + cmd_generate.rs + cmd_llm.rs + cmd_welcome.rs + cmd_init.rs + 4 test files), **2-3 new enum variants** (LlmError::ProtocolMismatch, LlmError::EndpointMissing, LlmError::UnknownProtocol), **6 integration tests** need HTTP-request shape updates. Current codebase hardcodes SiliconFlow endpoint and OpenAI-Chat-Completions protocol; multi-provider refactor requires moving endpoint config to turingos.toml + protocol enum, changing `chat_complete` signature from positional (api_key, model, messages, …) to a per-role `LlmConfig` struct.

## Touch-surface map

| File | Change | LoC Est. |
|------|--------|----------|
| `src/bin/turingos/siliconflow_client.rs:34-37` | Endpoint reader now reads from turingos.toml (protocol-aware dispatcher) | 20 |
| `src/bin/turingos/siliconflow_client.rs:266, 330` | `chat_complete` / `chat_complete_blocking` signature: add `endpoint: &str, protocol: Protocol` | 10 |
| `src/bin/turingos/siliconflow_client.rs:150-158` | `LlmError` enum: add `ProtocolMismatch`, `EndpointMissing`, `UnknownProtocol` | 15 |
| `src/bin/turingos/cmd_llm.rs:376-413` | `write_config` adds `llm.meta.endpoint`, `llm.meta.protocol`, `llm.blackbox.endpoint`, `llm.blackbox.protocol` slots | 35 |
| `src/bin/turingos/cmd_llm.rs:415-510` | 6 new reader functions: `read_meta_endpoint`, `read_blackbox_endpoint`, `read_meta_protocol`, `read_blackbox_protocol` + fallback defaults | 60 |
| `src/bin/turingos/cmd_llm.rs:875, 1352, 2122` | 3 call sites of `chat_complete`: thread endpoint + protocol from config readers | 30 |
| `src/bin/turingos/cmd_spec.rs:335, 1361` | 2 call sites of `chat_complete_blocking` (Meta role): thread endpoint + protocol | 20 |
| `src/bin/turingos/cmd_generate.rs:277` | 1 call site of `chat_complete_blocking` (Blackbox role): thread endpoint + protocol | 10 |
| `src/bin/turingos/cmd_welcome.rs:218-242` | Endpoint validation added: check `llm.meta.endpoint` and `llm.blackbox.endpoint` in welcome checklist | 15 |
| `src/bin/turingos/cmd_init.rs:307-334` | Init output updated: mention protocol selector + endpoint discovery in next-steps | 5 |
| **Tests**: `deepseek_thinking_param_serialized.rs` | Mirror structs stay same (JSON wire contract unchanged); no mock HTTP changes | 0 |
| **Tests**: 6 stubs (complete/triage/prompt_eval/generate/spec) | Mock server now validates endpoint match before allowing request (protocol TBD per role) | 40 |
| **Test**: `offline_replay_no_llm_dependency_static_check.rs:2` | Check siliconflow_client imports: no change needed (no live LLM in replay) | 0 |

**Total new code**: ~260 lines (reader functions, error variants, endpoint validation).

## Hidden complexity

1. **Per-role endpoint parity**: cmd_spec and cmd_generate read endpoint from turingos.toml independently. If a user configures mismatched endpoints (meta=OpenAI, blackbox=SiliconFlow), the code must handle protocol switching on-the-fly. Current `chat_complete` is OpenAI-Chat-Completions-only; Anthropic Messages protocol has incompatible message shape (no `role: "user"` field — uses `role: "user" | "assistant"` + `content: ContentBlock[]`). Cannot use same `ChatMessage` struct.

2. **Backward compatibility: env var fallback complexity**: TURINGOS_SILICONFLOW_ENDPOINT is currently read at call-time (siliconflow_client.rs:35, endpoint() function). Post-refactor, this must persist into turingos.toml on first `turingos llm config`. If user has TURINGOS_SILICONFLOW_ENDPOINT set but no turingos.toml endpoint, welcome checklist should warn. Current welcome only checks env-var names (not values). Will need 3-tier fallback: (1) turingos.toml, (2) TURINGOS_SILICONFLOW_ENDPOINT env var, (3) hardcoded default.

3. **ChatRequest / ChatResponse struct mutation**: Current structs are OpenAI-shaped (role: "user" | "assistant" | "system", content: String). Anthropic Messages API uses role: "user" | "assistant", content: ContentBlock[] (where ContentBlock = Text { text: String } | ToolUse { … } | …). Refactor needs separate request/response types per protocol, OR a unified envelope that serializes/deserializes per protocol at the boundary. The test mirrors (deepseek_thinking_param_serialized.rs) would need Anthropic equivalents.

4. **Test mock server complexity**: 6 integration tests (cmd_llm_complete_stub.rs, cmd_llm_triage_stub.rs, etc.) spin up httptest mock servers and set TURINGOS_SILICONFLOW_ENDPOINT. They currently mock a single OpenAI-compat response shape. Post-refactor, each test needs to:
   - Accept either endpoint (meta or blackbox)
   - Validate the protocol enum in the request body (if present)
   - Return response shaped per protocol
   Current mock bodies are: `{choices: [{message: {role, content, [reasoning_content]}}]}`. Anthropic Messages response is `{content: [{type, text}], stop_reason, usage: {input_tokens, output_tokens}}`.

5. **`cmd_init.rs` provider preset flow**: Currently, init writes static turingos.toml with no LLM config at all. The help text (line 328) points to `turingos llm config --workspace <ws>` without `--provider` flag. Proposed `--provider anthropic` / `--provider openrouter` CLI requires cmd_init.rs to parse and pass the provider to write_config. This is a new CLI surface (cmd_init.rs doesn't currently handle --provider) and would conflict with cmd_llm.rs (which owns provider config). Decision needed: add --provider to init, or require separate `turingos llm config` call.

> NOTE (post-research update 2026-05-21): The `--provider` flag was added to `cmd_init.rs` as part of PR #69 (B8 fix). That conflict is resolved as of `04b828f4`.

## Backward compat strategy

**Keep TURINGOS_SILICONFLOW_ENDPOINT as fallback (no hard break)**:

- siliconflow_client.rs:endpoint() already reads env var with fallback. Post-refactor, read order:
  1. `llm.meta.endpoint` / `llm.blackbox.endpoint` from turingos.toml (per-role)
  2. TURINGOS_SILICONFLOW_ENDPOINT env var (global, for both roles)
  3. Hardcoded SILICONFLOW_ENDPOINT constant (https://api.siliconflow.cn/v1/chat/completions)

- `turingos welcome` prints warning: "endpoint not in turingos.toml; falling back to TURINGOS_SILICONFLOW_ENDPOINT env var. For stability, run: turingos llm config --workspace <ws> to persist."

- Do NOT delete or deprecate TURINGOS_SILICONFLOW_ENDPOINT in this cycle — keep it live for users who have shell aliases / scripts using it.

## Trust Root churn estimate

**Files in [trust_root] affected**:

1. `src/bin/turingos/siliconflow_client.rs` (NOT pinned in genesis_payload.toml; lives in root workspace)
2. `src/bin/turingos/cmd_llm.rs` (NOT pinned)
3. `Cargo.lock` — if new protocol enum is in a dep, or if request/response serialization changes Serde code-gen (likely NO churn unless we add a dep like anthropic-sdk crate)
4. `Cargo.toml` — same as above

**Will genesis_payload.toml trust_root rehash?** YES.
- Reason: siliconflow_client.rs changes (endpoint reading logic, protocol dispatch) + cmd_llm.rs (config write slots) are in the binary, and any binary-crate Rust code change triggers Cargo.lock rehash.
- Per preceding Cz cycle 2 note (2026-05-21 PR #60): "Cargo.lock regenerated from auto-merged Cargo.toml" — even non-[dependencies] changes in binary crate force rehash.

**Cz cycle 3 impact**: Expect Cargo.lock + Cargo.toml to rehash. No new deps needed (keep using reqwest + serde_json). New enum + struct do NOT require dep additions; they live in siliconflow_client.rs.

## Test scaffolding required

**Existing tests that need HTTP-request-shape rework** (6 files):

| Test file | Current mock | Required update | LoC change |
|-----------|--------------|-----------------|-----------|
| `tests/deepseek_thinking_param_serialized.rs:3` | Serde round-trip for ChatRequest + ThinkingConfig | Add mirror structs for Anthropic Messages ChatRequest (content: ContentBlock[], role enum only 2 variants) | +40 |
| `tests/deepseek_model_name_rejected_actionable.rs:3` | HTTP 400 error message parsing ("supported API model names") | Endpoint error-message mapping per protocol (OpenAI ≠ Anthropic 401/403 format) | +25 |
| `tests/cmd_llm_complete_stub.rs:5` | httptest mock returns `{choices: [{message: {content, reasoning_content}}]}` | Parameterize response per protocol; validate `protocol` enum in request body before responding | +60 |
| `tests/cmd_llm_triage_stub.rs:5` | Same OpenAI shape | Same as above | +60 |
| `tests/cmd_llm_prompt_eval_*.rs` (3 files) | Same OpenAI shape | Same as above | +180 (60 per test) |
| `tests/generate_*.rs` stubs (4 files using TURINGOS_SILICONFLOW_ENDPOINT) | HTTP 500 error on protocol mismatch (currently hardcoded Blackbox=OpenAI-compat) | Validate protocol enum in request; return correct response shape | +80 |

**New tests to add** (3 recommended):

1. `protocol_mismatch_anthropic_vs_openai.rs` — verify that passing Anthropic Messages request to OpenAI endpoint (or vice versa) returns actionable error (not silent corruption). LoC: ~80.
2. `endpoint_override_per_role_independent.rs` — verify that Meta and Blackbox can use different endpoints/protocols without interference. LoC: ~100.
3. `turingos_init_provider_preset.rs` — (already shipped in PR #69) verify that `turingos init --project foo --provider anthropic` writes llm.meta.protocol=AnthropicMessages + correct endpoint. LoC: ~90.

**Total test churn**: ~500 LoC across 9 files (6 existing + 3 new).

## Cited call sites (working-tree snapshot 2026-05-21)

**chat_complete call sites:**
- cmd_spec.rs:335 (Meta role, spec grill 8-question loop)
- cmd_spec.rs:1361 (Meta role, mid-chain follow-up)
- cmd_generate.rs:277 (Blackbox role, fast code generation)
- cmd_llm.rs:875 (complete action, role-agnostic per flag)
- cmd_llm.rs:1352 (triage action, Blackbox only)
- cmd_llm.rs:2122 (prompt-eval action, role-agnostic per flag)

**TURINGOS_SILICONFLOW_ENDPOINT readers:**
- siliconflow_client.rs:35 (endpoint() function — current sole reader)
- Tests reference at 9 sites (all set it via .env() in test harness)

**Config readers (turingos.toml):**
- cmd_llm.rs:441 (read_meta_model)
- cmd_llm.rs:448 (read_blackbox_model)
- cmd_llm.rs:458 (read_meta_api_key_env)
- cmd_llm.rs:468 (read_blackbox_api_key_env)
- cmd_llm.rs:477 (read_meta_thinking)
- cmd_llm.rs:490 (read_blackbox_thinking)

**Config writers (turingos.toml):**
- cmd_llm.rs:376-413 (write_config — add 4 new slots for endpoints + protocols)

**LlmError enum variants** (cmd_llm.rs:166-174):
- MissingAction, UnknownAction, MissingFlag, WorkspaceNotFound, Io, MetaKeyEnvNotConfigured, BlackboxKeyEnvNotConfigured
- Add: ProtocolMismatch, EndpointMissing, UnknownProtocol (3 new variants)

## How to consume this analysis

1. If a future PR wants to add native Anthropic dispatch: use the touch-surface map above as the LoC scoping baseline.
2. If a future PR wants only to add a non-Anthropic OpenAI-compat provider (e.g. OpenAI direct, Azure OpenAI, Volcengine): no Protocol enum needed; just add a preset to `cmd_init.rs` (already wired) + ensure endpoint resolution covers the new base URL via `TURINGOS_SILICONFLOW_ENDPOINT` or future `llm.*.endpoint` TOML slot.
3. The 6 existing mock-LLM tests are the bottleneck for ANY wire-shape change. Adding protocol divergence multiplies their complexity.
