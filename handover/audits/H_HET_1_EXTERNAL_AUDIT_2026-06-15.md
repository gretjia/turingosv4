# H-HET-1 External Audit Dossier (self-contained)

**Date:** 2026-06-15. **Author:** Claude (autonomous research session).
**Audience:** an external auditor with NO access to the code repository. Every
load-bearing claim below is accompanied by (a) the real result data and (b) the
actual source code, pasted verbatim inline. You should be able to check the
logic end-to-end from this file alone.

**Claim boundary (read first):** every empirical result here is **DIRECTIONAL**,
not "proven." Sample sizes are small (K=3 seeds), the theorem set is narrow (one
family), and one provenance detail (which model produced a given proof) is an
*inference* from a deterministic assignment rule rather than a value written on
the immutable log (this is itself one of the recommendations). No causal or
"X is proven better than Y" claim is made. The strongest honest statement is a
*null with a mechanism*: on this band, heterogeneity bought nothing over the
best single model, and we explain why.

---

## 0. What this project is (one screen)

TuringOS is a "tape-first constitutional" substrate for running LLM agents. Two
ideas matter for this audit:

1. **Everything that happens is written to an append-only, content-addressed log
   ("ChainTape" + "CAS") and must be reconstructable from it by a deterministic
   replayer.** A run is only valid if an independent `verify_chaintape` pass can
   rebuild the economic/ledger state from the frozen log. This is the project's
   anti-fabrication control: a result you cannot replay does not count.

