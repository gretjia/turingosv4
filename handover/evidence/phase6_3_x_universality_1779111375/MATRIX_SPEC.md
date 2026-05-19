# Universality Test Matrix — Phase 6.3.x

**Synthesized**: 2026-05-18 by orchestrator (post 4-agent research)
**Total sessions planned**: ~40 (Waves 1-6), parallelism 4-way, ~2h wall clock
**API spend budget**: ~5¥ at SiliconFlow rates (negligible)

## Architecture / orchestration

- **One persistent backend** `turingos_web` on 127.0.0.1:8080 started by orchestrator after W9 baseline + F1+F2 fixes
- Within a wave: 4 agents in parallel hit same backend via **curl POST /api/spec/turn** (different session IDs; AppState.sessions HashMap isolates)
- **Browser MCP** reserved for 1 sanity-check session per wave (visual regression)
- Each agent writes per-persona evidence to `handover/evidence/phase6_3_x_universality_1779111375/wave<N>/<persona_id>/`

## Test runner per-persona contract

Each persona agent receives:
- Persona spec (verbatim from Research-A or Research-C)
- Turn-by-turn answer sequence (where applicable; improvise otherwise)
- Endpoint: `http://127.0.0.1:8080/api/spec/turn`
- Workspace path (fresh per session)
- Expected vs suspected behavior + observable signatures

Produces per-persona:
- `session_log.jsonl` — every POST request + response captured
- `verdict.json` — PASS/PARTIAL/FAIL + per-criterion + observed metrics
- `mrs_chen-style_answers.json` — answer sequence used (reproducibility)
- `final_spec.md` — if synthesis completed
- `cas_walk.txt` — turingos spec audit on the session

## Wave plan

### Wave 1 — Baseline easy (3 sessions; Mrs Chen + P1 + P4)

Goal: confirm happy-path works on shipped code. If any of these fail catastrophically, halt the campaign.

| Slot | Persona | Source | Dimensions exercised |
|---|---|---|---|
| 1.1 | Mrs Chen (Tetris/no-network) | charter §6 + Research-A | low expertise, terse-medium, single-domain |
| 1.2 | P1 资深后端 (webhook retry) | Research-A | high expertise, jargon-dense, terse |
| 1.3 | P4 一句话 minimalist | Research-A | extreme terseness |

S-predicates exercised: S1, S9 (turn-2 diversity), S10 (cost baseline)

### Wave 2 — Medium real-world (4 sessions)

| Slot | Persona | Source | Dimensions |
|---|---|---|---|
| 2.1 | P2 迷茫产品经理 | Research-A | low clarity, drifting |
| 2.2 | P5 中英夹杂海归 | Research-A | code-switching |
| 2.3 | P7 繁体中文台湾 | Research-A | zh-Hant + regional vocab |
| 2.4 | P8 语音转文字噪声 | Research-A | input noise |

S-predicates: S5 (degrades gracefully on noise)

### Wave 3 — Hard (4 sessions)

| Slot | Persona | Source | Dimensions |
|---|---|---|---|
| 3.1 | P3 老板式甩需求 | Research-A | one-shot dump |
| 3.2 | P6 emoji + 网络用语 | Research-A | slang + emoji noise |
| 3.3 | P11 反问/哲学型 | Research-A | high off-topic rate |
| 3.4 | P12 暴躁用户 | Research-A | emotional + light abusive |

S-predicates: S5 (grace under pressure)

### Wave 4 — Adversarial (8 scenarios, sanity-first order)

| Slot | Scenario | Source | Category |
|---|---|---|---|
| 4.1 | S12 Pure emoji | Research-C | language / triage-evade |
| 4.2 | S6 Monotonic repeat | Research-C | slot-manip / termination |
| 4.3 | S4 One-shot all-slot claim | Research-C | slot-manip |
| 4.4 | S14 Forced-termination spam | Research-C | termination |
| 4.5 | S2 Fake-JSON smuggling | Research-C | prompt-inj |
| 4.6 | S1 Direct override (zh) | Research-C | prompt-inj |
| 4.7 | S3 Role inversion | Research-C | prompt-inj |
| 4.8 | S15 Concurrent submission | Research-C | state-timing (HTTP-level) |

