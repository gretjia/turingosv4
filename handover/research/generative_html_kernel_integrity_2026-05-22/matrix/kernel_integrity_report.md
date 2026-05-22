# Kernel Integrity Report — Generative HTML Probe 2026-05-22

## Kernel Surface Coverage

### FC1: Runtime loop (spec → generate → verify → capsule chain)

| Problem | FC1 Node | Finding |
|---------|----------|---------|
| P01 XSS | FC1-N5 shielding | XSS passes validation; stored raw in spec.md appendix. Artifact sanitization unverifiable (generate blocked by 401) |
| P02 length | FC1-N5 trust boundary | Validation correct: 4096 passes, 4097 rejects on both spec/submit and spec/turn paths |
| P04 network | FC1-generate/verify | Verifier (MinimumBar/GameShape) does NOT detect network-dependent code (fetch/XHR). Network-impossible specs would pass static check |
| P07 reentry | FC1-N5 session state | spec/submit silently overwrites existing session. No idempotency guard at submission layer |
| P08 whitespace | FC1-N5 shielding | validate_answers uses is_empty() not trim().is_empty(). Whitespace-only answers bypass validation |
| P09 baseline | FC1 full chain | W8 retry chain works correctly (3 attempts, chain CIDs link attempt N → attempt N+1) |

### FC3: Meta-architecture (synthesis quality)

| Problem | FC3 Node | Finding |
|---------|----------|---------|
| P03 mode_drift | Synthesis filter | "请输出 PDF" absorbed as feature request by synthesis LLM. Spec explicitly says "不输出 HTML". Spec-artifact format mismatch unchecked |
| P04 impossible | Synthesis filter | "实时查股票价格" (requires network) absorbed as legitimate spec requirement. No platform-constraint filter |
| P05 contradictory | Contradiction handling | Correctly detected. "## 我听到的矛盾" section rendered. Simple spec preserved; complex features placed Out of Scope |
| P06 multilingual | Language normalization | Cantonese input correctly normalized to Simplified Chinese. Raw Q/A preserves original dialect |

### Shielding: Capsule chain integrity

| Capsule Type | Present | Referential Closure | Notes |
|-------------|---------|---------------------|-------|
| SpecCapsule | YES (P01/P03/P04/P05/P06/P09/p01_fifth_grader) | YES (session_id, model_id, logical_t) | Written by turingos spec CLI to session-local CAS |
| GenerationAttempt | YES (P01/P04/P09) | YES (spec_capsule_cid, parent_attempt_cid, retry_index) | 3 per failed generate run |
| GenerateRejection | YES (P01/P04/P09) | YES (spec_capsule_cid, generation_attempt_cid) | retryable=true for LlmApiError |
| BuildSessionView | NO | N/A | Correctly absent (no accepted artifacts) |

**C11 Invariant Check**: BuildStatus::Accepted has not flowed into sequencer admission for any session. All generate outcomes are LlmApiError. CAS contains no BuildSessionView capsules. C11 holds vacuously (no accepted artifacts to admit).

### Economy gate

Not exercised by this probe (no successful artifact generation to evaluate cost).

## Key Kernel Integrity Bugs Found

### BUG-1: validate_answers whitespace bypass (FC1-N5 shielding)
- **Location**: `src/web/spec.rs:420` — `answer.is_empty()` should be `answer.trim().is_empty()`
- **Impact**: Whitespace-only answers ("   ") pass validation, reach shellout
- **Severity**: Medium (spec synthesis LLM likely handles gracefully, but wastes LLM calls)

### BUG-2: spec/submit workspace-toml mismatch (FC1 spec path)
- **Location**: `src/web/spec.rs:302-311` — passes `session_dir` as `--workspace` to `turingos spec`
- **Impact**: `turingos spec` reads `session_dir/turingos.toml` which doesn't exist
- **Severity**: HIGH (spec/submit always fails; only spec/turn works for spec generation)
- **Note**: generate.rs has step 4b to copy toml but it silently fails (spawn_blocking bug)

### BUG-3: generate.rs step 4b copy silently fails (FC1 generate path)
- **Location**: `src/web/generate.rs:194-196` — `let _ = tokio::task::spawn_blocking(move || std::fs::copy(&src, &dst)).await;`
- **Impact**: turingos.toml is not copied to session_dir; generate fails with missing config
- **Severity**: HIGH (generate fails without manual toml copy)
- **Note**: Manual copy workaround: `cp workspace/turingos.toml session_dir/turingos.toml`

### BUG-4: Mode drift not filtered in synthesis (FC3 shielding)
- **Location**: `assets/prompts/grill_synthesis_zh.md` — no platform-constraint filter
- **Impact**: "输出 PDF"/"输出 Markdown" incorporated into spec as feature; spec-artifact format mismatch
- **Severity**: Medium (artifact will be HTML regardless; spec misleads user)

### BUG-5: Verifier has no runtime network detection (FC1 verify shielding)
- **Location**: `src/web/verify.rs:262-269` — MinimumBar verifier checks only HTML structure
- **Impact**: Network-dependent artifacts (fetch/XHR) would pass static check; fail silently at runtime
- **Severity**: Medium (sandboxed iframe; fetch would fail CORS, not cause data leak)

### GAP-1: C10 promotion guard blocks all spec/turn triage in clean workspace
- **Location**: `src/runtime/prompt_promotion.rs:96-140`
- **Impact**: Empty CAS (cas_dir exists but empty) hits `NoReceiptFound` (not `NoCasStore`). Triage blocked.
- **Note**: NoCasStore bypass only works when CAS directory doesn't exist at all. Expected behavior (C10 constitutional gate), but workspace setup documentation gap.

### GAP-2: deepseek-v4-pro API key invalid for generate (blackbox) role
- **Location**: Backend AppState API key; `SILICONFLOW_API_KEY` env var
- **Impact**: All generate LLM calls fail with HTTP 401. No artifact generation possible.
- **Note**: Same key works for spec synthesis (deepseek-v4-pro). Issue specific to generate timing or endpoint.

### GAP-3: assets/ not in workspace (spec/turn prompt_asset_missing)
- **Location**: `src/web/spec.rs:1393` — reads `workspace/assets/prompts/grill_meta_v1.md`
- **Impact**: spec/turn fails with `prompt_asset_missing` until assets symlink added to workspace
- **Workaround**: `ln -s /repo/assets /workspace/assets`
