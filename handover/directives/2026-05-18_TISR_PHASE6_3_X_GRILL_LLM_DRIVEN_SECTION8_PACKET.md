# TISR Phase 6.3.x — Software 3.0 LLM-Driven Grill: Separate Charter Section 8 Packet

**Date drafted**: 2026-05-18
**Driver environment**: Mac Studio (local, macOS Darwin 25.2.0)
**Predecessor**: Phase 7 Web MVP shipped 2026-05-18 (PR #6 merged on main, HEAD c2f20e4b → branch codex/tisr-phase7-web at 86e82406)
**Research basis**: 3 parallel Opus xhigh design docs committed at 86e82406
- Researcher A: "Software 3.0 Native" (top-down, Karpathy frame) — `handover/research/grill_software_3_0_2026-05-18/researcher_a/DESIGN.md`
- Researcher B: "TuringOS Substrate Fit" (bottom-up, constitutional) — `handover/research/grill_software_3_0_2026-05-18/researcher_b/DESIGN.md`
- Researcher C: "Extensibility Forward" (CCS / agent.toml abstraction) — `handover/research/grill_software_3_0_2026-05-18/researcher_c/DESIGN.md`

**Architect scope decision** (ratified 2026-05-18 in chat):
- Take **B's full implementation envelope** (zero Class-4 surface mutation, ~750 LOC)
- Adopt **B's grill-specific capsule schema naming** (NOT C's generic `turingos-ca-*`) — reversibility argument
- Adopt **A's predicate suite** (6 predicates) and **A's dual-gated termination**
- Adopt **A's `turingos llm complete --strict-json` CLI shape**
- **DEFER** A's Blackbox triage classifier to optional atom-1.5 (v1 ships without it)
- **REJECT** C's CCS / agent.toml manifest abstraction layer ("two real instances, then ratify" — premature generalisation risk)
- **REJECT** C's generic `/api/converse/<agent_id>/...` endpoints + generic `<tos-conversation>` Web Component (await 2nd real grill instance)

---

## §1 Scope

This packet ratifies TISR Phase 6.3.x — the **Software 3.0 native LLM-driven grill** that
transforms the Phase 6.3 static 8-question template into a turn-by-turn LLM-driven
interview, while leaving the Phase 6.3 synthesis path (spec.md generation) byte-identical.

### What changes (atomic summary)

| Surface | Phase 6.3 (today) | Phase 6.3.x (this packet) |
|---|---|---|
| Questions | 8 hardcoded `Vec<&'static str>` in `cmd_spec.rs::canonical_questions(lang)` | LLM-generated each turn via `turingos llm complete` |
| LLM calls per session | 1 (synthesis only) | N+1 (N turns + 1 synthesis); N ∈ [4, 15] |
| Termination | `i == 8` | Dual-gated: LLM `done=true` + kernel slot-coverage predicate |
| Per-turn evidence | None | PromptCapsule + AttemptTelemetry + EvidenceCapsule (3 CAS objects) |
| Final synthesis | `wrap_spec_md(...)` over fixed-8 answers | **UNCHANGED**: same `wrap_spec_md` over reconstructed answers list |
| Spec capsule schema | `turingos-spec-capsule-v1` | **UNCHANGED** |
| Web endpoint | `POST /api/spec/submit` (bulk 8-answer) | `POST /api/spec/turn` (per-turn); old endpoint kept as legacy alias |
| Frontend | Static 8-question stepper | Driven mode with LLM-generated next question; URL param `?mode=driven` |

### Phase 6.3.x ratifies six artifact additions

1. **New CLI subcommand action: `turingos llm complete`** (extending `src/bin/turingos/cmd_llm.rs`, currently 302 LOC)
   - One-shot chat-completion call wrapping existing `siliconflow_client::chat_complete`
   - Stateless: reads message array from `--prompt-file <p>` (or stdin if `-`)
   - Validates LLM output against turn-payload JSON schema (Researcher A §4.2 envelope)
   - Writes PromptCapsule + AttemptTelemetry to CAS (existing primitives, no new ObjectType)
   - Prints stdout JSON line `{ok, content, parsed_envelope, usage, prompt_capsule_cid, attempt_telemetry_cid, ...}`

2. **Two new EvidenceCapsule schema_id strings** (extending `src/bin/turingos/spec_capsule.rs`, currently 163 LOC)
   - `turingos-spec-grill-turn-v1` — per-turn rollup (prompt_capsule_cid + attempt_telemetry_cid + predicate_verdicts + parent_turn_cid + user_answer_cid)
   - `turingos-spec-grill-session-v1` — session rollup (turn_cids[] + final_spec_capsule_cid + termination_reason + session_id + total_turns)
   - Both reuse existing `ObjectType::EvidenceCapsule` — **zero new ObjectType variant**

3. **6 turn predicates** (new module `src/runtime/grill_predicates.rs`)
   - P1 `schema_parse_ok`: output parses cleanly into turn-payload JSON; all required fields present
   - P2 `kind_is_question_or_terminator`: top-level `done` field exists and is boolean; if `done=false` then `question` is non-null
   - P3 `slots_in_canonical_vocab`: every `covered_slots[]` / `open_slots[]` element ∈ `{job, anchor, memory, first_run, robustness, scope, acceptance, mirror}`
   - P4 `covered_slots_monotonic`: `covered_slots[turn N+1] ⊇ covered_slots[turn N]`
   - P5 `turn_index_bounded`: `turn_index ≤ 15`; turn 16+ refused
   - P6 `question_non_empty_lang`: if `done=false`, `question_text.len() ≥ 8` AND language matches `--lang` (Han-script ratio ≥ 0.5 for zh, ASCII ratio ≥ 0.8 for en)
   - **Termination predicate** (kernel-side, separate from P1-P6): required slot subset
     `{job, anchor, memory, first_run, robustness, scope, acceptance}` must be ⊆ `covered_slots` AND `confidence ≥ 0.8` AND `turn ≥ 4` before LLM's `done=true` can terminate

4. **`turingos spec --mode driven` flag** (extending `src/bin/turingos/cmd_spec.rs`, currently 581 LOC)
   - Adds `--mode {static|driven}` flag (default = static for backward compat)
   - In driven mode: loops, each iteration shells out to `turingos llm complete`,
     runs P1-P6, updates coverage_state, on termination invokes existing
     Phase 6.3 synthesis path with reconstructed answers list
   - Static mode (default): unchanged from Phase 6.3
   - Hard ceiling: 15 turns; on hit, kernel forces termination with `partial_session=true`

5. **Web endpoint: POST /api/spec/turn** (extending `src/web/spec.rs`, currently 628 LOC)
   - Accepts: `{session_id: string, user_answer: string, lang?: "zh"|"en"}` (lang only on turn 1)
   - Returns: `{turn_index, question_text?, covered_slots, open_slots, confidence, done, playback?, terminated}`
   - AppState gains `sessions: Arc<Mutex<HashMap<SessionId, GrillSession>>>`
     where `GrillSession = {turn_count, coverage_state, last_3_turns, terminated, parent_turn_cid, lang}`
   - WebSocket `WsBroadcastMsg` extended with `SpecTurnAdvanced{session_id, turn_index, question_text}` and `SpecGrillComplete{session_id, spec_capsule_cid}`
   - Existing `POST /api/spec/submit` and `GET /api/spec/questions` retained as legacy mode aliases (called when `?mode=static` or no mode param)

6. **Frontend `<tos-spec-grill>` driven mode** (extending `frontend/src/components/spec-grill.ts`, currently 375 LOC)
   - URL param `?mode=driven` toggles new behavior
   - State machine: `idle → awaiting_first_answer → awaiting_next_question → awaiting_user_answer → … → playback → confirmed → complete`
   - Each user submit POSTs `/api/spec/turn`, renders returned `question_text`
   - On `done=true` + `playback` present: shows playback confirmation step
   - On 5xx or three consecutive predicate failures: falls back to legacy 8-question static mode with one-time toast notification

### What Phase 6.3.x explicitly does NOT ratify

