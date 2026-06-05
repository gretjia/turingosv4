# P1 Real-Value v4 — Clean-Context Audit (2026-06-02)

> Independent, skeptical, clean-context audit (§9 / §17 G5). Auditor did NOT implement this work
> and did not assume it correct. Every load-bearing number re-derived from the tape; the two
> headline cracks + the v3-correction crack + the market CONFIRMED_WIN independently RECOMPILED
> under real Lean 4.24.0 + Mathlib. Report under audit:
> `handover/reports/P1_REALVALUE_V4_FINDINGS_2026-06-02.md`.
> Prereg (SHA-locked): `handover/preregistration/P1_REALVALUE_V4_CORRECTED_PREREG_2026-06-02.json`.

## Provenance / environment

- Prereg `.sha256` = `f1a36f16502644c6eb01311adf45f207738301db4ece8695d8255f2db12643f5`; recomputed
  `shasum -a 256` of the prereg file = **identical**. SHA lock intact.
- Binary source `src/bin/lean_market_agent.rs` last changed at commit `cb89a5d6`
  ("fix(P1): remove fatal confound B + inline axiom gate + compute telemetry + topology baselines")
  — the exact commit the report/prereg name. `git status` shows the bin + `src/judges/lean_judge.rs`
  **clean** (not in the modified list). The audited binary corresponds to the data-producing source.
- Census: `ls | grep -v replay` → **732 manifests**; `*.replay.json` → **732 replay files**.
  Arm split: single 108 (18 candidates × 6 floor seeds), each of the 8 crack/topology arms 78
  (13 hard × 6) → 8×78 + 108 = **732**. Reconciles exactly.
- Lean toolchain `~/.elan/bin/lean` = v4.24.0 (matches mathlib4 `lean-toolchain` pin
  `leanprover/lean4:v4.24.0`); LEAN_PATH resolved via `lake env printenv LEAN_PATH` in
  `/Users/zephryj/work/mathlib4`.

---

## A. RECOMPUTE (G1) — MATCHES THE REPORT EXACTLY

Ran the exact audit-brief command:
`python3 scripts/analyze_p1_v4.py --dir … --arms <9 arms> --theorems <13 hard> --seeds 1,2,3,4,5,6`.

- **HARD floor = 13/13** (every named theorem has single 0/78). ✓
- **Solve-rate table reproduced byte-for-byte**: single 0/78, single_restart 2/78, single_tree_no_price
  4/78, parallel 2/78, parallel_restart 2/78, no_price 6/78, shuffled_price 5/78, market 2/78,
  autonomous 1/78. Identical to report §0 table (lines 27–36). ✓
- **CONFIRMED_WINs**: PREREG_1 (market) = `[lm_fact]` (1); PREREG_2 (autonomous) = `[lm_deriv1]` (1).
  Matches report lines 70–76. ✓
- **All McNemar `p_holm = 1.0000`** for both pre-registrations across all 4 controls each. ✓
- **Topology deltas reproduced**: +2.6, +2.6, −2.6, +0.0, +5.1, −1.3, **−3.8** (shuffled→market),
  −1.3. Identical to report §2 decomposition (lines 81–86). ✓
- **tok/solve**: single_tree_no_price 329 538 (best), market 1 130 480, autonomous 3 086 598 (worst).
  Matches report (329K / 1.13M / 3.09M). ✓
- **No EXCLUDED block** printed → all 732 cells loaded clean (no silent drops). ✓
- **Route honesty**: autonomous hallucination 0.0% (valid_index_hit=1376, fresh_root=479,
  hallucinated=0). Matches report's "0% route hallucination" (line 75). ✓

**Analyzer logic re-derived independently:**

- `mcnemar_one_sided_greater(b,c)` = `Σ_{k=b}^{n} C(n,k) / 2^n`, n=b+c — the standard one-sided exact
  sign test on discordant pairs (B ~ Binom(n, ½), upper tail). Verified by hand against every
  observed (b,c) pair; e.g. market vs no_price (b=2,c=6) → raw p=0.9648, smallest raw p (vs single,
  b=2,c=0) = 0.25; Holm ×m=4 → capped at 1.0. **Correct test, correct Holm.** The NOT-significant
  conclusion is sound.
