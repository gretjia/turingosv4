# F9 Fix Report — Transcript Push Rollback on llm-complete Failure

**Status**: Done. Fix landed, regression test added and green, full `web_spec_turn_endpoint` suite (80 tests, was 79 before F10 added one) passes.

**Pre-fix HEAD**: `3e0fa79cbf999fafbf56eae67da2960a063cf5d5` (unchanged; F9 edits uncommitted on `codex/tisr-phase6-3-x-grill-driven`, alongside F10 and other in-flight fixes).

**Risk class**: 2 (production wire-up; `src/web/spec.rs` only — no Class-3/4 surfaces touched).

**Coordination note**: F10 fix agent ran in parallel and finished during this session. F10 touched the same handler (Step 11 of `spec_turn_handler`) to add `slot_evidence` population. The two edits are compatible: F9 establishes the `if let Some(user_answer) = req.user_answer.as_deref()` block inside Step 11 (after the successful `llm complete` parse) and pushes to `last_3_turns` + `all_user_answers`; F10's slot-keyed evidence map population reads the same `user_answer` variable inside the same block. Final code order is `last_3_turns push → all_user_answers push → slot_evidence insert`, all atomic under one `sessions.lock()` write guard.

## Sources (defect grounding)

- `handover/evidence/phase6_3_x_universality_1779111375/pi4/p5_codeswitch/verdict.json` — Π4.2 P5 code-switch verdict. `delta_verdict: REGRESSION vs M4`. Verdict author correctly identifies the dedup bug class: "the on-disk transcript dedup logic does not rollback after a 500, leading to duplicate (assistant,user) pairs in the next turn's Meta prompt."
- `handover/evidence/phase6_3_x_universality_1779111375/pi4/p5_codeswitch/turn5_main_evidence_capsule.json` — direct evidence. The captured `messages[]` for turn 5 shows assistant-T4-mirror at `[4]` and user-T4-answer at `[5]`, then the SAME assistant-T4-mirror at `[6]` and SAME user-T4-answer at `[7]`, immediately before the trailing `user: "Produce your turn-5 output per the contract."` instruction.
- `handover/evidence/phase6_3_x_universality_1779111375/pi4/p5_codeswitch/session_log.jsonl` — T5 first attempt elapsed 775ms (F6 silent-zero short-circuit on the corrupted prompt); T5 retry elapsed 1046ms (kernel `user_input_unparseable_no_spec` terminate).

The verdict was filed against P5 code-switch but the bug class is universal — any persona that hits ≥2 SiliconFlow transient 5xx failures (D8 rate: 5–43%) on consecutive turns will trigger it, surfacing as F6 silent-zero on the corrupted turn and (with A8 playback v2 active) `user_input_unparseable_no_spec` termination on the next.

## Root cause

In `src/web/spec.rs`, `spec_turn_handler` Step 9 (the triage-relevant branch, pre-F9 lines 1212–1232):

```rust
// Pre-F9 (BUGGY)
sess.last_3_turns.push_back((prev_question.clone(), user_answer.to_string()));
sess.all_user_answers.push(user_answer.to_string());
```

This push happened BEFORE Step 10's `turingos llm complete` shellout. If the shellout returned `ok=false` (which post-F6 correctly bubbles up as HTTP 500 with `kind: shellout_failed`), the handler returned `Err(...)` from Step 10 WITHOUT rolling back the Step 9 push. The session's `last_3_turns` and `all_user_answers` had been mutated, but `turn_count` had NOT advanced.

The client's natural recovery is to re-POST `/api/spec/turn` with the same `(session_id, user_answer)`. On retry:

1. Step 9 re-runs triage on the same answer → still relevant
2. Step 9 pushes the SAME `(prev_question, user_answer)` pair AGAIN — `last_3_turns` now contains the duplicate
3. Step 10 builds the prompt from the corrupted `last_3_turns`, writes the bytes to `turn-N-prompt.json` (overwriting the prior write), and shells out
4. The Meta LLM, given a history where the same user turn appears twice in immediate succession, emits a malformed/empty envelope
5. F6's `parse_turn_payload_from_llm_output` returns the envelope (which has `question=""`, `covered=[]`, `confidence=0.0`)
6. Step 11 advances `turn_count` and broadcasts `SpecTurnAdvanced` with the empty fields — the F6 "silent zero" surface
7. On the next turn the kernel's playback-v2 double-fail short-circuit (A8) fires, terminating with `user_input_unparseable_no_spec` and no spec.md

