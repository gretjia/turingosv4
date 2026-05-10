# Prompt-variant experiment plan — 2026-05-10 session #34

**Authority**: 2026-05-10 user verbatim **"你可以根据M0测试的真实数据，以及我的
Turing OS V3的Prompt尝试，进行Prompt实验。谁也不知道哪种效果好，我们现在在
开发阶段。"** — explicit authorization to run prompt experiments using M0
real-data baseline + v3 prompt as reference, in development mode.

## §1 — Reframe from boot-prompt option (a)

The boot prompt for session #34 framed the next-step choice as:
> Option A — full revert to v3 explicit-rule style (LAW 1 / 2 / 3 about
>            invest/short economy)
> Option B — minimal awareness injection (state-only "Your Economic Position")
> Option C — TB-12+ synchronized landing (defer until NodeMarket wired)
> Option D — staged (rejected by user)

**M0 empirical evidence reframes the question.** Across all 16 PROCEED
problems in the 2026-05-10 M0 batch, the agent NEVER invoked `invest` /
`search` / `post`. Tool distribution was exclusively `step` + `omega_wtool`
+ `step_reject` + `step_partial_ok` + `parse_fail`. The "wasted tokens on
dead-letter tool" hypothesis from `project_economy_prompt_landing_gap.md`
is empirically false at N=1 with deepseek-chat.

The actual failure mode in the 7 EXHAUSTED problems (out of 16 PROCEED):
**`tactic_diversity` collapses to 0.11–0.20** (≈1 unique tactic per 9
attempts). The agent retries near-identical failed Lean tactics until
budget exhaustion. This is a **tactic-search-strategy** issue, not an
economic-engagement one.

So the experiment will TEST option-A's economy-rules hypothesis (per user
"V3 prompt 尝试") AND the empirically-actionable diversity hypothesis,
rather than committing to a single landing without evidence.

## §2 — Variants (5)

Each variant is a function `build_agent_prompt_variant(base_args, variant_id)
-> String`. V0 is the baseline (current v4 behavior). V1-V4 inject extra
content. Selected at runtime via env var `TURINGOS_PROMPT_VARIANT=v0|v1|v2|v3|v4`
(default v0 = no change).

