# LLM Boundary Inventory — TB-SOFTWARE-3-0 Atom S4.2

**Date**: 2026-05-23
**Risk class**: 0 (docs)
**Charter**: `handover/tracer_bullets/TB-SOFTWARE-3-0_charter_2026-05-23.md`
**§8 directive**: `handover/directives/2026-05-23_SOFTWARE_3_0_CONSOLIDATION_DIRECTIVE_AND_§8.md`
**Atom**: S4.2

## Purpose

This document inventories every place in the codebase that crosses the LLM
boundary (i.e. calls an external chat-completion API). It is the input for
any future Class 3/4 packet that introduces a `ChatProvider` enum, a
`ModelCallReceipt` runtime module, or any provider abstraction layer.

Per Karpathy K10 (`feedback_defer_abstraction_until_second_impl`):
abstraction layers are intentionally NOT introduced in this package. They
wait until a 2nd concrete provider (e.g. VolcEngine, OpenAI direct) lands.
This inventory is the evidence base for that future packet.

## 1. Module rename (S4.1)

| Before | After |
|--------|-------|
| `src/bin/turingos/siliconflow_client.rs` | `src/bin/turingos/chat_client.rs` |
| `mod siliconflow_client;` in `src/bin/turingos.rs` | `mod chat_client;` |
| `crate::siliconflow_client::*` (in all cmd_*.rs) | `crate::chat_client::*` |

Function names (`chat_complete`, `chat_complete_blocking`, `require_api_key`,
`ChatMessage`, `ChatResult`, `Usage`, `ThinkingConfig`, `LlmError`) were
already provider-neutral; no function renames were required.

### Preserved (intentionally not renamed)

| Name | Reason |
|------|--------|
| `SILICONFLOW_ENDPOINT` constant | Describes the default API URL, which IS provider-specific. Future providers declare their own endpoint constants. |
| `TURINGOS_SILICONFLOW_ENDPOINT` env var | Observable in user shells; renaming would break existing user configs without functional benefit. Deprecated alias is fine if/when a generic name is introduced later. |

## 2. LLM call sites (post-S4.1)

Source files calling `chat_complete*` (counted on `feature/sw3-s4-1-chat-client-rename`):

| File | `chat_complete*` calls | Notes |
|------|-----------------------|-------|
| `src/bin/turingos/cmd_llm.rs` | 5 | `turingos llm complete` (raw), `turingos llm prompt-eval` (grill driver), `turingos llm thinking` (DeepSeek reasoner control), shared retry helper, role-dispatch wrapper. |
| `src/bin/turingos/cmd_tdma.rs` | 5 | TDMA-Bounded runner — chat closure adapter passed to `kernel_step`; per-attempt retry / parse_fail / llm_err telemetry. |
| `src/bin/turingos/cmd_spec.rs` | 3 | Grill driver: open-question, follow-up, finalize. |
| `src/bin/turingos/cmd_generate.rs` | 2 | Artifact codegen (HTML/text) — passes `ThinkingConfig` blackbox + meta model splits. |
| `src/bin/turingos/cmd_wizard.rs` | 2 | First-run interactive wizard prompts (Class 0). |
| `src/bin/turingos/cmd_init.rs` | 0 | Uses `DEFAULT_BLACKBOX_MODEL` / `DEFAULT_META_MODEL` constants only (no LLM call). |
| `src/bin/turingos/cmd_welcome.rs` | 0 | Uses `endpoint()` + `SILICONFLOW_ENDPOINT` for the smoke-probe URL only (no chat call). |

**Total chat_complete* sites: 17** (across 5 cmd files).

All sites resolve `api_key` through `crate::chat_client::require_api_key`,
which centralizes the env-var lookup. None of the cmd files build the HTTP
request body directly.

## 3. Prompt-guard coverage (current)

