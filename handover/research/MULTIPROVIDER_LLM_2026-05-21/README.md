# Multi-provider LLM Research Archive — 2026-05-21

| Field | Value |
|-------|-------|
| Date | 2026-05-21 |
| Triggered by | User-sim Round 2 NB3 finding (`TURINGOS_SILICONFLOW_ENDPOINT` silent misconfig trap) + user architectural question about supporting non-SiliconFlow providers |
| Phase | A (research × 3) + B (debate × 2) + C (orchestrator synthesis) |
| Decision outcome | Karpathy-lens minimum design adopted (Atom-K + Atom-D + Atom-S, ~45 LoC) — Constitution-lens 5-atom plan (~760 LoC) deferred to future trigger |
| Shipped from this research | PR #70 (`830f5661`) — endpoint warning + multi-provider help text + tape format reservation spec |
| Open trigger conditions | See `handover/specs/PROVIDER_TAPE_FORMAT_v1_DRAFT.md` §3 |

## Why this archive exists

Each future TuringOS architectural debate about multi-provider LLM topology (Anthropic native dispatch, OpenRouter routing, per-role endpoint, provider-prefixed tape format, etc.) should **read this archive first** before re-running research agents. The cost was 3 web-research dispatches + 2 lens-debate dispatches; preserve the findings.

## Files in this archive

| File | What it covers | When to re-read |
|------|----------------|-----------------|
| `A1_industry_best_practices.md` | Comparative survey of OpenRouter / LiteLLM / Vercel AI SDK / LangChain / Aider / OpenAI+Anthropic SDKs. 5 converged patterns + 5 diverged patterns + 5 anti-patterns. | Before re-debating how multi-provider should be exposed. |
| `A2_protocol_differences_anthropic_vs_openai.md` | Wire-level diff between Anthropic Messages API and OpenAI Chat Completions across 10 axes. Verdict: cannot cleanly unify behind thin adapter. Refactor cost native Anthropic ~500 LoC non-streaming. | Before any attempt to add Anthropic native dispatch. |
| `A3_v4_refactor_cost_analysis.md` | Code-grep level mapping of every `chat_complete*` call site, env-var reader, mock-LLM test, Trust Root pin that would churn. ~760 total LoC. | Before scoping any multi-provider refactor PR. |
| `B_C_constitution_lens_proposal.md` | The "do it properly for the future" lens: 5-atom plan, Protocol enum + LlmConfig struct, `provider:model_id` tape format, etc. Includes 5 self-acknowledged risks (R1-R5). | Reference for the maximalist architectural answer if the trigger conditions in `PROVIDER_TAPE_FORMAT_v1_DRAFT.md` fire. |
| `B_K_karpathy_lens_rebuttal.md` | The "ship the smallest correct thing today" lens: 6 specific Simple Code / Architect skill violations cited from `skills/KARPATHY_*.md`, fake-future-extensibility ledger, ≤2-atom counter-design, predictions about R1-R5. | Reference for why the maximalist plan was rejected; check before re-proposing it. |
| `C_ORCHESTRATOR_SYNTHESIS.md` | Final arbitration. Karpathy won most points; 3 concessions to Constitution kept; explicit trigger conditions documented. | Read for the binding decision; future debate must reopen these triggers, not relitigate the 2026-05-21 verdict. |

## Triggers that would reopen this debate

Per `handover/specs/PROVIDER_TAPE_FORMAT_v1_DRAFT.md` §3 + this archive:

1. A user submits a PR with `[llm.meta] provider = "anthropic"` and the OpenAI-compat path rejects their request → activates Anthropic native dispatch (Constitution Atom 3 ≈ ~500 LoC).
2. A replay tool / Trust Root audit / market settlement verifier needs to differentiate providers from `model_id` alone → activates `provider:model_id` tape format (Constitution Atom 4).
3. A second wire protocol is implemented and unit-tested → activates `Protocol` enum (Constitution Atom 2; defer until 2nd protocol is in production).
4. `TURINGOS_SILICONFLOW_ENDPOINT` env var becomes a documented support burden → activates endpoint migration into `turingos.toml` per-role.

Until any trigger fires, the Karpathy minimum design (PR #70 `830f5661`) is the canonical answer.

## How to use this archive on a future session

1. Read this README.
2. Skim `C_ORCHESTRATOR_SYNTHESIS.md` for the binding decision.
3. If your current question maps to one of the 4 trigger conditions above: read the matching section of `B_C_constitution_lens_proposal.md` for the maximalist path.
4. Otherwise: read `B_K_karpathy_lens_rebuttal.md` §3 (minimum design) and `§4` (deferred-with-triggers ledger).
5. Only re-run new research agents if the **industry landscape has changed** (e.g. OpenAI shipped a new protocol, Anthropic deprecated Messages, OpenRouter went bankrupt). Otherwise the A1/A2/A3 findings remain current.
