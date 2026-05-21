# B-C — Constitution-lens architectural proposal

| Field | Value |
|-------|-------|
| Date | 2026-05-21 |
| Phase | B (debate) — agent 1 of 2 |
| Agent | sonnet, general-purpose, read-only |
| Inputs | A1 + A2 + A3 research summaries; full read of constitution.md, CLAUDE.md, AGENTS.md, CONSTITUTION_EXECUTION_MATRIX.md, genesis_payload.toml, USERSIM_ROUND2 |
| Word count | ~2100 |
| Bias | Constitutional integrity > surface simplicity. Maximalist proposal designed for future-state correctness. |

> **Resolution**: This proposal was **rejected** by the orchestrator in favor of the Karpathy-lens minimum design (see `B_K_karpathy_lens_rebuttal.md` and `C_ORCHESTRATOR_SYNTHESIS.md`). Preserved here as the canonical "do it properly" path that activates when trigger conditions in `handover/specs/PROVIDER_TAPE_FORMAT_v1_DRAFT.md` §3 fire.

---

## §1. Constitutional anchors

**FC1 — externalized attempt count equality (CLAUDE.md §4; constitution.md FC1 flowchart ~line 455)**

The canonical invariant is:

```
evaluator_reported_completed_llm_calls
= tool_dist.step + tool_dist.parse_fail + tool_dist.llm_err
```

A `Protocol` enum does not change the count of externalized LLM-Lean cycles — one `chat_complete` call per branch regardless of which dispatch path fires. However, if a new Anthropic path introduces a separate streaming-error class that the current `AttemptOutcome` enum (`ParseFailed`, `LlmApiError`, `NoFilesParsed`, `InternalIo`) does not cover, that gap is a **FC1 violation** — the counter at `tool_dist.llm_err` must remain the exhaustive sink for all LLM-layer failures. The design must not add a silent Anthropic-specific failure path that bypasses `LlmApiError` accounting. This is a hard constraint, not a quality preference.

**FC2 — boot reconstructability (constitution.md Art. IV, ~line 515; Art. 0.2 ~line 52)**

Art. 0.2: "任意 cost / time / provenance / market price ... frozen tape 上必有充分信息可推导." `GenerationAttemptCapsule.model_id` (`src/runtime/generation_attempt.rs:27`) currently stores a bare string like `deepseek-ai/DeepSeek-V3.2`. If a workspace is exported and replayed on a machine without the original provider configured, the tape reader cannot determine whether to route the replay to SiliconFlow, DeepSeek Direct, or Anthropic. Provider ambiguity = reconstructability break = FC2 violation. The tape must carry `provider:model_id` as the canonical tape string.

`AttemptTelemetry.model_provider` (`src/runtime/attempt_telemetry.rs:358`) already exists as an `Option<String>` — but it is `#[serde(default)]` and `None` in current production paths that use the GRILL SiliconFlow route. This field must become non-optional and carry the provider string for every new attempt recorded after this atom.

**FC3 — Veto-AI verdict domain (constitution.md Art. V.1.3, ~line 740)**

The Veto-AI verdict domain is `{PASS, VETO}`. A protocol-dispatch failure (Anthropic-specific struct mismatch, missing `tool_result` routing) maps to `VIOLATION-FOUND <Art. 0.2 tape-reconstructability> <src/runtime/generation_attempt.rs:27>` in the clean-context Codex audit witness domain. It does not require a new verdict category. The protocol itself never mutates constitution semantics, so it cannot trigger `SECOND-SOURCE-DRIFT` unless the endpoint config itself becomes a source of truth (which this design prevents by keeping `turingos.toml` as a derived workspace config, not a canonical tape source).

**Art. V evidence-bearing (constitution.md Art. V ~line 664; CLAUDE.md §4)**

`GenerationAttemptCapsule.model_id` is written to CAS with `ObjectType::EvidenceCapsule` at `schema_id = "turingos-generation-attempt-v1"`. Because CAS objects are the canonical evidence anchor (Art. 0.2 point 2), and because the schema_id string contains `-v1`, a field-shape change requires either a new `schema_id` (`-v2`) or a backward-compatible tail-additive field. The design chooses the latter for the provider field (see §3).

**§3.1 forbidden surfaces (AGENTS.md §6)**

The surfaces this work approaches but must not cross:

- `src/bus.rs` — not touched. Provider config lives above the bus layer.
- `src/state/sequencer.rs` — not touched. No admission-rule change.
- `src/state/typed_tx.rs` — not touched. No `WorkTx` wire-schema change.
- `src/bottom_white/cas/schema.rs` — AMBER: `ObjectType` enum may need a new variant if a protocol-level capsule is added. This design avoids adding a new `ObjectType` — the `EvidenceCapsule` type already covers `GenerationAttemptCapsule`. No Class 4 surface crossed.
- `genesis_payload.toml` `[trust_root]` — `Cargo.lock` and `Cargo.toml` will rehash (Cz cycle 3). `src/bin/turingos/siliconflow_client.rs` is NOT in the Trust Root manifest currently; its rehash is not a boot-abort risk. `src/drivers/llm_http.rs` IS pinned (`genesis_payload.toml:147`); if it is touched, the pin must be updated. This work does NOT propose touching `src/drivers/llm_http.rs` — that is the mini-F2F evaluator LLM path, not the CLI LLM path.

## §2. Proposed architecture

### Config surface (turingos.toml per-role)

Extend the existing `[llm.meta]` / `[llm.blackbox]` TOML sections with two new optional keys:

```toml
[llm.meta]
model     = "deepseek-ai/DeepSeek-V3.2"     # existing
api_key_env = "SILICONFLOW_API_KEY"          # existing
endpoint  = ""                               # NEW: empty = use provider default
protocol  = "openai_compat"                  # NEW: "openai_compat" | "anthropic"

[llm.blackbox]
model     = "Qwen/Qwen3-Coder-30B-A3B-Instruct"
api_key_env = "SILICONFLOW_API_KEY"
endpoint  = ""
protocol  = "openai_compat"
```

`endpoint = ""` means the code resolves via the existing `TURINGOS_SILICONFLOW_ENDPOINT` env-var fallback, then the hardcoded default. This preserves the existing three-tier resolution: (1) `turingos.toml` endpoint key, (2) `TURINGOS_SILICONFLOW_ENDPOINT` env var, (3) `SILICONFLOW_ENDPOINT` constant.

`protocol` is the new axis. Default is `"openai_compat"` so no existing workspace breaks.

### Wire dispatch: Protocol enum with 2 paths

```rust
enum Protocol { OpenAiCompat, Anthropic }
```

**Why NOT one struct with optionals**: The Anthropic Messages API diverges at five structural axes identified in A2 (tool result routing, system prompt shape, thinking content, streaming event model, cache_control). A single struct with `Option<anthropic_system>`, `Option<openai_system>`, `Option<tool_result_content_block>` etc. produces a shape where the dead-branch fields are serialized as null and silently discarded by one provider or the other. This is the definition of a shadow ledger anti-pattern: the actual wire shape does not match the model. Two separate dispatch paths — one that builds a `ChatRequest` (existing struct in `siliconflow_client.rs`) and one that builds an Anthropic `MessagesRequest` (new struct) — are explicit about which shape is live at any given call.

**Why NOT OpenRouter proxy**: OpenRouter as a router introduces (a) a new singleton process between Rust and providers, (b) loss of `cache_control: ephemeral` headers (they are provider-specific HTTP knobs that OpenRouter normalizes away), and (c) replay ambiguity: the `GenerationAttemptCapsule.model_id` field would contain `openrouter/anthropic/claude-opus-4-1` which cannot be replayed without OpenRouter being live. This violates FC2 reconstructability.

**Dispatch logic** (in `siliconflow_client.rs` or a renamed `llm_client.rs`): read `Protocol` from config at call site, branch to `dispatch_openai_compat(...)` or `dispatch_anthropic(...)`. Both return the same `ChatResult` type. The `reasoning_content` field already exists in `ChatResult`; Anthropic thinking blocks map to it.

### Tape canonical IDs

`GenerationAttemptCapsule.model_id` MUST store `provider:model_id` as its canonical string, e.g. `siliconflow:deepseek-ai/DeepSeek-V3.2` or `anthropic:claude-opus-4-1`. This is the only change needed to satisfy FC2 reconstructability for offline replay — a replay agent reading the tape knows exactly which provider and which model was used, and can fail cleanly if that provider is not configured rather than silently routing to the wrong endpoint.