2. **The architecture's central bet ("anti-Oreo"):** instead of one big
   reasoning model, use many *heterogeneous, cheap, non-reasoning* proposer
   models (different vendors), coordinated by a transparent top layer that only
   manages *signals* (prices, broadcast of abstracted failures) and never
   micro-manages. The thesis: a *market* of heterogeneous agents produces
   capability *emergence* beyond any single model, because different models fail
   in *decorrelated* ways (one solves what another can't).

**H-HET-1** is the empirical test of that bet, in the domain of Lean 4 theorem
proving: *Can a heterogeneous cross-vendor agent market solve theorems that a
homogeneous single-vendor setup cannot?*

The four vendor models under test (the "roster"):
`deepseek-ai/DeepSeek-V4-Pro`, `Qwen/Qwen3-32B`, `zai-org/GLM-4.5-Air`,
`Qwen/Qwen3.5-397B-A17B`. All run **non-thinking** (reasoning disabled), capped
at 2048 output tokens — a deliberate, controlled regime (the log IS the
chain-of-thought, so internal reasoning is treated as redundant).

---

## 1. Integrity controls in force (what makes the numbers trustworthy)

These are the mechanical guards an auditor should know are active:

- **Real kernel verification.** A proof "Verified" only if the Lean 4 kernel
  accepts it. No self-report.
- **Axiom whitelist.** A Verified proof must use only
  `{Classical.choice, Quot.sound, propext}` — the standard sound Lean axioms.
  This blocks `native_decide`-style false-greens (which would pull
  `Lean.ofReduceBool`). Every Verified node records its `#print axioms` set.
- **Replay-recompute gate.** Each experiment cell is re-verified by
  `verify_chaintape`, which rebuilds the economic state from the log; a cell
  whose `economic_state_reconstructed != true` is excluded, not counted.
- **Preregistration.** Any paid multi-vendor run is preceded by a frozen
  (sha256-pinned) prereg fixing hypothesis, metric, budget, stopping rule, and
  exclusion rule *before* data is seen (anti-p-hacking).
- **Adversarial clean-context audit.** Findings are checked by independent
  agents that did not see the implementation, are told to *refute* the headline,
  and must recompute the numbers themselves (Section 9).

---

## 2. Experiment 1 — Rule out the harness before trusting any null

**Logic.** Before claiming "model X can't solve theorem T," you must rule out
that your *extraction/verification harness* is silently corrupting correct
proofs. A prior session found exactly such a bug: model outputs whose proof body
was indented inconsistently ("de-aligned") were mangled before reaching Lean,
producing false negatives.

**Method.** The proof-body extractor was hardened with a `realign` step. The key
soundness property: it only *re-flushes flat indentation*; it never *restructures*
a genuinely nested proof (those are deferred to the original conservative
`dedent`). So it can only ever turn a false-negative into a true result — it can
never manufacture a false positive (the Lean kernel still judges the real goal).

**The actual code (verbatim):**

```rust
// src/judges/lean_judge.rs
pub fn realign(body: &str) -> String {
    let expanded = body.replace('\t', "  ");
    if opens_nested_block(&expanded) {
        return dedent(&expanded);          // genuinely nested -> conservative path, unchanged
    }
    let lines: Vec<&str> = expanded.lines().map(str::trim).collect();
    let start = lines.iter().position(|l| !l.is_empty()).unwrap_or(0);
    let end = lines.iter().rposition(|l| !l.is_empty()).map_or(0, |i| i + 1);
    if start >= end {
        return String::new();
    }
    lines[start..end].join("\n")          // flat body -> flush to col 0; cannot restructure
}
```

**Result.** A controlled re-verification (the "Q2" probe) took 110 recoverable
historical failures that the *old* extractor would have mangled, and re-ran them
through real Lean with the fix: **flip rate = 0/110** — none became Verified. A
bound positive control (known-good proofs deliberately de-aligned) flipped 3/3,
proving the test has discriminating power (so 0/110 is a real null, not a dead
test). Conclusion: the historical near-zero solve rates were *not* a de-align
artifact — the proofs were genuinely wrong. **The harness was cleared before any
capability claim was made.** This is the "rule out the harness bug first"
discipline, and it matters because the eventual finding is itself a null.

---

## 3. Experiment 2 — Select a difficulty band from already-paid data ("Goldilocks")

**Logic.** To test "het solves what homogeneous can't," you need theorems in a
*Goldilocks band*: hard enough that the baseline model fails, easy enough that
*some* model succeeds. Too-hard theorems produce all-null (no signal); too-easy
produce all-solve (no contrast).

**Method.** Re-analyze (no new spend) existing single-shot probe data: 134 + 84
records covering 19 theorems × the 4 vendors, each at K=3 attempts, same
non-thinking regime. For each (theorem, model) count Verified. Goldilocks =
*DeepSeek fails 0/3 AND some other vendor succeeds ≥1/3*.

**Result (real data, probe = 1 proposal per attempt):**

```
theorem        DS    GLM   Q32   Q397   classification
lm_det_mul     0/3   0/3   1/3   3/3    GOLDILOCKS (DS fails, cross-lab solves)
lm_det_2x2     0/3   0/3   0/3   1/3    GOLDILOCKS
lm_det_zero    0/3   1/3   0/3   0/3    GOLDILOCKS
lm_det_3x3     1/3   0/3   0/3   0/3    DS-solves (control)
lm_geom_eval   1/3   0/3   0/3   3/3    DS-solves (control)
... 12 more theorems: all-null (0/3 across all 4) -> excluded as out-of-band
```

**Soundness check (real data).** Every Goldilocks "Verified" was axiom-clean:

```
lm_det_2x2  Qwen3.5-397B  verified=Y  axioms=[Classical.choice, Quot.sound, propext]
lm_det_mul  Qwen3-32B     verified=Y  axioms=[Classical.choice, Quot.sound, propext]
lm_det_mul  Qwen3.5-397B  verified=Y  axioms=[Classical.choice, Quot.sound, propext]  (×3)
lm_det_zero GLM-4.5-Air   verified=Y  axioms=[Classical.choice, Quot.sound, propext]
```

So at the *single-shot probe* level, the H-HET-1 phenomenon *appears* real and
sound: different cross-lab models solve det-family theorems DeepSeek fails, and
no single vendor covers everything. **This appearance is the hypothesis that
Experiment 4 then tests properly — and substantially overturns.**

---

## 4. The carrier (the real test vehicle) — what it is, with its core code

A single-shot probe is *not* the architecture. The real test vehicle is the
**autonomous market carrier**: 4 agents, each bound to a vendor model, propose
Lean proofs over several rounds; the market prices open proof-nodes; each agent
*self-selects* its action from a 2-item menu (no role is assigned). Four pieces
of its actual code matter for this audit:

**(a) Heterogeneity is realized by round-robin model assignment** (this is what
makes the "HET" arm genuinely multi-vendor and the "homo" arms single-vendor):

```rust
// src/bin/lean_market_agent.rs  (n_agents = 4)
let agent_models: Vec<String> = (0..n_agents)
    .map(|i| args.models[i % args.models.len()].clone())
    .collect();
// ... at every proposal call:
model: agent_models[ai].clone(),
```
With `--models` = 4 distinct vendors → 4 distinct agents (HET). With `--models`
= one vendor → all 4 agents that vendor (homogeneous control, *same budget*).

**(b) The Hayekian self-selection prompt** (agents are shown only signals — no
role, only prices + abstracted failure classes — and choose for themselves):

```text
You are an autonomous agent in a Lean 4 proof-search MARKET (Mathlib is available).
No role has been assigned to you. You are shown ONLY market signals — you decide for
yourself what to do. CHOOSE ONE of exactly two actions:
 - "solve": you believe a goal/node is provable — propose a Lean proof, take the LONG (YES) side.
 - "short": you believe an existing node's attempt will FAIL the kernel — take the SHORT (NO) side.
Decide from the prices and the collective failure memory below. Be selective — do not
all crowd onto the single highest-priced node ...
```

**(c) The decision parser, with honesty instrumentation** (so a malformed reply
forced to a default "solve" is not silently counted as a genuine free choice):

```rust
// src/bin/lean_market_agent.rs
struct AutonomousChoice { action: String, target: i64, confidence: f64,
    decision_source: &'static str }   // "agent" | "parse_fallback" | "llm_error"

fn parse_autonomous_choice(content: &str) -> AutonomousChoice {
    let v = match extract_json_object(content) {
        Some(v) => v,
        None => return AutonomousChoice { action:"solve".into(), target:-1,
                    confidence:0.6, decision_source:"parse_fallback" },
    };
    let valid_action = v.get("action").and_then(|x| x.as_str())
        .map(|s| s.to_ascii_lowercase()).filter(|s| s=="solve" || s=="short");
    let decision_source = if valid_action.is_some() { "agent" } else { "parse_fallback" };
    let action = valid_action.unwrap_or_else(|| "solve".into());
    // ... target, confidence ...
    AutonomousChoice { action, target, confidence, decision_source }
}
```
This `decision_source` is written to each node so any solve-rate metric can
exclude forced solves. (In the pilot, ~8% of decisions were `parse_fallback`.)

**(d) PPUT — the canonical token-economics metric** (golden-path tokens, i.e.
the tokens on the *winning* proof's ancestor chain, per wall-clock second; 0 if
unsolved):

```rust
// src/bin/lean_market_agent.rs
let pput = if omega_node.is_none() || wall_clock_s <= 0.0 {
    0.0
} else {
    golden_path_tokens as f64 / wall_clock_s
};
```
The project's reporting standard for PPUT is: ΣPPUT + Mean-PPUT(solved) + a
Wilson 95% confidence interval on the solve rate.

**Cost accounting** (used for the economics, integer micro-USD, vendor rates):

```rust
// src/market_tape_shared.rs   (rates are micro-USD per million tokens, in/out)
pub const MODEL_RATES: &[(&str, i64, i64)] = &[
    ("Qwen/Qwen3-32B",      140_000,   570_000),
    ("GLM-4.5-Air",         140_000,   860_000),
    ("Qwen3.5-397B-A17B",   390_000, 2_340_000),
    ("deepseek-v4-pro",     435_000,   870_000),
    // ...
];
pub fn call_micro_usd(model: &str, prompt_tok: u64, completion_tok: u64) -> i64 {
    let model_lc = model.to_ascii_lowercase();           // case-insensitive match (a real bug fix)
    let mut rate = (FALLBACK_IN_UPMT, FALLBACK_OUT_UPMT);
    for &(id, in_upmt, out_upmt) in MODEL_RATES {
        if model_lc.contains(&id.to_ascii_lowercase()) { rate = (in_upmt, out_upmt); break; }
    }
    (prompt_tok as i64 * rate.0 + completion_tok as i64 * rate.1) / 1_000_000
}
```

---

## 5. Experiment 3 — Carrier mechanism smoke (does the vehicle even work?)

**Logic / discipline.** The carrier's autonomous-market path had never been run
end-to-end. Before spending on a sweep, run one cheap cell and confirm the
mechanism couples to reality (agents really self-select, the choice lands on the
log, the market settles, the log replays).

**Result (real stdout, one cell, `lm_det_mul`, 4 vendors, 4 agents × 2 rounds):**

```
lean_market[autonomous_market] problem=lm_det_mul agents=4 rounds=2
  proposal_llm=4 route_llm=4 parse_fail=0 verified=1 failed=3 nodes=4
  omega=true ttfp=88.5s total_tokens=5721 pput=4.98
```
Per-node (real): `Agent_3 (Qwen3.5-397B) = Verified, axiom-clean`;
`Agent_0 (DeepSeek-V4-Pro) = SorryBlocked`; `chosen_action=solve, action_source=agent`
on every node (genuine self-selection, no fallbacks). The independent replay
returned `economic_state_reconstructed = true`, ledger root + signatures
verified, CAS payloads retrievable. **Mechanism validated and sound.** Note the
*appearance* again: a cross-lab model solved what DeepSeek did not.

---

## 6. Experiment 4 — The 3-arm equal-budget pilot (the decisive test)

**Logic.** The naive H-HET-1 framing ("het solves what *DeepSeek-solo* can't")
has an obvious confound: in the probe, DeepSeek got **1 proposal**; in a market
it gets **many**. And a het win could simply mean "one strong model (Q397) is in
the roster," not "heterogeneity helps." So the honest design needs three arms at
*equal budget*:

- **HET** — 4 distinct vendors.
- **DSHOMO** — 4× DeepSeek (the "homogeneous baseline").
- **Q397HOMO** — 4× Qwen3.5-397B (the *best single model* control — this is the
  arm that isolates "heterogeneity" from "a strong model exists").

All arms: `--policy autonomous_market`, 4 agents × 3 rounds = **12 proposals per
cell**, on the det-family band, K=3 seeds. 5 theorems × 3 arms × 3 seeds = 45
cells. Prereg frozen before the run (sha256 `621de565…`). Every cell
replay-gated.

**Result (real, 45/45 cells replay-clean, all Verified nodes axiom-clean):**

```
theorem        HET            DSHOMO     Q397HOMO
lm_det_mul     3/3 (Q397)     3/3 (DS)   3/3 (Q397)
lm_det_2x2     3/3 (Q397)     0/3        3/3 (Q397)
lm_det_zero    1/3 (Q397)     1/3 (DS)   0/3
lm_det_3x3     0/3            1/3 (DS)   0/3
lm_geom_eval   3/3 (Q32,Q397) 2/3 (DS)   3/3 (Q397)
-------------------------------------------------
SOLVED-TOTAL   10             7          9
```

Per-model contribution *inside the HET arm* (which vendor actually produced each
proof, via the fixed round-robin map):

```
DeepSeek-V4-Pro :  0 verified / 28 proposals
Qwen3-32B       :  2 verified / 26 proposals
GLM-4.5-Air     :  0 verified / 19 proposals
Qwen3.5-397B    :  8 verified / 26 proposals
  => of HET's 10 solves: 8 = Q397, 2 = Q32; DeepSeek and GLM contributed ZERO.
```

**Four honest readings (each adversarially confirmed in Section 9):**

1. **The probe "Goldilocks" was largely a budget artifact.** `lm_det_mul`: DS was
   0/3 at 1 proposal, but **DSHOMO is 3/3 at 12 proposals**. Give DeepSeek a
   market's worth of attempts and it solves what looked unsolvable.

2. **Heterogeneity ≈ best single model.** HET 10 vs Q397HOMO 9 is a *one-cell*
   margin (and that one cell, `lm_det_zero` seed 3, was won by HET's *own Q397
   slot* — within-Q397 variance, not cross-vendor decorrelation). The solve-rate
   confidence intervals fully overlap (Section 8). Heterogeneity did **not**
   beat the best single model.

3. **Half the heterogeneous roster was dead weight.** The two models that make
   the roster "heterogeneous" beyond Q397 — DeepSeek (0/28) and GLM (0/19) —
   verified nothing on this band. "Heterogeneity" here was effectively
   "Q397 + noise," and the noise cost budget.

4. **But there IS a real, latent positive signal: complementary coverage.** At
   the *homogeneous* level, DSHOMO uniquely solves `{lm_det_zero, lm_det_3x3}`
   (Q397HOMO gets 0/3 on both); Q397HOMO uniquely solves `lm_det_2x2` (DSHOMO
   0/3). **No single model covers all five.** Yet the round-robin carrier
   *cannot exploit* this — it even *lost* `lm_det_3x3`, which DeepSeek-solo
   solves, because spreading the fixed budget across 4 vendors starved DeepSeek
   of attempts. The architecture's promised mechanism (capital flows to the
   winning model) is exactly the thing that is **not built**: the market routes
   *which node* to extend, never *which model* gets more budget.

