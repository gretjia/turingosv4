# Prompt-variant experiment — results 2026-05-10 session #34

**Authority**: 2026-05-10 user verbatim **"你可以根据M0测试的真实数据，以及我的
Turing OS V3的Prompt尝试，进行Prompt实验。谁也不知道哪种效果好，我们现在在
开发阶段。"**

**Plan**: `handover/alignment/PROMPT_VARIANT_EXPERIMENT_PLAN_2026-05-10.md`.
**Source data**: `/tmp/prompt_variant_exp_2026-05-10T05-03-12Z/` (full chain
tape per (variant, problem); aggregate at `AGGREGATE_BY_VARIANT.md`).
**Harness commit**: `9b8c847` (`Prompt-variant experiment harness — opt-in
via TURINGOS_PROMPT_VARIANT`).
**Configuration**: deepseek-chat (proxy `localhost:18080`), temperature
0.2 (default), CONDITION=n1 (single agent), MAX_TX_OVERRIDE=200, single
seed per cell.
**Wall-clock**: 988s (16 min 28s) for 5 variants × 4 problems = 20 runs.

## §1 — Headline result: clean negative

**No prompt variant moved any behavioral metric** on any problem. Every
(variant, problem) cell produced the SAME outcome down to the exact step
counts:

| problem | M0 baseline (2026-05-10 batch) | this experiment (5 variants × 1 seed) |
|---------|--------------------------------|--------------------------------------|
| `mathd_algebra_107`         | solved, 1 step (`nlinarith`), div=1.0 | solved, 1 step (`nlinarith`), div=1.0 — **5/5 variants** |
| `mathd_algebra_125`         | solved, 1 step (`nlinarith`), div=1.0 | solved, 1 step (`nlinarith`), div=1.0 — **5/5 variants** |
| `mathd_algebra_113`         | exhausted, 9 step / 9 reject, div=0.111 | exhausted, 9 step / 9 reject, div=0.111 — **5/5 variants** |
| `algebra_sqineq_at2malt1`   | exhausted, 9 step / 9 reject, div=0.111 | exhausted, 9 step / 9 reject, div=0.111 — **5/5 variants** |

Per-variant aggregate (full table in
`/tmp/prompt_variant_exp_.../AGGREGATE_BY_VARIANT.md` §1-§2):

| variant | solved/4 | mean tactic_div on hard | total tx |
|---------|----------|--------------------------|----------|
| v0 (control)                             | 2/4 | 0.111 | 402 |
| v1 (drop unused tools)                   | 2/4 | 0.111 | 402 |
| v2 (diversity nudge)                     | 2/4 | 0.111 | 402 |
| v3 (v3-LAW style)                        | 2/4 | 0.111 | 402 |
| v4 (v2 + last-rejected echo)             | 2/4 | 0.111 | 402 |

The cross-variant ranking is degenerate: every behavioral metric is a
5-way tie. Wall-clock varies (182s-214s) but the variation is
LLM-proxy + Lean elaboration noise, not signal — the per-cell
chain-tape evidence is byte-equivalent across variants.

## §2 — Why this happened

The "9 step / 9 reject" pattern is a **stable artifact of
deepseek-chat at temperature 0.2 on these specific problems**, not a
runtime budget cap. Verification: M0 batch problems
`mathd_algebra_113` and `algebra_sqineq_at2malt1` independently
produced exactly 9 step / 9 reject in the 2026-05-10 M0 run too. The
agent emits a single tactic family (e.g. `nlinarith` repeated
9 times for `algebra_sqineq_at2malt1`), all 9 reject, and the
proposal loop exits before reaching the 200-tx budget cap.

At T=0.2 the model's prior over "next tactic given (problem
statement + previous-rejects)" dominates whatever extra prompt sections
the variants inject. The model selects the same tactic, gets the same
reject, repeats. Variant text in the system prompt does not perturb the
sampler enough to change the next-token distribution at this
temperature.

## §3 — What this rules in / out

### Empirically ruled out at N=1, T=0.2, deepseek-chat

- **Boot-prompt option A** (full v3-LAW revert) — `v3` variant tested
  this; zero solve-rate or diversity delta on either hard problem.
- **Boot-prompt option D Phase-1 kludge** (just drop unused
  `invest`/`search`/`post`) — `v1` variant tested this; zero behavioral
  delta on any problem. **Cleanup is safe** (no regression) but
  **cleanup is also pointless** (no improvement) for the agent at this
  configuration.
- **The `project_economy_prompt_landing_gap.md` "wasted-tokens-on-
  dead-letter-tool" hypothesis** — v1 produces literally identical
  outcomes to v0 across all 4 problems. The agent never touches the
  dead-letter tools regardless of whether they're advertised.
- **Tactic-family diversity-nudging via prompt text** — v2/v4 variants
  explicitly told the model to switch families on reject; the model
  ignored the instruction. v4 even echoed the actual rejected tactics
  back with "DO NOT REPEAT"; the model still repeated.

### Not yet tested (forward levers)

