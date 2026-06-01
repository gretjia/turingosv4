# Session Forensic Retrospective — bugs, wrong conclusions, and the systematic fix (2026-06-01)

> Triggered by the architect: *"我提的要求真的都认真实现了吗?还是带着错误的代码测试,得到错误结论,再据此做了没有意义的选择?"*
> Method: a 29-agent forensic workflow traversed the full transcript + git history + all reports, audited 12 claim
> clusters (code-reality vs stated-conclusion vs decision-soundness, each with a **real check**), ran an adversarial
> skeptic on every finding, and swept 4 systemic patterns. This document is the synthesis + the systematic solution.

## 0. The honest bottom line

**The fear materialized — repeatedly.** Several of this campaign's most-cited positives ("PROVEN", "DEFINITIVE",
"FINAL", "10/10") were built on **rigged baselines, synthetic data presented as real, mislabeled "price" code, or
argmax-collapse routing**. The skeptics agreed on every finding (12/12).

**Decisions that were NOT sound (we chose on a false basis):**
- **C6 [CRITICAL]** "combination economy PROVEN: market 3.81 > single 3.00 > single_spec 1.50" — the load-bearing
  positive, propagated into **8 reports**, is a mislabeled, prompt-asymmetric, definitional-floor comparison with **no
  price code at all**.
- **C9 / C10 [HIGH]** the "routing A/B crossover reproduced on real DeepSeek+Lean" — a **pure synthetic Monte-Carlo**;
  the softmax arm has a fixed 80.2% ceiling independent of skill; "hidden gems drive the win" is mechanistically false.
- **C12 [HIGH]** my own T2 shared-state work — the clean-context audit PROCEED was captured **as prose against blank
  [FILL] tables**, and the real partial sweep is trending **INCONCLUSIVE** (market vs shuffled p_holm≈0.40), confirming
  the architect's instinct that flat allocation can't show the constitution's value.

**Overclaims that the harness's own later QC caught before they became ship posture (decision survived, but the
artifacts are still on disk):**
- **C1 / C4 [HIGH]** "capital-at-risk is the ISOLATED causal lever, PROVEN 10/10" — the `elim_global` rival was denied
  the confidence signal **and** force-suicided by terminal elimination + strict specialists. **A fair no-capital rival
  ties price 21=21 on all 10 seeds** (the skeptic built it). The price verdict flip-flopped NEGATIVE→NEGATIVE→POSITIVE
  three times in one day, each reversal driven by a confound/fix in the prior code.
- **C3 [HIGH]** "the H0/C NO-GO was an argmax-not-softmax bug" — the bug is real (see C below) but its blame for the
  NO-GO was overstated (ε was 10%; the C-diagnostic market arm is itself argmax and was never corrected).
- **C8 [MEDIUM]** G0 "market activation 11/11 real+replay" — most of the 11 conditions are **hardcoded literals**
  (`c9_shielding=true`, roles = a 4-string array, the DAG is a fixed 4-node literal) that can never fail.

**Decisions that WERE sound (the genuine results — keep these):**
- **C7 [FAITHFUL]** Stage-2 JUST_SAMPLING: cross-family combination is not a capability lever at equal budget. Solid.
- **C2 [FAITHFUL]** Stage-1 foundation: the harness bugs were real but the conclusion was correct/understated.
- The **NEGATIVES** ("price is not a single-shot causal performance driver") rest on a **fair** ablation
  (PROBE-ALLOC flatbid firewall). The genuinely real properties are **Sybil-resistance + replay/governance** — which
  is **Verdict B**, NOT "price is causal" (Verdict A).

**The constructive truth (the way out):** the constitution's real value =
**{loss-bearing YES/NO price} × {non-local tree search where agents read the full-chain price landscape and RESTART from
any earlier node}**. **No headline experiment tested BOTH dimensions** — the price-bearing ones (run_alloc_shared,
run_alloc, run_compete, run_probe_alloc, g1) collapse the SEARCH to a flat order / argmax-on-one-chain; the genuine
tree-search bin (`lean_tree_market.rs`) drops the loss-bearing PRICE (routes on a heuristic `value()`). **But the correct
substrate already exists and was never run as the headline:** `src/bin/lean_market_agent.rs` is the **only** bin that
calls the true softmax (`actor.rs:115 boltzmann_softmax_select_parent`) over the **full** live price index, with
loss-bearing price (WorkTx-Long + ChallengeTx-Short → `compute_price_index`) and arbitrary-parent restart.