- **"solved" is NOT a token-scan.** `load()` sets `ok` from `omega_reached || verified_count>0`. I
  verified across all 732 cells these two are **perfectly equivalent** (both=35, omega_only=0,
  vc_only=0) and that **every one of the 35 solved cells** has an `is_verified` node whose `axioms`
  ⊆ {propext, Classical.choice, Quot.sound} (0 dirty). `solved` is only populated for
  replay-clean cells (replay≠true → excluded upstream), so the prereg's "replay-clean AND axiom-clean"
  condition is enforced before a cell can count. `confirmed_wins()` checks solve-count only, but its
  input `solved` is already the replay-clean + axiom-clean subset — a defensible factoring, not a gap.

**A verdict: G1 PASS — every load-bearing number recomputes from the tape and equals the report.**

---

## B. HARD-FLOOR SOUNDNESS — PASS

- **Budget parity on the floor**: all 78 single-hard cells ran **exactly 24** `proposal_llm_calls`
  (min=max=24, 0 cells <20). Not truncated. Total `verified_count` across all 78 = **0** → the 0/78
  floor is real, not an artifact of short budget.
- **Full 18-candidate split reproduced**: 13 HARD (single 0/6) + 5 EXCLUDED — lm_ineq1 **1/6**,
  lm_median 1/6, lm_deriv2 2/6, lm_det_zero **6/6**, lm_finset_sup 1/6. Matches report line 52
  ("lm_ineq1 (1/6), lm_det_zero (6/6) and 3 others correctly excluded").
- **lm_ineq1 exclusion is correct AND independently verified.** single solves lm_ineq1 on seed 1,
  manifest axioms `[Classical.choice, Quot.sound, propext]`. I extracted the verified body from
  CAS (`cas_p1v4_lm_ineq1_single_s1`, OID `076e0363…`, the AM-GM-via-`positivity`+`nlinarith` proof),
  assembled it against the pool preamble, and **recompiled it myself**:
  `'ineq_amgm_concrete' depends on axioms: [propext, Classical.choice, Quot.sound]`, **exit 0**.
  This grounds the report's v3-correction story (lines 49–52): single genuinely solves the AM-GM
  route that v3 mis-credited to an autonomous "crack"; the v3 floor was a small-sample fluke.

**B verdict: PASS — floor budget-parity holds, 0/78 is real, lm_ineq1 exclusion is grounded in a
recompiled axiom-clean single solve.**

---

## C. CRACKS REAL + AXIOM-CLEAN (independent of the inline gate) — PASS

Proof bodies live in the git-backed CAS (`cas_p1v4_<run>/.turingos_cas_index.jsonl` +
`.git` blobs); the manifest node carries only a 120-char `body_preview`, which is sometimes
**ambiguous between two proposals sharing the prefix** — so I compiled every candidate and confirmed
the gate credited the *right* one. I recompiled, under real Lean 4.24.0 + Mathlib, with an
independent `#print axioms`:

| crack | CAS OID | exit | `#print axioms` | verdict |
|---|---|---|---|---|
| **autonomous** `lm_deriv1` s3 (PREREG_2 win) | `f96e88b1…` | **0** | `[propext, Classical.choice, Quot.sound]` | REAL ✓ |
| **control** `lm_ineq2` `no_price` s3 | `c2698919…` | **0** | `[propext, Classical.choice, Quot.sound]` | REAL ✓ |
| **single** `lm_ineq1` s1 (v3 correction) | `076e0363…` | **0** | `[propext, Classical.choice, Quot.sound]` | REAL ✓ |
| **market** `lm_fact` s2 (PREREG_1 win) | `331aca24…` | **0** | `[propext, Classical.choice, Quot.sound]` | REAL ✓ |

For each crack the *sibling* candidates sharing the preview prefix were also compiled and **correctly
failed** (e.g. lm_deriv1 `90f0385257` → type-mismatch + `sorryAx`; lm_ineq2 `3d67e60` → unknown
identifier `cauchy_schwarz_iff`; lm_fact 3 of 4 → `rewrite`/`unsolved goals` + `sorryAx`). This proves
the inline gate **distinguishes pass from fail** and credits only the kernel-clean proposal, and that
`sorryAx` is excluded by the whitelist exactly as designed.

I read `lean_judge.rs` (lines 71, 199, 213–312): `AXIOM_WHITELIST = {propext, Classical.choice,
Quot.sound}`; on exit-0 the judge runs a **second** `lean` invocation `#print axioms <name>`, parses the
transitive set, requires ⊆ whitelist, and is **fail-closed** on every missing soundness fact
(no name / re-run !exit0 / no axiom line / non-whitelist axiom → `axiom_rejected`). My recompiles
match this behavior independently.

**C verdict: PASS — all four tested cracks compile exit-0 and are axiom-clean under an independent
Lean+Mathlib + `#print axioms`, fully independent of the inline gate.**

---