---

## 7. Experiment 5 — PPUT economics (the project's canonical efficiency metric)

**Logic.** Solve-count is not the economic question; the project benchmarks
token-economics via PPUT (Section 4d). Re-measure the arms on PPUT + the §17
report standard (ΣPPUT, Mean-PPUT(solved), Wilson 95% CI on solve rate).

**Result (real, from the 45-cell pilot manifests):**

```
arm        solve  Wilson95%CI   ΣPPUT   Mean-PPUT(solved)  golden_path_tok(solved)
HET        10/15  [0.42,0.85]    78.0    7.80               508
DSHOMO      7/15  [0.25,0.70]    63.6    9.09               415
Q397HOMO    9/15  [0.36,0.80]   164.9   18.32               324
```

Readings: (i) the three Wilson CIs **heavily overlap** — at 15 cells/arm the
solve-rate differences are statistically indistinguishable; "HET 10 vs Q397 9"
is noise. (ii) On PPUT, **Q397HOMO dominates**: ΣPPUT ~2× HET, Mean-PPUT ~2.3×
HET. (iii) The *concurrency-independent* metric, golden-path tokens, agrees:
Q397HOMO's winning proofs are leanest (324 vs HET's 508 — HET's winning proofs
are ~57% more tokens). So heterogeneity is not merely "not better" — on this
band it is **economically dominated** by the best single model.

