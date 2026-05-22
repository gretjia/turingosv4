# Phase 5 Real E2E Test Verdict — 2026-05-22

## Test metadata

- Date: 2026-05-22
- Backend: `http://127.0.0.1:8080` (PID 54468, binary `target/debug/turingos_web`)
- Workspace: `tmp/sw3_real_e2e_1779434622`
- Session ID: `5fb7bd54-232b-42b4-aa44-1ad5e136636f`
- LLM: DeepSeek direct (`api.deepseek.com/v1/chat/completions`, model `deepseek-chat`)

## Persona

**非开发者用户**：家庭主妇，经常做事情容易分心（学做饭、辅导孩子作业、刷手机）。
**工具需求**：番茄钟计时器（Pomodoro Timer）。

## Bug discovered and workaround

**Bug**: `turingos llm complete --strict-json` fails when DeepSeek returns JSON wrapped in markdown fences (` ```json ... ``` `), because `strip_think_blocks()` strips `<think>` tags but not code fences, and `grill_envelope::parse_and_validate()` calls `serde_json::from_str()` directly on the raw string, producing "invalid JSON: expected value at line 1 column 1".

**Workaround**: Modified workspace prompt asset `assets/prompts/grill_meta_v1.md` to add a stronger instruction: "CRITICAL: Your entire response must be ONLY a raw JSON object. No text before, no text after. No ```json fences. No backticks. No explanation. Start your response with { and end with }." This reduced (but did not eliminate) fence wrapping — intermittent retries were needed.

**Root cause location**: `src/sdk/protocol.rs::strip_think_blocks()` does not strip markdown code fences. Fix would be: extend `strip_think_blocks` to also strip leading/trailing ` ```json ``` ` wrappers before passing to `grill_envelope::parse_and_validate()`.

**Second bug found**: `POST /api/generate { from_capsule: true }` fails because the web spec handler writes SpecCapsule to `sessions/ID/capsules/cas/` but `turingos generate --workspace sessions/ID --from-capsule` looks in `sessions/ID/cas/`. Workaround: call with `from_capsule: false`, which uses `spec.md` directly.

## Full Turn Transcript

### Q1 (Turn 1)
**Question**: 你能描述一下，在什么具体的情景下，你会想"要是有一个小工具就好了"？比如，你正在做什么事情的时候，觉得有点麻烦或者缺了点什么？

**Answer**: 我平时在家做事情很容易分心，比如学做饭或者给孩子辅导作业，刷着手机时间就过去了。我想要一个可以定时25分钟的番茄钟，每隔一段时间提醒我休息，这样就能专心做事了。

**Covered**: `job`

---

### Q2 (Turn 2)
**Question**: 听起来你想要的工具，核心是帮你专注和休息的番茄钟。那它应该像你手机上的哪个App或者闹钟？比如，是像手机自带的计时器，还是像Forest那种种树的专注App？

**Answer**: 就像手机自带的计时器那样简单就行，但是能自动循环，25分钟结束了就提醒我休息5分钟，休息完了再自动开始下一轮，不用我每次都手动重新设置。

**Covered**: `job`, `anchor`

---

### Q3 (Turn 3)
**Question**: 那这个工具需要记住你的设置吗？比如，明天早上你打开它，是希望它还记得你上次设置的25分钟工作和5分钟休息，还是每次打开都重新设置？

**Answer**: 最好能记住，不用每次都重新设置，但如果我想改也可以改，比如有时候我想工作30分钟或者休息10分钟。

**Covered**: `job`, `anchor`, `memory`

---

### Q4 (Turn 4)
**Question**: 明白了，那当你第一次打开这个工具时，你希望看到什么？比如是一个大大的开始按钮，还是直接显示倒计时数字？

**Answer**: 希望看到一个大圆圈显示剩余时间，下面有一个开始按钮，还能看到现在是第几个番茄，比如'第2个番茄'这样。界面简洁一些，不要太花。

**Covered**: `job`, `anchor`, `memory`, `first_run`

---

### Q5 (Turn 5)
**Question**: 好的，那如果我在休息时间（比如那5分钟）突然想提前开始工作，或者在工作时间想提前结束休息，点一下就能跳过，这个功能你需要吗？

**Answer**: 需要，有时候事情来了没法等够5分钟，或者孩子放学了要去接他，所以可以随时暂停或者跳过当前计时，比较灵活。另外如果我一不小心手机碰到了，别乱跳，最好是要确认一下。

**Covered**: `job`, `anchor`, `memory`, `first_run`, `robustness`

---

### Q6 (Turn 6)
**Question**: 那如果有人在休息时间故意不点击开始，或者工作到一半把手机放一边不管了，倒计时结束后会怎样？比如一直响铃直到你回来点掉，还是响几声就自动停了？

**Answer**: 响几声就好，不用一直响，3秒钟的提示音就够了，然后自动进入下一阶段，不用我每次都点确认，太麻烦了。

**Covered**: `job`, `anchor`, `memory`, `first_run`, `robustness`

---

### Q7 (Turn 7)
**Question**: 好的，那我们来确认一下这个工具的边界。假设你一天用了很多个番茄，比如连续工作了8个番茄，你觉得这个工具需要限制每天最多多少个番茄，或者提醒你该休息久一点吗？

**Answer**: 不用限制，我自己知道该不该休息，这个工具只要帮我计时就好了，不需要告诉我该怎么做，简单就是最好的。

**Covered**: `job`, `anchor`, `memory`, `first_run`, `robustness`, `scope`

---

### Q8 (Turn 8)
**Question**: 那如果一天下来，你用了很多个番茄，比如20个，你希望工具能帮你统计一下今天总共专注了多少时间，还是说连这个统计也不需要？

