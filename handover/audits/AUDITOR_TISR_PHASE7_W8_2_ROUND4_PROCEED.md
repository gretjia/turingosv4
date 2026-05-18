# TISR Phase 7 W8.2 — Round 4 Final Validation — PROCEED

- **Audit timestamp**: 2026-05-18T05:25:18Z
- **HEAD under test**: `f0e08d9c` (W8.2: complete `resolve_workspace` harmonization, 4th file)
- **Server PID**: 94015 (W8.2 binary, fresh start, no `TURINGOS_WEB_WORKSPACE` env)
- **Evidence dir**: `handover/evidence/stage_phase7_w8_2_round4_20260518T051400Z/`
- **Final session**: `tmp/phase7_active/sessions/1779081457_2c7e041a/`
- **Verdict**: **PROCEED** — Phase 7 substrate is ready to ship to non-developer end users.

---

## 1. Verdict semantics

W8.2 closes the regression. The end-to-end real-LLM flow now delivers a working
artifact to the user in a single clean run:

```
/welcome → init → llm-config → api-key → agent-deploy
  → /build → spec/questions (8) → spec/submit (DeepSeek-V3.2, 31s)
  → spec_complete (WS, capsule_cid)
  → /api/generate → generate_attempt_started (1/3) → generate_complete (WS)
  → artifacts/index.html (10,159 bytes, single-shot W8 attempt=1)
```

The artifact:
- Renders a playable 俄罗斯方块 (Tetris) on a `<canvas>` 10x20 surface.
- Accepts ArrowLeft/Right/Up/Down keys (rotation visibly morphs the active piece).
- Ignores letter keys silently (no error dialog, no game-over trigger).
- Survives a 30+ key playthrough with no app-origin console errors.
- Persists high-score via `localStorage` (`localStorageAvail: true`).

P2 (workspace path harmonization) is fully closed: across 34 observer ticks
(~9 minutes), `repo_root_sessions=0` throughout; `workspace_sessions` reached 1
exactly when expected (after spec/submit), then `artifacts_indexhtml` reached 1
after generate. No HTTP 400 "session not found" from `/api/generate`.

---

## 2. W8 efficacy this round

- `/api/generate` response: HTTP 200, **`total_attempts: 1`** — single-shot success.
- WS envelopes: `generate_attempt_started { attempt: 1, max_attempts: 3 }`
  followed by `generate_complete { artifacts: ["index.html"] }`. No
  `generate_attempt_failed` envelope was emitted (none required).
- Qwen pattern this round: standard `<canvas>` Tetris with `keydown` listener,
  `setInterval` game loop, `localStorage` high-score persistence, balanced
  braces, no external scripts/stylesheets. **Trivially passes W8.1's relaxed
  `has_playfield`** via canvas branch.
- W8 retry pipeline coverage: this round exercised the **attempt-1 success
  path** end-to-end. Retry-on-failure was not stressed (Qwen happened to
  produce a clean canvas artifact first try).
- False-positive count for `has_playfield`: **0** (canvas matched; no spurious
  rejections).

---

## 3. 8 mechanical results (final artifact)

| # | Test | Result |
|---|---|---|
| 1 | iframe sandbox = `"allow-scripts"` only (no `allow-same-origin`) | **PASS** — bundle grep shows only `"allow-scripts"` token; `'allow-same-origin'` literal count = 0 |
| 2 | Visible game surface (canvas/grid/svg/table/cell) | **PASS** — `<canvas id="game-board" width="300" height="600">` |
| 3 | Score visible | **PASS** — DOM `#score-display` shows `得分: 0 \| 最高分: 0`; updates as game progresses |
| 4 | Keyboard reactive (arrow keys cause state change) | **PASS** — ArrowDown caused piece to descend; canvas pixel survey changed |
| 5 | Rotation works (ArrowUp morphs piece shape) | **PASS** — visible shape change in pre/post screenshots (J → rotated J) |
| 6 | Letter keys ignored cleanly | **PASS** — `a b c d e f` pressed, scoreText unchanged, no game-over, no errors |
| 7 | No app-origin console errors | **PASS** — 5 console errors observed but all are Chrome-extension "listener indicated an asynchronous response" (line 0:0); zero from artifact code |
| 8 | 30+ key playthrough survives | **PASS** — 30 arrow keys executed, first piece landed at bottom, new piece spawned, no exception, no game-over |

