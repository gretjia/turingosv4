#!/usr/bin/env bash
# NOT an audit — brainstorm partner mode. Codex generates ideas, doesn't VETO.
# Output: handover/brainstorms/CODEX_N_AGENTS_BRAINSTORM_2026-04-25.md
set -euo pipefail

ROOT="/home/zephryj/projects/turingosv4"
mkdir -p "$ROOT/handover/brainstorms"
OUT="${ROOT}/handover/brainstorms/CODEX_N_AGENTS_BRAINSTORM_2026-04-25.md"
TMP_PROMPT="$(mktemp /tmp/codex_brainstorm.XXXXXX.md)"
trap 'rm -f "$TMP_PROMPT"' EXIT

cat > "$TMP_PROMPT" <<'BRIEF_EOF'
# Brainstorm partner — N agents × PPUT × difficulty

**Role**: think-partner, NOT auditor. No PASS/CHALLENGE/VETO. Generative, divergent, idea-rich. Your job: critique the user's experiment design + propose additions + flag blind spots. Speculation is welcome, just label it as such.

**Background**: TuringOS v4 is a multi-agent LLM swarm for Lean4 theorem proving (MiniF2F). Each "run" is a single problem attempt where N agents propose tactics + a Lean compiler ground-truths. PPUT = progress / cost / time = `1[Lean accepts] / (total_tokens × wall_time_sec / 1000)`. Constitutional anchor: Proposal (LLM) → ∏p ground-truth (Lean) → Logging → Capability Compilation. Solo researcher, R&D phase, iterative.

**Three research questions the user wants answered**:
- **Q1**: For same-difficulty problems, does increasing N (number of agents) have a PPUT ceiling? Is there a plateau? Decline? Power-law?
- **Q2**: As difficulty increases, does increasing N speed up solving (higher PPUT)? Or is N less helpful on hard problems?
- **Q3**: Like a car (N = throttle): is there a regime where increasing N **sacrifices PPUT** but **improves solve speed (1/wall_time)**? I.e., does N let you trade efficiency for speed? Does this mean N should be a user-facing knob?

**User's constraint**: still in R&D, small targeted experiments preferred over big batches. Each test should answer one specific question. Code may change between experiments — designs robust to mid-stream code adjustments are valued.

**My (Claude's) draft experiment design**:

E1 — Same-difficulty PPUT ceiling
- 5 mathd_algebra problems × N ∈ {1, 3, 5, 8} × seed=31415 = 20 runs (~$1, ~45min)
- Plot PPUT vs N per problem + aggregate

E2 — Difficulty × N matrix
- 9 problems (3 easy mathd / 3 medium algebra / 3 hard aime) × N ∈ {1, 3, 8} × seed=31415 = 27 runs (~$2-3, 3-6h)
- Heatmap PPUT vs (difficulty, N); solve_rate vs (difficulty, N)

E3 — Throttle regime (Q3)
- Reuse E2 data; plot solve-time vs N AND PPUT vs N on dual-axis per difficulty bucket
- Look for points where solve-time↓ but PPUT↓ (the throttle regime)

**Key technical details**:
- N=1 is currently `CONDITION=oneshot` (1 LLM call, no tape) — different code path from `n*` swarm. Comparison concern.
- Boltzmann seed [BOLTZMANN_SEED env] only seeds agent-routing RNG; LLM sampling at temp=0.2 is non-deterministic.
- max_transactions = 200 hard-coded; hard problems hit it.
- Per-tx wall-clock ~10s avg (LLM call + tool + Lean verify).
- Easy problems solve in tx 1-5; hard problems take 100-200 tx or fail.
- Cost: ~1000 tokens per tx avg (per agent prompt + completion).
- PPUT = 1 / (total_tokens × time_sec / 1000) when SOLVED, else 0.

## What I want from you (be generative, not skeptical)

### A. Critique the design + propose alternatives
- Is "5 problems × 5 N values" enough power for E1? What N grid would you choose differently?
- Is the difficulty bucketing (mathd / algebra / aime) the right operationalization? What proxy for difficulty would you use?
- Single seed vs multi-seed for these small experiments — how to bound noise without inflating cost?
- Any confounders I'm missing (problem-text length, theorem name distribution, library imports, etc.)?

### B. Generate hypotheses I haven't considered
- The user's intuition is "throttle". What OTHER metaphors/regimes might be in play? E.g., diversity-vs-homogeneity, exploration-exploitation, parallel-vs-serial information bottleneck?
- Are there NON-monotonic relationships in N → PPUT that would surprise us? (e.g., minimum at N=2 from agent-confusion before benefits from N=3+ kick in)
- Could PPUT be the WRONG metric for some questions? (e.g., for "speed-up regime", solve-time alone matters)

### C. Connect to scaling laws / parallel computation theory
- Is there a published functional form for "agents × task difficulty → speedup" we should test against? (Amdahl's law, Brent's theorem, Universal Scalability Law, etc.)
- N=1 to N=8 is small range — would you suggest extending up to N=16 or N=32 to find the saturation point?
- What's the role of max_transactions in this picture? Is there a `tx_budget × N` interaction?

### D. Constitutional & engineering considerations
- The user wants per-line constitutional alignment (DO-178C-style) on the swarm hot loop. What runtime tracing / coverage tooling would you use to know which lines fire on a real run?
- Are there code paths in the swarm loop that should be considered "infrastructure not capability" and therefore exempt from the PPUT calculation? (e.g., tape-snapshot serialization)

### E. Any other angles
Whatever you'd want to discuss with a research collaborator over coffee. No format requirement. Be specific where useful, speculative where useful.

---

End with **3 prioritized concrete experiments you'd run NEXT** (in addition to or instead of E1/E2/E3). For each: name, hypothesis, sample size, key metric. Brief.

BRIEF_EOF

prompt_size=$(wc -c < "$TMP_PROMPT")
echo "[codex brainstorm] prompt size: ${prompt_size} chars" >&2

t0=$(date +%s)
{
  printf '# Codex brainstorm — N agents × PPUT × difficulty\n'
  printf '**Date**: 2026-04-25\n'
  printf '**Mode**: think-partner (NOT audit)\n'
  printf '**Prompt size**: %s chars\n\n---\n\n' "$prompt_size"
  codex exec --skip-git-repo-check < "$TMP_PROMPT" 2>&1
} > "$OUT"
t1=$(date +%s)
elapsed=$((t1 - t0))
echo "[codex brainstorm] done in ${elapsed}s, saved: $OUT" >&2
