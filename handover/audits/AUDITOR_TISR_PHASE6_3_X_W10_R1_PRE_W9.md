# AUDITOR_TISR_PHASE6_3_X_W10_R1_PRE_W9

Auditor: clean-context Opus xhigh
Date: 2026-05-18
Target branch: codex/tisr-phase6-3-x-grill-driven HEAD 3e0fa79c
Diff base: origin/main HEAD 886f7596
Scope: pre-W9 static audit (5 of 10 charter §9.W10 checks; remaining 5 deferred to W10-R2 post-W9 evidence)

## Verdict: PROCEED

The diff is constitution-clean for the pre-evidence audit scope. All five static checks pass. R2 amendments (§A1, §A2, §A3, §A6, §A7, §A8) and R2.1 path canon are honored in code. No Class-4 surface is touched; no new Cargo dependencies; no genesis_payload.toml change. LOC ~3920 src + 879 frontend = ~4799 (under R2.2 §A17 5000-LOC cap). W9 may dispatch.

## Critical violations (block W9)
- None.

## Challenges (require fix before W9, OR explicit architect deferral)
- None at the blocking severity.

## Non-blocking observations

- [O1] `src/runtime/grill_predicates.rs:229` — Termination predicate maps "required slot missing" to `PredicateFailureClass::QuestionMissing` with an inline TODO comment noting a future `SlotRequiredMissing` tail-additive variant. Semantically correct (predicate still returns Fail with a typed class), but the symbolic mismatch will surface in any future audit grep on `PredicateFailureClass::QuestionMissing` occurrences. Track as a Phase 6.3.y tail-additive enum extension.

- [O2] `tests/spec_grill_byte_compat.rs` does NOT mechanically enforce invariant I-7 (legacy `--mode static` produces byte-identical spec.md vs pre-W6). The test verifies the binary runs successfully and creates `spec.md`, but no byte-identity comparison against a committed pre-W6 snapshot exists. This is acknowledged in test comments (lines 17-20: "full byte-identity testing requires a known-good pre-W6 spec.md snapshot which we don't have committed"). **Mechanical I-7 enforcement is deferred to W10-R2 C5 via `legacy_byte_compat_hash.txt`.**

- [O3] `src/web/spec.rs:874,973,1086` — three `sessions.get(...).unwrap()` / `get_mut(...).unwrap()` calls after lock re-acquisition. Comments mark them "safe: we just inserted". Strictly there is a TOCTOU window: between the prior lock release and re-acquisition, a concurrent request could in principle remove the session. In this codebase no code path removes sessions during a process lifetime (only `terminated = true`), so this is currently theoretical. Worth a `sessions.get(...).ok_or(...)` defensive refactor in a future hygiene pass; not a W9 blocker.

- [O4] Asset path resolution: `cmd_llm.rs:1083` resolves `assets/prompts/grill_triage_blackbox_v1.md` relative to `--workspace`. For the W9 dispatch, the workspace must contain `assets/prompts/` (either by running with cwd=repo-root, or by the dispatch script copying assets in). This is an integration concern documented in the charter (`cmd_spec.rs:75-76` help string: "Default: assets/prompts/grill_meta_v1.md (relative to workspace)"). Verify before W9 dispatch.

- [O5] `src/web/ws.rs:72` — `WsBroadcastMsg` visibility loosened from `pub(crate)` to `pub` to support the integration test in `tests/web_spec_turn_endpoint.rs`. Justified, but expands the public API of the library crate. Acceptable for v1; consider re-tightening via a `#[doc(hidden)]` annotation or a test-only feature flag later.

- [O6] `src/bin/turingos/cmd_llm.rs:905` — `PromptCapsule.read_set` is `BTreeSet::new()` (empty) for Phase 6.3.x v1. Documented inline as "empty read_set for Phase 6.3.x v1". The grill flow doesn't yet record which transcript turns the LLM actually read; future enhancement.

## Per-check verdicts