The CLI driven path (`src/bin/turingos/cmd_spec.rs:1140-1200`) is structurally immune because it lives inside a single Rust loop: the push at line 1288-1289 happens AFTER `shell_llm_complete` returns Ok AND `parse_and_validate` succeeds, and the retry-once is internal (uses the same `state.last_3_turns` vector without re-pushing). The web layer's bug arises specifically from the HTTP request/response boundary, where the client is the retry loop and each request enters the handler from a clean stack.

## Fix description

Two edits to `src/web/spec.rs`:

**Edit 1** (Step 9 triage-relevant branch, ~lines 1212–1253): Remove the eager push of `last_3_turns` and `all_user_answers`. Keep only the counter updates (`triage_calls_relevant += 1`, `non_relevant_count = 0` reset) which are pure bookkeeping and safe to re-run on retry.

**Edit 2** (Step 10 prompt-build, ~lines 1269–1290): Build `last_3_for_prompt` as a TRANSIENT snapshot — clone the session's persisted `last_3_turns`, then append the CURRENT `(prev_question, user_answer)` pair without mutating the session. Apply the same size-3 rolling-window pop logic on the clone. This makes the prompt content for a given turn-index deterministic w.r.t. `(session_state, user_answer)` regardless of how many transient-500 retries happen on the client side — the on-disk `turn-N-prompt.json` rewrites identically on retry.

**Edit 3** (Step 11 post-`llm complete` state update, ~lines 1440–1470): After `turn_count`, `meta_turns_accepted`, `last_question_emitted`, `parent_turn_cid`, and `coverage_state` are all updated, NOW push to `last_3_turns` and `all_user_answers`. This is the rollback-safe location: a transient upstream failure on the `llm complete` call above returns HTTP 500 from an earlier `?` (specifically from the `parse_turn_payload_from_llm_output` error branch at line ~1385) without reaching this block, leaving `last_3_turns` and `all_user_answers` untouched.

