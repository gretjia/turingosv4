# AUDITOR_TISR_PHASE7_W8_VALIDATION_R1 — CHALLENGE_FIXABLE

**Auditor:** Test Director (Opus 4.7, xhigh)
**Stage:** Phase 7 W8 (auto-retry + heuristic verify) real-LLM E2E validation
**Run timestamp (UTC):** 2026-05-18T04:13:10Z → 2026-05-18T04:35:00Z (~22 min wall clock)
**Server PID under test:** 89066 (binary `target/debug/turingos_web` mtime 12:07)
**Commit HEAD under test:** `4d98ebdd` ("TISR Phase 7 W8: auto-retry on heuristic-fail")
**Risk class:** Class 2 (production write-path enhancement; quality heuristic + retry loop; no Class 4 surface)
**FC-trace:** FC1-N5 (post-generate verification protects read view) + FC1-N10 (write path quality gate before GenerateComplete)
**Evidence dir:** `/Users/zephryj/work/turingosv4/handover/evidence/stage_phase7_w8_validation_20260518T041310Z/`

---

## 1. VERDICT — CHALLENGE_FIXABLE

W8 substrate (retry loop, WS broadcasting, frontend state machine, last-artifact inspect link surfacing) is structurally sound and worked end-to-end. **However**, the load-bearing heuristic `Check 2: has_canvas` produced a **false positive** on three consecutive Qwen3-Coder attempts, rejecting a fully-functional DOM-grid Tetris implementation. The architect's stated pass criterion — *"a non-developer end user can hand off '做个俄罗斯方块' and get a working game with NO retry button click, NO debug, NO half-finished hand-off"* — is **NOT MET** for this run.

**Ship recommendation: fix-and-reship.** Do NOT ship W8 with Check 2 in its current form.

---

## 2. W8 efficacy — did retries fire?

**YES — retries fired exactly as designed.**

WS envelope sequence captured in `user_simulator/ws_envelope_log.json`:

| seq | t (epoch ms) | msg_type                  | attempt | reason                       |
|---:|---:|---|---:|---|
| 1 | 1779078295919 | generate_attempt_started  | 1/3 | — |
| 2 | 1779078353818 | generate_attempt_failed   | 1/3 | missing_canvas |
| 3 | 1779078353827 | generate_attempt_started  | 2/3 | — |
| 4 | 1779078395672 | generate_attempt_failed   | 2/3 | missing_canvas |
| 5 | 1779078395680 | generate_attempt_started  | 3/3 | — |
| 6 | 1779078446998 | generate_attempt_failed   | 3/3 | missing_canvas |
| 7 | 1779078447006 | POST /api/generate → 500  | — | `{kind: generate_quality_failed, reason: "missing_canvas: ... | last_artifact=1779078133_badebd6a/artifacts/index.html"}` |

Attempt durations (approximate): 58s, 42s, 51s. Total ≈ 151s of Qwen3-Coder time.

**`total_attempts` reported by final POST response:** ABSENT. The 500-failure path does NOT include `total_attempts` in the body (the field only appears on success response per `GenerateResponse` struct). The frontend recovers it from WS envelope count. This is an API contract asymmetry but a minor cosmetic issue, not a structural defect.

**GenerateComplete WS broadcast:** NEVER fired (because all attempts failed). Correct per W8 design.

**`<tos-artifact-viewer>` mount:** NEVER occurred (because 500 was returned). Correct per W8 design.

---

## 3. 8 mechanical test results (on `sessions/1779078133_badebd6a/artifacts/index.html`)

| # | Test | Result | One-line note |
|---|---|---|---|
| 1 | iframe HTTP 200 + body non-empty | PASS-equivalent | viewer didn't mount; top-level `/api/artifact/...` returned 200 + 11814 bytes |
| 2 | Canvas exists (10×20 grid) | **PARTIAL FAIL** | no `<canvas>` element; rendered via `<div id="game-board">` + 200 `.cell` divs (10×20). Functional, not literal-spec-compliant. |
| 3 | Score visible | PASS | `#score`=分数 0, `#high-score`=最高分 0 |
| 4 | Sandbox `"allow-scripts"` only | N/A | viewer never mounted; user inspect link bypasses iframe sandbox entirely (architectural side concern) |
| 5 | Keyboard reactive (ArrowDown) | PASS | board snapshot changed after key |
| 6 | Rotation (ArrowUp / Space) | PASS | board snapshot changed after ArrowUp; ArrowLeft also moves piece |
| 7 | No app-origin console errors (5s) | PASS | 5 exceptions observed, all Chrome MCP extension async-listener artifacts (not app-origin) |
| 8 | 30-random-key playthrough doesn't crash | PASS | pieces stacked at bottom, game continued without crash or white-screen |

