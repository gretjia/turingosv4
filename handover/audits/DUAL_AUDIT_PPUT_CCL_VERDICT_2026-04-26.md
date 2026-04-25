# Dual Audit Merged Verdict — PREREG_PPUT_CCL_2026-04-26 (Phase A4 round 1)

date: 2026-04-26
target: handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md
auditors: Codex (gpt-5-codex via codex-cli) + Gemini 2.5 Pro (independent)
conservative_rule: VETO > CHALLENGE > PASS (per `feedback_dual_audit_conflict` + C-070)
gate_state: **A5 commit gate NOT crossed; Phase B BLOCKED until round 2 PASS/PASS**

## merged verdict: **CHALLENGE** (round 2 required)

| Auditor | Verdict | Conviction |
|---|---|---|
| Codex | CHALLENGE | high; "fixable before Phase A5 without changing the research thesis" |
| Gemini | CHALLENGE | "would have flagged this PREREG as inadequate had Codex independently issued PASS" |

Both auditors converge on CHALLENGE despite different angles, which is the strongest signal of real defects (per F-2026-04-22-04 lesson on independent dual-audit signal). Per `feedback_dual_audit`, this is exactly the regime the dual-audit infrastructure is built for.

## convergent P0 (both auditors flagged the same area, different angles)

### M1 — Statistical family closure (Codex axis 2 + Gemini axis 2)

- **Codex finding**: family=5 declared in § 9 but H1-H4 each contain MULTIPLE endpoints (H1 = McNemar + paired PPUT; H2 = CPR + VPPUT; H4 = PPUT + IAC). H5 is a deterministic gate, NOT a statistical test at α=0.01. § 6 C2 "hard-10 × N=20 paired" does not specify the independent unit (per-problem? per-seed? per-(problem,seed))?
- **Gemini finding**: family-of-5 omits per-artifact guardrail tests (ΔFAR ≤ 0, RR=0, ΔCPR ≤ 0). These are inferential tests on heldout data subject to multiplicity. Family-wise α correction understates Type I error.
- **Synthesized fix**: rewrite § 5 to declare ONE primary endpoint per H1-H5 (others → descriptive); reclassify H5 as a deterministic gate (no α); expand family to include guardrail tests (5 + 3·k where k = count of artifacts entering heldout); declare Holm-Bonferroni (more powerful than vanilla Bonferroni at the larger family); explicitly state independent unit = per-(problem, seed) tuple.

## divergent P0 (only one auditor caught — but each is correct)

### M2 — Heldout sealing is operational not cryptographic (Codex axis 3, LEAKAGE)

- **Codex finding**: split JSON contains raw heldout IDs in cleartext; seed string `20260426_PPUT_CCL` and bucketing rule are public; ANY agent with repo access can read the JSON or recompute buckets. § 2.3's grep-based isolation is one defense layer, not "sealing".
- **Gemini missed this** (rated PASS on leakage).
- **Why Codex is right**: this is a definitional honesty issue. "Heldout sealed by SHA-256" suggests cryptographic protection; what we actually have is operational protection (conformance test enforces no-read at code-path level). If an agent CAN access the file (e.g., via LLM context window via tool calls that read repo files), the seal is broken.
- **Synthesized fix**: rename to "operational sealing" (PREREG § 2.3); reframe SHA-256 as integrity check (tamper detection), not access control; ADD a strong access-layer rule: PREREG and SPLITS files must NOT enter agent prompt context. New conformance test: `tests/prereg_split_files_not_in_agent_context.rs` greps Phase B-D agent loaders for ANY read of these files. Document the residual operational risk.

### M3 — Gate H measurement protocol underdefined (Codex axis 5)

