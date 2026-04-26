#!/usr/bin/env python3
"""Brainstorm partner mode (NOT audit). Gemini 2.5 Pro generates ideas."""
import json, sys, time, urllib.request
from pathlib import Path

ROOT = Path("/home/zephryj/projects/turingosv4")
V3_ENV = Path("/home/zephryj/projects/turingosv3/.env")

env = {}
for line in V3_ENV.read_text().splitlines():
    if "=" in line and not line.strip().startswith("#"):
        k, v = line.split("=", 1)
        env[k.strip()] = v.strip().strip('"').strip("'")

prompt = """# Brainstorm partner — N agents × PPUT × difficulty

**Role**: think-partner, NOT auditor. No PASS/CHALLENGE/VETO. Generative, divergent, idea-rich. Your job: critique the user's experiment design + propose additions + flag blind spots. Speculation is welcome, just label it as such. The goal is to produce a richer experimental program than what one mind alone would generate.

**Background**: TuringOS v4 is a multi-agent LLM swarm for Lean4 theorem proving (MiniF2F). Each "run" is a single problem attempt where N agents propose tactics + a Lean compiler ground-truths. PPUT = progress / cost / time = `1[Lean accepts] / (total_tokens × wall_time_sec / 1000)`. Constitutional anchor: Proposal (LLM) → ∏p ground-truth (Lean) → Logging → Capability Compilation. Solo researcher, R&D phase, iterative.

**Three research questions the user wants answered**:
- **Q1**: For same-difficulty problems, does increasing N (number of agents) have a PPUT ceiling? Is there a plateau? Decline? Power-law?
- **Q2**: As difficulty increases, does increasing N speed up solving (higher PPUT)? Or is N less helpful on hard problems?
- **Q3**: Like a car (N = throttle): is there a regime where increasing N **sacrifices PPUT** but **improves solve speed (1/wall_time)**? Does N let you trade efficiency for speed? Should N be a user-facing knob?

**User's constraint**: still in R&D, small targeted experiments preferred over big batches. Each test should answer one specific question. Code may change between experiments — designs robust to mid-stream code adjustments are valued.

**Claude's draft experiment design**:

E1 — Same-difficulty PPUT ceiling
- 5 mathd_algebra problems × N ∈ {1, 3, 5, 8} × seed=31415 = 20 runs (~$1, ~45min)
- Plot PPUT vs N per problem + aggregate

E2 — Difficulty × N matrix
- 9 problems (3 easy mathd / 3 medium algebra / 3 hard aime) × N ∈ {1, 3, 8} × seed=31415 = 27 runs (~$2-3, 3-6h)
- Heatmap PPUT vs (difficulty, N); solve_rate vs (difficulty, N)

E3 — Throttle regime (Q3)
- Reuse E2 data; plot solve-time vs N AND PPUT vs N on dual-axis per difficulty bucket
- Look for points where solve-time↓ but PPUT↓

**Key technical details**:
- N=1 is currently `CONDITION=oneshot` (1 LLM call, no tape) — different code path from `n*` swarm. Comparison concern.
- Boltzmann seed only seeds agent-routing RNG; LLM sampling at temp=0.2 is non-deterministic (DeepSeek-v4-flash thinking-off backend).
- max_transactions = 200 hard-coded; hard problems hit it.
- Per-tx wall-clock ~10s avg (LLM call + tool + Lean verify).
- Easy problems solve in tx 1-5; hard problems take 100-200 tx or fail.
- Cost: ~1000 tokens per tx avg.
- PPUT = 1 / (total_tokens × time_sec / 1000) when SOLVED, else 0.

## What I want from you (be generative, not skeptical)

### A. Critique the design + propose alternatives
- Is "5 problems × 5 N values" enough power for E1? What N grid would you choose differently?
- Is the difficulty bucketing (mathd / algebra / aime) the right operationalization? What proxy for difficulty would you use? Is theorem-name lexical structure a usable proxy or a misleading one?
- Single seed vs multi-seed for these small experiments — how to bound noise without inflating cost?
- Any confounders I'm missing (problem-text length, theorem name distribution, library imports, etc.)?

### B. Generate hypotheses I haven't considered
- The user's intuition is "throttle". What OTHER metaphors / regimes might be in play? E.g., diversity-vs-homogeneity, exploration-exploitation, parallel-vs-serial information bottleneck, anti-correlation between agents from temperature ladders?
- Are there NON-monotonic relationships in N → PPUT that would surprise us? (e.g., minimum at N=2 from agent-confusion before benefits from N=3+ kick in)
- Could PPUT be the WRONG metric for some questions? (e.g., for "speed-up regime", solve-time alone matters; for "diversity benefit", payload-uniqueness matters)

### C. Connect to scaling laws / parallel computation theory
- Is there a published functional form for "agents × task difficulty → speedup" we should test against? (Amdahl's law, Brent's theorem, Universal Scalability Law, Gustafson's law, etc.)
- N=1 to N=8 is small range — would you extend up to N=16 or N=32 to find saturation? At what point does API rate-limit or hardware become the constraint?
- What's the role of max_transactions in this picture? Is there a `tx_budget × N` interaction (e.g., when N is high but tx_budget is low, agents don't have enough turns to converge)?

### D. Statistical rigor at small N
- The user is doing N=20-27 run experiments. What's the right inferential framework? Bayesian with informative priors? Frequentist with paired t-tests? Bootstrap CIs?
- Per-problem variance vs across-problem variance — which dominates here? How would you partition?
- Are there any sequential-stopping rules that would let us learn faster (e.g., halt E1 early if PPUT vs N is clearly flat)?

### E. Any other angles
Whatever you'd want to discuss with a research collaborator over coffee. No format requirement. Be specific where useful, speculative where useful.

---

End with **3 prioritized concrete experiments you'd run NEXT** (in addition to or instead of E1/E2/E3). For each: name, hypothesis, sample size, key metric. Brief.
"""

print(f"[gemini brainstorm] prompt size: {len(prompt):,} chars", file=sys.stderr)

url = f"https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-pro:generateContent?key={env['GEMINI_API_KEY']}"
body = json.dumps({
    "contents": [{"parts": [{"text": prompt}]}],
    "generationConfig": {"temperature": 0.7, "maxOutputTokens": 16384},
}).encode()
headers = {"Content-Type": "application/json"}

t0 = time.time()
req = urllib.request.Request(url, data=body, headers=headers, method="POST")
try:
    with urllib.request.urlopen(req, timeout=900) as resp:
        data = json.loads(resp.read())
except Exception as e:
    print(f"[gemini brainstorm] error: {e}", file=sys.stderr)
    sys.exit(1)

elapsed = time.time() - t0
print(f"[gemini brainstorm] API returned in {elapsed:.1f}s", file=sys.stderr)

text = data["candidates"][0]["content"]["parts"][0]["text"]
out = ROOT / "handover/brainstorms/GEMINI_N_AGENTS_BRAINSTORM_2026-04-25.md"
out.parent.mkdir(parents=True, exist_ok=True)
header = (f"# Gemini brainstorm — N agents × PPUT × difficulty\n"
          f"**Date**: 2026-04-25\n"
          f"**Mode**: think-partner (NOT audit), temp=0.7\n"
          f"**Elapsed**: {elapsed:.1f}s\n"
          f"**Prompt size**: {len(prompt):,} chars\n\n---\n\n")
out.write_text(header + text)
print(f"[gemini brainstorm] saved: {out}")