### C1: Class-4 surface untouched
**PASS**.
- `git diff --name-only origin/main..HEAD | rg '^(src/(state/(sequencer|typed_tx)\.rs|kernel\.rs|bus\.rs|sdk/tools/wallet\.rs|bottom_white/cas/schema\.rs|runtime/(prompt_capsule|attempt_telemetry)\.rs)|genesis_payload\.toml|Cargo\.(toml|lock))$'` → ZERO MATCHES.
- `git diff origin/main..HEAD -- src/state/ src/bottom_white/cas/schema.rs src/runtime/prompt_capsule.rs src/runtime/attempt_telemetry.rs src/kernel.rs src/bus.rs src/sdk/tools/wallet.rs` → empty diff.
- `PromptCapsule` struct fields (the 7 architect-pinned fields per CLAUDE.md §4.3) are NOT mutated; the diff only USES `PromptCapsule::new(...)` from `cmd_llm.rs:903`.
- `AttemptOutcome` enum is NOT extended; the diff defines a separate `GrillAttemptOutcome` enum in `src/bin/turingos/spec_capsule.rs:185-197` per R2 §A1 hard rule.

### C7: Capsule schema names
**PASS**.
- `"turingos-spec-grill-turn-v1"` introduced in `src/bin/turingos/spec_capsule.rs:174` (and asserted by `grill_turn_capsule_schema_id_is_v1` test at line 528-533).
- `"turingos-spec-grill-session-v1"` introduced in `src/bin/turingos/spec_capsule.rs:177` (asserted by test at line 535-541).
- `"turingos-spec-capsule-v1"` unchanged (still `pub(crate) const SPEC_CAPSULE_SCHEMA_ID: &str = "turingos-spec-capsule-v1";` at `spec_capsule.rs:34`; asserted unchanged at line 522-525).
- `turingos-ca-*` (Researcher C's naming) does NOT appear anywhere in the actual source tree. All `turingos-ca-*` matches in `git diff` output are confined to research/charter/audit markdown documents (Researcher C's DESIGN.md and pre-impl audit recommendations) — they are not in compiled source.

### C8: No Class-4 imports in new modules
**PASS**.
- `rg 'use crate::state|use crate::bottom_white::cas::schema|AttemptOutcome|AttemptKind' src/runtime/grill_envelope.rs src/runtime/grill_predicates.rs` → ZERO HITS.
- `src/runtime/grill_envelope.rs` only uses `serde::{Deserialize, Serialize}` and `std::fmt` (pure-data module).
- `src/runtime/grill_predicates.rs` only uses `serde::{Deserialize, Serialize}` and `crate::runtime::grill_envelope::{TurnPayload, CANONICAL_SLOTS, REQUIRED_SLOTS}` (pure pure-function module).

### C9-revised: LOC budget (cap 5000 source per R2.2 §A17)
**PASS**.
- Rust source (`src/**/*.rs` excluding tests): **3920** insertions (under 5000 cap).
- Frontend (`frontend/src/**`): 879 insertions (TS source + types).
- Combined source: **~4799** (well under 5000 cap).
- Rust tests (`tests/**/*.rs`): 1467 insertions.
- Frontend tests (`frontend/test/**`): 352 insertions.
- Combined tests: **~1819** (under 2500 cap).
- Prompt assets: 163 lines.
- Total touched (incl. directives/charter/audit md): **11774 insertions, 9 deletions** per `git diff --stat`. Within 7500 hard cap when filtered to source+tests+assets only (~6800).

### C10: No new Cargo dependencies
**PASS**.
- `git diff origin/main..HEAD -- Cargo.toml Cargo.lock` → empty.

## Architectural assessment

### A. Latent bugs that would break W9

- **A.1 (low)** `src/web/spec.rs:874,973,1086`: three `.unwrap()` calls in async handler relying on session existence between lock release/re-acquire. No code path currently removes sessions, so this is theoretical. Recommend defensive `ok_or(...)` refactor post-ship. **NOT a W9 blocker.**

- **A.2 (low)** `src/web/spec.rs:888-911`: when `turn_count >= 15` the handler broadcasts `SpecGrillComplete` with empty `spec_capsule_cid`, then sets `terminated = true`, then returns 200 OK. Frontend must distinguish "successfully completed" from "force-terminated" via the empty CID. Frontend code in `frontend/src/components/spec-grill.ts` handles this via the `terminated` field on the response.

- **A.3 (negligible)** Asset path resolution (see O4) depends on workspace layout including `assets/prompts/`. If the W9 dispatcher runs with cwd ≠ repo root and doesn't bind-mount or copy assets, the triage / meta-prompt reads will fail with IO error → exit code 4 → all turns return `ok=false` → double-retry exhausted → `predicate_double_fail` session termination. **Pre-W9 sanity check: verify dispatch CWD or workspace layout.**