The `provider` prefix is a short lowercase label, not a URL. The mapping from provider label to endpoint URL lives in config, not on tape — the tape carries the stable identifier, not a mutable URL.

### Backward compatibility tier

1. **turingos.toml `endpoint` key** (new): when present and non-empty, overrides env var and constant.
2. **`TURINGOS_SILICONFLOW_ENDPOINT` env var** (existing): preserved as tier-2 fallback. Not deprecated in this atom; deprecation is a forward concern once the toml endpoint key has adoption.
3. **`SILICONFLOW_ENDPOINT` constant** (existing, `siliconflow_client.rs:21`): remains as tier-3 default. The current hardcoded URL at line 21 is not a "forbidden hardcoded endpoint" — it is the legitimate default constant for a named provider. The violation is if it were used as the single fallback with no override path, which it is not. What must be removed is any call site that bypasses the three-tier resolution and calls the constant directly.

### Trust Root churn (Cz cycle 3 scope)

Files that will rehash:

- `Cargo.lock` — rehashes if any new dep lands (e.g. no new dep if Anthropic path is pure reqwest; one new dep if `serde_json` shape divergence requires the `anthropic-sdk-rs` crate — the design recommends NOT using the SDK, to keep deps minimal; pure reqwest + manual JSON).
- `Cargo.toml` — rehashes if any dep is added.
- `src/bin/turingos/siliconflow_client.rs` — NOT in Trust Root, no boot impact.
- `src/runtime/generation_attempt.rs` — NOT in Trust Root.

Files that must NOT rehash: `src/kernel.rs`, `src/bus.rs`, `src/wal.rs`, `src/state/sequencer.rs`, `src/state/typed_tx.rs`, `src/bottom_white/cas/schema.rs`, `src/drivers/llm_http.rs`.

**Class rating**:

- Atom 1 (config surface extension + endpoint/protocol keys in turingos.toml): **Class 1** — additive, no production wire change.
- Atom 2 (Protocol enum + OpenAiCompat dispatch refactor): **Class 2** — production wire-up; clean-context Codex audit required.
- Atom 3 (Anthropic dispatch path): **Class 2** — new production wire-up.
- Atom 4 (`model_id` → `provider:model_id` tape format + `AttemptTelemetry.model_provider` non-optional): **Class 2** — production evidence schema change; clean-context Codex audit required.
- Atom 5 (Trust Root rehash + Cz cycle 3 genesis_payload.toml update): **Class 2** (manifest bookkeeping only, no new logic).

No atom in this set is Class 3. The `GenerationAttemptCapsule` schema change is Class 2 because it adds a tape field but does not change admission rules, money paths, or CAS integrity predicates. §8 sign-off is not required for any atom in this set.

## §3. What gets ADDED to canonical evidence

### `GenerationAttemptCapsule` schema: NO version bump

The current `schema_id` is `"turingos-generation-attempt-v1"`. The `model_id` field already exists at line 27. The change is: the **value** stored in that field becomes `provider:model_id` format instead of bare `model_id`. This is a semantic content change, not a structural shape change. A reader that expects the old bare string will still parse the JSON correctly; the value just contains a colon. The `schema_id` string pin (`GENERATION_ATTEMPT_CAPSULE_SCHEMA_ID`) does not change.

This is the correct call. Bumping to `-v2` would invalidate all existing CAS objects without providing a migration path, which violates "no retroactive evidence rewrite" (AGENTS.md §8 + MEMORY.md). Tail-additive approach: the field already exists; its value format changes going forward. Old values without the prefix remain interpretable (bare string = SiliconFlow-default for pre-migration records).

### `AttemptTelemetry.model_provider`: promote from `Option<String>` to populated-required

`model_provider` at `src/runtime/attempt_telemetry.rs:358` is currently `Option<String>` with `#[serde(default)]`. After this change, every new attempt record emitted by the CLI LLM path must populate this field. The field remains `Option<String>` in the Rust type (to preserve backward deserialization of old records), but the write path asserts `is_some()` before writing. This satisfies Art. 0.2 provenance requirement without a schema bump.

### `ArtifactBundle`: NO new `provider_origin` field

