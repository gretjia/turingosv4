# Phase 6.3.x Universality Test Campaign — Log

**Started**: 2026-05-18 (ts 1779111375)
**Architect mandate**: 10-hour autonomous sweep; design persona matrix via independent research agents, run multi-wave testing, fix safe issues found
**Branch**: codex/tisr-phase6-3-x-grill-driven HEAD 3e0fa79c
**API key source**: /Users/zephryj/projects/turingosv3/.env (SILICONFLOW_API_KEY)

## Dispatched agents (Round 1, parallel)

| Agent | Role | Background ID | Status |
|---|---|---|---|
| W9 baseline (corrected env path) | Real-LLM E2E with Mrs Chen persona | ae42540e8713b237e → re-dispatched | RUNNING |
| Research-A persona dimensions | HCI + Karpathy + red-team literature mining | a99548df51d3357b2 | RUNNING |
| Research-B internal failure modes | Audit + source + research doc mining | a07b3c3c4aed73122 | RUNNING |
| Research-C adversarial scenarios | Prompt injection + edge-case design | add24036faac83be0 | RUNNING |

## Planned waves (post-research synthesis)

1. **Wave 1 — Baseline easy** (5 personas): confirm happy path across demographics
2. **Wave 2 — Medium** (5-8 personas): varied domains (cooking app / Lean prover / kid game / B2B SaaS / etc.)
3. **Wave 3 — Hard** (5-8): terse, multilingual, off-topic-prone, IT-illiterate
4. **Wave 4 — Adversarial** (10-15): prompt injection, gibberish, abusive, slot manipulation, goal hijacking
5. **Wave 5 — Edge** (5-8): empty / 10k-char / emoji-only / language switching / encoding tricks

## Concurrency model

- One persistent backend `turingos_web` on 127.0.0.1:8080 (orchestrator-started after W9)
- Within a wave, multiple agents hit same backend via curl POST /api/spec/turn (different session IDs, isolated via AppState.sessions HashMap)
- Browser MCP reserved for 1-2 sanity-check sessions per wave (visual regression)
- Each agent writes per-persona evidence to wave subdir

## Constitution discipline

- No Class-4 surface edits (per memory + AGENTS.md §6)
- No git commit/push without explicit architect prompt
- Found defects: Class 0/1/2 may dispatch fix; Class 3/4 candidates recorded as deferred
- Trust Root NOT touched; genesis_payload.toml NOT modified

## Update log

### 2026-05-18 W9-restart-T+10min
- Research-A returned (227s): 12 personas × 12 dims, 4 difficulty bands. Archived to `research/RESEARCH_A_persona_dimensions.md`.
- Research-C returned (201s): 15 adversarial scenarios with turn-by-turn answers. Archived to `research/RESEARCH_C_adversarial_scenarios.md`.
- Research-B returned (237s): **P0 defect found** — `src/web/spec.rs:693-702` `extract_slots` uses wrong slot vocab (Researcher-C draft names instead of canonical). Will pollute every web session's WS broadcast. W10-R1 missed this. Archived to `research/RESEARCH_B_failure_inventory.md`.
- Still pending: W9 baseline re-dispatch (correct env path), Research-D (model sweep + S3.0 framework)
- Architect note: added Wave 6 (model-size sweep) + S3.0 falsifiability deliverable to todos

### Plan revision: dispatch a fix agent for the D1 P0 defect AFTER W9 baseline completes (so baseline captures the as-shipped behavior), BEFORE Wave 1 (so waves run against fixed code and produce clean evidence).

### 2026-05-18 W9-restart-T+45min — Research-D + Fix dispatches