8/8 PASS.

---

## 4. P2 verification

- `/api/welcome/status` → `workspace_path: "tmp/phase7_active"` (W8.1/W8.2 default).
- `tmp/phase7_active/sessions/1779081457_2c7e041a/` exists and contains
  `answers.json`, `spec.md`, `spec_transcript.jsonl`, `cas/` subdir, `artifacts/index.html`.
- `repo_root_sessions=0` across all 34 observer ticks (no stray `sessions/` at repo root).
- `/api/generate` resolved the session via the same `tmp/phase7_active` default and
  returned HTTP 200 — confirming W8.2's 4th-file harmonization (generate.rs:570).
- The "split-brain" failure mode from W8.1 Round 3 (spec writes to
  `tmp/phase7_active/`, generate looks under `cwd/`) is gone.

P2 status: **CLOSED**.

---

## 5. Tape + CAS audit

- **spec.md sha256** = `f3f273a9cf0929c9364014f4e6a42c08e7a43eb67b4ed4fac32fe9c12cee4245`
- **`/api/spec/submit` response.capsule_cid** = `f3f273a9cf0929c9364014f4e6a42c08e7a43eb67b4ed4fac32fe9c12cee4245`
- **WS `spec_complete.capsule_cid`** = `f3f273a9cf0929c9364014f4e6a42c08e7a43eb67b4ed4fac32fe9c12cee4245`

All three match. Content-addressed storage verified.

- **Session CAS index** (`cas/.turingos_cas_index.jsonl`): one entry with the
  matching 32-byte CID (`[243,242,115,169,...]` → `f3f273a9...`), backed by
  libgit2 OID `0e784b8c0d85fc9a8b3eae986e24da0ae30fd664`,
  `object_type: "EvidenceCapsule"`, `schema_id: "turingos-spec-capsule-v1"`,
  `size_bytes: 7674`.
- **`cas/.git/objects/`**: 5 content-addressed blobs (loose objects); name = sha-1
  of zlib-compressed content (the libgit2 C2 path).
- **`cas/.git/refs/chaintape/cas`** = `4ef7dd890e05c9a3d1d5835ce875c4747e63b41a`
  — real commit-style HEAD_t ref present.

CLI confirmation (`turingos welcome --workspace tmp/phase7_active`):

```
[x] 1. turingos init
[x] 2. turingos llm config
[x] 3. turingos agent deploy (1 registered)
[ ] 4. turingos spec (task decomposition)
[ ] 5. turingos generate (deliverable)
```

Note on items 4/5: the CLI welcome view inspects **workspace-level**
`spec.md`/`artifacts/`, whereas the web flow stores per-session
(`sessions/<sid>/spec.md`, `sessions/<sid>/artifacts/`). This is a known
architectural distinction (the server has its own `/api/welcome/status` that
does inspect session-level state and was used during the run), not a regression
introduced by W8.2. Steps 1-3 (workspace-level: init/llm/agent) confirmed.

Tape + CAS integrity: **PASS**.

---

## 6. WebSocket broadcast sequence

8 envelopes total across the run. Spec/generate-related:

```
spec_complete         { session_id: "1779081457_2c7e041a",
                        capsule_cid: "f3f273a9...4245" }
generate_attempt_started { session_id: "1779081457_2c7e041a",
                           attempt: 1, max_attempts: 3 }
generate_complete     { session_id: "1779081457_2c7e041a",
                        artifacts: ["index.html"] }
```