### V0 — control
Current v4 prompt. Lists `step` + `search` + `invest` + `post` in tools
schema. State-only philosophy ("V3L-39: LLMs follow incentives, not
explanations"). No tactic guidance, no LAWS.

### V1 — tool clean (drop unused tools)
V0 minus `invest`, `search`, `post` from the schema list (since M0 shows
these are never called). Tests: "does the noise from unused tools confuse
or distract the agent's `step` tactic selection?"
**Hypothesis**: no-op or marginal; if it DOES help, it's because the
schema was crowding the LLM's attention.

### V2 — diversity nudge
V0 + a new prompt section after Tools:

```
=== Tactic Search Guidance ===
If your previous tactic was rejected, try a STRUCTURALLY DIFFERENT
tactic family. Do not repeat near-identical failed tactics — the budget
shrinks on every submission whether accepted or rejected. Examples of
structurally distinct families:
  arithmetic decision: omega / linarith / nlinarith / polyrith
  algebraic rewrite:   ring / field_simp / norm_num / push_cast
  simplification:      simp / aesop / decide
  decomposition:       have ... := by ...; cases ...; induction ...
```

**Hypothesis**: directly addresses the observed failure mode; should
improve solve rate or extend the chain on the 2 collapsed-diversity
problems.

### V3 — v3-LAW style (ported)
V0 + a v3-style explicit-LAWS block adapted for v4 reality:

```
=== Operating Laws ===
LAW 1: Each `step` submission consumes 1 of 200 budgeted attempts,
       whether accepted or rejected.
LAW 2: A REJECTED step does not advance the proof; it only burns budget.
LAW 3: If two consecutive steps were rejected, switch to a structurally
       different tactic family (do not repeat the same approach).

=== What makes a step worth submitting ===
  ✓ Logically follows from the proof state in === Current Chain ===
  ✓ Uses a tactic family appropriate for the goal type
  ✓ Is atomic — one tactic, not a chain of `<;>` composites
  ✗ Repeats a tactic that already rejected
  ✗ Hand-waving (`sorry`, `admit`, `???`) is forbidden by the oracle
```

**Hypothesis**: combines V2's diversity nudge with v3-style criteria;
explicit framing might or might not move signal.

### V4 — V2 + last-rejected-set echo
V2 + a dynamic section showing the LAST 3 REJECTED TACTICS verbatim
(extracted from `recent_errors`, which the prompt-builder already
receives).

```
=== Last Rejected Tactics (DO NOT REPEAT) ===
- <tactic 1>  (reason: <classifier>)
- <tactic 2>  (reason: <classifier>)
- <tactic 3>  (reason: <classifier>)
```

**Hypothesis**: closes the recency gap — gives the model concrete
don't-do data instead of generic "try different".
**Implementation note**: the existing `recent_errors` slice may already
contain enough; if not, fall back to "no recent rejects to show".

## §3 — Problem set (4)

| Slot | Problem | M0 baseline | Why |
|------|---------|-------------|-----|
| P01 | `mathd_algebra_107`     | solved (1 step, `nlinarith`) | sanity: variants must not regress trivial-easy |
| P02 | `mathd_algebra_125`     | solved (1 step, `nlinarith`) | sanity: 2nd trivial control |
| P03 | `mathd_algebra_113`     | exhausted (9/9 reject; diversity 0.11) | hard: classic diversity collapse |
| P04 | `algebra_sqineq_at2malt1` | exhausted (9/9 reject; diversity 0.11) | hard: classic diversity collapse |

5 variants × 4 problems × 1 seed = **20 runs**.

## §4 — Metrics

Primary:
- **`solved`** (per-problem boolean) — the constitutional truth metric.
- **`tactic_diversity`** (per-problem float) — measures whether the
  diversity-targeted variants V2/V3/V4 actually move the failure mode.

Secondary:
- **`tx_count` / `step_reject` count** — efficiency.
- **`golden_path_token_count`** — when solved, how concise.
- **`time_secs`** — wall-clock per problem.
- **invariant: `invest`/`search`/`post` tool calls** — must remain 0
  across all variants (sanity that no variant accidentally re-enables a
  dead-letter tool).

Aggregate per variant:
- solved-rate (out of 4)
- mean tactic_diversity on the 2 exhausted problems
- total wall-clock + total token spend

## §5 — Cost + wall-clock budget

Per M0 real-load baseline:
- solved-quick: ≈ 10-12s, ≈ 450-600 tokens
- exhausted: ≈ 80-280s, ≈ 4400-7400 tokens

Estimate:
- 5 variants × 2 solved problems × ~12s ≈ 120s
- 5 variants × 2 exhausted problems × ~150s avg ≈ 1500s
- **Total wall-clock: ~30 min**
- Token spend: at deepseek-chat M0 rate ($1-3 / 20 problems), the per-run
  cost is ~$0.05-0.15 for exhausted, ~$0.01 for solved. **Estimated
  experiment total: $0.50-2.00.**

If proxy / API errors out, cap at $5 hard budget and stop.

## §6 — Output

Per (variant, problem):
- `evaluator.stdout` (PPUT_RESULT line)
- `evaluator.stderr`
- `runtime_repo/` + `cas/` (chain tape per CLAUDE.md §6 invariant)

Aggregate:
- `AGGREGATE_BY_VARIANT.md` — metrics table + per-variant ranking on each
  metric + recommendation.

## §7 — Decision rule

After the experiment:

- If V2 / V3 / V4 lifts the solve rate on the 2 exhausted problems by
  ANY non-zero amount AND does not regress the 2 sanity-controls →
  recommend that variant for landing.
- If NO variant moves the needle → the diversity collapse is a
  model-strength issue, not a prompt issue. Report negative result;
  defer prompt-landing decision.
- If V1 (tool clean) ties V0 (control) → recommend V1 anyway (cleaner
  prompt without behavior cost).

## §8 — Constitutional cleanliness

- Class 2 — production wire-up of an experimental switch (prompt-builder
  takes a new optional arg / reads new env var).
- `src/sdk/prompt.rs` is NOT on Trust Root (verified) — no rehash needed.
- Per CLAUDE.md §6: every externalized attempt is tape-visible — preserved
  (the experiment uses the same `TURINGOS_CHAINTAPE_PATH` mechanism).
- Per `feedback_real_problems_not_designed`: the 4 problems are real
  MiniF2F-Test problems, not synthesized. Seed values come from the M0
  evidence, not cherry-picked.
- No `f64` in money path — preserved (prompt change doesn't touch money).
- No sequencer admission / typed-tx schema / canonical signing payload
  changes — Class 4 STEP_B does not apply.

`FC-trace: FC1-N1 (prompt construction; agent-facing read view).`
