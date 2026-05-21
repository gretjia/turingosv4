# A1 — Industry multi-provider LLM best practices survey

| Field | Value |
|-------|-------|
| Date | 2026-05-21 |
| Phase | A (research) — dispatch 1 of 3 |
| Agent | sonnet, general-purpose, WebFetch + WebSearch |
| Sources | OpenRouter, LiteLLM, Vercel AI SDK, LangChain, Aider, Anthropic+OpenAI raw SDKs |
| Word count | ~2400 |

## TL;DR

The dominant pattern across all six systems is: **OpenAI-compatible HTTP as the universal wire protocol, `provider/model` prefixed strings as the canonical model ID namespace, and per-provider env vars (`OPENAI_API_KEY`, `ANTHROPIC_API_KEY`, etc.) as the auth surface**. Every system that supports multiple providers either (a) routes everything through a single OpenAI-compatible endpoint (OpenRouter, LiteLLM proxy), or (b) provides a thin typed wrapper per provider that normalizes to the same call shape at the application layer (Vercel AI SDK, LangChain). No system invented a new wire protocol — the only real divergence is whether Anthropic's native `messages` API is wrapped at the SDK layer or at the proxy layer.

## Per-system findings

| System | Q1: Config surface | Q2: Endpoint handling | Q3: Protocol abstraction | Q4: Auth | Q5: Model name normalization | Q6: Migration story |
|--------|-------------------|-----------------------|--------------------------|----------|------------------------------|---------------------|
| **OpenRouter** | In-code: `base_url="https://openrouter.ai/api/v1"` + model string per request. No per-role config surface — caller manages which model string to use per call. | Endpoint is fixed (`openrouter.ai/api/v1`). Provider selection happens via model string suffix (`:nitro`, `:floor`) or per-request `provider` object in extra body. | Single OpenAI-compatible `chat/completions` shape. OpenRouter translates to each backend provider's native API server-side. Caller never sees Anthropic's `x-api-key` header or native messages shape. | Single key: `OPENROUTER_API_KEY` passed as `Authorization: Bearer`. No per-provider keys needed. | Provider-slash-model: `anthropic/claude-opus-4-1`, `deepseek/deepseek-chat`, `openrouter/auto`. Special suffixes: `google/gemini-3.1-pro:nitro`. | Change the model string only. Endpoint, headers, code structure unchanged. |
| **LiteLLM** | Code: `completion(model="provider/model")`. Proxy: YAML `model_list` with `model_name` alias + `litellm_params`. Per-role: define separate aliases like `"fast-model"` and `"smart-model"`. | Per-model `api_base` in YAML. OpenAI-compatible providers use `openai/` prefix + `api_base`. Handles non-OpenAI providers (Anthropic, Bedrock, Vertex) via native adapters server-side. | Single `completion()` call; all responses normalized to OpenAI chat format. Anthropic's native system prompt placement, tool call format, and streaming SSE are translated internally. | Per-provider env vars: `OPENAI_API_KEY`, `ANTHROPIC_API_KEY`, `DEEPSEEK_API_KEY`. Proxy master key `sk-1234` for clients; credentials stored server-side via `os.environ/VAR` syntax in YAML. | Provider-prefix: `anthropic/claude-3-5-sonnet-20241022`, `deepseek/deepseek-chat`, `openai/gpt-4o`. OpenAI-compat: `openai/<model>` + `api_base`. | Change `model` string + set new env var. Code structure unchanged. YAML config: add new entry under `model_list`. |
| **Vercel AI SDK** | In-code: `openai('gpt-5')` or `anthropic('claude-3-haiku')`. Registry: `customProvider({ languageModels: { 'smart': gateway('anthropic/claude-opus-4.1'), 'fast': gateway('openai/gpt-5-mini') }})`. Semantic aliases supported. | Each provider has a `baseURL` override: `createOpenAI({ baseURL: '...' })`, `createAnthropic({ baseURL: '...' })`. OpenAI-compat via `createOpenAICompatible({ name, apiKey, baseURL })`. | Unified `generateText()` / `streamText()` API over all providers. Anthropic provider targets native Messages API internally; caller sees the same `generateText` interface. System prompt, tool calling, reasoning all normalized. | Per-provider env vars: `OPENAI_API_KEY`, `ANTHROPIC_API_KEY` (also `ANTHROPIC_AUTH_TOKEN`). Can override via constructor `apiKey` param. | Per-provider string passed to provider factory: `anthropic('claude-3-haiku-20240307')`, `openai('gpt-5')`. Registry aliases decouple logical names from provider IDs. | Swap registry alias target or provider factory call. Application `generateText` calls unchanged. |
| **LangChain** | `init_chat_model("gpt-5.4")` auto-detects provider, or `init_chat_model("claude-sonnet-4-6", model_provider="anthropic")`. Direct: `ChatOpenAI(...)` / `ChatAnthropic(...)`. Per-role: instantiate separate model objects. | `base_url` param in `init_chat_model()` for OpenAI-compat providers. Direct class: `ChatOpenAI(base_url="...")`. | Unified `.invoke()` / `.stream()` / `.batch()` interface. Each provider class translates internally. Anthropic system prompt and tool calling differences are hidden behind the `BaseChatModel` interface. | Per-provider env vars: `OPENAI_API_KEY`, `ANTHROPIC_API_KEY`, `GOOGLE_API_KEY`. No generic key. | `"provider:model"` syntax in `init_chat_model`: `"openai:o1"`, `"google_genai:gemini-2.5-flash-lite"`. Or bare string with auto-detect. Provider packages pass names directly to upstream API — no LangChain update needed for new models. | Swap model string in `init_chat_model()`. If using direct class, swap class name + env var. Downstream `.invoke()` calls unchanged. |
| **Aider** | CLI flag `--model <name>`, YAML `model: xxx`, or `.env` file. Per-role: primary model + `weak_model_name` in model settings file for "fast" tasks. | OpenAI-compat: `OPENAI_API_BASE` env var or `openai-api-base` in YAML. Provider-specific: `--anthropic-api-key` flag. | Pure OpenAI protocol for all compatible providers. Native Anthropic and Gemini support through separate SDK paths. | Per-provider env vars + YAML keys: `ANTHROPIC_API_KEY`, `OPENAI_API_KEY`, `openai-api-key`, `anthropic-api-key`. Generic `--api-key provider=key` for others. | Provider-prefix for non-native: `openai/<model>` for OpenAI-compat. Native providers: bare string `claude-3-sonnet`, `gpt-4o`. Model warnings on unknown models. | Change `--model` flag or YAML `model:` field + update relevant env var or API key setting. |
| **Anthropic SDK + OpenAI SDK** | In-code only. `Anthropic(api_key=..., base_url=...)` vs `OpenAI(api_key=..., base_url=...)`. No config file. No per-role abstraction at SDK level. | Both SDKs accept `base_url` constructor param and env var (`OPENAI_BASE_URL`). DeepSeek/SiliconFlow via `base_url` override on OpenAI client. Default: `api.anthropic.com/v1` and `api.openai.com/v1`. | Diverge at wire level: Anthropic uses `client.messages.create(model, messages, max_tokens)` with `x-api-key` header. OpenAI uses `client.chat.completions.create(model, messages)` with `Authorization: Bearer`. System prompt: Anthropic takes it as `system=` top-level param; OpenAI takes it as first message with `role: "system"`. | Anthropic: `ANTHROPIC_API_KEY` (or `ANTHROPIC_AUTH_TOKEN`). OpenAI: `OPENAI_API_KEY` (or `OPENAI_BASE_URL`). Per-provider, not generic. | Raw provider strings. No normalization. `"claude-opus-4-6"` for Anthropic; `"gpt-5.2"` for OpenAI. Caller manages naming across providers. | Swap the entire client instantiation + remap system prompt param + change env var. Code change required at call sites. |

