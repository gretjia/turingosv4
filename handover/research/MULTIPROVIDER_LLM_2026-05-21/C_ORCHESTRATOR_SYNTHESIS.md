# C — Orchestrator synthesis & binding decision

| Field | Value |
|-------|-------|
| Date | 2026-05-21 |
| Phase | C (synthesis) |
| Orchestrator | Claude opus 4.7 |
| Authority | User-delegated 2026-05-21 ("缺什么你补什么，找到问题修复问题是你的责任") |
| Status | BINDING — implemented as PR #70 (`830f5661`) |

## Decision

**Karpathy lens wins this round.** Ship Atom-K + Atom-D + Atom-S (the tape-format design contract). Reject Constitution's 5-atom plan in its full form; preserve its content as the activation-ready maximalist design for when trigger conditions fire.

## Where Karpathy won (and why)

| Issue | Winner | Reason |
|-------|--------|--------|
| 5-atom vs 2-atom decomposition | **Karpathy** | Constitution R3 self-acknowledges `provider:model_id` on tape has no consumer = textbook fake-future-extensibility violation per KARPATHY_ARCHITECT.md MetaAI Checklist |
| Protocol enum now | **Karpathy** | Single-variant enum is SIMPLE_CODE.md anti-pattern. Anthropic path has zero production users |
| 3rd-tier endpoint fallback (toml + env + const) | **Karpathy** | Already 2 sources for one value; adding a 3rd worsens the "single source of truth" violation, doesn't fix it |
| Provider-aware env var naming | **Constitution** (but no new code needed) | A1 industry practice + "no global mutable secret" agree; **existing `--meta-api-key-env` / `--blackbox-api-key-env` already deliver this** |
| Anthropic dispatch path now | **Karpathy** | ~500 LoC for 0 production users. Trigger: "first user PR with `provider=anthropic` hitting OpenAI-compat rejection." Not now. |
| `provider:model_id` tape format now | **Karpathy** | Producer ships without consumer = schema debt with no enforcer. Karpathy predicted (deterministically) that 1+ write paths slip with bare strings within first sprint. Wait for replay consumer. |
| Cz cycle 3 Trust Root rehash now | **Karpathy** | Conditional on Atom 3 crate addition that we're deferring → vanishes. |
| `ChatResult.reasoning_content` cross-protocol fit | **Karpathy** | Constitution's R1 admits Anthropic content blocks structurally won't fit `Option<String>`. Atom 3 will grow `content_blocks: Vec<ContentBlock>` field. Certainty not risk. |

Karpathy cited 6 explicit Simple Code / Architect skill rule violations with direct quotes from `skills/KARPATHY_*.md`. Constitution cited 5 self-acknowledged risks (R1-R5) — using a risk register as design defense is itself a Karpathy anti-pattern.

## Where Constitution kept ground (concessions Karpathy granted)

1. **Tape provenance is a real Art. 0.2 concern.** `model_id = "deepseek-v4-pro"` is ambiguous when multiple providers host the same model name. The disagreement is timing: write the field when the replay consumer is written, so the invariant has a gate. → Captured in `handover/specs/PROVIDER_TAPE_FORMAT_v1_DRAFT.md` as a reserved contract.

2. **Zero new crates for any future Anthropic path.** Adding `anthropic-sdk-rs` would duplicate reqwest + serde infrastructure that already exists. → Codified in the spec doc §3.

3. **Provider-aware env var naming is correct.** Industry practice + constitutional alignment. → Already shipped via `--meta-api-key-env` / `--blackbox-api-key-env` flags (PR #61 / OBS-R022 dual-key patch).

## What shipped (PR #70, `830f5661`)

### Atom-K (Class 1): NB3 fix

`src/bin/turingos/cmd_welcome.rs` `check_endpoint_not_default()` — prints `⚠ TURINGOS_SILICONFLOW_ENDPOINT overridden` warning when env var differs from `SILICONFLOW_ENDPOINT` constant. Closes the silent-misconfig trap that user-sim Round 2 flagged HIGH severity.

Test: `tests/welcome_surfaces_endpoint_override.rs` covers both branches.

LoC: 14 source + 128 test.

### Atom-D (Class 0): NB6 fix

`src/bin/turingos/cmd_llm.rs` FULL_HELP — adds ANTHROPIC + OPENAI dual-key examples alongside the existing DEEPSEEK example. `src/bin/turingos/cmd_generate.rs` FULL_HELP — adds ENVIRONMENT section listing the 3 required env vars.

LoC: 43 across both files (help text only).

### Atom-S (Class 0): Constitution concession

`handover/specs/PROVIDER_TAPE_FORMAT_v1_DRAFT.md` — reserves the `provider:model_id` tape format with explicit producer + consumer trigger conditions. No Rust code references the spec. Ships when the first replay consumer needs it.

Lines: 87.

## Triggers that would reopen this debate

(See also `handover/specs/PROVIDER_TAPE_FORMAT_v1_DRAFT.md` §3.)

1. **Anthropic native dispatch needed** — A user PR adds `[llm.meta] provider = "anthropic"` to a real workspace and hits a wire-format rejection from the OpenAI-compat path. Activate Constitution Atom 3 (~500 LoC).
2. **Replay consumer needs provider discrimination** — A new offline replay tool / Trust Root audit / market settlement verifier needs to differentiate providers from `model_id` alone. Activate Constitution Atom 4 + emit gate test asserting `^[a-z]+:.+` regex on all `model_id` writes.
3. **Second protocol shipped & unit-tested** — A real production protocol (not speculative) requires its own dispatch path. Activate Constitution Atom 2 (`Protocol` enum).
4. **`TURINGOS_SILICONFLOW_ENDPOINT` env var becomes documented support burden** — Multiple user reports show the env var is confusing. Activate Constitution Atom 1 (move endpoint into `turingos.toml` per-role with explicit deprecation of env var).

Until any trigger fires, PR #70's minimum design is the canonical answer.

## What this archive preserves

- The 3 research outputs (A1, A2, A3) — re-usable raw findings on industry practice, protocol divergence, and v4 code touch-surface.
- Both lens proposals (B-C maximalist, B-K minimalist) — both are valid reads of the same evidence; the choice between them is contextual to current trigger state.
- This synthesis — binding for 2026-05-21 decision context; future debates should reopen the trigger conditions in §6 above rather than relitigate the architectural choice from scratch.

## Costs avoided by this decision

| If we had taken Constitution's 5-atom path | Avoided |
|--------------------------------------------|---------|
| Source LoC | ~260 |
| Test LoC | ~500 |
| New PRs | 4 additional |
| Sub-agent dispatches | ~6 (5 sonnet + 1 Codex audit per Class-3 chain) |
| Cz Trust Root cycle 3 rehash | Yes (Cargo.lock + Cargo.toml + multiple binary source rehashes) |
| `ChatResult` shape divergence between providers | Deferred — current `Option<String> reasoning_content` continues to work for the OpenAI-compat universe |
| Schema debt (tape field with no reader) | Avoided |

| What we paid instead (PR #70) | Cost |
|-------------------------------|------|
| Source LoC | 27 |
| Test LoC | 128 |
| Spec doc lines | 87 |
| PRs | 1 |
| Sub-agent dispatches | 1 sonnet (implementation) + Round 3 user-sim validation |
| Trust Root churn | None |