## D. FAIR BASELINE (G3) — PASS (decouple is real, budget is equal, costs honest)

- `cargo build --bin lean_market_agent` OK; `./target/debug/lean_market_agent --self-test` →
  **`PROMPT-PARITY-OK route_summary_clean=true sha=94f6153c…` / exit 0.**
- The self-test is **load-bearing, not `f(x)==f(x)`** (read `self_test_inner`, lines 803–917): it
  drives the market prompt through the market path and the autonomous Stage-2 prompt through the
  *post-route* autonomous path and asserts SHA equality (866); it has a **negative control** — the
  confound-B landscape-augmented prompt must DIFFER (888–901) — so re-introducing a leak trips it; and
  it asserts the route summary leaks **no** body sentinel, **no** shield sentinel, **no** raw error
  text (905–917).
- **Live run loop confirms the design** (lines 1349–1413): autonomous Stage-1 calls
  `build_route_summary` (compact: index/price/confidence/error-class/age/hash — no bodies/errors),
  LLM returns only `parent_node`; Stage-2 (ALL arms) builds the proof via the single shared
  `stage2_proof_prompt(…)`, which is a **one-line delegate to `build_prompt`**. Same fn + same args
  for the same parent ⇒ proof context byte-identical; autonomous and market differ ONLY in who picks
  the parent. The librarian `lib` is computed once (1349) and reused → no recompute drift.
- **Equal budget = 24 proposals** for every arm (lines 1182–1191): 1-agent arms run
  `n_rounds × n_agents` = 6×4 = 24 rounds; 4-agent arms run 4×6 = 24. Empirically confirmed: every
  arm's `proposal_llm_calls` ≈ 1820–1872 (= 78 cells × ~24, minor variance from parse/LLM errors);
  single-hard cells are exactly 24 each.
- **Bear/route costs are recorded, not hidden.** Per-arm aggregate: bulls-only arms
  (single*, parallel*) have **bear=0**; price arms (market, no_price, shuffled_price, autonomous) have
  **bear ≈ 1782–1826**; only autonomous has **route ≈ 1855**. Token-conservation invariant
  `total_model_tokens == proof+route+bear+completion` holds for **all 732 cells (0 violations)** —
  no hidden compute discount. autonomous pays for route **and** bear on top of 24 proofs (why it is the
  3.09M-token worst arm); the price machinery's tokens are fully counted, so the "market loses on cost"
  finding is honest, not an accounting trick.
- **The "softmax" is a real distribution, not argmax-collapse (§17.3).** market cells show
  `distinct_price_ratios` mean 6.2 (max 19) and `price_discovery=True` on the 78 counted cells. The
  constitution gate `constitution_router_name_matches_mechanism` passes (8/8), including
  `softmax_distributes_even_under_near_equal_prices` and an argmax-mutant discrimination test.

**D verdict: PASS — no arm is crippled or advantaged; the autonomous decouple is byte-identical and
self-test-enforced; budget is equal; the extra price/route costs are transparently charged.**

---

## E. OVER-READ / HONESTY — PASS (correctly hedged; NO-GO is a *failure-to-confirm*, not over-stated)

- **No PROVEN / DEFINITIVE / causal / X>Y over-claim is asserted.** `grep -niE` found "causal" only
  at line 69 ("LAYER 3 — Causal / pre-registered — **both NOT confirmed**", a label immediately
  negated) and lines 122–123 ("**scoped, non-causal NO-GO** + a directional (**non-significant**) lever
  finding — **no PROVEN/causal claim**"). §17.1 respected.
- **Non-significance is hedged repeatedly**: "Nothing is statistically significant (all McNemar
  `p_holm = 1.0`)" (20), "directional, not significant" (67), "underpowered" (108), "CIs overlap
  (sparse data)" (67), "max 6/78" (107). The non-locality lever is explicitly **DIRECTIONAL**, not
  proven.
- **The price NO-GO is NOT over-stated.** The report's claim is the weaker, correct
  "PREREG_1 NOT CONFIRMED" (a failure to confirm the Hayek price-routing win), backed by point
  estimates against market (2 vs 5, 2 vs 6) and p_holm=1.0. I checked the reverse: could more seeds
  flip market > no_price? The discordant split (market-only=2, no_price-only=6) gives one-sided
  p(market beats no_price)=0.9648 — **NOT significant in either direction**. So the report correctly
  does **not** claim "price is proven worse"; it claims "real price does not BEAT destroyed/random
  price in this regime," which the data supports, and it explicitly invites more seeds to power the
  positive lever finding (108–109). The regime-scoping caveat (price-**routing** ≠ price-**allocation**,
  111–113) is appropriate and prevents over-generalization.
