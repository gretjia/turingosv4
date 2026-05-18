# Phase 7 Real-LLM E2E Audit Verdict

**Date**: 2026-05-18T03:18:04Z … ~03:34Z
**Branch HEAD**: `bb51571b` (TISR Phase 7 W7.1 evidence)
**Verdict**: **CHALLENGE_FIXABLE**
**Retries used**: 1 of 1 budget (attempt 1 broken → attempt 2 green)
**Session**: `1779074775_6d96a209`

---

## 1. Mechanical Test Results (final = attempt 2)

| # | Test | Attempt 1 | Attempt 2 |
|---|------|-----------|-----------|
| 1 | iframe HTTP 200 + body non-empty | PASS | PASS |
| 2 | Playfield exists (canvas 200×400 = 10×20 @ 20px) | PASS | PASS |
| 3 | Score visible (分数 / 最高分) | PASS | PASS |
| 4 | Sandbox safe (`"allow-scripts"` only) | PASS | PASS |
| 5 | Keyboard reactive (ArrowDown → posY changes) | **FAIL** | PASS |
| 6 | Rotation works (ArrowUp → matrix mutates) | **FAIL** | PASS |
| 7 | No console errors from game code | PASS | PASS |
| 8 | Plays through (30 keys, no panic) | **FAIL** | PASS |
| | **Total** | **5/8** | **8/8** |

Attempt 1 bug — the keyboard `Space` handler reads `if (player.matrix === null)` to decide whether to start the game, but `init() → resetGame()` had already populated `player.matrix` via `createPiece()`. The guard was therefore never true on first press, so the rAF `update()` loop was never invoked. Game rendered the initial UI but never animated. Real Qwen3-Coder logic bug, not a TuringOS-substrate defect.

Attempt 2 replaced the matrix-null guard with an explicit `gameStarted` boolean and called `update()` at startup so the rAF loop spins from page-load. This is the version delivered as `artifact/index.html`.

---

## 2. Tape + CAS Integrity

### 2.1 Spec capsule CID ↔ sha256(spec.md)
- **UI shows**: `cid:09209010…a7252756` (truncated for display)
- **shasum -a 256 spec.md** = `09209010b0271bf781bbaef643740754fa77b7133a7fa0f463859b48a7252756`
- **Match**: 8-char prefix `09209010` ✓ + 8-char suffix `a7252756` ✓ (and the CAS index JSONL byte array decodes to the same hex)
- **Contract held**: Phase 6.3 `spec_capsule.rs` writes the capsule via `CasStore::put_capsule()` which content-addresses by sha256 of `EvidenceCapsule.canonical_bytes()` — and here `canonical_bytes` = spec.md bytes exactly.

### 2.2 CAS objects content-addressed
- Session CAS index: `tmp/phase7_active/sessions/1779074775_6d96a209/cas/.turingos_cas_index.jsonl` records one EvidenceCapsule with `size_bytes: 7964` matching `spec.md` on disk.
- libgit2 backing store: `cas/.git/refs/heads/master` is `764207b cas put cid=cid:09209010…` — every CAS put gets a real git commit. ChainTape-grade durability.

### 2.3 agent_audit_trail
- The architect-spec'd file `tmp/phase7_active/agent_audit_trail.jsonl` does **not exist**. Phase 6.3 instead writes per-session `spec_transcript.jsonl` (11 lines: 1 system prompt + 8 user Q/A rounds + 1 assistant final). This is a **spec/reality gap** but not a bug — the trail is present in a different location with the same audit power.
- Cross-checked: spec_transcript.jsonl entries Q1..Q8 match the user-simulator transcript exactly. The grill's audit trail is honest.

### 2.4 `turingos welcome` independent confirmation
```
[x] 1. turingos init
[x] 2. turingos llm config
[x] 3. turingos agent deploy (1 registered)
[ ] 4. turingos spec (task decomposition)
[ ] 5. turingos generate (deliverable)
```
The native CLI sees init/llm/agent (workspace-level) but does NOT see spec/generate (session-level). The web `/api/welcome/status` exhibits the same blindness. **This is a real Phase 6.3 reporting gap** — both the CLI and the web wizard's "Done" detector are not session-aware. The spec.md and artifacts/index.html clearly exist on disk; the status reporter just doesn't look in `sessions/*/`. Recommend fixing in Phase 7.x.

### 2.5 ChainTape (runtime_repo)
- `tmp/phase7_active/runtime_repo/` is empty. Per Phase 6.3 contract, `cmd_spec.rs` and `cmd_generate.rs` write to session CAS only and do NOT emit `WorkTx` into the runtime ChainTape. This is intentional in the demo profile; for production-grade evidence one would want a `SpecTx` / `GenerateTx` anchored in L4. **Not a Phase 7 regression** — same behaviour as Phase 6.3 baseline.

---

## 3. API Key Invariant

- `grep -r "sk-bokl" tmp/phase7_active` → no match
- `grep -r "sk-bokl" /tmp/turingos_web_live.log` → no match
- `turingos.toml` on disk contains `api_key_env = "SILICONFLOW_API_KEY"` only — never the value
- Chrome MCP form_input wrote into `<input type="password">`; screenshots show no plaintext key
- Phase 6.3 `cmd_llm.rs:18` invariant "API key value is NEVER stored on disk" **HELD**

