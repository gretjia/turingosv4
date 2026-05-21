# Plan v6 — Tape-Relay Validation Test Matrix

| Field | Value |
|-------|-------|
| Status | DRAFT — orchestrator-led overnight execution; user asleep |
| Date | 2026-05-21 (evening) → 2026-05-22 (morning) |
| Orchestrator | Claude opus 4.7 |
| Authority | User-delegated 2026-05-21: "你是整个项目的 orchestrator,你来负责派发任务给 agent 完成" |
| Mission | Validate whether TuringOS v4 actually behaves as a Turing machine with tape-relay across attempts — vs being a fancy LLM wrapper. Multiple difficulty tiers. Not just games. |

## §1. Context

The user (Architect) gave Claude opus the orchestrator role for an overnight run with explicit directive:

> 安排 agent 模拟真人，进入 TUI 中进行真题测试 …… 设计的难度一定不是一个 LLM agent 只凭自己的一次输出就可以完成的，我要看到 turingos 内核中，agent 可以在 tape 上接力完成任务，图灵机真的在运转。

Translation: dispatch sim agents through the TUI on real problems, designed so a single LLM call cannot solve them. Goal: see agents relaying via the CAS-anchored tape — the Turing machine actually running.

## §2. Phase A research — brutal finding

Three parallel research agents (A1 historical, A2 current-code audit, A3 task taxonomy) converged on a single uncomfortable fact:

> **TuringOS v4 today writes `parent_attempt_cid` chains in CAS but never reads them back into the LLM prompt. Every retry of `turingos generate` constructs the IDENTICAL prompt (bit-for-bit) and hopes stochastic sampling produces a different result. The tape exists for audit, but the "relay" — the next agent picking up where the prior agent left off via tape state — is missing.**

Citations:
- `src/bin/turingos/cmd_generate.rs:266-289` (write parent_attempt_cid)
- `src/bin/turingos/cmd_generate.rs:242-254` (prompt construction — invariant across retries)
- `handover/architect-insights/TB18_TAPE_NON_EXTERNALIZATION_VETO_2026-05-06.md` (TB-18 era confirmed this gap)
- `handover/post-mortems/ROOT_CAUSE_TB18_DELAY_2026-05-06.md` §7.1 (denominator preflight gate was never implemented)

The minif2f-era smoke tests demonstrated **schema-stable relay** (capsule chains survive ABI bumps; data shape is preserved) but never exercised **semantic relay** (attempt 2's prompt informed by attempt 1's diagnostics). Historical evidence directory `handover/evidence/tb_4_smoke_2026-04-30/` shows n1 (single-attempt) and oneshot configurations only — no attempt-2-with-feedback tape.

**So: if I dispatch user-sim agents now on hard tasks, all I'll prove is that v4 doesn't do tape-relay yet. The user already knows that's possible; what they actually want is to see it work.**

## §3. Decision — build Atom-T before running the matrix

Per user's standing directive ("缺什么你补什么，找到问题修复问题是你的责任"), the orchestrator-correct move is:

1. **Build Atom-T**: minimum architectural change to close the missing READ side of the parent_attempt_cid chain.
2. **Then run the test matrix** comparing pre-Atom-T (baseline = no relay) vs post-Atom-T (relay enabled).

Atom-T design (dispatched in parallel with this doc):

- New helper `read_prior_rejection_feedback(workspace, session_id) -> Option<String>` in `cmd_generate.rs`.
- Walks CAS for the latest `GenerateRejectionCapsule` on this session.
- Extracts `public_error_summary`, `reason`, and (for `HeuristicFailed`) the linked `TestRunCapsule`'s failed-scenario names.
- Prepends a structured "PRIOR ATTEMPT FEEDBACK" block to the LLM user message on attempts after the first.
- First attempt unchanged → backward compatible.
- No new Cargo deps, no schema changes, no Trust Root churn.
- Class 2 additive, ~80-120 LoC.

This is **the smallest honest fix that makes the tape relay**. Anything less and we're shipping a demo, not a Turing machine.

## §4. Test matrix — 5 selected tasks

Picked from A3's 15-task taxonomy. Selection rationale:

| Tier | Task | Why selected |
|------|------|-------------|
| **E** | E1 Countdown Timer | Sanity baseline. If E fails after Atom-T, that's a regression, not a difficulty issue. |
| **M** | M2 Flashcard Quiz | Tests Meta-AI spec-expansion quality; medium-cost tokens. |
| **H** | H1 Snake Game | Common, well-understood test target. Self-collision bug is a classic single-shot failure. |
| **X** | X3 Wordle Clone | Duplicate-letter coloring is the cleanest "single-shot rarely succeeds" test. Algorithmic delta between attempt 1 and attempt 2 is precise. |
| **R** | R1 DeepMind Explainer | Tests non-game intent path (matches user's actual prior input). Validates the wizard's "any intent" promise. |

Mechanical pass/fail criteria are A3's `grep -c` checks. No LLM-judged "is this beautiful". All criteria are deterministic regex/substring checks.

## §5. Execution methodology

For each task, two runs in sequence:

### Round 1 — Baseline (no Atom-T)

- Dispatch sub-agent (sonnet) role-playing the non-expert user.
- Sub-agent runs `turingos` (the TUI binary built **before** Atom-T merges).
- One TUI walk-through: provider/key entry → free-form intent → Meta AI expansion → generate.
- After completion: capture artifacts, capsule chain, test_run_cid result.
- If `overall_pass=true` on first try → mark "E-tier-easy" for this task; tape-relay path untested.
- If `overall_pass=false` → manually re-run `turingos generate --workspace <ws>` to invoke attempt 2.
- Capture: did the second attempt produce different output? Read the new GenerationAttemptCapsule and compare its `prompt_hash` to attempt 1's. **Expected (pre-Atom-T): identical prompt_hash → relay confirmed absent.**

### Round 2 — Atom-T enabled (after Atom-T merges)

- Same sub-agent dispatch with the **post-Atom-T** binary.
- Same task.
- On failure, re-run `turingos generate`.
- **Expected: prompt_hash differs between attempts** because Atom-T injects prior-rejection feedback into the user message.
- Did attempt 2's artifact actually fix what attempt 1 broke?

### Comparison criteria

For each task × mode:
1. Does the prompt hash change between attempts? (binary: yes/no)
2. Does the artifact content differ meaningfully? (diff lines > N)
3. Did the failed test scenario from attempt 1 pass on attempt 2? (per C11 TestRunCapsule)
4. Is the final user-observable outcome better? (mechanical regex check from A3)

## §6. What's in scope vs deferred

### In scope (overnight)

- A1+A2+A3 research (done)
- Atom-T implementation + merge
- 5 tasks × 2 modes = 10 sim runs
- Per-task evidence capture (CAS dump, capsule chain, test_run_cid)
- Final report

### Deferred (next charter)

- Web layer's MAX_GENERATE_ATTEMPTS=3 retry loop also needs to consume Atom-T feedback (currently re-spawns CLI blindly). Atom-T as-implemented works regardless of who's calling cmd_generate because the relay is read inside cmd_generate itself.
- Cross-session tape-relay (one workspace, multiple sessions referencing each other's evidence). Not needed for the user's immediate goal.
- Spec-grill driven mode also doesn't do tape-relay. Spec is single-LLM-call territory anyway.
- TestScenario expansion (currently only EntrypointExists + HtmlParses; per Round 5 finding R5-1, HtmlParses is too lenient). Defer to its own charter.

## §7. What success looks like

**Minimum success**: At least 2 of the 5 tasks demonstrate observable tape relay — i.e. attempt 2's prompt is structurally different from attempt 1's (carries the prior rejection feedback block), AND attempt 2's artifact addresses the specific failure called out.

**Strong success**: Same as minimum + at least 1 X-tier task converges to `overall_pass=true` within 3 attempts, where the single-shot success rate is <5%. This is the closest mechanical evidence that "the Turing machine actually runs."

**Failure**: If Atom-T merges but no task's attempt 2 differs from attempt 1 in observable ways — that means the relay code is buggy. Debug + iterate.

**Honest non-success**: If the LLM ignores the "PRIOR ATTEMPT FEEDBACK" block (i.e. attempt 2 produces the same wrong code despite seeing the feedback), that's a finding about LLM behavior, not TuringOS architecture. Report honestly; don't tune the feedback wording until we see what attempts produce.

## §8. Risk controls

- **Token budget**: ~5 tasks × 2 modes × ~3 generate attempts × ~10K tokens = ~300K DeepSeek tokens ≈ $0.30-0.80. Acceptable.
- **Rate limit**: DeepSeek occasionally returns empty body (R4-1). Atom-T doesn't help with this; just retry per existing R4-1 hint logic.
- **Sim agent drift**: each sub-agent is sonnet in worktree isolation. None have access to other sims' workspaces.
- **Time budget**: User asleep ~22:00 → ~07:00 = 9 hours. Allotted: 1h research (done), 0.5h synthesis, 0.5h Atom-T build, 0.5h merge+verify, 4h test execution + per-test debug, 1h final report. Buffer 1.5h.

## §9. Orchestrator commitments

- I will **NOT** dispatch sims until Atom-T merges. Wasted token budget otherwise.
- I will **NOT** silently fix tests that the LLM gets wrong. Honest reporting wins over feel-good narrative.
- I will **debug live** if Atom-T has integration bugs (e.g. CAS path issue, prompt-injection encoding bug).
- I will **stop early** if 2-3 tests reveal the same architectural gap that's beyond Atom-T's scope — and produce an honest report instead of running the rest blindly.
- I will **NOT touch §3.1 forbidden surfaces**: constitution.md, genesis_payload.toml, src/bottom_white/cas/schema.rs, src/kernel.rs, src/bus.rs, src/state/**.

## §10. Tracking

See live `TaskList` in this session for current state. Top-level tasks:

1. Plan v6 orchestration umbrella (in_progress)
2. A1 — minif2f historical study ✅
3. A2 — v4 kernel audit ✅
4. A3 — task taxonomy ✅
5. B — synthesize Plan v6 matrix (this doc — in_progress)
6. Atom-T — tape-relay feedback build (sub-agent running)
7. C — test matrix execution (blocked on 5+6)
8. D — final overnight report (blocked on 7)

## §11. References

- `handover/research/MULTIPROVIDER_LLM_2026-05-21/` — earlier protocol/provider debate
- `handover/research/TUI_PHASE1_2026-05-21/` — TUI Phase-1 zero-dep decision
- `handover/observations/USERSIM_ROUND{1..5}_*.md` — prior 5-round user-sim study (Claude sub-agents)
- `handover/architect-insights/TB18_TAPE_NON_EXTERNALIZATION_VETO_2026-05-06.md` — historical canonical externalization rule
- `handover/post-mortems/ROOT_CAUSE_TB18_DELAY_2026-05-06.md` §7 — denominator preflight recommendations
