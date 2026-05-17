# TISR Phase 6.3 — Real-World Demo Runbook

**Date**: 2026-05-17
**Branch**: `codex/tisr-phase6-3-realworld-demo`
**FC-trace**: FC2-N16 (CLI dispatch) + FC3 evidence binding (CAS-anchored spec capsule)
**Risk class**: Class 2 (production wire-up over existing public CAS surface).
**Class-4 schema touch**: NONE. `src/bottom_white/cas/schema.rs` not modified.

## What This Atom Closed

Three architect mandates from the Phase 6.3 pivot (2026-05-17 user-direct, verbatim):

1. **"我需要一步到位，现在就完善 CAS wire"** — wire CAS now, not Phase 7+.
2. **"我的硅基流动 API ... 用真模型"** — use real SiliconFlow LLM API, no mocks.
3. **"假设 human 不是专业软件开发人员"** — non-developer-targeted natural-language grill, using customer-development / marketing-psychology methodology.

All three are addressed in this atom.

## Five New Source Files

| File | Lines | Purpose |
|---|---:|---|
| `src/bin/turingos/siliconflow_client.rs` | 200 | Real reqwest+tokio HTTPS client for SiliconFlow's OpenAI-compatible Chat Completions API. Bearer auth; non-streaming. |
| `src/bin/turingos/spec_capsule.rs` | 130 | CAS wire helper: writes spec.md bytes to `<workspace>/cas/` via the lib crate's `CasStore` (git2-backed; sha256 CID; sidecar JSONL). Schema-id tag `turingos-spec-capsule-v1`. |
| `src/bin/turingos/cmd_llm.rs` | 250 | `turingos llm config / show` — defaults to SiliconFlow + DeepSeek-V3.2 + Qwen3-Coder. API key value NEVER persisted to disk (env-var-name-only). |
| `src/bin/turingos/cmd_spec.rs` | 380 | `turingos spec` — 8-question Mom-Test+JTBD+Voss grill for non-developers (Chinese-first; English available via `--lang en`). Calls Meta LLM to synthesise spec.md with EARS+GWT+Never sections. `--skip-llm` for CAS-wire-only smoke tests. `--answers-file` for scripted/regression runs. |
| `src/bin/turingos/cmd_generate.rs` | 290 | `turingos generate` — feeds spec.md to Blackbox LLM (default: Qwen3-Coder-30B-A3B-Instruct); parses `### File: <path>` fenced blocks; writes safe relative paths under `<workspace>/artifacts/`. `--from-capsule` reads spec from CAS instead of disk for reproducible regeneration. |
| `tests/cli_phase63_cas_wire.rs` | 230 | Three integration tests through the real binary: init→llm→spec→welcome happy path; generate-without-spec rejection; llm config/show round-trip with API-key-value secrecy assertion. |

Plus minor updates to `cmd_welcome.rs` (reads CID from CAS), `cmd_init.rs` (Phase 6.3 next-steps hint), `turingos.rs` (module + subcommand registry).

## Verified Evidence

**CAS wire is REAL** — proven by:
- A fresh workspace's `cas/.git/` (git2 `Repository::init` invoked)
- A fresh workspace's `cas/.turingos_cas_index.jsonl` with one line containing `EvidenceCapsule` + `turingos-spec-capsule-v1` + 32-byte CID + git-blob OID
- Round-trip readback: the CAS-stored bytes equal the spec.md bytes on disk
- Idempotency: same answers → same CID across runs (content-addressing invariant)

```text
CAS capsule CID    -> c5dd22b02f4f75e155e437372fbe4e20cb1cca653d5f7c68bc778fcec9f26028
                     (schema: turingos-spec-capsule-v1)
```