**Measurement caveat (disclosed).** The 45-cell pilot ran 4 cells concurrently,
which inflates `wall_clock_s` and therefore contaminates the *time* component of
PPUT. A clean **serial (1-at-a-time) re-run** was then run to give the canonical PPUT
number. Result (uncontended wall-clock, 45/45 replay-clean):

```
arm        solve  Wilson95%CI   ΣPPUT   Mean-PPUT(solved)  gp_tok(solved)  wall_s(solved)
HET        9/15   [0.36,0.80]    81.4    9.04               422             194.3
DSHOMO     6/15   [0.20,0.64]    66.2   11.03               546              54.9
Q397HOMO   9/15   [0.36,0.80]   155.5   17.27               364              23.6
```

The clean measurement **strengthens** the conclusion: Q397-homo dominates PPUT
by ~1.9× (ΣPPUT 155.5 vs 81.4; Mean-PPUT 17.27 vs 9.04), has the leanest golden
path (364 tokens), and reaches a proof in **23.6s vs HET's 194.3s (~8× faster)**
— the het market dithers across dead-weight models (DeepSeek, GLM) before its
Q397 slot closes the proof. Note HET's solve count moved 10→9 between the
concurrent and serial runs: that one-cell swing *is* the K=3 sampling noise made
visible, and is why the headline rests on "indistinguishable" (overlapping CIs),
not on the raw count. Conclusion unchanged and reinforced: on this regime,
heterogeneity is economically dominated by the best single model.