- Research-D returned (407s): SiliconFlow catalog (V4-Flash + Qwen3.6 + GLM-5.1 + Kimi-K2.6 — defaults are mid-gen). Verified CLI surface (flags not subcommands). **Critical**: `complete --strict-json` does NOT strip `<think>` blocks — Wave 6 thinking-mode models would all fail at parse for uninformative reason. Defined 10 S-predicates for "S3.0 lands". 5 recommended Phase 6.3.y atoms (A1-A5). Archived to `research/RESEARCH_D_*.md`.
- Synthesized `MATRIX_SPEC.md` (40 planned sessions across 6 waves) + `WAVE_RUNNER_TEMPLATE.md` (verified API shape: `POST /api/spec/turn` with `user_answer: null` bootstrap, no separate /start)
- Dispatched F1 fix agent for the D1 web extract_slots vocab defect (Class 2)
- Dispatched F2 fix agent for the parser hole in `complete --strict-json` (A1; Class 2)

### 2026-05-18 W9-restart-T+50min — F2 landed
- F2 done (481s, 9 new tests, +23/-17 LOC on cmd_llm.rs). Wired existing `sdk/protocol.rs::strip_think_blocks` into both `complete --strict-json` and `triage`. Old asymmetric `strip_thinking_wrapper` removed. All `grill_*` + `cmd_llm_*` + `sdk::protocol` tests green.
- F2 self-flagged judgment call #2: ran `cargo build --bin turingos` once → binary mtime updated. W9 baseline parent PID 41146 unaffected (held original inode); new subprocess spawns after that time would pick up F2-fixed binary. For default-config models (DeepSeek-V3.2 non-thinking, Qwen3-Coder-30B thinking-off), F2 strip is a no-op so baseline purity preserved. Noted.

### 2026-05-18 W9-restart-T+75min — F1 landed + escalation
- F1 done (965s, 2 new tests on `web::spec::tests` module, +89/-14 LOC). Used `turingosv4::runtime::grill_envelope::CANONICAL_SLOTS` import (library-crate-qualified because `web/spec.rs` is `#[path]`-included from bin). All web::spec tests + cli_web_spec_smoke (59/59) + web_spec_turn_endpoint (65/65) tests green.
- **CRITICAL**: F1 spawn-task flagged F3 — same junk vocab in `build_coverage_summary` at `src/web/spec.rs:1286-1315`. This one IS on the LLM-facing path (not just UI) — `coverage_state` HashMap is keyed by canonical names but `build_coverage_summary` iterates draft names → **LLM never sees actual coverage** → forced to ask redundant questions, may hit 15-turn ceiling without ever emitting done=true.
- Orchestrator's earlier claim "LLM-facing flow is unaffected" was wrong. F3 dispatched immediately (same shape as F1 fix).
- F1 noted 15 pre-existing workspace test failures (boot::tests, cas::store::tests, fc_alignment_conformance, runtime::evidence_capsule, constitution_router_buy_with_coin) — unrelated to this campaign. Ignore.

### Currently running (orchestrator standby)
- F6 backend error-handling cluster fix (60 min budget; started T+135min)
- F7 Software 3.0 S2 prompt experiment (45 min budget; started T+135min, writes only — A/B test deferred to orchestrator after F6)

### 2026-05-18 W9-restart-T+90min — F3 done + F4 dispatched + W9 baseline FAIL

- F3 done (324s, 2 tests on web::spec::tests, +F1 import reused). Coverage summary now uses CANONICAL_SLOTS (all 8) — LLM-facing fix.
- **W9 baseline RE-RUN FAIL** with 3 NEW gaps surfaced:
  - **GAP-1 (P0 ship-blocker)**: `web/spec.rs::build_web_turn_prompt_json` drops meta-prompt → web turn-1 returns HTTP 500 `shellout_failed`. CLI same path includes it correctly. ~20 LOC fix.
  - **GAP-2**: `turingos spec audit` subcommand doesn't exist (charter §6 step 4 assumes it). ~250 LOC.
  - **GAP-3**: `--offline` replay flag doesn't exist. ~80 LOC.
  - **Compensation**: CLI driven mode worked end-to-end (335s, 12 turns, DeepSeek-V3.2, session capsule + 23 turn capsules, terminate_reason=predicate_double_fail).
- W10-R1 static audit COMPLETELY MISSED GAP-1 — validates user's intuition that universality testing requires real-world inputs, not just static audit. Key S2 data point.
- F4 dispatched immediately for GAP-1.

### 2026-05-18 W9-restart-T+115min — F4 done + F5 regression + smoke confirms web pipe

