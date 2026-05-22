# Failure Taxonomy — Generative HTML Kernel Probe 2026-05-22

## Classification Schema

Each failure classified by layer:
- **Validation**: pre-LLM input guard at HTTP trust boundary
- **Spec synthesis (LLM)**: synthesis prompt/LLM behavior
- **Kernel routing**: session management, workspace, capsule chain wiring
- **Artifact bundle**: generate/verify pipeline

---

## L1: Validation Failures

### V1: Whitespace bypass (BUG-1)
- **Layer**: Validation
- **Problem**: P08
- **Root cause**: `validate_answers` uses `is_empty()` not `trim().is_empty()`
- **Impact**: Whitespace-only answers ("   ") reach shellout; spec synthesis called with garbage input
- **Forward charter**: Fix `validate_answers` to reject whitespace-only answers with `invalid_input`

### V2: No session idempotency guard (BUG-5 analog)
- **Layer**: Validation + Kernel routing
- **Problem**: P07
- **Root cause**: `spec_submit_handler` does not check if session already has spec.md; overwrites silently
- **Impact**: Re-spec of an existing session replaces spec without warning
- **Forward charter**: Add check: if session_dir/spec.md exists, return 409 Conflict unless force=true

---

## L2: Spec Synthesis (LLM) Failures

### S1: Mode drift absorption (BUG-4)
- **Layer**: Spec synthesis
- **Problem**: P03
- **Root cause**: grill_synthesis_zh prompt has no platform-constraint filter. LLM interprets "输出 PDF" as feature requirement.
- **Impact**: Spec says "不输出 HTML" but generate always produces HTML. Spec-artifact format mismatch.
- **Forward charter**: Add synthesis prompt section: "Platform constraint: output is always a single HTML page. If user requests PDF/Markdown/Word/ZIP, note it as unsupported and synthesize the closest HTML equivalent."

### S2: Impossible network requirement absorption
- **Layer**: Spec synthesis + Artifact verify
- **Problem**: P04
- **Root cause**: Synthesis LLM accepts "实时查股票价格" (requires external API) without flagging it as runtime-impossible in a sandboxed HTML context.
- **Impact**: Spec calls for `fetch()` to external stock API; artifact would silently fail at runtime.
- **Forward charter**: Add synthesis prompt: "If the user requests real-time data from external servers (stock prices, weather, news), note in spec that this requires a backend proxy not available in the sandboxed HTML page. Synthesize a mock-data demo instead."

---

## L3: Kernel Routing Failures

### K1: spec/submit workspace-toml mismatch (BUG-2)
- **Layer**: Kernel routing
- **Problem**: All spec/submit calls
- **Root cause**: `spec_submit_handler` passes `session_dir` as `--workspace` to `turingos spec`. `turingos spec` reads `session_dir/turingos.toml` which doesn't exist.
- **Impact**: spec/submit always fails with `shellout_failed` (SILICONFLOW_API_KEY not configured).
- **Forward charter**: Fix: pass outer workspace to CLI, OR copy turingos.toml to session_dir before shellout (like generate.rs step 4b).

### K2: generate.rs step 4b copy silently fails (BUG-3)
- **Layer**: Kernel routing
- **Problem**: All generate calls without manual toml copy
- **Root cause**: `spawn_blocking(move || std::fs::copy(&src, &dst))` may fail for unknown reason (possibly move semantics or join error); failure silently ignored via `let _`.
- **Impact**: generate without manual toml copy fails with same workspace-toml error as spec/submit.
- **Forward charter**: Add error logging for step 4b copy failure; use `expect` or explicit error return if copy fails.

### K3: spec/turn assets path mismatch (GAP-3)
- **Layer**: Kernel routing
- **Problem**: spec/turn first answer (and all subsequent)
- **Root cause**: `spec_turn_handler` looks for `workspace/assets/prompts/grill_meta_v1.md` but assets live at repo root, not in the workspace subdirectory.
- **Impact**: spec/turn fails with `prompt_asset_missing` when workspace is not repo root.
- **Workaround**: Symlink `ln -s /repo/assets /workspace/assets`
- **Forward charter**: Either bundle assets into workspace on init, or read from binary CWD (repo root), or embed assets at compile time.

### K4: C10 promotion guard blocks clean workspace (GAP-1)
- **Layer**: Kernel routing (constitutional gate)
- **Problem**: spec/turn triage calls in empty CAS workspace
- **Root cause**: `check_promotion_guard` returns `NoReceiptFound` (not `NoCasStore`) when CAS dir exists but is empty. Only `NoCasStore` is bypassed for dev ergonomics.
- **Impact**: triage LLM calls are blocked until a PromptPromotionReceipt is written to CAS.
- **Note**: This is INTENTIONAL behavior (C10 kill criterion). The gap is in workspace setup: turingos init should create a promotion receipt for the bundled triage prompt, or the docs should clarify that spec/turn requires `turingos llm prompt-eval` to be run first.

---

## L4: Artifact Bundle / Verifier Gaps

### A1: No runtime network detection in verifier (BUG-5)
- **Layer**: Artifact verify
- **Problem**: P04
- **Root cause**: `verify_minimum_bar` and `verify_game_shape` check HTML structure only. No detection of `fetch()`, `XMLHttpRequest`, `WebSocket`, or external `<script src=...>` that would fail in sandboxed iframe.
- **Impact**: Network-dependent artifacts pass static verification; fail silently at runtime.
- **Forward charter**: Add MinimumBar check: flag if `fetch(` or `new XMLHttpRequest` or `open("GET"` or `open("POST"` found in artifact without a mock-data pattern. Surface as BrokenHtml with reason "network_call_in_sandboxed_context".

---

## Summary Table

| Failure ID | Layer | Problem | Severity | Bug/Gap |
|-----------|-------|---------|----------|--------|
| V1 | Validation | P08 | Medium | BUG: whitespace bypass |
| V2 | Validation | P07 | Medium | BUG: no respec guard |
| S1 | Spec synthesis | P03 | Medium | BUG: PDF mode drift absorbed |
| S2 | Spec synthesis | P04 | Medium | BUG: network requirement absorbed |
| K1 | Kernel routing | All spec/submit | HIGH | BUG: workspace-toml mismatch |
| K2 | Kernel routing | All generate | HIGH | BUG: toml copy silently fails |
| K3 | Kernel routing | All spec/turn | Medium | GAP: assets not in workspace |
| K4 | Kernel routing | All spec/turn | Low | GAP: C10 requires promotion receipt setup |
| A1 | Artifact verify | P04 | Medium | GAP: no network call detection |