---

## 8. Statistics note

K=3 seeds per (theorem, arm) → 15 cells per arm. Wilson 95% intervals (Section 7)
are the honest precision: every pairwise arm comparison's intervals overlap, so
**no arm is statistically distinguishable from another on solve rate.** This is
*why* the conclusion is stated as "heterogeneity bought nothing detectable," not
"homogeneity is better." Distinguishing them would require the larger,
seed-paired design recommended in Section 11.

---

## 9. Experiment 6 — Adversarial clean-context audit of the finding

**Method.** Four independent agents, none with the implementation transcript,
each told to *refute* the headline from a distinct lens and to *recompute every
number itself* from the raw cell files; then a synthesizer. Legal verdicts:
`NO-VIOLATION | VIOLATION-FOUND | RECONSTRUCTION-FAILURE | ANALYSIS-ERROR`.

**Result: all four lenses + synthesis = NO-VIOLATION; headline survives; zero
must-fix.** Representative *independently-recomputed* confirmations (auditor's
own words, paraphrased to the load-bearing numbers):

- **Analysis-correctness lens:** recomputed from the 45 manifests without the
  analysis script — "HET=10, DSHOMO=7, Q397HOMO=9, byte-identical to the
  headline; replay 45/45 clean; HET per-model verified DS 0/28, Q32 2/26,
  GLM 0/19, Q397 8/26."