- F4 done (427s): meta-prompt now prepended as messages[0] in web shellout. 3 new tests + RCA + invariant test recommendation.
- F4 RCA discovery: web was assuming `--meta-prompt` flag injects prompt server-side. Actually it's informational only (sha256 → capsule). Contract documentation problem.
- **F5 regression**: F4's full-path passed to `--meta-prompt` got workspace-prefixed AGAIN by cmd_llm → double-prefix path. Web returned 500 again on smoke test.
- F5 dispatched (401s): pass workspace-relative literal to `--meta-prompt`, keep F4's full-path read separate. End-to-end smoke 200 OK with real Chinese question.
- Backend running PID 54674 with all 5 fixes (F1+F2+F3+F4+F5).

### 2026-05-18 W9-restart-T+135min — Wave 1 results + new defect cluster

**Wave 1 results: 1 PASS / 2 FAIL** (3 personas in parallel via curl):

| Persona | Verdict | Key observation |
|---|---|---|
| W1.3 P4 minimalist | **PASS** | Grill correctly identified starvation, terminated turn 6 with no spec (correct behavior). No hallucination. 20 CAS objects clean. |
| W1.1 Mrs Chen | **FAIL** | 7 turns, 0/7 required slots covered, terminated=true with NO spec_capsule_cid. Empty-response short-circuits on turns 2/6/7. LLM stuck in Voss-mirror loop, only ever extracted `job`. |
| W1.2 P1 backend | **FAIL** | 5 shellout failures of 12 attempts. Slot stuck at `["job"]` for 5 turns despite explicit technical anchor info. Lang drift to English on turn-4 mirror. spec_capsule_cid null. |

**8 new defects surfaced** (D8-D15) requiring fix or documentation:

- **D8** Shellout flakiness: `ok=false content=<empty>` on 5/12 attempts (W9 also saw it in CLI driven mode). Possibly SiliconFlow transient or subprocess pipe issue.
- **D9** Slot-progression stall: LLM keeps extracting only `["job"]` despite multi-slot user answers. **Prompt-level** issue (Voss-mirror restatement loop, conservative extraction). F7 target.
- **D10** Silent SpecCapsule loss: predicate_double_fail abort doesn't write spec capsule. Web layer divergent from CLI partial_session path. F6 target.
- **D11** Language drift: P6 lang predicate doesn't gate synthesis output. Mirror playback emitted in English.
- **D12** covered_slots state regression: backend-side state oscillates `['job']`→`[]`→`['job']`. Monotonicity P4 doesn't catch because it's a separate state. F6 target.
- **D13** Triage failure clears state: when triage LLM unparseable, backend zeroes all fields silently (HTTP 200 not 500). F6 target.
- **D14** terminated=true without spec_capsule_cid: see D10.
- **D15** Concurrency stall: parallel sessions hitting same backend caused 9-min stuck curl on Mrs Chen's first attempt. Possible subprocess pipe deadlock under load. Probably wait-list ordering issue.

**Insight**: F1-F5 fixed the "obvious" defects (vocab + meta-prompt + parser). Wave 1 surfaced **second-order defects** that only manifest in real LLM runs:
- Defects exposed by LLM unreliability (D8 shellout flakiness)
- Defects in error-handling paths that the audit couldn't see (D10/D12/D13/D14)
- Defects in prompt-as-program semantics (D9/D11)

This validates the campaign's premise: real-input testing surfaces what static audit misses.

**Decision**: STOP Wave 2-6 dispatch until F6+F7 land. Re-run Wave 1 with fixes to confirm before continuing.

### Pending W9 baseline outcome
- If W9 returns PASS (cleanly) with Mrs Chen completing — confirms shipped binary works despite the LLM-facing coverage bug (LLM compensated somehow). Move forward with waves.
- If W9 returns PARTIAL (15-turn ceiling forced terminate, never emitted done) — that IS the F3 bug's fingerprint. Document, dispatch F3, then re-run W9 with fixed binary to confirm fix.
- If W9 returns FAIL (other reason) — diagnose before continuing.
