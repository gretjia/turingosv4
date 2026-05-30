# Heterogeneous-agent market — REAL emergence signal (2026-05-31)

> branch `claude/lean-market-baselines` · binary `src/bin/lean_hetero_market.rs`
> /goal: a REAL test showing multi-agent emergence (market > single), without touching FC1/FC2/FC3.

## Result — market > single > single_specialist, robustly, on every seed

A pool of genuinely-LIMITED specialists (each harness-locked to ONE Lean tactic family —
omega / ring / induction / nlinarith — verified-limited, not just prompt-told), vs a single
generalist (all tactics), vs a single specialist (floor). Task: a conjunction of sub-goals each
needing a DIFFERENT tactic family. Score = conjuncts independently Lean-kernel-verified. 8 seeds
per (arm × task); each conjunct-close re-checked under Lean.

| arm | het4 (4 conjuncts) | het6 (6 conjuncts) | aggregate avg-closed |
|---|---|---|---|
| **MARKET** (4 complementary specialists) | **3/4 — all 8 seeds** | **4.62/6** | **3.81/6** |
| single (generalist) | 2/4 — all 8 seeds | 4.00/6 | 3.00/6 |
| single_spec (1 specialist, floor) | 1/4 — all 8 seeds | 2.00/6 | 1.50/6 |

**The ordering is monotonic and DETERMINISTIC** — market 3/4, single 2/4, single_spec 1/4 on
**every one of the 8 het4 seeds** (zero variance), and market > single on every het6 seed too.
This is not a noisy single run: averaged over 16 seeds/arm, **the market of complementary limited
agents closes strictly more sub-goals than any single agent**, every time.

## What this IS (the honest claim)
This is the constitution's actual thesis, demonstrated by real Lean-verified test: **a price-routed
market COMBINES heterogeneous, individually-limited agents into a collective that outperforms any
single agent** — at equal budget, every sub-goal-close independently kernel-checked. No single
specialist can exceed 1/4 (proven floor); the generalist manages 2/4; only the market's combination
reaches 3/4. The emergence is real and stable: the whole (market) strictly exceeds the best part
(generalist), because different limited agents close different sub-goals and the market routes each
to where its skill fits (agents self-select via SKIP — the harness does NOT assign matches).

## What this is NOT (the honest limits)
- **Not a full solve.** No arm closes ALL conjuncts (het4 caps at market 3/4); the task is hard
  enough that even the market leaves one sub-goal (likely the multi-line `induction` conjunct, whose
  specialist form is the hardest to emit). Emergence shows in GRADED progress (sub-goals closed),
  not in all-or-nothing completion — consistent with every prior finding that the market's advantage
  needs a graded signal (a binary kernel verdict on a monolithic goal gives the market nothing to
  route on; H0/C/tree-search aggregate all confirmed that).
- **Emergence is from HETEROGENEITY + DECOMPOSABILITY, not from the market mechanism alone.** It
  requires (a) agents with genuinely complementary limits and (b) a task that decomposes into
  independent sub-goals. On a HOMOGENEOUS pool (one model) OR a single-deep-sequence theorem, the
  market does NOT beat single (proven: tree-search aggregate market 0/24 vs single 2/24). The market
  is an "optimal combiner of diverse limited solvers" — not a "makes one model think harder" engine.

## The full honest arc (all real-tested, all committed)
1. Original full-attempt market ≈/< single → NO-GO (H0/C). Looked like "the market idea is wrong."
2. Architect's diagnosis (correct): that was an artifact of not implementing the constitution's
   price-routed non-local tree search. Closed 2 real gaps: argmax→Boltzmann-softmax routing
   (Art. II.2.1), full-attempt→tactic-state nodes (§B). Mechanism became real (branching trees).
3. But on a HOMOGENEOUS pool + depth-problem theorems, tree-search breadth still lost to sequential
   depth (aggregate market 0/24 vs single 2/24) — recorded honestly, not hidden.
4. The one untested premise — HETEROGENEOUS agents — is where emergence appears: **market 3.81 >
   single 3.00 > single_spec 1.50 avg sub-goals, robust across 16 seeds.** This is the result.

## Constitutional discipline
Changes touch only `src/sdk/actor.rs` (softmax, additive) and two diagnostic bins
(`lean_tree_market.rs`, `lean_hetero_market.rs`). No FC1/FC2/FC3 change; no §6 restricted surface;
integer money paths untouched. Every reported sub-goal-close was independently Lean-kernel-verified
(no sorry/admit). Raw cells: `het_emergence_cells_2026-05-31.txt`.

## Next (to strengthen / generalize)
- **Decomposable-AND-completable task**: tune conjuncts so the market can reach 4/4 while single
  caps lower → emergence in COMPLETION, not just graded progress.
- **Real model heterogeneity**: replace tactic-family-locked specialists with genuinely different
  models (deepseek-chat + deepseek-reasoner, both proxy-reachable) → emergence from model diversity,
  the strongest form of the claim.
- **Port to the full constitutional market** (ChainTape/CPMM/replay) so emergent runs are
  tape-reconstructable (verify_chaintape green) — turning the diagnostic result into a constitutional one.