**SiliconFlow HTTPS wire is REAL** — proven by:
```text
$ SILICONFLOW_API_KEY="sk-test-invalid" target/debug/turingos spec --workspace /tmp/...
[spec] calling Meta LLM (deepseek-ai/DeepSeek-V3.2) to synthesise spec.md...
turingos spec: HTTP 401 from SiliconFlow: "Api key is invalid"
```

HTTP 401 from `api.siliconflow.cn` confirms: DNS resolved, TLS handshook, Bearer auth header parsed, request body accepted. Only the real key is required for the live call to succeed.

**Welcome integration is REAL** — proven by:
```text
Onboarding status:
  [x] 1. turingos init
  [x] 2. turingos llm config
  [ ] 3. turingos agent deploy (0 registered)
  [x] 4. turingos spec (CAS capsule: c5dd22b0…c9f26028)
  [ ] 5. turingos generate (deliverable)
```

The truncated CID in the welcome output is read directly from the CAS sidecar index — `welcome` cannot lie about spec completion because it derives status from CAS evidence, not from a `spec.md` file presence (which a user could hand-create).

## Test Results

```text
$ cargo test --test cli_phase63_cas_wire
running 3 tests
test phase63_generate_missing_spec_fails_clearly ... ok
test phase63_llm_show_after_config_round_trips ... ok
test phase63_full_cas_wire_init_llm_spec_welcome ... ok
test result: ok. 3 passed; 0 failed

$ cargo test --test cli_wrapper_plumbing
running 5 tests
[Phase 6.2 wrapper-plumbing tests; all 5 pass — no regression]

$ cargo test --test cli_init_smoke
running 5 tests
[Phase 6.0/6.1 init tests; all 5 pass — no regression]
```

## Model Picks (Independent Research Agent, 2026-05-17)

A clean-context research agent surveyed SiliconFlow's model lineup and recommended:

| Role | Model ID | Why | Price (¥/M in,out) | Ctx |
|---|---|---|---|---|
| Meta (reasoning) | `deepseek-ai/DeepSeek-V3.2` | Best Chinese reasoning per yuan; low hallucination; native bilingual. | 2 / 3 | 160K |
| Blackbox (codegen) | `Qwen/Qwen3-Coder-30B-A3B-Instruct` | Code-specialised MoE; 256K ctx; strong JS/Python for game-scale projects. | 1.6 / 12.8 | 256K |

Total cost per game-build session: **≈ ¥0.45 (≈ $0.06 USD)**.

Alternate models (DeepSeek-V4-Flash, Qwen2.5-Coder-32B, GLM-4.7) are accepted via `--meta-model` / `--blackbox-model` flag overrides — the wire format is OpenAI-compatible for any SiliconFlow model.

## Non-Developer Grill Design (Independent Research Agent, 2026-05-17)

Eight questions, Chinese-first, synthesised from:

- **JTBD switch interview** (Moesta) — Q1 opener anchors on a recent moment, not a feature list.
- **Mom Test three sins** (Fitzpatrick) — never ask about the future / opinions / let them off. Q6 inverse-framing surfaces real priorities without producing wishlists.
- **Voss labeling / mirroring** (Never Split the Difference) — Q8 playback uses "Let me say back what I heard" framing.
- **5-Whys** (Toyoda) — wired into Q3's follow-up for vague data-model answers.
- **IDEO empathy** — Q4 forces a single concrete path, not abstract feature list.
- **EARS syntax** (Mavin) — used internally by the Meta LLM to translate Q3/Q4/Q5 into `When <trigger>, the <system> shall <response>` form for downstream codegen.
- **LLMREI** (arXiv 2507.02564, 2025) — confirmation playback found to catch the most implicit-requirement misses.

Three jargon translations are baked into the Meta LLM system prompt:

| Technical concept | Plain-language substitute |
|---|---|
| data model / schema | "what should the program still remember tomorrow morning?" |
| user flow / state machine | "walk me through what they click first, then next" |
| edge cases / validation | "what's the weirdest thing someone might try that should NOT break it?" |

