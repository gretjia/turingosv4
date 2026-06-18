# UNMERGED WORK — DEEP SUMMARY (P1 Real-Value market experiment)

> **Branch:** `claude/p1-realvalue` **HEAD:** `1da3f667` — **49 commits ahead** of `origin/main`, **NONE merged / no PR open**.
> **Diff base for everything in this document:** `origin/main...HEAD`.
> **Total diff:** 1534 files, **8149 insertions / 221 deletions**. The *file count* is dominated by evidence JSON (732 manifests + 732 replay); the *line count* is dominated by code + docs.
> **Branch is ~85 commits BEHIND `origin/main` and advancing** (`git rev-list --count HEAD..origin/main` = 81 at assembly, **85 at finalize** — `origin/main` is active; the task brief's "5 behind" was a stale pre-fetch read). **A rebase onto latest `origin/main` is mandatory before any PR** — see §5. (§6-untouched claim re-verified at finalize: empty diff against all six §6 surfaces + `lib.rs` + `runtime/mod.rs`.)
> **Purpose of this file:** single self-contained brief so an auditor/architect can understand ALL unmerged work and decide the PR split WITHOUT reading anything else. Deepest dumps are pointed to, not duplicated.
> **Constitution status:** **NO §6 restricted surface and NO Class-4 trust-root file is touched anywhere in this branch.** No `mod` is added to `lib.rs` / `runtime/mod.rs`. No §8 per-atom ratification is required for any PR below. (The one trust-root-adjacent item, L5 `mr-tick` in `runtime/mod.rs`, was explicitly **DEFERRED** out of this work.)

---

## 0. Executive summary

This branch is the **P1 Real-Value market experiment** end-to-end: a single, falsifiable test of the TuringOS constitutional claim that *a loss-bearing price market routes attention better than a single agent*. Concretely it asks: **does a loss-bearing price market route a Lean-4 proof-tree search better than a single linear chain?**

**Headline verdicts (both real, both honest, neither over-claimed):**

1. **P1 price-routing = NO-GO.** Across a corrected, prereg-locked, 732-cell, 9-arm sweep on real Lean 4.24.0 + Mathlib, the loss-bearing price market does **NOT** beat the controls. Adding the *real* price signal is the **single biggest negative delta** in the topology decomposition (shuffled_price → market = **−3.8pp**). The lever that *does* move solve-rate is **non-local tree search** (revisit/branch from any earlier node): single 0/78 → single_tree_no_price 4/78. But that lever needs **neither price nor a swarm** — a single agent revisiting its own tree is the cheapest near-best policy. The finding is **directional, NOT statistically significant** (max 6/78 solves; every McNemar `p_holm = 1.0`; underpowered). **Neither pre-registration confirmed.**

2. **Market binary = COMPLIANT-WITH-GAPS** against the FC1/FC2/FC3 constitution. The experiment harness is a faithful FC1 runtime instance on a real FC2 boot producing the FC3 within-run feedback substrate. All four white-box mechanisms (广播 broadcast / 屏蔽 shielding / 谓词 predicate / VETO) are LIT; the constitutional Veto-AI is correctly ABSENT (lives at FC3, not a bypass). No high-severity gaps; six minor gaps, two of which (M2, L4) were then **fixed and live-verified**.

**The arc that produced this** (full narrative in §1): a market-mission plan → a **v3** experiment whose causal claim ("`autonomous` cracked theorems `single`+`market` could not") was **VETO-ed by two external auditors** for **confound B** (the autonomous prompt secretly leaked the full search landscape) → a **v4 correction** (commit `cb89a5d6`: F1 decouple / F5 inline `#print axioms` gate / F2 compute telemetry / F3 topology baselines) → a **deeper discovery** that the v3 "hard floor" was itself unreliable (`single` actually solves `lm_ineq1`) → the corrected **9-arm 732-cell sweep** → the **NO-GO** verdict → an independent **clean-context audit = NO-VIOLATION / PROCEED** → an **FC1/2/3 compliance audit = COMPLIANT-WITH-GAPS** → the **M2/L4 gap fixes**, live-verified.

**Scope of the change (what's actually in the 49 commits):**
- **9 changed `src/` files** (2410 ins / 221 del): the F1/F2/F3/F5 corrections to `lean_market_agent.rs`, the inline axiom gate in `lean_judge.rs`, a new `#[path]`-included shared tape module, a new standalone replay verifier bin, plus three small additive helpers (librarian fail-closed ignore, price-index experiment floor) and two doc-only name-lie corrections.
- **Benchmark scaffolding:** 3 bash runners + 5 python read-only analyzers + 4 conformance/governance gate tests + 2 liveness fixtures.
- **Governance harness:** AGENTS.md §17 + CLAUDE.md §5 + `skills/no-proven-checklist.md` + `.gitignore` — **but 3 of these 4 are inherited from already-merged PR #225, not original P1 work** (see §1.0 and §5 PR-E — this is the one merge-order hazard).
- **~36 Class-0 handover docs** (reports / audits / preregs) + **committed evidence** (732 manifests + 732 replay + crack proofs).

**One-paragraph PR-split recommendation.** Split into **five PRs by blast radius + intent**, merge core-first: **PR-A core-runtime** (the `lean_judge` axiom gate + `market_tape_shared` + `verify_market_tape` + the two tiny state/runtime helpers) — the only changes a reviewer must scrutinize for soundness/CAS-consistency; **PR-B benchmark-scaffolding** (the `lean_market_agent` experiment bin + the other bins + runners + analyzers + conformance gates + prereg JSONs + liveness fixtures + the `.gitignore` ignore lines); **PR-C reports/plans/audits** (the ~30 Class-0 handover docs); **PR-D evidence** (the committed 732 manifests/replay — a *separate* evidence PR, **never** in the code PR, with the 8.7 G of untracked raw tape kept strictly local and gitignored); **PR-E governance-harness** (AGENTS.md §17 + CLAUDE.md + skill + the two `constitution_*.rs` mechanism gates) — **flag: 3 of its 4 doc files belong to already-merged PR #225; confirm you are not double-merging before re-authoring**. Merge order: **PR-E (if not already on mainline) → PR-A → PR-B → PR-D → PR-C.** Everything is Class 0/1/2; nothing touches §6 or the trust root; no §8 ratification is needed.

---

## 1. Full experiment narrative (chronological)

### 1.0 (pre-work) Governance inheritance — the §17 forensic gates

Before the P1-realvalue line proper, the branch **inherited** (mid-branch, history position ~22 of 49) the **§17 Claim Integrity Gates** from commit `4ccd2602` (2026-06-01, "harness: structural gates vs the 7 forensic failure modes"). This is the **SAME work** as already-MERGED **PR #225** (head `claude/harness-forensic-gates`, merged into `claude/emerge-stage2` at `ee1a2f0f`). It appears in this diff only because PR #225's merge target (`claude/emerge-stage2`) never reached `origin/main`, so §17 is genuinely absent from the diff base.

§17 added: a **conjunctive no-PROVEN gate** (G1 recompute-from-tape, G2 real model + verifier, G3 fair equal-budget baseline, G4 ≥N seeds + paired stats, G5 post-data clean-context audit persisted under `handover/audits/`, G6 no literal pass-condition); §17.2 replay-green ≠ correctness; §17.3 named-mechanism-must-match-implementation (softmax must distribute, price must be a real identifier); §17.4 binds to two `constitution_*.rs` gates + the `/no-proven-checklist` skill. **The P1 v4 line consumed this checklist downstream** — the corrected report's "NO-GO + directional, non-significant" scoping is exactly the §17.1 / G3 / G4 legal fallback. **Attribution caveat for the PR-splitter:** AGENTS.md / CLAUDE.md / `skills/no-proven-checklist.md` are PR #225 work, NOT original P1 work — see §5 PR-E.

### 1.1 Market-mission plan + P1 v3 experiment

The market-mission planning docs (`handover/planning/`: EXECUTION_PLAN, TASK_PACKAGE) framed the priced-DAG agent-market mission. The first concrete P1 instrument was `src/bin/lean_market_agent.rs` — a **price-routed Lean proof-tree search**: multiple Bull agents propose proof attempts on a shared tree; a Boltzmann-softmax over a **live price index** routes which parent node to expand; a Bear agent shorts doubtful nodes (the loss-bearing veto). The **v3** run (binary HEAD `1a812204`, prereg `P1_REALVALUE_PREREG_2026-06-01.json`) produced the headline: **"`autonomous` cracked hard theorems that `single`+`market` could not."**

### 1.2 The double external VETO (confound B) — the pivot

Two independent external auditors both returned **VETO** on the v3 causal claim (the 2nd report is authoritative — "以最后的内容为准"). Recorded in `handover/audits/P1_EXTERNAL_AUDIT_VETO_AND_CORRECTION_2026-06-02.md`. The A–F veto matrix:

| # | Axis | Verdict | Why |
|---|------|---------|-----|
| A | Confound-shield integrity | **FAILED** | `single` was budget-matched (24 proposals) but **NOT topology-matched** (hardcoded `own_last`, no root-restart/backtrack). Price arms make an extra skeptic `bear` LLM call `single` does not → compute asymmetry. |
| B | autonomous-vs-market | **FATALLY CONFOUNDED** | The fatal finding. "Identical except who picks the node" was **FALSE in code.** `market` → `build_prompt` (one parent body + error); `autonomous` → `build_autonomous_prompt` fed the **FULL search landscape** (top-6 FULL bodies + errors + all-node snippets) into the same proof call. The "crack" could be ~6× richer failure-context synthesis, **not** free-choice routing. |
| C | Cracks real? | **PASS** (sandbox-limited) | `lm_ineq1` (3a²+5b²≥2√15·ab) math sound (SymPy); auditors had no Mathlib to recompile; `lm_deriv1` unconfirmed. |
| D | Statistics + over-read | **PASS** | `p_holm = 0.7500` reproduced; rarity honestly disclosed; no PROVEN language. |
| E | Replay | **PASS economic / INCOMPLETE causal-cost** | LLM calls, prompt hashes, tokens, route decisions, axiom lists not tape-canonical. |
| F | Warrant the claim? | **NO** | Violated the prereg (PRIMARY required `market ≥ 1` AND `parallel`/`shuffled_price`/`no_price` all 0; those controls were OMITTED and the crack was `autonomous`, not `market`; `market` was 0/18). Subject mismatch + missing controls. |

**Net:** VETO the causal + prereg-win claim; PASS engineering / stats / math / economic-replay; **RECOVERABLE.**

### 1.3 The v4 correction — commit `cb89a5d6`

Workflow `wf8a015uq` (research → implement → LLM-free sandbox → 2× adversarial audit → repair; both PROCEED). **Class 2; edits ONLY `src/bin/lean_market_agent.rs` (+1525) and `src/judges/lean_judge.rs` (+324); no §6 surface; FC1/2/3 untouched; integer money.** Four fixes, each mapped one-to-one to a veto finding:

- **F1 DECOUPLE (kills confound B).** `autonomous` is now **TWO LLM calls.** Stage-1 routes on a **compact `build_route_summary`** (per node ONLY index / price-num-den / confidence / error-CLASS / age / short-hash — **NO body, NO shielded error, NO librarian**) → returns a parent index. Stage-2 builds the proof via the **single shared `stage2_proof_prompt`** = the SAME `build_prompt` that `market`/`single` use → **byte-identical proof prompt across arms** for the same parent. Enforced by `--self-test` prompt-parity: `sha(autonomous_stage2) == sha(market)` AND `!= confound_b_control`, mutation-tested. `autonomous` and `market` now differ ONLY in **who picks the parent.**
- **F5 INLINE AXIOM GATE.** `LeanJudge::verify` routes a clean exit-0 compile through `axiom_gate` (a SECOND `#print axioms <name>` Lean run); **Verified IFF** transitive axiom set ⊆ `{propext, Classical.choice, Quot.sound}`; **fail-closed** (no name / re-run !exit0 / no axiom line / non-whitelist → `axiom_rejected`). A source-scan also rejects `KERNEL_BYPASS_TOKENS {sorry, admit, native_decide}` BEFORE the kernel run (`native_decide` would otherwise exit-0 and pull `Lean.ofReduceBool` — the exact Lean axiom-whitelist honesty-gate trap).
- **F2 COMPUTE TELEMETRY.** Manifest carries proposal/route/bear LLM-calls + prompt/completion token split; invariant `total_model_tokens == proof + route + bear + completion`; route counted in the PPUT denominator (no autonomous discount).
- **F3 TOPOLOGY BASELINES.** Added `single_restart`, `single_tree_no_price`, `parallel_restart` (all Bulls-only).

### 1.4 The deeper finding — the v3 "hard floor" was UNRELIABLE

A v4 validity smoke found that `single` **actually SOLVES `lm_ineq1`** (seed 1, axiom-clean, the **SAME AM-GM route** v3 had credited to an autonomous "crack"). Because `single` gets 24 attempts/cell, a v3 "single 0/3" was a ~5% small-sample fluke. **Consequence:** the hard floor was re-established at **`single` 0/6** (6 floor seeds). Of 18 candidates → **13 robustly HARD**; **5 EXCLUDED** (`lm_ineq1` 1/6, `lm_median` 1/6, `lm_deriv2` 2/6, `lm_det_zero` 6/6, `lm_finset_sup` 1/6). This is a deeper flaw than either external auditor found, caught by the corrected harness's own floor smoke.

### 1.5 The corrected 9-arm 732-cell sweep

- **Prereg:** `P1_REALVALUE_V4_CORRECTED_PREREG_2026-06-02.json`, **SHA-locked `d80ba8cd`** (`.sha256` = `f1a36f16…43f5`, recomputed identical by the auditor) — **locked before the first counted cell.**
- **Runner:** `scripts/run_p1_v4_parallel.sh` (JOBS=6). **Analyzer:** `scripts/analyze_p1_v4.py`.
- **Model:** `deepseek-v4-pro`. **Verifier:** real **Lean 4.24.0 + Mathlib.**
- **Budget:** **24 proposals/cell every arm** (1-agent arms: 6 rounds × 4; 4-agent arms: 4 × 6). Equal-budget by construction (G3).
- **Stages:** STAGE A floor (`single` × 18 × 6 = 108) → STAGE B crack (5 arms × 13 hard × 6) → STAGE C topology (3 arms × 13 × 6). **732 cells total** (8 × 78 + 108).
- **"solved" = `omega_reached` = a LeanJudge axiom-clean Verified** (NOT a token-scan).
- **All 732 cells `verify_chaintape` replay-clean; every crack axiom-clean** via the inline F5 gate.

### 1.6 The NO-GO verdict

The full solve-rate table, decomposition, and prereg outcomes are in §2. In one line: **price-routing is NOT the lever; non-local tree search is the (directional, non-significant) lever; neither pre-registration is confirmed (`p_holm = 1.0` on both).** See `handover/reports/P1_REALVALUE_V4_FINDINGS_2026-06-02.md`.

### 1.7 Clean-context audit — NO-VIOLATION / PROCEED (§17 G5)

An independent skeptical auditor with **no implementation transcript** (`handover/audits/P1_REALVALUE_V4_AUDIT_2026-06-02.md`) reproduced the solve-rate table byte-for-byte, re-derived every McNemar `p_holm = 1.0` by hand, recompiled 4 cracks under real Lean 4.24.0 + Mathlib with independent `#print axioms`, confirmed the hard floor (78 single-hard cells all ran exactly 24 proposals, `verified_count = 0`), confirmed token conservation on all 732 cells, and confirmed the softmax is a real distribution (not argmax-collapse). **Verdict: NO-VIOLATION; PROCEED.** Detail in §2.6.

### 1.8 FC1/FC2/FC3 compliance audit — COMPLIANT-WITH-GAPS

A read-only constitutional-compliance audit (workflow `wf7b6ms3m`, `handover/audits/MARKET_FC_COMPLIANCE_2026-06-02.md`) over 732 manifests / 16,758 nodes confirmed every FC1 node (N1–N15) + both FC2 boot/halt branches LIT, and all four white-box mechanisms LIT with the Veto-AI correctly absent (at FC3). **No high-severity gaps.** Six minor gaps (2 Medium / 3 Low / 1 Info). Detail in §2.7.

### 1.9 Gap fixes M2 + L4 — live-verified

M2 + L4 were then **FIXED** in `lean_market_agent.rs` (+ tests; build 14/0; `--self-test` OK; no §6/trust-root; integer money intact), and **re-verified on a LIVE `market`/`lm_e` cell** (12 proposals, 12 failed, `bear_calls = 12`):
- **M2:** `Manifest.librarian_notice_nonempty_count` + `librarian_notice_chars`, counted at the Stage-2 injection point → LIVE **`librarian_notice_nonempty_count = 10/12`, `librarian_notice_chars = 2642`** (广播 injection is now a DIRECT manifest scalar, was recompute-only; the 2 misses are the first proposals before any failure exists to summarize).
- **L4:** `reject_class_of()` + `AttemptNode.reject_class` (`lean-reject` / `axiom-rejected` / `sorry-blocked`; `None` iff Verified), deterministic `l4_reject_class_taxonomy` test → LIVE **`reject_class` on all 12 failed nodes**; invariant `reject_class == None ⟺ is_verified` holds.
- **M1** axiom-reject + **L3** SorryBlocked were already real-Lean unit-tested (`lean_judge.rs:668` hand-axiom → `axiom_rejected`; `:552` `verify("sorry")` → `SorryBlocked`) and are now tape-visible via `reject_class` if they fire in production.
- **L5 DEFERRED** (would touch `runtime/mod.rs` trust-root → dedicated atom). **Info6** by-design (covered by `tests/constitution_fc3_closure.rs`).

---

## 2. Results + evidence

### 2.1 The 9-arm solve-rate table (78 hard cells = 13 hard theorems × 6 seeds)

| arm | mechanism | solved | rate | Wilson 95% | tok/solve |
|-----|-----------|:------:|:----:|:----------:|:---------:|
| `single` | one linear chain | 0/78 | 0.0% | [0, 4.7%] | — |
| `single_restart` | 1 agent, root-restart or last | 2/78 | 2.6% | [0.7, 8.9%] | 584K |
| `single_tree_no_price` | 1 agent, restart from ANY own node, no price | 4/78 | 5.1% | [2.0, 12.5%] | **329K (BEST)** |
| `parallel` | 4 independent linear chains | 2/78 | 2.6% | [0.7, 8.9%] | 659K |
| `parallel_restart` | 4 indep chains + root-restart | 2/78 | 2.6% | [0.7, 8.9%] | 546K |
| `no_price` | 4 agents, shared tree, RANDOM restart | **6/78** | **7.7%** | [3.6, 15.8%] | 344K |
| `shuffled_price` | 4 agents, shared tree, PERMUTED-price softmax | 5/78 | 6.4% | [2.8, 14.1%] | 431K |
| `market` | 4 agents, shared tree, REAL-price softmax (Path-2) | 2/78 | 2.6% | [0.7, 8.9%] | 1.13M |
| `autonomous` | 4 agents, shared tree, LLM free-choice (Path-1) | 1/78 | 1.3% | [0.2, 6.9%] | **3.09M (WORST)** |

Floor split confirms **each of the 13 hard theorems has `single` 0/78**; total `verified_count` across all 78 single-hard = **0**, all ran exactly 24 proposals. (`single` floor cells = 108 total: 18 candidates × 6 seeds; the 13 hard × 6 = 78 are the ones in this table.)

### 2.2 Topology decomposition deltas (the contrasts the arms isolate; auditor reproduced exactly)

| contrast | Δ | reading |
|----------|:--:|--------|
| `single` → `single_restart` | **+2.6pp** | root-restart helps a little |
| `single_restart` → `single_tree_no_price` | **+2.6pp** | branching from ANY own node helps more |
| `single_tree_no_price` → `parallel` | **−2.6pp** | 1 agent w/ non-local tree BEATS 4 independent linear chains |
| `parallel` → `parallel_restart` | **+0.0pp** | — |
| `parallel_restart` → `no_price` | **+5.1pp** | a SHARED non-local tree is the big jump |
| `shuffled_price` → `market` | **−3.8pp** | **adding the REAL price signal is the BIGGEST NEGATIVE delta** |
| `market` → `autonomous` | **−1.3pp** | free-choice doesn't help either |

**Read:** **non-local tree search** (revisit/branch from any earlier node) is the lever; sharing the tree across agents amplifies it; **price-routing and free-choice routing add nothing and point-wise SUBTRACT** (they over-concentrate the search, losing the diversity that random/shuffled restart keeps).

### 2.3 The two pre-registrations — BOTH NOT CONFIRMED

- **PREREG_1 (market-Hayek loss-bearing price routing): NOT CONFIRMED.** `market` `CONFIRMED_WIN` = 1 theorem (`lm_fact`), but `market` **LOSES** to `shuffled_price` (2 vs 5) and `no_price` (2 vs 6). **`p_holm = 1.0`.** Real price does **worse** than destroyed/random price.
- **PREREG_2 (autonomous free-choice routing): NOT CONFIRMED.** `autonomous` uniquely cracks `lm_deriv1` (real, axiom-clean `[Classical.choice, Quot.sound, propext]`) — 1 theorem, **`p_holm = 1.0`**, AND it is the **lowest-productivity (1/78) + highest-cost (3.09M tok/solve)** arm. Free choice is genuine (0% route hallucination: `valid_index_hit = 1376`, `fresh_root = 479`, `hallucinated = 0`) but doesn't help.

### 2.4 The existence layer — 24 axiom-clean cracks (LAYER 1 = PASS)

24 axiom-clean cracks across multi-arm policies on single-0 theorems → **organization of SOME kind beats a single chain**. **7 of 13** hard theorems cracked by someone: `{lm_deriv1, lm_f, lm_fact, lm_ineq2, lm_ineq3, lm_natdeg_pow, lm_nt_gcd2}`. **6 cracked by NOBODY:** `{lm_c, lm_coeff_mul, lm_e, lm_lim1, lm_nt_cop_cubic, lm_probe1}` (these are genuinely beyond every arm at this budget — evidence the model is strong, not the test trivial).

**Three-layer read:** LAYER 1 EXISTENCE = PASS (organization beats single chain). LAYER 2 RELIABILITY = ordering consistent (non-local-tree arms 5–8% > linear-chain arms 0–2.6%; price/free-choice routing 1.3–2.6% does NOT beat random/heuristic restart) but **CIs overlap → directional, not significant.** LAYER 3 CAUSAL/PREREG = both NOT confirmed (§2.3).

### 2.5 FC1/2/3 compliance matrix — 4 white-box mechanisms LIT + Veto-AI correctly absent

Corpus: 732 manifests, 16,758 nodes (35 Verified + 16,723 Failed-with-feedback). Every FC1 node N1–N15 + both FC2 boot/halt branches LIT.

- **广播 BROADCAST (Librarian): LIT.** Fires every proposal (`real_librarian_solver_notice`, unconditional, `lm_agent:1349`); fed by real CAS `lean_result.v2` ×24 sidecars with `stderr_cid=None`; source-scoped + shielded + projected to the Solver role. Price-signal half independently LIT (`distinct_price_ratios` up to 18, `price_discovery=true`).
- **屏蔽 SHIELDING: DECISIVELY LIT.** Across 16,723 feedback nodes **MAX NEWLINE COUNT = 0** (max-len 160) — a raw multi-line Lean stderr cannot produce 0 newlines, so the shield fired on EVERY failing verify. Three shield sites confirmed (judge `shield_lean_diagnostic`, route-summary error-CLASS-only, librarian source-scope).
- **谓词 PREDICATE (Π_p): LIT (dominant gate).** Lean kernel Verified (`sorry`/`admit`/`native_decide` source-rejected) AND a SECOND `#print axioms` run ⊆ `{propext, Classical.choice, Quot.sound}`. All 35 omega nodes carry non-empty whitelisted axioms; **0 dirty.** Price never overrides the predicate.
- **VETO (split correctly into two orthogonal vetoes, both honored):** (a) in-loop **loss-bearing Bear-short price veto LIT** — `bear_calls` on every price-family arm (`market`/`autonomous`/`no_price`/`shuffled_price`), **exactly 0** on every Bulls-only control (`single*`/`parallel*`), 0 exceptions across 732; real `ChallengeTx` + `lm.counterexample.v1` CAS, replay-reconstructed. (b) **constitutional Veto-AI** (`{PASS,VETO}` over architecture) **CORRECTLY ABSENT** (0 grep hits, lives at FC3) — conformant, NOT a bypass, no name-lie.

**No high-severity gaps.** No name-lie, no price-overrides-predicate, no raw-stderr leak, no argmax-as-softmax, no silent-drop.

### 2.6 Clean-context audit detail (NO-VIOLATION / PROCEED)

- **A. RECOMPUTE (G1) PASS:** solve-rate table reproduced byte-for-byte; `CONFIRMED_WIN`s `market=[lm_fact]`, `autonomous=[lm_deriv1]`; all `p_holm=1.0000`; deltas `+2.6/+2.6/−2.6/+0.0/+5.1/−1.3/−3.8/−1.3`; tok/solve `329538 / 1130480 / 3086598`; **no EXCLUDED block (no silent drops)**; McNemar one-sided exact + Holm re-derived by hand (`market` vs `no_price` b=2,c=6 → raw p=0.9648; smallest raw p=0.25; Holm ×4 capped at 1.0); `"solved"` verified ≡ `omega_reached || verified_count>0` (both=35, 0 mismatch), all 35 ⊆ whitelist (0 dirty).
- **B. HARD-FLOOR PASS:** 78 single-hard cells all ran exactly 24 proposals (min=max=24), total `verified_count=0` → 0/78 floor real; 13 HARD + 5 EXCLUDED reproduced; `lm_ineq1` single-solve RECOMPILED (CAS `076e0363…`, AM-GM via `positivity`+`nlinarith`, exit 0, axioms `[propext, Classical.choice, Quot.sound]`).
- **C. CRACKS REAL + AXIOM-CLEAN PASS:** 4 cracks recompiled under real Lean 4.24.0 + Mathlib with independent `#print axioms` — `autonomous lm_deriv1 s3` (`f96e88b1…`), control `no_price lm_ineq2 s3` (`c2698919…`), `single lm_ineq1 s1` (`076e0363…`), `market lm_fact s2` (`331aca24…`) — all exit-0 + axiom-clean; sibling candidates sharing the `body_preview` prefix correctly FAILED (type-mismatch+sorryAx, unknown identifier `cauchy_schwarz_iff`, rewrite/unsolved goals+sorryAx) → the gate distinguishes pass from fail.
- **D. FAIR BASELINE (G3) PASS:** self-test → `PROMPT-PARITY-OK route_summary_clean=true sha=94f6153c…` exit 0; load-bearing (has a negative control); equal budget 24 every arm; bulls-only arms `bear=0`, price arms `bear≈1782–1826`, only `autonomous` `route≈1855`; token-conservation `total==proof+route+bear+completion` holds for all 732 cells (0 violations); softmax is a real distribution (`distinct_price_ratios` mean 6.2 max 19, `price_discovery=True`), not argmax-collapse.
- **E. OVER-READ / HONESTY PASS:** no PROVEN/causal over-claim; non-significance hedged repeatedly; checked the reverse — `p(market beats no_price)=0.9648`, NOT significant in either direction → report correctly claims "price does not BEAT random price," not "price proven worse."
- **F. REPLAY (G1) PASS:** all 732 `.replay.json` `economic_state_reconstructed=true` (0 not-true); EXISTENCE reconciles to 24 cracks; 7-cracked / 6-uncracked lists identical.
- **Harness gates on the binary:** `constitution_headline_recompute_from_tape` **6/0** (incl. `recompute_catches_a_lying_manifest_while_byte_chain_stays_green`), `constitution_router_name_matches_mechanism` **8/0**.

### 2.7 The six FC-compliance gaps + the M2/L4 closures

| gap | sev | what | status |
|-----|:---:|------|--------|
| M1 | Med | axiom-gate non-whitelist catch DORMANT (only the fail-closed name-miss branch fired; no proof smuggled a bad axiom) | real-Lean unit-tested (`lean_judge.rs:668`); now tape-visible via `reject_class`; red-team `native_decide` task suggested |
| M2 | Med | librarian Stage-2 injection had no direct manifest tell | **FIXED + LIVE** (`librarian_notice_nonempty_count=10/12`, `chars=2642`) |
| L3 | Low | SorryBlocked reject arm DORMANT (model emitted no bare `sorry`) | real-Lean unit-tested (`lean_judge.rs:552`); now tape-visible via `reject_class` |
| L4 | Low | §4 `reject_class` literal-field divergence (bin carried `verdict`+`parse_fails`, not a `reject_class` node field) | **FIXED + LIVE** (`reject_class` on all 12 failed nodes) |
| L5 | Low | FC2 `mr-tick` not JSON-re-derivable (only via L4/gate + grid count) | **DEFERRED** (would touch `runtime/mod.rs` trust-root → dedicated atom) |
| Info6 | Info | FC3 meta-loop by-design absent | covered by `tests/constitution_fc3_closure.rs` |

### 2.8 Evidence paths (cite these)

- **Committed corrected sweep:** `handover/evidence/p1_v4_2026-06-02/` — **732 manifest `.json` + 732 `.replay.json` + `STAGE_A_HARD_FLOOR_FINDINGS.md`** (~17 M, derived views only; raw `repo_*`/`cas_*` 35+35 correctly gitignored). The data behind the NO-GO.
- **Axiom-clean existence proof:** `handover/evidence/p1_axiom_confirm_2026-06-02/lm_ineq1_crack_VERIFIED.lean` (`ineq_amgm_concrete`, axioms `{propext, Classical.choice, Quot.sound}`).
- **Supersession annotation:** `handover/evidence/P1_CONFOUNDED_SUPERSEDED_2026-06-02.md` (AGENTS.md §8 annotate-not-mutate; retires v3/scaleup).
- **Prereg SHA-lock:** `handover/preregistration/P1_REALVALUE_V4_CORRECTED_PREREG_2026-06-02.json` + `.sha256` (`d80ba8cd`; lock `f1a36f16…43f5`, verified byte-exact).
- **Verdict reports:** `handover/reports/P1_REALVALUE_V4_FINDINGS_2026-06-02.md`, `handover/reports/SESSION_P1V4_STAGE_SUMMARY_2026-06-02.md`.
- **Audits:** `handover/audits/P1_REALVALUE_V4_AUDIT_2026-06-02.md`, `handover/audits/MARKET_FC_COMPLIANCE_2026-06-02.md`, `handover/audits/P1_EXTERNAL_AUDIT_VETO_AND_CORRECTION_2026-06-02.md`.

> **EVIDENCE HYGIENE WARNING (load-bearing for whoever cuts the evidence PR):** the working tree carries **8.7 G of UNTRACKED P1 evidence that is mostly NOT gitignored** — a naive `git add handover/evidence` would sweep it all in. The `.gitignore` excludes `repo_*`/`cas_*` ONLY for `p1_v4_2026-06-02` and `p1_v4_smoke_2026-06-02`. Biggest bloat risk: `p1_v4_laterday_full_2026-06-03/` = **8.2 G / 2.13 M files** (the G4 stability re-run, Stage-B 1056/1056 replay-clean, AWAITING-AUDIT) whose 1272 `cas_*` + 1272 `repo_*` are **NOT** ignored. Also untracked: `p1_scaleup_2026-06-02` (230 M, SUPERSEDED/CONFOUNDED), `p1_realvalue_v3_2026-06-02` (SUPERSEDED v3), `p1_realvalue*_2026-06-01` (earlier iterations), several smoke/scratch/driftfix/aborted dirs (~96 M), `t2_shared_sweep_2026-06-01`, `constitution_gates_..._rescued`. **Extend `.gitignore` to cover all `repo_*`/`cas_*` (and the superseded/smoke dirs) BEFORE any `git add`.** Also: inside the *committed* `p1_v4_2026-06-02/` dir there are **732 unstaged `run_p1v4_*.log` + 1 `CRACK_*.txt`** (raw stdout, not gitignored, not derived evidence) — confirm they are NOT `git add`-ed.

---

## 3. Code evidence (the load-bearing functions)

> File:line markers below are the anchors named in the slice surveys + the FC-compliance audit. For the fuller verbatim dumps see `handover/audits/P1_REALVALUE_V4_AUDIT_2026-06-02.md` and `handover/reports/P1_REALVALUE_V4_FINDINGS_2026-06-02.md`. (No `*_FULL_AUDIT_PACKAGE` doc is committed on this branch — the v3 `FULL_AUDIT_PACKAGE` lives only under the historical `handover/audits/GROUP_3_V3_AUDIT_INPUT_DOCS` group; the v4 detail is in the two docs just named.)

### 3.1 F1 — the autonomous decouple (kills confound B)

The fix has three load-bearing pieces, all in `src/bin/lean_market_agent.rs`:

1. **`stage2_proof_prompt`** — the SINGLE shared proof-prompt constructor. `market`, `single`, and `autonomous`-Stage-2 all call it for the same parent, so `sha(autonomous_stage2) == sha(market)` by construction. This is what makes "differ ONLY in who picks the parent" *true in code*.
2. **`build_route_summary`** — the Stage-1 router input. Per node it emits ONLY `{index, price_num/den, confidence, error-CLASS, age, short-hash}` — **no body, no shielded error, no librarian digest**. This is the leak that confound B exploited, now closed: the router sees a compact frontier, the prover sees the same body `market` sees.
3. **The live-loop Stage-1 / Stage-2 split** — `autonomous` makes a Stage-1 route-only LLM call over `build_route_summary` → parent index, then a Stage-2 proof call over `stage2_proof_prompt(parent)`. Route telemetry splits into `deliberate_fresh_root` / `valid_index_hit` / `hallucinated_out_of_range` (observed `479 / 1376 / 0`).
4. **`--self-test` parity gate** (LLM-free, Lean-free, mirrored as `#[test]`s): asserts `sha(autonomous_stage2) == sha(market)` AND `sha(autonomous_stage2) != sha(confound_b_control)` (the landscape-augmented control), mutation-tested. Self-test live output: `PROMPT-PARITY-OK route_summary_clean=true sha=94f6153c… exit 0`.

### 3.2 F5 — the inline `#print axioms` soundness gate (`src/judges/lean_judge.rs`, +312/−12)

The highest-attention change in the branch (it changes what counts as "Verified"). Added surface: `pub const AXIOM_WHITELIST = {propext, Classical.choice, Quot.sound}`, `LeanOutcome.axiom_rejected` + `LeanOutcome.axioms` fields, and helpers `axiom_gate()`, `extract_theorem_name()`, `parse_axiom_set()`, `axiom_rejected()`. Flow:

```
verify(src):
  source-scan reject KERNEL_BYPASS_TOKENS {sorry, admit, native_decide}   # BEFORE kernel; native_decide is exit-0 but pulls Lean.ofReduceBool
  compile src
  if exit-0 clean:
      name = extract_theorem_name(src)            # no name  -> axiom_rejected (fail-closed)
      out  = run a SECOND sanitized `lean` emitting `#print axioms <name>`
      if !exit-0 or no axiom line -> axiom_rejected (fail-closed)
      axioms = parse_axiom_set(out)
      Verified  IFF  axioms ⊆ AXIOM_WHITELIST      # else axiom_rejected
```

**Fail-closed everywhere.** Deliberately does **NOT** extend the repr-stable, CAS-hash-bearing `LeanVerdictKind` enum — an axiom-reject is modeled as the canonical `Failed` arm (`exit_code=1`) so the CAS `LeanResult` sidecar stays `assert_45`-consistent. Covered by 4 new tests (2 pure-parse always-run + 2 real-Lean gated: `:668` hand-axiom → `axiom_rejected`; `:552` `verify("sorry")` → `SorryBlocked`). Empirically pinned on Lean v4.24.0. The two new `LeanOutcome` fields ripple into `lean_market_agent` (per-node `axioms` + `reject_class`).

### 3.3 The loss-bearing price (`bear_doubt_short` + `compute_price_index`)

`src/bin/lean_market_agent.rs` is the ONLY bin with a **loss-bearing** price (the `constitution_router_name_matches_mechanism` gate pins it as the canonical priced-softmax substrate via `compute_price_index` + `boltzmann_softmax_select_parent`). The Bear short (`bear_doubt_short`) is the in-loop loss-bearing VETO: on every price-family arm it issues a real `ChallengeTx` + writes an `lm.counterexample.v1` CAS object (`bear_calls` = 1 per proposal on `market`/`autonomous`/`no_price`/`shuffled_price`; exactly 0 on Bulls-only `single*`/`parallel*`, 0 exceptions across 732). `compute_price_index` builds the live per-node price the softmax routes over (`distinct_price_ratios` up to 18/19, `price_discovery=true` — a real distribution, not argmax-collapse). **Price never overrides the predicate** (FC §2.5): a Verified node is gated by the kernel + axiom gate, never by price.

### 3.4 The real librarian wiring (广播)

Replaces an experiment-local lookalike: the librarian collective digest is wired from **typed `LeanResult` sidecars** read from real CAS (`lean_result.v2` ×24, `stderr_cid=None`), source-scoped + shielded + projected to the Solver role, fired unconditionally every proposal (`real_librarian_solver_notice`, `lm_agent:1349`). The M2 fix added the direct manifest tell at the Stage-2 injection point (see 3.6).

### 3.5 F2 — honest compute telemetry

Manifest carries `proposal` / `route` / `bear` LLM-call counts + a prompt/completion token split, with the invariant `total_model_tokens == proof + route + bear + completion` (holds on all 732 cells, 0 violations). The `route` call is counted in the PPUT denominator — `autonomous` gets **no** discount for its extra Stage-1 call, which is why it shows as the highest-cost arm (3.09 M tok/solve) rather than hiding the cost.

### 3.6 L4 — `reject_class_of` + M2 librarian tell

L4 added `reject_class_of()` and `AttemptNode.reject_class` (`lean-reject` / `axiom-rejected` / `sorry-blocked`; `None` iff Verified), with the invariant `reject_class == None ⟺ is_verified` (deterministic `l4_reject_class_taxonomy` test; LIVE on all 12 failed nodes of the verification cell). M2 added `Manifest.librarian_notice_nonempty_count` + `librarian_notice_chars` counted at the Stage-2 injection point (LIVE `10/12`, `2642` chars). Both asserted present by `f2_manifest_has_compute_telemetry_fields`.

---

## 4. Complete file inventory

Grouped by `pr_category`. **+added** is from the slice numstats. Grouped rows (e.g. doc-survey GROUP_* aggregates) are marked. **§6 / trust-root touched: NONE, anywhere.**

### 4.A core-runtime (the soundness-bearing changes — reviewer scrutiny)

| path | +added | −del | risk | purpose |
|------|:------:|:----:|:----:|---------|
| `src/judges/lean_judge.rs` | 312 | 12 | Class 2 (verifier) | **HIGHEST-attention.** Inline `#print axioms` whitelist gate (F5); changes what is "Verified"; fail-closed; does NOT extend CAS-bearing `LeanVerdictKind`; 4 new tests. |
| `src/market_tape_shared.rs` | 189 | 0 | Class 2 (substrate) | NEW `#[path]`-included shared module (TP-0A.1): `MarketEvent` enum incl. `GenesisPin`, `prev_hash` chain, `derive_*` replay derivations, `MODEL_RATES`/`call_micro_usd`. **NOT declared as `mod` in lib.rs** (deliberate trust-root avoidance) — pulled in via `#[path]`. Integer money. |
| `src/bin/verify_market_tape.rs` | 98 | 0 | Class 2 (new bin) | NEW standalone read-only replay verifier (TP-0A.6): reconstructs headline integers FROM THE TAPE ALONE, asserts integer equality with manifest + universal checks; exit 0 iff replay-clean. Depends on `market_tape_shared`. |
| `src/runtime/librarian_broadcast.rs` | 6 | 1 | Class 2-ish (real runtime file) | One match guard: `lm.proof_artifact.v1` → `Ok(None)` instead of HARD-ERROR. Purely additive fail-closed reader hardening; no behavior change for existing schemas. |
| `src/state/price_index.rs` | 29 | 0 | Class 1 (additive primitive) | TP-1 exploration-floor: `BOLTZMANN_MIN_EPSILON_NUM/DEN (1/10)` + `epsilon_meets_experiment_floor()` (integer cross-multiply). NOT a `from_env` change, NOT a clamp; additive predicate + 1 test. |

### 4.B benchmark-scaffolding (experiment harnesses + analysis + conformance — a red here cannot touch production)

| path | +added | −del | risk | purpose |
|------|:------:|:----:|:----:|---------|
| `src/bin/lean_market_agent.rs` | 1577 | 84 | Class 2 (experiment BIN) | The v4-corrected P1 harness (centerpiece). ~40% real (F1 decouple / F3 topology arms / F2 telemetry / F5 axiom footprint + `reject_class` / real librarian / route telemetry / `--self-test` parity gate); ~60% rustfmt reformatting. Depends on PR-A (`lean_judge::AXIOM_WHITELIST` + `LeanOutcome.axioms/axiom_rejected`). |
| `src/bin/lean_hayek_market.rs` | 184 | 121 | Class 2 (experiment BIN) | TP-0A shared-tape refactor (deletes inline tape/rates, imports `market_tape_shared`) + `GenesisPin` first event + `run_alloc_shared()` (shared-state confound fix: 6 arms = deterministic policies over IDENTICAL state) + flatbid/coordinator audit fixes + `--reasoner-model`. Depends on PR-A. |
| `src/bin/lean_hetero_market.rs` | 9 | 2 | Class 2 ≈ Class 0 (doc-only) | DOC-ONLY name-lie correction: this bin has ZERO market/price machinery (round-robin + SKIP), "do NOT cite as price-market evidence." |
| `src/bin/lean_tree_market.rs` | 6 | 1 | Class 2 ≈ Class 0 (doc-only) | DOC-ONLY name-lie correction: "market" policy is heuristic Boltzmann-softmax over a heuristic `value()`, no loss-bearing price; points to `lean_market_agent.rs` as the only priced bin. |
| `scripts/run_p1_realvalue.sh` | 66 | 0 | Class 1 (runner) | P1 per-(theorem,arm,seed) runner; resumable; every cell gated on `verify_chaintape` replay-clean; writes `handover/evidence/` only. |
| `scripts/run_p1_v4_parallel.sh` | 70 | 0 | Class 1 (runner) | Corrected-P1 (v4) bounded-concurrency (JOBS) sweep runner; `KEEP_CAS=1`; resumable, replay-gated. **The runner of record for the 732-cell sweep.** |
| `scripts/run_t2_shared_sweep.sh` | 62 | 0 | Class 1 (runner) | T2 shared-state confound-free sweep runner; 1 invocation/seed emits all 6 arm manifests+tapes over IDENTICAL state; gated on `verify_market_tape`. |
| `scripts/analyze_p1_v4.py` | 255 | 0 | Class 1 (read-only analyzer) | Corrected-P1 3-layer stats (existence/reliability/causal), hard-floor derivation, Wilson/Jeffreys CIs, exact McNemar+Holm, compute-telemetry parity, topology decomposition, route-honesty. **The analyzer of record.** |
| `scripts/analyze_p1_realvalue.py` | 114 | 0 | Class 1 (read-only) | Earlier P1 analyzer — paired exact McNemar + Holm, replay-clean cells only, Verdict A. |
| `scripts/analyze_p1_hardproblem.py` | 109 | 0 | Class 1 (read-only) | Hard-problem analyzer — confirmed-crack detection + autonomous/market/single McNemar + route honesty. |
| `scripts/analyze_t2_sweep.py` | 147 | 0 | Class 1 (read-only) | T2 paired Wilcoxon + Holm vs the locked T2 shared-state prereg; over-budget exclusions; GO/INCONCLUSIVE/NO-GO. |
| `scripts/analyze_t2_routing_efficiency.py` | 85 | 0 | Class 1 (read-only) | T2 routing-capture diagnostic — normalizes banked@B by routing room `(banked−free)/repairable`. |
| `tests/market_tape_canonical_roundtrip.rs` | 123 | 0 | Class 2 (conformance) | TP-0A PPUT replay-substrate conformance: `derive_*` reconstruction, GenesisPin-first, 1-byte tamper detection, failed-branch-on-tape, + end-to-end `verify_market_tape` exit-code contract. `#[path]` shared module. |
| `tests/t2_microstructure_conformance.rs` | 88 | 0 | Class 2 (conformance) | TP-1 5-pillar spec-lock: static predicates (sequencer price-blind, no MarketBuy/Sell, epsilon-floor, axiom-whitelist, named arms) run now; 2 live predicates `#[ignore]`'d until the T2 harness. READS §6 `sequencer.rs`/`typed_tx.rs` **as strings** to assert price-blindness (does not edit). |

### 4.C governance-harness (forensic gates + harness docs — **see attribution flags**)

| path | +added | −del | risk | purpose / attribution |
|------|:------:|:----:|:----:|---------|
| `tests/constitution_headline_recompute_from_tape.rs` | 198 | 0 | Class 2 (gate) | NEW §17.2/G1 gate — recompute headline integers from frozen tape via shared `derive_*`; catches a lying manifest while the byte chain stays green; paired anti-tamper + genesis-first controls. `#[path]` (avoids lib.rs mod). **Belongs to the §17 / PR#225 bundle.** |
| `tests/constitution_router_name_matches_mechanism.rs` | 305 | 0 | Class 2 (gate) | NEW §17.3 name-lie gate — softmax must DISTRIBUTE (≥3/5 equal-price nodes); any `src/bin/*.rs` claiming price-routing in prose must carry a real `price` identifier; pins `lean_market_agent.rs`. **§17 / PR#225 bundle.** |
| `scripts/constitution_gates.manifest.toml` | 10 | 0 | Class 2 (gate manifest) | Registers the two new §17 gates so `run_constitution_gates.sh` runs them. **§17 / PR#225 bundle.** |
| `tests/constitution_matrix_drift.rs` | 7 | 0 | Class 2 (gate self-config) | Grandfathers the 2 new gates into `BASELINE_ALLOWLIST`. **FOOTGUN: allowlist now = 69 = `K23_SHIP_ALLOWLIST_SIZE` cap** — any further allowlist add without bumping the cap turns `allowlist_doesnt_grow_silently` RED. **§17 / PR#225 bundle.** |
| `tests/agents_md_keep_anchors.rs` | 11 | 0 | Class 2 (anchor-survival) | Adds the §17 KEEP-anchor so a future AGENTS.md slim cannot drop §17. **§17 / PR#225 bundle.** |
| `AGENTS.md` | 55 | 0 | Class 0 (doc, ADD-only) | §17 Claim Integrity Gates (G1–G6, §17.2/.3/.4) + 1 §14 auditor-checklist line. **ATTRIBUTION: NOT P1 work — 100% from commit `4ccd2602` = already-MERGED PR #225.** |
| `CLAUDE.md` | 4 | 0 | Class 0 (doc) | §5 pointer to `/no-proven-checklist`. **ATTRIBUTION: PR #225, not P1.** |
| `skills/no-proven-checklist.md` | 101 | 0 | Class 0 (skill) | The conjunctive G1–G6 pre-claim checklist. Absent from `origin/main`. **ATTRIBUTION: PR #225, not P1** (but the P1 v4 line consumed it). |
| `tests/fixtures/liveness/script_liveness_inventory.toml` | 10 | 0 | Class 1 (inventory) | Registers 5 analyzers + 3 runners (dev_harness/dev_only, `counts_for_obl005=false`). **Working-tree drift: a +2 uncommitted edit adds a 4th runner NOT in the committed diff — exclude.** |
| `tests/fixtures/liveness/production_module_liveness.toml` | 1 | 0 | Class 1 (inventory) | Adds `bin::verify_market_tape` to the `runtime_replay_evidence_audit` group. |

> Note: in the slice surveys the two liveness fixtures and the FC-compliance audit doc were tagged `governance-harness`, but functionally the liveness fixtures classify the PR-B runners/analyzers and **must ship in PR-B** (shipping them in PR-E would reference paths PR-E doesn't add and red the no-zombie gate). They are listed here for completeness; assign them to PR-B at cut time.

### 4.D reports-plans (Class 0 handover docs)

| path | +added | risk | purpose |
|------|:------:|:----:|---------|
| `handover/ai-direct/LATEST.md` | 47 | Class 0 | LOAD-BEARING handover; v4 NO-GO snapshot |
| `handover/reports/SESSION_P1V4_STAGE_SUMMARY_2026-06-02.md` | 108 | Class 0 | LOAD-BEARING v4 narrative index |
| `handover/reports/P1_REALVALUE_V4_FINDINGS_2026-06-02.md` | 123 | Class 0 | PRIMARY result; price-routing NO-GO; supersedes v3 |
| `handover/reports/SESSION_FORENSIC_RETROSPECTIVE_2026-06-01.md` | 142 | Class 0 | LOAD-BEARING §17 authority; Verdict-A/B |
| `handover/preregistration/P1_REALVALUE_V4_CORRECTED_PREREG_2026-06-02.json` (+`.sha256`) | 93 (+1) | Class 0 SHA-pinned | The decisive prereg the v4 analyzer reads against; lock `f1a36f16…`, SHA `d80ba8cd` |
| `handover/preregistration/P1_REALVALUE_PREREG_2026-06-01.json` (+`.sha256`) | 77 | Class 0 | Original P1 prereg (pre-correction) |
| `handover/preregistration/T2_SHARED_STATE_PREREG_2026-06-01.json` (+`.sha256`) | 71 | Class 0 | T2 confound-free shared-state prereg (supersedes T2_COUNTED_SWEEP) |
| `handover/preregistration/T2_MICROSTRUCTURE_SPEC_PREREG_2026-06-01.json` (+`.sha256`) | 57 | Class 0 | T2 5-pillar spec prereg read by `t2_microstructure_conformance.rs` |
| `handover/preregistration/T2_COUNTED_SWEEP_PREREG_2026-06-01.json` (+`.sha256`) | 60 | Class 0 | Superseded v1 T2 counted-sweep prereg (kept for provenance) |
| `handover/preregistration/TP3_SYBIL_DEFUNDING_PREREG_SKELETON_2026-06-01.json` | 56 | Class 0 | Forward-bound TP-3 Sybil-defunding skeleton (no sha lock yet) |
| `handover/audits/MARKET_FC_COMPLIANCE_2026-06-02.md` | 113 | Class 0 | LOAD-BEARING FC-compliance witness (COMPLIANT-WITH-GAPS) |
| `handover/planning/GROUP_2_MARKET_MISSION_DOCS` (EXECUTION_PLAN 132 + TASK_PACKAGE 311) | 443 | Class 0 | ADJACENT: market-mission planning |
| `handover/GROUP_T2_PRICE_ALLOCATION_LINE` (FINDINGS 85 + 3 preregs+sha + TP-3 skeleton) | 520 | Class 0 SHA-pinned | ADJACENT T2 alloc line |
| `handover/reports/GROUP_SEVEN_CORRECTION_BANNERS` (7 × 7-line banners) | 49 | Class 0 banner-only | RETRACTED: 7 reports each a banner; bodies pre-existed; keep intact |
| `handover/audits/GROUP_V3_TRIO_SUPERSEDED` (FINDINGS 110 + AUDIT 55 + PREREG+sha 78) | 243 | Class 0 SHA-pinned | SUPERSEDED v3 trio |

### 4.E evidence (committed derived views — SEPARATE evidence PR; raw tape stays local)

| path | +added | risk | purpose |
|------|:------:|:----:|---------|
| `handover/evidence/p1_v4_2026-06-02/` | 1465 | Class 1 | COMMITTED corrected 9-arm sweep: 732 manifest `.json` + 732 `.replay.json` + `STAGE_A_HARD_FLOOR_FINDINGS.md`. ~17 M derived views; raw `repo_*`/`cas_*` gitignored. **732 unstaged `run_p1v4_*.log` + 1 `CRACK_*.txt` inside — exclude.** |
| `handover/evidence/p1_axiom_confirm_2026-06-02/lm_ineq1_crack_VERIFIED.lean` | 1 | Class 0/1 | Axiom-clean existence proof (Layer 1 PASS) |
| `handover/evidence/P1_CONFOUNDED_SUPERSEDED_2026-06-02.md` | 1 | Class 0 | Annotation retiring v3/scaleup |
| `handover/audits/P1_REALVALUE_V4_AUDIT_2026-06-02.md` | 240 | Class 0 | Clean-context audit (§17 G5): NO-VIOLATION / PROCEED |
| `handover/audits/P1_EXTERNAL_AUDIT_VETO_AND_CORRECTION_2026-06-02.md` | 47 | Class 0 | External VETO-to-fix provenance |
| `handover/audits/GROUP_3_V3_AUDIT_INPUT_DOCS` (FULL_AUDIT_PACKAGE 1036 + ARM_CODE 384 + AUDITOR_PROMPT 127) | 1547 | Class 0 | HISTORICAL: drew the VETO; keep as provenance |
| `handover/reports/GROUP_EMERGE_STAGE2` (EMERGE FINDINGS 56 + cell JSONs 156/149) | 361 | Class 0 | ADJACENT EMERGE stage-2 evidence |
| `handover/evidence/p1_v4_laterday_full_2026-06-03/` | 0 (untracked) | local-only | G4 later-day stability re-run, 8.2 G, AWAITING-AUDIT — **KEEP LOCAL; gitignore `repo_*`/`cas_*` before any add** |
| `handover/evidence/{p1_scaleup, p1_realvalue_v3, p1_realvalue*_v2_2026-06-01, *smoke*, *driftfix*, *fixture*, *aborted*}` | 0 (untracked) | local-only | SUPERSEDED / smoke / scratch — **KEEP LOCAL / gitignore** |

### 4.F repo hygiene (rides with PR-B)

| path | +added | risk | purpose / attribution |
|------|:------:|:----:|---------|
| `.gitignore` | 6 | Class 0 | Ignores `p1_v4_2026-06-02/{repo_*,cas_*}` + `_smoke_` raw tape. **ATTRIBUTION: GENUINE P1 work** (commit `31faaeb0`). Rides with the evidence/runner PR. |

---

## 5. PR-split plan

**Branch is ~85 commits behind `origin/main` (advancing) and is a 49-commit v3+v4 history.** The handover docs recommend **NOT** shipping the raw 49-commit branch but cutting **clean PRs off latest `origin/main`** (rebase first). All PRs below are **Class 0/1/2; none touches §6 or the trust root; no §8 ratification is needed.**

### Rebase / pre-flight (do this once, before cutting any PR)

1. **Rebase** the P1-realvalue work onto current `origin/main` (81 behind). Re-cut PRs from the rebased state so the diff base is fresh.
2. **Confirm the §17 governance bundle (PR-E) is not double-merged** — see PR-E. Decide *before* PR-A/B, because PR-B's `lean_market_agent.rs` is pinned by the §17.3 gate.
3. **Exclude working-tree drift** that is NOT in the committed diff: `scripts/run_p1_v4_deterministic_fixture.py` (untracked), `handover/preregistration/P1_REALVALUE_V4_LATERDAY_FULL_RERUN_2026-06-03.json`(+`.sha256`) (untracked), and the **+2 uncommitted edit** to `script_liveness_inventory.toml` adding that fixture script. Stash or commit deliberately as a separate follow-up; a half-applied state would red the no-zombie gate.
4. **Extend `.gitignore` for ALL raw `repo_*`/`cas_*`** (at minimum the laterday + forward_minimatrix dirs) and gitignore the superseded/smoke set **before** any `git add` in the evidence PR (§2.8 warning).

### PR-E — governance-harness (§17 forensic gates) — **DECIDE FIRST, likely merge FIRST**

- **Files:** `AGENTS.md` (§17 + §14 line), `CLAUDE.md` (§5), `skills/no-proven-checklist.md`, **plus its mechanism files** `tests/constitution_headline_recompute_from_tape.rs`, `tests/constitution_router_name_matches_mechanism.rs`, `scripts/constitution_gates.manifest.toml`, `tests/agents_md_keep_anchors.rs` (§17 anchor), `tests/constitution_matrix_drift.rs` (allowlist grandfather).
- **Risk:** LOW (additive Class 0 docs + can-fail Class 2 gates). Mind the **allowlist cap = 69 = `K23_SHIP_ALLOWLIST_SIZE`** footgun — call it out in the PR body for the next gate-adder.
- **Merge order:** FIRST (P1's NO-GO + "directional, non-significant" scoping reads as the §17 gates working as intended).
- **Dependencies:** depends only on already-merged `src/market_tape_shared.rs` + `src/sdk/actor.rs` *on the rebased base*. The two mechanism gates `#[path]`-include `market_tape_shared.rs`, so on a fresh rebase they need that module — if PR-A lands first the dependency is satisfied; otherwise the gates compile against the module PR-A introduces. **Practical sequencing: if you keep PR-E's gates in PR-E, land PR-A's `market_tape_shared.rs` first OR move the two mechanism gates into PR-A's PR; either way the gate + module must co-exist.**
- **⚠ MERGE-ORDER HAZARD — the headline risk of the whole split:** AGENTS.md / CLAUDE.md / `skills/no-proven-checklist.md` are **100% from commit `4ccd2602` = already-MERGED PR #225** (head `claude/harness-forensic-gates`, merged into `claude/emerge-stage2`). **Do NOT re-author these as new P1 work.** First check whether PR #225 (or `claude/emerge-stage2`) is independently being promoted to `origin/main`. If it is → **rebase the §17 commits OUT of this branch** to avoid a double-merge / duplicate-commit conflict. If PR #225 is stranded on a dead base → this PR-E *is* the canonical path to mainline for §17 and should go as its own PR.

### PR-A — core-runtime (verifier soundness + shared substrate) — merge after PR-E

- **Files:** `src/judges/lean_judge.rs` (the `#print axioms` gate — load-bearing), `src/market_tape_shared.rs` (new `#[path]` module), `src/bin/verify_market_tape.rs` (new replay verifier), `src/runtime/librarian_broadcast.rs` (fail-closed ignore), `src/state/price_index.rs` (additive epsilon floor).
- **Risk:** LOW but **highest-scrutiny** — `lean_judge` changes what counts as "Verified." Mitigations (all in-PR): fail-closed; does NOT extend CAS-bearing `LeanVerdictKind`; pure-parse + real-Lean tests; integer money; no §6/trust-root.
- **Merge order:** SECOND. `market_tape_shared` + `verify_market_tape` are a **producer/verifier pair — land together.** `librarian_broadcast`'s `lm.proof_artifact.v1` ignore is logically coupled to PR-B writing that schema but is harmless standalone (fail-closed reader hardening), so it sits naturally here.
- **Dependencies:** none new (this is the dependency root for PR-B and the §17 mechanism gates).

### PR-B — benchmark-scaffolding (P1 experiment harness + analysis) — merge after PR-A

- **Files:** `src/bin/lean_market_agent.rs` (the v4-corrected experiment), `src/bin/lean_hayek_market.rs` (TP-0A shared-tape + `run_alloc_shared` + flatbid/coordinator fixes), `src/bin/lean_hetero_market.rs` + `src/bin/lean_tree_market.rs` (doc-only name-lie fixes), the **3 runners** + **5 analyzers**, the **2 conformance gates** (`market_tape_canonical_roundtrip.rs`, `t2_microstructure_conformance.rs`), the **2 liveness fixtures** (`script_liveness_inventory.toml`, `production_module_liveness.toml` — they classify *this* PR's runners/analyzers, so they belong here), and **`.gitignore`** (+6).
- **Risk:** LOW (experiment harnesses; a red here cannot affect production runtime).
- **Dependencies:** `lean_market_agent.rs` imports `lean_judge::AXIOM_WHITELIST` + `LeanOutcome.axioms/axiom_rejected` and writes `lm.proof_artifact.v1` / `lm-lean-result` sidecars; `lean_hayek_market.rs` depends on `market_tape_shared` — **hence PR-A must merge first.**

### PR-C — reports / plans / audits (Class 0 docs) — merge after the evidence is committable

- **Files:** the ~30 Class-0 handover docs in §4.D + the audit docs in §4.E that are reports (`P1_REALVALUE_V4_AUDIT`, `P1_EXTERNAL_AUDIT_VETO_AND_CORRECTION`, `MARKET_FC_COMPLIANCE`, `SESSION_*`, `LATEST.md`, the prereg JSONs + `.sha256` locks, the GROUP_* doc aggregates).
- **Risk:** NONE (Class 0; no gate impact). **Keep the 7 correction banners + the kept-unmutated T2 v1 prereg intact** (annotate-not-mutate, §8).
- **Merge order:** LAST, pointing at the now-committed evidence path. (Prereg JSON + `.sha256` may ride earlier with PR-B if you want the analyzers' prereg target present at PR-B merge — but keeping them in PR-C is clean.)

### PR-D — evidence (committed derived views) — merge after PR-B, before PR-C verdict cites it

- **Files:** the 3 committed P1 adds — `handover/evidence/p1_v4_2026-06-02/` (732 manifests + 732 replay + findings), `p1_axiom_confirm_2026-06-02/lm_ineq1_crack_VERIFIED.lean`, `P1_CONFOUNDED_SUPERSEDED_2026-06-02.md`. ~17 M, derived views only.
- **Risk:** Class 0/1 (additive, immutable, no §6/trust-root) — but **load-bearing for a §17 NO-GO claim**, so its merge gate is G1 (recompute-from-tape: `derive_* == manifest` byte-equal) + G5 (the clean-context audit referencing prereg SHA `d80ba8cd`).
- **Critical hygiene before pushing:** scrub the 732 unstaged `run_p1v4_*.log` + `CRACK_*.txt` inside the committed dir (must NOT be in the index); extend `.gitignore` so the 8.7 G untracked raw tape (esp. `p1_v4_laterday_full_2026-06-03` 8.2 G) never auto-stages. **Recommendation: a SEPARATE evidence PR (or a dedicated evidence branch), NEVER folded into the code PR.** If the G4 laterday re-run is later cited, commit ONLY its derived views (same derived-only pattern), repo_*/cas_* gitignored.

### Recommended merge order (one line)

**PR-E (if §17 not already promoted via PR #225) → PR-A (core) → PR-B (harness) → PR-D (evidence) → PR-C (verdict/reports).** Rationale: governance first so the gates exist and P1's scoping reads as them working; core soundness before the bin that depends on it; harness before the evidence it produced; evidence before the reports that cite its committed path.

---

## 6. Recommendations + open decisions

### The honest claim to adopt

> **P1 price-routing is a NO-GO.** In this regime — price as a ROUTER of a homogeneous-agent, abundant-budget Lean proof-tree search — a loss-bearing price market does **not** beat its controls; adding the real price signal is the biggest negative delta (shuffled→market −3.8pp). The lever that moves solve-rate is **non-local tree search** (single 0% → single_tree_no_price 5.1%), and it needs neither price nor a swarm: a single agent revisiting its own tree is the cheapest near-best policy. The lever is **sampling + non-locality**, not the constitution's price mechanism. The finding is **directional, NOT statistically significant** (max 6/78; every McNemar `p_holm = 1.0`; underpowered). No PROVEN/causal language.

### Scope caveat (the steelman — do NOT over-read the NO-GO)

This tests price as a **router** of a homogeneous-agent abundant-budget search. It does **NOT** refute price **ALLOCATING a scarce shared budget across complementary specialists** (LEAN-ALLOC / PROBE-ALLOC — a different, untested-here mechanism). The model is strong (`deepseek-v4-pro` solves many `lm_` alone; 6/13 hard ones are beyond *every* arm), so the test is not trivial, but it is also not the scarce-allocation regime where Hayekian price has its strongest theoretical claim.

### What the auditor should re-verify

1. **G1 recompute-from-tape** on the committed `p1_v4_2026-06-02/`: `derive_* == manifest` byte-equal (the clean-context audit already did this; re-run the gate `constitution_headline_recompute_from_tape` → expect 6/0).
2. **The prereg SHA-lock** `d80ba8cd` / `f1a36f16…43f5` byte-exact (verified twice already).
3. **The 4 recompiled cracks** under real Lean 4.24.0 + Mathlib axiom-clean (auditor did `autonomous lm_deriv1` / `single lm_ineq1` / `market lm_fact` / control `no_price lm_ineq2`).
4. **`lean_judge` axiom gate is fail-closed** and does NOT extend `LeanVerdictKind` (CAS `assert_45` consistency).
5. **No §6 / trust-root touch** — empty diff against all six §6 surfaces + `lib.rs` + `runtime/mod.rs` (claimed; re-confirm post-rebase).
6. **The §17 governance double-merge question** (PR-E) — is PR #225 / `claude/emerge-stage2` reaching `origin/main` independently?

### Next-line options (architect's call)

- **(A) Power up non-locality to significance** on the crackable band (`lm_ineq2` / `lm_ineq3` / `lm_natdeg_pow`): more seeds, focused on `single_tree_no_price` vs `single` — the directional lever, to convert it to a significant result.
- **(B) Genuinely harder theorems** where 24 proposals is not abundant (the 6 nobody-cracked ones suggest the band exists), to test whether organization pays off when budget actually binds.
- **(C) Price × scarce-budget allocation** — the steelman: build LEAN-ALLOC / PROBE-ALLOC where price allocates a *scarce shared* budget across *complementary specialists* (the regime this NO-GO does NOT cover). The T2 shared-state prereg + `run_alloc_shared` infrastructure already exists for this.
- **(D) Accept the NO-GO and advance Mission Stage-2.**

### Deferred gaps

- **L5** (FC2 `mr-tick` not JSON-re-derivable) — **DEFERRED**; closing it would touch `runtime/mod.rs` (Class-4 trust root) → its own dedicated atom with §8 ratification.
- **Info6** (FC3 meta-loop by-design absent) — covered by `tests/constitution_fc3_closure.rs`; no action.
- **M1 / L3** (axiom-reject / SorryBlocked dormant in the clean corpus) — both real-Lean unit-tested and now tape-visible via `reject_class`; a red-team `native_decide` task would light M1 in production.

### DECISION CHECKLIST (architect)

1. **Adopt the NO-GO claim as written above?** (Y / N — edits)
2. **Cut the 5-PR split (E→A→B→D→C) off a fresh rebase onto `origin/main` (81 behind)?** (Y / N)
3. **§17 governance (PR-E):** is PR #225 reaching mainline independently? If YES → **rebase §17 out of this branch.** If NO → keep PR-E as the canonical §17 path. (decide)
4. **Evidence (PR-D):** separate evidence PR (or dedicated branch)? Confirm the `.gitignore` extension + scrub of the 732 unstaged logs before any `git add`. (Y / N)
5. **Working-tree drift** (`run_p1_v4_deterministic_fixture.py`, `LATERDAY_FULL_RERUN` prereg, the +2 liveness edit): commit as a deliberate follow-up or stash? (decide)
6. **Next line:** A (power-up non-locality) / B (harder theorems) / C (price × scarce allocation — the steelman) / D (Stage-2)? (pick)
7. **L5 trust-root atom:** schedule now or defer? (decide)

---

*Assembled 2026-06-02 (this summary written 2026-06-05). Ground truth: the 6 slice surveys + the six cited docs (`SESSION_P1V4_STAGE_SUMMARY`, `P1_REALVALUE_V4_FINDINGS`, `P1_REALVALUE_V4_AUDIT`, `MARKET_FC_COMPLIANCE`, `P1_EXTERNAL_AUDIT_VETO_AND_CORRECTION`, `P1_REALVALUE_V4_CORRECTED_PREREG`). All numbers cited, none invented. No §6 surface or Class-4 trust-root touched anywhere in the branch.*