**7 of 8 PASS, 1 PARTIAL FAIL (test 2: no `<canvas>` literal but DOM grid is functionally equivalent), 1 N/A.**

---

## 4. Tape + CAS integrity

- spec.md sha256: `b19fe1feb40c713061873edf40a7e6171dc8c6cb3d8396d75ffd706207a53a6f` — **matches** WS-displayed CAS CAPSULE cid `b19fe1fe…07a53a6f`. PASS.
- CAS index `/Users/zephryj/work/turingosv4/sessions/1779078133_badebd6a/cas/.turingos_cas_index.jsonl`: 1 EvidenceCapsule entry, `schema_id=turingos-spec-capsule-v1`, size 9577 bytes (== spec.md size), backend OID `110ed3c5...` resolves to a real git object. PASS.
- `turingos welcome --workspace tmp/phase7_active` reports steps 1-3 complete (init/llm/agent), but step 4 (spec) shows NOT complete — because the actual `sessions/<id>/` directory was written to `/Users/zephryj/work/turingosv4/sessions/` (cwd default) NOT `tmp/phase7_active/sessions/`. **TAPE LAYOUT DRIFT.**

---

## 5. Comparative analysis: Round 1 vs W8

| dimension | Round 1 (pre-W8) | W8 round |
|---|---|---|
| Qwen attempts triggered | 2 (user-sim manually re-POSTed) | 3 (W8 auto-retried) |
| Qwen attempt 1 outcome | broken (inverted nullish guard, top-of-game frozen) | functional DOM-grid Tetris |
| Qwen attempt 2 outcome | working Tetris (canvas-based) | functional DOM-grid Tetris |
| Qwen attempt 3 outcome | n/a | functional DOM-grid Tetris |
| Final user-visible state | working game in artifact-viewer iframe | failure chip + inspect link to working game |
| Manual retry button clicks required | 1 (user clicked re-generate) | 0 to see working game, but 重试生成代码 button still visible inviting more clicks |
| Single-shot success rate | 50% (1/2 Qwen calls produced working output) | 100% of attempts produced functional output; 0% passed heuristic |
| Architect pass-bar met? | NO (manual intervention required) | NO (failure UI shown for working artifact) |
| User experience qualitative | "manual retry required, but works" | "told it's broken, but actually works if you click an inspect link" |

**Qualitative conclusion:** Round 1's failure mode was Qwen-quality (the verifier could not catch the inverted-nullish bug because that round predated W8). W8's failure mode is **verifier over-strictness** — Qwen actually produced working Tetris all 3 times, and the heuristic falsely rejected all 3.

---

## 6. Heuristic verifier accuracy

**FALSE POSITIVES observed: 3** (one per attempt).

- `Check 2 (has_canvas)` rejected three artifacts that:
  - rendered a fully-functional 10×20 board via DOM grid
  - had document keydown handler ✓
  - had setInterval game loop ✓
  - had TETROMINOES dictionary with 7 pieces ✓
  - had clearLines() function ✓
  - had localStorage high-score persistence ✓
  - displayed score panel ✓
  - rendered, accepted keyboard input, and ran a 30-key playthrough without crashing

- The verifier's stated rationale (in `verify.rs:117-120`): *"game-class apps must have a canvas"*. This is **incorrect as written**. Tetris is routinely implemented in DOM, SVG, or WebGL; canvas is one of several valid rendering surfaces. The check is too narrow.

**FALSE NEGATIVES observed: 0** (no broken artifact slipped through; but this is a vacuous result since all 3 attempts were rejected before any could pass).

---

## 7. Wall clock + SiliconFlow cost estimate