## Patterns that converged

**1. OpenAI wire format as the universal substrate.** Every system either is OpenAI-compatible natively (OpenRouter, LiteLLM, Aider's OpenAI-compat path) or provides a thin adapter that normalizes Anthropic's Messages API to the same application-layer call shape (Vercel AI SDK, LangChain, LiteLLM). No new wire protocol was invented.

**2. `provider/model` as the canonical model ID namespace.** OpenRouter (`anthropic/claude-opus-4-1`), LiteLLM (`anthropic/claude-3-5-sonnet-20241022`), Vercel AI SDK (`gateway('anthropic/claude-3-5-sonnet-20240620')`), LangChain (`"anthropic:claude-sonnet-4-6"`) all converged on a prefixed namespace. The separator is `/` or `:` but the concept is identical.

**3. Per-provider env vars, not a single generic key.** Every system uses `OPENAI_API_KEY`, `ANTHROPIC_API_KEY` as separate named vars. None invented a single `LLM_API_KEY` generic. The closest exception is OpenRouter, which uses a single `OPENROUTER_API_KEY` because it is itself a proxy.

**4. `base_url` override as the OpenAI-compat provider handle.** Every system exposes a `base_url` / `api_base` override to point an OpenAI-protocol client at DeepSeek, SiliconFlow, Ollama, or any compatible endpoint. This is the escape hatch for "new provider, not yet natively supported."

**5. Per-role model config is opt-in, not default.** Only Vercel AI SDK's `customProvider` registry and LiteLLM's YAML `model_list` aliases make per-role (smart/fast) a first-class primitive. LangChain requires two separate instantiated objects. Aider uses `weak_model_name`. The raw SDKs have no concept at all. Converged pattern: a named registry/alias mapping semantic role → `provider/model` string.

## Patterns that diverged

**1. Proxy vs. in-process adapter.** OpenRouter and LiteLLM proxy solve the multi-provider problem at the network layer — the application talks to one HTTP endpoint. Vercel AI SDK and LangChain solve it in-process at the library layer. The proxy approach adds network hop and key centralization; the in-process approach requires the application to ship multiple SDK packages. Neither dominates: LiteLLM supports both modes simultaneously.

**2. Config file vs. pure code.** LiteLLM and Aider both have explicit YAML/dotenv config files for provider setup. Vercel AI SDK, LangChain, and the raw SDKs are pure code. OpenRouter is pure code (env var + model string per call). This matters for TuringOS: a Rust application must choose between TOML/YAML config surface or code-level provider objects.

**3. Model name aliasing depth.** Vercel AI SDK has first-class semantic aliases (`myProvider.languageModel('text-medium')`) that fully decouple the application from provider IDs. LangChain `init_chat_model` adds auto-detect heuristics. LiteLLM YAML adds user-facing aliases over internal IDs. Raw SDKs and OpenRouter have no aliasing — the caller always writes the full `provider/model` string.

**4. How Anthropic's system prompt divergence is handled.** LiteLLM and LangChain translate silently in the adapter. Vercel AI SDK uses the native Anthropic Messages provider internally but exposes the same `generateText` call shape. Raw Anthropic SDK requires `system=` as a top-level param; raw OpenAI SDK takes it as `{"role": "system", "content": "..."}` in messages. Only the proxy/wrapper layer hides this.

**5. Routing intelligence.** OpenRouter has built-in load-balancing, cost-routing, latency-routing, and model-suffix shortcuts (`:nitro`, `:floor`). LiteLLM has YAML-driven strategies (`latency-based-routing`, `cost-based-routing`) and custom hooks. Vercel AI Gateway exposes `sort`, `only`, `order` provider options. LangChain and the raw SDKs have no routing — the caller picks one model per call and that's it.

## Recommended primitive for a fresh multi-provider design

**1. Steal from LiteLLM/Vercel: a named role registry as the primary config surface.** The application code never writes `"anthropic/claude-opus-4-1"` directly. It writes `"meta"` or `"worker"`. The config (TOML or code constant) maps `meta = { provider = "anthropic", model = "claude-opus-4-6", base_url = "...", api_key_env = "ANTHROPIC_API_KEY" }`. This is the single place a user edits to change provider.

**2. Steal from OpenAI SDK + LiteLLM: `base_url` + `api_key` as the two universal provider primitives.** Every provider reduces to these two fields plus a model string. DeepSeek, SiliconFlow, Volcengine — all OpenAI-compat, all work with the same HTTP client once you override `base_url`. No special-case needed. Add a `protocol` field (`openai-compat` | `anthropic-native`) to pick the request serializer.

**3. Steal from LangChain: `provider:model` as the internal canonical ID, not the user-facing one.** Internally use `"anthropic:claude-opus-4-6"` as the stable key in logs and ChainTape. The user-facing config maps role → canonical ID. This prevents ChainTape entries from being ambiguous when the same model is available on multiple providers.

**4. Steal from Vercel AI SDK: `createOpenAICompatible` as the escape hatch for unlisted providers.** Any provider not natively supported gets configured as `{ name: "siliconflow", baseURL: "https://api.siliconflow.cn/v1", apiKey: env("SILICONFLOW_API_KEY") }`. The same HTTP code path handles it. No provider-specific code fork required.

**5. Steal from OpenRouter: single env var per gateway, not per provider, as an option.** For users who do not want to manage multiple API keys, offer an optional "route everything through OpenRouter" mode where `OPENROUTER_API_KEY` + model string is the only config. This gives the zero-friction onboarding path while the native multi-provider path serves power users.

## Things to NOT do

**1. Do not invent a single generic `LLM_API_KEY`.** Every established system uses named per-provider vars. A generic key creates confusion when debugging which provider was actually called, and prevents simultaneous use of multiple providers.

**2. Do not bake provider logic into call sites.** The raw SDK pattern (caller writes `client.messages.create(...)` for Anthropic vs. `client.chat.completions.create(...)` for OpenAI) means every call site must know which provider it is talking to. This is the anti-pattern all six wrappers exist to fix.

**3. Do not use model string directly as ChainTape canonical ID.** `"deepseek-chat"` is ambiguous — it could be DeepSeek direct, SiliconFlow, or OpenRouter routing. The canonical tape ID must include provider source: `"siliconflow:deepseek-chat"` or `"deepseek-direct:deepseek-chat"`. Not stated explicitly in any docs, but implied by the universal `provider/model` namespace convergence.

**4. Do not hardcode endpoint URLs in source.** LiteLLM's `api_base: os.environ/AZURE_API_BASE` pattern and Vercel's `baseURL: process.env.CUSTOM_API_URL` are the correct pattern. Hardcoded URLs make provider rotation without redeploy impossible. CLAUDE.md §4 explicitly forbids hardcoded behavior parameters.

**5. Do not conflate per-request routing logic with per-role config.** OpenRouter's `:nitro` suffix routing and LiteLLM's `latency-based-routing` are request-time optimizations that belong in the gateway, not the role config. Role config (`meta → claude-opus`, `worker → deepseek-flash`) is static and belongs in a config file or init block. Mixing them produces a config surface that neither users nor engineers can reason about.

## Sources cited

- OpenRouter Quickstart / Real Python tutorial: https://realpython.com/openrouter-api/
- OpenRouter provider routing docs: https://openrouter.ai/docs/guides/routing/provider-selection
- LiteLLM proxy config: https://docs.litellm.ai/docs/proxy/configs
- LiteLLM OpenAI-compat: https://docs.litellm.ai/docs/providers/openai_compatible
- LiteLLM routing: https://docs.litellm.ai/docs/routing
- Vercel AI SDK provider management: https://ai-sdk.dev/docs/ai-sdk-core/provider-management
- Vercel AI SDK OpenAI provider: https://ai-sdk.dev/providers/ai-sdk-providers/openai
- Vercel AI SDK Anthropic provider: https://ai-sdk.dev/providers/ai-sdk-providers/anthropic
- LangChain models doc: https://docs.langchain.com/oss/python/langchain/models
- Aider config YAML: https://aider.chat/docs/config/aider_conf.html
- Aider OpenAI-compat: https://aider.chat/docs/llms/openai-compat.html
- Aider Anthropic: https://aider.chat/docs/llms/anthropic.html
- OpenAI Python SDK GitHub: https://github.com/openai/openai-python
- Anthropic Python SDK GitHub: https://github.com/anthropics/anthropic-sdk-python