No malformed envelopes. No `generate_attempt_failed` (expected — single-shot
success). 3 `ir_update` envelopes for dashboard state. Sequence is coherent
and matches the spec→generate happy path.

---

## 7. Wall clock + SiliconFlow cost

- **Wall clock**: 2026-05-18T05:13:10Z → 2026-05-18T05:25:18Z ≈ **12 min 8 s**
- **SiliconFlow cost** (estimated): spec/submit (~31s of DeepSeek-V3.2 generating
  ~7,674 bytes of spec.md) + generate (~70s of Qwen3-Coder-30B generating
  ~10,159 bytes of HTML). Single LLM call per step; W8 retry not triggered.
  Estimated total token spend ≈ 10-15K input + 4-5K output across both calls.
  At SiliconFlow's listed DeepSeek-V3.2 / Qwen3-Coder-30B rates this is well
  under the **¥0.50** budget for this round (≈ ¥0.10-0.25 most likely).

---

## 8. Evidence dir + final artifact path

- **Evidence dir**: `handover/evidence/stage_phase7_w8_2_round4_20260518T051400Z/`
  - `welcome_init.json`, `welcome_llm.json`, `welcome_key.json`, `welcome_agent.json`
  - `spec_questions.json`, `spec_answers.json`, `spec_submit.json`
  - `generate.json` (HTTP 200, total_attempts=1)
  - `observer.jsonl` (34 ticks)
  - `ws_envelopes.jsonl` (8 envelopes incl. spec_complete + generate_*)
  - `final_artifact_index.html` (sha256 `a633f172e519ed12875d830d546223c5caaa27cb9213b8ed734daf091c1662f6`)
  - `static_main.js` (deployed bundle for sandbox audit)
  - `turingos_welcome_workspace.txt` (CLI confirm steps 1-3)
- **Final artifact**: `tmp/phase7_active/sessions/1779081457_2c7e041a/artifacts/index.html`
- **API-key handling**: passed via curl `--data @<tmpfile>` then file shredded;
  never written to disk in repo; referenced in this audit only as `sk-bokl…dxnck`.
- **Hard constraints honored**: iframe sandbox unchanged (`allow-scripts` only);
  no Phase 6.1/6.3 substrate edits; no new Cargo deps; no git push / PR;
  no edits to source under test.

---

## 9. One-line recommendation

**SHIP.** Phase 7 web substrate is end-user-deliverable; W8.2 closes the
spec→generate hand-off bug; W8 retry pipeline + W8.1 heuristic relaxation
remain available as safety nets but were not required this round.

---

## Optional Phase 7.z forward observations (non-blocking)

These are **not** ship-blockers. Surfacing them as honest signal for the architect:

1. **CLI welcome shows steps 4/5 as `[ ]` even after a successful web run.**
   This is by design (per-session vs workspace-level inspection) but may
   confuse mixed CLI+web users. A Phase 7.z atom could either (a) teach
   `cmd_welcome.rs::inspect_workspace` to also glance under
   `sessions/*/spec.md` and `sessions/*/artifacts/`, or (b) document the
   distinction explicitly in `turingos welcome --help`.

2. **Workspace-level `<workspace>/cas/` is empty**, while the real CAS lives
   under `sessions/<sid>/cas/`. The `/api/welcome/status` field
   `spec_capsule_cid` looks in `<workspace>/cas/` only and so returns
   `null` even after a successful spec submit. Minor frontend display
   issue; the spec_complete WS envelope carries the correct CID so the
   build/spec-result panel is unaffected.

3. **W8 retry pipeline has 0 firing coverage this round** (Qwen produced a
   clean canvas Tetris on attempt 1). The unit-level fix is sound and
   prior rounds exercised retries, but the production retry path could
   stand a synthetic forced-failure smoke test in a future TB.

None of (1)/(2)/(3) blocks PROCEED.