- **Codex finding**: § 6 E2 says "single sealed heldout eval"; § 6 E3 says "compute H-VPPUT_pre vs H-VPPUT_post per artifact". How is per-artifact Δ measured under the single-eval constraint? Iterating burns heldout. Need explicit protocol.
- **Gemini missed this** (rated PASS on this axis as "infrastructure works"; it focused on RR=0 brittleness).
- **Why Codex is right**: the single-eval constraint is real; per-artifact ΔPPUT requires either (a) factorial design (run the eval with all artifact subsets — combinatorial), (b) leave-one-out within the eval (k+1 sub-evals all on the same problems), or (c) declare per-artifact attribution out-of-scope and only measure aggregate.
- **Synthesized fix**: declare protocol — "**leave-one-out within the sealed eval**: compute baseline PPUT (no artifacts active) on heldout-54 once; compute PPUT with all-artifacts-active once; compute PPUT with each individual artifact ablated (k sub-evals where k = number of Quarantined artifacts entering Phase E). Per-artifact ΔPPUT = PPUT_all − PPUT_minus_one. This burns the heldout once with k+2 sub-evals on the SAME problem set, not k+2 independent draws. Single-touch discipline preserved." Multiple-comparison correction lives in M1.

### M4 — Gate H RR=0 too brittle (Gemini axis 5)

- **Gemini finding**: RR=0 (single regression invalidates artifact) is statistically over-determined. Single-problem stochastic edge case kills otherwise-beneficial artifact.
- **Codex missed this** (focused on definitional underspecification of Gate H rather than reachability).
- **Why Gemini is right**: RR=0 is operationally absolute but statistically uncalibrated. Even on a perfectly-behaved system, on 54 problems × stochastic decoding, P(any single regression) approaches certainty. The threshold makes null result inevitable.
- **Synthesized fix**: relax to **"RR ≤ 1 for k=1 artifact, OR pre-registered exact binomial test against per-problem regression null rate p₀"**. Specifically: declare p₀ = the system's baseline regression rate measured pre-Phase-D on adaptation set (a calibration measurement). Reject artifact if RR_post > p₀ at one-sided exact binomial α = 0.01.

### M5 — Human SPOF in Phase D (Gemini axis 6 + Codex P1)

- **Both flagged**, Gemini at P0, Codex at P1 with similar substance.
- **Synthesized fix** (raise to P0 per conservative rule): § 6 D5 add operational rule:
  - Maximum review delay: 48 hours per Accepted candidate
  - If user unavailable > 48h: candidate moves to "deferred" queue; throughput drops but arc clock does NOT pause
  - 30-day cap is wall-clock; user unavailability = lost capacity, accepted operational risk
  - If > 5 candidates queued > 48h, abort Phase D and proceed to Phase E with whatever artifacts are Quarantined

## divergent P1 (one auditor; downgraded rationale)

### M6 — Lookup-table evasion of § 3.5 (Codex P1 #1)

- **Codex finding**: 64-token rolling window is bypassed by 63-token chunks, problem-ID keys, theorem-name keys, paraphrased tactics. `parametric_templates` satisfied by dummy placeholders.
- **Synthesized fix**: § 3.5 strengthen:
  - `docs_no_problem_id_keys`: artifact body must not contain literal MiniF2F problem IDs from adaptation/meta_val (string match)
  - `docs_no_theorem_name_keys`: artifact body must not contain Mathlib-style theorem names AS DICT KEYS (parse YAML/JSON; reject if keys match Lean theorem name regex)
  - `docs_rolling_hash_multi_window`: rolling-hash check at window sizes 16, 32, 64, 128 (defends against off-by-one chunking attacks)
  - `docs_high_cardinality_map_bound`: any dict-style structure in artifact has ≤ 8 distinct keys (lookup tables have many keys; rules have few)

### M7 — Phase C ablation findings scoped to deepseek-v4-flash (Codex axis 7)

- **Codex finding**: § 10 forbids overclaim "PPUT detects constitutional violations" but doesn't scope it to the model. If only run on v4-flash, claim is v4-flash-specific.
- **Synthesized fix**: § 10 add row: "PPUT detects constitutional violations [generally]" → "FORBIDDEN; allowed wording: 'PPUT detects Soft Law / Panopticon / Amnesia constitutional violations on deepseek-v4-flash; replication on heterogeneous model stack deferred to future arc'".

