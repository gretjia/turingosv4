# No-PROVEN Pre-Claim Checklist (`/no-proven-checklist`)

**Invoke this BEFORE emitting any `PROVEN` / `DEFINITIVE` / `FINAL` / `causal` /
`isolated lever` / `X > Y` efficiency headline** — in a report, a commit
message, `LATEST.md`, a verdict, or a user-facing summary.

This is a mechanism, not advice. It exists because the 2026-06-01 forensic
retrospective (`handover/reports/SESSION_FORENSIC_RETROSPECTIVE_2026-06-01.md`)
found that this campaign's most-cited positives — "combination economy PROVEN
3.81 > 3.00 > 1.50", "capital-at-risk is the ISOLATED causal lever 10/10", "the
routing A/B crossover reproduced on real DeepSeek+Lean" — were built on rigged
baselines, synthetic data presented as real, mislabeled "price" code, and
argmax-collapse routing. The price-causal verdict flip-flopped
NEGATIVE→NEGATIVE→POSITIVE three times in ~2h, each declared DEFINITIVE on
un-replicated single-day code.

The gate is conjunctive: a headline ships only if **every** box is checked. A
single missing box ⇒ the legal output is a **scoped, non-causal** statement
(e.g. "consistent with", "on N seeds of single-day code", "Verdict B:
Sybil-resistance/governance, NOT single-shot causal efficiency"), never PROVEN.

## The six gates (copy-paste; answer each with a file:line or command output)

- [ ] **G1 — RECOMPUTE-FROM-TAPE (#1).** The load-bearing number is
      reconstructed from the frozen tape LINES ALONE (`derive_* == manifest`,
      integer/byte-equal) — NOT merely byte-chain-intact. Paste the
      `verify_market_tape` `replay_clean=true` line *with* the
      `derived` vs `manifest` block, or the equivalent recompute.
      - The arm emits a `GenesisPin`-first tape and passes through
        `verify_market_tape`. An arm that emits **no GenesisPin** and never
        recomputes (e.g. skillsweep, reputation Δ numbers) has **no headline
        warrant**, regardless of any "replay-green" footer.
      - "replay-green / matrix_drift 3/3" = **anti-tamper + constitution-untouched
        ONLY**. It is NOT a correctness warrant. Do not write footers that claim
        otherwise. Enforced by `tests/constitution_headline_recompute_from_tape.rs`.

- [ ] **G2 — REAL MODEL + REAL VERIFIER IN THE DECISION LOOP (#2).** The
      selection/decision loop contains a real LLM call AND a real verifier
      (Lean/judge) call. A synthetic skill axis (`est = skill*truth +
      (1-skill)*noise`, a fixed accuracy ceiling, a Monte-Carlo noise model) is
      **not** a "real-data / 真题" result, even if real models are called
      elsewhere in the binary. Name the file:line of the real model call and the
      real verifier call **on the path that produces the number**.

- [ ] **G3 — FAIR EQUAL-BUDGET BASELINE (#3).** The baseline `Y` in any `X > Y`
      claim gets the **same signal**, the **same compute/budget**, and is **not**
      constructed to lose. Reject if the baseline is: denied a signal `X` uses
      (e.g. `elim_global` denied `conf[a][fi]`); force-suicided (terminal
      elimination + strict specialists); a definitional floor (`single_spec` =
      1-of-4 families); or a silent degenerate fallback (coordinator → index
      order). State the baseline's signal/budget parity explicitly.

- [ ] **G4 — ≥ N SEEDS + PAIRED STATS, REPLICATED (#5).** ≥ N seeds (preregister
      N; default ≥ 12 for a within-seed paired design when each arm re-runs a
      stochastic shared step), paired Wilcoxon + Holm where
      applicable, and the verdict is **stable across a re-run on a later day** —
      not a single-day flip-flop. A verdict that reversed under a confound/fix in
      the prior 24h is NOT yet PROVEN.

- [ ] **G5 — CLEAN-CONTEXT AUDIT *AFTER* DATA LANDS (#6).** A fresh
      clean-context auditor (no implementation transcript, per AGENTS.md §9) ran
      **after** the real numbers landed — never against `[FILL ON COMPLETION]`
      tables — and its verdict is persisted as an **independent artifact under
      `handover/audits/`** that references the run **manifest SHA(s)**. An audit
      PROCEED captured as **prose inside the result report** does not count.

- [ ] **G6 — NO LITERAL "PASSING" CONDITIONS (#7).** No box in the success
      criterion is asserted against a **compile-time literal** (`c9_shielding =
      true`, `roles = ["a","b","c","d"]`, a fixed 4-node DAG literal). Every
      pass-condition must be a value **derived from the run** that *can* differ
      from the asserted value. "A test/condition that cannot fail is
      documentation, not a gate" (AGENTS.md §7).

## Mechanisms this checklist is bound to (not prose)

- `tests/constitution_headline_recompute_from_tape.rs` — G1: recompute reproduces
  an honest manifest, **catches a lying manifest while the byte chain stays
  green** (replay-green ≠ correctness), and `derive_*` is a function of the tape.
- `tests/constitution_router_name_matches_mechanism.rs` — name-lie guard (#4): the
  softmax router must DISTRIBUTE (argmax collapses), and a "price routing" claim
  must carry a real `price` identifier in code. (A name-lie is the most common
  way G2/G3 are silently violated.)
- `verify_market_tape` (`src/bin/verify_market_tape.rs`) — the recompute the
  G1 evidence command runs.
- AGENTS.md §17 — the contract entry that makes this checklist load-bearing
  across every agent runtime.

## Legal outputs when a box is unchecked

- Missing G1 → "byte-chain-intact (anti-tamper) only; headline NOT
  recomputed" — no Δ headline.
- Missing G2 → "synthetic / model-of-a-model result" — never "real / 真题".
- Missing G3 → report the negative or the unfair-baseline caveat; no `X > Y`
  causal/efficiency claim.
- Missing G4 → "single-day, N seeds, not replicated" — no PROVEN/DEFINITIVE.
- Missing G5 → "self-asserted; no post-data independent witness" — blocks ship.
- Missing G6 → "contains a literal pass-condition; not a falsifiable gate".

The genuine results from this campaign survive every box (Stage-2
JUST_SAMPLING; the fair-ablation NEGATIVES; Sybil-resistance / governance =
**Verdict B**). State those at full strength. Down-scope the rest.