| Surface | Hard guard exists? | Evidence |
|---------|--------------------|----------|
| Raw Lean stderr | yes — `shielded_diagnostic` wrapper in tdma_runner | grep `shielded_diagnostic` in src/ |
| Raw autopsy logs | yes — `autopsy_log` is internal-only; agent prompts see `agent_message` only | src/sdk/agent_message_view.rs |
| Private CAS sidecar diagnostics | yes — derived views (`BuildSessionView`, `GrillSessionSnapshot`) intentionally do not surface them | src/runtime/build_session_view.rs (S3 boundary), src/web/session_snapshot.rs (S2 boundary) |
| TestScenarioSet CID | yes — hidden oracle; `BuildSessionView` exposes `accepted_delivery: bool` only | src/runtime/build_session_view.rs |
| Benchmark leaks (MiniF2F, h-VPPUT) | yes — separate runner crates; `cmd_generate` does not import benchmark fixtures | constitutional matrix gates 2026-05-07 |

### Gaps (intentionally deferred to a future Class 3/4 packet)

- No central `ModelCallReceipt` capsule that captures `{provider, model_id,
  prompt_hash, request_canonical_bytes, response_raw, usage, latency_ms,
  request_ts}` — currently distributed across per-attempt `r2_write_attempt_telemetry`.
- No `ChatProvider` enum gating which provider is allowed for which surface
  (e.g. "blackbox model must be reasoner-class").
- No prompt-fingerprint registry — `prompt_hash` is per-attempt; cross-attempt
  uniqueness invariants are checked at runner level, not at call-site.

These are deliberate `Defer` items per Karpathy K10. They will be addressed
by a separate Class 3/4 packet AFTER a 2nd concrete provider implementation
exists, so the abstraction can be informed by two real call paths instead
of one (premature abstraction).

## 4. Existing evidence fields per call

Every `chat_complete_blocking` outcome is observed via these telemetry
fields (written at the runner layer, not at chat_client itself):

- `prompt_hash` — SHA-256 of canonical request bytes (`canonical_chat_request_bytes`)
- `raw_response_body` — full HTTP response body bytes (for replay)
- `raw_response_body_cid` — CAS CID of raw_response_body
- `usage_total_tokens` — sum of prompt + completion tokens
- `model_id` — concrete model name from the provider
- `retry_index` — 0-based; cap is set by per-runner config
- `llm_err.kind` — error taxonomy when `LlmError` is surfaced (Transport,
  HttpStatus, Decode, Schema)

Provider-side fields not yet captured per call (future inventory targets):

- request latency wall-clock (ms)
- request_ts (Unix epoch, monotonic per-runtime)
- effective endpoint URL at call time (in case it differs from the default)

These are stretch items for the future abstraction packet; not blocking
for S4.1's rename.

## 5. Deferral note (for the future Class 3/4 abstraction packet)

When the 2nd concrete provider lands (e.g. VolcEngine), the future packet
should:

1. **Introduce `ChatProvider` enum** with variants `SiliconFlow`, `VolcEngine`,
   etc. — gated by capability ("Allowed for blackbox model role?", "Supports
   thinking mode?"). Karpathy K10 (sum-type, not trait + single-impl).
2. **Introduce `ModelCallReceipt` capsule** with the canonical fields above.
   Class 3 (touches CAS evidence binding).
3. **Migrate all 17 chat_complete* sites** to take a `ChatProvider` parameter
   instead of relying on env-var endpoint lookup.
4. **Re-audit prompt guards** for each provider, since some providers have
   different system-prompt handling or thinking-mode behavior.
5. **Keep `chat_client.rs` as the file name** — the rename in S4.1 means the
   abstraction layer can land without another file rename.

Until that packet lands, callers continue to use `crate::chat_client::*`
helpers as a single shared boundary.

## 6. Cross-references

- TB charter: `handover/tracer_bullets/TB-SOFTWARE-3-0_charter_2026-05-23.md`
- §8 directive: `handover/directives/2026-05-23_SOFTWARE_3_0_CONSOLIDATION_DIRECTIVE_AND_§8.md`
- K10 (defer abstraction): `~/.claude/projects/-home-zephryj-projects-turingosv4/memory/feedback_defer_abstraction_until_second_impl.md`
- Conservative error semantics: `~/.claude/projects/-home-zephryj-projects-turingosv4/memory/feedback_conservative_error_semantics.md`

TRACE_MATRIX: FC1 (LLM boundary inside runtime loop), FC2-N16 (call-site
inventory feeds future derived-view layer), FC3 (CAS evidence binding for
`ModelCallReceipt` is the future packet's load-bearing surface).