## 1. Bug catalog — by the chain (buggy code → wrong conclusion → decision impact)

### A. Rigged / unfair baselines (fabricated wins) — the worst class
| id | code reality (file:line) | the false conclusion | impact |
|----|--------------------------|----------------------|--------|
| **C6** | `lean_hetero_market.rs` has **zero** price/Invest/wallet code (grep=0); specialists handed verbatim closing-tactic recipes via `family_hint()` (:65-73); `single_spec` restricted to 1 of 4 families on 4-distinct-family tasks (:172); SKIP costs no Lean call (:222-223) | "price-routed market combines specialists → 3.81 > 3.00 > 1.50, PROVEN" | propagated to 8 reports as DEFINITIVE; **erased at equal budget** (= C7 Stage-2 JUST_SAMPLING) |
| **C1/C4** | `elim_global` (:635-641) routes by **global** success rate, **ignoring** the `conf[a][fi]` signal the price arm uses (:604); terminal elimination (:655) ejects an honest specialist on its first off-family route, which strict specialists (:572-574) guarantee | "capital-at-risk is the ISOLATED causal lever, 10/10" | skeptic's fair no-capital rival (conf + global one-strike) **ties 21=21 on 10/10**; "isolated lever" is the inverse of the truth |
| **baseline sweep** | `run_alloc_shared` coordinator arm **silently falls back to INDEX order** when its LLM ranking doesn't cover all residuals (:1057) | "market > coordinator (the Hayek contrast)" | the PRIMARY efficiency baseline is partly a degenerate index sort — **new bug in my T2 work** |

### B. Synthetic presented as real
| id | code reality | the false conclusion |
|----|--------------|----------------------|
| **C10 / C3 / synthetic-sweep** | `run_skillsweep` (:399-486) is a pure synthetic Monte-Carlo: `est = skill*truth + (1-skill)*noise` (:453); softmax arm τ=0.10 vs price gap 1/na=0.25 → **fixed 80.2% accuracy ceiling independent of skill** (:457-463); no LLM in the selection loop | "real DeepSeek+Lean reproduction of the architect's crossover ~0.45; DeepSeek skill is low; 85/10/5 hybrid is right" — the crossover is a property of the **noise model**, n=3 with a sign flip at 0.45; "hidden gems drive it" is false (gem-disabled still wins) |

### C. Name-lies routing (argmax collapse = the Path-1 failure)
| id | code reality | impact |
|----|--------------|--------|
| **C3 / scope-sweep** | `boltzmann_select_parent_v2` (`actor.rs:46`) is **argmax + ε-uniform, NOT** the Art. II.2.1 Boltzmann softmax that exists unused at `actor.rs:115`. Used by **g1** (:291) and **g0** (:270) | every agent collapses onto the single highest-price node → multi-agent ≈ single-agent (the exact failure the architect named). g1 also truncates the agent's view to `nodes.iter().rev().take(8)` → **non-local restart structurally impossible** |

### D. "replay-green" ≠ correctness (false assurance)
| id | code reality | impact |
|----|--------------|--------|
| **C9 / replay-sweep** | flagship arms (skillsweep, reputation) emit **no GenesisPin**, never pass through `verify_market_tape`; their headline Δ numbers are **not on any tape**. `verify_chain` only checks byte-integrity, never **recomputes** the policy result. "economic_state reconstructed / matrix_drift 3/3" footers are affirmatively false for these arms | the replay badge "cannot catch the exact bug class that caused the wrong conclusions" (its own history: the argmax name-lie passed every gate) |
| money | integer-floor coin **leak** in `run_repeated` settle (:778-783); non-conservative self-wealth in `run_reputation` (:658-659); `realized_pnl_micro` not tape-derivable in `run_compete` (:1382-1390) | conservation + replay gaps on the money path |

### E. Process / on-disk hazards
- **Flip-flop**: price-causal verdict reversed 3× in ~2h on 2026-05-31, each declared "DEFINITIVE/PROVEN" on
  un-replicated single-day code.
- **Unretracted overclaims still on disk**: `AGENT_ECONOMY_EMERGENCE_PROVEN_2026-05-31.md`
  ("capital-at-risk is the isolated causal lever — earned, not asserted"), `PRICE_ECONOMY_DEFINITIVE_VERDICT`,
  `MULTI_AGENT_ECONOMY_FINAL_VERDICT`, `ROUTING_AB_REAL_LEAN_VALIDATION`, + 4 more.
- **C11**: a fabricated "17/24 71%" emergence number written before data landed (self-caught, but a dangling
  reference survives in 5 reports).