- **Heterogeneity-refutation lens:** "The entire +1 HET edge is ONE cell
  (`lm_det_zero` s3) won by HET's Q397 slot … within-Q397 sampling variance, NOT
  decorrelation. DeepSeek and GLM verified zero. No reading makes HET more
  efficient."
- **Harness/confound lens:** "HET rosters carry 4 distinct vendors (entropy
  1.0–2.0 bits, no silent fallback); homo arms single-vendor; equal budget held
  (proposal calls capped at 12, zero overruns); non-thinking bound (avg
  completion 135.5 tokens/call ≪ 2048). Arms are genuinely distinct."
- **Soundness/replay lens:** re-ran the replayer on sampled cells (all
  `economic_state_reconstructed=true`), and confirmed every Verified node's
  axioms ⊆ the whitelist (0 `native_decide`).
- **Synthesizer core finding:** *"Heterogeneity bought nothing over the best
  single model on this band … the genuinely true residual signal is cross-arm
  complementary coverage that an equal-budget round-robin carrier provably fails
  to exploit."*

One internal disagreement, adjudicated: the refutation lens claimed the token
figures didn't reproduce; the synthesizer showed they do (total_tokens
152979/10 = 15298; 99350/9 = 11039), the lens had used the wrong field. **No
correction to the headline was required.**

---

## 10. Conclusion (honest verdict)

On the det-family Goldilocks band, at equal carrier budget, **the heterogeneous
agent market did not outperform the single best model (Qwen3.5-397B), on solve
rate (indistinguishable) or on token-economics (it was dominated).** The
architecture's central bet — heterogeneity → emergent capability beyond any
single model — is **not supported by this pilot.**

Crucially, this is a null *with a diagnosed cause*, not a dead end. The one real
pro-heterogeneity signal — **complementary coverage** (different models uniquely
solve different theorems; no single model covers all) — genuinely exists, but
the current carrier **cannot convert it into a win** because it allocates a
*fixed* budget per model (round-robin) instead of *routing budget to whichever
model is succeeding*. The market, as built, does not actually market the
resource that matters (model choice). So the thesis is **not yet refuted** — it
is **untested by a carrier capable of expressing it.**

**Regime scope (added after review — the null is narrower than it first looks).**
Measured directly, the pilot's solved proofs have a **golden-path depth of mean
1.1, max 2** — i.e. essentially *one-shot* theorems — at a budget of **3
proposals per agent** (4 agents × 3 rounds = 12). A separate, much larger market
run (90 agents × 6000 transactions, ~19 tx/agent) shows a *weak* model reaching a
hard proof via an **18-step** collaborative chain, and establishes a budget
scaling law: deep proof search needs `tx ≥ agents × 20`; at ~3 tx/agent depth
collapses (~5) and proofs fail. The H-HET-1 pilot therefore sits squarely in the
**budget-starved, shallow-theorem regime — exactly where no collaborative or
market mechanism can add value, because a single strong model one-shots the
target.** So the honest scope of the null is: *"on one-shot theorems at 3
tx/agent, a fixed-roster market does not beat the best single model."* It is
**not** evidence about the market in the deep-chain, adequate-budget regime where
collaboration is necessary — that regime is the subject of the follow-on
experiment.

---

## 11. Future options, recommendation, and reasoning

**Option A — Stop / report null.** Conclude heterogeneity doesn't help and move
on. *Rejected:* it discards the genuine complementary-coverage signal and tests
the thesis with a vehicle structurally unable to express it.

