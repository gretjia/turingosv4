# PROVIDER_TAPE_FORMAT v1 (DRAFT — not yet active)

| Field | Value |
|-------|-------|
| Status | RESERVED — producer + consumer not yet shipped |
| Date drafted | 2026-05-21 |
| Trigger condition | (see §3) |
| Karpathy lens | This is a forward contract reservation, not an active feature |

## §1. Why this doc exists

Plan v4 user-sim Round 2 surfaced an Art. 0.2 reconstructability concern: the
field `GenerationAttemptCapsule.model_id` (`src/runtime/generation_attempt.rs`)
stores a bare model string like `deepseek-ai/DeepSeek-V3.2` which is ambiguous
when the same model name is hosted by multiple providers (SiliconFlow,
DeepSeek-direct, OpenRouter, etc.). A workspace exported and replayed on a
machine without the original provider configured cannot determine the original
provider from the tape alone.

Constitution-lens (this session, 2026-05-21) proposed shipping a
`provider:model_id` format on tape immediately. Karpathy-lens (this session,
2026-05-21) rejected this as fake-future-extensibility because no replay
consumer exists yet that would read the prefix.

This doc captures the **agreed-upon resolution**: reserve the future format,
ship neither producer nor consumer now, document the trigger.

## §2. The future format (when it ships)

`GenerationAttemptCapsule.model_id` value format MUST become:

```
<provider>:<model_id>
```

Where:
- `<provider>` is a stable lowercase identifier from the set:
  - `siliconflow` — https://api.siliconflow.cn/*
  - `deepseek` — https://api.deepseek.com/*
  - `openai` — https://api.openai.com/*
  - `anthropic` — https://api.anthropic.com/*
  - `openrouter` — https://openrouter.ai/*
  - Other providers added to this list before use
- `<model_id>` is the provider's canonical model string (e.g. `deepseek-v4-pro`,
  `claude-opus-4-1`, `gpt-5`, `Qwen/Qwen3-Coder-30B-A3B-Instruct`)

Backward compatibility: bare strings without `<provider>:` prefix are
interpreted as SiliconFlow (the pre-format-change default).

## §3. Trigger conditions (the producer-consumer pair)

This format ships when ALL of:

1. A replay consumer is being written that needs to differentiate providers
   (e.g. an offline replay tool, a Trust Root audit, a market settlement
   verifier that needs provenance).
2. The PR adding the consumer ALSO updates every `model_id` write path
   to emit the new format (cmd_spec, cmd_generate, cmd_llm `complete`,
   cmd_llm `triage`, cmd_llm `prompt-eval`).
3. A gate test asserts no write path emits a bare string after this PR
   merges (regex check: `^[a-z]+:.+`).

Until those three conditions are met, this doc is a contract reservation
only — no Rust code references this format.

## §4. What was NOT chosen

The Constitution proposal also considered:
- New `schema_id = "turingos-generation-attempt-v2"` (CAS schema bump) — rejected:
  invalidates existing capsules without migration; tail-additive value change is enough
- A new ObjectType variant — rejected: still in scope of `EvidenceCapsule`
- A new dedicated `ProviderProvenanceCapsule` schema — rejected: information
  belongs inline on the attempt capsule, not a separate cross-referenced one

## §5. Related FC nodes

- FC2-N18 (boot/replay reconstructability)
- FC1-N4 (LLM proposal externalization)
- Art. 0.2 — frozen tape must contain enough information to reconstruct cost / provenance

## §6. References

- Plan v4 (this session, 2026-05-21)
- USERSIM_ROUND2_DEEPSEEK_END_TO_END_2026-05-21.md — the user-sim that surfaced
  the provenance gap
- Constitution-lens proposal §3 (in this session's chat transcript)
- Karpathy-lens rebuttal §6 (this session)