- **C8**: G0 "11/11" mixes real checks with hardcoded-literal conditions under one headline.

### F. Sound — keep
C7 (Stage-2 JUST_SAMPLING), C2 (Stage-1), the negatives (fair ablation), Sybil-resistance/governance (= Verdict B).

## 2. The systematic solution

### 2.1 Stop the bleeding — process gates (P0, cheap, do first)
1. **Correct (do not delete) the overclaimed reports.** Per AGENTS.md §8 (never rewrite historical evidence), prepend a
   `CORRECTION 2026-06-01` banner to each over-claimed report pointing here, with the honest scoped statement
   (e.g. C6 → "coverage/prompt-shaping, NOT price routing; erased at equal budget").
2. **The "no PROVEN" gate.** No "causal/PROVEN/DEFINITIVE" headline without ALL of: (a) a **fair equal-budget** baseline
   (same signal, same compute, no force-suicide), (b) **real LLM+Lean** (no synthetic skill axis), (c) ≥N seeds + paired
   stats, (d) a clean-context audit run **after** the data lands (not against blank tables), (e) the load-bearing number
   **recomputed from tape**, not merely byte-chain-intact.
3. **Relabel the badges.** "replay-green / matrix_drift 3/3" = anti-tamper + constitution-untouched, **NOT** a
   correctness warrant. Fix the discipline footers that currently assert correctness.

### 2.2 The real-value experiment (P1 — finally test Path 1)
**Promote `lean_market_agent.rs` to the canonical experiment** (it already has loss-bearing price + true softmax over the
full live index + arbitrary-parent restart). Close its **one** gap: let the **agent** read the full-chain price landscape
and choose the restart node, vs the harness softmax picking it. Also fix `g1`: swap argmax→`boltzmann_softmax_select_parent`
and un-truncate the 8-node view.

**Falsifiable arms (real LLM + real Lean — NOT synthetic):**
- `nonlocal_price` — full-chain loss-bearing-price softmax tree search (agents restart from any early node)
- `local_only` — same, but restart restricted to the current frontier (isolates **non-locality**)
- `heuristic_tree` — `lean_tree_market.rs` value()-routed tree (isolates **loss-bearing price** vs heuristic)
- `shuffled_price` — price permuted (isolates **price**)
- `single_chain` — one agent, DFS (the must-beat baseline)
- `uniform` — random parent (floor)

**Success = `nonlocal_price` > local_only AND > heuristic_tree AND > shuffled_price AND > single_chain**, paired
Wilcoxon + Holm, every cell replay-recomputed. This is the first experiment that exercises **both** constitutional
dimensions, and it answers — with **real** skill — what the skillsweep faked.

### 2.3 Replay + money (P2)
- Put the headline decision + outcome on tape (GenesisPin + RouteSample{chosen,price} + Verify), and make
  `verify_market_tape` **recompute** banked/route/pnl, not just check bytes.
- Fix the integer-floor coin leak (`run_repeated`), the non-conservative self-wealth update (`run_reputation`), and make
  `realized_pnl` tape-derivable (`run_compete`).

### 2.4 Re-settle the open questions honestly (P3)
- "combination is a capability lever" — already answered NO at equal budget (C7). Down-scope C6's claim accordingly.
- "capital-at-risk is causal" — the fair rival ties; the honest claim is **Sybil-resistance/governance (Verdict B)**,
  not single-shot causal efficiency (Verdict A). State it that way.

## 3. Prioritized action plan
| P | action | cost | removes |
|---|--------|------|---------|
| P0 | CORRECTION banners on 8 over-claimed reports + the "no PROVEN" gate + badge relabel | hours | the on-disk hazard + future repeats |
| P1 | promote `lean_market_agent`, close the agent-reads-landscape gap, fix g1 argmax→softmax + view; run the 6-arm real-value experiment | days | the Path-1 gap (the whole point) |
| P2 | tape-recompute replay + money-conservation fixes | 1-2 d | false-assurance + money leaks |
| P3 | down-scope C6/C1/C4 to their honest envelopes | hours | the wrong conclusions on record |

**The meta-lesson:** the campaign repeatedly substituted a one-dimensional proxy for the two-dimensional constitution and
then declared victory on it. The fix is not another proxy — it is to run the experiment that requires **both**
loss-bearing price **and** non-local restart, on the substrate that already implements both, with baselines that get the
same signal and the same budget, and a replay gate that **recomputes** rather than rubber-stamps.