S-predicates: S5 (adversarial grace)

### Wave 5 — Edge (4 scenarios)

| Slot | Scenario | Source | Vector |
|---|---|---|---|
| 5.1 | S10 Code-switch + Unicode | Research-C | ZWSP, RTL marks |
| 5.2 | S11 Cantonese + Traditional | Research-C | low-resource zh variant |
| 5.3 | S9 Coherent gibberish | Research-C | triage semantic-vs-surface |
| 5.4 | S5 Wrong-slot misdirection | Research-C | slot ledger semantic routing |

### Wave 6 — Model sweep (5 bracket pairs × 2 anchor personas = 17 sessions)

**Prerequisite**: F2 (think-strip fix A1) must ship first. Otherwise thinking-mode models fail at parse for an uninformative reason.

Anchor personas (run each across all model configs):
- **A**: Mrs Chen (representative happy-path)
- **B**: P3 老板式甩需求 (representative stress case)

Brackets:

| # | Pair | Anchor count | Tests |
|---|---|---|---|
| **B1** | Meta envelope bound | A+B | (Qwen3-32B, Qwen3-Coder-30B-A3B) × 2 personas = 4 sessions |
| **B2** | Meta semantic bound | A+B | (DeepSeek-V4-Flash, DeepSeek-V3.2) × 2 = 4 |
| **B3** | Meta capability ceiling | A+B | (Kimi-K2.5, MiniMax-M2.5) × 2 = 4 |
| **B4** | Blackbox cost-floor | A+B | (Qwen3-8B, Qwen3-14B) × 2 = 4 |
| **B5** | Reasoning-trace falsifier | A only | DeepSeek-V3.1-Terminus think-on × 1 (expected fail-after-fix; pass-before-fix) = 1 |
| **Total** | | | **17 sessions** |

S-predicates: S3, S4, S8 (tight bound)

## Pre-wave dependencies

| Pre-req | What | Why |
|---|---|---|
| W9 baseline complete | Mrs Chen happy-path validated | Sanity that shipped code works end-to-end |
| F1 web extract_slots fix | Class 2 patch + test | Otherwise every wave's WS open_slots is polluted |
| F2 strict-json think-strip | Class 2 patch + test | Otherwise Wave 6 brackets fail for uninformative reason |

## Failure handling

- **Class 0/1/2 defect found in a wave** → dispatch fix agent; mark scenario re-test pending
- **Class 3 candidate** (e.g., touches CAS schema, read_set scoping) → document, defer to architect ratification; NOT auto-fixed
- **Class 4 candidate** (touches Trust Root, sequencer, typed_tx) → document only; STOP campaign and write architect note
- **Session crash / 5xx storm** → halt wave, dispatch diagnostic agent

## Metrics collected (every session)

Per Research-D Part I "Mechanical Capability Metrics":
- envelope_parse_rate, slot_coverage_at_term, turn_count_to_term
- triage_class_accuracy (where applicable), mean_tokens_per_response, mean_latency_ms
- p1_retry_rate, vocab_violation_rate, monotonicity_violation_rate
- cost_per_session_¥

## Final deliverables (post-Wave 6)

1. `final/UNIVERSALITY_REPORT.md` — per-wave pass/fail/partial, defect inventory, fixes applied
2. `final/S3_FALSIFICATION_MATRIX.md` — 10 S-predicates × evidence verdicts
3. `final/MODEL_SWEEP_RESULTS.md` — Wave 6 bracket matrices with mechanical metrics
4. `final/PHASE_6_3_Y_CANDIDATES.md` — recommended next atoms (A1-A5 from Research-D + any new ones)
5. Memory update: add findings to `~/.claude/projects/-Users-zephryj/memory/`