`ArtifactBundle` records which files were generated and their CIDs, anchored to `spec_capsule_cid` and the `model_id` of the generation session. It does NOT need a `provider_origin` field because: (a) provider provenance is already captured in `GenerationAttemptCapsule.model_id` (which this design fixes to `provider:model_id`), and (b) `ArtifactBundle` is a materialized view of the generation result — Art. 0.2 allows derived views to omit fields that are reconstructable from their upstream CAS anchor. A replay agent follows `GenerationAttemptCapsule` → reads `model_id` (now carrying provider) → knows the origin.

### `PromptPromotionReceipt` (C10): NO protocol awareness required

`PromptPromotionReceipt` (`src/runtime/prompt_promotion.rs:36`) records `from_prompt_cid`, `to_prompt_cid`, `eval_set_cid`, `eval_before_cid`, `eval_after_cid`, and `promotion_decision`. It is a gate on prompt content, not on model identity or provider routing. A promotion receipt validates that a specific prompt CID is safe to use for evaluation — the provider used during evaluation is captured in the `eval_before_cid` and `eval_after_cid` CAS objects (which are `GenerationAttemptCapsule` records that carry the provider in `model_id`). Adding `provider` to the receipt itself would create a new axis of promotion gatekeeping that is not constitutionally required and would break the current C10 promotion flow without evidence that provider-switching is a promotion-worthy event.

## §4. What the design refuses

**OpenRouter-as-router**: Rejected. OpenRouter normalizes provider-specific headers (Anthropic `cache_control: ephemeral` is the concrete example). Cache observability matters for PPUT-CCL cost accounting (genesis_payload.toml `[pput_accounting_0]` cost_definition = "sum(prompt_tokens + completion_tokens + tool_tokens)") — if the proxy silently drops caching signals, cost provenance diverges from actuality, which is a tape canonical violation under Art. 0.2. Additionally, replay requires OpenRouter to be live, violating FC2 reconstructability.

**Single generic `LLM_API_KEY`**: Rejected. The existing design already uses per-role `api_key_env` pointing to named env vars (`SILICONFLOW_API_KEY`, and potentially `ANTHROPIC_API_KEY`). A single `LLM_API_KEY` violates "no global mutable secret" (CLAUDE.md §4 forbidden patterns) and makes attempt provenance ambiguous on tape — the `model_provider` field would have no reliable correspondent. The per-provider env-var pattern from A1 industry practice is constitutionally correct here.

**Hardcoded endpoint URL in source**: `siliconflow_client.rs:21` (`const SILICONFLOW_ENDPOINT: &str = "https://api.siliconflow.cn/v1/chat/completions"`) is a named-provider constant, not a violation in isolation. The violation pattern is if a call site bypasses the three-tier resolution and calls this constant directly without the env-var check. The `endpoint()` function at line 34 correctly layers env-var over constant. Any Anthropic dispatch path must follow the same pattern: a named constant for the default endpoint, overridable via `turingos.toml` → env var → constant, never hardcoded at a call site.

## §5. Migration path (atom decomposition)

| Atom | Class | Files touched | What it does | §8 required? |
|------|-------|--------------|--------------|--------------|
| 1 | 1 | `cmd_llm.rs`, turingos.toml write/read helpers | Add `endpoint` and `protocol` keys to per-role TOML section; `turingos llm config` writes them; readers return defaults when absent | No |
| 2 | 2 | `siliconflow_client.rs` (rename or extend) | Introduce `Protocol` enum; refactor `chat_complete`/`chat_complete_blocking` to thread `Protocol` + resolved endpoint through a `LlmConfig` struct; existing OpenAI-compat path is unchanged in behavior | No (Codex witness required post-ship) |
| 3 | 2 | New `anthropic_client.rs` (or `dispatch_anthropic` in same file), `cmd_llm.rs`, `cmd_generate.rs`, `cmd_spec.rs` | Implement Anthropic Messages API dispatch path with native `system` field + `tool_result` routing; wire through `LlmConfig.protocol` branch; add mock server for test | No (Codex witness required post-ship) |
| 4 | 2 | `src/runtime/generation_attempt.rs`, `src/runtime/attempt_telemetry.rs`, all `write_generation_attempt_capsule` call sites | Change `model_id` value format to `provider:model_id`; enforce `model_provider` population on all new write paths; update tests that assert on bare model string | No (Codex witness required post-ship) |
| 5 | 2 | `genesis_payload.toml`, `Cargo.lock`, `Cargo.toml` | Rehash changed files into `[trust_root]`; add comment noting Cz cycle 3; run `boot::verify_trust_root` to confirm no abort | No |

