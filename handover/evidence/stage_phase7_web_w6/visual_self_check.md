# Phase 7 W6 — visual self-check

Date: 2026-05-18
HEAD: (will be commit SHA of W6 commit)

## Method

1. `cd frontend && npm run build` → `dist/main.js` = 51 079 bytes (50.0 kB cap satisfied)
2. `cargo build --bin turingos_web --features web` (embeds the bundle via include_bytes!)
3. `TURINGOS_WEB_WORKSPACE=/tmp/turingos_w6_test ./target/debug/turingos_web --bind 127.0.0.1:8080`
4. Chrome MCP — navigate to `http://127.0.0.1:8080/build`
5. Screenshot the idle state, click "开始访谈 →", screenshot the Q1 interview state
6. Read console messages — 0 errors / exceptions

## Aesthetic targets (from the §6a verifier brief)

- [x] Wordmark with teal accent diamond + "PHASE 7" subtitle pill
- [x] Primary nav with `aria-current="page"` Build highlighted by teal underline
- [x] Page title in italic Fraunces (`从一段闲聊开始，做出你想要的那个小工具。`)
- [x] Small mono caption (`build · spec interview · phase 7 w6`)
- [x] Idle card: small-caps eyebrow (`TISR · 八问访谈`), italic lede paragraph,
      text-only CTA with teal accent underline
- [x] FC3-N31 footer notice + CONNECTED status pill (green dot)

## Interviewing state (after clicking 开始访谈)

- [x] One question per screen (NOT a long form)
- [x] Progress `Q 1 / 8` in monospace small-caps, top-right corner
- [x] Question text in large italic Fraunces (~32px) with generous line-height
- [x] Borderless textarea with hairline bottom rule, transparent background,
      placeholder in italic with the Cmd/Ctrl+Enter hint
- [x] `下一题 →` button bottom-right, accent teal underline
- [x] Auto-focus on the textarea (rAF deferred for layout)

## Console hygiene

`mcp__Claude_in_Chrome__read_console_messages onlyErrors=true` returned
`No console errors or exceptions found for this tab.` on both states.

## Iterations

Iteration 1: noticed `<tos-spec-grill>` was rendered twice on `/build` because
both the server-rendered HTML and `<turingos-root>._renderBuildView()` mounted
it. Fixed `turingos-root.ts::_renderBuildView` to be a no-op (the build page
mounts the grill via server HTML; turingos-root remains present only so the
WS state pill listener still attaches).

Iteration 2: bundle was 24 bytes over the 50 kB cap. Trimmed inline comments
in spec-grill / spec-result / artifact-viewer to land at 51 079 bytes.

## Iframe sandbox spot check

`<tos-artifact-viewer>` was not exercised in this self-check because the
flow requires a SiliconFlow API key to drive the LLM. The sandbox attribute
value is `allow-scripts` (verified by the
`artifact_viewer_constructs_iframe_with_sandbox_attribute` frontend test
and the `artifact_viewer_blocks_dangerous_sandbox_combinations` security
guard). The source assigns the value via
`setAttribute('sandbox', buildSandboxAttribute())`; the helper enforces
`allow-scripts` alone, never paired with `allow-same-origin` (which would
be a documented XSS bypass).

## Outcome

Editorial register achieved. Does NOT feel like a SaaS form: one question
at a time, large italic Fraunces, no border on the textarea, mono progress
indicator in the corner, single text-button CTA. Matches the W4.4 visual
language (oxidized teal accent, hairline rules, paper-toned palette).