---

## 4. Sandbox Invariant

- `iframe.getAttribute('sandbox')` returned exactly `"allow-scripts"` (no `allow-same-origin`)
- Same behaviour observed in the artifact-viewer component source: `buildSandboxAttribute()` returns the constant `['allow-scripts'].join(' ')`
- The Phase 7 W6 hard-coded XSS mitigation **HELD**

---

## 5. Grill Mechanism Findings

### What worked
- All 8 questions surfaced cleanly, progress indicator `Q n/8` accurate, Cmd+Enter / button advance both work
- DeepSeek-V3.2 spec synthesis took ~40 sec wall clock and produced **structurally correct spec.md** with all 10 mandated sections (一句话目标, Goal, Reference, Memory, First Run, Robustness, Out of Scope, Acceptance, Given/When/Then, 一句话给 AI 编程员)
- The grill faithfully captured user intent: Q3 ("only remember high score") → Memory section: "玩家的最高分" (1 item, correct minimum); Q4 step-by-step flow → First Run as 7 numbered steps verbatim; Q5 "静静地忽略" → Robustness "应保持安静，不崩溃也不弹窗"
- Q8's reaffirmation block (single-file, no CDN, classic NES) was honored — the generated index.html has zero external `<script src=>` or `<link href=>` references

### What didn't
- **Q5 is mismatched for game-domain spec**: the canonical question text ("故意乱点乱填 — 把『金额』填成『哈哈哈』") assumes a CRUD-like form-driven tool. For a real-time game, this question lands awkwardly. The user simulator answered honestly ("game has no fields, but keys should not crash") and DeepSeek did the translation, but a domain-aware Q5 would yield richer adversarial cases
- **Q3 framing is also CRUD-biased** ("关掉电脑明天再打开") — works fine for games-with-persistence but assumes batch-style tools
- **Q7 success metric**: the spec captured the user's "high score climbs to 10000+" goal, but Qwen did not honor it — the generated code's `dropInterval = Math.max(100, 1000 - Math.floor(score / 500) * 100)` makes the game reasonably playable but doesn't ensure 10000 is achievable in a normal session. Not a grill bug, a generate-time scope leak

### What to fix next
- Phase 7.x grill polymorphism: detect "game" vs "tool" intent in Q1 and switch Q5/Q3 prompt wording (game domain: "如果有人疯狂连按各种键…" instead of "金额填成哈哈哈…")
- `/api/welcome/status` + `turingos welcome` CLI should walk `sessions/*/` and report spec_done/generate_done. This will green-up the wizard's "5/5 done" indicator after a real spec round
- The single-shot Qwen reliability (50% success rate on this run: 1 broken, 1 working) suggests TuringOS should add a **post-generate mechanical-test loop** — even a tiny "headless smoke" of "load index.html, click body, dispatch KeyboardEvent('Space'), assert canvas non-empty" would have caught attempt-1's frozen game before serving it to the user

---

## 6. Wall clock and cost

- Welcome wizard (init / llm / api-key / agent-deploy): ~30 sec total
- Spec grill (8 answers + DeepSeek synthesis): ~120 sec (8×8s typing + 40s LLM)
- Generate attempt 1 (Qwen3-Coder): ~73 sec
- Audit / inspection of attempt 1: ~120 sec
- Generate attempt 2 (Qwen retry): ~75 sec
- Mechanical tests on attempt 2: ~90 sec
- **Total wall clock**: ~14 minutes
- **SiliconFlow cost estimate**: 1× DeepSeek-V3.2 spec (~¥0.10) + 2× Qwen3-Coder generates (~¥0.30 × 2) ≈ **¥0.70** (Phase 6.3 cmd_llm.rs:14 budgeted "~¥0.45 per game-build session" — this run was ~1.5× because retry counted)

---

## 7. Recommendation

**Substrate verdict**: TuringOS Phase 7 demonstrated AGI-era Agent OS capability end-to-end. A non-developer Chinese-speaking user can walk from cold-start to a playable browser game using only the welcome wizard + 8 grill answers + 2 button clicks. CAS integrity holds, sandbox isolation holds, API key invariant holds.

**Single-shot reliability**: Qwen3-Coder on this run produced 1 broken-then-1-working artifact. That's a 50% single-shot fail rate, which is too low for shipping to non-developer users without an automated retry-on-mechanical-failure loop.

**Architect choices**:
- **Ship as-is** (Phase 7 MVP delivered, retry-loop policy is a user-facing button): acceptable for a developer-preview audience
- **Add a kernel-side iterate loop** (auto-retry on mechanical-test failure, max 2 retries, hand back the better-performing artifact): would have caught attempt-1's bug invisibly to the user. This is the Phase 7.x or Phase 8 obvious next step.

The Phase 6.3 substrate is solid. The Phase 7 web shell is solid. The weak link is the LLM-as-codegen single-shot reliability, which is an LLM quality limitation that TuringOS should surface (and patch around) rather than hide.