No atom touches `src/kernel.rs`, `src/bus.rs`, `src/wal.rs`, `src/state/sequencer.rs`, `src/state/typed_tx.rs`, `src/bottom_white/cas/schema.rs`, or `src/drivers/llm_http.rs`. No §8 sign-off is required.

## §6. Risks and open questions

**R1 — Anthropic `thinking` content block vs `reasoning_content` field ambiguity.**
The current `ChatResult.reasoning_content: Option<String>` (`siliconflow_client.rs:145`) is populated from `message.reasoning_content` (DeepSeek's sibling JSON field). Anthropic thinking blocks are typed content blocks at `choices[0].message.content[].type == "thinking"`. These are structurally different. If Atom 3 reuses `ChatResult.reasoning_content` for both, the field semantics become provider-specific, which is acceptable only if consumers treat it as "opaque reasoning trace" regardless of source. Confirm with the caller in `cmd_spec.rs` (`strip_think_blocks` usage) that the content-block form can be pre-extracted and assigned to the same field before returning. Resolution: prototype Anthropic message struct with explicit `extract_reasoning_from_content_blocks()` before populating `ChatResult`.

**R2 — `turingos.toml` endpoint field migration for existing workspaces.**
Existing workspaces have no `endpoint` key in their `turingos.toml`. The design relies on the absence returning a default. Confirm `read_config_value` in `cmd_llm.rs` returns `None`/empty (not an error) for missing keys. Currently `read_meta_api_key_env` returns `Err(LlmConfigError)` for missing key. The endpoint key must use a different read pattern — `Option<String>` with `None` meaning "use env var fallback", not an error. This edge case must be exercised in Atom 1 unit tests before Atom 2 dispatch depends on it.

**R3 — `provider:model_id` format contract for replayers not yet built.**
The tape will carry `siliconflow:deepseek-ai/DeepSeek-V3.2` as of Atom 4. There is no replay consumer yet that parses the provider prefix and routes accordingly. The format change is forward-only correct: old records (bare string) remain readable, new records carry the prefix. But until a replay consumer exists that acts on the prefix, the tape field is latent evidence — correct but unexercised. Resolution: this is acceptable per the Art. 0.2 "必有充分信息可推导" requirement — the information must be on tape; the consumer that uses it is a forward atom. Document the `provider:model_id` format contract in a `DESIGN_PROVIDER_FORMAT.md` note in `handover/specs/` so the replay consumer has an unambiguous spec to implement against.

**R4 — Anthropic API key env var naming.**
The existing pattern uses `SILICONFLOW_API_KEY` (provider-specific). The correct name for Anthropic is `ANTHROPIC_API_KEY` (which aligns with industry practice from A1). But `cmd_llm.rs` currently defaults to `SILICONFLOW_API_KEY` as the single `api_key_env` if none is configured. After Atom 1, the `api_key_env` field in `turingos.toml` is set per-role at init time. The `turingos llm config` command must be updated to emit `ANTHROPIC_API_KEY` when `protocol = "anthropic"` is selected, not `SILICONFLOW_API_KEY`. If this is not done, the UX hint (`export {api_key_env}=sk-...`) will be wrong. Atom 1 must include this conditional default logic.

**R5 — Cz cycle 3 Trust Root rehash race condition.**
`Cargo.lock` is currently pinned (`genesis_payload.toml:135`). If Atom 3 adds no new crates (pure reqwest + manual JSON for Anthropic), `Cargo.lock` does not change and no rehash is needed. If a new crate is added (e.g. for streaming or multipart), `Cargo.lock` rehashes and Atom 5 is correct. The decision point is Atom 3 implementation: if the implementer is tempted to pull in `aws-sigv4` or `anthropic-sdk-rs` for convenience, this bloats the Trust Root churn scope unnecessarily. Resolution: mandate zero new crates for Atom 3 — implement Anthropic dispatch as pure HTTP with manual JSON structs, exactly as SiliconFlow is today. This keeps Cz cycle 3 scope to Cargo.lock/Cargo.toml only, contingent on whether those files actually change.

---

*Successor (Karpathy lens) should probe: R1 content-block extraction complexity, R5 zero-new-crates constraint, and the `provider:model_id` format lock-in before a replay consumer exists.*