- **A.4 (none)** No race conditions found in `AppState.sessions` HashMap access — all critical sections use `std::sync::Mutex` with short-lived locks (no `.await` while holding lock).

- **A.5 (none)** `spawn_blocking` usage for `Command::output()` shell-outs is correct — properly awaited, results extracted before next lock acquisition.

- **A.6 (none)** Path-traversal hardening: `is_safe_session_id(s)` rejects anything outside `^[a-zA-Z0-9_-]{1,128}$` (`src/web/spec.rs:424-430`).

- **A.7 (none)** No removed/stale env vars referenced; `SILICONFLOW_API_KEY` plumbing intact.

- **A.8 (none)** Shell-out args order verified: `--workspace`, `--prompt-file`, `--capsule-dir`, `--turn-id`, `--lang`, `--meta-prompt`, `--strict-json` all match `cmd_llm.rs` arg parser.

### B. Architectural concerns

- **B.1 (PASS)** Static-mode legacy path byte-identity: the legacy `run_inner` (`cmd_spec.rs:154-362`) is byte-untouched for the synthesis path — `canonical_questions`, `system_prompt`, `wrap_spec_md`, `synthesise_spec_md_no_llm`, `build_synthesis_user_message`, `write_transcript_jsonl` are all untouched (no diff hunks intersect their line ranges). The only changes to the legacy code path are (a) `--mode` and `--meta-prompt` flag parsing additions (lines 198-238), (b) the dispatch branch at line 247-260 that routes `SpecMode::Driven` to `run_driven_mode` BEFORE reaching the legacy synthesis logic, and (c) one help-string deletion at line 60-69. None of these affect the actual byte output of static mode. **Mechanical I-7 enforcement deferred to W10-R2 C5.**

- **B.2 (PASS)** Driven-mode termination flow correctly writes `GrillSessionCapsuleBody` before exit: `cmd_spec.rs:1305-1378` runs unconditionally after the while-loop breaks — both for clean termination (`llm_done_predicate_pass`) AND for partial sessions (`predicate_double_fail`, `user_input_unparseable`, `turn_limit_forced`). `partial_session` boolean is correctly set at line 1312. Session capsule is written at line 1378.

- **B.3 (PASS)** Triage call ordering: per R2 §A5 flow, triage fires on user_answer at the END of turn N (`cmd_spec.rs:1213-1281`), AFTER `read_line` from stdin (line 1202-1206) but BEFORE the next turn's Meta-complete call. Non-relevant answers loop the same turn via `turn_index -= 1` (line 1251). The interpretation "triage call before the LLM-complete call on subsequent turns" is satisfied: at turn N+1, triage has already classified turn N's answer.

- **B.4 (PASS)** Termination predicate dual-gating: when LLM emits `done=true`, code at `cmd_spec.rs:1144-1156` calls `termination_predicate(&payload)` independently. If the LLM declares done but slots are still missing, the predicate fails → `GrillAttemptOutcome::TerminationGated` capsule is written (line 1180-1188), nudge injected, loop continues. Final termination requires BOTH LLM done=true AND predicate pass.

- **B.5 (CONCERN, advisory)** Static-mode answers reconstruction in driven mode: `cmd_spec.rs:1321-1326` reuses `canonical_questions(lang)` and pads `all_user_answers` to 8 entries with placeholder text "(not collected in driven session)" for synthesis. If a driven session terminates early (e.g. turn 4 with only 4 user answers), 4 slots get placeholder text. This is documented inline. The resulting `spec.md` will have 4 "(not collected)" answer lines, which the LLM synthesis may or may not handle gracefully. For W9, ensure all 7 required slots are answered in the test scenario; otherwise document the placeholder behavior.

### C. Constitution alignment (R2 amendment honoring)

- **R2 §A1 (PASS)** AttemptOutcome non-reuse: confirmed via grep — `AttemptOutcome` / `AttemptKind` / `write_attempt_telemetry_to_cas` are NOT referenced anywhere in grill source. The diff defines a parallel `GrillAttemptOutcome` enum in `src/bin/turingos/spec_capsule.rs:185-197` with byte-stable discriminants (asserted by test `grill_attempt_outcome_discriminant_lock` at line 509-520). CLAUDE.md §6 `evaluator_reported_completed_llm_calls` semantics preserved.

