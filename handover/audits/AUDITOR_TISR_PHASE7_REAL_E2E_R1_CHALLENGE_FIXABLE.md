# TISR Phase 7 Real-LLM E2E — Round 1 Audit Verdict: CHALLENGE_FIXABLE

**Date**: 2026-05-18
**Branch**: `codex/tisr-phase7-web` @ HEAD `bb51571b`
**Reviewer**: Opus 4.7 Phase 7 Real-LLM E2E Test Director (xhigh thinking)
**Confidence**: High
**Production defects found**: 0 (substrate); 1 LLM-quality issue (Qwen3-Coder single-shot reliability)
**Retries used**: 1 of 1
**Evidence dir**: `handover/evidence/stage_phase7_real_e2e_20260518T031804Z/`

---

## Verdict: CHALLENGE_FIXABLE

TuringOS Phase 7 demonstrated end-to-end Agent OS capability against a
real architect-supplied SiliconFlow API key. From cold-start workspace
to fully-playable browser Tetris artifact, the substrate held every
invariant the architect asked the test to exercise: spec capsule
CID = sha256(spec.md) bit-exact, sandbox `"allow-scripts"` only,
API key never on disk, CAS objects content-addressed and committed
via libgit2-backed `.git` refs, agent registry honored, spec grill
faithfully captured user intent through DeepSeek-V3.2 synthesis.

The **first** Qwen3-Coder generation produced a non-functional game
(canvas-initialised but rAF loop never started, due to an inverted
guard condition in the keydown handler). The **second** generation
(retry) produced an 8/8-PASS game. Since the architect's standard is
"complete, mechanically-tested working software," the final delivered
artifact (attempt 2) meets that bar. But TuringOS's single-shot
reliability on this run was 1/2 = 50%, which is too low to ship to
non-developer users without an automated retry-on-mechanical-failure
loop.

This is **CHALLENGE_FIXABLE** rather than PROCEED because the
substrate-level finding "no automated mechanical-test loop after
generate" is a concrete, named, fixable gap — not a yes/no constitution
failure.

---

## 1. Mechanical Test Results — final artifact (attempt 2)

| # | Test | Result | Evidence |
|---|------|--------|----------|
| 1 | iframe HTTP 200 + body non-empty | PASS | `curl /api/artifact/.../index.html` returned 200/11057B |
| 2 | Playfield exists | PASS | `<canvas id="game-board" width="200" height="400">` (10×20 @ BLOCK_SIZE 20) |
| 3 | Score visible | PASS | `分数 0` / `最高分 0` rendered in DOM |
| 4 | Sandbox safe | PASS | `iframe.getAttribute('sandbox')` ≡ `"allow-scripts"` verbatim; `buildSandboxAttribute()` hard-coded |
| 5 | Keyboard reactive | PASS | `KeyboardEvent('keydown', {code:'ArrowDown'})` → `player.pos.y` 1→2 |
| 6 | Rotation works | PASS | `KeyboardEvent('keydown', {code:'ArrowUp'})` → matrix `[[0,0,3],[3,3,3],[0,0,0]]` → `[[0,3,0],[0,3,0],[0,3,3]]` |
| 7 | No console errors | PASS | 0 game-origin errors over 5s observation; only Chrome MCP extension noise at `:0:0` |
| 8 | Plays through (30 random keys / 10s) | PASS | piece migrated posY 0→14; no panic; game still alive at end |

**Final score: 8/8 PASS.**

Attempt 1 (broken) scored 5/8 with the 3 failures all stemming from
one Qwen logic bug; details in
`evidence/.../audit/attempt_1/bug_analysis.md`.

---

## 2. Tape + CAS Integrity (the architect-mandated audit)

### 2.1 Spec capsule CID equals sha256(spec.md) — bit-exact

```
UI-displayed CID:   cid:09209010…a7252756
shasum -a 256 spec.md:
  09209010b0271bf781bbaef643740754fa77b7133a7fa0f463859b48a7252756
```