## housekeeping (Codex axis 9)

### H1 — counts mismatch in PREREG and notepad

- Codex caught: § 10 / notepad still reference "heldout-49" in some places; realized count is 54.
- Fix: grep-and-replace "heldout-49" / "heldout/49" / "N=49" → "heldout-54" / "N=54" in PREREG and notepad.

### H2 — date discrepancy

- Codex caught: today's UTC is 2026-04-25; PREREG dates as 2026-04-26.
- Reality: this is the running date stamp per `currentDate: 2026-04-25`. The arc launched on 2026-04-25 user-time; "2026-04-26" was Claude's projection. To stay honest: rename file → `PREREG_PPUT_CCL_2026-04-25.md`, OR keep the 2026-04-26 stamp and add a § Note stating "drafted 2026-04-25 evening UTC; dated forward to 2026-04-26 to align with split generation date stamp; both refer to the same arc launch".
- Recommendation: keep PREREG file name (avoid file rename now); add Note in § 14.

## items to leave alone (auditor PASS on both)

- DEFINITION (Gemini PASS) — Codex flagged § 1.x ambiguities; Gemini's PASS is more permissive but Codex's points are absorbed via M1 + M3.
- LEAKAGE / GOODHART core architecture — Gemini PASS; Codex axis 4 GOODHART CHALLENGE absorbed via M6.
- HETEROGENEITY-TIMING — Gemini PASS; Codex PASS (scoped via M7).
- TRUST-ROOT-ENFORCEMENT — Gemini PASS; Codex CHALLENGE absorbed as: PREREG must declare implementation contingency: if syscall EPERM not feasible in Rust user-space, fallback = lib-level write gate + sandbox + path whitelist; bypass = BLOCKER.
- REPRO — both broadly PASS; housekeeping fixes (H1, H2) only.
- CLAIM-LANG — Gemini PASS; Codex CHALLENGE absorbed in M7 + add "Quarantined-only / partial outcome" line.

## round-2 revision plan (estimated 90 min)

1. Apply M1 — § 5 + § 9 statistical family closure (largest edit)
2. Apply M2 — § 2.3 operational sealing reframe + new conformance test declaration
3. Apply M3 — § 6 E2-E3 leave-one-out heldout protocol (sharpest edit)
4. Apply M4 — § 7 Gate H RR≤1 / binomial test
5. Apply M5 — § 6 D5 SPOF contingency
6. Apply M6 — § 3.5 four additional content predicates
7. Apply M7 — § 10 v4-flash scoping row + partial-outcome wording
8. Apply H1 — heldout-54 grep-and-replace
9. Apply H2 — § 14 date note
10. Apply Trust Root contingency (§ 1.8) — fallback enforcement spec
11. Update changelog with round-2 entries
12. Commit; submit to Codex + Gemini round 2

If round 2 returns PASS/PASS → A5 commit gate; if CHALLENGE → round 3.

## evidence locations

- Gemini audit: `handover/audits/GEMINI_PPUT_CCL_AUDIT_2026-04-26.md` (51s elapsed, 140K-char prompt)
- Codex audit: `handover/audits/CODEX_PPUT_CCL_AUDIT_2026-04-26.md` (~3 min elapsed, 62K tokens)
- This merged verdict: `handover/audits/DUAL_AUDIT_PPUT_CCL_VERDICT_2026-04-26.md`
- Reproducer scripts: `handover/audits/run_gemini_pput_ccl_audit.py` + `handover/audits/run_codex_pput_ccl_audit.sh`

## next action

User decision required:
- (a) Proceed with round-2 revision now (10 fixes, ~90 min, then re-audit)
- (b) Pause; user reviews specific auditor objections before authorizing revisions
- (c) Modify research arc in response to objections (architectural, not just textual)

Default recommendation: (a) — both auditors agree fixes are textual + protocol, not architectural; PREREG round-2 is the right path forward; Phase B start delayed by ~3-4 hours total (round-2 revise + re-audit cycle).