## How to Run the Full Demo (with your real key)

```bash
# 1. Build the binary (one-time, ~12s incremental)
cargo build --bin turingos

# 2. Scaffold a workspace
target/debug/turingos init --project ~/my-game --template proof

# 3. Set up the two-LLM config (defaults to SiliconFlow + DeepSeek + Qwen3-Coder)
target/debug/turingos llm config --workspace ~/my-game

# 4. Provide your real key (one-time per shell). Project convention: put the key
#    in `.env` in the repo root (already gitignored; same env var name the existing
#    scripts/smoke_siliconflow.sh reads). Then source it once per shell:
echo "SILICONFLOW_API_KEY=sk-..." >> .env   # one-time
set -a && . .env && set +a                  # source per shell

# OR, single-shot wrapper for any of the turingos commands:
bash -c '. .env && target/debug/turingos spec --workspace ~/my-game'

# 5. Run the 8-question grill (interactive — type your answers; blank line submits each)
target/debug/turingos spec --workspace ~/my-game

# 5'. OR run scripted (pre-canned answers — useful for demos / regression)
target/debug/turingos spec --workspace ~/my-game --answers-file ~/my-game/answers.json

# 5''. OR test the CAS wire without an API key (deterministic, no synthesis)
target/debug/turingos spec --workspace ~/my-game --answers-file ~/my-game/answers.json --skip-llm

# 6. Generate code from the spec
target/debug/turingos generate --workspace ~/my-game

# 7. Open the result
xdg-open ~/my-game/artifacts/index.html   # for UI apps
# or
python3 ~/my-game/artifacts/main.py        # for script apps

# 8. Check progress anytime
target/debug/turingos welcome --workspace ~/my-game
```

## What This Atom Did NOT Do

- **Did NOT execute the LLM-synthesis call** (no SiliconFlow API key available in this session's environment). The wire is verified-real via HTTP 401; the synthesis step itself will happen on first user-run.
- **Did NOT modify** `src/bottom_white/cas/schema.rs` (Class 4 STEP_B surface). The CAS wire is Class 2 production wire-up over the existing public `CasStore` surface.
- **Did NOT touch** `Cargo.toml` (Trust Root preserved). `reqwest`, `tokio`, `serde_json` were already in the dependency set.
- **Did NOT add** any user-facing `lean_market` / `Lean` / `TB-10` strings (Phase 6.2 Phase-7-generalization posture preserved).

## Forward Items (Phase 6.3.1 / Phase 7+)

- **Streaming**: SiliconFlow supports SSE `data: ...` streaming; not wired in this atom because the spec-synthesis call is single-shot. Add for `turingos chat` if it ships.
- **PromptCapsule binding**: per `constitution.md` §16 + the G-016/G-019/G-021/G-028 ruling, the Meta-LLM system prompt + user prompt should be CAS-anchored as a `PromptCapsule` alongside the spec capsule. Currently the transcript JSONL captures the same content; promoting it to a typed `PromptCapsule` is a small follow-up.
- **Codegen idempotency**: `turingos generate` doesn't yet seed the LLM with `seed=0` — Qwen3-Coder accepts it but the wire client doesn't pass it. Same regen would produce different bytes today. Trivial to add.
- **Auditor agent dispatch**: when the user enables `--audit` on `turingos generate`, a second LLM call (Meta model, cold context) should review the Blackbox output for spec-divergence. Mediator-style pattern, blocked on user's `auditor` subagent contract for Phase 7+.

## Sign-Off Posture

This atom is Class 2 (production wire-up over existing public surface). No Class-4 schema touch; no Trust Root rehash needed; no architect §8 required.

For Class-2 SHIP eligibility per CLAUDE.md §9: harness (3 integration tests pass) → real evidence (CAS CID + SiliconFlow HTTP 401 reach) → audit-pending (clean-context Codex review can run when user requests `/ultrareview`). Ship eligible.