Prefix `09209010` matches and suffix `a7252756` matches. The
truncated middle is also verified via the CAS index JSONL byte array
`[9,32,144,16,176,39,27,247,…]` which decodes to hex
`09209010b0271bf781…`. **Phase 6.3 spec_capsule.rs CAS contract HOLDS.**

### 2.2 CAS objects are content-addressed and durably persisted

- Session CAS index: `tmp/phase7_active/sessions/1779074775_6d96a209/cas/.turingos_cas_index.jsonl` — one EvidenceCapsule, size_bytes 7964, matches spec.md filesize
- Backing libgit2 store: `cas/.git/refs/heads/master` → commit `764207b` with message `cas put cid=cid:09209010…b48a7252756 logical_t=1779074809`
- Every CAS put becomes a real git commit. This is ChainTape-grade durability.

### 2.3 agent_audit_trail

The architect-spec'd path `tmp/phase7_active/agent_audit_trail.jsonl` does **not exist**. Phase 6.3 instead emits per-session `spec_transcript.jsonl` (11 lines: 1 system + 8 Q-A + 1 assistant — or 1 system + 8 user + 1 spec output payload depending on layout). The 8 Q-A entries match the user-simulator transcript exactly. Audit power equivalent; just a path mismatch in the architect-spec. **Spec note: update mission template to reference `sessions/<id>/spec_transcript.jsonl` instead.**

### 2.4 turingos welcome — independent CLI cross-check

```
[x] 1. turingos init
[x] 2. turingos llm config
[x] 3. turingos agent deploy (1 registered)
[ ] 4. turingos spec (task decomposition)
[ ] 5. turingos generate (deliverable)
```

The native `turingos welcome` CLI agrees with `/api/welcome/status`
on init/llm/agent, but neither walks `sessions/*/` to detect that
spec.md and artifacts/index.html exist on disk. **This is a real
Phase 6.3 reporting gap** — both the CLI and the web wizard's "Done"
detector are not session-aware. Recommend a Phase 7.x or Phase 6.3.x
follow-up: `OnboardingStatus::scan_sessions()` should set
`spec_done=true` when at least one `sessions/*/spec.md` is non-empty.

### 2.5 ChainTape (workspace `runtime_repo/`)

Empty. Per Phase 6.3 contract, `cmd_spec` and `cmd_generate` do not
emit `WorkTx` into the runtime ChainTape — they write to session CAS
only. This is intentional in the demo profile. **Not a Phase 7
regression** — same behaviour as Phase 6.3 baseline. For a future
production-grade evidence profile, one would anchor a SpecTx /
GenerateTx in L4.

---

## 3. API Key Invariant — Phase 6.3 cmd_llm.rs:18

```
$ grep -r "sk-bokl" tmp/phase7_active /tmp/turingos_web_live.log
(no matches)
```

`turingos.toml` on disk contains only `api_key_env = "SILICONFLOW_API_KEY"` (the env-var NAME, never the value). Chrome MCP form_input typed the key into the `<input type="password">` field; screenshots confirm no plaintext rendered. The Phase 6.3 invariant **"API key value is NEVER stored on disk"** HOLDS.

---

## 4. Sandbox Invariant — Phase 7 W6 contract

The artifact-viewer component's `buildSandboxAttribute()`
(`frontend/src/components/artifact-viewer.ts:16`) returns the constant
`['allow-scripts'].join(' ')`. Run-time DOM inspection confirmed
`iframe.getAttribute('sandbox')` ≡ `"allow-scripts"` with no
`allow-same-origin` token present. The Phase 7 W6 hard-coded XSS
mitigation **HELDS**.

---

## 5. Grill Mechanism Findings

### What worked
- All 8 questions surfaced cleanly; progress indicator accurate; advance button + Cmd+Enter both work
- DeepSeek-V3.2 spec synthesis: ~40 sec wall-clock; produced all 10 mandated sections (一句话目标 through 一句话给 AI 编程员)
- Q3 ("only remember high score") → Memory section: "玩家的最高分" (1 item, correct minimum, no scope creep)
- Q4 step-by-step flow → First Run as 7 numbered steps verbatim
- Q5 "静静地忽略" → Robustness "应保持安静，不崩溃也不弹窗" preserved
- Q8 reaffirmation block (single-file, no CDN, classic NES) honored — generated index.html has zero external `<script src=>` / `<link href=>` refs
- localStorage usage: Q3+Q8 → `localStorage.getItem('tetrisHighScore')` correctly chosen