- The one presentation nit — the §0 headline is **bolded and forceful** ("The lever is NON-LOCAL TREE
  SEARCH") and a casual reader could over-read it as causal — is mitigated within the same section
  (lines 20–22) and is a style/emphasis matter, **out of audit scope** (§14 verdict domain). The
  substantive claim is honestly qualified.

**E verdict: PASS — honest, conservative, §17-compliant. No data-unsupported over-claim.**

---

## F. REPLAY (G1) — PASS

- **All 732 `.replay.json` have `economic_state_reconstructed: true`** (0 not-true). Spot-checked
  cells (the two recompiled cracks + lm_fact market win + a random market cell) additionally show
  `ledger_root_verified`, `system_signatures_verified`, `state_reconstructed`,
  `cas_payloads_retrievable`, `agent_signatures_verified`, `proposal_telemetry_cas_retrievable` all
  **true** and `replay_failure: null`, `initial_q_state_loaded_from_disk: true`.
- **No silent exclusion**: the analyzer prints an EXCLUDED block iff any cell is missing or
  replay-not-clean; it printed **none**. Independent census reconciles to 732/732.
- **EXISTENCE layer reconciles exactly**: 24 cracks on the hard set (report "24 axiom-clean cracks");
  7 theorems cracked by someone {lm_deriv1, lm_f, lm_fact, lm_ineq2, lm_ineq3, lm_natdeg_pow,
  lm_nt_gcd2} and 6 by nobody {lm_c, lm_coeff_mul, lm_e, lm_lim1, lm_nt_cop_cubic, lm_probe1} —
  both lists identical to report lines 60–63.

**F verdict: PASS — 732/732 replay-clean, full reconstruction, no hidden drops.**

---

## Cross-check: harness claim-integrity gates on this binary

- `cargo test --test constitution_headline_recompute_from_tape` → **6 passed / 0 failed**, including
  `recompute_catches_a_lying_manifest_while_the_byte_chain_stays_green` (G1 gate is load-bearing).
- `cargo test --test constitution_router_name_matches_mechanism` → **8 passed / 0 failed** (§17.3).

---

## VERDICT

**Clean-context audit verdict: `NO-VIOLATION`.**

Scope scanned: §4 tape-first evidence, §9 audit doctrine, §12 integer-money / shielded-views, §17.1
no-PROVEN gate (G1 recompute, G2 real-model+verifier, G3 fair equal-budget baseline, G4 ≥N seeds +
paired stats, G6 no compile-time-literal pass-condition), §17.2 replay-green≠correctness, §17.3
named-mechanism-matches-implementation. No constitutional violation found. Every load-bearing number
recomputes byte-equal from the tape (G1); the decision loop uses a real model + a real Lean kernel +
an independently-reproduced axiom gate (G2); all arms are equal-budget with transparently-charged
mechanism costs (G3); 6 preregistered seeds with a correct paired one-sided exact McNemar + Holm (G4);
this audit is the post-data clean-context artifact (G5); "solved" is `omega_reached` = an axiom-clean
kernel Verified, never a literal token-scan (G6). Four independent recompiles (autonomous lm_deriv1,
control lm_ineq2, single lm_ineq1, market lm_fact) all exit-0 and axiom-clean; their failing sibling
candidates correctly rejected.

**Judgment on the report's claims: `PROCEED`.**

All six load-bearing claims are warranted as written:
1. 13/18 robustly HARD; lm_ineq1 et al. EASY and excluded — verified, lm_ineq1 single-solve recompiled.
2. EXISTENCE: 24 axiom-clean cracks on single-0 theorems — verified; two independently recompiled.
3. PREREG_1 NOT confirmed (market loses 2 vs 6 / 2 vs 5, p_holm=1.0) — verified and correctly hedged.
4. PREREG_2 NOT confirmed (autonomous uniquely cracks only lm_deriv1, p_holm=1.0, costliest arm) —
   verified; the lm_deriv1 crack recompiled real + axiom-clean.
5. DECOMPOSITION (non-local tree the directional lever; real price the biggest negative delta −3.8pp;
   autonomous 3.09M vs single_tree_no_price 329K) — every delta and cost recomputed exactly, and the
   report flags it as DIRECTIONAL / underpowered, not proven.

The report is conservative, §17-compliant, and reproduces from the tape. The only non-blocking
observation is presentational (the §0 headline is bolded/forceful), which the report itself qualifies
two lines later and which is out of the audit's verdict domain. No gap to close before PR.