**Option B — Just add more budget / more seeds to the same round-robin carrier.**
*Rejected as the primary move:* more seeds tightens the CIs but cannot fix the
structural defect (fixed per-model budget); the audit shows the round-robin arm
wastes 28+19 proposals on models that solve nothing. You'd buy precision on the
wrong design.

**Option C (RECOMMENDED) — Build a dynamic model-budget market and test it
against the complementary-coverage seam.** Replace the fixed round-robin with a
priced/bandit carrier that *reallocates proposal budget toward whichever model
is verifying* (or whose price/historical-failure signal warrants it). Concretely:
concentrate budget on Q397 for `lm_det_2x2` and on DeepSeek for
`{lm_det_zero, lm_det_3x3}`. Prereg the success criterion as: *the dynamic
heterogeneous market beats the best single model on **union coverage** at
equal-or-lower total token budget* — i.e. it solves a theorem **no single model
solves alone within budget.** Power it properly: **K ≥ 12 seeds with
within-seed Wilcoxon pairing**, and a Goldilocks pool pre-selected as {some model
0/K AND another ≥1/K} so the budget actually binds and decorrelation can express.

**Why C.** It (1) turns the pilot's *only* real signal (complementary coverage)
into the treatment; (2) fixes the exact confound that doomed the static arm;
(3) is the *first* genuine test of the architecture's Hayekian thesis (capital →
the winning model), which the current carrier never implemented; (4) its required
instrumentation coincides with a compliance requirement (next section), so one
change pays two debts.

**Convergence worth noting.** Option C needs a true *per-proposal model field*
on the log (today, "which model produced this proof" is *inferred* from the
round-robin rule — sound here only because the map is a fixed 1:1, but an
inference nonetheless). That field is *also* required by the project's
tape-canonical rule (a result's model provenance must be reconstructable from the
immutable log, not a side file). So the same schema change closes a constitutional
gap **and** instruments the next experiment.

---

## 12. The one governance decision this raises (for completeness)

The per-proposal model field touches a *binding* log schema that the project
freezes behind explicit human ratification (it is content-addressed; adding a
field changes the encoding, so it must be version-bumped with a legacy-decoder to
preserve historical replay). The schema struct under discussion:

```rust
// src/runtime/proposal_telemetry.rs  — "do NOT add fields without architect ratification"
pub struct ProposalTelemetry {
    pub agent_id: AgentId,
    pub prompt_context_hash: Hash,
    pub proposal_artifact_cid: Cid,
    pub candidate_tactic: String,
    pub token_counts: TokenCounts,
    pub tool_calls: Vec<ToolCallRecord>,
    pub branch_id: String,
    pub parent_tx: Option<TxId>,
    #[serde(default)] pub verification_result_cid: Option<Cid>,   // a prior ratified additive field
    // PROPOSED (ratification pending): #[serde(default)] pub model_id: Option<String>
}
```
The proposed change mirrors the existing ratified `Option` precedent, plus a
schema version bump (v1→v2) with a v1 fallback decoder so historical logs still
replay. It is held pending sign-off — i.e. the project did **not** let an
autonomous agent silently mutate a frozen schema, which is itself an integrity
property an auditor should note.

---

## 13. Reproducibility pointers (paths inside the repo; for the architect, not the external auditor)

- Pilot cells + replays: `handover/evidence/het_carrier_pilot_2026-06-15/`
- Serial PPUT: `handover/evidence/het_carrier_pput_serial_2026-06-15/`
- Smoke: `handover/evidence/het_carrier_smoke_2026-06-15/`
- Prereg (sha256-pinned): `handover/preregistration/H_HET_1_CARRIER_PILOT_PREREG_2026-06-15.md`
- Analysis: `scripts/analyze_het_carrier_pilot.py`, runner `scripts/het_carrier_pilot.sh`
- Adversarial audit: workflow `wf_fd1ba89f` (5 agents, all NO-VIOLATION)
- §8 design: `handover/audits/ART_0_2_FULL_CLOSE_DESIGN_2026-06-15.md`
- Carrier source frozen at commit `f73163f4` (branch `claude/het-carrier-freeze`)