- Wall clock: ~22 minutes total (welcome flow ~3min, interview ~7min, spec synth ~30s, 3× Qwen generate ~151s, mechanical tests + audit writing ~7min).
- SiliconFlow spend (estimate): 1 DeepSeek-V3.2 spec synthesis call (~¥0.05) + 3 Qwen3-Coder-30B generate calls (~¥0.30 each per `src/runtime/cmd_llm.rs:14`) ≈ **¥0.95**. Within the 0.70-1.50 budgeted range.

---

## 8. Evidence dir absolute path

`/Users/zephryj/work/turingosv4/handover/evidence/stage_phase7_w8_validation_20260518T041310Z/`

Contents:
- `user_simulator/transcript.md` — 8 Q&A + W8 observations + mechanical test table
- `user_simulator/criteria_results.json` — structured 8-test results + W8 envelope summary
- `user_simulator/verdict.json` — false-positive analysis + Round 1 comparative
- `user_simulator/ws_envelope_log.json` — captured WS frames
- `backend_observer/*` — server log snapshots, process tree, workspace evolution, w8_signals (empty — server stdout doesn't log retry detail; no anomalies)
- (no separate director_audit/ files this round — analysis embedded in this audit verdict)

---

## 9. Final artifact path

`/Users/zephryj/work/turingosv4/sessions/1779078133_badebd6a/artifacts/index.html`

- size: 11814 bytes
- sha256: `a857599bbc9835c85bef454ea3497f230fb97fd807c6a9df5d7a14d1fd46d666`
- access URL (top-level, no sandbox): `http://127.0.0.1:8080/api/artifact/1779078133_badebd6a/index.html`
- access via UI: 查看最后一次产物 ↗ link on the build page after the 3-retry failure

---

## 10. Recommendation to architect

**Fix-and-reship.** Specific follow-ups, in order of importance:

### P0 (blocking ship)

1. **Relax `verify.rs:117-120` (`Check 2: has_canvas`)** to accept any of:
   - literal `<canvas` element, OR
   - a DOM grid with >= 100 `class~="cell"` descendants (or similar marker), OR
   - an `<svg>` root with non-trivial children, OR
   - explicit declaration in the artifact of `getContext('webgl')` / `'webgl2'`.
   
   Alternatively: split Check 2 into two: `has_some_visible_render_surface` (canvas OR div OR svg OR webgl context) AND `surface_appears_grid_shaped_or_canvas` (only required if spec mentions a grid).
   
2. **Add a stub-LLM negative test** that emits the captured 11814-byte DOM-grid Tetris and asserts `verify` PASSES. Mirror the existing `verify_accepts_good_artifact` pattern. Place in `tests/cli_web_verify_smoke.rs`.

### P1 (UX consistency)

3. **Add `total_attempts` to the failure-path 500 response body** for API symmetry. The frontend already recovers it, but downstream consumers (CI smokes, telemetry) shouldn't have to.

4. **On terminal failure when a last_artifact exists, route the user through the iframe sandbox viewer** with a "(实验性/未通过启发式)" warning chip, instead of opening the artifact at top-level origin. Preserves W6 sandbox invariant.

### P2 (out of W8 scope but caught)

5. **Investigate `TURINGOS_WEB_WORKSPACE` env-var plumbing.** The server (PID 89066) wrote `sessions/<id>/` to its cwd `/Users/zephryj/work/turingosv4/` rather than the documented `tmp/phase7_active/`. The welcome flow's `/api/welcome/state` returns workspace cleanly, but `/api/generate` resolution path uses cwd by default. File new TB.

---

## 11. Out-of-scope but worth flagging

- W8 itself works as a substrate. Retry loop, WS contracts, frontend state machine — all correct.
- The Qwen3-Coder model is **not** the root cause of this run's failure. Qwen produced functional Tetris all 3 times. The defect is in the verifier.
- Heuristic verification at this layer is *fragile* — it codifies "what a Tetris HTML should look like" rather than checking "what a Tetris HTML should do". Future Phase 7.y headless-browser smoke (mentioned in `verify.rs:24-26`) would close this gap properly.

---

Signed: Test Director, Opus 4.7 xhigh, 2026-05-18.