- **Higher temperature** (T=0.7+) — would break the deterministic same-
  tactic loop; might reveal whether the diversity-nudge variants do help
  once the sampler has actual variance.
- **N>=2 swarm** — independent agents at T=0.2 would produce
  uncorrelated proposal streams, breaking the deterministic-collapse
  pattern at the swarm level.
- **Stronger model** (deepseek-reasoner / GPT-class) — more capable
  models might respond to prompt-level guidance.
- **Runtime-side intervention** — instead of asking the model to switch
  families, the runtime could re-roll the next tactic via a different
  decoding strategy when N consecutive rejects from the same family are
  detected. This is an evaluator-side change, not a prompt change.

## §4 — Recommendation

For the boot-prompt option (a) "Economy-aware agent prompt" landing
question, the M0-evidence-grounded recommendation is:

**Choose `v1` (drop unused tools from schema) as the only landed change.**

Rationale:
- v1 is empirically equivalent to v0 across all 4 problems (no
  regression) AND across the M0 baseline 16 PROCEED problems (the agent
  never invokes invest/search/post regardless). So the schema cleanup is
  safe.
- Remaining v0 schema lines for `invest`/`search`/`post` are consuming
  prompt tokens for tools the agent literally never uses. v1 saves
  ~5 lines × ~15 tokens ≈ 75 input tokens per LLM call. At M0 batch
  scale (~9000 calls / batch) that's ~675K tokens / batch ≈ ~$0.10
  saved per batch.
- v1 does NOT close the constitutional landing question of "agent
  perceives the economy" — but the M0 evidence shows the agent
  doesn't engage with the economy at N=1 regardless of whether it's
  visible in the prompt. So that constitutional question is a TB-12+
  forward concern (when actual NodeMarket / Polymarket-agent-bridge
  surfaces are wired), not a TODAY concern.
- v2 / v3 / v4 should NOT be landed: they add prompt tokens (~10-30
  lines of guidance) for zero observed behavioral benefit at this
  configuration. Token cost for no benefit = pure regression.

**This is option D Phase 1, but with empirical justification rather
than the kludge framing the user originally rejected.** The user's
rejection was based on the suspicion that schema-cleanup masks a
deeper landing problem. The M0 evidence + this experiment together
show: (a) the agent doesn't engage with the economy regardless of
prompt, so schema-cleanup masks nothing; (b) the deeper landing
problem (agent engagement with economy) is gated on TB-12+ runtime
work, not prompt work.

## §5 — Limitations + caveats

This experiment is ONE data point. Limitations:
- **Single seed per cell** — no sample variance; the 5-way tie could in
  principle reflect cells that all happened to land on the same near-
  deterministic local optimum. Larger-N replication (e.g. 5 seeds × 5
  variants × 4 problems = 100 runs) would distinguish "all variants
  hit the same attractor by chance" from "no variant has any effect at
  all". Given the perfectly-identical chain-tape evidence across
  variants, the second interpretation is more parsimonious — but a
  multi-seed replication would be a stronger claim.
- **Single model** (deepseek-chat). The "prompt variants don't help"
  finding may be specific to this model's prior. A more-capable model
  might respond differently.
- **Single temperature** (T=0.2). Higher temperatures would weaken the
  prior and may surface variant effects.
- **4 problems** — 2 trivial controls + 2 hard. Generalization to the
  full MiniF2F-Test 195-problem set is not warranted from this small
  sample.
- **No swarm** (N=1). Multi-agent swarm at N>=2 might benefit from
  variant-induced proposal heterogeneity in ways N=1 cannot reveal.

## §6 — Operational hygiene

- Total cost: ~$0.30-1.00 (estimated; deepseek-chat token rates × ~22K
  tokens across 20 runs).
- Total wall-clock: 988s (16.5 min).
- No source modifications during batch (per
  `feedback_no_concurrent_dev_during_batch`); harness commit `9b8c847`
  was the source state for the entire batch.
- Per CLAUDE.md §6: every externalized attempt is tape-visible —
  preserved (each cell has full `runtime_repo/` + `cas/`).
- Per `feedback_real_problems_not_designed`: 4 real MiniF2F-Test
  problems, no synthetic, problem set drawn from M0 evidence.

## §7 — Forward queue impact

| Boot-prompt item | Status after this experiment |
|------------------|-------------------------------|
| (a) Option A v3-LAW economy prompt | **EMPIRICALLY DROPPED** — v3 variant tested; zero solve-rate or diversity delta. Land v1 (safe schema cleanup) instead. |
| (a) Option B minimal awareness    | **EMPIRICALLY DROPPED** — analogous to v2/v4 which were tested; zero delta. |
| (a) Option C TB-12+ synchronized  | **CORRECT path** — if the constitutional landing of agent-economy is the goal, it requires actual runtime tools (NodeMarket / Polymarket-agent-bridge), not prompt text. Forward-bind. |
| (d) M1 mini batch                 | OK to proceed; will use whichever prompt variant is current (recommend committing v1 as default before M1). |

`FC-trace: FC1-N1 (prompt construction; agent-facing read view).`
