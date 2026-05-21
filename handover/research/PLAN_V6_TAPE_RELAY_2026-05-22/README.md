# Plan v6 Tape-Relay Research Archive — 2026-05-22

| Field | Value |
|-------|-------|
| Date | 2026-05-21 evening → 2026-05-22 morning |
| Trigger | Architect's overnight directive: "我要看到 turingos 内核中，agent 可以在 tape 上接力完成任务，图灵机真的在运转" |
| Phase | A (research × 3) → Atom-T design+impl → empirical validation |
| Decision outcome | Atom-T built and shipped (commit `d8e0fda4`). Tape-relay validated on a 3-attempt chain with byte-different prompt_hash on each attempt. |
| Final report | `handover/architect-insights/PLAN_V6_OVERNIGHT_TAPE_RELAY_2026-05-22.md` (commit `0e190c95`) |

## Why this archive exists

Future TuringOS sessions revisiting "does the tape actually relay across attempts" should **read this archive first** instead of re-running the audit. The 3 research outputs cost ~3 sub-agent dispatches; the Atom-T fix landed in 75 LoC + 4 unit tests + 0 Cargo deps. Preserve the path so the next architect-question about "is this really a Turing machine" can be answered from documents, not freshly dispatched investigations.

## Files in this archive

| File | What it covers | When to re-read |
|------|----------------|-----------------|
| `A1_minif2f_historical_tape_relay.md` | Historical record dig in `handover/`. Operational definition of "tape relay" (parent ruling TB18_TAPE_NON_EXTERNALIZATION_VETO_2026-05-06). Past minif2f capsule-chain patterns. Confirmation that minif2f-era never exercised attempt-2-with-feedback either (single-tactic n1 runs only). | Before any future debate about whether v4's tape-relay matches the "canonical externalization rule." Cite TB18 verdict. |
| `A2_v4_kernel_multi_attempt_audit.md` | Code-level audit of cmd_generate.rs retry loop, parent_attempt_cid chaining, cross-invocation tape continuity, C11 feedback loop. Capsule cross-reference graph. Verdict: pre-Atom-T v4 was "fancy LLM wrapper" with write-only chain. | Before extending the tape-relay loop (e.g. adding HeuristicFailed-specific test_run feedback, cross-session relay, web-layer relay). |
| `A3_real_world_html_difficulty_taxonomy.md` | 5-tier (E/M/H/X/R) task taxonomy with 3 tasks per tier + mechanical pass criteria + multi-attempt signatures. | Before designing future TuringOS user-sim test matrices. **CAVEAT**: X3 Wordle was predicted <5% one-shot but DeepSeek nailed it; recalibrate before reusing. |
| `B_ATOM_T_DESIGN_AND_RESULTS.md` | Atom-T design (read_prior_rejection_feedback helper), implementation cost (~75 LoC, 0 deps), and the 3-attempt empirical validation with prompt_hash byte-level evidence. | Before any new "tape-relay" feature; this is the canonical end-to-end pattern. |

## Triggers that would reopen this debate

1. **HeuristicFailed scenario validation** — Atom-T handles `HeuristicFailed` in code (parses test_run_cid from rejection.reason, reads TestRunCapsule, surfaces failed-scenario names), and 4 unit tests cover the helper logic. But end-to-end production proof needs a C11 test failure → next-attempt-fed-with-scenario-names → success. Not yet validated.

2. **Web-layer relay surface** — `src/web/generate.rs` MAX_GENERATE_ATTEMPTS=3 loop re-spawns the CLI binary. Atom-T fires inside cmd_generate regardless, so the web layer's retries benefit automatically. But the web client doesn't see the relay signal. Defer.

3. **Cross-session relay** — Atom-T scopes by `session_id`. If a future user wants "this game is broken, generate v2 of it" across separate sessions, that's a different feature (provenance chain across spec_capsule_cids).

4. **Difficulty taxonomy recalibration** — A3 predicted X3 <5% one-shot; reality is DeepSeek-v4-flash often nails it. Future test matrices need genuinely-hard tasks OR forced-failure injection (the canonical pattern that finally worked).

## How to consume this archive on a future session

1. Read this README + the final report `handover/architect-insights/PLAN_V6_OVERNIGHT_TAPE_RELAY_2026-05-22.md` for the binding decision and evidence.
2. If your current question is "does v4 do tape-relay" → answer is YES (post-`d8e0fda4`), see `B_ATOM_T_DESIGN_AND_RESULTS.md` for the empirical proof.
3. If asking "why didn't we just port v5's pattern" → there was no v5 tape-relay pattern (v5 TUI is read-only DevTape projection; never exercised attempt-2-with-feedback either).
4. If asking "what would it take to extend relay to web layer / cross-session / HeuristicFailed-proven" → §triggers above.
5. Only re-run new research agents if the **codebase has shifted** (Atom-T helper signature changed, GenerateRejectionCapsule schema bumped, CAS layout changed). Otherwise the A1/A2/A3 findings remain current.

## Companion artifacts (live on main, not in this archive)

- `src/bin/turingos/cmd_generate.rs` — `read_prior_rejection_feedback()` helper (~75 LoC starting around line 700)
- `tests/tape_relay_feedback_loop.rs` — 4 unit tests covering the helper
- `handover/architect-insights/PLAN_V6_TAPE_RELAY_MATRIX_2026-05-21.md` — Phase B test-matrix design
- `handover/architect-insights/PLAN_V6_OVERNIGHT_TAPE_RELAY_2026-05-22.md` — final report with 3-attempt evidence + reproducible recipe
