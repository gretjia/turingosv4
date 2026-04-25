# Dual Audit Merged Verdict — PREREG_PPUT_CCL_2026-04-26 ROUND 4 (Phase A4 closing)

date: 2026-04-26
target: handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md (round 4)
auditors: Codex (gpt-5-codex via codex-cli) + Gemini 2.5 Pro (independent)
conservative_rule: VETO > CHALLENGE > PASS (per `feedback_dual_audit_conflict` + C-070)

## merged verdict: **PASS / PASS** — Phase A5 commit gate CLEARED

| Round | Gemini | Codex | Merged | Action |
|---|---|---|---|---|
| 1 | CHALLENGE | CHALLENGE | CHALLENGE | round 2 revision |
| 2 | PASS | CHALLENGE | CHALLENGE | round 3 revision |
| 3 | PASS | CHALLENGE | CHALLENGE | round 4 clean rewrite |
| **4** | **PASS** | **PASS** | **PASS** | **A5 commit gate clear** |

## Round 4 verdict details

### Gemini round 4 (PASS — 3rd consecutive PASS)

> "All round-4 changes are sound. R4-1, R4-2, R4-3 closed Codex's round-3 P0s without introducing new issues or regressing my round-2/3 PASSes. The pre-registration is now significantly more rigorous."

Verified by Gemini:
- R4-1 power tables correct: 10/10 Phase C, ≥39/54 Phase E
- R4-2 j-RR descriptive guardrail logic sound; consistency synced everywhere
- R4-3 sealing layers L1-L5 close all reasonable recomputation attacks

> "Four revision rounds + clean rewrite is normal pre-registration tightening, not structural instability. Core thesis remained stable; protocol progressively hardened against valid critiques."

### Codex round 4 (PASS — first PASS after 3 CHALLENGEs)

> "Round-4 rewrite closes my round-3 blockers without introducing a new P0."

Verified by Codex (with executable math):
- Computed `sum(C(54,x) for x in range(35,41)) / 2^54` to verify Phase E power table
- Computed `0.9^54 ≈ 0.00338` to confirm round-3 j-RR was unwinnable
- Computed `0.05/34 ≈ 0.00147` to confirm Holm threshold matches § 9.5
- Confirmed `(problem, seed)` references only remain in p_0 calibration (§ 5.5) where it correctly describes paired control-vs-treatment, not the inferential unit

P0 closures:
- **P0-fam**: closed — single source of truth on per-problem unit; family `4+3k`; N_max=34 consistent across § 5.6 + § 9.1
- **P0-gate-h**: closed — j-RR descriptive guardrail is statistically defensible (constraint, not hypothesis)
- **P0-leak**: closed for operational sealing model — § 2.3 L2-L5 block all concrete recomputation paths; residuals (exotic hardware hashes, semantic paraphrases) are implementation hygiene not prereg blockers

Codex non-blocking nit: § 10.1 had stale "hard-10 × N=20" wording — fixed in this round-4 final edit (committed alongside this verdict).

> "Conviction: high. Four revision rounds + clean rewrite shows the protocol was under-specified earlier, but here it looks like the natural shape of hard preregistration work: the thesis stayed stable while the statistical and sealing machinery was made auditable."

## Round-1 to Round-4 evolution summary

| Item | R1 baseline | R4 final |
|---|---|---|
| Independent unit (Phase C) | (problem, seed) n=20 | per-problem n=10 |
| Independent unit (Phase E) | (problem, seed) | per-problem n=54 |
| j-RR | inferential test (later mathematically unwinnable) | descriptive point-check guardrail |
| Family size | 5 (then 4+4k) | 4+3k |
| N_max | undefined | 34 |
| Phase C power requirement | undefined ("8/8 like Paper 1") | 10/10 paired wins (rigorously derived) |
| Phase E per-artifact pass bar | undefined | ≥ 39/54 + j-RR ≤ p_0 + Rollbackable + 3 inferential rejects |
| Heldout sealing | "sealed by SHA-256" (misleading) | operational 5-layer (honestly framed) |
| p_0 calibration | undefined | full protocol (toggle/sample/estimator/freeze/ceiling/audit) |
| Trust Root enforcement | gestured at SIGKILL | primary syscall EPERM + fallback lib-gate+panic |
| ArchitectAI / AuditorAI roles | mentioned | full cognitive isolation conformance |
| Doc/artifact content predicates | none | 4 base + 4 lookup-table-evasion (8 total) |
| Human SPOF | undefined | 48h SLA + queue + ≥5 abort |
| Heterogeneous LLM timing | undefined | Phase D explicit (v4-flash + Gemini 2.5 Pro) |

## next action

**Phase A5 commit gate**: cross. Commit PREREG round 4 + splits JSON + architect insights + audit chain + notepad as a single arc-launch commit set.

**Phase B start**: Week 1 — kernel instrumentation + PPUT accounting (per § 6 Phase B).

**Compute spent on Phase A4**:
- Codex: ~62K + 92K + 174K + 112K = ~440K tokens across 4 rounds
- Gemini: ~140K + 110K + 355K + 604K chars = ~310K tokens across 4 rounds
- Total: ~$15-20 in API spend; well within $500 arc cap

## evidence locations

- This merged verdict: `handover/audits/DUAL_AUDIT_PPUT_CCL_VERDICT_ROUND4_2026-04-26.md`
- Round-1 verdict: `handover/audits/DUAL_AUDIT_PPUT_CCL_VERDICT_2026-04-26.md`
- All round audits: `handover/audits/{CODEX,GEMINI}_PPUT_CCL_AUDIT*_2026-04-26.md`
- Reproducer scripts (4 rounds each): `handover/audits/run_{codex,gemini}_pput_ccl_audit*.{py,sh}`
- PREREG: `handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md`
- Splits: `handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json`

## C-070 precedent extension (candidate)

Paper 1 took 3 dual-audit rounds (CHALLENGE → CHALLENGE → PASS). PPUT-CCL PREREG took 4 dual-audit rounds (CHALLENGE → CHALLENGE → CHALLENGE → PASS). C-070 may be revised to: "expect 3-5 dual-audit rounds for any non-trivial pre-registration; if not converging by round 5, structural issue with thesis (not protocol) is likely."

Recommend: do NOT formalize this until after Phase B-E completes; the empirical effectiveness of the 4-round-converged PREREG is the test.