- **R2 §A2 (PASS)** `hidden_fields_redacted = true`: hardcoded as `true` in `cmd_llm.rs:907` (with comment "R2 §A2 hard rule"). Constructor `PromptCapsule::new(...)` is called with the literal `true`.

- **R2 §A3 (PASS)** Termination predicate FC1-N9 session-aggregate: implemented in `grill_predicates.rs:217-235` as `termination_predicate(payload)` returning `PredicateVerdict`. Returns Pass IFF `covered_slots ⊇ REQUIRED_SLOTS` AND `confidence ≥ 0.8` AND `turn ≥ 4`. Failure looped (NOT L4.E) per `cmd_spec.rs:1144-1195`. FC1-N9 mapping documented in module doc.

- **R2 §A6 (PASS)** Independent `grill_attempt_count` tally: `GrillAttemptTally` struct defined separately in `spec_capsule.rs:262-269` with five `u32` counters. `CoverageState.to_grill_attempt_tally()` aggregates them. NOT summed into evaluator counters — written into the session capsule as a sibling field. Test `tests/grill_session_capsule.rs` validates the rollup.

- **R2 §A7 (PASS)** `--meta-prompt` flag: implemented in `cmd_llm.rs:559-561` (complete sub-action) and `cmd_spec.rs:230-235` (driven mode). Used at `cmd_llm.rs:830-895` to compute `system_prompt_template_hash` (sha256 of meta-prompt file content). Triage path explicitly passes `&None` at `cmd_llm.rs:1248` to fall back to first system message content. Help text confirms behavior.

- **R2 §A8 (PASS)** Typed `PredicateFailureClass` enum: defined in `grill_predicates.rs:44-70` with 10 byte-stable variants (`#[repr(u8)]`, explicit `= 0..=9` discriminants). `PredicateVerdict` is `Pass | Fail(PredicateFailureClass)`. All `PredicateVerdict::Fail(...)` constructions in the predicate body use typed classes. Test `grill_attempt_outcome_discriminant_lock` (in `spec_capsule.rs` test mod) covers the parallel `GrillAttemptOutcome` discriminants.

- **R2.1 path (PASS)** `grill_envelope.rs` location: confirmed at `src/runtime/grill_envelope.rs` (133 LOC). Imports work as documented: `grill_predicates.rs:11` uses `use crate::runtime::grill_envelope::{TurnPayload, CANONICAL_SLOTS, REQUIRED_SLOTS}` (R2.1 documented this as `use crate::grill_envelope::...` but the actual code uses `use crate::runtime::grill_envelope::...` which is the equivalent fully-qualified path from the library crate root). `cmd_llm.rs` / `cmd_spec.rs` / `web/spec.rs` use `turingosv4::runtime::grill_envelope::...` as documented.

### D. Karpathy fidelity check

- **Prompt-as-program (HIGH)**: `assets/prompts/grill_meta_v1.md` (65 lines) declares ROLE, METHODOLOGY (8 slot ids with semantics), CONSTRAINTS (one question per turn, plain language, no jargon, build on latest answer), OUTPUT CONTRACT (JSON envelope with turn/question/covered_slots/open_slots/confidence/done/rationale/playback), field semantics, redaction policy declaration ("none-applies-grill-v1" with audit trail). This IS a program in natural language. Hash of file content becomes `system_prompt_template_hash` in every PromptCapsule. STRONG fidelity.

- **Per-turn LLM content authority (HIGH)**: `cmd_spec.rs:976` parses the LLM's emitted `TurnPayload` via `parse_and_validate`; on success the `question_text` (line 1199) is displayed to user verbatim — the LLM chose the question. Kernel only enforces predicates (P1-P6 + termination), never injects content beyond retry nudges or termination "missing slot X" injection (line 1167-1170). STRONG fidelity.

- **Replay-without-recall (PARTIAL/PENDING-W10-R2)**: `GrillTurnCapsuleBody.turn_payload_snapshot: serde_json::Value` stores the parsed payload in CAS (`spec_capsule.rs:234`). The capsule also stores `prompt_capsule_cid`, `parent_turn_cid`, `predicate_verdicts`, `grill_attempt_record`. In principle replay-without-recall is feasible from these CAS reads. Actual replay verification deferred to W10-R2 (`replay_diff.txt`).