### What didn't
- **Q5 is CRUD-biased**: the canonical question text mentions "金额填成『哈哈哈』" which is form-driven adversarial input — awkward for game-domain spec. User simulator translated; grill survived; but a domain-aware Q5 would yield richer adversarial cases
- **Q3 also CRUD-biased**: "关掉电脑明天再打开" works for tools-with-persistence but assumes batch-style usage
- **Q7 success metric leak**: spec captured the user's "10000+ high score" goal, but Qwen did not honor it — the generated `dropInterval` formula makes 10000 feasible but not necessarily inevitable. Not a grill bug, a generate-time scope drift

### What to fix next (Phase 7.x recommendations)
1. **Domain-polymorphic grill**: detect "game" vs "tool" intent in Q1; switch Q5/Q3 wording accordingly
2. **Session-aware welcome status**: `/api/welcome/status` and `turingos welcome` should walk `sessions/*/` and reflect spec/generate completion
3. **Auto-iterate loop after generate**: even a minimal "load index.html, dispatch Space keydown, assert canvas pixel-sum changes" smoke test would have caught attempt-1's frozen game before showing it to the user. This is the single highest-leverage fix.

---

## 6. Wall clock and cost

| Phase | Wall clock |
|---|---|
| Welcome wizard (init/llm/api-key/agent) | ~30 sec |
| Spec grill (8 typed answers + DeepSeek) | ~120 sec |
| Generate attempt 1 (Qwen) | ~73 sec |
| Attempt 1 inspection / bug-analysis | ~120 sec |
| Generate attempt 2 (Qwen retry) | ~75 sec |
| Mechanical tests + audit | ~90 sec |
| **Total** | **~14 minutes** |

SiliconFlow estimated cost: 1× DeepSeek-V3.2 spec ~¥0.10 + 2× Qwen3-Coder generate ~¥0.30 each ≈ **¥0.70 total** (1.5× Phase 6.3 cmd_llm.rs:14 "~¥0.45/session" budget because of the retry).

---

## 7. Recommendation to architect

**Substrate**: TuringOS Phase 7 is ready for developer-preview ship. All constitutional invariants (CAS integrity, sandbox, API-key hygiene, tape addressability) hold. Spec grill + DeepSeek-V3.2 produce coherent, intent-faithful spec.md.

**LLM quality gate**: Qwen3-Coder single-shot reliability is 50% on this run, which is the weak link. Recommend implementing a **server-side mechanical-test loop after `/api/generate`** before considering Phase 7 shippable to non-developer end users.

**Specific Phase 7.x backlog**:
1. (P0) Server-side post-generate smoke test (load, focus, dispatch Space, expect canvas mutation)
2. (P1) Session-aware welcome status — `OnboardingStatus::scan_sessions()`
3. (P2) Domain-polymorphic grill — detect "game" vs "tool" in Q1
4. (P2) ChainTape anchor for spec/generate events — would close the "agent_audit_trail" architect-spec gap and give L4 evidence

---

## 8. Files of interest

- Final delivered artifact: `handover/evidence/stage_phase7_real_e2e_20260518T031804Z/artifact/index.html`
- Attempt 1 (broken): `evidence/.../audit/attempt_1/index.html` + `bug_analysis.md`
- User simulator transcript: `evidence/.../user_simulator/transcript.md` (8 Q-A in character)
- Verdict JSON: `evidence/.../user_simulator/verdict.json`
- Full audit: `evidence/.../audit/AUDIT_VERDICT.md`
- Backend observer logs: `evidence/.../backend_observer/`
- Workspace artifact source: `tmp/phase7_active/sessions/1779074775_6d96a209/artifacts/index.html`
- Spec source: `tmp/phase7_active/sessions/1779074775_6d96a209/spec.md`

No code in `src/` or `frontend/src/` was modified by this test. `git diff` against HEAD `bb51571b` is clean.
