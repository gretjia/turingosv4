# USERSIM — Non-expert end-user first-run, DeepSeek dual-key plan, 2026-05-21

| Field | Value |
|-------|-------|
| Date | 2026-05-21 |
| Persona | Solo indie hobbyist, no Rust, knows HTML, wants a small browser game |
| Sub-agent | sonnet user-sim, worktree-isolated, ~4 min runtime |
| Plan | v4 (`/home/zephryj/.claude/plans/multi-agents-orchestrator-flash-agents-dazzling-eich.md`) |
| Phase 1 prerequisites | P2 dual-key, P3 thinking param, P4 UX hints, P1 C11 re-wire — all merged to main before this run |
| Workspace | `/tmp/turingos-usersim-2546293/` (4-min run; preserved for post-mortem) |
| Outcome | **Delivery succeeded** (Star Catcher game in `artifacts/index.html`, playable, matches user's plain-language description) — but 8 real UX bugs surfaced |
| Authority | User-delegated: "缺什么你补什么，找到问题修复问题是你的责任" |

## 1. Persona + ask

Persona stayed in character throughout: a solo indie hobbyist who has heard "TuringOS builds web apps from plain-language descriptions," has zero Rust experience, knows HTML exists from past dabbling. Two real DeepSeek API keys provided in `/.env` (`DEEPSEEK_API_KEY` + `DEEPSEEK_API_KEY_WORKER`).

Game decided **in the moment** (not pre-planned): **"Star Catcher"** — stars fall from top, basket at bottom moves with arrow keys, 3 misses = game over, single HTML file, no backend.

## 2. Capsule chain — confirmed end-to-end FC1/FC3 closure

In transcript order:

| # | CID | Schema | Status |
|---|-----|--------|--------|
| 1 | `f962cd35…b07d97bc` | turingos-spec-capsule-v1 | Written by spec (--skip-llm path) |
| 2 | `20ee840a…61537d56` | turingos-generation-attempt-v1 | Failed attempt (HTTP 400 model name mismatch) |
| 3 | `77dded20…64d82bb3` | turingos-generate-rejection-v1 | Rejection for #2 |
| 4 | `3018437c…0aae04b5` | turingos-generation-attempt-v1 | Successful attempt (outcome=Success) |
| 5 | `599863c8…197aa1c1` | turingos-artifact-bundle-v1 | Bundle for `index.html` |
| 6 | `967eecb3…b6e34336` | turingos-test-run-v1 | **C11 producer fired** (overall_pass=true, since user proceeded to success message and got `[x]` checkmark in welcome) |

**P1 verification on real run: ✅** — `test_run_cid=` appears after `artifact_bundle_cid=` in cmd_generate stdout, confirming the C11 producer call lands the TestRunCapsule into CAS post-bundle. The successful run had NO `rejection_cid`, meaning overall_pass was true and the delivery gate passed.

## 3. Bugs found (8) — prioritized

### B1 — Model-name mismatch when endpoint redirected to DeepSeek [HIGH]

User pointed `TURINGOS_SILICONFLOW_ENDPOINT=https://api.deepseek.com/v1/chat/completions` (per Plan v4 P4 example help text). But `turingos.toml` written by `turingos llm config` defaults Meta-model = `deepseek-ai/DeepSeek-V3.2` and Blackbox-model = `Qwen/Qwen3-Coder-30B-A3B-Instruct` — these are SiliconFlow's namespaced model strings, **not valid DeepSeek API model IDs**. DeepSeek's API responds:

```
HTTP 400 from SiliconFlow: {"error":{"message":"The supported API model names are deepseek-v4-pro or deepseek-v4-flash, but you passed deepseek-ai/DeepSeek-V3.2."}}
```

User had to abandon DeepSeek entirely and fall back to direct SiliconFlow with the existing `SILICONFLOW_API_KEY`. The whole P2+P3+P4 dual-key infrastructure was **never exercised on a real LLM** in this run.

**Root cause**: TuringOS's default config writes SiliconFlow-namespaced model strings even when the user wants DeepSeek direct-API.

**Surgical fix**: when an LLM call returns 4xx with the literal phrase `"supported API model names"`, intercept and rewrite the error as:

```
error: The LLM provider rejected model "<X>". This usually means the endpoint
       (TURINGOS_SILICONFLOW_ENDPOINT) expects different model strings than the
       SiliconFlow-style defaults.
       If you're targeting DeepSeek direct API, run:
         turingos llm config --workspace <ws> \
             --meta-model deepseek-v4-pro \
             --blackbox-model deepseek-v4-flash
```

**Risk class**: 2.

### B2 — `turingos init --project .` rejects empty dir without `--force` [HIGH]

User flow: `mkdir -p $WS && cd $WS && turingos init --project .` → exit 1: "project directory already exists". User had to add `--force`.

Reason: `cmd_init.rs` checks `if dir.exists()` without checking `if dir is empty`.

**Surgical fix**: change the existence check to `if dir.exists() && !dir.read_dir()?.next().is_none()`. Force is only needed when the dir has content.

**Risk class**: 1 (UX hygiene).

### B3 — Generation/rejection CIDs printed BEFORE error message [MEDIUM]

On a failed generate, stdout looked like:

```
generation_attempt_cid=20ee840a...
rejection_cid=77dded20...
turingos generate: HTTP 400 from SiliconFlow: ...
```

A non-expert reads top-to-bottom and sees `…_cid=…` first → thinks it's a success. The error appears at the END, easy to miss in noisy output.

**Surgical fix**: route CID lines to stderr with a `FAIL:` prefix when the outcome is not `Success`. Successful CIDs continue to stdout as today.

**Risk class**: 1.

### B4 — `test_run_cid=` has no plain-language explanation [MEDIUM]

On success: `test_run_cid=967eecb3...` appears. User wrote: "I have no idea. Did TuringOS run my game somehow? Did it test it in a browser?"

**Surgical fix**: after `test_run_cid=<hex>` line, print:

```
Internal tests: PASS (3/3 scenarios)  — entrypoint exists, HTML parses, sandbox policy preserved
```

(or `FAIL (X/Y scenarios)` with the failing scenario names listed). This makes the C11 gate visible to humans, not just CAS readers.

**Risk class**: 1 (additive print line).

### B5 — Welcome step `[ ] 3. turingos agent deploy` permanently unchecked but "All onboarding steps complete" message also printed [LOW]

Contradiction in welcome's state machine. Agent deploy is for multi-agent template, not relevant to web-game generation.

**Surgical fix**: when template is `proof` or default, suppress the "agent deploy" line from the checklist (or mark it `[—]` "skipped for this template"). When user actually runs multi-agent template, show it.

**Risk class**: 1.

### B6 — Template `proof` is misleading default for web-app users [MEDIUM]

`turingos init --template proof` → Lean theorem-proving market. User wanted a web game, almost gave up thinking they had the wrong tool.

**Surgical fix** (small): rename `proof` template UX-help to clarify it's the most generic / non-economic, and add brief help text on each template. Larger fix: add a `webapp` or `game` template that's preset for HTML deliveries.

**Risk class**: 1-2 depending on scope.

### B7 — `--skip-llm` framing makes user feel they're in "test mode" [LOW]

When the real LLM call failed in spec, user fell back to `--skip-llm`. The flag's `--help` says "Useful when SILICONFLOW_API_KEY is unset and you only want to test the CAS wire" — making the user feel they're not doing a "real" run.

**Surgical fix**: reframe `--skip-llm` help text: "Synthesize spec.md from raw answers without LLM-driven narrative. Use when LLM is unavailable; output is functionally equivalent and still proceeds to generate."

**Risk class**: 0 (docs only).

### B8 — Dual-key DeepSeek path entirely unexercised in this run [STRUCTURAL]

Despite Plan v4 P2+P3+P4 building the dual-key infrastructure, the user-sim flow used **single SILICONFLOW_API_KEY** throughout. This is partly B1's fault (model defaults broke DeepSeek), partly that welcome doesn't push users toward dual-key as the recommended path.

**Defer**: not a fix in this loop. Subsequent post-mortem should design either:
- (a) DeepSeek as a first-class provider with `turingos init --provider deepseek` flag that pre-fills `turingos.toml` correctly, OR
- (b) Documentation that explicitly walks the dual-key DeepSeek path as Tier-1 instead of as an example in `llm config --help`.

## 4. What worked well (4)

| # | Win |
|---|-----|
| W1 | `turingos welcome` progress checklist (`[x]` / `[ ]`) was immediately legible at any point — "where am I?" answered in one command |
| W2 | Generate output gave a single copy-pasteable `xdg-open ./artifacts/index.html` line — user didn't have to hunt for the file |
| W3 | Spec.md correctly captured the 8 raw answers into labeled sections even with `--skip-llm` — non-expert felt their input was being taken seriously |
| W4 | `turingos spec --help` listed all 8 interview questions verbatim with methodology citation ("Customer Development", "The Mom Test") — felt like the tool respected the user's non-expert input |

## 5. FC trace

- **FC1 (Runtime loop)**: closed. spec → generation_attempt → artifact_bundle → test_run capsule chain reconstructable via offline replay (orchestrator did not re-run replay because workspace is on /tmp and lives in tmpfs).
- **FC2 (Boot)**: closed. `welcome` correctly derived state from on-disk + CAS each invocation; no in-memory state assumed.
- **FC3 (Meta)**: surfaced 8 feedback items (this doc) — Article V Veto-AI loop equivalent: `{NO-VIOLATION ∧ UX-DEFECTS-FOUND}`. None constitutional, all product-UX.

## 6. Verdict

| Question | Answer |
|----------|--------|
| Did the system close the spec→artifact→test→delivery loop on a real run? | **Yes**. |
| Did the C11 producer (P1) actually fire on the real run? | **Yes** — `test_run_cid` visible in stdout, overall_pass=true confirmed via welcome `[x]` |
| Was the user-facing UX releasable to a non-expert as-is? | **No** — 8 bugs (4 high/medium) must be fixed first |
| Was the dual-key DeepSeek path exercised end-to-end? | **No** — B1 broke it; B8 structural fix needed before claiming the path works |
| Is anything load-bearing constitutionally violated? | **No** — all findings are product-UX, not constitution / FC / Trust Root |

## 7. Next step

Phase 3 immediate: surgical fix PR for B1+B2+B3+B4 (Risk-1/2 patches). B5+B6+B7 batched as cleanup. B8 deferred to next charter (decide DeepSeek tier).

After fixes land + Cz trust-root rehash (Cargo.lock from P3): re-run user-sim with same persona to verify B1-B4 cleared. Then halt — all Plan v4 tasks merged.

## 8. Evidence preservation

User-sim full transcript is in this session's task output file (kept by harness). Workspace `/tmp/turingos-usersim-2546293/` will be lost on tmpfs cleanup — not preserved here because no `handover/evidence/` write was authorized by §8 (Class 1 observation doc only).