- **ConversationalAgent / CCS / agent.toml manifest abstraction** (Researcher C's proposal; deferred to Phase 6.3.y after 2nd real grill instance materializes)
- **Generic `/api/converse/<agent_id>/...` endpoint family** (Researcher C's proposal; deferred)
- **Generic `<tos-conversation>` Web Component** (Researcher C's proposal; deferred)
- **Blackbox triage classifier** (Researcher A §2.4 §5.3; deferred to optional atom-1.5; may be added once base driven mode lands and a real abuse-vector demo case exists)
- **Multi-LLM concurrent calls per turn** (Meta + Blackbox in parallel); not in v1
- **Custom completion predicates beyond P1-P6 + termination predicate**
- **New typed_tx variants** (sequencer admission unchanged)
- **New ObjectType variants** in `src/bottom_white/cas/schema.rs`
- **New sequencer admission rules**
- **PromptCapsule schema mutation** (Class-4 architect-pinned 7 fields; unchanged)
- **ArchitectAI / Veto-AI in-runtime hooks** (Phase 11+ deferred per CONSTITUTION_GAP_ANALYSIS_2026-05-07.md:169 "likely permanent" — architectural choice, not debt)
- **Cooking / Lean Canvas / code review grills** (await this packet ship as input signal)
- **Per-provider abstraction layer** (SiliconFlow only; no openai/anthropic provider swap)

---

## §2 FC Mapping

### Touched / extended FC nodes

- **FC1-N4 (Q_t → rtool)**: per-turn message-array assembly reads `coverage_state_summary` deterministically computed from prior turn outputs; this is a derived view per Art. 0.2 (no shadow ledger)
- **FC1-N5 (rtool / scoped context)**: PromptCapsule.read_set per turn includes only `{session_id, last_3_turn_cids, canonical_slot_table_cid, system_prompt_template_hash}`; enforces Art. III.2 progressive-disclosure shielding; turns N-4 and earlier are NOT in agent's read access
- **FC1-N7 (Agent externalized output)**: each LLM call = one externalized attempt per CLAUDE.md §6 explicit clause "output used in future prompt context"
- **FC1-N9 (predicate / oracle)**: P1-P6 fire per turn before output influences next turn; predicate failure → retry-up-to-1 then halt (Researcher A §5.1)
- **FC1-N10 (wtool / Sequencer)**: grill turns do NOT submit WorkTx; CAS-only evidence path per Researcher B §1.3
- **FC1-N12 (L4 accepted)**: turn EvidenceCapsule + AttemptTelemetry both land in CAS; FC1 invariant satisfied via `explicitly_anchored_capsule_attempt_count` bucket per CLAUDE.md §3.1
- **FC2-N16 (replay)**: session is replayable from session-rollup capsule + per-turn capsule chain; LLM is NOT re-invoked at replay (Researcher A §3.3 + Researcher B §6 design)

### New invariants for Phase 6.3.x

| Inv ID | Statement | Test witness |
|---|---|---|
| I-1 | Every accepted turn writes exactly 1 PromptCapsule + 1 AttemptTelemetry + 1 EvidenceCapsule, CID-linked via turn capsule body | `tests/grill_turn_capsule_*.rs::three_capsules_per_accepted_turn` |
| I-2 | `AttemptTelemetry.prompt_context_hash == PromptCapsule.prompt_context_hash` for the same turn (extends existing Phase 6.3 invariant) | `tests/grill_turn_capsule_*.rs::context_hash_identity_per_turn` |
| I-3 | Session-rollup capsule's `turn_cids[]` is monotonic and contiguous (turn_index 1..N) | `tests/constitution_grill_driven_anchors.rs::turn_cids_monotonic_contiguous` |
| I-4 | At termination, session-rollup capsule's `final_spec_capsule_cid` resolves to a `turingos-spec-capsule-v1` capsule (Phase 6.3 schema unchanged) | `tests/constitution_grill_driven_anchors.rs::final_spec_capsule_resolves_v1_schema` |
| I-5 | Hard turn ceiling: N ≤ 15; turn 16+ refused by predicate P5 | `tests/grill_predicates_*.rs::p5_bound_rejects_16` |
| I-6 | Termination predicate enforces required slot subset `{job, anchor, memory, first_run, robustness, scope, acceptance}` ⊆ `covered_slots` before allowing LLM `done=true` to terminate | `tests/grill_predicates_*.rs::termination_requires_seven_slots` |
| I-7 | Legacy `--mode static` path produces byte-identical spec.md to pre-Phase-6.3.x for the same 8-answer fixture | `tests/spec_grill_byte_compat.rs::legacy_path_byte_identical` |
| I-8 | Session replay walks per-turn capsules without invoking SiliconFlow API (replay-without-recall per Researcher A §3.3) | `tests/constitution_grill_driven_anchors.rs::replay_no_llm_call` |

---

## §3 Risk Class

**Default risk class: Class 2-3** (application-layer wire-up + new CAS-anchored evidence flow; no Class-4 surface mutation; no Trust Root rehash; no sequencer admission change).

### Sub-task classes

- **Class 0**: charter, prompt asset docs, predicate vocab docs, FC trace docs
- **Class 1**: `grill_meta_v1.md` content (pure prompt text), predicate unit tests (pure functions over JSON values)
- **Class 2**: `cmd_llm.rs complete` action, `cmd_spec.rs --mode driven` loop, `spec_capsule.rs` new schema writers, `web/spec.rs` new endpoint, frontend driven mode, WS broadcast variants
- **Class 3**: real-LLM E2E with SiliconFlow live API (production credential use; touches `handover/evidence/`); replay verifier test

### Class 4 surfaces explicitly NOT touched (forbidden in atom-1; would require fresh §8 amendment)

- `src/state/sequencer.rs` — no new admission rule
- `src/state/typed_tx.rs` — no new variant
- `src/bottom_white/cas/schema.rs::ObjectType` — no new variant; reuse `PromptCapsule` + `AttemptTelemetry` + `EvidenceCapsule`
- `src/runtime/prompt_capsule.rs::PromptCapsule` struct — architect-pinned 7 fields, unchanged
- `genesis_payload.toml` Trust Root — no new entries; no rehash
- canonical signing payload surfaces
- `src/kernel.rs`, `src/bus.rs`, `src/sdk/tools/wallet.rs` (Phase 7 STEP_B list)
- `src/runtime/attempt_telemetry.rs::AttemptKind` / `AttemptOutcome` enums (reuse existing `AttemptKind::ExternalizedLlmCycle` per Researcher B §7)

---

## §4 Allowed Paths

This list is **exhaustive**. Edits outside this list trigger stop-and-ratify.

### Source code (Rust)

```
src/bin/turingos/cmd_llm.rs                       (extend: +complete action, ~+150 LOC)
src/bin/turingos/cmd_spec.rs                      (extend: +--mode driven, ~+250 LOC)
src/bin/turingos/spec_capsule.rs                  (extend: +turn/session writers, ~+180 LOC)
src/bin/turingos/grill_envelope.rs                (NEW: turn-payload schema + parser + slot vocab, ~+200 LOC)
src/runtime/grill_predicates.rs                   (NEW: P1-P6 + termination predicate, ~+250 LOC)
src/runtime/mod.rs                                (extend: +pub mod grill_predicates)
src/web/spec.rs                                   (extend: +POST /api/spec/turn handler + GrillSession state, ~+220 LOC)
src/web/ws.rs                                     (extend: AppState.sessions map; WsBroadcastMsg::SpecTurnAdvanced / SpecGrillComplete)
src/web/router.rs                                 (extend: register /api/spec/turn route)
src/web/mod.rs                                    (extend: re-export GrillSession if needed by tests)
```

### Assets (NEW)

```
assets/prompts/grill_meta_v1.md                   (NEW: meta-prompt, ~150 lines markdown; hashed as system_prompt_template_hash)
assets/prompts/grill_synthesis_zh.md              (NEW: extracted from cmd_spec.rs lines 346-393 verbatim; reuse for hashability)
assets/prompts/grill_synthesis_en.md              (NEW: extracted similarly from English variant)
```

### Frontend

```
frontend/src/components/spec-grill.ts             (extend: driven mode + state machine, ~+200 LOC)
frontend/src/types/spec.ts                        (NEW or extend: TurnPayload, TurnResponse, GrillState interfaces)
```

### Tests

```
tests/grill_predicates_p1_p6.rs                   (NEW: ~200 LOC; one #[test] per predicate × pass/fail cases)
tests/grill_predicates_termination.rs             (NEW: ~120 LOC; required-slot subset + confidence threshold + turn-floor logic)
tests/grill_envelope_parse.rs                     (NEW: ~150 LOC; parse valid + invalid + malformed envelopes)
tests/grill_turn_capsule_write_read.rs            (NEW: ~180 LOC; capsule roundtrip; triple-CID anchoring; I-1, I-2)
tests/grill_session_capsule.rs                    (NEW: ~120 LOC; session rollup; I-3, I-4)
tests/cmd_llm_complete_stub.rs                    (NEW: ~150 LOC; CLI integration with mocked SiliconFlow stub)
tests/cmd_spec_driven_mode_stub.rs                (NEW: ~250 LOC; driven loop with mock stub; I-1 through I-6 end-to-end)
tests/web_spec_turn_endpoint.rs                   (NEW: ~150 LOC; HTTP endpoint + AppState; mock-LLM)
tests/spec_grill_byte_compat.rs                   (NEW: ~80 LOC; I-7 legacy path byte-identical)
tests/constitution_grill_driven_anchors.rs        (NEW: ~250 LOC; constitution gates I-1, I-3, I-4, I-8)
```

### Evidence + audit

```
handover/evidence/stage_phase6_3_x_grill_driven_<timestamp>/   (all sub-paths)
handover/audits/AUDITOR_TISR_PHASE6_3_X_*                       (audit records)
handover/reports/TISR_PHASE6_3_X_*                              (ship report)
handover/alignment/OBS_R022_TISR_PHASE6_3_X_*                   (anomalies if found)
handover/directives/2026-05-18_TISR_PHASE6_3_X_*                (this packet + amendments)
```

### Build infrastructure — NOT touched

```
Cargo.toml      — UNCHANGED (no new dependencies; everything within existing axum/tokio/serde/reqwest stack)
Cargo.lock      — UNCHANGED
genesis_payload.toml — UNCHANGED (no Trust Root rehash)
scripts/run_constitution_gates.sh — UNCHANGED (new gate tests auto-discovered via cargo test --workspace)
```

---

## §5 Exit Gates

All must pass before §6 witness or ship:

### Build + lint gates

```bash
cargo check
cargo build --bin turingos
cargo build --bin turingos_web --features web
cargo fmt --all -- --check
cargo clippy --workspace --tests --no-deps -- -D warnings
```

### Test gates

```bash
cargo test --workspace --no-fail-fast --features web
# Includes all new tests under tests/grill_*.rs + tests/constitution_grill_driven_anchors.rs

cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo
# Trust Root unchanged; must still pass

bash scripts/run_constitution_gates.sh
# All existing constitution gates green + new grill_driven_anchors gate green
```

### Frontend gates

```bash
cd frontend && npm install
cd frontend && npm test
cd frontend && npm run build
```

### Real-witness gate

See §6.

### Audit gate

One **clean-context Claude auditor agent** (`subagent_type: auditor`, `model: opus`, xhigh thinking) verdict = **PROCEED**. See §9 for dispatch protocol.

### New constitution gates added by this packet

- `tests/constitution_grill_driven_anchors.rs::grill_driven_writes_three_capsules_per_turn` (I-1)
- `tests/constitution_grill_driven_anchors.rs::grill_session_replay_yields_identical_view` (I-8)
- `tests/constitution_grill_driven_anchors.rs::turn_cids_monotonic_contiguous` (I-3)
- `tests/constitution_grill_driven_anchors.rs::final_spec_capsule_resolves_v1_schema` (I-4)
- `tests/spec_grill_byte_compat.rs::legacy_path_byte_identical` (I-7)

---

## §6 Real Witness Requirement

Phase 6.3.x ship requires producing
`handover/evidence/stage_phase6_3_x_grill_driven_<timestamp>/`,
executed end-to-end by the autonomous verification agent specified in §6a.
**Real SiliconFlow API call** (DeepSeek-V3.2 = Meta model) — not mocked.

### Pipeline (6 steps)

1. **Backend startup**:
   - `cargo run --bin turingos_web --features web` on Mac Studio
   - Binds `127.0.0.1:8080`
   - Verify boot via curl `http://127.0.0.1:8080/health` returns 200

2. **Frontend build + serve**:
   - `cd frontend && npm run build && npm run serve` (port 5173)
   - Verify dist served via curl `http://127.0.0.1:5173/` returns 200

3. **Browser session** (driven by Chrome MCP):
   - Navigate to `http://localhost:8080/build?mode=driven`
   - Click "开始 spec 访谈" CTA
   - Submit user answer 1 (verbatim Chinese): `"我儿子放学想玩俄罗斯方块那种简单游戏，但家里 WiFi 不稳，老断。"`
   - Wait for `SpecTurnAdvanced` WS event (≤ 30s); verify returned `question_text` is NOT byte-equal to any string in legacy `canonical_questions(zh)[1..8]` (proves LLM is generating, not echoing)
   - Submit user answer 2 (verbatim): `"之前在一个网站上玩，但需要网络。"`
   - Continue with prepared answer fixture (`tests/fixtures/grill_mrs_chen_zh.json`) until LLM emits `done=true`
   - Verify termination between turn 6 and turn 12 (inclusive)
   - Verify `playback` field present in final response; render to screen
   - Confirm playback ("没问题"); verify spec.md generated + spec capsule CID returned

4. **CAS walk verification** (shell out, no browser):
   ```bash
   turingos spec audit --workspace <ws> --session <session_id>
   ```
   - Lists session-rollup capsule (`turingos-spec-grill-session-v1`)
   - Walk `turn_cids[]`; for each:
     - Resolve `prompt_capsule_cid` → fetch PromptCapsule
     - Resolve `attempt_telemetry_cid` → fetch AttemptTelemetry
     - Verify `AttemptTelemetry.prompt_context_hash == PromptCapsule.prompt_context_hash`
   - Resolve `final_spec_capsule_cid` → verify schema = `turingos-spec-capsule-v1`

5. **Replay** (no API call):
   - Re-run audit walk with `--offline` flag (does not call SiliconFlow)
   - Verify all per-turn data reconstructable from CAS only
   - This proves replay-without-recall (I-8)

6. **Legacy byte-compat fallback** (third session):
   - Run `turingos spec --workspace <ws3> --answers-file tests/fixtures/legacy_8_answers.json --mode static`
   - Verify final `spec.md` is byte-identical to snapshot `tests/fixtures/legacy_spec_md_snapshot.txt`
   - This proves no regression of legacy path (I-7)

### Witness output

```
handover/evidence/stage_phase6_3_x_grill_driven_<timestamp>/
├── agent_verdict.json                  (PASS|FAIL|PARTIAL + per-step verdicts)
├── browser_screenshots/                (turn 1..N + playback + complete)
├── browser_console_logs.json           (zero errors required)
├── browser_network_logs.json           (every POST /api/spec/turn captured)
├── cas_walk_output.txt                 (turingos spec audit stdout)
├── replay_diff.txt                     (must be empty)
├── legacy_byte_compat_hash.txt         (sha256 of legacy spec.md vs snapshot)
├── final_spec_md.md                    (driven-mode product, for human review)
├── session_rollup_capsule_cid.txt
└── all_turn_capsule_cids.txt
```

---

## §6a Autonomous Verification Agent

Inherits Phase 7 §6a Chrome-driving pattern. The verifier is dispatched once
by the orchestrator and runs unsupervised until final verdict.

### Agent specification

| Field | Value |
|---|---|
| Dispatch | `Agent` tool, `subagent_type: general-purpose`, `model: opus`, no worktree isolation (driver session needs local Chrome + SiliconFlow API key from env) |
| Tools allowed | `mcp__Claude_in_Chrome__navigate`, `get_page_text`, `javascript_tool`, `read_console_messages`, `read_network_requests`, `computer` (screenshot), `form_input`, `Bash` (server start/stop + `turingos spec audit`), `Read`, `Write` (`handover/evidence/stage_phase6_3_x_*` ONLY), `Glob`, `Grep` |
| Tools forbidden | `Edit` (no source modification), `WebSearch`, `WebFetch`, write outside evidence dir |
| Input | (a) commit SHA of Phase 6.3.x ship candidate; (b) localhost URL + port; (c) the 6-step pipeline from §6; (d) per-step mechanical decision criteria below; (e) evidence directory path; (f) prepared answer fixture `tests/fixtures/grill_mrs_chen_zh.json` |
| Output | `agent_verdict.json` + all witnesses listed above |
| Timeout | 75 min wall clock (server bootstrap + frontend build + 12 LLM turns × 30s + CAS walk + replay + legacy compat) |
| Failure escalation | Crashes mid-run OR unparseable verdict → orchestrator re-dispatches once; second crash → halt + architect ratify |

### Per-step mechanical decision criteria

**Step 1 — Backend startup**
- `cargo run --bin turingos_web --features web` exit code 0 within 60s OR process running with `lsof -i :8080` showing LISTEN
- `curl -sf http://127.0.0.1:8080/health` → HTTP 200

**Step 2 — Frontend build**
- `npm run build` exit code 0 within 120s
- `dist/` directory contains `index.html` + at least one `.js` chunk

**Step 3 — Browser driven session**
- Per-turn checks:
  - `mcp__Claude_in_Chrome__read_network_requests` shows one `POST /api/spec/turn` per user submit
  - Response `question_text` ≠ any string in `canonical_questions(zh)[..]` (byte comparison after stripping leading/trailing whitespace)
  - Response `turn_index` is N+1 for the Nth submit
  - Response `covered_slots ⊆ {job, anchor, memory, first_run, robustness, scope, acceptance, mirror}`
  - Response `covered_slots ⊇` covered_slots from previous turn (monotonicity)
  - WS message `SpecTurnAdvanced` received within 30s of submit
- Termination check:
  - LLM emits `done=true` between turn 6 and turn 12 inclusive
  - Response includes non-empty `playback` field
- After playback confirm:
  - `SpecGrillComplete{spec_capsule_cid}` WS event received
  - `spec_capsule_cid` is a 64-char hex string
- Console: zero `level=error` messages across all turns
- Network: zero 5xx responses

**Step 4 — CAS walk**
- `turingos spec audit --session <id>` exit code 0
- Stdout contains session-rollup capsule line: schema = `turingos-spec-grill-session-v1`
- Stdout contains N turn capsule lines (N from step 3): schema = `turingos-spec-grill-turn-v1`
- For each turn, the printed `prompt_context_hash` (from PromptCapsule) equals printed `prompt_context_hash` (from AttemptTelemetry)
- Final spec capsule line: schema = `turingos-spec-capsule-v1`

**Step 5 — Replay**
- `turingos spec audit --session <id> --offline` exit code 0
- Stdout byte-identical to step 4 stdout (no LLM-call diffs)
- Process did NOT make outbound network calls (verified via `lsof -i` snapshot before/after)

**Step 6 — Legacy byte-compat**
- `turingos spec --answers-file legacy_8_answers.json --mode static` exit code 0
- `sha256sum <ws3>/spec.md` equals `sha256sum tests/fixtures/legacy_spec_md_snapshot.txt`

### Anti-hallucination safeguards

- Verifier writes ONLY to evidence directory; cannot Edit source
- Binary pinned to audited commit at session start (verifier shells `git rev-parse HEAD` and records)
- Every HTTP/WS interaction logged via Chrome MCP network capture
- If any step fails, verifier continues remaining steps (architect sees full picture, not just first failure)
- Verifier does NOT attempt to "fix" the underlying code

---

## §7 Constraints

### Hard

- **Trust Root NOT rehashed.** `genesis_payload.toml` unchanged. Binary pinned to commit at audit time.
- **Class 4 STEP_B surfaces unchanged** (see §3).
- **No multimodal grill input.** Text only.
- **No agent.toml / CCS manifest parsing.** Deferred.
- **LLM provider: SiliconFlow only.** No openai/anthropic provider swap.
- **Turn ceiling: 15.** Hard. Predicate P5 refuses turn 16+. Kernel forces termination at turn 15.
- **Per-turn LLM timeout: 30s.** Two consecutive timeouts → session pause + user-facing "网络不稳，稍等一下再试" toast.
- **Session timeout: 30 min wall clock.** Idle sessions pruned from `AppState.sessions`.
- **Predicate retry budget: 1 per turn.** If P1 or P2 fails twice consecutively → halt session with typed error.
- **Predicate vocabulary frozen** for this atom: `{job, anchor, memory, first_run, robustness, scope, acceptance, mirror}`. No new slot ids added mid-atom.

### Soft

- **New code LOC target: ~900 LOC.** Hard cap 1500 LOC. Atom-1 cannot exceed 1500 without architect §8 amendment.
- **Atom-1.5 (Blackbox triage) deferred** unless a real abuse-vector demo case materializes during §6 witness run.
- **No streaming partial-token UX** in frontend; request/response per turn. Streaming is a Phase 6.3.y additive.

---

## §8 Architect Sign-off

The architect must mark each of the following before atom dispatch:

```
[ ] §1 Scope — read and approved (6 artifact additions; 8+ explicit non-ratifications)
[ ] §2 FC Mapping — all 8 new invariants (I-1 through I-8) approved
[ ] §3 Risk Class — Class 2-3 default + Class 4 forbidden list approved
[ ] §4 Allowed Paths — exhaustive file list approved
[ ] §5 Exit Gates — all build/test/lint/audit gates approved
[ ] §6 Real Witness — 6-step pipeline approved; SiliconFlow live API authorized
[ ] §6a Verifier Agent — Opus auditor + Chrome MCP authorized
[ ] §7 Constraints — hard + soft caps approved; turn ceiling = 15 confirmed
[ ] §9 Multi-Agent Execution Model — W0-W8 atom decomposition approved
[ ] §10 Risk Register — known risks acknowledged
[ ] LOC budget: target ~900, cap 1500 — confirmed
[ ] Atom-1.5 (Blackbox triage) deferral — confirmed (or override)
[ ] Naming policy: `turingos-spec-grill-*` (NOT `turingos-ca-*`) — confirmed
[ ] Dispatch authorization: orchestrator may proceed to W0
```

**Architect signature** (free-text + date): _____________________________

---

## §9 Multi-Agent Execution Model

This section is the **fire-and-forget brief** for downstream Sonnet/Haiku
sub-agents executing each W atom. Every atom is self-contained; a sub-agent
receives the atom spec verbatim as its initial prompt and runs without
orchestrator intervention until completion.

### W-atom dependency graph

```
W0 (worktree + plan)
  │
  ▼
W1 (prompt assets)              ← Class 0/1, can ship first
  │
  ▼
W2 (grill_envelope.rs)          ← Class 1, pure parser; no async
  │
  ▼
W3 (grill_predicates.rs)        ← Class 1, pure functions over envelope
  │
  ▼
W4 (cmd_llm.rs complete)        ← Class 2, depends on envelope + predicates
  │
  ▼
W5 (spec_capsule.rs writers)    ← Class 2, depends on envelope
  │
  ▼
W6 (cmd_spec.rs --mode driven)  ← Class 2-3, depends on W4 + W5 + W3
  │
  ▼
W7 (web/spec.rs /turn endpoint) ← Class 2, depends on W6
  │
  ▼
W8 (frontend driven mode)       ← Class 2, depends on W7
  │
  ▼
W9 (real-LLM E2E witness)       ← Class 3, depends on W8 ship
  │
  ▼
W10 (clean-context audit)       ← Audit gate; xhigh Opus reviewer
```

W1-W3 can be dispatched in parallel (no inter-dependency).
W4 depends on W2+W3.
W5 depends on W2.
W6 depends on W4+W5+W3.
W7 depends on W6.
W8 depends on W7.
W9 depends on W8 (real witness).
W10 depends on W9 (audit needs evidence).

### Dispatch protocol

**Orchestrator** (this conversation) does the following per atom:

1. Create / verify worktree at base `86e82406` on branch `codex/tisr-phase6-3-x-grill-driven` (in-place dispatch per architect's prior feedback memory — avoid worktree isolation drift)
2. Dispatch `Agent` tool with:
   - `subagent_type: general-purpose`
   - `model: sonnet` (default; Haiku for W1 docs-only; Opus for W10 audit)
   - `description`: `<5-word atom name>`
   - `prompt`: the **verbatim atom spec from §9.W<N>** below
   - `run_in_background: false` (block on completion; predictable progress)
3. On agent return:
   - Run §5 gates locally
   - On gate fail: dispatch a fresh agent with the gate-fail message + the original spec
   - On gate pass: mark atom complete; dispatch next atom
4. After W9 evidence exists, dispatch W10 clean-context audit

### Atom specs (verbatim briefs)

---

### W0 — Worktree + Plan

**Agent role**: Sonnet, in-place (no worktree isolation)
**Depends on**: nothing
**Risk class**: Class 0
**LOC**: 0 source; ~50 lines plan doc

**Brief**:

> You are working in `/Users/zephryj/work/turingosv4` on branch
> `codex/tisr-phase7-web` at HEAD `86e82406`. Create a new branch
> `codex/tisr-phase6-3-x-grill-driven` based on current HEAD. Do NOT switch
> away from existing branch — `git branch <name>` only, do not checkout.
>
> Read these files completely to ground yourself:
> 1. `handover/directives/2026-05-18_TISR_PHASE6_3_X_GRILL_LLM_DRIVEN_SECTION8_PACKET.md`
> 2. `handover/research/grill_software_3_0_2026-05-18/researcher_a/DESIGN.md`
> 3. `handover/research/grill_software_3_0_2026-05-18/researcher_b/DESIGN.md`
> 4. `src/bin/turingos/cmd_spec.rs` (lines 280-410 for canonical_questions + system_prompt + synthesis)
> 5. `src/bin/turingos/cmd_llm.rs` (full file; understand existing run() shape)
> 6. `src/runtime/prompt_capsule.rs` (PromptCapsule struct + write_prompt_capsule_to_cas)
> 7. `src/runtime/attempt_telemetry.rs` (AttemptKind::ExternalizedLlmCycle + AttemptTelemetry::new_root + with_prompt_capsule_cid)
>
> Write a 30-50 line file `handover/directives/2026-05-18_TISR_PHASE6_3_X_W0_PLAN.md`
> answering:
> - Confirm branch created: `git branch --list codex/tisr-phase6-3-x-grill-driven` returns the name
> - List the exact line ranges in `cmd_spec.rs` you will modify in W6
> - List the exact public fn signatures you will add in W2 (grill_envelope.rs) and W3 (grill_predicates.rs)
> - Confirm no Class-4 surface (§3 forbidden list) appears in your planned edits
>
> Do NOT write source code in this atom. Do NOT modify any .rs file. Only
> the plan markdown.
>
> Done criteria:
> - `git branch --list codex/tisr-phase6-3-x-grill-driven` shows the branch
> - Plan file exists at the named path with 30-50 lines
> - `cargo check` still passes (no spurious file mutations)

**Acceptance test**:
```bash
git branch --list codex/tisr-phase6-3-x-grill-driven  # must print the name
test -f handover/directives/2026-05-18_TISR_PHASE6_3_X_W0_PLAN.md
wc -l handover/directives/2026-05-18_TISR_PHASE6_3_X_W0_PLAN.md  # 30 ≤ lines ≤ 60
cargo check
```

---

### W1 — Prompt assets

**Agent role**: Haiku 4.5 (docs-only)
**Depends on**: W0
**Risk class**: Class 0/1
**LOC**: ~250 lines markdown across 3 files

**Brief**:

> Read `handover/research/grill_software_3_0_2026-05-18/researcher_a/DESIGN.md`
> §2.1 (meta-prompt body) verbatim.
>
> Create `assets/prompts/grill_meta_v1.md` with the ROLE + METHODOLOGY +
> CONSTRAINTS + OUTPUT CONTRACT structure from Researcher A §2.1, but
> rendered as proper markdown (not a Rust raw-string fenced block).
> Include both Chinese-default phrasing and an English-fallback variant.
>
> Read `src/bin/turingos/cmd_spec.rs` lines 346-393. Extract the existing
> `system_prompt(Lang::Zh)` body verbatim into a new file
> `assets/prompts/grill_synthesis_zh.md`. Do the same for `system_prompt(Lang::En)`
> into `assets/prompts/grill_synthesis_en.md`. These extracts are for
> later hashability — do NOT modify the source `cmd_spec.rs` yet (that's W6).
>
> The exact slot vocabulary the meta-prompt must reference:
> `{job, anchor, memory, first_run, robustness, scope, acceptance, mirror}`
> (8 slots; first 7 are "required" for termination; mirror is optional).
>
> The exact JSON envelope shape the meta-prompt instructs the LLM to emit:
> ```json
> {
>   "turn": <int 1..15>,
>   "question_text": "<string|null>",
>   "covered_slots": ["<slot_id>"],
>   "open_slots": ["<slot_id>"],
>   "confidence": <float 0.0..1.0>,
>   "done": <bool>,
>   "rationale_brief": "<string ≤ 200 chars>",
>   "playback": "<string, only if done=true>"
> }
> ```
>
> Done criteria:
> - 3 files exist at the named paths
> - `grill_meta_v1.md` total: 120-200 lines
> - `grill_synthesis_zh.md` and `grill_synthesis_en.md` each: at least 30 lines
> - All slot ids spelled exactly per the vocabulary above
> - All field names in envelope spelled exactly as shown
> - No Rust code touched
>
> Do NOT make decisions about prompt phrasing beyond what Researcher A §2.1
> verbatim says. If unsure, copy verbatim.

**Acceptance test**:
```bash
test -f assets/prompts/grill_meta_v1.md
test -f assets/prompts/grill_synthesis_zh.md
test -f assets/prompts/grill_synthesis_en.md
[ $(wc -l < assets/prompts/grill_meta_v1.md) -ge 120 ]
grep -q "covered_slots" assets/prompts/grill_meta_v1.md
grep -q "open_slots" assets/prompts/grill_meta_v1.md
grep -q "rationale_brief" assets/prompts/grill_meta_v1.md
grep -q "playback" assets/prompts/grill_meta_v1.md
```

---

### W2 — grill_envelope.rs (parser + slot vocab)

**Agent role**: Sonnet 4.6
**Depends on**: W1
**Risk class**: Class 1 (pure additive helper)
**LOC**: ~200 LOC source + ~150 LOC tests

**Brief**:

> Create new file `src/bin/turingos/grill_envelope.rs` with NO modifications
> to other files (except adding `pub mod grill_envelope;` to
> `src/bin/turingos/mod.rs` or `src/bin/turingos.rs` — whichever is the
> module root; inspect first).
>
> Public surface:
>
> ```rust
> /// Canonical slot vocabulary. Order matters for tests but not semantics.
> pub const CANONICAL_SLOTS: &[&str] = &[
>     "job", "anchor", "memory", "first_run",
>     "robustness", "scope", "acceptance", "mirror",
> ];
>
> /// Required for termination (all 7 must be ⊆ covered_slots).
> pub const REQUIRED_SLOTS: &[&str] = &[
>     "job", "anchor", "memory", "first_run",
>     "robustness", "scope", "acceptance",
> ];
>
> #[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
> pub struct TurnPayload {
>     pub turn: u32,
>     pub question_text: Option<String>,
>     pub covered_slots: Vec<String>,
>     pub open_slots: Vec<String>,
>     pub confidence: f64,
>     pub done: bool,
>     pub rationale_brief: String,
>     #[serde(default, skip_serializing_if = "Option::is_none")]
>     pub playback: Option<String>,
> }
>
> #[derive(Debug, thiserror::Error)]
> pub enum EnvelopeParseError {
>     #[error("invalid JSON: {0}")]
>     Json(String),
>     #[error("turn must be 1..=15, got {0}")]
>     TurnOutOfRange(u32),
>     #[error("confidence must be in [0.0, 1.0], got {0}")]
>     ConfidenceOutOfRange(f64),
>     #[error("slot {0:?} not in canonical vocabulary")]
>     UnknownSlot(String),
>     #[error("if done=false, question_text must be non-empty")]
>     EmptyQuestionWhenNotDone,
>     #[error("if done=true, playback must be present and non-empty")]
>     MissingPlaybackOnDone,
> }
>
> pub fn parse_turn_payload(raw: &str) -> Result<TurnPayload, EnvelopeParseError>;
>
> /// Strict validation: parses + applies semantic checks.
> /// Returns Ok only if all envelope-level invariants hold.
> /// This is a SUPERSET of P1 (schema parse) — predicates in
> /// grill_predicates.rs treat the rest (P2-P6) separately.
> pub fn parse_and_validate(raw: &str) -> Result<TurnPayload, EnvelopeParseError>;
> ```
>
> Implementation notes:
> - Use existing `serde_json` (already in Cargo workspace deps)
> - `parse_turn_payload`: pure deserialize, no semantic checks
> - `parse_and_validate`: deserialize + checks: turn∈[1,15], confidence∈[0,1],
>   all slot ids ∈ CANONICAL_SLOTS, question_text non-empty when !done,
>   playback present when done
> - No async; no tokio; no I/O
>
> Write `tests/grill_envelope_parse.rs` with at least these test functions:
> - `parses_valid_turn_3_question`
> - `parses_valid_terminal_with_playback`
> - `rejects_turn_16_out_of_range`
> - `rejects_unknown_slot_id`
> - `rejects_confidence_above_1`
> - `rejects_empty_question_when_done_false`
> - `rejects_missing_playback_when_done_true`
> - `accepts_optional_playback_absent_when_done_false`
>
> Done criteria:
> - `cargo build --bin turingos` passes
> - `cargo test --test grill_envelope_parse` shows ≥ 8 tests passing
> - No edits to files outside the allowed list for W2:
>   - `src/bin/turingos/grill_envelope.rs` (new)
>   - `src/bin/turingos/mod.rs` (or `src/bin/turingos.rs`) — only adding `pub mod grill_envelope;`
>   - `tests/grill_envelope_parse.rs` (new)
> - `cargo fmt --all -- --check` passes
> - `cargo clippy -- -D warnings` clean for new module

**Acceptance test**:
```bash
cargo build --bin turingos
cargo test --test grill_envelope_parse --no-fail-fast
cargo fmt --all -- --check
cargo clippy --bin turingos -- -D warnings
```

---

### W3 — grill_predicates.rs (P1-P6 + termination)

**Agent role**: Sonnet 4.6
**Depends on**: W2 (uses TurnPayload + CANONICAL_SLOTS)
**Risk class**: Class 1 (pure functions)
**LOC**: ~250 LOC source + ~250 LOC tests

**Brief**:

> Create new file `src/runtime/grill_predicates.rs`. Add `pub mod grill_predicates;`
> to `src/runtime/mod.rs`.
>
> Public surface:
>
> ```rust
> use crate::bin::turingos::grill_envelope::{TurnPayload, REQUIRED_SLOTS};
> // ^ adjust import path based on actual module layout — inspect first
>
> #[derive(Debug, Clone, PartialEq)]
> pub enum PredicateVerdict {
>     Pass,
>     Fail { reason: String },
> }
>
> impl PredicateVerdict {
>     pub fn is_pass(&self) -> bool { matches!(self, PredicateVerdict::Pass) }
> }
>
> #[derive(Debug, Clone)]
> pub struct PredicateBundle {
>     pub p1_schema_parse: PredicateVerdict,
>     pub p2_kind_ok: PredicateVerdict,
>     pub p3_slots_in_vocab: PredicateVerdict,
>     pub p4_monotonic: PredicateVerdict,
>     pub p5_turn_bounded: PredicateVerdict,
>     pub p6_question_nonempty_lang: PredicateVerdict,
> }
>
> impl PredicateBundle {
>     pub fn all_pass(&self) -> bool { /* all 6 pass */ }
> }
>
> pub enum Lang { Zh, En }
>
> /// Run P1-P6 over a parsed payload + previous-turn's covered_slots.
> /// P1 is implicitly satisfied here (caller already parsed); kept for trace symmetry.
> pub fn run_turn_predicates(
>     payload: &TurnPayload,
>     prev_covered: &[String],
>     lang: Lang,
> ) -> PredicateBundle;
>
> /// Termination predicate: returns true ONLY IF the kernel allows LLM's
> /// done=true to terminate the session.
> /// Rules:
> /// - covered_slots must ⊇ REQUIRED_SLOTS (7 slots)
> /// - confidence ≥ 0.8
> /// - turn ≥ 4 (no premature termination)
> pub fn termination_predicate(payload: &TurnPayload) -> PredicateVerdict;
>
> // Individual predicate fns (pub for testing):
> pub fn p2_kind_ok(payload: &TurnPayload) -> PredicateVerdict;
> pub fn p3_slots_in_vocab(payload: &TurnPayload) -> PredicateVerdict;
> pub fn p4_monotonic(payload: &TurnPayload, prev_covered: &[String]) -> PredicateVerdict;
> pub fn p5_turn_bounded(payload: &TurnPayload) -> PredicateVerdict;
> pub fn p6_question_nonempty_lang(payload: &TurnPayload, lang: Lang) -> PredicateVerdict;
> ```
>
> P6 language heuristic (Han-script ratio):
> ```rust
> // For Lang::Zh, count chars where char as u32 in [0x4E00, 0x9FFF];
> // ratio must be ≥ 0.5 of all non-whitespace, non-punct chars.
> // For Lang::En, ASCII alphanumeric ratio ≥ 0.8 over the same denominator.
> ```
>
> No async. No I/O. No new dependencies.
>
> Write `tests/grill_predicates_p1_p6.rs` and `tests/grill_predicates_termination.rs`:
>
> Required test cases (minimum):
> - `p2_pass_on_question_when_not_done`
> - `p2_pass_on_done_terminal`
> - `p2_fail_on_done_true_but_no_playback` (envelope-level catches this too;
>   verify P2 surface is independent)
> - `p3_pass_on_canonical_slots`
> - `p3_fail_on_invented_slot` (e.g. `"foo"`)
> - `p4_pass_on_strict_superset`
> - `p4_pass_on_equality` (no slot uncovered, no slot added — allowed)
> - `p4_fail_on_shrinking_set` (e.g. prev=[job,anchor], next=[job])
> - `p5_pass_on_turn_15`
> - `p5_fail_on_turn_16`
> - `p6_pass_on_chinese_question`
> - `p6_fail_on_english_question_when_zh_requested`
> - `p6_pass_on_8_char_chinese`
> - `p6_fail_on_7_char_chinese` (8-char floor per §1)
> - `termination_pass_when_seven_required_and_confidence_high`
> - `termination_fail_when_missing_acceptance_slot`
> - `termination_fail_when_confidence_below_0_8`
> - `termination_fail_when_turn_below_4`
>
> Done criteria:
> - `cargo build` passes
> - `cargo test --test grill_predicates_p1_p6 --test grill_predicates_termination` ≥ 16 passing
> - No edits outside: `src/runtime/grill_predicates.rs` (new), `src/runtime/mod.rs` (add pub mod line), test files
> - `cargo clippy -- -D warnings` clean for new code

**Acceptance test**:
```bash
cargo build
cargo test --test grill_predicates_p1_p6 --test grill_predicates_termination --no-fail-fast
cargo fmt --all -- --check
cargo clippy --workspace --tests -- -D warnings
```

---

### W4 — cmd_llm.rs `complete` action

**Agent role**: Sonnet 4.6
**Depends on**: W2 (TurnPayload + parse_and_validate), W3 (predicates if --strict)
**Risk class**: Class 2
**LOC**: ~150 LOC

**Brief**:

> Extend `src/bin/turingos/cmd_llm.rs` with a new sub-action `complete`.
> Current shape: `run(args)` dispatches on `args[0]` (action). Today's
> actions cover keystore config. Add a new branch for `"complete"`.
>
> CLI surface:
>
> ```
> turingos llm complete
>     --workspace <PATH>             # required
>     --role <meta|blackbox>         # default: meta
>     --prompt-file <PATH>           # JSON file with messages array; use "-" for stdin
>     --max-tokens <N>               # default 2000 (meta) or 400 (blackbox)
>     --temperature <FLOAT>          # default 0.4 (meta) or 0.2 (blackbox)
>     --capsule-dir <PATH>           # optional; if set, write prompt+attempt capsules here
>     --turn-id <STRING>             # for capsule filenames
>     --strict-json                  # if set, parse output via grill_envelope::parse_and_validate
>     --lang <zh|en>                 # only for error messages on parse-fail
> ```
>
> Stdout (single JSON line):
>
> ```json
> {
>   "ok": true,
>   "content": "<verbatim LLM content string>",
>   "parsed_envelope": <TurnPayload | null>,
>   "usage": { "prompt_tokens": <int>, "completion_tokens": <int>, "total_tokens": <int> },
>   "finish_reason": "<string>",
>   "model": "<string>",
>   "prompt_capsule_cid": "<hex|null>",
>   "attempt_telemetry_cid": "<hex|null>",
>   "elapsed_ms": <int>
> }
> ```
>
> On error: `{ok: false, error: {kind: "parse_failed"|"http_status"|"timeout"|..., detail: "..."}}`.
>
> Implementation notes:
> - Reuse `siliconflow_client::chat_complete` (existing, line 154)
> - Reuse `cmd_llm::read_meta_model` / `read_blackbox_model` / `read_api_key_env_var`
> - For `--prompt-file -`: read full stdin via `std::io::Read::read_to_string`
> - prompt_file JSON format: `{"messages": [{"role": "system|user|assistant", "content": "..."}], "max_tokens"?: int, "temperature"?: float}`
> - If `--strict-json` is set:
>   - Parse content via `grill_envelope::parse_and_validate`
>   - On parse fail: emit `{ok: false, error: {kind: "parse_failed", detail: "..."}}` and exit 3
> - Always (whether strict or not), if `--capsule-dir` is set:
>   - Build PromptCapsule using `prompt_capsule::write_prompt_capsule_to_cas`
>     - `prompt_context_hash` = sha256 of canonical-encoded message array
>     - `read_set` = empty Vec (Phase 6.3.x v1; future atom may fill)
>     - `policy_version` = `"grill_meta_v1"` if --strict-json else `"complete_v1"`
>     - `hidden_fields_redacted` = false
>     - `visible_context_cid` = CAS-store the raw message array bytes; cid hex
>     - `system_prompt_template_hash` = sha256 of system message content (if any)
>     - `agent_view_manifest_cid` = same as visible_context_cid for v1
>   - Build AttemptTelemetry using `attempt_telemetry::AttemptTelemetry::new_root`
>     - `attempt_kind` = `AttemptKind::ExternalizedLlmCycle`
>     - `outcome` = `AttemptOutcome::LeanPass` (re-purposed per Researcher B §7 note;
>       grill turn "pass" = predicate-accepted; if --strict-json and parse fails,
>       use `AttemptOutcome::ParseFail`)
>     - `prompt_context_hash` = same as PromptCapsule
>     - chain via `with_prompt_capsule_cid(prompt_capsule_cid)`
>   - Write both via existing `write_prompt_capsule_to_cas` and `write_attempt_telemetry_to_cas`
>   - Print CIDs in stdout JSON
>
> Wire into `run()` dispatcher. Update help string to list "complete" action.
>
> Allowed edits:
> - `src/bin/turingos/cmd_llm.rs` (extend)
> - `tests/cmd_llm_complete_stub.rs` (new)
>
> No other files.
>
> Test file `tests/cmd_llm_complete_stub.rs`:
> - Mock SiliconFlow by setting env var `TURINGOS_SILICONFLOW_ENDPOINT` to a
>   local in-process HTTP stub server that returns a canned `chat/completions`
>   response — pattern same as Phase 7 stub tests
> - Test: stub returns valid turn payload JSON, CLI exit 0, stdout JSON has
>   `ok: true` + parsed_envelope populated
> - Test: stub returns invalid JSON, CLI with --strict-json exits 3
> - Test: stub returns 500, CLI prints error JSON with kind="http_status"
>
> Done criteria:
> - `cargo build --bin turingos` passes
> - `cargo test --test cmd_llm_complete_stub --no-fail-fast` all passing
> - Help string includes "complete" action
> - No edits outside allowed list

**Acceptance test**:
```bash
cargo build --bin turingos
cargo test --test cmd_llm_complete_stub --no-fail-fast
./target/debug/turingos llm 2>&1 | grep -q complete
cargo fmt --all -- --check
```

---

### W5 — spec_capsule.rs (grill turn + session writers)

**Agent role**: Sonnet 4.6
**Depends on**: W2 (TurnPayload struct)
**Risk class**: Class 2
**LOC**: ~180 LOC source + ~180 LOC tests

**Brief**:

> Extend `src/bin/turingos/spec_capsule.rs` (currently 163 LOC). Do NOT
> modify existing public symbols (`SPEC_CAPSULE_SCHEMA_ID`, `write_spec_capsule`,
> `latest_spec_capsule_cid`, `read_spec_capsule`). Add new pub items below
> the existing ones.
>
> New public surface:
>
> ```rust
> pub(crate) const SPEC_GRILL_TURN_CAPSULE_SCHEMA_ID: &str = "turingos-spec-grill-turn-v1";
> pub(crate) const SPEC_GRILL_SESSION_CAPSULE_SCHEMA_ID: &str = "turingos-spec-grill-session-v1";
>
> #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
> pub struct GrillTurnCapsuleBody {
>     pub session_id: String,
>     pub turn_index: u32,
>     pub prompt_capsule_cid: String,
>     pub attempt_telemetry_cid: String,
>     pub user_answer_cid: Option<String>,    // None on turn 1's initial LLM call (no prior answer)
>     pub parent_turn_cid: Option<String>,    // None for turn 1
>     pub predicate_verdicts: GrillPredicateVerdicts,
>     pub turn_payload_snapshot: serde_json::Value,  // the parsed TurnPayload as JSON
>     pub logical_t: u64,
> }
>
> #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
> pub struct GrillPredicateVerdicts {
>     pub p1_pass: bool,
>     pub p2_pass: bool,
>     pub p3_pass: bool,
>     pub p4_pass: bool,
>     pub p5_pass: bool,
>     pub p6_pass: bool,
>     pub termination_predicate_pass: Option<bool>,  // None if turn didn't claim termination
>     pub failure_reasons: Vec<String>,
> }
>
> #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
> pub struct GrillSessionCapsuleBody {
>     pub session_id: String,
>     pub turn_cids: Vec<String>,             // ordered turn_index 1..N
>     pub final_spec_capsule_cid: String,
>     pub termination_reason: String,          // "llm_done_predicate_pass" | "turn_limit_forced" | "user_abort" | "predicate_double_fail"
>     pub total_turns: u32,
>     pub partial_session: bool,               // true if forced terminate
>     pub lang: String,                        // "zh" | "en"
>     pub logical_t: u64,
> }
>
> pub(crate) fn write_grill_turn_capsule(
>     workspace: &Path,
>     body: &GrillTurnCapsuleBody,
> ) -> Result<String, CapsuleError>;  // returns CID hex
>
> pub(crate) fn write_grill_session_capsule(
>     workspace: &Path,
>     body: &GrillSessionCapsuleBody,
> ) -> Result<String, CapsuleError>;
>
> pub(crate) fn read_grill_turn_capsule(
>     workspace: &Path,
>     cid_hex: &str,
> ) -> Result<GrillTurnCapsuleBody, CapsuleError>;
>
> pub(crate) fn read_grill_session_capsule(
>     workspace: &Path,
>     cid_hex: &str,
> ) -> Result<GrillSessionCapsuleBody, CapsuleError>;
>
> pub(crate) fn list_grill_session_capsules(workspace: &Path)
>     -> Result<Vec<String>, CapsuleError>;
> ```
>
> Implementation pattern: follow existing `write_spec_capsule` exactly
> (line 73). The CAS write call uses `ObjectType::EvidenceCapsule` with the
> schema_id constant.
>
> Allowed edits:
> - `src/bin/turingos/spec_capsule.rs` (extend)
> - `tests/grill_turn_capsule_write_read.rs` (new)
> - `tests/grill_session_capsule.rs` (new)
>
> Required tests:
> - `write_then_read_turn_capsule_roundtrip`
> - `write_then_read_session_capsule_roundtrip`
> - `list_returns_only_grill_session_schemas` (not spec-capsule-v1, not random other capsules)
> - `turn_capsule_with_no_parent_for_turn_1` (parent_turn_cid = None allowed)
> - `turn_capsule_chain_resolves_from_session` (write 3 turn capsules + session capsule referencing them; verify each turn capsule resolves from session.turn_cids[])
>
> Done criteria:
> - `cargo build` passes
> - `cargo test --test grill_turn_capsule_write_read --test grill_session_capsule --no-fail-fast` all passing
> - `SPEC_CAPSULE_SCHEMA_ID` constant value unchanged: still `"turingos-spec-capsule-v1"`
> - No edits outside allowed list

**Acceptance test**:
```bash
cargo build
cargo test --test grill_turn_capsule_write_read --test grill_session_capsule --no-fail-fast
grep -q 'pub(crate) const SPEC_CAPSULE_SCHEMA_ID: &str = "turingos-spec-capsule-v1";' src/bin/turingos/spec_capsule.rs
grep -q 'turingos-spec-grill-turn-v1' src/bin/turingos/spec_capsule.rs
grep -q 'turingos-spec-grill-session-v1' src/bin/turingos/spec_capsule.rs
```

---

### W6 — cmd_spec.rs `--mode driven`

**Agent role**: Sonnet 4.6
**Depends on**: W4 (cmd_llm complete) + W5 (spec_capsule writers) + W3 (predicates)
**Risk class**: Class 2-3
**LOC**: ~250 LOC

**Brief**:

> Extend `src/bin/turingos/cmd_spec.rs` (currently 581 LOC). Do NOT modify
> `canonical_questions` (line 293) or `system_prompt` (line 346) — those
> remain for legacy `--mode static`.
>
> Add `--mode {static|driven}` CLI flag. Default = static. Parse with existing
> arg-parsing pattern in this file.
>
> When `--mode driven`:
> 1. Initialize session: generate session_id (timestamp + 8 hex chars per existing pattern), create `<workspace>/sessions/<session_id>/` dir, write empty `coverage_state.json` (CanonicalSlotTable with all 8 slots at state "empty")
> 2. Loop turn 1..15:
>    a. Build message array:
>       - system: contents of `assets/prompts/grill_meta_v1.md`
>       - system: `COVERAGE STATE:\n covered_slots: [...]\n open_slots: [...]\n turn_number: <N>\n language: <zh|en>`
>       - For turns 2..N: prior Q/A pairs as alternating user/assistant messages (only last 3 pairs included)
>       - user: `"Produce your turn-<N> output per the contract."`
>    b. Serialize to JSON, write to `<session>/turn-<N>-prompt.json`
>    c. Shell out: `turingos llm complete --workspace <ws> --role meta --prompt-file <path> --strict-json --capsule-dir <session>/capsules/ --turn-id turn-<N> --lang <zh|en>`
>    d. Parse stdout JSON. If `ok=false`, run retry (max 1):
>       - Append a system message `"Your previous output failed predicate X. Output only the JSON envelope."` to the prompt
>       - Re-shell. If still fail → halt session with typed error, write session capsule with `termination_reason = "predicate_double_fail"`
>    e. Run P1-P6 via `grill_predicates::run_turn_predicates`. If any fail → same retry path as (d).
>    f. Write `GrillTurnCapsuleBody`, get turn_cid; append to `turn_cids[]`
>    g. If LLM emits `done=true`:
>       - Run `termination_predicate`. If pass → break loop.
>       - If fail → inject "You declared done but slot X is missing. Ask one more question." continue loop.
>    h. Display question to user (CLI: stdin); read user answer; CAS-store answer bytes; record answer_cid
>    i. Update coverage_state from `covered_slots` field
> 3. After loop:
>    - Reconstruct answers list: extract each user answer in order, map to one of the 8 canonical slot ids by parsing payload's `covered_slots` deltas
>    - **Call existing Phase 6.3 synthesis path**:
>      `wrap_spec_md(...)` + `write_spec_capsule(...)` — these stay byte-identical to legacy
>    - Get `final_spec_capsule_cid`
>    - Write `GrillSessionCapsuleBody` with all turn_cids + final_spec_capsule_cid
>    - Print session_capsule_cid to stdout
>
> Allowed edits:
> - `src/bin/turingos/cmd_spec.rs` (extend; NO change to canonical_questions or system_prompt)
> - `tests/cmd_spec_driven_mode_stub.rs` (new)
>
> Required tests (use stub LLM via `TURINGOS_SILICONFLOW_ENDPOINT` redirect):
> - `driven_mode_completes_in_8_turns_when_llm_cooperative` (stub returns valid turns 1-7 + terminal turn 8 with playback)
> - `driven_mode_forces_terminate_at_turn_15` (stub never emits done=true)
> - `driven_mode_halts_on_double_predicate_fail`
> - `driven_mode_final_spec_capsule_resolves_v1_schema`
> - `driven_mode_session_capsule_has_8_turn_cids`
> - `static_mode_unchanged_byte_compat` (run --mode static with legacy 8-answer fixture; verify spec.md sha256 matches snapshot)
>
> Done criteria:
> - `cargo build --bin turingos` passes
> - `cargo test --test cmd_spec_driven_mode_stub --no-fail-fast` all passing
> - `cargo test --test spec_grill_byte_compat --no-fail-fast` all passing
> - `canonical_questions` and `system_prompt` source bodies unchanged (verify via `git diff src/bin/turingos/cmd_spec.rs | grep -E '^-.*canonical_questions|^-.*system_prompt'` returns nothing)

**Acceptance test**:
```bash
cargo build --bin turingos
cargo test --test cmd_spec_driven_mode_stub --test spec_grill_byte_compat --no-fail-fast
./target/debug/turingos spec --help 2>&1 | grep -q "mode"
cargo fmt --all -- --check
```

---

### W7 — web/spec.rs POST /api/spec/turn

**Agent role**: Sonnet 4.6
**Depends on**: W6
**Risk class**: Class 2
**LOC**: ~220 LOC

**Brief**:

> Extend `src/web/spec.rs` (currently 628 LOC). Do NOT modify existing
> `spec_questions_handler` or `spec_submit_handler`. Add new handler below.
>
> New handler:
>
> ```rust
> pub(crate) async fn spec_turn_handler(
>     State(state): State<AppState>,
>     Json(req): Json<SpecTurnRequest>,
> ) -> Result<Json<SpecTurnResponse>, (StatusCode, String)> { ... }
>
> #[derive(Deserialize)]
> pub(crate) struct SpecTurnRequest {
>     pub session_id: String,
>     pub user_answer: Option<String>,        // None on turn 1's initial call (no prior answer)
>     pub lang: Option<String>,                // "zh" | "en"; only honored on first call per session
> }
>
> #[derive(Serialize)]
> pub(crate) struct SpecTurnResponse {
>     pub turn_index: u32,
>     pub question_text: Option<String>,
>     pub covered_slots: Vec<String>,
>     pub open_slots: Vec<String>,
>     pub confidence: f64,
>     pub done: bool,
>     pub playback: Option<String>,
>     pub terminated: bool,
>     pub spec_capsule_cid: Option<String>,    // set when terminated=true
>     pub turn_capsule_cid: String,
> }
> ```
>
> Extend `AppState` in `src/web/ws.rs`:
>
> ```rust
> pub struct AppState {
>     // ... existing fields ...
>     pub sessions: Arc<Mutex<HashMap<String, GrillSession>>>,
> }
>
> pub struct GrillSession {
>     pub turn_count: u32,
>     pub coverage_state: HashMap<String, SlotState>,  // canonical slot id → SlotState
>     pub last_3_turns: VecDeque<(String, String)>,    // (question_text, user_answer) pairs
>     pub terminated: bool,
>     pub parent_turn_cid: Option<String>,
>     pub turn_cids: Vec<String>,
>     pub lang: String,
>     pub created_at_unix: u64,
> }
>
> pub enum SlotState { Empty, Partial, Satisfied }
> ```
>
> Extend `WsBroadcastMsg`:
>
> ```rust
> pub enum WsBroadcastMsg {
>     // ... existing variants ...
>     SpecTurnAdvanced { session_id: String, turn_index: u32, question_text: String },
>     SpecGrillComplete { session_id: String, spec_capsule_cid: String },
> }
> ```
>
> Handler logic:
> 1. Validate `session_id` via existing `is_safe_session_id` (line 424)
> 2. Validate `user_answer` length ≤ 4096 chars (existing 4096 rule from validate_answers)
> 3. Lock sessions map; create or fetch GrillSession
> 4. If session.terminated → return 400
> 5. If session.turn_count == 0 (first call) and user_answer.is_some() → error (first call has no prior answer)
> 6. If session.turn_count >= 1 and user_answer.is_none() → error
> 7. Spawn-blocking: shell out `turingos llm complete --workspace <ws> --role meta --prompt-file - --strict-json --capsule-dir <session>/capsules --turn-id turn-<N> --lang <zh|en>` with the assembled message array piped to stdin
> 8. Parse stdout JSON; run grill_predicates; on fail retry once
> 9. Write turn capsule; broadcast `SpecTurnAdvanced` to WS subscribers
> 10. Update GrillSession state
> 11. If LLM done=true AND termination_predicate passes:
>     - Trigger synthesis (shell out `turingos spec --workspace <ws> --session <id> --mode driven --synthesize-only`)
>     - Get spec_capsule_cid; broadcast `SpecGrillComplete`
> 12. Return SpecTurnResponse
>
> Register new route in `src/web/router.rs`:
> ```rust
> .route("/api/spec/turn", post(spec::spec_turn_handler))
> ```
>
> Allowed edits:
> - `src/web/spec.rs` (extend; do NOT modify existing handlers)
> - `src/web/ws.rs` (extend AppState + WsBroadcastMsg)
> - `src/web/router.rs` (add route)
> - `src/web/mod.rs` (re-export GrillSession if test accesses it)
> - `tests/web_spec_turn_endpoint.rs` (new)
>
> Test file uses stub LLM via env override (same pattern as W4 test).
>
> Required tests:
> - `first_turn_call_no_user_answer_required`
> - `subsequent_turn_requires_user_answer`
> - `rejects_invalid_session_id`
> - `rejects_user_answer_over_4096_chars`
> - `terminates_after_llm_done_and_predicate_pass`
> - `broadcasts_spec_turn_advanced_on_each_turn`
> - `broadcasts_spec_grill_complete_on_termination`
>
> Done criteria:
> - `cargo build --bin turingos_web --features web` passes
> - `cargo test --test web_spec_turn_endpoint --features web --no-fail-fast` all passing
> - Existing `spec_questions_handler` + `spec_submit_handler` source bodies unchanged

**Acceptance test**:
```bash
cargo build --bin turingos_web --features web
cargo test --test web_spec_turn_endpoint --features web --no-fail-fast
git diff src/web/spec.rs | grep -E '^-.*spec_questions_handler|^-.*spec_submit_handler' | wc -l  # must be 0
cargo fmt --all -- --check
```

---

### W8 — frontend `<tos-spec-grill>` driven mode

**Agent role**: Sonnet 4.6
**Depends on**: W7
**Risk class**: Class 2
**LOC**: ~200 LOC

**Brief**:

> Extend `frontend/src/components/spec-grill.ts` (currently 375 LOC).
> Do NOT modify existing static-mode rendering. Add driven mode behind URL
> param `?mode=driven`.
>
> State machine:
> ```
> idle
>   → on CTA click → POST /api/spec/turn with {session_id, user_answer: null, lang}
>     → response received → render question
>   → awaiting_user_answer
>     → on form submit → POST /api/spec/turn with {session_id, user_answer: <text>}
>       → response received with done=false → render next question
>       → response received with done=true + playback → render playback step
>   → playback_review
>     → user confirms → POST /api/spec/turn one more time with {session_id, user_answer: "确认"}
>       → response with terminated=true + spec_capsule_cid → render complete state
>     → user requests edit → reset to awaiting_user_answer
>   → complete
> ```
>
> Error handling:
> - 5xx response → display "网络不稳，请稍等" toast, retry once
> - Two consecutive 5xx → fall back to legacy static mode (one-time toast: "切换至 8 问经典模式"); call existing handler
> - Predicate-fail returned from backend → display "AI 输出异常，重试中…" + auto-retry once via backend's retry path
>
> WS listener: subscribe to `SpecTurnAdvanced` + `SpecGrillComplete`.
> Use existing `<turingos-root>` WS plumbing.
>
> Allowed edits:
> - `frontend/src/components/spec-grill.ts` (extend)
> - `frontend/src/types/spec.ts` (new or extend; add TurnPayload, TurnResponse interfaces)
> - `frontend/tests/spec-grill-driven.test.ts` (new; vitest)
>
> Required tests:
> - `static_mode_unchanged_when_url_param_absent`
> - `driven_mode_renders_first_question_from_backend`
> - `driven_mode_submits_each_answer_to_turn_endpoint`
> - `driven_mode_renders_playback_on_done_true`
> - `driven_mode_falls_back_to_static_after_two_5xx`
>
> Done criteria:
> - `cd frontend && npm test` passes
> - `cd frontend && npm run build` exit code 0
> - No edits outside allowed list
> - Static mode (no URL param) rendering byte-identical (via vitest snapshot
>   test against pre-W8 snapshot if available, else visual review)

**Acceptance test**:
```bash
cd frontend
npm install
npm test
npm run build
```

---

### W9 — Real-LLM E2E witness

**Agent role**: Opus, Chrome-driving (per §6a)
**Depends on**: W8
**Risk class**: Class 3 (production credential use)
**LOC**: 0 (no source edits; only evidence)

**Brief**: see §6 + §6a verbatim.

Agent dispatch (orchestrator does this):

```
Agent({
  description: "Phase 6.3.x driven grill E2E witness",
  subagent_type: "general-purpose",
  model: "opus",
  prompt: <verbatim §6 + §6a contents + path to evidence dir>,
})
```

**Acceptance**: `agent_verdict.json` field `overall_verdict == "PASS"`.

---

### W10 — Clean-context Audit

**Agent role**: Opus xhigh (fresh context, no implementation transcript)
**Depends on**: W9
**Risk class**: Audit gate

**Brief**:

> You are a clean-context auditor reviewing TISR Phase 6.3.x. Do NOT read
> the implementation chat transcript. Read only the following:
>
> Required reading:
> 1. `handover/directives/2026-05-18_TISR_PHASE6_3_X_GRILL_LLM_DRIVEN_SECTION8_PACKET.md` (this charter)
> 2. `git diff <base-sha>..HEAD` for branch `codex/tisr-phase6-3-x-grill-driven`
> 3. `handover/evidence/stage_phase6_3_x_grill_driven_<timestamp>/agent_verdict.json`
> 4. `handover/evidence/stage_phase6_3_x_grill_driven_<timestamp>/cas_walk_output.txt`
> 5. `handover/evidence/stage_phase6_3_x_grill_driven_<timestamp>/replay_diff.txt`
> 6. `handover/evidence/stage_phase6_3_x_grill_driven_<timestamp>/legacy_byte_compat_hash.txt`
>
> Verify each item below; cite file:line for any finding:
>
> - **C1 (Class-4 untouched)**: Diff touches zero files in §3 forbidden list.
>   `git diff --name-only <base>..HEAD` shows zero matches for
>   `src/state/sequencer.rs|src/state/typed_tx.rs|src/bottom_white/cas/schema.rs|src/runtime/prompt_capsule.rs|src/kernel.rs|src/bus.rs|src/sdk/tools/wallet.rs|genesis_payload.toml|Cargo.toml|Cargo.lock`.
>
> - **C2 (FC1 anchoring)**: Every LLM turn produces (PromptCapsule, AttemptTelemetry, EvidenceCapsule) triple in CAS. Witness: `cas_walk_output.txt` lists N turn capsules and N AttemptTelemetry CIDs.
>
> - **C3 (Context-hash identity)**: For every turn, AttemptTelemetry.prompt_context_hash == PromptCapsule.prompt_context_hash. Witness: `cas_walk_output.txt` shows identical hash per turn pair.
>
> - **C4 (Replay-without-recall)**: `replay_diff.txt` is empty AND replay process made zero outbound SiliconFlow network calls.
>
> - **C5 (Legacy byte-compat)**: `legacy_byte_compat_hash.txt` shows static-mode spec.md sha256 == snapshot sha256 (I-7).
>
> - **C6 (Termination predicate enforced)**: Termination required {job, anchor, memory, first_run, robustness, scope, acceptance} ⊆ covered_slots. Witness: final turn capsule's `predicate_verdicts.termination_predicate_pass == true` AND payload.covered_slots ⊇ those 7 slots.
>
> - **C7 (Schema names correct)**: Diff introduces strings `"turingos-spec-grill-turn-v1"` and `"turingos-spec-grill-session-v1"` exactly. NOT `"turingos-ca-*"`. Existing `"turingos-spec-capsule-v1"` unchanged.
>
> - **C8 (No Class-4 imports)**: New modules `grill_envelope.rs` and `grill_predicates.rs` do not import anything from `src/state/sequencer.rs` or `src/state/typed_tx.rs`.
>
> - **C9 (LOC budget)**: Total LOC added ≤ 1500. Compute via `git diff --stat <base>..HEAD | tail -1`.
>
> - **C10 (No new dependencies)**: `Cargo.toml` diff is empty.
>
> Output verdict as final markdown:
> ```
> # AUDITOR_TISR_PHASE6_3_X_<run>_VERDICT
>
> Verdict: PROCEED | CHALLENGE | VETO
>
> Findings (cite file:line):
>   ...
>
> Critical defects (block ship):
>   ...
>
> Non-blocking concerns:
>   ...
> ```
>
> Conservative interpretation per AGENTS.md §9:
> - VETO blocks ship
> - CHALLENGE requires fix or explicit forward-defer with rationale
> - PROCEED is necessary but not sufficient; gates + evidence still required

Agent dispatch:

```
Agent({
  description: "Phase 6.3.x clean-context audit",
  subagent_type: "general-purpose",
  model: "opus",
  prompt: <verbatim W10 brief above>,
})
```

**Acceptance**: written `handover/audits/AUDITOR_TISR_PHASE6_3_X_R1_*.md` with verdict = PROCEED.

---

## §10 Risk Register

| ID | Risk | Source | Severity | Mitigation |
|---|---|---|---|---|
| R-1 | LLM nondeterminism breaks replay | Researcher A §3.3, B §8.2 | High | Replay reads parsed turn payload from `candidate_payload_cid` (CAS-resident); does NOT re-invoke LLM. Test I-8 verifies. |
| R-2 | Premature termination (LLM declares done with empty slots) | Researcher A §5.2 | High | Termination predicate (W3) requires 7 required slots ⊆ covered. Test in grill_predicates_termination. |
| R-3 | User answer abuse / prompt injection | Researcher A §5.3 | Medium | Per-answer 4096-char cap (existing). Blackbox triage deferred to atom-1.5 (v1 ships without). |
| R-4 | LLM produces non-Chinese output when lang=zh | Researcher A §5.1 | Medium | P6 language predicate (Han-script ratio ≥ 0.5). Retry-once on fail. |
| R-5 | Cost overrun (LLM loops without progressing) | Researcher A §5.4, B §8.5 | Medium | Hard ceiling N=15 (P5). Monotonicity predicate P4. |
| R-6 | Cross-session prompt leakage in AppState | Researcher B §8.1 | High | session_id validated via is_safe_session_id. PromptCapsule.read_set per turn limited to that session's last 3 turn cids. |
| R-7 | rationale_brief field leaks into next prompt | Researcher B §8.6 | Medium | Structured-history view (last 3 Q/A only) excludes rationale_brief from next prompt assembly. |
| R-8 | Premature generalisation (CCS abstraction) | Researcher C §9 | High | EXPLICITLY DEFERRED. This atom ships zero CCS. Second real grill instance is the trigger for CCS design. |
| R-9 | Legacy --mode static regression | Direct risk | Critical | Test I-7 (`spec_grill_byte_compat::legacy_path_byte_identical`) is a hard gate. |
| R-10 | Phase 7 web mode regression | Direct risk | High | Existing `/api/spec/questions` + `/api/spec/submit` handlers untouched; legacy frontend mode preserved. |
| R-11 | Sonnet/Haiku sub-agent over-reach (touches Class-4 surface) | Multi-agent execution risk | Critical | Each atom spec has explicit "Allowed edits" list. Orchestrator runs `git diff --name-only` after each atom and aborts if forbidden files appear. |
| R-12 | Audit Opus context window saturation | Audit risk | Medium | W10 brief restricts auditor reading list to ~6 files; no implementation transcript. xhigh thinking budget. |

---

## §11 Backout / Rollback

If §6 witness fails irrecoverably:
1. Branch `codex/tisr-phase6-3-x-grill-driven` is not merged.
2. Phase 7 main remains the production HEAD (already on main).
3. Frontend URL `?mode=driven` returns 404 (handler never registered) — silent fallback to static mode by frontend.
4. No CAS evidence regression possible: new schemas (`turingos-spec-grill-turn-v1` etc.) coexist with existing capsules but are never read by legacy code paths.
5. Architect files OBS at `handover/alignment/OBS_R022_TISR_PHASE6_3_X_<reason>.md` documenting why witness failed.

If §6 witness passes but W10 audit returns VETO:
1. Branch not merged. Architect reviews findings.
2. If findings are fix-in-place: dispatch a remediation atom (W11.X); re-run W10.
3. If findings indicate constitution violation: stop; file `handover/observations/CONSTITUTION_BREAK_TISR_PHASE6_3_X_*`; escalate.

If §6 witness passes + W10 audit returns CHALLENGE:
1. Architect reads challenge text.
2. Either fix-in-place (dispatch W11.X) or forward-defer with rationale documented in `handover/directives/2026-05-18_TISR_PHASE6_3_X_FORWARD_DEFER.md`.
3. Ship requires explicit architect §8 amendment overriding CHALLENGE.

---

## §12 Glossary

- **Driven mode**: new LLM-driven grill mode (this packet)
- **Static mode**: legacy 8-hardcoded-question mode (Phase 6.3, unchanged)
- **TurnPayload**: parsed JSON envelope from one LLM turn (see W2 spec)
- **GrillSession**: in-memory AppState entry for one active session
- **GrillTurnCapsuleBody**: CAS-anchored evidence body for one turn
- **GrillSessionCapsuleBody**: CAS-anchored rollup of full session
- **Canonical slot vocab**: `{job, anchor, memory, first_run, robustness, scope, acceptance, mirror}` (8 ids; 7 required, mirror optional)
- **Required slot subset**: the 7 slots needed for termination
- **Coverage state**: per-session mapping of slot id → {empty, partial, satisfied}
- **Structured history**: prompt-assembly view that includes only last 3 Q/A pairs (not full history)
- **Replay-without-recall**: replay reads parsed payload from CAS; does not re-invoke LLM

---

## §13 Appendix A — Section 8 Pre-flight Questions for Architect

Before signing §8, architect should confirm answers to these:

1. **Q: Confirm scope decision matches §1 (B's envelope + B's naming + A's predicates + deferred Blackbox + rejected CCS)?**
   - [ ] Yes, proceed
   - [ ] Revise: ____________

2. **Q: LOC budget OK at 900 target / 1500 cap?**
   - [ ] Yes
   - [ ] Different: ____________

3. **Q: Turn ceiling = 15 is hard (no override)?**
   - [ ] Yes
   - [ ] Different: ____________

4. **Q: Required slot subset = 7 (job, anchor, memory, first_run, robustness, scope, acceptance) confirmed?**
   - [ ] Yes
   - [ ] Different: ____________

5. **Q: SiliconFlow live API authorized for §6 witness?**
   - [ ] Yes (current key from env)
   - [ ] Different provider: ____________
   - [ ] Witness must use stub only: ____________

6. **Q: Frontend fallback to legacy static mode after 2 5xx is acceptable UX?**
   - [ ] Yes
   - [ ] Different: ____________

7. **Q: Branch name `codex/tisr-phase6-3-x-grill-driven` OK?**
   - [ ] Yes
   - [ ] Different: ____________

8. **Q: Dispatch order W0 → W1 → (W2,W3 parallel) → (W4,W5 parallel after W2,W3) → W6 → W7 → W8 → W9 → W10 acceptable?**
   - [ ] Yes
   - [ ] Different sequencing: ____________

9. **Q: Mid-flight ratification: architect wants checkpoint after which atoms?**
   - [ ] After W3 (predicates complete, before LLM wire-up)
   - [ ] After W6 (CLI driven mode runs against stub)
   - [ ] After W8 (full stack runs against stub)
   - [ ] After W9 (real-LLM evidence exists)
   - [ ] Multiple of the above: ____________

10. **Q: Audit model: Opus xhigh, or other?**
    - [ ] Opus xhigh
    - [ ] Different: ____________

---

## §14 Appendix B — Trace Matrix Row

For future `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md` updates after ship:

```
| Phase 6.3.x — Software 3.0 LLM-driven grill | application-layer wire-up | 
  tests/grill_predicates_*.rs + tests/grill_turn_capsule_*.rs + tests/constitution_grill_driven_anchors.rs + tests/spec_grill_byte_compat.rs | 
  handover/evidence/stage_phase6_3_x_grill_driven_<ts>/ + AUDITOR_TISR_PHASE6_3_X_R1_PROCEED.md | 
  🟢 LANDED (post-ship) | 
  legacy static mode regresses; CCS framework added unilaterally
```

---

**End of packet. Awaiting architect §8 signature.**
