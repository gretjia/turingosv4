# P1 v4 — STAGE A hard-floor result (2026-06-02)

Binary `cb89a5d6`; prereg `P1_REALVALUE_V4_CORRECTED_PREREG_2026-06-02.json` (SHA-locked `d80ba8cd`).
`single` × 18 candidate theorems × **6 seeds** (24 attempts/cell), all 108 cells replay-clean.

## The robust hard floor (single 0/6)

| group | theorems | note |
|---|---|---|
| **HARD (single 0/6) — 13** | lm_deriv1, lm_ineq2, lm_coeff_mul, lm_nt_gcd2, lm_c, lm_e, lm_f, lm_fact, lm_ineq3, lm_lim1, lm_natdeg_pow, lm_nt_cop_cubic, lm_probe1 | the confound-shielded set for STAGE B |
| **EASY (single ≥1/6) — 5** | lm_ineq1 (1/6), lm_median (1/6), lm_deriv2 (2/6), lm_finset_sup (1/6), **lm_det_zero (6/6)** | EXCLUDED — single can do them, no shield |

## Why this matters (the finding deeper than the auditors' confound B)

**`lm_ineq1` is EASY (single 1/6), not hard.** In v3 it was the flagship "autonomous cracked a hard
theorem" result — classified hard because `single` got 0/3. But `single` gets 24 attempts/cell; the v4
floor shows `single` proves `lm_ineq1` (axiom-clean `[Classical.choice, Quot.sound, propext]`, the SAME
AM-GM route v3 credited to `autonomous`). A 0/3 seed-triple is a ~5% fluke if the true per-cell rate is
~60%. **So the v3 3-seed hard floor was not robust, and the `lm_ineq1` "crack" was a small-sample
artifact, not evidence of organization.** This is the auditors' question A — *"is single-fails-0/N robust
or just unlucky?"* — answered empirically: **unlucky.** It is deeper than confound B (the prompt-context
leak): even with a perfect single-variable contrast, the *floor* itself was flawed.

**`lm_deriv1` IS robustly hard (single 0/6)** — the v3 *other* crack. It stays in the hard set. (Recall:
in the confounded v3 axiom-confirm re-run, `autonomous` failed to reproduce `lm_deriv1` 0/6 — so it may be
hard for every arm. STAGE B decides.)

13/18 robustly hard is a usable confound-shielded set with real statistical room (13 theorems × 6 seeds).

## STAGE B (running, bnxkb2wq0)

`{market, autonomous, parallel, shuffled_price, no_price}` × the 13 hard × 6 seeds (390 cells), paired
with `single` over (theorem, seed). Decides: does **decoupled** `autonomous` (Path-1 free-choice, proof
prompt byte-identical to `market`) or `market` (Path-2 forced price-routing) crack any of the 13 hard
theorems that `single` + the control family cannot — and is it significant under the two pre-registrations?
The honest outcomes remain: a clean organizational win on a robustly-hard theorem, OR (if the strong model
+ controls also fail/also pass) no causal evidence — the lm set may simply be too easy or uniformly hard
for this model to expose an organization effect.