- **Dual-gated termination (HIGH)**: `cmd_spec.rs:1143-1156` implements LLM-declared `done=true` + independent kernel-side `termination_predicate(payload)` verification. If predicate fails, capsule outcome is `TerminationGated`, loop continues with injected nudge. Cannot terminate on LLM declaration alone. STRONG fidelity.

- **FC1 tape anchoring via EvidenceCapsule (PARTIAL/PENDING-W10-R2)**: Per R2 §A1, grill turns embed `GrillAttemptRecord` IN the EvidenceCapsule body (NOT as a separate AttemptTelemetry CAS object). The grill capsule schema_id distinguishes it from generic EvidenceCapsule. ObjectType is still `EvidenceCapsule` (re-used, not extended). Tape anchoring (via cas_index.jsonl) is implicit in the existing CasStore::put flow. Per-turn evidence chain (`parent_turn_cid`) gives the L4-style accept chain. Static check passes; per-turn anchoring in real run deferred to W10-R2 (`cas_walk_output.txt`).

- **Blackbox scheduler/worker (B → B+ upgrade per R2 §A5)**: W4.5 `turingos llm triage` is implemented in `cmd_llm.rs:1051-1305`. Called from W6 main loop at `cmd_spec.rs:1213-1221` after each user answer (relevant/off_topic/abusive/gibberish classes). Off-topic re-loops the turn, double-non-relevant aborts. This IS the scheduler/worker pattern: Meta plans (asks), user replies, Blackbox triages (classifies relevance), Meta resumes on next turn with cleaned context. STRONG upgrade from Karpathy R1's B rating. Note: W7 `src/web/spec.rs:914-1042` mirrors the triage flow for the web endpoint.

- **Overall rating: B+** (vs Karpathy R1's B). The Software 3.0 prompt-as-program is well-instantiated; per-turn LLM authority is faithful; dual-gated termination is mechanically enforced; the Blackbox scheduler/worker pattern is implemented end-to-end. Two areas need W10-R2 to upgrade to A: (1) mechanical replay-without-recall verification, and (2) FC1 anchor walk showing the parent_turn_cid chain. Both are evidence-dependent and deferred.

## Recommended pre-W9 actions

1. **Verify W9 dispatcher CWD or asset layout**: the dispatcher must either (a) set `cwd = /Users/zephryj/work/turingosv4/` (where `assets/prompts/` lives), or (b) copy `assets/prompts/` into the test workspace before invoking `turingos spec --mode driven` and `turingos llm triage`. Without this, all turns will fail with IO error on the prompt-asset read. (Per O4 / A.3.)

2. **Run a smoke test of `--mode driven`** with the existing binary against a throwaway workspace before committing real LLM spend. Confirm session capsule is written, turn capsules accumulate, parent_turn_cid chain holds.

3. **Optional cleanup (defer if time-pressed)**: replace the three `.unwrap()` calls in `src/web/spec.rs:874,973,1086` with `ok_or(...)` defensive returns. Theoretical TOCTOU risk only.

4. **Defer to W10-R2**: legacy byte-compat enforcement (O2), `SlotRequiredMissing` variant addition (O1), `read_set` population (O6).

## W10-R2 deferred checks (post-W9)

- C2: FC1 anchoring per turn (needs `cas_walk_output.txt` showing parent_turn_cid chain and PromptCapsule-CID linkage)
- C3: prompt_context_hash identity (needs `cas_walk_output.txt` confirming each `GrillAttemptRecord.prompt_context_hash` matches the sha256 of the messages-array bytes stored at `visible_context_cid`)
- C4: replay-without-recall (needs `replay_diff.txt` + network capture proving the second-pass reconstruction reads only from CAS, not via fresh LLM calls)
- C5: legacy byte-compat sha256 (needs `legacy_byte_compat_hash.txt`: sha256 of `spec.md` from `--mode static` against the same 8-answer fixture, compared to a pre-W6 committed snapshot or a re-built `origin/main` binary output)
- C6: termination predicate enforced 7-slot subset in real session (needs final turn capsule from W9 run showing `covered_slots ⊇ REQUIRED_SLOTS` at the terminating turn)