The push order mirrors the CLI driven path (`cmd_spec.rs:1288-1289`): rolling-3 window first, then the full ordered history. (F10's `slot_evidence` insert chains after this, inside the same `if let Some(user_answer)` block.)

## Verification

**Test invocation** (from `/Users/zephryj/work/turingosv4`):

```bash
cargo check --features web
cargo test --features web --test web_spec_retry_no_transcript_duplication
cargo test --features web --test web_spec_turn_endpoint
```

**Results**:

- `cargo check --features web` — clean (15 warnings in `turingos_web`, all pre-existing and unrelated; finished in 0.07s incremental).
- `cargo test --features web --test web_spec_retry_no_transcript_duplication` — 69 passed, 0 failed (68 inline unit tests from `src/web/*/tests` modules plus 1 new integration test `retry_after_llm_complete_flake_does_not_duplicate_user_turn`).
- `cargo test --features web --test web_spec_turn_endpoint` — 80 passed, 0 failed, 5 ignored. The "all existing 79 must pass" target is met; the count grew to 80 because F10 added `slot_evidence_attribution_uses_covered_slot_delta` to the `mod tests` block.

## Regression test added

New file: `tests/web_spec_retry_no_transcript_duplication.rs` (`#![cfg(feature = "web")]`).

The test stands up:
1. A real axum server (`web::router::build_with_state`) bound to a random port on `127.0.0.1`.
2. A `tempfile::tempdir()` workspace with a stub `assets/prompts/grill_meta_v1.md` (the F4 prompt-asset read is unconditional).
3. A `/bin/sh` stub binary at `TURINGOS_BACKEND_OVERRIDE` that:
   - Detects `triage` vs `complete` subcommand in argv
   - On `triage`: always emits `{"ok":true,"class":"relevant","class_confidence":0.95}`
   - On `complete`: increments a counter file; the 2nd call emits `{"ok":false,"content":"transient flake","parsed_envelope":null}`; all other calls emit a canned valid envelope

The test then drives the handler through three POSTs to `/api/spec/turn`:
- Bootstrap (null `user_answer`) → expect 200; this is the stub's 1st `llm complete` call
- Turn-2 first attempt (`user_answer = "I want a tracker for my grocery list"`) → expect 500 with body containing `shellout_failed` or `ok=false`; this is the stub's 2nd `llm complete` call which triggers the failure
- Turn-2 retry (same `user_answer`) → expect 200; this is the stub's 3rd `llm complete` call which succeeds

Final assertions read the persisted `<workspace>/sessions/<sid>/turn-2-prompt.json` and verify:
1. The `user_answer` string appears EXACTLY ONCE as the `content` of a `role: user` message.
2. The bootstrap question (`"What is your favourite colour?"`) appears EXACTLY ONCE as the `content` of a `role: assistant` message.

Pre-F9 both counts would be 2 (the original push + the retry push). Post-F9 both counts are 1.

A subtle property of the test: it does NOT assert against the response body of the failed turn-2 first attempt beyond status code + error kind. The point of the test is that the persisted prompt file is correct AFTER the retry — i.e. the in-memory `last_3_turns` was not corrupted by the failed attempt. The test exercises the exact failure shape the Π4.2 P5 evidence captured.

## Followup

This fix is necessary but not sufficient. The dedup bug is closed, but the underlying provider-flake rate (D8) remains the root cause of poor session continuity:

- If SiliconFlow's transient 5xx rate stays at the upper end of the 5–43% range observed in Π4 round 1, even with F9 the client UX is "you have to click Submit again, sometimes twice". A retry-with-exponential-backoff inside the handler (similar to the CLI's single internal retry at `cmd_spec.rs:884-973`) would mask this without changing the persisted-state contract, but is out of F9 scope.
- The Π4 round-2 universality re-run (orchestrator-scheduled) will exercise F9 against real backend flakes; if a clean P5 run completes with `done=true` + `spec_capsule_cid` populated, F9 is confirmed end-to-end. If the run still fails, the failure will be in a different surface (the persisted-prompt dedup is the only known bug class affected by consecutive 500s).
- F6's silent-zero pathway is now strictly dominated by F9 + F10: F9 prevents the corrupted prompt that triggers F6 silent-zero in the first place; F10 prevents D-NEW-3a slot-shift at synthesis. The F6 silent-zero fingerprint (775ms response with all-zero envelope) should not recur in Π4 round 2 evidence.

## Files modified

- `src/web/spec.rs` — three edit blocks in `spec_turn_handler`: Step 9 triage-relevant branch (push removed), Step 10 prompt-build (transient-snapshot append), Step 11 post-`llm complete` state update (deferred push). In-code RCA narrative ~80 LOC, executable delta ~25 LOC.
- `tests/web_spec_turn_endpoint.rs` — added `slot_evidence: BTreeMap::new()` to the `grill_session_default_constructs` test's `GrillSession` initializer (line 197). This was unrelated F10 fallout — F10 added the `slot_evidence` field to `GrillSession` but missed updating this test file, breaking compilation of the test binary. F9 fixed it because the same test binary contains my own F9 regression test (one cargo test target = one compilation unit).
- `tests/web_spec_retry_no_transcript_duplication.rs` — new file (~290 LOC including doc + stub script generator + assertions).

## Out of scope (NOT touched by F9)

- No Cargo.toml/Cargo.lock changes
- No Class-3/4 surfaces (`state/*`, `kernel.rs`, `bus.rs`, `typed_tx.rs`, etc.) modified
- No constitution/flowchart artifacts modified
- No `handover/ai-direct/LATEST.md` update (ship state is unchanged; F9 is a Class-2 in-flight fix on `codex/tisr-phase6-3-x-grill-driven`, not a Phase-7 ship event)