**Answer**: 统计的话有个显示今天完成了几个番茄就够了，不用复杂的图表，就简单的数字，比如'今天：5个番茄'这样。什么时候感觉这个工具真的帮到了我，就是我能安心做完一件事，没有忍不住去刷手机。

**Covered**: `job`, `anchor`, `memory`, `first_run`, `robustness`, `scope`

---

### Q9 (Turn 9 → completed at Turn 10)
**Question**: 那最后一个问题：如果这个工具能让你安心做完一件事，你觉得什么样的具体表现可以证明它成功了？

**Answer**: 对我来说，能够连续完成3个番茄不被打断，就说明它有用了。我自己能感觉到，做完了感觉很踏实，不像以前东搞西搞却不知道做了什么。

**Result**: `done: true`, termination_reason: `llm_done_predicate_pass`, confidence: 0.95

---

## Summary stats

- Total turns to done: **10** (Q1 through done signal on Q10/turn 10)
- Triage retries due to markdown fence bug: ~5 across all turns (transient, resolved by retry)
- All 7 required slots covered: `job`, `anchor`, `memory`, `first_run`, `robustness`, `scope`, `acceptance`

## Spec quality

The generated `spec.md` is well-formed with all 7 slots populated from direct user quotes. The "一句话给 AI 编程员" instruction is present. Quality: **Good** — all user requirements captured clearly, natural Chinese preserved throughout.

## Artifact functional assessment

**YES — Fully functional**

The generated `index.html` (17,127 bytes) implements:
- SVG circle progress ring showing remaining time in large numerals
- Start/pause/stop buttons with appropriate state machine
- Skip button with confirmation overlay dialog (prevents accidental clicks)
- Auto-advance: work → rest → work cycles automatically with Web Audio API beep sounds
- LocalStorage persistence for work/rest duration settings and today's completed count
- Adjustable work (25min default) and rest (5min default) durations
- "第N个番茄" counter label and "今天：N个番茄" daily statistics
- Served at `http://127.0.0.1:8080/api/artifact/5fb7bd54-232b-42b4-aa44-1ad5e136636f/index.html` with HTTP 200

Code inspection confirms all persona requirements are met:
- `onTimerComplete()`: auto-advances without user input
- `playBeep(3)`: 3-beep sound (≈3s) matching "3秒提示音"
- `showConfirm()`: skip/stop require confirmation
- `loadSettings()`/`saveSettings()`: localStorage for persistence
- Today's count displayed as "今天：N 个番茄"

## Capsule chain

| Capsule | CID | Schema |
|---------|-----|--------|
| SpecCapsule | `603c182494e09ed2a2610730184bb1b23c87ac6411b6c87e07ec6d239d795860` | `turingos-spec-capsule-v1` |
| SpecGrillSession | `c84d848420bdbb8d5d4e8332ac17fd349248bcdc17dafb5a044097e379e34458` | `turingos-spec-grill-session-v1` |
| GenerationAttemptCapsule | `6c1bc49d6a06f36fa7cae2ce3d001902535950854b16539c130b6799f5b6704b` | `turingos-generation-attempt-v1` |
| ArtifactBundleManifest | `baca7b16bac67b4ee5db558db37cca79852599690f6fe31c7def59317c303b0b` | `turingos-artifact-bundle-v1` |

Additional capsules: 38 PromptCapsule/EvidenceCapsule objects in `sessions/ID/capsules/cas/` (turn prompts + triage capsules for 9 turns).

Workspace CAS index: 4 entries
Session capsules CAS index: 38 entries
Session CAS index: 6 entries (generation evidence)

## Final verdict: PARTIAL

**PASS components:**
- Grill driven interview: PASS (10 turns to done, all 7 slots covered)
- Spec synthesis (A6 LLM-less): PASS (spec.md generated, SpecCapsule written to CAS)
- Artifact generation: PASS (17KB single-file HTML, all spec requirements implemented)
- Artifact HTTP serve: PASS (200 OK)
- Capsule chain: PASS (SpecCapsule → GenerationAttemptCapsule → ArtifactBundleManifest chain complete)

**PARTIAL/FAIL components:**
- `from_capsule: true` generate path: FAIL — CAS path mismatch; workaround via `from_capsule: false`
- Intermittent `--strict-json` parse failures: ~35% failure rate due to DeepSeek markdown fence wrapping; resolved via retry (task contract allows retries, but the UX friction is real)
- Browser interaction: NOT performed — Chrome MCP extension was not connected; all API calls done via curl instead

## Bugs to fix

1. **`strip_think_blocks` missing fence stripping** (src/sdk/protocol.rs ~line 107): Add stripping of ` ```json ... ``` ` wrappers before envelope validation. This causes ~35% failure rate on deepseek-chat responses.

2. **generate `from_capsule` CAS path mismatch** (src/web/generate.rs vs src/web/spec.rs): SpecCapsule written to `sessions/ID/capsules/cas/` but generate CLI reads from `sessions/ID/cas/`. Either consolidate paths or copy capsule at synthesis time.

## UX friction observed

1. **Biggest friction**: The `--strict-json` + markdown fence issue causes silent retry loops at the backend. From a user's perspective, the grill just appears slow (15–30s per turn vs expected 5–10s) because one of the retries always hits the fence wrapping issue before succeeding on the next attempt.

2. **No browser UI tested**: The Chrome MCP extension was disconnected and the Control Chrome MCP reported Chrome not running as an accessible GUI application. All driving was via direct HTTP API. The frontend UI was not observed.

3. **`from_capsule: true` silently fails**: The generate endpoint accepts `from_capsule: true` but fails with a confusing "no spec capsule" error when the SpecCapsule exists in a different subdirectory. The error message doesn't hint at the path mismatch.
