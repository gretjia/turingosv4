# A2 — Anthropic Messages API vs OpenAI Chat Completions protocol differences

| Field | Value |
|-------|-------|
| Date | 2026-05-21 |
| Phase | A (research) — dispatch 2 of 3 |
| Agent | sonnet, general-purpose, WebFetch |
| Sources | docs.anthropic.com, platform.openai.com, api-docs.deepseek.com, openrouter.ai/docs |
| Word count | ~2000 |

## TL;DR

A single Rust client cannot cleanly speak both protocols behind a thin adapter without losing information. The protocols differ in authentication headers, system-prompt placement, tool-result routing, streaming event shape, and thinking-block representation. The cleanest architecture is a protocol enum with two concrete dispatch paths sharing a common `ChatResult` output type — not a single wire struct with optional fields, and not OpenRouter-as-router (which silently drops Anthropic-specific fields).

## Side-by-Side Comparison Table

| Axis | Anthropic Messages API | OpenAI Chat Completions | DeepSeek (OpenAI-clone) | Key diff |
|------|----------------------|------------------------|------------------------|----------|
| **1. Endpoint URL** | `POST https://api.anthropic.com/v1/messages` ([source](https://platform.claude.com/docs/en/api/messages)) | `POST https://<base>/v1/chat/completions` | `POST https://api.deepseek.com/v1/chat/completions`; also `https://api.deepseek.com/anthropic` for native Anthropic wire ([source](https://api-docs.deepseek.com/api/create-chat-completion)) | Different path; Anthropic path is `/v1/messages`, not `/v1/chat/completions` — base-URL swap alone is insufficient |
| **2. Authentication** | `x-api-key: <key>` + required `anthropic-version: 2023-06-01` header ([source](https://platform.claude.com/docs/en/api/messages)) | `Authorization: Bearer <key>` | `Authorization: Bearer <key>` | Two different auth header names; Anthropic also mandates a version header that OpenAI has no equivalent for |
| **3. System prompt** | Top-level `system` field: `string` or `Array<TextBlockParam>` (with per-block `cache_control`) ([source](https://platform.claude.com/docs/en/api/messages)) | First message in `messages` array with `role: "system"` | Same as OpenAI | Anthropic system is structurally separate; OpenAI system is a message-array entry. Cache control attaches to Anthropic's system blocks, not possible in OpenAI shape |
| **4. Messages array** | `[{role: "user"\|"assistant", content: string \| ContentBlock[]}]` — no `"system"` or `"tool"` role in array ([source](https://platform.claude.com/docs/en/api/messages)) | `[{role: "system"\|"user"\|"assistant"\|"tool", content: string}]` | Same as OpenAI | Tool results go into user-turn content blocks (`type: "tool_result"`) in Anthropic vs dedicated `role: "tool"` messages in OpenAI |
| **5. Tool use** | `tools: [{name, description, input_schema}]`; response content contains `{type: "tool_use", id, name, input: {}}` blocks ([source](https://platform.claude.com/docs/en/api/messages)) | `tools: [{type: "function", function: {name, description, parameters}}]`; response `choices[0].message.tool_calls: [{id, type, function: {name, arguments}}]` | Same as OpenAI; `arguments` is a JSON string, not object | Schema key is `input_schema` vs `parameters`; tool result is a content block vs a separate `role:"tool"` message; `input` is already parsed object vs `arguments` JSON string |
| **6. Streaming** | SSE: named events `message_start`, `content_block_start`, `content_block_delta` (`text_delta` \| `input_json_delta` \| `thinking_delta` \| `signature_delta`), `content_block_stop`, `message_delta`, `message_stop`, `ping`, `error` ([source](https://platform.claude.com/docs/en/api/messages-streaming)) | SSE: `data: {choices: [{delta: {role?, content?, tool_calls?}}]}` chunks; final `data: [DONE]` | Same as OpenAI | Anthropic is block-indexed multi-event per content piece; OpenAI is a flat delta on `choices[0].delta`. Tool-input streaming: Anthropic emits `input_json_delta.partial_json` strings; OpenAI emits `tool_calls[i].function.arguments` string chunks |
| **7. Reasoning/thinking** | `thinking: {type: "enabled"\|"disabled"\|"adaptive", budget_tokens: N}`; response has `{type: "thinking", thinking: "..."}` content blocks alongside text blocks ([source](https://platform.claude.com/docs/en/api/messages)) | o1/o3: `reasoning_effort` field (not `thinking`); reasoning content is hidden from response — not returned to caller ([not stated in official docs for return shape](https://platform.openai.com/docs/api-reference/chat/create)) | `thinking: {type: "enabled"}` + `reasoning_effort: "high"\|"max"`; response `choices[0].message.reasoning_content: string\|null` ([source](https://api-docs.deepseek.com/api/create-chat-completion)) | Three different shapes: Anthropic returns thinking as a typed content block; DeepSeek returns it as a sibling field on the message; OpenAI hides it entirely |
| **8. Token usage** | `usage: {input_tokens, output_tokens, cache_creation_input_tokens?, cache_read_input_tokens?}` ([source](https://platform.claude.com/docs/en/api/messages)) | `usage: {prompt_tokens, completion_tokens, total_tokens, completion_tokens_details?}` | `usage: {prompt_tokens, completion_tokens, total_tokens, prompt_cache_hit_tokens, prompt_cache_miss_tokens, completion_tokens_details: {reasoning_tokens}}` ([source](https://api-docs.deepseek.com/api/create-chat-completion)) | Field names differ (`input_tokens` vs `prompt_tokens`, `output_tokens` vs `completion_tokens`); cache accounting is provider-specific (Anthropic: creation+read; DeepSeek: hit+miss) |
| **9. Errors** | `{type: "error", error: {type: string, message: string}}`; HTTP 529 for overload; error events in stream use same shape ([source](https://platform.claude.com/docs/en/api/messages)) | `{error: {message, type, param, code}}`; HTTP 503 for overload (not stated in official docs for all codes) | Same as OpenAI shape | Anthropic wraps in outer `type: "error"` key; HTTP 529 (Anthropic) vs 503 (OpenAI) for overload — retry logic needs separate branches |
| **10. Cache/prompt-caching** | Explicit `cache_control: {type: "ephemeral", ttl?: "5m"\|"1h"}` on individual content blocks or system field; reflected in `cache_creation_input_tokens` / `cache_read_input_tokens` in usage ([source](https://platform.claude.com/docs/en/api/messages)) | Automatic, no client knob; not stated in official docs how to observe cache hits | `prompt_cache_hit_tokens` / `prompt_cache_miss_tokens` in usage; no explicit request-side knob (automatic) | Only Anthropic exposes explicit per-block cache control, enabling deterministic cache placement for prompt-hash CIDs |

## Where the Protocols Don't Map

**1. Tool result routing.** In Anthropic, tool results are `type: "tool_result"` blocks inside a `user`-role message's content array, referencing `tool_use_id`. In OpenAI, they are separate `role: "tool"` messages with `tool_call_id`. A unified `messages` builder must branch on protocol before serialization — there is no shared shape.

**2. System prompt as typed content.** Anthropic's `system` field accepts an array of `TextBlockParam` objects each with `cache_control`. OpenAI's system is a plain string inside a message-array entry. A unified abstraction that wants to honor Anthropic cache placement on system context has no OpenAI equivalent; the cache hint is silently dropped when targeting OpenAI-compatible endpoints.

**3. Thinking/reasoning content block.** Anthropic returns `thinking` as a typed content block in the `content` array, adjacent to text blocks, with a cryptographic `signature_delta` in streaming. DeepSeek returns it as `message.reasoning_content` (sibling field). OpenAI hides it entirely. These three shapes cannot be mapped to a single response type without losing either the block ordering (Anthropic) or the reasoning text (OpenAI).

**4. Streaming event granularity.** Anthropic streaming is block-indexed: you get a `content_block_start` naming the block type, then deltas referencing that index. Multiple block types (text, tool_use, thinking) can interleave. OpenAI streaming is a single flat `choices[0].delta` object. Consumers that want to act on "thinking started" vs "text started" signals must write two completely different stream parsers.

**5. Prompt-cache observability.** TuringOS writes prompt-hash CIDs and may want cache-aware routing. Anthropic exposes `cache_creation_input_tokens` and `cache_read_input_tokens` with explicit per-block `cache_control` knobs — meaning a client can predict and verify caching. OpenAI/DeepSeek automatic caching gives only after-the-fact token counts with no request-side control. Any CID-anchored cache-accounting logic is Anthropic-only.

## Refactor Cost Estimate

The existing codebase has two OpenAI-compatible clients: `src/drivers/llm_http.rs` (proxy-based, ~200 LOC) and `src/bin/turingos/siliconflow_client.rs` (~364 LOC, already handles `reasoning_content` and `thinking` config for DeepSeek).

To add a native Anthropic client alongside, back-of-envelope:

| Item | Estimate |
|------|----------|
| New `AnthropicRequest` / `AnthropicMessage` / `ContentBlock` types (system, text, tool_use, tool_result, thinking, document) | ~150 LOC new types |
| New `AnthropicResponse` / `ContentBlock` enum deserializer + `AnthropicUsage` | ~80 LOC |
| New `AnthropicError` variant in `LlmError` or new error type | ~20 LOC |
| Request-building logic (system field extraction, content block construction, tool schema translation) | ~100 LOC |
| Response-normalization into existing `ChatResult` (flatten content blocks to string, extract `reasoning_content` from thinking blocks) | ~60 LOC |
| Streaming parser (new SSE event enum: `MessageStart`, `ContentBlockStart`, `ContentBlockDelta`, `MessageStop`, etc.) | ~200 LOC (if streaming required) |
| Auth header dispatch (`x-api-key` + `anthropic-version` instead of `Authorization: Bearer`) | ~15 LOC change in `chat_complete` |
| `Protocol` enum and dispatch wrapper in `siliconflow_client.rs` or new `llm_dispatch.rs` | ~50 LOC |

**Total: ~500 LOC for non-streaming; ~700 LOC with streaming.** The existing `ChatResult` output type can be reused with one addition: `reasoning_content: Option<String>` already exists in `siliconflow_client.rs`, so the output contract is already Anthropic-aware (mostly — see A2 §3 thinking-block caveat below).

No existing types need to be deleted; the change is additive. The biggest complexity is the streaming event parser if streaming is needed immediately.

## Practical Recommendation

**Use a `Protocol` enum with two concrete dispatch paths — not OpenRouter-as-router.**

```rust
enum Protocol { OpenAiCompat, Anthropic }
```

Reasoning:

- **OpenRouter-as-router** unifies the wire call but silently drops Anthropic-specific features: `cache_control` blocks, `thinking` content blocks, `cache_creation_input_tokens` in usage, and explicit `anthropic-version` negotiation. For TuringOS's CID-anchored cache accounting, this is a hard loss. OpenRouter's documented base URL is `https://openrouter.ai/api/v1` using `Authorization: Bearer` and OpenAI-compatible shape — it translates Anthropic models to its own normalized format, not to the native Anthropic wire ([openrouter.ai/docs](https://openrouter.ai/docs)).

- **A single unified struct with optional fields** becomes unmaintainable because the structural divergences (tool result routing, system prompt placement, thinking blocks) require branching at serialization time anyway — you end up with hidden protocol-detection logic inside `to_json()` methods.

- **Two concrete dispatch paths behind a `Protocol` enum** is the right posture. The `siliconflow_client.rs` already does this implicitly (it is an OpenAI-compat client). Adding an `anthropic_client.rs` alongside it, with a thin `LlmDispatch` wrapper that routes on `Protocol`, is ~500 LOC, additive, and keeps each protocol's serialization logic isolated. The `ChatResult` output type is already compatible.

For the Meta=Claude Opus + Worker=DeepSeek Flash scenario specifically: Claude Opus runs the Anthropic path (native `thinking` blocks, explicit cache control on the long system prompt — which matters for TuringOS's repeated-context token costs); DeepSeek Flash runs the existing SiliconFlow/OpenAI-compat path. The dispatch wrapper selects path per-model-config, not per-call. This matches the `ResilientLLMClient` / `siliconflow_client` split already in the codebase.

## Key sources

- https://platform.claude.com/docs/en/api/messages
- https://platform.claude.com/docs/en/api/messages-streaming
- https://platform.openai.com/docs/api-reference/chat/create
- https://api-docs.deepseek.com/api/create-chat-completion
- https://openrouter.ai/docs
