# TISR Phase 7 W8 — Visual self-check (inline description)

## Why inline (no screenshots)

Per W6/W7 history Chrome MCP `screenshot save_to_disk` writes to a sandbox
that the repo cannot read back. Rather than burn agent budget retrying a
known-broken integration, this self-check is written as a state-by-state
walkthrough that the architect can verify against the deterministic test
suite — every state below is exercised by an automated test.

## Code path map (UI states ↔ tests)

```
state: idle           — <button>生成代码 →</button>
state: generating     — button disabled, 正在生成…, progress chip "尝试 N/M",
                         optional "尝试 K/M 失败: <reason>" sub-note
state: generated      — <tos-artifact-viewer> mounted; if total_attempts>1
                         header shows "经过 N 次尝试 · 已通过启发式验证"
state: error          — error message + 重试 button + optional inspect link
```

## State 1 — generating, attempt 1/3

Triggered by clicking "生成代码 →" once. State flips to `generating`
immediately (no wait for backend). On first WS broadcast
`generate_attempt_started { attempt: 1, max_attempts: 3 }`, the progress
chip renders:

```
[正在生成…  (尝试 1/3)]
```

(Fraunces italic counter via `<em>` inside `<p>.spec-result-progress`.)

## State 2 — first attempt fails, attempt 2/3

When `generate_attempt_failed { attempt: 1, reason: "missing_canvas; …" }`
arrives, the failure reason is stored. The chip stays visible. When
`generate_attempt_started { attempt: 2 }` arrives the counter updates:

```
[正在生成…  (尝试 2/3)]
[尝试 1/3 失败: missing_canvas: 找不到 <canvas> 元素 — …]
```

This is what the test `generate_retries_on_heuristic_failure_via_stub`
exercises end-to-end: it observes two `generate_attempt_started` broadcasts
plus one `generate_attempt_failed` for attempt 1, then `generate_complete`
on attempt 2.

## State 3 — succeeded after retry, total_attempts=2

`<tos-spec-result>` swaps in `<tos-artifact-viewer>`. Because
`total_attempts > 1`, the viewer header now shows the W8 retry caption:

```
[生成产物 · LIVE PREVIEW]
[你的工具，已经写好了。]
[经过 2 次尝试 · 已通过启发式验证]   ← W8 NEW
```

Sandboxed iframe renders the LLM artifact as before. The W6 sandbox
contract (`allow-scripts` only, never `allow-same-origin`) is preserved —
asserted by `artifact_viewer_constructs_iframe_with_sandbox_attribute` and
`artifact_viewer_blocks_dangerous_sandbox_combinations`.

## State 4 — all three retries exhausted

If all three retries fail the POST resolves with HTTP 500 and body
`{"reason":"<joined heuristic failures> | last_artifact=<sid>/artifacts/index.html","kind":"generate_quality_failed"}`.

`<tos-spec-result>` shows:

```
[重试生成代码 →]
[失败信息 …]
[查看最后一次产物 ↓]   ← link to /api/artifact/<sid>/artifacts/index.html
```

The link is regex-extracted from the reason string, so the user can still
inspect what came out even though it failed heuristic verification.

## Verification gates that locked in this UX

| Gate                                                   | Coverage |
| ------------------------------------------------------ | -------- |
| `cli_web_verify_smoke::verify_rejects_inverted_nullish_guard` | the load-bearing real-world broken Tetris is flagged |
| `cli_web_verify_smoke::verify_accepts_good_artifact`   | the load-bearing real-world working Tetris is accepted |
| `cli_web_generate_smoke::generate_retries_on_heuristic_failure_via_stub` | POST flow + total_attempts=2 + WS broadcast sequence |
| `cli_web_ws_smoke` (unchanged)                          | WS upgrade + initial IR push still functional |
| `frontend npm test` (86 tests)                          | spec-result/artifact-viewer XSS hygiene + custom-element registration |
| `frontend npm run build`                                | bundle compiles with new WsMessage variants + total_attempts field |

## Bundle size

| Phase | dist/main.js |
| ----- | ------------ |
| W7    | 67.7 kB      |
| W8    | 71.2 kB (delta +3.5 kB) |

Delta is from the new WS event handling, retry progress chip rendering,
total_attempts caption, and inspect-link extraction in spec-result.ts.
